# BRAWL-1 build review, round 1: engine-architect seat

Snapshot under review: wip/brawl1.patch, sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified).
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified, 2414 lines).
Read copy: .claude/worktrees/brawl1 at 352a245 + the change uncommitted (10 files modified: bots/lib.rs, client/lib.rs,
client/tests/endpoint.rs, lab det-recordings.json, precheck-recordings.json, lab/src/hash.rs, logs.rs, precheck.rs,
scenario.rs, marks/SERIES.md).
No memory directory for this seat (.claude/agent-memory-local has security-auditor only). Fresh start.
No Edit tool in this run: this file is rewritten whole with Write at each update.

## Plan
1. Read the packet: approved plan sections for BRAWL-1, conditions 1-18, rulings copied (esp. condition 5, 11,
   plan.md 4.2, EA-U4R-1, EA-U4R-2).
2. Read the patch whole.
3. Check in the read copy: npt-client pub items (new or widened) vs condition 5 / plan 4.2; the wake step form
   (EA-U4R-1); the Sim alias and shared constructor (EA-U4R-2); npt-bots library names no World/SimCore, gains no pub
   item, no new dependency (Cargo.toml / Cargo.lock untouched?).
4. Wider: who owns the cluster-centre state; data direction (bot steers from decoded position only: no server truth
   leak); lab items' boundaries; dead code / features no requirement needs.
5. Run, if useful, via u4r-run.sh: cargo check/clippy/tests of npt-bots and npt-client; logs to .gate/u4r/brawl1-review-arch/.
6. Findings, one line each below, then the result.

## Progress
- 1. Packet read whole (2414 lines): diffs of all 10 files, plan 1-7 and 11-12, conditions 1-18, sets ruling BRAWL-1,
  r2 next steps, TE-R2-1/2, N2-1, RR4-L1/L2, EA-U4R-1/2/5, RR3-L2, build ruling 1 points 1 and 7, plan.md 4.2.
- 2-4. Code read in the read copy. Checked and holding (no finding):
  - npt-client: one new pub item, `ClientEndpoint::own_soldier(&self) -> Option<([i32; 3], bool)>` (client/src/lib.rs:350-369).
    Condition 5's form met: &self, by value, no float, no ring reference, no new dependency (client Cargo.toml
    unchanged), doc says client's own decoded state; filter used && complete() && Filed::Hash, reduce by
    Wrap::after32("tick", ..) (Wrap is Copy, after32 pure: proto/src/wrap.rs:60-65; "tick" is the endpoint's own name
    for its 32-bit compares, :526, :588 etc.). frame.own is sorted by key in both writers (apply :103-136, full_state
    :822-823), so the binary search is valid. The room's own section holds (Soldier, owner) (room/src/room.rs:1076).
    plan.md 4.7 lists npt-client's public surface by type (ClientEndpoint, ...): a method on ClientEndpoint stays
    inside it; the anonymous tuple avoids a new pub type. Not exported through npt-client-wasm.
  - npt-bots: no new pub item (next_command, leash, toward_centre private; steering a private field);
    World/SimCore named only in the #[cfg(test)] module (bots/src/lib.rs:322-326, :474-475, :497); library use of
    npt-sim stays Command and Scene (:28); Cargo.toml and Cargo.lock untouched (git diff --stat empty). Leash state
    owned per client in Behaviour. own_soldier read after the endpoint is fed (:215, :223).
  - EA-U4R-1: wake() (scenario.rs:591-623) is the narrow form: accept, io, admitted and ended taken, due count; each
    caller keeps its own body and sinks (run: logs.admit/gone; run_leg: maps and o.ended). The reorder (due computed
    before the admitted loop) is neutral: due_ticks is pure (server/src/schedule.rs:149-). run_leg uses SimClock
    (advance_to asserts forward; next.max(now+1) > now). Part 1's precheck tests output: p1-before-a.log and
    p1-after-e2.log equal once time lines are dropped (my diff, empty).
  - EA-U4R-2: `pub(crate) type Sim = World` (scenario.rs:51); World remains only in hash.rs's bite list and dump
    checks (documented) and the printed "World::new(scene, ..)" strings (precheck.rs:677, :691). Server::build
    (scenario.rs:373-394) used by run, run_leg and lab_room; with_event_start(0) equals the room's default
    (room.rs:459, :488-491); Bites::default's world/room/quant are identity (scenario.rs:258-285); RttSource::Link both.
  - Counts: final-check.log all 11 steps pass, 535/613 (= 531/609 + 4: bots 2 non-ignored, client 1, logs 1).
  - summary_line's appended "in a building" sits at the end; no script parses the tail (grep of ops/, check.mjs).
- No run needed for my findings (all are read from code and logs); nothing run under .gate/u4r/brawl1-review-arch/.

## Findings (one line each as found)
- EA-B1-1 (Low): client/src/lib.rs:245-248, :368 copy npt-sim's private SOLDIER_FLAGS (world.rs:92) and FLAG_DEAD
  (world.rs:120); no test ties them to npt-sim (endpoint.rs:408 sets the bit by the same literal; bots/src/lib.rs:479
  a third copy). Fix: follow-up at the next npt-sim change: pub consts or EntityRecord accessor; or now a client unit
  test using p6's focus-fire setup (logs.rs:784-800) asserting a SimEvent::Kill victim reads dead by the client's consts.
- EA-B1-2 (Low): scenario.rs:397 Server::new stays pub(crate) though Server::build (:390) is its only caller: a second
  in-crate route around EA-U4R-2's one constructor. Fix: drop pub(crate) (no other reference: grep).
- EA-B1-3 (Low): client/src/lib.rs:344-349 doc over-claims "a newer tick that is incomplete ... changes nothing": a
  newer tick taking the ring slot (slot_for :564-574, t % ACK_RING_TICKS = 32) drops the complete tick; and the wrap
  rule (:357) is unproven: endpoint.rs:397-443 uses ticks 100-106 (max_by_key would pass). Fix: doc words + one case
  across u32::MAX.
- EA-B1-4 (Low): logs.rs:6-7 "Every tick and second record carries all its counters, 0 included" now has an unnamed
  exception: in_building_slot_area only on ticks with one (logs.rs:586-598; SERIES.md:368). Fix: doc words.
- EA-B1-5 (Low): hash.rs:239 MAX_ENTRY_TICK_COUNT vs RR4-L1's ruled MAX_ENTRY_TICKS, renamed to pass check.mjs
  TICK_NAMED (check.mjs:760); reason holds; a deviation from ruled words the report must name; clean route is a
  NOT_TICK_COUNTS entry (check.mjs:761-765) in the check.mjs follow-up.

## Verdict (draft)
APPROVE WITH FINDINGS: condition 11's part (the npt-client pub item vs condition 5 and plan 4.2/4.7; EA-U4R-1's narrow
form; EA-U4R-2's alias and constructor; npt-bots' boundaries) holds; five Low findings, none blocking.
