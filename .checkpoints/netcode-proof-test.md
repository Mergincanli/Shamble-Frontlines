# Checkpoint: netcode-proof-test
- **Goal:** start the netcode proof test (SPIKE_PLAN item 1) as its own change-set; this step is the PLAN only, for the
  advisor's review and Mustafa's (no code before approval) · plan: `.checkpoints/netcode-proof-test/plan.md` (revision 3.2: APPROVED by the advisor (C1-C12) and by Mustafa, 2026-11-06; revision 1 786b7d2, revision 2 d8f0712, revision 3 a6607bc) · verdict: `.checkpoints/netcode-proof-test/advisor-verdict.md`
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main `af8393e9d40f3bd382e77f86e6df2612785c706c`
- **Last update:** 2026-11-06 (after Mustafa's approval) Amsterdam

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

## Current step
17. Waiting for Mustafa's own "merge #41" in this chat, for head 1b4094a. Meanwhile U1 part 2b: the message catalogue
    draft (netcode-architect, DONE: `docs/design/netcode-proof-messages.md`, 22 wire rows, 24 field groups, 4 session
    and 12 room-wide controls, Q1-Q9 for the advisor) and its class audit test; test-engineer drafts marks.json
    (`marks-draft.jsonl`); then the dependency-rule checks.

## Next steps
1. On Mustafa's "merge #41": check gate and smoke green on 1b4094a, merge with that SHA as the expected head; then
   `git merge origin/main` into this branch (only reviewed main comes in) so marks.json's row hashes match; remove
   the worktree `.gate/npt-readings`.
1b. The catalogue's questions Q1-Q9 (netcode-architect's notes `netcode-architect-catalogue.md`) and test-engineer's
   marks questions go to the advisor together, in one request.
2. U1 part 2: npt-log's record types and writer; npt-judge with its PASS, FAIL and INVALID fixtures (PM-0006-14's
   short padding among them); marks.json (test-engineer drafts it per `marks/README.md`, with SERIES.md); the drift
   test (after the merge); the dependency-rule checks (a cargo metadata test in npt-lab) and the no-reference grep;
   redact-check; the message catalogue (netcode-architect) and its class audit test; then docs-keeper (REPO_MAP gets
   spikes/ and the catalogue; STATUS: PR #38's line, the Mac line at 44-45, the plan's approval, UNDER-WAY (a)).
3. For Mustafa, before R1: Chrome and Firefox (stable) on the laptop; MSI GPU switch on hybrid if offered.
4. Write files with the Write and Edit tools only, never shell redirects or scripts that write (Mustafa, 2026-11-06:
   the guard can't follow cd, so such writes ask him); run commands from the main folder without cd.

## Open findings
- Review round 1's 49 findings: fixed in revision 2 (§0.1). Review round 2's 46 findings: fixed in revision 3 (§0.2). The advisor's C1-C12: folded into revision 3.1 (§0.3).

## Key numbers
- Hetzner, excl. VAT, read 2026-11-05 (infra digest D): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h (plan prices include
  IPv4); 1 RS + 1 LG = 0.1562/h; each VM rounds up per session; RS<->LG traffic internal and free.
- Rust stable 1.99.1 (2026-10-13; static.rust-lang.org answers 200); container has 1.97.0, x86_64 target only.
- macOS runner images: macos-26 arm64 Safari 26.6.2; macos-15 arm64 Safari 27.2 (infra digest F).

## Decisions taken
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

## Running work
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
- Saver: background task `bar6y14h2` · output `/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/bar6y14h2.output`
  · started 10:36 · stops by 12:26 (110 min).
- The advisor's review of PR #41 (advisor agent, xhigh, background) · notes `advisor-pr41.md` · DONE: REVISE of
  2ccc184, then APPROVE of 16763a7 (re-review via SendMessage, same agent).
- The message catalogue (netcode-architect, xhigh, background) · notes `netcode-architect-catalogue.md` · writes
  `docs/design/netcode-proof-messages.md` · launched 11:12 · DONE 11:20 (Q1-Q9 for the advisor).
- marks.json's draft (test-engineer, xhigh, background) · notes `test-engineer-marks.md` · writes `marks-draft.jsonl`
  and `series-draft.md` · launched 11:16 · running.
- R-PR's text: engine-architect (xhigh) · DONE (placements, then 8 fixes); its scripts rpr-edit.py, rpr-patch-c.py,
  rpr-verify.py, rpr-words.json and notes engine-architect-rpr.md in the notes folder.
