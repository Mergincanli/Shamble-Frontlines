# U3C-HELD-1: G-NC2's false failure fixed (test code only): the close report
2026-11-17, the day session (session_0173vVo6igCNyQYBWPvhH36A), PR #40's branch. Head reviewed and approved:
aab3712fca3af64b25169af1f4ee75c01a0411cd. The fix commit: 0ad9661657715dfbb154eb77d8fd7abce0b891c4. The step-by-step
record with every time and hash: build.md (this folder). U4's report (its step 7) folds this one.

## What broke, and why
- **The symptom:** G-NC2, the network layer's held-bytes test (npt-net, l2::delay), failed once in C62H-BOUND's first
  counted run (held_over_bound [1, 0]) although nothing grew past its limit.
- **The cause, confirmed by a run:** the release transient. A release ends the front record's acknowledged prefix
  while its bytes stay held until the stream takes them, so a sample in that window read over the bound with nothing
  grown. L1, a loop with diagnostic counters that were never committed (D-1), caught it in loop 1's run 94 (two loops
  side by side, 189 runs in all; VER-UH1-1): the one sample over the bound had a released record waiting, and none was
  at a write (l1.log:202-232; the run's log l1-1-94.log).

## The fix
- The bound (64 KB plus the front record's acknowledged prefix) is checked at every write, inside model_write, right
  after the held records grow, in both directions. Between writes, check_held_falls counts any sample above what the
  latest write left (held_grew; the counted fact is "no sample above what the latest write left", TE-UH1-1).
- The stalled-client test's post-hold write now offers a whole record, so a lost released-bytes term fails it.
- Every changed non-doc line is under #[cfg(test)]: no shipped code changes. 116 changed spike lines in three files
  (delay.rs, delay_tests.rs, marks/SERIES.md's two rows); the plan said about 130.

## Why the old flake is gone (C9; NA-UH1-1, the round's one Medium)
- The argument stands word for word in the plan, u3c-held-1-plan-draft.md:132-141 (the plan's sha256 after the fix
  pass: d68012898843e04638de901da35b05110b6a273cf246e1251c404b1ddd591f9a; approved as a53b2803…, changed since only by
  C9's three dated edits; ADV-UH1H-3).
- In short: between writes the acknowledged prefix falls only at a release, and no byte is admitted between writes.
  So a sample that the old check flagged and the new pair passes needs a release since the latest write. The fix is
  looser only on that release transient, which admits no byte, and every sample stays below 64 KB + 16,406 B.
- That the flake is gone rests on this argument and on L1's run 94, where D-1's at-write counter (the new formula)
  read [0, 0] at the sample the old check flagged (l1.log:227). It never rests on E3's pass counts: E3 shows the new
  check gives no false failure; it cannot show the old one gone (TE-UH1-3).

## Evidence (build.md has each line with its log and hashes)
- **E3, 1,220 runs on the fixed tree, none failed:** G-NC2 alone 600 of 600, the stalled-client test alone 600 of 600,
  the whole test binary 20 of 20 (169 passed each).
- **E4, the five planted bugs:** each fails on its named asserts (P1: the stalled-client test [1, 0] "...at every
  write", and G-NC2 too, a timing catch; the stalled-client test is P1's deterministic proof (TE-UH1-4, NA-UH1-2).
  P2: G-NC2 [121, 0]. P3: "held only falls between writes" [134, 0] and G-NC2 [1144, 0]. P4: [137, 129] and G-NC2's
  hang guard. P5: [0, 172].) No other test failed (the per-run logs p1-1-1.log:287-289 and p3-1-1.log:287-289;
  TE-UH1-2). Each plant was restored by hash; the verifier rebuilt each plant from 0ad9661's delay.rs and matched its
  log header's hash.
- **E5:** the lab binary's sha256 is unchanged (98c3967e…, as preflight.log:24).
- **E6, the counted check.mjs on 0ad9661** (its pass bar written in build.md before the call): "check: all 11 steps
  pass"; 552 / 632 tests passed, 0 failed, 0 skipped; refs 1382; marks 81; self 78 of 78; redact 10 of 10 with 306
  items and 0 findings; deps 137 crate files; wasm 27000 ticks, 0 mismatching both ways (12:18:37-12:34:46 UTC).
- **The gate:** "GATE PASS (5a smoke skipped: --no-smoke)" at 09:53, 11:34 and 12:35 UTC (TE-UH1-5); smoke is CI's.
- **CI green on aab3712:** gate 44084854535 (push) and 44084863595 (pull request), each with its gate and smoke jobs;
  toolchain 44084854552 and 44084863752.
- The design notes' F15 / F11 premise is corrected: released records do wait untaken at G-NC2's writes (NA-UH1-2).
- D-1 and the plants are in no commit's code (nothing under spikes/ holds them; a few notes-only commits name D-1's
  field names; VER-UH1-2).

## Reviews
- **The round** (netcode-architect, test-engineer and rust-reviewer, each xhigh, one batch): APPROVE WITH FINDINGS
  from all three; 0 Critical, 0 High, 1 Medium (NA-UH1-1, met by C9's argument above), 12 Low. One fix pass, notes
  only, so no evidence re-run.
- **The advisor's check of aab3712:** APPROVE WITH FINDINGS; C1-C10 met (C9's report half by this report); three
  Lows (ADV-UH1H-1 to -3), each met at this close: this report and the Open findings entry, the third pack's entry on
  the card, the plan's new hash recorded.
- **The verifier:** PASS, every claim backed by the logs and git; two Lows (VER-UH1-1 and -2), worded here.

## Left for later (deferred in writing; on the card's Open findings)
NA-UH1-3, NA-UH1-4, RR-UH1-1, RR-UH1-2, TE-UH1-1's strict variant, TE-UH1-6, TE-UH1-7 and the loop script's
failure-list print. Taker: the next change to delay.rs, delay_tests.rs or SERIES.md. A Low never triggers a push.

## Also in this unit
- **Disk:** the B1 raw logs packed by the kept-logs rule (free 4,950 → 5,553 MiB; list commit 978202d); P3 not packed
  (the 5,220 MiB bar held).
- **The spend limit** cut the session from 11:57 to 12:13 UTC, between the staging and the commit; nothing was lost
  (the card, "Cuts by the spend limit").

## For Mustafa
Nothing: test code only, nothing a player sees, hears or feels.
