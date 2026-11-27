// CI Builder (ci-time): copies three saved messages of Mustafa into docs/DECISIONS.md, after docs-keepers script.
// retyped. Adapted from ci-after-3a's copy (git show 57fca475:.checkpoints/ci-after-3a/docs-keeper-decisions.mjs): only
// NOTES, CHAT, AT and the entries changed. Each quote is the saved file's text after its "---" line (less the one
// trailing newline the saver adds), checked against the bytes and sha256 the saved file's own header states before
// anything is written. Run from the worktree root: node .checkpoints/ci-preview-handover/docs-keeper-decisions.mjs [--check]
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/ci-time/';
const DOC = 'docs/DECISIONS.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const CHAT = 'the chat he calls CI Builder, session_019YNMUUemXge56777MiUWFz, a Claude Code chat on Opus 5.5 for CI work';
const AT = 'readable at commit 2afe425 of the chat\'s branch `claude/keen-tesla-v807cf`';
const ANCHOR = '- **Taken by Mustafa, 2026-11-25, 20:48:11 UTC:';

const E = [
  { file: 'mustafa-2026-11-27-full-paths.md', before: '- **Taken by Mustafa, 2026-11-26, 18:16:35 UTC:',
    title: 'a standing rule against needless guard prompts: in a shell command every file that is written, moved, copied or deleted is spelled out with its full path, with no variable, substitution or wildcard in its name, or the Write and Edit tools are used, so the guard can see it is not a kit or gate file; writes to kit and gate files still ask him first',
    records: 'Records, not his words: the CI chat follows it from this message on (its CI-TIME notes and commits).' },
  { file: 'mustafa-2026-11-27T1250-ci-time.md', before: '- **Taken by Mustafa, 2026-11-26, 18:16:35 UTC:',
    title: 'CI-TIME (a): `.github/workflows/toolchain.yml` line 38 `timeout-minutes: 15` becomes `timeout-minutes: 25`, this exact text approved in advance, nothing else in the file, the 5-minute Firefox stop stays; the main folder first fast-forwarded to main b9aad01; a plan for the advisor agent, the change and security-auditor\'s read; the chat opens the draft pull request if it can; on his go for the approved head\'s full id it marks it ready and merges with that id as the expected head, and a refused merge is told to him',
    records: 'Records, not his words: the main folder fast-forwarded to b9aad01; the line written at 12:43:24 UTC with the mode read right before as `acceptEdits`, the file then sha256 d1dd16d7eb850a8b27118eab51008b2e8ad97fcded27c468fa1d310f080ff11c (commit af127c2); draft pull request #78 opened by the chat through the GitHub tools; the advisor agent, security-auditor and infra-engineer approved (the report `docs/reports/2026-11-27-ci-time.md`).' },
  { file: 'mustafa-2026-11-26-74-merged.md', before: '- **Taken by Mustafa, 2026-11-26, 15:07:24 UTC:',
    title: '#74 is merged, main 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b; nothing more for the CI chat then; its board thread set to done',
    records: 'Records, not his words: main 3b949c1a\'s second parent is the chat\'s head e8340eb (read with git); the trial Firefox folder and the read-only copy of the upload action\'s code deleted at that merge.' },
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
