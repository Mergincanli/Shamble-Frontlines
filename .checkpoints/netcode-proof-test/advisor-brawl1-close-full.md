# Advisor: the close check of BRAWL-1's report section (workflow wf_631f7311-f96), 2026-11-15

Written by script, word for word, from the workflow's journal. Its notes: advisor-brawl1-close.md. Nothing below is the main session's.

## Verdict

TRUE TO THE FILES, WITH SEVEN SMALL CORRECTIONS (all Low). Every figure, hash, verdict and ruling in BRAWL-1's section is right when checked against the rulings, the builder reports, the logs under .gate/, the committed results and git. Every long quote is word for word. I found three small factual slips (F1-F3) and four record-keeping gaps (F4-F7). None of them changes a verdict, a count or a ruling. BRAWL-1 may be recorded closed once corrections C1-C7 below are made exactly as written. C8 and C9 are optional. This was a read, not a review round. I changed nothing. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-close.md.

## True to the files

What I checked (read-only: print-only python, grep, sha256sum and git; nothing written except my notes):
- Hashes: all 14 sha256 prefixes the section cites match their files. These are the plan, brief-check, points, r1, r2, M6, P3 and P0 rulings, both builder reports, the round-1 seats, the packet, the brief and u4g-brief-carry. The 3 set logs, pre-P0.log and the 7 committed results files also match (cc2f626c, a24d5856, e07e882e, 2e0bb273; 5361fd53, 21c9abbf, c2995379, 1d2ff9e5, ada5a58a, e926b812, b472784f).
- Blocks copied by script, compared line for line, all equal:
  - round 2 (:827-862) = r2:131-166;
  - the re-run sets (:982-1023) = p0:212-253. The one difference is :988, the fix the NOTE at p0:266 ordered;
  - the corrections (:1089-1093) = p0:260-264;
  - KEPT-LOGS (:1163-1169) = p0:278-284;
  - the plain words (:1175-1179) = p0:271-275, and item 1 = p0:287;
  - EVQ-LOAD (:1041) = p3:158;
  - TICK16-WORDING (:1047) = p0:135.
- Nine inline quotes are word for word in their sources: condition 18, ADV-BP-3 and its masking rule, ADV-BP-4, ADV-R1-1 and -2, TE-B1-8, FP-9 item 5, and the P3 evidence line.
- The plan and the build: every figure holds. That covers Part 1's proofs (532/610; 232.9 s with a non-cargo sum of 222.0; 6933b56a; 109 lines), Part 2, the full-test table (all 9 lines against m3.log:75-83), the pins, the size (998 = 875+123; 1,266 = 1,139+127) and the 426 lines in lib.rs.
- M1-M7, checked against git and m2/m3/m4/m7*.log:
  - b30314a at 08:09:41, 11 files;
  - M2 (B) 08:37:41-08:37:47, 15 commands; (C) 08:38:04-08:38:09, 6 commands;
  - the M2 frees 434.8 and 5,924.2 MiB;
  - M3's header, check.mjs 538/616, self 77, marks 81, redact 287/0;
  - the two df readings 8,342,093,824 and 8,342,081,536 B;
  - det-u4 234.1 s with a non-cargo sum of 222.8, 9 commands;
  - M7's five logs: times, df figures, HEAD fc93733, tree ebbfeb40.
  - A5 holds: after b30314a, only fc41111, 93509df and be41ce6 touch spikes/.
