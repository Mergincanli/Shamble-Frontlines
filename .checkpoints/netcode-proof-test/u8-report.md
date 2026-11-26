# U8, the run package for R1 (the first real run on rented servers): the merge report
Merged into the lane on 2026-11-26 by the main session, plan u8-merge-plan.md (U4f's M1-M9 form, the advisor tool's
two reads folded in; allowed by MAIN-66's check Q1). Brief u8-brief.md rev 1.1; the builder's notes u8-build.md; the
rulings advisor-u8-stopb.md. The advisor reads the pushed head (a3de649cd25e474df, in the 04:50 UTC window, with
MAIN-67's step 7).

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
6,170 added lines at C9's end, under the 6,200 stop set for U8 (history.md:7703-7722); nothing carried.

## Evidence on the merged tree
- M1: wip/u8-build.patch sha256 09cb4a07928cf2a48c8516df18d68fd45e0b22a6666e04b65610934a9a72e660 (removed by this merge,
  WIP-CLEAN). M3/M4: `git apply --check --index`, then `git apply --index`, on H0 37badaf; the 43 files byte-equal to the
  C9 tree in U8's worktree (agent-ac59cce3e5d002441).
- C1 (advisor-u7-brief.md:118-119), a form test only: U7's T5 (ii) run 3, leg 6-mds (a container loopback leg, U7's
  T-builder tree 7e2d6f3 plus its unsaved R-half, t5-dist of 19:35 UTC), written through U8's save() as R1, eu-path and
  ML*; npt-judge with PM-0001-36 read it with no read error: NOT-RUN (the ML and MAC-b parts empty), its "ML* (rough,
  reported)" part PASS. The log stays under .gate/ only.
- M5 (one counted run, .gate/u8-merge/m5-check.log): M5_RESULT
- M7: M7_RESULT
- D10: no marks.json or crates/judge change (the merge touches ops/ only), so nothing for his read of judge changes.
- DEP-HOLD: no manifest or lockfile in the merge. ops/browser's package.json 8370495d... and package-lock.json
  a0efce8e... (webdriverio 10.2.0 exact) were cleared before U8's launch (dependency-check.md) and are unchanged.

## Choices taken without him (for the day message)
- 15. The `ops` step in check.mjs waits until after U7's merge (u8-brief.md:524: "merge, then the `ops` step"): U7's tree
  changes the same step-list pin (check.test.mjs:2785), so adding it now would break U7's apply check. Until then U8's
  tests run beside check.mjs in the merge's counted run, with ops/browser/node_modules linked, not installed.

## Open at the merge (with their takers)
- From advisor-u8-stopb.md: F28 (U6b's flush, proven live at R1); U6b's null certificate pair; ds-selftest's place in
  R1's run list; U6c's names (NC-R2-4, read by test 7 on the merged code: M5 above); SEC-SB-9's rest (the page host's
  sandbox options, from U6c's host needs, else carried to R2 with R1's `systemd-analyze security` reading); SEC-SB-5's
  two-path dist build (at U7's merge, before Stop B's fill).
- Stop B's fills and reads: the laptop's reads (driver listen, passphrase, fsn1 stock, Claude Code >= 2.1.257),
  `sudo -l` over ssh on RS (only the container's read so far), SEC-U6C-5's URL, rs.yaml's commit line (Y).
- NC-R3-1 (Low): r1.sh:438-452, save_log unguarded under set -e: a follow-up.
- The policy-name INVALID (an unasked addition beyond the four reads, plan.md:1256): for the advisor's head read.
- The `ops` step and how later trees get webdriverio (an install by U8's one form, or the step reporting its absence):
  at that commit's head read.
- Not run on a VM yet; `node --test ops/` fails on Node 22.23: use the glob form.
