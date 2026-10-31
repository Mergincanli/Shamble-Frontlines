# Plan: v2-decision-drafts (docs only), revision 1

## Goal
Brief: `inputs/DRAFTS_BRIEF.md` (Mustafa and the Advisor, 2026-10-31; byte-identical copy of the attachment). Write the
v2 phase 0 decision drafts from the R1–R7 research, then stop for Mustafa and the Advisor:
1. `docs/ENGINE_OVERVIEW.md`: the engine on one page, with a diagram (the parts and the Rust ↔ TypeScript boundary; the
   data flow per tick and per frame; where each VISION_V2 §6 requirement lives).
2. Six drafts `docs/adr/0001`–`0006` on template 0000, Status: Proposed. Each has the context, the options with their
   trade-offs, the proposed decision and its consequences, the open questions, and what its spike must prove with
   measurable pass marks (numbers, hardware, method). Every claim cites the research file and section it rests on.
3. A spike plan: every spike (at least the netcode proof test, the look proof test and the Rust ↔ TypeScript boundary
   spike), with its pass marks, its place in the order and a rough size.
4. Loose ends: the five STATUS carry-forwards and the critic's remaining items resolved, or each listed as an open
   question.
Then a draft PR, the report per CLAUDE.md, and a stop. No engine code, no spikes (brief §4).

## Branch, base, slug
- Branch `claude/laughing-lovelace-ox61se`: this cloud session is limited to it, so it stands in for
  `cs/v2-decision-drafts` (CLAUDE.md, standing permission). Base: origin/main 9e7db28 (PR #32's merge). Slug and
  checkpoint: `v2-decision-drafts` (`.checkpoints/v2-decision-drafts.md`, notes in `.checkpoints/v2-decision-drafts/`).
- Resume check: no checkpoint for this task exists on main or this branch (`git ls-tree` on origin/main: none), so this
  is a fresh start, not a restart.

## Step 0, done before this review (brief §0, "add it to docs/ first")
`docs/LOOK_V2.md` added byte for byte from the attachment (sha256 52b4fc06…, LF, cmp-checked). The brief is copied to
`inputs/DRAFTS_BRIEF.md` so agents can read it. Nothing else is edited before your verdict.

## Change type, scope and symbols
Docs only (decision drafts: the research evidence type). Symbols +0 −0: no engine file, page, tool, CI or rules file
changes.
- **New:** `docs/LOOK_V2.md` (step 0) · `docs/ENGINE_OVERVIEW.md` · `docs/SPIKE_PLAN.md` · `docs/adr/README.md` (the
  index of the drafts, every decision code they raise, and the loose-ends register) ·
  `docs/adr/0001-multiplayer-netcode.md` · `docs/adr/0002-code-structure-build-toolchain.md` · `docs/adr/0003-rendering.md`
  · `docs/adr/0004-simulation-core.md` · `docs/adr/0005-content-abilities-assets.md` ·
  `docs/adr/0006-infrastructure-operations.md` · `docs/reports/2026-10-31-v2-decision-drafts.md` (+ `-appendix.md` if
  the register and counts do not fit 60 lines; the date is the day the report is written).
- **Changed:** `docs/research/GLOSSARY.md`: one new section, "8. Phase 1 terms (the Advisor, 2026-10-31)", with two
  entries, the foundation and the netcode proof test, quoting brief §2 word for word (engine-architect, the glossary's
  owner; closes carry-forward 5 where every draft looks up its terms). `docs/STATUS.md`, `docs/ROADMAP.md`,
  `docs/REPO_MAP.md` (docs-keeper, last).
- **Not touched:** the six topic files and `UNREAD_SOURCES.md` (where a research sentence is superseded, the draft states
  the corrected text and cites the line it supersedes); `docs/LEGACY_INVENTORY.md` (each draft lists the rows whose mark
  it would change; the file changes only after Mustafa accepts the drafts, R7-MARKS (a)); the Advisor's documents
  (VISION_V2, LOOP_V2, EDITOR_V2, V2_PHASE0_BRIEF, DESIGN_NOTES, LOOK_V2 beyond step 0); CLAUDE.md and `.claude/`.
- **Write guard** before every commit: `git diff --name-only origin/main...HEAD` lists only the paths above and
  `.checkpoints/v2-decision-drafts*`; any other path stops the commit.

## Settled inputs (brief §1–2; never reopened)
The stack; N = 48 (MODES-3 (a)); TICK-30 (30 Hz baseline, 60 Hz only with at least 25 % headroom in the netcode proof
test); SLICE-ORDER and its three conditions; PREVIEW-WALL (a); SEC-SPREAD (a); ABL-PREDICT (waits for the Feel Lab
test); BASELINE-WHEN (b); UPGRADE-SCOPE (the player owns in-match upgrade picks: they survive death and reset every
match; GAS on the PlayerState, not the Pawn); SPIKE-SCOPE (a); R7-MARKS (a); LOOK_V2 (authored glTF/GLB models allowed in
v2); the brief's definitions of the foundation and the netcode proof test. Where a research mark gives 48 and 54, the
drafts use 48; where it says "until … answers", the answer applies (carry-forward 3).

## Rules every author follows (written into every prompt)
1. **Citations:** every claim and number cites its research file, section and id, with lines at 9e7db28 (the topic files
   do not change in this PR, so the lines hold at the head), for example `(r2-netcode.md, Recommendations, Brief question
   2, R2-BUD-REC6, lines 2377–2387)`. Settled inputs cite brief §1 or the STATUS decision code; design rules cite
   VISION_V2, LOOP_V2, EDITOR_V2 or LOOK_V2 by section.
2. **Numbers:** a number comes from the research (cited), is arithmetic on research numbers (shown), is a settled input,
   or is a pass mark marked "proposed" with its reason. Nothing else. A claim the research does not support is dropped,
   becomes an open question, or becomes a spike mark.
3. **Options** come from the research's Options sections; an option the research lacks is labelled "not researched"
   and cannot be the proposed decision.
4. **Trace:** every engine feature a draft proposes names the VISION_V2 §6 item (or LOOP_V2, EDITOR_V2, LOOK_V2 section)
   that needs it; a feature with no need is left out (CLAUDE.md, the v2 rebuild).
5. **Glossary:** the GLOSSARY's terms with their marking words; no "Retire (exact)" phrase; an "in context" phrase only
   where it does not carry the term's meaning; the first use in each draft links the entry. No new meaning for an
   existing term. Spike names are defined once, in `docs/SPIKE_PLAN.md`.
6. **The look:** LOOK_V2 is Mustafa's decision (brief §1). ADR 0003 shows how each LOOK_V2 `[DECIDED]` item is achieved,
   builds the `[DEFAULT]` items as defaults, and decides nothing LOOK_V2 leaves `[OPEN]` (the tree style). The prompts
   say this outright, because graphics-engineer.md:19 and art-director.md:16 still say the look is not decided.
7. **Open questions** are numbered per draft (`OQ-0001-1` …), each with the question, why it matters, the options, the
   decider (Mustafa, the Advisor, a spike, a later design session) and the research ids it carries.
8. **Appendix A in every draft: research dispositions.** Every recommendation of the draft's research units (adopted,
   changed (how and why), rejected (why), or owned by ADR X) and every open question of those units, including the
   cross-unit (XU), cross-topic (XT) and shared ones (answered here (how), still open (its OQ id), for a spike (which),
   or owned by ADR X). A script checks that every id in R1–R6 has exactly one owner disposition across the six drafts.
