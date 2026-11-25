# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-25 (change-set 3b's docs pass, after #65's merge; main 28254a7)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged:** change-set 3a (#65, 28254a7, 2026-11-24 22:40:39 UTC, by his main Advisor on his tap); see ROADMAP.
- **The kit:** KIT-FREEZE-2, FREEZE-FDN, his exceptions and additions, and his words of 2026-11-13 for step 2.3 (on kit,
  gate and CI edits): `docs/DECISIONS.md`; the bullet that quoted them whole: the archive's "Phase", at its end.
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); gate, smoke and toolchain green on
  28254a7 (read 2026-11-24, 22:45 UTC). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| none yet | change-set 3b, "The loop closes" (the second half of change-set 3, his SIZE-3 (b)), by the chat he calls Builder V3: the page's keys reach a local server in the Worker through the real messages and come back (the codec of five messages, the transport interface and the loopback, the room, the client endpoint, local play's one step); a player sees a key diagram (W, A, S, D, Space, Shift) lit from the local server's echo and two new readout lines (messages; presses); the decoder test at its row's count, a mark started, never passed (PM-0001-34); the link simulator and the first recording moved to change-set 3c by the size ladder; report `docs/reports/2026-11-25-cs3b-the-loop-closes.md` (being written) | `claude/loving-hopper-6p0mg2` (stands in for `cs/3b-loop-closes`) | risk tier 1; built; the final code 6abf3d2a; 6,909 added lines against about 3,800 planned (stop line 7,600); the players' Wasm 53,813 bytes; 114 Rust tests, 139 Node tests, 18 boundary tests, 16 browser tests; 53 of 53 planted defects caught on it; gate, smoke and toolchain green on it (toolchain run 45537884772); six reviewers passed two rounds, nothing Critical or High open; no pull request yet (his GH-NEXT (a): the chat puts the title and text in its notes, his main Advisor opens the draft, he presses Ready and Merge) | the verifier, then the advisor agent; then his browser check (Brave, then Firefox) and his go |

## Next
1. In his order (ORDER (b), 2026-11-10): his unit walk of 2026-11-09 (a docs change-set: UNITS.md §0 and LOOP_V2),
   then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); step 2.3's chat stays stopped for now. Change-set 3b (above); beside it the
   small CI pull request by an Opus 5.5 chat (his answer of 2026-11-24, 22:44:50 UTC; E1: every crate's release tests,
   Firefox in CI by his FIREFOX-TESTS (b), the vector files' check, the test-only module's build step); whichever
   merges second takes the other in. **Then change-set 3c** (new): the link simulator and the first recording, moved
   out of 3b by the plan's size ladder (he was told on 2026-11-24, before anything moved), with the one new read-only
   number on the players' file (the state's hash) and `devtools`' three path edges that 3b declares and does not use
   yet; its plan holds the reviewers' items (the archive's "Follow-ups", "From change-set 3b"). Then change-set 4, the
   first outside crate in `sim` (the collision library; its plan names 3A-FMA-CHECK). The first builder chat stays
   paused; then lane 2's work; Slice 3 waits for #40, and picks how commands are timed (his decision of 2026-11-24).

