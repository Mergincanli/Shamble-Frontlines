# Advisor: plan review of the KEPT-LOGS kit rule draft (kept-logs-rule-draft.md), 2026-11-15

Seat: a day-session advisor, invoked from workflow wf_0f69bce5-688; advisor.md read from origin/main (4460ef5).
Branch claude/netcode-proof-test-plan-hcky5g at 4de29cf. Read-only except this file.

## Plan
1. Read advisor.md on main; his words (history.md:2088-2111); the draft whole (1,036 lines). DONE
2. Check scope against his two items; his three steps and stay-and-tell. DONE
3. Check the code's delete paths, the guard and settings facts the draft cites, step 2.3's overlap. DONE
4. Check the tests and the mutation table; set the risk tier and reviewers. DONE
5. Verdict, exact corrected lines, and the final text for Mustafa (below, blocks marked for extraction). DONE

## Checks (a line per check)
- K1 his words: history.md:2088-2111 read whole; scope = 2 items; 3 steps before a folder goes; stay-and-tell; kit PR to
  main; show every kit line, wait for yes; stop for the merge go; nothing packed or deleted before merge + main taken in.
- K2 the 2026-11-09 quote: history.md:404-406 "if it's ever needed, it becomes a narrow kit rule instead" (wrapped
  before "instead"): matches §11's quote once joined.
- K3 KIT-FREEZE-2: DECISIONS.md:29-37 on main, "The freeze stops new rules, new guard checks and kit redesign". His
  08:20 words order this kit PR by name: his own exception. A statement in the message, not a re-ask.
- K4 CHECKPOINTS.md on main: 229 lines; line 229 "- `rm -rf` stays denied." §11 appended after it changes no line.
- K5 step 2.3 (origin/claude/sweet-mayer-grsukw at 5f7f682, kit-texts.md): CHECKPOINTS.md edits K-7 (219-224) and
  K-14 (128) only; no §11, no `.claude/scripts/` file. Unchanged lines 225-229 separate K-7 from the append: no hunk
  overlap. Its guard (CS-D) and settings (S-1) work untouched by this draft.
