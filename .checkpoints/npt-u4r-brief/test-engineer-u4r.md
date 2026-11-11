Task: npt-u4r-brief rev 1 -- test-engineer drafts U4r's builder brief (night item 4, approved in issue #16), notes only.

## Plan (blocks, in order)
B1. Read the approved plan (night-2026-11-10.md at 2dc1f31: rules + item 4 + item 2's shape).
B2. Read the format model: U4e brief rev 2 and the test-engineer check of rev 1 (at 0dccbf0).
B3. Read u4-build-plan.md §5 U4r (:717-772).
B4. Read u4-build-plan.md §4.11-§4.15 (:379-488).
B5. Read u4-build-plan.md Lanes (:814-845) and §7 (:864-888).
B6. Read §5 of U4c, U4d2, U4e, U4j (what U4r uses).
B7. Read advisor-u4.md (conditions 2, 3, 5, 9; O1 :343-347; Q10; Q21).
B8. Read u4s-carries.md §U4r (:176-182, carry 40) and any other carry routed to U4r.
B9. Read marks.json; the catalogue (relevant parts).
B10. Read ops/runs/c-u2-pm-0001-3.mjs; crates/lab and crates/judge (what exists).
B11. Read advisor-u2.md; advisor-u3q.md:180-186; .claude/rules/netcode.md.
W1. Write part s1-s2 (Start, What to build) -> parts/p1.md
W2. Write part s3-s5 (carries, build rules, end checks) -> parts/p2.md
W3. Write part s6 judged runs + -30 pre-check + wall time + evidence files -> parts/p3.md
W4. Write part O1 -> parts/p4.md
W5. Write part recheck table + advisor questions + needs-a-run list + return list -> parts/p5.md
W6. Assemble u4r-brief-draft.md from parts (one Write; no Edit tool and no cat available).
END. DONE line + answer.

Tool note: no Edit tool in this run; files are written whole with Write. Draft sections go to parts/ first, then one assembly. Per-block reading notes go to notes-bN.md beside this file (B4 onward); B1-B3 and B5 notes are below.

## Notes
- B1 done: night file read. Item 4 shape = item 2's shape for U4r's lane (crates/lab, crates/bots' library, ops/runs/c-u4r-*.mjs) + judged runs + wall time + evidence files + O1 + recheck table (U4c, U4d2, U4e, U4j) + advisor Qs.
- B2 done: U4e rev 2 format; lessons L1-L7: (L1) every bite's proof must be able to fail under the bite; (L2) edge/alignment preconditions recorded and asserted; (L3) base counts on check.mjs's measure (cargoCounts check.mjs:1321-1331; :1378-1385), `--list` only names; (L4) recheck rows must state a check that settles the claim; (L5) do not forbid what the plan prescribes (scratch script for bites, §7.2 :881-882); (L6) drop questions the plan answers; (L7) "within X of Y, on which clock". U4e brief rev 2: U4E-Q5 (rewind_at outside ring before U4h: (a) no Rewind, counted, "so the U4r pre-check's setup reads it"); U4E-Q2 (c) (lt-tick-offset judged run waits for U4g: U4r's -58 judges five); U4E-Q7 (U4r writes per-process logs incl. hs, snap_incomplete, burst_eviction); U4E-Q1 (lt-cmd-seq, lt-press planted by U4e).
- B3 done (U4r §5 :717-772): Builds :718-724; Tests :725-732; judged :733-764; fixes.rs sizes :765-767; Evidence :768-770; Reviewers :771. Full line map is in the draft's sources; key lines: --bite :719; per-process logs :719-720; replay :720-722; four recordings :722-723; -57 pairing :723; bots :723; run scripts :723-724; twin :725; contract :725-728; replay test :728-730; class audit :731; ticks/s + wall time :731-732; -17 :734-735; -57 :736; -58 :737; -18 :738-739; -30 :740-764.
- finding: U4r's lane files not stated in Lanes (:837); night plan gives crates/lab, crates/bots' library, ops/runs/c-u4r-*.mjs.
- finding: 63,736 = 65,536 - 1,800 (60 s at 30 Hz).
- B5 done: Lanes :814-845; §7.1 :865-877 (item 6 :873-874; item 8 :876-877); §7.2 :879-887.
- B4 done (notes-b4.md): findings (a) npt-lab/npt-bots clippy.toml clock ban (§4.11 :380-383), no unit named, vs measured ticks/s; (b) §4.15 :481-483's P0 argument is the tolerance's (U4h) - O1; (c) TC B1 recording (:479-480) needs a TC behaviour (TC shooter is U4h's :801); (d) runner recordings regenerated at check time (:485-487) cost; (e) -57's bites not §4.14 rows (Quant, U4a).
- B6 done (notes-b6.md): finding (f) Evidence :769-770 routes the runner recordings through check.mjs or ops/runs/det-u4.mjs: both outside the U4r lane as the night plan names it; who writes the recordings manifest (:485-487). Next: B7.
