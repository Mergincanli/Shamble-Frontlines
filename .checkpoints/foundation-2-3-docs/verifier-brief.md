# The verifier's brief for PR #52 (foundation 2.3's first docs pass), head a1d0317

You are the verifier for pull request #52 of Mergincanli/Shamble-Frontlines: foundation step 2.3's first docs pass.
Read main's verifier instructions (`git show origin/main:.claude/agents/verifier.md`) and follow them; this brief adds
the change-set's specifics.

You work read-only:
- never edit a tracked file;
- never switch branches or check out another ref in the main folder;
- never commit or push;
- never touch `spikes/` or pull request #40's branch.

A worktree under `.gate/` is allowed if you need one (`git worktree add .gate/fdn23-verify-base 4d21c1a`).

**Saving your progress (`.claude/CHECKPOINTS.md` §4):**
- Before you start, write your plan into `.checkpoints/foundation-2-3-docs/verifier-1.md`.
- Then write one notes line at once after each check or finding: the command, the record, the verdict. A restart
  continues after the last complete block.
- Never commit: the session's saver commits.
- Scratch text and small scripts go in that notes folder; builds and binaries go under `.gate/fdn23-verify/`.
- The guard refuses some command forms: `git -C`, a loop that writes to a variable path, a `node -e` line with
  comments, and `sed -i`. Use literal paths, script files in the notes folder, and the Write tool.

