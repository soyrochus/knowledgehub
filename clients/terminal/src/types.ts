export type Ref = { model: string; id: string; revision: string };
export type Selection =
  | { selection: "Snapshot"; reference: string }
  | { selection: "Baseline"; reference: Ref };
// Native DTOs retain tagged authored values and decimal-string counters. These
// JSON types deliberately do not reinterpret a numeric-looking authored Text.
export type Json = any;
export const key = (r: Ref) => JSON.stringify([r.model, r.id, r.revision]);
export const exact = (name: string): Ref => ({
  model: "sdo",
  id: `sdo:type:${name}`,
  revision: "catalogue-3",
});
export const ontology: Ref = {
  model: "sdo",
  id: "sdo:ontology:system-definition",
  revision: "catalogue-3",
};
export const clean = (s: unknown): string =>
  String(s ?? "").replace(
    /[\x00-\x1f\x7f-\x9f\u202a-\u202e\u2066-\u2069]/g,
    (c) => `\\u${c.charCodeAt(0).toString(16).padStart(4, "0")}`,
  );
export const label = (r: Json): string =>
  r?.name ??
  r?.data?.data?.properties?.name?.value ??
  r?.data?.data?.name ??
  r?.data?.data?.label ??
  r?.reference?.id ??
  "";
export class Failure extends Error {
  constructor(public detail: Json) {
    super(detail.message ?? detail.code);
  }
}
export function uint(v: string | undefined, def: number, max: number): number {
  if (v === undefined) return def;
  if (!/^(0|[1-9][0-9]*)$/.test(v))
    throw new Failure({
      code: "Usage",
      message: "canonical positive decimal limit required",
    });
  const n = BigInt(v);
  if (n < 1n || n > BigInt(max))
    throw new Failure({
      code: "Usage",
      message: `limit must be 1..${max}; values are never clamped`,
    });
  return Number(n);
}
export const preparation = {
  allow_rebuild: true,
  owner_limit: "10000",
  fact_limit: "100000",
  byte_limit: "67108864",
};
export const readBudgets = {
  hops: "8",
  starts: "100",
  states: "2000",
  examined: "5000",
  candidates: "2000",
  targets: "1000",
  witnesses: "1",
  page_size: "100",
  bytes: "262144",
  preparation,
};
export const historyDefaults = {
  items: "100",
  nodes: "256",
  storage_bytes: "16777216",
  output_bytes: "262144",
};
export const historyMethods = [
  "publication",
  "publications",
  "publication_affected",
  "continue_history",
  "release_history",
];
export function exitFor(e: Json): number {
  return ["Usage", "InvalidInput"].includes(e.code)
    ? 2
    : ["HistoryLimit", "Capacity", "Busy", "IndexNotReady", "Partial"].includes(
          e.code,
        )
      ? 3
      : e.code === "Denied"
        ? 4
        : ["Ambiguous", "NotFound"].includes(e.code)
          ? 5
          : e.code === "Cancelled"
            ? 130
            : 6;
}
