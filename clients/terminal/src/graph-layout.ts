import dagre from "@dagrejs/dagre";
import glyphs from "../assets/graphics/glyph-ranges.json";
import { Failure, clean, key, type Json } from "./types.ts";

export const limits = Object.freeze({
  vertices: 200,
  edges: 500,
  input: 2 * 1024 * 1024,
  svg: 2 * 1024 * 1024,
  dimension: 4096,
  pixels: 4194304,
  png: 8 * 1024 * 1024,
  frame: 12 * 1024 * 1024,
});
export type DisplayGraph = {
  nodes: { id: string; label: string; type: string; status: string }[];
  edges: { id: string; from: string; to: string; label: string }[];
};
export type Viewport = {
  width: number;
  height: number;
  x: number;
  y: number;
  zoom: number;
};
const fail = (message: string): never => {
  throw new Failure({ code: "RenderLimit", message });
};
const supports = (cp: number) => glyphs.some(([a, b]) => cp >= a && cp <= b);
export function displayText(value: unknown, max = 160): string {
  const original = String(value ?? "");
  let s = "";
  for (const c of clean(original.slice(0, 4096))) {
    const part = supports(c.codePointAt(0)!)
      ? c
      : `[U+${c.codePointAt(0)!.toString(16).toUpperCase()}]`;
    if (s.length + part.length > max - 1) return s + "…";
    s += part;
  }
  return original.length > 4096 ? s + "…" : s;
}
export const xml = (s: string) =>
  s.replace(
    /[&<>"']/g,
    (c) =>
      ({
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&apos;",
      })[c]!,
  );
/** No semantic projection here: only copy existing exact graph IDs/endpoints. */
export function presentation(graph: Json): DisplayGraph {
  if (graph.format !== "knowledgehub-view/1")
    fail("expected knowledgehub-view/1");
  if (graph.nodes.length > limits.vertices || graph.edges.length > limits.edges)
    fail("graph exceeds 200 vertices / 500 edges");
  const references = new Map<string, string>(
    graph.nodes.map((n: Json) => [key(n.reference), n.id]),
  );
  const nodes = graph.nodes
    .map((n: Json) => ({
      id: n.id,
      label: displayText(n.label || n.reference.id),
      type: displayText(n.type_ref?.id ?? "unrecorded type", 70),
      status: displayText(
        `${n.available === false ? "UNRESOLVED" : (n.membership ?? "recorded")} · review unrecorded`,
        85,
      ),
    }))
    .sort((a: Json, b: Json) => a.id.localeCompare(b.id));
  const edges = graph.edges
    .map((e: Json) => {
      const from = references.get(key(e.from)),
        to = references.get(key(e.to));
      if (!from || !to) fail("edge endpoint missing from exact graph");
      return {
        id: e.id,
        from,
        to,
        label: displayText(
          e.label ??
            e.selector?.path ??
            e.selector?.definition?.id ??
            e.category,
          70,
        ),
      };
    })
    .sort((a: Json, b: Json) => a.id.localeCompare(b.id));
  const result = { nodes, edges };
  checkDisplay(result);
  return result;
}
export function checkDisplay(graph: DisplayGraph) {
  if (
    !Array.isArray(graph.nodes) ||
    !Array.isArray(graph.edges) ||
    graph.nodes.length > limits.vertices ||
    graph.edges.length > limits.edges
  )
    fail("graph count limit");
  if (Buffer.byteLength(JSON.stringify(graph)) > limits.input)
    fail("display input exceeds 2 MiB");
  const ids = new Set<string>();
  for (const n of graph.nodes) {
    if (typeof n.id !== "string" || n.id.length > 256 || ids.has(n.id))
      fail("invalid/duplicate node identity");
    ids.add(n.id);
    if (
      [n.label, n.type, n.status].some(
        (v) => typeof v !== "string" || v.length > 512,
      )
    )
      fail("display label limit");
  }
  const edgeIds = new Set<string>();
  for (const e of graph.edges) {
    if (
      !ids.has(e.from) ||
      !ids.has(e.to) ||
      typeof e.id !== "string" ||
      e.id.length > 256 ||
      edgeIds.has(e.id)
    )
      fail("invalid/duplicate edge identity or endpoint");
    edgeIds.add(e.id);
    if (typeof e.label !== "string" || e.label.length > 512)
      fail("edge label limit");
  }
}
export function checkViewport(v: Viewport) {
  if (
    ![v.width, v.height].every(
      (n) => Number.isSafeInteger(n) && n > 0 && n <= limits.dimension,
    ) ||
    v.width * v.height > limits.pixels
  )
    fail("viewport exceeds 4096 dimensions / 4194304 pixels");
  if (
    ![v.x, v.y, v.zoom].every(Number.isFinite) ||
    Math.abs(v.x) > 1e7 ||
    Math.abs(v.y) > 1e7 ||
    v.zoom < 0 ||
    v.zoom > 8
  )
    fail("invalid viewport transform");
}
export function layout(graph: DisplayGraph) {
  checkDisplay(graph);
  const g = new dagre.graphlib.Graph({ directed: true, multigraph: true });
  g.setGraph({
    rankdir: "LR",
    nodesep: 32,
    ranksep: 100,
    edgesep: 28,
    marginx: 24,
    marginy: 24,
  });
  g.setDefaultEdgeLabel(() => ({}));
  for (const n of [...graph.nodes].sort((a, b) => a.id.localeCompare(b.id)))
    g.setNode(n.id, { width: 350, height: 140 });
  for (const e of [...graph.edges].sort((a, b) => a.id.localeCompare(b.id)))
    g.setEdge(e.from, e.to, { width: 180, height: 24 }, e.id);
  dagre.layout(g);
  const bounds = g.graph();
  if (
    ![bounds.width, bounds.height].every(
      (n) => typeof n === "number" && Number.isFinite(n) && n >= 0 && n < 1e7,
    )
  )
    fail("layout geometry bound");
  return {
    width: Math.max(1, bounds.width!),
    height: Math.max(1, bounds.height!),
    nodes: graph.nodes.map((n) => ({ ...n, ...g.node(n.id) })),
    edges: graph.edges.map((e) => ({
      ...e,
      ...g.edge({ v: e.from, w: e.to, name: e.id }),
    })),
  };
}
export function svgViewport(
  graph: DisplayGraph,
  viewport: Viewport,
  selected?: string,
  center = false,
) {
  checkViewport(viewport);
  const geometry = layout(graph);
  const scale =
    viewport.zoom ||
    Math.min(
      1,
      viewport.width / geometry.width,
      viewport.height / geometry.height,
    );
  const target =
    geometry.nodes.find((n) => n.id === selected) ??
    geometry.edges.find((e) => e.id === selected);
  const w = viewport.width / scale,
    h = viewport.height / scale;
  const x = center && target ? target.x - w / 2 : viewport.x,
    y = center && target ? target.y - h / 2 : viewport.y;
  const intersects = (
    left: number,
    top: number,
    width: number,
    height: number,
  ) => left + width >= x && top + height >= y && left <= x + w && top <= y + h;
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${viewport.width}" height="${viewport.height}" viewBox="0 0 ${viewport.width} ${viewport.height}"><rect width="${viewport.width}" height="${viewport.height}" fill="#f4f7fa"/><defs><marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z" fill="#536d7d"/></marker></defs><g transform="translate(${-x * scale} ${-y * scale}) scale(${scale})" font-family="DejaVu Sans" font-size="14">`;
  for (const edge of geometry.edges) {
    if (!edge.points?.length) fail("edge route missing");
    const xs = edge.points.map((p: { x: number }) => p.x),
      ys = edge.points.map((p: { y: number }) => p.y);
    if (
      !intersects(
        Math.min(...xs) - 12,
        Math.min(...ys) - 12,
        Math.max(...xs) - Math.min(...xs) + 24,
        Math.max(...ys) - Math.min(...ys) + 24,
      )
    )
      continue;
    const points = edge.points
      .map((p: { x: number; y: number }) => {
        if (![p.x, p.y].every((n) => Number.isFinite(n) && Math.abs(n) < 1e7))
          fail("edge geometry bound");
        return `${p.x},${p.y}`;
      })
      .join(" ");
    svg += `<polyline points="${points}" fill="none" stroke="${edge.id === selected ? "#b13d00" : "#536d7d"}" stroke-width="${edge.id === selected ? 4 : 2}" marker-end="url(#arrow)"/><text x="${edge.x}" y="${edge.y}" text-anchor="middle" fill="#183143">${edge.id === selected ? "SELECTED · " : ""}${xml(displayText(edge.label, 26))}</text>`;
  }
  for (const node of geometry.nodes) {
    const left = node.x - 175,
      top = node.y - 70;
    if (!intersects(left - 4, top - 4, 358, 148)) continue;
    svg += `<rect x="${left}" y="${top}" width="350" height="140" rx="10" fill="${node.id === selected ? "#fff1d8" : "white"}" stroke="${node.id === selected ? "#b13d00" : "#536d7d"}" stroke-width="${node.id === selected ? 4 : 1}"/>`;
    let text = node.label;
    if (text.length > 126) text = text.slice(0, 125) + "…";
    for (let i = 0; i < 3; i++)
      svg += `<text x="${left + 12}" y="${top + 25 + i * 19}" fill="#183143">${xml(text.slice(i * 42, (i + 1) * 42))}</text>`;
    svg += `<text x="${left + 12}" y="${top + 94}" font-size="11" fill="#3c596c">${xml(displayText(node.type, 52))}</text><text x="${left + 12}" y="${top + 114}" font-size="10" fill="#3c596c">${xml(displayText(node.status, 58))}</text><text x="${left + 12}" y="${top + 132}" font-size="10" fill="#b13d00">${node.id === selected ? "SELECTED · " : ""}${xml(node.id)}</text>`;
  }
  svg += "</g></svg>";
  if (Buffer.byteLength(svg) > limits.svg) fail("SVG exceeds 2 MiB");
  return {
    svg,
    scale,
    offset: { x, y },
    geometry: { width: geometry.width, height: geometry.height },
  };
}
