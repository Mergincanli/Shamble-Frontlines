# U6c round 2, the netcode seat (netcode-architect, xhigh), on the fix pass's diff only. Read only.
Inputs, sha256 checked equal (2026-11-24): advisor-u6c-r1.md ad72c0b8…faed179, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
Diff: `git diff aad21031… 6c3ff00b…` (10 files, +356/-94). Paths below: W = .claude/worktrees/agent-a3a3e3fdf559de095/
spikes/netcode-proof.

## 1. Plan
1. FP-1: read `leg_series` whole in W (net l2/probe.rs) and `run_leg` (server probe.rs) at the changed lines and what
   they call (in_burst, sec, the RTT join, window_count, the E set, the Measured phase, the second rows). Check: exactly
   `packets` outside the bursts, t_in order, tie rule, up for the pinned peer only, the two spans, edge cases (short
   tail, receive batch sharing one t_in, a burst at the end, peer None, start's second vs first record), interaction
   with the RTT join (kept vs echoes ordinal), the measured-phase extension vs the judge's window and the per-second rows.
2. SEC-U6C-4's pin: `Intake::take` with `ProbeDatagram::decode(b, None)`; what decode(None) accepts; can a real peer's
   first datagrams fail it (leg tag, version, size), so the leg pins late or never.
3. NC-U6C-3: `probe_down_unindexed_count` and its SERIES.md row.
4. load.rs riders: NC-U6C-5, NC-U6C-6, SEC-U6C-3's load.rs half, NC-U6C-8's SERIES.md rows.
5. Readings 1-3 (only the code defining each named count, the per-second input count, U8's lines in item 3, and
   tests/load.rs's `a2`).
6. Write notes per finding as I go; verdict, table, per-item status, outside the diff, not checked, DONE.

## 2. Notes (one line per finding)
