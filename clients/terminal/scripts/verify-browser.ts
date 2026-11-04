import { chromium } from "@playwright/test";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";
const file = resolve(process.argv[2]),
  out = resolve(process.argv[3]);
mkdirSync(out, { recursive: true });
const browser = await chromium.launch({
  headless: true,
  executablePath:
    process.env.CHROMIUM_PATH ??
    "/Users/ivanderk/Library/Caches/ms-playwright/chromium-1234/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
});
const page = await browser.newPage({ viewport: { width: 1440, height: 960 } }),
  network: string[] = [],
  errors: string[] = [];
await page.route("**/*", (route) => {
  if (!route.request().url().startsWith("file:")) {
    network.push(route.request().url());
    return route.abort();
  }
  return route.continue();
});
page.on("pageerror", (e) => errors.push(String(e)));
try {
  page.setDefaultTimeout(5000);
  await page.goto(pathToFileURL(file).href);
  await page.waitForFunction(
    () => document.documentElement.dataset.rendered !== undefined,
  );
  const state = await page.locator("html").getAttribute("data-rendered");
  if (state !== "true") throw Error(await page.locator("#error").innerText());
  const data = await page.locator("#view-data").textContent(),
    view = JSON.parse(data!),
    graph = view.graph;
  if ((await page.locator("svg .node").count()) !== graph.nodes.length)
    throw Error("node count differs from native DTO");
  const first = graph.nodes[0];
  await page.locator(`[id*="flowchart-${first.id}-"]`).click();
  const node = JSON.parse(await page.locator("#inspector").innerText());
  if (JSON.stringify(node.reference) !== JSON.stringify(first.reference))
    throw Error("node inspector identity mismatch");
  let edge: any;
  if (graph.edges.length) {
    const expected = graph.edges[0];
    await page.locator(`[id$="-${expected.id}"]`).focus();
    await page.locator(`[id$="-${expected.id}"]`).press("Enter");
    edge = JSON.parse(await page.locator("#inspector").innerText());
    if (JSON.stringify(edge.origin) !== JSON.stringify(expected.origin))
      throw Error("edge origin mismatch");
  }
  await page.locator("#zoom-in").click();
  const transform = await page
    .locator("#drawing")
    .evaluate((el) => (el as HTMLElement).style.transform);
  if (!transform.includes("1.2")) throw Error("zoom did not change transform");
  await page.locator("#reset").click();
  await page.screenshot({ path: join(out, "preview.png"), fullPage: true });
  const result = {
    file,
    network_blocked: true,
    network_attempts: network,
    browser_errors: errors,
    rendered: true,
    selection: graph.selection,
    publication_anchor: graph.publication_anchor,
    coverage: graph.coverage,
    node_inspected: node,
    edge_inspected: edge,
    zoom: transform,
  };
  if (network.length || errors.length)
    throw Error(JSON.stringify({ network, errors }));
  writeFileSync(
    join(out, "browser.json"),
    JSON.stringify(result, null, 2) + "\n",
  );
  console.log(
    JSON.stringify({
      rendered: true,
      nodes: graph.nodes.length,
      edges: graph.edges.length,
      network_attempts: network.length,
      evidence: out,
    }),
  );
} finally {
  await browser.close();
}
