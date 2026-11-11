# Advisor: U4j (judge and marks change), round 2 rulings and the merge

Date: 2026-11-11. Seat: day session, advisor agent (rules read from origin/main:.claude/agents/advisor.md).
Scope: U4j's fix pass in /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (base 411eeee, 66
uncommitted paths; saved as wip/u4j.patch at 4ad80a9, 64 spike files). Round 2's notes: u4j-round2-netcode.md,
u4j-round2-security.md, u4j-round2-test.md (runs in u4j-round2-test/). My round 1: advisor-u4j.md (FX, FU, R2-R5).
I rule; I edit nothing else, build nothing, run nothing of the project, commit nothing.

## Plan
1. Read my role (origin/main), advisor-u4j.md whole, the three round-2 notes whole, the fixer's "Fix pass, round 1"
   and its handback (workflow run wf_0906017a-a9b's result), the checkpoint card's U4j block, u4s-carries.md whole,
   Mustafa's 2026-11-11 words (history.md :341).
2. Check by reading (no runs): the worktree's 66 paths and HEAD; that crates/judge, crates/log and marks/ did not
   change on the branch since 411eeee; the F1 path in practice.rs; the lines each Low names; check.mjs's failure
   output; the fixer's saved check.mjs run 2; the public items U4j adds; each text anchor's uniqueness.
3. B1 the merge: verdict and conditions. B2 the three Mediums. B3 the Lows. B4 the fixer's points. B5 what goes into
   the notes and the report at the merge. B6 the process slips. B7 the exact words (text commit, carries, plan text).
4. What I checked and what I did not; Decisions needed (Mustafa); For Mustafa.

## Progress
- [x] 1 read  - [x] 2 checked (facts cited in each block)  - [x] B1  - [x] B2  - [x] B3  - [x] B4  - [x] B5
- [x] B6  - [ ] B7  - [ ] the end sections

## B1. The merge: APPROVE, U4j merges into the spike branch now, on conditions M1 to M9
**Why now.** Round 2 was the last ordinary round, and it leaves nothing that opens a round 3:
- F1 (High, the FAR BETTER cap hole) is CLOSED by all three seats, each by its own route:
  - security: every path to `Ok` in `find` tabled (u4j-round2-security.md Block 1): f <= tolerance <= 1.10 on each,
    and no log input can raise it;
  - test: probe A INVALID, a 200-path grid whose largest X in force is 737 ms (= 1.10 x 670 ms), and each of my
    plants caught by name (runs 1 to 4);
  - netcode: README §4's paragraph checked against my R6, R8 (v) and Mustafa's WS-BAR refinement and NETEM-BAR (a).
  - I read the path myself (practice.rs:497-607 in the worktree): the record's line must equal marks.json's byte for
    byte; the bound is parsed from marks.json's line only; class_ok caps s at the tolerance for IN TOLERANCE and
    BEYOND; FAR BETTER takes f = 1; the guard refuses f > tolerance. It holds.
- All 14 fixes are closed or closed as routed. No reviewer finds a new Critical or High.
- Mustafa's words (2026-11-11, history.md:341): "U4j's High: carry on as the review budget says. If it is still open
  after round 3, stop and bring it to me." It is closed after round 2, so nothing goes to him.
- An erratum to my round-1 R4 (step 2, plant 3): "caught by `u4j_ws_far_better_past_the_cap_is_invalid`" does not
  hold as written while FX-1's guard stands (the test-engineer's run 3). Plant 3 is caught by name all the same, by
  the pin's tests and the band's tests; the named test bites once the guard goes too (run 4, round 1's PASS at
  1.273 s back). So every layer of the cap but the guard has its own test, and the guard (unreachable by design) is
  shown by runs 3 and 4 side by side. F1's proof stands.

