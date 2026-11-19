# U4f review panel, round 1: seat `arch` (engine-architect), 2026-11-19

Tree: builder 3's worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4,
U4f uncommitted). Paths below are relative to spikes/netcode-proof/crates/ in that tree unless they say otherwise.
Memory: none (no agent-memory folder for engine-architect). Read only; no edit anywhere but this file.

## Plan
1. Read the seat's rules: u4f-brief.md 2.13 (public items by crate) and 4 (build rules on pub items/deps); plan.md
   §7.2 :930-931 (A19, the seam rule) and §5 :512; earlier arch reviews (u4e-review-arch.md, u4r-review-arch.md) for
   the rule's past reading.
2. Read Mustafa's SIZE-U4F words (history.md:6896-6903) and advisor-u4f-stop.md :80-89 (what is U4f2), :198-210.
3. In the worktree: `git status --short`, `git diff HEAD --stat`, the Cargo.toml diffs (crate edges), then every
   added/widened `pub` line in the diff (grep `^\+.*pub `), each against 2.13's list.
4. For each new pub item: who uses it across a crate edge (grep the other crates), whether it is needed by a §6 / plan
   item, whether it leaks room state (ownership), whether a test-only item is gated.
5. Dependency direction: npt-proto <- npt-room <- npt-server; npt-client; npt-bots; npt-lab at the top. Check no new
   edge points down-up, no new dep in Cargo.toml/Cargo.lock.
6. Check the client clock / lead controller seam (npt-client) against the Wasm boundary (no per-entity calls).
7. Check the U4f2 scope line: nothing built that lets -21 be judged before U4f2 (lab suites, scripts, SERIES lines).
8. Runs only to prove a finding (u4f-run.sh), few and short.

## Rules read (what binds this seat)
- plan (N/u4-build-plan.md) §7.2 :930-931 (A19): each report lists the pub items it adds, by crate, against §4.1;
  engine-architect checks them; check.mjs does not check narrow surfaces, so review is the check. §5 :512 the same.
- u4f-brief.md 2.13 (:782-798): no seam type changes; npt-sim "none, or two ids in World::BITES (bites builds only)";
  "any other sim change is a seam change for the advisor"; the table of proposed items per crate; rev 3.2: "the
  endpoint keeps its JoinAccepted tick, no new public item". Section 4: no Cargo.toml or Cargo.lock change.
- Precedent: advisor-u4e-r1.md:158-167 (R2, A5): items with no consumer in any test, plan line or ruling become
  pub(crate); "A later unit that needs one of the eight makes it public with its reason"; the merge commit brings §4.1's
  rows up to date (T6). u4e-review-arch.md:60-66 counted integration tests as consumers.
- Scope (advisor-u4f-stop.md:70-89; history.md:6896-6903, his SIZE-U4F (a)): the five forgers (kinds 1, 2, 4, 5, 6)
  are built ahead for U4f2's suite, "which is their consumer"; the twins and the twin's skip in the lab are U4f2's.

## What holds (checked, no finding)
- Manifests: `git status --short` lists no Cargo.toml and no Cargo.lock; no new dependency, no new crate.
- Edges: every `npt_*` path the diff adds is on an existing edge: bots -> proto (bots/Cargo.toml already has
  npt-proto), client -> proto, sim; room -> proto, sim; lab -> room, sim, server, log; server tests -> client (a dev
  dependency, as at U4e). No upward edge. The moved bite-homes test (lab/tests/bite_homes.rs) sits in the top crate,
  under `#![cfg(feature = "bites")]`, which is the right home (npt-lab's bites feature turns on every crate's).
- The seam (plan §4.2): `Command`, `Rewind`, `TickInput` and `SimCore`'s methods unchanged (sim diff is world.rs only;
  no seam.rs change). Room<S> still calls only SimCore's methods.
- npt-proto: `Pong::server_ns` is 2.13's one item; codec both ways (msg.rs:352, :444); catalogue row updated (24 B
  inside 32 B); npt-server sets it from the injected `now` (pump.rs:467) with no signature change (U4F-Q6 (b)).
