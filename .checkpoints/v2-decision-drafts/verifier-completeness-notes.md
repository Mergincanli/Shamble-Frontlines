Task: completeness critic (plan step 8) of the v2 decision drafts: what is missing. Started 14:00 Amsterdam 2026-11-01, deadline 15:30.

## Check 2 (draft shape, template fields): MET (14:10)
- All six drafts carry every heading of prompt_common.md's shape (Context, Decision, Options considered, Trade-off analysis, Consequences, Open questions, What a spike must prove, ### Pass marks, Migration plan, Test plan, Rollback, Appendix A, Sources), none empty (smallest: 0004 Migration plan 13 nonblank lines, 0003 Rollback 11).
- "Hard constraints in CLAUDE.md that change": 0001:824, 0002:759, 0003:659, 0004:724, 0005:1745, 0006:587; each names constraints + SPIKE-SCOPE (a) / LOOK_V2 §8. Legacy inventory rows tables: 0001:834, 0002:781, 0003:672, 0004:734, 0005:1758, 0006:596.
- 0005 has 4 extra section headings (content data, abilities, asset pipeline, GPU rules) between Decision and Options: allowed by C11 (frame).
- Minor: author line credits test-engineer for the Pass marks table in 0001, 0002, 0006 but not 0003:5, 0004:5, 0005:5 (the Pass marks section itself says "Written by test-engineer" in all six). Session aligns at assembly (checkpoint notes this for 0002 only).
## Check 5 (VISION_V2 §6's ten items in ENGINE_OVERVIEW's table): MET (14:13)
- ENGINE_OVERVIEW.md lines ~99-110: rows 1-10, each with draft + part; [OPEN] seats (§6.4), [LATER] kill-cams (§6.8), [LATER] accounts (§6.10) say "no feature" + what keeps it possible (C8).
## Check 1a (ENGINE_OVERVIEW): MET with a note
- 133 lines (C19 <= ~150). Mermaid diagram: parts + per-tick edges (command up, block down, snapshots); boundary spelled out in text below ("The Rust <-> TypeScript boundary"); per-tick flow (4 steps) and per-frame flow in prose; §6 in a table. Note: the brief says "a diagram showing" all three; the diagram itself shows parts and per-tick flow only, per-frame flow and §6 placement are prose/table. Low.
- Cites register anchors adr/README.md#codes-the-drafts-cite, #needs-outside-vision_v2-6, #decisions-the-drafts-ask-for: check they exist.
## README register read (14:17)
- docs/adr/README.md: index (6 drafts, 88 OQs), Decisions the drafts ask for (l.25-104), Design questions (38 DESIGN ids, l.105), Loose-ends register (l.147: CF-1..CF-5, critic items + answered escalations, 28 XT ranks, OLD-ADR/ADR-001/002/003, SEC-MDL-2 STATUS:257, CDN-SRI :262, WALL-CLOCK :273, LE-A..LE-P), Needs outside §6 (l.235), Codes the drafts cite (l.260: I-1..I-21, S1,S3,S4,S6,S12, G1-G5,G7-G9,G11,G15, N-subset, J1-J6).
- PR #28 finding 4's condition 13 closed: register row present.
## Check 1c (SPIKE_PLAN): MET (14:20)
- docs/SPIKE_PLAN.md 385 lines: table l.~72 with 10 spikes + foundation checks + asset-pipeline research; each with size (S/M/L defined l.17), place (waves), pass-mark ids; netcode proof test §1 l.87, boundary §3 l.151, look §5 l.212 (Mustafa judges, LOOK_V2 items beside legacy screenshots), preview hosting §6 l.242 with PREV-DELETE l.246, asset-pipeline research l.328 before asset spike §9, Feel Lab §10 l.357; research spikes subsumed (carry-forward 5) l.~118.
## GLOSSARY §8 (C3): MET
- GLOSSARY.md l.744-797: heading exact, 3 entries (foundation, proof test quoted word for word from brief §2, 60 Hz margin with Depends on / Marked by / Retire), closing note on line 85. Base 9e7db28 is a byte prefix of today's file (checked by cmp).
## Write guard (C1): met locally (no fetch run by me)
- git diff --name-only 9e7db28...HEAD (HEAD a4cef71): only allowed paths; research files r1-r6, UNREAD_SOURCES, LEGACY_INVENTORY unchanged vs 9e7db28; origin/main = merge base = 9e7db28. Gate (step 12): not run yet -> pending.
