# Advisor: review of the pushed head after U4r's merge into the spike branch (PR #40), 2026-11-14

Task: review merge commit 71ee13d and the branch head after it, against advisor-u4r-r2-full.md next steps 2-10 (M1-M9).

## Plan
1. Branch head: confirm the commits after 71ee13d touch only .checkpoints/.
2. Merge commit file list: spikes/ part equals the reviewed patch (450af1a's wip/u4r.patch, sha256 21ca46be...),
   file for file; no marks.json change; other paths .checkpoints/ only; wip/u4r.patch removed.
3. Logs (.gate/u4r-merge/: m5.log, gate.log, gate-no-smoke.log) against M5-M7's expectations: uptime first, six
   builds with main-folder provenance, check 11 steps, 531/609, 0 failed 0 skips, self 77/77, marks 81, redact 0,
   deps and wasm lines as c6-end2.log:173-174, det-u4 PASS with non-cargo sum <= 240 s, SS-P0-s4 sha256 6933b56a...
   equal to the manifest's, TE2-5's state.
4. u4r-build.md "## Merge into the spike branch": M1-M8 records against the logs.
5. u4r-report.md: each statement against its sources; step 9's list all present.
6. CI on 71ee13d (gate run 43603457356 gate+smoke, toolchain 43603457370), if GitHub is reachable.
7. Verdict, next steps, Mustafa.

## Checks (a line per check as I go)
- C1 head: origin branch = local HEAD = f0a1e60 (now two .checkpoints/-only commits after 71ee13d: 94dc9e3 and f0a1e60,
  both only .checkpoints/netcode-proof-test.md, mode 100644). Main still 4460ef5.
- C2 patch bytes: git show 450af1a:...wip/u4r.patch and 71ee13d^:... both sha256 21ca46be...bce. 26 diff chunks.
- C3 spikes/ = patch: per-file split of `git diff --no-renames 71ee13d^ 71ee13d -- spikes/` vs the patch: 26 files each,
  none only in one, 26 of 26 chunks byte-identical (only the file order differs); every patch index abbreviation is a
  prefix of the full blob ids. 13 A + 13 M, all 100644, no 120000/160000. No marks.json path.
- C4 other paths: .checkpoints/netcode-proof-test.md (M), u4r-build.md (M), u4r-report.md (M; drafted at 5e0eae1),
  wip/u4r.patch (D). Nothing else.
- C5 base: spikes/ at 71ee13d^ equals 2fbec77's (git diff --quiet exit 0); commits 2fbec77..71ee13d^ touch only
  .checkpoints/ (I used git diff-tree for this list: read-only, but outside my named commands; noted).
- C6 m5.log: lock exclusive; uptime first (load 0.27, 0.09, 0.10); HEAD 0eded3c + the applied patch (uncommitted, as
  M3-M5 run before M8); six builds, all provenance lines under /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/;
  check 11/11; test 531 / 609, 0 failed, 0 skips; self 77/77; marks 81, ADR rows unchanged; redact 0 findings.
- C7 deps line (m5:176) = c6-end2.log:173 with the time stripped (exact); det-u4's deps line = check's deps line.
- C8 wasm line (m5:177) = c6-end2.log:174 except times and the native hash-map digests (a05095fd/070080d7 vs
  dd69689d/f2a899af). Native digests differ in every kept log (7 distinct pairs found), Wasm digest f5e8d0e8 stable:
  by design (the bite is the hash-map order). Catch points as expected (SS-s1 tick 1234; std-sin SS-s1 tick 2
  vehicle[8].pos.y). 43.5 s < 300 s.
- C9 det-u4: PASS, 18000 / 0 both pairs; non-cargo sum = npt-lab hash 223.8 + Wasm 2.5 + 0.9 + 0.0s = 227.2 s <= 240
  (round-2 base 223.7). Step total 240.3 s includes builds 11.6 + 1.4.
- C10 regen: sha256 6933b56a... printed "as the manifest's"; I hashed .gate/u4r-merge/regen/rec/SS-P0-s4.rec myself:
  6933b56a...bab3 = precheck-recordings.json at 71ee13d; 16950444 bytes; native-1.lines cmp-equal native-2.lines
  (9000 lines). Command = regen-precheck.log:18's but the folders (.gate/u4r-merge/regen vs .gate/u4r/regen). 130.1 s.
- C11 gate.log: full gate FAIL only at 5a (Chromium executable missing); gate-no-smoke.log PASS, smoke skipped.
