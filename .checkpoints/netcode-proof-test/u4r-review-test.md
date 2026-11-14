# U4r review, round 1, test-engineer seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum, 2026-11-14).
Packet: u4r-packet.md sha256 cbd82485... (checked); tests file u4r-packet-tests.md sha256 d12555c8... (checked).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch). No run of mine: nothing under .gate/u4r-review-test/.

## Plan (seat list, advisor-u4r-panel-full.md section 2)
1. r1..r10 and the 2.5 bites: each unit-shown, fails by name under its bite/plant.
2. Ruling-added tests (dump reader, ring edge, hash.rs header compare, radius None, late-command exact, r6 L and whole
   list, one-id-outside-PM, parse cases, precheck no --commit, wasmStep options, carry 63 before-refusal, scratch helper).
3. Counts 11/491/561/71 -> 11/526/603/76; each new test named and in --list; TE2-5.
4. Evidence standing: pre-commit labels; c-u4r-r2.mjs and c-u4r-runs.mjs lines as run, tick16 expect text, bites first,
   carry 62 (h) plain check after the record run.
5. SERIES.md's 21 new rows against their writers.
6. Own earlier findings: carries 73, 83, 84, 85, 59.
No run made: disk 4.7 GB free (07:2x UTC, df), the runner takes only .claude/worktrees/ copies and .gate/u4r/ logs,
and the panel says never plant. T-1 is reasoned from code, with its plant named for the fix pass.

## Deviation (mine)
One Bash call created and at once deleted an empty file u4r-review-test.md.tmp in the notes folder by a shell redirect
(a slip, no content). This session has no Edit tool, so this notes file is rewritten whole with Write. One read-only
timing probe (node, in the session scratchpad, no file written): a node child reaches both pid lines 20-30 ms after its
own start, load 0.02.

## Checked (held)
- Hashes match (patch, packet, tests file).
- Counts: 42 new Rust test fns, 7 bites-only (r1_bite, r3_bite, r4_bite, p4_bite, late-command, wallclock, 83 (d)):
  +35 measured (491 -> 526), +42 bites (561 -> 603); check.test.mjs +5 (71 -> 76). All 42 in c5-end2.log's --list.
  No test removed. c5-end1/-end2: 11 of 11 steps each.
- TE2-5: the only full check.mjs runs are c5-end1, c5-end2 (and c4-deps, 2 steps): no test-step failure to record.
  Every other FAILED cargo log is a plant or a development run outside check.mjs.
- c-u4r-r2.mjs SUITES args equal the builder's lines (c3-r2-p0/suites/bites, c5-p3, c5-p8a, c5-r2lines) but for
  --date/--commit/--out; tick16's expect text is ruling 4 pt 1's word for word. npt-judge (judge/src/main.rs:52-123)
  and precheck (main.rs:271-283) exit 0 on FAIL/VOID, so neither script stops on an expected FAIL. Bites first is the
  judge's rule over a folder; run order inside a set does not change it.
- Ruling tests: p2 None panics (scenario.rs:2205-2214); p5 exact (:2114); late-command three exact asserts
  (precheck.rs:928-939); r6 L from o.lead and whole list (:853-876); one-id-outside-PM (contract.rs:579-588); p9 both
  flags (scenario.rs:2217-2237); precheck no --commit exit 2 (precheck.rs:949-958); options case (check.test.mjs);
  p6 dump reader against SimCore::step per tick (logs.rs:726-785); h5 header compare per field (hash.rs:1417-1433);
  FU-3/FU-7 failing before (b3-log-before.log). Plants in logs: r8, r9, 84 (b), run() timeout, 83 (b), 83 (c), p1 (4),
  85 (a)-(d), p2, p5, p6, 83 (d); bots clippy.toml clock and HashMap (b4-plant-*.log).
- SERIES.md: 21 new rows; every name has a writer in npt-lab (grep of scenario.rs, logs.rs, mirror.rs); 8 (B)'s
  baseline_age words in (:355).
- Own carries: 73 met (tick16d per ruling 4); 83 (a)-(d) met (lib.rs:8-9, check.test.mjs cases, hash.rs:1653-1749);
  84 (a)-(c) met (check.mjs run() :1894-1906 group kill, two tests); 85 (a)-(d) one case each, env and config pass;
  59: ES suite run (ruling 3 pt 2), the pair is the report's (no report on disk).

## Findings
- T-1 (Medium) scenario.rs:1903-1908, :1958-1968. r8's "the first's Inputs reach no room call" cannot fail for that
  misroute. room_counts omits ack_refused, ack_refused_commands, acks_old. The replaced endpoint acked its join FullState
  (client/src/lib.rs:802), so its Input has ack_groups 1; the replacing ClientSlot's fresh AckRing (room.rs:644) never
  sent that tick: ack() is NeverSent (ack.rs:161-171) and receive_input returns at room.rs:783-788, moving only
  ack_refused/ack_refused_commands. b11-plant-r8.log fired at the glue's `ended` assert, never at this one; under a
  plant that skips `*s = None` (scenario.rs:416) r8 fails only at the Leave assert. Fix: compare the whole
  ClientCounters (PartialEq, room.rs:63-64) before and after; show it with that plant, restored by Edit, sha256 checked.
- T-2 (Medium) det-recordings.json and precheck-recordings.json pins unverified at the snapshot. det-u4.log is copy 4's
  (01:08-01:11 UTC); copy 5 changed mirror.rs's keep after it (u4r-build.md:1217-1222, 01:44-01:49). SS-P0-s4 was
  regenerated in full only in regen-precheck.log (copy 1, 20:27). check.mjs's wasm step reads recordings.json alone.
  Fix: re-run det-u4.mjs and `npt-lab hash --manifest crates/lab/precheck-recordings.json` once each (-x) at the head;
  report both PASS lines; label the old logs by their tree.
- T-3 (Low) check.test.mjs:2901-2919: real-process test races node start against 200 ms (pids asserted at :2910).
  Fix: /bin/sh child, same 200 ms.
- T-4 (Low) scenario.rs:1642-1651, :1653-1677, :1694: r1 clean passes with no fate_log event in either log, and with
  room ticks alone above 200.
- T-5 (Low) contract.rs:452-460, scenario.rs:1812-1822: Scratch dropped before the test's asserts.
- T-6 (Low) contract.rs:464-591: no doctored case for carry 60, missing class_audit or breaches_count, duplicate admit;
  precheck.rs:486-504 verdict's rewinds/off-stamp/RTT/Empty VOID causes have no unit case.
- T-7 (Low, against ruling 4's further finding) advisor-u4r-build4-full.md:32 names .gate/u4r/whole-as-lost/ for the
  before-judged-runs Shown check; ruling 2 pt 33 (build2-full.md:40-42) makes the re-made run the record; no script
  re-makes it (c-u4r-r2.mjs:52-66 has no whole-as-lost suite). Fix: a whole-as-lost suite in c-u4r-r2.mjs.

## Could not check
The builder's report (not on disk): its labels (ruling 2 pt 33), r4 wording, r5 estimate, tests named per file.
No plant or run of mine; T-1 from code.
