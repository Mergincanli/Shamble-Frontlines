Task: security-auditor adversarial review of netcode-proof-test plan revision 1 (.checkpoints/netcode-proof-test/plan.md), lens: run setup + internet-facing security.

## Plan (blocks, in order)
B1. Read plan.md whole (memory check first).
B2. Read ssh-probe.md + infra-engineer-digest.md sections E-H (+ relevant parts).
B3. Read SPIKE_PLAN.md (153-175 and around) + ADR 0006 (part 5, part 13, secret, B9, B12) + ADR 0001 part 10 (419-473).
B4. Read netcode-architect-digest for PM-0001-25/-26/-34/-59 and H1-H20 in my lens; test-engineer-digest spikes/ rules.
B5. Read .github/workflows/gate.yml + .claude settings/hooks relevant to the laptop session (12.3).
B6. Web checks: Hetzner (firewall, user data/cloud-init metadata, abuse), Let's Encrypt (6-day profile, staging, rate limits, http-01), GitHub (workflow_dispatch perms, macOS runners), Cloudflare DNS only.
B7. Write findings (<= ~20) + coverage table + picks.
B8. DONE.

## Notes
