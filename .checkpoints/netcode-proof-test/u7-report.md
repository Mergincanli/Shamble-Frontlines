# U7, the plain browser client for R1 (the first real run on rented servers): the merge report
DRAFT, written 2026-11-26 22:1x UTC as M0's prep (U7-M-7's items); the lines marked (fill) are filled at the merge.
Merged into the lane by the main session, plan u7-merge-plan.md (U8's M0-M9 form with the snapshot read's U7-M-1 to
-8; its check advisor-u7-merge.md, UM-1 to UM-5 applied). Brief u7-brief.md rev 1.1 (the advisor's C1-C14 in
advisor-u7-brief.md); brief 3 (launch-0850.md section 3) and the fix-pass brief (advisor-u7-round.md section 2); the
builder's notes wip/u7-build-rs-notes-1055.md (a65282fa…) and wip/u7-t-build-notes-2050.md; the round
u7-review-netcode.md and u7-review-test.md, ruled in advisor-u7-round.md; the snapshot read advisor-u7-snapshot.md
(MERGE, no fix item). The advisor reads the pushed head in a window (U7-M-8; not a round).

## What it is
43 files, all under spikes/netcode-proof/ (wip/u7-build-rs.patch, sha256 6535c181…2728, 267,380 B, base 80b6748):
- 10 changed: check.mjs and check.test.mjs (the `web` step; npt-client-wasm a leaf crate with its deps rule);
  crates/client/src/clock.rs and lib.rs (R1: the offset clamp, the closed-form next ping, the trimmed surface; R2:
  `sent` and HONEST_MAX_LEAD; the `wire` re-exports); crates/client/tests/trace.rs, crates/lab/src/scenario.rs,
  crates/bots/src/lib.rs and load.rs, crates/server/tests/carry.rs (the cap's readers); crates/client-wasm/src/lib.rs
  (the wasm-bindgen wrappers).
- 33 new: crates/client-wasm/clippy.toml, src/engine.rs (the plain-Rust core, its tests and the in-process room host)
  and src/script.rs (the press script); crates/client/tests/sent.rs; web/ (the page: index.html, four src/ files,
  bites/hooks.js, build.mjs, tsconfig.json; nine e2e/ files, the T5 drivers; thirteen test/ files with
  fixtures/boundary.json).
- No manifest or lockfile (U7's dependencies are on the lane since Block 0b, 2701220, and its amendment, 56b3e1b:
  dependency-check.md); no marks.json; nothing under crates/judge or marks/.

## Size
5,193 added and 47 removed (`git apply --numstat`), under SIZE-CHECK's 6,000. Planned: about 2,200 plus C2-C9's
150-250 (the brief, section 8); the fix pass added about 462 against the advisor's estimate of about 300.

## Evidence on the merged tree (filled at the merge)
- M1: the patch's sha256 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728 (the patch is removed by
  this merge, WIP-CLEAN). M2: (fill). M2b, the tie to run 4: (fill: u7-run.sh's header in the builder's worktree,
  "diff 551e9033…; untracked b4811e75…"). M3 and M4: (fill: `git apply --check --index`, then `git apply --index`, on
  H0; the 43 paths cmp-equal to the worktree's, mode 100644).
- C1 (U7-M-5, a form test): (fill: run 4's 6-mds `pages[0]` through U8's save() as R1, eu-path and ML*; npt-judge
  with PM-0001-36: a verdict and no read error; the printed wasm sha256 = run 4's 901a6492…0e77).
- M5 (one counted run through main-run.sh, the lane lock, incremental off): (fill: U8's ops tests N of N; wrappers
  "315 ok, 0 failed"; check.mjs all 12 steps: test measured and bites against UM-2's 931 and 1,028; web 70; self 80;
  marks 81, rows unchanged; redact 0 findings, items as counted; refs N0 + 33; wasm 27,000 ticks, 0 mismatching).
- M7: (fill: the link removed; gate --no-smoke; the repo map check; `git status --short`).
- D10: no marks.json or crates/judge change, so nothing for his read of judge changes.
- DEP-HOLD: no manifest or lockfile in the merge.

## The browser runs (T5 (ii)): the marks against the brief's words (u7-brief.md:192-209)
Run 4 (2026-11-26T10-48-42-618Z), on the fixed tree (80b6748 plus the snapshot): its three runner headers (end-2,
final-dist and t5ii-run-4) each read "diff 551e9033…; untracked b4811e75…"; summary.json sits in the builder's
worktree under .gate/u7/t5ii/. Chrome for Testing 153.0.8010.12 headless, playwright-core 1.56.1, both browsers with
`--no-proxy-server`; npt-server 0db5ca7c… and npt-host 5c7c7201… (7e2d6f3's code: no file of their trees changed
since). t5ii.mjs records values and judges no mark, so the readings below are mine against the brief's words (the
snapshot read: a hand read is enough).

| Mark | The brief's words (short) | Run 4 | Read |
|---|---|---|---|
| 1 | `smoke` on wt: "WebTransport · Playing" within 3 s | Playing at 264 ms | met |
| 2 | `smoke` on ws: "WebSocket · Playing" | Playing at 254 ms | met |
| 3 | three `cert-14d` attempts, `ok` 1 each | [1], [1], [1] | met |
| 4 | `wt-attempts` with `hash=wrong`, `ok` 0 | [0], "Ended: the WebTransport session did not open" | met |
| 5 | `fallback` (silent UDP): Playing over WebSocket, `playing_ns` 1.5-3 s | 1,564,100,000 ns | met |
| 6 | `mds`: at least 2 reads in 12 s, each at least 1,000 B | 1,024 B and 1,024 B | met |
| 7 | every record valid, no token; no CSP violation in the console | no record problem; no CSP line; `page_csp_violations` 0 on every page of every load; no `nptj_` in the run's folder or log | met (below) |
| 8 | the bites dist: read-once 1 read in 12 s; no-wt-timeout, restated: "a fallback later than 3 s, or none within 10 s" | read-once 1; no-wt-timeout 1 fallback at 4,067,000,000 ns | met (8b under his restated words) |
| 9 | the room's log: the page's slot has no kind-10 refusal and nothing counted as forged | not readable from the binary's log | NOT-RUN |

- **Mark 7:** both halves, the page's and the Worker's, are read through `page_csp_violations` (F5); the probe's
  control shows the CSP refusing in each ("page csp 1; worker csp 1"). The three shipped calls that feed the row are
  in the built dist, but no test or browser enters them, so the zeros are read with the wiring inferred (ADV-U7S-2).
- **Mark 8b**, under his U7-MARK8B (a) (history.md:8068-8088; u7-brief.md:200-209): run 3 (25 Nov, before the fix
  pass) fell back at 1.60 s normal and 4.06 s planted; run 4 at 1.56 s and 4.07 s. The planted bug (the page's 1.5 s
  timer removed) moves the fallback past 3 s, to Chromium's own QUIC idle timeout (4 s), so it is caught. As first
  written ("no fallback event in 10 s") the mark is not met (NC-3, below).
- **Mark 9**, under his U7-MARK9 (a): NOT-RUN, never PASS. The room's results log holds no per-slot refusal or forged
  count (tick_loop.rs:1159-1185: T5ii-A). The in-process stand-ins (advisor-u7-snapshot.md section 1):
  - F3's third leg (100 ms each way, no clamp, a 2.1 s stall at step 659) is the one that discriminates: the old step
    counter fails it with a PressEdge refusal (seq 649), and the fix passes it.
  - F3's first two legs (the brief's schedule, with and without the 4 ms clamp) and R3's press-script test (3,000
    steps, every slot counter 0, plants room-a and room-b caught) are coverage, not discrimination: after a gap of 2 s
    or more the clock's samples age out (clock.rs:20, WINDOW_NS 2 s), so no command is stamped until a new sample.
  - Their limits: in process, behind npt-server's real Decoder and npt-room; no jitter or loss; the brief's 3 s-gap
    leg cannot catch the old counter; the fix is shown by the narrow leg only; in Chromium the steady wake chain is
    clamped, so NC-2's case is narrow there.
  - The real mark is read in U6d, which adds the room's per-slot refusal list; U6d comes after R1 (replan.md:22, :92),
    so R1 runs with mark 9 not run.
