# advisor plan review: netcode-proof-test plan revision 3 (day session, 2026-11-06)

## Plan (blocks, in order)
B1 plan.md sections 0, 0.1, 0.2 (summary, fix maps)
B2 plan.md body (sections 1-14)
B3 plan.md 15 (R-1..R-29) and 16 (decisions/asks)
B4 sources: SPIKE_PLAN item 1, ADR 0001/0006 (relevant parts), STATUS, advisor rules context
B5 review1/review2 findings + ssh-probe (spot checks)
B6 own checks (git diff scope, gate if needed)
B7 rulings R-1..R-29 written
B8 verdict written
DONE

## Notes
- B1 done: summary read. SSH no; hybrid route (cloud-init + laptop Claude Code, Manual mode); spike in spikes/netcode-proof/ Rust+TS; marks PM-0001-1..-62 minus 60Hz group, PM-0004-67, PM-0006-11..-27; 7 run days, EUR 13.51 planned, caps EUR 14.90; FIXED-IP EUR 1.00/mo; Safari via macOS runners, .github/ PR to Mustafa; DNS A rs.nt, DNS only, TTL 1 min. Next B2.
- B2 partial (lines 1-532 read): §1 SSH probe table, routes a/b/c, rec (a) laptop; key made on laptop; host-key check via HTTPS-published known_hosts + CT. §2 scope; out: 60Hz, look, lobby, CI for spikes/ (R-8), legacy. §4 layout spikes/netcode-proof/ 16 crates + web + ops + results + docs/design/netcode-proof-messages.md. §4.7 symbols listed per crate. §4.9 kept: whole spike merges to main.
  - Q to check: 16 crates for a throwaway spike = heavy; but justified by ADR 0002 part 3 split + boundary spike keep. Watch generic-feature creep (npt-super, npt-host ACME client hand-written?).
  - Q: npt-host "page host and the ACME client" - writing own ACME client? check §8.4 which names ACME client.
- B2 partial (to line 745): §6 verdict rules, mark table, §6.3 rows before named machines, §6.4 L1 deterministic. §7 prices CCX23 0.1386/h CAX21 0.0176/h; 75 RS-h vs drafts' 38-53 (ADR 0006:1107-1113) -> 22-37 h over drafts' estimate; explained (3-times-of-day rotation, idle, bring-up). Money is Mustafa's: total EUR 13.51 / caps 14.90 should be framed as his OK (check §16 asks it explicitly, not only per run).
  - FINDING F-a: drafts' estimate exceeded ~1.4-2x; must be stated to Mustafa as an overrun vs the ADR estimate in For Mustafa (check §16).
  - R7 ML* legs 5.3 h (-44 12x10 min = 2 h) - on laptop while RS billed; ok.
- B2 partial (to 904): §8 ports UDP 30001-30032, TCP 31000-31032 (31000 page host), 9100 metrics LG-only, 22, 80 only while order live. DNS: A rs.nt -> RS IPv4, TTL 1 min, DNS only; spare rs-b.nt. Exact (type/name/value) given: OK for Mustafa ask. ACME http-01 with instant-acme 0.8.5, 6-day shortlived profile; 90-day fallback needs security sign-off at U6.
  - §9 Safari: npt-safari.yml workflow_dispatch only, permissions {}, SHA-pinned actions, dispatched on change-set branch; W-PR to main before R1 on Mustafa's approval+go; ".github/ change goes to Mustafa" restated: OK. "kit is frozen until this test is under way; he decides" -> check STATUS kit freeze (Q3).
  - NOTE: W-PR dispatch-from-any-ref after merge: any branch's copy runs (accepted risk ADR 0006:551-555). Check that line.
- B2 partial (to 1083): §10 security: control plane local Unix sockets + per-run capability; join tokens per slot 128-bit; URL fragment, replaceState; page host allow-list + CSP; cloud-init users, sshd drop-in, pinned toolchains, dist hash compare; redact-check pre-push. §11 Brave checks: R1 smoke (expected results given), ping, R7 netcode checklist 14 steps (test-engineer writes from ADR 0001:1631-1681, advisor checks before R7), costs read. Brave checklist present with expectations: OK.
  - NOTE: §10.6 "The laptop's git author set to his GitHub noreply address" - fine.
