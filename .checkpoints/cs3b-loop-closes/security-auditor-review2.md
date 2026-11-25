# security-auditor, change-set 3b round 2 (head 0dfca749; round 1 bbbf09e4; base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/security-auditor-cs3b2 (detached at 0dfca749).
Round 1's notes: security-auditor-review1.md (threat model, I1-I7, M1, L1, L2; L3 named only in the memory line).

## Plan
A. M1: read ADR 0001 OQ-0001-6 (around :1156) and PM-0001-21, DECISIONS 2026-11-24 + the three mustafa-*.md files;
   judge the reason (1)-(4); the deadline ("before online play" vs "before a command moves anything"); whether a
   bound now would pre-empt OQ-0001-6; re-run my bank probe on 0dfca749 (scratch test in my copy only).
B. L1: read Room::on_datagram's new doc against my L1 list. L2: confirm no manifest change (diff --stat). L3:
   re-derive from my notes (its text was lost) and judge it on the new head.
C. The fix pass as attack surface: git diff bbbf09e4 0dfca749 -- crates packages, file by file:
   C1 endpoint.rs (new/set_post/block/write_block), local_match.rs, host.ts: Sim::input without the Rust guard;
      what changes online.
   C2 Readout.late vs refused: what a detector should see.
   C3 queue.rs insert window; Endpoint::build_input newest always goes; codec.rs Join version before token;
      LocalMatch::step assert text.
   C4 tests/any_bytes.rs: which security properties it asserts; what cheap ones are missing.
   C5 page: keyboard.ts key(.., typing), boot.ts event.timeStamp, worker-link.ts olderPosts + #done list,
      report.ts textContent.
   C6 seed/state (I3): messages, block, logs, panic texts, Debug; the boundary test's Sim pin.
D. Where the 255-presses threat-model line should be written down for change-set 4 and the weapons change-sets.
E. Runs: cargo test (room, client-endpoint, protocol, sim-wasm), clippy --workspace --all-targets, my probes.
F. Answer <= 30 lines.

## Findings (one line each, as found)
