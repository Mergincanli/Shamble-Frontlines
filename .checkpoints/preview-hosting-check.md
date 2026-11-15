# Checkpoint: preview-hosting-check
- **Goal:** the plan for the preview hosting check (docs/SPIKE_PLAN.md entry 6; ADR 0006 parts 1-3 and 13; pass marks
  PM-0006-1 to -10 and -32), by Mustafa's task (his words of 2026-11-14, sent 2026-11-15 15:33:37 UTC; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-15-task.md`, sha256 e86c6b53...4009). The plan only: no build, no
  secret, no edit outside this checkpoint. · plan: `.checkpoints/preview-hosting-check/plan.md` (not yet written)
- **Branch:** claude/optimistic-albattani-mdoonz (stands in for cs/preview-hosting-check) · **Base:** main
  4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-15 16:47 Amsterdam (15:47 UTC)

## Steps done
1. Orientation · the session's model read with get_session at 2026-11-15 15:36:35 UTC: `session_context.model`,
   `configured_model` and `external_metadata.last_served_model` all `claude-opus-5-5`; effort max; the branch at main
   4460ef5. The advisor tool's model, from the transcript: `/advisor fable` -> "Advisor set to Fable 5.2" (event
   9eb677a4-cd3d-4dff-8eab-c7ba5dd050e1, 2026-11-15 15:33:29.746 UTC); every assistant turn carries
   `"advisorModel": "claude-fable-5-2"` (first: event a1548b9c-0d4c-473d-9bad-97370d817f57, 15:33:47.219 UTC);
   ~/.claude/settings.json holds `"advisorModel": "fable"`.
2. Reading (no commit): CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS, DECISIONS (Queued; 2026-11-11 to 11-13 entries,
   the five moments at DECISIONS.md:470-482), SPIKE_PLAN (:1-160, entry 6 :416-438, the foundation :440-472), ADR 0006
   (parts 1-4 :128-222, part 13 :531-581, part 14 :582-587, Q1/Q2 :605-623, OQ-0006-4 :876-887, OQ-0006-13 :988-1006,
   spike 1 :1198-1222, the marks' key and PM-0006-1 to -10, -32 :1268-1376, "When a mark fails" :1401-1415, the
   hosting checklist :1436-1457, migration :1467-1500), ADR 0002 part 10 (:655-706) and PM-0002-65 to -68
   (:1331-1334), GLOSSARY §5 (:560-665), r5 Question 6 (F1-F20 :760-900; REC1-REC5 :1585-1650; OQ4-OQ6 :1855-1862);
   the first builder chat's plan at 68addf9 (Part 8 :836-876, Part 10 :880-912, decisions :1165-1268) and checkpoint;
   the netcode chat's infra digest (macOS runner Safari versions, read 2026-11-05); .github/workflows/gate.yml (pins).
3. Facts read for the plan (no commit): workflow runs started by sessions show `actor` and `triggering_actor`
   `Mergincanli` (gate run 43576714278, toolchain runs 43576714298 and 43536148433, read 2026-11-15 15:44:54 UTC with
   the GitHub MCP tool); the MCP tool `actions_run_trigger` offers run_workflow, rerun_workflow_run, rerun_failed_jobs,
   cancel_workflow_run, delete_workflow_run_logs: no method approves a pending deployment (its schema, loaded
   2026-11-15 15:44:43 UTC); the guard's merge check (routine-guard.mjs:1474-1491) refuses an MCP merge in any
   repository without green `gate` and `smoke` checks, for every session; npm's registry answers (wrangler latest
   4.162.0) and `git ls-remote` reads public GitHub repositories (actions/upload-artifact, download-artifact tags).
4. Mustafa's task message saved byte for byte by script (the file above; 5830 bytes).

5. The saver started (15:52 UTC) and the advisor tool's consultation 1 (15:49 UTC, below).
6. Doc reads by two agents, both DONE: infra-engineer (C1-C9, notes `infra-engineer-docs.md`, last hand-back after
   16:13 UTC) and security-auditor (G1-G10, notes `security-auditor-docs.md`, hand-back before 16:10 UTC).
