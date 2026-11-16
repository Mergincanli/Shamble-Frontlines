Task: night 2026-11-16 item 7 (npt-u4f-brief), security-auditor check of U4f brief rev 1 (hostile-input half). Notes and check file.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1. Read the approved plan (781212 night-2026-11-16.md) - its Agents item 2 bullet is the authority.
- B2. Read sources: N/u4-build-plan.md §4.10 (:360-390), §4.12, §4.14 (:453-476), §5 U4f; N/plan.md §4.4; N/advisor-u4-wave1.md:492-495 (U4E-Q11); marks.json -21/-22; N/night-u4f-scout-full.md (map).
- B3. Read the carries' text at N/u4s-carries.md for 46, 80, 76, 74, 57, 86, 21, 27, 18, 39, 50, 61, 58, 77.
- B4. Read the draft rev 1 (837 lines) and the drafter's notes (NA-6, NA-7, NA-8).
- B5. Read the code the draft cites for the hostile-input half (room checks, -21 counters, Kick path, pump) at 11fd190.
- B6. Threat model: list what a forged client can send or do; table refusal/counter/twin/bite per kind vs the draft.
- B7. Findings SA-n.
- B8. NA findings verdicts; could-not-do list; DONE.

## Notes
- B1 done (plan read; my bullet: §4.10's kinds, clamp points, 1.5x clock, lag switch, never-acker, U4E-Q11 server-side counters, carry 76 Kick path; refusal, counter, twin, bite each; nothing forged uncounted).
- B2 done. Key source facts (11fd190):
  - 11 forged kinds = marks.json -21 bites :5063-5104; notes :5118. -21 checks :4907-5047 (unrejected_count max==0 :4932-4943; twin :4944-4953; clamp counted_count :4966-4977; fast clock :4988-5023; lag switch :5024-5047). -22 :5120-5202.
  - plan.md §4.4 :510-514 (each check a home and a -21 counter; server column incl. size, trailing/unknown fields, wrong channel, per-connection and new-session rate limits, ping rate and the hold-time cap).
  - §4.10 :360-390; U4E-Q11 wave1 :492-495 (four server checks; U4f adds their -21 counters and the judged runs on U4e's pump).
- B3 done (carries 46 :506-509, 80 :510-534, 86 :535-537, 57 :223-228, 74 :343-354, 21 :148-149, 27 :166-168, 18 :129-132, 39 :169-179, 50 :180-183, 61 :229-247, 58 :269-273, 76 :364-371, 77 :372-381 read).
- B4 done (draft read whole; NA-6/7/8 at netcode-architect-u4f.md:15-17).
- B5 done. Code facts at 11fd190:
  - room/src/input.rs:80-125 `offer` order: seq-forward (Repeat) :81-88; `newest_seq = Some(c.seq)` :89 BEFORE press :90-106, late :107-113, full :114-116.
  - room.rs:1322 the header carries `newest_received`; client/src/lib.rs:678-686 drops every unacked command at or below it (never resent).
  - room.rs:744-801 receive_input (kicked silent :753-755; ack_groups 0 skips the ack check :756; NeverSent :783-787; offer counters :794-798). room.rs:713-742 receive (`_ => {}` :740). room.rs:363-369 canonical (wrap-aware, non-total). room.rs:318-323 kick; :979-1038 route_events; :1057-1063 forced full state unacked -> kick.
  - lab/src/scenario.rs:21-27 session doc; :441-455 drop_slot; :505-508; :527-538; :557-584 kicked drop; r8 :2117-2170, r9 :2172-2224; client_counts footer :1165-1184 writes late, press_refused, buffer_full, rewinds_refused only; logs.rs:542-554 writes only encoder refusals. No pump counter reaches any log (grep of lab/src).
  - server/src/pump.rs Counters :110-161 (21 counters); stream decode error other than WrongChannel/WrongDirection closes the session :414-422; bucket on Input only :449-457; Ping limit :458-471; Pong unlimited, matched against <= 8 outstanding :500-508; hold time clamped to one tick, no counter :510.
  - catalogue :59 (every refusal counted by kind), :87 (Input on a stream: wrong channel, counted as forged), :88 (room->client type from a client counted as forged), :92 (hold time), :96 (ClientSettings, U4g), :120-121 (logs hold a length or hash of client data, never the data).
  - proto quant.rs:34-42, :115-131; msg.rs:220-226, :420. Lab applies a proto id to every Quant (scenario.rs:212-237, :261-270); stream_of :140-150 still works under no-channel (framings differ): checked, no finding.
  - sim world.rs:1401-1420 (Reload, Ammunition, Cooldown refusals), :1446-1452 refuse (slot, seq, kind); clamp :1285-1312 per command from the current origin; World::BITES :937-938 (4 ids, none -21's); no -21 id anywhere in crates (grep).
  - bots/src/lib.rs:74-76 FAR_CM 32,000; greybox.rs:62 HALF_MAP 12,800, :151; room.rs:441 extent.
  - events.rs:20 MAX_EVENT_AGE 255; :52-59 push (unbounded bite keeps past cap); :87-92 expire runs under the bite too.
  - l1/src/lib.rs:181-196 profiles (P1 50 ms ±10, P3 150 ms ±30).
- B6 done (threat list below). B7 done (findings below). B8 done (verdicts below).

---

# Check of U4f brief rev 1: the hostile-input half (security-auditor)

Read at 11fd190. Draft cites are `D:<line>` (u4f-brief-draft.md, rev 1). Source cites are at 11fd190 with the draft's
short names (plan = N/u4-build-plan.md; carries = N/u4s-carries.md; crate paths under spikes/netcode-proof/crates/).
Rulings and Mustafa's words are cited by file and line range only.

## 1. Threat model: what a forged client can send or do against U4f's room and server-side checks

Ground truth: every client may be modified, replayed or scripted; the room and the pump repeat every check. Columns:
refusal, counter (written to a log U4f's runs keep), twin, bite. P = present in the draft, M = missing, U = present but
untestable or vacuous as worded, n/a = not applicable. "SA-n" names the finding.

| # | Hostile kind or behaviour | Refusal | Counter | Twin | Bite | Notes |
|---|---|---|---|---|---|---|
| T1 | kind 1, tick far ahead (lead bound) | P (D:152; U4F-Q8) | P | P (D:323) | P (D:298) | no lower bound on a slot's first commands under A: SA-1; the bound is a speed and rewind knob: SA-11 |
| T2 | kind 2, tick equal to or older than the last run | P (D:153) | P, but honest copies land in it: SA-2 | P | P (no-newer-tick, not-older-than) | honest stamps must strictly rise: SA-8 |
| T3 | kind 3, sequence not forward | P by U4F-Q9 (D:154, :715-724) | P | P | P | NA-6 holds; window undefined: SA-9; a refused command still moves the seq reference: SA-3 |
| T4 | kind 4, move length above 1 | P (D:155) | P | P | P | |
| T5 | kind 5, extra press edges | P (D:156; input.rs:90-106) | P (`press_refused`) | P | P | refused command acknowledged: SA-3 |
| T6 | kind 6, aim off the map | P (D:157) | P | P | P | z half refuses nothing: SA-14; NA-8 holds |
| T7 | kind 7, fire, reload or cooldown edge against the sim | P (sim, world.rs:1401-1420) | P (`reports()`) | P (carry 46, D:327) | P, site in npt-sim (NA-9, U4F-Q1) | ammunition unreachable in SS: SA-15 |
| T8 | kind 8, ack of a snapshot never sent | P (room.rs:783-787) | P, unit unclear: SA-4 | P (D:324) | P | |
| T9 | kind 9, trailing bytes or a set reserved/pad bit | P (pump) | P (two counters) | P (D:324) | P (proto, D:306) | must ride datagrams; both sub-forms: SA-12 |
| T10 | kind 10, more commands per tick than the catch-up cap | P (A: new cap; B: `Full`) | P | U: construction unasked, excess acknowledged and lost: SA-3 | P (A only) | |
| T11 | kind 11, a known type on the wrong channel | P (pump.rs:414-422) | P (`wrong_channel`) | P (D:326) | P (proto, D:308) | never kicked: P (D:162) |
| T12 | clamp: aim beyond reach, inside the map | never refused, by design (world.rs:1285-1312) | none by design (counted_count from refusals) | U under option B reruns: SA-10 | P (refuse-not-clamp, npt-sim: NA-9) | NA-8 holds |
| T13 | 1.5x host clock for 60 s | P (kinds 1 and 2; B's `Full`) | P (excess series) | n/a (bound arithmetic, U4F-Q11) | P (not-older-than, A) | U: stamps may never reach the bound: SA-6 |
| T14 | lag switch, 200 ms held at P1, one burst | P (catch-up cap under A; late under B) | P (lagswitch series) | n/a | M (no bite in its suite) | U: 6 commands never meet a cap of 8: SA-7 |
| T15 | never-acker (ack groups 0) | P (cap, resync, Kick(overflow): room.rs:1014-1034, :1057-1063) | P (headroom, dropped) | n/a | P (unbounded, U4e's) | Kick path met at U4r (carry 76, r9 scenario.rs:2172-2224); NA-12 holds; ack-only-full-states variant: SA-16; counters lost at session drop: SA-5 |
| T16 | oversize datagram or stream message | P (pump `too_large`) | P (D:171) | n/a | none | "reported", read from U4E-Q11 without a question: SA-5 |
| T17 | Input datagrams over the bucket | P (`input_limited`) | P (D:172) | n/a | none | cheats must stay inside it: SA-13 |
| T18 | other decode refusals: truncated, bad length, unknown type, wrong direction, out of range, malformed | P (U4e) | M (in memory only; catalogue :88 calls wrong direction forged) | n/a | none | SA-5 |
| T19 | Ping flood | P (`ping_limited`) | M (D:110 claims a clean-run figure no series writes) | n/a | none | SA-5 |
| T20 | Pong forgery: unknown id; hold time above one tick | P (`pong_unknown`); hold clamped | M (pong_unknown unlogged; the clamp has no counter: pump.rs:510; plan.md:514 names it) | n/a | none | no effect on L1 (RTT from the link, pump.rs:339-344); ws only |
| T21 | Join again, datagrams before Join, stream messages while joining | P | M (unlogged) | n/a | none | SA-5 |
| T22 | Resync flood | P (room.rs:716-737) | M (unlogged) | n/a | none | SA-5 |
| T23 | ClientSettings, SessionControl | ignored, uncounted (room.rs:740) | M | kind 11's twin uses ClientSettings | none | U4g's clamp and the capability check's unit: outside U4f (D:165-166), correctly left |
| T24 | forged view time, buying rewind | none at U4f (tolerance and cap are U4h's: carry 40, D:165-166) | `rewinds_refused` (ring misses only) | n/a | n/a | correctly left; the lead bound feeds it: SA-11 |
| T25 | forged ack bitmask on a sent snapshot; an old ack | accepted (self-harm only); `acks_old` | M (unlogged) | n/a | n/a | SA-5 |
| T26 | a stolen token joining a live slot | roster, bind, Kick(replaced) (U4e/U4r; r8 scenario.rs:2117-2170) | `ended` "replaced" | n/a | n/a | not U4f's; -47's rejoins are U4F-Q4 |
| T27 | a crash from a non-total sort (carry 80 (a)) | guard as worded leaves a gap | n/a | n/a | n/a | SA-1 |
| T28 | a slow reader (stops reading its streams) | WsServer's send budget (plan.md:521-523), U6 | n/a | n/a | n/a | outside U4f |
| T29 | a flood of non-Input datagrams (Pongs, garbage) | counted per kind; no rate limit at the pump but Input's and Ping's | M (unlogged) | n/a | n/a | U6's transport limits; noted only |

## 2. Findings (most severe first)

### SA-1 (High): carry 80 (a)'s guard, as worded, leaves a sort panic reachable from one forged Input under option A. carried (entry 80 (a)); refines its guard
- **Where:** D:139 (2.3's carry 80 (a) row: the lead bound "refuses a command 2^31 ticks or more from the room's tick"),
  D:263 (f8: "one slot's commands 2^31 ticks apart"). Source: room/src/room.rs:363-369 (`canonical`, wrap-aware compare
  `(b.tick.wrapping_sub(a.tick) as i32)`); room/src/input.rs:107-110 (the old-tick reference is `ran_through`, `None`
  before the first take); carries :513-518 (Rust's sorts may panic on a non-total order).
- **Scenario:** under option A a slot's first `Input` (no command run yet, so kind 2 has no reference) carries three
  commands, seqs forward, ticks T-2^31+1, T-2^30 and T+1 (T the room's tick). None is 2^31 or more from T, none is far
  ahead, so the guard as worded passes all three; option A runs all three in one tick; `canonical` then sees a < b,
  b < c and c < a (c - a = 2^31 reads as i32::MIN). That is no total order, the case carry 80 (a) says Rust's sorts
  may panic on, and a panic on the tick thread stops the room for all 48 clients. f8's exact-2^31 pair does not cover
  a cyclic triple.
- **Fix:** the brief states the accepted window as one unsigned wrapping difference from the room's tick, for example
  accepted only if `tick.wrapping_sub(room_tick)` as i32 lies in [-W, bound] with W a stated past window (the redundancy
  cap plus the reorder depth, well under 2^31), applied before any command reaches the sort, under both options and
  before the first command runs; or `canonical` takes a total key (slot, then seq widened, or tick relative to the room's
  tick). f8 adds the cyclic triple on a fresh slot and a property test over random u32 ticks: no panic, one order.
- **Cheapest proof (day):** needs a run (day): after the build, a room unit test under option A feeding that triple to a
  fresh slot, run through u4f-run.sh: `cargo test -p npt-room --all-features canonical` (the test name the builder
  gives). Before the build, the arithmetic above stands on room.rs:363-369 alone.

### SA-2 (High): the fixed check order counts every honest redundant copy as a forged kind 2
- **Where:** D:143-148 (the counting rule: the first check in the order refuses), D:152-154 (kind 1 first, kind 2
  second, kind 3 third), D:146-147 (the claim that honest commands are never counted as forged), D:267 (f12: an honest
  redundant repeat is not refused), D:720-722 (U4F-Q9 (a): the identical repeat "dropped uncounted as now", placed in
  kind 3's step). Source: room/src/input.rs:81-88 (today the repeat filter runs first); client/src/lib.rs:442-443 and
  carry 27 (each command rides up to 8 datagrams).
- **Scenario:** every command is sent up to 8 times until acknowledged; most copies arrive after the command ran, so
  their tick is equal to or older than the last run. With the tick checks ahead of the repeat filter, each such copy is
  refused and counted as forged kind 2: thousands of forged-kind refusals on the 37 honest clients of the forged suite
  and on every -20 client, f12 failing against the brief's own order, and kind 2's refusal records swamped by honest
  copies (which also defeats attribution, SA-4).
- **Fix:** the order starts with the uncounted repeat filter (sequence not after the newest received and content equal
  to the command kept under that sequence), then kinds 1, 2, 3 (a backward sequence with other content), 4, 5, 6, 10;
  kind 8 before all of them (whole datagram), 9 and 11 at the pump. f11 and f12 assert 0 refusals of any kind on an
  honest P3 trace with redundancy 8, and the forged suite reports refusals per kind on its honest clients (0 expected for
  kinds 1-6 and 8-11).
- **Cheapest proof (day):** f12 as worded, against a room built in the brief's order (it fails); or by reading: the
  draft's order against input.rs:81-88.

### SA-3 (High): a refused command still moves the room's sequence reference and is acknowledged, so it is never resent; the twin claim "the accepted stream is then the forged run's" is unproven, and no test checks a kind's twin hash before the judged runs
- **Where:** D:323 (2.12: twin for kinds 1-6 and 10 = the datagram with the forged command left out), D:325 (catch-up
  twin: repeats only), D:189 (the catch-up kind's construction routed to U4F-Q8), D:705-713 (U4F-Q8 asks only the
  bound's form and value), D:271-273 (f16 compares fates and group counts only; f17 the press twin; f18 the clamp twin).
  Source: room/src/input.rs:89 (`newest_seq = Some(c.seq)` before the press, late and full checks at :90-116);
  room/src/room.rs:1322 (the header's `newest_received`); client/src/lib.rs:678-686 (the endpoint drops every unacked
  command at or below it); plan :385-388 (the catch-up kind and its twin); advisor-u4.md:78-89 (condition 1, whose
  input-state point the room's state is the analogue of).
- **Scenario:** the catch-up forger sends 16 new commands in one tick. The 8 past the cap are refused (`Full` under B,
  the new cap under A), yet the first refused one already set `newest_seq` to its sequence, the next header acknowledges
  all 16 as received, and the endpoint never resends the 8. In the forged run those 8 ticks run nothing (A) or the last
  command again (B); in a twin built as D:325 says, the endpoint still holds them unacknowledged and they run. The state
  hashes differ on a correct build, and the first place this shows is the judged -21 forged set (49 runs, about 4-5 h:
  D:516). The same trace makes any forger that forges only some copies of a command fail its twin.
- **Fix:** the brief rules (or asks as a U4F-Q, recommending the first) that only a command the buffer accepted moves
  the sequence reference and the "newest received" acknowledgement, so a refused command is resent and refused again
  (counted once per (slot, seq): SA-4), and every forger forges every copy of its command; or, if the room keeps
  acknowledging refused commands (the lag-switch reading: excess commands are lost), the twin is built to match and the
  brief says so. U4F-Q8 gains the catch-up kind's construction and twin under each option (D:189 sends it there). A new
  lab test before the judged runs: for each kind (1-11) and the clamp, a short forged run and its twin give equal state
  hashes on every tick, under A and B.
- **Cheapest proof (day):** on today's code, a room unit test (option B): offer 8 buffered commands then a ninth (seq
  s+8, `Full`), then its honest copy: assert the copy is `Repeat` and the next header's `newest_received` is s+8. Run
  through the runner: `cargo test -p npt-room --all-features <that test>`.

### SA-4 (Medium): "rejected and counted" has no stated unit or matching key, so a count can go negative and hide a leak
- **Where:** D:143-148 (npt-lab counts a forged command rejected only when a record names its kind), D:159 (kind 8's
  counter `ack_refused_commands`), D:160, D:162 (kinds 9 and 11: per-datagram pump counters), D:188 (1,000 forged
  commands per kind). Source: SERIES.md:158-159 (unrejected = sent minus rejected-and-counted, a subtraction);
  marks.json:4932-4943 (max == 0); room.rs:785-786 (one `ack_refused` per datagram, `ack_refused_commands` per command
  carried, redundant copies included).
- **Scenario:** a forged command rides up to 8 datagrams. If copies are refused one by one (SA-3's fix) and the lab
  subtracts refusal records from commands sent, one forged command can yield 8 records: unrejected goes negative, and a
  leak elsewhere in the same kind (a copy that ran) still passes max == 0. For kind 8, "sent" counted in datagrams
  against `ack_refused_commands` counted in commands gives the same negative. The twin check catches a leak only when
  it changes state.
- **Fix:** define the unit per kind: one forged (slot, seq) for kinds 1-7 and 10; one forged datagram for 8 and 9; one
  forged message for 11. A forged item is rejected and counted only if at least one refusal record names it with its
  own kind and no copy of it ran (checked against the tick's `TickInput` commands); unrejected = items failing that,
  never negative. Kind 8 counts `ack_refused` (datagrams). A lab unit test plants one leaked copy and shows unrejected 1.
- **Cheapest proof (day):** a unit test of the lab's attribution with a planted leaked copy (after the build).

### SA-5 (Medium): most pump and room refusal counters never reach a log; the hold-time clamp has no counter; U4E-Q11's "judged runs" for size and bucket are read as "reported" without a question
- **Where:** D:170-176 (2.5 writes five pump counters), D:110 (ClockSync's row expects `ping_limited` 0 on a clean run,
  with no series for it), D:165-166, D:173-175. Source: server/src/pump.rs:110-161 (21 counters), :510 (hold time
  clamped to one tick, uncounted); lab/src/scenario.rs:1165-1184 and lab/src/logs.rs:542-554 (the only counters
  written: late, press_refused, buffer_full, rewinds_refused, the encoder's refusals); plan.md:510-514 (each check, the
  ping rate and the hold-time cap included, gets a -21 counter); catalogue :59 (every refusal counted by kind), :88 (a
  room-to-client type from a client is counted as forged); wave1 :492-495 (U4E-Q11).
- **Scenario:** a forged client sends room-to-client types, out-of-range codes, unknown types, Ping floods, stray Pongs,
  a second Join or Resync floods. Each is refused and counted in memory, then lost: no log of any U4f run shows it, and a
  dropped session's pump counters (kicked, replaced or closed: scenario.rs:441-455, :527-538, :575-582) vanish with its
  `Decoder`. The forged suite cannot show that a forger landed in its own kind rather than in a decode refusal, and
  -22's never-acker's pump counts are gone at its kick.
- **Fix:** npt-lab writes every refusal field of `Counters` and `ClientCounters` per session when the session ends and
  at the stop (one event per session; names and SERIES.md rows by U4F-Q1); the forged suite reports, per client, every
  counter other than the forger's own kind (0 expected). The hold clamp gets a counter if U4F-Q1 opens npt-server (then
  section 6 item 4 applies), else the brief says why it stays uncounted (no effect on L1, pump.rs:339-344). A U4F-Q asks
  how U4E-Q11's judged runs read for size and bucket (reported only, since no -21 kind is theirs; or a forged size and
  rate behaviour, which needs a marks.json change and its read).
- **Cheapest proof (day):** none needed for the gap (Grep of lab/src at 11fd190 finds no pump counter name); after the
  build, f14 widened to every counter.

### SA-6 (Medium): the 1.5x clock's stamps may never reach the lead bound, so its check can pass with the bound idle
- **Where:** D:191 (the cheat gives "the stamps its controller gives"), D:274 (f19: excess at most 0, clean build),
  D:696-703 (U4F-Q7). Source: marks.json:5000-5011 (at most the server's ticks plus the lead bound), :5107-5109 (the
  suite's only bite, not-older-than, option A); ADR 0001:231-233.
- **Scenario:** under U4F-Q7 (a), stamps are the newest snapshot tick plus L. The newest tick moves with the server, so a
  fast host clock gives more commands per tick but no later stamps: the extras are equal-tick (kind 2), the lead bound
  never refuses, and "at most server ticks plus the lead bound" holds by construction. A cheater stamps each command one
  tick newer than the last, racing to the bound; that is the case the check exists for, and no bite in the fast-clock
  suite removes the lead bound (no-lead-bound runs in the forged suite).
- **Fix:** the 1.5x client stamps adversarially (each command one tick newer than its previous one, or from its own fast
  host clock with no snapshot correction) under either U4F-Q7 option; each episode reports the lead-bound and not-newer
  refusals beside its excess (more than 0 lead-bound refusals shows the bound acted); f19 asserts both.
- **Cheapest proof (day):** arithmetic on D:122 against marks.json:5000-5011; after the build, f19 with the per-episode
  refusal counts.

### SA-7 (Medium): the lag switch's 200 ms burst never meets the catch-up cap, so its check is vacuous and no bite tests it
- **Where:** D:192, D:275 (f20), D:467 (5.1: no bite in the suite). Source: ADR 0001:1611 and marks.json:5024-5047
  (inputs held 200 ms under P1; movement within the catch-up cap); input.rs:61 (`CMD_CATCHUP` = 8); bots/src/lib.rs:14-17
  (at most one command per step, when the newest tick moved).
- **Scenario:** 200 ms at 30 commands a second holds 6 new commands; under option A all 6 run in one tick, under the cap
  of 8, so "moves no further in any tick than the catch-up cap allows" passes with no cap at all. Under option B they are
  late. The suite proves only that a short burst runs.
- **Fix:** the brief states the arithmetic and asks (a U4F-Q) whether a reported, unjudged hold above 8/30 s (for
  example 400 ms, 12 commands) and a no-catchup-cap bite run in the lag-switch suite join U4f's runs; marks.json stays
  unchanged unless the advisor rules otherwise (then its read is owed: D:527-535).
- **Cheapest proof (day):** arithmetic (0.2 s x 30 = 6 < 8); after the build, one short lag-switch run under
  PM-0001-21/no-catchup-cap, option A, P1, whose lag-switch checks still pass (needs a run (day); the command line is the
  builder's c-u4f-*.mjs's).

### SA-8 (Medium): the lead controller has no rule that stamps strictly rise, so honest commands can be counted as forged kind 2
- **Where:** D:125 (2.2's Bounds row: at least 1, at most the bound less a margin), D:122 (U4F-Q7 (b) puts stamps on
  ClockSync's estimate), D:109 (the filter resets over 500 ms), D:260-261 (f5, f6). Source: ADR 0001:231-233 (strictly
  newer than the last command run); scenario.rs:1179 (U4r already logs `duplicate_stamps_count`).
- **Scenario:** the controller lowers its lead, or ClockSync's estimate steps back on a reset; the next command's stamp
  is equal to or below the previous one, and kind 2 refuses an honest command as forged. Honest clients then carry
  forged-kind refusals and -20 loses on-time commands.
- **Fix:** 2.2 adds: each stamp is strictly above the previous command's; a lower lead is reached by holding the stamp
  (skipping a command) never by stamping backwards; f5 adds a step-down case and a ClockSync reset, both with strictly
  rising stamps, and SA-2's honest-client refusal report covers it in runs.
- **Cheapest proof (day):** a client unit test after the build (f5 widened).

### SA-9 (Medium): U4F-Q9 (a)'s kind-3 rule leaves a backward sequence outside the kept window undefined, and honest reordered copies reach it
- **Where:** D:720-722 (kind 3 refused when the content differs from the command kept under that sequence, "the last 8
  per client kept"). Source: input.rs:81-88; l1/src/lib.rs:190-192 (P3 ±30 ms jitter against a 33.3 ms send period);
  client/src/lib.rs:442-443 (8 unacked per datagram).
- **Scenario:** at P3 a datagram overtaken by the next two arrives carrying sequences down to newest - 9, outside a
  window of the last 8. If outside-window sequences are refused, honest copies are counted as forged kind 3; if dropped
  uncounted, a forger sends its backward sequence there and goes uncounted (marks.json:4932-4943 then cannot pass for
  kind 3's honest reading, or a forgery hides).
- **Fix:** U4F-Q9 states the outside-window rule; the window is the redundancy cap plus the profile's reorder depth
  (stated), outside it a backward sequence is dropped uncounted, and the kind-3 forger's backward sequence lies inside
  the window with content that differs. f12 adds a P3-like reordered honest trace with 0 kind-3 refusals.
- **Cheapest proof (day):** arithmetic; after the build, f12 widened.

### SA-10 (Medium): the clamp twin can mismatch under option B, where a missed tick reruns a clamped command from a moved origin
- **Where:** D:190, D:328, D:273 (f18: no option named). Source: room/src/input.rs:135-141 (option B reruns the last
  command); sim/src/world.rs:1285-1312 (the clamp runs per command from the current origin and reports (slot, seq,
  point)); plan :363-366.
- **Scenario:** the clamp cheat's command runs, then is rerun on a tick with none stamped. The sim clamps it again from
  the soldier's new origin: two clamp reports with one (slot, seq) and different points. The twin can send one point per
  command; on the rerun tick its point lies on the old ray, so the aim, and the state the aim feeds, differ: a twin
  mismatch on a correct build, in clamp option B.
- **Fix:** the clamp suite reports reruns of clamp-cheat commands and the brief either requires 0 (the cheat gives a
  command every tick, and a rerun voids the pair) or defines the twin per tick; f18 runs under option B with a forced
  rerun after a move.
- **Cheapest proof (day):** needs a run (day): a lab or room unit test, option B, a clamp command rerun after a move,
  asserting two reports for one sequence and then the twin's hash on the rerun tick.

### SA-11 (Medium): the lead bound's value is a security parameter twice over, and U4F-Q8's 600 ms is driven by a test construction
- **Where:** D:705-713 (U4F-Q8 (b) at 600 ms, 18 ticks: "the smallest round value above P4's lead plus the catch-up
  kind's need"). Source: plan :351-352 (§4.9's tolerance adds the input-buffer wait to a client's accepted view time);
  marks.json:5000-5023 (the fast-clock allowance grows with the bound); ADR 0001:1611.
- **Scenario:** under option B a client may stamp up to the bound ahead; its commands then wait up to 18 ticks in the
  buffer, and U4h's tolerance adds that wait to the view time it accepts: up to 600 ms more rewind bought by stamping
  ahead (inside the rewind cap). The same 18 ticks are the speed hack's allowance per episode. Honest need is P4's 9
  ticks; the extra comes from building the catch-up forger under B with 16 future commands.
- **Fix:** U4F-Q8 states both allowances (ticks, ms and metres at top speed) beside each value, shows the smallest bound
  in which the catch-up kind can be built under B, and asks for a carry to U4h: the input-buffer wait in §4.9's estimate
  is capped by the honest lead, not by the lead bound.
- **Cheapest proof (day):** arithmetic (18 ticks x 33.3 ms = 600 ms; P4's lead 9 at l1/src/lib.rs:193-196 by U4R-Q14's
  formula).

### SA-12 (Low): kind 9's forgery must ride Input datagrams only, and both of its forms need sending
- **Where:** D:160, D:188, D:270 (f15). Source: pump.rs:414-422 (any stream decode error other than the wrong channel
  or direction closes the session as malformed); proto/src/quant.rs:115-122 (`strict`: trailing bytes and set
  reserved or pad bits); plan :383-384 (no forged kind closes the session).
- **Scenario:** a kind-9 forgery on the control stream closes the forger's session, drops its soldier and voids the
  run's `pre:PM-0001-2`; a forger that sends only trailing bytes leaves `unknown_field` unexercised in the judged runs.
- **Fix:** 2.6 says kind 9 rides datagrams only and alternates its two forms; f15 checks the channel and both forms.
- **Cheapest proof (day):** reading (pump.rs:414-422).

### SA-13 (Low): the cheats' traffic must stay inside the Input bucket, or the bucket, not the room, does the refusing
- **Where:** D:191-192, D:172. Source: pump.rs:19-22, :84 (8 datagrams, refilled at 60 a second at 30 Hz),
  :449-457; plan :385-388 (the same rule stated for the catch-up kind).
- **Scenario:** a lag-switch burst or a fast-clock rate above the bucket would be cut by `input_limited`, and the
  room's checks would pass partly untested.
- **Fix:** 2.6 states the arithmetic per cheat (lag switch 6 datagrams in a burst against 8; fast clock 45 a second
  against 60) and each cheat client's `input_limited` is reported, 0 expected.
- **Cheapest proof (day):** arithmetic; the per-session counter once SA-5 writes it.

### SA-14 (Low): kind 6's z bound is the wire's own range, so the z half refuses nothing a decoder accepts
- **Where:** D:157 ("z inside the position grid"). Source: sim/src/greybox.rs:151 (the extent is x and y only);
  ADR 0001:235-236.
- **Scenario:** an aim at any decodable z passes kind 6; the clamp then handles it, so there is no effect, but the
  check's z half is nominal.
- **Fix:** state the z bound used (the greybox's floor and a ceiling) or that z is left to the clamp by design, with why.
- **Cheapest proof (day):** reading.

### SA-15 (Low): kind 7's ammunition case cannot occur in SS
- **Where:** D:158, D:188. Source: plan :434-435 (SS scripts no reloads, with a magazine covering an S30 plus 5
  minutes); sim/src/world.rs:1411-1419 (Ammunition only at 0 rounds); advisor-u4.md:87-88 (U4d's unit test per kind).
- **Scenario:** the kind-7 forger can send cooldown and reload edges only; the ammunition refusal is never forged in a
  judged run.
- **Fix:** 2.6 names the sub-cases the forger sends (cooldown, reload) and cites U4d's unit test for ammunition.
- **Cheapest proof (day):** reading.

### SA-16 (Low): a never-acker variant that acknowledges only full states cycles forced resyncs and is never kicked
- **Where:** D:193, D:276 (f21). Source: room.rs:1023-1034 (an overflow inside an unacknowledged forced resync kicks;
  otherwise it resyncs), :1057-1063; events.rs:40 (cap 1,024).
- **Scenario:** at about 460 events a second the queue refills in about 2.2 s, outside the 60-tick window, so a client
  that acknowledges each forced full state and no event costs the room a full state every few seconds for the whole run.
  It is counted (`overflow_resyncs`) and bounded by the 256 kbps window, but no part or report reads it.
- **Fix:** -22's runs report `overflow_resyncs` per client; the variant goes to STATUS as a follow-up for U6's limits.
- **Cheapest proof (day):** arithmetic on room.rs:1023-1034.

### SA-17 (Low): the logging rule for client data is not carried into the brief, which adds the first logs of hostile input
- **Where:** D:450-452 (the netcode.md paragraph). Source: catalogue :120-121 (logs record a length or a hash of client
  data, never the data); .claude/rules/netcode.md.
- **Scenario:** a builder logs a forged datagram's bytes while debugging a forger; results/ is committed.
- **Fix:** D:450-452 adds the catalogue's rule: hostile-input logs hold kinds, counts and the clamp points SERIES.md
  rows name, never a datagram's bytes.
- **Cheapest proof (day):** a review grep of U4f's diff for logged byte buffers.

## 3. The drafter's NA findings I touched

| NA | Verdict | Evidence at 11fd190 |
|---|---|---|
| NA-6 | **holds** | input.rs:81-84 returns `Repeat` for any sequence not after the newest; room.rs:795 counts it with honest copies. Its fix must also meet SA-2 (order) and SA-9 (window). |
| NA-7 | **holds, with a correction** | input.rs:107-116: an old tick under B is `Late`, and nothing limits how far ahead a command may be, so 8 far-ahead commands fill the buffer and later ones are `Full`. The harm is to the forger's own slot (room.rs:792 sets the slot from the session) and to attribution and twins; the far-ahead commands also run later at their stamps (pre-loaded input), which the lead bound exists to stop. A first batch under A has no lower bound either: SA-1. |
| NA-8 | **holds** | bots/src/lib.rs:74-76 (32,000 cm) against greybox.rs:62 and :151 (±12,800 cm) and room.rs:441 (the room's extent is that rectangle). |
| NA-9 | **holds** | sim/src/world.rs:937-938 holds 4 ids, none of -21's; no -21 bite id appears anywhere under crates/ (Grep). |
| NA-12 | **holds** | events.rs:20 and :87-92: events expire at 255 ticks, also under the unbounded bite (:52-59 skips only the cap), so a cap above about the rate x 8.5 s is never reached and both -22's clean check and its bite go idle; U4F-Q12 (b)'s upper check keeps both live. |
| NA-2 | **holds** | `rss_bytes` appears only in crates/log/src/lib.rs:721 and :739 (a test fixture): -22's "cannot grow the server" rests on the queue headroom alone until U4F-Q1 names a writer. |
| NA-1, NA-3, NA-4, NA-5, NA-10, NA-11 | not checked | outside the hostile-input half (the test-engineer's check covers them) |

## 4. What I could not do, and why
- No build, test or run (the night's rules). SA-1, SA-3, SA-7 and SA-10 have "needs a run (day)" entries above; SA-1's
  panic is argued from the comparator's arithmetic and carry 80 (a)'s own words about Rust's sorts, not observed.
- No web: Rust's sort documentation was not read tonight; SA-1 rests on carries :513-518.
- The twin mismatch in SA-3 and SA-10 depends on twin construction details the draft leaves to U4F-Q8 and U4F-Q11;
  each is written as a risk with the unit test that settles it.
- I did not check the test counts, budgets or disk (the test-engineer's half), nor ClockSync's accuracy (NA-4).

DONE
