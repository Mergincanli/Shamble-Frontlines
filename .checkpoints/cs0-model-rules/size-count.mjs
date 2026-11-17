// Counts change-set 0's written lines against Mustafa's stop line (1,200 written lines; copied lines do not count).
//   node .checkpoints/cs0-model-rules/size-count.mjs [<base ref>]     (default origin/main; the head is the working tree)
// Written = lines added under docs/ minus the copied ones: his words quoted in DECISIONS (the five entries), the
// appendix's fenced blocks, and the plan copied from commit 68addf9 into the report.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const base = process.argv[2] ?? 'origin/main';
const mb = execFileSync('git', ['merge-base', base, 'HEAD']).toString().trim();
const num = execFileSync('git', ['diff', '--numstat', mb, '--', 'docs']).toString().trim().split('\n').filter(Boolean)
  .map((l) => { const [a, d, f] = l.split('\t'); return { added: Number(a), deleted: Number(d), file: f }; });
const lines = (s) => s.split('\n').length;
const N = '.checkpoints/build-plan-slice-1/', C = '.checkpoints/cs0-model-rules/';
const words = (p) => { const t = readFileSync(p, 'utf8'); return t.slice(t.indexOf('\n---\n') + 5).replace(/\n$/, ''); };
const quoted = [N + 'mustafa-2026-11-14-task.md', N + 'mustafa-2026-11-14-addition.md', N + 'mustafa-2026-11-17-go.md',
  C + 'mustafa-2026-11-17-2058.md', C + 'mustafa-2026-11-17-2133.md'].reduce((n, p) => n + lines(words(p)), 0);
const app = readFileSync('docs/reports/2026-11-17-build-plan-slice-1-appendix.md', 'utf8');
let blocks = 0; for (const m of app.matchAll(/````text\n([\s\S]*?)\n````\n/g)) blocks += lines(m[1]);
const plan = lines(execFileSync('git', ['show', '68addf9:.checkpoints/build-plan-slice-1/plan.md'], { maxBuffer: 1 << 26 }).toString().replace(/\n$/, ''));
const added = num.reduce((n, x) => n + x.added, 0);
for (const x of num) console.log(String(x.added).padStart(6), String(-x.deleted).padStart(5), x.file);
console.log('added under docs/:', added, '| copied: his words in DECISIONS', quoted, '+ the appendix\'s blocks', blocks, '+ the plan', plan, '=', quoted + blocks + plan);
console.log('WRITTEN LINES:', added - quoted - blocks - plan, '(stop line 1,200)');
