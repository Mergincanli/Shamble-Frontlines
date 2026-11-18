// Verifier: RECORD-FILES on this change. For each file: every base line the diff removes must be blamed (at the base)
// to a commit inside #56's range (c94c81b..cd61b28); for each insertion point, print the blame of the base lines just
// before and after it (an insertion inside another lane's item would split it). Prints hashes and line numbers only.
//   node verifier-blame.mjs <base> <file> [<file> ...]
import { execFileSync } from 'node:child_process';
const [base, ...files] = process.argv.slice(2);
const sh = (a) => execFileSync('git', a, { maxBuffer: 1 << 28 }).toString('utf8');
const range = new Set(sh(['log', '--format=%H', 'c94c81b..cd61b28']).trim().split('\n'));
for (const f of files) {
  const bl = {}; let cur = null; const por = sh(['blame', '--porcelain', base, '--', f]).split('\n');
  for (const l of por) { const m = /^([0-9a-f]{40}) \d+ (\d+)/.exec(l); if (m) { cur = m[1]; bl[Number(m[2])] = cur; } }
  const diff = sh(['diff', '-U0', base, 'HEAD', '--', f]);
  let bad = 0, removed = 0; const inserts = [];
  for (const h of diff.matchAll(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/gm)) {
    const a = Number(h[1]), b = h[2] === undefined ? 1 : Number(h[2]);
    for (let i = a; i < a + b; i++) { removed++; if (!range.has(bl[i])) { bad++; console.log(f, 'REMOVED/CHANGED base line', i, 'blamed to', bl[i].slice(0, 7), 'OUTSIDE #56'); } }
    const before = b === 0 ? a : a - 1, after = b === 0 ? a + 1 : a + b;
    const tag = (i) => bl[i] ? bl[i].slice(0, 7) + (range.has(bl[i]) ? '(#56)' : '') : '-';
    inserts.push(`-${a},${b}: before l.${before} ${tag(before)} | after l.${after} ${tag(after)}`);
  }
  console.log('==', f, '| base lines removed or changed:', removed, '| blamed outside #56:', bad);
  for (const s of inserts) console.log('   hunk', s);
}
