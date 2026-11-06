# Checkpoint: netcode-proof-test
- **Goal:** start the netcode proof test (SPIKE_PLAN item 1) as its own change-set; this step is the PLAN only, for the
  advisor's review and Mustafa's (no code before approval) · plan: `.checkpoints/netcode-proof-test/plan.md` (revision 3.1, the advisor's APPROVE with C1-C12 folded in; revision 1 786b7d2, revision 2 d8f0712, revision 3 a6607bc) · verdict: `.checkpoints/netcode-proof-test/advisor-verdict.md`
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main `af8393e9d40f3bd382e77f86e6df2612785c706c`
- **Last update:** 2026-11-06 03:20 Amsterdam

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

## Current step
10. STOP A: waiting for Mustafa: his answers to ROUTE, SAFARI-TIME, FIXED-IP, DENSITY-FIGURE and UNDER-WAY, his
    approval of the plan, and his OK (or not) for two more branches (`claude/npt-readings`, `claude/npt-safari-wf`).

## Next steps
1. On Mustafa's approval and answers: write his answers into the plan (and FIXED-IP's line into R-PR if (a)).
2. R-PR: one docs PR from main with the advisor's exact words (`advisor-verdict.md`), on `claude/npt-readings` if he
   allows it (else another session); needs-mustafa; merges before the first counted run.
3. U0: the container probes (rustup 1.99.1 + wasm32, wasm-bindgen CLI, a dated nightly, Chromium, loopback QUIC).
   Under UNDER-WAY (a) the kit freeze lifts once U0 is committed.
4. Then U1-U8 (plan §12.1), stop B (plan §12.2), the runs R1-R7, stop C.

## Open findings
- Review round 1's 49 findings: fixed in revision 2 (§0.1). Review round 2's 46 findings: fixed in revision 3 (§0.2). The advisor's C1-C12: folded into revision 3.1 (§0.3).

## Key numbers
- Hetzner, excl. VAT, read 2026-11-05 (infra digest D): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h (plan prices include
  IPv4); 1 RS + 1 LG = 0.1562/h; each VM rounds up per session; RS<->LG traffic internal and free.
- Rust stable 1.99.1 (2026-10-13; static.rust-lang.org answers 200); container has 1.97.0, x86_64 target only.
- macOS runner images: macos-26 arm64 Safari 26.6.2; macos-15 arm64 Safari 27.2 (infra digest F).

## Decisions taken
- The advisor, 2026-11-06: the plan APPROVED with C1-C12 (folded in); readings ruled as plan §15 shows; R-19 and
  UNDER-WAY escalated to Mustafa; R-PR's exact words in `advisor-verdict.md`.
- Settled inputs (Mustafa, this session's prompt, 2026-11-05): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms
  round trip, +-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only; CCX23 (RS, x86) + CAX21
  (LG, Arm64) in Falkenstein, created and deleted by Mustafa per run; SSH by a public key the session gives him, never
  a Hetzner token; each run's plan states its hours and a cost cap for his OK; Safari leg tries GitHub's macOS runners
  first, any `.github/` change goes to him; shamblefrontlines.com at Cloudflare, one exact DNS record per test host,
  always "DNS only" (grey cloud).

## Running work
- PR: [Mergincanli/Shamble-Frontlines#40](https://github.com/Mergincanli/Shamble-Frontlines/pull/40), draft, opened
  02:56 on head a6607bc; this session is subscribed to its activity; safety-net check-in `trig_017Q5fgnrrQsugi3dbZ9pa3L`
  fires 03:45 Amsterdam (re-arm silently if nothing changed; stop after 3 empty ones).
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
