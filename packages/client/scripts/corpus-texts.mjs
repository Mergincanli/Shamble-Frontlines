// The corpora for the content reader's proof (content item 1, PM-0005-2 and -3), from the repository's root:
//
//   node packages/client/scripts/corpus-texts.mjs [--check]
//
// It writes two files, and nothing else:
//
//   crates/devtools/vectors/content-corpus.txt   one line a number: `<the double's 16 hex digits> <a text of it>
//     <its canonical text>`, where the double is what this engine's `JSON.parse` makes of the text and the canonical
//     text is what its `JSON.stringify` writes. The numbers: every number of the old tuning file as written
//     (legacy-numbers.txt), 5,000 doubles from random bits, 5,000 decimal texts of up to 17 digits in many
//     spellings, 1,000 doubles that may lie exactly between their two shortest texts, and the edges.
//   crates/devtools/vectors/content-trees.txt    400 random trees, records parted by an empty line: the tree as
//     `JSON.stringify` writes it on one line, keys as they came, then as it writes it with sorted keys and an
//     indent of two, which is the content files' one canonical form.
//
// So the Rust reader and writer are compared with a browser engine's own answers, never with their own (a corpus
// is not made by the code it tests). All of it comes from one fixed seed, so `--check` can make both files again.

import { createHash } from "node:crypto";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { NUMBERS, readPlain, writePlain } from "./legacy-extract.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
export const CORPUS = path.join(root, "crates/devtools/vectors/content-corpus.txt");
export const TREES = path.join(root, "crates/devtools/vectors/content-trees.txt");
const SEED = "shamble content corpus 1";

/** 32 bytes that depend on the seed, a label and a counter alone. */
const draw = (label, at) => createHash("sha256").update(`${SEED}:${label}:${at}`).digest();
const view = new DataView(new ArrayBuffer(8));
/** A double's bits as 16 hex digits. */
export function bitsOf(value) {
  view.setFloat64(0, value);
  return view.getBigUint64(0).toString(16).padStart(16, "0");
}