7. Draft workflows in `tr-draft/` (not a build: text for the plan's Part 7), each parsed as YAML with PyYAML 6.0.1:
   deploy.yml (172 lines; the pull-request and by-hand deploys merged into one file), close.yml (93), probe.yml (63),
   browsers.yml (48); the two token-test variants under `tr-draft/b9-edit/` and `tr-draft/b9-prod/`.

8. plan.md revision 1 written (1,183 lines; every embedded workflow block checked byte for byte against its draft file
   and its sha256): Parts 0-9, seven decisions, For Mustafa, the summary.

## Current step
9. The advisor agent's plan review of revision 1 (the `spikes/` tension named for its ruling).

## Next steps
1. Fold its findings (revision 2 if REVISE); consultation 2 (moment 5) before the plan is saved as done.
2. The closing message: the plan's path, the summary, his steps in order, the decisions, the chat's fullness.

## Open findings
- Refused by the guard, 2026-11-15 15:54:02 UTC: `git -C <dir> fetch ...` ("git: no option before the subcommand (run
  git from the folder, without -C, -c or --git-dir)"). Route taken: `cd` into each scratch folder under
  `.gate/preview-hosting-check/tags/` and ran git there (a shallow fetch of two tags to read their dates; nothing run).

- Process slip (his words of 2026-11-15: consult the advisor tool "always before you save a step as done and never
  after"): steps 5-7 were saved as done in the save of 16:16:22 UTC (0290625) and step 8 in the save of 16:21:42 UTC
  (d8451dc), with no consultation before them; consultation 2 followed at 16:22:00 UTC. From here on, each step is
  saved as done only after its consultation. To be told to him once, in the closing message.

## Facts for the plan (read by the session)
- Action pins: actions/checkout v7.0.1 3d3c42e5aac5ba805825da76410c181273ba90b1 and actions/setup-node v7.0.0
  820762786026740c76f36085b0efc47a31fe5020 (as gate.yml pins them); actions/upload-artifact v7.0.2
  cf430e030ddbb5b0abf93d22962f4752f3646cd9 (commit 2026-10-07); actions/download-artifact v8.0.2
  9000827ccba6bdab643e8b6fd33ac0654aef8333 (commit 2026-10-07): read with `git ls-remote` and a shallow tag fetch,
  2026-11-15 15:54-15:56 UTC.
- The deny list (.claude/settings.json) refuses `git push * main`, `git push --delete`, `gh api` with -X, --method,
  -f, -F, --field, --raw-field or --input, `gh pr merge`, and `npx *`: a session here cannot push a test repository's
  main, delete a remote branch, approve a pending deployment through gh, or run wrangler locally.

## Key numbers
- (none yet)

## Decisions taken
- His task (the file above): the plan for the preview hosting check; PREVIEW-START answered "it starts now, on the
  infra draft as proposed"; main model Opus 5.5, Fable 5.2 on the advisor tool; agents Opus 5.5, max effort for those
  that review, verify or design game and engine code; never push to another chat's branch; never touch spikes/.
- His three notes (his words of 2026-11-15, 17:00 Amsterdam, queued 16:03:05 UTC; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-15-notes.md`, 914 bytes): every "today" in the task message means
  2026-11-14; the first builder chat gets his go on Tuesday 2026-11-17, so its change-set 0 and Part 15 do not exist
  yet, and his decision on two builder chats stands on the task message; a third chat builds Shamble VFX in its own
  repository (Mergincanli/ShambleVFX): not this lane, never touched. The task is unchanged.

## Advisor tool consultations (his words of 2026-11-11: when, the question, what changed)
1. 2026-11-15 15:49:20 UTC (called; answer 15:51:36 UTC; transcript times), moment 1 · Q: is the approach sound (a
   separate test repository; one Pages Edit token in `preview` only, a reviewer with self-review off; probes and
   browser rows on GitHub runners; no custom domain)? · Changed: the test repository's place, the runner stand-ins and
   the one-token choice become decisions with both branches costed (the `spikes/` tension put to the advisor agent by
   name); a mark-by-mark table (as drafted / as planned / departure and who decides); every click read from a cited
   page first (else "if your screen differs, stop and tell me"); the reviewer gate's clicks and sittings counted and the
   close-time vs close-job time both recorded; the 24 h reading not on cron alone; asks listed (push to named branches
   there, his merges and branch deletion, the token straight from Cloudflare into GitHub's secret box); the saver
   before agents; no action toward the test repository this turn.
2. 2026-11-15 16:22:00 UTC (called; transcript time), moment 5 for plan revision 1 (late: see Open findings) · Q: is
   revision 1 ready for the advisor agent? · Changed: the `setup` branch is pushed without a pull request and opened
   only after "setup done" and his yes, so no workflow starts before `preview` is protected; the setup pull request's
   deploy and its merge's cleanup become two named runs (a pre-flight of the merged-close path), added to the count;
   Part 4's two over-claims hedged (reading a secret back; a custom domain on his zone) and the summary's "nothing
   outside Pages"; the check's own risk tier stated for the advisor agent to rule; the agent's brief gives questions,
   not conclusions; the saver checked first.

## Running work
- saver: background task bc4g68il4 · output /tmp/claude-0/-home-user-Shamble-Frontlines/a7e7f150-3195-52bd-a549-73b27426198b/tasks/bc4g68il4.output ·
  started 15:52 UTC · stops by 17:42 UTC (110 min) · last check: just started.
- agent infra-engineer (Opus by its file, effort high by its file): C1-C8 and the follow-up C9 · DONE.
- agent security-auditor (Opus by its file, effort high by its file): G1-G10 · DONE.
- saver last check 16:16 UTC: ok ("saved and pushed" at 16:02 and 16:12 UTC).
- Facts read by the session meanwhile: read_documentation `github.access` (15:52 UTC: a repository becomes attachable
  once the Claude GitHub App can reach it; add_repo attaches it); list_repos (15:52 UTC: five public repositories of
  his, all `can_push: true`; the app's repository-access mode is not shown).
