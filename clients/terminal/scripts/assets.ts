// The compiled executable has a virtual import.meta.dir. Retain the actual
// protected build-workspace boundary as embedded text, without runtime reads.
import { mkdirSync, realpathSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const root = realpathSync(resolve(import.meta.dir, "../../.."));
mkdirSync(resolve(root, "clients/terminal/generated"), { recursive: true });
writeFileSync(
  resolve(root, "clients/terminal/generated/build-root.txt"),
  root + "\n",
);
const p = Bun.spawn(
  [
    process.execPath,
    "build",
    "src/renderer.ts",
    "--target",
    "browser",
    "--format",
    "iife",
    "--minify",
    "--outfile",
    "generated/renderer.js",
  ],
  {
    cwd: resolve(root, "clients/terminal"),
    stdout: "inherit",
    stderr: "inherit",
  },
);
if (await p.exited) throw Error("offline renderer bundle failed");
