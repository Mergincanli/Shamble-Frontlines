// Compares the fix pass's E3 (e3-results-f1.json: the new gate 3f29cb8, blob 7a8480c) with the record of 2026-11-10
// (e3-results.json: dd12aeb's gate), entry by entry and column by column: verdict, failing check ids and exit code.
// Run from the repo root: node .gate/foundation-restructure/e3-compare.mjs
import { readFileSync, writeFileSync } from 'node:fs';

const N = '.checkpoints/foundation-restructure/';
const old = JSON.parse(readFileSync(N + 'e3-results.json', 'utf8'));
const neu = JSON.parse(readFileSync(N + 'e3-results-f1.json', 'utf8'));
const table = JSON.parse(readFileSync(N + 'e3-mutations.json', 'utf8')).map(m => m.id);
const lines = [], diffs = [];
let pass = 0;
for (const id of table) {
  const a = old[id], b = neu[id];
  if (!b) { lines.push(`${id}: not run yet`); continue; }
  if (/^PASS/.test(b.verdict)) pass++;
  const cols = ['old', 'new', 'newleg'].map(c => {
    const same = JSON.stringify(a[c].failing) === JSON.stringify(b[c].failing) && a[c].exit === b[c].exit;
    if (!same) diffs.push(`${id} ${c}: 2026-11-10 ${JSON.stringify(a[c].failing)}/${a[c].exit} -> now ${JSON.stringify(b[c].failing)}/${b[c].exit}`);
    return `${c} ${JSON.stringify(b[c].failing)}/${b[c].exit}${same ? '' : ' (DIFFERS)'}`;
  });
  const verdictSame = a.verdict.split(' ')[0] === b.verdict.split(' ')[0];
  if (!verdictSame) diffs.push(`${id} verdict: ${a.verdict.split(' ')[0]} -> ${b.verdict.split(' ')[0]}`);
  lines.push(`${b.verdict.split(' ')[0].padEnd(5)} ${id.padEnd(4)} ${cols.join(' | ')}`);
}
const done = table.filter(id => neu[id]).length;
const summary = `E3 on the new gate: ${pass} of ${done} PASS (${done} of ${table.length} entries run); against 2026-11-10: ${diffs.length ? diffs.length + ' differences' : 'identical failing checks and exit codes in every column'}`;
const out = [summary, ...lines, ...(diffs.length ? ['Differences:', ...diffs] : [])].join('\n') + '\n';
writeFileSync(N + 'evidence-f1/e3-compare.txt', out);
console.log(out);
