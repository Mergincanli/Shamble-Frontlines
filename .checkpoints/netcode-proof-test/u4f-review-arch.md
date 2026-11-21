# U4f review panel, round 1: seat `arch` (engine-architect), 2026-11-19

Tree: builder 3's worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4,
U4f uncommitted). Paths below are relative to spikes/netcode-proof/crates/ in that tree unless they say otherwise.
Memory: none (no agent-memory folder for engine-architect). Read only; no edit anywhere but this file. No runner call
(every finding is proven by a read or a grep; `df -h /home/user` at the end: 5.0G free).

## Plan
1. Read the seat's rules: u4f-brief.md 2.13 (public items by crate) and 4 (build rules on pub items/deps); plan
   (N/u4-build-plan.md) §7.2 :930-931 (A19) and §5 :512; earlier arch reviews (u4e-review-arch.md) for the rule's
   past reading.
2. Read Mustafa's SIZE-U4F words (history.md:6896-6903) and advisor-u4f-stop.md :70-89 (what is U4f2), :198-210.
3. In the worktree: `git status --short`, `git diff HEAD --stat`, the manifests (crate edges), then every added or
   widened `pub` line in the diff, each against 2.13's list and the builders' lists.
4. For each new pub item: who uses it across a crate edge (grep the other crates), whether a test, plan line or
   ruling needs it, whether it leaks room state (ownership), whether a test-only item is gated.
5. Dependency direction: no new edge, no upward edge, no new dep in Cargo.toml/Cargo.lock.
6. The client clock and the controller against the planned Wasm boundary (plan.md §4.7's four exports).
7. The U4f2 scope line: nothing built that lets -21 be judged before U4f2 (lab suites, scripts, SERIES lines).
8. Runs only to prove a finding (u4f-run.sh), few and short.

## Rules read (what binds this seat)
- plan §7.2 :930-931 (A19): each report lists the pub items it adds, by crate, against §4.1; engine-architect checks
  them; check.mjs does not check narrow surfaces, so review is the check. §5 :512 the same.
- u4f-brief.md 2.13 (:782-798): no seam type changes; npt-sim "none, or two ids in World::BITES (bites builds only)";
  "any other sim change is a seam change for the advisor"; the table of proposed items per crate; rev 3.2: "the
  endpoint keeps its JoinAccepted tick, no new public item". Section 4: no Cargo.toml or Cargo.lock change.
- Precedent: advisor-u4e-r1.md:158-167 (R2, A5): items with no consumer in any test, plan line or ruling become
  pub(crate); "A later unit that needs one of the eight makes it public with its reason"; the merge commit brings §4.1's
  rows up to date (T6). u4e-review-arch.md:60-66 counted integration tests as consumers.
- Scope (advisor-u4f-stop.md:70-89; history.md:6896-6903, his SIZE-U4F (a)): the five forgers (kinds 1, 2, 4, 5, 6)
  are built ahead for U4f2's suite, "which is their consumer"; the twins and the twin's skip in the lab are U4f2's;
  c-u4f-r2.mjs's -21 rows run at U4f's merge are "an early read of the built suites and not -21's record" (:83-86).

## What holds (checked, no finding)
- Manifests: `git status --short` lists no Cargo.toml and no Cargo.lock; no new dependency, no new crate.
- Edges: every `npt_*` path the diff adds is on an existing edge: bots -> proto (bots/Cargo.toml already has
  npt-proto), client -> proto, sim; room -> proto, sim; lab -> room, sim, server, log; server tests -> client (a dev
  dependency, as at U4e). No upward edge. The moved bite-homes test (lab/tests/bite_homes.rs) sits in the top crate,
  under `#![cfg(feature = "bites")]`, the right home (npt-lab's bites feature turns on every crate's).
- The seam (plan §4.2): `Command`, `Rewind`, `TickInput` and `SimCore`'s methods unchanged (the sim diff is world.rs
  only; no seam.rs change). Room<S> still calls only SimCore's methods.
