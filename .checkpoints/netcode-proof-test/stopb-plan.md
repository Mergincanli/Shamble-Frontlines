# Stop B's preparation: the owed items, the two notes, Y0, Y, the dist hashes, the fill X, the message (rev 0.2, 2026-11-27 22:1x UTC)
Status: rev 0.3: rev 0.2 (B6's REVISE applied: advisor-stopb-plan.md, COMPLETE 22:00 UTC; one High, STOPB-1) passed
B6's re-check at 22:12 UTC with R1-R3, applied here [R1]-[R3]. Tags: [C1]-[C7] B6's, [T] the advisor tool's (22:0x).
Y0 is cut only when A2 has reported (a fix after Y0 costs new Y0, Y, X and both builds).

## Why now
- His NO WAITING FOR WINDOWS (history.md:8163-8186) and his 21:34 and 21:35 words (history.md:8241-8257): continue
  after the planning, with no break. Stop B is Tue 1 Dec (R1.md:57; u8-brief.md:409-415); R1 is Thu 3 Dec.
- R1.md section 2 step 8 and section 8: before his OK at Stop B, the main session fills Y and X, the Subscriber
  Agreement's URL, the expected dist SHA-256s, and re-reads hetzner.com's two price pages.
- U8-FILLS (a) (history.md:8137): the fills sit outside U8's stop; the two notes are built at the fill and count (6,284).
- [C6] Risk tier 1 (spikes/ code and tests, notes, one PR body edit; no kit, CI, gate, secret or money). Risks:
  STOPB-1's commit pair; a move on main before R1; disk for two dist builds; the proxy for fetch and npm; unequal
  builds; A2 finding a fix past his stop; TE-5 not runnable here.

## Step 0 (each printed)
Main still b8f02b0 (MAIN-52: a move with code, CI or rules comes first); branch = origin, clean; no build or run
going; free disk; usage read.

## A. The owed items, before Y0
- **A1, the static link check: DONE 22:06** (web step passed, .gate/stopb/a1-web.log): 133 links, 0 mismatches, exit 0
  (.gate/stopb/a1-links.log); the count equals the slots' sum (smoke 8 + cert 61 + fallback 61 + mds 3); a smoke
  link carries `smoke=1`, which parseLink takes, returning exactly its seven keys; a planted wrong transport gives 4
  MISMATCH lines and exit 1 (.gate/stopb/a1-plant.log). No config.mjs fix.
- **A2, Stop B's host read** [C6]: one security-auditor seat (opus, xhigh), launched 22:0x with section 1's reading
  list and B's two notes' exact text, into stopb-a2-security.md. Each item: fixed where (file, owner, stop) or carried
  with his word. A fix past 6,284 or in crates/host goes to him as a decision before X; a fix in a file the servers
  build comes before Y0.
- **A3 DONE 22:2x.** Read 1: u6c-r2-netcode.md:90-93 ("Nothing moves 122 s or 0.20 h"), :237-238 (the bound: one-way
  plus half jitter under about 9 s); crates/l1/src/lib.rs:177-202 gives RTT and jitter of P2 100 +-20, P3 150 +-30,
  P4 200 +-40, B1 100 +-20 ms: even a full RTT plus its jitter (at most 240 ms) is far inside, so the 0.20 h stands
  (how the probe halves RTT into one-way was not read; the bound holds either way). Read 2: NC-SB-16's count is
  server/src/probe.rs:84-91 `window_total` (send at interval/2 + i x interval until `packets` lie outside the burst),
  L1's loop at lab/src/probe.rs:124-131 in its logic, pinned on a burst profile by probe_tests.rs:59-78
  (`window_total(1_000_000, 100_000, Some((50 * SEC, SEC)))` = 1,010,000, "B1: the burst's 10,000 on top").
- **A3, two reads, no run** [C4]: (1) the existing reading (u6c-r2-netcode.md:90-93, :162) with its limits: P0 and P1
  only, before H1; P2-P4's and B1's one-way delay plus half jitter, each read from its profile file, against the
  ~9 s bound (:237-238); probe and load code since 17964a3d: only load.rs's MAX_LEAD (U7). (2) NC-SB-16's pin:
  server/src/probe_tests.rs:59-78 beside probe.rs:82-92 and lab/src/probe.rs:124-131, quoted. R1.md:321's stale "no
  reading exists yet" is named in his message, not edited.
- **A4, main's tip** at Y0, Y and X; a move by MAIN-52 first. Re-fill triggers [Q5]: (1) any commit changing a path
  under spikes/netcode-proof means a new Y0, Y and X (the dist rebuilt only if its inputs moved, else build 2 redone at
  the new Y); (2) main's CLAUDE.md or .claude moved and taken in means a new X naming Y. A new X after his OK goes to
  him with its two raw URLs. On Stop B's day: `git diff --name-only X origin/<branch> -- spikes/netcode-proof` and
  `git diff --name-only X origin/main -- CLAUDE.md .claude` printed; the price pages and the terms link (XH-1) re-read.
- **A5, owed items B6 added** [C5]: ADV-U8H-3 DONE (adv-u8h-3-read.md: the clock thread is never on the tick vCPU; the
  skip stands for R1; a netcode follow-up after R1, named in his message). TE-5 (u7-report.md:190-191) runs before X
  in a disposable worktree under .gate/ [T]: t5i.mjs:108's ignoreHTTPSErrors flipped there only, never in the tracked
  file; one B-ws load; handshake_failed and accepted recorded; time-boxed to 45 minutes, else its reason recorded and
  R1's first WebSocket leg reads it. NC-3 SATISFIED (B6's read). ADV-U7S-6 SATISFIED, repeated at X (`git diff --stat
  9c2fcf4 X -- r1.sh load.rs` against today's 17/6).
- **TE-5 DONE 22:49** (worktree .gate/stopb-te5 at Y0, t5i.mjs:108's flag flipped there only, restored, removed; the
  release npt-server 0db5ca7c...; /opt/pw-browsers/chromium; NPT_WEB_OUT = the main tree's web-check): B-ws without
  ignoreHTTPSErrors: "connect starts@0, chosen: failed@12"; the room's ws_accept: handshake_failed_count 1,
  accepted_count 0. A-wt and D-real-page accepted 1 over WebTransport; C-wt-wrong handshake_failed 1 (its control).
  Reading: the "handshake_failed 1 beside accepted 1" of every WebSocket load with the flag on is Chromium's refusal
  of the room's self-signed certificate before the flag's retry; with R1's issued certificate a WebSocket leg should
  show none. Kept: .gate/stopb/te5/ (summary.json 0b74e425..., B-ws-room.jsonl 30055e38..., B-ws-contact.json
  e365810e...).
- **A6, the commit-pair read** [T] (blocks Y0): grep PROMPT-R1.md, R1.md, r1.sh, npt-userdata and both cloud-init
  files for any check of what X adds to Y (a `diff --name-only` against a list, a user-data comparison, a file count,
  a parent or ancestry check), knowing saves put notes-only commits between Y and X. One that fails with the list at Y
  or with those commits is a U8 line in Y0, to him first. None found: one line for B6.

## B. The two notes and the stand-ins: Y0
- **B1 DONE 22:15** [R1]: worktree .gate/stopb-dry at 9897cb37; ops/browser installed in the cleared form (`npm ci
  --ignore-scripts --no-audit --no-fund --prefix <its ops/browser>`, npm_config_ignore_scripts=true; 167 packages).
  Base: self 88/88, ops 104/104, wrappers 315 ok 0 failed, redact 464 items 0 findings, refs 2188 (= the real tree's
  counted run, .gate/um4-fix/counted.log). Red (.gate/stopb/dry-f0-red*.log): the three fills without stand-ins fail
  exactly two tests by name, browser.test.mjs:169 "item 9 ..." ('Missing expected exception.') and r1sh.test.mjs:424
  "2.9 SEC-SB-5 ..." (no "the expected list is empty"). Green with the notes and stand-ins: X, Y and Y0 shapes all
  4/4, the same counts (dry-x.log, dry-y.log, dry-y0.log); redact passes on the real URL. Numstat at X: rs.yaml and
  lg.yaml `1 1`, R1.md `3 2` (note `1 0`, URL `2 2`), PROMPT-R1.md `2 1`, r1-dist.sha256 `7 0`, stand-ins `1 1` each.
  The worktree is removed after its files are restored (no --force).
- **B1, the full dry run** [C2][T]: in a git worktree under .gate/ (not a copy, so refs and redact read real git state),
  three shapes, each through `node check.mjs --only self,ops,redact,refs`:
  Y0 (the notes and stand-ins, unfilled); Y (plus U7's real seven list lines, u7-report.md:134-141, commit zeros);
  X (plus any 40-hex Y in both cloud-init files and the real URL). The redact item count and wrappers' 315 match the
  real tree's. Every failure is named; each fix lands in Y0. Red first: the two pins fail by name on the filled
  shapes before the stand-ins, and pass on all three shapes after.
- **The texts:** SEC-U8F1-3 (u8-f1-build.md:378-379): PROMPT-R1.md:129's "Then push to `claude/npt-results` ..."
  becomes "Then, in the same PowerShell, push to `claude/npt-results` after the pre-push hook passes (a prompt).",
  re-wrapped as one more line. SEC-U8F1-4's second part (:380-381): a new line after R1.md:88, "   Its printed path
  and `git --version` are recorded." (A2's seat may sharpen either wording at the same count.)
- **The stand-ins** [C2]: r1sh.test.mjs:448 runs dist_check on the committed file's comment lines alone (an unfilled
  stand-in); browser.test.mjs:206 builds `rs` from the committed rs.yaml with its commit line set back to zeros. Each
  rewritten in place (`1 1`), no new assert line; they then test the same rules on Y0, Y and X alike.
- **Bar:** numstat net +1 in PROMPT-R1.md and net +1 in R1.md (U8 at 6,284); the two stand-ins `1 1` each; nothing
  else under spikes/; a re-wrap changes a line in place only as B6's section 2 allows. The ops tests, wrappers "315 ok,
  0 failed" and redact 0 findings in the counted run. A net line more, or a change to redact-check.mjs (with the
  pre-push hook's pinned hash), stops before the edit and goes to him.
- **B2 DONE: Y0 = 87cb8677d51535ea5000f2fbb7e7b2f86144dbee** (22:2x; 4 files, +5 -3: PROMPT-R1.md `2 1`, R1.md `1 0`,
  the two stand-ins `1 1`); counted run .gate/stopb/y0-counted.log (diff 2479dba3..., re-read equal before the
  commit); gate .gate/stopb/y0-gate.txt PASS; A2 reported first (no server file moves).
- **B2, Y0:** applied by Edit; the counted run `--only self,ops,redact,refs` through main-run.sh; the gate; one commit
  naming both notes and both stand-ins; pushed by the save; CI green.
- **The closing list** [C3]: DONE 22:10 UTC: PR #40's body read whole, only its list changed (SEC-U8F1-3, SEC-U8F1-4,
  SEC-SB-9's rest, each with its source, and one line that they were missed since U8-F1's merge), written back and
  re-read (7 entries; nothing else changed; still a draft). At B2: -3 and -4 off, "closed by Y0 <sha>";
  SEC-SB-9's rest stays until A2 settles it. The report says the list missed them since U8-F1's merge.

## C. Y and the expected dist hashes
- **Build 1 DONE 22:31** at Y0 (.gate/stopb-a, CARGO_HOME .gate/stopb-a/.gate/cargo; log .gate/stopb/build1.log):
  rustup 1.29.3, rustc 1.99.1, a fresh wasm-bindgen 0.2.130, node v22.23.4, npm 10.9.9; seven files; free 7,749 ->
  6,801 MiB. A second named deviation: `rustup toolchain install --no-self-update` (the container's rustup lives
  outside the build's CARGO_HOME, so its self-update failed: first try's log). Two earlier tries stopped before any
  build (a shell quoting slip, then the self-update). Supporting line: six files equal U7's run 4; the wasm differs
  (2a97394b... against 901a6492...), U7's list being its snapshot tree's (B6: not a bar).
  [Correction 02:5x UTC on 11-28, B6's YPH-3: the Wasm differed from run 4's because of STOPB-8's leak (build 1's
  CARGO_HOME sat inside the worktree, so its Wasm carried Y0's name), not because of U7's snapshot tree; build 1' and
  build 2' equal run 4 on all seven files.]
- **STOPB-8 (High, B6's ruling 02:14 UTC on 11-28, advisor-stopb-plan.md "## Ruling on the dist difference"):** build 2
  at Y failed C1's bar; root cause stopb-dist-diff.md: wasm-bindgen-shared's build.rs wrote our commit's first 9 hex
  digits into the Wasm's `producers` because CARGO_HOME sat inside the worktree; RS's CARGO_HOME is outside any
  repository. Y (652e9513) is VOID and is never named in a cloud-init file. The fix, inside C3: CARGO_HOME, the
  wasm-bindgen root and its target outside every repository (scratchpad/npt-dist-a and -b); per build the bars: every
  `GIT_*` name printed and unset; `git rev-parse HEAD` failing in the cargo base and in wasm-bindgen-shared-0.2.130's
  own registry folder; `producers` reading plain "wasm-bindgen 0.2.130" (the reader's red case: dist-c's Wasm prints
  "(87cb8677d)"). Build 1' at Y (or Y0), then Y' = the list's Wasm line replaced in place (`1 1` against Y, `7 0`
  against Y0), then build 2' at Y' itself, equal; the six other lines equal Y's. Y' after MAIN-70. A follow-up after R1
  (B6's Q3): web/build.mjs refuses a hashed `producers` entry, with a test and a plant (the next change to build.mjs).
- **Build 1' DONE 02:15 UTC on 11-28** at Y (.gate/stopb-a; cargo base scratchpad/npt-dist-a; build1p.log): every bar
  met (git fails in the cargo base and in wasm-bindgen-shared-0.2.130's folder; 11 GIT_ names printed and unset;
  `producers: wasm-bindgen 0.2.130`); Wasm 901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77, which is
  U7's run-4 value (u7-report.md:141), made by another builder on another path and day; the six other lines equal Y's
  (diff: line 6 only). Y' (this list) waits for MAIN-70.
- **Y' DONE 02:4x UTC on 11-28: 1ceecc321eaf194d59568fdab56644d1e9eb4f05** (after MAIN-70's merge 2d379d9c, pushed in
  8d1306fe, and its four CI runs all success): r1-dist.sha256 line 10 replaced in place, 2a97394b... -> 901a6492...;
  bars, each alone: `git diff --numstat 652e9513 HEAD -- spikes/netcode-proof` "1 1" on the list alone; against
  87cb8677 "7 0" on the list alone; the non-comment lines cmp-equal to .gate/stopb/build1p-dist.sha256 (exit 0); the
  advisor tool's added bar `git grep -n 2a97394b -- spikes/ docs/` prints nothing (exit 1); counted run
  .gate/stopb/yp-counted.log 02:44:12-25 (self 88/88; ops 104/104, 315 ok, 0 failed; redact 11/11, 464 items, 0
  findings; refs 2241; diff a8182b1e..., re-read equal before the commit; untracked empty); gate
  .gate/stopb/yp-gate.txt PASS. B6's Q4 (e) triggers, both cleared: MAIN-70 changed nothing under spikes/ (M70-3's
  third bar printed nothing) and main's 107 paths hold no kit or CI path (.gate/main-70/main-names.txt, a printing grep:
  nothing), so Y0 stands and X takes in no kit file. Build 1' ran on Y's tree (before MAIN-70), build 2' runs on Y'
  (after it): inside B6's ruling, and their equality also shows #79 is no dist input; if they differ, `producers` is read
  first (plain means main's root files are the suspect, not the commit leak), both lists go to B6, X stays blocked.
- **Build 2' DONE 02:47:25 UTC on 11-28, EQUAL** at Y' (worktree .gate/stopb-b at 1ceecc32, detached; cargo base
  scratchpad/npt-dist-b; log .gate/stopb/build2p.log, list .gate/stopb/build2p-dist.sha256; exit 0, 02:45:57-02:47:25):
  git fails in the cargo base and in wasm-bindgen-shared-0.2.130's registry folder (both "ok" lines); 11 GIT_ names
  printed and unset; `producers: wasm-bindgen 0.2.130`; wasm-bindgen-cli built fresh (1m 15s), the client crates from
  the worktree's own path (182 Compiling lines; build.mjs "measured dist, 7 files"); `cmp` build1p-dist.sha256 with
  build2p-dist.sha256: exit 0; `cmp` of Y''s committed non-comment lines (`git show 1ceecc32:...`) with
  build2p-dist.sha256: exit 0. Build 1' ran at Y (652e9513, before MAIN-70) and build 2' at Y' (after it): equal, so
  neither the commit nor #79 is a dist input. Free after 5,846 MiB. STOPB-8 resolved by the method, for B6's read.
- **B6's Y' head read DONE 02:51 UTC (advisor-stopb-plan.md "## Y' head read", Status COMPLETE): APPROVE WITH
  FINDINGS, three Lows; Y' 1ceecc32 stands as R1's code commit for X; STOPB-8 resolved by the method.** Its one
  condition: CI on the save 232cc8c9 all four success (the main session sends the runs; a failure reopens the read
  before X). Both build lists hash d577c9d7... (`git hash-object`). U7's run 4 is a third agreeing build, quoted as
  supporting evidence, not one of C1's two bars. YPH-1 (Low): the build script prints no `git rev-parse HEAD` from
  inside its worktree; a follow-up for the next build (the script's first log lines). YPH-2 (Low): build1p.log:15's
  garbled summary, fixed in the script since (build2p.log:15 "11 unset above"); record only. YPH-3 (Low): the dated
  correction above (lines 107-108's note). Its line 623 corrected in its own file. Next: X after his HOST answers.
  **Condition met 02:59 UTC:** CI on 232cc8c9 all four success at attempt 1 (gate 46099627026, 46099631890; toolchain
  jobs 140838881813 and 140838895873, about 11 and 10 minutes); B6 told. **Y' stands; STOPB-8 closed.**
- **Y DONE (void, STOPB-8): 652e9513ce2c1369d8d6146e84109feadf021a55** (22:50): r1-dist.sha256 gains build 1's list byte for byte
  (cmp), numstat `7 0` against Y0, counted run .gate/stopb/y-counted.log PASS; pushed by the save f49a7ba.
- **C1, the order** [C1]: Y0 -> build 1 at Y0 (worktree A, CARGO_HOME A) -> Y = Y0 plus r1-dist.sha256's seven lines
  only, under its 4-line comment -> build 2 at Y (worktree B, CARGO_HOME B) -> X = Y plus the two commit lines and the
  URL. Bars [T]: `git diff --name-only Y0 Y -- spikes/netcode-proof` is the list's file alone, `--numstat` `7 0`;
  build 2's list equals Y's committed non-comment lines byte for byte; `git diff --name-only Y X --
  spikes/netcode-proof` is exactly rs.yaml, lg.yaml and R1.md, `--numstat` `1 1` for each cloud-init file and, for
  R1.md, net 0 with its hunk inside :93-94 (`2 2`: the URL cannot join :93 under the wrap; :92-93 until Y0's note at :89, re-printed 06:3x UTC on 11-28) [R2], hunks printed. A difference
  goes to B6 before X with both lists. Y and X each pushed with CI green.
- **C2, the tools** (read before the check, all matching rs.yaml's pins): rustup 1.29.3, rustc 1.99.1 (the spike's
  rust-toolchain.toml), wasm-bindgen 0.2.130 (build.mjs refuses otherwise), node v22.23.4, npm 10.9.9. Re-printed in
  each build's log.
- **C3, the recipe** [C6]: only rs.yaml's dist lines, :193 (`rustup toolchain install`), :194 (`cargo install
  wasm-bindgen-cli --version 0.2.130 --locked`, its CARGO_TARGET_DIR prefix and `--root` in that worktree's scratch
  home), :195 (`cargo fetch --locked`), :198-200 (in web/, `npm ci` and `CARGO_NET_OFFLINE=true node build.mjs`, with
  npm_config_ignore_scripts=true); not :196-197. Outside main-run.sh (its CARGO_NET_OFFLINE and CARGO_TARGET_DIR
  exports would refuse :195); each build its own background command under the lane lock (.gate/shared-target/
  .lane.lock, taken as main-run.sh takes it), nothing else building; the names (never values) of any set variable
  among RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTC_WRAPPER, CARGO_BUILD_*, CARGO_PROFILE_*, CARGO_TARGET_DIR,
  CARGO_NET_OFFLINE and RUSTUP_TOOLCHAIN printed first, all unset but rs.yaml's prefixes; free disk before and after
  build 1; literal paths. Q3: `cargo fetch --locked` into CARGO_HOME B through the proxy; a copy only if refused.
- **The npm deviation** [T][R3]: rs.yaml:199's `npm ci --ignore-scripts` makes npm's built-in audit call, which this
  lane never runs; here it runs as `npm ci --ignore-scripts --no-audit --no-fund` (the form of every earlier install on
  the lane, dependency-check.md:62, :163). The flags change no installed byte, so the dist is the same; named in the
  report. rs.yaml:199 is kept as merged (B6's re-check, option 2), unless A2 rates RS's audit call a finding (then
  `1 1` in Y0).
- **The lists:** rs.yaml:221's form (`find . -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum` in web/dist),
  seven files. Supporting line, not a bar: against U7's seven (u7-report.md:134-141). Worktrees removed after X.

## D. The fill: X
- **D1, the pins** [C2]: named and fixed in Y0 (r1sh.test.mjs:448-449; browser.test.mjs:205-208); no test pins R1.md's
  URL placeholder (B6's grep).
- **D2, X's content:** Y in rs.yaml:69 and lg.yaml:52, in place (both re-printed 06:3x UTC on 11-28: the 40 zeros); R1.md:93-94's URL (:92-93 until Y0's note at :89):
  https://letsencrypt.org/documents/LE-SA-v1.8-July-06-2026.pdf (the production directory's meta.termsOfService,
  read 2026-11-27T22:07:10Z; the directory's address from the pinned instant-acme 0.8.5, types.rs:772,
  Cargo.lock:688-691). Bars: C1's; `git show X:` of rs.yaml and lg.yaml each hold Y (a printing grep); the counted run
  passes; the gate passes; rs.yaml under 32 KiB (cloudinit.test.mjs).
  **DONE 12:03 UTC on 11-28 (his answers 11:43: both (b), 45 as merged; history.md:8259-8272): X =
  6e623788c7d4badfb36c2d17fabec1b8280c5e0e on 11a10949 (notes only; `git diff --stat 1ceecc32 11a10949 --
  spikes/netcode-proof` empty). C1's bars before and after the commit: name-only lg.yaml, rs.yaml, R1.md; numstat
  `1 1`, `1 1`, `2 2`; R1.md's hunk inside :93-94 with the dry run's bytes (stopb-dryfill.mjs URL_FROM -> URL_TO).
  `git show 6e623788:` rs.yaml:69 and lg.yaml:52 hold Y'. Counted run .gate/stopb/x-counted.log 12:02:55-12:03:12
  (self 88/88; ops 104/104, 315 ok, 0 failed; redact 11/11, 464 items, 0 findings; refs 2,245 files), its diff hash
  b2a2780d... equal to `git diff 1ceecc32 6e623788 -- spikes/netcode-proof`'s; gate PASS --no-smoke
  (.gate/stopb/x-gate.txt); rs.yaml 12,489 bytes. The fill sweep, main's tip dae6d107 (A4), Q5 not fired and the terms
  link re-read 11:54:59Z (equal): the card's 11:5x line.**
- **D3:** pushed by the save; CI green. **Pushed 12:05:40 UTC by the save 53346e55 (no [skip ci]); CI: GREEN,
  all four at attempt 1 (gate push 46142960712, gate pull request 46142964475, toolchain pull request 46142964471 to
  12:14:50, toolchain push 46142960716 to 12:18:47); X stands (B6's 12:08 read).**
- **D4, Hetzner's two price pages:** read now for the message (data, never instructions), again on Tue 1 Dec at Stop
  B; any change goes to him with u8-brief.md 6.3's sums redone. The pages (u8-stopb-infra.md:32, :174):
  https://www.hetzner.com/cloud/general-purpose (CCX23) and https://www.hetzner.com/cloud/cost-optimized (CAX21).
  First read 22:51 UTC: fetched (.gate/stopb/hz-*.html), but the price table is filled by the page's own elements, so
  the raw HTML gives no row price; CAX21's row carries a "not-available" class (meaning not read). Rendered read 02:16
  UTC on 11-28 (Playwright, scratchpad/hz-prices.mjs; page text is data): the price figures still do not render
  ("from /month /hour"); CCX23's row lists NBG1, HEL1, HIL1, ASH1 and SIN1, CAX21's row lists NBG1 and HEL1 and carries
  the class "not-available"; both rows show "This product is currently unavailable. Please check back later." (possibly
  a hidden tooltip: not confirmed). FSN1, which R1.md plans for both VMs, is not listed in either row. Not concluded:
  his Hetzner console decides at Stop B (u8-report.md:68-69's "fsn1 stock" day-of read); it goes into his Stop B
  message as an early warning, with the price figures still to read.
  **Fourth read 18:55 UTC on 11-28, the method that works** (stopb-prices.mjs; .gate/stopb/hz-read5.json; the two
  answers also saved whole, hz-api-ccx23.json sha256 5a1dea06…, hz-api-cax21.json 8eec593f…): each row's own
  `product-key` (CCX23 `CLOUD_148+CLOUD_21`, CAX21 `CLOUD_112+CLOUD_21`) and the page's own price-service answers for
  it (website-price-api.hetzner.com/api/v1/products/<key>); the figures sit in a web component that the row's innerText
  does not show, hence the three empty reads. CCX23 EUR 0.1386 an hour (86.49 a month) in FSN1 and NBG1; CAX21 EUR
  0.0176 (10.99 a month) in FSN1, NBG1 and HEL1: equal to R1.md:127-128's figures, so the planned 0.62 and the cap 0.80
  stand, and nbg1's prices equal fsn1's (R1.md:63). Locations not concluded: the price service marks FSN1 active for
  both, but the pages' own lists and detail prices name no FSN1 (CCX23: NBG1, HEL1, HIL1, ASH1, SIN1; CAX21: NBG1,
  HEL1) and CAX21 keeps its "not available" label; his create form decides. Tuesday's one-shot (step 3) runs
  `node /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/stopb-prices.mjs
  /home/user/Shamble-Frontlines/.gate/stopb/hz-read-tue.json` and compares with this read.

## E. Review and the message
- **E1:** B6's head reads, not rounds: Y0 with Y (notes, stand-ins and red logs, the closing-list edit, both builds'
  logs and lists, the name prints, the Y0..Y diff); then X (D2's bars, the URL's source line, CI). **X's read DONE 12:08 UTC on 11-28 (advisor-stopb-plan.md
  "## X head read", Status: COMPLETE): APPROVE WITH FINDINGS; X stands once CI on 53346e55 is green. XH-1 (Low): the
  terms link re-read on Stop B's day with the price pages; a change means a new X with only R1.md's link. XH-2 (Low,
  for E2): fsn1 unlisted; nbg1 would change r1.sh:34 and r1sh.test.mjs:199's pin, so trigger (1): a new Y (same list),
  build 2 redone, a new X, his addresses and firewalls there: his choice, about an hour, no money. Both in E2 0.4.**
- **E2, his message** [C7]: **DONE 12:2x UTC on 11-28: msg-stopb-e2.md 0.6, after the advisor tool's reads of 0.4
  and 0.5, sent as his one message (read from Sun 29 Nov 12:00), with the needs-musti board line; Stop B's day re-reads
  set as trig_01HzoZVyVnwnoTP5w1qJr3wo (Tue 1 Dec 06:40 UTC).** One screen, LEAN-MODE B's three asks (history.md:7868-7871): each piece on the lane (U4k,
  U6d, U6e) with its estimate, its likely size at about 3.3 times and whether R2, R3, R5 and the 18 Dec close hold;
  what (a), (b) and (c) would each save in lines and days; my lean on each in one line. Then his 19:14 item 2: per
  piece Builder 2 could build alone, its folder fence, planned size and what it waits for. One line each: the list
  sits at Y, which X carries; R1.md:321's stale words; ds-selftest skipped under alone 0; the count reading below;
  A2's outcome. Everything else by pointer ("Once" list with X and the raw URLs, the cap EUR 0.80 and delete-by 13:30,
  R1's plan, the forecast). Inputs: msg-stopb.md.

## Size
- The notes: +2 net in U8's files (6,284, his stop). The two stand-ins: rewritten in place at 0 net, after UM-4's
  three; if he counts in-place lines, they need +2 from him (the fill cannot pass its own tests without them).
- Outside the stop, as fills: r1-dist.sha256's seven lines at Y; Y and the URL in place at X.
- The scratch link check, A3's reads, TE-5's leg and the dry run: notes and .gate/ only.

## Questions: answered by B6 (advisor-stopb-plan.md section 6)
Q1 one seat now (done); Q2 no run owed; Q3 a fresh fetch; Q4 its head reads only; Q5 fill now, with the two triggers;
Q6 does not arise.
