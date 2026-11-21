# C62H-BOUND (a): the exact wording and the planned size, for your yes

Assembled by script (c62h_bound_paste.py) from c62h-bound-texts-v3.md (sha256 3f21c9b7e5d5116d…) and the advisor's ruling (advisor-c62h-bound-r2-full.json). The texts below are word for word; nothing is retyped. Nothing is built before your yes.

## What the limit does

On the lossy test networks (P2, P3 and B1), every snapshot a test client files as "lost" in each run's measured ten minutes (not its warm-up or its last few seconds) now needs a reason. The reason has to come from a record the client does not write: either the server's own log says it sent that client nothing at that moment, or the simulated network's own log says it dropped at least one packet of that snapshot. Any other "lost" snapshot counts against the client, and the mark needs zero of them on every run. A second check runs the other way: a snapshot the network dropped a packet of can never be filed as received whole. That proves the network's record does not claim losses that never happened. "It arrived too late" is not on the list, because on the simulated network nothing can arrive that late (the slowest packet takes about a fifth of a second, and the client keeps about one second of snapshots). Two planted bugs prove the checks work: a planted bug in the lab's per-client log writer that files one snapshot per client, received whole, as lost, fails the first check (48 of 48 clients), and a network record that invents a loss fails the second.

## The exact texts (fixed at your yes)

**marks.json, PM-0001-17: two checks, after its "L1 P0 loses no group" check**

```json
        {
          "name": "L1 P2, P3 and B1: every measured client-tick left incomplete is one the room sent that client nothing for, or one the link lost a snapshot datagram of (0 unexplained)",
          "figure": "snap_lost_unexplained_count",
          "op": "==",
          "value": 0,
          "parts": [
            "L1 P2",
            "L1 P3",
            "L1 B1"
          ]
        },
        {
          "name": "L1 P2, P3 and B1: no measured client-tick filed whole is one the link lost a snapshot datagram of (0; the link's record is not overstated)",
          "figure": "snap_whole_despite_link_loss_count",
          "op": "==",
          "value": 0,
          "parts": [
            "L1 P2",
            "L1 P3",
            "L1 B1"
          ]
        }
```

**marks.json, PM-0001-17: one planted bug, after whole-as-lost**

```json
        {
          "id": "PM-0001-17/link-lost-overstated",
          "what": "a lab record of the link's losses that counts a delivered snapshot datagram as lost: on each client, its first measured tick whose datagrams all arrived"
        }
```

**SERIES.md: two new figure rows, one new series row, and the changed `tick.s.snap_incomplete` row** (and the two header counts: Figures 36 to 38; series read only by figures 18 to 19)

