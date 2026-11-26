// The one read of a committed recording refuses whatever is no plain file in a plain folder. The links are planted
// in a temporary folder; a machine that cannot make a link fails this test, it does not skip it.
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { afterAll, beforeAll, expect, test } from "vitest";
import { readPlainFile } from "./plain-file.ts";

let dir;
beforeAll(() => {
  dir = mkdtempSync(path.join(tmpdir(), "shamble-plain-"));
  mkdirSync(path.join(dir, "recordings"));
  writeFileSync(path.join(dir, "recordings", "a.rec"), "the recording");
  writeFileSync(path.join(dir, "outside.txt"), "not a recording");
  symlinkSync(path.join(dir, "outside.txt"), path.join(dir, "recordings", "link.rec"));
  symlinkSync("/dev/zero", path.join(dir, "recordings", "zero.rec"));
  symlinkSync(path.join(dir, "recordings"), path.join(dir, "linked"));
  mkdirSync(path.join(dir, "recordings", "folder.rec"));
});
afterAll(() => rmSync(dir, { recursive: true, force: true }));

test("a plain file in a plain folder is read, by a path or by an address, with or without the folder's last separator", () => {
  const folder = path.join(dir, "recordings");
  for (const given of [folder, `${folder}${path.sep}`, pathToFileURL(`${folder}${path.sep}`)]) {
    expect(Buffer.from(readPlainFile(given, "a.rec")).toString()).toBe("the recording");
  }
});

test("a link, a folder, a missing file and a name that is a path are refused, and a linked folder with them", () => {
  const folder = path.join(dir, "recordings");
  // A link to a file outside, and a link to a file that never ends: neither is opened.
  expect(() => readPlainFile(folder, "link.rec")).toThrow("link.rec is a link or not a plain file");
  expect(() => readPlainFile(folder, "zero.rec")).toThrow("zero.rec is a link or not a plain file");
  expect(() => readPlainFile(folder, "folder.rec")).toThrow("is a link or not a plain file");
  expect(() => readPlainFile(folder, "missing.rec")).toThrow("ENOENT");
  for (const name of ["", ".", "..", "../outside.txt", "sub/a.rec", "sub\\a.rec"]) {
    expect(() => readPlainFile(folder, name), name).toThrow("is not a file's name");
  }
  // The same file through a folder that is a link: refused at the folder, with or without a separator at its end.
  expect(() => readPlainFile(path.join(dir, "linked"), "a.rec")).toThrow("is a link or not a plain folder");
  expect(() => readPlainFile(`${path.join(dir, "linked")}${path.sep}`, "a.rec")).toThrow("is a link or not a plain folder");
});
