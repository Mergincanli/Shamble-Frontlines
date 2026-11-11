# U4d round 2 + npt-proto (W1-S1): security-auditor notes

Copy under review: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597, nothing committed). Read only.
Scope: W1-S1 only (SEC-U4a-1 leak, SEC-U4a-2 codec enforcement). Rest of U4d: not reviewed.
(Process note: the first appends to this file went through a shell heredoc, before I noticed the brief's "Write and
Edit tools only"; this whole file was then rewritten with Write, line refs corrected. No other file touched.)

## Plan
1. Read rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304, the parts on SOLDIER/OwnState), W1-PUB (:353-436).
2. Read round-1 security findings: u4ab-review-security.md SEC-U4a-1, SEC-U4a-2 (:34-45); u4ab-round2-security-auditor.md.
3. Read catalogue §3 rows :136-137 (own-state, remote soldier).
4. Read crates/sim/src/seam.rs (SOLDIER, OwnState) and the world's record writer.
5. Read crates/proto diff: src/snapshot.rs (every encode/decode path, kind check before use), src/error.rs, tests.
6. Re-read s1-proto.md plants P1-P8 against the code: does each plant fail by name?
7. Hostile-datagram scenarios: owner-only record in entity section; spatial record in own-state section; add/update/remove; FullState.
8. Open items for U4e (encoder chooses sections; class filter).
9. Verdict.

## Progress
- 1-3 done: W1-S1 (:45-111) read; round-1 SEC-U4a-1/-2 (u4ab-review-security.md:34-45) and round-2 carries read;
  catalogue :136 (own state: position+velocity, weapon state, attribute rows) and :137 (remote: pos 16/16/12, yaw 16,
  pitch 10, health, 8 flags, weapon id) read.
- 4 done (sim): seam.rs:249 SOLDIER = [x, y, z, YAW, PITCH, FLAGS, HEALTH, WEAPON] (8 fields: the remote row exactly);
  seam.rs:252-261 OWN_STATE = 3x VELOCITY, AMMO, FIRE_TIMER, RELOAD_LEFT, MOVE_SPEED, FIRE_INTERVAL; OwnState last in
  ALL (seam.rs:289-298), KIND_BITS stays 3, FIELDS 12. world.rs:1743-1762 soldier_record writes pos, yaw, pitch,
  flags, health, weapon 0: no velocity, no ammo/timer/reload. world.rs:1767-1786 own_state_record holds them, index =
  slot, generation = soldier's. Flags (world.rs:117-120): crouch, sprint, reloading, dead; no fired/empty/cooldown bit
  (ADR 0001:310-311 lists reloading among the remote flags). write_records (world.rs:1790-1898) pushes one OwnState per
  slot last; predict (world.rs:999-1009) refreshes both of the slot's records. s6 (tests/seam.rs:458-515; SOLDIER pin
  :485-495, OWN_STATE pin :503-515) pins both by Field identity, not only width. The own-state values are server-derived
  from commands only (world.rs:1370-1402 weapon; :1116-1118 scripted attributes clamped on entry; :1143-1145 spawn).
- 5 done (proto): carries() snapshot.rs:175-179 (own section = Owner rows' kinds; entity section = every other row's).
  Checks before any write or further read: put_record :229, get_record :245 (right after get_kind), put_delta :274-280
  (kind of Add/Update/Remove, before the match), get_delta :338 (before index/op). Callers: Snapshot::put :544-549,
  get :591-608, FullState::put :681-686, get :702-708; no other caller (grep). GROUPS: OwnState only in the two
  "own state" Owner rows (:491-499, :647-655); Effect likewise. Errors: EncodeError::OutOfRange, DecodeError::Malformed
  (error.rs docs :33-36, :43-47). Encode truncates on error (msg.rs:319). Snapshot/FullState decode only RoomToClient
  (msg.rs:264 direction check): a hostile mover is a spoofed or hostile server, refused as Malformed; no server-side
  exposure.
- 6 done: plants P1-P8 (s1-proto.md) re-read against the code. P1-P4 each switch off one of the four check sites
  (get_delta :338, get_record :245, put_delta :278, put_record :229); w1_s1 (tests/wire.rs:188-320) hits each: the
  encoder half tries Add, Update and Remove in both sections of Snapshot and the record in both of FullState (28
  refusals, nothing appended: out == [0xee]); the decoder half re-encodes a valid one-record message and swaps the two
  section counts after asserting their offset (14 refusals, 7 single-section kinds x 2 messages; :319). The named first
  failures (Vehicle) match ALL's order (Soldier is in both sections, skipped). P5/P6 hit the GROUPS test's home and
  owner-only checks (snapshot.rs:726-740; it loops over both tables); P7 hits the width check (:747; velocity's 48
  bits); P8 the -57 bites test. Not planted but covered by reading: a call site passing the wrong Section (fails a1 or
  w1_s1 both ways); carries() itself always true (fails all four). The decoder test moves Adds only, but the check sits
  before the op is read (:337-342), so Update/Remove take the same line. The check is unconditional (not under
  strict()): bites builds keep it.
- 7 done: hostile datagrams. An OwnState/Effect record moved into the entity section, or a Vehicle/Part/Projectile/
  Door/Wall moved into the own-state section, in any op, in Snapshot or FullState: Malformed right after the 3-bit kind,
  before index, op, generation or any field is read; the partial Vec is dropped with the error. No new allocation, no
  new index or unwrap in non-test code; carries() is <= 12 rows x <= 6 kinds per record inside the 1,150 B bound.
