# u4r_fold.py: notes (workflow agent, opus, high)

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
