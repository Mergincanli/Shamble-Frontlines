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
with the ruling. Later rulings add entries 45 on, each marked; the briefs carry them word for word (its condition 4).

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
    (Split by the advisor's W1-U4D ruling, C13: the storage-identity half is met at U4d; the counting-allocator half is
    entry 48, U5's.)
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
24. **arch F1 (-17's half; reworded by the advisor's W1-U4E ruling, U4E-Q4 and U4E-Q10):** "`ClientEndpoint` takes
    the grid for every call it makes into `npt_sim::grid` from `npt_client::grid_with_bite` (U4s) under
    `PM-0001-17/client-rounding`; `npt_client::BITES` grows with U4e's other client-site ids (-58/lt-tick-offset and
    -58/lt-event-seq, the client's rebuilds); -17/tick16 and -58/tick16 stay in `npt_proto::BITES`, the endpoint taking
    its `Quant` from `npt_proto::Quant::with_bite` (U4E-Q4); -45's ids and -62/cues-from-replay join npt-client in U4g,
    whose code holds their sites (U4E-Q10); no id appears in two crates' lists."
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
50. **the event age (advisor-u4-wave1.md:347-349; the advisor's W1-N2 ruling):** "An event older than 255 ticks cannot
    be coded: the event section's age is 8 bits (snapshot.rs's wire text). The room's EventQueue never queues one past
    that age: the overflow resync (§4.7) ends it first, or the room drops it from the resend and counts it. A unit
    test."
51. **SEC-U4d-R2-1, Low (u4d-round2-security.md; deferred to U4e by Mustafa, 2026-11-11: "The Low about the effect
    row's owner: defer it to U4e in writing. A Low never triggers a push."):** "An effect row's owner has no public
    home in npt-sim: the World finds it as the row's index / the scene's effect rows per soldier (world.rs:1879-1881),
    that count is pub(crate) (scene.rs:119), and the public doc only says 'its owner is its index / 8' (seam.rs:283).
    U4e's own-state filter never copies the 8. It reads the mapping from one public home (a pub accessor such as
    `Scene::effect_rows()`, or a pub rows-per-soldier constant with a const assert on every scene); npt-sim is not U4e's
    lane, and no such home exists today: the main session adds `Scene::effect_rows()` (pub, the rows per soldier, 8 or 0
    in today's scenes; approved as that home by the advisor's U4d close ruling) in its own commit when U4e's build
    starts, with its line in u4-build-plan.md §4.1's npt-sim row ("`Scene::effect_rows` (an effect row's owner, the one
    public home: carry 51)"). Any other home U4e finds it needs is a seam change for the advisor (carry 26's route),
    never a local copy of the 8. A test shows that slot k's client receives exactly slot k's effect rows."
53. **N2-L1 and the security-auditor's own-state carry (u4d-round2-netcode.md, N2-L1; u4d-round2-security.md,
    'Carries for U4e', first bullet; the advisor's U4d close ruling):** "The codec checks a record's kind against its
    section, never its slot against the client: any slot's Soldier, OwnState or Effect record passes in the own-state
    section. So the room's encoder fills client c's own-state section with c's slot's records only. In every
    `Snapshot` and `FullState` sent to c, the own-state section's Soldier and OwnState records have index = c's slot
    and that soldier's generation, and its Effect rows are exactly the rows whose owner is c's slot (carry 51's
    mapping). A test shows it for every client of an SS room (48 clients) on each of at least 300 ticks, the ticks
    around a respawn (a generation change) included. Catalogue item 5's L1 check reads each own-section record's
    identity (kind, index, generation, an Effect row's owner) against the receiving client's slot, not only the
    section the record sits in. Its owner-to-all bite takes the route the codec leaves open: other slots' own-state
    records in every client's own-state section. Moving them into the entity section ends in `Err(OutOfRange)` at the
    encoder; that proves the codec, not the class filter, so it does not count as that bite."
54. **The encoder's refusals in the room (u4d-round2-security.md, 'Carries for U4e', second bullet; the advisor's U4d
    close ruling):** "The room handles `Err(OutOfRange)` from the encoder (a section mix-up, or an integer outside its
    field's range) without a panic, `unwrap` or `expect`: that client's snapshot for that tick is skipped, the error
    is counted per client and logged (under a name with its SERIES.md row), and the next tick encodes afresh. A test
    plants one out-of-range record for one client and shows every other client's snapshot for that tick unchanged."
55. **Team and Public rows (u4d-round2-security.md, 'Carries for U4e', third bullet; the advisor's U4d close
    ruling):** "npt-proto's `carries()` (snapshot.rs:173-179) counts Team and Public rows as the entity section; both
    are empty today. A kind placed in a Team row needs the room's class filter (that team only), since the codec
    cannot tell teams apart. Any Team or Public row added before U4's end comes to the advisor with its filter and a
    test."
56. **N2-L3 (c), the own-state section's size (u4d-round2-netcode.md; probe X1, u4d-round2-test.md; the advisor's U4d
    close ruling):** "U4e's report states the measured own-state section per snapshot: bytes on the wire (mean, p99
    and max per client), in SS and in ES, with the OwnState record's share of it and how often that record changes.
    Over an S30 of SS, 47.49 of 48 OwnState records changed each tick (X1): the fire timer and the reload left count
    down every tick. R2's estimate allows 16 B per snapshot for the local player's own state (r2-netcode.md:1756). A
    mean above it is reported beside that figure, not judged: PM-0001-12 judges the downstream total."