- 8 done: U4e's path (plan §5 U4e :703-735; catalogue item 5 :251-254) read; carries below.

## Verdicts on round-1 findings
- SEC-U4a-1 CLOSED: SOLDIER is the catalogue's remote row exactly (seam.rs:249; world.rs:1743-1762), pinned by Field
  identity (tests/seam.rs:485-495) and by wire width (snapshot.rs:741-771; widths :747, 129 bits :771); velocity, ammo,
  fire timer, reload left and the two attributes live only in OwnState (seam.rs:252-261; world.rs:1767-1786), which only
  Owner rows list (snapshot.rs:491-499, :647-655) and the codec refuses elsewhere. Flags: crouch, sprint, reloading,
  dead (world.rs:117-120); FLAG_FIRED gone. Remaining half is U4e's (slot filter): carry below.
- SEC-U4a-2 CLOSED: refusal both ways at all four sites, before use, existing error kinds (EncodeError::OutOfRange,
  DecodeError::Malformed; error.rs:33-36, :43-47); the tests pin it (w1_s1), and the old pins that sent any kind in
  either section (common/mod.rs generators, a1) now draw from each section's own rows.
- SEC-U4a-4, -5 (U4a round-1 Lows): not claimed by this change; unchanged open follow-ups (U4a's).

## New findings (round 2)
- SEC-U4d-R2-1 Low, not before merge (U4e carry): an Effect record's owner is index / effect_rows (world.rs:1879-1881),
  but effect_rows is pub(crate) (scene.rs:119) and the public doc says "its owner is its index / 8" (seam.rs:283). U4e's
  owner filter must then copy the 8 from a doc. Scenario: a later scene with 4 rows per soldier; the room's filter
  (index / 8) gives slot k's client the rows of slots 2k and 2k+1 (owner-class leak). Today every scene has 8 or 0
  (scene.rs:164, :190, :213): no reach today. Proof: none possible on today's scenes. Fix: one public home for the
  mapping (Scene::effect_rows() or a pub const rows-per-soldier with a const assert), which U4e's filter reads.

## Carries for U4e (not findings against this change)
- The codec checks kind per section, not slot per client: any slot's Soldier, OwnState or Effect record passes in the
  own-state section. U4e's encoder puts only the receiving slot's records there (Soldier and OwnState index == slot,
  generation the soldier's; Effect owner == slot), and catalogue item 5's L1 check reads record identity in the own
  section, not just the section. Its owner-to-all bite must take the route the codec leaves open (other slots' own
  records in every client's own section): the entity-section route now ends in Err(OutOfRange), which proves the codec,
  not the class filter.
- The room handles Err(OutOfRange) from encode (a section mix-up, an out-of-range integer) without panic or expect:
  dropped and counted, the client's snapshot skipped for that tick.
- Team and Public rows count as the entity section in carries(); they are empty today. A kind in a Team row later
  needs U4e's class filter (team only): the codec cannot tell teams.
- Merge-time items the ruling names (not in this copy, due at the merge): the catalogue's own-state row gains the
  owner-only record (advisor-u4-wave1.md:107-108); U4e's brief rev 3 states the kind and the own-section-only rule (:109).

## Fixer's open points (weighed)
- (A), (B): not reviewed (stand-in tuning and statistics; no security bearing).
- (C): 8 is right. The ruling's list names 8 values (x, y, z, yaw, pitch, flags, health, weapon id), catalogue :137 the
  same 8: "(9 fields)" is a slip, advisor to confirm. `Weapon`, `Scene::weapon`, `Scene::health` (scene.rs:290-298) are
  read-only accessors of code-built scene constants, no network or content input: security-neutral; listing them is
  W1-PUB's (advisor). Carry 13: not mine.

## Considered, no finding
- The GROUPS test's remote-row check compares widths only (a same-width swap, e.g. AMMO's 16 bits for YAW, would pass
  it), but s6 pins SOLDIER by Field identity, so the two together hold the row.
- Health in the remote row and FLAG_RELOADING are the catalogue's and ADR 0001's choices (:137; :310-311).
- The section refusal on the client protects only against a bad server or a spoofed one (TLS covers the latter); the
  server never decodes Snapshot or FullState.

## Anti-cheat coverage (W1-S1's path)
| What a client could want | Where it is held | Gap |
|---|---|---|
| Read an enemy's velocity | not in SOLDIER (seam.rs:249); OwnState only in Owner rows; codec refuses it in entities | none (U4e: slot filter) |
| Read an enemy's ammo, fire timer, reload left | same (seam.rs:252-261) | none (U4e: slot filter) |
| Read an enemy's attributes (move speed, fire interval) | same | none (U4e: slot filter) |
| Read an enemy's effect rows | Effect only in Owner rows; codec refuses it in entities | owner mapping not public (SEC-U4d-R2-1, Low) |
| Infer weapon state from flags | flags: crouch, sprint, reloading, dead; no fired, empty or cooldown bit | none |
| Report its own ammo/timer/velocity | never accepted: World derives them from commands (world.rs:1370-1402) | none |
| Feed the client a mixed section (hostile server) | decoder refuses as Malformed before use | none |

## Verdict
APPROVE WITH FINDINGS (W1-S1's part only). SEC-U4a-1 and SEC-U4a-2 CLOSED. One new Low (SEC-U4d-R2-1, a U4e carry,
not before merge). Rest of U4d: not reviewed. DONE 2026-11-11.
