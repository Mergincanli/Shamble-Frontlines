# Advisor: U4e round 1 (the room and the codec path), 2026-11-12

Seat: the advisor agent, called from the day session's workflow (branch claude/netcode-proof-test-plan-hcky5g; PR
#40). Role read whole from main (`git show origin/main:.claude/agents/advisor.md`). I rule; the main session applies
every text change, and one builder runs the fix pass in the copy. I write only this file, with Write and Edit, block by
block. Read only: no build, no run, no plant; reading, git and sha256 only.

**Verdict: REVISE, then merge on conditions.** Round 1 stands as one round. The High (NC-H1 = TE-1) is real by reading
and by run: the tick16 bites' unit proof passes on a silent drop, and a judged run would read 0 mismatches. R1 rules the
client's reading (a header naming a tick before the client's first snapshot, at least the ring behind its newest, is a
bad header: the sentinel under that tick) and what the proof asserts (what the judge counts). The six Mediums marked
before the merge and the test seat's five Mediums go into one fix pass, with SEC-U4E-4's room half moved in; seven
Lows ride it; every other finding is routed (carries 73-81, C-U6-12 to C-U6-15, report words, card follow-ups). Round 2
on what changed, five seats in two batches; the merge after it on M1-M9. Nothing goes to Mustafa: no word or decision
of his changes, and nothing a player sees, hears or feels is touched.

## Plan
1. Read the brief (advisor-u4e-r1-brief.md), the five seats' notes, the builder's record, my brief check
   (advisor-u4e-brief.md), U4d-F's ruling (M1-M9's form), U4j round 2's B2.1 and B5, the plan's §4.5, §4.6, §4.9,
   §4.14 and §5 U4e. (done)
2. Spot-check the code the rulings rest on, in .gate/u4e-r1 (the read copy): the client's stale drop and the sentinel
   (NC-H1, TE-1), the pump's RTT (NC-M3, A2), `take_control` (A1), `tick_input` (A4), the Resync rule (SEC-U4E-4), the
   tick16 twin; the copy against the snapshot (sha256); carries 61, 62, 68 and the card's "Running".
3. Rule items 1-9 in order, one block each (R1 ... R9), then the texts, the two lists, and what I checked.