57. **U4E-Q6, the room's half (advisor-u4-wave1.md:473-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "The event queue's cap is data, with a provisional value from net F121's arithmetic (1,024 is
    F121's example, above the about 400 a B1 burst adds), recorded as provisional wherever the run's header records
    the room's settings; U4f measures the rate and records §4.7's value before -22's runs. The room counts each
    overflow resync per client, with its tick, in its log, so that every judged -17 and -18 run can report them (carry
    58)."
61. **U4j's hash rules, the room and the client (the advisor's U4j ruling, "Carries", U4e, in FX-12's words; its
    round-2 ruling, B2.1: NC-U4J-R2-N1 and N6):** "U4e builds the room's and the client's side of marks/README.md
    §1's `hs` rules as U4j merged them (its `server_hash` and `decoded_hash` items, with the round-2 words), and
    exposes per client and tick what U4r's logs write (U4E-Q7): (a) the room's `server_hash` for each connected client
    from its admission on, in every phase the clients log; none for a client not connected or for a tick whose
    snapshot the room did not send that client; the room sends no snapshot of a tick it does not log, so its last
    tick record covers every snapshot it sent; (b) the client's hash filed under the tick it rebuilt, at the room's
    slot from `JoinAccepted`; two different hashes for one tick give the sentinel, one hash met twice (a duplicated
    datagram) is filed once, and no hash is ever dropped; (c) a tick resolved lost (`tick.s.snap_incomplete` 1, no
    hash) only once none of its groups can still decode: once it has left the client's frame ring (at
    `npt_sim::ACK_RING_TICKS` ticks old), or at the stop; a tick none of whose groups arrived counts, known from a
    later header's tick; (d) a group of a tick older than one the client has already decoded (L1's reorder step
    delivers such groups) is decoded and hashed under its own tick like any other, or, if the client drops it as
    stale, it is never acknowledged and that tick is lost, never the sentinel; the sentinel stays for §4.6's causes
    (no baseline, a bad header, a size or decode error). The advisor's lean is the decode: it keeps -58's P3
    comparing the late side of a reordered wrap (ADR 0001:1607). U4e's report says which its client does; a client
    that drops counts each drop per client under a name with its SERIES.md row. A unit test feeds `ClientEndpoint`
    tick t+1's snapshot before tick t's (as L1's reorder step delivers them) and shows t filed under t (its hash, or
    lost if the client drops it; never the sentinel), and that one datagram received twice files one hash."
68. **NF-L1, ES's bytes and the synthetic recording (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4e
    takes no ES snapshot byte or bandwidth figure from `Recording::synthetic`'s ES recording as a measure of ES's load.
    Since U4d-F that recording's alive soldiers change yaw on about 48 % of soldier-ticks and pitch on about 18 % (its
    sky aims sit nearly still while the soldier moves), against R2's moving-delta assumption of yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751), so its bytes read lighter than R2's estimate.
    Any ES byte figure U4e reports from that recording states those two shares beside R2's."

## U4r (the L1 runner and its judged runs)
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1; its last sentence the main
    session's words for the advisor's W1-U4E ruling, U4E-Q5, approved with the clause 'for the tolerance and the cap' by
    the advisor's U4d close ruling):** "U4r's brief, which comes to the advisor, states that at U4r the room has neither
    the view-time tolerance nor the cap (both land in U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30
    pre-check's setup condition (0 view-time replacements and 0 clipped shots on both legs: the advisor's condition 3)
    holds by construction at U4r for the tolerance and the cap, and its RTT argument cannot be tested there; and it
    either re-runs the pre-check pair after U4h merges (6 runs) or says why not (the advisor's lean is the re-run). The
    room's count of commands that `rewind_at` gives no Rewind, because the ring does not hold their claimed view time's
    tick, counts as a replacement in that setup condition: a pre-check leg with any is VOID (U4E-Q5)."
49. **N-M4 and TUNE-2 (advisor-u4-wave1.md:291-294; the advisor's W1-U4D ruling):** "The contract test's report gives,
    for each SS suite, the share of snapshots whose client set holds all 4 very large units, and the mean number of
    projectiles per client set (reported, not judged: W1-U4D N-M4). The runner refuses a scripted spawn or teleport
    above World's stated clamp bound (W1-U4D TUNE-2)."
58. **U4E-Q6's two conditions (advisor-u4-wave1.md:476-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "U4r's contract test logs each client's relevant gameplay-event rate and its event queue's
    high-water mark per burst. An overflow resync in any judged -17 or -18 run (U4r's, or a later unit's) is reported,
    and those runs come to the advisor before they count: net F121 warns that a cap set too low resyncs every client
    on every burst and changes -18's heal path."
59. **A2-L3 and T2-M1, ES under the bots (u4d-round2-arch.md, u4d-round2-test.md; the advisor's U4d close ruling):**
    "U4r's contract test also runs a short ES suite of the brawl `Behaviour` (48 clients) and resolves PM-0001-2's ES
    checks on it: shots, gameplay events, live projectiles and spawns in their bands (W1-U4D T-M1's numbers) and the
    share of snapshots whose client set holds all 47 other soldiers and all 15 vehicles. A band or share missed goes
    to the advisor before any ES run of PM-0001-12 counts. The ES runs themselves stay under PM-0001-2's counter
    script, run by run (ADR 0001:1480). Since U4d-F all 15 of ES's hulls loop inside the fight (the advisor's U4d close
    ruling, B1.3), where they are cover and block moves: on the synthetic recording, at its old reckoning and share,
    they cut ES's gameplay events from 9.853 to 7.250 a second over 300 s (U4d-F's step 2). The brawl meets the same
    hulls. U4r's report gives that figure beside its ES suite's event rate, and a missed event band is not read as a
    bot defect before the advisor has seen both."
60. **TUNE-1's header (advisor-u4-wave1.md:252-253; the advisor's W1-U4D ruling; homed by the advisor's U4d close
    ruling):** "Each run's header records the scene's weapon (interval, damage, reach, magazine, reload, spread) and
    its soldiers' health, read from `Scene::weapon()` and `Scene::health()`. U4r's runner writes them into npt-lab's
    L1 headers, and the contract test shows them in every header. U6's npt-server header does the same (plan.md's U6
    row)."
