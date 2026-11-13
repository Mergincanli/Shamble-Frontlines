# Advisor: rulings on U4r build copy 1's nine points (+ point 10 follow-ups), 2026-11-13

Seat: a day session's advisor (workflow agent, opus), on main's `.claude/agents/advisor.md` (read with `git show
origin/main:.claude/agents/advisor.md`). Read only: no build, no run, no edit but this file.
Sources read: u4r-build1-questions.md; u4r-build.md (282 lines); the snapshot .gate/u4r-snap1 (2fbec77 + wip/u4r.patch,
`git status` shows the 11 files); the logs .gate/u4r/ (regen-all.log, d13.log, rec.log; file times of regen-all/);
u4r-brief.md (sha256 59d570b8..., 1,909 lines, checked); u4s-carries.md (e8675a02..., 661 lines) carries 62, 82-84, 87;
u4-build-plan.md (1e631c55...) :65-72, :476-504, :750-803; advisor-u4d2-r1.md :209-212; advisor-u4d2-m9.md :105-150;
advisor-u4r-brief.md (verdict, L1-L6); advisor-u4r-fold.md FQ6; u4r-fold-check2.md :46-55; history.md (40ae4c30...,
2,027 lines): USAGE-SAVINGS and the panels (:281-310), the 9.5 floor and "A Low never triggers a push" (:335-340),
LEFTOVER-DELETE (:404-410), the 2026-11-12/13 decisions (:1927-2027); the fact check's result (in the task).

## Plan
1. Read the advisor file, the questions, the notes (done).
2. Read the brief's cited sections, the carries, the plan, the earlier rulings (done).
3. history.md: Mustafa's decisions to quote (done).
4. Check the code in the snapshot for each point (done).
5. Rule points 1-9, point 10, further findings (done, below).
6. StructuredOutput.

## Progress
- [done] 1-4.
- [done] 5: the rulings below.

## Code checked myself (beyond the fact check)
- hash.rs `legs` :823-857 (`selected` then `regenerate_all(&entries, ...)` over the whole manifest); `serve` :954-975
  reads files only; `compare` :1090-1130 regenerates nothing; `generate` :330-344 ignores a runner entry's span_ms;
  `check` :355-365 compares length and SHA-256 only.
- check.mjs `wasmStep` :2254-2450: manifest path fixed (RECORDINGS_MANIFEST via `io.read` and `manifestArgs`); the
  bites half's two legs calls pass `--manifest RECORDINGS_MANIFEST --dir rec`, so each regenerates every entry again.
- regen-all.log :20-27 and regen-all/rec file times: the five "regenerated" lines, 45,000 ticks 0 mismatching,
  real 3m20.992s; SS-P2-s1.rec at 20:30:32.54, ES-P3-s2.rec at 20:31:25.14 after the synthetic three at 20:28:09.99.
- room.rs :29 RADIUS_CM, :33 HISTORY_MS (private); find_relevant :904-966 (radius AND flood for soldiers, hulls,
  projectiles; parts by hull; doors by sees_portal, the wall by reaches).
- mirror.rs :26-28 the copy; audit_records :244-295: `audit.spatial += 1` for every entity record, doors and the wall
  fall to `_ => None` (:281) and `if let (Some(me), Some(at))` (:283) passes them unchecked; breach list capped at 16
  (:236-240).
- logs.rs :132, :275 `refused_as_lost && h == 0`; scenario.rs :198-200 LAB_BITES (bites only), :218, :699.
- scenario.rs :54-56 ROOM_HISTORY_MS; :804-808 the admission bound; :833-835 errors kept only with logs; :884-888
  room_tick only with logs; :982-985 a recording stops at the measured end; :1022 rewinds_refused from ClientCounters
  (logs or not); :1260-1292 the flags; :1344-1375 the summary line (shots and incomplete print 0 with no logs).
- judge: eval.rs:28-34 a run serves a mark only if a header lists it; :310-328 the gap rule (no values at all);
  lib.rs:37-46 reads every log under --logs into memory; log/src/lib.rs:104-121, :534-536 (Log holds every tick).
- seam.rs:543-550: SimCore::step returns the tick's events; the canonical hash includes the event list.
- SERIES.md :66-73 (-57's rows: 66, 67, 73), :91 reliable_events_count and :94 shots_count with writer npt-room.
- Disk now: df -B1 /home/user avail 5,720,961,024 B; .gate/shared-target 8.3G, .gate/netcode-proof-test 8.8G,
  .gate/u4r 291M. Copy 2 has reached B10 (c2-b9-*.log, c2-b10a-*.log, c2-b10c-*.log in .gate/u4r/).
- l1/src/lib.rs: BITE_IDS :746-751; MODEL_BITE_IDS block :753-766 (doc :753-755, cfg :756, array :757-766).
- Brief rev 3.2 line numbers: section 0's "How rev 2's df8975b cites were re-pointed" bullet is :217-218 (the task's
  :210-211); "is now 4e91830's" is :186-190 (the task's :182-186).

## The rulings (each: decision, severity, builder, main session, Mustafa)
(Full texts are in the StructuredOutput result; summary here.)
1. Determinism route: (c) with my ok: runner entries in a new crates/lab/det-recordings.json, ops/runs/det-u4.mjs
   runs wasmStep's clean half on it (wasmStep gains manifest and bites options); recordings.json keeps the synthetic
   three; hash.rs legs unchanged. Margin: check.mjs's step stays only while its cold printed figure is at most 200 s;
   det-u4.mjs's non-cargo calls at most 240 s of carry 84's 300 s shared deadline. Machine: this container (CI runs no
   check.mjs). Medium. Not Mustafa's (plan :800-802: "on the advisor's ok").
2. F1: (a) + a behaviour pin of the radius copy; r4 counts checked/unchecked apart; full breach count; honest words.
   Medium. Not Mustafa's.
3. F3: no change of form; -57's series written only on runs whose header lists PM-0001-57; the main session's disk
   plan (approved cleans only; judge memory measured first; to Mustafa only if it cannot fit). Medium. Not now.
4. F4: the bite becomes "a tick decoded whole filed as lost", id PM-0001-17/whole-as-lost; my words amend carry 62
   (h)'s bite wording (the advisor's U4j R2.4). Medium. Not Mustafa's.
5. F5: keep the copy; a behaviour pin if reachable; hash.rs check compares a recording's header with its entry; a §U4h
   carry. Low.
6. F2: the dump accepted; a test pins the reader against SimCore::step's events. Low. "The 9.5 floor (PM-0001-2) does
   not move."
7. The brawl's view: accepted, worded in the report; a follow-up for U6's brief. Low.
8. D13 (2): a fact; nothing follows; the notes' length line corrected. none.
9. Flags: --lead/--every-step refused with --record; no-log summary prints "not counted"; wall times labelled.
   Medium (the zeros). Not Mustafa's.
10. Four Lows recorded with their homes.
Further: r4's spatial count overstates (Medium); judge memory on the set (Medium); line-number nuances (Low).
- bots/Cargo.toml after L3: npt-net at :25 (was :24); npt-sim :22. The brief's C26 (:173) and the record of U4R-Q1
  (:1565) still say :24 (records; point 10 (4)'s effect).
- [done] 6: StructuredOutput sent.
