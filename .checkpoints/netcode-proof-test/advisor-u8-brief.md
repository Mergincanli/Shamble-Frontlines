# Advisor: U8's build brief, rev 1 (one check under NPT-TRIM)

Working from main's `.claude/agents/advisor.md` (git show origin/main, 335 lines, read 2026-11-22).
Brief: `.checkpoints/netcode-proof-test/u8-brief.md` (368 lines). Drafter's notes: `infra-u8-brief.md`.

## Plan
1. Read the rules in force: history.md:7561-7607 (NPT-TRIM, his answers of 2026-11-22), :2234-2282 (TEST-FIRST).
2. Read replan.md, replan-units.md §3, §5, §7; plan.md:1256, §7.2-§7.3, §12.2-§12.3.
3. Read security-r1-deps.md (SEC-R1D-2, -3, -5, -6, npm rules), advisor-u6a-brief.md cond 2-4, u6a-brief.md rev 1.1.
4. Check facts: tests.rs:1407, advisor-u3d.md:80-83, u3c-kernel-check rig, prices, arithmetic (hours, cap, lines, days).
5. Rule Q-U8-1 to -7; the U6c test change; size; R1's run plan and dates; stale plan.md lines; Stop B; other findings.
6. Verdict and paste-ready conditions.

## Notes (one line per finding)
- Blocks 1-3 DONE (rules, re-plan, plan.md sections, security-r1-deps, u6a carries read).
- N1 R1 hours sum 3.55 OK (u8-brief.md:233-247); cap 5x0.1386+5x0.0176=0.781 -> 0.80 OK; planned 0.6248 OK; R2-R5 6.62/7.30 OK.
- N2 dates sums 7,410/11,120/9,265 and days 6.3/7.9/9.5 OK as computed, BUT MOVED (replan.md:20) and U6j (replan-units.md:63, Likely blank) are estimates mixed with likely sizes -> likely 8,118-13,005, middle 10,561 (9.0 d); top U6j ~Tue 15 -> lever's "keeps Fri 18 at the top" (u8-brief.md:303-304) does not hold.
- N3 HIGH: Manual mode (plan.md:1304-1305) prompts every laptop-session command; operator settings add no ssh allow (plan.md:1308-1320; main settings.json has no Bash(ssh)). The brief has him "away" for bring-up and the server blocks with one block per ssh call (u8-brief.md:89-90, :233-246, :266) -> the session stalls; sittings 2h20 (u8-brief.md:270) wrong; delete-by risk.
- N4 MED: `.npmrc` is protected at any depth (routine-guard.mjs:38; dayEdit refuses agents :962-968; settings.json:256 denies Read(**/.npmrc)); advisor.md:43 "any .npmrc" is a gate file -> builder refused, PR tier-2 effect. Brief Block 9 (u8-brief.md:111).
- N5 MED: PM-0006-25 suite cert-14d and PM-0001-29 need 20 attempts per browser, each closed before the next (marks.json notes); brief counts 15 slots, one per leg (u8-brief.md:184-186) and has no cert-14d script (:117-119) -> condition 2's "slot for every leg" not met.
- N6 MED: RS-to-LG key and npt-load-ctl (plan.md:1157-1162) unnamed in the brief; r1.sh blocks need LG driven; owner unassigned (U6c builds npt-load only).
- N7 MED: Limits tab trimmed to "RS plus one CAX21" (u8-brief.md:254) vs plan.md:871-872 five CAX21s; R5 needs RS + up to 4 LG on Mon 14 Dec.
- N8 MED: SEC-R1D-2's third part "the laptop's firewall blocks inbound" (security-r1-deps.md:206) not carried.
- N9 MED: `alone` = 0 at bring-up stops R1 (u8-brief.md:91-93) -> his sittings lost; name what continues.
- N10 Q-U8-6 (i): model side runs after R1 (u8-brief.md:87-88) -> knob need not be in U6c; rides MOVED before R1's report.
- N11 Q-U8-1/-7 security: root reads of npt-room-written files (token file) must drop to npt-room (link attack); sudoers anchored regex (sudo 1.9.15p5 here, regex since 1.9.10), never `*`.
- N12 LOW: LG created ~09:15-09:20, delete ~13:25 > 4 h -> 5 h billed; per-VM delete-by.
- N13 LOW: operator deny `node .gate/*` (plan.md:1318) vs browser scripts in `.gate/npt-run/` -> invocation form in PROMPT-R1.md.
- N14 LOW: stale list misses plan.md:1140-1141, :1144, :940, :969, :1110, :1215.
- N15 MED: Stop B round lacks the netcode seat for R1.md's judged blocks (S14, two self-tests, C-U8 reads); NPT-TRIM default "netcode and test".
- Checked: Ubuntu 24.04.5 container, util-linux 2.39.3-9ubuntu6.6 Essential/required owns /usr/bin/taskset; nft -c -f works here.
- N16 MED: Q-U8-2's lean leaves the Hetzner firewall as the only /32 wall on the game ports; nothing proves fw-rs/fw-lg are applied on the day. LG->RS:22 must fail (nftables accepts 22 from any) = cheap proof.
- N17 PR #41 merged_at 2026-11-06T13:35:37Z (gh api, REST) OK. replan-marks.md:115 cite OK. sudo 1.9.15p5 here.
- Blocks 4-5 DONE (facts checked, rulings formed). Block 6 DONE (the ruling below). All blocks DONE.

