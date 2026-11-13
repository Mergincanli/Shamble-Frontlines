// R-STATE round 2: which committed versions of the reviewers' scratch files hold runs of his words (A1, A2, A3)?
// For each commit and file, counts maximal runs of at least MIN characters that also occur in his words, outside
// lines that are quoted elsewhere as pointers. Prints counts and lengths only, never the text. Reads only.
//   node .checkpoints/foundation-2-3-docs/r2-frag.mjs <min> <commit:path> ...
import { execFileSync } from 'node:child_process';
import { messageText, ITEMS, go48 } from './words.mjs';

const MIN = Number(process.argv[2]);
const his = [messageText(ITEMS[0].uuid), messageText(ITEMS[1].uuid), go48()].join('\n');
const norm = (s) => s.replace(/\s+/g, ' ');
const H = norm(his);

function runs(text) {
  const t = norm(text);
  const out = [];
  let i = 0;
  while (i + MIN <= t.length) {
    if (H.includes(t.slice(i, i + MIN))) {
      let j = i + MIN;
      while (j < t.length && H.includes(t.slice(i, j + 1))) j++;
      out.push(j - i);
      i = j;
    } else i++;
  }
  return out;
}

for (const spec of process.argv.slice(3)) {
  let text;
  try { text = execFileSync('git', ['show', spec], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }); } catch { console.log(spec, 'absent'); continue; }
  const r = runs(text);
  console.log(spec, `runs >= ${MIN}:`, r.length, r.length ? `longest ${Math.max(...r)}` : '');
}
