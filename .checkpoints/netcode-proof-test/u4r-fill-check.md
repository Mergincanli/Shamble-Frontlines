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
- C3 done: P1 (carries :255-435, 17 entries, identical), P2 (history :292-293, marker dropped as u4d2_fold.py did),
  P3, P4 (both clauses = advisor's string, check :371-372), P5, P8, P9, P10, P11, P12, P16, P13-P15: each copy word for
  word against the source lines; closed sha256 full-asserted; P7 sha 349b51ad + bash -n 0. All match.
- C4 done: stamps plan 1,093/1e631c55, carries 661/e8675a02, history 2,027/40ae4c30, plan.md 1,518/3ca4e12a: right.
- C5 done: 28 shifted cites checked by text (old vs new lines equal), all 17 MANUAL keys read in both versions: right.
  Own token sweep outside section 10: no moved plan/carries cite missed. Change lists :160 (plan :885-886) and :166
  (:917-918) left unmoved (:45's :500-501 holds by text) -- finding F-CL (FQ4 "every other cite" vs fold :93-95).
- C6 done: R11 facts match `git show 4e91830` and HEAD files. Finding F-R11: bare "(:25)" now follows Cargo.toml:42 and
  Cargo.lock, reads as Cargo.toml:25 (blank). Out-of-scope note: R6's "BITE_IDS :752-758" is l1 lib.rs :746-751.
- C7 done: word-level diff of rev 3.2 vs result: 82 groups, every one an item; nothing else changed. Finding F-P6: the
  P6 fill in section 1 item 1 adds an instruction ("your worktree may start at a later commit...") beyond a value.
  Section 0 :211 still says lab/bots manifests unchanged since df8975b (no ruling moves it; left, Low note).
- Extra checks: ast ok; script sha256 2c65d78b... unchanged; no .pyc; main's 4d21c1a touches no spikes/, .checkpoints/,
  ADR 0001 or catalogue (its root workspace is crates/*; the spike keeps its own toolchain file, same 1.99.1). HEAD moved
  209e35d -> dac2aec during the check (autosave): P6 follows HEAD, as it should.
- C8 result: FAIL (small fix pass). Findings:
  F1 Medium: change lists :160 (plan :885-886 -> :886-887) and :166 (:917-918 -> :918-919) left unmoved; FQ4 says every
     cite but section 10; the script's exemption rests on fold :93-95 (the fold's list). Fix or advisor's word.
  F2 Medium: R11 new words "npt-net (:25)" follow Cargo.toml:42/Cargo.lock: reads as Cargo.toml:25 (blank). Fix:
     "(bots/Cargo.toml:25)" (script :679).
  F3 Low: P6 fill in section 1 item 1 adds a builder instruction (worktree at a later commit), unruled; U4d2's P6 did not.
  F4 Low: SENT0 names no version where section 10's and the change lists' cites hold (U4d2's re-stamp gave git show).
  F5 Low: no assert that main's 4d21c1a (MAIN-48, history's new lines) is in HEAD before P6 is filled.
  F6 Low (pre-existing, outside L4): R6 "BITE_IDS :752-758" is l1/src/lib.rs:746-751.
