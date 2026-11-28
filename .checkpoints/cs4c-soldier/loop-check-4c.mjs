// The advisor agent's condition F-S0-5 on 4c's step 0 (`advisor-reread-4c.md`): his item 3, "Keep the old text for
// the history ... Change nothing else in the design docs", proven byte for byte. docs/LOOP_V2.md may differ from main's
// (`loop-v2-main.md` beside this script, saved by `git show dae6d107:docs/LOOP_V2.md`) only by: one sentence added to
// the header's "Updated" line that names section 14; his pointer, exactly his string, at the end of each listed line;
// and section 14 after section 13 (whose own words `section14-4c.mjs check` proves). docs/EDITOR_V2.md likewise
// against `editor-v2-main.md`, by his DESIGN-CLASH answer. From the worktree's root:
//   node .checkpoints/cs4c-soldier/loop-check-4c.mjs
import { readFileSync } from 'node:fs';

const HERE = '.checkpoints/cs4c-soldier/';
/** His pointer, word for word (his design notes of 2026-11-27, item 3). */
const POINTER = '(2026-11-27: resource points replace open fields, see §14)';
/**
 * The lines (1-based, main's numbering, which appending does not move) whose sentence or bullet speaks of the harvest
 * field in the centre or the safe field near each base, each at the end of its sentence or bullet: section 1 (15-16),
 * section 3 (32-33, 34-35), section 9 (150-151, 154, 156), section 11 (181). DESIGN-CLASH (a) would add line 52.
 */
const LOOP_LINES = [16, 33, 35, 151, 154, 156, 181];
/** DESIGN-CLASH (a) would add EDITOR_V2.md's line 45 here, with the pointer "(2026-11-27: see §14)". */
const EDITOR_LINES = [];
const HEADER_LINE = 5;
const SECTION_14 = '## 14. Resource points and repair bots';

const lines = (file) => readFileSync(file, 'utf8').split('\n');
const fail = (why) => {
  console.error(why);
  process.exit(1);
};

// The loop doc: cut section 14 off the end, then hold every other line to main's.
const main = lines(HERE + 'loop-v2-main.md');
const head = lines('docs/LOOP_V2.md');
const at = head.indexOf(SECTION_14);
if (at < 0 || head.lastIndexOf(SECTION_14) !== at) fail('docs/LOOP_V2.md: section 14 does not stand there once');
// Before section 14: main's lines, then one blank line (main ends with a line feed, so its last element is "").
const before = head.slice(0, at);
if (before.length !== main.length) fail(`docs/LOOP_V2.md: ${before.length} lines before section 14, main has ${main.length - 1} and a blank`);
for (let i = 0; i < main.length; i += 1) {
  const n = i + 1;
  const was = main[i];
  const is = before[i];
  if (n === HEADER_LINE) {
    if (!is.startsWith(`${was} `) || !is.slice(was.length + 1).includes('§14')) fail(`docs/LOOP_V2.md:${n}: the header's line is not main's with one sentence more that names §14`);
    const added = is.slice(was.length + 1);
    if ((added.match(/\. |\.$/g) ?? []).length !== 1) fail(`docs/LOOP_V2.md:${n}: more than one sentence added: "${added}"`);
  } else if (LOOP_LINES.includes(n)) {
    if (is !== `${was} ${POINTER}`) fail(`docs/LOOP_V2.md:${n}: not main's line with his pointer at its end`);
  } else if (is !== was) fail(`docs/LOOP_V2.md:${n}: changed from main's`);
}
console.log(`docs/LOOP_V2.md: main's ${main.length - 1} lines unchanged but the header's one sentence and his pointer at lines ${LOOP_LINES.join(', ')}; section 14 from line ${at + 1}`);

// The editor doc: main's lines, but his pointer at the end of the listed lines.
const editorMain = lines(HERE + 'editor-v2-main.md');
const editorHead = lines('docs/EDITOR_V2.md');
if (editorHead.length !== editorMain.length) fail('docs/EDITOR_V2.md: its line count changed');
for (let i = 0; i < editorMain.length; i += 1) {
  const n = i + 1;
  const expected = EDITOR_LINES.includes(n) ? `${editorMain[i]} (2026-11-27: see §14)` : editorMain[i];
  if (editorHead[i] !== expected) fail(`docs/EDITOR_V2.md:${n}: changed from main's`);
}
console.log(`docs/EDITOR_V2.md: main's ${editorMain.length - 1} lines${EDITOR_LINES.length === 0 ? ' unchanged' : `, his pointer at lines ${EDITOR_LINES.join(', ')}`}`);
