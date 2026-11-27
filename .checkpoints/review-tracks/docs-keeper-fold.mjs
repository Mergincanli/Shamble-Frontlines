// docs-keeper (review-tracks docs pass): folds ROADMAP's two oldest "Recent work" entries (change-set 3a and ci-wasm)
// into "Build history", byte for byte, after the change-set 2 entry, oldest first, so "Recent work" keeps 12 entries.
// Run once from the worktree root:  node .checkpoints/review-tracks/docs-keeper-fold.mjs
import { readFileSync, writeFileSync } from 'node:fs';
const F = 'docs/ROADMAP.md';
let t = readFileSync(F, 'utf8');
const rw = t.indexOf('\n## Recent work\n');
if (rw < 0) throw new Error('no Recent work');
const starts = ['**Change-set 3a of the build plan, Rules and boundaries [2026-11-24, MERGED]**', '**CI builds the WebAssembly and runs the tests that need it [2026-11-19 to 11-24, MERGED]**'];
const a = t.indexOf(`\n${starts[0]}`, rw), b = t.indexOf(`\n${starts[1]}`, rw);
if (a < 0 || b < 0 || b < a) throw new Error('entries not found in order');
const tail = t.slice(a + 1); // 3a's entry, a blank line, ci-wasm's entry to the end of the file
const e3a = t.slice(a + 1, b).replace(/\n+$/, ''), ewasm = t.slice(b + 1).replace(/\n+$/, '');
if (t.slice(b + 1).includes('\n## ')) throw new Error('ci-wasm is not the last entry');
t = t.slice(0, a + 1).replace(/\n+$/, '\n');
const anchor = 'main (gate run 934 with smoke, toolchain run 232).\n';
const at = t.indexOf(anchor);
if (at < 0 || t.indexOf(anchor, at + 1) >= 0 || at > rw) throw new Error('change-set 2 anchor not found once in Build history');
const end = at + anchor.length;
t = t.slice(0, end) + `\n${ewasm}\n\n${e3a}\n` + t.slice(end);
writeFileSync(F, t);
const n = (t.slice(t.indexOf('\n## Recent work\n')).match(/\n\*\*/g) || []).length;
console.log(`folded 2 entries (${tail.length} bytes); Recent work now holds ${n} entries`);
