# Step 3d, R1+R4: the verifier's report (verbatim)

Workflow run wf_5519b651-63d (prompts `prompts/verifier-r14-3d.txt`, then `prompts/verifier-r14-3d-skeptic.txt`), launched 2026-10-30 20:39 UTC, returned about 20:57 UTC. Notes: `verifier-r14-3d.md`; hit judgments `verifier-r14-3d-hits.tsv`.

VERDICT: PASS WITH NOTES

Only nits remain. Five in-context glossary hits go back to the owner (engine-architect). Three source-sample cells go back to the session's UNREAD_SOURCES script.

**Gate:** not run. This is a research check, and the prompt's rule 1 says the main session runs the gate.

**Symbol diff:** none. No code changed (docs `r1-simulation-core.md` and `r4-physics-navigation.md` only, 7c6e0d7..f67245e, identical at HEAD and in the working tree).

**Check 1: every change in the diffs**
- **Resolution changes:** my own script (scratchpad/3d-r14w/chk.py, link markup and GLOSSARY marks normalized) finds all 50 R1/R4 "before" quotes exactly once in 7c6e0d7.
  - Every approved "after" text is in f67245e.
  - Two exceptions differ only by logged glossary renames:
    - C9.4 at r1:2046: "tick time" became "sim step time".
    - C9.8 at r1:2841: "the tick budget" became "the server tick budget".
  - C11.6 (r1:2659-2661) and C11.13 (r4:666-667) were checked by hand: present.
  - C17.b1 is not applied, which is right: it waits for BASELINE-WHEN.
  - The owner logged no OBJECT, and no approved R1/R4 change is missing.
- **Shared questions:** word for word as approved. R1 has ABL-PREDICT, SEC-SPREAD, MODES-3 and BASELINE-WHEN; R4 has MODES-3.
- **Cross-topic questions:** R1-XT-OQ1–18 and R4-XT-OQ8 match cross-topic-oqs.md, with the right ids and ranks. The "at commit 8dee746" nits are present.
- **Pointer lines:** every R1/R4 recommendation named as Side A, Side B or Also named has one. Nit: three recommendations named only inside a why/decided-by clause have no pointer (R1-BND-REC2 for XT-OQ16 and XT-OQ17, R1-DET-REC6 for XT-OQ2, R4-NAV-REC4 for XT-OQ5).
- **Headers and part 7:** the header lines, id-table rows, totals (R1 77→99, R4 23→32) and the part-7 "Consistency resolutions" subsections are as the step asked.
- **Unlisted but acceptable (nits):**
  - R1-XU-OQ2 restates R1-ABL-REC4 to match C5.1.
  - The R4 legacy-facts wave-1 line gains GLOSSARY's "5 % of the tick period" along with the rename.
  - The R1 security pointer gains "(R6-SEC-OQ1)".
  - ECS-REC1 mark 1's label "Tick cost" became "Sim step time".
  - Renames were applied inside R1-XU-OQ1's record of the old marks.
- **Stale "Rests on" lines (nits):**
  - R4-NAV-REC2 still rests on "R1-TICK-REC2 (the P1 scene), R1-XU-OQ1".
  - R1-ECS-REC1 mark 1 now uses R2's count of 15 big vehicles without naming R2 in its Rests on.

