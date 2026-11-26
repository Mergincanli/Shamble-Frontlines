// The old game's tuning file, read for the content crate's proofs, from the repository's root:
//
//   node packages/client/scripts/legacy-extract.mjs [--check]
//
// docs/legacy-tuning.json holds `true`, `false`, `null` and a string that is not ASCII: the game's own content reader
// refuses all four on purpose (Mustafa, 2026-11-26: "Keep the reader to what the content files need and refuse
// everything else"). So that file is never read by the game's reader. This script reads it with the platform's own
// JSON reader and writes what the Rust side needs as plain text:
//
//   crates/devtools/vectors/legacy-numbers.txt     every number of the file, as its text is written, in file order
//   crates/devtools/vectors/legacy-durations.txt   every duration of the file, a line each: its path, its unit (s
//     or ms), its kind (rules, look or step) and its text as written, for the time unit's rounding report. Which
//     fields are durations is no guess from their names: it is the reviewed list
//     crates/devtools/vectors/legacy-duration-paths.txt (game-designer's read), which this script only reads.
//
// Each file's first line names the tuning file by its sha256 and gives the count. `--check` writes nothing and fails
// when a committed extract is not byte for byte what this script makes (a test runs it: legacy-extract.test.mjs).
// The script takes no path from outside: its inputs and its outputs are the constants below.

import { createHash } from "node:crypto";
import { lstatSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
export const TUNING = path.join(root, "docs/legacy-tuning.json");
export const NUMBERS = path.join(root, "crates/devtools/vectors/legacy-numbers.txt");
export const DURATION_PATHS = path.join(root, "crates/devtools/vectors/legacy-duration-paths.txt");
export const DURATIONS = path.join(root, "crates/devtools/vectors/legacy-durations.txt");
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

/** Every number of a JSON text with its path (`a.b[2].c`), in the text's order: pairs of path and text as written. */
export function numberPaths(text) {
  const texts = numberTexts(text);
  const found = [];
  // The same order as the reader's own walk: a value's entries as the platform lists them.
  const walk = (value, at) => {
    if (typeof value === "number") found.push([at, texts[found.length]]);
    else if (value !== null && typeof value === "object") {
      for (const [key, inner] of Object.entries(value)) walk(inner, Array.isArray(value) ? `${at}[${key}]` : at === "" ? key : `${at}.${key}`);
    }
  };
  walk(JSON.parse(text), "");
  if (found.length !== texts.length) throw new Error("the walk and the reader count different numbers");
  return found;
}

/** The durations extract for a tuning file's bytes and the reviewed list's text (a field a line, array places as
 *  `[*]`, then its unit and its kind, tab-separated; a line that starts with `#` is a comment). */
export function durationsFile(bytes, list) {
  const fields = new Map();
  for (const line of list.split("\n").filter((line) => line !== "" && !line.startsWith("#"))) {
    const [field, unit, kind] = line.split("\t");
    if (!["s", "ms"].includes(unit) || !["rules", "look", "step"].includes(kind) || fields.has(field)) throw new Error(`the reviewed list has a line that is no field, unit and kind, or a field twice: ${line}`);
    fields.set(field, { unit, kind, places: 0 });
  }
  const lines = [];
  for (const [at, text] of numberPaths(bytes.toString("utf8"))) {
    const entry = fields.get(at.replace(/\[\d+\]/g, "[*]"));
    if (entry === undefined) continue;
    entry.places += 1;
    lines.push(`${at}\t${entry.unit}\t${entry.kind}\t${text}`);
  }
  const absent = [...fields].filter(([, entry]) => entry.places === 0).map(([field]) => field);
  if (absent.length > 0) throw new Error(`the reviewed list names fields the tuning file does not have: ${absent.join(", ")}`);
  const digest = createHash("sha256").update(bytes).digest("hex");
  return `shamble-legacy-durations 1 ${digest} ${lines.length}\n${lines.join("\n")}\n`;
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

/** `--check` or the write of both extracts; returns the line to print. Throws when the check fails. */
export function run(check, tuning = TUNING, numbers = NUMBERS, paths = DURATION_PATHS, durations = DURATIONS) {
  const bytes = readPlain(tuning);
  const files = [[numbers, numbersFile(bytes)], [durations, durationsFile(bytes, readPlain(paths).toString("utf8"))]];
  for (const [file, made] of files) {
    if (!check) writePlain(file, made);
    else if (readPlain(file).toString("utf8") !== made) {
      throw new Error(`${path.relative(root, file)} is not what this script makes from ${path.relative(root, tuning)}: run it without --check`);
    }
  }
  const counts = files.map(([file, made]) => `${path.relative(root, file)} (${made.split("\n").length - 2})`).join(" and ");
  return check ? `${counts} are what this script makes` : `${counts} written`;
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
