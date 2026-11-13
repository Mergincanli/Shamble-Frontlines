# History: netcode-proof-test

Moved here byte for byte from .checkpoints/netcode-proof-test.md by RESUME-CARD (Mustafa, 2026-11-10). The
checkpoint's resume card is the handoff; open this file only when a step needs the history.

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

## Decisions taken (to 2026-11-11, newest first; later ones are appended at the end, each under "## Decision: …")
- **Mustafa, 2026-11-11 about 16:45 UTC (verbatim):** "My words of 2026-11-11, for the netcode chat (PR #40, the
  netcode proof test).
  First, before anything else and without agents: save what exists only in a work folder.
  - U4j's fix pass is not on the branch. wip/u4j.patch was saved at 09:54 UTC, before the fix pass began, and
    u4j-build.md on the branch has no "Fix pass" section.
  - Put the fixed code, the builder's notes with "Fix pass, round 1" and its runner on the branch: snapshot U4j's
    worktree again with worktree_patch.py and copy the notes into the checkpoint folder. Do the same for U4d-F's
    worktree with its partial edit.
  - Commit, push, and tell me the SHA.
  - If a worktree is gone or differs from what the card says, stop there and tell me.
  USAGE-SAVINGS: (a), all three, with these limits.
  - Builders: pass or fail comes from the exit code and the test runner's own summary line, never from a grep. A
    fresh copy reads its notes and the diff in its worktree before it changes anything.
  - The main session still reads in full every finding rated Medium or higher, and everything that touches the
    judge, the marks or a pass bar.
  - The review packet is cut by script and holds no conclusion of the builder or the main session. It ends with the
    ids of the carries it left out. Reviewers stay free to read any file and run any test.
  - Give me the chat's size in every report. I type /compact when you tell me it is at 35 to 40 %, after you have
    refreshed the card.
  NIGHT-TONIGHT: (c). One item: the test-engineer's check of the applied text batch, with the script's log as its
  brief.
  - It says in one line each, for R-31 and for U3D-MISS's sampling case, whether any pass bar's number or direction
    moved.
  - If this item does not fit the night shift's rules, post none tonight and tell me.
  - PR #40's report lists those two edits as readings for me to confirm.
  The hold lifts when the save is pushed. Restart the saver, then follow the card's order: U4j's round 2, then the
  U4e brief's check, then U4d-F.
  In your next report, from your notes and without agents: a small table with, for each reviewer role, how many
  findings rated Medium or higher only that role found in wave 1's units (U4s, U4a, U4b, U4c, U4d, U4j). The review
  panels stay as they are until I decide."
  - Done: the save, 4ad80a9 (both worktrees matched the card): wip/u4j.patch (U4j's fixed build on 411eeee, 64
    files, applies cleanly), u4j-build.md with "Fix pass, round 1" (the branch's 135 lines plus 199 added),
    u4j-fix/run.sh; wip/u4df.patch (U4d-F's world.rs edit on dad9566, applies cleanly) and u4df-build.md. The hold
    lifted with that push.
- **Mustafa, 2026-11-11 about 16:25 UTC, after his /compact (verbatim):** "You were just compacted to cut usage. Read
  the resume card in .checkpoints/netcode-proof-test.md first and continue from it. Do not redo finished work.
  Then, without agents and in ten minutes at most:
  - a rough usage split for the last 24 hours from the session's own figures, in five lines: main-session turns,
    builders, reviewers, advisor rulings, everything else;
  - your three biggest savings that do not weaken the judge, the marks or any pass bar.
  Apply the advisor's text batch yourself when the main session is free; no agent for it."
  - Done: the split and the savings in usage-20261111.md (script usage24.py), reported in the chat; the text batch
    applied by the main session with text_batch.py (every block and quoted string read from advisor-u4d-close.md by
    line range), no agent. The HOLD stands.
- **Mustafa, 2026-11-11 about 16:15 UTC (verbatim; the HOLD):** "Usage on this account is the bottleneck, and this chat
  is 70% full: every request re-sends the whole chat. Do this now and start no new agent until I say so:
  1. Update the resume card so a compacted chat can carry on from it alone: the current step, the three running agents
     (task ids, worktrees, what to do with each result), my open decisions and the next steps. Push it and give me the
     commit.
  2. Hold the advisor's text batch. Do not give it an agent slot; the main session applies those edits later.
  3. Then stop and tell me. I will run /compact."
  - Done: the card rewritten (the old body moved to "Running work, U4 wave 1 (2)" by card_refresh2.py), pushed; no
    agent started; the text batch held for the main session. The three workflows had already been cut by the spend
    limit (about 13:00-13:30 UTC), so none was running at the hold.
- **Mustafa, 2026-11-11 about 11:50 UTC (verbatim):** "NIGHT-BUDGET: (a). Leave it as it is. Switch to (b) on the
  first night whose queue does not fit, and tell me when you do.
  Three notes, no new work:
  - The event-rate drift in the estimate scene: fix the scene before U4d2 pins the recordings. The 9.5 floor
    (PM-0001-2) does not move.
  - The Low about the effect row's owner: defer it to U4e in writing. A Low never triggers a push.
  - U4j's High: carry on as the review budget says. If it is still open after round 3, stop and bring it to me.
  In your next report, tell me how full this chat is (the session's own usage figure)."
  - Read as:
    - NIGHT-BUDGET (a): the day session keeps working into the evening. On the first night whose queue does not fit,
      the day session switches to (b), starting no new agent work after about 21:00 Amsterdam, and tells him.
    - N2-M1, the event-rate drift: fixed in the estimate scene before U4d2 pins any recording; the 9.5 floor stays.
      Written as carry 52 (u4s-carries.md, §U4d2).
    - SEC-U4d-R2-1, the effect row's owner: deferred to U4e, in writing as carry 51 (u4s-carries.md, §U4e). No push
      for it.
    - U4j's F1 (High): round 2 as planned, a round 3 for F1 alone if it stays open; still open after round 3, the
      change-set stops for him.
    - Each later report gives the session's context figure (get_session's context_usage).
- **Mustafa, 2026-11-11 (verbatim):** "PR #49 is merged. STATUS is split (dashboard in docs/STATUS.md, my decisions in
  docs/DECISIONS.md, the rest in docs/STATUS_ARCHIVE.md). At your next natural stop, not in the middle of U4: update
  from main, move your block of my 2026-11-06 answers into docs/DECISIONS.md by script and line ranges, never by
  retyping, and keep one line for #40 in STATUS's "In flight" table."
  - **Done at 5ca77eb, about 09:58 UTC.** The natural stop: no merge or unit step was in flight in the main folder;
    the lanes worked in their own copies.
  - The move: by merge_status_split.py, each block by line range, every moved line checked in its new home.
    - DECISIONS.md: the 22-line answers block and MACHINES-TIMING's note.
    - STATUS_ARCHIVE.md: the Phase list, the follow-ups line, the risks entry, a one-line fix and the two Health rows.
    - STATUS.md: the one #40 line, updated.
  - Checks: the repo map OK; marks, refs and redact PASS; gate PASS.
- **Mustafa, 2026-11-10 about 20:40 UTC (verbatim):** "NETEM-BAR (may the WebSocket pass bar loosen when real Linux
  disagrees with the model): (a). The bar stays capped as WS-BAR says. If a WebSocket row fails because the kernel is
  slower than the model, report it to me as the kernel's, with the netem numbers, and I decide." And: "The night
  shift: I am setting it to CRON_TZ=Europe/Amsterdam 10 0-7 * * * myself tonight."
  - The U8 row's TCP-FREEZE text (W1-PLAN) takes NETEM-BAR (a) at the next plan edit.
  - NS-START: his edit, so nothing is left for the session.
- **Mustafa, 2026-11-10 about 06:45 UTC (verbatim): "A"**, to the two open questions, both of whose recommendations
  were (a). Read as both:
  - **SQUASH-CITES (a):** PR #40 squashes; the closing commit re-points the four citations (bf9d248, 975e72b, 95a04a1,
    1e020a7 in ADR 0001 and docs/design/ws-reconnect.md) to files in the final tree, checked by the advisor; plan.md:1436
    changes to match at the next plan edit.
  - **NS-START (a):** the night shift runs hourly from 00:10 to 07:10 Amsterdam again (`CRON_TZ=Europe/Amsterdam 10 0-7
    * * *`). The session's `update_trigger` call was refused: the routine was created via http_api, not by an agent,
    so only Mustafa can edit it (https://claude.ai/code/routines/trig_01QpV6y6UaG1Na2q3jumRDxh). It is in his For
    Mustafa list.
- **Mustafa, 2026-11-10 about 06:40 UTC, for the netcode session (verbatim, "Record these as my words of 2026-11-10.
  None of them stops wave 1."):**
  - "1. RESUME-CARD. The checkpoint is 148 KB and the plan 144 KB, against CHECKPOINTS.md's "keep it short: it is a
    handoff, not a log". At the next natural stop, put a resume card at the top of .checkpoints/netcode-proof-test.md:
    at most 150 lines, with the current step, the next three actions, the open questions for me, and pointers. Move
    the "Steps done" and "Decisions taken" history into .checkpoints/netcode-proof-test/history.md. A resuming
    session reads the card first and opens the rest only when a step needs it."
  - "2. WIP-CLEAN. When a wip/*.patch is merged or replaced, delete it in the same commit. Keep one snapshot per live
    builder."
  - "3. TCP-FREEZE. The Linux TCP model is done. No more fidelity work on it. If U8's netem check on the VM disagrees
    with the model, netem wins: the WebSocket rows are then judged on VM runs with netem, and the model is not
    repaired. Tell me if any later unit plans work on the model."
  - "4. SQUASH-40. PR #40 merges by squash. Its history carries about 28 MB, most of it checkpoints. Write that into
    the closing checklist now. No .checkpoints/ in the final tree, as always."
  - "5. PROMOTE, a question to answer at U4's close and not before. engine-architect lists, on one page, which crates
    could move into the real engine as they are (my guess: npt-link, npt-l1, npt-proto, npt-room, npt-client), what
    each would need, and what must be rewritten. Plan §4.9 says the foundation writes its crates fresh. I want to
    decide that with numbers, because U4 builds the protocol, the room, prediction and lag compensation to a
    standard I do not want to pay for twice."
  - "6. SIZE-CHECK. U3 was sized S and came to about 20,000 lines. From now on, when a unit grows well past its plan,
    stop and tell me in three lines: what grew, why, and the cheaper option."
  - "7. A second lane starts on my personal account (the foundation's first steps). It does not touch spikes/ or your
    branch. After its PRs merge, merge main as usual."
  - "Reply with: which of 1 to 6 you can do without slowing wave 1, and anything here that conflicts with a ruling I
    already gave."
- **Mustafa, 2026-11-09 about 10:00 UTC, LEFTOVER-DELETE (verbatim):** "leave them. 14 GB free is enough, and a
  one-off delete past the deny list isn't a habit we start; if it's ever needed, it becomes a narrow kit rule
  instead." Applied: .gate/u3c-review-target (4.0 GB, no CACHEDIR.TAG) and the nine plain scratch copies stay; no
  delete outside the approved methods (`cargo clean --target-dir`, plain `git worktree remove`). If a delete of that
  kind is ever needed, it goes to Mustafa as a narrow kit-rule change (a `.claude/` edit, asked first), not as a
  one-off. Agent briefs keep critics' scratch work in forms the approved methods can remove (a cargo target dir, or a
  git worktree), never plain source copies.
- **Mustafa, 2026-11-09 about 01:10 UTC (verbatim):** "yes, by the approved method. And from U3d on, point all copies
  at one shared Cargo target folder (CARGO_TARGET_DIR), so each unit doesn't leave a 4–6 GB cache behind."
  - **How it is applied (the main session):** every agent copy (worktree or scratch) from U3d on builds with
    `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`, one folder for all copies, created by cargo
    on first use (so it carries cargo's CACHEDIR.TAG and `cargo clean --target-dir` can empty it later). The main
    folder keeps its own cache (.gate/netcode-proof-test/target, from the spike's .cargo/config.toml), so the merge
    checks never read a copy's artifacts. The G-METHOD safeguards stay, because cargo's hash ignores a copy's location
    and another copy's newer artifact can read as fresh: before each build a copy touches its own crate sources (mtime
    only) and checks each run's test count (`--list` where it matters); cargo's own lock serializes two copies'
    builds in the folder. Every workflow brief from U3d on names this folder and these two steps.
  - **The cleanup (2026-11-09 about 01:15 UTC):** 16 dirty copies' leftovers committed to local branches
    `leftover/<copy>` (never pushed), then all 17 copies removed with a plain `git worktree remove` (the 15 under
    .claude/worktrees, .gate/flaky-wt, .gate/npt-readings); `cargo clean --target-dir` emptied
    u3v3-harness-check-target (6.8 GiB), u3v3-model-target (5.0 GiB), netcode-proof-test/u3vf-target (203 MiB),
    c2b-review-scratch-target (66 MiB), c2b-review-mini-target, and the target folders inside the old probe projects
    netcode-proof-test/u0-wt and u3-wt-probe (303 + 412 MiB). Free disk 5.2 GB -> 17 GB. **Not removable by the approved
    method** (cargo 1.99.1 refuses a folder without its CACHEDIR.TAG, "to prevent accidental deletion of unrelated
    files"): .gate/u3c-review-target (4.0 GB; its tag is missing) and the plain scratch copies c1b-, c1c-, c2c-,
    c2d-review-scratch, u3c-fixrev-c1, u3c-review-f1 (+ .tar), u3bfix-base (+ .tar), u3bv-copy, u3bfix-copy (about 27 MB),
    and the probe projects' small sources: asked Mustafa whether to leave them or allow a plain delete of those paths.
- **Mustafa, 2026-11-08 about 09:58 UTC (verbatim):** "Yes to committing each dirty agent copy's leftovers to its own
  local branch and then removing the copy normally, for the two U3b copies now too. And if the reviewers confirm the
  lock-while-encrypting lead, fix it in U3c's fix pass if it's small: it may remove the timing tail we accepted."
  - **Done for the U3b copies:**
    - agent-a9099feea96fafb39's leftovers are on local branch u3bfix-a9099 (1550aa6);
    - agent-adf1deee6caf1481f's leftovers are on worktree-agent-adf1deee6caf1481f (85de721);
    - both copies were removed with plain `git worktree remove`; the disk has 12 GB free.
  - **The standing procedure for every finished copy:** commit its leftovers on its own local branch (never pushed),
    then `git worktree remove` without force. Each cleanup is asked once.
  - **Still to ask at U3c's cleanup:**
    - U3a's copies agent-a487e419ee806d37b and agent-a79d7290e0e6a6e50 (72e6fab; merged at 1e020a7);
    - .gate/flaky-wt, my throwaway measurement copy;
    - .gate/npt-readings, PR #45's copy (merged);
    - U3c's own copies and caches.
  - **The lock-while-encrypting lead** (rust-reviewer F3: Pump::poll holds the Conn mutex across S's polls): if the
    re-run review confirms it and the fix is small, it goes into U3c's fix pass, then the lateness run is repeated to
    see whether C10's tail goes.
- **Mustafa, 2026-11-08 about 00:10 UTC (verbatim):** "WS-RECONNECT-BAR: (b), open until the reconnect work's plan."
  - So the reconnect's own pass bar is proposed by the reconnect work with its plan, for his go, against the target;
    the borrowed WS-BAR form in docs/design/ws-reconnect.md §6.4 is only a starting proposal.
  - Recorded on the spike branch in docs/design/ws-reconnect.md (:51-52, §6.4, §7 item 5). It reaches main with the
    readings PR before R5 (or PR #40).
- **Mustafa, 2026-11-07 about 14:30 UTC, refining WS-BAR (verbatim):** "Refinement to my WS-BAR rule: the model's
  bound has almost no margin (5 % slower already fails a correct build), so declare a tolerance now, before any
  practice run. If the practice run is up to 10 % slower than the model, the numbers may move by that much; beyond
  10 %, investigate first and never loosen. The advisor may set a different tolerance with reasons, but it must be
  fixed before the run." So: the docs PR fixes the tolerance (10 % unless the advisor sets another with reasons) and
  how "slower than the model" is measured, before R3's practice run; within it the bar's numbers may move up by the
  measured factor; beyond it, investigate first and never loosen. The advisor's draft from workflow wf_f3167435-5b4
  predates this: fold it in with a follow-up advisor request.
- **Mustafa, 2026-11-07 about 13:45 UTC (verbatim):** "WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer)
  in the proof test. The reconnect becomes a requirement for the real client, built and measured before the first
  online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s). Record that in the
  docs PR so it can't get lost. RESUME-CRED: deferred with the reconnect. WS-BAR: the three-part form now; its
  numbers after one practice run on the real test setup, but never looser than the model predicts for a correct build
  with fix 1. If the practice run comes out worse than the model, find out why before setting the numbers."
  So: fix 1 = the adaptive newest-only budget (ceil(1.5 x srtt / tick) + 1 snapshots, at least 3, from TCP_INFO and
  SIOCOUTQ; in the proof test from DelayStream's model); fix 2 dropped; fix 3 (the reconnect) out of the proof test,
  a real-client requirement before the first online playtest, target = model v3's buffer + reconnect row (600,000
  phases at P2, 1 s burst: p99 1,051 ms; fixes.rs at 709e44a); RESUME-CRED and the reconnect's ADR 0001 changes wait
  for that work; PM-0001-18's WebSocket part gets a three-part bar (median, count over Y, max), numbers set after one
  unjudged practice run on L2, never looser than the model's fix-1 prediction; a practice run worse than the model is
  investigated first. The docs PR records the reconnect requirement and its target.
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

## Running work (older entries: U3c and before)
- **About 22:50 UTC: U3c CLOSED on the closing head 768d00ba7a77529b524fc9f6df90d6494f87abcc** (pushed). The advisor's
  U3C-CLOSE conditions: (1) C2d's critic complete, no gap, no regression (u3c-fix4-review-c2d.md; 10 of 12 bites, the 2
  missed are design details cargo itself refuses); (2) check.mjs 9/9 on the clean closing head (260 measured, 274
  bites, 0 failed, 0 skips; deps 78 crate files + 1 config; refs 472; marks 81; self 43/43; redact 149 items, 0
  findings) and GATE PASS --no-smoke (5a skipped: Chromium/Playwright mismatch; CI's smoke job runs it), the nine step
  lines and the SHA in the report; (3)+(4) the report fixes and the plan's U6/U8 rows (941832e); (5) this checkpoint
  pushed with the close; the cleanup ask goes to Mustafa (list below). Report: u3c-report.md (folded; the advisor's
  rulings in advisor-u3c-plan.md). C2d also fixed one gap its builder found: a quoted `[["include"]]` config header,
  which cargo 1.99.1 loads.
  - **Next: U3d, the timing self-test** (u3-build-plan.md:45: ImpairSocket's expected excess lateness beyond 0.25 ms at
    20,000 releases/s, inbound measured where quinn polls, p50/p99/p99.9/max, reported here, RS judges at R1; U3-Q9's
    facts; the advisor's M1, M13, M14; plus the advisor's 2026-11-08 additions: steal read before and after each
    self-test run, and ImpairSocket's up E over about 10 us in this container reported). PLAN FIRST
    (.checkpoints/netcode-proof-test/u3d-plan.md), then the advisor.
  - **About 22:50 UTC: U3d's planning workflow `wf_77078bc9-95f` (npt-u3d-plan) launched on 95dac9c**: scouts
    (netcode-architect: requirements and what exists, notes u3d-scout-req.md; test-engineer: how R1 runs and records
    the self-tests, u3d-scout-r1.md), then a netcode-architect drafter (u3d-plan.md rev 1), then two critics
    (u3d-plan-review-test.md, u3d-plan-review-rust.md). Read-only agents, notes in this folder (the saver commits
    them). If cut: resume with Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3d-plan-wf_77078bc9-95f.js,
    resumeFromRunId: "wf_77078bc9-95f", args: {head: "95dac9c"}}). Then: fold the critics' findings into the plan
    (rev 2), then the advisor's review; no code before its APPROVE.
  - **The cleanup ask went to Mustafa in chat (about 22:55 UTC)**; waiting for his answer (task: clean after his yes,
    by the approved method; the list is below; about 16 GB in all, 5.0 GB free).
  - **SAVE at Mustafa's 96 % usage (about 22:50 UTC).** U3d's planning workflow `wf_77078bc9-95f` was in its Scout
    phase: u3d-scout-req.md (plan only) and u3d-scout-r1.md (F1-F4 so far) on disk and committed; no draft yet. These
    agents work read-only in the main folder (no worktrees), so their notes are all on disk. **After the usage reset:**
    resume with Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3d-plan-wf_77078bc9-95f.js,
    resumeFromRunId: "wf_77078bc9-95f", args: {head: "95dac9c"}}) (finished agents replay from cache; a cut agent
    reruns: tell it to continue its notes file after the last line); if the run is gone, relaunch the same script.
    Then fold the critics into u3d-plan.md rev 2 and send it to the advisor. Also restart the saver
    (`node .claude/scripts/checkpoint.mjs watch netcode-proof-test --trailer "..."`), and check Mustafa's cleanup answer.
  - **2026-11-09 about 01:40 UTC: U3d's planning run wf_941d1cee-cfa DONE** (5 of 5). Scouts: U3d is mostly packaging
    (u3d-scout-req.md; u3d-scout-r1.md F1-F25, S1-S7). Draft: u3d-plan.md rev 1 (285 lines) + u3d-plan-notes.md: a
    test-only change (a new ignored run `l2_timing_self_test` at P1, 10,000/s each way for 100 s, a tokio-task poller
    with quinn's 32-buffer batch, the fate log for exact E forms and -3's own L2 P1 checks; four fast tests with bites;
    a dry-run script with U3-Q9's four kernel facts and steal; plan text), questions U3D-BAR, U3D-L0, U3D-POLL, R1
    judging, a miss's route, -3's checks. Critics: test-engineer REVISE (5 medium: T1 race, T2 near-vacuous, -3 delay
    formula unbitten, no producer for the L0 = 0 form at R1, judged window not tied to -3's; low/nits F6-F12;
    u3d-plan-review-test.md); rust-reviewer APPROVE WITH FINDINGS (F1 medium: spawn the poller onto a multi-thread
    runtime, not block_on; F2 medium: pre-touch the fate log; F3-F10 low/nits; u3d-plan-review-rust.md).
  - **About 02:25 UTC: U3d's plan APPROVED WITH CONDITIONS by the advisor** (advisor-u3d.md, verbatim; the approved text
    pinned at b2e0980, u3d-plan.md sha256 3650755d...). Rulings: U3D-BAR (a) E ≤ 10,000 ns per direction, inclusive;
    U3D-L0 (b) judged with L0 = 0 for ImpairSocket (M1's median form reported); U3D-WINDOW (W1); U3D-POLL (b) without
    the control round; U3D-JUDGE (J3) the advisor reads it; U3D-MISS (a); U3D-XCHECK (a). Nothing for Mustafa.
    Conditions 1-4 applied in u3d-plan.md by the main session (the poller defined inside `start_poller`; the judged
    form named in the SERIES.md and U8 row texts; C-U8-5; C-U6-6 never INVALID by design), each marked; the plan
    text of §3.6 made in plan.md (§12.1 U3 row's U3d sentence, U6 row's C-U6-1..7, U8 row's ImpairSocket self-test
    with C-U8-1..5) and §15's new reading R-31. Condition 6's nits go to the builder's brief.
  - **About 02:25 UTC: U3d's build launched, workflow `wf_2c76ec82-8fc` (npt-u3d-build) on dae4f0f:** build:u3d
    (general-purpose, xhigh, worktree from isolation, CARGO_TARGET_DIR=.gate/shared-target, notes u3d-build.md in the
    worktree), then review:netcode and review:rust read-only in parallel (u3d-build-review-netcode.md,
    u3d-build-review-rust.md), then prove:test (the only one that builds and plants bites; u3d-build-review-test.md).
    **If cut:** snapshot the builder's worktree with worktree_patch.py (wip/u3d-build.patch), then Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3d-build-wf_2c76ec82-8fc.js,
    resumeFromRunId: "wf_2c76ec82-8fc", args: {sha: "dae4f0f396d655d194e4300e8d56f07b619be908"}}).
    **After it:** fold findings (a fix pass if needed), merge (worktree_patch.py, git apply), check.mjs 9/9 and the gate
    on the main folder's own cache, commit, push; then the dry run (results/u3d-timing/run.sh) on a quiet machine,
    reported, not judged; then U3d's report to the advisor.
  - **About 03:30 UTC: the build DONE** (worktree .claude/worktrees/wf_2c76ec82-8fc-1, uncommitted on dae4f0f;
    socket_tests.rs about 990 lines appended, M13 untouched by sha256; SERIES.md :427/:452 + one sentence; run.sh new;
    check.mjs 9/9 twice: 267 measured / 281 bites; 22 of 22 bites). Reviews: netcode-architect, rust-reviewer and
    test-engineer all APPROVE WITH FINDINGS, no regression. Findings: LOW (all three) the slack run condition demands
    0 under a real-time policy, so the recorded `chrt -f -p` switch always makes the run INVALID (shown by a run);
    LOW the loss band checked before log completeness (wrong cause on bites S4, S5); LOW the pre-touch not asserted;
    nits (non-UTF-8 env, T6's agree branches and loss denominator, samples() panic before prints, run.sh check-mode
    touch, SERIES.md "Its", FATE_BYTES import). INFO for the report: the CPU share's dilution against M13's 0.27.
  - **About 03:55 UTC: U3d MERGED at 1ccff06** (pushed): the fix pass complete per the test-engineer (u3d-fix-review-test.md;
    16 of 16 bites; the slack condition now accepts 0 or 1 under SCHED_FIFO/RR at both reads, since a process born
    under `chrt -f 1` reads 0 at start; the plan allows it). check.mjs 9/9 in the main folder (267 measured, 281 bites,
    self 43, redact 150 items, 0 findings); gate PASS --no-smoke. run.sh read in full by the main session before
    running it (reads /proc and /sys, builds once, runs that binary, writes only its own new record file).
  - **About 04:02 UTC: the dry run DONE** (record committed at 78b0c07; four in-progress commits 41f9cd1, 133dc8d,
    d6186bf, 4f37623 during it, at the stop hook's demand: their times are in the report). SELFTEST 1 and 2 valid:
    E (L0 = 0) up 17,067 / 19,728 ns, down 11,540 / 12,450 ns, all would MISS; -3's p99.9 would FAIL in all four, the
    12 agree lines all "agree"; every other -3 L2 P1 check would pass. SELFTEST 3 INVALID: 87 up datagrams never reached
    ImpairSocket = the kernel's Udp RcvbufErrors 87 since boot, in a ~34 ms stall of both directions. M13 rounds: E up
    40.3 / 52.8 / 4.27 us, down 45.7 / 55.9 / 3.35 us. Report: u3d-report.md (to the advisor: U3D-CLOSE, U3D-RCVBUF).
  - **About 03:56 UTC: the dry run RUNNING** (background, `bash spikes/netcode-proof/results/u3d-timing/run.sh`, quiet
    machine, CARGO_TARGET_DIR unset, about 6-7 min; record results/u3d-timing/selftest-<UTC>.txt). Then: commit it, write
    U3d's report (u3d-report.md: reported, not judged; the CPU-share dilution note against M13's 0.27), send it to the
    advisor for the U3d close. If cut: rerun the script (it never overwrites a record).
  - **Done: the fix pass, workflow `wf_5c8d15c1-1f7` (npt-u3d-fix):** fix:u3d in the same worktree
    (no isolation), then check:u3d-fix (test-engineer, the changed lines; notes u3d-fix-review-test.md). If cut:
    snapshot the worktree (worktree_patch.py ... wip/u3d-build.patch u3d-build), then Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3d-fix-wf_5c8d15c1-1f7.js,
    resumeFromRunId: "wf_5c8d15c1-1f7"}).
  - **Done (launched about 01:38 UTC): workflow `wf_bbda74db-c79` (npt-u3d-plan-rev2) on dacdb77:** fold both critiques
    + Mustafa's shared-target rule into rev 2 (netcode-architect; notes u3d-plan-rev2-notes.md), a completeness check
    (test-engineer; u3d-plan-rev2-check.md; one fix pass if gaps), then the advisor's review of rev 2 (it returns text;
    the main session writes it to advisor-u3d.md). If cut: Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3d-plan-rev2-wf_bbda74db-c79.js,
    resumeFromRunId: "wf_bbda74db-c79", args: {head: "dacdb77"}}). No code before the advisor's APPROVE.
  - **2026-11-09 about 01:05 UTC (after the reset; Mustafa: "continue"):** wf_77078bc9-95f had ended with all 5 agents
    failed on the spend limit (none completed, so nothing to replay). The script now tells each agent to continue its
    notes file if it exists; relaunched as **`wf_941d1cee-cfa`** (same script path, args {head: "95dac9c"}); the saver
    restarted. If cut again: Workflow({scriptPath: the same script, resumeFromRunId: "wf_941d1cee-cfa", args: {head:
    "95dac9c"}}). Mustafa has not answered the cleanup ask yet.
