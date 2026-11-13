// R-STATE round 2: does STATUS's kit quote run to the end of his kit paragraph in A2? Prints positions and lengths
// only, never his words. Reads only.
//   node .checkpoints/foundation-2-3-docs/r2-kit.mjs
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { kitWords } from './pointers.mjs';
import { messageText, ITEMS } from './words.mjs';

const a2 = messageText(ITEMS[1].uuid);
const k = kitWords();
const i = a2.indexOf(k);
const lineStart = a2.lastIndexOf('\n', i) + 1;
let lineEnd = a2.indexOf('\n', i);
if (lineEnd < 0) lineEnd = a2.length;
const line = a2.slice(lineStart, lineEnd);
console.log('kit cut length', k.length, 'found once in A2:', a2.split(k).length === 2);
console.log('its A2 line: chars before the cut', i - lineStart, '; chars after the cut', lineEnd - (i + k.length));
console.log('the line before the cut starts with a label of', i - lineStart, 'chars; next line is blank:', a2.slice(lineEnd + 1, lineEnd + 2) === '\n' || lineEnd === a2.length);
console.log('the cut ends the paragraph (nothing after it on its line):', lineEnd === i + k.length);
for (const ref of ['554faceded1948a39eb635c42c4aa12ac9edf496']) {
  const st = execFileSync('git', ['show', `${ref}:docs/STATUS.md`], { encoding: 'utf8' });
  const flat = st.replace(/\n */g, ' ');
  console.log(ref.slice(0, 7), 'STATUS holds the quoted cut (with quote marks), whitespace-joined:', flat.includes(`"${k}"`));
  // byte-exact without joining: the quote's words, in order, with only line breaks + indentation between them
  const words = k.split(' ');
  const re = new RegExp('"' + words.map((w) => w.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('(?: |\\n +)') + '"');
  console.log(ref.slice(0, 7), 'STATUS holds it with only line breaks between words:', re.test(st));
}
const decisions = readFileSync('docs/DECISIONS.md', 'utf8');
console.log('DECISIONS.md (worktree) holds the cut inside E-TASK:', decisions.includes(k));
