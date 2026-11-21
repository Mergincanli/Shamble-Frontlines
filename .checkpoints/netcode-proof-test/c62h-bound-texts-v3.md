# C62H-BOUND: the exact texts, v3 (v2 with the advisor's four replacements of its ruling r2, applied by script)

Written by c62h_bound_texts_v3.py from c62h-bound-texts-v2.md and advisor-c62h-bound-r2-full.json. Each block is word for word; nothing here is retyped. The freeze records these blocks' sha256 at Mustafa's yes.

<!-- BEGIN plain-words -->
```
On the lossy test networks (P2, P3 and B1), every snapshot a test client files as "lost" in each run's measured ten minutes (not its warm-up or its last few seconds) now needs a reason. The reason has to come from a record the client does not write: either the server's own log says it sent that client nothing at that moment, or the simulated network's own log says it dropped at least one packet of that snapshot. Any other "lost" snapshot counts against the client, and the mark needs zero of them on every run. A second check runs the other way: a snapshot the network dropped a packet of can never be filed as received whole. That proves the network's record does not claim losses that never happened. "It arrived too late" is not on the list, because on the simulated network nothing can arrive that late (the slowest packet takes about a fifth of a second, and the client keeps about one second of snapshots). Two planted bugs prove the checks work: a planted bug in the lab's per-client log writer that files one snapshot per client, received whole, as lost, fails the first check (48 of 48 clients), and a network record that invents a loss fails the second.
```
<!-- END plain-words -->

<!-- BEGIN marks-json-placement -->
```
PM-0001-17 in spikes/netcode-proof/marks/marks.json. The 'L1 P0 loses no group' check (marks.json:3972-3982) gets a comma after its closing brace, and these two checks follow it inside "checks":
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
In "bites", the whole-as-lost entry (marks.json:4009-4012) gets a comma after its closing brace, and this entry follows it:
        {
          "id": "PM-0001-17/link-lost-overstated",
          "what": "a lab record of the link's losses that counts a delivered snapshot datagram as lost: on each client, its first measured tick whose datagrams all arrived"
        }
Nothing else in marks.json changes: no other check, report, part, note or draft pin. PM-0001-17/whole-as-lost keeps its id and text, and gains a run on L1 P2 through the runner (see lane). About 26 lines added and 2 changed.
```
<!-- END marks-json-placement -->

<!-- BEGIN check-objects -->
```
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
<!-- END check-objects -->

<!-- BEGIN bite-object -->
```
        {
          "id": "PM-0001-17/link-lost-overstated",
          "what": "a lab record of the link's losses that counts a delivered snapshot datagram as lost: on each client, its first measured tick whose datagrams all arrived"
        }
```
<!-- END bite-object -->

<!-- BEGIN series-row-snap_lost_unexplained_count -->
```
| `snap_lost_unexplained_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with `tick.s.snap_incomplete` 1 whose room-log record at that tick gives that slot a non-null `hs.server_hash` (the room sent it a snapshot or a full state) and `tick.s.snap_link_lost_count` 0 (the link lost none of the snapshot datagrams the room sent it then). Explained, so not counted: `hs.server_hash` null (the room sent that client nothing: a full state not yet acknowledged, budget short, an encoder refusal, a kick) or `tick.s.snap_link_lost_count` 1 or more (npt-l1's fate log lost one by its loss draw or in a burst). Nothing else explains a tick filed lost: not a stale drop (L1 delivers every snapshot datagram well inside the 32-tick ring), not a full-state tick, not a tick whose datagrams all arrived. 0 incomplete is a real 0. Gaps: no room log gives `tick.s.snap_link_lost_count`, a room tick record without it, a room tick record without `hs.server_hash`, a list that does not hold the run's `clients` entries, an entry below 0, a measured client tick record without `tick.s.snap_incomplete`, a `snap_incomplete` entry other than 0 or 1, a measured incomplete tick the room log has no record of. Inputs: `tick.s.snap_incomplete`, `hs.server_hash`, `tick.s.snap_link_lost_count` | PM-0001-17 '100 % of decoded snapshots': no decoded snapshot escapes the comparison by being filed lost (C62H-BOUND (a), 2026-11-15) | -17 |
```
<!-- END series-row-snap_lost_unexplained_count -->

