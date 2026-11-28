# U4d2 review round 1: engine-architect seat (the boundary)

Read only. Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + U4d2's snapshot patch). Base copy:
/home/user/Shamble-Frontlines/.gate/u4d2-prefix (51520b4). No cargo or node runs by this seat; runs I would want are
listed for the main session at the end. This seat has no Edit tool: the notes file is rewritten whole with Write each
time, its content kept. No shell write by this seat (reads only: cat, sed, grep, git diff, git show in the copies and
the main folder).

## Plan
1. Read the packet's heads and the judged-against texts (u4d2-packet.md): §5 U4d2, §4.1, §4.11, §4.14, §4.15, Lanes,
   §7.1; plan.md §4.1 hash-wasm line, §4.2, §4.5, §4.7's npt-lab line; advisor rulings named; carries routed to U4d2.
2. Edges: crates/hash-wasm/Cargo.toml, crates/lab/Cargo.toml, the workspace Cargo.toml; check.mjs's deps step (LEAF,
   WASM_CRATES, rules), against the plan's edges.
3. The Rust-to-JS boundary: hash-wasm's lib.rs (seven exports, status constants), loader.mjs (export list, one home),
   check.mjs's wasm step's use of it; reuse by U7's Chromium leg and wasm-opt (C-U7-1).
4. HashCompare and TickHash (lab): shape, Empty, reuse by U4r's -30 pre-check (condition 2).
5. Public items: every pub item added (Rust, check.mjs exports, loader exports) from the diff, each with its reason.
6. Seam (u4-build-plan.md §4.2) and lane (Lanes): the file list of the change vs the lane; anything built that no line
   asks for.
7. Verdict and the run list for the main session.

## Notes (one line per finding, as found)
- [resume 2026-11-12, after a usage cut] Plan step 1 done: packet's judged-against texts read (u4d2-packet.md:4255-6130).
- Lane: 9 tracked files changed (Cargo.lock, Cargo.toml, check.mjs, check.test.mjs, client-wasm/Cargo.toml,
  lab/src/{lib,main,probe}.rs, proto/Cargo.toml) and 8 new (hash-wasm/{Cargo.toml, co-initial-dump.txt, loader.mjs,
  src/lib.rs}, lab/{clippy.toml, recordings.json, src/hash.rs, std-sin-scan.txt}). Every path is in the Lanes line's
  U4d2 list (as U4D2-Q9 rewrote it). proto and client-wasm manifests: comment lines only (diff read). Cargo.toml:
  members + the lints and libm comments only. Cargo.lock: the one npt-hash-wasm entry. lab/Cargo.toml unchanged.
- Seam: no crates/sim file changed, so no seam change (u4-build-plan.md §4.2). hash-wasm and lab use only npt-sim items
  already public (Replayer::{new, with_bite, feed, step, world, state_hash, dump}, Step, World::BITES,
  World::bite_order_digest, StateDump, FieldDiff, Recording, Scene accessors): no new npt-sim item.
- Edges: hash-wasm/Cargo.toml: npt-sim only, no dev dependency, `bites` = ["npt-sim/bites"]; check.mjs:130-144 enforce
  "ships npt-sim only" (shipped kinds) and "nothing depends on it" (every kind, dev included), as tcpinfo's rule; LEAF
  (check.mjs:39) and WASM_CRATES (:1807) hold it, as §5 U4d2 asks. npt-lab's edges unchanged; nothing depends on it.
- HashCompare (lab/src/lib.rs:9-61): pure function of two lists of (String, u32, u64); no npt-sim type in its
  signature, reads no file. Fits U4r's -30 pre-check unchanged: npt-room at c88eb31 gives `last_tick() -> u32`
  (room.rs:573) and u64 hashes (SimCore::state_hash; ClientSlot::server_hash -> Option<u64>, room.rs:206).