/** One corpus line for a text: the double `JSON.parse` gives, the text, and `JSON.stringify`'s text of that double. */
export function lineOf(text) {
  const value = JSON.parse(text);
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${text} is not a finite number`);
  return `${bitsOf(value)} ${text} ${JSON.stringify(value)}`;
}

/** The `count` doubles from random bits: every finite one, as the engine writes it. */
function randomDoubles(count) {
  const lines = [];
  for (let at = 0; lines.length < count; at += 1) {
    const value = draw("bits", at).readDoubleBE(0);
    if (Number.isFinite(value) && !Object.is(value, -0)) lines.push(lineOf(String(value)));
  }
  return lines;
}

/** The `count` decimal texts: up to 17 digits, with and without a fraction and an exponent, in every spelling. */
function decimals(count) {
  const lines = [];
  for (let at = 0; lines.length < count; at += 1) {
    const bytes = draw("decimal", at);
    const digits = Array.from(bytes.subarray(0, 17), (byte) => String(byte % 10)).join("");
    const total = 1 + (bytes[17] % 17);
    const whole = Math.min(total, bytes[18] % 8);
    // No leading zero in the whole part (the grammar refuses one); "0" when there is no whole part.
    const head = whole === 0 ? "0" : `${1 + (bytes[19] % 9)}${digits.slice(1, whole)}`;
    const fraction = digits.slice(whole, total);
    // Half of the exponents are small; the other half reach both ends of the range (a text past the top is no number).
    const exponent = bytes[20] % 4 === 0 ? "" : ["e", "E", "e+", "e-"][bytes[21] % 4] + String(bytes.readUInt16BE(22) % (bytes[24] % 2 === 0 ? 30 : 330));
    const text = `${bytes[25] % 4 === 0 ? "-" : ""}${head}${fraction === "" ? "" : `.${fraction}`}${exponent}`;
    if (Number.isFinite(JSON.parse(text))) lines.push(lineOf(text));
  }
  return lines;
}

/** The `count` doubles with one to eight binary places and a full mantissa: where a number can lie exactly between
 *  its two shortest texts, and the rule is the text whose last digit is even. */
function ties(count) {
  return Array.from({ length: count }, (_, at) => {
    const bytes = draw("tie", at);
    const whole = (1n << 52n) + (bytes.readBigUInt64BE(0) >> 12n);
    return lineOf(String((bytes[8] % 2 === 0 ? 1 : -1) * (Number(whole) / 2 ** (1 + (bytes[9] % 8)))));
  });
}

/** The edges, each written by hand: both zeros, the largest and smallest doubles, 2^53 and its neighbours, and the
 *  places where the canonical text changes its form (1e-7 and 1e21). */
const EDGES = [
  "0", "-0", "0.0", "-0.0", "1", "-1", "1.7976931348623157e308", "5e-324", "2.2250738585072014e-308", "9007199254740992",
  "9007199254740993", "9007199254740991", "1e21", "1e20", "999999999999999900000", "123456789012345680000", "0.000001", "0.0000001",
  "1e-7", "0.0000012345678901234567", "0.1", "0.30000000000000004", "1.50", "1E5", "1e+5", "100", "327.67", "-327.67", "655.35",
];

/** The whole corpus file for the old tuning file's number texts. */
export function corpusFile(legacyTexts) {
  const lines = [...legacyTexts.map(lineOf), ...randomDoubles(5000), ...decimals(5000), ...ties(1000), ...EDGES.map(lineOf)];
  return `shamble-content-corpus 1 ${lines.length}\n${lines.join("\n")}\n`;
}

/** A value with its keys in byte order at every level: what the canonical writer writes. */
export function sortedKeys(value) {
  if (Array.isArray(value)) return value.map(sortedKeys);
  if (typeof value !== "object" || value === null) return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortedKeys(value[key])]));
}

const LETTERS = "abcdefghijklmnopqrstuvwxyz0123456789_";
// What a content file's string may hold, the quote and the line feed among it; never a backslash.
const CHARACTERS = " abcXYZ019_-<>&'/{}[]:,.\"\n";
const TREE_NUMBERS = [0, 1, -1, 0.5, 6.3, 100, 1e21, 1e-7, 327.67, -0.25, 123456789, 5e-324];

/** One random tree: objects (keys in the order they were drawn), arrays, strings and numbers, at most five deep. */
function tree(next, depth) {
  const kind = depth === 0 ? 0 : depth >= 4 ? 2 + next(2) : next(4);
  if (kind === 0) {
    const object = {};
    for (let members = next(5) + (depth === 0 ? 1 : 0); members > 0; members -= 1) {
      const key = LETTERS[next(26)] + Array.from({ length: next(8) }, () => LETTERS[next(LETTERS.length)]).join("");
      object[key] = tree(next, depth + 1);
    }
    return object;
  }
  if (kind === 1) return Array.from({ length: next(5) }, () => tree(next, depth + 1));
  if (kind === 2) return Array.from({ length: next(12) }, () => CHARACTERS[next(CHARACTERS.length)]).join("");
  return TREE_NUMBERS[next(TREE_NUMBERS.length)];
}

/** The whole trees file: `count` records, each a tree on one line and then in canonical form. */
export function treesFile(count = 400) {
  const records = Array.from({ length: count }, (_, at) => {
    let block = 0;
    let used = 32;
    let bytes;
    const next = (bound) => {
      if (used === 32) [bytes, used] = [draw(`tree ${at}`, block++), 0];
      return bytes[used++] % bound;
    };
    const made = tree(next, 0);
    return `${JSON.stringify(made)}\n${JSON.stringify(sortedKeys(made), null, 2)}\n`;
  });
  return `shamble-content-trees 1 ${count}\n${records.join("\n")}`;
}

/** `--check` or the write of both files; returns the line to print. Throws when the check fails. */
export function run(check, corpus = CORPUS, trees = TREES) {
  const legacy = readPlain(NUMBERS).toString("utf8").split("\n").slice(1, -1);
  const files = [[corpus, corpusFile(legacy)], [trees, treesFile()]];
  for (const [file, made] of files) {
    if (!check) writePlain(file, made);
    else if (readPlain(file).toString("utf8") !== made) {
      throw new Error(`${path.relative(root, file)} is not what this script makes: run it without --check`);
    }
  }
  const names = files.map(([file]) => path.relative(root, file)).join(" and ");
  return check ? `${names} are what this script makes` : `${names} written`;
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  try {
    if (args.length > 1 || (args.length === 1 && args[0] !== "--check")) throw new Error(`unknown option: ${args.join(" ")}`);
    console.log(run(args[0] === "--check"));
  } catch (error) {
    console.error(`corpus-texts: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