9. **A claims ledger per draft** (a notes file, not in the draft): every claim and number, its citation and its source
   tier (research verified · spot-checked and confirmed · spot-checked topic, not sampled · arithmetic · settled ·
   proposed). The verifier works from it and checks that no claim in the draft is missing from it.
10. Never decide the look, write engine code, run a spike, or state a number you did not source.

## Who writes what (brief §4)
| Part | Author (Opus 5.5) | Effort | Main research |
|---|---|---|---|
| Decision map (step 2) | engine-architect (lead) | xhigh | all |
| ADR 0001 multiplayer and netcode | netcode-architect | xhigh | R2 (all units), R1-TICK, R6-HOST (servers) |
| ADR 0002 code structure, build and toolchain | engine-architect | xhigh | R5-LAYOUT, TSLINT, RUSTLINT, TEST, CI, GATE, PREV (the build half); R1-BND |
| ADR 0003 rendering | graphics-engineer | xhigh | R3 (all units), LOOK_V2 |
| ADR 0004 simulation core | rust-reviewer | xhigh | R1-ECS, TICK, DET; R4 (all units) |
| ADR 0005 content data, abilities and assets | engine-architect (content data, the asset pipeline, assembly), game-designer (the ability system, in-match upgrades included), graphics-engineer (the asset pipeline's GPU rules: LODs, material slots, per-unit budgets) | xhigh | R1-DATA, R1-ABL, R3-CHAR, R3-BUDGET, LOOK_V2 §8 |
| ADR 0006 infrastructure and operations | infra-engineer | xhigh | R6 (all units), R5-PREV (the hosting half), BASELINE-WHEN (b) |
| Pass marks for every draft and the spike plan | test-engineer | xhigh | R5-TEST, R5-GATE, each draft |
| ENGINE_OVERVIEW, SPIKE_PLAN, adr/README, GLOSSARY §8 | engine-architect | xhigh | the drafts |

The prefix split above is the starting assignment; the decision map makes it exact per id. Authors whose agent file does
not name `docs/adr/` (graphics-engineer, rust-reviewer, game-designer, infra-engineer, test-engineer) write their text
whole into their notes; the session copies it into `docs/adr/` verbatim (`cmp` after copying) and credits the author in
the draft's header. netcode-architect and engine-architect follow the same route, so every draft is assembled the same
way. Each author fixes only its own text.

## Steps
1. **Plan review** (this): the advisor agent. On REVISE, the plan is fixed and comes back; on ESCALATE, Mustafa decides.
2. **Decision map** (engine-architect, notes only: `engine-architect-decision-map.md`). It may start while you review,
   since notes are not edits; if your verdict changes its inputs, it is redone. Contents: (a) which draft decides what,
   with the seams settled (the tick rate rule and the server tick budget in 0001, the tick mechanism and the sim step
   share in 0004; the boundary, the Worker and the generated types in 0002; CI, the gate v2 and the preview build in
   0002, preview hosting and privacy in 0006; rooms and the room process in 0001, the machines under them in 0006;
   recording moments and telemetry in the sim (0004/0005), storing them in 0006); (b) every R1–R6 recommendation and
   open question id with one owner draft; (c) every shared number (tick shares, bandwidth per client, memory per room,
   frame and GPU budgets, Wasm size, costs) with one owner draft and its GLOSSARY term; (d) the interfaces between the
   drafts; (e) the VISION_V2 §6 and LOOK_V2 traces; (f) the loose-ends register (carry-forwards 1–5; ABL-PREDICT,
   PREV-DELETE and the answered escalations to apply; every XT and XU question) with an owner each. A script checks (b):
   every id exactly once.
3. **Drafting** (≤ 5 agents at once, NIGHT-RESEARCH (a)): the seven units of the table. Each writes its draft, its claims
   ledger and its notes, and returns a short summary. Prompts saved before launch in `prompts/`.
4. **Pass marks** (test-engineer, one unit per draft, as each draft lands): a "Pass marks" table under the draft's "What
   a spike must prove" (mark id · what · number and statistic · hardware · method and tool · runs and noise · basis),
   plus the draft's spike entries for the spike plan. The drafter's prose stays; the table is the test-engineer's.
5. **Assembly** (the session): copy the drafts into `docs/adr/`, insert the pass-mark tables verbatim, `cmp`; commit.
6. **Synthesis** (engine-architect): `docs/ENGINE_OVERVIEW.md`, `docs/SPIKE_PLAN.md` (each spike: what it proves, its
   pass marks from step 4, its place in the order, a rough size; the netcode proof test runs before the foundation, and
   the plan names the research spikes it subsumes), `docs/adr/README.md` (the register: each loose end → where it is
   resolved or which OQ lists it), GLOSSARY §8; commit.
7. **Verification** (verifier, xhigh, one unit per draft plus one for the synthesis docs, ≤ 5 at once, pipelined from
   step 5): every claim and number in the ledger and the text (carry-forward 1, research plan condition 9), against the
   research file and section it cites; arithmetic recomputed. A claim that rests on an R3–R6 finding outside that
   topic's spot-check table is also checked against the original source (allow-listed pages, read with curl by day,
   VERIFY-FETCH (a), with the host audit in the report); an unreadable source makes the claim "unverified" in the
   draft. Verdicts go to the ledger. A "wrong" or "unsupported" verdict gets one skeptic (a fresh verifier asked to
   refute it) before it goes to the owner.
