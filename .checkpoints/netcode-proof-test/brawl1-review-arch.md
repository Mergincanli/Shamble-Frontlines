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
- Leads to check in code: (a) own_soldier: ring/filed/complete semantics, wrap.after32 side effects, frame.own's
  type and sort, SOLDIER_FLAGS/SOLDIER_DEAD duplicated from npt-sim privates (drift unpinned?); (b) plan.md 4.7's
  npt-client list vs the new pub item; (c) wake(): order change neutral? SimClock semantics in run_leg; (d)
  Server::build: with_event_start(0) identity? Bites::default quant/room identity; (e) npt-bots: World/SimCore only
  in cfg(test); no pub; Cargo.toml untouched; (f) leash on stale own position (constant heading); twin test and the
  link-fate independence build ruling 1 point 7 relied on; (g) logged() on no-log runs; r.index == slot.
- 2. Reading npt-client (client/src/lib.rs): ring = ACK_RING_TICKS entries; Filed enum :140; Tick :149; complete :162.

## Findings (one line each as found)