## Progress
- [x] 1 read  - [x] 2 spot-checks  - [x] R1  - [x] R2  - [x] R3  - [x] R4  - [x] R5  - [x] R6  - [x] R7  - [x] R8
- [x] R9  - [x] texts (T1-T6)  - [x] end sections. DONE.

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
the client was sent and can still place. A header tick falls in one of three cases:
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
client_hash` (its name may change to say what it now shows) and its clean twin, over `tick16_lab`'s 30 ticks:
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
test runs one short tick16 bite run past the wrap and shows the figure FAIL with mismatches ≥ 1, never INVALID through
a `crash` footer.

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
  same point). Route: R2, T5's own-state bullet; carry 68 then counts as met.
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

### R8. The fix pass: its scope, its builder, and round 2's seats. APPROVE on these terms.
**Scope (each item with its test and its named plant; nothing else):**
- FX-1 R1 (NC-H1, TE-1): the client's case 3 and the tick16 twin's assertions; plants (a) and (b).
- FX-2 NC-M3; FX-3 A1 (with the lab's send in server/tests/common/mod.rs); FX-4 A2 (`RttSource`); FX-5 A4; FX-6 A5 (the
  eight items pub(crate), `with_cover`'s test); FX-7 SEC-U4E-4's room half (the Resync limit). All as R2 and R3 say.
- FX-8 TE-2, FX-9 TE-3, FX-10 TE-4, FX-11 TE-5, FX-12 TE-6, as R6 says.
- FX-13 the riders: SEC-U4E-1, SEC-U4E-3, SEC-U4E-7, TE-7, TE-8 with RR-2, RR-3, RR-10, as R5 says.
- Not the builder's: the report, marks/, the plan files and the carries (the main session's, from T1-T6).
- **Bounds:** files only under crates/room, crates/server (not schedule.rs) and crates/client; no manifest, no
  Cargo.lock, no npt-proto, npt-sim, npt-link or npt-lab file. One new public item, `RttSource`, and `take_control`'s
  changed type; any other public item, any change to the window's arithmetic, or a test that cannot be made to fail
  under its plant stops that item: the builder reports it, and it comes to me.

**Who builds:** one builder (general-purpose, Opus, effort xhigh), in the same worktree
(`.claude/worktrees/wf_0fa6595a-3e2-1`, uncommitted on 97bb9d3), reading its record (u4e-build.md), this ruling's R1-R6
and the copy's diff first, as U4d-F's fix pass did; under W1-TARGET's lock (the lane lock held for each cargo run, the
shared target, offline), counted in the three-agent limit. Its brief carries, word for word, my U4d-F ruling's T4:
item 1 (the write rule: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes;
never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole
with Write.") and item 2 with the crates named (npt-room, npt-server and npt-client: "a failure in npt-room,
npt-server or npt-client, or in a test that reads their files, stops your step group (report it), unless your step's
plan names it as expected …"). Its plan is written first into the record's new section "Fix pass, round 1", then a
notes line after each FX.

**Its evidence** (in the record): for each FX the plant's diff, the run (`-x`, by name), the test FAILED by name, the
restore by Edit and the file's sha256 equal to its fixed form; s03/s04 re-run once (R1 plant (b)); fmt (rustfmt's diff
applied by Edit), clippy in both builds and for wasm32, doc; check.mjs twice counted in the copy, all 10 steps, with
the counts as deltas on 458 / 524 / 52 and each new test named; the printed lines of e7, the tick16 twin and its clean
test, and carry 56's test (for T5); and at the end the sha256 of every changed or new file under crates/, in
`sha256sum` form with paths from the copy's root, saved as .gate/u4e/fx-hashes-end.sha256.

**Then the main session:** re-cuts the snapshot by worktree_patch.py (wip/u4e.patch replaced, committed), checks that
the copy's files equal fx-hashes-end.sha256, and cuts round 2's packet by review_packet.py on what changed (round 1's
snapshot at db49108 against the new one), with this ruling as the "judged against" half.

**Round 2's seats, on what changed** (Opus at xhigh; condition 10 (c)'s limit: two batches; each brief carries T4
item 1):
- Batch 1, read only: **netcode-architect** (R1's cases and the twin; NC-M3; A2; the Resync limit; SEC-U4E-1),
  **security-auditor** (the pump's changes: NC-M3, A2, SEC-U4E-1 and -3, TE-5; the client's decode path: R1's case 3
  and SEC-U4E-7; the Resync limit; A1's bytes), **engine-architect** (A1, A2, A4, A5, and T6's §4.1 rows against the
  final diff).
- Batch 2: **test-engineer** (with Edit, in the copy: each FX's test fails under its plant, its own form where it doubts
  the builder's; its start hashes first, as .gate/u4e/r2-hashes-start.sha256, then one counted check.mjs), beside the
  **rust-reviewer** (read only, no bench: panics, casts and overflow in the changed code, R1's bounded memory, RR-2 and
  RR-3).
- **Round 2's outcome:** APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead on M1-M9 (its Lows
  become follow-ups the main session records). A Medium comes to me first and becomes a follow-up or a carry, never a
  third round. A High or a Critical comes to me for a third round on it alone (the review budget's one exception); one
  still open after that stops U4e for Mustafa.

R8 written 2026-11-12.

### R9. The merge into the spike branch: after round 2, on conditions M1 to M9 (U4d-F's form)
Checked now: the spike branch's spikes/ equals 97bb9d3's (`git diff --quiet 97bb9d3 HEAD -- spikes/`: true, at
3032b88), and 97bb9d3's spikes/ equals `<base>` 28a85e7's; worktree_patch.py and main-run.sh are in the notes folder.
U4d2's lane (check.mjs, Cargo.toml's members, Cargo.lock, crates/hash-wasm, npt-lab's `hash`) touches no U4e file, but
if it merges first the base counts move.

The conditions (the main session; each result in u4e-build.md's new section "Merge into the spike branch"):
- **M1, the copy is the one reviewed.** Before the snapshot: the copy's HEAD is still 97bb9d32aa7b…; `git status
  --untracked-files=all` shows only round 2's files under crates/room, crates/server and crates/client and the record;
  `sha256sum -c .gate/u4e/r2-hashes-start.sha256` from the copy's root gives all OK. Anything else stops the merge and
  comes to me.
- **M2, the base still fits.** On the spike branch's head, `git diff --quiet 97bb9d3 HEAD --
  spikes/netcode-proof/crates/room spikes/netcode-proof/crates/server spikes/netcode-proof/crates/client
  spikes/netcode-proof/crates/sim spikes/netcode-proof/crates/proto spikes/netcode-proof/crates/link` is true. Any other
  spikes/ change since 97bb9d3 is a named lane's merge (U4d2's), listed in the notes with its counts; anything else
  stops the merge.
- **M3, the snapshot and the apply.** The saver stopped first. worktree_patch.py on the copy gives a patch equal (cmp)
  to the committed wip/u4e.patch that round 2 reviewed, listing exactly round 2's files; then `git apply --check`, then
  `git apply`, in the main folder.
- **M4, the merged bytes are the reviewed bytes.** From the main folder's root, `sha256sum -c` of round 2's start
  hashes gives all OK. A patch that applies but differs stops the merge.
- **M5, the texts, then check.mjs counted on the merged tree.** First T1 (marks/README.md §1) and T4 (SERIES.md),
  applied by script as M8 says, since check.mjs's marks step reads them. Then check.mjs through main-run.sh (the lane
  lock held exclusively and idle at the start, the 2 s touch loop, the shared target, offline), its log under
  .gate/u4e-merge/, after the pre-run builds `cargo test --locked --workspace --no-run` and the same with
  `--all-features`, whose "Compiling npt-room", "npt-server" and "npt-client" lines name the main folder's path. All 10
  steps pass, with measured and bites counts equal to round 2's counted run in the copy (458 and 524 plus the fix
  pass's new tests; plus U4d2's deltas if it merged first, the sum shown), self 52 of 52 (or U4d2's), marks 81 read and
  checked with the ADR rows unchanged, redact 0 findings. In the same main-run.sh call, by name with `--nocapture`: e7
  (its print line equal to round 2's), e2, e5a and 2.5's clean test, the tick16 twin and its clean test (the twin's
  orphan count at least 1 for each id, the clean compare 0 bad), and carry 56's test (its SS and ES figures): T5's
  numbers come from these lines. Any other count or line is explained in the notes before the commit.
- **M6, a failing test is never retried into green unnamed** (TE2-5's rule): a failing build's crate is re-run at once
  through main-run.sh with `--no-fail-fast`, its whole output under .gate/u4e-merge/, the failing test's name and panic
  in the notes. A failure in npt-room, npt-server or npt-client, or in a crate that links them, stops the merge and
  comes to me; any other failure is recorded under TE2-5, and the merge goes ahead only on a clean counted run, the
  earlier failure named.
- **M7, the gate:** `node tools/gate.mjs` before the commit; if smoke (5a) cannot start this container's Chromium,
  `--no-smoke` with both logs kept and the reason in the notes. CI's gate and smoke green on the pushed head.
- **M8, one commit,** "U4e: the room and the codec path (npt-room, npt-server's library, npt-client), merged after review
  round 2": the spikes/ files exactly as applied, with T1 and T4 in marks/; `git rm` of wip/u4e.patch; and under
  .checkpoints/ only: u4e-build.md (with "Fix pass, round 1" and "Merge into the spike branch"), u4e-report.md (new; T5
  is its rulings section, the rest in u4df-report.md's form: who built it, the rounds, the checks on the merged tree,
  the carries met and added by number, the public items beyond §4.1 with reasons, the follow-ups, Decisions needed, For
  Mustafa), u4-build-plan.md's §4.1 rows (T6), u4s-carries.md (T2), plan.md's U6 row (T3), and the card. Every text is
  copied by script from this file's code blocks, never retyped: each old text found exactly once (line breaks read as
  spaces), each new text found in place after, no changed prose line over 120 characters. The commit's spikes/ part is
  the reviewed bytes plus T1 and T4 and nothing else.
- **M9, after the push:** the saver restarted; the card updated (U4e merged at <sha>; the base counts for U4r and for
  U4d2's merge; carries 73-81 and C-U6-12 to C-U6-15 listed; U4e's open security findings for PR #40's report:
  SEC-U4E-2, SEC-U4E-4's transport half, SEC-U4E-5, SEC-U4E-6, SEC-U4E-8 and SEC-U4E-9, each Low but SEC-U4E-4
  (Medium), each with its carry); the copies U4e used (.claude/worktrees/wf_0fa6595a-3e2-1, .gate/u4e-r1,
  .gate/u4e-prefix) removed per CHECKPOINTS.md §10 once CI is green; U4r's brief names carries 73-77 and 62 (h)'s new
  window.

R9 written 2026-11-12.

## The texts (each copied by script from its code block; never retyped)

### T1. marks/README.md §1, the `decoded_hash` item (the merge commit)
The old text `that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel.` (found once, line breaks
read as spaces; checked at the branch head) becomes the text below, rewrapped to the item's indent within 120:
```
that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel. That covers the ticks from the client's first snapshot on: a snapshot group or full state whose header names a tick before the client's first snapshot and at least `npt_sim::ACK_RING_TICKS` ticks behind the newest tick it holds names a tick it was never sent (a bad header), so the client files the sentinel under that tick, once, never acknowledged, and `decoded_hash_mismatch_count` counts it as a client hash at a tick the server never logged (the advisor's U4e round-1 ruling, R1).
```

### T2. u4s-carries.md (the merge commit)
**(1) §U4r, after entry 69 (its last):**
```
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
```
**(2) §U4r, entry 62 (h):** the old text `"window": "all", "op": "==", "value": 0, "parts": ["L1 P0"], "setup": true}`
(found once) becomes `"window": "measured", "op": "==", "value": 0, "parts": ["L1 P0"], "setup": true}`; and after the old
text `If it is not 0, the cause goes to the advisor first.` (found once) this is inserted, before the rest of the entry:
```
(Both the reported sum and the check read the measured phase: each join's ticks between its full state and that full state's acknowledgement carry no snapshot, so the client resolves them lost by design in the warm-up: the advisor's U4e round-1 ruling, R5, NC-L10.)
```
**(3) §U4f, after entry 46 (its last):**
```
80. **U4e's round-1 notes for option A and the client (u4e-review-rust.md, RR-7; u4e-review-security.md, SEC-U4E-9;
    u4e-review-test.md, TE-9; u4e-review-netcode.md, NC-L7 and NC-L8; the advisor's U4e round-1 ruling, R5):** "(a) The
    room's canonical order (`canonical`, crates/room/src/room.rs) compares one slot's ticks wrap-aware, which is no
    total order for ticks 2^31 or more apart, and Rust's sorts may panic on a non-total order; under option B one
    command per slot per tick makes it unreachable, and carry 21's room test passes with no sort at all (TE-9). Before
    option A runs several commands of one slot in a tick, U4f's lead bound refuses such a command first, or the sort
    takes a total key, with a test of one slot's commands 2^31 ticks apart, and carry 21's room test is made to fail
    without the sort. (b) A refused snapshot datagram whose header tick reads far ahead moves the client's newest tick
    and resolves the whole span lost (crates/client/src/lib.rs, `receive` and `see`): the client files its sentinel
    without moving its newest tick past a bound, with a test. (c) `ClientEndpoint::step` drops a whole `Input`, acks
    included and uncounted, when one command fails to encode: it drops that command, counts it and keeps the acks,
    with a test."
```
**(4) §U4g, after entry 70 (its last):**
```
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
```
**(5) §U4h, after entry 45 (its last):**
```
81. **SEC-U4E-8, option B's reruns and the view time (u4e-review-security.md; the advisor's U4e round-1 ruling, R5):**
    "Under option B the room reruns a client's last command on a miss with its original view time (crates/room/src/
    room.rs, `fill_input`), so a client that holds fire and stops sending gets each rerun shot rewound to an ever older
    view tick, up to the history's span, and `rewinds_refused` also counts reruns whose view tick has left the ring.
    U4h's tolerance judges each run of a command, reruns included, against the estimate at the tick it runs (or runs a
    rerun with fire released), counts reruns apart from claims, and shows by test that a starved uplink buys no rewind."
