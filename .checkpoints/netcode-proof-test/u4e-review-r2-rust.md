# U4e round 2, rust-reviewer notes (log prefix rr)

Read only. Fixed copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted).
Round-1 read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1. (No Edit tool in this seat: this file is rewritten
whole with Write each time.)

## Plan
1. Read the packet's R1-R8 and T1 (advisor-u4e-r1.md) for my seat's items and the fix pass's scope/bounds.
2. Read my round-1 notes (u4e-review-rust.md): list my findings R8 put in the fix pass.
3. Read the diffs (packet) per crate: room src, server lib, client src, then tests.
4. Focus checks: panics/casts/overflow in changed code; R1 bounded memory (case 3 record of filed ticks);
   RR-2 (event age via u8::try_from + assert); RR-3 (const assert on Limits::GROUPS); changed tests passing
   for the wrong reason.
5. Check for changes beyond R8 scope/bounds.
6. Optional runs via u4e-run.sh (clippy maybe); test-engineer settles runs.
7. Verdict.

## Progress
- Block 1 (done): packet read whole (diffs, R1-R8, T1, carries). My round-1 findings R8 put in the fix pass (FX-13
  riders, R5): RR-2, RR-3, RR-10. The others: RR-1/RR-8 report words, RR-4 into C-U6-15, RR-5 C-U6-15, RR-6/RR-9
  card follow-ups, RR-7 carry 80.
- Start hashes: `sha256sum -c .gate/u4e/fx-hashes-end.sha256` in the copy: 23 of 23 OK; again at the end: 23 of 23.
- Bounds: `git diff --no-index --stat` of the whole spike tree r1 -> fixed: 11 files, all under crates/room,
  crates/server (lib.rs, pump.rs, tests) and crates/client; no manifest, no Cargo.lock, no schedule.rs, no
  proto/sim/link/lab file. Public surface diff (grep of pub lines, r1 vs fixed): + RttSource (pub enum, re-exported),
  take_control's type, Decoder::new's new arg (A2's form), + ClientCounters::resyncs_limited and
  Counters::joining_dropped (pub fields of the existing pub counter structs, each the name R3 and R5 ask for), eight
  items pub(crate). Within R8's bounds.
- Block 2 (done): runs (both through u4e-run.sh, shared lock; none counted, none planted, the copy untouched):
  - r2-rr-probe.log: scratch crate .gate/u4e/rr-probe (outside the workspace, own target dir, path deps on the
    copy's npt-client/proto/sim/link; endpoint.rs's helpers copied). Exit 0. Printed:
    A: "full state 102 filed [(102, 2619550959489136125)] ...; then a snapshot of 102 decoding to the same frame filed
       [(102, 0)]; refused 1 duplicates 0"
    B: "snapshot 101 filed [(101, 6693525158538537769)]; then a full state of 101 with another hash
       (1093065824202657116) filed []; refused 0 duplicates 1"
    C: "newest Some(65560); bad headers 0..=40 filed 41 entries, all sentinels true; tick 0 again filed [(0, 0)];
       tick 40 again filed []; refused 43 stale 0"
  - r2-rr-clippy-extra.log: round 1's command again (clippy on the three libs with cast_*, indexing_slicing,
    unwrap/expect_used, panic, missing_docs, missing_panics_doc). Exit 0; 878 warnings (r1: 879). In room/server/
    client sources the per-lint counts are equal to round 1's but one: the `u32 as u8` cast is gone (RR-2's site).
    No new cast, indexing, unwrap/expect, panic or missing-doc site from the fix pass.
- The builder's lint and counted runs (read, not re-run): fx-lint2.log fmt/clippy/clippy-wasm32/doc all PASS;
  fx-counted2.log all 10 steps PASS, measured 473, bites 541, self 52 of 52.

## Checked (reading)
- check.mjs runs `cargo test --locked --workspace [--all-features]` (check.mjs:1494-1496): dev profile, so
  debug_assertions and overflow checks are on in every counted test run.
- RR-2 (room.rs:1213-1220): route_events(t) runs `queue.expire(t)` (retain age <= MAX_EVENT_AGE = 255, events.rs:20,
  :87-91) for every non-kicked client (room.rs:979-985) before encode_all(t) with the same t; events pushed are
  stamped <= t. So try_from is always Ok; the debug_assert is unreachable from network input; release skips the
  event (`continue`, right: the queue is oldest first). CLOSED.
- RR-3: const asserts at room/src/ack.rs:48-53 and client/src/lib.rs:167-172 (`Limits::GROUPS <= 8`); the decoder
  bounds group < groups <= 8 (3-bit field), so `1 << g` on a u8 mask never overflows. CLOSED.
- RR-10: `checked > 0` asserted after the loop (room.rs:1469). CLOSED.
- send_control (room.rs:325-332) drops an encode failure silently: JoinAccepted and Kick encode infallibly (fixed
  fields, under their caps: proto msg.rs:355-373); no finding.
