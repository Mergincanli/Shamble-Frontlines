# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-19 (change-set 2, the first page, PR #61, after #60's merge; main b763ba3)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged:** the kit hold and two CI edits (#60, b763ba3, 2026-11-18 20:30:46 UTC, his go; CI green); see ROADMAP.
- **The kit:** KIT-FREEZE-2, what he meant by it (FREEZE-FDN) and his exceptions (two more since, for #53 and #54):
  `docs/DECISIONS.md`, "Queued"; his two additions of 2026-11-11 for the foundation chat: `docs/DECISIONS.md`. His words
  of 2026-11-13 for step 2.3:
  "The kit's freeze (KIT-FREEZE-2) ends with this step. That makes no edit unasked: every edit to a kit, gate or CI file
  is asked of me in chat with its exact text and written only after my yes, with the chat in Accept edits. A guard
  prompt alone is not my yes. No such edit happens in the docs pass or while planning."
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); CI green on 82c605e (gate run
  44035715175, toolchain run 44035715349). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| #61 | change-set 2 of the build plan, "First page" (foundation step 2.5, first half), by lane 1: the Rust simulation (a tick counter and the match seed; no random-number library yet, his RNG-CRATE (c)), built as WebAssembly with wasm-bindgen `=0.2.130`, runs in one module Web Worker at 30 ticks a second and hands the page one block per tick (so far its header: the tick and the layout's hash); the page, which makes no Wasm call, draws an empty lit ground with a 1 m grid on three.js 0.187.0's WebGPU renderer (WebGL 2 as the fallback), names its backend and has a development readout with "Copy report"; `packages/client/serve.mjs` serves a built page to this computer only, under a strict content security policy; `packages/client/scripts/build.mjs` builds the Wasm and the page and checks the tool's file (size and sha256) before every run; browser tests in `packages/e2e` (CI does not run them yet: his CI-WASM (a)); his five messages of 2026-11-18 and 2026-11-19 in `docs/DECISIONS.md` by script; report `docs/reports/2026-11-19-cs2-first-page.md` (the size: there). Follow-ups (their text: the archive's "Follow-ups", "From change-set 2, the first page (PR #61)"): for the CI pull request TE-NT6, RR-N4, RR-F5, E2E-TYPES, SA-TREE, SA-DIST; for change-set 3 NA-N1 to NA-N7, EA-F8, EA-F9, EA-F10, EA-F13, EA-N3, EA-N5, EA-N6, GE-N1, GE-N4, GE-N5, GE-N6, TE-NT3, TE-NT4, TE-NT5, TE-NT7, PA-N2, PA-L2, TE-L, SA-ENV, FAVICON; for change-sets 5 and 6 GE-F7, PA-L4; for the quality gate step SA-F6 (the npm scope `@shamble`), SA-SWAP, TE-NT2; for the drafts' next revision EA-F6, EA-F12 | `claude/wonderful-hamilton-jdmlnf` (stands in for `cs/cs2-first-page`) | opened as a draft 2026-11-19 11:01:27 UTC; the eight build steps in (3833e3e, 784ae3e, 4c32a06, a7f785a, a68e3da, cc9fc71 (the dependency step, pushed through the hold), 6e76f30, 015a212, de63a30, c4a12eb, 74a0c24), then the fix passes 29378dd and d257bb6; round 2's fix pass: the report; two review rounds (seven reviewers, then six on what the fixes changed): no Critical, no High open; the verifier's verdict: the report; the advisor agent's: the pull request's description. **Not verified anywhere yet:** WebGPU itself (this container has no WebGPU adapter: every run here was on the WebGL 2 fallback with software rendering), Brave, Firefox, Safari, a real GPU | the Advisor's check of its ready report, then his Brave check (the zip `shamble-v2-02-first-page.zip`, sent in the chat; steps: the report's "For Mustafa") and his go (risk tier 1: new dependencies, something a player sees) |

