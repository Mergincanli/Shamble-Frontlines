# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-23 (the CI-WASM pull request, PR #64, after #61's merge; main f5030a3)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged:** change-set 2, the first page (#61, f5030a3, 2026-11-19 15:10:54 UTC, his go; CI green); see ROADMAP.
- **The kit:** KIT-FREEZE-2, FREEZE-FDN, his exceptions and additions, and his words of 2026-11-13 for step 2.3 (on kit,
  gate and CI edits): `docs/DECISIONS.md`; the bullet that quoted them whole: the archive's "Phase", at its end.
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); CI green on f5030a3 (gate run
  44551896122, toolchain run 44551894194). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| #64 | the CI-WASM pull request (it replaces #63 and #62), by the builder chat he calls Builder V3; risk tier 2 (`.github/workflows/toolchain.yml`, +97 −0). After it, a push or a pull request cannot go green unless the Wasm builds with the checked tool, every boundary test that runs passes on that Wasm, the Worker, the page and the specs type-check, and every browser test that runs passes, all inside the network cut. CI does not pin how many tests run: a skipped or removed test keeps it green (only zero tests turns it red; follow-up TE-COUNT). Before the cut: the wasm-bindgen tool's archive by his TOOL-SOURCE values, GitHub's own check of it (`gh release verify-asset`), its one program installed root-owned; inside it: the page built twice (production, development), the boundary tests, five type checks, the 12 browser tests in the runner image's Google Chrome. With it: the build's refusal of cargo settings from the environment moved into `build-checks.mjs` with a test; `dist-*/` in `.gitignore`; `packages/e2e/node-names.d.ts`; his words of 2026-11-19 and 2026-11-23 in `docs/DECISIONS.md` by script; his browser check of #61 (`docs/reports/2026-11-19-cs2-browser-check.md`); VISION_V2's dated note (his VISION-NOTE (a)); report `docs/reports/2026-11-19-ci-wasm.md` | `claude/loving-hopper-6p0mg2` (stands in for `cs/ci-wasm`) | draft, complete: two CI writes on his yes (cb476b1; the correction 2a654f3, after the first text stopped at its install check), three planted faults each red at its own step and taken back; `gate`, `smoke` and `toolchain` green on the code head 6d0b5b1, push and pull request; two review rounds, no Critical or High open; the verifier's and the advisor agent's verdicts on the final head: the report | his go (risk tier 2); by his GH-TOOLS (a) he presses Ready and Merge on GitHub himself (`docs/DECISIONS.md`, 2026-11-23, 17:15:07 UTC) |
| #63, #62 | the earlier drafts of the CI-WASM pull request, replaced by #64: #63 the project thread's (records only), #62 the first builder chat's (the plan and the first code); neither wrote a CI file | `claude/project-thread-qx9woh`; `claude/wonderful-hamilton-jdmlnf` | both open drafts (read 2026-11-23) | his word: "Ask me before #62 or #63 is closed." (2026-11-23, 16:30:46 UTC, point 9) |

