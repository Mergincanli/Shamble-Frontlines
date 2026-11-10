// Advisor's own run of the guard's marks for 2.2's planned paths (imports main's guard from this checkout).
import { protectedRel, mergeAllowed } from '../../.claude/hooks/routine-guard.mjs';
const paths = process.argv.slice(2);
for (const p of paths) console.log(`${protectedRel(p) ? 'P' : '-'} ${mergeAllowed(p) ? 'M' : '-'} ${p}`);
