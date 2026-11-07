# Checkpoint: netcode-proof-test
- **Goal:** start the netcode proof test (SPIKE_PLAN item 1) as its own change-set; this step is the PLAN only, for the
  advisor's review and Mustafa's (no code before approval) · plan: `.checkpoints/netcode-proof-test/plan.md` (revision 3.2: APPROVED by the advisor (C1-C12) and by Mustafa, 2026-11-06; revision 1 786b7d2, revision 2 d8f0712, revision 3 a6607bc) · verdict: `.checkpoints/netcode-proof-test/advisor-verdict.md`
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main `af8393e9d40f3bd382e77f86e6df2612785c706c`
- **Last update:** 2026-11-06 (after Mustafa's approval) Amsterdam

## Steps done
1. SSH reachability probe (plan item 1, first pass) · this commit · NO: no ssh client in the image; TCP 22 and every
   port but 443 time out, direct and via the proxy; 443 is TLS-re-terminated by the egress gateway. Evidence:
   `.checkpoints/netcode-proof-test/ssh-probe.md`. So the plan proposes cloud-init and/or Claude Code on Mustafa's PC.
   Second pass (no commit of its own): any public HTTPS host is readable (nasa.gov, ikea.com, Let's Encrypt's ACME
   directory: 200); shamblefrontlines.com is on Cloudflare (NS cora/steven), no A/AAAA/CAA yet.
2. Requirement digests · workflow `wf_1ae38d53-da9`, 3 of 3 returned (netcode-architect 52 KB, infra-engineer 71 KB,
   test-engineer 48 KB, each ends DONE) · saved by 4e15330 and the saver · findings: ~20 readings for the advisor
   (reordering at P3, packet count, loopback LAN, x86-64 replays, L2 delay formula, OQ-0001-9 cover, stand-in
   conditions, bites, runner list, ML close, ports, cert method, metrics path, host keys) and 2 for Mustafa (route
   and its machine; Safari timed rows).
3. Plan revision 1 · this commit · 699 lines: item 1 (no SSH; route (a) hybrid recommended, laptop), scope, layout
   `spikes/netcode-proof/`, components E1-E22, symbols, pins (Rust 1.99.1), mark map (every PM-0001-1..62 but the 60 Hz
   group, PM-0004-67, PM-0006-11..27), sessions VS1-VS8 (39 RS-h, 84 LG-h, EUR 6.88 planned, caps EUR 8.60) + VS9 later,
   ports UDP 30001-30032 / TCP 31000-31032, DNS `A rs.nt` DNS only, http-01 on RS, the Safari workflow as W-PR,
   readings R-1..R-24 for the advisor, decisions ROUTE and SAFARI-TIME for Mustafa.
4. Plan review round 1 · workflow `wf_88b642c1-384`, 3 of 3 returned · security-auditor REVISE (1 Critical: the
   spike-only controls are unauthenticated; 7 High: firewall at creation, join tokens, host-key method, the laptop
   session's rules, the ssh-keygen command, page-host hardening, cloud-init users), engine-architect REVISE (6 High:
   room lib vs server bin, Link in a leaf crate and poll-based, npt-lab and the simulated clock, the Worker host's
   shape, determinism rules, the 60 Hz seam), verifier FAIL (1 High: PM-0006-14 needs 3 sessions per N; hours without
   working; -36 must come first; readings that change a number belong in the drafts). Compact copy:
   `.checkpoints/netcode-proof-test/review1-findings.md`. The verifier's gate run (worktree, since removed): checks
   1-4 and 5b-5d PASS, symbols +0 -0, 5a skipped (no Playwright in tools/).
5. Plan revision 2 · this commit · 1,022 lines; §0.1 maps all 49 findings to their fixes. Main changes: the control
   plane (local socket + per-run capability), roster and tokens, host keys via HTTPS for RS and ProxyJump for LG, the
   laptop session's rules, cloud-init users and walls, the crate split (npt-link, npt-l1, npt-room lib, npt-server,
   npt-client-wasm, npt-log, npt-lab, npt-super, npt-host), determinism rules, the sim seam and TICK_HZ, "kept",
   seven run days R1-R7 (72 RS-h, 171 LG-h, EUR 12.99 planned, caps EUR 14.50) + R8 later, R-PR for readings that
   change a number, readings R-1..R-27, decisions ROUTE, SAFARI-TIME, FIXED-IP.
6. Plan review round 2 · workflow `wf_e7cdd40f-712` (failed once at the usage limit, resumed 02:13), 3 of 3 returned ·
   security-auditor APPROVE WITH CONDITIONS (3 High from the fixes: `node .gate/*` runs unprompted in the laptop's
   worktrees; the operator settings over-claim (deny-first, PowerShell, a moving file); the main-ref Safari guard
   gives change-set code main's cache scope), netcode-architect REVISE (4 High, text only: no channels in Link and the
   endpoint; no loss/stall model in the WebSocket delay; the limiter's queue smaller than a datagram; which
   command-timing option sets the rule cap), infra-engineer APPROVE WITH CONDITIONS (3 High: the density order inside
   blocks breaks the time-of-day rule; the day's addresses cannot sit in the hashed cloud-config; RS has no key to
   drive LG). Both OQ-0006-16 deciders agree with §8.1's ports, with small conditions. Compact copy:
   `.checkpoints/netcode-proof-test/review2-findings.md`.
7. Plan revision 3 · this commit · 1,299 lines; §0.2 maps all 46 round-2 findings to their fixes. Main changes: the
   laptop session's settings rewritten to what holds (Manual mode prompts as the wall; `node .gate/*` denied; fixed,
   hashed settings copy; worktree checks; orphan results branch); Safari dispatched on the change-set branch with no
   commit input and no cache; per-slot control sockets with SO_PEERCRED under systemd units; RS-to-LG restricted key;
   `npt-ctl peers` and `npt-cert issue/revoke` wrappers; channels in Link and the endpoint; the WebSocket loss and
   stall model; the limiter's queue depths; both command-timing options before the rule cap; density order fixed;
   hours table adding to 75 RS-h (177 LG-h, EUR 13.51 planned, caps EUR 14.90); readings R-1..R-29.