```
The file holds no count of its own; the card's line follows ("carries 1-81").

### T3. plan.md's U6 row: U4e's U6 carries (the merge commit)
The old text `since -19 judges ML's Chrome "as it stands" (ADR 0001:1609) |` (found once: the row's end) becomes the
text below (one table line; `<br>•` as the row's other entries):
```
since -19 judges ML's Chrome "as it stands" (ADR 0001:1609);<br>**U4e's carries (the advisor's U4e round-1 ruling, 2026-11-12):**<br>• **C-U6-12 the full-state stream and the control stream (R3 and R5: SEC-U4E-4's transport half, SEC-U4E-2):** at most one open full-state stream per session (the catalogue's FullState row, proposed there), npt-server refusing a client-opened one beyond it; and a per-connection limit on control-stream messages (`ClientSettings`, `Resync`, `SessionControl`), counted, a message over it dropped or the session closed as malformed, with the catalogue's rows gaining the limit; the room's half of SEC-U4E-4 is U4e's (a client's Resync answered at most once per `FULL_STATE_ACK_TICKS` ticks);<br>• **C-U6-13 the event base on the wire (R3: NC-M5, A3):** a join or rejoin starts the client's 32-bit event numbers from the room's: `JoinAccepted` or `FullState` carries the room's next event number (npt-proto and the catalogue), the client rebuilding from it; -19's join hash (C-U6-11) is the first judged case, since today a client admitted after the room has stamped 32,768 or more events past its start rebuilds every event number wrong (U4e's probe Q-10; u4s-carries.md entry 75 is U4r's stopgap);<br>• **C-U6-14 ending a session (R3: A6, SEC-U4E-5):** TickLoop, when `Room::join` reports a replacement, sends the old connection `Kick(replaced)` and closes it before routing anything else, and closes a kicked or left session; either the room checks a session generation on `receive` and `leave` (returned by `Room::join`, a stale one dropped and counted) or U6's report shows by test that no message of a closed session reaches the room; npt-server's library gains a call that ends a session only if TickLoop needs one, with its reason;<br>• **C-U6-15 the room's tick before -4 (R3: RR-5, with RR-4, A10 and A12):** before PM-0001-4's judged runs, the room's own tick at SS with 48 clients is measured on R2 by the rust-reviewer's U4e method (a counting allocator around `Room::tick` only; a twin World stepped on `tick_input()` with its state hash asserted equal; 100 warm and 600 measured ticks; release; three runs) and set beside -4's 16.67 ms p99 with the pump and the logs; U4e's cheap wins are made first, each with that bench before and after: the relevant set emitted in the index's key order instead of a sort by key through the records, no clone of the Packer's vectors per group, no fresh baseline-frame clone per client per tick, no per-client vectors built each tick, no double clone of a tick's baseline frame in the client, a linear acknowledgement pass over the event queue, and no per-tick copy of the sim's reports (`Room::reports`); at U4e, on a 4-vCPU VM, the room's own work was 5.4-5.9 ms mean and 9.5-11.2 ms p99, with 2,838 allocations and 6.2 MB requested a tick, linear in clients (u4e-review-rust.md, block 2) |
```

### T4. marks/SERIES.md, carry 54's row (the merge commit)
In the table that holds `event.counter_wrap.tick_count`, as a new line right before the row named
`event.soak_trigger.bad_token_refusals_count` (one such row, SERIES.md:218 at the branch head; the rows run in name
order, and `event.slow_loris_close.age_ns` is the row above):
```
| `event.snap_refused.tick_count` | count (a tick number) | npt-lab (from npt-room's per-client refusal counters) | per snapshot the room's encoder refused for a client (carry 54: that client's snapshot for that tick is skipped; `slot` names the client, `kind` the refusal: `out_of_range`, `too_many`, `too_large` or `not_canonical`): the tick it skipped | -17, -18 (reported) |
```

### T5. u4e-report.md's section on the review's rulings (new file, beside u4df-report.md)
Copied by script as it is. Each `<…>` is a number the main session fills from the merged tree's M5 lines (or, where it
says so, from round 2's notes), and the notes name the log and line of each fill; no other word changes.
```
## What U4e's review settled (the advisor's U4e round-1 ruling, advisor-u4e-r1.md)
- **Round 1** (five seats on one head, 97bb9d3 plus the snapshot of db49108; the test seat was cut by the spend limit
  at about 06:42 UTC and resumed at about 07:36 on the same bytes, so one round): netcode-architect and
  engine-architect REVISE, test-engineer REVISE, security-auditor and rust-reviewer APPROVE WITH FINDINGS; two Highs
  (NC-H1 and TE-1, one defect). One fix pass, then round 2 on what changed: <round 2's verdicts, from its notes>.
- **tick16's proof (NC-H1, TE-1; High; fixed).** Under PM-0001-17/tick16 and PM-0001-58/tick16 the client dropped
  every post-wrap group as too old and filed nothing, so the twin passed on a silent drop and a judged run would have
  read 0 mismatches. Since the fix the client files the sentinel when a header names a tick before its first snapshot
  and at least `npt_sim::ACK_RING_TICKS` ticks behind its newest (a bad header: marks/README.md §1), and the twin
  asserts what the judge counts: under each bite <n> client hashes at ticks the room never logged for that slot and
  none at or past 65,536; in the clean build 0 bad of <n> compared, the hash at 65,537 equal to the mirror's. U4r's
  judged half: carry 73.
- **The client's late groups (carry 61 (d); B2.1): "decode".** A group of a tick still in the client's frame ring is
  decoded and hashed under its tick. A group of a tick from the client's first snapshot on that has left the ring
  (already resolved) is dropped, never acknowledged, and counted in `ClientEndpoint::stale()`, which is not carry 61
  (d)'s drop and has no SERIES.md row (U4r reports its sum: carry 77). The readings list gains no entry, and carry 62
  (g)'s drop count does not apply.
- **-18's condition 6 (§5 U4e; NC-L1): not triggered.** Under no-fallback the room codes F = <363> against B = <329> at
  age <34> (AfterSend) and F = <362> at age <33> (BeforeSend). B's bytes are still in the client's ring slot <9> (F uses
  slot <11> or <10>), but the client refuses a baseline age of `ACK_RING_TICKS` or more before it looks at the frame:
  §4.5's "the client keeps its complete frames for the same ages". B is not held, so the bite is a defect on L1, and
  -18's L1 part counts at U4r.
- **The own-state section (carry 56, with carry 68).** SS: <mean> B mean, <p99> B p99, <max> B max per snapshot, the
  OwnState record <mean> B, changed in <share> of snapshots; ES: <mean> B, <p99> B, <max> B, OwnState <mean> B, changed
  in <share>. The SS mean is above R2's 16 B a snapshot for the local player's own state (r2-netcode.md:1756): reported
  beside it, not judged (PM-0001-12 judges the downstream total). ES's figure comes from `Recording::synthetic`'s ES
  recording, whose alive soldiers change yaw on about 48 % of soldier-ticks and pitch on about 18 %, against R2's
  moving-delta assumption of yaw in every moving update and pitch half the time (docs/research/r2-netcode.md:1751);
  the own Soldier record carries both, so ES's figure reads lighter than R2's estimate and is no measure of ES's load
  (carry 68).
- **e7's clauses (NC-L3).** On zero-delay Loopback no tick splits and no event is resent (e7: <its print line>). The
  split clause is carried by e2 (300 B links: <n> split ticks, <n> compared equal) and the resend clause by gap.rs's
  clean run (<n> pairs, 0 differ, <n> and <n> events resent).
- **Carry 26 (A7).** The room copies each step's events once a tick into its stamp list (carry 23's tick and 32-bit
  number), a reused vector of `Copy` events; raised with the advisor, who keeps the seam as it stands.
- **Carry 54's series (A14):** `event.snap_refused.tick_count`, its row added to SERIES.md in the merge commit.
- **§4.2's rewind assert (NC-L5, RR-1):** met by construction: `rewind_at` returns `None` before it builds a Rewind
  that names an unheld tick, so its debug assert cannot fire.
- **Values the plan leaves open (NC-L11, A11):** the interest radius (64 m) and its cell, on which the plan is silent,
  and the hit history's 2 s (provisional until U4h: carry 28) are module constants, U4e's choices; a setter or getter
  comes with the unit that first needs one.
- **The RTT (A2).** The pump takes each session's RTT from its `RttSource` (§4.9). The room's per-tick report from the
  pump (`pump_report`) carries the bytes sent and the link's largest datagram, not the RTT, which U4h's tolerance will
  read; the builder's record (u4e-build.md, design decisions) says otherwise.
- **A known deviation (SEC-U4E-6, A13).** A Hit or Kill reaches a client that sees only the victim with the shooter's
  slot, against ADR 0001 part 9 (f) ("no shooter id when the shooter is not relevant to the viewer"); the wire has no
  form without it, and part 9 is proven at slice 3 (ADR 0001:1687-1688).
- **The 1,150 B case (TE-10).** The endpoint's test datagram is refused by its trailing bytes too; the size check's own
  proof is npt-proto's a3 test (1,151 B refused by its size).
- **Public items beyond §4.1's rows** (A5, RR-8): <the list re-derived from the merged tree for T6, each with its
  reason>. Made pub(crate) in the fix pass: `Roster::new`, `Roster::is_bound`, `Decoder::limits`, `Decoder::is_joined`,
  `ConnLimits::join_deadline_ns`, `ClientEndpoint::closed`, `ClientEndpoint::unknown_pongs`, `ClientSlot::skipped`.
  `ClientEndpoint::with_lead` stays public for U4r (carry 74). The six types plan §4.7 lists whose methods are all
  pub(crate) (`AckRing`, `Encoder`, `EventQueue`, `InterestGrid`, `AreaFlood`, `Priority`) are public in name only.
- **Open security findings, for PR #40's report** (each left open keeps it needs-mustafa): SEC-U4E-4's transport half
  (Medium; C-U6-12), SEC-U4E-2 (Low; C-U6-12), SEC-U4E-5 (Low; carry 76 and C-U6-14), SEC-U4E-6 (Low; slice 3),
  SEC-U4E-8 (Low; carry 81), SEC-U4E-9 (Low; carry 80). Fixed in U4e: SEC-U4E-4's room half, SEC-U4E-1, SEC-U4E-3 and
  SEC-U4E-7.
```

### T6. u4-build-plan.md §4.1: npt-room's, npt-server's and npt-client's rows (the merge commit)
First the main session re-derives the list from the merged tree: `git diff 28a85e7 HEAD -- spikes/netcode-proof/crates/
room/src spikes/netcode-proof/crates/server/src spikes/netcode-proof/crates/client/src`, every added `pub` item and every
public method of a public type, less plan.md §4.7's own items (npt-room's eleven; npt-server's `Counters`,
`ConnLimits`, `Decoder`; npt-client's `ClientEndpoint`, `StepOut`). The rows below are written from the
engine-architect's list (u4e-review-arch.md:57-66) with this ruling's changes; if the merged list differs, the rows
follow the merged list and the notes say why, and round 2's engine-architect checks them against the final diff.
- The npt-room row, old (one line):
```text
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP`; bites builds only: `BITES` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling) |
```
- New:
```text
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP` and `buffer_depth_ticks`; `ClientCounters` (the type `ClientSlot::counters` returns, its fields the per-client counts); `Room`'s methods (`new`, `join`, `leave`, `receive`, `pump_report`, `tick`, `take_datagrams`, `take_control` handing out each reliable message's bytes, `client`, `last_tick`, `sim`, `roster`, `script`, `tick_input`, `reports`, `event_cap`, `with_event_cap`, `with_event_start`, `with_hashes`, `with_cover`); `ClientSlot`'s getters (`slot`, `counters`, `server_hash`, `burst_eviction`, `baseline_age`, `groups`, `bytes`, `budget`, `last_acked`, `own_section`, `overflow_resync`, `wraps`, `rewinds_refused`, `queue`, `headroom`, `buffer_depth_ticks`, `kicked`); `Roster::live` and `is_live`; bites builds only: `BITES` and `Room::with_bite` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling); §4.2 (`script`, `reports`), §4.5 (`burst_eviction`, the window's pair), §4.9 (`with_cover`), §4.11 (`with_bite`); U4E-Q6, U4E-Q7, U4E-Q11, U4E-Q12; carries 23, 40, 54, 56, 57 and 61 (a); plan.md:508-509 (`take_control`: encoding stays on the tick); the advisor's U4e round-1 ruling, R2 (A1, A4, A5) |
```
- The npt-server row keeps its start; its end, from the words "bites builds only:" (once in that row) to the row's
  closing bar, old:
