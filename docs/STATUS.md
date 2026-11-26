# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-26 (ci-preview-handover's docs pass, by the CI Builder's docs-keeper, on main 2483e9a: the preview hand-over in flight; the merged #70's row and the follow-ups bullets of the merged #69 and #71 moved to the archive by script; in lane 1's lines only #70's merge and FIREFOX-REPLAY's answer; lane 2's lines left as they are)._
_Last updated: 2026-11-26 (guard-wrappers' docs pass, on the lane branch with main 2483e9a merged in: guard-wrappers waits for his go with R4-1 open; #70 merged; #70's row and the follow-ups bullets of the merged #67, #68, #70 and #71 moved to the archive by script; lane 2's #66 and #67 shown as merged where STATUS still had them waiting)._
_Last updated: 2026-11-26 (step 2.6's docs pass after his answers of 12:21 UTC, lane 2, on #72's branch: #72's row; lane 2's lines in Next, Waiting on Mustafa, Follow-ups and Open risks; every other lane's and chat's line left as it was)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged** (read with git, 2026-11-26): #70, 4a (2483e9a, 2026-11-26); #71, ci-after-3a (07d673b); #67 (c1da1bd); #69 (311c048); #68, 3b (3ab9eae).
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
| none yet (title and text in the notes) | ci-preview-handover, by the chat he calls CI Builder (his block of 2026-11-26, 13:20 Amsterdam; lane 2's request on issue #59): at the end of the `toolchain` job, on pull requests from this repository only, a check that the tested player build holds only plain files and folders with no name that begins with a dot, then the hand-over of that build to lane 2's preview workflow as the artifact `preview-site`, kept 7 days (`actions/upload-artifact` v7.0.2 at cf430e0, his UPLOAD-SOURCE (a)); TOKEN-WORDS and two comments made true; the recording's replay test joins the rule tests in Firefox (FIREFOX-REPLAY) | `claude/keen-tesla-v807cf` (stands in for `cs/preview-handover`) | risk tier 2; no browser check; written at d12e3a8 on his CI-WRITE-3 (a), GUARD-STEP (a) and HIDDEN-FILES (b) (2026-11-26, 15:05 Amsterdam), both files at his sha256 values; one review round before his yes (security-auditor: nothing stops the change, its SA-1 Medium is the accepted CUT-ESCAPE; infra-engineer: APPROVE); in the container Firefox 161 3 passed (the replay test included), Chromium 17 passed, the check on the real build and on seven planted faults; the upload step has never run, and no preview starts until lane 2's `preview.yml` is on main; CI: the push run on d12e3a8 green (the new steps skipped on a push; Firefox 3 passed, the replay test among them); the pull request run to read; report `docs/reports/2026-11-26-ci-preview-handover.md` | his Advisor opens the draft and watches the first run; the advisor agent's review of the final head; his go for that head |
| none yet | guard-wrappers (his SAME-GAP (a): nothing in front of git or gh gets through the day guard unread), by the chat he calls the Guard fix chat; what it does: "Phase", "The day guard" | `claude/confident-gates-tbnq38` (stands in for `cs/guard-wrappers`) | risk tier 2 (his go; no pull request yet); code head 83bc2417; main 2483e9a merged in (7be0a933) in a separate worktree; guard tests 1551/1551; four security-auditor rounds (the fourth by his leave, the last), R4-1 left open by his R4-END (1); 166 added lines of code and tests (stop line 200); report `docs/reports/2026-11-26-guard-wrappers.md` | the advisor agent's review of the pushed head; his Advisor's merge check; his go for that full head id |
| #72 | step 2.6, the preview links per pull request (lane 2, the chat he calls Builder V2): `.github/workflows/preview.yml` and `preview-close.yml`, `hosting/`, `docs/runbooks/previews.md`; report `docs/reports/2026-11-26-step-2-6-previews.md` | `claude/optimistic-albattani-mdoonz` (stands in for `cs/step-2-6-previews`) | draft; risk tier 2; built; review rounds 1 and 2 closed (infra-engineer REVISE, then APPROVE; security-auditor APPROVE in both; test-engineer REVISE, then REVISE narrow, fixed in 80a9b6a8, no third round); gate, smoke, toolchain and `preview / tests` (175 pass) green on code head 84a3ea44; the advisor agent on the guard: ESCALATE, answered by his decisions of 2026-11-26; no verifier run yet; a new chat with only this repository takes over the sensitive steps (his GUARD-OFF (a)); until then no change under .github/ or the kit, no main-folder move, no merge | the new chat, started by him from Builder V2's start block; the token sitting, which waits for the Guard fix chat's protection for main (the token's end date comes here then; no token yet); the first live link needs the CI Builder's hand-over step (asked on #59); the throwaway `wall-test.yml` out at the token sitting; the verifier, ready, the advisor agent's review of the head, his go |

## Next
1. In his order (ORDER (b), 2026-11-10): his unit walk of 2026-11-09 (a docs change-set: UNITS.md §0 and LOOP_V2),
   then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. 3b (#68), the small CI pull request (#71) and 4a (#70) merged.
   **No change-set 3c:** his ORDER-3C (c), 2026-11-25, 16:59:52 UTC (`docs/DECISIONS.md`): "Split 3c at its seam." The
   recording is change-set 4a (#70); "The link simulator follows Walk as its own small change-set and is merged before
   change-set 7 starts." Walk is four change-sets (his WALK-CUT (a)): 4a the recording, 4b content and the world, 4c the
   first soldier (he plays; an early look inside it), 4d sights; then the link simulator, then 7. Change-set 4's plan,
   section 8 (where to read it: 4a's report), places each 3c item by name. The first builder chat stays paused; Slice 3 waits for #40 and picks how commands are timed.
3. Lane 2 (Builder V2), his order of 2026-11-25: #66 (09faefa) and #67 (c1da1bd) merged; step 2.6 (#72) in flight above, stopped until his GUARD-OFF answer; the carry of his look decisions when his block comes; one plan for the look test page and the model-check tool (EARLY-LOOK (a)), built after his go.
4. The Guard fix chat, his order of 2026-11-26 (`docs/DECISIONS.md`, "Queued"): MAIN-WALL, then one guard change-set
   with R4-1's fix and GUARD-KIT, then the review tracks with SOURCE-GO (a); each plan first.

## Waiting on Mustafa
- ci-preview-handover (CI Builder): his go for the head the advisor agent approves, after his Advisor has opened the draft and watched the first run (the push run on d12e3a8 green; the pull request run is the first to run the two new steps); a red on the new steps or on Firefox's replay test comes to him first; if the run fails at its start because the action is not allowed, the chat tells him which setting to change (his 15:05 points 5 and 7).
- guard-wrappers: his go for the head id the advisor agent approves, after his Advisor's merge check (no browser check).
- From 4a's report (#70, merged 2026-11-26, 2483e9a): EARLY-LOOK-ORDER (his go's point 4: "I decide then"); before 4b's first edit
  CONTENT-SOURCE and DEPART-4; FIREFOX-REPLAY (it widens the Firefox entry he cut to the rule tests; answered by his block of 2026-11-26, 13:20 Amsterdam; carried by ci-preview-handover); LINKS-CHECK
  (4A-S3-READ). STEP-60 (a): his Export at 30 steps before 4b's step 6, then DIFF-LIST. Later: MOVE-RULES, NO-SOURCE,
  RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; the two `docs/UNITS.md` §0 lines the Hornet's entry clashes with (his unit
  walk).
- **RELEASE-COUNT, decided by him on 2026-11-25: (a), accept as built** (`docs/DECISIONS.md`, 11:38:37 UTC), with
  two conditions: no report calls any part of the presses mark passed; and the two drafts are made to agree at
  their next revision, which comes to him before it is written (**3B-RELEASE-DRAFTS**, his).
- His to give: ACCT-3's roles and CLAUDE.md line (his third account, counted like the other two); his go before anything
  goes to the Rust project (3A-RUST-REPORT); ADR 0004 part 7's pointer sentence, shown first (3A-PART7-POINTER).
- Lane 2's step 2.6 (#72): his answers of 2026-11-26 (`docs/DECISIONS.md`): GUARD-OFF (a), a new builder chat with only this repository takes over the sensitive steps, from Builder V2's start block; MAIN-WALL-AGAIN (a); GUARD-KIT (a); GIT-IN-NODE (a); TAG-CHECK (a), recorded as untested. Waiting: he starts the new chat; the real token waits for the Guard fix chat's protection for main (his TOKEN-TONIGHT (b)). At his own timing (TR-END met): the test repository's hourly checker off before 2 December, then its archive, then the test project and its Access app deleted.
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
- **From step 2.6** (#72, lane 2; below the bar, from its two review rounds; their text is here, none in the archive yet): the lint of `preview.yml` does not check six guards: WRANGLER_LOG, metrics, job permissions, github-script, the download path, the upload's working folder (TE-H's rest); nor code beside the token: a custom shell, NODE_OPTIONS or BASH_ENV, a job container or services (the security-auditor's round-2 Low); check-headers fetches one file at a time (the infra-engineer's round-1 Low); `preview / tests` is not a required check and the gate does not read `hosting/` (TE-K).
- **From guard-env-fix (#69):** SA-R2-2 (very long lines against the hook timeout; answered in guard-wrappers' report, nothing changed without his word), V-2 (the folder tracking after a split-string value), the reason-text nit. Closed by guard-wrappers at its merge: GEF-SAME (what is left of its class: R4-1) and SA-R1-3; CAP decided, his (a) of 2026-11-25.
- **From ci-preview-handover** (the archive's block "From ci-preview-handover"; the ids are that docs pass's, not his): done on its branch, closed when it merges, TOKEN-WORDS and 4A-FIREFOX-REPLAY (his FIREFOX-REPLAY); CUT-ESCAPE stays on record as accepted (his 15:05 point 6) and its later fix is CUT-ESCAPE-FIX ("the cut's code as its own throwaway user"); PH-SA-2 (CUT-ESCAPE's acceptance now also rests on lane 2's preview treating the build as data); PH-IE-4 (a re-run and the artifact's name, read on the first re-run); for the gate step 2.3 PH-KEEP-NAMES; for lane 2 PH-MERGE-REF; on the first run PH-TIME and PH-ACTIONS-SETTING; at the merge PH-DELETE (the action's read-only copy and the Firefox folder).
- **From guard-wrappers** (`docs/reports/2026-11-26-guard-wrappers.md`; the archive's block): INPUT-RUN (a named limit:
  programs outside the guard's wrapper table that run their input are not inspected, the class of stated limits L1-L4);
  next kit change-set: R2-2, R2-3, R3-3 to R3-6, ADV-R4-2, ADV-R4-4, R4-2, SA-c, SA-e, shell function definitions.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted);
  guard-wrappers' R4-1 (Critical, open: "Phase") and INPUT-RUN (a named limit, a security finding left open).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image. ci-preview-handover's upload runs after the cut on what the cut wrote (CUT-ESCAPE, accepted; PH-SA-2), and its artifact is readable by signed-in users for 7 days.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first); 3b's (no bound yet on how late a command runs, before
  online play; releases worked out, not counted); 4a's (the recording tests still follow a link, 4A-S3-READ; two planted
  divergences only in a stand-in or native form). **Legacy (frozen):** 3a's ranked list (PR #15); bot navigation; the
  editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).
- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL. Step 2.6 (#72; the report's "Risks"): a guard that seems on but is not (lane 2's chat, Builder V2, ran without it since 2026-11-15: two repositories in one chat, so no repository hooks loaded), and asks that may not reach him in auto mode (kit-wide); rated High for the kit by the security-auditor (Critical if the hooks ran and an ask was settled without him), not a follow-up: his GUARD-KIT (a) gives the kit change to the Guard fix chat after its protection for main, and his GUARD-OFF (a) moves step 2.6's sensitive steps to a new chat with only this repository; M1 did not test a tag push (his TAG-CHECK (a): untested, because the rule admits branch `main` only, with 0 tags allowed); the report's risk 2 (no token yet; the token sitting waits for the Guard fix chat's protection for main); unproven until live: M5 to M11 (the first link, the comment, the headers served, reconcile, the count).

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
