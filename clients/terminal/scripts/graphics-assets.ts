import inventory from "../assets/graphics/inventory.json";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const root = resolve(import.meta.dir, "..");
for (const [file, hash] of Object.entries(inventory.sha256)) {
  const bytes = await Bun.file(resolve(root, file)).arrayBuffer();
  if (createHash("sha256").update(new Uint8Array(bytes)).digest("hex") !== hash)
    throw Error(`graphics asset hash mismatch: ${file}`);
}
for (const pkg of inventory.packages) {
  const metadata = await Bun.file(
    resolve(root, "node_modules", pkg.name, "package.json"),
  ).json();
  if (metadata.version !== pkg.version)
    throw Error(`graphics version mismatch: ${pkg.name}`);
}
console.log(
  JSON.stringify({
    status: "pass",
    assets: Object.keys(inventory.sha256).length,
    packages: inventory.packages,
  }),
);