- K6 settings.json on main: no allow line matches `node .claude/scripts/kept-logs.mjs` (allows: checkpoint.mjs *,
  checkpoint.test, routine-guard.test, repo-map-check, `node .gate/*.mjs`, ...); deny `Bash(rm -rf *)` at :247 stays.
  So each run asks (or auto mode's classifier decides), as the draft says.
- K7 guard on main: routine-guard.mjs:767-770 "scripts, inline code and the other writers are not inspected" (day
  sessions); interpreter() :989-1006 lets a routine run main's tracked scripts (pr-advisor runs none). Both routines
  carry SHAMBLE_ROLE (NIGHT_SHIFT.md:15, advisor.md:133; day sessions have none, routine-guard.mjs:4,11). So the
  script's own SHAMBLE_ROLE refusal is the routine stop, and it holds for both routines.
- K8 the folders, read-only (node, no writes): 10 set folders under runs/, all match SET_OK, each pairs with a
  committed results path holding its 12 hex digits; 195 files, all plain, all pass PATH_OK; longest archive name 63;
  biggest file 546 MiB; 11,684 MiB in all. Draft's figures hold.
- K9 the cache: .gate/u3c-review-target 4,051 MiB; newest entry 2026-11-08 22:40 (find -printf %T+, files and folders);
  nothing newer than 2026-11-15 07:20 UTC (find -newermt printed nothing); no registered worktree inside (git worktree
  list); nothing tracked.
- K10 the runner: c-u4r-runs.mjs:419 refuses when a set's logs folder OR results folder exists, and results are written
  last (:523, :583). So no runner writes into a set folder whose results are committed; a re-run after pack is refused.
  No re-check of the folder just before removal is needed.
- K11 redact-check on a list file: pure hex is never flagged (redact-check.mjs:51); HOSTNAME (:135, :229) flags only
  names that embed IPv4 quads; runs.txt already holds 64-hex sha256s. A list file passes.
- K12 repo-map-check.mjs:30-31 requires every `.claude/scripts/*` file in the map: the two new rows are needed. The
  CHECKPOINTS row change is not (the new kept-logs row names §11). Neither CI nor the gate runs repo-map-check or the
  kit tests (git grep .github: none): the session runs them.
- K13 the draft's code: both blocks pass `node --input-type=module --check` from stdin; 304 and 278 lines, widest 120.
- K14 delete paths in the draft: rmSync on (a) runs/<set> after the read-back, (b) the cache, (c) the archive's `.part`
  in the catch, and (d) a `.part` left by an earlier run (lines 283-287). (c) and (d) remove a file outside his two
  items, and (c) destroys the evidence of a mismatch he asked to be told about. Finding F1.
- K15 `cache` is a standing command: after the old cache goes, any later folder at that path would be removed too. His
  item 2 is "The one old build cache". Finding F2.
- K16 a folder that no longer matches its list (pre-pack, exit 2) says "the folder stays" but not "tell Mustafa"; that
  set can never be packed (a list is never overwritten), so he must hear of it. Finding F3.
- K17 tests: walked each of the 8 tests against the code; each assertion is reachable as written; mutations M1-M8 each
  hit the named test (M4 matters most: without the folder-equals-list check, an extra file is deleted unarchived).
- K18 5e563b95 (his last line): 2026-11-14T16:14:54Z, "carry 62 (h)", marks.json +15 and the card; checked by the
  advisor's carry-62 (h) review (advisor-u4r-c62h.md C1, C14: "right as committed"), after the r2-run ruling that
  ordered it (advisor-u4r-r2run.md item 6); check.mjs all 11 steps (c62h-check.log); no reviewer seat in a round
  (the session's checkpoint, netcode-proof-test.md:403-406, says the same).
- K19 my own slip: one probe file written by a shell redirect into the session scratchpad (outside the repository;
  rl-probe.txt, one line) before I re-read my limits. Nothing in the repository changed but this file.

## Verdict: APPROVE WITH CONDITIONS (C1-C9)

Risk tier 2 (`.claude/CHECKPOINTS.md`, `.claude/scripts/kept-logs.mjs`, `.claude/scripts/kept-logs.test.mjs`; plus
`docs/REPO_MAP.md`, tier 0 by itself). The later acts: removing a raw-log folder is reversible (the archive holds every
byte, hash-checked against the committed list); removing the cache is not, but it is rebuildable build output and his
words of 08:20 name it: his decision of the act is in. So no separate tier-3 step; the PR is needs-mustafa (kit), merged
from a day session on his go for its head.

Shape: right. One appended section, one new script and its tests, no guard, settings, CLAUDE.md or agent-file line;
clear of step 2.3. The script is the small place that makes "nothing else" and "every hash read back" true by code;
a text-only rule would rest on care alone. Size after the conditions: about 21 + 318 + 296 lines, all new.

## Findings
- F1 Medium (scope): the script removes its own unfinished archive (`.part`) on failure and at the next start: a third
  thing outside his two items, and on a mismatch it destroys the evidence he asked to be told about. Fix: C1.
- F2 Medium (scope): `cache` stays a standing delete of whatever sits at `.gate/u3c-review-target`; his item is "The one
  old build cache". Fix: C2 (refuse anything changed after his words, 2026-11-15 07:20 UTC).
- F3 Low (stay-and-tell): the pre-pack mismatch does not say "tell Mustafa". Fix: C3.
- F4 Low (small): the changed CHECKPOINTS row in REPO_MAP is not needed. Fix: C4.
- F5 Low (record): free space and the ruling should reach the report, not only the checkpoint; the rule should name
  the netcode proof test's folder. Fix: C5 (in §11's final text).
- F6 Low (message): KL-4 as a question re-asks what his words already say. Fix: C6.

## Conditions (exact lines; the final blocks below already carry them)
C1. No `.part` is ever removed (F1).
  - pack: the block "if (exists(part)) { if (!lstatSync... rmSync(part); say(...removed this script's unfinished...) }"
    becomes:
      if (exists(part)) {
        refuse(`${ARCHIVES}/${set}.tar.gz.part exists: an earlier pack did not finish, and it stays: tell Mustafa`);
      }
  - pack's catch drops `rmSync(part, { force: true });` and writes:
      process.stderr.write(`kept-logs: ${e instanceof Mismatch ? 'MISMATCH' : 'failed'}: ${e?.message ?? e}\n` +
        `kept-logs: the folder stays (${RUNS}/${set}), and so does any unfinished archive ` +
        `(${ARCHIVES}/${set}.tar.gz.part); tell Mustafa (CHECKPOINTS.md §11)\n`);
  - header: "the folder stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell
    Mustafa."; "no archive, finished or not"; "an existing list, archive or unfinished archive"; the sentence "The one
    other file it may remove is its own unfinished archive" goes.
  - test 4: asserts the `.part` stays, then `refused(pack(repo), /did not finish/)` and the `.part` still there.
  - mutations: M9 (the stale-.part refusal removed: test 4 fails), M10 (the catch removes the .part: test 4 fails).
C2. `cache` removes only the one old cache (F2).
  - const: `const OLD_BEFORE = Date.parse('2026-11-15T07:20:00Z');` with its comment.
  - function `newest(dir)` (lstat walk, never follows a link) and, last before the removal in cache():
      if (newest(dir) >= OLD_BEFORE) {
        refuse(`${CACHE} holds something changed after 2026-11-15 07:20 UTC: not the one old build cache; it stays`);
      }
  - header: "cache: removes .gate/u3c-review-target, the one old build cache (...), only while nothing in it is newer
    than his words (2026-11-15 07:20 UTC)."
  - test 7: a fresh planted cache is refused (/not the one old build cache/) and stays; `age(cache)` (a helper that sets
    every entry to 2026-11-08 22:40 UTC with utimesSync) then lets it go; imports gain lstatSync, readdirSync,
    utimesSync.
  - mutation M11 (the check removed: test 7 fails).
  - evidence at build, read-only: `find .gate/u3c-review-target -newermt '2026-11-15 07:20 UTC' | head -1` prints
    nothing (today it prints nothing: K9).
C3. pack's pre-pack mismatch: `refuse(\`the folder's files or sizes differ from ${listRel}: the folder stays; tell
    Mustafa (CHECKPOINTS.md §11)\`)` (one line); test 4's first refusal matches /differ from[\s\S]*tell Mustafa/; §11:
    "If one differs, or the folder no longer matches its list, the folder stays: tell Mustafa ..." (F3).
C4. REPO_MAP: only the two new rows (final text below); the CHECKPOINTS row stays as it is (F4).
C5. §11 is the final text below (F1-F3, F5): it names the netcode proof test's folder, the cache's date bound, "or
    the folder no longer matches its list", the ruling going into the checkpoint with free space and on into the report,
    and "The archives stay, finished or not".
C6. The message to Mustafa is the final text below: KIT-FREEZE-2 as one statement his yes confirms (no separate
    question); it says plainly that the tests pack and delete copies only, in a temp folder, and that the three places
    where the rule is narrower than his words are shown; it names 5e563b95 and the review that covered it (his words:
    "In your next report").
C7. Build evidence (after his yes, in `git worktree add .gate/kept-logs-wt -b <branch> origin/main`, never switching the
    main folder; the `.claude/` writes ask him first, in Accept edits mode): the draft's (c) steps 1-6, with
      - the mutation table M1-M11, each mutation's failing test names recorded;
      - the kit tests' pass counts on main 4460ef5 and on the PR head, equal (`node --test
        .claude/hooks/routine-guard.test.mjs`, `node --test .claude/hooks/settings.test.mjs`, `node --test
        .claude/scripts/checkpoint.test.mjs`), plus `node .claude/scripts/repo-map-check.mjs` and `node tools/gate.mjs`
        PASS;
      - the PR's three kit files extracted by script from the text he said yes to and compared byte for byte (`cmp`);
        the line counts measured on them;
      - the dry run's real-folder fingerprints equal before and after (draft (c)3).
C8. The PR report carries the tier-2 table: no existing rule sentence changes; one new rule (§11) is looser than §10's
    "`rm -rf` stays denied" for exactly his two items, with his words of 2026-11-15 quoted; and narrower than his
    words in three places (pack nothing more after a mismatch; the unfinished archive stays; the cache's date bound).
C9. Reviews and merge: security-auditor (required at tier 2) and test-engineer, one round, one batch of two; the
    advisor on every pushed head; if review changes a kit line, the merge message shows him those lines before and
    after; merged from a day session on his go for that head; nothing packed or deleted until the PR is on main and the
    netcode branch has taken main in. The list commits later are data on PR #40's branch, in their own commits, not
    part of the bot fix's line count.

Decided here (technical, inside his words): KL-1 (a) the list beside the set's results; KL-2 (a) no settings allow line;
KL-3 (a) kept, shown to him as a narrowing; KL-4 a statement, not a question; KL-5 (a) docs-keeper's next pass.

## The final text for Mustafa (the session pastes it whole; the blocks between the markers are the PR's exact lines)

<!-- BEGIN final-message -->
**KEPT-LOGS: every kit line, for your yes.** Nothing is built, packed or deleted yet.

This is the narrow kit rule of your words of 2026-11-15, 08:20. It covers your two items and nothing else:
- the raw-log folder of a run set whose results are committed and whose ruling is closed;
- the one old build cache, `.gate/u3c-review-target`.

Where it goes: one new section at the end of `.claude/CHECKPOINTS.md` (§11, 20 lines, no existing line changed) and one
new script with its tests. The rule itself is short. The script makes "nothing else" and "every hash read back" hold by
code, not by care. No change to the guard, `settings.json`, `CLAUDE.md` or any agent file, so it stays clear of step
2.3's guard change. `rm -rf` stays denied.

Your three steps, in the script's order (`list`, then `pack`):
1. `list` writes the folder's list (sha256, size, path per file) beside the set's committed results. I commit it,
   naming the closed ruling, and push it.
2. `pack` refuses unless that list is committed and pushed and the folder still matches it. It packs the folder into
   one `.tar.gz`, reads every file back out of that archive, hashes it, and compares it with the list.
3. It prints free space before and after; I copy both lines into the checkpoint, and from there into the report.

If one hash differs, the folder stays, the unfinished archive stays too (for you to look at), and I tell you.

Three places where the rule is narrower than your words (so you can strike them):
- after a mismatch, nothing more is packed until you answer;
- an unfinished archive is never deleted;
- the cache is removed only while nothing in it is newer than your words of 08:20; a later folder at that path stays.

KIT-FREEZE-2: this is a new rule during the freeze. I read your words of 08:20 ("It is its own reviewed kit pull
request to main") as your exception for it; your yes to these lines confirms that.

How it is proven, after your yes:
- its tests: 8, plus a dry run;
- 11 planted bugs, each of which must make a test fail;
- the dry run on copies of two real run-set folders in a temp folder: the tests pack and delete those copies only, and
  each real folder must be unchanged afterwards (its fingerprint, before and after);
- the kit's own tests with the same counts as on main; the gate.

Reviews: the advisor (this plan, then every pushed head), security-auditor and test-engineer. Then I stop for your go on
the merge. Nothing real is packed or deleted before the pull request is on main and this branch has taken main in.

Your last line: the pass-bar file's change of 2026-11-14, 16:14 UTC, was commit 5e563b95 ("carry 62 (h)"). It added one
check and one planted bug (whole-as-lost) to PM-0001-17 in `marks.json`. Two things covered it:
- the advisor's carry-62 (h) review (advisor-u4r-c62h: "right as committed"), after the r2-run ruling that ordered it;
- `check.mjs`, all 11 steps.
No reviewer seat reviewed it in a round.

### 1. `.claude/CHECKPOINTS.md`: appended after line 229, one blank line before it

<!-- BEGIN checkpoints-11 -->
```
## 11. Kept raw logs and the old build cache (Mustafa, 2026-11-15, KEPT-LOGS: a narrow rule)
His words: "What the rule may cover, and nothing else: 1. The raw-log folder of a run set whose results are committed
and whose ruling is closed. 2. The one old build cache, .gate/u3c-review-target." It is the narrow kit rule of his
words of 2026-11-09 ("if it's ever needed, it becomes a narrow kit rule instead"). `rm -rf` stays denied: only
`node .claude/scripts/kept-logs.mjs` removes either, and its header gives the commands.
- Who and when: the main session of a day session, in the main folder (the script refuses a routine and a linked
  worktree), never while a run set or a build runs: packing takes a core from timed runs, the list's commit under
  `spikes/netcode-proof/` stops a running set, and a build's cache can fill the disk under the archive.
- A raw-log folder (the netcode proof test's, under `.gate/netcode-proof-test/runs/`), one at a time: `list` writes
  its list (each file's sha256, size and path) beside the set's committed results; commit it, naming the set's closed
  ruling (file and line) in the message, and push it. `pack` then packs the folder into one compressed file under
  `.gate/netcode-proof-test/kept-logs/`, reads every file's hash back from it, and removes the folder only if every
  hash equals the list. If one differs, or the folder no longer matches its list, the folder stays: tell Mustafa, and
  pack nothing more until he answers.
- The cache: `cache` removes it only while nothing in it is newer than his words (2026-11-15 07:20 UTC); a later
  folder at that path is not the old cache and stays.
- Free space before and after, as the script prints it, goes into the checkpoint with the folder's ruling, and from
  there into the report. The archives stay, finished or not: nothing here removes one.
```
<!-- END checkpoints-11 -->

### 2. `.claude/scripts/kept-logs.mjs`: a new file, whole

<!-- BEGIN kept-logs.mjs -->
```js
#!/usr/bin/env node
// Kept raw logs and the one old build cache (.claude/CHECKPOINTS.md §11; KEPT-LOGS, Mustafa, 2026-11-15): the only way
// a session removes either. The main session of a day session runs it in the main folder, from the repository root:
//   node .claude/scripts/kept-logs.mjs list <set-folder> --results <results-folder>
//   node .claude/scripts/kept-logs.mjs pack <set-folder> --results <results-folder>
//   node .claude/scripts/kept-logs.mjs cache
// <set-folder>: a folder name under .gate/netcode-proof-test/runs/ that ends in the 12 hex digits of its run's commit.
// <results-folder>: the set's committed results under spikes/netcode-proof/results/, a path that holds those digits.
// list:  writes <results-folder>/raw-logs.sha256, one line per file of the set folder (sha256, bytes, path). The
//        session commits it, naming the set's closed ruling in the commit message, pushes it, then runs pack.
// pack:  checks that the list is committed and pushed and that the folder holds exactly its files and sizes; packs the
//        folder into .gate/netcode-proof-test/kept-logs/<set-folder>.tar.gz; reads every file back from that archive
//        and hashes it; and removes the folder only if every path, size and sha256 equals the list. If one differs,
//        the folder stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell Mustafa.
// cache: removes .gate/u3c-review-target, the one old build cache (cargo refuses it: it has no CACHEDIR.TAG), only
//        while nothing in it is newer than his words (2026-11-15 07:20 UTC).
// pack and cache print free space before and after; the session copies both lines into its checkpoint.
// It removes nothing else: no other path, nothing given as a pattern, nothing through a symbolic link, no archive,
// finished or not. It refuses a routine (SHAMBLE_ROLE set), a linked worktree, a set folder that holds anything but
// plain files and folders, and an existing list, archive or unfinished archive. git runs without a shell and only
// reads. Node 22, no dependencies.
// Exit codes: 0 done · 1 a check failed while working (the folder stays) · 2 usage error, or refused before any work.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { closeSync, createReadStream, createWriteStream, fsyncSync, lstatSync, mkdirSync, openSync, readdirSync,
  readFileSync, realpathSync, renameSync, rmSync, statfsSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pipeline as chain } from 'node:stream';
import { pipeline } from 'node:stream/promises';
import { fileURLToPath } from 'node:url';
import { createGunzip, createGzip } from 'node:zlib';

const RUNS = '.gate/netcode-proof-test/runs';
const ARCHIVES = '.gate/netcode-proof-test/kept-logs';
const CACHE = '.gate/u3c-review-target';
const RESULTS = 'spikes/netcode-proof/results/';
const LIST = 'raw-logs.sha256';
const SET_OK = /^[A-Za-z0-9][A-Za-z0-9-]{0,63}-([0-9a-f]{12})$/;
// A relative path of plain names: letters, digits, . _ and -, and no name that starts with a dot (so no . or ..).
const PATH_OK = /^[A-Za-z0-9_-][A-Za-z0-9._-]*(\/[A-Za-z0-9_-][A-Za-z0-9._-]*)*$/;
const MIB = 1048576;
// The one OLD build cache: a folder there with anything changed after his words (07:20 UTC) is not that one.
const OLD_BEFORE = Date.parse('2026-11-15T07:20:00Z');

export class Mismatch extends Error {}
const say = (msg) => process.stdout.write(`kept-logs: ${msg}\n`);
const refuse = (msg) => { process.stderr.write(`kept-logs: refused: ${msg}\n`); process.exit(2); };
const git = (...args) => {
  try {
    const out = execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      maxBuffer: 64 << 20 });
    return { ok: true, out: out.trim() };
  } catch { return { ok: false, out: '' }; }
};
const exists = (p) => { try { lstatSync(p); return true; } catch { return false; } };
const freeMiB = (top) => { const s = statfsSync(top); return Math.floor((s.bavail * s.bsize) / MIB); };
const total = (rows) => rows.reduce((n, r) => n + r.bytes, 0);

