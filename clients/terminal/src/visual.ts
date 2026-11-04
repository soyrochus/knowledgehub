import { createHash } from "node:crypto";
import { Explorer } from "./explorer.ts";
import { Failure, exact, key, label, type Ref, type Json } from "./types.ts";
import { outputPath, writeOutput } from "./output.ts";
import renderer from "../generated/renderer.js" with { type: "text" };
export const digest = (v: Json) =>
  createHash("sha256").update(JSON.stringify(v)).digest("hex");
// Display syntax/control characters visibly; the DTO and inspector keep exact text.
export const mermaidText = (s: string) =>
  s.replace(
    /[<>\"`\\[\]{}|&#\x00-\x1f\x7f-\x9f\u2028\u2029]/g,
    (c) =>
      ({
        "<": "‹",
        ">": "›",
        '"': "”",
        "`": "′",
        "\\": "∖",
        "[": "⟦",
        "]": "⟧",
        "{": "⦃",
        "}": "⦄",
        "|": "¦",
        "&": "＆",
        "#": "＃",
      })[c] ??
      `U+${c.charCodeAt(0).toString(16).toUpperCase().padStart(4, "0")}`,
  );
const refs = (v: Json): Ref[] =>
  v?.kind === "Ref"
    ? [v.value]
    : ["List", "Set"].includes(v?.kind)
      ? v.value.flatMap(refs)
      : [];
const sourceBindings = (r: Json) => ({
  provenance: r.provenance,
  narratives: findNarratives(r.data),
});
function findNarratives(v: Json): Json[] {
  if (!v || typeof v !== "object") return [];
  if (v.kind === "Narrative") return [v.value];
  return Object.values(v).flatMap(findNarratives);
}
export async function extract(
  e: Explorer,
  input: string | Ref,
  projection = "relationships",
  depth = 1,
  maxVertices = 200,
  maxEdges = 500,
): Promise<Json> {
  const root = await e.resolve(input),
    nodes = new Map<string, Json>(),
    edges = new Map<string, Json>(),
    warnings: string[] = [];
  if (
    !Number.isSafeInteger(depth) ||
    depth < 1 ||
    depth > 8 ||
    !Number.isSafeInteger(maxVertices) ||
    maxVertices < 1 ||
    maxVertices > 1000 ||
    !Number.isSafeInteger(maxEdges) ||
    maxEdges < 1 ||
    maxEdges > 10000
  )
    throw new Failure({
      code: "Usage",
      message: "invalid finite diagram bounds",
    });
  let graphBytes = 4096;
  const admit = (value: Json) => {
    const n = Buffer.byteLength(JSON.stringify(value));
    if (graphBytes + n > (e.options.bytes ?? 1048576) - 4096) {
      e.stop("diagram data output byte budget reached");
      return false;
    }
    graphBytes += n;
    return true;
  };
  const node = async (r: Ref, metadata: Json = {}) => {
    const k = key(r);
    if (nodes.has(k)) {
      Object.assign(nodes.get(k), metadata);
      return;
    }
    if (nodes.size >= maxVertices) {
      e.stop("diagram vertex limit reached");
      return;
    }
    try {
      const inspected = await e.show(r),
        record = inspected.record;
      const value: Json = {
        id: `n${digest(r).slice(0, 20)}`,
        reference: r,
        type_ref: record.type_ref,
        label: label(record),
        record,
        available: true,
        membership: inspected.membership,
        source_bindings: sourceBindings(record),
        ...metadata,
      };
      if (admit(value)) nodes.set(k, value);
    } catch (error) {
      if (error instanceof Failure && error.detail.code === "NotFound") {
        nodes.set(k, {
          id: `n${digest(r).slice(0, 20)}`,
          reference: r,
          label: r.id,
          available: false,
          membership: "external/unresolved",
          ...metadata,
        });
        warnings.push(`unresolved ${key(r)}`);
        e.stop("unresolved diagram endpoint");
      } else throw error;
    }
  };
  const edge = (
    from: Ref,
    to: Ref,
    origin: Json,
    selector: Json,
    category: string,
    extra: Json = {},
  ) => {
    if (!nodes.has(key(from)) || !nodes.has(key(to))) {
      e.stop("diagram edge endpoint omitted by vertex limit");
      return;
    }
    const identity = { from, to, origin, selector, category };
    const id = `e${digest(identity).slice(0, 20)}`;
    if (edges.has(id)) return;
    if (edges.size >= maxEdges) {
      e.stop("diagram edge limit reached");
      return;
    }
    const value = { id, from, to, origin, selector, category, ...extra };
    if (admit(value)) edges.set(id, value);
  };
  await node(root);
  if (projection === "relationships") {
    let frontier = [root];
    const visited = new Set<string>();
    for (let d = 0; d < depth; d++) {
      const next: Ref[] = [];
      for (const r of frontier) {
        if (visited.has(key(r))) continue;
        visited.add(key(r));
        const p = await e.links(r);
        for (const v of p.vertices)
          await node(v.reference, {
            available: v.available,
            membership: v.selected
              ? "selected"
              : v.available
                ? "supporting"
                : "external/unresolved",
          });
        for (const n of p.neighbours) {
          const incoming = n.directions.includes("Incoming");
          const from = incoming ? n.opposite : n.subject,
            to = incoming ? n.subject : n.opposite;
          await node(from);
          await node(to);
          edge(from, to, n.origin, n.selector, n.category, {
            directions: n.directions,
          });
          next.push(n.opposite);
        }
      }
      frontier = next;
      if (e.partial) break;
    }
  } else if (projection === "lifecycle") {
    const record = await e.read(root);
    if (key(record.type_ref) !== key(exact("LifecycleDefinition")))
      throw new Failure({
        code: "Usage",
        message:
          "lifecycle projection requires exact registered LifecycleDefinition identity",
      });
    const p = record.data.data.properties;
    for (const s of refs(p.states))
      await node(s, { membership: "declared lifecycle state" });
    for (const t of refs(p.transitions)) {
      const record = await e.read(t),
        v = record.data.data.properties,
        from = refs(v.from)[0],
        to = refs(v.to)[0];
      if (!from || !to) {
        warnings.push(`transition ${key(t)} lacks declared endpoints`);
        e.stop("unsupported incomplete transition");
        continue;
      }
      await node(from);
      await node(to);
      const facts = await e.links(t, {
        direction: "Outgoing",
        categories: ["Property"],
        selectors: [
          { kind: "Property", definition: record.type_ref, path: "/from" },
          { kind: "Property", definition: record.type_ref, path: "/to" },
        ],
      });
      const fromFact = facts.neighbours.find(
        (n: Json) =>
          n.selector.path === "/from" && key(n.opposite) === key(from),
      );
      const toFact = facts.neighbours.find(
        (n: Json) => n.selector.path === "/to" && key(n.opposite) === key(to),
      );
      if (!fromFact || !toFact) {
        e.stop("explicit transition endpoint occurrences unavailable");
        continue;
      }
      edge(
        from,
        to,
        fromFact.origin,
        {
          kind: "ExplicitTransition",
          definition: record.type_ref,
          from_path: "/from",
          to_path: "/to",
        },
        "declared-transition",
        {
          label: label(record),
          transition: record,
          supporting_origins: [toFact.origin],
          projection_rule:
            "Compose the declared Transition.from and Transition.to occurrences; this edge is a derived transition projection.",
          guards: refs(v.guards),
          effects: refs(v.effects),
          source_bindings: sourceBindings(record),
        },
      );
    }
    warnings.push(
      "Only declared transitions; expiry is descriptive proposed design, no running timer.",
    );
  } else if (projection === "journey") {
    const record = await e.read(root);
    if (key(record.type_ref) !== key(exact("Journey")))
      throw new Failure({
        code: "Usage",
        message:
          "journey projection requires exact registered Journey identity",
      });
    let found = false;
    for (const h of await e.list(exact("StoryMap"))) {
      const map = await e.read(h.reference),
        p = map.data.data.properties;
      if (!refs(p.journeys).some((r) => key(r) === key(root))) continue;
      const activities = refs(p.activities);
      const facts = await e.links(map.reference, {
        direction: "Outgoing",
        categories: ["Property"],
        selectors: [
          { kind: "Property", definition: map.type_ref, path: "/activities/*" },
        ],
      });
      for (const a of activities)
        await node(a, { membership: "declared ordered StoryMap activity" });
      for (let i = 1; i < activities.length; i++) {
        const fromFact = facts.neighbours.find(
          (n: Json) =>
            n.origin.occurrence === `/activities/${i - 1}` &&
            key(n.opposite) === key(activities[i - 1]),
        );
        const toFact = facts.neighbours.find(
          (n: Json) =>
            n.origin.occurrence === `/activities/${i}` &&
            key(n.opposite) === key(activities[i]),
        );
        if (!fromFact || !toFact) {
          e.stop("declared activity-order occurrences unavailable");
          continue;
        }
        edge(
          activities[i - 1],
          activities[i],
          fromFact.origin,
          {
            kind: "DeclaredListOrder",
            definition: map.type_ref,
            path: "/activities",
          },
          "declared-order",
          {
            label: "recorded list order",
            supporting_origins: [toFact.origin],
            source_bindings: sourceBindings(map),
            projection_rule:
              "Adjacent occurrences in the declared ordered StoryMap.activities list; no order inferred from names.",
          },
        );
      }
      found = true;
    }
    if (!found) {
      warnings.push(
        "No explicit StoryMap activities/order found; no sequence inferred.",
      );
      e.stop("journey order unsupported by selected definitions");
    }
  } else
    throw new Failure({
      code: "Usage",
      message: "projection is relationships, journey or lifecycle",
    });
  return {
    format: "knowledgehub-view/1",
    model: { alias: e.info.alias, scope: e.info.scope },
    selection: e.selection,
    publication_anchor: e.anchor,
    intent: {
      root,
      projection,
      depth,
      max_vertices: maxVertices,
      max_edges: maxEdges,
    },
    nodes: [...nodes.values()].sort((a, b) => a.id.localeCompare(b.id)),
    edges: [...edges.values()].sort((a, b) => a.id.localeCompare(b.id)),
    coverage: {
      complete: !e.partial,
      warnings: [...e.warnings, ...warnings],
      vertices: String(nodes.size),
      edges: String(edges.size),
    },
    status:
      "derived inspection artifact; no native import, backup, execution or acceptance",
  };
}
export function mermaid(graph: Json): string {
  const ids = new Map<string, string>(
    graph.nodes.map((n: Json) => [key(n.reference), n.id]),
  );
  let s = "flowchart LR\n";
  for (const n of graph.nodes)
    s += `  ${n.id}["${mermaidText(n.label || n.reference.id)}"]\n`;
  for (const e of graph.edges) {
    const a = ids.get(key(e.from)),
      b = ids.get(key(e.to));
    if (a && b)
      s += `  ${a} ${e.id}@-->|"${mermaidText(e.label ?? e.selector.path ?? e.selector.definition.id.replace(/^sdo:relationship:/, ""))}"| ${b}\n`;
  }
  return s;
}
export function html(graph: Json): string {
  const payload = JSON.stringify({ graph, mermaid: mermaid(graph) }).replace(
    /[<>&\u2028\u2029]/g,
    (c) => `\\u${c.charCodeAt(0).toString(16).padStart(4, "0")}`,
  );
  return `<!doctype html><html lang="en"><meta charset="utf-8"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src data:; font-src data:; connect-src 'none'; base-uri 'none'; form-action 'none'"><title>KnowledgeHub derived view</title><style>body{margin:0;font:14px system-ui;color:#183143;background:#f4f7fa}header{padding:16px;background:#183143;color:white}h1{font-size:20px;margin:0 0 8px}button{padding:8px;margin:5px}main{display:grid;grid-template-columns:3fr 2fr;height:78vh}#viewport{overflow:auto;background:white;touch-action:none}#drawing{transform-origin:0 0;padding:20px}svg{max-width:none!important}aside{overflow:auto;padding:16px;border-left:1px solid #ccd}pre{white-space:pre-wrap;overflow-wrap:anywhere}#error{color:#a21}svg .node,svg .edgePaths path{cursor:pointer}footer{padding:8px} @media(max-width:700px){main{grid-template-columns:1fr;height:auto}#viewport{height:50vh}}</style><header><h1>KnowledgeHub · derived inspection view</h1><div id="banner"></div><div>Legend: selected / supporting / external; assertion and property occurrences retain exact owners. No execution or acceptance.</div></header><nav><button id="zoom-in">Zoom +</button><button id="zoom-out">Zoom −</button><button id="reset">Reset view</button><span>Drag background to pan. Select a node or edge to inspect its native origin.</span></nav><main><section id="viewport"><div id="drawing"></div><pre id="error"></pre></section><aside><h2>Exact reference / source inspector</h2><pre id="inspector">Select a node or edge.</pre></aside></main><footer id="coverage"></footer><script id="view-data" type="application/json">${payload}</script><script>${renderer.replace(/<\/script/gi, "<\\/script")}</script></html>`;
}
export function exportView(
  graph: Json,
  format: string,
  path: string,
  workspace: string,
  replace = false,
) {
  let content: string;
  if (format === "json") content = JSON.stringify(graph, null, 2) + "\n";
  else if (format === "mermaid") content = mermaid(graph);
  else if (format === "html") content = html(graph);
  else
    throw new Failure({
      code: "Usage",
      message: "format is json, mermaid or html",
    });
  const manifestPath = path + ".manifest.json";
  outputPath(path, workspace, replace);
  outputPath(manifestPath, workspace, replace);
  const manifest = {
    format: "knowledgehub-view-manifest/1",
    selection: graph.selection,
    publication_anchor: graph.publication_anchor,
    intent: graph.intent,
    coverage: graph.coverage,
    graph_sha256: digest(graph),
    artifact_sha256: createHash("sha256").update(content).digest("hex"),
    renderer: { name: "mermaid", version: "12.3.0", offline: true },
    derived: true,
  };
  writeOutput(path, content, workspace, replace);
  writeOutput(
    manifestPath,
    JSON.stringify(manifest, null, 2) + "\n",
    workspace,
    replace,
  );
  return { path, manifest: manifestPath, ...manifest };
}
