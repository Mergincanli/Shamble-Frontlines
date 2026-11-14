# Advisor: plan review of BRAWL-1's lane (npt-lab, PR #40)

## My plan
1. Read brawl1-plan.md (sections 1 to 13, Q1 to Q8).
2. Read my rulings: advisor-u4r-sets-r2-full.md (BRAWL-1, Fix (a)-(d), next steps 4-6); advisor-u4r-r2-full.md (item 18,
   N2-1, TE-R2-1, TE-R2-2, RR4-L1, RR4-L2); advisor-u4r-r1-full.md (EA-U4R-1, EA-U4R-2, RR3-L2 (a)); U4r build ruling 1
   point 7 (R16); FP-7 item 1 / carry 39; advisor-u4r-f3-plan-full.md (start figures).
3. Read brawl1-facts-full.md.
4. Check against code: Fix (a) source (client decoded snapshot vs room mirror); leash geometry; constants not
   rate-derived; Fix (b) figure; Fix (c) evidence + sim-only seam; runner recordings re-pin; det-u4.mjs 240 s; N2-1
   plant; TE-R2-2/RR4-L2 order; RR4-L1 place; EA-U4R-1/-2 form; order of work; disk plan; expected lines; reviewers;
   size.
5. Kept-logs draft (Q7).
6. Verdict.

## Checks (a line each as I go)
- C1 read: plan (13 sections, Q1-Q8); sets-r2 (BRAWL-1 Fix a-d, steps 4-6); r2 (item 18, N2-1, TE-R2-1/2, RR4-L1/2);
  r1 (EA-U4R-1/-2, RR3-L2 (a)); build1 point 7 (R16 is a brief facts row: "npt-client exposes no decoded record";
  point 7 routes U6 to "an npt-client read item (R16) or U4g's Interp"; no rule bars a pub item, but r1 said
  engine-architect rejoins when a pub item / dependency / seam change comes); facts-full (A1-A9, B).
- C1a build1 line 14: "A SHA-256 failure there means that unit re-pins, never a retry" (det-u4 at every merge touching
  the scenario path) vs INF-U4R-1 "a re-pin without a ruling is not allowed: a mismatch comes to me whole". Both stand:
  the unit re-pins, but on a ruling. Q3 asks for that ruling up front.
- C2 Fix (a) source VERIFIED: scenario.rs:1004-1008 mirror.view(c.slot, newest, &mut view) -> mirror.rs:227-244 lists
  every Soldier in the mirror's frame; Records::new (mirror.rs:110-138) built from the room's EntityRecords. Server
  state: barred. Q2 (a) right. npt-client: Frame{entities, own: Vec<Held>} private (client lib.rs:95-100), Tick.filed
  Hash/Sentinel/Pending, complete() :160-162; pub items new..step (:253-415), none returns records. R16 is a facts row,
  not a rule; build1 point 7 named "an npt-client read item (R16)" for U6. It may come now (BRAWL-1 Fix (a) needs it).
  But it is a new pub item (and next_command another): r1 said engine-architect rejoins on a pub item / seam change
  -> add engine-architect to the round.
- C3 geometry VERIFIED (greybox.rs:61-163): footprint [c±400]; walls OUTSIDE it (THICK 30); outer wall nearest corner
  of bldg 1/2/4/5 (670,1475) = 16.2 m (soldier contact ~15.8 m); footprint corner (700,1505) = 16.6 m (plan right);
  bldg 0/3 footprint face 18.0 m. Entry only through the door gap: centre must be at >= 18.0 m (bldg 0/3, x=1800,
  |y|<60) or >= 18.3 m (others, (1040,1505)). So true margin from R_OUT 14 m to any entry is >= 4 m (plan's 2.6 m is
  conservative). Spawn grid: home ±300 cm (world.rs:703-722): team 1/2 corner (-800,±1166) = 14.14 m > R_OUT 14 m
  (slot 20 starts there: per_team index 4). Harmless (steers in) but "homes inside" != spawns inside.
