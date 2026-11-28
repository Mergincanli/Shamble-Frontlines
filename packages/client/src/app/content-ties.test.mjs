// The page's copies of the game's loader's own words, read from the Rust source (engine-architect's round of
// 2026-11-27): the bundle's format and version the page checks after the Worker's `ready` (content.ts). Plain
// JavaScript, so that it reads the source with Node's own module: the page's packages have no Node types.
import { readFileSync } from "node:fs";
import { expect, test } from "vitest";
import { BUNDLE_FORMAT, BUNDLE_VERSION } from "./content.ts";

test("the bundle's format and version the page reads are the loader's own", () => {
  const source = readFileSync(new URL("../../../../crates/content/src/bundle.rs", import.meta.url), "utf8");
  const format = /^pub const FORMAT: &str = "([a-z-]+)";$/m.exec(source);
  const version = /^pub const VERSION: u32 = (\d+);$/m.exec(source);
  expect(format, "FORMAT in crates/content/src/bundle.rs").not.toBeNull();
  expect(version, "VERSION in crates/content/src/bundle.rs").not.toBeNull();
  expect([format?.[1], Number(version?.[1])]).toEqual([BUNDLE_FORMAT, BUNDLE_VERSION]);
});
