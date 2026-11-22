# infra-engineer: U8 build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22): draft U8's build brief rev 1 to `.checkpoints/netcode-proof-test/u8-brief.md`.
U8 = the run package without Safari: cloud-init, firewall, R1's run plan, operator files, wrappers, browser scripts.
No Rust; builds off the cargo lane beside U6a from Mon 23 Nov; needed by Stop B (~Tue 1 Dec) and R1 (Thu 3 Dec).
(No Edit tool in this agent: this file is rewritten whole with Write at each block.)

## Plan
1. Memory: none for infra-engineer under .claude/agent-memory-local/. DONE
2. Read replan.md, replan-units.md §3 §5 §7, plan.md:1256 + §7.2-7.3 + §12.2-12.3, security-r1-deps.md,
   advisor-u6a-brief.md, u6a-brief.md, history.md:7561-7607. DONE
3. Layout facts (plan.md §4.1 ops/ paths, §4.8 npm pins, ROUTE, FIXED-IP), container tools. DONE
4. Dates for R2, R3, R5, close with arithmetic. DONE (below)
5. Write u8-brief.md. IN PROGRESS
6. Hand back <=15-line summary.

## Notes (one line per finding)
- N1 replan.md:19 U8 1,360 (1,850-2,050); :27-39 runs R1 4/4 h 0.62/0.80, R2 11/11 1.72/1.90, R3 10/10 1.56/1.80, R5 17/56 3.34/3.60; his part: VMs, home address in firewall, one TXT per order (two in R1, ~2 min each), ~2 h present in R1.
- N2 replan.md:89-94 order: after R1 MOVED first, then R1 report, then U4k, U6d, U6e, U6j; R2, R3, R5. Stop B = U8's one round from U6c's merge.
- N3 replan-units.md §5 :151-171 U8 stays/goes, size split. §7 :194-205 day table had R1 Wed 2 Dec; he chose Thu 3 Dec (history.md:7604).
- N4 replan-units.md §3 :107-121 DNS-01 by hand: npt-cert issue staging|production, TXT `_acme-challenge.rs.nt`, TTL 1 min, poll authoritative NS <=30 min, shortlived in prod, no revoke, fw-rs loses TCP 80.
- N5 STALE TEXT (not a contradiction): plan.md:898 (§7.3 step 9.1 revoke), :914, :923 (TCP 80), :961-964, :983 (http-01, revoke) superseded by replan-units.md:107-118 under RE-PLAN-OK (4) (history.md:7600). Also plan.md:909-911 game ports "everyone" superseded by replan-units.md:153-154 (home /32 + LG /32).
- N6 plan.md:1256 U8 row: S14, DelayStream loaded self-test (cond-10), ImpairSocket R1 self-test, C-U8-1..-6; row lists netcode checklist and W-PR: cut (replan-units.md:163-165).
- N7 plan.md:819 R1 hours 2.95 incl. Safari 0.3; re-plan +0.45 (-29 0.25, PM-0006-25 0.2: replan-marks.md:115). 2.95-0.3+0.45 = 3.10; + DelayStream self-test 0.15 + S14 0.30 (my sizing) = 3.55 <= 4.
- N8 plan.md §12.3 laptop session rules (operator settings, Manual mode, xhigh, redact-check pre-push).
- N9 plan.md:1268-1281 Stop B; W-PR part gone.
- N10 security-r1-deps.md:241-247, :256-257 webdriverio 10.2.0 standalone, SEC-R1D-2/-3/-5/-6, npm rules, U8 binding check.
- N11 advisor-u6a-brief.md:233-250 conditions 2, 3, 4; :82-92 Q-U6A-5: U8 confirms util-linux taskset on image.
- N12 u6a-brief.md:134-139 token file 0600, room ends by duration or stdin close; cond 3 hold form.
- N13 u6a-brief.md:189-190 U8 sets slices' allowed CPUs.
- N14 plan.md:178 ROUTE (a): laptop MSI Vector 18, native Windows + PowerShell, not WSL; Brave + Edge, Chrome/Firefox to install; :181 FIXED-IP (a) kept addresses, deleted at close; :182 DENSITY-FIGURE (a) answered.
- N15 PR #41 (R-PR, claude/npt-readings) merged 2026-11-06T13:35:37Z (gh api) - replan-units.md:222's open risk closes.
- N16 card (netcode-proof-test.md:150-158): U6a builder launched Sun 22 14:2x, a day early.
- N17 container: util-linux 2.39.3 Essential: yes provides /usr/bin/taskset (dpkg -S) -> on Ubuntu 24.04 image (high). nft 1.0.9, visudo, PyYAML 6.0.3, node 22.23.4 present; shellcheck, cloud-init, tc, ip, sshd missing.
- N18 tcp_model/tests.rs:1391-1419 kernel_cross_check_model: env NPT_KC_RTT_NS/MESSAGES/RUNS, `bursts_ns: Vec::new()` -> S14's 10 one-second bursts have no model-side knob: a Rust test change (not U8). results/u3c-kernel-check/{probe.py,compare.py,run.sh} = the rig to adapt.
- N19 advisor-u3d.md:80-83 C-U8-5 scope; DelayStream form plan.md:1256 E[(L-L0-0.25ms)+] < ~10us, p99 <= 1 ms.
- N20 Sources read 2026-11-22: systemd.resource-control AllowedCPUs= = cgroup v2 cpuset, applies to slices (freedesktop man page; runtime set-property not stated there: medium); kernel-per-CPU-kthreads.rst: bind non-per-CPU kthreads to housekeeping CPU, workqueue cpumask sysfs (medium-high); netem module packaging on Ubuntu 24.04 unconfirmed (Launchpad #1993814: kvm flavour lacks net/sched; generic has it) -> bring-up `modprobe sch_netem` check.
- N21 redact-check.mjs:106-124 built-in allow list incl 169.254.169.254, fe80::a9fe:a9fe, 1.1.1.1, doc ranges -> placeholders must be text or doc ranges.
- N22 DATES: lane after R1 likely 7,410-11,120 (MOVED 360-720, U4k 800-1,900, U6d 2,450-2,700, U6e 2,150-2,750, U6j 1,650-3,050); at 1,175/day 6.3-9.5 days, middle 9,265 -> 7.9 days; start Fri 4 -> U6j Fri 11 (middle), Thu 10 (low), Sun 13 (top). R2 Sat 12, R3 Sun 13, R5 Mon 14, spare Tue 15, close Wed 16-Fri 18. Top case without lever: close ~Mon 21. Lever: R2/R3 after U6e, judged when U6j lands.
- N23 size: my estimate ~2,040 (+lock apart) vs re-plan 1,360: adds RS run script, cond-4 CPU verb, S14 rig, QUIC-block verb, rooms scope wrapper; likely 2,750-3,050; far under 6,000.
