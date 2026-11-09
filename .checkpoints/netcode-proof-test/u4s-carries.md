# U4s carries: the review findings the fix pass did not fix, by the brief that carries each

Written by the U4s fix-pass builder (workflow agent), 2026-11-09, from u4s-review-netcode.md, u4s-review-arch.md and
u4s-review-rust.md (this folder) and the fix pass's own changes. (Main session, 2026-11-09: U4s has since merged at
f080551; paths below are under spikes/netcode-proof/ and line numbers are the fix pass's, as merged. NF-D4 is ruled:
the advisor's U4s ruling, advisor-u4.md, "Rulings (U4s)", option (a) in its own words, the carries file's option
letters being swapped; the NF-D4 text at :32 and in "For the advisor" below is that ruling's record, no longer open.)

Each entry: the finding (review, id), then **the text the brief must carry**, word for word. A brief may reword the
plain parts, never the rule.

Completed by the netcode-architect, 2026-11-09, under the advisor's U4s ruling (advisor-u4.md:236-375): its condition 2
(:313-327) adds entries 34-39 and 41-44 and rewords carry 7; its open item O1 (:343-347) adds entry 40. Each is marked
with the ruling. The file now holds 44 entries; the wave-1 briefs carry them word for word (its condition 4).

## What the fix pass fixed (for the record; not carried)
- netcode F1 / arch N1: seam.rs:15-21, :26, :38-42 (the sim reads slot, seq, buttons, presses, move_xy, aim; tick and
  view time are the room's and the recording's; the sim uses the Rewind). Its test is carried to U4d (below).
