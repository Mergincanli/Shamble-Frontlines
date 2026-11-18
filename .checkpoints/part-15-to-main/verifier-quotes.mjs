// Verifier: for the lines ADDED to each docs file since a base (git diff, hunks kept together), finds every span
// between straight double quotes (12+ chars) and says which source holds it word for word (white space aside):
// P1 (his answers), P2 (his go), PART15 (Part 15's text), OLD (the file at the base), or NONE.
//   node verifier-quotes.mjs <base> <file> [<file> ...]
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const [base, ...files] = process.argv.slice(2);
const P = '.checkpoints/part-15-to-main/';
const flat = (s) => s.replace(/^\s*>\s?/gm, '').replace(/\s+/g, ' ');
const words = (f) => { const t = readFileSync(f, 'utf8'); return t.slice(t.indexOf('\n---\n') + 5); };
const S = { P1: flat(words(P + 'mustafa-2026-11-17-2306.md')), P2: flat(words(P + 'mustafa-2026-11-17-2357-merge-56.md')), PART15: flat(readFileSync(P + 'part-15-the-second-lane.md', 'utf8')) };
for (const f of files) {
  let old = ''; try { old = flat(execFileSync('git', ['show', `${base}:${f}`], { maxBuffer: 1 << 26 }).toString('utf8')); } catch {}
  const diff = execFileSync('git', ['diff', '-U0', base, 'HEAD', '--', f], { maxBuffer: 1 << 26 }).toString('utf8');
  const hunks = diff.split(/^@@.*$/m).slice(1).map((h) => h.split('\n').filter((l) => l.startsWith('+') && !l.startsWith('+++')).map((l) => l.slice(1)).join('\n')).filter(Boolean);
  let n = 0;
  for (const h of hunks) {
    const parts = flat(h).split('"');
    for (let i = 1; i < parts.length; i += 2) {
      const q = parts[i].trim(); if (q.length < 12) continue; n++;
      const where = Object.keys(S).filter((k) => S[k].includes(q)); if (old.includes(q)) where.push('OLD');
      console.log(f.replace('docs/', ''), '|', where.join('+') || 'NONE', '|', q.slice(0, 90));
    }
    if (parts.length % 2 === 0) console.log(f, 'hunk with an odd number of quote marks (a quote may cross hunks):', flat(h).slice(0, 80));
  }
  console.log('==', f, 'quoted spans:', n);
}
