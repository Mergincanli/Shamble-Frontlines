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
// Since 2026-11-23, 16:30 UTC the chat he calls Builder V3 holds the pull request: his messages there are saved by
// chat-words.mjs. A quote of his inside a lead is checked against his saved words, so it cannot drift from them.
const V3 =
  'the builder chat he calls Builder V3, session_01WLuqnvpAh9XTitx8ehKJcQ, a Claude Code chat on the account that also holds his claude.ai project "Shamble Frontlines ALT"';
// The commit that holds Builder V3's saved messages, for when the notes are deleted.
const V3_NOTES_AT = '08053c7';
const BLOCK = saved('mustafa-2026-11-23T163046-builder-v3.md');
const BLOCK_AS_WRITTEN = readFileSync(NOTES + 'handover-block.txt', 'utf8').replace(/\n$/, '');
const said = (m, s) => {
  if (!m.words.includes(s)) throw new Error(`not in his saved words: ${s}`);
  return `"${s}"`;
};
// The thread's own text under his words: a label line, then the text whole in a fenced block.
const fence = (label, n) =>
  `${wrap(`- _The thread's text, not his words: ${label} (copied by script, ${n.bytes} bytes, sha256 ${n.sha256}):_`)}\n\`\`\`text\n${n.words}\n\`\`\`\n`;