- **About 22:10 UTC: the condition-10 diagnostic DONE and committed (ecdeeb0)**:
  results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt (identical to the raw output; redact 0). Split violations 0.
  Clock part p99.9 1.6-2.3 ms, max 3.1-6.8 ms; hop p99.9 1.5-2.0 ms, max 2.7-3.8 ms; work down max 3.1-6.5 ms, up (the
  pass's rest + the reader task's turn) max 8.4 / 22.2 ms and most of up's E (16.6-18.6 of 26.8-35.4 us); ImpairSocket
  in the same window p99.9 1.25-4.24 ms, max 3.1-8.2 ms. Totals: down E 11.5 / 16.6 us, up E 26.8 / 35.4 us.
  - **The advisor ruled (about 22:17; appended to advisor-u3c-plan.md, "Side ruling: the condition-10 diagnostic,
    UP-STAMP and U3c's close"):** C10-B APPROVE ((b) not built before R1; fallback order at R1: split to the advisor
    first, a miss in up's pass part -> wake the reader right after the releasing step and yield, (b) only for a miss in
    the hop). UP-STAMP APPROVE with changes (stamp R at the reader's wake, delay.rs:1884; built in U6 with an
    early-reader class, identity test and bite; bar stays on L; steal recorded at R1). Correction: up's work = the
    pump's pass + the reader's turn (not shown to be the reader's turn). U3C-CLOSE APPROVE on five conditions: (1)
    C2d's critic complete, no regression (a new finding >= low goes to the advisor); (2) check.mjs 9/9 + gate on the
    closing head, the nine step lines + head SHA in the report; (3) report fixes (a)-(d): DONE (941832e); (4) plan U6
    and U8 rows: DONE in the same commit (941832e); (5) checkpoint pushed with the close; the critics' scratch copies in
    Mustafa's cleanup ask.
  - **Cleanup list (for the ask at the close; sizes at 22:20):** caches .gate/u3v3-harness-check-target 6.2 GB,
    .gate/u3v3-model-target 4.5 GB, .gate/u3c-review-target 3.8 GB+ (after C2d), .gate/netcode-proof-test/u3vf-target
    198 MB, u0-wt 294 MB, u3-wt-probe 397 MB, .gate/c2b-review-scratch-target 56 MB, c2b-review-mini-target; scratch
    copies .gate/c1b-review-scratch, c1c-review-scratch, c2c-review-scratch, c2d-review-scratch (after), u3c-fixrev-c1,
    u3c-review-f1 (+ .tar), u3bfix-base (+ .tar), u3bv-copy, u3bfix-copy; worktrees .gate/flaky-wt, .gate/npt-readings,
    .claude/worktrees/{agent-a487e419ee806d37b, agent-a79d7290e0e6a6e50, agent-a3dfcb36e3c15ab56,
    agent-a7fafe27dcdee96cd, agent-a3d04e3f48e4d97db, wf_87550c07-bbd-1, -2, wf_3ea724fc-a63-1, -2, wf_ac5e4575-9da-1,
    -2, wf_10e244da-89a-1, wf_5c9f078a-da1-1, -2, wf_b13b9984-6bc-1}. Keep: .gate/netcode-proof-test/target (the main
    folder's cache) and runs/ (U2's raw run logs). Method as approved: caches by `cargo clean --target-dir`; dirty
    copies' leftovers committed to a local branch, then `git worktree unlock` (workflow copies are locked) and a plain
    `git worktree remove`.
- **About 22:11 UTC: C2d launched, workflow `wf_b13b9984-6bc` (npt-u3c-fix4) on ecdeeb0**: fix:C2d (N1-N5; notes
  u3c-fix4-c2d.md in its worktree; cache .gate/u3c-review-target) then critic:C2d (test-engineer; notes
  u3c-fix4-review-c2d.md; deliberate-only ways around go to known limits). If cut: snapshot its worktree with
  worktree_patch.py (wip/u3c-fix4-wt1.patch), then resume with Workflow({scriptPath:
  /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3c-fix4-wf_b13b9984-6bc.js,
  resumeFromRunId: "wf_b13b9984-6bc", args: {sha: "ecdeeb0beee0994cdb5644830d813cdca0c3af5c"}}).
- **About 21:50 UTC: the final round MERGED at f6cd1d7** (C1c + C2c; check.mjs 9/9: 260 measured, 274 bites, self 41,
  redact 0; gate PASS --no-smoke; pushed). C1c's critic: COMPLETE (APPROVE), every bite caught. C2c's critic: no
  regression; five small gaps:
  - N2 (low): inheritsWorkspaceLints (check.mjs ~:421-430) reads raw lines (strings unmasked): `[lints] workspace =
    true` moved into a multi-line string passes deps while cargo drops the workspace lints for that crate; two claims
    then false. Fix: read it through tomlScan(text).mask; a test with the plant.
  - N1 (nit): deleting depsStep's member-manifest loop (~:1146) passes every self test; add a member plant.
  - N3 (nit): `__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS=nightly` lets stable cargo accept cargo-features; not
    refused nor named (opens nothing today: every key it unlocks is refused).
  - N4 (nit): a UTF-8 BOM hides line 1's key from tomlKeyAt (only a new tool's lint table gets in, e.g. rustdoc).
  - N5 (nit, comment): unacked_and_unsent's new SAFETY line omits non-inet families that answer 0x5411 (AF_UNIX,
    AF_PACKET: one int by TIOCOUTQ's convention).
  - **Plan:** one tiny round C2d for N1-N5 (a builder + a test-engineer critic on the changed lines only) AFTER the
    condition-10 diagnostic (the machine must stay quiet).
- **About 21:53 UTC: the condition-10 diagnostic RUNNING** (background task bkp8uz21c; output
  /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/bkp8uz21c.output): DELAY 1,
  IMPAIR 1, DELAY 2, IMPAIR 2 on f6cd1d7's release build (.gate/netcode-proof-test/target). Then write each part to
  results/u3c-lateness/c10-diag-*.txt with the Write tool, commit, fill the report. If cut: rerun the same loop (the
  build is cached), quiet machine.
