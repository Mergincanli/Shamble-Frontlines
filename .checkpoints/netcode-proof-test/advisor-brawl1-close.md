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
- Round 1, round 2, the corrections and follow-ups: every line, home and line citation checked (points :5, :36,
  :87-91; r1 :29-37, :188, :190, :194-211, :279-288, :342-345, :357-360, :372-375, :402-405, :407-410, The size;
  r2 :40-58; p3 :37, :158, :194; sets-r2 :30, :34; brawl1-build.md :251, :620). True.
- Re-run sets (advisor's own report words): re-checked against the committed results by print-only python: P3, P2, P0
  verdicts and bites, U4r's same-profile verdicts, set-log wall/peak/bytes lines, sha256 of the 3 set logs, pre-P0,
  the 7 results files; raw lab.jsonl resync counts (tick16 94 = 47/46/1, after, 48 slots, 5 and 17 once; P3 s1 27 on
  25 slots 76,910-77,844 measured; s3 1 at 76,803). All true. Kept logs now: U2 1.94, U4r 5.26, BRAWL-1 4.20,
  evidence 0.59 GiB; free 8,089,780,224 B (7.5 GiB): KEPT-LOGS's numbers still hold.
- Card (item 3a): P2's record corrected in place (:322-324). EVQ-LOAD and TICK16 are on the card as exact pointers,
  not word for word (ADV-CL-1 below).
- More found:
  - F4 (Low): :1043's heading still says "(it replaces the line at :562-564)"; that line is now :566-575.
  - F5 (Low): :644-646 "Still open: the advisor reads this section..." must say the read is done when BRAWL-1 is
    recorded closed.
  - F6 (Low, a home): ADV-PT-L3's home is "the check.mjs follow-up", but that list (:264-271) does not name it.
  - F7 (Low): the "next npt-lab lane" entry (:272-276) still reads as open; BRAWL-1's Part 1 carried every item in it.
  - F8 (Low, optional): plan time (see above).
  - F9 (Low, optional): the quoted For Mustafa says "committing the last results" is still to do; done at be41ce6.
  - ADV-CL-1 (Low, my wording): item 3a asked the card to record EVQ-LOAD word for word; an exact pointer to
    advisor-brawl1-sets-p3-full.md:158 and u4r-report.md's script copy is accepted instead (a third copy risks drift).

## Verdict
TRUE TO THE FILES in every figure that decides anything: no verdict, count, hash or ruling is misstated. Three small
slips of fact (F1-F3) and four record-keeping items (F4-F7), all Low. BRAWL-1 MAY BE RECORDED CLOSED once C1-C7 are
applied as written (C8-C9 optional). No further check by me: the main session confirms by a print-only diff that
only those lines changed, then commits with the card. Nothing for Mustafa beyond KEPT-LOGS, still open.

## Step 3 done; step 4 (the result) returned to the workflow.
