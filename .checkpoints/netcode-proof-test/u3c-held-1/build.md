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
