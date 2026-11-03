# touchup4 engine-architect fixes (fixcheck-d3 findings D3C-1, D3C-2, D3C-3, D3C-4)


### FIX D3C-2
FILE: docs/adr/README.md
BEFORE:
| S6 | Moments and telemetry | 0004 recognizes moments and emits events; 0005 holds moment rules and telemetry ids as data; 0001's room writes the record | 0006: the record's shape, storage, analysis and retention |
| S11 | Offline and local play | DESIGN: whether offline play ships, and at the largest mode on the minimum laptop (R1-BND-OQ1, R1-TICK-OQ7, critic rank 15) | 0002 and 0004: the local server from slice 1 and local play with bots in slice 2 (SLICE-ORDER), whatever the answer |
AFTER:
| S5 | Content vs execution vs drawing | 0005: the definitions, their schema, validation, versions and hot reload, the bundle and its content hash, the ability data model and in-match upgrades, the asset pipeline and per-unit budgets | 0004: running them in the sim step (order, integer time, attribute maths, damage, the player record that holds the upgrade tracks); 0003: drawing them (materials in TSL, LOD switching at run time, faction colour as per-instance data) |
| S6 | Moments and telemetry | 0004 recognizes moments and emits events; 0005 holds moment rules and telemetry ids as data; 0001's room writes the record | 0006: the record's shape, storage, analysis and retention |
| S9 | The look vs the asset rules | 0003: how every LOOK_V2 item is achieved on screen | 0005: what an asset must carry for it (authored silhouettes and proportions, material slot names, faction colour and emissive slots, LOD levels, per-unit budgets) |
| S10 | The editor | 0002: the editor package, its entry and the double-click question (OQ-0002-1) | 0005: the map document as content data (schema, versions, migration); 0004: the reach checks and the navigation bake the editor's validation panel runs |
| S11 | Offline and local play | DESIGN: whether offline play ships, and at the largest mode on the minimum laptop (R1-BND-OQ1, R1-TICK-OQ7, critic rank 15) | 0002 and 0004: the local server from slice 1 and local play with bots in slice 2 (SLICE-ORDER), whatever the answer |
END

### FIX D3C-4a
FILE: docs/adr/README.md
BEFORE:
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 20 (OQ-0003-1 to -20) |
AFTER:
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 20 numbered (OQ-0003-1 to -20), 19 open: -14 answered by its decider, the verifier |
END

### FIX D3C-4b
FILE: docs/adr/README.md
BEFORE:
Total: 114 numbered open questions, 112 of them open (OQ-0004-5 and -6 point to ADR 0005); SPIKE_PLAN adds one of its
own, OQ-SPIKE-1. Pass marks: 62, 78, 47, 75, 46 and 32 rows (340). None of the drafts is signed off; no spike starts
before Mustafa and the Advisor approve them (the drafts brief, §4).
AFTER:
Total: 114 numbered open questions, 111 of them open (OQ-0004-5 and -6 point to ADR 0005; OQ-0003-14 is answered);
SPIKE_PLAN adds one of its own, OQ-SPIKE-1. Pass marks: 62, 78, 47, 75, 46 and 32 rows (340). None of the drafts is
signed off; no spike starts before Mustafa and the Advisor approve them (the drafts brief, §4).
END

### FIX D3C-3
FILE: docs/adr/README.md
BEFORE:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes, for example ADR 0002's gate checks G0–G13, ADR 0004's spike claims S1–S22, ADR 0006's
option labels (G1–G4, K1–K4, S1–S5) and its baseline items B1–B12 (part 13); a register code always carries its word
("gap G5", "seam S1").
AFTER:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Review codes
(GD-REJOIN, GD-PICK-VIS, GD-CARRIER, RELEASE-WINDOW, LOOK-FLOOR, REDUCED-MOTION, COLOUR-TELL, HEAT-SOAK, POOL-FULL,
EA-8, SA-10, RR-10) and "the X review, finding n" (X a reviewer's short name, as in "the art review" or "the Rust
review") point into the ten reviews on the final diff (plan step 10, 2026-11-02): the same appendix's §7.2 summarizes
each, and their files stay readable in the branch's history (`.checkpoints/v2-decision-drafts/review-*.md`). Codes local
to one draft are not register codes, for example ADR 0002's gate checks G0–G13, ADR 0004's spike claims S1–S22, ADR
0006's option labels (G1–G4, K1–K4, S1–S5) and its baseline items B1–B12 (part 13); a register code always carries its
word ("gap G5", "seam S1").
END

### FIX D3C-3b
FILE: docs/adr/README.md
BEFORE:
| OQ-0003-20 | Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop? (the test review's HEAT-SOAK) | the Advisor | [ADR 0003](0003-rendering.md) line 938 |
AFTER:
| OQ-0003-20 | Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop? (the perf review's HEAT-SOAK) | the Advisor | [ADR 0003](0003-rendering.md) line 938 |
END

### FIX D3C-1
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
5. **Slice 3, online:** `server` and `supervisor` enter; L2 in the nightly run; G12 over both transport backends; the
   first multiplayer
   preview behind the access wall, with the binary-delivery job once the security baseline has set its token's scope
   (BASELINE-WHEN (b): before the first online playtest); the Sentry upload job if Sentry is set up. *Done when:* G12
   passes on both backends and a multiplayer preview plays against the staging game server.
AFTER:
5. **Slice 3, online:** `server` and `supervisor` enter; L2 in the nightly run; G12 over both transport backends; the
   first multiplayer preview behind the access wall, with the binary-delivery job once the security baseline has set its
   token's scope (BASELINE-WHEN (b): before the first online playtest); the Sentry upload job if Sentry is set up.
   *Done when:* G12 passes on both backends and a multiplayer preview plays against the staging game server.
END

### NOTE D3C-residue (what these blocks leave to others, by the task's scope)
- D3C-3b is new: found while checking D3C-3. README:81 said "the test review's HEAT-SOAK", but the code is the perf
  review's (review-perf-auditor.md:86, "Q4. HEAT-SOAK (finding 7)"; final-round-by-author.md:113, "PA HEAT-SOAK");
  test-engineer B only wrote OQ-0003-20 from it (test-engineer-B-fixes-final.md:392, "perf review's question
  HEAT-SOAK"). Without D3C-3b, the new sentence in D3C-3 would send a reader to the wrong review file.
- README:87 "the reviews' POOL-FULL": the code is the Rust review's Q1 (review-rust-reviewer.md:72), passed to
  game-designer; not wrong, left as it is.
- D3C-4: the report's line 26 "(112 open)" is the session's text (fixcheck-d3c, task 3): it becomes "(111 open)".
- D3C-3's other half, the decision map's parts a and g (0005:2103 "decision map, part g"; 0005:2500 "the decision
  map's parts a, d, g and j"): not changed, since the task keeps D3C-3 to one sentence. 0005:2500 already says these
  process inputs "leave the branch with the checkpoint", and part g's spikes now live in SPIKE_PLAN.
- D3C-1's other half (0001:1518, 156 characters) is test-engineer A's text.
END
