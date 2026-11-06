// Maintainer checkpoint; compile together with graph-render-worker.ts.
import { GraphRenderer } from "../src/graph-render.ts";
const renderer = new GraphRenderer();
try {
  const frame = await renderer.render({ checkpoint: true });
  if (
    frame.width !== 640 ||
    frame.height !== 160 ||
    Buffer.from(frame.png).subarray(0, 8).toString("hex") !== "89504e470d0a1a0a"
  )
    throw Error("invalid PNG checkpoint");
  if (process.argv[2]) await Bun.write(process.argv[2], frame.png);
  const pending = renderer.render({ checkpoint: true }).then(
    () => false,
    () => true,
  );
  renderer.close();
  if (!(await pending)) throw Error("cancellation failed");
  console.log(
    JSON.stringify({
      status: "pass",
      bun: Bun.version,
      png_bytes: frame.png.length,
      width: frame.width,
      height: frame.height,
      cancelled: true,
    }),
  );
} finally {
  renderer.close();
}
