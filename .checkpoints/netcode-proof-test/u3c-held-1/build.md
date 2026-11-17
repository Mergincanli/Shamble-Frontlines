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
