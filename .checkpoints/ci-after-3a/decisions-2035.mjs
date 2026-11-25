// CI Builder (ci-after-3a): copies Mustafa's answers of 20:35 Amsterdam, after docs-keeper's script for the eight earlier ones, into docs/DECISIONS.md, newest first, never retyped.
// Each quote is the saved file's text after its "---" line (less the one trailing newline the saver adds), checked
// against the bytes and sha256 the saved file's own header states before anything is written. Run from the worktree
// root: node .checkpoints/ci-after-3a/docs-keeper-decisions.mjs [--check]
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/ci-after-3a/';
const DOC = 'docs/DECISIONS.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const CHAT = 'the chat he calls CI Builder, session_019YNMUUemXge56777MiUWFz, a Claude Code chat on Opus 5.5 for the small CI pull request after change-set 3a';
const AT = 'readable at commit 9c9066c of the chat\'s branch `claude/keen-tesla-v807cf`';


const E = [
  { file: 'mustafa-2026-11-25T2035-rules-only.md', before: '- **Taken by Mustafa, 2026-11-25, 18:00:43 UTC:',
    title: 'RULES-ONLY-READ (a): he accepts the Firefox entry cut to the rule tests as made (packages/e2e/playwright.config.ts, sha256 ef2136a14096fd12d5b9cce7b7fe28d611279850412a3269e5868139b19c9b0e), the report kept as it is (one "no WebGL" line, three time-outs, the shared cause read from the code, not shown by the run), and next time a red his words send to him comes to him before the change; TOKEN-WORDS (a), with one change: it rides in the next workflow text the chat asks him for, the hand-over step for Builder V2\'s previews, at no yes of its own, and STATUS names that home; FF-DRAW-WHEN (a), with step 2.3, as its own question; push the last head and bring him its full commit id with the advisor agent\'s verdict, and the pull request\'s title and text, corrected for the red run\'s wording, in a file with its commit and path',
    records: 'Records, not his words: the advisor agent\'s read of 80359fc was REVISE on the docs only; its two Highs were fixed in f7bd1f2 (pushed with 679e5b5) before this answer.' },
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
