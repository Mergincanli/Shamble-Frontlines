// docs-keeper (ci-after-3a): copies Mustafa's eight saved messages into docs/DECISIONS.md, newest first, never retyped.
// Each quote is the saved file's text after its "---" line (less the one trailing newline the saver adds), checked
// against the bytes and sha256 the saved file's own header states before anything is written. Run from the worktree
// root: node .checkpoints/ci-after-3a/docs-keeper-decisions.mjs [--check]
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/ci-after-3a/';
const DOC = 'docs/DECISIONS.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const CHAT = 'the chat he calls CI Builder, session_019YNMUUemXge56777MiUWFz, a Claude Code chat on Opus 5.5 for the small CI pull request after change-set 3a';
const AT = 'readable at commit 832268e of the chat\'s branch `claude/keen-tesla-v807cf`';

const E = [
  { file: 'mustafa-2026-11-25T1900-ask2-answers.md', before: '- **Taken by Mustafa, 2026-11-25, 13:55:08 UTC:',
    title: 'his answers to the CI chat\'s ask 2: CI-WRITE-2 (a), write both texts exactly as saved at e12bb442, the workflow then sha256 01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742 and the test configuration fed4b79d38cbbbf15cd69b0225e576dbfce880f9a69292096e74b1619b9e43ed, else stop and show him; FF-STOP (a), keep the 5-minute stop for the first run; FF-FILEURL (a), yes, with the comment that says so; GUARD-LOAD (a), `git merge --ff-only origin/main` in the main folder, alone in its command; main c1da1bd taken in once before the docs pass; on the first GitHub run, if the Firefox drawing tests fail on the page\'s "no WebGL" line, the firefox entry runs the rule tests only, by a change to the test configuration alone, and the drawing tests become a follow-up, while any other red, a time-out included, comes to him first; then the pull request\'s title and text into the notes and his Advisor opens the draft; until the second guard fix is merged, git and gh always start their own command',
    records: 'Records, not his words: the permission mode read right before the write: `default`, which his rule of 2026-11-23, 17:15:07 UTC covers; the workflow written at 18:02:02 UTC (the report `docs/reports/2026-11-25-ci-after-3a.md`, lines 47-57). At 52ad5d3 the workflow is sha256 01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742 and the configuration fed4b79d38cbbbf15cd69b0225e576dbfce880f9a69292096e74b1619b9e43ed, his two values (read with `git show` by docs-keeper, 2026-11-25). The first GitHub run on 52ad5d3 (toolchain 45670972698, red; its log read with gh by docs-keeper): Chromium "16 passed (3.5m)"; "libEGL.so.1: not in the loader cache of this runner (printed, not judged)"; "Mozilla Firefox 160.0"; in Firefox "4 failed", "9 did not run", "2 passed (5.0m)" after "Timed out waiting 300s for the test suite to run", the failures reading "This page could not start drawing: can\'t access property "getSupportedExtensions", this.gl is null". The run reached Playwright\'s 300 s stop as well as that line. The chat read the red as the case his point 6 names, not as a time-out of its own, because its container trial had the same shape (the drawing tests failing on that line, the rest not run at the stop: `trials-main-311c048.md`, lines 14-16, the case his FIREFOX-GL (a), point 2, names): the chat\'s reading, not his words. It then narrowed the firefox entry to `tests/rules/` by a change to the test configuration alone (832268e; the configuration then sha256 ef2136a14096fd12d5b9cce7b7fe28d611279850412a3269e5868139b19c9b0e, the workflow still 01898e4b...5742, both read with `git show`). With it `file-url.spec.ts` and the comment his FF-FILEURL (a) names left Firefox\'s run; FF-STOP\'s 5-minute stop stands in the workflow. On 832268e toolchain 45676587814 and gate 45676587871 (its gate and smoke jobs) are green (read with gh): Chromium "16 passed (3.4m)", Firefox "2 passed (15.0s)", the job 18:34:50 to 18:40:12 UTC. GUARD-LOAD and point 5: the main folder\'s head and the worktree\'s merge 3c02729\'s second parent both read c1da1bd (docs-keeper, with git).' },
  { file: 'mustafa-2026-11-25T1705-main-in.md', before: '- **Taken by Mustafa, 2026-11-25, 13:55:08 UTC:',
    title: '3b (#68, main 3ab9eaec) and the guard fix (#69) are merged: take today\'s main in once and go on (the test lists and timings again, the reviewers, then ask 2 with every CI text in one message); lane 1\'s builder is now Builder V4, a new chat, and nothing changes for this work; the interim rule wider until a second guard fix is merged: git and gh always start their own command, with no other program in front of them; one line on whether the session loads the fixed guard after taking main in',
    records: 'Records, not his words: main 311c048 (the merge of #69, after #68\'s 3ab9eae) taken in at 77d28ee (`trials-main-311c048.md`, line 1, in the same notes).' },
  { file: 'mustafa-2026-11-25T1455-firefox-gl.md', before: '- **Taken by Mustafa, 2026-11-25, 12:14:23 UTC:',
    title: 'FIREFOX-GL (a): ask 2 brings the full Firefox run with the read-only line about libEGL.so.1, and the first GitHub run is the drawing tests\' trial; decided now: if the Firefox drawing tests fail there with the same no-WebGL line, the Firefox entry of the test configuration goes to the rule tests only, the drawing tests become a follow-up, three lines to him, Chrome\'s drawing tests unchanged, and any other red or cause comes back to him first with options; no install line for the runner and no Firefox setting that switches off its graphics refusal without his word, each its own question in that follow-up; wait for #68\'s merge, take that main in and run the lists and timings again (one line to him if not merged by Thursday 26 November, 12:00 Amsterdam); `.gate/egl` deleted now; until the guard is fixed, no env, stdbuf or nice with options in front of git or gh',
    records: 'Records, not his words: how his point 2 was applied on GitHub is in the records of the entry of 2026-11-25, 18:00:43 UTC (above); the follow-up is FF-DRAW (`docs/STATUS_ARCHIVE.md`, "From ci-after-3a"); `.gate/egl` deleted (the chat\'s card).' },
  { file: 'mustafa-2026-11-25T1215-egl-trial.md', before: '- **Taken by Mustafa, 2026-11-25, 11:07:07 UTC:',
    title: 'EGL-TRIAL (a): yes to two library files (libegl1_1.7.0-1build1_amd64.deb and libegl-mesa0_25.2.8-0ubuntu0.24.04.5_amd64.deb, at the sizes and sha256 he gives) for the container\'s trial only, by the question\'s route, the folder deleted when the pull request is merged; ask 2 says which Firefox tests passed with them, adds one read-only CI line that prints whether the runner has libEGL.so.1 and brings no install line for the runner; a runner without it comes back as its own question; a Firefox test that still fails is read first, and a cause in the game\'s page is the three-line stop; every other file, version or checksum to him first',
    records: 'Records, not his words: with the two files the 2 rule tests passed and every drawing test still failed on "this.gl is null" (`question-2026-11-25-firefox-gl.md`, in the same notes).' },
  { file: 'mustafa-2026-11-25T0135-pw-runs-route.md', before: '- **Taken by Mustafa, 2026-11-24, 23:53:15 UTC:',
    title: 'PW-RUNS (a) with one change of route: Playwright started through `node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts`, as Builder V3 does, since the repository\'s deny list refuses npx and stays as it is; his yes at the permission question covers test runs and listing of the project\'s own configuration, not "install" or "install-deps"; if the node route is refused too, stop in three lines, no third route; ask 2 says in one line that CI keeps "npx --no --" and the trial ran the same program through node',
    records: 'Records, not his words: the trials ran Playwright that way (`trials-main-311c048.md`, in the same notes); CI keeps `npx --no --`.', queued: true },
  { file: 'mustafa-2026-11-25-pw-runs.md', before: '- **Taken by Mustafa, 2026-11-24, 23:53:15 UTC:',
    title: 'PW-RUNS (a): he allows the Playwright prompts',
    records: 'Records, not his words: the message he sent about a minute and a half later (the entry above) changes the route.' },
  { file: 'mustafa-2026-11-25T0120-ask1-answers.md', before: '- **Taken by Mustafa, 2026-11-24, 23:53:15 UTC:',
    title: 'FIREFOX-SOURCE (c): CI tests in GitHub\'s own Mozilla Firefox, with his yes in advance to fall back to Playwright\'s own Firefox only for a failure Playwright\'s route causes, never one the game\'s page causes, and a fallback named in the second ask with the failing lines; TRIAL-DOWNLOAD (a), for one file only, the Firefox 161.0 deb (90437572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1), by the plan\'s route; Playwright\'s Firefox zip stays unopened unless the fallback happens and is deleted when the pull request is merged without it; every other file, version or checksum to him first; a question he waits on is saved as a pushed notes file, named in the board post',
    records: 'Records, not his words: no fallback happened (the CI run used the runner image\'s Firefox 160.0; the entry of 2026-11-25, 18:00:43 UTC); the zip lies unopened under `.gate/` (the chat\'s card), to be deleted at the merge.' },
  { file: 'mustafa-2026-11-24-opening-block.md', before: '- **Taken by Mustafa, 2026-11-24, 22:44:50 UTC:',
    title: 'his opening block for a new builder chat he calls CI Builder: its one job, the small CI pull request after change-set 3a on main 28254a73 (a. every crate\'s release tests, the plan\'s E1; b. Firefox in CI, his FIREFOX-TESTS (b); c. the two vector generators\' --check, 3A-GEN-CHECK; d. the test-only WebAssembly module\'s own build step, 3A-RULES-STEP), and his eight rules for it (among them: plan first and the advisor agent\'s APPROVE, risk tier 2; every CI write asked of him first with its exact text and written after his yes, the permission mode read right before; every new download to him first; the fence; no pull request of its own: its title and text in the notes)',
    records: 'Records, not his words: the four parts are on the branch `claude/keen-tesla-v807cf` (standing in for `cs/ci-after-3a`), code head 832268e, part b in part (Firefox runs the rule tests only: the entry of 2026-11-25, 18:00:43 UTC).' },
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
