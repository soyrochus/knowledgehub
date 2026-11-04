import { Explorer } from "./explorer.ts";
import { Failure, exact, type Ref, type Json } from "./types.ts";
import { extract, exportView } from "./visual.ts";
import { writeOutput } from "./output.ts";
export type Command = {
  name: string;
  positionals: string[];
  flags: Record<string, string | boolean>;
};
export const ref = (s: string): Ref => {
  try {
    const v = JSON.parse(s);
    if (
      Object.keys(v).sort().join(",") !== "id,model,revision" ||
      !Object.values(v).every((x) => typeof x === "string" && x.length)
    )
      throw 0;
    return v;
  } catch {
    throw new Failure({
      code: "Usage",
      message:
        "exact reference is JSON {model,id,revision}, all nonempty strings",
    });
  }
};
export async function execute(e: Explorer, c: Command): Promise<Json> {
  const { name, positionals: p, flags: f } = c;
  const arg = (i = 0) => {
    if (!p[i])
      throw new Failure({
        code: "Usage",
        message: `${name}: argument ${i + 1} required; kh ${name} --help`,
      });
    return p[i];
  };
  switch (name) {
    case "info":
      return {
        ...e.info,
        selection: e.selection,
        publication_anchor: e.anchor,
        capabilities: e.transport.capabilities,
      };
    case "types":
      return { types: await e.types() };
    case "type":
      return e.schema(arg());
    case "list":
      return {
        items: await e.list(
          p[0] ? await e.resolve(p[0], true) : undefined,
          !!f.subtypes,
        ),
      };
    case "show":
      return e.show(arg());
    case "links":
      return e.links(arg(), linkOptions(f));
    case "tree":
      return e.tree(arg(), Number(f.depth ?? 2), linkOptions(f));
    case "views":
      return e.groups(String(f.view ?? "type"));
    case "search":
      return e.search(
        arg(),
        String(f.type ?? "Requirement"),
        String(f.field ?? "/name"),
        String(f.mode ?? "literal-utf8/1"),
      );
    case "source":
      if (
        (f.export &&
          (f.preview || f.offset !== undefined || f.length !== undefined)) ||
        (!f.preview && (f.offset !== undefined || f.length !== undefined)) ||
        (f.replace && !f.export)
      )
        throw new Failure({
          code: "Usage",
          message:
            "source preview/range and original export are separate modes; --replace requires --export",
        });
      if (f.export) {
        const x = await e.original(arg());
        return {
          reference: x.reference,
          original: x.original,
          path: writeOutput(
            String(f.export),
            x.bytes,
            e.transport.workspace,
            !!f.replace,
          ),
          full_content_verified: true,
        };
      }
      return f.preview
        ? e.preview(arg(), String(f.offset ?? "0"), String(f.length ?? "65536"))
        : e.sources(arg());
    case "history":
      return e.lineage(arg());
    case "publications":
      return e.history();
    case "publication": {
      const original = await e.publication(arg());
      return f.affected
        ? { publication: original, affected: await e.history(arg()) }
        : original;
    }
    case "baselines":
      return e.baselines(!!f.retained);
    case "compare":
      return e.compare(ref(arg()), ref(arg(1)));
    case "gaps":
      return e.gaps();
    case "diagram": {
      if (!f.output)
        throw new Failure({
          code: "Usage",
          message: "diagram requires --output PATH",
        });
      const graph = await extract(
        e,
        arg(),
        String(f.projection ?? "relationships"),
        Number(f.depth ?? 1),
        Number(f.vertices ?? 200),
        Number(f.edges ?? 500),
      );
      return exportView(
        graph,
        String(f.format ?? "html"),
        String(f.output),
        e.transport.workspace,
        !!f.replace,
      );
    }
    default:
      throw new Failure({
        code: "Usage",
        message: `unknown command ${name}; kh --help`,
      });
  }
}
export function linkOptions(f: Record<string, string | boolean>): Json {
  const out: Json = {};
  if (f.direction) {
    const directions: Record<string, string> = {
      incoming: "Incoming",
      outgoing: "Outgoing",
      both: "Both",
    };
    if (!directions[String(f.direction)])
      throw new Failure({
        code: "Usage",
        message: "direction is incoming, outgoing or both",
      });
    out.direction = directions[String(f.direction)];
  }
  if (f.category) {
    const allowed = [
      "Association",
      "Property",
      "Representation",
      "Provenance",
      "Administrative",
    ];
    const categories = String(f.category).split(",");
    if (categories.some((c) => !allowed.includes(c)))
      throw new Failure({
        code: "Usage",
        message: `category is a comma-separated subset of ${allowed}`,
      });
    out.categories = categories;
  }
  if (f.selector) {
    try {
      out.selectors = JSON.parse(String(f.selector));
    } catch {
      throw new Failure({
        code: "Usage",
        message: "selector is a JSON array of exact native selectors",
      });
    }
  }
  if (f.closure) out.fact_scope = "Closure";
  return out;
}
