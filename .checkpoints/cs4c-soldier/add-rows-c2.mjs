// C2 (4c-2's step 1): copies the thirteen rows of game-designer's block 10 (`game-designer-read.md`, the json block
// under "The thirteen rows") into the Testlines table, `content/source/testlines-table.json`, by script: no value is
// typed by hand. The sandbox's constant names in brackets are left out ("numbers and behaviour only", his
// TESTLINES answer): each row keeps its file and lines. The table is written back in its own form (checked first:
// the file is `JSON.stringify(table, null, 2)` plus a newline, with each row's keys in byte order). Run once:
//   node /home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/add-rows-c2.mjs
import { readFileSync, writeFileSync } from "node:fs";

const root = "/home/user/Shamble-Frontlines/.gate/cs4b-2";
const notes = readFileSync(`${root}/.checkpoints/cs4c-soldier/game-designer-read.md`, "utf8");
const tablePath = `${root}/content/source/testlines-table.json`;
const tableText = readFileSync(tablePath, "utf8");

const start = notes.indexOf("### The thirteen rows");
const open = notes.indexOf("```json\n", start);
const close = notes.indexOf("\n```", open + 8);
if (start < 0 || open < 0 || close < 0) throw new Error("game-designer's json block is not found");
const rows = JSON.parse(notes.slice(open + 8, close));
if (rows.length !== 13) throw new Error(`${rows.length} rows, not 13`);

const sorted = (object) => Object.fromEntries(Object.keys(object).sort().map((key) => [key, object[key]]));
const table = JSON.parse(tableText);
if (`${JSON.stringify(table, null, 2)}\n` !== tableText) throw new Error("the table is not in the form this script writes");
if (table.rows.some((row) => JSON.stringify(row) !== JSON.stringify(sorted(row)))) throw new Error("a row's keys are not in byte order");
const fields = new Set(table.rows.map((row) => row.field).filter(Boolean));

for (const row of rows) {
  if (fields.has(row.field)) throw new Error(`${row.field} is in the table already`);
  if (typeof row.sandbox === "string") row.sandbox = row.sandbox.replace(/ \([A-Z][A-Z0-9_]*\)/g, "");
  table.rows.push(sorted(row));
}
writeFileSync(tablePath, `${JSON.stringify(table, null, 2)}\n`);
console.log(`${tablePath}: ${rows.length} rows added (${rows.filter((row) => row.sandbox).length} from his sandbox, ${rows.filter((row) => row.export).length} from his Export), ${table.rows.length} rows in all`);
for (const row of rows) console.log(`  ${row.field}: ${row.export ?? `${row.value} (sandbox ${row.sandbox})`}`);
