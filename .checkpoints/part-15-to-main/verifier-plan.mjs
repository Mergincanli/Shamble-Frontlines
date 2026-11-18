// Verifier: the plan below the build-plan report's mark equals commit 68addf9's plan.md (byte for byte), and the
// report's lines above the mark changed only in the two pointer spots. Prints verdicts only.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const rep = readFileSync('docs/reports/2026-11-17-build-plan-slice-1.md', 'utf8');
const base = execFileSync('git', ['show', 'd7dad97:docs/reports/2026-11-17-build-plan-slice-1.md']).toString('utf8');
const plan = execFileSync('git', ['show', '68addf9:.checkpoints/build-plan-slice-1/plan.md']).toString('utf8');
const MARK = '## The plan as approved (revision 2, commit 68addf9, copied by script)\n';
for (const [k, t] of [['head', rep], ['base', base]]) {
  const i = t.indexOf(MARK); const below = t.slice(i + MARK.length);
  const j = below.indexOf(plan);
  console.log(k, 'mark count', t.split(MARK).length - 1, '| plan found below mark at offset', j, '| text below mark before plan:', JSON.stringify(below.slice(0, j)), '| after plan:', JSON.stringify(below.slice(j + plan.length).slice(0, 80)), '| bytes after', below.length - j - plan.length);
}
const below = (t) => t.slice(t.indexOf(MARK));
console.log('below-mark text equal head vs base:', below(rep) === below(base));