- The evidence runs: every bar and fact holds against m7-ev.log and both verdicts. That covers 693.09 / 693.18 shots a second, 878,352 and 835,238 compared, 44,395 incomplete, 27 resyncs, -57's values, the judge's 8.6 s / 1,856.3 MiB and 2.3 s / 392.9 MiB, and the two df figures.
- The re-run sets: checked against the committed verdicts and U4r's same-profile verdicts, run by run and bite by bite. Also checked: the wall, peak and bytes lines in the set logs, pre-P2.log and pre-P0.log, the redact log (7 items, 0 findings), and the raw lab.jsonl resync counts. tick16: 94 resyncs (47/46/1), all in the after phase, on 48 slots, with slots 5 and 17 once each. P3: s1 27 on 25 slots, s3 1. All true.
- Homes: every finding's home matches its ruling, and every line citation lands on the cited text. One exception: ADV-PT-L3's home list does not name it (F6).
- KEPT-LOGS' numbers still hold today: kept logs 1.94 + 5.26 + 4.20 GiB plus 0.59 GiB of evidence; free space 8,089,780,224 B (7.5 GiB).
- Item 3b's list is complete. Item 3a (the card): P2's record is corrected in place. The card holds EVQ-LOAD and TICK16 as exact pointers rather than word for word. I accept that (ADV-CL-1, my own wording).

What is not exact (all Low):
- F1: :890 quotes uptime without "0 user,".
- F2: :794 says both .diff files were committed at 7402dac. They were committed at ab3b7d7 (06:41:33).
- F3: :810 says "the full test's 13 printed lines". The 13 lines are the whole sim-only test's: the short copy's 4 plus the full test's 9 (M6 check 8). They are identical only after the stated normalisation (copy 2's report :61).
- F4: the heading at :1043 points to ":562-564". That line now sits at :566-575.
- F5: :644-646 still says the advisor's read is to come.
- F6: ADV-PT-L3's home is "the check.mjs follow-up", but that list (:264-271) does not name it.
- F7: the "next npt-lab lane" entry (:272-276) still reads as open, although BRAWL-1's Part 1 did every item in it.
- F8 (optional): :650 gives the plan's own "about 23:45 UTC", but the plan was committed at 9658c10 at 23:30:12.
- F9 (optional): the quoted For Mustafa says "committing the last results" is still to do. That was done at be41ce6.

## The corrections

Make each by Edit, exactly as written (line numbers are today's):

C1 (:890). Replace:
uptime: "08:53:06 up 18 min, load average: 2.31, 2.21, 1.54" (:26).
with:
uptime: "08:53:06 up 18 min, 0 user, load average: 2.31, 2.21, 1.54" (:26, spaces folded).

C2 (:794). Replace:
  - Committed at 7402dac, with u4r-run.sh.te-b1-7.diff and main-run.sh.te-b1-7.diff, and `bash -n` silent on both.
with:
  - The two lines were committed at 7402dac (the 05:41 autosave). The .diff files u4r-run.sh.te-b1-7.diff and
    main-run.sh.te-b1-7.diff were cut from 7402dac's own diff by fp0_diffs.py and committed at ab3b7d7 (06:41 UTC).
    `bash -n` prints nothing for both.

C3 (:810). Replace:
    - the full test's 13 printed lines are identical to round 1's;
with:
    - the sim-only test's 13 printed lines (the short copy's 4 and the full test's 9) are identical to round 1's, once
      the new dead figure is set aside and the old skip text is mapped to the new wording (:61);

C4 (:1043). Replace:
### TICK16-WORDING, corrected (it replaces the line at :562-564)
with:
### TICK16-WORDING, corrected (it replaces the sets-r2 line under "The judged runs", Corrections (Q3 (7)), above)

C5 (:644-646). Replace the bullet "- Still open: the advisor reads this section against the files before BRAWL-1 is recorded closed. That read is not a review round (advisor-brawl1-sets-p0-full.md, The commit and the close, 3c). PR #40 stays a draft, and nothing here merges to main (the same section, "Unchanged")." with:
- Closed: the advisor read this section against the files (item 3c of advisor-brawl1-sets-p0-full.md; not a review
  round) and found it true once the corrections in advisor-brawl1-close.md were made. BRAWL-1 is recorded closed.
  PR #40 stays a draft, and nothing here merges to main (advisor-brawl1-sets-p0-full.md, "Unchanged").

C6 (:267, the check.mjs follow-up: give ADV-PT-L3 its home). Replace:
:87-90, :122-125; advisor-u4r-r1-full.md:17). It also carries
with:
:87-90, :122-125; advisor-u4r-r1-full.md:17), and BRAWL-1's ADV-PT-L3 (TICK_NAMED becomes
  /_TICKS$|_TICK_COUNT$|CATCHUP/, NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with its reason, and check.test.mjs gains
  one case; advisor-brawl1-points-full.md:87-91). It also carries

