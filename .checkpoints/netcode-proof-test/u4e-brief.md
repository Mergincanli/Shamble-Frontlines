# U4e builder brief: the room and the codec path (rev 3.3)

**The main session's fold before the check (2026-11-11 about 17:50 UTC):** 3.1 re-copied by script from u4s-carries.md
§U4e as it stands (carry 51's corrected span and the new carries 53-57, advisor-u4d-close.md B2); 3.2 rewritten to
record the rulings (B1.2, B1.3, B2); 7.2 item 3 marked done. Everything else is rev 3 as drafted.

**The main session's fold at U4j's merge (2026-11-11 about 20:40 UTC; rev 3.2):** U4j merged (4d2b0ef, d50396a), and its
merge numbered round 1's and round 2's carries (advisor-u4j-r2.md B5, B7.2). So 3.1 is re-copied by script from
u4s-carries.md §U4e (carry 61 added; carries :147-247), with a note for carry 61, carry 40's cite moved, and the count,
the return list's item 6 and section 0's notes updated; the lines that called U4j unmerged now say what merged (carry
54's note, the build rules, U4E-Q13's option (c)); R27, R28 and R38 are rechecked at d50396a. Nothing else changed.

**The main session's fold of the advisor's check (2026-11-11 about 21:20 UTC; rev 3.3):** the advisor checked rev 3.2
(advisor-u4e-brief.md: APPROVE on its conditions L1 to L4). By its L1 the main session folded, by script (u4e_fold3.py,
each phrase read from the ruling's lines), its findings A1 to A7 and its words for U4E-Q12 (c), U4E-Q13 (a) and U4E-Q14
(b); 3.1 is untouched. The plan's four edits and carry 66 are the main session's own commit (L2); the carry-51 commit
comes before launch (L3).

**Changes in rev 3.1** (test-engineer's check of rev 3, u4e-brief-r3-check.md: findings F1-F8 and its Lows; each
finding's line, done or not, in netcode-architect's notes, u4e-brief-r3-notes.md, "Revision for the check"):
- D1. F1: the fold carried into the body. Carry 53 states the own-section rule (2.1); e12 gains tests for carries 53,
  54 and 57, carry 56's measurement and carry 55's "met by adding no row"; the count is fifteen (3.1); the return
  list names 53-57 (section 6). Carry 51's home follows advisor-u4d-close.md B2 (2.7, 3.1's notes, R31, 7.2 item 2,
  section 6 item 8): the main session adds `Scene::effect_rows()` when U4e's build starts; "launch as is" is gone.
  C17 and C29 are marked superseded.
- D2. F2: every bite proof takes wave 1's two parts, a bites-only twin that passes and an in-place scratch switch that
  shows the clean test failing by name, so check.mjs stays green in both builds (2.6, 2.5, e5b, section 4, section 6
  item 4).
- D3. F3: the owner-to-all bite's proof is carry 53's per-record test (2.6).
- D4. F4: e12's carry-22 test gains the accepting edges.
- D5. F5: e7 asserts that the parts the mirror models ran (splits, priority skips, events carried and resent, a
  baseline other than the defaults).
- D6. F6: the three room-side checks -58's counters pass through get their clean rules and clean negative tests (2.1,
  e9).
- D7. F7: the 1 s gap's start is stated against the tick, and both alignments are run and reported (e5a, e5b, 2.5).
  One wording differs from the check's fix: neither alignment is named the worse in advance (the notes give the
  arithmetic). New open question U4E-Q14 (section 8).
- D8. F8: routes for carry 54's series row and carry 56's ES half (3.1's notes; 3.2).
- D9. The check's Lows, each a one-line text fix: stale cites and the plan's stamp (C1, C16, section 0, 1.4, 3.1); the
  build rules' provenance line, the ordinary build's touch and npt-server's bites feature (section 4 rules 2 and 3;
  2.4); test wording in e6, e9, e10, e11 and e12 (carries 21 and 50); the token compare shown by review (2.1);
  client-rounding's share (2.6); a `Pong` with an id not outstanding under "Plan silent" (2.4); Q13 (a)'s pin
  (section 8).

**Changes in rev 3** (rev 2 is the night draft at 0dccbf0, `.checkpoints/npt-u4e-brief/u4e-brief-draft.md`, written
before wave 1 merged). Each change names its source: a W1-U4E answer by Q number (advisor-u4-wave1.md:437-497), a
carry by number (u4s-carries.md), a recheck row by R number (section 7), a plan line (u4-build-plan.md as it stands).
- C1. Every citation re-read and re-pointed at the branch head (section 0). The W1 text batch moved every plan line;
  rev 2's df8975b cites are gone. Source: the plan file (1,070 lines; sha256 7375cdcf… since d7a54cd's line-neutral
  edits, which rev 3 stamped as a81b230f…), the merged code.
- C2. U4E-Q1 (a) applied: U4e plants and unit-shows -58/lt-cmd-seq and -58/lt-press (2.1, 2.6); the plan's Bites line
  now lists them (plan :729-731). Off the open list.
- C3. U4E-Q2 (c), widened, applied: U4e builds the client's rebuild of the header's 16-bit last-run sequence and shows
  the bite as "the rebuilt sequence names the wrong command"; all of -58 is judged after U4g (2.3, 2.6; plan §4.14
  :469, U4g :814-817; W1-N2). Off the open list.
- C4. U4E-Q3 (a) applied: npt-room gains a bites-only `BITES`; npt-server keeps its one list (schedule.rs:251,
  lib.rs:13) (2.1, 2.2, 2.8; plan §4.1 :194). Off the open list.
- C5. U4E-Q4 (a) applied: -17/tick16 and -58/tick16 stay in `npt_proto::BITES`; the endpoint (and the room's encoder)
  takes its `Quant` from `Quant::with_bite`; carry 24's new words quoted (2.3, 2.6, section 3; R11). Off the open list.
- C6. U4E-Q5 (a) and its condition applied: no Rewind for a claimed view time whose tick the ring lacks, counted; the
  counter is a replacement in -30's setup (2.1; e12; carry 40). Off the open list.
- C7. U4E-Q6 (a) and its two conditions applied: the cap as data, provisional, 1,024 as F121's example; U4f measures
  (2.1). Off the open list.
- C8. U4E-Q7 and U4E-Q9: confirmed as answered (2.1, 2.2). Off the open list.
- C9. U4E-Q8 (a) applied: client/clippy.toml gains the hash-map bans and joins U4e's files (2.1, section 4; plan :710).
  Off the open list.
- C10. U4E-Q10 (a) applied: -62/server-echo planted in npt-room with "one event sent"; -45's ids and
  -62/cues-from-replay are U4g's (2.1, 2.6; plan :730-731; carry 24). Off the open list.
- C11. U4E-Q11 (a) applied: U4e builds the server-side checks and their unit tests; U4f adds their -21 counters and
  judged runs (section 2's scope, 2.2; plan U4f :793-794). Off the open list.
- C12. W1-S1 applied: the owner-only kind `EntityKind::OwnState`, sent only in its owner's own-state section (2.1;
  advisor-u4-wave1.md:109; catalogue :136; R32, R37).
- C13. W1-N2 applied: the header as built (16-bit command sequences, not tick offsets), the event-age carry (carry 50),
  the client's `event.counter_wrap` exposure (2.1, 2.3, 2.6; advisor-u4-wave1.md:305-349; R34).
- C14. W1-TARGET's rule in the build rules: the flock lock, the provenance check, 4 GB before a wasm32 or release
  build (section 4; advisor-u4-wave1.md:558-586).
- C15. W1-PUB's two U4e nits: one path for U4b's items in server/src/lib.rs (2.2; advisor-u4-wave1.md:385-386); the
  room's report names its route to the static graph (2.1; :410-411).
- C16. The carries re-quoted from u4s-carries.md §U4e as it stands (now :147-228): 24 reworded; 50 and 51 new; checked
  by script (section 3). 49 is not in §U4e (it sits in §U4r, now :240-243); 40 is U4r's and is cited where U4e feeds
  it.
- C17. U4d's round-2 U4e carries the advisor has not ruled on (netcode N2-L1, N2-L3 (c); security's "Carries for U4e";
  netcode N2-M2's U4e option) added as "pending the advisor's ruling (advisor-u4d-close.md)" (section 3.2).
  (Superseded by the fold, lines 3-5.)
- C18. The recheck table walked at the head: 8 of rev 2's 28 rows changed (R4, R6, R11, R15, R19, R20, R27, R28); 10
  rows added (R29-R38) for merged code the brief relies on (section 7.1).
- C19. Base counts filled: measured 365, bites 412, self 52 of 52; check.mjs 10 steps (section 4 rule 5; R25, R38;
  u4d-report.md:68-70). `<base>` stays a placeholder for the main session.
- C20. rev 2's `<floor>` filled: 2 GB for a build, 4 GB before a wasm32 or release build (section 4 rule 6;
  u4j-brief.md:43; W1-TARGET item 3).
- C21. The notes path confirmed against wave 1's builders (1.3; u4j-brief.md:18).
- C22. Write and Edit only, in wave 1's words: no shell redirects, heredocs or `python3 -c`; scratch plants in place,
  restored and sha256-checked; U4e's own bites are compiled in and need no planted copy (section 4; u4j-brief.md:46-48;
  plan :510-511).
- C23. tick16's unit proof reworded: the header carries the full 32-bit tick and the client reads it whole; the bite is
  the writer's (2.6; R11; snapshot.rs:532, :558).
- C24. The endpoint takes received messages as (Channel, bytes, arrival_ns), never `Recv` or `Link`: npt-client may
  name only `Channel` from npt-link (2.3; R29; check.mjs:809-837; plan.md:486).
- C25. U4e's sources never spell another crate's bite id; they forward the id string (2.6; R30; check.mjs:734-758).
- C26. The room logs `reports()`'s per-command refusal records beside the counters (2.1; R19; W1-U4D N-M2,
  seam.rs:497-531). W1-U4D's new carries name U4f, U4g, U4r and U5, none U4e (advisor-u4-wave1.md:232-235, :242-247,
  :274-276, :291-294).
- C27. The client's `BITES` pin in client/tests/bites.rs:10 follows U4e's new client ids (2.3; R33).
- C28. e10 notes that 32,000 / 30 is not a whole number of bytes, so the window's pin sums one second (2.4; advisor
  :447's "32,000 B a second at both rates").
- C29. e12 gains carry 50's and carry 51's tests; the return list covers carries 21-27, 39, 50 and 51 (2.4, section 6).
  (Superseded by the fold, lines 3-5, and by D1.)
