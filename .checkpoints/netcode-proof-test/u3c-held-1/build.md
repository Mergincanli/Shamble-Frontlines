# U3C-HELD-1's build notes (the main session; one line per step, as it goes)
Plan: u3c-held-1-plan-draft.md (sha256 a53b2803…), approved with conditions C1-C10 and rulings HELD-Q1-Q12 by the
advisor's plan review (advisor-plan-review-full.json, 627bfc6c…, 08:36-08:51 UTC). Branch HEAD at the start: the card
saves after 18b139c; the spike unchanged since ddd38ba.

## Restore facts (read first after a cut)
- Before D-1 (about 08:52 UTC): delay.rs aaa6a5887085c6399fb8b37b7d4d31ffbdb281cdd23b9d41d3c8ce2f044964c6,
  delay_tests.rs 74d49de77004dc2f33ed243cac22c0295c93475c00a066ef06cf89f89e640b64 (both as HEAD).
- D-1 is a diagnostic patch in the main folder, NEVER committed: Life's two fields over_waiting and over_at_write,
  the over_waiting count in check_held_bound, the over_at_write block at model_write's end, G-NC2's println. If a cut
  finds it in place and no loop running: take it out by Edit, then the two hashes above must hold and
  `git diff --quiet HEAD -- spikes/netcode-proof` must exit 0.

