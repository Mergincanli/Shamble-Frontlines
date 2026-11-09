# U4 build plan (rev 1) review: test-engineer

Task: adversarial review of .checkpoints/netcode-proof-test/u4-build-plan.md (rev 1), against scouts u4-scout-arch.md,
u4-scout-marks.md, u4-scout-netcode.md, plan.md and the code. Focus: requirements and carries met and proven, marks
with tests that bite, judge figures, fixtures and evidence honest, nothing judged here that only a run can judge,
split testable unit by unit. No code edits, no builds, no runs. Memory: no agent-memory dir found (.claude/ has none).
Rule (checkpoint :294-295): notes by the Write tool only, no shell writes, no cd. (The first append went in by a shell
heredoc before I read that rule; this rewrite carries it unchanged.)

## Plan
1. Read the build plan in full.
2. Read plan.md U4 section, R/C carries that name U4; the marks for U4.
3. Read the 3 scouts; check each claim the plan leans on.
4. Check code file:line for the plan's claims (crate layout, existing harness, judge).
5. Findings list; verdict.

## Progress
- step 1 done: read u4-build-plan.md rev 1 in full (646 lines).
- step 3 done: scouts read (marks F1-F32, arch F1-F48, net F1-F124) and the plan's notes (M1-M25).
- step 2/4 in progress: marks.json read by python (read only) for -2, -17, -18, -20, -21, -22, -30, -41, -42, -45,
  -47, -48, -49, -50, -51, -54, -55, -57, -58, -60, -61, -62, PM-0004-67; judge eval.rs :15-75, :921-935,
  :1052-1360; README §3-§5 (:140-215); SERIES figures (:259-300, :736 Q30); figures.rs :150-240; log lib.rs :36-120;
  l1 lib.rs :1-60, :105-170, :348-365; advisor-u3q.md:158-190; advisor-ws-docs-r2.md:368-386; checkpoint :296-330.

## Sources read (summary per source, as read)
- S-RFC9002 (www.rfc-editor.org/rfc/rfc9002.html, fetched 2026-11-09, first 100,000 of 105,234 chars): §6.2.2 "When
  no previous RTT is available, the initial RTT SHOULD be set to 333 milliseconds"; §5.3 before any sample
  smoothed_rtt = kInitialRtt, rttvar = kInitialRtt / 2, the first sample resets the estimator; §6.1.2 time threshold
  max(kTimeThreshold x max(smoothed_rtt, latest_rtt), kGranularity), kGranularity 1 ms recommended; §6.2.1 restart the
  PTO timer on each ack-eliciting send or acknowledgement, backoff doubles at expiry, reset on an ack (client Initial
  exception). Sure: high (spec text).

## Hypotheses checked (H-n -> finding or dropped)
- H12 check.mjs protected? DROPPED: routine-guard.mjs:26-27 GATE_FILES lists only tools/ files; spikes/ is not protected.
- H6 -41 per option: LOW only. -41's parts select scene + profile, no cmd_timing (marks.json -41); checks are per run
  (eval.rs:1141-1160, pooled only if marked), so A+B runs pooled in one part still each must pass; per-option numbers
  come from verdict.json's per-run lines. Plan should say so.
- H19 process `server`: no U4 L1 check reads process server (marks.json, the checks above); server-lib counters land
  in which log is still unstated (nit).

## Findings (a line each as found)
- T1 HIGH Q22 x §4.7 x -22: Q22 (a) adds scene, clients and pre:PM-0001-2 to -14, -16..-25, -27 (plan :638-640;
  checkpoint :325-328), so -17, -18, -20, -21, -22's L1 parts get the precondition. README :195-200: a run must pass
  PM-0001-2's checks else INVALID. PM-0001-2 checks "a connected snapshot-receiving client on every soldier" min >= 48
  and "48 soldiers on every measured tick" (marks.json -2). §4.7 (plan :173-176) Kicks and drops a never-acker after
  one unacknowledged FullState: -22's suite is a never-acker (marks.json -22: "resynced or dropped"), so with Q22 every
  -22 run is INVALID by design. Also B1's bursts (option A runs nothing without inputs: -17/-18 B1) and the forged
  suites (-21) cut second.v.shots_count against 665-735; none analysed. The contract test (§4.12) does not name
  PM-0001-2's series. Fix: a per-mark Q22 table (which parts get scene/clients/pre), -22's part without the pre (47 + a
  never-acker by design) or a drop rule that keeps the slot's soldier and a client; the contract test resolves
  PM-0001-2's checks on every SS suite; B1 and forged suites shown to hold 665-735 by arithmetic before the runs.
- T2 HIGH -30 pre-check vs the C part: marks.json -30's C part selects {params.suite both-transports, profile P0,
  machine.role C} with NO layer (marks.json -30). A U4 pre-check run on Loopback/L1 P0 that lists PM-0001-30 and uses
  suite both-transports would be judged as the marked "C: loopback, 3 recordings x 2 transports" part (WT vs WS, U6):
  a mislabelled PASS. If it does not list -30, the judge cannot judge it, yet plan :408 puts it under "Marks moved
  (L1 parts ... judged by npt-judge --only)", and -30 has no L1 part. late-command is expect-invalid on late-zero
  (marks.json -30; eval.rs:1282-1311) and needs a part to match. Fix: the pre-check uses its own suite (e.g.
  `replay-precheck`) and does not serve -30; its PASS and both bites are a test that calls the judge's twin figure and
  late-zero rule on the pre-check's logs as fixtures (or a provisional part by marks change, the advisor's call); the
  report never calls it a -30 part.
