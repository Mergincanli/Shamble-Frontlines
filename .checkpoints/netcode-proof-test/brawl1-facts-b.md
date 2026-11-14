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
  is results/u4r-precheck-30/faf42aeb034c/ at d666aeb (old brawl). Walls: record 127.5 s, clean 192.8, wallclock 177.4,
  late-command 174.8; start/end uptime 26169/26854 s (685 s of runs); kept logs 17 MiB (sets-r2-full :65). Its pair
  output prints "World::new(scene, 4, 0)" (runs.txt:129, from precheck.rs:691 and :705).
- F13 a third room build, test-only: lab_room scenario.rs:2018-2034 (World::new, Room::new(n), with_hashes,
  Server::new(Quant::default())), used by r8, r9, fp1 and others.
- F14 Bites (scenario.rs:168-) derives Default; Bites::default()'s world()/room()/quant()/net()/schedule() equal
  World::new / room / Quant::default() / L1Net::new / Schedule::new (scenario.rs:257-326), i.e. run_leg's own calls
  (precheck.rs:296-321). So a shared constructor called by run_leg with Bites::default() and event start 0 builds the
  same room as today. The reviewer's signature `server(scene, seed, first_tick, clients, bites)` lacks event_start and
  arm_tick, which run needs (scenario.rs:746, :760): open point.
