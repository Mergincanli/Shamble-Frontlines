# STATUS: the one-screen dashboard
_Mustafa and the Advisor read this first. At most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): the phase, in flight,
next, waiting on Mustafa, the routines' state, pointers. His decisions, word for word, newest first:
`docs/DECISIONS.md`. The rest: `docs/STATUS_ARCHIVE.md`. Kept current by docs-keeper after every change-set._
_Last updated: 2026-11-24 (change-set 3a's docs pass, after #64's merge; main 1ec6f6c)._

## Phase
- **v2 phase 0** (`docs/V2_PHASE0_BRIEF.md`; the rebuild: `docs/VISION_V2.md`): the process upgrades (kit-v3-0 to
  kit-v3-5), the research (R1-R7) and the decision drafts are merged. The six drafts (`docs/adr/0001`-`0006`, with
  `docs/ENGINE_OVERVIEW.md` and `docs/SPIKE_PLAN.md`) are the basis for the spikes and stay Proposed (the stop's
  answers, 2026-11-03: `docs/DECISIONS.md`), but for his SCOPE (a) of 2026-11-10: "I accept ADR 0002's part 1 alone
  (the repository layout, with the legacy constraints applying to legacy/ only)", recorded in ADR 0002 by 2.1b (#51).
- **Last merged:** CI-WASM (#64, 1ec6f6c, 2026-11-24 10:15:31 UTC, by his main Advisor on his tap; CI green); see ROADMAP.
- **The kit:** KIT-FREEZE-2, FREEZE-FDN, his exceptions and additions, and his words of 2026-11-13 for step 2.3 (on kit,
  gate and CI edits): `docs/DECISIONS.md`; the bullet that quoted them whole: the archive's "Phase", at its end.
- **The legacy game is frozen** (critical fixes only). Legacy phase 0 stays PARTIAL until his Windows run.
- **main** is protected (a pull request with the `gate` and `smoke` checks green); `toolchain` runs on every push and
  pull request but is not a required check (REQUIRED-CHECK, a finding for step 2.3); CI green on 1ec6f6c (gate run
  45373057216, toolchain run 45373056953). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).

## In flight (one line per PR)
| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #40 | the netcode proof test (SPIKE_PLAN item 1; a throwaway spike, `spikes/netcode-proof/`) | `claude/netcode-proof-test-plan-hcky5g` | draft; U1-U3 done; U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC) | its units, runs and report; his OK of each run's plan and cost cap |
| none yet | change-set 3a, "Rules and boundaries" (the first half of change-set 3, his SIZE-3 (b)), by the chat he calls Builder V3: the simulation's rules under tests (the scan of `sim`, our own ChaCha8 against the published vectors, the float vectors natively and in WebAssembly, `mul_add` refused and pinned by his MUL-ADD (a)), the panic hook's text on the page, the build's refusals; report `docs/reports/2026-11-24-cs3a-rules-boundaries.md` | `claude/loving-hopper-6p0mg2` (stands in for `cs/3a-rules-boundaries`) | risk tier 1; no pull request yet (his GH-NEXT (a): the chat puts its title and text in its notes folder, his main Advisor opens the draft, he presses Ready and Merge); built in 8 steps; the last commit that changed what runs is 8619d539, the corrected head 8d277cf2 (comments, docs), CI green on both; two review rounds by five reviewers, then a third by security-auditor on one fix (APPROVE: the High closed); the verifier PASS and the advisor agent APPROVE on 8d277cf2; his browser check of 2026-11-24, 21:48:49 UTC: errors 0 in Brave and in Firefox (`docs/reports/2026-11-24-cs3a-browser-check.md`: the Shields setting and the page opened as a file are shown; no gap with the window covered); the notes: commit 6491b839 | the advisor agent's read of the last commit; his go for its full SHA |
| #67 | ADR 0006's small revision after the preview hosting check, by lane 2 (the chat he calls Builder V2), on his ONE-OFF-HEADER (a) and STEP4-COUNT (b): the header mark (PM-0006-2) judged on production's own addresses, with the reason in the row, and production's one-off links with the host's header recorded, not judged; part 1 weighs Workers static assets in a short paragraph from Cloudflare's docs read 2026-11-25 (nothing measured on Workers; in the draft, Pages stays); the build-count question said plainly to be not answered; the pass words of PM-0006-7 and checklist step 4 re-stated as the advisor agent's reading of his STEP4-COUNT (b), never as his decision; part 1's "no cap on previews" dated; report `docs/reports/2026-11-25-adr0006-header-row.md` | `claude/optimistic-albattani-mdoonz` (stands in for `cs/adr0006-header-row`) | a draft until the advisor agent approves its final head, then ready; risk tier 1 (an ADR draft, and his message of 2026-11-25, 11:07:07 UTC in `docs/DECISIONS.md`) | the advisor agent's review of the final head; his go, which is his go on the revised row and comes with his answer PASS-WORDS (does his go cover the re-stated words?); his word on checklist step 4 |

