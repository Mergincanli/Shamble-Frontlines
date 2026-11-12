# U4d2 review round 1: engine-architect seat (the boundary)

Read only. Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + U4d2's snapshot patch). Base copy:
/home/user/Shamble-Frontlines/.gate/u4d2-prefix (51520b4). No cargo or node runs by this seat; runs I would want are
listed for the main session at the end. This seat has no Edit tool: the notes file is rewritten whole with Write each
time, its content kept.

## Plan
1. Read the packet's heads and the judged-against texts (u4d2-packet.md): §5 U4d2, §4.1, §4.11, §4.14, §4.15, Lanes,
   §7.1; plan.md §4.1 hash-wasm line, §4.2, §4.5, §4.7's npt-lab line; advisor rulings named; carries routed to U4d2.
2. Edges: crates/hash-wasm/Cargo.toml, crates/lab/Cargo.toml, the workspace Cargo.toml; check.mjs's deps step (LEAF,
   WASM_CRATES, rules), against the plan's edges.
3. The Rust-to-JS boundary: hash-wasm's lib.rs (seven exports, status constants), loader.mjs (export list, one home),
   check.mjs's wasm step's use of it; reuse by U7's Chromium leg and wasm-opt (C-U7-1).
4. HashCompare and TickHash (lab): shape, Empty, reuse by U4r's -30 pre-check (condition 2).
5. Public items: every pub item added (Rust, check.mjs exports, loader exports) from the diff, each with its reason.
6. Seam (u4-build-plan.md §4.2) and lane (Lanes): the file list of the change vs the lane; anything built that no line
   asks for.
7. Verdict and the run list for the main session.

## Notes (one line per finding, as found)
- [resume 2026-11-12, after a usage cut] Plan step 1 done: packet's judged-against texts read (u4d2-packet.md:4255-6130):
  §5 U4d2, §4.1, §4.11, §4.14, §4.15, Lanes, §7.1; plan.md §4.1, §4.2, §4.5, §4.7; conditions 2, 7, 8, 10; Q3; W1-PUB;
  W1-TARGET; B1.2, B1.3; R1, R4.1, R6, T4; B2.3, B6; F2-ROUTE, L4; advisor-u4d2-brief whole; ADR rows; carries.
- Lane, file list (git status in the read copy): 9 tracked changed (Cargo.lock, Cargo.toml, check.mjs, check.test.mjs,
  client-wasm/Cargo.toml, lab/src/{lib,main,probe}.rs, proto/Cargo.toml) and 8 new (hash-wasm/{Cargo.toml,
  co-initial-dump.txt, loader.mjs, src/lib.rs}, lab/{clippy.toml, recordings.json, src/hash.rs, std-sin-scan.txt}).
  Every path is inside the Lanes line's U4d2 list (u4-build-plan.md Lanes, wave 2, as U4D2-Q9 rewrote it). No
  crates/sim, room, client, server, l1, proto source file.
