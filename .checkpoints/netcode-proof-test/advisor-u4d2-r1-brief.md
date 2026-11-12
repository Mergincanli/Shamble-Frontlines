# The advisor's ruling on U4d2's round 1: the brief (the main session, 2026-11-12 about 23:00 UTC; item 5 filled at 23:25)

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. You rule on round 1 of U4d2 under CLAUDE.md's review budget: all five seats' findings, each one's route,
the fix pass's scope, round 2's seats, and the merge's conditions. Write your ruling to
`.checkpoints/netcode-proof-test/advisor-u4d2-r1.md`, one block at a time as you finish each, so that a cut run leaves
every finished block on disk. Write files with the Write and Edit tools only (no sed, no shell redirect, no python
writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes
file whole with Write. Run nothing that builds or touches a copy; reading, git and sha256 are fine.

**What U4d2 is:** the Wasm-in-Node hash and the determinism step (u4-build-plan.md §5 U4d2): npt-hash-wasm (seven
integer-only `#[unsafe(no_mangle)] extern "C"` exports over npt-sim's `Replayer`, the workspace's third unsafe
exception) and its Node loader; npt-lab's `HashCompare`, `TickHash` and `hash` subcommand, the recordings manifest and
the std-sin scan data; npt-lab's clippy.toml; check.mjs's `wasm` step, features comparison, build-flag check with its
read-back, hash-wasm's rules and carry 65; carries 29, 41, 52 and 66. Built from the brief you approved (u4d2-brief.md
rev 1.2; your check: advisor-u4d2-brief.md, L1 to L5) on `<base>` bfda3cd (counts 399 / 446 / 52 of 52, 10 steps) by
one builder in `.claude/worktrees/u4d2-build` (branch u4d2-build at 51520b4, whose spikes/ equals bfda3cd's;
uncommitted). Its snapshot on the spike branch: `.checkpoints/netcode-proof-test/wip/u4d2.patch` (e2f2a5b; 17 files;
`git apply --check` exits 0 on c88eb31, U4e's merge, the spike branch's head since). A byte-identical read copy:
`.gate/u4d2-r1` (51520b4 plus the patch).

**Evidence (read whole; all paths under /home/user/Shamble-Frontlines):**
- The packet every seat started from: `.checkpoints/netcode-proof-test/u4d2-packet.md` and its side file
  `u4d2-packet-tests.md` (cut by review_packet.py from u4d2-packet.json; it holds no conclusion of the builder or the
  main session; base copy `.gate/u4d2-prefix`, a worktree at 51520b4). Your ruling advisor-u4d2-brief.md is in it whole,
  by ALLOWED's first addition (history.md, the note at its end).
- The five seats' notes, each with its verdict:
  - rust-reviewer `u4d2-review-rust.md`: REVISE (RR-M1, RR-M2 Medium, both marked before the merge; RR-L1 to RR-L5
    Low);
  - security-auditor `u4d2-review-security.md`: REVISE (SEC-U4D2-1 and -2 Medium, before the merge; SEC-U4D2-7 Medium,
    not before the merge, its route yours; SEC-U4D2-3 to -6 and -8 Low);
  - infra-engineer `u4d2-review-infra.md`: APPROVE WITH FINDINGS (INF-1 Medium, before the merge, text only; INF-2 to
    INF-7 Low);
  - engine-architect `u4d2-review-arch.md`: APPROVE WITH FINDINGS (EA-1 to EA-9, all Low);
  - test-engineer `u4d2-review-test.md`: REVISE (TE-M1 and TE-M2 Medium, before the merge; TE-M3 Medium, not before
    the merge; TE-L1 to TE-L4 Low): see item 5.
- The builder's record (its claims, not evidence): `.checkpoints/netcode-proof-test/u4d2-build.md`. Its run logs:
  `.gate/u4d2/` (e1-check.log and e2-check.log: check.mjs all 11 steps PASS, measured 416, bites 465, self 63 of 63;
  e3-h7.log; b8-wasm1.log and b8-wasm2.log; b8-scan1.log and b8-scan2.log; p1-features.log to p6-usize.log). The seats'
  runs: `.gate/u4d2/r1-*`.
