# Advisor verdicts: plan amendments (A) and the docs-after-r7 plan (B), verbatim

The advisor agent in session_0173vVo5q59ameR6cnN9BENg, hand-back at 2026-10-27T18:22:14.276Z (prompt: `prompts/advisor-amendments-review.txt`,
notes: `advisor-amendments-review.md`).

## A. Amendments to the R1–R6 plan: APPROVE, with 8 conditions

**Short answer:** Items 1–7 apply the Advisor's instructions correctly, and nothing else in the plan changes. Item 8 is acceptable. Wave 1 does not need to stop, and its 11 agents do not need a new brief.

**What I checked myself:**
- **The files:** `plan.md`, `advisor-verdict.md`, `plan-amendments.md`, `provenance.md` and the checkpoint `.checkpoints/v2-research.md`.
- **Item 2 (R6 scope):** it matches the R6 row of brief §4 word for word: hosting for previews and EU game servers; costs; logs, metrics and crash reports; balance telemetry (§6.6). The security baseline is the only part dropped.
- **Item 5 (R7-MARKS):** it matches option (a) in the R7 report (`docs/reports/2026-10-27-r7-legacy-inventory.md:94`). 118 inventory rows carry "look brief pending".
- **Branch:** `cs/v2-research` was pushed at 8ff1c74, on top of 07ab240. `docs/LEGACY_INVENTORY.md` and `docs/legacy-tuning.json` are on it.
- **The saver:** it stages only `.checkpoints/` (`.claude/scripts/checkpoint.mjs:151`), so it cannot commit a stray file elsewhere.
- **Item 8's premise:** `nproc` is 4. I could not check the workflow runtime's limit of min(16, CPUs − 2) agents at once; that is the session's claim.
- **Wave 1 notes so far (11 files, 169 lines):**
  - Each source records the date it was read and its tier.
  - Each unit starts from inventory rows by id (for example CS1, GC5, GP14) and VISION §4 and §6.
  - Every fetched domain is on the allow list in `.claude/settings.json` (raw.githubusercontent.com, docs.aws.amazon.com, gafferongames.com, datatracker.ietf.org and the others).
  - So the prompts carry the rules. No new brief is needed.

**Conditions:**
1. **Save the prompts now.** Condition 10 asks for "a launched copy of each script". For background agents that means the exact prompts. Save the 11 wave-1 prompts word for word now, for example under `.checkpoints/v2-research/prompts/<agent>-<topic>-<unit>.txt`. Save every later stage's prompts before it launches. In the per-topic table, record agent ids where it says "run ids". Without the saved prompts, "restart with the same prompt" depends on the session's context surviving a cut-off.
2. **The 00:10 stop.** Background agents have no workflow cache. At the 23:50 Amsterdam check:
   - stop every agent still running and record which ones;
   - after 08:30, restart each with its saved prompt plus one line: "resume from your notes at <path>".
3. **What the checkpoint must carry forward.** Today it has neither of these:
   - condition 9's line: "the synthesis plan verifies every ADR claim and number, including those from R3–R6";
   - the deferred security baseline, carried to the synthesis: ADR 0006 lists it as open, or it is researched before 0006 is accepted.
4. **Where the deferral shows.**
   - The report's coverage table (condition 12) shows the security-baseline question as "deferred by the Advisor, 2026-10-27", not as a gap.
   - R1's cross-origin isolation question (the headers that shared memory needs) stays in R1. It is not part of the deferred baseline.
   - R6's hosting part states, for each host, whether it can send those headers. That is a hosting fact that R1 depends on.
