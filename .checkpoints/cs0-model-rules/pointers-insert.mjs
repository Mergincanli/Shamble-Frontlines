// Change-set 0: the dated pointers (the advisor agent's findings 9 and 10 on the plan). Run after decisions-insert.mjs,
// from the repository root:
//   node .checkpoints/cs0-model-rules/pointers-insert.mjs            (writes, then checks)
//   node .checkpoints/cs0-model-rules/pointers-insert.mjs --check    (checks only)
// Every anchor must be found exactly as often as stated, or nothing is written. Every piece of his words a pointer
// quotes is checked against the message files that were copied by script from the transcript.
import { readFileSync, writeFileSync } from 'node:fs';
const N = '.checkpoints/build-plan-slice-1/';
const his = ['mustafa-2026-11-14-task.md', 'mustafa-2026-11-17-go.md'].map((f) => readFileSync(N + f, 'utf8')).join('\n');
const DEC = 'docs/DECISIONS.md', SP = 'docs/SPIKE_PLAN.md', A2 = 'docs/adr/0002-code-structure-build-toolchain.md';
const TAG = '2026-11-17, change-set 0 of the build plan';
const GAME = 'the entry "Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the game is built now"';
const GO = 'the entry "Taken by Mustafa, 2026-11-17, 19:10:44 UTC: his go on the build plan"';
// his words, each quoted in a pointer below, each checked against the message files
const Q = {
  steps: 'Foundation steps 2.4 and 2.5 and Slice 1 may be planned now and built after my go on the plan, on the decision drafts as proposed. They do not wait for the tests that docs/SPIKE_PLAN.md places before Slice 1 (the sim core spike, the 60 Hz run, the boundary spike, the look proof test, the content and ability spike and the render budget bench), nor for the end of step 2.3.',
  rest: 'Slice 3, online, still waits for the netcode proof test\'s result. Step 2.6, the previews, follows the preview hosting check as planned.',
  marks: 'No pass mark and no open question of theirs is dropped or loosened: each is measured on the real game, at the first build that can show it, and a draft is accepted only when its marks are met.',
  fable: 'On my personal account, Fable 5.2 is the main model of every chat that writes game or engine code, and of every builder agent that chat starts to write that code.',
  second: 'A second builder chat has Opus 5.5 as its main model, with Fable 5.2 answering its advisor tool, and it also writes game and engine code.',
  only: 'for the second builder chat only',
  gate: 'The guard, the gate and CI work stays with Opus 5.5.',
  ask: 'You ask me for each gate and CI edit with its exact text',
};
const wrap = (text, first, rest) => { // greedy wrap at 120 columns
  const out = []; let line = first; let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) { out.push(line); line = rest + w; }
    else { line += (empty ? '' : ' ') + w; empty = false; }
  }
  out.push(line); return out.join('\n') + '\n';
};
const stepsText = `_Pointer (${TAG}): for foundation steps 2.4 and 2.5 and for Slice 1, his words of 2026-11-13 quoted above are changed by his decision of 2026-11-14 (\`docs/DECISIONS.md\`, ${GAME}): "${Q.steps}" His go on the plan for them: ${GO}. The marks stay, his words: "${Q.marks}" Unchanged, his words: "${Q.rest}"_`;
const edits = [
  { file: SP, n: 2, anchor: 'Every gate check that needs a result joins later, when that result is in, with its pass marks unchanged. Steps 2.4 to\n2.6 still wait for their results."_\n', add: wrap(stepsText, '', '') },
  { file: SP, n: 1, anchor: '\n  Steps 2.4 to 2.6 still wait for their results."_\n', add: wrap(stepsText, '  ', '  ') },
  { file: A2, n: 1, anchor: '\n   Steps 2.4 to 2.6 still wait for their results."_\n', add: wrap(stepsText, '   ', '   ') },
  { file: DEC, n: 1, anchor: '    heavy spike at a time"; "The preview hosting check may run alongside any of them."\n',
    add: wrap(`_Pointer, ${TAG} (a pointer, not his words): for foundation steps 2.4 and 2.5 and Slice 1, changed by his decision of 2026-11-14 (${GAME}, above): "${Q.steps}" The marks stay: "${Q.marks}"_`, '    - ', '      ') },
  { file: DEC, n: 1, anchor: '    > - I allow the rule change in the kit for this. Step 2.3 changes CLAUDE.md\'s "Models and effort" section and the agent files to match, each edit asked of me in chat with its exact text. List it as a finding for 2.3. Do not edit the kit for it in this PR.\n',
    before: '  - The kit\'s next change after the freeze',
    add: wrap(`_Pointer, ${TAG} (a pointer, not his words): his words of 2026-11-17 (${GO}, under "Decisions, newest first") change the words this kit change carries, in two places. His CHANGE 2: "${Q.second}" His answer 3 (GATE-FOLLOW): "${Q.ask}"; for those edits he sets "${Q.gate}" aside. The kit change, when it is made, carries both. It is not made by change-set 0._`, '    - ', '      ') },
  { file: DEC, n: 1, anchor: '  change item 3 allows waits in "Queued", above, for step 2.3. Copied by script from\n  `docs/reports/2026-11-12-foundation-2-2.md`, Appendix A9, and checked byte for byte against it:\n',
    replace: '  change item 3 allows waits in "Queued", above, for step 2.3.\n' +
      wrap(`Pointer, ${TAG} (not his words): two lines of its item 3 are changed by his words of 2026-11-17 (${GO}, at the top of this list). The line "${Q.fable}" is changed "${Q.only}" (its CHANGE 2). The line "${Q.gate}" is set aside for the gate and CI edits the first builder chat asks of him, each with its exact text (its answer 3, GATE-FOLLOW).`, '  ', '  ') +
      '  Copied by script from `docs/reports/2026-11-12-foundation-2-2.md`, Appendix A9, and checked byte for byte against it:\n' },
];
const count = (t, s) => t.split(s).length - 1;
const flat = (s) => s.replace(/\s+/g, ' ');
let ok = true;
for (const [k, q] of Object.entries(Q)) if (!flat(his).includes(flat(q))) { console.log('NOT HIS WORDS:', k); ok = false; }
if (!ok) process.exit(1);
const texts = {};
for (const f of [DEC, SP, A2]) texts[f] = readFileSync(f, 'utf8');
if (!process.argv.includes('--check')) {
  if (!texts[DEC].includes('2026-11-17, 19:10:44 UTC: his go on the build plan')) throw new Error('run decisions-insert.mjs first');
  for (const f of [DEC, SP, A2]) if (texts[f].includes(TAG + ')') && f !== DEC) throw new Error(`already inserted in ${f}`);
  if (texts[DEC].includes(`Pointer, ${TAG}`)) throw new Error('already inserted in DECISIONS');
  for (const e of edits) { const n = count(texts[e.file], e.anchor); if (n !== e.n) throw new Error(`anchor found ${n} times, wanted ${e.n}: ${e.anchor.slice(0, 70)}`); }
  for (const e of edits) {
    if (e.before && !texts[e.file].includes(e.anchor + e.before)) throw new Error('the line after the anchor is not the one expected');
    texts[e.file] = texts[e.file].split(e.anchor).join(e.replace ?? e.anchor + e.add);
  }
  for (const f of [DEC, SP, A2]) writeFileSync(f, texts[f]);
}
const want = { [SP]: 3, [A2]: 1, [DEC]: 3 + 1 }; // DECISIONS: three pointers here, one from decisions-insert.mjs (SLICE-ORDER's condition 3)
for (const f of [DEC, SP, A2]) {
  const t = readFileSync(f, 'utf8'); const n = count(t, TAG);
  const long = t.split('\n').filter((l) => l.includes('change-set 0 of the build plan') && l.length > 120).length;
  console.log(f, 'pointers:', n, n === want[f] ? 'ok' : `WANTED ${want[f]}`, 'lines over 120:', long);
  if (n !== want[f] || long) ok = false;
}
process.exit(ok ? 0 : 1);
