# U4e round 1: rust-reviewer notes

Seat: rust-reviewer (read only). Read copy: .gate/u4e-r1 (97bb9d3 + U4e snapshot). Builder copy:
.claude/worktrees/wf_0fa6595a-3e2-1. Packet: u4e-packet.md, u4e-packet-tests.md. Log prefix: rr. (No Edit tool in
this seat: the file is rewritten whole with Write each time.)

## Plan
1. Read the packet's judged-against sections (plan §5 U4e, §4.1, §4.2, §4.10, W1-*, advisor-u4e-brief, B2.1, B5,
   carries 21-27, 39, 50, 51, 53-57, 61, 68, 40, 62, 66; the left-out ids).
2. Read the sources in .gate/u4e-r1: crates/room/src, clippy.toml files, crates/server/src, crates/client/src,
   crates/sim/src/scene.rs (effect_rows).
3. Focus checks: (a) integer-only, wraps, overflow; (b) hash-map bans, order, carry 21; (c) panics and casts, reach
   from network input; (d) allocation per tick at 48 clients, carry 26, quadratic paths; (e) constant-time compare;
   (f) Send, wasm32, API shape, docs, lints, tests passing for the wrong reason; (g) Scene::effect_rows (A19).
4. Optional runs through u4e-run.sh with prefix rr.
5. Verdict.