- **About 21:00 UTC: the FINAL small fix round launched, workflow `wf_5c9f078a-da1` (npt-u3c-fix3), on c37dfe1.**
  - fix:C1c (npt-net; worktree from isolation, notes u3c-fix3-c1c.md in it; cache .gate/u3v3-harness-check-target)
    then critic:C1c (rust-reviewer; notes u3c-fix3-review-c1c.md in the main folder).
  - fix:C2c (check.mjs and guards; notes u3c-fix3-c2c.md in its worktree; cache .gate/u3c-review-target) then
    critic:C2c (test-engineer; notes u3c-fix3-review-c2c.md).
  - **If cut:** snapshot each builder's worktree first (`python3 -I .checkpoints/netcode-proof-test/worktree_patch.py
    <worktree> .checkpoints/netcode-proof-test/wip/u3c-fix3-<part>.patch u3c-fix3-<part>`), then resume with
    Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3c-fix3-wf_5c9f078a-da1.js,
    resumeFromRunId: "wf_5c9f078a-da1", args: {sha: "c37dfe14f222d62f429785498e189262ca7a8f7b"}}), or a fresh agent
    applies the snapshot at c37dfe1 and continues after its notes' last Progress line.
  - **After it:** apply both (worktree_patch.py, `git apply --check`, `git apply`), check.mjs 9/9 and the gate,
    commit, push; then the condition-10 diagnostic (quiet machine, no agents building), U3c's report, the cleanup ask,
    U3d.
- **About 20:55 UTC: C1b MERGED at f01d573** (applied from .claude/worktrees/wf_10e244da-89a-1 via worktree_patch.py;
  check.mjs 9/9: 258 measured, 272 bites, self 35, redact 0; gate PASS --no-smoke).
  - R1 fixed; G-PRE-WAKE, G-PRE-PANIC, G-F10, G-NC4, G-FH, G-NC8, G-NC2; the split pooled per process (309,583 B per
    connection + 1,350,400 B per process). Notes u3c-fix2-c1b-finish.md.
  - **Both critics DONE** (workflow wf_10e244da-89a): rust-reviewer on C1b (u3c-fix2-review-c1b.md: code APPROVE,
    REVISE for small tests); test-engineer on C2b (u3c-fix2-review-c2b.md: REVISE small, 9/9 reproduced, 32 of 33
    revert bites caught).
  - **Their remaining gaps (the final small fix round, C1c and C2c):**
    - C1c (npt-net): (1) MEDIUM: R1's no-spin guard (`!s_owes_write`) is unpinned: with it removed, a_full_budget's
      1.5 s hold makes ~170,000 pump polls (fixed: ~120) and still passes; bound life().polls there. (2) LOW:
      add_burst's G-PRE-WAKE wake (delay.rs ~:1132) unpinned. (3) LOW: G-PRE-PANIC's `thread::panicking()` condition
      unpinned the other way (a normal pump end turned into an error passes every test). (4) INFO: poll_shutdown
      returns Ok(()) after a panicked pump (pre-existing; the writer-side twin of the clean-EOF lie).
    - C2c (check.mjs, tcpinfo, the Python test): F1 lintTables skips `[lints.<tool>.<lint>]` sub-tables and hyphen
      lint names (a hyphen sub-table relaxes the exceptions' clippy deny and tcpinfo's deny; the forbid backstop
      holds); F2 test_siocoutq_root_check.py's git() inherits GIT_DIR and can commit into another repository; F3 the
      CARGO_HOME parameter of depsStep is untested; F4 the MPTCP test's real stderr wiring is untested; F5
      RUSTC_BOOTSTRAP plus a manifest's `cargo-features = ["profile-rustflags"]` and profile `rustflags` cap the lints
      unseen; F6 the BPF-hook line in linux.rs's SOL_SOCKET SAFETY comments (C2R-9 (2)).
    - Done by the main session (F6's text part): plan §4.2 (forbid, own tables, four unsafe blocks), §4.8 (four
      blocks), u3c-plan.md 1.8 (a) (superseded by the ruling); plan §12.1 U6 gets the pool carry (INFO 5).
  - **Working rule (G-METHOD):** each build copy uses its own CARGO_TARGET_DIR, or touches its tree's sources before a
    run and checks each run's test count (a shared target dir once ran another copy's build).
- **About 20:15 UTC: C2b MERGED at b5e4043** (the forbid backstop and check.mjs's scope).
  - The main session read the diff, since the classifier was down during that agent's run: no network or process use,
    and writes only to temporary test folders. Its Python test only imports provenance() into a temp git repo, never
    runs the script.
  - check.mjs 9/9 (252 measured, 266 bites, self 35); gate PASS.
- **Workflow `wf_10e244da-89a`** (npt-u3c-fix2b), launched about 20:15 on b5e4043.
  - C1b's continuation in a fresh worktree applies wip/u3c-fix2-wt1.patch and finishes from its plan (notes
    u3c-fix2-c1b-finish.md in the worktree); then the rust-reviewer critic on C1b.
  - In parallel, the test-engineer critic on C2b's merged diff (notes u3c-fix2-review-c2b.md).
  - If cut: resume with resumeFromRunId "wf_10e244da-89a" and args {sha: b5e40438...}.
- **Earlier (about 17:15 UTC; the spend limit reset 20:00 UTC): the last fix round ENDED PARTWAY.**
  - **C2b DONE**, worktree .claude/worktrees/wf_ac5e4575-9da-2, snapshot wip/u3c-fix2-wt2.patch, notes
    u3c-fix2-c2b.md.
    - The forbid backstop LANDED: the workspace unsafe_code is "forbid"; tcpinfo has its own lint table with "deny" and
      client-wasm its own with "allow"; check.mjs's exceptionLintProblems compares the tables.
    - Also: C2R-3 closed (lintTables), C2R-5 depsStep self-test, C2R-6 honest comments, the C2R-8, C2R-4 and C2R-7
      refusals, C2R-9 visible skips, C2R-10 with Python tests, REG-3's cost doc.
    - check.mjs 9/9 twice (252 measured, 266 bites, self 35); 22 of 22 mutation bites caught.
    - **The safety classifier was UNAVAILABLE during C2b's run: read its whole diff before merging** (Claude Code's
      note).
    - Its critic failed on the spend limit. Not fixed, as known limits: C2R-4 G/H ([env] CLIPPY_ARGS, build-script
      rustc-env), a rust-toolchain.toml path override, and tcpinfo's deny level.
  - **C1b FAILED on the spend limit**, worktree .claude/worktrees/wf_ac5e4575-9da-1, snapshot wip/u3c-fix2-wt1.patch,
    notes u3c-fix2-c1b.md: partial. After 20:00 UTC resume with resumeFromRunId "wf_ac5e4575-9da" and args {sha:
    "b2e879b"}; C2b replays from cache. Or launch a fresh C1b agent that applies wip/u3c-fix2-wt1.patch at b2e879b and
    continues from its notes.
  - **Then:** merge C2b after reading its diff, then C1b; critics on both diffs; check.mjs 9/9; the condition-10
    diagnostic; U3c's report; the cleanup ask; U3d.
- **Earlier (Mustafa at 95 % usage, about 16:30 UTC): U3c's LAST FIX ROUND launched**, workflow
  `wf_ac5e4575-9da` (npt-u3c-fix2), launched 16:26 on b2e879b.
  - **C1b** (npt-net): R1, G-PRE-WAKE, G-PRE-PANIC, G-F10, G-NC4, G-FH, G-NC8, G-NC2, and the split's memory moved to
    shared run-level histograms.
  - **C2b** (check.mjs): (a) the self-test and honest comments; (b) the RUSTC, include and runner refusals plus the
    forbid backstop, kept only if 9/9; (c) the nits; (d) the known limits.
  - The critics: rust-reviewer on C1b, test-engineer on C2b. Rulings: "Side rulings: U3c's last fix round" in
    advisor-u3c-plan.md.
  - **The agents' work-in-progress is snapshotted** (base b2e879b; worktrees .claude/worktrees/wf_ac5e4575-9da-1 and
    -2):
    - wip/u3c-fix2-wt1.patch and wip/u3c-fix2-wt2.patch;
    - notes u3c-fix2-*.md, copied by worktree_patch.py.
  - **If cut:** resume with Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3c-fix2-wf_ac5e4575-9da.js,
    resumeFromRunId: "wf_ac5e4575-9da", args: {sha: "b2e879b"}}). A fresh agent first applies its part's wip patch at
    b2e879b (`git apply --exclude='.checkpoints/*'`) and continues after its notes' last Progress line.
  - **After it:**
    - merge C2b and C1b;
    - check.mjs 9/9 and the gate;
    - the advisor's condition-10 diagnostic: the DelayStream evidence run alternating with ImpairSocket's M13
      (commands below, under "THE SPLIT IS BUILT"), reported, not judged;
    - U3c's report section;
    - the cleanup ask (U3a copies, flaky-wt, npt-readings, U3c copies and caches);
    - then U3d.
  - **Done since the last block:**
    - split merged at b5c9009 (check 9/9);
    - re-review record u3c-fix-review-result.txt;
    - plan text NC-U3c-1/2/3 corrected in u3c-plan.md;
    - U6 carries added to plan §12.1 (6538a5d).
- **U3c FIX PASS MERGED at 81203da** (about 15:08 UTC 2026-11-08).
  - The fix workflow wf_3ea724fc-a63 hit the spend limit: C2 had finished and its critic was cut; C1 had finished every
    fix and passed check.mjs twice, but its final report was lost. Both were snapshotted (wip/u3c-fix-c1-partial.patch,
    wip/u3c-fix-c2.patch, notes u3c-fix-c1.md and u3c-fix-c1/, u3c-fix-c2.md), then applied.
  - check.mjs 9/9 (250 measured, 264 bites, self 30); gate PASS.
  - **The quiet lateness rerun came out WORSE** (committed 2351770).
    - The VM was 7 min after a restart, load average 1.47 -> 0.04.
    - Down p50 0.155, p99 0.628, p99.9 3.483, max 11.281 ms, E 16.4 us (run 1: 0.112 / 0.345 / 1.908 / 4.200, E 3.5).
    - Up 0.206 / 1.398 / 2.906 / 10.641, E 25.9 us (run 1: 0.161 / 0.509 / 2.111 / 2.839, E 6.5).
    - On these numbers R1's bar (E < about 10 us, p99 <= 1 ms) would fail.
  - **A/B DONE (e317067): ENVIRONMENT, NOT CODE.**
    - pre: down E 18.1 and 11.6 us, p99 0.61 and 0.67 ms; up E 28.2 and 26.7 us, p99 1.41 and 1.52 ms.
    - post: down E 16.2 and 22.1 us, p99 0.71 and 0.89 ms; up E 27.9 and 38.2 us, p99 1.37 and 1.88 ms.
    - No consistent difference. The same pre-fix code gave E 3.5 and 6.5 us at 00:41, so the container got noisier
      after the 15:01 restart; F3 neither caused nor removed the tail. Steal while idle 0.02 % (settles nothing).
    - **The advisor (2026-11-08; "Side ruling: condition 10 again"): (a) STANDS, plus one condition before U3c's report.**
      - Build the per-release split now: clock = wake - model stamp; hop = pump start - wake; work = S write or hand-over
        - pump start; an identity test that the parts sum to the total.
      - One quiet diagnostic, interleaved with ImpairSocket's M13 lateness run, reported and not judged.
      - No plain rerun. The split's diff is reviewed after the fix-diff re-review, changed lines only.
      - On RS a max over 10 ms goes to the advisor with the result.
    - **THE SPLIT IS BUILT** (.claude/worktrees/agent-a3d04e3f48e4d97db, base f8c3812, uncommitted; snapshot
      wip/u3c-split.patch; notes u3c-split.md). check.mjs 9/9 twice (251 measured, 265 bites).
      - **Classes:** woken: clock = W - M, hop = P - W, work = (3) - P. Other: wait = T - M, work = (3) - T, by step
        (open, mid or app).
      - **Identity test:**
        `l2::delay::tests::the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it`; its bites are
        DropHop and WorkFromWake. loopback() asserts the identity too.
      - **The evidence run** prints each part's statistics and the 5 largest releases split into their parts.
      - **Its findings:**
        - **(1) A likely cause of the rare ~35 ms maxima:** a down record released in the pump's up-loop step
          (delay.rs:1686) is not written in that pass, and the closing reschedule (:1747) can move its clock entry, so
          it waits for the next wake (up to about one tick). The diagnostic confirms it if down "mid" > 0 with large
          work. Then fix it (small; a release that is late by our own logic).
        - **(2) Memory:** ws_conn_fixed_bytes 309,583 -> 1,731,983 B per connection (10 more histograms): 83 MB at 48
          connections, and 1.7 GB for -25's 1,000 never-joining sessions if each gets a DelayStream. To review:
          allocate the split lazily, use run-level histograms, or a coarser one.
        - (3) A race in its own change was fixed (cancel clears the fire).
      - **Diagnostic commands** (from spikes/netcode-proof, after the merge, quiet machine, alternating twice):
        - `NPT_WS_LATENESS_TICKS=10050 cargo +1.99.1 test --release --locked -p npt-net --lib
          l2::delay::tests::lateness_evidence_run -- --ignored --nocapture --exact`;
        - `NPT_L2_SECS=10 cargo +1.99.1 test --release --locked -p npt-net --lib
          l2::socket::tests::l2_sustains_10000_per_second_each_way -- --ignored --nocapture --exact`.
      - **Merge it after the fix-diff re-review** (wf_7360a111-ea8) finishes, so the reviewers read a stable tree.
  - **The fix-diff re-review, workflow `wf_7360a111-ea8`** (npt-u3c-fix-review), on 95597ef..81203da: rust-reviewer
    on C1 (the F3 lock change most carefully) and security-auditor on C2 (try to bypass the new rules), each followed
    by an adversarial verifier. Notes: u3c-fix-review-{c1,c2}.md. If cut, resume with resumeFromRunId
    "wf_7360a111-ea8".
  - **Earlier, the interleaved A/B setup** (task birwiy3e6, about 22 min): pre-fix against post-fix release binaries,
    alternating pre, post, pre, post, each 10,050 ticks, on a quiet machine.
    - pre: .gate/u3v3-harness-check-target/release/deps/npt_net-5596387307f68428, from
      .claude/worktrees/agent-a7fafe27dcdee96cd (DelayStream sources equal 95597ef);
    - post: spikes/netcode-proof/target/release/deps/npt_net-f72f21a42e60b4c9 (81203da);
    - outputs .gate/u3c-evidence/ab-{1..4}-{pre,post}.txt.
    - If post is consistently worse, the fix pass regressed lateness: find the cause (F3's unlock/relock, F7's
      histogram, the fresh-now intake, the 527 B reads) before going on; it goes to the advisor with the numbers. If not,
      it was VM noise: record it, and the advisor rules.
  - **Then:** the re-review of only the fix diff (95597ef..81203da), since neither critic's verdict survived; the U6
    carries from C1's and C2's notes; the plan 1.4 (c) wording; U3c's close; cleanup per Mustafa's rule, asking once.
- **U3c REVIEW DONE (all 8 agents; record in u3c-review-result.txt and the journal of wf_8f2da9fc-8fa).** Verdicts:
  rust, netcode and security APPROVE_WITH_FINDINGS; test REVISE.
  - Verified medium: F1 (add_burst overflow) and F-K (test (c)'s hidden wall-clock limit).
  - Verified low:
    - NC-U3c-1 (test (c) uses N at the return; the gate guarantees only N at the last write);
    - NC-2, NC-3, NC-6 (the cross-check's check (2) has little power: the report says so);
    - F7 (565 KB per connection);
    - S1, F-D, F-E (unsafe-check bypasses), S2 (request scanner against tungstenite on the head's end), S7;
    - F-B, F-A, F-G, F-M (untested claims), F-L (gate inputs from four locks), F-N (the off-Linux build unchecked).
  - Nits: F3 (lock across S's I/O: REAL, rated nit; fixed anyway per Mustafa), F2, F4-F6, F8, F10, NC-4/5/7-12,
    S3-S6, F-C, F-F, F-H, F-I, F-J, F-O, F-P, F-R2.
- **The U3c fix pass, workflow `wf_3ea724fc-a63`** (npt-u3c-fix), launched about 10:51 UTC on 95597ef.
  - C1 (general-purpose, xhigh, worktree): DelayStream, ws_gate, tests and SERIES; CARGO_TARGET_DIR
    u3v3-harness-check-target.
  - C2 (the same): check.mjs and npt-tcpinfo; CARGO_TARGET_DIR u3v3-model-target.
  - Each is followed by a test-engineer critic. Notes in each worktree: u3c-fix-{c1,c2}.md.
  - If cut: resume with resumeFromRunId "wf_3ea724fc-a63" and args {sha: 95597ef...}.
  - **After it:**
    - merge C2, then C1;
    - check.mjs 9/9 and the gate;
    - re-run the quiet lateness evidence (does F3's fix remove C10's tail?);
    - re-review only what changed;
    - the plan 1.4 (c) wording (N at the last write) and the U6 carries (S5, S7's burst-length bound);
    - close U3c;
    - cleanup per Mustafa's rule, asking once.
- **DISK FULL, then freed** (about 01:30 UTC 2026-11-08: 132 KB free). The saver b7zmsy7v8/brque24xu died on it, and the
  review workflow's three verifiers failed on the spend limit at the same time.
  - **Mustafa's standing rule (verbatim):** "From now on, after each unit merges: delete that unit's build cache under
    .gate/ and remove its finished agent copies (check each is clean first, then use git worktree remove), so the disk
    never fills again. Ask me once per cleanup until the kit allows it."
  - **How the kit lets it happen:** `Bash(rm -rf *)` and `git worktree remove --force/-f` are in the kit's DENY list
    (.claude/settings.json:214-216, :247), so they are refused before any prompt.
    - Caches: delete them with `cargo +1.99.1 clean --manifest-path spikes/netcode-proof/Cargo.toml --target-dir <dir>`.
      That worked.
    - Agent copies: plain `git worktree remove`, only when clean.
  - **Done with his OK (AskUserQuestion, "Caches + old copies"):** 15 stale .gate caches removed, about 7 GB; 6.3 GB
    free afterwards.
  - **Not removed (not clean):**
    - .claude/worktrees/agent-a9099feea96fafb39: 975e72b, 13 changed/untracked files, U3b's fix pass, merged at 7321cc0;
    - .claude/worktrees/agent-adf1deee6caf1481f: a0d7916, 6 files, U3b's build, merged at 5a5aba9.
    - Proposal for his next OK: commit each copy's leftovers on its own local branch (nothing lost, nothing pushed),
      then `git worktree remove`.
  - **Still kept for U3c's fix pass:** .gate/u3v3-harness-check-target (A), u3v3-model-target (B) and u3c-review-target.
    After U3c closes: clean them, and the U3c worktrees (wf_87550c07-bbd-1/-2, agent-a3dfcb36e3c15ab56,
    agent-a7fafe27dcdee96cd), per his rule, asking once.
- **The U3c review round is RE-RUNNING** (workflow wf_8f2da9fc-8fa resumed about 09:54 UTC as task wxvbx8lyq). The
  first run finished while the disk was full, so its journal results and output file (0 B) were lost; only the
  completion notice survived. The re-run's notes are u3c-review-{rust,netcode,security,test}.md.
  - **The rust-reviewer's first-run findings, from the notice, for comparison:**
    - F1 (medium): DelayStream::add_burst lets start+len overflow, so due()'s `x + origin` wraps, giving a release-build
      spin or a debug panic. Not reachable from peers today; fix with checked_add and InvalidInput, plus saturating
      adds in due() and record_return.
    - F7 (low): current() and add_burst can consume the ACK event that reopens the pump's up read gate without waking
      the pump. Fix: wake on any dispatched event.
    - F3 (low): Pump::poll holds the Conn mutex across S's poll_write/read/flush/shutdown, so rustls and syscalls run
      under the lock. A lead for C10's tail. Fix: copy out, unlock, poll S, relock to commit.
    - F2 (low): current() wakes under the lock.
    - F9 (low): a pump panic is swallowed as a clean EOF. Set failed in Pump::drop when panicking.
    - F6 (low): header_numbers blocks up to 1 s on the clock's Condvar. Document it, or have the clock publish slack.
    - F8 (low): the gate's four inputs come from four lock acquisitions. Add gate_inputs() under one current().
    - F4 and F5 (nits).
    - The rust verifier had confirmed some of these before the notice; its verdict was lost.
- **U3c EVIDENCE RUNS DONE** (about 00:55 UTC 2026-11-08; committed 1a9fd73 and 989118e):
  - **Kernel cross-check PASS (final)**, out-20261108T004723Z.
    - RACK p50: kernel 33.344 ms (166 episodes), model 33.458 ms (5,672).
    - Probe/timeout share: kernel 6 of 172 against the model's 0.0416, band 1-15.
    - Noise guard held (60/61, 54/56, 58/61); one-gap count 100 against 172; iptables unchanged.
  - **C12e root run PASS**: R1-R4; SIOCOUTQ 200 k exactly with notsent 0; release in 435 ms, within 1.632 s.
  - **C10 quiet lateness: THE TRIGGER FIRES.**
    - down p99.9 1.908 ms, max 4.200 ms, E 3.5 us; up p99.9 2.111 ms, max 2.839 ms, E 6.5 us; n 10,051 each way.
    - Load average 0.94 -> 0.00. U3-Q1's E (< about 10 us) is met.
    - **The advisor's ruling (2026-11-08; "Side ruling: condition 10" in advisor-u3c-plan.md): (a) ACCEPT; RS's run at
      R1 decides against a stated bar.**
    - Why:
      - the tail is about 100 times below what any WebSocket mark resolves (-18's WS median about 654 ms on a 10 ms
        grid with 10 % tolerance; -31's 250 ms p99; lateness p99 0.35-0.5 ms);
      - lateness feeds back into nothing the model decides (the replay identity);
      - U3-Q1's E is met;
      - ImpairSocket shows similar tails in this container;
      - (b) would move S's TLS onto another runtime for no mark's gain.
    - **Recorded for U3c's report.**
    - **Carried to R1's run plan:** DelayStream's self-test on RS.
      - Under the WebSocket part's real load (48 connections at P2 with the room running), at least 10,000 samples per
        direction.
      - JUDGED per direction: E < about 10 us and p99 <= 1 ms. REPORTED: p50, p99.9 and max.
      - Each release split into the clock thread's wake, the pump's start and S's poll_write return. **Add those stamps
        before R1 (U6 or U8).**
      - A miss goes to the advisor before -18's and -31's WebSocket parts count; then (b) is the fallback: the pumps on
        a dedicated thread with 1 ns slack, re-measured.
- **The skeptic round, workflow `wf_8f2da9fc-8fa`** (npt-u3c-review): rust-reviewer, netcode-architect,
  security-auditor and test-engineer on `git diff 2c68bf9..989118e -- spikes/`, each followed by an adversarial
  verifier.
  - Notes: u3c-review-{rust,netcode,security,test}.md.
  - Script:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3c-review-wf_8f2da9fc-8fa.js
  - If cut: resume with resumeFromRunId "wf_8f2da9fc-8fa" and args {base: "2c68bf9", head: "989118e"}.
  - **After it:** fix the verified findings, apply C10's ruling, check.mjs 9/9, then the U3c close (the report's
    section in the checkpoint) and U3d.
- **U3c build A MERGED at c5d7260** (about 00:35 UTC). From .claude/worktrees/agent-a7fafe27dcdee96cd via
  worktree_patch.py and `git apply --3way`; the additive conflicts in Cargo.toml, crates/net/Cargo.toml and lib.rs were
  resolved; Cargo.lock adds tokio-rustls only. check.mjs 9/9 (242 measured, 256 bites); gate PASS.
- **Flaky loopback P2 test FIXED at 292e526** (FT1-FT4; three release runs pass, up 193-195; check.mjs 9/9).
- **NEXT: the evidence runs** (main session, one at a time, no agent running, loopback quiet; outputs committed under
  spikes/netcode-proof/results/u3c-kernel-check/):
  - (1) C10 quiet lateness: `NPT_WS_LATENESS_TICKS=10050 cargo +1.99.1 test --release --locked -p npt-net --lib
    <lateness_evidence_run> -- --ignored --nocapture --exact` (at least 10,000 samples per direction; record the load
    average). It goes to the advisor if p99.9 > 1 ms or max > 10 ms.
  - (2) The kernel cross-check: `bash spikes/netcode-proof/results/u3c-kernel-check/run.sh`, as root. The advisor's
    rules: VOID and INVALID may be re-run once, a MISS never.
  - (3) The C12e root run: `python3 spikes/netcode-proof/results/u3c-kernel-check/siocoutq-root-check.py`.
  - Then the skeptic round (rust-reviewer, netcode-architect, security-auditor, test-engineer) on U3c's final diff,
    ebe82b2..HEAD, as a workflow.
- **U3c build B MERGED at 2a93038** (about 00:20 UTC 2026-11-08). Content: npt-tcpinfo, check.mjs's unsafe-placement and
  libc checks, the C12e root-run script; fixed per critic B (u3c-build-b-fix.md).
  - Merged by worktree_patch.py from .claude/worktrees/agent-a3dfcb36e3c15ab56, then npt-tcpinfo moved into
    [workspace.dependencies].
  - check.mjs 9/9 (202 measured, 215 bites, self 21); gate PASS --no-smoke; pushed.
  - C12 (f) carried in plan §12.1's U8 row.
  - **Next:** fix-A finishes, then merge A on top with `git apply --3way`. Both builds touch Cargo.toml, Cargo.lock,
    crates/net/Cargo.toml and lib.rs; regenerate the lock and check that it adds tokio-rustls only. Then check.mjs 9/9,
    the gate, and a commit.
- **About 23:50 UTC: main merged into the branch** at 2c68bf9: PR #45's ADR, with the -18 and -31 hashes and
  drafts_commit 626df67 re-taken, and the STATUS and REPO_MAP conflicts resolved. check.mjs passes 9/9 (193/206), the
  gate passes with --no-smoke, and CI is green on 4f3a560.
- **Critic A: gaps (20 met, 4 missing, 8 wrong, 7 unproven)**; its notes are in u3c-critic-a.md.
  - W1 and W2 (high): flaky tests. The added-burst test races the wall clock; the replay test reads under three locks.
  - W3 (medium): poll_shutdown is unbounded and busy-loops when S's shutdown stays Pending.
  - W4: the stall rule is half a gap (the advisor rules); use iptables -I and make a run with no kernel retransmits
    VOID.
  - W5: the drop test hides a 1 s threshold. W7: fixes.rs's room step to call may_write. W8: a doc.
  - M2: no bites for most limit reasons.
  - Unproven: ws_config "one source" untested; no TLS 1.2 refusal test; the loopback identity checked only as bounds;
    tautological checks in (b)/(c); base_rtt_ns > 0 netting untested.
- **The advisor's U3c evidence rulings (2026-11-08; "Side rulings: U3c evidence" in advisor-u3c-plan.md), passed to
  fix-A:**
  - **(1) Half a send gap confirmed** as the judged stall rule.
    - Noise guard: judged episodes <= the client's tcpi_total_retrans per run. More episodes, or no retransmissions,
      make the run VOID.
    - No re-running until it passes: every run's output is kept. A VOID or INVALID run is re-run once on a quiet
      container; a second one goes to the advisor. A MISS is never re-run and goes to the advisor.
    - -I INPUT 1 approved; the one-gap count reported, not judged; the ruling recorded in run.sh's header before any
      run.
  - **(2) The tests switch to tokio-tungstenite now:** futures-util =0.3.34 as a dev-dependency of npt-net (default
    features off, ["sink", "std"]; no new lock package). The Bridge is deleted, and a split read/write loopback test
    added. For U6: WsServer uses tokio-tungstenite with futures-util as a normal dependency (§4.8, U5 audit), with no
    hand-written adapter.
  - **The C10 quiet run:** at least 10,000 samples per direction, nothing else running, the load average recorded. If it
    still fires, it goes to the advisor.
  - **W7 endorsed** if fixes.rs's pins stay unchanged. The room's no-takeover rule stays in fixes.rs.
- **Running now (3 agents):**
  - **fix-A**: general-purpose, xhigh, an isolated worktree at 4f3a560 with A's patch applied. Notes in its worktree:
    u3c-build-a-fix.md.
  - **fix-B**: the same at 539709a with B's patch. Notes: u3c-build-b-fix.md.
  - **The advisor** (afa86ecfb9b00e463) on (1) the half-gap stall rule and (2) the sync tungstenite Bridge against
    tokio-tungstenite.

  After them: merge B, then A (check.mjs and the gate after each), then the evidence runs (C10 quiet lateness, the
  kernel cross-check, the C12e root run), then the flaky-test fix (FT1-FT4), then the skeptic round.
- **RESUME HERE (after the usage reset, 2026-11-07 late; the worker restarted once): U3c BUILDS DONE**
  (workflow wf_87550c07-bbd; its result in u3c-workflow-result.txt; the per-agent journal is the session's
  subagents/workflows/wf_87550c07-bbd/journal.jsonl). Final snapshots in wip/ (A 272 KB, B 106 KB, base ebe82b2).
  - **Build A** (.claude/worktrees/wf_87550c07-bbd-1):
    - check.mjs 9/9 (227 measured, 241 bites); Cargo.lock adds tokio-rustls only.
    - Lateness: condition 10's trigger fired in 2 of 3 release runs (max up 3.45 ms, down 1.15 ms; p50 0.11-0.17 ms;
      n = 301, B running alongside). A quiet evidence run with more samples decides, and it likely goes to the
      advisor.
    - Mechanism test (c): outstanding 7,350 = N 7 x 1,050 at every return; the bites are caught.
    - New fix-1 pin over 6,000 phases: (0xb6f3_4ac0_188a_2727, [675,523,429,228], [0,0,0,0]); p50 653, p99 2,172,
      max 5,160 ms.
    - Open points:
      - kernel check stall rule at half a gap (the advisor must confirm before the run);
      - tungstenite driven through its sync API over a test Bridge, since there is no futures-util (U6's decision);
      - histograms 0.54 MB per connection;
      - per-second rows per connection (merging them is U6's).
    - **Critic A FAILED on the org spend limit.** Re-run it by resuming the workflow.
  - **Build B** (.claude/worktrees/wf_87550c07-bbd-2):
    - check.mjs 9/9 (201 measured, 214 bites); Cargo.lock adds npt-tcpinfo (libc, socket2 dev) only.
    - Open points:
      - the root-run script is Python, with signal handlers as the trap;
      - tcpi_notsent_bytes is read through NETLINK_SOCK_DIAG with socket2;
      - a zeroed byte buffer with offset_of!;
      - a generic non-Linux stub;
      - a path dependency, not a workspace entry;
      - tcpi_rtt can read 0 on loopback (1-4 us), which maps to None.
  - **Critic B: gaps.** To fix before B merges:
    - (1) 1.8 (f) is not carried: R1's run plan reads srtt against netem's 100 ms. Add it to the carried list and §12.1.
    - (2) The unsafe check only counts allows: `#[allow(unsafe_code)] mod linux;` or an allow moved to a helper still
      passes. Make it enforce placement on the two fns, with negative tests.
    - (3) The root run cannot pass on this kernel (always VOID): with every ACK dropped only cwnd (10) packets go out,
      so notsent grows too. Write at most cwnd x MSS during the drop.
    - (4) Comments overstate what check.mjs enforces (ties to 2).
    - (5) The deps step does not check that npt-net has no direct libc (M14). Add it.
    - (6) Stale .sh references; the script is .py.
    - (7) unacked_and_unsent's SAFETY invariant fails for a non-TCP fd (0x5411 on a device). Check SO_PROTOCOL ==
      IPPROTO_TCP with a bounded getsockopt first.
    - (8) The npt-net test's comparisons can all be skipped silently. Count them and assert a minimum.
    - (9) R4's 5 s wall-clock criterion: state it, or make it an identity.
    - (10) Off-Linux builds are unproven (stub): recorded.
