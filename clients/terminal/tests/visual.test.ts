import { test, expect } from "bun:test";
import {
  html,
  mermaidText,
  mermaid,
  digest,
  exportView,
} from "../src/visual.ts";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  realpathSync,
  symlinkSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { outputPath } from "../src/output.ts";
import { clean } from "../src/types.ts";
test("terminal, JSON-script and Mermaid controls remain text with exact DTO identity", () => {
  const attack =
    '</script><img src=x onerror=alert(1)> " ] --> evil[\u001b[31m';
  const r = { model: "test", id: "id", revision: "1" };
  const graph = {
    format: "knowledgehub-view/1",
    model: { alias: "test" },
    selection: { selection: "Snapshot", reference: "a".repeat(64) },
    publication_anchor: "b".repeat(64),
    nodes: [
      {
        id: "n123",
        reference: r,
        label: attack,
        source_bindings: { provenance: [] },
      },
    ],
    edges: [],
    coverage: {
      complete: false,
      vertices: "1",
      edges: "0",
      warnings: ["limited"],
    },
  };
  const encoded = html(graph);
  expect(encoded).not.toContain("<img src=x");
  expect(encoded).toContain("connect-src 'none'");
  expect(encoded).toContain("\\u003c/script\\u003e");
  expect(mermaidText(attack)).not.toContain("<");
  expect(mermaidText("R8 — sdo:x")).toBe("R8 — sdo:x");
  expect(mermaid(graph)).not.toContain("] --> evil[");
  expect(clean(attack)).not.toContain("\u001b");
  expect(graph.nodes[0].label).toBe(attack);
  expect(digest(graph)).toHaveLength(64);
});
test("unresolved/limited coverage survives all formats; store and symlink escapes are refused", () => {
  const dir = mkdtempSync(join(realpathSync(tmpdir()), "kh-view-"));
  try {
    mkdirSync(join(dir, "hub"));
    const workspace = join(dir, "workspace.json");
    writeFileSync(
      workspace,
      JSON.stringify({
        format: "knowledgehub-client-workspace/1",
        models: [{ directory: "hub" }],
      }),
    );
    const r = {
      model: "test:external",
      id: "missing:definition",
      revision: "1",
    };
    const graph = {
      format: "knowledgehub-view/1",
      model: { alias: "synthetic-unresolved-view" },
      selection: { selection: "Snapshot", reference: "a".repeat(64) },
      publication_anchor: "a".repeat(64),
      intent: { root: r, projection: "relationships" },
      nodes: [
        {
          id: "n123",
          reference: r,
          label: "Unresolved exact endpoint",
          available: false,
          membership: "external/unresolved",
          source_bindings: { provenance: [], narratives: [] },
        },
      ],
      edges: [],
      coverage: {
        complete: false,
        vertices: "1",
        edges: "0",
        warnings: ["unresolved endpoint; no content inferred"],
      },
    };
    for (const format of ["json", "mermaid", "html"]) {
      const path = join(dir, "view." + format);
      const result = exportView(graph, format, path, workspace);
      expect(result.coverage.complete).toBe(false);
      expect(
        JSON.parse(readFileSync(path + ".manifest.json", "utf8")).coverage
          .warnings[0],
      ).toContain("unresolved");
      expect(readFileSync(path, "utf8").length).toBeGreaterThan(10);
    }
    symlinkSync(join(dir, "hub"), join(dir, "escape"));
    expect(() =>
      outputPath(join(dir, "escape", "bad.json"), workspace),
    ).toThrow();
    expect(() =>
      outputPath(join(dir, "hub", "..", "hub", "bad.json"), workspace),
    ).toThrow();
  } finally {
    rmSync(dir, { recursive: true });
  }
});
