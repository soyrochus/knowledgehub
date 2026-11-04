import { test, expect } from "bun:test";
import {
  readFileSync,
  mkdtempSync,
  copyFileSync,
  writeFileSync,
  rmSync,
  realpathSync,
} from "node:fs";
import { resolve, join } from "node:path";
import { tmpdir } from "node:os";
import { Transport } from "../src/transport.ts";
import { Explorer } from "../src/explorer.ts";
import { exact, key, type Json } from "../src/types.ts";
import { extract, mermaid, html } from "../src/visual.ts";
import { outputPath, writeOutput } from "../src/output.ts";
const workspace = process.env.KH_TEST_WORKSPACE;
const native = workspace ? test : test.skip;
native(
  "native reader B: schema, occurrences, sources, original receipt, retained baseline and limits",
  async () => {
    const t = new Transport(workspace!);
    await t.handshake();
    try {
      for (const model of ["fruittrucks", "equipment"]) {
        const e = await new Explorer(t, { model }).open();
        try {
          const dir = resolve(workspace!, "..", model),
            hand = JSON.parse(readFileSync(join(dir, "handover.json"), "utf8"));
          expect(e.info.reader).toBe("host:reader-B");
          const types = await e.types();
          expect(types.length).toBeGreaterThan(100);
          if (model === "equipment") {
            const schema = await e.schema("SpecialDefinition");
            expect(schema.definitions.length).toBeGreaterThan(2);
            expect(
              schema.properties.some((x: Json) => x.path === "related_subject"),
            ).toBe(true);
          }
          if (model === "equipment") {
            const nav = await e.fork().open();
            const direct = await nav.list(exact("EntityType")),
              subtypes = await nav.list(exact("EntityType"), true);
            expect(subtypes.length).toBeGreaterThan(direct.length);
            const links = await nav.links("model:map");
            expect(
              links.neighbours.some(
                (n: Json) =>
                  n.selector.path === "/activities/*" &&
                  n.origin.occurrence.endsWith("/0"),
              ),
            ).toBe(true);
            const tree = JSON.stringify(await nav.tree("model:map", 3));
            expect(tree).toContain('"cycle":true');
            expect(tree).toContain('"shared":true');
            await nav.release();
          }
          for (const original of hand.publications) {
            const lookup = await e.publication(original.operation_id);
            const h = lookup.outcome.Published;
            expect(h.actor).toBe("host:publisher-A");
            expect(h.fingerprint).toBe(original.fingerprint);
            expect(h.sequence).toBe(String(original.sequence));
            const affected = await e.history(original.operation_id);
            expect(affected.items).toEqual(original.affected);
            expect(h.affected_count).toBe(String(affected.items.length));
          }
          const baselines = await e.baselines(true);
          expect(baselines.baselines.length).toBe(3);
          expect(baselines.retained.complete).toBe(true);
          expect(
            baselines.baselines.find(
              (x: Json) => key(x.reference) === key(hand.baselines[0]),
            ).origins,
          ).not.toContain("selected");
          const before = await e
            .fork({
              baseline: hand.baselines[0],
              snapshot: undefined,
              historyAnchor: hand.publications.at(-1).head,
            })
            .open();
          expect(before.anchor).toBe(hand.publications.at(-1).head);
          expect(before.selection.selection).toBe("Baseline");
          expect(
            (await before.publication(hand.publications[0].operation_id))
              .outcome,
          ).toBe("NotPublishedAt");
          await before.release();
          expect(
            (await e.compare(hand.baselines[0], hand.baselines[1])).differences
              .length,
          ).toBeGreaterThan(0);
          for (const source of hand.sources) {
            const original = await e.original(source.reference);
            expect(original.original.sha256).toBe(source.original.sha256);
          }
          const tiny = await e.fork({ items: 1 }).open();
          const partial = await tiny.history();
          expect(partial.complete).toBe(false);
          expect(tiny.partial).toBe(true);
          expect(partial.items.length).toBe(1);
          await tiny.release();
          const bounded = await e
            .fork({ historyLimits: { nodes: "1" } })
            .open();
          const limited = await bounded.history();
          expect(limited.stopped_error.code).toBe("HistoryLimit");
          expect(limited.items).toEqual([]);
          expect(bounded.partial).toBe(true);
          await bounded.release();
          if (model === "fruittrucks") {
            const links = await e.links("model:requirement:ack", {
              direction: "Incoming",
            });
            expect(
              links.neighbours.filter(
                (n: Json) =>
                  n.selector.definition.id ===
                  "sdo:relationship:UseCase.satisfies",
              ).length,
            ).toBe(2);
            const search = await e.search(
              "acknowledge",
              "Requirement",
              "/name",
            );
            expect(search.hits.length).toBeGreaterThan(0);
            expect(search.hits[0].start).toMatch(/^\d+$/);
            const graph = await extract(
              e,
              "model:requirement:ack",
              "relationships",
              1,
            );
            expect(
              graph.edges.filter(
                (x: Json) =>
                  x.selector.definition.id ===
                  "sdo:relationship:UseCase.satisfies",
              ).length,
            ).toBe(2);
            expect(mermaid(graph)).toContain("flowchart LR");
            const lifecycle = await extract(
              e,
              "model:lifecycle:offer",
              "lifecycle",
            );
            expect(lifecycle.edges.length).toBe(4);
            expect(
              lifecycle.edges.every(
                (x: Json) =>
                  x.origin.occurrence === "/from" &&
                  x.supporting_origins[0].occurrence === "/to",
              ),
            ).toBe(true);
            expect(lifecycle.coverage.warnings.join(" ")).toContain("proposed");
            const journey = await extract(e, "model:journey:change", "journey");
            expect(journey.edges.length).toBe(5);
            expect(
              journey.edges.every(
                (x: Json) =>
                  /^\/activities\/\d+$/.test(x.origin.occurrence) &&
                  x.supporting_origins.length === 1,
              ),
            ).toBe(true);
            expect((await e.gaps()).coverage.gaps.length).toBeGreaterThan(0);
            const narrative = await e.search(
              "expiry",
              "Requirement",
              "/statement",
            );
            expect(
              narrative.hits.some((h: Json) => h.source?.document && h.snippet),
            ).toBe(true);
          }
        } finally {
          await e.release();
        }
      }
    } finally {
      await t.close();
    }
  },
  120000,
);
native(
  "no history grants, live revocation and safe outputs",
  async () => {
    const folder = mkdtempSync(
        join(realpathSync(tmpdir()), "kh-reader-policy-"),
      ),
      config = JSON.parse(readFileSync(workspace!, "utf8"));
    for (const m of config.models)
      m.directory = resolve(workspace!, "..", m.directory);
    const file = join(folder, "workspace.json");
    writeFileSync(file, JSON.stringify(config));
    const t = new Transport(file);
    await t.handshake();
    try {
      const e = await new Explorer(t).open();
      await e.publication("authoring:model");
      config.models[0].grants = ["Open", "Read"];
      writeFileSync(file, JSON.stringify(config));
      await expect(e.publication("authoring:model")).rejects.toMatchObject({
        detail: { code: "Denied", action: "ReadHistory" },
      });
      expect((await e.types()).length).toBeGreaterThan(100);
      const out = join(folder, "new.json");
      writeOutput(out, "exact", file);
      expect(() => writeOutput(out, "overwrite", file)).toThrow();
      writeOutput(out, "replaced", file, true);
      expect(readFileSync(out, "utf8")).toBe("replaced");
      expect(() =>
        outputPath(join(config.models[0].directory, "forbidden.json"), file),
      ).toThrow();
      await e.release();
    } finally {
      await t.close();
      rmSync(folder, { recursive: true });
    }
  },
  30000,
);
