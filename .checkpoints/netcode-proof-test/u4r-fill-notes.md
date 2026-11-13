# u4r_fill.py: the U4r brief's launch step (L4, section 12 items 1 to 3 and 9), on rev 3.2

Task: write .checkpoints/netcode-proof-test/u4r_fill.py (dry run by default; the main session applies it). Brief rev
3.2 (362e771e…, 1,682 lines, a6b3410). Read-only commands only: git reads, bash -n, sha256sum, python3 -I print-only
or the script's dry run. Never --apply, never cargo or check.mjs, never commit.

## Plan (blocks)
- B1. Read: section 12, section 0, section 9's table, section 10's bounds, every placeholder's home; u4d2_restamp.py
  and u4r_fold.py's helpers; the rulings L2, L4, FQ4, N7a, FQ6; L2's diff (8bcbf36) and L3's (4e91830).
- B2. Inventory the cites of the brief into the plan and the carries (outside section 10; the change lists noted):
  difflib maps for both; the cites touching a rewritten block, each read by its text, into MANUAL.
- B3. Inventory the cites of the four files L3 changed (Cargo.toml, Cargo.lock, lab/Cargo.toml, bots/Cargo.toml):
  which are old (pre-4e91830) and which the fold wrote at the head; R11's row.
- B4. The placeholders P1 to P5, P8 to P16: each source span read, its ends found once; P6 and P7 homes.
- B5. Write the script: asserts (sha256 of the brief and every source, HEAD and the spikes diff, the counted log),
  fills, re-points, stamps, R11, the words check, the log, the diff.
- B6. Dry run, read in full, fix; syntax check by ast; sha256 of the script; result.

## Progress
