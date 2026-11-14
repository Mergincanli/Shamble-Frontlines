# security-auditor: U4r review, round 1 (batch 1), 2026-11-14

Snapshot under review: .checkpoints/netcode-proof-test/wip/u4r.patch, sha256
572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum: equal).
Packet u4r-packet.md sha256 cbd82485... (equal), tests u4r-packet-tests.md d12555c8... (equal).
Read copy: .gate/u4r-snap1 (2fbec77 + patch; 26 entries: 13 M, 13 new, by git status).

## Plan (the seat's list, advisor-u4r-panel-full.md section 3)
1. Carry 76: session boundary, lab/src/scenario.rs :405-475; r8 and r9 tests.
2. Carry 85 (a)-(e): check.mjs deps step refusals; each names source and value; one self-test each;
   container env and spike config still pass. hash-wasm/loader.mjs header sentence (only edit there).
3. Carry 63: crates/log/src/lib.rs no_duplicate_key :396 (called from read_log :578), is_control :305-324,
   tests from :976; each test failing before its refusal; nothing else in crates/log changed.
   FU-3 and FU-7: closed or open.
4. U4R-Q6 (a): wall-clock read in pump.rs :217-290: bites builds only; allow on the site alone
   (server/clippy.toml:6-17); no wall clock on non-bite paths of npt-server, npt-lab, npt-bots
   (clippy.toml bans; bots/clippy.toml new).
5. Paths from manifest names and command-line text: carry 82's name and ticks checks in hash.rs; run
   scripts' --out folders.
Sources: brief u4r-brief.md, carries u4s-carries.md :255-441, rulings 1-4, plan :750-803.
Logs (if any run): .gate/u4r-review-security/.

## Progress
