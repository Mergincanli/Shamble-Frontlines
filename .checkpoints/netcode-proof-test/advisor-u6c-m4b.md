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