- EA-1 Low, boundary cost: hash_lo and hash_hi (hash-wasm/src/lib.rs:143-174) each call Replayer::state_hash, which walks
  the whole state (sim/src/world.rs:2141-2145, write_state); the loader calls both every tick (loader.mjs:132, :151), so
  the Wasm leg walks the state twice per tick where the native leg walks it once (lab/src/hash.rs:914). Within budget
  today (the step 46.5 s whole, e2-check.log:156). Fix, no export change: cache the u64 in State when step() steps,
  clear it at begin. Not before merge.
- EA-2 Low, the ABI's facts with two homes: the status numbers live in lib.rs:50-56 (private consts, Rust tests use the
  names) and loader.mjs:38-40 (check.test.mjs:2001 pins the JS numbers): a renumbering on one side passes both unit
  suites and is caught only by the wasm step's end-to-end run (fail-closed: loader.mjs:158). And step() drops
  Stepped(tick) (lib.rs:137), so the loader numbers ticks from `--first-tick` (loader.mjs:130), which check.mjs reads
  from the manifest (check.mjs:2197, :2203): a second path for the first tick beside the header Replayer reads
  (U4D2-Q3); a wrong first tick would show as 9,000 mismatching ticks, not as a numbering error. lib.rs:9's "1..=4"
  hard-codes World::BITES's length (the code reads BITES.len()). Fix: one check.test.mjs case reads lib.rs's six
  status consts (the deps scan already reads that file) and compares them with BEGIN and STEP; the doc says "1 to
  World::BITES.len()". Not before merge.