- The copy itself: `cd .claude/worktrees/u4d2-build && git status --untracked-files=all` and `git diff`.
- One process event: the org's spend limit cut batch 2 (the architect and test seats) at 20:52 UTC, each just after
  writing its plan, with no plant in place (the builder's copy's spikes/ equal to the read copy's by `diff -rq`, checked
  at 22:52). Both were resumed from their notes at 22:53 UTC; batch 1's three seats had returned before the cut.

**The rules that bind it** (cited by place; Mustafa's words are not retyped here): CLAUDE.md's review budget (a round,
the fix pass, round 2 on what changed, a third only for an open Critical or High; a Low never triggers a push on its
own); Mustafa's shared-target rule, u4-build-plan.md §7.1 (items 1 and 7: every cargo call on the shared target; no
copy cleans it), which INF-1 meets; the 9.5 floor (PM-0001-2), in carry 52's text (u4s-carries.md, entry 52).

**What I ask you to rule on:**
1. **The Mediums marked before the merge:**
   - RR-M2 (the read-back gives rustc every rustflags source at once, but cargo uses only the first non-empty one, so a
     negation in an ignored source hides an enabling flag in the used one; shown by run, r1-rr-superset.log: `--only
     deps` exit 0 with CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge plus RUSTFLAGS=-Ctarget-feature=-simd128,
     -relaxed-simd, and the module built so lists +simd128 and +relaxed-simd). Its fix: read back each source alone,
     and the wasm step refuses a module whose target_features section lists either; your Q4 turned down re-implementing
     cargo's choice.
   - RR-M1 with SEC-U4D2-2 (one point: an `unsafe` block inside a listed export's body is accepted by rustc under the
     export's placed allow, so only the scan guards it; the scan catches it today, but neither a self-test nor the P3
     plant, which sits in a new `fn stray()` that rustc refuses anyway, pins it).
   - SEC-U4D2-1 (the loader's import object `{}` inherits Object.prototype, so an import named `constructor.constructor`
     resolves and the module instantiates with moduleProblems returning []; r1-sec-imports.log).
   - INF-1 (no cold time was measured; the brief's "cold (its first run: …)" is not met as worded; the seat's bound is
     about 80 s against 300 s; a real cold run needs an empty CARGO_TARGET_DIR, which §7.1 does not allow).
   For each: in the fix pass, with the fix's form, or another route. If any route needs an exception to §7.1, that is
   Mustafa's, not yours: say so and give your recommendation.
2. **The Medium marked not before the merge:** SEC-U4D2-7 (a safe `#[target_feature(enable = "relaxed-simd")]` function
   reaches the Wasm leg with no rustflag and no `unsafe`; the module's target_features section records it,
   r1-sec-tf2.log; the exposure is npt-sim's since U4s; U4d2's check is the one that claims simd128 and relaxed-simd
   off). Its route: this fix pass (the same module-section refusal as RR-M2's second half, fail closed when the
   section is missing) or a carry beside C-U7-1, with the words.
3. **The Lows that touch a build's integrity or a verdict:** SEC-U4D2-5 (`--cfg feature="bites"` in RUSTFLAGS compiles
   planted defects into a measured build unseen by the features comparison; `-C overflow-checks=off`; `-C llvm-args`;
   its fix: refuse every non-empty rustflags source, the spike's builds using none); INF-3 (the std-sin catch depends
   on the CPU through glibc's IFUNC `sinf`; elsewhere a false FAIL naming only the libc, never a false pass); INF-4 (no
   child call of the step has a timeout: a hung leg holds the exclusive lane lock); RR-L3 (`--digest-field` misnames a
   digest when the layouts differ; no wrong pass). Route each.