- C30. Two new open questions where no ruling gives the answer: U4E-Q12 (-22/unbounded's planting unit) and U4E-Q13
  (the owner-to-all bite's id) (section 8; R27).
- C31. Start aligned with wave 1's briefs: fast-forward to `<base>` in the worktree (1.1; u4j-brief.md:16-17).

---

Drafted by netcode-architect on the night shift of 2026-11-10 (rev 1, ea2dd8a), checked by test-engineer and revised
(rev 2, 0dccbf0); brought up to date by netcode-architect on 2026-11-11 after wave 1 merged (rev 3); checked by
test-engineer (u4e-brief-r3-check.md) and revised once for that check (rev 3.1); rev 3.2 is the main session's fold at
U4j's merge, and rev 3.3, this file, its fold of the advisor's check (advisor-u4e-brief.md). Next: the advisor checks
the brief before the build (advisor-u4.md:157; Q21 at :220).

## 0. How to read the citations
- Every file:line is at the spike branch's head when rev 3 was written, 56214fa, whose spikes/ and docs/ are identical
  to 39f4290 (U4d's merge; `git diff --stat 39f4290 56214fa -- spikes/ docs/` is empty). Rev 3.1's new cites are at
  a28c962, whose spikes/ and docs/ are also identical to 39f4290. Wave 1 merged at 952f734 (U4a), 411eeee (U4b), f3f7c2f
  (U4c) and 39f4290 (U4d). U4j merged after rev 3.1 (4d2b0ef and d50396a; under spikes/ it changed only crates/judge,
  crates/log and marks/): rev 3.2's new cites are at d50396a, and the brief relies on U4j through carry 61 and rows R27,
  R28 and R38 only.
- `plan` is .checkpoints/netcode-proof-test/u4-build-plan.md, its cites to the version of sha256 7375cdcf… (1,070 lines;
  `git show 30528c7:.checkpoints/netcode-proof-test/u4-build-plan.md`): U4j's merge added a row after :199 (§4.1), three
  lines to §10's preamble and two to §11, so in the current file (4b8eaab6…, 1,076 lines) a cite from :200 to :953 is
  one line lower, and one from :964 on four lines lower; `carries` is u4s-carries.md; `advisor` is advisor-u4.md;
  `wave1` is advisor-u4-wave1.md; `close` is advisor-u4d-close.md; `plan.md` is plan.md (all in that folder);
  `catalogue` is docs/design/netcode-proof-messages.md; a crate path (`room/src/lib.rs`, `seam.rs`) is under
  spikes/netcode-proof/crates/ (`seam.rs`, `grid.rs`, `ticks.rs`, `history.rs`, `world.rs`, `scene.rs`, `greybox.rs` are
  sim/src/; `quant.rs`, `snapshot.rs`, `wrap.rs`, `hash.rs`, `msg.rs`, `input.rs`, `error.rs`, `limits.rs` are
  proto/src/); `check.mjs` is spikes/netcode-proof/check.mjs.

---

## 1. Start

1. **Base:** `<base>` (the main session fills it in at launch: the spike branch's head, which holds U4a, U4b and U4d,
   U4e's three dependencies: plan :703; Lanes, plan :859-863). In your worktree: `git merge --ff-only <base>`; if that
   cannot fast-forward, `git switch -c u4e-build <base>`; then `git log -1 --format=%H` shows `<base>`. Record it in
   your notes before anything else.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree (`isolation: worktree`),
   under §7.1's shared-target rule and W1-TARGET's lock (section 4; plan :892-904; plan §7.2 :906-909). U4e runs in
   wave 2 beside U4d2 (plan :859-863): two lanes, never sharing a file.
3. **Your notes:** `.checkpoints/netcode-proof-test/u4e-build.md` in your worktree (wave 1's builders used the same
   folder: u4j-brief.md:18). Its first line names this task; then your plan as blocks (one block: one type with its
   tests, one test group, one bite and its proof); each finished block written at once, a notes line per finding, and
   one line after each block (done, what next). On a restart, continue after the last block marked done.
4. **Read before you build** (and nothing you do not need):
   - plan §5 U4e (:703-734), the sections it builds to: §4.1 (:178-202), §4.2 (:204-248), §4.3 (:250-264), §4.4
     (:266-289), §4.5 (:291-303), §4.6 (:305-327), §4.7 (:329-335), §4.9 (:342-357, the integer half only), §4.11
     (:391-406), §4.14 (:452-475); Lanes (:836-871); §6 (:873-889); §7 (:891-914);
   - plan.md §4.7 (:568-612): each crate's whole public surface (npt-room :596-598, npt-server :599-600, npt-client
     :609); plan.md:454 (everything else `pub(crate)`); the endpoint's calls (plan.md:485-489); plan.md:514 (the
     room's checks);
   - advisor: the U4 verdict's conditions 6 (:125-129), 10 (:143-149) and 12 (:154-158) and nit (d) (:164-165);
     "Rulings (U4s)" (:349-375); "Ruling: F1 and F2" (:401-479), whose F2 part binds U4e (:436-465);
   - wave1: W1-S1 (:45-111), W1-N2 (:305-351), W1-PUB (:353-435, the U4b and U4d parts), W1-U4E (:437-497),
     W1-TARGET (:558-586); its W1-U4D (:113-303) only for the refusal records (:197-235);
   - close: B2 (:229-308, the U4e carries and carry 51's route) and its U4d-F part for U4e (:120-121);
   - carries §U4e (:147-247), quoted in section 3; carry 40 (§U4r, :250-258), which U4e's `rewind_at` counter feeds;
   - catalogue §1 (:52-78), the wire rows (:85-101), §3 Field groups (:125-157), §6 at U4 (:240-255);
   - .claude/rules/netcode.md: binding on all proto, room, client and server code (plan §7.1 item 8, :903-904);
   - the code as merged at `<base>`: sim/src/seam.rs, grid.rs, ticks.rs, history.rs, lib.rs, and World's and Scene's
     public items, `Scene::effect_rows()` among them once the main session adds it (carry 51; 2.7); proto (U4a's, with
     W1-S1's half); server/src/lib.rs and schedule.rs (U4b's); link/src/lib.rs (`Link`, `Loopback`); room, server and
     client as they stand (room/src/lib.rs:1-5 and server/src/lib.rs:1-14; client/src/lib.rs:8-22 holds `BITES` and
     `grid_with_bite`); check.mjs's deps step (:1563-1591), whose tick, float, bite-home and npt-link checks are
     :592-648, :668-682, :734-758 and :821-837, and its test step (`testBothBuilds`, :1493-1501).

---

## 2. What to build

U4e builds plan §5 U4e (:703-734) exactly: its Builds, its Tests, -18's bite end to end and its Bites. It moves no mark
(plan :732: "none judged (U4r runs them)"). Out of scope, by the plan: option A, `ClockSync` and every other room-side
check with its -21 counter (U4f, plan :792-793); the server-side checks' -21 counters and judged runs (U4f, :793-794:
"U4e builds and unit-tests the checks: U4E-Q11"); `Predictor`, `Interp`, `CueLog` (U4g, :805-809); `rewind_at`'s
tolerance and cap (U4h, :709, :821); AddrTable and the handshake cap (U6, :713, :878); token minting (U6, :705, :878);
fuzzing (U5, :874). Where this brief finds the plan silent it says so; it adds nothing.

### 2.1 npt-room (crates/room/src/, crates/room/clippy.toml)
Each item from plan :705-710, built to the section named.
- **`Room<S: SimCore>`** (§4.1 :194; plan.md:596-597): drives the sim through the seam alone (seam.rs:544-564); owns
  its `HitHistory` (built with `HitHistory::for_ms`, history.rs:29-39) and pushes `hit_volumes()` into it after each
  `step` (§4.2 :223-225; the push copies into the evicted entry: history.rs:59-74); fills one `TickInput` per tick
  (seam.rs:111-128): the accepted commands in the canonical order (seam.rs:116-121; carry 21), each `Command::slot`
  set from its session (the decoder writes 0: input.rs:27), each with the Rewind `rewind_at` gives it when it holds or
  presses fire (seam.rs:116-117), and the tick's spawns, teleports and attribute changes (seam.rs:122-127, integers
  only); consumes or copies `step`'s events before it reads `hit_volumes`, `reports` or `entity_records` (carry 26);
  reads `reports()` and logs, never into the hash, the clamp reports, the per-command refusal records
  (`Refusal { slot, seq, kind }`, cleared each step: seam.rs:497-531; the advisor's W1-U4D ruling, wave1 :210-213) and
  the five running counters (seam.rs:532-541) (§4.2 :211-213, :238-248). Bites per instance through `Room::with_bite`
  or a config value passed in, never a static (§4.11 :398-400).
- **`Roster`, the slot token compare, `ClientSlot`** (:705): tokens passed in by the caller (the lab at U4r; minting
  is U6's); the compare in constant time (catalogue :93), shown by review (its file:line in your report), not by a
  timing test; `bind` bound at the slot's first accepted `Join` (catalogue :93, :112-115); no join beyond the roster's
  N; the five JoinRejected reasons (msg.rs:112-118; catalogue :95).
- **`AckRing`** (§4.5 :292-297): ages 0 to 31 kept, 32 evicted, at 30 Hz; its length is `npt_sim::ACK_RING_TICKS` by
  path (ticks.rs:70); baseline B is the newest sent snapshot whose every group the client acknowledged (the `Input`'s
  `ack_tick`, its tick's low 16 bits, and `ack_groups`: input.rs:41-45, the tick rebuilt through `Wrap::widen16`); a
  partly acknowledged tick is never a baseline; a B older than the ring means the per-type defaults (the header's
  `baseline_age` 0: snapshot.rs:446-448).
- **`Encoder`** (:706-707): the window's remainder (§4.5 :300-302: the room owns each client's 256 kbps window; the
  pump's bytes sent since the last tick count first); groups with index and count, all against one B, each decoding
  alone (§4.4 :279-283; §4.5 :292-294); the layout table is npt-proto's `Snapshot::GROUPS` and `FullState::GROUPS`
  (snapshot.rs:488-517, :644-670), and the encoder builds none of its own (one home; R6); the class filter (catalogue
  item 5, :251-254); the mirror hash of §4.6 (:309-311) through npt-proto's `client_hash` (hash.rs:72-97); one tick's
  whole snapshot exposed as a unit (§4.5 :303); encoding through npt-proto's `EntityDelta` (snapshot.rs:63-149) and a
  `Quant` passed in (quant.rs:57-61; U4E-Q4), never a second mapping. Each burst yields `burst_eviction` (§4.5
  :298-299). U4e exposes `burst_eviction`, `snap_incomplete` and the client hash as counters and per-tick values, and
  U4r's per-process logs write them (§4.12 :409-410; U4r :739; npt-log's `hs` is U4j's: :645) (U4E-Q7, confirmed: wave1
  :480-481). An `Err(OutOfRange)` from the encoder is carry 54's: no panic, `unwrap` or `expect`; that client's snapshot
  for that tick is skipped (and, never sent, never a baseline) and counted per client; the next tick encodes afresh. The
  other `EncodeError` kinds (`TooMany`, `TooLarge`, `NotCanonical`: error.rs:42-58) take the same path, counted under
  their kind. A correct encoder meets none: it keeps each section within its count (own-state 8 bits, entities 16,
  events 8: snapshot.rs:26, :526-530) and each group within `Limits::SNAPSHOT` (1,000 B: limits.rs:25), so an event
  backlog (after a gap, up to the queue's cap) goes out over as many snapshots as the remainder needs, carry 50 still
  bounding its age.
- **The own-state section (the advisor's W1-S1, wave1 :109: "U4e's brief (rev 3) states the kind and that the room
  sends it only in its owner's own-state section").** The owner-only kind is `EntityKind::OwnState` (seam.rs:199-204):
  a soldier's velocity, ammunition, fire timer, reload left, move speed and fire interval (`OWN_STATE`,
  seam.rs:252-261), index the slot, generation the soldier's. The room sends it only in its owner's own-state section,
  with that owner's `Soldier` record and its effect rows (catalogue :136; `Snapshot::GROUPS`' own-state row,
  snapshot.rs:491-499). The codec refuses an `OwnState` or `Effect` record in the entity section
  (snapshot.rs:14-19, :175-180), but it cannot tell slots apart: which slot's records go into client c's own section
  is the room's choice alone. Carry 53 rules it: the room's encoder fills client c's own-state section with c's
  slot's records only (index = c's slot, that soldier's generation); its Effect rows by carry 51's mapping, read from
  `Scene::effect_rows()` (R31; 2.7). Its test: e12.
- **The window's rate (the F2 ruling, advisor :442-449):** the window's per-tick budget takes the tick rate as an
  argument; production callers pass `npt_sim::TICK_HZ` by path; a test pins it at 30 and 60 Hz (32,000 B a second at
  both); no const or static outside npt-sim is derived from TICK_HZ.
- **`EventQueue`** (§4.7 :330-333): bounded, its cap as data. The cap, by U4E-Q6 (a) (wave1 :473-475): "U4e builds the
  cap as data, with a provisional value from net F121's arithmetic, recorded as provisional (1,024 is F121's example,
  above the about 400 a B1 burst adds). U4f measures the rate and records §4.7's value before -22's runs." Its two
  conditions bind U4r and the judged runs (wave1 :476-479), so the room exposes what they read: per client, the relevant
  gameplay events it queued and the queue's high-water mark (condition 1), and every overflow resync, counted per client
  with its tick (condition 2; carry 57); the cap's value is exposed with its provisional mark, for the run's header
  (carry 57; U4r's header writes it). An overflow forces one resync; a FullState not acknowledged within
  `npt_sim::FULL_STATE_ACK_TICKS` (ticks.rs:56), or a second overflow inside it, sends `Kick::Overflow` (msg.rs:165) and
  drops the client; event numbers are 16-bit on the wire (snapshot.rs:31-32), rebuilt to 32 bits by `Wrap`; each
  `SimEvent` stamped with its tick and 32-bit number when queued (carry 23; the events at seam.rs:452-481 carry
  neither); no event queued past age 255 (carry 50; the age is 8 bits: snapshot.rs:31-32, :388-390). By U4E-Q12 (c), the
  cap is also PM-0001-22/unbounded's site (§4.14 :474; 2.6), so the room exposes, per client and tick, its queue's
  length beside the cap (the headroom, cap less length, signed: below 0 only under the bite), which U4f's -22 runs write
  as `tick.v.ack_starved_headroom_count` for the never-acking client (SERIES.md:21).
- **`InterestGrid`** (integer cm), **`AreaFlood`** (re-floods from `portal_states` when they change: §4.2 :215-216),
  **`Priority`** (catalogue :129-130: own state always, the nearest 8 enemies in every snapshot, every relevant entity
  at least every 4 snapshots). Float-free (§4.11 :396-397). The static graph: `World::greybox()` (world.rs:967) or
  `Greybox::dm_tri()` (greybox.rs:83), with `areas()` and `portals()` (greybox.rs:166, :171); your report says which
  route the room takes (W1-PUB, wave1 :410-411; R35).
- **`InputBuffer`, option B, with `buffer_depth_ticks`** (:708); `InputBuffer::CMD_CATCHUP` (carry 27; §4.3 :260;
  `Limits::REDUNDANCY_CAP` is a `usize`: limits.rs:17). Option A is U4f's (:792).
- **`rewind_at`, the integer half** (:708-709; §4.9 :347-349): a command's view time (seam.rs:39-43) to a `Rewind`
  (seam.rs:95-103); the debug assert that a correct room never sends a Rewind the ring does not hold sits here (§4.2
  :219-222); carry 22. The tolerance and the cap land in U4h (:709). **A claimed view time whose tick the ring does not
  hold, by U4E-Q5 (a) (wave1 :467-469):** "`rewind_at` gives the command no Rewind and the room counts it. A refused
  Rewind runs as none in the sim, so this is one rule. U4h then replaces it with the tolerance and the cap." The ring
  "does not hold" a view time when it lacks `tick_a`, or `fraction` is above 0 and it lacks `tick_a + 1`
  (seam.rs:87-90). Its condition (wave1 :470-471; carry 40's last sentence): that counter counts as a replacement in
  -30's setup condition, so the room exposes the count and each such command's (slot, seq) for U4r's per-shot log
  (plan :764-765).
- **The room-side checks -58's counters pass through** (:709): the sequence-forward compare, the press-edge compare
  and the ack rebuild (§4.14 :467, :468, :471), each through npt-proto's `Wrap` (§4.4 :284-285). `Wrap` takes each
  counter's name as a string and holds no list of names; the counters and their injectable start values are U4e's
  (wrap.rs:1-13, :43-77; R4). Their rules (plan.md:514; catalogue :87): a command whose sequence is not after the
  newest accepted one (`Wrap::after16`) is not run again; at most one press edge per button for each tick a command
  covers, the counter compared by `Wrap::after8`; an acknowledgement whose rebuilt tick names no snapshot sent to that
  client refuses that datagram's commands (§4.14 :471). U4e builds each with a clean negative unit test (a repeated
  command not run twice; two press edges for one button inside one covered tick refused; an ack naming a never-sent
  tick refusing its datagram's commands), and each counter accepted across its wrap in the clean build (e9); U4f adds
  their -21 counters (U4E-Q11). Each counter's wrap (its tick and the counter's name) is exposed for -58's
  `event.counter_wrap.tick_count` (marks/SERIES.md:150), which U4r's logs write. Their bites: lt-ack, and lt-cmd-seq
  and lt-press by U4E-Q1 (a) (wave1 :441-442: "post-wrap commands refused, and an edge lost") (2.6).
- **Resync, one in flight** (:709): a second `Resync` before the FullState's ack dropped and counted (catalogue :90).
- **The hash-map type bans** (:710: "room's and client's clippy.toml gain the hash-map type bans (§4.11; U4E-Q8)"):
  crates/room/clippy.toml (beside its clock ban, clippy.toml:1-9) and crates/client/clippy.toml (2.3) gain the
  `HashMap` and `HashSet` type bans (§4.11 :392; U4E-Q8 (a), wave1 :482-485).
- **Float arithmetic:** carry 25.
- **`npt_room::BITES`** (U4E-Q3 (a), wave1 :453-457; §4.1 :194): a bites-only list of the room's own ids
  (-18/no-fallback, -58/lt-ack, -58/lt-cmd-seq, -58/lt-press, -62/server-echo, -22/unbounded (U4E-Q12), and
  `catalogue-5/owner-to-all` (U4E-Q13)), named in your report as beyond plan.md §4.7 under that ruling. npt-proto's ids
  reach the room's encoder through `Quant::with_bite`, never spelled in the room's sources (R30).
- **-62/server-echo's site** (U4E-Q10 (a), wave1 :488-489): the room's event routing sends a cosmetic `Shot` to every
  client its test passes, never the shooter's own (catalogue :143; seam.rs:454-455).

### 2.2 npt-server's library (crates/server/src/, but schedule.rs, which is U4b's)
From plan :711-713; §4.4 :286-289 (A26); §4.5 :300-302; §4.9 :351-355.
- **`Decoder::pump`** (§4.1 :195): the I/O step over `&mut dyn Link` (link/src/lib.rs:64-88) with an injected `now`:
  decodes every inbound message through npt-proto's `Quant::decode` with a `Quant` passed in (msg.rs:391-; U4E-Q4's
  route for no-trailing and no-channel: R10); the socket checks, size first (`decode` checks the size before anything
  else: msg.rs:397-405; catalogue :65-71); the wrong channel, trailing bytes and unknown fields as npt-proto's
  `WrongChannel`, `TrailingBytes` and `UnknownField` (error.rs:7-8, :24-29), counted by kind by the pump, the only
  caller of the decoder on client input (§4.4 :286-289); the ping sender (a few a second: catalogue :91) and the
  answers with their hold time (`Pong::hold_us`, µs: msg.rs:56-65; catalogue :92; its limit below); per-client RTT
  (the link's `rtt_ns` for an L1 or `wt` session: §4.9 :351-353; Loopback's is `Some(0)`: link/src/lib.rs:203-205) and
  bytes sent per client since the last tick (payload + 28 B each: §4.5 :300-302). The `Decoder` holds no `Link`
  (§4.11 :401-402).
- **`ConnLimits`, `Counters`** (plan.md:599-600): the rate buckets (the Input bucket: 8 datagrams, refilled at 2 ×
  TICK_HZ a second: catalogue :87; the Ping limit: 10 a second per session: catalogue :91) and the 2 s Join deadline
  (catalogue :93), all on the injected now, in ns.
- **The server-side checks, by U4E-Q11 (a)** (wave1 :492-493): "U4e builds the server-side checks (size, trailing
  bytes, wrong channel, the bucket) and their unit tests, as its §5 says. U4f adds their -21 counters and the judged
  runs on U4e's pump."
- **The Input bucket's rate (the F2 ruling, advisor :442-449):** the bucket takes the tick rate as an argument;
  production callers pass `npt_sim::TICK_HZ` by path; a test pins the refill at 30 and 60 Hz (60 and 120 a second); no
  const or static outside npt-sim is derived from TICK_HZ.
- **The Pong hold time's limit** ("one tick period, 33.3 ms at 30 Hz": catalogue :92), by U4E-Q9 (confirmed, wave1
  :486-487: "The F2 ruling covers it: the rate is passed as an argument, with a pin at 30 and 60 Hz. No ruling
  needed."): the pump takes the rate as an argument, production callers pass `npt_sim::TICK_HZ` by path, and a test
  pins the limit at 30 and 60 Hz (e10); if the limit turns out to need a named constant, it takes route (b): no local
  form, the need in your report (advisor :449-458; carry 39).
- **U4b's module and one path (W1-PUB's nit, wave1 :385-386: "each item has two paths (`npt_server::schedule::X` and
  `npt_server::X`: lib.rs:10, :13-14). Keep one at U4e's next edit of server/src/lib.rs.").** U4b wrote
  `pub mod schedule;` and the re-exports (server/src/lib.rs:10, :12-14). Keep the module and schedule.rs as merged, and
  keep one path. The shorter route: keep the crate-root re-exports (§4.1 :195 lists the items there;
  server/tests/schedule.rs:5 and :347 use them) and make the module private (`mod schedule;`, the U4s ruling's words,
  advisor :369-370); lib.rs:7's intra-doc link to [`schedule`] then names a private module, which the doc step's
  `-D warnings` refuses (check.mjs:1472-1490), so reword that line. Say in your report which path you kept.
- **`npt_server::BITES`** (U4E-Q3 (a), wave1 :453-455: "U4e adds its pump's ids to that list rather than making a
  second"): U4e's pump plants no id of its own (its two bites are npt-proto's: quant.rs:40-41), so the list keeps
  PM-0004-67's three (schedule.rs:241-251; server/tests/schedule.rs:347 pins them). If U4e does plant an id in
  npt-server, it joins that one list; schedule.rs is not U4e's file, so lib.rs then builds the one list from
  schedule.rs's three and U4e's, the test's pin follows, and your report names it under U4E-Q3.
- **Not built:** AddrTable and the handshake cap (:713; U6), `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, the
  binary (§4.1 :202; §6 :876-878). server/src/main.rs stays as merged.

### 2.3 npt-client (crates/client/src/, crates/client/clippy.toml)
From plan :714-715; §4.6 :312-321.
- Decoding into integer frames and held floats; the complete-tick rule and the sentinel 0 (§4.6 :316-321): every
  group decoded, its hash logged under the tick it read (the header's full 32-bit tick, read whole: snapshot.rs:558;
  R11, R34); a received group it could not decode, the sentinel; a group lost and none refused, nothing, and
  `snap_incomplete` = 1.
- The client side of `client_hash` (§4.6 :312-315): its held floats turned back into grid integers through `grid`, with
  the 32-bit tick and event numbers as it rebuilt them (each event's number widened from its 16 bits through
  `Wrap::widen16`, wrap.rs:67-77; its tick the header's tick less its 8-bit age: snapshot.rs:31-32).
- **The last-run sequence's rebuild** (U4E-Q2 (c), widened, wave1 :446-448: "U4e builds the client's rebuild of the
  header's 16-bit last-run sequence (W1-N2) and unit-shows the bite as 'the rebuilt sequence names the wrong command'.
  U4g's Predictor starts its replay there and shows 'the replay starts at the wrong command'."). The header carries the
  newest command received and the last command run as 16-bit sequences, each with a known flag (snapshot.rs:449-452;
  §4.4 :279-283); the client rebuilds `last_run` to the command it names through a wrap-aware compare (the
  -58/lt-tick-offset site: §4.14 :469). That counter's wrap is exposed as its own `event.counter_wrap` (W1-N2, wave1
  :343-344: "The client logs that counter's wrap as its own `event.counter_wrap` (U4e), so the check's count of 6 stays
  honest").
- Acks with the group mask (`Input::ack_tick`, `ack_groups`: input.rs:41-45; §4.5 :292-294).
- A minimal **`ClientEndpoint`** (plan.md:486-489, :609): input, step, the next due time in ns (:715). It takes
  received messages as (Channel, bytes, arrival_ns), as plan.md:486 has it, never npt-link's `Recv` or `Link`:
  npt-client's sources may name nothing from `npt_link` but `Channel` (check.mjs:809, :821-837; R29). Every call into
  `npt_sim::grid` takes its config from `grid_with_bite` under the bite, and its `Quant` comes from
  `npt_proto::Quant::with_bite` (carry 24; client/src/lib.rs:14-22).
- A message on the wrong channel dropped and counted (`decode`'s `WrongChannel`: error.rs:24; :723-724; catalogue
  item 2's "on both sides", :243-244; nit d, advisor :164-165); a datagram above 1,150 B refused (`decode`'s
  `TooLarge` against `Limits::SNAPSHOT_RECEIVE`: limits.rs:28, msg.rs:397-405; catalogue :88).
- The frame ring keeps the same ages as the AckRing (`npt_sim::ACK_RING_TICKS` by path: §4.5 :295-297; §4.3 :255).
- **`npt_client::BITES`** grows with -58/lt-tick-offset and -58/lt-event-seq (carry 24); client/tests/bites.rs:10
  pins the list at its one id today, so that pin follows (R33).
- **The hash-map type bans** in crates/client/clippy.toml (2.1's bullet; U4E-Q8 (a)).

### 2.4 Tests (plan :716-724), each named in your report with its file
- e1 catalogue item 4: `Snapshot::GROUPS` and `FullState::GROUPS` (snapshot.rs:488-517, :644-670) equal the
  Field-group rows for `Snapshot` and `FullState` (catalogue :132-157), each with the same class; the authority-only
  groups have no entry (:716; catalogue :249-250). It lives in U4e's crates (crates/proto is not in U4e's lane: Lanes
  :859-863), reading the catalogue as proto/tests/catalogue.rs does (its path from `CARGO_MANIFEST_DIR`, :19-22; its
  table and cell readers, :68-100).
- e2 a forced group split, each group decoding alone (:716-717).
- e3 a lost group leaves its tick unlogged and counted; a refused one logs the sentinel (:717-718).
- e4 the baseline rule and the ring's age edges: age 31 kept, age 32 evicted (T15; §4.5 :295-297); a partly
  acknowledged tick never a baseline.
- e5 a gap evicting B (:718's "a 1 s gap evicting B": on Loopback the gap is `npt_sim::ACK_RING_TICKS` tick periods,
  U4E-Q14 (b); §4.14 :460's unit proof, the server half), as two tests, so the one that shows the bite could bite stays
  apart from the one the bite breaks:
  - e5a: on the injected clock it records the gap's start and end, B's tick and B's age at the first tick after the gap,
    and asserts that age ≥ 32 (`npt_sim::ACK_RING_TICKS`; §4.3 :255; §4.5 :295-297) before it asserts `burst_eviction`
    = 1. The gap lasts `npt_sim::ACK_RING_TICKS` tick periods (1.067 s at 30 Hz: 1 s plus two tick periods; U4E-Q14
    (b)), its length in ns on the injected clock computed in the test from that constant by path and the rate the test
    passes in (a test's input, never a const or static: carry 39). Why: every snapshot whose send falls in the gap is
    lost, at least 31 at 30 Hz, so B's age at the first tick after the gap is at least 32 at either alignment, while a 1
    s gap loses exactly 30 and leaves B at 31 or 32 on Loopback, at the ring's edge. **The gap's alignment** (Loopback
    neither delays nor loses: link/src/lib.rs:128-131, so B's age at the gap's start is 1 or 2 ticks): the test states
    the gap's start against the tick and runs at two alignments, (1) just after the server's send of tick T (T
    delivered, the client's ack of T the first datagram lost) and (2) just before that send (T the first datagram lost),
    and states when the client's inputs go out against the server's ticks. It reports B's tick and age at the first tick
    after the gap for both; the precondition holds only if both give at least 32 (`npt_sim::ACK_RING_TICKS`); if either
    gives less, stop that part and report both, and change neither the gap nor the alignment on your own. e5a also runs
    a 1 s gap at both alignments and reports B's tick and age there (reported, not asserted). e5a passes in both builds
    (`burst_eviction` says whether B left the ring, which the bite does not change: §4.5 :298-299).
  - e5b, under e5a's precondition (B's age ≥ 32 at the first tick after the gap, at both alignments): the first
    snapshot after the gap is coded against the per-type defaults (`baseline_age` 0: §4.5 :295-297;
    snapshot.rs:446-448). It passes in both builds; its bites-only twin asserts the defect on the bitten room (2.6's
    part (i)); part (ii) shows e5b failing by name under no-fallback.
- e6 the window counting the pump's bytes (:718-719; §4.5 :300-302): with the pump reporting k pings and pongs since
  the last tick (k small enough that the remainder still holds the tick's own-state group; the plan is silent on a
  smaller remainder), the encoder's budget for that tick equals the per-tick budget minus the sum of (payload + 28 B)
  over those k; the snapshot sent that tick stays within that remainder.
- e7 the mirror equal to the client's hash over Loopback at P0 for 1,000 ticks (:719): 1,000 compared ticks, every pair
  equal, none the sentinel 0. The scene is not named by the plan: say which you used and why (R18). The test also
  asserts, and your report states the counts, that over the 1,000 compared ticks at least one tick was sent as 2 or more
  groups (if the encoder never splits a tick at the window's budget, which is likely: one group carries up to 1,000 B of
  payload and two at most about 1,010 B of the about 1,067 B a tick allows, counting 28 B each: catalogue :88,
  limits.rs:25; then e2's forced split tick has the mirror compared with the client's hash the same way, and your report
  says which), at least one entity was skipped by the priority accumulator and hashed at B's value, at least one
  gameplay event was carried and at least one resent, and at least one snapshot had `baseline_age` > 0 (the parts §4.6
  :309-311 makes the mirror model); all 48 clients are compared if SS is the scene.
- e8 the Send asserts: compile-time asserts that `Room<World>`, `Decoder` and `ClientEndpoint` are `Send` (:719;
  §4.11 :401-402).
- e9 the negative cases (T17, A15; :719-724), each its own named test (the five JoinRejected reasons may be one table
  test that names the failing reason): each JoinRejected reason (bad token, bind mismatch, roster full, wrong version,
  wrong content hash); a datagram before Join dropped and counted; a control message before Join closing the session
  as malformed, counted under its reason, never INVALID (the run verdict INVALID is the judge's, U4r's: the test
  asserts the reason's count and that the roster admitted no session); the Input bucket (8, refilled at 60 a second)
  and the Ping limit (10 a second) on a fake clock, each tested either side of a refill instant, never on it
  (10^9 / 60 ns is not a whole number); the 2 s Join deadline on a fake clock; a second Resync before the FullState's
  ack dropped and counted; npt-client dropping and counting a message on the wrong channel; the three room-side checks'
  clean negatives (2.1: a repeated command not run twice; two press edges for one button inside one covered tick
  refused; an ack naming a never-sent tick refusing its datagram's commands; each counter accepted across its wrap).
- e10 the F2 pins: the Input bucket's refill at 30 and 60 Hz (60 and 120 a second); the window at 30 and 60 Hz
  (32,000 B a second at both: advisor :446-447; 32,000 / 30 is not a whole number of bytes, so the pin sums one
  second's per-tick budgets at each rate, asserts the sum, and your report states it: 32,000 B if the per-tick
  remainders carry, else the shortfall at each rate); the Pong hold time's limit at 30 and 60 Hz (one tick period:
  2.2).
- e11 one bite instance and one clean instance on parallel threads in one process (§4.11 :398-400): the bitten one
  shows its defect and the clean one none, the clean one's result equal to the same run alone.
- e12 a test per carry that names a behaviour:
  - carry 21: commands received in two orders give one `TickInput`, two of one slot whose ticks straddle the 32-bit
    tick wrap among them (seam.rs:116-121);
  - carry 22: with N the newest tick pushed into the ring, a command whose view time gives `tick_a` = N and
    `fraction > 0`: `rewind_at` returns no `Rewind` naming N + 1; by U4E-Q5 it gives that command no Rewind and the
    room's count of such commands rises by one; and the accepting edges, each with the count unchanged: `tick_a` = N
    with `fraction` 0 gives `Rewind { tick_a: N, fraction: 0, .. }`; `tick_a` = N - 1 with `fraction` > 0 gives a
    Rewind; `tick_a` = the oldest tick the ring holds gives a Rewind; `tick_a` one below it gives none and the count
    rises by one; for an accepted command, the Rewind's `tick_a` and `fraction` equal the command's `view_tick` and
    `view_fraction` (seam.rs:39-43), its `cover` the room's setting;
  - carry 23: each queued event carries the `TickInput::tick` it ran and consecutive 32-bit numbers, across a 16-bit
    wrap, the counter started near the wrap by its injectable start value;
  - carry 27: the const assert;
  - carry 50: no event is queued past age 255 (the overflow resync ends it first, or it is dropped from the resend and
    counted); the edge: an event at age 255 is still sent, none at 256;
  - carry 51: slot k's client receives exactly slot k's effect rows (the mapping read from `Scene::effect_rows()`:
    R31);
  - carry 53: for every client of an SS room (48 clients) on each of at least 300 ticks, in every `Snapshot` and
    `FullState` sent to it, every own-section `Soldier` and `OwnState` record has index = the client's slot and that
    soldier's generation, and its Effect rows are exactly the rows whose owner is that slot (`Scene::effect_rows()`);
    each client receives a `Snapshot` on each compared tick, and the test fails if one has none (a snapshot skipped
    under carry 54 would make the check vacuous); the compared ticks include a generation change, and the test asserts
    that at least one happened, failing if none (a generation changes only when a dead soldier respawns:
    world.rs:1100-1101, :1712-1723; SS's kills come at 0.272 a second: close :221-223); the window may run past 300
    ticks to hold one, and your report gives its length and seed;
  - carry 54: one out-of-range record planted for one client on one tick (for example slot k's `OwnState` field
    outside its range, through a test `SimCore` that wraps `World` and alters that record; only client k's own
    section carries it): client k's snapshot for that tick skipped and its count up by one, no panic, every other
    client's snapshot for that tick byte-equal to the same run without the plant, and client k's next tick encoded
    afresh;
  - carry 55: met by adding no Team or Public row (say so);
  - carry 56: a measurement, not a pass/fail test: 48 clients, the own-state section's bytes per snapshot on the wire
    (mean, p99 and max per client), the `OwnState` record's share of it and how often it changes, over a stated tick
    count and seed, in SS, and in ES as 3.1's note on carry 56 says;
  - carry 57: a never-acking client overflows its queue: one resync, counted for that client with its tick; then, as two
    tests, a second overflow before the FullState's ack, and no ack within `npt_sim::FULL_STATE_ACK_TICKS`, each sends
    `Kick::Overflow` and drops the client (both edges); the cap is read from the room's data, its provisional mark
    exposed;
  - carry 61: `ClientEndpoint` fed tick t+1's snapshot before tick t's (as L1's reorder step delivers them) files t
    under t (its hash if the client decodes the stale group, or lost if it drops it, the drop counted; never the
    sentinel), and one datagram received twice files one hash (the carry's unit test); a tick with a group missing is
    not resolved lost while it is younger than `npt_sim::ACK_RING_TICKS` ticks (by the newest tick the client holds) and
    is resolved lost (`snap_incomplete` 1, no hash) once it is that old (carry 61 (c)); and in carry 54's test the room
    exposes no server hash for client k at the skipped tick (carry 61 (a)).
- Plan silent (not added): `Join` on a slot with a live session replacing it (`Kick(replaced)`, catalogue :93), and a
  `Pong` naming a ping id that is not outstanding (dropped and counted: catalogue :92), have no U4e test in the plan;
  say in your report whether each is built and tested.
- Where U4e's tests live (advisor condition 10 (b), :145-146; Lanes :861-863): in its own crates, never in crates/lab;
  the Loopback tests in npt-server's tests, through its npt-link and its dev dependency on npt-client
  (server/Cargo.toml:32-35). npt-server's `bites` feature turns on npt-sim's, npt-proto's and npt-room's, not
  npt-client's (server/Cargo.toml:22), so an npt-server test that needs a client bite builds only under `--workspace
  --all-features` (check.mjs:1496), never `-p npt-server --features bites`; no manifest change (section 4).

### 2.5 -18's bite end to end (plan :725-728; the advisor's condition 6, advisor :125-129)
§4.14's unit proof (e5) is the server half only. U4e's proof runs from the encoder to the client, through a gap in
delivery (e5a's: `npt_sim::ACK_RING_TICKS` tick periods, U4E-Q14 (b)): under no-fallback the client refuses every delta
after the gap, so no fresh delta decodes within 500 ms of the gap's end, on the injected clock; the clean build decodes
one. Its proof takes 2.6's two parts: the end-to-end test passes in both builds; its bites-only twin asserts that no
fresh delta decodes within 500 ms under the bitten room (part (i)); part (ii) shows the end-to-end test failing by name.
If the bite's baseline turns out to be one the client still holds, the bite is not a defect on L1, and -18's L1 part
comes to the advisor before it counts (report it; do not count it).
- The gap: Loopback neither loses nor delays (link/src/lib.rs:128-131), and the plan leaves the mechanism open. A
  test-only `Link` wrapper in npt-server's tests that drops every datagram both ways for e5a's gap on the injected clock
  is inside the lane; its length in ns, from `npt_sim::ACK_RING_TICKS` by path and the rate passed in, never a const or
  static (carries 27 and 39). It runs at e5a's two alignments, and your report gives the clean build's first fresh delta
  and the bite's none for both. Say in the report what you built.

### 2.6 Bites (unit-shown; plan :729-731; sites from §4.14 :458-474)
Each planted where its code lives, chosen per instance (§4.11 :398-400), compiled into the bites build only (plan
:510-511), and shown to change state (§7.1 item 3, :896-897) in wave 1's two parts, so that check.mjs's test step,
which fails on any failing bites-build test (`testBothBuilds`, check.mjs:1493-1501), stays green in both builds:
- (i) a bites-only test (`#[cfg(feature = "bites")]`) that builds the bitten instance and asserts that the clean
  test's check fails on it, so it passes (proto/tests/bites.rs; client/tests/bites.rs:7-36), beside the clean test,
  which passes in both builds;
- (ii) per bite, one at a time under `flock -x`, a scratch switch in place that points the clean test at the bitten
  instance, run by name in the bites build to show it failing by name (the output line), then the file restored and
  its sha256 checked against the clean file (u4c-build.md:28-31; u4d-report.md:19).

The table's last column is what the bitten instance shows: part (i) asserts it, part (ii) shows the clean test failing
on it.

| Bite (plan :729-731) | Site | Unit proof |
|---|---|---|
| PM-0001-18/no-fallback | the encoder's baseline choice, never falling back to the defaults (§4.14 :460) | e5b (the server half) and 2.5's end-to-end test pass in both builds; their bites-only twins assert the defect on the bitten room (part (i)); part (ii) shows each failing by name; e5a passes in both builds (it shows the bite could bite: §4.5 :298-299) |
| PM-0001-17/tick16 and PM-0001-58/tick16 | npt-proto's header writer: under the bite it writes the tick's low 16 bits (quant.rs:133-140; snapshot.rs:532); the ids stay in `npt_proto::BITES` (quant.rs:38-39; U4E-Q4 (a)); the room's encoder takes its `Quant` from `Quant::with_bite` for the instance, and so does the endpoint (carry 24) | over Loopback with the room's tick past 65,535: in the clean build the client logs the frame at 65,537 (the header's 32-bit tick, read whole: snapshot.rs:558) with a hash equal to the mirror's; under the bite it logs it under 1 or as the sentinel, so the server's pair at 65,537 has no equal client hash (§4.14 :458: "a frame at tick 65,537 hashes differently or is refused"; §4.6 :322-325's orphan pair) |
| PM-0001-17/client-rounding, the client's call site | `ClientEndpoint`'s calls into `grid` through `grid_with_bite` (§4.14 :459; carry 24) | over a sweep of position values including half-cm ties and negatives, the client's grid integers equal the server's; the floor bite breaks a share your report states (§4.14 :459's "about half" is over U4s's sweep of arbitrary values, client/tests/bites.rs:24-35; values decoded from the wire may give another share) |
| PM-0001-58/lt-ack | the room's rebuild of the 16-bit ack (§4.14 :471) | a post-wrap ack refused |
| PM-0001-58/lt-cmd-seq | the room's sequence-forward compare (§4.14 :467; U4E-Q1 (a)) | post-wrap commands refused |
| PM-0001-58/lt-press | the room's press-edge compare (§4.14 :468; U4E-Q1 (a)) | an edge lost |
| PM-0001-58/lt-tick-offset | the client's rebuild of the header's 16-bit last-run sequence (a plain `<` in place of its wrap-aware compare: §4.14 :469; U4E-Q2 (c)) | the rebuilt sequence names the wrong command (U4g re-shows it as "the replay starts at the wrong command"; -58 is judged after U4g: plan :814-817) |
| PM-0001-58/lt-event-seq | the client's event-number rebuild (§4.14 :470) | a post-wrap event rebuilt wrong |
| PM-0001-62/server-echo | the room sending the owner its own cosmetic shot events (§4.14 :473; U4E-Q10 (a)) | one event sent |
| PM-0001-22/unbounded | the event queue's cap: under the bite the queue keeps every event past its cap, with no overflow resync and no `Kick::Overflow` (§4.14 :474; U4E-Q12 (c)) | a never-acking client: in the clean build its headroom (cap less queue length) falls to 0, never below, and one resync follows (carry 57's test); under the bite it goes below 0 with no resync and no kick (§4.14 :474's "headroom below 0"); the test may give the room a smaller cap, as data, to stay short. U4f runs it judged over a full S30 (plan :801) |
| owner-to-all (catalogue item 5's) | the encoder's class filter: one `owner` group sent to every client (catalogue :251-254) | carry 53's test (every own-section record's kind, index and generation, and an Effect row's owner, equal to the receiving client's slot, for each of 48 clients) fails under the bite, which writes one other slot's own-state records (its Soldier, OwnState and Effect rows) into every client's own-state section (carry 53's route; catalogue :254's "one `owner` group to every client"), within the section's 8-bit count (snapshot.rs:26, :526-530) and each group's 1,000 B, so that every bitten snapshot still encodes: a refused one is skipped under carry 54 and would leave carry 53's check nothing to fail on; a group-level class check is not the proof (it passes under this bite); moving the records into the entity section ends in `Err(OutOfRange)` and proves the codec (snapshot.rs:14-19), so it does not count (carry 53). The L1 audit is U4r's (plan :750). Its id: `catalogue-5/owner-to-all` (U4E-Q13 (a)) |
| PM-0001-21/no-trailing and PM-0001-21/no-channel, through the pump | npt-proto's decode (U4a's sites, quant.rs:40-41, chosen by `Quant::with_bite`, quant.rs:102-113; §4.4 :286-289) | with the bite, the pump's `TrailingBytes` or `WrongChannel` count stays 0, and the pump test fails |

- Bite ids are `PM-<ADR>-<row>/<name>` (check.mjs:689); each id lives in one crate's sources, `crates/<crate>/src/`
  only, tests and `#[cfg(test)]` modules unread (check.mjs:697-758; carry 24). So U4e's sources never spell npt-proto's
  or npt-sim's ids: they pass the caller's id string to `Quant::with_bite` (R30). Each -58 id U4e plants maps to one
  `Wrap` counter name through `Wrap::plain_lt` (wrap.rs:10-13, :26-31). Every id U4e plants is marks.json's spelling but
  one: `catalogue-5/owner-to-all` (U4E-Q13 (a)), which marks.json does not have and check.mjs's bite-home grep does not
  read (it reads only `PM-<ADR>-<row>/<name>`: check.mjs:689). So npt-room's bites test pins `npt_room::BITES` and
  checks each of its PM-form ids against marks.json, as server/tests/schedule.rs:345-353 does, exempting that one id by
  name; and a bites-only test in npt-server's tests (which reach every crate's list under `--workspace --all-features`)
  asserts that no other list holds it (`npt_proto::BITES`, npt-sim's `World::BITES` and `OWN_COPY_SNAP_BITES`,
  `npt_server::BITES`, `npt_client::BITES`). U4r's `--bite` dispatch selects it by that string.
