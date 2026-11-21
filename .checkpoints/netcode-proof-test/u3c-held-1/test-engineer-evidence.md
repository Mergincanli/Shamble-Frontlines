# U3C-HELD-1: test-engineer's evidence and costs

Agent: test-engineer. READ ONLY (no builds, tests, cargo, main-run.sh). Writes only this file (Write tool).
Branch: claude/netcode-proof-test-plan-hcky5g (PR #40). Started 2026-11-17 about 07:58 UTC; done about 08:30 UTC.

## Plan
1. Read the card (netcode-proof-test.md:3876-3892), the kept evidence (merge.log:51-52,
   c62h-bound-build.md:426-455, advisor-c62h-bound-merge-stop-full.json C3-C4).
2. check.mjs: the test step, how the measured and bites builds are built and named (features, profile, target dir),
   the cargo test command line for each, FX-10 (2655-2657), any --no-fail-fast.
3. main-run.sh: usage, the lane lock, the 4 GB floor, the crates touch, log format (free lines).
4. Logs: time per run of the bites build's npt-net tests; free-disk lines before/after (merge.log, merge-2.log,
   other recent whole runs); number of whole runs so far that ran this test (basis for N).
5. delay.rs:528-543 and the test, to know what the unit test asserts and where the plant goes (only enough for the
   evidence part; the design is netcode-architect's).
6. Brief 8.1 (packing) and R4's list; du of the packable folders.
7. Write the answer blocks: diagnostic commands and N; confirm/stop criteria; fix evidence; CHECK-NFF-1 rule; disk;
   steps and reviewers.

## Findings (one line per finding, appended as found)
- F1 The bites test build is `cargo test --locked --workspace --all-features` from spikes/netcode-proof
  (check.mjs:2057); measured is `cargo test --locked --workspace` (:2053). Neither has --no-fail-fast. Target: main-run.sh
  exports CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target (main-run.sh:15,18); profile `test`
  (unoptimized + debuginfo, merge.log:26), so the binary is .gate/shared-target/debug/deps/npt_net-<hash>.
- F2 The failing binary was npt_net-ed8673eeeaf4db12 (merge.log:48). The same hash comes from `cargo test --locked
  --workspace --all-features --no-run` (u4r/c6-end3.log:65,105), te2-5/run1.log and u4e/b11-list.log: the hash follows
  features/profile, not source (9e0f103, 561bf42 alike). Measured build's npt-net: 088534cbcaf56b90 (c6-end3.log:3,43).
- F3 TRAP 1: `cargo test --locked -p npt-net --lib --all-features` (te_m3_loop.sh:25; u4d2/r1-te-te25-npt-net.log)
  builds a DIFFERENT binary, npt_net-301de27e05915ce7: -p turns on only npt-net's own features (wt-wtransport, wt-wtq,
  crates/net/Cargo.toml:12-15), while --workspace --all-features also turns on npt-l1's `bites`
  (crates/l1/Cargo.toml:12-15) and unifies the other members' dependency features. TE-M3's loop ran 301de27, not
  check.mjs's binary.
- F4 TRAP 2: in a --workspace --no-run build there are 30 "Executable unittests src/lib.rs" lines (c6-end3.log), the
  last one npt_tcpinfo's (:63). te_m3_loop.sh's `sed ... | tail -1` (:28) would pick npt_tcpinfo there. The new
  script must match `npt_net-` and then check the name equals npt_net-ed8673eeeaf4db12.
- F5 Time per run of the ed8673 binary, whole (169 passed, 12 ignored): 29.43-31.05 s, 14 kept runs (te2-5/run1-10.log,
  u4r/c2-rul-reg.log, c3-reg.log, c2-b10-reg.log, c2-b10a-2.log; their "finished in" lines). The 301de27 binary in 2
  parallel loops: 32.52-35.51 s each (te-m3/loop.log:4-18); 60 runs took 11:47-12:05, about 18 s a run overall
  (card :2973). Build from touched sources: 58.39 s for -p npt-net (te-m3/build.log); the bites workspace test build
  21.30 s after the measured one (merge.log:26).