// The repository root of the main folder (never a linked worktree), in a day session.
function root() {
  if (process.env.SHAMBLE_ROLE) refuse('a routine never removes kept logs: day sessions only (CHECKPOINTS.md §11)');
  if (git('rev-parse', '--is-inside-work-tree').out !== 'true' || git('rev-parse', '--show-prefix').out !== '') {
    refuse('run from the repository root');
  }
  const real = (p) => realpathSync(resolve(p));
  if (real(git('rev-parse', '--git-dir').out) !== real(git('rev-parse', '--git-common-dir').out)) {
    refuse('run in the main folder: a linked worktree holds no kept logs');
  }
  return realpathSync(process.cwd());
}
// A folder under top, reached with no symbolic link on the way (its real path is the plain join).
function realDir(top, rel) {
  const abs = join(top, rel);
  let real = '';
  try { real = realpathSync(abs); } catch { refuse(`${rel} does not exist`); }
  if (real !== abs) refuse(`${rel} goes through a symbolic link (${real})`);
  if (!lstatSync(abs).isDirectory()) refuse(`${rel} is not a folder`);
  return abs;
}
// Every file under the set folder as { path, bytes }, sorted by path. Anything but plain files and folders refuses.
function walk(dir, set) {
  const files = [];
  const visit = (rel) => {
    for (const name of readdirSync(join(dir, rel))) {
      const path = rel ? `${rel}/${name}` : name;
      if (!PATH_OK.test(path) || `${set}/${path}`.length > 99) {
        refuse(`${set}/${path}: not a plain name of at most 99 characters`);
      }
      const st = lstatSync(join(dir, path));
      if (st.isDirectory()) visit(path);
      else if (st.isFile() && st.size < 8 ** 11) files.push({ path, bytes: st.size });
      else refuse(`${set}/${path} is not a plain file under 8 GiB (a symbolic link, a device or a socket stays)`);
    }
  };
  visit('');
  if (!files.length) refuse(`${set} holds no files`);
  return files.sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));
}
const listText = (set, rows) => `# ${RUNS}/${set}/: ${rows.length} files, ${total(rows)} bytes (sha256, bytes, path; ` +
  `.claude/scripts/kept-logs.mjs)\n${rows.map((r) => `${r.sha} ${r.bytes} ${r.path}\n`).join('')}`;
