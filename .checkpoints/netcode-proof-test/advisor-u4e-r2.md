# Advisor: U4e round 2 (on what changed: the fix pass FX-1 to FX-13), 2026-11-12

Seat: the advisor, run as a general-purpose agent (Opus, xhigh) by the day session's workflow (branch
claude/netcode-proof-test-plan-hcky5g; PR #40). Role read whole from main (`git show
origin/main:.claude/agents/advisor.md`, main at 2aa5e9c). I rule under my round-1 ruling's outcome rule
(advisor-u4e-r1.md, R8, :431-434) and CLAUDE.md's review budget. I write only this file, with Write and Edit, block by
block. Read only: no build, no run, no plant; reading, git and sha256 only.

**Verdict: APPROVE; the merge goes ahead on M1-M9 as Q3 amends them.** Round 2 is complete: five APPROVE WITH
FINDINGS, no Critical or High, one Medium (NC2-M1), twelve Lows, none before the merge. NC2-M1 becomes a U6 carry
(C-U6-16 (a), with SEC-U4E-R2-1); the catalogue's `Resync` row gains the limit in the merge commit (T7, closing
SEC-U4E-R2-2); eight Lows go into carry texts, one into T1, two onto the card. T1, T2 block (3), T3 and T5 change; T4
and T6 stand; T7 is new. The test seat's longer wait is accepted. Nothing goes to Mustafa.

## Plan
1. Read the brief (advisor-u4e-r2-brief.md), my round-1 ruling (R1-R9, T1-T6), the packet (sha256), the five seats'
   round-2 notes, the builder's "Fix pass, round 1", history.md's 15:32 UTC decision entry.
2. Spot-check what the rulings rest on: the copy's 23 files against fx-hashes-end.sha256 and the snapshot; the Resync
   limit and the client's case 3 in the copy (NC2-M1, SEC-U4E-R2-1, NC2-L1, TE2-L1, RR2-3); the catalogue's Resync row
   (:90); T1-T6's anchors at the branch head after bfda3cd; R9's premises (spikes/ unmoved; main-run.sh, the gate's
   layout); the counted run's log (473 / 541 / 52, 10 steps) and its limit.
3. Rule items 1-4, one block each (Q1 ... Q4), then any text, the two lists, and what I checked.

## Progress
- [x] 1 read (the brief; advisor.md from main; R1-R9 and T1-T6; the packet whole, sha256 ecf4394a… as the brief says;
  the five seats' notes whole; the builder's "Fix pass, round 1"; history.md's 15:32 UTC entry)
- [x] 2 spot-checks  - [x] Q1  - [x] Q2  - [x] Q3  - [x] Q4  - [ ] texts  - [ ] end sections

## Spot-checks (reading, git and sha256 only)
- **The copy is the one round 2 reviewed.** In .claude/worktrees/wf_0fa6595a-3e2-1: HEAD 97bb9d32aa7b…;
  `sha256sum -c` of .gate/u4e/fx-hashes-end.sha256 23 of 23 OK, and of r2-hashes-start.sha256 0 not OK (the two lists
  are byte-equal: `cmp`); `git status --untracked-files=all --short` 24 lines, all under crates/room, crates/server and
  crates/client but the record. wip/u4e.patch in the main folder is 4300944's (sha256 1c31eeee…, equal to
  `git show 4300944:…`).
- **R9's premises hold at ab48790.** `git diff --quiet 97bb9d3 HEAD -- spikes/` true; M2's six crates true;
  `git diff --quiet 28a85e7 97bb9d3 -- spikes/` true. The gate files, .github/ and CLAUDE.md equal origin/main's
  (2aa5e9c); tools/gate.mjs is where M7 names it; main-run.sh and worktree_patch.py are in the notes folder.
  docs/design/ is unchanged since 97bb9d3; docs/adr/ changed only in ADR 0002 and the README (main's merges), and
  marks.json hashes no ADR 0002 row (the marks step passed on bfda3cd: history.md's 15:32 entry, "Done (item 3)").
- **The counted run** (.gate/u4e/r2-te-check-counted.log): the lock held (-x) at 16:15:33 UTC, idle after 0 s; pwd the
  copy, HEAD 97bb9d3, the shared target; both `--no-run` builds exit 0 with Compiling lines naming the copy (npt-room,
  npt-server, npt-client among them); check.mjs's ten lines PASS, "measured build: 473 tests passed, 0 failed, 0
  reported a skip; bites build: 541 tests passed, 0 failed, 0 reported a skip", marks "81 marks read and checked …
  all unchanged", self "52 of 52", redact "0 finding(s)", "check: all 10 steps pass"; end 16:27:02, exit 0. Start to
  end 689 s (round 1's check.mjs alone: 590 s).
- **The code the rulings rest on**, read in the packet's diffs and the copy: the Resync arm (room.rs:716-737: one in
  flight first, then `next.wrapping_sub(f) < FULL_STATE_ACK_TICKS` limited, else answered), `last_full_state` set
  only in `full_state` after a successful encode (:1125), and `None` in the fresh `ClientSlot` every accepted `Join`
  builds; the client's `old_header` and `bad_header` (client/src/lib.rs:503-530; `BAD_HEADERS_HELD` = 32 at :248);
  the lab's `stream_of` (server/tests/common/mod.rs:47); pump.rs:384's pre-`Join` close.
- **The documents NC2-M1 cites:** the catalogue's `Resync` row (docs/design/netcode-proof-messages.md:90) names only
  the one-in-flight rule; ADR 0001's message table proposes for `Resync` "rate limit (proposed: one in flight, since a
  full state is the largest message a client can make the server send)" (ADR 0001:688); its failure table recovers a
  Worker trap with `Resync` within the join mark and a hidden tab with `Resync` "back in step within 0.5 s"
  (ADR 0001:771-772); ADR 0004 part 20 rejoins "through ADR 0001's resync path" (:582); PM-0004-61 injects "20 traps
  per browser at random ticks (proposed)", each resync within the join mark, 20 of 20, against this test's server at
  P2 (ADR 0004:1182), which this test leaves re-runnable for it (plan.md:330-333). This test's own -19 rejoins after a
  trap through a fresh `Join` with a resume secret (plan.md:1100-1101; marks.json -19: "every join: a complete world
  within 300 ms of sending Join"), which today builds a new slot and is not limited.
- **The catalogue's tests** (crates/proto/tests/catalogue.rs, run by check.mjs's test step): a built row needs 8 cells
  and an Authority cell that starts with "Sender:" and names a side; nothing reads the `Resync` row's wording. The refs
  step skips docs/ and every Markdown file (check.mjs:1336-1344).
- **The texts' anchors at ab48790** (line breaks read as spaces): T1's in marks/README.md (sha256 0ad7a80f…), T2's two
  in entry 62 (h) (u4s-carries.md cdab0240…, unchanged), T3's in plan.md (2bef127b…, unchanged since 4d2b0ef; its
  :486 and :508-509 still read as T6 cites them), T4's in SERIES.md (881d7fde…), T6's three in u4-build-plan.md
  (3f58ff0d…), and T7's below in the catalogue: each found exactly once.

## Rulings

### Q1. NC2-M1 (Medium), with SEC-U4E-R2-1 and SEC-U4E-R2-2: a carry (C-U6-16 (a)) and the catalogue row in the merge commit (T7). APPROVE as a finding; not before the merge.
**The finding holds, by reading, at its label (Medium; the netcode seat's, kept).** The limit is built as R3 worded it:
a `Resync` whose answer would come sooner than `FULL_STATE_ACK_TICKS` ticks after the last full state the room sent
that client is dropped, with no reply, and counted `resyncs_limited` (room.rs:716-737). The catalogue's row still names
only the one-in-flight rule (:90), so a client written to it waits for an answer that never comes. R3's premise ("an
honest client … never resyncs twice in 2 s") holds for players at play, not for every honest recovery the drafts
name: a Worker trap or a hidden tab soon after a join (ADR 0001:771-772: the join mark, "back in step within 0.5 s"),
and PM-0004-61's 20 traps per browser at random ticks against this test's server (ADR 0004:1182; plan.md:330-333).

**What my own check adds (it narrows the reach, not the label).** This test's -19 rejoins after a trap through a
fresh `Join` (plan.md:1100-1101), which builds a new `ClientSlot` with no last full state, so -19 is not limited today.
It meets the limit only if U6 recovers by `Resync`, or if U6 fixes SEC-U4E-R2-1 by carrying the clock across a re-join.
So NC2-M1 and SEC-U4E-R2-1 are one design question: the bound on the full states a client can draw has to hold across
a re-join, and an honest recovery has to be answered at once wherever it falls. One carry takes both.

**Why a carry, and why not now:**
- R8's outcome rule: a Medium becomes a follow-up or a carry, never a third round. No U4 mark meets the limit (the
  endpoint sends no `Resync`; -22's forced resync is not limited), and the limit closes the attack round 1 measured
  (Q-6: 29 full states in 30 ticks, about 8 times the window).
- A carry, not a card follow-up: it binds U6's form and the outcome of -19's runs and of the server this test leaves
  for PM-0004-61, so it must sit where U6's brief reads it: plan.md's U6 row, as C-U6-16 (a) (T3).
- Not the injector's spacing as the route: it would bend two marks' methods (PM-0004-61's "at random ticks
  (proposed)", the boundary spike's; -19's traps) to fit the build. The build's form changes instead; if U6 finds no
  form that meets both the marks and the bound, the spacing comes to me first as a plan §15 reading.
- The client's resend belongs to the browser client, U6's; it goes in the same carry.

**The catalogue row: in the merge commit (T7), which closes SEC-U4E-R2-2.** The rule `.claude/rules/netcode.md:17`
("Every protocol change updates the message catalogue in docs/design/ and its tests") covers a change in how the room
answers a message, and U4a's and U4d's merge commits carried their catalogue edits the same way (952f734, 39f4290). The
Low rides the push the merge needs anyway, so Mustafa's rule on Lows holds. T7 keeps the cell's audit form (it starts
"Sender:", names a side, adds no "|") and names no path (the refs step skips docs/ anyway).

**For ADR 0001.** Its message table proposes "one in flight" as `Resync`'s rate limit (ADR 0001:688); round 1 showed
that a client that acknowledges each full state draws 29 in 30 ticks under it, so the proof test adds the time limit.
That is a finding of the proof test for the draft, recorded in U4e's report (T5) and so in PR #40's: the drafts are
Proposed (docs/adr/README.md), no word or decision of Mustafa's changes, and nothing is decided here.

**SEC-U4E-R2-1 (Low, kept):** C-U6-16 (a), with NC2-M1. Open until then, so it joins U4e's open security findings
(T5, M9).

Q1 written 2026-11-12.

### Q2. The twelve Lows: labels kept; eight go into carry texts, one into T1, one into T7, and two become card follow-ups. APPROVE.
The rule I apply: a Low goes into a carry when a named later unit edits that code and its brief must hold the point;
into a text when the merge commit applies that text anyway; else it is a follow-up on the card with its file:line. No
Low pushes on its own: every text below is copied in the merge commit, which is needed anyway. Lines are the copy's.
- **NC2-L1 with TE2-L1 (the client's two too-old cases pinned only by what the judge counts): carry 80 (d), U4f.** The
  test seat's plant P-form (case 2 folded into case 3, a ring slot taken, the bad tick acknowledged) passes every test
  (r2-te-pform.log), and the acknowledgement even reached the room. R1's case 2 is the half that keeps a correct build
  meeting one very late group (U6's real network) from ending INVALID or `crash`, so it needs a guard before U6. U4f is
  the next unit to edit npt-client, and carry 80 (b) already sends it to the same path (`receive`, `see`). One endpoint
  test, both halves, each failing under its plant. Not U4r: no U4r outcome depends on the form (honest L1 runs never
  reach either case; a bitten run fails either way, as the twin shows).
- **RR2-2 (carry 61 (b)'s "two different hashes for one tick give the sentinel" fails in one order): carry 80 (e),
  U4f.** A snapshot, then a full state of another hash for that tick, files nothing (`full_state` counts it a
  duplicate: client/src/lib.rs:782-785; the rust seat's probe B), and `file`'s different-hash arm is unreachable (P-H's
  first edit alone passes every test: r2-te-p13c-ph1.log). U4e's room never sends both for one tick (room.rs:1083-1089),
  so no U4 run meets it, but the report must not list carry 61 (b) as met without that limit: T5 states it.
- **RR2-3 (case 3's "once" holds while the client remembers the last 32 bad-header ticks): T1 made exact.** T1 is the
  contract both sides read, and the merge commit applies it anyway; its "once" becomes "once while the client
  remembers that tick (the last `npt_sim::ACK_RING_TICKS` such ticks)". Carry 73 needs no line: README §1 already has
  the writer write one hash met twice once (marks/README.md:60-61), which covers a repeated sentinel.
- **SEC-U4E-R2-2 (the catalogue's `Resync` row): T7, in the merge commit** (Q1). Closed there.
- **SEC-U4E-R2-3 (the limit's tick count is the 30 Hz constant: room.rs:728): C-U6-16 (b),** beside RR-6's card
  follow-up, which gains this site. Open until then.
- **SEC-U4E-R2-4 with A-R2-1 (`take_control`'s bytes carry no channel; the lab's `stream_of` probe,
  server/tests/common/mod.rs:47, holds only while a full state's length stays below 262,144 B): C-U6-16 (c),** since
  U6's TickLoop is the production caller that must not copy the probe; R5's A9 card follow-up (the next npt-proto
  change) gains both, and may take them first. SEC-U4E-R2-4 stays open until then.
- **TE2-L2 (SEC-U4E-1's wrong-direction half has no test: pump.rs:384; plant P-13a' passes): C-U6-16 (d),** with the
  first change to npt-server's pre-`Join` path. The code is right by reading, and the 2 s `Join` deadline bounds the
  case meanwhile. SEC-U4E-1 counts as fixed; T5 names its open test.
- **A-R2-2 (`send_control` drops an encode error unseen, room.rs:327-332): a card follow-up.** Unreachable today
  (`JoinAccepted` and `Kick` are fixed-width and under their limits); the next npt-room edit adds a `debug_assert!` or
  the full state's `count_refusal` there.
- **RR2-1 (carry 50's test's "none at 256 or older" assertion cannot fail alone, room/tests/room.rs:627): a card
  follow-up.** The edge is guarded by the room's `u8::try_from` and debug assert, which the test reaches by name (P-G,
  P-G+); the next npt-room test edit matches the event by its number alone and asserts no snapshot after tick at + 255
  carries it.
- **The two new public fields** (`ClientCounters::resyncs_limited`, `Counters::joining_dropped`), flagged by the builder
  and three seats: inside R3's and R5's words ("counted under its own name"), within R8's bounds, and covered by T6's
  rows ("its fields the per-client counts"; "`Counters`' fields"). T6 does not change.
- Not findings, no route: the netcode seat's note that pump.rs:373-374's comment says "on a joined session" while a
  Joining session also counts and goes on (wording); the security seat's note that `first` may come from a refused
  datagram's tick (the same tick as the first snapshot in every honest run; README §1's own term is "a client's first
  snapshot", :67).

Q2 written 2026-11-12.

### Q3. The merge: round 2's outcome lets it go ahead on M1-M9, with the changes below. APPROVE.
Round 2's outcome under R8: five APPROVE WITH FINDINGS, one Medium ruled a carry (Q1), Lows only otherwise, no Critical
or High. The merge goes ahead. Round 2 ran in three batches of at most two seats, not R8's two batches, because my U4d2
brief check's L5 made room for U4d2's builder: the same five seats on the same head, so one round. No third round.

**The conditions** (each as R9 wrote it, unless said; their results in u4e-build.md's "Merge into the spike branch"):
- **M1 stands.** Checked now: HEAD 97bb9d3, the 23 files and the record, r2-hashes-start.sha256 23 of 23 OK (the same
  list as fx-hashes-end.sha256).
- **M2 stands.** True at ab48790 (main's merge bfda3cd moved nothing under spikes/). If U4d2 merges first, its spikes/
  change is the named lane's, listed with its counts.
- **M3 stands**, with this precision: the committed wip/u4e.patch that round 2 reviewed is 4300944's (sha256
  1c31eeee…).
- **M4 stands.**
- **M5 changes in three places.** (1) "First T1 (marks/README.md §1) and T4 (SERIES.md)" becomes "First T1
  (marks/README.md §1), T4 (SERIES.md) and T7 (the catalogue's `Resync` row, which npt-proto's catalogue test reads)",
  with T1's and T7's words from this ruling. (2) The counts: all 10 steps pass, measured 473, bites 541, self 52 of 52
  (the fix pass's and round 2's counted runs; plus U4d2's deltas if it merged first, the sum shown), marks 81 read and
  checked with the ADR rows unchanged, redact 0 findings. (3) e7's print line equal to the fix pass's
  (.gate/u4e/fx-t5print.log), which round 2 reviewed. The rest stands: main-run.sh with the lane lock exclusive and
  idle at the start, the touch loop, the shared target, offline; the two `--no-run` builds first with Compiling lines
  naming the main folder; the by-name lines for T5. The run may go in the background with a limit of at least
  1,500,000 ms (Q4).
- **M6 stands.**
- **M7 stands.** The gate files are main's since bfda3cd (the 2.1b layout, legacy/); the command is the same, with the
  `--no-smoke` fallback and both logs kept if this container's Chromium cannot start.
- **M8 changes in two places.** (1) The commit also carries docs/design/netcode-proof-messages.md with T7 alone (its
  only docs/ change; the spikes/ part stays the reviewed bytes plus T1 and T4 and nothing else). (2) Where each text
  comes from: T1, T2's block (3) (entry 80), T3, T5 and T7 from this file's code blocks, which replace advisor-u4e-r1.md's
  T1, T2 (3), T3 and T5; T2's blocks (1), (2), (4) and (5), T4 and T6 from advisor-u4e-r1.md's. Copied by script, never
  retyped, each old text found exactly once (line breaks read as spaces), each new text found in place after, no
  changed prose line over 120 characters (a table line, as T3's and T7's, is not prose).
- **M9 changes in three places.** (1) The carries listed: 73-81 and C-U6-12 to C-U6-16. (2) U4e's open security
  findings for PR #40's report: SEC-U4E-2, SEC-U4E-4's transport half (Medium), SEC-U4E-5, SEC-U4E-6, SEC-U4E-8,
  SEC-U4E-9, SEC-U4E-R2-1, SEC-U4E-R2-3 and SEC-U4E-R2-4, each Low but SEC-U4E-4's half, each with its carry. (3) The
  card's follow-ups gain A-R2-2 (room.rs:327-332) and RR2-1 (room/tests/room.rs:627); RR-6's gains the limit's site
  (room.rs:728, SEC-U4E-R2-3); A9's gains SEC-U4E-R2-4 and A-R2-1 (server/tests/common/mod.rs:47). The rest stands,
  U4r's brief naming carries 73-77 and 62 (h)'s measured window among it.

**The texts.** T1 changes (Q2, RR2-3). T2 changes in block (3) only, entry 80 (Q2). T3 changes: C-U6-16 is added after
C-U6-15 (Q1, Q2). T5 changes: round 2's outcome, the Resync limit, round 2's Lows, carry 61 (b)'s limit, the two
public fields and the open security findings. T4 and T6 stand: T4's anchor holds, and T6's rows match the final diff
(the engine-architect's check, u4e-review-r2-arch.md, "T6's §4.1 rows against the final diff"), the two new public
fields inside them (Q2). T7 is new: the catalogue's `Resync` row. The new words are below, in full.

Q3 written 2026-11-12.

### Q4. The test seat's counted run, in the background with a 1,500,000 ms limit: ACCEPTED.
The limit is how long the tool waited for the command, not part of the evidence: the runner (u4e-run.sh), the lock
(held exclusively at 16:15:33, idle after 0 s), the command (the two `--no-run` builds, then check.mjs with `-x`) and
the copy were the brief's, and the log's own lines carry the result. The run took 689 s from the lock to the end
(check.mjs alone 579 s), so a 600,000 ms wait would have cut it mid-run and left no counted result. The seat named the
change and its reason (its round-1 run took 590 s) in its notes beside the result. The same form serves M5's run on
the merged tree.

Q4 written 2026-11-12.

## The texts that change or are new (each copied by script from its code block; never retyped)
These replace advisor-u4e-r1.md's T1, T2 block (3), T3 and T5; T7 is new. T2's blocks (1), (2), (4) and (5), T4 and T6
stand as advisor-u4e-r1.md gives them.

### T1. marks/README.md §1, the `decoded_hash` item (the merge commit)
The old text `that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel.` (found once, line breaks
read as spaces; checked at ab48790) becomes the text below, rewrapped to the item's indent within 120. Against round
1's T1, only the words after "files the sentinel under that tick" up to "never acknowledged" and the cite change
(RR2-3):
```
that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel. That covers the ticks from the client's first snapshot on: a snapshot group or full state whose header names a tick before the client's first snapshot and at least `npt_sim::ACK_RING_TICKS` ticks behind the newest tick it holds names a tick it was never sent (a bad header), so the client files the sentinel under that tick once while it remembers that tick (the last `npt_sim::ACK_RING_TICKS` such ticks; a repeat past them is one hash met twice, written once), never acknowledged, and `decoded_hash_mismatch_count` counts it as a client hash at a tick the server never logged (the advisor's U4e round-1 ruling, R1, and round-2 ruling, Q2).
```

### T2, block (3). u4s-carries.md §U4f, after entry 46 (its last) (the merge commit)
Against round 1's block (3): the heading's sources gain round 2's, and (d) and (e) are added; (a) to (c) are unchanged.
```
80. **U4e's notes for option A and the client (round 1: u4e-review-rust.md, RR-7; u4e-review-security.md, SEC-U4E-9;
    u4e-review-test.md, TE-9; u4e-review-netcode.md, NC-L7 and NC-L8; the advisor's U4e round-1 ruling, R5; round 2:
    u4e-review-r2-netcode.md, NC2-L1; u4e-review-r2-test.md, TE2-L1; u4e-review-r2-rust.md, RR2-2; the advisor's U4e
    round-2 ruling, Q2):** "(a) The room's canonical order (`canonical`, crates/room/src/room.rs) compares one slot's
    ticks wrap-aware, which is no total order for ticks 2^31 or more apart, and Rust's sorts may panic on a non-total
    order; under option B one command per slot per tick makes it unreachable, and carry 21's room test passes with no
    sort at all (TE-9). Before option A runs several commands of one slot in a tick, U4f's lead bound refuses such a
    command first, or the sort takes a total key, with a test of one slot's commands 2^31 ticks apart, and carry 21's
    room test is made to fail without the sort. (b) A refused snapshot datagram whose header tick reads far ahead moves
    the client's newest tick and resolves the whole span lost (crates/client/src/lib.rs, `receive` and `see`): the
    client files its sentinel without moving its newest tick past a bound, with a test. (c) `ClientEndpoint::step`
    drops a whole `Input`, acks included and uncounted, when one command fails to encode: it drops that command,
    counts it and keeps the acks, with a test. (d) The client's two too-old cases (marks/README.md §1's bad-header
    sentence; the advisor's U4e round-1 ruling, R1) are pinned by no test beyond what the judge counts: the round-2
    test seat's plant P-form (a too-old tick from the first snapshot on given the sentinel too, a ring slot taken, the
    bad tick acknowledged) passed every test. U4f adds one endpoint test: joined at tick 100 and fed to about 140, a
    group of tick 60 (before the first snapshot: a bad header) fed twice files `(60, 0)` once, the next `Input`
    acknowledges no tick 60, `newest_tick()` is unchanged, `refused()` rose by 2 and a delta on a live baseline still
    decodes; then a group of tick 105 (from the first snapshot on, already out of the ring) files nothing and adds 1 to
    `stale()`, never the sentinel; each half fails under its own plant. (e) Carry 61 (b)'s 'two different hashes for
    one tick give the sentinel' fails in one order: a tick completed by a snapshot and then sent as a full state of
    another hash files nothing (`full_state` counts it a duplicate), and `file`'s different-hash arm has no route from
    the endpoint's input; U4e's room sends a client a full state or a snapshot for a tick, never both, so no U4 run
    meets it. For a tick already received, `full_state` compares the frame's hash with the one filed and files the
    sentinel when they differ, with a test of that order that fails under its plant."
