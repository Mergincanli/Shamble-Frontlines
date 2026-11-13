// Round 2's fix pass: which of round 2's notes files (and the session's own fix-pass-2 notes) hold a run of 40 or more
// characters of his words (A1 to A4), as pushed to the branch. Prints file names and counts only, never the text.
//   node .checkpoints/foundation-2-3-docs/fp2-fragments.mjs
import { readFileSync, readdirSync } from 'node:fs';
import { allItems } from './words.mjs';
const dir = '.checkpoints/foundation-2-3-docs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
const files = readdirSync(dir).filter((f) => /^(r2|sk-f|r2w|fp2|fixpass2)/.test(f) && f !== 'fp2-fragments.mjs');
for (const f of files) {
  const t = readFileSync(`${dir}/${f}`, 'utf8').replace(/\s+/g, ' ');
  const hits = [];
  for (const [id, w] of texts) {
    let n = 0;
    for (let i = 0; i + 40 <= w.length; i += 20) if (t.includes(w.slice(i, i + 40))) n++;
    if (n) hits.push(`${id}: ${n} windows`);
  }
  console.log(`${f}: ${hits.length ? hits.join(', ') : 'none'}`);
}