```text
bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling |
```
- New (so the row reads its old items, then these):
```text
`Decoder`'s methods (`new` taking an `RttSource`, `counters`, `is_closed`, `accept`, `reject`, `take_sent_bytes`, `rtt_ns`); `RttSource` (`Link`, `Pings`); `ConnLimits::new`, `input_refill_per_second`, `hold_cap_ns`; `Counters`' fields; the modules private, each item at the crate root; bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling; §4.9 (`RttSource`: the RTT by the session's kind), §4.10 (`Counters`, U4f's -21 series); carry 39's 30 and 60 Hz pins; the advisor's U4e round-1 ruling, R2 (A2, A5) |
```
- The npt-client row, old (one line):
```text
| npt-client | a dependency on npt-link, for `Channel`; bites builds only, `BITES` and `grid_with_bite` (the advisor's U4s ruling, condition 1 (a)) | as above; u4s-carries.md carry 30: arch F1 |
```
- New:
```text
| npt-client | a dependency on npt-link, for `Channel`; `StepOut`'s fields (`datagrams`, `reliable`, `next_due_ns`, `hashes`, `lost`, `wraps`); `ClientEndpoint`'s methods (`new`, `input`, `step`, `with_starts`, `with_lead`, `slot`, `newest_tick`, `last_run_command`, `wrong_channel`, `refused`, `stale`, `duplicates`); bites builds only, `BITES` (3 ids), `grid_with_bite` and `ClientEndpoint::with_bite` | as above; u4s-carries.md carry 30: arch F1; plan.md:486 (`StepOut`); carries 24, 61 (b) and (c); U4E-Q2, U4E-Q7; -58's counter wraps; carry 74 (`with_lead`, set by U4r's lab); the advisor's U4e brief ruling, A5 (the bites-only constructor); the advisor's U4e round-1 ruling, R2 (A5) |
```

