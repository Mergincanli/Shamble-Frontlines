# Advisor: U8-F1's head read (merge ddb06ff on H0 833c3f0, pushed in 08e8b0f), 2026-11-27, the 10:50 window

Plan: main's advisor.md; the commits (parents, the merge's file list and numstat against the patch, what 08e8b0f and
later add); CI on 08e8b0f; u8-f1-merge-plan.md's done lines M0-M9 against my C1-C3 and Q3's addition; u8-report.md's
U8-F1 section and :73-82; light-base.log and m5-check.log; exit-silent.md. Then one block each: 1 the merge, 2 the
smoke proposal, 3 EXIT-SILENT (a)-(c), 4 the worktree's cleanup; Decisions needed, For Mustafa, the status line.
Every check uses a printing form (EXIT-SILENT), quoted.

**Probe of this tool, done first:**
- `test -e <absent path>` alone printed "(Bash completed with no output)".
- The same with `; echo "test exit $?"` printed "test exit 1".
- `git diff --quiet 076233a f67cd9a; echo ...` printed "git diff --quiet exit 1".
- So EXIT-SILENT holds here too. Every claim below is read from printed output.

## 1. The merge: it stands; owed, one record line
What I read myself:
- **Parents:** `git log -1 --format='%H %P'` gives ddb06ffe… with parent 833c3f000a6c…, and 08e8b0f1… with parent
  ddb06ffe…. HEAD and origin both print 0bb93284aa30….
- **The merge's files** (`git diff --raw --no-renames 833c3f0 ddb06ff`): the six ops files, M, 100644 to 100644; then
  u8-f1-merge-plan.md and u8-report.md, M; then wip/u8-f1-build.patch, D. Nothing else.
- **Numstat under spikes/:** 8/2, 13/1, 17/6, 7/0, 17/2, 43/1, which is the patch's 105/12.
- **The committed tree is the tested tree:**
  - `git show 833c3f0:…/wip/u8-f1-build.patch | sha256sum` printed 4d6a248425ae…f7164.
  - `git diff 833c3f0 ddb06ff -- spikes/ | sha256sum` printed the same 4d6a248425ae…f7164.
  - That is also M5's header (.gate/u8-f1-merge/m5-check.log:3: "diff 4d6a2484…; untracked e3b0c442…", HEAD 833c3f0).
- **The six merged blobs** (`git ls-tree ddb06ff`): 2ad43cac, 84342b97, 24122c0e, c1384ca5, e77feb06, eb11e524, all
  100644, matching exit-silent.md item 4.
- **The premises, re-read by `git diff --name-only`; each printed nothing:**
  - 6968215..833c3f0 -- spikes/;
  - 77ffc95..833c3f0 -- spikes/netcode-proof/ops/;
  - ddb06ff..0bb9328 -- spikes/.
  - The control 076233a..f67cd9a printed ".checkpoints/netcode-proof-test.md", so an empty result is a real "equal".
- **After the merge:** `git diff --name-only ddb06ff 0bb9328` lists only .checkpoints/ paths (11 commits, all notes).
- **CI on 08e8b0f** (`gh run view`, each printed): gate push 45953185098, toolchain push 45953185158, gate
  pull_request 45953191100 and toolchain pull_request 45953191037. Each reads conclusion "success", attempt 1, headSha
  08e8b0f1….
- **M5 against my C1 bars, from the log:**
  - ops: "# tests 104", "# pass 104", "# fail 0" (:632-635);
  - wrappers: "wrappers.test.sh: 315 ok, 0 failed" (:960);
  - test: 931 and 1,028 passed, 0 failed, 0 skipped;
  - wasm: 27,000 ticks, 0 mismatching;
  - web: 70 of 70;
  - self, marks, refs and redact: equal to the base run (light-base.log:5-8: refs 2,145, marks 81, self 80 of 80,
    redact 11 of 11, 464 items, 0 findings; M5 :970-974, the same four);
  - "check: all 12 steps pass" (:975).
  - Every bar is met.
- **C2:** the card's PR #40 closing list holds SEC-U8F1-3 and SEC-U8F1-4 (card :115).
- **C3:** u8-report.md's U8-F1 section has all seven items and ADV-U8H-4's three corrections (:63-66, :73-82).
- **Q3's addition:** the five u8-f1/ files are on the lane (block 4).

**Owed, one record line (notes only, no round, no fix item):** u8-f1-merge-plan.md's done lines still record "exit 0"
for silent forms (M0's spikes check, M2, M3a, M8's after-commit check, `flock -n`, `test -d`). u8-report.md's U8-F1
section gains: "The done lines' silent checks are re-proved in printing forms by exit-silent.md items 1, 3-5 and 8,
and by the head read (advisor-u8-f1-head.md, block 1)". The done lines themselves stay as written, since they are the
record of what ran.

