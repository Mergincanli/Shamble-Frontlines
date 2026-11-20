# U4f2 review round 1: the security seat (security-auditor), 2026-11-20

Scope: hostile input by u4f2-brief.md section 7 (the forged kinds, the attribution, `--forge-every`'s parser), as the
panel brief's security bullet (u4f2-panel-brief.md:50-54). Read only: the patch
`.checkpoints/netcode-proof-test/wip/u4f2-build.patch` and the builder's worktree
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807` (below: WT) by absolute path, plus the logs
under .gate/u4f/. No runs. (This agent has no Edit tool: the file is rewritten whole with Write at each block.)

**Verdict: APPROVE WITH FINDINGS.** Three Lows (SA-U4F2-1 to -3), no security defect. Every forged kind is refused by
the server's own state, and no new server path exists. F-B blocks -21's set as the builder says, but it is a test-validity
matter for the netcode seat, not a security hole (block B). Nothing here touches a pass mark, the judge, marks.json, money
or main. One fix option for F-B would change what the attribution counts, and that is Mustafa's (block B, last bullets).

## Plan
1. Read u4f2-build.md (the builder's record: blocks, plants, F-A..F-E, departures). Check the patch's sha256.
2. Threat model: each forged kind (3, 7, 8, 9, 10, 11) as a hostile client's command. For each, find the room's own
   refusal check (crates/room input.rs / room.rs) and the forger (crates/bots forged code), and confirm the refusal
   depends only on room/sim state, never on a client-supplied claim.
3. The rebuilt kind 7: fire edges inside the cooldown, reload edges inside the reload. Where does the room/sim refuse
   them, and which sim state decides? Weigh F-B (dead forger, respawn reset) from the hostile side: can a forged edge
   while dead or just after respawn change state (fire, reload, cooldown skip)?
4. The attribution (C-U4F2-3, SEC-1): `CommandRefusal`, `RefusalRecord { slot, seq, kind }`, the records' bounds,
   `refusals_dropped_count`, what the lab reads, whether a hostile client can flood records (unbounded growth) or hide
   its refusals behind honest ones (attribution matching by (slot, seq, kind)).
5. `--forge-every`'s parser: below 2 a usage error; 0, 1, negative, overflow (u32/u64), odd values, non-numeric, leading
   +, whitespace; the interval's use in arithmetic (modulo, multiplication overflow).
6. Any new path where a forged command changes state (room or sim): every new branch in room.rs/input.rs.
7. Panics reachable from input in the new code (indexing, unwrap, arithmetic overflow) in room and lab paths.
8. Findings, coverage table, verdict. (All 8 done.)

## Notes (one line per finding as found)
- Stamp: the patch hashes to e9a0ae3e3f48af18**e**a7a947fdf9aba849061fa32fbd91876a948c46f3046406b (64 hex); the panel
  brief (u4f2-panel-brief.md:11) prints 63, one "e" dropped after "18". The patch read is the one named. (A wording
  slip in the panel brief, not the diff; no finding.)
- Room (patch :2134-2410): a rename plus the struct form only; `receive_input` (room.rs:932-1035) unchanged in logic.
  Records capped at REFUSALS_HELD 4,096 per tick (room.rs:37, :944-950), the rest counted in a u64: bounded.
- forged.rs: tallies keyed by the 16-bit seq (BTreeMap/BTreeSet), so per-forger memory is at most 65,536 entries
  whatever the run length (the module text's "bounded by its length" holds, and better). Dropped room records can only
  raise `unrejected_count` (no record = unrejected for room kinds), never hide an item: SEC-1's direction is safe.
- Parser (scenario.rs:2240-2247): `parse::<u64>()`, `k >= 2`; "0", "1", "x", "-1", "" refused; "+12" and "012" taken
  (Rust's unsigned parse takes a leading '+'; harmless, the header records the parsed 12); no upper bound.
- bots lib.rs:944 `f.every * u64::from(f.burst_refused())`: unchecked multiply of the operator's interval (x 8 under
  A, x 4 under B). Cargo.toml:116-120 turns overflow checks on in release for npt-sim and npt-proto only: npt-bots
  wraps in release, panics in debug. -> SA-U4F2-1.
- Kind 7 at `--forge-every 2`: the construction is not sound (block B arithmetic). -> SA-U4F2-2.
- Kinds 8, 9, 11 by count with `saturating_sub` (forged.rs:116): an over-count is clipped silently. -> SA-U4F2-3.
- F-A from the hostile side: the cap's removal is still caught on the forged suite. Log: .gate/u4f/u4f2-hand/bites/
  bite-no-catchup-cap/lab.jsonl, kind 10 `unrejected_count` 32 of 32, every other kind 0. No security gap (block C).
- F-B from the hostile side: no effect beyond the twin in either half; the record is all that is missing (block B).
- No new non-test `unwrap`/`expect`/indexing panic: the one new `expect` (patch :1547) is `#[cfg(test)]` behind
  `get(..32)`; kind 9's `out[PAD_BYTE]` is safe (block A).

## Block A: each forged kind as a hostile client's command (done)
No server path is new: the room's diff is a rename and the record struct; crates/sim and crates/proto are untouched.
Every forged command goes through refusal code that predates U4f2, and each refusal is decided by the room's, the
pump's or the sim's own state:
- **Kind 3 (a changed copy of an accepted sequence):** room input.rs:238-260 `filter`: the kept window holds each
  offered command with whether it was accepted; a copy whose content differs from an accepted one is refused
  `SeqNotForward`. A sequence once refused is kept with `accepted = false` (:233), so every later copy is a `Repeat`
  (:243): a refused command cannot be slipped in on a redundant resend. Forger: bots lib.rs:283-299 (copy of seq-1,
  move x + 1 wrapping or aim x + 100 saturating, in turn: always differs from the original).
- **Kind 7 (press edges where the weapon refuses):** the sim, world.rs:1412-1499, on its own weapon state (`wait`,
  `reload_left`, `ammo`); see block B.
- **Kind 8 (an acknowledgement of a tick never sent):** room.rs:961-1000 with ack.rs:145-171: `AckRing::ack` changes
  nothing on the `NeverSent` path, `last_ack_hi` is set only when the answer is not `NeverSent` (room.rs:967-973), and
  the datagram's commands are refused whole and recorded, without entering the kept window (the loop at :1003 is never
  reached), so their honest resends are checked afresh. Forger: lib.rs:304-307 (ack tick + 1,000, only when the
  `Input` acknowledges a group, so the encoder never refuses it).
