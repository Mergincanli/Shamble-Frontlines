# Verifier round 2: C62H-BOUND report draft (working copy vs e2d06ec)

## Plan
- Read only. No Edit tool here: this file is rewritten whole with Write after each check.
- 1. Round 1's W1-W5 and M1: RESOLVED / NOT RESOLVED with evidence (git log b423a7d; card at 67de11e; history.md
  :2113, :2131, :2163, :2177, :2189; c62h-bound-build.md:79-81; .gate/u4r/c62h-bound/merge.log; P6 check run here).
- 2. Every added/changed line of `git diff e2d06ec -- draft build-notes`, claim by claim: P1-P7 vs the ruling's
  conditions; the decision entry (C62HB-Q4 in c62h-bound-design-full.json, brief :589-596; marks.json PM-0001-58 parts;
  section 14 (ii) + its gaps fixture; advisor-c62h-review-full.md:90); For Mustafa item 4 vs the card; P6 check: git
  rev-parse HEAD, git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'.
- 3. Outside the hunks unchanged: e2d06ec's draft sha256 9397900…296c77, 538 lines; diff hunks only.
- 4. Drafter's gaps list in order; the last one: u4r-report.md :997, :1008, :1035, :1167, section ends :1199.

## Results (one line per check)
- Setup: e2d06ec draft sha256 9397900330ce…296c77, 538 lines (equal to the brief). Working draft 558 lines, sha256
  5d8fb338…94c3, equal to the draft committed at HEAD. HEAD moved during this round by the session's saves:
  40dcbb7 (02:42:19), then 67718a7 (02:45:16, the fix pass committed), then a5baab8 (02:45:46, these notes). The
  draft's bytes did not change while I checked (5d8fb338 at the start and at a5baab8).
- W1 RESOLVED: `git log -1 --format='%ad | %cd' b423a7d` = 2026-11-16 12:18:58 +0000 both; draft :48 "(12:18)";
  build notes :243 "12:18 (git's time, 12:18:58; first written here as 11:54)". 12:18 truncates like 11:01 (11:01:06).
- W2 RESOLVED: key :11 pins card lines to 67de11e. Card at 67de11e (4129 lines): U3C-HELD-1 :3760-3773, CHECK-NFF-1
  :3774-3775, ten Lows :3776-3779, 17:09-20:36 entry :3198-3211, slip 16 :3209-3211, 12:44-14:22 :3212-3217, slip 17
  :3601-3602, commit 4 / boot :55-56: every draft ref (:242, :248, :454, :506, :510, :511, :519, :521) equals them.
  No old ref (3746, 3184, 3587, 3762) and no "11:54" left in the draft (grep exit 1).
- W3 RESOLVED (one Low): key :9-11 now says Mustafa's word times are Amsterdam and gives UTC for each: 16:35 / 20:50
  of 2026-11-15 = 15:35 / 19:50 UTC (history.md:2131, :2163), 13:35 = 12:35 UTC (:2113), 21:50 / 22:09 of 2026-11-16
  = 20:50 / 21:09 UTC (:2177, :2189): all equal. LOW: the 13:35 rule is given no date; it is 2026-11-15 (:2113), while
  the key's default date is 2026-11-16 "unless named". Nothing in the draft depends on its date (:174 cites
  history.md:2127).
- W4 RESOLVED: draft :113-114 "one each, except CB-P6, which took two: a `prev_link` vector, and room_tick passed
  `std::mem::replace(...)`; c62h-bound-build.md:79-81" = build notes :79-81 ("a `prev_link` vector and room_tick passed
  `&std::mem::replace(&mut prev_link, link.clone())` ... Both edits restored").
- W5 RESOLVED: merge.log rows :8-11 fmt / clippy / clippy-wasm32 / doc PASS, :12 FAIL test, :54-59 deps / wasm / refs /
  marks / self / redact PASS, :60 "check: 1 step(s) failed", :61 "exit 1: node check.mjs", :77 "--- end 16:16:23 UTC,
  exit 1"; no det-u4 or build lines. Draft :209-213 says exactly that. The call is main-run.sh (build notes :428,
  brief :424).
