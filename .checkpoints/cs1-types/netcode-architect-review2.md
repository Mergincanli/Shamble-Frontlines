# netcode-architect, round 2 review of cs1-types (code at 65a7b13; round 1 reviewed 82142fa)

Read-only review. No source edits. Never `spikes/`, never /root/.claude/uploads/.

## Plan
1. Diff 82142fa..65a7b13 for crates/protocol/src/messages.rs and lib.rs (doc lines only); confirm HEAD's copies equal
   65a7b13's; confirm no type, name or width changed.
2. Read fix-pass-1.md "NA 1 to 6" and "NA 7" (done).
3. For each new doc sentence: check against A1 parts 5, 6, 10 and the message table, A4 parts 2 and 4; flag overclaims
   (the five the brief names, and any other).
4. L7: accept or not.
5. The provisional paragraph against my round-1 list (the handback of round 1 and notes N1-N6 + notes lines).
6. Verdict + findings (file:line, draft line, fix-now or follow-up), at most 15 lines.

## Findings (one line each, as found)
Keys: A1 = docs/adr/0001-multiplayer-netcode.md, A2 = 0002, A4 = 0004. The caller's L-numbers: L1 AimPoint (my r1 N1),
L2 counters wrap (r1 N3), L3 sequence (r1 N2), L4 rates (r1 N4), L5 commands (r1 N5), L6 slot/squad (r1 N6), L7 const.
- (verified) 82142fa..65a7b13 changes only `///` and `//!` lines in messages.rs and lib.rs (a grep of the changed lines
  without doc markers is empty); HEAD (7907e43) holds 65a7b13's crates/protocol unchanged; A1 and A4 unchanged since
  round 1. The six structs whose docs changed have regenerated TS files (AimPoint, Command, Input, JoinAccepted,
  Snapshot, ViewTime), each carrying the new words; Join.ts and SnapshotAck.ts rightly unchanged.
- L1 closed: messages.rs:94-105. "(ADR 0001 part 5: "at most 80 bits")" = A1:226-227. "(part 6, E3)" = A1:279-280.
  "the codec sets them under PM-0001-51" holds: PM-0001-51 reads "the aim's bits ... from the codec's field widths" and
  bounds them (5 cm hit error, at most 80 bits before delta coding: A1:1648). (fix-pass-1.md:51's summary "PM-0001-51
  sets them" would overclaim; the code does not say that.) "The encoder clamps to the map" follows from A1:236 (the
  server checks the point lies inside the map bounds); who clamps is no draft's, it is my r1 N1's proposal: accept.
  "the widths are borrowed" is exact for x and z only; the next sentence says y is wider than E3's 12 bits: accept.
- L2 closed: messages.rs:7-8 and :85. "compared only with the codec's wrap-aware comparison, never with `<`" holds:
  PM-0001-58's bite is "the counter compared with a plain `<` instead of its wrap-aware comparison" (A1:1607); "the
  codec's" is a placement, and the right one: a counter narrower on the wire than in its Rust type wraps at the wire
  width, which only the codec knows. "a sequence, a press counter" are examples; PM-0001-58 also lists tick offsets,
  event and acknowledgement numbers (codec-only today). Accept.
- L3 closed: messages.rs:77 "(the drafts say 16 bits or narrower)" = PM-0001-58, "every 16-bit or narrower counter the
  protocol carries (command sequence numbers, ...)" (A1:1607).
