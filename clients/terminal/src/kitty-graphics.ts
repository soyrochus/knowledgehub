import type { Writable } from "node:stream";
import { TerminalInput, type Reply } from "./terminal-input.ts";
import { Failure, clean } from "./types.ts";
// ED2/ED3 delete graphics too. ED0 from home only erases text, retaining
// uploaded image data and unrelated applications' placements.
export const textScreen = "\x1b[H\x1b[0J";
export const apc = (controls: string, payload = "") =>
  `\x1b_G${controls};${payload}\x1b\\`;
let nextId = ((process.pid & 0x3fff) << 16) + 1;
export function ownedId() {
  return ++nextId;
}
export type Geometry = {
  columns: number;
  rows: number;
  width: number;
  height: number;
};
/** All viewer text, image chunks and deletes share one serialized writer. */
export class TerminalWriter {
  private tail: Promise<void> = Promise.resolve();
  failure?: Failure;
  constructor(
    private output: Writable,
    private timeout = 2000,
  ) {}
  transaction(
    work: (
      write: (bytes: string | Uint8Array) => Promise<void>,
    ) => Promise<void>,
  ): Promise<void> {
    const result = this.tail.then(async () => {
      if (this.failure) throw this.failure;
      await work((bytes) => this.writeNow(bytes));
    });
    this.tail = result.catch(() => {});
    return result;
  }
  write(bytes: string | Uint8Array) {
    return this.transaction(async (write) => write(bytes));
  }
  private writeNow(bytes: string | Uint8Array): Promise<void> {
    return new Promise((resolve, reject) => {
      let settled = false;
      const finish = (error?: unknown) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        this.output.off("error", onError);
        if (error) {
          this.failure = new Failure({
            code: "GraphicsOutput",
            message: String(error),
          });
          reject(this.failure);
        } else resolve();
      };
      const onError = (e: unknown) => finish(e);
      const timer = setTimeout(
        () =>
          finish(Error("output stalled for 2 seconds; cleanup not certified")),
        this.timeout,
      );
      this.output.once("error", onError);
      try {
        this.output.write(bytes, (e) => finish(e));
      } catch (e) {
        finish(e);
      }
    });
  }
  async idle() {
    await this.tail;
    if (this.failure) throw this.failure;
  }
}
export class KittyGraphics {
  private queryId = ownedId();
  private images = new Set<number>();
  private placement = ownedId();
  private visible?: number;
  cleanupWritten = false;
  constructor(
    readonly input: TerminalInput,
    readonly writer: TerminalWriter,
    private deadline = 1000,
  ) {}
  async probe(
    signal?: AbortSignal,
    env: NodeJS.ProcessEnv = process.env,
  ): Promise<Geometry> {
    if (env.TMUX || env.STY)
      throw new Failure({
        code: "GraphicsUnavailable",
        message:
          "direct terminal required; tmux/screen passthrough is outside this viewer",
      });
    const waiting = this.input.wait(
      (r) => r.kind === "graphics" && r.id === this.queryId,
      this.deadline,
      signal,
    );
    // Attach rejection handlers before output awaits: cancellation cannot leak a rejection.
    const response = waiting.then(
      (r) => r,
      (e) => {
        throw e;
      },
    );
    void response.catch(() => {});
    try {
      await this.writer.write(
        apc(`a=q,t=d,f=32,s=1,v=1,i=${this.queryId}`, "AAAAAA==") + "\x1b[c",
      );
      if ((await response).payload !== "OK")
        throw new Failure({
          code: "GraphicsUnavailable",
          message: "terminal refused the Kitty graphics query",
        });
      return await this.geometry(signal);
    } finally {
      await this.writer.write(apc(`a=d,d=I,i=${this.queryId},q=2`));
    }
  }
  async geometry(signal?: AbortSignal): Promise<Geometry> {
    const pixel = this.input.wait(
      (r) => r.kind === "pixels",
      this.deadline,
      signal,
    );
    const cells = this.input.wait(
      (r) => r.kind === "cells",
      this.deadline,
      signal,
    );
    const both = Promise.all([pixel, cells]);
    void both.catch(() => {});
    await this.writer.write("\x1b[14t\x1b[18t");
    const [p, c] = await both,
      [height, width] = p.values!,
      [rows, columns] = c.values!;
    if (
      ![width, height, rows, columns].every(
        (n) => Number.isSafeInteger(n) && n > 0 && n <= 65535,
      ) ||
      width < columns ||
      height < rows
    )
      throw new Failure({
        code: "GraphicsUnavailable",
        message: "unusable terminal cell/pixel geometry",
      });
    return { width, height, rows, columns };
  }
  async hide() {
    if (this.visible !== undefined) {
      const id = this.visible;
      this.visible = undefined;
      await this.writer.write(apc(`a=d,d=i,i=${id},p=${this.placement},q=2`));
    }
  }
  /** Upload without placing. Reveal is a separate, freshly authorized operation. */
  async upload(
    png: Uint8Array,
    valid: () => boolean,
    signal?: AbortSignal,
  ): Promise<number> {
    if (
      png.length > 8 * 1024 * 1024 ||
      Math.ceil(png.length / 3) * 4 > 12 * 1024 * 1024
    )
      throw new Failure({ code: "RenderLimit", message: "PNG/frame bound" });
    if (this.images.size >= 2)
      throw new Failure({
        code: "GraphicsTransport",
        message: "two owned image limit",
      });
    const id = ownedId();
    this.images.add(id);
    const ack = this.input.wait(
      (r) => r.kind === "graphics" && r.id === id,
      this.deadline,
      signal,
    );
    void ack.catch(() => {});
    await this.writer.transaction(async (write) => {
      const encoded = Buffer.from(png).toString("base64");
      for (let offset = 0; offset < encoded.length; offset += 4096) {
        if (!valid() || signal?.aborted)
          throw new Failure({ code: "Cancelled" });
        const first = offset === 0,
          more = offset + 4096 < encoded.length;
        await write(
          apc(
            `${first ? `a=t,t=d,f=100,i=${id},q=0,` : ""}m=${more ? 1 : 0}`,
            encoded.slice(offset, offset + 4096),
          ),
        );
      }
    });
    const reply = await ack;
    if (reply.payload !== "OK")
      throw new Failure({
        code: "GraphicsTransport",
        message: `terminal rejected image: ${reply.payload}`,
      });
    return id;
  }
  async reveal(
    id: number,
    columns: number,
    rows: number,
    valid: () => boolean,
    signal?: AbortSignal,
    screen = "",
  ) {
    if (!valid() || signal?.aborted) throw new Failure({ code: "Cancelled" });
    const ack = this.input.wait(
      (r) => r.kind === "graphics" && r.id === id,
      this.deadline,
      signal,
    );
    void ack.catch(() => {});
    this.visible = id; // placement may exist even if its write/ack later fails
    await this.writer.write(
      screen +
        `\x1b[5;1H` +
        apc(`a=p,i=${id},p=${this.placement},c=${columns},r=${rows},C=1,q=0`),
    );
    const reply = await ack;
    if (reply.payload !== "OK")
      throw new Failure({
        code: "GraphicsTransport",
        message: `terminal rejected placement: ${clean(reply.payload ?? "missing response").slice(0, 512)}`,
        image_id: id,
        placement_id: this.placement,
      });
  }
  async clear() {
    this.visible = undefined;
    this.cleanupWritten = false;
    const ids = [...this.images];
    await this.writer.write(
      ids.map((id) => apc(`a=d,d=I,i=${id},q=2`)).join(""),
    );
    ids.forEach((id) => this.images.delete(id));
    this.cleanupWritten = true;
  }
  async retain(id: number) {
    const old = [...this.images].filter((n) => n !== id);
    if (old.length)
      await this.writer.write(
        old.map((n) => apc(`a=d,d=I,i=${n},q=2`)).join(""),
      );
    old.forEach((n) => this.images.delete(n));
  }
  async close() {
    await this.clear();
    await this.writer.idle();
  }
}