- **Earlier (Mustafa at 96 % usage, about 20:52 UTC):**
  - **PR #45 MERGED** at 626df67 (merge commit, expected head 6ae3571) on Mustafa's go: "merge $45", read as #45.
  - **Next on the spike branch** (advisor-ws-docs-r2.md ORDER (3)): merge origin/main into it, and in the same commit
    re-take PM-0001-18's and PM-0001-31's row hashes in marks.json and move drafts_commit to 626df67's full SHA;
    check.mjs 9/9. Then the -18 marks.json split and the SERIES rows.
  - **The U3c build agents' work-in-progress is snapshotted** in .checkpoints/netcode-proof-test/wip/:
    - wf_87550c07-bbd-1.patch is build A, 267 KB on base ebe82b2;
    - wf_87550c07-bbd-2.patch is build B, 106 KB on base ebe82b2;
    - their notes are copied: u3c-build-a.md, u3c-build-b.md, plus B's u3c-build-b-bites.py and
      u3c-build-b-dryrun.txt.
  - **If the session is cut:**
    - Resume the workflow with Workflow({scriptPath, resumeFromRunId: "wf_87550c07-bbd"}). The worktrees under
      .claude/worktrees/wf_87550c07-bbd-1 and -2 keep their state while the container lives. A resumed agent that
      starts fresh should first `git apply` its patch at ebe82b2 and continue after its notes' last Progress line.
    - If the container is gone, apply each patch in a new worktree at ebe82b2 and continue from the notes.
    - The snapshot script is scratchpad/snapshot_wip.py: re-run it before any further cut.
- **U3c build workflow `wf_87550c07-bbd`** (npt-u3c-build), launched about 20:03 UTC on ebe82b2.
  - Two build agents (general-purpose, xhigh, isolated worktrees under .claude/worktrees/), each followed by a
    test-engineer completeness critic.
  - **A**: DelayStream, the fix-1 gate and writer, mechanism tests (a)-(e), SERIES rows, fixes.rs leftovers, and the
    kernel cross-check script (not run). CARGO_TARGET_DIR .gate/u3v3-harness-check-target.
  - **B**: npt-tcpinfo, the check.mjs deps checks, and the C12e root-run script (not run). CARGO_TARGET_DIR
    .gate/u3v3-model-target.
  - Notes inside each worktree: .checkpoints/netcode-proof-test/u3c-build-{a,b}.md (copy them over). Critics' notes in
    the main folder: u3c-critic-{a,b}.md.
  - Script:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3c-build-wf_87550c07-bbd.js
  - If cut: Workflow({scriptPath, resumeFromRunId: "wf_87550c07-bbd"}).
  - **After it:** merge B then A by hand (both touch the workspace and npt-net Cargo.toml), check.mjs 9/9 and the gate
    after each, commit. Then the evidence runs by the main session, one at a time with loopback quiet: the kernel
    cross-check, the C12e root run, and lateness at P2 with C10's trigger. Then the skeptic round.
  - **Meanwhile the main session writes the plan text** (u3c-plan.md 1.9; committed before any code merges).
- **Plan rev 2** (ebe82b2): the advisor's APPROVE with C1-C16 folded in. R20 (a) is granted by the advisor under ADR
  0002:264-265 (no escalation). Carries to U6 are in 1.10.
- **The deletion of stale .gate build folders was refused** by the permission check (about 20:00 UTC). Not retried;
  the old folders are reused as build dirs. Disk 7.1 GB free at 20:00.
- **The flaky loopback P2 test, measured** (40 runs per route, an instrumented throwaway worktree .gate/flaky-wt,
  log .gate/flaky-runs.log):
  - 1 failure in 80: wtq up 178. The client's quinn sent 200 DATAGRAM frames, ImpairSocket lost exactly 7 up packets
    with no other drops, and the server's quinn received 178 (the app got all 178). So 22 datagrams rode in 7 packets.
  - Cause: P2's +-10 ms independent jitter at the test's 5 ms spacing reorders packets. quinn's packet-threshold loss
    detection then declares 13-23 % of its packets lost (26-51 per run against 6-8 real), with 12-27 congestion events
    in about 1 s. cwnd falls to about 2.9-4.6 KB, datagrams queue and are packed several per packet, and one real loss
    takes many datagrams.
  - At 33.3 ms spacing (the tick; .gate/flaky-runs-30hz.log): no failure in 43+ route-runs so far, up min 191, no
    packing. quinn still reports 18-26 "lost" packets against 8-10 real, with 5-10 congestion events and cwnd at its
    2,904 B floor (likely its time threshold with delayed ACKs and P2's RTT jitter).
  - **Next:** the advisor rules on the fix: pace the P2 runs at TICK, keep >= 180, and assert app received == quinn's
    frame_rx.datagram. Apply it after U3c's build A merges.
  - **Finding for U6's C6**, and for -17 and -18 on WebTransport: ImpairSocket's independent per-packet jitter reorders
    back-to-back packets. A FullState (about 45 packets back-to-back), or any burst of 4 or more packets at P2+, meets
    quinn's packet threshold, and quinn's loss counter overstates real loss even at 30 Hz. Ask: record quinn's
    lost_packets and congestion_events against ImpairSocket's real losses in C6, and is independent jitter for
    back-to-back packets the intended model?
- **Docs PR #45: the advisor APPROVED head 6ae357111887849f30fc74a619b8bef7fb4f02f7** (about 19:45 UTC; notes
  advisor-pr45.md N1-N11). History:
  - Its review of e290c97 was REVISE: E1's "the bound times the measured slowdown, itself at most 1.10" read as a 1.10
    cap beyond the tolerance, and README line 23 had a stray space.
  - Fixed in 6ae3571 (worktree .gate/npt-readings) with its exact words. The PR text was updated to match, and step 3
    now says "merge #45".
  - Gate PASS --no-smoke; CI gate and smoke green on 6ae3571.

  **Waiting on Mustafa's go ("merge #45")**, then merge with 6ae3571 as the expected head (re-check the checks first).
  A later push needs the advisor again. Advisor finding 4: **PR #40 must merge with a merge commit, never squashed**,
  so the evidence commits bf9d248, 975e72b, 95a04a1 and 1e020a7 that ADR 0001 and docs/design/ws-reconnect.md cite stay
  reachable (put it in the report). Safety-net check-in trig_011mDKEXqy1t34dbADSL3HDh at 20:06 UTC.
- **U3c plan written**: .checkpoints/netcode-proof-test/u3c-plan.md. It covers:
  - DelayStream above TLS with a pump task and a process-wide release clock; TLS records at +22 B; frame limits before
    buffering;
  - the fix-1 gate functions (snapshot_budget, may_write = fixes.rs:777's three conditions); U6's WsServer calls them;
  - mechanism tests (a)-(e), the kernel cross-check, and SERIES rows;
  - R20 (a) recommended: a new `npt-tcpinfo` crate with two unsafe libc calls, the workspace's second unsafe exception,
    against (b) netlink sock_diag crates. Facts: socket2 0.6.5, rustix 1.1.6 and nix 0.31.3 expose neither TCP_INFO nor
    SIOCOUTQ safely (sources in scratchpad u3c-crates/);
  - tokio-rustls =0.26.6 (2026-09-28, rustls ^0.23.27).

  **Next:** the advisor reviews the plan. On APPROVE, a build agent in a worktree builds it, as for U3b.
- **Docs PR #45 OPEN** (about 19:15 UTC): https://github.com/Mergincanli/Shamble-Frontlines/pull/45, branch
  claude/npt-readings at e290c97 (fast-forwarded from main 69ea88e; worktree .gate/npt-readings), label needs-mustafa,
  subscribed. Content: the advisor's revision 2 (advisor-ws-docs-r2.md, 15 edits) with the second verifier's ADR-text
  corrections (verify-ws-docs-r2.md: W21 target from the link's return; W19/W17 never looser than bound x slowdown, a
  looser recomputed bound = a changed model to Mustafa; W20 readings marked; W24 risk; W12 record cites as on main);
  applied by script (scratchpad docspr/apply.py, every old text matched once), only the 7 changed lines re-wrapped;
  docs/design/ws-reconnect.md from the architect's aligned draft; REPO_MAP row, STATUS line; gate PASS --no-smoke,
  repo-map OK. **Next for #45:** the advisor's review of its head (needed before Mustafa's go); Mustafa's go;
  WS-RECONNECT-BAR (recommended (b), not blocking). **Spike-branch follow-ons** (advisor's plan_and_marks_changes in
  advisor-ws-docs-r2.md, with the verifier's fixes): after #45 merges, merge origin/main and re-take -18/-31 row
  hashes; marks.json -18 split (parts by params.transport, the three WebSocket checks, count >= 480, bites
  no-fallback on WT parts only, ws-budget-x2 on WS; W6: a bite run is held against the measured build's in-tolerance
  practice run and uses its factor, no own practice run, with a judge fixture); the ws-practice validity rule (W23:
  fewer than 480 records = beyond, investigated); SERIES.md (burst_heal = first snapshot written at or after the
  return; event.burst_return with offset, tick phase, unacknowledged bytes, srtt, per-tick sizes); plan §6.2/§7.2 R3
  (+0.4 h), R3's run plan (burst offsets 0-1 tick, RS kernel + CONFIG_HZ); fixes.rs's run-structured bound (48 x 10
  lineages, shared instants, tick offsets, per-tick snapshot sizes and message splits, W18; needs the scene's sizes);
  U3c (DelayStream exposes srtt and outstanding; fix 1 in WsServer with the model's other write conditions, W22;
  mechanism tests: formula table and properties); a report line for the time until extra delay is back under 250 ms
  (W15).
- **About 18:35 UTC (after the spend limit reset):** CI green on 7321cc0. The commit security review of 7321cc0 did not
  complete (HTTP 429; /root/.claude/security/log.txt); a later push re-checks. wf_abc2ef9b-8d4's advisor revision
  finished before the limit: saved as **advisor-ws-docs-r2.md** (15 edits incl. E13 for check item 11; the 10 % tolerance
  kept, measured as the practice run's median over the model's median for the same kind of run, run-to-run noise about
  +-3.5 %, the 2x-budget bite about 19 % slower; a new escalation WS-RECONNECT-BAR, recommended (b) leave it open,
  not blocking). Its architect and verifier steps failed on the org spend limit; resumed as task wuh42ub3z (same run
  id, the advisor cached). Saver b6su9re21 (2 h). **Next:** fold the verifier's result, open the docs PR, then U3c.
- **RESUME HERE (Mustafa at 98 % usage, about 16:00 UTC):** U3b fixes merged and pushed at **7321cc0** (check.mjs 9/9;
  notes u3-build-u3b-fix.md, planted defects u3bfix-plant.py: 16 planted, none survives). Open finding: the loopback
  P2 test's ">= 180 of 200 up datagrams" (wt_tests.rs:203) failed 1 of 18 runs, at base too (likely several datagrams
  per QUIC packet under load): measure before touching the threshold. Carried: U6 accept-loop caps and pinning joined
  clients; U3c's SERIES TCP rows; R1 reads ed4fb6d7ef68 on RS. Still running: workflow **wf_abc2ef9b-8d4** (advisor
  revision with the 10 % tolerance, record alignment, verifier); if the session is cut, resume it with
  Workflow({scriptPath: the npt-ws-docs-revise script under Running work, resumeFromRunId: "wf_abc2ef9b-8d4"}), then
  write the advisor's notes_text to advisor-ws-docs-r2.md, open the docs PR (claude/npt-readings from main in
  .gate/npt-readings, needs-mustafa). Then U3c. Saver benvikdbz (2 h, from about 15:40 UTC).
