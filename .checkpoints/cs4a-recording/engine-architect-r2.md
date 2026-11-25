# engine-architect, cs4a "The recording", review round 2 (head f9f9971, source as at 07903d9)

Plan (read-only): (1) read round1-fold.md's last section (the fixes); (2) read `git diff eb14739..f9f9971 -- crates
packages`; (3) check E1-E5 each as fixed or not; (4) the export's surface after the pass (one export, no argument, one
number, &self) by reading and by the built file's imports/exports if a fresh build exists (no build of my own unless
needed; cargo --locked only); (5) boundaries after the pass: Fnv1a64 public in sim, the replay tool's `fields`
subcommand, files::plain, anything else new; (6) weigh rust-reviewer's new Low (write_state + STATE_BYTES to
#[cfg(test)]): does 4c or cs7 need them public; (7) verdict, short.

Findings (a line each as found):
- Read: fold "What the fix pass did"; diff eb14739..f9f9971 -- crates packages (19 files, +418/-231); no source change
  07903d9..f9f9971 nor 40bf269..07903d9. Note: HEAD is now 21dff0c, a merge of main made after f9f9971; it changes only
  packages/e2e/playwright.config.ts in crates/packages (main's firefox project, testDir ./tests/rules; the chromium
  project and so determinism.spec.ts unchanged).
- E1 FIXED: state_hash.rs:15-34 `Fnv1a64` plain struct (doc says why not a Hasher); world.rs private `Bytes` + `Canonical<'_, B>`;
  World::state_hash(&self)/shared_hash(&self) streamed; Room/LocalMatch/Sim::shared_hash(&self); room buffer and its
  SHARED_STATE_BYTES import gone; world.rs test pins (shared_hash, state_hash) == fnv1a_64 over write_shared_state /
  write_state bytes; state_hash_alloc.rs now 0 on the first call too; fnv pieces test. Measured: pkg 54,514 B sha256
  198477964866... (built 21:47 after the last source commit 21:42:28; = the fold's and the built pages').
- E2 FIXED: world.rs:52-55 "Every canonical writing ... The snapshot and the block are no such writing". Also the old
  "the one a client's state can be compared with" became "never exchanged with a client: no state hash is (ADR 0001
  part 6)": checked, ADR 0001 part 6 (line 276) says at :323 "no state hash is exchanged with clients". Correct.
- E3 FIXED: bytes<const N>/secret<const N> with &[u8; N]; doc "a run whose length can vary ... is a count, then rows of
  a fixed width". Lines prints `({N} bytes, never shown)`.
- E4 FIXED: recording.test.ts `import type { Sim }` + `export const players: (sim: Sim) => Replayable = (sim) => sim;`
  (type import erased; CI's worker_tests tsc line includes pkg/sim_wasm.d.ts); test-engineer showed the compile failure.
- E5 FIXED: sim-wasm.test.mjs:154-155 adds ["shared_hash()", "sim_shared_hash"] (no argument, one export); lib.rs:4-5
  header names the replay-only number.
- R1 (visit names every field) FIXED: world.rs `let Self { tick: _, player: _, seed } = self;` in visit.
- Surface after the pass (measured, my scratchpad script, base rr-4a-base vs pkg): imports 3 -> 3; exports 18 -> 19,
  only sim_shared_hash; JS glue = base + the one method (only its doc words changed); production Worker chunk: one
  definition, no caller; page chunk 0; no SHMBLREC/TESTSEED/shamble-hashes/readRecording in either build.
- Boundaries after the pass: `fields` (bin/replay.rs) reads two text files into compare::field_diff: tools only, fine.
  `files::plain` is private to devtools (fn plain), guards write/check: fine. recording.rs lint guard, Header Debug
  without the seed, replay seed check: devtools-internal. Nothing crosses a boundary wrongly.
- New nit (Low, optional, 1 line): state_hash.rs:15 `pub struct Fnv1a64` has no user outside sim (grep: world.rs and its
  own test) -> pub(crate) until one needs it; an outside crate cannot make it a Hasher (orphan rule) but a newtype could.
- Follow-up (4c, 0 lines now): SHARED_STATE_BYTES (re-exported sim/lib.rs:12) lost its last outside user (room); 4c
  turns it into a per-match length: drop the re-export then, do not keep a constant that no longer is the length.
- Follow-up (4c or cs8, ~10 lines): with write_state test-only, `visit`'s `secret` is the one public route for seed
  bytes out of sim; a `Secret` wrapper that outside sinks can only count (sim's Canonical alone reads the bytes) makes
  ADR 0004 part 8 a type rule. When the streams join the authority part (cs8) at the latest.
- Follow-up question (after 4a, his scope): main's firefox project runs tests/rules only (his FIREFOX-SOURCE (c));
  tests/replay needs no WebGL, so one line would run the recording and the BigInt path in a second engine. His word
  first (it widens his decision's scope).
- Point 3 (rust-reviewer's Low): AGREE, write_state + STATE_BYTES to #[cfg(test)]. Callers: world.rs tests only (grep);
  devtools uses write_shared_state (keep pub); the hashes stream; 4c's dumps and finite check go through the visitor;
  cs7's prediction copies one soldier (ADR 0004 part 3), the snapshot has its own layout, and no state hash or seed byte
  goes to a client (ADR 0001 part 6; ADR 0004 part 8). A later save/restore would bring its reader and make the writer
  public in that same change, with the seed's handling decided there. Integration tests do not see cfg(test) items:
  none uses it (grep crates/sim/tests).
- Verdict: APPROVE. E1-E5 fixed. One optional 1-line nit; the rest follow-ups. Not verified: no cargo, tsc, build or
  browser run of mine; CI green on ec005ad per the coordinator; test-engineer's E4 compile failure not re-run; the merge
  21dff0c (after the reviewed head) read for crates/packages only.
DONE
