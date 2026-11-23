// The docs pass of the CI-WASM pull request: writes Mustafa's chat messages since his go on #61 into docs/DECISIONS.md,
// newest first, and one dated pointer under the entry of 2026-11-19, 13:36:37 UTC. His words are never retyped: they
// come from the files saved by script from the session's transcript (words.mjs), each with its byte count and sha256.
// Since 2026-11-23 the project thread that took the pull request over adds its entries on top: his messages there are
// saved by thread-words.mjs; the thread's own message and cards are shown under his words as the thread's text.
// The file is always rebuilt from BEFORE (main's copy), so a second run writes the same bytes. From the repository root:
//   node .checkpoints/ci-wasm/decisions-3.mjs [--check] [--out <file>]
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
// The question VISION-NOTE and its options, copied by script from the chat's message of 14:29:51 UTC (never retyped).
import { OPTION_A, OPTION_B, QUESTION } from './vision-note.mjs';

const DEC = 'docs/DECISIONS.md';
const at = process.argv.indexOf('--out');
const OUT = at === -1 ? DEC : process.argv[at + 1];
const NOTES = '.checkpoints/ci-wasm/';
const BEFORE = 'f5030a33185b1ea1b15329174e82823f0215f993';
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const RECORD = 'docs/reports/2026-11-19-cs2-browser-check.md';
const THREAD =
  'the project thread "Build the game from change-set 3", session_01WLuqnwQ8c8JuNgfBX7Wixg, in his claude.ai project "Shamble Frontlines ALT", on an account beside the two that CLAUDE.md\'s ACCOUNTS names (the project\'s description calls it his second personal account)';
// The commit that holds the thread's saved messages and both plans, for when the notes are deleted.
const NOTES_AT = 'fcf6def';
const TAP =
  "a tap on the thread's decision card: the line below is the message the platform wrote for his tap, in the card's wording, not text he typed; extracted by script from the session's transcript";
const sha = (s) => createHash('sha256').update(s).digest('hex');
// A line that stands for text left out of his message (thread-words.mjs) is shown as a pointer, outside the quote.
const LEFT_OUT = /^\[left out, lines \d+ to \d+: .*\]$/;
const quote = (s) => s.split('\n').map((l) => (LEFT_OUT.test(l) ? `- _${l}_` : l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) =>
  s.split('\n').map((l) => (l === '>' ? '' : /^- _\[left out, lines .*\]_$/.test(l) ? l.slice(3, -1) : l.replace(/^> /, ''))).join('\n');
const wrap = (text, rest = '  ') => {
  const out = [];
  let line = '';
  let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) {
      out.push(line);
      line = rest + w;
    } else {
      line += (empty ? '' : ' ') + w;
      empty = false;
    }
  }
  out.push(line);
  return out.join('\n');
};
function saved(file) {
  const note = readFileSync(NOTES + file, 'utf8');
  const cut = note.indexOf('\n---\n');
  if (cut < 0) throw new Error(`no --- in ${file}`);
  const words = note.slice(cut + 5).replace(/\n$/, '');
  const said = note.slice(0, cut).match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
  if (!said || Number(said[1]) !== Buffer.byteLength(words) || said[2] !== sha(words)) throw new Error(`${file}: the words differ from its own count or sha256`);
  return { words, bytes: Buffer.byteLength(words), sha256: sha(words) };
}
const by = (m) => `Copied by script (${m.bytes} bytes, sha256 ${m.sha256}):`;
// The thread's own text under his words: a label line, then the text whole in a fenced block.
const fence = (label, n) =>
  `${wrap(`- _The thread's text, not his words: ${label} (copied by script, ${n.bytes} bytes, sha256 ${n.sha256}):_`)}\n\`\`\`text\n${n.words}\n\`\`\`\n`;

