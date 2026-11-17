# U3C-HELD-1 plan drafter: notes

Role: the plan drafter (read only; writes only this file and u3c-held-1-plan-draft.md). Ran nothing.

## Plan of my work
1. Read the card's finding (netcode-proof-test.md:3870-3885), merge.log:51-52, c62h-bound-build.md:426-455,
   the advisor's merge-stop ruling.
2. Read both design notes in full (netcode-architect-design.md, test-engineer-evidence.md).
3. Re-check the key cited code: delay.rs (check_held_bound, held_now, down_free, model_write, harvest, pump's
   loops and closing block, the two growth sites), delay_tests.rs (G-NC2, the stalled-client test), SERIES.md
   rows, u3c-plan.md:122-125.
4. Read check.mjs's test step and FX-10, main-run.sh's usage and floor, plan.md §12.4, brief 8.1, R4,
   history.md:2187 (the one-command-per-call rule).
5. Note every disagreement between the two notes.
6. Write the plan draft (under about 220 lines).

## Findings (one line each, with sources)
- D1 HEAD is 6d24666 (git log -1); the spike tree is unchanged since 43b02e8 (`git diff --stat 43b02e8 HEAD --
  spikes/netcode-proof` prints nothing), so the two notes' line numbers (read at 25b262f / 43b02e8) hold.
- D2 Code re-read and matches the architect: check_held_bound delay.rs:527-543 (every-sample, both directions), called
  only from held_now :518-519; down_free :473-477; model_write :598-613 (no check today); down growth :1574 then
  model_write :1592 then held_now :1594; up growth :806 then model_write :807; release in harvest :684-685; take
  :1748 then pop/released_model -= :1759-1761; down loop's release check :1727; pump's up-loop step By::Mid :1814;
  closing block held_now :1879; R1 comment :1869-1872.
- D3 My own check of F7 (bound holds at a write): held = sum(len of recs) - front_done; model bytes of recs = W - F =
  released_model + (W - R); W - R = outstanding + (snd_una - R). Admission n+22 <= free_budget - released_model gives
  outstanding + released_model <= 65,536 after the write, so held <= 65,536 + max(0, snd_una - R) - 22N - front_done.
  Agrees with the architect.