62. **U4j's log rules, U4r's runner and logs (the advisor's U4j ruling, "Carries", U4r, with FX-5 (a), FX-10, FX-12
    and R2.4; its round-2 ruling, B2.1: NC-U4J-R2-N1 and N6):** "U4r's logs meet marks/README.md §1's `hs` rules as
    U4j merged them, and the judge's coverage rule (SERIES.md's `decoded_hash_mismatch_count` row): (a) each name
    only in its own process's log: `server_hash` in the room's (process `room`), `decoded_hash` in the clients'
    (`client` on L1, `bots` on L2), `replay_hash` only in a `replay` log on x86_64; (b) every `hs` list and every
    `tick.s.snap_incomplete` list holds exactly the room header's `clients` entries, at the room's slot; (c) the
    writer buffers each client's ticks until every earlier tick is resolved, and writes a tick lost only once none of
    its groups can still decode (once it has left the client's frame ring, or at the stop: on L1 the runner may
    decide it there); a hash is never dropped, else the log ends `crash`; (d) the head: every client is admitted,
    and its first snapshot resolved, in the warm-up, before the room's first measured hash for its slot (a client
    admitted later is -19's join: C-U6-11); (e) the tail: after the measured phase the clients keep logging until
    every measured tick is resolved, a tick still in flight at the stop is not written, and the room's log covers
    every snapshot the room sent (it sends none of a tick it does not log); (f) a run and its twin log `h` over the
    same ticks; (g) the contract test shows `decoded_hash_mismatch_count` resolving with no gap on every -17 suite, a
    short L1 P3 suite with the reorder step on among them (no `crash` footer, 0 mismatches, no gap), and, if U4e's
    client drops stale groups (entry 61 (d)), reports its drop count beside `decoded_hash_compared_count` on -58's
    P3 suite; (h) NC-10 (the advisor's U4j ruling, R2.4): the contract test reports `tick.s.snap_incomplete`'s sum on
    each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0 part, with a bite (a client that writes
    null and incomplete 1 for a refused group), before -17's judged runs: `{"name": "setup: L1 P0 loses no group (no
    client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "measured", "op": "==",
    "value": 0, "parts": ["L1 P0"], "setup": true}`. If it is not 0, the cause goes to the advisor first. (Both the
    reported sum and the check read the measured phase: each join's ticks between its full state and that full state's
    acknowledgement carry no snapshot, so the client resolves them lost by design in the warm-up: the advisor's U4e
    round-1 ruling, R5, NC-L10.) marks/ is not U4r's lane unless its brief gives it this one check; else the main
    session adds it in its own commit."
63. **FU-3 and FU-7's log half, the next change to npt-log (the advisor's U4j ruling, FU-3: SEC-F10; its round-2
    ruling, B3: SEC-R2-N2):** "The next change to npt-log (U4r's, if its brief takes crates/log; else the unit that
    next edits it) adds two refusals, each with a test that fails before it: `read_log` refuses a JSON object with a
    duplicate key (today the last one wins, silently: FU-3); `check_header` refuses any header text field holding a
    character for which Rust's `char::is_control` holds (today it refuses only an empty `run`, `set`, `role` or
    `process`: log/src/lib.rs:305-321; FU-7). Until then both stay open security findings in PR #40's report."
69. **NF-L1's U4r half (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4r takes no ES byte or bandwidth
    figure from `Recording::synthetic`'s ES recording as ES's load: its judged ES runs are the bots' (carry 59). Any ES
    byte figure it reports from that recording states the recording's churn beside R2's moving-delta assumption: yaw
    changing on about 48 % of alive soldier-ticks and pitch on about 18 % (U4d-F's scene), against yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751)."
73. **NC-H1 and TE-1, tick16's judged half (u4e-review-netcode.md, Block h; u4e-review-test.md, TE-1 and probe Q-1;
    the advisor's U4e round-1 ruling, R1):** "Since U4e, under PM-0001-17/tick16 and PM-0001-58/tick16 the client
    files the sentinel for each post-wrap group under the tick its 16-bit header names (0, 1, 2 …): ticks before the
    client's first snapshot, which the room never logged for that slot (marks/README.md §1's bad-header sentence), so
    `decoded_hash_mismatch_count` counts each one, while the post-wrap ticks the room did log stay unresolved for that
    client. U4r's client writer places those hashes in its log (README §1: it files every hash under its rebuilt tick,
    buffering and sorting before it writes, so that records stay in tick order) and never ends the log `crash` for
    them. Its contract test runs one short tick16 bite run past the wrap (-58/tick16's `ticks_below_wrap` = 30 keeps it
    short) and shows the judge's outcome: the mismatch figure FAIL with at least one mismatch, never INVALID (the judge
    returns a mismatch count before it reads a coverage gap: judge/src/figures.rs:542-554)."
74. **NC-M6, the endpoint's lead (u4e-review-netcode.md, Block h; the advisor's U4e round-1 ruling, R3):** "U4e's
    `ClientEndpoint` stamps each command `newest + lead`, with a lead of 1 by default (crates/client/src/lib.rs), and
    U4f's lead controller comes after U4r's runs. On a link whose one-way delay is above about half a tick period every
    command would reach the room after its tick and be dropped as late, `last_run` would stay `None`, and option B
    would have nothing to repeat: -17 would compare a world of soldiers standing still. U4r sets each endpoint's lead
    with `ClientEndpoint::with_lead` from the run's profile (its largest one-way delay, half the round trip plus half
    the jitter, in ticks rounded up, plus 1), records it with `cmd_timing` in the header, and its contract test reports
    on each short run the commands that arrived late (the room's `late` count) beside the commands sent; a run with more
    than 1 % of its commands late comes to the advisor before its marks count."
75. **NC-M5 and A3, the client's event-number base (u4e-review-netcode.md, Block h; u4e-review-arch.md, A3; the test
    seat's probe Q-10; the advisor's U4e round-1 ruling, R3):** "U4e's client rebuilds each event's 16-bit number near
    its own start (`ClientEndpoint::with_starts`, 0 by default), and the wire carries no event base (`JoinAccepted` and
    `FullState` have none), so a client whose start is 32,768 or more from the room's counter rebuilds every event
    number wrong, and the hash reads them (Q-10). U4r's lab gives each endpoint, through `with_starts`, the event start
    it gave the room (`Room::with_event_start`) and its command start, and admits every client before the room has
    stamped 32,768 events past that start (about 30 s of SS's events); a later admission comes to the advisor first
    (the room exposes no event counter). The contract test shows `decoded_hash_mismatch_count` 0 on a run whose event
    start is far from 0 (-58/lt-event-seq's, 3 below the wrap, among them). The wire field is U6's (C-U6-13)."
76. **A6 and SEC-U4E-5, ending a session (u4e-review-arch.md, A6; u4e-review-security.md, SEC-U4E-5; the advisor's U4e
    round-1 ruling, R3):** "npt-room decides a replacement and a `Kick(overflow)` but sends the old connection nothing,
    npt-server's `Decoder` has no call that ends a session, and `Room::receive` takes any message by slot alone, so the
    caller's session handling is a security boundary. U4r's lab, when `Room::join` reports a replacement, sends the old
    session `Kick(replaced)` and drops its `Decoder` and link before it routes anything else; when the room kicks a
    client or takes its `Leave`, it drops that session. Two tests: a second endpoint joining with slot 0's token and
    bind while the first is live, after which the first's Inputs and `Leave` reach no room call and the second keeps
    the slot; and a kicked session whose later datagrams reach no room call."
77. **U4r's readings of U4e's room and client (u4e-review-netcode.md, NC-L2, NC-L4, NC-L9 and NC-L10; the advisor's U4e
    round-1 ruling, R4 and R5):** "(a) -58's wrap check counts `event.counter_wrap` records (marks.json -58), and the
    room and the client report wraps per client slot (`ClientSlot::wraps`, `StepOut::wraps`): U4r writes one record per
    counter name that wrapped (SERIES.md's row: 'per counter that wrapped'), never one per client, so one counter's 48
    per-client wraps cannot pass the check. (b) Under PM-0001-18/no-fallback, a burst whose last acknowledged baseline
    is exactly 32 ticks old at the first tick after it logs `event.burst_eviction` 1, yet the bitten room codes that
    tick from the defaults (its stale copy is taken only once B's ring slot is overwritten) and heals: U4r's report
    counts the bitten runs' bursts at that age apart. (c) The contract test reports each client's
    `ClientEndpoint::stale()` sum on each short L1 run (groups of ticks already resolved when they left the frame ring;
    no SERIES.md row); a sum above 0 on L1 comes to the advisor."
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5; with EA2-3, RR2-L3 and TE2-R2-2: the
    advisor's U4d2 round-2 ruling, Block 2):** "(a) For -30's pre-check, U4r gives `HashCompare::of` two lists that
    name the same recording on both legs and line up tick for tick (it compares by position, names included), and says
    where its printed comparison line comes from (`compare_text` is private to npt-lab's binary); TickHash's struct doc
    then says what its field doc says, 'the recording both legs replay' (today it still says 'the recording (or run) it
    replays'). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also requires
    exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines alone,
    which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that the bite
    causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg is the
    committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's first
    recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside the
    hash-map pair's equal prefix). (d) One npt-lab bin test runs `hash compare` with a bite and no `--on` over two
    recordings and asserts that only the first listed is armed (EA-9's one default, the legs'; today no test pins it:
    compare arming every recording again leaves every test and the step green, r2-te-rr2-l3-compare-arm.log)."
84. **INF2-1 with INF2-2 and TE2-R2-1, the wasm step's time bound and a stopped call's processes
    (u4d2-r2-review-infra.md, INF2-1 and INF2-2; u4d2-r2-review-test.md, TE2-R2-1; the advisor's U4d2 round-2 ruling,
    Block 1):** "At U4r's change to check.mjs's `wasm` step (entries 82 and 83), or in ops/runs/det-u4.mjs if U4r's
    Evidence decision moves the step's runs there: (a) the step's calls other than its cargo builds share one deadline,
    WASM_CALL_LIMIT_MS (300 s) from the first of them: each is given what is left, and once one is stopped the rest are
    not started, each named as skipped (today each call has its own 300 s, so a hang in npt-sim's replay makes 12 calls
    each wait out 300 s, 60 min under the exclusive lane lock: r2-inf-hang.log). (b) A call stopped by the deadline
    takes every process it started with it (today spawnSync's timeout signals only the direct child, so `npt-lab
    hash`'s two leg processes can keep running after their parent is stopped, unseen by the runners' idle wait, which
    looks for cargo and rustc only), by a documented means (its own process group, killed whole, or another) or by one
    whose effect (c)'s second test proves. (c) Two self-tests: the shared deadline, with a fake runner and clock (a
    first call that takes the whole limit; the next calls skipped and named); and run() itself on real processes (a
    child that starts a grandchild and sleeps, with a limit of about 200 ms: run() returns `timedOut`, and neither
    process is left), which also pins run()'s timeout wiring (today dropping it leaves 71 of 71 green:
    r2-te-inf2-2-run-timeout.log)."
85. **The deps step's unread inputs and two overclaims (u4d2-r2-review-security.md, SEC-U4D2-R2-1 to -3;
    u4d2-r2-review-rust.md, RR2-L1, RR2-L2 and RR2-L4 (a); u4d2-r2-review-test.md, TE2-R2-3; the advisor's U4d2 round-2
    ruling, Block 2):** "At U4r's change to check.mjs (entries 82 to 84), the deps step, each refusal naming its source
    and value, with one self-test case each, and the container's environment and the spike's own config still passing:
    (a) refuses a linker set for any target, by an environment variable whose name ends in _LINKER or by a `linker` key
    in any table of a cargo config read, as COMPILER_ENV and COMPILER_KEYS refuse a compiler (today
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER passes: r2-rr-superset.log run 3); and DEPS_CHECKED's limits and
    crates/hash-wasm/loader.mjs's header (the one edit U4r makes in crates/hash-wasm) say what the target_features read
    gives: a feature enabled at compile time shows in the section, which the loader refuses, but the section is the
    linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can rewrite it (built:
    sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check (today both say a flag from a
    route check.mjs does not read is refused too). (b) It refuses any `profile` key in a cargo config read and any
    environment variable whose name starts with CARGO_PROFILE_ (today a config's `[profile.release.package.npt-sim]
    overflow-checks = false` passes and drops npt-sim's `-C overflow-checks=on`, which plan §4.5 asks for in every
    profile: r2-sec-fx1-config.log, r2-sec-routes.log). (c) In npt-tcpinfo and npt-hash-wasm it refuses `include` other
    than directly before `!` and an opening bracket, and in every crate an attribute whose text starts with a
    metavariable (`#[$` or `#![$`; none today), so that neither `m!(include!)` nor a #[path] built from a `meta`
    fragment brings an unread file under a listed export's allow (today both pass, and deps' held line says every
    include! literal names a .rs file: r2-te-sec-r2-3-macro-include.log, r2-sec-pathmeta.log). (d) It reads an empty
    CARGO_HOME as unset and a relative one against the folder cargo runs in, as cargo does (today an empty one hides
    ~/.cargo/config.toml from FX-1's refusal and from the read-back: r2-rr-cargohome.log). (e) Its text beside
    buildInputProblems says which source cargo uses: CARGO_ENCODED_RUSTFLAGS if set, even empty; else RUSTFLAGS if set,
    even empty; else the matching target tables' flags, if any; else build.rustflags (today 'the first non-empty
    one')."

## The next change to crates/judge (whoever makes it; its first step)
64. **Round 2's judge follow-ups (the advisor's U4j round-2 ruling, B2.2, B3, B4.2, B4.3):** "Before any other edit
    to crates/judge/src, and at the latest before U6 builds -19's figure, the change adds these, each shown failing
    under its plant (planted by Edit in place, restored and sha256-checked) and passing restored: (a) TE2-1, Medium:
    in tests/engine.rs's Arm test
    (`u4j_the_arm_rule_counts_an_aarch64_mismatch_only_when_the_x86_64_re_decode_misses_too`), a case for
    `machine.role` C and one for ML: an aarch64 client log of process `client` with the test's `wrong` hashes, the
    room on x86_64 and the x86-64 replay agreeing with the server: FAIL with 1 mismatch, and 'aarch64 only' 0.
    Plant: figures.rs's `h.machine.role == "LG" &&` dropped. Then fixtures/README.md's l1-aarch64-claim row's 'this
    row does not show the Arm rule's LG key (TE2-1: carry 64 (a))' becomes 'the Arm rule's LG key is shown by the
    engine Arm test's role C and ML cases (TE2-1)'. (b) TE2-2 = NC-U4J-R2-N3 = SEC-R2-N1, Low:
    `u4j_ws_far_better_past_the_cap_is_invalid` asserts the pin's own reason for probe A as committed ('its bound
    line is not PM-0001-18's ws_bound_line, byte for byte'), and gains probe A-pinned (the record carries
    marks.json's own line; FAR BETTER at s = 1.9 with a chance ruling): INVALID by the class rule ('is "FAR BETTER"
    at s = 1.9'). Plants: the pin removed (the first case's reason changes); FAR BETTER's `s < bound.band_low`
    removed (the second's). (c) TE2-3, Low: `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run` gains a case
    (5): the bite's set holds two measured runs of one build with different practice runs in force (the later one
    started after a second practice run of the build, which has a `second_ruling`): INVALID ('have practice runs …
    in force'). Plant: that refusal in `find` removed. (d) TE2-4, Low: a case where a coverage gap and an LG Arm
    mismatch waiting for its re-decode meet in one run: INVALID, never NOT-RUN. Plant: the gap and waiting checks in
    `decoded_hash_mismatch_count` swapped. (e) NC-U4J-R2-N4 and N5, the code's words, Low: eval.rs's Arm-rule FAIL
    note reads 'an LG aarch64 mismatch (the room on x86_64) whose x86-64 re-decode agrees is a cross-architecture
    finding' in place of 'an aarch64-only mismatch is a cross-architecture finding', and practice.rs's module doc
    reads 'f = max(1, s), or 1 for FAR BETTER' in place of 'f = max(1, s)'; words only, no plant. (f) FU-7's judge
    half, Low, security (SEC-R2-N2): `summary_line` (judge/src/lib.rs) escapes every control character in what it
    prints, so no log text starts a line of its own on stdout; a test with a run id holding a newline and the word
    PASS shows one line printed. Plant: the escape removed."

## Any later change to npt-sim's synthetic recordings, or to the World's outcome on them (whoever makes it)
67. **NF-M1 and TF-L2, the ES re-measure RM (u4df-review-netcode.md, u4df-review-test.md; the advisor's U4d-F
    ruling, R1):** "Before the merge of any change that moves the estimate scene's synthetic recording (D-7's ES
    digest) or the World's outcome on it (t_m1's printed line differs from the base's), and at once whenever t_m1
    fails, the change's owner runs RM and its report gives the numbers. (1) Runs: seeds 1 to 9, each a 36-minute ES
    recording (64,800 ticks from tick 1,000,000), replayed and counted as tests/world.rs's `EsRun` counts (an
    `#[ignore]` test run by name, or a scratch crate on npt-sim's public API outside every copy). (2) The floor, run by
    run: each seed's window (minutes 2-11) meets PM-0001-2's four ES bands (shots 68.4-75.6 a second, gameplay events
    9.5-10.5, live projectiles 30.4-33.6 a tick, spawns 20.064-22.176 a second), with 48 soldiers and 15 vehicles on
    every tick and every hull within 1,400 cm of the cluster's centre. (3) Drift: the nine seeds' mean gameplay-event
    rate over each 300 s block from minute 2 to minute 31 (six blocks) lies in 9.5-10.5; the least-squares slope of the
    nine seeds' mean per-minute rate over minutes 2-35 is reported beside it. (4) Every seed's two 300 s halves of its
    window are reported, not judged: a single half's noise is 0.2-0.36 a second at ES's level. (5) `aim_one_in` is
    re-set only if (2) or (3) fails at the current value, then to the value whose nine-seed mean window lies nearest
    10.0 with (2) and (3) passing at it, never on three seeds' in-sample margins; the numbers come to the advisor
    before D-7 or t_m1 is re-pinned for it. (6) If t_m1 fails while (2) and (3) pass, neither the share nor t_m1
    changes: the numbers come to the advisor, who rules on t_m1's form. The band and its 9.5 floor do not move
    (Mustafa, 2026-11-11; carry 52 quotes him). At U4d-F's merge the scene passes RM: windows 9.693-10.171 on seeds
    1-9; blocks 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857 (u4df-review-netcode.md [10]; .gate/u4df/nc-long.log)."