- L4 closed: messages.rs:41-46. "fixed when the game is built (ADR 0004 part 4)" = A4:186-189 ("dt is one build-time
  constant ... never a server option"); "never sets its clock from it" follows from the same lines; "the client checks
  this against its own" is my r1 N4's rule, which no draft states but which is the one use a build-time constant
  leaves: accept. "(ADR 0001 part 10)" for "under pressure the room sends fewer" is right: A1:450-451 ("falls back to
  20 Hz snapshots (two of every three ticks)"), and A1:428 (WebSocket: a snapshot that does not fit is skipped). "the
  client takes the time between snapshots from their ticks" follows from A1:318-319 and A1:332-333: accept.
- L5 closed as asked, with N1 below: messages.rs:56-57; the order is provisional and "codes each repeat against the
  newer command beside it" = A1:221-222.
- L6 closed: messages.rs:26-31. "the soldier's index in the simulation (ADR 0004 part 2)" = A4:155 ("A soldier's slot
  index is its sim index (GC2)"); "its width follows the wire's entity index" = A4:155-156 -> A1:314-316. Squad's
  numbering marked open.
- L7: the deferral is accepted. `buttons: u16` and `presses: [u8; 16]` are both provisional capacities; when cs3 fixes
  the button set, one `const` for the count plus a compile-time assert against `u16::BITS` ties them.
- N1 Low (follow-up, no push) messages.rs:56 "their sequence numbers consecutive" claims more than a draft says. A1:234
  says only "sequence numbers only move forward, wrap-around handled". Consecutive follows from one command per tick,
  every unacknowledged command repeated (A1:220-221) and the "newest received" acknowledgement (A1:228-229) only if a
  sequence also advances by exactly one per command, which no draft states (my r1 N5 asked for the word: I overreached).
  Words I accept: "The commands not acknowledged yet. Provisional: their order (which end is the newest) and whether
  their sequence numbers must be consecutive come with the codec, which codes each repeat against the newer command
  beside it." Or: keep the line until cs3 and carry the item in the provisional list.
- N2 Low (report text, no code push; fold in while the report is written) the provisional paragraph
  (docs/reports/2026-11-18-cs1-types.md:170-174, report-draft.md:165-169) says "each marked in the code's doc line" but
  misses four of the code's own markers: the move vector's scale (messages.rs:79, 81: "its scale and width"); how
  squads are numbered (:31); the two command acknowledgements' width as well as their form (:127, :130); which axis is
  up (:102; the sim's convention, wider than `AimPoint`). And one codec decision outside the markers, which my r1 notes
  carried: the anchor of `Input`'s 16-bit tick offsets (A1:318-319 give the 32-bit tick to the snapshot header only;
  A1:776 "16-bit offsets elsewhere"; `Input` has no header tick). Plus N1's item if the line stays.
- N3 Low (closed in these notes) the report says "What settles each, and the checks the decoders must make, are in
  netcode-architect's notes"; my round-1 notes deferred both to the handback (review1.md:56), so the pointer led
  nowhere. Both are now below ("For change-set 3"); the report should name this file.
- (note) Command.view_time's doc (messages.rs:90, "when the command was made") is loose but right: the view time is the
  drawn frame's whose post the command was built from (A1:256-257, A2:320). No finding.

## For change-set 3: the provisional items, what settles each, and the decoder checks
Provisional (each with what settles it):
1. Join token's format: the lobby (slice 3); its checks are A1:685 (signature first, audience, expiry, unique id) and
   the replay cache keyed on the unique id (A1 part 11).
2. `protocol_version`'s width: the codec; it also rides in WebTransport's `protocols` option (A1:446-447).
3. `slot`'s width: the wire's entity-index width for soldiers (A4:155-156 -> A1 part 6), at N up to 54 (MODES-3 (b)).
4. `team`'s width: modes as data (A1 Q25); 3 teams to start.
5. `squad`'s width and numbering (per team or across the room): modes as data and the roster (A1 part 11).
6. `tick_rate_hz`'s width: the codec (the value is build-time: A4 part 4; 30 Hz under TICK-30, A1 part 2).
7. `snapshot_rate_hz`'s width: the codec (nominal 30 Hz, A1 part 2; 20 Hz fallback, A1:450-451).
8. Both `group` widths: the splitter's group count per tick (A1:293-294; Q11 P1), measured by the proof test.
9. `sequence`'s width: the codec, at 16 bits or narrower (PM-0001-58); whether consecutive (N1).
10. `presses`' width: the codec (U2; R2's small press counters); PM-0001-20 and PM-0001-58 (0 presses lost or doubled,
    across a wrap); the width must exceed the largest gap the server can see between two run commands (the redundancy
    cap plus the input buffer, A1:221 and A1:250-251), or a wrap aliases.
11. `move_x`/`move_y`'s scale and width: the codec; the length-at-most-1 check (A1:234); R2-XU-OQ4.
12. `mode_hash`: what it hashes, and whether it stays (only A1:686 names it): ADR 0005's content hashing and A1 Q25.
13. Which buttons exist (16 is a capacity): the systems that read them, from slice 1.
14. `AimPoint` as a whole: the codec, checked by PM-0001-51 (5 cm, 80 bits); which axis is up: ADR 0004's convention.
15. `ViewTime.fraction`: the sim's sub-tick unit (A4 part 4: set in the sim core spike from the rounding report);
    OQ-0001-7 (sub-tick fire stamps) may change it.
16. The two acknowledgements' form (sequence or tick) and width: OQ-0001-6 (under (b), "repeating the last on a miss",
    the replay keys on ticks).
17. The order of `Input.commands`: the codec (U2, A1:221-222); R2-XU-OQ4 measures both forms.
18. The anchor of `Input`'s 16-bit tick offsets (`Command.tick`, `ViewTime.tick`, `SnapshotAck.tick`): the codec
    (options: the room's own tick window, nearest value wrap-aware; or the acknowledged snapshot's tick).
Notes for the codec: release counters: A2:319-320's page post holds press and release edge counters, "their exact
fields are ADR 0001's", and A1:226 names press counters only, so the command follows A1 unless A1 changes. One
baseline per tick for all group datagrams (A1:293-294) or one per group (A1:892): the types fit both. Later: the
derived `Eq` on `Snapshot` ends if the own-state section is sent unquantised (A1:207-208).
Decoder checks:
- Every decoder returns an error, never panics, and checks sizes before it allocates (A1:445-446); the sim thread never
  parses raw bytes; a datagram with trailing bytes or unknown fields is dropped whole (A1:238).
- `Input`: datagram size at most the command size times the redundancy cap (A1:453-454); the command count at most the
  cap (data, proposed 8; the count field's width caps it too); acks at most the room's group count; each command's tick
  not too far ahead and strictly newer than the last run (A1:231-232); sequences forward only, wrap-aware (A1:234);
  move length at most 1 (A1:234); a press counter advances at most one edge per button per tick covered (A1:234-235);
  the aim point inside the map bounds, clamped to reach, never refused for reach (A1:236); the acknowledged snapshot
  one the server sent (A1:237); commands run per client per tick capped, catch-up cap at most the redundancy cap
  (A1:238-239).
- `Join`: within 2 s (A1:454); the token's length capped before allocating (data); token checks per A1:685; a
  redacting `Debug` once the token is a credential; never the derived `==` to authenticate (the room verifies the
  signature and keys its replay cache on the unique id).
- Client side: `mode_id`'s length capped; the content hash compared with the client's own; `tick_rate_hz` checked
  against the build; `Option` tags valid; snapshot ticks rebuilt from the 32-bit header tick; what the client cannot
  decode is dropped (A1:683).
- Every counter compared only wrap-aware at its wire width (PM-0001-58's bite).

## Verdict
APPROVE. No finding at Medium or above; nothing to push. L1-L6 closed, L7's deferral accepted; N1-N3 Low (N1 a
follow-up or a list item, N2 report text, N3 closed here).
Not checked: spikes/ (not read); no cargo or ts-rs run; the generated files only by grep for the new words.
DONE 2026-11-18
