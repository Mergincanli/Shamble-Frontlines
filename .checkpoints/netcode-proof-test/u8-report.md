# U8, the run package for R1 (the first real run on rented servers): the merge report
Merged into the lane on 2026-11-26 by the main session, plan u8-merge-plan.md (U4f's M1-M9 form, the advisor tool's
two reads folded in; allowed by MAIN-66's check Q1), on the second attempt (the first stopped at M5: below). Brief
u8-brief.md rev 1.1; the builder's notes u8-build.md and u8-fix-m5.md; the rulings advisor-u8-stopb.md and
advisor-u8-m5-ruling.md. The advisor (a3de649cd25e474df) reads the pushed head in the 09:50 UTC window.

## What it is
43 new files, all under spikes/netcode-proof/ops/ (nothing else changes):
- browser/ (6): config.mjs (one home for the URL contract with U7's page, the header and footer builders), lib.mjs
  (WebDriver sessions, read(), save(), runInfo()), smoke.mjs, cert14d.mjs, fallback.mjs, mds36.mjs;
- runs/ (5): R1.md (the run plan's operator text), r1.sh (the run script), r1-dist.sha256, s14/probe.py, s14/compare.py;
- sbin/ (10): npt-ctl, npt-lib.sh, npt-cert, npt-capture, npt-load-ctl, npt-s14, npt-userdata, npt-ns.txt and
  sudoers.d/npt, sudoers.d/npt-lg;
- cloud-init/ (2), firewall/ (2), systemd/ (3), operator/ (4: PROMPT-R1.md, rules.md, settings.json, pre-push),
  checklists/operations.md;
- tests/ (10): nine node test files and wrappers.test.sh.

## Size and carries
6,189 added lines: 6,170 at C9's end plus 19 by the M5 fix (test 7 +19; r1.sh one line changed), 11 under the 6,200
stop set for U8 (history.md:7703-7722); nothing carried.

## The first attempt, its stop and the fix
- 2026-11-26 00:34-00:56 UTC, on H0 37badaf with the C9 patch (09cb4a07...e660, base 096808b): check.mjs all 11 steps
  passed, but U8's ops tests 99 of 100: test 7's item 8 (r1sh.test.mjs:226-227) pinned U6a's 27 settings fields and
  the merged server's Settings holds 40 (U6b's and U6c's fields): the alignment test 7 exists to catch. M6 undid the
  apply (`git apply -R --index`), nothing committed.
- The advisor's ruling (advisor-u8-m5-ruling.md): FIX, then the merge again from M0. It found a High: r1.sh's
  room_settings wrote the WebSocket certificate pair as null and the merged server takes strings, so every non-browser
  R1 room would have refused its settings.
- U8's builder (ac59cce3e5d002441, on the lane head 03d9ff3; u8-fix-m5.md): r1.sh:230 writes U6b's empty pair (`""`,
  main.rs:119-122); item 8 rewritten as two tests against the merged 40-field struct (the pin, and every required field
  of the type the struct takes, `cc` absent), red on the null pair before the fix; test 2.2 pins the empty pair; plants
  P8 (a required field dropped), P9 (a null value) and P10 (a struct field added) each caught, all restored. End
  checks there: ops tests 101 of 101, wrappers.test.sh 315 ok, redact 421 items, 0 findings.

## Evidence on the merged tree
- M1: wip/u8-build.patch sha256 3e1cd5b53d4c224020233f8e7edc3a1ec3b795edc729dd257101fff0742ea531 (377,866 B, 43 files,
  base 03d9ff3; removed by this merge, WIP-CLEAN). M3/M4: `git apply --check --index`, then `git apply --index`, on H0
  9aa6dde; the 43 files byte-equal to the fixed tree in U8's worktree (agent-ac59cce3e5d002441: `diff -rq` exit 0).
- C1 (advisor-u7-brief.md:118-119), a form test only, from the first attempt (it reads no file the fix changed): U7's
  T5 (ii) run 3, leg 6-mds (a container loopback leg, U7's T-builder tree 7e2d6f3 plus its unsaved R-half, t5-dist of
  19:35 UTC), written through U8's save() as R1, eu-path and ML*; npt-judge with PM-0001-36 read it with no read error:
  NOT-RUN (the ML and MAC-b parts empty), its "ML* (rough, reported)" part PASS. The log stays under .gate/ only.
- M5 (one counted run through main-run.sh, lane lock, incremental off, cold cache; .gate/u8-merge/m5-check-r.log,
  05:19:47-05:41:33 UTC, exit 0): U8's tests `node --test 'ops/**/*.test.mjs'` 101 of 101, 0 fail; `bash
  ops/tests/wrappers.test.sh` "315 ok, 0 failed"; `node check.mjs` all 11 steps PASS: test 861 measured and 958 bites,
  0 failed, 0 skipped; wasm 27,000 ticks, 0 mismatching; refs 1,982 tracked files (H0's 1,939 + U8's 43); marks 81,
  rows unchanged; self 78 of 78; redact 11 of 11 with 421 items (378 + U8's 43), 0 findings. Test 7's merged-state run
  is inside it (it reads the post-U6c server and super main.rs).
- M7: the node_modules link removed (the install in agent-abf71d95fc07b3522 kept); `node tools/gate.mjs --no-smoke`
  GATE PASS; `node .claude/scripts/repo-map-check.mjs` "repo map OK: 207 entries cover 110 required paths"; `git status
  --short`: the 43 paths staged (A) and notes only.
- D10: no marks.json or crates/judge change (the merge touches ops/ only), so nothing for his read of judge changes.
- DEP-HOLD: no manifest or lockfile in the merge. ops/browser's package.json 8370495d... and package-lock.json
  a0efce8e... (webdriverio 10.2.0 exact) were cleared before U8's launch (dependency-check.md) and are unchanged.

## Choices taken without him (for the day message)
- 15. The `ops` step in check.mjs waits until after U7's merge (u8-brief.md:524: "merge, then the `ops` step"): U7's tree
  changes the same step-list pin (check.test.mjs:2785), so adding it now would break U7's apply check. Until then U8's
  tests run beside check.mjs in the merge's counted run, with ops/browser/node_modules linked, not installed.

## Open at the merge (with their takers)
- From advisor-u8-stopb.md: F28 (U6b's flush, proven live at R1); ds-selftest's place in R1's run list; U6c's names
  (NC-R2-4, read by test 7 on the merged code: M5 above); SEC-SB-9's rest (the page host's sandbox options, from U6c's
  host needs, else carried to R2 with R1's `systemd-analyze security` reading); SEC-SB-5's two-path dist build (at U7's
  merge, before Stop B's fill). U6b's null certificate pair: CLOSED by the fix above.
- Stop B's fills and reads: the laptop's reads (driver listen, passphrase, fsn1 stock, Claude Code >= 2.1.257),
  `sudo -l` over ssh on RS (only the container's read so far), SEC-U6C-5's URL, rs.yaml's commit line (Y).
- NC-R3-1 (Low): r1.sh:438-452, save_log unguarded under set -e: a follow-up.
- For the advisor's head read: the policy-name INVALID (an unasked addition beyond the four reads, plan.md:1256), and
  the fix's two files (r1.sh, ops/tests/r1sh.test.mjs) with plants P8-P10, which no review seat has read.
- The `ops` step and how later trees get webdriverio (an install by U8's one form, or the step reporting its absence):
  at that commit's head read.
- Not run on a VM yet; `node --test ops/` fails on Node 22.23: use the glob form.
