# u4r_fold.py second check: notes (workflow agent, opus)

Task: check, adversarially, the revised u4r_fold.py's dry run against advisor-u4r-brief.md (L1) and advisor-u4r-fold.md.
Read-only: dry run only (`python3 -I`, no --apply); no commit, no switch.

Inputs hashed at start (17:44 UTC):
- u4r-brief.md bcaef383 (1,625 lines): as the task says.
- advisor-u4r-brief.md 35f281bf (463 lines): as the task says.
- advisor-u4r-fold.md d4417040 72d5… (217 lines): starts d4417040, as the task says.
- u4-build-plan.md 1e631c55: as the task says.
- u4r_fold.py 9e3a123c fe923fb0… (693 lines): NOT the task's a9196126… (520 lines). a9196126 is the PRE-revision
  version (in git from bb412c8 17:04 to b975871 17:34; the first check u4r-fold-check.md failed it); 9e3a123c is the
  reviser's final (its notes, V5; autosave 598472c 17:44:53). This check reads 9e3a123c, the revised script.

## Plan (blocks)
- C1. Read advisor-u4r-fold.md whole; advisor-u4r-brief.md :74-175, :176-382, :415-420; the questions, the first
  check, the reviser's notes.
- C2. Run the dry run; read its whole output (diff).
- C3. Each id bullet (F1-1..F6-9, N1..N7b, F-D13): its hunk word for word (print-only python3 -I reads).
- C4. Replaced items (A2.1-A2.4, A2.7, A2.8, Q9.2): not in old form.
- C5. The twelve table rows cell by cell; third cells vs :109-112, :149, :121-126.
- C6. The record paragraph vs what the fold does.
- C7. "by U4R-Q" count (any case) in the folded text.
- C8. Any other line changed (unexplained hunks).
- C9. Result.

## Log
- C1 done: fold ruling read whole (217 lines); advisor-u4r-brief.md :60-424 read; questions, first check, reviser's
  notes read; the script (9e3a123c, 693 lines) read whole.
- C2 done: dry run exit 0, 678 output lines, last "dry run: nothing written"; brief still bcaef383; 74 changes,
  1,625 -> 1,682; output read whole by eye. Next: an independent in-memory rebuild of rev 3.2 from the diff and my own
  replacement list (print-only `python3 -I -c`, dry run piped in).
- C3-C4 done: rev 3.2 rebuilt in memory from the diff (1,682 lines). My own parse of the fold ruling: 32 id bullets,
  each old phrase once in flattened rev 3.1 at the line its brackets name (all 31 brief sites match), each new phrase
  0 in rev 3.1 and once in rev 3.2, each old gone (F6-3's old is the prefix of its new; F-D13's old 0 in 3.2). My own
  extraction of L1's items (A1.1-A1.5, A2.5/.6/.9, A5-A8, Q7, Q8, Q9 with F3-6, the 4 R rows, Q14.1-Q14.4 with F-D13,
  S4.1-S4.5) plus the 32 bullets applied to flattened rev 3.1: equal to flattened rev 3.2 but for exactly three
  inserts (the record paragraph, the P16 bullet, the twelve rows). Replaced old forms (A2.1-4, A2.7, A2.8, Q9.2) absent.
- C5 done: twelve rows, Question cell = rev 3.1's asked title (12/12); Ruling cell = label + whole text (Q1, Q2, Q4,
  Q6, Q10-Q13) / first sentence (Q7-Q9) / Q14's three pieces (:289, :301-302, :322-323), each the ruling's words
  exactly; cites = each ruling's paragraph lines. Third cells: Q1 = fold :149 whole; Q4 = :109; Q12 = :111; Q9 adds
  " (u4s-carries.md entry 87)" after "§U4h carry line" (:110); Q14 ends with :112's words; no "your"/"you". WHERE lists
  unchanged from the first script (which the fold ruling read and took for Q1/Q4/Q12).
- C6 done: record paragraph's claims each hold (sites listed match the ids; sha prefixes right; "Not folded" true).
- C7 done: "by U4R-Q" any case: rev 3.1 10 (:20, :250, :385, :439, :440, :446, :476, :680, :859, :1605); rev 3.2 3
  (:35, :270, :1661 = rev 3.1's :20, :250, :1605); in added lines 1 (N2's rewrap of :250's text).
- C8 done: every hunk maps to an item's unit; no word changed outside items. Whitespace only: rewraps.
- Finding (Low): rewraps split code spans that rev 3.1 kept whole on a line: D1's main-run.sh command (rev 3.2
  :1556-1557, F6-8's rewrap; rev 3.1 :1506 had it on its own line) and `Schedule::new(npt_sim::TICK_HZ,
  npt_sim::LOOP_CATCHUP)` (:342-343, A6's rewrap of 10 lines with no ruled word); F6-3's new span (:1571-1572).
  Words unchanged; Markdown renders them right; a copy of D1's raw line breaks the command.
- Finding (Low): the task's script stamp a9196126 (520 lines) is the pre-revision script; the revised one is 9e3a123c.
- End re-hash (17:5x): script 9e3a123c, brief bcaef383, fold ruling d4417040, ruling 35f281bf, plan 1e631c55, all
  unchanged; git status: only this notes file; no .pyc written.
- C9 done: verdict PASS (every ruled word exact; two Lows, no push on their own). No question for the advisor.