- **Kind 9 (an undecodable `Input`):** proto bits.rs:132-151 in strict mode: a set pad bit is `UnknownField`, a byte
  left is `TrailingBytes`. The pad bit is real: proto input.rs:5, :10 (LSB first; the newest command's 180 bits end 4
  bits into its 23rd byte), so bots lib.rs:197 `PAD_BYTE = 5 + 22` and `|= 0x80` set a pad bit, not a data bit. The
  index is safe: `way` is set only when the `Input` has a fresh newest command (lib.rs:344-367), so it encodes to at
  least 28 bytes.
- **Kind 10 (a burst past the per-tick cap / B's buffer):** room input.rs:316-325 (`buffered.len() >= cap`): the room's
  own buffer length. The refused sequences are kept refused (as kind 3's note), so they never run on a resend.
- **Kind 11 (a datagram type on the control stream):** the pump's wrong-channel drop (pre-existing); forger
  lib.rs:240-259 frames a `Ping` for the control stream (`u16::try_from(len).ok()?`: no panic).
- **Kinds 1, 2, 4, 5, 6** (built in U4f, re-read for the shared path): input.rs:263-305, each on the room's
  `next_tick`, newest accepted, the extent, the counters. `forged()` (lib.rs:400-425) has no overflow: `n >= 1`, k at
  most 100, `covered + 1` at most 101 (u8), under `after8`'s half range.
- **While dead:** kinds 1-6 and 8-11 are refused by the room and pump, which never read the soldier's state, so a dead
  or just-respawned forger is refused the same. Only kind 7 depends on the sim's soldier (block B).
Verdict of block A: each forged kind is refused by the server's own state; nothing in the room trusts a client's claim.
The new code that changes state on a forged command is all client-side (npt-bots) or harness-side (npt-lab).

## Block B: the rebuilt kind 7 and F-B from the hostile side (done)
- **The construction at the built rate** (npt-sim `TICK_HZ` = 30, ticks.rs:12; `UNITS_PER_TICK` 1,000,000,
  seam.rs:177; SS's interval 68,571 µs, scene.rs:64, so a shot costs 2,057,130 units; the weapon clock advances once per
  command run, world.rs:1416): after a shot `wait` lies in [1,057,131, 2,057,130], so the weapon is ready again after
  2 or 3 commands. `FIRE_REST` 3 released commands (lib.rs:153, :892, :915-918) therefore always leave it ready for the
  honest press at d - 1. One command later `wait` lies in [57,131, 1,057,130] > 0, so the forged press at d always
  falls inside the cooldown and gets a `Cooldown` record (world.rs:1464-1471). Reload items: the honest edge at d - 1
  starts a 60-tick reload (world.rs:1426-1435; `reload_ms` 2,000), the forged edge at d is refused `Reload`; fire items
  that fall inside that reload are refused `Cooldown` (not ready, ammo above 0). SS's magazine is 32,000 (scene.rs:67),
  so an `Ammunition` record (which the kind-7 rule does not count) does not arise. **Sound at 30 Hz, alive, at the
  judged 12 and at 3** (at 3 fire is released on every command and the presses come 3 commands apart: same
  arithmetic). **Not sound at 2:** SA-U4F2-2.
- **What proves it:** f15's kind-7 case (lib.rs:1781-1836) checks the shape (one edge, an honest edge before, three
  rest commands), not the sim's verdict. The sim's verdict is proven only by the runs (f16 at 2 s + 4 s: unrejected 0;
  the 610 s run: 18 of 1,523, F-B) and by the arithmetic above. No unit case ties `FIRE_REST` to the weapon's interval.
- **Rate dependence (for the rust and netcode seats, not a security finding):** `FIRE_REST` is a count of commands
  that must cover the weapon's interval. At 60 Hz (cost 4,114,260 units) 3 commands do not: when the last held shot
  falls at d - 4, the honest press at d - 1 is refused and the forged press at d can fire, unrecorded. The code's doc
  says "Counts of items and commands, no time" (lib.rs:151). Only a `TICK_HZ` edit reaches it, and it gives a false
  FAIL, never a false pass. Whether `FIRE_REST` is a tick-counted value under carry 39 is the rust seat's call
  (check.mjs:760's name rule cannot see it). SA-U4F2-2's fix (b) would close it too.
- **F-B, a forged edge while dead:** world.rs:1346-1355: the press counters are claimed (`self.claimed[slot] =
  c.presses`) before the `!s.alive` return, and nothing else changes. So a hostile client's presses while dead are
  consumed, never banked for a burst at respawn, and the command has no effect. The server is right; only the record
  is missing, so the attribution counts the item unrejected (a false FAIL).
- **F-B, a forged edge just after a respawn:** world.rs:1194-1202 resets the weapon whole (magazine full, `wait` 0,
  `reload_left` 0); respawns run in phase 7 (world.rs:1802-1810), after the tick's commands. If the forger is dead at
  d - 1 and respawned before d, the honest press goes unfired. At d the steady script holds `FIRE` (lib.rs:1049-1054;
  the rest covers only d - 3 to d - 1), so the fresh weapon fires at d **with or without the forged edge**. The twin's
  command at d (fire held, no edge) fires the same shot. So the edge adds nothing beyond the valid part, and the twin-hash
  check (marks.json:4970-4979) is unaffected. There is no `Cooldown` record because the weapon was ready, so the
  attribution counts the item unrejected. A forged reload edge after a respawn meets a full magazine and is refused
  `Reload` (counted rejected).
- **Weighing F-B:** no security defect in either half. Both are the client's belief about the server's state going
  stale, and the server decides from its own state each time. SS's 1,000 ms respawn (scene.rs:186) and the builder's
  5,300-6,200 dead soldier-ticks per 610 s (about 120 per soldier, about 4 deaths) fit 18 items of 1,523 mostly sent
  while dead. That reading is unverified. For -21's set F-B still blocks "every forged command rejected and counted"
  (marks.json:4958-4969, max 0), so the builder's High for the set stands as a test-validity matter for the netcode
  seat.
- **The fix is not this seat's, but two limits apply.** (1) Any fix that changes what the attribution counts as
  rejected (for example "ran while dead" or "absorbed by a ready, held trigger") changes kind 7's rule in 2.4 item 6.
  By C-U4F2-10's own words the own-kind rule "would be his" (TEST-FIRST), so that fix is Mustafa's, through the
  advisor. (2) A forger-side fix (forge kind 7 only while its own soldier shows alive in the view) narrows the gap but
  cannot close it, because the view lags the tick the command runs at. It also feeds decoded state into an open-loop
  script (u4-build-plan.md:411-412's twin rule): netcode's call.
- **The run that would verify F-B's reading (for the fix pass):** a 610 s forged run (seed 1, option A, release, as
  u4f2-hand-long2) that logs, for slot 6's items, the tick each ran and its soldier's alive flag at that tick and the
  one before. Expected: every one of the unrejected items ran while dead, or ran with the weapon fresh from a respawn
  before it; none ran alive with a stale cooldown.
- **Detection note (security, no finding):** a dead client's forged edges leave no refusal record, so the room's
  refusal counts (the anti-cheat signal) miss them. They have no effect, so only telemetry is lost.

## Block C: the attribution, SEC-1, F-A (done)
- **Matching** (forged.rs:74-131): room kinds 1, 2, 4, 5, 6 and 10 count rejected only with a record of the forger's
  own kind for (slot, seq) **and** no run of (slot, seq) all run long. Kind 3 also needs the copy's content never run
  and the original run at most once. Kind 7 needs a sim `Cooldown`/`Reload` record (the command runs by design), and
  the twin hash backs it up: a defect that recorded and fired would differ from the twin. Records naming unforged
  sequences change nothing (Q4). A hostile honest-looking refusal on the forger's slot cannot mark a forged item
  rejected unless it names that item's sequence under its kind, and a forged item that runs is unrejected whatever the
  records say.
- **Bounds:** room records 4,096 per tick, overflow counted (u64). The lab's tallies are at most 65,536 sequences per
  forger (the u16 key) and 11 forgers. The `refusals_dropped_count` (SEC-1) is cumulative over the run and logged on
  every `forged_kind` record (patch :1573-1576; SERIES.md's new row). No mark reads it, and none needs to: a dropped
  record can only raise `unrejected_count`. The long run reads 0 (u4f2-hand-long2's lab.jsonl).
- **Cutoff and order** (scenario.rs:1349-1353, :1563-1579, :1657-1660): items register only while `k < cutoff`;
  kinds 8, 9 and 11's counters are read once, right after the tick that makes `k == cutoff`, before any later client
  step, so no later drop enters the count.
- **Kinds 8, 9, 11 by count:** `sent.saturating_sub(counted)` (forged.rs:116) clips an over-count to 0 without
  showing it: SA-U4F2-3.
- **The judged interval:** marks.json reads no `forge_every_count`; only the run scripts pin 12 (c-u4f-runs.mjs
  PARTS_21; c-u4f-r2.mjs FORGED). The header records the value, and a run at another interval still has to send 1,000
  of each kind. No finding: pinning it in the judge would be Mustafa's.
- **F-A from the hostile side:** with the lag client standing, "no tick moves further than the catch-up cap allows"
  (marks.json:5062-5073) cannot fail by movement, and p13 already had it passing under the cap bite (TE-7). The
  defence itself, the per-tick cap, is still proven, on the forged suite. -21's bite `PM-0001-21/no-catchup-cap` runs
  there (c-u4f-runs.mjs NO_STAR, part "forged"), and its hand run reads kind 10 unrejected 32 of 32, every other kind
  0 (.gate/u4f/u4f2-hand/bites/bite-no-catchup-cap/lab.jsonl). So F-A costs the lag check its power, which is the
  netcode seat's (agree Medium for the test), and leaves no anti-cheat gap in the room.

## Findings
- **SA-U4F2-1 (Low): `--forge-every` has no upper bound, and kind 10 multiplies it unchecked.** bots
  lib.rs:944 `let period = f.every * u64::from(f.burst_refused());` with the parser at scenario.rs:2240-2244
  (`parse::<u64>()`, `k >= 2`, no maximum). Above `u64::MAX / 8` (A) or `/ 4` (B) the product overflows. A debug lab
  (the tests' build) panics with "attempt to multiply with overflow". A release lab wraps, because npt-bots has no
  release overflow checks (Cargo.toml:116-120 name only npt-sim and npt-proto). So at 2^61 kind 10 never forges
  (`is_multiple_of(0)` is false for `given > 0`), and at 2^61 + 1 under A it forges every 8 commands while the header's
  `forge_every_count` says 2^61 + 1. Exploit sketch: an operator's typo or script value; no network reach, and no
  false pass (every other kind sends 0, so "1,000 commands of every kind" fails). Fix: bound the parser as the
  neighbouring options are bounded (scenario.rs:2225, :2229: "at most an hour"), for example at most 1,000,000 commands.
  Make `forger()` return `None` (a usage error) when `every.checked_mul(8)` is `None`. Add "18446744073709551615" to
  c_u4f2_1's refused list. About 4 to 6 lines.
- **SA-U4F2-2 (Low): `--forge-every 2` is taken, but the rebuilt kind 7 is not sound at 2.** At 2, `rest`
  (lib.rs:892) releases fire on every command, the honest presses come at every odd command and the forged ones at every
  even one. After each shot `wait` gains 2,057,130 units and loses 2 x 1,000,000 by the next honest press, so its floor
  drifts by 57,130 a shot. About one fire item in 17 or 18 then finds the honest press refused (an honest `Cooldown`)
  and the forged press one command later ready: it fires, unrecorded, and counts unrejected. Exploit sketch: none (a
  false FAIL of the harness, never a pass); the judged 12 and f15's 3 are sound. Fix, one of: (a) `forger(7, every, _)`
  returns `None` below 3 (a usage error naming kind 7's floor), with the parser's case; (b) derive the rest from the
  scene's weapon (`ceil(interval_us x TICK_HZ / 1e6)` commands, read once in `Behaviour::script`) and require `every`
  above it, which also closes the 60 Hz note in block B. The parser's floor of 2 is C-U4F2-1's ruled text, so moving it
  to 3 is the advisor's; (a) leaves the parser alone. About 3 to 8 lines. Verify (fix pass): a 16-client 2 s + 4 s
  forged run at P0, option A, `--forge-every 2`: kind 7's `unrejected_count` above 0 by this reading; at 3, 0.
- **SA-U4F2-3 (Low, hardening): kinds 8, 9 and 11's count clips an over-count silently.** forged.rs:113-116
  `8 | 9 | 11 => sent.saturating_sub(f.counted)`, with f33's `(3, 0)` case pinning the clip (forged.rs:285-295). If the
  forger's session counter ever rose for a datagram that was not a forged item, the surplus would hide an accepted
  forged item: a false pass. No path does that today (the forgers' honest datagrams never meet `ack_refused`,
  trailing or unknown-field, or wrong-channel drops), so this is defence in depth. Fix: write the counter beside it
  (`counted_count` on kinds 8, 9 and 11's `forged_kind` record, one field in SERIES.md's new U4f2 row) and have f16
  assert `counted_count == sent_count` for those kinds at P0. About 6 to 10 lines. Under the review budget a Low alone
  triggers no push: a STATUS follow-up if not taken.

## Anti-cheat coverage (hostile action -> where the server validates it -> gap)
| Hostile action | Where the server refuses it (its own state) | Gap |
|---|---|---|
| Kind 1: command stamped beyond the lead bound | room input.rs:266-268, `next_tick` + `lead_bound` | none |
| Kind 2: stamp not newer than the newest accepted | input.rs:269-276, the newest accepted / join anchor | none |
| Kind 3: changed copy of an accepted sequence | input.rs:238-260, the kept window's content and accepted flag | none |
| Kind 3 variant: resend a refused sequence | input.rs:233, :243: kept as refused, later copies are `Repeat` | none |
| Kind 4: move vector too long | input.rs:281-285, widened to i32 | none |
| Kind 5: more press edges than ticks covered | input.rs:287-300, the counters' forward rule | none |
| Kind 6: aim outside the map | input.rs:302-306, the room's extent | none |
| Kind 7: fire edge in the cooldown, reload edge in a reload | sim world.rs:1412-1499, `wait` / `reload_left` / `ammo` | none in the server; while dead no record is written (no effect; telemetry only, F-B) |
| Presses banked while dead for a burst at respawn | world.rs:1351 claims the counters before the alive check | none |
| Kind 8: acknowledgement of a tick never sent | room.rs:961-1000, ack.rs:145-171 (no mutation on `NeverSent`) | none |
| Kind 9: trailing byte or set pad bit | proto bits.rs:132-151, strict decoder | none |
| Kind 10: burst past the per-tick cap or B's buffer | input.rs:316-325, the room's buffer length | none; proven by `no-catchup-cap` on the forged suite |
| Lag switch (hold, then burst) | the same cap; Input bucket; W | the movement check cannot fail with a standing client (F-A); the cap is still bitten via kind 10 |
| Kind 11: datagram type on the control stream | the pump's wrong-channel drop | none |
| Refusal-record flood | room.rs:944-950, 4,096 a tick, overflow counted | none (a drop can only fail the attribution, never pass it) |
| Weapon clock advanced by extra commands (option A) | lead bound + kind 2 + cap; -21's fast-clock checks | pre-existing design (ADR 0001), not new in U4f2 |

## For the main session
- Durable points for this seat's memory (the caller limited this agent to one file, so they are recorded here): the
  forged suite adds no server path; kind 7 is the only alive-dependent kind; a dead soldier's presses are consumed,
  not banked (world.rs:1351); the attribution errs toward false FAIL except kinds 8, 9 and 11's clip (SA-U4F2-3).
