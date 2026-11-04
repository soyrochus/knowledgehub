import { test, expect } from "bun:test";
import { Explorer } from "../src/explorer.ts";
import { exact, type Json } from "../src/types.ts";
test("64 history calls, zero progress and bounded cleanup preserve exact restart intent", async () => {
  let calls = 0,
    released = 0;
  const transport = {
    call: async (method: string) => {
      if (method === "release_history") {
        released++;
        return {};
      }
      calls++;
      return {
        items: { Publications: [{ operation_id: `op:${calls}` }] },
        work: { visited_nodes: "1", object_reads: "1", output_bytes: "300" },
        complete: false,
        continuation: "live-token",
      };
    },
  };
  const e = new Explorer(transport as any);
  e.anchor = "a".repeat(64);
  const result = await e.history();
  expect(calls).toBe(64);
  expect(result.items).toHaveLength(64);
  expect(e.partial).toBe(true);
  expect(result.stopped_error.code).toBe("Partial");
  await e.release();
  expect(released).toBe(1);
  expect(e.anchor).toBe("a".repeat(64));
  const noProgress = new Explorer({
    call: async () => ({
      items: { Publications: [] },
      work: { visited_nodes: "0", object_reads: "0", output_bytes: "300" },
      complete: false,
      continuation: "idle",
    }),
  } as any);
  noProgress.anchor = "b".repeat(64);
  const empty = await noProgress.history();
  expect(empty.complete).toBe(false);
  expect(noProgress.warnings.join(" ")).toContain("no progress");
});
test("duplicate type/display names require exact identity; programmatic budgets cannot disable bounds", async () => {
  const e = new Explorer({} as any);
  e.list = async () => [
    { reference: { model: "a", id: "same", revision: "1" }, name: "Repeated" },
    { reference: { model: "b", id: "same", revision: "1" }, name: "Repeated" },
  ];
  await expect(e.resolve("Repeated", true)).rejects.toMatchObject({
    detail: {
      code: "Ambiguous",
      candidates: [
        {
          reference: { model: "a", id: "same", revision: "1" },
          name: "Repeated",
        },
        {
          reference: { model: "b", id: "same", revision: "1" },
          name: "Repeated",
        },
      ],
    },
  });
  expect(
    await e.resolve({ model: "a", id: "same", revision: "1" }, true),
  ).toEqual({ model: "a", id: "same", revision: "1" });
  for (const opts of [
    { calls: Infinity },
    { items: 10001 },
    { bytes: 0 },
    { historyLimits: { items: "9007199254740993123" } },
  ])
    expect(() => new Explorer({} as any, opts)).toThrow();
});
test("binary preview is explicitly unsupported and never invents text", async () => {
  const r = { model: "x", id: "binary", revision: "1" };
  const e = new Explorer({} as any);
  e.read = async () => ({
    reference: r,
    data: {
      kind: "SourceRevision",
      data: {
        media_type: "application/octet-stream",
        original: { length: "3" },
      },
    },
  });
  e.call = async () => ({
    original: { length: "3" },
    bytes_base64: "AP+A",
    full_content_verified: true,
  });
  const p = await e.preview(r);
  expect(p.text).toBeUndefined();
  expect(p.unsupported).toContain("original-byte export");
  expect(p.full_content_verified).toBe(true);
});
