// Compares the saved Testlines Export's "old_game_terms" with docs/legacy-tuning.json and lists every difference
// (his point 8c: "Those are my tunings, and I confirm the list before it becomes the starting table"). A first
// version, in the notes; the change-set builds the kept one. From the worktree's root:
//   node .checkpoints/cs4-walk/testlines-compare.mjs
// How a name is looked up: "TABLE.a.b" is tables.TABLE.values.a.b; where a table's values are a list (WEAPONS,
// UNIT_TYPES), the next part is the entry's id; a bare name (EJUMP_REACH) is the table's own number. A note in
// brackets after a name is not part of the path: "(deg)" means the old value is in radians and is turned into
// degrees; "(speed-up)" and "(stop)" both mean the old game's one value. The Export writes these names rounded to
// 3, 4, 5 or 6 decimals, depending on the name; a value counts as equal when the old value rounded to one of those
// gives exactly the Export's value. (The first run rounded to the decimals the Export's number happened to print,
// so an Export value of 0 passed as equal to an old 0.3: caught by reading the output against "changed_from_start";
// the kept script gets a planted case for it.) Node's built-in modules only.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/cs4-walk/';
const exportText = readFileSync(NOTES + 'testlines-export.json', 'utf8');
const manifest = readFileSync(NOTES + 'testlines-export.manifest.txt', 'utf8');
const digest = createHash('sha256').update(exportText).digest('hex');
if (!manifest.includes(`sha256: ${digest}`)) { console.error('the saved Export does not match its manifest'); process.exit(1); }
const terms = JSON.parse(exportText).old_game_terms;
const legacyText = readFileSync('docs/legacy-tuning.json', 'utf8');
const tables = JSON.parse(legacyText).tables;
function lookUp(name) {
  const note = (name.match(/\s\(([^)]*)\)$/) || [])[1] || '';
  const parts = name.replace(/\s\([^)]*\)$/, '').split('.');
  const table = tables[parts[0]];
  if (!table) return { found: false, why: `no table ${parts[0]}` };
  let at = table.values;
  for (const part of parts.slice(1)) {
    if (Array.isArray(at)) at = at.find((entry) => entry && entry.id === part);
    else if (at && typeof at === 'object') at = Object.prototype.hasOwnProperty.call(at, part) ? at[part] : undefined;
    else at = undefined;
    if (at === undefined) return { found: false, why: `no "${part}" under ${parts[0]}` };
  }
  if (typeof at !== 'number') return { found: false, why: 'the old value is not a number' };
  const old = note === 'deg' ? (at * 180) / Math.PI : at;
  return { found: true, old, raw: at, note };
}
const equal = [], different = [], missing = [];
for (const [name, value] of Object.entries(terms)) {
  const got = lookUp(name);
  if (!got.found) { missing.push(`${name} = ${JSON.stringify(value)} (${got.why})`); continue; }
  const same = typeof value === 'number' && [3, 4, 5, 6].some((d) => Number(got.old.toFixed(d)) === value);
  const shown = got.note === 'deg' ? `${got.raw} rad = ${Number(got.old.toFixed(4))} deg` : `${got.raw}`;
  (same ? equal : different).push(same ? `${name} = ${value}` : `${name}: the old game ${shown}; his Export ${JSON.stringify(value)}`);
}
const out = [
  `The Export's old_game_terms against docs/legacy-tuning.json (${Object.keys(terms).length} names)`,
  `Export: ${NOTES}testlines-export.json, sha256 ${digest}`,
  `Legacy tuning: docs/legacy-tuning.json, sha256 ${createHash('sha256').update(legacyText).digest('hex')}`,
  '',
  `DIFFERENT (${different.length}): his tunings, for him to confirm`,
  ...different.map((l) => `- ${l}`),
  '',
  `NO COUNTERPART in the legacy file (${missing.length})`,
  ...missing.map((l) => `- ${l}`),
  '',
  `EQUAL (${equal.length})`,
  ...equal.map((l) => `- ${l}`),
  '',
];
writeFileSync(NOTES + 'testlines-compare.txt', out.join('\n'));
console.log(out.join('\n'));
