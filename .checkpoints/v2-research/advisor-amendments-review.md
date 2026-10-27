advisor review of v2-research amendments and docs-after-r7
- started 18:1x UTC; reading plan, verdict, amendments
## Checked
- plan.md, advisor-verdict.md, plan-amendments.md, provenance.md, checkpoint v2-research.md; R7 report R7-MARKS text (option a = working recs, change when ADR draft comes up); brief §4 R6 row (item 2 matches it word for word); inventory: 118 rows "look brief pending".
- nproc 4. Saver stages only .checkpoints (checkpoint.mjs:151). cs/v2-research pushed (8ff1c74) -> cs/ pushes work.
- Wave-1 notes sampled: dates read, tiers, inventory rows, §6 trace present; all fetched domains on the allow list. No re-brief needed.
- Checkpoint lacks: saved prompts (cond 10), cond 9 synthesis line, security-baseline carry-forward.
- B: merges/dates on main verified (#26 f2ccc13 10-25, #25 4c24d07 10-27, #27 07ab240 17:36 UTC, VISION/brief 10-24). STATUS sections: no "Decisions"/"Routines" heading (has "Decisions pending"). ROADMAP "Next / open" queue line stale too. Issue #16 body read via REST; DOMAINS-ACCEPT comment read. CI runs on drafts. tools/node_modules absent in main checkout.
## Verdicts
- A: APPROVE with conditions; wave 1 continues, no stop, no re-brief.
- B: REVISE (branch cs/docs-after-r7; sections; codes explained; kit-v3-1 wording; ROADMAP Next/open; #16 text). Decision KIT31-STATE to Mustafa.
DONE
## Re-review: docs-after-r7 plan revision 2
- Worktree on cs/docs-after-r7 at 07ab240, clean except .checkpoints. B.1-B.8 each applied.
- Checked decision meanings against kit-v3-0 report (main, docs/reports/2026-10-24-kit-v3-0.md:160-188) and request-and-decisions.md:106-112.
  - KV3-MERGE (a) wrong in plan ("merge kit-v3-0"): it is "session merges on Mustafa's fresh merge #n after APPROVE, not his advance go".
  - DOMAINS-ACCEPT (a) too broad ("web allow list accepted"): it is six publish-anyone hosts kept on the no-prompt list, risk recorded, only once WRITE-GUARD merged.
  - WRITE-GUARD (a) missing: enforced in settings (ask rules / deny if routines allow silently) + secret-pattern fix.
  - SPEND-1, ROUTINE-EFFORT, SPIKE-SCOPE, BRANCH-2 wording OK.
- tools/node_modules in main checkout: 12 entries, no playwright (optional dep, --omit=optional) -> local smoke 5a can't run.
- .gitignore `node_modules/` (trailing slash) does not match a symlink -> worktree link would show as untracked.
- Repo is public (#16 comment quoting MANAGED fact: acceptable, STATUS carries it anyway).
- Verdict: APPROVE with 6 conditions.
DONE