8. The advisor's plan review of revision 3 · notes `advisor-plan-review.md` (DONE), verdict `advisor-verdict.md` ·
   **APPROVE with 12 conditions** (C1 the push ask: main allows `git push origin claude/*` unprompted; C2 OQ-0006-19 is
   Mustafa's (DENSITY-FIGURE); C3 UNDER-WAY to Mustafa; C4 he approves the operator settings and prompts; C5 the
   kit-change re-check; C6 the laptop's model and effort; C7 the runner's limits for PM-0006-26 and -32; C8 R-14's
   wording; C9 R-PR's scope and exact words; C10 the total vs the drafts; C11 two routes back; C12 SAFARI-TIME's text).
   Rulings on R-1..R-29: all approved except R-10 (changed: the advisor states the cover rule) and R-19 (Mustafa's).
9. Plan revision 3.1 · this commit · 1,374 lines; §0.3 maps C1-C12 and the minor items to where they are met; §15 is
   the ruled table; §16 has five decisions for Mustafa and the two-branch ask.
10. Mustafa approved the plan and answered the five decisions (2026-11-06) · 411824a · revision 3.2 records them (§0.4).
11. **U0, the container probes** (the first build step) · this commit · `u0-probes.md`: Rust 1.99.1 + wasm32 installed;
    a dated nightly exists; crates.io reachable (wtransport 0.7.2, web-transport-quinn 0.13.6, wasm-bindgen-cli
    0.2.130); Chromium 153 headless; **loopback WebTransport with a pinned hash PASSES** (datagram and stream echo;
    maxDatagramSize 1024 B; a wrong hash refused); no IPv6 in the container. Per UNDER-WAY (a) the kit freeze lifts now.
12. **U1 part 1, the spike skeleton** · 760fa89 (pushed; CI runs gate and smoke on PR #40) · `spikes/netcode-proof/`:
    the workspace pinned to Rust 1.99.1 (resolver 3, edition 2024, Cargo.lock: 26 packages), the 15 crates of plan
    §4.1 as compiling stubs wired per §4.2, deny.toml, per-crate clippy.toml bans made errors (a planted f32::sin,
    f32::max and HashMap each failed clippy; probe removed), builds under .gate/ (.cargo/config.toml + .gitignore).
    build, clippy host and wasm32 (leaf crates), fmt: clean. Legacy gate: 1-4 and 5b-5d PASS, +0 -0; 5a cannot start
    here (tools' Playwright wants chromium_headless_shell-1194, absent), so CI's smoke job is the check.
13. **R-PR** · [Mergincanli/Shamble-Frontlines#41](https://github.com/Mergincanli/Shamble-Frontlines/pull/41), head
    2ccc184 on `claude/npt-readings` (worktree `.gate/npt-readings`), ready for review, subscribed · engine-architect
    placed the advisor's exact words + Mustafa's answers (5 docs files, +96 -34; its verify script: word for word,
    tables intact, lines <= 120), then 8 consistency fixes (old lines now point to the readings); the main session read
    the whole word diff. needs-mustafa (ADR edits).

14. **The advisor's review of PR #41** · notes `advisor-pr41.md` (DONE) · of 2ccc184: REVISE (C1 the register's open
    counts; C2 ROUTE written whole; C3 OQ-0001-9's opening sentence "answered in part"; optional PM-0001-3 wording) ·
    fixed in **16763a7** (pushed; gate 1-4 and 5b-5d PASS in the worktree; PR body updated) · re-review of
    2ccc184..16763a7: **APPROVE, no conditions**, head `16763a724b741a40615dc8e7c2ef25c5ea3e4b30`; CI gate and smoke
    green twice each; labelled needs-mustafa; the advisor's worktree `.gate/pr-41` removed.

15. **PR #41 round 3** · netcode-architect found ADR 0001's checklist saying "The profile changes from the overlay"
    twice (the pointer edit plus R-28's words) · fixed in **1b4094a** (the advisor's words unchanged) · the advisor's
    re-review: **APPROVE**, head `1b4094a7750177269b9d5f2fa8c21e292c76e009`, CI gate and smoke green; PR body updated.
16. **U1 part 2a** · d984535 (pushed) · npt-log (records, Writer, read_log; 9 tests); npt-judge's engine and CLI
    (8 unit + 15 engine tests, one per verdict path, PM-0006-14's short padding INVALID among them); `marks/README.md`
    (the contract); `ops/operator/redact-check.mjs` (9 tests; spike, .checkpoints, docs/design: 0 findings);
    rustfmt.toml (ADR 0002 part 7). cargo test 32, clippy host and wasm32, fmt: clean; gate 1-4, 5b-5d PASS.
    A slip, recorded: one `sed -i` edit of engine.rs (shell write) and its `.bak`, removed; the change was the intended
    three lines. A Python `__pycache__` from test-engineer's scratch scripts deleted before any save (they now run -B).

17. **redact-check fixes** · 41e5565 · the automated security review of d984535 found 3 (all real): a host name
    accepted when either reading was allowed; the SRI blanking unanchored; pre-push missing a merge commit's own
    changes. Fixed with tests; the old code (loaded from git into memory) missed all 3, the new code catches each.
18. **U1 part 2b** · 8368869 · `check.mjs`, the spike's test command (fmt, clippy host and wasm32 -D warnings, cargo
    test, the §4.2 dependency rules from cargo metadata, the no-reference grep, its own 8 tests, redact-check); the
    catalogue `docs/design/netcode-proof-messages.md` (netcode-architect) and its class audit test
    (`crates/proto/tests/catalogue.rs`, 15 bites). **`node spikes/netcode-proof/check.mjs`: all 8 steps pass** (35
    Rust tests; deps hold; 246 tracked files, none outside spikes/ references it; 8/8 and 10/10 node tests; redact
    0 findings).

19. **docs-keeper and the drift check** · 54e8b84 · REPO_MAP (spikes/, docs/design/, the spike's section; map check
    OK) and STATUS (kit-v3-3 merged; the netcode proof test in flight with Mustafa's five answers; the freeze lifted;
    the no-Mac words); check.mjs's `expectedMarks`/`marksProblems` with tests; the judge's CLI tests. (The container
    restarted around 11:40 UTC; both agents had finished; their reports were read back from their transcripts.)
20. **U1 part 2c** · 5ac4042 · `marks/marks.json` (81 marks: 74 pass, 1 reported, 3 person, 3 outside; row hashes at
    1b4094a, unchanged since 16763a7) and `marks/SERIES.md` (235 series, 36 figures), machine-copied byte for byte
    from test-engineer's drafts (one node command, main folder, no cd; an exception to Write/Edit-only, for byte-exact
    data); the judge: reports may name parts or pool, outside marks keep bites as data, 36 figures listed as not
    built with their units; the fixtures on the real marks (PM-0001-4 PASS, FAIL, INVALID; PM-0006-14 short padding
    INVALID, full padding NOT-RUN until N = 36 and 18); check.mjs's marks step. **check.mjs: all 9 steps pass** (44
    Rust tests; 81 marks; rows unchanged at 1b4094a; redact 0 findings over 80 files).

21. **The advisor's U1 rulings** · notes `advisor-u1.md` (DONE) on `u1-questions.md` · most APPROVE; two fixes: C2
    (a bug it reproduced: a FAIL plus a gap came out INVALID, hiding the failure and some bites) and B13 (PM-0001-30's
    second bite was dropped); changes for A1, A7, B6-B10, B12-B15, Q10, Q12, Q21, Q22, Q27, C4, C8, C9; nothing for
    Mustafa now (one later readings PR before R5: Q15 (a)).
22. **U1 part 2d** · 3b1212e · the rulings in the judge (C2's run precedence; part validity; `pre:<mark>`; setup
    checks; expect-invalid bites; p001; first-N; PM-0006-19's rule; everyone_in_view_min_share built), marks.json (one
    asserting script in the scratchpad: 16 parts of 12 marks get pre:PM-0001-2, PM-0001-2's N = 24/36 parts, 8 setup
    flags, -30's validity bite, -3's p001/p999, -19's traps, -49's delayer, -67's first 36,000, PM-0006-19's rule), the
    catalogue (SessionStatus, the 15th type; its Q1-Q9 answered), the audit (15 names, a bite), README and SERIES,
    the fixtures (PM-0001-4 carries the load; load-short INVALID, fail-with-gap FAIL; PM-0001-30's bite shown and
    missed), the plan's text (§4.1, §4.2, §4.7, E3/E6, §6.1, §10.2). **check.mjs: all 9 steps pass, 58 Rust tests**;
    gate PASS. On the advisor's own reproduction, the fixed judge shows the bite (it was INVALID).
23. **U1 CLOSED** (the advisor's conditions met: each change with its fixture, check.mjs green, the gate run).

24. **PR #41 merged** · Mustafa's go in this chat ("merge #41", 2026-11-06, after the advisor's APPROVE of 1b4094a;
    the scheduled PR Advisor also approved that SHA in its comment, two Low notes: PM-0001-31's "TCP proxy" wording
    wants a pointer, for the readings PR before R5; STATUS's Mac line, already fixed on this branch) · merged with
    expected head `1b4094a7750177269b9d5f2fa8c21e292c76e009`, gate and smoke green, by merge commit: **main d8c485a**.
    Main into this branch: 1ea36a1. The marks check now reads this branch's own docs/adr/: 0950081 (check.mjs all 9
    green; 81 rows unchanged). The worktree `.gate/npt-readings` removed; unsubscribed from #41.

25. **U2 part 1** · 08782eb (pushed) · npt-link (Link, Listener, Channel, Recv, SendResult, LinkState, Loopback; 7
    tests) and npt-l1 (L1Net, Profile, ReorderStep, FateLog; fates per (seed, link, direction, channel, index) from
    ChaCha8 streams, 16 words a packet; the reorder step as "overtaken by the successor when the timing allows",
    never below the floor; bursts; reliable streams with E15's retransmission rule; RFC 9002 RTT from implicit acks;
    the bites build's four PM-0001-3 defects; 18 tests + 5 bite tests). rand_chacha 0.10.0 / rand_core 0.10.1 pinned,
    sha2 0.11.1 declared. check.mjs: clippy with every feature, tests on both builds, deps parses stdout only.
    check.mjs all 9 pass (83 / 88 tests); gate PASS with --no-smoke.

26. **U2 part 2a** · 921174d (pushed) · `npt-lab probe` (crates/lab/src/{main,probe,machine}.rs; 11 tests) and the
    judge's `fate_log_twin_equal` (figures.rs: `Lack::{Gap, Waiting}`; Waiting = NOT-RUN). One full run: ~1 s, 27 MB;
    P2 seed 1: loss 2.02 % each way, one-way 50.04 ms, RTT 100.07 ms over 610,746 samples (~61 % of down datagrams
    echo: e^-0.98 of 0.1 ms intervals see no new arrival), reorder 910 of 1000; the judge passes every P2 check on it.
    check.mjs fmt, clippy, test green (94 / 99).

27. **U2 part 2b** · 2529675 (pushed) · the twin figure's 3 engine tests (PASS with a shown clock-seed bite; a
    matching bite twin missed = FAIL; differing = FAIL; absent twin = NOT-RUN; five gap cases = INVALID). check.mjs all
    9 pass (97 / 102 tests).

28. **U2 part 2c** · 81c6d01 (the run script; SERIES.md's probe writers) and **6900e52 (results)** · the script at
    81c6d01: 38 runs in 67 s, judged: the five L1 parts PASS (3 valid runs each, 25 checks a run, B1 27); every twin's
    fate log equals its run's; each bite shown by its own check only (rtt-each-way: mean, range, RTT; loss-per-round-
    trip: loss 1.01 %; normal-jitter: p90 54.2 ms, its range passing; clock-seed: the twin check). PM-0001-3 NOT-RUN
    until L2, MP and the not-net-of-base bite. Results: `results/c-u2-pm-0001-3/81c6d01e371b/` (verdict.json, runs.txt).
    Mustafa (2026-11-06): Chrome is installed; he asked for his steps (given in chat: prep now = Firefox, two-factor
    login, a clean Brave profile; then stop B's approvals; then each run's console steps).

29. **The advisor's U2 review** · notes `advisor-u2.md` (DONE; cut once at the 14:31 usage limit, resumed 16:12) ·
    every reading approved (U2-R1..R8); its own checks: check.mjs 9/9, clippy without features, the gate, its own
    release build reproducing the committed fate-log hashes at P2, B1, P4 seed 1, its Python matching the judge's
    numbers exactly, each bite failing only its kind of check, the reorder step on a scratch crate (P0 0 %, P1 0 %, P2
    0.92-1.06 %, P3 0.90-1.02 %, P4 2.2-2.3 %; never below the floor, about 1 % above the ceiling). Conditions N1-N4
    met: N1 the plan (§4.1 ops/runs/ and results/<set>/<commit>/; §4.7 LinkState, rtt_ns, arrival_ns, `loss` at U4,
    npt-lab's binary; §15's R-5 row), N2-N4 in **04e40bc** (crate doc; SERIES.md and README; clippy on both builds).
30. **U2 CLOSED.** check.mjs all 9 (97 / 102 tests, clippy on both builds), gate PASS `--no-smoke`, CI green on
    81c6d01, 6900e52, ad527f3. docs-keeper deferred to U3's close (the repo map check passes; STATUS's line for the
    test is still true at that grain).

## Current step
37. **Both skeptics done (workflow wf_b08132f7-b1c); model and harness corrected** · **cc72f6f** (pushed) · notes
    `u3-verify-fixes-model.md` (B1-B11, D1-D10), `u3-verify-fixes-adr.md`, plan `u3a-fix-plan.md`. Model v2: Linux's
    RTT sampling (FLAG_RETRANS_DATA_ACKED, first_ackt, SACK, TSecr), the timestamp option (ts_recent at rcv_wup; off =
    Windows peer, MSS 1,460), RACK rtt on every advance, RTO marking with RACK's window and cwnd = in flight + 1,
    handshake fixes. Harness: browser lock (one attempt at a time per endpoint, freed 10 ms after the 101), trigger from
    the client's input-echo RTT, writes and Joins on the tick, "decodes deltas again" (first delta after a full state),
    6 sets x 1,000 phases, a 60,000-phase tail with a run's pass chance (480 client-bursts). check.mjs 9/9, gate PASS
    `--no-smoke` (the container's Chromium is 1243, tools' Playwright wants 1194). **Numbers (ms, 60,000 phases):**
    today (64 KB, defaults) 6,000: 746/947/3,170; fix 1 alone p50 435, p99 664, p99.9 1,552, max 5,106 (82 over 1.5 s);
    **fix 1 + one-endpoint reconnect, never abandoned: p99 659, p99.9 1,025, p99.99 1,363, max 1,685 (63 over 1 s, 2
    over 1.5 s); pinned 6,000: 437/665/1,318**; abort at 1 s: 660/1,028/1,691; two endpoints: 637/880/1,918; no
    timestamps (Windows) about the same. **Run pass chance (480 client-bursts), recommended: Mustafa's (0.8 s, 1.0 s)
    0.56; (0.8, 1.5) 0.91; (1.0, 1.5) 0.984; (1.0, 2.0) 1.000.** Fix 1 alone: (1.0, 1.5) 0.38, (1.0, 2.0) 0.69.
    Side effects (never abandoned, 30 min per profile): skipped 4/20/74/116 per mille at P1-P4; false reconnects 0, 0,
    11, 18 (0, 0, 0.37, 0.6 a minute). **Next:** second skeptic round on cc72f6f (model + harness/statistics), then
    WS-FIX, RESUME-CRED, WS-LIMIT back to Mustafa (his rule: a skeptic changed a number and found conflicts).
    **Harness skeptic DONE (u3-verify2-harness.md, H1-H20):** H6 never-abandoned attempts end in a session takeover
    after 99 % of bursts (the old connection had recovered): 480 per run, 48 at once from LG; H11 the gain rests on the
    client OS's SYN retry at 1 s (a 3 s Windows InitialRto: no gain, max 2,879; abort-500 robust, max 1,815); H12
    Firefox's fail delay after a page-closed attempt (200-400 ms; abort 1 s: 370 over 1 s); H17 L2 cannot measure the
    modelled reconnect (DelayStream above TLS: SYN and TLS flights never lost; ~2 RTT faster) and H18 LG is no browser;
    H8 the attempt cap is per run (must be per outage); H7 busy uses room state; H10 server flight pessimistic (TLS
    resumption ~1 segment); "bound to the session" readings 1-3; conflict list updated (429-432 resolved).
    Diag patch: scratchpad/u3v2h/diag.patch. Chromium's WebSocket throttler (H3) grows with failed attempts.
    **Model skeptic DONE (u3-verify2-model.md; patch .gate/u3v2-model-diag.patch):** v2's rules confirmed line by line,
    but three kernel paths are missing and fire in nearly every burst phase, all making things worse: G1 old duplicates
    (end < rcv_wup) discarded whole, ACK field included (tcp_validate_incoming, tcp_input.c:5882-5900): recommended
    479/716/1,143/1,944; B1 no loss undo (Eifel via tcp_packet_delayed, F-RTO 3b, DSACK; tcp_process_loss
    2873-2890, tcp_try_undo_loss 2616-2639): max to 3,296; B2 cwnd = ssthresh at Recovery exit
    (tcp_end_cwnd_reduction 2689-2703; S2's text wrong); B3 no Disorder state (TLPs). Combined (knobs, approximations):
    recommended 496-497/723-787/1,143-1,155/1,890-1,978; fix 1 alone p99 1.2-1.3 s (608 over 1 s): the reconnect is what
    holds a 1 s bar. Seed 1762's 3.17 s path is a model artifact (its ACK rides an old duplicate). T1: S13 (d) passes via
    TSecr now; no test pins a v2 rule. G2 spurious-SYN undo (minor).
    **Stats skeptic DONE (u3-verify2-stats.md):** the pass chance must be the exact joint (fix 1 alone (1.0, 1.5) 0.43
    not 0.38); 600,000 phases needed (recommended v2: max 2,094, 35 over 1.5 s, P(1.0,1.5) 0.975 [0.969-0.981]); a
    "99 % + max" bar misses a broken reconnect (passes 43 %) and a broken budget (87 %); three-part bars bite: at 10
    bursts "half within 0.6 s, at most 1 over 1.2 s, none over 2.0 s" (correct 0.974, reconnect broken 0.136, budget
    broken 0.000); at 30 bursts (1,440) "..., at most 3 over 1.2 s, ..." 0.992/0.007/0.000. Input phase (seed x 7,919 mod
    tick) is a sawtooth: seed it from a hash. The pinned test's goal assertion holds in only 71 % of 6,000-blocks.
    **Decided next (main session):** (1) tell Mustafa now: his three answers do not stand; ask the decisions that do not
    hang on the exact numbers (WS-FIX scope, RESUME-CRED reading, WS-LIMIT form and bursts per run) and his Windows
    Get-NetTCPSetting check; (2) model v3: G1, B1 (with DSACK and F-RTO), B2, B3, G2, T1 + rule tests; harness: H6
    cancel-on-fresh, H7, H8, H19, hashed input phase, Windows SYN and Firefox options; stats: exact joint, 600 sets,
    intervals, three-part bar evaluator, pinned test without the goal claim; (3) a focused third skeptic round; (4) the
    thresholds to Mustafa.
36. **The fallback's three fixes, measured** · **41bd8a0** (pushed) · TcpConfig (budget, thin linear timeouts,
    rto_min), the handshake for reconnects, `fixes.rs` (1,000 B1 phases per configuration). Results (p50/p99/max ms):
    defaults 708/913/2,248; 3-snapshot budget alone 314/1,415/4,203; + thin LT 314/570/995 (but 33-65 % of snapshots
    skipped at P2-P4); rto_min 100/50 worsen the max; reconnect alone 651/772/868; **adaptive budget + RTT-scaled
    reconnect 399/611/738, 0 over 1 s, 0-12 % skipped, ~0.7 false reconnects/min at P3/P4 (pinned)**. ADR 0001:
    fix 3 conflicts with part 10 / Q22 (single-use tokens fetched from the lobby on reconnect; a resume secret was
    rejected); fixes 1 and 2 do not. Sent to Mustafa: WS-FIX, RESUME-CRED, WS-LIMIT (99 % within 0.8 s, every one
    within 1.0 s for the recommendation). README-BRANCH answered yes (restart `claude/npt-readings` from main).
35. **U3a, the TCP model** · **0d25833** (pushed) · npt-net's pub(crate) `tcp_model` (S0-S14; every constant read in
    v6.8, Cubic's 717/1024 included) and its tests S13 (a)-(f) with the three bites, determinism and the write budget
    (12 tests + 1 ignored diagnostic). check.mjs all 9 (109 / 114 tests), gate PASS. **S13 (e)** (1,000 phases, P2, a
    1 s burst, the room's newest-only writer and the client's inputs): first fresh snapshot after the return p50
    708,081,049 ns, p99 914,405,971 ns, max 2,516,541,273 ns (8 phases over 1.5 s, 1 over 2 s); first byte p50 284 ms,
    max 1,509 ms. Pinned in the test. The worst phases: the head's last retry falls just inside the burst (+959 ms),
    the next comes 1.2 s later (backoff 2), then the 30 stale snapshots drain from cwnd 1. Real Linux behaviour, not a
    bug. **The advisor's rule (max x 1.25, rounded up to 0.1 s) gives 3.2 s, against Mustafa's "around 1.5 s"**: a
    decision for him (WS-LIMIT, in chat) before the docs PR opens. Next while he decides: U3b (ImpairSocket).
34. **The advisor's U3 notes DONE** (`advisor-u3q.md`, 21:15 UTC): S0-S14 (DelayStream's Linux-faithful model, every
    Linux line checked against v6.8 but Cubic's 0.7, U3 checks it); C1' replaces U2's C1 (L1's streams model QUIC,
    RFC 9002 as quinn does it; U4); C6 new (U6, before R3: loopback B1 on real quinn, worst over 400 ms goes to the
    advisor); the docs PR's words E1-E4 ({L} = the model's worst recovery over >= 1,000 phases x 1.25, rounded up to
    0.1 s; the fresh-delta reading; R3 +0.2 h, inside its cap); M1-M19 bind U3's build (mapped in u3-build-plan.md);
    U6-1..4 carried. For Mustafa (from the advisor): the docs PR's branch (I propose reusing `claude/npt-readings`,
    which he allowed on 2026-11-06, restarted from main since PR #41 merged); his go on that PR once {L} is measured;
    the extra R1 kernel check when he OKs R1's run plan. Next: U3a, the TCP model (u3-build-plan.md).