// Newest first. `file` undefined: a pointer, with no words of his quoted under it.
const ENTRIES = [
  {
    key: '2026-11-23, 14:54:47 UTC: WRITE-WHERE',
    file: 'mustafa-2026-11-23T145447-write-where.md',
    after: [
      { file: 'thread-2026-11-23T145131-correction.md', label: 'its message of 14:51:31 UTC, which corrects its answer of 14:33:55 UTC, holds the CI write and announces the card' },
      { file: 'thread-2026-11-23T145133-card-write-where.md', label: 'the decision card he answered, posted at 14:51:33 UTC' },
    ],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 14:54:47 UTC: WRITE-WHERE, the CI file is written in a new Claude Code chat, not in the project thread** (${THREAD}; ${TAP}). It answers the thread's card of 14:51:33 UTC, which the thread posted after correcting its answer of 14:33:55 UTC to his question of 14:32:43 UTC; the correction of 14:51:31 UTC and the card stand under his tap as the thread's text, not his words. It replaces his answer WRITE-THREAD (the entry below). Nothing was written under \`.github/\` in the thread. ${by(m)}`,
  },
  {
    key: '2026-11-23, 14:32:43 UTC: WRITE-THREAD',
    file: 'mustafa-2026-11-23T143243-write-thread.md',
    after: [{ file: 'thread-2026-11-23T142918-card-write-thread.md', label: 'the decision card he answered, posted at 14:29:18 UTC' }],
    lead: (m) =>
      `- **Replaced by WRITE-WHERE (the entry above). Taken by Mustafa, 2026-11-23, 14:32:43 UTC: WRITE-THREAD, his answer to the thread's card "Allow the CI file write in this thread, which has no Accept edits mode?": a yes, with his question why "a new Claude Code chat running on Fable 5.2" does not create the CI files instead** (${THREAD}; his message cmsg_019xis1ZabqTLNQbEMzA6WbhUZGHHmFH2k2HzELe9gE4ZM, typed in the app, whole; extracted by script from the session's transcript). His first line is the card's question; the card is shown under his words as the thread's text, not his words. The thread's reply (14:33:55 UTC) said a normal Claude Code chat adds only the Accept edits switch and left out the repository's guard; the thread corrected it at 14:51:31 UTC and asked again (WRITE-WHERE). Nothing was written on this answer. His WRITE-MODE words (the entry of 2026-11-18, 20:20:09 UTC, below: "From now on you read the chat's mode right before each kit write and stop unless it is Accept edits.") are unchanged. The thread's reply of 14:33:55 UTC is in the notes (\`.checkpoints/ci-wasm/thread-2026-11-23T143355-why-here.md\`, readable at ${NOTES_AT} once the notes are deleted). ${by(m)}`,
  },
  {
    key: '2026-11-23, 14:32:42 UTC: LONG-RUNS',
    file: 'mustafa-2026-11-23T143242-long-runs.md',
    after: [{ file: 'thread-2026-11-23T143025-card-long-runs.md', label: 'the decision card he answered, posted at 14:30:25 UTC' }],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 14:32:42 UTC: LONG-RUNS, long runs in his project go on GitHub, from a branch that is pushed** (${THREAD}; ${TAP}). The card is shown under it as the thread's text, not his words. The thread reads it as: a full test suite or a long build is pushed and read from GitHub's checks on that branch; the short runs (a test file by its path, the legacy gate, one build, a rehearsal) still run in the thread's container; and none of the evidence the change-set protocol asks for changes. ${by(m)}`,
  },
  {
    key: '2026-11-23, 14:31:06 UTC: ANSWERS-7',
    file: 'mustafa-2026-11-23T143106-answers-7.md',
    after: [
      { file: 'thread-2026-11-23T142825-seven-answers-part.md', label: 'its message of 14:28:25 UTC from its third paragraph to its end, which states the seven answers back to him (the two paragraphs before it are about the thread itself and about his question on the chats\' names; the whole message is in the notes); the CI text was attached to the message as a file' },
      { file: 'thread-2026-11-23T142827-card-answers-7.md', label: 'the decision card he answered, posted at 14:28:27 UTC' },
    ],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 14:31:06 UTC: ANSWERS-7, his yes to the seven recommended answers. For the CI-WASM pull request: CHROME-SOURCE (a), the Google Chrome in GitHub's runner image; CI-TEXT (a), yes to the CI text and what goes with it; FIREFOX-TESTS (c), a small pull request of its own right after it. For change-set 3's plan: RNG-SOURCE (c), our own ChaCha8, checked against the published test vectors; SIZE-3 (b), two change-sets, 3a then 3b; PRESS-PACE (a), a second tap inside one tick is applied one tick later; DEPART-3 (a), he accepts the list** (${THREAD}; ${TAP}). What his tap answers is shown under it as the thread's text, not his words: the part of the thread's message of 14:28:25 UTC that states the seven answers back in plain words, and the card. The file attached to that message is the CI text, byte for byte \`.checkpoints/ci-wasm/ci-edit.diff\` (128 lines, 93 added, none removed, sha256 240c7db726dee600b54d10b0252b33e527482ecf88ba41af849660d55e8d2673). Each question in full, with every option and the recommended one, stands in the CI plan's section 16 and in change-set 3's plan (draft 3), section 12 (\`.checkpoints/ci-wasm/plan.md\` and \`.checkpoints/ci-wasm/cs3-plan.md\`, readable at ${NOTES_AT} once the notes are deleted): so the first builder chat put them to him on 2026-11-19 (at about 20:03 and 21:07 UTC, by its card), so a thread of his project, "Answer the seven paused lane questions", laid them out for him on 2026-11-23 (its messages of 13:39:57 and 14:11:11 UTC, in plain words, with the options; the thread read them as tool output), and he pasted the first builder chat's own summary of them into this thread himself (the entry of 14:25:03 UTC below: lines 5 to 90 of his message). The recommended answers hold more than the thread's message spells out. CI-TEXT (a) is one yes for five things: \`ci-edit.diff\`; \`ci-plant-p-gh.diff\` (the two planted lines, sha256 e4f21ab359bc9fd911c663a4fcbc892133195fefe44e88ad7ec51b50e3d0b1ac), written in one commit and taken out in the next; the second planted change, one committed render count changed from 3 to 4 and taken back; the stop rule of the CI plan's section 9; and the tool's second route, the runner's curl line with the limits of the route he approved. His yes to CI-TEXT came with his WRITE-MODE condition: the CI file is written only in a chat in Accept edits (WRITE-THREAD and WRITE-WHERE, above). The card's "start building change-set 3a after it" changes no word of his: by CI-WASM (a) change-set 3's build starts after the CI pull request is merged, and by FIREFOX-TESTS (c) the Firefox pull request comes right after it. SIZE-3 (b) carries his yes in advance to the two ladders of change-set 3's plan (its sections 4 and 5: a part moves whole with its tests to a named place, no test is dropped, he is told when a part moves). DEPART-3 (a) is the list of that plan's section 12, question 4. The advisor agent's rulings that went to him with the questions, each his to overrule, stand, since he has named no objection: PERM-ATT (a), PLANTS both, PYCACHE-NOW (b), WASM-TEST-ROUTE (a), CI-LOOP (a) for E1 only. ${by(m)}`,
  },
  {
    key: '2026-11-23, 14:25:03 UTC: he would rather start today',
    file: 'mustafa-2026-11-23T142503-builder-chat.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 14:25:03 UTC: he would rather start today, and would like a new Fable 5.2 based Claude Code chat to continue building the game** (his message cmsg_019xis1ZabqTLNQbEMzA6WbhR97Lu4zJ8QjD4X3hYFruVA, typed in the main chat of his claude.ai project "Shamble Frontlines ALT", on an account beside the two that CLAUDE.md's ACCOUNTS names (the project's description calls it his second personal account); his project made the thread "Build the game from change-set 3" from it, session_01WLuqnwQ8c8JuNgfBX7Wixg; extracted by script from that session's transcript). Of its 211 lines, 201 are the last texts of three other chats, pasted by him: they are left out and named in the lines in square brackets, and his own lines are whole (the whole message: 16731 bytes, sha256 93c7645aebfee23df6658110d7cfe3fdd4d0e8f8db6be4d6ca2216f773848f93). The chat he calls Builder V1 is the first builder chat, lane 1 of this repository's records (session_019YNMUV9LFhASseJ9EP12kL, branch \`claude/wonderful-hamilton-jdmlnf\`), paused by his words of 2026-11-19, 21:44:09 UTC (the entry below; the project's note to the thread: that account is out of usage until 2026-11-24, 20:00 Amsterdam time). His project made this thread from his message; the thread took lane 1's records forward on a branch of its own, \`claude/project-thread-qx9woh\`, started from that chat's last commit d064e17b84b73c8040ffe66db7105a517cde9215, and never pushes to that chat's branch (LANE-RULES: "a lane never pushes to another lane's branch"); the CI write went to a new Claude Code chat (WRITE-WHERE, above). The first line of his message quotes a paste block another thread of his project had written for the first builder chat; his question under it was answered in the thread's message of 14:28:25 UTC (\`.checkpoints/ci-wasm/thread-2026-11-23T142825-seven-answers.md\`, readable at ${NOTES_AT} once the notes are deleted): by "Lane 1" the paste block means the chat he calls Builder V1. ${by(m)}`,
  },
  {
    key: '2026-11-19, 21:44:09 UTC: the pause',
    file: 'mustafa-2026-11-19-pause.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 21:44:09 UTC: the pause: the first builder chat finishes what is in hand and starts nothing until he says so** (${SESSION}; extracted by script from the session's transcript, message 314f33b2-7c58-441f-8099-0b96cf4cd024, whole). When it came, the CI-WASM pull request (#62) was a draft with no CI file written, and its three questions and change-set 3's four were with him; the chat finished what was in hand and stopped (its last commit: d064e17). He has not told that chat to continue; on 2026-11-23 a thread of his project took the records forward, and the CI-WASM pull request goes to a new Claude Code chat (WRITE-WHERE; the entries above). ${by(m)}`,
  },
  {
    key: '2026-11-19, 16:08:04 UTC: two notes for the work after #61',
    file: 'mustafa-2026-11-19-1638-two-notes.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 16:08:04 UTC: two notes for the work after #61: the docs pass that records the merge of #61 stays with the CI-WASM pull request, whose report keeps the workflow text apart from the docs; and for change-set 3's browser check the README asks him to keep the browser's window in front and uncovered during the first run and has one step with the window covered on purpose, so that BR-GAP is read and not guessed** (${SESSION}; his label: "My words of 2026-11-19, 16:38 (Amsterdam time)"; extracted by script from the session's transcript, message e02d3ad1-8f0c-4b6a-a30a-5b267f681630, whole: its first paragraph is his instruction to the chat after a compaction). BR-GAP is the one unexplained reading of his browser check: \`${RECORD}\`. ${by(m)}`,
  },
  {
    key: 'his browser check of the first page (#61), 2026-11-19, 14:46:25 to 15:09:29 UTC',
    lead: () =>
      `- _A pointer, not a decision: **his browser check of the first page (#61), 2026-11-19, 14:46:25 to 15:09:29 UTC.** Four chat messages (14:46:25, 14:54:19, 15:02:50 and 15:09:29 UTC) with eight "Copy report" texts, from his own browser (Brave 1.100.58) and from Firefox 160 on his Windows computer, and one picture. His go below says: "My browser reports are recorded with the next docs pass, not by a push to this head." They stand whole, copied by script, in \`${RECORD}\`, Appendix A, with the build chat's reading of them against the zip's README. His first Brave report read "last pause: none yet" where the README expects a recorded pause; by his go's condition ("If your reading of them shows a line that is not as your README expects, stop and tell me before you merge.") the chat told him at 14:50:49 UTC (its message 7e4ded94-23e4-459c-ad2a-6d1fdcb3d7d8 in the session's transcript), before any merge, and his re-check of 15:02:50 UTC showed the pause (45.0 s hidden, 0 ticks while hidden)._`,
  },
  {
    key: '2026-11-19, 14:31:30 UTC: his answer to the question VISION-NOTE',
    file: 'mustafa-2026-11-19T143130.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 14:31:30 UTC: his answer to the question VISION-NOTE ((a): the dated note for \`docs/VISION_V2.md\` is right as the chat wrote it)** (${SESSION}; extracted by script from the session's transcript, message 94cf4e6c-e46b-48a4-b48e-3270856870be, whole). The question, as the chat put it to him at 14:29:51 UTC under the note's exact text (the chat's message 699736ec-f365-4458-aceb-592bba842d59, copied by script): "${QUESTION}", with the options "${OPTION_A}" and "${OPTION_B}" The text it showed is the dated note now in \`docs/VISION_V2.md\` §4 and the sentence added to that file's "Updated on" lines, both written by this docs pass from that message, byte for byte. His second sentence asks for the instructions again; the chat answered with the browser check's steps at 14:31:56 UTC. ${by(m)}`,
  },
  {
    key: '2026-11-19, 14:28:25 UTC: his go on the merge of pull request #61',
    file: 'mustafa-2026-11-19-1516-go-61.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 14:28:25 UTC: his go on the merge of pull request #61 (change-set 2, the first page) into main by a merge commit at head 4cf30ce303b16e96b3ca5d5c0b20b928a4db6b86, what he accepts with that merge, his answer BROWSER-SET ((a), order only: Safari stays a target of v2; Chrome, the Chromium-based browsers and Firefox come first; Safari and Mac come later), and what comes after the merge (the CI-WASM pull request, its exact text to him first, with change-set 3's plan beside it; with change-set 3, one line in \`.gitignore\` for \`__pycache__/\`)** (${SESSION}; his label: "My words of 2026-11-19, 15:16 (Amsterdam time)"; extracted by script from the session's transcript, message 65801ff7-6e9d-4343-b384-16a67118bc87, whole; the same bytes stand in the message of the merge commit f5030a3). When these words came, no browser report was in the chat; the merge waited for his reports (the pointer above) and was made at 15:10:54 UTC, at that head, with \`gate\`, \`smoke\` and \`toolchain\` green on it (read at 15:10 UTC). The dated note he asks for stands in \`docs/VISION_V2.md\` §4 (his answer VISION-NOTE, the entry above). ${by(m)}`,
  },
];
// The dated pointer (REC-OLD: a line that records what he decided is never reworded). It goes under the quoted words
// of the entry of 13:36:37 UTC, that is, right before the entry of 11:07:48 UTC.
const ANCHOR = '- **Taken by Mustafa, 2026-11-19, 11:07:48 UTC: his answer to the question SIZE-STOP-2 of change-set 2';
const POINTER =
  wrap(
    `  _Dated pointer, 2026-11-19 (the docs pass after #61's merge; the advisor agent's wording point on head 4cf30ce): where the header of this entry says the lost night is "accepted as the report writes it down", his words above are "is accepted as you wrote it down, with your lesson."_`,
  ) + '\n';