5. **Item 4's fallback.** If docs-after-r7 has not merged when research reaches its docs-keeper step, condition 11 applies in full again: the research PR adds the earlier STATUS and ROADMAP entries itself.
6. **Item 5 in later prompts.** The assembler prompts and the wave-2 prompts say: the inventory's marks are working recommendations (R7-MARKS (a)), and rows marked "look brief pending" stay pending. The running units need nothing, because the inventory's column is already headed "recommended".
7. **What item 3 means for merging.** With the PR Advisor routine off, nothing merges without Mustafa's "merge #n" in a session's chat. That includes this docs-only research PR, which the routine would otherwise have merged by itself. The research PR's "For Mustafa" section must say so.
8. **Spend and the verifier's prompt.**
   - Item 8 raises the peak spend but not the total. If the spend limit cuts all 11 at once, count what came back and restart from notes. Launch only the plan's agents for that stage.
   - Nit: the commit message of 8ff1c74 starts with "checkpoint(v2-research):" twice. Leave the prefix out of `-m`, because the script adds it.
   - Some notes mark claims "confirmed per fetch summary". Such a claim was checked against WebFetch's summary of the page, not its text. The verifier's prompt should require the exact quoted text before it counts one as confirmed.

## B. docs-after-r7 plan: REVISE, with 8 changes

**What I checked myself:**
- **Merge facts on main:** PR #26 is f2ccc13 (2026-10-25), PR #25 is 4c24d07 (2026-10-27), PR #27 is 07ab240 (17:36 UTC), and VISION_V2 and the brief were added 2026-10-24. The plan's facts match.
- **STATUS on main:** last updated 2026-10-17. Its sections are Phase, In flight, Targets, "Decisions pending", "Open bugs and risks" and Health.
- **ROADMAP:** "Next / open" (line 62) still says N1 is next.
- **Issue #16:** I read the body through the REST API (last updated 2026-10-26) and the DOMAINS-ACCEPT comment 6396928581.
- **CI:** `gate.yml` runs on push and on pull_request, so draft PRs get both checks.
- **Remote branches:** `cs/` branches are accepted, and `cs/kit-v3-1` exists at e6d9012. I did not open it.

**Changes needed:**
1. **Branch.** Use `cs/docs-after-r7`, from origin/main 07ab240, in its own worktree. The stand-in `claude/` branch is only for a session that can push nothing else, and this session pushed `cs/v2-research`. If a `cs/` push is refused, say so in the PR body.
2. **Use the sections STATUS has.**
   - Put the taken decisions in "Decisions pending", following the existing pattern at line 67: "Taken by the Advisor 2026-10-27: …".
   - Routines can get a new short "Routines" section.
   - Name the new phase "v2 phase 0 (V2_PHASE0_BRIEF)". Keep the existing "Phase 0" paragraph, labelled "legacy phase 0 (environment)". Today they share one name.
3. **Explain every decision code in plain words, on the line where it appears.** STATUS is the first page Mustafa reads, so the reporting rule applies:
   - BRANCH-2 (a): research on `cs/v2-research`, kit-v3-1 on its own branch, kit-v3-1 lands first if they collide.
   - SPEND-1 (b): full checks for R1 and R2, spot checks for R3–R6.
   - KV3-MERGE, ROUTINE-EFFORT, SPIKE-SCOPE, WRITE-GUARD, DOMAINS-ACCEPT: a few words each, plus the link.
   - MANAGED (b): state the fact ("Mustafa is not an admin of the university's Claude plan") and what it rules out, but only if the session knows the question from a source it may read. Otherwise write the fact plus "a kit-v3-1 question, recorded for when it returns", and nothing invented.
4. **kit-v3-1 wording.** Write a line that is true under either reading, so this PR does not wait for the decision below. In "In flight":
   > kit-v3-1 (write guards for kit and gate files, a builder agent, a default effort, the secret-pattern fix) | `cs/kit-v3-1` | **paused**: "shelved" by the Advisor, 2026-10-27; this session leaves the branch untouched | Waiting on: Mustafa and the Advisor. Note: the R7 session was still committing to this branch at 18:01 UTC, on an earlier Advisor message (17:32 UTC: finish it there with a smaller, fixed scope and open its PR). Which instruction holds is open (see "Decisions pending").
   - Add the same open question to "Decisions pending".
   - Keep the plan's risk 1: if a kit-v3-1 PR opens, the line changes before merge.