## Next
1. In his order (ORDER (b), 2026-11-10), after status-diet (merged, #49): his unit walk of 2026-11-09 (a docs
   change-set: UNITS.md §0 and LOOP_V2), then the Slice 1 note of 2026-11-10, each in a fresh chat. His words:
   `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); the foundation (his order of 2026-11-11): #49, #50, #51, #48 and #52 merged; step
   2.3: the 2.3 chat stays stopped for now; #64 waits for his go. Then, by his words of 2026-11-23, 21:24:59 UTC:
   change-set 3a starts when #64 is merged, with no second ask, built by the chat he calls Builder V3 (BUILD-SEAT);
   Firefox in CI joins the small CI pull request after 3a (FIREFOX-TESTS (b), in place of (c)); three items for 3b come
   from his Advisor before 3b's build starts. The first builder chat stays paused (his words of 2026-11-19, 21:44:09
   UTC); then lane 2's work; Slice 3 waits for the netcode proof test.

## Waiting on Mustafa
- His go on #64 (risk tier 2); he presses Ready and Merge himself (GH-TOOLS (a), "for this pull request": the steps are
  in `docs/DECISIONS.md`, 2026-11-23, 17:15:07 UTC). Open: GH-NEXT, how pull requests are opened and merged for
  change-set 3a and after (Builder V3 cannot write to GitHub; options: the #64 report's "Decisions needed"). ACCT-3
  (the account of his project "Shamble Frontlines ALT" counts like his other two): its roles and the CLAUDE.md line.
- From Part 15 (its "Lane requests" issue: #59): give lane 2 Part 15 and ask for its needs from the block; tell lane 2
  which chat on his PC makes the model files; the VFX chat's exports; pass the VFX findings on.
- The netcode proof test's items (its plan in #40; the older list: the archive's "Phase", "Next (v2)"): OK each run's
  plan and cost cap; name the laptop and desktop; the Mac when one is available; the Safari workflow's own PR.
- Legacy, frozen (the archive's "In flight" and "Phase"): his Windows run ("Check a new machine", `tools/README.md`);
  the UI theme step 1 Brave check (DEV-panel FPS); `/permissions` locally (kit-v1-1); the editor inventory marks
  (docs-editor-v2); re-saving each old map once (map-dedupe-on-load). Routines: delete the old "Night shift" (below).
- Decisions still open for him: OQ-0006-22 (before slice 3); the follow-ups marked as his call (FU-P4b, B1-MODES and
  the editor's other questions); the security review of kit-v2-2's finding 4 picks its option for him; and, from R1–R6
  (the full wording: each topic's "Shared open questions" and the v2-research report), ABL-PREDICT (Mustafa, after a
  Feel Lab test that compares both ways on the Molotov and the Shield): does the client predict the player's own abilities?

## Routines (last read with the routines API, get_trigger, 2026-11-10 20:10 UTC; that read whole: the archive's "Routines")
- **Night shift** (trig_01QpV6y6UaG1Na2q3jumRDxh): enabled; `CRON_TZ=Europe/Amsterdam 10 0-6 * * *`; Opus 5.5.
  **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ): enabled; `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, a schedule only,
  never pull-request events; Opus 5.5. EFFORT-SOURCE's evidence (the night shift at xhigh, the PR Advisor at max), with
  TEST-RUN, the old routines and the known noise (PROBE-FILE (c)): `docs/STATUS_ARCHIVE.md`, "Routines".
- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz): disabled, for him to delete (that also deletes its session).

## Follow-ups (open, by id; a finding below the review budget's bar; their text: the archive's live "Follow-ups")
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
- **From change-set 2 (#61)** (the archive's block for #61): for change-set 3 NA-N1 to -N7, EA-F8 to -F10, EA-F13,
  EA-N3, -N5, -N6, VF-L4, VF-L7, GE-N1, -N4 to -N6, TE-NT3 to -NT5, -NT7, PA-N2, PA-L2, TE-L, SA-ENV, FAVICON, SA-TREE,
  RR-F5's fix, the `__pycache__/` line; for 5 and 6 GE-F7, PA-L4; for the gate step SA-F6, SA-SWAP, TE-NT2; for the
  drafts EA-F6, EA-F12; his four things (SLICE-FEEL, LANE2-LIST, PROJ-DRAW, FIREFOX); his browser check's readings
  (BR-GAP, not explained; FPS-LINE, NEG-ZERO, FF-WAKES, BR-LATE, ADAPTER, TQ-FF, NOT-SHOWN).
- **From CI-WASM (#64)** (the archive's block for #64): TE-COUNT, TE-TYPE-PLANTS, TE-TSC-LINES, TE-104, TE-MONO,
  TE-WATCH, TE-CPU, TE-HIDDEN-FRAMES, IMAGE-CHROME, CUT-PATH, TOKEN-WORDS, F-CHANNEL, GH-TOOLS for later pull requests,
  SAVER, MANUAL-MODE's record value; Firefox in CI after 3a.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's
  (R3-R6 spot-checked; unread sources). **Legacy (frozen):** 3a's ranked list; bot navigation; the editor and map bugs;
  the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