- F6 Runs on record of this test in a bites build: 1 FAILED (merge.log:51); "... ok" in 14 kept ed8673 runs and 81
  kept 301de27 runs (te-m3 run-1/2: 60, p3: 20 with 4 busy loops; u4d2: 1), plus the bites half of about 54 check.mjs
  test steps kept only as summary lines (54 logs under .gate/ with a "PASS  test" or "FAIL  test" line), whose test
  output is not kept. So 1 in about 97 (the advisor's count: logs with the "... ok" line) to about 150 bites runs.
- F7 The test (delay_tests.rs:2266-2308) is multi_thread with 2 workers; its println (:2292-2299: largest excess,
  "samples above the bound [d, u]", first losses, stalls) shows only with --nocapture or on failure. The check
  (delay.rs:521-544) only counts; it keeps no state of the sample that counted. So a loop of the unchanged code can
  show the failure again with its counts, but cannot by itself show the release-pending state the cause names.
- F8 main-run.sh: lock .gate/shared-target/.lane.lock exclusive (:27-28), waits until no cargo/rustc runs (:30-37),
  header with free MiB, HEAD, diff and untracked hashes of spikes/netcode-proof (:39-43), STOP exit 3 under 4 GiB free,
  checked only at the start (:44-47), touches every file under crates/ first and every 2 s (:48-53), runs each command
  without a shell from spikes/netcode-proof (:22,57-61), stops at the first non-zero exit (:66-72), provenance at the
  end (:77-84).
- F9 Precedent TE-M3 (card :2964-3002; te_m3_loop.sh): mode all, 60 whole-binary runs caught nothing; mode one (the
  test alone, --nocapture, 2 loops x 300, 6 busy loops) caught it on the 9th run. Its fix's proof: P2 (the fixed test
  600 runs), P3 (mode all 10 x 2, 4 busy loops), P4 (two plants, each exit 101, restored by Edit, hashes), P5 (counted
  check.mjs), P6 (gate); the advisor's commit check ran 300 more.
- F10 Only ignored tests read env vars at run time (tcp_model/tests.rs:533,583,587,1393; socket_tests.rs:514,1353;
  delay_tests.rs:3094), so running the binary from crates/net, as cargo does, runs the same tests (te2-5's cargo run
  and te-m3's direct run both give 169 passed, 12 ignored).
- F11 PLANT REACH: in the failing test the client drains a 1 MB duplex (delay_tests.rs:2268-2274), so released
  records are taken at once and released_model is about 0 at most writes. The card's plant (drop released_model from
  down_free, delay.rs:473-477) then admits extra bytes only inside the same rare window: it may not bite in this test,
  and a plant that bites only sometimes proves nothing. It bites every run only where S does not read (released bytes
  stay untaken, as in a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded, duplex(1_024),
  delay_tests.rs:745-747). In that test today the plant would fail the Pending assertion (:779-782) first, and a 100 B
  write after the hold stays under 64 KB, so the bound check would not be the failing line there either.
- F12 DESIGN DISCRIMINATOR: a fix that checks at every sample "held less the released-untaken bytes <= 64 KB +
  prefix" cannot catch the card's plant: with released_model dropped, admission still keeps outstanding <= 64 KB, so
  the unreleased part stays within the bound and all the extra bytes sit in released-untaken records. Only a check of
  the whole held (released-untaken included) at each write (netcode-architect F7, F10) catches it. The plant therefore
  also tests the fix's design, not only its wiring.
- F13 DETERMINISTIC CAUSE: with S never reading and P2's loss, once the writer back-fills a front record's
  acknowledged prefix (held above 64 KB at a write) and that record is released, held stays above 64 KB + the new
  front's prefix until S reads, which it never does. The old every-sample check then counts at the next sample on
  every run: a run that shows the cause by construction, with no race. A test built that way, run against the old
  check (a plant), is a stronger confirmation than a 1-in-100 loop.