```

### T3. plan.md's U6 row: U4e's U6 carries (the merge commit)
The old text `since -19 judges ML's Chrome "as it stands" (ADR 0001:1609) |` (found once: the row's end; checked at
ab48790) becomes the text below (one table line; `<br>•` as the row's other entries). Against round 1's T3, only two
places change: the heading's "(the advisor's U4e round-1 ruling, 2026-11-12)" now names both rulings, and C-U6-16 is
added after C-U6-15, before the row's closing bar; C-U6-12 to C-U6-15 are unchanged.
```
since -19 judges ML's Chrome "as it stands" (ADR 0001:1609);<br>**U4e's carries (the advisor's U4e round-1 and round-2 rulings, 2026-11-12):**<br>• **C-U6-12 the full-state stream and the control stream (R3 and R5: SEC-U4E-4's transport half, SEC-U4E-2):** at most one open full-state stream per session (the catalogue's FullState row, proposed there), npt-server refusing a client-opened one beyond it; and a per-connection limit on control-stream messages (`ClientSettings`, `Resync`, `SessionControl`), counted, a message over it dropped or the session closed as malformed, with the catalogue's rows gaining the limit; the room's half of SEC-U4E-4 is U4e's (a client's Resync answered at most once per `FULL_STATE_ACK_TICKS` ticks);<br>• **C-U6-13 the event base on the wire (R3: NC-M5, A3):** a join or rejoin starts the client's 32-bit event numbers from the room's: `JoinAccepted` or `FullState` carries the room's next event number (npt-proto and the catalogue), the client rebuilding from it; -19's join hash (C-U6-11) is the first judged case, since today a client admitted after the room has stamped 32,768 or more events past its start rebuilds every event number wrong (U4e's probe Q-10; u4s-carries.md entry 75 is U4r's stopgap);<br>• **C-U6-14 ending a session (R3: A6, SEC-U4E-5):** TickLoop, when `Room::join` reports a replacement, sends the old connection `Kick(replaced)` and closes it before routing anything else, and closes a kicked or left session; either the room checks a session generation on `receive` and `leave` (returned by `Room::join`, a stale one dropped and counted) or U6's report shows by test that no message of a closed session reaches the room; npt-server's library gains a call that ends a session only if TickLoop needs one, with its reason;<br>• **C-U6-15 the room's tick before -4 (R3: RR-5, with RR-4, A10 and A12):** before PM-0001-4's judged runs, the room's own tick at SS with 48 clients is measured on R2 by the rust-reviewer's U4e method (a counting allocator around `Room::tick` only; a twin World stepped on `tick_input()` with its state hash asserted equal; 100 warm and 600 measured ticks; release; three runs) and set beside -4's 16.67 ms p99 with the pump and the logs; U4e's cheap wins are made first, each with that bench before and after: the relevant set emitted in the index's key order instead of a sort by key through the records, no clone of the Packer's vectors per group, no fresh baseline-frame clone per client per tick, no per-client vectors built each tick, no double clone of a tick's baseline frame in the client, a linear acknowledgement pass over the event queue, and no per-tick copy of the sim's reports (`Room::reports`); at U4e, on a 4-vCPU VM, the room's own work was 5.4-5.9 ms mean and 9.5-11.2 ms p99, with 2,838 allocations and 6.2 MB requested a tick, linear in clients (u4e-review-rust.md, block 2);<br>• **C-U6-16 the Resync limit and U4e's other round-2 notes (round 2's Q1 and Q2: NC2-M1, SEC-U4E-R2-1, -3 and -4, A-R2-1, TE2-L2):** (a) U4e's room answers a client's `Resync` at most once per `FULL_STATE_ACK_TICKS` ticks after the last full state it sent that client (its join's, a forced one or an answered one), drops a sooner one with no reply (`resyncs_limited`), and keeps that clock in the client's slot, which every accepted `Join` rebuilds; so an honest `Resync` soon after a full state goes unanswered (a Worker trap or a hidden tab soon after a join: ADR 0001:771-772; PM-0004-61's 20 traps per browser at random ticks against this test's server: ADR 0004:1182 and §2's "Left to the boundary spike"), while a replacement or a re-join draws a fresh full state at once (SEC-U4E-R2-1). Before -19's runs and before the server is left re-runnable at P2 for PM-0004-61's online half, U6 gives the limit a form, chosen with the advisor, that answers an honest client's recovery at once wherever it falls and still bounds the full states one slot can draw, by `Resync` and by re-join together: for example a bucket allowing one full state beyond the last per window, kept outside `ClientSlot` (in the roster entry), or the other option the advisor's U4e round-1 ruling weighed (R3), every full state counted against the client's downstream window; its tests show a `Resync` one tick after the join's full state answered at the next tick, and N replacements or `Resync`s in N ticks drawing no more full states than the bound. The browser client's resync path sends its `Resync` again when no `FullState` has come within `FULL_STATE_ACK_TICKS` ticks (the catalogue's `Resync` row states the limit since U4e's merge). Spacing the trap injector instead would read -19's and PM-0004-61's rows: it comes to the advisor first, as a §15 reading; (b) the limit's tick count is taken at the room's rate (a `full_state_ack_ticks_at` form, carry 39's route) when a room first runs at a rate other than 30 Hz (SEC-U4E-R2-3; the card's RR-6 follow-up names the room's other 30 Hz sites); (c) TickLoop sends `Room::take_control`'s bytes on a channel that npt-proto or the room gives it (a public read of a framed message's channel, or each message's `Limits::TYPES` row beside its bytes), never by a copy of the test lab's `stream_of`, which finds the type byte at the offset of the channel it is trying to learn and holds only while a full state's length stays below 262,144 B (SEC-U4E-R2-4, A-R2-1; the card's A9 follow-up, at the next npt-proto change, may take it first); (d) the pump's SEC-U4E-1 test gains its wrong-direction case (a room-to-client type framed first on the control stream: the session closed, `first_not_join` 1, `Kick(malformed)` sent, the `Join` behind it never taken), with the first change to npt-server's pre-`Join` path (TE2-L2) |
```

### T7. docs/design/netcode-proof-messages.md, the `Resync` row's Authority cell (the merge commit; new)
The old text `is dropped and counted (row 673, proposed there; plan §4.4); answered with `FullState`` (found once, in
the `Resync` row, :90 at ab48790) becomes the text below, in place, on the row's one line (no "|" added; the cell still
starts "Sender:" and names a side, as npt-proto's catalogue test asks):
```
is dropped and counted (row 673, proposed there; plan §4.4); and at most one is answered per `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state the room sent that client (its join's, a forced one or an answered one): a sooner one is dropped with no reply and counted (`resyncs_limited`; built in U4e for SEC-U4E-4, since an acknowledgement is the client's own claim and never frees a full state by itself), so a client whose `Resync` brings no `FullState` within that span sends it again (the limit's form is revisited before -19's runs, so that an honest recovery is answered at once); answered with `FullState`
```