- F15 no tool pins the three runner recordings' sha256 outside the manifests (grep of 6933b56a/5b79abb2/fad8f67d over
  *.mjs/*.rs/*.json/*.md outside results/); check.mjs pins no test count (grep 531/609 empty). hash.rs h5 :1504-1527
  pins only the manifests' names, kinds and generator.
- F16 other homes that may fall in this lane only if BRAWL-1's plan opens their files: EA-U4R-4 (the next lane that opens
  crates/sim; u4r-build.md:1941, u4r-report.md:245), T-6's contract half (the next unit that edits contract.rs;
  advisor-u4r-r1-full.md:122), JDG-ONLY-1 (npt-judge's next change; advisor-u4r-c62h-full.md:20, :47), the check.mjs
  follow-up (check.mjs's next change; advisor-u4r-r2-full.md:24). Adjacent but homed elsewhere: EA-U4R-5 (U6 brief:
  one route for feeding the endpoint; bots lib.rs doc), NC-U4R-7 (carry 18's route). The P2 attribution re-run: "the new
  home is BRAWL-1's lane, with the re-run of -17 L1 P2 at the fix's commit" (advisor-u4r-f2-head-full.md:49).
  TICK16-WORDING's expected line applies from BRAWL-1's re-run sets on (advisor-u4r-sets-r2-full.md:117).

## Per item (B1-B6)

### N2-1 with TE-R2-1
- B1: advisor-u4r-r2-full.md:73 (N2-1): "closed_checks (scenario.rs:2150-2164) gains a fourth named check: `if
  server.room.roster().is_live(slot) { failed.push(format!("the roster still holds slot {slot}: a rejoin would be a
  replacement")); }`. The 'no replacement' assert on `ended` (:2209-2212) is dropped, or its doc says it guards only the
  lab's own push. Under the leave plant (room.leave deleted), all four checks fail by name in both cases." Home :74 (the
  next npt-lab lane, with EA-U4R-1/-2, RR3-L2 (a), TE-R2-2, RR4-L1, RR4-L2). TE-R2-1 :66: "The same finding as N2-1,
  fixed once by N2-1's follow-up: the roster check added to closed_checks, and the no-op 'no replacement' assert dropped
  or reworded." (:67 home N2-1's follow-up.)
- B2: lines unchanged at HEAD (F0, F1). closed_checks :2150-2164 holds three checks (room.client :2152-2156, owner
  :2157-2159, ended (slot,"closed") :2160-2162); doc :2148-2149 says "FP-1's three checks". Test fp1 :2170-2213, doc
  :2166-2169 ("admitted, and not as a replacement"); asserts :2204-2212 (owner == again; !ended.contains((0,"replaced"))).
- B3: add the check as quoted; message exactly "the roster still holds slot {slot}: a rejoin would be a replacement".
  Open: (i) drop :2209-2212 or keep it with a doc that it guards only the lab's own push; (ii) closed_checks' doc
  "three" -> four, and fp1's doc :2168-2169 "and not as a replacement" reworded to rest on the roster; (iii) the ruled
  expectation "all four checks fail by name in both cases" cannot hold under the leave plant: owner.remove (:500) and
  ended.push (:502) still run, so only the client check fails today (c6-plant-fp1.log:20-23) and with the new check two
  of four would fail per case: ["the room still holds slot 0: its slot was not freed", "the roster still holds slot 0: a
  rejoin would be a replacement"]. The plan must word the expected plant output (or ask the advisor); an optional second
  plant (room.rs:691 `self.roster.leave(slot)` deleted, in npt-room) would make the roster check the only one that fails.
- B4: extends fp1 (scenario.rs:2170) through closed_checks; plant = the FP-1 leave plant at scenario.rs:499, restored
  by Edit with sha256 checked; test count unchanged (same fn).
- B5: test-only, both builds; no recording, no run output, no pin changes. Independent of BRAWL-1's order.
- B6: about 8-12 changed lines (+6 check, ~3 doc, -4 or reworded assert).

### TE-R2-2
- B1: advisor-u4r-r2-full.md:80: "fp3's parse test gains the missing cases, each asserting the exact usage error.
  First, `--profile B1 --bursts 2 --burst-first-s 0 --burst-every-s 1000000000000000000`: the error starts with
  '--bursts 2, --burst-first-s 0 and --burst-every-s 1000000000000000000: the last burst's tick overflows 64 bits'.
  Second, `--warmup-s 310000000000000000 --measured-s 310000000000000000`: the error equals '--warmup-s and --measured-s:
  their ticks' sum overflows 64 bits'. Third, an --after-max-s case for the last refusal. Made in the same change as
  RR4-L2, so the cases test the final messages." Home :81 (the next npt-lab lane, with RR4-L2).
- B2: unchanged lines: fp3 scenario.rs:2420-2431 (one case, `--profile P0 --measured-s 10^18`, starts_with check);
  sum refusal :1376-1378; after-max refusal :1379-1381 ("--after-max-s: the run's last tick overflows 64 bits");
  last-burst check :1442-1453 (comment :1442, block :1443-1453, message :1450-1451; the ruling's :1443-1454).
- B3: case 1 needs a B1 base line (fp3's base uses --profile P0); arithmetic: 1 x 10^18 x 30 > u64::MAX, refused by the
  64-bit chain before any RR4-L2 cap (default lengths 3,000 ticks; bursts 2). Case 2: 3.1e17 x 30 = 9.3e18 each (fits);
  sum 1.86e19 > 1.8447e19: equals the quoted message. Case 3 open: values and expected text; e.g. --warmup-s 3e17
  --measured-s 3e17 --after-max-s 1e17 gives the 64-bit after-max message, while once RR4-L2's u32 cap exists any
  --after-max-s over 143,165,507 s (with defaults) gives the new cap's message instead. The plan says which refusal the
  third case pins (or both). Constraint for RR4-L2: keep the 64-bit refusals' messages and checks first, else case 1's
  ruled prefix changes.
- B4: extends fp3_a_length_whose_ticks_overflow_is_a_usage_error_naming_its_option (scenario.rs:2423); count unchanged.
- B5: test-only; no recording change. Shares parse() with RR4-L2 and with any option BRAWL-1 adds to parse.
- B6: about +20 to +30 lines.

### RR4-L2
- B1: advisor-u4r-r2-full.md:101: "parse refuses these as usage errors that name their options: a run's last tick
  (measured_end + after_max_ticks) above u64::from(u32::MAX), and the last burst's tick above the same; --bursts above
  1,000; and --burst-every-s 0 with more than one burst. The comment at scenario.rs:1442 says exactly what is bounded.
  Add one parse case: `--profile B1 --bursts 2 --burst-every-s 600000000000`, refused, naming --burst-every-s." Home
  :102 (the next npt-lab lane, with TE-R2-2).
- B2: unchanged lines: parse :1313-; measured_end :1376-1378 and its +after_max check :1379-1381 (result discarded);
  bursts parsed :1388-1390 (defaults 10, 30, 60) for every profile; B1 check :1442-1454 only `profile_name == "B1" &&
  bursts > 0`; burst_starts :1295-1310 (plain u64 `warmup_ticks + first_s * hz + i * every_s * hz`, `boundary +
  below(..)`); due_ns :130-132 (`as u64`); :862 `(measured_end.max(1) - 1) as u32`.
- B3: four refusals + comment + one case. Ruled case arithmetic: 300 + (600,000,000,000 + 30) x 30 =
  18,000,000,001,200 (fits u64, above 4,294,967,295) -> refused by the new last-burst cap, message must contain
  "--burst-every-s". Open: exact messages (only "name their options" is ruled); which options the run's-last-tick
  message names; whether the burst caps apply to B1 only (today's guard) or to every profile; order (the 64-bit checks
  first, for TE-R2-2's ruled prefix); the new comment text at :1442; whether to add cases for the bursts cap, the
  every-s 0 rule and the run's-last-tick cap (not ruled). With the caps, due_ns(u32::MAX) = 1.43e17 ns fits and :862's
  cast becomes lossless (no edit ruled there).
- B4: extends fp3 (same fn as TE-R2-2); count unchanged unless the plan adds a new test fn.
- B5: no run script is refused (F5); recordings unchanged (record() argv lengths come from manifests whose ticks are
  9,000). If BRAWL-1 adds a parse option (a leash setting), both edit parse() and its USAGE (:1286-1291).
- B6: about +25 to +40 lines (refusals ~20, comment 2, case 6-8).

### RR4-L1
- B1: advisor-u4r-r2-full.md:94: "hash.rs gets `const MAX_ENTRY_TICKS: u32 = 36_000;` (four times today's 9,000, the
  same rule as the 64 MiB cap). manifest() refuses any entry above it, beside the bytes cap, with: 'manifest entry
  {name}: ticks {ticks} is above 36000 (four times today's largest entry): a longer recording comes to the advisor
  first'. Add one r10_fp3 case: 36,001 refused by name, and the three committed manifests still read." Home :95 (the
  next npt-lab lane, and at the latest before U4h adds its runner entry to det-recordings.json, carry 87).
- B2: hash.rs:229-237 constants; :319-324 ticks <= bytes/12; :325-331 bytes cap; runner checks :332-; r10_fp3
  :1591-1650. Unchanged since the merge.
- B3: const + one refusal after the bytes cap and before `if let Kind::Runner` (else a runner text with ticks 36,001
  is refused first by the runner ticks check, "ticks 36001 is not (warmup_s 10 + measured_s 290) x 30 (9000)").
  Case: `refused("\"ticks\": 9000", "\"ticks\": 36001", ..)`: bytes 999,999 / 12 = 83,333 >= 36,001, so the new cap
  fires; expected "manifest entry SS-P2-s1: ticks 36001 is above 36000 (four times today's largest entry): a longer
  recording comes to the advisor first". The committed-manifests loop :1636-1649 already reads all three. Open: whether
  to show 36,000 passes (a synthetic `one("TC-s3", 36_000, k)` entry: 799,693 / 12 = 66,641 >= 36,000).
- B4: extends r10_fp3 (hash.rs:1594); count unchanged.
- B5: all six committed entries are 9,000 ticks (F4); BRAWL-1's re-pins keep 9,000 ticks, so they pass. Not a
  recording change. Precheck reads hash::manifest (precheck.rs:632), so the cap guards that manifest too.
- B6: about +12 to +18 lines.

### EA-U4R-1
- B1: advisor-u4r-r1-full.md:169 (Where it goes): "A STATUS follow-up for the next npt-lab lane, before the -30
  pre-check's re-run after U4h (carry 40): one shared loop for RQ8's steps 1-3." :170 (Why): "...precheck.rs:350 runs a
  plain u64 `now`, while its doc (:3) says SimClock, and lib.rs:3-5 places items where they are not. The doc corrections
  and the report's public-items line are made now. Sharing one loop would refactor the pre-check's run order, and its
  evidence would then have to be made again, so that part is kept out of this fix pass." FP-7 item 2 (:15) did the docs.
  r2 ruling :24 and :74 home it in this lane. Reviewer: u4r-review-r1-full.md:193-198 (fix :198: SimClock in run_leg,
  card follow-up "one shared loop for steps 1-3 before the pre-check is re-run after U4h").
- B2: done docs at HEAD: precheck.rs:3 "each leg on its own simulated clock (integer ns)"; lib.rs:3-7. Open code:
  precheck.rs:372 `let mut now = 0u64;`, :475 `now = next.max(now + 1);` (moved from :350/:471); run_leg's steps 1-3
  precheck.rs:374-453; runner's scenario.rs:879-975 with SimClock :859 and advance_to :1105; next-instant computation
  scenario.rs:1097-1105 against precheck.rs:471-475.
- B3: open (the ruling names only "one shared loop for RQ8's steps 1-3"): the shape (a shared function for steps 1-2
  and the due count, with each caller's per-tick body; or one loop driver taking the per-tick body and step 5 as
  closures/trait); whether run_leg moves to SimClock (the reviewer's fix); where admitted/ended go (runner: logs.admit
  with carry 75's bound and logs.gone; pre-check: its own maps and o.ended); the per-tick body differs (pre-check:
  Room::script, tick-base check, hashes, depth, take/send over replay clients only; runner: recording, mirror, logs,
  bursts, take/send over 0..n). Today's order is already the same in both (F7).
- B4: no test of the loop order itself; existing coverage: precheck r6 :901, precheck_bite_late_command :995,
  precheck_bite_wallclock :1044 (run_leg), s5 :1857 and r1 (scenario::run). Evidence the ruling implies: the pre-check's
  evidence made again (the pre-check set's three pairs), plus det-u4.mjs and `npt-lab hash --manifest
  crates/lab/precheck-recordings.json` byte-equal at the old pins.
- B5: see "Order" below. It edits scenario::run's tick loop, where BRAWL-1 (b)'s building-area figure would be
  computed after room.tick.
- B6: narrow form (shared steps 1-2 + due, SimClock in run_leg) about 40-70 changed lines; a single loop driver with
  closures about 150-250.

### EA-U4R-2
- B1: advisor-u4r-r1-full.md:174: "A STATUS follow-up for the same next npt-lab lane as EA-U4R-1, before the
  pre-check's re-run after U4h: `pub(crate) type Sim = npt_sim::World` and one server constructor used by run and
  run_leg." :175 (Why): "...npt_sim::World is named in logs.rs:97, :455 and :578 and in precheck.rs:277, and the room is
  built in two places, against U4-SEAM (u4-build-plan.md:162-163). Refactoring both construction paths inside a fix pass
  risks behaviour, and no current result differs." Reviewer fix u4r-review-r1-full.md:205.
- B2: F6, F13, F14. Moved: logs.rs :455 -> :472, :578 -> :595; precheck.rs :277 -> :296 (Room::new :301-309,
  Server::new :310); run's build scenario.rs:742-766.
- B3: `pub(crate) type Sim = npt_sim::World;` (in scenario.rs per the reviewer); one `pub(crate) fn server(..) ->
  Server` used by run and run_leg. Open: its parameters (event_start and arm_tick are missing from the reviewer's
  signature; run needs both); whether lab_room (tests) uses it; whether scenario.rs's own World sites (:42, :212, :268,
  :273, :277, :280, :343, :363) and hash.rs's bite sites (:532, :546, :1378, :1714, :1887) move to Sim; keep the printed
  "World::new(scene, ..)" strings (precheck.rs:691, :705) unchanged so runs.txt stays comparable; precheck.rs:6-8's
  module text.
- B4: no new test needed for a pure refactor; evidence: check.mjs counts unchanged, the precheck tests, det-u4.mjs and
  the precheck regeneration byte-equal at the old pins.
- B5: see "Order".
- B6: about 60-90 changed lines (+25 constructor, -20 run, -15 run_leg, ~6 logs.rs/precheck.rs type sites, +2 alias;
  -12 more if lab_room uses it).

### RR3-L2 (a)
- B1: advisor-u4r-r1-full.md:229: "Part (a): a STATUS follow-up for the next npt-lab lane. On a no-log run `filed`
  keeps only what `started` needs." :230 Why (a): "On a no-log run, `filed` grows `res` for the whole run, about 7 MB for
  a 9,000-tick regeneration: bounded, so it waits." Reviewer u4r-review-r1-full.md:276-279 (fix (a): "When Logs has no
  folder, make `filed` keep only first and last (all `started` needs), or skip `res`").
- B2: F8. (b) done at scenario.rs:886-889 (FP-7 item 6).
- B3: Logs gains a flag set from open's `dir.is_some()`; on a no-log run filed calls only resolved() (first/last) and
  never entry(). Open: keep `last` (finish's `inside` reads it; started needs admitted + first only); the bites path
  (`bitten`) on a no-log run; the conflicts count (written stays None on a no-log run, so it never fires there).
- B4: no existing test covers it; a new logs.rs unit test (Logs::open(None, ..), admit, filed over many ticks: res
  empty, started true) would add one test to each build's count; s5 (record_mem) and the precheck tests'
  short_recording() exercise the no-log path whole (the warm-up check scenario.rs:981-990 reads started()).
- B5: no recording byte and no printed figure changes (summary_line prints "not counted (--logs 0)" for log figures).
  Judged runs keep logs, so the re-run sets are untouched by it.
- B6: about +25 to +35 lines with the test (+8 code).

## Order (B5 across the items)
1. No pin moves for N2-1, TE-R2-2, RR4-L2, RR4-L1, RR3-L2 (a): each proves itself by unit tests; any order.
2. EA-U4R-1 and EA-U4R-2 are pure refactors of the paths that make the three runner recordings (scenario::run via
   record) and the pre-check legs (run_leg). Their no-behaviour-change proof is byte-equality at the OLD pins:
   det-u4.mjs (SS-P2-s1 5b79abb2, ES-P3-s2 fad8f67d) and `npt-lab hash --manifest crates/lab/precheck-recordings.json`
   (SS-P0-s4 6933b56a), plus the pre-check pairs as before (comparable count for count with d666aeb's runs.txt only on
   the old recording). BRAWL-1 changes the brawl, so all three pins move; built in one step with BRAWL-1, the refactors'
   proof is lost. So: refactors (and the Lows) first, checked at the old pins; then BRAWL-1; then re-pin
   det-recordings.json and precheck-recordings.json; det-u4.mjs and the precheck regeneration PASS at the new pins.
3. The pre-check set (3 pairs, about 685 s of runs plus two release builds, about 17 MiB) refuses a recording whose
   bytes or sha differ from the manifest (precheck.rs:647-656) and re-records at the manifest's settings: after the
   re-pin, the old runs.txt (d666aeb) no longer matches the tree. Whether it is re-made in this lane (the ruling's
   "evidence made again") or at U4h's re-run (carry 40) is open for the plan.
4. The re-run sets (P3, P2, P0 with bites) run scenario::run: every scenario.rs change (EA-U4R-1/-2, RR4-L2, BRAWL-1)
   must be in the commit they run at; sets are run with no commit under spikes/netcode-proof between them except each
   set's results/ (advisor-u4r-f1-plan-full.md:14).
5. Shared edit sites with BRAWL-1: scenario::run's tick loop (EA-U4R-1; BRAWL-1 (b)'s figure), logs.rs room_tick's
   `&Room<World>` (EA-U4R-2; if the figure is logged there), parse() and USAGE (RR4-L2/TE-R2-2; any BRAWL-1 option).

## Summary (returned)
See the StructuredOutput items.