- D4 The stalled-client test (delay_tests.rs:741-839) is QUIET (no loss: quiet() sets loss_ppm 0, :52-55; used at
  :749), so its prefix is always 0; it also reaches the UP bound (max[UP] >= up_least, :816-822): the at-write UP
  check gets exercised there (answers the test-engineer's open question on UP, by reading only).
- D5 Loss is drawn per packet from a seeded per-path RNG (tcp_model.rs:2592-2604); `forced` (tcp_model.rs:2049) is a
  private field used only by tcp_model/tests.rs (:90, :144, :176, :202, :1046); only `with_loss_from` is pub(crate)
  (:2261). So a delay_tests test cannot force a loss without a new hook (architect §4 is right on this).
- D6 DISAGREEMENT 1 (T-NEW): the test-engineer's E2/F13 say a new test with S never reading and P2 loss shows the cause
  "on every run"; the architect (§4) says a deterministic reproduction needs a forced-loss hook (not proposed) and
  proposes no new test, putting the plant in the stalled-client test instead. My reading: with S never reading only
  about 64 KB + back-fill is ever sent (about 45-56 segments of 1,448 B), so whether a loss lands inside a record past
  its first segment depends on the seed's draws and on the packet order; "every run" is not shown. Open question.
- D7 DISAGREEMENT 2 (where the finding's plant bites): both agree P1 cannot bite reliably in G-NC2 (architect F15,
  test-engineer F11). Architect: put it in the stalled-client test with a RECORD_MAX post-hold write (predicted
  [1, 0]). Test-engineer: put it in T-NEW. Not a conflict on the facts; a conflict on the vehicle (tied to D6).
- D8 DISAGREEMENT 3 (the coverage count): test-engineer E1 and P-c want a test-only count of write-time checks (at
  least `records` in G-NC2) and a plant that removes the down call; the architect's design has no such count but
  has check_held_falls + P3 (growth outside the checked write). Different guards for "every write is checked".
- D9 DISAGREEMENT 4 (plant runs' build): the architect runs plants by name with `cargo test -p npt-net --lib
  --all-features -- <name> --exact` (its §3); the test-engineer F3 shows that builds a different binary (301de27,
  without npt-l1's bites) and wants the whole ed8673 binary via the workspace bites build. The plan should use the
  test-engineer's build (it is check.mjs's own) and run the named tests from that binary.
- D10 DISAGREEMENT 5 (npt-lab): the architect's §2 repeats the advisor's "npt-lab does not depend on npt-net"; the
  test-engineer F14 shows it does transitively (lab Cargo.toml:30,32 -> server :29, bots :25). To check.
- D11 Checked D10: the manifests do chain (crates/lab/Cargo.toml:30,32; crates/server/Cargo.toml:29;
  crates/bots/Cargo.toml:25), but `git grep -n npt_net -- crates/server crates/bots crates/lab` prints nothing: no
  source there names npt-net. So the advisor's sentence is wrong on manifests, right in effect (no npt-net code is
  used by npt-lab). Consequence: E5 (npt-lab release sha256 98c3967e…, preflight.log:24) shows the lab binary is
  unaffected, but it proves nothing about npt-net's own non-test code; that rests on a diff audit (every changed
  non-doc line inside a #[cfg(test)] item or statement).
- D12 DISAGREEMENT 6 (plants' place): architect §3 "a worktree copy"; test-engineer E4 Edit in the main folder with
  hashes before/after. main-run.sh runs only in the main folder (main-run.sh:13-14, :26: cd $S), and the TE-M3
  precedent restored plants by Edit (test-engineer F9). Main folder is the only way through main-run.sh.
- D13 DISAGREEMENT 7 (the loop's bytes): architect §4: HEAD + D-1 counters (confirms the cause); test-engineer
  block 1: HEAD unchanged (L1), then L2, with a probe or T-NEW for the cause. Both agree unchanged code cannot
  confirm the cause (architect §4; test-engineer F7). Open question.
- D14 Plant P4 (the at-write call removed) is caught by the architect's own falls check: held_at_write is set only in
  check_held_bound, so without the call it stays [0, 0] and every non-empty sample counts in held_grew; G-NC2's loop
  then never sees beyond > 0 (set only at writes) and fails at its hang guard (delay_tests.rs:2283-2286) after HANG
  = 30 s (:38). So the test-engineer's P-c needs no new counter in this design (predicted, reading only).
- D15 P3 under the architect's design also weakens G-NC2: beyond is computed at the write without the moved bytes,
  so G-NC2 may end at its hang guard instead of the held_grew assert; the plan must list every test a plant may fail.
- D16 Plant P5 (up room forgets released bytes, delay.rs:482-483): the stalled-client test floods 300 frames of
  500 B (delay_tests.rs:753-757) that tungstenite never reads, and reaches the up bound today (:816-822); with
  released_model[UP] dropped, acknowledged up records free the room, the pump reads on, bytes[UP] passes 64 KB at a
  write: the new assert fails with [0, k], k >= 1, before the max[UP] assert (:798). Predicted; drafter's addition
  for the test-engineer's UP question.
- D17 Drafter's note (unverified, for netcode-architect): a prefix arises without loss when a record spans two
  slow-start flights: INIT_CWND 10 (tcp_model.rs:94) x MSS 1,448 (:87) = 14,480 B, while the stalled test's second
  record covers model bytes 8,214-16,428. A quiet stalled-client writer that polls through the first round trip
  could back-fill that prefix and then see the record released untaken: a deterministic transient for the old
  check, no forced-loss hook needed. Goes into the T-NEW open question only.
- D18 SERIES.md rows are not pinned by marks: marks.json's 81 row_sha256 entries point only at docs/adr/0001 (63),
  0004 (1) and 0006 (17) (`grep -o '"file": ...' | uniq -c`); check.mjs:1894-1906 hashes only rows starting
  `| <mark id> |`. Rewording SERIES.md:613/642 changes no marks hash (supports architect F17).
- D19 The saver adds only its checkpoint folder (checkpoint.mjs:155) but pushes the branch head (:132), so a local fix
  commit is pushed by the next autosave: stop it from the fix commit to the card save (precedent
  c62h-bound-build.md:424).
- D20 Now: free 5,231,706,112 B = 4,989 MiB (df at about 08:40); `git ls-files | wc -l` 1357; delay.rs sha256
  aaa6a588…, delay_tests.rs 74d49de7…; architect notes ae88c5cd…, test-engineer notes 1c06246c….
