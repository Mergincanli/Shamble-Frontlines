# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-28 (change-set 4c-1's docs pass, by lane 1's docs-keeper, on code 0645432f: 4c-1, the page's scene, built and reviewed in two rounds, its row and follow-ups; his two open marks of 2026-11-28, quoted by script; HEADROOM-FEET open; #79 (4b-2) merged, its row, the old top line, "Last merged" and lane 1's replaced lines moved to the archive by script under OLD-LINES)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged** (read with git, 2026-11-28): #79, 4b-2 (dae6d107, 2026-11-27 23:12:03 UTC); #78, ci-time (b8f02b0); #75, 4b-1 (b9aad01); #77 and #76, main-wall (47bb79e, 2671595); #74, ci-preview-handover (3b949c1a); #73, guard-wrappers (4a62873e); #70, 4a (2483e9a).
- **The kit:** KIT-FREEZE-2, FREEZE-FDN, his exceptions and additions, and his words of 2026-11-13 for step 2.3 (on kit,
  gate and CI edits): `docs/DECISIONS.md`; the bullet that quoted them whole: the archive's "Phase", at its end.
- **The day guard, after guard-wrappers** (merged as #73, 4a62873e, by his R4-END (1) and his go of 14:56 UTC): every
  stripped wrapper and keyword reads its options from one table; an unknown form, or a line at the turn limit (SA-R1-3),
  asks in a session and refuses a subagent; git or gh behind a program it does not strip asks; xargs asks unless a
  read-only program follows, also behind another program; the direct git helper check (SA-D-DIRECT (a)). **OPEN SECURITY
  FINDING: R4-1 (Critical, open): one more spelling hides git, gh or xargs from the guard's reading, for a session and a
  subagent; part is this change-set's class, part older; open on main; its fix is the guard change-set after the review tracks (SPEED-ORDER (b)).** Until
  then, his interim rule: "git and gh always start their own command, with no other program in front of them."
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **Main's wall is on** (2026-11-26, about 17:15 UTC, his click; TECHNICAL, "Main's wall"): only his account updates main, and gate and smoke stay required for every merge, his too; rollback: the wall's Enforcement status, Disabled.
  Open (T10): one day-session and one PR Advisor merge under it, then the other lane's token sitting (his TOKEN-RELEASE (c)); not measured: his own web merge, and the first merge from his second personal Claude account.
- **main** needs a pull request with the `gate` and `smoke` checks green; `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); gate, smoke and toolchain green on
  2483e9a (runs 45818982040 and 45818982052, read with gh's REST route 2026-11-26; read 2026-11-26, 14:24 UTC). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| none yet | change-set 4c-1, "The page's scene" (the first of 4c's two pull requests, his SIZE-4C (b)), by lane 1 (the chat he calls Builder V6): step 0 (his words of 2026-11-27 and 2026-11-28 in the register by script; LOOP_V2's §14 with its dated pointers, EDITOR_V2's pointer) and step S (the page reads the test scene's map field by field and draws it in greys by height under an overview camera; the readout's content and scene rows) | `claude/ecstatic-wozniak-5cbdzs` (stands in for `cs/4c-soldier`); his Advisor opens the pull request from a branch of its own at the scene's final head (his item 6 c) | risk tier 1, needs-mustafa (his Brave check; LOOP_V2 and the register are decision docs); built; reviewed in two rounds (six reviewers, then graphics-engineer and test-engineer on what changed, both APPROVE); planted defects 45 of 45 caught, 3 of 3 controls held (on 2b2f44c2); the verifier PASS; report `docs/reports/2026-11-28-cs4c1-scene.md`; the advisor agent's verdict on the final head comes in the builder chat | his Brave check on Sunday 29 November on that head, then his go; 4c-2 (the soldier, steps 1 to 7) is built on top meanwhile (his item 6 b) |

## Next
1. In his order (ORDER (b), 2026-11-10): his unit walk of 2026-11-09 (a docs change-set: UNITS.md §0 and LOOP_V2),
   then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); step 2.3's chat stays stopped; the first builder chat paused. Walk (his WALK-CUT
   (a)): 4a, 4b-1 and 4b-2 merged; **4c-1, the page's scene, reviewed**; then 4c-2 (the soldier, on the same branch), 4d, the link simulator, change-set 7 (ROADMAP, "Next / open").
3. Lane 2 (Builder V2), his order of 2026-11-25: #66 and #67 (ADR 0006's revision) merged (09faefa, c1da1bd);
   step 2.6's plan (its first edit after his go on the revised row); the carry of his look decisions when
   his block comes; one plan for the look test page and the model-check tool (EARLY-LOOK (a)), built after his go.
4. The Guard fix chat (`docs/DECISIONS.md`, "Queued"): MAIN-WALL done but T10 ("Phase"); then, by his SPEED-ORDER (b) of 2026-11-26, 17:37:10 UTC, the review tracks with SOURCE-GO (a), then one guard change-set with R4-1's fix and GUARD-KIT; each plan first.

## Waiting on Mustafa
- 4c-1, the page's scene: his Brave check on Sunday 29 November (art-director's six steps, about 6 minutes, come with its final head), then his go for that head. Open, needed before 4c-2's movement step: HEADROOM-FEET (question 13 of 4c's question file): may the game differ from his sandbox where the top of a block at or below the soldier's feet would stop him like a wall? Answered on 2026-11-28 (`docs/DECISIONS.md`, 11:43:45 UTC): MOVE-RULES (its row 9, the slopes, recorded there as open), NO-SOURCE, RESUME-FLASH, TUNE-HOME. Still open from 4a's report: SPRINT-DELAY-4; `docs/UNITS.md` §0's two lines (his unit walk).
- His accounts, his words (2026-11-26, 17:11:23 UTC, `docs/DECISIONS.md`): "My first personal account (I have two) and 1 enterprise (university), so I have 3 accounts working on the project" (ACCT-3, below).
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
- **From MAIN-WALL** (`docs/reports/2026-11-26-main-wall.md`; the archive's block "From MAIN-WALL"): SA-MW-R1 to -R8 (R5 and R6 done); T1e not run; his own web merge under the wall not measured; for the rule-file change-set, ACCT-LINES: CLAUDE.md's lanes lines say two accounts, he has three ("Waiting on Mustafa").; the advisor agent's results condition 5 (click 0's Actions > General screenshot) not received [assumed: open], a Low, no new ask
- **Merged pull requests' bullets** (moved by script): #49, #50, #51, #48 (gate step 2.3): the archive's "Moved from
  STATUS (2026-11-24)"; #60, #61, #64 to #71, #73, #74: its "Moved from STATUS (2026-11-25...)" and "(2026-11-26...)".
- **VISION-RULE** (his point 4c, `docs/DECISIONS.md`, 2026-11-26, 15:51:38 UTC; no ADR text rewritten now), his words:
  "for the next edit of ADR 0001 (part 9 (c) and its table Q14: hiding enemies by line of sight is now asked for, so
  R2-INT-OQ5's cost measurement at 48 players becomes due; part 9 (f): shots and their traces reach every player who can
  have them on screen, hidden shooter or not), of ADR 0004 part 13 and of ADR 0003's cutaway rule."
  His addition (`docs/DECISIONS.md`, 2026-11-27, 13:57:21 UTC, item 9; it sharpens version 2's point 2c; no code now) leaves open, his words: "the exact eye height, which reaches the game through my Export, and which points of an enemy's body the line is tested against".
- **From change-sets 4b-1 and 4b-2** (the archive's blocks of those names): 4B-AIM-REACH (the loader refuses a playable map
  beyond the aim's reach; the aim widens at the latest in Slice 3's change-set that sets the command codec, PM-0001-51; no
  numbered change-set holds it yet) and 13 more of 4b-1's, 4B-SIGHT-MARK among them. 4b-2's: for 4c, 4B-QUERY-RECORDING,
  4B-RAY-WASM, 4B-RAY-COUNTS, 4B-WORLD-ROOM, 4B-LIMIT-IMPORT; the CI Builder's 4B-CHROMIUM-TIME; hosting's 4B-CONTENT-CACHE,
  4B-LICENCES; 4B-HEAPLESS; the advisor agent's 4B-FIRST-DEFAULT, 4B-UNIT-COLUMN; 4B-WALLCLOCK, 4B-SERVE-LINKS. Met by 4c-1, closed when it merges: 4B-TYPES-READER (for the keys), 4B-MEMBER-KINDS, 4B-BOOT-CALLS.
- **From change-set 4c-1** (the archive's block of that name; each owned by the reviewer who found it): GE-F2 (graphics-engineer: the depth range, with its round 2 note on a window a few pixels wide; for the first large map); GE-F3 with P-2 (graphics-engineer, perf-auditor: draw calls on the largest map, merge the floor by material; with the look's change-set); GE-F4 (graphics-engineer: the WebGPU pipelines built when the map arrives; no change-set named); AD-4 (art-director: the readout over a short window; at 4c-2's step 4, where the overview leaves); TE2-1 to TE2-5 (test-engineer: five gaps in the scene's tests; no change-set named); V-1 (the verifier: the street's and the grass's greys are tested as different, not for which is darker; no change-set named).
- **His two open marks of 2026-11-28** (his DEPART-4C terms, items 12 b and c, quoted by script from his saved message, as in `docs/DECISIONS.md:220-221`): "PM-0004-60: 2,000,000 per CI run now. The mark is not met by that: it stays open in STATUS until the nightly job runs its 10 million, named with the change-set that brings it." and "PM-0004-20's 48-soldier run: not met by the one-soldier recording. It stays open in STATUS, named with the first build that can show it." Neither is named yet: no plan names the change-set that brings the nightly 10 million or the first build that can show 48 soldiers. The build plan's rows (`docs/reports/2026-11-17-build-plan-slice-1.md`, read 2026-11-28) give PM-0004-60's later clause as "its nightly run is CI work" (:654) and PM-0004-20's as "the stress scene's form at Slice 6" (:626); neither names a change-set or a build (whether that stress scene is the 48-soldier run is not settled here [assumed: open]).

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted);
  guard-wrappers' R4-1 (Critical, open: "Phase") and INPUT-RUN (a named limit, a security finding left open);
  MAIN-WALL's: the wall cannot tell a session from him, does not guard itself, and Disabled reopens it (SA-MW-11).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image. ci-preview-handover's upload runs after the cut on what the cut wrote (CUT-ESCAPE, accepted; PH-SA-2), and its artifact is readable by signed-in users for 7 days.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first); 3b's (no bound yet on how late a command runs, before
  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form); 4b-1's (the fuzz run
  cannot reach the caps, 4B-FUZZ-REACH); 4b-2's (four, in its report and the archive); 4c-1's (two, in the archive: nothing seen on WebGPU; the browser run's time). **Legacy (frozen):** 3a's ranked list (PR #15); bot
  navigation; the editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).
- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
