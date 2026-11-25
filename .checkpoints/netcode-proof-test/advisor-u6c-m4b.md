# Advisor: U6c merge step M4b, head read of the hand edits (2026-11-25)

Seat: the advisor agent, called from the main session; rules read from `git show origin/main:.claude/agents/advisor.md`.
Read only, effort xhigh. Brief: u6c-m4b-request.txt (sha256 d124e576… checked, matches). My ruling: advisor-u6c-r2.md
(sections 3-5 read).

## Plan
1. Check the brief's sha256 (done: matches), read main's advisor rules and my round-2 ruling's sections 3-5 (done).
2. hunks.diff: its sha256 against 9a3bbeb7…; its file list (exactly three files); each hunk against section 3's text.
3. The working tree: `git status --short` under spikes/netcode-proof (24 files); the three edited files diffed against
   the worktree's copies; the other 21 cmp-equal to the worktree (cmp per file).
4. The fmt log (m4a-fmt.log).
5. Section 3's "check by reading" items, in the main folder's probe.rs after the edits.
6. Mustafa's yes and condition (history.md:7902-7924), and the condition checked in the code (the leg's loop from
   `zero`, the edges around warm-up and window).
7. The two extra questions (a compile-only probe before M5; M6's fallbacks).
8. Verdict, conditions, For Mustafa. End with DONE.

## Notes
- Head: bae6232 on claude/netcode-proof-test-plan-hcky5g. `git diff --stat e83d9fb HEAD`: five files, all under
  .checkpoints/ (notes only, as the launch note says). `git diff --stat 8c81952 HEAD -- spikes/netcode-proof`: empty.
- hunks.diff sha256 9a3bbeb7faec8aa928460fe34643c971cd4dd5e84d4f5f30d372f3758407b846: matches the brief.
  Files: crates/bots/tests/load.rs (+4 -3), crates/server/src/probe.rs (+7 -3), marks/SERIES.md (+1 -1). Exactly three.
- hunks.diff against section 3, hunk by hunk: H2a, H2b, H1a, H1b, H1c, H1d each equal section 3's old and new text
  character for character (indent, punctuation, line breaks). No rustfmt change was needed (m4a-fmt.log: `cargo fmt
  --all -- --check` exit 0 at 00:21:48 UTC, HEAD e83d9fb, free 10,256 MiB).
- Live check, not only the file: my own `diff -u` of the worktree's copy against the main folder's, for each of the
  three files, gives the same hunks as hunks.diff (only the labels and timestamps differ). So hunks.diff is the tree.
- `git status --short -- spikes/netcode-proof`: 12 modified, 12 untracked entries (crates/host/tests/ holds one file,
  args.rs): 24 files, as at M4.
- `diff -rq` of the whole spikes/netcode-proof folder, worktree against main folder: only load.rs, probe.rs and SERIES.md
  differ; the one other line is web/node_modules, present only in the main folder and gitignored (.gitignore:1). So the
  other 21 files, and every tracked file besides, are byte-equal to the worktree. Stronger than a 21-file cmp.
- mtimes: all 156 .rs files under crates/ carry 00:21:48.146 UTC; SERIES.md 00:21:27. Cause read in main-run.sh:48-52
  (it touches every file under crates/ before and during each command, for cargo's fingerprints). Benign; content is
  what the diffs check.
- Check by reading, main folder's probe.rs after the edits (line numbers are the main folder's; the brief's :224-225 and
  :541-543 are the worktree's, 4 lines up):
  - `before` (:498) is read before `drop(sock)` (:575): FP-2's compile hold stands (edge_events borrows the socket, :180).
  - `port` is bound at :465, before :498 and the end read :557; a u16, Copy.
  - `before` appears in run_leg only at :498 (its read) and :657 (`before: before.into()`); grep of the file finds no
    other use (the other hits are the module doc, the struct field :338, its writer :348, and comments).
  - The Err path (:487-493) returns before :498 and names no `before`; on that path no start edge is read now, and none
    was ever written (the old read was dropped with the early return). edge_events reads /proc files only (:180-196):
    moving it has no side effect on the socket.
  - H2: `transport` is still used (:137, :140, :187, :215), and :171's `measured.len() >= 2` keeps `len() - 1` safe;
    :179 still fails an empty slice. The WT test runs 4 s (:235), the WS one 30 s (:240).
- His condition, in the code: `zero` is `s.now_ns()` read in send_down just before `Pacer::start` (net l2/probe.rs:423-424,
  returned at :442). The new read (:498) follows send_down's return, the match, nothing else: so it lands a sliver after
  `zero` (the pacer thread's start plus the /proc read's own opening; not measured, on the order of tens to hundreds of
  microseconds). The loop runs from `zero` (:544-550: start `zero`, end `zero + warm + window + tail`; seconds_loop
  :225-262 counts its seconds from `zero`); the warm-up is [zero, zero + warm) (:607, :620; leg_series's window start
  `zero + warm`, :569-570); the end read (:557) is after the loop and the poller's stop. So the whole measured window, the
  tail and the warm-up after that sliver lie between the edges.
  - The sliver: a drop counted in it needs the kernel buffer full at that instant, which needs the reader a full buffer
    behind (about 28 ms at R1's rate, my ruling, item 5), so its stall began before `zero`: the pre-zero backlog his (a)
    stops counting. A stall that begins at or after `zero` overflows the buffer only after the read, so every one of its
    drops is between the edges. His condition holds in substance; the report should say "from a sliver after the leg's
    zero" rather than "from the zero".
- fmt ran on this exact tree: `git diff HEAD -- spikes/netcode-proof | sha256sum` gives 6c3b4dbb…11ec2 and the untracked
  hash (main-run.sh:42's form, run in spikes/netcode-proof) 5cb9a300…f33a, both equal to m4a-fmt.log:3. So the tree
  rustfmt checked is the tree I read, and these two hashes are the ones M5's log header must show.
- Compile, by reading (no cargo run): the moved line is the old line unchanged; `sock` (an Arc) is only borrowed by the
  poller (:478) and by the `started` closure, whose borrows end at :486; `before` ([EventRecord; 2]) still meets
  `before.into()` at :657. The new comment lines are `//`, not doc text; H1a's continuation lines keep the list's
  4-space indent. probe_tests.rs:299-312 checks both edges' presence (row found, the drops key, cpu_user), not timing.
- For the two questions: fix-check.log (the round-2 tree, worktree, 13:27-13:50 UTC 11-24): clippy 25.8 s warm, test
  1,176.3 s, "861 tests passed, 0 failed, 0 reported a skip; bites build: 958 … 0 reported a skip", wasm 27,000 / 0,
  refs 1802 (this head has more tracked files: not comparable), marks 81, self 78/78, redact 0 findings. The builder
  used `node check.mjs --only clippy,doc` as a pre-check (.gate/u6c/fp6-fmt-clippy.log:6). The card (checkpoint :148):
  debug profile 212 MiB, so M5 is near-cold and may fill about 7.8 GB. U6b's precedent (advisor-u6b-m6.md:74-76,
  :166-168): a test failing in a counted run stops the merge, no re-run to green; a run cut by a container restart with
  no result was run again (:374-375). check.mjs runs every step even after a FAIL (check.mjs:2675-2693).
- His words: history.md:7906, the main session's word-for-word record of his chat message (transcript record
  97389ddd…, 00:07:40 UTC): "(a), yes, the one-line move, with your advisor's read, in U6c's merge. The 10 seconds of
  warm-up and the measured window stay fully counted. The 214 drops inside P1's window on your own machine stay recorded
  as unexplained; this answer does not cover them." I read his chat only through that record. The code change is one
  moved line; the rest of H1 is its comment, the module doc and SERIES.md's wording. It matches "the one-line move".
- The sliver, read (after the advisor tool's check): `Pacer::start` (net l2/probe.rs:326-375) spawns its thread and
  blocks on `rx.recv()` (:365-367) until the thread has read /proc/thread-self, written and read back its
  /proc/<tid>/timerslack_ns and read its /proc/self/task/<tid>/stat (thread.rs:20-61; no sleep, no lock). So the read
  comes after the new thread has run three /proc operations: normally well under a millisecond; under CPU contention
  the thread's first wake can take milliseconds. A drop is missed only if the read lands about 28 ms or more after the
  start of a stall that began at or after the zero, i.e. only if the stall also holds the main thread back that long
  at the leg's first instant. The "about 28 ms" is carried from my round-2 ruling (item 5), not re-derived here: npt-net
  sets no receive-buffer size on ImpairSocket (grep of crates/net/src for rcvbuf finds only tests and tcp_kernel), so
  the kernel default holds; not read on R1's machines.
- The strictly-before-zero form (the read after `set_peer`, before `send_down`) means splitting the `started` closure:
  several changed lines, more than "the one-line move" he said yes to, and new text needing its own head read (no
  advisor before 08:50). So: approve what he approved, and give him the sliver plainly (section 8), with the exact form
  as an option.

## 1. Verdict: APPROVE WITH CONDITIONS (M5 may start once C1-C3 hold)
The hand edits are exactly my round-2 section 3 text, with no rustfmt change. They touch only the three named files,
and every other file in the spike is byte-equal to the builder's worktree. Section 3's four reading checks hold.
His condition holds for the whole measured window, and for the whole warm-up except its first moment: the read now
comes just after the leg's zero, not before it. By reading that moment is normally under a millisecond; it is not
measured (section 4).
- He said yes to (a) as my question described it: "move the read to the leg's start", a one-line move. The exact line
  placement is my section 3 text (H1c), which his "with your advisor's read" sends to this read. That placement does
  not stop M5.
- The residual is ADV-M4B-1, labelled Low (below). He gets it as one plain entry (NPT-EDGE-SLIVER, section 8).

**Findings of this read:**
- **ADV-M4B-1 (Low): the warm-up's first moment lies outside the edges.** That moment runs from the zero to the read:
  the pacer thread's start, then the /proc read. A drop there is missed only if a stall begins in it and also holds
  the read back by about 28 ms (carried, not re-measured).
  - Not High under "a rule loosened beyond his words": the placement is the one his (a) described, the 10 s stay
    counted save that moment, and he is told.
  - Not Medium: its reach is a stall of about 28 ms or more that lands on a sub-millisecond moment, once a leg.
  - Taker: none if (a) stands (closed by his answer). If he picks (b), the small follow-up in section 8 takes it.
- **ADV-M4B-2 (Low, wording): "at the leg's zero" should read "just after the leg's zero".** It appears in H1a's
  module-doc line, H1c's comment and SERIES.md:915. It does not change M5 and triggers no push on its own.
  - Taker: (b) if picked; otherwise a STATUS follow-up for the next change-set that edits
    crates/server/src/probe.rs or marks/SERIES.md.

## 2. Section 4's head read
- hunks.diff sha256 9a3bbeb7…7b846: matches.
- Exactly three files: crates/bots/tests/load.rs +4 -3, crates/server/src/probe.rs +7 -3, marks/SERIES.md +1 -1. My
  section 4's "two with H1" was a miscount; with H2 it is three, as the brief says. SERIES.md:915 is one of U6c's own
  added rows.
- Each hunk equals section 3's text character for character (H2a, H2b, H1a, H1b, H1c, H1d). There is no rustfmt
  difference: `cargo fmt --all -- --check` gave exit 0 (m4a-fmt.log:5), and the two tree hashes show that check ran on
  this exact tree.
- It is the live tree, not only the file: my own `diff -u` of the three files against the worktree gives the same hunks.
- The other 21 files: `diff -rq` of the whole spikes/netcode-proof folder (worktree against main folder) shows only the
  three files. The one other line is the gitignored web/node_modules. So all 21, and every tracked file besides, are
  byte-equal.
- HEAD bae6232: everything after e83d9fb is notes only (five .checkpoints/ files), and the spike is unchanged since
  8c81952. The set is still 24 files: 12 modified, 12 untracked.
- The size is +5 net lines, within my "about 6". No test is added or removed, and nothing under marks.json or
  crates/judge is touched (D10 holds).

## 3. Section 3's "check by reading" (main folder's probe.rs)
- `before` (:498) is read before `drop(sock)` (:575), so FP-2's compile hold stands.
- `port` is bound at :465.
- Nothing reads `before` between :498 and its one use at :657.
- The Err path (:487-493) returns before :498 and needs no `before`.
- edge_events only reads /proc files (:180-196), so moving it changes nothing on the socket.
- H2: `transport` is still used, `measured.len() >= 2` (:171) keeps `- 1` safe, and an empty slice still fails (:179).

## 4. His condition: "The 10 seconds of warm-up and the measured window stay fully counted"
- **What the code does.**
  - `zero` is the socket clock read in send_down just before `Pacer::start` (net l2/probe.rs:423-424).
  - `Pacer::start` waits until its new thread has done three /proc operations (:365-367).
  - Then comes the match, and then the read at :498.
  - The loop runs from `zero` to `zero + warm + window + tail` (:544-550). The end read (:557) comes after it and after
    the poller stops.
- **What that means.**
  - The measured window and the tail lie fully between the edges.
  - So does the warm-up, except the moment from `zero` to the read. Normally that is well under a millisecond. It is
    not measured, and it can stretch under CPU load.
- **What could be missed.**
  - A buffer only overflows after the reader has fallen about 28 ms behind (carried from my round-2 ruling).
  - So a drop from a stall that starts in the warm-up is missed only if that same stall also holds this read back by
    about 28 ms or more, right at the leg's first instant.
  - Every stall that starts later in the warm-up, or in the window, is counted.
- **The main session's reading** ("the warm-up and the window stay between the edges") is right except for that first
  moment. The report must say so (C5), not "fully" without the caveat.
- **The strictly exact form** reads just before the pacer starts, so before `zero`. It means splitting the `started`
  closure: more than the one-line move he approved, and new text needing a head read. It is offered to him as an
  option, not built now.

## 5. Conditions, one line each
- **C1:** M5's log header (and a probe's) shows exactly the tree I read, equal to m4a-fmt.log:3. Anything else: stop.
  - diff 6c3b4dbbd5e9c9fafda5ee316eb16ce4b61de12e8db1bf5aa14483b141711ec2
  - untracked 5cb9a30097a402787e5d685bf70cb97ee816709eaeaac411f16ea7206006f33a
- **C2:** right before M5, at least 8,192 MiB free. After a probe (Q1), at least 8,192 MiB minus the probe's own use
  (its header's free minus a `df` right after it). Otherwise the approved dev part-clean first, then at least 8,192 MiB.
- **C3:** M5 starts only once the beside-running checks (step 7's reruns, Netcode Builder 2's plan check) have ended,
  with `cat /proc/loadavg` before and after it in the log (`::`, U6b's form).
- **C4:** M5 passes only as ruled:
  - 11 of 11 steps;
  - measured 861 and bites 958, 0 failed, 0 reported a skip (fix-check.log:9);
  - wasm 27,000 ticks, 0 mismatching; marks 81; self 78/78; redact 0 findings.
  The refs count is not compared, because this head tracks more files.
- **C5:** the report states his condition as section 4 does: the read lands just after the leg's zero, normally under
  a millisecond by reading, not measured. It names ADV-M4B-1 and -2, and keeps his sentence on the 214 in-window drops
  ("stay recorded as unexplained") word for word.
- **C6:** before any fallback in section 7, copy the M5 log and every .gate/netcode-proof-test/failed-steps/*.log into
  .gate/u6c-merge/. check.mjs deletes a step's kept log when that step next runs (check.mjs:2677).

## 6. Question 1: a compile-only probe before M5
- **Yes. It is optional, not counted, and replaces no M5 step.** The command, from the main session, in the
  background, timeout 3600000:
  `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/u6c-merge/m4c-probe.log env CARGO_INCREMENTAL=0 node check.mjs --only fmt,clippy,doc`
  Then run `df -B1M --output=avail /home/user` for C2.
- **Why this form and not `cargo check`:** these are M5's own first steps.
  - They cover both builds (the bites build is `--all-features`, which a plain `cargo check` skips), with warnings as
    errors and all targets.
  - rustdoc checks the changed module doc.
  - With the same CARGO_INCREMENTAL=0, M5 reuses what the probe builds, so its disk is an advance on M5's, not extra.
  - The builder used the same form (.gate/u6c/fp6-fmt-clippy.log:6).
- **Worth it?** A failure is unlikely: the moved line is unchanged and its types and borrows are the same, by reading.
  But in M5 itself a compile error would not stop the run: check.mjs runs every later step anyway, filling disk on a
  near-cold target.
- **Probe PASS:** C2, then M5.
- **Probe FAIL, or its header not C1's:** no M5 and no edit. Copy the failed step's log (C6), then stop and wait for me.

## 7. Question 2: M6's fallbacks
- **(a) Counts other than 861 / 958, a failed test, a skip, or any C4 value off.** M5 has not passed.
  - Stop: no M7-M9, no commit, no edit, no re-run.
  - On its own, read-only: C6's copies; the test step's line against fix-check.log:9; the header hashes; the loadavg
    lines; the probe's log; a checkpoint note and a request file for me.
  - The applied tree stays uncommitted as it is, and the worktree is kept.
  - Netcode Builder 2's merge does not start while U6c's is held ("the two merges never overlap", history.md:7909).
- **(b) One timing-sensitive test failing once.** The same as (a).
  - No re-run to green, and no re-run of that test before my ruling. U6b's precedent: re-running a counted check to
    green is not evidence, and a second failure on a correct tree is (advisor-u6b-m6.md:74-76, :166-168).
  - Collect, read-only:
    - the test's name, its panic and file:line (failed-steps/test.log);
    - whether it is H2's test (tests/load.rs `hosts_four`) or another in U6c's diff;
    - the test step's time against 1,176.3 s;
    - the loadavg lines, and what else ran during M5.
  - It may run the approved dev part-clean after C6's copies if free is under 8,192 MiB, so a run I rule can start at
    once. Nothing else.
- **(c) A disk stop.**
  - **Before M5** (C2 not met, or main-run.sh exits 3 before any command): the approved dev part-clean with no lane
    running, re-measure, then M5 at 8,192 MiB or more. Still under after the part-clean: stop and wait. Delete
    nothing else; the kept logs are evidence.
  - **During M5:** the run counts as void (neither a code failure nor a pass) only if two things hold.
    - The first error names "No space left on device" or "os error 28". The source is the failing step's kept log or,
      when that log could not be written on a full disk (check.mjs:2685-2691), the step's last 40 lines that check.mjs
      printed into the M5 log. If neither shows the first error: stop and wait.
    - No other test fails without that error.
    Record a `df` right after.
  - Then: C6's copies, the part-clean, at least 8,192 MiB, and one full M5 again on C1's hashes. The report names both
    logs.
  - A run cut with no result is treated the same way: one full M5 again (U6b's run 2, advisor-u6b-m6.md:374-375).
    "Cut with no result" means the log has no "--- end" line, after a container restart or the session's limit.
  - A second void run, or a mixed case (a failure without the literal disk error), falls under (b): stop and wait.
- **Stop and wait for me on:**
  - any FAIL that is not (c)'s literal case;
  - any C4 value off;
  - header hashes not C1's;
  - a second void run;
  - under 8,192 MiB after the part-clean;
  - main-run.sh exit 4 or 5;
  - any edit needed under spikes/;
  - anything that would touch marks.json or crates/judge.
- **After a passing M5:** M7-M9 as my round-2 section 5 rules. No further call to me before M9.

## 8. Decisions needed
- **NPT-EDGE-SLIVER (new; no answer needed unless you want (b)).**
  - The question: each probe leg's first reading of dropped packets now comes just after the leg starts, not before
    it. By reading of the code that gap is normally under a millisecond; it is not measured. Is that close enough to
    your "the 10 seconds of warm-up … stay fully counted"?
  - Why it matters: drops from a stall that starts in that gap are missed only if the stall also holds the reading
    back by about 28 ms (a figure carried from the round-2 ruling, not re-measured). Everything after the gap is
    counted.
  - Options:
    - (a) Keep it as merged: your (a), "the one-line move", placed as the round-2 ruling's text.
    - (b) A small follow-up before R1 that takes the reading just before the leg starts. That is a few changed lines,
      with its own plan, the advisor's check and one counted run.
      - Last day to pick it: the end of Sat 28 November (Amsterdam). It must land before Stop B's fill on Tue 1
        December, which records R1's build hashes (the card, :190-191).
  - Recommendation: (a).

## 9. For Mustafa
1. Nothing to do for U6c's merge now. Read NPT-EDGE-SLIVER (section 8) and answer only if you want (b).

## 10. What I did not check
- I compiled and ran nothing (no cargo, test, gate or plant). Compile and lint correctness are by reading; the probe or
  M5 shows them.
- His chat I read only through history.md:7902-7924, the main session's word-for-word record.
- The sliver's length is not measured. The "about 28 ms" is carried from my round-2 ruling, not re-derived, and the
  receive buffer was not read on R1's machines.
- I did not read cmp_files.py; `diff -rq` of the whole spike folder stood in for it. I did not compare the worktree's own
  HEAD; the content comparison covers it.
- Not read: MAIN-65's step 7 results, Netcode Builder 2's plan, U8's side.
- Method:
  - Several calls were compound against the brief's one-simple-command line (`cd … &&`, pipes into `sha256sum`); all
    were read-only.
  - The guard refused one git call with `-c`/`--git-dir`.
  - One `find` from the repository root ran past 120 s and I stopped it (read-only).

DONE