## Steps
- 08:51 The plan review saved and read in full (APPROVE WITH CONDITIONS C1-C10). The loop script written:
  u3c-held-1/u3c_held_loop.sh (te_m3_loop.sh's form; the workspace bites build; BIN by its one npt_net line, exit 2
  unless npt_net-ed8673eeeaf4db12; every failing test's stdout block, C3).
- 08:52-08:53 D-1 in by Edit (four edits, the plan's text): `git diff --stat -- spikes/netcode-proof`: delay.rs +17,
  delay_tests.rs +4 -2, nothing else. With D-1: delay.rs b70ab4e5934f5b59dac80dbcaf09b5a1507a80048d8cae20fdcc68b1d27db911,
  delay_tests.rs 2ccaccc39097c2687bc015928221d4ad688576009ec54eef689721dd06ea651b.
- 08:53 L1 started in the background (task b7jyrgn37; about 90 min): `bash …/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u3c-held-1/l1.log df -B1 /home/user :: bash …/u3c-held-1/u3c_held_loop.sh l1 all
  150 2 0 :: df -B1 /home/user`. Read it against the plan's Confirmed and Stop lines (step 1; C2); if nothing fails,
  L2 (`l2 one 300 2 6`).
- 09:46 L1's header: free 4,983 MiB, HEAD e85fdba, diff bdc5bba4… (D-1), untracked e3b0…; the build exit 0; the
  binary npt_net-ed8673eeeaf4db12 as required (sha256 04c76dc44aea9dbc9f11a67272df98fbd532274ec490554e234f88662d4541c0);
  the two files' hashes as D-1's above. 182 of 300 runs done by 09:46:04 UTC, every one exit 0, 169 passed, G-NC2 ok
  (about 33 s a run; the end about 10:19).
- **09:47:43 L1 CAUGHT IT: loop 1 run 94, exit 101** (l1.log:202-232; the run's log .gate/u3c-held-1/l1-1-94.log):
  "test result: FAILED. 168 passed; 1 failed; 12 ignored", the one failing test G-NC2
  (l2::delay::tests::under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix); its stdout:
  "100 writes of 16 KB at P2: largest down excess over 64 KB 14326 B; samples above the bound [1, 0]; first losses
  28, stalls 28; D-1: over-samples with a released record waiting [1, 0], at writes [0, 0]"; "panicked at
  crates/net/src/l2/delay_tests.rs:2302:5" (HEAD's :2300, two lines down by D-1's println), "left: [1, 0]", "right:
  [0, 0]". Loop 2 stopped at its next check; "end 09:48:12 UTC; ... first failure: 1-94"; main-run.sh exit 101 (the
  last df did not run). 189 runs in all: 188 exit 0, 1 exit 101.
- **The plan's Confirmed line holds, every item:** the only failing test is G-NC2, at its held_over_bound assert;
  left [k, 0] with k = 1 (UP 0); over_waiting [1, 0] = held_over_bound [1, 0]; over_at_write [0, 0]; excess 14,326 B
  under 16,406; first losses 28 above 0. No stop line met (BIN ed8673; no other test; not :2283 or :2305; UP 0; no
  over-sample at a write; main-run.sh not exit 3). **The cause is confirmed by a run:** the one sample over the bound
  had a released record waiting and none was at a write, the release transient. L2 is not needed (the plan: "If none:
  L2").
- 09:48-09:49 D-1 out by Edit (its four edits reversed): delay.rs aaa6a5887085…, delay_tests.rs 74d49de77004… (as
  before D-1, as HEAD); `git diff --quiet HEAD -- spikes/netcode-proof` exit 0. Next: step 2, the fix.
- 09:50-09:52 **Step 2, the fix, by Edit** (the plan's diff and the design notes' doc texts, C1): delay.rs (module doc
  :46-53; Life's doc and held_at_write, held_grew; held_now calls check_held_falls; check_held_bound(d) with its new
  doc and `held_at_write[d] = held`; the new check_held_falls; model_write's doc and its `#[cfg(test)]
  self.check_held_bound(dir);`), delay_tests.rs (the stalled-client test's doc sentence, its whole-record post-hold
  write and three asserts; G-NC2's doc, println, message and held_grew assert), marks/SERIES.md (each old phrase
  counted 2 before, replaced in both rows), u3c-plan.md :122-128 in place with dated notes, :125's up prefix too
  (HELD-Q6). `git diff --stat`: delay.rs 73, delay_tests.rs 39, SERIES.md 4: **116 changed spike lines** (the plan:
  about 130; SIZE-CHECK holds), u3c-plan.md 9. Fixed: delay.rs 3dbbccf73d9c5735d61c107b6d2ab7bf67b3bd620fba0db4c84465b78231e3d7,
  delay_tests.rs 8f8c0f9ab42c17ed7f0c7df47505f5f50b8d876e32092925505dac03799685d2, SERIES.md
  1dd67e2f2db8e3c569a23fa0580dd2fc47c240d94526d6440348df7dcc41725c (each plant's restore must give these).
- Next: C4's light pre-check first (fmt, clippy, doc: a lint or rustfmt diff now costs no loop), then E3, E4, E5,
  the gate.
- 09:52-09:53 **C4's light pre-check PASS** (.gate/u3c-held-1/light.log: header free 4,919 MiB, HEAD 801d284; fmt
  PASS (no diff), clippy PASS (both builds), doc PASS (host and wasm32, warnings as errors); "check: all 3 steps pass";
  exit 0, 09:53:06 UTC). **The gate --no-smoke: GATE PASS** (09:53 UTC; legacy/ unchanged). The stalled-client test's
  name checked: delay_tests.rs:747 `a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded`.
- 09:54 E3a started in the background (G-NC2 alone, `one 300 2 6`; log .gate/u3c-held-1/e3a.log).
- **09:53:52-11:12:33 E3a PASS: 600 of 600 exit 0**, each "test result: ok. 1 passed; 0 failed" (count 600; none
  non-zero); the binary npt_net-ed8673eeeaf4db12; the fixed hashes in its header (delay.rs 3dbbccf7…, delay_tests.rs
  8f8c0f9a…); "end 11:12:33 UTC; ... first failure: none"; main-run.sh exit 0.
- 11:13 E3b started in the background (the stalled-client test alone, `one 300 2 6
  l2::delay::tests::a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded`; log e3b.log).
- **11:13:04-11:20:49 E3b PASS: 600 of 600 exit 0**, each "test result: ok. 1 passed; 0 failed" (the name matched:
  one test each run); none non-zero; the fixed hashes in its header; "first failure: none"; main-run.sh exit 0.
- 11:21 E3c started in the background (the whole binary, `all 10 2 4`; log e3c.log).
- **E3c PASS: 20 of 20 exit 0**, each "test result: ok. 169 passed; 0 failed"; the fixed hashes; "end 11:28:05 UTC;
  ... first failure: none"; main-run.sh exit 0. E3 whole: 1,220 runs on the fixed tree, none failed.
- 11:28 E4, P1 (the finding's example: down_free without `.saturating_sub(self.released_model[DOWN])`) in by Edit.
- **P1 (p1.log; header delay.rs 091957eb…, the plant in): exit 101, "167 passed; 2 failed"**. The stalled-client test
  at delay_tests.rs:782: "held <= 64 KB + the front record's acknowledged prefix, at every write", left [1, 0], right
  [0, 0], as the table predicts. G-NC2 at :2308, the same message, left [1, 0], right [0, 0]: the table predicted a
  PASS; under C3 a G-NC2 FAIL at "...at every write" under P1 is evidence, not a stop. No other test failed. Restored
  by Edit: the three fixed hashes again (3dbbccf7…, 8f8c0f9a…, 1dd67e2f…).
- 11:30 P2 (the old bite: `if held > budget + prefix {` → `if held > budget {`) in by Edit.
- **P2 (p2.log; header delay.rs 0e65d301…): exit 101, "168 passed; 1 failed"**: G-NC2 alone, at :2308, "held <= 64 KB
  + the front record's acknowledged prefix, at every write", left [121, 0], right [0, 0] ([k >= 1, 0] as the table
  predicts: the prefix term is reached). Restored by Edit: delay.rs 3dbbccf7… again.
- 11:31 P3 (growth outside the write: the down write's `c.bytes[DOWN].extend(data);` moved after its
  `c.model_write(DOWN, n, t, false);`) in by two Edits.
- **P3 (p3.log; header delay.rs dfbf8d2d…): exit 101, "167 passed; 2 failed"**: the stalled-client test at :787,
  "held only falls between writes", left [134, 0] (the table: [k >= 8, 0]); G-NC2 at :2313, the same message, left
  [1144, 0] (the table: "G-NC2 too"). Restored by two Edits: delay.rs 3dbbccf7… again.
- 11:32 P4 (the at-write call gone: model_write's `#[cfg(test)] self.check_held_bound(dir);` deleted) in by Edit.
- **P4 (p4.log; header delay.rs 7b41e90a…): exit 101, "167 passed; 2 failed" (39.31 s)**: the stalled-client test
  at :787, "held only falls between writes", left [137, 129] (the table: [k >= 1, k' >= 1]); G-NC2 at :2290, its hang
  guard: "hang guard: 186 records, largest excess 0 B, 48 first losses" (the table: its hang guard after 30 s).
  Restored by Edit: delay.rs 3dbbccf7… again.
- 11:33 P5 (the up room forgets released bytes: up_room without `.saturating_sub(self.released_model[UP])`) in by Edit.
- **P5 (p5.log; header delay.rs 09e3fb98…): exit 101, "168 passed; 1 failed"**: the stalled-client test alone, at
  :782, "held <= 64 KB + the front record's acknowledged prefix, at every write", left [0, 172] (the table: [0, k >=
  1]); G-NC2 ok. Restored by Edit. **All five plants failed as the table says** (P1's G-NC2 FAIL at "...at every
  write" is evidence under C3); after the last restore the three fixed hashes hold and `git diff --stat -- spikes/
  netcode-proof` is the fix alone (3 files, 74 + 42).
- 11:34 E5 (the lab binary's sha256 against preflight.log's 98c3967e…).
- **E5 PASS** (e5.log, 11:33:54-11:34:23 UTC: header free 4,955 MiB, HEAD 1df9072; `cargo build --locked --release -p
  npt-lab` exit 0; npt-lab 98c3967e04e4260985d24173b1fb2126ea1444636f4725e12cf76333d90b5113, equal to preflight.log's):
  the lab binary is unaffected. **The gate --no-smoke: GATE PASS** (11:34 UTC). Step 3 done.
- 11:35 Step 4: the review round (C8): netcode-architect, test-engineer, rust-reviewer, xhigh, three at once, on the
  final working-tree diff; notes u3c-held-1/<agent>-review.md.
- **The review round (wf_88c44554-26a, 11:35-11:52 UTC): all three APPROVE WITH FINDINGS** (saved word for word by
  u3c_held_1_reviews_full.py: netcode-architect-review-full.json 015aa739…, test-engineer-review-full.json 834bf936…,
  rust-reviewer-review-full.json ac2cf470…, read in full). 0 Critical, 0 High, 1 Medium, 12 Low. The reviewers checked
  the tree they read is the tree tested (the spike diff's hash a7199f7a… in every evidence log's header).
- **The fix pass (notes only, no spike file changed, so no evidence re-runs):** NA-UH1-1 (Medium, C9 not yet met):
  the difference-set argument written into the plan's "Not looser" line with its cites (dated), the card entry cited
  by name, the saver sentence corrected; the report states it too and rests "the flake is gone" on it and on L1's run
  94, never on E3's counts. Carried into the report: TE-UH1-3 (E3 shows no false failure; it cannot show the old one
  gone), TE-UH1-4 and NA-UH1-2 (P1's G-NC2 catch is a timing one, the stalled-client test is P1's deterministic proof,
  and the design notes' F15 / F11 premise is corrected: released records do wait untaken at G-NC2's writes),
  TE-UH1-2 (cite the per-run logs p1-1-1.log:287-289 and p3-1-1.log:287-289 for "no other test failed"), TE-UH1-5 (the
  gate's PASS lines quoted with their times), TE-UH1-1 (the counted fact as "no sample above what the latest write
  left"). **Deferred as follow-ups** (spike text; a Low never triggers a push or a re-run): NA-UH1-3 (the assert message
  says "only falls", the check counts "above the latest write"), NA-UH1-4 (the up part's wording; Life's "largest
  excess" kept at writes; check_held_bound's "the only place they grow"), RR-UH1-2 (the same up wording in SERIES.md),
  RR-UH1-1 (a message on the post-hold Pending assert), TE-UH1-1's strict variant (`held_at_write[d] =
  held_at_write[d].min(len)`), TE-UH1-6 (assert both counts again before drop and in a shared helper), TE-UH1-7 ("A
  few seconds" is about 16 s), and the loop script's failure-list print (TE-UH1-2's reuse fix). Taker: the next change
  to delay.rs, delay_tests.rs or SERIES.md (on the card's Open findings).
- **11:53-11:57 Step 5, the disk and the B1 pack** (PACK-READ (a); brief 8.1's procedure; the saver bzkwerbtb stopped
  first, its last save 3a28107 on the remote): pgrep found no cargo, rustc, npt-lab, npt-judge, check.mjs or runner
  (exit 1); free 5,191,479,296 B (4,951 MiB, above the folder's 821 + 1,024); `kept-logs.mjs list u4r-B1-64f3307d5c7f
  --results spikes/netcode-proof/results/u4r/64f3307d5c7f/B1`: 11 files, 860,377,134 bytes; the spike's redact on it
  (b1-redact.log): 306 items, 0 findings; **the list commit 978202d** (that one file, the message naming
  advisor-u4r-sets-r2-full.md:45, B1's results at 73060fd and PACK-READ), pushed; **the pack: free 4,950 → 5,553 MiB**,
  archive .gate/netcode-proof-test/kept-logs/u4r-B1-64f3307d5c7f.tar.gz (227,889,786 B), "read back: 11 of 11 files
  equal the list (path, bytes, sha256)", the raw folder removed. The 5,220 MiB bar holds: P3 not packed.
- 11:57-12:13 **The spend limit cut the session** between the staging and the commit (the three files staged, the
  message written; the card's "Cuts by the spend limit" has the notice, word for word, and the times). After the
  resume the three staged files hash as fixed (3dbbccf7…, 8f8c0f9a…, 1dd67e2f…), nothing else staged.
- 12:17 **Step 6, the commit: 0ad9661657715dfbb154eb77d8fd7abce0b891c4** (`git commit -F
  .gate/u3c-held-1/fix-commit-msg.txt`: 3 files changed, 74 insertions, 42 deletions). The saver stays stopped
  (bzkwerbtb, stopped at step 5). Not pushed until E6 and the gate pass.
- 12:18 **Step 7, E6's pass bar, every item, written before the call** (the plan's step 7 as C5 amends it): the
  header's HEAD 0ad9661657715dfbb154eb77d8fd7abce0b891c4, diff and untracked e3b0c442…; check.mjs exit 0, "check: all
  11 steps pass"; test "measured build: 552 tests passed, 0 failed, 0 reported a skip; bites build: 632 tests passed, 0
  failed, 0 reported a skip" (no new test); refs 1382 (`git ls-files`, read at 0ad9661 just before the call by
  main54_counts.py; M 306; `git status --short --ignored -- spikes/netcode-proof` empty); marks 81; self 78 of 78;
  redact 10 of 10, 0 findings, 306 items (305 plus the one pack list since, B1's at 978202d); deps 137 crate files;
  wasm 27000 ticks, 0 mismatching both ways (digests never pinned). Pass is the exit code and the runner's summary,
  never a grep. One failed item: stop for the advisor, whole; never a second counted call without it. det-u4 and the
  hash regeneration stay out (D11, E5). Disk at 12:18: free 5,822,124,032 B (5,552 MiB; floor 4,096; E6 up to 1,022
  MiB net).
- **12:18:37-12:34:46 E6 PASS, every item of the bar** (task bs9tngfxi; .gate/u3c-held-1/counted.log, 29 lines, read
  whole: the long deps line at the 12:34 check-in, the rest after the run): header HEAD 0ad9661, diff and untracked
  e3b0c442…, free 5552 MiB; fmt, clippy, clippy-wasm32, doc PASS; "PASS test 804.8 s measured build: 552 tests passed,
  0 failed, 0 reported a skip; bites build: 632 tests passed, 0 failed, 0 reported a skip"; deps "(137 crate files and
  1 cargo config file(s) read ...)"; wasm "native leg 1 against native leg 2: 27000 ticks compared, 0 mismatching,
  equal ...; native leg 1 against Wasm leg 1: 27000 ticks compared, 0 mismatching, equal"; refs "1382 tracked files";
  marks "81 marks read and checked"; self "78 of 78 tests passed"; redact "10 of 10 tests passed; redact-check: 306
  item(s) checked, 0 finding(s)"; "check: all 11 steps pass"; "=== 12:34:46 UTC exit 0: node check.mjs"; "--- end
  12:34:46 UTC, exit 0" (the task's exit code 0). Free after 5,781,766,144 B (5,514 MiB; 38 MiB used net).
- 12:35 **Step 8, the gate on 0ad9661: "GATE PASS (5a smoke skipped: --no-smoke)"** (`node tools/gate.mjs --no-smoke`,
  about 12:35 UTC; base merge-base 82c605e; [1]-[4], [5b]-[5d] PASS; legacy/ unchanged; smoke is CI's job). Next: the
  card save (it pushes 0ad9661 with the notes; code ahead, so no [skip ci] and CI runs), the saver restarted, step 9.
- 12:37 **The card save aab3712 pushed 0ad9661** (head aab3712fca3af64b25169af1f4ee75c01a0411cd, no [skip ci]); the saver
  restarted as bxvsewbnz. About 12:38 **step 9's review started**: workflow wf_dd3fb0a8-14f (the advisor on main's
  advisor.md and the verifier, both xhigh, read only).
- 12:48 **CI green on aab3712** (read by the GitHub MCP tools, list_workflow_runs for the branch; the check_suite event
  of 12:41:05 UTC said nothing ran or failed): gate push 44084854535 and pull_request 44084863595 (each with its gate
  and smoke jobs), toolchain push 44084854552 and pull_request 44084863752; all four completed, success. The later card
  saves 3eef0e1 and 8354a8e touch .checkpoints/ only ([skip ci]).
- 12:53 **Step 9's review DONE** (wf_dd3fb0a8-14f; saved word for word by u3c_held_1_head_full.py and read in full):
  **the advisor APPROVE WITH FINDINGS of aab3712fca3af64b25169af1f4ee75c01a0411cd** (advisor-head-check-full.json,
  a31ed50a…; CI green, read by it too; C1-C10 met, C9's report half due at the close; ADV-UH1H-1 to -3, Low) and **the
  verifier PASS** (verifier-evidence-full.json, 174ca715…; VER-UH1-1 and -2, Low, report wording).
- 12:58 **U3C-HELD-1 CLOSED.** The close report: u3c-held-1-report.md (C9's argument by the plan's lines :132-141, run
  94 as "loop 1's run 94 (two loops, 189 runs in all)", the carried Lows TE-UH1-2, -3, -4, -5 and NA-UH1-2, the
  verifier's wording). The plan after the fix pass (C9's three dated edits only, as the advisor checked):
  d68012898843e04638de901da35b05110b6a273cf246e1251c404b1ddd591f9a (ADV-UH1H-3; approved as a53b2803…). The eight
  deferred Lows are now on the card's Open findings, under U3C-HELD-1's closed entry (ADV-UH1H-1), and the third
  pack's entry is under Key numbers beside the first two (ADV-UH1H-2). So the lines above that say "the report
  states" and "on the card's Open findings" hold from this close on.
