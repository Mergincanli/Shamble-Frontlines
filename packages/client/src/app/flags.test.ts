import { expect, test } from "vitest";
import { readFlags } from "./flags.ts";

test("no query: the game's own settings", () => {
  expect(readFlags("", false)).toEqual({ devtools: false, forceWebGL: false, padBytes: 0, runLabel: "" });
  expect(readFlags("", true)).toEqual({ devtools: true, forceWebGL: false, padBytes: 0, runLabel: "" });
});

test("?backend=webgl forces the fallback in every build; other values do not", () => {
  expect(readFlags("?backend=webgl", false).forceWebGL).toBe(true);
  expect(readFlags("?backend=webgpu", true).forceWebGL).toBe(false);
  expect(readFlags("?backend=WEBGL", true).forceWebGL).toBe(false);
});

test("the padding and the run label exist in development builds only", () => {
  expect(readFlags("?pad=4096&run=brave-4k", true)).toMatchObject({ padBytes: 4096, runLabel: "brave-4k" });
  expect(readFlags("?pad=4096&run=brave-4k", false)).toMatchObject({ padBytes: 0, runLabel: "" });
});

test("a padding or a label that makes no sense is ignored", () => {
  for (const pad of ["-1", "1.5", "abc", "1048577", "1e99"]) {
    expect(readFlags(`?pad=${pad}`, true).padBytes, pad).toBe(0);
  }
  expect(readFlags("?pad=1048576", true).padBytes).toBe(1_048_576);
  for (const label of ["a b", "<script>", "x".repeat(41), ""]) {
    expect(readFlags(`?run=${encodeURIComponent(label)}`, true).runLabel, label).toBe("");
  }
});
