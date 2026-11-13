# Advisor ruling D1-R2 (round 2 on route A: "*", the row's wording, the panic clause, the fix pass) - notes

Seat: advisor agent, day session, run as a general agent reading main's .claude/agents/advisor.md (origin/main 2aa5e9c;
the branch changes that file, K4, so main's copy governs).
Worktree: .gate/foundation-toolchain-wt, branch claude/laughing-pasteur-fwya24, HEAD 8523d50.
Content under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (4bccf55..8523d50 touches only .checkpoints/: checked,
`git diff --stat 4bccf55 HEAD -- . ':!.checkpoints'` is empty).
Scratch only under /home/user/Shamble-Frontlines/.gate/advisor-d1-r2/. No tracked file changed.

## Plan
1. Read the evidence first: review-r2-findings.md (table + TE/EA full texts), review-r2-verdicts.json (4 skeptic
   verdicts, exact wording), skeptic-r2-R2-D1..D4.md, the four reviewers' notes (RR/SA/TE/EA r2), session-std-share.txt.
2. Read the head's texts the drafts touch: Cargo.toml comment, docs/adr/README.md:234 row (OVERFLOW-ALL), ADR 0002
   part 4 / PM-0002-53 / Appendix A R5-RUSTLINT-REC4 (0002:1586), the report (Outcome, Plan and approval, As built,
   E-rows, OVERFLOW-* follow-ups, the decision entry :293-310), STATUS (2.2 bullet, :65-66, the 120-line cap), REPO_MAP,
   ROADMAP, STATUS_ARCHIVE overflow lines.
3. Then the candidate: fix-r2-drafts.md (A, B, C, D1-D10, E, F, G).
4. Own checks where a ruling rests on a mechanism claim: (a) re-run R2-D1's share-generics repro in scratch (opt-level
   0/1/s/z vs 2/3; workspace crate's own sum() via a dependency's unchecked copy), with and without "*"; (b) whether
   dropping "*" closes it; (c) the std-share claim; (d) column widths of draft A's lines; (e) STATUS line count.
5. Answer Q1-Q7 with exact approved texts; verdict, conditions, escalations. Severity on the one scale. Last fix pass:
   anything below High that arrives now becomes a report line or follow-up.

## Findings (one line each, as done)
