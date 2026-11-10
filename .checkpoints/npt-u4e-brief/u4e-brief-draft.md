# U4e builder brief: the room and the codec path (DRAFT, rev 2)

**Changes in rev 2** (answering test-engineer's check, test-engineer-u4e-check.md; rev 1 is commit ea2dd8a). 23
findings: 23 accepted (F1 with e5 split in two so both halves of its fix hold together), none rejected. Every Medium's
cites were checked at df8975b before applying it, as were F8's, F12's, F16's (but :627) and F17's; F16's :627, F18's
advisor :361, F19's :632 and F20's :833-834 are taken from the check, not reopened.
- F1 (Medium) accepted: 2.4 e5 split into e5a (B evicted, `burst_eviction` = 1: passes in both builds, §4.5 :289-290)
  and e5b (the first snapshot after the gap coded against the per-type defaults, §4.5 :287-288: the server half the
  bite breaks; under e5a's precondition); 2.6's no-fallback row now names e5b and 2.5's test as the proofs that fail by
  name, e5a as the one that shows the bite could bite. The check's two fix parts ("e5 passes in both builds" and "add
  the defaults assertion to e5") cannot both hold for one test; the split keeps both.
- F2 (Medium) accepted: 2.4 e5a records, on the injected clock, the gap's start and end, B's tick and B's age at the
  first tick after the gap, asserts age ≥ 32 before `burst_eviction` = 1, and reports (never lengthens the gap on its
  own) if a 1 s gap leaves B at age 31 or less (plan §4.3 :249; §4.5 :286-287; §4.14 :445).
- F3 (Medium) accepted: section 2's out-of-scope paragraph names U4f :775's "server-side checks" overlap with U4e's
  :693-694, :704, :712 and its reading; new advisor question U4E-Q11 (section 8).
- F4 (Medium) accepted: 2.4 e6 reworded as an assertion (budget = per-tick budget minus the sum of (payload + 28 B) of
  the pump's k pings and pongs; the snapshot within that remainder: §4.5 :291-293).
- F5 (Medium) accepted: 2.4 e12's carry 22 test reworded with its expected outcome (no `Rewind` naming N + 1; what it
  returns instead: U4E-Q5).
- F6 (Low) accepted: 2.4 e12's carry 23 test reworded (the tick it ran, consecutive 32-bit numbers across a 16-bit wrap,
  the counter started near the wrap).
- F7 (Medium) accepted: section 4's "Write and Edit only: no shell writes, no generated edits" now allows each bite's
  scratch copy to be made by a scratch script under section 4's rules (plan §7.2 :881-882).
- F8 (Low) accepted: section 4 rules 2 and 5 now cite where earlier builders' and reviewers' briefs carry them
  (u4s-fix-review-rust.md:6-7, u3d-build.md:7, u4s-review-rust.md:30 at df8975b); `<floor>` stays for the day session,
  with those briefs' 2 GB named.
- F9 (Low) accepted: 2.4 e7 adds "1,000 compared ticks, every pair equal, none the sentinel 0".
- F10 (Low) accepted: 2.5 and section 6 item 5: "within 500 ms of the gap's end, on the injected clock".
- F11 (Low) accepted: 2.4 e9: each negative case its own named test (the five JoinRejected reasons may be one table test
  that names the failing reason).
- F12 (Low) accepted: 2.6's tick16 and client-rounding proofs reworded as assertions.
- F13 (Medium) accepted: U4E-Q2's why adds that the failing check (§4.14 :454, "0 untraced corrections") needs U4g's
  Predictor; option (c) added; recommendation now (c), with that reason.
- F14 (Medium) accepted: U4E-Q4's option (a) says it leaves carry 24's tick16 half unmet (reported so); (c) named as the
  option that meets carry 24 as worded, at the cost of an id marks.json may not spell (R27).
- F15 (Medium) accepted: U4E-Q6's option (a) names U4f, which moves -22 (plan :779-782), not U4r.
- F16 (Low) accepted: U4E-Q7 answered by the plan (§4.12 :397-398; U4r :719-720; npt-log's `hs` U4j's, :627); the
  answer stated in 2.1 and R28; the question dropped (its code kept, marked answered, so the other codes stay stable).
- F17 (Low) accepted: U4E-Q9 answered by the F2 ruling (advisor :449-452, :459-460); applied in 2.2 (a new bullet) and
  e10 (the pin); the question dropped (code kept, marked answered).
- F18 (Low) accepted: R19's cite is now Lanes :827 and advisor :361.
- F19 (Low) accepted: R27's "Built by" is now "marks.json at `<base>` (U4j edits it: plan :632)".
- F20 (Low) accepted: 2.4 e1: "crates/proto is not in U4e's lane (Lanes :833-834)".
- F21 (Medium) accepted: section 4 rule 4 records the base counts on check.mjs's measure (the `test result:` sums of
  the two runs, check.mjs:1321-1331, :1378-1385); `--list` (flag before `--`) only names the new tests; section 6 item
  3 follows.
- F22 (Medium) accepted: R24's check reads the diff line by line against Lanes :824-827 instead of "additions only".
- F23 (Low) accepted: section 5 item 4 asks whether the room copies `step`'s events anywhere beyond the EventQueue's
  stamped entries (file:line); section 3's note follows.
- Questions (point 4 of the check): Q1 narrowed to a confirm (the plan's own rule gives (a): §4.14 :438, plan :691,
  :737); Q7 and Q9 dropped as answered (above); Q11 added. Open: Q1 (confirm), Q2, Q3, Q4, Q5, Q6, Q8, Q10, Q11.

---

Drafted by netcode-architect on the night shift of 2026-11-10 (night item npt-u4e-brief, step 1 of 3); checked by
test-engineer (step 2) and revised once (step 3, this rev). Next: the day session brings it up to date after wave 1
merges (the "Recheck after wave 1" table, section 7) and sends it to the advisor, who checks each of the five briefs
before its build (advisor-u4.md:157; Q21 at :220). Section 8 holds eleven question codes: nine open for the advisor,
two (Q7, Q9) answered by the plan and a ruling and kept only so the codes stay stable; the brief marks each place that
waits on one.

**How to read the citations.** Every file:line is at df8975b. `plan` is
.checkpoints/netcode-proof-test/u4-build-plan.md; `carries` is .checkpoints/netcode-proof-test/u4s-carries.md;
`advisor` is .checkpoints/netcode-proof-test/advisor-u4.md; `plan.md` is .checkpoints/netcode-proof-test/plan.md;
`catalogue` is docs/design/netcode-proof-messages.md; a crate path (`room/src/lib.rs`, `seam.rs`) is under
spikes/netcode-proof/crates/. "Needs a run (day)" marks what only a build or a run can settle, with its command.

---

## 1. Start

