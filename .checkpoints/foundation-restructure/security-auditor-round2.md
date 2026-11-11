Task: security-auditor ROUND 2 review of foundation 2.1a (PR #50), delta 3c98ad5...76544aa (F1 fix and what came with it).

## Plan (blocks, in order)
B1. Read my round-1 F1 finding (security-auditor-review.md) and the diff stat 3c98ad5...76544aa.
B2. Review the new gate.mjs block (3f29cb8) against F1: both directions, case, symlink, dir-named-like-page, Windows names, other page names, fail-closed, move-only scope, A-1, v2 limit.
B3. Decoy evidence (evidence-f1/decoy-*.txt): read; reproduce one run in my own worktree if in doubt.
B4. His words: Appendix A3.3-A3.5, DECISIONS entries, L-2 labels.
B5. Weakening of guard/rules/merge route in the delta (kit/gate files touched?).
B6. Verdict section. DONE.

## Notes
- B1 done: delta outside .checkpoints = tools/gate.mjs (+10/-1), docs/DECISIONS.md, ROADMAP, STATUS, STATUS_ARCHIVE, the report. My round-1 F1: flip -> decoy (clean legacy/ + broken root page) passes both CI jobs; fix asked = case-insensitive listing of the not-chosen folder when base.prefix !== BUILD_PREFIX.
- B2 reading: new block gate.mjs:243-249 (76544aa): if base && base.prefix !== BUILD_PREFIX, readdirSync(join(ROOT, base.prefix)) filtered /^(index\.html|editor\.html|shamble-.*\.js)$/i, any hit -> die (exit 2). Exactly the fix I asked. Skipped when --smoke-only (base null): the decoy is caught only by the gate job -> needs gate job required + base resolved (check gate.yml). Candidates to probe: trailing dot/space names ("index.html." not matched by the regex); `legacy` as a regular file in the revert direction (readdirSync ENOTDIR -> uncaught -> exit 1, not 2: still fail closed); dir named index.html (matched -> exit 2, false positive, fine); post-2.1b root page with no flip (v2 limit, by design).
