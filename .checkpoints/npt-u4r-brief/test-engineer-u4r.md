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

Tool note: no Edit tool in this run; files are written whole with Write. Per-block reading notes are in notes-b4.md ... notes-b11.md; write-block notes in notes-w.md.

## Notes
- B1-B11 done (see notes-bN.md). Extra reads done: plan §4.5-§4.9, §2, §3; check.mjs steps and bite-home rule; judge eval.rs gap rule.
- W1-W6 done (see notes-w.md). Deliverable: u4r-brief-draft.md (rev 1).
- Findings recorded in the draft as questions (U4R-Q1..Q12): manifests npt-lab -> npt-bots (lab/Cargo.toml:23-34; bots/Cargo.toml:20-25); out-of-lane files (SERIES.md, recordings manifest, check.mjs/det-u4.mjs); clippy.toml clock bans (§4.11 :380-383) vs ticks/s; contract test scope vs option A suites (marks.json:4631-4704; plan :774, :776); -58's untraced corrections (marks.json:4162-4169; plan :786; judge eval.rs:310-323); wallclock bite's site in npt-server (plan :759; :711-712); -17 clean tick_start; -18 shared runs; TC B1 recording (plan :801); early-input bite for the RTT rule; contract test negative cases; the re-run's home.
- O1 finding: advisor :344's cites :678-679 and :787 do not hold at df8975b (U4d2 bites; U4g Predictor); the words stand at :690-691 and :799-800. Advisor :343's "u4-conditions-check.md:39-42" is :42-45 at df8975b.
DONE
