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
- From advisor-u8-stopb.md: F28 (U6b's flush, proven live at R1); ds-selftest's place in R1's run list; U6c's names:
  test 7's item 4 reads fixtures; on real output, the head read's run of probe_presence on U6c's 24 Nov log (P0
  present, P1 at the drop rule); SEC-SB-9's rest (the page host's sandbox options, from U6c's
  host needs, else carried to R2 with R1's `systemd-analyze security` reading); SEC-SB-5's two-path dist build (at U7's
  merge, before Stop B's fill). U6b's null certificate pair: CLOSED by the fix above.
- Stop B's fills and reads: the laptop's reads (driver listen, passphrase, fsn1 stock, Claude Code >= 2.1.257),
  `sudo -l` over ssh on RS (only the container's read so far), SEC-U6C-5's URL, rs.yaml's commit line (Y).
- NC-R3-1 (Low): r1.sh:438-452, save_log unguarded under set -e: a follow-up.
- For the advisor's head read (read: advisor-u8-head.md): the policy-name INVALID (an unasked addition beyond the four
  reads, plan.md:1256), and the fix's two files (r1.sh, ops/tests/r1sh.test.mjs) with plants P8-P10.
- U7's alignments (ADV-U8H-4), as found:
  - `node build.mjs` in cloud-init/rs.yaml, read at this merge (2026-11-27): :198-200 build in the spike's web/ with
    `npm ci --ignore-scripts`, then `node build.mjs` (U7's web/build.mjs), after :194's wasm-bindgen-cli 0.2.130, the
    version build.mjs pins. Not run here: the two-path dist build before Stop B's fill runs it (SEC-SB-5).
  - The URL contract in ops/browser/config.mjs: not verified. No run has driven the real page through config.mjs's
    URLs: U7's T5 (ii) driver builds its own (web/e2e/t5ii.mjs:290, its `link()`); web/test/lib.test.mjs:207 pins only
    the status texts U8's PAGE reads; U7's merge's C1, like U8's (line 40 above), was a form test of save() against the
    judge. Proposed, for the advisor's head read of this merge to rule: the main session runs U8's smoke.mjs against
    U7's dist in the container before Stop B's fill (it writes no U8 file), else R1's first leg is the first such run.
    ADV-U7S-5's stale comment at config.mjs:60 stays open (U8-F1 does not touch config.mjs).
- The `ops` step and how later trees get webdriverio (an install by U8's one form, or the step reporting its absence):
  at that commit's head read.
- Not run on a VM yet; `node --test ops/` fails on Node 22.23: use the glob form.

## U8-F1, merged 2026-11-27 (the head read's fix list and its security round)
Merged into the lane by the main session, plan u8-f1-merge-plan.md (U7's M0-M9 form with M3a), checked by the advisor
(a3de649cd25e474df: APPROVE WITH CONDITIONS C1-C3, advisor-u8-f1-merge.md, applied word for word). Brief u8-f1-brief.md
(C1-C7 applied); the builder's notes u8-f1-build.md (a7fe1b9f436f36bb1, on 77ffc95); snapshot wip/u8-f1-build.patch
4d6a2484…7164 (6 files; removed by this merge, WIP-CLEAN). The commit: the merge commit on H0 833c3f0 (its id is in
the plan's M8 done line and on the card).
- What it built, on six files under spikes/netcode-proof/ops/ (operator/PROMPT-R1.md, runs/R1.md, runs/r1.sh,
  tests/r1plan.test.mjs, tests/r1sh.test.mjs, tests/s14.test.mjs):
  - item 1 (ADV-U8H-1 under his RESULTS-PUSH (b)): probe_presence prints P1's l2_summary fields and the four P1 reads,
    integers only, into probe/presence.txt (the policies as number and priority); ds_selftest writes ws_split's fields
    into ds-selftest/ws_split.txt; the pushed list, the same in PROMPT-R1.md and R1.md, with a test that the two are
    equal; his S14-PUSH (a): the s14 folder's own redaction run before the push, its output saved as s14-redact.txt in
    the pushed set, R1.md's sentence naming every writer into the folder, and the pin test (s14.test.mjs) refusing any
    other writer, key or address by name;
  - item 2 (SR-1 with SR-2): PROMPT-R1.md's PowerShell check that `git -C <results> rev-parse --git-path hooks` prints
    the clone's own hooks folder before the push, and R1.md's Stop B start-check line; its four-way proof was a scratch
    run (u8-f1-build.md), not committed;
  - item 3 (ADV-U8H-2): R1.md's clause that the advisor applies plan.md:1256's slack-value rule at R1's report.
- The fix pass after the security round (security-u8-f1.md; the ruling advisor-u8-f1-sec.md): SEC-U8F1-1's fixes (a)
  one exact list for npt-s14's `$OUT` lines and a wider writer filter for r1.sh's s14 block, (b) host names, interface
  names and dash-form hardware addresses in the address loop, stamps.txt's writers read, (c) S14-VMS: the s14 folder's
  own check runs with no VM list; SEC-U8F1-2's parts (the shared "goes to Mustafa first" message, the one-loop count);
  item 6's save wording (`| Out-File -Encoding utf8 <results>\r1\s14-redact.txt`, "not `>`, which writes UTF-16").
- The size: +93 net (105 added, 12 removed) of 93; U8 at 6,282 of his 6,282 stop (U8F1-SIZE (a)).
- S14-VMS: the advisor's ruling: inside his "the same redaction check" (the same program, only stricter); his to
  reverse (choice 25).
- The carries: SEC-U8F1-3's "in the same PowerShell" wording and SEC-U8F1-4's Stop B record of the printed hooks path
  and `git --version` (C2: on PR #40's closing list now, as security findings left open; no fill and no building of
  them until he answers U8-FILLS); logq's non-object line (a STATUS follow-up); the earlier Lows (re-runs keep the last
  attempt's prints only).
- The proofs, by pointer: u8-f1-build.md's "# FIX PASS (security round) HAND-BACK" (each item's red run, its plant and
  the restore hash); item 5's scratch proof with private stand-in addresses (the fixture's documentation addresses are
  allowed anyway, so they cannot show the gap).
- The rounds and rulings: the head read: advisor-u8-head.md, APPROVE WITH FINDINGS; ADV-U8H-1 to -4; SR-1 and SR-2
  weighed Low; his RESULTS-PUSH (b) and S14-PUSH (a), at history.md:8068-8106; the brief check:
  advisor-u8-f1-brief.md, C1-C7; the security round: security-u8-f1.md; its ruling: advisor-u8-f1-sec.md; the stop's
  moves: 6,230, then 6,282 by U8F1-SIZE (a).
- What leaves the server now, for his Stop B read: `collect/`, `probe/presence.txt`, `ds-selftest/ws_split.txt`,
  `bringup/facts.txt`, `s14/` whole and `s14-redact.txt`. The new prints are integers only, and the policy reads are
  numbers. The s14 folder's own check runs with no VM list (S14-VMS), and the hook checks the whole push with it.
- D10 and DEP-HOLD: no marks.json or judge change; no manifest or lockfile; ops/browser's package.json and
  package-lock.json unchanged.
- The first run against the merged trees: the builder's end checks ran at 77ffc95, so M5 is U8-F1's first run against
  NB2's and U7's merges.
- The security findings left open, for PR #40's closing list: SEC-U8F1-3 and SEC-U8F1-4 (C2); SEC-SB-9's rest, as
  before. logq's non-object line is a STATUS follow-up, not a security finding.
- The two wordings for his Stop B read (u8-f1-build.md, "Open"): "runs on `<results>\r1\s14` with no VM list"
  (S14-VMS, choice 25); the save's shorter reason.
- M5 (one counted run through main-run.sh, the lane lock, incremental off, cold after the part-clean;
  .gate/u8-f1-merge/m5-check.log, 06:13:46-06:36:32 UTC, exit 0, HEAD 833c3f0; the run's diff hash 4d6a2484…7164 = the
  patch's, no untracked file): `node --test 'ops/**/*.test.mjs'` 104 of 104, 0 fail (U7's M5's 101 + U8-F1's three);
  `bash ops/tests/wrappers.test.sh` "315 ok, 0 failed"; `node check.mjs` all 12 steps pass: test 931 measured and
  1,028 bites passed, 0 failed, 0 skipped; web 70 of 70; wasm 27,000 ticks native-native and native-Wasm, 0
  mismatching, the hash-map bite caught at SS-s1 tick 1234 (all = U7's M5); self 80 of 80, marks 81 (rows unchanged),
  refs 2,145 (= N0), redact 11 of 11 with 464 items, 0 findings (all = M0's base run, by C1).
- M7: the node_modules link removed (the install in agent-abf71d95fc07b3522 kept); `node tools/gate.mjs --no-smoke`
  GATE PASS (base b9aad01); `node .claude/scripts/repo-map-check.mjs` "repo map OK: 245 entries cover 110 required
  paths"; `git status --short`: the 6 paths staged and notes only.