33. **U3's build plan** written: `.checkpoints/netcode-proof-test/u3-build-plan.md` (U3a the TCP model and {L}; the
    docs PR for PM-0001-18; U3b ImpairSocket; U3c DelayStream; U3d the timing self-test). The advisor's
    `advisor-u3q.md` §(1), (1b), (2) are written; §(3) (design problems U3 must fix) was still being written when the
    advisor was resumed at 21:12 UTC after the 18:09 usage-limit cut. Next: read §(3) when it arrives, then build U3a.
32. **U3's scouting DONE** (workflow `wf_d6bdeed6-3c0`, 4 of 4 agents, about 17:30 UTC) · synthesis:
    `.checkpoints/netcode-proof-test/u3-scout-summary.md` · **R5-TEST-OQ2: YES for both crates** (wtransport through
    its `quinn` feature's `with_quic_incoming`; web-transport-quinn through `Request::accept`), so R-22 does not
    trigger; a probe proved both here. Verdicts: 26 claims confirmed, 3 refuted, 2 unclear (listed in the summary).
    **Next (after the usage reset):** send the summary's nine questions to the advisor (one request), then write U3's
    build plan from the confirmed claims and its rulings, then build ImpairSocket and DelayStream.
31. (done) U3: the scouting workflow `wf_d6bdeed6-3c0` ran; then build L2 from its verified findings.
    **State at 16:45 UTC (Mustafa: usage at 90 %):** both scouts' notes are finished (`u3-scout-wt.md`,
    `u3-scout-timing.md`, each ends DONE); the two verifiers are writing `u3-verify-wt.md` and `u3-verify-timing.md`.
    **If the session is cut off:** (1) read the four u3-* notes; (2) if a verify file does not end with DONE, resume
    the workflow with Workflow({scriptPath: the script in Running work, resumeFromRunId: "wf_d6bdeed6-3c0"}) (the
    finished agents come back from cache); (3) then write U3's build plan from the confirmed claims only (refuted or
    unclear ones are re-checked by hand first), and build npt-net's ImpairSocket and DelayStream per plan §12.1's U3
    row. Probe sources in the notes folder (u3-*-probe.rs, u3-tcp-stall-probe.py) are throwaway and never built
    into the spike.
    **The old U2 list, kept for its record:**
    **Remaining from the old list:**
    2. (done) A run script, `spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs` (Write tool): clean tree check; release builds
       of npt-lab without and with `--features bites`, each copied to `.gate/netcode-proof-test/bin/`; **every L1 part
       has its own twin check (P1-P4 and B1: marks.json)**, so 15 measured runs (seeds 1-3) + 15 twins (role twin) +
       4 bite runs at B1 or P2 (clock-seed at B1 with its own twin, both bites build) = 35 runs, ~1 GB under
       `.gate/netcode-proof-test/runs/u2/`; then `npt-judge --only PM-0001-3`. Expected: the five L1 parts PASS, the L2
       and MP parts and the not-net-of-base bite NOT-RUN. Check each bite fails for its own reason (rtt-each-way: mean;
       loss-per-round-trip: loss; normal-jitter: p90; clock-seed: the twin check).
    3. Commit the verdict summary under `spikes/netcode-proof/results/` (no raw logs); SERIES.md's "written by" for the
       probe names: npt-lab (L1), npt-net (L2).
    4. check.mjs all 9, gate `--no-smoke`, then the advisor on U2 with these readings: the reorder step's model
       (overtaken, causal: no swap where the spacing exceeds the delay, so P0 and P1 at 30 Hz have none); L1's stream
       model (E15's RTO rule); RTT from implicit acks; Link's additions (`LinkState`, `rtt_ns`; no `loss` yet, which
       npt-room wants per client at U4); the probe in npt-lab with its own binary (plan §4.7 lists no npt-lab binary).
    R1 still waits for stop B.

## Next steps
1. On Mustafa's "merge #41": check gate and smoke green on 1b4094a, merge with that SHA as the expected head; then
   `git merge origin/main` into this branch (only reviewed main comes in) so marks.json's row hashes match; remove
   the worktree `.gate/npt-readings`.
1b. The catalogue's questions Q1-Q9 (netcode-architect's notes `netcode-architect-catalogue.md`) and test-engineer's
   marks questions go to the advisor together, in one request.