- **wf_f3167435-5b4 DONE** (about 15:10 UTC): U3b skeptic (u3-verify-u3b.md: pool starvable by one peer; slack stale
  under the RT switch; slot_for before the size/bound checks; throttle toggle reorders; ApplicationClosed counted as
  QUIC; accept loops unbounded (U6); SERIES text; 6 planted defects survive: bites needed); the advisor's docs-PR words
  (advisor-ws-docs.md, written by the session from its output: 14 edits E-PR, E0-E12, rulings: 480 client-bursts,
  NAME-L2PROBE approved, practice-run rules, a run-structured bound needed (F5: lineages and shared burst instants put a
  correct build's single-burst median at risk), real message sizes (F6), bite ws-budget-x2); the netcode-architect's
  record draft (ws-reconnect-record-draft.md, 565 lines; slice 3); the verifier (verify-ws-docs.md: the drafts predate
  the 14:30 tolerance; Mustafa's check item 11 at 1697-1698; E7 too strong; E11 wrong policy; E0/E8 "unsent" for -31;
  kernel cites tcp.c:630, :3842). 975e72b commits the model outputs under spikes/netcode-proof/results/u3-ws-fallback/.
- **U3b fix build** (general-purpose, opus, xhigh, worktree) · launched about 15:25 UTC · the skeptic's fixes and
  bites · notes inside its worktree (u3-build-u3b-fix.md; copy them over) · then copy files, check.mjs, gate, commit.
- **Workflow `wf_abc2ef9b-8d4`** (npt-ws-docs-revise) · launched about 15:26 UTC · advisor revision with the 14:30
  tolerance (returns notes_text: write it to advisor-ws-docs-r2.md), architect aligns the record, verifier
  (verify-ws-docs-r2.md) · script:
  /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-ws-docs-revise-wf_abc2ef9b-8d4.js
  · if cut: resume with resumeFromRunId "wf_abc2ef9b-8d4". **Then:** open the docs PR (claude/npt-readings from main
  in .gate/npt-readings), needs-mustafa; the spike-branch changes after it merges.
- **Mustafa decided** WS-SCOPE no, RESUME-CRED deferred, WS-BAR three-part (Decisions taken, 13:45 UTC). Since:
  bf9d248 ws_bar_fix1 (the model's bound for a correct fix-1 build at 480: "half within 0.67 s, at most 5 over 2.5 s,
  none over 6.9 s", correct 0.968, today and a 2x budget 0.000; 5 % slower 0.001: fragile, so the practice run
  matters); 5a5aba9 U3b ImpairSocket + L2Config merged from the build agent's worktree (check.mjs 9/9, 184/196 tests,
  gate PASS --no-smoke; notes u3-build-u3b.md; open: NAME-L2PROBE, slot id across runs).
- **Workflow `wf_f3167435-5b4`** (npt-u3b-verify-and-ws-docs) · launched about 14:40 UTC · phase 1 in parallel:
  rust-reviewer skeptic of U3b (notes u3-verify-u3b.md), the advisor's docs-PR words and rulings (advisor-ws-docs.md:
  ADR 0001 424, PM-0001-18's split with the practice-run rule and the model bound, the reconnect requirement, marks.json
  and plan R3 changes, bursts per run, NAME-L2PROBE), the netcode-architect's reconnect record (draft
  ws-reconnect-record-draft.md -> docs/design/ws-reconnect.md); phase 2: a verifier on both drafts (verify-ws-docs.md).
  Script: /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u3b-verify-and-ws-docs-wf_f3167435-5b4.js
  · if cut: Workflow({scriptPath, resumeFromRunId: "wf_f3167435-5b4"}). **After it:** fix U3b's findings; open the
  docs PR on claude/npt-readings from main in a worktree (.gate/npt-readings), label needs-mustafa; the spike-branch
  changes (marks.json -18, plan R3, SERIES.md); then U3c (DelayStream with fix 1's writer inputs).
- (done) **Waiting on Mustafa:** WS-SCOPE (recommended (b): fix 1 only in the proof test, the reconnect with slice 2),
  RESUME-CRED (defer under (b); else the session-id reading), WS-BAR (recommended: the form now, numbers after an
  unjudged L2 calibration run). Sent about 11:45 UTC 2026-11-07. His Windows TCP settings: recorded in "Facts".
- **Done meanwhile:** a0d7916 plan §4.7 lists `L2Config` (M6, before the code); 709e44a the round-3 clean-ups (rto_ns
  capped per doubling; recovery_ends_with_cwnd_at_ssthresh documented as a property; S11 additions incl. CONFIG_HZ to
  record; censored count in the tail; plan.md:297 cite). check.mjs 9/9. Deferred (move no number): rule tests for the
  retrans_out leak, the partial and D-SACK undos, reord_seen, the stuck L|R path; pins for the bite rows; the seed-1475
  story's leak (5,115 without it); the lineage/tail tables' stale round-2 bar (after WS-BAR); RS's kernel and
  CONFIG_HZ recorded at R1 against the "linux-6.8-tcp v3" header.
- **U3b build** (general-purpose agent, opus, xhigh, isolated worktree under .claude/worktrees/) · launched about
  12:00 UTC · ImpairSocket and L2Config per u3-build-plan.md U3b and M1-M5, M9-M15, M19 (M7/M8 for its test servers),
  loopback tests with both WebTransport crates, 10,000 dgram/s per direction, SERIES.md rows · notes inside its
  worktree (.checkpoints/netcode-proof-test/u3-build-u3b.md; copy them over) · returns its worktree path, uncommitted.
  If cut: SendMessage the same agent after the reset. **After it:** review the diff, copy files over (spikes/ equal to
  its base first), check.mjs, gate, commit, then a skeptic round on U3b.
- **Third skeptic round DONE** (wf_fed73d64-ec0; notes u3-verify3-{model,harness,stats}.md, 1.09 M subagent tokens):
  MODEL confirmed faithful to Linux 6.8 line by line (6.12's undo changes: identical rows; CONFIG_HZ 1,000 unmeasured:
  record RS's kernel and HZ on run day); p50 rise 435 -> 654 is Linux's own (post-burst undo clears the loss marks, a
  1.3 s TSecr sample delays the probe, recovery from cwnd 1; undo off: 509/828); max 4,196 a single tail draw (next
  3,766/3,375/3,013). Minor: recovery_ends_with_cwnd_at_ssthresh's bite does not bite; no rule tests for the leak,
  partial/D-SACK undo, reord_seen, the stuck L|R path; rto_ns shift overflow (unreachable); seed 1475's story omits the
  leak (5,115 without it). HARNESS: all as documented; browser models match sources; but "never abandoned" is tuned to
  B1's 1 s burst: at 0.6/1.5/2.5 s bursts an abort at max(500 ms, 4 x RTT) + JOIN-OR-CLOSE (at the 101, Join only if
  the old connection is still silent, else close with no Join) wins (p99 1,135 vs 1,270; 1,378 vs 1,632; 1,463 vs
  2,466; over 1.5 s at 1.5 s bursts 35 vs 613 per 20,000), costs +70-90 ms p99 at P2 1 s; join-or-close removes
  Chromium's throttle ratchet (H6's pre-101 close counts as a failure). L2 cannot measure the reconnect as planned
  (impair below TCP or model new connections' handshakes; LG runs the browser policy; Windows-like LG clients; per-
  address limits). Without the reconnect the WS bar is fix 1 alone's (p99 2,164, max 11,384; no-TS 2,035/12,550).
  STATS: joint DP exact; bootstrap = sampling noise only; the searched bars sit at the grid's edge (overfit); the
  median part has 46 ms (7 %) margin: x1.07 -> 0.505, +45 ms -> ~0.5; tail parts survive +/-20 %; a trigger twice too
  slow passes 0.98; real runs are not independent (20 s = 600 ticks: same tick phase every burst; spread bursts by a
  random 0-33 ms); use tail parts + mechanism checks (budget formula, trigger; per-burst L2 series: bytes outstanding
  at the return, silence before each attempt), a median part midway between correct and the nearest broken build
  (e.g. ~0.75 s), and an unjudged L2 calibration run before freezing any number; the lineage/tail tables still print
  round 2's stale bar; pin only covers the recommended row. marks.json/npt-judge can express a three-part bar
  (p50, count with where, max) on a part selected by params.transport 'ws', plus count >= 480.
  **Next:** WS-SCOPE (recommend now: fix 1 in the proof test, the reconnect designed and measured with slice 2),
  RESUME-CRED (defer with slice 2 under that choice), WS-BAR (form + calibration) back to Mustafa; follow-ups after
  his answer: stale bars in the lineage/tail tables, the non-biting test, more pins, rto_ns guard, plan.md:297 cite.
