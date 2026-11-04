// Example boundary for a Workbench: consume derived views, retain native identity.
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { html, mermaid } from "../src/visual.ts";
const [input, output] = process.argv.slice(2);
if (!input || !output)
  throw Error("usage: bun consume-view.ts GRAPH.json NEW-PREVIEW.html");
const bytes = readFileSync(input),
  graph = JSON.parse(bytes.toString("utf8"));
if (graph.format !== "knowledgehub-view/1")
  throw Error("unsupported view format");
if (
  !graph.selection ||
  !graph.publication_anchor ||
  !graph.nodes.every(
    (n: any) => n.reference && (n.available === false || n.source_bindings),
  )
)
  throw Error("missing exact input identities");
writeFileSync(output, html(graph), { flag: "wx" });
console.log(
  JSON.stringify({
    input_sha256: createHash("sha256").update(bytes).digest("hex"),
    selection: graph.selection,
    publication_anchor: graph.publication_anchor,
    coverage: graph.coverage,
    mermaid: mermaid(graph),
    preview: output,
    meaning:
      "derived inspection only; no automatic learner-artifact or native-model import",
  }),
);
