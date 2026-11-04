import { openExplorer } from "../src/explorer.ts";
const e = await openExplorer(process.argv[2], {
  model: process.argv[3] ?? "fruittrucks",
});
try {
  const publications = await e.history(),
    original = await e.publication("authoring:model"),
    affected = await e.history("authoring:model");
  const baselines = await e.baselines();
  console.log(
    JSON.stringify(
      e.envelope(
        { example: "local TypeScript exploration" },
        { info: e.info, publications, original, affected, baselines },
      ),
      null,
      2,
    ),
  );
} finally {
  await e.close();
}
