# Advisor: U4r review round 1 ruling (PR #40, netcode proof test)

## Plan
1. Read advisor.md from main (done), the round-1 file u4r-review-r1-full.md whole (done), the seats' notes as needed.
2. Context: the panel ruling, ruling 1 point 1 and point 3, ruling 4 (done); carries 40-85 (done); history.md for
   Mustafa's words (A4 Low rule :340, LEFTOVER-DELETE :404, SIZE-CHECK :398, USAGE-SAVINGS :291).
3. Each finding checked against the read copy .gate/u4r-snap1 (patch sha256 572c4f69..., verified).
4. Fix list, round 2 scope, Mustafa items.

## Facts I checked myself (read copy .gate/u4r-snap1/spikes/netcode-proof unless named)
- Patch sha256 572c4f69435da2ad...c09c2 (wip/u4r.patch), read copy at 2fbec77 with the 26 files.
- scenario.rs:488-497: closed branch sets sessions[i] = None and pushes (slot,"closed"); no room.leave, no owner.remove.
  pump.rs:364-367 close_malformed sets State::Closed; :414-421 a joined session's bad stream message closes it.
  room.rs:689-694 Room::leave (roster.leave + clients[slot] = None); roster.rs:116 replaced = s.live (so a rejoin
  after leave is not a replacement). link Loopback state() Closed when either end dropped; L1 End state Closed when an
  end dropped (l1 :1740-1746); run() keeps every client for the whole loop (no mid-run drop).
- r8 (scenario.rs:1943-1979): room_counts (:1903-1908) sums buffered, repeat, late, buffer_full, press_refused only;
  receive_input (room.rs:768-788) returns on Acked::NeverSent after ack_refused only. ClientCounters derives
  PartialEq, Eq (room.rs:63-64). pump_report touches no counter (room.rs:699-708). r8 never polls c1. Loopback keeps
  messages already sent toward the open end after the other end drops (link lib.rs Drop clears only its own side).
- det-u4.log 01:08:04-01:11:41 UTC; ruling 3's 8 (A) mirror.rs/scenario.rs changes written 01:44-01:49
  (u4r-build.md:1215-1222); regen-precheck.log mtime 2026-11-13 20:27. c5-end1/c5-end2 hold no runner regeneration.
- record() (scenario.rs:578-602): format! then split_whitespace; hash.rs:271-282 takes profile as any string.
  parse refuses --lead/--every-step only with record.is_some() (:1377); record() sets record_mem after parse.
  Manifests: det SS-P2-s1, ES-P3-s2; precheck SS-P0-s4; all (10+290)x30 = 9,000 = ticks, span 2,000, bytes
  16.69-16.95 MB; recordings.json's synthetic up to 16,700,849 B.
- parse `num(..)? * hz` unchecked (:1322-1324); Cargo.toml release overflow-checks only for npt-sim, npt-proto.
- c-u4r-runs.mjs: bin mkdir (:127) and three builds before any floor(); logs mkdir (:131) before the first floor
  (:146/:180); exists-check on logs/results (:123). floor() checks free >= 4 GiB only. c-u4r-r2.mjs: out/bin mkdir
  before builds, exists-check on out. step() no timeout. Both call main() at import.
- u4r-run.sh:63/:68 and main-run.sh:38/:43 use df -BG; now df -B1 5,021,802,496 B (4.68 GiB) reads "5G".
  The 4 GB floor is the advisor's W1-TARGET rule (advisor-u4-wave1.md:577), not Mustafa's words.
