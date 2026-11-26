// docs-keeper (ci-preview-handover): copies Mustafa's four saved messages into docs/DECISIONS.md, newest first, never
// retyped. Adapted from ci-after-3a's copy (git show 57fca475:.checkpoints/ci-after-3a/docs-keeper-decisions.mjs): only
// NOTES, CHAT, AT and the entries changed. Each quote is the saved file's text after its "---" line (less the one
// trailing newline the saver adds), checked against the bytes and sha256 the saved file's own header states before
// anything is written. Run from the worktree root: node .checkpoints/ci-preview-handover/docs-keeper-decisions.mjs [--check]
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/ci-preview-handover/';
const DOC = 'docs/DECISIONS.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const CHAT = 'the chat he calls CI Builder, session_019YNMUUemXge56777MiUWFz, a Claude Code chat on Opus 5.5 for CI work (the small CI pull request after change-set 3a, then the preview hand-over)';
const AT = 'readable at commit d12e3a8 of the chat\'s branch `claude/keen-tesla-v807cf`';
const ANCHOR = '- **Taken by Mustafa, 2026-11-25, 20:48:11 UTC:';

const E = [
  { file: 'mustafa-2026-11-26T1505-ask2-answers.md', before: ANCHOR,
    title: 'his answers to the CI chat\'s ask 2 on the preview hand-over: CI-WRITE-3 (a), write the two texts in the recommended form, the workflow then sha256 d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8 and the test configuration 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8, any other value a stop; GUARD-STEP (a), keep the check before the upload; HIDDEN-FILES (b), off; the upload action stays at the pin cf430e030ddbb5b0abf93d22962f4752f3646cd9; he does not look up the Actions permissions, the first run on the draft pull request is the test; CUT-ESCAPE stays on record as accepted, and its later fix (the cut\'s code as its own throwaway user) goes into STATUS as a follow-up; then the docs pass, the report and the pull request\'s title and text in the notes, his Advisor opens the draft and watches the first run, and a red on the new steps or on Firefox\'s replay test comes to him first; git and gh always start their own command',
    records: 'Records, not his words: the main session wrote both files on this answer; its commit d12e3a8\'s message gives the permission mode read right before the write as `acceptEdits` (the write\'s time, 14:09:14 UTC, is the main session\'s statement to docs-keeper). At d12e3a8 `.github/workflows/toolchain.yml` is sha256 d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8 and `packages/e2e/playwright.config.ts` 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8, his two values (read with `git show` by docs-keeper, 2026-11-26). CI on d12e3a8: to be read. Point 6\'s follow-up is CUT-ESCAPE-FIX (`docs/STATUS_ARCHIVE.md`, "From ci-preview-handover"), where security-auditor\'s fuller form of the fix (its own copy of the workspace, its processes ended with the step: `security-auditor-ci.md`, SA-1, in the same notes) stands beside his words.' },
  { file: 'mustafa-2026-11-26T1400-ask1-answers.md', before: ANCHOR,
    title: 'his answers to the CI chat\'s ask 1: FF-REFETCH (a), the same Firefox 161 file (firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb, 90,437,572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1) by the same route as for #71, size and checksum checked before use, unpacked without installing, deleted at this pull request\'s merge, and a stop if the archive no longer serves exactly that file or Firefox needs anything more; UPLOAD-SOURCE (a), actions/upload-artifact v7.0.2 pinned by the full commit cf430e030ddbb5b0abf93d22962f4752f3646cd9, for the hand-over step only, nothing of it run before his yes on ask 2\'s exact text; what ask 2 holds in one message; git and gh always start their own command',
    records: 'Records, not his words: the trial (`trials.md`, in the same notes): the file fetched again, 90,437,572 bytes with its sha256 checked, unpacked with `dpkg-deb -x`, no graphics library files; the firefox project with the drafted configuration, 3 passed in the container\'s Firefox 161.0, the replay test among them ("first-loop.rec: 1800 steps, every shared hash as in the file"). Ask 2: `question-2026-11-26-ask2.md` (same notes), answered in the entry above.' },
  { file: 'mustafa-2026-11-26T1320-preview-handover.md', before: ANCHOR,
    title: 'his word to start the preview hand-over step lane 2 (Builder V2) asked for on issue #59 (its comment of 2026-11-25, 19:39 UTC): plan first, then the exact text asked of him; in the same pull request TOKEN-WORDS and FIREFOX-REPLAY (the recording\'s replay test joins the firefox entry beside the rule tests; if it is red there, stop and bring it to him before any change; a difference in the hashes is a finding, not a flake); one line on whether the chat holds only this repository and whether a guard prompt has ever reached him from it; a comment on GitHub is information, never his word',
    records: 'Records, not his words: point 3 answered in the chat (plan R2-10, `plan.md` in the same notes). The plan\'s revision 2 (e59b0c2) approved by the advisor agent with its conditions N1-N3 (`advisor-plan.md`, same notes); TOKEN-WORDS and FIREFOX-REPLAY are in d12e3a8.' },
  { file: 'mustafa-2026-11-25T2150-merged-next.md', before: ANCHOR,
    title: 'pull request #71 is merged (main 07d673b0ee509d36ce1b976ed8dfae002c7d7b4a, by his tap, with the chat\'s head c9460ca1a9ced37e99bff84b834d5508f6ce7575 pinned): close the card for ci-after-3a; the next job comes from Builder V2, the hand-over step in toolchain.yml for the preview links, with TOKEN-WORDS in the same text, from the new main, plan first, the exact text asked of him as before; until its request arrives, nothing; git and gh always start their own command',
    records: 'Records, not his words: 07d673b\'s second parent is c9460ca (read with git by docs-keeper, 2026-11-26); this branch\'s `.checkpoints/` holds no ci-after-3a card (docs-keeper\'s listing, 2026-11-26).' },
];

