# Checkpoint: netcode-proof-test
- **Goal:** start the netcode proof test (SPIKE_PLAN item 1) as its own change-set; this step is the PLAN only, for the
  advisor's review and Mustafa's (no code before approval) · plan: `.checkpoints/netcode-proof-test/plan.md` (revision 2; revision 1 is 786b7d2)
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main `af8393e9d40f3bd382e77f86e6df2612785c706c`
- **Last update:** 2026-11-06 02:13 Amsterdam

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

## Current step
6. Plan review round 2 on revision 2 (one workflow, 3 agents at xhigh): security-auditor (its 19 fixes and the new
   surface), netcode-architect and infra-engineer (both deciders of OQ-0006-16, plus their lenses). Notes:
   `<agent>-plan-review2.md`.

## Next steps
5. Fix, then the advisor's plan review (APPROVE / REVISE / ESCALATE); iterate on REVISE.
6. Push, open the draft PR, report to Mustafa (Decisions needed, For Mustafa).

## Open findings
- Review round 1's 49 findings: fixed in plan revision 2 (§0.1); round 2 checks them.

## Key numbers
- Hetzner, excl. VAT, read 2026-11-05 (infra digest D): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h (plan prices include
  IPv4); 1 RS + 1 LG = 0.1562/h; each VM rounds up per session; RS<->LG traffic internal and free.
- Rust stable 1.99.1 (2026-10-13; static.rust-lang.org answers 200); container has 1.97.0, x86_64 target only.
- macOS runner images: macos-26 arm64 Safari 26.6.2; macos-15 arm64 Safari 27.2 (infra digest F).

## Decisions taken
- Settled inputs (Mustafa, this session's prompt, 2026-11-05): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms
  round trip, +-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only; CCX23 (RS, x86) + CAX21
  (LG, Arm64) in Falkenstein, created and deleted by Mustafa per run; SSH by a public key the session gives him, never
  a Hetzner token; each run's plan states its hours and a cost cap for his OK; Safari leg tries GitHub's macOS runners
  first, any `.github/` change goes to him; shamblefrontlines.com at Cloudflare, one exact DNS record per test host,
  always "DNS only" (grey cloud).

## Running work
- Workflow `wf_e7cdd40f-712` (npt-plan-review2: security-auditor, netcode-architect, infra-engineer; xhigh) · script as
  launched: `.checkpoints/netcode-proof-test/npt-plan-review2-run1.js` (no args) · notes: `<agent>-plan-review2.md` ·
  launched 23:54 · FAILED 23:54: all 3 agents hit the usage limit at start (2 tool calls in all, no notes written;
  "your 5-hour limit resets 1:10am (UTC)") · resumed 02:13 Amsterdam with resumeFromRunId, same script, no args.
- Workflow `wf_88b642c1-384` (npt-plan-review) · script as launched: `.checkpoints/netcode-proof-test/npt-plan-review-run1.js`
  · launched 23:16 · DONE 23:37 (3 of 3 returned).
- Workflow `wf_1ae38d53-da9` (npt-plan-digests) · script as launched: `.checkpoints/netcode-proof-test/npt-plan-digests-run1.js`
  · launched 22:38 · DONE 23:00 (3 of 3 returned; journal in the session's workflow folder).
- Saver: background task `btpf0ptzw` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/btpf0ptzw.output`
  · started 22:35 · stopped 00:25 ('time is up').
- Saver: background task `be4kec0xf` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/be4kec0xf.output`
  · started 02:11 · stops by 04:01 (110 min) · last check 02:13: started.
