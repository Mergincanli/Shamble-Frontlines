// The docs pass of change-set 2 (the first page): writes Mustafa's chat messages of this change-set into
// docs/DECISIONS.md, newest first. His words are never retyped: they come from the files saved by script from the
// session's transcript (words.mjs), each with its byte count and sha256. Run from the repository root:
//   node .checkpoints/cs2-first-page/decisions.mjs [--check]
// The list ENTRIES below is newest first. An entry is added here when a new message of his is saved to the notes.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const DEC = 'docs/DECISIONS.md';
const NOTES = '.checkpoints/cs2-first-page/';
const REPORT = process.env.CS2_REPORT || 'docs/reports/2026-11-19-cs2-first-page.md';
const MERGE_60 = 'b763ba3e5f34c67fc4f875cfbd9266167fb095f3';
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const wordsOf = (file) => {
  const t = readFileSync(NOTES + file, 'utf8');
  const i = t.indexOf('\n---\n');
  if (i < 0) throw new Error('no --- in ' + file);
  return t.slice(i + 5).replace(/\n$/, '');
};
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
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
const copied = (w) => `Copied by script (${Buffer.byteLength(w)} ${Buffer.byteLength(w) === 1 ? 'byte' : 'bytes'}, sha256 ${sha(w)}):`;

/** Newest first. `key` is the text that must not yet stand in DECISIONS.md; `head(words)` is the entry's lead. */
const ENTRIES = [
  {
    file: 'mustafa-2026-11-19-size-stop-2.md',
    key: '2026-11-19, 11:07:48 UTC: his answer to the question SIZE-STOP-2',
    head: (w) =>
      `- **Taken by Mustafa, 2026-11-19, 11:07:48 UTC: his answer to the question SIZE-STOP-2 of change-set 2 (the change-set stood at 4,430 added lines against the stop line of 4,400, with review round 2's fix pass of 41 lines prepared and not yet applied): (a), the stop line moves to 4,700 and the fix pass goes in** (${SESSION}; extracted by script from the session's transcript, message fd359559-31d2-4e96-8d41-ac8723b7ffb1, whole: one letter). Two questions stood open when he wrote it, both as the chat had put them: SIZE-STOP-2, and WATCH-61 (should the chat watch pull request #61: (a) no, as with #58 and #60; (b) yes). The chat read his letter as (a) on SIZE-STOP-2; on WATCH-61 the same letter means no, which is also what happens when nothing is done: #61 is not watched. Both questions as asked stand in \`${REPORT}\`. ${copied(w)}`,
  },
  {
    file: 'mustafa-2026-11-19-until-7am.md',
    key: '2026-11-19, 00:23:22 UTC: for the night',
    head: (w) =>
      `- **Taken by Mustafa, 2026-11-19, 00:23:22 UTC: for the night, the build chat need not ask his permission until 7 am and goes on with its work as it is** (${SESSION}; sent while a turn was running and recorded in the session's transcript as a queued command, uuid 9c97c1e4-c16b-497b-bac5-376f8fad67a3, whole). How the chat read it, and what it did not take it for (a go on a merge, an edit to a kit, gate or CI file, or other code from the internet), stands in \`${REPORT}\`; the window ended at 7 am Amsterdam time (06:00 UTC) on 2026-11-19. ${copied(w)}`,
  },
  {
    file: 'mustafa-2026-11-19-size-stop.md',
    key: '2026-11-19, 00:18:21 UTC: his answer to the question SIZE-STOP',
    head: (w) =>
      `- **Taken by Mustafa, 2026-11-19, 00:18:21 UTC: his answer to the question SIZE-STOP of change-set 2 (the change-set stood at 3,964 added lines against the stop line of 4,000, with four small browser test files unwritten): (a), the stop line moves to 4,400 and the four files are written as planned** (${SESSION}; sent while a turn was running and recorded in the session's transcript as a queued command, uuid 08191595-0b56-4531-a60e-2720c6028109, whole). The question as it was put to him at 00:17 UTC, with its two options, stands in \`${REPORT}\`: it was the only decision open at that moment. ${copied(w)}`,
  },
  {
    file: 'mustafa-2026-11-18-2326-five-answers.md',
    key: '2026-11-18, 22:34:37 UTC: his answers to the five questions of change-set 2',
    head: (w) =>
      `- **Taken by Mustafa, 2026-11-18, 22:34:37 UTC: his answers to the five questions of change-set 2 (the first page, foundation step 2.5), and the size: RNG-CRATE (which random-number library, and when: none now, the library enters in change-set 3 and he is asked which with that plan), WB-HEALTH (go on with wasm-bindgen or not: go on, pinned to exactly 0.2.130, what crosses it kept small), TOOL-SOURCE (where the wasm-bindgen tool comes from: the ready-built file he names, this one file only), DEPART-2 (the plan's six departures from the decision drafts: all six) and CI-WASM (when CI builds the Wasm: a small pull request of its own right after change-set 2 is merged, its exact text to him first)** (${SESSION}; his label: "My words of 2026-11-18, 23:26 (Amsterdam time)"; extracted by script from the session's transcript, message d6abf351-4175-4bc2-bb81-9db74fc152fb, whole). The five questions as asked stand in the plan he answered on, \`${NOTES}plan.md\` at commit 1d79e8e4, section 16, and in \`${REPORT}\`. ${copied(w)}`,
  },
  {
    file: 'mustafa-2026-11-18-2059-merge-60.md',
    key: '2026-11-18, 20:20:09 UTC: his go on the merge of #60',
    head: (w) =>
      `- **Taken by Mustafa, 2026-11-18, 20:20:09 UTC: his go on the merge of #60 (the kit hold and the two CI edits), what he accepts with it, his answers to WRITE-MODE (the chat's mode at the two kit writes: accepted as made; from now on the mode is read right before each kit write) and REC-OLD (may a docs pass correct older lines of the shared record files: yes, with one limit), and what comes after** (${SESSION}; his label: "My words of 2026-11-18, 20:59 (Amsterdam time)"; extracted by script from the session's transcript, message 8d566988-02ff-489c-a370-7ffd79f49dae, whole: its first lines are his own, typed above the Advisor's block: his answer "REC-OLD: A", the chat's question about the test zip of #58 quoted back, and his request for the zip's steps). The two questions as asked stand in \`docs/reports/2026-11-18-kit-hold-ci.md\`. His words from the line "My words of 2026-11-18, 20:59 (Amsterdam time)" to the end stand byte for byte in the message of merge commit ${MERGE_60}. ${copied(w)}`,
  },
];

