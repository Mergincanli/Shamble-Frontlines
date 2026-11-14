# U4r review, round 1, infra-engineer seat (batch 2)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum 2026-11-14).
Packet: u4r-packet.md sha256 cbd82485... (checked); tests file u4r-packet-tests.md sha256 d12555c8... (checked).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch). Cite file:line there.
Logs of my own runs (if any): /home/user/Shamble-Frontlines/.gate/u4r-review-infra/ (task text; the panel file names .gate/u4r/rev-infra-<n>.log: the task text is the newer, more specific word, so I follow it).
Verdict vocabulary: the schema's (APPROVE / APPROVE WITH FINDINGS / REQUEST CHANGES).

## Plan
1. Sources: panel section 6 + "What every seat is told" (read); carry 84 (u4s-carries.md :402-416); ruling 1 point 1 (advisor-u4r-build1-full.md); brief :1898-1899 and section 4 rules (no-retry, floors); LEFTOVER-DELETE wording.
2. Item (1): check.mjs wasmStep: shared deadline WASM_CALL_LIMIT_MS 300 s from first non-cargo call; skip-and-name after a stop; process-tree kill by documented means; the two self-tests (fake runner+clock; real processes with grandchild ~200 ms).
3. Item (2): ops/runs/det-u4.mjs + wasmStep's two options: imports wasmStep with det-recordings.json, bites half off, own .gate/ folder, prints like check.mjs main, exit 0 only on PASS, no retry; check.test.mjs case for the options.
4. Item (3): times vs budgets: c5-end1.log:174, c5-end2.log:174 (39.6 s / 41.9 s), warm/cold labels with uptime; det-u4.log:24 200.5 s; non-cargo <= 240 s inside 300 s; report section 5 item 4 and return item 9 vs ruling 1 point 1 (7).
5. Item (4): ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs: lane lock and -x, refuse uncommitted state, .gate/ folder by commit 12 hex, df and floors before release builds, no clean/delete. Report disk figures only.
6. Runs only if a finding needs one (through u4r-run.sh -x, df first).

## Progress / findings (a line per finding as I go)
