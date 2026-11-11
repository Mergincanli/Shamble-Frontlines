# U4r builder brief: the L1 runner and its judged runs (DRAFT, rev 2)

Drafted by test-engineer on the night shift of 2026-11-11 (night item npt-u4r-brief, under the plan at 2dc1f31's
night-2026-11-10.md, "Item 4"). Step 1 wrote rev 1 (commit 3bdfb3b); step 2, netcode-architect's check of the -30
pre-check part, O1, the RTT argument and -18's fresh-delta reading (`netcode-architect-u4r-check.md`), asked for this
revision; rev 2 is step 3, test-engineer's one revision for it, with the change list below. Then the day session
brings it up to date after wave 2 merges (the "Recheck" table, section 9, and the "Needs a run (day)" list, section
11) and sends it to the advisor, who checks each brief before its build (advisor :157; Q21 at :220). Section 10 holds
twelve questions; the brief marks each place that waits on one.

## Changes in rev 2

One revision pass for netcode-architect's check (`netcode-architect-u4r-check.md`: REVISE, 0 Critical, 0 High, 8
Medium, 8 Low). Each finding's cite was rechecked at df8975b before acting on it; all 16 are accepted, two with a
correction to the check's own fix (N2, N6). "check Nk" below is the check's finding k; section 11's run items are
renamed D1-D12 (X1) so the two never collide.

- **N1 (Medium), accepted.** 7.2's RTT rule now times the sample of the server end of that client's L1 link, as the
  pump reads it, never the client end's (each link keeps one estimate per end: l1/src/lib.rs:456-458, :784-785; plan
  :755-757 says "the server's L1 link").
- **N2 (Medium), accepted, with a correction.** U4R-Q10 rewritten: (a') the run-level bite with the only timing that
  could fire it, and (b') a unit test on doctored records (new test r7 in 2.4), recommending (b') at U4r; 8.2 item 3
  reworded ("can fire only when a command runs in the instant JoinAccepted is sent"). Correction (X3): at df8975b a
  P0 ack's sample applies at its packet's own send instant, not at the next instant as the check says
  (l1/src/lib.rs:515 with 0 delay each way at P0, :418-420, and acks applied whenever `at_ns <= now`, :527, at any send
  or poll, :582, :661, :714), so even (a')'s timing may not fire; whether it can is U4c's and U4e's code (R4, R15 and
  new run item D12).
- **N3 (Low), accepted.** 7.2 logs one more field per client, the first command's receipt instant, so 8.4's "the
  re-run needs no new log" holds whichever instant U4h's estimate reads the RTT at; U4R-Q12 names both premises for
  U4h's carry (that comparand, and the Loopback leg's sessions taking the link's `rtt_ns`, not `ws` pings: §4.9
  :341-344); 7.2 asks the leg glue to class them so; R15's check gains it.
- **N4 (Medium), accepted.** U4R-Q12 now names both readings of the advisor's "6 runs" with their cost ((i) one
  recording, three pairs; (ii) -30's three-recording shape, marks.json:6389), recommends (i) with its reason, and adds
  why a third reading (three clean pairs on one recording) is not offered (a deterministic repeat). 7.2's "runs at
  U4r" bullet no longer adopts one reading of the advisor's words; 8.3 points to U4R-Q12.
- **N5 (Medium), accepted.** New recheck row R33 (every reader of the per-client RTT and every replace, clip or
  substitute path in npt-room and npt-server at `<base>`) and run item D10 with the check's two `git grep` lines (the
  second also searching `rewind_at`, so its body is read); 7.2 and 8.1 point to R33.
- **N6 (Medium), accepted, with a correction.** 7.2 gains "Pacing and the tick base (plan silent: state and test
  them)" and 2.4 gains test r6 (a clean short replay gives `late_count` 0 on both legs). Correction: the check names
  "the recording header's start tick", but §4.15 :463's recording header (scene, seed, TICK_HZ, pool sizes, capacity)
  has none: rev 2 takes the recording run's own `tick_start` and asks R30 whether `Recording` carries it. R17's check
  gains how far ahead option B keeps a command.
- **N7 (Medium), accepted.** 7.2's comparer and 8.2 item 3 name the hash: npt-sim's canonical per-tick hash of the
  room's `World` (FNV-1a-64: plan :590), one per tick in each leg's room log, never the mirror hash per (client, tick).
  Extra (X4): rev 1's 2.1 log list had no per-tick World hash in the room log, which the comparer reads (plan :634);
  added, and R30's check asks its name.
- **N8 (Medium), accepted.** 7.2's wallclock bite: the read must change a sim input on that leg on every run, whatever
  R is, never through the RTT, lateness or a rate bucket's refill; a bitten pair that comes out PASS is a missed bite,
  reported, never re-planted without saying so (marks.json:6377: "a wall-clock read on one transport's command path");
  U4R-Q6 and 2.5's row carry it. The check's second example (the accepted-command set) is left out: a command the pump
  drops can come back by redundancy or count as late, which makes the pair VOID.
- **N9 (Medium), accepted.** -18's heal record (2.1, 7.1): the first decode of a `Snapshot` delta encoded at or after
  the return; a `FullState` never counts; each burst's record notes whether a `FullState` reached that client between
  the burst's start and its heal; a bite run healed through one goes to the advisor with condition 6's report. New
  recheck row R34 and run item D11 (whether npt-client sends `Resync` on an undecodable delta; the event queue's cap).
- **N10 (Low), accepted.** 8.2 item 3 reworded to "the sim's inputs (which command runs at each tick, and its Rewind)
  are independent of the link once `late_count` is 0 and no view time is replaced or clipped"; 7.2's result and 7.4
  list per leg the admits, any drop or kick.
- **N11 (Low), accepted.** 7.2: "whose packet L1 acknowledges at once (U4c's immediate acks; nothing the replay client
  sends; Loopback has none)".
- **N12 (Low), accepted.** 7.2's RTT rule: (a) the first-sample instant as the pump reads it (link/src/lib.rs:79-80);
  (b) the run instant is the tick's due instant, VOID only if strictly earlier, equal instants not VOID; (c) the
  Loopback column reads "Some(0) from the start"; (d) 8.2 item 2 (and, for the same reason, 8.3's "Why") give U4c's
  333 ms with df8975b's `None` (l1/src/lib.rs:349-353) until R2 settles it.
- **N13 (Low), accepted, with one addition (X5).** 7.2 and 2.5: the late-command bite holds the command out of every
  `Input`, redundant copies included, until its tick has run on that leg, and then sends it (delayed, never dropped: a
  command never sent is a miss, which the late counter may not count); its records state the delayed command's tick
  and the leg's `late_count` (expected 1). R17's check gains how option B counts a late command.
- **N14 (Low), accepted.** 8.3's cost states both branches: 6 runs (about 1,800 s simulated) with no replacement, 9
  runs (about 2,700 s) otherwise. Extra (X6): rev 1's wall-time range there ("about 6 min (R = 300) to about 1.2 h (R =
  30)" for 2,100 s) was wrong by 7.3's own table (2,100 s is 3.5 min at R = 300 and 35 min at R = 30); recomputed.
- **N15 (Low), accepted.** -18 (2.1, 7.1): marks.json:4307's plain words named and the record following the ruling
  whichever words U4j's split carries, the report saying which; the builder states how the 10 offsets are drawn and
  the bursts spaced; `burst_eviction` per client and burst.
- **N16 (Low), accepted.** 7.2 marks the out-of-ring count as an addition beyond the plan, with its reason.
- **X1 (extra).** Section 11's run items N1-N9 renamed D1-D9 (they collided with the check's ids), every cite updated
  (4 rule 4; 7.3; 8.3; R2, R9-R12, R20, R29; U4R-Q3); new D10-D12. Carry 20's "arch N2" is the carry's own words, kept.
- **X2 (extra, found fixing N6).** Loopback delivers in the instant of the send (link/src/lib.rs:154-163, :187), L1
  only after it (l1/src/lib.rs:30-31, :727-728), so the two legs can admit a client at different instants: 7.2 asks
  the builder to state whether an admit changes the World's hashed state and logs each client's admit tick per leg.
- **X7 (extra).** The new written names (the RTT rule's three per-client instants, each client's admit tick, -18's
  `FullState` flag) join R26's list and U4R-Q2 (i); 6's return list items 2 and 3 name 7.2's statements and r6, r7;
  7.4's pre-check bullet lists the new records; section 9's preamble counts R33 and R34; section 2's opening now says
  that this brief adds nothing it does not mark as its own (2.4 r6 and r7, 7.2's added records, 8.4's switch).

**How to read the citations.** Every file:line is at df8975b. `plan` is
.checkpoints/netcode-proof-test/u4-build-plan.md; `advisor` is .checkpoints/netcode-proof-test/advisor-u4.md;
`carries` is .checkpoints/netcode-proof-test/u4s-carries.md; `plan.md` is .checkpoints/netcode-proof-test/plan.md;
`catalogue` is docs/design/netcode-proof-messages.md; `marks.json` is spikes/netcode-proof/marks/marks.json (U4j edits
it before U4r: plan :632); `c-u2` is spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs; a crate path (`lab/src/main.rs`)
is under spikes/netcode-proof/crates/. "Needs a run (day)" marks what only a build or a run can settle, with its
command; section 11 lists them all (D1-D12). "check Nk" is finding k of `netcode-architect-u4r-check.md`.

---

## 1. Start

