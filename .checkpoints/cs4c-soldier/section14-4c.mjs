// LOOP_V2's section 14, from item 8 of his design notes of 2026-11-27 (`.checkpoints/cs4-walk/mustafa-2026-11-27-
// design-notes.md`, saved by script), for 4c's first docs commit (the sheet's step 0). No word of his is typed here:
// `section14-as-pasted.md` beside this script was cut from the saved message by `chat-words.mjs part` (its lines 61
// to 128 of 130). From the worktree's root:
//   node .checkpoints/cs4c-soldier/section14-4c.mjs mend
//     Only under his PASTE-BREAKS (a). Mends the paste's two breaks and nothing else: a line that is "- " is joined
//     to the line after it, and a line that is "  " is joined to the line after it. Writes the text to place,
//     `section14-mended.md` beside this script, and refuses any other change: the two texts' words and signs, white
//     space aside, are equal and in the same order, and no line "- " or "  " is left.
//   node .checkpoints/cs4c-soldier/section14-4c.mjs check [--as-pasted]
//     Reads docs/LOOP_V2.md: the text to place (the mended one, or the as-pasted one under PASTE-BREAKS (b)) stands
//     there once, whole, as the file's last section, after section 13 and one blank line; and its words and signs are
//     the message's item 8, in order.
import { readFileSync, writeFileSync } from 'node:fs';

const HERE = '.checkpoints/cs4c-soldier/';
const PASTED = HERE + 'section14-as-pasted.md';
const MENDED = HERE + 'section14-mended.md';
const LOOP = 'docs/LOOP_V2.md';
const FIRST = '## 14. Resource points and repair bots';

/** The cut text: what follows the saved file's "---" line, without its last line feed. */
function pasted() {
  const all = readFileSync(PASTED, 'utf8');
  const at = all.indexOf('\n---\n');
  if (at < 0) throw new Error(`${PASTED}: no "---" line`);
  const text = all.slice(at + 5).replace(/\n$/, '');
  if (!text.startsWith(FIRST + '\n')) throw new Error(`${PASTED}: it does not begin with "${FIRST}"`);
  return text;
}
/** Words and signs in order, white space aside. */
const tokens = (text) => text.split(/\s+/).filter((token) => token !== '');
function sameWords(a, b, what) {
  const x = tokens(a);
  const y = tokens(b);
  const n = Math.max(x.length, y.length);
  for (let i = 0; i < n; i += 1) {
    if (x[i] !== y[i]) throw new Error(`${what}: word ${i + 1} differs: "${x[i]}" against "${y[i]}"`);
  }
  return x.length;
}
function mend(text) {
  const lines = text.split('\n');
  const out = [];
  let joined = 0;
  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (line === '- ' || line === '  ') {
      const next = lines[i + 1];
      if (next === undefined || next === '' || next === '- ' || next === '  ') throw new Error(`line ${i + 1}: a break with nothing to join`);
      out.push(line + next);
      i += 1;
      joined += 1;
    } else out.push(line);
  }
  return { text: out.join('\n'), joined };
}

const [mode, flag] = process.argv.slice(2);
if (mode === 'mend') {
  const before = pasted();
  const { text, joined } = mend(before);
  const words = sameWords(before, text, 'mended against pasted');
  if (text.split('\n').some((line) => line === '- ' || line === '  ')) throw new Error('a break is left');
  writeFileSync(MENDED, text + '\n');
  console.log(`${MENDED}: ${joined} breaks joined, ${text.split('\n').length} lines, ${words} words and signs, the same as pasted and in order`);
} else if (mode === 'check') {
  const place = flag === '--as-pasted' ? pasted() : readFileSync(MENDED, 'utf8').replace(/\n$/, '');
  const loop = readFileSync(LOOP, 'utf8');
  const at = loop.indexOf(place);
  if (at < 0 || loop.indexOf(place, at + 1) >= 0) throw new Error(`${LOOP}: the text to place does not stand there exactly once`);
  if (loop.slice(at + place.length) !== '\n') throw new Error(`${LOOP}: the section is not the file's last`);
  const before = loop.slice(0, at);
  if (!before.endsWith('\n\n') || before.endsWith('\n\n\n')) throw new Error(`${LOOP}: the section does not follow one blank line`);
  const last = before.lastIndexOf('\n## ');
  if (last < 0 || !before.slice(last + 1).startsWith('## 13. ')) throw new Error(`${LOOP}: the section before it is not section 13`);
  const words = sameWords(pasted(), place, `${LOOP}'s section 14 against his item 8`);
  console.log(`${LOOP}: section 14 stands once, last, after section 13; ${words} words and signs, his item 8's, in order`);
} else throw new Error('usage: section14-4c.mjs mend | check [--as-pasted]');
