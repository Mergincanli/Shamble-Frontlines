# u4r-fill-check: adversarial check of u4r_fill.py (L4, section 12 items 1, 2, 3, 9) on brief rev 3.2

Script under check: u4r_fill.py sha256 2c65d78b00164f71d6d815600e7b571343a4d0606aeae6a50a90a01b3e0d616c (985 lines,
working tree, uncommitted change at :672-679 R11 words). Read-only: git reads, bash -n, sha256sum, python3 -I dry run.

## Plan (blocks)
- C1. Read section 12 and section 0 of the brief; the rulings L4, L2, FQ4, N7a, FQ6.
- C2. Read the script whole; run the dry run; save its output to scratchpad; read it whole.
- C3. Placeholders: for each Pn, open the source at item 3's lines, compare the copy word for word; P4 both clauses;
  P1 seventeen entries; closed files' sha256 asserted; P6 head + spikes/ equal to 4e91830; P7 run.sh sha256.
- C4. Stamps: section 0 entries = file's sha256 and line count now.
- C5. Re-pointed cites: sample >= 20 plus every MANUAL key against 8bcbf36~1 and now; search the brief for missed
  "plan :" and "carries :" cites; none inside section 10 moved.
- C6. L3's four files: cites into them; R11 new words vs `git show 4e91830` and the files.
- C7. Any other line changed (the diff whole).
- C8. Result.

## Progress
- C1 done: section 0 (:180-237), section 12 (:1619-1681), L2 :421-424, L4 :428-429, fold :85-170 read.
- C2 done: script read whole (985 lines); dry run exit 0, 666 output lines, 63 changes, 1,682 -> 1,908 lines, base
  209e35d (autosave). No .pyc written by it (__pycache__ holds only u4c-test-plant's). history moved at 822c010 (+16
  lines at end): the task's "history unchanged" is false; the script handles it (prefix assert, re-stamp).