## Next
1. In his order (ORDER (b), 2026-11-10), after status-diet (merged, #49): his unit walk of 2026-11-09 (a docs
   change-set: UNITS.md §0 and LOOP_V2), then the Slice 1 note of 2026-11-10, each in a fresh chat. His words:
   `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); the foundation (his order of 2026-11-11): #49, #50, #51, #48 and #52 merged; step
   2.3: the 2.3 chat stays stopped for now; #61 waits for his go; after it, the CI-WASM pull request (its exact text to
   him first), change-set 3 (its plan asks him RNG-CRATE) and lane 2's work; Slice 3 waits for the netcode proof test.

## Waiting on Mustafa
- His Brave check and go on #61, after the Advisor's check of its ready report (steps: its report, "For Mustafa").
- From Part 15 (its "Lane requests" issue: #59): give lane 2 Part 15 and ask for its needs from the block; tell lane 2
  which chat on his PC makes the model files; the VFX chat's exports; pass the VFX findings on.
- The netcode proof test's items (its plan in #40; the older list: the archive's "Phase", "Next (v2)"): OK each run's
  plan and cost cap; name the laptop and desktop; the Mac when one is available; the Safari workflow's own PR.
- Legacy, frozen (the archive's "In flight" and "Phase"): his Windows run ("Check a new machine", `tools/README.md`);
  the UI theme step 1 Brave check (DEV-panel FPS); `/permissions` locally (kit-v1-1); the editor inventory marks
  (docs-editor-v2); re-saving each old map once (map-dedupe-on-load).
- Routines: delete the old "Night shift" (below).
- Decisions still open for him: OQ-0006-22 (before slice 3); the follow-ups marked as his call (FU-P4b, B1-MODES and
  the editor's other questions); the security review of kit-v2-2's finding 4 picks its option for him; and:
- **Waiting (from R1–R6; the full wording is in each topic's "Shared open questions" and the v2-research report):**
  - ABL-PREDICT (Mustafa, after a Feel Lab test that compares both ways on the Molotov and the Shield): does the client
    predict the player's own abilities?

## Routines (read with the routines API, get_trigger, 2026-11-10 20:10 UTC)
- **Night shift** (trig_01QpV6y6UaG1Na2q3jumRDxh): enabled; `CRON_TZ=Europe/Amsterdam 10 0-6 * * *` (hourly, 00:10 to
  06:10 Amsterdam), set 2026-11-10 18:00 UTC (daily at 06:00 from 2026-11-07 00:02 UTC until then); Opus 5.5; last run
  2026-11-10 05:04 UTC, succeeded; next 2026-11-10 23:10 UTC.
- **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ): enabled again 2026-11-10 16:07:30 UTC, after #46's merge (paused
  2026-11-09 18:22 UTC by Mustafa); `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, a schedule only, never pull-request
  events; Opus 5.5; runs 16:07 UTC (succeeded) and 18:46 UTC (failed: the usage limit); next 2026-11-11 06:45 UTC.
- EFFORT-SOURCE's evidence (the night shift at xhigh, the PR Advisor at max): `docs/STATUS_ARCHIVE.md`, "Routines",
  with TEST-RUN, the old routines and the known noise (PROBE-FILE (c)).
- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz): disabled, for him to delete (that also deletes its session).

## Follow-ups (open; their text: `docs/STATUS_ARCHIVE.md`, "Follow-ups", a live section)
A finding below the review budget's bar becomes a follow-up: its id goes here, its text in the archive.
- **Security, staying in STATUS until the foundation's gate step** (his go for #46, 2026-11-10: `docs/DECISIONS.md`):
  PX-1, the CI results trusted by name alone (its two test gaps ride it); W1 and W2, the two write-side gaps; W3, the
  folder check; the recorded Windows-name limits (PX-1 and its test gaps: the archive's "Follow-ups", "From kit-v3-5";
  W1-W3 and the limits: its "Open bugs and risks", kit-v3-5).
