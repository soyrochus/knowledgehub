// Maintainer-only: fresh disposable copies, ordinary native API, no native file decoding.
import { openExplorer } from "../src/explorer.ts";
import { extract, digest } from "../src/visual.ts";
import { presentation } from "../src/graph-layout.ts";
import { GraphRenderer } from "../src/graph-render.ts";
import { mkdirSync } from "node:fs";
import { resolve, join } from "node:path";
const [workspace, destination] = process.argv.slice(2);
if (!workspace || !destination)
  throw Error("usage: bun verify-graph-data.ts WORKSPACE NEW-DESTINATION");
mkdirSync(destination, { recursive: false });
const cases = [];
for (const [model, root, projection] of [
  ["fruittrucks", "model:requirement:ack", "relationships"],
  ["fruittrucks", "model:journey:change", "journey"],
  ["fruittrucks", "model:lifecycle:offer", "lifecycle"],
  ["equipment", "model:entity:Booking", "relationships"],
]) {
  const e = await openExplorer(resolve(workspace), { model, bytes: 1048576 });
  const renderer = new GraphRenderer();
  try {
    const graph = await extract(e, root, projection, 1, 60, 120),
      before = digest(graph),
      display = presentation(graph);
    const equivalent = await e
      .fork({
        snapshot:
          e.selection.selection === "Snapshot"
            ? e.selection.reference
            : undefined,
        historyAnchor: e.anchor,
      })
      .open();
    try {
      const json = await extract(equivalent, root, projection, 1, 60, 120);
      if (digest(json) !== before)
        throw Error("ordinary JSON diagram extraction changed");
    } finally {
      await equivalent.release();
    }
    const frame = await renderer.render({
      display,
      viewport: { width: 1400, height: 850, x: 0, y: 0, zoom: 0 },
      selected: display.nodes[0]?.id,
    });
    if (digest(graph) !== before) throw Error("presentation mutated exact DTO");
    if (
      display.edges.length !== graph.edges.length ||
      display.nodes.length !== graph.nodes.length
    )
      throw Error("semantic identities dropped");
    const name = model + "-" + projection;
    await Bun.write(
      join(destination, name + ".json"),
      JSON.stringify(graph, null, 2) + "\n",
    );
    await Bun.write(join(destination, name + ".png"), frame.png);
    cases.push({
      model,
      root,
      projection,
      selection: graph.selection,
      publication_anchor: graph.publication_anchor,
      coverage: graph.coverage,
      graph_sha256: before,
      png_bytes: frame.png.length,
      nodes: display.nodes.map((n) => n.id),
      edges: display.edges.map((e) => e.id),
    });
  } finally {
    renderer.close();
    await e.close();
  }
}
await Bun.write(
  join(destination, "graph-data.json"),
  JSON.stringify(
    {
      status: "pass",
      ordinary_json_equivalent: true,
      viewer_dto_unchanged: true,
      cases,
    },
    null,
    2,
  ) + "\n",
);
console.log(
  JSON.stringify({ status: "pass", cases: cases.length, destination }),
);