- **The live counters per load** (commands sent / last command the room ran / unencodable / client decode refusals):
  smoke-wt 149/146/0/0, smoke-ws 89/86/0/0, mds 327/325/0/0, read-once 327/325/0/0, no-wt-timeout 148/145/0/0, C8
  57/55/0/0 and 58/56/0/0. The last column is the client endpoint's own decode refusals (client/src/lib.rs:480-482),
  not the room's: it is not mark 9 (NC-8 (a)).
- **C8, the real host's leg** (through U6c's npt-host): WebTransport Playing at 278 ms, WebSocket at 265 ms; all seven
  dist names 200 with their listed types; every response from 127.0.0.1:31000 only; no failed request; no CSP line.
  It ran under ignoreHTTPSErrors (the page origin uses a test certificate), so it shows the real host's setup, its CSP
  and the room accepting, not the pin admitting the certificate: the pin's evidence is marks 1 and 4 (no ignore; the
  wrong hash refused) (NC-8 (b), TE-10).
- **The seven dist SHA-256s** (measured): index.html 46febaac…5f95, main.js c9be4a16…ccec, worker.js 4f060d18…1857,
  lib.js 4f374eaf…4bbe, hooks.js be1c4ce7…f814, npt_client_wasm.js b355d85c…3965, npt_client_wasm_bg.wasm
  901a6492…0e77; the bites dist the same six and hooks.js cd7be6c0…2972. They are the snapshot tree's; R1.md's
  expected values come from the merged commit at Stop B (C13).

