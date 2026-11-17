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
- 1. Head 3626a37 parent 8c72ab7 parent a5baab8; since a59e0ee (last CI run) 18 commits, all [skip ci], and every changed path is under .checkpoints/ (git diff --name-only --no-renames; --raw: all 100644, no symlink or submodule).
- 2. Append-only: diff a5baab8..head on u4r-report.md is one hunk @@ -1197,3 +1197,568 @@ with no removed line. sha256 777ee782... (1199 lines) -> c5dee3fa... (1764 lines). Line 1200 blank, 1201 the heading. tail -n +1201 sha256 = e4af2d54... = the draft at head. Draft at a5baab8 = 5d8fb338... (the verified one). Insert script checks the before-sha, draft sha, prefix, cited lines 997/1008/1035/1167.
- 3. (c) diff 7526ffe: +3 -2 on u4r-report.md: the old :996 and :1007 replaced, the note added at :990. My python read of the ruling's the_c_lines: each of the three texts equals exactly one whole line (997, 1008, 990) at 7526ffe and at the head. Report at 7526ffe sha256 777ee782 (= a5baab8's). B1 lines :1035/:1167 (were :1034/:1166) byte-equal before and after. The script checks A/B 0/0/0, tallies, resyncs, verdict path, bites 48/48 against the files and exits 1 on difference. P2 met.