C7 (:272). Replace:
- **The next npt-lab lane:** N2-1 =
with:
- **The next npt-lab lane, done in BRAWL-1's Part 1** (merged at b30314a; EA-U4R-1 in the narrow form the plan ruled,
  with no one-loop driver; the record is this report's BRAWL-1 section, "Part 1, the refactors and N2-1"): N2-1 =

Optional:
C8 (:650). Replace "written by the main session on 2026-11-14 at about 23:45 UTC." with "written by the main session on 2026-11-14 and committed at 9658c10 at 23:30 UTC (its own first line says 'about 23:45 UTC')."
C9 (after :1181, before the closing italic note). Add:
_Since the P0 ruling: P0's results are committed (be41ce6), and this section was read against the files
(advisor-brawl1-close.md). BRAWL-1 is closed; KEPT-LOGS stays open._

Do not change the quoted blocks (round 2, the re-run sets, EVQ-LOAD, TICK16, the corrections, KEPT-LOGS, the plain words). C1 to C9 all fall outside them.

## The close

YES. BRAWL-1 may be recorded closed once C1-C7 are made as written (C8 and C9 are optional). I do not need to read it again. The main session runs one print-only diff of u4r-report.md against its current state to show that only those lines changed. Then it commits the report, the card and my notes file together and pushes. On the card, mark item 3c done and record ADV-CL-1: the card holds EVQ-LOAD and the corrected TICK16-WORDING as exact pointers, and I accept that in place of word for word, because the words sit word for word in u4r-report.md, copied by script. Task #74 then moves to what my P0 ruling lists after BRAWL-1:
1. TE-B1-R2-2's runner edit, which is due now, before U4g's first run. It goes at u4r-run.sh:67 and main-run.sh:42, in the ruled form, each with a .diff and a silent `bash -n`.
2. KEPT-LOGS before U4g's build.
3. U4g's brief carries u4g-brief-carry.md and EVQ-LOAD.

Unchanged: PR #40 stays a draft, main is taken in before it is marked ready (MAIN-52), and nothing here merges to main.

## For Mustafa

In plain words: the bot fix (BRAWL-1) is finished. I checked its write-up against the raw logs, the saved results and my own rulings, and every number that decides anything is right. Seven small slips in the write-up get fixed: a shortened quote, a wrong save point for two files, a miscounted label, a stale line pointer, a status line, and two follow-ups that were not filed where they belong. None of them changes a result. After those fixes the work is recorded as closed. You have nothing to check in a browser and nothing to merge.

Decisions needed:
- **KEPT-LOGS** (your open question, unchanged): should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings are closed?
  - Why it matters: kept raw logs take about 11.4 GiB, plus 0.6 GiB of evidence runs. 7.5 GiB is free against the 4 GiB safety floor, and the next unit's (U4g's) build cache needs about 8-9 GiB. U4g cannot build until space is freed.
  - Options: (a) keep everything, and U4g cannot build; (b) delete each closed run set's raw-log folder, which frees up to about 11.4 GiB, but those runs could never be re-judged; (c) first pack each set into one compressed file and check it opens, then delete the folder. That frees about 8 GiB and keeps re-judging possible.
  - My recommendation: (c). The same rule also names the old 4 GB build cache. It comes as its own reviewed kit pull request and merges only on your go. Nothing is deleted before then.

For Mustafa:
1. Answer KEPT-LOGS (a, b or c) before U4g's build needs the disk.
