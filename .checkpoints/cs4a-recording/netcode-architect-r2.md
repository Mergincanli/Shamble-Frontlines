# netcode-architect, review round 2 of cs4a "The recording" (head f9f9971, source at 07903d9)

## Plan
1. Read round1-fold.md (last section: the fixes), plants-4a.txt (15 caught, 4 controls), `git diff eb14739..f9f9971 -- crates packages`.
2. Check N1, N2, N3, N6, N7, N8, N9 (nits) each fixed / not fixed; N4, N5 follow-ups.
3. The new shape: hash fed straight from the visitor, `shared_hash(&self)` at room, local match, Sim; no buffer on World::state_hash / shared_hash. Against my 4c design (varying shared length, soldier table in the visitor, prediction calling the hash in cs7).
4. Anything the pass broke in my lane; cargo test -p devtools --locked and the bins (read-only).
5. Verdict, findings with lines; what I did not verify.

## Findings (one line each, as found)
- Source identical at 07903d9, ec005ad and f9f9971 (git diff --stat empty). Recordings' data files unchanged by the pass. cargo test -p devtools -p sim -p room -p sim-wasm --locked: all green (6 s). replay check / fields / make_recordings --check as expected; two_processes prints both fingerprints (94be9caa2b471a29 and 27563eec89dd4bec here).
- CI run 45707588626 (ec005ad): a_rebase_before_every_step_changes_no_hash and two_more_processes ran in both workspace runs; boundary replay.test.mjs 6 tests; determinism.spec.ts passed on HeadlessChrome 157; pkg sha256 198477964866... = the fold's.
- N1 FIXED: plants-4a.txt (commit 07903d9): C and D: two_processes names step 600 ("the two processes differ", "step: 600"), `replay check` names step 600, `replay fields` on two processes' state prints names player.presses (script lines 82-87); E: Node at 600, field by native vs test-module state (lines 130-137). two_processes.rs:95-122 compares child vs child per step in both columns and file vs child (Whole, then whole equality).
- N2 FIXED: replay.rs:37-40 seed first in `check`, test :295-299; recording.ts:134 seed first in checkReplayable, test through replayOn with no call made (recording.test.ts:150-156).
- N3 FIXED: replay.rs:303-317 `a_rebase_before_every_step_changes_no_hash` (both columns and ticks).
- N6 FIXED: two_processes.rs:24 prefix "two-processes ", steps parsed per name (:44-60), so walk / lane-walk lines are kept.
- N7 FIXED: files.rs doc on `write` (re-made in the commit that changes the sim on purpose, after --check named the first step that moved; report says why; no-change cs leaves files byte-identical) = design 1.4.
- N8 FIXED: recording.rs doc on Step::ray_origin (turned back into f32 before anything reads it; the format holds no wider number).
- N9 FIXED: determinism.spec.ts:70-74 checks recordingSha256 against SHA-256 of the .rec; two_processes.rs:90 prints the two fingerprints.
- N4 (cs 8) and N5 (4c): follow-ups as agreed. Note: the new two-process file comparison already uses the whole column first.
- Q2 (buffer-free hash, &self): nothing in my 4c design wants it otherwise. Varying shared length: the only per-room buffer sized by a constant is gone; write_shared_state/write_state and SHARED_STATE_BYTES remain for the vector and tests only (4c turns them into the world's lengths). Soldier table: the hash takes whatever the visitor hands over; bytes<const N> forces count + fixed rows. cs7: state_hash(&self) with no allocation suits per-tick hashes of restored/predicted worlds (PM-0004-13); PM-0001-17's quantised-state hash is another StateSink over the public Fnv1a64. The export is read-only by signature now (DEPART-3's "read-only").
- 4c note (no line now): hand over every slot of the fixed-capacity soldier table with its occupied flag and generation, so dump line i is slot i (field_diff compares line by line and would misalign on a missing row) and the canonical length is fixed per match; per-row names need a row index (engine-architect's row()).
- Nothing broken in my lane by the pass. No new finding.
- Verdict: APPROVE.
DONE