## Tests (the builder's end-2 on the fixed tree; this merge's own counts are M5's, above)
- end-2.log (10:26-10:48 UTC, HEAD 80b6748, the same diff and untracked hashes): all 12 steps PASS; test measured
  884 and bites 981, 0 failed, 0 skipped (861 and 958 at the R-half's base: +23 each, R1 5, R2 4, R3 11 and the fix
  pass's F1, F3 and TE-7's Core test); web 70 of 70; self 80 of 80; wasm 27,000 ticks, 0 mismatching; refs 1,988;
  marks 81; redact 0 findings in 454 items.
- Plants (each failed its named test, then restored and sha256-checked): R1-R4's P1-P7, P17, P18 and the two room
  plants (notes 1055 :31, :38, :48, :97); the fix pass's F1 (i) and (ii), F3's narrow leg, TE-7 and one batch of 13
  web plants (web 57 of 70, exactly the 13 new tests failing; self 79 of 80) (notes 1055 :137-155).

## The wording items (the round's ruling, advisor-u7-round.md section 1)
- **NC-3 (T5ii-B):** mark 8b "not met as written", with run 3's facts: 1.60 s for mark 5, and 4.06 s under the bite at
  Chromium's QUIC idle timeout. He restated the bar (above). Carried to R8's bite plan and to Stop B's read of
  `npt-ctl quic off`: the block must DROP, never REJECT (with a REJECT, Chrome fails at once and the bite cannot bite);
  each browser's own timeout is recorded at R8.
- **NC-8:** (a) the counters' last column is the client's decode refusals, not the room's (labelled above); (b) C8
  does not show the pin; marks 1 and 4 do.
- **TE-6:** C2's JoinAccepted check cannot fail on a shipped change: both hashes echo RoomHost's own
  `with_hashes(CONTENT_HASH, MODE_HASH)`, and MODE_HASH is all zero, as are the room's defaults (room.rs:581-582). The
  evidence is the Decoder accepting the Core's Join (plant room-b bites) and T5 (ii)'s joins accepted by the real
  binary.
