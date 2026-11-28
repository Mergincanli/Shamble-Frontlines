// The hand-over from Mustafa's Testlines sandbox to the game's content (his point 8 of 2026-11-25: "The values come
// from my Export and from nothing else ... No value is retyped by hand"), from the repository's root:
//
//   node packages/client/scripts/testlines.mjs --check       the content holds his Export's values; writes nothing
//   node packages/client/scripts/testlines.mjs write         writes content/movement/soldier.json from his Export
//   node packages/client/scripts/testlines.mjs compare-old   his Export against the old game's tables; writes nothing
//   node packages/client/scripts/testlines.mjs table         the table as people read it; writes nothing
//
// His Export is read here, with the platform's own JSON reader, and nowhere else: the game never loads it (his
// CONTENT-SOURCE answer of 2026-11-26: "A tool script reads my Export with the platform's own JSON reader, the game's
// reader stays as narrow as you proposed, and the Rust tool proves the written content file is canonical"). What
// this script writes is judged afterwards by the game's own loader (`content_bundle --check`).
//
// The files, all constants of this script (no path comes from an argument, the environment or a file's contents):
// content/source/testlines-export.json (his Export, saved byte for byte from the chat), its manifest beside it,
// content/source/testlines-table.json (which of its values goes to which field), and the one file `write` makes,
// content/movement/soldier.json. The table names fields, never files: a row cannot send a write anywhere.

import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readSync, renameSync, rmSync, writeSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
/** Where each file lies, from the repository's root. */
export const PATHS = {
  export: "content/source/testlines-export.json",
  manifest: "content/source/testlines-export.manifest.txt",
  table: "content/source/testlines-table.json",
  target: "content/movement/soldier.json",
  legacy: "docs/legacy-tuning.json",
};

/**
 * The sha256 of the Export this script hands over: the one Mustafa sent on 2026-11-26 at 30 steps a second. A new
 * Export changes this line in the change that saves it, so the Export in use is named in reviewed code and not only
 * in the manifest that lies beside it.
 */
export const EXPORT_SHA256 = "a9d47785ed8f70d895d10480d180c9bba5a67f7e955e0c460c0c1bef7718c4ab";

/** The header of the file `write` makes: the movement kind's format and version, and the table's id. */
// The version is the movement kind's newest (crates/content/src/movement.rs's VERSION; a test ties the two).
export const HEADER = { format: "shamble-movement", id: "soldier", version: 2 };
/** The most bytes one read takes: far above any of these files. */
export const MAX_READ_BYTES = 1024 * 1024;

const sha256 = (text) => createHash("sha256").update(text).digest("hex");
const isObject = (value) => typeof value === "object" && value !== null && !Array.isArray(value);
const own = (object, key) => (isObject(object) && Object.hasOwn(object, key) ? object[key] : undefined);
/** A value as a line shows it: JSON's text, but a minus zero with its sign (JSON's own text for it is "0"). */
const shown = (value) => (Object.is(value, -0) ? "-0" : JSON.stringify(value));

/**
 * Every folder from `base` down to `file`'s own is a plain folder, never a link (`file` must lie under `base`):
 * so no link anywhere on the way, the content folder itself included, leads a read or a write elsewhere.
 */
function plainFolders(file, base, refused) {
  const relative = path.relative(base, path.dirname(file));
  if (relative === ".." || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) throw new Error(`${file} is not under ${base}: ${refused}`);
  let at = base;
  for (const part of ["", ...(relative === "" ? [] : relative.split(path.sep))]) {
    at = path.join(at, part);
    if (!lstatSync(at).isDirectory()) throw new Error(`${at} is a link or not a plain folder: ${refused}`);
  }
}

/**
 * A plain file's text: every folder from `base` down to it and the file itself are no links, and the file is at
 * most `MAX_READ_BYTES`. The file is opened without following a link (a link put in its place after the check is
 * refused there) and judged again by what was opened. Its path is first resolved as it reads, so the folders checked
 * and the file opened are one (a `link/..` in it is never followed: security-auditor's round 2).
 */
export function readPlain(file, base = path.dirname(file)) {
  file = path.resolve(file);
  plainFolders(file, base, "it is not read");
  if (!lstatSync(file).isFile()) throw new Error(`${file} is a link or not a plain file: it is not read`);
  const handle = openSync(file, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    if (!fstatSync(handle).isFile()) throw new Error(`${file} is not a plain file: it is not read`);
    const bytes = Buffer.alloc(MAX_READ_BYTES + 1);
    let length = 0;
    for (let read; length < bytes.length && (read = readSync(handle, bytes, length, bytes.length - length, null)) > 0; ) length += read;
    if (length > MAX_READ_BYTES) throw new Error(`${file} is larger than the ${MAX_READ_BYTES} bytes a read takes`);
    return bytes.subarray(0, length).toString("utf8");
  } finally {
    closeSync(handle);
  }
}

