# U4 report: the L1 lab, closed 2026-11-23

For Mustafa and his Advisor. U4 is plan rev 3.3's unit for the in-container lab (L1), cut by NPT-TRIM
(history.md:7561-7593): "U4f2: finish FP-2 with F-B-COUNT (a), its review, the merge and -21's set; then close U4 with
its report. Of U4's other rows, keep only those about the wire and the server" (:7570). This report folds the sub-unit
records (u4-build-plan.md:1001: "U4's report folds them"); each keeps its own detail.

## In plain words
- **What U4 built:** the in-container test lab for the server's netcode, made of:
  - the wire format and the server loop's timing rule;
  - a simulated network;
  - a stand-in game sim, with a Wasm twin that must give the same hash;
  - the room and its codec;
  - the judge that reads the logs against marks.json, and the run scripts;
  - the client clock and command checks, with cheating test clients.
- **What passed here:**
  - whole: -3, -21, -22 and -57;
  - in part: -17 on P0, P2 and P3; -20 at L1 (its real-network half is R3's); -2's stress-scene load;
  - -47 passes, and moves to the real game as a record.
- **What did not:**
  - the loss-burst set (B1) for -17 and -18 reads INVALID, as the advisor ruled;
  - -2's estimate scene fails until carry 93's hit rule is set;
  - -58 and -30's lab part were never run.
  Each has a taker below.
- **What moved** (NPT-TRIM): prediction, interpolation, events and lag compensation (U4g, U4g2's client half, U4h)
  go to the real game's Slice 1 change-sets 7 and 8 and to Slice 3, with their marks. Nothing was loosened.
- **What stays, after R1:** U4's server rows, as a new small unit, U4k.
- **Size:** about 57,800 added lines in spikes/netcode-proof over 22 commits. Plan rev 3.3 sized all of U4 as M,
  about 3,000 lines (his count, history.md:2249). He answered each size stop the sub-units reached (below).
- Nothing is stopped on him.

## The sub-units, as merged
Added and removed lines in spikes/netcode-proof come from `git show --shortstat <commit> -- spikes/netcode-proof`.

| Sub-unit | What it built | Commit (date) | Lines | Record |
|---|---|---|---|---|
| U4s | seam types, wire grid, tick constants, manifests | f080551 (11-09) | +1,979 −13 | u4s-report.md |
| U4a | the wire (npt-proto), the catalogue's edits | 952f734 (11-10) | +5,813 −32 | u4a-build.md |
| U4b | the loop policy (schedule.rs), PM-0004-67's three bites | 411eeee (11-10) | +701 | history.md:1432-1435 |
| U4c | the link model (npt-l1, C1' and C2) | f3f7c2f (11-11) | +3,133 −341 | u4c-lc8-rerun.md |
| U4d | the stand-in sim (npt-sim), W1-S1's proto half | 39f4290 (11-11) | +7,653 −102 | u4d-report.md |
| U4j | the judge and marks change; round 2's wording | 4d2b0ef, d50396a (11-11) | +4,584 −178 | u4j-build.md |
| U4e | the room and codec path (npt-room, the server library, npt-client) | c88eb31 (11-12) | +7,937 −17 | u4e-report.md |
| U4d-F | the estimate scene's fix | adb1bab (11-12) | +663 −146 | u4df-report.md |
| U4d2 | the Wasm-in-Node hash, check.mjs's determinism step | cc8595d (11-13) | +5,219 −108 | u4d2-report.md |
| U4r | the L1 runner, -30's pre-check, the run scripts | 71ee13d (11-14) | +8,362 −106 | u4r-report.md |
| U4r-F1 to F3 | the scripts' re-check and limits; P0 judged in two calls; a runner change | f1f131b, 791bc55, 493b3d7 (11-14) | +334 −104 | u4r-report.md |
| BRAWL-1 | the bots' leash from the client's own decoded position | b30314a (11-15) | +1,139 −127 | u4r-report.md:630 |
| C62H-BOUND | -17's two lost-tick checks and their judge figures | eb2bebf to 561bf42 (11-16) | +904 −63 | u4r-report.md:1201 |
| U4f | clock and commands, option A, the cheats, the suites of -47, -20 and -22 | 25d316e (11-19) | +6,424 −309 | u4f-report.md |
| U4f-F1 | the stand-in sim takes every press edge | 1faf894 (11-20) | +239 −24 | u4f-f1-report.md |
| U4f2 | -21's forged-command test, with F-B-COUNT (a) | b4e1f43 (11-22) | +2,674 −194 | u4f2-report.md |
| **All** | | **22 commits** | **+57,758 −1,864** | |

- **His size answers:**
  - U4d: he was told on 2026-11-11 (u4d-report.md:73-77);
  - SIZE-U4R (a): history.md:2044-2047;
  - SIZE-U4F (a): :6896-6903;
  - U4F2-SIZE (a): :7255-7271.
- Each sub-unit merged after its review round or rounds and the advisor's go. Each record names its verdicts and what
  it left open.

## The marks judged here
L1 is this container: a simulated network (P0 none, P1-P4 the profiles, B1 a loss burst) and the stand-in sim. Each
row's sources are in replan-marks.md section 1, at the line given.

| Mark | At L1 | Next |
|---|---|---|
| -3 Network profiles are what they say | PASS on P1-P4 and B1; the same answers on U4c's build (9cd7c39) | R1, R2 and R3's probes (:20) |
| -17 Every decoded snapshot is the server's | PASS on P0, P2, P3; B1 INVALID, as ruled | B1: U4k; then R2, R3 (:34) |
| -57 Decoded values keep protocol precision | PASS on P0, P2, P3, B1 | done (:35) |
| -18 A loss burst heals | B1 INVALID, as ruled | B1: U4k; then R3 (:37) |
| -20 Commands arrive in time, count once | PASS, options A and B, 3 of 3 (46ca143, after U4f-F1) | R3 on the real network (:39) |
| -21 Forged commands and lag switches change nothing | PASS: 8 parts, A and B, seeds 1-3; 13 bites shown (33d246f) | done (u4f2-report.md) |
| -22 A never-acker cannot grow the server | PASS, 3 of 3 (2dae645) | done (:41) |
| -47 The client's clock is right | PASS, 3 of 3 (a4cac39) | moved to CS7; this stays as the record (:70) |
| -2 The load is the stress scene | SS PASS in every set; ES FAIL, 50.05 against 10.5 events a second | ES: carry 93 in U4k (:125-127) |
| -58 Every counter survives its wrap | not run | U4k, after his U4G-Q2 (:36, :110-113) |
| -30's lab part | pre-check recorded (d666aeb), not judged | U6d and U6j (:50, :155) |

## What moved, and what stays
- **Moved to the real game** (history.md:7577; nothing dropped or loosened; each mark measured on the real game at the
  first build that can show it):
  - U4g: its built early half stays for reference as wip/u4g-early.patch (sha256 a78e2853…);
  - U4g2's client half;
  - U4h: briefed (rev 2.1), never built;
  - R7.
- **Their marks** (replan-marks.md:62-79): -41, -42, -60, -45, -61, -62 and -47 to CS7 or CS8; -48 to -51 to S3, with
  -51's hitscan to CS8; -54 to CS9; -55 to where vehicles enter. The judge reports a moved mark as MOVED, never as
  PASS; that is the MOVED unit, after R1.
- **Kept, as U4k after R1** (replan-marks.md:135-158: about 540-1,280 lines, 800-1,900 at 1.5x, one S unit; none is an
  R1 prerequisite):
  - (b) the room's recovery after an eviction (carry 78) and the ack fix (carry 91), before R3;
  - (c) the B1 rows of -17 and -18 on U4r's own B1 set, re-run at (b)'s commit, before R3;
  - (d) -58's wrap suite, before R5. His U4G-Q2 comes first: his yes to the wording of -58's two lost-tick checks;
  - (e) carry 93's ES hit rule, before R2's ES rows.

## What stays open, and who takes it
- **Inside the plan:** the card's Open findings, each with its taker: U4f's brief 9.2's items; the readings PR's list;
  the U1 and U2 carries; C6 widened (before R3); JITTER-ORDER (parked).
- **For PR #40's closing docs pass** (STATUS follow-ups; none blocks):
  - U4f2's, by id: u4f2-report.md:159-165;
  - U4r's: u4r-report.md:171-286;
  - U4d2's: u4d2-report.md:160-177;
  - U4e's: u4e-report.md:152-165;
  - U4d-F's: u4df-report.md:100-113.
- **Security lines for PR #40's report:** u4r-report.md:287-294, u4f-report.md:467-475, u4f-f1-report.md:97-102.
- **Public items** added per crate: each record's own section (A19; engine-architect read them in review).

## What was not verified
- Everything here ran at L1: one container, a simulated network, the stand-in sim. Nothing ran on a real network path
  (L2), a run server, a browser or a load machine; R1, R2, R3 and R5 test those.
- Each record's own "not verified" lines stand (u4f-report.md:512-524, u4f-f1-report.md:113-119,
  u4f2-report.md:166-178).

## Decisions needed
- None.

## For Mustafa
- Nothing.