- npt-proto: `Pong::server_ns` is 2.13's one item; codec both ways (proto/src/msg.rs:352, :444); catalogue row
  updated (24 B inside 32 B); npt-server sets it from the injected `now` (server/src/pump.rs:467), no signature change
  (U4F-Q6 (b)).
- npt-room: the lead bound in ms turned into ticks by `ticks_for_ms_at(tick_hz, ..)` (room/src/room.rs:580, :603),
  the rate an argument (carry 39); f31 pins 30 and 60 Hz (room/tests/commands.rs:804-815). `kept_stamps` and
  `run_presses` are fixed arrays of the room's slots (room.rs:503-509), never keyed by client data. Refusal records
  bounded at 4,096 between two ticks with a dropped count (room.rs:35, :931-937). The per-tick scratch lists are
  reused (`drained` taken and put back, room.rs:1066-1108; `incoming`/`refusals` swapped, :1032-1033). Room bites are
  per instance (`Relax` built per receive from `bitten()`), the room's established pattern (EventQueue's `unbounded`).
- npt-client: the clock reads no host clock (every time passed in); its two buffers capped (OUTSTANDING 8,
  SAMPLES_HELD 64: client/src/clock.rs:16-18); modules private, `ClockSync` and `ClockSample` re-exported at the crate
  root (client/src/lib.rs:23-25), W1-PUB's pattern. plan.md §4.7 (N/plan.md:610) lists `ClockSync`; the planned Wasm
  boundary is four exports (npt-client-wasm: create, mode, input, step: N/plan.md:611), so the new per-client accessors
  stay lab-side: no call per entity per frame across a boundary.
- `ClientEndpoint::with_quant` (client/src/lib.rs:375) is unfeatured, but `Quant`'s fields are private and its only
  constructors are `Default` and the bites-only `with_bite` (proto/src/quant.rs:57-58, :102), so outside bites builds
  it can only pass the wire's default: the identity. Its consumer is server/tests/loopback.rs:252-259 (P33), and
  Cargo cannot name a dev-dependency's feature, so I accept it as named by the builder (u4f-build.md:93-96).
- `World::CHECK_BITES` (sim/src/world.rs:951, bites only): beyond 2.13's "two ids in World::BITES", but I accept the
  builder's reason (World::BITES is positional for npt-hash-wasm and pinned at 4: u4f-build.md:128-130); the report
  names it for the advisor (EA-U4F-2).
- SERIES.md: `git diff HEAD -U0` shows one hunk, `@@ -371,0 +372,20 @@`: 20 lines added, none removed; marks.json,
  marks/README.md and crates/judge are not in `git status --short`.
- U4f2's line: c-u4f-runs.mjs's SETS hold 47, 22 and 20 only (ops/runs/c-u4f-runs.mjs:44-48): no -21 set, so -21
  cannot be judged as a set before U4f2. The forged rows (p1, p7, p14, p17) are not in c-u4f-r2.mjs's RUNS.
- The known findings S1-S7 and N1 (advisor-u4f-stop.md:198-210): I disagree with none.

