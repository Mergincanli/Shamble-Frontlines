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
