# The advisor's ruling on U4d2's round 2 (2026-11-13, from advisor-u4d2-r2-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md`
(read at origin/main). It rules; it edits no code and runs nothing that builds or touches a copy.

## Plan
1. Read the brief whole (done), main's advisor.md (done), my round-1 ruling whole (done: Blocks 1 to 9, T1 to T5,
   "Before the merge").
2. Read the five round-2 seats' notes whole (security, rust, infra, test, arch), then the builder's "Fix pass, round 1"
   section (claims only), the packet's head and outline.
3. Spot-check the load-bearing claims by reading, git and sha256 only: the copies' hashes against fx-hashes-end.sha256
   and r2-hashes-start.sha256; the patch's sha256 and its commit; FX-7's timeout code and run() wiring (INF-1's form);
   the lane lock's place against M9's clean (INF2-4); the TP10 removal (check.mjs's sha256, the labelled block); the
   counted logs' totals (fx-check1/2, r2-te-check); the leftover folder.
4. Rule, one block per item (1 to 6 of the brief), each written to this file as it is finished.
5. The amended texts (T3, T5 whole, in code blocks), then the two lists (before the merge; for Mustafa), then the
   summary for the caller.

## Notes (one line per check, as done)
- Read whole: the brief (73 lines); advisor.md at origin/main; my round-1 ruling (730 lines: Blocks 1-9, T1-T5,
  "Before the merge", the verdict table); the five seats' notes (infra 190, security 201, rust 141, arch 127, test 254
  lines); the builder's "Fix pass, round 1" (u4d2-build.md:258-556, its claims).
- Hashes, checked now: `cmp` fx-hashes-end.sha256 r2-hashes-start.sha256 equal; `sha256sum -c` r2-hashes-start from
  .gate/u4d2-r2's root 18 OK, from the builder's copy's root 18 OK (0 not OK); the copy's HEAD 51520b43391a…, `git
  status --short --untracked-files=all` 19 lines (the 18 and the record). wip/u4d2.patch sha256 9e71cda07d4a…, its blob
  ba0f25bd… unchanged since cb27931 (`git diff --quiet cb27931 HEAD --` it).
- M2, checked now at the head ff6331d: `git log 51520b4..HEAD -- spikes/` is c88eb31 (U4e) and 1b4a3c8 (A13) only;
  `git diff --quiet 51520b4 HEAD --` U4d2's lane paths and crates/sim: true; `git status --short spikes/` empty.
- The texts' anchors, counted now: T1 (a) and (b) once each in u4-build-plan.md; T3's `or states why not |` once in
  plan.md (:1247), C-U7-2 absent; T4's `fate_log_sha256` row once in marks/SERIES.md; u4s-carries.md §U4r ends at
  entry 77 (:361) above "## The next change to crates/judge" (:372); entries 82 to 86 free.
- The logs at the lines the findings cite: r2-inf-hang.log (23 calls, 19 with the limit, 12 that would each wait out
  300 s); r2-inf-wasm1.log (lock 01:03:57, PASS 73.7 s, npt-lab build 35.9 s), r2-inf-wasm2.log (40.4 s), fx-check1
  (41.8 s), fx-check2 (40.5 s), r2-te-check.log (34.8 s; test 417 / 466; self 71 of 71; all 11 steps pass);
  r2-te-tp10c (not ok 38 and 54, 69 of 71, then deps exit 0 on the real tree); r2-te-sec-r2-3-macro-include.log (the
  wasm32 build, both clippy runs, the crate's tests and deps, each exit 0, deps' held line still claiming every include!
  literal names a .rs file); r2-te-inf2-2-run-timeout.log (71 of 71 with run()'s timeout dropped).
- Code read in the read copy: run() (check.mjs:1847-1856: spawnSync's `timeout`, `timedOut` from ETIMEDOUT);
  WASM_CALL_LIMIT_MS and `call` (:2212, :2255-2268: the limit by command, `cargo` none); WASM_BUILDS (:2120-2140: all
  four builds `--release`, two of them `--target wasm32-unknown-unknown`); the deps limits text (:1985-2006: "a flag
  reaching rustc another way ... is the Wasm leg's loader's to refuse"); loader.mjs's header (:7-15: "a flag reaching
  rustc by a route check.mjs does not read is refused too"); COMPILER_ENV and COMPILER_KEYS (:1400-1408: RUSTC_WRAPPER
  allowed, no linker entry); cargoConfigFiles (:1765-1772, `env.CARGO_HOME ?? ...`); the include rule (:680-688,
  `/\binclude\b(?!\s*!)/`) and pathProblems' include! read (:413, only before an opening bracket).
- A grep of the read copy's crates/*.rs: no `#[$` or `#![$` anywhere; `macro_rules!` only in crates/l1 (lib.rs,
  tests.rs); no `#[macro_export]`. So a refusal of a metavariable attribute costs nothing lawful workspace-wide.
- The runners: main-run.sh:10-21 and u4d2-run.sh:15-17, :45-46 (`set -u` only; `exec 9>"$LOCK"` then `flock`, neither
  checked; LOCK under .gate/shared-target). The shared target's root holds CACHEDIR.TAG (Nov 9), .lane.lock, debug, doc,
  release, tmp, wasm32-unknown-unknown. history.md:427: cargo 1.99.1 refuses to clean a folder without its CACHEDIR.TAG.
- The copies: .gate/u4d2-r2, .gate/u4d2-r1 and .gate/u4d2-prefix are git worktrees at 51520b4 (detached; the first two
  dirty with a patch applied); .gate/u4d2-rr-target holds a CACHEDIR.TAG (38 MB). CHECKPOINTS.md §10 (origin/main):
  plain `git worktree remove`, never `--force`; the card's "Housekeeping later" (approved method: leftovers to a local
  branch, then a plain `git worktree remove`).
- The leftover: /nonexistent-cargo-home, 12 KB: an empty .package-cache and registry/CACHEDIR.TAG (00:46).
  LEFTOVER-DELETE read at history.md:404-409, verbatim "leave them. ... it becomes a narrow kit rule instead."

## Rulings (one block per item of the brief)

### Block 1. Item 1: INF2-1 (Medium; the wasm step's time bound and a stopped call's children): APPROVE as a finding, not before the merge; a carry for U4r, entry 84 (T2). Never a third round.
- **The finding holds, and its label stands (Medium).** FX-7 gives each of the step's 19 non-build calls its own 300 s
  (check.mjs:2261-2266). A hang in npt-sim's replay, which every leg runs, makes 12 of them each wait out 300 s: 60 min
  under the exclusive lane lock (r2-inf-hang.log, a counted fake runner). spawnSync's timeout signals only its direct
  child, so `npt-lab hash`'s two leg processes can keep running after their parent is stopped: up to 6 busy processes on
  this 4-vCPU machine, unseen by the runners' idle wait, which looks for cargo and rustc only (the infra seat's probe:
  the grandchild survived; a group kill cleared it). The lane lock itself is not held by them (Node marks fd 9
  close-on-exec; the seat's live sample). The reach is real but narrow: it needs a hang, none occurs today, and the
  harm is time and a loaded machine (later timed runs and TE-M3's load-sensitive npt-net test), never a wrong verdict.
  So Medium, not High.
- **FX-7 meets the form I ruled, and the gap is in my form (my error, corrected in writing).** My round-1 Block 3 said
  each non-build call gets 300 s and "That is the step's whole budget". A limit per call is each call's bound, not the
  step's: under it the step can run 19 x 300 s, and I said nothing about what a stopped call leaves behind. The builder
  built what I asked; nothing is redone now.
- **Route: a carry for U4r (entry 84), not a follow-up.** U4r changes this step anyway (entries 82 and 83: its
  recordings feed it, and its clean half gains exit-code checks), and U4r's recordings are the first new input a
  replay could hang on. A card follow-up would have no named home. Before U4r, the step runs only at this merge's M5
  and at M9's one cold run, on code whose step passed six clean runs since the fix pass (fx-check1 and -2,
  r2-inf-wasm1 and -2, r2-inf-holders, r2-te-check), no call above 4.3 s. The interim rule is in M5 and M9 (Block 5):
  a call stopped by the limit is a wasm FAIL, which stops the merge and comes to me, after the main session lists and
  ends any leftover npt-lab or loader process (`pgrep -a npt-lab`, `pgrep -af loader.mjs`) and records them.
- **Entry 84's words** are in T2 (below). They ask for the property and its proof, not one mechanism: (a) one deadline
  shared by the step's non-build calls; (b) a stopped call takes every process it started with it; (c) two self-tests,
  one on the deadline (a fake runner and clock) and one on run() itself with real processes, which is also INF2-2's
  and TE2-R2-1's missing test. The infra seat's candidate for (b), spawnSync with `detached` and a group kill, worked
  in its probe, but `detached` is not documented for spawnSync in Node 22 (the seat's own read, medium confidence), so
  the carry asks for a documented means or the real-process self-test that proves the one used.

Block 1 written 2026-11-13.

### Block 2. Item 2: the Lows, each one's route (done; every label stands; none pushes on its own)
Round 1's rule applies (Block 4 there): a Low whose natural home is a later unit's change goes to that unit as a carry,
with its words in T2 or T3; a wording Low in a text of mine is fixed in that text now; the rest are card follow-ups
the report names. U4r is the next change to check.mjs (entries 82 to 84) and runs alone (u4-build-plan.md, Lanes:
"Then U4r ... then U4f, U4g and U4h in order"), so the deps step's small refusals go to it as one entry, 85.
- **SEC-U4D2-R2-1 (two comments claim more for the section read than it gives) with RR2-L1 (a linker is read by no
  rule): U4r's carry, entry 85 (a).** Both hold. A RUSTC_WRAPPER (allowed, for sccache: check.mjs:1400-1406) can pass
  link arguments that make wasm-ld write a section listing neither feature while the code uses relaxed SIMD (built:
  sec_probe_tf2_ld.wasm, r2-sec-loader.log part A); a linker set by CARGO_TARGET_<TRIPLE>_LINKER passes the deps step
  (r2-rr-superset.log run 3) and can do the same to the link step (by reasoning). **The overclaim began in my words:**
  my round-1 Block 1 said the section read "also catches ... a flag that reaches rustc by a route check.mjs does not
  read", and check.mjs:2001-2005 and loader.mjs:13-14 follow it. Corrected here: the section is wasm-ld's record of
  what it linked, so it catches a compile-time feature reaching rustc by an unread route unless the same route also
  rewrites the link; and the x86-64 legs have no artifact check. Entry 85 (a) refuses a linker setting (as COMPILER_ENV
  and COMPILER_KEYS refuse a compiler) and corrects both sentences; U4r's brief names loader.mjs's header sentence in
  its lane for that edit. T5 states the limit now, so the report is true before the code is.
- **SEC-U4D2-R2-2 (a cargo config's per-package profile turns npt-sim's overflow checks off, unread): entry 85 (b).**
  Shown by run (r2-sec-fx1-config.log: deps PASS; r2-sec-routes.log: rustc's `-C overflow-checks=on` gone). Native and
  Wasm wrap alike, so no divergence follows, but plan §4.5's "`overflow-checks = true` for npt-sim and npt-proto in
  every profile" is then off unseen. The refusal (any `profile` key in a cargo config read, and any environment variable
  starting CARGO_PROFILE_) costs nothing lawful: the spike's configs and the container set none.
- **SEC-U4D2-R2-3 with TE2-R2-3 (`m!(include!)` and a #[path] built from a `meta` fragment get past FX-6): entry 85
  (c).** Built by the security seat and settled live by the test seat: the wasm32 build, both clippy runs, the crate's
  tests and the deps step each exit 0, and deps' held line says "every #[path] and include! literal names a .rs file in
  its own crate" while the plant names a .txt outside the repository. FX-6's ruled form (V4's two forms) is met; this is
  a near form of the same class (a deliberate committed change past review, which check.mjs:1990-1997 names as the wall
  in the deny-level crates), so Low. **The form:** the seat's first option, "no macro_rules! in a deny-level crate",
  does not close it alone: by my reading a dependency's exported macro could carry the same tokens into an export's
  body. So entry 85 (c) refuses the call's form, wherever the macro lives: in the deny-level crates, `include` other
  than directly before `!` and an opening bracket (the seat's lookahead `(?!\s*!\s*[([{])` at check.mjs:683), and in
  every crate an attribute whose text starts with a metavariable (`#[$`, `#![$`), of which the workspace has none
  (grep, Notes). With that, deps' held line is true for both forms.
- **SEC-U4D2-R2-4 (T3 and T5 say the rule reads the token `u32`; it reads `u32` and `i32`, check.mjs:225): fixed in
  my texts now** (Block 3): "reads the type's token (`u32` or `i32`), not the type". It is fixed in this change.
- **RR2-L2 (an empty CARGO_HOME is read as a folder named "", where cargo falls back to ~/.cargo): entry 85 (d).**
  Pre-existing code that FX-1's claim now rests on (r2-rr-cargohome.log). `||` for `??`, a relative value resolved
  against the folder cargo runs in, and one self-test.
- **RR2-L3 with TE2-R2-2 (EA-9's one default for `--on` in `hash compare` is pinned by no test): carry 83, new item
  (d).** Settled by run (r2-te-rr2-l3-compare-arm.log: every recording armed again, 19 passed in both builds, the wasm
  step PASS). No verdict depends on it today: check.mjs always passes `--on`. U4r changes hash.rs (entry 82) and the
  step's compare calls (entry 83 (b)), so the test goes there.
- **RR2-L4 (wording): (a) entry 85 (e); (b) a card follow-up.** (a) check.mjs:1548-1550's "the first non-empty one":
  the Cargo book says "the first one being used", and cargo uses a set but empty CARGO_ENCODED_RUSTFLAGS or RUSTFLAGS
  (the rust seat's read of cargo's source); no result changes under FX-1. (b) Cargo.toml's libm comment cites
  arch/x86/fma.rs:9-27, which is fma's dispatch only (fmaf's is :28-44): at the next edit of that comment.
- **INF2-2 with TE2-R2-1 (run()'s timeout wiring is pinned by no test): entry 84 (c).** Settled by run
  (r2-te-inf2-2-run-timeout.log: 71 of 71 with the timeout dropped). Block 8's ruled proof was "one self-test", which
  FX-7 has; the real-process test belongs with entry 84's group kill, which needs one anyway.
- **INF2-4 (the lane lock's file lies inside the folder M9's clean deletes): M9 amended (Block 5), and a follow-up.**
  It holds by reading: main-run.sh:20-21 neither checks the `exec 9>` nor the flock (`set -u` only), so after a full
  clean the run would go on unlocked while its log says "held exclusively". **The seat's fixes would plant a worse
  trap:** a `mkdir -p` of the lock's folder before cargo has built into it makes .gate/shared-target a folder cargo did
  not create, and by my reading cargo then writes no CACHEDIR.TAG into it; cargo 1.99.1 refuses to clean a folder
  without one (history.md:427), so the one approved method for the shared folder would be lost. So M9's cold run cleans
  only the release profile of both targets, from inside main-run.sh's own exclusive call: the lock's file and cargo's
  CACHEDIR.TAG are never deleted, and the wasm step builds release only (WASM_BUILDS), so the cold figure is the same.
  Follow-up, before U4r's brief: main-run.sh and the lane runner U4r's brief names exit non-zero when the lock's file
  cannot be opened or the flock fails, and no runner creates the shared target's folder by hand.
- **INF2-5 (FX-10's files carry no time, head or command; `--only` keeps other steps' older files): a card
  follow-up,** at the next change to check.mjs's main: a first line `<UTC time> <HEAD> check.mjs <argv>`. A FAIL line
  always names its own run's file, so no verdict is misread today.
- **EA2-1 (check.mjs has 25 new exports, not round 1's 23): the report's wording.** The report's public-items line
  says check.mjs exports 25 new names against <base> (round 1's 23, plus `buildInputProblems` (FX-1) and
  `FAILED_STEP_DIR` (FX-10)), each imported by check.test.mjs, and that `main` gains three injected parameters whose
  defaults keep the CLI's call. T1 is unchanged: no public Rust item and no loader export was added or dropped (the
  arch seat's diff), which is M8's one allowed T1 change, so none is made.
- **EA2-2 (the wasm32 feature ban has three homes, none bound to another by a test): C-U7-2 (T3).** U7 splits the
  loader and so decides where the list lives: one home that check.mjs's READ_BACK row imports (as it imports
  HASH_WASM_EXPORTS), or a test binding the loader's list to that row. check.mjs's flag-name list stays beside FX-1's
  rule as round 1 ruled; it decides no verdict alone.
- **EA2-3 (TickHash's struct doc still says "the recording (or run) it replays"): carry 83 (a).** FX-11's ruled form
  named the field's doc only, which the builder met. U4r's -30 work is the next change to TickHash's use.
- **EA2-4 (C-U7-2 says moduleProblems "reads each export's type from the module's type section"; the JS API's export
  descriptor has only `name` and `kind`): T3 amended (Block 3).** The claim holds (WebAssembly JS API, the
  ModuleExportDescriptor; the arch seat's read; it matches mine). U7's type check reads the module's bytes (the type,
  function and export sections), so it takes the bytes, not a compiled Module.
- **Round 2's process slips, each Low, recorded in T5, none weakening the evidence:** the arch seat wrote one scratch
  file by a shell redirect (into the scratchpad, read by nothing after); the rust seat's control run let cargo create
  /nonexistent-cargo-home (item 6); the test seat's first two TP10 forms were vacuous by its own error (it searched
  `mask`, where lexRust blanks `"C"`), labelled and not counted, and its third form bit.

Block 2 written 2026-11-13.

### Block 3. Item 3: the texts. APPROVE as amended: T2, T3 and T5 whole below; T1 and T4 unchanged.
- **T1** (u4-build-plan.md §4.1's two rows) stays as round 1 wrote it (advisor-u4d2-r1.md, T1): the arch seat found no
  public Rust item and no loader export added or dropped by the fix pass, so M8's one allowed change is not needed.
  **T4** (SERIES.md's row) stays as written there. Both anchors were found once at the head (Notes).
- **T2** (u4s-carries.md §U4r) now carries four entries: 82 as round 1 wrote it; 83 with EA2-3 added to (a) and a new
  (d) (RR2-L3, TE2-R2-2); 84 (INF2-1 with INF2-2 and TE2-R2-1: Block 1); 85 (the deps step's reads: SEC-U4D2-R2-1 to
  -3, RR2-L1, RR2-L2, RR2-L4 (a), TE2-R2-3: Block 2). Same anchor as round 1 (after entry 77, above "## The next change
  to crates/judge").
- **T3** (plan.md's U7 row, C-U7-2): **SEC-U4D2-R2-4's** words ("reads the type's token, `u32` or `i32`, not the
  type"); **EA2-4's** (the type check reads the module's bytes: its type, function and export sections, because the
  JS API's export descriptor gives only a name and a kind); **EA2-2's** (the refused wasm32 features get one home or a
  binding test in the split); and SEC-U4D2-R2-1's limit of the section, in a few words. Still one table line, no "|"
  inside the cell, same anchor.
- **T5** (u4d2-report.md's rulings section): **round 2's verdicts and process events** in their own bullet (two cuts,
  one round; the TP10 plant's removal); **INF2-3's fill**, each timing figure labelled, with the six values written in
  (their logs: fx-check1.log and fx-check2.log, 41.8 and 40.5 s; r2-inf-wasm2.log, 40.4 s; r2-te-check.log, 34.8 s;
  r2-inf-holders.log, 41.3 s with the seat's process sampler; r2-inf-wasm1.log, 73.7 s, after the machine's restart),
  and the bound's words ("about 80 s with a warm page cache; a first run after a restart adds about 30 s"); the cold
  figure's source as M9 now takes it (Block 5); **INF2-1** in its own bullet (carried, with my form's error named);
  round 2's process slips; the carries added (82 to 85); and **SEC-U4D2-R2-4's** point: the last bullet names every
  open security finding (SEC-U4D2-3, and SEC-U4D2-R2-1 to -3, each Low and carried), so "Every other security finding
  of the review is fixed in this change" is true. Two placeholders stay for the main session to fill from the named
  source: the merged tree's wasm seconds with M5's label (M5's log) and carry 66's values (M5's h7 (ii) run).
- **Three corrections to round 1's T5, mine:** (1) its "<its seconds, from the card>" for the cold figure could not be
  filled: T5 goes into the merge commit (M8), and the cold run comes after the push (M9). The sentence now says where
  the figure will be recorded (the card and U4's report, before U4r's brief), with no placeholder. (2) "38 MB, removed
  by `cargo clean --target-dir`" would be untrue at the commit, for the same reason: it now says the removal follows
  the merge. (3) "cargo uses only the first non-empty one" is RR2-L4 (a)'s imprecision in my own words: it now says
  "cargo uses only one of them", which is exact.
- **What else changed against round 1's T5** (a word diff of the two code blocks, read): the heading names this file;
  the round-1 bullet ends "then round 2 on what changed."; new bullets for round 2, for the step's time (rewritten for
  INF2-3) and for INF2-1; one added sentence each in the build-flag, third-exception and TE-M3 bullets, and round 2's
  slips added to the process-slips bullet; the carries added are 82 to 85; the open-security bullet is rewritten
  (SEC-U4D2-R2-4). Every other word of round 1's T5 is kept.
- The plant runs' seconds (fx-2-tf.log's 26.4 s FAIL; r2-te-rr2-l3-compare-arm.log's 54.9 s) are not timing figures
  and stay out of T5.

Block 3 written 2026-11-13.

### Block 4. Item 4: the process events. APPROVE: round 2 is one round, and the TP10 plant's removal was right.
- **One round.** A round is "one review of one head by all the change-set's reviewers" (CLAUDE.md, the review budget),
  and "a round run in batches of at most 3 agents ... is one round" (advisor.md). The head never moved: every seat
  reviewed 51520b4 plus cb27931's snapshot, and the copy's 18 files equalled fx-hashes-end.sha256 at each resume (the
  card: 18 of 18 at 00:55; at 08:25, 17 and check.mjs, which was the plant, then 18 after its removal). The rust seat's
  result came from the run's cache, not a re-run; the security and infra seats continued after their notes' last lines;
  batch 2's two seats continued after theirs. No seat reviewed a different head, and no finding was re-reviewed. This
  is round 1's 20:52 ruling applied again. The restart's one effect on the evidence is a timing figure: the infra seat's
  first run came 11 minutes after the machine restarted (73.7 s, page cache cold), which T5 labels (INF2-3).
- **The TP10 removal.** The cut left the test seat's plant in check.mjs (two lines in hashWasmProblems), in the copy
  that later seats and the counted run use. Removing it was the only way to put the reviewed bytes back before any other
  run, and the main session did the least act that does it: an Edit back to the start bytes, checked three ways
  (check.mjs's sha256 343c59c5… equal to r2-hashes-start's, `sha256sum -c` 18 of 18, `diff -rq` against .gate/u4d2-r2
  equal), recorded in a labelled block in the seat's notes, not as the seat's own line, with the cut run's log marked as
  no result. The seat was stopped, so no two agents edited one file at once. The seat then re-planted TP10 in its own
  form (r2-te-tp10c: tests 38 and 54 fail by name, 69 of 71), so FX-3's proof rests on the seat's run, not on the cut
  one. A plant in place at a cut is exactly what B6's "plants by Edit in place" exposes; the card's SPEND-LIMIT record
  and this removal are the right handling, and the next review workflow's briefs keep them.

Block 4 written 2026-11-13.

### Block 5. Item 5: the merge. APPROVE on round 1's M1 to M9 (advisor-u4d2-r1.md, Block 9), with these amendments.
Round 2's outcome is APPROVE WITH FINDINGS from all five seats, one Medium carried (Block 1) and Lows only otherwise, so
Block 8's rule there lets the merge go ahead. Each condition, as it stands now:
- **M1 stands.** Checked now: the copy's HEAD 51520b43391a…; 19 status lines (the 18 files and the record); `sha256sum
  -c` of r2-hashes-start.sha256 from the copy's root 18 OK; that file equals fx-hashes-end.sha256 (`cmp`). The main
  session repeats it right before the snapshot, as written.
- **M2 stands.** Checked now at ff6331d: spikes/ changed since 51520b4 only by c88eb31 and 1b4a3c8; the lane paths and
  crates/sim unchanged; spikes/ clean in the main folder. A13's CI (gate and smoke) on its pushed head is still the main
  session's to read.
- **M3 stands,** with its reference named: the committed patch is cb27931's wip/u4d2.patch (sha256 9e71cda07d4a…, blob
  ba0f25bd…, unchanged since), and worktree_patch.py's new cut must `cmp` equal to it.
- **M4 stands** (r2-hashes-start.sha256, 18 lines, from the main folder's root).
- **M5 amended:**
  - **the counts:** measured 491, bites 561, self 71 of 71, 11 steps, shown as sums: 473 + 17 + 1 = 491, 541 + 19 + 1 =
    561, 52 + 11 + 8 = 71 (c88eb31's counts, round 1's deltas, the fix pass's), each fix-pass test named from
    u4d2-build.md's list (FX-8's unit test in each build; the eight self-tests). The brief's arithmetic is right. marks
    81, redact 0 findings, as written;
  - **the deps line** also holds the fix pass's clauses: "no rustflags source holds a flag ... and neither the
    environment nor a cargo config sets the build target", and "neither npt-tcpinfo nor npt-hash-wasm has a build
    script or names `include` but as a macro call" (as r2-te-check.log's deps line has them);
  - **the wasm line** as written, plus: no call reported stopped by the 300 s limit, and the step's seconds labelled as
    T5 labels them (warm for dependencies; the machine's uptime in the notes, since a first run after a restart reads
    slower: INF2-3);
  - **INF2-1's interim rule:** if any call of the wasm step reports the 300 s stop, that is a wasm FAIL (which stops the
    merge and comes to me, as written); before anything else runs, the main session lists any npt-lab or loader process
    left (`pgrep -a npt-lab`; `pgrep -af loader.mjs`), records them in the notes, and ends them.
- **M6 and M7 stand.**
- **M8 amended (the texts):** T1 and T4 are copied from advisor-u4d2-r1.md, unchanged; T2, T3 and T5 from this file
  (below), each by script under M8's rules (each old text found once, each new text found in place after, no changed
  prose line over 120 characters; T3 is a table line). u4s-carries.md gains entries 82 to 85. The report's
  public-items line takes EA2-1's count (25 new check.mjs exports; `main`'s three injected parameters), and its
  follow-ups list is round 1's plus round 2's (M9). The commit's spikes/ part is still the reviewed bytes plus T4, and
  nothing else.
- **M9 amended:**
  - the saver restarted;
  - the card: U4d2 merged at <sha>; the base counts for U4r (as M5 gives them); carries 82 to 85 and C-U7-2; the open
    security findings for PR #40's report, SEC-U4D2-3 (C-U7-2) and SEC-U4D2-R2-1 to -3 (entry 85); INF2-1 (Medium,
    entry 84); TE2-5 (c), TE-M3's planned change; the follow-ups: round 1's (RR-L4 with EA-1, INF-2, INF-3's CPU
    record, INF-7, §7.1 item 6's text) and round 2's (RR2-L4 (b), the libm comment's fmaf cite; INF2-4's runner half,
    before U4r's brief; INF2-5, the failed-step file's first line); and Block 6's probe lesson;
  - **the copies,** once CI is green, by the approved method only (each copy's leftovers to a local branch, then a plain
    `git worktree remove`, never `--force`, after CHECKPOINTS.md §10's checks), or put on the card's "Housekeeping
    later" list in that form as U4e's and U4d-F's were: `.claude/worktrees/u4d2-build`, `.gate/u4d2-r1`,
    **`.gate/u4d2-r2` (it joins: round 2's read copy, a git worktree at 51520b4 with the reviewed patch applied, which
    holds nothing the merge does not)**, and `.gate/u4d2-prefix`; `.gate/u4d2-rr-target` by `cargo clean
    --target-dir` (it holds its CACHEDIR.TAG);
  - **the cold run (amended, INF2-4):** with wave 2 over and no lane running, one main-run.sh call, its log under
    .gate/u4d2-merge/, holding these commands in order (`<shared>` is /home/user/Shamble-Frontlines/.gate/shared-target;
    main-run.sh stops at the first that fails): `cat /proc/uptime` (the machine's uptime, into the log); `cargo clean
    --target-dir <shared> --release`; `cargo clean --target-dir <shared> --release --target wasm32-unknown-unknown`;
    `test -f <shared>/.lane.lock`, `test -f <shared>/CACHEDIR.TAG`, `test ! -e <shared>/release/deps` and `test ! -e
    <shared>/wasm32-unknown-unknown/release/deps` (the clean took what it should and kept what it must); then `node
    check.mjs --only wasm`. So the clean runs under the exclusive lock it would otherwise delete, the lock's file and
    cargo's CACHEDIR.TAG stay, and every release artifact the step builds (all four builds are `--release`:
    WASM_BUILDS) is rebuilt from nothing, the toolchain and the registry cache as they are. The step's seconds and
    split are INF-1's cold figure, with that uptime, recorded on the card and in U4's report before U4r's brief. If any
    command before the step fails, the run stops there and it comes to me. No full clean of the shared folder until
    INF2-4's runner follow-up is in, and never a hand-made shared-target folder (Block 2). Over 300 s, it comes to me
    as U4r's det-u4.mjs question; a stopped call, as at M5.

Block 5 written 2026-11-13.

### Block 6. Item 6: /nonexistent-cargo-home. Nothing more is needed: leave it, record it, and keep the lesson.
- What it is (read now): a 12 KB folder at the filesystem's root, holding an empty `.package-cache` and
  `registry/CACHEDIR.TAG`, made by cargo at 00:46 in the rust seat's control run (`CARGO_HOME=/nonexistent-cargo-home
  cargo tree`, exit 101: r2-rr-cargohome.log). It is outside the repository, so nothing commits it; it holds no secret
  and no build; nothing reads it.
- Mustafa's rule covers it: LEFTOVER-DELETE (history.md:404, his words: "leave them. ... a one-off delete past the deny
  list isn't a habit we start; if it's ever needed, it becomes a narrow kit rule instead"). No approved method removes
  it: `cargo clean --target-dir` needs a CACHEDIR.TAG at the folder's own root, and it is no git worktree. A kit rule
  for 12 KB is not needed. So the main session was right not to remove it, and no session or agent removes it.
- Needed: one line on the card's "Housekeeping later" list (its path, size and contents, left under LEFTOVER-DELETE),
  and one line in T5's process slips (done). **The lesson, for every later brief that probes cargo:** never run cargo
  with CARGO_HOME or CARGO_TARGET_DIR at a path that does not exist (cargo creates it, outside every approved removal
  method); to test check.mjs's reading of such a value, pass it to check.mjs's function only, as the security seat did
  (sec-r2-deps-home.mjs: the probe value for the reader, the real environment for cargo and rustc). The path also no
  longer means "missing" on this machine, so no later control may use it as one.
- For Mustafa: information only (For Mustafa, below); no decision.

Block 6 written 2026-11-13.

## The texts (each copied by script from its code block; never retyped)
T1 and T4: advisor-u4d2-r1.md's code blocks, unchanged (Block 3). T2, T3 and T5 below replace round 1's.

### T2. u4s-carries.md §U4r, after entry 77 (its last), above "## The next change to crates/judge (whoever makes it; its first step)" (the merge commit)
```
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5; with EA2-3, RR2-L3 and TE2-R2-2: the
    advisor's U4d2 round-2 ruling, Block 2):** "(a) For -30's pre-check, U4r gives `HashCompare::of` two lists that
    name the same recording on both legs and line up tick for tick (it compares by position, names included), and says
    where its printed comparison line comes from (`compare_text` is private to npt-lab's binary); TickHash's struct doc
    then says what its field doc says, 'the recording both legs replay' (today it still says 'the recording (or run) it
    replays'). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also requires
    exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines alone,
    which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that the bite
    causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg is the
    committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's first
    recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside the
    hash-map pair's equal prefix). (d) One npt-lab bin test runs `hash compare` with a bite and no `--on` over two
    recordings and asserts that only the first listed is armed (EA-9's one default, the legs'; today no test pins it:
    compare arming every recording again leaves every test and the step green, r2-te-rr2-l3-compare-arm.log)."
84. **INF2-1 with INF2-2 and TE2-R2-1, the wasm step's time bound and a stopped call's processes
    (u4d2-r2-review-infra.md, INF2-1 and INF2-2; u4d2-r2-review-test.md, TE2-R2-1; the advisor's U4d2 round-2 ruling,
    Block 1):** "At U4r's change to check.mjs's `wasm` step (entries 82 and 83), or in ops/runs/det-u4.mjs if U4r's
    Evidence decision moves the step's runs there: (a) the step's calls other than its cargo builds share one deadline,
    WASM_CALL_LIMIT_MS (300 s) from the first of them: each is given what is left, and once one is stopped the rest are
    not started, each named as skipped (today each call has its own 300 s, so a hang in npt-sim's replay makes 12 calls
    each wait out 300 s, 60 min under the exclusive lane lock: r2-inf-hang.log). (b) A call stopped by the deadline
    takes every process it started with it (today spawnSync's timeout signals only the direct child, so `npt-lab
    hash`'s two leg processes can keep running after their parent is stopped, unseen by the runners' idle wait, which
    looks for cargo and rustc only), by a documented means (its own process group, killed whole, or another) or by one
    whose effect (c)'s second test proves. (c) Two self-tests: the shared deadline, with a fake runner and clock (a
    first call that takes the whole limit; the next calls skipped and named); and run() itself on real processes (a
    child that starts a grandchild and sleeps, with a limit of about 200 ms: run() returns `timedOut`, and neither
    process is left), which also pins run()'s timeout wiring (today dropping it leaves 71 of 71 green:
    r2-te-inf2-2-run-timeout.log)."
85. **The deps step's unread inputs and two overclaims (u4d2-r2-review-security.md, SEC-U4D2-R2-1 to -3;
    u4d2-r2-review-rust.md, RR2-L1, RR2-L2 and RR2-L4 (a); u4d2-r2-review-test.md, TE2-R2-3; the advisor's U4d2 round-2
    ruling, Block 2):** "At U4r's change to check.mjs (entries 82 to 84), the deps step, each refusal naming its source
    and value, with one self-test case each, and the container's environment and the spike's own config still passing:
    (a) refuses a linker set for any target, by an environment variable whose name ends in _LINKER or by a `linker` key
    in any table of a cargo config read, as COMPILER_ENV and COMPILER_KEYS refuse a compiler (today
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER passes: r2-rr-superset.log run 3); and DEPS_CHECKED's limits and
    crates/hash-wasm/loader.mjs's header (the one edit U4r makes in crates/hash-wasm) say what the target_features read
    gives: a feature enabled at compile time shows in the section, which the loader refuses, but the section is the
    linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can rewrite it (built:
    sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check (today both say a flag from a
    route check.mjs does not read is refused too). (b) It refuses any `profile` key in a cargo config read and any
    environment variable whose name starts with CARGO_PROFILE_ (today a config's `[profile.release.package.npt-sim]
    overflow-checks = false` passes and drops npt-sim's `-C overflow-checks=on`, which plan §4.5 asks for in every
    profile: r2-sec-fx1-config.log, r2-sec-routes.log). (c) In npt-tcpinfo and npt-hash-wasm it refuses `include` other
    than directly before `!` and an opening bracket, and in every crate an attribute whose text starts with a
    metavariable (`#[$` or `#![$`; none today), so that neither `m!(include!)` nor a #[path] built from a `meta`
    fragment brings an unread file under a listed export's allow (today both pass, and deps' held line says every
    include! literal names a .rs file: r2-te-sec-r2-3-macro-include.log, r2-sec-pathmeta.log). (d) It reads an empty
    CARGO_HOME as unset and a relative one against the folder cargo runs in, as cargo does (today an empty one hides
    ~/.cargo/config.toml from FX-1's refusal and from the read-back: r2-rr-cargohome.log). (e) Its text beside
    buildInputProblems says which source cargo uses: CARGO_ENCODED_RUSTFLAGS if set, even empty; else RUSTFLAGS if set,
    even empty; else the matching target tables' flags, if any; else build.rustflags (today 'the first non-empty
    one')."
```

### T3. plan.md's U7 row: C-U7-2 (the merge commit)
The text `or states why not |` (found once, at the end of the U7 row's last cell, plan.md:1247 at the head) becomes the
text below, on the row's one line (no "|" added inside the cell):
```
or states why not. **C-U7-2** (the advisor's U4d2 rulings: round 1, 2026-11-12, EA-2, EA-4, INF-6, SEC-U4D2-3 and TE-L2; round 2, 2026-11-13, EA2-2, EA2-4, SEC-U4D2-R2-1 and SEC-U4D2-R2-4): U7's Chromium hash check loads U4d2's npt-hash-wasm.wasm as it is (no imports; the seven integer exports and `memory`), and splits crates/hash-wasm/loader.mjs into a browser-safe module (the export list, its one home kept; the status numbers, with one home or a test binding them to lib.rs's constants; the refused wasm32 features, with one home that check.mjs's read-back row also uses or a test binding the two lists; `moduleProblems` with its import, kind and `target_features` refusals; the replay loop over a chunk source) beside the Node script; the module check also reads each export's type from the module's bytes (the type, function and export sections: the JS API's export descriptor gives only a name and a kind, so this check takes the bytes, not a compiled module) and refuses any but i32 (U4d2's source scan reads the type's token, `u32` or `i32`, not the type), and it runs on the wasm-opt output C-U7-1 checks, so U7 keeps that output's `target_features` section or states how it checks the features (the section is the linker's record, which link arguments can rewrite); U7's brief names how its Chromium leg and wasm-opt get pinned tools (no CI job runs spikes/; a CI home is .github/ work, risk tier 2, Mustafa's go) |
```

### T5. u4d2-report.md's section on the review's rulings (new file, beside u4e-report.md)
Copied by script as it is. Each `<…>` is a value the main session fills from the named source (M5's log, the fix
pass's record, the merged tree's h7 (ii) run), and the notes name the log and line of each fill; no other word
changes. The cold figure is not in it: it comes after the merge commit (M9) and goes on the card and in U4's report.
```
## What U4d2's review settled (the advisor's U4d2 rulings: round 1, advisor-u4d2-r1.md; round 2, advisor-u4d2-r2.md)
- **Round 1** (five seats on one head, 51520b4 plus the snapshot at e2f2a5b; the spend limit cut the second batch at
  20:52 UTC, after the architect's and the test seat's plans and with no plant in place, and both resumed at 22:53 on
  the same bytes, so one round): rust-reviewer, security-auditor and test-engineer REVISE, infra-engineer and
  engine-architect APPROVE WITH FINDINGS; no Critical or High; six Mediums (RR-M2 = TE-M1; RR-M1 = SEC-U4D2-2 = TE-M2;
  SEC-U4D2-1; SEC-U4D2-7; INF-1; TE-M3). One fix pass (FX-1 to FX-12), then round 2 on what changed.
- **Round 2** (the same five seats on the fix pass's head, 51520b4 plus the snapshot at cb27931): all five APPROVE WITH
  FINDINGS; no Critical or High; one Medium (INF2-1, below), carried to U4r; the rest Low, each carried to U4r or U7,
  fixed in these texts, or a follow-up; none needed a push. It was cut twice, by a container restart at about 00:52
  UTC (resumed about 00:55) and by the spend limit at 01:22 UTC (resumed 08:25), and resumed each time from its
  journal on the same bytes, so it is one round. The second cut left the test seat's TP10 plant in check.mjs: the main
  session removed it by Edit (the file back to its start sha256, all 18 files equal to the start list, the copy equal
  to the read copy by `diff -rq`), and the seat re-planted TP10 in its own form, which failed two tests by name.
- **The build-flag guard was fail-open for one case, now closed (RR-M2; Medium; fixed).** The read-back gave rustc every
  rustflags source at once, but cargo uses only one of them, so a negation in a source cargo skips hid an enabling flag
  in the one it used: the deps step passed while the module built so listed +simd128 and +relaxed-simd. The advisor's
  U4D2-Q4 ruling had called that superset fail-closed; it was not. Since the fix the deps step refuses every rustflags
  source that holds a flag (the spike's builds use none), and a set CARGO_BUILD_TARGET or `build.target`, so the
  read-back reads exactly what cargo gives rustc; and the loader refuses a module whose `target_features` section lists
  simd128 or relaxed-simd, or that has none, which also covers a safe `#[target_feature]` function reaching the Wasm
  leg with no flag and no `unsafe` (SEC-U4D2-7; Medium; fixed). The section is the linker's record, so link arguments
  can rewrite it (SEC-U4D2-R2-1, below).
- **The third exception's guards (RR-M1, SEC-U4D2-1; Medium; fixed).** An `unsafe` block inside a listed export's body
  is accepted by rustc under the export's own allow, so only the deps scan refuses it: that case now has its self-test
  and was shown live (build exit 0, deps FAIL at its line). The loader's empty import object inherited
  Object.prototype, so a module importing `constructor.constructor` instantiated; it now refuses every import by name
  and instantiates with a null-prototype object. The kind refusal has its two self-tests (SEC-U4D2-8); a build script
  and `include` used as a path are refused in the deny-level exception crates (SEC-U4D2-4). Two macro forms still pass
  that scan (SEC-U4D2-R2-3, below).
- **The step's time (INF-1 and INF2-3; Medium and Low; report text).** Each figure is labelled. Warm for dependencies,
  the workspace rebuilt by the runner's touch loop: 41.8 and 40.5 s (the fix pass's two counted runs), 40.4 s (round
  2's infra seat), 41.3 s (the same seat, with a process sampler running), 34.8 s (round 2's test seat's counted run)
  and <the merged tree's seconds, with M5's label> (the merged tree's counted run), against a 5-minute budget. Round
  2's first run, 73.7 s, was the first run after a machine restart (page cache cold, no dependency rebuilt). No cold
  run was taken before the merge: the shared target always held other lanes' release dependencies, and Mustafa's
  one-folder rule gives a copy no empty target. The first release builds in the lane took 16.04 s (npt-lab) and 3.57 s
  (npt-hash-wasm, wasm32), and the infra seat's bound for a cold step is about 80 s with a warm page cache; a first run
  after a restart adds about 30 s. The cold figure comes from one run after this merge, once the shared target's
  release builds for both targets are cleaned between waves (W1-TARGET item 3's method, narrowed so that the lane
  lock's file stays: INF2-4), and it is recorded on the card and in U4's report before U4r's brief.
- **A stopped call's bound and its processes (INF2-1; Medium; carried to U4r, entry 84).** The wasm step gives each of
  its calls but the four cargo builds its own 300 s (FX-7), so a hang in npt-sim's replay could hold the exclusive lane
  lock for 12 x 300 s, and a stopped `npt-lab hash` call can leave its two leg processes running. The advisor's
  round-1 form had called that per-call limit "the step's whole budget"; it was not. U4r's change to the step gives
  these calls one shared deadline and makes a stopped call take its processes with it, each with a self-test. No call
  has come near the limit: the largest since the fix pass took 4.3 s.
- **An npt-net test that fails under load (TE-M3; Medium; not U4d2's).** `l2::delay::tests::a_shutdown_s_never_
  completes_ends_at_its_deadline_without_spinning` failed once in the test seat's counted run (bites build) and passed
  on the immediate re-run; it failed the same way in U3c. It reads no U4d2 file and is in no crate U4d2 changes, so it
  did not stop the merge (B2.3's rule); its cause is a planned change of the main session's before U4r's merge (TE2-5
  (c)). Since FX-10, a failed step's whole output is kept in a file, so the next failure keeps its panic. It passed in
  the three counted runs since (the fix pass's two and round 2's one).
- **Process slips, none weakening the evidence.** The builder judged an unnamed check.test.mjs failure inside its step
  instead of stopping (its own new rule repeated a sentence the test pins once; the test is unchanged and still bites:
  TP9); three early `npt-lab hash` runs took the shared lock where `-x` was asked (none a reported result; each value
  re-checked under `-x`); the rust seat built one module in a scratch target off the shared one (38 MB; its removal by
  `cargo clean --target-dir` follows the merge) to keep a SIMD-built module out of it. The builder's brief cited four
  plan spans, and U4r's two lines, at the plan's earlier stamp; the build covered §5 U4d2 whole. In round 2, the
  architect seat wrote one scratch file by a shell redirect (read by nothing after); the rust seat's control run let
  cargo create /nonexistent-cargo-home (12 KB, outside the repository, left in place under Mustafa's LEFTOVER-DELETE);
  and the test seat's first two TP10 forms were vacuous by its own error, labelled and not counted.
- **Carries.** Met: 29 (Cargo.toml's libm comment; sqrtf is `sqrtss`, sqrt `sqrtsd`, a correction to the carry's own
  words), 41, 52 (only recordings by U4d-F's generator, adb1bab), 65 (the failing tests named; their panics kept since
  FX-10), 66 (<the three v1 datagram hashes and their commands>); 42 does not apply (entry 34 was met by U4a). Added:
  82, 83, 84 and 85 (U4r), C-U7-2 (U7).
- **Open security findings (each Low and carried; none fixed in this change).** SEC-U4D2-3: the integer-signature rule
  reads the type's token (`u32` or `i32`), not the type; the limit is written beside the rule, and the type read is
  U7's (C-U7-2). From round 2, each carried to U4r (entry 85): SEC-U4D2-R2-1, the target_features read is the linker's
  record, which a RUSTC_WRAPPER's link arguments can rewrite, no linker setting is read, and two comments claim more
  for the read than that; SEC-U4D2-R2-2, a cargo config's per-package profile can turn npt-sim's overflow checks off
  with no rustflag, unread; SEC-U4D2-R2-3, `include!` handed to a macro as tokens, or a #[path] built from a macro
  fragment, brings an unread file under a listed export's allow in npt-hash-wasm. Every other security finding of the
  review is fixed in this change, SEC-U4D2-R2-4's wording in these texts included.
```

## Before the merge (in this order)
Nothing from round 2 needs a push or a code change: the copy's 18 files are the reviewed bytes as they stand.
1. **M1 to M4** as round 1 wrote them (Block 5: each still holds today): the copy is the one reviewed (HEAD 51520b4,
   the 18 files and the record, r2-hashes-start 18 OK and equal to fx-hashes-end); the base still fits (only c88eb31
   and 1b4a3c8 since 51520b4; A13's CI green, to read); the snapshot equal by `cmp` to cb27931's wip/u4d2.patch
   (9e71cda0…), then applied; the merged bytes 18 of 18 against r2-hashes-start.
2. **M5 as amended:** T4 by script first; the six builds; check.mjs counted in the background (limit at least
   1,500,000 ms): all 11 steps, **491 / 561 / 71 of 71** shown as 473 + 17 + 1, 541 + 19 + 1 and 52 + 11 + 8 with each
   fix-pass test named, marks 81, redact 0; the deps line with the fix pass's clauses; the wasm line as written, no
   call stopped, its seconds labelled with the machine's uptime; h7 (ii) by name. A stopped call is a wasm FAIL: list,
   record and end any npt-lab or loader process left, then it comes to me.
3. **M6 and M7** as written.
4. **M8 as amended:** one commit; T1 and T4 from advisor-u4d2-r1.md, T2, T3 and T5 from this file, each by script;
   u4s-carries.md gains 82 to 85; the report's public-items line with EA2-1's count (25 check.mjs exports; `main`'s
   three injected parameters) and its follow-ups list (round 1's and round 2's); `git rm` of wip/u4d2.patch.
5. **M9 after the push, as amended:** the saver; the card (carries 82 to 85 and C-U7-2; the four open security Lows;
   INF2-1; TE2-5 (c); round 1's and round 2's follow-ups; Block 6's lesson and the leftover's line); the copies,
   `.gate/u4d2-r2` among them, by the approved method or on "Housekeeping later"; `.gate/u4d2-rr-target` by `cargo
   clean --target-dir`; then the cold run in one main-run.sh call, release-only clean on both targets inside it, with
   its four `test` guards and the uptime, before U4r's brief. INF2-4's runner follow-up (fail closed on the lock, no
   hand-made target folder) before U4r's brief too.

## Verdict, per item of the brief
| Item | Verdict | In one line |
|---|---|---|
| 1. INF2-1 (Medium) | APPROVE as a finding, not before the merge; carry 84 (U4r) | one shared deadline, a stopped call takes its processes, two self-tests; my round-1 "whole budget" corrected; interim rule at M5 and M9 |
| 2. SEC-U4D2-R2-1 with RR2-L1 | carry 85 (a) (U4r); T5 states the limit now | refuse a linker setting; both comments say the section is the linker's record; my round-1 words corrected |
| 2. SEC-U4D2-R2-2 | carry 85 (b) | refuse config `profile` keys and CARGO_PROFILE_* |
| 2. SEC-U4D2-R2-3 with TE2-R2-3 | carry 85 (c) | `include` only directly before `!(`; no metavariable attribute in any crate |
| 2. SEC-U4D2-R2-4 | fixed in T3 and T5 | "the type's token (`u32` or `i32`), not the type" |
| 2. RR2-L2 | carry 85 (d) | an empty CARGO_HOME read as unset; a relative one as cargo reads it |
| 2. RR2-L3 with TE2-R2-2 | carry 83 (d) | one bin test pins compare's one default |
| 2. RR2-L4 | (a) carry 85 (e); (b) card follow-up | cargo's source order in words; the fmaf cite |
| 2. INF2-2 with TE2-R2-1 | carry 84 (c) | run()'s real-process test |
| 2. INF2-4 | M9 amended; runner follow-up before U4r's brief | release-only clean inside main-run.sh's call; never a hand-made target folder |
| 2. INF2-5 | card follow-up | a first line with time, HEAD and argv |
| 2. EA2-1 | the report's wording | 25 new check.mjs exports; T1 unchanged |
| 2. EA2-2 | C-U7-2 (T3) | the refused features get one home or a binding test |
| 2. EA2-3 | carry 83 (a) | TickHash's struct doc |
| 2. EA2-4 | T3 amended | the type check reads the module's bytes |
| 3. T3, T5 (and T2) | APPROVE as amended (code blocks above) | T1 and T4 unchanged; three corrections to round 1's T5, mine |
| 4. the two cuts; TP10's removal | APPROVE: one round; the removal right | same head throughout; restored bytes checked three ways |
| 5. the merge | APPROVE on M1 to M9 as amended | 491 / 561 / 71 of 71, 11 steps; `.gate/u4d2-r2` joins M9's copies |
| 6. /nonexistent-cargo-home | nothing more: leave it (LEFTOVER-DELETE) | a card line and a probe lesson for later briefs |

Risk tier 1 (spike code and the spike's own check on PR #40's branch), as in round 1. Rounds: two, the second on what
changed; no Critical or High open, so no third round. PR #40 stays needs-mustafa; the four open security Lows keep it
so on their own as well (advisor.md, PR review step 4).

## Decisions needed
None. INF2-1's route is the Medium-to-me case round 1's Block 8 set, and a carry is one of its two allowed routes. The
other routes, the narrowed clean at M9 (still `cargo clean --target-dir`, deleting less) and the corrections of my own
round-1 words are technical choices inside the documented rules, which I decide. No word or decision of Mustafa's
changes, and nothing a player sees, hears or feels is touched.

## For Mustafa
Nothing to do now. For his information, at PR #40's review: U4d2's report will list four open security findings, all
Low and each carried (SEC-U4D2-3 to U7; SEC-U4D2-R2-1 to -3 to U4r), and one Medium carried to U4r (INF2-1: a hang
could hold the build lock for up to an hour and leave processes running; none has come near it). One reviewer probe
left a 12 KB empty folder outside the repository (/nonexistent-cargo-home); under his LEFTOVER-DELETE ("leave them")
it stays, and no session removes it.

## What I checked, and what I did not
**Checked myself (reading, git and sha256; no build, run, plant or apply):** my role file from main; the brief and my
round-1 ruling, whole; the five seats' notes and the fix pass's record, whole; the two hash lists (`cmp`) and both
copies against them (18 OK each); the copy's HEAD and status; the patch's sha256 and blob since cb27931; M2's spikes/
history at the head; the five texts' anchors; the logs at the lines the findings and T5's figures cite (the six timing
runs, the hang count, the counted run's counts and lines, TP10c, the macro-include plant, the run()-timeout plant, the
rust seat's CARGO_HOME runs); the code each route rests on (run(), `call`, WASM_CALL_LIMIT_MS, WASM_BUILDS, the deps
limits text, the loader's header, COMPILER_ENV, cargoConfigFiles, the include rules); a grep of the workspace for
metavariable attributes and macro definitions; both runners' lock lines; the shared target's root; the copies' kinds
(`git worktree list`); the leftover's contents; LEFTOVER-DELETE's words. A script read both rulings' code blocks: entry
82 and carry 83's (b) and (c) equal round 1's word for word, and no changed prose line in T2 or T5 exceeds 120
characters.

**Not checked:**
- I built, ran and planted nothing. The findings' facts rest on the seats' runs, which I read at their lines.
- That cargo writes no CACHEDIR.TAG into a target folder that already exists (my reading of cargo-util's
  directory creation, from memory; its source is not on this machine). Block 2 and M9 are written so that nothing
  depends on it: no folder is made by hand, and the release-only clean keeps the root.
- That `cargo clean --release` (with and without `--target wasm32-unknown-unknown`) removes exactly that profile's
  folders and keeps the target's root (the Cargo book's clean page, from memory). M9's four `test` guards check it in
  the run itself and stop it if not.
- That `detached` is undocumented for spawnSync in Node 22 (the infra seat's read, medium confidence); entry 84 does
  not depend on it.
- That a dependency's exported macro could carry `include!` tokens into an export's body under its allow (my reading;
  not built). Entry 85 (c) refuses the call's form, so it holds either way.
- A13's CI run (M2 asks the main session to read it).