// The committed list, read only if it is exactly what list writes for this set.
function parseList(text, set) {
  const rows = text.split('\n').slice(1, -1).map((line) => {
    const m = /^([0-9a-f]{64}) (\d{1,15}) (\S+)$/.exec(line);
    if (!m) refuse(`${LIST}: "${line.slice(0, 80)}" is not "<sha256> <bytes> <path>"`);
    return { sha: m[1], bytes: Number(m[2]), path: m[3] };
  });
  if (listText(set, rows) !== text) refuse(`${LIST} is not the list this script writes for ${set}`);
  return rows;
}
// The set folder and its results folder, checked: results committed in ref (HEAD for list, the pushed branch for
// pack) and nothing uncommitted there. Returns the set folder's path and the list's path.
function checkSet(top, set, results, ref) {
  const m = SET_OK.exec(set ?? '');
  if (!m) refuse(`"${set}" is not a run-set folder name (<set>-<the 12 hex digits of its commit>)`);
  const res = String(results ?? '').replace(/\/$/, '');
  if (!res.startsWith(RESULTS) || !PATH_OK.test(res) || !res.split('/').includes(m[1])) {
    refuse(`--results must be the set's results folder under ${RESULTS}, a path that holds ${m[1]}`);
  }
  realDir(top, res);
  const listRel = `${res}/${LIST}`;
  if (!git('ls-tree', '-r', '--name-only', ref, `${res}/`).out.split('\n').some((f) => f && f !== listRel)) {
    refuse(`${res} holds no results committed in ${ref}`);
  }
  const status = git('status', '--porcelain', '--', res);
  if (!status.ok || status.out !== '') refuse(`${res} has changes that are not committed`);
  return { dir: realDir(top, `${RUNS}/${set}`), listRel };
}

async function list(top, set, results) {
  const { dir, listRel } = checkSet(top, set, results, 'HEAD');
  if (exists(join(top, listRel))) refuse(`${listRel} exists: a list is never overwritten`);
  const rows = [];
  for (const f of walk(dir, set)) {
    const hash = createHash('sha256');
    for await (const chunk of createReadStream(join(dir, f.path))) hash.update(chunk);
    rows.push({ ...f, sha: hash.digest('hex') });
  }
  writeFileSync(join(top, listRel), listText(set, rows), { flag: 'wx' });
  say(`list written: ${listRel} (${rows.length} files, ${total(rows)} bytes)`);
  say("next: commit it, naming the set's closed ruling (file and line) in the message; push it; then run pack");
}

