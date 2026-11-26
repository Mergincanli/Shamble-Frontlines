// The old game's tuning file, read for the content crate's proofs, from the repository's root:
//
//   node packages/client/scripts/legacy-extract.mjs [--check]
//
// docs/legacy-tuning.json holds `true`, `false`, `null` and a string that is not ASCII: the game's own content reader
// refuses all four on purpose (Mustafa, 2026-11-26: "Keep the reader to what the content files need and refuse
// everything else"). So that file is never read by the game's reader. This script reads it with the platform's own
// JSON reader and writes what the Rust side needs as plain text:
//
//   crates/devtools/vectors/legacy-numbers.txt   every number of the file, as its text is written, in file order
//
// The file's first line names the tuning file by its sha256 and gives the count. `--check` writes nothing and fails
// when the committed extract is not byte for byte what this script makes (a test runs it: legacy-extract.test.mjs).
// The script takes no path from outside: its one input and its one output are the constants below.

import { createHash } from "node:crypto";
import { lstatSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
export const TUNING = path.join(root, "docs/legacy-tuning.json");
export const NUMBERS = path.join(root, "crates/devtools/vectors/legacy-numbers.txt");
/** The largest file this script reads whole. */
const MAX_BYTES = 4 * 1024 * 1024;

/** Every number of a JSON text as its own text is written (`1.50` stays `1.50`), in the text's order. */
export function numberTexts(text) {
  const texts = [];
  JSON.parse(text, (_key, value, context) => {
    if (typeof value === "number") {
      // The source text of a primitive value (ECMAScript's JSON.parse source text access).
      if (typeof context?.source !== "string") throw new Error("this JavaScript engine does not give a number's source text");
      texts.push(context.source);
    }
    return value;
  });
  return texts;
}

/** The extract's whole text for a tuning file's bytes. */
export function numbersFile(bytes) {
  const texts = numberTexts(bytes.toString("utf8"));
  const digest = createHash("sha256").update(bytes).digest("hex");
  return `shamble-legacy-numbers 1 ${digest} ${texts.length}\n${texts.join("\n")}\n`;
}

/** A file's bytes, read only when the file is a plain file (never through a link) of a size this script takes. */
export function readPlain(file) {
  const entry = lstatSync(file);
  if (!entry.isFile()) throw new Error(`${file} is not a plain file`);
  if (entry.size > MAX_BYTES) throw new Error(`${file} is larger than ${MAX_BYTES} bytes`);
  return readFileSync(file);
}

/** Writes `text` to `file` through a new temporary file, never through a link standing in the file's place. */
export function writePlain(file, text) {
  const entry = lstatSync(file, { throwIfNoEntry: false });
  if (entry !== undefined && !entry.isFile()) throw new Error(`${file} is not a plain file`);
  const temporary = `${file}.${process.pid}.tmp`;
  writeFileSync(temporary, text, { flag: "wx" });
  renameSync(temporary, file);
}

/** `--check` or the write; returns the line to print. Throws when the check fails. */
export function run(check, tuning = TUNING, numbers = NUMBERS) {
  const made = numbersFile(readPlain(tuning));
  if (!check) {
    writePlain(numbers, made);
    return `${path.relative(root, numbers)}: ${made.split("\n").length - 2} numbers written`;
  }
  if (readPlain(numbers).toString("utf8") !== made) {
    throw new Error(`${path.relative(root, numbers)} is not what this script makes from ${path.relative(root, tuning)}: run it without --check`);
  }
  return `${path.relative(root, numbers)} is what this script makes`;
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  try {
    if (args.length > 1 || (args.length === 1 && args[0] !== "--check")) throw new Error(`unknown option: ${args.join(" ")}`);
    console.log(run(args[0] === "--check"));
  } catch (error) {
    console.error(`legacy-extract: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