**Check 2: the recommendation changes**
- All of R1 C1.1, C1.5, C5.1–C5.4, C6.1, C6.2, C9.1, C9.3, C9.7 and C29.1–C29.3 hold. So do R4 C1.15–C1.18 and C9.18–C9.20.
- The four marks the p99 rule changed also hold: R1-ECS-REC1 marks 2 and 3, R4-PHYS-REC1 mark 5 (now the largest mode's N), and R4-OBST-REC2's flip cost. No value changed; each is a proposed mark.
- One-off deadlines left without a statistic (rebuild ≤ 10 s, recovery within 1 s, BVH build ≤ 50 ms, bake ≤ 1 s / ≤ 3 s): the owner's reading holds. Nit: GLOSSARY's time-statistics definition should say that a single event has no p99.
- Resolution 1's table, recomputed independently:
  - N = 48 and the 25 % option: every figure matches.
  - N = 54 at 60 Hz: matches only if the 0.7 ms interest mark also scales ×1.125, which resolutions.md does not state.
  - N = 54 at 30 Hz: the exact sums are 4.11 / 5.11 / 9.92 / 10.92 ms, against 4.12 / 5.12 / 9.93 / 10.93 in the table (sums of rounded parts; also at GLOSSARY.md:72). 0.01 ms, no conclusion changes. Nit.

**Check 3: new claims, numbers or sources:** none outside the approved texts and the glossary. The part-7 "before" counts were recomputed on 7c6e0d7: R1 has 15 raw hits, which is 13 once two overlaps are removed ("server core" sits inside two longer phrases); R4 has 5. Both match part 7.

**Claims checked**

| id | claim (short) | source | verdict | location and exact words |
|---|---|---|---|---|
| K1 | 15 big vehicles (R1-ECS-REC1 m1, TICK-REC2 P1, R4-PHYS-REC4) | r2 worked estimate | confirmed (an R2 assumption) | r2:1762 "one per squad of 4 (LOOP_V2 §2) + one harvester per team (GE6) \| assumption"; r2:1777 "24v24 or 16v16v16 (goal; design point 48) \| 48 \| 15" |
| K2 | the game-designer's caps replace the counts (R2-BUD-OQ5) | r2 | confirmed | r2:2981 "Decides: game-designer's caps; the spike then tests the cap as the worst case." |
| K3 | what the server-only share holds (R2-SYNC-REC9) | r2 | confirmed | "Snapshot encoding, lag compensation, command handling, interest (…) and the tick's send calls together stay within the server-only share" |
| K4 | how many matches share a vCPU is measured (R2-ROOM-REC2 mark 2) | r2 | confirmed | "(2) 8 such rooms as 8 processes on the reference server: each room's p99 within 10 % of the single-room figure" |
| K5 | reference server = CCX23; minimum laptop (R6-HOST-REC2, R3-BUDGET-REC1) | r6:644-646, r3 | confirmed | "the dedicated CCX23 (4 vCPU, 16 GB, x86-64 AMD)"; "the minimum is one named laptop with integrated graphics of Intel Gen12 (Iris Xe) class" |
| K6 | own weapon state predicted; replay from the authoritative state; Quake III's method; abilities wait for ABL-PREDICT | R2-SYNC-REC3 | confirmed | "own movement, own weapon state, own vehicle … replay from the authoritative state"; "That is Quake III's method (F12, F13)"; "its own abilities, until ABL-PREDICT (below) says otherwise" |
| K7 | spread authority-only, finding H2 | R2-SYNC-REC3 | confirmed | "A seed the client knows would let a cheat cancel the spread through the aim point it sends" |
| K8 | R5-RUSTLINT-REC2 bans the types | R5 file; clippy source (curl raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/disallowed_items.rs, 200) | confirmed | "`disallowed-types` = `std::collections::HashMap`, `std::collections::HashSet`"; clippy: "Denies the configured types in clippy.toml." (DISALLOWED_TYPES, version 1.55.0) |
| K9 | an `#[allow]` with a written reason | doc.rust-lang.org/reference/attributes/diagnostics.html (curl, 200), section lint.reason | confirmed | "All lint attributes support an additional reason parameter, to give context why a certain attribute was added." |
| K10 | REC6's canonical serialization walks the whole state | r1:2223 | confirmed | "after each tick the sim writes a canonical serialization of its whole state" |
| K11 | the MODES-3 quotes | docs/VISION_V2.md:43-44 | confirmed | "two teams with 2–8 players, and three teams with 6–18 players"; "**The goal:** 12v12v12 or 24v24" |

Counts: 11 confirmed, 0 wrong, 0 unsupported.

**Check 4: glossary terms**
- Header links GLOSSARY.md in both files.
- "Retire (exact)" is 0 in both, confirmed by reading. The "whole ticks" hits are a "Keeps" phrase. "the tick budget" survives only inside part 7's 2026-10-30 record, where it quotes the old phrase.
- Every unlinked first literal use has another meaning (R1:209, 293, 518; R4:165, 347, 403, 727).
- In-context hits: all 184 judged, no sampling. 179 right, 5 wrong (R1 129 rows: 127 right, 2 wrong; R4 55 rows: 52 right, 3 wrong).
- The wrong ones, back to the owner. Each uses a bare 48 where the largest mode is meant (54 under MODES-3 (b)):
  1. r1:2279, R1-BND-REC2 mark 4: "a 10-minute match with 48 soldiers".
  2. r1:2736, R1-BND-OQ6: "copy time at N = 48", in a question about the largest mode.
  3. r4:778, R4-PHYS-REC4 spike: "a crowd of 48 soldiers".
  4. r4:877, R4-NAV-REC3: "48 bots crossing in a 6 m choke point".
  5. r4:879, R4-NAV-REC3: "avoidance step costs ≤ 0.2 ms p99 native at 48". Resolution 1 treats this as the avoidance row at the largest mode and scales it ×1.125 at N = 54.
- Nit: r1:1633 (options table) still says "a rule against default-hasher HashMap iteration".
- Nit: the owner's in-context counts (R1 130, R4 56) are one more than the session's check output (129, 55).
- "Used today" check, all six topics (945 rows): no id missing.
  - 621 rows match their block's id directly.
  - The other 324 are either named through their part's descriptive name (legacy facts, options tables, pointers, header, TL;DR, assumptions, cost tables, preface, wave-1 list, MODES-3), or covered by the refresh notes' "Rows left out" reasons (start/goal in other senses 82, Player count 6, Team size 3, Authority 1, and so on).
  - The matcher is heuristic; I read what it could not place by hand.

**Check 5: labels**
- R1 keeps "verified (every recommendation claim)": every changed basis claim is confirmed (K1–K11).
- R4 keeps "spot-checked (50 of 52 key claims)". No "Rests on" list changed in step 3c, and every changed R4 recommendation rests on a finding part 7 already confirms:

| R4 recommendation | confirmed findings it rests on |
|---|---|
| PHYS-REC1 | F1 and others |
| PHYS-REC4 | F7, F8, F13 |
| NAV-REC2 | F4, F19 |
| LOS-REC1 | F1, F3, F4 |
| LOS-REC2 | F1, F2 |
| OBST-REC1 | F1–F3 |
| OBST-REC2 | F4 |

**Check 6: source sample (10 rows):** 7 hold, 3 do not.
- Rows 1, 2, 4, 6, 7, 8 and 9 hold.
- Row 3 (the sohu.com, juejin.cn and gamedev.net write-ups of the Overwatch talk), "no tie found", does not hold. r2:3531 and UNREAD_SOURCES.md:34 list them in R2-SYNC-REC1's hinge row. The cell should read "a recommendation hinges on it (part 1: R2 R2-SYNC-REC1)".
- Row 5 (rapier.rs/docs/), "named by R4-LOS-F4, PHYS-F1, F7, F9, F11, F12, F17", does not hold. Those findings cite guide sub-pages that were read; no finding cites the index (r4:1503: "allowed, not fetched: the docs index"). It is a URL-prefix artefact.
- Row 10 (Intel, AMD and NVIDIA product specification pages), "no tie found", does not hold. r3:1524 and UNREAD_SOURCES.md:50 list them under R3-BUDGET-REC1.

**Evidence:** `git diff` and word diff 7c6e0d7..f67245e. Scratch scripts in /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/3d-r14w/ (chk.py, chk2.py, chk3.py, chk4.py, show.py, used2.py).

**Host audit**
- curl, 2 fetches:
  - raw.githubusercontent.com: HTTP 200, final URL the same.
  - doc.rust-lang.org: HTTP 200, final URL the same.
- WebFetch: 0.

**Not verified**
- R2 and R5 ids were checked against their topic files, not their external sources, except K8 and K9.
- The used-today matcher is heuristic: the rows its patterns covered were not read one by one.

Files:
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r14-3d.md (notes, ends DONE)
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r14-3d-hits.tsv (184 judgments)