- Resync limit (room.rs:717-736): `next.wrapping_sub(f) < FULL_STATE_ACK_TICKS`, u32 wrapping, no panic; next is
  next_tick (the answer's tick), f the last full state's tick (set for join, forced and answered: room.rs:1125);
  last_full_state is None only while the join's full state is due, which the first branch drops.
- A4 (room.rs:831-838): `append` moves the script into `input` and keeps both Vecs' capacity: no per-tick
  allocation; fill_input runs on every tick (room.rs:805-810).
- pump `_ => joining_dropped` (pump.rs:434): only (Joining, stream non-Join) reaches it (arms :393-431 enumerated);
  pong (pump.rs:457-473): echo checked with the id, arrival_ns.saturating_sub: no panic.
- Client case 3 (client/src/lib.rs:493-530): after32(a, b) = b after a (proto wrap.rs:60-65), so
  `after32("tick", t, f)` = t before first: right. `first` is set with `newest` (lib.rs:551-555), so it is Some
  whenever too_old can be true. Memory: `first` one u32, `bad_headers` VecDeque <= 32 (pop_front at the cap),
  contains O(32); no ring slot, no see, no acknowledge. A case-3 tick is never in a lost run (lost runs start at
  low = first) and never gets a slot later (newest never moves back): never both. lost(): checked_add (SEC-U4E-7).
- stream_of (server/tests/common/mod.rs:193-204): a FullState's length byte 2 is 0 or 1 within FULL_STATE = 65,536
  (proto limits.rs:30), and TYPES[0] is Input (Datagram): never misrouted as Control.
- Changed tests read for vacuity: SEC-U4E-4 (boundary exact: limited while next - f < 60, answered at f + 60, and
  resyncs_dropped 0 separates the two rules), e9, A4, A5 (len >= 3 and all Present; non-empty and all Rewound), A1
  (bitten bytes = the bitten Quant's re-encoding, clean = npt_proto::encode's, and they differ), TE-2 (pump_report
  adds: room.rs:700-708, so P counts; `left` is the remainder the encoder packs into), TE-5, NC-M3, A2 (each
  plant flips its assert), SEC-U4E-1/-3, TE-4, SEC-U4E-7 (want = 3 x (2^31 - 1) - 31, the runs chained), TE-3,
  tick16 twin (bad >= 1, orphans below 65,520, none >= 65,536), carry 57 (resyncs.len() == 1 before resyncs[0]).
  Sound but the three below.

## Findings (a line after each)
- RR2-1 (Low, test; not before merge) room/tests/room.rs:626-627 (carry_50_the_room_sends_an_event_at_age_255_and_
  none_at_256, from :573): `ages` holds u8 ages widened (:604), so `assert_eq!(ages.iter().max(), Some(&255), "none
  at 256 or older")` cannot fail once `ages.contains(&255)` holds; and the event is matched by (number, header tick
  - age), so a wrapped resend (age 256 coded 0) would show as another (n, at) and be skipped. The "none at 256" half
  rests on the room's own debug_assert (P-G failed the test by that panic: fx-pg.log:36, "panicked at
  crates/room/src/room.rs:1214:9") and on `events_expired >= 1` (:628). Sound as a whole in check.mjs's dev profile;
  the assert's message claims a check it does not make. Fix (cheap): match the event by its number alone and assert
  no snapshot after tick at + 255 carries it.
- RR2-2 (Low, test and a carry-61 (b) gap; not before merge) client/tests/endpoint.rs:197-221 (TE-7) with
  client/src/lib.rs:694-701, :782-785, :617-620. The second case ("a tick completed by its full state and then sent
  again as a snapshot of other content") gives the sentinel by the byte-signature rule (a full state leaves
  sigs[0] = 0), whatever the hash: probe A (r2-rr-probe.log) feeds a snapshot decoding to the SAME frame and gets
  (102, 0). The order in which one tick really completes twice (snapshot first, then a full state with another hash)
  files no sentinel: full_state() counts it a duplicate (lib.rs:782-785); probe B: filed [], duplicates 1, against
  carry 61 (b)'s "two different hashes for one tick give the sentinel". `file()`'s `Filed::Hash(h) if h != hash`
  branch is unreachable (group() returns at the got-bit check for any group of a complete tick; full_state() at
  got != 0), so P-H's Edit 1 changes nothing and P-H fails TE-7 by its Edit 2 alone. Reach: none from the U4e
  room (encode_all sends a client a full state or a snapshot per tick, never both: room.rs:1083-1090), and a judged
  run would count the pair anyway (one server hash per tick against the client's first). Fix: full_state() at
  got != 0 compares (hash the frame, call file()), or the test's doc names the byte rule and the one order.
- RR2-3 (Low, contract wording; not before merge) client/src/lib.rs:519-530 with T1: case 3's "once" holds while
  the tick is among the last BAD_HEADERS_HELD = 32 bad-header ticks remembered: probe C files tick 0's sentinel a
  second time after 40 other bad ticks (tick 40, still held, files nothing). The bound is R1's own ("with bounded
  memory") and the doc says it (:519); in the tick16 bites a tick's groups arrive together, so no judged run meets
  it. T1's sentence says "once" unconditionally; U4r's writer should take an equal repeat (t, 0) as filed once (the
  lab already does: server/tests/common/mod.rs:338-342). Route: a line in U4r's carry 73 or the report.

## Round 1 closed (my seat, R8's FX-13 riders)
- RR-2: closed (room.rs:1213-1220; the cast lint's site gone in r2-rr-clippy-extra.log).
- RR-3: closed (ack.rs:48-53, client lib.rs:167-172).
- RR-10: closed (room.rs:1469, `checked > 0`).

## Verdict
APPROVE WITH FINDINGS: Lows only (RR2-1, RR2-2, RR2-3), none before the merge; no panic, cast or overflow from the
changed code reachable by network input; R1's case-3 memory bounded (one u32 and at most 32 ticks); the fix pass
within R8's bounds.

## Not verified
- No plant run of mine (the test-engineer's seat); no bench (R8: none this round); no counted check.mjs run.
- No wasm32 run of the client (clippy-wasm32 and doc only, from the builder's fx-lint2.log).