// A POSIX ustar header for a plain file: name (at most 99 bytes), mode 644, owner 0, size and mtime in octal.
export function header(name, size, mtime) {
  const h = Buffer.alloc(512);
  const put = (text, at) => h.write(text, at, 'latin1');
  const oct = (n, width) => `${n.toString(8).padStart(width, '0')}\0`;
  put(name, 0); put(oct(0o644, 7), 100); put(oct(0, 7), 108); put(oct(0, 7), 116);
  put(oct(size, 11), 124); put(oct(mtime, 11), 136); put('        ', 148); put('0', 156);
  put('ustar\0', 257); put('00', 263);
  put(`${oct(h.reduce((n, b) => n + b, 0), 6)} `, 148);
  return h;
}
// Reads back a gzip'd ustar stream as pack writes it: plain-file entries, then zero blocks to the end. Returns
// [{ path, bytes, sha }]; anything else throws Mismatch.
export async function readBack(gz) {
  const got = [];
  let buf = Buffer.alloc(0), cur = null, pad = 0, zeros = 0;
  const field = (h, a, b) => h.subarray(a, b).toString('latin1').replace(/\0[\s\S]*$/, '');
  for await (const chunk of chain(gz, createGunzip(), () => {})) {
    buf = Buffer.concat([buf, chunk]);
    for (;;) {
      if (cur) {
        const n = Math.min(cur.left, buf.length);
        cur.hash.update(buf.subarray(0, n));
        buf = buf.subarray(n);
        cur.left -= n;
        if (cur.left) break;
        got.push({ path: cur.path, bytes: cur.bytes, sha: cur.hash.digest('hex') });
        pad = (512 - (cur.bytes % 512)) % 512;
        cur = null;
      }
      if (pad) {
        const n = Math.min(pad, buf.length);
        buf = buf.subarray(n);
        pad -= n;
        if (pad) break;
      }
      if (buf.length < 512) break;
      const h = buf.subarray(0, 512);
      buf = buf.subarray(512);
      if (h.every((b) => b === 0)) { zeros++; continue; }
      if (zeros) throw new Mismatch('data after the end of the archive');
      const sum = h.reduce((n, b, i) => n + (i >= 148 && i < 156 ? 32 : b), 0);
      if (parseInt(field(h, 148, 156).trim(), 8) !== sum) throw new Mismatch('a header checksum is wrong');
      if (field(h, 257, 263) !== 'ustar' || ![0x30, 0].includes(h[156]) || field(h, 345, 500) !== '') {
        throw new Mismatch(`an entry that is not a plain ustar file (${field(h, 0, 100)})`);
      }
      const bytes = parseInt(field(h, 124, 136).trim(), 8);
      if (!Number.isSafeInteger(bytes)) throw new Mismatch('a header size is not a number');
      cur = { path: field(h, 0, 100), bytes, left: bytes, hash: createHash('sha256') };
    }
  }
  if (cur || pad || zeros < 2 || buf.length) throw new Mismatch('the archive ends early');
  return got;
}

async function pack(top, set, results) {
  const branch = git('symbolic-ref', '--quiet', '--short', 'HEAD');
  if (!branch.ok) refuse("detached HEAD: pack runs on the branch that holds the set's results");
  const pushed = `refs/remotes/origin/${branch.out}`;
  const { dir, listRel } = checkSet(top, set, results, pushed);
  const onDisk = git('hash-object', '--', listRel).out;
  for (const ref of ['HEAD', pushed]) {
    const blob = git('rev-parse', '--verify', '--quiet', `${ref}:${listRel}`);
    if (!onDisk || !blob.ok || blob.out !== onDisk) {
      refuse(`${listRel} is not in ${ref} as it is on disk: commit it and push it first`);
    }
  }
  const rows = parseList(readFileSync(join(top, listRel), 'utf8'), set);
  const now = walk(dir, set);
  if (now.length !== rows.length || now.some((f, i) => f.path !== rows[i].path || f.bytes !== rows[i].bytes)) {
    refuse(`the folder's files or sizes differ from ${listRel}: the folder stays; tell Mustafa (CHECKPOINTS.md §11)`);
  }
  mkdirSync(join(top, ARCHIVES), { recursive: true });
  const archive = join(realDir(top, ARCHIVES), `${set}.tar.gz`);
  const part = `${archive}.part`;
  if (exists(archive)) refuse(`${ARCHIVES}/${set}.tar.gz exists: an archive is never overwritten`);
  if (exists(part)) {
    refuse(`${ARCHIVES}/${set}.tar.gz.part exists: an earlier pack did not finish, and it stays: tell Mustafa`);
  }
  const before = freeMiB(top);
  say(`free before: ${before} MiB`);
  if (before * MIB < total(rows) + 1024 * MIB) {
    refuse(`${before} MiB free is less than the folder's ${Math.ceil(total(rows) / MIB)} MiB plus 1024 MiB`);
  }
  try {
    await pipeline(async function* entries() {
      for (const r of rows) {
        const st = lstatSync(join(dir, r.path));
        if (!st.isFile()) throw new Mismatch(`${r.path} is no longer a plain file`);
        yield header(`${set}/${r.path}`, r.bytes, Math.floor(st.mtimeMs / 1000));
        let n = 0;
        for await (const chunk of createReadStream(join(dir, r.path))) {
          n += chunk.length;
          if (n > r.bytes) break;
          yield chunk;
        }
        if (n !== r.bytes) throw new Mismatch(`${r.path} changed size while it was packed`);
        if (r.bytes % 512) yield Buffer.alloc(512 - (r.bytes % 512));
      }
      yield Buffer.alloc(1024);
    }, createGzip(), createWriteStream(part, { flags: 'wx' }));
    const fd = openSync(part, 'r');
    try { fsyncSync(fd); } finally { closeSync(fd); }
    const got = await readBack(createReadStream(part));
    const bad = rows.filter((r, i) => got[i]?.path !== `${set}/${r.path}` || got[i].bytes !== r.bytes ||
      got[i].sha !== r.sha);
    if (bad.length || got.length !== rows.length) {
      throw new Mismatch(`read back from the archive: ${bad.length} of ${rows.length} files differ from the list ` +
        `(${got.length} entries): ${bad.map((r) => r.path).join(', ')}`);
    }
    renameSync(part, archive);
  } catch (e) {
    process.stderr.write(`kept-logs: ${e instanceof Mismatch ? 'MISMATCH' : 'failed'}: ${e?.message ?? e}\n` +
      `kept-logs: the folder stays (${RUNS}/${set}), and so does any unfinished archive ` +
      `(${ARCHIVES}/${set}.tar.gz.part); tell Mustafa (CHECKPOINTS.md §11)\n`);
    process.exit(1);
  }
  say(`archive: ${ARCHIVES}/${set}.tar.gz, ${lstatSync(archive).size} bytes from ${total(rows)}; read back: ` +
    `${rows.length} of ${rows.length} files equal the list (path, bytes, sha256)`);
  try { rmSync(dir, { recursive: true }); } catch (e) {
    process.stderr.write(`kept-logs: failed while removing ${RUNS}/${set} (${e.message}): the archive is complete ` +
      'and checked, and the folder may be partly removed; tell Mustafa\n');
    process.exit(1);
  }
  say(`removed: ${RUNS}/${set}`);
  say(`free after: ${freeMiB(top)} MiB`);
}

// The newest modification time of a folder and everything in it, never following a symbolic link.
const newest = (dir) => readdirSync(dir).reduce((max, name) => {
  const st = lstatSync(join(dir, name));
  return Math.max(max, st.isDirectory() ? newest(join(dir, name)) : st.mtimeMs);
}, lstatSync(dir).mtimeMs);