71. **NF-L3, 3b's switch (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "The generator's re-home (3b) is
    switched on by a scene's `aim_one_in` above 1 (replay.rs's `synthetic`; scene.rs's `aim_one_in` doc says so), and
    only ES sets it above 1. A change that would let another scene set `aim_one_in` above 1, or set ES's to 1, first
    gives the re-home its own pub(crate) scene knob (its period, 0 for none), in that same change and with the
    advisor's approval of its symbols. Until then no knob is added."
72. **TF-L1, a_m1's non-vacuity (u4df-review-test.md; the advisor's U4d-F ruling, R3):** "a_m1
    (tests/world.rs:940) is not vacuous only because SS seed 5's 1,200 ticks hold about two deaths (58 dead
    soldier-ticks: the test-engineer's U4d-F probe), and two generator-only plants took them away. Before the merge of
    any change that moves SS's synthetic recording, a_m1 is made independent of the recording's draws: it scripts its
    deaths through its own inputs, or runs long enough for at least 10, and asserts that count by name."

## U4f (clock and commands)
46. **N-M2 = R-M2 = A-C, -21's press twins (advisor-u4-wave1.md:197-235; the advisor's W1-U4D ruling):** "-21's press
    twins: the twin removes exactly the presses the forged run's `reports()` list as refused (slot, seq, kind) and
    lowers that button's counter by one from the refused command on, every other field equal; a test shows a forged
    press stream and its twin give equal state hashes on every tick (D-10's rule, at the room's level)."
