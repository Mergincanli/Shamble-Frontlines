// 4c's step 0 (the plan sheet's section C): places his section 14 and the dated pointers in the two design docs, by
// script, for docs-keeper to run (his design notes of 2026-11-27, item 2: "docs-keeper places that text and writes no
// design of its own"). No word of his is typed here: section 14 is `section14-mended.md` beside this script (his item
// 8, the paste's breaks mended under his PASTE-BREAKS (a) by `section14-4c.mjs mend`, 517 words and signs his own, in
// order), and every pointer is read from his saved messages by the same rule as `loop-check-4c.mjs`. The header's one
// sentence (LOOP_V2.md line 5) and the register's dated line (docs/adr/README.md) are docs-keeper's own words, added
// after this script, and are not placed here. From the worktree's root, once:
//   node .checkpoints/cs4c-soldier/place-4c.mjs
// It refuses unless both design docs still equal main's (`loop-v2-main.md`, `editor-v2-main.md` beside it), so it never
// places twice. Then run `section14-4c.mjs check` and `loop-check-4c.mjs`.
//   node .checkpoints/cs4c-soldier/place-4c.mjs fix-16
// The one correction after the placement (security-auditor's note on step S's review, 2026-11-28): line 16 holds the end
// of section 1's sentence on the centre field ("centre.") and then two more sentences, the second running on to line
// 17, so his pointer moves from the line's end to right after "centre." (the advisor agent's F-S0-2: no sentence cut).
// It refuses unless line 16 is exactly main's with the pointer at its end.
import { readFileSync, writeFileSync } from 'node:fs';

const HERE = '.checkpoints/cs4c-soldier/';
const LOOP = 'docs/LOOP_V2.md';
const EDITOR = 'docs/EDITOR_V2.md';
const POINTER = '(2026-11-27: resource points replace open fields, see §14)';
const ANSWERS = '.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md';
const QUESTIONS = HERE + 'question-2026-11-28-before-4c.md';
function between(file, lineStart, before, after) {
  const line = readFileSync(file, 'utf8').split('\n').find((l) => l.startsWith(lineStart));
  if (line === undefined) throw new Error(`${file}: no line that starts with ${lineStart}`);
  const from = line.indexOf(before);
  const to = line.indexOf(after, from + before.length);
  if (from < 0 || to < 0) throw new Error(`${file}: the pointer is not where it should stand`);
  return line.slice(from + before.length, to);
}
const SILO_POINTER = between(ANSWERS, '9. DESIGN-CLASH: (a) at both lines.', 'the pointer reads: "', '"');
const EDITOR_POINTER = between(QUESTIONS, '- **Options:** (a) the same dated pointer as his item 3 at both lines, "', 'at both lines, "', '"');
// The same lines as loop-check-4c.mjs (main's numbering): his item 3's pointer at the end of each sentence or bullet on
// the open fields, and his own DESIGN-CLASH pointer at the end of line 52; the editor doc's at the end of its bullet.
const LOOP_POINTERS = new Map([[16, POINTER], [33, POINTER], [35, POINTER], [52, SILO_POINTER], [151, POINTER], [154, POINTER], [156, POINTER], [181, POINTER]]);
const EDITOR_LINES = [47];

const loopMain = readFileSync(HERE + 'loop-v2-main.md', 'utf8');
const editorMain = readFileSync(HERE + 'editor-v2-main.md', 'utf8');
if (process.argv[2] === 'fix-16') {
  const head = readFileSync(LOOP, 'utf8').split('\n');
  const was = loopMain.split('\n')[15];
  if (head[15] !== `${was} ${POINTER}`) throw new Error(`${LOOP}:16 is not main's line with the pointer at its end`);
  if (was.indexOf('centre.') < 0 || was.indexOf('centre.') !== was.lastIndexOf('centre.')) throw new Error(`main's line 16 does not hold "centre." once`);
  head[15] = was.replace('centre.', `centre. ${POINTER}`);
  writeFileSync(LOOP, head.join('\n'));
  console.log(`${LOOP}:16: his pointer moved to the end of its sentence, right after "centre."`);
  process.exit(0);
}
if (readFileSync(LOOP, 'utf8') !== loopMain) throw new Error(`${LOOP} is not main's: placed already, or changed`);
if (readFileSync(EDITOR, 'utf8') !== editorMain) throw new Error(`${EDITOR} is not main's: placed already, or changed`);
if (!loopMain.endsWith('\n') || !editorMain.endsWith('\n')) throw new Error('a design doc does not end with a line feed');

const section14 = readFileSync(HERE + 'section14-mended.md', 'utf8').replace(/\n$/, '');
const loopLines = loopMain.split('\n');
for (const [n, pointer] of LOOP_POINTERS) {
  // Line 16: right after its sentence's end, "centre." (see fix-16 above); every other line: at its end.
  loopLines[n - 1] = n === 16 ? loopLines[n - 1].replace('centre.', `centre. ${pointer}`) : `${loopLines[n - 1]} ${pointer}`;
}
// Main ends with a line feed, so the joined text does too; then one blank line, section 14, and its own line feed.
writeFileSync(LOOP, `${loopLines.join('\n')}\n${section14}\n`);
const editorLines = editorMain.split('\n');
for (const n of EDITOR_LINES) editorLines[n - 1] = `${editorLines[n - 1]} ${EDITOR_POINTER}`;
writeFileSync(EDITOR, editorLines.join('\n'));
console.log(`${LOOP}: his pointer at lines ${[...LOOP_POINTERS.keys()].join(', ')} (line 52 his own DESIGN-CLASH words), section 14 after section 13; ${EDITOR}: the pointer at line ${EDITOR_LINES.join(', ')}. The header's sentence and the register's line are still to be written.`);
