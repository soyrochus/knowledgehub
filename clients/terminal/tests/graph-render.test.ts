import { expect, test } from "bun:test";
import {
  presentation,
  layout,
  svgViewport,
  displayText,
  checkViewport,
} from "../src/graph-layout.ts";
import { GraphRenderer } from "../src/graph-render.ts";
import { Failure } from "../src/types.ts";
const ref = (id: string) => ({ model: "m", id, revision: "1" });
export function syntheticGraph() {
  return {
    format: "knowledgehub-view/1",
    nodes: ["a", "b"].map((id) => ({
      id,
      reference: ref(id),
      label: "duplicate <script>\x1b[31m 🦄",
      type_ref: ref("Type"),
      membership: "supporting",
      available: id !== "b",
    })),
    edges: [
      {
        id: "e1",
        from: ref("a"),
        to: ref("b"),
        origin: { assertion: ref("assert1") },
        selector: { path: "/satisfies" },
      },
      {
        id: "e2",
        from: ref("a"),
        to: ref("b"),
        origin: { assertion: ref("assert2") },
        selector: { path: "/satisfies" },
      },
      {
        id: "e3",
        from: ref("b"),
        to: ref("a"),
        origin: { owner: ref("b"), occurrence: "/p/0" },
        selector: { path: "/p/*" },
      },
      {
        id: "e4",
        from: ref("a"),
        to: ref("a"),
        origin: { owner: ref("a"), occurrence: "/p/1" },
        selector: { path: "/p/*" },
      },
    ],
    coverage: { complete: false, warnings: ["unresolved; proposed"] },
  };
}
test("wide ordered graphs rasterize clipped pan/zoom viewports without allocating the full graph", async () => {
  const nodes = Array.from({ length: 7 }, (_, i) => ({
    id: `n${i}`,
    label: `Activity ${i}`,
    type: "JourneyStep",
    status: "declared order",
  }));
  const edges = nodes.slice(1).map((n, i) => ({
    id: `e${i}`,
    from: nodes[i].id,
    to: n.id,
    label: "recorded list order",
  }));
  const r = new GraphRenderer();
  try {
    for (const [x, zoom] of [
      [0, 1],
      [-200, 1],
      [2000, 2],
      [0, 0],
    ]) {
      const f = await r.render({
        display: { nodes, edges },
        viewport: { width: 816, height: 675, x, y: 0, zoom },
        selected: "n6",
        center: zoom === 1,
      });
      expect(f.width).toBe(816);
      expect(f.height).toBe(675);
      expect(f.png.length).toBeGreaterThan(100);
    }
    expect(edges).toHaveLength(6);
  } finally {
    r.close();
  }
});
test("layout preserves exact multigraph occurrences, cycles, self edges and duplicate labels", () => {
  const graph = syntheticGraph(),
    before = JSON.stringify(graph),
    display = presentation(graph),
    result = layout(display);
  expect(result.nodes.map((n) => n.id)).toEqual(["a", "b"]);
  expect(result.edges.map((e) => e.id)).toEqual(["e1", "e2", "e3", "e4"]);
  expect(result.edges.every((e) => e.points.length >= 2)).toBe(true);
  expect(result.edges[0].points).not.toEqual(result.edges[1].points);
  expect(JSON.stringify(graph)).toBe(before);
  expect(display.nodes[1].status).toContain("UNRESOLVED");
});
test("only generated SVG primitives survive hostile text; missing glyphs and truncation are visible", () => {
  const graph = presentation(syntheticGraph());
  const { svg } = svgViewport(
    graph,
    { width: 800, height: 500, x: 0, y: 0, zoom: 0 },
    "e2",
  );
  expect(svg).not.toContain("<script>");
  expect(svg).toContain("&lt;script&gt;");
  expect(svg).not.toContain("\x1b");
  expect(svg).not.toMatch(/<(image|foreignObject|script)|href=/);
  expect(svg).toContain("SELECTED");
  expect(displayText("🦄")).toBe("[U+1F984]");
  expect(displayText("a".repeat(10000))).toEndWith("…");
  expect(() =>
    checkViewport({ width: 4096, height: 4096, x: 0, y: 0, zoom: 1 }),
  ).toThrow();
  expect(() =>
    presentation({
      ...syntheticGraph(),
      nodes: Array(201).fill(syntheticGraph().nodes[0]),
    }),
  ).toThrow();
});
test("real worker rasterizes with bundled assets, coalesces intents, rejects obsolete work and cancels", async () => {
  const r = new GraphRenderer();
  try {
    const input = {
      display: presentation(syntheticGraph()),
      viewport: { width: 800, height: 500, x: 0, y: 0, zoom: 0 },
      selected: "a",
    };
    const first = r.render(input).then(
      () => "shown",
      () => "obsolete",
    );
    await Bun.sleep(1);
    const middle = r.render(input).then(
      () => "shown",
      () => "obsolete",
    );
    const last = r.render(input);
    expect(await first).toBe("obsolete");
    expect(await middle).toBe("obsolete");
    const frame = await last;
    expect(frame.width).toBe(800);
    expect(frame.png.length).toBeGreaterThan(100);
    const rejected = r.render(input).catch((e) => e as Failure);
    r.cancel();
    expect(((await rejected) as Failure).detail.code).toBe("Cancelled");
    await expect(
      r.render({
        display: input.display,
        viewport: { ...input.viewport, width: 100000 },
      }),
    ).rejects.toBeInstanceOf(Failure);
    await expect(
      r.render({ padding: "a".repeat(2 * 1024 * 1024) }),
    ).rejects.toBeInstanceOf(Failure);
  } finally {
    r.close();
  }
  const slow = new GraphRenderer(1);
  try {
    await expect(slow.render({ checkpoint: true })).rejects.toBeInstanceOf(
      Failure,
    );
  } finally {
    slow.close();
  }
});