- npt-room: lead bound in ms turned to ticks by `ticks_for_ms_at(tick_hz, ..)` (room.rs:580, :603), the rate an
  argument (carry 39), f31 pins 30 and 60 Hz (room/tests/commands.rs:804-815). `kept_stamps` and `run_presses` are
  fixed arrays of the room's slots (room.rs:503-509), never keyed by client data. `refusals` bounded at 4,096 a tick
  with a dropped count (room.rs:35, :931-937). Option A and B behind one `CmdTiming` start-up setting.
- npt-client: the clock reads no host clock (every time passed in); its two buffers capped (OUTSTANDING 8,
  SAMPLES_HELD 64: clock.rs:16-18); no call per entity per frame added to the endpoint's surface (the lab reads
  per-client counters once per step or at the run's end).
- The builders' public-item lists exist and flag most beyond-2.13 items (u4f-build.md:283-295, :569-581, :88-96).

## Findings (one line each, as found)
- EA-U4F-1 (Low): pub items with no consumer in any crate, test, plan line or ruling (A5's rule): ClientEndpoint::
  `not_due` (client/src/lib.rs:516), `join_tick` (:490; also against 2.13's "no new public item"); ClockSync::
  `ping_every_ns` (clock.rs:90: used nowhere), `next_tick_at` (:215), `one_way_hi_ns` (:225), `period_ns` (:85; own
  unit test only); npt-room `InputBuffer::KEPT_WINDOW` (input.rs:182), `Room::refusals_dropped` (room.rs:637);
  npt-bots `Behaviour::fast_left_out` (bots/src/lib.rs:340). Built-ahead for U4f2 (consumer U4f2's lab):
  `ClientEndpoint::skip_command` (:567), `twin_left_out` (:526), `Behaviour::forger` (:307), `take_forged` (:319).
- EA-U4F-2 (Low): the report's public-item list is incomplete: builder 3's four npt-bots items (`fast_clock`,
  `fast_left_out`, `forger`, `take_forged`) are in no builder list (u4f-build.md:692-866 has none); builder 1's list
  still names `Behaviour::brawl_presses` (u4f-build.md:293), which builder 2 replaced (:571-572); ClockSync's nine
  pub methods and ClockSample's five pub fields are listed only as "ClockSync, ClockSample"; §4.1's rows not yet
  updated (T6's step at the merge).
- EA-U4F-3 (Low): npt-sim's two check bites' sites leave unfeatured lines in the authority shot phase
  (sim/src/world.rs:1429-1438, :1452-1457: `let mut ready`, `let mut refuse_far = false`, the refuse block, `ready &&
  !refuse_far`, `&& !refuse_far`), against 2.13's "their sites, bites builds only". Behaviour-neutral outside bites
  (refuse_far is a const false; the wasm step's 27,000 ticks match both ways in both end checks), so no seam change;
  but the production World::step source now carries a bite's control flow. `World::CHECK_BITES` (world.rs:951, bites
  only) is beyond 2.13's "two ids in World::BITES"; I accept the builder's reason (World::BITES is positional for
  npt-hash-wasm and pinned at 4), as a report line for the advisor.
- EA-U4F-4 (Low): hidden coupling of the map extent: kind 6 reads it from the greybox (room.rs:548:
  `areas.rects.first()`), npt-sim keeps it private (`HALF_MAP`, greybox.rs:62), while npt-bots hard-codes it
  (FAR_CM 11,000 "inside the map's ±12,800", bots/src/lib.rs:76-80; kind 6's 20,000, :185) and its tests assert
  against the literal 12_800 (bots/src/lib.rs:934, :1104), not against the room's extent. NA-8 was this very drift.
- EA-U4F-5 (Low): the lab's `CONTROLLER_MAX_LEAD = 9` (lab/src/scenario.rs:47-49) is a tick count whose two
  reasons (below the room's 12-tick bound less a margin; "at most 9 so option B's buffer holds every command") are
  npt-room's public constants (`Room::lead_bound()`, `InputBuffer::CMD_CATCHUP`), tied only by a comment: no const
  assert or test pins it to them; and the header's `lead_bound_ms` is written from the constant
  `Room::<Sim>::LEAD_BOUND_MS` (scenario.rs:1899), not from the room it ran (`server.room.lead_bound().0`), while
  `Room::with_lead_bound_ms` is public. Also the const sits between two `use` lines (:47-49).