function cache(top) {
  const dir = realDir(top, CACHE);
  const tracked = git('ls-files', '--', CACHE);
  if (!tracked.ok || tracked.out !== '') refuse(`${CACHE} holds tracked files`);
  const wts = git('worktree', 'list', '--porcelain', '-z');
  if (!wts.ok) refuse('git could not list the worktrees');
  for (const w of wts.out.split('\0').filter((l) => l.startsWith('worktree ')).map((l) => l.slice(9))) {
    let real = w;
    try { real = realpathSync(w); } catch { /* a worktree that is gone keeps its path */ }
    if (real === dir || real.startsWith(`${dir}/`)) refuse(`${CACHE} holds the worktree ${w}`);
  }
  if (newest(dir) >= OLD_BEFORE) {
    refuse(`${CACHE} holds something changed after 2026-11-15 07:20 UTC: not the one old build cache; it stays`);
  }
  say(`free before: ${freeMiB(top)} MiB`);
  rmSync(dir, { recursive: true });
  say(`removed: ${CACHE}`);
  say(`free after: ${freeMiB(top)} MiB`);
}

async function main(args) {
  const [cmd, set, flag, results, ...extra] = args;
  if (cmd === 'cache' && args.length === 1) return cache(root());
  if ((cmd === 'list' || cmd === 'pack') && flag === '--results' && results && !extra.length) {
    const top = root();
    return cmd === 'list' ? list(top, set, results) : pack(top, set, results);
  }
  return refuse('usage: kept-logs.mjs list|pack <set-folder> --results <results-folder> | kept-logs.mjs cache');
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try { await main(process.argv.slice(2)); } catch (e) {
    process.stderr.write(`kept-logs: failed: ${e?.message ?? e}\n`);
    process.exit(1);
  }
}
```
<!-- END kept-logs.mjs -->

### 3. `.claude/scripts/kept-logs.test.mjs`: a new file, whole

<!-- BEGIN kept-logs.test.mjs -->
```js
// Tests for kept-logs.mjs: node --test .claude/scripts/kept-logs.test.mjs
// Each test builds a scratch repository with a bare "origin" in the system temp folder (a run-set folder of small files
// under .gate/, its results committed and pushed) and deletes it afterwards. KEPT_LOGS_SCRIPT points the tests at
// another copy of the script (used to prove the tests bite). KEPT_LOGS_DRY_RUN=<a real run-set folder's path> adds the
// dry run: a copy of that folder (only read) is listed, packed and removed in a scratch repository, and GNU tar's
// unpacking of the archive is compared file by file with the real folder.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { cpSync, createReadStream, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, renameSync,
  rmSync, symlinkSync, utimesSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, dirname, join } from 'node:path';
import { Readable } from 'node:stream';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { gzipSync } from 'node:zlib';

const HERE = dirname(fileURLToPath(import.meta.url));
const SCRIPT = process.env.KEPT_LOGS_SCRIPT || join(HERE, 'kept-logs.mjs');
const { header, readBack } = await import(pathToFileURL(SCRIPT).href);
const RUNS = '.gate/netcode-proof-test/runs';
const SET = 'u4r-P3-0123456789ab';
const RES = 'spikes/netcode-proof/results/u4r/0123456789ab/P3';
const ARCHIVE = `.gate/netcode-proof-test/kept-logs/${SET}.tar.gz`;
const FILES = { 'marks.json': '{}\n', 'l1-p3-s1/lab.jsonl': 'x'.repeat(1500), 'l1-p3-s1/room.jsonl': 'y'.repeat(512),
  'l1-p3-s2/empty.jsonl': '' };
const TAR = /GNU tar/.test(spawnSync('tar', ['--version'], { encoding: 'utf8' }).stdout ?? '');
const DRY = process.env.KEPT_LOGS_DRY_RUN;
const sha = (data) => createHash('sha256').update(data).digest('hex');
const fileSha = async (path) => {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest('hex');
};
const sh = (cwd, ...args) => {
  const r = spawnSync('git', args, { cwd, encoding: 'utf8' });
  if (r.status !== 0) throw new Error(`git ${args.join(' ')}: ${r.stderr}`);
  return r.stdout.trim();
};
const run = (cwd, args, role = '') => {
  const env = { ...process.env };
  delete env.SHAMBLE_ROLE;
  if (role) env.SHAMBLE_ROLE = role;
  return spawnSync(process.execPath, [SCRIPT, ...args], { cwd, encoding: 'utf8', env });
};
const list = (repo, set = SET, res = RES) => run(repo, ['list', set, '--results', res]);
const pack = (repo, set = SET, res = RES) => run(repo, ['pack', set, '--results', res]);
const refused = (r, re) => { assert.equal(r.status, 2, r.stderr + r.stdout); assert.match(r.stderr, re); };
const want = () => Object.keys(FILES).sort().map((p) => `${sha(FILES[p])} ${Buffer.byteLength(FILES[p])} ${p}`);
const plant = (set, files) => {
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(dirname(join(set, path)), { recursive: true });
    writeFileSync(join(set, path), text);
  }
};
const commit = (repo, path, message, push = true) => {
  sh(repo, 'add', path);
  sh(repo, 'commit', '--quiet', '-m', message);
  if (push) sh(repo, 'push', '--quiet', 'origin', 'claude/test');
};
// Sets a folder and everything in it to an old time (deepest first), as the one old build cache's files are.
const age = (dir, when = new Date('2026-11-08T22:40:00Z')) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (lstatSync(path).isDirectory()) age(path, when);
    else utimesSync(path, when, when);
  }
  utimesSync(dir, when, when);
};

