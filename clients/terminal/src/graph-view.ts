import { emitKeypressEvents } from "node:readline";
import { Explorer } from "./explorer.ts";
import { extract } from "./visual.ts";
import {
  Failure,
  clean,
  exitFor,
  key as refKey,
  type Json,
  type Ref,
} from "./types.ts";
import { GraphRenderer } from "./graph-render.ts";
import {
  presentation,
  type DisplayGraph,
  type Viewport,
} from "./graph-layout.ts";
import {
  KittyGraphics,
  TerminalWriter,
  textScreen,
  type Geometry,
} from "./kitty-graphics.ts";
import { TerminalInput } from "./terminal-input.ts";
import type { ViewOptions } from "./view-options.ts";
export const viewerHelp =
  "Esc/q close · ? help · Ctrl-C cancel · Tab focus · n/p next/previous · e nodes/edges · Enter inspect/preview · s sources · arrows pan · +/- zoom · 0 fit · j/k/PgUp/PgDn inspector · R pinned retry";
const safeLines = (value: Json, width: number): string[] => {
  const text =
    typeof value === "string" ? value : JSON.stringify(value, null, 2);
  return text.split("\n").flatMap((line) => {
    const s = clean(line),
      out: string[] = [];
    for (let i = 0; i < s.length; i += width) out.push(s.slice(i, i + width));
    return out.length ? out : [""];
  });
};
export class GraphSession {
  graph?: Json;
  display?: DisplayGraph;
  detail: Json = "";
  selectedIndex = 0;
  mode: "nodes" | "edges" = "nodes";
  focus = 0;
  scroll = 0;
  help = false;
  sources?: Json[];
  sourceIndex = 0;
  viewport: Viewport = { width: 1, height: 1, x: 0, y: 0, zoom: 1 };
  private centerSelection = true;
  lastScale = 1;
  blocked?: Json;
  epoch = 0;
  closed = false;
  geometry!: Geometry;
  private render = new GraphRenderer();
  private abort = new AbortController();
  private root?: Ref;
  private explorer!: Explorer;
  private partial = false;
  constructor(
    private initial: Explorer,
    private input: string | Ref,
    readonly options: ViewOptions,
    readonly graphics: KittyGraphics,
  ) {}
  invalidate() {
    this.epoch++;
    this.abort.abort();
    this.abort = new AbortController();
    this.render.cancel();
  }
  private valid(epoch: number) {
    return !this.closed && epoch === this.epoch && !this.abort.signal.aborted;
  }
  private async authorize(epoch: number) {
    await this.initial.transport.call(
      "authorize",
      { history: false },
      this.initial.info.alias,
      this.abort.signal,
    );
    if (!this.valid(epoch)) throw new Failure({ code: "Cancelled" });
  }
  private async blank(message = "Checking current authority…") {
    // Also free any uploaded replacement whose epoch was invalidated before
    // placement/retain. Such an image must not occupy the next frame's budget.
    await this.graphics.clear();
    await this.graphics.writer.write(textScreen + clean(message));
  }
  async open() {
    const epoch = this.epoch;
    await this.blank();
    await this.authorize(epoch);
    this.explorer = await this.initial
      .fork({
        model: this.initial.info.alias,
        snapshot:
          this.initial.selection.selection === "Snapshot"
            ? this.initial.selection.reference
            : undefined,
        baseline:
          this.initial.selection.selection === "Baseline"
            ? this.initial.selection.reference
            : undefined,
        historyAnchor: this.initial.anchor,
        signal: this.abort.signal,
      })
      .open();
    this.root = await this.explorer.resolve(this.root ?? this.input);
    this.geometry = await this.graphics.probe(this.abort.signal);
    await this.reload(epoch);
    await this.draw(epoch);
  }
  private async reload(epoch: number) {
    this.explorer.options.signal = this.abort.signal;
    const graph = await extract(
      this.explorer,
      this.root!,
      this.options.projection,
      this.options.depth,
      this.options.vertices,
      this.options.edges,
    );
    if (!this.valid(epoch)) throw new Failure({ code: "Cancelled" });
    this.graph = graph;
    this.display = presentation(graph);
    this.partial = !graph.coverage.complete;
    this.blocked = undefined;
    this.mode = "nodes";
    this.selectedIndex = Math.max(
      0,
      graph.nodes.findIndex(
        (n: Json) => refKey(n.reference) === refKey(this.root!),
      ),
    );
    this.centerSelection = true;
    this.detail = this.inspection();
  }
  private selected() {
    return this.graph?.[this.mode]?.[this.selectedIndex];
  }
  private inspection(): Json {
    return {
      selected: this.selected() ?? "No recorded edges in requested scope",
      selection: this.graph?.selection,
      publication_anchor: this.graph?.publication_anchor,
      projection: this.options.projection,
      coverage: this.graph?.coverage,
      qualification:
        "derived/read only; viewport clipping is presentation; model draft, human review unrecorded; no business execution",
    };
  }
  private owner(): Ref | undefined {
    const selected = this.selected();
    return this.mode === "nodes"
      ? selected?.reference
      : selected?.origin?.owner;
  }
  async action(str: string, key: Json, epoch = this.epoch) {
    await this.blank();
    await this.authorize(epoch);
    if (this.blocked) {
      if (str !== "R") {
        await this.errorScreen();
        return;
      }
      await this.explorer?.release();
      this.explorer = await this.initial
        .fork({
          snapshot:
            this.initial.selection.selection === "Snapshot"
              ? this.initial.selection.reference
              : undefined,
          baseline:
            this.initial.selection.selection === "Baseline"
              ? this.initial.selection.reference
              : undefined,
          historyAnchor: this.initial.anchor,
          signal: this.abort.signal,
        })
        .open();
      this.geometry = await this.graphics.probe(this.abort.signal);
      await this.reload(epoch);
    } else {
      this.explorer.options.signal = this.abort.signal;
      if (str === "resize" || str === "resume") {
        this.geometry = await this.graphics.geometry(this.abort.signal);
        this.centerSelection = true;
      } else if (key.name === "tab") this.focus = 1 - this.focus;
      else if (str === "?") this.help = !this.help;
      else if (str === "e") {
        this.mode = this.mode === "nodes" ? "edges" : "nodes";
        this.selectedIndex = 0;
        this.sources = undefined;
        this.scroll = 0;
        this.centerSelection = true;
        this.detail = this.inspection();
      } else if (str === "n" || str === "p") {
        const count = this.sources?.length ?? this.graph[this.mode].length;
        if (this.sources)
          this.sourceIndex =
            (this.sourceIndex + (str === "n" ? 1 : -1) + count) %
            Math.max(1, count);
        else
          this.selectedIndex =
            (this.selectedIndex + (str === "n" ? 1 : -1) + count) %
            Math.max(1, count);
        if (!this.sources) this.centerSelection = true;
        this.scroll = 0;
        this.detail = this.sources
          ? {
              instruction: "Enter explicitly reads a bounded exact excerpt",
              selected_source: this.sources[this.sourceIndex]?.reference,
              sources: this.sources.map((r) => ({
                reference: r.reference,
                kind: r.data.kind,
              })),
            }
          : this.inspection();
      } else if (str === "s") {
        const owner = this.owner();
        if (owner) {
          const bindings = await this.explorer.sources(owner);
          if (!this.valid(epoch)) return;
          this.sources = bindings.records.filter((r: Json) =>
            ["Anchor", "SourceRevision"].includes(r.data.kind),
          );
          this.sourceIndex = 0;
          this.detail = {
            instruction:
              "n/p chooses an exact source; Enter reads up to 4096 bytes; e returns to graph selection",
            selected_source:
              this.sources![0]?.reference ?? "No recorded source excerpt",
            sources: this.sources!.map((r) => ({
              reference: r.reference,
              kind: r.data.kind,
            })),
            bindings,
          };
          this.focus = 1;
          this.scroll = 0;
        }
      } else if (key.name === "return") {
        if (this.sources?.length) {
          const preview = await this.explorer.preview(
            this.sources[this.sourceIndex].reference,
            "0",
            "4096",
          );
          this.detail = {
            media_type: preview.media_type,
            reference: preview.reference,
            text: preview.text,
            ...preview,
          };
        } else {
          const owner = this.owner();
          this.detail = {
            ...this.inspection(),
            native_origin_owner: owner
              ? await this.explorer.show(owner)
              : "unresolved/unrecorded owner",
          };
        }
        this.focus = 1;
        this.scroll = 0;
      } else if (str === "j" || key.name === "pagedown")
        this.scroll += key.name === "pagedown" ? 10 : 1;
      else if (str === "k" || key.name === "pageup")
        this.scroll = Math.max(
          0,
          this.scroll - (key.name === "pageup" ? 10 : 1),
        );
      else if (str === "+" || str === "=") {
        this.viewport.zoom = Math.min(
          8,
          (this.viewport.zoom || this.lastScale) * 1.25,
        );
        this.centerSelection = true;
      } else if (str === "-") {
        this.viewport.zoom = Math.max(
          0.01,
          (this.viewport.zoom || this.lastScale) / 1.25,
        );
        this.centerSelection = true;
      } else if (str === "0") {
        this.viewport.x = this.viewport.y = this.viewport.zoom = 0;
        this.centerSelection = false;
      } else if (["left", "right", "up", "down"].includes(key.name)) {
        const step = 80 / (this.viewport.zoom || this.lastScale);
        if (key.name === "left")
          this.viewport.x = Math.max(-1e7, this.viewport.x - step);
        if (key.name === "right")
          this.viewport.x = Math.min(1e7, this.viewport.x + step);
        if (key.name === "up")
          this.viewport.y = Math.max(-1e7, this.viewport.y - step);
        if (key.name === "down")
          this.viewport.y = Math.min(1e7, this.viewport.y + step);
      }
    }
    if (this.valid(epoch)) await this.draw(epoch);
  }
  private async draw(epoch: number) {
    if (!this.graph || !this.display || !this.valid(epoch)) return;
    const g = this.geometry,
      cols = Math.floor(g.columns * 0.57),
      body = g.rows - 9;
    if (g.columns < 90 || g.rows < 24) {
      await this.graphics.clear();
      await this.authorize(epoch);
      await this.graphics.writer.write(
        textScreen +
          "Window too small: need 90 columns × 24 rows. Exact selection retained. Resize or Esc/q close.",
      );
      return;
    }
    const inspectorWidth = g.columns - cols - 3;
    this.viewport.width = Math.floor((g.width / g.columns) * cols);
    this.viewport.height = Math.floor((g.height / g.rows) * body);
    if (this.help) {
      await this.authorize(epoch);
      await this.graphics.writer.write(
        textScreen +
          "View graph (Kitty) · keyboard help\n" +
          safeLines(viewerHelp, g.columns - 1).join("\n") +
          "\n? returns · Esc/q close · R retries the same exact selection",
      );
      return;
    }
    const frame = await this.render.render({
      display: this.display,
      viewport: this.viewport,
      selected: this.selected()?.id,
      center: this.centerSelection,
    });
    if (!this.valid(epoch)) return;
    this.lastScale = frame.scale ?? 1;
    if (frame.offset) {
      this.viewport.x = frame.offset.x;
      this.viewport.y = frame.offset.y;
    }
    this.centerSelection = false;
    await this.authorize(epoch);
    const id = await this.graphics.upload(
      frame.png,
      () => this.valid(epoch),
      this.abort.signal,
    );
    if (!this.valid(epoch)) return;
    await this.authorize(epoch);
    const coverage = this.graph.coverage;
    const lines = [
      `View graph (Kitty) · ${this.graph.model.alias} · ${this.options.projection} · derived READ ONLY`,
      `Content ${JSON.stringify(this.graph.selection)}`,
      `Publication anchor ${this.graph.publication_anchor}`,
      `${coverage.complete ? "COMPLETE requested extraction" : "PARTIAL"} · ${coverage.vertices} nodes / ${coverage.edges} edges · viewport clipped · model draft / review unrecorded`,
    ];
    const details = safeLines(this.detail, inspectorWidth);
    this.scroll = Math.min(this.scroll, Math.max(0, details.length - 1));
    for (let i = 0; i < body; i++)
      lines.push(" ".repeat(cols) + " │ " + (details[i + this.scroll] ?? ""));
    lines.push(
      `Focus ${this.focus ? "inspector" : "graph"} · ${this.mode} ${this.selectedIndex + 1}/${this.graph[this.mode].length} · ${clean(this.selected()?.id ?? "no recorded edge")} · no semantic writes`,
    );
    lines.push(...safeLines(viewerHelp, g.columns - 1));
    lines.push(
      coverage.warnings.join("; ") ||
        "Requested graph coverage only; no acceptance or execution inferred.",
    );
    const screen =
      textScreen +
      lines.map((s) => clean(s).slice(0, g.columns - 1)).join("\n");
    if (!this.valid(epoch)) return;
    await this.authorize(epoch);
    await this.graphics.reveal(
      id,
      cols,
      body,
      () => this.valid(epoch),
      this.abort.signal,
      screen,
    );
    if (this.valid(epoch)) await this.graphics.retain(id);
  }
  async fail(error: unknown) {
    const detail =
      error instanceof Failure
        ? error.detail
        : { code: "Operational", message: String(error) };
    if (detail.code === "Cancelled") return;
    this.invalidate();
    this.blocked = detail;
    this.graph = this.display = this.sources = undefined;
    this.detail = "";
    await this.graphics.clear();
    await this.errorScreen();
  }
  private async errorScreen() {
    await this.graphics.writer.write(
      textScreen +
        clean(
          `${this.blocked?.code}: ${this.blocked?.message ?? this.blocked?.action ?? "unavailable"}`,
        ) +
        "\n" +
        "R explicitly retries the pinned selection · Esc/q close. Alternatives: kh browse; kh diagram REF --format html --output NEW.html",
    );
  }
  async suspend() {
    this.invalidate();
    await this.blank(
      "Suspended graph view; resume rechecks current authority.",
    );
    await this.graphics.clear();
  }
  exitCode() {
    return this.blocked
      ? ["RenderLimit", "Partial"].includes(this.blocked.code)
        ? 3
        : exitFor(this.blocked)
      : this.partial
        ? 3
        : 0;
  }
  async close() {
    this.closed = true;
    this.invalidate();
    this.render.close();
    try {
      await this.graphics.close();
    } finally {
      await this.explorer?.release();
    }
  }
}
export async function viewGraph(
  initial: Explorer,
  root: string | Ref,
  options: ViewOptions,
  shared?: { input: TerminalInput; writer: TerminalWriter },
): Promise<number> {
  const input = shared?.input ?? new TerminalInput(process.stdin),
    writer = shared?.writer ?? new TerminalWriter(process.stdout);
  const graphics = new KittyGraphics(input, writer),
    session = new GraphSession(initial, root, options, graphics);
  const wasRaw = !!process.stdin.isRaw,
    wasPaused = process.stdin.isPaused();
  let done = false,
    cancelled = false,
    active = false,
    opening = true,
    queue: { str: string; key: Json }[] = [],
    failure: unknown;
  let finish!: () => void;
  const finished = new Promise<void>((r) => (finish = r));
  const stop = (cancel = false) => {
    if (done) return;
    cancelled = cancel;
    done = true;
    session.closed = true;
    queue = [];
    session.invalidate();
    finish();
  };
  const pump = async () => {
    if (active || opening || done) return;
    active = true;
    try {
      while (queue.length && !done) {
        const action = queue.shift()!;
        session.invalidate();
        try {
          await session.action(action.str, action.key);
        } catch (e) {
          await session.fail(e);
        }
      }
    } catch (e) {
      failure = e;
      stop();
    } finally {
      active = false;
    }
  };
  const enqueue = (str: string, key: Json = {}) => {
    if (
      str === "q" ||
      key.name === "escape" ||
      (key.ctrl && key.name === "c")
    ) {
      stop(key.ctrl && key.name === "c");
      return;
    }
    if (str === "resize") queue = queue.filter((a) => a.str !== "resize");
    if (active && !opening) session.invalidate();
    if (queue.length < 64) queue.push({ str, key });
    else {
      failure = new Failure({
        code: "Capacity",
        message:
          "64 pending keyboard actions reached; close and retry the pinned view",
      });
      stop();
      return;
    }
    void pump();
  };
  const keypress = (s: string, k: Json) => enqueue(s, k),
    resize = () => enqueue("resize"),
    terminate = () => stop(true);
  const suspend = () => {
    void session
      .suspend()
      .then(() => {
        process.stdin.setRawMode(wasRaw);
        return writer.write("\x1b[?25h");
      })
      .then(() => process.kill(process.pid, "SIGSTOP"))
      .catch((e) => {
        failure = e;
        stop();
      });
  };
  const resume = () => {
    process.stdin.setRawMode(true);
    enqueue("resume");
  };
  emitKeypressEvents(input.keys);
  process.stdin.setRawMode(true);
  process.stdin.resume();
  input.keys.on("keypress", keypress);
  process.stdout.on("resize", resize);
  for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"] as const)
    process.on(signal, terminate);
  process.on("SIGTSTP", suspend);
  process.on("SIGCONT", resume);
  try {
    await writer.write("\x1b[?25l");
    try {
      if (!done) await session.open();
    } catch (e) {
      if (!done) {
        await session.fail(e);
        if (!shared || (e as Failure).detail?.code === "GraphicsUnavailable")
          stop();
      }
    }
    opening = false;
    void pump();
    if (!done) await finished;
  } finally {
    done = true;
    session.invalidate();
    input.keys.off("keypress", keypress);
    process.stdout.off("resize", resize);
    for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"] as const)
      process.off(signal, terminate);
    process.off("SIGTSTP", suspend);
    process.off("SIGCONT", resume);
    try {
      await session.close();
    } catch (e) {
      failure = e;
    }
    try {
      await writer.write("\x1b[?25h\x1b[0m\n");
    } catch (e) {
      failure = e;
    }
    process.stdin.setRawMode(wasRaw);
    if (wasPaused || !shared) process.stdin.pause();
    if (!shared) input.close();
  }
  if (failure) throw failure;
  return cancelled ? 130 : session.exitCode();
}
