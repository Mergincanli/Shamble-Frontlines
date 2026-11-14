# Advisor: U4r review round 2 ruling (PR #40, netcode proof test)

## Plan
1. Read advisor.md from main (done), the round-2 results (u4r-review-r2-full.md), my round-1 ruling
   (advisor-u4r-r1-full.md), the fix-pass report (u4r-build.md, "## Fix pass, round 1: report"), the brief's merge and
   report sections, history.md's last entries (Mustafa 2026-11-14).
2. Per seat (netcode, security, test, rust, infra): check each round-2 finding against the snapshot code on
   .gate/u4r-snap2 (and the patch u4r.patch); confirm, correct or reject; final severity; ruling. A block per seat here.
3. Copy 6's two deviations (FP-2 plant (i) first run; end check 1, c6-end1.log): read the logs, rule.
4. Next steps in order for the main session (merge steps and checks, judged sets' order, disk); anything for Mustafa.

## Inputs read (13:3x UTC)
- advisor.md (main); u4r-review-r2-full.md whole (13 findings: INF-R2-1 Medium; N2-1, TE-R2-1, SEC-U4R-R2-1/-2,
  TE-R2-2/-3, RR4-L1/-L2, INF-R2-2/-3/-4/-5 Low); advisor-u4r-r1-full.md whole; u4r-build.md fix-pass report
  (:1955-2111) and FP-2/end-check blocks (:1656-1675, :1835-1870); u4r-copy6-prompt.md :34 ("If any outcome differs
  from what the ruling expects, stop and report: never re-run a failed suite into a pass (condition 5)"); brief
  sections 4 (end), 5, 6, 7 head, 12; history.md last entries (2026-11-14 11:07 SIZE-U4R (a), SIZE-CHECK, COMPACT,
  MAIN-52); card NOW block. U4r's merge conditions are NOT yet written (advisor-u4r-brief.md:441: "my merge conditions
  follow U4d2's M1 to M9 form") -> this ruling writes them (U4d2 r1 Block 9 / r2 Block 5 form read).

## Progress
- [x] inputs read
- [ ] netcode block
- [ ] security block
- [ ] test block
- [ ] rust block
- [ ] infra block
- [ ] deviations
- [ ] next steps