## Next
1. In his order (ORDER (b), 2026-11-10): his unit walk of 2026-11-09 (a docs change-set: UNITS.md §0 and LOOP_V2),
   then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`, "Queued".
2. v2: the netcode proof test (#40); foundation step 2.3: its chat stays stopped for now. After change-set 3a, in the
   approved plan's order: the small CI pull request (E1: every crate's release tests; Firefox in CI by his
   FIREFOX-TESTS (b); the vector files' check, a build step for the test-only module; each CI edit's exact text, and
   which model writes it, asked of him first: his "CI work stays with Opus 5.5" was set aside for #64's); then 3b (his
   Advisor's three items first; "3b stays first in line for the main builder"); then change-set 4, the first outside
   crate in `sim` (the collision library), whose plan names the check that the players' WebAssembly holds no fused
   multiply-add (3A-FMA-CHECK). The first builder chat stays paused; then lane 2's work; Slice 3 waits for #40.
3. Lane 2 (Builder V2), in his order of 2026-11-25: #66 (merged, 09faefa); ADR 0006's small revision of the header row
   (#67, waiting for his go); step 2.6's plan (after #67's merge; its first edit after his go on the revised row); the
   carry of his look decisions when his block comes; then one plan for the look test page and the model-check tool (his
   EARLY-LOOK (a)); nothing built before his go on that plan.

## Waiting on Mustafa
- Change-set 3a: his go for the head the advisor agent approves (his browser check is in; GH-NEXT (a): his main
  Advisor opens the draft from the chat's notes at 6491b839, he presses Ready and Merge). His to give: the roles and CLAUDE.md
  line of ACCT-3 (his third account, which counts like the other two); his go before anything goes to the Rust project
  (3A-RUST-REPORT); part 7's pointer sentence at ADR 0004's next revision, shown to him first (3A-PART7-POINTER).
- Lane 2's ADR 0006 revision (#67): his go on the final head (his go on the revised row) with PASS-WORDS (does it
  cover the re-stated pass words?); his word on checklist step 4. At his own timing (TR-END met): the test repository's
  hourly checker off before 2 December, then its archive, then the test project and its Access application deleted.
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
- **For the gate step (2.3), from status-diet (#49), foundation 2.1a (#50), 2.1b (#51) and 2.2 (#48):** moved by
  script, byte for byte, 2026-11-24 (OLD-LINES): the archive's "Follow-ups", "Moved from STATUS (2026-11-24)".
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
  3A-PART7-POINTER; for the small CI pull request 3A-GEN-CHECK, 3A-RULES-STEP; for 3b 3A-FROM-F64, 3A-HOOK-TEST,
  3A-MANIFEST-COMMENT; for the gate step 3A-LINT-MULADD, 3A-SCAN-NAMES, 3A-CONFIG-FILE, 3A-PEDANTIC; later
  3A-FEATURE-VERSION, 3A-SCAN-HELPERS, 3A-PART5-CLASH, 3A-NAME-SECTION, 3A-RNG-COPY, 3A-STATS-ROWS, 3A-PLANTS,
  3A-FMA-LOG, 3A-NO-FMA, 3A-OTHER-BROWSERS; for the drafts 3A-DEPARTURES.
- **From the preview hosting check** (lane 2; the archive's block): for step 2.6's plan PHC-CLICKLESS, PHC-LOCKFILE,
  PHC-TOKEN-SETUP, PHC-PROBES, PHC-DELETE, PHC-ACTIONS-READ, PHC-ERROR-PATH; at the merge PHC-TR-END; PHC-SCRIPT-LOWS.
- **From ADR 0006's revision** (#67, lane 2; the archive's block): ADR6-ROLLBACK (Medium), -WK-ROUTE, -WK-BIC,
  -PAGES-RETENTION, -PM7-PAGES, -PM7-FIGURE, -WORDING; for step 2.6 PHC-PREVIEW-NOINDEX (Medium); his: WORKERS-NEXT.

## Open risks (their text: `docs/STATUS_ARCHIVE.md`, "Open bugs and risks", a live section)
- **Kit:** kit-v3-5's merge-check limits (Windows names, gitignored names, case-only twins, symlinks, more than 100
  files) and W1-W3; kit-v3-3's stated limits (L1-L18, R-PLAIN, D-ASK); kit-v3-2's items 1-5; kit-v3-1's items 1-2;
  kit-v2-2's finding 4 (Medium, open) and its Lows; kit-v2-1's night-rules note (issue #16's comments are trusted).
- **CI:** the smoke job's Chromium step stalled once (PR #42); on #50 (3f29cb8, 8d8c422) its library install failed in 5
  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image.
- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first). **Legacy (frozen):** 3a's ranked list (PR #15); bot
  navigation; the editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).
- **Preview hosting (lane 2):** PHC-HEADER, the header mark not met as written (back to the draft; revised by #67,
  waiting for his go); PHC-PROBE-MAIL.

## Where STATUS's old sections went: the table is at the end of `docs/STATUS_ARCHIVE.md` (moved there 2026-11-18)
