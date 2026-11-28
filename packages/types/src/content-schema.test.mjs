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

// The ends a field built from the schema flags; that the loader refuses the same values at the same numbers is the
// Rust test's (crates/content/src/schema.rs, at_each_end_the_loader_does_what_the_schema_says).
test("at every end of every number range of both schemas, a field built from the schema flags the value past the end", () => {
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

/** A generated type's declaration (generated/content/<name>.ts), as its text stands. */
const declaration = (name) => {
  const text = readFileSync(new URL(`./generated/content/${name}.ts`, import.meta.url), "utf8");
  const line = text.split("\n").find((entry) => entry.startsWith(`export type ${name} = `));
  if (line === undefined) throw new Error(`${name}.ts declares no type ${name}`);
  return line.slice(`export type ${name} = `.length).replace(/;$/, "");
};
/** An object type's keys in order, and each key's type, from a declaration's `{ key: type, ... }`. */
const members = (body) =>
  Object.fromEntries(
    [...body.matchAll(/(?:^|[{,] )"?([a-z][a-z0-9_]*)"?: ((?:\[[^\]]*\])|[^,]+)/g)].map(([, key, type]) => [key, type.trim()]),
  );
/** A union of words' members, from a declaration's `"a" | "b"`. */
const words = (body) => [...body.matchAll(/"([^"]+)"/g)].map(([, word]) => word);
/** A schema's own keys, but for the header's two, which the types leave out (they say what a file holds beside them). */
const keys = (schema) => Object.keys(schema.properties).filter((key) => key !== "format" && key !== "version");

// The TypeScript types (written by ts-rs from the Rust structs) and the schemas (written from the reader's tables)
// come from two sources in the content crate: held here to name the same keys, lengths and words, so the two cannot
// drift apart (engine-architect's round of 2026-11-27; ADR 0005 CD-3: both "from the same Rust types").
test("the generated types and the schemas name the same keys, the same corner lengths and the same words", () => {
  const map = schemas.map.properties;
  const pairs = [
    ["Movement", schemas.movement],
    ["MapDocument", schemas.map],
    ["Bounds", map.bounds],
    ["Spawn", map.spawns.items],
    ["Region", map.regions.items],
  ];
  // A corner is a tuple of as many numbers as the schema's fixed length. Each member's kind is compared too, below.
  const corners = (typed, schema, name) => {
    for (const [key, type] of Object.entries(typed)) {
      if (!type.startsWith("[")) continue;
      const length = type.split(",").length;
      const fixed = schema.properties[key];
      expect([fixed.minItems ?? fixed.prefixItems?.length, fixed.maxItems], `${name}.${key}`).toEqual([length, length]);
    }
  };
  for (const [name, schema] of pairs) {
    const typed = members(declaration(name));
    expect(Object.keys(typed).sort(), name).toEqual(keys(schema).sort());
    corners(typed, schema, name);
  }
  // A shape: its own keys, then each form's (a box's type alone, a ramp's type and its side).
  const shape = declaration("Shape");
  const [own, forms] = shape.split(" & ");
  const variants = forms.replace(/^\(|\)$/g, "").split(" | ").map((form) => members(form));
  const schemaForms = map.shapes.items.oneOf;
  expect(variants).toHaveLength(schemaForms.length);
  variants.forEach((variant, at) => {
    expect([...Object.keys(members(own)), ...Object.keys(variant)].sort(), `Shape form ${at}`).toEqual(keys(schemaForms[at]).sort());
    expect(words(variant.type ?? ""), `Shape form ${at}`).toEqual([schemaForms[at].properties.type.const]);
    corners(members(own), schemaForms[at], `Shape form ${at}`);
  });
  // The words: a ramp's side and a region's ground.
  expect(words(declaration("Toward"))).toEqual(schemaForms[1].properties.rises_toward.enum);
  expect(words(declaration("Ground"))).toEqual(map.regions.items.properties.kind.enum);
});

/** What a schema's node says a member is (the id rule is a string by its own definition). */
function schemaKind(node, root) {
  if (node.$ref === "#/$defs/id") return root.$defs?.id?.type === "string" ? "string" : "unknown";
  if ("const" in node) return "word";
  if (Array.isArray(node.enum)) return "words";
  if (node.type === "number") return "number";
  if (node.type === "string") return "string";
  if (node.type === "array" && (Array.isArray(node.prefixItems) ? node.prefixItems.every((item) => item.type === "number") : node.items?.type === "number")) return "tuple";
  if (node.type === "array") return "list";
  if (node.type === "object") return "object";
  return "unknown";
}
/** What a generated type's member is, from its TypeScript type's text (a form's last member keeps the closing brace). */
function typeKind(text) {
  const type = text.replace(/\s*\}$/, "");
  if (type === "number" || type === "string") return type;
  if (/^"[^"]+"$/.test(type)) return "word";
  if (type.startsWith("[")) return type.slice(1, -1).split(",").every((part) => part.trim() === "number") ? "tuple" : "unknown";
  if (type.startsWith("Array<")) return "list";
  if (type === "Toward" || type === "Ground") return "words";
  if (type === "Bounds") return "object";
  return "unknown";
}
/** Each member whose kind in the generated type is not its kind in the schema, in words. */
const kindProblems = (typed, schema, root) =>
  Object.entries(typed)
    .map(([key, type]) => [key, type, typeKind(type), schemaKind(schema.properties[key] ?? {}, root)])
    .filter(([, , typedKind, schemaKindOf]) => typedKind !== schemaKindOf || typedKind === "unknown")
    .map(([key, type, , schemaKindOf]) => `${key}: ${type} against ${schemaKindOf}`);

// Follow-up 4B-MEMBER-KINDS (engine-architect's round 2 of 4b-2): the page now reads these types (the map's reading,
// packages/client/src/app/content.ts), so each member's kind is held to the schema's: a number to a number, a string
// to a string or the id rule, a corner to an array of numbers, a word list to an `enum`, a list to an array.
test("each member of the generated types is of the kind its schema says", () => {
  const map = schemas.map.properties;
  const pairs = [
    ["Movement", schemas.movement, schemas.movement],
    ["MapDocument", schemas.map, schemas.map],
    ["Bounds", map.bounds, schemas.map],
    ["Spawn", map.spawns.items, schemas.map],
    ["Region", map.regions.items, schemas.map],
  ];
  for (const [name, schema, root] of pairs) expect(kindProblems(members(declaration(name)), schema, root), name).toEqual([]);
  const [own, forms] = declaration("Shape").split(" & ");
  forms
    .replace(/^\(|\)$/g, "")
    .split(" | ")
    .forEach((form, at) => {
      const typed = { ...members(own), ...members(form) };
      expect(kindProblems(typed, map.shapes.items.oneOf[at], schemas.map), `Shape form ${at}`).toEqual([]);
    });
  // The compare bites: a spawn whose facing were a string would be told apart from the schema's number.
  const planted = { ...members(declaration("Spawn")), facing_deg: "string" };
  expect(kindProblems(planted, map.spawns.items, schemas.map)).toEqual(["facing_deg: string against number"]);
});
