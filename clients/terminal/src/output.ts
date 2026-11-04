import {
  constants,
  linkSync,
  lstatSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  renameSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import {
  basename,
  dirname,
  isAbsolute,
  relative,
  resolve,
  sep,
} from "node:path";
import { Failure, type Json } from "./types.ts";
import buildRoot from "../generated/build-root.txt" with { type: "text" };
export function outputPath(
  input: string,
  workspace: string,
  replace = false,
): string {
  const path = resolve(input),
    parent = dirname(path);
  if (realpathSync(parent) !== parent)
    throw new Failure({
      code: "Usage",
      message: "output parent must be canonical; symlink traversal refused",
    });
  const w = JSON.parse(readFileSync(workspace, "utf8"));
  for (const m of w.models) {
    const store = realpathSync(resolve(dirname(workspace), m.directory));
    if (path === store || path.startsWith(store + sep))
      throw new Failure({
        code: "Usage",
        message: "output inside a native store is forbidden",
      });
  }
  const repo = buildRoot.trim();
  for (const name of ["hub", "old_hub"]) {
    const store = resolve(repo, name);
    if (path === store || path.startsWith(store + sep))
      throw new Failure({
        code: "Usage",
        message: "protected historical store",
      });
  }
  try {
    const stat = lstatSync(path);
    if (!stat.isFile() || stat.isSymbolicLink() || stat.nlink !== 1)
      throw new Failure({
        code: "Usage",
        message: "output is not an ordinary single-link file",
      });
    if (!replace)
      throw new Failure({
        code: "AlreadyExists",
        message: "output exists; use --replace explicitly",
      });
  } catch (e: Json) {
    if (e.code !== "ENOENT") throw e;
  }
  return path;
}
export function writeOutput(
  input: string,
  data: string | Uint8Array,
  workspace: string,
  replace = false,
) {
  const path = outputPath(input, workspace, replace),
    temp = resolve(
      dirname(path),
      `.${basename(path)}.${crypto.randomUUID()}.tmp`,
    );
  writeFileSync(temp, data, { flag: "wx", mode: 0o600 });
  try {
    if (replace) {
      outputPath(path, workspace, true);
      renameSync(temp, path);
    } else {
      linkSync(temp, path);
      unlinkSync(temp);
    }
  } catch (e) {
    try {
      unlinkSync(temp);
    } catch {}
    throw e;
  }
  return path;
}
