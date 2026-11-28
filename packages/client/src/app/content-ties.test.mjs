// The page's copies of the game's loader's own words, read from the Rust source (engine-architect's round of
// 2026-11-27): the bundle's format and version the page checks after the Worker's `ready`, each kind's format and
// newest version, and the kinds' folders (content.ts). Plain JavaScript, so that it reads the source with Node's own
// module: the page's packages have no Node types.
import { readFileSync } from "node:fs";
import { expect, test } from "vitest";
import {
  BUNDLE_FORMAT,
  BUNDLE_VERSION,
  CONTENT_KINDS,
  MAP_FORMAT,
  MAP_FORMS,
  MAP_GROUNDS,
  MAP_TOWARDS,
  MAP_VERSION,
  MOVEMENT_FORMAT,
  MOVEMENT_VERSION,
} from "./content.ts";

/** A Rust file of the content crate. */
const rust = (file) => readFileSync(new URL(`../../../../crates/content/src/${file}`, import.meta.url), "utf8");

/** The `FORMAT` and `VERSION` constants of one Rust file, as the loader holds them. */
function formatOf(file) {
  const source = rust(file);
  const format = /^pub const FORMAT: &str = "([a-z-]+)";$/m.exec(source);
  const version = /^pub const VERSION: u32 = (\d+);$/m.exec(source);
  expect(format, `FORMAT in crates/content/src/${file}`).not.toBeNull();
  expect(version, `VERSION in crates/content/src/${file}`).not.toBeNull();
  return [format?.[1], Number(version?.[1])];
}

/** One of the map's word lists in map.rs (`pub(crate) const NAME: [&str; n] = [...];`), as the loader reads it. */
function wordsOf(name) {
  const found = new RegExp(`^pub\\(crate\\) const ${name}: \\[&str; \\d+\\] = \\[([^\\]]*)\\];$`, "m").exec(rust("map.rs"));
  expect(found, `${name} in crates/content/src/map.rs`).not.toBeNull();
  return [...(found?.[1] ?? "").matchAll(/"([^"]+)"/g)].map((match) => match[1]);
}

test("the map's fixed words the page reads are the loader's own, in its order (step S's review, EA-3)", () => {
  expect([...MAP_FORMS]).toEqual(wordsOf("FORMS"));
  expect([...MAP_TOWARDS]).toEqual(wordsOf("TOWARDS"));
  expect([...MAP_GROUNDS]).toEqual(wordsOf("GROUNDS"));
});

test("the bundle's format and version the page reads are the loader's own", () => {
  expect(formatOf("bundle.rs")).toEqual([BUNDLE_FORMAT, BUNDLE_VERSION]);
});

test("each kind's format and newest version, and the kinds' folders, are the loader's own", () => {
  expect(formatOf("map.rs")).toEqual([MAP_FORMAT, MAP_VERSION]);
  expect(formatOf("movement.rs")).toEqual([MOVEMENT_FORMAT, MOVEMENT_VERSION]);
  const folders = /^pub const FOLDERS: \[&str; \d+\] = \[([^\]]*)\];$/m.exec(rust("bundle.rs"));
  expect(folders, "FOLDERS in crates/content/src/bundle.rs").not.toBeNull();
  const named = [...(folders?.[1] ?? "").matchAll(/"([a-z]+)"/g)].map((match) => match[1]);
  expect([...CONTENT_KINDS.keys()]).toEqual(named);
  expect([...CONTENT_KINDS.values()]).toEqual([
    { format: MAP_FORMAT, newest: MAP_VERSION },
    { format: MOVEMENT_FORMAT, newest: MOVEMENT_VERSION },
  ]);
});
