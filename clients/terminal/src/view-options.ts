import { Failure, uint } from "./types.ts";
export type ViewOptions = {
  projection: string;
  depth: number;
  vertices: number;
  edges: number;
};
export function viewOptions(
  flags: Record<string, string | boolean>,
): ViewOptions {
  const projection = String(flags.projection ?? "relationships");
  if (!["relationships", "journey", "lifecycle"].includes(projection))
    throw new Failure({
      code: "Usage",
      message: "projection is relationships, journey or lifecycle",
    });
  const n = (key: string, def: number, max: number) =>
    uint(flags[key] === undefined ? undefined : String(flags[key]), def, max);
  return {
    projection,
    depth: n("depth", 1, 8),
    vertices: n("vertices", 60, 200),
    edges: n("edges", 120, 500),
  };
}