- F14 The advisor's ruling says "npt-lab does not depend on npt-net at all (crates/lab/Cargo.toml)"
  (advisor-c62h-bound-merge-stop-full.json:3). Directly true, transitively false: crates/lab/Cargo.toml:30,32 (npt-server,
  npt-bots) and crates/server/Cargo.toml:29, crates/bots/Cargo.toml:25 (npt-net); merge-2.log:43 compiles npt-net for
  `cargo build --release -p npt-lab`. C62H-BOUND's conclusion still holds (crates/net unchanged in that lane). For
  U3C-HELD-1: the npt-lab release binary does contain npt-net's non-test code, so "test-only" must be shown.
- F15 Test-only proof available cheaply: the npt-lab release sha256 98c3967e04e4260985d24173b1fb2126ea1444636f4725e12cf76333d90b5113
  (preflight.log; c62h-bound-build.md:478) was equal across both build caches (u4r-report.md:1589), and the spike tree
  outside results/ is unchanged from ea08f89 to HEAD (`git diff --quiet ea08f89 HEAD -- spikes/netcode-proof
  ':(exclude)spikes/netcode-proof/results'` exit 0). The same hash after the fix shows no lab/server/bots code changed.
- F16 Counts at HEAD 43b02e8: `git diff --stat 561bf42 HEAD -- spikes/netcode-proof ':(exclude)…/results'` prints
  nothing, so the test totals stay 552 / 632 (merge-2.log:12) until the fix. refs read 1350 tracked files and redact
  305 items at 3f2fa21 (main-54/light.log:5,8); marks 81, self 78 of 78.
- F17 Disk lines: today 4,951 MiB at 07:29 (main-54/light.log:2); df now 4,993 MiB (read at about 08:10). Whole
  check.mjs net use: -1,022 MiB worst (refs-kit/counted-merge-2.log:2 6,913, card :373 "free after 5,891"), +804
  best (merge-2.log:2 5,278; :23 6,377,758,720 B = 6,082 MiB after check.mjs). The 4.76 GB of 2026-11-15 was a near-cold
  fill (advisor-disk-deadlock.md:18); the shared target is warm now (du 6,308 MiB; debug 5,847; incremental 2,979, of which
  npt_net sessions 273 and 221 MiB). A bites test build nets -16 to -33 MiB (c62h-bound step2 headers: 6,096 -> 6,063,
  6,107 -> 6,091). Whole-binary run log about 14 KB (te-m3/run-1-1.log 14,281 B).
- F18 Brief 8.1 (c62h-bound-brief.md:485-501): pack one at a time, in order u4r-P2-ab10f058a828, u4r-B1-64f3307d5c7f,
  u4r-P3-0f066338dd8d. The first is already packed (kept-logs/u4r-P2-ab10f058a828.tar.gz, 217,268,267 B; card :4009-4017,
  572 MiB back from 779). du now: u4r-B1-64f3307d5c7f 821 MiB (results/u4r/64f3307d5c7f/B1), u4r-P3-0f066338dd8d
  519 MiB (results/u4r/0f066338dd8d/P3). Not packable: both P0 (3,249 and 3,393 MiB), fc411116c1f1 (623),
  fd4612050cd6 (291), the two c-u2 (998 each) by R4; u4r-P2-8ee453c22882 (741) and u4r-P3-d66e23cd78f2 (298) are
  not on 8.1's list.
- F19 R4's precondition: card :197 "R4 P5 holds until then; after that, pack only by brief 8.1" and :202-203
  "Nothing is packed before he has read it (R4; the chat's For Mustafa item 2)"; card :10 (07:38 UTC) still "waiting
  for his answers". 8.1 is also written for C62H-BOUND's sets ("Before each set", :489). Whether either lets
  U3C-HELD-1 pack now is for the advisor's plan review to say.
- F20 plan.md §12.4 (:1349-1351): "node tools/gate.mjs must PASS at every step. The spike's own test command ... runs at
  every unit". CLAUDE.md step 5 names rust-reviewer for Rust code; the card names netcode-architect and test-engineer.

## Answer blocks (for the drafter)