// What a message of his in Builder V3's chat answers: the chat's own text, kept in the saved note's header by
// chat-words.mjs. It is shown in the lead as the chat's words, never as his.
const asked = (file) => {
  const note = readFileSync(NOTES + file, 'utf8');
  const m = note.slice(0, note.indexOf('\n---\n')).match(/What it answers: ([\s\S]*)_\s*$/);
  if (!m) throw new Error(`${file}: no "What it answers" in its header`);
  return m[1].trim();
};
const ASKED = (file) => `What it answers, in the chat's words (not his): ${asked(file)}`;
const V3_MSG = (id) => `${V3}; his message ${id}, whole; extracted by script from the chat's transcript`;
// Newest first. `file` undefined: a pointer, with no words of his quoted under it.
const ENTRIES = [
  {
    key: '2026-11-23, 21:24:59 UTC: FIREFOX-TESTS (b)',
    file: 'mustafa-2026-11-23T212459-after-64.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 21:24:59 UTC: FIREFOX-TESTS (b) in place of (c): Firefox in CI joins the small CI pull request after change-set 3a, and no Firefox pull request comes between #64 and 3a; change-set 3a starts when #64 is merged, on this word, with no second ask; three items for change-set 3 come from his Advisor before 3b's build starts** (${V3}; his message 219448d3-8dec-4c23-bf93-d3e88bb6110c, sent while the chat was working, whole; extracted by script from the chat's transcript). It replaces his answer FIREFOX-TESTS (c) in ANSWERS-7 (the entry of 14:31:06 UTC below, which stays as written). In his words: ${said(m, 'Until then my own browser check covers Firefox, as at change-set 2.')} On the start of 3a: ${said(m, 'You need not ask me again to start. If your context is above 40 % at that point, tell me first and I compact the chat.')} On the three items: ${said(m, 'They do not touch 3a.')} ${ASKED('mustafa-2026-11-23T212459-after-64.md')} ${by(m)}`,
  },
  {
    key: '2026-11-23, 20:47:59 UTC: PLANT-WRITE (a)',
    file: 'mustafa-2026-11-23T204759-plant-write.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 20:47:59 UTC: PLANT-WRITE (a), yes to the two planted lines in GitHub's check; and his yes in advance to their take-back, only if the planted run fails the expected way** (${V3_MSG('8fe87b69-7b75-4459-bdd0-82430dc07783')}). The expected way, in his words: ${said(m, 'at the step "GitHub\'s own check of the archive", with gh\'s own line that the file is not part of the release, and nothing after it runs')}; and the other case: ${said(m, 'If the run fails any other way, stop and bring me the lines before any write, as the stop rule says.')} His point 3: ${said(m, 'My mode rule of 17:15:07 UTC holds for both writes, read on this message.')} ${ASKED('mustafa-2026-11-23T204759-plant-write.md')} Records, not his words: the mode on this message read \`default\`; the planted file was written at 20:48:40 UTC (sha256 84b6b3fd61dc1e798375374eef48f16ca3e9098887bd63cd944c4191bc631ff8, commit 47d7024); both of its runs failed at that step with gh's line "attestation for 0.2.130 does not contain subject sha256:27c24e42e238fa86ca5feb42b3eeb1a4535040ed5dd0eb183da810f85cff6e10" and the three later steps were skipped; the take-back was written at 21:01:33 UTC (sha256 56d37fc64ccfce2a21765613401a746633c6a9c661f3af52a9054bf650df9811, commit ce5a7e7). ${by(m)}`,
  },
  {
    key: '2026-11-23, 19:58:23 UTC: FIX-WRITE (a)',
    file: 'mustafa-2026-11-23T195823-fix-write.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 19:58:23 UTC: FIX-WRITE (a), yes to the corrected CI text (\`ci-edit-2.diff\`, against the file at cb476b1), to the stop rule's new wording, and to the two new checksums for the planted lines and their take-back; his mode rule of 17:15:07 UTC holds for this write** (${V3_MSG('e06bc496-dee4-49fe-8321-68a323d5e5f4')}). The text puts the tool into a folder the job makes fresh as root directly under \`/\` (the place his Advisor's input of 18:40:57 UTC named, the entry below, weighed and taken by infra-engineer) and has the check judge the program and every folder above it up to \`/\`. ${ASKED('mustafa-2026-11-23T195823-fix-write.md')} Records, not his words: the mode on this message read \`default\`; the file was written at 20:00:41 UTC (sha256 56d37fc64ccfce2a21765613401a746633c6a9c661f3af52a9054bf650df9811, commit 2a654f3), and its first runs were clean under the stop rule. ${by(m)}`,
  },
  {
    key: '2026-11-23, 18:40:57 UTC: OPT-PATH (a)',
    file: 'mustafa-2026-11-23T184057-opt-path.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 18:40:57 UTC: OPT-PATH (a): the tool moves to a place where every parent folder is root-owned and closed to others, and the check is widened to every parent up to \`/\`, not weakened; the corrected text is prepared in one short pass and its exact text comes to him before anything is written; NOTES-PUSH (a)** (${V3_MSG('ee900136-bb16-4ee5-a6d4-8a7eccef346c')}). In his words: ${said(m, 'The check is not weakened.')} The place directly under \`/\` is his Advisor's input, ${said(m, 'to weigh, not an order')}. The pass: ${said(m, 'infra-engineer proposes, security-auditor reads the changed lines, your advisor agent rules. A second round only for a Critical or High.')} And: ${said(m, 'Nothing under .github/ is written before my yes on that text. The stop rule holds for the next run too.')} His point 5 holds two sentences on the guard's prompt for the copy, one that he saw and allowed it and one that he saw none; it stands as he sent it, and he asks what the transcript shows. ${ASKED('mustafa-2026-11-23T184057-opt-path.md')} Records, not his words: the transcript shows the guard's hook asked at 17:23:24 UTC and a one-time allow given in the app answered it at 17:23:43 UTC, which the chat told him in its next message; the mode on this message read \`default\`. ${by(m)}`,
  },
  {
    key: '2026-11-23, 17:22:42 UTC: his yes to CI-WRITE-1 again',
    file: 'mustafa-2026-11-23T172242-ci-write-1-again.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 17:22:42 UTC: his yes to CI-WRITE-1 again, sent with the mode switched; he has opened the pull request** (${V3_MSG('99a5bc79-9b73-4f2a-ba5f-4e1c78eb6fb2')}). ${ASKED('mustafa-2026-11-23T172242-ci-write-1-again.md')} Records, not his words: the mode on this message read \`default\`, which his rule of 17:15:07 UTC (the entry below) covers; the file was written at 17:23:43 UTC (sha256 d1dbcb533460936e9f4f3ca2e97ed28c6d586638e52c00fa034e875f7a4e975c, commit cb476b1); the pull request is #64, opened at 17:21:24 UTC and made a draft by him later that day. ${by(m)}`,
  },
  {
    key: '2026-11-23, 17:15:07 UTC: CI-WRITE-1 (a)',
    file: 'mustafa-2026-11-23T171507-ci-write-1.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 17:15:07 UTC: CI-WRITE-1 (a), yes to the first CI write; his rule for how the chat's mode is read before a CI write; GH-TOOLS (a) for this pull request: he opens the draft, presses Ready and presses Merge on GitHub himself, by steps that stand in for the "expected head" part of his point 9** (${V3_MSG('4c139df2-6889-4a70-bc2d-461bfdc159cc')}). The mode rule, in his words: ${said(m, 'Write if the record on this message reads acceptEdits. If it reads default: my screen shows Accept edits, and on 2026-11-12 the record gave that same mode as "default" (the archive\'s MANUAL-MODE), so write as well and name the value in the report. Any other value: do not write, and tell me the value. The same reading holds for writes 2 and 3.')} The merge steps, in his words: ${said(m, 'your advisor agent approves the full SHA, gate, smoke and toolchain are green on it, you push nothing after that approval, you give me the full SHA when you ask for my go, and I type my go in this chat before I press Merge. My main Advisor checks from the repo that the pull request\'s head is that SHA.')} His point 1 leaves the pull request's number open, as he sent it: ${said(m, 'The draft pull request is open: #__.')} ${ASKED('mustafa-2026-11-23T171507-ci-write-1.md')} Records, not his words: the mode on this message read \`auto\` (the session record and the message), so by his own rule nothing was written on it; no pull request from the branch was on GitHub at 17:15 UTC. ${by(m)}`,
  },
  {
    key: '2026-11-23, 16:30:46 UTC: Builder V3',
    file: 'mustafa-2026-11-23T163046-builder-v3.md',
    after: [
      { file: 'thread-2026-11-23T150951-handover-decisions.md', label: 'the "Decisions needed" of its message of 15:09:51 UTC, the message that carried the block for him to paste (the whole message is in the notes)' },
    ],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 16:30:46 UTC: Builder V3, a new Claude Code chat on Fable 5.2, takes over the CI-WASM pull request; CI-MODEL (a): for that chat's CI edits of the CI-WASM pull request, each asked of him with its exact text, he sets his words of 2026-11-12 on CI work aside** (${V3}; his message 67cc6bba-62e1-48f2-bb37-8d7ae79afa81, whole; extracted by script from the chat's transcript). Points 1 to 10 are the block the project thread wrote for him to paste (\`.checkpoints/ci-wasm/handover-block.txt\`, saved and pushed at c405ccb before the thread posted it; his message starts with it byte for byte); pasted into the chat, it is his word. Points 11 and 12 he added himself. What it answers is shown under it as the thread's text, not his words. The thread's question CI-MODEL is answered by his point 6: ${said(m, 'For this chat\'s CI edits of the CI-WASM pull request, each asked of me with its exact text, I set my words of 2026-11-12 ("The guard, the gate and CI work stays with Opus 5.5.") aside, as GATE-FOLLOW did for Builder V1.')} His point 11 gives his taps BUILD-SEAT and ACCT-3 (the two entries below) in his own words. At 16:33:16 UTC he also sent points 1 to 10 as a file, with the same sha256 as \`handover-block.txt\` (compared by sha256; the file was not opened); that message is in the notes (\`.checkpoints/ci-wasm/mustafa-2026-11-23T163316-attached-block.md\`, readable at ${V3_NOTES_AT} once the notes are deleted). ${by(m)}`,
  },
  {
    key: '2026-11-23, 15:11:01 UTC: ACCT-3',
    file: 'mustafa-2026-11-23T151101-acct-3.md',
    after: [{ file: 'thread-2026-11-23T150956-card-acct-3.md', label: 'the decision card he answered, posted at 15:09:56 UTC' }],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 15:11:01 UTC: ACCT-3, the account his claude.ai project "Shamble Frontlines ALT" runs on counts like his other two** (${THREAD}; ${TAP}). The card is shown under it as the thread's text, not his words. In his block of 16:30:46 UTC (the entry above, point 11) he puts it as ${said(BLOCK, 'ACCT-3 (the account this chat runs on counts like my other two)')}. No line of CLAUDE.md is written from this tap: by the card's first option the account's roles and the exact line for CLAUDE.md come to him for a yes first. The entries of 2026-11-23 below call it "an account beside the two that CLAUDE.md's ACCOUNTS names"; they stay as written. ${by(m)}`,
  },
  {
    key: '2026-11-23, 15:10:32 UTC: BUILD-SEAT',
    file: 'mustafa-2026-11-23T151032-build-seat.md',
    after: [{ file: 'thread-2026-11-23T150954-card-build-seat.md', label: 'the decision card he answered, posted at 15:09:54 UTC' }],
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-23, 15:10:32 UTC: BUILD-SEAT, the new Claude Code chat builds change-set 3 after the CI pull request is merged, when he tells it to go on** (${THREAD}; ${TAP}). The card is shown under it as the thread's text, not his words. In his block of 16:30:46 UTC (the entry above) he puts it as ${said(BLOCK, 'BUILD-SEAT (once the CI pull request is merged, you build change-set 3, starting when I say so in this chat)')}, and its point 9 says ${said(BLOCK, 'Do not start change-set 3.')} ${by(m)}`,
  },
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
  'his block of 16:30:46 UTC starts with handover-block.txt, byte for byte': BLOCK.words.startsWith(BLOCK_AS_WRITTEN),
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