80. **U4e's notes for option A and the client (round 1: u4e-review-rust.md, RR-7; u4e-review-security.md, SEC-U4E-9;
    u4e-review-test.md, TE-9; u4e-review-netcode.md, NC-L7 and NC-L8; the advisor's U4e round-1 ruling, R5; round 2:
    u4e-review-r2-netcode.md, NC2-L1; u4e-review-r2-test.md, TE2-L1; u4e-review-r2-rust.md, RR2-2; the advisor's U4e
    round-2 ruling, Q2):** "(a) The room's canonical order (`canonical`, crates/room/src/room.rs) compares one slot's
    ticks wrap-aware, which is no total order for ticks 2^31 or more apart, and Rust's sorts may panic on a non-total
    order; under option B one command per slot per tick makes it unreachable, and carry 21's room test passes with no
    sort at all (TE-9). Before option A runs several commands of one slot in a tick, U4f's lead bound refuses such a
    command first, or the sort takes a total key, with a test of one slot's commands 2^31 ticks apart, and carry 21's
    room test is made to fail without the sort. (b) A refused snapshot datagram whose header tick reads far ahead moves
    the client's newest tick and resolves the whole span lost (crates/client/src/lib.rs, `receive` and `see`): the
    client files its sentinel without moving its newest tick past a bound, with a test. (c) `ClientEndpoint::step`
    drops a whole `Input`, acks included and uncounted, when one command fails to encode: it drops that command,
    counts it and keeps the acks, with a test. (d) The client's two too-old cases (marks/README.md §1's bad-header
    sentence; the advisor's U4e round-1 ruling, R1) are pinned by no test beyond what the judge counts: the round-2
    test seat's plant P-form (a too-old tick from the first snapshot on given the sentinel too, a ring slot taken, the
    bad tick acknowledged) passed every test. U4f adds one endpoint test: joined at tick 100 and fed to about 140, a
    group of tick 60 (before the first snapshot: a bad header) fed twice files `(60, 0)` once, the next `Input`
    acknowledges no tick 60, `newest_tick()` is unchanged, `refused()` rose by 2 and a delta on a live baseline still
    decodes; then a group of tick 105 (from the first snapshot on, already out of the ring) files nothing and adds 1 to
    `stale()`, never the sentinel; each half fails under its own plant. (e) Carry 61 (b)'s 'two different hashes for
    one tick give the sentinel' fails in one order: a tick completed by a snapshot and then sent as a full state of
    another hash files nothing (`full_state` counts it a duplicate), and `file`'s different-hash arm has no route from
    the endpoint's input; U4e's room sends a client a full state or a snapshot for a tick, never both, so no U4 run
    meets it. For a tick already received, `full_state` compares the frame's hash with the one filed and files the
    sentinel when they differ, with a test of that order that fails under its plant."

