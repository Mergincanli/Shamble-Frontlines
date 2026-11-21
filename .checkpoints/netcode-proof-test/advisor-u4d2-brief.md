# The advisor's check of the U4d2 builder brief (rev 1.1), 2026-11-12

Seat: a day session's advisor (workflow agent, opus), on main's `.claude/agents/advisor.md` (read with `git show
origin/main:.claude/agents/advisor.md`). Read only: no build, no run, no commit. Branch
`claude/netcode-proof-test-plan-hcky5g` at f94b078 (the main folder's head). The brief: u4d2-brief.md rev 1.1, sha256
e6bcefc841dd963dd4ce9c4bd72b3dc62efa5bf11aea2eb4aa06c3c51b9268fa (929 lines), as the task names it.

## Plan
1. Read the brief whole (N/u4d2-brief.md rev 1.1), the check (N/u4d2-brief-check.md) and the drafter's notes
   (N/u4d2-brief-notes.md, "Revision for the check").
2. Read the plan's §5 U4d2 (:666-702) and the parts of the plan, plan.md and the ADRs the brief leans on.
3. Read my conditions on U4d2 (advisor-u4.md 2, 7, 8, 10 (b)), W1-TARGET and W1-PUB's U4d part, the U4d-F ruling's T4,
   j-r2's B2.3 and B6, e-ruling's F2-ROUTE and L1-L4, e-r1's R6 and R9 and T2/T6.
4. Read carries "## U4d2 or the main session" (29, 41, 42, 52, 65, 66) and the recordings section (67, 71, 72).
5. Coverage table: every item of §5 U4d2 and of my conditions against the brief, testable as worded.
6. The check's findings TB-1 to TB-20 against rev 1.1 (TB-1 to TB-3 first; TB-7 and TB-11 with Q12).
7. Spot-check the brief's code claims at the head (crates sim, lab, l1, tcpinfo; check.mjs, check.test.mjs; Cargo.toml).
8. Rule U4D2-Q1 to Q12, with exact words.
9. The placeholders P1 to P8, 7.2's launch steps, `<base>`, the runner, U4e merging first; the lanes beside U4e.
10. The main session's points (task item 6).
11. Verdict, conditions, Decisions needed, For Mustafa.

## Blocks (written as each finishes)

### Block 1. What I read, and the state it rests on (done)
- The brief whole (929 lines, sha256 e6bcefc8…, as the task names it); the check (u4d2-brief-check.md, 276 lines); the
  drafter's notes whole (u4d2-brief-notes.md, 356 lines: facts F1-F25, choices C1-C17, conflicts K1-K15, the
  "Revision for the check" with its process slips and each finding's outcome).
- The plan (u4-build-plan.md, sha256 686a583c…, 1,079 lines): §2's Features and C-U7-1 rows (:157, :159), §4.1
  (:178-203), §4.11 (:392-407), §4.14's hash-map row (:476), §4.15 (:478-504), §5's preamble (:507-512), U4d's D-2,
  D-7 and Bites (:612-614, :629, :640-642), §5 U4d2 (:666-702), U4r's -30 comparer and Evidence lines (:766-767,
  :790-792), Lanes (:840-875), §6 (:877-893), §7 (:895-918), Q9 (:1002-1005). plan.md (rev 3.3, 2bef127b…) §4.2
  (:432-450) and §4.5 (:527-551), its out-of-scope list (:324-329) and R-8 (:1401), its U7 row (:1247).
- My rulings: advisor-u4.md conditions 2, 7, 8, 10, 12 and the Q3 and Q9 rulings (:90-101, :130-149, :154-157,
  :198-199, :207); W1-PUB's U4d part and W1-TARGET (advisor-u4-wave1.md :387-416, :558-586); the U4d-F ruling's T4
  (advisor-u4df.md :459-470); the U4j round-2 ruling's B2.3 and B6 (advisor-u4j-r2.md :146-171, :283-298); the U4e brief
  check (advisor-u4e-brief.md, whole: the form followed here); the U4e round-1 ruling's R6, R8, R9, T2, T6 and "Before
  the merge" (advisor-u4e-r1.md :317-353, :386-493, :504-600, :681-730).
- u4s-carries.md (cdab0240…, 490 lines): "## U4d2 or the main session" (:407-447: entries 29, 41, 42, 52, 65, 66), the
  recordings section (:351-379: 67, 71, 72), entry 34 (:62-69), carry 39 (:169-179); every other mention of U4d2 (:68,
  :178: neither routes work to U4d2).
- ADR 0004's PM-0004-21, -22 and -74 rows (docs/adr/0004-simulation-core.md:1142, :1143, :1184).
- The runners: u4e-run.sh whole (98 lines, sha256 669f7c65…), main-run.sh :1-25; .github/workflows/gate.yml whole
  (the only workflow).
