// te-r2-e3-compare.mjs (test-engineer, round 2): read-only, independent of e3-compare.mjs. Compares the 2026-11-10 E3
// record (e3-results.json) with the fix pass's run (e3-results-f1.json) per entry and column: exit, failing ids, build,
// base prefix, verdict, and each check's result line except 5a (smoke wording carries a random walk distance).
// Also checks that every newleg run had base prefix '' and build 'legacy/' (a move, so the new check ran and passed).
import { readFileSync } from 'node:fs';
const N = '/home/user/Shamble-Frontlines/.checkpoints/foundation-restructure/';
const a = JSON.parse(readFileSync(N + 'e3-results.json', 'utf8'));
const b = JSON.parse(readFileSync(N + 'e3-results-f1.json', 'utf8'));
const ids = Object.keys(a);
let diffs = 0, lineDiffs = 0, moveRuns = 0, n = 0;
const ka = JSON.stringify(ids), kb = JSON.stringify(Object.keys(b));
console.log(`ids: ${ids.length} vs ${Object.keys(b).length}; same order: ${ka === kb}`);
for (const id of ids) {
  const x = a[id], y = b[id];
  if (!y) { console.log(`MISSING ${id}`); diffs++; continue; }
  if (x.verdict !== y.verdict) { console.log(`${id} verdict ${x.verdict} vs ${y.verdict}`); diffs++; }
  for (const c of ['old', 'new', 'newleg']) {
    n++;
    for (const k of ['exit', 'build', 'basePrefix']) if (JSON.stringify(x[c][k]) !== JSON.stringify(y[c][k])) { console.log(`${id} ${c} ${k}: ${JSON.stringify(x[c][k])} vs ${JSON.stringify(y[c][k])}`); diffs++; }
    if (JSON.stringify(x[c].failing) !== JSON.stringify(y[c].failing)) { console.log(`${id} ${c} failing: ${JSON.stringify(x[c].failing)} vs ${JSON.stringify(y[c].failing)}`); diffs++; }
    const keys = new Set([...Object.keys(x[c].lines || {}), ...Object.keys(y[c].lines || {})]);
    for (const k of keys) {
      if (k === '5a') continue;
      if ((x[c].lines || {})[k] !== (y[c].lines || {})[k]) { lineDiffs++; if (lineDiffs <= 8) console.log(`${id} ${c} line ${k}:\n  A ${(x[c].lines || {})[k]}\n  B ${(y[c].lines || {})[k]}`); }
    }
    if (c === 'newleg' && y[c].basePrefix === '' && y[c].build === 'legacy/') moveRuns++;
  }
}
console.log(`runs compared: ${n}; field diffs: ${diffs}; non-5a line diffs: ${lineDiffs}; newleg runs that were a move (prefix '' -> build legacy/): ${moveRuns} of ${ids.length}`);
