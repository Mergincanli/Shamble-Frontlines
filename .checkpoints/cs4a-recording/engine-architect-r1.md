# engine-architect, cs4a "The recording", review round 1

Plan (read-only review; source as at eb14739; diff `git diff c1da1bd...HEAD -- crates packages`):
1. Read: plan.md §4, netcode-architect-cs4.md block 1, my engine-architect-cs4.md (Walk design), ADR 0002 part 3,
   build plan Part 15.2, the players' file header (WB-HEALTH words), DEPART-3 wording.
2. Q1: Sim::shared_hash chain (sim-wasm -> LocalMatch -> Room -> World): read-only? live-tick reachable? buffer out?
   u64/BigInt cost, glue growth 53,813 -> 55,607 bytes, &mut self shape, the boundary pin test.
3. Q2: StateSink trait + World::visit/visit_shared; room reads SHARED_STATE_BYTES; devtools new modules, bins,
   wasm.rs test-only exports; recording.ts test-only reader; e2e replay tests + node-names.d.ts. Boundary/direction,
   test-only reach into production, StateSink shape for 4c (floats, variable-length runs).
4. Q3: lane/file ownership vs Part 15.2.
5. Q4: cheap-now shaping for 4b/4c; what must not change.
6. Each finding: severity, file:line, fix in one line, ~lines, follow-up target.