- -22/unbounded's site is U4e's `EventQueue` cap: U4e plants and unit-shows it, and U4f runs it judged (U4E-Q12 (c); the
  plan's Bites line gains it).

### 2.7 The seam rule, and what stops the build
- No existing item of npt-sim changes: `SimCore` and its methods, `Command` and its constants, `Rewind`,
  `CoverVariant`, `TickInput`, `Placement`, `AttributeChange`, `Attribute`, `EntityKind`, `EntityRecord` (FIELDS),
  `HitVolumes`, `VolumeBox`, `PortalStates`, `SimEvent`, `ClampReport`, `Refusal`, `RefusalKind`, `InputReports`
  (seam.rs:22-564), `HitHistory` (history.rs), `grid` (grid.rs), the tick constants (ticks.rs), `World`, `Scene`,
  `Greybox`, as U4d merged them; nor any npt-proto item (U4a's, with W1-S1's half) or `Schedule`, `Due`, `due_ticks`
  and the rest of schedule.rs (U4b's). crates/sim, crates/proto and server/src/schedule.rs are not U4e's files.
- A needed change stops that part: write it in your notes and report (the item, the change, why, the site) and build
  the rest. It is a seam change for the advisor (carry 26's route): your report reaches the advisor through the main
  session; you never work around it.
- Carry 51's home is not such a stop: the main session adds `Scene::effect_rows()` (pub, the rows per soldier) and its
  §4.1 line in its own commit when U4e's build starts (carry 51; close B2, :237-241), so `<base>` holds it and 7.2
  item 1 applies. Check that `<base>` holds it (`pub fn effect_rows` in scene.rs) before you build the own-state
  filter; if it does not, stop that part (the filter's Effect rows, carry 51's and 53's tests) and report it, never a
  local copy of the 8. Any other home U4e finds it needs is a seam change for the advisor (carry 26's route).
- A tick count or a named constant derived from the rate: no local form; the need goes in your report (carry 39 and
  its route, quoted in section 3).

### 2.8 Public items (A19; plan §7.2 :912-913)
- Allowed: npt-room's surface, plan.md:596-598 (`Room`, `Roster`, `ClientSlot`, `InputBuffer`, `EventQueue`, `AckRing`,
  `Encoder`, `InterestGrid`, `AreaFlood`, `Priority`, `rewind_at`) with §4.1's `Room<S: SimCore>`,
  `InputBuffer::CMD_CATCHUP` and, bites builds only, `BITES` (:194; U4E-Q3); the bites-only constructors that choose a
  bite per instance (`Room::with_bite`, which §4.11 :398-400 names, and the pump's and the endpoint's, where they take
  the id rather than a `Quant` or grid config passed in) are expected: list them as beyond §4.1's rows, for
  engine-architect; npt-server's U4e part of plan.md:599-600 (`Counters`, `ConnLimits`, `Decoder`, with §4.1's
  `Decoder::pump`, :195), beside U4b's items; npt-client's `ClientEndpoint` and `StepOut` (plan.md:609) beside U4s's
  `BITES` and `grid_with_bite` (§4.1 :193). Everything else `pub(crate)` (plan.md:454). `Scene::effect_rows()` is the
  main session's (carry 51), not an item U4e adds.
- Your report lists every pub item U4e adds (types, functions, methods, fields, constants), by crate, against §4.1 and
  plan.md §4.7; engine-architect checks them (check.mjs does not: plan :912-913).

---

## 3. The carries

### 3.1 Word for word
Quoted from carries :147-247, the file's §U4e as it stands (the advisor's U4s ruling, condition 4: advisor :334-335).
Checked: the main session re-copied this block by script at U4j's merge (rev 3.2, carry 61 added) and compared it with
carries :147-247 by a `diff` of the two line ranges: equal. Your report says of each, by number, "met" (with the
file:line and the test that shows it) or "not met" (with why).

```
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
```

Sixteen carries (21-27, 39, 50, 51, 53-57 and 61). Notes for the builder, which change no word:
- carry 24: its tick16 part is met by the endpoint's and the encoder's `Quant` (2.6's tick16 row), its -45 and
  cues-from-replay parts are U4g's (U4E-Q10), so U4e reports carry 24 as met under these words (wave1 :466);
- carry 25: a "why not" is reported, never assumed;
- carry 26: section 5 item 4 says what to report;
- carry 39: its "check.mjs:627" was the line at U4s; at the head the pass that lets an initializer naming `TICK_HZ`
  through is check.mjs:637-644;
- carry 51: the main session adds `Scene::effect_rows()` in its own commit when U4e's build starts (close B2), so
  `<base>` holds it (2.7; R31; 7.2 item 2); if it does not, that part stops (2.7), never a local copy of the 8;
- carry 53: its test (e12) must hold a generation change and asserts that one happened: a generation changes only when
  a dead soldier respawns (world.rs:1100-1101, :1712-1723), and SS's kills are rare (0.272 a second: close :221-223);
- carry 54: U4e counts the refusal per client and exposes it under a proposed series name; your report names that series
  and its proposed SERIES.md row for the main session (marks/ is not U4e's lane: section 4; npt-room has no npt-log
  dependency, room/Cargo.toml:17-21; U4r's logs write it, as U4E-Q7's values: 2.1);
- carry 56: if `<base>` lacks U4d-F (unmerged at rev 3.1: 4ad80a9 saved its partial edit as a patch), your report
  gives SS and marks ES "after U4d-F", measured once U4d-F merges (3.2); the measurement states its ticks, seed and
  client count;
- carry 57: the room exposes each overflow resync per client with its tick, and U4r's logs write them (as U4E-Q7's
  values: 2.1); the cap's provisional mark is exposed for the run's header;
- carry 61: the room's `server_hash` and the client's `decoded_hash` follow marks/README.md §1 as U4j merged it
  (d50396a: :50-71, with round 2's words); its unit test (tick t+1's snapshot fed to `ClientEndpoint` before tick t's,
  and one datagram received twice) is one of your tests (2.4), named in the report; your report says whether your client
  decodes a stale group or drops it (the advisor's lean is the decode: advisor-u4j-r2.md B2.1), and a client that drops
  counts each drop per client under a proposed series name with its SERIES.md row for the main session, as carry 54's
  refusal count does;
- carry 40 (§U4r, carries :250-258) is U4r's; U4e feeds its last sentence with the `rewind_at` count (2.1).

### 3.2 U4d's round-2 carries: ruled (advisor-u4d-close.md, 2026-11-11)
The advisor ruled on the items rev 3 held here as pending, and the main session folded the rulings in by script before
this brief's check:
- N2-L1, joined with the security-auditor's first carry, is carry 53; the security-auditor's second and third carries
are 54 and 55; N2-L3 (c) is 56; U4E-Q6's room half is 57. All five are in 3.1, word for word from u4s-carries.md §U4e as
it stands (B2). The security-auditor's fourth bullet (the merge-time items) is no carry (B2).
- Carry 51's route is corrected: the main session adds `Scene::effect_rows()` in its own commit when U4e's build starts
(carry 51 in 3.1; B2).
- N2-M2 (ES's vehicles) is fixed in the scene by the main session's change U4d-F, before any ES pin (B1.3); U4e edits no
crates/sim file for it. Any ES number in U4e's report comes from U4d-F's scene (B1.2), so without U4d-F at `<base>`,
the ES numbers wait for it (3.1's note on carry 56).

---

## 4. Build rules

**The shared target** (Mustafa's rule, 2026-11-09: plan §7.1 :892-904; the advisor's W1-TARGET, wave1 :558-586),
binding on every cargo, rustc and rustdoc call in your worktree, check.mjs's included:
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` on every call
   (item 1, :893-894; wave 1's briefs carry the second: u4j-brief.md:38-39; U4s made every manifest and Cargo.lock
   change wave 2 needs, Lanes :837-841, so nothing is fetched). Checked once after the first build: the artifacts are
   in that folder, not in your worktree's `target/`.
2. **The lock** (W1-TARGET item 1, wave1 :570-573): every such call runs under `flock` on
   `/home/user/Shamble-Frontlines/.gate/shared-target/.lane.lock`: shared (`flock -s`) for an ordinary build, exclusive
   (`flock -x`) for a counted run (the touch, check.mjs, the `--list` and clippy confirmations) and for each bite run.
   With `T=/home/user/Shamble-Frontlines/.gate/shared-target` and `W=<your worktree>`, from `$W/spikes/netcode-proof`:
   - an ordinary build, after rule 4's touch: `flock -s $T/.lane.lock sh -c 'find $W/spikes/netcode-proof/crates -type f -exec touch {} + && CARGO_TARGET_DIR=$T CARGO_NET_OFFLINE=true cargo test --locked -p npt-room'`
   - a counted run: `flock -x $T/.lane.lock sh -c 'find $W/spikes/netcode-proof/crates -type f -exec touch {} + && CARGO_TARGET_DIR=$T CARGO_NET_OFFLINE=true cargo test --locked --workspace --no-run && CARGO_TARGET_DIR=$T CARGO_NET_OFFLINE=true cargo test --locked --workspace --all-features --no-run && CARGO_TARGET_DIR=$T CARGO_NET_OFFLINE=true node check.mjs'`
     (with `T` and `W` written out in full).
3. **Provenance** (W1-TARGET item 2, wave1 :574-575): a counted run's log shows cargo's "Compiling" line for npt-room
   with your copy's path in its brackets (cargo prints "Compiling npt-room v0.0.0 (<your
   worktree>/spikes/netcode-proof/crates/room)": match on the path, not the version), and the same for npt-server and
   npt-client, in each cargo invocation that builds them. check.mjs prints a passing step's last line only
   (check.mjs:1646; a failing step's last 40 lines: :1649), so the two `--no-run` builds inside the same exclusive lock
   carry those lines (wave 1's form: u4j-build.md:110-111; u4d-report.md:68-70). A missing line means "not counted",
   whatever the count says.
4. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (item 2, :895).
5. **Counts are deltas** (item 3, :896-897; plan :866-867), on check.mjs's own measure: its test step runs `cargo test
   --locked --workspace` and then the same with `--all-features` (check.mjs:1493-1501), and its count (`cargoCounts`,
   check.mjs:1411-1421) sums the "N passed; M failed" of each run's `test result:` lines. **The base counts at
   39f4290:** measured 365, bites 412; self 52 of 52 (the main session's counted run, 12:05-12:10 UTC:
   u4d-report.md:68-70). At d50396a (U4j's merge): measured 396, bites 443, self 52 of 52 (R38). `<base>` also holds the
   carry-51 commit, so it differs from both under spikes/, and the main session gives `<base>`'s counts at launch (7.2
   item 1). Each run's count matches the base plus the tests you added; a bite run shows its own test failing by name
   (2.6's part (ii)); a mismatch means another copy's artifact ran: touch, rebuild, never report that run. `--list` only
   names the new tests (the flag before `--`: `cargo test --locked --workspace --all-features -- --list`); it is not a
   count (it lists ignored tests too and runs nothing).
6. **Disk** (W1-TARGET item 3, wave1 :576-580; u4j-brief.md:43): `df -h /home/user` before each build, the reading in
   your notes. At least 4 GB free before a run that builds for wasm32 or in release (check.mjs is one: its
   clippy-wasm32 and doc steps build for wasm32, check.mjs:1438-1439, :1472-1490), at least 2 GB before every other
   build; below that, stop and report; never clean anything to make room (item 7, :902; the between-waves clean is the
   main session's, never a lane's: wave1 :578-580).
7. After the last bite, touch and rebuild before any further claim (item 4, :898).
8. Cargo's own lock still serializes builds: expected (item 5, :899). The main folder's cache is not yours (item 6,
   :900-901).

**Your files, and only these** (Lanes :859-863; plan :710): crates/room/ (src/, tests/, clippy.toml); crates/server/
(src/ but schedule.rs, which is U4b's, and tests/); crates/client/ (src/, tests/, clippy.toml).
- No manifest edit: no Cargo.toml (workspace or crate) and no Cargo.lock; U4s made them (Lanes :837-841; npt-room's and
  npt-client's `bites` features: room/Cargo.toml:12-15, client/Cargo.toml:12-15; npt-server's dev dependency on
  npt-client: server/Cargo.toml:32-35). A needed manifest change stops that part and goes in your report.
- server/clippy.toml was U4b's in wave 1 (:843-844); the plan names no U4e change to it: leave it as merged.
- Never crates/sim, crates/proto, crates/lab (U4d2's alone in wave 2: :861-862; advisor :145-146), crates/log,
  crates/judge and marks/ (U4j's in wave 1, merged at 4d2b0ef; not U4e's), check.mjs (U4d2's in wave 2: :859-860),
  docs/.
- Files are written with the Write and Edit tools only, never shell redirects, heredocs or `python3 -c` (wave 1's words:
  u4j-brief.md:46-47). U4e's bites are compiled into the bites build and chosen per instance (plan :510-511; §4.11
  :398-400), so a bite needs no planted defect; its failure by name is shown by 2.6's part (ii) scratch switch. A
  scratch switch or plant (part (ii), or a defect that is not a bite, to show that a test can fail) is made in place,
  one at a time, each file restored and its sha256 checked against the clean file (u4j-brief.md:47-48; plan §7.2
  :908-909), and run under `flock -x`. Run git only in your worktree, never `git -C` or `git -c` (u4j-brief.md:46).
  Commit nothing and push nothing; the main session merges (plan §7.2 :906-909).
- .claude/rules/netcode.md binds proto, room, client and server code (plan :903-904): validate every inbound message
  before use (:10), no unbounded buffer, map or queue keyed by client data (:14-15), send each client only what it may
  know (:16), no clock in shared code (:12-13).

---

## 5. End checks

Run from spikes/netcode-proof/ in your worktree, each as a counted run under section 4's rules (rule 2's form; `df`
before each):
1. check.mjs twice, each with every step passing. At 39f4290 it has 10 steps (fmt, clippy, clippy-wasm32, doc, test,
   deps, refs, marks, self, redact: check.mjs:1594-1631); the tick, float, bite-home and npt-link checks run inside deps
   (:1582-1586). U4d2 owns check.mjs in wave 2 and adds its `wasm` step (plan :674-675), so if U4d2 merges first the
   count at U4e's merge is the main session's to state (R25). Report both runs' last lines ("check: all N steps pass":
   check.mjs:1652), the test step's two counts and each run's provenance lines.
2. clippy on the host and on wasm32, in both builds, warnings as errors. check.mjs's clippy and clippy-wasm32 steps
   do this (`lintBothBuilds`, :1456-1462; the wasm32 crates are LEAF plus npt-tcpinfo, :31 and :1438, which include
   npt-room, npt-client and npt-proto, not npt-server); state each step's line.
3. fmt: check.mjs's fmt step (`cargo fmt --all --check`, :1595).
4. Carry 26: report whether the room copies `step`'s events anywhere other than into the EventQueue's stamped entries
   (carry 23) before it reads `hit_volumes`, `reports` or `entity_records` (file:line); if it does, carry 26's seam
   change goes to the advisor through your report (2.7). Whether `Room<World>`, `Decoder` and `ClientEndpoint` are
   `Send` is e8's compile (World is: sim/tests/world.rs:158-160; Loopback is not, so nothing `Send` may hold one: R36).

---

## 6. Return list (your report, in this order)

1. `<base>` and your final diff's file list (`git status --porcelain` and `git diff --stat <base>`), each file inside
   section 4's lane.
2. Each Build item of 2.1-2.3: built (file:line) or not (why).
3. The tests e1-e12 (e5 as e5a and e5b) and any you added: name, file, what each shows (e7's counts of splits, priority
   skips, events carried and resent, and non-default baselines); the counts in both builds as deltas from `<base>`'s,
   on section 4 rule 5's measure, with the `--list` lines that name the new tests.
4. Each bite of 2.6: its id, its site (file:line), its part (i) test (name, passing in the bites build) and part (ii)'s
   output line showing the clean test failing by name, with the restore's sha256; the parallel-instances test (e11).
5. -18 end to end (2.5), at both of e5a's alignments: the clean build's first fresh delta after the gap (its time on
   the injected clock); the bite's none within 500 ms of the gap's end, on the injected clock; B's tick and age at the
   first tick after the gap (e5a); whether the bite's baseline was one the client still held (if so: not counted, to
   the advisor).
6. Carries 21-27, 39, 50, 51, 53-57 and 61, by number: met (file:line, test) or not met (why); carry 56 as its
   measurement (ticks, seed, client count; ES "after U4d-F" if `<base>` lacks it); carry 61 with its client's choice for
   a stale group (decoded, or dropped and counted); and, for 3.2, which scene each ES number came from.
7. Every pub item U4e adds, by crate, against §4.1 and plan.md §4.7 (2.8); any beyond them named as such (the bites-only
   `npt_room::BITES` under U4E-Q3).
8. What stopped and goes to the main session: seam changes for the advisor (carry 26's route; carry 51's home only if
   `<base>` lacked it), tick counts or rate-derived constants (carry 39's route: name, definition, values at 30 and 60
   Hz, site), manifest needs, and carry 54's proposed series name with its SERIES.md row.
9. The end checks: check.mjs's two runs (each step's line), the clippy host and wasm32 lines in both builds, fmt;
   `CARGO_TARGET_DIR` checked after the first build; each `df` reading; each counted run's provenance lines and its
   `flock -x` use.
10. How each ruling was applied: U4E-Q1 to Q11 (section 8's table), W1-S1's own-state rule, W1-N2's header and
    counter-wrap parts, W1-PUB's two nits (the server path you kept; the room's route to the static graph), and the
    answers to U4E-Q12, Q13 and Q14.
11. What you did not verify, and what is left.
12. Who built it (the advisor's U4S-BUILDER terms, advisor :365-371).

---

## 7. Recheck after wave 1

### 7.1 The table, walked at the head
Rev 2's rows R1-R28 named each item U4a, U4b or U4d builds (plus four that wave 2 or U4j can change), for a check
against the merged code. Each is now read at the head (section 0): "holds" (the brief's assumption stands) or
"changed" (what the code has, and what the brief now says). R29-R38 are new: merged code the brief relies on that the
table missed.

| Row | Item | Built by | Use in U4e | At the head |
|---|---|---|---|---|
| R1 | `Limits::REDUNDANCY_CAP` (§4.3 :260) | U4a | `CMD_CATCHUP` = it, const assert (2.1; carry 27) | holds: `pub const REDUNDANCY_CAP: usize = 8` (limits.rs:17); a `usize`, so `CMD_CATCHUP` takes that type |
| R2 | `Limits`' size limits (catalogue :87-101) | U4a | the pump's socket checks; the client's 1,150 B refusal (2.2, 2.3) | holds: limits.rs:17-52, compared with the catalogue by U4a's item 3 test (proto/tests/catalogue.rs:666-); `SNAPSHOT` 1,000 B send (limits.rs:25), `SNAPSHOT_RECEIVE` 1,150 B (:28); `decode` applies the size first (msg.rs:397-405), so the client's refusal is `decode`'s `TooLarge` |
| R3 | `Quant` (§4.4 :273-275) | U4a | the encoder and the client's decode (2.1, 2.3) | holds: `Quant::default()` and bites-only `Quant::with_bite(id)` (quant.rs:57-61, :102-113); `Quant::encode` and `Quant::decode` (msg.rs:308, :391); it carries all seven of npt-proto's bites per instance, -57's three among them (quant.rs:14-25) |
| R4 | `Wrap` (§4.4 :284-285) | U4a | the seq, press, ack, event-number and last-run rebuilds (2.1, 2.3, 2.6) | changed: `Wrap` holds no list of counter names: each compare takes the counter's name as a string (`after8`, `after16`, `after32`, `widen16`: wrap.rs:43-77); `plain_lt(counter)` plants the plain `<` for one named counter and names no bite id (wrap.rs:10-13, :26-31); the counters and their injectable start values are their owners' (wrap.rs:3-5). The brief now says the room and the client own each counter, its name and its start value, and map each -58 id to its counter's name (2.1, 2.3, 2.6) |
| R5 | the `Input` codec (§4.4 :276-278) | U4a | the pump decodes; `InputBuffer` (2.1, 2.2) | holds: the field table (input.rs:4-14: 5 + 23 + 7 × 19 = 161 B at the cap); `Input { ack_tick: u16, ack_groups: u8, commands }`, newest first (input.rs:41-50); `Command::slot` is not on the wire, the decoder writes 0 and the room sets it (input.rs:27; 2.1 says so) |
| R6 | the Snapshot container and its field-group layout table (§4.4 :279-283) | U4a | the encoder, the client's complete-tick rule, e1 (2.1, 2.3, 2.4) | changed (the open home settled): the table lives in npt-proto, `Snapshot::GROUPS` (12 rows) and `FullState::GROUPS` (6 rows), each (name, class, kinds) (snapshot.rs:488-517, :644-670); the brief now says the encoder uses them and builds none (2.1), and e1 compares them with the catalogue (2.4) |
| R7 | the `EntityRecord` ↔ wire mapping (A18) | U4a | the encoder and the client (2.1, 2.3) | holds: `EntityDelta::between` and `apply` (snapshot.rs:86-149); `area` and `raw` are never sent, 0 from the wire (snapshot.rs:58-60, :151-161); a1's round trip in proto/tests/wire.rs |
| R8 | `client_hash` (§4.6) | U4a | the mirror and the client's hash (2.1, 2.3, e7) | holds: `client_hash(tick, entities, own, events: &[(u32, SimEvent)])` (hash.rs:72-97): (32-bit number, kind) per event, any order on either side (hash.rs:69-71); never 0 (hash.rs:100-102) |
| R9 | `encode`, `decode`, `TrailingBytes`, `WrongChannel` (§4.4 :286-289) | U4a | the pump counts them (2.2) | holds: `decode(way, channel, bytes)` (msg.rs:299-301), the channel an argument (from the link's `Recv`); `WrongChannel`, `TrailingBytes` and `UnknownField` (error.rs:24, :26, :29), all three "the socket checks" (error.rs:7-8) |
| R10 | no-trailing and no-channel (plan :549-552) | U4a | shown through the pump (2.6) | holds: `PM-0001-21/no-trailing` and `PM-0001-21/no-channel` (quant.rs:40-41), chosen by `Quant::with_bite` (quant.rs:108-109) for the instance that holds the `Quant` |
| R11 | -17/tick16 and -58/tick16 (a5: plan :548; §4.14 :458) | U4a | the client half (2.6; carry 24) | changed: `git grep -n 'tick16"' 39f4290 -- spikes/netcode-proof/crates` finds the ids only in npt-proto (quant.rs:38-39, :107; proto/tests/bites.rs:32-33, :167); the defect is the header writer's (quant.rs:133-140; snapshot.rs:532), and the header carries the full 32-bit tick, which the decoder reads whole (snapshot.rs:558): the client has no 16-bit tick rebuild. The brief now applies U4E-Q4 (a) and words tick16's proof through the room's encoder to the client (2.6) |
| R12 | `Join`, `JoinAccepted`, `JoinRejected`, `Resync`, `FullState`, `Ping`, `Pong`, `Kick`, `ClientSettings` (catalogue :87-101) | U4a | the roster, the pump, the client (2.1-2.3, e9) | holds: JoinRejected's five reasons (msg.rs:112-118); Kick's `Replaced`, `Malformed`, `Overflow`, `Stopped` (msg.rs:159-168); Pong's hold time `hold_us: u16`, µs (msg.rs:56-65) |
| R13 | check.mjs's sim/proto clippy.toml comparison (plan :538, a9 :554) | U4a | the hash-map bans in room's and client's clippy.toml (2.1, 2.3) | holds: `CLIPPY_TWINS` is sim's and proto's files only (check.mjs:765, :779-803); room's and client's are not compared, so the bans fail nothing there |
| R14 | proto/tests/catalogue.rs, items 1, 2, 3, 6 (plan :535-537) | U4a | e1 reads the doc the same way (2.4) | holds: the doc's path from `CARGO_MANIFEST_DIR` (tests/catalogue.rs:19-22), read at :320-322, tables by header row (:74-100), cells (:68-72); items 1, 2, 3 and 6 at :448-, :566-, :575-, :666-, :760-. A test in crates/room or crates/server sits at the same depth, so the same relative path reaches the doc |
| R15 | `Schedule`, `Due`, `due_ticks` and U4b's line in server/src/lib.rs (plan :559; advisor :369-370) | U4b | U4e edits server/src/lib.rs around it (2.2) | changed: U4b wrote `pub mod schedule;` (server/src/lib.rs:10) and re-exports (:12-14), two paths per item (W1-PUB's nit, wave1 :385-386); server/tests/schedule.rs uses the re-exports (:5, :347). No U4e name collides (`Decoder`, `ConnLimits`, `Counters` against `Dropped`, `Due`, `LoopCounters`, `Schedule`, `due_ticks`, `BITES`). The brief now asks for one path (2.2) |
| R16 | server/clippy.toml's clock bans (plan :559-560) | U4b | the pump reads only the injected now (2.2) | holds: `std::time::Instant`, `SystemTime` and `tokio::time::Instant` (allow-invalid) and their `now` (server/clippy.toml:4-13) |
| R17 | `World` implementing `SimCore` (plan :602) | U4d | `Room<World>` (e7, e8; 2.5) | holds: `World::new(scene: &Scene, seed: u64, first_tick: u32)` (world.rs:663); `impl SimCore for World` (world.rs:2060); `World` is `Send` (sim/tests/world.rs:158-160) |
| R18 | `Scene` (SS, ES, CO, TC: plan :602; §4.13) | U4d | e7's 1,000 ticks at P0 and 2.5's run need a scene | holds (the choice stays yours): `Scene::stress` (scene.rs:149: 48 slots, 15 vehicles, 300 projectiles, 8 effect rows a soldier), `estimate` (:179: 0 effect rows), `course` (:198: 1 slot), `targets` (:233: 5 slots), `by_id` (:246); name the scene and why |
| R19 | `InputReports::non_finite` removed (NF-D4; advisor :351-364) | U4d | the room logs `reports()` (2.1) | changed: the field is gone (seam.rs:525-542), and `InputReports` also gained `refusals: Vec<Refusal { slot, seq, kind }>`, cleared each step (seam.rs:497-531; the advisor's W1-U4D N-M2); the brief now has the room log the clamp reports, the refusal records and the five counters (2.1) |
| R20 | `EntityKind::layout()` completed within FIELDS (carry 12) | U4d | the layout table and e1 (2.1, 2.4) | changed: eight kinds, `OwnState` appended last (seam.rs:199-204, `ALL` :289-298; W1-S1); `SOLDIER` is 8 fields, the catalogue's remote row (seam.rs:249); `OWN_STATE` 8 (:252-261); a part's transform relative to its hull (:264-267); `PROJECTILE` 9, `WALL` 2, `EFFECT` 3 (:269-285); FIELDS still 12 (:360). The brief now states the kind and the own-section rule (2.1) |
| R21 | `TickInput`'s new fields (Lanes :848) | U4d | the room fills them each tick (2.1) | holds: `spawns: Vec<Placement>`, `teleports: Vec<Placement>`, `attributes: Vec<AttributeChange>` (seam.rs:122-127); `Placement { slot: u8, position: [i32; 3] }` (:131-138); `AttributeChange { slot: u8, attribute, value: u32 }` (:142-151); integers only (:108-110) |
| R22 | `HitVolumes` filled (plan :602-603) | U4d | the room copies them into `HitHistory` (2.1) | holds: `tick`, `soldiers`, `cover` (seam.rs:385-394); a hand-written `Clone` whose `clone_from` keeps the storage (:413-428), which `HitHistory::push` uses on a full ring (history.rs:59-74), so a push allocates nothing once full; at most 4,536 B a tick (`MAX_BYTES`, :405-406) |
| R23 | carry 17's button constants (Lanes :848) | U4d | the press-edge compare (2.1) | holds: `Command::FIRE`, `CROUCH`, `SPRINT`, `BUTTONS` (seam.rs:52-58); `PRESS_FIRE`, `PRESS_RELOAD`, `PRESS_THROW`, `PRESS_MELEE` (:62-71) |
| R24 | the seam freeze (Lanes :846-858, with W1-S1's lift) | U4d | the whole brief (2.7) | holds, with the one lifted change: `git diff df8975b 39f4290 -- spikes/netcode-proof/crates/sim/src/seam.rs spikes/netcode-proof/crates/sim/src/grid.rs` (312 lines added, 31 removed) read line by line: `Command`'s fields unchanged (an `impl Command` block of constants added); `Rewind` and `CoverVariant` unchanged; `EntityRecord`'s fields unchanged, FIELDS 12; `EntityKind` gains `OwnState` last and `SOLDIER` loses velocity (W1-S1's lift, Lanes :853-858); `layout()`'s body completed (H1's part record, the projectile's type and fuse, the wall, the effect rows); `TickInput`'s three fields; `SimEvent`'s variants reformatted, fields unchanged; `InputReports` loses `non_finite` and gains `refusals`; grid.rs: four doc lines only |
| R25 | check.mjs's steps | U4a, U4d2 | section 5's "every step passing" | holds: 10 steps at 39f4290 (check.mjs:1594-1631); U4d2 may add its `wasm` step in wave 2 (plan :674-675), so the count at U4e's merge is the main session's to state |
| R26 | sim's and proto's bite ids | U4d, U4a | the one-crate id rule (2.6; carry 24) | holds: sim `OWN_COPY_SNAP_BITES` (lib.rs:47) and `World::BITES` (world.rs:563-568, :896-897: PM-0004-21's two, -41's two); proto's seven (quant.rs:35-41); server's PM-0004-67 three (schedule.rs:241-251); client's one (client/src/lib.rs:12). None is an id U4e plants |
| R27 | marks.json's ids for -17, -18, -58, -62 and owner-to-all | marks.json at `<base>` | the ids U4e plants (2.6) | changed (one id): marks.json spells -18/no-fallback, -58/lt-ack, lt-cmd-seq, lt-event-seq, lt-press, lt-tick-offset and tick16, -62/server-echo, -17/tick16 and client-rounding; it has no owner-to-all id (catalogue item 5 is a catalogue test, no mark's: plan :731, :750 name it "owner-to-all" only). The brief now asks U4E-Q13. W1-N2 keeps lt-tick-offset's id (wave1 :341-342). Rechecked at d50396a (U4j's merge): U4j added one id, `PM-0001-18/ws-budget-x2` (marks.json:4431), the send budget's bite on the L2 B1 ws part; the send budget is npt-net's WsServer, wired in U6 (plan.md:217, :521), not U4e's, so the ids U4e plants are unchanged |
| R28 | npt-log's `TickRecord.hs`, `snap_incomplete`, `burst_eviction` (plan :645, :657-658) | U4j | U4e exposes the client hash and the two series (2.1; U4E-Q7) | changed at d50396a (U4j's merge): crates/log's `TickRecord` has `hs` (log/src/lib.rs:121) with `set_hashes` and `hashes` (:158, :168); SERIES.md has the `tick.s.snap_incomplete` row (:319) and `event.burst_eviction.evicted`'s (:320); marks/README.md §1's `hs` rules bind U4e through carry 61. U4e still exposes the values under the plan's names (`event.burst_eviction`: §4.5 :298-299; `tick.s.snap_incomplete`: §4.6 :321) and U4r's logs write them; npt-room and npt-client still have no npt-log dependency (room/Cargo.toml:17-21; client/Cargo.toml:17-22). Recheck at launch |
| R29 | npt-link's `Channel`-only rule (catalogue item 2; carry 3) | U4s, U4a | the endpoint's input (2.3) | new: npt-client's (and npt-proto's) sources may name nothing from `npt_link` but `Channel` (check.mjs:809, :821-837; client/Cargo.toml:18-20). So `ClientEndpoint::input` takes (Channel, bytes, arrival_ns), as plan.md:486 has it; npt-server's tests may name `Link` and `Loopback` |
| R30 | check.mjs's bite-home rule | U4s, U4a | U4e's id lists (2.1, 2.2, 2.6) | new: it reads string literals in `crates/<crate>/src/` only; `tests/` and `#[cfg(test)]` modules are not read (check.mjs:697-758). U4e's sources never spell another crate's id; they pass the caller's id string to `Quant::with_bite`; tests may name any id |
| R31 | carry 51's public home for an effect row's owner | the main session, at U4e's start (carry 51) | the own-state filter (2.1; carry 51) | new, missing at 39f4290: `Scene`'s public items (scene.rs:149-300) have no effect-rows accessor; `effect_rows` is `pub(crate)` (scene.rs:119); the World maps index / rows (world.rs:1879-1881); seam.rs:283 says "its owner is its index / 8". By carry 51 (close B2), the main session adds `Scene::effect_rows()` and its §4.1 line in its own commit when U4e's build starts, so `<base>` holds it (2.7; 7.2 item 2) |
| R32 | W1-S1's codec half | the main session, in U4d's merge | the own-state section (2.1) | new: both own-state rows list `Soldier`, `Effect`, `OwnState` (snapshot.rs:491-499, :647-655); `encode` refuses a kind its section does not list (`EncodeError::OutOfRange`) and `decode` returns `Malformed` (snapshot.rs:14-19, :175-180); the codec checks the kind per section, never the slot |
| R33 | the bite-list pins in tests | U4s, U4b | U4e's id lists (2.2, 2.3) | new: client/tests/bites.rs:10 pins `npt_client::BITES` at its one id, so U4e's two client ids change that pin (U4e's file); server/tests/schedule.rs:347 pins `npt_server::BITES` at PM-0004-67's three, which U4e leaves as they are (2.2) |
| R34 | the header and the event section as built (W1-N2) | U4a | the encoder, the client's rebuilds, carry 50 (2.1, 2.3) | new: `Snapshot { tick: u32, baseline_age: u16, newest_received: Option<u16>, last_run: Option<u16>, group, groups, own, entities, events: Vec<(u16, u8, SimEvent)> }` (snapshot.rs:443-463); the wire text (snapshot.rs:21-35): an event's number is its low 16 bits and its age 8 bits; `FullState { tick, last_run, own, entities }`, no baseline and no events (:630-639) |
| R35 | the records-to-volumes function and the static graph (W1-U4D A-M1; W1-PUB) | U4d | the ring and AreaFlood (2.1) | new: `World::hit_volumes_from` (world.rs:874-891) is the one function from records to volumes; the room pushes the world's own `hit_volumes()` (seam.rs:557) and rebuilds none. The static graph: `World::greybox()` (world.rs:967) or `Greybox::dm_tri()` (greybox.rs:83), with `areas()` and `portals()` (greybox.rs:166, :171) |
| R36 | `Loopback` (U2) | U2 | e7, 2.5, the Loopback tests (2.4, 2.5) | new: no loss, no delay, one order for all channels (link/src/lib.rs:128-131); built on `Rc` in one thread (:132-135), so not `Send`, and nothing `Send` may hold one (§4.11 :401-402); `rtt_ns` is `Some(0)` (:203-205) |
| R37 | the catalogue's rows as built | U4a, U4d (N8's edits) | the own-state section and the header (2.1, 2.3) | new: the own-state row now states the section as built, the owner's soldier record, its `OwnState` record and its effect rows (catalogue :136); the header row carries the 16-bit command sequences (catalogue :135) |
| R38 | the base counts and check.mjs's steps at 39f4290 | the main session | section 4 rule 5; section 5 | new: measured 365, bites 412, self 52 of 52, 10 steps (u4d-report.md:68-70); at d50396a (U4j's merge): measured 396, bites 443, self 52 of 52, 10 steps (u4j-build.md, "Merge into the spike branch", M5) |

Rows changed: R4, R6, R11, R15, R19, R20, R27, R28 (8 of 28); the other 20 hold. Rev 3.1 rewrote R31's route by close
B2. Rev 3.2 rechecked R27, R28 and R38 at d50396a (U4j's merge).

### 7.2 Left for the main session at launch
1. **`<base>`.** Fill it with the spike branch's head. If spikes/ at `<base>` differs from 39f4290 (`cd
   /home/user/Shamble-Frontlines && git diff --stat 39f4290 <base> -- spikes/netcode-proof`), give `<base>`'s counts
   from a counted run in the main folder on its own cache (§7.1 item 6, :900-901): `cd
   /home/user/Shamble-Frontlines/spikes/netcode-proof && node check.mjs`, reading its test step's line ("measured build:
   N tests passed …; bites build: M …"), self's "K of K" and the step count. (`<base>` holds U4j's merge and the
   carry-51 commit, so this run is always needed; the last counts known are d50396a's: 396, 443, 52 of 52, 10 steps.)
   **Filled at launch (the main session, 2026-11-12 about 02:05 UTC):** `<base>` is
   28a85e773e665f9944c3ab8bd606337298bd3273, the carry-51 commit (L3), on U4d-F's merge (adb1bab) and U4j's
   (d50396a). Its counts, from a counted run in the main folder (main-run.sh, the lane lock held exclusively;
   .gate/u4e-l3/l3-check.log; npt-sim compiled from the main folder's path): check.mjs 10 steps, all PASS; measured
   399, bites 446, self 52 of 52. `<base>` differs from 39f4290 under spikes/ (U4j's crates/judge, crates/log and
   marks/; U4d-F's and L3's crates/sim; L-c8's results).
2. **Carry 51's home** (R31; close B2): add `Scene::effect_rows()` (pub, the rows per soldier) and its §4.1 line
   ("`Scene::effect_rows` (an effect row's owner, the one public home: carry 51)") in your own commit when U4e's build
   starts, so `<base>` holds it and item 1 applies. Any other home U4e finds it needs is a seam change for the advisor
   (carry 26's route). **Done:** 28a85e7 (`pub fn effect_rows(&self) -> u8` in scene.rs); its §4.1 line went in at
   1e8c188, one commit earlier.
3. **The pending items** (3.2): folded in before the check (2026-11-11), from advisor-u4d-close.md B1.2, B1.3, B2.
4. **U4E-Q12, U4E-Q13 and U4E-Q14**: answered by the advisor's check (advisor-u4e-brief.md) and written into 2.1, 2.4
   (e5), 2.5 and 2.6 (done).
5. **This brief's checks**: test-engineer's check of rev 3 (done: u4e-brief-r3-check.md; rev 3.1 answers it), then the
   advisor's check before the build (advisor :157).

---

## 8. Questions for the advisor

**Ruled and applied** (W1-U4E, wave1 :437-497; the codes stay stable):

| Code | Ruling | Applied in |
|---|---|---|
| U4E-Q1 | (a): U4e plants and unit-shows lt-cmd-seq and lt-press | 2.1, 2.6 |
| U4E-Q2 | (c), widened: the last-run rebuild and its proof here; all of -58 judged after U4g | 2.3, 2.6 |
| U4E-Q3 | (a): bites-only `npt_room::BITES`; npt-server keeps its one list | 2.1, 2.2, 2.8 |
| U4E-Q4 | (a): the tick16 ids stay in `npt_proto::BITES`; the endpoint takes its `Quant` from `Quant::with_bite` | 2.3, 2.6, carry 24 |
| U4E-Q5 | (a), with its condition: no Rewind, counted; the count is a replacement in -30's setup | 2.1, e12, carry 40 |
| U4E-Q6 | (a), with two conditions: the cap as data, provisional; U4f measures | 2.1 |
| U4E-Q7 | answered, confirmed | 2.1 |
| U4E-Q8 | (a): client/clippy.toml gains the bans | 2.1, 2.3, section 4 |
| U4E-Q9 | answered, confirmed | 2.2, e10 |
| U4E-Q10 | (a): -62/server-echo in npt-room; -45's and cues-from-replay are U4g's | 2.1, 2.6, carry 24 |
| U4E-Q11 | (a): U4e builds the server-side checks and their tests; U4f their -21 counters and runs | section 2, 2.2 |
| U4E-Q12 | (c): U4e plants and unit-shows -22/unbounded at the event queue's cap; U4f runs it judged; the plan's Bites line gains it | 2.1, 2.6 |
| U4E-Q13 | (a): the id `catalogue-5/owner-to-all`, in `npt_room::BITES` only, its home held by tests | 2.1, 2.6 |
| U4E-Q14 | (b), the gap tied to the ring: `npt_sim::ACK_RING_TICKS` tick periods on Loopback; the 1 s ages reported | 2.4 (e5), 2.5 |

**Ruled at the advisor's check** (advisor-u4e-brief.md; the questions as asked, kept for the record; the rulings are in
the table above):

**U4E-Q12 (-22/unbounded: which unit plants it?).** Does U4e plant PM-0001-22/unbounded at its `EventQueue`'s cap?
- Why it matters: §4.14 :474 puts the bite's site at "the event queue's cap", with the unit proof "headroom below 0";
  U4e builds the queue and its cap (plan :707; §4.7 :330-335; U4E-Q6), and §4.14 :453-454 plants each bite where its
  code lives, shown by a unit test in that sub-unit. But U4e's Bites line (:729-731) does not list it, and U4f, which
  moves -22 ("bite unbounded over a full S30": :801), lists no event-queue build (:792-795). Without a reading, the
  bite has no planting unit, or U4f plants it in U4e's code after the fact.
- Options: (a) U4e plants it with §4.14's unit proof (the queue's headroom below 0 under the bite, never in the clean
  build), and U4f runs it judged over a full S30; (b) U4f plants it, editing the room's queue then (U4f may: no lane
  holds the room's files after U4r, :864-865); (c) as (a), and U4e's plan line gains it as U4E-Q1's text did.
- Recommendation: (c), by U4E-Q1's reasoning (wave1 :443-444: the bite planted where its code lives, in the unit that
  builds that code); the run stays U4f's.

**U4E-Q13 (the owner-to-all bite's id).** What id does catalogue item 5's owner-to-all bite carry?
- Why it matters: U4e plants it (plan :731) and U4r's class audit runs it at SS P2 over 3 seeds (:750), so the bite
  needs an id that U4r's `--bite` dispatch selects per instance; but marks.json has none (catalogue item 5 is a
  catalogue test that no pass mark holds: the visibility classes are proven at slice 3, ADR 0001:1687-1688), and
  check.mjs's bite-home rule reads only ids of the form `PM-<ADR>-<row>/<name>` (check.mjs:689, :734-758).
- Options: (a) an id outside the PM form (for example `catalogue-5/owner-to-all`) in `npt_room::BITES`; check.mjs's rule
  does not read it, so a unit test holds that only npt-room lists it, and U4r selects it by that string; a room pin that
  checks each id against marks.json, as server/tests/schedule.rs:345-353 does, exempts that one id by name; (b) a
  PM-form id under no mark's row, which marks.json would not spell and check.test.mjs's pin of marks.json's ids would
  not cover; (c) a marks.json entry for it, through the main session (marks/ is not U4e's lane), for a bite no mark
  judges.
- Recommendation: (a): it claims no pass mark that does not exist, needs no other lane, and its one home is still
  tested.

**U4E-Q14 (the 1 s gap on Loopback: does it evict B?).** Does §4.14 :460's unit proof, "a 1 s gap evicts B;
`burst_eviction` = 1", keep its 1 s on Loopback?
- Why it matters: the ring keeps ages 0 to 31 (§4.5 :295-297), and a 1 s gap is 30 ticks at 30 Hz. Loopback neither
  delays nor loses (link/src/lib.rs:128-131), so B's age at the gap's start is 1 or 2 ticks, and at the first tick
  after the gap it is 31 or 32, by where the gap starts against the tick and when the client's inputs go out (e5a's
  two alignments). On L1, B's age at a gap's start grows with the RTT, so -18's judged runs are not this edge. If
  e5a's precondition fails at one alignment, e5b, 2.5's bite half and the no-fallback row are not shown, and U4e
  stops there for a ruling.
- Options: (a) as the brief now says: run both alignments, report both ages, bring a 31 to the advisor, and never
  lengthen the gap; (b) rule now that U4e's Loopback tests use a gap of 1 s plus two tick periods (in ns, a local
  value from the rate passed in, never a const or static: carry 39), which leaves B at age 32 or more at both
  alignments with a tick to spare, with the 1 s ages reported beside it; §4.14 :460 gains "on Loopback, 1 s plus two
  tick periods"; (c) align the gap so that it gives 32 (tuning the alignment is tuning the gap, so the brief does not
  offer it to the builder).
- Recommendation: (b): the unit proof exists to show a gap that evicts B, so that the bite can bite (§4.5 :298-299);
  Loopback's zero delay puts a 1 s gap exactly at the edge, which says nothing about -18 on L1, and a ruling before
  launch saves a stop. Until it is ruled, (a) applies.
