// docs-keeper (review-tracks docs pass): the change-set's own copied-words check on the worktree's docs/DECISIONS.md
// against origin/main's, saved files read with git show at their named commit. Run from the worktree root:
//   node .checkpoints/review-tracks/docs-keeper-words.mjs
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { wordsCheck, otherAdded } from '../../.claude/scripts/track.mjs';
const git = (a) => execFileSync('git', a, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
const head = readFileSync('docs/DECISIONS.md', 'utf8');
const base = git(['show', 'origin/main:docs/DECISIONS.md']);
const read = (c, p) => { try { return git(['show', `${c}:${p}`]); } catch { return null; } };
const out = wordsCheck(head, base, read);
for (const w of out) console.log(`${w.entry.padEnd(36)} ${w.floor ? 'FLOOR' : 'ok   '} ${w.result}${w.sha ? ` (sha256 ${w.sha})` : ''}`);
const want = ['5ab2d0b4-596a-45ba-a9ca-8c157a8028ad', '442b9871-8e0d-40f4-941c-52d9214d20a9', '376dac3c-777d-470b-879e-0781daaa2b7c', '17f11dda-16e0-434d-a9df-b390d13e596c', '2d747465-b4f4-49e9-9c26-e0bd2148196c'];
const ids = out.map((w) => w.entry);
const fail = [];
if (out.length !== 5) fail.push(`${out.length} results, want 5`);
if (JSON.stringify(ids) !== JSON.stringify(want)) fail.push(`ids ${JSON.stringify(ids)}`);
if (out.some((w) => w.result !== 'ok' || w.floor)) fail.push('a result is not ok');
const diff = git(['diff', '--unified=0', 'origin/main', '--', 'docs/DECISIONS.md']);
const added = diff.split('\n').filter((l) => l.startsWith('+') && !l.startsWith('+++')).join('\n');
const removed = diff.split('\n').filter((l) => l.startsWith('-') && !l.startsWith('---')).length;
if (removed) fail.push(`${removed} lines removed`);
if (otherAdded(head, base, added)) fail.push('other added text outside the new entries');
console.log(fail.length ? `FAIL: ${fail.join('; ')}` : `PASS: 5 new entries, all ok, in order; 0 lines removed; no other added text`);
process.exitCode = fail.length ? 1 : 0;
