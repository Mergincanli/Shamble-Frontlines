# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-11 (foundation 2.1a, PR #50, its docs pass; main bb2d817)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in its Status line by 2.1b.
- **Last merged:** status-diet, STATUS-DIET with LANE-RULES (PR #49, bb2d817, 2026-11-11 09:03:11 UTC, by merge
  commit, on Mustafa's go after the advisor's APPROVE of 2e6e5d2, PR comment 6779064873): STATUS split into this
  dashboard, `docs/DECISIONS.md` and `docs/STATUS_ARCHIVE.md`; CLAUDE.md's "Lanes and accounts" and the advisor-tool
  lines. Every merged change-set: `docs/STATUS_ARCHIVE.md`, "In flight"; `docs/ROADMAP.md`.
- **The kit is frozen** until the foundation's gate step (KIT-FREEZE-2, and what he meant by it, FREEZE-FDN:
  `docs/DECISIONS.md`, "Queued"). His exceptions: a broken main, STATUS-DIET and LANE-RULES (carried out by
  status-diet, #49, merged); and by FREEZE-FDN it "does not stop the smallest edits the foundation needs to keep the
  kit true after its own moves" ("Each such edit still asks me and merges on my go"). His two additions of
  2026-11-11 for the foundation chat: `docs/DECISIONS.md`.
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); CI green on bb2d817 (run
  42991918886). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done, U4 in build | its units, runs and report; his OK of each run's plan and cost cap |
| #50 | the foundation, 2.1a: the legacy gate and tools read both layouts (no file moves); replaces #47 (his BRANCH-47 (a), 2026-11-11) | `claude/pensive-heisenberg-aazidf` (his personal account) | draft: the docs pass done; the review round and the report next | the review round and the report, then the advisor's review of its ready head and his go (risk tier 2: gate files; decision docs); 2.1b next |
| #48 | the foundation, 2.2: the toolchain (pins, empty crates and packages, the offline CI job) | `claude/laughing-pasteur-fwya24` | draft | 2.1b's merge |

## Next
1. In his order (ORDER (b), 2026-11-10), after status-diet (merged, #49): his unit walk of 2026-11-09 (a docs
   change-set: UNITS.md §0 and LOOP_V2), then the Slice 1 note of 2026-11-10, each in a fresh chat. His words:
   `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); the foundation in his order of 2026-11-11 (#49 merged; then 2.1a, #50 in #47's
   place; then 2.1b; then 2.2, #48). Wave 1 runs one heavy spike at a time (SPIKE_PLAN: the netcode proof test, the
   sim core spike and its 60 Hz run, the boundary spike, the look proof test once its models exist; the preview
   hosting check alongside any). _Pointer: his decision of 2026-11-10 lets the second account plan and build
   foundation steps 2.1 and 2.2 now (`docs/DECISIONS.md`, 2026-11-10)._ Then phase 1 and the playable slices
   (SLICE-ORDER, 2026-10-31: `docs/DECISIONS.md`).

## Waiting on Mustafa
- His go for #50 (2.1a; risk tier 2: gate files; `docs/DECISIONS.md` and the decision docs' pointers), once the advisor
  approves its final head; no Brave check for 2.1a. Then, in his order of 2026-11-11, 2.1b and #48 (2.2).
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
- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz): still disabled, for him to delete (deleting it also deletes its
  session).

## Follow-ups (open; their text: `docs/STATUS_ARCHIVE.md`, "Follow-ups", a live section)
A finding below the review budget's bar becomes a follow-up: its id goes here, its text in the archive.
- **Security, staying in STATUS until the foundation's gate step** (his go for #46, 2026-11-10: `docs/DECISIONS.md`):
  PX-1, the CI results trusted by name alone (its two test gaps ride it); W1 and W2, the two write-side gaps; W3, the
  folder check; the recorded Windows-name limits (PX-1 and its test gaps: the archive's "Follow-ups", "From kit-v3-5";
  W1-W3 and the limits: its "Open bugs and risks", kit-v3-5).
- **Kit, after the foundation's gate step (KIT-FREEZE-2):** F2 and F5 (kit-v3-4); F-1; L12, L14, L9 / SA-9, L13, L1's
  runners, A-4, FU-P3a, FU-P4a, FU-P4b, FU-P4c and FU-P4d (kit-v3-3); A3's plan readers; A3's cuts; CHECKPOINTS.md §7's
  cap line; E9's untested cases; KIT-ALIGN; the kit-v3-1 Windows test (its own change-set).
- **For the decision drafts' next revision:** "B3" means three things; the scale, "1 unit = 1 metre"; editing while
  the match runs (OQ-0005-CD4); OQ-0006-21's pointer; VISION_V2's line ranges in ADR 0005; and, from status-diet,
  the STATUS citations in the ADRs, the register, LOOP_V2 (:202, :214, :227) and ENGINE_OVERVIEW (:140).
- **For design sessions:** LATER's design session after the netcode proof test; what the note leaves open for rented
  servers; "the war around the matches" for VISION_V2 §7; the VFX style guide (LOOK (a)'s note) and the VFX tool's
  design session; and for the editor's
  architecture plan, B1-MODES, B1-LEAP, the objectives list, the fairness panel's pass rule, heatmaps and sightlines,
  and quality bars.
- **Docs passes:** the B3 report's and the kit-v3-2 report's edits (R-1, R-2); the toy-box tone; DOCS-ALIGN (the
  Advisor's documents); routine proofs to record; local smoke's Chromium build; the stale citations left unedited.
- **From status-diet (PR #49), after the gate step:** the STATUS citations left in advisor.md (:17, :286),
  netcode-architect.md (:24) and the decision docs (one hop through the table below until then); SD-N1, SD-N2, SD-N3
  and SD-N6 (security-auditor) and SD-L3 (the verifier): the archive's "Follow-ups", "From status-diet".

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted).
- **CI:** the smoke job's Chromium step stalled once (PR #42).
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's
  (R3-R6 spot-checked; unread sources); R7's (its line numbers go stale when the legacy files move).
- **Legacy (frozen):** 3a's ranked list; bot navigation; the editor and map bugs; the first-deploy tutorial; the
  wall-clock reads (an input for ADR 0004 part 4).

## Where STATUS's old sections went (STATUS-DIET, 2026-11-10; old line numbers: `git show 2d22b69:docs/STATUS.md`)
| STATUS at 2d22b69 (lines) | Now |
|---|---|
| The header (2-27), Phase (30-139), In flight (142-185), Targets (188-195) | `docs/STATUS_ARCHIVE.md`, under the same names |
| Decisions pending: his blocks, the relayed Advisor's, and the queue of his words (198-272, 281-420, 674-799, 813-846, 880-928) | `docs/DECISIONS.md` |
| Decisions pending: Follow-ups, KIT-ALIGN and DOCS-ALIGN (421-673, 862-879) | `docs/STATUS_ARCHIVE.md`, "Follow-ups" |
| Decisions pending: the advisor agent's rulings, the carry-forwards, the old ADR lines (273-280, 800-812, 850-861, 929-946) | `docs/STATUS_ARCHIVE.md`, "Decisions pending: the rest" |
| Decisions pending: Waiting (847-849) | this file, "Waiting on Mustafa" |
| Routines (949-1008), Open bugs and risks (1011-1217), Health (1220-1233) | `docs/STATUS_ARCHIVE.md`, under the same names |