2. U1 part 2: npt-log's record types and writer; npt-judge with its PASS, FAIL and INVALID fixtures (PM-0006-14's
   short padding among them); marks.json (test-engineer drafts it per `marks/README.md`, with SERIES.md); the drift
   test (after the merge); the dependency-rule checks (a cargo metadata test in npt-lab) and the no-reference grep;
   redact-check; the message catalogue (netcode-architect) and its class audit test; then docs-keeper (REPO_MAP gets
   spikes/ and the catalogue; STATUS: PR #38's line, the Mac line at 44-45, the plan's approval, UNDER-WAY (a)).
3. For Mustafa, before R1: Chrome and Firefox (stable) on the laptop; MSI GPU switch on hybrid if offered.
4. Write files with the Write and Edit tools only, never shell redirects or scripts that write (Mustafa, 2026-11-06:
   the guard can't follow cd, so such writes ask him); run commands from the main folder without cd.

## Open findings
- **From the advisor's U2 review (advisor-u2.md), each with the unit that fixes it:**
  - **C1 (U4, before PM-0001-18's L1 leg counts):** L1's stream retries back off (each waits twice the last timeout:
    RFC 6298 §5.5, RFC 9002 §6.2.1); loss drawn per 1,448 B segment, any lost segment delaying the message one timeout
    and holding back the ones behind it (E15's L2 model); unit tests; L1 runs record each stream's stall share.
  - **C2 (U4, before npt-room reads `rtt_ns`):** a sample only from a packet that is the newest of its link and
    direction to arrive (RFC 9002 §5.1); a stream message's sample from its own packet's arrival, not its in-order
    delivery; tests for both.
  - **C3 (U4):** `loss` on Link, with §4.4's send-buffer space settled at the same time; each backend reports its own
    estimate, never the simulator's true fate; plan §4.7 then.
  - **C4 (U4, with the next judge change):** INVALID, with fixtures: a measured run (or its twin) whose build lists
    `bites` or names a bite; a bite run whose build lacks `bites`.
  - **C5 (U3/U6):** ImpairSocket's reorder step uses L1's model; SERIES.md names the binary that writes L2's probe
    log, not only the npt-net library.
  - **Note (U6):** links are `Box<dyn Link>` without `Send`; if npt-server moves links across tasks, the L2 backends
    need `+ Send`.
- Review round 1's 49 findings: fixed in revision 2 (§0.1). Review round 2's 46 findings: fixed in revision 3 (§0.2). The advisor's C1-C12: folded into revision 3.1 (§0.3).
- **Carried forward from the advisor's U1 rulings** (advisor-u1.md): C4 (`params.killed` only with npt-super's kill
  record) and C6 (tokens only in their prefixed form, with a test) at U6; A3's systemd readiness check for real at
  R1's bring-up, before any PM-0001-16 start counts; ImpairSocket's release precision at U3 (worse than 0.25 ms goes
  back to the advisor before R1); the results ledger (invalid runs superseded only with a written cause; failing
  runs only by a build fixing the found cause; the judge refuses a folder missing a listed run) at U8, reviewed by
  the advisor; Q13 (R4 creates LG with its IPv6 /64) and Q23 (-11 and -25 once per WebTransport crate) in the run
  plans; Q15 (a), -27's one-sided "within 5 %", in a readings PR before R5's first counted run (Mustafa's go).
- **For the readings PR before R5** (with Q15 (a)): the PR Advisor's Low note on PM-0001-31's "L2's TCP proxy on RS",
  which R-25 makes the WebSocket stream wrapper for the proof test: a pointer in that row.
- **Raised by the session, for the advisor at U4:** marks -14, -16 to -25 and -27 run inside an S30 room or the stress
  scene by their rows, but their parts do not select on the scene, so `pre:PM-0001-2` is not on them yet. When the
  L1 runner and the run plans name each run's scene, their selects add scene and clients, and the precondition with
  them.
- **A nuance, recorded:** an expect-invalid bite counts as shown if its rule reports a missing counter too; that
  cannot give a false PASS, because measured runs without the counter are INVALID by the same rule.

## Key numbers
- Hetzner, excl. VAT, read 2026-11-05 (infra digest D): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h (plan prices include
  IPv4); 1 RS + 1 LG = 0.1562/h; each VM rounds up per session; RS<->LG traffic internal and free.
- Rust stable 1.99.1 (2026-10-13; static.rust-lang.org answers 200); container has 1.97.0, x86_64 target only.
- macOS runner images: macos-26 arm64 Safari 26.6.2; macos-15 arm64 Safari 27.2 (infra digest F).

## Facts from Mustafa
- **2026-11-07 about 11:10 UTC, his Windows laptop:** `Get-NetTCPSetting | Select-Object SettingName, InitialRtoMs,
  MaxSynRetransmissions` gave MaxSynRetransmissions 4 for every template (Internet, InternetCustom, Datacenter,
  DatacenterCustom, Compat; Automatic blank) and InitialRtoMs blank (the built-in default). Read with: Catchpoint's
  write-up (docs.catchpoint.com/docs/catchpoint-nodes-tcp-syn-retries: "Windows 10 is configured out of the box with
  an initial RTO of 1 second and 4 Syn Retransmits", SYNs at 0, 1, 3, 7, 15 s, final timeout 21 s) and a Windows 10
  1909 `netsh interface tcp show global` dump (my-computer-memo.blogspot.com, 2020-01: "Initial RTO : 1000",
  "Max SYN Retransmissions : 4"; earlier Windows used 3 s). So a current Windows client retries a lost SYN after 1 s,
  doubling: the "Windows-like client, first SYN timeout 1000 ms" rows apply (never abandoned 645/886/1,484/2,871 on
  60,000 phases); the 3 s rows do not. Optional direct reading on his machine: `netsh interface tcp show global`
  (its "Initial RTO" line).

## Decisions taken
- **Mustafa, 2026-11-06 about 23:40 UTC:** "WS-FIX: (a). RESUME-CRED: (a), with the spare tokens kept in memory only,
  short-lived and bound to the session. WS-LIMIT: 99 % of client-bursts within 0.8 s, and every one within 1.0 s.
  All three stand unless the skeptics change a number or find a conflict; if they do, bring it back to me." So: the
  fallback gets the newest-only send budget sized to the round trip (1.5 round trips of snapshots, at least 3) and
  the client's reconnect after max(300 ms, 2 x RTT + 100 ms), one attempt every 300 ms in parallel (at most 8), the
  session resumed on the newest Join; no thin-stream timeouts or lower rto_min; reconnect credentials are spare
  single-use tokens held in memory only, short-lived and bound to the session; PM-0001-18's WebSocket part: 99 % of
  client-bursts within 0.8 s and every one within 1.0 s (WebTransport keeps 500 ms). Pending: the verification
  workflow wf_b08132f7-b1c; then the advisor's exact words for the docs PR on `claude/npt-readings` (restarted from
  main), needs-mustafa.
- **Mustafa, 2026-11-06 about 21:40 UTC, on WS-LIMIT and README-BRANCH:** "WS-LIMIT: not yet. Before we set the
  numbers, make the fallback recover faster. Try three standard fixes in the TCP model first: 1. Newest snapshot
  only: while the WebSocket is backed up, the room keeps only the newest snapshot instead of filling TCP's 64 KB
  buffer with ~30 stale ones, and keeps the socket's send buffer small. 2. No doubled waits on the server: enable
  Linux's thin-stream linear timeouts for the game's sockets (tcp_thin_linear_timeouts / TCP_THIN_LINEAR_TIMEOUTS),
  and consider a lower minimum retransmission timeout, so a retry just before the outage ends doesn't cost another
  1.2 s. 3. Fast reconnect on the client: if no data arrives for about 300 ms, start a fresh WebSocket connection
  attempt every 300 ms, in parallel, and resume the session on whichever connects first, instead of waiting out TCP's
  backoff. Goal: every recovery within 1.5 s, and 99 % well under 1 s. Measure again (1,000 timings), then bring
  WS-LIMIT back with the new numbers, in the two-part form (a 99 % bar plus a maximum), for my go. If a fix conflicts
  with ADR 0001, tell me which and why. README-BRANCH: yes, restart claude/npt-readings from main."
- **Mustafa, 2026-11-06, about 17:55 UTC (with his Advisor), U3-Q1 to Q9:** "I agree with the recommendations on
  U3-Q1, Q2, Q4, Q5, Q6, Q7, Q8 and Q9. U3-Q3: (b), model Linux TCP faithfully, so the result shows how the fallback
  really behaves. My pre-decision for the mark: PM-0001-18 keeps 500 ms for WebTransport; the WebSocket fallback gets
  its own realistic recovery limit, set from the faithful model with a margin (around 1.5 s). Bring that as the docs
  PR for my go. U3-Q6 note: migration stays off for the test; whether the real game allows it (players switching
  networks mid-match) is a later ADR 0001 question." So: Q1 (c) expected excess lateness beyond 0.25 ms under about
  10 us at the tightest profile, at 20,000 releases/s on RS, percentiles reported; Q2 (b) 1 ns timer slack via the
  /proc file, real-time priority a recorded switch, never a spin; Q3 (b) Linux-faithful DelayStream (RACK, TLP, RTO
  with doubling), and a docs PR (needs-mustafa) splitting PM-0001-18: WebTransport 500 ms; WebSocket = the model's
  measured recovery plus a margin (about 1.5 s), the number from U3's measurement; Q4 (b) the wrapper above TLS,
  below tungstenite; Q5 pin tokio-rustls in U3, audited at U5; Q6 (a) migration off for the test, recorded (the
  game's own rule: a later ADR 0001 question); Q7 reuse npt-l1's Profile and ReorderStep plus one small L2 config
  type, added to §4.7; Q8 ImpairSocket stamps each peer's first raw packet for the Join deadline, oversize datagrams
  dropped and counted before buffering; Q9 RS's kernel, HZ, cpuidle and clocksource read at R1 (R1's run plan).
  The advisor (running on the same questions) was told his answers decide; it finishes only the faithful model's
  spec, the docs PR's exact words, and which design problems U3's build plan must fix (advisor-u3q.md).
- Mustafa, 2026-11-06 (later): "No Mac available for now (the loan Macs are out of stock). Nothing in this test waits on
  one: the timed Safari rows wait until a Mac is available, at the latest for the look proof test." So if the runner
  fails its conditions, every Safari row waits for that Mac too (sent to R-PR's agent for SPIKE_PLAN).
- Mustafa, 2026-11-06: plan APPROVED (revision 3.1 with C1-C12). ROUTE (a) on his laptop (MSI Vector 18 HX AI, Core
  Ultra 9, RTX 5080, native Windows/PowerShell; Brave + Edge, Chrome/Firefox installable; Claude Code CLI on his
  university's Enterprise login; Manual mode). SAFARI-TIME (b). FIXED-IP (a), deleted at the close. DENSITY-FIGURE (a).
  UNDER-WAY (a): when the plan is approved and the first build step is committed. Branches `claude/npt-readings` and
  `claude/npt-safari-wf` allowed.
- The advisor, 2026-11-06: the plan APPROVED with C1-C12 (folded in); readings ruled as plan §15 shows; R-19 and
  UNDER-WAY escalated to Mustafa; R-PR's exact words in `advisor-verdict.md`.
- Settled inputs (Mustafa, this session's prompt, 2026-11-05): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms
  round trip, +-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only; CCX23 (RS, x86) + CAX21
  (LG, Arm64) in Falkenstein, created and deleted by Mustafa per run; SSH by a public key the session gives him, never
  a Hetzner token; each run's plan states its hours and a cost cap for his OK; Safari leg tries GitHub's macOS runners
  first, any `.github/` change goes to him; shamblefrontlines.com at Cloudflare, one exact DNS record per test host,
  always "DNS only" (grey cloud).

## Running work
- **Merged v3 = 1e020a7 (pushed)**: model v3 + harness v3 copied whole from the two worktrees (spikes/ equal to
  72e6fab first), Windows-like client wired (client_cfg), phases censored at 60 s (none were), pin re-set
  (0x52e7_b772_29d8_5ebb, [467,77,32,10], [5998,3999,946,1056]); check.mjs 9/9 (146/151 tests), gate PASS
  `--no-smoke`. Tables: .gate/netcode-proof-test/u3v3-tables1.txt. **600,000 phases:** recommended 654/1,051/1,859/
  p99.99 2,092/max 4,196 (6,768/1,009/205 over 1/1.5/2 s); fix 1 alone 654/2,164/max 11,384; fix 3 alone 844/1,374/
  max 5,343; today 1,121/2,686/max 12,068. Two-part (1.0,1.5) rec 0.227. Search, three-part: 480 "half within 0.7 s,
  at most 6 over 1.3 s, none over 3.0 s" 0.997 [0.993-0.999], broken builds 0.000; 1,440 "half 0.7, at most 16 over
  1.2, none over 3.0" 0.990. Windows-like 3 s SYN: never-abandoned no gain (p99 2,035), abort-500 robust (p99 903).
  Multi-burst rec 10/30 bursts ~ single bursts (Cubic ends the ratchet); abort-500 Chromium 30 bursts throttle to 5 s.
- **Third skeptic round on 1e020a7** · workflow `wf_fed73d64-ec0` (npt-ws-fixes-verify3; rust-reviewer model v3,
  netcode-architect harness v3 + policy + ADR, test-engineer statistics + marks.json/npt-judge; xhigh) · launched
  about 11:05 UTC · script:
  `/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-ws-fixes-verify3-wf_fed73d64-ec0.js`
  · notes `u3-verify3-{model,harness,stats}.md` · if cut: Workflow({scriptPath, resumeFromRunId: "wf_fed73d64-ec0"}),
  editing a cut agent's prompt to continue after its notes' last block first. Then: fold, final numbers to Mustafa.
- (done) **Model v3 build** (general-purpose agent, opus, xhigh, isolated worktree; tcp_model.rs + tests.rs only) · launched
  about 05:45 UTC 2026-11-07 · notes `u3-build-v3-model.md` (ends DONE) · G1 discard, D-SACK, the undo machinery,
  F-RTO 2b/3a/3b, Disorder, Recovery-exit cwnd, PRR if faithful, Cubic, G2, a client SYN-timer knob, rule tests,
  S13 (d) without timestamps, PINNED_E re-pinned · returns its worktree path (uncommitted). If cut: SendMessage the
  same agent after the reset (its context is kept), never a new one.
- **Harness and statistics build** (general-purpose agent, opus, xhigh, isolated worktree; fixes.rs only) · launched
  about 05:46 UTC · notes `u3-build-v3-harness.md` · H6 cancel-on-fresh + post-recovery takeovers, H7, H8, H19, hashed
  input phase, Chromium throttler and Firefox fail delay options, TLS resumption flights, a multi-burst run harness,
  exact joint pass chances with intervals over 600,000 phases, three-part bars and bite rows, a hash pin · leaves a
  slot for the Windows SYN option. **After both:** apply both diffs to spikes/ (git diff | git apply, no merge),
  wire the SYN option, re-pin, run the tables, check.mjs, gate, commit, then a focused third skeptic round.
- **Worker restart about 10:15 UTC stopped both builds.** Worktrees kept: `.claude/worktrees/agent-a79d7290e0e6a6e50`
  (model: tcp_model.rs partly edited, notes inside the worktree, Progress empty) and
  `.claude/worktrees/agent-a487e419ee806d37b` (harness: near the end, re-running the bar search). Both resumed about
  10:20 UTC by SendMessage, told to mirror notes to the main folder. `.claude/worktrees/` is excluded locally in
  .git/info/exclude (never committed). The guard refuses `git -C`: run git from inside each worktree. Saver
  `bydoy9piu` started about 10:20 UTC (2 h limit).
- **Harness build DONE** (about 10:25 UTC; notes copied to `u3-build-v3-harness.md`, the Write tool refuses the main
  folder from an isolated worktree): fixes.rs only, +1,664/-425, check.mjs 9/9 in its worktree, uncommitted in
  `.claude/worktrees/agent-a487e419ee806d37b` (branch worktree-agent-a487e419ee806d37b). On the v2 model: H6 cuts
  post-recovery takeovers from ~99 % of phases to 0.6 %; 600,000 recommended 435/661/967/1,448/2,067 (532/260/40/1
  over 1/1.2/1.5/2 s); three-part bar at 480 "half within 0.6 s, at most 1 over 1.2 s, none over 2.0 s" rec 0.981
  [0.975-0.985], fix 1 alone 0.141, fix 3 alone 0, today 0; at 1,440 "..., at most 3 ..." 0.994/0.008/0/0.
  Multi-burst lineages degrade (cwnd ratchets 10 -> 7 -> 4 -> 2 with no Cubic: re-run after model v3). Chromium's
  throttler builds up over 30 bursts (rec longest 444 ms; abort-500 4.9 s). Windows SYN slot: ClientOs::Windows,
  client_cfg()'s arm unimplemented!. Pin: (0x20e2_d621_a6c1_1f84, [31,4,4,0], [6000,5916,56,28]), re-pin after v3.
- (done) **Second skeptic round on cc72f6f** · workflow `wf_a8e0f590-0bc` (npt-ws-fixes-verify2; rust-reviewer on model v2,
  netcode-architect on the harness vs browsers and ADR 0001, test-engineer on the bar statistics; xhigh) · launched
  about 04:08 UTC 2026-11-07 · script:
  `/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-ws-fixes-verify2-wf_a8e0f590-0bc.js`
  · notes `u3-verify2-{model,harness,stats}.md` (each ends DONE) · if cut: resume with Workflow({scriptPath,
  resumeFromRunId: "wf_a8e0f590-0bc"}); an agent cut mid-way: edit its prompt to "continue after your notes' last
  block" first (the first round's pattern). Then: fold its findings, bring WS-FIX, RESUME-CRED, WS-LIMIT to Mustafa.
- Saver: background task `b1t71q312` · started about 04:31 UTC 2026-11-07 with a 2 h background limit (110 minutes,
  to about 06:21); `btvszgr34` was killed at the 30-minute default limit (start savers with timeout 7200000).
- (done) **Verification of the fallback fixes** · workflow `wf_b08132f7-b1c` (npt-ws-fixes-verify; rust-reviewer on the
  model and numbers, netcode-architect on the ADR 0001 reading; xhigh) · launched about 23:36 UTC · script:
  `/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-ws-fixes-verify-wf_b08132f7-b1c.js`
  · notes `u3-verify-fixes-model.md`, `u3-verify-fixes-adr.md` · if cut: resume with Workflow({scriptPath,
  resumeFromRunId}). Its result may correct what WS-FIX, RESUME-CRED and WS-LIMIT told Mustafa; tell him if so.
- Saver: background task `bmsomg1pb` · started about 23:36 UTC (110 minutes, to about 01:26).
- **The advisor on U3's nine questions** (advisor agent, xhigh, background) · launched about 17:40 UTC at Mustafa's
  "8 % left, so we can already start" · reads `u3-scout-summary.md` · notes `advisor-u3q.md` (ends DONE) · if cut by
  the usage limit: SendMessage to the same agent after the reset (its context is kept), never a new one. The same
  nine questions went to Mustafa in chat, so his own Advisor can weigh in; his answers override.
- Saver: background task `bl4ghplet` · started about 17:40 UTC (110 minutes, to about 19:30); no model usage.
- **U3's scouting workflow** `wf_d6bdeed6-3c0` (npt-u3-scout; 2 scouts + 2 adversarial verifiers, xhigh, at most 2 at
  a time) · launched about 16:20 UTC · **DONE about 17:30 UTC** (4 of 4; synthesis in u3-scout-summary.md) · script:
  `/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3-scout-wf_d6bdeed6-3c0.js`
  · notes `.checkpoints/netcode-proof-test/u3-scout-{wt,timing}.md` and `u3-verify-{wt,timing}.md` · questions:
  each WebTransport crate's server over a custom quinn AsyncUdpSocket (R5-TEST-OQ2); quinn 0.11.12's socket trait and
  GSO/GRO; R-17's queue depths; the L2 probe; timer precision (tokio's resolution, measured lateness here vs the
  0.25 ms bar); Linux's TCP retransmission (TCP_RTO_MIN, RACK-TLP, TLP) against E15's rule; tokio-tungstenite's
  wrapper and limits; a DelayStream sketch. Resume with Workflow({scriptPath, resumeFromRunId}) after a cut.
- U3 part 0 · ad527f3 (pushed) · npt-net's crates pinned (wtransport 0.7.2, web-transport-quinn 0.13.6, quinn 0.11.12,
  tokio 1.54.1, tokio-tungstenite 0.30.0; floors met: quinn-proto 0.11.19, rustls 0.23.46, ring 0.17.14); check.mjs
  all 9 (97 / 102).
- **The advisor on U2** (resumed 16:12 UTC by SendMessage after the 14:31 usage-limit cut; it had written nothing) ·
  launched 14:31 UTC · notes
  `.checkpoints/netcode-proof-test/advisor-u2.md` (ends DONE) · rules on U2-R1..R8 (reorder model, L1 streams, L1 RTT,
  npt-link's surface and `loss`, the probe's placement, the run set and records, check.mjs, the twin figure's
  semantics) on `git diff c104302..6900e52 -- spikes/`. Before it: check.mjs all 9 green (97 / 102), gate PASS
  `--no-smoke`, repo map OK. If the session restarts: read its notes file; if not DONE, relaunch with the same brief.
- Saver: background task `btui06lbn` · started 14:31 UTC · stopped by the session 16:12 (replaced).
- Saver: background task `brd3l2ph7` · started 16:12 UTC (110 minutes, to about 18:02).
- PR: [Mergincanli/Shamble-Frontlines#40](https://github.com/Mergincanli/Shamble-Frontlines/pull/40), draft, opened
  02:56 on head a6607bc; this session is subscribed to its activity; safety-net check-in `trig_017Q5fgnrrQsugi3dbZ9pa3L`
  fired 03:45 Amsterdam: nothing new (draft, head b95a3dd, no checks, comments or threads; main unmoved); check-ins
  stopped, since Mustafa has not written since the PR went up. The next GitHub event or his message resumes the work.
- The advisor's plan review (advisor agent, xhigh) · launched 02:54 · DONE 03:04: APPROVE with C1-C12.
- Workflow `wf_e7cdd40f-712` (npt-plan-review2: security-auditor, netcode-architect, infra-engineer; xhigh) · script as
  launched: `.checkpoints/netcode-proof-test/npt-plan-review2-run1.js` (no args) · notes: `<agent>-plan-review2.md` ·
  launched 23:54 · FAILED 23:54 at the usage limit · resumed 02:13 · DONE 02:44 (3 of 3 returned).
- Workflow `wf_88b642c1-384` (npt-plan-review) · script as launched: `.checkpoints/netcode-proof-test/npt-plan-review-run1.js`
  · launched 23:16 · DONE 23:37 (3 of 3 returned).
- Workflow `wf_1ae38d53-da9` (npt-plan-digests) · script as launched: `.checkpoints/netcode-proof-test/npt-plan-digests-run1.js`
  · launched 22:38 · DONE 23:00 (3 of 3 returned; journal in the session's workflow folder).
- Saver: background task `btpf0ptzw` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/btpf0ptzw.output`
  · started 22:35 · stopped 00:25 ('time is up').
- Saver: background task `be4kec0xf` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/be4kec0xf.output`
  · started 02:11 · stopped by the session at 03:18 (no agent running; final saves by hand).
- Saver: background task `bar6y14h2` · started 10:36 · stopped by the session 12:18 (replaced).
- Saver: background task `bekt3thjk` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/bekt3thjk.output`
  · started 12:18 · stops by about 14:08 (110 min).
- The advisor's review of PR #41 (advisor agent, xhigh, background) · notes `advisor-pr41.md` · DONE: REVISE of
  2ccc184, then APPROVE of 16763a7 (re-review via SendMessage, same agent).
- The message catalogue (netcode-architect, xhigh, background) · notes `netcode-architect-catalogue.md` · writes
  `docs/design/netcode-proof-messages.md` · launched 11:12 · DONE 11:20 (Q1-Q9 for the advisor).
- marks.json's draft (test-engineer, xhigh, background) · notes `test-engineer-marks.md` · writes `marks-draft.jsonl`
  and `series-draft.md` · launched 11:16 · stopped at the usage limit, resumed 12:12 · DONE (81 marks, 0 validator
  errors, Q1-Q31).
- docs-keeper (xhigh, background) · notes `docs-keeper-u1.md` · launched 12:22 · DONE (REPO_MAP, STATUS; map OK).
- The advisor on U1's questions (advisor agent, xhigh, background) · notes `advisor-u1.md` · launched 13:15 · DONE
  13:29.
- Saver: background task `brikhjzxz` · started 13:14 (the restart had ended `bekt3thjk`) · stopped about 13:45 by the
  shell's default 30-minute limit for background commands (no agent running then). Restart it with a 2-hour timeout
  whenever an agent runs.
- R-PR's text: engine-architect (xhigh) · DONE (placements, then 8 fixes); its scripts rpr-edit.py, rpr-patch-c.py,
  rpr-verify.py, rpr-words.json and notes engine-architect-rpr.md in the notes folder.
