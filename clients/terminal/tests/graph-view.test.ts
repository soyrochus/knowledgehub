import { expect, test } from "bun:test";
import { Writable } from "node:stream";
import { GraphSession } from "../src/graph-view.ts";
import { TerminalInput } from "../src/terminal-input.ts";
import { KittyGraphics, TerminalWriter, apc } from "../src/kitty-graphics.ts";
import { Failure } from "../src/types.ts";
import { viewOptions } from "../src/view-options.ts";
import { parse } from "../src/kh.ts";
function fixture() {
  const input = new TerminalInput(),
    writes: string[] = [];
  let allowed = true,
    checks = 0;
  const foreignImage = 0xffffffff,
    images = new Set<number>([foreignImage]);
  let uploading = 0;
  const output = new Writable({
    write(chunk, _e, cb) {
      const s = chunk.toString();
      writes.push(s);
      for (const match of s.matchAll(
        /\x1b_G([^;]*);[^\x1b]*\x1b\\|(\x1b\[[23]J)/g,
      )) {
        if (match[2]) {
          images.clear();
          continue;
        }
        const controls = Object.fromEntries(
          match[1].split(",").map((v: string) => v.split("=")),
        );
        const id = Number(controls.i);
        let reply: string | undefined;
        if (controls.a === "q") reply = "OK";
        if (controls.a === "t") uploading = id;
        if (controls.m === "0") {
          images.add(uploading);
          queueMicrotask(() =>
            input.feed(Buffer.from(apc(`i=${uploading}`, "OK"))),
          );
        }
        if (controls.a === "p")
          reply = images.has(id) ? "OK" : "ENOENT: image not found";
        if (controls.a === "d" && controls.d === "I") images.delete(id);
        if (reply)
          queueMicrotask(() => input.feed(Buffer.from(apc(`i=${id}`, reply))));
      }
      if (s.includes("\x1b[14t"))
        queueMicrotask(() =>
          input.feed(Buffer.from("\x1b[4;900;1440t\x1b[8;36;150t")),
        );
      cb();
    },
  });
  const r = { model: "m", id: "root", revision: "1" },
    source = { model: "m", id: "source", revision: "1" };
  const base: any = {
    info: { alias: "test", scope: {}, grants: ["Open", "Read"] },
    selection: { selection: "Snapshot", reference: "a".repeat(64) },
    anchor: "b".repeat(64),
    options: { bytes: 1048576 },
    partial: false,
    warnings: [],
    transport: {
      call: async (method: string, args: any) => {
        expect(method).toBe("authorize");
        expect(args.history).toBe(false);
        checks++;
        if (!allowed) throw new Failure({ code: "Denied", action: "Read" });
        return {};
      },
    },
    fork: (opts: any) => ({
      ...base,
      options: opts,
      open: async function () {
        return this;
      },
    }),
    resolve: async () => r,
    show: async () => ({
      record: {
        reference: r,
        type_ref: {
          model: "sdo",
          id: "sdo:type:EntityType",
          revision: "catalogue-3",
        },
        data: {
          data: {
            properties: {
              name: { value: "Root" },
              fields: { kind: "List", value: [] },
            },
          },
        },
      },
      membership: "selected",
    }),
    links: async () => ({ vertices: [], neighbours: [] }),
    release: async () => {},
    sources: async () => ({
      records: [{ reference: source, data: { kind: "SourceRevision" } }],
    }),
    preview: async () => ({
      reference: source,
      text: "exact bounded source",
      partial: true,
    }),
  };
  const graphics = new KittyGraphics(input, new TerminalWriter(output)),
    session = new GraphSession(base, r, viewOptions({}), graphics);
  return {
    session,
    writes,
    input,
    setAllowed: (v: boolean) => (allowed = v),
    checks: () => checks,
    images,
    foreignImage,
  };
}
test("redraw retains uploaded images and never erases unrelated terminal graphics", async () => {
  const f = fixture();
  try {
    await f.session.open();
    expect(f.images.has(f.foreignImage)).toBe(true);
    await f.session.action("+", {});
    await f.session.action("?", {});
    await f.session.action("?", {});
    expect(f.writes.join("")).not.toMatch(/\x1b\[[23]J/);
  } finally {
    await f.session.close();
    f.input.close();
  }
  expect([...f.images]).toEqual([f.foreignImage]);
});
test("viewer controls preserve exact selection; no history permission; explicit exact sources and small-window recovery", async () => {
  const f = fixture();
  try {
    await f.session.open();
    const selection = JSON.stringify(f.session.graph.selection);
    for (const [str, key] of [
      ["n", {}],
      ["p", {}],
      ["e", {}],
      ["e", {}],
      ["", { name: "tab" }],
      ["", { name: "return" }],
      ["j", {}],
      ["k", {}],
      ["", { name: "pagedown" }],
      ["", { name: "pageup" }],
      ["", { name: "right" }],
      ["+", {}],
      ["-", {}],
      ["0", {}],
      ["?", {}],
      ["?", {}],
      ["s", {}],
      ["", { name: "return" }],
    ] as const)
      await f.session.action(str, key);
    expect(JSON.stringify(f.session.detail)).toContain("exact bounded source");
    expect(JSON.stringify(f.session.graph.selection)).toBe(selection);
    f.session.geometry.columns = 60;
    await f.session.action("j", {});
    expect(f.writes.join("")).toContain("Window too small");
    await f.session.action("resize", {});
    expect(f.session.geometry.columns).toBe(150);
    expect(f.checks()).toBeGreaterThan(30);
  } finally {
    await f.session.close();
    f.input.close();
  }
});
test("Read revocation hides images and drops every cached detail; only explicit permitted pinned recovery reveals again", async () => {
  for (const [str, key] of [
    ["+", {}],
    ["?", {}],
    ["s", {}],
    ["resize", {}],
    ["", { name: "right" }],
  ] as const) {
    const f = fixture();
    try {
      await f.session.open();
      f.setAllowed(false);
      try {
        await f.session.action(str, key);
      } catch (e) {
        await f.session.fail(e);
      }
      expect(f.session.graph).toBeUndefined();
      expect(f.session.detail).toBe("");
      expect(f.session.blocked.code).toBe("Denied");
      f.setAllowed(true);
      const start = f.writes.length;
      await f.session.action("?", {});
      expect(f.writes.slice(start).join("")).not.toContain("a=p");
      await f.session.action("R", {});
      expect(f.session.graph.selection.reference).toBe("a".repeat(64));
    } finally {
      await f.session.close();
      f.input.close();
    }
  }
});
test("cancellation invalidates an in-flight frame; invalid viewer options are rejected before terminal/host access", async () => {
  const f = fixture();
  try {
    const opened = f.session.open();
    await Bun.sleep(1);
    f.session.invalidate();
    await expect(opened).rejects.toBeInstanceOf(Failure);
  } finally {
    await f.session.close();
    f.input.close();
  }
  for (const flags of [
    { vertices: "201" },
    { edges: "501" },
    { depth: "0" },
    { projection: "bpmn" },
  ] as Record<string, string>[])
    expect(() => viewOptions(flags)).toThrow();
  expect(parse(["view", "root", "--projection", "lifecycle"]).name).toBe(
    "view",
  );
  expect(() => parse(["view", "root", "--output", "bad"])).toThrow();
});
test("revocation during worker rendering cannot reveal a finished frame or inspector", async () => {
  const f = fixture();
  try {
    await f.session.open();
    const original = (f.session as any).render;
    (f.session as any).render = {
      render: (input: any) => {
        f.setAllowed(false);
        return original.render(input);
      },
      cancel: () => original.cancel(),
      close: () => original.close(),
    };
    const start = f.writes.length;
    try {
      await f.session.action("+", {});
    } catch (e) {
      await f.session.fail(e);
    }
    const after = f.writes.slice(start).join("");
    expect(after).not.toContain("a=p");
    expect(after).not.toContain('"selected"');
    expect(f.session.graph).toBeUndefined();
  } finally {
    await f.session.close();
    f.input.close();
  }
});