1. **Base:** `<base>` (the day session fills it in: the spike branch's head after U4a, U4b and U4d have merged, since
   U4e depends on all three: plan :685; Lanes, plan :832-836). Record it in your notes before anything else, with
   `git log -1 --format=%H` in your worktree.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree (`isolation: worktree`),
   under §7.1's shared-target rule (plan :880-882; section 4 below). U4e runs in wave 2 beside U4d2 (plan :832-836):
   two lanes, never sharing a file.
3. **Your notes:** `.checkpoints/netcode-proof-test/u4e-build.md` in your worktree (the day session confirms the
   folder against wave 1's builders). Its first line names this task; then your plan as blocks (one block: one type
   with its tests, one test group, one bite and its proof); each finished block written at once, a notes line per
   finding, and one line after each block (done, what next). On a restart, continue after the last block marked done.
4. **Read before you build** (and nothing you do not need):
   - plan §5 U4e (:685-716), the sections it builds to: §4.1 (:178-201), §4.2 (:203-242), §4.3 (:244-258), §4.4
     (:260-280), §4.5 (:282-294), §4.6 (:296-318), §4.7 (:320-326), §4.9 (:333-347, the integer half only), §4.11
     (:379-394), §4.14 (:437-460); Lanes (:814-845); §6 (:846-863); §7 (:864-887);
   - plan.md §4.7 (:568-616): each crate's whole public surface (npt-room :595-597, npt-server :598-599, npt-client
     :608); plan.md:454 (everything else `pub(crate)`);
   - advisor: the U4 verdict's conditions 6 (:125-129) and 10 (:143-149) and nit (d) (:164-165); "Rulings (U4s)"
     (:349-375) and its conditions (:304-335); "Ruling: F1 and F2" (:401-475), whose F2 part binds U4e (:436-465);
     the advisor's answers to section 8's questions (the day session adds where they are recorded);
   - carries §U4e (:145-174): entries 21-27 and 39, quoted in section 3;
   - catalogue §1 (:52-78), the wire rows (:87-101), §3 Field groups (:125-158), §6 at U4 (:240-253);
   - .claude/rules/netcode.md: binding on all proto, room, client and server code (plan §7.1 item 8, :876-877);
   - the code as merged at `<base>`: sim/src/seam.rs, grid.rs, ticks.rs, history.rs; proto (U4a's); server/src/
     schedule.rs (U4b's); room, server and client as they stand (at df8975b: room/src/lib.rs:1-5 and
     server/src/lib.rs:1-5 are doc stubs; client/src/lib.rs:8-22 holds `BITES` and `grid_with_bite`); check.mjs's
     deps step (:1447-1472), whose tick, float and bite-id checks are :581-637, :657-671 and :723-747.

---

## 2. What to build

U4e builds plan §5 U4e (:685-716) exactly: its Builds, its Tests, -18's bite end to end and its Bites. It moves no mark
(plan :713: "none judged (U4r runs them)"). Out of scope, by the plan: option A, `ClockSync` and every other room-side
check with its -21 counter (U4f, plan :774-776); `Predictor`, `Interp`, `CueLog` (U4g, :786-790); `rewind_at`'s
tolerance and cap (U4h, :691, :799-800); AddrTable and the handshake cap (U6, :695, :851-853); token minting (U6,
:687, :853); fuzzing (U5, :847). One overlap in the plan: U4f :775 also lists "the server-side checks (size, trailing
bytes, wrong channel, the bucket)", which U4e's §5 builds (:693-694) and tests (:704, :712); U4e builds and tests them
as its §5 says, and this brief reads U4f's line as their -21 counters and judged runs (U4E-Q11 asks the advisor to
confirm). Where this brief finds the plan silent it says so; it adds nothing.

### 2.1 npt-room (crates/room/src/, crates/room/clippy.toml)
Each item from plan :687-692, built to the section named.
- **`Room<S: SimCore>`** (§4.1 :193; plan.md:595): drives the sim through the seam alone (seam.rs:268-287); owns its
  `HitHistory` (built with `HitHistory::for_ms`, history.rs:22) and pushes `hit_volumes()` into it after each `step`
  (§4.2 :222-224); fills one `TickInput` per tick (seam.rs:79-88) with the accepted commands in the canonical order
  (seam.rs:82-86; carry 21); consumes or copies `step`'s events before it reads `hit_volumes`, `reports` or
  `entity_records` (carry 26); reads `reports()` and logs them, never into the hash (§4.2 :210-212). Bites per instance
  through `Room::with_bite` or a config value passed in, never a static (§4.11 :386-388).