<!-- BEGIN series-row-snap_whole_despite_link_loss_count -->
```
| `snap_whole_despite_link_loss_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with a whole `hs.decoded_hash` (not null, not the sentinel 0) whose room-log record at that tick gives that slot `tick.s.snap_link_lost_count` 1 or more. No client can complete such a tick, so each one counted is a record that overstates the link's loss. A client hash at a tick the room log has no record of is not read here (`decoded_hash_mismatch_count` counts it). Gaps as `snap_lost_unexplained_count`'s. Inputs: `hs.decoded_hash`, `tick.s.snap_link_lost_count` | PM-0001-17, the link record's own check (C62H-BOUND (a), 2026-11-15) | -17 |
```
<!-- END series-row-snap_whole_despite_link_loss_count -->

<!-- BEGIN series-row-tick.s.snap_link_lost_count -->
```
| `tick.s.snap_link_lost_count` | count | npt-lab (from npt-l1's fate log; L1 only) | the room log, one per client slot on every room tick record (the run's `clients` entries, 0 included): of the snapshot datagrams (groups) the room sent that client at this tick, how many npt-l1's fate log records as lost by its loss draw or lost in a burst; read at the send, from the fate-log datagram entries that slot's send pushed, which must be exactly its groups (else the run fails) | snap_lost_unexplained_count, snap_whole_despite_link_loss_count |
```
<!-- END series-row-tick.s.snap_link_lost_count -->

<!-- BEGIN series-row-tick.s.snap_incomplete -->
```
| `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot (the run's `clients` entries, each 0 or 1), at the tick it resolves: 1 when a group of that tick was lost and none refused, or the client dropped one as stale (README §1's `decoded_hash`), so the client logs no hash for it (§4.6), a tick none of whose groups arrived included (known from a later header's tick); with `hs.decoded_hash` it decides README §1's coverage rule (`decoded_hash_mismatch_count`); on L1 P2, P3 and B1 each measured 1 must be explained by the room's or the link's record (`snap_lost_unexplained_count`) | -17 (the coverage rule, its L1 P0 check and its report; snap_lost_unexplained_count), -58, -19 (the coverage rule) |
```
<!-- END series-row-tick.s.snap_incomplete -->

