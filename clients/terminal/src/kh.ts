#!/usr/bin/env bun
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { Transport } from "./transport.ts";
import { Explorer, type Options } from "./explorer.ts";
import { execute, ref, type Command } from "./commands.ts";
import { Failure, clean, exitFor, uint, type Json } from "./types.ts";
import { browse } from "./tui.ts";

export const help = `kh 0.10.0 — read-only native KnowledgeHub explorer
Usage: kh [OPTIONS] COMMAND [ARGUMENTS] [OPTIONS]
  models | info | types | type TYPE | list [TYPE] [--subtypes]
  show REF | links REF | tree REF [--depth 1..8]
  views [--view type|context|journey|source]
  search PHRASE [--type TYPE --field /name --mode literal-utf8/1]
  source REF [--preview --offset N --length 1..65536 | --export PATH]
  history REF                         exact predecessor lineage
  publications                        accepted native publications
  publication OPERATION [--affected]  original outcome and affected order
  baselines [--retained]               selected/handover; retained is opt-in
  compare BASELINE_JSON BASELINE_JSON  native unpaged exact comparison
  gaps                                bounded native coverage metadata
  diagram REF --output PATH [--format html|json|mermaid]
    [--projection relationships|journey|lifecycle --depth 1..8]
    [--vertices 1..1000 --edges 1..10000 --replace]
  browse                              keyboard TUI; ? opens help

Context: --workspace FILE --model ALIAS --snapshot HASH
         --baseline '{"model":"...","id":"...","revision":"..."}'
         --history-anchor HASH (independent accepted snapshot)
REF: exact JSON {model,id,revision}, selected semantic ID or unambiguous name.
Links: --direction incoming|outgoing|both --category Association,Property
       --selector '[{"kind":"Property","definition":{...},"path":"/..."}]'
       --closure explicitly includes supporting fact owners.
Limits: --calls 1..1024 (default 512), --items 1..10000 (10000),
        --bytes 8192..8387584 (1048576); 64 history data reads/command.
History page: --history-items 1..1000 --history-nodes 1..8192
              --history-storage 1..134217728 --history-output 1..4194304
Defaults: 100 items,256 nodes,16 MiB storage,256 KiB native output.
All limits are finite; invalid values fail without clamping. Process tokens die
on close; incomplete results retain pinned intent for a fresh query. Native
HistoryWork and full transport/output byte counts are separate.
--json emits knowledgehub-exploration/1. --no-color / NO_COLOR is supported.
Exits: 0 complete (including NotPublishedAt),2 usage,3 partial/limit/readiness,
       4 denied,5 not found/ambiguous,6 operational/integrity/unavailable,
       130 cancelled. --help and --version require no model or development tools.
Read-only may perform ordinary native recovery/index writes; no semantic writes.
`;
const bools = new Set([
  "json",
  "help",
  "version",
  "subtypes",
  "affected",
  "retained",
  "replace",
  "preview",
  "closure",
  "no-color",
]);
const globals = [
  "workspace",
  "model",
  "snapshot",
  "baseline",
  "history-anchor",
  "calls",
  "items",
  "bytes",
  "history-items",
  "history-nodes",
  "history-storage",
  "history-output",
  "json",
  "help",
  "version",
  "no-color",
];
const allowed: Record<string, string[]> = {
  models: [],
  info: [],
  types: [],
  type: [],
  list: ["subtypes"],
  show: [],
  links: ["direction", "category", "selector", "closure"],
  tree: ["direction", "category", "selector", "closure", "depth"],
  views: ["view"],
  search: ["type", "field", "mode"],
  source: ["preview", "offset", "length", "export", "replace"],
  history: [],
  publications: [],
  publication: ["affected"],
  baselines: ["retained"],
  compare: [],
  gaps: [],
  diagram: [
    "output",
    "format",
    "projection",
    "depth",
    "vertices",
    "edges",
    "replace",
  ],
  browse: [],
};
export function parse(argv: string[]): Command {
  const flags: Record<string, string | boolean> = {},
    p: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a.startsWith("--")) {
      const name = a.slice(2);
      if (flags[name] !== undefined)
        throw new Failure({ code: "Usage", message: `duplicate --${name}` });
      if (bools.has(name)) flags[name] = true;
      else {
        if (!argv[i + 1] || argv[i + 1].startsWith("--"))
          throw new Failure({
            code: "Usage",
            message: `--${name} requires a value`,
          });
        flags[name] = argv[++i];
      }
    } else p.push(a);
  }
  const name = p.shift() ?? "browse";
  if (!allowed[name] && !flags.help && !flags.version)
    throw new Failure({ code: "Usage", message: `unknown command ${name}` });
  for (const n of Object.keys(flags))
    if (!globals.includes(n) && !allowed[name]?.includes(n))
      throw new Failure({
        code: "Usage",
        message: `unsupported --${n} for ${name}`,
      });
  return { name, positionals: p, flags };
}
function options(f: Command["flags"]): Options {
  const n = (name: string, def: number, max: number) =>
    uint(f[name] === undefined ? undefined : String(f[name]), def, max);
  const bytes = n("bytes", 1048576, 8387584);
  if (bytes < 8192)
    throw new Failure({
      code: "Usage",
      message: "--bytes must be at least 8192 for the result envelope",
    });
  for (const name of ["depth", "vertices", "edges", "length"])
    if (f[name] !== undefined)
      n(
        name,
        1,
        name === "depth"
          ? 8
          : name === "vertices"
            ? 1000
            : name === "edges"
              ? 10000
              : 65536,
      );
  if (f.offset !== undefined && !/^(0|[1-9][0-9]*)$/.test(String(f.offset)))
    throw new Failure({
      code: "Usage",
      message: "offset must be canonical unsigned decimal",
    });
  return {
    model: f.model ? String(f.model) : undefined,
    snapshot: f.snapshot ? String(f.snapshot) : undefined,
    baseline: f.baseline ? ref(String(f.baseline)) : undefined,
    historyAnchor: f["history-anchor"]
      ? String(f["history-anchor"])
      : undefined,
    calls: n("calls", 512, 1024),
    items: n("items", 10000, 10000),
    bytes,
    historyLimits: {
      items: String(n("history-items", 100, 1000)),
      nodes: String(n("history-nodes", 256, 8192)),
      storage_bytes: String(n("history-storage", 16777216, 134217728)),
      output_bytes: String(n("history-output", 262144, 4194304)),
    },
  };
}
export async function main(argv = process.argv.slice(2)): Promise<number> {
  let command: Command | undefined,
    e: Explorer | undefined,
    t: Transport | undefined;
  const json = argv.includes("--json");
  let error: Json, data: Json;
  let tuiFinished = false;
  try {
    command = parse(argv);
    if (command.flags.help) {
      process.stdout.write(help);
      return 0;
    }
    if (command.flags.version) {
      process.stdout.write(
        "kh 0.10.0 · knowledgehub.client/1 · knowledgehub-history/1 · catalogue-3\n",
      );
      return 0;
    }
    const adjacent = resolve(
        dirname(process.execPath),
        "models/workspace.json",
      ),
      workspace = resolve(
        String(
          command.flags.workspace ??
            process.env.KNOWLEDGEHUB_WORKSPACE ??
            (existsSync(adjacent) ? adjacent : "workspace.json"),
        ),
      );
    if (
      command.name === "browse" &&
      (!process.stdin.isTTY || command.flags.json)
    )
      throw new Failure({
        code: "Usage",
        message:
          "browse requires a TTY and human output; kh --help lists scripting commands",
      });
    const arity: Record<string, number> = {
      models: 0,
      info: 0,
      types: 0,
      type: 1,
      list: 1,
      show: 1,
      links: 1,
      tree: 1,
      views: 0,
      search: 1,
      source: 1,
      history: 1,
      publications: 0,
      publication: 1,
      baselines: 0,
      compare: 2,
      gaps: 0,
      diagram: 1,
      browse: 0,
    };
    if (command.positionals.length > arity[command.name])
      throw new Failure({
        code: "Usage",
        message: "too many positional arguments",
      });
    const opts = options(command.flags);
    t = new Transport(workspace);
    await t.handshake();
    if (command.name === "models") {
      data = await t.call("models");
      e = new Explorer(t, opts);
    } else {
      e = new Explorer(t, opts);
      await e.open();
      if (command.name === "browse") {
        await browse(
          e,
          !!command.flags["no-color"] || process.env.NO_COLOR !== undefined,
        );
        tuiFinished = true;
      } else data = await execute(e, command);
    }
  } catch (err) {
    error =
      err instanceof Failure
        ? err.detail
        : { code: "Operational", message: String(err) };
  } finally {
    if (e) await e.release();
    if (t)
      try {
        await t.close();
      } catch (err) {
        error =
          err instanceof Failure
            ? err.detail
            : { code: "Operational", message: String(err) };
      }
  }
  if (tuiFinished && !error) return 0;
  let result: Json = e
    ? e.envelope(command ?? { argv }, data, error)
    : {
        format: "knowledgehub-exploration/1",
        status: "error",
        intent: command ?? { argv },
        error,
      };
  const cap = e?.options.bytes ?? 1048576;
  const encode = () => {
    const render = () =>
      JSON.stringify(result, null, json ? undefined : 2).replace(
        /[\x00-\x08\x0b-\x1f\x7f-\x9f\u202a-\u202e\u2066-\u2069]/g,
        (c) => clean(c),
      ) + "\n";
    if (result.coverage)
      for (let i = 0; i < 5; i++)
        result.coverage.encoded_output_bytes = String(
          Buffer.byteLength(render()),
        );
    return render();
  };
  let encoded = encode();
  if (Buffer.byteLength(encoded) > cap) {
    e?.stop("final envelope output budget reached; indivisible data omitted");
    error = {
      code: "Capacity",
      message: "final envelope exceeds output budget",
      encoded_bytes: String(Buffer.byteLength(encoded)),
      limit: String(cap),
    };
    result = e
      ? e.envelope(command ?? { argv }, null, error)
      : {
          format: "knowledgehub-exploration/1",
          status: "partial",
          intent: command ?? { argv },
          error,
        };
    encoded = encode();
  }
  if (json || !error) process.stdout.write(encoded);
  else process.stderr.write(encoded);
  return error ? exitFor(error) : e?.partial ? 3 : 0;
}
if (import.meta.main) process.exitCode = await main();
