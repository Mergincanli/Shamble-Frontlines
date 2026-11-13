# Skeptic, round 1: D14 (= EA-3, Low), head 4d78ad6

Plan: read every source EA-3 rests on at 4d78ad6 through `git show 4d78ad6:<path>` (sim-wasm's lib.rs, the report's
EMPTY-SLOTS, STATUS_ARCHIVE's EMPTY-SLOTS, ADR 0002's OQ-0002-7 and every other place in the ADR that names wasm-bindgen,
#4533 or "by-day", plan.md Part 3, the register in docs/adr/README.md); then sweep the lasting record (everything
tracked outside .checkpoints/: STATUS, ROADMAP, DECISIONS, SPIKE_PLAN, research, the other ADRs, REPO_MAP) for a carrier
of the by-day reading the finding says is lost; then weigh the claim, the severity and the fix's wording against the
ADR's own text. Work copies, if any, go in /home/user/Shamble-Frontlines/.gate/skeptic-D14/. No file in the worktree
changes except this one.

## Checks
1. sim-wasm's doc line at 4d78ad6 (`git show 4d78ad6:crates/sim-wasm/src/lib.rs`, lines 1-2): "Empty, with no
   bindings, until the boundary spike reports (OQ-0002-3, OQ-0002-7)." Quoted correctly by EA-3. The manifest has no
   dependency at all (no wasm-bindgen yet).
