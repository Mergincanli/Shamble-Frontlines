// The content kinds' JSON Schemas (written by crates/content/src/schema.rs, committed in generated/content/schema),
// read the way an editor's field will read them (ADR 0005 CD-3: "minimum and maximum become slider limits"):
// PM-0005-7's schema half, "flagged by a schema-driven limit test". The limits reader is this test's own: no product
// code reads a schema yet (the camera panel, its first reader, is a later change-set's). Plain JavaScript, so that it
// reads the files with Node's own module: the types package has no Node types.
import { readFileSync } from "node:fs";
import { expect, test } from "vitest";

const folder = new URL("./generated/content/schema/", import.meta.url);
const texts = Object.fromEntries(["map", "movement"].map((kind) => [kind, readFileSync(new URL(`${kind}.schema.json`, folder), "utf8")]));
const schemas = Object.fromEntries(Object.entries(texts).map(([kind, text]) => [kind, JSON.parse(text)]));

/** The value with every object's keys in order, as the canonical writer writes them. */
function sorted(value) {
  if (Array.isArray(value)) return value.map(sorted);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sorted(value[key])]));
  return value;
}

/** A number schema's two ends, each with whether it is open (`exclusive...`) or closed. */
function limits(schema) {
  const low = "minimum" in schema ? { at: schema.minimum, open: false } : { at: schema.exclusiveMinimum, open: true };
  const high = "maximum" in schema ? { at: schema.maximum, open: false } : { at: schema.exclusiveMaximum, open: true };
  return { low, high };
}

/** Whether a value lies outside the limits: what a field built from the schema flags. */
const flagged = ({ low, high }, value) => (low.open ? value <= low.at : value < low.at) || (high.open ? value >= high.at : value > high.at);

/** Every number schema in a schema, by its path: in properties, in `items`, and in `prefixItems` one by one. */
function numbers(node, path, found = []) {
  if (Array.isArray(node)) node.forEach((item, at) => numbers(item, `${path}[${at}]`, found));
  else if (node !== null && typeof node === "object") {
    if (node.type === "number") found.push([path, node]);
    for (const [key, value] of Object.entries(node)) numbers(value, `${path}.${key}`, found);
  }
  return found;
}

const bits = new DataView(new ArrayBuffer(8));
/** The next double above (`up`) or below a value. */
function next(value, up) {
  if (value === 0) return up ? Number.MIN_VALUE : -Number.MIN_VALUE;
  bits.setFloat64(0, value);
  const word = bits.getBigUint64(0);
  bits.setBigUint64(0, value > 0 === up ? word + 1n : word - 1n);
  return bits.getFloat64(0);
}

test("each schema file is the canonical writer's text: JSON.stringify of the sorted value, and a line feed", () => {
  for (const [kind, text] of Object.entries(texts)) {
    expect(`${JSON.stringify(sorted(JSON.parse(text)), null, 2)}\n`, kind).toBe(text);
    expect(schemas[kind].$schema).toBe("https://json-schema.org/draft/2020-12/schema");
  }
});

test("at every end of every number range of both schemas, a field flags what the loader refuses", () => {
  const ranges = [...numbers(schemas.movement, "movement"), ...numbers(schemas.map, "map")];
  for (const [path, schema] of ranges) {
    const { low, high } = limits(schema);
    expect([typeof low.at, typeof high.at], path).toEqual(["number", "number"]);
    const ends = limits(schema);
    // A closed end is taken and the next double outside it flagged; an open end is flagged and the next double
    // inside it taken (the Rust half holds the loader to the same, at the same numbers).
    if (low.open) expect([flagged(ends, low.at), flagged(ends, next(low.at, true))], `${path} low`).toEqual([true, false]);
    else expect([flagged(ends, low.at), flagged(ends, next(low.at, false))], `${path} low`).toEqual([false, true]);
    if (high.open) expect([flagged(ends, high.at), flagged(ends, next(high.at, false))], `${path} high`).toEqual([true, false]);
    else expect([flagged(ends, high.at), flagged(ends, next(high.at, true))], `${path} high`).toEqual([false, true]);
  }
  // Every form is walked: the movement table's open low ends and the facing's open top, the one of its kind.
  const open = (side) => ranges.filter(([, schema]) => `exclusive${side}` in schema).map(([path]) => path);
  expect(open("Minimum").length).toBeGreaterThan(0);
  expect(open("Maximum")).toEqual(["map.properties.spawns.items.properties.facing_deg"]);
  // The bounds' corners carry the aim's reach, axis by axis.
  const axes = (side) => schemas.map.properties.bounds.properties[side].prefixItems.map((axis) => limits(axis));
  expect(axes("min").map(({ low }) => low.at)).toEqual([-327.67, 0, -327.67]);
  expect(axes("max").map(({ high }) => high.at)).toEqual([327.67, 655.35, 327.67]);
  expect(flagged(axes("min")[1], -0.01)).toBe(true);
  expect(flagged(axes("max")[1], 655.36)).toBe(true);
});

test("the id rule, defined once, takes what the loader takes and flags what it refuses", () => {
  for (const schema of Object.values(schemas)) {
    const rule = schema.$defs.id;
    const taken = (id) => id.length <= rule.maxLength && new RegExp(rule.pattern).test(id) && !new RegExp(rule.not.pattern).test(id);
    for (const id of ["test-scene", "a", "0", "a_b-c", "x".repeat(64), "con1", "com10", "lpt"]) expect(taken(id), id).toBe(true);
    for (const id of ["", "Test", "-a", "_a", "a.b", "a b", "x".repeat(65), "con", "prn", "aux", "nul", "com0", "com9", "lpt5"]) {
      expect(taken(id), id).toBe(false);
    }
  }
  // Every id of both kinds is that rule.
  const ids = (node, found = []) => {
    if (node !== null && typeof node === "object") {
      if (node.properties?.id !== undefined) found.push(node.properties.id.$ref);
      for (const value of Object.values(node)) ids(value, found);
    }
    return found;
  };
  expect(ids(schemas.movement)).toEqual(["#/$defs/id"]);
  // The map's own, a spawn's, a shape's in each of its two forms, and a region's.
  expect(ids(schemas.map)).toEqual(Array(5).fill("#/$defs/id"));
});

test("every object is closed, and a shape is a box or a ramp with all its own keys", () => {
  const objects = (node, path, found = []) => {
    if (node !== null && typeof node === "object" && !Array.isArray(node)) {
      if (node.type === "object") found.push([path, node]);
      for (const [key, value] of Object.entries(node)) objects(value, `${path}.${key}`, found);
    } else if (Array.isArray(node)) node.forEach((item, at) => objects(item, `${path}[${at}]`, found));
    return found;
  };
  for (const [path, node] of [...objects(schemas.movement, "movement"), ...objects(schemas.map, "map")]) {
    expect(node.additionalProperties, path).toEqual({ not: {} });
    expect([...node.required].sort(), path).toEqual(Object.keys(node.properties).sort());
  }
  const forms = schemas.map.properties.shapes.items.oneOf;
  expect(forms.map((form) => form.properties.type.const)).toEqual(["box", "ramp"]);
  expect(Object.keys(forms[1].properties).sort()).toEqual(["id", "max", "min", "rises_toward", "type"]);
});