---

# Ruling (block 6)

## Verdict: APPROVE WITH CONDITIONS (C1-C10 below; Lows L1-L5 ride with them)
One High (ADV-U8-1: the brief has him away while a Manual-mode session needs his prompt for every command). C1 closes
it as written; applied word for word, no re-check is needed (the precedent: advisor-replan.md's High, closed by its
paste-ready conditions, replan.md:4-5). If the main session changes C1's substance, C1 alone comes back.
Risk tier of the work U8 builds: 1 (spike scripts and docs) as long as C2 holds; a `.npmrc` would make it a gate-file
change (advisor.md:43, :166).

## 1. Q-U8-1 to -7
- **Q-U8-1 (rooms' CPU arrangement): (a), the lean.** Under cgroup v2 a task's allowed list is cut to its cgroup's
  cpuset, so once `npt-ctl cpus` takes the tick vCPU out of `user.slice`, a room started from `npt`'s session cannot pin
  onto it (the brief's own reasoning, u8-brief.md:330-332, holds); (b) lets every SSH command count against `alone` at
  the start and header reads, a flaky gate. (a) does not undo the re-plan's cut (replan-units.md:28-30): rooms still
  start and die as npt-super's children under one unprivileged user; the new entry starts npt-super, not a room unit.
  **security-auditor confirms at Stop B, each by file:line:**
  1. sudoers admits only `npt-ctl rooms start NN` and `npt-ctl rooms tokens NN`, NN by an anchored regular expression
     for 00-31 (sudo 1.9.15p5 on 24.04 reads `^…$`), never `*`, which in sudoers matches spaces too; `visudo -c`.
  2. The wrapper builds its command line from NN and root-owned values only (the housekeeping set from `npt-ctl cpus`'s
     root-owned record, the installed binary path); no caller text reaches `systemd-run`, `setpriv` or `taskset`.
  3. `setpriv` drops to `npt-room` and its group, clears supplementary groups, sets no_new_privs, and clears the
     bounding and ambient capability sets; `npt-room` has no shell, no sudo and no write to binaries or the clone.
  4. Root never opens, follows, chmods or chowns a path `npt` or `npt-room` can write: `rooms tokens NN` reads the
     token file as `npt-room` (setpriv), so a link the room plants reaches nothing `npt-room` cannot read; a restart of
     NN frees its token path as `npt-room` (U6a refuses an existing path: SEC-U6AD-3, u6a-brief.md:79-82). The same
     holds for every other wrapper's writes (`npt-capture`, `npt-s14`, `npt-cert`): a root-owned directory, or the
     write made after dropping to that directory's owner.
  5. Directory owners: settings in a directory `npt` writes and `npt-room` only reads; tokens and room logs in one
     `npt-room` writes and `npt` cannot; both under root-owned parents.
  6. The scope ends with npt-super; npt-super's end (stdin closed by the run script) kills its rooms, so nothing
     started by root outlives the run script.
- **Q-U8-2 (home address on RS): the lean, with one check (C5).** His address never reaches RS, its logs or results,
  as plan.md:928-929 already does for TCP 22. But the lean makes the Hetzner firewall the only /32 wall on the game
  ports, and NPT-TRIM's cuts rest on "The VMs' firewall admits only our own machines" (history.md:7571); a firewall
  object not applied at creation would expose every room. C5 proves both objects are in force before a room opens.
- **Q-U8-3 (-29's QUIC block): the lean.** `npt-ctl quic off|on`, LG exempt through `metrics_src`, 15-minute timeout,
  one leg at a time; re-armed per browser, and an attempt that reaches WebTransport while the block is meant on makes
  that leg INVALID (C3). The verb's rule text passes `nft -c` in test 1 beside bootcmd's.
- **Q-U8-4 (who polls DNS): the lean, with terms.** U8's `npt-cert` polls with `dig +norecurse`; U6c's issue mode is
  **one process** across both phases (it prints name and value, waits for "go" on stdin, then finishes), so no ACME
  account key or order state is written to disk between phases; the wrapper asks **every** authoritative server of the
  zone (fixed names in the root-owned clone, plan.md:950-951's `cora` and `steven`, re-read at Stop B) and sends "go"
  only when all show the value, since Let's Encrypt validates from several perspectives (replan-units.md:91); one
  30-minute bound covers both phases; `dig` takes no caller argument. U6c's brief takes this interface.
- **Q-U8-5 (leg-edge reads): the lean.** It is C-U8-6's own fallback (plan.md:1256: "U6 adds them to C-U6-7's
  leg-edge calls and brings any new npt-log name to the advisor"); a shell cannot see the edges of a leg whose socket
  lives in the probe. U6c's brief carries it, and its new npt-log names come to me in U6c's brief check; U8's `probe`
  block checks presence, a missing read makes the leg INVALID.
- **Q-U8-6 (i) (the burst knob): needed, but not in U6c; see section 2.**
- **Q-U8-6 (ii) (DelayStream's judged form, C-U8-5): ruled.** Judged per direction: **E with L0 = 0, ≤ 10,000 ns
  (U3D-BAR's reading of "about"), and p99 of L ≤ 1 ms**, over the self-test's measured window (5 min after 1 min
  warm-up, at least 10,000 samples per direction, 48 connections at P2 with the room running); reported beside it: E
  with L0 = the window's p50 (M1's built form), p50, p99.9, max, steal; a max over 10 ms comes to me with the result;
  a miss takes the fallback order of 2026-11-08 (plan.md:1256). Why: U3D-L0's reason holds here too. DelayStream
  releases at the model's stamp plus L and nothing downstream subtracts a median, so what the WebSocket rows see is L;
  L0 = 0 is the exact predictor and never more lenient than L0 = p50 (advisor-u3d.md:116-118; u3d-plan.md:119-124).
  The cost is small: at most p50 × P(L > 0.25 ms), sub-microsecond at DelayStream's 0.065-0.070 ms hop p50
  (advisor-u3c-plan.md:158). One bar,
  one form, for both self-tests. This changes my own condition-10 form, no pass mark and nothing in marks.json.
- **Q-U8-7 (token file reader): the lean, with Q-U8-1's term 4** (`rooms tokens NN` reads as `npt-room`, never as
  root). U6a's 0600 and create_new stay as SEC-U6AD-3 set them; U6b's security seat reads it with the minting.

## 2. The burst knob U8 asks of U6c (tests.rs:1407)
- **R1's report needs it; R1's run does not.** S14's test is "stall **or** burst-recovery p50 or p99 more than 10 %"
  (plan.md:1256), and the burst half is the only kernel check of DelayStream's burst path, which -18's WebSocket part
  (U4k, R3) runs through; without it TCP-FREEZE is decided on half its evidence ("faithful" stays a reading:
  advisor-u3q.md:139-143). But the model side runs in this container after R1, on R1's measured `tcpi_rtt`
  (u8-brief.md:87-88), so nothing needs the knob before R1.
- **Ruling: it rides in MOVED's brief** (the first lane unit after R1, before R1's report: replan.md:89-92), as a named
  test-only rider of about 20 lines (`NPT_KC_BURSTS`, its own test), read in MOVED's round; U6c's brief stays off it,
  so R1's critical path carries no line for it. U8 fixes the interface now: Block 4 records the burst schedule as
  (start_ns, len_ns) from the run's first message, exactly as the knob will read it, and compare.py computes recovery
  per burst for both sides by advisor-u3q.md:130-133's (e) definition (link's return to the in-order delivery of the
  first message written at or after it), tested now on fixture KCMODEL lines. S14's burst half then reads judged in
  R1's report, never NOT-RUN; if MOVED drops the rider, R1's report says the burst half is NOT-RUN and why, and it
  comes to me before -18's WebSocket part counts.

## 3. Size
- Each addition is needed: the run script (plan.md:261: server blocks run from RS, and C1 makes its chained form
  necessary), the CPU verb (condition 4), the rooms verb (condition 3, Q-U8-1), S14's rig (plan.md:1256; the re-plan
  folded it into "R1.md 200", undersized), the QUIC verb (-29 moved into R1 by RE-PLAN-OK (1)). None is a generic
  feature "for later". C1, C3, C4 and C5 add about 150-200 (the chain, the cert-14d script, npt-load-ctl, the firewall
  check, their tests): about 2,300-2,350 estimated, likely about 3,100-3,500; with the npm lock's about 2,000 generated
  lines counted too, about 5,500, still under SIZE-CHECK's 6,000.
- **One line in the day's message is enough.** It is off the lane (no date moves), under the stop, inside the plan,
  undoable, and touches no pass mark, money or main: TEST-FIRST rule 2's four tests hold (history.md:2261).

## 4. R1's run plan and the dates
- **Arithmetic checked:** the hours sum to 3.55 (u8-brief.md:233-247) and the working at :248-250 is right; planned
  EUR 0.6248 and cap 5 × 0.1386 + 5 × 0.0176 = 0.781, up to EUR 0.80 (plan.md:795-796), equal to the re-plan's
  (replan.md:31); delete-by 13:30 = T0 + 4 h; R2-R5 EUR 6.62 planned, 7.30 in caps (replan.md:32-34); the block times
  in :260-261 match the table; his presence as written sums to 142 min.
- **What does not hold:** (a) the sittings (ADV-U8-1, C1): Manual mode means he must be there for every command, so
  "away" during bring-up's first commands and the server blocks is not possible as built; likely about 2 h 40 min in
  three sittings after C1, against the re-plan's "about 2 hours" (replan.md:39). (b) The dates' sums mix two estimates
  with likely sizes (ADV-U8-6, C6). Corrected: about 8,110-13,010 likely, middle about 10,560 (6.9, 9.0 and 11.1 lane
  days); U6e lands about Thu 10 (top Fri 11), U6j about Sat 12 (top about Tue 15).
- **Does Fri 18 hold?** Yes at the middle, with Tue 15 spare (R2 Sat 12 and R3 Sun 13 after U6e; R5 Mon 14 after
  U6j). At the top, no: R2 and R3 still run on the weekend, but R5 waits for U6j (Wed 16) and the close moves to about
  Mon 21. The brief's line 2 (u8-brief.md:303-304, "keeps Fri 18 even at the top") rests on the uncorrected sum.
- **What is his:** the run days (R1 Thu 3 is decided: history.md:7604; R2 Sat 12, R3 Sun 13 are weekend runs, and
  R5 Mon 14 asks him from about 06:45 to about 23:30); his time in R1 (about 2 h 40 min after C1); R1's plan, cap EUR
  0.80 and delete-by at Stop B (plan.md:875, :1280); R2-R5's plans, caps and delete-bys (about Thu 10); the close's go.
  None needs an answer now: the dates go in the day's message with the main session's lean, and R1's plan at Stop B.

## 5. The stale plan.md lines
No record beyond what exists. NPT-TRIM changes rev 3.3 where they differ (history.md:7565) and his RE-PLAN-OK (4) is
"yes" (history.md:7600); editing plan.md would be paper whose only reader is other papers (history.md:7592). Two
uses only: Stop B's reviewer prompt carries the brief's list (u8-brief.md:19-23, completed by L3) so the reviewers judge
against the re-plan, and U8's report names the superseded lines once.

## 6. Stop B as U8's one round, and other findings
- **Stop B's read is U8's one round: yes**, with one seat added (C9): R1.md holds the judged instruments' rules (S14,
  both self-tests, the C-U8 reads and INVALID conditions), and NPT-TRIM's default seat is "netcode and test"
  (history.md:7590); security and infra alone read none of that. Three at once is still one round.