- **TE-11's records:** the plants P8-P16 and P19 kept no log (pass counts in the notes only; the web sources' sha256s
  equal the plant-time "before" hashes); the notes' "13 steps" is 12 (end-1); the round packet's 243,353 B was
  243,393 B; webStep's `npm ci` branch is tested only through the recorder (no real install in a test); the fresh-tree
  run of the web step is this merge's M5, in the main folder.
- **T5ii-C:** closed (the flag is in place). On 25 Nov at 19:3x UTC, run 1's C8 leg sent one CONNECT naming the real
  host and port to the session's agent proxy, which reset it, and one diagnostic curl did the same. No join token left
  the browser (a URL's fragment is never sent); whether the proxy reached the real host is not known. Since run 2 both
  browsers launch with `--no-proxy-server`, and run 4's C8 responses all came from 127.0.0.1:31000.

## The departures (each recorded; none is a defect)
- **F3's plant:** the brief's plant premise was wrong (the advisor's own): its plant after the 3 s gap could not fail
  (the clock's 2 s window). The builder found why and added the leg where it does fail (ADV-U7S-1).
- **HONEST_MAX_LEAD outside the file list:** bots/src/load.rs:45 and server/tests/carry.rs:129 point at the constant
  too (C3's rule: no literal 9 is left as a cap).
- **`npt_client::wire`'s re-exports** (new pub items: Channel, CONTENT_HASH, Direction, Join, Limits,
  PROTOCOL_VERSION, Command, LOOP_CATCHUP, TICK_HZ): exactly what npt-client-wasm names; MODE_HASH is not re-exported.
- **C6 reworded:** the 0b amendment (npt-link and npt-proto as dev-dependencies of npt-client-wasm, for the in-process
  room tests) departs from C6's first wording; the advisor re-ruled it (advisor-main-66-head.md section 1, its exact
  rewording there).
- **TE-11's limits literal kept:** the limits test pins 65,541 as FULL_STATE + 5, because U6b's FULL_STATE_MAX is
  `pub(crate)` in npt-net (wt_server.rs:49) and not reachable through client-wasm's dev edges.

## Carries (each with its taker)
- TE-5 (T5i-1): one WebSocket leg without ignoreHTTPSErrors before R1, to record why every WebSocket load shows
  handshake_failed 1 beside accepted 1: Stop B's fill, by the main session. No mark reads that series.
- U6d: the room's per-slot refusal list (mark 9's real read) and T5i-2 (Join-deadline closes in
  `deadline_closed_count`).
- CLIPPY_TWINS: client-wasm's clippy.toml joins the list, for check.mjs's next owner (beside CHECK-NFF-1).
- DROP-not-REJECT: R8's bite plan and Stop B (NC-3, above).
- ADV-U7S-5 (U8's ops/browser/config.mjs:60 comment, stale after NC-5) and ADV-U7S-6 (no test pins r1.sh's
  load_settings to LoadSettings; the advisor read them equal): U8's next change.
- The `ops` step: the first change after this merge (UM-4: test first, U8's fix 2 install, the dependency record),
  before Stop B's fill.
- ADV-U7S-2: a real-page control for the three shipped calls, at the next T5 (ii) run if one comes. ADV-U7S-7: `fail`'s
  call to `reap` is not shown, and TE-7's refused-by-name test has no plant (recorded here).

## The builder's slips (none touched evidence)
- ADV-U7S-3 (Medium, process): his interim rule broken once: the last `git status` ran after `cd`, `sha256sum` and
  `ls` in one command (read only). The next brief to this builder repeats the rule and asks for one command per call.
- ADV-U7S-4 (Low): one `tee` write of a hash list into the scratchpad (outside the repo); one no-op shell loop; waits by
  a foreground `tail -f --pid` (bounded), not the background form. Earlier, in brief 3: shell loops to wait for logs
  (read only; stopped at 00:16 UTC on 26 Nov).

## Not verified
- Firefox, Edge, Brave and Safari; the CSP outside Chromium; RS's network; the build's byte equality on RS (Stop B
  compares). The three shipped calls in the dist (ADV-U7S-2). Mark 9 (U6d, after R1).
