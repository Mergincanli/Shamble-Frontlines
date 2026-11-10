Task: npt-u4e-brief step 3 of 3 - netcode-architect revises the U4e builder brief to rev 2 (answering test-engineer-u4e-check.md). Started 06:27 Amsterdam; rev 2 due on disk 06:50.

Plan (blocks, in order):
B1 read the check (test-engineer-u4e-check.md)
B2 read the draft rev 1 (u4e-brief-draft.md)
B3 verify Medium findings against the cited plan lines at df8975b (only where needed, time box)
B4 write rev 2 whole (no Edit tool in this session: one Write of the whole file, so a cut-off leaves rev 1 or rev 2,
   both consistent), change list at the top, header -> DRAFT, rev 2
B4b second whole Write: e5b's precondition (e5a's age >= 32) and the change list's honesty line on which cites I checked
B5 final notes + DONE

Notes:
- The Edit tool is not available here; Write only. Rev 2 goes in as whole-file Writes.
- B1, B2 done (check read: 11 Medium, 12 Low; draft rev 1 read, 464 lines).
- B3 done. Verified at df8975b: plan :289-290 (burst_eviction = whether B left the ring: same in both builds -> F1 holds);
  :286 (32 entries at 30 Hz -> F2 holds); :775 (U4f lists the server-side checks -> F3 holds); :779-782 (-22 is U4f's
  -> F15 holds); :881-882 (bites planted by a scratch script -> F7 holds); :454 (lt-tick-offset's check "0 untraced
  corrections" -> F13 holds); :443-444 (tick16 / client-rounding proof text -> F12 holds); :397, :720 (per-process logs:
  F16 holds); advisor :449-452, :459-460 (F17 holds); :707-712 (§5 U4e text); :773 (U4f depends on U4r); Lanes
  :824-827 (freeze words, drop non_finite -> F22, F18 hold); check.mjs :1321-1331, :1378-1385 (cargoCounts sums
  test result lines; two runs -> F21 holds).
  F8: precedents for CARGO_NET_OFFLINE and the df stop at df8975b: u4s-fix-review-rust.md:6-7 (CARGO_NET_OFFLINE=true;
  df -h /home/user, stop < 2 GB), u3d-build.md:7 (df, stop under 2 GB), u4s-review-rust.md:30. Accept F8 with these cites.
  Not opened by me: F19's :632, F20's :833-834, F16's :627, advisor :361 (taken from the check; said in the change list).
  Decision: all 23 accepted (none rejected). Q7 and Q9 dropped (codes kept, marked answered); Q1 narrowed to a confirm;
  Q2 gains option (c), recommendation now (c); Q4 options reworded (F14); Q6 (a) names U4f (F15); Q11 added (F3).
  F1 resolution: the check's two fix parts conflict for one test (e5 "passes in both builds" vs adding the defaults
  assertion the bite breaks), so e5 is split: e5a (age >= 32, burst_eviction = 1, both builds pass) and e5b (first
  snapshot after the gap coded against the defaults, fails by name under no-fallback; runs under e5a's precondition).
- B4 done 06:32 (rev 2 written whole). B4b done 06:35 (second whole Write; git diff --stat: 199 insertions, 85
  deletions against ea2dd8a's file). Consistency edits beyond the check (no new scope): e10 gains the Pong pin (Q9's
  answer applied), section 6 items 3 and 5 follow e5's split and F21, R28's use column follows Q7's answer, intro
  paragraph counts the codes, 2.1's -58 bullet and 2.6's lt-cmd-seq/lt-press bullet follow Q1's narrowing.

## Answer (from these notes)
- Counts: 23 findings, 23 accepted (11 Medium, 12 Low), 0 in part, 0 rejected. F1 accepted with e5 split into e5a
  and e5b, because the check's two fix parts cannot hold for one test.
- Advisor questions now: open Q1 (confirm only: plant lt-cmd-seq and lt-press, the plan's own rule), Q2 (lt-tick-offset:
  rec. new option (c), the rebuild and unit proof in U4e, the judged run after U4g, U4r's -58 judges five), Q3 (bites-only
  BITES lists in room/server), Q4 (tick16 id home; (a) leaves carry 24's tick16 half unmet, (c) meets it), Q5
  (rewind_at outside the ring before U4h), Q6 (event cap; U4f measures before -22), Q8 (client hash-map ban), Q10
  (-45/-62 ids, server-echo), Q11 new (U4f :775's server-side checks read as -21 counters and runs; U4e builds the
  checks). Answered and dropped (codes kept): Q7 (plan §4.12 :397-398, U4r :719-720), Q9 (advisor :449-452, :459-460).
- Left open: the day-session placeholders (<base>, <floor>, the recheck table); the cites the check gave that I did
  not reopen (F16 :627, F18 advisor :361, F19 :632, F20 :833-834); not committed (the session commits).

DONE