/**
 * Puts `text` in the target's place: written whole into a new file beside it (made only if no such file is there,
 * never through a link), then renamed over it. Every folder from `base` down to the target's must be there as a
 * plain folder, and whatever stands in the target's place must be a plain file.
 */
export function writePlain(file, text, base = path.dirname(file)) {
  file = path.resolve(file);
  plainFolders(file, base, "nothing is written");
  const there = lstatSync(file, { throwIfNoEntry: false });
  if (there !== undefined && !there.isFile()) throw new Error(`${file} is a link or not a plain file: nothing is written`);
  const fresh = path.join(path.dirname(file), `.${path.basename(file)}.new`);
  const handle = openSync(fresh, "wx");
  try {
    writeSync(handle, text);
    closeSync(handle);
    renameSync(fresh, file);
  } catch (error) {
    rmSync(fresh, { force: true });
    throw error;
  }
}

/**
 * His Export and the table, read and held against each other's rules. Throws, naming what is wrong: an Export whose
 * bytes are not the pinned ones or not the manifest's; one that is not a Testlines Export of version 1; one made at
 * another step rate than the game's 30, or with a warning of its own (his point f); a table that is not a table.
 */
export function readSource({ exportText, manifestText, tableText }, pinned = EXPORT_SHA256) {
  const found = sha256(exportText);
  if (found !== pinned) throw new Error(`the saved Export has sha256 ${found}, and this script hands over ${pinned}`);
  if (!manifestText.split("\n").includes(`sha256: ${found}`)) throw new Error("the manifest beside the Export does not name the Export's sha256");
  const exp = JSON.parse(exportText);
  if (own(exp, "format") !== "testlines-export" || own(exp, "format_version") !== 1) throw new Error("the saved file is not a Testlines Export of version 1");
  const rate = exportValue(exp, "simulation.step_rate_hz");
  if (rate !== 30) throw new Error(`the Export was made at ${JSON.stringify(rate)} steps a second, and the game runs at 30: it is not handed over`);
  const warnings = own(exp, "warnings");
  if (!Array.isArray(warnings) || warnings.length > 0) throw new Error(`the Export carries a warning of its own: ${JSON.stringify(warnings)}`);
  const table = JSON.parse(tableText);
  if (own(table, "format") !== "shamble-testlines-table" || own(table, "version") !== 1 || !Array.isArray(own(table, "rows"))) {
    throw new Error("the table is not a shamble-testlines-table of version 1 with rows");
  }
  return { exp, table };
}

/**
 * A row's keys: where the value stands in the Export, then either the field it fills or the value it must have; or,
 * for a number his sandbox keeps in its code and not in its Export (his NO-SOURCE (b) of 2026-11-28), no place in
 * the Export but the field, its `value` and its place in his sandbox (`sandbox`: file and lines).
 */
const ROW_KEYS = ["export", "field", "must_be", "old_game", "sandbox", "unit", "value", "what"];

/**
 * The value at `values.<group>.<name>` of the Export, or at `measured.<name>` (his Export's block of what it measured
 * on screen), or nothing when the Export has no such place. While the Export's values hold a group named `measured`
 * too, no `measured.` place is read at all: a place names one value or none.
 */
function exportValue(exp, place) {
  const steps = typeof place === "string" ? place.split(".") : [];
  if (steps.length !== 2) return undefined;
  if (steps[0] !== "measured") return own(own(own(exp, "values"), steps[0]), steps[1]);
  return own(own(exp, "values"), "measured") === undefined ? own(own(exp, "measured"), steps[1]) : undefined;
}

/** Where a filled value comes from, as the check's lines name it: whose it is, and its place there. */
const fromWhere = ({ place, sandbox }) =>
  place === null ? ["his sandbox", sandbox] : ["his Export", place.startsWith("measured.") ? place : `values.${place}`];

/**
 * The table's rows, read strictly: `fills` (field, place or sandbox line, and value, for the rows that fill a field
 * of the content file) and one line in `problems` for everything a row may not be. A row has only the keys of
 * `ROW_KEYS`. A row with a place in the Export names a place the Export has, holds no value or sandbox line of its
 * own, and fills a field, named as a key is (never a path), with a finite number, or holds the Export to one value
 * (`must_be`: the step rate, stamina being on). A row with no place in the Export fills a field with its own finite
 * `value` and names its place in his sandbox. No field is filled twice.
 */