1. **Base:** `<base>` (the day session fills it in: the spike branch's head after U4c, U4d2, U4e and U4j have merged,
   since U4r depends on all four: plan :717; Lanes, plan :837). Record it in your notes before anything else, with
   `git log -1 --format=%H` in your worktree.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree (`isolation: worktree`),
   under §7.1's shared-target rule (plan :866-877; section 4 below). U4r runs alone after wave 2; U4f, U4g and U4h
   follow it one at a time (Lanes, plan :837-838). The judged runs are not yours: the main session runs them after the
   merge, in the main folder (plan :733; §7.2 :883-884; section 7).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4r-build.md` in your worktree (the day session confirms the folder
   against wave 1's and wave 2's builders). Its first line names this task; then your plan as blocks (one block: one
   type with its tests, one test, one bite and its proof, one run script); each finished block written at once, a
   notes line per finding, and one line after each block (done, what next). On a restart, continue after the last
   block marked done.
4. **Read before you build** (and nothing you do not need):
   - plan §5 U4r (:717-772) and the sections it builds to: §2's rows C2, C4, -18 split and "Sizes for fixes.rs"
     (:145-150); §3's U4-SEAM and U4-SERVER (:162-170); §4.1 (:178-201; npt-bots :196, npt-lab :197); §4.5 (:282-294);
     §4.6 (:296-318; -57's pairing :317-318); §4.9 (:333-347); §4.11-§4.15 (:379-487); Lanes (:814-845); §6
     (:846-863); §7 (:864-887);
   - plan.md §4.2 (:421-454: the dependency rules; "Nothing depends on npt-lab", :432) and §4.7's npt-lab entry with
     RQ8's order (:616-627), its binaries (:610-611) and bites (:629-630);
   - advisor: the U4 verdict's conditions 2, 3, 5 and 9 (:90-111, :119-124, :140-142), 10 (:143-149) and 12
     (:154-158); O1 (:343-347); the F2 ruling (:436-465); the advisor's answers to section 10's questions (the day
     session adds where they are recorded);
   - carries: entry 40 (:176-182) and entries 6, 14 and 20, all quoted in section 3;
   - marks.json as merged at `<base>`: -17 (:3845-3980), -57 (:3982-4079), -58 (:4081-4234), -18 (:4236-4308), -30
     (:6325-6390), PM-0001-2 (:152 on) and -21's parts (:4630-4705);
   - catalogue §3 Field groups (:125-158) and §6 at U4, item 5 (:249-252);
   - advisor-u3q.md:180-186 (-18's fresh-delta reading); advisor-u2.md:72-83 (C1, C2) and :97-103 (records; C4);
   - c-u2 (the run-script model, 114 lines);
   - the code as merged at `<base>`: crates/lab (U2's `probe`, U4d2's `hash` and `HashCompare`), crates/bots (at
     df8975b bots/src/lib.rs:1-3 and both binaries are stubs), crates/l1 (U4c), crates/room, crates/server and
     crates/client (U4e), crates/log and crates/judge (U4j), crates/sim (U4d), crates/proto (U4a);
   - .claude/rules/netcode.md: binding on proto, room, client and server code (plan §7.1 item 8, :876-877); U4r edits
     none of those crates, but its runner, replay clients and bots feed them, and its determinism lines (seeded RNG, no
     clock in shared code, deterministic iteration order) are what the twin test checks.

---

## 2. What to build

U4r builds plan §5 U4r (:717-772) exactly: its Builds, its Tests, and the run scripts for the judged runs (section 7),
which the main session runs after the merge. U4r moves the marks -17, -57, -58 and -18 by those runs (plan :733-739),
and records -30's pre-check without moving -30 (:740-764). Out of scope, by the plan: `InputBuffer` option A, `ClockSync`
and the cheats' `Behaviour`s (the 11 forged kinds, the clamp points, the 1.5x clock, the lag switch, the never-acker:
U4f, :774-776); `Predictor`, `Interp`, `CueLog` (U4g, :786-790); the view-time tolerance, the cap, the view-time
estimate with the pump's RTT, the TC shooter's `Behaviour`, the tolerance-edge stamper and the 150 ms pong delayer
(U4h, :799-802); npt-bots' npt-net glue and npt-load (U6, :852-853); -30's marked C part (U6, :850-851); fixes.rs's
run-structured bound (its own small unit after U4r, :859-860: U4r only logs the sizes, §2 :150). Where this brief
finds the plan silent it says so; it adds nothing it does not mark as its own (2.4 r6 and r7, 7.2's added records,
8.4's switch), each there for the advisor to keep or drop.

### 2.1 npt-lab (crates/lab/src/)
Each item from plan :718-724, built to the section named.
- **`SimClock`** (integer ns; plan.md:616).
- **`L1Scenario`, in RQ8's order** (plan.md:617-623), at each simulated instant: (1) packets due, by arrival time and
  then fate-log index, never delivered in the instant they were sent, P0 included; (2) the server's I/O step (U4e's
  `Decoder::pump`); (3) the server tick if due (tick k due at base + ⌊k × 10⁹ / TICK_HZ⌋ ns, through U4b's `Schedule`:
  carry 6); (4) the client frames that are due (60 Hz, seeded phase); (5) each client whose `next_due_ns` has come
  (U4e's `ClientEndpoint`), in slot order. Each client's host clock is offset + rate × SimClock, both seeded
  (plan.md:624; the 1.5 rate is -21's cheat, U4f's). The link is U4c's `L1Net`, event-driven through `next_event_ns`
  (plan :553-554); L1's RTT is its RFC 9002-style estimate, never the simulator's true value (plan.md:625). npt-lab names
  `npt_sim::World` once (U4-SEAM, plan :163). The plan gives `L1Scenario` no test of the order itself (plan silent,
  not added): say in your report how the order is shown.
- **The header params** (plan :718-719): `scene`, `clients`, `profile`, `seed`, `cmd_timing`, `tick_start`,
  `ticks_below_wrap`, `suite`, `stand_ins`. `cmd_timing` is `B` in every U4r run (option A is U4f's, :774);
  `stand_ins` records S1's 1 ms burn and 2.80 ms top-up as off on L1 (§4.13 :432-433). Each header also records the
  bite id, the enabled features and the binary's SHA-256 (plan.md:629-630), and the params the judged runs read:
  `burst_starts_ns` (as U2's probe writes it: lab/src/main.rs:132-135), the input-buffer depth under marks.json -30's
  name `buffer_depth_ticks_count` (marks.json:6368-6372; plan :743 writes `buffer_depth_ticks`: recheck row R27), and
  `twin` for the twin test's runs. The header's `tick_hz` comes from `npt_sim::TICK_HZ` by path, not from a literal as
  U2's probe header has it (lab/src/main.rs:151; carry 6's pattern, the F2 ruling's point 2, advisor :445-449).
- **The `--bite` dispatch to each instance** (plan :719; §4.11 :386-388): one `--bite <id>`, handed to the one instance
  whose crate's list holds it (`World::with_bite`, `Room::with_bite`, the pump's bite config, `L1Net::with_bite` at
  l1/src/lib.rs:858, `Quant`'s own bite choice (carry 2), `npt_client::grid_with_bite` (carry 24)), never a static. An
  id no instance knows is a usage error (exit 2, as U2's probe: lab/src/main.rs:10-11). npt-lab's sources never spell
  another crate's id (section 4). The measured build takes no bite (plan.md:629-630), and U4j's C4 makes a measured run
  whose build lists `bites`, or a bite run whose build lacks it, INVALID (plan §2 :147).
- **The per-process logs with `admit` events and footers** (§4.12 :397-402; plan :719-720): on L1 one npt-lab process
  writes one log per process value the checks select on (`room`, `client`, `lab`), each with its own header; one
  `admit` event per client; a footer per log; `tick.v.late_count` wherever `late-zero` applies; the room's per-tick
  state hash, one per tick in the room log: npt-sim's canonical hash of the room's `World` (U4d's FNV-1a-64, plan
  :590), which r1 and 7.2's comparer read (its name: recheck row R30), never to be confused with the server's mirror
  per (client, tick) below; each client's hash per tick in npt-log's `hs` (U4j, plan :627; `null` for a client with no
  hash at a tick, :634-635); U4e's exposed values (`burst_eviction`, `snap_incomplete`, the client hash and the
  server's mirror: §4.5 :289-290, §4.6 :297-312; U4e's brief reads them as U4r's to write, its U4E-Q7). Every tick and
  second record carries all its counters, 0 included: an empty tick or second series is a gap, and a gap makes the run
  INVALID (judge eval.rs:310-323, :1048-1051).
- **`scenario`** (with `--record`) and **`replay`** (§4.1 :197; plan :720-722): `replay` is the x86-64 re-decode (R-8,
  the advisor's condition 9, :140-142): it re-decodes a run's logged client bytes natively and writes the clients'
  hashes. The clients' received bytes (datagrams and stream bytes) are logged under .gate/ (plan :729); never
  committed (plan §7.2 :883-884). U4d2's `hash` subcommand and `HashCompare` stay as merged (section 2.6).
- **The four runner recordings** (§4.15 :478-484; plan :722-723; the advisor's condition 3): `scenario --record` at SS
  P2 seed 1, ES P3 seed 2, TC B1 seed 3, and SS P0 seed 4 (the -30 pre-check's source only). Each written through
  npt-sim's `Recording` (§4.1 :185), never a second writer; the header as §4.15 :463-466 gives it, with HitHistory's
  span in ms beside TICK_HZ (nit b, advisor :162; carry 14). Where they live (regenerated under .gate/ from a committed
  recordings manifest with each file's SHA-256: §4.15 :485-487) and the determinism step's route (Evidence, :769-770):
  U4R-Q2. TC B1 at U4r: U4R-Q9.
- **-57's pairing** (§4.6 :317-318): npt-lab pairs (client, tick, entity) only for the fields that tick's snapshot
  carried and the client decoded, against the server's pre-grid value at that tick, and writes `tick.s.pos_err_um`,
  `tick.s.yaw_err_microdeg` and `tick.s.pitch_err_microdeg` (marks.json:4029-4051; notes :4078).
- **-18's heal record, by the fresh-delta reading** (plan :738; advisor-u3q.md:184-185: "a delta counts once it was
  encoded at or after the link's return"): per client and burst, the SimClock ns from the link's return to the first
  decode of a `Snapshot` delta (against a baseline the client holds, or the per-type defaults) encoded at or after
  that return. A `FullState` (the join's or a resync's) never counts: it is not a delta (marks.json:4269: "decodes
  deltas again"; catalogue :150: a full state is coded from the per-type defaults, with no baseline). Per client and
  burst, the record also notes whether a `FullState` reached that client between the burst's start and its heal (the
  room forces one on an event-queue overflow, and "a cap below about 400 would resync all 48 clients on every burst":
  §4.7 :321-326; whether the client asks for one itself: recheck row R34); and `burst_eviction` per client and burst
  (§4.5 :289-290: "the last acknowledged baseline" is each client's). marks.json:4307's plain words ("when it first
  decodes a delta after the link returns") are the text the ruling tightens inside the row being rewritten
  (advisor-u3q.md:184): the record follows the ruling whichever words U4j's split carries, and your report says which.
  The event and series names are U4j's split (plan :632; §2 :149): recheck row R21.
- **-58's wrap settings** (§4.14 :443, :452-456; marks.json:4170-4185, :4233): the server tick starts
  `ticks_below_wrap` = 30 below a multiple of 65,536; every counter starts a few steps below its wrap and wraps within
  the first 2 s, through each counter's injectable start (§4.4 :276): the command sequence 2 below its wrap, presses
  scripted through the wrap, "last command run" offsets crossing it, event numbers 3 below it, acks crossing it; one
  `counter_wrap` event per counter that wraps (the check wants at least 6: marks.json:4178-4185).
- **The -30 pre-check's replay clients** (their pacing, the tick base, and the per-client records the RTT rule reads)
  and the late-command bite: section 7.2.
- **The sizes for fixes.rs** (§2 :150; plan :765-767): on L1 B1 runs of the stress scene, each client's per-tick
  snapshot payload and datagram count, each input's payload, and the Join, JoinAccepted and FullState sizes, as series
  with SERIES.md rows (whose file: U4R-Q2); state in your report the WebSocket framing they assume, as plan :766-767
  gives it (a 1 B channel tag, catalogue :73; a 2 B header under 126 B and 4 B up to 65,535 B, RFC 6455 §5.2; +4 B for
  the client's mask; TLS 1.3's +22 B, RFC 8446 §5.2).
- **Tokens:** the lab passes each slot's token in to U4e's `Roster` (plan :687: minting is U6's), seeded, never logged
  in clear.

### 2.2 npt-bots' library (crates/bots/src/, not its binaries)
- **`Behaviour`** (§4.1 :196), clock-injected and transport-free (U4-SERVER, plan :169-170), with the two kinds U4r
  builds (plan :723): the brawl (the stress scene's 48 soldiers fighting in 3 × 16, holding fire so the scene's shot
  rate can meet PM-0001-2's band: §4.13 :415-431) and the open-loop scripts (the contract test's suites, U4R-Q4, and
  -58's scripted presses). The TC shooter is U4h's (:801); the cheats and the never-acker are U4f's (:776).
- The button bits and press-counter indices come from npt-sim's constants (carry 17: "the one home the sim, the client
  and the bots read"). At df8975b npt-bots depends on neither npt-sim nor anything npt-lab can call it through
  (bots/Cargo.toml:20-25; lab/Cargo.toml:23-34): U4R-Q1.
- crates/bots/src/bin/ (npt-bots, npt-load) stay as merged: their U4 halves are not in the plan.

### 2.3 Run scripts (ops/runs/c-u4r-*.mjs)
- As c-u2 does (plan :723-724): `--date YYYY-MM-DD` (c-u2:40-42); refuse uncommitted changes to tracked files
  (c-u2:43-45); raw logs under .gate/netcode-proof-test/runs/<set>-<short>/ and results under results/<set>/<short>/,
  never overwritten (c-u2:47-50); both builds of npt-lab built `--release --locked` and copied out before the next build
  replaces them, then npt-judge (c-u2:52-61); `npt-judge --marks marks/marks.json --logs <logs> --only <ids> --out
  verdict.json` (c-u2:86-92); runs.txt with the commit, each binary's SHA-256, the raw-log path, the judge's line and
  one line per run (c-u2:93-104).
- Each script also times each npt-lab process it starts (wall time around the call) and writes it per run, for the
  ticks-per-second figure (2.4 r5).
- The sets and their files: section 7. One script per set or one for all is your choice; say which.
- You cannot run them as written: they refuse uncommitted changes (c-u2:43-45), and you commit nothing. Run each
  npt-lab command line a script issues by hand on a short setting instead, and list those lines in your report.

### 2.4 Tests (plan :725-732), each named in your report with its file
- **r1, the runner's twin test** (§4.11 :393-394): two processes, same seed and scenario: equal fate-log hashes, equal
  per-tick server and client hashes over the run. Bite: a HashMap-ordered client order (marks F29), bites build only,
  planted in npt-lab's client loop: the two processes' per-tick hashes must differ. Each process logs its order
  digest; equal digests mean "not exercised", never caught or missed (the rule §4.14 :460 gives the sim's hash-map
  bite). Say how the second process is started (U4d's D-2 re-runs itself through `std::env::current_exe()`: plan :594).
- **r2, the contract test** (§4.12 :399-402; plan :725-728; the advisor's condition 5, :119-122): a short scenario per
  suite shows (a) every check of every U4 L1 part resolves, no gap (the scope at U4r: U4R-Q4); (b) every written name
  has its SERIES.md row; (c) the validity inputs exist: one `admit` event per client, a footer per log, and
  `tick.v.late_count` wherever `late-zero` applies (T18); (d) PM-0001-2's checks resolve on every SS suite (T1) and
  **pass** on the short runs of each precondition suite: -17's four profiles (P0, P2, P3, B1), -18's B1, -20's P3, and
  each of -21's eight suites with its open-loop scripts (which of them can run at U4r: U4R-Q4); (e) the shot band per
  suite (§4.13 :430-431). A suite that fails comes to the advisor before its marks count (condition 5, :122): report
  it, never re-run it into a pass. The short runs' length is not in the plan: say which you used. Negative cases for
  (b) and (c): U4R-Q11.
- **r3, `replay`'s U4 test** (plan :728-730; the advisor's condition 9): one short L1 run (SS P2) logs each client's
  received bytes under .gate/; `replay`, re-decoding them, gives each client's logged hash at every (client, tick)
  that has one, with the number compared stated and above 0 (a run of 0 compared fails, as the decoded-hash figure does:
  §4.6 :315). Bite: the re-decoder's grid config floored (`grid::Config` passed by value: carry 20) must give at least
  one mismatch, and the test fails by name in the bites build. The plan names no id for this bite: either the
  re-decoder's client instance takes PM-0001-17/client-rounding through `npt_client::grid_with_bite` (no new id), or
  npt-lab gets its own id (check.mjs's one-crate rule, check.mjs:712-747); say which.
- **r4, the class audit** (catalogue §6 at U4, item 5, :249-252; plan :731): SS, 48 clients in 3 × 16, P2, 3 seeds:
  every group the encoder writes into each client's snapshots and full states is one its class lets that client see
  (`owner` only to its own slot, `team` only to that team, `spatial` only inside that client's interest set or event
  test, `public` to anyone), each client's join FullState included. Bite: owner-to-all (U4e's site in the encoder's
  class filter) must fail it, by name. The `team` and `public` sections are empty in the spike (catalogue :144-145,
  :153-154), so their two rules cannot fire: report them as not exercised, never as passed. The run length is not in
  the plan: say which you used.
- **r5, the runner's measured ticks per second, and the total wall time of the run set stated before it starts**
  (plan :731-732): section 7.3.
- **r6, the pre-check's pacing test** (this brief's, for 7.2's pacing rule, which the plan leaves silent: check N6): a
  short clean replay of the SS P0 recording (its first ticks, enough for every replay client's first commands; say how
  many) over both legs gives `tick.v.late_count` 0 on both legs, every command run at its stamped tick, and equal
  per-tick World hashes. Its negative case is the late-command bite (2.5), which must give exactly that leg's one late
  command.
- **r7, the RTT rule's unit test** (only if U4R-Q10 rules (b')): the rule's code on doctored per-client records: a
  first-sample instant after the first command's run instant gives VOID; equal instants give not VOID; no first-sample
  instant on the L1 leg before the first command gives VOID; the Loopback leg's "Some(0) from the start" never gives
  VOID; each case failing by name if the rule's code is changed to miss it.
- Where they live: npt-lab's and npt-bots' own tests; nothing in the crates other lanes built (section 4).

### 2.5 Bites U4r plants (unit-shown; sites in U4r's files unless marked)
Each chosen per instance (§4.11 :386-388), shown by a test that fails by name in the bites build (§7.1 item 3,
:869-870) and passes in the clean build. The bites U4r runs judged but other units planted are in section 7.

| Bite | Site | Proof |
|---|---|---|
| the twin test's HashMap-ordered client order (marks F29) | npt-lab's client loop, bites only | r1: the per-tick hashes of the two processes differ; equal order digests mean "not exercised" (§4.14 :460) |
| the re-decoder's floored grid (plan :730) | `ReplayX86`'s decoder (id: 2.4 r3) | r3: at least one (client, tick) mismatch |
| PM-0001-30/late-command (plan :760; marks.json:6380-6383) | the pre-check's replay client: one replayed command held out of every `Input` that would carry it, redundant copies included, until its tick has run on that leg, then sent (delayed, never dropped) | section 7.2: the pair VOID by `late_count` alone (that leg's `late_count` expected 1), never a pass |
| PM-0001-30/wallclock (plan :759) | "a wall-clock read in the pump for one leg's sessions" (npt-server, U4e's crate): U4R-Q6; the read changes a sim input on that leg on every run (7.2) | section 7.2: the pair valid and the comparer naming a tick; a PASS is a missed bite, reported |

- Bite ids are `PM-<ADR>-<row>/<name>` (check.mjs:678); each id lives in one crate's sources (check.mjs:712-747).

### 2.6 The seam rule, and what stops the build
- No existing item changes in what the earlier units merged: npt-sim's seam, `World`, `Scene`, `Recording`,
  `replay::Replayer` and the tick constants (U4s, U4d); npt-proto (U4a); `Schedule`, `Due`, `due_ticks` (U4b); npt-l1
  (U4c); npt-room, npt-server and npt-client (U4e); npt-log, npt-judge and marks.json (U4j); npt-lab's `hash`,
  `HashCompare`, crates/hash-wasm and check.mjs (U4d2).
- A needed change stops that part: write it in your notes and report (the item, the change, why, the site), build the
  rest, and leave the change to the main session, which takes it to the advisor.
- A tick count or a named constant derived from the rate: no local form; the need goes in your report (carry 39's rule,
  routed to U4b and U4e but binding every crate outside npt-sim; the F2 ruling's route (b), advisor :449-458: the main
  session adds it by carry 18's route; no lane holds crates/sim or check.mjs during U4r, Lanes :837). Run settings that
  marks.json fixes as data (`tick_start` = 63,736, `ticks_below_wrap` = 30: marks.json:3941-3951, :4170-4177) are
  header values, never `*_TICKS` constants; durations are in ns or s, turned into ticks through npt-sim's functions.

### 2.7 Public items (A19; plan §7.2 :885-886)
- Allowed: npt-lab's `SimClock`, `L1Scenario`, `ReplayX86` (plan.md:616; `HashCompare` is U4d2's) and its
  subcommands `scenario` (with `--record`) and `replay` beside U4d2's `hash` (§4.1 :197); npt-bots' `Behaviour`
  (§4.1 :196). Everything else `pub(crate)` (plan.md:454); U2's `probe` and `machine` stay modules of the binary
  (lab/src/main.rs:13-14).
- Your report lists every pub item U4r adds (types, functions, methods, fields, constants), by crate, against §4.1 and
  plan.md §4.7; engine-architect checks them (check.mjs does not: plan :886).

---

## 3. The carries, word for word

Quoted from carries :176-182, the one entry the file routes to U4r (the advisor's U4s ruling, O1; carries' rule,
:9-10: "A brief may reword the plain parts, never the rule"). Your report says of it, by number, "met" (with the
file:line and the test or record that shows it) or "not met" (with why).

```
## U4r (the L1 runner and its judged runs)
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1):** "U4r's brief, which
    comes to the advisor, states that at U4r the room has neither the view-time tolerance nor the cap (both land in
    U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30 pre-check's setup condition (0 view-time
    replacements and 0 clipped shots on both legs: the advisor's condition 3) holds by construction at U4r, and its
    RTT argument cannot be tested there; and it either re-runs the pre-check pair after U4h merges (6 runs) or says
    why not (the advisor's lean is the re-run)."
```

Carry 40 is met by this brief's text (section 8) and, for your part, by what section 7.2 asks the runner to log at U4r.
Your report states the setup counts as "not built: 0 by construction at U4r", never as evidence that the setup held.

**Three entries routed to other units whose words reach U4r's code.** Quoted word for word; your report says of each,
by number, whether U4r's code meets it (met, with file:line; or not met, with why). The routing stays as the file has
it; whether U4r reports them is the advisor's to confirm in reviewing this brief.

```
6. **arch F6 / rust 1 (the grep's stated limit):** "The tick grep cannot see a literal passed as an argument. npt-server's
   and npt-lab's callers build `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`; the 60 Hz tests pass
   `npt_sim::loop_catchup_at(60)`, never a literal 15; a test asserts the server's schedule's catch-up cap equals
   `npt_sim::LOOP_CATCHUP`."
14. **rust 3 (the 1 MB half):** "`HitHistory::for_ms` also returns `None` for a span whose ring would exceed 1 MB
    (capacity × one tick's `HitVolumes` bytes: u4-build-plan.md §4.9), tested at the edge, once `HitVolumes` holds its
    volumes. The recording header carries the ring's span in ms beside `TICK_HZ` (nit b's capacity), and the replayer
    rebuilds the ring with `HitHistory::for_ms(span_ms)`."
20. **arch N2:** "`grid::Config` has a bites-only field, so its shape differs by build: never hash, record or compare it
    across builds; pass it by value."
```

Notes for the builder, which change no word: carry 6's npt-lab half is the `Schedule::new` call in `L1Scenario` (the
file routes carry 6 to U4b, which was built before npt-lab had such a caller); carry 14's header half binds the four
runner recordings, whose span is the one the room used in that run (U4h sizes the ring later: §4.9 :335-336; carry
28); carry 20 binds `replay`'s floored-grid bite (2.4 r3) and every recording and header U4r writes.

---

## 4. Build rules

**The shared-target rule** (Mustafa's, 2026-11-09; plan §7.1 :864-877), binding on every cargo call in your worktree
and in each bite's scratch copy:
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` on every cargo call; checked once after the
   first build (the build's artifacts are in that folder, and no `target/` appears in the worktree: item 1, :866-867;
   nit g, advisor :169-171).
2. `CARGO_NET_OFFLINE=true` on every cargo call. (Plan §7.1's text names neither this rule nor rule 5; the earlier
   builders' and reviewers' briefs carry both: u4s-fix-review-rust.md:6-7, u4s-review-rust.md:30, u3d-build.md:7, as
   the U4e brief rev 2 cites them. The day session aligns their wording with wave 1's and wave 2's briefs.) If U4R-Q1
   rules that U4r adds a dependency between workspace crates, no fetch is needed (both crates are in the workspace and
   the lock), so the rule stands.
3. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (item 2, :868).
4. Counts are deltas (item 3, :869-870; Lanes :839-840; §5's preamble :490-491), on check.mjs's own measure: its test
   step runs `cargo test --locked --workspace` and then the same with `--all-features` (check.mjs:1380-1385), and its
   count (cargoCounts, check.mjs:1321) sums the "N passed; M failed" of each run's `test result:` lines. Before the first
   build, record `<base>`'s two counts that way: check.mjs's test-step line at `<base>`, or the sum of the `test result:`
   lines of those two runs. Each run's count matches the base plus the tests you added; a bite run shows its own test
   failing by name; a mismatch means another copy's artifact ran: touch, rebuild, never report that run. `--list` only
   names the new tests (the flag before `--`: `cargo test --locked --workspace --all-features -- --list`); it is not a
   count (it lists ignored tests too and runs nothing). Needs a run (day), at `<base>`: D1 in section 11.
5. `df -h /home/user/Shamble-Frontlines/.gate` before each build, the reading in your notes; below the floor the day
   session sets (`<floor>`; the earlier briefs above stop under 2 GB), stop and report; never clean anything to make
   room. Release builds (2.4 r5) are larger than debug ones: read `df` before each.
6. After the last bite, touch and rebuild before any further claim (item 4, :871).
7. Cargo's own lock serializes your builds with any other copy's: expected (item 5, :872). No copy cleans the shared
   folder (item 7, :875). The judged runs use the main folder's own cache with `CARGO_TARGET_DIR` unset (item 6,
   :873-874): not yours.

**Your files, and only these** (the night plan's U4r lane; the plan's Lanes name U4r's place, :837, not its files):
crates/lab/ (src/ and tests/; U2's `probe` and `machine` and U4d2's `hash` and `HashCompare` kept as merged);
crates/bots/src/ but src/bin/, and crates/bots/tests/; ops/runs/c-u4r-*.mjs.
- **Waiting on a ruling, so not yours until it comes:** crates/lab/Cargo.toml, crates/bots/Cargo.toml and Cargo.lock
  (U4R-Q1); marks/SERIES.md, the recordings manifest, and check.mjs or ops/runs/det-u4.mjs (U4R-Q2);
  crates/lab/clippy.toml and crates/bots/clippy.toml (U4R-Q3); the wallclock bite's site in npt-server (U4R-Q6). A part
  that needs one stops, goes in your report, and the rest is built.
- Never: crates/sim, crates/proto, crates/l1, crates/room, crates/server, crates/client, crates/log, crates/judge,
  crates/hash-wasm; marks/marks.json and marks/README.md (U4j's); docs/; results/ (the main session's runs write it:
  §7.2 :883-884).
- Write and Edit only in your worktree's sources: no shell writes there and no generated source edits. Each bite's
  scratch copy may be made by a scratch script under this section's rules (plan §7.2 :881-882: "the bites planted by a
  scratch script under §7.1"). Commit nothing and push nothing; the main session merges (plan §7.2 :880-882).
- npt-lab's sources never spell another crate's bite id as a string literal: check.mjs reads every exact-id literal
  in crates/<crate>/src/ outside test modules and fails an id found in two crates (check.mjs:712-747). The `--bite`
  dispatch finds each id's owner through that crate's own list at run time; tests and the run scripts may name ids
  (check.mjs:716-718).
- .claude/rules/netcode.md binds proto, room, client and server code (plan :876-877): none of them is yours; your
  runner, replay clients and bots feed them only through their public items.

---

## 5. End checks

Run from spikes/netcode-proof/ in your worktree, under section 4's rules (touch and `df` before each):
1. check.mjs twice, each with every step passing: `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true node check.mjs`.
   At df8975b it has 10 steps (fmt, clippy, clippy-wasm32, doc, test, deps, refs, marks, self, redact:
   check.mjs:1474-1507); U4d2 adds a `wasm` step (plan :656-657). The day session states the step count at `<base>`
   (recheck row R12). Report both runs' last lines ("check: all N steps pass": check.mjs:1532) and the test step's two
   counts.
2. clippy with warnings as errors in both builds: check.mjs's clippy step lints the whole workspace with all targets
   (check.mjs:1478, lintBothBuilds :1352-1357), npt-lab and npt-bots included; its clippy-wasm32 step covers LEAF plus
   npt-tcpinfo (check.mjs:28, :1348), which holds neither npt-lab nor npt-bots, so U4r's crates are linted on the host
   only, as far as the plan's host-and-wasm32 rule (§5's preamble :492) reaches them. State each step's line.
3. fmt: check.mjs's fmt step (`cargo fmt --all --check`, check.mjs:1475).
4. The run scripts: each npt-lab command line they issue, run by hand on a short setting (2.3), with its output line.
5. `node tools/gate.mjs --no-smoke` is the merge step's, in the main folder (§5's preamble :493): not yours.

---

## 6. Return list (your report, in this order)

1. `<base>` and your final diff's file list (`git status --porcelain` and `git diff --stat <base>`), each file inside
   section 4's lane (or inside a lane a ruling opened, named with the ruling).
2. Each Build item of 2.1-2.3: built (file:line) or not (why); and 7.2's three statements (the room's tick base on both
   legs, the replay clients' pacing rule and why it lands each command before its tick, whether an admit changes the
   World's hashed state), each with the file:line that does it.
3. The tests r1-r7 (r7 only if U4R-Q10 rules (b')) and any you added: name, file, what each shows; the counts in both
   builds as deltas from `<base>`'s, on the measure of section 4 rule 4, with the `--list` lines that name the new
   tests.
4. Each bite of 2.5: its id (or "no id: 2.4 r3's choice"), its site (file:line), the test that fails by name in the
   bites build (the output line), and that test passing in the clean build; for r1, both processes' order digests.
5. The contract test (r2): per suite, each check that resolved, each PM-0001-2 check's value against its band, the shot
   band; every suite that failed (to the advisor before its marks count: condition 5); the short runs' length.
6. The class audit (r4): per seed, the groups audited by class, the run length, the bite's failing line; `team` and
   `public` reported as not exercised.
7. `replay` (r3): the (client, tick) pairs compared, 0 mismatches clean, the bite's mismatch count.
8. The ticks per second (r5): per scene and profile, with the method (section 7.3), the exact command used, and the
   set's estimate redone with the measured value.
9. The four recordings: each file's SHA-256, size and `scenario --record` wall time, its span in ms and HitHistory's
   capacity from its header (carry 14); and the determinism route as ruled (U4R-Q2).
10. Carry 40 (met by this brief and section 7.2's logs; the setup counts stated as "not built: 0 by construction at
    U4r") and carries 6, 14 and 20 for U4r's code: met (file:line, test) or not met (why).
11. Every pub item U4r adds, by crate, against §4.1 and plan.md §4.7 (2.7); any beyond them named as such.
12. What stopped and goes to the main session: seam changes, tick counts or rate-derived constants (carry 39's rule:
    name, definition, values at 30 and 60 Hz, site), manifest and other out-of-lane needs (U4R-Q1 to Q3, Q6).
13. The end checks: check.mjs's two runs (each step's line), clippy's two lines, fmt; `CARGO_TARGET_DIR` checked after
    the first build; each `df` reading.
14. Each run script: its file, the sets it runs (section 7), and the npt-lab lines run by hand (2.3).
15. How each of section 10's rulings was applied.
16. What you did not verify, and what is left.
17. Who built it (the advisor's U4S-BUILDER terms, advisor :365-371).

---

## 7. The judged runs and the -30 pre-check (the main session's, after the merge)

Run in the main folder, on release builds, one set at a time, in the background, raw logs under .gate/, verdicts and
runs.txt committed under results/ (plan :733; §7.2 :883-884), by the scripts of 2.3, judged by `npt-judge --only`
(plan :733). You build the scripts and show their npt-lab lines work (2.3); you run none of the sets.

### 7.1 One row per part
3 seeds per part (plan :733): seeds 1, 2 and 3, as c-u2 runs them (c-u2:23) and as U4h's judged runs are numbered
(plan :805-806). Every part's select gains scene SS, clients 48 and `pre:PM-0001-2` where §4.12's table gives them
(plan :405-411; U4j writes them into marks.json: :632). Bite runs: the plan fixes neither their number nor their
profile; the scripts follow c-u2's model, one run per bite at seed 1 (c-u2:80-84), with the part's own settings
(length, scene, clients), so a bite is shown on a run its part would accept; each bite run is in the bites build and
each measured run in the measured build (U4j's C4: plan §2 :147).

| Part (marks.json) | Layer, profile | Scene, clients | Length | Seeds | Bites (planted by) and their run settings | What the part checks |
|---|---|---|---|---|---|---|
| -17 L1 P0 (:3858-3864) | L1, P0 | SS, 48 | 10 min (plan :734) | 1-3 | tick16 at `tick_start` = 63,736, so the bite run crosses the 16-bit wrap 1,800 ticks (60 s at 30 Hz) in (plan :734; marks.json:3966); client-rounding, any -17 profile (§4.14 :444). Both U4e's (tick16 with U4a: plan :532, :711); the clean runs' `tick_start`: U4R-Q7 | `decoded_hash_mismatch_count` == 0 (marks.json:3935-3940); compared count reported (:3958-3961) |
| -17 L1 P2 (:3866-3872) | L1, P2 | SS, 48 | 10 min | 1-3 | as above | as above |
| -17 L1 P3 (:3874-3880) | L1, P3 | SS, 48 | 10 min | 1-3 | as above | as above |
| -17 L1 B1 (:3882-3888) | L1, B1 | SS, 48 | 10 min | 1-3 | as above; its bursts: U4R-Q8 | as above; also logs the sizes for fixes.rs (§2 :150) |
| -57 L1 P0 (:3994-3999) | on -17's P0 runs (plan :736) | SS, 48 | -17's | -17's | pos-4cm, yaw-12bit, pitch-8bit, in npt-proto's `Quant` (U4a: carry 2), each its own bite run | position error max ≤ 10,000 µm, yaw ≤ 10,000 µdeg, pitch ≤ 180,000 µdeg (marks.json:4029-4051), from 2.1's pairing |
| -57 L1 P2 (:4002-4007) | on -17's P2 runs | SS, 48 | -17's | -17's | as above | as above |
| -57 L1 P3 (:4010-4015) | on -17's P3 runs | SS, 48 | -17's | -17's | as above | as above |
| -57 L1 B1 (:4018-4023) | on -17's B1 runs | SS, 48 | -17's | -17's | as above | as above |
| -58 L1 P0, suite `wrap` (:4092-4099) | L1, P0 | SS (§4.14 :455: "SS's hits flow"), 48 (marks.json:4233) | the counters' wraps within the first 2 s, then 10 s (marks.json:4233): about 12 s, at least 300 ticks counted (:4186-4193) | 1-3 | §4.14's six (plan :737; marks.json:4203-4226): tick16 (`ticks_below_wrap` = 30: §4.14 :443), lt-cmd-seq (sequence 2 below its wrap, :452), lt-press (presses through the wrap, :453), lt-tick-offset (offsets crossing it, :454: U4R-Q5), lt-event-seq (event numbers 3 below it, :455), lt-ack (acks crossing it, :456); all U4e's sites (U4e's brief, U4E-Q1 and Q2) | decoded hash 0 mismatches; 0 commands refused, lost or doubled; 0 presses lost or doubled; 0 untraced corrections (U4R-Q5); setup `ticks_below_wrap` == 30; at least 6 counter wraps (marks.json:4115-4193) |
| -58 L1 P3, suite `wrap` (:4101-4112) | L1, P3; validity `reorder` (:4109-4111) | as above | as above | 1-3 | as above | as above; reorder share reported (:4195-4201) |
| -18 L1 B1 (:4248-4255) | L1, B1 | SS, 48 (§4.12 :408) | 10 min with 10 bursts (§4.13 :423-425's 10 bursts in 600 s; marks.json:4307), each burst's start offset 0 to 1 tick (plan :738; read here as from a tick boundary: at 30 Hz every whole second is one), the offsets in the header's `burst_starts_ns`. The plan fixes the offsets' range only: state how the 10 offsets are drawn and how the bursts are spaced in 600 s (for example a seeded draw in [0, 1 tick) ns per burst, and bursts every 60 s from 30 s) | 1-3 | no-fallback (U4e's encoder: §4.14 :445), with `burst_eviction` logged per client and burst (plan :739; §4.5 :289-290); a bite run whose heals came through a `FullState` (2.1) goes to the advisor with condition 6's report, never counted as caught or missed on its own (this brief's proposal); if U4e reported the bite's baseline as one the client still holds, -18's L1 part goes to the advisor before it counts (condition 6, advisor :128-129) | at `<base>`, U4j's split (plan :632); at df8975b, each client heals within 500 ms of the link's return and 48 × 10 heal records exist (marks.json:4268-4281), by the fresh-delta reading (2.1), a `FullState` never counting as a heal |
| -30 pre-check, suite `precheck-30`: not a part, no mark (7.2) | leg 1 `loopback`; leg 2 L1, P0 | SS (the recording), 48 replay clients | the SS P0 recording's length (seed 4; about 5 min if it holds 9,000 ticks: §4.15 :468) | the recording's seed 4 only (one recording; "3 seeds per part" is for parts) | wallclock (U4R-Q6) and late-command (U4r's), each run as a pair | 7.2 |

- **-58's six bites at U4r:** lt-tick-offset's failing check is "0 untraced corrections" (§4.14 :454), which only U4g's
  `Predictor` can feed (plan :786), and -58's part also checks that series (marks.json:4162-4169): U4R-Q5.
- **The order inside the set:** the four recordings first (the pre-check needs SS P0 seed 4), then the pre-check, then
  the judged sets; the ticks-per-second measure (7.3) before any of them.

### 7.2 The -30 pre-check (plan :740-764; the advisor's conditions 2 and 3, :90-111)
A recorded npt-lab test, not a judged id (Q10, advisor :208). Its parts as the plan gives them, and, where the plan is
silent, what you state:
- **Suite and legs:** suite `precheck-30`; one leg over Loopback (layer `loopback`, the value U4j adds to README §1:
  plan :633-635), one over L1 P0 (layer `L1`). Replay clients send the SS P0 recording's commands (§4.15) to the room on
  both legs, re-encoded as `Input` (stamped ticks, view times, sequences and press counters from the recording; the ack
  from what that leg's replay client decodes live), and the room computes its own Rewinds (§4.15 :472-475). Option B
  (`cmd_timing` B), the input buffer deep enough for P0; both legs' depth recorded (marks.json -30's report reads
  `header.params.buffer_depth_ticks_count`: marks.json:6368-6372; condition 3, advisor :111). The plan gives no depth
  value: state yours and why it is deep enough for P0. Both legs run one room configuration (the same depth, the same
  HitHistory span, the same seed, the same tick base), recorded in both headers; only the leg's link and the wallclock
  bite's leg differ (this line is this brief's: the comparison implies it).
- **Pacing and the tick base (plan silent: state them, and show them by test r6).** The recording holds one command
  per soldier per tick and no send instants (§4.15 :463-466); option B runs exactly one command per client per tick,
  at its stamped tick, and drops late ones (ADR 0001:1160-1162); a pair counts only if every command ran at the tick
  stamped on it with the late counter at 0 (ADR 0001:1312-1315). The replay clients have no `ClockSync` and no
  tick-lead controller (both U4f's: plan :774). State:
  (a) the room's tick base on both legs: the recording run's own `tick_start` (its npt-lab header; the recording
  header of §4.15 :463 names no start tick: recheck row R30 asks whether `Recording` carries one), recorded in both
  legs' headers beside the depth;
  (b) the pacing rule: when a replay client sends the command stamped T (for example at the SimClock instant at which
  tick T minus the depth is due, the same instant on both legs, or once its decoded server tick reaches T minus the
  depth plus one), why it reaches the room before tick T runs on both legs at P0 and is still kept by option B's buffer
  (how far ahead the buffer keeps a command: R17), and how the `Input`'s redundancy covers it;
  (c) whether admitting a client changes the World's hashed state (a spawn, for example: spawns are `TickInput`'s,
  §4.15 :465). Loopback delivers in the instant of the send (link/src/lib.rs:154-163, :187), L1 only after it
  (l1/src/lib.rs:30-31, :727-728), so the two legs can admit a client at different instants; if an admit changes the
  hashed state, both legs must admit each client at the same tick, and the records' admit ticks (below) show it.
- **Each replay client sends its first `Input` only after it decodes `JoinAccepted`,** whose packet L1 acknowledges at
  once (U4c's immediate acks; nothing the replay client sends; Loopback has none) (plan :753-755).
- **The comparer:** npt-lab's `HashCompare` (U4d2's; it "takes two per-tick hash lists": plan :652-654) compares the two
  legs' per-tick server state hashes, npt-sim's canonical per-tick hash of the room's `World` (U4d's FNV-1a-64: plan
  :590; the hash D-2 compares), one per tick in each leg's room log (2.1), and names the first tick that differs. Never
  the mirror hash per (client, tick) (§4.6 :300-302): it follows each leg's baselines, acks and priority skips, so a
  correct pair could read FAIL on it. `tick.v.late_count` must sum to 0 on both legs (condition 2, advisor :93-94).
- **The setup condition** (condition 3, advisor :107-108): 0 view-time replacements and 0 clipped shots on both legs,
  each replacement and clip logged per shot; a leg with any, or with `late_count` above 0, makes the pair VOID,
  reported, never a pass or a fail (plan :746-748). **At U4r the room has no code that replaces or clips** (section 8;
  recheck row R33 checks it at `<base>`): write both counts per leg as "not built: 0 by construction" (8.4) and say so
  in runs.txt; never cite them as the setup holding. U4e's out-of-ring count (no Rewind for a view time the ring does
  not hold: U4E-Q5's ruling) is logged per leg and reported. This log is an addition beyond the plan (a log, not a
  rule): with `late_count` 0 both legs run each command at its stamped tick against rings built from equal states, so
  `rewind_at`'s out-of-ring answer cannot differ between them, and the plan does not make it a VOID cause.
- **The RTT rule** (plan :749-758). The runner logs per client, on each leg:
  (1) the first-sample instant: the SimClock instant of the first pump call (RQ8's step 2) at which the server end of
  that client's L1 link holds an RTT sample, as the pump reads it (an end reads an ack once its clock reaches it:
  l1/src/lib.rs:33-34, :526-532; the estimate is "as of the last call that took `now_ns`": link/src/lib.rs:79-80), not
  the ack's own time; never the client end's (each L1 link keeps one estimate per end: l1/src/lib.rs:456-458, :784-785;
  the client end samples from its `Join`'s ack, which always comes before its first command, so the rule would pass
  vacuously); how the lab learns it: recheck row R4;
  (2) the first command's receipt instant: the pump call that decoded the `Input` carrying it (this brief's addition,
  one field: check N3; it lets the re-run after U4h compare whichever instant U4h's estimate reads the RTT at: 8.4);
  (3) the tick of its first command run, and that tick's due instant (base + ⌊k × 10⁹ / TICK_HZ⌋: 2.1).
  A client whose first command's run instant is strictly earlier than its first-sample instant, or that has no
  first-sample instant on the L1 leg when its first command runs, makes that leg VOID. Equal instants are not VOID: at
  one instant the pump (step 2) comes before the tick (step 3). On the Loopback leg `rtt_ns` is always Some(0)
  (link/src/lib.rs:203-205): its column reads "Some(0) from the start", never an empty gap, and the rule binds the L1
  leg. Your leg glue gives the Loopback leg's sessions the link's `rtt_ns`, as L1's (§4.9 :341-344 gives it to `wt`
  and L1 sessions, and the server's own ping samples to `ws`; the plan's argument assumes Loopback's Some(0): :749-750;
  R15); say how.
- **The result:** VOID if either leg breaks the setup condition or the RTT rule; else PASS if the comparer names no
  differing tick, FAIL if it names one. Each written to runs.txt with its reason. Beside it, runs.txt lists per leg the
  clients admitted (48), each client's admit tick, and any client dropped or kicked (Kick(overflow): §4.7 :321-324), so
  a FAIL caused by a client dropped on one leg is read as that.
- **The bites** (plan :759-760; condition 2, advisor :95; marks.json:6377: "a wall-clock read on one transport's
  command path"):
  - **wallclock** (a wall-clock read in the pump for one leg's sessions, bites build only: its site is U4R-Q6). The
    read must change a sim input on that leg on every run, whatever R is (for example one field of each command the
    pump passes on for that leg, set from the wall-clock value), never through the RTT (nothing reads it at U4r: 8.2
    item 2), through lateness (that makes the pair VOID, which shows nothing) or through a rate bucket's refill
    (whether a bucket starves depends on the machine's ticks per second, 7.3). It is shown only on a pair that is not
    VOID, with the comparer naming a first differing tick; a wallclock pair that comes out VOID shows nothing, and one
    that comes out PASS is a missed bite, reported as such, never re-planted into a fail without saying so.
  - **late-command** (one replayed command delayed past its tick, in the replay client: U4r's site). The replay client
    holds that command out of every `Input` that would carry it, redundant copies included, until its tick has run on
    that leg, then sends it: delayed, never dropped (a command never sent is a miss, which the late counter may not
    count: R17). It is shown only when the pair comes out VOID by `late_count` alone, with the setup counts and the
    RTT rule holding (the judge's rule for an expect-invalid bite, judge eval.rs:1283: "shown only when its run is
    INVALID by that one rule and holds every other"); a late-command pair that comes out PASS or FAIL means the check
    missed. The bite run's records state the delayed command's tick and that leg's `late_count` (expected exactly 1;
    any other value reported with its reason).
- **The runs at U4r:** three pairs (clean, wallclock, late-command), six runs, all on the one recording (seed 4). The
  plan does not fix the count. The advisor's "6 runs" is said of the re-run after U4h (advisor :346) and has two
  readings: U4R-Q12. U4R-Q10 (a') would add a fourth pair; its (b') adds none.
- **Records** (plan :761-763; condition 2, advisor :96-98): the headers list no mark, so the judge never reads these
  runs; results/u4r-precheck-30/<commit>/runs.txt and the comparer's output are committed; the advisor reads them in
  U4r's review, as for U3d's self-test (advisor-u3d.md, U3D-JUDGE); nothing goes into marks.json.
- **-30 stays NOT-RUN until U6** (plan :764): never reported as -30's part, never in U6's folder.

### 7.3 The run set's wall time: what U4r measures first, and the estimate
**Measured first** (plan :731-732), on the release build, before any set starts: the runner's ticks per second R,
per scene and profile the set uses, each a short timed run (for example 1,800 ticks, 60 s simulated, 48 clients, seeds
1-3), wall time read by the run script around the npt-lab process (2.3; npt-lab itself reads no clock: U4R-Q3), R =
ticks run ÷ wall seconds, the slowest of the three seeds kept. The builder measures it in its worktree (return list
item 8); the day session measures it again in the main folder before the set starts (needs a run (day): D4 in
section 11), and states the set's total before starting it.

**Simulated time of the set** (30 Hz), from section 7.1 and its assumptions:

| Item | Runs × length | Simulated s |
|---|---|---|
| -17 measured, also -57's (P0, P2, P3, B1 × seeds 1-3) | 12 × 600 s | 7,200 |
| -17's bites (tick16, client-rounding) | 2 × 600 s | 1,200 |
| -57's bites (pos-4cm, yaw-12bit, pitch-8bit) | 3 × 600 s | 1,800 |
| -18 measured (B1 × seeds 1-3), if not shared with -17's B1 runs (U4R-Q8) | 3 × 600 s | 1,800 |
| -18's bite (no-fallback) | 1 × 600 s | 600 |
| -58 measured (P0, P3 × seeds 1-3) and its six bites | 12 × 12 s | 144 |
| -30 pre-check (three pairs) | 6 × 300 s | 1,800 |
| the four runner recordings | 4 × 300 s | 1,200 |
| **Total** | | **15,744 s = 4.37 h = 472,320 ticks** (13,944 s = 418,320 ticks if -18 shares -17's B1 runs) |

**Wall time** = 472,320 ticks ÷ R, plus the builds (two release builds of npt-lab and one of npt-judge, as c-u2:52-61:
not measured here) and the judge's passes:

| R (ticks per second) | As a multiple of real time | The set's runs |
|---|---|---|
| 300 | 10× | 1,574 s ≈ 26 min |
| 90 | 3× | 5,248 s ≈ 1.5 h |
| 30 | 1× | 15,744 s ≈ 4.4 h |
| 10 | 0.33× | 47,232 s ≈ 13.1 h |

Assumptions, each to be replaced by the measured value: runs one at a time (c-u2 starts one process at a time:
c-u2:65-84; §7.2 :883's "one set at a time"); every bite run at its part's full length; each runner recording and the
pre-check's legs 9,000 ticks (5 min: §4.15 :467-468; marks.json -30's "3 recordings of 5 minutes", :6389); -58's runs
12 s; every run costing what SS with 48 clients costs (the ES and TC recordings likely less); no run repeated. The
tests' own runs (the contract test's short runs, the class audit, `replay`'s test, the twin test) run in the build's
and the merge's test steps, not in this set. Which R applies cannot be known before a run: the estimate is the
arithmetic, never the answer.

### 7.4 The evidence files
- **results/u4r/<commit>/verdict.json and runs.txt** (plan :768): the commit, the binaries' SHA-256s (both npt-lab
  builds and npt-judge, as c-u2:93-99), the raw logs' path, the judge's line, and one line per run; each run's line
  also gives its ticks and wall time (7.3). `<commit>` is the folder name c-u2 uses: the commit's first 12 hex digits
  (c-u2:46-49).
- **results/u4r-precheck-30/<commit>/runs.txt and the comparer's output** (plan :761-763): per pair, both legs' setup
  counts, `late_count` sums, buffer depths and tick bases, the RTT rule's per-client instants (first sample, first
  receipt, first run), the clients admitted with each one's admit tick and any drop or kick, and the result with its
  reason; for the late-command pair, the delayed command's tick and that leg's `late_count`; no verdict.json (the judge
  never reads these runs).
- **The recordings' SHA-256s** (plan :769) in runs.txt and in the committed recordings manifest (§4.15 :485-487), and
  U4d2's determinism step green on them (in check.mjs if the step stays within 5 minutes, else through
  ops/runs/det-u4.mjs at every merge that touches sim, proto, room or hash-wasm, on the advisor's ok: plan :769-770;
  whose file: U4R-Q2).
- **Raw logs** under .gate/netcode-proof-test/runs/<set>-<short>/ (c-u2:9-10, :48), never committed (plan §7.2 :884).
- **The sizes for fixes.rs** (§2 :150) as series in -17's (and -18's) B1 logs, with SERIES.md rows (U4R-Q2).

---

## 8. O1 (carry 40): what the -30 pre-check can and cannot show at U4r, and the re-run after U4h

### 8.1 The statement carry 40 asks for
At U4r the room has neither the view-time tolerance nor the cap: both land in U4h. The plan's own lines, at df8975b:
U4e builds "`rewind_at` (the integer half; the tolerance and cap rules land in U4h)" (plan :690-691); U4h builds
"`HitHistory`'s sizing and `rewind_at`'s rules (§4.9); the view-time estimate with the pump's RTT; the switchable cap;
the per-shot log of its parts" (plan :799-800). So the -30 pre-check's setup condition (0 view-time replacements and 0
clipped shots on both legs: the advisor's condition 3, advisor :107-108) holds by construction at U4r, and its RTT
argument (plan :749-758) cannot be tested there.

This rests on the plan's split of work: at df8975b crates/room/src/lib.rs:1-5 and crates/server/src/lib.rs:1-5 are
module docs only, so nothing in code can be checked yet. Recheck row R33 (run item D10) checks it against U4e's
merged code at `<base>`: every reader of the pump's per-client RTT and every replace, clip or substitute path in
npt-room and npt-server. Any reader outside the pump's own estimate and logs takes this statement back to the advisor
before U4r's build.

**The cites, checked.** O1 cites "u4-build-plan.md:678-679, :787" (advisor :344), and the conditions check it comes
from cites "U4e :678-679" and "U4h :787" (u4-conditions-check.md:42-45). At df8975b those lines hold other text:
:678-679 are U4d2's Bites ("the std `sin` ... the hash-map (caught, or recorded as not catchable on wasm32 ..."), and
:787 is U4g's `Predictor` line ("`World::predict`, the replay skipped when the stored prediction matches ..."). The
words O1 relies on stand at :690-691 and :799-800 at df8975b, as quoted above. The claim holds; only the line numbers
are those of an earlier text of the plan (probably the text before the U4s ruling's twelve condition-1 edits, advisor
:386-388, which moved §5 down: my reading, not checked against that text). Carry 40 itself cites by section ("§5 U4e's
`rewind_at` and §5 U4h": carries :179), so its words need no change. Also: advisor :343 names its source as
"u4-conditions-check.md:39-42"; at df8975b O1's text there is :42-45, the lines carry 40 cites.

### 8.2 What follows at U4r (this brief's reading of plan :740-758)
1. **The setup condition cannot fail at U4r.** No code in the room replaces a view time or clips a shot before U4h, so
   both counts are 0 whatever the build does. A check that cannot fail is evidence of nothing (the advisor's own words
   in the NF-D4 ruling, advisor :353). The pre-check's records say "not built: 0 by construction at U4r" (7.2, 8.4),
   never "the setup held".
2. **The RTT argument holds trivially and is untested at U4r.** The RTT reaches the room only through the tolerance's
   accept-or-replace and the cap's clip (plan :750-752; §4.9 :340-345), and the view-time estimate that reads the pump's
   RTT is U4h's too (plan :799-800). With none of them built, nothing in the room reads the RTT (R33 checks it at
   `<base>`), so the legs cannot decide differently through it, whatever the L1 leg's RTT is (U4c's 333 ms before its
   first sample, plan :555-557; at df8975b `rtt_ns` is `None` then, l1/src/lib.rs:349-353, until R2 settles it at
   `<base>`; Some(0) on Loopback: link/src/lib.rs:203-205). No defect on the RTT's path can show.
3. **What the pre-check does test at U4r:** that the two legs give equal per-tick World hashes (7.2's comparer) when
   the same commands run at the same ticks: carry 21's canonical order, which "-30's comparer relies on" (carries
   :146-147); option B's buffer; and the sim's inputs (which command runs at each tick, and its Rewind) being
   independent of the link once `late_count` is 0 and no view time is replaced or clipped. The encoder's baseline, the
   window's remainder, the priority skips, event resends and Resync do follow each leg's acks (§4.5 :283-293; §4.7
   :321-324), but they stay out of the World hash, and a client dropped or kicked on one leg is listed beside the
   result (7.2). It also tests `late_count` (condition 2); the wallclock bite, a mismatch on a valid pair, which needs
   no tolerance and no cap; and the late-command bite, a VOID by `late_count`, which needs neither. And the RTT rule's
   premise: each client's first-sample instant, first-command receipt instant and first-command tick are logged at
   U4r. The "first command before first sample" VOID can fire only when a command runs in the instant `JoinAccepted`
   is sent, a timing no waiting replay client reaches (it rests on U4c's immediate acks and L-c6, plan :558, :571, and
   on the replay clients' wait for `JoinAccepted`, :753-756), and even then only if the server end's estimate, as the
   pump reads it, does not yet hold that packet's sample at that tick (at df8975b a P0 ack's sample applies at its
   packet's own send instant: l1/src/lib.rs:515, :527; after U4c: R4). A breach changes no hash at U4r. How the rule's
   code is shown to fire: U4R-Q10.
4. **So at U4r the pre-check is evidence for the comparer and the command path, not for the RTT argument nor for the
   replacement-and-clip half of the setup condition.** U4r's report and the advisor's reading of
   results/u4r-precheck-30/ say so.

### 8.3 The proposal: re-run the pre-check pair after U4h merges (the advisor's lean, advisor :346-347)
- **Why:** U4h builds exactly what the argument is about: the estimate with the pump's RTT, the tolerance, the
  switchable cap and the per-shot log of their parts (plan :799-800). Only then can a replacement or a clip happen, so
  the setup condition can fail, and only then can the L1 leg's RTT before its first sample reach the room, so the
  argument means something: in a correct build each L1 client's first sample comes before its first command (the
  ordering log shows it), its RTT is about 0 at P0, both legs accept the same view times, and the pair is PASS with 0
  replacements; a client whose first command ran first would carry U4c's 333 ms of RTT (R2) into its estimate and be
  caught by the RTT rule (and, at P0, very likely by its replacements).
- **What it is (this brief's reading of the advisor's "6 runs"; the other reading and the choice: U4R-Q12):** the same
  three pairs as at U4r (7.2: the clean pair, the wallclock pair and the late-command pair), 6 runs, on the merged U4h
  build, by the same script, its results in results/u4h-precheck-30/<commit>/ (a folder name this brief proposes:
  U4R-Q12), read by the advisor in U4h's review as condition 2 asks for U4r's; -30 stays NOT-RUN until U6.
- **The recording:** first replay U4r's SS P0 seed 4 recording on U4h's room and count the replacements; this replay
  is the re-run's clean pair. U4r's clients have no `Interp` (U4g, plan :789) and no `ClockSync` (U4f, :774), so the
  view times they stamp may not be what U4h's estimate expects, and §4.15 :481-483's reason for a P0 recording ("a
  recording made at P0 keeps them inside the tolerance") was written for clients that interpolate. If any view time is
  replaced on either leg, that clean pair is VOID: re-record SS P0 seed 4 with `scenario --record` on the U4h build
  (one more run, its SHA-256 beside the old one's), then run the six. Needs a run (day), after U4h merges: D7 in
  section 11.
- **Its cost, both branches:** 6 runs of the recording's length (about 6 × 300 s = 1,800 s simulated) if the
  replay-first clean pair shows no replacement; 9 runs otherwise (the VOID pair, the re-recording, then the six: about
  2,700 s). At 7.3's R values: 1,800 s takes about 3 min at R = 300 and 30 min at R = 30; 2,700 s about 4.5 min and
  45 min. U4R-Q12's reading (ii) and U4R-Q10 (a') add to these (U4R-Q12 gives their totals).
- **Why not the alternatives:** (b) no re-run leaves the setup condition and the RTT argument untested until U6's -30 C
  part on the real transports (plan :850-851), where a VOID or a mismatch would mix the transports' effects with the
  room's, which is what the pre-check exists to separate first (conditions 2 and 3); (c) moving U4r's pre-check to
  after U4h drops what it already tests at U4r (8.2 item 3), and condition 2 records it as U4r's (advisor :97).

### 8.4 What U4r builds now so the re-run reads real counts (this brief's proposal)
- The pre-check script reads the replacement and clip counts by name from the room's logs. At U4r no such series
  exists (U4h's per-shot log is not built: plan :800), so the script writes "not built: 0 by construction" for both,
  never a bare 0; after U4h it reads U4h's series, and it gives no result (VOID, with the reason) if a room log lacks
  them, the same logic as §4.12's validity inputs (:400-401) and the judge's gap rule (judge eval.rs:310-323). Your
  report names the switch and its test.
- The per-client first-sample instant, the first command's receipt instant and its run tick (7.2) are written at U4r,
  so the re-run needs no new log whichever instant U4h's estimate reads the RTT at (the receipt, if the estimate is
  taken in the pump; the run, if `rewind_at` reads it at the command's tick: check N3).
- U4h's brief then carries the re-run and its two premises (a line the main session adds to u4s-carries.md or to U4h's
  brief: U4R-Q12).

---

## 9. Recheck after wave 1 (and wave 2)

Every place this brief names an item U4c, U4d2, U4e or U4j builds (rows R1-R29, and R33 and R34, which the netcode
check asked for), plus three items of U4d, U4b and U4a it leans on (R30-R32), for the day session to check against the
merged code at `<base>` before sending the brief to the advisor. "As named" means the name, the shape and the
behaviour this brief assumes; each row says what settles it.

| Row | Item (its name in the plan) | Built by | Its use in U4r (brief section) | Check at `<base>` |
|---|---|---|---|---|
| R1 | `L1Net::next_event_ns`, C1' event-driven (plan :553-554; §4.1 :195) | U4c | how `L1Scenario` reaches each instant (2.1) | its signature; U4c's report shows L-c5 (:570) passing |
| R2 | the RFC 9002 starting state: smoothed RTT 333 ms before the first sample, which sets it (plan :555-557) | U4c | the RTT rule (7.2); O1 (8.2) | the value in l1's code (D2) and what `rtt_ns` returns before the first sample (Some(333 ms), or None) |
| R3 | immediate acks on their own per-flow stream (plan :558-560); L-c6, internal events before application sends at one instant (:571) | U4c | the RTT argument's premise (7.2; plan :753-755) | L-c6's test name and its pass in U4c's report |
| R4 | C2's sampling rules (L-c7, plan :572-573; §2 :145) | U4c | the per-client first-sample instant (7.2); U4R-Q10 | how the lab learns the instant at which the server end of a link first holds an RTT sample: an accessor, or `rtt_ns` read after each event (name the item); and, at P0, when JoinAccepted's ack sample is applied relative to JoinAccepted's own send instant (at df8975b, at that instant itself: l1/src/lib.rs:515, :527): D12 |
| R5 | the fate log v2, tag `npt-l1 fate log v2` (plan :562-563) | U4c | r1's fate-log hashes (2.4) | the tag, and how a run's fate-log hash is read (U2's probe writes `fate_log_sha256`: lab/src/main.rs:116) |
| R6 | `L1Net::with_bite` and U4c's bite ids (l1/src/lib.rs:858; plan :578-581) | U4c | the `--bite` dispatch (2.1) | its signature (at df8975b `with_bite(seed, bite) -> Option<L1Net>`) and its id list's name |
| R7 | `Profile` P0, P2, P3 and `Profile::b1(burst_starts_ns)` (l1/src/lib.rs:116, :135, :143) | U2, kept by U4c | every run's profile; -18's bursts (7.1) | unchanged by U4c; B1's burst still 1 s of 100 % loss both ways |
| R8 | `HashCompare`, which "takes two per-tick hash lists" and names the first mismatching tick (plan :652-654) | U4d2 | the pre-check's comparer (7.2) | its signature; what it reports when one list is shorter (one leg ended early) |
| R9 | npt-lab's `hash` subcommand (plan :651-653) | U4d2 | U4r's `scenario` and `replay` sit beside it (2.1, 2.7) | main.rs's dispatch after U4d2; no name clash; whether `hash` reads a clock (D8; U4R-Q3) |
| R10 | check.mjs's `wasm` step: its 5-minute budget and printed wall time (plan :656-657) | U4d2 | the recordings' determinism route (7.4; U4R-Q2) | the step's time at `<base>` on U4d's three synthetic recordings (D3), so the budget left for U4r's four is known |
| R11 | crates/lab/Cargo.toml as U4d2 left it (Lanes :832-834) | U4d2 | U4R-Q1 | D5: whether npt-bots is a dependency of npt-lab, and whether npt-bots gained npt-sim |
| R12 | check.mjs's steps (10 at df8975b: :1474-1507; U4d2's `wasm` step added: :656) | U4d2 | section 5's "every step passing" | the step count at `<base>` (D3) |
| R13 | `Room<S: SimCore>`, `Room::with_bite` (§4.1 :193; §4.11 :387) | U4e | `L1Scenario`; the dispatch (2.1) | the constructor; how a bite reaches it; whether npt-room has a bites-only `BITES` list (U4e's U4E-Q3 ruling) |
| R14 | `Roster`, tokens passed in by the caller (plan :687) | U4e | the lab's tokens (2.1) | the token type and how the lab hands each slot's token in |
| R15 | `Decoder::pump`: decode, socket checks, ping sender and answers, per-client RTT and bytes sent, on an injected now (plan :693-694; §4.1 :194) | U4e | step 2 of the order; the RTT rule (7.2); the wallclock bite's site (U4R-Q6) | its signature; whether the per-client RTT it uses is readable by the lab; where in a pump call it reads each session's `rtt_ns` (before or after the call's sends); how a session's kind (`wt` and L1, or `ws`) is set, so the Loopback leg's sessions take the link's `rtt_ns` (7.2; check N3) |
| R16 | `ClientEndpoint` (input, step, the next due time in ns) and `StepOut` (plan :697; plan.md:608) | U4e | step 5 of the order; the client logs; -57's pairing; -18's heal record (2.1) | what it gives per tick: the client hash or the sentinel, `snap_incomplete`, the decoded values -57 pairs, and each decoded delta's encode tick (the fresh-delta reading needs it) |
| R17 | `InputBuffer` option B with `buffer_depth_ticks` (plan :690) | U4e | every run's `cmd_timing` B; the pre-check's depth, pacing and late-command bite (7.2) | how the depth is set and read; how far ahead of its tick a command is kept (the replay clients' pacing); how a late command is counted (once, when its first copy arrives after its tick has run; a redundant copy of a command already run never counted) |
| R18 | the encoder's exposed values: `burst_eviction`, `snap_incomplete`, the server's mirror hash per (client, tick), the fields each snapshot carried per client (§4.5 :289-290; §4.6 :297-318; U4e's U4E-Q7) | U4e; pre-grid values from U4d's `EntityRecord::raw` (carry 9) | 2.1's logs; -57's pairing; -18's record | each value's name and how the lab reads it |
| R19 | `rewind_at`'s answer for a view time the ring does not hold, and its counter (U4e's U4E-Q5 ruling) | U4e | the pre-check's out-of-ring count (7.2) | the counter's name; logged per command |
| R20 | the bites U4e planted and where: tick16's home (U4E-Q4), client-rounding, lt-ack, lt-tick-offset (U4E-Q2), lt-event-seq, lt-cmd-seq and lt-press (U4E-Q1), no-fallback, owner-to-all, no-trailing, no-channel; and U4e's -18 end-to-end finding (condition 6, advisor :128-129) | U4e (tick16 with U4a) | the judged runs' bites (7.1); r4 (2.4) | D6; U4e's report for which -58 bites were planted and for the -18 finding |
| R21 | marks.json -18's split, `burst_heal` and `burst_return`, its checks (plan :632; §2 :149) | U4j | -18's record (2.1, 7.1) | the event and series names; the instant `after_link_ns` counts from; whether the fresh-delta reading is in the check's words or its notes |
| R22 | §4.12's table in marks.json: scene SS, clients 48 and `pre:PM-0001-2` on -17, -18, -20 and -21 (plan :405-411, :632) | U4j | each header's scene and clients; the precondition (7.1) | each part's select at `<base>` |
| R23 | npt-log's `hs` (`null` for a client with no hash at a tick), `admit` events, footers, `tick.v.late_count`, one header per process value (plan :627, :633-635; §4.12 :397-401) | U4j, on U2's npt-log | the per-process logs (2.1) | the field names; U4j's schema test |
| R24 | the figures `decoded_hash_mismatch_count`, `decoded_hash_compared_count`, `twin_hash_mismatch_count` (plan :627-629) | U4j | -17's and -58's checks (7.1); r1 if judged through the figure | what each reads: the room log's mirror hash per (client, tick) and the client logs' `hs` |
| R25 | README §1's layer value `loopback` (plan :633-635) | U4j | the pre-check's Loopback leg (7.2) | its spelling |
| R26 | SERIES.md's new rows: `snap_incomplete`, `burst_eviction`, the clamp events, the precheck suite, the figure rows (plan :639-640) | U4j | r2's "every written name has its SERIES.md row" (2.4) | which names U4r writes still lack a row (the -57 error series, the fixes.rs sizes, `counter_wrap`, the RTT rule's three per-client instants, each client's admit tick, -18's `FullState` flag per client and burst): U4R-Q2 |
| R27 | marks.json -30's report series `header.params.buffer_depth_ticks_count` (marks.json:6368-6372) | marks.json at `<base>` (U4j edits it: plan :632) | the pre-check's depth param (2.1, 7.2) | the name at `<base>` (the plan writes `buffer_depth_ticks`: :743) |
| R28 | C4: a measured run from a bites build, or a bite run from a measured build, is INVALID (plan §2 :147; advisor-u2.md:101-103) | U4j | the scripts' build per run (2.3, 7.1) | its fixtures pass at `<base>` |
| R29 | marks.json's bite ids for -17, -57, -58, -18 and -30 (marks.json:3965-3970, :4062-4073, :4203-4226, :4296-4299, :6375-6384) | marks.json at `<base>` | the ids the scripts pass to `--bite` (7.1) | each id the scripts name is marks.json's spelling (D6) |
| R30 | `World`, `Scene` (SS, ES, TC), `Recording` (its header: scene, seed, TICK_HZ, pool sizes, HitHistory's capacity: §4.15 :463-466; nit b; carry 14), `replay::Replayer` (plan :586-591) | U4d | the runner and its recordings (2.1); the room log's per-tick World hash and the pre-check's tick base (7.2) | the Recording writer's API and header fields (the span in ms beside TICK_HZ: carry 14); TC's scene constructor; the canonical per-tick hash's name (FNV-1a-64: plan :590) and how the lab reads it each tick; whether `Recording` carries the recording run's start tick, or only each command's stamped tick |
| R31 | `Schedule::new(rate, catch-up)` (plan :543; carry 6) | U4b | step 3 of the order (2.1; carry 6) | its signature |
| R32 | `Quant`'s own bite choice for -57's three bites (carry 2; plan :529-531) | U4a | -57's bite runs (7.1) | how a caller selects them per instance through the encoder |
| R33 | every reader of the pump's per-client RTT, and every replace, clip or substitute path (plan :690-691, :693-694; §4.9 :337-345) | U4e | carry 40's statement (8.1); the setup counts "not built: 0 by construction" (7.2, 8.4) | D10: none outside the pump's own estimate and logs, and `rewind_at` substituting nothing for a view time the ring does not hold; any reader takes O1's statement back to the advisor before U4r's build |
| R34 | npt-client's `Resync` sender, and the event queue's cap (§4.7 :321-326; catalogue :90) | U4e | -18's heal record and its `FullState` flag (2.1, 7.1) | D11: whether the client sends `Resync` on an undecodable delta or only when it "has lost its state" (catalogue :90); the cap's value against §4.7 :325-326's "below about 400" |

---

## 10. Questions for the advisor

Only where the plan or a ruling leaves a choice open. Each is self-contained.

**U4R-Q1 (the npt-lab to npt-bots dependency).** May npt-lab gain a dependency on npt-bots, and npt-bots one on
npt-sim, and who makes the edit?
- Why it matters: `L1Scenario`'s 48 clients run npt-bots' `Behaviour` (plan :723; §4.1 :196), and the bots read the
  button bits from npt-sim (carry 17); at df8975b neither edge exists (lab/Cargo.toml:23-34; bots/Cargo.toml:20-25).
  The Lanes give manifests to U4s (wave 1 and U4e's crates: :815-819) and, in wave 2, to U4d2 alone (:832-834); none
  to U4r. Both are workspace crates already in the lock, so no package is fetched.
- Options: (a) U4r adds both edges and names them in its report as the only Cargo.lock change (§5's preamble, :492);
  (b) the main session adds them in its own commit on committed state before U4r's build starts (condition 10 (d)),
  and U4r's lane stays free of manifests; (c) `Behaviour` lives in npt-lab (against §4.1 :196).
- Recommendation: (b): every manifest change so far came from U4s or U4d2 under a ruling, and a separate commit keeps
  the builder's diff to sources; (a) if the advisor prefers one step less.

**U4R-Q2 (the files outside U4r's three paths).** Who writes (i) the SERIES.md rows for the names U4r writes that U4j's
rows (plan :639-640) leave out (the -57 error series, the fixes.rs sizes, `counter_wrap`, the RTT rule's three
per-client instants, each client's admit tick, -18's `FullState` flag, the pre-check's params), (ii) the committed
recordings manifest (§4.15 :485-487), and (iii) the determinism step's change for the runner recordings (in check.mjs
if within 5 minutes, else ops/runs/det-u4.mjs on the advisor's ok: plan :769-770)?
- Why it matters: r2 fails on a written name with no row (§4.12 :400); the evidence needs the recordings' SHA-256s with
  U4d2's step green on them (:769); none of these files is in U4r's lane, and the plan names no owner (check.mjs is
  U4d2's in wave 2, Lanes :832-833, and no lane's during U4r). Regenerating the runner recordings "at check time" means
  four `scenario --record` runs of about 9,000 ticks each inside the step, which 7.3's arithmetic puts beyond 5 minutes
  unless R is above about 120 ticks per second (36,000 ticks ÷ 300 s), so det-u4.mjs is the likely route.
- Options: (a) U4r's lane gains marks/SERIES.md (new rows only), the manifest file, and check.mjs's recordings list or
  ops/runs/det-u4.mjs; (b) the main session writes all three after U4r merges, before the judged runs; (c) the rows and
  the manifest in U4r (they follow the code that writes those names and files), the determinism route by the main
  session once U4r's measured times are in, on the advisor's ok (the plan's own words, :770).
- Recommendation: (c).

**U4R-Q3 (the clock bans for npt-lab and npt-bots, and where ticks per second is timed).** Does U4r add §4.11's
crates/lab/clippy.toml and crates/bots/clippy.toml, each with the clock ban (`std::time::Instant`,
`std::time::SystemTime`, `tokio::time::Instant` and their `now`: §4.11 :380-383)?
- Why it matters: §4.11 names both files and no §5 unit lists them; U4r is the first unit to write npt-lab's runner
  and npt-bots' library; a clock read in the runner would undo the twin test's determinism, yet r5 needs a wall clock
  somewhere. At df8975b neither file exists, and npt-lab's sources read no clock (lab/src/: no `Instant` or
  `SystemTime`); U4d2's `hash` may (R9, D8).
- Options: (a) both files in U4r, crate-wide (npt-bots' binaries are stubs at df8975b: bots/src/bin/npt-bots.rs:1-3,
  npt-load.rs:1-3; U6 adds its reviewed allows: §4.11 :382-383), and the run scripts time each npt-lab process; (b)
  npt-lab's file in U4r, npt-bots' with U6's runtime half; (c) neither at U4.
- Recommendation: (a), unless D8 shows U4d2's `hash` reading a clock, in which case (a) with that one reviewed allow.

**U4R-Q4 (the contract test's scope at U4r).** Which suites and parts does r2 cover at U4r?
- Why it matters: condition 5 (advisor :120-121) and plan :726-727 ask PM-0001-2 to pass on "each of -21's eight
  suites" and on -20's P3, and §4.12 :399 asks that "every check of every U4 L1 part resolves"; but four of -21's parts
  select `cmd_timing` A (marks.json:4631-4704) and -20's P3 is "A and B" (§4.12 :409), while option A and the cheats'
  `Behaviour`s are U4f's (plan :774, :776); and the series of the parts U4f, U4g and U4h move come from their code, so
  at U4r they would be gaps, which make runs INVALID (judge eval.rs:310-323).
- Options: (a) at U4r, r2 covers -17's four profiles, -18's B1, -58's `wrap` suite, -20's P3 option B and -21's four
  option-B suites, the open-loop scripts standing in for the cheats' command patterns (PM-0001-2 and the shot band
  only; those suites' own checks not resolved), and each later unit extends r2 to its parts before its judged runs,
  option A's suites joining at U4f; (b) as (a), but -20's and -21's suites wait whole for U4f; (c) U4r writes every
  later part's series as placeholders (a placeholder is a check that cannot fail).
- Recommendation: (a): it keeps condition 5's order (the load passes before the marks count; -20 and -21 move in U4f:
  :777-779) for every suite that can run now.

**U4R-Q5 (-58 at U4r).** How is -58 judged at U4r, when its part checks "0 untraced corrections" and its bite
lt-tick-offset fails only that check, both of which need U4g's `Predictor`?
- Why it matters: marks.json:4162-4169 checks `tick.v.untraced_corrections_count`; §4.14 :454 gives lt-tick-offset
  that check; corrections exist only with prediction (U4g, plan :786). Unwritten, the series is a gap and every -58 run
  is INVALID (judge eval.rs:310-323, :1048-1051); written as 0 at U4r, it is a check that cannot fail. U4e's brief
  already proposes that lt-tick-offset's judged run wait for U4g (its U4E-Q2 (c)).
- Options: (a) U4r judges -58 with the series written as 0 and the check stated as vacuous at U4r, re-judged after U4g
  with lt-tick-offset; (b) -58's judged runs move to after U4g; U4r runs them unjudged as a recorded dry run (as the
  pre-check is), showing the other five bites and checks from the logs; (c) a judge change so a series can be "not
  built" (NOT-RUN) at U4r: U4j's crate, not U4r's lane.
- Recommendation: (b): no vacuous PASS and no judge change, the wrap code still exercised at U4r, and the runs are 12 s
  each.

**U4R-Q6 (the wallclock bite's site).** Who plants PM-0001-30/wallclock, "a wall-clock read in the pump for one leg's
sessions, bites build only" (plan :759)?
- Why it matters: the pump is npt-server's (U4e, :693); U4e's bite list (:711-712) does not name it; U4r's lane holds no
  npt-server file; server/clippy.toml bans clocks (U4b, :543-544), so the site needs a bites-only allow, as the sim's
  bite sites carry (plan :622-623); check.mjs's one-crate rule then puts its id in npt-server's sources. Wherever it
  sits, the read must change a sim input on that leg on every run, whatever the machine's ticks per second (7.2; check
  N8; marks.json:6377: "a wall-clock read on one transport's command path"): on the commands the pump passes on, never
  in its RTT or ping code, which nothing reads at U4r, nor in a rate bucket, whose starving depends on the machine.
- Options: (a) U4r's lane gains that one bites-only site and its allow in crates/server/src, nothing else; (b) the main
  session plants it in its own commit before U4r's build; (c) the site moves into npt-lab's leg glue (not "in the
  pump", and npt-lab's own ban, U4R-Q3, would need an allow too).
- Recommendation: (a): the plan plants each bite where its code lives (§4.14 :438), and U4r is the unit that shows it.

**U4R-Q7 (-17's clean runs' `tick_start`).** Do -17's clean L1 runs start at `tick_start` = 63,736 like its tick16 bite
runs?
- Why it matters: the plan sets 63,736 on the bite runs (plan :734); marks.json's tick_start setup check covers only the
  S30 part (marks.json:3941-3951). If the clean runs start at 0, a bite run differs from its clean run in two things,
  and no clean L1 run shows the codec surviving the 16-bit wrap under the stress scene's load (-58's runs last 12 s).
- Options: (a) every -17 L1 run, clean and bitten, starts at 63,736; (b) the clean runs start at 0.
- Recommendation: (a): same run count and length, one difference per bite run, and the wrap covered under load.

**U4R-Q8 (-18's runs).** Are -18's L1 B1 runs -17's B1 runs (10 bursts each, headers listing -17, -57 and -18), or a
set of their own?
- Why it matters: both parts select L1 B1 with SS and 48 clients (marks.json:3882-3888, :4248-4255; §4.12 :407-408), B1
  is the burst profile, and the judge selects runs by the marks their headers list (judge eval.rs:30); separate runs add
  1,800 s simulated (7.3).
- Options: (a) shared, with -18's burst settings (10 bursts, offsets 0 to 1 tick) on -17's B1 runs; (b) separate.
- Recommendation: (a).

**U4R-Q9 (the TC B1 recording at U4r).** Which clients make the TC B1 seed 3 runner recording (§4.15 :479-480), when
the TC shooter `Behaviour` is U4h's (:801) and U4r builds only the brawl and the open-loop scripts (:723)?
- Why it matters: the runner recordings are meant to hold "the room's real Rewinds" (§4.15 :479-480); a TC recording
  made without TC's shooter holds Rewinds that shooter would not make.
- Options: (a) U4r records it with the open-loop scripts aimed at TC's targets, and U4h re-records it with its shooter
  (the manifest's generator version changes); (b) U4r records SS P2 seed 1, ES P3 seed 2 and SS P0 seed 4, and TC B1
  seed 3 waits for U4h, the determinism step keeping U4d's three synthetic recordings meanwhile (§4.15 :478, :484); (c)
  the brawl on TC.
- Recommendation: (b): U4h builds both the shooter and the Rewind rules (:799-801).

**U4R-Q10 (showing the RTT rule can fire).** How is it shown that the pre-check's RTT rule (a client whose first
command ran before its first sample makes the leg VOID: plan :756-758) can fire?
- Why it matters: the rule is the pre-check's only run-level guard on the RTT argument once U4h's estimate reads the
  RTT, and the plan's two bites (:759-760) never exercise it; a guard never shown to fire is not shown to work (the
  "bites first" rule: catalogue :237-238; §5's preamble :494-495). Rev 1's early-input bite (one L1-leg replay client
  sends its first `Input` before it decodes `JoinAccepted`) cannot fire as worded (check N2): at P0 an `Input` sent at
  any step after the `Join` reaches the pump no earlier than the instant after `JoinAccepted` is sent, when the server
  end already holds its sample. It could fire only if the client sends `Join` and its first `Input` in one step, both
  decoded in the one pump call that sends `JoinAccepted` (the `Join` first by fate index, else the `Input` is "a
  datagram before Join dropped and counted": plan :703), the `Input` stamped for the tick due at that instant; and
  even then only if the server end's estimate, as the pump reads it, does not yet hold `JoinAccepted`'s sample when
  that tick runs. At df8975b a P0 ack's sample applies at its packet's own send instant (l1/src/lib.rs:515 with 0 delay
  each way at P0, :418-420; applied at any later call at that instant, :527, :582, :714), so whether that timing fires
  depends on U4c's and U4e's merged code (R4, R15; D12).
- Options: (a') the run-level bite with that exact timing, at U4r and in the re-run (8 runs each), expected "the L1
  leg VOID by the RTT rule alone, `late_count` 0, the Loopback leg valid"; if U4e's room refuses an `Input` in the pump
  call that admits its `Join`, or D12 shows the sample already held when that tick runs, the bite is not exercised and
  the report says so; (b') test r7 (2.4): the rule's code on doctored per-client records (a first-sample instant after
  the first command's run instant: VOID; equal instants: not VOID; no first sample on the L1 leg: VOID; Loopback's
  "Some(0) from the start": never VOID), U4R-Q11's pattern, with no runs; (c) neither.
- Recommendation: (b') at U4r; (a') only in the re-run after U4h, and only if the advisor wants a run-level proof and
  D12 shows the timing can fire.

**U4R-Q11 (negative cases for the contract test).** Does r2 get negative cases: a short run's logs doctored to lose one
`admit` event, one footer, one written name's SERIES.md row, or one `late_count` series where `late-zero` applies, each
failing r2 by name?
- Why it matters: the plan gives r2 no bite (plan :725-728), so nothing shows it can fail, and it is the test that
  stands between a log missing a validity input and a judged run.
- Options: (a) yes, as unit tests on doctored copies, the catalogue's "bites first" pattern (catalogue :237-238); (b) no.
- Recommendation: (a).

**U4R-Q12 (the re-run after U4h: its shape and its home).** Which re-run does the advisor's "re-run the pre-check pair
after U4h merges (6 runs)" (advisor :346; carries :181) mean, and where does it live: section 8.3's replay-first rule
for the recording, results in results/u4h-precheck-30/<commit>/ read in U4h's review, and a carry for it in U4h's
section of u4s-carries.md (:184-189)?
- Why it matters: carry 40 asks this brief to propose the re-run or say why not, and the words have two readings
  (check N4), so the brief must not settle them alone; U4h's §5 (:798-812) has no line for the re-run, so without a
  carry and a folder it has no owner.
- The two readings, with their cost (each run the recording's length, about 300 s simulated; 7.3's R values turn them
  into wall time):
  - (i) this brief's (8.3): one recording (SS P0 seed 4), three pairs (clean, wallclock, late-command): 6 runs, about
    1,800 s simulated; 9 runs, about 2,700 s, if the replay-first clean pair shows replacements (the VOID pair, one
    re-recording, then the six).
  - (ii) -30's own shape ("3 recordings of 5 minutes ... so all 6 runs are measured": marks.json:6389; parts `min` 6:
    :6344, :6353): three P0 recordings (two more recorded on U4h's build), three clean pairs plus the two bite pairs:
    10 runs and 2 recordings, about 3,600 s; 12 runs and 3 recordings, about 4,500 s, if seed 4 must be re-recorded.
  - Not offered: three clean pairs on the one recording would repeat one deterministic run (the same commands, and P0
    draws no delay and no loss: l1/src/lib.rs:115-117), so they would show nothing one pair does not.
  - U4R-Q10 (a') in the re-run adds one pair (600 s) to either.
- The carry's two premises, whichever reading (check N3): the re-run compares each client's first-sample instant with
  the instant the room first reads that client's RTT for a command (its receipt, if U4h's estimate is taken in the
  pump; its run, if `rewind_at` reads it at the command's tick), both logged at U4r (7.2); and the Loopback leg's
  sessions take the link's `rtt_ns` (Some(0)), not the `ws` ping samples (§4.9 :341-344), as U4r's leg glue sets them
  (R15).
- Options: (a) reading (i), as 8.3 proposes, with the carry; (b) reading (ii), with the carry; (c) either reading,
  written straight into U4h's brief from this ruling, with no carry; (d) no re-run (8.3's alternative (b)).
- Recommendation: (a): the pre-check separates the room's command path from the link, which one P0 recording tests;
  the three-recording shape is -30's C part at U6 (plan :850-851).

---

## 11. Needs a run (day)

What only a build, a test or a run can settle, with its command. Commands run from the main folder unless marked;
`<base>` is the SHA of section 1. (Rev 1 named these N1-N9; rev 2 renames them D1-D9 so they never read as the netcode
check's findings, and adds D10-D12.)
- **D1, the base counts** (section 4 rule 4), from spikes/netcode-proof/ at `<base>`:
  `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true cargo test --locked --workspace`
  and `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target CARGO_NET_OFFLINE=true cargo test --locked --workspace --all-features`,
  each run's `test result:` lines summed; or check.mjs's test-step line at `<base>`.
- **D2, the starting RTT** (R2): `git grep -n 333 <base> -- spikes/netcode-proof/crates/l1/src`, then a read of what
  `rtt_ns` returns before the first sample.
- **D3, check.mjs's step count and the `wasm` step's time** (R10, R12), from spikes/netcode-proof/ in the main folder at
  `<base>`, its own cache: `CARGO_NET_OFFLINE=true node check.mjs`, reading the last line and the `wasm` step's printed
  wall time.
- **D4, the runner's ticks per second** (7.3), in the main folder after the merge, before any set starts: the timing
  command the builder reports (return list item 8). Its exact form cannot be written before the build.
- **D5, the manifests** (R11; U4R-Q1): `git diff df8975b <base> -- spikes/netcode-proof/crates/lab/Cargo.toml spikes/netcode-proof/crates/bots/Cargo.toml spikes/netcode-proof/Cargo.lock`.
- **D6, the bite ids at their sites** (R20, R29): `git grep -n PM-0001-17/ <base> -- spikes/netcode-proof/crates`, and
  the same for `PM-0001-18/`, `PM-0001-57/`, `PM-0001-58/` and `PM-0001-30/`, read against marks.json's ids.
- **D7, after U4h merges** (8.3): the pre-check's clean pair on U4r's SS P0 seed 4 recording, on the U4h build, by the
  pre-check script, reading both legs' replacement counts (re-record if either is above 0); with both at 0, this pair
  is the re-run's clean pair.
- **D8, a clock in npt-lab at `<base>`** (R9; U4R-Q3): `git grep -n -e Instant -e SystemTime <base> -- spikes/netcode-proof/crates/lab spikes/netcode-proof/crates/bots`.
- **D9, the recordings' time** (U4R-Q2): the four `scenario --record` runs' wall times as the builder reports them
  (return list item 9), against check.mjs's 5-minute budget less D3's `wasm` time.
- **D10, the RTT's readers and the replace, clip or substitute paths** (R33; 8.1; check N5):
  `git grep -n -i -e rtt -e round_trip -e srtt <base> -- spikes/netcode-proof/crates/room/src spikes/netcode-proof/crates/server/src`
  and `git grep -n -i -e replace -e clip -e tolerance -e estimate -e rewind_at <base> -- spikes/netcode-proof/crates/room/src`,
  each hit read; any reader of the per-client RTT outside the pump's own estimate and logs, or any path that replaces,
  clips or substitutes a view time, takes O1's statement back to the advisor before U4r's build.
- **D11, the client's `Resync` and the event cap** (R34; -18's record, 2.1; check N9):
  `git grep -n -e Resync -e FullState <base> -- spikes/netcode-proof/crates/client/src` and
  `git grep -n -e EventQueue <base> -- spikes/netcode-proof/crates/room/src`, then a read of when the client sends
  `Resync` (on an undecodable delta, or only when it has lost its state) and of the queue's cap value.
- **D12, when the server end first holds a sample at P0** (R4, R15; U4R-Q10):
  `git grep -n -e rtt_ns -e advance_acks -e sample <base> -- spikes/netcode-proof/crates/l1/src`, then a read of when
  a P0 ack's sample is applied relative to its packet's send instant; and, from D10's first grep, where in a pump call
  the pump reads each session's `rtt_ns` (before or after that call's sends).
