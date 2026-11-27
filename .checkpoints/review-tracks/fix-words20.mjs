// Fix-pass evidence: the copied-words check on the 20 newest entries of origin/main's docs/DECISIONS.md (TE-B3, plan 3f),
// each checked as if newly added, saved files read with git show at their named commit. Run from the repository root:
//   node .checkpoints/review-tracks/fix-words20.mjs
import { execFileSync } from 'node:child_process';
import { wordsCheck } from '../../.gate/review-tracks/build/.claude/scripts/track.mjs';
const git = (a) => execFileSync('git', a, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
const text = git(['show', 'origin/main:docs/DECISIONS.md']);
const entries = text.split(/\n(?=- \*\*)/).filter((e) => e.startsWith('- **Taken by Mustafa'));
const read = (c, p) => { try { return git(['show', `${c}:${p}`]); } catch { return null; } };
const tally = {};
for (const e of entries.slice(0, 20)) {
  const [w] = wordsCheck(e, '', read);
  tally[w.result] = (tally[w.result] || 0) + 1;
  console.log(`${w.entry.padEnd(36)} ${w.floor ? 'FLOOR' : 'ok   '} ${w.result}`);
}
console.log(`\n${entries.length} entries in the record; the first 20 (newest first) checked:`, JSON.stringify(tally));
