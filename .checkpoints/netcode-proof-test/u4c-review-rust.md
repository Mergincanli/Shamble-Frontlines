# U4c review, rust-reviewer (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (uncommitted), base 097f8970.
Plan: u4-build-plan.md §5 U4c (:552-583), §4.1 (npt-l1 row); advisor-u4.md; advisor-u3q.md:160-173 (C1'); advisor-u2.md (C2).

## Plan
1. git status / diff stat in the worktree; confirm only crates/l1 changed; sha256 of the three files vs the report.
2. Read lib.rs whole: model, event queue, ordering, integer time math, allocation, panics, public items, docs.
3. Read tests.rs + tests/l1.rs: L-c1..L-c9 coverage vs the plan; bites per instance; cfg gating.
4. Determinism: RNG streams (ChaCha8 per flow), event order (time, fate index), no HashMap order, floats.
5. Arithmetic: overflow, saturating vs checked, PTO doubling shift overflow, division.
6. Findings -> verdict.

## Progress
- Step 1 done: worktree has only crates/l1/src/lib.rs (M), tests/l1.rs (M), src/tests.rs (new) + build notes. sha256 match report (1647e75d, c21a362f, 639ab9c5). Cargo.toml unchanged (deps npt-link, rand_chacha, rand_core).
- Step 2 read lib.rs 1-1895 whole. Candidates to check: (a) next_event_ns(&self) does borrow_mut + pops stale events (interior mutation behind &self; panics if called while a borrow is live? only from inside callbacks: none); (b) Message bytes stored at send in receiver queue; arrive() indexes s.messages[(message-base)] (invariant: base..next_message); (c) arrive() allocs Vec in bites only; send_piece/detect_lost `Vec::new()` resend per call (alloc only when non-empty? Vec::new doesn't alloc); (d) pto_ns checked_shl -> saturating; (e) set_timer at.saturating_add(pto); (f) arrival_ns now+delay unchecked; Rtt 7*sm unchecked; (g) Event rank (at, key=fate index, order, link, dir, space, tie).
- Step 3 tests read (tests.rs whole; tests/l1.rs diff). Event rank uniqueness verified by construction (Arrive key=pkt idx order0; HeldDue order1; Ack own idx order2; Timer tie=generation strictly increasing) -> pop order layout-independent, so next_event's stale pruning cannot change order; staleness monotone (generation only grows; timer Some always bumps generation; held idx never re-held).
- F-cand M: L-c5 cannot fail on a wrong next_event_ns: event mode polls every whole ms to 6 s (next_send=(t/MS+1)*MS regardless of real sends) and ends with poll(end) at 10 s; advance() applies every event at its own time, sends never react to data, so any next_event_ns that never names a past instant passes (e.g. always None). Only other test (tests/l1.rs:516) covers a datagram arrival + its ack, not timers/HeldDue/gate+1. Fix: event mode polls only at next_event_ns and the script's real send instants; assert each item's hand-out call instant == max(arrival, gate+1).
