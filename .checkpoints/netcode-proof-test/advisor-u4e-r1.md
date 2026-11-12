# Advisor: U4e round 1 (the room and the codec path), 2026-11-12

Seat: the advisor agent, called from the day session's workflow (branch claude/netcode-proof-test-plan-hcky5g; PR #40).
Role read whole from main (`git show origin/main:.claude/agents/advisor.md`). I rule; the main session applies every text
change and builds nothing itself in the copy; one builder runs the fix pass. I write only this file, with Write and
Edit, block by block. Read only: no build, no run, no plant; reading, git and sha256 only.

## Plan
1. Read the brief (advisor-u4e-r1-brief.md), the five seats' notes, the builder's record, my brief check
   (advisor-u4e-brief.md), U4d-F's ruling (M1-M9's form), U4j round 2's B2.1 and B5, the plan's §4.5, §4.6, §4.9,
   §4.14 and §5 U4e. (done)
2. Spot-check the code the rulings rest on, in .gate/u4e-r1 (the read copy): the client's stale drop and the sentinel
   (NC-H1, TE-1), the pump's RTT (NC-M3, A2), `take_control` (A1), `tick_input` (A4), the Resync rule (SEC-U4E-4), the
   tick16 twin; the copy against the snapshot (sha256); carries 61, 62, 68 and the card's "Running".
3. Rule items 1-9 in order, one block each (R1 ... R9), then the texts, the two lists, and what I checked.

## Progress
- [x] 1 read  - [x] 2 spot-checks  - [x] R1  - [x] R2  - [x] R3  - [x] R4  - [x] R5  - [ ] R6  - [ ] R7  - [ ] R8
- [ ] R9  - [ ] texts  - [ ] end sections

## Spot-checks (reading, git and sha256 only)
- Both copies equal the test seat's start list: `sha256sum -c` of u4e-review-test/hashes-start.sha256 run in the
  builder's copy and in .gate/u4e-r1, 23 of 23 OK each; the builder's copy at HEAD 97bb9d32aa7b…, 24 status lines (the
  23 files and its record). wip/u4e.patch sha256 e4203346… equals db49108's committed copy.
- NC-H1 / TE-1, read in .gate/u4e-r1: `too_old` (client/src/lib.rs:466-471) runs first in `group` (:622-625),
  `full_state` and `sentinel` (:534-538), so a header tick at least `ACK_RING_TICKS` behind the newest is counted in
  `stale` and nothing is filed; npt-proto's bite writes `tick & 0xffff` (proto/src/quant.rs:133-139); the twin asserts
  only `server.is_some() && server != client` (server/tests/loopback.rs:241-252). The lab's `compare` counts a client
  hash as bad when it is 0 or the server has no equal one (server/tests/common/mod.rs:345-355): the judge's rule
  (SERIES.md:266). The judge returns a mismatch count before it looks at a coverage gap
  (judge/src/figures.rs:542-554), so orphans make a bite run FAIL even with the gap the stuck client leaves.
- NC-M3 and A2: `pong` takes the pump's `now_ns` (pump.rs:389, :424) while the Ping answer uses `r.arrival_ns` (:376);
  `rtt_ns` is `link.rtt_ns().or_else(..)` and `Decoder::new` takes no session kind (pump.rs:203-209, :264-269).
- A1: `take_control` returns `Message`s (room.rs:576-584); `full_state` encodes into a `probe` it throws away
  (:1076-1079); the lab encodes again with `npt_proto::encode` (server/tests/common/mod.rs:274-282). plan.md:508-509:
  "Encoding stays on the tick, as -6's share." (verified).
- A4: `tick` clears spawns, teleports and attributes after the step (room.rs:780-782); `script` writes the same fields
  (:587-599).
- SEC-U4E-4: a Resync is refused only while a full state is due or in flight (room.rs:692-698), and an `Input`'s ack
  of the full state's tick clears `in_flight` (:734-741).
- A5: of its nine items, `Roster::new` and `is_bound` are used only by roster.rs's own unit tests; `with_lead`,
  `closed`, `unknown_pongs`, `skipped`, `is_joined`, `limits`, `join_deadline_ns` have no caller outside their crate
  (grep); `with_cover` has none either.
- The highest carry number in u4s-carries.md is 72; the highest U6 carry in plan.md is C-U6-11.

## Rulings