### 1. The diagnostic step (the unchanged code)
Script (the main session writes it, text only, in this folder): u3c_held_loop.sh, te_m3_loop.sh with these changes:
TEST=l2::delay::tests::under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix;
OUT=/home/user/Shamble-Frontlines/.gate/u3c-held-1; build `cargo test --locked --workspace --all-features --no-run`
(check.mjs:2057 plus --no-run); BIN from the line matching `Executable unittests src/lib.rs (.*/npt_net-[0-9a-f]{16})`,
STOP exit 2 unless its name is npt_net-ed8673eeeaf4db12; sha256sum of delay.rs, delay_tests.rs and BIN; each run's
own line adds the watched test's by-name line (`test $TEST ... ok|FAILED`); on a failure print libtest's failures list
and the test's `---- $TEST stdout ----` block (the println and the panic with left and right).
Call L1 (mode all, the faithful one):
bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/u3c-held-1/l1.log df -B1 /home/user :: bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-held-1/u3c_held_loop.sh l1 all 150 2 0 :: df -B1 /home/user
N = 300 (150 in each of 2 loops), stop at the first failure. Basis F6: 1 in 97 to 1 in 150. Chance to see it at least
once: N 100: 64 % / 49 %; 200: 87 % / 74 %; 300: 95.5 % / 86.6 %; 450: 99 % / 95 %. None in 300 bounds the rate under
1 % (95 %, rule of three). Time: about 1 min build + about 90 min (F5: 2 loops about 18 s a run overall).
Call L2, only if L1 catches nothing: `... u3c_held_loop.sh l2 one 300 2 6` (the test alone, --nocapture
--test-threads=1, 6 busy loops: TE-M3's catching set-up); per run a few seconds (doc :2265; not measured yet).
Confirms the cause only with state: either the probe (an eprintln in check_held_bound when it counts: held, budget,
prefix, released[d], released_model[d], records released and untaken; Edit in, Edit out, hashes before and after, never
committed) or, better, the deterministic test T-NEW with plant P-a (block 2). Confirmed when every counted sample has
released_model[DOWN] > 0 and held <= 64 KB + the prefix the released record had at the last write. Reproduced only
(not yet confirmed): the assert at delay_tests.rs:2300-2304 fails with [k, 0], k >= 1, the println's excess < 16,406 B
and first losses > 0, nothing else fails.
Stops for the advisor: BIN not ed8673; any other test fails, or this test fails by another line (hang guard :2283-2286,
excess :2305); UP count > 0; a counted sample with released_model[DOWN] = 0 or held above the last write's bound (a
real DelayStream defect: severity up); none in 300 (+ L2) unless the plan review already ruled that T-NEW + P-a carry
the confirmation; main-run.sh exit 3 (floor).

