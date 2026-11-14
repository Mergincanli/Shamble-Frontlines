# brawl1-facts-b: the other npt-lab lane items (facts for BRAWL-1's plan)

Agent: facts-b (read-only). Branch claude/netcode-proof-test-plan-hcky5g, main folder.

## Plan
1. Read the rulings: advisor-u4r-r2-full.md (N2-1, TE-R2-1, TE-R2-2, RR4-L1, RR4-L2, item 18),
   advisor-u4r-r1-full.md (EA-U4R-1, EA-U4R-2, RR3-L2, FP-7 items), u4r-review-r1-full.md, u4r-review-r2-full.md,
   advisor-u4r-sets-r2-full.md (BRAWL-1 Fix a-d) for the interaction.
2. Per item: B1 quote + source line; B2 code at HEAD (file:line, moved?); B3 exact change + open points; B4 test;
   B5 interaction with BRAWL-1 / recordings; B6 size.
3. Grep notes for "next npt-lab lane" for other items.
4. Summary per item at the end.

## Findings (a line per finding, file:line)
- F0: crates/ unchanged since the merge 71ee13d (`git log -- spikes/netcode-proof/crates` tops at 71ee13d); round 2
  lines (snap2 = merged bytes) equal HEAD; round 1 lines (snap1 572c4f69) moved by the fix pass.
- F1 N2-1: closed_checks scenario.rs:2148-2164 (doc :2148-2149 "FP-1's three checks"); fp1 test :2166-2213 (doc
  :2166-2169 says "and not as a replacement"); the 'no replacement' assert :2209-2212; drop_slot :408-422 ('replaced'
  push only on a live old session :411-419); closed branch :491-500. Room::roster room.rs:568, Room::leave room.rs:690-695
  (roster.leave :691), Roster::is_live roster.rs:80. All pub.
- F2 N2-1 plant fact: under the leave plant only ONE of today's three checks fails (owner.remove and ended.push still
  run): c6-plant-fp1.log:20-23 (.gate/u4r/), u4r-build.md:1649-1654, :2089-2090. With the roster check, two of four would
  fail (client + roster), not "all four" as the ruling's text says: an open point.
- F3 parse: sum refusal scenario.rs:1376-1378, after-max :1379-1381, last-burst check :1442-1454 (comment :1442),
  burst_starts :1295-1310 (plain u64 sums :1306-1307), due_ns :130-132 (`as u64`), :862 `as u32`. fp3 test :2420-2431
  (only --measured-s). All lines as the round-2 rulings name them.
- F4 RR4-L1: hash.rs constants :229-237 (TICK_HEADER_BYTES :231, MAX_ENTRY_BYTES :234, RUNNER_PROFILES :237);
  manifest() :248-; ticks<=bytes/12 :319-324; bytes cap :325-331; runner checks after it (:332-). r10_fp3 test
  :1591-1650 (runner text ticks 9000, bytes 999,999; committed manifests read :1636-1649). Committed manifests: all
  six entries 9,000 ticks (recordings.json SS-s1/ES-s2/TC-s3 synthetic; det-recordings.json SS-P2-s1, ES-P3-s2 runner;
  precheck-recordings.json SS-P0-s4 runner, sha 6933b56a6191). A runner text with ticks 36001 hits the new cap first only
  if the cap sits before the runner checks (beside the bytes cap) - else "ticks 36001 is not (10 + 290) x 30".
- F5 scripts' lengths: c-u4r-r2.mjs:64 B1 `--bursts 2 --burst-first-s 5 --burst-every-s 15`; c-u4r-runs.mjs:23 B1 10
  bursts every 60 s from 30 s (defaults scenario.rs:1388-1390: 10, 30, 60). No script passes --burst-every-s 0 or
  bursts > 10; RR4-L2's caps (1,000; u32::MAX ticks) refuse none of them.