## 2. The smoke proposal (u8-report.md:80-81): yes, in a narrower, static form first
- **What it is for:** config.mjs's URL contract with U7's page has never been run (u8-report.md:77-80). A defect there
  would cost R1's browser sitting (his 80 minutes).
- **Why not U8's smoke.mjs as written:**
  - It needs the laptop's ~/npt-operator/browsers.json and recorded drivers.
  - lib.mjs fetches tokens from RS over SSH.
  - The page loads from config.mjs's PAGE_ORIGIN, `https://rs.nt.shamblefrontlines.com:31000` (config.mjs:61).
  - In the container each of those needs a stand-in, which is a new harness, not "U8's smoke.mjs".
- **The form, ruled:** a scratch check in the container, before Stop B's fill.
  - It is one node script under .checkpoints/netcode-proof-test/ (text) and writes no U8 file.
  - For every link R1's browser legs build (the slot table's suites, each browser and transport, attempts 0 and the
    last), it calls config.mjs's pure `pageUrl(...)` (config.mjs:63).
  - It feeds that URL's search and hash to U7's own exported `parseLink(search, hash)` (web/src/lib.ts:97), compiled
    by the web step into .gate/netcode-proof-test/web-check.
  - It asserts each parses `ok`, with the intended room, transport and token, and no field dropped.
  - It prints one line per link and a count.
  - The page's status texts that U8's PAGE reads are already pinned (web/test/lib.test.mjs:207).
- **Counts against no stop:** it is scratch, not one of U8's files. It goes in u8-report.md's bullet with its printed
  count.
- **If it finds a mismatch:** the fix is a change to config.mjs, which is U8's file and therefore under U8-FILLS'
  stop question. It comes to me with the printed lines, before Stop B.
- **The full live load** (a real browser on a loopback room and host) stays R1's first leg. A container harness for it
  is not asked for: it would be new code, for a check the static form covers in its contract part.

## 3. EXIT-SILENT
**(a) The rule for the 293 older uses: yes, with one concrete list added.**
- An older silent check stands where later printing evidence re-proved the same state: a status listing, hashes, a
  counted run, or CI.
- Any silent check a later step still relies on is re-run in a printing form before that step.
- A silent check that nothing re-proved and nothing relies on is moot. It is listed by count only, as exit-silent.md
  does.
- **Added, before PR #40 is marked ready:** one sweep over every unit merged into the lane. For each, the merge
  commit's spike diff is hashed by the pipe form in (b) and compared with its tested snapshot's sha256, or its M5 log
  header's diff hash.
  - U8 (3e1cd5b5, my head read), U7 (0a4a6eed, exit-silent.md) and U8-F1 (4d6a2484, block 1) are done.
  - The rest are the earlier lane merges: U4f, U4r, the U6 units, NB2 and the take-ins.
  - Why this list: that equality ("the reviewed snapshot is the merged code") is the state every head read leaned on.
    A later counted run proves the merged tree passes, not that it is the reviewed one.
  - One line per merge goes in a notes file, written by Write.
- The routines stay on "After the close", as exit-silent.md says.

**(b) The pipe `git diff <a> <b> -- spikes/netcode-proof | sha256sum`: accepted.**
- git starts the command, it only reads, nothing is written, and its output carries the answer (a hash).
- One rule with it: e3b0c442…b855, the hash of empty input, means git printed nothing. That is "no difference" only
  when no difference was expected. Where a difference was expected, the same hash is a failure: a bad revision, or git
  refused. In that case the stderr text is quoted, or `git diff --stat` of the same range is run beside it.

**(c) R1's own package: three exit-code reads, not two.** A wider grep (`--quiet`, `exit`, `grep -q`, `test -`,
`prints nothing` over operator/PROMPT-R1.md, operator/rules.md and runs/R1.md) finds one more than exit-silent.md
lists:
1. **PROMPT-R1.md:22, start check 4** (missed by exit-silent.md, whose grep looked for the word "exit"):
   - Today: "`git -C <run> diff --quiet origin/main -- CLAUDE.md .claude` holds".
   - It is the same guard as rules.md's: a planted CLAUDE.md or .claude in the run worktree.
   - Fix: "`git -C <run> diff --name-only origin/main -- CLAUDE.md .claude` prints nothing", rewrapped within :22-23.
2. **rules.md:14-15:**
   - Today: "(`git diff --quiet origin/main -- CLAUDE.md .claude`) … on a mismatch it stops".
   - Fix: "(`git diff --name-only origin/main -- CLAUDE.md .claude` prints nothing)", rewrapped within :14-15.
