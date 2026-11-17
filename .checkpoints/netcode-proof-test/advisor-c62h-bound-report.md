# Advisor: C62H-BOUND report check (P6, order item 11)

Head under check: 3626a377f95c0a9dc0edb3bce49227f73caf733d on claude/netcode-proof-test-plan-hcky5g.
Role: origin/main:.claude/agents/advisor.md, read whole. Read only; this file is my only write.

## Plan
1. Confirm the head SHA and its parents; the commit list a5baab8..3626a37 and 7526ffe.
2. Append-only check: git diff a5baab8 3626a37 -- u4r-report.md; sha256 of the file at a5baab8 and at the head; line counts.
3. The (c) diff: git show 7526ffe -- u4r-report.md against the_c_lines (P3 line, P2 line, note line); B1 lines :1034 and :1166 unchanged.
4. Read the section (u4r-report.md 1201-1764) against order items 4-12 and P1-P7, item by item.
5. Spot-check the section's key numbers against sources (runs.txt, verdict.json, verdict-bites.json, merge logs, review JSONs).
6. The six Lows: read verify-r2-full.json; diff a5baab8..head on the draft (+18 -12); judge each against the Low text and source.
7. Draft-to-report binding: the draft at head sha256 = e4af2d54...; the insert script; the section equals the draft.
8. P6: git diff --quiet ddd38ba 3626a37 -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'; exit code.
9. CI ruling (item 10): the [skip ci] rule in .claude/CHECKPOINTS.md section 5; what changed after a59e0ee.
10. C62HB-Q4 (b): read the design's C62HB-Q4; confirm or give mine.
11. Verdict, findings, conditions, to_mustafa.

## Log (one line per check)
