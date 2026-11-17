// Change-set 0: builds docs/reports/2026-11-17-build-plan-slice-1.md = "since the plan" (written for this change-set)
// + the plan as approved, copied from commit 68addf9 and not edited. Run from the repository root:
//   node .checkpoints/cs0-model-rules/report-build.mjs            (writes, then checks)
//   node .checkpoints/cs0-model-rules/report-build.mjs --check    (checks only)
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const C = '.checkpoints/cs0-model-rules/';
const OUT = 'docs/reports/2026-11-17-build-plan-slice-1.md';
const plan = execFileSync('git', ['show', '68addf9:.checkpoints/build-plan-slice-1/plan.md'], { maxBuffer: 1 << 26 }).toString('utf8');
const sha = (s) => createHash('sha256').update(s).digest('hex');
if (!process.argv.includes('--check')) {
  const top = readFileSync(C + 'since-the-plan.md', 'utf8');
  const found = readFileSync(C + 'found.md', 'utf8').replace(/\n$/, '');
  if (top.split('FOUND_SECTION').length !== 2) throw new Error('FOUND_SECTION must stand once');
  writeFileSync(OUT, top.replace('FOUND_SECTION', found) + plan);
}
const t = readFileSync(OUT, 'utf8');
const MARK = '## The plan as approved (revision 2, commit 68addf9, copied by script)\n\n';
const i = t.indexOf(MARK); const tail = i < 0 ? '' : t.slice(i + MARK.length);
console.log(OUT, 'lines:', t.split('\n').length - 1, '| the plan below the mark equals commit 68addf9:', tail === plan ? 'yes' : 'NO',
  '| sha256', sha(plan).slice(0, 16), Buffer.byteLength(plan), 'bytes');
process.exit(tail === plan ? 0 : 1);
