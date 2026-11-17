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