- M1 RESOLVED: draft :396-399 carries P6 whole (the advisor's check naming the full SHA before Mustafa; the spike-tree
  command verbatim as the ruling's P6). My run: `git rev-parse HEAD` = a5baab8c3c5ef2a57670c0f94b93679f68bd3bae;
  `git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exit 0. At e2d06ec:
  exit 0 (the draft's claim holds). LOW: "e2d06ec, the branch head when this was written": the P6 line was written
  after the cut (02:37-02:45; `git diff e0fd859 67718a7` adds it), when the head was e0fd859 / 40dcbb7; e2d06ec was
  the head of the round-1 text. The result is the same at every one of these heads (checkpoint-only commits).
- P1-P7 vs ruling conditions (advisor-c62h-bound-p3-full.json :60-68, sha256 facf298e… 71 lines): P1 = ruling P1 (card
  :157-159 at 67de11e "P3 COUNTS AND P2 COUNTS (s1 and s3 under carry 58", citing the ruling; build notes :527-533;
  draft section 11 cites it): OK. P2 = ruling P2 (section 13): OK. P3: OK (the carry-58 reading and E1 tie ride in
  order item 9, carried in section 11). P4: build notes :515-533 hold P3's end 1029.4 s / exit 0 / 995.8 MiB, 8ca774a,
  set-check-P3 32 PASS 0 FAIL, both flags, the ruling; added in 9a0e687 at 21:55:59, before the draft (from 21:59;
  first committed b3fb754 22:11:10): OK. P5: LOW, the summary drops "the c-u2 folders" of ruling P5, but section 14
  (:437-438), which it points to, names them. P6: see M1. P7 = ruling P7 (C8 in section 6 :240; capacity side
  :388-389): OK.
- Decision entry, the code: design JSON :170-174 C62HB-Q4 ("Should -58's L1 wrap parts (P0 and P3) get the same two
  checks?"; (a) not in this lane; (b) offer Mustafa a follow-up decision after this lane's sets PASS, which would also
  need -58's L1 P3 wrap runs re-run; recommendation "(a) now, with (b) offered in the report"); brief :590-591
  ("offered to Mustafa as a later decision now that both sets pass (C62HB-Q4 (b))"). Draft's explanation equal: OK.
- Decision entry, -58's parts: marks.json :4148-4180 PM-0001-58 "Every counter survives its wrap", parts "L1 P0" and
  "L1 P3", select params.suite "wrap", min 3 each (min counts valid runs: the judge's "1 of 3 valid runs", section 9);
  its checks hold no lost-tick check. "two L1 parts, P0 and P3, read on the "wrap" suite's runs (at least three valid
  runs each, marks.json)": OK. The spike tree is equal to ddd38ba's (P6 check), so HEAD's marks.json is the lane's.
- Decision entry, INVALID reason: section 14 (ii) :439-442 (shown by the gaps fixture, not by old logs);
  crates/judge/tests/engine.rs:1642-1684 c62hb_eight_gaps_read_on_both_figures, case 1 removes
  tick.s.snap_link_lost_count from every room log -> `!o.valid`, both figures "Gap -", phrase "no room log gives
  tick.s.snap_link_lost_count". "runs logged before this lane lack the network's record and read INVALID on the two
  checks (section 14 (ii))": OK. "the L1 P0 wrap runs would need it too" is marked as the report's own reading (the
  design names P3 only): OK, and it follows from case 1.
- Decision entry, U4g: advisor-c62h-review-full.md:90 "U4g moves ... (6) and -58. Its plan also says how a B1 building
  entry reads in -17 ... marks.json changes are likely." Option (b) equal: OK. Recommendation "(b), with U4g": the
  drafter's own, stated as the recommendation: OK.
- For Mustafa item 4: card at 67de11e :164-180: PR #54 opened 21:57 UTC, "Its review round DONE", "C4: ready for review
  at 22:11 UTC ... stopped for his go on head 20f9d59918eea1e3431ae830f72510f83f6424c7"; card now :3217-3218 "PR #54:
  unchanged (... head 20f9d59 ...)". "its review round is done, and it has been ready since 22:11 UTC on 2026-11-16":
  OK. LOW: "came to you on its own" has no record (the card says "stopped for his go"; no line records a message to
  him about it); and 22:09 (Amsterdam, by the key) sits next to 22:11 UTC, which reads as two minutes apart (they
  are an hour and two minutes apart). The head SHA is not named (not required).
- Build notes hunk (:243): the only change is the time, as stated: OK.
- Check 3, outside the hunks: `git diff --numstat e2d06ec` = draft +47 -27, build notes +1 -1; 538 + 47 - 27 = 558 =
  wc. Every hunk read (11 in the draft: key, commit 2, CB-P6, first call, U3C/NFF refs, P1-P7, Lows ref, two process
  refs, slips refs, Decisions, For Mustafa 4); the rest of the draft is e2d06ec's text. Hunks 3 and 4 rewrap lines
  past 120 characters (LOW, polish: :115 and :213; the build notes :243 too).
- Gaps (drafter's list, in order):
  1. Shared taker for the ten Lows: draft :454 gives the shared taker with the card ref now :3776-3779 (67de11e),
     as the merge-stop ruling words it: acceptable.
  2. C62HB-Q4 (b) recommendation and P0 wrap runs: the recommendation is the report's; the P0 point is now labelled
     "(this report's reading)" next to the design's P3-only text: acceptable.
  3. history.md :2133 vs :2146: draft cites :2143-2146 (C62H-BOUND (a), the (c) words at :2146) and :2146 in section
     13: acceptable.
  4. No run log of c62h_bound_c_lines.py: section 13 cites 7526ffe (+3 -2, 76680d3a -> 777ee782) and claims no log;
     the lines equal the_c_lines byte for byte and their numbers equal runs.txt (round 1): acceptable.
  5. 5,307 MiB before P3 (draft :305, round-1 text): the notes :510 and the ruling's order item 9 give 5,307; no log
     holds it (pre-P3-c62h.log :2 "free 5306 MiB"; 5,563,871,232 B = 5,306.1 MiB); 1 MiB, no outcome against 5,100.
     Acceptable as round 1 ruled; LOW: the draft could name the logs' 5,306.
  6. CI on ddd38ba: section 17 :501 says it is the main session's relay; round 1 read the six check runs (all
     success): acceptable.
  7. PR #54's state: item 4 now gives the review round done and ready since 22:11 UTC, as the card: acceptable (the
     Low above).
  8. u4r-report.md refs: the file is 1199 lines, sha256 777ee782… (the (c) script's after); :997 = -17 L1 P3's Bounded
     line (under :996, "The re-run sets" :985), :1008 = -17 L1 P2's Bounded line (under :1007), :1035 = "- -17 L1 B1
     stays with U4g's judged B1 runs.", :1167 = "- **-17 L1 B1** stays with U4g's judged B1 runs (...)": all as the
     draft says (header :7, section 13 :428). BRAWL-1's section (:630) runs to :1199, the file's end; the insert script
     appends at the end, bound to 777ee782 and the draft's sha256: the refs hold.

## Verdict
PASS: W1-W5 and M1 resolved; every changed line checked against its source holds; outside the hunks unchanged.
Lows (do not fail): the 13:35 rule's date; "e2d06ec, the branch head when this was written"; P5's summary omits the
c-u2 folders; For Mustafa 4's "came to you on its own" and the 22:09 / 22:11 UTC pairing; 5,307 vs the logs' 5,306;
long rewrapped lines. About 75 claims checked.