- **`Roster`, the slot token compare, `ClientSlot`** (:687): tokens passed in by the caller (the lab at U4r; minting
  is U6's); the compare in constant time (catalogue :93); `bind` bound at the slot's first accepted `Join` (catalogue
  :93, :112-115); no join beyond the roster's N; the five JoinRejected reasons (catalogue :95).
- **`AckRing`** (§4.5 :282-288): ages 0 to 31 kept, 32 evicted, at 30 Hz; its length is `npt_sim::ACK_RING_TICKS` by
  path (ticks.rs:70; advisor :439-440); baseline B is the newest sent snapshot whose every group the client
  acknowledged; a partly acknowledged tick is never a baseline; a B older than the ring means the per-type defaults.
- **`Encoder`** (:688-689): the window's remainder (§4.5 :291-293: the room owns each client's 256 kbps window; the
  pump's bytes sent since the last tick count first); groups with index and count, all against one B, each decoding
  alone (§4.4 :273-274; §4.5 :283-285); the layout table (catalogue item 4, :247-248; whose home, U4a's or the
  encoder's: recheck row R6); the class filter (catalogue item 5, :249-252); the mirror hash of §4.6 (:300-302) through
  npt-proto's `client_hash` (U4a, plan :519, :537); one tick's whole snapshot exposed as a unit (§4.5 :294); encoding
  through npt-proto's `EntityRecord` ↔ wire mapping and `Quant` (§4.4 :267-269), never a second mapping. Each burst
  yields `burst_eviction` (§4.5 :289-290). Who writes it, the plan's answer: U4e exposes `burst_eviction`,
  `snap_incomplete` and the client hash as counters and per-tick values, and U4r's per-process logs write them (on L1
  one npt-lab process writes one log per process value: §4.12 :397-398; U4r builds those logs: plan :719-720; npt-log's
  `hs` is U4j's: :627). (This was U4E-Q7 in rev 1.)
- **The window's rate (the F2 ruling, advisor :443-449):** the window's per-tick budget takes the tick rate as an
  argument; production callers pass `npt_sim::TICK_HZ` by path; a test pins it at 30 and 60 Hz (32,000 B a second at
  both); no const or static outside npt-sim is derived from TICK_HZ.
- **`EventQueue`** (§4.7 :320-326): bounded, its cap as data; an overflow forces one resync; a FullState not
  acknowledged within `npt_sim::FULL_STATE_ACK_TICKS` (ticks.rs:56; advisor :439), or a second overflow inside it,
  sends Kick(overflow) and drops the client; event numbers 16-bit on the wire, rebuilt to 32 bits by `Wrap`; each
  `SimEvent` stamped with its tick and 32-bit number when queued (carry 23; the events at seam.rs:214-232 carry
  neither). The cap's value: U4E-Q6.
- **`InterestGrid`** (integer cm), **`AreaFlood`** (re-floods from `portal_states` when they change: §4.2 :214-215),
  **`Priority`** (catalogue :129-130: own state always, the nearest 8 enemies in every snapshot, every relevant entity
  at least every 4 snapshots). Float-free (§4.11 :384-385).
- **`InputBuffer`, option B, with `buffer_depth_ticks`** (:690); `InputBuffer::CMD_CATCHUP` (carry 27; §4.3 :254).
  Option A is U4f's (:774).
- **`rewind_at`, the integer half** (:690-691; §4.9 :337-339): a command's view time (seam.rs:38-42) to a `Rewind`
  (seam.rs:67-74); the debug assert that a correct room never sends a Rewind the ring does not hold sits here (§4.2
  :219-221); carry 22. The tolerance and the cap land in U4h (:691). What it does with a claimed view time outside the
  ring before U4h: the plan is silent (U4E-Q5).
- **The room-side checks -58's counters pass through** (:691): the sequence-forward compare, the press-edge compare
  and the ack rebuild (§4.14 :452-453, :456), each through npt-proto's `Wrap` (§4.4 :275-276), every counter with an
  injectable start value (§4.4 :276). Their bites: lt-ack (2.6), and lt-cmd-seq and lt-press by the plan's own rule
  (§4.14 :438; U4E-Q1 asks the advisor to confirm).
- **Resync, one in flight** (:691): a second `Resync` before the FullState's ack dropped and counted (catalogue :90).
- **crates/room/clippy.toml gains the hash-map type bans** (:691; §4.11 :380), beside its clock ban (clippy.toml:1-9).
  §4.11 :380 also names client's clippy.toml; §5 U4e names only room's: U4E-Q8.
- **Float arithmetic:** carry 25.

### 2.2 npt-server's library (crates/server/src/, but schedule.rs, which is U4b's)
From plan :693-695; §4.4 :277-280 (A26); §4.5 :291-293; §4.9 :341-345.
- **`Decoder::pump`** (§4.1 :194): the I/O step over `&mut dyn Link` with an injected `now`: decodes every inbound
  message (catalogue :53-55); the socket checks (size first, at the socket, before decoding: catalogue :65-71); the
  wrong channel and trailing bytes as npt-proto's `WrongChannel` and `TrailingBytes` errors, counted by the pump, the
  only caller of the decoder on client input (§4.4 :277-280); the ping sender (a few a second: catalogue :91) and the
  answers with their hold time (catalogue :92; its limit: the bullet below); per-client RTT (from the link's `rtt_ns`
  for an L1 or `wt` session: §4.9 :341-343) and bytes sent per client since the last tick (payload + 28 B each: §4.5
  :291-292).
- **`ConnLimits`, `Counters`** (plan.md:598-599): the rate buckets (the Input bucket: 8 datagrams, refilled at 2 ×
  TICK_HZ a second: catalogue :87; the Ping limit: 10 a second per session: catalogue :91) and the 2 s Join deadline
  (catalogue :93), all on the injected now, in ns.
- **The Input bucket's rate (the F2 ruling, advisor :443-447):** the bucket takes the tick rate as an argument;
  production callers pass `npt_sim::TICK_HZ` by path; a test pins the refill at 30 and 60 Hz (60 and 120 a second); no
  const or static outside npt-sim is derived from TICK_HZ.
- **The Pong hold time's limit** ("one tick period, 33.3 ms at 30 Hz": catalogue :92), by the F2 ruling: point 2's
  pattern gives the 60 Hz pin wherever no constant is needed (advisor :459-460): the pump takes the rate as an
  argument, production callers pass `npt_sim::TICK_HZ` by path, and a test pins the limit at 30 and 60 Hz (e10); if
  the limit turns out to need a named constant, it takes route (b): no local form, the need in your report (advisor
  :449-452; carry 39). (This was U4E-Q9 in rev 1.)
- **Keep U4b's `mod schedule;` line** in server/src/lib.rs (advisor :369-370).
- **Not built:** AddrTable and the handshake cap (:695; U6), `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, the
  binary (§4.1 :201; §6 :851-852). server/src/main.rs stays as merged.

### 2.3 npt-client (crates/client/src/)
From plan :696-697; §4.6 :303-312.
- Decoding into integer frames and held floats; the complete-tick rule and the sentinel 0 (§4.6 :307-312): every
  group decoded, its hash logged under the tick it rebuilt; a received group it could not decode, the sentinel; a
  group lost and none refused, nothing, and `snap_incomplete` = 1.
- The client side of `client_hash` (§4.6 :303-306): its held floats turned back into grid integers through `grid`,
  with the 32-bit tick and event numbers as it rebuilt them.
- Acks with the group mask (§4.5 :284).
- A minimal **`ClientEndpoint`** (plan.md:608): input, step, the next due time in ns (:697). Every call into
  `npt_sim::grid` takes its config from `grid_with_bite` under the bite (carry 24; client/src/lib.rs:14-22).
- A message on the wrong channel dropped and counted (:705-706; catalogue item 2's "on both sides", :243-244; nit d,
  advisor :164-165); a datagram above 1,150 B refused (catalogue :88).
- The frame ring keeps the same ages as the AckRing (`npt_sim::ACK_RING_TICKS` by path: §4.5 :287-288; §4.3 :249).

### 2.4 Tests (plan :698-706), each named in your report with its file
- e1 catalogue item 4: the layout table equals the Field-group rows for `Snapshot` and `FullState`, each with the same
  class; the authority-only groups have no entry (:698; catalogue :247-248). It lives in U4e's crates (crates/proto
  is not in U4e's lane: Lanes :833-834), reading the catalogue as proto/tests/catalogue.rs does.
- e2 a forced group split, each group decoding alone (:698-699).
- e3 a lost group leaves its tick unlogged and counted; a refused one logs the sentinel (:699-700).
- e4 the baseline rule and the ring's age edges: age 31 kept, age 32 evicted (T15; §4.5 :286-287); a partly
  acknowledged tick never a baseline.
- e5 a 1 s gap evicting B (:700; §4.14 :445's unit proof, the server half), as two tests, so the one that shows the
  bite could bite stays apart from the one the bite breaks:
  - e5a: on the injected clock it records the gap's start and end, B's tick and B's age at the first tick after the
    gap, and asserts that age ≥ 32 (`npt_sim::ACK_RING_TICKS`; §4.3 :249; §4.5 :286-287) before it asserts
    `burst_eviction` = 1. The ring holds 32 ticks = 1.067 s at 30 Hz, so a 1 s gap (30 ticks) evicts B only with B's
    age at the gap's start counted in; if a 1 s gap leaves B at age 31 or less, report it (§4.14 :445's "a 1 s gap
    evicts B" then does not hold on Loopback) and do not lengthen the gap on your own. e5a passes in both builds
    (`burst_eviction` says whether B left the ring, which the bite does not change: §4.5 :289-290).
  - e5b, under e5a's precondition (B's age ≥ 32 at the first tick after the gap): the first snapshot after the gap is
    coded against the per-type defaults (§4.5 :287-288). It passes in the clean build and fails by name under
    no-fallback.
- e6 the window counting the pump's bytes (:700-701; §4.5 :291-293): with the pump reporting k pings and pongs since
  the last tick, the encoder's budget for that tick equals the per-tick budget minus the sum of (payload + 28 B) over
  those k; the snapshot sent that tick stays within that remainder.
- e7 the mirror equal to the client's hash over Loopback at P0 for 1,000 ticks (:701): 1,000 compared ticks, every
  pair equal, none the sentinel 0. The scene is not named by the plan: say which you used (recheck row R18).
- e8 the Send asserts: compile-time asserts that `Room<World>`, `Decoder` and `ClientEndpoint` are `Send` (:701;
  §4.11 :389-390).
- e9 the negative cases (T17, A15; :701-706), each its own named test (the five JoinRejected reasons may be one table
  test that names the failing reason): each JoinRejected reason (bad token, bind mismatch, roster full, wrong version,
  wrong content hash); a datagram before Join dropped and counted; a control message before Join closing the session
  as malformed, counted, never INVALID; the Input bucket (8, refilled at 60 a second) and the Ping limit (10 a second)
  on a fake clock; the 2 s Join deadline on a fake clock; a second Resync before the FullState's ack dropped and
  counted; npt-client dropping and counting a message on the wrong channel.
- e10 the F2 pins: the Input bucket's refill at 30 and 60 Hz (60 and 120 a second); the window at 30 and 60 Hz
  (32,000 B a second at both) (advisor :446-447); the Pong hold time's limit at 30 and 60 Hz (one tick period: 2.2).
- e11 one bite instance and one clean instance on parallel threads in one process (§4.11 :386-388).
- e12 a test per carry that names a behaviour: carry 21 (commands received in two orders give one `TickInput`);
  carry 22 (with N the newest tick pushed into the ring, a command whose view time gives `tick_a` = N and
  `fraction > 0`: `rewind_at` returns no `Rewind` naming N + 1; what it returns instead: U4E-Q5's ruling); carry 23
  (each queued event carries the `TickInput::tick` it ran and consecutive 32-bit numbers, across a 16-bit wrap, the
  counter started near the wrap by its injectable start value); carry 27 (the const assert).
- Plan silent (not added): `Join` on a slot with a live session replacing it (`Kick(replaced)`, catalogue :93) has no
  U4e test in the plan; say in your report whether it is built and tested.
- Where U4e's tests live (advisor condition 10 (b), :145-146; Lanes :834-836): in its own crates, never in
  crates/lab; the Loopback tests in npt-server's tests, through its npt-link and its dev dependency on npt-client
  (server/Cargo.toml:32-35).

### 2.5 -18's bite end to end (plan :707-710; the advisor's condition 6, advisor :125-129)
§4.14's unit proof (e5) is the server half only. U4e's proof runs from the encoder to the client, through a 1 s gap in
delivery: under no-fallback the client refuses every delta after the gap, so no fresh delta decodes within 500 ms of
the gap's end, on the injected clock; the clean build decodes one. If the bite's baseline turns out to be one the
client still holds, the bite is not a defect on L1, and -18's L1 part comes to the advisor before it counts (report
it; do not count it).
- The gap: Loopback neither loses nor delays (link/src/lib.rs:129-131), and the plan leaves the mechanism open. A
  test-only `Link` wrapper in npt-server's tests that drops every datagram both ways for 1 s on the injected clock is
  inside the lane; its length in ns, never a `*_TICKS` constant (carry 27). Say in the report what you built.

### 2.6 Bites (unit-shown; plan :711-712; sites from §4.14 :441-460)
Each planted where its code lives, chosen per instance (§4.11 :386-388), shown to change state by a unit test that
fails by name in the bites build (§7.1 item 3, :869-870), and passes in the clean build.

| Bite (plan :711-712) | Site | Unit proof |
|---|---|---|
| PM-0001-18/no-fallback | the encoder's baseline choice, never falling back to the defaults (§4.14 :445) | e5b (the server half) and 2.5's end-to-end test both fail by name in the bites build; e5a passes in both builds (it shows the bite could bite: §4.5 :289-290) |
| PM-0001-17/tick16 and PM-0001-58/tick16, the client half | the client's tick rebuild, the 16-bit value taken as the tick (§4.14 :443) | after 65,535, the client rebuilds 65,537 from its 16-bit value and logs a hash under 65,537 equal to the mirror's; the bite logs it under another tick or as the sentinel; the ids' home: U4E-Q4 |
| PM-0001-17/client-rounding, the client's call site | `ClientEndpoint`'s calls into `grid` through `grid_with_bite` (§4.14 :444; carry 24) | over a sweep of position values including half-cm ties and negatives, the client's grid integers equal the server's; the floor bite breaks about half (§4.14 :444) |
| PM-0001-58/lt-ack | the room's rebuild of the 16-bit ack (§4.14 :456) | a post-wrap ack refused |
| PM-0001-58/lt-tick-offset | the client's "last command run" compare (§4.14 :454) | "replay starts at the wrong command": U4E-Q2 |
| PM-0001-58/lt-event-seq | the client's event-number rebuild (§4.14 :455) | a post-wrap event rebuilt wrong |
| owner-to-all (catalogue item 5's) | the encoder's class filter: one `owner` group sent to every client (catalogue :251-252) | a test that every group a client receives is one its class lets it see fails; the L1 audit is U4r's (plan :731) |
| no-trailing and no-channel, through the pump | npt-proto's decode (U4a's sites: plan :533-536; §4.4 :279-280) | with the bite, the pump's `TrailingBytes` or `WrongChannel` count stays 0, and the pump test fails |

- Bite ids are `PM-<ADR>-<row>/<name>` (check.mjs:678); each id lives in one crate's sources (check.mjs:712-747;
  carry 24). The owner-to-all id is not spelled in the plan: use marks.json's, if it lists one (recheck row R27).
- -58/lt-cmd-seq and -58/lt-press: their sites are U4e's (§4.14 :452-453; plan :691), and the plan's own rule plants
  each bite where its code lives (§4.14 :438), but U4e's Bites line omits them; plant and unit-show them (post-wrap
  commands refused; an edge lost) unless the advisor's answer to U4E-Q1 says otherwise.

### 2.7 The seam rule, and what stops the build
- No existing item of the seam changes: `SimCore` and its methods, `Command`, `Rewind`, `CoverVariant`, `TickInput`,
  `EntityKind`, `EntityRecord` (FIELDS), `HitVolumes`, `PortalStates`, `SimEvent`, `ClampReport`, `InputReports`
  (seam.rs:21-287), `HitHistory` (history.rs), `grid` (grid.rs) and the tick constants (ticks.rs), as U4d merged them;
  nor any npt-proto item (U4a's) or `Schedule`, `Due`, `due_ticks` (U4b's). crates/sim, crates/proto and
  server/src/schedule.rs are not U4e's files.
- A needed change stops that part: write it in your notes and report (the item, the change, why, the site), build the
  rest, and leave the change to the main session, which takes it to the advisor (carry 26's route).
- A tick count or a named constant derived from the rate: no local form; the need goes in your report (carry 39 and
  its route, quoted in section 3).

### 2.8 Public items (A19; plan §7.2 :885-886)
- Allowed: npt-room's surface, plan.md:595-597 (`Room`, `Roster`, `ClientSlot`, `InputBuffer`, `EventQueue`,
  `AckRing`, `Encoder`, `InterestGrid`, `AreaFlood`, `Priority`, `rewind_at`) with §4.1's `Room<S: SimCore>` and
  `InputBuffer::CMD_CATCHUP` (:193); npt-server's U4e part of plan.md:598-599 (`Counters`, `ConnLimits`, `Decoder`,
  with §4.1's `Decoder::pump`, :194); npt-client's `ClientEndpoint` and `StepOut` (plan.md:608) beside U4s's `BITES`
  and `grid_with_bite` (§4.1 :192). Everything else `pub(crate)` (plan.md:454).
- Your report lists every pub item U4e adds (types, functions, methods, fields, constants), by crate, against §4.1 and
  plan.md §4.7; engine-architect checks them (check.mjs does not: plan :886). A bites-only id list in npt-room or
  npt-server is on neither list: U4E-Q3.

---

## 3. The carries, word for word

Quoted from carries :145-174 (the advisor's U4s ruling, condition 4: advisor :334-335). Your report says of each, by
number, "met" (with the file:line and the test that shows it) or "not met" (with why).

```
## U4e (the room, the server I/O step and the client)
21. **netcode F2 (the room's half):** "The room sorts each tick's accepted commands into the canonical order
    (seam.rs `TickInput::commands`) before `step`; -30's comparer relies on it."
22. **netcode F5 (the room's half):** "`rewind_at` never names `tick_a + 1` beyond the newest tick pushed into the ring
    when `fraction > 0`."
23. **netcode F12:** "The room stamps each `SimEvent` with the tick it ran (`TickInput::tick`) and its 32-bit event
    number when it queues it; the sim's events carry neither."
24. **arch F1 (-17's half):** "`ClientEndpoint` takes the grid for every call it makes into `npt_sim::grid` from
    `npt_client::grid_with_bite` (U4s) under `PM-0001-17/client-rounding`; `npt_client::BITES` grows with U4e's other
    ids (-17/tick16's client half, -58's, -45's, -62's); no id appears in two crates' lists."
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
```

Eight carries (21-27 and 39). Notes for the builder, which change no word: carry 24's "-45's, -62's" name ids whose
sites U4g builds (U4E-Q10); carry 24's tick16 half meets check.mjs's one-crate rule (U4E-Q4); for carry 25, a
"why not" is reported, never assumed; for carry 26, section 5 item 4 says what to report.

---

## 4. Build rules

**The shared-target rule** (Mustafa's, 2026-11-09; plan §7.1 :864-877), binding on every cargo call in your worktree
and in each bite's scratch copy:
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` on every cargo call; checked once after the
   first build (the build's artifacts are in that folder, not in the worktree's `target/`) (item 1, :866-867).
2. `CARGO_NET_OFFLINE=true` on every cargo call: U4s made every manifest and Cargo.lock change wave 2 needs (Lanes
   :815-819), so nothing is fetched. (Plan §7.1's text does not name this rule or rule 5; earlier builders' and
   reviewers' briefs carry both: u4s-fix-review-rust.md:6-7, u4s-review-rust.md:30, u3d-build.md:7, at df8975b. The
   day session aligns their wording with wave 1's briefs.)
3. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (item 2, :868).
4. Counts are deltas (item 3, :869-870; Lanes :839-840), on check.mjs's own measure: its test step runs
   `cargo test --locked --workspace` and then the same with `--all-features` (check.mjs:1378-1385), and its count
   (cargoCounts, check.mjs:1321-1331) sums the "N passed; M failed" of each run's `test result:` lines. Before the first
   build, record `<base>`'s two counts that way: check.mjs's test-step line at `<base>`, or the sum of the `test
   result:` lines of those two runs. Each run's count matches the base plus the tests you added; a bite run shows its
   own test failing by name; a mismatch means another copy's artifact ran: touch, rebuild, never report that run.
   `--list` only names the new tests (the flag before `--`: `cargo test --locked --workspace --all-features -- --list`);
   it is not a count (it lists ignored tests too and runs nothing).
   Needs a run (day), at `<base>`, from spikes/netcode-proof/:
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true cargo test --locked --workspace`
   and `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true cargo test --locked --workspace --all-features`,
   each run's `test result:` lines summed.
5. `df -h /home/user/Shamble-Frontlines/.gate` before each build, the reading in your notes; below the floor wave 1's
   briefs set (`<floor>`, the day session fills it; the earlier briefs above stop under 2 GB), stop and report; never
   clean anything to make room.
6. After the last bite, touch and rebuild before any further claim (item 4, :871).
7. Cargo's own lock serializes your builds with U4d2's: expected (item 5, :872). No copy cleans the shared folder
   (item 7, :875).

**Your files, and only these** (Lanes :832-836): crates/room/ (src/, tests/, clippy.toml); crates/server/src/ but
schedule.rs (U4b's), and crates/server/tests/; crates/client/ (src/, tests/; clippy.toml only if U4E-Q8 says so).
- No manifest edit: no Cargo.toml (workspace or crate) and no Cargo.lock; U4s made them (Lanes :815-819; npt-room's
  and npt-client's `bites` features: room/Cargo.toml:12-15, client/Cargo.toml:12-15; npt-server's dev dependency on
  npt-client: server/Cargo.toml:32-35). A needed manifest change stops that part and goes in your report.
- server/clippy.toml is U4b's (plan :543-544); the plan names no U4e change to it.
- Never crates/sim, crates/proto, crates/lab (U4d2's alone in wave 2: advisor :145-146), check.mjs (U4d2's in wave 2:
  Lanes :832-833), marks/, docs/.
- Write and Edit only in your worktree's sources: no shell writes there and no generated source edits. Each bite's
  scratch copy may be made by a scratch script under this section's rules (plan §7.2 :881-882: "the bites planted by a
  scratch script under §7.1"). Commit nothing and push nothing; the main session merges (plan §7.2 :880-882).
- .claude/rules/netcode.md binds proto, room, client and server code (plan :876-877): validate every inbound message
  before use, no unbounded buffer, map or queue keyed by client data, no clock in shared code.

---

## 5. End checks

Run from spikes/netcode-proof/ in your worktree, under section 4's rules (touch and `df` before each):
1. check.mjs twice, each with every step passing: `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true node check.mjs`.
   At df8975b it has 10 steps (fmt, clippy, clippy-wasm32, doc, test, deps, refs, marks, self, redact:
   check.mjs:1474-1507); the tick, float and bite-id checks run inside deps (:1466-1468). The day session states the
   step count at `<base>` (recheck row R25). Report both runs' last lines ("check: all N steps pass") and the test
   step's two counts.
2. clippy on the host and on wasm32, in both builds, warnings as errors. check.mjs's clippy and clippy-wasm32 steps
   do this (lintBothBuilds, :1352-1357; the wasm32 crates are LEAF plus npt-tcpinfo, :28 and :1348, which include
   npt-room, npt-client and npt-proto, not npt-server); state each step's line.
3. fmt: check.mjs's fmt step (`cargo fmt --all --check`, :1475).
4. Carry 26: report whether the room copies `step`'s events anywhere other than into the EventQueue's stamped entries
   (carry 23) before it reads `hit_volumes`, `reports` or `entity_records` (file:line); if it does, carry 26's seam
   change goes to the main session (2.7). Needs a run (day): whether `Room<World>`, `Decoder` and `ClientEndpoint` are
   `Send` (e8 compiles or not).

---

## 6. Return list (your report, in this order)

1. `<base>` and your final diff's file list (`git status --porcelain` and `git diff --stat <base>`), each file inside
   section 4's lane.
2. Each Build item of section 2.1-2.3: built (file:line) or not (why).
3. The tests e1-e12 (e5 as e5a and e5b) and any you added: name, file, what each shows; the counts in both builds as
   deltas from `<base>`'s, on the measure of section 4 rule 4, with the `--list` lines that name the new tests.
4. Each bite of 2.6: its id, its site (file:line), the test that fails by name in the bites build (the output line),
   and that test passing in the clean build; the parallel-instances test (e11).
5. -18 end to end (2.5): the clean build's first fresh delta after the gap (its time on the injected clock); the
   bite's none within 500 ms of the gap's end, on the injected clock; B's age at the first tick after the gap (e5a);
   whether the bite's baseline was one the client still held (if so: not counted, to the advisor).
6. Carries 21-27 and 39, by number: met (file:line, test) or not met (why).
7. Every pub item U4e adds, by crate, against §4.1 and plan.md §4.7 (2.8); any beyond them named as such.
8. What stopped and goes to the main session: seam changes (carry 26's route), tick counts or rate-derived constants
   (carry 39's route: name, definition, values at 30 and 60 Hz, site), manifest needs.
9. The end checks: check.mjs's two runs (each step's line), the clippy host and wasm32 lines in both builds, fmt;
   `CARGO_TARGET_DIR` checked after the first build; each `df` reading.
10. How each of section 8's rulings was applied.
11. What you did not verify, and what is left.
12. Who built it (the advisor's U4S-BUILDER terms, advisor :371).

---

## 7. Recheck after wave 1

Every place this brief names an item U4a, U4b or U4d builds (plus four items wave 2 or U4j can change), for the day
session to check against the merged code at `<base>` before sending the brief to the advisor. "As named" means the
name, the shape and the behaviour this brief assumes.

| Row | Item (its name in the plan) | Built by | Its use in U4e (brief section) | Check at `<base>` |
|---|---|---|---|---|
| R1 | `Limits::REDUNDANCY_CAP` (plan :516; §4.3 :254) | U4a | `InputBuffer::CMD_CATCHUP` = it, const assert (2.1; carry 27) | exists as a const, value 8 |
| R2 | `Limits`' size limits (plan :516; catalogue :87-101) | U4a | the pump's socket checks; the client's 1,150 B refusal (2.2, 2.3) | the values equal the catalogue's |
| R3 | `Quant` (plan :517; §4.4 :267-269) | U4a | the encoder and the client's decode (2.1, 2.3) | its API; -57's bites inside it (a4 :529-531) |
| R4 | `Wrap`, one compare per width, per counter name, injectable start (plan :517, :532; §4.4 :275-276) | U4a | the seq, press, ack, event-number and tick rebuilds (2.1, 2.3, 2.6) | the counter names it takes; its bite (a plain `<` for one named counter) and which counter |
| R5 | the `Input` codec with delta repeats (plan :517; §4.4 :270-272) | U4a | the pump decodes; `InputBuffer` (2.1, 2.2) | the field table; 8 commands within 166 B |
| R6 | the Snapshot container, its header (32-bit tick, 16-bit offsets, group index and count) and its field-group layout table (plan :517-518; §4.4 :273-274) | U4a | the encoder, the client's complete-tick rule, e1 (2.1, 2.3, 2.4) | whether the layout table lives in npt-proto; if so the encoder uses it and builds none of its own (one home: arch F2), and e1 reads it |
| R7 | the `EntityRecord` ↔ wire mapping (plan :518; A18) | U4a | the encoder and the client (2.1, 2.3) | its API; its round-trip test |
| R8 | `client_hash` (plan :519, :537; §4.6) | U4a | the encoder's mirror and the client's hash (2.1, 2.3, e7) | its inputs (tick, entity set, own state, events as (32-bit number, kind)); never 0 |
| R9 | `encode`, `decode` and the errors `TrailingBytes`, `WrongChannel` (plan :519, :533-536; §4.4 :277-280) | U4a | `Decoder::pump` counts them (2.2) | the error names and how the channel reaches `decode` |
| R10 | the bites no-trailing and no-channel (a6, a7: plan :533-536) | U4a | shown through the pump (2.6) | their ids and how a caller selects them per instance |
| R11 | PM-0001-17/tick16 and PM-0001-58/tick16 in proto's header writer (a5: plan :532; §4.14 :443) | U4a | the client half (2.6; carry 24) | which crate's sources hold the id literals: `git grep -n 'tick16"' <base> -- spikes/netcode-proof/crates` (U4E-Q4) |
| R12 | the messages `Join`, `JoinAccepted`, `JoinRejected` (its reason codes), `Resync`, `FullState`, `Ping`, `Pong`, `Kick` (its reasons), `ClientSettings` (plan :516; catalogue :87-101) | U4a | the roster, the pump, the client (2.1-2.3, e9) | the five JoinRejected reasons; Kick's overflow and malformed reasons; Pong's hold-time field |
| R13 | check.mjs's sim/proto clippy.toml comparison (plan :522, a9 :538) | U4a | room/clippy.toml gains the hash-map bans (2.1) | which files it compares; that a room or client clippy.toml change does not fail it |
| R14 | the catalogue's §6 item 3 note and proto/tests/catalogue.rs items 1, 2, 3, 6 (plan :519-521) | U4a | e1 sits beside them in U4e's crates (2.4) | how catalogue.rs reads the doc, so e1 reads it the same way |
| R15 | `Schedule`, `Due`, `due_ticks` in server/src/schedule.rs and `mod schedule;` in server/src/lib.rs (plan :543; advisor :369-370) | U4b | U4e edits server/src/lib.rs around that line (2.2) | the line's place; no U4e name collides |
| R16 | server/clippy.toml with the std and tokio clock bans (plan :543-544) | U4b | the pump reads only the injected now (2.2) | the bans' list |
| R17 | `World` implementing `SimCore` (plan :586) | U4d | `Room<World>` (e7, e8; 2.5) | its constructor; `Send` |
| R18 | `Scene` (SS, ES, CO, TC: plan :586; §4.13) | U4d | e7's 1,000 ticks at P0 and 2.5's run need a scene | which scene fits a 1,000-tick unit test's time |
| R19 | `InputReports::non_finite` removed (NF-D4; Lanes :827: "drop `InputReports::non_finite` (NF-D4)"; advisor :361; seam.rs:260 at df8975b) | U4d | the room logs `reports()` (2.1) | the field is gone; the room logs every remaining counter |
| R20 | `EntityKind::layout()` completed within `EntityRecord::FIELDS` (carry 12; Lanes :826-827) | U4d | the layout table and e1 (2.1, 2.4) | every kind's fields; FIELDS still 12 |
| R21 | `TickInput`'s new fields (spawn, teleport, attributes: Lanes :826; D-4 :599-601) | U4d | the room fills them each tick (2.1) | their names and types (integers only) |
| R22 | `HitVolumes` filled with the volumes (seam.rs:196-204; plan :587) | U4d | the room copies them into `HitHistory` (2.1) | `Clone` cost per tick (carry 26) |
| R23 | carry 17's button constants (Lanes :826) | U4d | the press-edge compare (2.1) | their names |
| R24 | the seam freeze held: `Command`, `Rewind`, `CoverVariant`, `EntityKind`, `EntityRecord`, the five grid fields unchanged (Lanes :824-831) | U4d | the whole brief (2.7) | read `git diff df8975b <base> -- spikes/netcode-proof/crates/sim/src/seam.rs spikes/netcode-proof/crates/sim/src/grid.rs` line by line: every added line is a new item, a new `TickInput` field or `layout()`'s body (Lanes :826-827); no line inside `Command`, `Rewind`, `CoverVariant`, `EntityKind`, `EntityRecord` or the five grid fields is added, removed or changed ("keeps unchanged every existing field, variant and width": Lanes :824-826); FIELDS still 12 |
| R25 | check.mjs's steps (10 at df8975b: :1474-1507) | U4a, U4d2 | section 5's "every step passing" | the step count at `<base>`, and again at U4e's merge if U4d2 merged first |
| R26 | sim's bite ids (`OWN_COPY_SNAP_BITES`, §4.1 :187) and proto's (a4, a5, a6, a7) | U4d, U4a | the one-crate id rule (2.6; carry 24) | no U4e id repeats one |
| R27 | marks.json's bite ids for -17, -18, -58 and catalogue item 5's owner-to-all | marks.json at `<base>` (U4j edits it: plan :632) | the ids U4e plants (2.6) | each id U4e plants is marks.json's spelling |
| R28 | npt-log's `TickRecord.hs`, `snap_incomplete`, `burst_eviction` (§4.1 :198; SERIES rows: plan :639-640) | U4j | U4e exposes the client's hash and the two series; U4r's per-process logs write them (2.1; was U4E-Q7) | their names, as U4e's exposed values must match them |

---

## 8. Questions for the advisor

Only where the plan or a ruling leaves a choice open. Each is self-contained. Eleven codes: nine open (Q1 only as a
confirm), two answered (Q7, Q9), kept so the codes stay stable.

**U4E-Q1 (two -58 bites with no planting unit: a confirm).** Confirm that U4e plants PM-0001-58/lt-cmd-seq and
PM-0001-58/lt-press, as the plan's own rule gives.
- Why it matters: their sites are the room's sequence-forward and press-edge compares (§4.14 :452-453), which U4e
  builds (plan :691: "the room-side checks -58's counters pass through"); §4.14 :438 plants each bite "where its code
  lives", shown by a unit test in that sub-unit; U4r judges "-58 ... its six bites" (plan :737; §4.14's -58 rows,
  :443 and :452-456, are six). Only U4e's Bites line (:711-712) omits them, which this brief reads as a slip.
- Options: (a) confirm: U4e plants and unit-shows both (§4.14's unit proofs: post-wrap commands refused; an edge lost);
  (b) they wait for a later unit and -58 is judged at U4r without them.
- Recommendation: (a): the plan's own rule (§4.14 :438), and no other unit before U4r builds those compares.

**U4E-Q2 (lt-tick-offset's proof before prediction exists).** How does U4e unit-show PM-0001-58/lt-tick-offset, and
when is it judged?
- Why it matters: §4.14 :454 puts the site at "the client's 'last command run' compare, where its replay starts", its
  proof at "replay starts at the wrong command", and its failing check at "0 untraced corrections"; the replay is
  U4g's `Predictor` (plan :786), and the failing check needs the Predictor too, so U4r (:737) cannot judge this bite
  before U4g whichever option is taken. U4e builds only a minimal `ClientEndpoint` (:697).
- Options: (a) U4e builds the compare (the header's 16-bit "last command run" offset rebuilt to the command it names)
  and shows the bite as "the rebuilt last command run names the wrong command"; U4g's Predictor starts its replay from
  it and re-shows the bite as §4.14 words it; (b) the bite moves to U4g, and U4e's list loses it; (c) as (a), and
  lt-tick-offset's judged run waits for U4g: U4r's -58 judges five bites, and the sixth is judged after U4g.
- Recommendation: (c): U4e owns the header's decode, so the rebuild and its unit proof belong here; the bite's failing
  check (:454) needs the Predictor, so its judged run cannot come before U4g.

**U4E-Q3 (bite id lists in npt-room and npt-server).** May npt-room and npt-server each gain a bites-only `BITES`
list, as npt-client has (client/src/lib.rs:11-12)?
- Why it matters: U4r's `--bite` dispatch reaches each instance (plan :719), and a crate's ids must be findable; but
  §4.1 (:178-201) and plan.md §4.7 (:595-599) list no such item for either crate, and A19 holds the surfaces narrow.
- Options: (a) yes, bites-only, named in the report as beyond §4.1 under this ruling; (b) no list: `Room::with_bite`
  and the pump's bite config take the id, and U4r dispatches by id string alone.
- Recommendation: (a), the npt-client precedent (carry 30's arch F1), bites builds only.

**U4E-Q4 (tick16's id home).** Where do PM-0001-17/tick16 and PM-0001-58/tick16 live?
- Why it matters: U4a plants tick16 in npt-proto's header writer (plan :532), carry 24 puts "-17/tick16's client half"
  in `npt_client::BITES`, and check.mjs refuses an id literal in two crates' sources (check.mjs:712-747).
- Options: (a) the ids stay in npt-proto; the client's rebuild takes the bite through a proto value it is passed (as
  the grid's defect reaches the client through `grid_with_bite`), so the literal lives once; carry 24's "-17/tick16's
  client half" in `npt_client::BITES` is then not met as worded, and U4e's report says so under this ruling; (b) the
  ids move to npt-client and proto's half is selected through it; (c) each half gets its own id: the only option that
  meets carry 24 as worded, at the cost of an id marks.json may not spell (recheck row R27).
- Recommendation: decide after recheck row R11 shows where U4a put them; (a) if they are in npt-proto, since it changes
  no merged item, with carry 24's tick16 part reported "not met, by U4E-Q4's ruling"; (c) if the advisor holds carry
  24's words binding and marks.json can carry the new id.

**U4E-Q5 (`rewind_at` before U4h).** What does `rewind_at` do at U4e with a claimed view time whose tick the ring
does not hold?
- Why it matters: the tolerance and the cap land in U4h (plan :691, :799-800), but a correct room never sends a Rewind
  the ring lacks (the debug assert, §4.2 :219-221), and U4r's runs use this room (the advisor's O1, :343-347).
- Options: (a) no Rewind for that command, counted by the room (the sim treats a refused Rewind as none:
  seam.rs:62-65), until U4h replaces it; (b) the nearest held tick, which is the cap's job, early.
- Recommendation: (a), with the counter logged, so the U4r pre-check's setup reads it.

**U4E-Q6 (the event queue's cap).** Does U4e set the EventQueue's cap value?
- Why it matters: §4.7 :325-326 sets it from "the stress scene's measured rate of relevant gameplay events x (B1's 1 s
  + RTT + a margin), recorded", with about 350 hit events a second (net F121); a measurement needs SS runs, and the
  plan does not say which unit takes them.
- Options: (a) U4e builds the cap as data with a provisional value from net F121's arithmetic, recorded as such, and
  U4f measures and records the value before -22's runs (U4f moves -22: plan :779-782); (b) U4e measures it in a unit
  test over SS.
- Recommendation: (a): U4e moves no mark, and the measurement belongs with -22's L1 runs.

**U4E-Q7 (who writes the series): answered, no ruling needed.** The plan gives it: on L1 one npt-lab process writes
one log per process value (§4.12 :397-398), U4r builds the per-process logs (plan :719-720), and npt-log's `hs` is
U4j's (:627). U4e exposes `snap_incomplete`, `burst_eviction` and the client hash as counters and per-tick values; U4r
writes them (2.1; R28). No manifest change follows (npt-room and npt-client do not depend on npt-log).

**U4E-Q8 (client's hash-map ban).** Does U4e add the HashMap and HashSet type bans to crates/client/clippy.toml?
- Why it matters: §4.11 :380 says they join room's and client's clippy.toml; §5 U4e (:691) names only room's;
  client/clippy.toml at df8975b holds the clock ban only (:1-9), and crates/client is U4e's lane.
- Options: (a) yes, in U4e; (b) no, a later unit.
- Recommendation: (a): §4.11's text, U4e's lane, and the client hash depends on iteration order.

**U4E-Q9 (the Pong hold time's limit): answered, no ruling needed.** The F2 ruling gives it: point 2's pattern gives
the 60 Hz pin wherever no constant is needed (advisor :459-460), and a named constant takes route (b) (advisor
:449-452). Applied in 2.2 (the pump takes the rate as an argument; production passes `npt_sim::TICK_HZ`) and e10 (the
pin at 30 and 60 Hz).

**U4E-Q10 (carry 24's -45 and -62 ids, and -62's room site).** Does U4e add the -45 and -62 bite ids, and does it
plant PM-0001-62/server-echo?
- Why it matters: carry 24 says `npt_client::BITES` "grows with U4e's other ids (... -45's, -62's)", but the client
  sites of those ids are `Interp`'s slot count and `CueLog` (§4.14 :450, :457), which U4g builds (plan :789-790); an
  id with no site is a bite that cannot bite. -62/server-echo's site is the room sending the owner its own cosmetic
  shot events (§4.14 :458; seam.rs:219-220: "never to the shooter's own client"), which is U4e's event routing, while
  -62 moves in U4g (plan :794).
- Options: (a) U4e adds only the ids whose sites it builds: it plants -62/server-echo in npt-room with a unit proof
  ("one event sent"), and -45's and -62/cues-from-replay join npt-client in U4g; U4e reports carry 24 as met for its
  part; (b) U4e plants none of -45's or -62's, and U4g plants server-echo in the room's code then; (c) U4e adds the
  client ids now, with no site.
- Recommendation: (a): each bite planted where its code lives (§4.14 :438), by the unit that builds that code.

**U4E-Q11 (the server-side checks in U4e and U4f).** Is U4f's "the server-side checks (size, trailing bytes, wrong
channel, the bucket)" (plan :775) read as their -21 counters and judged runs, with U4e building and unit-testing the
checks themselves?
- Why it matters: U4e's §5 builds them (plan :693-694) and tests them (:704, :712), and U4f's Builds list them too
  (:775); without one reading, two lanes build the same code, or each leaves it to the other, and a reviewer of either
  unit cannot tell what is in scope.
- Options: (a) U4e builds the checks and their unit tests, as its §5 says; U4f adds their -21 counters and judged runs
  on U4e's pump; (b) U4e builds only the decode errors' counting, and U4f builds the checks themselves; (c) U4e builds
  them now and U4f rebuilds them its own way.
- Recommendation: (a): U4e's §5 names them as Builds and Tests, U4e's no-trailing and no-channel bites are shown
  through the pump (:712), and U4f comes after U4r (:773), which comes after U4e.