const TOP = '## Decisions, newest first\n';
const entries = ENTRIES.map((e) => {
  const words = wordsOf(e.file);
  return { ...e, words, lead: wrap(e.head(words)) };
});
const block = entries.map((e) => `${e.lead}\n${quote(e.words)}\n`).join('');

if (!process.argv.includes('--check')) {
  const t = readFileSync(DEC, 'utf8');
  for (const e of entries) if (t.includes(e.key)) throw new Error(`already inserted: ${e.key}`);
  if (t.split(TOP).length !== 2) throw new Error('the list heading must stand once');
  writeFileSync(DEC, t.replace(TOP, TOP + block));
}

// checks
const t = readFileSync(DEC, 'utf8');
const before = execFileSync('git', ['show', 'origin/main:' + DEC], { maxBuffer: 1 << 26 }).toString('utf8');
const merge = execFileSync('git', ['log', '-1', '--format=%B', MERGE_60]).toString('utf8');
const ok = {};
let last = -1;
for (const e of entries) {
  const i = t.indexOf(e.lead);
  const got = i < 0 ? null : unquote(t.slice(i + e.lead.length + 1, i + e.lead.length + 1 + quote(e.words).length));
  ok[`${e.file}: in DECISIONS, unquoted, equal to the saved words`] = got === e.words;
  ok[`${e.file}: its lead holds its key`] = e.lead.replace(/\n {2}/g, ' ').includes(e.key);
  const note = readFileSync(NOTES + e.file, 'utf8');
  ok[`${e.file}: the notes file's own count or sha256 equals the words`] =
    note.includes(`sha256 ${sha(e.words)}`) || note.includes(`whole (${Buffer.byteLength(e.words)} byte)`);
  ok[`${e.file}: after the entry above it (newest first)`] = i > last;
  last = i;
  console.log(`${e.file}: ${Buffer.byteLength(e.words)} bytes, sha256 ${sha(e.words)}`);
}
const go60 = entries.find((e) => e.file.includes('merge-60')).words;
ok[`his go on #60, from "My words of" to the end, is a whole part of merge commit ${MERGE_60.slice(0, 7)}'s message`] =
  merge.includes(go60.slice(go60.indexOf('My words of 2026-11-18, 20:59')));
ok['the entries stand at the top of the list'] = t.includes(TOP + entries[0].lead);
ok["every other byte of main's file is unchanged"] = t.replace(block, '') === before;
ok['no lead line over 120 characters'] = entries.every((e) => e.lead.split('\n').every((l) => l.length <= 120));
ok['no carriage return'] = !t.includes('\r');
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