**The conditions** (the main session; each result goes in u4j-build.md's merge section):
- **M1, the copy is the one reviewed.** Before the snapshot: the worktree's HEAD is still 411eeeed50d8…, and
  `sha256sum -c` of u4j-round2-test/hashes-start.sha256, run from the worktree's root, gives 66 OK. Any other result
  stops the merge and comes back to me.
- **M2, the base still fits.** `git diff --quiet 411eeee HEAD -- spikes/netcode-proof/crates/judge
  spikes/netcode-proof/crates/log spikes/netcode-proof/marks` on the branch head (true at 123ccd0: I checked; nothing
  under spikes/ changed since 39f4290 either). If false, stop: the patch's base no longer matches.
- **M3, the snapshot and the apply,** as the card says: the saver stopped first; `python3 -I
  .checkpoints/netcode-proof-test/worktree_patch.py .claude/worktrees/agent-a3b8dfeb01ed52b50
  .checkpoints/netcode-proof-test/wip/u4j.patch u4j-` lists 64 files, all under spikes/netcode-proof/;
  `git apply --check`, then `git apply`.
- **M4, the merged bytes are the reviewed bytes.** From the main folder's root, `sha256sum -c` of the 64
  `spikes/` lines of hashes-start.sha256 gives 64 OK. This is the proof that round 2's verdicts and runs apply to
  what lands; a patch that applies but differs stops the merge.
- **M5, check.mjs counted on the merged tree** (the shared-target rules: idle wait, touch loop, the pre-run
  "Compiling npt-judge" line from the main folder's path): all 10 steps pass, with measured 396 (365 + 31), bites 443
  (412 + 31), self 52 of 52, marks 81 read and checked with the ADR rows unchanged, redact 0 findings. Any other
  count is explained in the notes before the commit.
- **M6, a failing test is never retried into green unnamed** (TE2-5's rule, B2.3, applied here):
  - if a test fails, the main session re-runs that build's failing crate at once through the runner with
    `--no-fail-fast`, its whole output saved under .gate/, and writes the failing test's name and panic in the notes;
  - a failure in npt-judge or npt-log, or in a test that reads U4j's files (npt-net's SERIES-row tests include
    marks/SERIES.md at compile time: crates/net/src/l2/names.rs:143, delay_names.rs:449, socket_tests.rs:390; check.mjs's
    marks and self steps read marks.json), stops the merge and comes to me;
  - any other failure is recorded under TE2-5 and the merge goes ahead only on a clean counted run, the earlier
    failure named in the notes.
- **M7, the gate:** `node tools/gate.mjs` PASS before each commit (CLAUDE.md).
- **M8, the commits** (both in the same push):
  1. The merge commit, "U4j: the judge and marks change (npt-judge, npt-log, marks/), merged after review round 2":
     the 64 spike files exactly as applied, `git rm` of wip/u4j.patch, and the plan and notes text of B5 and B7.3 to
     B7.5 (all under .checkpoints/). Its spikes/ part is the reviewed bytes and nothing else.
  2. The text commit, "U4j: round 2's wording (the advisor's U4j round-2 ruling)": B7.1's words only, applied by
     script (each old text found exactly once, each new text present after, the commit's diff holding nothing else,
     prose lines at most 120 characters), then check.mjs counted again with M5's counts and the gate. These are Lows
     and one Medium's text that ride a push needed anyway (the review budget); none changes code or a pass bar.
- **M9, after the push:** the saver restarted; the card updated (U4j merged at <sha>; base counts 396 / 443 / 52 for
  the next lanes, U4e's brief's `<base>` included); then U4c's L-c8 judged re-run on U4j, as the card says.

No code or test change rides the merge: round 2's code-side findings are follow-ups (B2, B3), since below the bar
"a finding becomes a STATUS follow-up" (CLAUDE.md, review budget).

## B2. Round 2's three Mediums
Each keeps its reviewer's label (Medium). None is a security finding.

### B2.1 NC-U4J-R2-N1, when a tick is written lost under L1's reorder: a CARRY (U4e and U4r), and README §1's words
- **The finding holds.** L1 reorders datagrams on every profile where the jitter or the reorder step can
  (l1/src/lib.rs:15-23: 1 % of a flow's datagrams overtaken by the next; P0 finds nothing to swap). README §1's
  `decoded_hash` item (worktree README.md:55-65) says no hash is dropped and a tick none of whose groups arrived is
  lost, but not when a writer may decide a tick is lost, nor what a late group of an older tick is. Either wrong
  reading fails a correct build (every P3 run INVALID through `crash`, or a FAIL through the sentinel); neither gives a
  false pass.
- **The rule (the reviewer's sentence, taken with two additions):**
  - a tick is written lost only once none of its groups can still decode: once it has left the client's frame ring
    (at `npt_sim::ACK_RING_TICKS` ticks old: plan §4.5, the client keeps its frames for the AckRing's ages), or at
    the stop (on L1 the runner may decide it there). The reviewer's "none of its groups can still decode", made a
    point a writer can test;
  - a group of a tick older than one already decoded is decoded and hashed under its own tick like any other, or, if
    the client drops it as stale, it is never acknowledged and that tick is lost (`null`, `tick.s.snap_incomplete` 1),
    never the sentinel. The addition "never acknowledged": plan §4.5 lets the server take as baseline only a tick
    whose every group the client acknowledged, and the client must hold every baseline the server can pick; a group
    dropped but acknowledged would end in the sentinel ("no baseline") later, a correct build failing;
  - the sentinel stays for §4.6's causes (no baseline, a bad header, a size or decode error).
- **Why this does not loosen -17, and what keeps -58's P3 honest.**
  - ADR 0001's -17 row (:1605) asks that "100 % of decoded snapshots rebuild the server's quantised state"; a group
    the client drops undecoded is not a decoded snapshot, and the tick stays resolved (lost, not compared) under
    FX-10's coverage rule. On L1 P0 nothing reorders, so R2.4's setup check (snap_incomplete sums to 0 on L1 P0)
    still catches a client that marks ticks lost without cause.
  - But -58 runs "at P3 (so reordering crosses the wrap)" (ADR 0001:1607). A client that drops every late group never
    decodes the late side of a reordered wrap, so those snapshots go uncompared. Both behaviours are legitimate client
    designs, and forcing a decode could demand baselines older than the frame ring keeps (a late group right after
    a B1 burst), so I do not forbid the drop. My lean is the decode: it keeps -58's P3 comparing the late snapshots
    its row exists for.
  - So the choice is made visible: U4e's report says which its client does; a client that drops counts each drop
    per client under a name with its SERIES.md row (carry 54's pattern), and U4r's -58 P3 contract run reports that
    count beside `decoded_hash_compared_count`.
  - If U4e's client drops, the readings list (B5) gains one entry; if it decodes, none is needed.
- **N6's two writer rules fold in** (the reviewer's request): the head (every client admitted, and its first snapshot
  resolved, in the warm-up) and the stop (the room sends no snapshot of a tick it does not log).
- **Where the words go:**
  - README §1's `server_hash` and `decoded_hash` items gain them in the text commit (B7.1, T1 and T2), so the
    contract both sides read holds one rule. This changes no judge code and no pass bar.
  - u4s-carries.md, §U4e: entry 61 (B7.2). §U4r: entry 62 (B7.2), with the proof the reviewer names (U4r's contract
    test on a short L1 P3 run with the reorder step on: no `crash` footer, `decoded_hash_mismatch_count` 0, no gap).
  - The U4e brief (rev 3, in its check now) re-copies §U4e by script before the advisor's check of the brief, since
    3.1 is word for word from u4s-carries.md.

### B2.2 TE2-1, no committed test pins the LG part of the Arm rule's key: a FOLLOW-UP (u4s-carries.md entry 64 (a))
- **The finding holds.** The test-engineer's plant ROLE (figures.rs:248's `h.machine.role == "LG" &&` dropped)
  survives every committed judge test (run 9: lib 14, cli 2, engine 43, fixtures 19). l1-aarch64-claim's only
  mismatch is an orphan, which FX-5 (c) counts on every machine, so that fixture cannot show the LG key. The code is
  right today (probe ROLE: role C, ML and MAC FAIL; LG PASSes as aarch64-only).
- **Why not in the merge:** it is a test (code) change; below the bar after round 2 it becomes a follow-up, and a
  test the main session writes after the reviews would be unreviewed evidence riding the reviewed bytes.
- **Its reach and its deadline:** no part judged today selects a non-LG aarch64 client log with an x86-64 replay
  (-17's L2 parts select LG; L1 logs carry no replay), so the gap matters only when someone next edits the judge.
  So the case is the first step of the next change to crates/judge (whoever makes it), before that change's own
  edits, and at the latest before U6 builds -19's figure (ML's Chrome judged "as it stands": ADR 0001:1609).
- **Now, in the text commit:** fixtures/README.md's l1-aarch64-claim row stops claiming what it does not show (B7.1,
  T9). Its final words come with the case (entry 64 (a)).
- The case's words and its proof: entry 64 (a), B7.2.

### B2.3 TE2-5, an npt-net test failed once and was retried until green, with no name: a FOLLOW-UP in two halves, and a rule from now on
- **The finding holds, and the name is gone for good.** The fixer's saved output of check.mjs run 2 (its
  scratchpad's check-run2.txt, 12:39 UTC) ends "bites build: 299 tests passed, 1 failed" and "error: test failed, to
  rerun pass `-p npt-net --lib`", with no test name. Why: check.mjs's `run` joins stdout before stderr
  (check.mjs:1402), and `main` prints only the last 40 lines of a failed step (:1649), so libtest's `failures:` list,
  on stdout, is above the tail; cargo stops at the first failing binary. Then npt-net's lib passed three runs alone
  and the next two check.mjs runs, and the fixer called it "a timing flake". A test that fails once in eight
  attempts (the fixer's four check.mjs runs and three runs alone, the test-engineer's one counted run) is a defect
  until its cause is known, and a later green proves nothing about it.
- **Not U4j's, and why that is shown, not assumed:** U4j touches no npt-net source. npt-net's tests do read U4j's
  marks/SERIES.md (include_str!, the three files in M6), but those are fixed `contains` checks on fixed bytes, which
  cannot fail once in eight runs of one build. So it does not hold U4j's merge (M6 keeps watch).
- **Half (a), check.mjs names every failing test:** u4s-carries.md entry 65, §U4d2 (check.mjs is U4d2's in wave 2:
  u4-build-plan.md, Lanes). The main session may do it sooner in its own commit, before U4d2's lane opens; U4d2's
  report then lists entry 65 as met.
- **Half (b), the cause:** the main session, before the next merge into the spike branch after U4j's (U4d-F's or
  U4e's): the bites build's workspace tests, as check.mjs's test step runs them but with `--no-fail-fast`
  (`cargo test --locked --workspace --all-features --no-fail-fast`), ten times through the shared-target runner, each
  run's whole output saved under .gate/te2-5/. Any failure's name and panic go into the notes, and that test's cause
  is found and fixed in a planned change of its own. If none fails in ten, that is recorded, and the rule below stays
  until a name is caught.
- **The rule, from now on, for the main session and every builder brief until entry 65 is met** (the card's "Rules
  in force" carries it as this ruling's): a FAIL in check.mjs's test step is never retried into green unnamed. The
  failing build's crate is re-run at once through the runner with `--no-fail-fast`, its whole output saved under
  .gate/, and the failing test's name and panic go into the notes, tagged TE2-5. A failure in a crate the unit
  changes, or in a test that reads its files, stops that unit's merge until it is explained.

## B3. Round 2's Lows, one line each (labels stand; none triggers a push of its own)
- **NC-U4J-R2-N2** (a same-build practice run with no readable `params.started_utc` turns a FAIL into INVALID;
  confirmed by the test-engineer's probe2): the behaviour stays, as the security-auditor argues (skipping such a run
  would let a stripped timestamp hide a first practice run from the one-per-build count; it fails closed, never
  looser); README §4 says so in the text commit (T4 (ii)). No code change.
- **NC-U4J-R2-N3 = SEC-R2-N1 = TE2-2** (the probe-A test asserts only the "ws-practice: " prefix, so it cannot say
  which layer refused): follow-up, entry 64 (b), the reasons asserted; my R4 erratum is in B1. Not a hole: each layer
  but the guard has its own test.
- **NC-U4J-R2-N4** ("f = max(1, s)" with no FAR BETTER exception in README §2.5 and -18's notes; equal in value):
  text commit, T3 and T5; practice.rs:4's module doc with the code follow-up, entry 64 (e).
- **NC-U4J-R2-N5** (SERIES's cross_arch and snap_incomplete rows, eval.rs:1715's FAIL note, behind the code): the two
  SERIES rows in the text commit (T6, T8); the FAIL note is code, entry 64 (e).
- **NC-U4J-R2-N6** (the head and the stop writer rules): folded into N1 (B2.1): README T1 and T2, entries 61 and 62.
- **SEC-R2-N2** (security, older than U4j: a run, set, role or process id may hold a control character, and
  `summary_line` prints a reason raw on stdout, so a log could print a forged summary line): follow-up FU-7, riding
  FU-3 (entry 63: the next change to npt-log; U4r's brief names it). An open security finding: PR #40's report names
  it with FU-1 to FU-5 (B5).
- **TE2-3** (the fixer's stricter FX-3 (c) reading has no test): the reading is ACCEPTED (B4.2); README §4 says it
  in the text commit (T4 (i)); its case is entry 64 (c).
- **TE2-4** (no case pins "a coverage gap before a waiting Arm mismatch"): the order is ACCEPTED (B4.3); its case is
  entry 64 (d).
- **TE2-6** (SERIES's twin_hash row still says "any role"): text commit, T7, with the words B4.1 gives.

## B4. The fixer's points (its handback, "Where I stopped short, or read something not spelled out")
### B4.1 SERIES's `twin_hash_mismatch_count` row still says "any role" (with TE2-6): the row's words change
- The code refuses a twin of any role but `twin` as a gap (figures.rs:192-203, FX-4), while the row a writer follows
  says "any role (-21's twins are role twin, -30's each other's measured runs)". -30's form is U6's (round 1's carry
  U6-a, now C-U6-9: B7.4). Until then a -30 run judged on this figure is a gap, never a pass.
- The words (text commit, T7):
  - "any role (-21's twins are role twin, -30's each other's measured runs)" becomes "role `twin` (-21's twins;
    -30's form, each other's measured runs differing in `params.transport`, comes with U6: C-U6-9)";
  - "a twin that breaks one, a run naming no twin" becomes "a twin of another role, a twin that breaks one, a run
    naming no twin".

### B4.2 The stricter reading of FX-3 (c): two practice runs in force make a bite run INVALID (TE2-3): ACCEPTED
- The case: a bite run's set holds measured runs of one build, but they have two different practice runs in force
  (one measured run started after a second practice run of the build, which carries a `second_ruling`). The code
  refuses it (practice.rs:458-470, in `find`); the test-engineer's probe BITE2 shows it on the head.
- Why accepted: FX-3 (c) judges a bite run "at its counted run's numbers"; here there are two sets of numbers. A
  smaller f could show the bite falsely, a larger one hide it, so any choice is a guess. INVALID never guesses and can
  only lower a verdict (an INVALID bite run shows no bite, so the mark cannot pass on it). The same holds when one of
  the set's measured runs has no practice run in force at all (`practice_of`'s error, which `find` passes on). The
  cost: re-run the bite after the set's measured runs share one practice run.
- README §4 says it (text commit, T4 (i)). Its case: entry 64 (c).
- A reading of my ruling, not of an ADR row or of Mustafa's words, so it goes into the notes (B5), not the readings
  PR.

### B4.3 FX-10's malformed entries and its order of outcomes (TE2-4): ACCEPTED
- A `tick.s.snap_incomplete` value other than 0 or 1 is a structural gap. README §1 names such a series "a 0/1 flag"
  and SERIES's row says "flag", so this is FX-10 (f)'s "malformed entries", checked first. The fixer added two engine
  cases for it (a list of 3 under `clients` 2; a value of 2).
- The order count > empty > gap > waiting > 0 (figures.rs:538-560). FX-10 (f) fixed the first three and put the
  waiting case after the counted mismatches; it left gap against waiting open. Gap first is right: a run with a
  coverage gap or an incomplete re-decode cannot become valid when its replay log arrives, so NOT-RUN ("wait for the
  re-decode") would send the operator after a log that cannot save the run. Both are non-pass, so the order can never
  loosen a verdict.
- `cross_arch_mismatch_count` NOT-RUN while a mismatch waits for its re-decode: accepted (it cannot be counted
  before the re-decode exists).
- Its case: entry 64 (d).

### B4.4 FX-9's plant T also raised FX-1's new guard to 1.12: ACCEPTED, the right plant
- Plant T moves "the cap" to 1.12. After FX-1 the cap is applied three times: IN TOLERANCE's s <= tolerance,
  BEYOND's, and the guard. A faithful plant moves all three. Left at 1.10, the guard would refuse T1b (s = 1.1001)
  itself, and the plant would test the guard, not FX-9's boundaries.
- The test-engineer planted it the same way (run 1): T1b, T2 and T3 failed by name, alone. Its consequence for my
  plant 3 is B1's erratum.

### B4.5 The fixer's other notes (its points 4 to 6)
- README §2.1's `ws_bound_line` row (FX-6's §2.5 words plus FX-6 (a)'s sentence) and the L2 fixtures' 2 clients:
  accepted.
- The "f = max(1, s)" sentences it left: N4, reworded in the text commit (T3, T5; practice.rs:4 in 64 (e)). README
  §2.3's "prints ... M_p, M_m, s and f" and §2.5's "the WebSocket practice record's header checks read it": true as
  they stand (FX-11 prints more, not less); left.
- Notes in u4j-build.md rather than my u4j-fix.md: accepted; one notes file per unit is better.

## B5. At the merge: the readings, the notes and PR #40's report (confirmed, with what round 2 adds)
- **R3's readings (a) to (f) go into u4j-build.md, all six, as ruled.** A new section "## Readings, as ruled", in the
  merge commit, copied by script by line range: advisor-u4j.md :472-506 (R3, whole), then this file's B4.2, B4.3
  and B3's NC-U4J-R2-N2 line. As ruled:
  - (a) C4's "at least one log" form: ACCEPTED (FU-1 tightens its bite direction);
  - (b) a bite run takes its set's measured runs' practice run: CHANGED by FX-3 (c), and now with B4.2's step (two
    practice runs in force: INVALID);
  - (c) `params.config_hz` for the ruling's `machine.config_hz`: ACCEPTED;
  - (d) a bite run that compared nothing is INVALID: ACCEPTED;
  - (e) the Arm rule's key: CHANGED by FX-5 (LG on aarch64, the room on x86_64, each name from its own process, an
    orphan counts everywhere, a missing replay entry a gap);
  - (f) the decoded-hash figures read every phase: ACCEPTED.
  None of these reads Mustafa's words or an ADR row, except (e)'s orphan part, below.
- **R5's orphan reading goes into the readings list** (u4-build-plan.md §10's preamble and §11), its words copied by
  script from advisor-u4j.md :538-540, in the merge commit (B7.5 gives the edits). It reaches Mustafa with the
  readings PR before R5, for his go then; it is stricter.
- **One more entry, only if U4e's client drops stale groups** (B2.1), added when U4e's report says so, in these words:
  "PM-0001-17's '100 % of decoded snapshots' (ADR 0001:1605; PM-0001-58 at :1607 by its check) read as leaving out a
  group the client drops undecoded as stale (a group of a tick older than one it already decoded, never
  acknowledged, each drop counted): that tick is lost, as a tick none of whose groups arrived, and is not compared
  (the advisor's U4j round-2 ruling, B2.1)."
- **PR #40's report** (its U4j part, when U4's report is folded; until then the card's closing checklist under Next
  steps holds the line) names, by id, severity and owner:
  - the open security findings, which keep PR #40 needs-mustafa: FU-1 (SEC-F7, Low), FU-2 (SEC-F9's later half,
    Medium), FU-3 (SEC-F10, Low), FU-4 (SEC-F12, Low), FU-5 (C3's flag, Low), and the new FU-7 (SEC-R2-N2, Low);
  - FU-6 (mine, A-U4J-1, Medium: the message-size condition), open;
  - round 2's follow-ups and carries: entries 61 to 65 (B7.2) and C-U6-9 to C-U6-11 (B7.4);
  - the review count: two rounds, F1 (High) closed in round 2, no round 3.
- **§4.1's U4j public items** (u4-build-plan.md, the merge commit; B7.3): found by diff against 411eeee, not from
  memory. The fixer's "only `Mark::ws_bound_line`" covers its fix pass alone; U4j as a whole adds more.
- **Round 1's carries are not yet in u4s-carries.md** (advisor-u4j.md "Carries": U4e, U4r, U6; nothing numbered
  them, and the U4e brief rev 3 says it "relies on nothing of U4j's"). They are numbered in the merge commit, merged
  with round 2's: entries 61 and 62 (B7.2) and C-U6-9 to C-U6-11 in plan.md's U6 row (B7.4). Then the U4e brief's
  3.1 is re-copied from §U4e, and its R28 row ("U4j has not merged") is redone at launch.

## B6. The test-engineer's two process slips: neither weakens the evidence
- **Plants by whole-file Write, not by Edit in place.** One planted practice.rs and one planted figures.rs, each
  holding its plants behind a `ZZ_PLANT` switch; one run per plant set; a whole-file restore.
  - What would weaken it: a planted file that differs from the reviewed one beyond the plant lines (a run then tests
    other code), or a restore that is not exact (the copy reviewed afterwards, and merged, would not be the one
    planted from).
  - What rules both out:
    - each planted file was diffed against the original rebuilt from wip/u4j.patch (only the plant lines differed);
    - each restore was hash-checked (practice.rs back to e074d58d, figures.rs to e9282de6), and all 66 paths equalled
      the start hashes at step 7 and again after probe2 (u4j-round2-test.md);
    - each run's failures match its plant exactly (run 9's ROLE: nothing else failed; run 2's BAND: the three band
      tests; runs 5 to 8 and 10 to 11: the named test with the plant's own message), so no hidden extra change shows.
  - The one gap: the planted files' diffs were checked but not saved, so "only the plant lines" rests on the
    reviewer's word, backed by the failure pattern. It does not change a conclusion. M1 and M4 re-check the hashes
    independently before anything lands.
  - From now on: plants by Edit in place, or the planted file's diff saved beside the run's output.
- **Three notes lines appended with python** (and, in the other two seats, a shell heredoc), then each file rewritten
  whole with Write, its content kept. The rule (Write and Edit only) exists so the guard sees every write. The lines
  are notes, not evidence: every result in them comes from the runner's exit code and the test runner's own summary
  line, and the counted check.mjs run is reproduced by M5. No weakening.
  - The cause is the seats, not the agents: all three say their run had no Edit tool. The main session gives the next
    review workflow's agents the Edit tool, or tells them to rewrite their notes whole with Write from the start.
- **The netcode-architect's first read-only commands ran outside run.sh** (ls, git status, grep, a python read of
  marks.json): reads only; no build or test result depends on them.