- F6 EA-U4R-2 sites at HEAD (moved from snap1): logs.rs:24 (use), :97 `world: &World`, :472 `room: &Room<World>` (was
  :455), :595 `world: &World` (was :578); precheck.rs:50 (use), :296 `World::new` (was :277); the two room builds:
  scenario.rs:742-766 (run: bites.world, Room::new(n), with_hashes, with_event_start(s.event_start), bites.room,
  bites.quant, Server::new, decoder_bite if pump) and precheck.rs:284-318 (run_leg: World::new, Room::new(cmds.len()),
  with_hashes, Server::new(Quant::default()), decoder_bite if pump bite; no with_event_start (default next_event 0,
  room.rs:459) and no Bites::room). U4-SEAM: u4-build-plan.md:162-164 "npt-server and npt-lab name `npt_sim::World` once".
- F7 EA-U4R-1 at HEAD: precheck.rs:3 already reads "each leg on its own simulated clock (integer ns)" (FP-7 item 2
  done); lib.rs:3-7 reworded; run_leg's loop precheck.rs:372-476 (`let mut now = 0u64` :372, `now = next.max(now + 1)`
  :475; was :350/:471); runner loop scenario.rs:859-1106 (SimClock :859, advance_to :1105). SimClock scenario.rs:66-82.
  Shared steps today: (1) poll_accept/accept, (2) server.io + take admitted/ended, (3) due_ticks + room.tick + take/send
  per slot; written twice (scenario.rs:880-905 + :903-975 against precheck.rs:374-453).
- F8 RR3-L2 (a): Logs::open(dir None) on a no-log run (scenario.rs:779-780 `s.logs.then_some(..)`); filed logs.rs:325-363
  inserts into `res` via entry() :292-295; only write_client_tick :389-391 removes; flush :452-465 walks `phases`, filled
  only by room_tick :480 (not called when !s.logs, scenario.rs:971). started() :259-263 needs admitted + first. finish
  :689-733 writes the rest of `res` (counts.hashes/sentinels :418-419), but summary_line prints "not counted (--logs 0)"
  for those on a no-log run (scenario.rs:1532-1573), so emptying `res` changes no printed figure. Logs keeps no
  "has a folder" flag today: the change needs one (from open's `dir.is_some()`).
- F9 plant site for N2-1: Server::io's closed branch scenario.rs:493-503 (`self.room.leave(slot)` :499, owner.remove
  :500, ended.push((slot,"closed")) :502). Room::leave's roster half room.rs:691 (`self.roster.leave(slot)`).
- F10 pins that hash the brawl (runner kind, default behaviour brawl; record() argv scenario.rs:589-635 passes no
  --behaviour): det-recordings.json SS-P2-s1 (5b79abb2..., 16,937,652 B) and ES-P3-s2 (fad8f67d..., 16,929,501 B) read by
  ops/runs/det-u4.mjs:27; precheck-recordings.json SS-P0-s4 (6933b56a..., 16,950,444 B) read by c-u4r-runs.mjs:332 and
  checked by `npt-lab precheck` against bytes + sha256 (precheck.rs:631-660). recordings.json's three are synthetic
  (check.mjs:2170, the wasm step). hash.rs:1506-1525 and :1636-1649 include_str! all three manifests.
- F11 precheck tests (precheck.rs:784-1057): short_recording() :786-792 calls scenario::record (brawl, SS P0 seed 4,
  4 s warm-up): r6 :901, precheck_bite_late_command :995, precheck_bite_wallclock :1044, fp4 :851, r7 x4. They compare
  legs, so a changed brawl changes their inputs, not their verdicts.
- F12 the pre-check set (c-u4r-runs.mjs:391-425): re-records SS-P0-s4 with `scenario --record` at the manifest's settings,
  then 3 pairs (clean PASS, wallclock FAIL, late-command the whole line "result: VOID (L1 late_count 1)"); its runs.txt
  is results/u4r-precheck-30/faf42aeb034c/ at d666aeb (old brawl).
