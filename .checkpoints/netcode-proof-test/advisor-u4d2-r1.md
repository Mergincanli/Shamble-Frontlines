# The advisor's ruling on U4d2's round 1 (2026-11-12, from advisor-u4d2-r1-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md`
(read at origin/main). It rules; it edits no code and runs nothing that builds.

## Plan
1. Read the five seats' notes whole (rust, security, infra, arch, test), then the builder's record (claims only).
2. Read my own rulings that bind the form: advisor-u4d2-brief.md (L1 to L5, the conditions), advisor-u4e-r1.md and
   advisor-u4e-r2.md (M1 to M9's form).
3. Read the rules cited by place: CLAUDE.md's review budget (in context), u4-build-plan.md §7.1 items 1 and 7,
   u4s-carries.md entry 52 (the 9.5 floor), §5 U4d2, B2.3 / TE2-5 in u4d2-brief.md, carry 65.
4. Spot-check the load-bearing claims against the copy (`.gate/u4d2-r1`, the worktree's diff) and the seats' logs
   (`.gate/u4d2/r1-*`): RR-M2's run, SEC-U4D2-1's run, SEC-U4D2-7's run, TE-M3's logs, the counts.
5. Rule, one block per item (1 to 9 of the brief), each written to this file as it is finished.
6. End with the two lists (before the merge; for Mustafa) and the summary for the caller.

## Notes (one line per check, as done)
- Read whole: advisor.md (origin/main, 334 lines); the five seats' notes (rust 124, security 172, infra 150, arch 112,
  test 266 lines); the builder's record u4d2-build.md (256 lines, its claims).
- Read whole: my brief check advisor-u4d2-brief.md (756 lines: A1-A15, Q1-Q12, L1-L5; Q4 (a) chose the superset,
  "fail-closed"; A15 found the superset's first cost; block 7 set U4d2's review in batches).
- Read the form: advisor-u4e-r1.md R8 (fix pass, its builder, round 2's seats, the outcome rule) and R9 (M1-M9);
  advisor-u4e-r2.md Q3 (M1-M9 as amended), Q4 (background run, >= 1,500,000 ms), "Before the merge".
- The rules by place: u4-build-plan.md (now 52dd3186…) §5 U4d2 :666-712, Lanes :850-888, §6 :890-907, §7.1 :909-921;
  Mustafa's words behind §7.1 (history.md:411-412, 2026-11-09: "point all copies at one shared Cargo target folder
  (CARGO_TARGET_DIR), so each unit doesn't leave a 4–6 GB cache behind"); W1-TARGET item 3 (advisor-u4-wave1.md:576-580:
  "Between waves, with no lane running, the main session may empty the shared folder by `cargo clean --target-dir
  .gate/shared-target` ... §7.1 item 7 ... binds the copies, not the main session"); B2.3 (advisor-u4j-r2.md:146-171).
- TE-M3's test read: delay_tests.rs:2551-2603, `#[tokio::test(flavor = "multi_thread", worker_threads = 2)]`, counted
  asserts (`polls_after_deadline == 1`, `polls <= 3`, `s_calls <= polls`) after a 300 ms wait; U3c's failure
  (u3c-fix2-c2b.md:74-76: "panicked at delay_tests.rs:2002 ... once, under C1b's parallel load"). Which assert fails is
  unknown both times (no panic text kept).
- The copy, checked now: `.claude/worktrees/u4d2-build` HEAD 51520b43391a…; `git status` 9 M + 8 new + the record; the
  17 files' sha256 equal the test seat's step-1 list (first 16 hex each compared). wip/u4d2.patch a526ef8e… (5,366
  lines); the packet 9b670276… (as the card names it); the brief 146acf18… (as built from).
- Seats' logs read at the lines the findings cite: r1-rr-superset.log:145-158 (the negation env PASS twice, the control
  FAIL); r1-rr-superset-build.log:3-14 (`--target-dir .gate/u4d2-rr-target`, the module lists +simd128 and
  +relaxed-simd among 17 features); r1-sec-imports.log:5-7 (three Object.prototype imports INSTANTIATED, moduleProblems
  []); r1-sec-tf2.log:6 (+relaxed-simd +simd128 in a module built under deny(unsafe_code)); r1-te-final-check.log:154,
  :193 (the test step's FAIL and carry 65's name line); r1-te-te25-npt-net.log:17, :194 (the test ok, 169 passed).
- The spike branch since 51520b4: `git diff --stat 51520b4 HEAD` over U4d2's lane paths and crates/sim is empty; U4e's
  merge changed only crates/room, crates/server, crates/client and marks/ (README.md, SERIES.md +1 line). A13 is
  uncommitted in the main folder (crates/server/clippy.toml, the room file's two type entries and a header; read), its
  counted run under way (.gate/a13/a13-check-counted.log: fmt, clippy, clippy-wasm32, doc PASS so far). df 6.5 GB free.
- The main session's point 1, re-checked at 51520b4's plan (3f58ff0d): §5 U4d2 666-712, Lanes 850-888, §6's U7 line
  898-900, §7 908-931 (headings at 666/714, 850/890, 898, 908/933): as stated. Also stale in the same item 6 (brief
  :143): "U4r's lines that reuse your work (:766-767, :790-792)", which read :776-777 and :800-802 at 3f58ff0d (same
  text, read). §4.1, §4.11, §4.14, §4.15 and U4d's lines did not move.
- Point 4 re-checked: a grep of the copy's spikes/ finds three "two" texts, all in check.test.mjs: :328 and :364 (as the
  card says) and :529 ("unsafe code outside the two exception crates fails ...", a test name).
- §4.1 at HEAD read (:178-203): npt-lab's row lists only the subcommands; npt-hash-wasm's row "(cdylib; integer-only
  exports)". The loader's exports (copy): HASH_WASM_EXPORTS, LINKER_EXPORTS, LOADER_EXPORTS, BEGIN, STEP,
  moduleProblems, exportProblems, instantiate, forEachChunk, replay, parse, main; check.mjs 65 exports against <base>'s
  42 (+23, EA-7's count). SERIES.md's header-params table at HEAD: `fate_log_sha256` at :397.
- Neither exception crate with a deny level has a build script (crates/tcpinfo, crates/hash-wasm: no build.rs, no
  `build` key), so SEC-U4D2-4's build-script refusal costs nothing lawful.