| Id | Sev | Finding | Where | Fix |
|---|---|---|---|---|
| ADV-U8-1 | High | Manual mode prompts every laptop-session command (plan.md:1304-1305; "His prompt is the wall", :1323); the operator settings add no ssh allow (:1308-1320) and main's settings.json has none. The brief runs one block per ssh call and has him away for bring-up's first commands and all server blocks, so the session stalls; his 2 h 20 min is wrong and the delete-by is at risk. `alone` = 0 also stops the whole run, losing his sittings | u8-brief.md:89-90, :233-246, :266, :270, :91-93 | C1 |
| ADV-U8-2 | Medium | Block 9 writes `ops/browser/.npmrc`. Any `.npmrc` is a gate file (advisor.md:43, :166) and guard-protected at any depth (routine-guard.mjs:38): the builder's write is refused (:962-968), the main session's asks him and turns PR #40 into a gate-file change; reading it is denied (settings.json:256) | u8-brief.md:111 | C2 |
| ADV-U8-3 | Medium | -29 and PM-0006-25 (suite `cert-14d`) take 20 attempts per browser, each closed before the next (marks.json notes). The brief counts 15 slots, one per leg, and has no `cert-14d` script, so condition 2's "a slot for every leg" (advisor-u6a-brief.md:233-237) fails for those attempts | u8-brief.md:117-119, :184-186 | C3 |
| ADV-U8-4 | Medium | The RS-to-LG key and LG's forced command `npt-load-ctl` (plan.md:1157-1162), which let RS's run script drive LG while he is away, have no owner: U6c builds npt-load only (replan-units.md:60) | u8-brief.md:26-41, :89-97 | C4 |
| ADV-U8-5 | Medium | Q-U8-2's lean leaves the Hetzner firewall as the only /32 wall on the game ports; nothing proves on the day that `fw-rs` and `fw-lg` are applied | u8-brief.md:29-30, :337-340 | C5 |
| ADV-U8-6 | Medium | The dates mix estimates (MOVED, U6j: replan.md:20, replan-units.md:63) with likely sizes; corrected, U6j lands about Tue 15 at the top and line 2's "keeps Fri 18 even at the top" fails | u8-brief.md:287-290, :302-304 | C6 |
| ADV-U8-7 | Medium | Stop B's Limits read is trimmed to "RS plus one CAX21"; plan.md:871-872 reads it for RS plus five CAX21s, and R5 needs up to four on Mon 14 Dec | u8-brief.md:254 | C7 |
| ADV-U8-8 | Medium | SEC-R1D-2's third part, "the laptop's firewall blocks inbound" (security-r1-deps.md:206), is not carried | u8-brief.md:112-113, :252-259 | C8 |
| ADV-U8-9 | Medium | Stop B's round has no netcode seat, yet R1.md holds the judged instruments' rules | u8-brief.md:309-317 | C9 |
| ADV-U8-10 | Low | Q-U8-6 (i) sends the knob to U6c (R1's critical path), though its consumer runs after R1 | u8-brief.md:349-352, :365 | C10 |
| ADV-U8-11 | Low | LG is created about 09:15-09:20 and deleted about 13:25: past 4 h, billed 5 (inside the cap) | u8-brief.md:265, :269, :277 | L1 |
| ADV-U8-12 | Low | The operator deny `node .gate/*` (plan.md:1318) refuses `node .gate/npt-run/…/smoke.mjs`; PROMPT-R1.md's start lacks §12.3's checks (plan.md:1301-1306, :1310, :1328-1329) | u8-brief.md:103-106 | L2 |
| ADV-U8-13 | Low | The stale list misses plan.md:940, :969, :1110, :1140-1141, :1144, :1215 | u8-brief.md:19-22 | L3 |
| ADV-U8-14 | Low | systemd may refuse `AllowedCPUs=` on init.scope (not verified here); `alone` would then read 0 | u8-brief.md:54-57 | L4 |

## 7. Conditions (paste-ready; the main session applies them word for word to u8-brief.md, rev 1.1)
**C1 (ADV-U8-1, High), sections 1 Block 5 and 6.2.** "Every command the laptop session runs prompts him (Manual
mode, plan.md:1304-1305); the operator settings add no ssh allow, and none is added. So while he is away the session
runs nothing. `r1.sh chain <name>` runs a fixed list of blocks in order on RS, detached from the SSH session (it
survives the disconnect, holds each room's stdin itself, and writes a done or stopped marker per block); the laptop
session starts a chain with one prompt before he leaves and collects after he is back. R1's server chain: headers,
probe, calibrate, starts, ds-selftest, s14 (bites where R1.md places them). The first commands (§7.3 step 7 and
§12.3's start checks), the RS-to-LG key install, `npt-ctl peers`, `npt-ctl cpus` and `r1.sh bringup` run once
cloud-init's build ends, with him present. If `alone` reads 0, bring-up records every task it lists and R1 goes on
with the certificate, the browser legs and the ping; the timing blocks are recorded INVALID with that list and come
to the advisor, so his sittings are not lost. Section 6.2 recomputes his sittings and the total (likely about
2 h 40 min: 09:10-09:32; from the end of cloud-init's build to the server chain's start; 11:40-13:25), and test 5
checks that no block marked away needs a laptop-session command. R2's, R3's and R5's plans use the same chain form."

**C2 (ADV-U8-2), Block 9 and test 4.** "No `.npmrc` anywhere under spikes/: any `.npmrc` is a gate file
(advisor.md:43) and guard-protected (routine-guard.mjs:38). Install scripts stay off by `--ignore-scripts` on every
npm command R1.md, PROMPT-R1.md and section 6.2 name, with `npm_config_ignore_scripts=true` in each command's
environment; test 4 asserts both and that `git ls-files` lists no `.npmrc` under spikes/. security-auditor confirms
this substitute for its npm rule (security-r1-deps.md:245) at Stop B. U7's brief takes the same rule for `web/`."

**C3 (ADV-U8-3), sections 1 Block 9, 4 and 6.** "-29 and PM-0006-25 (suite `cert-14d`) take 20 attempts per
browser, one at a time, each closed before the next (marks.json notes). R1.md states the attempts it runs for these
rough readings (lean: the marks' 20 on each laptop browser, one on Brave), and each attempt joins its own slot: a room
per mark and browser, started through `npt-ctl rooms start`, with a slot per attempt, or the room restarted between
attempts. Section 4's slot count and R1.md's slot table are recomputed. Block 9 adds the `cert-14d` script (each
attempt fetches the hash and opens WebTransport with `serverCertificateHashes`; one `event.attempt.ok` per attempt);
`fallback.mjs` writes one `event.fallback.playing_ns` per attempt and re-arms `npt-ctl quic off` per browser; an
attempt that reaches WebTransport while the block is meant on makes that browser's leg INVALID, never a pass. R1.md
carries condition 2's line word for word."

**C4 (ADV-U8-4), Blocks 1, 3 and 7, test 3, plants.** "U8 builds the RS-to-LG control path of plan.md:1157-1162:
rs.yaml makes `npt`'s ed25519 key on RS (0600, never leaves RS); PROMPT-R1.md's first commands install its `.pub` on
LG for `npt-load` with `from="<RS IPv4>",restrict,command="/usr/local/bin/npt-load-ctl"`; `ops/sbin/npt-load-ctl`
(Block 3's rules: fixed verbs read from `SSH_ORIGINAL_COMMAND`, each argument validated in `npt-lib.sh`, fixed paths)
starts, feeds and stops U6c's npt-load, copies token files and runs C5's check. Its verbs follow the npt-load
interface U6c's brief names. It joins test 3, plant P7 (a free argument passed through, which must fail test 3) and
Stop B's GTFOBins read."

**C5 (ADV-U8-5), Block 5.** "`r1.sh bringup`, before any room opens, proves both Hetzner firewalls are in force: from
LG, a TCP connect to RS's port 22 must fail (nftables accepts 22 from any source, so only `fw-rs` can drop it), and
from RS an ICMP echo to LG must get no answer (`fw-lg` admits ICMP only from his home address). Either reply stops
the run before a room opens and comes to him. The check prints no address outside the VM list."

**C6 (ADV-U8-6), section 6.4.** "MOVED 360-720 and U6j 1,650-3,050 are estimates (replan.md:20; replan-units.md:63),
so at 1.35-1.5 they are about 490-1,080 and 2,230-4,580. The likely total is about 8,110-13,010, middle about
10,560: 6.9, 9.0 and 11.1 lane days at 1,175 a day. U6e merges about Thu 10 (Fri 11 at the top), U6j about Sat 12
(about Tue 15 at the top). The calendar stays (R2 Sat 12, R3 Sun 13, R5 Mon 14, Tue 15 spare, the close Fri 18) and
holds at the middle. The three lines read: 1. At the top, R2 and R3 still run on the weekend after U6e, but R5 waits
for U6j (Wed 16), and the close moves to about Mon 21. 2. A lever then is the main session's (for one, MOVED built
beside R1 on Thu 3). 3. Each lost day uses Tue 15; a second moves the close a day. Test 5 recomputes these from the
cited lines."

**C7 (ADV-U8-7), section 6.2.** "The Limits tab is read for the largest run, as plan.md:871-872 asks: RS plus five
CAX21s and two Primary IPs (R5 needs RS plus up to four), so any increase asked at Stop B is in place by Mon 14 Dec."

**C8 (ADV-U8-8), section 6.2 and Block 7.** "SEC-R1D-2's third part: at Stop B and at each start, the laptop session
reads (and does not change) the laptop's firewall profile; inbound connections must be blocked on the active network
profile, else it comes to him before any driver starts."

**C9 (ADV-U8-9), section 7.** "Stop B's round adds netcode-architect (xhigh) on R1.md's judged blocks, S14's rig and
compare.py, and the probe block's presence and INVALID checks; three agents at once, one round."

**C10 (ADV-U8-10), sections 9 and 10.** Replace Q-U8-6 (i)'s lean and section 10's U6c line with section 2 of this
ruling ("rides in MOVED's brief … tested now on fixture KCMODEL lines … never NOT-RUN"), and write section 1 of this
ruling's terms into Q-U8-1 (the six Stop B confirmations), Q-U8-4 (one process; every authoritative server), Q-U8-6
(ii) (the judged form) and Q-U8-7 (read as `npt-room`). U6c's brief then takes Q-U8-4's interface, Q-U8-5's reads,
the probe's ports and npt-load's control interface for C4; not the knob.

**Lows (fold with the conditions; none alone would need a push):**
- L1: each VM gets its own delete-by (its creation + 4 h); teardown deletes LG first (u8-brief.md:277).
- L2: PROMPT-R1.md runs the browser scripts from their folder or by absolute path (each a prompt), since `node
  .gate/*` is denied (plan.md:1318); its first commands include §12.3's start checks (plan.md:1301-1306, :1310,
  :1328-1329).
- L3: the stale list adds plan.md:940 and :969 (revocation), :1110 (`replaceState`), :1140-1141 (`revoke` and the
  room-unit controls on the sudo list), :1144 (one user per slot), :1215 (the five-browser check).
- L4: `npt-ctl cpus` falls back to `taskset` on PID 1's threads where systemd refuses `AllowedCPUs=` on init.scope,
  recording which path it took.
- L5: Q-U8-1's confirmation 1 (anchored sudoers patterns) holds for every sudoers line, not only the rooms verb.

## 8. What I could not check
- No VM: cgroup cpuset behaviour on the CCX23 image, init.scope's `AllowedCPUs=`, netem's packaging and Hetzner's
  per-hour billing edge are reasoned from documentation and stock Linux; C1's `alone` path and L4 cover the misses.
- Whether Claude Code on native Windows prompts for each `ssh` exactly as in Linux Manual mode: I read the rules
  (plan.md:1304-1325; main's settings.json allow list has no `ssh`), not a Windows session.
- The marks' attempt counts for R1's rough readings are R1.md's to state (C3's lean); marks.json fixes only the ML
  parts' 20.
- Container checks run: Ubuntu 24.04.5, util-linux 2.39.3-9ubuntu6.6 (Essential, required) owns /usr/bin/taskset;
  `nft -c -f` works here; sudo 1.9.15p5; PR #41 merged 2026-11-06T13:35:37Z (REST API).

## Decisions needed
None now. His run days, his time in R1 and R1's cap come to him in the day's message and at Stop B (section 4).

## For Mustafa
Nothing from this check. The day's message carries the planned dates (R2 Sat 12, R3 Sun 13, R5 Mon 14 from about
06:45 to 23:30; close Fri 18, about Mon 21 at the top sizes) and R1's presence (about 2 h 40 min in three sittings).

DONE