- C4 PIN FOUND: wall stand-in [-300,1180]..[300,1220] (greybox :156); contact band to 12.6 m (1220+40) > R_IN 12 m.
  A soldier steering in from > 14 m within ~±15 deg of north hits the wall's north face at y 1260 and stays in steer
  mode (r > R_IN). The origin steer's x part = -127x/r points to x = 0: it funnels to |x| ~35 cm, where |m| <= 3 snaps
  away (A4); the plan's sideways part (only when straight along an axis) pushes out, the steer pulls back:
  oscillation at the face = a pin the "not moved in 300 steps" print cannot see (it moves 1 cm). Crates (<= 9.9 m)
  lie inside R_IN: never met in steer mode. Fix: R_IN above 1,260 cm (no fixed box in the steer band; then R_OUT
  above the spawn grid's 1,414 cm, e.g. R_IN 1,300 / R_OUT 1,450, entry margin >= 3.5 m), or a sideways rule that
  frees a soldier pinned at (x,1260) for every x, shown; stuck print by a box (100 cm for 300 commands), not "moved".
- C5 constants: R_OUT/R_IN in cm vs cm positions: not rate-derived (A8; carry 39). Leash vector: move_velocity scales
  > 127 by f32 sqrt (world.rs:494-498); MOVES stay <= 127^2; leash vector should too (unit-test it). Low.
- C6 profiles (l1 lib.rs:176-201): P3 150 ms ±30, 5 %; lead_for P0 2, P2 5, P3 7 (scenario.rs:1829-1836). Steer
  delay ~ lead ticks: P3 7 x 20 cm = 1.4 m < 4 m. B1: 1 s total-loss bursts (U4g's) -> ~30+ re-runs x 20 cm = 6+ m:
  a pure leash at 14 m may NOT hold B1 (entry >= 18 m). Not this lane's sets; sim-only test should print a B1-like
  burst pattern for U4g's record. Sim-only test feeds zero-age positions: add a delayed feed (lead cap 8).
- C7 Fix (b) VERIFIED: logs.rs:470-578 iterates Soldier records from room.sim(); area pub; room_tick only on logged
  runs (scenario.rs:971). contract.rs:66-110: rows() and written() -> every written name (tick.v/s, event fields)
  needs a row; a Counts total written to a lab end event is a second name. Writing only non-empty ticks: fine.
- C8 Q4: #[ignore] precedent (net l2 socket_tests.rs:512 with reason); cargoCounts reads passed/failed only; check.mjs
  "SKIPPED" is a custom print, so an ignored test is no skip. Dev profile: npt-sim unoptimised (Cargo.toml only opts
  rand_chacha/ppv-lite86): full test in dev on two builds would be costly. Q4 (a) right; measure the short copy.
- C9 EA-U4R-1/-2 proof GAP: det-u4.mjs and `npt-lab hash --manifest precheck-recordings.json` regenerate via
  scenario::run --record only (hash.rs:413-480; det-u4.mjs:1-37). run_leg (precheck.rs:285-480, SimClock change,
  the constructor) is exercised only by precheck tests r6/late_command/wallclock (precheck.rs:901-1050), whose
  outputs are not byte-compared. Clean pre-check pair at the old pin vs d666aeb's runs.txt section (192.8 s + record
  127.5 s, ~17 MB) is a whole proof. Condition.
- C10 N2-1 Q5 VERIFIED: closed branch scenario.rs:496-503: room.leave :499, owner.remove :500, ended.push :502.
  Leave plant -> room check + roster check fail; owner and ended pass: two of four. My r2 "all four" wrong (mine).
  Second plant roster.leave (room.rs:691): clients[i] still None -> only the roster check fails. Q5 (a).
- C11 TE-R2-2 case 3: num() parses u64 (scenario.rs:1327-1332); "3e17" -> "is not a whole number". Must be full
  digits (300000000000000000 x2, 100000000000000000: sum 1.8e19 fits, +3e18 overflows). Order 64-bit first: right.
- C12 RR3-L2 (a): filed (logs.rs:325-363) on no-log: bitten() needs phases (room_tick only) so false; resolved()
  only is right; `last` kept for finish. OK.
- C13 disk: shared target measured full 8.2G (u4r-disk-plan.md:12: debug 7.7G); 13,225 - 8,397 = ~4.8 GiB, but
  plan's own "about 9 GiB" gives 3.9 GiB (< floor). Plan's "4.2" is my sets-r2 arithmetic mixing units. Rule 6:
  4 GB before wasm32/release builds, 2 GB others. Thin margin either way.
- C15 crates unchanged since faf42aeb (git diff --stat faf42aeb034c HEAD -- spikes/netcode-proof/crates: empty), so
  results/u4r-precheck-30/faf42aeb034c/runs.txt "## pair clean" (wall 192.8 s; record 127.5 s) is a valid "before".
- C16 manifests read: det SS-P2-s1 5b79abb2 16,937,652; ES-P3-s2 fad8f67d 16,929,501 (ES scene, first_tick 1,000,000);
  precheck SS-P0-s4 6933b56a 16,950,444. Re-pin moves only bytes+sha256.
- C17 npt-sim pub: ticks_for_ms, ticks_for_ms_at (lib.rs:34-38) -> test lengths via npt-sim (carry 39). CMD_CATCHUP
  is npt-room's; --lead cap 8 (scenario.rs:1291). npt-bots deps include npt-sim (normal): an in-crate #[cfg(test)]
  module can host both tests, so npt-bots needs no pub item (smaller surface than pub next_command; EA-U4R-5 is U6's).
- C18 wall health 500, damage 1/hit, down 10 s (world.rs:111-113, :1602-1605): pins at the wall likely self-heal when
  shot down; still Medium-low, constants fix is free.
- C19 self-note: one stray no-op shell call used a heredoc into /dev/null (no file written, nothing changed). Recorded
  for honesty; my limits say Write/Edit only for files, and none was written that way.
- C14 compression measured (python zlib-6 in memory, P3 s2 181.4 MB -> 51.2 MB): 3.55x (lab 4.17, room 3.04,
  client 2.15). Draft's "5-10x, about 1 GiB" is wrong: about 2.1 GiB of archives for 7.4 GiB.

## Verdict (written before the StructuredOutput)
APPROVE WITH CONDITIONS 1-18 (no Critical or High). Findings: BP-1 Medium (run_leg proof gap), BP-2 Medium (wall
pin + stuck print), BP-3 Medium (sim-only test: zero-age feed, no B1 record), BP-4 Medium (merge step lacks det-u4,
build ruling 1), BP-5 Low (engine-architect missing; pub next_command), BP-6..BP-10 Low (case-3 digits; -57 wording;
disk arithmetic; draft ratio; names/rows). Mine: ADV-BP-1 (N2-1 "all four"), ADV-BP-2 (sets-r2 "4.2 GiB").
Q1 (a) with bounds R_IN >= 1,300, R_OUT in [R_IN+100, 1,500]; Q2 (a); Q3 (a) with conditions; Q4 (a); Q5 (a);
Q6 (a) conformance only; Q7 (a) now, recommend (c) + the old 4 GB cache; Q8 (a).
Size: about 700 planned (450-750 with conditions); well past > 1,400. To Mustafa: KEPT-LOGS only.
