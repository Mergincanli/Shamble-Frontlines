# The advisor's ruling on U6c's round 1 (NPT-TRIM's one round; 2026-11-24). Read only.
Inputs, sha256 checked equal: u6c-review-netcode.md f29e1ac1…, u6c-review-test.md 79197b42…, u6c-review-security.md
c29540fa…, wip/u6c-build.patch a4e63455…, u6c-build.md 426b31da…. Paths below are under the worktree's
spikes/netcode-proof (W). Every Medium or higher read at its cited lines (leg_series, run_leg, serve.rs, the tests).

## 1. Verdict: REVISE (one fix pass, FP-1 to FP-6, then a narrow round 2 on what changed)
One High open (NC-U6C-1: PM-0001-3's ten `count >= 1000000` checks, marks.json:752-760 and the nine like it, read
records cut by a span of RS's clock, net probe.rs:691-692, :733; server probe.rs:447-450, :557-563, so each up count
lands at 1,000,000 plus or minus the edge jitter and LG's clock drift and fails about half the time at R1). Mediums:
NC-U6C-2 = F-A2-EXACT, F-P8 = NC-U6C-11 (kept at the test seat's Medium: a plant that cannot fail), F-READINGS,
F-TIMING, F-P2, and NC-U6C-4 (outside the diff: his, section 5). Security's five are Low; its host read stands as Stop
B's, with round 2 reading serve.rs's changed lines only. Nothing in the fix pass touches a pass mark, the judge or
marks.json. **Size:** 5,431 added; the fix pass adds about 200, to about 5,630. The builder writes its count after FP-5;
past 5,850 the riders in FP-6 stop where they are and go to section 3 as follow-ups; past 6,000 SIZE-CHECK applies as
written (three lines to him). **R1 on Thursday 3 December holds:** U6c is days ahead of C4's trigger (merged by the end
of Sun 29).

## 2. The fix list (one builder, xhigh, in the same worktree; in this order; each with its test and plant)
**FP-1 (NC-U6C-1, High): L1's count-based records, not the 10 ms margin.** In `leg_series` (crates/net/src/l2/
probe.rs:684-770) each direction's measured records are its first `packets` fates outside the bursts whose t_in is at
or after the window's start, in t_in order (ties in log order; any fate sharing the last kept fate's t_in is kept too),
up for the pinned peer only; in-burst fates between them go to `probe_burst_*` as now. t_in order, not flow-index
order: QueueDrop and PoolDrop fates carry no index (crates/net/src/l2/impair.rs:489-512). `leg_series` takes `packets`
in place of the window's end; `run_leg` (crates/server/src/probe.rs:447-450, :557-563) passes `s.packets` and the
start; the tail (:452) already holds the extra sends. The second rows run to the last kept fate's second; C-U6-5's E
sets are these records (rule unchanged). New `l2_summary` fields `probe_up_window_ns` and `probe_down_window_ns` (each
direction's last kept t_in minus the start, plus one interval) with their SERIES.md rows (names ruled: unit-suffixed);
`probe_window_ns` stays the nominal window. Fewer than `packets` available: the count is what there is (the mark fails
honestly; U8's rate check names why). Why count, not margin: it is exact by construction whatever the day's clocks,
and it is L1's own rule (lab/src/probe.rs:124-132) and R-6's "both layers … read one count" (lab/src/probe.rs:21-22).
**Test** (net probe_tests.rs): fixed fates, up spaced 100 µs × (1 + 5 × 10⁻⁶), down's last three stamped 60 µs late
(a late pacer), with and without a B1 burst: each direction gives exactly `packets` outside the burst, the burst's
fates apart, the two spans as computed. **Plant P22:** the clock span back (`inside(t_in)` over [start, end)); it must
fail that test. The end-to-end bound (crates/server/src/probe_tests.rs:313-319) becomes `>= packets` a direction (this
also settles F-TIMING (a)). About +110.
**FP-2 (F-P8 = NC-U6C-11, Medium).** `edge_events(sock: &ImpairSocket, port, ph)` (crates/server/src/probe.rs:176;
calls :461, :549), so a read after `drop(sock)` (:564) is E0382; probe_tests.rs:177's doc drops "P8's test". **P8
rerun:** the end-edge read moved after the drop must fail to compile (plant-p8c.log). **Report it as:** "P8 cannot fail
a test (plant-p8.log: 1 passed); after FP-2 the compiler holds it (plant-p8c.log, E0382)", never as a test failure.
About +3.
**FP-3 (F-P2, Medium).** In the end-to-end test (crates/server/src/probe_tests.rs:284-328) assert P1's whole-life
`l2_down_delivered_count` <= P1's `probe_down_sent_count` (a socket carried from P0 exceeds it). **P2 rerun as a
shipped-code plant** (bind once in `run_probe`, `set_default` per leg, in place of crates/server/src/probe.rs:456); it
must fail that assert. The test-body P2 stays as the unit half. About +4.
**FP-4 (F-TIMING (b) and (c), Medium).** (b) crates/bots/tests/load.rs:260-265: `probe_peer_sent_count >= 5_000`.
(c) crates/net/src/l2/probe_tests.rs:439-449: read until echo 9 arrives or 1 s passes, assert the echoes are [9] or
[8, 9], then ten reads with none. **P19 rerun** on the new form must still fail it. About +8.
**FP-5 (NC-U6C-2 = F-A2-EXACT, Medium; with F-READINGS' transport half).** A second ignored test,
`a2_48_webtransport_clients_at_p2_against_one_room`, sharing A2's body (crates/bots/tests/load.rs:289-338) with `wt`;
the doc at :284-288 and u6c-build.md:73-77 give both commands by full name: `cargo test --locked --release -p npt-bots
--test load <full name> -- --ignored --nocapture --exact`, after `cargo build --locked --release -p npt-server`.
**Proof:** one `--list` call per name prints "1 test". About +10.
**FP-6, the Lows that ride this push** (each a few lines in files the pass edits anyway; none would push on its own):
- SEC-U6C-1: crates/host/src/serve.rs:253-255, on an accept error sleep 100 ms (tokio's timer) before the next accept.
- SEC-U6C-3 = F-HASH-PLUS: serve.rs:202 adds `n.bytes().all(|b| b.is_ascii_digit())`; `/npt/cert-hash-+5.json` joins
  the 404 list (crates/host/src/serve_tests.rs:209-220); crates/bots/src/load.rs:265 takes the same check.
- SEC-U6C-4: crates/server/src/probe.rs:279-281 pins only when `ProbeDatagram::decode(b, None)` is Some; P18 rerun.
- NC-U6C-3: `probe_down_unindexed_count` in the summary (down fates with no index; net probe.rs:704-712), its row.
- NC-U6C-5 = F-FAILED: `handshake_failed_count` in LG's footer and in the "no READY" line (load.rs:142-148, :658-659).
- NC-U6C-6: `w.finish(Ended::Stopped)` before the returns at load.rs:560-562 and :588-591.
- NC-U6C-8 = F-SERIES-LOAD: SERIES.md rows for `probe_peer`'s six `_count` fields and LG's params `room` and `mode`;
  the row cross-check (server probe_tests.rs:272-282) extended to LG's log in tests/load.rs's probe test.
- F-NO-PEER: a test, `run_probe` with `peer_wait_ns` 200 ms and no peer ends Err((1, …)) and writes no leg log
  (crates/server/src/probe.rs:413-435, :521-522).
- F-NAMES: `!seconds.is_empty()` (server probe_tests.rs:266-271); the last second's index where the end matters
  (tests/load.rs:214-222); "every measured second" from the second measured second (:172-177).
- F-5S: tests/load.rs:216 `hosts_four("wt", 4)`. F-JOIN-4 and F-STALE-BIN: doc wording only (tests/load.rs:126-131,
  :203-210, :86-90).
About +60. Counts after the pass: +54 measured and +54 bites (FP-1's test and F-NO-PEER; the two A2s stay ignored).
**End of the pass:** rustfmt's diff by Edit; the full check.mjs once (11 steps); the plants rerun or new: P2, P8, P18,
P19, P22, each restored and sha256-checked; notes and size per item.

**The container readings (F-READINGS; the main session, at U6c's close, before Stop B's round reads R1.md;
reported, never judged; through the runner, release builds):** (1) the probe with the shipped binaries, no example:
`npt-server --settings` in probe mode (`probe_legs` ["P0", "P1"], `probe_packets` 1,000,000, peer 127.0.0.1, UDP
30003) with its stdin held, and `npt-load` in probe mode against it: both E forms a direction, the over counts, p50 to
max, `probe_drain_hold_ns`, the counts and FP-1's spans (C-U6-6's 10,000 a second each way), steal before and after,
and each leg's wall time, which sets U8's probe `duration_s` and R1.md's probe hours (the 0.20 h is re-checked). (2) A2
on both transports (FP-5): yes, WebTransport needs its 48-client reading, since R1's headers and calibration run 48 WT
clients (U8's `load_up 01 bots wt 48 300`) and no test has run more than a few WT sessions at once. (3) The host
serving U7's dist only if U7 has built one; else the tests stand. U6c's report names every reading not taken.

## 3. Carried, each with its taker
- NC-U6C-4 (Medium, outside the diff): his marks.json and judge question (section 5). It does not hold R1: R1's logs
  keep every stamp and each leg's `base_rtt_ns`, so the judge can read -3's L2 parts either way after his answer.
- NC-U6C-12 (Low): U6c's placement is plan.md:1256's ("both reads (start and header)"), so U8 changes: probe_presence
  (wip/u8-build.patch:2446, :2479-2485) reads P1's header `probe_start_timer_slack_ns`, `probe_start_sched_policy`,
  `l2_timer_slack_ns` and `l2_sched_policy`, and takes the rate from FP-1's `probe_up_window_ns` and
  `probe_down_window_ns`, with its fixture test. The main session makes it at U6c's merge (C9) into U8's snapshot
  before Stop B's round reads it; if that round has begun, in Stop B's fix pass.
- SEC-U6C-5 (Low): U8, at the same C9 edit: one line in R1.md's certificate block naming Let's Encrypt's Subscriber
  Agreement and its URL from the directory's `meta.termsOfService`; he reads it at Stop B (section 5).
- SEC-U6C-2 (Low): STATUS follow-up (the host copies each body; for any reuse of npt-host, at the security baseline).
- NC-U6C-7: U6c's report names each pub item with the brief's reason; narrowed at each crate's next change.
- NC-U6C-9 and NC-U6C-10: U6e's brief (it reuses the glue for -30) and R2's run plan, before any V-S run.
- F-JOIN-3, F-FAKE-GAPS: STATUS follow-ups; R1's staging order is the issue mode's first real test.
- F-WT-CLOSE: U6d and U6e (as the builder named). F-JOIN-DEADLINE-READ: U6c's report names "no kind-10 refusal" as
  unproven, for U6d's and U6e's refusal counts.
- C-U6-8 with TE2-TE4: MOVED's brief (lever (1), pulled by C4 at 5,426 added; C4 counts added lines, so the valve
  stands); U6c's report says R1's self-test reads stamps no test pins until then.
- No change: F-ECHO-RACE, F-PORT, F-POLLER-STATE, F-LG-SHA, F-TERM (all three seats agree).

## 4. Round 2: yes (NPT-TRIM allows it for the open High), on the fix pass's diff only
One batch, each at xhigh: **netcode-architect** on FP-1, SEC-U6C-4's pin, NC-U6C-3 and the load.rs riders;
**test-engineer** on the changed and new tests and the plants P2, P8 (by the compiler), P18, P19 and P22;
**security-auditor** on serve.rs's and server probe.rs's changed lines only (SEC-U6C-1, -3, -4), so its host read keeps
standing as Stop B's. Nothing outside the diff is re-read. A Critical or High still open after it stops U6c for him;
a Medium or Low becomes a follow-up.

## 5. For Mustafa (one question for the day's message; nothing holds R1)
**Decisions needed:**
- **NPT-L2-ONEWAY, PM-0001-3's one-way checks on the real network.** Question: may the judge compare -3's L2 one-way
  delay checks with the profile's one-way delay minus half the measured base round trip? Why it matters: on L2 the
  network model adds (profile round trip − measured base) ÷ 2 each way, as -3's own notes say (marks.json:2348, R-9),
  and the probe stamps only that added part, so its one-way delays sit half the base below the profile's (about
  0.2-0.5 ms between two Hetzner VMs). P1's tightest ends are ±0.25 ms (marks.json:815-874), so they would fail on that
  offset, not on the model. Options: (a) yes: on L2 every one-way check (mean, both ends, the p90) reads against the
  profile's one-way minus `base_rtt_ns` ÷ 2 from the leg's header; L1 is unchanged (base 0), no tolerance changes, the
  round-trip checks unchanged; a marks.json notes line and a judge change with a reviewer's read, built in MOVED before
  R1's report. (b) Leave it: those checks likely fail at R1 and come back to you. (c) Add half the base to every
  stamped one-way delay in the probe: the logs then hold an estimate, not a measurement. **Lean: (a).**
- No other decision. Information only: at R1 each certificate order agrees to Let's Encrypt's Subscriber Agreement for
  him; R1.md will say so, and he sees it at Stop B (SEC-U6C-5).

**For Mustafa:** nothing to do now; answer NPT-L2-ONEWAY in the day's message.

## 6. What I did not check
- I ran nothing: no cargo, test, plant or `--list`. I read the reviewers' and the builder's logs as they report them.
- FP-1's behaviour at the edges (ties in a receive batch, a short tail) is reasoned from the code, not run. The edge
  jitter and clock drift behind NC-U6C-1 are arithmetic, not measured.
- F-P2's claim that no test fails on a shipped-code P2: I accepted the test seat's reading of the end-to-end test.
- NC-U6C-4's base on two Hetzner VMs is not known; 0.2-0.5 ms is an estimate.
- U8's snapshot was read only at the lines cited (2446, 2479-2485). U7's dist was not checked.

DONE
