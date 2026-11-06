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
- B2+B3 done (plan fully read). Concerns to verify in sources:
  - C1 money: 75 RS-h vs ADR 0006:1107-1113 estimate 38-53; total EUR 13.51/caps 14.90 + FIXED-IP 1.00/mo. Per-run cap OK is Mustafa's settled ask; the total overrun vs draft estimate should be visible in his asks (not just §7.1).
  - C2 R-10 cover rule (OQ-0001-9): favour-shooter vs cover is game feel/design -> likely Mustafa's; check ADR 0001 OQ-0001-9.
  - C3 operator settings file (ops/operator/settings.json, run with --settings on his laptop) = a security rule for a Claude session on his machine -> his explicit approval at stop B (category 3); plan only lists guard check.
  - C4 kit freeze / W-PR / A4-FREEZE "under way": check STATUS.
  - C5 SAFARI-TIME (b): -36 datagram cap "final only once Mac reading exists" vs ADR 0001 results at close incl. datagram cap; does foundation wait? check SPIKE_PLAN.
  - C6 harness "for the boundary spike to keep": check SPIKE_PLAN:252-279 says boundary spike builds on it (else generic-for-later).
  - C7 R-14 "migration step 1 waits for R8": check SPIKE_PLAN/ADR.
  - C8 checkpoint file .checkpoints/netcode-proof-test.md exists?
  - C9 R-6 1,000,000 packets (PM-0002-49 is boundary spike's mark) - check source lines.
  - C10 R-19 OQ-0006-19 reading deferred - check what OQ-0006-19 is (maybe Mustafa's: money/machines?).
  - C11 R-26 changes meaning of "session" in ADR 0006 density rule - check ADR 0006:1314-1316, 1352.
  Next B4 (sources).
- B4 STATUS read: kit freeze = SCOPE-STOP words "After A3, the kit is frozen until the netcode proof test is under way" (STATUS:176); A4-FREEZE (a) "A4 waits until the netcode proof test is under way" (STATUS:450). A3 = PR #38 merged (main af8393e) but STATUS (2026-11-05) still says in flight -> docs drift, flag for docs-keeper. MACHINES-TIMING (STATUS:318-322): "If they can, that is the Safari check, and I borrow the Mac only for the look proof test" -> SAFARI-TIME (b) matches his earlier words; ask is fair since drafts limit runner to yes/no+count rows. STATUS:48-49 For Mustafa: OK each run's cost cap; the .github/ workflow if the plan puts it here.
  - C4 thought: "under way" undefined; also CLAUDE.md protocol 1 "one change-set at a time" vs A4 kit work during the spike -> UNDER-WAY is his (his freeze rule). W-PR: category 3 anyway, his approval covers freeze question.
- B4 SPIKE_PLAN item 1 read (130-204, 22-79, 206-279): C6 resolved: SPIKE_PLAN:179-181 "It builds first the client harness the boundary spike shares ... accepted with the order (J6 (5))" -> E7 traces, not for-later. Online halves of PM-0004-61/-70 (SPIKE_PLAN:144-145) match plan §2. Runner list SPIKE_PLAN:53-57 = -28, -14, -25 Origin, PM-0006-15; "a row whose limit is a time, or which ADR 0003's look proof test needs, stays on the borrowed Mac" -> R-13 (add -32, PM-0006-25..-27) must check those aren't time rows / look-test rows. SSH key "made and held without the session reading a secret it did not make" (SPIKE_PLAN:154-155) -> laptop makes key: satisfies. DNS record "set on each run day to that day's address" (SPIKE_PLAN:173) + "No draft writes an address" (SPIKE_PLAN:64-66) -> FIXED-IP changes setup "VMs created fresh, new IPs" (Mustafa's words SPIKE_PLAN:63-64) -> FIXED-IP rightly his. OQ-0001-5 rule cap: "honest rewind at P2, rounded up; beyond the cap the victim's view wins" (SPIKE_PLAN:201-203).
- B4 ADR 0001 partial: OQ-0001-9 (1159-1164): decider for cover = "the netcode proof test"; melee = Mustafa+game-designer. So R-10's "netcode-architect picks with Mustafa at the close unless the advisor states a rule" -> a rule is needed; principle already settled by Mustafa's REWIND-CAP/OQ-0001-5 ("beyond the cap, the victim's view wins" => inside, shooter's view) -> my rule: cover judged rewound (shooter's view) inside the cap per item, present only where rewinding fails a cost mark; shares reported; he may overrule at the close. ADR 0001:1603-1604 "A mark's number changes only in the draft". PM-0001-32 = error count (runner ok); PM-0001-29 = 3 s time (Mac); PM-0001-36 = per-browser size on home EU path (Mac).
- B4 ADR 0006 partial: OQ-0006-20 (1114-1121) answered by Mustafa = one cap per run (b-like). Drafts' estimate (~23 RS-h ADR 0006 + 15-30 ADR 0001 guess = 38-53) vs plan 75: overrun ~22-37 h, ~EUR 3-5 more; money is his -> must be said plainly in Decisions/For Mustafa (FINDING F1). PM-0006-25/-26/-27 (1363-1365) are count rows (20 attempts, 0 dropped, 5 of 5), no time limit -> R-13 may add them; -32 is error-count on both backends -> runner ok only if its WebGPU adapter exists; else WebGPU half stays on MAC-b (condition).
- B4 ADR 0006 done: OQ-0006-19 (1087-1099): "Decider: Mustafa (money), on the Advisor's advice" -> plan's R-19 treats as advisor reading only: FINDING F2 - must go to Mustafa (my advice (a), the drafts' interim, which PM-0006-14 already assumes); can be asked now with the other three. Part 5 names TCP 80 only if http-01 (ADR 0006:241) -> R-2 http-01 consistent with PM-0006-15 list. Density "three sessions at least 6 h apart, one 18-23 Amsterdam" (ADR 0006:1314-1316); R-26 one VM set per day = setup reading, mine; accept with "one host per N" labelling. OQ-0006-16 decider infra+netcode, confirmed by Advisor (1038-1050) -> R-1 is mine to confirm.
  Next B5 (review findings, ssh-probe).
- FINDING F3 (security, plan §12.3): main's settings allow `Bash(git push origin claude/*)` and `-u origin claude/*` with no prompt (origin/main .claude/settings.json:34-35). The results branch `claude/npt-results` matches it, so the plan's claim "every other push prompts, the results push included. His prompt is the wall" is false as written; any other `claude/*` branch also pushes unprompted. Fix: the operator settings add an `ask` rule for every push form (ask beats allow; Bash and PowerShell forms), and stop B's guard check adds a fourth attempt: a push to claude/npt-results must prompt.
- B5 partial: ssh-probe.md read, no secret/addresses beyond public 1.1.1.1 and metadata IP. Own check: `command -v ssh ssh-keygen scp sftp` -> none (exit 1), /usr/bin/ssh* absent: reproduced "no SSH client". TCP-22 probe: my shell refused the /dev/tcp probe (permission denied), so port-22 egress NOT reproduced by me (only the client half).
- Spot-checked round-2 fixes present in plan text: N1 (node .gate/* deny §12.3), N3 (dispatch on change-set branch §9), NC2-F1 (channels in Link §4.7), NC2-F3 (queue depths R-17), INF2-1 (time-of-day table §7.2), INF2-2 (npt-ctl peers §10.5), INF2-3 (RS-to-LG key §10.5): all present.
- FINDING F4 (contradiction): R-14 "migration step 1 waits for R8" + R8 includes "the Mac if SAFARI-TIME (b)" vs SAFARI-TIME (b) text "nothing before the foundation waits on them". Under (b) the foundation would wait for the Mac. Check SPIKE_PLAN foundation deps (~100-115, 402-420).
- FINDING F5 (models/effort): the laptop operator session's model and effort not stated (CLAUDE.md: Opus 5.5, xhigh for netcode) -> state in PROMPT-Rn / start command.
- F4 checked: SPIKE_PLAN:103-112 wave 1 = netcode, sim core, 60Hz, boundary, then look proof test "once its models exist"; (2) foundation after. Foundation deps (SPIKE_PLAN:411-414) don't name the look test. So under (b) R8's Mac rows normally land before the foundation, but if models are late R-14 would hold the foundation on the Mac. Ruling: migration step 1 waits for R8's ML and RD rows only; Safari's time rows close at the Mac's first visit; -36's cap stays provisional (data value) till then. Wording fix, not a blocker.
- R-item checks: R-7 changes setup (-28 "server on the same local network", ADR 0001:1543) -> loopback = setup reading -> needs R-PR words (plan says none). R-9's draft words ambiguous ("half of the profile's RTT minus base RTT", ADR 0001:1513): plan's (RTT-base)/2 is right; clarify words in R-PR. R-13: PM-0006-26 needs 3 rotations in one 30-min session with each browser live (ADR 0006:1364) but W-PR job timeout 30 min + runner token 30-min expiry -> conflict: FINDING F6 (condition). -32 both backends: runner WebGPU adapter may be absent -> WebGPU half to MAC-b if adapter null. R-28 checklist "The profile and the cap change from the overlay" (ADR 0001 checklist line ~1640) -> read-only cap OK, R-PR wording.
- FINDING F7: laptop clone is on main; a later kit change on main (A4 etc.) changes the laptop's rules after stop B's guard check -> record main's CLAUDE.md+.claude tree hash at stop B; rerun guard check at a run start if it changed.
- FINDING F8: operator settings file is a security rule for a Claude session on his machine -> his explicit approval at stop B (not only the guard check).
- B6 done: gate inspects only the five legacy files (tools/gate.mjs:34 FILES); branch diff vs main af8393e touches only .checkpoints/ (git diff --name-only, non-checkpoint list empty) -> gate not rerun (no code; verifier's run stands for a plan). repo-map-check not in CI; REPO_MAP has no spikes/ entry -> docs-keeper adds it (minor). STATUS drift: says PR #38 in flight but main af8393e = its merge.
  Not checkable here (no web tools): prices, runner Safari versions, fsn1 availability, LE shortlived profile, crate versions, Cloudflare TTL options.

## B7 Rulings R-1..R-29 (done)
R-1 PORTS: confirm (mine: OQ-0006-16 decider infra+netcode, confirmed by Advisor) -> R-PR (ADR 0006 OQ-0006-16 + SPIKE_PLAN item 1).
R-2 CERT: approve; 90-day fallback needs security-auditor AND advisor OK at U6.
R-3 METRICS: approve (mine).
R-4 HOST KEYS: approve (mine).
R-5 REORDER: approve -> R-PR (ADR 0001 part 13 para ~1447-1450).
R-6 PACKETS: approve 1,000,000 both layers -> R-PR (PM-0001-3).
R-7 LOOPBACK: approve, but it IS a setup change -> R-PR (PM-0001-28).
R-8 X86 REPLAY: approve (mine), no CI.
R-9 L2 DELAY: reading right; add wording fix to R-PR (PM-0001-3 method).
R-10 COVER: change: I state the rule (from Mustafa's OQ-0001-5 answer): rewound per item unless cost marks fail; shares to Mustafa at close, he may overrule -> R-PR (OQ-0001-9).
R-11 STAND-INS: approve, no R-PR.
R-12 BITES: approve -> R-PR, incl. ADR 0004 PM-0004-67 row (not only 0001/0006).
R-13 RUNNER LIST: approve with 2 conditions (-32 WebGPU adapter; -26 job time limit + token life cover the 30-min session) -> R-PR (SPIKE_PLAN:53-55).
R-14 CLOSE: approve, reworded: migration step 1 waits for R8's ML+RD rows only; Safari time rows at the Mac's first visit -> R-PR (SPIKE_PLAN item 1).
R-15 PM-0004-67: approve -> R-PR (ADR 0004 row).
R-16 -59: approve -> R-PR (PM-0001-59).
R-17 SIM GAPS: approve -> R-PR depths (PM-0001-23/-24 + part 13).
R-18: approve (a) in R-1's words, (b) every extra CAX21 on fw-lg.
R-19 OQ-0006-19: NOT mine: decider Mustafa (money) on Advisor's advice (ADR 0006:1098). My advice (a). Escalate now as DENSITY-FIGURE.
R-20: approve, no R-PR.
R-21: approve.
R-22: approve + a fallback that changes crate/transport goes to the drafts (ADR 0001:1596-1597).
R-23: approve with §12.3 + F3 fix + F7.
R-24: approve.
R-25: approve -> R-PR (ADR 0001:1444-1445).
R-26: approve -> R-PR (ADR 0006:1314-1316).
R-27: approve; R-PR files = ADR 0001, 0004, 0006, SPIKE_PLAN; own branch from main; needs-mustafa (ADR edits).
R-28: approve -> R-PR (checklist wording).
R-29: approve; add: if the laptop's IPv4 changes mid-run, Mustafa updates fw-rs and the session reruns npt-ctl peers.

## B8 Verdict (done)
APPROVE with conditions C1-C12 (text fixes into rev 3.1 before Mustafa reads; no new advisor round unless more than the listed text changes):
C1 F3 push ask rule + 4th guard attempt. C2 OQ-0006-19 -> Mustafa now. C3 UNDER-WAY -> Mustafa. C4 operator settings+PROMPT approved by Mustafa at stop B. C5 kit-change re-check on laptop (tree hash at stop B). C6 laptop session Opus 5.5 xhigh stated. C7 R-13 conditions (-26 timeout/token; -32 WebGPU). C8 R-14 wording. C9 R-PR = my exact words, adds ADR 0004, R-7, R-9, R-14 (+FIXED-IP line if (a)). C10 §16 one line: total vs drafts' estimate. C11 R-22 drafts route; R-2 90-day needs advisor too. C12 SAFARI-TIME text: (b) matches his MACHINES-TIMING words; -36 cap provisional. Minor: REPO_MAP spikes/; STATUS drift (#38 merged); R-29 address change.
Decisions for Mustafa: ROUTE, SAFARI-TIME, FIXED-IP (fair, his), + DENSITY-FIGURE (OQ-0006-19), UNDER-WAY (A4-FREEZE). W-PR: his approval anyway (category 3) - no separate freeze decision if UNDER-WAY (a).
DONE