function fixture(t, files = FILES) {
  const root = mkdtempSync(join(tmpdir(), 'kept-logs-test-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const origin = join(root, 'origin.git'), repo = join(root, 'repo');
  sh(root, 'init', '--quiet', '--bare', '-b', 'main', origin);
  sh(root, 'init', '--quiet', '-b', 'main', repo);
  for (const [k, v] of [['user.name', 'Test'], ['user.email', 'test@example.invalid'], ['commit.gpgsign', 'false']]) {
    sh(repo, 'config', k, v);
  }
  writeFileSync(join(repo, '.gitignore'), '.gate/\n');
  mkdirSync(join(repo, RES), { recursive: true });
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"PASS"}\n');
  sh(repo, 'add', '.gitignore', RES);
  sh(repo, 'commit', '--quiet', '-m', 'results');
  sh(repo, 'remote', 'add', 'origin', origin);
  sh(repo, 'switch', '--quiet', '-c', 'claude/test');
  sh(repo, 'push', '--quiet', '-u', 'origin', 'claude/test');
  const set = join(repo, RUNS, SET);
  plant(set, files);
  return { root, repo, set };
}

test('list, then pack: the list is right, every file is read back, the folder goes, free space is printed',
  async (t) => {
  const { repo, set } = fixture(t);
  const l = list(repo);
  assert.equal(l.status, 0, l.stderr);
  const lines = readFileSync(join(repo, RES, 'raw-logs.sha256'), 'utf8').trimEnd().split('\n');
  assert.equal(lines[0], `# ${RUNS}/${SET}/: 4 files, 2015 bytes (sha256, bytes, path; .claude/scripts/kept-logs.mjs)`);
  assert.deepEqual(lines.slice(1), want());
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list (ruling: a test)');
  refused(list(repo), /never overwritten/);
  const p = pack(repo);
  assert.equal(p.status, 0, p.stderr + p.stdout);
  assert.match(p.stdout, /free before: \d+ MiB[\s\S]*read back: 4 of 4 files[\s\S]*removed: [\s\S]*free after: \d+/);
  assert.equal(existsSync(set), false, 'the folder is removed');
  const got = await readBack(Readable.from([readFileSync(join(repo, ARCHIVE))]));
  assert.deepEqual(got.map((e) => `${e.sha} ${e.bytes} ${e.path.slice(SET.length + 1)}`), want());
  plant(set, FILES);
  refused(pack(repo), /never overwritten/);
});

test('GNU tar unpacks the archive to the same bytes, and readBack reads an archive GNU tar wrote',
  { skip: !TAR && 'GNU tar is missing' }, async (t) => {
    const { root, repo } = fixture(t);
    assert.equal(list(repo).status, 0);
    commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
    assert.equal(pack(repo).status, 0);
    const out = join(root, 'unpacked');
    mkdirSync(out);
    const x = spawnSync('tar', ['-xzf', join(repo, ARCHIVE), '-C', out], { encoding: 'utf8' });
    assert.equal(x.status, 0, x.stderr);
    for (const [path, text] of Object.entries(FILES)) assert.equal(readFileSync(join(out, SET, path), 'utf8'), text);
    const made = spawnSync('tar', ['--format=ustar', '-czf', '-', '-C', join(out, SET), ...Object.keys(FILES).sort()]);
    assert.equal(made.status, 0, String(made.stderr));
    const got = await readBack(Readable.from([made.stdout]));
    assert.deepEqual(got.map((e) => `${e.sha} ${e.bytes} ${e.path}`), want());
  });

test('pack refuses a list that is not committed, or committed but not pushed; the folder stays, no archive', (t) => {
  const { repo, set } = fixture(t);
  assert.equal(list(repo).status, 0);
  refused(pack(repo), /not committed/);
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list', false);
  refused(pack(repo), /not in refs\/remotes\/origin\/claude\/test/);
  assert.ok(existsSync(join(set, 'marks.json')));
  assert.equal(existsSync(join(repo, ARCHIVE)), false);
});

test('a folder that changed after its list: an extra file or a new size refuses (2), the same size with other bytes ' +
  'fails on read-back (1); the folder and the unfinished archive stay, and the next pack refuses', (t) => {
  const { repo, set } = fixture(t);
  assert.equal(list(repo).status, 0);
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
  writeFileSync(join(set, 'extra.log'), 'late\n');
  refused(pack(repo), /differ from[\s\S]*tell Mustafa/);
  rmSync(join(set, 'extra.log'));
  writeFileSync(join(set, 'marks.json'), '{}\n\n');
  refused(pack(repo), /differ from/);
  writeFileSync(join(set, 'marks.json'), '[]\n');
  const c = pack(repo);
  assert.equal(c.status, 1, c.stderr + c.stdout);
  assert.match(c.stderr, /MISMATCH[\s\S]*1 of 4 files differ[\s\S]*marks\.json[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(set, 'marks.json'), 'utf8'), '[]\n', 'the folder stays');
  assert.equal(existsSync(join(repo, ARCHIVE)), false);
  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'the unfinished archive stays for Mustafa to see');
  refused(pack(repo), /did not finish/);
  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'and the next pack leaves it');
});

test('refused before any work: a routine, the usage, a bad set name or results path, results not committed, not the ' +
  'root, a linked worktree', (t) => {
  const { root, repo, set } = fixture(t);
  refused(run(repo, ['list', SET, '--results', RES], 'night-shift'), /routine/);
  refused(run(repo, ['cache'], 'pr-advisor'), /routine/);
  for (const args of [[], ['pack', SET], ['list', SET, '--results', RES, 'more'], ['remove', SET, '--results', RES]]) {
    refused(run(repo, args), /usage/);
  }
  for (const bad of ['..', `${SET}/..`, 'u4r-P3', 'u4r-P3-0123456789AB', `../runs/${SET}`, '-rf']) {
    refused(list(repo, bad), /not a run-set folder/);
  }
  for (const res of ['spikes/netcode-proof/results/u4r/ffffffffffff/P3', '.gate/x/0123456789ab',
    `spikes/netcode-proof/results/../../../${RUNS}/0123456789ab`]) refused(list(repo, SET, res), /--results/);
  mkdirSync(join(repo, 'spikes/netcode-proof/results/u4r/0123456789ab/P2'));
  refused(list(repo, SET, 'spikes/netcode-proof/results/u4r/0123456789ab/P2'), /no results committed/);
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"FAIL"}\n');
  refused(list(repo), /not committed/);
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"PASS"}\n');
  refused(run(join(repo, 'spikes'), ['list', SET, '--results', RES]), /repository root/);
  sh(repo, 'worktree', 'add', '--quiet', '-b', 'claude/wt', join(root, 'wt'));
  refused(run(join(root, 'wt'), ['list', SET, '--results', RES]), /main folder/);
  assert.ok(existsSync(join(set, 'marks.json')));
  assert.equal(existsSync(join(repo, RES, 'raw-logs.sha256')), false, 'no list was written');
});

test('odd entries refuse and stay: a symbolic link anywhere, a pipe, a long, spaced or hidden name', (t) => {
  const { root, repo, set } = fixture(t);
  symlinkSync(join(set, 'marks.json'), join(set, 'link.json'));
  refused(list(repo), /not a plain file/);
  rmSync(join(set, 'link.json'));
  symlinkSync(join(set, 'l1-p3-s1'), join(set, 'l1-p3-s1-link'));
  refused(list(repo), /not a plain file/);
  rmSync(join(set, 'l1-p3-s1-link'));
  if (spawnSync('mkfifo', [join(set, 'pipe')]).status === 0) {
    refused(list(repo), /not a plain file/);
    rmSync(join(set, 'pipe'));
  }
  for (const name of ['a'.repeat(90), 'a b.log', '.hidden']) {
    writeFileSync(join(set, name), 'x');
    refused(list(repo), /plain name/);
    rmSync(join(set, name));
  }
  const away = join(root, 'away');
  renameSync(set, away);
  symlinkSync(away, set);
  refused(list(repo), /symbolic link/);
  assert.ok(existsSync(join(away, 'marks.json')), 'the link target stays');
  rmSync(set);
  renameSync(away, set);
  assert.equal(list(repo).status, 0, 'the folder as it was lists');
});

