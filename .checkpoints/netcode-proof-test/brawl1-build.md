# BRAWL-1's build notes (builder copy 1)

## Start (brief section 1)

- Brief: brawl1-brief.md, sha256 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50, 261 lines (checked).
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/brawl1, detached (no branch), HEAD
  352a245c3018c50843f62dcb90e91973ea56d9da (= `<base>`), `git status --short` empty, spikes/netcode-proof tree
  abad063555b0105e2afd47dcff96acd7bd74f63e (checked 2026-11-15).
- Runner u4r-run.sh sha256 6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8 (checked). Logs under
  /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.
- Disk at start: 13,174 MiB free.
- No earlier notes: this is the first copy.

## Plan

1. Read the binding docs (advisor plan review, plan, rulings, facts) and check the facts I rely on.
2. Capture the precheck tests' --nocapture "before" log (r6, late-command bite, wallclock) at <base>.
3. Part 1 items 1-6 (EA-U4R-2, EA-U4R-1 narrow, N2-1 + TE-R2-1 with two plants, TE-R2-2 + RR4-L2, RR4-L1, RR3-L2 (a)).
4. Part 1 proofs (a)-(e) at the old pins; record each here before Part 2.
5. Part 2: npt-client read item + test; npt-bots leash (crate-private) + unit test + sim-only test (full ignored,
   short copy); the figure in Logs::room_tick + Counts + summary line; SERIES.md rows drafted here.
6. Full test by name in release; short copy times.
7. Re-pin the three entries; check.mjs, det-u4.mjs, precheck regeneration at the new pins.
8. git diff --stat; report.

## Progress

- 02:3x: read the brief, advisor-brawl1-plan-full.md (conditions 1-18, Q1-Q8), brawl1-plan.md, the rulings
  (sets-r2 BRAWL-1; r2 N2-1/TE-R2-1/TE-R2-2/RR4-L1/RR4-L2/item 18; r1 EA-U4R-1/-2/RR3-L2), brawl1-facts-full.md. Facts
  re-checked at <base>: closed_checks scenario.rs:2150-2164, fp1 :2171, closed branch :493-503 (leave :499, owner.remove
  :500, ended.push :502), Room::roster room.rs:568, Room::leave :690-695 (roster.leave :691), Roster::is_live
  roster.rs:80; fp3 :2423; parse's 64-bit checks :1376-1381, B1 check :1442-1453; hash.rs caps :229-237, bytes cap
  :325-331, runner checks from :332, r10_fp3 :1594, `one` helper :1531; run_leg precheck.rs:285-503 (`now` :372, :475);
  run's build scenario.rs:742-766, loop :879-1106; lab_room :2019-2034; Logs logs.rs:141-221, filed :325-364,
  room_tick :470-589; client Tick::complete lib.rs:161-165, Filed :140-145, ring :218; Wrap::after32 proto wrap.rs:60;
  Room::new next_event 0 (room.rs:459), so with_event_start(0) is the identity.
