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
