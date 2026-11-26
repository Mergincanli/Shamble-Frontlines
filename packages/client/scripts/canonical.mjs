// Brings one hand-written content file to the form the game's loader takes, from the repository's root:
//
//   node packages/client/scripts/canonical.mjs maps/test-scene.json
//
// The loader refuses a file that is not byte for byte what its own writer would write (sorted keys, two spaces, one
// member a line, a last newline, each number in its shortest form). A map is written by hand, and nobody types that
// form reliably: this rewrites the file as `JSON.stringify(<its value, keys sorted>, null, 2)` and a newline, which
// is the same form for every value a content file can hold but one, a minus zero, which it refuses. It changes no
// value. Whether the result is content is the loader's to say: `cargo run -p devtools --bin content_bundle --
// --check` judges it. The one argument is a content path (`<kind's folder>/<id>.json`), never a file path: the
// script reads and writes under `content/` only, never under `content/source/`, and never through a link.

import path from "node:path";
import { fileURLToPath } from "node:url";
import { readPlain, writePlain } from "./testlines.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
/** A content path: a kind's folder and an id's file. (The loader's own rule is stricter; this keeps the script in its folder.) */
export const CONTENT_PATH = /^(maps|movement)\/[a-z0-9][a-z0-9_-]{0,63}\.json$/;

/** A value with every object's keys in the order of their bytes; lists keep their order. A minus zero is refused. */
export function sorted(value) {
  if (Array.isArray(value)) return value.map(sorted);
  if (typeof value === "object" && value !== null) return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sorted(value[key])]));
  if (Object.is(value, -0)) throw new Error("the file holds a minus zero, which this form cannot keep: write 0");
  return value;
}

/** The text in canonical form. Throws on a text that is not JSON. */
export const canonicalText = (text) => `${JSON.stringify(sorted(JSON.parse(text)), null, 2)}\n`;

/** Rewrites the content file at `contentPath` under `base`. Returns what to print. */
export function run(contentPath, base = root) {
  if (typeof contentPath !== "string" || !CONTENT_PATH.test(contentPath)) throw new Error("usage: canonical.mjs <maps|movement>/<id>.json");
  const file = path.join(base, "content", contentPath);
  const before = readPlain(file);
  const after = canonicalText(before);
  if (after === before) return `content/${contentPath}: already in canonical form`;
  writePlain(file, after);
  return `content/${contentPath}: rewritten in canonical form (${Buffer.byteLength(before)} bytes before, ${Buffer.byteLength(after)} after)`;
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    console.log(run(process.argv.length === 3 ? process.argv[2] : undefined));
  } catch (error) {
    console.error(`canonical: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