- netcode F2: seam.rs:82-87 (the canonical order). Its debug assert is carried to U4d.
- netcode F4: seam.rs:166-175 (raw = before the own-copy snap; grid = the wire Config's to_grid, apart from the snap).
- netcode F5: seam.rs:58-67 (outside the ring names tick_a + 1 when fraction > 0; a refused Rewind behaves as None).
- netcode F6 / rust 3 (its first half): history.rs:19-33, `HitHistory::for_ms(span_ms) -> Option<HitHistory>`
  (ticks_for_ms; None for 0 ms); `new(capacity_ticks)` removed. The 1 MB half is carried to U4d.
- netcode F7 (VELOCITY's half): grid.rs:17-19, :147-150 (fixed for wave 1 as built; a change waits for U4d's merge).
  The givens are carried to U4a.
- netcode F8 / arch F6 / rust 1: check.mjs:527-630 (tickConstantProblems widened; why not "no literal 7" word for word
  at :568-570); ticks.rs const asserts :18-21, :81-98; s1 table tests/seam.rs:14-38, :84-120.
- rust 4: ticks.rs:12-21 (TICK_HZ in 1..=1,000, const assert), :58-67 (ack_ring_ticks_at's # Panics).
- arch F1 (its npt-sim half): grid.rs:21-26, :160-201 (one float-side defect, bites-only `Config::floor_rounding`, no
  ids); lib.rs:28-44 (-41's id at the own-copy snap side); client/src/lib.rs:8-22 and client/tests/bites.rs (-17's id
  at npt-client). -57's half is carried to U4a.
- arch F2: seam.rs:105-153 (`EntityKind::ALL`, `EntityKind::layout` and its private lists), :184-194 (every layout
  fits FIELDS); grid.rs:53-70 (`Field::integer`). The rest is carried to U4a and U4d.
- arch F4: check.mjs:67-72, :117-124 (SIM_DEPENDENCIES), check.test.mjs:158 (its self-test).
- arch F7: proto/Cargo.toml:23-28 (rand_chacha, rand_core as dev dependencies; Cargo.lock +2 lines).
- netcode F3 / arch F3: seam.rs:257-260 (non_finite documented as always 0); the question is below.

## U4a (the wire: npt-proto; wave 1)
1. **netcode F7 (the givens) and arch F2:** "Givens from npt-sim (U4s), fixed for wave 1, not U4a's to choose:
   `grid::VELOCITY` is 1 cm/s in 16 bits (-327.68 to 327.67 m/s); `EntityRecord::FIELDS` is 12; `Rewind::fraction` is
   8 bits (256ths). Each entity kind's field order is `EntityKind::layout()`: build the field-group layout table and
   the `EntityRecord` ↔ wire mapping from it, for every kind in `EntityKind::ALL`, never from a copy, and compute every
   size from it (each `Field`'s `min()`, `max()`, `bits()`, `wraps()`; two fields of one range but different kinds are
   unequal), since U4d completes the lists in wave 1 (integer fields come as `Field::integer`). A different step or
   width goes to the main session after U4d merges."
2. **arch F1 (-57's half):** "PM-0001-57's three bites live in npt-proto's `Quant`, as integer coarsening of the grid
   integer before packing, chosen per `Quant` value the caller passes (never a static: §4.11), each id in npt-proto's
   own `BITES` list and in no other crate: `PM-0001-57/pos-4cm` every 4th integer of `POSITION_XY` and `POSITION_Z`
   (errors up to 2 cm), `PM-0001-57/yaw-12bit` every 16th of `YAW` (0.088 degree steps, wrapping around the turn),
   `PM-0001-57/pitch-8bit` every 4th of `PITCH` (0.70 degree steps); to the nearest multiple in integers, inside the
   field's range; other kinds untouched. a4 tests each at the range ends, and shows its worst error is within half the
   coarse step and above half the step less one (the test U4s removed from npt-sim, the model). npt-sim's grid keeps
   only the float-side defect (`Config::floor_rounding`) and no ids."
3. **arch F5:** "A check.mjs deps-step grep, with a self-test bite in check.test.mjs: crates/proto/src and
   crates/client/src name nothing from `npt_link` but `Channel` (catalogue item 2: `npt-link` is a dependency of both
   for `Channel` only); any other `npt_link` path there fails, named by file and line."
4. **netcode F11 (nit 11):** "The decoder refuses the 10-bit pitch code -512 (`grid::PITCH` is -511..=511, so -512 is
   no pitch) as a decode error, never a panic and never a clamp, with a unit test."
5. **this pass (§5 U4a's a4 text):** "a4's -57 bites are set through `Quant`'s own bite choice, not through the grid
   config (u4-build-plan.md §5 U4a's 'set through the grid config' is superseded by the fix pass: see the main
   session's plan-text item below)."
34. **rust fix2 nits (u4s-fix2-review-rust.md:90-93; the advisor's U4s ruling, condition 2):** "U4a addresses each of
    four check.mjs nits, by a fix listed in its report (a deferral leaves this entry unmet, and entry 42 applies): nothing in check.test.mjs pins `STEPS.doc` or its
    `-D warnings` (as for fmt, clippy and test); the doc step documents the host build only (npt-tcpinfo's non-Linux
    stub is never documented); `lintBothBuilds` says 'both linted' when only the bites build fails; a measured-build
    doc failure's one-line summary is cargo's last line ('warning: build failed, waiting for other jobs to
    finish...'), with the detail in the lines below it. Their home is U4a's brief (wave 1's only check.mjs editor:
    u4-build-plan.md Lanes, wave 1) or, if U4a's report does not list this entry as met, U4d2's (entry 42); never the
    main session while U4a owns check.mjs."

## U4b (the loop policy: npt-server's schedule.rs; wave 1)
6. **arch F6 / rust 1 (the grep's stated limit):** "The tick grep cannot see a literal passed as an argument. npt-server's
   and npt-lab's callers build `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`; the 60 Hz tests pass
   `npt_sim::loop_catchup_at(60)`, never a literal 15; a test asserts the server's schedule's catch-up cap equals
   `npt_sim::LOOP_CATCHUP`."
35. **rust fix review nit (u4s-fix-review-rust.md:114-115; the advisor's U4s ruling, condition 2):** "A tick count
    needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it
    by path. Why: check.mjs:627 lets an initializer that merely names `TICK_HZ` through, and nothing pins such a value
    at 60 Hz." (Route, the advisor's F2 ruling, 2026-11-09: the plan names no new tick count for U4b; `Schedule::new`
    takes the rate and the catch-up cap as arguments (carry 6) and computes the period from the rate passed, tested at
    30 and 60 Hz. If U4b finds it needs one, it writes no local form and records the need (name, definition, values at
    30 and 60 Hz, the site) in its report; the main session adds it by carry 18's route in its own commit once U4d and
    U4a have merged, and U4b merges after it.)

## U4d (the stand-in sim: npt-sim; wave 1)
7. **netcode F1 (reworded by the advisor's U4s ruling, condition 2):** "A unit test: two commands that differ only in
   `tick`, `view_tick` and `view_fraction` (with equal Rewinds) step to equal state hashes on that tick and on every
   tick after; of `Command`'s fields, the state reads `slot`, `seq`, `buttons`, `presses`, `move_xy` and `aim`, and no
   state depends on any other `Command` field (seam.rs `Command`; carry 8's order assert reads `tick`, and that is
   correct)."
8. **netcode F2:** "`World::step` debug-asserts that `TickInput::commands` is in the canonical order (slot ascending;
   within a slot, command tick strictly ascending, wrap-aware: `b` after `a` when `b.wrapping_sub(a) as i32 > 0`),
   with a debug-build test that an out-of-order input panics. The synthetic recording generator writes the order, and
   the replayer feeds it unchanged."
9. **netcode F4:** "`World` fills `EntityRecord::raw` with each float field's value before the own-copy snap (f32
   bits) and `EntityRecord::grid` with `Config::default().to_grid(field, that value)`, computed apart from the snap's
   own `Config`. -41/server-quant-differs's unit proof: on a moving tick under that bite, the snapped copy differs from
   `from_grid` of the record's grid entry."
10. **netcode F5:** "D-9's outside-the-ring cases include `fraction > 0` with `tick_a` held and `tick_a + 1` not held.
    D-10's Rewind kind: the twin of a command with a refused Rewind is the same command with `None`, and the two give
    equal state hashes on that tick and after. `World`'s doc states what a command that holds or presses fire does with
    no Rewind: one rule for `None` and a refused Rewind."
11. **arch F1 (-41's half):** "`World::with_bite` plants `PM-0001-41/server-quant-differs` by taking its own-copy
    snap's grid from `npt_sim::own_copy_snap_with_bite` (U4s, lib.rs); the encoder's grid stays `Config::default()`;
    `World`'s bite list includes `npt_sim::OWN_COPY_SNAP_BITES`, and no other crate names the id."
12. **arch F2 (the lists):** "U4d completes `EntityKind::layout()` (health, weapon id and state, a projectile's type and
    fuse, the wall's and the effect rows' fields) with the grid's fields and `Field::integer`, within
    `EntityRecord::FIELDS` (seam.rs's const assert holds it), and fills records in that order. s6 pins the soldier's and
    the projectile's U4s lists: U4d updates those two assertions with its lists. `grid::VELOCITY` does not change in
    wave 1."
13. **netcode F9 / rust 2:** "`HitVolumes` gets a hand-written `Clone` whose `clone_from` copies field by field into the
    existing storage (rust 2's probe: the derived `clone_from` does not keep a `Vec`'s storage), and a counting-allocator
    test: a push into a full `HitHistory` allocates nothing."
14. **rust 3 (the 1 MB half):** "`HitHistory::for_ms` also returns `None` for a span whose ring would exceed 1 MB
    (capacity × one tick's `HitVolumes` bytes: u4-build-plan.md §4.9), tested at the edge, once `HitVolumes` holds its
    volumes. The recording header carries the ring's span in ms beside `TICK_HZ` (nit b's capacity), and the replayer
    rebuilds the ring with `HitHistory::for_ms(span_ms)`."
15. **rust 7:** "The sim widens `move_xy` (i8: -128 reaches it from a recording, though the room never sends it) and
    `aim` (i32) to a wider type before any arithmetic, so overflow checks never trip on any recorded command; `World`
    sets `HitVolumes::tick` from its own tick counter, not from `TickInput::tick` (`HitHistory::push` debug-asserts
    consecutive ticks)."
16. **rust 6:** "Doc the pub fields and variants U4s left without docs: `EntityKind::Soldier`, `::Vehicle`, `::Door`;
    `EntityRecord::kind`; `SimEvent::Shot`'s `slot`, `seq`; `Hit`'s `shooter`, `seq`, `victim`, `damage`; `Kill`'s
    `killer`, `victim`; `ClampReport::slot`, `::seq`."
17. **netcode F10:** "npt-sim names the button bits and press-counter indices (fire and the other pressable buttons) as
    constants beside `Command`, the one home the sim, the client and the bots read."
18. **this pass (the tick grep):** "A new tick-count constant in crates/sim/src joins check.mjs's `TICK_DERIVED` as
    `<name in lower case>_at(TICK_HZ)`, its function pinned at 30 and 60 Hz in s1's `DERIVED` table and tied by a const
    assert in ticks.rs; the deps step refuses any other form (no mechanical check ties s1's table to `TICK_DERIVED`:
    review keeps the two lists equal)."
19. **netcode side nit (§4.15's sizes):** "The recording's command is 39 B, not 41 B: `move` is 2 B (`[i8; 2]`), not 4.
    U4d states the measured size."
20. **arch N2:** "`grid::Config` has a bites-only field, so its shape differs by build: never hash, record or compare it
    across builds; pass it by value."
36. **rust fix review nit (u4s-fix-review-rust.md:112-113; the advisor's U4s ruling, condition 2):** "s6 ties
    `EntityKind::ALL` to the enum with an exhaustive match in place of the literal length assert
    (`assert_eq!(EntityKind::ALL.len(), 7)`, crates/sim/tests/seam.rs:440), so a new variant fails to compile."
37. **NF-D4 (advisor-u4.md:351-364; the advisor's U4s ruling, condition 2):** "`TickInput` stays integer-only, its
    spawn, teleport and attribute fields included, so every value that enters the sim from outside is finite by type
    (the new D-4, u4-build-plan.md §5 U4d); U4d removes `InputReports::non_finite`. If a float field ever joins
    `TickInput`, the counter and its refusal test come back (none is planned)."
38. **condition 1's rule (advisor-u4.md:83-84; the advisor's U4s ruling, condition 2):** "`StateDump` holds exactly
    what `state_hash` hashes, never `InputReports`, so a twin's field diff never shows a counter."

## U4e (the room, the server I/O step and the client)
21. **netcode F2 (the room's half):** "The room sorts each tick's accepted commands into the canonical order
    (seam.rs `TickInput::commands`) before `step`; -30's comparer relies on it."
22. **netcode F5 (the room's half):** "`rewind_at` never names `tick_a + 1` beyond the newest tick pushed into the ring
    when `fraction > 0`."
23. **netcode F12:** "The room stamps each `SimEvent` with the tick it ran (`TickInput::tick`) and its 32-bit event
    number when it queues it; the sim's events carry neither."
24. **arch F1 (-17's half):** "`ClientEndpoint` takes the grid for every call it makes into `npt_sim::grid` from
    `npt_client::grid_with_bite` (U4s) under `PM-0001-17/client-rounding`; `npt_client::BITES` grows with U4e's other
    ids (-17/tick16's client half, -58's, -45's, -62's); no id appears in two crates' lists."
25. **arch F8:** "npt-room adds `#![deny(clippy::float_arithmetic)]` at the top of crates/room/src/lib.rs beside
    check.mjs's float grep (the grep sees types and literals, not arithmetic on a value whose type it never names), or
    says why not."
26. **arch N3:** "`step`'s `&[SimEvent]` holds the `&mut` borrow of the sim: the room consumes or copies the events
    before it reads `hit_volumes`, `reports` or `entity_records`; if that costs a copy each tick, U4e raises a seam
    change with the advisor rather than working around it."
27. **this pass (CMD_CATCHUP):** "`InputBuffer::CMD_CATCHUP = npt_proto::Limits::REDUNDANCY_CAP` with its const assert
    (§4.3). The tick grep exempts the name `CMD_CATCHUP` alone (check.mjs `NOT_TICK_COUNTS`); every other `*_TICKS` or
    `*CATCHUP*` const or static under crates/ but crates/net/ is never set from a literal, tests included."
39. **rust fix review nit (u4s-fix-review-rust.md:114-115; the advisor's U4s ruling, condition 2):** "A tick count
    needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it
    by path. Why: check.mjs:627 lets an initializer that merely names `TICK_HZ` through, and nothing pins such a value
    at 60 Hz." (Route, the advisor's F2 ruling, 2026-11-09: the plan names no new tick count for U4e; its counts are
    `FULL_STATE_ACK_TICKS` and `ACK_RING_TICKS`, in npt-sim since U4s, and its deadlines and limits run in ns on the
    injected now. The Input bucket's refill (2 × TICK_HZ a second) and the window's per-tick budget (256 kbps) follow
    carry 6's pattern: the type takes the rate as an argument, production callers pass `npt_sim::TICK_HZ`, a test pins
    each at 30 and 60 Hz, and no const or static outside npt-sim is derived from TICK_HZ. If U4e finds it needs a new
    tick count or a named constant derived from the rate, it writes no local form and records the need in its report;
    the main session adds it by carry 18's route in its own commit once U4d2 has merged, and the site and U4e's merge
    follow it.)

## U4r (the L1 runner and its judged runs)
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1):** "U4r's brief, which
    comes to the advisor, states that at U4r the room has neither the view-time tolerance nor the cap (both land in
    U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30 pre-check's setup condition (0 view-time
    replacements and 0 clipped shots on both legs: the advisor's condition 3) holds by construction at U4r, and its
    RTT argument cannot be tested there; and it either re-runs the pre-check pair after U4h merges (6 runs) or says
    why not (the advisor's lean is the re-run)."

## U4h (lag compensation and aim)
28. **netcode F6 / rust 3:** "The ring's span is U4h's test cap plus its margin in ms, recorded; the room builds it with
    `HitHistory::for_ms(span_ms)` (no tick count is ever written for it), and the recording header carries the span."
45. **NF-D4's U4h line (the advisor's U4s ruling, "Rulings (U4s)", NF-D4; added by the main session, the conditions
    check's F3):** "`World::ray_cast`'s ray is the only float that reaches npt-sim code from outside; it takes `&self`
    and changes no state; U4h's AimCast states what a non-finite ray does."

## U4d2 or the main session (the workspace Cargo.toml, crates/proto/Cargo.toml and check.mjs; wave 2 or later)
29. **rust 5 (nit):** "The workspace Cargo.toml's libm comment says `arch` gives 'hardware sqrt, floor, ceil, trunc and
    rint'. Per the rust review (libm 0.2.16 configure.rs:82-101, arch/x86/fma.rs:9-27) it gives sqrt and sqrtf (x86-64
    `sqrtsd`) and fma and fmaf with run-time CPU dispatch on x86-64, and nothing on wasm32 stable; all correctly
    rounded. Reword it at the next edit of that file."
41. **proto's bites comment (the advisor's U4s ruling, condition 2):** "At the next edit of crates/proto/Cargo.toml
    (U4d2 or later), reword its bites comment (:13-14), which still says the grid's bites are chosen through its
    config: after carry 2, npt-proto's bites are its own."
42. **entry 34's second home (the advisor's U4s ruling, condition 2):** "If U4a's report does not list entry 34 as
    met, U4d2's brief carries entry 34's text word for word, U4d2 in place of U4a (U4d2 owns check.mjs in wave 2:
    u4-build-plan.md Lanes)."

## The main session (plan text, before the code merges: risk 11)
30. **A19 (§4.1's table):** "npt-sim's rows gain `EntityKind::ALL` and `EntityKind::layout` (arch F2: one home for the
    per-kind field order), `grid::Field::integer` (one type for a layout's fields), `full_state_ack_ticks_at` (each
    derived constant is `<name>_at(TICK_HZ)`: rust 1), `HitHistory::for_ms` in place of `HitHistory::new` (netcode F6,
    A5), and, bites builds only, `grid::Config::floor_rounding`, `OWN_COPY_SNAP_BITES` and `own_copy_snap_with_bite`
    (arch F1); npt-client's row gains, bites builds only, `BITES` and `grid_with_bite` (arch F1). Removed: bites-only
    `grid::Config::BITES` and `grid::Config::with_bite`."
31. **§5 U4a a4 and §4.14:** "-57's coarsening bites are set through `Quant`'s own bite choice in npt-proto (integer
    coarsening), not through the grid config; §4.14's -17/client-rounding and -41/server-quant-differs rows name their
    ids' homes (npt-client; npt-sim's own-copy snap side), the grid holding only the defect."
32. **§4.3:** "The FullState acknowledgement bound is `full_state_ack_ticks_at(TICK_HZ)`, which is `ticks_for_ms(2_000)`;
    `TICK_HZ` is asserted in 1 to 1,000."
33. **§4.15:** "39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers before spawns and
    the tick's own header; U4d states the measured size." (Item 19's plan half.)

## U4's report (when it folds u4s-report.md)
43. **the 1 MB cap (the advisor's U4s ruling, condition 2):** "The 1 MB cap on `HitHistory` is carry 14, at U4d, once
    `HitVolumes` holds its volumes, not U4h (u4s-report.md:58-59 says U4h)."
44. **the public items (the advisor's U4s ruling, condition 2):** "The public-items line (u4s-report.md:38-40) takes
    carry 30's list: it adds what the line leaves out, `OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`,
    `Config::floor_rounding`, `Field::integer`, the five field constants, and `EntityKind::ALL` and
    `EntityKind::layout` (advisor-u4.md:268-270)."

## For the advisor
**NF-D4: drop `InputReports::non_finite` and reword D-4?**
- The fact: `Command` holds integers only (net F116's field table: buttons u16, presses 4 × u8, move 2 × i8, aim in cm),
  so no command component can be non-finite, and `non_finite` (seam.rs:257-260) can never count. D-4 ("a non-finite
  command component is refused and counted; a debug assert each tick": u4-build-plan.md §5 U4d) then has nothing to
  feed it. u4-build-plan.md §4.2's refusal list (and the advisor's condition 1's list) names "non-finite components".
- Why it matters: a counter that cannot count makes D-4 vacuous, and a test that cannot fail is evidence of nothing.
- Options:
  - (a) keep the counter (always 0) and D-4 as written;
  - (b) drop the counter, take "non-finite components" off §4.2's list, and replace D-4 by the hazard integer
    components do carry (rust 7): a recorded command with `move_xy` -128 and `aim` at `i32::MIN` and `i32::MAX` steps
    without an overflow panic, and the room's range checks are the room's (U4e);
  - (c) keep the counter for a float the sim reads from outside, if U4d adds one (none is planned).
- My recommendation: (b), since §4.2's list was written before the command became integers and (b) tests the risk that
  remains. The fix pass kept the counter and documented it as always 0, as its brief said.
