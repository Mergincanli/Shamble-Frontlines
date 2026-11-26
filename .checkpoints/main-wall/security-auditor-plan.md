# security-auditor: MAIN-WALL design review (risk tier 2, advisor condition C6)
Public notes: ids, severities and class words only (his 2026-11-25 20:35 item 3). No actor list, no attack path, no
test detail; the detail is in the session scratchpad, never pushed. Read-only on GitHub; nothing committed by me.
Inputs read: plan at 6cbcd0d (after THROWAWAY-PUBLIC (a)), advisor REVISE, the design, brief and card (unpushed, re-read
after the coordinator's update), the other lane's notes at e58d049e (lines 30-90, 148-160), DECISIONS (20:35 items 2-3,
12:24 item 1, 14:09 item 6, 15:05 item 4), my memory.

## Plan (blocks)
1. Q1 does the rule close the class of gap; what stays open. DONE.
2. Q2 tests and pass marks, false-PASS risks. DONE.
3. Q3 brief safety. DONE.
4. Q4 THROWAWAY-PUBLIC (decided (a); residual only). DONE.
5. Q5 real clicks, rollback, risk 4. DONE.
6. Verdict. DONE.

## Sources (summary saved as read)
- READ the guard's coverage of settings writes (routine-guard.mjs): detail in the scratchpad.
- RAW GitHub REST "Repository rules" Markdown (docs article-body API, 2026-11-26), line 890: a note on one field's
  visibility, re-read verbatim for SA-MW-3; detail in the scratchpad.
- READ gate.yml:24,56, toolchain.yml:54: the checkout pin in the brief matches the real repository's.
- WEB GitHub REST "Repository rules" (2026-11-26): exempt = "rules will not be run for that actor and a bypass audit
  entry will not be created"; nothing on a bypass in one ruleset affecting another. Sure: high (official).
- WEB GitHub Docs "About rulesets" (2026-11-26): "Anyone with read access to a repository can view its active
  rulesets."; bypass granted per ruleset; nothing on cross-ruleset bypass. Sure: high.
- WEB GitHub Docs "Deleting a repository" (2026-11-26): "Deleting a public repository will not delete any forks of
  the repository."; restore "within 90 days". Sure: high.
- WEB GitHub Docs "Manually running a workflow" (2026-11-26): "Write access to the repository is required". Sure: high.
- WEB GitHub Docs "Managing GitHub Actions settings for a repository" (2026-11-26): "By default, all first-time
  contributors require approval to run workflows". Sure: high.
- WEB GitHub Docs "Reviewing and modifying installed GitHub Apps" (2026-11-26): the installer changes repository
  access, suspends or uninstalls; permissions are only reviewed. Sure: high.

## Findings (one line each, as found)
- Q1: W1 closes the class "an actor other than the owner's account updates main", provided T1 and T4 hold.
- Still open by class: own-account actors (by nature); settings-level changes (SA-MW-3); check identity (pre-existing,
  out of scope); stored credentials (SA-MW-9); other branches; exempt audit gap; the Disabled window (SA-MW-11).
- SA-MW-1 High: the only-his-account pass mark can be met by a refusal from the other ruleset (false PASS); no baseline.
- SA-MW-2 High: the decisive test can pass without the actor being shown exempt (false PASS).
- SA-MW-3 Critical if confirmed, pre-existing, sure medium: the wall does not protect itself; click 0 decides.
- SA-MW-4 Medium: a read-back can take a hidden field for an empty one; carried-over settings not compared.
- SA-MW-5 Medium: the unguarded test chat's limits are incomplete for a public repository.
- SA-MW-6 Medium: one decisive sub-test may be unrunnable by the test chat; no fallback on the card.
- SA-MW-7 Medium: one route per actor tested; other routes exist.
- SA-MW-8 Low: pending state at call time not recorded.
- SA-MW-9 Medium: stored-credential invariant not stated.
- SA-MW-10 Low: THROWAWAY-PUBLIC residual: deletion does not erase (forks); stated, not reopened.
- SA-MW-11 Low: the rollback reopens the class while it lasts; no time bound.
- SA-MW-12 Low: click 11's name ambiguous.
- New decisions for him: T4-BASELINE, GUARD-SETTINGS (detail in the hand-back).
- Second pass (2026-11-26): proposed texts made consistent (fallback mode, baseline pass route, log content, merge-tool
  exception, pushed wording); exact texts in the hand-back.

## Verdict
REVISE (text only; W1 holds). Exact text changes in the hand-back and the scratchpad detail file.