5. **ROADMAP "Next / open".** Mark the line 62 queue superseded by v2, pointing to STATUS and the brief, the same way the plan treats STATUS. Today only "Recent work" changes, which leaves "N1 (next)" standing. Leave "Phases" as is.
6. **Issue #16 text.**
   - Make the condition "Until kit-v3-1 returns and is merged". DOMAINS-ACCEPT still depends on WRITE-GUARD being merged, and amendment item 1 says the same.
   - Use "paused (the Advisor, 2026-10-27: 'shelved')" to match STATUS.
   - Put the new night rule (only items with no web access; a night run stopped at a permission prompt lists it in the morning report) in the "Rules" list, with its source. Keep the DOMAINS-ACCEPT link.
   - The rest of the plan's text for #16 is fine: queue item 1 becomes done, stays a plain bullet, no item is added.
7. **Keep the issue edit reversible.** The issue has no history in git. Put #16's old and new body text, or the diff of the changed sentences, in the PR body.
8. **Report and evidence.** Earlier docs-only change-sets had no report file, so the PR body serves as the report: what changed, what was checked against which source, the gate result, Decisions needed, For Mustafa. On the gate:
   - `tools/node_modules` is missing in the main checkout. If `npm ci` cannot run, say so; CI's gate and smoke checks on the head are then the gate evidence.
   - At PR review I must run the gate myself, so the review checkout needs `npm ci` and Playwright's Chromium.

Accepted as planned:
- The merge path: a draft PR, the advisor agent reviews its head in this session, then Mustafa's "merge #n", then mark it ready and merge with the approved full SHA as the expected head, with gate and smoke green.
- It is fine if Mustafa gives his go before switching the routine off. The expected head protects the merge.
- No `.claude/`, `CLAUDE.md` or `tools/` edits. The night rule lives in #16 and STATUS, not in `NIGHT_SHIFT.md`, which would be a rules change.

## Decisions needed
- **KIT31-STATE: Is kit-v3-1 paused, or being finished in the R7 session?**
  - **The conflict:** at 17:32 UTC the Advisor told the R7 session to finish kit-v3-1 there with a smaller, fixed scope and open its PR for Mustafa. About 5 minutes later this session was told "kit-v3-1 is shelved … record it in STATUS as paused". At 18:01 UTC the R7 session was still committing to the branch.
  - **Why it matters:** two sessions are acting on opposite instructions. kit-v3-1's write guard is what lets research move to the night shift (DOMAINS-ACCEPT). Also, "on Opus 4.8" departs from CLAUDE.md's rule of Opus 5.5 and no other model; only Mustafa can allow that.
  - **Option (a):** paused. The R7 session stops, its commits stay on `cs/kit-v3-1` unreviewed, and any PR it opens stays a draft.
  - **Option (b):** finished in the R7 session, and "shelved" meant "not in this session". Its PR comes to Mustafa as needs-mustafa (it changes `.claude/`), and the Opus 4.8 departure is named in it.
  - **Recommendation:** Mustafa says which holds, in both sessions. Until then STATUS uses the wording in B.4, which is true either way, so docs-after-r7 does not wait for the answer.

## For Mustafa
1. Switch the PR Advisor routine off on its routine page (trig_01YPyWAsQmsaQoydhMGG3CkJ). An agent can't: the API refused because the routine was created outside an agent.
2. Answer KIT31-STATE above, and tell the R7 session too.
3. Later: say "merge #<n>" for docs-after-r7 once the advisor approves its head. With the routine off, every PR, research included, needs your go to merge.

**What I could not check:**
- The routine states and the refused update_trigger call: no routine tools here.
- The kit-v3-1 branch contents: not opened, as instructed.
- MANAGED's original question: it is not on main.
- The workflow runtime's 2-agent limit.

I posted nothing and committed nothing. My notes, which end in DONE, are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-amendments-review.md`.