Findings (a line each as found):
- Read: plan s4; netcode block 1; my N14/N15 (my own cs3b advice was the room buffer + &mut self); cs3-plan @80a51e42
  :152-159, :315-321 (DEPART-3's words: "one new read-only number on the file players get, the state's hash ... no
  call that takes a block of bytes ... one type and plain numbers ... read only when a recording is replayed, never on
  a live tick"; "two 32-bit numbers, or one large integer read only during a replay"); WB-HEALTH (DECISIONS :1392).
- Measured (my script in the scratchpad, WebAssembly.Module.imports/exports): players' Wasm base 53,813 (rr-4a-base)
  vs head 55,607 (pkg, sha256 e419e4cc... = built page's): imports 3 -> 3 unchanged; exports 18 -> 19, added only
  `sim_shared_hash`. JS glue 9,824 -> 10,341 B: the diff is exactly the one 11-line method (JSDoc ~370 B; minified in
  the production Worker chunk: `shared_hash(){let e=m.sim_shared_hash(this.__wbg_ptr);return BigInt.asUintN(64,e)}`).
- Measured: .gate/dist-production (built 20:19:21 from the reviewed source; Wasm sha256 = pkg's): `shared_hash` once
  in the Worker chunk (the glue's definition, no caller), 0 in the page chunk; no SHMBLREC/TESTSEED/shamble-hashes/
  readRecording in production or development builds -> recording.ts is in no build. wasmProblems (build-checks.mjs)
  keeps the test-only module out of every built page.
- Q1 verdict: within DEPART-3 and WB-HEALTH: one method on the one type, one number out, no argument, no buffer, no
  import added, module exports pinned unchanged; live tick: HostSim (host.ts:9-15) lacks it, host.test.ts:113 pins
  {input 90, step 90, rebase 1, block_ptr 1, block_len 1}. u64 as BigInt: DEPART-3 allows "one large integer read only
  during a replay"; cost = BigInt.asUintN per call, replay only. &mut self: the World is provably untouched
  (World::shared_hash(&self)); only Room's private scratch is written -> read-only in substance, not in the signature.
- EA1 (Low, decision asked by the fold's R8): take rust-reviewer's streamed hash NOW: export becomes &self (the
  signature carries DEPART-3's "read-only"), room holds no buffer and stops reading SHARED_STATE_BYTES (4c makes the
  shared length per match: my Walk design 2.2/2.4 slot count; netcode N10), -1,093 B of the +1,794, no allocation by
  construction. Shape: one Canonical encoder generic over its byte destination (Vec or an Fnv1a64 state); the FNV
  state a plain struct, NOT core::hash::Hasher (Hash impls feed write_usize/length prefixes: 4 vs 8 bytes, wasm32 vs
  native); a test that the streamed hash == fnv1a_64(write_shared_state bytes). ~40-60 lines. Else first step of 4c.
- Sourced (doc.rust-lang.org, std::hash::Hash, "Portability"): "Due to differences in endianness and type sizes, data
  fed by Hash to a Hasher should not be considered portable across platforms"; Hasher::write_length_prefix: "The impl<T>
  Hash for [T] includes a call to this method" -> EA1's FNV state must not be a Hasher. Hash's "Prefix collisions"
  rule (('ab','c') vs ('a','bc')) is the principle behind EA3.
- EA2 (Low, world.rs:52-54, 1-2 doc lines, now): StateSink's doc says "Every writing of the state goes through it". 4c
  writes the soldier into the snapshot (protocol) and the block (client-endpoint layout): read literally, the doc
  invites routing them through the visitor, coupling the wire's order/quantisation to the hash's canonical order and
  putting `secret` (the seed) one mistake away from a client. Say: every canonical writing (the hashes' bytes, the
  named dumps); the snapshot and the block keep their own layouts and never go through it.
- EA3 (Low, world.rs:58-59 + 80, 1-2 doc lines or ~5 lines by type): `bytes(&[u8])` writes no length; prefix-free
  today only because its one caller's run is fixed (presses, 16). For 4c's table: the rule "a run whose length can
  vary is a count field (u16) then fixed-width rows, never `bytes`" in the trait doc, or make it the type:
  `fn bytes<const N: usize>(&mut self, name, value: &[u8; N])` (StateSink is only used as `impl`, never `dyn`).
- EA4 (Low, recording.ts:40-45, ~3 lines, now or before 4c widens `input` to 13 numbers): `Replayable` is written by
  hand and nothing ties it to the generated `Sim` type (replay.test.mjs is untyped JS; HostSim is tied through
  worker.ts:26-28, Replayable is not). In 4c a 6-argument replayStep against a 13-argument Sim.input type-checks and
  runs: missing u32s arrive as 0 and missing f32s as NaN, so first-loop's zero rays may even replay green. Fix: build
  it as `Pick<HostSim, "rebase" | "input" | "step"> & { shared_hash(): bigint }` and add in recording.test.ts (type-
  checked with pkg/sim_wasm.d.ts in CI's worker_tests line) `const _pin = (sim: Sim): Replayable => sim;` (type-only
  import, erased under verbatimModuleSyntax).
- EA5 (Low, optional, sim-wasm.test.mjs:151-158, ~2 lines): the export pin catches a new name, not a changed shape of
  shared_hash (an argument, a buffer). Add ["shared_hash()", "sim_shared_hash"] to "each per-tick call reaches exactly
  one export" (rename it "each call"): pins no argument and exactly one Wasm call, at the boundary test.
- Agree: rust-reviewer R3 (fold R1, Medium): `visit` must destructure Self; it is the guard on who owns which field
  (shared vs authority), which 4c's soldier table and cs8's streams rely on. netcode N5 (marker in 4c), N10 (subsumed
  by EA1 if taken).
- EA4 corrected: drop the Pick<HostSim> idea (recording.ts is also type-checked in the e2e spec program with the dom
  lib; pulling host.ts in there is a risk for nothing). The pin alone suffices: recording.test.ts (CI's worker_tests
  line includes pkg/sim_wasm.d.ts) gets `import type { Sim } from "../pkg/sim_wasm.js"` and
  `const pinned: (sim: Sim) => Replayable = (sim) => sim;` -> in 4c a 13-number Sim.input no longer fits a 6-number
  Replayable.input ("too few arguments"), so Replayable and replayStep must widen. ~3 lines.
- EA5 extended (nit): sim-wasm/src/lib.rs:1-8, the header lists what crosses (two calls per tick) but not the one
  replay-only number; 1 line makes the header the whole list.
- Q2 checked: no crate depends on devtools (grep of every Cargo.toml); devtools' new code uses only sim's public API and
  sim_wasm::{Sim, MODE_LOCAL} (+ accessors Sim::local -> LocalMatch::room -> Room::world, all pre-existing); no new
  edge used, no manifest or lockfile change (diff stat); the test-only module is built only by non-production builds
  into .gate/rules-module (build.mjs:11-16, 119-133) and wasmProblems refuses any other Wasm in dist; recording.ts is
  imported by recording.test.ts, boundary/replay.test.mjs and e2e determinism.spec.ts only (grep); recordings sit in
  crates/devtools/recordings like crates/devtools/vectors (read by rules.test.mjs, wasm-rules.spec.ts: precedent).
  determinism.spec.ts:9 is e2e's first relative import of another package's src: test -> shipped direction, fine; an
  exports entry would make a test reader public and needs a manifest change. No finding.
- Q3: every touched file is lane 1's by Part 15.2 (crates/**; packages/sim-worker; packages/e2e "the package, its
  config, the simulation's tests": tests/replay is a simulation test, not look/assets/feel; node-names.d.ts was made by
  lane 1's CI-WASM pull request 8e7f503 and is the package's own source, not the 2.3 chat's settings). Outside crates/
  and packages/ only .checkpoints changed. No kit, gate, CI, manifest or lockfile file.
- Q4 for 4b (0 lines now): content through the constructor only (plan s5 step 7), no second export: the replay checks
  the frozen bundle's SHA-256 against the header itself (devtools natively, node:crypto in Node); if the players' file
  must compute the content's SHA-256 (the Join's content_hash, protocol messages.rs:37-38, room lib.rs:108;
  local_match.rs:20 passes NO_CONTENT today), sha256.rs moves whole with its tests to the content crate and devtools
  uses it from there (one implementation, R8's 312 lengths kept), never a second copy; first-loop's header re-made with
  its frozen content set while every hash line stays byte-identical (the proof that loading content changed no
  state; answers netcode N9 without a "no content" path on the players' constructor, which would be a test hook on
  the shipped file).
- Q4 for 4c (0 lines now): StateSink gains u8, f32 (u32::to_le_bytes(value.to_bits())) and row(table, index) (Canonical
  writes nothing for it: the u16 slot count before the rows keeps the bytes prefix-free; Lines prefixes names with
  table[index]), all additive, existing signatures unchanged; input to 13 numbers in Replay::next, replayStep and the
  e2e WORKER string, the ray crossing as f32 values made from the recorded bits (getFloat32 / f32::from_bits) and the
  writer refusing non-finite ray bits (netcode N8); the paced replay (6b) reads the hash through a dev-only replay
  driver beside SimHost, never through HostSim (host.test.ts:113 stays two calls a live tick), recording.ts behind a
  DEV_MARKERS marker then (N5); button meanings (Space 4 jump, Shift 5 sprint) get one home in Rust when the sim first
  reads them, generated to TS for keyboard.ts:10-20, and scripts.rs:10-16 reads them from there.
- Must not change: the pinned surface (module exports; Sim.prototype names; 4b's content via the constructor, 4c's input
  widened in place); shared_hash = the local server's SHARED hash only, never the whole one, no argument, no bytes;
  HostSim without it; today's canonical bytes and order until 4c re-makes first-loop on purpose; shared part before
  authority(), the seed only through secret, never printed; format v1 and first-loop's input records; devtools a leaf.
- Line fix: EA5's test is sim-wasm.test.mjs:152-160 ("in the glue, each per-tick call reaches exactly one export").
  Also seen: sim-wasm.test.mjs:162-200 hands the real-Wasm host only the five HostSim methods, so a live shared_hash
  call would throw there too (second pin of "never on a live tick").
- Verdict: APPROVE. Nothing Critical, High or Medium in my lane. Answer to the fold's R8: yes, the streamed hash and
  &self now (EA1, ~40-60 lines; else 4c step 1). Lows EA2 (1-2), EA3 (1-5), EA4 (~3) cheap now, at the latest before
  4c reaches them; EA5 (~3) optional. Not verified: no cargo, tsc, build or browser run by me; the streamed variant is
  rust-reviewer's measurement; Firefox/WebKit BigInt path unrun; EA4's compile error reasoned, not run.
DONE