### R1. NC-H1 (High) with TE-1 (High): the finding holds; the reading, the fix's form and the unit proof. REVISE (before the merge).
**The finding holds, by reading and by run.** Under either tick16 bite the header carries the tick's low 16 bits, so
after 65,535 the client reads 0, 1, 2 …; each is at least `ACK_RING_TICKS` behind its newest (65,535), so `too_old`
drops it before `see`, a hash, the sentinel or a lost record. The test seat's Q-1 shows it: "client 0 post-wrap keys
[]; server post-wrap hashes 14; compared 30, bad 0; sentinels 0; … stale 14; lost []" (r1-te-q1-tick16.log, for both
ids). The twin's `server != client` holds with the client's `None`, which is the silent drop. §4.14's row asks for "a
frame at tick 65,537 [that] hashes differently or is refused", and its failing check is "post-wrap frames logged under
the wrong tick (an orphan pair) or as the sentinel"; the figure counts neither a dropped group nor a server-only pair
(SERIES.md:266: "a server hash with no client hash is loss, not compared"). So U4r's judged bite runs would read 0
mismatches: a bite never caught. Evidence that does not hold: High, as both seats rate it.

**The reading (mine to rule: it reads my own text, B2.1 and carry 61, not an ADR row or a word of Mustafa's).** Carry
61 (d) ("a group of a tick older than one the client has already decoded … is decoded … or, if the client drops it as
stale, it is never acknowledged and that tick is lost, never the sentinel") was written for L1's reorder step: a tick
the client was sent and can still place. A tick from the client's first snapshot on is one of three kinds, and a tick
before it is a fourth:
1. **Inside the ring** (less than `npt_sim::ACK_RING_TICKS` ticks behind the newest tick the client holds): decoded and
   hashed under its own tick, as built. This is (d)'s decode, my B2.1 lean; NC-L4 and A's "decodes" are right.
2. **From the first snapshot on, at least the ring behind the newest:** that tick was resolved when it left the ring
   (lost, or filed), so nothing can be filed for it now; a group of it is dropped, never acknowledged, and counted
   (`stale`), as built. This is (d)'s drop, for a tick already lost; it creates no new lost tick.
3. **Before the client's first snapshot and at least the ring behind the newest:** a tick the client was never sent and
   never resolved. Its header is §4.6's "bad header" (the sentinel causes B2.1 keeps: "no baseline, a bad header, a
   size or decode error"): the client files the sentinel under the tick the header names. That is a client hash at a
   (slot, tick) the server never logged, which the figure counts (SERIES.md:266; §4.6 :323-325's orphan).
- Why case 3 is safe for a correct build: in an honest run no group names such a tick. L1 builds no duplication
  (l1/src/lib.rs:75) and changes no byte; a datagram arrives late by at most its profile's jitter spread plus one
  spacing (:15-23; P4's ±40 ms round trip is the widest, :195), far under the ring's 1.067 s; the room sends no
  snapshot while a client's full state is unacknowledged (room.rs:1041-1047), and that full state is the client's
  first tick. So it changes nothing in an honest run and catches the bite: stricter, never looser.
- Why not the netcode-architect's simpler form (every header at least the ring behind the newest gets the sentinel):
  for a tick of case 2 that would file a sentinel at a tick already written lost, which README §1's coverage rule
  forbids ("never both") and a writer cannot place, so a correct build meeting one very late group (possible only
  off L1, on a real network at U6) would end INVALID or `crash`. Case 3 carries the same proof without that risk.
- No readings-list entry: no ADR row or word of Mustafa's is read; -17's "100 % of decoded snapshots" is untouched (the
  sentinel is not a decoded snapshot, and the figure already counts it).

**The fix's form (npt-client; the fix pass):**
- The endpoint keeps the tick of the first header it took (its first group's or full state's).
- Wherever it meets a header tick (`group`, `full_state`, and the refused snapshot datagram's tick read in `receive`):
  case 1 as built; case 2 as built (`stale`); case 3 files `(t, 0)` in `StepOut::hashes` once per tick, with bounded
  memory, without `see`, without `slot_for` (the ring slot `t % 32` may hold a live tick: 65,537's slot is 1's), never
  acknowledged, and counted in `refused` with the other sentinel causes.
- The newest tick never moves for a case-3 header (it is behind by definition). NC-L7's far-ahead header stays a
  follow-up (R5).

**What the unit proof asserts (what the judge counts),** in `pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_
client_hash` (renamed to say it, e.g. `..._are_counted_as_mismatches`) and its clean twin, over `tick16_lab`'s 30 ticks:
- clean build: `lab.compare()` gives (n, 0) with n ≥ 30, the client's hash at (0, 65,537) equal to the server's, and no
  client key outside the ticks the room ran (65,520 on);
- under each of PM-0001-17/tick16 and PM-0001-58/tick16: `lab.compare().1` ≥ 1 (the figure fails); slot 0 has at least
  one client key below 65,520 whose server entry is absent (an orphan), and none at or past 65,536; the test prints
  compared, bad, the orphan keys and `stale()`. (Q-1's run predicts 14 orphans, one per post-wrap tick reached.)
- **Plants** (by Edit in the copy, restored, sha256-checked; each named in the record): (a) case 3 put back to case 2's
  silent drop: the bitten twin FAILS by name; (b) s03/s04's scratch switch (the bite's defect in place in the clean
  build) re-run once: the clean test FAILS by name. Part (i) in the counted run: the twin passes and no clean test
  fails.

**The judged half is U4r's** (carry 73, T2): its client writer must place hashes filed at ticks before the run's first
tick (README §1 already asks it to buffer and sort, since "a client's rebuilt ticks can go back"), and its contract
test runs one short -17/tick16 bite run past the wrap and shows the figure FAIL with mismatches ≥ 1, never INVALID
through a `crash` footer.

**The contract text** (marks/README.md §1, in the merge commit: T1) gains case 3 in one sentence, so the rule both
sides read holds it. No judge code changes. Severity of NC-H1 and TE-1 stays High until round 2 sees the fix.

R1 written 2026-11-12.

### R2. The six Mediums marked before the merge: all six in the fix pass. APPROVE as findings, with these forms.
Each label stands. Each code fix comes with a test that fails under a named plant (by Edit in the copy, restored,
sha256-checked, recorded), as the test seat proved its findings.
- **NC-M2 (carry 68 not met): the report's words, no code.** Carry 56's ES figure is "an ES byte figure U4e reports
  from that recording", so carry 68 applies as worded ("states those two shares beside R2's"); the two shares are the
  recording's, which carry 68 gives (about 48 % and 18 %). The report carries T5's line beside carry 56's figures, with
  the numbers of the merged tree's run (M5). A print of the own soldier's own shares would be better evidence but is
  not what the carry asks; it is not required. Carry 68 is then listed as met.
- **NC-M3 (the Pong's RTT sample from the pump's now): fix.** `pong` takes the datagram's `r.arrival_ns`, as the Ping
  answer beside it does (pump.rs:376). Test: Q-2's case committed (the Pong sent at 1,000 ns + 100 ms, the pump run 20
  ms later): `rtt_ns` gives 100 ms less the hold, where today it gives 120 ms (r1-te-q2-q5-pump.log). Plant: `now_ns`
  passed again; the test fails by name.
- **A1 (control messages handed out typed, encoded twice): fix.** `Room::take_control` hands out bytes: each reliable
  message encoded once, on the tick, with the room's own `Quant` (the encode `full_state` already does to count a
  refusal is the one kept; `JoinAccepted` and `Kick` likewise), so plan.md:508-509's "Encoding stays on the tick" holds
  in the library, not in each caller. The caller sends each on the channel its type byte's row of `Limits::TYPES`
  names (npt-proto's documented rule, limits.rs:55); the lab (server/tests/common/mod.rs) does exactly that and no
  longer encodes. Test (bites builds): under `Room::with_bite("PM-0001-57/pos-4cm")`, one join; the FullState bytes from
  `take_control` equal `Quant::with_bite("PM-0001-57/pos-4cm")`'s encoding of the message they decode to and differ
  from `npt_proto::encode`'s. Plant: `take_control`'s bytes encoded with `npt_proto::encode`; the test fails by name.
- **A2 (the RTT source chosen by what the link answers): fix.** §4.9 (u4-build-plan.md:352-354) names the source by the
  session's kind, which the pump learns from its listener. `Decoder::new` takes it as a new npt-server type,
  `RttSource` with two variants, `Link` (a wt or L1 session: the link's `rtt_ns`, `None` until it has one, never the
  ping samples) and `Pings` (a ws session: the lowest of the last 8 samples, never the link's). One new public item,
  its reason §4.9, in §4.1's npt-server row (T6). Tests: a link answering `Some(x)` with a `Pings` session and one
  sample y ≠ x gives y; a `Link` session over `NoRtt` gives `None` with a sample held. e10's ping-path test passes
  `Pings`. Plant: `rtt_ns` back to `link.rtt_ns().or_else(..)`; the first test fails by name. The report does not repeat
  the record's line that the pump's report to the room carries the RTT (u4e-build.md:22-23): `pump_report` has none,
  which is right at U4e (U4h's tolerance is its consumer).
- **A4 (`tick_input()` holding two ticks' state): fix.** The scripted spawns, teleports and attributes wait in their own
  field; `fill_input` moves them into `input` for the tick they are for; nothing clears `input` after the step. Then
  `tick_input()` after `tick()` is that tick's whole input (its doc: "what a recording holds"). Test: script one
  teleport, `tick()`, and `tick_input().teleports` holds it; a second `tick()` with no script gives none. Plant: the old
  clear after the step; the test fails by name.
- **A5 (public items beyond §4.1 and plan §4.7): fix, with one correction.** Eight of the nine items with no consumer
  become pub(crate): `Roster::new`, `Roster::is_bound` (both used only by roster.rs's own unit tests), `Decoder::limits`,
  `Decoder::is_joined`, `ConnLimits::join_deadline_ns`, `ClientEndpoint::closed`, `ClientEndpoint::unknown_pongs`,
  `ClientSlot::skipped`. **`ClientEndpoint::with_lead` stays public:** its consumer is U4r's lab, which must set the
  lead from the profile (NC-M6's carry 74, R3); the report gives that reason. `Room::with_cover` stays public (§4.9's
  two cover variants, recorded per run) and gains a test (§4.11's rule): a room built `with_cover(CoverVariant::Present)`
  hands the sim each Rewind with `cover` Present. A later unit that needs one of the eight makes it public with its
  reason. The report lists every public item beyond §4.1's rows with its reason (the engine-architect's list,
  u4e-review-arch.md:57-66, updated for A2's `RttSource`, A1's `take_control` and this item), and the merge commit
  brings §4.1's npt-room, npt-server and npt-client rows up to date (T6).

R2 written 2026-11-12.

### R3. The Mediums marked not before the merge: each one's route (labels stand)
New carries start at 73 (u4s-carries.md ends at 72) and at C-U6-12 (plan.md's U6 row ends at C-U6-11). Their words are
in T2 and T3.
- **SEC-U4E-4 (Resync, ack, Resync): the room's half moves INTO the fix pass; the transport's half is C-U6-12.** The
  test seat measured it (Q-6: 29 full states in 30 ticks, 256,418 B, about 8 times the 32,000 B a second the window
  allows; r1-te-pg-q6-q7.log). The room meets the catalogue's words (a second Resync before the ack is dropped), but the
  ack is the client's own claim, and the security-auditor's durable note is the rule: an acknowledgement must never gate
  server work by itself. The room is the rule's home and U4e owns it now; a later unit would reopen npt-room for it. Of
  the two fixes offered I take the rate limit, not counting full states against the window: it is local, it leaves the
  window's arithmetic (e6, -12, -18) as reviewed, and an honest client (a Worker restart, a long hidden tab: catalogue
  :90) never resyncs twice in 2 s. **The rule:** a client's `Resync` is answered only when at least
  `npt_sim::FULL_STATE_ACK_TICKS` ticks (by path) have passed since the tick of the last full state the room sent that
  client (its join's, a forced one or an answered one); a sooner one is dropped and counted under its own name
  (`resyncs_limited`), beside the one-in-flight rule's `resyncs_dropped`. The overflow's forced resync is not limited by
  it (its own bound is carry 57's kick). Test: Q-6's case committed: over the 30 ticks after the join's full state no
  Resync is answered and each is counted; after `FULL_STATE_ACK_TICKS` ticks one is. Plant: the limit removed; the test
  fails by name. This bounds the extra bytes to one full state per 2 s per client. The transport's half (at most one
  open full-state stream per session, the catalogue FullState row's proposal) is U6's (C-U6-12), so SEC-U4E-4 stays
  open in PR #40's report for that half.
- **NC-M4 (events packed before the nearest 8 enemies): carry 78, U4g.** The order holds by reading (room.rs:1164-1180
  before :1181-1237) and breaks the brief's Priority rule ("the nearest 8 enemies in every snapshot": u4e-brief.md:251)
  under a backlog larger than a snapshot's remainder. The test seat's Q-11 shows the gap test never meets it (about 23
  events resent a snapshot, entity changes in every snapshot from F on: r1-te-q11-nearest8.log), and no U4r mark reads
  entity freshness (-17 compares hashes; -18 reads the fresh delta; -57 reads only carried fields). U4g builds
  interpolation and event release on the room's and the client's files, where stale entities would show; it fixes the
  order first, with a test.
- **NC-M5 with A3 (no event base on the wire): carry 75 (U4r) and C-U6-13 (U6).** Q-10 shows it (an endpoint starting at
  0 rebuilds events numbered 200,000 wrong, and the hash reads them: r1-te-p23-q10.log). U4r's lab can avoid it today
  by giving each endpoint the room's numbers at its admission (`with_starts`); the wire field (JoinAccepted or FullState
  carries the room's next 32-bit event number) is npt-proto's and the catalogue's, not U4e's lane, and its first judged
  need is -19's join hash (C-U6-11), so it is U6's.
- **NC-M6 (the endpoint's default lead of 1): carry 74, U4r.** U4r runs -17 at P2, P3 and B1 and -18 at B1 with this
  endpoint before U4f's controller exists; with lead 1 every command is late above about half a tick of one-way delay,
  and -17 would compare a world of soldiers standing still. U4r sets the lead per profile with `with_lead` (which is why
  it stays public: R2, A5) and its contract test shows the commands arrive in time.
- **A6 (no library path to end a session), with SEC-U4E-5: carry 76 (U4r) and C-U6-14 (U6).** The lab ignores
  `replaced` and never closes a kicked session (server/tests/common/mod.rs:238-249), so a replaced session's Inputs keep
  reaching the slot. U4r's lab and U6's TickLoop each write this glue; each carry names the rule and its test. No
  `Decoder::close` now: U4r's lab holds the link and can send `Kick(replaced)` itself and drop the session; U6 adds a
  library item if its TickLoop needs one, with its reason.
- **RR-5 (the room's tick at 48 clients against PM-0001-4's budget): C-U6-15 (U6), and a card follow-up.** PM-0001-4 is
  judged at L2, SS, P2, 48 clients over an S30 (marks.json), which is U6's. The rust-reviewer's bench (three runs:
  room.tick 5.61 / 5.89 / 6.07 ms mean, p99 10.5 / 11.5 / 9.7 ms, the twin's step about 0.2 ms; 2,838 allocations and
  6.20 MB requested a tick, identical across runs; linear in clients: r1-rr-bench.log) puts the room's own work at about
  60-70 % of -4's p99 on this 4-vCPU VM, before the pump and the logs. Not judged at U4e, no regression against a base
  (the room is new), so a carry, with the bench method and the cheap wins named; any unit that edits the encode path
  before U6 (U4f, U4g or U4h) may take them, with the bench before and after.

R3 written 2026-11-12.

### R4. The five Lows marked before the merge: each rides the push the High needs anyway, as report words (T5)
Mustafa's rule holds ("A Low never triggers a push"): none of these would push on its own; the fix pass is needed for
R1 and R2, so they ride it as the report's words and no code.
- **NC-L1, condition 6 (§5 U4e): CONFIRMED, not triggered.** The bite's baseline is not "one the client still holds".
  §4.5 defines holding by age: "The client keeps its complete frames for the same ages [0 to 31], so every B the server
  can pick, the client holds"; the client refuses any `baseline_age` of `ACK_RING_TICKS` or more before it looks at the
  frame (client/src/lib.rs:655-663). That the bytes of B = 329 still sit in ring slot 9 (329 mod 32), because F = 363
  or 362 uses slot 11 or 10 (F − B = 34 or 33, not a multiple of 32), does not make B held: a client that decoded
  against a frame older than the ring would keep baselines the server can never pick, which no rule asks for, and it
  would be the lenient client, not the bitten room, that differs from the contract. So the bite is a defect on L1, and
  -18's L1 part counts at U4r as planned, with no further check of mine. The report states B, F, both ages, both slots,
  and this reading in one line (T5).
- **NC-L3, e7's "at least one resent" carried by gap.rs: ACCEPTED.** On zero-delay Loopback every ack lands before the
  next tick, so a legitimate build resends nothing in e7 (b11-e7print.log: resent 0), the same reason my brief check
  gave for the split clause (its A2). The mirror is compared over resent events in gap.rs's clean run (236 pairs, 0
  differ, 439 and 462 resent). The report says which test carries each clause (T5).
- **NC-L4, B2.1's answer: "decode". CONFIRMED** (R1's case 1). B5's extra reading is not needed (no readings-list entry),
  carry 62 (g)'s drop count does not apply, and `stale()` counts only R1's case 2 (a group of a tick already resolved
  when it left the ring), which is not carry 61 (d)'s drop, so it needs no SERIES.md row; U4r's contract test reports
  its sum (carry 77), 0 in an honest L1 run. The builder's proposed `tick.s.snap_too_old_count` is not added.
- **A7, carry 26: raised now, and the seam stays. ACCEPTED.** The per-tick copy is carry 23's own stamp list (each
  event with its tick and 32-bit number) into a reused `Vec` of `Copy` events (room.rs:772-779); a seam change would
  remove no copy that carry 23 does not need anyway. Carry 26 is met by this raise; the report says so.
- **A14, carry 54's series: the builder's name ACCEPTED, the row in the merge commit.** `event.snap_refused.tick_count`
  follows SERIES.md's form for a per-event tick (`event.counter_wrap.tick_count`, SERIES.md:150). Its row (T4) goes into
  SERIES.md in the merge commit (marks/ is not U4e's lane; check.mjs's marks step then reads it on the merged tree:
  M5), before U4r writes it.

R4 written 2026-11-12.

### R5. The other Lows: each one's route (labels stand; none pushes on its own)
**Ride the fix pass** (each small, in a file the fix pass edits anyway, each with its proof; the security ones close
open security findings in PR #40's report):
- **SEC-U4E-1: the catalogue's rule wins before Join.** Catalogue :93 ("any other type first on the stream closes the
  session as malformed") governs a session in AwaitJoin; §4.10's "no forged kind closes the session" is written for
  joined sessions (the forged suite's twins) and stays so. In AwaitJoin a stream message the decoder refuses as wrong
  channel or wrong direction closes the session as `first_not_join` with `Kick(malformed)`, as a decodable non-Join
  does. Test: Q-3's case committed, asserting the session closed and `first_not_join` 1. Plant: today's count-and-go-on.
- **SEC-U4E-3:** a stream message in the Joining state is counted under its own name (pump.rs:391's `_ => {}`). Test:
  Q-4's case, asserting that counter 1. Plant: `_ => {}` back.
- **SEC-U4E-7:** the lost run's add (client/src/lib.rs:526) never overflows: a sum that would pass `u32::MAX` starts a
  new run. Test: Q-7's case committed, asserting no panic and lost runs that cover the ticks resolved. Plant:
  `last.1 += count` back; the test panics by name (debug build).
- **TE-7** (carry 61 (b)'s "two different hashes for one tick give the sentinel"): a test feeding one group twice with
  different bytes, and one tick completed twice with different hashes, each giving the sentinel. Plant: P-H.
- **TE-8 with RR-2** (carry 50's room half): a room test where a never-acking client's queued event reaches age 256:
  the room drops it from the resend and counts `events_expired`, and no snapshot carries it (brief e12's "an event at
  age 255 is still sent, none at 256", shown for what the room sends); the age cast at room.rs:1170 becomes
  `u8::try_from` with a debug assert. Plant: P-G.
- **RR-3:** a const assert `Limits::GROUPS <= 8` beside the u8 group masks (room/src/ack.rs, client/src/lib.rs).
  Compile-time; no plant.
- **RR-10:** carry 23's unit test asserts that it checked at least one queued event (P-23 showed the half is not
  vacuous today; the assert keeps it so).

**Carries** (T2, T3):
- **SEC-U4E-2** (no rate limit on the control stream): C-U6-12, with SEC-U4E-4's transport half.
- **SEC-U4E-5:** with A6 (carry 76, C-U6-14).
- **SEC-U4E-8** (option B reruns a firing command with its old view time): carry 81, U4h.
- **SEC-U4E-9 with RR-7 and TE-9** (the canonical sort under option A): carry 80, U4f.
- **NC-L7** (a refused datagram whose tick reads far ahead moves `newest`) and **NC-L8** (an `Input` dropped whole when
  one command fails to encode): carry 80, U4f, the next unit to edit npt-client.
- **NC-L9, NC-L10, NC-L2** (U4r's readings of the room: one wrap record per counter name; carry 62 (h)'s setup window;
  `burst_eviction` 1 at age exactly 32 while the bitten room heals there): carry 77, U4r. NC-L10 changes my own words
  in carry 62 (h): its window "all" becomes the measured phase (each join's ticks between the full state and its ack
  are unsent, so the client resolves them lost by design); a non-zero sum in the measured phase still comes to me.
- **NC-L12** (-62's own cosmetic count per client): carry 79, U4g.
- **RR-4, A10, A12:** into RR-5's list of cheap wins (C-U6-15).

**The report's words** (T5):
- **NC-L5 = RR-1:** §4.2's "the debug assert that a correct room never sends one" is met by construction: `rewind_at`
  returns `None` before it builds a Rewind naming an unheld tick (rewind.rs:18-33), so the assert at :28-31 can never
  fire; it may stay as documentation. (RR-1 cites rewind.rs:73-77; the file has 33 lines.)
- **NC-L11 with A11:** the three values the plan leaves open are module constants: the interest radius 64 m and its
  cell (the plan is silent on both), and the hit history's 2 s (provisional until U4h: carry 28). The report names them
  as U4e's choices. A setter or getter comes with the unit that first needs one (U4h for the history; the radius per
  mode at slice 3, ADR 0001 part 9 (a)).
- **SEC-U4E-6 = A13 = NC-L11's second half:** a Hit or Kill reaches a client that sees only the victim with the
  shooter's slot (room.rs:954-968), against ADR 0001 part 9 (f) ("no shooter id when the shooter is not relevant to the
  viewer"); the wire has no form without it (U4a's), and part 9 is proven at slice 3 (ADR 0001:1687-1688). A known
  deviation until then; PR #40's report lists it as an open security finding (Low).
- **RR-8:** the six types public in name only are plan §4.7's list (u4-build-plan.md §4.1 base); A5's report list says
  so.
- **TE-10:** the endpoint's 1,150 B case is refused by its trailing bytes too; the size check's own proof is npt-proto's
  `a3_a_snapshot_is_taken_up_to_1_150_b_and_sent_up_to_1_000_b` (proto/src/msg.rs:587 on: 1,001 B and 1,150 B taken,
  1,151 B refused `TooLarge` by its size, :607-622). No code.

**Follow-ups on the card** (open findings, each with its file:line; no carry, no push): **NC-L6 = RR-9** (a carry-54
skip has already moved `sent`, the accumulator and `cosmetic_sent`: room.rs:1166-1180, :1226-1240, :1288-1291; the
next encoder edit moves them after the encode succeeds); **RR-6** (the room's full-state deadline and ring use the
30 Hz constants while `Room::new` takes `tick_hz`: room.rs:1003, :1015, ack.rs:76-77; the first unit to run a room at
another rate takes their `_at` forms by carry 39's route); **A8** (`Room::new` takes the scene and greybox beside the
sim unchecked: room.rs:408-415); **A9** (wire knowledge outside npt-proto: the client's raw tick read and the two 28 B
overheads, unpinned; the next npt-proto change gains a header-tick read and the overhead in `Limits`).

R5 written 2026-11-12.

### R6. The test-engineer's findings (REVISE): accepted at their labels; the five Mediums in the fix pass
The ids are its Block 5's, which supersede the ids written inline in its Block 4 (it says so). I read its logs for the
results each finding rests on: Q-1, Q-2, Q-6, Q-7, Q-9 and Q-10's printed lines, P-A's, P-B's and P-D''s exit 0 with
every binary ok, P-C's single failure (its own probe), P-21/22/57's two carry-22 failures, and the counted run's "check:
all 10 steps pass" with 458 and 524 (r1-te-*.log under .gate/u4e/). Each plant was restored and the copy's 23 files
equal the start list (I re-ran `sha256sum -c` in both copies: 23 of 23 OK).
- **TE-1 (High):** R1. Its probe Q-1 is what settles NC-H1.
- **TE-2 (e6's remainder never binds; P-A passes): fix.** An e6 case where the pump's bytes change what is sent: on one
  tick, the same room state with pump bytes p large enough that the snapshot must shrink (48 SS clients, or a large
  `pump_report`) sends at most budget − p, while with p = 0 it sends more than budget − p. Plant: P-A
  (`left = c.t.budget`); the case fails by name. The plan's "the window counting the pump's bytes" is then proven, not
  only read back.
- **TE-3 (client-rounding's call site in the endpoint untested; P-B passes): fix.** A bites-only twin feeding one
  decoded snapshot to a clean endpoint and to one built `with_bite("PM-0001-17/client-rounding")`: the clean one files
  the mirror's hash, the bitten one a different hash. Plant: P-B; the twin fails by name. That meets §5 U4e's
  "client-rounding (the client's call site)" and carry 24's first clause.
- **TE-4 (group 1 without group 0 not shown; P-D' passes): fix.** e2 (or e3) also drops group 0 of one split tick:
  every other group of it decodes and is acknowledged (the ack's group mask without bit 0), and the tick is resolved
  lost, never the sentinel. Plant: P-D'; the case fails by name (P-D, the in-order form, already fails e2).
- **TE-5 (the pump's size check untested; P-C passes): fix.** Q-5's case committed (an `Input` datagram of
  `Limits::INPUT + 1` B: `too_large` 1, nothing passed on), and one control-stream message one byte over its channel's
  cap: `too_large` 1 and the session closed as malformed (pump.rs:341-347). Plant: P-C; both fail by name. That is
  U4E-Q11 (a)'s "size ... and their unit tests".
- **TE-6 (carry 57's second-overflow kick cannot fail; P-57 passes): fix.** The test asserts that the kick's tick is
  below the first overflow resync's tick + `FULL_STATE_ACK_TICKS` + 1 (Q-9: 459 against 495), so the timeout cannot
  stand in for it. Plant: P-57; the test fails by name.
- **TE-7, TE-8:** ride the fix pass (R5). **TE-9:** carry 80 (U4f). **TE-10:** the report's words (R5).
- **Its settling runs hold** as it reports: NC-H1 (Q-1), NC-M3 (Q-2), SEC-U4E-1 (Q-3), SEC-U4E-3 (Q-4), SEC-U4E-4 (Q-6),
  SEC-U4E-7 (Q-7), NC-M5 (Q-10), NC-M4 in part (Q-11: the order yes, the figures no). Each becomes, or is the model
  for, the committed test named in R2, R3 and R5.
- **Its counted run** (r1-te-check-counted.log): all 10 steps PASS, measured 458, bites 524, self 52 of 52, equal to the
  builder's two (b11-counted1.log, b11-counted2.log). Its empty provenance block is a fact of check.mjs, which keeps
  cargo's Compiling lines out of its output; for a review seat's run the pwd line (the copy) and the touch before it are
  enough. The merge's counted run keeps the builder's form: separate `--no-run` builds whose Compiling lines name the
  main folder (M5).

R6 written 2026-11-12.

### R7. The main session's points (the card, "Running")
- **Carry 68 missing from the builder's carries met, though carry 56's ES figure is reported: CONFIRMED** (NC-M2 is the
  same point). Route: R2, the report's line T5 (c); carry 68 then counts as met.
- **Carry 61's stale-group choice ("decode within the ring, drop and count beyond it"): CONFIRMED, with R1's
  correction.** Within the ring: decode (NC-L4). Beyond it, two cases: a tick from the client's first snapshot on was
  already resolved when it left the ring, so its group is dropped and counted (`stale`); a tick before the first
  snapshot is a bad header and gets the sentinel (R1, case 3). B5's extra reading does not apply (R4).
- **The builder's own points:**
  - e7's split and resend clauses: CONFIRMED as carried by e2 (my brief check's A2) and by gap.rs's clean run (R4,
    NC-L3).
  - -18's baseline still in memory: condition 6 is not triggered (R4, NC-L1).
  - Owner-to-all writes the next slot's records into each client's own-state section: CONFIRMED. That is my brief
    check's A3 ("one other slot's own-state records ... within the section's 8-bit count ... so that every bitten
    snapshot still encodes"), and s15 fails carry 53's per-record test by name ("tick 1050: client 0's full state holds
    Soldier 1 gen 0").
  - Carry 26's copy: raised now and accepted (R4, A7).
  - Its proposed series: `event.snap_refused.tick_count` for carry 54 ACCEPTED (R4, A14; row T4);
    `tick.s.snap_too_old_count` for carry 61 NOT added (R4, NC-L4).
  - `Kick(replaced)` left to the caller: ACCEPTED for U4e (the room's doc says so: room.rs:601-606), with the caller's
    rule and its tests carried to U4r and U6 (R3: carries 76 and C-U6-14).
- **Whether the resumed test seat counts with the other four as one round: YES.** A round is one review of one head by
  all the change-set's reviewers, in batches of at most three (CLAUDE.md's review budget; my role file's "Review
  budget"). The head did not change across the cut: the main session restored plant P-D by Edit (client/src/lib.rs
  sha256 = start; the copy's crates/ equal .gate/u4e-r1's; the re-cut patch equal to wip/u4e.patch), the resumed seat
  found 23 of 23 files equal to its start list, recorded its two earlier plants' diffs afresh and re-ran them in its own
  form, and ended at 23 of 23; I re-checked both copies now (23 of 23 each). The cut is a process event, not a second
  review. Round 1 is complete; round 2 on what changed is the last ordinary round, and a third runs only for a Critical
  or High still open after it.

R7 written 2026-11-12.

