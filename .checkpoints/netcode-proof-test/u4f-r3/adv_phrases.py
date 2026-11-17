# Read-only (the advisor's check of u4f-brief.md rev 3.2): counts each fold anchor in the brief, with every line break
# and the next line's indent read as one space (the fold's own rule). Each must print 1. Writes nothing.
# Run: python3 -I /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/adv_phrases.py
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
flat = re.sub(r"\n[ \t]*", " ", open(N + "u4f-brief.md", encoding="utf-8").read())
PHRASES = [
    "### 2.7 npt-lab: suites, settings and series",
    "| f19 | the 1.5x clock: excess commands at most 0 in the clean build,",
    "| f20 | the lag switch under P1: at most `CMD_CATCHUP` commands run per tick; movement within the catch-up cap's |",
    "| f34 | NA-14: under option A,",
    "plus crates/proto and the catalogue only by U4F-Q6 (b) and crates/server only by U4F-Q14 (a) or U4F-Q6 (d);",
    "crates/proto and the catalogue if U4F-Q6 (b) rules a wire change; crates/sim's carry 18 places if U4F-Q8 (a); crates/server if U4F-Q14 (a) or U4F-Q6 (d) needs a pump change.",
    "4. **Only if a ruling opens npt-server before U4h** (U4F-Q1, U4F-Q14 (a) or U4F-Q6 (d)):",
    "U4f changes no pump code by this brief; if U4F-Q14 or U4F-Q6 needs any, the lane opens npt-server (U4F-Q1) and section 6's `--all-features` item applies.",
    "| npt-server | none (2.5), unless U4F-Q14 or U4F-Q6 opens it | |",
    "| D9 | `cargo test -p npt-server --all-features` compiles (only if a ruling opens npt-server) |",
    "after U4f's merge and before its first set,",
    "| f31 | carry 39: each rate-derived value U4f adds, pinned at 30 and 60 Hz: ClockSync's tick period in ns;",
    "| B2 | new SERIES.md rows for report-only series",
    "| B7 | U4F-Q15 (b):",
    "**LEAD-BOUND:** may U4f's room refuse a command stamped more than 400 ms",
    "| -20 | `tick.s.cmd_on_time` (:61),",
    "| p12 | -20 | U4F-Q10's option, P3 | redundancy-1 |",
    "| f1 | ClockSync on a fixed P2 arrival trace at one phase per connect,",
    "| Ping rate and phase | data, at most the room's limit",
    "| **Total** | **about 4,850 (3,900 to 5,900)** |",
    "with its first `<crate>` replaced by \"npt-room, npt-client, npt-bots or npt-lab (and each crate U4F-Q1 opens)\"",
    "**Counts at rev 3.2**",
]
for p in PHRASES:
    print(flat.count(p), "|", p[:100])
