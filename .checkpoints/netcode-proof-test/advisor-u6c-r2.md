# Advisor: U6c round-2 ruling (2026-11-24)

Seat: the advisor agent, called from the main session; rules read from `git show origin/main:.claude/agents/advisor.md`.
Read only, effort xhigh. Brief: u6c-r2-ruling-request.txt (sha256 9a57324e… checked, matches).

## Plan
1. Check every input's sha256 (done: all six match; see Notes).
2. Read round 1's ruling (advisor-u6c-r1.md), round 2's three reviews, the fix pass (u6c-build.md:214-296).
3. Read the code at the cited lines in the worktree (probe.rs:462/:465/:478; load.rs:675/:705-707; tests/load.rs:177-184;
   probe_tests.rs:358) and U8's snapshot (u8-build.patch:1708-1711, :2189-2221).
4. Read history.md:7022-7028 (U4f's M3-M9 conditions) and :2125-2146 (Mustafa's 2026-11-15 rule), and advisor-u6b-m6.md
   :153-165.
5. Rule item by item (1a, 1b, 2, 3, 4, 5), a notes line after each.
6. Name the tree that merges, who makes each change, the M3-M9 conditions, the effect of MAIN-65's order, For Mustafa.
7. End with DONE.

## Notes
- Hashes: advisor-u6c-r1.md ad72c0b8… OK; u6c-r2-netcode.md 444b0f78… OK; u6c-r2-test.md d06a8c4f… OK;
  u6c-r2-security.md 7dfe60b6… OK; u6c-build.md a9d45dd4… OK; wip/u6c-fix.patch 0549389e… OK.
- Read: advisor-u6c-r1.md (section 4: Critical/High after round 2 stops U6c; Medium/Low follow-up), the three round-2
  reviews (netcode 0 High, 0 Medium in diff, 2 Medium outside; test 1 Medium TE-U6C-R2-4 + 7 Low; security 2 Low).
- Code read (W = worktree spikes/netcode-proof): server probe.rs:440-658 (run_leg: bind :460, pretouch :462, start edge
  :465, poller :478, wait_peer :480, window start zero+warm, seconds loop's edge hook :513-521, end edge :553);
  load.rs:611-709 (`seconds`: READY :667-671, Stopped at 30 s without READY :672-674, `over` :675, Clean :705-707;
  probe mode has no clients, so `ready` is true from the start :663 and it prints no READY).
- U8 read: :1700-1713 (the self-test row and C-U8-2/-4/-6's leg-edge rule), :2156-2237 (load_settings, load_up,
  load_ready, lg_block, load_wait), :3860-3873 (npt-load-ctl). Every LG run with clients goes through lg_block, whose
  load_ready (:2193-2201) waits for READY in LG's stdout until t0 + warm - 10 and otherwise writes INVALID-ready.txt and
  brings the room down (:2220-2224). Nothing in U8 reads LG's exit code or DONE as a pass.
- C-U8-6's source: the advisor's U3D-RCVBUF ruling (advisor-u3d.md:251-277; plan.md:1256): a drop on P1's socket
  between the two reads makes ImpairSocket's self-test INVALID, cause named, to the advisor before any L2 mark counts,
  never re-run away (a drop = a reader stall of tens of ms, which t_in stamped at drain cannot show). U8's :1709-1711
  says only "goes to the advisor": weaker words than the rule.
- Mustafa's 2026-11-15 words (history.md:2125): "A new bar, or a change to one, comes to me under Decisions needed; it
  is not built on the ruling alone." U4f's merge steps M3-M9: history.md:7022-7028.
- 1(a) mechanism, checked in the code (net socket.rs): the kernel reader is its own thread from bind (:214-216,
  :586-646) and takes the shared lock per batch (:634); the pre-touch holds that lock (:434-436). So the buffer fills
  during the pre-touch (the start values 679 and 569), and the start read (:465) races the reader's catch-up after the
  pre-touch, not the poller's start (the seat's mechanism slightly off; its conclusion stands). The pin (Intake::take
  :283) needs a datagram the reader took in and the model released (P1: about 30 ms of one-way delay later), so a read
  after `started` (:487-494, at the leg's zero) comes after the reader has drained that backlog.
- A2 logs still in /tmp (load.jsonl, room.jsonl; a `tokens` file beside them, 0600): item 4's look needs no run.