export function readRows(exp, table) {
  const fills = [];
  const problems = [];
  for (const [at, row] of table.rows.entries()) {
    const fromSandbox = isObject(row) && !Object.hasOwn(row, "export");
    const said = fromSandbox ? `row ${at + 1} (his sandbox's ${JSON.stringify(own(row, "field"))})` : `row ${at + 1} (${JSON.stringify(own(row, "export"))})`;
    const value = fromSandbox ? own(row, "value") : exportValue(exp, own(row, "export"));
    const extra = isObject(row) ? Object.keys(row).filter((key) => !ROW_KEYS.includes(key)) : ["not an object"];
    if (extra.length > 0) problems.push(`${said}: a row holds ${ROW_KEYS.join(", ")} and nothing else, not ${extra.join(", ")}`);
    else if (fromSandbox && (typeof row.sandbox !== "string" || row.sandbox.trim() === "" || Object.hasOwn(row, "must_be") || !Object.hasOwn(row, "field"))) {
      problems.push(`${said}: a row with no place in his Export fills a field with its own value and names its place in his sandbox`);
    } else if (!fromSandbox && (Object.hasOwn(row, "value") || Object.hasOwn(row, "sandbox"))) {
      problems.push(`${said}: a row with a place in his Export takes its value from there, with no value or sandbox line of its own`);
    } else if (value === undefined) problems.push(fromSandbox ? `${said}: it has no value` : `${said}: his Export has no such value`);
    else if (Object.hasOwn(row, "must_be") === Object.hasOwn(row, "field")) problems.push(`${said}: a row fills a field or holds the Export to a value, one of the two`);
    else if (Object.hasOwn(row, "must_be")) {
      if (!Object.is(value, row.must_be)) problems.push(`${said}: his Export has ${JSON.stringify(value)}, and the game is built for ${JSON.stringify(row.must_be)}`);
    } else if (typeof row.field !== "string" || !/^[a-z][a-z0-9_]{0,63}$/.test(row.field) || Object.hasOwn(HEADER, row.field)) {
      problems.push(`${said}: ${JSON.stringify(row.field)} is not a field's name`);
    } else if (typeof value !== "number" || !Number.isFinite(value)) problems.push(`${said}: ${JSON.stringify(value)} is not a number a content file can hold`);
    else if (fills.some((fill) => fill.field === row.field)) problems.push(`${said}: the field ${row.field} is filled twice`);
    else fills.push({ field: row.field, place: fromSandbox ? null : row.export, sandbox: fromSandbox ? row.sandbox : null, value });
  }
  return { fills, problems };
}

/** An object's own keys in the order of their bytes, as the game's writer orders them (the keys are ASCII). */
const sortedKeys = (object) => Object.fromEntries(Object.keys(object).sort().map((key) => [key, object[key]]));

/** The content file's text for these rows: the header and one member per filled field, as the game's writer writes it. */
export function contentText(fills) {
  return `${JSON.stringify(sortedKeys({ ...HEADER, ...Object.fromEntries(fills.map(({ field, value }) => [field, value])) }), null, 2)}\n`;
}

/**
 * Where the content file differs from his Export through the table, one line each; empty when it holds exactly his
 * values. Numbers are compared as numbers, bit for bit (`Object.is`), so a value one step of a double away is
 * found; a field no row fills and a row whose field is missing are found by name; and the text itself must be the
 * one `write` gives, so nothing else can differ either.
 */
export function checkProblems(exp, table, text) {
  const { fills, problems } = readRows(exp, table);
  if (problems.length > 0) return problems;
  let content;
  try {
    content = JSON.parse(text);
  } catch (error) {
    return [`${PATHS.target} is not JSON: ${error.message}`];
  }
  if (!isObject(content)) return [`${PATHS.target} is not an object`];
  const lines = [];
  for (const [key, value] of Object.entries(HEADER)) {
    if (!Object.is(own(content, key), value)) lines.push(`${PATHS.target}: ${key} is ${JSON.stringify(own(content, key))}, not ${JSON.stringify(value)}`);
  }
  for (const fill of fills) {
    const { field, value } = fill;
    const [whose, where] = fromWhere(fill);
    if (!Object.hasOwn(content, field)) lines.push(`${PATHS.target} has no ${field} (${whose}'s ${where})`);
    else if (!Object.is(content[field], value)) lines.push(`${PATHS.target}: ${field} is ${shown(content[field])}, and ${whose} has ${shown(value)} (${where})`);
  }
  for (const key of Object.keys(content)) {
    if (!Object.hasOwn(HEADER, key) && !fills.some(({ field }) => field === key)) lines.push(`${PATHS.target}: no row of the table fills ${key}`);
  }
  if (lines.length === 0 && text !== contentText(fills)) lines.push(`${PATHS.target} holds his values and is not the text \`write\` gives (its form differs)`);
  return lines;
}

