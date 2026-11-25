# security-auditor, review round 2 of change-set 4a "The recording" (the last round unless a Critical or High is open)

Head ec005ad (source as at 07903d9). Diff: `git diff eb14739..ec005ad -- crates packages` (19 files, +418 -231).
Round 1: `security-auditor-r1.md` (APPROVE, four Lows S1-S4 = my F1-F4; conditions C1-C4 for later change-sets).
Threat model unchanged from round 1 (five lines there); what round 2 must add to it is in item 2 below.

## Plan
1. Diff stat for manifests, lockfiles, CI, kit, gate files (none expected).
2. S1: Replay::new's `cannot` list, checkReplayable, the two refusal tests, Header's hand-written Debug.
3. S2: the comment on World::shared_hash.
4. S3: files::plain, its use in write and check, the folder asked by its own path, names quoted, the unix-only test.
   Experiment in a copy (.gate/sa-4a-r2 at ec005ad, own target dir): plant the round 1 link again, run write and check.
5. S4: two_processes.rs child role only with the parent's own arguments; re-run the preset-variable experiment.
6. The &self shape: Fnv1a64 (state_hash.rs), the private Bytes trait (world.rs), room/local_match/Sim; what the
   players' file now holds (exports, strings, size, digest); any new path for seed bytes.
7. What the pass opened: `replay fields` (two files from the command line), the children's per-step output.
8. Which Lows (if any) are still to be named to Mustafa as left open.

## Findings (a line each, as found)
- Diff scope: 19 source files in crates/ and packages/; no manifest, lockfile, CI, kit, gate or build-script file; the
  committed recordings are byte-identical (the buffer-free hash gives the same values).
- S2 FIXED: world.rs:214-215 "the number a replay compares tick by tick across targets. It is never exchanged with a
  client: no state hash is (ADR 0001 part 6)".
- Shape (item 2): shared_hash(&self) at World/Room/LocalMatch/Sim; the hash fed through the private `Bytes` trait
  (world.rs: `trait Bytes`, not pub; impls Vec<u8> and Fnv1a64) and `Canonical<B>` (private); `bytes`/`secret` take
  `&[u8; N]`. No interior mutability in sim/ or room/ (grep Cell/RefCell/UnsafeCell/static mut/Atomic: none), so the
  export is read-only by its type. The room's buffer of shared bytes is gone, and so is devtools' buffer of whole-state
  bytes (replay.rs:126 `state_hash()`): seed bytes now pass only through the hash's u64. `write_state` (whole bytes,
  seed included, into a caller's Vec) has no caller outside tests any more (note, not a finding).
- Players' file (here): 54,514 B, sha256 19847796...f708, equal in pkg and both built pages; raw exports unchanged
  (one hash export, sim_shared_hash); no state_hash/write_state/Fnv1a64/visit/TESTSEED/SHMBLREC strings; name section
  holds Room::shared_hash and World::shared_hash only. Built pages: no reader text; Worker chunk worker-DooL-xcp.js
  holds the glue wrapper only, no globals. Threat model: no change, one place fewer where state bytes lay.
- S1 FIXED (read): replay.rs:37-40 the first `cannot` entry refuses a seed without TESTSEED, test at :295-299;
  recording.ts:134 the same first in checkReplayable, test in recording.test.ts (replayOn refuses seed [7;32] and a move
  with no call made); recording.rs:59-69 Header's Debug written by hand, seed left out (finish_non_exhaustive), test at
  the `shown` assertion. Recording/Step derive Debug through it; no other devtools type holds a seed.
- S3 FIXED (run, copy .gate/sa-4a-r2 at ec005ad, own target .gate/sa-target, offline): round 1's link (vector.txt ->
  a scratch file outside the repo): write mode exit 1, "...first-loop.vector.txt" is a link or not a plain file", the
  outside file untouched (write had rewritten .rec and .hashes in place first, same bytes); --check exit 1, same text.
  The folder as a link: both modes exit 1, "...recordings" is a link or not a plain folder", nothing written. A link to
  /dev/zero in first-loop.rec's place: --check (ulimit -v, timeout) exit 1 at once, never read. Unlisted names quoted
  ({name:?}) and non-UTF-8 names now reported (to_string_lossy) instead of skipped. files.rs:65-83, :92-94, :108-115.
- S3 read side, still open as recorded (fold file: follow-up for the gate step, risk tier 2): the other readers do not
  go through plain(). Run: with first-loop.rec -> /dev/zero, tests/recordings.rs:18 `read` panicked "first-loop.rec:
  out of memory" under a 1.5 GB cap (uncapped: reads until memory runs out). Same for two_processes.rs (path("rec")),
  two_instances.rs:42-44, replay.test.mjs:15, determinism.spec.ts:68,70 (read, not run). Exposure: a test run on a
  hostile branch; CI cannot go green with such a link (check refuses it).
- S4 FIXED (run, same copy): SHAMBLE_TWO_PROCESSES_CHILD=1 without the parent's arguments now runs as the parent:
  two children started (fingerprints f588d16e9539c4de and fd15704e71b89730), compared, passed (two_processes.rs:67).
- Item 3, children's output: every step's tick/shared/whole of the scripted test seeds (first-loop, long-loop:
  test_seed(name), and Replay::new now refuses any other) goes to the parent through pipes (.output()); the parent
  prints only the two fingerprints and, on a failure, one Difference or the child's stderr. No new exposure.
- R2-1 Low (new, run): bin/replay.rs:34-39 `fields` echoes the differing lines of two files verbatim, through
  field_diff (compare.rs) and eprintln (:61). A dump with ESC ] 0 ; ... BEL (printf to a scratch file) came out with
  033 and \a raw (od -c): a dump from another machine (the tool's very use: two targets' prints) can drive the
  terminal. Fix: escape each printed line (`line.escape_debug()` in the `fields` arm), 1 line.
- Copy removed with `git worktree remove` (it was clean); `.gate/sa-target` still left (round 1's refusal).
