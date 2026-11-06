// Explicit compile entry. Loaded only by an explicitly invoked graphics view.
import { initWasm, Resvg } from "@resvg/resvg-wasm";
import wasm from "@resvg/resvg-wasm/index_bg.wasm" with { type: "file" };
import font from "../assets/graphics/DejaVuSans.ttf" with { type: "file" };
import { createHash } from "node:crypto";
import {
  checkDisplay,
  checkViewport,
  svgViewport,
  limits,
} from "./graph-layout.ts";
import { Failure } from "./types.ts";

const scope = self as unknown as Worker;
let initialization: Promise<Uint8Array> | undefined;
async function initialize() {
  const bytes = new Uint8Array(await Bun.file(font).arrayBuffer());
  if (
    createHash("sha256").update(bytes).digest("hex") !==
    "7da195a74c55bef988d0d48f9508bd5d849425c1770dba5d7bfc6ce9ed848954"
  )
    throw Error("bundled DejaVu Sans integrity failure");
  const wasmBytes = await Bun.file(wasm).arrayBuffer();
  if (
    createHash("sha256").update(new Uint8Array(wasmBytes)).digest("hex") !==
    "22bf6e9f9a100d972da0411a69c5ba504367fc1fa87b3b64e3f35e53926d2d70"
  )
    throw Error("bundled resvg WASM integrity failure");
  await initWasm(wasmBytes);
  return bytes;
}
scope.onmessage = async (event: MessageEvent) => {
  let phase = "initialization";
  try {
    if (!event.data.checkpoint) {
      checkDisplay(event.data.display);
      checkViewport(event.data.viewport);
    }
    const drawing = event.data.checkpoint
      ? undefined
      : svgViewport(
          event.data.display,
          event.data.viewport,
          event.data.selected,
          event.data.center,
        );
    const fontBytes = await (initialization ??= initialize());
    const svg =
      drawing?.svg ??
      '<svg xmlns="http://www.w3.org/2000/svg" width="640" height="160"><rect width="640" height="160" fill="#f4f7fa"/><text x="24" y="80" font-family="DejaVu Sans" font-size="24">KnowledgeHub · labelled worker checkpoint</text></svg>';
    const renderer = new Resvg(svg, {
      font: {
        fontBuffers: [fontBytes],
        defaultFontFamily: "DejaVu Sans",
        loadSystemFonts: false,
      },
    });
    let rendered;
    let result;
    let failed: unknown;
    try {
      phase = "rasterization";
      rendered = renderer.render();
      const width = rendered.width,
        height = rendered.height;
      const png = Uint8Array.from(rendered.asPng());
      if (png.length > limits.png)
        throw new Failure({
          code: "RenderLimit",
          message: "PNG exceeds 8 MiB",
        });
      result = {
        id: event.data.id,
        png,
        width,
        height,
        scale: drawing?.scale,
        offset: drawing?.offset,
        geometry: drawing?.geometry,
      };
    } catch (error) {
      failed = error;
    } finally {
      try {
        rendered?.free();
        renderer.free();
      } catch (error) {
        if (!failed) {
          phase = "resource release";
          failed = error;
        }
      }
    }
    if (failed) throw failed;
    // No WASM resources remain live when the UI receives/terminates this worker.
    scope.postMessage(result);
  } catch (error) {
    scope.postMessage({
      id: event.data.id,
      error:
        error instanceof Failure
          ? error.detail
          : { code: "GraphicsAsset", message: `${phase}: ${String(error)}` },
    });
  }
};
