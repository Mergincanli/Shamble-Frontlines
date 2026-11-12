# U4d2 brief: drafter's notes (rust-reviewer, 2026-11-12)

No Edit tool in this agent: this file is rewritten whole with Write at each save (the task's rule). No cargo build,
test, clippy, doc or check.mjs was run. Read-only commands run: git log/diff/merge-base, `rustc --print cfg` (host and
wasm32), `cargo rustc --print cfg` (refused by stable cargo before any build: see R-rows), `cargo tree` (offline,
--locked, resolution only), `nm -D` on glibc, `node --version`.

## Plan
1. Read the model brief u4e-brief.md (structure: sections 0-8) and its approval (advisor-u4e-brief.md).
2. Read u4-build-plan.md §5 U4d2 (:666-702), §5's Lanes (:840 on), §4.1, §4.2, §4.15, and every section U4d2 cites.
3. Read plan.md §4.1 layout, §4.2 named exceptions, the lines U4d2 names.
4. Read the carries; u4a-build.md and u4ab-*.md for entry 34.
5. Read the rulings named in the task.
6. Read the resume card's "Rules in force" and history.md "Decisions taken" (2026-11-11).
7. Read u4e-run.sh (and main-run.sh).
8. Read the code (check.mjs, check.test.mjs, Cargo.toml, crates/, marks/, the -3 run script, docs/adr rows).
9. Draft the brief by section (0..8), saving after each section; log choices and conflicts here.

## Progress
- [x] sources read (all of steps 1-8)
- [ ] brief §0-1
- [ ] brief §2
- [ ] brief §3
- [ ] brief §4-6
- [ ] brief §7-8

## Sources read (file:line, at the main folder's head cea8209; spikes/ equal to 28a85e7: `git diff --stat 28a85e7 HEAD -- spikes/` empty)
- u4e-brief.md (982 lines, rev 3.3): structure §0 :120-135; §1 :139-172; §2 :176-564; §3 :568-717; §4 :721-781;
  §5 :785-801; §6 :805-834; §7 :838-912; §8 :916-982.
- u4-build-plan.md (1,079 lines): §5 U4d2 :666-702; §4.1 :178-203 (hash-wasm :202; npt-lab subcommands :198;
  Recording/Replayer :185); §4.2 :205-249; §4.3 :251-265; §4.11 :392-407; §4.14 hash-map row :476; §4.15 :478-504;
  U4d :602-643 (D-2 :612-614; D-7 :629; bites :640-642); U4r :740-793 (HashCompare :766-767; det-u4.mjs :790-792);
  Lanes :840-875 (step 0 :841-845; wave 2 :863-867; reviewers :872-873); §6 U7 :885-887; §7.1 :896-908; §7.2 :910-918;
  §2 Features :157, C-U7-1 :159; RQ4/RQ5 :117-118; T9 :69, T22 :82, A8 :91, A12 :95, A13 :96; Q3 :981-985;
  Q9 :1002-1005; Q21 :1040-1042.
- plan.md: §4.1 :366-419 (hash-wasm :392-393); §4.2 :421-454 (three exceptions :435-447, hash-wasm :442-446, how read
  :449-450); §4.5 :527-551 (flags :533-535; features :536; the check :546-551, Chromium :548, bites :551); §4.7 npt-lab
  :617 (`HashCompare`), binaries :611-612.
- u4s-carries.md: §U4d2 :407-446 (29 :408-411, 41 :412-414, 42 :415-417, 52 :418-428, 65 :429-435, 66 :436-446);
  entry 34 :62-69; recordings section :351-379 (67 :352-369, 71 :370-374, 72 :375-379).
- Entry 34: met by U4a (u4a-build.md:125-126; history.md:1335; u4ab-review-test.md:90; u4ab-review-rust.md:27). Carry
  42's condition ("If U4a's report does not list entry 34 as met") is false: U4d2 takes no word of entry 34.
- advisor-u4.md: cond 2 :90-101; cond 7 :130-136; cond 8 :137-139; cond 10 :143-149 ((b) :145-146); cond 12 :154-158;
  Q3 ruling :198-199; Q9 :207; U4s cond 2 :313-327 (carry 41's origin :324-326); F2 (3) :450-458 ("in wave 2 after
  U4d2 has merged" :456).
- advisor-u4-wave1.md: W1-PUB :353-435 (`Recording::synthetic` in the library :408-409; Recording one path :412-413);
  W1-U4D's plan text :300-301; W1-TARGET :558-586 (disk :576-580).
- advisor-u4d-close.md: B1.2 :42-134 (closed loop refused :48-51; U4d2 bases on U4d-F :117-119); B1.3 :165-180; B2
  :229-308 (U4e's carries; nothing for U4d2 but carry 52's text :136-150).
- advisor-u4df.md: R1 :28-87 (RM :71-85; "fix the scene before U4d2 pins" :53-56); R3 :182-210; R4.1 :213-231
  (TE2-5 stop clause for next time :228-231); R5 M9 :294-297 (U4d2's brief names U4d-F's commit); R6 :301-322; T4
  :459-470.
- advisor-u4j-r2.md: B2.3 :146-171 (carry 65's half (a) :158-160; the rule :167-171).
- advisor-u4e-brief.md: F2-ROUTE :277-310; L4 :335-336.
- netcode-proof-test.md (the card): Rules in force :137-176 (USAGE-SAVINGS :138-147; TE2-5 rule :161-165; T4 :168-176);
  U4e's state :26-62 (build done, round 1 running); U4d2's brief workflow :63-70.
- history.md: Decisions taken :280-309 (USAGE-SAVINGS builders' limits :291-293).
- u4e-run.sh (90 lines) and main-run.sh.
- Code (spikes/netcode-proof/ at 28a85e7): Cargo.toml (members :6-23; lints comment :73-92, forbid :93; libm comment
  :50-53; release overrides :108-112); rust-toolchain.toml; .cargo/config.toml; crates/proto/Cargo.toml (bites comment
  :13-14); crates/lab/Cargo.toml; crates/lab/src/{lib.rs,main.rs,probe.rs}; crates/sim/src/{replay.rs,world.rs,
  hash.rs,lib.rs}; crates/sim/tests/replay.rs; crates/tcpinfo/{Cargo.toml,src/lib.rs,src/linux.rs};
  crates/client-wasm/Cargo.toml; crates/server/{Cargo.toml,clippy.toml}; crates/l1/src/lib.rs :325-352;
  crates/l1/tests/l1.rs :478-532; check.mjs (whole structure; :1-18, :31, :80-159, :161-216, :301-312, :394-452,
  :489-543, :1013-1093, :1196-1284, :1398-1403, :1411-1421, :1433-1500, :1502-1590, :1594-1654); check.test.mjs
  (:1-98, :1094-1165, :1246-1249, :1617-1698); ops/runs/c-u2-pm-0001-3.mjs; results/c-u2-pm-0001-3/*/runs.txt;
  marks/SERIES.md :385-405; docs/adr/0002 :252-266, :498-506; docs/adr/0004 :1138-1145, :1184.

## Facts read or measured (each with its source)
- F1. `rustc --print cfg` at 1.99.1: host target_feature fxsr, sse, sse2 only (no fma); wasm32-unknown-unknown:
  bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext (no simd128, no
  relaxed-simd), panic="abort", pointer width 32.
- F2. `cargo rustc --print cfg` on cargo 1.99.1: "error: the `print` flag is unstable, and only available on the
  nightly channel of Cargo" (cargo issue 9357). So the read-back cannot ask cargo for the flags it would pass.
- F3. Cargo's rustflags sources (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags): four
  mutually exclusive sources, first used: CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, all matching target.<triple>/<cfg>
  rustflags joined, build.rustflags.
- F4. cargo tree (offline, --locked, -e normal, --format '{p} [{f}]'): npt-sim's subtree is libm [arch,default],
  ppv-lite86 [simd], rand_chacha [], rand_core [], zerocopy [simd], npt-sim [] in npt-server (x86-64), npt-lab
  (x86-64) and npt-sim alone (wasm32). A plain grep of the whole tree also picks getrandom and cfg-if from other
  subtrees: the comparison must walk npt-sim's subtree, not grep names.
- F5. glibc 2.39 here defines `step@GLIBC_2.2.5` and `advance@GLIBC_2.2.5` (compat, non-default versions; nm -D). No
  default-version symbol named begin, feed, step, hash_lo, hash_hi, dump_len or dump_word.
- F6. Node v22.23.4 in the container. No wasm-opt installed (C-U7-1 stands).
- F7. `Replayer` (replay.rs:621-770): new(), with_bite(&str, arm_tick) (bites only, :662-670), feed(u32), step() ->
  Step {Stepped(tick), NeedMore, Done, Failed(&'static str)} (:608-619), state_hash(), dump(), world(). Scene, seed,
  first tick, TICK_HZ and pools come from the recording's header (:8-15, :113-140, :709-736): nothing takes a seed or a
  scene from the caller.
- F8. `World::BITES` (world.rs:934-938) = PM-0004-21/hash-map, PM-0004-21/std-sin, PM-0001-41/server-speed-1pct,
  PM-0001-41/server-quant-differs; the std-sin site replaces libm::sinf in the vehicles' loops only
  (world.rs:994-1000, :1216-1228): theta = TAU * along / period, along = (elapsed + phase) mod period. Its input is
  chosen by the loops, not by the caller: `World::with_bite` takes the arming tick only (:944-966).
- F9. D-2 (sim/tests/replay.rs:21-29, :264-309) and D-7 (:311-390) pin the three recordings: SS seed 1 from tick 0,
  ES seed 2 from 1,000,000, TC seed 3 from u32::MAX - 4,499; 9,000 ticks; span 2,000 ms; bytes 16,694,921 /
  16,700,849 / 799,693 with FNV-1a-64 35974458070191737 / 15077317756489230056 / 9126256229328065459; GENERATOR 2
  (replay.rs:288). Initial-world dumps: SS 6,488 words, ES 1,944, CO 260, TC 340, with their hashes (:320-325).
- F10. `Replayer::step` on a 0-tick recording builds the World and returns Done (replay.rs:709-739), so a header-only
  recording gives the initial state's hash and dump through the same calls.
- F11. U2's full fate-log SHA-256s (v1): P2 913db8e914bdb773f6f19ac404c7098f711798790fcd1a861502502764b11ecc, B1
  feabf2f02d023fa9784b2f9630ec6e2ceeed0bcdd28507c203b0ba40de1c25bf, P4
  b8a04f2f8ddc0d39624d853730636c50ffb354ddb187938066fabc156c1f602b (raw logs under
  .gate/netcode-proof-test/runs/c-u2-pm-0001-3-81c6d01e371b/, uncommitted); committed as 16-hex prefixes in
  results/c-u2-pm-0001-3/81c6d01e371b/runs.txt:16, :34, :28. U4c's L-c8 whole-v2-log prefixes are in
  results/c-u2-pm-0001-3/65869918809f/runs.txt (43745683ab36d67b, 722f20713e18352e, 432693efb9698c05).
- F12. No crates/lab/clippy.toml exists (ls crates/*/clippy.toml: client, l1, link, log, proto, room, server, sim);
  server/clippy.toml has the clock ban but no hash-map ban.
- F13. marks.json holds no PM-0004-21, PM-0004-74 or S-HashMap (git grep): marks.json's marks are ADR 0001's PM-0001
  rows, PM-0004-67 and PM-0006-11 to -27 (check.mjs:1355-1361). "marks S-HashMap" in the plan is the marks scout's
  finding (u4-scout-marks.md:18; plan :12-15 names the scouts' findings "marks Fn"); PM-0004-21 and -74 are ADR 0004's
  rows (0004:1142, :1184).
- F14. plan.md §4.1 (:392-393) and §4.2 (:435-447) already hold hash-wasm's layout line and "three named exceptions"
  (u4-build-plan.md :687 and :692-693's plan.md edits are done).
- F15. U4e's patch (wip/u4e.patch) touches only crates/room, crates/server (not schedule.rs) and crates/client: no
  file of U4d2's lane.

## Choices where the plan is silent (each written into the brief, marked "plan silent")
- C1. Exports take and return 32-bit integers (u32 or i32) only, as the plan's lo/hi split implies: no i64 (Node
  would pass BigInt).
- C2. The wrapper's state lives in a `thread_local!` RefCell (no `static mut`, no unsafe; per-thread, so the native
  unit tests do not share it); a Mutex static is the alternative.
- C3. `dump_len` takes the dump once; `dump_word(i)` reads that copy (no dump per word: an SS dump is 6,488 words).
- C4. The bite argument of `begin` is 0 for none, else k for `World::BITES[k - 1]`; ids map to indices at run time
  from npt-lab's bites build (it prints World::BITES), never from a copy in JS or in hash-wasm's sources (one home:
  check.mjs's bite-home rule, :734-758).
- C5. The Wasm leg's hash-map order digest is read from its dump at the arming tick (the folded word's field, named
  in the native dump), so no export is added.
- C6. T22's committed state is one scene's initial dump (CO, 260 words: the smallest), committed as a vector file in
  crates/hash-wasm/; its digest is its state hash; read on wasm32 through a header-only recording (F10).
- C7. The manifest's "script" (§4.15:502) for a synthetic entry is the generator's other arguments (first tick, tick
  count, span ms); it lives in crates/lab/ (npt-lab regenerates; U4d2's lane).
- C8. The `wasm` step sits after `deps` in STEPS (the flags it builds with are checked first); 11 steps.
- C9. The flag and feature checks are deps-step lines (plan §2's Features row :157 says "a deps-step line"); their
  subprocesses (cargo tree, rustc --print cfg) are injected into depsStep as `metadata` is, so check.test.mjs's
  fixture test (:1094) gains one plant per new check.
- C10. If the step runs over 5 minutes warm, U4d2 reports the split and leaves the step in check.mjs; moving it to
  ops/runs/det-u4.mjs is U4r's, on the advisor's ok (plan :790-792).
- C11. The loader instantiates with an empty import object, so a module that imports anything fails to instantiate
  (LinkError): no host clock or random source can reach the Wasm leg.
- C12. npt-lab gains no dependency (serde_json and sha2 are already its own: lab/Cargo.toml); Cargo.lock changes by
  npt-hash-wasm's own entry only.

## Conflicts found (plan vs rulings vs code), each a U4D2-Q in the brief
- K1 (Q1). Drifted cites in plan §5 U4d2: check.mjs:25 (LEAF, now :31), :1110 (WASM_CRATES, now :1438), :135-137
  (now :161-174), :94-102 (tcpinfo's rule, now :107-121), :903-942 (env and configs, now :1196-1284), :1156-1158 (the
  runner note, now :1527-1529), :135 (UNSAFE_EXCEPTIONS, now :169).
- K2 (Q2). "the release build with §4.5's flags" (:675): the build plan's §4.5 (:292-304) holds no flags; plan.md
  §4.5:533-535 does.
- K3 (Q3). `begin(seed_lo, seed_hi, scene, bite, arm_tick)` (:668): the Replayer reads scene and seed from the
  recording's header (F7); the World has no seed accessor; nothing in the code takes them.
- K4 (Q4). The read-back "with `rustc --print cfg` per target" (:680-682): a bare `rustc --print cfg` sees no cargo
  rustflags (env or config), so it cannot see a flag set there; `cargo rustc --print` is unstable on 1.99.1 (F2).
- K5 (Q5). The std-sin bite's "input from a scan" (:696-697; ADR 0004:1142 "the sin input taken from PM-0004-22's
  table"): the site U4d built takes its input from the vehicles' loops and `World::with_bite` takes only the arming
  tick (F8); the 1-ulp stand-in has no home in U4d2's lane (crates/sim is nobody's in wave 2).
- K6 (Q6). The features comparison names "the server build against the hash-wasm build" (:677-678), but the native
  legs run in npt-lab's build (:670-671), which the comparison does not cover; ADR 0004:1184 says "every build that
  runs the sim".
- K7 (Q7). The invariant has two clauses (names, integer signatures: plan.md:444-446; advisor cond 7 (b)); the
  named test (the loader's export list) covers names only.
- K8 (Q8). "no exported name collides with another symbol": glibc's compat `step@GLIBC_2.2.5` (F5) shares a name
  with the plan's `step`; harmless today (non-default version), but the invariant is not true by construction on the
  native build.
- K9 (Q9). Carries 29 and 41 name "U4d2 or the main session (the workspace Cargo.toml, crates/proto/Cargo.toml and
  check.mjs)" (carries :407), while Lanes :863-865 give U4d2 "Cargo.toml's members" and not crates/proto/Cargo.toml;
  the workspace Cargo.toml's lints comment (:74-92: "Two crates are ... exceptions") must change with the third
  exception.
- K10 (Q10). §4.11 (:393-395) and Q9 (a) (:1002-1005; advisor-u4.md:207) give npt-lab a clippy.toml (the clock ban;
  Q9 (a) adds the hash-map bans); none exists (F12) and no sub-unit's row builds it; U4d2 is the only wave-2 editor of
  crates/lab and adds its first U4 code. (Also: server/clippy.toml lacks Q9 (a)'s hash-map bans: not U4d2's lane,
  reported for the advisor.)
- K11 (Q11). T22's word-size bite (ADR 0004:1142: "a planted usize field in the serialized state must change the
  bytes between the two targets") is not in U4d2's Bites line (:696-700); without a plant the vector test has no
  shown failure.
- K12 (notes only, not a question). The task's source list calls PM-0004-21, PM-0004-74 and S-HashMap "marks.json"
  rows; none is (F13). The brief says where each lives and that no U4d2 bite is judged by npt-judge.

## Carries: what applies
- §U4d2 (:407-446): 29 (workspace Cargo.toml's libm comment), 41 (proto's bites comment: lane question Q9), 42 (met
  by U4a: entry 34 met; no action), 52 (pin only GENERATOR 2's recordings and U4d-F's ES; name adb1bab), 65 (check.mjs
  names failing tests; "the main session may make this change sooner": check at launch), 66 (the probe's datagram
  SHA-256; three probe runs equal U2's).
- §recordings (:351-379: 67, 71, 72): U4d2's work is not such a change as planned: it changes no crates/sim file,
  regenerates with `Recording::synthetic` as merged and pins the bytes D-7 already pins (F9). They bind only if a
  stop is lifted into a crates/sim change (Q5's option (b), or Q11's plant if it were kept), and then the change's
  owner runs them.
