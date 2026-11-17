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
- W3 RESOLVED (one Low): key :9-11 now says Mustafa's word times are Amsterdam and gives UTC for each: 16:35 / 20:50
  of 2026-11-15 = 15:35 / 19:50 UTC (history.md:2131, :2163), 13:35 = 12:35 UTC (:2113), 21:50 / 22:09 of 2026-11-16
  = 20:50 / 21:09 UTC (:2177, :2189): all equal. LOW: the 13:35 rule is given no date; it is 2026-11-15 (:2113), while
  the key's default date is 2026-11-16 "unless named". Nothing in the draft depends on its date (:174 cites
  history.md:2127).
- W4 RESOLVED: draft :113-115 "one each, except CB-P6, which took two: a `prev_link` vector, and room_tick passed
  `std::mem::replace(...)`; c62h-bound-build.md:79-81" = build notes :79-81 ("a `prev_link` vector and room_tick passed
  `&std::mem::replace(&mut prev_link, link.clone())` ... Both edits restored").
- W5 RESOLVED: merge.log rows :8-11 fmt / clippy / clippy-wasm32 / doc PASS, :12 FAIL test, :54-59 deps / wasm / refs /
  marks / self / redact PASS, :60 "check: 1 step(s) failed", :62 exit 1 node check.mjs, :77 "--- end 16:16:23 UTC,
  exit 1"; no det-u4 or build lines. Draft :209-213 says exactly that ("check.mjs's test step failed ... ran its later
  steps ... all PASS; every other step passed too ... ended "check: 1 step(s) failed". The main-run.sh call stopped
  there"). The call is main-run.sh: build notes :428, brief :424.
- M1 RESOLVED: draft :396-399 carries P6 whole (the advisor's check naming the full SHA before Mustafa; the spike-tree
  command verbatim as the ruling's P6). My run: `git rev-parse HEAD` = a5baab8c3c5ef2a57670c0f94b93679f68bd3bae;
  `git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exit 0. At e2d06ec:
  exit 0 (the draft's claim holds). LOW: "e2d06ec, the branch head when this was written": the P6 line was written
  after the cut (02:37-02:45; `git diff e0fd859 67718a7` adds it), when the head was e0fd859 / 40dcbb7; e2d06ec was
  the head of the round-1 text. The result is the same at every one of these heads (checkpoint-only commits).
- P1-P7 vs ruling conditions (advisor-c62h-bound-p3-full.json :60-68, sha256 facf298e… 71 lines): P1 = ruling P1 (card
  :157-159 at 67de11e "P3 COUNTS AND P2 COUNTS (s1 and s3 under carry 58"; build notes :527-533; draft section 11 cites
  the ruling): OK. P2 = ruling P2 (section 13): OK. P3: OK (the carry-58 reading and E1 tie ride in order item 9,
  carried in section 11). P4: build notes :515-533 hold P3's end 1029.4 s / exit 0 / 995.8 MiB, 8ca774a, set-check-P3
  32 PASS 0 FAIL, both flags, the ruling; added in 9a0e687 at 21:55:59, before the draft (from 21:59; first committed
  b3fb754 22:11:10): OK. P5: LOW, the summary drops "the c-u2 folders" of ruling P5, but the section it points to
  (14, :437-438) names them. P6: see M1. P7 = ruling P7 (C8 in section 6; capacity side :385-386): OK.
