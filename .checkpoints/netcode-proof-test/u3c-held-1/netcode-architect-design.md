# U3C-HELD-1: netcode-architect's design notes (cause and fix)

Agent: netcode-architect. READ ONLY (no builds, tests, cargo). Writes only this file (Write tool).
Branch: claude/netcode-proof-test-plan-hcky5g (PR #40); HEAD read at 25b262f (`git log --oneline -1`).

## Plan
1. Read the card (netcode-proof-test.md:3870-3885), the kept evidence (merge.log:51-52, c62h-bound-build.md:426-455,
   advisor-c62h-bound-merge-stop-full.json C3-C4).
2. Read delay.rs: the failing test and every function it calls; grep held, down_free, released_model, 64 KB bound.
3. Explain the transient (order of writes/releases, which counter moves first, when the test samples, what
   held_over_bound [1, 0] means); the bites build's feature-gated code and why only once.
4. If the code does not support the release-transient story, say so and give the cause it does support.
5. Fix design: the check, where evaluated (every write), symbols; diff as text.
6. Planted bug: exact plant, place, expected failure line.
7. NC-U3c-2 bound sentence: every place, file:line, which become wrong.
8. Size estimate with basis.

## Findings (one line per finding, appended as found)
- F1 (evidence): the failure is held_over_bound == [1, 0] at delay_tests.rs:2300-2304 (c62h-bound-build.md:430-433:
  "left [1, 0], right [0, 0]"); name at merge.log:51; the card's "delay.rs:528-543" is check_held_bound (delay.rs:528).
- F2 (index): Life.held_over_bound is [u64; 2] indexed by direction, DOWN = 0, UP = 1 (delay.rs:121-122, :382, :538).
  [1, 0] = exactly one call of check_held_bound found down held > 64 KB + prefix; up never (this test sends nothing up).
- F3 (where the check runs today): check_held_bound is called only from held_now (delay.rs:518-519); held_now runs at
  poll_read (:1522), poll_write after its write (:1594), the pump's closing block (:1879), and the per-second and
  run rows (:1417, :1445; not called by this test).
- F4 (the accounting): free_budget(DOWN) = 65,536 - (write_seq - snd_una) = 65,536 - outstanding (tcp_model.rs:2211-
  2214, :91); down_free = free_budget - released_model[DOWN] (delay.rs:473-477); released_model += len+22 at release
  (harvest, :685) and -= len+22 only when S has taken the whole record (:1759-1761). A released record's unacked part
  is therefore in both outstanding and released_model (counted twice) until its ack: down_free is conservative.
- F5 (held only grows at writes): bytes[DOWN] grows only at poll_write (:1574, then model_write :1592); bytes[UP] only
  at move_up (:806, then model_write :807); bytes shrink only at :1748 (S took) and :1503 (tungstenite read). Grep
  `bytes\[...\]\.(extend|drain...)` over delay.rs: those four sites only.
- F6 (identity, model bytes): with W = model_end[DOWN], F = start of the oldest held record, R = end of the last
  released record, H = W - F = outstanding + released_model + (snd_una - R) at every instant (signed). Acks leave H
  unchanged (outstanding down, snd_una up), releases leave it unchanged (R and released_model up by the same size),
  takes lower it. So H only falls between writes.
- F7 (the bound holds at every write): after a write n+22 <= down_free, so outstanding + released_model <= 65,536.
  If snd_una <= R: H <= 65,536. If snd_una > R: snd_una - R is exactly check_held_bound's prefix (front unreleased
  record's acked part, < its size, since a record delivered whole is released by the step to now that precedes every
  write: poll_write's current() :1557, move_up after step_to :1814-1815), so H <= 65,536 + prefix. Plaintext held =
  H - 22 x records - front_done, so held <= 64 KB + prefix at every write, with 22 B per record to spare.
- F8 (the transient): the bound's prefix term drops at the release of the front record (prefix p -> the new front's,
  0 since its start R_new > snd_una), while held is unchanged until S takes the record (:1748). If held > 65,536 at
  that release (it is, whenever the writer back-filled p: held ~ 65,536 + p - 22N), every sample between that release
  and the take counts as over the bound. Nothing grew: H unchanged (F6).
- F9 (when the test samples it): the take happens in the pump's down loop (:1721-1784), which runs only after the
  pass's opening step (:1715). A release by the pass's own up-loop step (By::Mid, :1813-1814) or by another task's
  step while the lock is free (the test loop's ds.snapshot() -> current(), delay_tests.rs:2279, and write_all's
  poll_write -> current(), delay.rs:1557, on the second tokio worker) after the down loop's last check
  (`released[DOWN] == 0`, :1727-1728) leaves the record released and untaken at the closing block's held_now
  (:1879). The code's own R1 comment says so (:1869-1872: "is still unwritten here"). That is the one counted sample.
- F10 (poll_write's own sample never trips): poll_write samples only after a write (:1594), when F7 holds; a
  poll_write whose current() released the prefixed record either returns Pending (free <= 22, :1562-1568, no sample)
  or writes within outstanding + released_model <= 65,536 (F7).
