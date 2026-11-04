import { dirname, resolve } from "node:path";
import { existsSync } from "node:fs";
import { Failure, historyMethods, type Json } from "./types.ts";
export const protocol = "knowledgehub.client/1",
  frameLimit = 8 * 1024 * 1024;
type Pending = {
  resolve: (x: Json) => void;
  reject: (e: unknown) => void;
  remove?: () => void;
};
export function encodeFrame(value: Json): Buffer {
  const body = Buffer.from(JSON.stringify(value));
  if (!body.length || body.length > frameLimit)
    throw new Failure({
      code: "Capacity",
      message: "request frame exceeds 8 MiB",
    });
  const n = Buffer.alloc(4);
  n.writeUInt32BE(body.length);
  return Buffer.concat([n, body]);
}
export class Decoder {
  private bytes = Buffer.alloc(0);
  push(chunk: Uint8Array): Json[] {
    this.bytes = Buffer.concat([this.bytes, chunk]);
    const out: Json[] = [];
    while (this.bytes.length >= 4) {
      const n = this.bytes.readUInt32BE(0);
      if (!n || n > frameLimit)
        throw new Failure({
          code: "Protocol",
          message: "invalid frame length",
        });
      if (this.bytes.length < n + 4) break;
      let j;
      try {
        j = JSON.parse(
          new TextDecoder("utf-8", { fatal: true }).decode(
            this.bytes.subarray(4, n + 4),
          ),
        );
      } catch {
        throw new Failure({ code: "Protocol", message: "malformed host JSON" });
      }
      out.push(j);
      this.bytes = this.bytes.subarray(n + 4);
    }
    return out;
  }
  end() {
    if (this.bytes.length)
      throw new Failure({ code: "Protocol", message: "truncated host frame" });
  }
}
export function defaultHost(): string {
  const adjacent = resolve(
    dirname(process.execPath),
    "knowledgehub-client-host",
  );
  return existsSync(adjacent)
    ? adjacent
    : resolve(
        import.meta.dir,
        "../../../target/debug/knowledgehub-client-host",
      );
}
export class Transport {
  private child: ReturnType<typeof Bun.spawn>;
  private pending = new Map<string, Pending>();
  private counter = 0;
  private failure?: Failure;
  private closing = false;
  private pump: Promise<void>;
  public wireBytes = 0;
  public calls = 0;
  public capabilities: Json;
  constructor(
    public workspace: string,
    host = process.env.KNOWLEDGEHUB_CLIENT_HOST ?? defaultHost(),
    private drainTimeoutMs = 30000,
  ) {
    if (
      !Number.isSafeInteger(drainTimeoutMs) ||
      drainTimeoutMs < 1 ||
      drainTimeoutMs > 30000
    )
      throw new Failure({
        code: "Usage",
        message: "finite drain timeout must be 1..30000 ms",
      });
    this.child = Bun.spawn([host, "--workspace", resolve(workspace)], {
      stdin: "pipe",
      stdout: "pipe",
      stderr: "inherit",
    });
    this.pump = this.read();
  }
  private fail(error: unknown) {
    const e =
      error instanceof Failure
        ? error
        : new Failure({ code: "Disconnected", message: String(error) });
    this.failure = e;
    for (const p of this.pending.values()) {
      p.remove?.();
      p.reject(e);
    }
    this.pending.clear();
  }
  private async read() {
    const decoder = new Decoder();
    try {
      for await (const bytes of this.child
        .stdout as ReadableStream<Uint8Array>) {
        this.wireBytes += bytes.length;
        for (const j of decoder.push(bytes)) {
          if (
            j.protocol !== protocol ||
            typeof j.id !== "string" ||
            typeof j.ok !== "boolean" ||
            (!j.ok && !j.error)
          )
            throw new Failure({
              code: "Protocol",
              message: "incompatible response",
            });
          const p = this.pending.get(j.id);
          if (!p)
            throw new Failure({
              code: "Protocol",
              message: "uncorrelated/duplicate response",
            });
          this.pending.delete(j.id);
          p.remove?.();
          j.ok ? p.resolve(j.data) : p.reject(new Failure(j.error));
        }
      }
      decoder.end();
      if (this.pending.size || !this.closing)
        throw new Failure({
          code: "Disconnected",
          message: "host EOF; restart the exact pinned intent",
        });
    } catch (e) {
      this.fail(e);
      this.child.kill();
    }
  }
  async handshake() {
    const c = await this.call("handshake", { protocol });
    if (
      c.protocol !== protocol ||
      c.build?.history_projection !== "knowledgehub-history/1" ||
      c.build?.record_codec !== "knowledgehub-record/3" ||
      c.build?.request_codec !== "knowledgehub-request/3" ||
      !historyMethods.every((m) => c.methods.includes(m))
    )
      throw new Failure({
        code: "Incompatible",
        message: "host method/DTO contract mismatch",
      });
    this.capabilities = c;
    return c;
  }
  async call(
    method: string,
    args: Json = {},
    model?: string,
    signal?: AbortSignal,
    resultBytes?: number,
  ): Promise<Json> {
    if (this.failure) throw this.failure;
    if (this.closing && method !== "drain")
      throw new Failure({ code: "Disconnected", message: "host is draining" });
    if (this.pending.size >= 16)
      throw new Failure({
        code: "Busy",
        message: "16 admitted operations pending",
      });
    if (signal?.aborted) throw new Failure({ code: "Cancelled" });
    const id = String(++this.counter),
      bytes = encodeFrame({
        id,
        method,
        args,
        ...(model ? { model } : {}),
        ...(resultBytes ? { result_bytes: String(resultBytes) } : {}),
      });
    this.calls++;
    return new Promise((resolve, reject) => {
      const p: Pending = { resolve, reject };
      if (signal) {
        const abort = () =>
          reject(
            new Failure({
              code: "Cancelled",
              message:
                "caller cancelled; admitted native work drains before close",
            }),
          );
        signal.addEventListener("abort", abort, { once: true });
        p.remove = () => signal.removeEventListener("abort", abort);
      }
      this.pending.set(id, p);
      try {
        (this.child.stdin as import("bun").FileSink).write(bytes);
        (this.child.stdin as import("bun").FileSink).flush();
      } catch (e) {
        this.fail(e);
      }
    });
  }
  async close() {
    if (this.closing) return;
    this.closing = true;
    // Cancellation only detaches callers. Keep their slots until the host replies.
    let deadlineFailure: Failure | undefined;
    const deadline = Date.now() + this.drainTimeoutMs;
    while (this.pending.size && !this.failure && Date.now() < deadline)
      await Bun.sleep(5);
    if (this.pending.size) {
      deadlineFailure = new Failure({
        code: "Disconnected",
        message: "drain deadline; no completion claimed",
      });
      this.fail(deadlineFailure);
      this.child.kill();
    }
    if (!this.failure) {
      try {
        await this.call("drain");
      } catch (e) {
        this.fail(e);
      }
    }
    (this.child.stdin as import("bun").FileSink).end();
    await this.pump;
    await this.child.exited;
    if (deadlineFailure) throw deadlineFailure;
  }
}
