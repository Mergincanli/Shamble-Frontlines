# U4d2 brief: drafter's notes (rust-reviewer, 2026-11-12)

No Edit tool in this agent: this file and the brief are rewritten whole with Write at each save (the task's rule). No
cargo build, test, clippy, doc or check.mjs was run. Read-only commands run: git log/diff/merge-base/status, `rustc
--print cfg` (host and wasm32), `cargo rustc --print cfg` (refused by stable cargo before any build; scratch target dir
in the session scratchpad, nothing created), `cargo tree` (offline, --locked, resolution only), `nm -D` on glibc,
`node --version`, `sha256sum`, `wc`, `grep`/`sed` reads. Web: two official Cargo pages (cargo-rustc, config), read as
data.

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
- [x] sources read (steps 1-8)
- [x] brief §0-1 (first save)
- [x] brief §2 (second save)
- [x] brief §3-8 (third save, 717 lines, sha256 ca2c3373…)
- [x] cite pass over the whole brief, then a whole rewrite with the fixes (fourth save, 719 lines, sha256 e5869dbf…):
  u4e-run.sh :15-16 → :16-17 and :28-31 → :30-32; u4e-brief.md :733-742 → :734-742; plan :691-693 → :688-693 (the
  third exception); plan :700 → :699 ("an export not on the loader's list"); "no wasm-opt" → "wasm-opt is not run
  here: C-U7-1"; the usize plant's site named (world.rs:1018 `dump`, hash.rs:14's word sink); `<crate>` in T4 item 2
  read as "npt-hash-wasm, npt-lab or check.mjs (its self-tests)"; the -3 script's lines (:43-45, :52-59, :55, :65-72).
- DONE: the brief is rev 1; the return goes to the workflow.

## Sources read (file:line, at the main folder's head cea8209; spikes/ equal to 28a85e7: `git diff --stat 28a85e7 HEAD -- spikes/` empty)
- u4e-brief.md (982 lines, rev 3.3): structure §0 :120-135; §1 :139-172; §2 :176-564; §3 :568-717; §4 :721-781;
  §5 :785-801; §6 :805-834; §7 :838-912; §8 :916-982.
- u4-build-plan.md (1,079 lines, sha256 686a583c…): §5 U4d2 :666-702; §4.1 :178-203 (hash-wasm :202; npt-lab
  subcommands :198; Recording/Replayer :185); §4.2 :205-249; §4.3 :251-265; §4.5 :292-304 (baselines: no flags);
  §4.11 :392-407; §4.14 hash-map row :476; §4.15 :478-504; §5 intro :507-512 (Cargo.lock :509); U4d :602-643 (D-2
  :612-614; D-7 :629; bites :640-642); U4r :740-793 (HashCompare :766-767; det-u4.mjs :790-792); Lanes :840-875;
  §6 U7 :885-887; §7.1 :896-908; §7.2 :910-918; §2 Features :157, C-U7-1 :159; RQ4/RQ5 :117-118; T9 :69, T22 :82,
  A8 :91, A12 :95, A13 :96; Q3 :981-985; Q9 :1002-1005; Q21 :1040-1042.
- plan.md (rev 3.3, sha256 2bef127b…): §4.1 :366-419 (hash-wasm :392-393); §4.2 :421-454 (three exceptions
  :435-447, hash-wasm :442-446, how read :449-450, narrow surfaces :454); §4.5 :527-551 (flags :533-535; features :536;
  the check :546-551, Chromium :548, bites :551); §4.7 npt-lab :617 (`HashCompare`), binaries :611-612.
- u4s-carries.md (sha256 cdab0240…): §U4d2 :407-446 (29 :408-411, 41 :412-414, 42 :415-417, 52 :418-428, 65 :429-435,
  66 :436-446); entry 34 :62-69; recordings section :351-379 (67 :352-369, 71 :370-374, 72 :375-379); carry 39
  :169-179.
- Entry 34: met by U4a (u4a-build.md:125-126; history.md:1335; u4ab-review-test.md:90; u4ab-review-rust.md:27). Carry
  42's condition ("If U4a's report does not list entry 34 as met") is false: U4d2 takes no word of entry 34.
- advisor-u4.md: cond 2 :90-101; cond 7 :130-136; cond 8 :137-139; cond 10 :143-149 ((b) :145-146); cond 12 :154-158;
  nit (g) :169-171; Q3 ruling :198-199; Q9 :207; U4s cond 2 :313-327 (carry 41's origin :324-326); cond 4 :334-335;
  U4S-BUILDER :365-371; F2 (3) :450-458 ("in wave 2 after U4d2 has merged" :456).
- advisor-u4-wave1.md: W1-PUB :353-435 (U4d part :387-416); W1-U4D's plan text :300-301; W1-TARGET :558-586 (lock
  :570-573, provenance :574-575, disk :576-580).
- advisor-u4d-close.md: B1.2 :42-134 (closed loop refused :48-51; who lands U4d-F :116-119); B1.3 :165-180; B2
  :229-308 (U4e's carries; carry 52's text :136-150).
- advisor-u4df.md: R1 :28-87 (RM :71-85; "fix the scene before U4d2 pins" :53-56); R3 :182-210; R4.1 :213-231
  (TE2-5 stop clause for next time :228-231); R5 M9 :294-297; R6 :301-322; T4 :459-470.
- advisor-u4j-r2.md: B2.3 :146-171 (carry 65's half (a) :158-160; the rule :167-171).
- advisor-u4e-brief.md: F2-ROUTE :277-310; L4 :335-336.
- netcode-proof-test.md (the card): base counts :23-25; U4e's state :26-62; U4d2's brief workflow :63-70; Rules in
  force :137-176 (USAGE-SAVINGS :138-147; U4 carries word for word :158-160; TE2-5 rule :161-165; B6 :166-167; T4
  :168-176).
- history.md: Decisions taken :280-309 (USAGE-SAVINGS line :291, builders' limits :292-293).
- u4e-run.sh (90 lines: exports :16-17, log check :30-32) and main-run.sh.
- Code (spikes/netcode-proof/ at 28a85e7): Cargo.toml (members :6-23; libm comment :50-53; lints comment :73-92, forbid
  :93, clippy :95-99; release overrides :108-112); rust-toolchain.toml; .cargo/config.toml; crates/proto/Cargo.toml
  (bites comment :13-14); crates/lab/{Cargo.toml, src/lib.rs, src/main.rs (doc :1-12, params :116, summary :167-183,
  dispatch :186-213, tests :215-275), src/probe.rs (sha256 :210-217, :238-242; tests :415-440)}; crates/sim/src/
  {replay.rs, world.rs (bites :585-635, loops' periods :794, BITES :934-938, with_bite :940-966, digest :968-973, sinf
  :994-1000, dump :1018, place_vehicles :1215-1228), hash.rs (sink :14, StateDump :66-115), lib.rs};
  crates/sim/tests/replay.rs; crates/tcpinfo/{Cargo.toml, src/lib.rs, src/linux.rs}; crates/client-wasm/Cargo.toml
  (:9-13 "two unsafe_code exceptions"); crates/server/{Cargo.toml, clippy.toml}; crates/l1/src/lib.rs :325-352;
  crates/l1/tests/l1.rs :478-532; check.mjs (:1-18, :31, :80-159, :161-216, :230-312, :364-380, :382-452, :489-543,
  :689, :734-758, :1013-1093, :1196-1284, :1355-1361, :1398-1403, :1411-1421, :1433-1500, :1502-1590, :1594-1654);
  check.test.mjs (:1-98, :1094-1165, :1246-1249, :1617-1698); ops/runs/c-u2-pm-0001-3.mjs (:43-45, :52-59, :65-72);
  results/c-u2-pm-0001-3/{81c6d01e371b, 65869918809f}/runs.txt; marks/SERIES.md :385-405; docs/adr/0002 :252-266,
  :498-506; docs/adr/0004 :1138-1145, :1184.

## Facts read or measured (each with its source)
- F1. `rustc --print cfg` at 1.99.1: host target_feature fxsr, sse, sse2 only (no fma); wasm32-unknown-unknown:
  bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext (no simd128, no
  relaxed-simd), panic="abort", pointer width 32.
- F2. `cargo rustc --print cfg` on cargo 1.99.1: "error: the `print` flag is unstable, and only available on the
  nightly channel of Cargo" (cargo issue 9357). The stable cargo-rustc page documents no --print
  (https://doc.rust-lang.org/cargo/commands/cargo-rustc.html). So the read-back cannot ask cargo for its flags.
- F3. Cargo's rustflags sources (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags): four
  mutually exclusive sources, first used: CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, all matching target.<triple>/<cfg>
  rustflags joined, build.rustflags; with --target, host artifacts get none.
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
  scene from the caller; World has no seed accessor.
- F8. `World::BITES` (world.rs:934-938) = PM-0004-21/hash-map, PM-0004-21/std-sin, PM-0001-41/server-speed-1pct,
  PM-0001-41/server-quant-differs; the std-sin site replaces libm::sinf in the vehicles' loops only
  (world.rs:994-1000, :1216-1228): theta = TAU * along / period, along = (elapsed + phase) mod period, period =
  ticks_for_ms(60,000 + 5,000 v) (:794), at most 3,900 ticks for 15 vehicles. Its input is chosen by the loops, not by
  the caller: `World::with_bite` takes the arming tick only (:944-966).
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
  server/clippy.toml has the clock ban but no hash-map ban; crates/lab/src holds no HashMap, HashSet, Instant or
  SystemTime (grep).
- F13. marks.json holds no PM-0004-21, PM-0004-74 or S-HashMap (git grep): marks.json's marks are ADR 0001's PM-0001
  rows, PM-0004-67 and PM-0006-11 to -27 (check.mjs:1355-1361). "marks S-HashMap" in the plan is the marks scout's
  finding (u4-scout-marks.md:18; plan :12-15 names the scouts' findings "marks Fn"); PM-0004-21 and -74 are ADR 0004's
  rows (0004:1142, :1184).
- F14. plan.md §4.1 (:392-393) and §4.2 (:435-447) already hold hash-wasm's layout line and "three named exceptions"
  (u4-build-plan.md :687 and :692-693's plan.md edits are done).
- F15. U4e's patch (wip/u4e.patch) touches only crates/room, crates/server (not schedule.rs) and crates/client: no
  file of U4d2's lane.
- F16. The -3 script refuses a dirty tree (:43-45), builds into and copies from the main folder's target (:52-59,
  `built` :55), and pushes each probe line as printed (:65-72); SERIES.md's header-params table holds
  `fate_log_sha256` at :396; the probe has no SERIES test.

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
  crates/hash-wasm/; its digest is its state hash; read on wasm32 through a header-only recording (F10); every other
  scene's initial state is checked for native-against-Wasm equality, so D-7's pins keep one home.
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
- C13. The PM-0004-21 bites run in the `wasm` step's bites half on every check.mjs run (plan.md §4.5:551 makes the two
  bites part of "the check"); the hash-map run stops at its first mismatch (tick 1,234), so it is cheap; std-sin runs
  there only if it is catchable (Q5).
- C14. The loader and the deps scan share one list of export names, in one JS module the other imports.
- C15. Carry 66's runs go through `npt-lab probe` directly (the -3 script refuses a dirty tree and reads the main
  folder's target: F16), with -3's arguments; their values are compared with U2's full SHA-256s (F11). (Rev 1.1:
  the comparison is now an `#[ignore]` test's assertion: TB-10.)
- C16. T4 item 2's `<crate>` for U4d2 is "npt-hash-wasm, npt-lab or check.mjs (its self-tests)": the unit changes two
  crates and the test script. (Rev 1.1: replaced by the check's string, "npt-hash-wasm, npt-lab, check.mjs's wasm and
  deps steps or check.test.mjs": TB-7.)
- C17 (rev 1.1). The std-sin scan's result is a committed text file in crates/lab/ beside the manifest, with rustc's
  and libc's versions; a moved T' FAILs naming the libc the step runs on (TB-2).

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
  tick (F8); the 1-ulp stand-in has no home in U4d2's lane (crates/sim is nobody's in wave 2: close :117-118).
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
  the workspace Cargo.toml's lints comment (:73-92: "Two crates are ... exceptions") must change with the third
  exception.
- K10 (Q10). §4.11 (:393-395) and Q9 (a) (:1002-1005; advisor-u4.md:207) give npt-lab a clippy.toml (the clock ban;
  Q9 (a) adds the hash-map bans); none exists (F12) and no sub-unit's row builds it; U4d2 is the only wave-2 editor of
  crates/lab and adds its first U4 code. (Also: server/clippy.toml lacks Q9 (a)'s hash-map bans: not U4d2's lane,
  reported for the advisor.)
- K11 (Q11). T22's word-size bite (ADR 0004:1142: "a planted usize field in the serialized state must change the
  bytes between the two targets") is not in U4d2's Bites line (:696-700); without a plant the vector test has no
  shown failure.
- K12 (notes and brief §0 only, not a question). The task's source list calls PM-0004-21, PM-0004-74 and S-HashMap
  "marks.json" rows; none is (F13). The brief says where each lives and that no U4d2 bite is judged by npt-judge.
- K13 (brief 6 item 9, not a question). crates/client-wasm/Cargo.toml:9-13 says "one of the workspace's two
  unsafe_code exceptions": false once hash-wasm lands; not U4d2's lane: the builder reports it for the main session.
- K14 (not a question). The card stamps u4-build-plan.md as "d928358b…" (in its resume card, "The current step"); the
  file at the head is 686a583c…, so the plan changed after the card's stamp; the brief cites the file at the head and
  stamps 686a583c….
- K15 (Q12, rev 1.1). Two of the check's fixes read differently against the code: TB-7's pin list (:1099 and the steps
  pin do not fail from the edit) and TB-11's fixed list of five packages (replaced by npt-sim's manifest-derived set).

## Carries: what applies
- §U4d2 (:407-446): 29 (workspace Cargo.toml's libm comment), 41 (proto's bites comment: lane question Q9), 42 (not
  applicable: entry 34 met at U4a), 52 (pin only GENERATOR 2's recordings and U4d-F's ES; name adb1bab), 65 (check.mjs
  names failing tests; "the main session may make this change sooner": check at launch), 66 (the probe's datagram
  SHA-256; three probe runs equal U2's).
- §recordings (:351-379: 67, 71, 72): U4d2's work is not such a change as planned: it changes no crates/sim file,
  regenerates with `Recording::synthetic` as merged and pins the bytes D-7 already pins (F9). They bind only if a
  stop is lifted into a crates/sim change (Q5's option (b)), and then the change's owner runs them; the scratch plants
  of Q5 (a) and Q11 (b) are restored and change no committed byte.

## Placeholders left for the main session (brief 7.2 item 3)
- Rev 1: P1 carries :407-446; P2 history.md:291-293; P3 card :169-171 (df-ruling :462-464); P4 card :172-175
  (df-ruling :465-468) with `<crate>` read per C16; P5 card :162-165 (j-r2 :167-171); P6 `<base>` and its counts; P7
  the runner (u4d2-run.sh).
- Rev 1.1 (TB-1): P1 carries by its heading "## U4d2 or the main session" to the line before the next "## ", entries
  29, 41, 42, 52, 65, 66 asserted (at fa91186 :407-447); P2 history by its first words "  - Builders: pass or fail
  comes from the exit code" and the one continuation line, the line before asserted to be "  USAGE-SAVINGS: (a), all
  three, with these limits."; P3 df-ruling :462-464 (the quoted sentence); P4 df-ruling :465-468 (the quoted clause,
  both `<crate>` replaced); P5 j-r2 :168-171 (the quoted rule); P8 j-r2 :298 (B6's sentence after "From now on: ");
  df-ruling's and j-r2's sha256 asserted against section 0's stamps; P6; P7.

---

## Revision for the check (rev 1.1; a fresh copy of the drafter, 2026-11-12)

Input: u4d2-brief-check.md (test-engineer, verdict REVISE: TB-1, TB-2, TB-3 High; TB-4 to TB-11 Medium; TB-12 to TB-20
Low). Head at start: fa91186; spikes/ and docs/ equal 28a85e7 (`git diff --stat 28a85e7 HEAD -- spikes/ docs/`
empty); stamps: plan 686a583c…, plan.md 2bef127b…, carries cdab0240…, card 6b36da10… (602 lines), brief rev 1
e5869dbf….
**Own slips:** (1) at the start two `cp` calls copied rev 1 of the brief and of these notes into the session scratchpad
(brief-rev1.md, notes-rev1.md); (2) one `rustc --print cfg ... 2>` redirect wrote rustc's warning to a scratch file
(x.err) in the session scratchpad. Both are shell writes against "Write and Edit tools only"; both outside the repo,
used only to read; no repo file was written but this one and the brief, always by Write.

### Plan
1. Verify each finding against its source (code at the head, the card, the rulings): one block per group
   (TB-1 placeholders; TB-2/TB-5/TB-15 hash-map and std-sin; TB-3/TB-16 flags; TB-4/TB-18 word size; TB-6/TB-9/TB-8
   step; TB-7 stop clause; TB-10/TB-11/TB-12/TB-13/TB-14/TB-17/TB-19/TB-20).
2. Record each finding's outcome below (applied as the check says, applied with a change, or kept with a
   question for the advisor), with the source lines read.
3. Rewrite the brief whole as rev 1.1 (title, "Changes in rev 1.1"), section by section, saving after each.
4. A cite pass over the changed text; final stamps; the return.

### Progress
- [x] plan written
- [x] block 1: every finding verified (below); new facts F17-F25 measured or read.
- [x] block 2: brief rewritten whole as rev 1.1 by Write (one save: the whole file is one Write; the outcomes were
  saved here first, so a cut run could redo it from these notes).
- [x] block 3: cite pass; fixes folded by a second whole Write: e-r1 R9 M1 :444-447 → :445-448; R9 :438-470 →
  :438-493; the base counts' card cite "The current step" → its "Merged into the spike branch" bullet; rule 5's
  sentence on h7 (ii)'s ignored test corrected (F25); h7 (ii)'s command given `--exact <its full path>`; a typo in
  2.2 ("`hash_hi`'s partner"); the Changes line for TB-2 and TB-6 made exact. Checks: no card line number left
  (`grep "card :[0-9]"` empty); placeholders P1-P8 all present; 20 TB ids in the Changes list.
- DONE: the brief is rev 1.1 (929 lines, sha256 e6bcefc8…); the return goes to the workflow.

### New facts (rev 1.1)
- F17. The card moves: at fa91186 "Rules in force" starts at card :164; TE2-5's rule :189-192, B6 :193-195, T4 item 1
  :196-198, item 2 :199-202, RM :203; "carries word for word" in the U4 bullet :185-186; next step 3 (U4d2's brief,
  "with the advisor") :140; the base counts in the resume card's "Merged into the spike branch" bullet (:20-25). The
  check saw them 7 lines higher at a7f3906; rev 1 cited cea8209's.
- F18. Closed sources for the placeholders (stamps at fa91186): advisor-u4df.md (531 lines, sha256 842ec092…) T4
  item 1 :462-464, item 2 :465-468; advisor-u4j-r2.md (657 lines, sha256 7efa9410…) the rule :167-171 (the quoted
  words from :168 "a FAIL in check.mjs's test step" to :171 "until it is explained."), B6's sentence :298 ("From now
  on: plants by Edit in place, or the planted file's diff saved beside the run's output."); history.md (1,925 lines,
  sha256 c62ac831…, "Decisions taken" newest first): "  USAGE-SAVINGS: (a), all three, with these limits." :291, the
  builders' bullet :292-293 (its first words found once: grep). u4s-carries.md headings: §U4d2 :407, next "## " :448;
  entries at line start 29 :408, 41 :412, 42 :415, 52 :418, 65 :429, 66 :436.
- F19. advisor-u4e-r1.md (sha256 7c129309…) T2 (:504-600) inserts carries 73-81 into §U4r (after 69), §U4f (after
  46), §U4g (after 70), §U4h (after 45), all above :407; T6 (:681) edits plan §4.1's npt-room, npt-server and
  npt-client rows; R9 (:438-493): M1 :445-448, M2 :449-453 and M5 :459-466 recompute U4e's counts if U4d2 merges
  first; R6 (:317-353), its :349: check.mjs keeps cargo's Compiling lines out of its output.
- F20. std 1.99.1 (https://raw.githubusercontent.com/rust-lang/rust/1.99.1/library/std/src/sys/random/mod.rs): the
  `cfg_select!` arm `all(target_family = "wasm", target_os = "unknown")` routes to `unsupported`, whose
  `hashmap_random_keys` (…/sys/random/unsupported.rs) returns the addresses of a stack local and of a `Box<u8>`
  ("a bit of randomness"); no rust-src component is installed here, so read on GitHub at the toolchain's tag.
- F21. `rustc --print cfg` (1.99.1, read only): `-C target-cpu=haswell` on x86-64 lists `fma` (with avx, avx2, …);
  `-C target-cpu=bleeding-edge` on wasm32 lists `simd128` and `relaxed-simd`; `-C target-feature=-fma`, `+sse2` and
  `-C target-cpu=x86-64` list fxsr, sse, sse2 only; a flag meant for the other target only warns ("not a recognized
  processor for this target (ignoring processor)") and exits 0, so giving every flag to both read-backs adds no false
  failure.
- F22. world.rs: `dump` (:1018-1022) and `state_hash` (:2141-2145) both call `write_state` (:1926; also
  `non_finite`, :2029); the hash-map fold is written by `wide` (two words, one name: hash.rs:21-24) at :2017-2019;
  std-sin feeds only `vehicle[v].pos.y` (place_vehicles :1225; written at :1959). `Fnv` is pub(crate) (hash.rs:33);
  `StateDump::hash` (:98) is the public route to the hash.
- F23. check.test.mjs: the pins quoting "only npt-client-wasm and npt-tcpinfo may" are :288, :486, :508, :1141 (the
  message is built from `UNSAFE_EXCEPTIONS`: check.mjs:493); the deps fixture's summary pin :1099 is a regex
  `^held: .* \(6 crate files and 1 cargo config file\(s\) read\)$` (fails only if DEPS_CHECKED stops starting "held:"
  or the fixture's file counts change); the steps pin (:1617-1658) builds its wasm32 line from `WASM_CRATES` and
  pins no step list; 52 tests.
- F24. npt-lab's probe runs on simulated time (main.rs:1), 1,000,000 packets at 10,000 a second by default
  (probe.rs:20-22; main.rs:73-75); `probe::run(&Setup)` (:116) is callable from a test; the workspace's run-by-name
  precedent is sim/tests/world.rs:1188 (`#[ignore = "... run once by name for the report: about 30 s"]`). u4e-run.sh
  is 98 lines now: exports :16-17, lock :38-39, log check :30-33, touch first and every 2 s :58-62, provenance
  :91-97. main-run.sh runs on the shared target (:14). npt-sim's manifest names libm, rand_chacha and rand_core
  (crates/sim/Cargo.toml).
- F25. check.mjs's `cargoCounts` (:1411-1421) sums libtest's "N passed; M failed" only, so an `#[ignore]` test is in
  no count (libtest reports it as ignored); the base passes with sim/tests/world.rs:1188's ignored test, so an ignored
  test does not fail the test step.

### Each finding's outcome
- TB-1 High: APPLIED. P1 by heading (copied from "## U4d2 or the main session" to the line before the next "## "),
  with the entry set asserted; P2 by its first words in history.md, with the line before asserted; P3, P4, P5 and the
  new P8 (B6) from the closed rulings by line range, each file's sha256 asserted against section 0's stamp and the
  copy's first and last words asserted; `<crate>` replaced in P4 by the script (TB-7); every card line number dropped
  (the card is cited by section and item); 7.2 gains the re-stamp at launch and the re-check of plan cites (item 8).
- TB-2 High: APPLIED. 2.4 (a) item 6 and 2.7 give the bites half's verdict rules (hash-map: D-2's rule per pair,
  sim/tests/replay.rs:300-308, and a FAIL if no pair was exercised; std-sin: the scan's result committed as data in
  crates/lab/ and the step's (recording, T', field) rule, or the data's "not catchable" line printed); the self-test
  gains the four recorded outcomes per bite. Added (plan silent): the scan data names the libc it was scanned on, and
  a moved T' FAILs naming the libc the step runs on (CI's glibc may differ). Q5's options re-worded to say what the
  committed step holds under each, and (b) named as the choice if a sin catch must be in every committed run.
- TB-3 High: APPLIED. Two live runs (`node check.mjs --only deps` under the runner, -x; no build: the deps step
  compiles nothing) with RUSTFLAGS="-C target-cpu=haswell" and
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C target-cpu=bleeding-edge", each FAIL by name on the read-back; the
  +fma run stays the name scan's; the argv recorder pin per source; the clean read-back must list sse2 / sign-ext.
- TB-4 Medium: APPLIED. Plant site `write_state` (world.rs:1926); expected outcome split by pair.
- TB-5 Medium: APPLIED. Two Wasm hash-map legs in two Node processes; the URL at tag 1.99.1 cited (F20).
- TB-6 Medium: APPLIED. Exact counts (the manifest's 9,000 per recording, 27,000 per pair); the six self-test cases.
- TB-7 Medium: APPLIED WITH A CHANGE (U4D2-Q12 (a)). The expected failures named are the four pins that quote the
  exception list (F23); :1099 is named as failing only if the fixture's file counts or DEPS_CHECKED's "held:" start
  change; the steps pin is named as one U4d2 extends (2.4 (g)), not one the edit breaks. `<crate>` replaced in the
  copy by the check's string; the wasm step's no-retry sentence added (outside the copy).
- TB-8 Medium: APPLIED. The four release builds before check.mjs inside the same -x runner call are the release
  provenance.
- TB-9 Medium: APPLIED. check.mjs times what it calls (npt-lab hash as one call); lane timings labelled "under the
  runner's touch loop"; CI's figure is the infra-engineer's from CI's run.
- TB-10 Medium: APPLIED. An `#[ignore]` test in npt-lab, run by name on the release build, asserts U2's three full
  values through `probe::run`; the three binary runs with -3's arguments stay for the carry's "the commands it ran".
- TB-11 Medium: APPLIED WITH A CHANGE (U4D2-Q12 (b)). The live plant in hash-wasm's own manifest, -x, restored; the
  non-vacuity check reads npt-sim's normal dependencies from cargo metadata (libm, rand_chacha, rand_core at the head)
  and fails unless each list holds npt-sim and each of them, rather than a fixed list of the five packages (a pinned
  list would fail the deps step on any lawful dependency change; equality across the builds already catches a
  differing set).
- TB-12 Low: APPLIED (check.mjs names the two ids it arms; no copy of World::BITES in JS or hash-wasm; no id literal
  in a crate's src outside npt-sim, which the bite-home rule refuses: check.mjs:734-758).
- TB-13 Low: APPLIED (the panel in 7.2 item 7; the header cites the card's next step 3).
- TB-14 Low: APPLIED.
- TB-15 Low: APPLIED.
- TB-16 Low: APPLIED (the three controls measured: F21).
- TB-17 Low: APPLIED.
- TB-18 Low: APPLIED.
- TB-19 Low: APPLIED.
- TB-20 Low: APPLIED.
