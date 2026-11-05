// verifier pass 3: ruling's limit and follow-up texts vs STATUS.md, whitespace folded
import { readFileSync } from 'node:fs';
const [rf, sf] = process.argv.slice(2);
const fold = (s) => s.replace(/\s+/g, ' ').trim();
const R = fold(readFileSync(rf, 'utf8')), S = fold(readFileSync(sf, 'utf8'));
const pick = (src, start, end) => { const i = src.indexOf(start); if (i < 0) return null; const j = src.indexOf(end, i + start.length); return src.slice(i, j < 0 ? undefined : j); };
const want = [
  'The night switch check does not fold case either; the night shift runs only on Linux.',
  'A new clone or worktree made in a folder that is not a kit folder (for example `git clone <url> tools/x`) brings its own kit files, such as a nested CLAUDE.md, with no ask.',
];
for (const w of want) console.log(R.includes(fold(w)) ? 'ruling-has' : 'ruling-MISSING', S.includes(fold(w)) ? 'status-has' : 'status-MISSING', w.slice(0, 50));
for (const [a, b] of [['L16, deliberately', '- **L17'], ['L17, folder', '- **L18'], ['L18, parallel', '### Follow-ups']]) {
  let r = pick(R, a.replace(',', ',').replace(/^/, ''), b); if (r) r = r.replace(/\*\*/g, '').replace(/ -$/, '').trim();
  // ruling writes **L16, deliberately disguised shell text:** ...
  const key = a.split(',')[0];
  const rr = fold((R.match(new RegExp(`\\*\\*${key}, [^*]+:\\*\\* (.*?)(?= - \\*\\*L|### |$)`)) || [])[1] || '');
  const head = (R.match(new RegExp(`\\*\\*(${key}, [^*]+):\\*\\*`)) || [])[1];
  const line = fold(`${head}: ${rr}`);
  console.log(S.includes(line) ? 'MATCH' : 'DIFF', key, line.slice(0, 60));
}
const a4 = fold((R.match(/- (A-4: .*?next kit change-set \(SD-2's fix closes today's path to it\)\.)/) || [])[1] || '');
console.log(a4 && S.includes(a4) ? 'MATCH A-4' : 'DIFF A-4', a4.slice(0, 40));