3. **PROMPT-R1.md:127:**
   - Today: "A finding, or any exit but 0, means no push".
   - redact-check always prints its summary line, "redact-check: <n> item(s) checked, <m> finding(s)". So the bar reads
     that output.
   - Fix: "Unless its saved last line reads `… 0 finding(s)` (an empty file included), no push", rewrapped within
     :123-129.

- **Why a printing form:** whether Claude Code's PowerShell tool on Windows hides an exit code the same way is not
  known. A printing form holds either way.
- **Deny check:** none of the three new forms names "hookspath", "push" or a `git config` write, so no deny matches
  them. operator.test.mjs 3.6 checks every backticked git command in PROMPT-R1.md against every deny; run it.
- **When: now, before Stop B, where he approves both files by hand.** Three net-zero rewordings in one small change by
  the main session.
  - This block is its plan check; it needs no security round. Each rewording makes an existing check observable and
    loosens nothing.
  - Its proof:
    - `git diff --numstat` shows added = removed for each of the two files;
    - operator.test.mjs and r1plan.test.mjs pass;
    - `node ops/operator/redact-check.mjs .` reports 0 findings;
    - a scratch printing of the three changed lines, quoted.
  - Net zero, so U8-FILLS does not touch it.
  - If any of the three cannot be rewrapped net zero, that one waits for U8-FILLS' answer and is named in the day's
    message. It does not pass the stop.

## 4. U8-F1's builder worktree: yes, by A's re-ruled form with my Q3 addition, after this read
**Step 1, notes on the lane:** done.
- Your list: `git ls-files` shows u8-f1-build.md and the five u8-f1/ files, with six sha256 equal in both places.
- My ffcheck run, from the main folder: `node .checkpoints/netcode-proof-test/ffcheck.mjs
  /home/user/Shamble-Frontlines/.claude/worktrees/agent-a7fe1b9f436f36bb1 77ffc95… 0bb9328…`. It printed:
  - "merge 77ffc95…..0bb9328…: 228 paths {"A":161,"M":64,"D":3}";
  - "ADDED paths present in the worktree: 6" (u8-f1-build.md and the five u8-f1/ files);
  - "MODIFIED/DELETED paths present: 67, absent: 0";
  - "differing from 77ffc95's blob: 6" (U8-F1's six ops files; exit-silent.md item 4 shows they equal ddb06ff's
    blobs);
  - "hash-object ids read: 67 (want 67)".

**The order:**
1. A1's ruling on U7's builder worktree. It binds both on MAIN-68's kit and CI files, which the fast-forward also brings
   (the 228 paths).
2. U7's builder's three commands, if A1 says go.
3. Then this worktree, in the same or a later window. One worktree's git steps at a time, each finished and its
   output quoted before the next starts.

**The form:**
- **ffcheck again**, just before, against the lane head of that moment. Expected: 6 ADDED present, 6 differing,
  nothing else. Any other result: stop.
- **The builder (a7fe1b9f436f36bb1), resumed in a window**, one command per call, with the two standing rules word for
  word:
  - `git add` of exactly those 12 paths;
  - `git merge --ff-only <lane head>`;
  - `git status --short; echo "status above"`. The echo marks the end, so "nothing above" is read, not assumed.
  - If git refuses at any step: stop and report its message. Remove, restore or move nothing.
- **Keep-first, the main session**, before any remove:
  - `cp -a` of the worktree's .gate/u8-f1/ (hooks-proof-1/ and s14-fixture/, the scratch behind item 2's and item 5's
    recorded proofs) into /home/user/Shamble-Frontlines/.gate/u8-f1/kept-agent-a7fe1b9f/;
  - `diff -rq` of the original and the copy, which prints nothing when equal, followed by an echo;
  - one sha256 list, written by Write into .checkpoints/netcode-proof-test/u8-f1-kept-list.md.
- **The remove:** `git worktree remove /home/user/Shamble-Frontlines/.claude/worktrees/agent-a7fe1b9f436f36bb1`, plain,
  from the main folder. Then `git worktree list`, quoted.
  - If it refuses, the fallback is (b): the worktree stays until PR #40's close, its evidence copied first.
  - Never `--force`, never `rm -rf`.

## Decisions needed
None new. U8-FILLS stays his (advisor-u8-f1-merge.md). Nothing here needs it: block 3 (c)'s three rewordings are net
zero, and block 2's check is scratch. One link: if block 2's check finds a config.mjs mismatch, its fix is a U8 line
and joins U8-FILLS.

## For Mustafa
1. Nothing to do now.
2. For the day's message, his to reverse: three operator checks in R1's package (start check 4, rules.md's worktree
   check, and the push's redaction check) are reworded to read printed output rather than an exit code. They are net
   zero, and he sees them at Stop B when he approves the two files.
3. Also for the day's message: before Stop B, a container check runs every R1 browser link through U7's page's own
   parser.

Status: COMPLETE 2026-11-27T11:02Z
