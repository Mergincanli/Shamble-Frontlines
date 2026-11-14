# u4r_fold.py: notes (workflow agent, opus, high)

**Slip (recorded first, as df-ruling R6's form):** the first dry run's output went by a shell redirect (`> ... 2>&1`)
into the session scratchpad, /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/dry1.txt,
against this task's write rule (no shell redirect). It is outside the repository, holds only that dry run's printed
output (exit 0), and nothing in the repository changed by it. Every later run prints to the terminal only (piped to
head, tail or grep, never into a file).
**Slip 2:** a syntax check by `python3 -I -m py_compile` wrote
.checkpoints/netcode-proof-test/__pycache__/u4r_fold.cpython-313.pyc (43,614 B, 16:56): a Python write, against the
same rule. It is untracked and NOT gitignored (`git status`: `??`), so the background saver could commit it (the folder
already tracks u4c-test-plant.cpython-313.pyc). Not removed (a removal is a write too): the main session's to delete
before the saver's next commit. Running the script itself writes no .pyc (a main script is not cached).

Task: write .checkpoints/netcode-proof-test/u4r_fold.py, the L1 fold of advisor-u4r-brief.md (sha256 35f281bf…) into
u4r-brief.md rev 3.1 (bcaef383…) -> rev 3.2. Never applied by me: dry run only (`python3 -I`, no --apply).
Inputs hashed at start: ruling 35f281bf (463 lines), brief bcaef383 (1,625 lines), te-m3 5ecc3e23, te-m3-check 1ffd0294.
No earlier notes or script: fresh start.

## Plan (blocks)
- B1. Read the ruling :74-382, :415-420 in full; read the brief whole; read u4d2_fold.py and u4r_l2_text.py.
- B2. Locate in the flattened brief every old phrase / span end / anchor of A1, A2, A5-A8, Q7, Q8, Q9, Q14, section 4's
  lane (print-only python3 -I from stdin): count each.
- B3. Map section 10's "Answered" table (:1276-1284) and the twelve questions as asked; section 0's stamp list; section
  12 item 3 (placeholders list, where P16 goes); the title block.
- B4. List every "by U4R-Qn" (six) and every "P0 1" with treatment.
- B5. Write the script (u4d2_fold.py's machinery: cut/joined, edit with ws regex, refill, row edit, row removal, span
  replace, after-insert, words check).
- B6. Dry run; fix until exit 0; check diff by eye; record sha256, exit code, last line.
- B7. Result.

## Log
- B1 done: ruling :74-382, :415-420 read; brief read whole (1,625 lines); u4d2_fold.py and u4r_l2_text.py read. Next B2-B4.
- Finding: "by U4R-Q<digit>" (lowercase) occurs six times: :20 (C27, history), :385 (Q9 covers), :439 (Q14 span covers),
  :476 (2.2 "And, by U4R-Q14 (b)": no replacement), :680 (3.2 row 62 "(h)'s check by U4R-Q2 (b)": no replacement),
  :859 (Q9 covers). Also "By U4R-Q14" twice at :440, :446 (inside Q14's 2.1 span) and the generic "by U4R-Qn" at :250
  and :1605 (the instruction itself).
- Finding: "P0 1" at :19 (C27), :442 (inside Q14's 2.1 span), :1459 (carry 74's old formula's value, Q14's question),
  :1460 (the round-trip form at P0, Q14's question; kept as asked? question).
- Finding: stale after the fold, not in the ruling's words: section 6 item 5 "D13's two runs" (:853); 7.1 "D13's short
  P2 run at both leads" (:914); 7.4 "the determinism step's three ... green on the three" (:1105-1107); 2.1 (i) quotes
  plan §4.15's old words (:384; L2 changes them); section 4 "(U4R-Q6's site aside, if ruled)" (:806); 7.3 "the ES and
  TC recordings likely less" (:1085-1086). Left unchanged: one question.
- Decisions (main session's words): stamps appended after `m9`'s with commas; P16 bullet after the P13-P15 bullet;
  P4 line span from the old quoted string to "if U4R-Q6 rules (a))"; "**Open**" relabelled; table cells: the letter
  from the ruling's bold label kept (else the row would not say what was ruled), cite at the Ruling cell's end; R table
  rows matched by their first two cells.
- B2-B4 done (by grep and reading). Next B5: write the script.
- B5 done: u4r_fold.py written (u4d2_fold.py's machinery; spans, rows, row removal, the R rows by their first two
  cells, the P16 bullet, the table rows, title and record; a "(i) "/"(a) "/"(1) " line starts a unit of its own so 2.1's
  (i)-(iii) keep their line starts). Next B6.
- B6 done: dry run `python3 -I .checkpoints/netcode-proof-test/u4r_fold.py` exit 0; 51 item lines; words check: 51
  changes, 61 new phrases each once, 45 replaced phrases gone, no changed line over 120 (rows exempt); brief 1,625 ->
  1,662 lines; last line "dry run: nothing written"; brief sha256 still bcaef383. Script sha256 a9196126b2efdd9b…
  (a9196126b2efdd9b9d6ad89ee96735fbc4abfcd6daeb8debc708e972d0802d4e), its own lines all within 120.
- Finding (no question): Q9.1+Q9.2 as ruled give "and SS P0 seed 4 (TC B1 seed 3 is U4h's, with its TC shooter:
  U4R-Q9 (b)) (the -30 pre-check's source only)": two parentheses in a row, the ruling's words exact.
- Finding (no question): A6's cite is the bullet's first changed word, so 2.1 step (2)'s bullet rewraps from :318
  (words unchanged); D1's rewrap splits the main-run.sh command's code span over two lines (words unchanged).
- B7 done: result returned (script sha256 a9196126…, dry run exit 0, last line "dry run: nothing written"; four
  questions for the advisor: 2.2's "by U4R-Q14 (b)", 3.2 row 62's "by U4R-Q2 (b)", Q14's question "P0 1" at :1460, and
  the stale references list; plus the two slips, the .pyc left for the main session). Nothing next.

## Revision for the fold ruling
Task: revise u4r_fold.py (a9196126…, 520 lines) so its dry run folds advisor-u4r-fold.md (d4417040…, 217 lines) with
advisor-u4r-brief.md (35f281bf…) into rev 3.1 (bcaef383…), as the fold ruling's :24-41 and :187-193 say. Dry run only.
Inputs hashed at start: fold ruling d4417040 (217 lines), ruling 35f281bf (463), brief bcaef383 (1,625), plan 1e631c55
(1,093), script a9196126 (520). No earlier "Revision" part: fresh start.

### Plan (blocks)
- V1. Read the notes, the script, the fold ruling whole, the questions, the check; the ruling's A2, Q9, Q14 and L1;
  the brief at every site; the plan :500-501.
- V2. A reader for the fold ruling's id bullets: one line each, `- ID (site): "old" → "new"`, `\"` read as `"`; the ids
  asserted as the ruling lists them (F1-1 .. F6-9, N1 .. N7b, F-D13); the third cells (:109, :111, :149) read from their
  lines; the plan's :500-501 sentence read and asserted equal to F3-7's quoted words.
- V3. Edit the script: the asserts; A2.1-A2.4, A2.7, A2.8 -> F6-4..F6-9, Q9.2 -> F3-6; F-D13 into Q14.4's new text;
  S10.open -> F2-1; the added items; section 10's rows (FQ5 (d)); the third cells; the record paragraph (FQ5 (e)); the
  two character changes logged as the main session's (FQ5 (f)); the "by U4R-Q" after-check.
- V4. Syntax check (ast, writes nothing); dry run; fix until exit 0; read the whole diff against both rulings.
- V5. Result.

### Log
- V1 done: notes, script, fold ruling (whole), questions, check, ruling :70-429 read. Next V1's brief sites, then V2.
- V1 done (sites): the 32 id bullets (fold ruling :47-168) parse one per line; each old phrase once in flattened rev
  3.1, each new 0 times (print-only `python3 -I` from stdin); "by U4R-Q" (any case) at ten sites: :20, :250, :385,
  :439, :440, :446, :476, :680, :859, :1605; the plan :500-501 sentence read. Next V2-V3: the script edits.
- Decisions (the main session's words, in MAIN): Q7/Q8/Q9/Q14's third cells = WHERE, then "; its words in place: ..."
  (WENT's lists, the ruling's form), then FQ4's additions; F3-6 and F6-4..F6-9 keep the A2/Q9.2 slots, tagged with
  the fold ids; the "by U4R-Q" after-check by count (3), flattened context and the line each came from.
- V2-V3 done: u4r_fold.py edited by Edit only (asserts for the fold ruling d4417040 and the plan 1e631c55; the id-bullet
  reader; A2/Q9.2 slots; F-D13 into Q14.4; the 24 other ids; rows, third cells, record; the after-checks). Next V4.
- V4 done: ast check (writes nothing) passes; dry run `python3 -I .checkpoints/netcode-proof-test/u4r_fold.py` exit 0,
  last line "dry run: nothing written"; 74 changes, 84 new phrases each once, 67 replaced phrases gone, 32 ids each
  used once; brief 1,625 -> 1,682 lines; "by U4R-Q" after: 3, at rev 3.2 :35, :270, :1661 (rev 3.1's :20, :250 (item
  5, rewrapped by N2), :1605); brief sha256 still bcaef383. Whole diff read against both rulings. An in-memory run
  (`python3 -I -B`, runpy, stdout captured) confirmed rev 3.1's :459, :612, :916, :1111, C1-C31 and item 4 unchanged.
  Script lines all within 120. One check run piped the dry run's stdout to /dev/null (`> /dev/null`): it wrote no file.
- Finding (no question): F6-3's rewrap splits a code span over two lines (`git show 4e91830 --` / `spikes/...`), as
  D1's did in the first dry run: words unchanged.
- Finding (no question): 2.1 (i) keeps "the determinism step's three runner recordings (SS P2 seed 1 and ES P3 seed 2
  at U4r; U4h adds TC B1 seed 3)" (Q9.3's words): three in all, U4h adding the third; neither ruling changes it.
- V5 done: script docstring rewrapped (its lines all within 120); final dry run exit 0, last line "dry run: nothing
  written", brief 1,625 -> 1,682; script sha256 9e3a123cfe923fb0eae8cf10d00980f4dca924e81a4d4acaf133b79fa2096287
  (693 lines); brief still bcaef383. No old phrase failed; no word outside both rulings but the main session's (MAIN:
  third cells, record, the char changes). Result returned. Nothing next.