## Before the merge (in this order)
1. **The fix pass** (R8): FX-1 to FX-13, each with its test failing under its named plant, restored and
   sha256-checked; lint, doc and fmt clean; check.mjs twice counted in the copy (all 10 steps, counts as deltas on 458 /
   524 / 52); the record's "Fix pass, round 1"; .gate/u4e/fx-hashes-end.sha256. The builder's brief carries my U4d-F
   ruling's T4, items 1 and 2.
2. **The main session:** the copy's files equal fx-hashes-end.sha256; the snapshot re-cut (wip/u4e.patch) and
   committed; round 2's packet cut by review_packet.py on what changed, this ruling as its "judged against" half.
3. **Round 2, on what changed** (R8): netcode-architect, security-auditor and engine-architect (read only), then the
   test-engineer (plants, its start hashes as .gate/u4e/r2-hashes-start.sha256, one counted check.mjs) beside the
   rust-reviewer (read only). APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead; a Medium comes
   to me first and becomes a follow-up or a carry; a High or Critical comes to me for a third round on it alone.
4. **M1-M8** (R9): the copy is the one reviewed; the base still fits; the snapshot and the apply; the merged bytes are
   the reviewed bytes; T1 and T4 applied, then check.mjs counted on the merged tree with the by-name lines for T5; TE2-5's
   rule; the gate; one commit carrying T1-T6, the record and the report.
