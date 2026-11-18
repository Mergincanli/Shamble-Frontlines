// The docs pass of kit-hold-ci: writes Mustafa's three chat messages of 2026-11-18 into docs/DECISIONS.md, newest first:
//   15:38:13 UTC  his go on the merge of #58, what he accepts with it, his answers to its five questions
//   16:39:23 UTC  his answer to KIT-TEXT (yes, with one change), the planted change, "nothing else in it", the stop rule
//   18:01:08 UTC  his answers to KIT-FIX (yes), KIT-REMOVED (stays held) and WATCH-60 (no); the size accepted
// His words are never retyped: they come from the three files saved by script from the session's transcript. The
// first is also checked byte for byte against the message of merge commit 9b4b518. Run from the repository root:
//   node .checkpoints/kit-hold-ci/decisions.mjs [--check]
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const DEC = 'docs/DECISIONS.md';
const NOTES = '.checkpoints/kit-hold-ci/';
const REPORT = process.env.KIT_HOLD_REPORT || 'docs/reports/2026-11-18-kit-hold-ci.md';
const MERGE = '9b4b518f520aa2360a8007dc857ba74b343c8a40';
const SHOWN = '3ebe654ab0992b216842b3470fe938f31d83a9f7';
const FIXED = 'f7e232fbfb78d41ec06f72f6444a8973096a1685';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const wordsOf = (file) => { const t = readFileSync(NOTES + file, 'utf8'); const i = t.indexOf('\n---\n'); if (i < 0) throw new Error('no --- in ' + file); return t.slice(i + 5).replace(/\n$/, ''); };
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
const wrap = (text, first, rest) => { const out = []; let line = first; let empty = true;
  for (const w of text.split(' ')) { if (!empty && line.length + 1 + w.length > 120) { out.push(line); line = rest + w; } else { line += (empty ? '' : ' ') + w; empty = false; } }
  out.push(line); return out.join('\n'); };
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const go = wordsOf('mustafa-2026-11-18-1624-merge-58.md');
const kit = wordsOf('mustafa-2026-11-18-1730-kit-text.md');
const fix = wordsOf('mustafa-2026-11-18-1852-kit-fix.md');
const shownSha = sha(execFileSync('git', ['show', `${SHOWN}:${NOTES}kit-edit.diff`], { maxBuffer: 1 << 26 }));
const headGo = wrap(`- **Taken by Mustafa, 2026-11-18, 15:38:13 UTC: his go on the merge of #58 (change-set 1, Types: the generated types, the block's layout table and its hash), what he accepts with it, his answers to its five questions, and what comes after** (${SESSION}; his label: "My words of 2026-11-18, 16:24 (Amsterdam time)"; extracted by script from the session's transcript, message 12484c8a-3a90-425f-b2a3-fbd2ced44837, whole). The five questions stand in \`docs/reports/2026-11-18-cs1-types.md\`, "Decisions needed": DEP-HOLD (how a dependency change is kept from being built before its security check), CI-TYPES (a type check and a regenerate-and-compare step in CI), CI-CUT (no cache in the toolchain job), LANE-ISSUE (who opens the "Lane requests" issue) and WATCH-58 (whether the build chat watches that pull request). The same words stand byte for byte in the message of merge commit ${MERGE} (2026-11-18, 15:39:28 UTC; parents 22c6afa and 1bfe462). Before the merge the session read the head again (1bfe462e339511cfae6b30a1015330d9ff2e718c) and \`gate\`, \`smoke\` and \`toolchain\` on it, for the push and for the pull request: green. Copied by script and checked byte for byte against that merge commit's message:`, '', '  ');
const headKit = wrap(`- **Taken by Mustafa, 2026-11-18, 16:39:23 UTC: his answer to the question KIT-TEXT (may the checkpoint script's hold on a dependency change be written with the exact text shown to him): yes, with one change, a manifest or lockfile whose content in the push equals its content on main is not held; and his yes to the planted change for CI's compare step, to the reading of "nothing else in it" and to the stop rule for the new CI lines** (${SESSION}; his label: "My words of 2026-11-18, 17:30 (Amsterdam time)"; extracted by script from the session's transcript, message 5f5cb1d6-18bd-43d9-beb6-fd9953e6c260, whole: its first paragraph is his instruction to the chat after a compaction). The text he answered on is \`${NOTES}kit-edit.diff\` at commit ${SHOWN} (262 lines, sha256 ${shownSha}); the question, that text, the lines changed after his answer, the planted change and the stop rule stand in \`${REPORT}\`. Copied by script (${Buffer.byteLength(kit)} bytes, sha256 ${sha(kit)}):`, '', '  ');
const fixSha = sha(execFileSync('git', ['show', `${FIXED}:${NOTES}kit-fix2.diff`], { maxBuffer: 1 << 26 }));
const headFix = wrap(`- **Taken by Mustafa, 2026-11-18, 18:01:08 UTC: his answers to three questions of the build chat, and the size accepted: KIT-FIX (may a fix to the checkpoint script's hold, in the lines changed after his first answer, be written with the exact text shown: yes, as shown), KIT-REMOVED (should a manifest that main removed also pass the hold when a branch takes main in: no, it stays held, and the question goes on the list for the quality gate v2's plan) and WATCH-60 (should the build chat watch pull request #60: no)** (${SESSION}; his label: "My words of 2026-11-18, 18:52 (Amsterdam time)"; extracted by script from the session's transcript, message 1eed5c8d-349d-471c-a9e3-106bfb012cc0, whole). The text he said yes to is \`${NOTES}kit-fix2.diff\` at commit ${FIXED} (45 lines, sha256 ${fixSha}); the three questions as asked, and that text, stand in \`${REPORT}\`. Copied by script (${Buffer.byteLength(fix)} bytes, sha256 ${sha(fix)}):`, '', '  ');
const TOP = '## Decisions, newest first\n';
const block = `${headFix}\n${quote(fix)}\n${headKit}\n${quote(kit)}\n${headGo}\n${quote(go)}\n`;
if (!process.argv.includes('--check')) {
  let t = readFileSync(DEC, 'utf8');
  if (t.includes('2026-11-18, 15:38:13 UTC: his go on the merge of #58') || t.includes('2026-11-18, 16:39:23 UTC: his answer to the question KIT-TEXT') || t.includes('2026-11-18, 18:01:08 UTC: his answers to three questions')) throw new Error('already inserted');
  if (t.split(TOP).length !== 2) throw new Error('the list heading must stand once');
  writeFileSync(DEC, t.replace(TOP, TOP + block));
}
// checks
const t = readFileSync(DEC, 'utf8');
const got = (head, words) => { const i = t.indexOf(head); return i < 0 ? null : unquote(t.slice(i + head.length + 1, i + head.length + 1 + quote(words).length)); };
const merge = execFileSync('git', ['log', '-1', '--format=%B', MERGE]).toString('utf8');
const before = execFileSync('git', ['show', 'origin/main:' + DEC], { maxBuffer: 1 << 26 }).toString('utf8');
const long = (headGo + '\n' + headKit + '\n' + headFix).split('\n').filter((l) => l.length > 120).length;
const ok = {
  'his go on #58, in DECISIONS, unquoted': got(headGo, go) === go,
  [`his go on #58 is a whole part of the message of merge commit ${MERGE.slice(0, 7)}`]: merge.includes(go),
  'his answer to KIT-TEXT, in DECISIONS, unquoted': got(headKit, kit) === kit,
  'the sha256 in the notes file of his answer equals the words': readFileSync(NOTES + 'mustafa-2026-11-18-1730-kit-text.md', 'utf8').includes(`sha256 ${sha(kit)}`),
  'the text he answered on has the sha256 his message names': kit.includes(shownSha),
  'his answers to KIT-FIX, KIT-REMOVED and WATCH-60, in DECISIONS, unquoted': got(headFix, fix) === fix,
  'the sha256 in the notes file of those answers equals the words': readFileSync(NOTES + 'mustafa-2026-11-18-1852-kit-fix.md', 'utf8').includes(`sha256 ${sha(fix)}`),
  'the fix he said yes to has the sha256 his message names': fix.includes(fixSha),
  'the three entries stand at the top of the list, newest first': t.indexOf(TOP + headFix) >= 0 && t.indexOf(headFix) < t.indexOf(headKit) && t.indexOf(headKit) < t.indexOf(headGo),
  "every other byte of main's file is unchanged": t.replace(block, '') === before,
  'no header line over 120 characters': long === 0,
  'no carriage return': !t.includes('\r'),
};
console.log(`his go on #58: ${Buffer.byteLength(go)} bytes, sha256 ${sha(go)}`);
console.log(`his answer to KIT-TEXT: ${Buffer.byteLength(kit)} bytes, sha256 ${sha(kit)}`);
console.log(`his answers to KIT-FIX, KIT-REMOVED, WATCH-60: ${Buffer.byteLength(fix)} bytes, sha256 ${sha(fix)}`);
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