8. **Consistency pass** (a fresh engine-architect as the critic, plus scripts): the drafts against each other (every
   shared number, owner, interface and term the same everywhere), against the glossary (the research's glossary check
   run on the drafts: 0 exact, every in-context hit judged), and for completeness (every id disposed, every §6 and
   LOOK_V2 `[DECIDED]` item traced, every carry-forward handled). Then a completeness critic: what is missing.
9. **Fix round:** every finding to its owner, who writes exact before/after texts in its notes; the session applies them
   and re-copies; the verifier re-checks only what changed.
10. **Review on the final diff** (CLAUDE.md step 5; each reviewer reads the drafts it does not author, ≤ 5 at once):
    engine-architect (boundaries: 0001, 0003, 0004, 0006) · netcode-architect (0002, 0004, the overview) · rust-reviewer
    (0001, 0002, 0005) · graphics-engineer (0002, 0005, the overview's per-frame flow) · security-auditor (0001, 0002,
    0006) · infra-engineer (0001, 0002) · game-designer (0001, 0004 for rules and data) · art-director (0003 against
    LOOK_V2) · perf-auditor (every budget) · test-engineer (is it proven: every draft and the spike plan). Every finding
    is fixed or explicitly deferred with a reason; fixes go through step 9's loop once more.
11. **docs-keeper** (high): STATUS (this change-set's row; PR #32's row marked merged; UPGRADE-SCOPE recorded with the
    other 2026-10-31 decisions, as brief §1 asks; the carry-forwards' state; KIT-ALIGN gains the stale "look is not
    decided" lines, CLAUDE.md:5, :23–24, art-director.md:16, graphics-engineer.md:19, which are rules files left for the
    kit change-set; "Next": the stop), ROADMAP, REPO_MAP (the new files); repo-map check.