```
| `snap_lost_unexplained_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with `tick.s.snap_incomplete` 1 whose room-log record at that tick gives that slot a non-null `hs.server_hash` (the room sent it a snapshot or a full state) and `tick.s.snap_link_lost_count` 0 (the link lost none of the snapshot datagrams the room sent it then). Explained, so not counted: `hs.server_hash` null (the room sent that client nothing: a full state not yet acknowledged, budget short, an encoder refusal, a kick) or `tick.s.snap_link_lost_count` 1 or more (npt-l1's fate log lost one by its loss draw or in a burst). Nothing else explains a tick filed lost: not a stale drop (L1 delivers every snapshot datagram well inside the 32-tick ring), not a full-state tick, not a tick whose datagrams all arrived. 0 incomplete is a real 0. Gaps: no room log gives `tick.s.snap_link_lost_count`, a room tick record without it, a room tick record without `hs.server_hash`, a list that does not hold the run's `clients` entries, an entry below 0, a measured client tick record without `tick.s.snap_incomplete`, a `snap_incomplete` entry other than 0 or 1, a measured incomplete tick the room log has no record of. Inputs: `tick.s.snap_incomplete`, `hs.server_hash`, `tick.s.snap_link_lost_count` | PM-0001-17 '100 % of decoded snapshots': no decoded snapshot escapes the comparison by being filed lost (C62H-BOUND (a), 2026-11-15) | -17 |
```

```
| `snap_whole_despite_link_loss_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with a whole `hs.decoded_hash` (not null, not the sentinel 0) whose room-log record at that tick gives that slot `tick.s.snap_link_lost_count` 1 or more. No client can complete such a tick, so each one counted is a record that overstates the link's loss. A client hash at a tick the room log has no record of is not read here (`decoded_hash_mismatch_count` counts it). Gaps as `snap_lost_unexplained_count`'s. Inputs: `hs.decoded_hash`, `tick.s.snap_link_lost_count` | PM-0001-17, the link record's own check (C62H-BOUND (a), 2026-11-15) | -17 |
```

```
| `tick.s.snap_link_lost_count` | count | npt-lab (from npt-l1's fate log; L1 only) | the room log, one per client slot on every room tick record (the run's `clients` entries, 0 included): of the snapshot datagrams (groups) the room sent that client at this tick, how many npt-l1's fate log records as lost by its loss draw or lost in a burst; read at the send, from the fate-log datagram entries that slot's send pushed, which must be exactly its groups (else the run fails) | snap_lost_unexplained_count, snap_whole_despite_link_loss_count |
```

```
| `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot (the run's `clients` entries, each 0 or 1), at the tick it resolves: 1 when a group of that tick was lost and none refused, or the client dropped one as stale (README §1's `decoded_hash`), so the client logs no hash for it (§4.6), a tick none of whose groups arrived included (known from a later header's tick); with `hs.decoded_hash` it decides README §1's coverage rule (`decoded_hash_mismatch_count`); on L1 P2, P3 and B1 each measured 1 must be explained by the room's or the link's record (`snap_lost_unexplained_count`) | -17 (the coverage rule, its L1 P0 check and its report; snap_lost_unexplained_count), -58, -19 (the coverage rule) |
```

**README.md §1, one sentence in the `decoded_hash` paragraph**

```
On L1's P2, P3 and B1 a measured tick filed lost must have a reason in the room's or the link's record, never the client's own: the room sent that client nothing then (`hs.server_hash` `null`), or npt-l1 lost a snapshot datagram of it (`tick.s.snap_link_lost_count`, the room log); a stale drop is no reason there, since L1 delivers every snapshot datagram well inside the ring (PM-0001-17's `snap_lost_unexplained_count`; `snap_whole_despite_link_loss_count` checks the record the other way).
```

## The planned size

| File | Lines | What |
|---|---|---|
| spikes/netcode-proof/crates/l1/src/lib.rs | +25 to +32 | DatagramCounts and FateLog::server_datagrams_since, with docs. Read-only: no draw, and no change to the fate log's canonical form or hash. |
| spikes/netcode-proof/crates/l1/src/tests.rs | +30 to +40 | One test: a forced datagram loss and a B1 burst are counted; Ack and Stream entries and client-to-server datagrams in the same window are not. |
| spikes/netcode-proof/crates/lab/src/scenario.rs (code) | +70 to +95 | the per-slot read as one pure join function (TE-CB-1; C7: groups with no session is an Err) and its call at each send; the per-slot list into room_tick; the measured totals and the summary phrase with TE-CB-2's three counts; the Bites field, LAB_BITES to 3 ids and the link-lost-overstated plant; the module text. |
| spikes/netcode-proof/crates/lab/src/scenario.rs (tests) | +115 to +170 | The short P2 run (16 clients, 6 s warm-up, 4 s measured) asserts: 16 entries per room record; a link-lost sum above 0; a test-side join of 0 and 0; E1 + E2 + N = incomplete; N equals the join. whole-as-lost gives 16 unexplained; link-lost-overstated gives 16 whole-despite-loss. TE-CB-1's five join tests. |
| spikes/netcode-proof/crates/lab/src/logs.rs | +25 to +35 | room_tick's link_lost parameter and the series; the per-(tick, slot) link-loss kept beside `logged` and the three measured counts at the client write (TE-CB-2); the module text; the two existing test call sites. |
| spikes/netcode-proof/crates/judge/src/figures.rs | +95 to +120 | One shared room-side reader with one gap list (C2's and TE-CB-5's gaps included), the two figures and two FIGURES entries. |
| spikes/netcode-proof/crates/judge/tests/engine.rs | +135 to +195 | Fixtures: each explanation alone; an unexplained tick; a warm-up tick and an after-phase tick counted on neither figure (TE-CB-6); the converse; the sentinel; seven gaps, each read on both figures: the series missing, a room tick without it, a room tick without hs.server_hash (C2), a wrong-length list, a negative entry, a measured client record without snap_incomplete (TE-CB-5), an incomplete measured tick with no room record; a shown bite. |
| spikes/netcode-proof/marks/marks.json | +26, 2 changed (numstat about 30) | Two checks on PM-0001-17's L1 P2, P3 and B1 parts, and one planted bug. |
| spikes/netcode-proof/marks/SERIES.md | +3 rows, 3 changed (numstat about 9) | Two figure rows, one figures-only series row, two header counts and the :320 row. |
| spikes/netcode-proof/marks/README.md | about +4 to +8 | One sentence in §1's decoded_hash paragraph, with the paragraph reflowed to 120 columns. git diff --word-diff must show only the added sentence. |
| spikes/netcode-proof/ops/runs/c-u4r-runs.mjs | +10 to +20 | bitesOf per bite: -17's whole-as-lost on P0 and P2, link-lost-overstated on P2 only, every other bite as today; SPLIT_LINE names no profile (C6); the header comment's 'only P0'. |

About 650 changed lines for C62H-BOUND alone, range 545 to 755. That is up from about 530 (450 to 615) under C2, C6 and C7, by about 120 lines for TE-CB-1, -2, -5 and -6. JDG-ONLY-1 is not in it (C5). Evidence harnesses under .gate/ and .checkpoints/ are not counted.

SIZE-CHECK: if `git diff --numstat <lane base>..HEAD -- spikes/netcode-proof/` passes 1,050 changed lines (added plus deleted) of C62H-BOUND alone, the build stops and goes to Mustafa in three lines: what grew, why, and the cheaper option. The line stays at 1,050: about 1.6 times the plan and 1.4 times the top of its range.

## How it differs, what stays open, the runs

How this differs from option (a) as you chose it:
- "It arrived too late" is no longer an accepted reason. On the simulated network nothing can arrive that late, so the bar is stricter.
- Two checks and two planted bugs, not one of each. The second check catches a network record that claims losses that never happened.
- The judge makes the comparison, in two new figures: a judge change, read under your 13:35 rule. It works from one new lab record: per client and tick, how many snapshot packets the network lost, checked one to one against what was sent. Option (a) named one lab change and one series definition; this adds three series rows, changes one, and adds one README sentence.
- The planted bugs run as full ten-minute runs inside the judged P2 set, not on one short P2 run. A short P2 run stays as a lab test.
- About 46 minutes of runs, not about 35 (the two planted-bug runs).
- Added from the test-engineer's read: each run's summary line also counts its measured lost ticks by reason (the server sent nothing, the network lost a packet, neither). It is a report only and is never judged. The lab's "neither" must equal the judge's count on every run.

Still unbounded after this:
- -58's two wrap parts (L1 P0 and L1 P3): the same kind of hiding place. It can come to you as a decision once both sets pass.
- -17's real-network parts (L2): no network record exists there to explain a loss.
- -17's L1 B1 part, until U4g's judged B1 runs carry both checks. Until then, B1's report line from option (c) stays.
- Outside each run's measured ten minutes (the warm-up and the last few seconds), no lost tick is checked, as the wording says.
- A double fault stays possible: a wrong network record at exactly the tick where a client bug files a snapshot as lost. The two-way checks, the planted bugs and the 17,000 to 43,000 real losses in every run make it narrow, not zero.

This is a stricter reading of ADR 0001's row for -17 ("Every decoded snapshot is the server's state"). The row itself does not change (sha256 3075f638…). Your yes is the decision, and the readings pull request lists it.

The runs after the build: P2, then P3. P0 is not re-run; B1 comes with U4g.
- Time: P2 about 1,735 s (29 min): the last set's 1,047 s, plus two planted-bug runs at about 336 s each, plus a second judge call of about 15 s. P3 about 1,020 s (17 min). About 46 minutes in all.
- Disk: the runner checks at each set's start that its projected logs leave the 4,096 MiB floor. P2 projects 887.6 MiB, so it needs 4,983.6 MiB free; P3 projects 524.4 MiB, so it needs 4,620.4 MiB. Free today: 7,638 MiB, before the build's own compiles. If it is short on run day, the kept-logs packing goes first, and only after its pull request (#53) is on main and this branch has taken main in.
- Judge memory, as the runner's projections against its 10,240 MiB line: P2 measured runs 2,129 MiB, P2 planted-bug runs about 552 MiB, P3 1,530 MiB. Last time, from the logs on disk: 1,758 and 982 MiB.
- Ticks the lab decides lost at the stop: last sets P2 30 / 24 / 23 and P3 87 / 62 / 59 per run. All fall after the measured window and are never checked.
- Expected: every clean run reads 0 on both checks. whole-as-lost reads 48 on the first check and 0 on the second; link-lost-overstated reads 48 on the second and 0 on the first.

Freeze: at your yes, the main session records by script the sha256 of each exact text above: the two checks, the planted bug, the four SERIES rows and the README sentence.
- Before the judged runs, a script shows that the built marks.json, SERIES.md and README.md hold exactly those texts. Each set's runs.txt prints the sha256 of that same marks.json.
- Any change after your yes, a review finding included, comes back to you before the judged runs. That covers an approved text, and what either figure computes as its SERIES row states it.
- The work stops and comes to you if:
  - any clean run or test (the lab's short P2 test included) shows a lost tick that nothing explains;
  - the lab's own count of unexplained ticks differs from the judge's;
  - a planted-bug run reads anything other than 48 on its own check, with every other check passing and no gap.
- The check is never reworded to fit.

**Decisions needed:**
- **C62H-BOUND-YES: yes to the exact wording above (texts v3) and the planned size (about 650 changed lines, 545 to 755; the build stops and tells you past 1,050)?**
  - Why it matters: nothing is built before your yes, and the wording is fixed from then on; the judged runs cannot reword it.
  - Options: (a) yes; (b) yes with changes you name; (c) no, keep only option (c)'s report lines.
  - Recommendation: (a), the advisor's too.

**For Mustafa:**
1. Answer C62H-BOUND-YES.
