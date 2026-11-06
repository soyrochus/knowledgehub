import { Failure, type Json } from "./types.ts";

export type Frame = {
  png: Uint8Array;
  width: number;
  height: number;
  scale?: number;
  offset?: { x: number; y: number };
  geometry?: { width: number; height: number };
};
type Request = {
  id: number;
  input: Json;
  resolve: (v: Frame) => void;
  reject: (e: unknown) => void;
};
/** At most one active worker and one latest intent; never reveal obsolete work. */
export class GraphRenderer {
  private generation = 0;
  private active?: {
    request: Request;
    worker: Worker;
    timer: ReturnType<typeof setTimeout>;
  };
  private latest?: Request;
  private closed = false;
  constructor(private deadline = 5000) {
    if (!Number.isInteger(deadline) || deadline < 1 || deadline > 5000)
      throw new Failure({
        code: "Usage",
        message: "worker deadline must be 1..5000 ms",
      });
  }
  render(input: Json): Promise<Frame> {
    if (this.closed) return Promise.reject(new Failure({ code: "Cancelled" }));
    if (Buffer.byteLength(JSON.stringify(input)) > 2 * 1024 * 1024)
      return Promise.reject(
        new Failure({
          code: "RenderLimit",
          message: "worker input exceeds 2 MiB",
        }),
      );
    this.latest?.reject(
      new Failure({ code: "Cancelled", message: "coalesced viewport" }),
    );
    return new Promise((resolve, reject) => {
      this.latest = { id: ++this.generation, input, resolve, reject };
      queueMicrotask(() => this.pump());
    });
  }
  private pump() {
    if (this.closed || this.active || !this.latest) return;
    const request = this.latest;
    this.latest = undefined;
    let worker: Worker;
    // The compile recipe fixes --root clients/terminal. Bun 1.4.0 bundles
    // entrypoints as .js under that root, rather than beside the executable.
    const workerPath = import.meta.dir.startsWith("/$bunfs/")
      ? new URL("./src/graph-render-worker.js", import.meta.url)
      : new URL("./graph-render-worker.ts", import.meta.url);
    try {
      worker = new Worker(workerPath.href);
    } catch (e) {
      request.reject(
        new Failure({ code: "GraphicsAsset", message: String(e) }),
      );
      return;
    }
    const finish = (frame?: Frame, error?: unknown) => {
      if (this.active?.request !== request) return;
      clearTimeout(this.active.timer);
      this.active = undefined;
      worker.terminate();
      if (request.id !== this.generation)
        request.reject(
          new Failure({ code: "Cancelled", message: "obsolete frame" }),
        );
      else if (error) request.reject(error);
      else request.resolve(frame!);
      this.pump();
    };
    const timer = setTimeout(
      () =>
        finish(
          undefined,
          new Failure({
            code: "RenderLimit",
            message: "layout/raster 5 second deadline reached",
          }),
        ),
      this.deadline,
    );
    this.active = { request, worker, timer };
    worker.onmessage = (event: MessageEvent) => {
      if (event.data.id !== request.id) return;
      if (event.data.error) finish(undefined, new Failure(event.data.error));
      else finish(event.data);
    };
    worker.onerror = (event) => {
      event.preventDefault();
      finish(
        undefined,
        new Failure({ code: "GraphicsAsset", message: event.message }),
      );
    };
    worker.postMessage({ ...request.input, id: request.id });
  }
  cancel() {
    this.generation++;
    this.latest?.reject(new Failure({ code: "Cancelled" }));
    this.latest = undefined;
    if (this.active) {
      clearTimeout(this.active.timer);
      this.active.worker.terminate();
      this.active.request.reject(new Failure({ code: "Cancelled" }));
      this.active = undefined;
    }
  }
  close() {
    this.closed = true;
    this.cancel();
  }
}