## Waiting on Mustafa
- Change-set 3b: after the verifier and the advisor agent, his browser check (Brave, then Firefox) and his go for the
  approved head (his main Advisor opens the draft from the chat's notes; he presses Ready and Merge).
- **RELEASE-COUNT, open (his):** the presses mark's row (PM-0002-12) says the Worker counts presses and releases
  "through the edge counters"; the boundary draft (ADR 0002 part 5) has the post carry press and release edge
  counters; the netcode draft (ADR 0001 part 5) defines press counters only, and that is what 3b built: releases are
  worked out from presses and held keys, with two checks on the page (a button held again with no press; each tick's
  held keys against the post it used). (a) accept as built and correct the row's sentence at the drafts' next edit;
  (b) add a release counter per button to the messages. The builder recommends (a). Text: the archive, "From 3b".
- His to give: the roles and CLAUDE.md line of ACCT-3 (his third account, which counts like the other two); his go
  before anything goes to the Rust project (3A-RUST-REPORT); ADR 0004 part 7's pointer sentence, shown first
  (3A-PART7-POINTER).
- From Part 15 (its "Lane requests" issue: #59): give lane 2 Part 15 and ask for its needs from the block; tell lane 2
  which chat on his PC makes the model files; the VFX chat's exports; pass the VFX findings on.
- The netcode proof test's items (its plan in #40; the older list: the archive's "Phase", "Next (v2)"): OK each run's
  plan and cost cap; name the laptop and desktop; the Mac when one is available; the Safari workflow's own PR.
- Legacy, frozen: five old items of his and the old routine's deletion (moved 2026-11-24: the archive's "Phase", at its end).
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
  PX-1, the CI results trusted by name alone (its two test gaps ride it); W1 and W2, the write-side gaps; W3, the folder
  check; the Windows-name limits (PX-1: the archive's "Follow-ups", "From kit-v3-5"; the rest: its risks, kit-v3-5).
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
- **For the gate step (2.3), from status-diet and foundation 2.1a, 2.1b and 2.2 (PRs #49, #50, #51, #48):** their ids,
  moved by script on 2026-11-25 to the archive's "Follow-ups", at its end, each block with its pointer to its text.
- **From change-sets 0 and 1 (PRs #56, #58) and Part 15:** O1 (PM-0005-54) and O2, his calls with change-set M's plan;
  the first real export; the coat's motion; the machine lines; ADR 0002's package list; #58's other follow-ups and Lows.
- **From #60 (Mediums: with the quality gate step's kit changes, his go):** security-auditor's F1-F5, test-engineer's
  F1-F11, infra-engineer's L1-L6; his list for that plan: a CI lockfile check, the cut's part 2, KIT-REMOVED, ":" paths.
- **From change-set 2 (#61)** (3a's marked in the archive): for 3b NA-N1, -N2, -N4 to -N7, EA-F9, EA-F10's bindings
  part, EA-F13's one-writer item, TE-NT7, PA-N2; for 4 NA-N3; for 5 EA-N5, GE-N1, -N4 to -N6, PA-L2; for 5 and 6 GE-F7,
  PA-L4; for the gate step SA-F6, SA-SWAP, TE-NT2; drafts EA-F6, EA-F12; his four things (SLICE-FEEL, LANE2-LIST,
  PROJ-DRAW, FIREFOX); his check's readings (BR-GAP, not explained; BR-LATE, NOT-SHOWN, FF-WAKES, ADAPTER, TQ-FF).
- **From CI-WASM (#64)** (the archive's block for #64): TE-COUNT, TE-TYPE-PLANTS, TE-TSC-LINES, TE-WATCH, TE-CPU,
  TE-HIDDEN-FRAMES, IMAGE-CHROME, CUT-PATH, TOKEN-WORDS, GH-TOOLS after 3a, SAVER, MANUAL-MODE's record value.
- **From change-set 3a** (the archive's block): 3A-FMA-CHECK (his: first outside crate), 3A-RUST-REPORT (his),
  3A-PART7-POINTER; for the small CI pull request 3A-GEN-CHECK, 3A-RULES-STEP; built in 3b, closed by its merge
  3A-FROM-F64, 3A-HOOK-TEST, 3A-MANIFEST-COMMENT; for the gate step 3A-LINT-MULADD, 3A-SCAN-NAMES, 3A-CONFIG-FILE,
  3A-PEDANTIC; later 3A-FEATURE-VERSION, 3A-SCAN-HELPERS, 3A-PART5-CLASH, 3A-NAME-SECTION, 3A-RNG-COPY, 3A-STATS-ROWS,
  3A-PLANTS, 3A-FMA-LOG, 3A-NO-FMA, 3A-OTHER-BROWSERS; for the drafts 3A-DEPARTURES.
- **From change-set 3b** (the archive's block, in review): for change-set 7 3B-LATE-BOUND (no room behind a network
  transport without it), 3B-PLAUSIBLE; before online play 3B-ACKS, 3B-OUTAGE-256; for 4, 8 and 9 3B-ONE-PER-TICK; for
  5 3B-ALLOC-METHOD (PA-N2 carries over); for PM-0002-13 and -8 3B-KEY-POST, 3B-SATURATED; for Slice 3
  3B-REFUSED-FIELD; his: 3B-LONG-HOLD, at ADR 0001's next edit, with 3B-OQ6-POINTER; design: 3B-PACER-255; for the
  CI pull request 3B-OTHER-BROWSERS; later 3B-POOL-16, 3B-CSS-PROD, 3B-PUB-MODS; and change-set 3c's plan items.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first); change-set 3b's (no bound yet on how late a command
  runs, before online play; releases worked out, not counted). **Legacy (frozen):** 3a's ranked list (PR #15); bot
  navigation; the editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
