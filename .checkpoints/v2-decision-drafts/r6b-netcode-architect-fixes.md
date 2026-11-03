# r6b netcode-architect fixes (ADR 0001 outside Pass marks and the checklist)

### NOTE rows-for-test-engineer
no change: this block edits nothing; it tells test-engineer exactly which pass-mark rows AMENDMENT condition 5's
rule covers (the rule itself is FIX arm-rule, after item 5 of "What a spike must prove"; the rows' text is
test-engineer's). Line numbers are docs/adr/0001-multiplayer-netcode.md on disk before this round's edits. 23 rows
name LG in the machine column (script): PM-0001-1, -2, -3, -14, -17, -18, -19, -20, -23, -24, -25, -26, -59, -27,
-31, -37, -38, -63, -42, -61, -48, -49, -54.
Covered (the row compares a result an LG client computes with the server's, and the client side does float work):
- PM-0001-54 (line 1546): "1,000 scripted throws, each landing compared with the server's", machine "L1; L2 with
  LG"; the client side is the shared trajectory function (float), whose aarch64 identity no row proves (PM-0004-73,
  ADR 0004 line 1178: CI native and Wasm only). Suggested way: (ii): a throw whose landing on an LG client differs
  from the server's is fed again (its start position, velocity and fuse, as PM-0004-73 feeds throws) to the
  trajectory function on x86-64 (L1 on CI's runner); it counts as a desync only if it differs there too.
- PM-0001-48 (line 1538): "of the shots whose ray hits the target's volume in the shooter's view (the state the
  Worker published for the command's view time)", machine "L1; L2 with LG's shooter"; the client casts that ray
  with the sim crate's ray query against interpolated remote entities and quantises the hit point (part 5, lines
  257-260): float work. Suggested way: (i): L2's shooter runs in ML's Chrome (Wasm) or an x86-64 client, with LG's
  clients as load only (per-shot replay of a share mark needs every snapshot's arrival logged); (ii) stays open to
  the row.
- PM-0001-17 (line 1498): "100 % of decoded snapshots rebuild the server's quantised state ... checked by the
  client-held state's hash", machine "L1 (48 headless clients); L2: LG's 48 clients". Float work: yes. Part 6
  (lines 321-322) has the Worker decode the wire "into its own state" and fixes no integer form for it, and the
  row's bite plants "a client quantiser that rounds differently from the server's", which exists only if the client
  turns decoded values into floats and back. Suggested way: (ii): each mismatched tick's datagrams (logged as bytes)
  are decoded again on x86-64 (L1's decoder on CI's runner) against the server's logged hash; only a mismatch that
  repeats there counts.
- PM-0001-19 (line 1502): "the client-held state's hash equal to the server's for that client", machine "L2: LG's
  clients and ML's Chrome": the same decode and hash as -17, so covered. Suggested way: ML's Chrome's joins are
  judged as they stand (i); LG's joins follow (ii) as -17, with the full state's stream bytes; the 300 ms limit
  on LG's joins is a time and counts as measured.
Not covered:
- PM-0001-18 (line 1501): "every client decodes deltas again within 500 ms of the link coming back (a plain limit
  per client and burst)": its verdict is a time per client and burst, and it compares no value an LG client
  computes with the server's (a hash checked in the same B1 run is -17's).
- PM-0001-49 (line 1539): its verdict, "100 % of shots resolved with a rewind at most the cap", is the server's own
  rewind against the cap; the registration it records beside it is PM-0001-48's, under -48's way.
- PM-0001-42 (line 1530) and PM-0001-61 (line 1536): judged on "ML: Chrome over L2 with LG's 47 clients" and on
  L1, with LG as load only: already way (i), no change.
- The other LG rows compare nothing an LG client computes with the server's: PM-0001-1, -2, -3 (rig and profile
  facts, server counters: lines 1481-1483); -14 (datagram sizes at the send side, 1495); -20 and -38 (arrival
  counts and press counters, integers by part 5, line 226: 1503, 1523); -23 (snapshots missing at the client,
  1506); -24 and -31 (a snapshot's age on the client, a time against a limit: 1507, 1516); -37 (lost-or-late
  shares, 1522); -25, -26, -59 (server tick, memory, closes and exits: 1508-1510); -27 and -63 (server tick times
  of rooms per VM: 1511, 1527).