- EA-3 Low, the lab-to-check.mjs contract is prose: the wasm step reads npt-lab's verdicts from its English lines
  (readComparison, check.mjs:2075-2104, `equal` by regex at :2097; regexes at :2204, :2224, :2226, :2227, :2237, :2270)
  and never reads the exit code of `npt-lab hash` (:2223) or `hash compare` (:2242). Every path fails closed today (no
  comparison line means "no comparison was written"; per-recording counts and the total are checked), so no wrong pass;
  but the line forms have three homes (hash.rs hash_line :426, dump_text :499, compare_text :590; loader.mjs :137, :151;
  check.mjs's readers), bound only by the end-to-end run and the fixtures. Hardening: the clean half also requires exit
  0 from both calls (`legs.ok`, `cr.ok`), one fixture case each. Not before merge.
- EA-4 Low, U7 reuse: the .wasm is reusable as is by U7's Chromium check (u4-build-plan.md:898-899, "it loads U4d2's
  file") and through wasm-opt (C-U7-1): no imports (the loader's empty import object), integer-only exports, memory the
  only other export. The loader is not: node:fs and node:url at its top (loader.mjs:23-24) stop a page from importing
  it, `replay` reads by path (:144, forEachChunk :91), and it compiles synchronously (:80, :219) where §5 U4d2 says
  "loads it with `WebAssembly.instantiate`" (equivalent in Node). U7 then splits it (export list, statuses,
  moduleProblems and the replay loop over a chunk source in a browser-safe module; the CLI beside it), keeping
  HASH_WASM_EXPORTS's one home with check.mjs's import following it. Building that now would be a for-later feature:
  record it as a U7 carry instead. Not before merge.
- EA-5 Low, LEAF's rule as worded: hash-wasm joins LEAF (check.mjs:39, as §5 U4d2 asks), whose doc cites plan.md §4.2;
  plan.md §4.2's leaf line (plan.md:422-424) neither names it nor holds for it ("Clock reads are banned in them by
  clippy"): every other LEAF crate has a clippy.toml (link, l1, sim, proto, room, client, log), crates/hash-wasm has
  none. Reach: none today (no clock or map in the crate; the loader's empty import object refuses a host clock). Fix,
  the advisor's pick: a crates/hash-wasm/clippy.toml with lab's bans, or plan.md §4.2's line naming hash-wasm as a leaf
  for the transport and wasm32 rules only (the main session's plan.md edit). Not before merge.
- EA-6 Low, TickHash's contract for -30: lab/src/lib.rs:11 says the name is "the recording's ... (or a run's, for
  U4r's legs)", but HashCompare compares the name with the tick and hash (lib.rs:47), so legs named by their own run ids
  mismatch at entry 0. Positional, not tick-keyed: legs out of step by one tick mismatch everywhere. And the printed
  comparison (compare_text) is binary-private (hash.rs:590), so a pre-check outside the binary writes its own form of
  "the comparer's output" (condition 2's record). Fix now: the doc names "the recording both legs replay (the same name
  on both legs)". For U4r's brief: align the two lists, and say where the printed line comes from. Not before merge.
- EA-7 Low, public items to record (W1-PUB form): npt-lab's `TickHash` (a type in `HashCompare::of`'s signature),
  `HashCompare::of` and the variant `HashCompare::Empty` (the listed type's own shape; §4.6's "0 compared fails", as
  npt-judge's `eval::Outcome::Empty` in §4.1) are each justified, but §4.1's npt-lab row lists only the subcommands: the
  report and the main session's §4.1 update list them. hash-wasm: seven `pub extern "C" fn` only, every const private.
  loader.mjs exports three items no file imports: forEachChunk (:91), parse (:175), main (:209): drop `export` or test
  them. check.mjs's 23 new exports: each imported by check.test.mjs (counted). Not before merge.
- EA-8 Low, the manifest's form for U4r: recordings.json's entries carry the synthetic generator's inputs only, and
  hash.rs's `manifest` (:212-259) requires them while `regenerate_all` (:291-318) synthesizes every entry; §4.15 names
  "(scene, seed, script, generator version)" and puts U4r's runner recordings under "a committed recordings manifest".
  U4r will change this schema (a kind or script key and a dispatch); its brief should say so. No change now.
- EA-9 Low, one subcommand, two defaults: `hash compare`'s replay_to arms a bite on every recording when --on is
  omitted (hash.rs:959), while the legs arm it on the first replayed (bitten_name, :421). check.mjs always passes --on
  (:2277, :2340), so it is not reached. Fix: one default. Not before merge.
- Carries in my reach, met as worded: 29 (Cargo.toml's libm comment, carry's words); 41 (proto's bites comment); 52
  (generator 2 only, refused otherwise: hash.rs:296-303; the manifest's about names adb1bab); 65 (failingTests,
  check.mjs failedRun's note above the summary; self-test check.test.mjs:2256-2282); 66 (probe.rs through
  for_each_canonical, no crates/l1 edit; e3-h7.log:25-30, exit 0, the three values equal U2's). Carry 42 does not apply
  (the advisor's brief check, block 2).
- Nothing built that no plan line or ruling asks for: `hash vectors` (T22, all four scenes, stricter than one),
  `hash bites` (the index ABI), --digest-field (D-2's digests through the dump, since the exports are fixed), the
  libc line (TB-2), the all-pairs output on a FAIL (A2).

## Verdict
APPROVE WITH FINDINGS: nine Lows (EA-1 to EA-9), none before the merge, no Critical, High or Medium. Edges, lane, seam
and the export boundary hold as the plan and rulings word them.

## Runs I would want (for the main session; this seat ran none)
1. check.mjs counted on the merged head (c88eb31 with wip/u4d2.patch): the features comparison now walks U4e's
   npt-server library, and the wasm step's release npt-lab builds U4e's crates (the merge step plans it).
2. EA-2's proof: a scratch plant `const STEPPED: u32 = 4;` in crates/hash-wasm/src/lib.rs, one `-x -k` call:
   `cargo test -p npt-hash-wasm` passes, `node --test check.test.mjs` passes, `node check.mjs` FAILs its wasm step;
   restored by Edit, sha256 checked.
3. EA-3's fail-closed claim: a scratch plant in compare_text ("equal" written "same"): the wasm step FAILs; restored.
4. EA-1, optional: the Wasm SS-s1 leg timed with the hash cached (scratch) against the copy as built.