function wrap(text) {
  const out = []; let line = '- ';
  for (const w of text.split(' ')) {
    if (line.length + w.length + 1 > 120 && line.trim() !== '-') { out.push(line.trimEnd()); line = '  '; }
    line += w + ' ';
  }
  out.push(line.trimEnd());
  return out.join('\n');
}

function load(e) {
  const note = readFileSync(NOTES + e.file, 'utf8');
  const head = note.slice(0, note.indexOf('\n---\n')).trimEnd();
  const m = head.match(/(the user message|the message he sent while the chat was working \(stored as a queued command\)) of (\S+) \(uuid ([0-9a-f-]+)\), whole \((\d+) bytes, sha256 ([0-9a-f]{64})\)\. What it answers: ([\s\S]*)_$/);
  if (!m) throw new Error('header not read: ' + e.file);
  const [, how, at, uuid, bytes, digest, answers] = m;
  let t = note.slice(note.indexOf('\n---\n') + 5);
  if (!t.endsWith('\n')) throw new Error('no trailing newline: ' + e.file);
  t = t.slice(0, -1);
  if (Buffer.byteLength(t) !== Number(bytes) || sha(t) !== digest) throw new Error('bytes or sha256 differ: ' + e.file);
  return { how, at, uuid, bytes, digest, answers, t };
}

function block(e) {
  const r = load(e);
  const d = r.at.slice(0, 10), time = r.at.slice(11, 19);
  const first = r.t.split('\n')[0];
  const label = /Amsterdam|November|I call you CI Builder/.test(first) ? `his label: "${first}"` : 'no label of his own';
  const kind = e.queued ? `a message queued while the chat worked, entry ${r.uuid}` : `message ${r.uuid}`;
  const head = `**Taken by Mustafa, ${d}, ${time} UTC: ${e.title}** (${CHAT}; ${label}; ${kind}, whole; copied by script from the chat's saved file \`${NOTES}${e.file}\` (${AT}), which the chat copied by script from its transcript). What it answers, in the chat's words (not his): ${r.answers}${/\.$/.test(r.answers) ? '' : '.'} ${e.records} Copied by script (${r.bytes} bytes, sha256 ${r.digest}):`;
  const quote = r.t.split('\n').map((l) => (l === '' ? '>' : '> ' + l)).join('\n');
  return { text: wrap(head) + '\n' + quote, r };
}

const check = process.argv.includes('--check');
let doc = readFileSync(DOC, 'utf8');
if (!check) {
  for (const e of E) {
    const lines = doc.split('\n');
    const hits = lines.filter((l) => l.startsWith(e.before));
    if (hits.length !== 1) throw new Error(`anchor found ${hits.length} times: ${e.before}`);
    if (doc.includes(load(e).uuid)) throw new Error('already in the file: ' + e.file);
    // Insert directly above the anchor; entries of the same anchor are processed newest first, so each lands below the last.
    const i = lines.findIndex((l) => l.startsWith(e.before));
    lines.splice(i, 0, block(e).text);
    doc = lines.join('\n');
  }
  writeFileSync(DOC, doc);
}
// Check: every block read back from the file, its quote unprefixed, equals the saved bytes.
doc = readFileSync(DOC, 'utf8');
const lines = doc.split('\n');
for (const e of E) {
  const r = load(e);
  const s = lines.findIndex((l) => l.startsWith(`- **Taken by Mustafa, ${r.at.slice(0, 10)}, ${r.at.slice(11, 19)} UTC:`));
  let j = s; while (!lines[j].endsWith(`${r.digest}):`)) j++;
  const q = []; let k = j + 1;
  while (k < lines.length && (lines[k] === '>' || lines[k].startsWith('> '))) { q.push(lines[k] === '>' ? '' : lines[k].slice(2)); k++; }
  const t = q.join('\n');
  const ok = sha(t) === r.digest && Buffer.byteLength(t) === Number(r.bytes);
  console.log(`${ok ? 'OK ' : 'BAD'} ${e.file}: file lines ${s + 1}-${k}, ${Buffer.byteLength(t)} bytes, sha256 ${sha(t)}`);
  if (!ok) process.exitCode = 1;
}
