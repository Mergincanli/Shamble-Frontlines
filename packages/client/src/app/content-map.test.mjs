// The page's reading of the committed test scene, the map the local match is played on: read field by field, it is the
// file itself less its `format` and `version`, key for key. Plain JavaScript, so that it reads the committed file with
// Node's own module: the page's packages have no Node types.
import { readFileSync } from "node:fs";
import { expect, test } from "vitest";
import { readMap } from "./content.ts";

test("the committed test scene reads field by field as the file itself, less its format and version", () => {
  const tree = JSON.parse(readFileSync(new URL("../../../../content/maps/test-scene.json", import.meta.url), "utf8"));
  const read = readMap(tree);
  if (!read.ok) throw new Error(read.problem);
  const { format, version, ...rest } = tree;
  expect([format, version]).toEqual(["shamble-map", 1]);
  expect(read.value).toStrictEqual(rest);
  // Every list is read whole: the test scene's 8 spawns, 23 shapes and 6 patches of ground.
  expect([read.value.spawns.length, read.value.shapes.length, read.value.regions.length]).toEqual([8, 23, 6]);
});
