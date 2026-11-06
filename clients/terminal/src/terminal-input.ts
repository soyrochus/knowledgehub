import { EventEmitter } from "node:events";
import { PassThrough, type Readable } from "node:stream";
import { Failure } from "./types.ts";
export type Reply = {
  kind: "graphics" | "pixels" | "cells" | "device";
  id?: number;
  payload?: string;
  values?: number[];
};
/** One raw input owner, also used by the text browser. No probes are sent here.
 * Recognized replies stay filtered after a viewer closes (including stale IDs).
 */
export class TerminalInput extends EventEmitter {
  readonly keys = new PassThrough();
  private buffer = "";
  private discarding = false;
  private escapeTimer?: ReturnType<typeof setTimeout>;
  private fragmentTimer?: ReturnType<typeof setTimeout>;
  private rawListener = (chunk: Buffer) => this.feed(chunk);
  constructor(
    private raw?: Readable,
    private fragmentDeadline = 1000,
  ) {
    super();
    raw?.on("data", this.rawListener);
  }
  get retainedBytes() {
    return this.buffer.length;
  }
  feed(chunk: Uint8Array) {
    clearTimeout(this.escapeTimer);
    const incoming = Buffer.from(chunk).toString("latin1");
    // Ctrl-C is never valid inside a graphics response. Preserve cancellation
    // even when the terminal left a malformed APC unterminated.
    if (
      (this.discarding ||
        this.buffer.startsWith("\x1b_G") ||
        incoming.startsWith("\x1b_G")) &&
      incoming.includes("\x03")
    ) {
      this.buffer = "";
      this.discarding = false;
      this.clearFragment();
      this.key("\x03");
      const rest = incoming.slice(incoming.lastIndexOf("\x03") + 1);
      if (rest) this.feed(Buffer.from(rest, "latin1"));
      return;
    }
    this.buffer += incoming;
    while (this.buffer.length) {
      if (this.discarding) {
        const end = this.buffer.indexOf("\x1b\\");
        if (end < 0) {
          this.buffer = this.buffer.endsWith("\x1b") ? "\x1b" : "";
          this.holdFragment();
          return;
        }
        this.buffer = this.buffer.slice(end + 2);
        this.discarding = false;
        continue;
      }
      const esc = this.buffer.indexOf("\x1b");
      if (esc < 0) {
        this.key(this.buffer);
        this.buffer = "";
        break;
      }
      if (esc > 0) {
        this.key(this.buffer.slice(0, esc));
        this.buffer = this.buffer.slice(esc);
        continue;
      }
      if (this.buffer === "\x1b") {
        this.escapeTimer = setTimeout(() => {
          if (this.buffer === "\x1b") {
            this.buffer = "";
            this.key("\x1b");
          }
        }, 30);
        break;
      }
      if ("\x1b_G".startsWith(this.buffer)) break;
      if (this.buffer.startsWith("\x1b_G")) {
        const end = this.buffer.indexOf("\x1b\\", 3);
        if (end < 0) {
          if (this.buffer.length > 65536) {
            this.discarding = true;
            this.buffer = this.buffer.endsWith("\x1b") ? "\x1b" : "";
          }
          break;
        }
        const frame = this.buffer.slice(3, end);
        this.buffer = this.buffer.slice(end + 2);
        if (frame.length > 65536) continue;
        const match = /^([^;]*);([^\x00-\x1f]*)$/.exec(frame);
        if (match) {
          const id = /(?:^|,)i=([0-9]+)(?:,|$)/.exec(match[1]);
          if (id && Number(id[1]) > 0 && Number(id[1]) <= 0xffffffff)
            this.emit("reply", {
              kind: "graphics",
              id: Number(id[1]),
              payload: match[2],
            } satisfies Reply);
        }
        continue;
      }
      if (this.buffer.startsWith("\x1b[")) {
        if (this.buffer.length === 2) break;
        const frame = /^\x1b\[([0-?]*[ -/]*)([@-~])/.exec(this.buffer);
        if (!frame) {
          if (this.buffer.length > 65536) {
            this.key("\x1b[");
            this.buffer = this.buffer.slice(2);
          }
          break;
        }
        this.buffer = this.buffer.slice(frame[0].length);
        const params = frame[1];
        if (frame[2] === "t" && /^(4|8);[0-9]+;[0-9]+$/.test(params)) {
          const values = params.split(";").map(Number);
          this.emit("reply", {
            kind: values[0] === 4 ? "pixels" : "cells",
            values: values.slice(1),
          } satisfies Reply);
        } else if (frame[2] === "t" && /^(4|8);/.test(params)) {
          /* malformed geometry is never keyboard input */
        } else if (frame[2] === "c" && /^\?[0-9;]*$/.test(params))
          this.emit("reply", { kind: "device" } satisfies Reply);
        else this.key(frame[0]);
        continue;
      }
      this.key(this.buffer.slice(0, 2));
      this.buffer = this.buffer.slice(2);
    }
    if (this.buffer.length > 1 || this.discarding) this.holdFragment();
    else if (!this.buffer.length) this.clearFragment();
  }
  private clearFragment() {
    clearTimeout(this.fragmentTimer);
    this.fragmentTimer = undefined;
  }
  private holdFragment() {
    if (this.fragmentTimer) return;
    this.fragmentTimer = setTimeout(() => {
      const pending = this.buffer;
      const response =
        this.discarding ||
        pending.startsWith("\x1b_") ||
        /^\x1b\[(?:[48];|\?)/.test(pending);
      this.buffer = "";
      this.discarding = false;
      this.fragmentTimer = undefined;
      // Expired malformed response bytes are never replayed as browser keys.
      // Unknown keyboard prefixes belong to readline's own key decoder.
      if (!response && pending) this.key(pending);
    }, this.fragmentDeadline);
  }
  private key(s: string) {
    this.keys.write(Buffer.from(s, "latin1"));
  }
  wait(
    predicate: (reply: Reply) => boolean,
    timeout = 1000,
    signal?: AbortSignal,
  ): Promise<Reply> {
    if (signal?.aborted)
      return Promise.reject(new Failure({ code: "Cancelled" }));
    if (this.listenerCount("reply") >= 8)
      return Promise.reject(
        new Failure({
          code: "GraphicsTransport",
          message: "reply waiter limit",
        }),
      );
    return new Promise((resolve, reject) => {
      const finish = (reply?: Reply, error?: unknown) => {
        clearTimeout(timer);
        this.off("reply", onReply);
        signal?.removeEventListener("abort", onAbort);
        error ? reject(error) : resolve(reply!);
      };
      const onReply = (reply: Reply) => {
        if (predicate(reply)) finish(reply);
      };
      const onAbort = () =>
        finish(undefined, new Failure({ code: "Cancelled" }));
      const timer = setTimeout(
        () =>
          finish(
            undefined,
            new Failure({
              code: "GraphicsUnavailable",
              message: "terminal response deadline (1 second)",
            }),
          ),
        timeout,
      );
      this.on("reply", onReply);
      signal?.addEventListener("abort", onAbort, { once: true });
    });
  }
  close() {
    clearTimeout(this.escapeTimer);
    this.clearFragment();
    this.raw?.off("data", this.rawListener);
    this.keys.end();
    this.removeAllListeners();
    this.buffer = "";
  }
}
