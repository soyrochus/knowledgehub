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
} from "node:fs";
import { resolve, join, relative } from "node:path";
import { createHash } from "node:crypto";
const root = resolve(import.meta.dir, "../../.."),
  destination = resolve(process.argv[2] ?? "");
if (process.argv.length !== 3 || existsSync(destination))
  throw Error(
    "usage: bun scripts/package.ts NEW-DESTINATION (occupied paths refused)",
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
await run([
  "bun",
  "build",
  "--compile",
  "--target=bun-darwin-arm64",
  "clients/terminal/src/kh.ts",
  "--outfile",
  join(destination, "kh"),
]);
cpSync(
  join(root, "target/release/knowledgehub-client-host"),
  join(destination, "knowledgehub-client-host"),
);
chmodSync(join(destination, "knowledgehub-client-host"), 0o755);
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
  "Offline Mermaid 12.3.0 and transitive dependencies are bundled under their licenses. Bun 1.4.0 is embedded in kh. See build-manifest.json and dependency-licenses/ for retained notices.\n",
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
