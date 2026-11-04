import { createHash } from "node:crypto";
import { Transport } from "./transport.ts";
import {
  Failure,
  exact,
  ontology,
  key,
  label,
  preparation,
  readBudgets,
  historyDefaults,
  type Ref,
  type Selection,
  type Json,
} from "./types.ts";

export type Options = {
  model?: string;
  snapshot?: string;
  baseline?: Ref;
  historyAnchor?: string;
  calls?: number;
  items?: number;
  bytes?: number;
  historyLimits?: Json;
  signal?: AbortSignal;
};
/** Local scripts share the CLI/TUI semantics without handling the private wire. */
export async function openExplorer(workspace: string, options: Options = {}) {
  const transport = new Transport(workspace);
  try {
    const explorer = new Explorer(transport, options);
    await transport.handshake();
    return await explorer.open();
  } catch (error) {
    await transport.close();
    throw error;
  }
}
export class Explorer {
  selection!: Selection;
  anchor!: string;
  info: Json;
  anchorStatus = "requested-unverified";
  options: Options;
  calls = 0;
  historyCalls = 0;
  items = 0;
  output = 0;
  partial = false;
  warnings: string[] = [];
  nativeWork: Record<string, string> = {};
  private tokens = new Map<string, boolean>();
  constructor(
    public transport: Transport,
    options: Options = {},
  ) {
    for (const [name, value, max, min] of [
      ["calls", options.calls, 1024, 1],
      ["items", options.items, 10000, 1],
      ["bytes", options.bytes, 8387584, 8192],
    ] as const)
      if (
        value !== undefined &&
        (!Number.isSafeInteger(value) || value < min || value > max)
      )
        throw new Failure({
          code: "Usage",
          message: `${name} outside ${min}..${max}; no clamping`,
        });
    const maxima: Record<string, bigint> = {
      items: 1000n,
      nodes: 8192n,
      storage_bytes: 134217728n,
      output_bytes: 4194304n,
    };
    for (const [name, value] of Object.entries(options.historyLimits ?? {}))
      if (
        !maxima[name] ||
        typeof value !== "string" ||
        !/^[1-9][0-9]*$/.test(value) ||
        BigInt(value) > maxima[name]
      )
        throw new Failure({
          code: "Usage",
          message: "invalid native history limit",
        });
    this.options = options;
  }
  async open() {
    this.info = await this.transport.call(
      "info",
      {},
      this.options.model,
      this.options.signal,
    );
    this.options.model = this.info.alias;
    if (this.options.snapshot && this.options.baseline)
      throw new Failure({
        code: "Usage",
        message: "choose snapshot or baseline",
      });
    this.selection = this.options.baseline
      ? { selection: "Baseline", reference: this.options.baseline }
      : {
          selection: "Snapshot",
          reference: this.options.snapshot ?? this.info.head,
        };
    this.anchor =
      this.options.historyAnchor ??
      (this.selection.selection === "Snapshot"
        ? this.selection.reference
        : this.info.head);
    if (!/^[a-f0-9]{64}$/.test(this.anchor))
      throw new Failure({
        code: "Usage",
        message: "publication anchor must be an exact lowercase snapshot hash",
      });
    if (this.anchor === this.info.head)
      this.anchorStatus = "captured-native-head";
    if (this.options.snapshot || this.options.baseline)
      await this.queryPage(undefined, false, undefined, "summary", {
        ids: ["knowledgehub-client:selection-validation"],
        page_size: "1",
      });
    return this;
  }
  fork(options: Options = {}) {
    return new Explorer(this.transport, { ...this.options, ...options });
  }
  stop(reason: string) {
    this.partial = true;
    if (!this.warnings.includes(reason)) this.warnings.push(reason);
  }
  async call(method: string, args: Json = {}, history = false): Promise<Json> {
    if (this.options.signal?.aborted) throw new Failure({ code: "Cancelled" });
    if (
      this.calls >= (this.options.calls ?? 512) ||
      (history && this.historyCalls >= 64)
    ) {
      this.stop(
        history
          ? "64 history data-read calls reached"
          : "aggregate native-call budget reached",
      );
      throw new Failure({ code: "Partial", message: this.warnings.at(-1) });
    }
    this.calls++;
    if (history) this.historyCalls++;
    try {
      const value = await this.transport.call(
        method,
        args,
        this.options.model,
        this.options.signal,
        this.options.bytes ?? 1048576,
      );
      if (history) this.anchorStatus = "verified-native-read";
      if (history && value.work)
        for (const [n, v] of Object.entries(value.work))
          this.nativeWork[n] = String(
            BigInt(this.nativeWork[n] ?? "0") + BigInt(v as string),
          );
      return value;
    } catch (e) {
      if (
        e instanceof Failure &&
        ["HistoryLimit", "Capacity", "IndexNotReady"].includes(e.detail.code)
      ) {
        this.stop(
          `${e.detail.code}: ${e.detail.resource ?? e.detail.message ?? e.detail.reason ?? "native/wire bound"}`,
        );
        if (history && e.detail.work)
          for (const [n, v] of Object.entries(e.detail.work))
            this.nativeWork[n] = String(
              BigInt(this.nativeWork[n] ?? "0") + BigInt(v as string),
            );
      }
      throw e;
    }
  }
  private admit(value: Json): boolean {
    const size = Buffer.byteLength(JSON.stringify(value));
    if (
      this.items >= (this.options.items ?? 10000) ||
      this.output + size > (this.options.bytes ?? 1048576) - 8192
    ) {
      this.stop(
        "aggregate item/output budget reached; rerun the pinned intent with narrower scope or permitted larger bounds",
      );
      return false;
    }
    this.items++;
    this.output += size;
    return true;
  }
  private track(page: Json, history: boolean) {
    if (page.continuation) this.tokens.set(page.continuation, history);
  }
  async release() {
    for (const [t, h] of this.tokens) {
      try {
        await this.transport.call(
          h ? "release_history" : "release_cursor",
          { continuation: t },
          this.options.model,
        );
      } catch (e) {
        if (!(
          e instanceof Failure &&
          ["InvalidCursor", "Denied", "Disconnected"].includes(e.detail.code)
        ))
          this.warnings.push("cursor cleanup failed; host drain required");
      }
    }
    this.tokens.clear();
  }
  async close() {
    await this.release();
    await this.transport.close();
  }
  async queryPage(
    type?: Ref,
    includeSubtypes = false,
    token?: string,
    projection = "summary",
    extra: Json = {},
  ): Promise<Json> {
    const request = {
      selection: this.selection,
      type_ref: type ?? null,
      include_subtypes: includeSubtypes,
      ids: [],
      starts: [],
      signatures: [],
      direction: "Forward",
      depth: "1",
      page_size: "100",
      subject_limit: "10000",
      edge_limit: "10000",
      preparation,
      ...extra,
    };
    const p = await this.call(token ? "continue_query" : "query", {
      request,
      projection,
      ...(token ? { continuation: token } : {}),
    });
    if (token) this.tokens.delete(token);
    this.track(p, false);
    return p;
  }
  async list(type?: Ref, subtypes = false, extra: Json = {}): Promise<Json[]> {
    const out: Json[] = [];
    let token: string | undefined;
    do {
      let p: Json;
      try {
        p = await this.queryPage(type, subtypes, token, "summary", extra);
      } catch (error) {
        if (
          error instanceof Failure &&
          ["Partial", "Capacity", "IndexNotReady"].includes(error.detail.code)
        )
          return out;
        throw error;
      }
      for (const r of p.items) {
        if (!this.admit(r)) return out;
        out.push(r);
      }
      token = p.continuation ?? undefined;
      if (p.truncated) this.stop("native query subject/edge bound reached");
      if (!token && !p.exhausted) this.stop("native query is incomplete");
    } while (token);
    return out;
  }
  async resolve(input: string | Ref, types = false): Promise<Ref> {
    if (typeof input !== "string") return input;
    if (input.startsWith("{")) {
      let r: Json;
      try {
        r = JSON.parse(input);
      } catch {
        throw new Failure({
          code: "Usage",
          message: "exact reference must be JSON {model,id,revision}",
        });
      }
      if (
        typeof r.model !== "string" ||
        typeof r.id !== "string" ||
        typeof r.revision !== "string" ||
        Object.keys(r).length !== 3
      )
        throw new Failure({
          code: "Usage",
          message: "invalid exact reference",
        });
      return r;
    }
    const candidates = (
      await this.list(types ? exact("ElementTypeDefinition") : undefined)
    ).filter(
      (r) =>
        r.reference.id === input ||
        r.name === input ||
        (types && r.reference.id === `sdo:type:${input}`),
    );
    if (candidates.length === 1 && !this.partial)
      return candidates[0].reference;
    if (candidates.length > 1)
      throw new Failure({
        code: "Ambiguous",
        message: "explicit exact reference required",
        candidates: candidates.map((r) => ({
          reference: r.reference,
          name: r.name,
        })),
      });
    throw new Failure({
      code: this.partial ? "Partial" : "NotFound",
      message: this.partial
        ? "resolution coverage incomplete; supply an exact reference"
        : "no exact ID/display-name match in selected records",
    });
  }
  async read(reference: Ref, selection = this.selection) {
    return this.call("read", { selection, reference });
  }
  async show(input: string | Ref): Promise<Json> {
    const r = await this.read(await this.resolve(input));
    const p = await this.queryPage(undefined, false, undefined, "summary", {
      ids: [r.reference.id],
    });
    return {
      record: r,
      membership: p.items.some(
        (x: Json) => key(x.reference) === key(r.reference),
      )
        ? "selected"
        : "historical/supporting",
      selection: this.selection,
    };
  }
  async types() {
    return this.list(exact("ElementTypeDefinition"));
  }
  async schema(input: string | Ref) {
    const reference = await this.resolve(input, true),
      records: Json[] = [],
      seen = new Set<string>();
    const walk = async (r: Ref) => {
      if (seen.has(key(r))) return;
      seen.add(key(r));
      const record = await this.read(r);
      if (record.data.kind !== "Type")
        throw new Failure({
          code: "Usage",
          message: "not a registered type definition",
        });
      for (const p of record.data.data.parents) await walk(p);
      records.push(record);
    };
    await walk(reference);
    return {
      reference,
      definitions: records,
      properties: records.flatMap((r) =>
        Object.entries(r.data.data.properties).map(([path, schema]) => ({
          path,
          declaring_type: r.reference,
          schema,
        })),
      ),
      note: "All declaring definitions are retained; inherited collisions are not silently overwritten.",
    };
  }
  neighbourRequest(starts: Ref[], extra: Json = {}) {
    return {
      selection: this.selection,
      starts,
      direction: "Both",
      selectors: [],
      categories: ["Association", "Property"],
      opposite_type: null,
      include_subtypes: false,
      fact_scope: "Selected",
      projection: "References",
      page_size: "100",
      examined_limit: "10000",
      byte_limit: "262144",
      preparation,
      ...extra,
    };
  }
  async linkPage(starts: Ref[], extra: Json = {}, token?: string) {
    const request = this.neighbourRequest(starts, extra);
    const p = await this.call(token ? "continue_neighbours" : "neighbours", {
      request,
      ...(token ? { continuation: token } : {}),
    });
    if (token) this.tokens.delete(token);
    this.track(p, false);
    return p;
  }
  async links(input: string | Ref, extra: Json = {}): Promise<Json> {
    const reference = await this.resolve(input),
      neighbours: Json[] = [],
      vertices = new Map<string, Json>();
    let token: string | undefined;
    do {
      let p: Json;
      try {
        p = await this.linkPage([reference], extra, token);
      } catch (error) {
        if (
          error instanceof Failure &&
          ["Partial", "Capacity", "IndexNotReady"].includes(error.detail.code)
        )
          return {
            reference,
            neighbours,
            vertices: [...vertices.values()],
            stopped_error: error.detail,
          };
        throw error;
      }
      for (const v of p.vertices) vertices.set(key(v.reference), v);
      for (const n of p.neighbours) {
        if (!this.admit(n))
          return { reference, neighbours, vertices: [...vertices.values()] };
        neighbours.push(n);
      }
      token = p.continuation ?? undefined;
      if (!token && !p.exhausted)
        this.stop(`native links stopped at ${p.reached}`);
    } while (token);
    return { reference, neighbours, vertices: [...vertices.values()] };
  }
  async tree(input: string | Ref, depth = 2, extra: Json = {}): Promise<Json> {
    const root = await this.resolve(input),
      seen = new Set<string>();
    const visit = async (
      r: Ref,
      d: number,
      path: Set<string>,
    ): Promise<Json> => {
      const node: Json = { reference: r };
      if (path.has(key(r))) {
        node.cycle = true;
        return node;
      }
      if (seen.has(key(r))) {
        node.shared = true;
        return node;
      }
      seen.add(key(r));
      if (!d) {
        node.boundary = true;
        return node;
      }
      const links = await this.links(r, extra);
      node.children = [];
      for (const edge of links.neighbours) {
        const next = new Set(path);
        next.add(key(r));
        node.children.push({
          edge,
          node: await visit(edge.opposite, d - 1, next),
        });
      }
      return node;
    };
    return {
      meaning:
        "bounded recorded-reference projection; edges retain native origin; repeated identities are references",
      root: await visit(root, depth, new Set()),
    };
  }
  async search(
    phrase: string,
    typeInput: string | Ref,
    path = "/name",
    mode = "literal-utf8/1",
  ) {
    const type = await this.resolve(typeInput, true);
    const result = await this.call("search_text", {
      request: {
        selection: this.selection,
        text: { phrase, mode, fields: [{ type_ref: type, path }] },
        budgets: readBudgets,
      },
    });
    if (!result.exhausted || result.reached.length)
      this.stop(
        `native literal field search reached ${result.reached.join(",")}`,
      );
    return result;
  }
  async sources(input: string | Ref): Promise<Json> {
    const reference = await this.resolve(input),
      found = new Map<string, Ref>();
    const record = await this.read(reference);
    const walk = (v: Json) => {
      if (!v || typeof v !== "object") return;
      if (v.kind === "Narrative") {
        found.set(key(v.value.fragment), v.value.fragment);
        found.set(key(v.value.document), v.value.document);
      }
      if (Array.isArray(v)) v.forEach(walk);
      else Object.values(v).forEach(walk);
    };
    walk(record.data);
    for (const p of record.provenance) found.set(key(p), p);
    if (
      [
        "Anchor",
        "SourceRevision",
        "SourceArtifact",
        "Interpretation",
        "Representation",
        "Review",
      ].includes(record.data.kind)
    )
      found.set(key(reference), reference);
    const out: Json[] = [];
    for (const ref of found.values()) {
      const r = await this.read(ref);
      out.push(r);
      if (r.data.kind === "Anchor")
        found.set(key(r.data.data.source), r.data.data.source);
      if (r.data.kind === "SourceRevision")
        found.set(key(r.data.data.source), r.data.data.source);
      if (r.data.kind === "Interpretation")
        found.set(key(r.data.data.anchor), r.data.data.anchor);
      if (out.length >= 100) {
        this.stop("source/provenance walk limited to 100 records");
        break;
      }
    }
    return {
      reference,
      records: out,
      note: "Exact source and interpretation status; no approval/execution inference",
    };
  }
  async preview(input: string | Ref, offset = "0", length = "65536") {
    const r = await this.read(await this.resolve(input));
    let source: Ref = r.reference;
    if (r.data.kind === "Anchor") source = r.data.data.source;
    else if (r.data.kind !== "SourceRevision")
      throw new Failure({
        code: "Usage",
        message:
          "preview/export requires an exact SourceRevision or Anchor; sources lists the bindings",
      });
    const record = await this.read(source);
    const media = record.data.data.media_type;
    const remaining = BigInt(record.data.data.original.length) - BigInt(offset);
    if (remaining < 0n)
      throw new Failure({
        code: "Usage",
        message: "preview offset exceeds original length",
      });
    const actual = BigInt(length) < remaining ? BigInt(length) : remaining;
    const p = await this.call("artifact_range", {
      selection: this.selection,
      reference: source,
      offset,
      length: String(actual),
    });
    const bytes = Buffer.from(p.bytes_base64, "base64");
    let text: string | undefined;
    let unsupported: string | undefined;
    if (!/^(text\/|application\/(json|javascript|xml))/.test(media))
      unsupported =
        "binary/unsupported media; use explicit original-byte export";
    else
      try {
        text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
      } catch {
        unsupported =
          "range is not complete UTF-8 text; choose aligned range or original export";
      }
    return {
      ...p,
      media_type: media,
      text,
      unsupported,
      partial:
        BigInt(offset) !== 0n ||
        BigInt(bytes.length) < BigInt(p.original.length),
      bytes_base64: undefined,
    };
  }
  async original(
    input: string | Ref,
  ): Promise<{ reference: Ref; bytes: Buffer; original: Json }> {
    const r = await this.read(await this.resolve(input));
    const reference =
      r.data.kind === "Anchor" ? r.data.data.source : r.reference;
    const source = await this.read(reference);
    if (source.data.kind !== "SourceRevision")
      throw new Failure({
        code: "Usage",
        message: "original export requires SourceRevision/Anchor",
      });
    const original = source.data.data.original;
    if (BigInt(original.length) > 8388608n)
      throw new Failure({
        code: "Capacity",
        message: "original export capped at 8 MiB; no partial original written",
      });
    const chunks: Buffer[] = [];
    for (let offset = 0n; offset < BigInt(original.length);) {
      const p = await this.call("artifact_range", {
        selection: this.selection,
        reference,
        offset: String(offset),
        length: String(
          BigInt(original.length) - offset > 65536n
            ? 65536n
            : BigInt(original.length) - offset,
        ),
      });
      const bytes = Buffer.from(p.bytes_base64, "base64");
      if (!bytes.length)
        throw new Failure({
          code: "Integrity",
          message: "source made no progress",
        });
      chunks.push(bytes);
      offset += BigInt(bytes.length);
    }
    const bytes = Buffer.concat(chunks);
    if (
      String(bytes.length) !== String(original.length) ||
      createHash("sha256").update(bytes).digest("hex") !== original.sha256
    )
      throw new Failure({
        code: "Integrity",
        message: "original source digest mismatch",
      });
    return { reference, original, bytes };
  }
  async lineage(input: string | Ref) {
    let reference = await this.resolve(input);
    const records: Json[] = [];
    const seen = new Set<string>();
    while (reference) {
      if (seen.has(key(reference)))
        throw new Failure({ code: "Integrity", message: "predecessor cycle" });
      seen.add(key(reference));
      const r = await this.read(reference);
      records.push(r);
      if (!r.predecessor) break;
      if (records.length >= 100) {
        this.stop("predecessor lineage limited to 100 exact records");
        break;
      }
      reference = r.predecessor;
    }
    return {
      meaning:
        "exact predecessor lineage; distinct from navigation, baseline parents and publications",
      records,
    };
  }
  private limits() {
    return { ...historyDefaults, ...this.options.historyLimits };
  }
  async publication(operation_id: string) {
    return this.call(
      "publication",
      { request: { anchor: this.anchor, operation_id, limits: this.limits() } },
      true,
    );
  }
  async historyPage(operation_id?: string, token?: string) {
    const request = {
      anchor: this.anchor,
      ...(operation_id ? { operation_id } : {}),
      limits: this.limits(),
    };
    const p = await this.call(
      token
        ? "continue_history"
        : operation_id
          ? "publication_affected"
          : "publications",
      token
        ? {
            request: operation_id
              ? { Affected: request }
              : { Publications: request },
            continuation: token,
          }
        : { request },
      true,
    );
    if (token) this.tokens.delete(token);
    this.track(p, true);
    return p;
  }
  async history(operation_id?: string) {
    const items: Json[] = [],
      pages: Json[] = [];
    let token: string | undefined;
    let stopped_error: Json;
    do {
      let p: Json;
      try {
        p = await this.historyPage(operation_id, token);
      } catch (error) {
        if (
          error instanceof Failure &&
          ["Partial", "HistoryLimit", "Capacity", "IndexNotReady"].includes(
            error.detail.code,
          )
        ) {
          stopped_error = error.detail;
          break;
        }
        throw error;
      }
      const values = operation_id
        ? p.items.Affected.references
        : p.items.Publications;
      for (const r of values) {
        if (!this.admit(r)) return { items, pages, complete: false };
        items.push(r);
      }
      pages.push({
        ...p,
        items: operation_id
          ? { Affected: { ...p.items.Affected, references: undefined } }
          : { Publications: undefined },
        continuation: undefined,
      });
      token = p.continuation ?? undefined;
      if (!token && !p.complete)
        this.stop(`native history stopped by ${p.stopped_by}`);
      if (
        token &&
        !values.length &&
        BigInt(p.work.visited_nodes ?? "0") === 0n &&
        BigInt(p.work.object_reads ?? "0") === 0n
      ) {
        this.stop(
          "native history made no progress; fresh pinned restart required",
        );
        break;
      }
    } while (token);
    return {
      items,
      pages,
      stopped_error,
      complete: !this.partial,
      meaning: operation_id
        ? "Original affected vector order, including repeated references"
        : "Descending accepted publication sequence; no timestamps or review inference",
    };
  }
  async baselines(retained = false) {
    const all = new Map<string, Json>();
    const add = (r: Json, origin: string, head?: string) => {
      const k = key(r.reference);
      const previous = all.get(k);
      if (previous) {
        previous.origins.push(origin);
        return;
      }
      all.set(k, {
        reference: r.reference,
        origins: [origin],
        published_head: head,
        predecessor: r.predecessor,
        parents: r.data?.data?.parents,
      });
    };
    for (const h of await this.list(exact("ModelBaseline")))
      add(await this.read(h.reference), "selected");
    for (const r of this.info.known_baselines)
      add(await this.read(r), "validated-handover");
    if (retained) {
      await this.call("authorize", { history: true });
      const publications = await this.history();
      for (const h of publications.items) {
        if (this.partial) break;
        const affected = await this.history(h.operation_id);
        for (const r of affected.items) {
          if (this.partial) break;
          const record = await this.read(r, {
            selection: "Snapshot",
            reference: h.head,
          });
          if (record.data.kind === "Baseline")
            add(record, "retained-publication", h.head);
        }
      }
    }
    if (retained)
      await this.transport.call(
        "authorize",
        { history: true },
        this.options.model,
        this.options.signal,
      );
    return {
      baselines: [...all.values()],
      selected_and_handover: true,
      retained: {
        requested: retained,
        anchor: this.anchor,
        complete: retained && !this.partial,
      },
      note: retained
        ? "Composed publication/affected/exact-read discovery; no native retained-baseline catalogue"
        : "Selected query and exact handover only; retained inventory unexamined",
    };
  }
  async compare(before: Ref, after: Ref) {
    return {
      before,
      after,
      contract:
        "existing unpaged native exact-baseline comparison; no snapshot-anchor parameter",
      differences: await this.call("compare", { before, after }),
    };
  }
  async gaps() {
    const p = await this.call("traverse", {
      request: {
        selection: this.selection,
        starts: [ontology],
        plan: {
          kind: "Repeated",
          alternatives: [
            {
              direction: "Outgoing",
              selectors: [
                {
                  kind: "Native",
                  definition: exact("ElementTypeDefinition"),
                  path: "/parents/*",
                },
              ],
              categories: ["Administrative"],
              opposite_type: null,
              include_subtypes: false,
            },
          ],
        },
        target: {
          type_ref: exact("ElementTypeDefinition"),
          ids: [],
          include_subtypes: false,
        },
        fact_scope: "Selected",
        projection: {
          target_records: false,
          witness_records: false,
          snippets: false,
        },
        budgets: { ...readBudgets, hops: "1" },
      },
    });
    if (p.coverage.omitted)
      this.stop("native coverage gap projection omitted entries");
    return {
      coverage: p.coverage,
      note: "Native bounded coverage metadata from a declared type-parent traversal; not a complete gap catalogue",
      work: p.work,
    };
  }
  async groups(view: string) {
    if (view === "type")
      return {
        groups: await this.types(),
        meaning: "exact registered types; expand lazily",
      };
    const names: Record<string, string> = {
      context: "BoundedContext",
      journey: "Journey",
      source: "SourceArtifact",
    };
    if (!names[view])
      throw new Failure({
        code: "Usage",
        message: "view is type, context, journey or source",
      });
    return {
      groups: await this.list(exact(names[view])),
      unassigned: {
        virtual: true,
        name: "Unassigned / not examined",
        meaning:
          "No membership is inferred. Expand to bounded unassigned inspection.",
      },
      meaning:
        "virtual recorded-reference grouping; expansion uses native occurrences, not names or inferred containment",
    };
  }
  async groupMembers(view: string, reference?: Ref) {
    if (view === "type" && reference)
      return {
        items: await this.list(reference),
        meaning: "selected exact type instances",
      };
    if (reference) {
      const links = await this.links(reference, {
        categories:
          view === "source"
            ? ["Provenance", "Representation", "Property"]
            : ["Association", "Property"],
      });
      return {
        items: links.neighbours.map((n: Json) => ({
          reference: n.opposite,
          origin: n.origin,
          selector: n.selector,
          directions: n.directions,
        })),
        meaning:
          "recorded references; virtual membership does not establish containment",
        vertices: links.vertices,
      };
    }
    const groups = (await this.groups(view)).groups,
      assigned = new Set<string>();
    for (const g of groups) {
      const p = await this.groupMembers(view, g.reference);
      for (const x of p.items) assigned.add(key(x.reference));
    }
    const items = (await this.list()).filter(
      (r) => !assigned.has(key(r.reference)) && r.reference.model !== "sdo",
    );
    return {
      items,
      meaning:
        "No recorded one-hop membership in the inspected group set; bounded coverage applies",
    };
  }
  envelope(intent: Json, data: Json, error?: Json) {
    return {
      format: "knowledgehub-exploration/1",
      status: error
        ? this.partial
          ? "partial"
          : "error"
        : this.partial
          ? "partial"
          : "complete",
      model: this.info
        ? {
            alias: this.info.alias,
            scope: this.info.scope,
            reader: this.info.reader,
          }
        : this.options.model,
      selection: this.selection,
      publication_anchor: this.anchor,
      publication_anchor_status: this.anchorStatus,
      intent,
      data,
      coverage: {
        complete: !this.partial && !error,
        native_calls: String(this.calls),
        history_data_calls: String(this.historyCalls),
        collected_items: String(this.items),
        native_history_work: this.nativeWork,
        transport_response_bytes: String(this.transport.wireBytes),
        process_local_cursors:
          "released; rerun pinned intent, never reuse a token after exit",
      },
      warnings: this.warnings,
      error,
    };
  }
}
