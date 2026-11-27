// Step 8's second part (Builder V6, 2026-11-27): the movement table the hand-over wrote, against the list he said yes
// to (DIFF-LIST (a): "The list as printed from my Export at 30 steps (.checkpoints/cs4b-content/question-2026-11-26-
// diff-list.md) is mine, and its movement values become the game's starting movement table"). By script, for each
// row of the table that fills a field: the content file's value, his Export's value at the row's place (they must be
// the same number, bit for bit), and where the printed list shows the value: part A by the old game's name, part B or
// C by the Export's name (the printed value must be the content's), or nowhere, when the old game has the same number
// (then the comparison's EQUAL list must name the row's old-game term). Nothing is typed: every value is read.
//   node .checkpoints/cs4b-content/diff-list-agree.mjs <repository root> <the printed list> <the comparison> <out file>
// All four are full paths; it writes its lines to <out file> and prints them; it exits 1 on any disagreement.
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
const args = process.argv.slice(2);
if (args.length !== 4 || args.some((p) => !p.startsWith('/') || p.split('/').includes('..'))) { console.error('four full paths are wanted'); process.exit(2); }
const [root, listPath, comparePath, out] = args;
const read = (p) => readFileSync(p, 'utf8');
const exp = JSON.parse(read(path.join(root, 'content/source/testlines-export.json')));
const table = JSON.parse(read(path.join(root, 'content/source/testlines-table.json')));
const content = JSON.parse(read(path.join(root, 'content/movement/soldier.json')));
const list = read(listPath).split('\n');
const compare = read(comparePath).split('\n');
// The printed list's parts, each a table of rows "| `name` | ... |".
const part = (head) => {
  const start = list.findIndex((line) => line.startsWith(`**${head}.`));
  if (start < 0) { console.error(`the printed list has no part ${head}`); process.exit(1); }
  const rows = [];
  for (let i = start + 1; i < list.length && !list[i].startsWith('**'); i++) {
    const cells = list[i].split('|').slice(1, -1).map((cell) => cell.trim());
    if (cells.length >= 2 && /^`[^`]+`$/.test(cells[0])) rows.push({ name: cells[0].slice(1, -1), cells, line: i + 1 });
  }
  return rows;
};
const A = part('A'), B = part('B'), C = part('C');
const equalStart = compare.findIndex((line) => line.startsWith('EQUAL'));
const equal = compare.slice(equalStart + 1).filter((line) => line.startsWith('- '));
const place = (where) => where.split('.').reduce((at, step) => (at && typeof at === 'object' ? at[step] : undefined), exp.values);
const lines = [];
let bad = 0;
let shown = 0;
for (const row of table.rows.filter((r) => Object.hasOwn(r, 'field'))) {
  const c = content[row.field];
  const v = place(row.export);
  const same = Object.is(c, v);
  const oldName = String(row.old_game ?? '').replace(/ \(.*\)$/, '');
  const hits = [];
  for (const r of A) if (r.name === oldName) hits.push({ where: `part A, line ${r.line}`, printed: r.cells[2] });
  for (const r of B) if (r.name === row.export) hits.push({ where: `part B, line ${r.line}`, printed: r.cells[2] });
  for (const r of C) if (r.name === row.export) hits.push({ where: `part C, line ${r.line}`, printed: r.cells[1] });
  let said;
  if (hits.length > 0) {
    shown += 1;
    const wrong = hits.filter((h) => Number(h.printed) !== c);
    if (wrong.length > 0) bad += 1;
    said = hits.map((h) => `${h.where} prints ${h.printed}${Number(h.printed) === c ? '' : ' (NOT the content value)'}`).join('; ');
  } else {
    const names = equal.filter((line) => oldName !== '' && line.startsWith(`- ${oldName} = `));
    if (names.length !== 1) bad += 1;
    said = names.length === 1 ? `not in the list: the old game has the same (the comparison's EQUAL list: "${names[0].slice(2)}")` : 'NOT in the list, and no EQUAL line names its old-game term';
  }
  if (!same) bad += 1;
  lines.push(`${row.field} = ${JSON.stringify(c)}; his Export's values.${row.export} = ${JSON.stringify(v)}${same ? '' : ' (DIFFERS)'}; ${said}`);
}
lines.push(`${table.rows.filter((r) => Object.hasOwn(r, 'field')).length} fields; ${shown} shown in the printed list, the rest equal to the old game; ${bad === 0 ? 'every value agrees' : `${bad} disagreement(s)`}`);
writeFileSync(out, lines.join('\n') + '\n');
console.log(lines.join('\n'));
process.exit(bad === 0 ? 0 : 1);
