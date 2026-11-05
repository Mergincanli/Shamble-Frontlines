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
- No prior memory dir found for security-auditor (.claude/agent-memory absent). Threat model to be written in findings.
- B1 done: plan.md read whole (700 lines). Early suspicions: (a) 1.6 HTTPS-published host keys - cert issued via http-01 to whoever holds the IP; file published by the same VM it authenticates (circular if attacker holds the address); (b) TCP 80 "any" in Hetzner fw for whole VM life, only nftables drops it; (c) TCP 31000 page host open to all with host-key, hash files; no headers/CSP stated; (d) laptop session rules/settings undefined (.claude loaded from which branch?); (e) Safari workflow inputs host/ports/commit - checkout of arbitrary commit on dispatch; (f) cloud-init pasted by Mustafa - which commit, integrity check of paste; (g) results branch push creds on laptop; (h) ICMP on LG; IPv6 on VMs open with Hetzner fw? (i) Join tokens "test tokens made per session" - how distributed to bots/browser; (j) 9100 exporter; (k) spike-only controls SetProfile/SetRewindCap/TriggerBurst/BlockUdp in proto reachable from internet clients? Next B2.
