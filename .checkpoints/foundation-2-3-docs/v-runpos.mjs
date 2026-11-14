// Verifier (read-only): for given <commit>:<path> versions, each maximal run (>= 25 chars) shared with his words (A1 to
// A4, in memory only), located in the item: the item, the run's length, the sentence it falls in (index, length) and
// whether the run covers that whole sentence. Sentences: split at [.!?] + space and at line breaks. Never prints text.
//   node .checkpoints/foundation-2-3-docs/v-runpos.mjs <commit>:<path> [...]
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const items = allItems().map((it) => [it.id, it.text]);
function sentenceSpans(t) {
  const spans = []; let s = 0;
  for (let i = 0; i < t.length; i++) {
    const end = (/[.!?]/.test(t[i]) && (i + 1 === t.length || /\s/.test(t[i + 1]))) || t[i] === '\n';
    if (end) { const e = t[i] === '\n' ? i : i + 1; if (e > s) spans.push([s, e]); s = i + 1; while (s < t.length && /\s/.test(t[s])) s++; i = s - 1; }
  }
  if (s < t.length) spans.push([s, t.length]);
  return spans;
}
for (const spec of process.argv.slice(2)) {
  let file; try { file = execFileSync('git', ['show', spec], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }); } catch { console.log(`${spec}: not found`); continue; }
  const f = file.replace(/\s+/g, ' ');
  const out = [];
  for (const [id, raw] of items) {
    const spans = sentenceSpans(raw);
    // Work on the raw item, matching each candidate start greedily against the whitespace-normalised file.
    const t = raw;
    let i = 0;
    while (i < t.length) {
      let j = i, best = i;
      while (j < t.length && f.includes(t.slice(i, j + 1).replace(/\s+/g, ' '))) j++;
      best = j;
      if (best - i >= 25) {
        const si = spans.findIndex(([a, b]) => i < b && best > a);
        const whole = spans.filter(([a, b]) => a >= i && b <= best).length;
        const run = t.slice(i, best).replace(/\s+/g, ' ').trim();
        const cls = /^[\s0-9a-f()]+$/.test(run) ? 'sha' : /[a-z]+ [a-z]+ [a-z]+/.test(run.replace(/[^A-Za-z ]/g, ' ').replace(/\s+/g, ' ').toLowerCase()) && !/\//.test(run) ? 'prose' : (/\//.test(run) ? 'path-ish' : 'codes/other');
        out.push(`${id} run ${best - i} chars [${cls}]; first sentence touched #${si} (len ${spans[si][1] - spans[si][0]}); whole sentences inside the run: ${whole}`);
        i = best;
      } else i++;
    }
  }
  console.log(`== ${spec}`); for (const l of out) console.log('  ' + l);
}
