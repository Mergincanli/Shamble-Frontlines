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
- The leftover: /nonexistent-cargo-home, 12 KB: an empty .package-cache and registry/CACHEDIR.TAG (00:46). LEFTOVER-DELETE
  read at history.md:404-409, verbatim "leave them. ... it becomes a narrow kit rule instead."

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
