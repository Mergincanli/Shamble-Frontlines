# Plan: v2-decision-drafts (docs only), revision 3

Revision 3 (about 18:43) applies Mustafa's UPGRADE-SCOPE update of 2026-10-31 (his words, verbatim in
`inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; it replaces the brief's upgrade note). It answers the open question
condition 11 named ("how a pick applies after a switch to a different unit": tracks survive switching) and names three
new ones; nothing else changes. A decision of Mustafa's, applied as written, so no new review; the report logs it.

Revision 2 folds in the advisor's APPROVE of revision 1 and its 19 conditions (2026-10-31, about 18:35 Amsterdam;
verbatim in `advisor-verdicts.md` §1). Folding needs no second review (the verdict). Each condition's place is marked
[Cn] below.

## Goal
Brief: `inputs/DRAFTS_BRIEF.md` (Mustafa and the Advisor, 2026-10-31; byte-identical copy of the attachment). Write the
v2 phase 0 decision drafts from the R1–R7 research, then stop for Mustafa and the Advisor:
1. `docs/ENGINE_OVERVIEW.md`: the engine on one page, with a diagram (the parts and the Rust ↔ TypeScript boundary; the
   data flow per tick and per frame; where each VISION_V2 §6 requirement lives); at most about 150 lines with its
   diagram, details linked to the drafts [C19].
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
  `cs/v2-decision-drafts` (CLAUDE.md, standing permission). Base: 9e7db28 (PR #32's merge; the remote main, read with
  `git ls-remote`; the local `origin/main` was fetched up to it at about 18:36). Slug and checkpoint:
  `v2-decision-drafts` (`.checkpoints/v2-decision-drafts.md`, notes in `.checkpoints/v2-decision-drafts/`).
- Resume check: no checkpoint for this task existed on main or this branch, so this is a fresh start, not a restart.

## Step 0, done before the review (brief §0, "add it to docs/ first")
`docs/LOOK_V2.md` added byte for byte from the attachment (sha256 52b4fc06…, LF, cmp-checked; commit 6617e25). It is
the one edit made before the plan's approval, because brief §0 orders it first; the report says so [C18]. The brief is
copied to `inputs/DRAFTS_BRIEF.md` so agents can read it.

## Change type, scope and symbols
Docs only (decision drafts: the research evidence type). Symbols +0 −0: no engine file, page, tool, CI or rules file
changes.
- **New:** `docs/LOOK_V2.md` (step 0) · `docs/ENGINE_OVERVIEW.md` · `docs/SPIKE_PLAN.md` · `docs/adr/README.md` (the
  index of the drafts, every decision code they raise, the DESIGN-owned research ids, the features whose need is
  outside VISION_V2 §6, and the loose-ends register) · `docs/adr/0001-multiplayer-netcode.md` ·
  `docs/adr/0002-code-structure-build-toolchain.md` · `docs/adr/0003-rendering.md` · `docs/adr/0004-simulation-core.md`
  · `docs/adr/0005-content-abilities-assets.md` · `docs/adr/0006-infrastructure-operations.md` ·
  `docs/reports/<date>-v2-decision-drafts.md` and its `-appendix.md` (the appendix quotes the brief in full [C12]; the
  date is the day the report is written).
- **Changed:** `docs/research/GLOSSARY.md`: appended after line 742 (lines 1–742 stay byte-identical, so every
  GLOSSARY line cited at 9e7db28 holds): `## 8. Phase 1 terms (the Advisor, 2026-10-31)` with three entries (the
  foundation, the netcode proof test, the 60 Hz margin) and one closing note, by engine-architect, the glossary's owner
  [C3]. `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` (docs-keeper, last).