## U4g (prediction, interpolation and events)
47. **A-M2, the predicting World's reset (advisor-u4-wave1.md:240-247; the advisor's W1-U4D ruling):** "The predicting
    World's reset: an npt-sim entry that sets one slot's soldier from the decoded own-state records (its SOLDIER record
    and its owner-only record: W1-S1) and the press counters of the client's own command at the snapshot's last-run
    sequence, exactly as the authority held them after that command. A test: reset from the authority's records at
    tick T, then predict N commands, equals the authority bit for bit (D-3's rule from a reset, not from a clone). U4g
    edits crates/sim for it; no lane holds it then (the F2 ruling, point 6)."
70. **NF-L2, the synthetic ES recording's teleports (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "Since
    U4d-F, ES's synthetic recording teleports every soldier home once a minute (3b's re-home: about 52 teleports a
    minute with the random ones, against about 4 before). A predicting client fed that recording meets each one as a
    forced correction that no network caused, about 52 a minute. Any U4g test or figure that predicts on it counts
    teleport corrections apart from the others and reports both."
78. **NC-M4, the nearest 8 before the events (u4e-review-netcode.md, Block b; the test seat's probe Q-11; the advisor's
    U4e round-1 ruling, R3):** "U4e's encoder packs a snapshot's events before any entity change (crates/room/src/
    room.rs, `snapshot`), so an event backlog larger than a snapshot's remainder sends snapshots with no entity change,
    against `Priority`'s rule 'the nearest 8 enemies in every snapshot' (catalogue :129-130; ADR 0001 part 9 (d); the
    U4e brief, 2.1). After U4e's gap of `ACK_RING_TICKS` tick periods the backlog went out at about 23 events a
    snapshot and every snapshot still carried entity changes (Q-11), so no U4e or U4r run meets it; a backlog near the
    event cap would. U4g packs the nearest 8 enemies' changes first, then the events, then the rest, with a test in
    which a backlog larger than one snapshot's remainder still sends the nearest 8's changes in every snapshot."
