import mermaid from "mermaid";
async function run() {
  const { graph, mermaid: source } = JSON.parse(
    document.getElementById("view-data")!.textContent!,
  );
  document.getElementById("banner")!.textContent =
    `${graph.model.alias} · ${JSON.stringify(graph.selection)} · publication anchor ${graph.publication_anchor}`;
  document.getElementById("coverage")!.textContent =
    `${graph.coverage.complete ? "COMPLETE requested scope" : "PARTIAL"} · ${graph.coverage.vertices} vertices / ${graph.coverage.edges} edges · ${graph.coverage.warnings.join("; ")}`;
  mermaid.initialize({
    startOnLoad: false,
    securityLevel: "strict",
    htmlLabels: false,
    fontFamily: "system-ui",
    theme: "neutral",
    flowchart: { htmlLabels: false, useMaxWidth: false, wrappingWidth: 240 },
  });
  const { svg } = await mermaid.render("native-view", source),
    drawing = document.getElementById("drawing")!;
  drawing.innerHTML = svg;
  const inspect = (data: unknown) => {
    document.getElementById("inspector")!.textContent = JSON.stringify(
      data,
      null,
      2,
    );
  };
  for (const n of graph.nodes) {
    const element = drawing.querySelector(`[id*="flowchart-${n.id}-"]`);
    element?.setAttribute("tabindex", "0");
    element?.addEventListener("click", () => inspect(n));
    element?.addEventListener("keydown", (e) => {
      if ((e as KeyboardEvent).key === "Enter") inspect(n);
    });
  }
  for (const e of graph.edges) {
    const element = drawing.querySelector(`[id$="-${e.id}"]`);
    element?.setAttribute("tabindex", "0");
    element?.addEventListener("click", () => inspect(e));
    element?.addEventListener("keydown", (event) => {
      if ((event as KeyboardEvent).key === "Enter") inspect(e);
    });
  }
  let scale = 1,
    x = 0,
    y = 0;
  const apply = () => {
    drawing.style.transform = `translate(${x}px,${y}px) scale(${scale})`;
  };
  document.getElementById("zoom-in")!.onclick = () => {
    scale *= 1.2;
    apply();
  };
  document.getElementById("zoom-out")!.onclick = () => {
    scale /= 1.2;
    apply();
  };
  document.getElementById("reset")!.onclick = () => {
    scale = 1;
    x = y = 0;
    apply();
  };
  const viewport = document.getElementById("viewport")!;
  let drag: { x: number; y: number } | undefined;
  viewport.onpointerdown = (e) => {
    if ((e.target as Element).closest(".node,.edgePaths")) return;
    drag = { x: e.clientX - x, y: e.clientY - y };
    viewport.setPointerCapture(e.pointerId);
  };
  viewport.onpointermove = (e) => {
    if (drag) {
      x = e.clientX - drag.x;
      y = e.clientY - drag.y;
      apply();
    }
  };
  viewport.onpointerup = () => {
    drag = undefined;
  };
  document.documentElement.dataset.rendered = "true";
}
run().catch((e) => {
  document.getElementById("error")!.textContent = String(e);
  document.documentElement.dataset.rendered = "error";
});