4. **The other Lows** (RR-L1: carry 29's comment says `sqrtsd` for sqrtf, which is `sqrtss`, and the carry's own words
   have the same slip; RR-L2, RR-L4, RR-L5; SEC-U4D2-3, -4, -6, -8; INF-2 Windows, INF-5, INF-6 U7's CI, INF-7 the
   README): each one's route (a fix-pass item where it is cheap and in the lane, a carry for U4r, U7 or U5, a
   follow-up, or the report's wording), with words where a carry is to be written.
5. **The engine-architect's and the test-engineer's findings:**
   - The engine-architect's nine Lows: EA-1 (the Wasm leg walks the state twice per tick, hash_lo and hash_hi; with
     RR-L4); EA-2 (the status numbers have two homes, lib.rs's consts and loader.mjs); EA-3 (check.mjs reads npt-lab's
     verdicts from its English lines and never the exit codes of `npt-lab hash` and `hash compare`; every path fails
     closed today); EA-4 (U7 can reuse the .wasm file as it is, not the loader); EA-5 with RR-L2 (LEAF's plan.md §4.2
     line neither names npt-hash-wasm nor holds for it: no clippy.toml); EA-6 (HashCompare serves U4r's -30
     pre-check unchanged, as your condition 2 asks, but TickHash.recording's doc invites legs named by run ids, which
     mismatch at entry 0, and the comparison is by position, not keyed by tick); EA-7 (the public items to record in
     W1-PUB's form: TickHash, HashCompare::of, HashCompare::Empty, the probe's field, the loader's and check.mjs's
     exports); EA-8 (the manifest holds only the generator's inputs; §4.15 puts U4r's recordings in it, so U4r changes
     its format); EA-9 (`hash compare` and the legs default `--on` differently; check.mjs always passes it).
   - The test-engineer, resumed after the cut with no plant in place, ended with all 17 files at their first sha256,
     `diff -rq` against the read copy empty and crates/sim untouched:
     - TE-M1 is RR-M2, proved by its own run (r1-te-tp2-flags.log, run 3: the negation case passes, the control
       fails); TE-M2 is RR-M1 with SEC-U4D2-2, proved by run (TP4: an `unsafe` read in begin's body is accepted by the
       wasm32 build, clippy and the unit tests and refused only by the deps step; TP10: a scan regression that lets
       it through leaves 63 of 63 self-tests and the live deps step passing). Both before the merge.
     - TE-M3, Medium, not before the merge, its TE2-5 entry: `l2::delay::tests::a_shutdown_s_never_completes_ends_at_
       its_deadline_without_spinning` (npt-net, U3's code, delay_tests.rs:2552) failed once in the bites build in its
       counted run (r1-te-final-check.log; carry 65's line named it; its panic text lost), passed on the immediate
       `--no-fail-fast` re-run (r1-te-te25-npt-net.log: 169 passed) and on a second test-step run; it failed the same
       way in U3c (u3c-fix2-c2b.md:74). A recurring flake under load, outside U4d2's crates, which can fail a merge's
       counted run.
     - TE-L1 (SEC-U4D2-8, by run TP11: the loader's kind check disabled, 63 of 63 pass); TE-L2 (supports SEC-U4D2-3,
       by run TP5: an f64 export caught by name by the deps scan only; the loader ran it); TE-L3 (the std-sin verdict
       never re-shows that the bite caused the mismatch; sound for the committed catch only because SS-s1 tick 2 lies
       inside the hash-map pair's equal prefix, ticks 0 to 1,233); TE-L4 (carry 65 names a failing test but check.mjs
       prints only its last 40 lines, so B2.3's "name and panic" cannot be met when the re-run passes, as with TE-M3).
     - Its other results: the builder's six plants and its own TP1 to TP11 each failed by name where they should
       (TP9: the builder's TE2-5 fix undone makes the unchanged base test fail, so that test still pins what it pinned
       at `<base>`); h1 to h8 assert what they claim; no wrong-reason path found for the hash-map verdict, one residual
       for std-sin (TE-L3). Its counted run: 10 of 11 steps passed, the test step failed on TE-M3's flake; a second
       test-step run passed with measured 416 and bites 465; self 63 of 63; the wasm step 40.0 s.
   Rule each, and TE-M3's route under B2.3: what the merge's counted run does if the flake recurs there, and where its
   root cause goes.
6. **Points of the builder's** (u4d2-build.md; its return list, section 6, items 9, 12 and 13), not in the packet:
   - TE2-5: one existing check.test.mjs test failed mid-build (62 of 63), a failure its step plan did not name; it
     fixed its new hash-wasm rule (which repeated the sentence "npt-sim is not a workspace member" that the test pins
     once), not the test, and judged it inside the unfinished step rather than a stop. Was that within TE2-5 (your
     B2.3) and section 4's stop clause?
   - Three early `npt-lab hash` runs (b3-lab1 to b3-lab3) took the runner's shared lock (`-s`), not `-x`; none is a
     reported result. rustfmt's diffs were applied by Edit after the cut.
   - Its open choices for the reviewers: a "cold" run cannot be measured under the touch loop without cleaning (INF-1);
     `--digest-field` reads both dumps at the native layout's indexes (RR-L3).
7. **Points of the main session's, not in the packet** (the card, `.checkpoints/netcode-proof-test.md`, NOW item 2).
   Confirm or correct each:
   - My restamp script (u4d2_restamp.py) left the bare-colon plan cites of the brief's section 1 item 6 and of its line
     173 at 686a583c. At 3f58ff0d (the plan at 51520b4) they read §5 U4d2 :666-712 (the brief: :666-702), Lanes
     :850-888 (:840-875), §6's U7 line :898-900 (:885-887), §7 :908-931 (:895-918). The builder built every Bites
     item of §5 U4d2 to its end, Q11's two plants included; the review checks the build against the whole section.
   - Carry 66's header param `fate_log_v1_datagram_sha256`: the builder proposes a marks/SERIES.md header-params row
     after :396, "| `fate_log_v1_datagram_sha256` | hex string | -3 |" (marks/ is not U4d2's lane). Who writes it, and
     when?
   - Your A13 (npt-server's clippy.toml gains the HashMap and HashSet type bans): the card's Next steps 4, after U4e's
     merge (done, c88eb31) and before U4r's brief. I plan to make it in its own commit while you rule (no agent runs
     cargo then); say if it should wait for U4d2's merge instead.
   - The "two exceptions" text: none outside U4d2's lane (docs/, plan.md and the notes: only records and the plan's own
     instruction quote the old phrase); inside it, the copy's check.test.mjs:328 (a test name) and :364 (a comment)
     still say "two".
   - The merge's counts are recomputed on c88eb31 (473 / 541 / 52 of 52, 10 steps, at U4e's merge): U4d2's deltas
     (+17, +19, +11, +1 step) would give 490 / 560 / 63 of 63, 11 steps, if nothing interacts; a counted run there takes
     about 11 minutes (INF-5), so it runs in the background, as U4e's M5 did.
8. **The fix pass:** its scope (the findings it takes), who builds it (one builder in the same worktree, reading its
   notes and the diff first, as U4e's fix pass did), and round 2's seats on what changed.
9. **The merge's conditions after round 2,** in M1 to M9's form (yours for U4e: advisor-u4e-r1.md and advisor-u4e-r2.md),
   with 11 steps: the snapshot, the apply on the spike head c88eb31, the gate, check.mjs counted in the main folder
   with the expected counts, the record's place, and the plan and carries text the merge commit carries (plan.md §4.2's
   three exceptions are already in place: plan.md:392-393 and §4.2).

For each item give APPROVE, REVISE or ESCALATE where one fits, the words wherever a text is to be written, and at the
end two lists: what must be done before the merge, and what goes to Mustafa (if anything), with the reason.
