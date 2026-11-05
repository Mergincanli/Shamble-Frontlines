// step 9 E1 on the base: copy the head's routine-guard.test.mjs into the base worktree (.gate/kit-v3-3/base, at
// 22f2480, the guard without switchTargetCheck), so the new cases run against the old rules. Usage:
//   node step9-e1.mjs <a3 folder>
import { copyFileSync } from 'node:fs';
import { join } from 'node:path';
const a3 = process.argv[2];
copyFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), join(a3, '.gate/kit-v3-3/base/.claude/hooks/routine-guard.test.mjs'));
console.log('copied');