79. **NC-L12, -62's own cosmetic count (u4e-review-netcode.md, NC-L12; the advisor's U4e round-1 ruling, R5):**
    "SERIES.md's `second.v.own_cosmetic_sent_count` names npt-room as its writer, and U4e's room counts every cosmetic
    event it sends (`cosmetic_sent`), not those sent to a shooter for its own shots. Before -62's judged runs U4g has
    the room count, per client, the cosmetic events sent for that client's own shots (0 in a correct build; above 0
    under PM-0001-62/server-echo), with a test under the bite."

## U4h (lag compensation and aim)
28. **netcode F6 / rust 3:** "The ring's span is U4h's test cap plus its margin in ms, recorded; the room builds it with
    `HitHistory::for_ms(span_ms)` (no tick count is ever written for it), and the recording header carries the span."
45. **NF-D4's U4h line (the advisor's U4s ruling, "Rulings (U4s)", NF-D4; added by the main session, the conditions
    check's F3):** "`World::ray_cast`'s ray is the only float that reaches npt-sim code from outside; it takes `&self`
    and changes no state; U4h's AimCast states what a non-finite ray does."
81. **SEC-U4E-8, option B's reruns and the view time (u4e-review-security.md; the advisor's U4e round-1 ruling, R5):**
    "Under option B the room reruns a client's last command on a miss with its original view time (crates/room/src/
    room.rs, `fill_input`), so a client that holds fire and stops sending gets each rerun shot rewound to an ever older
    view tick, up to the history's span, and `rewinds_refused` also counts reruns whose view tick has left the ring.
    U4h's tolerance judges each run of a command, reruns included, against the estimate at the tick it runs (or runs a
    rerun with fire released), counts reruns apart from claims, and shows by test that a starved uplink buys no rewind."

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
52. **N2-M1, Medium (u4d-round2-netcode.md; Mustafa, 2026-11-11: "The event-rate drift in the estimate scene: fix the
    scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does not move."):** "Before U4d2 pins any
    recording or hash, the estimate scene's event-rate drift is fixed: the recording's dead reckoning (replay.rs's
    generator) ignores crouch, dropped commands, scripted move speeds, blocked moves and deaths, so the event rate
    falls over a run (per 50 s window: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7 a second at 1 in 10; the test-engineer's probe
    X2, seed 2 at 1 in 11: 7.578 a second over 600 s). The fix and the re-calibration of `aim_one_in` are measured on
    the judged window (a 10-minute measured window after warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from
    tick 0. PM-0001-2's floor of 9.5 events a second does not move. The form, the measurement and the owner are the
    advisor's U4d close ruling (advisor-u4d-close.md, B1.2 and B1.3): the main session's change U4d-F lands the fix,
    with ES's vehicles inside the fight (N2-M2), before U4d2's brief is final. U4d2 pins only recordings written by
    U4d-F's `Recording::GENERATOR` or later, and ES's hashes only from U4d-F's scene; its report names U4d-F's commit."
65. **TE2-5's check.mjs half (the advisor's U4j round-2 ruling, B2.3):** "check.mjs's test step names every failing
    test. When a cargo test run fails, the lines `main` prints under its FAIL hold each failing test's name from
    libtest's `failures:` list (on the run's stdout) beside the summary line. Today `run` joins stdout before stderr
    (check.mjs:1402) and `main` prints a failed step's last 40 lines (:1649), so the names never show (the U4j
    fixer's check.mjs run 2: u4j-build.md:322-325). A check.test.mjs self-test feeds a recorded failing run and
    asserts the name is printed. The main session may make this change sooner, in its own commit; U4d2's report then
    lists entry 65 as met."
