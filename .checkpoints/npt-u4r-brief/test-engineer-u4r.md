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
W2. Write part s3-s6 (carries, build rules, end checks, return list) -> parts/p2.md
W3. Write part s7 judged runs + -30 pre-check + wall time + evidence files -> parts/p3.md
W4. Write part s8 O1 -> parts/p4.md
W5. Write part s9 recheck table + s10 advisor questions + s11 needs-a-run list -> parts/p5.md
W6. Assemble u4r-brief-draft.md from parts (one Write; no Edit tool and no cat available).
END. DONE line + answer.

Tool note: no Edit tool in this run; files are written whole with Write. Draft sections go to parts/ first, then one assembly. Per-block reading notes are in notes-b4.md ... notes-b11.md beside this file; B1-B3 and B5 notes are below.

## Notes
- B1 done: night file read. Item 4 shape = item 2's shape for U4r's lane (crates/lab, crates/bots' library, ops/runs/c-u4r-*.mjs) + judged runs + wall time + evidence files + O1 + recheck table (U4c, U4d2, U4e, U4j) + advisor Qs.
- B2 done: U4e rev 2 format; lessons L1-L7: (L1) every bite's proof must be able to fail under the bite; (L2) edge/alignment preconditions recorded and asserted; (L3) base counts on check.mjs's measure (cargoCounts check.mjs:1321-1331; :1378-1385), `--list` only names; (L4) recheck rows must state a check that settles the claim; (L5) do not forbid what the plan prescribes (scratch script for bites, §7.2 :881-882); (L6) drop questions the plan answers; (L7) "within X of Y, on which clock". U4e brief rev 2: U4E-Q5 (a) (no Rewind, counted, "so the U4r pre-check's setup reads it"); U4E-Q2 (c) (U4r's -58 judges five); U4E-Q7 (U4r writes per-process logs incl. hs, snap_incomplete, burst_eviction); U4E-Q1 (lt-cmd-seq, lt-press by U4e).
- B3 done (U4r §5 :717-772): Builds :718-724; Tests :725-732; judged :733-764; fixes.rs sizes :765-767; Evidence :768-770; Reviewers :771.
- finding: U4r's lane files not stated in Lanes (:837).
- finding: 63,736 = 65,536 - 1,800 (60 s at 30 Hz).
- B5 done: Lanes :814-845; §7.1 :865-877 (item 6 :873-874; item 8 :876-877); §7.2 :879-887.
- B4 done (notes-b4.md).
- B6 done (notes-b6.md).
- B7 done (notes-b7.md): O1 cite drift (:678-679, :787 are pre-condition-1 lines; at df8975b :690-691 and :799-800); 6 runs reading.
- B8 done (notes-b8.md): only carry 40 routed to U4r; carries 6, 14, 20 reach U4r's code by their words.
- B9 done (notes-b9.md): -58's untraced_corrections_count (U4g); -21's option A suites (U4f); -17 clean tick_start open; -30 depth series name.
- B10 done (notes-b10.md): npt-lab has no npt-bots dependency (manifest stop); no lab/bots clippy.toml; judge gap = INVALID.
- B11 done (notes-b11.md).
- extra reads done: §4.5-§4.9 (:282-347): -57 pairing :317-318; HitHistory capacity from U4h's test cap, proposed 64 ticks (:335-336); tolerance and RTT from the pump (:340-345). §2 :141-159: sizes for fixes.rs (:150: L1 B1 SS runs log per-tick snapshot payload + datagram count, each input's payload, Join/JoinAccepted/FullState sizes; series with SERIES.md rows); C4 (:147); -18 split (:149). §3 :162-170: U4-SEAM npt-lab names World once (:163); U4-SERVER npt-bots' U4 half transport-free, clock-injected (:169-170).
- finding: wallclock bite's site is the pump (npt-server: U4e's crate, not in U4e's bite list :711-712, not in U4r's lane) -> question.
- finding: twin test's HashMap bite: equal order digests mean "not exercised" (§4.14 :460's rule) -> apply.
- finding: class audit's team and public rules cannot fire in the spike (catalogue :144-145, :153-154: empty sections) -> report not exercised.
- finding: wallclock bite must show a mismatch on a valid (non-VOID) pair; late-command shown only when VOID by late_count alone (judge's expect-invalid rule eval.rs:1283).
- Next: W1.
