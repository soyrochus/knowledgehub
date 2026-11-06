// Maintainer tool. End users receive two native executables and quiescent stores.
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  statSync,
  writeFileSync,
  chmodSync,
  realpathSync,
} from "node:fs";
import { resolve, join, relative } from "node:path";
import { createHash } from "node:crypto";
const root = resolve(import.meta.dir, "../../.."),
  destination = resolve(process.argv[2] ?? "");
const retainedModels =
  process.argv[3] === "--models" && process.argv[4]
    ? realpathSync(resolve(process.argv[4]))
    : undefined;
if (
  ![3, 5].includes(process.argv.length) ||
  (process.argv.length === 5 && !retainedModels) ||
  existsSync(destination)
)
  throw Error(
    "usage: bun scripts/package.ts NEW-DESTINATION [--models QUIESCENT-MODELS-DIR] (occupied paths refused)",
  );
if (process.platform !== "darwin" || process.arch !== "arm64")
  throw Error("This build recipe currently verifies macOS arm64 only.");
mkdirSync(destination, { recursive: true });
async function run(cmd: string[], cwd = root) {
  const p = Bun.spawn(cmd, { cwd, stdout: "inherit", stderr: "inherit" });
  if (await p.exited) throw Error(`build failed: ${cmd[0]}`);
}
await run([
  "cargo",
  "build",
  "--release",
  "--bin",
  "knowledgehub-client-host",
  "--bin",
  "knowledgehub-model",
  "--example",
  "model_successor",
  "--example",
  "terminal_fixture_handover",
]);
await run(["bun", "run", "assets"], join(root, "clients/terminal"));
await run(["bun", "clients/terminal/scripts/graphics-assets.ts"]);
await run([
  "bun",
  "build",
  "--compile",
  "--target=bun-darwin-arm64",
  "--root",
  "clients/terminal",
  "clients/terminal/src/kh.ts",
  "clients/terminal/src/graph-render-worker.ts",
  "--outfile",
  join(destination, "kh"),
]);
cpSync(
  join(root, "target/release/knowledgehub-client-host"),
  join(destination, "knowledgehub-client-host"),
);
chmodSync(join(destination, "knowledgehub-client-host"), 0o755);
if (retainedModels) {
  if (
    ["hub", "old_hub"].some(
      (name) =>
        retainedModels === join(root, name) ||
        retainedModels.startsWith(join(root, name) + "/"),
    ) ||
    destination.startsWith(retainedModels + "/")
  )
    throw Error("protected or recursive model copy refused");
  if (
    !existsSync(join(retainedModels, "workspace.json")) ||
    !existsSync(join(retainedModels, "workspace-no-history.json"))
  )
    throw Error("both retained workspace files required");
  const ordinary = (dir: string) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isSymbolicLink()) throw Error("retained model symlink refused");
      if (entry.isDirectory()) ordinary(join(dir, entry.name));
      else if (!entry.isFile())
        throw Error("retained model special file refused");
    }
  };
  ordinary(retainedModels);
  cpSync(retainedModels, join(destination, "models"), {
    recursive: true,
    errorOnExist: true,
  });
} else
  await run([
    "python3",
    "scripts/prepare-terminal-models.py",
    join(destination, "models"),
    "--release",
  ]);
// Publication and source evidence is portable. Preparation journals retain local
// build paths, so ship only the source-bound reference inputs and exact handover.
for (const name of ["fruittrucks", "equipment"]) {
  const model = join(destination, "models", name);
  const allowed = new Set([
    "hub",
    "reference",
    "handover.json",
    "successor.json",
  ]);
  for (const file of readdirSync(model))
    if (!allowed.has(file))
      await run([
        "/bin/mv",
        join(model, file),
        join(destination, `.build-${name}-${file}`),
      ]);
}
// Store maintainer logs beside the package, not inside its end-user inventory.
const logdir = destination + ".build-evidence";
mkdirSync(logdir);
for (const file of readdirSync(destination))
  if (file.startsWith(".build-"))
    await run([
      "/bin/mv",
      join(destination, file),
      join(logdir, file.slice(1)),
    ]);
