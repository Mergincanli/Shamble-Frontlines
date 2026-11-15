# Advisor: the close check of BRAWL-1's report section (item 3c of advisor-brawl1-sets-p0-full.md), 2026-11-15

A read against the files, not a review round. Nothing changed in any tree; only this notes file is written.

## Plan
1. Read main's advisor.md (done), the P0 ruling's item 3b/3c (done), the assembled section (u4r-report.md :625-1184,
   plus the three replacements at :215-219, :528-530, :566-575) and the assembly and draft notes (done).
2. Check every figure, quote and home, section by section, against: the seven rulings, both builder reports, the
   card, the logs under .gate/ (brawl1-merge, brawl1-evidence, u4r-sets), the committed results, git.
3. Check item 3b's list is complete.
4. Write the verdict: true or not; exact corrections; may BRAWL-1 be recorded closed; for Mustafa.

## Progress
- Step 1 done. Step 2 in progress (print-only reads; nothing written but this file).
- Hashes: every cited sha256 prefix matches its file (plan a7e68b90, brief-check e0cb1a99, m6 c2fc64c1, points 0c1f6f5d,
  r1 86bec63b, r2 64e361ab, p0 4b16dab5, p3 1bc5889c, build report f8df429d, copy 2 b089826a, review r1 fc979c63,
  packet 16eab462, brief 921c3243, u4g-brief-carry c3abeccb).
- Blocks by script: round 2 :827-862 = r2:131-166; corrections :1089-1093 = p0:260-264; KEPT-LOGS :1163-1169 =
  p0:278-284; plain words :1175-1179 = p0:271-275; item 1 = p0:287; EVQ-LOAD :1041 = p3:158; TICK16 :1047 = p0:135;
  re-run sets :982-1023 = p0:212-253 except :988 (the ordered NOTE fix). Nine inline quotes verbatim in their sources.
- Plan and build subsection: true, except the plan's time (its own line says 23:45; committed at 9658c10, 23:30:12).
- M1-M7, the evidence runs: every figure true against m2/m3/m4/m7 logs, the verdicts and git (A5 holds: only fc41111,
  93509df, be41ce6 touch spikes/ after b30314a; trees ebbfeb40 / 079ffffc / 07baa855).
- Found so far:
  - F1 (Low): :890 quotes uptime without "0 user," (m3.log:26 reads "08:53:06 up 18 min,  0 user,  load average: ...").
  - F2 (Low): :794 says the runner lines were "Committed at 7402dac, with" both .diff files; the .diff files were
    committed at ab3b7d7 (06:41:33), cut from 7402dac's diff by fp0_diffs.py (card, FP-0 DONE).
  - F3 (Low): :810 "the full test's 13 printed lines": the 13 are the sim-only test's (short copy 4 + full test 9;
    m6 check 8), identical after the stated normalisation (copy 2 report :61).