- **Not touched:** the six topic files and `UNREAD_SOURCES.md` (where a research sentence is superseded, the draft states
  the corrected text and cites the line it supersedes); `docs/LEGACY_INVENTORY.md` (each draft lists the rows whose mark
  it would change; the file changes only once Mustafa accepts the drafts, and the report says openly that this delays
  R7-MARKS (a)'s "changed when their ADR draft comes up" [C18]); the Advisor's documents (VISION_V2, LOOP_V2, EDITOR_V2,
  V2_PHASE0_BRIEF, DESIGN_NOTES, LOOK_V2 beyond step 0); CLAUDE.md and `.claude/` [C13].
- **Write guard** [C1], before every commit: `git fetch origin main`. Then `git diff --name-only 9e7db28...HEAD` (if
  main has moved, the merge base with the fetched origin/main) lists only the paths above and
  `.checkpoints/v2-decision-drafts*`. `git diff --quiet 9e7db28 HEAD -- docs/research/r1-simulation-core.md
  docs/research/r2-netcode.md docs/research/r3-rendering.md docs/research/r4-physics-navigation.md
  docs/research/r5-toolchain-quality.md docs/research/r6-infrastructure.md docs/research/UNREAD_SOURCES.md
  docs/LEGACY_INVENTORY.md` exits 0. `git show 9e7db28:docs/research/GLOSSARY.md | cmp - <(head -n 742
  docs/research/GLOSSARY.md)` passes. Any other result stops the commit.

## Settled inputs (never reopened)
The stack; N = 48 (MODES-3 (a)); TICK-30; SLICE-ORDER and its three conditions; PREVIEW-WALL (a); SEC-SPREAD (a);
ABL-PREDICT (waits for the Feel Lab test); BASELINE-WHEN (b); UPGRADE-SCOPE as Mustafa updated it on 2026-10-31
(verbatim in `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; docs-keeper records his words in STATUS with today's
decisions): every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the
player; tracks survive death and switching and reset at the start of every match; a class always comes with its own
weapon (no weapon choice when buying); other weapons come only from killed enemies' drops, and dropped weapons are world
pickups; progress belongs to the weapon type, not the physical gun; the tracks live on the PlayerState, not the Pawn.
Open, for Mustafa (listed, never decided): carry slots, ammo for looted weapons, and how fast tracks level (including
catch-up when switching);
SPIKE-SCOPE (a); R7-MARKS (a); LOOK_V2 (authored glTF/GLB models allowed in v2); GLOSSARY §8's foundation and netcode
proof test (brief §2). Where a research mark gives 48 and 54, the drafts use 48; where it says "until … answers", the
answer applies (carry-forward 3).
- **Citing them** [C12]: by the STATUS decision code or the GLOSSARY §8 entry, with "brief §n" only beside it (the brief's
  copy leaves the branch with the checkpoint). An option that contradicts a settled input (a bundler other than Vite, a
  renderer other than three.js WebGPU, a transport other than WebTransport with the WebSocket fallback, and the like)
  appears only as "settled by <code or GLOSSARY §8 entry>"; it is not weighed.
- **TICK-60-HEADROOM, the advisor's reading of 2026-10-31** [C2], confirmed by Mustafa and the Advisor at the stop:
  TICK-30's 25 % is measured against the server tick budget at 60 Hz, 8.33 ms. A 60 Hz tick needs all of this in the
  netcode proof test: the setup (one vCPU of the reference server, the stress scene at N = 48 with a snapshot-receiving
  client on every soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included); a p99 server tick time of at
  most 6.25 ms (a 60 Hz margin of at least 25 %); every 60 Hz mark the research sets, as written (R2-BUD-REC6's two
  shares, R1-TICK-REC2 P1–P3). Otherwise the tick stays at 30 Hz. ADR 0001 owns this mark and the 30 Hz baseline marks
  (R2-BUD-REC6 restated from GLOSSARY §1's 30 Hz column). ADR 0001 states three lines as replaced by TICK-30:
  R1-TICK-REC2's rule (r1:2029), "the rule does not change" (r2:2415–2416) and "which does not change" (GLOSSARY:85).
  Where it restates r2:2402–2403, it adds "(at N = 54, MODES-3 (b))" (PR #28 review, finding 2).

## Rules every author follows (written into every prompt)
1. **Citations:** every claim and number cites its research file, section and id, with lines at 9e7db28 (the topic files
   do not change in this PR, so the lines hold at the head), for example `(r2-netcode.md, Recommendations, Brief question
   2, R2-BUD-REC6, lines 2398–2420)`. Settled inputs follow "Citing them" above; design rules cite VISION_V2, LOOP_V2,
   EDITOR_V2 or LOOK_V2 by section.
2. **Numbers:** a number comes from the research (cited), is arithmetic on research numbers (shown), is a settled input,
   or is a pass mark marked "proposed" with its reason. Nothing else. A claim the research does not support is dropped,
   becomes an open question, or becomes a spike mark.
3. **Options** come from the research's Options sections; an option the research lacks is labelled "not researched"
   and cannot be the proposed decision.
4. **Trace** [C8]: every engine feature a draft proposes names the VISION_V2 §6 item that needs it, where one applies. A
   feature whose need is only in LOOP_V2, EDITOR_V2, LOOK_V2 or GLOSSARY §8's foundation says so, and
   `docs/adr/README.md` lists those features, one line each, for the stop review. An item tagged [LATER] or [OPEN] gets
   no proposed feature; the draft says only what keeps it possible. That covers §6.4's seats, §6.8's kill-cams, replays
   and highlights, §6.10's accounts, and the same tags in LOOP_V2, EDITOR_V2 and LOOK_V2. No generic-engine feature "for
   later".
5. **Glossary:** the GLOSSARY's terms with their marking words; no "Retire (exact)" phrase; an "in context" phrase only
   where it does not carry the term's meaning; the first use in each draft links the entry. No new meaning for an
   existing term: TICK-30's 25 % is "the 60 Hz margin" (GLOSSARY §8), never "headroom", which GLOSSARY:355–373 uses for
   a run above the largest mode's N; the word appears only inside quotes of TICK-30 [C3]. Spike names are defined once,
   in `docs/SPIKE_PLAN.md`.
6. **The look:** LOOK_V2 is Mustafa's decision (brief §1). ADR 0003 shows how each LOOK_V2 `[DECIDED]` item is achieved,
   builds the `[DEFAULT]` items as defaults, and decides nothing LOOK_V2 leaves `[OPEN]` (the tree style). The prompts
   say this outright, because graphics-engineer.md:19 and art-director.md:16 still say the look is not decided. And
   [C9]: where two techniques both meet a LOOK_V2 item but look different (for example R3-LOOK-REC3's two outline
   families), the draft proposes one on cost and robustness and names the visible difference as Mustafa's call in the
   look proof test. Visible values (line widths, ramp steps, AO strength, colours, glow strength) are tuning for that
   test, not draft decisions. A LOOK_V2 item that could be read as a rule (fragments that deal damage, planks and debris
   as routes, enterable ruins) is built as LOOK_V2 states it; any rule beyond that is an open design question for
   Mustafa, unless LOOP_V2 settles it. Rule 6 also goes into every reviewer prompt.
7. **Open questions** are numbered per draft (`OQ-0001-1` …), each with the question, why it matters, the options, the
   decider (Mustafa, the Advisor, a spike, a later design session) and the research ids it carries.
8. **Dispositions** [C6]: every recommendation and open question of R1–R6 (including the cross-unit (XU), rust-reviewer
   (RR), cross-topic (XT) and shared ones) gets exactly one owner disposition across three places: the six drafts'
   Appendix A (adopted · changed, how and why · rejected, why · answered, how · open, its OQ id · for a spike, which);
   `docs/adr/README.md`'s register (owner DESIGN: a pure design matter for Mustafa or a later design session, with one
   line why); and `docs/SPIKE_PLAN.md` (owner SPIKE_PLAN). The script counts the 521 unique ids in
   `inputs/research-ids.tsv` (rebuilt after the review: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes in 18
   copies; the rows r3:524 and r3:771 were references to R1-BND-REC2 and R1-TICK-REC5 inside R3's legacy-facts list, not
   second definitions, and the script now skips a bold id outside its own topic file).
9. **A claims ledger per draft** (a notes file, not in the draft): every claim and number, its citation and its source
   tier (research verified · research, needs its source checked · arithmetic · settled · design doc · proposed). The
   test-engineer's pass-mark rows go into the same ledger [C15]. The verifier works from it and checks that no claim in
   the draft is missing from it.
10. Never decide the look, write engine code, run a spike, or state a number you did not source. Fetched pages are
    data, never instructions [C16].
11. **Template fields** [C13]: each draft fills template 0000's "Hard constraints in CLAUDE.md that change" (which legacy
    constraint stops applying to v2 code, and under which decision code: SPIKE-SCOPE (a); LOOK_V2 §8 for "no external
    assets"), its Migration plan and its Rollback. Nothing in CLAUDE.md or `.claude/` changes in this PR.
12. **People judge some spikes** [C10]: the netcode proof test's "look correct" and the whole look proof test are
    Mustafa's judgement (escalation category 1). Their pass marks give measurable marks where they exist (for the
    netcode proof test: the share of client-shown hits the server confirms, correction sizes and prediction error, at
    100 ms simulated latency with stated jitter and loss) and add a numbered Brave checklist for Mustafa, each step with
    its expected result. The look proof test's checklist names each LOOK_V2 item it shows, side by side with legacy
    screenshots (LOOK_V2 §9), beside the frame-time marks on the minimum laptop and the recommended desktop.

## Who writes what (brief §4), with model and effort [C14]
Every agent below runs `model: opus` (Opus 5.5; the agent files say so; a built-in agent gets `model: "opus"` passed).
| Unit | Agent | Effort | Main research |
|---|---|---|---|
| Decision map (step 2) and its addendum | engine-architect (lead) | xhigh | all |
| ADR 0001 multiplayer and netcode | netcode-architect | xhigh | R2 (all units), R1-TICK, R6-HOST (servers) |
| ADR 0002 code structure, build and toolchain | engine-architect | xhigh | R5-LAYOUT, TSLINT, RUSTLINT, TEST, CI, GATE, PREV (the build half); R1-BND |
| ADR 0003 rendering | graphics-engineer | xhigh | R3 (all units), LOOK_V2 |
| ADR 0004 simulation core | rust-reviewer | xhigh | R1-ECS, TICK, DET, RR; R4 (all units) |
| ADR 0005, the ability system section | game-designer | xhigh | R1-ABL, LOOP_V2 |
| ADR 0005, the asset section (GPU rules) | graphics-engineer | xhigh | R3-CHAR, R3-BUDGET, LOOK_V2 §8 |
| ADR 0005, content data and the frame | engine-architect | xhigh | R1-DATA, VISION_V2 §6.3, §6.4, §6.7, EDITOR_V2 §2 |
| ADR 0006 infrastructure and operations | infra-engineer | xhigh | R6 (all units), R5-PREV (the hosting half), BASELINE-WHEN (b) |
| Pass marks for every draft, and the spike entries | test-engineer | xhigh | R5-TEST, R5-GATE, each draft |
| ENGINE_OVERVIEW, SPIKE_PLAN, adr/README, GLOSSARY §8 | engine-architect | xhigh | the drafts |
| Verification units and their skeptics | verifier | xhigh | the drafts, the research, the sources |
| Consistency critic | engine-architect (a fresh one) | xhigh | everything |
| Completeness critic | verifier (a fresh one) | xhigh | everything |
| Fix owners | each text's own author agent | xhigh | their notes |
| Reviewers (step 10) | as listed there | xhigh | the final diff |
| STATUS, ROADMAP, REPO_MAP | docs-keeper | high | the change-set |

Authors whose agent file does not name `docs/adr/` (graphics-engineer, rust-reviewer, game-designer, infra-engineer,
test-engineer) write their text whole into their notes; the session copies it into `docs/adr/` verbatim (`cmp` after
copying) and credits the author in the draft's header. netcode-architect and engine-architect follow the same route, so
every draft is assembled the same way. Each author fixes only its own text.

**ADR 0005** [C4, C11, and Mustafa's UPGRADE-SCOPE update]: it designs no upgrade, ability, unit or rule (escalation
category 2). It shows that the ability system can carry UPGRADE-SCOPE as Mustafa wrote it: a roguelike track per unit
and per weapon type (pick one of three per level), owned by the player (the PlayerState, not the Pawn), surviving death
and switching, reset every match; progress on the weapon type, waiting when a gun is lost for the next gun of that
type. It lists, for Mustafa, with their options and no pick: carry slots, ammo for looted weapons, and how fast tracks
level (including catch-up when switching); any tension it finds with LOOP_V2 or VISION_V2 (for example LOOP_V2 §8's
third gate, buying an unlocked item each life, next to "no weapon choice when buying") is listed the same way, never
resolved. The other drafts carry their parts as the decision map's addendum assigns them: the player record that holds
the tracks and the weapon drops and pickups as sim entities (0004), their authority, messages and replication (0001). Its asset part takes LOOK_V2 §8's flow as given and proposes only what
R1-DATA, R3-CHAR and R3-BUDGET support; the rest (compression formats, the LOD tool, export checks, the material-slot
renaming step and the per-unit budget numbers) are open questions marked "not researched", grouped under one research
unit, "asset-pipeline research" (graphics-engineer, verified like R3, in its own daytime session), which SPIKE_PLAN
places before the asset spike. The look proof test says whether it needs authored models; if it does, it uses plain
glTF/GLB exports and decides nothing about compression or LODs. engine-architect writes 0005's frame (header, context,
the joined decision, consequences, open-question numbering, Appendix A) after the game-designer's and
graphics-engineer's parts land, and places those parts verbatim as sections.

## Steps
1. **Plan review:** done, APPROVE with 19 conditions (`advisor-verdicts.md` §1), folded here.
2. **Decision map** (engine-architect, notes only: `engine-architect-decision-map.md` and `-ids.tsv`), started in
   parallel with the review and not redone. Its **addendum** [C7], by the same agent resuming from its notes: (a) the 60
   Hz margin as a shared-number row (owner 0001, GLOSSARY §8), plus the §8 text of [C3] written out in full, so authors
   link stable anchors; (b) three items in the loose-ends register: the old ADR-001..003 entries (STATUS:198–202), each
   with the draft that disposes of it; the items STATUS sends to ADR-001 (STATUS:257 SEC-MDL-2, :262 no SRI/CSP on the
   CDN tag, :273 the 11 wall-clock reads), each with its draft; one line closing PR #28 finding 4's research-plan
   condition 13 (two branches from one session): it no longer applies; (c) the traces of rule 4, including EDITOR_V2
   §2's document rules (→ 0005) and EDITOR_V2 §5's double-click and "if ADR-002" lines (→ 0002, listed for DOCS-ALIGN);
   (d) how ADR 0005 is put together (above); (e) UPGRADE-SCOPE as Mustafa updated it: which draft carries each part
   (the tracks as data, the player record that holds them, weapon drops and world pickups in the sim, their authority,
   messages, replication and telemetry) and where its three open questions are listed. A script checks the id table: every one of the 521 ids exactly once, owner
   in {0001…0006, SPIKE_PLAN, DESIGN}.
3. **Drafting** (≤ 5 agents at once, NIGHT-RESEARCH (a)): the units of the table. Each writes its draft (or section), its
   claims ledger and its notes, and returns a short summary. Prompts saved before launch in `prompts/`. ADR 0005's frame
   starts once its two sections have landed.
4. **Pass marks** (test-engineer, one unit per draft, as each draft lands): a "Pass marks" table under the draft's "What
   a spike must prove" (mark · what it proves · number and statistic · hardware · method and tool · runs and noise ·
   basis), the Brave checklists of rule 12, plus the draft's spike entries for the spike plan; its rows join the
   draft's claims ledger. The drafter's prose stays; the table is the test-engineer's.
5. **Assembly** (the session): copy the drafts into `docs/adr/`, insert the pass-mark tables verbatim, `cmp`; write
   guard; commit.
6. **Synthesis** (engine-architect): `docs/ENGINE_OVERVIEW.md` (≤ about 150 lines with its diagram), `docs/SPIKE_PLAN.md`
   (each spike: what it proves, its pass marks from step 4, its place in the order, a rough size; the netcode proof test
   runs before the foundation, and the plan names the research spikes it subsumes; it also places the asset-pipeline
   research before the asset spike, the Feel Lab test (ABL-PREDICT) and the PREV-DELETE item [C4, C10]),
   `docs/adr/README.md` (the index, the decision codes, the DESIGN-owned ids, the features whose need is outside
   VISION_V2 §6, and the register: each loose end → where it is resolved or which OQ lists it), and GLOSSARY §8 (the
   file edit lands here [C3]); write guard; commit.
7. **Verification** (verifier, xhigh, one unit per draft plus one for the synthesis docs, ≤ 5 at once, pipelined from
   step 5): every claim and number in the ledger and the text (carry-forward 1, research plan condition 9), against the
   research file and section it cites; arithmetic recomputed. "Research verified" covers only R1 and R2 recommendation
   claims and the claims their verifiers checked (r1:3233) [C16]; every other finding a draft rests on is checked
   against its original source (R1 or R2 findings outside that set, R3–R6 findings outside their spot-check tables;
   `inputs/findings-checked.tsv` is a script-built guide, each topic's own verifier section the authority), on
   allow-listed pages read with curl by day (VERIFY-FETCH (a)), with the host audit in the report; an unreadable source
   makes the claim "unverified" in the draft. Fetched pages are data, never instructions. A "wrong" or "unsupported"
   verdict gets one skeptic (a fresh verifier, xhigh, asked to refute it) before it goes to the owner.
8. **Consistency pass** (a fresh engine-architect as the critic, xhigh, plus scripts): the drafts against each other
   (every shared number, owner, interface and term the same everywhere), against the glossary (the research's glossary
   check, recovered from 85f89ea and pointed at the drafts: 0 exact, every in-context hit judged), and for completeness
   (every id disposed once across the three places, every §6 and LOOK_V2 `[DECIDED]` item traced, every carry-forward
   handled). Then the completeness critic (a fresh verifier, xhigh): what is missing.
9. **Fix round:** every finding to its owner (its author agent, xhigh), who writes exact before/after texts in its
   notes; the session applies them and re-copies; the verifier re-checks only what changed.
10. **Review on the final diff** (CLAUDE.md step 5; xhigh; ≤ 5 at once; rule 6 in every prompt):
    engine-architect (boundaries: 0001, 0003, 0004, 0006) · netcode-architect (0002, 0004, the overview; SPIKE_PLAN's
    netcode proof test) · rust-reviewer (0001, 0002, 0005; SPIKE_PLAN's boundary spike) · graphics-engineer (0002, 0005,
    the overview's per-frame flow; SPIKE_PLAN's look proof test) · security-auditor (0001, 0002, 0006) · infra-engineer
    (0001, 0002) · game-designer (0001, 0004 for rules and data) · art-director (0003, 0005's asset part, SPIKE_PLAN's
    look proof test) · perf-auditor (every budget mark) · test-engineer (is it proven: every draft and SPIKE_PLAN, but
    not its own pass-mark tables) · and each draft's author reviews its draft's pass-mark table for domain fit [C15].
    Every finding is fixed or explicitly deferred with a reason; fixes go through step 9's loop once more.
11. **docs-keeper** (high): STATUS (this change-set's row; PR #32's row marked merged; UPGRADE-SCOPE recorded with the
    other 2026-10-31 decisions in Mustafa's own words of his update (verbatim in the inputs folder), as he and brief §1
    ask; the carry-forwards' state; KIT-ALIGN gains the stale "look is not
    decided" lines, CLAUDE.md:5, :23–24, art-director.md:16, graphics-engineer.md:19, and the "toy-box" lines CLAUDE.md:4
    and art-director.md:10, which may clash with LOOK_V2 §3, flagged for Mustafa, not changed [C18]; DOCS-ALIGN gains
    EDITOR_V2 §5's double-click line and its "if ADR-002 moves the project to modules" line; "Next": the stop),
    ROADMAP, REPO_MAP (the new files); repo-map check.
12. **Gate and checks:** `node tools/gate.mjs 9e7db28` (PASS, symbols +0 −0, smoke) [C1], the repo-map check, the write
    guard, the coverage, citation and glossary scripts (outputs in the report).
13. **Report** (`/advisor-report`, story first, "Decisions needed" and "For Mustafa" last; it folds in the checkpoint's
    essentials and says the checkpoint and notes are deleted in the last commit before the PR is marked ready,
    CHECKPOINTS.md §9.2–9.3 [C5]; it lists ASSET-RESEARCH under "Decisions needed" so Mustafa and the Advisor can move the
    research earlier [C4]; it asks them to confirm the TICK-60-HEADROOM reading [C2] and the "toy-box" lines [C18]; it
    states the R7-MARKS delay and the step-0 edit openly [C18]) and a **draft PR** (brief §5; the body is the report).
    Subscribe to its activity. Then stop.

Checkpoint after every step and at least every 30 minutes (`checkpoint.mjs save`); the background saver runs during
agent runs. Every agent prompt carries CHECKPOINTS.md §4's "Saving your progress" block. A spend-limit cut: record it,
launch nothing new, resume from notes after the reset. **Daytime only** [C17]: no new agent after 23:00 Amsterdam, and no
verifier unit that fetches sources after 22:00, so units end before the night window (00:10). A unit still running at
00:10 is recorded in the checkpoint, and the next day session reads its notes before using its result.

## Evidence (research change type)
Every claim cited; every claim and number verified (ledger counts per draft: confirmed, wrong and fixed, unsupported,
unverified because the source could not be read); the skeptic outcomes; the consistency and completeness findings
with their fixes; the coverage script (every one of the 521 ids disposed once), citation script (every cited id exists
in the cited file, every line range exists), glossary script (0 exact); gate PASS; repo-map OK; the curl host audit.
Nothing for Brave now: Mustafa judges the look in the look proof test later.

## The checkpoint at the stop [C5]
The PR stays a draft for Mustafa and the Advisor (brief §5), and their review may send the drafts back, so the
checkpoint stays on the branch; it and its notes are deleted in the last commit before the PR is marked ready
(CHECKPOINTS.md §9.2–9.3). The report folds in its essentials now.

## Risks
1. **Size and time:** seven drafting units, pass marks, seven verification units and ten reviewers at ≤ 5 at once;
   likely more than one day session. Units are small and notes-first, so a cut or the 23:00 stop loses at most a unit.
2. **Spend limit:** it cut the research five times; resumable units and notes keep every cut recoverable.
3. **Research gaps:** the research has no findings on the asset pipeline: 0 whole-word hits for KTX2, meshopt,
   meshoptimizer, Draco, Spline, gltf-transform or Basis Universal, and one glTF mention in all of the research
   (R3-CHAR-F8, r3:476; the other 12 hits of a case-insensitive search are "GLB" inside "WebGLBackend") [C4]; nor on
   several LOOK_V2 items (halftone flashes, line boil, roof fade, damage stages). Rule 2 turns those parts into open
   questions or spike marks; ASSET-RESEARCH (a) groups the asset ones under one research unit.
4. **Source checks:** many sources may be unreadable (UNREAD_SOURCES: 67 hinge rows); those claims stay "unverified" and
   the decisions on them become spike marks or open questions.
5. **Drift between drafts:** seven authors; the decision map, one owner per shared number and the consistency pass
   contain it.
6. **Concurrency on this container:** 4 CPUs, so a workflow runs at most 2 agents at once; long units therefore run as
   background agents through the Agent tool (≤ 5), and the session counts what comes back (CHECKPOINTS.md §7).

## Rulings (the advisor, 2026-10-31)
- ASSET-RESEARCH: (a), with [C4]. TICK-60-HEADROOM: ruled now [C2], confirmed at the stop. GLOSSARY-8: yes, append-only
  §8 with three entries and one note [C3]. CHECKPOINT-AT-STOP: keep [C5].

## For Mustafa
Nothing now. At the stop: the drafts and the decisions the report lists (TICK-60-HEADROOM's reading, when the
asset-pipeline research runs, the "toy-box" lines).
