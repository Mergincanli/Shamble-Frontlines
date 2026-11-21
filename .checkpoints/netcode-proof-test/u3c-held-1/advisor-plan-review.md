# Advisor plan review: U3C-HELD-1 (plan draft u3c-held-1-plan-draft.md)
Started 2026-11-17. Seat: day-session advisor subagent, read only. Plan sha256 read:
a53b2803278968cdc9624428b738b352cd72e1f1489251bc649f8947b0b2243f (matches the task's hash). Branch HEAD f4868f2.

## My plan
1. Read main's advisor.md (done), the plan (done), the card entry U3C-HELD-1 and CHECK-NFF-1, my merge-stop ruling.
2. Code: delay.rs (Life, check_held_bound, held_now, model_write, harvest, pump loops + closing block, down_free,
   up_room, poll_write, poll_read), delay_tests.rs (G-NC2, the stalled-client test). Every held growth / shrink site.
3. Design notes, test-engineer evidence, drafter notes: check their key claims against the code.
4. Evidence: loop script te_m3_loop.sh, main-run.sh, plants P1-P5 predictions, E3-E6, pass bar counts, disk, R4,
   brief 8.1, history.md:2187 command forms.
5. Rule HELD-Q1..Q12; size/tier/reviewers/budget; verdict.

## Notes (one line per claim checked / finding)
- C1 delay.rs sha256 aaa6a588… = the plan's; `git diff --stat HEAD -- spikes/` empty; last spike commit 8ca774a.
- C2 growth sites: grep `bytes[` in delay.rs: extend only at :806 (UP, then model_write :807) and :1574 (DOWN, then
  model_write :1592); drains only :1503 (poll_read) and :1748 (pump). delay_tests.rs touches bytes only by
  `bytes[DOWN][0] ^= 0xff` (:1790, no length change). model_write has exactly two callers (:807, :1592); model.write
  only from model_write :600. CONFIRMED: the plan's growth/shrink list is complete.
- C3 by construction at a write: free_budget = write_budget − (write_seq − snd_una) (tcp_model.rs:2211-2214),
  outstanding = write_seq − snd_una (:2239-2242), write_seq starts 0 (:600) and moves only in write (:2221), so
  model_end[d] == write_seq and check's una == snd_una. Write admits n+22 <= down_free = free_budget − released_model
  (:473-477, :1561-1572); up admits len+22 <= up_room (:480-484, :803). Held model bytes = released_model (decremented
  only when a record is taken WHOLE, :1510, :1761, so >= the held released plaintext) + U, U = E − s (s = start of
  front unreleased); U <= outstanding + p whenever una does not pass the front unreleased record's end (a fully acked
  record is fully delivered, rcv_nxt :2228-2230, and harvest at every event in step_to :618-641 released it). So at a
  write, held <= budget + p. CONFIRMED (my own derivation; holds both directions).
- C4 between writes: an ack raises p or leaves it; only a release (harvest :675-696) moves s and resets the new front's
  p to ~0 while held is unchanged (released_model += size, U −= size). So the old check reads over only after a release
  not yet taken. CONFIRMED "only in a release transient, never at a write".
- C5 sample points of held_now: :1417 take_second, :1445 run_summary, :1522 poll_read, :1594 poll_write, :1879 pump
  closing block (grep). probe() (:1295-1298) reads only. G-NC2 calls none of take_second/run_summary/poll_read on ds,
  so its non-write sample is :1879 only. model.write calls advance(now) itself (tcp_model.rs:2218): a no-op at both
  write sites (poll_write's t = model.now_ns() after current(), :1557, :1571; move_up's t is step_to's t, :1813-1815;
  scan_up and move_up have one caller each). So F7's "the step to now precedes every write" holds.
