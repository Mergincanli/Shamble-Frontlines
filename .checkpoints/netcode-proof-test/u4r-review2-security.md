# U4r review round 2: security-auditor seat (notes)

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked, matches).
Packet u4r-r2-packet.md b1526bde... (checked); tests file 637249a5... (checked); runner u4r-run.sh 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + patch 21ca46be: cite here), .gate/u4r-snap1 (572c4f69, compare).
Probe log: .gate/u4r-review2-security/probe.log (written with Write from printed output); binaries form1, form2 there.

## Plan
1. Ruling "Round 2", my line: FP-1 (the boundary, both halves), FP-3 (manifest and argv, the bounds), FP-5 (h), SEC-U4R-5/-6 in check.mjs.
2. Confirm SEC-U4R-1..7 fixed / deferred as ruled, each with file:line in snap2.
3. Review changed lines only (diff snap1 -> snap2 in my areas).
4. Check the builder report's claims for my areas against code and the c6-*.log logs.
5. Run only if a finding needs it.
6. Result: verdict, sha, round1 states, new findings.

## Progress (all steps done)

- FP-1 / SEC-U4R-1: FIXED as ruled. snap2 scenario.rs:492-505 (closed decoder OR LinkState::Closed, after the pump loop; owner check :498; room.leave :499; owner.remove :500; (slot,"closed") :502); module text scenario.rs:24-25. LinkState has only Connecting/Open/Closed (link/src/lib.rs:45-51), so Closed covers drop and transport failure. Room::leave (room.rs:690-695) uses get_mut: no panic. ended "closed" consumed by logs.gone (scenario.rs:900-902, :968-970); precheck shares Server::io/send (precheck.rs:380, :452). Test fp1 scenario.rs:2171-2213 (closed_checks :2150). Plant c6-plant-fp1.log: both cases fail by name ("the room still holds slot 0"), exit 101. Orderings checked: a replaced old session is gone before its index is reached (drop_slot); a later rejoin in the same pass finds the slot free; Join-then-drop gives admit then gone. Test-engineer's lane: the rejoin asserts hold under the plant too (the builder says so).
- FP-3 / SEC-U4R-2: FIXED as ruled. hash.rs:237 RUNNER_PROFILES, :339-344; scenario.rs:589-635 record() argv as separate items; doc :583-588. All manifest readers go through hash::manifest (hash.rs:914, :1044, :1184; precheck.rs:632); only generate (hash.rs:391) calls record; manifest() errs before regenerate_all generates. Plant c6-plant-fp3.log: r10_fp3 fails at hash.rs:1596 "the profile \"P2 --record x\"", exit 101.
- SEC-U4R-3: FIXED as ruled. hash.rs:345-356 ticks == (w+m) x 30 checked; :325-331 bytes <= 64 MiB; :357-364 span_ms. scenario.rs:1367-1381; :1443-1453. Test scenario.rs:2423-2431. run()'s sums (:861, :1081) covered; burst_starts' terms (:1306) bounded; due_ns u128 (:130-132). Observation only: run bound ticks <= bytes/12 <= 5,592,405 is ~150x looser than ~1,883 B/tick, but the wasm step's and det-u4's shared SIGKILL deadline caps npt-lab hash, and manifests are committed reviewed text. No finding.
- SEC-U4R-4: FIXED as ruled (in-PR half); U6 half DEFERRED as ruled. r8 scenario.rs:2045-2068; module text scenario.rs:22-23. Plant c6-plant-fp2ii.log fails at "one Control-channel message to the first" (left 0, right 1). U6 half in the report's follow-up list (u4r-build.md:1949-1950).
- SEC-U4R-5: FIXED as ruled (check.mjs:694-703; DEPS_CHECKED :2102; self-test check.test.mjs:2103-2104). Round-1 probe re-run on snap2: the three round-1 forms now 1 new problem each.
- SEC-U4R-6: FIXED as ruled (check.mjs:1499-1509; self-test check.test.mjs:1129-1140).
- SEC-U4R-7: FIXED as ruled. c-u4r-runs.mjs:96, :115, :118-122; check :272-275 before first use (:324-327; run ids :189 via :279); marks copy sha checked :301 before re-plan :302. c-u4r-r2.mjs:66, :97-98, :162-163. c6-fp5-helpers.log test 2.
- End checks: c6-end2.log, c6-end3.log "check: all 11 steps pass", 531/609, deps PASS, self 77 of 77; c6-end1.log exit 2 at wasm (reported).

## New findings

- SEC-U4R-R2-1 (Low) check.mjs:694-703: comment overclaims; split-attribute #[path] (`# $a` with $a:tt = [path = ".."], or `$h [path = ".."]` with $h:tt = #) passes the scan (probe 2: 0 new problems) and rustc builds both, loading a .txt module (probe 3). Reach: deny-level crates only (tcpinfo, hash-wasm), neither has macro_rules today. Fix: refuse macro_rules!/macro in the deny-level crates; correct comment; self-test both forms.
- SEC-U4R-R2-2 (Low) check.mjs:1502-1507: quoted whole-name header (["profile"], [ "profile" ], ['profile']) is refused but named as line 1 / "[build]" (probe 4). Fix: accept `]` after the string in `quoted`, no silent line-1 fallback, self-test case.

## Anti-cheat coverage (the lab glue, this round's subject)

| Player action | Where the server validates it | Gap |
|---|---|---|
| Join with token and bind | Room::join via Server::io (scenario.rs:440-453) | none new |
| Malformed control frame | pump closes the decoder; Server::io ends the session, frees the slot (scenario.rs:494-503) | closed (SEC-U4R-1) |
| Connection dropped / transport failed | LinkState::Closed, same branch (scenario.rs:494) | closed |
| Inputs or Leave from a replaced session | drop_slot before routing (scenario.rs:408-421, :472-475); r8 whole counters | Kick best effort on L1 (U6 carry) |
| Kicked session's later traffic | Server::send drops it (scenario.rs:542-549); r9 | none |
| Manifest text into a regeneration | hash.rs:325-364, record argv (scenario.rs:589-635) | closed (SEC-U4R-2/-3) |
| Committed JSON into run-script paths | nameProblems (c-u4r-runs.mjs:272-275) | closed (SEC-U4R-7) |
| Unsafe code via an unread module (CI scan) | carry 85 (c) (check.mjs:694-703) | split `# $a` form (SEC-U4R-R2-1) |
| Cargo config profile | cargoConfigProblems (check.mjs:1496-1509) | message line only (SEC-U4R-R2-2) |

Verdict: APPROVE WITH FINDINGS (two Lows; below High, they become STATUS follow-ups per the budget; a security finding left open keeps the PR needs-mustafa).
