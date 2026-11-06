import { expect, test } from "bun:test";
import { Writable } from "node:stream";
import { TerminalInput, type Reply } from "../src/terminal-input.ts";
import { KittyGraphics, TerminalWriter, apc } from "../src/kitty-graphics.ts";
test("bounded demux preserves UTF-8, arrows and ordinary keys, filters fragmented/late/malformed replies", async () => {
  const input = new TerminalInput(),
    keys: Buffer[] = [],
    replies: Reply[] = [];
  input.keys.on("data", (b) => keys.push(b));
  input.on("reply", (r) => replies.push(r));
  const raw = Buffer.from(
    "é" +
      apc("i=23", "OK") +
      "\x1b[4;900;1440t\x1b[8;36;150t\x1b[?62;4c\x1b[Aq" +
      apc("i=bad", "bad"),
  );
  for (const byte of raw) input.feed(Buffer.from([byte]));
  expect(Buffer.concat(keys).toString()).toBe("é\x1b[Aq");
  expect(replies.map((r) => r.kind)).toEqual([
    "graphics",
    "pixels",
    "cells",
    "device",
  ]);
  input.feed(Buffer.from("\x1b_G" + "a".repeat(100000)));
  expect(input.retainedBytes).toBeLessThanOrEqual(65536);
  input.feed(Buffer.from("\x1b\\z"));
  expect(Buffer.concat(keys).toString()).toEndWith("qz");
  await expect(input.wait((r) => r.id === 999, 5)).rejects.toThrow();
  input.feed(Buffer.from(apc("i=999", "OK")));
  expect(Buffer.concat(keys).toString()).toEndWith("qz");
  const abort = new AbortController(),
    wait = input.wait(() => true, 1000, abort.signal);
  abort.abort();
  await expect(wait).rejects.toThrow();
  input.close();
});
function terminal(negative = false, missing = false, placementError?: string) {
  const input = new TerminalInput(),
    writes: string[] = [];
  const output = new Writable({
    write(chunk, _encoding, callback) {
      const text = chunk.toString();
      writes.push(text);
      for (const match of text.matchAll(/\x1b_G([^;]*);[^\x1b]*\x1b\\/g)) {
        const id = /(?:^|,)i=(\d+)/.exec(match[1]);
        if (id && !missing && /a=[qtp]/.test(match[1]))
          queueMicrotask(() =>
            input.feed(
              Buffer.from(
                apc(
                  `i=${id[1]}`,
                  /a=p/.test(match[1]) && placementError
                    ? placementError
                    : negative
                      ? "ENOTSUP"
                      : "OK",
                ),
              ),
            ),
          );
      }
      if (text.includes("\x1b[14t"))
        queueMicrotask(() =>
          input.feed(Buffer.from("\x1b[4;900;1440t\x1b[8;36;150t")),
        );
      setTimeout(callback, 1);
    },
  });
  return {
    input,
    writes,
    output,
    graphics: new KittyGraphics(input, new TerminalWriter(output), 20),
  };
}
test("unterminated response fragments expire, late whole replies stay filtered and Ctrl-C is never trapped", async () => {
  const input = new TerminalInput(undefined, 10),
    keys: Buffer[] = [];
  input.keys.on("data", (b) => keys.push(b));
  input.feed(Buffer.from("\x1b_Gi=123;O"));
  await Bun.sleep(20);
  expect(input.retainedBytes).toBe(0);
  input.feed(Buffer.from("q" + apc("i=123", "OK")));
  expect(Buffer.concat(keys).toString()).toBe("q");
  input.feed(Buffer.from("\x1b_Gi=124;O"));
  input.feed(Buffer.from("\x03"));
  expect(Buffer.concat(keys).toString()).toBe("q\x03");
  input.close();
});
test("direct support/geometry, bounded chunks, serialized text, acknowledgements and owned cleanup", async () => {
  const t = terminal();
  expect(await t.graphics.probe(undefined, {})).toEqual({
    width: 1440,
    height: 900,
    rows: 36,
    columns: 150,
  });
  const upload = t.graphics.upload(new Uint8Array(10000), () => true);
  const text = t.graphics.writer.write("TEXT");
  const id = await upload;
  await text;
  await t.graphics.reveal(id, 90, 24, () => true);
  const chunks = t.writes.filter((s) => /m=[01];/.test(s));
  expect(chunks.length).toBeGreaterThan(1);
  expect(
    chunks.every((s) => s.split(";")[1].split("\x1b")[0].length <= 4096),
  ).toBe(true);
  expect(t.writes.indexOf("TEXT")).toBeGreaterThan(
    t.writes.indexOf(chunks.at(-1)!),
  );
  await t.graphics.hide();
  await t.graphics.close();
  expect(t.graphics.cleanupWritten).toBe(true);
  expect(t.writes.join("")).not.toMatch(/a=d,d=[aA](?:,|;)/);
  expect(t.writes.join("")).toContain(`d=I,i=${id}`);
  t.input.close();
});
test("negative/missing/device-only replies, multiplexer and cancellation fail finitely", async () => {
  for (const [negative, missing] of [
    [true, false],
    [false, true],
  ]) {
    const t = terminal(negative, missing);
    await expect(t.graphics.probe(undefined, {})).rejects.toThrow();
    await t.graphics.close();
    t.input.close();
  }
  const t = terminal();
  await expect(t.graphics.probe(undefined, { TMUX: "yes" })).rejects.toThrow();
  expect(t.writes).toHaveLength(0);
  const no = new TerminalInput(),
    writer = new TerminalWriter(
      new Writable({
        write(_b, _e, cb) {
          cb();
        },
      }),
    ),
    g = new KittyGraphics(no, writer, 5);
  const pending = g.probe(undefined, {});
  no.feed(Buffer.from("\x1b[?62c"));
  await expect(pending).rejects.toThrow();
  no.close();
  t.input.close();
});
test("placement rejection retains the terminal reason and owned image identity", async () => {
  const t = terminal(false, false, "ENOENT: image not found");
  try {
    const id = await t.graphics.upload(new Uint8Array(4), () => true);
    await expect(
      t.graphics.reveal(id, 90, 24, () => true),
    ).rejects.toMatchObject({
      detail: {
        code: "GraphicsTransport",
        message: "terminal rejected placement: ENOENT: image not found",
        image_id: id,
      },
    });
  } finally {
    await t.graphics.close();
    t.input.close();
  }
});
test("partial upload, stale reveal and negative ack clean only owned resources; output stalls never certify deletion", async () => {
  const t = terminal();
  await expect(
    t.graphics.upload(new Uint8Array(10000), () => false),
  ).rejects.toThrow();
  await expect(t.graphics.reveal(1, 90, 24, () => false)).rejects.toThrow();
  await t.graphics.close();
  expect(t.graphics.cleanupWritten).toBe(true);
  t.input.close();
  const stalled = new Writable({ write(_b, _e, _cb) {} }),
    input = new TerminalInput(),
    writer = new TerminalWriter(stalled, 5),
    g = new KittyGraphics(input, writer, 5);
  await expect(g.upload(new Uint8Array(2), () => true)).rejects.toThrow();
  await expect(g.close()).rejects.toThrow();
  expect(g.cleanupWritten).toBe(false);
  input.close();
});
