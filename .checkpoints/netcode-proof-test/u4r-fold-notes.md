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
