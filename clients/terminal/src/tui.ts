import { emitKeypressEvents } from "node:readline";
import { Explorer } from "./explorer.ts";
import { Transport } from "./transport.ts";
import { execute, ref, linkOptions } from "./commands.ts";
import { TerminalInput } from "./terminal-input.ts";
import { TerminalWriter, textScreen } from "./kitty-graphics.ts";
import { viewOptions } from "./view-options.ts";
import {
  Failure,
  clean,
  key,
  label,
  type Json,
  type Ref,
  type Selection,
} from "./types.ts";

type Intent = {
  kind: string;
  reference?: Ref;
  view?: string;
  operation?: string;
  head?: string;
  phrase?: string;
  type?: string;
  field?: string;
  extra?: Json;
};
type State = {
  model: string;
  selection: Selection;
  anchor: string;
  intent: Intent;
  row: number;
  scroll: number;
  page: number;
  filter?: string;
  focus?: number;
  detailScroll?: number;
};
type Row = { name: string; reference?: Ref; kind: string; data?: Json };
const help = `Keyboard · all model operations are READ ONLY
↑/↓ or j/k move · Enter inspect/follow · Space expand · Tab focus panes
t by type · c by context · J by journey · u by source · U unassigned group
l links · i link filters · T inherited schema · L lineage · w bounded tree · I info
s sources/status · z exact text preview · x original export
g native gaps · p accepted publications · o known operation lookup
a affected references (publication selected) · b baselines · D retained baselines
B baseline JSON · S snapshot · A publication anchor · C compare A/B · m models
/ literal search · f local display filter (=NAME is exact)
n next native page · h back / H forward · Alt←/Alt→ same navigation
r explicit head refresh · R reconnect at retained selection/anchor
v diagram export · V View graph (Kitty) · ? help · Esc cancel/back
q quit · Ctrl-C cancel/quit · --no-color / NO_COLOR
Navigation, predecessor lineage, baseline parents and publications are separate.
Exact selections/anchors survive back/forward; restored pages query afresh.
Invalid/expired process tokens restart visibly at their original pinned intent.
No mouse, services or development tools are needed. Narrow windows retain keys.
`;
export async function browse(
  initial: Explorer,
  noColor = false,
): Promise<void> {
  const input = new TerminalInput(process.stdin),
    writer = new TerminalWriter(process.stdout);
  let graphActive = false;
  let e = initial,
    t = initial.transport,
    state: State = {
      model: initial.info.alias,
      selection: initial.selection,
      anchor: initial.anchor,
      intent: { kind: "groups", view: "type" },
      row: 0,
      scroll: 0,
      page: 0,
    };
  const back: State[] = [],
    forward: State[] = [];
  let rows: Row[] = [],
    detail: Json =
      "Expand a registered type with Space; inspect a row with Enter.",
    focus = 0,
    detailScroll = 0,
    filter = "",
    message = "",
    busy = false,
    showHelp = false,
    connection = "connected",
    token: string | undefined,
    done = false,
    historyPermission = initial.info.grants.includes("ReadHistory")
      ? "ReadHistory"
      : "no ReadHistory";
  let abort: AbortController | undefined,
    prompt:
      | {
          text: string;
          value: string;
          resolve: (s: string) => void;
          reject: (e: unknown) => void;
        }
      | undefined;
  const clone = () =>
    structuredClone({ ...state, filter, focus, detailScroll });
  const visible = () =>
    rows.filter((r) =>
      filter.startsWith("=")
        ? clean(r.name) === filter.slice(1)
        : clean(r.name).toLowerCase().includes(filter.toLowerCase()),
    );
  const historyView = () =>
    ["publications", "affected", "retained", "publication"].includes(
      state.intent.kind,
    );
  const reference = () =>
    visible()[state.row]?.reference ?? state.intent.reference;
  function render() {
    if (done || graphActive) return;
    const width = process.stdout.columns ?? 100,
      height = process.stdout.rows ?? 30;
    const out: string[] = [];
    const line = (s: string) => out.push(s.slice(0, Math.max(width - 1, 10)));
    line(
      `KnowledgeHub · ${state.model} · READ ONLY · ${busy ? "working / Esc cancels" : connection} · ${historyPermission}`,
    );
    line(`Content ${JSON.stringify(state.selection)}`);
    line(`Publications ${state.anchor}`);
    line(
      `${state.intent.kind}${state.intent.view ? " / " + state.intent.view : ""} · page ${state.page + 1} · ${rows.length} rows · ${token ? "n: more" : "end / coverage below"} · filter ${clean(filter)}`,
    );
    if (showHelp) {
      for (const s of help.split("\n").slice(0, Math.max(1, height - 7)))
        line(s);
    } else if (busy) {
      line("Checking current authority / reading pinned intent…");
      for (let i = 0; i < Math.max(0, height - 8); i++) line("");
    } else {
      const left = Math.max(18, Math.floor(width * 0.38)),
        body = Math.max(1, height - 7),
        v = visible();
      state.row = Math.max(0, Math.min(state.row, v.length - 1));
      if (state.row < state.scroll) state.scroll = state.row;
      if (state.row >= state.scroll + body) state.scroll = state.row - body + 1;
      const text =
        typeof detail === "string" ? detail : JSON.stringify(detail, null, 2);
      const right = clean(text)
        .replace(/\\u000a/g, "\n")
        .split("\n");
      for (let i = 0; i < body; i++) {
        const row = v[state.scroll + i];
        const a = row
          ? `${focus === 0 && state.scroll + i === state.row ? ">" : " "} ${clean(row.name)}`
          : "";
        line(
          a.slice(0, left - 1).padEnd(left) +
            "│ " +
            (right[detailScroll + i] ?? ""),
        );
      }
    }
    line(
      prompt
        ? `${prompt.text}: ${clean(prompt.value)} (Esc cancels)`
        : message ||
            "Tab panes · Space expand · Enter inspect · l links · s sources · p publications · v export · V View graph (Kitty) · ? help · q quit",
    );
    line(
      width < 70 || height < 15
        ? "Compact view · ↑↓ · Enter · Esc · ? help · q quit"
        : "Arrow/jk · h/H history · r explicit head refresh · R reconnect · no semantic writes",
    );
    process.stdout.write(textScreen + out.join("\n"));
  }
  const ask = (text: string) =>
    new Promise<string>((resolve, reject) => {
      prompt = { text, value: "", resolve, reject };
      render();
    });
  async function fresh() {
    await e.release();
    e = new Explorer(t, {
      ...e.options,
      model: state.model,
      snapshot:
        state.selection.selection === "Snapshot"
          ? state.selection.reference
          : undefined,
      baseline:
        state.selection.selection === "Baseline"
          ? state.selection.reference
          : undefined,
      historyAnchor: state.anchor,
      signal: abort?.signal,
    });
    await e.open();
    historyPermission = e.info.grants.includes("ReadHistory")
      ? "ReadHistory"
      : "no ReadHistory";
  }
  function recordRows(values: Json[]): Row[] {
    return values.map((r) => ({
      name: label(r) || r.reference?.id || "record",
      reference: r.reference,
      kind: "record",
      data: r,
    }));
  }
  async function page(next = false) {
    const i = state.intent;
    if (i.kind === "instances") {
      const p = await e.queryPage(i.reference, false, next ? token : undefined);
      rows = recordRows(p.items);
      token = p.continuation ?? undefined;
      detail = {
        selection: e.selection,
        coverage: { exhausted: p.exhausted, truncated: p.truncated },
        type: i.reference,
      };
    } else if (i.kind === "publications" || i.kind === "affected") {
      const p = await e.historyPage(
        i.kind === "affected" ? i.operation : undefined,
        next ? token : undefined,
      );
      token = p.continuation ?? undefined;
      if (i.kind === "publications")
        rows = p.items.Publications.map((h: Json) => ({
          name: `${h.sequence} ${h.operation_id} · ${h.actor}`,
          kind: "publication",
          data: h,
        }));
      else
        rows = p.items.Affected.references.map((r: Ref, n: number) => ({
          name: `${BigInt(p.items.Affected.offset) + BigInt(n)} ${r.id}@${r.revision}`,
          kind: "affected",
          reference: r,
          data: { head: i.head },
        }));
      detail = {
        ...p,
        items: undefined,
        continuation: undefined,
        note: "a: affected; Enter: original header / exact original-snapshot record",
      };
    }
  }
  async function load() {
    token = undefined;
    rows = [];
    detailScroll = 0;
    const i = state.intent;
    if (i.kind === "groups") {
      const g = await e.groups(i.view ?? "type");
      rows = g.groups.map((r: Json) => ({
        name: label(r),
        kind: "group",
        reference: r.reference,
        data: { view: i.view },
      }));
      if (g.unassigned)
        rows.push({
          name: g.unassigned.name,
          kind: "unassigned",
          data: { view: i.view },
        });
      detail = g.meaning;
    } else if (["instances", "publications", "affected"].includes(i.kind)) {
      for (let p = 0; p <= state.page; p++) {
        await page(p > 0);
        if (p < state.page && !token) {
          state.page = p;
          message = "Restored page ended earlier at the retained exact intent";
          break;
        }
      }
    } else if (i.kind === "members") {
      const result = await e.groupMembers(i.view!, i.reference);
      rows = recordRows(result.items);
      detail = result;
    } else if (i.kind === "record") {
      detail = await e.show(i.reference!);
      rows = [
        {
          name: label(detail.record),
          reference: i.reference,
          kind: "record",
          data: detail.record,
        },
      ];
    } else if (i.kind === "links") {
      const p = await e.links(i.reference!, i.extra ?? {});
      rows = p.neighbours.map((n: Json) => ({
        name: `${n.directions.join("/")} ${n.selector.path ?? n.selector.definition.id} → ${n.opposite.id}`,
        reference: n.opposite,
        kind: "link",
        data: n,
      }));
      detail = {
        reference: i.reference,
        meaning:
          "Exact assertion/property origins; Enter follows target, Space opens owner",
        coverage: e.warnings,
      };
    } else if (i.kind === "sources") {
      const p = await e.sources(i.reference!);
      rows = recordRows(p.records);
      detail = p.note;
    } else if (i.kind === "baselines" || i.kind === "retained") {
      const p = await e.baselines(i.kind === "retained");
      rows = p.baselines.map((b: Json) => ({
        name: `${b.reference.id}@${b.reference.revision} · ${b.origins.join(",")}`,
        reference: b.reference,
        kind: "baseline",
        data: b,
      }));
      detail = p;
    } else if (i.kind === "models") {
      const p = await t.call("models");
      rows = p.models.map((m: Json) => ({
        name: `${m.alias} · ${m.name}`,
        kind: "model",
        data: m,
      }));
      detail = p;
    } else if (i.kind === "search") {
      const p = await e.search(i.phrase!, i.type!, i.field!);
      rows = p.hits.map((h: Json) => ({
        name: `${h.record.id} ${h.field} ${h.start}:${h.end}`,
        reference: h.record,
        kind: "hit",
        data: h,
      }));
      detail = p;
    } else if (i.kind === "publication") {
      detail = await e.publication(i.operation!);
      rows = detail.outcome?.Published
        ? [
            {
              name: i.operation!,
              kind: "publication",
              data: detail.outcome.Published,
            },
          ]
        : [];
    } else if (i.kind === "gaps") {
      detail = await e.gaps();
      rows = detail.coverage.gaps.map((g: Json) => ({
        name: `${g.subject.id} ${g.path}`,
        reference: g.subject,
        kind: "gap",
        data: g,
      }));
    }
    if (e.partial) message = "PARTIAL: " + e.warnings.join("; ");
  }
  async function navigate(
    intent: Intent,
    change?: Partial<State>,
    remember = true,
  ) {
    if (remember) {
      back.push(clone());
      if (back.length > 100) back.shift();
      forward.length = 0;
    }
    state = { ...state, ...change, intent, row: 0, scroll: 0, page: 0 };
    filter = "";
    focus = 0;
    await fresh();
    await load();
  }
  async function restore(from: State[], to: State[]) {
    const previous = from.pop();
    if (!previous) {
      message = "No navigation state";
      return;
    }
    to.push(clone());
    state = previous;
    filter = previous.filter ?? "";
    focus = previous.focus ?? 0;
    message = "Fresh query at retained exact content and publication anchor";
    await fresh();
    await load();
    detailScroll = previous.detailScroll ?? 0;
  }
  let resolveDone: () => void;
  const finished = new Promise<void>((r) => (resolveDone = r));
  function report(error: unknown) {
    const d =
      error instanceof Failure
        ? error.detail
        : { code: "Operational", message: String(error) };
    if (["Disconnected", "Protocol"].includes(d.code))
      connection = "disconnected";
    if (
      ["Denied", "Unavailable", "Disconnected", "Protocol"].includes(d.code)
    ) {
      rows = [];
      token = undefined;
    }
    if (d.code === "Denied" && d.action === "ReadHistory")
      historyPermission = "ReadHistory DENIED";
    message = `${d.code}: ${clean(d.message ?? d.reason ?? d.action ?? "")}`;
    if (d.code !== "Cancelled") detail = d;
    if (["InvalidCursor", "Disconnected", "Protocol"].includes(d.code))
      message +=
        " · R reconnect / fresh pinned restart; no latest substitution";
  }
  async function job(action: () => Promise<void>) {
    if (busy) return;
    busy = true;
    abort = new AbortController();
    e.options.signal = abort.signal;
    render();
    try {
      await action();
    } catch (error) {
      report(error);
    } finally {
      if (!done && connection === "connected")
        try {
          await t.call("authorize", { history: historyView() }, state.model);
        } catch (error) {
          report(error);
        }
      busy = false;
      abort = undefined;
      render();
    }
  }
  async function keypress(str: string, k: Json) {
    if (graphActive) return;
    if (prompt) {
      if (k.name === "escape") {
        const p = prompt;
        prompt = undefined;
        p.reject(new Failure({ code: "Cancelled" }));
        render();
      } else if (k.name === "return") {
        const p = prompt;
        prompt = undefined;
        p.resolve(p.value);
      } else if (k.name === "backspace") {
        prompt.value = [...prompt.value].slice(0, -1).join("");
        render();
      } else if (!k.ctrl && !k.meta && str) {
        prompt.value += str;
        render();
      }
      return;
    }
    if (k.name === "q" || (k.ctrl && k.name === "c" && !busy)) {
      done = true;
      abort?.abort();
      resolveDone!();
      return;
    }
    if (k.name === "escape" && busy) {
      abort?.abort();
      message = "Cancelled caller; native work drains";
      render();
      return;
    }
    if (busy) return;
    if (str === "?" && !showHelp) {
      showHelp = true;
      render();
      return;
    }
    if (showHelp && (str === "?" || k.name === "escape")) {
      showHelp = false;
      await job(async () => {});
      return;
    }
    await job(async () => {
      if (str !== "R")
        await t.call("authorize", { history: false }, state.model);
      if (
        historyView() &&
        !(
          ["h", "H", "t", "c", "J", "u", "m", "g", "B", "S", "r", "R"].includes(
            str,
          ) ||
          k.name === "escape" ||
          (k.meta && (k.name === "left" || k.name === "right"))
        )
      )
        await t.call("authorize", { history: true }, state.model);
      const row = visible()[state.row];
      if (k.name === "tab") {
        focus = 1 - focus;
      } else if (k.name === "down" || str === "j") {
        if (focus) detailScroll++;
        else state.row++;
      } else if (k.name === "up" || str === "k") {
        if (focus) detailScroll = Math.max(0, detailScroll - 1);
        else state.row = Math.max(0, state.row - 1);
      } else if (k.name === "pageup") {
        if (focus) detailScroll = Math.max(0, detailScroll - 10);
        else state.row = Math.max(0, state.row - 10);
      } else if (k.name === "pagedown") {
        if (focus) detailScroll += 10;
        else state.row += 10;
      } else if (
        str === "h" ||
        (k.meta && k.name === "left") ||
        k.name === "escape"
      )
        await restore(back, forward);
      else if (str === "H" || (k.meta && k.name === "right"))
        await restore(forward, back);
      else if (["t", "c", "J", "u"].includes(str))
        await navigate({
          kind: "groups",
          view: (
            { t: "type", c: "context", J: "journey", u: "source" } as Json
          )[str],
        });
      else if (str === "U")
        await navigate({
          kind: "members",
          view: state.intent.view ?? "context",
        });
      else if (str === " " && row?.kind === "link")
        await navigate({ kind: "record", reference: row.data.origin.owner });
      else if ((str === " " || k.name === "return") && row?.kind === "group")
        await navigate({
          kind: row.data.view === "type" ? "instances" : "members",
          view: row.data.view,
          reference: row.reference,
        });
      else if (
        (str === " " || k.name === "return") &&
        row?.kind === "unassigned"
      )
        await navigate({ kind: "members", view: row.data.view });
      else if (k.name === "return" && row?.kind === "model") {
        const next = await new Explorer(t, {
          ...e.options,
          model: row.data.alias,
          snapshot: undefined,
          baseline: undefined,
          historyAnchor: undefined,
        }).open();
        await navigate(
          { kind: "groups", view: "type" },
          {
            model: next.info.alias,
            selection: next.selection,
            anchor: next.anchor,
          },
        );
      } else if (k.name === "return" && row?.kind === "baseline")
        await navigate(
          { kind: "groups", view: "type" },
          { selection: { selection: "Baseline", reference: row.reference! } },
        );
      else if (k.name === "return" && row?.kind === "publication") {
        detail = row.data;
        focus = 1;
        detailScroll = 0;
      } else if (k.name === "return" && row?.reference)
        await navigate(
          { kind: "record", reference: row.reference },
          row.kind === "affected"
            ? { selection: { selection: "Snapshot", reference: row.data.head } }
            : undefined,
        );
      else if (str === "l" && reference())
        await navigate({ kind: "links", reference: reference() });
      else if (str === "s" && reference())
        await navigate({ kind: "sources", reference: reference() });
      else if (str === "z" && reference()) {
        detail = await e.preview(reference()!);
        focus = 1;
        detailScroll = 0;
      } else if (str === "x" && reference()) {
        const path = await ask("Original export path (new file)");
        detail = await execute(e, {
          name: "source",
          positionals: [JSON.stringify(reference())],
          flags: { export: path },
        });
        focus = 1;
      } else if (str === "p") await navigate({ kind: "publications" });
      else if (str === "o") {
        const operation = await ask("Exact operation ID");
        await navigate({ kind: "publication", operation });
        focus = 1;
      } else if (str === "a" && row?.kind === "publication")
        await navigate({
          kind: "affected",
          operation: row.data.operation_id,
          head: row.data.head,
        });
      else if (str === "b" || str === "D")
        await navigate({ kind: str === "D" ? "retained" : "baselines" });
      else if (str === "g") await navigate({ kind: "gaps" });
      else if (str === "w" && reference()) {
        const depth = await ask("Tree depth 1..8");
        if (!/^[1-8]$/.test(depth))
          throw new Failure({ code: "Usage", message: "depth must be 1..8" });
        detail = await e.tree(reference()!, Number(depth));
        focus = 1;
        detailScroll = 0;
      } else if (str === "I") {
        detail = {
          ...e.info,
          selection: e.selection,
          publication_anchor: e.anchor,
          capabilities: t.capabilities,
        };
        focus = 1;
        detailScroll = 0;
      } else if (str === "L" && reference()) {
        detail = await e.lineage(reference()!);
        focus = 1;
        detailScroll = 0;
      } else if (str === "T" && reference()) {
        const r = await e.read(reference()!);
        detail = await e.schema(
          r.data.kind === "Type" ? r.reference : r.type_ref,
        );
        focus = 1;
        detailScroll = 0;
      } else if (str === "C") {
        const before = ref(await ask("Exact baseline A JSON")),
          after = ref(await ask("Exact baseline B JSON"));
        detail = await e.compare(before, after);
        focus = 1;
        detailScroll = 0;
      } else if (str === "A") {
        const anchor = await ask("Exact publication snapshot anchor");
        await navigate(state.intent, { anchor });
      } else if (str === "i" && reference()) {
        const direction =
            (await ask("Link direction incoming/outgoing/both")) || "both",
          category =
            (await ask("Native categories (comma-separated)")) ||
            "Association,Property",
          selector =
            (await ask("Exact selector JSON array (blank = all)")) || "[]";
        await navigate({
          kind: "links",
          reference: reference(),
          extra: linkOptions({ direction, category, selector }),
        });
      } else if (str === "m") await navigate({ kind: "models" });
      else if (str === "B") {
        const baseline = ref(await ask("Exact baseline JSON"));
        await navigate(
          { kind: "groups", view: "type" },
          { selection: { selection: "Baseline", reference: baseline } },
        );
      } else if (str === "S") {
        const snapshot = await ask("Exact snapshot hash");
        await navigate(
          { kind: "groups", view: "type" },
          { selection: { selection: "Snapshot", reference: snapshot } },
        );
      } else if (str === "f") {
        const previous = filter;
        try {
          filter = await ask("Local display filter");
          state.row = state.scroll = 0;
        } catch (error) {
          filter = previous;
          throw error;
        }
      } else if (str === "/") {
        const phrase = await ask("Literal phrase (at least 3 UTF-8 bytes)"),
          type = await ask("Registered type name or exact JSON"),
          field = (await ask("Field path (blank = /name)")) || "/name";
        await navigate({ kind: "search", phrase, type, field });
      } else if (str === "n") {
        if (token) {
          state.page++;
          state.row = state.scroll = 0;
          try {
            await page(true);
          } catch (error) {
            if (
              error instanceof Failure &&
              error.detail.code === "InvalidCursor"
            ) {
              state.page = 0;
              message =
                "Token expired/lost; visibly restarted first page at retained anchor";
              await fresh();
              await load();
            } else throw error;
          }
        } else
          message =
            "No live next page; use fresh pinned intent with permitted bounds";
      } else if (str === "r") {
        const next = await e.call("head");
        await navigate(
          { kind: "groups", view: "type" },
          {
            selection: { selection: "Snapshot", reference: next.head },
            anchor: next.head,
          },
        );
        message =
          "Explicitly refreshed to head; h restores the previous pinned selection/anchor";
      } else if (str === "R") {
        await e.release();
        await t.close();
        t = new Transport(t.workspace);
        await t.handshake();
        connection = "connected";
        message = "Reconnected; fresh query at retained exact selection/anchor";
        await fresh();
        await load();
      } else if (str === "V" && reference()) {
        const selected = structuredClone(reference()!);
        const projection =
          (await ask("View graph (Kitty): relationships/journey/lifecycle")) ||
          "relationships";
        const depth = (await ask("Graph depth 1..8 (blank 1)")) || "1";
        const vertices =
          (await ask("Graph vertices 1..200 (blank 60)")) || "60";
        const edges = (await ask("Graph edges 1..500 (blank 120)")) || "120";
        const opts = viewOptions({ projection, depth, vertices, edges });
        const { viewGraph } = await import("./graph-view.ts");
        graphActive = true;
        try {
          const code = await viewGraph(e, selected, opts, { input, writer });
          message = `View graph closed (exit ${code}); retained exact browser selection, page and position`;
        } finally {
          graphActive = false;
        }
      } else if (str === "v" && reference()) {
        const format =
            (await ask("Diagram format html/json/mermaid")) || "html",
          output = await ask("New output path"),
          projection =
            (await ask("Projection relationships/journey/lifecycle")) ||
            "relationships",
          depth = (await ask("Depth 1..8")) || "1";
        if (!/^[1-8]$/.test(depth))
          throw new Failure({ code: "Usage", message: "depth must be 1..8" });
        detail = await execute(e, {
          name: "diagram",
          positionals: [JSON.stringify(reference())],
          flags: { format, output, projection, depth },
        });
        message = e.partial
          ? "PARTIAL derived diagram exported"
          : "Derived diagram exported";
        focus = 1;
        detailScroll = 0;
      }
    });
  }
  emitKeypressEvents(input.keys);
  process.stdin.setRawMode(true);
  process.stdin.resume();
  process.stdout.write("\x1b[?25l");
  const onKey = (s: string, k: Json) => {
    void keypress(s, k);
  };
  input.keys.on("keypress", onKey);
  const onResize = () => {
    void job(async () => {});
  };
  process.stdout.on("resize", onResize);
  try {
    await job(async () => {
      await load();
    });
    await finished;
  } finally {
    done = true;
    input.keys.off("keypress", onKey);
    input.close();
    process.stdout.off("resize", onResize);
    process.stdin.setRawMode(false);
    process.stdin.pause();
    process.stdout.write("\x1b[?25h\x1b[0m\n");
    await e.release();
    if (t !== initial.transport) await t.close();
  }
}