cpSync(
  join(root, "examples/fixtures/terminal-client/QUICKSTART.md"),
  join(destination, "README.md"),
);
cpSync(join(root, "LICENSE"), join(destination, "LICENSE"));
cpSync(
  join(root, "clients/terminal/node_modules/mermaid/LICENSE"),
  join(destination, "MERMAID-LICENSE"),
);
writeFileSync(
  join(destination, "THIRD-PARTY.md"),
  "Offline Mermaid 12.3.0, Dagre 3.1.3/graphlib 4.0.6 (MIT), resvg-WASM 2.6.2 (MPL-2.0), unmodified DejaVu Sans 2.37 and transitive dependencies are bundled under their licenses. Bun 1.4.0 is embedded in kh. See build-manifest.json, dependency-licenses/ and graphics-notices/ for retained notices, exact hashes and corresponding source locations.\n",
);
cpSync(
  join(root, "clients/terminal/assets/graphics"),
  join(destination, "graphics-notices"),
  { recursive: true },
);
mkdirSync(join(destination, "dependency-licenses"));
function notices(dir: string) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === ".bin") continue;
    const p = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (existsSync(join(p, "package.json"))) {
        const metadata = JSON.parse(
          readFileSync(join(p, "package.json"), "utf8"),
        );
        const out = join(
          destination,
          "dependency-licenses",
          String(metadata.name).replace(/[^a-zA-Z0-9_.-]/g, "_"),
        );
        mkdirSync(out, { recursive: true });
        writeFileSync(
          join(out, "package.json"),
          JSON.stringify(
            {
              name: metadata.name,
              version: metadata.version,
              license: metadata.license,
            },
            null,
            2,
          ),
        );
        for (const f of readdirSync(p))
          if (
            /^(licen[cs]e|notice|copying)/i.test(f) &&
            statSync(join(p, f)).isFile()
          )
            cpSync(join(p, f), join(out, f));
      } else notices(p);
    }
  }
}
notices(join(root, "clients/terminal/node_modules"));
const sha = (p: string) =>
  createHash("sha256").update(readFileSync(p)).digest("hex");
const inputs: Record<string, string> = {};
for (const folder of [
  "src",
  "clients/terminal/src",
  "clients/terminal/scripts",
  "clients/terminal/assets/graphics",
  "examples/fixtures/build_plans.py",
  "examples/fixtures/terminal-client",
  "scripts/prepare-terminal-models.py",
  "scripts/archive-terminal.py",
]) {
  const walk = (p: string) => {
    if (statSync(p).isDirectory()) {
      for (const n of readdirSync(p).sort()) walk(join(p, n));
    } else inputs[relative(root, p)] = sha(p);
  };
  walk(join(root, folder));
}
for (const p of [
  "Cargo.toml",
  "Cargo.lock",
  "clients/terminal/package.json",
  "clients/terminal/bun.lock",
  "examples/model_successor.rs",
  "examples/terminal_fixture_handover.rs",
  "clients/terminal/node_modules/@resvg/resvg-wasm/index_bg.wasm",
])
  inputs[p] = sha(join(root, p));
const host = join(destination, "knowledgehub-client-host"),
  workspace = join(destination, "models/workspace.json");
const p = Bun.spawn(
  [join(destination, "kh"), "--workspace", workspace, "info", "--json"],
  { stdout: "pipe", stderr: "inherit" },
);
const info = await new Response(p.stdout).json();
if (await p.exited) throw Error("compiled handshake failed");
writeFileSync(
  join(destination, "build-manifest.json"),
  JSON.stringify(
    {
      format: "knowledgehub-terminal-distribution/1",
      created: new Date().toISOString(),
      target: "aarch64-apple-darwin",
      filesystem: "native macOS/APFS",
      bun: Bun.version,
      rust: "1.99.1",
      client: "0.10.0",
      protocol: "knowledgehub.client/1",
      host: info.data.capabilities,
      inputs,
      graphics: {
        worker: "src/graph-render-worker.js",
        input_inventory: "graphics-notices/inventory.json",
        lazy: true,
        actual_kitty: "pending",
        actual_ghostty: "pending",
      },
      fixtures: {
        mode: retainedModels
          ? "quiescent retained fixture copy; no reauthoring"
          : "fresh source-bound authoring",
      },
      checks: {
        end_user_development_tools: false,
        linux_native: "not verified",
        signing: "not signed/not notarized",
        publication: "not published",
        human_acceptance: "pending",
      },
    },
    null,
    2,
  ) + "\n",
);
const files: Record<string, string> = {};
const walk = (dir: string) => {
  for (const name of readdirSync(dir).sort()) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) walk(path);
    else files[relative(destination, path)] = sha(path);
  }
};
walk(destination);
writeFileSync(
  join(destination, "checksums.json"),
  JSON.stringify({ algorithm: "sha256", files }, null, 2) + "\n",
);
await run(["python3", "scripts/archive-terminal.py", destination]);
console.log(
  JSON.stringify({
    directory: destination,
    archive: destination + ".tar.gz",
    host_sha256: sha(host),
    files: Object.keys(files).length,
  }),
);