L1 alone never needs the rule: its server and headless clients run as one build on one machine on simulated time
(line 1378, "any machine or CI"; line 1698), so an L1 verdict compares no two architectures. A replay under (ii)
must name x86-64 (CI's runner: ADR 0002's determinism job has no arm64 leg, ADR 0002 line 626), since "any
machine" could be an Arm laptop. Each covered row names its way in its method or runs cell; no row is added.
END

### FIX arm-rule
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   input to screen at P0, p50 and p99 on ML and RD (OQ-0001-15).
AFTER:
   input to screen at P0, p50 and p99 on ML and RD (OQ-0001-15).

**LG's clients on Arm: where a client's result is judged.** LG's headless bot clients (the key below names LG, RS and
L1) run on Hetzner CAX21s, Arm64 VMs ("a CAX21 for the 48 simulated players", "so their build targets aarch64":
Mustafa, 2026-11-03), while RS is x86-64. No pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm.
That agreement is unproven: ADR 0004's PM-0004-73 checks the trajectory function on CI's native x86-64 build and on
Wasm only, and its PM-0004-26 adds native AArch64 only before ADR 0006 may pick Arm hosts; ADR 0004 part 7's float
rules are not enough on their own. So every row that compares a result an LG client computes with the server's
(PM-0001-17, -19, -48 and -54) names one of two ways: (i) its verdict comes from L1, an x86-64 client or a browser
(Wasm), with LG's clients as load only; or (ii) a mismatch seen on LG counts only if the same inputs, replayed on L1 on
x86-64, show it too. A mismatch that shows only on aarch64 is reported as a cross-architecture finding and decides
nothing. The rule holds whatever LG turns out to be, and it adds no row (the advisor's reading, 2026-11-03).
END

### FIX sh-1293
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   (R2-ROOM-REC2 marks 2 and 4, on the CCX23 and the CX33: R6-HOST-REC2, ADR 0006).
AFTER:
   (R2-ROOM-REC2 marks 2 and 4, on the CCX23 (ADR 0006)).
END

### FIX capacity-756
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| No Hetzner capacity in the EU | the reference server cannot be ordered | R6-HOST-OQ1 (ADR 0006) |
AFTER:
| No Hetzner capacity in the EU | the reference server or a load generator's CAX21 cannot be created in `fsn1` on a run day: both are created fresh each run day ("They'll be created fresh on run day": Mustafa, 2026-11-03), so his console check does not promise that day's capacity (the advisor's reading, 2026-11-03); that run waits for capacity (proposed), and a lasting shortage takes OQ-0006-1's option (b) route, the fallbacks read and a new reference server named | OQ-0006-1, answered by his check, and its option (b) (ADR 0006; R6-HOST-OQ1) |
END

### FIX domain-464
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    Findings, R2-ROOM-F12, lines 1494–1500), so in the proof test it presents a browser-trusted certificate on a domain
    Mustafa registers ("for the WebSocket leg, a domain I register": SPIKE-MACHINES, 2026-11-03), OQ-0006-5's
    development case (i), issued by a method the proof test's plan sets with security-auditor (the advisor's reading,
    2026-11-03). *Why:*
AFTER:
    Findings, R2-ROOM-F12, lines 1494–1500), so in the proof test it presents a browser-trusted certificate on
    shamblefrontlines.com, registered (Mustafa, 2026-11-03: "DOMAIN: done. shamblefrontlines.com is registered at
    Cloudflare."; his earlier words, SPIKE-MACHINES, 2026-11-03: "for the WebSocket leg, a domain I register"),
    OQ-0006-5's development case (i), issued by a method the proof test's plan sets with security-auditor (the
    advisor's reading, 2026-11-03). *Why:*
END

### FIX domain-1274
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
     on the domain he registers ("for the WebSocket leg, a domain I register"), OQ-0006-5's development case (i), issued
     by a method the proof test's plan sets with security-auditor (the advisor's reading, 2026-11-03; ADR 0006's
     PM-0006-15).
AFTER:
     on the domain, registered (Mustafa, 2026-11-03; his earlier words: "for the WebSocket leg, a domain I register"),
     OQ-0006-5's development case (i), issued by a method the proof test's plan sets with security-auditor (the
     advisor's reading, 2026-11-03; ADR 0006's PM-0006-15).
END

