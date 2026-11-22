# Advisor: check of pushed head b4e1f43 (U4f2's merge, PM-0001-21's forged-command test)

## Plan (written first)
Seat: day-session advisor agent, read only (caller's rules: no builds, no runs). Main's advisor.md governs; NPT-TRIM
(history.md:7561-7593) keeps this head review. Steps: (1) `git show b4e1f43 --stat`, parent, patch sha256, a few files'
sha256 head vs worktree, spikes/ vs a289def outside results/; (2) read R6 logs a/b, M5 log header and tail, CI check
runs on b4e1f43; (3) read round-2 netcode/test reviews and u4f2-build.md's dispositions, spot-check FP-10 and
TE-U4F2-R2-4 in the code; (4) read advisor-u4f2-parta.md:102-105 and the call sites of `kind_7_left_out`; (5) check
F-B-COUNT (a) in bots lib.rs against advisor-u4f2-r1.md:404; (6) what -21's set must hold. Then verdict and findings.