- **Kit, after the foundation's gate step (KIT-FREEZE-2):** F2 and F5 (kit-v3-4); F-1; L12, L14, L9 / SA-9, L13, L1's
  runners, A-4, FU-P3a, FU-P4a, FU-P4b, FU-P4c and FU-P4d (kit-v3-3); A3's plan readers; A3's cuts; CHECKPOINTS.md §7's
  cap line; E9's untested cases; KIT-ALIGN; the kit-v3-1 Windows test (its own change-set). For the kit's next change
  after the freeze: SA-KL-1 to -4 and KLT-1 to -8 (#53); ADV-NS-L2 (= SA-NS-2), -L3 and -L4 (#54); NS-TRAILER, NS-FIRST.
- **For the decision drafts' next revision:** "B3" means three things; the scale, "1 unit = 1 metre"; editing while
  the match runs (OQ-0005-CD4); OQ-0006-21's pointer; VISION_V2's line ranges in ADR 0005; and, from status-diet,
  the STATUS citations in the ADRs, the register, LOOP_V2 (:202, :214, :227) and ENGINE_OVERVIEW (:140).
- **For design sessions:** LATER's design session after the netcode proof test; what the note leaves open for rented
  servers; "the war around the matches" for VISION_V2 §7; the VFX style guide (LOOK (a)'s note) and the VFX tool's
  design session; and for the editor's architecture plan, B1-MODES, B1-LEAP, the objectives list, the fairness
  panel's pass rule, heatmaps and sightlines, and quality bars.
- **Docs passes:** the B3 report's and the kit-v3-2 report's edits (R-1, R-2); the toy-box tone; DOCS-ALIGN (the
  Advisor's documents); routine proofs to record; local smoke's Chromium build; the stale citations left unedited.
- **From status-diet (PR #49), after the gate step:** the STATUS citations left in advisor.md (:17, :286),
  netcode-architect.md (:24) and the decision docs (one hop through the table below until then); SD-N1, SD-N2, SD-N3
  and SD-N6 (security-auditor) and SD-L3 (the verifier): the archive's "Follow-ups", "From status-diet".
- **From foundation 2.1a (PR #50), for the gate step (2.3):** IE-1, IE-5 (Medium: CI's live mirrors), TE-2, TE-7,
  R2-1, A-1, N2, L-4 and the v2 root page against pre-move bases; for the drafts' next revision, five "before the
  foundation" lines left without a pointer; the archive's "Follow-ups", "From foundation 2.1a".
- **From foundation 2.1b (PR #51), for the gate step (2.3):** the Node 24 installer hang (his words: DECISIONS.md,
  2026-11-11, 20:44:29 UTC); the gate's stray-page check stops after the merge (security-auditor's F4, Low, open);
  PROMPTS-2.3 and the guard's `git -C` refusals (the 2.1b report, "Findings for 2.3").
- **From foundation 2.2 (PR #48), for 2.3:** AUTO-ASK, MANUAL-MODE, GIT-C, ADVISOR-TOOL-WORDS, MODELS-2026-11-12,
  REFUSED-PUSH, PIN-GUARD, G0, OVERFLOW-XCRATE-CI (or the first crate to depend on sim), TSCONFIG, REQUIRED-CHECK,
  SAVER-SKIP-CI, CRATE-NAMES, CI-ENFORCED-LATER, WASM-ARTIFACT, TWO-PLAYWRIGHTS, TOOLKIT-RESOLVES-UP, CARGO-NET-OFFLINE,
  CRATE-LICENCE, CUT-CARRY, CANCEL-ON-MAIN, LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE,
  NODE-26; later: OVERFLOW-PER-CRATE (with the first third-party crate), PACKAGE-ENTRIES, EMPTY-SLOTS, OQ-0002-8-TEST,
  BRIEF-STALE. Accepted as left open (his go for #48, item 5): SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE (step 2.3,
  each a CI edit that asks him); OVERFLOW-PANIC-RULE (with the first code in the room crate or a server crate).
- **From change-sets 0 and 1 (PRs #56, #58) and Part 15:** O1 (PM-0005-54) and O2, his calls with change-set M's plan;
  the first real export; the coat's motion; the machine lines; ADR 0002's package list; #58's other follow-ups and Lows.
- **From #60 (Mediums: with the quality gate step's kit changes, his go):** security-auditor's F1-F5, test-engineer's
  F1-F11, infra-engineer's L1-L6; his list for that plan: a CI lockfile check, the cut's part 2, KIT-REMOVED, ":" paths.
- **From change-set 2 (#61, in flight):** the ids in its row above; their text: the archive's block for PR #61.
- **Binding this chat:** `.claude/CHECKPOINTS.md` §3's hold; the advisor agent's ruling of #58: manifest and lockfile
  edits stay uncommitted and unbuilt, other code is committed by path only, until security-auditor answers.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage).
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's
  (R3-R6 spot-checked; unread sources).
- **Legacy (frozen):** 3a's ranked list; bot navigation; the editor and map bugs; the first-deploy tutorial; the
  wall-clock reads (an input for ADR 0004 part 4).

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