- C6 check_held_falls sound: no held_now between :1574 and :1592 (the scan loop only breaks) nor :806-:807; no other
  growth; so every sample <= held_at_write[d] unless held grows outside model_write. No legit rise between writes, up
  included (bytes[UP] grows only at :806; parse_len is outside the check's held, :531 vs :512). CONFIRMED.
- C6b difference set (my own): the fixed pair (at-write bound + falls) passes exactly those samples the old check
  flagged where held has not grown since a write that met the bound while the bound fell, and the bound falls only at a
  release (C4). So it is looser ONLY on the release transient, which admits no byte (H unchanged). This is the
  strongest form of "never a looser bound without its cause"; the plan's "Not looser" line (:128-129) should say it.
- C7 G-NC2 under the fix: held_beyond_budget is set only at writes (the body :540-541 moves into the at-write check),
  so the loop's exit `beyond > 0` (delay_tests.rs:2278-2281) needs a WRITE above 64 KB: a back-fill episode gives that
  at the write itself (p >= one 1,448 B segment > 22 B x records), so the run length should not change; E3a covers it.
- C8 stalled-client test: quiet P2 (:749, quiet :52), duplex(1_024); phase 1 writes 65,360 B (comment :803-806), so
  released_model = 65,360 + 8 x 22 = 65,536 after the hold and the fixed down_free is 0: Pending, as asserted. Under P1
  the RECORD_MAX write takes 16,384: 64,336 + 16,384 = 80,720 > 65,536 + 0 -> [1, 0] at the new bound assert, which
  comes before the Pending assert. The 100 B write could not cross (64,436). owed unread after (:834-839). CONFIRMED.
- C9 plants by reading: P2 [k>=1,0] in G-NC2 (exit needs a write > 64 KB); P3 stalled [>=8,0] at held_grew (8 phase-1
  writes, n_8 = 8,016 > 1,024 taken, so closing samples count too); P4 stalled [>=1,>=1] at held_grew, G-NC2 hang
  guard 30 s (:38, :2283); P5 stalled [0,>=1] at the bound assert (tungstenite never reads; up_room = free_budget(UP)
  after acks; the flood is ~150 KB). All compile (P4: dead_code is a warning; plants run no clippy). CONFIRMED.
- C10 E5: SERIES.md enters npt-lab only under #[cfg(test)] (contract.rs:443-448); no lab/server/bots source names
  npt_net (D11): the release hash stays a valid witness that lab code is unchanged, and proves nothing on npt-net (the
  plan says so, :150-152). OK.
- C11 SERIES.md :613/:642: each old phrase occurs exactly twice (grep -c 2 and 2). Not C62H-BOUND's frozen rows
  (brief §2 freezes its own rows; figures.rs:564 names snap_lost_unexplained_count). Presence tests read names only.
- C12 u3c-plan.md:121-125: the down line already carries an in-place dated correction ("corrected 2026-11-08", :123);
  :125's up line lacks the up prefix that delay.rs:49 states.
- C13 R4 (advisor-c62h-bound-report-head-full.json conditions): "P5 holds until the report has reached him. After that,
  pack only by brief section 8.1's rules, never both P0 folders, fc411116c1f1, fd4612050cd6 or the c-u2 folders without
  his word." Reached at 03:14:03 UTC (card :198-199). BUT card :202-203: "Nothing is packed before he has read it (R4;
  the chat's For Mustafa item 2)": a promise in the chat, stricter than R4; card :10 still waiting for his answers.
- C14 disk: df now 5,227,540,480 B = 4,985 MiB (below the plan's 5,220 bar already). main-run.sh touches every crates/
  file first (:48) so each call rebuilds: L1, L2, E3a, E3b, E3c, P1-P5 = 10 bites test builds + E5 release + a light
  pre-check, not "about 7"; worst ~360 MiB, so ~4,600 before E6, ~3,600 after: every call still starts above 4,096.
- C15 commands: main-run.sh's `::` form (precedent light.log, merge-2.log); one per call. check.mjs's fmt is `cargo fmt
  --all --check` (check.mjs:2615) and clippy is `--workspace --all-targets ... -D warnings` (:2015-2017, :2618): test
  code is linted with warnings as errors, and nothing before E6 runs clippy or doc.
- C16 checkpoint.mjs:9-11: watch never pushes while unpushed commits outside .checkpoints/ exist; save does push (:6).
  The plan's reason for stopping the saver (:39-40) is half right; stopping it stays the right call (precedent).
- C17 counts: merge-2.log:12-19 552/632, deps 137 crate files, wasm 27000 both ways, marks 81, self 78; light.log:5-8
  refs 1350 at 3f2fa21, redact 305. The fix adds no test, so 552/632 stand (no T-NEW).
- C18 SIZE-CHECK words (history.md:398-399) quoted right; his 21:50 rule (history.md:2187) quoted right.
- C19 the card's entry is at netcode-proof-test.md:3882-3895 now, not :3877-3890 (drift; cite by name).
- C20 te_m3_loop.sh's failure print gives names only (sed of the failures list) and the plan adds G-NC2's stdout block
  only; P1, P3, P4, P5 fail the stalled-client test, whose panic would be only in the per-run log.

## Rulings (summary)
Q1 HEAD+D-1 · Q2 no T-NEW · Q3 keep held_grew (required) · Q4 P4 alone · Q5 reword here · Q6 in place, dated, :125
too · Q7 no pack until he has read the report; else E6 above the floor and U4f's shortfall on the card · Q8
rust-reviewer yes, xhigh · Q9 no new Low (H4 corrected it) · Q10 main folder, whole ed8673 · Q11 proceed, the reading
+ difference set + P2 carry the cause, recorded "not reproduced" · Q12 keep P5.

## Findings
- ADV-UH1-1 Medium: packing (HELD-Q7) vs the chat promise at card :202-203 (C13).
- ADV-UH1-2 Low: E6 bar's "(with T-NEW 553 / 633)": 552 / 632 only (C17).
- ADV-UH1-3 Low: plant panics not printed for the stalled-client test (C20).
- ADV-UH1-4 Low: no fmt/clippy/doc before the commit; clippy lints tests -D warnings (C15).
- ADV-UH1-5 Low: disk build count (10 test builds + E5 + pre-check); free 4,985 already under the 5,220 bar (C14).
- ADV-UH1-6 Low: "Not looser" line lacks the difference-set argument (C6b).
- ADV-UH1-7 Low: card cite drift (C19) and the saver's reason (C16).

## Verdict: APPROVE WITH CONDITIONS (C1-C10 in the return). Done.