- Before log, attempt 1: .gate/u4r/brawl1/p1-before-1.log ran `--lib` (the library's 5 tests): 0 of the three ran. Not
  a capture. The precheck tests live in the binary (`--bin npt-lab`).
- Before log A (condition 6 (ii), the "before"): .gate/u4r/brawl1/p1-before-a.log, at <base>, exit 0: measured build
  r6 1 passed (39.40 s); bites build r6 + late-command + wallclock 3 passed (103.04 s). Cold build first: 12,500 MiB
  free after it.
- Finding: the wallclock bite's comparer line prints the L1 leg's World hash, which the bite XORs with wall-clock ns
  (server pump.rs:228-246), so it can differ between two runs of the same code. Before log B (a second run at <base>)
  is captured to show which lines vary with no change at all.
- Before log B: .gate/u4r/brawl1/p1-before-b.log, at <base>, exit 0 (same counts). Normalised diff A against B (runner
  lines, Compiling/Finished/Running lines and libtest's "finished in" seconds left out; 67 lines each): exactly two
  lines differ, the wallclock bite's comparer line and its result line, and only in the L1 leg's hash (theirs
  9375924675163200265 vs 1388086141469307493; L1 hex 821df9d0f7ec1b09 vs 134378fdc3ebb265). Equal in both: index 3,
  loopback hash 6328158668484839173 (57d222c7ed590705), compared 120, mismatching 117. So proof (e) is judged as: every
  line equal to A, except those two L1 hash values, which vary at <base> itself. This reading goes into the report
  whole (item 7), for the advisor.
- Normaliser used for every (e) diff: `grep -v -E "^(lock |pwd |=== |--- |      [0-9]+ Compiling| *Compiling| *Finished|
  *Running| *Blocking)" | sed -E 's/finished in [0-9.]+s/finished in <t>/'`.
- Part 1 item 1, EA-U4R-2, done: scenario.rs `pub(crate) type Sim = World` (after the imports), `Server::build(scene,
  seed, first_tick, clients, event_start, arm_tick, &Bites)` (in `impl Server`, before `new`); Bites::world/room, Server
  and Server::new name Sim; run builds through it; run_leg passes `Bites::default()`, event start 0 (arm tick
  rec.first_tick, unused without a World bite) and keeps its decoder-bite lines; lab_room uses it. logs.rs names Sim
  (dump_events, room_tick, room_second, p6's `Sim::new`). precheck.rs names no World type now (imports Room, RttSource,
  due_ticks, Quant, World, CONTENT, credentials dropped); its printed "World::new(scene, ..)" strings unchanged.
  hash.rs's `npt_sim::World::BITES` / bite digest / dump sites left as they are (the hash tool's, not a room's: the
  alias's doc says so).
- Part 1 item 2, EA-U4R-1 narrow form, done: scenario.rs `struct Wake { admitted, ended, to_run }` and `fn wake(server,
  &mut dyn Listener, &mut Schedule, now) -> Wake` (steps 1-2 plus the due count), after `impl Server`. run: admitted ->
  carry 75 check + logs.admit, ended -> logs.gone, then its per-tick body. run_leg: SimClock (`clock.now()` at the loop
  top, `clock.advance_to(next.max(now + 1))`), admitted -> its map, ended -> o.ended. No one-loop driver.
- Compile check after items 1-2 (.gate/u4r/brawl1/p1-check-1.log): cargo check -p npt-lab --all-targets, both builds,
  exit 0, no warning.
- Part 1 item 3, N2-1 with TE-R2-1, done: closed_checks' fourth check `if server.room.roster().is_live(slot)` with the
  ruled message "the roster still holds slot {slot}: a rejoin would be a replacement"; its doc says four checks. fp1's
  doc rests "not a replacement" on the roster (Room::join reports a replacement only for a roster slot held live:
  roster.rs:116 `replaced = s.live`). The no-op `!a.ended.contains(&(0, "replaced"))` assert dropped.
- The plants (no plant in place now):
  - room.leave deleted (scenario.rs's closed branch; scenario.rs sha256 before 9f65b752..., restored by Edit, after
    9f65b75252d945d85b74117cf56eb5fcd87a98bccdb9d1093c1cfbf14482ae65, equal): .gate/u4r/brawl1/p1-plant-leave.log, exit
    101, fp1 FAILED, both cases ["the room still holds slot 0: its slot was not freed", "the roster still holds slot 0: a
    rejoin would be a replacement"]: two of four, by name (Q5 (a); ADV-BP-1).
  - roster.leave deleted (room.rs:691; room.rs sha256 8fbe015c659515d67995f60ea4d86a30306b41368de4d946287116c6d8472c33
    before, restored by Edit, equal after; room.rs not in git status): .gate/u4r/brawl1/p1-plant-roster.log, exit 101,
    both cases ["the roster still holds slot 0: a rejoin would be a replacement"]: the new check alone.
