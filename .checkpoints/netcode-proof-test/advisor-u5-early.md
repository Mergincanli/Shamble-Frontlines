# Advisor ruling U5-EARLY: may U5 start building now, beside U4f2's wait for F-B-COUNT? (2026-11-21)

Stand-in: a general-purpose agent following main's `.claude/agents/advisor.md` (read with `git show origin/main:...`).
Seat: day session, PR #40, branch claude/netcode-proof-test-plan-hcky5g. Writes only this file.

## Plan (the blocks I will write, in this order)
1. Facts read: each fact the caller cites, checked at its file and line (u5-brief.md, advisor-u5-brief.md,
   advisor-u4g-early.md, history.md, plan.md, u4-build-plan.md, u4f2/u4g patches' file lists, u4h-brief.md, the card,
   main's CHECKPOINTS.md DEP-HOLD, tools today, disk today).
2. Overlap map: U5's files against U4f2, U4g (early and after re-base) and U4h; the room code -34 drives.
3. Ruling on (i): worth its risks, or U6's brief first.
4. Ruling on (ii): TEST-FIRST rule 2 or Mustafa's.
5. Ruling on (iii): the file and hunk rule.
6. Ruling on (iv): section 9's launch items now or at the merge.
7. Ruling on (v): the line for his message.
8. Verdict and numbered conditions.

## Status
- [x] 1 facts · [x] 2 overlap · [x] 3 (i) · [x] 4 (ii) · [x] 5 (iii) · [x] 6 (iv) · [x] 7 (v) · [x] 8 verdict

## Notes (one line per finding, as found)
- N1: advisor-u5-brief.md:283-285 made U5-ALLOC and U5-SIZE "neither a stop" **because U5 builds after U4's close**.
  An early start removes that premise for U5-SIZE: his SIZE-CHECK (history.md:2048-2051, "When a unit passes that,
  stop and tell me in three lines") puts an S unit's line at about 1,400 (u5-brief.md:303-304; card :175-176), and
  both U5-SIZE options raise it (5,750 whole; 4,800 for U5 split). Until he answers, the early build runs under ~1,400.
- N2: U5-SIZE's two options share everything but -35's tool (u5-brief.md:389-391): the U4G-Q1 precedent (build only
  what both options contain, under the stricter stop) leaves Block 7 out until he answers (a).
- N3: the runner sets `CARGO_NET_OFFLINE=true` on every call (u4f-brief.md:311): the fuzz lock's resolve, the fetch of
  libfuzzer-sys's sources and the tool installs cannot go through it; they are networked steps outside the builder.
- N4: /root and /tmp sit on the same filesystem as the repository (`df -B1 /root /tmp`: one device, 9,359,495,168 B
  free): the nightly toolchain, the registry sources and `cargo install`'s temporary builds all spend the same disk
  that U4f2's FP-2, R6 and -21's set need.
- N5: u5-brief.md:52's "check.mjs may move with them" does not hold today: check.mjs is shut in u4g-brief.md:50 and
  u4h-brief.md:62, outside U4f2's lane (u4f2-brief.md:170-172) and absent from its patch. check.mjs is U5's alone.
- N6: C-U5-1 asserts "refusals at most 4,096 a tick" and "one counter per offered command": U4f2 retypes
  `Room::refusals()` to `&[RefusalRecord]` and renames `Refusal` (advisor-u4g-early.md:50-58), U4g adds -58's three
  command counters (u4g-brief.md:45, :63), U4h opens the room (u4h-brief.md:55). Squarely in U4's churn: it waits.
- N7: U4h opens crates/sim for `HitHistory`'s sizing (u4-build-plan.md:904; u4h-brief.md:54): carry 48's test (a push
  into a full `HitHistory`) may need a touch at the re-base. Small; allowed early in a new file, noted.
- N8: U4g's `Interp`, event release and ClientSettings send land in the client (u4g-brief.md:57): Block 4's client bars
  (u5-alloc-bounds.md, 304 x L_e and 2,213,272) may move with them. Section 9 re-measures "if U4g or U4h changed a
  decoder"; it should read "a decoder or the client's receive path".
- N9: plan.md:644 runs the audits "at U5, at stop B and before every run"; :648-649 an advisory on a network-facing crate
  "blocks the next run". An audit run during the early build would put a finding in front of U4f2's -21 set: it comes to
  the advisor at once, never acted on by the builder.

## 1. Facts read (DONE)
| The caller's fact | Holds? | My evidence |
|---|---|---|
| u5-brief.md rev 2.1, about 395 lines; :4, :33, :52, :55, :88-89; Blocks 0-8; sections 6, 8, 9 | yes; 407 lines | read whole: :4 and :33 "after U4's close"; :51-54 the lane; :55-59 the files; :88-90 the tools; Blocks :101-209 (Block 0 ends at DEP-HOLD's stop, :117-120); section 6 :301-327; section 8 :336-396; section 9 :398-407. :52's check.mjs clause does not hold (N5) |
| advisor-u5-brief.md rev-1 :1-322, rev-2 :324-430; Q1-Q3, Q5-Q7 ruled | yes | read whole; rulings :157-203; "neither a stop" rests on "U5 builds after U4's close" (:236, :283-285: N1) |
| Tools: stable and 1.99.1 only; no nightly, cargo-fuzz, cargo-deny or cargo-audit | yes | `rustup toolchain list`: stable, 1.99.1 (active); `ls /root/.cargo/bin`: none of the three |
| crates.io reachable (cargo-fuzz 0.13.2) | not rechecked | I made no network call |
| The nightly existed at U0 | yes | u0-probes.md:9: nightly-2026-11-01 = 1.101.0-nightly f9ceb4a4d 2026-10-31, "installed at U5 when fuzzing starts" |
| DEP-HOLD (main's CHECKPOINTS.md §3; card :52-54) | yes | `git show origin/main:.claude/CHECKPOINTS.md` §3: held by path in any folder; released by dependency-check.md after security-auditor's answer on exactly that content; "build nothing that holds the change, and push nothing by hand". No dependency-check.md exists today (`ls`: absent) |
| U4f2's built tree: the diff headers named | yes | Grep `^diff --git` wip/u4f2-build.patch: 14 files (bots lib.rs; client lib.rs, tests/endpoint.rs; lab contract.rs, main.rs, scenario.rs, forged.rs new; room input.rs, lib.rs, room.rs, tests/commands.rs; SERIES.md; ops/runs c-u4f-r2.mjs, c-u4f-runs.mjs). lab main.rs has one hunk, `@@ -14,6 +14,7 @@`: `mod forged;` after `mod contract;` (:1730-1737) |
| U4g's early snapshot: client, room, sim, new tests; lab, bots, ops after the re-base | yes | wip/u4g-early.patch: 11 files (client lib.rs, cues.rs, predict.rs, tests/bites.rs, tests/predict.rs; room lib.rs, room.rs, tests/bites.rs, tests/prediction.rs; sim world.rs, tests/predict.rs); u4g-brief.md:44-50 opens client, sim, room, lab, bots, ops (new files), SERIES.md (new rows); shuts proto, check.mjs, Cargo.toml and Cargo.lock |
| U4h's lane, u4h-brief.md section 1 | yes | :53-63: opens sim, room, server, client, proto (one pub item, input.rs:53-54), bots, lab, ops, SERIES.md; shuts net, log, check.mjs, Cargo.toml and Cargo.lock |
| Lane order u4-build-plan.md:951-952; plan.md §12.1 lists U5 after U4 | yes | :951-952 "U4f, U4f2, U4g and U4h in order (... never two at once)": a rule among U4's lanes. plan.md:1244-1255 is the units' table; the stops are §12.2 (:1257 on) |
| U4G-EARLY's conditions 1-10 | yes | advisor-u4g-early.md:163-203 |
| Disk: a cold whole-workspace run both builds 4.4-4.8 GB; P27; runner's 4 GB; the 8.8 GB line | yes | u4g-build.md:62, :77, :93, :107: 4.53, 4.42, 4.38, 4.48 GB; :99 "whole-workspace runs start only at >= 8.8 GB"; u4f-brief.md:853-856 the floors. Also u4h-brief.md:197-200: a full check.mjs run fills the shared target by about 6.3-7.1 GiB |
| Free disk today | 9,359,613,952 B | `df -B1 /home/user/Shamble-Frontlines` (14:3x UTC); shared target 463,304,857 B (`du -sb`); same device for /root and /tmp (N4) |
| A builder's worktree starts at main; first step `git merge --ff-only` | yes | u4f-brief.md:292-295; card :181-182 |
| C-U6-19's line, plan.md:1253's end; u5-brief.md states U5 has none | yes | plan.md:1253 ("every unit brief before U6's merge (U4g, U4h, U5) states in one line..."); u5-brief.md:257-267 |
| TEST-FIRST rule 2 | yes | history.md:2259-2262; :2262 keeps for him "the stops and run days your plan gives me" |
| KEEP-PROGRESSING; his words of 13:17 UTC | yes | history.md:7402-7410; :7507-7509 |
| (mine) SIZE-CHECK and the U4F2-SIZE precedent | yes | history.md:2048-2051, :2067-2075; advisor-u4f2-brief.md:238, :245 "The build waits for his U4F2-SIZE answer" |
| (mine) the register lines of section 9 | not yet made | u4s-carries.md:759-764: U5's section holds only C-U6-19's line and carry 48 |
| (mine) the spike tree since 1faf894 | unchanged | `git diff --stat 1faf894 HEAD -- spikes/netcode-proof ':(exclude)…/results'` prints nothing; HEAD dd77796 |

## 2. Overlap map: U5's files against U4's remaining lanes (DONE)
| U5's file (u5-brief.md:55-59) | U4f2 (patch; FP-2) | U4g (early patch; after its re-base) | U4h | Early? |
|---|---|---|---|---|
| fuzz/ (new crate, its own workspace and lock) | no | no | no | yes |
| crates/alloc (new) | no | no | no | yes |
| root Cargo.toml, Cargo.lock; the Block-0 manifests (sim, proto, room dev deps; net's `fuzz` feature) | shut | shut | shut | yes, after dependency-check.md |
| check.mjs, check.test.mjs | not in its lane | shut | shut | yes (N5) |
| crates/net (the scanner's entry) | no | shut | shut | yes |
| crates/proto (the -34 bite: quant.rs `BITES` :34 and one decoder) | no | shut | one pub item, input.rs:53-54 | yes, 3 lines clear of input.rs:53-54 |
| crates/sim/tests (carry 48, T2-L2; new files) | shut; opens sim/src under F-B-COUNT (b) | world.rs, tests/predict.rs | sim, `HitHistory`'s sizing (N7) | yes, new files only |
| crates/proto/tests (new files) | no | shut | no | yes |
| crates/room/tests (C-U5-1) | retypes `Room::refusals()`, renames `Refusal` | room.rs, -58's counters later | opens the room | **no** (N6) |
| crates/client (an entry, only if needed) | lib.rs docs, tests/endpoint.rs | Predictor, CueLog, then Interp | AimCast, -54's site | **no**; Block 4's client targets in fuzz/ only, bars re-measured at the re-base (N8) |
| crates/lab (two writers) | main.rs:14-19, contract.rs, scenario.rs, forged.rs | scenario.rs, logs.rs | lab, recordings | new modules early; main.rs hook-up only outside :14-19; no other lab file |
| ops/runs (new runners) | edits c-u4f-r2.mjs, c-u4f-runs.mjs | new files | new c-u4h-*, det-u4.mjs | yes, new files only |
| marks/SERIES.md (new rows) | edits it | new rows | new rows, cells | **no**: insertion points collide |
| README's status line; results/u5-audits/ | every unit; results are the close's | | | **no** |

## 3. (i) Worth its risks? Yes, bounded; and U6's brief beside it, not instead (DONE)
**What the early start can deliver before Tuesday.** Under SIZE-CHECK's S line (N1, condition 2) the build stops near
1,400 added lines at a block end: by section 6's ranges that is Blocks 0, 1, Block 2's sim half and most or all of Block
3 (npt-alloc 180-320 plus check.mjs's part 190-500, carry 48 and T2-L2 about 60-140, the fuzz core and plain targets
about 400-650). Those are the parts with the most unknowns: the dependency read (DEP-HOLD's stop, a long lead), the fourth
unsafe exception and check.mjs's fuzz/ scan and lock check, the nightly's re-measure of the bars and the inputs-a-second
rates that decide the sanitizer (U5-Q5). A bar that moves on the nightly then reaches him before he answers U5-ALLOC on
Tuesday, not after.

| Risk | How big | What bounds it |
|---|---|---|
| Tool installs from the internet | the same as at the planned time: the plan names the tools (plan.md:641, :644-647, approved with rev 3.3) and Block 0's read covers their exact versions; only the date moves. Early adds one thing: fewer people at hand to answer an ask | the read and dependency-check.md first; the networked steps by the main session only (N3), each named before it is sent if it will ask; early, only the nightly and cargo-fuzz (Block 3 needs them); cargo-deny and cargo-audit wait for Block 8, past the S line (condition 3) |
| The lab overlap | small early: Blocks 5 and 7 lie past the S line (7 also past U5-SIZE: N2) | new modules only; main.rs's hook-up outside U4f2's hunk :14-19 in a `mod` group of its own (rustfmt sorts a group, so an alphabetical `mod fuzz…` would land inside :14-19); no other lab file (condition 4) |
| The room code (C-U5-1) against U4f2's retyped records | real: its asserts read `Room::refusals()` and the command counters U4f2, U4g and U4h change (N6) | C-U5-1 waits for the re-base after U4's close, as the brief planned (condition 1) |
| Disk | the largest. 9.36 GB free, 5.07 GB above the 4 GiB floor; one whole-workspace run both builds costs 4.4-4.5 GB; the nightly and the tools sit on the same device for good (N4); U4f2's R6 and -21's set come first and are already tight (u4h-brief.md:197-200) | Block 1's whole-workspace run before the nightly is installed; later early blocks touch only new files outside every other crate's build, so their own crates' tests suffice; the fuzz builds' footprint measured and cleanable; U4f2's lines first, the nightly uninstallable (condition 7) |
| Agent slots | 0 run now; the builder 1, security-auditor's read 1 (while the builder waits at Block 0) | U4f2's agents first at F-B-COUNT's answer (FP-2, then round 2's three seats): the builder returns before them (condition 6) |
| A quiet machine and the lane lock | as U4G-EARLY's N4: a plain U5 build makes a U4f2 counted call or plant exit 5 | never at the same time (condition 6) |
| -21's evidence | none by construction if nothing of U5 enters a lane commit; U5's manifests in a lane commit would also trip DEP-HOLD on every save of the lane | condition 5 |
| An audit finding before -21's set (N9) | low: Block 8 lies past the S line | any advisory comes to me at once; the builder acts on none (condition 8) |
| Re-base work | moderate: new files merge clean; the manifests are shut in every U4 lane; proto's bite sits away from U4h's one item; carry 48's test may need a touch (N7); the client bars re-measured (N8) | condition 9 |

- **U6's brief:** main-session paper, no build lane, no disk. It does not compete with U5's builder except for one
  advisor slot at its check, so the two go side by side inside the 3-agent cap; U5-EARLY first, since its Block 0 read
  has the long lead. U6's "there now" lines will be re-read at U6's own `<base>` either way (TEST-FIRST rule 3 keeps it
  short).
- **What the proposal misses** (each folded into the conditions): the S line binds while U5-SIZE is open, so "the build
  goes under U5-SIZE's lean (a)" would raise his stop (ADV-U5E-1); Block 7 before his answer acts on the lean of an open
  question (ADV-U5E-2); the runner is offline, so Block 0's resolve and every install are main-session steps (ADV-U5E-3);
  the permanent disk of the nightly and the tools against U4f2's lines, and P27 not reaching release-profile fuzz builds
  (ADV-U5E-4); C-U5-1 is the room-code overlap, not only the lab (ADV-U5E-5).

### Findings on the proposal (CLAUDE.md's scale)
- **ADV-U5E-1 (High):** item 3's "the build goes under U5-SIZE's lean (a)" sets a stop near 5,750 while his U5-SIZE is
  open; his SIZE-CHECK puts an S unit's stop past about 1,400 (history.md:2048-2051; u5-brief.md:303-304), and raising a
  stop is his (history.md:2262; the U4F2-SIZE precedent, advisor-u4f2-brief.md:238, :245). Fix: condition 2.
- **ADV-U5E-2 (Medium):** -35's tool built before his answer pre-empts option (b) (u5-brief.md:389-391). Fix: Block 7
  only on his (a) (condition 1).
- **ADV-U5E-3 (Medium):** the runner sets `CARGO_NET_OFFLINE=true` (u4f-brief.md:311): Block 0's fuzz-lock resolve, the
  fetch of libfuzzer-sys's sources for the read, and the installs are networked steps nobody is named for. Fix: condition 3.
- **ADV-U5E-4 (Medium):** disk: the installs are permanent on the shared device (N4); P27 (`--profile dev`) likely does
  not reach cargo-fuzz's release-profile builds under the target triple (unverified); U4f2's R6 and -21's set have
  priority. Fix: condition 7.
- **ADV-U5E-5 (Medium):** C-U5-1 drives `Room::receive` and reads `Room::refusals()` and the command counters U4f2, U4g
  and U4h change (N6); item 2 covers only the lab. Fix: condition 1 (deferred to the re-base).
- **ADV-U5E-6 (Low):** u5-brief.md:52's "check.mjs may move with them" does not hold (N5). Fix: condition 10.
- **ADV-U5E-7 (Low):** section 9 re-measures the bars only "if U4g or U4h changed a decoder"; U4g's `Interp` can move the
  client's receive path (N8). Fix: condition 10.
- **ADV-U5E-8 (Low):** an early audit result meets plan.md:644-649's "blocks the next run" before -21's set (N9). Fix:
  condition 8.

## 4. (ii) Whose choice: mine under TEST-FIRST rule 2, inside his S line (DONE)
- **Rule 2's four tests hold** (history.md:2261): inside the plan (U5's own work by its checked brief rev 2.1; only its
  date moves; the order of approved work is the advisor's: advisor.md, "You decide: ... the order of approved work";
  plan.md §12.1 is the units' table, its stops are §12.2); undoable (a worktree and a patch, nothing merged; the nightly
  and cargo-fuzz uninstall; dropping it all costs builder time only); no pass mark, judge, marks.json text, run count or
  SERIES.md row moves (new rows wait for the re-base); no money (crates.io and the Rust project's distribution are free;
  agent usage under the university's cap, SPEND-LIMIT (a), history.md:1932) and nothing on main (installs go to the
  container's home, never main's tree).
- **Not one of "the stops and run days your plan gives me"** (:2262), on two terms: the early build keeps SIZE-CHECK's S
  line until his U5-SIZE answer (raising it is his: ADV-U5E-1), and builds only what both U5-SIZE options contain (Block 7
  waits: ADV-U5E-2). U5-ALLOC gates only -34's counted runs, none of which run early. Stop B and Stop C do not move.
- **The tools are not a new decision of his:** plan rev 3.3 (his approval) names cargo-fuzz on a dated nightly and the
  audits (plan.md:641, :644-647; u0-probes.md:9); the brief's Block 0 read is the review of the exact versions that
  CLAUDE.md's Never line ("run unreviewed code fetched from the internet") asks for. The date moves, not the review.
- **Not a chat question** under KEEP-PROGRESSING: nothing here is a pass mark, money or a stop; he can reverse it on
  reading the 2026-11-22 message, and the installs come only after the read (Sunday at the earliest).

## 5. (iii) The file and hunk rule (DONE)
- **Open early** (no remaining U4 lane opens them; section 2): fuzz/ and crates/alloc (new); the root Cargo.toml and
  Cargo.lock and the other Block-0 manifests, exactly as dependency-check.md names them; check.mjs and check.test.mjs;
  crates/net (the scanner's entry, under `fuzz`); crates/proto (the -34 bite in quant.rs's `BITES` and its decoder site),
  with at least 3 unchanged lines between any U5 change and proto/src/input.rs:53-54 (U4h's one pub item); new test
  files under crates/sim/tests and crates/proto/tests; new files under ops/runs; new modules under crates/lab/src.
- **crates/lab/src/main.rs:** U5's `mod` lines as a group of their own (a blank line from the existing list, so
  rustfmt's module sort keeps them there) and its dispatch arms before the probe fallback, each with at least 3
  unchanged lines from U4f2's hunk (old lines 14-19 at 1faf894). No other existing lab file (contract.rs, scenario.rs,
  logs.rs, mirror.rs, forged.rs and the rest).
- **Deferred to the re-base after U4's close:** C-U5-1 (crates/room/tests; N6); any crates/client edit; SERIES.md's new
  rows (and any test that needs them); README's status line; results/u5-audits/ (the close's); Block 7 until his U5-SIZE
  answer is (a). Block 4's two client targets may be written in fuzz/ only if they need no client edit, their bars
  re-measured at the re-base (N8).
- **Names:** none of the names U4f2 renames or retypes (the room's `Refusal`, `npt_room::Refusal`, `Room::refusals()`,
  the fields `incoming` and `refusals`, `Offered::Refused`: advisor-u4g-early.md:50-58).
- **A needed edit that would cross the rule** is deferred to the re-base and noted with file:line; it stops the builder
  for the main session only when the block's other items cannot be built or tested without it (U4G-EARLY's form).
- **Under F-B-COUNT (b) or (c):** U4f2 opens crates/sim's source; U5's sim test files come to me with that change before
  either builds on (U4G-EARLY condition 5's form).

## 6. (iv) Section 9's launch items: now, before the builder starts; the rest at the re-base (DONE)
- **Now** (my rev-2 check's condition 4 ties them to the launch, advisor-u5-brief.md:401; the launch is now):
  - the register (u4s-carries.md, the next free numbers, each quoting its source): U5's section gains U5-Q1 to Q3 and
    Q5 to Q7 with their terms, T2-L2 and S2; U6's section gains -35's counted run after U6's merge and the swap-form carry;
    :764's cite becomes ADR 0002:269;
  - plan.md §12.1's U5 and U6 rows, one line each, by U5-Q2 (a) term 2;
  - u5-brief.md: one pointer line at its head (U5-EARLY, advisor-u5-early.md: the blocks and files of condition 1 only;
    :4, :33 and :51-54 stand for the rest), :52's check.mjs clause corrected (ADV-U5E-6), section 9's re-measure line
    widened to "a decoder or the client's receive path" (ADV-U5E-7);
  - the early `<base>` filled (condition 8) and the runner copy made as u4f-run.sh was (u4f-brief.md:306-309: by script
    from it, the lane's names only, its sha256 and `bash -n` recorded);
  - the card's S-letter record: already there (card :175-176); the size line "S line about 1,400 until U5-SIZE" beside it.
- **At the re-base:** the real `<base>`; the "there now" re-read in room, client, lab, sim and proto; the bars
  re-measured; SERIES.md's rows and README's line; the deferred items.
- **When his answers come:** U5-ALLOC's and U5-SIZE's answers on the card; the stop his U5-SIZE answer sets.

## 7. (v) The line for his 2026-11-22 message (DONE)
Paste-ready, one item of "Choices taken without you" (U5-ALLOC's and U5-SIZE's texts stay as finalised,
advisor-u5-brief.md:403-422):
> By the advisor's ruling (U5-EARLY), unit U5 (the decoder fuzz tests, the memory counter, the dependency checks and the
> audits) has started in a separate copy while U4f2 waits for your F-B-COUNT answer. Its dependency list goes to the
> security reviewer first, and only after that read are the two tools the plan names fetched and installed (a dated
> nightly compiler and cargo-fuzz). It builds only the parts no remaining U4 unit touches, pauses at the small-unit size
> line (about 1,400 lines) until your U5-SIZE answer, gives way to U4f2 as soon as you answer, and merges only after U4
> closes. You can reverse it.

For the card's "Choices taken without him": "2026-11-21 by the advisor's U5-EARLY ruling (advisor-u5-early.md): U5's
build begins now beside U4f2's wait, under its conditions 1-10 (the S line until U5-SIZE; Block 7 only on U5-SIZE (a);
C-U5-1 and the shared files at the re-base); why: inside the plan, undoable, no pass mark, judge, marks.json or money;
nothing on main. To name in the 2026-11-22 message (section 7's line)."

## 8. Verdict: APPROVE WITH CONDITIONS (U5-EARLY (a), with conditions 1-10) (DONE)
Mine under TEST-FIRST rule 2 (section 4); no ESCALATE. No new advisor check before launch unless a condition cannot be
met as written. ADV-U5E-1 (High) is met by condition 2; the Mediums by conditions 1, 3 and 7.
1. **Scope and order:** one builder (general-purpose, Opus 5.5, xhigh), its own worktree, first `git merge --ff-only`
   to the lane head and the check (u4f-brief.md:292-295); u5-brief.md rev 2.1 with U4f's start, stops and build rules.
   Order: Block 0, 1, Block 2's sim half (carry 48, T2-L2), 3, 4 (the scanner; the client targets per section 5), 5, 6,
   8; Block 7 only after his U5-SIZE answer is (a). Deferred to the re-base: C-U5-1, any client edit, SERIES.md's rows,
   README's line, results/.
2. **The size line:** SIZE-CHECK's S line until his U5-SIZE answer: past about 1,400 added lines at a block end
   (section 6's count, the fuzz lock apart), the builder stops, snapshots and returns; his answer then sets the stop
   (about 5,750 whole; about 4,800 for U5 split). The day's message says it paused there; no new question.
3. **Block 0, the networked steps, the tools:** Block 0 as worded, every file by path with its sha256; the main session
   alone makes the networked steps, by `--manifest-path` on the worktree's paths, never by `cd`: before the read, the
   fuzz lock's minimal resolve from the copied workspace lock and its fetch (downloads, no build), so the read sees the
   exact sources; after dependency-check.md, the nightly by rustup (`nightly-2026-11-01`, minimal profile) and
   cargo-fuzz by `cargo install --locked --version <the read's>`, each named before it is sent if it will ask.
   cargo-deny and cargo-audit: in the read now, installed only for Block 8. A version the read does not clear is not
   installed and comes to me. The builder runs nothing networked and builds nothing that holds the change before
   dependency-check.md; rust-toolchain.toml untouched.
4. **The file and hunk rule:** section 5 whole.
5. **The lane branch:** nothing of U5's in any lane commit before its merge; snapshots only under wip/ by
   worktree_patch.py, sha256 on the card, the first after dependency-check.md, its manifest and lock files equal to its
   lines; the builder never in the main folder; the judged runs keep the main folder's own target.
6. **Time-sharing and slots:** U4f2 first, then U4g's re-base, then U4h. The builder runs only while no U4 lane builder,
   reviewer, runner call, R6, merge check or set is active, inside the 3-agent cap with theirs first. At F-B-COUNT's
   answer it makes no new runner call, leaves no plant, snapshots and returns; it resumes only in such a window. Under
   F-B-COUNT (b) or (c), section 5's last line.
7. **Disk (U4f2's lines first):** launch at 6.5 GB free or more (`df -B1` in the notes; 9,359,613,952 B at 14:3x).
   Block 1's whole-workspace run (both builds) at 8.8 GB or more and before the nightly is installed; a later early block
   touching only new files outside every other crate's build runs its own crates' and the fuzz crate's tests; one that
   changes a crate others build against (proto's bite, net's feature, a manifest) runs workspace-wide at 8.8 GB or waits
   for the re-base. No build call under 5.5 GB; each install at 8.0 GB or more, `df -B1` before and after. The builder
   measures its fuzz builds' footprint at the first fuzz build; the main session tests one clean form for it (by target
   triple, `cargo clean --target-dir`; P27's dev clean likely does not reach it: unverified) and runs it with P27 before
   any U4 lane's build. Build-time corpora and artifacts stay in the builder's worktree's .gate/, short runs only (the
   rates, the re-measure, the bite). If U4f2's FP-2, R6 or -21's set lacks room, the main session empties U5's artifacts
   and may uninstall the nightly (reinstalled at the re-base under the same read).
8. **The early `<base>`, block ends, findings:** filled now as U4G-EARLY's condition 7 filled it (the lane head, its
   spike tree equal to 1faf894's outside results/ by `git diff --quiet`, counts 642 measured and 739 bites, the test-step
   line 1,207 s, the stamps refilled); no counted check.mjs run early; at the early stop, fmt and clippy of the touched
   crates and the fuzz crate on stable 1.99.1, `--locked`, in the shared target. Any advisory or audit result the build
   sees comes to me the same day; the builder acts on none.
9. **The re-base and merge after U4's close, as planned (not earlier: U5's manifests, check.mjs and npt-alloc would move
   every later U4 lane's base and counts, and C-U5-1 needs U4's room):** a fresh worktree from U4's closed head,
   `git apply --3way`, each hand-resolved hunk recorded with file:line; every Block-0 file byte-equal to its
   dependency-check.md line, else a second read before anything builds; `<base>` refilled; "there now" re-read in room,
   client, lab, sim and proto; the bars re-measured (decoders and the client's receive path, stable and nightly); the
   deferred items first, then the rest; the size counted from the new `<base>`, the early lines included.
10. **Launch items, record, review:** section 6's "now" list before the builder starts; recorded under "Choices taken
    without him" with section 7's card line; the 2026-11-22 message carries section 7's paste-ready line; no review round
    on the early half alone (section 7 of the brief reads U5's final diff, inside the budget).

## What I could not check
- No network call: crates.io's reachability and the tools' current versions are the caller's. The disk cost of the
  nightly, cargo-fuzz's install and the fuzz builds is estimated, not measured; cargo clean's reach by target triple and
  profile is from memory. Whether the fuzz core reaches the S line inside Block 3 or at its end is section 6's estimate.
  F-B-COUNT's answer and its day.
DONE

## Ruling on the dependency read's findings (SEC-U5D-1, -4, -2), 2026-11-21 ~15:4x UTC
### Plan
1. Read security-u5-deps.md (295 lines), dependency-check.md (18), u5-deps-facts.md (39) whole; the builder's
   fuzz/Cargo.toml and the root comment (worktree agent-a60a432d4564f4cc7, by Read). 2. Rule SEC-U5D-1. 3. Rule SEC-U5D-4.
   4. Rule SEC-U5D-2. 5. Conditions.
### Notes
- The read cleared all eight Block-0 files at the hashes dependency-check.md holds (:11-18 = security-u5-deps.md:286-293).
- The expansion as read (security-u5-deps.md:96-108): per target, `#[no_mangle] pub extern "C" fn LLVMFuzzerInitialize`
  and `rust_fuzzer_test_input`, plus a safe `__libfuzzer_sys_run`; no unsafe block, unsafe fn or extern block enters
  npt-fuzz; libfuzzer-sys's own unsafe stays in its crate (:105-108). Two points are reasoned, not run (:130-132): the
  forbid stays silent, and edition 2024 accepts the bare `#[no_mangle]` from 2018-edition tokens.
- fuzz/Cargo.toml:24-28 already says the macro expands an exported entry in the crate and that rustc does not lint
  another crate's macro; it does not name the two symbols. The root comment (Cargo.toml:83-110) counts "Four crates" as
  ADR 0002:269's exceptions, each with an `#[allow]` with its reason, and says a forbid lets no `#[allow]` compile (E0453).
- The ADR's form (an invariant, a test, an `#[allow]` with its reason) cannot be met here: under forbid an `#[allow]` is
  E0453, and under deny one would be inert, since the lint is never raised inside the expansion: a false record.
- Writing the two exports by hand instead (with real `#[allow]`s) would copy libfuzzer-sys's private contract: more
  unsafe by our hand than the read macro, so worse.
- cargo-fuzz's own flags (sancov passes, `--cfg fuzzing`, the sanitizer: security-u5-deps.md:160-165) copied into a
  plain `cargo build --locked` would tie our scripts to its internals; a missed flag silently weakens the
  instrumentation, and nothing would fail. That is a quiet loss of -34's evidence; plan §4.8 names cargo-fuzz.

### SEC-U5D-1: a recorded dependency-macro case, not a fifth ADR 0002:269 exception
- **Form:** recorded beside the four, not counted among them (they are crates whose own code holds unsafe under a
  reasoned `#[allow]`; none can exist here). The root comment's "Four crates" stays true. npt-fuzz keeps forbid for its
  own code, the stricter state. Mine under U5-Q7 term 3 and TEST-FIRST rule 2: cargo-fuzz is plan §4.8's tool, and
  `fuzz_target!` is its harness; ADR 0002's text does not change.
- **Invariant:** each target bin holds exactly the two exports `fuzz_target!` expands (LLVMFuzzerInitialize,
  rust_fuzzer_test_input), from libfuzzer-sys 0.4.13 as read; one `fuzz_target!` per bin, so no symbol twice in a binary;
  no unsafe block or unsafe fn, and no hand-written `no_mangle`, `export_name`, `link_section` or `extern` anywhere in
  fuzz/.
- **Tests:** in check.mjs, before Block 3's first cargo-fuzz build (in Block 1 if still open), each with a
  check.test.mjs plant that fails by name:
  - fuzz/Cargo.lock's libfuzzer-sys entry is exactly one, at 0.4.13, from crates.io, with its full checksum copied from
    the lock (not retyped from the read's prefix). arbitrary 1.5.0 is pinned likewise, and no derive_arbitrary package
    is in the lock (no proc macro arrives unread);
  - every fuzz_targets/*.rs calls `fuzz_target!` exactly once, and fuzz/ uses no other libfuzzer_sys macro: fuzz_mutator!,
    fuzz_crossover! and any other refused by name (SEC-U5D-3). No glob, renamed or `macro_use` import of libfuzzer_sys
    (the check reads text);
  - none of the hand-written attributes above (fuzz/ read as crates/ is: U5-Q7 term 1).
- **At the site:** the fuzz crate's shared module doc names the two exports, the read (security-u5-deps.md:96-115) and
  check.mjs's rule; one comment line above each `fuzz_target!` call points there (the reason where an `#[allow]` would
  stand).
- **Block 3's first `cargo check` of a target** records in the notes the two reasoned points. If the forbid does fire,
  it comes to me: no `#[allow]` and no lowering of the lint.
- **Recorded:** under "Choices taken without him", and named in the day's message. "After the close" gains his
  ADR-text item: ADR 0002:269's three-part form does not cover the exports a dependency's macro expands. U5's report
  names the case in a line of its own, with security-auditor and the advisor as its readers.

### SEC-U5D-4: the lock check around every cargo-fuzz call, in a guard of its own; not plain cargo
- **The form:** before every cargo-fuzz call:
  - fuzz/Cargo.toml's and fuzz/Cargo.lock's sha256 equal dependency-check.md's lines (read from the file, never
    retyped);
  - `cargo +1.99.1 tree --locked --offline --manifest-path <worktree>/spikes/netcode-proof/fuzz/Cargo.toml` exits 0;
  - CUSTOM_LIBFUZZER_PATH and CUSTOM_LIBFUZZER_STD_CXX are unset;
  - the subcommand is `build` or `run` only, never init or add; never --build-std, --careful or the memory sanitizer;
  - the target dir is the shared target.

  After the call, both files are hashed again. A mismatch voids the call's outputs: the main session cleans the fuzz
  builds' folder, and it comes to me. Plain `cargo +nightly build --locked` with copied flags is refused (notes,
  last line).
- **Where it lives:**
  - u5-run.sh stays a names-only copy of u4f-run.sh. The check is one small script of its own in the notes folder,
    written by the main session, its sha256 on the card. Every build-time cargo-fuzz call runs as the command of a
    runner call, `bash u5-run.sh [-x] <wt> <log> bash <guard> <wt> <cargo-fuzz args>`, never directly.
  - security-auditor reads the script before its first use (a short read of its own finding's fix). Its refusal is
    shown once on a planted one-byte manifest change, restored and re-hashed inside the same exclusive runner call.
  - The builder's brief says that cargo-fuzz is never called outside the guard.
- **The counted runs** (c-u5-fuzz.mjs, Block 5, in the repository and under U5's review) carry the same check in their own
  form: both fuzz files equal to the commit's tracked content and `cargo tree --locked --offline` exit 0 before each
  cargo-fuzz call, the lock re-hashed after; tested on canned cases. .checkpoints/ will be gone by then.
- Any later fuzz manifest change is a second read (Block 0's rule). The guard then passes only on the new lines.

### SEC-U5D-2: `=0.4.13` at the re-base, in one read with the comment, unless an earlier read comes first
- The caret's only gap (a re-resolve after a lost lock) is shut twice: by the guard before every cargo-fuzz call, and by
  check.mjs's pin at every end check. Pinning now would cost a second read and re-hashes for a Low these already cover.
  Never pinning would leave the spike's own exact-version rule (root Cargo.toml:51) broken.
- At the re-base (or with any earlier fuzz manifest change and its read): `libfuzzer-sys = "=0.4.13"`, fuzz/Cargo.toml's
  lints comment naming the two exports and check.mjs's pin, and the root comment's fuzz sentence pointing to it. All
  comment-and-pin, in one read, dependency-check.md's two lines replaced.

### Verdict on the read's conditions: APPROVE, with these conditions (S1-S4)
- **S1:** SEC-U5D-1 as above: the check.mjs rules and their plants before Block 3's first cargo-fuzz build; the first
  `cargo check` records the two reasoned points.
- **S2:** SEC-U5D-4's guard as above, read by security-auditor and shown on a plant before Block 3's first cargo-fuzz
  call. The installs may go ahead after Block 1 as planned: an install runs no cargo-fuzz on our crate.
- **S3:** SEC-U5D-2 at the re-base, or at an earlier fuzz manifest read.
- **S4:** the read's other conditions stand as dependency-check.md:4-10 lists them. The lines S1 adds count toward the
  S line (condition 2).
DONE (SEC-U5D ruling)