const blocks = ENTRIES.map((e) => {
  const m = e.file === undefined ? undefined : saved(e.file);
  const lead = wrap(e.lead(m));
  const after = (e.after ?? []).map((a) => {
    const n = saved(a.file);
    return { ...a, n, text: fence(a.label, n) };
  });
  return { ...e, m, lead, after, block: (m === undefined ? `${lead}\n` : `${lead}\n${quote(m.words)}\n`) + after.map((a) => a.text).join('') };
});
const TOP = '## Decisions, newest first\n';
const before = execFileSync('git', ['show', `${BEFORE}:${DEC}`], { maxBuffer: 1 << 26 }).toString('utf8');
if (before.split(TOP).length !== 2) throw new Error('the list heading must stand once');
if (before.split(ANCHOR).length !== 2) throw new Error('the anchor entry must stand once');
const text = before.replace(TOP, TOP + blocks.map((b) => b.block).join('')).replace(ANCHOR, POINTER + ANCHOR);

if (!process.argv.includes('--check')) writeFileSync(OUT, text);
const t = readFileSync(OUT, 'utf8');
const ok = {
  'the file on disk is what the script writes': t === text,
  [`with the new blocks and the pointer taken out, every byte equals the file at ${BEFORE.slice(0, 7)}`]:
    blocks.reduce((s, b) => s.replace(b.block, ''), t).replace(POINTER, '') === before,
  'the newest entry stands at the top of the list': t.includes(TOP + blocks[0].lead),
  'no new line over 120 characters outside his quoted words and the thread\'s quoted text': [
    ...blocks.map((b) => b.lead),
    ...blocks.flatMap((b) => b.after.map((a) => a.text.split('\n```text\n')[0])),
    ...blocks.flatMap((b) => (b.m === undefined ? [] : b.m.words.split('\n').filter((l) => LEFT_OUT.test(l)).map((l) => `- _${l}_`))),
    POINTER,
  ].every((s) => s.split('\n').every((l) => l.length <= 120)),
  'no carriage return': !t.includes('\r'),
};
for (const b of blocks) {
  ok[`the lead holds its key: ${b.key.slice(0, 60)}`] = b.lead.replace(/\n {2}/g, ' ').includes(b.key);
  if (b.m === undefined) continue;
  const i = t.indexOf(b.lead);
  ok[`${b.file}: in DECISIONS, unquoted, equal to the saved words (${b.m.bytes} bytes, sha256 ${b.m.sha256.slice(0, 12)})`] =
    i >= 0 && unquote(t.slice(i + b.lead.length + 1, i + b.lead.length + 1 + quote(b.m.words).length)) === b.m.words;
  for (const a of b.after) {
    const j = t.indexOf(a.text);
    ok[`${a.file}: under that entry as the thread's text, equal to the saved text (${a.n.bytes} bytes, sha256 ${a.n.sha256.slice(0, 12)})`] =
      j > i && t.slice(j, j + a.text.length).split('\n```text\n')[1] === `${a.n.words}\n\`\`\`\n`;
  }
}
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