12. **Gate and checks:** `node tools/gate.mjs` (PASS, symbols +0 −0, smoke), the repo-map check, the write guard, the
    coverage, citation and glossary scripts (outputs in the report).
13. **Report** (`/advisor-report`, story first, "Decisions needed" and "For Mustafa" last) and a **draft PR** (brief §5;
    the body is the report). Subscribe to its activity. Then stop.

Checkpoint after every step and at least every 30 minutes (`checkpoint.mjs save`); the background saver runs during
agent runs. Every agent prompt carries CHECKPOINTS.md §4's "Saving your progress" block. A spend-limit cut: record it,
launch nothing new, resume from notes after the reset. "Daytime only": no new agent after 23:30 Amsterdam; the session
checkpoints and the next day session resumes from "Next steps".

## Evidence (research change type)
Every claim cited; every claim and number verified (ledger counts per draft: confirmed, wrong and fixed, unsupported,
unverified because the source could not be read); the skeptic outcomes; the consistency and completeness findings
with their fixes; the coverage script (every R1–R6 id disposed once), citation script (every cited id exists in the
cited file, every line range exists), glossary script (0 exact); gate PASS; repo-map OK; the curl host audit. Nothing
for Brave: Mustafa judges the look in the look proof test later, not now.

## The checkpoint at the stop
The PR stays a draft for Mustafa and the Advisor (brief §5), and their review may send the drafts back, so the
checkpoint stays on the branch (CHECKPOINTS.md §9 deletes it before the PR is marked ready). The report folds in its
essentials now. Recommendation; your call.

## Risks
1. **Size and time:** seven drafting units, pass marks, seven verification units and ten reviewers at ≤ 5 at once;
   likely more than one day session. Units are small and notes-first, so a cut or the 23:30 stop loses at most a unit.
2. **Spend limit:** it cut the research five times; resumable units and notes keep every cut recoverable.
3. **Research gaps:** the research has no findings on the asset pipeline (glTF/GLB compression, LOD generation, Spline
   export; grep: 0 hits for KTX2, Basis, meshopt, Draco or Spline, and R3's 12 glTF hits are about skinned instancing)
   or on several LOOK_V2 items (halftone flashes, line boil, roof fade, damage stages). Rule 2 then turns those parts
   into open questions or spike marks. ASSET-RESEARCH below asks whether that is enough.
4. **R3–R6 source checks:** many sources may be unreadable (UNREAD_SOURCES: 67 hinge rows); those claims stay
   "unverified" and the decisions on them become spike marks or open questions.
5. **Drift between drafts:** seven authors; the decision map, one owner per shared number and the consistency pass
   contain it.
6. **Concurrency on this container:** 4 CPUs, so a workflow runs at most 2 agents at once; long units therefore run as
   background agents through the Agent tool (≤ 5), and the session counts what comes back (CHECKPOINTS.md §7).

## Decisions for you (the advisor)
- **ASSET-RESEARCH:** the asset pipeline (LOOK_V2 §8) has no research findings. (a) ADR 0005 states the pipeline at
  LOOK_V2's level, cites what R1-DATA, R3-CHAR and R3-BUDGET support, and lists the technical choices (compression
  formats, LOD tool, export checks, per-unit budgets) as open questions, researched before the asset spike; (b) a small
  graphics-engineer research addendum in this change-set, verified like R3. Recommendation: (a), which keeps this
  change-set to drafts.
- **TICK-60-HEADROOM:** TICK-30 says 60 Hz only with "at least 25 % headroom"; the measuring base is undefined
  (carry-forward 2). ADR 0001 proposes one (for example: the stress scene with a snapshot-receiving client on every
  soldier, on one vCPU of the reference server, p99 server tick time at 60 Hz at most 75 % of the 8.33 ms server tick
  budget, re-flood ticks included) and lists it for you to confirm. Recommendation: confirm it at the stop, not now.
- **GLOSSARY-8:** add the two brief §2 definitions to GLOSSARY (a new §8), or keep them only in the overview and the
  spike plan. Recommendation: GLOSSARY §8, quoted word for word.
- **CHECKPOINT-AT-STOP:** keep the checkpoint on the draft PR (above), or fold and delete it before the stop.
  Recommendation: keep it.

## For Mustafa
Nothing now. At the stop: the drafts and the decisions the report lists.