5. **M9 after the push:** the saver restarted; the card (U4e's merge, the base counts, carries 73-81 and C-U6-12 to
   C-U6-15, U4e's open security findings); the copies removed per CHECKPOINTS.md §10 once CI is green; U4r's brief
   names carries 73-77 and carry 62 (h)'s measured window.

## Decisions needed
None. Every point here is a technical or test choice inside the documented vision, or a route between lanes, which is
mine to decide. R1 reads my own text (B2.1, carry 61) and makes the tick16 proof stricter; it reads no ADR row and no
word of Mustafa's, and changes no pass bar. Mustafa's words on the Low ("A Low never triggers a push") hold: every Low
here rides the push the High needs anyway, or becomes a carry, a report line or a follow-up.

## For Mustafa
Nothing. PR #40 stays needs-mustafa for its existing reasons; U4e adds to its report's list of open security findings
(SEC-U4E-4's transport half, Medium; SEC-U4E-2, -5, -6, -8 and -9, Low), with no action for you now.

## What I checked, and what I did not
**Checked myself (reading, git and sha256; no build, run or plant):**
- My role file from main, whole; the brief; the five seats' notes, whole; the builder's record; my U4e brief check; my
  U4d-F ruling (R5's M1-M9 and T4); my U4j round-2 ruling's B2.1, B5 and B7.2 (carries 61 and 62); the card's
  "Running".
- The plan: u4-build-plan.md §4.1, §4.5, §4.6, §4.9, §4.12-§4.14, §5 U4e, U4r, U4f-U4h and the lanes; plan.md:508-509
  (verified word for word) and §4.7's lists; u4s-carries.md §U4e (21-27, 39, 50, 51, 53-57, 61, 68) and the other
  sections' last entries (the highest number 72); plan.md's U6 row (the last C-U6-11).
- The contract: marks/README.md §1 (:44-73) and its window default (:134); SERIES.md:150, :218, :266, :319; marks.json's
  PM-0001-4, -18 and -34 entries.
- The code in .gate/u4e-r1: client/src/lib.rs (`receive`, `too_old`, `see`, `lost`, `sentinel`, `file`, `group`, the
  start of `full_state`: :431-720), pump.rs (:196-275, :366-430), room.rs (:545-606, :682-800, :1038-1100), the tick16
  tests (server/tests/loopback.rs:195-252; proto/tests/bites.rs:166-190), proto/src/quant.rs's bite (:133-139),
  judge/src/figures.rs:542-554 and the lab's `compare` (server/tests/common/mod.rs:345-355), l1/src/lib.rs's datagram
  model (:13-24, :75, :179-199), proto/src/msg.rs's a3 test (:587-623), and greps for A5's nine items and `with_cover`.
- Both copies against the test seat's start hashes (23 of 23 OK each); the builder's copy's HEAD and status; the
  snapshot's sha256 against db49108's; the branch's spikes/ unchanged since 97bb9d3.
- The test seat's logs behind each settled finding (Q-1, Q-2, Q-6, Q-7, Q-9, Q-10; P-A, P-B2, P-C, P-D3, P-21/22/57;
  the counted run).
- Each old text that T1-T4 replace or anchor on: found once at the branch head (line breaks read as spaces).

**Not checked:**
- I built, ran and planted nothing. R1's form is reasoned, not built: whether the first-tick memory and the bounded
  once-per-tick filing fit cleanly is the builder's to show and round 2's to check.
- That no honest L1 group names a tick before the client's first snapshot at least the ring behind its newest rests on
  L1's documented model and the room's full-state gate, not on a run; round 2's netcode-architect checks it.
- RR-5's figures are the rust-reviewer's on this 4-vCPU VM; I did not re-run its bench. NC-L2's age-32 edge and
  SEC-U4E-5 were not run by anyone (by reading only).
- T6's rows follow the fix pass's final public items, which the main session re-derives at the merge; T5's numbers are
  filled from M5's lines.
- The rust-reviewer's cites for RR-1, RR-2 and RR-4 point past the end of rewind.rs and events.rs (the test seat noticed
  it); I read their claims against room.rs and rewind.rs's actual lines.


