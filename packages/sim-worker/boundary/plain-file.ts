// The one read of a committed recording for the tests that run under Node (the boundary tests and the browser
// specs): a file is read only when it and its folder are what they are by their own type, a plain file in a plain
// folder, never a link. The repository's tree is not trusted: a link there could hand a test any file of the machine
// as a recording (Mustafa's LINKS-CHECK (c), 2026-11-26). The Rust tests read through `devtools::files::read_plain`,
// which holds the same rule.
import { lstatSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

/** The most bytes one read takes: far above any committed recording. */
const MAX_BYTES = 64 * 1024 * 1024;

/** The bytes of the plain file `name` in the plain folder `folder`; anything else throws, by name. */
export function readPlainFile(folder: string | URL, name: string): Uint8Array {
  // The folder by its own path, with no separator at its end: a path that ends in one is followed through a link.
  const own = (typeof folder === "string" ? folder : fileURLToPath(folder)).replace(/[\\/]+$/, "");
  if (name === "" || name === "." || name === ".." || /[\\/]/.test(name)) throw new Error(`${JSON.stringify(name)} is not a file's name`);
  if (!lstatSync(own).isDirectory()) throw new Error(`${own} is a link or not a plain folder`);
  const file = path.join(own, name);
  const entry = lstatSync(file);
  if (!entry.isFile()) throw new Error(`${file} is a link or not a plain file`);
  if (entry.size > MAX_BYTES) throw new Error(`${file} is ${entry.size} bytes, more than a read takes`);
  return readFileSync(file);
}