## Inputs
- **Head:** a1d0317b5d6d6bef920e394c51c07ca869efb601 (round 2's fix pass 8e4fe7c and its follow-up). Verify this.
- **Base:** main 4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300.
- **The notes as used:** commit fe2482c holds the scripts in `.checkpoints/foundation-2-3-docs/`. The tree's scripts
  are the same; later commits are checkpoint saves only. Check with
  `git diff fe2482c -- '.checkpoints/foundation-2-3-docs/*.mjs'`.
- **The approved plan:** `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b, 2c and 2d. The advisor
  agent's rulings are in `advisor-plan-rev1.md`.
- **Change type:** docs only. They are decision docs, so risk tier 1. Every claim is checked against a record. The
  symbol diff must be +0 −0.
- **The report:** `docs/reports/2026-11-13-foundation-2-3-docs.md`.
- **The review notes:**
  - round 1: `r-words.md`, `r-state.md` and `skeptic-*.md`;
  - round 2: `r2-words.md`, `r2-state.md` and `r2-skeptic-R-WORDS-RW2-F1.md`.
- **The session's records, read-only:**
  - the transcript: `/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl`;
  - its agents' transcripts: `.../7261f3d7-f651-586a-9c39-c052553b68a3/subagents/` (workflow agents under
    `subagents/workflows/`).
- **CI:** GitHub Actions runs of the branch `claude/sweet-mayer-grsukw`.
  - Read them with the GitHub MCP tools. Load them first with ToolSearch:
    `select:mcp__github__actions_list,mcp__github__actions_get,mcp__github__get_job_logs,mcp__github__pull_request_read,mcp__github__get_commit`.
  - Use `actions_list` with `list_workflow_runs` (filtered by branch) and `list_workflow_jobs`.
  - Only if the tools will not load, list CI as not verified.
  - Post nothing on GitHub: no comment, no review, no label.
- **E8 names you.** It says R-STATE and the verifier checked every SHA, run id and time. So check every SHA, run id and
  time in the report against its record, and list in your notes what you checked.
- **His words stay out of your notes and scripts.** Write pointers (file:line, transcript uuid), lengths and booleans
  only, never a copy of his words, not even a fragment. Both review rounds ended with a disclosure of exactly that.

## What to verify
1. **The gate.**
   - Run `node tools/gate.mjs 4d21c1a --no-smoke`: every check must PASS, with symbols +0 −0.
   - The smoke cannot start in this container: its Chromium is not the toolkit's. CI's smoke job is the record.
   - Also run `node .claude/scripts/repo-map-check.mjs`.
2. **The scripts.** Run each from the repository root; each must exit 0:
   - `node .checkpoints/foundation-2-3-docs/words.mjs`;
   - `report-appendix.mjs --check`;
   - `decisions.mjs --check --negative`: four entries, and seven negative controls caught, E-STATUS's quote among them;
   - `pointers.mjs --check`;
   - `fixes.mjs --check`;
   - `quotes.mjs`;
   - `scope.mjs`;
   - 2.2's own entries still pass: `node .gate/fdn23-docs/s22/decisions-entries.mjs --check` (a copy of 2.2's script,
     run from the repository root).

   Read what each checks, and say where a check could pass while its claim is false.
3. **Every claim in the report about what ran, checked against a record:**
   - each advisor-tool consultation's time, and their count: the transcript's advisor calls (`r2w-advisor-model.py`
     lists them). Eleven were made before the head's commit, all answered by Fable 5.2;
   - each "done" save's time and SHA, against `git log -1 --format=%cI <sha>`;
   - each CI run id and its result (E7);
   - each agent's model and effort, from the agents' transcripts;
   - each refusal and its time (`python3 -I .checkpoints/foundation-2-3-docs/r2-errors.py <transcript> <session dir>`),
     including the harness's block of a foreground sleep at 23:38:22 UTC;
   - the counts in E1 to E4 and E8, against the scripts' output;
   - round 1's 23 findings and round 2's 10 (nine rows), with their dispositions, against the reviewers' notes, and
     each disposition against the tree.
4. **RW2-F1's fix, checked against the records:**
   - every statement on the advisor model in the report body and in the appendix's intro, against the transcript's
     `advisor_tool` attachment entries and each advisor call's usage record (the `advisor_message` iterations and their
     `model`);
   - the `/advisor fable` output, quoted exactly from the system entry at 18:30:08 UTC;
   - the claim "No chat message named the advisor model before round 2", against every main-session assistant text
     before 22:48 UTC;
   - the 22:48:47 UTC message.
   - Two of the session's chat updates after the compaction (22:54:40) are in neither the transcript nor the sessions
     API's event record, and the report says so. Check that the records hold no such update; do not look for one.
5. **His words:**
   - every block of Appendix A, byte for byte against its source: A1, A2 and A4 from the transcript by uuid; A3 from
     merge commit 4d21c1a's message;
   - the four DECISIONS.md entries' quotes, byte for byte against Appendix A, and E-STATUS's negative control;
   - every pointer's quote, inside his decision;
   - no decision of his recorded as given when it is not: his go for #52, step 2.3's plan and any kit edit are all
     still to come.
6. **The state docs:**
   - STATUS, ROADMAP and the archive are true for main 4d21c1a and for #52's state at a1d0317;
   - STATUS has at most 120 lines;
   - #40's row matches the API, read-only. The report's second read of it comes at step 8.
7. **The appendix's cut-off (2026-11-13 23:47:48 UTC).** Say whether any message of his came after it. If one did, a new
   appendix item is due before the final commit: report it.
8. **Pending by design at this head.** These are not findings; step 8's final commit settles each:
   - `{{NOTES-SHA}}` in the report's header;
   - STATUS's #52 row ("draft"; waiting on "the verifier, ...") and ROADMAP's #52 entry ("in review"; "Next: the
     verifier, ...");
   - the Reviews section's "The verifier and the advisor agent's review of the final head: to come.";
   - E7's "its follow-up: listed in the final commit";
   - #40's second read;
   - the pull request's body: still revision 1's, rewritten at ready;
   - any consultation after 23:39:10 UTC: it goes into the list at the final commit, or into the PR body after it;
   - the `.checkpoints/` deletion: the advisor agent's final-head review checks it, so verifier.md's item 3 is "to
     come", not a FAIL.

## Severity and the report
- Use CLAUDE.md's scale: Critical, High, Medium, Low.
- A wrong time, SHA, run id, count or quote is High ("evidence that does not hold").
- Report as verifier.md says: `VERDICT: PASS | FAIL | PASS WITH NOTES`.
- Give each problem with its file:line and its record.
- Keep the final message short; the details go in your notes.
