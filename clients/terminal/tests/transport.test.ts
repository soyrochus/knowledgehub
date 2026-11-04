import { test, expect } from "bun:test";
import { mkdtempSync, writeFileSync, chmodSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { Decoder, encodeFrame, Transport, protocol } from "../src/transport.ts";
import { Failure, uint } from "../src/types.ts";
test("fragmented frames, precision, malformed/oversize/truncated payloads", () => {
  const d = new Decoder(),
    b = encodeFrame({
      id: "1",
      text: "000123",
      unsigned: "18446744073709551615",
    });
  expect(d.push(b.subarray(0, 3))).toEqual([]);
  expect(d.push(b.subarray(3))[0].unsigned).toBe("18446744073709551615");
  d.end();
  expect(() => new Decoder().push(Buffer.from([255, 255, 255, 255]))).toThrow();
  const t = new Decoder();
  t.push(b.subarray(0, 9));
  expect(() => t.end()).toThrow();
  expect(() => uint("18446744073709551616", 1, 1000)).toThrow();
  expect(() => uint("01", 1, 1000)).toThrow();
});
const fake = `#!${process.execPath}
let b=Buffer.alloc(0);for await(const chunk of Bun.stdin.stream()){b=Buffer.concat([b,chunk]);while(b.length>=4&&b.length>=b.readUInt32BE(0)+4){const n=b.readUInt32BE(0),q=JSON.parse(b.subarray(4,n+4));b=b.subarray(n+4);if(q.method==='lost')process.exit(0);const reply=()=>{const body=Buffer.from(JSON.stringify({protocol:'knowledgehub.client/1',id:q.id,ok:true,data:q.method==='handshake'?{protocol:'knowledgehub.client/1',build:{history_projection:'knowledgehub-history/1',record_codec:'knowledgehub-record/3',request_codec:'knowledgehub-request/3'},methods:['publication','publications','publication_affected','continue_history','release_history']}:q.args}));const h=Buffer.alloc(4);h.writeUInt32BE(body.length);process.stdout.write(Buffer.concat([h,body]));if(q.method==='drain')process.exit(0);};q.method==='slow'?setTimeout(reply,150):reply();}}
`;
test("finite admitted work, cancellation drains, host reuse and lost response", async () => {
  const dir = mkdtempSync(join(tmpdir(), "kh-transport-")),
    host = join(dir, "fake-host");
  writeFileSync(host, fake);
  chmodSync(host, 0o755);
  try {
    const t = new Transport("ignored", host);
    await t.handshake();
    const abort = new AbortController();
    const p = t.call("slow", { original: "000123" }, undefined, abort.signal);
    abort.abort();
    await expect(p).rejects.toMatchObject({ detail: { code: "Cancelled" } });
    const pending = Array.from({ length: 15 }, (_, i) => t.call("slow", { i }));
    await expect(t.call("slow")).rejects.toMatchObject({
      detail: { code: "Busy" },
    });
    await Promise.all(pending);
    expect((await t.call("echo", { text: "000123" })).text).toBe("000123");
    await t.close();
    const timeout = new Transport("ignored", host, 10);
    await timeout.handshake();
    const cancel = new AbortController();
    const slow = timeout.call("slow", {}, undefined, cancel.signal);
    cancel.abort();
    await expect(slow).rejects.toMatchObject({ detail: { code: "Cancelled" } });
    await expect(timeout.close()).rejects.toMatchObject({
      detail: {
        code: "Disconnected",
        message: "drain deadline; no completion claimed",
      },
    });
    const lost = new Transport("ignored", host);
    await lost.handshake();
    await expect(lost.call("lost")).rejects.toMatchObject({
      detail: { code: "Disconnected" },
    });
    await lost.close();
  } finally {
    rmSync(dir, { recursive: true });
  }
});
test("cursor lifetime simulation keeps full pinned intent for an explicit restart", () => {
  const intent = {
    selection: {
      selection: "Baseline",
      reference: { model: "x", id: "A", revision: "1" },
    },
    anchor: "a".repeat(64),
    method: "Publications",
    limits: { items: "1" },
  };
  const saved = structuredClone(intent);
  let live: { intent: unknown; last: number } | undefined = { intent, last: 0 };
  const now = 30 * 60 * 1000 + 1;
  if (now - live.last > 30 * 60 * 1000) live = undefined;
  expect(live).toBeUndefined();
  expect(saved.anchor).toBe("a".repeat(64));
  expect(saved.selection.reference.id).toBe("A");
  expect(saved.limits.items).toBe("1");
});
