# Checkpoint: netcode-proof-test
- **Goal:** start the netcode proof test (SPIKE_PLAN item 1) as its own change-set; this step is the PLAN only, for the
  advisor's review and Mustafa's (no code before approval) · plan: `.checkpoints/netcode-proof-test/plan.md` (to write)
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main `af8393e9d40f3bd382e77f86e6df2612785c706c`
- **Last update:** 2026-11-05 22:40 Amsterdam

## Steps done
1. SSH reachability probe (plan item 1, first pass) · this commit · NO: no ssh client in the image; TCP 22 and every
   port but 443 time out, direct and via the proxy; 443 is TLS-re-terminated by the egress gateway. Evidence:
   `.checkpoints/netcode-proof-test/ssh-probe.md`. So the plan proposes cloud-init and/or Claude Code on Mustafa's PC.

## Current step
2. Requirement digests (one workflow, 3 agents at xhigh): netcode-architect (ADR 0001 spike text, PM-0001-1..63, the
   netcode checklist, budgets, profiles), infra-engineer (ADR 0006 spike item 2, PM-0006-11..27, operations checklist,
   machines, certs, ports, costs, Safari runners, cloud-init), test-engineer (ADR 0002's link simulator L1/L2 and the
   client harness, ADR 0004 PM-0004-67/S17, test plan). Notes: `.checkpoints/netcode-proof-test/<agent>-digest.md`.

## Next steps
3. Write the plan from the digests: `.checkpoints/netcode-proof-test/plan.md`.
4. Plan review round (at most 3 agents): security-auditor (SSH key, certificates, firewall, cloud-init, the Safari
   workflow), engine-architect (spikes/ layout, the harness shared with the boundary spike), a completeness critic
   (every pass mark covered).
5. Fix, then the advisor's plan review (APPROVE / REVISE / ESCALATE); iterate on REVISE.
6. Push, open the draft PR, report to Mustafa (Decisions needed, For Mustafa).

## Open findings
- (none yet)

## Key numbers
- (none yet)

## Decisions taken
- Settled inputs (Mustafa, this session's prompt, 2026-11-05): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms
  round trip, +-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only; CCX23 (RS, x86) + CAX21
  (LG, Arm64) in Falkenstein, created and deleted by Mustafa per run; SSH by a public key the session gives him, never
  a Hetzner token; each run's plan states its hours and a cost cap for his OK; Safari leg tries GitHub's macOS runners
  first, any `.github/` change goes to him; shamblefrontlines.com at Cloudflare, one exact DNS record per test host,
  always "DNS only" (grey cloud).

## Running work
- Workflow `wf_1ae38d53-da9` (npt-plan-digests: 3 digest agents, xhigh) · script as launched:
  `.checkpoints/netcode-proof-test/npt-plan-digests-run1.js` (no args) · notes: `.checkpoints/netcode-proof-test/` ·
  launched 22:38 · running. Resume: Workflow({scriptPath: <that copy>, resumeFromRunId: "wf_1ae38d53-da9"}).
- Saver: background task `btpf0ptzw` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/btpf0ptzw.output`
  · started 22:37 · stops by 00:27 (110 min) · last check 22:40: started.
