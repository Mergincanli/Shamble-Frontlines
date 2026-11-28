# infra-engineer: U8 build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22): draft U8's build brief rev 1 to `.checkpoints/netcode-proof-test/u8-brief.md`.
(No Edit tool in this agent: this file is rewritten whole with Write at each block.)

## Plan: DONE
1. Memory: none for infra-engineer under .claude/agent-memory-local/. DONE
2. Sources read (replan.md, replan-units.md §3 §5 §7, plan.md:1256 §7.2-7.3 §8 §10.5-10.6 §12.2-12.3, security-r1-deps.md, advisor-u6a-brief.md, u6a-brief.md, history.md:7561-7607). DONE
3. Layout facts, container tools. DONE
4. Dates arithmetic. DONE
5. u8-brief.md written (368 lines; R1 table sum 3.55 checked; size parts sum 2,160; lane sums 7,410-11,120 checked). DONE
6. Hand back. DONE

## Notes (one line per finding)
- N1 replan.md:19 U8 1,360 (1,850-2,050); :27-39 runs; his part per run.
- N2 replan.md:89-94 order after R1: MOVED, R1 report, U4k, U6d, U6e, U6j; R2, R3, R5.
- N3 replan-units.md §5 :151-171; §7 :194-205 (R1 Wed 2 there; he chose Thu 3, history.md:7604).
- N4 replan-units.md §3 :107-121 DNS-01 by hand.
- N5 STALE plan.md text superseded by RE-PLAN-OK (4): :898, :914, :923, :961-964, :983 (http-01/revoke), :909-911 (ports from everyone), :819 (Safari, five browsers).
- N6 plan.md:1256 U8 row items; netcode checklist and W-PR cut.
- N7 R1 hours: 2.95 - 0.30 + 0.25 + 0.20 + 0.15 + 0.30 = 3.55 <= 4.
- N8-N9 plan.md §12.3, Stop B :1268-1281.
- N10 security-r1-deps.md SEC-R1D-2/-3/-5/-6, webdriverio 10.2.0 standalone, binding :256-257.
- N11-N13 advisor-u6a-brief.md cond 2-4, Q-U6A-5 term; u6a-brief.md token file 0600, hold form, slices.
- N14 plan.md:178 ROUTE native Windows; :181 FIXED-IP (a); :182 DENSITY-FIGURE (a).
- N15 PR #41 merged 2026-11-06T13:35:37Z.
- N16 U6a builder launched Sun 22 (card :157).
- N17 container tools: util-linux Essential provides taskset; nft, visudo, PyYAML, node present; no shellcheck/cloud-init/tc/sshd.
- N18 tcp_model/tests.rs:1407 bursts_ns empty: S14 burst half lacks model side (Q-U8-6).
- N19 advisor-u3d.md:80-83 C-U8-5.
- N20 sources 2026-11-22: systemd.resource-control AllowedCPUs (cpuset, slices; runtime not stated: medium); kernel-per-CPU-kthreads (medium-high); Launchpad #1993814 netem packaging (unconfirmed for 24.04 generic -> bring-up check).
- N21 redact-check allow-list.
- N22 dates: R2 Sat 12, R3 Sun 13, R5 Mon 14, spare Tue 15, close Fri 18 at middle; top -> ~Mon 21 unless lever (R2/R3 after U6e).
- N23 size ~2,150, likely 2,900-3,250 (re-plan 1,360 / 1,850-2,050), npm lock apart.
- N24 cross-unit findings: token file 0600 owned by room user unreadable by run script (Q-U8-7); C-U8-2/-6 per-leg reads must sit in U6c's probe (Q-U8-5); DNS polling split with U6c (Q-U8-4); user.slice vs tick vCPU (Q-U8-1).