2. ADR 0002 at 4d78ad6, OQ-0002-7 (lines 1077-1085): "Decider: a by-day reading of its maintenance issue (#4533) and
   the guide's governance page, both on the allow list (VERIFY-FETCH (a)); this draft fetched nothing." Quoted
   correctly (EA-3 says "settles ... with"; the ADR says "Decider": the reading decides, it has not settled it). The ADR
   sets no time for the reading: the OQ names no step, and grep of the ADR for 4533, by-day, governance, VERIFY-FETCH and
   R1-BND-OQ7 finds only 175/178 (part 2's ids), 1079-1085 (the OQ) and 1557 (the research map: "R1-BND-OQ7 | open:
   OQ-0002-7 | part 2 (the pin is the hedge)"). Line 1388 (spike section): "OQ-0002-7 (wasm-bindgen's health) is a
   reading, not a measurement." So the ADR itself says the spike does not decide OQ-0002-7.
3. plan.md at 4d78ad6, Part 3 row 1 (line 253): "... decider: 'a by-day reading of its maintenance issue (#4533) and
   the guide's governance page') and R5-CI-REC3 | `sim-wasm` has no bindings until the boundary spike fixes its
   exports; the by-day reading is done before the step that adds wasm-bindgen". Quoted correctly. (Its ADR line refs,
   `0002:1063-1071`, are stale against 4d78ad6's 1077-1085; not EA-3's point.) The timing ("before the step that adds
   wasm-bindgen") is the plan's, not the ADR's.
4. Report at 4d78ad6, lines 185-186: "EMPTY-SLOTS: OQ-0002-7 and the wasm-bindgen pin, OQ-0002-3, OQ-0002-4,
   OQ-0002-1, the boundary spike's items 4 and 8: each waits for the boundary spike." STATUS_ARCHIVE at 4d78ad6, lines
   634-637: the same list, "each waits for the boundary spike", BUT it opens with "(2.2's plan, Part 3, readable at the
   branch's last checkpoint commit)": the archive points to the plan's Part 3, so the timing sentence stays reachable
   through git history if that commit stays reachable (checked below). The report's line has no such pointer.
5. The register (docs/adr/README.md at 4d78ad6): OQ-0002 rows exist for -1, -4, -5, -6, -8 (line 74) and -9; none for
   -2, -3 or -7 (EA-3's "no -7" is literally true). But the section's own scope line (line 41-43): "Every decision code
   and open question the drafts route to Mustafa or the Advisor, one row each. ... Open questions decided by a spike
   alone, or by an agent, are in the drafts and in SPIKE_PLAN, not here." OQ-0002-7's decider is a reading a session
   does, not Mustafa or the Advisor, so a missing row is the register working as designed (as for -3, a spike question).
   The drafts table (line 17) still counts it: "9 numbered (OQ-0002-1 to -9 ...), 8 open". EA-3's register leg is not a
   defect of this PR.
6. Sweep of the tracked tree at 4d78ad6 outside .checkpoints/ (`git grep` for 4533, "by-day", "governance"): besides ADR
   0002:1082, the reading also lives in docs/research/r1-simulation-core.md:2741-2744 (R1-BND-OQ7: "Decides: reading
   the maintenance issue (#4533) and the guide's governance page in the synthesis; pinning versions limits the risk
   meanwhile."), docs/research/UNREAD_SOURCES.md:26 and :132 (#4533 as the source R1-BND-REC4 hinges on, "on the allow
   list; not read in this run"), and r1:3222, :3986 (unread-source tables). So EA-3's "only ADR 0002:1082-1084 carries
   the by-day reading" is too strong: three lasting docs name the reading. What only the plan carries is its TIME
   ("before the step that adds wasm-bindgen"); the ADR, r1 and UNREAD_SOURCES set none (r1 said "in the synthesis",
   which did not fetch it: ADR 0002:1083 "this draft fetched nothing").
7. The ADR's migration plan (0002:1431-1500) names no step for the reading; 2.4 (generated types: wasm-bindgen's
   `.d.ts` is one of the three generators) and 2.5 (the skeleton, sim-wasm's bindings) are where wasm-bindgen would
   enter. The ADR's "What has no measurable mark here" list (0002:1371) holds line 1388: "OQ-0002-7 (wasm-bindgen's
   health) is a reading, not a measurement." So the ADR says the boundary spike does not settle OQ-0002-7, and the
   wording EA-3 flags (EMPTY-SLOTS' "each waits for the boundary spike" covering "OQ-0002-7 and the wasm-bindgen pin";
   sim-wasm's "until the boundary spike reports (OQ-0002-3, OQ-0002-7)") does pair OQ-0002-7 with the spike. That part
   of EA-3 holds, as an imprecise pairing.
8. Pointers to the plan in the lasting record: the report at 4d78ad6, line 39: "The plan:
   `.checkpoints/foundation-toolchain/plan.md` (readable at this branch's last checkpoint commit, named in the PR
   body)"; the archive's EMPTY-SLOTS (line 634) opens "(2.2's plan, Part 3, readable at the branch's last checkpoint
   commit)". Main merges with merge commits (`git log origin/main --first-parent`: every PR a "Merge pull request #N";
   333 checkpoint commits reachable from origin/main), so the branch's checkpoint commits, and plan Part 3's timing
   sentence with them, stay readable from main after this PR merges. EA-3's "once the plan is deleted, only ADR
   0002:1082-1084 carries the by-day reading" does not hold: the ADR (the record of open questions), r1 and
   UNREAD_SOURCES carry the reading, and both follow-up homes point to the plan that carries its time.
9. "By-day" = VERIFY-FETCH (a) (docs/DECISIONS.md:872, the Advisor 2026-10-30: research may read allow-listed pages,
   with curl by day; the v2 research report :28). So the reading is a day session's fetch; routines cannot do it.
10. What the fix would touch: report at 4d78ad6, lines 118-119: "the pins, lockfiles, crates, packages and `.gitignore`
   are unchanged since step 2 (566363f), but for the two overflow tests' expected message (70f1c31)". A sim-wasm doc-line
   edit makes "crates ... unchanged" need one more "but for"; EA-2's fix names the same knock-on, and EA-3's fix rides
   EA-2 (D13), which rides D1's push. PR #48 (API read): open, draft, head ed393f0 (4d78ad6 + checkpoint commits only;
   `git diff --stat 4d78ad6 HEAD -- . ':!.checkpoints'` is empty).

11. After the advisor consult: plan.md is byte-identical at 4d78ad6 and HEAD (`git diff --quiet`), row 253 with "the
   by-day reading is done before the step that adds wasm-bindgen" intact. Check 8's durability is conditional: the
   report's and archive's pointers name no SHA ("the branch's last checkpoint commit"), so they stay readable from main
   only if PR #48 merges with a merge commit like #44 to #51. The boundary spike (ADR 0002:1127, "throwaway, in
   `spikes/`") counts "wasm-bindgen's glue calls" (0002:1137), so it uses wasm-bindgen before any foundation step: "the
   step that adds wasm-bindgen" is ambiguous about whether the spike counts.

## Verdict
HOLDS, narrowed; Low (the same as EA-3). The text EA-3's fix targets is inaccurate at 4d78ad6: both EMPTY-SLOTS lines
(report :185-186, STATUS_ARCHIVE :634-637) put "OQ-0002-7 and the wasm-bindgen pin" under "each waits for the boundary
spike", and sim-wasm's doc line says "until the boundary spike reports (OQ-0002-3, OQ-0002-7)", while ADR 0002 lists
OQ-0002-7 under "What has no measurable mark here" (1371, 1388: "a reading, not a measurement") and names a reading as
its decider (1082-1083); neither follow-up repeats plan Part 3's time for that reading. Two legs fail:
- The register: no OQ-0002-7 row is by design (its scope line, 41-43: questions decided "by an agent" are not there).
  No row should be added.
- "Only ADR 0002:1082-1084 carries the by-day reading" is too strong: r1's R1-BND-OQ7 (2741-2744) and UNREAD_SOURCES
  (:26, :132) name the source and the question (r1's "in the synthesis" is stale, and neither tells a later step to do
  the reading), and the report (:39) and the archive (:634) point to the plan, which carries the time (conditional on a
  merge commit, check 11). So the headline "drops out of the lasting record" overstates: the ADR, the record of open
  questions, keeps the decider word for word; what is missing is the time and the follow-ups' accuracy.
Risk: small (a later plan that names OQ-0002-7, as the follow-up does, meets the decider in the ADR). Wording: Low.

Fix: right in substance, incomplete. (1) "the other slots wait for the boundary spike" drops the pin's own wait for the
spike's exports (plan Part 3: "`sim-wasm` has no bindings until the boundary spike fixes its exports"): that slot needs
both conditions. (2) "before the step that adds wasm-bindgen" is the plan's time, not the ADR's: attribute it to 2.2's
plan, Part 3. (3) Say whether the boundary spike's throwaway use in `spikes/` counts (the question is about a
multi-year commitment, so "before a foundation step adds wasm-bindgen to `crates/`" reads closest to the plan's
intent; the plan does not say). (4) Keep the archive's pointer to the plan. (5) The doc-line edit touches report
:118-119 ("crates ... unchanged since step 2 (566363f)") and, as a Low, cannot trigger a push alone: it rides D13/D1.
Suggested EMPTY-SLOTS text: "OQ-0002-7 and the wasm-bindgen pin: the pin comes with `sim-wasm`'s bindings, after the
boundary spike fixes its exports; OQ-0002-7's decider, a by-day reading of issue #4533 and the guide's governance page
(ADR 0002), comes before a foundation step adds wasm-bindgen (2.2's plan, Part 3); the other slots wait for the
boundary spike."
