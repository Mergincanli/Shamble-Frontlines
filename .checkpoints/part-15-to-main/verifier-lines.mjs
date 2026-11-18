// Verifier: on the lines ADDED under docs/ since a base: CR characters; lines over 120 characters (code points, not
// bytes) outside tables (lines starting with "|") and fenced blocks (tracked in the head file); `.checkpoints/` paths
// and whether the same line (or its sentence) names a commit. Prints file:line, length and a short cut only.
//   node verifier-lines.mjs <base>
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const base = process.argv[2];
const sh = (a) => execFileSync('git', a, { maxBuffer: 1 << 28 }).toString('utf8');
const files = sh(['diff', '--name-only', base, 'HEAD', '--', 'docs']).trim().split('\n');
let cr = 0, long = 0, cp = 0;
for (const f of files) {
  const head = readFileSync(f, 'utf8'); if (head.includes('\r')) { cr++; console.log('CR in', f); }
  const lines = head.split('\n'); const inFence = []; let fence = false;
  lines.forEach((l, i) => { if (/^\s*(```|~~~)/.test(l)) { inFence[i] = true; fence = !fence; } else inFence[i] = fence; });
  const added = new Set();
  const diff = sh(['diff', '-U0', base, 'HEAD', '--', f]);
  for (const h of diff.matchAll(/^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@/gm)) { const a = Number(h[1]), b = h[2] === undefined ? 1 : Number(h[2]); for (let i = a; i < a + b; i++) added.add(i); }
  for (const n of added) {
    const l = lines[n - 1]; if (l === undefined) continue; const len = [...l].length;
    if (len > 120 && !l.trimStart().startsWith('|') && !inFence[n - 1]) { long++; console.log('LONG', f + ':' + n, len, JSON.stringify(l.slice(0, 70))); }
    if (l.includes('.checkpoints/')) { cp++; const ctx = (lines[n - 2] || '') + ' ' + l + ' ' + (lines[n] || '');
      console.log('CHECKPOINTS', f + ':' + n, inFence[n - 1] ? '(in fence)' : '', '| commit named nearby:', /\b[0-9a-f]{7,40}\b/.test(ctx), '|', JSON.stringify(l.slice(Math.max(0, l.indexOf('.checkpoints/') - 60), l.indexOf('.checkpoints/') + 60))); }
  }
}
console.log('files', files.length, '| CR files', cr, '| long added lines outside tables/fences', long, '| added lines with .checkpoints/', cp);