test('cache removes the one old .gate/u3c-review-target and nothing else; it refuses a link, a tracked file, a ' +
  'worktree inside, anything changed after his words, more words', (t) => {
  const { root, repo, set } = fixture(t);
  const rel = '.gate/u3c-review-target', cache = join(repo, rel);
  refused(run(repo, ['cache']), /does not exist/);
  plant(cache, { 'debug/libx.rlib': 'x' });
  refused(run(repo, ['cache', 'now']), /usage/);
  sh(repo, 'worktree', 'add', '--quiet', '-b', 'claude/in-cache', join(cache, 'wt'));
  refused(run(repo, ['cache']), /holds the worktree/);
  sh(repo, 'worktree', 'remove', join(cache, 'wt'));
  sh(repo, 'add', '--force', `${rel}/debug/libx.rlib`);
  refused(run(repo, ['cache']), /tracked files/);
  sh(repo, 'rm', '--cached', '--quiet', `${rel}/debug/libx.rlib`);
  refused(run(repo, ['cache']), /not the one old build cache/);
  assert.ok(existsSync(join(cache, 'debug/libx.rlib')), 'a newer cache stays');
  age(cache);
  const ok = run(repo, ['cache']);
  assert.equal(ok.status, 0, ok.stderr);
  assert.match(ok.stdout, /free before: \d+ MiB[\s\S]*removed: \.gate\/u3c-review-target[\s\S]*free after: \d+ MiB/);
  assert.equal(existsSync(cache), false);
  assert.ok(existsSync(join(set, 'marks.json')), 'the run-set folder is untouched');
  const away = join(root, 'away');
  plant(away, { keep: 'x' });
  symlinkSync(away, cache);
  refused(run(repo, ['cache']), /symbolic link/);
  assert.ok(existsSync(join(away, 'keep')), 'the link target stays');
});

const entry = (name, body) => Buffer.concat([header(name, body.length, 0), body,
  Buffer.alloc((512 - (body.length % 512)) % 512)]);
const tarGz = (...blocks) => Readable.from([gzipSync(Buffer.concat(blocks))]);
test('readBack reads a plain archive and refuses a wrong checksum, another entry type, a short end and data after ' +
  'the end', async () => {
  const body = Buffer.from('hello\n'), end = Buffer.alloc(1024);
  assert.deepEqual(await readBack(tarGz(entry('s/a.txt', body), end)), [{ path: 's/a.txt', bytes: 6, sha: sha(body) }]);
  const flipped = entry('s/a.txt', body);
  flipped[0] ^= 1;
  await assert.rejects(readBack(tarGz(flipped, end)), /checksum/);
  const folder = entry('s/d/', Buffer.alloc(0));
  folder[156] = 0x35;
  folder.fill(32, 148, 156);
  folder.write(`${folder.reduce((n, b) => n + b, 0).toString(8).padStart(6, '0')}\0 `, 148, 'latin1');
  await assert.rejects(readBack(tarGz(folder, end)), /not a plain ustar file/);
  await assert.rejects(readBack(tarGz(entry('s/a.txt', body))), /ends early/);
  await assert.rejects(readBack(tarGz(entry('s/a.txt', body), end, entry('s/b.txt', body))), /after the end/);
});

test('dry run on a copy of a real run-set folder: listed, packed, read back, removed; GNU tar unpacks the same bytes',
  { skip: (!DRY || !TAR) && 'set KEPT_LOGS_DRY_RUN=<a real run-set folder> (GNU tar needed)' }, async (t) => {
    const name = basename(DRY), commit12 = /-([0-9a-f]{12})$/.exec(name)?.[1];
    assert.ok(commit12 && existsSync(DRY), `${DRY} is a run-set folder`);
    const { root, repo } = fixture(t, {});
    const res = `spikes/netcode-proof/results/dry-run/${commit12}`;
    plant(join(repo, res), { 'verdict.json': '{}\n' });
    commit(repo, res, 'dry-run results');
    cpSync(DRY, join(repo, RUNS, name), { recursive: true, preserveTimestamps: true });
    const t0 = Date.now();
    const l = list(repo, name, res);
    const t1 = Date.now();
    assert.equal(l.status, 0, l.stderr);
    commit(repo, `${res}/raw-logs.sha256`, 'dry-run list');
    const t2 = Date.now();
    const p = pack(repo, name, res);
    const t3 = Date.now();
    assert.equal(p.status, 0, p.stderr);
    assert.equal(existsSync(join(repo, RUNS, name)), false, 'the copy is removed');
    const out = join(root, 'unpacked');
    mkdirSync(out);
    const x = spawnSync('tar', ['-xzf', join(repo, `.gate/netcode-proof-test/kept-logs/${name}.tar.gz`), '-C', out]);
    assert.equal(x.status, 0, String(x.stderr));
    const rows = readFileSync(join(repo, res, 'raw-logs.sha256'), 'utf8').trimEnd().split('\n').slice(1);
    for (const row of rows) {
      const [hash, , path] = row.split(' ');
      assert.equal(await fileSha(join(DRY, path)), hash, `the real ${path} equals the list`);
      assert.equal(await fileSha(join(out, name, path)), hash, `GNU tar's ${path} equals the list`);
    }
    t.diagnostic(`${name}: ${rows.length} files; list ${t1 - t0} ms, pack ${t3 - t2} ms`);
    t.diagnostic(p.stdout.trim());
  });
```
<!-- END kept-logs.test.mjs -->

### Not kit, in the same pull request: two new rows in `docs/REPO_MAP.md`, after the `checkpoint.test.mjs` row

<!-- BEGIN repo-map-rows -->
```
| `.claude/scripts/kept-logs.mjs` | KEPT-LOGS (`CHECKPOINTS.md` §11): `list` writes a closed run set's raw-log list beside its results; `pack` packs the folder, reads every file's hash back and removes the folder only if all equal the committed list; `cache` removes the one old build cache, `.gate/u3c-review-target`; it refuses everything else |
| `.claude/scripts/kept-logs.test.mjs` | Its tests: `node --test .claude/scripts/kept-logs.test.mjs`; `KEPT_LOGS_DRY_RUN=<a real run-set folder>` adds the dry run on a copy |
```
<!-- END repo-map-rows -->

Not in this pull request: `settings.json`, the guard, `CLAUDE.md`, the agent files, `docs/DECISIONS.md` (docs-keeper
records these words in its next pass; §11 quotes your scope itself).

**Decisions needed:**
- **KL-YES: do these kit lines go in exactly as shown?** Why it matters: you asked to see every kit line and to say yes
  before anything is built. Options: (a) yes, as shown; (b) yes, with changes you name (for example, strike one of the
  three narrowings above); (c) no. Recommendation: (a). The advisor approved them with conditions, and all of those
  conditions are already in the text above.

**For Mustafa:**
1. Read the three kit blocks (§11, the script, the tests) and the two map rows.
2. Say "yes" (or what to change) in this chat. Nothing is built before that, and nothing is packed or deleted before
   the pull request is on main and this branch has taken main in.
<!-- END final-message -->