## Block 1 (done): what I read
room/src all 9 files, server/src/pump.rs + lib.rs, client/src/lib.rs, scene.rs accessor, both clippy.toml, workspace
Cargo.toml (overflow-checks in release only for npt-sim and npt-proto), proto wrap.rs, snapshot.rs decode bounds (group
3 bits, groups 1..=8, group >= groups refused: snapshot.rs:563-580), hash.rs client_hash (sorts events by (number,
kind): hash.rs:83-95), msg.rs Join's redacted Debug (:67-84), seam.rs:116-121 (the canonical order's words), plan.md
§4.7 :596-598 (npt-room's listed types), marks.json PM-0001-4. Counted run b11-counted2.log: 10 steps PASS, measured
458, bites 524, self 52/52. check.mjs WASM_CRATES = LEAF (npt-room, npt-client included) + npt-tcpinfo (check.mjs:31,
:1438): clippy-wasm32 and the wasm32 doc step cover both. e8: Room<World> (room/tests/room.rs:214-218), Decoder
(server/tests/pump.rs:52-56), ClientEndpoint (client/tests/endpoint.rs:91-95). Change set (builder copy `git status`):
no manifest or Cargo.lock change, so no new dependency (cargo deny / audit surface unchanged); server/src/lib.rs drops
the module paths (W1-PUB nit met). Tests skimmed for vacuity: room/tests/room.rs OwnCheck (fails on "got no snapshot":
:516-518), e7 (per client >= 1,000 hashes; client-only pairs count as bad: common/mod.rs:345-356), carry 61
(endpoint.rs:157-200), bites pins: none found passing for the wrong reason but RR-10.

## Block 2 (done): runs (all through u4e-run.sh, shared lock; none counted, none planted)
- r1-rr-asm-native.log, r1-rr-asm-wasm.log: `cargo rustc --locked -p npt-room --release --lib [--target
  wasm32-unknown-unknown] -- -C codegen-units=1 --emit asm=.gate/u4e/rr-asm/room-{native,wasm32}.s`, exit 0.
  Roster::admit, x86_64 (room-native.s:13067-13248): per slot one 16 B pxor + OR-reduce, the black_box spill (#APP),
  shl/add/sar to the 0/1 mask, `cmovne` for the `hit == 0` guard, and/or; the loop's only branch is its counter.
  wasm32 (room-wasm32.s from :20538): 16 load8/xor/or, store8/load8 (black_box), shl/shr_s, `i32.select`, or; only the
  loop's br_if. The constant-time fold survives optimisation on both targets at 1.99.1. std's own words on black_box
  ("best-effort ... does not offer any guarantees for cryptographic or security purposes", doc.rust-lang.org
  std/hint/fn.black_box.html) mean this holds for this toolchain's codegen, which the pin fixes; not a finding.
- r1-rr-clippy-extra.log: clippy on room, server, client libs with -W cast_possible_truncation, cast_sign_loss,
  cast_possible_wrap, indexing_slicing, unwrap_used, expect_used, panic, missing_docs, missing_panics_doc. Exit 0.
  U4e crates: 0 missing_docs, 0 missing_panics_doc; one expect (room.rs:423, Room::new, const input, `# Panics`
  documented). Every indexing site checked by hand: each index comes from a binary_search Ok, a `% ACK_RING_TICKS`
  slot into a 32-entry ring, a clamp, a checked bound, or this tick's own records; none from a wire value unchecked.
  Casts: each intended (16-bit wire values, wrap-aware i32 reads) or bounded (groups <= 8, ages <= 31), but RR-2.
- r1-rr-bench.log: scratch bench .gate/u4e/rr-bench (outside the workspace, own target dir; counting global allocator
  on only around `room.tick()`; twin World stepped on `room.tick_input()`, its state hash asserted equal each tick:
  held for all 700 ticks of every run). SS seed 1, 48 clients acking every snapshot at once, 100 warm + 600 measured
  ticks, release, overflow checks on for sim and proto as in the workspace. Three runs:
  room.tick mean 5.61 / 5.89 / 6.07 ms, p50 5.46 / 5.70 / 5.96, p99 10.5 / 11.5 / 9.7, max 23.2 / 17.2 / 12.6 ms;
  twin step mean 0.20 / 0.21 / 0.21 ms; so the room's own share is about 5.4-5.9 ms mean, p99 9.5-11.2 ms.
  Allocations inside room.tick: mean 2,838 a tick (p99 3,310), bytes requested mean 6.20 MB a tick (p99 11.3 MB),
  identical across the three runs. At 8 clients: room.tick mean 0.89 ms, 401 allocations, 0.68 MB.
  Per client: 96 us at 8, 115 us at 48: linear in clients (no quadratic term seen). Snapshots 1.000 groups each,
  852.5 B with 28 B a datagram. Noise: the runner's 2 s touch loop runs beside it; 4 vCPU, load 0.16 before; the max
  values vary run to run (12.6-23.2 ms) and are not a stable figure.
- r1-rr-callgrind.log (+ .gate/u4e/rr-bench/callgrind.out): 48 clients, 20 warm + 60 ticks, line tables only.
  Inclusive Ir: Room::tick 94.2% of the program (which also runs the twin's step); snapshot() 73.7%;
  EntityDelta::between 13.5%; encoder::view 10.0%; Quant::encode 10.0%; client_hash 10.0% (records 9.8%); the
  `c.relevant.sort_by_key(|&i| key(&records[i]))` driftsort 8.9%; memcpy 6.9%; World::step 9.9% for both calls.

## Findings (a line after each)
- RR-5 (Medium, perf; not before merge) the room's tick at 48 clients: 5.4-5.9 ms mean, p99 9.5-11.2 ms of its own
  work, 2,838 allocations and 6.2 MB requested per tick (block 2). PM-0001-4 (marks.json) holds the room's
  tick.v.work_ns to p99 16.67 ms and max < 33.33 ms at L2 SS 48: the room alone takes about 60-70 % of the p99 budget
  on this machine, before the pump, logging and the network. Cheapest wins: room.rs:926 (sort indices by key through
  `records`, 8.9 % of Ir: emit `relevant` in `index.by_key` order instead), :1271-1273 (three clones per group of
  vectors the Packer owns: move them after `view`), encoder.rs:192 (a fresh baseline-frame clone per client per tick:
  reuse the evicted ring entry's vectors), room.rs:880-881, :946, :1028, :1133, :1183-1184, :1260, :1275, :1297,
  :1303 (per-client Vecs each tick); client lib.rs:663 + :672 (the baseline frame cloned twice for a tick's first
  group). Carry 26's one copy into the reused `stamped` list is met (room.rs:774-779). Proof: r1-rr-bench.log; settle
  by re-running the bench after a fix (same command).
- RR-1 (Low) rewind.rs:73-77: the debug_assert re-checks `holds()` right after `if !holds() { return None }`: it
  can never fire, so §4.2's "the debug assert that a correct room never sends one sits in the room's rewind_at"
  asserts nothing. Fix: drop it, or assert in fill_input that a Some Rewind's ticks are held.
- RR-2 (Low) room.rs:1170: `t.wrapping_sub(q.tick) as u8` truncates silently; safe only because route_events'
  `expire(t)` (events.rs:332-337) runs first in the same tick. A `u8::try_from` with a debug_assert ties the two.
- RR-3 (Low) ack.rs:40, :145; client lib.rs:154, :163, :566, :633: the u8 group masks and `sigs: [u64; GROUPS]` rely
  on Limits::GROUPS <= 8 (limits.rs:18-20) and the wire's 3-bit group (snapshot.rs:578-580). Safe today; no const
  assert pins `Limits::GROUPS <= 8` beside the masks.
- RR-4 (Low, perf) events.rs:327: `retain(|q| !numbers.contains(..))` is O(queue x acked) per acking Input (cap
  1,024 x about 100 numbers in a burst). The queue is in number order: a merge or binary search makes it linear.
- RR-6 (Low) room.rs:1003, :1015 and ack.rs:76-77: the full-state deadline and the ring use the 30 Hz constants
  while Room::new takes `tick_hz` for the window: a Room at 60 Hz keeps a 1 s deadline and a 32-tick ring.
- RR-7 (Low, a carry for U4f) room.rs:344-350: `canonical`'s within-slot comparator is not a total order for two
  ticks 2^31 apart (each sorts before the other) or spanning more. Unreachable today: fill_input pushes at most one
  command per slot (room.rs:799-834). If U4f's option A runs several commands of one slot per tick, the ticks are the
  client's (input.rs:107-122 takes any tick up to 2^31 - 1 ahead), and std's sorts "may panic" on a non-total order
  (slice docs, sort_unstable_by's text; I did not read sort_by's own paragraph) - a panic from network input.
- RR-8 (Low, API) lib.rs:23-30: AckRing, Encoder, EventQueue, InterestGrid, AreaFlood and Priority are pub (plan.md
  §4.7 :596-598 lists them) but every constructor and method is pub(crate) and no accessor hands one out: public in
  name only. ClientCounters (room.rs:63-133) is a new pub type beyond plan §4.7's list (W1-PUB's "type in a listed
  item's signature": ClientSlot::counters), to be named in the report for engine-architect.
- RR-9 (Low) room.rs:1166-1180, :1226-1240, :1248-1256 then :1288-1291: when the encoder refuses (carry 54's path),
  each event's `q.sent`, the priority accumulator and `cosmetic_sent` were already moved, so the next tick counts the
  events as resent and the cosmetic counter overstates. State and hashes are unaffected (deltas are against B).
- RR-10 (Low, test) room.rs:1402-1406 (carry 23's unit test): the queue check runs only if client 0's queue holds an
  event; nothing asserts one was seen, so that half can pass vacuously (the stamps half cannot: :1408-1411).

## Checked, no finding
- Integer only: `#![deny(clippy::float_arithmetic)]` at room lib.rs:12 (carry 25); no f32/f64 in room/src; the
  client's only floats are npt-sim's grid calls (lib.rs:67, :84). Wraps: every 16/32-bit compare goes through Wrap
  with its counter name; budgets: tick_budget's sums exact (encoder.rs:26-31, pinned at 30 and 60 Hz :239-251),
  Packer's `left` never underflows (encoder.rs:136, :146), pump buckets saturate (pump.rs:51-64).
- Order: no HashMap/HashSet in room, server or client sources; the bans in both clippy.toml (U4E-Q8); BTreeMap in
  Priority (interest.rs:159); every sort has a total key (room.rs:856, :926, :927, :1217-1224); carry 21 met by
  `canonical` (unit test room.rs:1347-1356) - its integration test (tests/room.rs:222-266) shows arrival-order
  independence, which fill_input's slot-order loop gives by itself.
- Panics from network input: none found (block 2); overflow in release wraps silently in room/server/client (no
  overflow-checks there), and no wire-driven arithmetic reaches one (all wrapping or saturating).
- Constant time: block 2 (both targets branch-free per slot).
- Send: all three e8 checks present (compile-time, in test targets). wasm32: clippy-wasm32 and doc pass
  (b11-counted2.log:124-125). Docs: 0 missing on public items. fmt and clippy pass in the counted run.
- Scene::effect_rows (A19): a plain getter, listed in §4.1's npt-sim row (carry 51), its doc matches world.rs:1901-1903
  (`i / rows`, 0 rows giving no rows); the room reads it once (room.rs:430) and never copies the 8 (:1036-1039).
- Wasm size: not applicable at U4e (npt-client-wasm is a 2-line stub with no exports until U7); not measured.

## Verdict
APPROVE WITH FINDINGS: no Critical or High; one Medium (RR-5, perf against PM-0001-4's budget, not judged at U4e)
and nine Lows; none must be fixed before the merge. The fix pass may take RR-5's cheap wins and RR-1/RR-2/RR-3 in
passing; RR-7 goes to U4f's brief as a carry.

## Not verified
- No wasm32 run of the client's hash against the native mirror (no wasm runner at U4e; U4d2/U7).
- The bench measures room.tick only (no pump, no logging, no real link) on this 4-vCPU VM, not R2.
- I did not re-run the counted check.mjs or any bite (the test-engineer's).