## Findings (one line each, as found; all Low)
- EA-U4F-1 (Low) pub items with no consumer in any crate, test, plan line or ruling (A5's rule).
- EA-U4F-2 (Low) the report's public-item list is incomplete and partly stale.
- EA-U4F-3 (Low) npt-sim's check-bite sites leave unfeatured control flow in the authority shot phase; the build
  record calls them bites-only.
- EA-U4F-4 (Low) the map extent has three homes: npt-sim's private HALF_MAP, the room's read of the greybox, and
  npt-bots' literals, whose tests assert against the literal.
- EA-U4F-5 (Low) the lab's CONTROLLER_MAX_LEAD is tied to npt-room's bound and buffer by a comment only; the header
  records the constant, not the room's bound; the client has no source for the bound (route for U7).
- EA-U4F-6 (Low) the lab copies npt-sim's private walk speed (MOVE_CM_S) into two -21 movement series; the header
  does not record it.
- EA-U4F-7 (Low) two public `Refusal`s across the seam with inverted meanings; the room's records are tuples, not
  plan §4.2's `{ slot, seq, kind }` form.
- EA-U4F-8 (Low) c-u4f-r2.mjs judges its -21 rows at U4f's merge commit with no "early read, not -21's record" label.
- EA-U4F-9 (Low) each cheat is split across two `Behaviour` calls a host must make in order (`step`, then
  `outgoing`): a host that skips `outgoing` runs every cheat honest, with no error (route for U6's npt-bots binaries).

## Findings in full

### EA-U4F-1, Low: public items with no consumer (A5's rule: pub(crate) until a unit needs them, or name the consumer)
- Where: npt-client `ClientEndpoint::join_tick` (client/src/lib.rs:490; also against 2.13's rev 3.2 "the endpoint
  keeps its JoinAccepted tick, no new public item"), `ClientEndpoint::not_due` (:516); `ClockSync::new`
  (client/src/clock.rs:63), `period_ns` (:85), `ping_every_ns` (:90: used nowhere, not even in the crate),
  `unknown_pongs` (:105: summed inside the endpoint only, lib.rs:481), `next_tick_at` (:215), `one_way_hi_ns` (:225);
  npt-room `InputBuffer::KEPT_WINDOW` (room/src/input.rs:179: used at :229 only), `Room::refusals_dropped`
  (room/src/room.rs:637); npt-bots `Behaviour::fast_left_out` (bots/src/lib.rs:340).
- What goes wrong: each widens the surface §4.1 records with no reader. Two are counts nobody logs, so their "0
  expected" claims are never shown: `fast_left_out` ("0 expected at P0", bots/src/lib.rs:339: the fast clock's copies
  dropped from an `Input` that would not encode, which changes what the room sees of the cheat) and
  `refusals_dropped` (records past 4,096 a tick, which U4f2's attribution will read). Built ahead with a consumer in
  U4f2 only: `ClientEndpoint::skip_command` (:565) and `twin_left_out` (:527) (the twin's skip: builder 3 records it
  "unused by the lab", u4f-build.md:858-859; no test in U4f calls it: grep), `Behaviour::forger` (bots/src/lib.rs:307)
  and `take_forged` (:319).
- Proof: grep each name over spikes/netcode-proof/crates (`.not_due(`, `.join_tick(`, `ping_every_ns`,
  `next_tick_at`, `one_way_hi_ns`, `period_ns`, `KEPT_WINDOW`, `refusals_dropped`, `fast_left_out`, `skip_command`,
  `twin_left_out`): no hit outside the defining crate's src.
- Fix: make the first list pub(crate) (`ping_every_ns` removed); either log `fast_left_out` in the fast-clock
  episode record (a new SERIES.md row) or make it pub(crate); the report names `skip_command`, `twin_left_out`,
  `forger` and `take_forged` as built ahead for U4f2, as the stop ruling names the forgers, and hands `skip_command`'s
  missing test to U4f2 (test seat).

### EA-U4F-2, Low: the report's public-item list (A19) is incomplete and partly stale
- Where: u4f-build.md:283-295 (builder 1), :569-581 (builder 2); builder 3's block (:692-866) has no list.
- What goes wrong: builder 3's four npt-bots items (`fast_clock`, `fast_left_out`, `forger`, `take_forged`) are in no
  list; builder 1's list still names `Behaviour::brawl_presses` (:293), which builder 2 replaced by
  `with_press_every` (:571-572); ClockSync's nine pub methods and ClockSample's five pub fields stand as "ClockSync,
  ClockSample"; `World::CHECK_BITES` and `with_quant` need their reasons in the report (both accepted above). §4.1's
  npt-room, npt-client, npt-bots, npt-sim and npt-proto rows are not yet brought up to date (U4e's T6 step at the
  merge, advisor-u4e-r1.md:165-167).
- Proof: the diff's added `pub` lines (git diff HEAD -U0, grep `^\+.*\bpub\b`) against the two lists.
- Fix: the report's list re-derived from the merged tree, each item with its reason or "built ahead for U4f2"; the
  merge commit updates §4.1's rows.

### EA-U4F-3, Low: npt-sim's two check-bite sites leave unfeatured control flow in the authority shot phase
- Where: sim/src/world.rs:1429-1438 (`let mut ready` and `let mut refuse_far = false`, each with an `unused_mut`
  allow) and :1452-1457 (the `refuse_far` refusal block, `let ready = ready && !refuse_far;`, `&& !refuse_far`), all
  outside `#[cfg(feature = "bites")]`; the record says "bites builds only" (u4f-build.md:42, :239).
- What goes wrong: 2.13 allows "their sites, bites builds only"; every earlier npt-sim bite site is under the gate
  (`bitten()` itself is bites-only, world.rs:988-989; StdSin :1008, Speed :1358, RefuseNotClamp's clamp exit
  :1319). The non-bites World::step source now carries a bite's control flow. Behaviour-neutral (refuse_far is a
  const false outside bites; both end checks' wasm step matched 27,000 ticks both ways, w3-end-check-a2.log and -b),
  so it is not a seam change, but the record line does not hold as worded.
- Proof: the diff of world.rs: those lines carry no cfg; grep `bitten(` lists every other site under the gate.
- Fix: keep the refusal decision and its effect inside the gate (for example a bites-only helper that returns
  `(ready, refused_far)`, with a `#[cfg(not(feature = "bites"))]` twin that returns `(ready, false)`), so the non-bites
  source is unchanged; or the report states the lines and the advisor accepts them.

### EA-U4F-4, Low: the map extent has three homes, and the tests pin the copy
- Where: npt-sim keeps it private (`HALF_MAP` 12,800, sim/src/greybox.rs:62); kind 6 reads it from the greybox
  (room/src/room.rs:544, `areas.rects.first()`); npt-bots hard-codes it: FAR_CM 11,000 "inside the map's ±12,800"
  (bots/src/lib.rs:76-80) and kind 6's forged 20,000 (:185); bots tests assert against the literal 12_800 (:934,
  :1104), and room/tests/commands.rs uses 12_801 and 20,000 (:258, :320).
- What goes wrong: if the greybox's extent changes, f18 still passes while the room refuses the clamp script's points
  as kind 6 (the clamp suite becomes an aim-bounds suite), and kind 6's forger may land inside the map. NA-8 was this
  drift (U4r's stand-in at 32,000 cm lay outside the map).
- Proof: read the four sites; f18 (bots/src/lib.rs:925-940) never reads the greybox.
- Fix (in lane): f18 and f15's kind-6 case assert against the extent the room reads
  (`World::new(..).greybox().areas().rects[0]`; the bots test module already names World), and FAR_CM's doc names
  that test.

### EA-U4F-5, Low: the controller's cap and the lead bound are tied by a comment only
- Where: lab/src/scenario.rs:47-49 (`CONTROLLER_MAX_LEAD = 9`, between two `use` lines); :1899 (the header's
  `lead_bound_ms` from the constant `Room::<Sim>::LEAD_BOUND_MS`); room/src/room.rs:601-605 (`with_lead_bound_ms`,
  public).
- What goes wrong: 9 must stay below the room's 12-tick bound less a margin and within option B's buffer (its doc);
  both are npt-room public items (`Room::lead_bound()`, `InputBuffer::CMD_CATCHUP`), but no const assert or test ties
  9 to them. The header records the constant, not the bound the room ran (`server.room.lead_bound().0`): equal today
  because no lab path calls `with_lead_bound_ms`. Route for U7: the client crate has no source for the bound (it does
  not depend on npt-room, and no message carries it), so a real client's cap is configuration only.
- Proof: read the three sites; grep `with_lead_bound_ms` (one test caller, room/tests/commands.rs:814).
- Fix: a const assert or a lab test pinning CONTROLLER_MAX_LEAD against `InputBuffer::CMD_CATCHUP` and
  `ticks_for_ms_at(TICK_HZ, LEAD_BOUND_MS)`; the header from the room's `lead_bound()` (`params` takes it, or the lab
  checks the two equal); a U7 note that the cap needs a source the client reaches (a `JoinAccepted` field or an
  npt-sim constant, the advisor's call). The const moves above the `use` block's end or below it.

### EA-U4F-6, Low: the lab copies npt-sim's private walk speed into two -21 movement series
- Where: lab/src/logs.rs:194-196 (`MOVE_CM_S = 600`, "a copy of its private `Scene::move_speed`"); used at
  logs.rs:701 (`tick.v.lagswitch_excess_move_um`) and lab/src/scenario.rs:1023 (`event.fast_clock.excess_move_um`);
  the source is pub(crate) (sim/src/scene.rs:139, :184, :244).
- What goes wrong: the two -21 movement checks (the lag switch's and the fast clock's) read an excess the lab computes
  from a copy no header records, while TUNE-1 made `Scene::weapon` and `Scene::health` public for exactly this
  reason (the facts each run's header records: §4.1's npt-sim row). A slower sim walk would let a cheat's extra
  movement pass unseen; a faster one would fail a correct build. Equal today (600 in both scenes).
- Proof: read the three sites; no header param names a walk speed (lab/src/scenario.rs:1880-1940).
- Fix: in lane, a lab test that walks one soldier at full length through a World and asserts MOVE_CM_S per second
  (catches drift); the clean fix, `Scene::move_speed()` public with a header param, is an npt-sim item and so the
  advisor's call (carry 26's route).

### EA-U4F-7, Low: two public `Refusal`s across the seam, with inverted meanings
- Where: npt-sim `Refusal` is the record `{ slot, seq, kind: RefusalKind }` (sim/src/seam.rs:498-505, plan §4.2:
  N/u4-build-plan.md:246-247's form); npt-room's new `Refusal` is the kind enum (room/src/input.rs:52), and its
  records are tuples `(u8, u16, Refusal)` (room/src/room.rs:632), whose doc cites ":246-247's form".
- What goes wrong: U4f2's attribution (2.4 item 6) reads both lists, kind 7 from the sim's and kinds 1-6, 8 and 10
  from the room's: one name means a record in one crate and a kind in the other, and the two records differ in shape,
  so the consumer aliases one import and maps one shape onto the other. No wrong result today (the room's records have
  no consumer outside room/tests/commands.rs).
- Proof: read the two definitions; grep `Refusal` in lab/src (only `npt_sim::RefusalKind` today).
- Fix (cheapest before U4f2 consumes it): name the room's enum for what it is (for example `CommandRefusal`) and give
  its record the sim's form (a struct `{ slot, seq, kind }`), so the two lists read alike.

### EA-U4F-8, Low: c-u4f-r2.mjs judges its -21 rows at U4f's merge commit with no "early read" label
- Where: ops/runs/c-u4f-r2.mjs:2-3, :22-23 (its header), :44 (`M21`), :55-66 (eleven -21 rows judged with `--only
  PM-0001-2,PM-0001-21`), :141-147 (r2.txt's head).
- What goes wrong: the stop ruling (advisor-u4f-stop.md:83-86) says these rows, run from U4f's merge, are "an early
  read of the built suites and not -21's record", and -21's 5.3 record is the whole run from U4f2's merge commit. The
  script's header and r2.txt's head say only "U4f's 5.3 pre-runs from commit X", and each -21 row's verdict file
  (`<key>-verdict.json`) reads like a 5.3 record; nothing in the output tells a later reader it is not.
- Proof: read the header and the head lines; grep "early read" in the two scripts: no hit.
- Fix: one line in the header comment and one in r2.txt's head: "-21's rows from a commit before U4f2's merge are an
  early read, not -21's 5.3 record (advisor-u4f-stop.md:83-86)". No row, run count, mark or judge rule changes.

### EA-U4F-9, Low (route for U6): each cheat is split across two `Behaviour` calls
- Where: bots/src/lib.rs:399-494 (`outgoing`, the never-acker's, the clamp twin's, the lag switch's, the fast clock's
  and the forger's changes) apart from `step` (:559-588); the lab calls both in order (lab/src/scenario.rs:1570-1578).
- What goes wrong: a host that calls `step` and the endpoint's `step` but not `outgoing` runs every cheat as an honest
  client, with no error and no count. The lab is the only host today (npt-bots' two binaries are 6-line stubs), so it
  holds; U6's npt-bots and npt-load will host the same `Behaviour`s.
- Proof: read the two methods; `outgoing` is the only place any cheat changes a datagram.
- Fix: route to U6: one entry point that runs the endpoint's `input`, `step` and the cheat's `outgoing` in order and
  returns the datagrams, or the module text names the order as the contract.

## Verdict
APPROVE WITH FINDINGS. No Critical, High or Medium in my seat. The crate edges, the seam and the dependency rules hold;
no manifest, marks.json, judge file or existing SERIES.md row changes. Nine Lows: EA-U4F-1, -2 and -8 are cheap
fix-pass or report items; -3 a small rewrite or a report line; -4, -5 (its first half), -6 and -7 small in-lane
pins or renames best done before U4f2 consumes them; -5's U7 note and -9 are routes.

| Id | Severity | Where | What | Fix |
|---|---|---|---|---|
| EA-U4F-1 | Low | client/src/lib.rs:490, :516; client/src/clock.rs:63, :85, :90, :105, :215, :225; room/src/input.rs:179; room/src/room.rs:637; bots/src/lib.rs:340 | pub items with no consumer (A5's rule); two unlogged "0 expected" counts | pub(crate) or log; name the U4f2-ahead items |
| EA-U4F-2 | Low | u4f-build.md:283-295, :569-581; builder 3's block | public-item list incomplete and stale | re-derive the list; §4.1 rows at merge |
| EA-U4F-3 | Low | sim/src/world.rs:1429-1438, :1452-1457 | unfeatured bite control flow in the shot phase; record says bites-only | gate it, or report it |
| EA-U4F-4 | Low | bots/src/lib.rs:76-80, :185, :934, :1104; room/src/room.rs:544 | map extent in three homes; tests pin the copy | tests read the greybox's extent |
| EA-U4F-5 | Low | lab/src/scenario.rs:47-49, :1899 | controller cap tied to bound and buffer by a comment; header from the constant | const assert; header from the room; U7 note |
| EA-U4F-6 | Low | lab/src/logs.rs:194-196, :701; lab/src/scenario.rs:1023 | copy of the sim's private walk speed in two -21 series | a pinning test; accessor by the advisor |
| EA-U4F-7 | Low | room/src/input.rs:52; room/src/room.rs:632; sim/src/seam.rs:498-505 | two public `Refusal`s, inverted meanings and shapes | rename; struct record |
| EA-U4F-8 | Low | ops/runs/c-u4f-r2.mjs:2-3, :44, :55-66, :141-147 | -21 rows judged with no early-read label | one label line each place |
| EA-U4F-9 | Low | bots/src/lib.rs:399-494, :559-588 | cheats split over two calls; a host that skips one runs honest | route to U6 |

## What I ran
- No runner call and no cargo. Reads and greps only: `git status --short`, `git diff HEAD --stat`, `git diff HEAD`
  (client, room, sim, proto, server, bots, lab, SERIES.md, the catalogue), `git rev-parse HEAD` in the worktree;
  greps of each new pub item over spikes/netcode-proof/crates; `df -h /home/user` (5.0G free).

## What I did not check
- The clock's and the controller's arithmetic against ADR 0001 and plan §4.8, the per-tick order, R2 (g)'s anchor,
  the cap and N1 (the netcode seat); the hostile-input kinds and the clamp points (security); test counts, bites and
  the SERIES.md read (test); panics, overflow, allocation and Wasm identity (rust). I did not run any test, bite or
  clippy, and did not rebuild to prove the pub(crate) changes compile. I read the lab's diff for its use of the crates'
  public items, not every line of scenario.rs's 1,301 added lines. Hand-off to the rust seat: the client's encode now
  clones the `Input` each step (client/src/lib.rs:696, :702), one extra allocation per step per client.