- (done) **Merged v3 = 1e020a7 (pushed)**: model v3 + harness v3 copied whole from the two worktrees (spikes/ equal to
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

## Running work, U4 wave 1: the resume card's blocks of 2026-11-10 (moved 2026-11-11 by card_refresh.py)
_Lines 11-184 of the checkpoint at 6486e6e, verbatim._

**The current step: U4 wave 1** (u4-build-plan.md rev 2, sha256 0d69332c…, approved with conditions; advisor-u4.md).
- **U4s:** merged (f080551). **Main:** merged in (df8975b).
- **U4a, the wire (npt-proto):** built, not merged.
  - all 10 steps pass twice: measured 296 = 275 + 21, bites 321 = 292 + 29, self 52 = 48 + 4;
  - carries 1-5 and 34 met; no seam change;
  - worktree `.claude/worktrees/agent-a9297644e7c0354f3` (base dffef0e); notes u4a-build.md; snapshot wip/u4a.patch.
- **U4b, the loop policy (npt-server's schedule.rs):** built by the main session, not merged.
  - all 10 steps pass twice: measured 285 = 275 + 10, bites 308 = 292 + 16; carries 6 and 35 met;
  - worktree `.gate/u4b-wt` (base 4b5812a); record u4b-build.md; snapshot wip/u4b.patch.
- **The U4a + U4b review, round 1 of 2:** workflow wf_4153791e-ba1, resumed 10:05 after the 07:3x spend-limit cut
  (task wum1henzm). Notes u4ab-review-{netcode,security,rust,test}.md.
  - **Done:** netcode-architect, APPROVE WITH FINDINGS for both; security-auditor, APPROVE WITH FINDINGS for U4a.
    - **Before U4a merges:**
      - N1, Medium: encode never applies Limits::SNAPSHOT (1,000 B); content_limit gives the 1,150 B receive cap to
        both sides. Fix in msg.rs, with a 1,000/1,001 B test.
      - N2, Medium: the snapshot header carries 16-bit command sequences where §4.4 and §4.14 say tick offsets. The
        sequences are the better choice; record it in the catalogue and U4a's report; §4.14's -58 row changes. A
        plan text edit: the advisor rules on it (condition 12).
      - N8, Low: the catalogue edits the builder listed (item 3's note, the codec maxima, the header fields, the event
        form, the proposals). The main session makes them at the merge.
    - **After U4d merges, before U4e's brief goes out:** S1 holds, and SEC-U4a-1 is its security side. The fix: an
      owner-only record kind in npt-sim (velocity, ammo, fire timer, reload, active reload; velocity leaves SOLDIER);
      U4a's half is two lines (the kind joins the own-state rows of both GROUPS tables). It goes in the main session's
      commit, with the advisor.
    - **Lows:** N3-N7 and U4b-L1 (the time origin for U6), follow-ups.
  - **Round 1 complete:**
    - rust-reviewer and test-engineer: REVISE for U4a, for the send cap alone; APPROVE WITH FINDINGS for U4b (Lows).
    - All four reviewers name the send cap as U4a's one blocking fix. Security marks S1 (SEC-U4a-1) as before the
      merge; the other three say after U4d. It goes to the advisor with all four views.
  - **The U4b fix pass:** done by the main session (u4b-build.md, "Round 1 and its fix pass"): T-B1's new test (its
    plant fails it by name), T-B3, T-B2. 11 clean tests, 17 bites.
  - **The U4a fix pass, then round 2 on what changed:** workflow wf_5a059972-5c8 (task w0yg500x5).
    - The fixer works in U4a's worktree: the send cap, the single "no ack" encoding, Debug redaction, the never-0
      guard's test, a nit, and the N2 record.
    - Then rust-reviewer, test-engineer, netcode-architect and security-auditor, one at a time, on the diff against
      the pre-fix copies `.gate/u4a-prefix` and `.gate/u4b-prefix` (remove both after the merge).
- **U4d (the stand-in sim):** built and not merged.
  - check.mjs twice: measured 304 = 275 + 29, bites 327 = 292 + 35.
  - Carry 13 is partly met: a counting allocator is impossible under `unsafe_code = "forbid"`, so a storage-identity
    test stands in. The freeze stopped nothing.
  - **Its slip:** at 06:38 a backgrounded `cd` chain ran check.mjs in the main folder. It wrote only 408 build files
    into the main folder's gitignored `.gate/netcode-proof-test/target`; `git status` is clean.
  - **Its shared-target lesson:** cargo's build records do not tell copies apart, so keep touching the crate files
    every 2 s during a run. The test-engineers' prompts now say so.
  - **Round 1 complete** (u4d-review-{netcode,rust,arch,test}.md):
    - netcode-architect: REVISE, one High (vehicle parts written as world transforms that change every tick, not
      travelling with their parent) and five Mediums (the stress scene's magazine runs dry in an S30; press counters
      against condition 1's wording; the fired flag; vehicles and stand-in projectiles not "in the fight"; effect rows
      changing every tick).
    - test-engineer: REVISE, five Mediums (ES out of two PM-0001-2 bands, untested; the 1 MB cap as 2^20 where
      PM-0001-7 judges 10^6; D-9's "by its fraction"; prediction on a catch-up tick; load_movers).
    - rust-reviewer: APPROVE WITH FINDINGS, 2 Medium and 9 Low. engine-architect: APPROVE WITH FINDINGS, 2 Medium,
      the gaps U4g and U4h will hit, tied to S1.
  - **U4d's fix pass is running** (workflow wf_a8ff8550-67d, task wihgj9fvm), to the advisor's W1-U4D and W1-S1's
    npt-sim half. Its pre-fix copy is `.gate/u4d-prefix`.
  - **U4d's round 2** waits for U4a's merge and the main session's npt-proto half of S1. It runs on the combined diff
    in a scratch worktree, with the security-auditor added for S1's part.
- **The advisor's wave-1 rulings: done** (advisor-u4-wave1.md, 626 lines, with the exact words for every text
  change).
  - **The order:** U4b merges as it is. U4a merges after its round 2, before the S1 fix. U4d merges after U4a.
  - **W1-S1:** an owner-only record kind in npt-sim; the freeze is lifted for exactly this change, in U4d's fix pass.
    - npt-proto's half belongs to the main session, in U4d's merge commit after U4a merges: the kind joins both GROUPS
      tables' own-state rows; the codec refuses a kind its section's rows do not list (this closes SEC-U4a-2); the
      GROUPS test is extended; the -57 bites test sends the new record.
    - Lanes gets a "Lifted for one change" sentence.
  - **W1-U4D:**
    - Fix now: H1, N-M1, N-M3 = T-M4, N-M4, N-M5, R-M1 = T-M5, A-M1, T-M1, T-M2, T-M3.
    - The press counters: the build is right. Condition 1's words are amended, §4.2's and §4.10's words change, and
      InputReports gains per-command refusal records; there is a new U4f carry and a new reading.
    - A-M2 is carried to U4g. The tuning is approved with conditions. Carry 13's counting-allocator half goes to U5.
  - **W1-N2:** approved, with new words for §4.4 and §4.14. -58's whole judged set moves from U4r to U4g. marks.json's
    lt-tick-offset text changes in U4j's brief. A reading, and a new U4e carry: no event ages past 255 ticks.
  - **W1-PUB:** all approved; nits.
  - **W1-U4E:** Q1-Q11 answered; the brief's rev 3 carries them with W1-S1, W1-N2 and the new U4e carries.
  - **W1-PLAN:** SQUASH-CITES and TCP-FREEZE confirmed, with new words.
    - plan.md:1436-1438 changes.
    - ws-reconnect.md's `.checkpoints/` cites at 95a04a1 (:26, :74) must point at a kept copy: the closing report's
      appendix.
    - The U8 row and §15's R-25 row change for TCP-FREEZE.
  - **W1-TARGET, for wave 2:**
    - a `flock` lock on `.gate/shared-target/.lane.lock` (shared for builds, exclusive for counted and bite runs);
    - a provenance check: the log shows "Compiling <crate> (<this copy's path>)";
    - at least 4 GB free before a wasm32 or release build;
    - `cargo clean --target-dir .gate/shared-target` between waves.
  - **The plan text edits** (plan.md, u4-build-plan.md, u4s-carries.md, advisor-u4.md's amendment line) are the main
    session's, with the advisor's words, at the next natural stop.
- **U4c (the link model):** built and not merged.
  - netcode-architect and rust-reviewer: both APPROVE WITH FINDINGS. The rust Medium: one test cannot catch a fault
    in `next_event_ns`.
  - Its test-engineer: re-running in the same workflow.
  - Notes u4d-build.md, u4c-build.md and u4c-build/; snapshots wip/u4d.patch and wip/u4c.patch, refreshed 10:06.
- **U4j (the judge and marks change):** not started. Its brief is ready (u4j-brief.md): the first free slot, or the
  main session.
- **Agents:** 3 at most, reviewers included. Now: 2 in wf_f54246ff-e0d and 1 in wf_4153791e-ba1.
- **Night work (issue #16):**
  - items 2 and 3 done: the U4e brief draft, rev 2, at `cs/npt-u4e-brief` 0dccbf0; the U4s skeptic check at
    `cs/npt-u4s-skeptic` 6652be4, nothing Critical or High, S1 Medium, 11 Lows;
  - items 4 (the U4r brief) and 5 (the U3d skeptic check) are queued for tonight.

**NOW, 2026-11-10 about 20:40 UTC (after the container restart; the spend limit cut the fixers about 16:25 UTC and
reset at 20:00):**
- **U4b MERGED at 411eeee** (pushed): check.mjs all 10 steps, measured 286 = 275 + 11, bites 309 = 292 + 17, self 48;
  gate PASS --no-smoke. wip/u4b.patch was deleted in that commit.
- **U4a MERGED at 952f734** (pushed; about 20:45 UTC): check.mjs all 10 steps, measured 311, bites 342, self 52;
  gate PASS. wip/u4a.patch was deleted in that commit. CI was green on 411eeee (U4b) and on 952f734 (U4a).
  - The merge checks' output file mixed two overlapping runs; both exited 0.
  - **The clean evidence:** a fresh run on 17a8b2c (952f734 plus checkpoint notes only) passed all 10 steps: measured
    311, bites 342, self 52 (scratchpad post-u4a-check-952f734.txt).
  - Housekeeping later, by the approved method (leftovers to a local branch, then a plain `git worktree remove`):
    `.gate/u4a-prefix`, `.gate/u4b-prefix`, `.gate/u4b-wt`, `.claude/worktrees/agent-a9297644e7c0354f3`.
  - **Next merges:** U4c after its round 2. U4d after its fix pass, then the main session's npt-proto half of S1 in
    a scratch worktree, then round 2 on the combined diff (with the security-auditor for S1), then the merge.
  - **The plan text edits** (W1 rulings with the advisor's words, plus NETEM-BAR (a) in the U8 row): give them to an
    agent when a slot frees, then a test-engineer check, as with the U4 conditions.
- **(Done) U4a's merge, as it stood at 20:40:**
  - `wip/u4a.patch` (the fixed copy) is applied, with three catalogue edits: the Input row's codec maxima (5, 23, 19,
    161 B); §3's header row as built, in W1-N2's words; §6 item 3's U5 note.
  - check.mjs and the gate are re-running (task bdu2c9yb1; scratchpad merge-u4a-check.txt and merge-u4a-gate.txt).
    The first run was killed by the restart. Expect measured 300 + 11 = 311, bites 325 + 17 = 342, self 52.
  - Then commit with `git rm` of `wip/u4a.patch`, and push.
  - N8's last items (the event wire form; the proposals for Leave, ClientSettings, the peer set and SessionStatus):
    a follow-up, Low.
- **SAVE at about 20:50 UTC (Mustafa at 91 % usage):** snapshots refreshed: wip/u4d.patch (14 files), wip/u4c.patch
  (3 files), and **wip/u4j.patch** (29 files, base 411eeee). U4j's worktree is
  `.claude/worktrees/agent-a3b8dfeb01ed52b50`, with notes u4j-build.md, copied to this folder.
  - To resume U4j: SendMessage to its agent; or, if the worktree is gone, a fresh builder at 411eeee applies
    wip/u4j.patch and continues from u4j-build.md.
- **Resumed at 20:36:**
  - U4d's fixer (wf_a8ff8550-67d, task wsnzz9qtq);
  - U4c's fixer and its round 2 (wf_610486f7-cc1, task w11llknmp);
  - **U4j's builder** (an Agent in its own worktree; brief u4j-brief.md, plus W1-N2's marks change and the
    shared-target lessons; base 411eeee).
- Their snapshots, taken at 20:35, are unchanged since 16:05.

**SAVE POINT, 2026-11-10 about 16:05 UTC (Mustafa at 92 % usage):**
- **U4b's merge is in progress in the main folder.** `wip/u4b.patch` (the fixed U4b, with tests/schedule.rs at
  05e2068b) is applied, uncommitted: crates/server's lib.rs, clippy.toml, src/schedule.rs, tests/schedule.rs.
  - check.mjs and the gate were running (task bvya54bq9; outputs in the scratchpad, merge-u4b-check.txt and
    merge-u4b-gate.txt).
  - On resume, if they are lost: re-run `node spikes/netcode-proof/check.mjs` (expect measured 286 = 275 + 11, bites
    309 = 292 + 17) and `node tools/gate.mjs --no-smoke`.
  - Then commit "U4b: the loop policy …" with `git rm .checkpoints/netcode-proof-test/wip/u4b.patch` in the same
    commit (WIP-CLEAN), and push.
- **U4a: round 2 complete, ready to merge after U4b** (wf_5a059972-5c8).
  - All four reviewers APPROVE WITH FINDINGS, Lows only: R2-L1, R2-T1, R2-N2a; for U4b, R2-T2. Under the budget these
    are follow-ups; there is no third round.
  - The fixer's counts: measured 300, bites 325, self 52.
  - The merge: apply `wip/u4a.patch` (the fixed copy), and in the same commit make the catalogue edits N8 lists (item
    3's note, the codec maxima, the header's sequence fields per W1-N2, the event form, the proposals) and delete
    `wip/u4a.patch`. Then check.mjs and the gate.
- **U4c: fix pass, then round 2, running** (wf_610486f7-cc1). Pre-fix copy: `.gate/u4c-prefix`.
- **U4d: fix pass running** (wf_a8ff8550-67d). Pre-fix copy: `.gate/u4d-prefix`. Its round 2 comes after U4a's
  merge and the npt-proto half of S1.
- **Resume the workflows:** `Workflow({scriptPath: <the run's script under /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/>, resumeFromRunId})`:
  - npt-u4c-fix-round2-wf_610486f7-cc1.js;
  - npt-u4d-fix-wf_a8ff8550-67d.js.

**The next three actions**
1. **When the U4a + U4b review returns:** a fix pass within the budget (two rounds at most; Medium and Low become
   follow-ups).
   - Then merge U4b, then U4a, into the main folder: one at a time, on committed state, by
     `python3 -I .checkpoints/netcode-proof-test/worktree_patch.py` then `git apply`, check.mjs and the gate.
   - Delete each one's wip patch in its merge commit (WIP-CLEAN). Give each unit's size in three lines if it ran well
     past its plan (SIZE-CHECK; U4a: 5,500 lines for S-M).
2. **When wf_f54246ff-e0d returns:** fold U4d's and U4c's reports, run their fix passes, and merge them one at a time.
   - Then U4j (u4j-brief.md), then U4c's L-c8 judged re-run on U4j.
   - Then wave 2: U4d2, and U4e once U4a, U4b and U4d are merged.
3. **To the advisor** (a general-purpose agent reading `git show origin/main:.claude/agents/advisor.md`), in one
   request:
   - S1, with the review round's views: the soldier's layout serves two catalogue rows; the fix needs a per-group field
     selection in U4a and an owner-only record from npt-sim, a seam change after U4d merges;
   - U4a's supporting public items, beyond §4.1's list;
   - the U4e brief's 9 questions (`git show 0dccbf0:.checkpoints/npt-u4e-brief/u4e-brief-draft.md`).
   - The brief itself goes to the advisor after wave 1 merges, with its 28-row recheck done.

## Running work, U4 wave 1 (2): the resume card of 2026-11-11 morning (moved at Mustafa's HOLD, about 16:20 UTC, by card_refresh2.py)
_The card's body as it stood at that moment, verbatim._

**The current step: U4 wave 1** (u4-build-plan.md rev 2, approved with conditions, sha256 0d69332c… until the W1 text
batch of 2026-11-11, a81b230f… since; advisor-u4.md;
the advisor's wave-1 rulings, with the exact words for every text change: advisor-u4-wave1.md). The card's blocks of
2026-11-10 (the wave-1 list, NOW, SAVE, SAVE POINT, the next three actions of then) are in history.md, "Running work,
U4 wave 1", moved by card_refresh.py.
- **Merged:** U4s (f080551), U4b (411eeee), U4a (952f734), **U4c (f3f7c2f, 10:38 UTC)**, **U4d with W1-S1's npt-proto
  half (39f4290, about 12:15 UTC)**; CI green on 411eeee, 952f734, f3f7c2f and 39f4290. Main merged in: kit-v3-4 (df8975b) and
  PR #49, the STATUS split (5ca77eb, by merge_status_split.py, as Mustafa asked on 2026-11-11; CI green on 5ca77eb).
- **U4d's merge (39f4290):** round 2 (wf_4b5ee472-66b) all five APPROVE WITH FINDINGS, no Critical or High;
  SEC-U4a-1 and SEC-U4a-2 closed in writing. check.mjs on the merged tree: measured 365 = 325 + 39 + 1, bites 412 =
  366 + 45 + 1, self 52; gate PASS. The catalogue's own-state row records the section as built (W1-S1). Report:
  u4d-report.md (the N2-M1 merge condition met: T-M1's window, the per-window rates, the test-engineer's probe X2 at
  7.578 a second over 600 s). wip/u4d.patch and wip/u4d-s1.patch deleted in that commit.
  - Open for the advisor (Part B): (A) the ES knobs; N2-M1's fix form (carry 52); N2-M2 (ES's vehicles); (C) the
    "(9 fields)" slip; Weapon, Scene::weapon, Scene::health for §4.1; the U4e carries from round 2 (N2-L1, the
    security auditor's 1-2, N2-L3 (c)).
  - Housekeeping later (approved method): `.claude/worktrees/wf_f54246ff-e0d-1`, `.gate/u4d-prefix`, `.gate/u4d-s1`.
- **U4c's merge (f3f7c2f):** round 2 (wf_610486f7-cc1): rust-reviewer, test-engineer and netcode-architect APPROVE WITH
  FINDINGS, Lows only (R2-A = R2-1 = R2-N1, the jitter figure in time_threshold's doc; R2-B = R2-2, drive() hangs
  rather than fails on a model that keeps naming a past instant; R2-3, the fixer's third run never ran): follow-ups.
  check.mjs on the merged tree: measured 325 = 311 + 14, bites 366 = 342 + 24, self 52; gate PASS. wip/u4c.patch
  deleted in that commit. SIZE-CHECK: about 3,130 lines for size M (lib.rs +1,180 −323, tests.rs 1,723, tests/l1.rs
  +230): not well past.
  - Housekeeping later (approved method): `.claude/worktrees/wf_f54246ff-e0d-2`, `.gate/u4c-prefix`.
- **The plan text edits: done** (workflow wf_e1058c57-961, about 10:44 UTC; notes w1-plan-edits.md and
  w1-plan-edits-check.md; base 20702fd; +125 −47 lines in the four files). All 37 ordered texts are in, word for
  word (the checker's script took each from advisor-u4-wave1.md by line range); Mustafa's NETEM-BAR (a) is quoted
  exactly in the U8 row (plan.md). The check's two Mediums are fixed and re-checked: F1 (the W1-U4D amendment line
  moved to advisor-u4.md's end, so lines 1-475 match the base and every cite into it holds) and F2 (§5 U4d's
  generator words, "words mine"). New sha256: plan.md 4717029c… (1,518 lines), u4-build-plan.md a81b230f… (1,070),
  u4s-carries.md 19e7ca98… (272), advisor-u4.md 7b3e76ca… (479).
  - Open Lows, for the advisor request after wave 1: F10 (the ruling's §4.2 sentence quotes condition 1 as "never"
    where it says "not": a one-word change on the advisor's nod); F3 (carry 40's added sentence and its label); F5
    (a bare "§4.7" in §4.1's Why cell means plan.md §4.7); F6 (two clamp-idempotence claims left unbounded beside
    TUNE-2's bound); F7 (u4s-carries.md's header "44 entries", now 50; carry 13's moved half); F8 (U4E-Q6's two
    conditions go to the U4e and U4r briefs); F9 (a sentence the swap left reading as the manifest's); F11 (W1-PUB's
    approved public items its §4.1 list leaves out); F4 (a cite off by one line: advisor-u3q.md:142-143, not
    :143-144).
- **U4j, the judge and marks change:** **built** (agent a3b8dfeb01ed52b50, done about 10:02): all 10 steps pass twice
  (measured 304 = 286 + 18, bites 327 = 309 + 18, self 48 of 48); its four plants fail by name. Copy
  `.claude/worktrees/agent-a3b8dfeb01ed52b50` (base 411eeee); notes u4j-build.md; snapshot wip/u4j.patch (10:05).
  - SIZE-CHECK: about 2,860 lines added for its size M (code 1,330, tests 1,030, marks.json and docs 370, fixtures
    130): not well past its plan, so no stop.
  - **Review round 1 running** (workflow wf_1ea8a0dd-fee; the test-engineer cut by the spend limit, resumed 11:11 as
    task w4p9h5eio): netcode-architect, security-auditor, then test-engineer; notes
    u4j-review-{netcode,security,test}.md.
    - **netcode-architect: REVISE.** Before the merge: NC-U4J-1 (FAR BETTER uses f = max(1, s) with no band or
      tolerance check: Mustafa's 1.10 cap can be exceeded), NC-U4J-2 (a practice run added after the counted run
      turns its FAIL into INVALID), NC-U4J-3 (README never says whose slot indexes `hs`), NC-U4J-4 (the twin's role
      is not checked), NC-U4J-5 (the `hs` writer rules' four gaps), NC-U4J-7 (the practice run's heal-record count is
      never checked). Readings (a), (b), (d), (f) right; (e) partly (NC-U4J-6). Follow-ups NC-U4J-6, -8 to -14.
    - **security-auditor: REVISE.** Before the merge: **F1 High** (the same FAR BETTER hole: a log can move X, Y and Z
      past WS-BAR's cap), F2 (M_m, the band and the guard come from the log's copy of the bound line, never checked
      against marks.json), F3 (one practice run per build checked only inside the counted run's set), F4 (any log
      can supply any hash list; the Arm rule trusts the log's own aarch64 claim), F5 (the twin's identity unchecked),
      F6 (parse_bound can panic on two 30-digit decimals: an i128 overflow), F9 (a ruling is any non-empty text).
      Follow-ups: F7, F8 (Medium: no minimum coverage for the decoded-hash check), F10-F12.
    - **test-engineer: REVISE** (done about 11:25): F1 High confirmed by a probe run (band 2.0-2.0, s 1.9: X' 1.273
      s, a 1.2 s median PASSES); F9 (the 1.10 cap unpinned: a plant raising it to 1.12 passes all 65 judge tests);
      F2-F8, F10-F12 Medium before the merge, most shown by probes on the unchanged code; F13-F16 Low.
    - **Round 1 complete: three REVISE.** The advisor rules on it now (agent aef3e07087d16c503, writing
      advisor-u4j.md): the deduplicated fix list with the proofs; the calls (coverage floor F10, rulings as text F9,
      pinning the bound line F2/F6, NC-U4J-10, F16, the `hs` writer rules, the Arm rule's key); the readings (a)-(f);
      F1's severity; anything for Mustafa.
    - **The advisor's rulings: done** (advisor-u4j.md, about 11:45): REVISE; 14 fixes FX-1 to FX-14 in one pass
      (FX-1, the High, first: the band must contain 1, FAR BETTER gives f = 1, no f above the tolerance), each with
      its named test or fixture; follow-ups FU-1 to FU-6 (owners named); carries to U4e, U4r and U6. Readings (a),
      (c), (d), (f) accepted; (b) changed by FX-3, (e) by FX-5. F1 stays High (a rule loosened beyond Mustafa's
      words); a round 3 is open to F1 alone. Decisions for Mustafa: none.
      - At the merge: copy the builder's readings (a)-(f) into the notes (R3 quotes them; they are not in
        u4j-build.md); the readings list gains the orphan reading (R5's words); PR #40's report names FU-1 to FU-5 as
        security findings left open.
    - **The fix pass and round 2: running** (workflow wf_0906017a-a9b, task wgj6zbosy): the fixer in U4j's worktree
      (each named test first, failing on the unfixed code), then netcode-architect and security-auditor (read only;
      the security table of every path through `find`), then test-engineer (probe A and the advisor's three F1
      plants on the new head; counted check.mjs). Notes u4j-round2-{netcode,security,test}.md. Pre-fix copy
      `.gate/u4j-prefix` (411eeee plus wip/u4j.patch of 10:05; byte-identical to the reviewed build, checked 11:27).
  - Its fix pass, if any: by its builder (SendMessage to a3b8dfeb01ed52b50), in its copy.
  - At its merge, the main session's: u4-build-plan.md §4.1 gains U4j's public items (`TickRecord.hs`,
    `set_hashes`, `hashes`; `marks::BoundNumber`, `Check.ws_bound`, `Rule::WsPractice`; `Outcome::Empty`,
    `fails()`; `Lack::Empty`); the builder's readings (a)-(f) go to the advisor with the reviewers' views.
  - Then U4c's L-c8 judged re-run on U4j.
- **The advisor's Part B: done** (advisor-u4d-close.md, 521 lines, about 12:40): no escalation.
  - **U4d-F** (B1.2 and B1.3 are its approved plan, with its files and symbols): one move rule shared by the World and
    the generator; the generator reckons what it emits (crouch, its own MoveSpeed, dropped commands), still
    open-loop; ES's 15 hulls loop within 1,400 cm (a scene knob); `Recording::GENERATOR` 1 to 2; the measurement on 3
    seeds (12 minutes, the last 10, all four bands and both halves in 9.5-10.5); `aim_one_in` set from it; T2-L1's
    two tests first. No setting works: stop and bring the numbers to the advisor; the floor never moves. Its own
    review budget: netcode-architect and test-engineer. It merges before U4d2's brief is final; U4e need not wait.
  - **The text batch (for the main session, when a slot frees; copied from the file's code blocks by line range,
    never retyped):** carry 52's tail (B1.2) and new carries 59 (U4r), 53-57 (U4e) and 58, 60 (U4r); carry 51's span
    fix; the erratum block appended to advisor-u4-wave1.md (B1.4: "(9 fields)" and three more slips); §4.1's npt-sim
    rows gain Weapon, Scene::weapon, Scene::health (B1.5, u4-build-plan.md:184-185); u4d-report.md's TUNE-1 kill-rate
    bullet (B1.6: 0.272 a second over an S30); F10 ("never" to "not", u4-build-plan.md:247), F3, F4, F5, F6, F7, F9
    (B3); plan.md's NA1 (R-31 at :1424), the U8 row's new case and NA4's run condition (:1248), NA5 (C-U8-6, C-U8-4),
    C-U6-8 and the U6 row's weapon and health (:1246) (B4). Then a test-engineer check, as for the W1 batch.
- **Agents (3 at most), since about 12:45:**
  - U4j's fix pass and round 2 (wf_0906017a-a9b, task wgj6zbosy);
  - **U4d-F's build and review** (wf_9ce88fa3-452, task wg0l1ugq1): one builder in a fresh worktree from the spike
    head (notes u4df-build.md there), then netcode-architect and test-engineer (u4df-review-{netcode,test}.md);
  - the U4e brief's rev 3 (wf_c92cd986-072, task wnwulchr5): netcode-architect drafts u4e-brief.md from the night's
    rev 2 (0dccbf0) with W1-U4E's answers, the carries as they stand, W1-TARGET and the 28-row recheck against
    39f4290; test-engineer checks (u4e-brief-r3-check.md); one revision. Then the advisor checks the brief.
- **Night work (issue #16):** all four items done: 2 and 3 on the night of 2026-11-10; 4 and 5 last night, from 02:16
  Amsterdam. The queue is empty: post tonight's items this evening.
  - **4, the U4r brief:** rev 2 at `cs/npt-u4r-brief` ed973c9 (1,007 lines). netcode-architect's check: 8 Medium and 8
    Low, all fixed in rev 2.
    - 12 open questions for the advisor; U4R-Q1 blocks the bots (npt-lab has no npt-bots dependency, and no unit owns
      that manifest edit).
    - 34 rows to recheck against U4c, U4d2, U4e and U4j; 12 day runs (D1-D12).
    - It goes to the advisor once U4r's dependencies merge.
  - **5, the U3d skeptic check:** `cs/npt-u3d-skeptic` 558fc0e. No Critical or High; 4 Mediums (NA1 = TE1, TE2, NA4,
    NA5) and 8 Lows, follow-ups for U3d's report.
    - NA1's proof, run here (na1-output-20261111.txt): at E 9,999 ns (a PASS), -3's p99.9 fails in 49.7 % of
      2,000 seeds. The finding holds.
    - NA1 (R-31's wording plus a U3D-MISS case) and NA5 (C-U8-6: read while the socket is bound; a missing row makes
      the leg INVALID) change ruled text: to the advisor with the plan text edits.
    - TE2-TE4's plants need a quiet shared target: after wave 1's merges.
  - **NS-START, the evidence (b) asks for:** get_trigger at 10:00 UTC today.
    - The routine reads `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`, updated 2026-11-10 20:48 UTC.
    - Last run 06:17 UTC, SUCCEEDED; next 23:10 UTC.
    - Last night's 00:10 and 01:10 Amsterdam runs left no comment.
    - A likely cause, unproven: this session's agents hit the account's spend limit at about 21:20-21:30 UTC, and the
      night shift runs on the same (university) account; the first run that worked fired at 01:16 UTC.
    - STATUS on main still says `10 0-6`, set 18:00 UTC: one change behind (docs-keeper's, on main).

**The next three actions**
1. **(Done 12:15)** U4d merged at 39f4290. **Now: wave 2's preparation**, since U4e's dependencies (U4a, U4b, U4d)
   have merged: the advisor's Part B (U4d's open points, N2-M1's fix form, N2-M2, the public items, the U4e carries,
   the plan batch's Lows, NA1 and NA5); the U4e brief's rev 3 with the 28-row recheck against the merged code; then
   U4d2's builder brief (carrying carries 52 and N2-M2's ruling, and W1-TARGET's lock).
2. **When U4j's review returns:** a fix pass within the budget, by its builder (SendMessage); round 2 on what changed
   if a finding needs it; then the merge, the same way, with §4.1's items; then U4c's L-c8 re-run on U4j.
3. **(Done 10:44)** The plan text edits; their open Lows join the advisor request below.
- **After wave 1:**
  - One advisor request: U4j's readings (a)-(f); U4d's points (A), (B), (C) and its new public items; NA1 and NA5;
    the plan text batch's open Lows (F10 first: its one-word change); and the U4e brief's rev 3 (W1-U4E's answers,
    W1-S1, W1-N2, the new carries).
  - Then wave 2 (U4d2, U4e), under W1-TARGET's lock.
- **This evening:** post tonight's night items to issue #16 (a check-in is scheduled for 19:30 UTC:
  trig_01W9mBGauhxrWnn1rSuweDic). Candidates, by what merges today: the U4e brief's 28-row recheck and rev 3 draft
  and a U4d2 builder brief carrying N2-M1 and N2-M2 (both need U4d merged); a skeptic check of U4a and U4b's merged
  code; one of U4c's.

**Open questions for Mustafa:** none.
- **Answered 2026-11-11 about 11:50 UTC** (history.md, "Decisions taken", verbatim):
  - NIGHT-BUDGET (a): leave it; switch to (b) (no new agent work after about 21:00 Amsterdam) on the first night whose
    queue does not fit, and tell him when.
  - N2-M1: fix the estimate scene before U4d2 pins the recordings; the 9.5 floor does not move (carry 52).
  - SEC-U4d-R2-1: deferred to U4e in writing (carry 51); no push for a Low.
  - U4j's F1: the review budget as is; open after round 3, stop and bring it to him.
  - Every report gives the session's context figure: 622,250 of 1,000,000 tokens (62 %) at 11:50 UTC, compacting at
    about 901,600 (get_session, context_usage).
- **Relayed 2026-11-11 about 10:45 UTC** (it had not been sent before: no earlier reply carries it): the advisor's
  answer to TCP-FREEZE's "Tell me if any later unit plans work on the model" (advisor-u4-wave1.md:546-554: none plans
  fidelity work; U6's send path and U8/R1's DelayStream self-test work on the wrapper's timing, outside the freeze),
  and its "For Mustafa" item 2 (the readings PR before R5 carries three more readings).
- **Answered 2026-11-10 about 20:40 UTC** (history.md, "Decisions taken"):
  - NETEM-BAR (a): the bar stays capped as WS-BAR says; a WebSocket row that fails because the kernel is slower than
    the model is reported to him as the kernel's, with the netem numbers, and he decides.
  - NS-START: he sets the routine to `10 0-7 * * *` himself tonight.

**Rules in force** (his words, verbatim, in history.md's "Decisions taken" unless marked)
- **Mustafa, 2026-11-11** (history.md, "Decisions taken"): STATUS's "In flight" table keeps one line for #40; his
  words move between docs by script and line ranges, never by retyping (done for PR #49 at 5ca77eb).
- **Mustafa, 2026-11-10** (history.md, "Decisions taken": his seven points and his "A"):
  - RESUME-CARD; WIP-CLEAN; SIZE-CHECK;
  - TCP-FREEZE: no more model fidelity work; if U8's netem check disagrees, netem wins;
  - SQUASH-40 with SQUASH-CITES (a): the closing checklist under Next steps;
  - PROMOTE: at U4's close; against SPIKE-SCOPE (a);
  - the second lane: merge main as usual, by the plain merge the guard allows, as at df8975b.
- **Mustafa, 2026-11-09:**
  - the night shift's role: post the next night items to issue #16 each evening and fold them in the morning;
    building and testing stay by day;
  - one shared Cargo target for every copy, `.gate/shared-target` (touch before builds, check the counts);
  - LEFTOVER-DELETE: "leave them"; no delete past the deny list.
- **Standing:**
  - write files with Write and Edit, never cd plus a redirect;
  - never switch the main folder; the guard merges only origin/main into it;
  - CHECKPOINTS.md;
  - the review budget and one severity scale (kit-v3-4);
  - Opus at xhigh for netcode;
  - the advisor reads main's advisor.md.
- **U4:**
  - the five briefs (U4e, U4r, U4f, U4g, U4h) go to the advisor before their builds;
  - carries word for word from u4s-carries.md, listed as met by number in each report;
  - the wave-1 seam freeze (u4-build-plan.md, Lanes).

**Pointers**
- The plan: plan.md (rev 3.3).
- U4: u4-build-plan.md, advisor-u4.md, u4s-carries.md, u4s-report.md, and Running work below (U4 era).
- History: history.md (Steps done U0-U3d; the old Current step; Decisions taken; Running work up to U3c; "Running
  work, U4 wave 1": the card's blocks of 2026-11-10).
- The closing checklist for PR #40: under Next steps.
- Resume commands (scripts under /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/;
  a resumed run keeps its run id, and finished agents replay from cache):
  - U4c's round 2: `Workflow({scriptPath: <scripts>/npt-u4c-fix-round2-wf_610486f7-cc1.js, resumeFromRunId:
    "wf_610486f7-cc1"})`.
  - U4d's fix pass: `Workflow({scriptPath: <scripts>/npt-u4d-fix-wf_a8ff8550-67d.js, resumeFromRunId:
    "wf_a8ff8550-67d"})`.
  - U4j's review: `Workflow({scriptPath: <scripts>/npt-u4j-review-wf_1ea8a0dd-fee.js, resumeFromRunId:
    "wf_1ea8a0dd-fee"})`.
  - A cut agent runs again with its context only if its prompt changes: add a dated RESUME line that points at its
    notes file.
  - U4j's builder: SendMessage to a3b8dfeb01ed52b50.
  - If a worktree is gone: a new one at its base, with its wip patch applied first.

## Running work, U4 wave 1 (3): the resume card's U4j blocks at U4j's merge (moved 2026-11-11 about 20:15 UTC, by card_refresh3.py)
From "The three cut workflows", item 1 (card lines 33-62):

1. **U4j, the judge and marks change** (task wgj6zbosy; run wf_0906017a-a9b; script npt-u4j-fix-round2-wf_0906017a-a9b.js).
   - Done: round 1 (three REVISE; F1 High, the FAR BETTER cap hole); the advisor's rulings (advisor-u4j.md: FX-1 to
     FX-14, FU-1 to FU-6, the carries); **the fix pass, all 14 fixes**, each test shown failing first. The fixed build
     is uncommitted in `.claude/worktrees/agent-a3b8dfeb01ed52b50` (base 411eeee; 21 changed or new entries; notes
     there: .checkpoints/netcode-proof-test/u4j-build.md, "Fix pass, round 1"; runner u4j-fix/run.sh). Round 2 diffs
     against the pre-fix copy `.gate/u4j-prefix`.
   - Round 2 (netcode-architect, security-auditor, test-engineer) **resumed about 17:00 UTC** under USAGE-SAVINGS:
     the reviewers start from the review packet u4j-round2-packet.md (cut by review_packet.py from
     u4j-round2-packet.json; the fixtures' diff in u4j-round2-packet-fixtures.diff); the fixer's report is out of the
     prompts (its notes stay readable); the two readers run in parallel, then the test-engineer; every test runs
     through u4j-fix/run.sh, pass or fail by its exit code and the runner's summary line. The fixed build is saved as
     wip/u4j.patch (4ad80a9: 64 files on 411eeee, applies cleanly), with its notes and runner.
   - On its return: the main session reads every round-2 finding in full (U4j is the judge and marks change, so all of
     it touches the judge or the marks: his limit).
   - **Round 2 done (about 17:45 UTC; read in full):** all three APPROVE WITH FINDINGS; F1 closed by each (the
     test-engineer by runs and the advisor's three plants), so no round 3; the counted check.mjs run: all 10 steps,
     measured 317 = 286 + 31, bites 340 = 309 + 31, self 48; the 66 files' hashes match. New Mediums: NC-U4J-R2-N1
     (the writer rule for a tick written lost under L1's reorder: a carry sentence for U4e and U4r), TE2-1 (no test
     pins the LG part of the Arm rule's key), TE2-5 (outside U4j: an npt-net test failed once on the bites build and
     was retried until green; check.mjs hides failing names). Lows: NC N2-N6, SEC-R2-N1 and N2, TE2-2 to TE2-4, TE2-6.
     The advisor rules on routing and the merge (advisor-u4j-r2.md); the merge follows its ruling.
   - The fixer's points for round 2 and the advisor: SERIES's twin_hash row still says "any role" (the advisor's
     words needed); FX-3 (c) read stricter (two practice runs in force make a bite run INVALID); FX-10's malformed
     entries and its order of outcomes; FX-9's plant T also raised FX-1's new guard to 1.12 (round 2 plants the same).
   - Then: F1 still open after round 2 → a round 3 for F1 alone; **open after round 3 → stop and bring it to
     Mustafa** (his words). The merge: snapshot by `python3 -I .checkpoints/netcode-proof-test/worktree_patch.py
     .claude/worktrees/agent-a3b8dfeb01ed52b50 .checkpoints/netcode-proof-test/wip/u4j.patch u4j-`; stop the saver;
     `git apply`; check.mjs counted and the gate; commit with `git rm` of wip/u4j.patch. With it: §4.1's U4j public
     items; the builder's readings (a)-(f) from advisor-u4j.md R3 into the notes; the orphan reading (R5); PR #40's
     report names FU-1 to FU-5 as open security findings. Then U4c's L-c8 judged re-run on U4j.

From "Next steps, in order", item 1 (card lines 104-111):

1. U4j: round 2 done, no round 3 (F1 closed). **The advisor's round-2 ruling is in (advisor-u4j-r2.md, about 18:15
   UTC): APPROVE, U4j merges now, on its conditions M1-M9.** Next, after Mustafa's /compact: read the ruling in full (it
   touches the judge), then merge as it rules: check the worktree against round 2's hashes and that the branch has not
   touched judge, log or marks since 411eeee; apply; check the 64 files' hashes; check.mjs counted (it expects measured
   396, bites 443, self 52) and the gate; a failing test is never retried into green without its name; two commits in
   one push (the merge with the reviewed bytes, the notes and plan text, wip/u4j.patch removed; then its T1-T9 text
   edits verbatim). With it: carries 61 (§U4e), 62 (§U4r), 63-65 and C-U6-9 to C-U6-11 as it words them; the readings
   (a)-(f) and the orphan reading into u4j-build.md; PR #40's report names FU-1 to FU-5 and FU-7.

## Running work, U4d-F: the resume card's blocks at U4d-F's merge (moved 2026-11-12 about 01:55 UTC, by card_refresh4.py)
From the card's header and "The current step" (as they were):

- **Last update:** 2026-11-11 about 20:15 UTC (U4j merged: 4d2b0ef and d50396a, pushed together; the chat at 96,748
  of 1,000,000 (10 %) at 19:58 UTC, after Mustafa's /compact)

**The current step: U4 wave 1's close and wave 2's preparation.** Plan files, with the advisor's text batch of
2026-11-11 and U4j's merge text applied: plan.md (rev 3.3, sha256 2bef127b…), u4-build-plan.md (rev 2, 4b8eaab6…),
u4s-carries.md (carries 1-65, 32acca91…); the advisor's rulings: advisor-u4.md, advisor-u4-wave1.md (with B1.4's
erratum), advisor-u4j.md, advisor-u4d-close.md, advisor-u4j-r2.md.
- **Merged into the spike branch:** U4s f080551, U4b 411eeee, U4a 952f734, U4c f3f7c2f, U4d with W1-S1's npt-proto half
  39f4290 (report u4d-report.md); CI green on each. Main merged in at 5ca77eb (PR #49). **U4j: 4d2b0ef (the merge)
  and d50396a (round 2's wording), pushed together** (u4j-build.md, "Merge into the spike branch"). At d50396a:
  check.mjs 10 steps, measured 396, bites 443, self 52: the base counts for the next lanes, U4e's `<base>` included.

From "Running" (as it was):

- **Running:**
  - Done: the advisor's check of the U4e brief (advisor-u4e-brief.md, complete on disk: all seven blocks and the end
    sections; the usage limit cut the agent after it wrote them, before its summary); folded as rev 3.3 (3088bb5).
  - **U4d-F's run A, the build: DONE** (run wf_3dbc5af1-0d4, task wcj2e504g, 23:07 UTC; all four groups read in full;
    none stopped). Snapshot 9dd037a: wip/u4df.patch (5 crates/sim files on dad9566; `git apply --check` clean on
    .gate/u4df-prefix) and u4df-build.md. check.mjs twice, counted, all 10 steps PASS: measured 367, bites 414, self 52
    (+2 / +2 / +0 on 365 / 412 / 52). `aim_one_in` 40, with 3b's re-home (the drift check failed after step 3).
    t_m1's bands read against B1.2 step 4 by the main session: all equal (shots 68.4-75.6, events 9.5-10.5 over the
    window and each half, live 30.4-33.6, spawns 20.064-22.176, at least 48 and 15; the hull bound 1,400 in D-6's
    rounded-down form, the exact maximum 1,400.6 cm on the 1 cm grid).
    - The main session's readings (not in the packet): **TE2-5:** t_m1 failed in steps 2-5 by the plan's own design
      (B1.3: "both move the event rate"; steps 5 and 6 fix it), deterministically, named, re-run at once, recorded
      tagged TE2-5; the build brief's stop clause was stricter than B2.3's merge stop; explained, and the final runs
      pass, so no stop. **rust-reviewer:** not triggered (B1.2's "only if step 1 changes a World line beyond moving the
      speed rule": run_command's lines moved into `move_velocity`; the bite's `speed *= 1.01` became `gain = 1.01` in
      the same product, bit-identical; D-7 unchanged at step 1).
  - **U4d-F's run B, the review** (round 1 of its own budget; run wf_8a12a843-56b, task wmoh0xxq9, started about
    23:12 UTC; script workflows/scripts/npt-u4d-f-review-wf_8a12a843-56b.js; if cut, resume it with
    `Workflow({scriptPath, resumeFromRunId})`, and a cut agent continues after its notes' last line): packet
    u4df-packet.md (cut 23:12 UTC, sha256 85f8f86931b1, carries left out 65); netcode-architect (its type, read only;
    notes u4df-review-netcode.md), then the
    test-engineer (a general-purpose agent with the Edit tool, B6's first form, reading main's test-engineer.md; notes
    u4df-review-test.md) proves: plants O1-O3, P1, P2 by Edit, the three-seed test, check.mjs once counted. On return:
    read every Medium or higher in full; the fix pass; round 2 on what changed; then the merge (saver stopped first).
    - **netcode-architect: APPROVE WITH FINDINGS** (23:31 UTC; read in full by the main session): no Critical or High.
      NF-M1 Medium, not before merge, for the advisor: the drift check (each 300 s half in 9.5-10.5) fires on noise
      at ES's stationary level, 9.88 over 36 min and 9 seeds; out of sample, seed 6's second half is 9.446; so the
      advisor rules on the report's wording and on how a later re-measure judges drift (the band does not move).
      NF-L1 to NF-L4, Low: carry lines (no ES byte figure from the synthetic recording; carry 59 names the -26 %),
      the report's numbers on 3b, and 3b's switch tied to `aim_one_in > 1` (a follow-up). Its slip: blocks [5]-[8] of
      its notes were first appended by a python one-off, then the file was rewritten whole with Write.
    - **test-engineer: APPROVE WITH FINDINGS** (23:56 UTC; read in full): every claim proven by its own plant or run
      (O1-O3 by name; P1 in the base's own lines fails t_m1's drift check, window 9.846 in band, halves 9.240 and
      10.453; P2, a different hull, the bound; the three-seed table equal; one cause per re-pin; check.mjs 367 / 414 /
      52; the copy byte-identical at the end). TF-M1 Medium, not before merge, for the advisor: the counted suite
      guards the reckoning only by t_m1's one seed, inside one half's noise; D-7's digest holds it for certain; a
      deterministic reckoning test is proposed (outside B1.2's symbols). TF-L1 to TF-L3 Low. NF-M1 confirmed by run.
      Same slip as the netcode-architect's (two blocks first written by python, then the file rewritten whole).
  - **The advisor's ruling on round 1: APPROVE WITH CONDITIONS** (advisor-u4df.md, 00:24 UTC 2026-11-12; read in full
    by the main session; brief advisor-u4df-brief.md). Nothing for Mustafa. R1: 40 stays; RM (carry 67) for later
    re-measures (9 seeds, 36-minute runs, every window in band, drift on the nine seeds' mean per 300 s block); its
    erratum on B1.2 step 4's "stationary". R2: TF-M1's test lands before the merge as FX-1 (replay.rs's new test
    module, plants Q1-Q5), with FX-2 (scene.rs docs, T3's words). R3: the Lows routed (carries 68-72, carry 59's
    sentence, T1's report words). R4: both readings confirmed, two corrections of fact (t_m1 failed in steps 2-4; the
    re-runs were in steps 2, 4 and 5). R5: the merge on M1-M9 after round 2. R6: T4 (now in Rules in force).
  - **The fix pass: DONE** (run wf_6d45d257-7c0, 00:57 UTC; read in full; not stopped). FX-1
    (`replay::tests::the_sky_aims_reckon_every_input_the_recording_emits`) passed on the real code at its first run
    in both builds (1.75 s), all 954,628 sky aims of seed 2 exact, its six counts above 0 (the rarest 11); Q1-Q5 each
    fail it by name; FX-2 is T3's words; check.mjs twice 368 / 415 / 52; the three-seed lines identical; world.rs and
    both test files equal to round 1's. Snapshot 9bf615c (wip/u4df.patch, applies on dad9566). Final sha256:
    replay.rs 802842b0…, scene.rs a5ce8ac2…, the other three as round 1's.
  - **Running: round 2** (run wf_18a25138-5d9, task wdzysc1jw, started 00:58 UTC; script
    workflows/scripts/npt-u4d-f-review-r2-wf_18a25138-5d9.js; packet u4df-r2-packet.md, cut by script, sha256
    7d8a4113f71b, 65 carries left out): the netcode-architect (read only; notes u4df-r2-netcode.md), then the
    test-engineer (general-purpose with Edit; notes u4df-r2-test.md; start hashes .gate/u4df/r2-start.sha256 for M1
    and M4; check.mjs once counted). APPROVE or Lows only: merge on M1-M9 with no further ruling; a Medium or above,
    or a REVISE: the advisor first. The merge's texts: u4df_merge_text.py (T1 and T2 by script; dry run clean), run
    with --apply after u4df-report.md is written with its anchor line.
  - Done meanwhile: TE2-5 (b), ten of ten runs exit 0, each 443 passed, 0 failed, 12 ignored (te2-5-record.md).
  - the saver, restarted after U4j's push (about 20:15 UTC, 110 minutes): `node .claude/scripts/checkpoint.mjs watch
    netcode-proof-test --trailer "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" --trailer "Claude-Session:
    https://claude.ai/code/session_0173vVo6igCNyQYBWPvhH36A"`. Stop it before a merge's apply and restart it after the
    push, as at U4j's.

From "The three cut workflows" (as it was):

**The three cut workflows, and what to do with each when the hold lifts.** Scripts are under
/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/. A resumed
run (`Workflow({scriptPath, resumeFromRunId})`) keeps its run id; finished agents replay from cache, and only the cut
ones run again.
1. **U4j, the judge and marks change: MERGED** (4d2b0ef, the merge; d50396a, round 2's wording; pushed together;
   run wf_0906017a-a9b finished). Round 1 (three REVISE; F1 High), the fix pass (FX-1 to FX-14), round 2 (three
   APPROVE WITH FINDINGS; F1 closed by all three; no round 3), the advisor's round-2 ruling (advisor-u4j-r2.md:
   APPROVE on M1-M9), then the merge: M1 66 OK, M2 true, M3 64 files (the snapshot equal to 4ad80a9's), M4 64 OK, M5
   and M8.2 measured 396, bites 443, self 52, M6 no failure, M7 gate PASS with --no-smoke (this container's Chromium
   does not match tools/'s Playwright). The record: u4j-build.md, "Merge into the spike branch" and "Readings, as
   ruled". The old block: history.md, "Running work, U4 wave 1 (3)".
   - Open from U4j, as routed: carries 61 (§U4e), 62 and 63 (§U4r), 64 (the next change to crates/judge), 65
     (§U4d2, or the main session sooner); C-U6-9 to C-U6-11 (plan.md's U6 row); the FU list for PR #40's report (the
     closing checklist); TE2-5 (b) (next steps; L-c8's re-run done, 9cd7c39); one more reading only if U4e's client
     drops stale groups (advisor-u4j-r2.md B5).
2. **The U4e builder brief, rev 3** (task wnwulchr5; run wf_c92cd986-072; script npt-u4e-brief-r3-wf_c92cd986-072.js).
   - Done: the draft, u4e-brief.md (719 lines; notes u4e-brief-r3-notes.md): W1-U4E's answers, W1-S1, W1-N2, W1-TARGET's
     lock, the carries re-quoted by script, 8 of rev 2's 28 recheck rows changed and R29-R38 added, two new questions
     (U4E-Q12, who plants -22/unbounded; U4E-Q13, the owner-to-all bite's id).
   - Resumed about 17:55 UTC (task wwyzh9w7f) for the test-engineer's check (u4e-brief-r3-check.md), then one
     revision. Before it, the main session folded the brief by script: 3.1 re-copied from u4s-carries.md §U4e (carry
     51's corrected span, carries 53-57), 3.2 rewritten to record advisor-u4d-close.md's rulings, 7.2 item 3 done, a
     note under the title. The check's prompt no longer carries the drafter's summary; a SINCE note lists what changed
     after the draft (the draft call is unchanged, so it replayed from cache).
   - Then: fold advisor-u4d-close.md's U4e carries 53-57 (with the text batch) into the brief's carries section; the
     advisor checks the brief and answers Q12 and Q13; fill `<base>`; U4e's build. U4e need not wait for U4d-F.
3. **U4d-F, the estimate scene fix** (task wg0l1ugq1; run wf_9ce88fa3-452; script npt-u4d-f-wf_9ce88fa3-452.js).
   - The approved plan: advisor-u4d-close.md B1.1-B1.3 (steps 0-7; the files and symbols allowed; the measurement on 3
     seeds as the acceptance). No setting works → stop and bring the numbers to the advisor; the 9.5 floor never moves.
   - Cut early: its builder ran about 9 minutes in `.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build at
     dad9566). Its notes (.checkpoints/netcode-proof-test/u4df-build.md there) hold the plan and an empty Progress;
     world.rs has an uncommitted partial edit.
   - **Do not resume the run as it is:** its builder uses `isolation: 'worktree'` and would start again in a fresh
     copy. Edit the script: drop the isolation; the builder works in `.claude/worktrees/wf_9ce88fa3-452-1`, reads `git
     diff` there first, completes or undoes the partial edit by Edit, and continues from its notes. Then the review
     (netcode-architect, test-engineer) as scripted.
   - Saved: wip/u4df.patch (4ad80a9: world.rs on dad9566, applies cleanly) and u4df-build.md. Under USAGE-SAVINGS the
     builder runs as fresh copies, one per step group of B1.2 (0-1, 2-3, 4-5, 6-7), each reading its notes and the
     diff in the worktree before it changes anything; pass or fail by the exit code and the test runner's own summary
     line, never a grep; long output to files under .gate/. Its review starts from a packet cut by review_packet.py.
   - It merges before U4d2's brief is final; U4d2 pins only U4d-F's recordings (carry 52).
   - **The new route (2026-11-11 about 20:30 UTC), in place of resuming wf_9ce88fa3-452:** run A, the build: a new
     workflow, four fresh general-purpose builders (opus, xhigh) in turn in that worktree, no isolation, one per step
     group (0-1, 2-3, 4-5, 6-7 with the End checks), each reading the notes and the diff first; every cargo and
     check.mjs run through u4df-run.sh (the lane lock, shared, or `-x` for counted, check.mjs and bite runs; logs under
     .gate/u4df/); TE2-5's rule and B6's plant rule in its brief. Then the main session cuts the review packet
     (review_packet.py with a u4df-packet.json, diffs against the base copy .gate/u4df-prefix, a worktree at dad9566).
     Run B, the review: netcode-architect read only, then test-engineer proves, from the packet; their agent types have
     no Edit tool, so each rewrites its notes whole with Write and saves each planted file's diff beside its run (B6).
     Run A starts when TE2-5's ten runs end (a builder waiting on their exclusive lock could hit its Bash timeout).

From "Next steps, in order" (as it was):

**Next steps, in order (Mustafa: "follow the card's order: U4j's round 2, then the U4e brief's check, then U4d-F"):**
1. **U4j: MERGED** (4d2b0ef, d50396a; about 20:10 UTC), M1-M9 met. **U4c's L-c8 judged re-run: done** (9cd7c39;
   u4c-lc8-rerun.md): U2's script at 6586991; the five L1 parts PASS with every check's outcome and value equal to
   U2's; the four bites fail exactly U2's checks; the hash half holds through U4c's scratch (equal whole-v2-log
   hashes), since npt-lab still records only the whole log's hash (U4c's netcode F2, Medium, open; a carry for U4d2,
   crates/lab's lane in wave 2, goes to the advisor with the U4e brief's check). **TE2-5 (b): done** (advisor-u4j-r2.md
   B2.3; te2-5-record.md): the bites build's workspace tests ten times with `--no-fail-fast` (te2-5-runs.sh), 20:20 to
   20:41 UTC: every run exit 0, 443 passed, 0 failed, 12 ignored. None failed in ten; the cause stays unknown, and
   TE2-5's rule stays until carry 65 is met or a failure is caught by name.
2. The U4e brief: **check and revision done (about 18:35 UTC; read in full).** The check said FAIL with 3 High (F1: the
   main session's fold reached 3.1 and 3.2 only, not e12, 2.6, the return list or carry 51's route; F2: the bite
   proofs as worded clash with a green check.mjs, so wave 1's two-part form is stated; F3: the owner-to-all proof was a
   group check that cannot fail on the open route, so carry 53's per-record test is the proof), 5 Medium (F4 carry
   22's accepting edges; F5 e7's exercise counts; F6 the three room-side checks' rules and clean negatives; F7 e5a's
   gap alignment; F8 routes for carries 54 and 56) and 5 Low. The revision made rev 3.1 (893 lines): all 13 fixed,
   none rejected; a new question U4E-Q14 (does a 1 s gap still evict B on Loopback; recommended (b)). Rev 3.2: U4j's
   merge folded (u4e_fold2.py: 3.1 with carry 61; R27, R28, R38 at d50396a). **The advisor's check of rev 3.2
   (advisor-u4e-brief.md, read in full): APPROVE on L1 to L4**: four Mediums (A1 carry 61's test line; A2 e7's split
   clause; A3 owner-to-all within the section's limits; A4 every encoder error kind) and three Lows (A5-A7), each with
   words; U4E-Q12 (c) (U4e plants -22/unbounded; U4f runs it judged), U4E-Q13 (a) (`catalogue-5/owner-to-all`),
   U4E-Q14 (b) (the Loopback gap is `npt_sim::ACK_RING_TICKS` tick periods); F2's route: carry 66 for U4d2. **L1 and
   L2 done (3088bb5, u4e_fold3.py):** the brief is rev 3.3; the plan's four edits; carry 66. Next: **L3**, the
   carry-51 commit (`Scene::effect_rows()` and its §4.1 line), after U4d-F's merge, since both touch scene.rs; then
   7.2 item 1's counted run gives `<base>`'s counts; U4e's review reads its diff from the commit before the carry-51
   commit; then **L4**, the launch as section 1 says (one builder, general-purpose, opus, xhigh, in a worktree, under
   W1-TARGET's lock; at most three agents counting reviewers).
3. Then U4d-F (workflow 3) as above; then U4d2's builder brief (carrying carry 52, N2-M2's ruling, W1-TARGET), with
   the advisor.
4. U4r's brief: the night's rev 2 at `cs/npt-u4r-brief` ed973c9 goes to the advisor after U4c, U4d2, U4e and U4j merge.

## Decision: Mustafa, 2026-11-12 about 15:32 UTC, after his /compact (SPEND-LIMIT (a), review_packet.py, main)
- **Mustafa, 2026-11-12 about 15:32 UTC, after his /compact (verbatim):** "You were just compacted to cut usage. Read
  the resume card in .checkpoints/netcode-proof-test.md first and continue from its next steps. Do not redo finished
  work.
  My words of 2026-11-12.
  1. SPEND-LIMIT (a). The cap is my university's, so it stays as it is. Restore and resume after each cut, as you did
     this morning. At every cut, write on the card the notice's exact text, the time of the cut and the time you
     resumed.
  2. review_packet.py. Your change in 1829b5f lets every file whose name starts with "advisor-" pass the name check.
     Four of those files are your own briefs to the advisor: advisor-u4e-brief-brief.md, advisor-u4df-brief.md,
     advisor-u4j-r2-brief.md and advisor-u4e-r1-brief.md. My rule is that the packet holds no conclusion of the builder
     or the main session.
  Before the next packet is cut, replace the "advisor-" exception with a list of exact file names that may pass,
  starting with advisor-u4e-brief.md alone. Every other name the pattern forbids stays refused. You may add a file's
  exact name to the list yourself when a packet needs it, only if the advisor agent wrote that file as its ruling, and
  you note each addition in history.md. A file written by you or by a builder never goes on the list.
  The packets already cut stay as they are. Do not re-cut them and do not restart a review.
  3. Main moved. PR #51 (foundation step 2.1b, the legacy game into legacy/) merged as
     2aa5e9ce03e9ab9ccfecd53e6aabf951abe705dd. Merge main into your branch at your next clean stop. Do not stop a
     running agent for it. docs/DECISIONS.md and docs/STATUS.md will conflict: keep both sides, as you did for PR #49.
  Record these words in history.md under my decisions. Tell me in a few lines what the last limit notice said and that
  the script is changed. Then carry on."
  - Read as (the main session):
    - SPEND-LIMIT (a): the cap stays. After each cut the main session restores what a cut agent left (a plant, a half
      edit) and resumes, as this morning; the card's block "Cuts by the spend limit" gives, for each cut, the notice's
      exact text, the time of the cut and the time of the resume.
    - review_packet.py: the "advisor-" exception gives way to ALLOWED, the exact paths that pass though the pattern
      forbids them: advisor-u4e-brief.md alone. Every other name the pattern forbids is refused: the main session's four
      briefs to the advisor, and advisor-u4d2-brief.md (the advisor's ruling, not needed by a packet yet). A file goes
      on the list only if the advisor agent wrote it as its ruling and a packet needs it; each addition is noted below.
      The packets already cut stay as they are; no review restarts.
    - Main (PR #51, 2aa5e9c): merged into the branch at the next clean stop, no running agent stopped; docs/DECISIONS.md
      and docs/STATUS.md keep both sides, as for PR #49.
  - The main session's own addition, which only refuses more: the flag for his words (`words_of_mustafa`) now passes
    history.md only; before, any flagged path passed the name check.
  - Additions to ALLOWED (the exact name; that the advisor agent wrote it as its ruling; the packet that needed it):
    none yet.
  - Done: review_packet.py changed (sha256 784cfedf…). packet_name_check.py shows its name check on 11 paths (logs
    .gate/packet-name-check/): the new script 11 of 11 as wanted; the script before the change (153b76ea…, 1829b5f's
    rule) 5 of 11, the four briefs, advisor-u4d2-brief.md and a flagged report passing. The card's block "Cuts by the
    spend limit" holds today's three cuts (02:02, 06:41 and 09:24 UTC), read from the session's transcript. This entry
    is appended at the end of history.md under its own heading, and "Decisions taken" points here from its heading line,
    so no cited line of history.md moves.
  - Done (item 3, the main session): main (2aa5e9c, PRs #50 and #51) merged into the branch at bfda3cd, no agent
    stopped; the two conflicts kept both sides by merge_main_51.py (DECISIONS.md: main's 3-line pointer, then this
    branch's 22-line block of the 2026-11-06 answers; STATUS.md: this branch's #40 row, its state brought up to date,
    then main's #51 row). Checks: rules, CI and gate files equal main's; repo-map check OK; check.mjs --only
    marks,refs,redact all pass; gate PASS --no-smoke.

## Note: an addition to review_packet.py's ALLOWED (Mustafa's words of 2026-11-12, item 2), 2026-11-12 about 20:15 UTC
- Addition 1: `.checkpoints/netcode-proof-test/advisor-u4d2-brief.md`, the exact path.
  - Written by the advisor agent as its ruling: its title "The advisor's check of the U4d2 builder brief (rev 1.1),
    2026-11-12"; its seat line "a day session's advisor (workflow agent, opus), on main's `.claude/agents/advisor.md`".
    The run: workflow wf_6f64e718-539 (npt-u4d2-brief-advisor), one agent, "advisor:u4d2-brief", general-purpose,
    opus, reading `git show origin/main:.claude/agents/advisor.md`; completed 13:29:54 UTC. The file's last commit is
    the saver's 90f2bfd (13:34:18 UTC); `git diff --stat 90f2bfd HEAD` on it is empty, so nobody changed it since.
  - The packet that needs it: U4d2's review round 1 (u4d2-packet.json). The build is judged against this ruling:
    U4D2-Q1 to Q12, findings A1 to A15 and conditions L1 to L5.
  - Not added: any brief of the main session's or a builder's. Every other name the pattern forbids stays refused.
- review_packet.py after the addition: sha256 7d887492…. packet_name_check.py now has 12 cases: case 6 wants "pass"
  for this file, and a new case 7 wants "refused" for advisor-u4e-r2-brief.md, the main session's fifth brief to the
  advisor. Result: 12 of 12 as wanted (.gate/packet-name-check/).

## Decision: Mustafa, 2026-11-13 about 11:31 UTC, after his /compact (CHECK-IN)
- **Mustafa, 2026-11-13 about 11:31 UTC, after his /compact (verbatim):** "You were just compacted to cut usage. Read
  the resume card in .checkpoints/netcode-proof-test.md first and continue from its next steps. Do not redo finished
  work.
  My words of 2026-11-13. Last night the work stood still from 01:22 to 08:25 UTC, although the limit reset at 03:50.
  From now on, while agents are running, keep one check-in scheduled about every two hours. A check-in only looks
  whether a cut stopped something, restores and resumes it by the card as SPEND-LIMIT (a) says, and ends. If you cannot
  schedule that, tell me. Record these words in history.md under my decisions."
  - Read as (the main session):
    - CHECK-IN: while any agent or workflow of this session runs, one scheduled check-in into this session comes due
      about every two hours; while none runs, it is paused, so there is never more than one. A check-in only looks
      whether a cut stopped something (the spend limit's cut; a container restart stops runs the same way, so it is
      looked for too), restores and resumes it by the card as SPEND-LIMIT (a) says (the notice's exact text, the time of
      the cut and the time of the resume on the card's block "Cuts by the spend limit"), and ends: it starts no new
      work. A check-in that comes due while the limit still holds is expected not to run; the next one comes due two
      hours later, so the work should resume within about two hours of the reset (not yet seen on a real cut).
  - Done: the scheduled task "npt check-in (every 2 h while agents run)" (trig_01EKqNqwZrQTVJ6caAyRrug5), bound to this
    session, created 11:34:38 UTC, schedule `34 */2 * * *` (UTC: 12:34, 14:34, and so on; the server set the minute from
    the time it was made). Its prompt holds three steps: look whether a cut stopped something; if nothing did, one line
    and end; if something did, restore and resume it by the card, the cut recorded, the saver restarted if it stopped,
    the card saved, then end. It is paused while no agent runs and enabled before each launch (the card's saver line
    says so). The schedule could be made, so there is nothing to tell him about it.

## Decision: Mustafa, 2026-11-13 about 18:14 UTC (MAIN-48)
- **Mustafa, 2026-11-13 about 18:14 UTC (verbatim):** "My words of 2026-11-13. Main moved again: PR #48 (foundation step
  2.2, the toolchain) merged as 4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300. Merge main into your branch at your next clean
  stop. Do not stop a running agent for it. docs/STATUS.md will conflict: keep both sides and stay within its 120 lines.
  Main now has a Rust workspace at the root and a new CI job named toolchain, which will also run on your pushes. Your
  spike keeps its own workspace. Tell me in two lines when main is in and whether the toolchain job is green on your
  branch. Record these words in history.md under my decisions."
  - Read as (the main session):
    - MAIN-48: main's PR #48 (foundation step 2.2, the toolchain) is merged as 4d21c1a. Main (origin/main, 4d21c1a or
      later) is merged into this branch at the next clean stop: the point where no step is half done, never by stopping
      a running agent. Here that is after L4's script returns and its commit, before L6's launch, so U4r's builder
      starts on the merged state. docs/STATUS.md's conflict keeps both sides, no line retyped, within its 120 lines (as
      at the PR #49 and PR #51 merges). The spike keeps its own workspace (spikes/netcode-proof/Cargo.toml); main's root
      workspace and its CI job `toolchain` are main's, and the job also runs on this branch's pushes. The report to him:
      two lines, when main is in (the merge commit) and whether the `toolchain` job is green on this branch.