66. **U4c's netcode F2, Medium (u4c-review-netcode.md:53-54; L-c8's judged re-run, u4c-lc8-rerun.md; the advisor's U4e
    brief ruling, F2-ROUTE):** "npt-lab's probe also writes the SHA-256 of (v1's tag + the v2 fate log's datagram
    entries), the hash §5 U4c's L-c8 compares with U2's committed 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4
    seed 1, beside the whole log's SHA-256: in the run log's header and in its summary line, so that -3's script
    (ops/runs/c-u2-pm-0001-3.mjs) records it unchanged. It reads the log through npt-l1's public
    `FateLog::for_each_canonical` and edits no crates/l1 file; v1's tag and the datagram entry's form are those U4c's
    test `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` reads (l1/tests/l1.rs:481-532). U4d2's
    report shows the three values from probe runs of those three runs (the -3 set's own arguments), each equal to
    U2's, with the commands it ran. Until then L-c8's hash half rests on U4c's scratch harness through equal
    whole-log hashes (u4c-lc8-rerun.md). If U4d2's brief is final without this entry, U4r's brief takes it word for
    word, U4r in place of U4d2."

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

## U5 (fuzzing and the counting allocator)
48. **C13, carry 13's second half (advisor-u4-wave1.md:269-277; the advisor's W1-U4D ruling):** "Carry 13's second
    half: with PM-0001-34's counting allocator, a test that a push into a full `HitHistory` allocates nothing
    (u4s-carries.md entry 13; W1-U4D)." (U5's plan states where its allocator's `unsafe` lives under ADR 0002:264-265.)

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