### 2. The fix's evidence
E1 the old test kept meaningful: its exit conditions stay (records >= 100, beyond > 0, lost > 0); held_over_bound [0, 0]
now from the write-time check; new: the down write checks ran at least `records` times (a test-only count); :2305 kept.
E2 T-NEW (S never reads, duplex(1_024), P2 loss): writes until held > 64 KB at a write and a front record is released
untaken; waits by until(HANG, ...), never sleep; asserts the bound (held_over_bound [0, 0]) BEFORE any Pending assert;
then writes again until Pending. UP: a test with up traffic gets an UP count > 0 assert (or the design says why not).
E3 loops on the fixed code: mode one, T-NEW and the old test, 2 x 300, 6 busy loops, every run exit 0 (TE-M3 P2);
mode all 10 x 2 with 4 busy loops, 20 of 20 (TE-M3 P3).
E4 plants, each: sha256 before; Edit; main-run.sh call building with --workspace --all-features and running the whole
ed8673 binary (its header's diff hash shows the plant in place); expected exit 101 and the named line; Edit back;
sha256 equal to the fix's bytes; `git diff --quiet HEAD -- spikes/netcode-proof/crates/net` exit 0.
P-a the old every-sample check restored: T-NEW must fail (the cause, by construction).
P-b drop released_model from down_free (delay.rs:473-477): the bound assert in T-NEW must fail ([k >= 1, 0]).
P-c remove the down write site's check call: the write-check count assert must fail (every write is covered).
(P-d the same for UP if E2 has an UP count.)
E5 `cargo build --locked --release -p npt-lab` and sha256 = 98c3967e…5113 (F15): the fix changes no lab/server code.
E6 the counted run: one main-run.sh call `df -B1 /home/user :: node check.mjs :: df -B1 /home/user` on the fixed head.
Pass bar: header HEAD = the fixed head, diff and untracked e3b0c442…; exit 0, "check: all 11 steps pass"; test
"measured build: 552+a tests passed, 0 failed, 0 reported a skip; bites build: 632+a ..." (a = new npt-net tests not
under a feature; T-NEW alone gives 553 / 633); refs PASS on `git ls-files | wc -l` at the head (1350 at 3f2fa21 plus
the change-set's tracked files); marks 81; self 78 of 78; redact 10 of 10 and 305 items (+1 per pack list); deps PASS
(137 crate files unless one is added); wasm PASS with 27000 ticks and 0 mismatching both ways (hash-map digests vary by
design: never pinned). det-u4 and the hash are not in it: E5 shows the lab binary unchanged. A failure: stop, the advisor,
whole; never a second counted call without it.
E7 `node tools/gate.mjs --no-smoke` GATE PASS (lane precedent gate-merge.log); CI green on the pushed head.
CHECK-NFF-1 stays out. Rule: one change-set at a time, and a change carries only what its own claim needs to be
proven (CLAUDE.md protocol 1-2); a Low never triggers a push on its own (review budget); its taker is check.mjs's next
owner (card :3891-3892, advisor C4). Nothing here needs it (the loops run the binary directly; the counted bar is
all-pass, where fail-fast hides nothing), and putting it in would change the counted runner inside the change it is
counting (self 78 changes; it needs its own planted failing bites test).

### 3. Disk
Start 4,951 MiB (floor 4,096; 855 above). L1 + L2 under 60 MiB net (build -16 to -33, logs about 4 MB); peak during a
build some hundreds of MiB (npt_net's incremental session 221-273 MiB is rewritten), back after. E3-E5: about 5
rebuilds, about 150 MiB. E6: -1,022 MiB worst, +804 best. Worst sum about 1,230 MiB: about 3,720 MiB at the end, under
the floor, so U4f's counted run would stop at main-run.sh:44-47. Every call of this change-set still starts above
4,096 in this order (the counted run at about 4,750). Bar: free >= 5,220 MiB (4,096 + 1,022 + 100) at the counted
run's start; below it, pack first (after E5, before E6; never during a build or a run). 8.1 allows next
u4r-B1-64f3307d5c7f (821 MiB; about 600 back by ab10f's 73 %: to about 5,500), then only if still short
u4r-P3-0f066338dd8d (519 MiB; about 380 back). Each by 8.1's procedure (ruling advisor-u4r-sets-r2-full.md:45 cited;
pgrep empty; free >= size + 1,024: 1,845 and 1,543 both hold; list, commit raw-logs.sha256 alone, push, pack; free lines
on the card; exit 1 or a refusal: stop, tell Mustafa, pack nothing more). R4's precondition (F19) first.

### 4. Steps and reviewers
0 the plan (this draft) -> the advisor's plan review (pre-rules the not-reproduced branch and the packing question).
1 L1 (and L2 or the probe as ruled) -> results to the card; a stop -> the advisor.
2 the fix by Edit in the main folder (delay.rs test-only, delay_tests.rs, T-NEW; NC-U3c-2's sentences).
3 E3, E4, E5, each its own main-run.sh log under .gate/u3c-held-1/; the gate.
4 commit on the spike branch; one review round on the final diff: netcode-architect + test-engineer (rust-reviewer
  too if the advisor reads CLAUDE.md step 5 that way); one fix pass, then E3/E4 again on what changed.
5 pack if the bar says so -> E6 the counted run -> E7 the gate -> save the card (push) -> CI green.
6 the advisor's check of the pushed head, naming the full SHA -> U3C-HELD-1 closed; all before U4f's merge (C3).
Time: about 4 h (L1 1.5 h; fix and E2-E5 about 1 h; review about 45 min; E6 12 min; the rest about 30 min).
