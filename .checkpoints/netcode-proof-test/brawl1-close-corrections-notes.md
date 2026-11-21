# Notes: applying the advisor's close corrections C1-C9 to u4r-report.md (2026-11-15)

Source: advisor-brawl1-close-full.md, "The corrections" (:49-100).
Target: u4r-report.md (tracked; clean at start, 1184 lines).

Plan:
1. Read each target line; Edit with the exact words; log each here.
2. Order: C9 (end), C4, C1, C3, C2, C8, C5, C7, C6 (bottom-up, so the line numbers above stay valid).
3. Print-only check: git diff -U0 against the pre-edit commit (note any autosave commit in between).

Baseline: HEAD was 9f7674c when the task started; two checkpoint commits landed before the first edit (c4fe8d1, f691559),
neither touching u4r-report.md (git diff --stat 9f7674c f691559 -- u4r-report.md: empty). No commit after f691559
touches it. Diff base used: f691559 (= 9f7674c for this file).

Pre-edit check (grep -cF, print-only): every target text found exactly once, at the advisor's line numbers
(:890, :794, :810, :1043, :650, :267, :272, :1181, :644). C5's bullet is wrapped over :644-646 in the file; with line
breaks folded to spaces it equals the advisor's one-line quote word for word, so it counts as found.

Log (all applied with Edit, exact words; new line numbers after all edits):
- C9: applied. New 1189-1190 (+ blank 1191), after "1. Answer KEPT-LOGS" (now 1187), before the closing italic note.
- C4: applied. Old 1043, new 1049 (heading only; the quoted TICK16 line untouched).
- C1: applied. Old 890, new 896.
- C3: applied. Old 810, new 815-816.
- C2: applied. Old 794, new 797-799.
- C8 (optional): applied. Old 650, new 653 (one line, as given; about 156 characters, not rewrapped).
- C5: applied. Old 644-646, new 647-649.
- C7: applied. Old 272, new 274-275 (the rest of old 272 follows "N2-1 =" on 275, not rewrapped).
- C6: applied. Old 267, new 267-269 (the rest of old 267 follows "It also carries" on 269).

Diff check: git diff -U0 f691559 -- u4r-report.md: 9 hunks, numstat 19 added / 10 removed, 1184 -> 1193 lines.
Each hunk is exactly one correction; no other line changed; no hunk inside the quoted blocks.