<!-- BEGIN series-placement -->
```
spikes/netcode-proof/marks/SERIES.md.
(1) Header ':258 ## Figures (36): computations npt-judge carries in code' becomes '## Figures (38): computations npt-judge carries in code'. Two rows go in alphabetically, after `session_memory_growth_count` and before `sync_cost_ratio`:
| `snap_lost_unexplained_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with `tick.s.snap_incomplete` 1 whose room-log record at that tick gives that slot a non-null `hs.server_hash` (the room sent it a snapshot or a full state) and `tick.s.snap_link_lost_count` 0 (the link lost none of the snapshot datagrams the room sent it then). Explained, so not counted: `hs.server_hash` null (the room sent that client nothing: a full state not yet acknowledged, budget short, an encoder refusal, a kick) or `tick.s.snap_link_lost_count` 1 or more (npt-l1's fate log lost one by its loss draw or in a burst). Nothing else explains a tick filed lost: not a stale drop (L1 delivers every snapshot datagram well inside the 32-tick ring), not a full-state tick, not a tick whose datagrams all arrived. 0 incomplete is a real 0. Gaps: no room log gives `tick.s.snap_link_lost_count`, a room tick record without it, a room tick record without `hs.server_hash`, a list that does not hold the run's `clients` entries, an entry below 0, a measured client tick record without `tick.s.snap_incomplete`, a `snap_incomplete` entry other than 0 or 1, a measured incomplete tick the room log has no record of. Inputs: `tick.s.snap_incomplete`, `hs.server_hash`, `tick.s.snap_link_lost_count` | PM-0001-17 '100 % of decoded snapshots': no decoded snapshot escapes the comparison by being filed lost (C62H-BOUND (a), 2026-11-15) | -17 |
| `snap_whole_despite_link_loss_count` | count | npt-judge | over the run's client logs (process `client`), measured tick records only: the (slot, tick) pairs with a whole `hs.decoded_hash` (not null, not the sentinel 0) whose room-log record at that tick gives that slot `tick.s.snap_link_lost_count` 1 or more. No client can complete such a tick, so each one counted is a record that overstates the link's loss. A client hash at a tick the room log has no record of is not read here (`decoded_hash_mismatch_count` counts it). Gaps as `snap_lost_unexplained_count`'s. Inputs: `hs.decoded_hash`, `tick.s.snap_link_lost_count` | PM-0001-17, the link record's own check (C62H-BOUND (a), 2026-11-15) | -17 |
(2) Header '## Series read only by figures (18)' becomes '(19)'. This row goes in after `tick.s.relevant_count`:
| `tick.s.snap_link_lost_count` | count | npt-lab (from npt-l1's fate log; L1 only) | the room log, one per client slot on every room tick record (the run's `clients` entries, 0 included): of the snapshot datagrams (groups) the room sent that client at this tick, how many npt-l1's fate log records as lost by its loss draw or lost in a burst; read at the send, from the fate-log datagram entries that slot's send pushed, which must be exactly its groups (else the run fails) | snap_lost_unexplained_count, snap_whole_despite_link_loss_count |
(3) The `tick.s.snap_incomplete` row (:320) gets a new last clause in its Meaning cell and a new Marks cell. The advisor's r2-run Low says '-17 reports it' is stale once -17 checks it:
| `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot (the run's `clients` entries, each 0 or 1), at the tick it resolves: 1 when a group of that tick was lost and none refused, or the client dropped one as stale (README §1's `decoded_hash`), so the client logs no hash for it (§4.6), a tick none of whose groups arrived included (known from a later header's tick); with `hs.decoded_hash` it decides README §1's coverage rule (`decoded_hash_mismatch_count`); on L1 P2, P3 and B1 each measured 1 must be explained by the room's or the link's record (`snap_lost_unexplained_count`) | -17 (the coverage rule, its L1 P0 check and its report; snap_lost_unexplained_count), -58, -19 (the coverage rule) |
```
<!-- END series-placement -->

<!-- BEGIN readme-sentence -->
```
On L1's P2, P3 and B1 a measured tick filed lost must have a reason in the room's or the link's record, never the client's own: the room sent that client nothing then (`hs.server_hash` `null`), or npt-l1 lost a snapshot datagram of it (`tick.s.snap_link_lost_count`, the room log); a stale drop is no reason there, since L1 delivers every snapshot datagram well inside the ring (PM-0001-17's `snap_lost_unexplained_count`; `snap_whole_despite_link_loss_count` checks the record the other way).
```
<!-- END readme-sentence -->

<!-- BEGIN readme-placement -->
```
spikes/netcode-proof/marks/README.md §1, in the `decoded_hash` paragraph. One sentence goes in after '...that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel.' (README.md:66-67), and the following lines are reflowed to 120 columns:
On L1's P2, P3 and B1 a measured tick filed lost must have a reason in the room's or the link's record, never the client's own: the room sent that client nothing then (`hs.server_hash` `null`), or npt-l1 lost a snapshot datagram of it (`tick.s.snap_link_lost_count`, the room log); a stale drop is no reason there, since L1 delivers every snapshot datagram well inside the ring (PM-0001-17's `snap_lost_unexplained_count`; `snap_whole_despite_link_loss_count` checks the record the other way).
README.md:6 holds: this contract change ships together with npt-judge and its fixtures.
```
<!-- END readme-placement -->