- check.mjs:694-698 metavariable rule matches only /#\s*!?\s*\[\s*\$/; :1495 profile refusal names the file only.
- check.test.mjs:2901-2923 node child at 200 ms. det-u4.mjs:30 steps = { "det-u4" } only. Ruling 1 point 1 (4):
  wasmStep "With the defaults it runs and prints exactly what it does now" (so INF-U4R-6's summary change waits).
- Bites::of (scenario.rs:214) schedule = npt_server::BITES.contains; :226 late_command accepted, only precheck reads it.
- precheck.rs:455-462 end counters, unwrap_or_default for a gone slot; verdict :486-504 ignores refused_logged and
  `ended`; LegOutcome derives Default (:121).
- replay.rs:265-279 n from u32 clients, u8 check per slot inside the loop.
- server/tests/schedule.rs:348 test name says three, asserts four.
- bots lib.rs:49-52 the two constants' docs claim time and speed.
- /tmp/npt-lab-tests: 124 folders (my count).
- U4r size: 1,286 insertions in 13 modified files + 6,290 lines in the 13 new = about 7,500 lines for size M
  (U4c 3,130 and U4j 2,860 at M were "not well past"; U4a 5,500 at S-M was flagged). No SIZE-CHECK line for U4r on
  the card or in history.md.

## Block: netcode (7)
- NC-U4R-1 Medium, confirmed (= SEC-U4R-1). Fix now: FP-1.
- NC-U4R-2 Low, confirmed. Fix now (parse refusal + case): FP-6.
- NC-U4R-3 Low, confirmed; FP-1 makes its gone-slot case reachable on the close path. Fix now with T-6's verdict
  half: FP-4 (VOID on refused_logged and on any session ended on a leg).
- NC-U4R-4 Low, confirmed (five names reported; -58 counts >= 6). Follow-up: U4g's brief line.
- NC-U4R-5 Low, confirmed (= EA-U4R-7's logs half). No U4r run replaces; fails closed today (crash footer or
  coverage gap). Follow-up: the first unit with a logged replacement (U4f hostile clients / U6 reconnect).
- NC-U4R-6 Low, confirmed. Fix now in the script edit: FP-5 (g).
- NC-U4R-7 Low, confirmed. Docs now (FP-7) + report line under carry 39; derivation by carry 18's route: follow-up.

## Block: test (7)
- T-1 Medium, confirmed from code (reasoned, not run). Fix now: FP-2 (whole counters compare + plant).
- T-2 Medium, confirmed (= INF-U4R-1). Fix now: FP-8 (both regenerations at the final tree, labelled).
- T-3 Low, confirmed. Fix now: FP-6 (sh child).
- T-4 Low, confirmed. Fix now: FP-6.
- T-5 Low, confirmed (against my ruling 2 point 32). Fix now: FP-6.
- T-6 Low, confirmed. Verdict half now (FP-4); contract half follow-up (next unit editing contract.rs).
- T-7 Low, confirmed: a correction of my ruling 4's further finding. Fix now: FP-5 (i); ruling 4's words corrected.

## Block: security (7)
- SEC-U4R-1 Medium, confirmed; link-closed half accepted. FP-1.
- SEC-U4R-2 Medium, confirmed (= RR3-M1). FP-3.
- SEC-U4R-3 Low, confirmed (= RR3-L1 part). FP-3.
- SEC-U4R-4 Low, confirmed (= EA-U4R-6). r8 Kick assert + doc now (FP-2); U6 half follow-up (U6 carry).
- SEC-U4R-5 Low, confirmed (rule text read; the probe's rustc build is the seat's). FP-6.
- SEC-U4R-6 Low, confirmed. FP-6.
- SEC-U4R-7 Low, confirmed. FP-5 (h).

## Block: arch (9)
- EA-U4R-1 Low, confirmed. Docs + report line now (FP-7); one shared loop: follow-up before the pre-check re-run.
- EA-U4R-2 Low, confirmed. Report names the deviation from U4-SEAM; refactor follow-up (same lane as EA-1's).
- EA-U4R-3 Low, confirmed (= RR3-L8). FP-6.
- EA-U4R-4 Low, confirmed. Doc cite + report now (FP-7); pub npt-sim item follow-up.
- EA-U4R-5 Low, confirmed. Doc line now (FP-7); one route follow-up (U6 brief).
- EA-U4R-6 Low (= SEC-U4R-4). FP-2.
- EA-U4R-7 Low, confirmed: owner half fixed by FP-1; logs half = NC-U4R-5 follow-up.
- EA-U4R-8 Low, confirmed. FP-7 + report.
- EA-U4R-9 Low, confirmed. FP-6 rename.

## Block: rust (9)
- RR3-M1 Medium, confirmed. FP-3 (profile by name, argv as separate items; the record_mem refusal by construction).
- RR3-L1 Low, confirmed. FP-3 (relation, span_ms, bytes cap 64 MiB, checked parse).
- RR3-L2 Low, confirmed. (b) doc now (FP-7); (a) follow-up (7 MB bounded).
- RR3-L3 Low, confirmed. FP-3 (checked arithmetic in parse incl. the last burst's tick).
- RR3-L4 Low, confirmed against plan.md:630-631. FP-6 (cfg gate).
- RR3-L5 Low, confirmed (124). Report words (FP-9).
- RR3-L6 Low, an estimate. Follow-up: measured inside ruling 1 point 3 (4)'s judge timing.
- RR3-L7 Low, confirmed. FP-6.
- RR3-L8 Low (= EA-U4R-3). FP-6.

## Block: infra (11)
- INF-U4R-1 Medium (= T-2). FP-8.
- INF-U4R-2 Medium, confirmed. FP-5 (a).
- INF-U4R-3 Medium, confirmed. FP-5 (a)-(c).
- INF-U4R-4 Medium, confirmed (5G shown at 4.68 GiB). FP-0, the main session (approved here).
- INF-U4R-5 Low. FP-6.
- INF-U4R-6 Low. Follow-up (U4h's brief): wasmStep's default print must stay as ruling 1 point 1 (4) set it.
- INF-U4R-7 Low. Report words (FP-9).
- INF-U4R-8 Low. FP-5 (d).
- INF-U4R-9 Low. FP-5 (e).
- INF-U4R-10 Low. (1) and (3) now (FP-6); (2) follow-up (pid-reuse weighed).
- INF-U4R-11 Low. FP-5 (f).

## Round 2
netcode-architect, security-auditor, test-engineer (batch 1); rust-reviewer, infra-engineer (batch 2); xhigh; on the
diff from snapshot 572c4f69 to the fix pass's snapshot plus the fix pass's logs. engine-architect sits out (its items:
EA-3 by rust-reviewer; docs and the rename by the main session's read) unless the fix pass adds a pub item, a
dependency or a seam change.

## Mustafa
- SIZE-CHECK owed (his words, history.md:398-399): U4r about 7,500 lines for size M. Three lines before the fix pass.
- Disk heads-up only (ruling 1 point 3 (5) already routes it to him if the set cannot fit).

## Progress
- 2026-11-14: all 50 findings ruled; result returned via StructuredOutput.