/**
 * His Export's `old_game_terms` against the old game's tables (`docs/legacy-tuning.json`): the names that are
 * `equal`, the ones that are `different` (his tunings: the list he confirms, his point 8c), and the ones with
 * `noCounterpart`. A name is looked up as `TABLE.a.b`; in a table whose values are a list the next step is an
 * entry's id; a note in brackets is not part of the name ("(deg)": the old value is in radians). The Export
 * rounds these names to 3, 4, 5 or 6 decimals: a value is equal when the old one, rounded to one of those, is it.
 */
export function compareOld(exp, legacy) {
  const result = { equal: [], different: [], noCounterpart: [] };
  for (const [name, value] of Object.entries(own(exp, "old_game_terms") ?? {})) {
    const note = /\s\(([^)]*)\)$/.exec(name)?.[1] ?? "";
    const [table, ...steps] = name.replace(/\s\([^)]*\)$/, "").split(".");
    let at = own(own(own(legacy, "tables"), table), "values");
    for (const step of steps) at = Array.isArray(at) ? at.find((entry) => own(entry, "id") === step) : own(at, step);
    if (typeof at !== "number") {
      result.noCounterpart.push(`${name} = ${JSON.stringify(value)}`);
      continue;
    }
    const old = note === "deg" ? (at * 180) / Math.PI : at;
    const same = typeof value === "number" && [3, 4, 5, 6].some((digits) => Number(old.toFixed(digits)) === value);
    const shown = note === "deg" ? `${at} rad = ${Number(old.toFixed(4))} deg` : `${at}`;
    if (same) result.equal.push(`${name} = ${value}`);
    else result.different.push(`${name}: the old game ${shown}; his Export ${JSON.stringify(value)}`);
  }
  return result;
}

/** The table as people read it, printed from the table's own rows and the Export's own values. */
export function tableMarkdown(exp, table) {
  const cell = (value) => (value === undefined ? "" : String(value).replaceAll("|", "\\|"));
  const rows = table.rows.map((row) => {
    const goes = Object.hasOwn(row, "field") ? `\`${row.field}\`` : `must be ${JSON.stringify(row.must_be)}`;
    const where = Object.hasOwn(row, "export") ? `\`${cell(row.export)}\`` : `his sandbox: ${cell(row.sandbox)}`;
    const value = Object.hasOwn(row, "export") ? exportValue(exp, row.export) : row.value;
    return `| ${where} | ${cell(row.unit)} | ${cell(row.old_game)} | ${cell(row.what)} | ${goes} | ${cell(JSON.stringify(value))} |`;
  });
  return ["| His Export (`values.`... or `measured.`...), or his sandbox | Unit | The old game's name | What it is | In `movement/soldier.json` | His value |", "|---|---|---|---|---|---|", ...rows].join("\n");
}

/** One mode of the script on the files of `paths` under `base`. Returns what to print; throws what stops it. */
export function run(mode, base = root, paths = PATHS, pinned = EXPORT_SHA256) {
  // Every folder from `base` down is held to be no link, the content folder included.
  const read = (name) => readPlain(path.join(base, paths[name]), base);
  const { exp, table } = readSource({ exportText: read("export"), manifestText: read("manifest"), tableText: read("table") }, pinned);
  if (mode === "compare-old") {
    const { equal, different, noCounterpart } = compareOld(exp, JSON.parse(read("legacy")));
    return [`DIFFERENT (${different.length}): his tunings, for him to confirm`, ...different, "", `NO COUNTERPART in the old game's tables (${noCounterpart.length})`, ...noCounterpart, "", `EQUAL (${equal.length})`, ...equal].join("\n");
  }
  if (mode === "table") return tableMarkdown(exp, table);
  const { fills, problems } = readRows(exp, table);
  if (mode === "write") {
    if (problems.length > 0) throw new Error(`the table is refused, nothing is written:\n  ${problems.join("\n  ")}`);
    writePlain(path.join(base, paths.target), contentText(fills), base);
    return `${paths.target}: ${counted(fills, pinned)}, written`;
  }
  if (mode !== "--check") throw new Error("usage: testlines.mjs --check | write | compare-old | table");
  const lines = checkProblems(exp, table, read("target"));
  if (lines.length > 0) throw new Error(`the content is not his Export's and his sandbox's as the table names them:\n  ${lines.join("\n  ")}`);
  return `${paths.target}: ${counted(fills, pinned)}, each as the table names it`;
}

/** How many values a file holds, and from where: his Export (by its sha256) and his sandbox. */
function counted(fills, pinned) {
  const fromSandbox = fills.filter(({ place }) => place === null).length;
  return `${fills.length} values, ${fills.length - fromSandbox} from his Export (sha256 ${pinned.slice(0, 8)}...) and ${fromSandbox} from his sandbox`;
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    console.log(run(process.argv.length === 3 ? process.argv[2] : ""));
  } catch (error) {
    console.error(`testlines: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
