# U4r review, round 2: netcode-architect seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce
(checked by sha256sum at start). Packet u4r-r2-packet.md sha256 b1526bde... and u4r-r2-packet-tests.md sha256
637249a5... (both checked). Runner u4r-run.sh sha256 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + new patch), .gate/u4r-snap1 (round 1, 572c4f69).
No cargo or check.mjs run: no finding needed one; nothing went through the runner. No file but this one written.
(This file is rewritten whole with Write each time.)

## Plan
1. Ruling "Round 2" whole: my list = FP-1's session ending, FP-4's verdict causes, FP-5 (g), NC-U4R-2's refusal.
   Plus confirm NC-U4R-1..7 fixed/deferred as ruled (1,2,3,6,7 fix now; 4,5 follow-up).
2. Read packet (diff 572c4f69 -> new), tests file.
3. FP-1: scenario.rs Server::io closed branch, module text, new test + plant log.
4. FP-4: precheck.rs verdict + doc, new unit test + plant log.
5. FP-5 (g): c-u4r-runs.mjs overflow resync flag, late-command whole-line check; helper test output.
6. NC-U4R-2: scenario::parse refusal + parse case.
7. NC-U4R-7: bots lib.rs docs. NC-U4R-4/-5: report's follow-up list.
8. Report's claims vs c6-*.log for my items.
9. Verdict.

## Progress (all done)
- 1, 2: done.
- 3. FP-1: scenario.rs:492-505 (snap2) closed branch after the pump loop (:427-488), so the pump drained what had
  arrived (pump.rs:405 drains every arrival; returns only after a Join, which the outer loop re-pumps, or on close).
  leave + owner.remove only when owner == id; push (slot,"closed"). Module text :21-27. L1 state() Closed only when an
  end is dropped (l1 lib.rs:1740-1745, :887-889, Drop :1761-1765): no idle timeout and bursts never toggle `open`, so
  no honest run reaches it; c6-regen-precheck.log reproduces SS-P0-s4's manifest sha256 6933b56a... and c6-det-u4.log
  PASS (both after the last source edit 12:12). Room::leave (room.rs:690-695) frees roster + clients[slot]. Test fp1
  (scenario.rs:2148-2213): passes (c6-fp12-1.log 11:23:26); plant (leave removed) fails both cases by name
  (c6-plant-fp1.log 11:23:41). Under the plant (a)'s rejoin is still admitted ("new session 1") with no "replaced":
  drop_slot (:408-422) pushes only when it finds an old session, which FP-1 already removed -> N2-1.
  L1 close semantics (l1 lib.rs:1683-1695): in-flight client->server data after a client drop is discarded when the
  server drops its end at the next io; matches the ruled "what had arrived"; unreachable at U4r. Observation only.
- 4. FP-4: precheck.rs:505-560 verdict, causes refused_logged (:518-520) + ended (:521-526); doc :505-508, module
  :37-38; test fp4 (:846-893) passes (c6-fp4-1.log 11:30:02; the clippy type_complexity failure there was fixed by the
  Doctor alias, end checks green); plant (ended removed) fails at the ended case by name (c6-plant-fp4.log 11:30:23).
  The late-command line is unaffected: refused_logged > 0 only alongside rewinds_refused > 0 while the slot lives;
  precheck_bite_late_command (:995-1027) asserts the whole line and passes in both end checks (609 bites).
  RR3-L4's held()/release() refactor (:176-195, :218-235) is equivalent to the old inline code (read side by side).
- 5. FP-5 (g): overflowFlag c-u4r-runs.mjs:152-155 applied :362 on every judged run with its marks (measured runs
  list -17, plus -18 on B1; -17 bites list -17); the summary counts resyncs in every phase (scenario.rs:944-945), so
  tick16's after-phase resyncs are flagged. pairAsExpected :161-164, LATE_COMMAND_LINE :112, applied :345; precheck
  prints exactly one "result: " line, last (precheck.rs:772), exit 0 on any result (main.rs:271-277). Helper test 4/4
  (c6-fp5-helpers.log 11:36:49).
- 6. NC-U4R-2: scenario.rs:1466-1474 in parse after Bites::of; late_command true only for LAB_BITES[1] (:231); no
  crate's list holds the id (grep: only judge tests); exit 2 via main.rs:226-231; precheck takes run_leg's own
  Bites::of (precheck.rs:312-321), never scenario::parse. Test fp6 (:2434-2452, bites build).
- 7. NC-U4R-7: bots lib.rs:29-30, :49-52 (no time or speed claim). Report: u4r-build.md:1943-1944, :2070-2071.
  NC-U4R-4, -5: deferred, listed u4r-build.md:1936-1939 with their homes.
- 8. Report lines for my items match the code (FP-1 :492-506, FP-4 :509-528, (g) :152/:161, FP-6.1 :1468-1474) and
  the logs (plant failures, end checks c6-end2/3 "all 11 steps pass", 531/609/77). det-u4 235.0 s (non-cargo 223.7 s):
  the only hot-path change under `npt-lab hash` is FP-1's O(1) state() read per session per instant; the report
  leaves the split unmeasured, as it says. No finding.

## Findings (round 2)
- N2-1 Low: scenario.rs:2195-2213 (fp1 case (a)'s rejoin asserts) - "no (0, replaced)" cannot fail; the roster half of
  Room::leave is unchecked. Fix: add `server.room.roster().is_live(slot)` to closed_checks (:2150) as a fourth named
  check (room.rs:568, roster.rs:80). STATUS follow-up under the budget.

## Verdict
APPROVE WITH FINDINGS (one Low; NC-U4R-1, -2, -3, -6, -7 fixed as ruled; -4, -5 deferred as ruled).
