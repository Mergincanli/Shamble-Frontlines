# Advisor plan review of kit-v3-1, round 2 (2026-10-27): APPROVE, two conditions
Re-review of revision 2's changed parts (cs/kit-v3-1 at b75bc2b). Summary of the verdict as returned:
- Every round-1 change applied correctly: threat model (R1a, R1b), PR-HOME (R1c), guard scope (R3a–c, R4, R5, R2), the
  findings ledger (R6; the deferred sha256 pin is acceptable with its stated reason), the decisions taken, C1–C8.
- Condition 1: prove PR-HOME in a real pull-request run. Part B row: the run's first command shows HEAD = origin/main,
  not the test PR's head; if it shows the PR head, RESIDUAL goes to Mustafa before the PR Advisor is switched on.
- Condition 2: check that the trigger fires at all (it did not fire when PR #27 was marked ready). Part B row: the PR
  Advisor fires on the test PR's ready event; if not, the infra-engineer finds out why before switching on.
- Nits: the report path after deny plus interrupt may need a fallback (night: the next run's "Cut off" note; PR Advisor:
  a ready PR with no review comment); MANAGED and RESIDUAL need their "why it matters" lines (use the round-1 wording);
  C3b should say a sample of the real-git cases runs on throwaway repos.
- Decisions needed: none new (MANAGED; RESIDUAL conditional). For Mustafa: PR Advisor off until the test run passes;
  its two filters; the environments and the PR-HOME setup script when the session sends the steps; answer MANAGED;
  later, reconfirm the merge.
All conditions and nits are applied in plan.md.