- Code at the main folder's head (f94b078 at my start; the saver's 3de696c since, notes only): spikes/netcode-proof
  equals 28a85e7 (`git diff --stat 28a85e7 HEAD -- spikes/netcode-proof` empty); adb1bab (U4d-F) and d50396a (U4j) are
  ancestors of HEAD (`git merge-base --is-ancestor`, exit 0 each). Read: replay.rs :285-289, :367-373, :608-706;
  hash.rs :10-34, :64-116; world.rs :585-636, :792-796, :930-1000, :1016-1023, :1214-1229, :1924-1928, :1955-1962,
  :2012-2022, :2139-2146; sim/tests/replay.rs :21-43, :296-340, :380-390, :512-516; l1/src/lib.rs :320-356;
  l1/tests/l1.rs :476-535; lab (Cargo.toml, lib.rs, main.rs :1-20, :60-215, probe.rs :15-30, :110-125);
  ops/runs/c-u2-pm-0001-3.mjs :1-80; check.mjs :25-35, :70-175, :1196-1290, :1395-1445, :1527-1663;
  check.test.mjs :1094-1105, :1617-1660 and its four "only npt-client-wasm and npt-tcpinfo may" pins (grep);
  Cargo.toml :1-25, :45-56, :70-115; crates/proto/Cargo.toml; crates/client-wasm/Cargo.toml; every crates/*/clippy.toml;
  rust-toolchain.toml; .cargo/config.toml; wip/u4e.patch's file list and its two clippy.toml hunks.
- Stamps now: brief e6bcefc8…; plan 686a583c…; plan.md 2bef127b…; carries cdab0240…; df-ruling 842ec092…; j-r2
  7efa9410…; e-r1 7c129309…; history c62ac831… (1,925 lines); the card 2c40717e… (613 lines: it moved since the
  brief's 6b36da10… stamp, as a living file does; the brief cites it by section only, so nothing in the brief breaks).
- Disk now: 7.8 GB free on /home/user; the shared target 6.8 GB (220 MB of it release); the main folder's own cache
  (.gate/netcode-proof-test/target) 6.7 GB.

### Block 2. Coverage of §5 U4d2 and of my conditions, testable as worded (done)
"Testable" means the brief's instruction ends in an exit code and a test runner's own summary line (check.mjs's step
line and exit code, libtest's or node --test's summary), not in a reading.

| Plan item or condition | Brief | Verdict |
|---|---|---|
| hash-wasm cdylib, npt-sim only, `bites` forwards (:667-670) | 2.1 manifest | present, testable (2.4 (d)'s deps lines) |
| the seven exports, `#[unsafe(no_mangle)] extern "C"`, integers only (:668-669) | 2.1; Q3, Q7, Q8 | present; `begin`'s arguments are Q3's (ruled below) |
| the Node loader, `WebAssembly.instantiate`, streaming (:670-671) | 2.2 | present, testable (2.4 (a), (d)) |
| `npt-lab hash`, release, two native legs in two processes, first tick and field diff (:670-672) | 2.3 | present, testable (h6, h1) |
| `HashCompare` on two per-tick lists, reused by U4r's -30 comparer (:672-673; condition 2) | 2.3, h4 | present, testable; it reads no file and knows no recording, which is what U4r needs |
| the `wasm` step: release build, flags, Node run (a script: the runner refusal stands, its note updated), the comparison, wall time, 5 min (:675-676) | 2.4 (a); Q2 | present, testable; the CI half of its timing names a run that does not exist (finding A1) |
| the features comparison, like for like, measured and bites apart (:677-678; A12) | 2.4 (b); Q6, Q12 (b) | present, testable; a shallow walk is not yet caught (finding A5) |
| the build-flag check, refusal in the sources, read-back that fails, self-test bite (:679-683; condition 7 (a)) | 2.4 (c); Q4 | present, testable; never only reports |
| hash-wasm's rules: its own `[lints]`, one allow per export, the scan, LEAF and WASM_CRATES, its dependency rule (:684-687) | 2.4 (d) | present, testable |
| the third exception: invariant, test, `UNSAFE_EXCEPTIONS`, plan.md §4.2 (:688-693; condition 7 (b), (c)) | 2.1, 2.2, 2.4 (d), (e); Q7, Q8 | present; plan.md §4.2 already names three (plan.md:436-447: read) |
| 0 mismatching ticks over 27,000, native twice and Wasm once (:694) | h1 | present, testable, exact counts |
| PM-0004-21's cross-target vectors (:694-695; T22) | h2; Q11 | present, testable |
| Bites: std sin (:696-697) | 2.4 (a) item 6; 2.7; Q5 | present, testable both ways the scan can fall |
| Bites: hash-map, "caught, or recorded as not catchable on wasm32" (:697-698) | 2.4 (a) item 6; 2.7 | present, testable (D-2's rule per pair; the step FAILs if no pair was exercised) |
| Bites: an extra feature (PM-0004-74), each flag, a stray `unsafe`, an export not on the list (:698-700) | 2.4 (b)-(d); 2.7 | present; several plant runs cannot show all their expected failures under the runner as it is (finding A2) |
| Reviewers (:701-702) | 7.2 item 7 | present (xhigh, counted: condition 10 (c)) |
| §5's preamble: deltas, clippy host and wasm32 both builds, fmt, Cargo.lock by the named dependencies only, pub items (:507-512) | sections 4, 5, 6; 2.5; 2.9 | present, testable |
| condition 8 (C-U7-1: wasm-opt is U7's) | 2 intro; 2.4 (a) item 1; 2.6's "not built" | present; plan.md's U7 row holds C-U7-1 (plan.md:1247: read) |
| condition 10 (b) (only U4d2 edits crates/lab) | section 4's files | present |
| W1-TARGET items 1-3 | section 4 rules 1-6 | present (the lock per call, provenance by the four release builds, the disk floors) |
| carries 29, 41, 42, 52, 65, 66 | 3.1 (P1) and its notes; 2.3-2.5; h7 | present; each has a home, a test or a "not applicable" with its reason |
| 67, 71, 72 do not apply | 3.2; 2.8 | agreed: U4d2 edits no crates/sim file, regenerates with the merged generator and pins what D-7 pins; every scratch plant is restored before the next call. The reading holds only while that stays true, which 2.8's stop rule keeps |

- **Beyond the plan:** nothing built that the plan, a carry or a ruling does not name. The scan data file, the vector
  file, the lab's clippy.toml and the exit-code form of carry 66 (h7 (ii)) each answer a plan item or a carry; the
  "plan silent" choices (C1-C17) are the least that makes an item testable, and none adds a feature for later.
- **Carry 42, checked myself:** entry 34 (carries :62-69) was met by U4a (u4a-build.md:125-126 lists the fixes; the
  check's C-rows and history.md:1335 agree), so carry 42's condition does not hold and the brief rightly carries no
  word of entry 34.
- **Carry 52, checked myself:** adb1bab is an ancestor of the head; `Recording::GENERATOR` is 2 (replay.rs:288) and
  D-7 pins it (sim/tests/replay.rs:382); the brief's 1.1 makes `<base>`'s ancestry a recorded first step.

### Block 3. The test-engineer's check against rev 1.1 (done)
Each finding against rev 1.1's text, and where the text cites code, against the code at the head.
- **TB-1 High (placeholders by line range in living files): answered as asked.** P1 by its heading, its entry set
  asserted; P2 by its first words in history.md (found once at :292, the line before it :291 "  USAGE-SAVINGS: (a), all
  three, with these limits.", the continuation :293 ending "before it changes anything.": checked); P3, P4, P5 and the
  new P8 from the closed rulings, each file's sha256 asserted (842ec092… and 7efa9410…: both hold now) and each span's
  first and last words named (df-ruling :462-464 and :465-468, j-r2 :168-171 and :298: each lands, read); every card
  line number gone (grep: only section and item cites remain); 7.2 item 8's re-stamp and plan re-check. Two gaps the
  answer leaves, both Low, with words in block 4: a placeholder that stands in several places (A4), and the carries
  cites that move if U4e merges first (A7).
- **TB-2 High (no verdict rule for the bites half): answered as asked, with one addition that holds.** Hash-map: D-2's
  rule per pair (sim/tests/replay.rs:300-308, read), three pairs, a FAIL when no pair is exercised, every digest
  printed. Std-sin: the scan's result committed as data (crates/lab/), the step's (R, T', F) rule, its FAIL naming the
  libc; the self-test's four outcomes per bite. The addition (the scan data names its libc; a moved T' names the
  step's) is test strength, not a feature. Two Lows ride it: which recording the step arms when several catch (A8),
  and the libc note's "CI" (A1).
- **TB-3 High (Q4 (a)'s property untested): answered as asked.** The two live runs that only the read-back can catch
  (`target-cpu=haswell` on x86-64, `target-cpu=bleeding-edge` on wasm32), the argv pin per source, and the clean
  read-back's non-vacuity (sse2, sign-ext). The facts it rests on (R24, notes F21) are the test-engineer's and the
  drafter's own `rustc --print cfg` runs; I re-ran none (block 10). As worded, each live run ends in check.mjs's exit
  code and its deps line naming the feature: testable.
- **TB-4 Medium (the word-size plant's site): answered as asked.** `write_state` (world.rs:1926) is the walk both
  `dump` (:1018-1022) and `state_hash` (:2141-2145) call (read). The expected outcome per pair needs the step to print
  every comparison on a FAIL and the runner to run past the first failing command (A2).
- **TB-5 Medium: answered as asked** (two Wasm legs in two Node processes; the std source cited at the toolchain's tag;
  the step decides from the digests it prints, not from the citation, so no verdict rests on a page I did not read).
- **TB-6 Medium: answered as asked** (exact 9,000 and 27,000; the six cases, and more).
- **TB-7 Medium: answered with a change that holds (U4D2-Q12 (a), ruled in block 5).** I read the four pins (:288,
  :486, :508, :1141: each quotes "only npt-client-wasm and npt-tcpinfo may", and `unsafeProblems` builds that message
  from `UNSAFE_EXCEPTIONS`), the deps fixture's summary pin (:1099: a pattern over "held: …" and "(6 crate files and 1
  cargo config file(s) read)") and the steps pin (:1617-1658: it pins each step's commands, builds its wasm32 line from
  `WASM_CRATES` and pins no step list). The drafter's reading is right on the code. `<crate>` is replaced in the copy,
  as asked, but its second use reads badly (A3), and the deps fixture's call of `depsStep` is a test the builder
  extends, like the steps pin (A10).
- **TB-8 Medium: answered as asked** (the four release builds before check.mjs in the same `-x` call; matching on the
  path).
- **TB-9 Medium: answered as asked, but its CI half cannot be met:** no CI job runs check.mjs (A1).
- **TB-10 Medium: answered as asked** (an `#[ignore]` test run by name: libtest's summary line). One Low rides it: B1's
  burst and where the test sits (A6).
- **TB-11 Medium: answered with a change that holds (U4D2-Q12 (b)), with one self-test case added (A5).**
- **TB-12 to TB-20 Low: each answered as asked.** Checked against the text: 2.1's id rule (TB-12), 7.2 item 7 and the
  header (TB-13), 7.2 item 1 (TB-14: main-run.sh:13-15 sets the shared target, read), h3 (TB-15), 2.4 (c)'s three
  controls (TB-16), 2.5's one lock-writing call (TB-17), h2's reason and text file (TB-18), 2.4 (f)'s three cases
  (TB-19), rule 2 (TB-20).
- **So:** rev 1.1 answers all twenty, the three Highs as asked. What remains is mine: three Mediums (A1, A2 in block
  4; A12 in block 7) and nine Lows (A3-A10 in block 4; A11 in block 6), none a High, none a security finding (A12 adds a
  security seat; it finds no defect). (Counts updated at block 7.)

### Block 4. My findings on rev 1.1, each with its exact words (done)
Severity on CLAUDE.md's scale. Every "old" phrase is the brief's (or the plan's) text read with its line breaks and the
next line's indent as one space; I counted each in the flattened file (python, read only): each occurs exactly once,
the one ordered pair aside (A1 (a) before (d)). The fold rewraps to 120.

**A1, Medium: no CI job runs check.mjs, so the timing's CI half and the infra-engineer's seat name a run that does
not exist.** The only workflow, .github/workflows/gate.yml, runs tools/gate.mjs and its smoke test, never
spikes/netcode-proof/check.mjs; and plan.md puts "a CI job for `spikes/` (R-8)" out of scope (plan.md:328; R-8 at
:1401: "approved; no CI job"). Plan :702's "infra-engineer (the step's time in CI)" and the brief's "The CI figure is
the infra-engineer's, from CI's run of the merged tree" therefore read a number nobody can produce; the reviewer seat
would come back with nothing, and the det-u4.mjs decision (plan :790-792) would lack its input. A CI job is .github/
work (risk tier 2, Mustafa's go) and out of the proof test's scope, so the reading changes, not the scope.
- 2.4 (a), Timing: replace "The CI figure is the infra-engineer's, from CI's run of the merged tree (plan :702:
  "infra-engineer (the step's time in CI)")." with "No CI job runs check.mjs (.github/workflows/gate.yml runs
  tools/gate.mjs only, and a CI job for spikes/ is out of scope: plan.md:328 and R-8), so plan :702's "the step's time
  in CI" is read as the step's time as a fresh build meets it: the infra-engineer reviews your cold and warm figures
  against the 5-minute budget, and says what a CI job would change (the advisor's U4d2 brief check, A1)."
- 2.4 (a) item 6, std-sin: replace "(CI's glibc may differ from this container's 2.39)" with "(another machine's
  glibc may differ from this container's 2.39)".
- Section 5 item 4: replace "against the 5-minute budget (plan :676); CI's figure is the infra-engineer's (2.4 (a))."
  with "against the 5-minute budget (plan :676); the infra-engineer reviews these figures (2.4 (a): no CI job runs
  check.mjs)."
- 7.2 item 7 (after the 2.4 (a) replacement, the phrase occurs once): replace "infra-engineer (the step's time in CI);"
  with "infra-engineer (the step's time against its budget, from your cold and warm figures: no CI job runs check.mjs,
  A1); security-auditor (the third `unsafe` exception, and check.mjs's unsafe and rustflags scans: A12);". (Edited at
  block 7 to carry A12.)
- **The plan** (the main session's commit): §5 U4d2's Reviewers line, replace "infra-engineer (the step's time in CI)."
  with "infra-engineer (the step's time against its budget; no CI job runs spikes/: plan.md R-8; the advisor's U4d2
  brief check, A1), security-auditor (the third `unsafe` exception and check.mjs's unsafe and rustflags scans: the same
  check, A12)." (Edited at block 7 to carry A12.)

**A2, Medium: several plant proofs cannot show all their expected failures under the runner as it is.** u4e-run.sh
stops a call at its first failing command (:74-88), and section 4 rule 2 forbids leaving a plant in place between
calls. 2.7's h2 row expects, in one call, the `wasm` step to fail and then the h2 unit test to fail; the stray-unsafe
row expects the deps step and rustc to refuse; the export row expects the loader and the deps scan to refuse; plant (1)
of the h2 row expects h2 (i) and the step's vector check to fail. In each, the second command never runs. And the h2
row's key evidence, leg 1 against leg 2 at 0 while native against Wasm fails, shows only if the `wasm` step prints
every comparison on a FAIL; as worded it may stop at its first failing sub-check. The builder would have to bend a
rule or stop.
- 7.2 item 2: replace "u4d2-run.sh: u4e-run.sh with the log check's folder `.gate/u4d2/` (u4e-run.sh:30-33) and its
  header naming U4d2; nothing else changes." with "u4d2-run.sh: u4e-run.sh with the log check's folder `.gate/u4d2/`
  (u4e-run.sh:30-33), its header naming U4d2, and one option more, `-k`, given after `-x` when both are: every command
  of the call runs even after one fails, each exit code logged as now, and the call exits with the first non-zero
  code; without `-k` it stops at the first failing command, as u4e-run.sh:74-88 does. Nothing else changes (the
  advisor's U4d2 brief check, A2)."
- 1.4: replace "`bash <runner> [-x] <worktree> <log> <command...> [:: <command...>]`" with "`bash <runner> [-x] [-k]
  <worktree> <log> <command...> [:: <command...>]`"; and replace "exit code and the provenance lines (u4e-run.sh:1-12,
  :38-39, :58-62, :91-97)." with "exit code and the provenance lines (u4e-run.sh:1-12, :38-39, :58-62, :91-97). It
  stops at the first command that fails; with `-k` it runs them all and exits with the first non-zero code (7.2 item
  2)."
- 2.4 (a), after "Its last line (the line main prints: :1646) gives the ticks compared per pair, the mismatches, each
  bite's outcome and each timed call's seconds." add: "On a FAIL the step still runs every comparison it has the legs
  for, and prints one line for each (its pair, the ticks compared, the mismatches, the first mismatch with its field)
  and for h2's vectors and each bite, within the last 40 lines main prints under a FAIL (:1649), so one run shows which
  pairs agree (2.7's h2 row reads leg 1 with leg 2 beside leg 1 with Wasm); a trap or a failed regeneration ends only
  the legs that need it."
- 2.7's lead: replace "its commands go in one runner call with `-x` (separated by `::`);" with "its commands go in one
  runner call with `-x -k` (separated by `::`), so that each expected failure shows;".
- 2.7's h2 row: replace "in one -x call" with "in one `-x -k` call".
- Section 4 rule 2: replace "a plant's commands go in one call," with "a plant's commands go in one call (`-x -k`: 7.2
  item 2),".

**A3, Low: the stop clause's second `<crate>` reads badly.** Rev 1.1's one string in both places gives "or in a test
that reads npt-hash-wasm, npt-lab, check.mjs's wasm and deps steps or check.test.mjs's files". My U4e round-1 ruling
(R8) named the crates once and wrote "their files"; the same here.
- 7.2 item 3's P4 bullet: replace "with both `<crate>` replaced by "npt-hash-wasm, npt-lab, check.mjs's wasm and deps
  steps or check.test.mjs" (the T4 item: "reads, with the unit's crate named"); the script asserts two replacements and
  no `<crate>` left." with "with the first `<crate>` replaced by "npt-hash-wasm or npt-lab, in check.mjs's deps or wasm
  step, or in check.test.mjs" and "<crate>'s files" by "their files" (the T4 item: "reads, with the unit's crate
  named"; the form of the advisor's U4e round-1 ruling, R8); the script asserts the two replacements and no `<crate>`
  left."

**A4, Low: five placeholders stand in more than one place, and 7.2 item 3 does not say where the copy goes or how a
wrapped source is copied.** P2 stands at 1.3, 1.4 and section 4; P3 at section 4 and section 5 item 3; P4 and P5 at 1.5
and section 4; P8 at 2.7 and section 4; and 7.2 item 3's own list names each again. "The placeholder gone" then
either copies Mustafa's words three times, two of them inside parentheses mid-sentence, or fails its own assert; and
P3, P4 and P5 wrap across indented lines in their sources.
- 1.3: replace "(the builders' limits, `<<P2>>`)" with "(the builders' limits: section 4)".
- 1.4: replace "never a grep (`<<P2>>`)." with "never a grep (the builders' limits: section 4)."
- 1.5: replace "(`<<P4>>`, with `<<P5>>`)" with "(section 4's stop clause)".
- 2.7's lead: replace "or its diff saved beside the run's output (`<<P8>>`), one at a time" with "or its diff saved
  beside the run's output (section 4's plant rule), one at a time".
- Section 5 item 3: replace "a diff is applied by Edit (`<<P3>>`)." with "a diff is applied by Edit (section 4's write
  rule)."
- 7.2 item 3's lead: replace "(the anchor found once, the copy's first and last words as named, the placeholder gone,
  no other byte of the brief changed):" with "(the anchor found once, the copy's first and last words as named, the
  placeholder gone from its one home outside this list, no other byte of the brief changed; this list keeps the names
  as the record; P6 and P7 are values, filled wherever they stand outside this list; a source span that wraps is
  copied with each line break and the next line's indent read as one space and rewrapped to 120, and the assert
  compares copy and source with every run of white space read as one space):".

**A5, Low: the features comparison's self-test does not catch a walk of npt-sim's direct dependencies alone.** Rev
1.1's non-vacuity (each list holds npt-sim and its manifest's direct dependencies) and its one self-test case (libm,
a direct dependency) both pass a walk that stops one level down, which would miss a feature difference on ppv-lite86
or zerocopy (notes F4). TB-11's point was the walk's reach; one case closes it without a pinned list.
- 2.4 (b)'s self-test: after "fail, naming libm and the feature;" insert " recorded outputs where only a transitive
  dependency differs (zerocopy, below rand_chacha through ppv-lite86) fail, naming it (a walk of npt-sim's direct
  dependencies alone would pass them);".

**A6, Low: h7 (ii)'s -3 arguments leave B1's burst and the test's home unsaid, and carry 66's summary form is open.**
B1's burst comes from main.rs's `profile` with `--burst-at-s` defaulting to 50 (main.rs:75, :89-93), and `probe` is a
module of the binary, not of the library (main.rs:13-14), so a test that calls `probe::run` sits in main.rs's or
probe.rs's test module. The summary line prints the whole log's hash as 16 hex digits (main.rs:182).
- h7 (ii): replace "(the default packet count and rate: main.rs:73-75, probe.rs:20-22; the -3 script's call, :65-72)"
  with "(the default packet count and rate: main.rs:73-75, probe.rs:20-22; B1's burst at the default 50 s, built as
  main.rs's `profile` builds it: :75, :89-93; the -3 script's call, :65-72; `probe` is a module of the binary,
  main.rs:13-14, so the test sits in main.rs's or probe.rs's test module)".
- 2.3's carry 66 bullet: replace "and in the summary line (main.rs:167-183)," with "and in the summary line
  (main.rs:167-183; as the first 16 hex digits, the whole-log hash's form there, :182, or whole: your report says
  which),".

**A7, Low: the carries cites that move if U4e merges first are not re-checked.** U4e's merge commit applies T2, which
inserts carries 73-77 at §U4r's end (above u4s-carries.md :323), so the recordings section (:351-379, cited in 1.6
and 3.2's heading) and §U4d2 (:407-447, cited in 1.6 and U4D2-Q9) move; 7.2 item 8 re-checks plan cites only. (T6
rewrites three plan rows in place, one line each: no plan line moves, only the plan's sha256.)
- 7.2 item 8: replace "re-check every plan cite of this brief and fold the new lines in the main session's commit;"
  with "re-check every plan cite and every u4s-carries.md cite of this brief (T2 inserts carries 73-77 above the
  recordings section and §U4d2; T6 moves no plan line) and fold the new lines in the main session's commit;".

**A8, Low: which recording the std-sin step arms when the scan catches on more than one.**
- 2.4 (a) item 6: replace "If it records a catch (recording R, tick T', field F):" with "If it records a catch
  (recording R, tick T', field F; with catches on more than one recording, the first in the manifest's order, the
  others recorded beside it in the data):".

**A9, Low: `feed` has no result, so Q7 (a)'s rule must allow none.**
- 2.4 (d): replace "By U4D2-Q7 (a), each listed export's parameters and result hold u32 and i32 only;" with "By
  U4D2-Q7 (a), each listed export's parameters, and its result if it has one, are u32 or i32 only (`feed` returns
  nothing, which Wasm lowers to no result);".

**A10, Low: the deps fixture test's call of `depsStep` is a test the builder extends.** The fixture test calls
`depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }))` (check.test.mjs:1096-1098); each new deps-step check
that runs a subprocess (`cargo tree`, `rustc --print cfg`) is injected as `metadata` is (notes C9), so that call
changes in the step that adds the check, or it runs real subprocesses on the fixture.
- Section 4's expected failures: after "The steps pin (:1617-1658) is one you extend (2.4 (g)); a failure there is not
  expected (U4D2-Q12 (a))." add " So is the deps fixture test's call of `depsStep` (:1096-1098): each new check's
  injected runner (the `cargo tree` and `rustc --print cfg` calls, passed as `metadata` is: notes C9) joins that call
  in the step that adds the check."

### Block 5. U4D2-Q1 to Q12 (done)
The brief is written to its recommendations; where I rule as recommended, its body stands and only the words below
change. Plan words are for the main session's own commit before the launch; each old phrase occurs once in the
flattened plan (checked), and my advisor-u4.md condition 12's list of allowed plan edits gains them by this ruling.

**U4D2-Q1 (the plan's moved check.mjs cites): (a).** The plan is a record approved at a hash; section 0's table names
every moved line, and U4d2 moves check.mjs again. The U4e brief's way (u4e-brief.md:127-130).
- Brief, section 0: replace "This brief uses the lines at the head (U4D2-Q1 asks whether the plan text is re-pointed):"
  with "This brief uses the lines at the head (U4D2-Q1 (a): the plan's text stays as approved, and this table is the
  reading):".

**U4D2-Q2 ("§4.5's flags"): (a).** The build plan's §4.5 (:292-304) holds no flag; plan.md §4.5 (:533-535) is the set
the rest of the row's checks come from, and wasm-opt is U7's (condition 8).
- Plan, §5 U4d2's `wasm` step: replace "the release build with §4.5's flags," with "the release build with plan.md
  §4.5's flags (baseline x86-64, no `+fma` or `target-cpu=native`, simd128 and relaxed-simd off; wasm-opt is U7's:
  C-U7-1; the advisor's U4D2-Q2 ruling),".

**U4D2-Q3 (`begin`'s seed and scene): (b), `begin(bite, arm_tick)`.** `Replayer` reads scene and seed from the
header (replay.rs:8-15, read; `Replayer::new` takes nothing: :645-648, read), so (a) would
put a second reader of the header's layout outside replay.rs and (c) a signature that says what it does not do. The
manifest's SHA-256 check binds each file to its entry before a word is fed, which is stricter than two fields, and
`HashCompare` names each entry's recording, so a leg fed the wrong file fails at its first tick.
- Plan, §5 U4d2 Builds: replace "`begin(seed_lo, seed_hi, scene, bite, arm_tick)`," with "`begin(bite, arm_tick)` (the
  recording's header carries the scene and the seed, and the manifest's SHA-256 binds each file to its entry: the
  advisor's U4D2-Q3 ruling),".
- Brief, 2.1: replace "(by U4D2-Q3 (b); under (a) the plan's `begin(seed_lo, seed_hi, scene, bite, arm_tick)`,
  failing at the header when scene or seed differs)" with "(U4D2-Q3 (b), the advisor's ruling: the header carries
  scene and seed)".

**U4D2-Q4 (the read-back's flags): (a), with the toolchain named.** A bare `rustc --print cfg` sees no flag cargo
adds, and `cargo rustc --print` is unstable on 1.99.1 (notes F2), so (c) can fail only on a toolchain change, against
condition 7 (a)'s "never only reports"; (b) re-implements cargo's choice of source and can be wrong in the same way
cargo's own rules change. (a) is a superset, fail-closed, and rev 1.1 proves it with runs only the read-back can catch.
rustup chooses the toolchain by the folder (rust-toolchain.toml pins 1.99.1 in spikes/netcode-proof), so the read-back
must run there to read the rustc cargo uses.
- Brief, 2.4 (c) (ii): replace "`rustc --print cfg --target <t>` for x86_64-unknown-linux-gnu and wasm32-unknown-unknown,
  given," with "`rustc --print cfg --target <t>` for x86_64-unknown-linux-gnu and wasm32-unknown-unknown, run from
  spikes/netcode-proof (so rust-toolchain.toml picks 1.99.1, the toolchain cargo uses; the step's line names `rustc
  --version`), given,".
- No plan words: "read back with `rustc --print cfg` per target" (plan :680-682) still says what is built.

**U4D2-Q5 (the std-sin bite): (a).** U4d's site takes its input from the vehicles' loops and `with_bite` takes only an
arming tick (world.rs:626-635, :940-966, :994-1000, :1214-1229: read), so the "scan for inputs" can only be a scan of
the loops' ticks, which every recording with vehicles meets within one period (at most 3,900 ticks: :794, read). If
the targets differ, the committed step holds a real sin catch at its tick; if they never differ, ADR 0004:1142 itself
says "recorded as not catchable on those targets and a 1-ulp planted change stands in", and the scratch stand-in shows
the step catching a change at that site at its arming tick. The step's catch-at-the-tick machinery is re-proved on
every run by the hash-map bite's native pair. (b) would need a crates/sim change by the main session with no wave-2
owner, a new pub item, and carries 67-72 checked, for a bite whose recurring value over (a) is small. Not looser:
plan.md §4.5:551's "caught at the first tick it changes" holds both ways (a bite that changes nothing has no tick to
catch), and no ADR or mark word changes.
- Brief, 2.4 (a) item 6: replace "and runs no std-sin leg; U4D2-Q5's ruling says what stands in." with "and runs no
  std-sin leg; the stand-in is 2.7's scratch plant, shown once (U4D2-Q5 (a))."
- Brief, 2.7's std-sin row: replace "the stand-in is U4D2-Q5's" with "the stand-in is the scratch plant in this row's
  last column (U4D2-Q5 (a))".
- No plan words: the plan's Bites line ("a 1-ulp planted change stands in") holds as it is.

**U4D2-Q6 (which builds the features comparison compares): (a), three builds.** The 27,000-tick result means "the
server's sim" only if the native legs' build (npt-lab's) resolves npt-sim's features as the server's does; ADR
0004:1184 asks it of "every build that runs the sim". One more `cargo tree` call, equal today (notes F4).
- Plan, §5 U4d2's features bullet: replace "npt-sim's resolved features under the server build against the hash-wasm
  build," with "npt-sim's resolved features under the server build and under npt-lab's (the native legs' build)
  against the hash-wasm build (the advisor's U4D2-Q6 ruling),".

**U4D2-Q7 (the invariant's second clause): (a).** The Rust signature is the Wasm signature's source, the scan already
finds the listed exports, and an f64 parameter is a bite it can fail on. (b) would read the type section in the loader
for the same fact; (c) leaves a clause no test can fail, which ADR 0002:264-265's "a test" does not allow. A9's words
let `feed`'s missing result through.
- Plan, §5 U4d2's third-exception bullet: replace "refuses any not on the list (its bite below);" with "refuses any not
  on the list (its bite below), and the deps scan refuses a listed export whose parameters or result are not u32 or
  i32 (its bite: an export taking an f64; the advisor's U4D2-Q7 ruling);".

**U4D2-Q8 (the names, and "no exported name collides"): (a).** The native build is never loaded as a library, glibc's
`step@GLIBC_2.2.5` is a compat version no new link binds (notes F5), and the names stay the ones U7's Chromium check
will load (plan :885-886). The invariant stays true as written, with what it was checked against recorded in the crate
doc (2.1). No words change.

**U4D2-Q9 (which manifest lines are U4d2's): (a), and crates/client-wasm/Cargo.toml's lints comment with them.** The
carries' own section names U4d2 and these files, no wave-2 lane holds them, and comment lines change nothing cargo
resolves. The client-wasm comment (:9-13: "one of the workspace's two unsafe_code exceptions") becomes false at the
very commit that adds the third exception, so the same lane makes it true (the main session's point, block 8); no
other text outside check.mjs and check.test.mjs says "two" (grep: only Cargo.toml:75 and client-wasm's :9).
- Brief, 2.5: after "- crates/proto/Cargo.toml: carry 41's comment lines (:13-14) only, by U4D2-Q9 (a); no feature or
  dependency change." add "\n- crates/client-wasm/Cargo.toml: its lints comment (:9-13, "one of the workspace's two
  unsafe_code exceptions") names three, npt-hash-wasm the third, in the workspace comment's form; comment lines only
  (U4D2-Q9 (a))."
- Brief, 2.4 (e): replace "Cargo.toml:73-92, in 2.5)" with "Cargo.toml:73-92 and crates/client-wasm/Cargo.toml:9-13,
  in 2.5)".
- Brief, section 4's files: replace "crates/proto/Cargo.toml's two comment lines (by U4D2-Q9)." with
  "crates/proto/Cargo.toml's two comment lines and crates/client-wasm/Cargo.toml's lints comment (:9-13), comments
  only (by U4D2-Q9 (a))."
- Brief, 3.1's carry 41 note: replace "and nothing else; under (b) or (c) your report gives it as not met (not your
  lane)." with "and nothing else (U4D2-Q9 (a))."
- Brief, return item 9: replace "texts outside your lane that still say "two" exceptions
  (crates/client-wasm/Cargo.toml:9-13's comment: "one of the workspace's two unsafe_code exceptions");" with "any text
  outside your lane that still says "two" exceptions (crates/client-wasm/Cargo.toml's comment is yours: 2.5);".
- Plan, Lanes' wave-2 line: replace "U4d2 owns check.mjs, Cargo.toml's members, Cargo.lock, crates/hash-wasm and
  npt-lab's `hash`;" with "U4d2 owns check.mjs and check.test.mjs, Cargo.toml's members and its lints and libm comments
  (carry 29), Cargo.lock, crates/hash-wasm, crates/lab (npt-lab's `hash`, carry 66's probe change and its
  clippy.toml), and two manifests' comment lines only: crates/proto/Cargo.toml's bites comment (carry 41) and
  crates/client-wasm/Cargo.toml's lints comment (the exceptions now three) (the advisor's U4D2-Q9 ruling);".

**U4D2-Q10 (npt-lab's clippy.toml): (a).** §4.11 and my Q9 (a) ruling give npt-lab both bans (plan :393-395,
:1002-1005); U4d2 is crates/lab's only wave-2 editor and adds the comparer of a determinism check, and lab/src holds no
`HashMap`, `HashSet`, `Instant` or `SystemTime` today (notes F12; the check's C9 row). The workspace's
`iter_over_hash_type = "deny"` (Cargo.toml) already refuses iterating a hash map in every crate; the type ban is the
stricter half. server/clippy.toml's missing type bans are block 8's.
- Plan, §5 U4d2 Builds: replace "on the two legs' per-tick server hashes (the advisor's condition 2)." with "on the two
  legs' per-tick server hashes (the advisor's condition 2). npt-lab gains its clippy.toml: §4.11's clock ban and Q9
  (a)'s HashMap and HashSet type bans (the advisor's U4D2-Q10 ruling)."

**U4D2-Q11 (h2's shown failure): (b).** Plant (1) shows the comparison against the file can fail; only plant (2) shows
the check catches what T22 exists for: native and Wasm differ while native and native agree (A2 makes that one run
show both). Both are scratch, restored and sha256-checked, and change no committed npt-sim byte, so carries 67-72 stay
out of reach.
- Plan, §5 U4d2 Bites: replace "an export not on the loader's list (the advisor's condition 7 (b))." with "an export not
  on the loader's list (the advisor's condition 7 (b)); and the vectors' shown failures: a word of the committed
  vector file changed, and a length written as `usize`'s bytes in npt-sim's walk `write_state` (ADR 0004:1142's
  word-size bite), both scratch plants, restored (the advisor's U4D2-Q11 ruling)."

**U4D2-Q12 (rev 1.1's two changes to the check's fixes): (a) and (a).**
- (a) TB-7's list: the drafter's reading is the code's (block 3): the four pins fail from the `UNSAFE_EXCEPTIONS` edit;
  :1099 fails only if the fixture's counts or `DEPS_CHECKED`'s "held:" start change; the steps pin is one the builder
  extends. Naming a pin as expected that does not fail would let a real failure of it pass; P4's own words also let a
  step's plan name any further pin it re-pins. A10 adds the deps fixture's `depsStep` call beside the steps pin.
- (b) TB-11's non-vacuity: the manifest-derived set (npt-sim and its shipped direct dependencies, read from the `cargo
  metadata` the deps step takes). Not `SIM_DEPENDENCIES` (check.mjs:77): the deps step holds npt-sim's shipped
  dependencies inside that list (:130-133, a subset check), not equal to it, so a lawful drop would fail a check that
  read the list. A fixed five would fail the step on a lawful lock change too; the equality across builds already
  catches a set that differs between them. A5 adds the transitive case TB-11's point needs.

**Section 8 and 7.2, for all twelve (brief words):**
- Section 8: after the heading "## 8. Questions for the advisor (U4D2-Q1 to Q12)" insert:
```
**Ruled at the advisor's check** (advisor-u4d2-brief.md, 2026-11-12; the questions below are kept as asked, for the record):

| Question | Ruling | Where it stands |
|---|---|---|
| U4D2-Q1 | (a): the plan's cites stay as approved; section 0's table is the reading | 0 |
| U4D2-Q2 | (a): plan.md §4.5's flags; wasm-opt is U7's; the plan's words changed | 2.4 (a) |
| U4D2-Q3 | (b): `begin(bite, arm_tick)`; the plan's words changed | 2.1 |
| U4D2-Q4 | (a), the read-back run from spikes/netcode-proof | 2.4 (c), 2.7 |
| U4D2-Q5 | (a): the scan's result committed as data; a catch judged at (R, T', F); none, the scratch stand-in once | 2.3, 2.4 (a), 2.7 |
| U4D2-Q6 | (a): three builds; the plan's words changed | 2.4 (b) |
| U4D2-Q7 | (a): the scan holds the signatures; the plan's words changed | 2.4 (d) |
| U4D2-Q8 | (a): the plan's names; the crate doc records what the invariant was checked against | 2.1 |
| U4D2-Q9 | (a), with crates/client-wasm/Cargo.toml's lints comment; the plan's Lanes words changed | 2.5, section 4 |
| U4D2-Q10 | (a); server/clippy.toml's type bans are the main session's, after U4e's merge | 2.5 |
| U4D2-Q11 | (b); the plan's Bites words changed | 2.6 h2, 2.7 |
| U4D2-Q12 | (a) and (a), with the check's A5 and A10 | 2.4 (b), section 4 |
```
- 7.2 item 4: replace "folded by script into the places marked "by U4D2-Qn"; any plan text a ruling allows (Q1's
  cites, Q2's "§4.5", Q3's `begin`, Q6's comparison line, Q8's names, Q10's clippy.toml line, Q11's bite line) in the
  main session's own commit, as the U4e brief ruling's L2 did." with "answered by the advisor's check
  (advisor-u4d2-brief.md, blocks 4 and 5) and folded into the places it names (done); the plan text it changes (Q2,
  Q3, Q6, Q7, Q9, Q10, Q11 and A1) is in the main session's own commit, as the U4e brief ruling's L2 did."
- 7.2 item 9: replace "next the advisor's check before the build (the card's "Next steps, in order", item 3)." with
  "the advisor's check (advisor-u4d2-brief.md) is done and folded here (rev 1.2)."
- The header: replace "Next: the main session's fold (section 7), then the advisor's check before the build (the
  card's "Next steps, in order", item 3: U4d2's builder brief, with the advisor)." with "Done since: the advisor's check
  (advisor-u4d2-brief.md, APPROVE on conditions) and its fold (rev 1.2). Next: section 7.2's launch steps."

### Block 6. The placeholders, 7.2's launch steps, `<base>`, the runner, and U4e merging first (done)
**The anchors, checked against the files now:**
| Placeholder | Source and anchor | Holds now | If U4e merges first |
|---|---|---|---|
| P1 | u4s-carries.md, the heading "## U4d2 or the main session" (once, :407) to the line before the next "## " (:448); entries at line start exactly 29, 41, 42, 52, 65, 66 | yes (read) | holds: T2 adds nothing to §U4d2 and inserts no "## " heading; the section moves down (A7) |
| P2 | history.md, the line beginning "  - Builders: pass or fail comes from the exit code" (once, :292) and :293; the line before is "  USAGE-SAVINGS: (a), all three, with these limits." (:291) | yes (grep, read) | holds (by text; a newer decision above it moves it, harmlessly) |
| P3 | advisor-u4df.md (842ec092…: holds) :462-464, from "Write files with the" to "whole with Write." | yes | unaffected (closed) |
| P4 | advisor-u4df.md :465-468, from "a failure in <crate>" to "and go on.", the `<crate>` replacements per A3 | yes | unaffected |
| P5 | advisor-u4j-r2.md (7efa9410…: holds) :168-171, from "a FAIL in check.mjs's test step" to "until it is explained." | yes | unaffected |
| P8 | advisor-u4j-r2.md :298, after "From now on: " to the line's end ("plants by Edit in place, or the planted file's diff saved beside the run's output.") | yes | unaffected |
| P6 | `<base>` and its counts (7.2 item 1) | see below | `<base>` holds U4e; a counted run gives the counts |
| P7 | the runner u4d2-run.sh (7.2 item 2, with A2's `-k`) | to write | unchanged |
- With A4's words, P1-P5 and P8 each stand once outside 7.2 item 3's list, and the copy rule for wrapped spans is
  stated. 7.2 item 3's checks (anchor once, first and last words, placeholder gone, no other byte changed) then make
  each copy testable by the script's own asserts.

**`<base>`:** the spike branch's head at launch. It must hold U4d-F's merge adb1bab (carry 52: the report names it),
and it does today: adb1bab, d50396a and 28a85e7 are ancestors of the head, and spikes/netcode-proof equals 28a85e7's
(read, block 1). 1.1's `git merge-base --is-ancestor adb1bab HEAD` in the worktree makes it the builder's first
recorded fact. If spikes/ equals 28a85e7's at launch, 7.2 item 1's counts are 399 / 446 / 52 of 52 / 10 steps (the
card's L3 entry: the counted run at 28a85e7, .gate/u4e-l3/l3-check.log); otherwise a counted run through main-run.sh
gives them (the shared target, the lock exclusive: main-run.sh:13-15, :19-20, read).

**The runner:** u4d2-run.sh as 7.2 item 2 says, with A2's `-k`. Read whole: u4e-run.sh's worktree and log checks
(:26-33), lock (:38-39), idle wait (:40-48), 4 GB check (:49-57), touch first and every 2 s (:58-64), the sequence
(:73-88) and provenance (:91-97) are what section 4 needs. One more call needs `-x` (A11 below).

**A11, Low: two kinds of run that give a result are missing from rule 2's `-x` list.** h7 (ii)'s `#[ignore]` test and
h7 (iii)'s probe runs give carry 66's evidence; a shared-lock run could meet U4e's copy's npt-lab test binary (U4e's
`cargo test --workspace` builds npt-lab from its own copy, without carry 66's test). Section 5 item 6 runs them
counted, but 2.6 does not say so.
- Section 4 rule 2: replace "each bite run, the std-sin scan, and every run with a scratch plant in place;" with "each
  bite run, the std-sin scan, h7 (ii) and (iii), every run whose output your report gives as a result, and every run
  with a scratch plant in place;". (Checked: the phrase occurs once.)

**If U4e merges first** (before launch, or while U4d2 builds): what changes, and what the brief must say.
- Before launch: `<base>` holds U4e's merge commit; 7.2 item 1's counted run gives the counts (R20 superseded); T2
  moves the carries lines the brief cites (A7: re-checked under 7.2 item 8, P1 unaffected); T6 changes the plan's
  sha256 and three §4.1 rows in place, moving no line the brief cites (7.2 item 8 re-stamps). The step count at
  `<base>` stays 10 (U4e adds no step). U4e's room and client clippy.toml then hold the hash-map bans (wip/u4e.patch);
  R16 still holds for lab and server.
- 1.2's sentence on U4e then becomes history. Brief words, used only if U4e has merged before launch: replace "U4d2 runs
  in wave 2 beside U4e's lane (plan :863-867; e-ruling L4, :335-336): two lanes, no shared file (U4e's patch touches
  crates/room, crates/client and crates/server only: wip/u4e.patch's file list; e-r1 R9 M1, :445-448)." with "U4e
  merged at `<U4e's merge sha>` before your launch (its merge commit's spikes/ files are under crates/room,
  crates/client and crates/server only, none in your lane), so `<base>` holds it and its counts include it." (The
  phrase occurs once; checked.)
- While U4d2 builds (the likely order: U4e's fix pass and round 2 are ahead): nothing in U4d2's worktree changes. The
  brief must say so to the builder. Brief words: in 1.2, after "Reviewers count toward the three-agent limit (plan
  :872-873)." add " If U4e merges while you build, your worktree does not change: never merge, pull or rebase; your
  counts stay deltas on `<base>`, and the main session applies your work onto the merged head and recomputes the
  counts (plan :870-871)." (The anchor occurs once; checked.)
- My U4e round-1 ruling's M5 says "All 10 steps pass" for U4e's merge. If U4d2 merges first, M5 reads "all 11 steps
  pass", the counts are U4d2's merged counts plus U4e's (as M5 already says), and the pre-run builds also hold the
  four release builds of section 4 rule 3 (their npt-lab and npt-hash-wasm lines naming the main folder), since the
  `wasm` step then builds U4e's crates in release inside npt-lab. That is my reading of my own M5; it changes no other
  M-condition.

### Block 7. The lanes beside U4e: are the merge order and W1-TARGET's lock enough? (done)
**Files: disjoint.** U4e's fix pass and round 2 hold crates/room, crates/server (not schedule.rs) and crates/client
(e-r1 R8's bounds: "no manifest, no Cargo.lock, no npt-proto, npt-sim, npt-link or npt-lab file"); wip/u4e.patch's 23
files are all there (read). U4d2 holds check.mjs, check.test.mjs, the workspace Cargo.toml's members and comments,
Cargo.lock's one entry, crates/hash-wasm, crates/lab, and two manifests' comment lines (proto, client-wasm: Q9). No
file is in both. U4d2's scratch plants in crates/sim (2.7's h2 and std-sin rows) live in its own worktree only and are
restored before its next call.

**The shared target: enough, on the rules as written.** Both lanes build the same package names into one folder, so
each can replace the other's artifacts (W1-TARGET's finding). The lock makes every claim safe: a counted run, a bite
run, a plant run and (A11) every run whose output is a result are `-x`, which no other lane's call can enter, and each
starts by touching its own crates/, so cargo rebuilds that copy's sources; the provenance lines name the copy's path.
An ordinary `-s` build may meet the other lane's artifacts, but it proves nothing and is never reported. Two cases
checked:
- U4d2's plant in npt-sim compiles into the shared target; U4e's next call touches its own crates/ (npt-sim's sources
  included) first, so its npt-sim is rebuilt from its own, unplanted copy.
- U4e's copy builds npt-lab from its own copy under `cargo test --workspace` (without carry 66's test); U4d2's h7
  results are therefore `-x` (A11), and a libtest summary of "0 passed" there would show the wrong binary ran.
- Cargo.lock and members differ between the copies (U4d2's has npt-hash-wasm); each copy's `--locked` reads its own
  lock, and an artifact's hash does not depend on the lock's other entries, so neither lane's lock check meets the
  other's.

**The merge order: enough, with the counts recomputed.** One lane at a time (plan :910-913), whichever is approved
first; the second's merge recomputes the counts on the merged tree (plan :870-871). U4e is ahead (its fix pass runs;
round 2 follows), so U4e merging during U4d2's build is the likely order; block 6 gives the brief's words for it, and
U4d2's own merge conditions come with my ruling on its review (U4e's M1-M9 form: the base still fits outside U4d2's
lane, the bytes merged are the bytes reviewed, check.mjs counted on the merged tree with 11 steps).

**One more limit, the three-agent one.** Today the session runs U4e's fix-pass builder and this check. Once U4d2's
builder launches, two agent slots are taken (U4e's builder, or its round-2 seats, and U4d2's builder), so U4e's round 2
cannot run my R8's batch 1 of three seats at once. Ruling: while U4d2's builder runs, U4e's round-2 seats run at most
two at a time (batch 1: netcode-architect and security-auditor; batch 2: engine-architect and the test-engineer;
batch 3: rust-reviewer), still one round (my role file's "Review budget": a round in batches is one round). U4d2's own
review round later runs the same way.

**What the brief must say about U4e's merge:** block 6's words (1.2's added sentence for the builder; 1.2's
replacement if U4e merged before launch; A7's carries re-check). Nothing else: the builder never merges, pulls or
rebases, and 7.2 item 5 already states the counts' rule.

**A12, Medium: the panel has no security seat, though U4d2 widens the spike's `unsafe` guard.** U4d2 adds the third
`unsafe_code` exception and changes the scan that holds the other two (`unsafeProblems`, `UNSAFE_EXCEPTIONS`,
`EXCEPTION_LEVELS`), and adds the rustflags read-back to the guard against lint-lowering flags. CLAUDE.md step 5 names
security-auditor for "deps" changes, and U3c's precedent is exact: the security seat reviewed npt-tcpinfo's exception
and check.mjs's scan and found that the scan missed relaxations rustc accepts (u3c-review-security.md S1). The plan's
U4d2 panel (:701-702) omits it. Fix: security-auditor joins U4d2's review round (read only, xhigh, counted toward the
limit). Its words are folded into A1's two Reviewers replacements (block 4, edited to carry them).

### Block 8. The main session's points (task item 6) (done)
Resumed after a usage cut: blocks 1-7 stand as written; the head is d526f91 (autosaves and notes since f94b078; the
brief, plan, plan.md, carries, df-ruling, j-r2 and e-r1 stamps re-hashed: all as block 1 gives them; spikes/netcode-proof
still equals 28a85e7).

**(a) The card's stamp of the plan: confirmed, and already corrected.** `git show 1e8c188^:` of u4-build-plan.md hashes
to d928358b…, the head's to 686a583c…; 1e8c188 (an autosave, "terminated") holds one changed line, L3's §4.1 row
(`git show --stat`: one file, 1+ 1-). The card's resume line (:16-17) now reads "686a583c…; it read d928358b… before
1e8c188", and the brief's section 0 stamps 686a583c… (:53). Nothing in the brief rested on the old stamp. The two other
files that name d928358b (u4d2-brief-notes.md, u4df-packet.md) are records of their day: no change. The lesson is in
the card already (next step 1: stop the saver before an edit toward a commit that must hold both).

**A13, Medium: npt-server's clippy.toml lacks the type bans my Q9 (a) ruling gave it; the gap is the plan's text.**
- What: the plan's Q9 (a) (:1002-1005), which I ruled (advisor-u4.md:207), reads "The HashMap and HashSet type bans in
  room, client, server, lab and bots; the clock bans in server, lab and bots". §4.11 (:393-395) wrote it narrower: the
  type bans join room's and client's files, and the three new files (server, lab, bots) get "the clock ban". U4b's
  Builds (:560: "crates/server/clippy.toml with the std and tokio clock bans (§4.11)") and U4b's file followed §4.11:
  crates/server/clippy.toml (read) bans the clocks only and cites §4.11 in its header. U4b did what the plan said.
- Reach: small. The workspace denies `iter_over_hash_type` for every crate (Cargo.toml's [workspace.lints.clippy],
  read), which refuses the order hazard itself; crates/server holds no HashMap or HashSet (src and tests: grep, none),
  nor do U4e's patch's added lines outside the room's and client's new bans (grep). The type ban is the stricter half
  Q9 (a) chose. npt-lab gets both bans by U4D2-Q10; npt-bots has no clippy.toml yet (a three-line library stub; its U4
  half, `Behaviour`, is U4r's: plan :746).
- Why neither U4d2 nor U4e: crates/server is U4e's lane until U4e merges (Lanes; e-r1 R8's bounds), its fix pass runs
  on fixed bounds, and its round 2 reviews what changed, which these two lines are not. check.mjs reads no
  server/clippy.toml (`CLIPPY_TWINS` holds only the sim and proto pair: check.mjs:765), so the change touches no file of
  U4d2's.
- Plan §4.11 (the main session's commit before U4d2's launch, with the other plan edits): replace "new clippy.toml files
  for npt-server and npt-lab, and for npt-bots (all three with the clock ban," with "new clippy.toml files for npt-server
  and npt-lab, and for npt-bots (all three with the same HashMap and HashSet type bans, by Q9 (a), and with the clock
  ban," (found once in the flattened plan). U4b's Builds line stays: it records what U4b built.
- The card's "Next steps", a new item that follows U4e's merge (the main session writes it): "**After U4e's merge,
  before U4r's brief (the advisor's U4d2 brief check, A13):** crates/server/clippy.toml gains the HashMap and HashSet
  type bans in the form U4e's room and client files use, and its header names them (Q9 (a); plan §4.11); proof: a
  scratch `std::collections::HashMap` in crates/server/src refused by clippy by name under the runner (-x), restored by
  Edit and its sha256 checked; then check.mjs counted, every step PASS and the counts unchanged; its own commit, which
  the advisor's next ruling on the spike branch checks. U4r's brief gives npt-bots its clippy.toml with both bans
  (§4.11)."
- The brief: nothing more. U4D2-Q10's row in section 8's table (block 5) already says the server's bans are the main
  session's, after U4e's merge.

**(c) crates/client-wasm/Cargo.toml:9-13 still says "two": settled by U4D2-Q9 (block 5).** Read now: "(one of the
workspace's two unsafe_code exceptions, with npt-tcpinfo's two functions; …)". The comment joins U4d2's lane, comment
lines only, so the third exception and every text that counts the exceptions change in one commit; the brief's return
item 9 is reworded to match (block 5).

**A14, Low: the drafter's process slips; recorded, nothing redone.**
- What: two `cp` copies of rev 1 (the brief and the notes) into the session scratchpad, and one `rustc --print cfg …
  2>` redirect into a scratch file there (notes :243-246). The sentence the drafter worked under (T4 item 1:
  "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; …)") is broken by both:
  the redirect by its words, the copies by its first clause.
- Why no redo: neither wrote in the repo or in .gate/ (the working tree was clean at my start; git shows only the
  brief, the notes, the card and this file changing since f94b078); the brief I checked is the file whose sha256 the
  task names, read whole; and the one fact the redirect carried (rustc's warning for a cross-target flag, notes F21) I
  re-ran myself (block 10, where it proves incomplete: A15; the slip did not cause that, the reading did). Rev 1 was in
  git (3032b88 holds e5869dbf…, the notes' own stamp), so `git show` would have served.
- Brief, section 4's files bullet: after "then restored and the sha256 checked against the clean file." insert " A
  `cp`, `mv` or `tee`, and a `>` or `2>` into a file, are shell writes too, in any folder, the session scratchpad
  included (this brief's drafter slipped so: u4d2-brief-notes.md, "Own slips"): read an older revision with `git show
  <sha>:<path>`, and let the runner's log hold a command's output." (The anchor occurs once.)

### Block 9. An erratum to my block 4 (done)
**A7 undercounts T2's carries.** Read again (advisor-u4e-r1.md :504-597): T2 inserts carries 73-81, not 73-77: 73-77
after entry 69 at §U4r's end (above :323, so above the recordings section), 78 and 79 in §U4g, 80 in §U4f, 81 in §U4h,
every one above §U4d2 (:407). The drafter's notes F19 had it right. A7's point holds (both sections the brief cites
move; P1, found by its heading, does not break), but its words change, and they are the ones the fold uses:
- In A7's replacement for 7.2 item 8, read "(T2 inserts carries 73-81 above §U4d2, 73-77 of them above the recordings
  section; T6 moves no plan line)" in place of "(T2 inserts carries 73-77 above the recordings section and §U4d2; T6
  moves no plan line)". Everything else in A7 stands.
- So the next free carry number once T2 has landed is 82 (block 8's A13 uses none: its item lives in the card).

### Block 10. The read-back's facts, re-run (`rustc --print`, allowed in this run) (done)
Block 3 left TB-3's facts (R24, notes F21) as the drafter's and the test-engineer's runs. This run allows `rustc
--print`, so I ran them from spikes/netcode-proof (rust-toolchain.toml pins 1.99.1; `rustc --version`: "rustc 1.99.1
(58cf8f9c3 2026-10-13)"), output to the terminal only, nothing built:
| Given | x86_64-unknown-linux-gnu lists | wasm32-unknown-unknown lists | exit |
|---|---|---|---|
| nothing | fxsr, sse, sse2 | bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext | 0 |
| `-C target-cpu=haswell` | `fma` among them | **no target_feature at all**, warning "not a recognized processor" | 0 |
| `-C target-cpu=x86-64` / `native` | not run here (F21: x86-64 lists fxsr, sse, sse2) | **none at all**, the same warning (native: one warning per host feature) | 0 |
| `-C target-feature=-fma` / `+sse2` | fxsr, sse, sse2 | unchanged, warning "unknown and unstable feature" | 0 |
| `-C target-cpu=bleeding-edge` / `mvp` | **sse, sse2 only** (fxsr gone), warning "target feature `x87` must be enabled …" | bleeding-edge: adds extended-const, relaxed-simd, simd128, tail-call | 0 |
| `-C target-feature=+simd128` | unchanged, warning "unknown and unstable feature" | not run (the name scan's case) | 0 |
- TB-3's answer holds: `haswell` lists fma on x86-64 and `bleeding-edge` lists simd128 and relaxed-simd on wasm32, and
  the name scan passes both, so each live run can fail only by the read-back. The clean lists are F1's.
- **A15, Medium: a CPU name of the other target's does not "only warn"; it empties wasm32's list.** The brief says "A
  flag meant for the other target only warns and exits 0 … so giving every flag to both read-backs adds no false
  failure" (2.4 (c) (ii)), and R24 "a cross-target flag only warns, exit 0". True for target features; false for CPU
  names. With the brief's own non-vacuity rule (the wasm32 read-back must list `sign-ext`), an x86-64 CPU name in any
  source fails the step on wasm32, even in `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, which cargo gives to
  x86-64 builds alone; and TB-3's haswell run, through `RUSTFLAGS`, fails the wasm32 read-back too, a second FAIL line
  the brief does not expect. Medium, not High: no wrong pass (it fails closed), and the only flags it refuses are CPU
  names a lawful build never needs (plan.md §4.5's baseline x86-64 is the default CPU; wasm32's is the default too);
  but a builder meeting an unexplained wasm32 FAIL could loosen the non-vacuity rule to pass it, and the controls'
  "(whose read-backs list no fma)" invites a whole-step pass the step will not give. My Q4 (a) ruling stands: this is
  the superset's cost, it refuses no needed flag, and (b) would still be cargo's rules re-implemented.
- Brief, 2.4 (c) (ii): replace "A flag meant for the other target only warns and exits 0 (measured: `-C
  target-cpu=bleeding-edge` on x86-64 and `-C target-cpu=haswell` on wasm32; notes F21), so giving every flag to both
  read-backs adds no false failure." with "A flag meant for the other target warns and exits 0, and a target feature of
  the other target's (`-C target-feature=-fma` or `+sse2` on wasm32, `+simd128` on x86-64) leaves the list as it is;
  but a CPU name of the other target's changes it: given `-C target-cpu=haswell`, `x86-64` or `native`, the wasm32
  read-back lists no target_feature at all, and given `-C target-cpu=bleeding-edge` or `mvp`, the x86-64 one lists sse
  and sse2 only, warning that x87 must be enabled (rustc 1.99.1, run by the advisor's U4d2 brief check, A15; notes F21
  saw the warning and the exit code). So an x86-64 CPU name in any source fails the step on the wasm32 read-back's
  non-vacuity below, even in `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, which cargo gives to x86-64 builds
  alone: the superset's cost under U4D2-Q4 (a), fail-closed, and it refuses no flag a lawful build needs (plan.md
  §4.5's baseline x86-64 is the default CPU, so a CPU name adds nothing). That FAIL names the flags the empty read-back
  was given."
- Brief, 2.4 (c)'s self-test controls: replace "`-C target-cpu=x86-64` (whose read-backs list no fma: notes F21);" with
  "`-C target-cpu=x86-64` (whose x86-64 read-backs list no fma: notes F21; the name scan passes all three, and the whole
  step passes the first two and FAILs the third on the wasm32 read-back's non-vacuity, as A15's sentence above says);".
- Brief, 2.7's flags row: replace "`RUSTFLAGS="-C target-cpu=haswell"` (the read-back: x86-64 lists `fma`, notes F21;
  the name scan passes it);" with "`RUSTFLAGS="-C target-cpu=haswell"` (the read-back: x86-64 lists `fma`, notes F21,
  and the wasm32 read-back lists nothing, so it fails its non-vacuity too: A15; the name scan passes it);".
- Brief, R24: replace "a cross-target flag only warns, exit 0 | notes F21 |" with "a cross-target flag warns, exit 0; a
  cross-target CPU name empties wasm32's list and cuts x86-64's to sse, sse2 | notes F21; the advisor's U4d2 brief
  check, A15 |".
- Each old phrase occurs once in the flattened brief (counted, read only). Section 8's U4D2-Q4 text stays as asked
  (block 5's table is the ruling).

### Block 11. The verdict (done)
**APPROVE for launch, on conditions L1 to L5.** The build may start once L1 to L4 are done; L5 is the launch.
- Why not REVISE: rev 1.1 answers all twenty findings of the test-engineer's check, the three Highs as asked and TB-7
  and TB-11 with changes that hold (block 3; U4D2-Q12, block 5); every item of §5 U4d2 and of my conditions 2, 7, 8
  and 10 (b) is in the brief and ends in an exit code and a runner's own summary line (block 2); the carries routed to
  U4d2 each have a home, and 67, 71 and 72 rightly stay out (blocks 2 and 6); nothing goes beyond the plan. My
  findings are five Mediums (A1, A2, A12, A13, A15) and ten Lows (A3 to A11, A14), none High and no security defect
  (A12 adds a seat; it finds no defect), each with exact words a script can fold; A13 lies outside the brief (plan text,
  and a main-session item after U4e's merge). The twelve questions are ruled with exact words (block 5). Nothing needs
  a second check unless the fold goes beyond those words.
- Risk tier 1 (spike code on PR #40's branch; check.mjs is the spike's own check, not one of tools/' gate files); this
  ruling is a notes-folder text. U4d2's panel: rust-reviewer, engine-architect, test-engineer and infra-engineer (the
  plan's) and security-auditor (A12), at xhigh, counted toward the three-agent limit (condition 10 (c)): five seats in
  batches of at most three, two while U4e's round 2 runs (block 7), one round.

**Conditions (the main session's):**
- **L1. Fold this ruling into the brief by script (rev 1.2):** block 4's A1 to A10 (A7 as block 9 corrects it; A1
  carrying A12's seat), block 5's words for U4D2-Q1 to Q12 with section 8's table, the header and 7.2 items 4 and 9,
  block 6's A11 and its 1.2 sentence for the builder (1.2's replacement only if U4e merged before launch), block 8's
  A14 and block 10's A15; and, in block 5's replacement for 7.2 item 4, "(advisor-u4d2-brief.md, blocks 4, 5, 8, 9 and
  10)" for "(advisor-u4d2-brief.md, blocks 4 and 5)" and "(Q2, Q3, Q6, Q7, Q9, Q10, Q11, A1 and A13)" for "(Q2, Q3, Q6,
  Q7, Q9, Q10, Q11 and A1)"; and, in 7.2 item 8, "if the plan's moved (the advisor's U4d2 brief check's plan edits, L2,
  move lines below §4.11; U4e's merge applies T6 to §4.1 in place)," for "if the plan's moved (U4e's merge applies T6
  to §4.1),". Each old phrase is matched with its line breaks and the next line's indent read as one space; I counted
  each in rev 1.1: once each (A1's ordered pair aside: block 4), and no two replacements share a sentence's anchor. The
  fold is recorded under the brief's title. If a phrase fails to match, or the fold must change any other word, it
  comes back to me; otherwise there is no second check.
- **L2. The plan text, in the main session's own commit, after U4e's fix pass returns and before U4d2's launch:**
  u4-build-plan.md's edits for U4D2-Q2, Q3, Q6, Q7, Q9 (the Lanes line), Q10 and Q11 (block 5), A1 with A12's seat
  (block 4) and A13's §4.11 (block 8), each old phrase found once (counted), allowed by my condition 12 through this
  ruling. They lengthen §4.11 and §5 U4d2 and so move every later plan line, §5 U4e's included: none lands while an
  agent that reads the plan by line is running (U4e's fix pass works from u4e-brief.md's 41 plan cites). From that
  commit on, u4e-brief.md's, e-r1's and every earlier ruling's plan cites read at 686a583c (`git show
  1e8c188:.checkpoints/netcode-proof-test/u4-build-plan.md`), and U4e's round-2 briefs say so. Then 7.2 item 8 runs on
  the brief: section 0's plan stamp re-set, and every `plan :N` cite whose line moved re-pointed by script (the cited
  line's text found at its new number; a cite inside an edited paragraph takes that paragraph's new span), each
  asserted. 7.2 item 6 (carry 65) as the brief says.
- **L3. The placeholders and the runner (7.2 items 1 to 3, as L1 leaves them):** P1 to P5 and P8 by their anchors and
  sha256 asserts (block 6: each holds now); P6, `<base>`: the spike branch's head at launch, with `git merge-base
  --is-ancestor adb1bab <base>` exit 0 recorded, and its counts 399 / 446 / 52 of 52 / 10 steps if `git diff --stat
  28a85e7 <base> -- spikes/netcode-proof` is empty, else from a counted run through main-run.sh; P7, u4d2-run.sh per
  A2: its diff against u4e-run.sh (669f7c65…) saved beside it, showing only the log folder, the header and `-k`; and
  `-k` shown once before launch in U4d2's worktree, in the runner's shared mode (no `-x`): `false :: true` with `-k`
  exits 1 with both exit codes in its log, and without `-k` its log holds the first only (both logs under
  .gate/u4d2/).
- **L4. U4e's order, said to the builder (block 6):** 1.2's added sentence ("If U4e merges while you build, your
  worktree does not change: …"); if U4e merged before launch, 1.2's replacement with its merge sha, `<base>` holding
  it, the counts from a counted run, and 7.2 item 8 run for T2's carries 73-81 (block 9) and T6.
- **L5. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in its own worktree from `<base>`, under
  W1-TARGET's lock; at most three agents at a time counting reviewers, so U4e's round-2 seats run at most two at a time
  while it builds (block 7). A13's server item waits for U4e's merge and is not U4d2's.

**For later briefs and rulings (no action now):**
- U4r's brief: npt-bots' clippy.toml with both bans (A13; §4.11 as L2 leaves it).
- U4d2's review round: the infra-engineer judges the step's cold and warm times against 5 minutes, with no CI figure
  (A1); the security seat reads the third exception and the scans (A12); my merge conditions will follow U4e's M1 to
  M9 form, with 11 steps.
- U4e's merge: if U4d2 merges first, e-r1's M5 reads "all 11 steps pass", and its pre-run builds hold section 4 rule
  3's four release builds (block 6).

## What I checked, and what I did not
- **Checked, by reading:** block 1's files and code cites; every old phrase this ruling replaces, counted in the
  flattened brief (blocks 4, 5, 6, 8, 10 and L1: once each, A1's ordered pair aside) or plan (once each), and the
  anchors that share an item checked for overlap (none); P1 to P5 and P8's anchors and the two closed rulings' sha256
  (block 6); the plan's stamp history by git (block 8); every crate's clippy.toml, crates/server and crates/lab for
  HashMap and HashSet, wip/u4e.patch's added lines and `CLIPPY_TWINS` (block 8); T2's carry numbers (block 9).
- **Checked, by running (read only, nothing written):** `rustc --print cfg` for both targets, clean and with nine
  flags (block 10); the sha256 of every stamped file; git ancestry and diffs.
- **Not checked:** I built, tested, planted and timed nothing (the shared target is U4e's fix pass's, and this run
  allows no build). So the 27,000-tick equality, the Wasm legs, the hash-map bite's outcome on wasm32, the std-sin
  scan's result, the cold and warm times, the Wasm size and notes F4's `cargo tree` output (the features comparison's
  inputs) are the builder's to show, each by an exit code. I fetched no web page (F20's std 1.99.1 source: the step
  decides from the digests it prints, block 3), did not re-run notes F5's `nm -D` (Q8 rests first on the native build
  never being loaded as a library), and ran neither `-C target-cpu=x86-64` nor `native` on x86-64 (block 10's table
  says so). U4e's fix pass may still change wip/u4e.patch within e-r1 R8's bounds; block 7's lane reading assumes it
  keeps them.

## Decisions needed (Mustafa)
None. Every point here is a technical or test choice inside the documented vision, a route between lanes, or the text
of my own ruling (Q9 (a)'s type bans put back into the plan's §4.11): mine to decide. No word or decision of
Mustafa's changes, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)
