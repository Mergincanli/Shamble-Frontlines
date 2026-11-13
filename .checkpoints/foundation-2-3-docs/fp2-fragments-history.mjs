// Round 2's fix pass: every pushed version of round 2's notes files, checked for runs of 40 or more characters of his
// words (A1 to A4). Prints commit, file and counts only.   node .checkpoints/foundation-2-3-docs/fp2-fragments-history.mjs
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
const files = ['r2-words.md', 'r2-state.md', 'r2-skeptic-R-WORDS-RW2-F1.md', 'r2-frag.mjs', 'r2-kit.mjs', 'r2w-kitbytes.py'].map((f) => `.checkpoints/foundation-2-3-docs/${f}`);
for (const f of files) {
  const shas = execFileSync('git', ['log', '--format=%h', '--', f], { encoding: 'utf8' }).split('\n').filter(Boolean);
  for (const s of shas) {
    let t; try { t = execFileSync('git', ['show', `${s}:${f}`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).replace(/\s+/g, ' '); } catch { continue; }
    const hits = [];
    for (const [id, w] of texts) { let n = 0; for (let i = 0; i + 40 <= w.length; i += 20) if (t.includes(w.slice(i, i + 40))) n++; if (n) hits.push(`${id}: ${n}`); }
    if (hits.length) console.log(`${s} ${f.split('/').pop()}: ${hits.join(', ')}`);
  }
}
console.log('done');
