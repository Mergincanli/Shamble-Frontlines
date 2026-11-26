# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-26 (change-set 4b-1's docs pass, first part, by lane 1's docs-keeper, on its branch with main 3b949c1 merged in: 4b-1 in flight, in review round 1; VISION-RULE's follow-up and pointers; 4b-1's follow-ups and the builder's two questions; the merged #73's and #74's rows, waiting bullets and top lines moved to the archive by script under OLD-LINES, with lane 1's replaced lines; the reviews' outcomes come in the main session's own pass)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged** (read with git, 2026-11-26): #74, ci-preview-handover (3b949c1, 2026-11-26); #73, guard-wrappers (4a62873, 2026-11-26); #70, 4a (2483e9a); #71, ci-after-3a (07d673b); #67 (c1da1bd).
- **The kit:** KIT-FREEZE-2, FREEZE-FDN, his exceptions and additions, and his words of 2026-11-13 for step 2.3 (on kit,
  gate and CI edits): `docs/DECISIONS.md`; the bullet that quoted them whole: the archive's "Phase", at its end.
- **The day guard, after guard-wrappers** (waiting for his go; his R4-END (1): "Merge what is approved."): every
  stripped wrapper and keyword reads its options from one table; an unknown form, or a line at the turn limit (SA-R1-3),
  asks in a session and refuses a subagent; git or gh behind a program it does not strip asks; xargs asks unless a
  read-only program follows, also behind another program; the direct git helper check (SA-D-DIRECT (a)). **OPEN SECURITY
  FINDING: R4-1 (Critical, open): one more spelling hides git, gh or xargs from the guard's reading, for a session and a
  subagent; part is this change-set's class, part older; open on main too; its fix is the next guard change-set.** Until
  then, his interim rule: "git and gh always start their own command, with no other program in front of them."
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); gate, smoke and toolchain green on
  2483e9a (runs 45818982040 and 45818982052, read with gh's REST route 2026-11-26; read 2026-11-26, 14:24 UTC). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| none yet | change-set 4b-1, "Content" (the first of 4b's two pull requests, his SIZE-4B (b)), by lane 1 (the chat he calls Builder V4): the content crate `crates/content` with no dependency (one reader for a cut grammar of JSON, the canonical writer, the bundle, the movement kind, the map kind with its own exact rules and the aim's reach, the time unit's conversion), the simulation's time unit, the reader's proofs and a bounded fuzz run natively, under Node and in the browsers, the old tuning's numbers and durations extracted by script, the time unit's rounding report, and the links check (his LINKS-CHECK (c)); 6,591 written lines against the stop line 7,450 | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4b-content`) | risk tier 1, no browser check; built (code head ef2edec, main 3b949c1 merged in), in review round 1 (rust-reviewer, engine-architect, security-auditor; test-engineer follows) | the reviews and their fix pass, the verifier; the advisor agent's verdict on the final head; then his go for that head |

## Next
1. In his order (ORDER (b), 2026-11-10): his unit walk of 2026-11-09 (a docs change-set: UNITS.md §0 and LOOP_V2),
   then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. Walk (his WALK-CUT (a); no 3c: his ORDER-3C (c),
   `docs/DECISIONS.md`): 4a merged (#70); **4b-1 in review**; 4b-2 (steps 6 to 12: the collision library and world, his
   Export's data, the page's scene; after a short re-read by the advisor agent); 4c the first soldier; 4d sights; then
   the link simulator, before change-set 7. The first builder chat stays paused; Slice 3 waits for #40.
3. Lane 2 (Builder V2), his order of 2026-11-25: #66 and #67 (ADR 0006's revision) merged (09faefa, c1da1bd);
   step 2.6's plan (its first edit after his go on the revised row); the carry of his look decisions when
   his block comes; one plan for the look test page and the model-check tool (EARLY-LOOK (a)), built after his go.
4. The Guard fix chat, his order of 2026-11-26 (`docs/DECISIONS.md`, "Queued"): MAIN-WALL, then one guard change-set
   with R4-1's fix and GUARD-KIT, then the review tracks with SOURCE-GO (a); each plan first.

## Waiting on Mustafa
- Change-set 4b-1 (no browser check): his go for the head the advisor agent approves; and the builder's two questions,
  in its report. **SIGHT-FLAG**: should 4b already give each map object a "blocks sight" flag? The builder and
  engine-architect recommend not in 4b: it comes in Slice 2 with the sight check. **DUR-1**: which fire rate counts as
  the old game's? The old code never carried a timer's leftover time, so the Shabzon fired about 720 shots a minute at
  60 frames a second where its table value of 0.072 s gives 833; it binds change-set 8; game-designer's read
  (`.checkpoints/cs4b-content/game-designer-durations.md`, gone from the tree at the pull request's close, readable at
  the branch's commits) recommends the table's value now, tuned by play.
- Answered on 2026-11-26 (`docs/DECISIONS.md`, 12:21:17, 13:05:03 and 14:09:13 UTC): CONTENT-SOURCE, DEPART-4,
  EARLY-LOOK-ORDER, LINKS-CHECK, SOURCE-GO, SIZE-4B, FEATURE-COMPARE; FIREFOX-REPLAY (carried by #74). STEP-60: his
  Export at 30 steps is in (15:07:24 UTC; saved whole in the chat's notes); DIFF-LIST is asked at 4b-2's data step
  (step 8). Later, from 4a's report: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; the two
  `docs/UNITS.md` §0 lines the Hornet's entry clashes with (his unit walk).
- His to give: ACCT-3's roles and CLAUDE.md line (his third account, counted like the other two); his go before anything
  goes to the Rust project (3A-RUST-REPORT); ADR 0004 part 7's pointer sentence, shown first (3A-PART7-POINTER).
- Lane 2, at his own timing (TR-END met): the test
  repository's hourly checker off before 2 December, then its archive, then the test project and its Access app deleted.
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
- **From change-sets 0 and 1 (PRs #56, #58) and Part 15:** O1 (PM-0005-54) and O2, his calls with change-set M's plan;
  the first real export; the coat's motion; the machine lines; ADR 0002's package list; #58's other follow-ups and Lows.
- **Merged pull requests' bullets** (moved by script): #49, #50, #51, #48 (gate step 2.3): the archive's "Moved from
  STATUS (2026-11-24)"; #60, #61, #64 to #68, #70, #71: its "Moved from STATUS (2026-11-25...)" and "(2026-11-26...)".
- **From #60, #61, #64, change-set 3a (#65), guard-env-fix (#69) and ci-after-3a (#71)** (moved by script, OLD-LINES): the archive's "Moved from STATUS (2026-11-25...)" and "(2026-11-26, ci-preview-handover's docs pass)".
- **From guard-env-fix (#69):** SA-R2-2 (very long lines against the hook timeout; answered in guard-wrappers' report, nothing changed without his word), V-2 (the folder tracking after a split-string value), the reason-text nit. Closed by guard-wrappers at its merge: GEF-SAME (what is left of its class: R4-1) and SA-R1-3; CAP decided, his (a) of 2026-11-25.
- **From ci-preview-handover** (the archive's block "From ci-preview-handover"; the ids are that docs pass's, not his): done on its branch, closed when it merges, TOKEN-WORDS and 4A-FIREFOX-REPLAY (his FIREFOX-REPLAY); CUT-ESCAPE stays on record as accepted (his 15:05 point 6) and its later fix is CUT-ESCAPE-FIX ("the cut's code as its own throwaway user"); PH-SA-2 (CUT-ESCAPE's acceptance now also rests on lane 2's preview treating the build as data); PH-IE-4 (a re-run and the artifact's name, read on the first re-run); for the gate step 2.3 PH-KEEP-NAMES; for lane 2 PH-MERGE-REF; on the first run PH-TIME and PH-ACTIONS-SETTING; at the merge PH-DELETE (the action's read-only copy and the Firefox folder).
- **From guard-wrappers** (`docs/reports/2026-11-26-guard-wrappers.md`; the archive's block): INPUT-RUN (a named limit:
  programs outside the guard's wrapper table that run their input are not inspected, the class of stated limits L1-L4);
  next kit change-set: R2-2, R2-3, R3-3 to R3-6, ADV-R4-2, ADV-R4-4, R4-2, SA-c, SA-e, shell function definitions.
- **VISION-RULE** (his point 4c, `docs/DECISIONS.md`, 2026-11-26, 15:51:38 UTC; no ADR text rewritten now), his words:
  "for the next edit of ADR 0001 (part 9 (c) and its table Q14: hiding enemies by line of sight is now asked for, so
  R2-INT-OQ5's cost measurement at 48 players becomes due; part 9 (f): shots and their traces reach every player who can
  have them on screen, hidden shooter or not), of ADR 0004 part 13 and of ADR 0003's cutaway rule."
- **From change-set 4b-1** (in review; the archive's block "From change-set 4b-1"; the 4B- ids are its docs pass's):
  4B-AIM-REACH: the loader refuses a playable map beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to
  655.35 m). The aim widens in the first change-set whose map leaves that square (none in Slice 1; Slice 2's plan says
  whether its maps do), at the latest in Slice 3's change-set that sets the command codec for online play (PM-0001-51),
  beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet. ADR 0002's next revision: 4B-FEATURE-POINTER;
  the drafts' next revision: 4B-DEPART-3, 4B-PM5-3 (his acceptance); the CI Builder: 4B-CI-CONTENT; 4d's import and
  change-set 8: 4B-OLD-TIMERS. Closed by 4b-1's step 4, at its merge: 4A-S3-READ's read side.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted);
  guard-wrappers' R4-1 (Critical, open: "Phase") and INPUT-RUN (a named limit, a security finding left open).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image. ci-preview-handover's upload runs after the cut on what the cut wrote (CUT-ESCAPE, accepted; PH-SA-2), and its artifact is readable by signed-in users for 7 days.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first); 3b's (no bound yet on how late a command runs, before
  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form; the
  recording tests' link read, 4A-S3-READ, closed on 4b-1's branch, at its merge). **Legacy (frozen):** 3a's ranked list (PR #15); bot navigation; the
  editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).
- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
