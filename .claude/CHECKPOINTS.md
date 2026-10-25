# Checkpoints: never lose work to a usage limit

A usage limit, a restart or a lost container can cut a run off mid-task. Without a record, the next run redoes the
work; with one, it loses at most one step. These rules come from the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §2b,
rules 1–5, and §2c). CLAUDE.md "Checkpoints" points here.

## 1. Who keeps one, and where (brief 2b rule 1)
- **Every task of more than one step** keeps a checkpoint on its own branch: `.checkpoints/<task-slug>.md`. A task is a
  change-set, a research topic, a night-shift item, or any workflow run on the task's own branch. A review of someone
  else's pull request keeps notes (§4) but never commits or pushes them to the branch under review. The slug is the
  branch's slug (`cs/kit-v3-0` → `kit-v3-0`); a cloud session on a `claude/…` branch uses the task's name.
- **Subagents keep notes** in `.checkpoints/<task-slug>/<agent>-<part>.md`, one file per agent and part (for example
  `.checkpoints/r3-rendering/graphics-engineer-outlines.md`).
- The repository is public: no secrets, tokens, credentials or raw network payloads in a checkpoint or a note. Write
  example credentials without the password (`scheme://user@host`); the saver leaves out any note that looks like a
  secret, and says so in its log.

## 2. The checkpoint file (brief 2b rule 1)
Copy this template. Keep it short: it is a handoff, not a log.
```
# Checkpoint: <task-slug>
- **Goal:** <one or two lines> · plan or verdict: <link or path>
- **Branch:** <branch> · **Base:** <ref and SHA>
- **Last update:** <YYYY-MM-DD HH:MM> Amsterdam

## Steps done
1. <step> · <commit SHA> · <result in a few words>

## Current step
<what is in hand right now>

## Next steps
1. <the next step, concrete enough to start without re-reading the plan>

## Open findings
- <finding · source · status (open, fixed in <SHA>, deferred: why)>

## Key numbers
- <metric: value (method, commit)>

## Decisions taken
- <decision · who took it · when>

## Running work
- <workflow run id · script path (the copy as launched) · arguments or the file holding them · notes folder ·
  launched at · status (running, done, failed: reason and reset time)>
```

## 3. Keeping it current (brief 2b rule 1)
- **Update it after every step, and at least every 30 minutes** while a step runs.
- **Commit and push it each time**, so a cut-off loses at most one step. Commit the step's work first, then record
  that commit's SHA in the checkpoint and commit the checkpoint.
- One command commits and pushes the checkpoint folder alone, leaving anything else you staged as it is:
  `node .claude/scripts/checkpoint.mjs save <task-slug> -m "<step>" --trailer "<attribution line>"` (one `--trailer` per
  attribution line the session's instructions give).
- Each step ends with a three-part summary for Mustafa (brief §6): what's done, what's checkpointed (where), what's
  next.

## 4. Subagents write as they go (brief 2b rule 2)
- **Never only in context.** An agent saves each finished unit to its notes file before it moves on. Example: a
  research agent writes each source's summary (URL, what it says, how sure) as soon as it has read the source.
- **Agents write; they never commit or push.** One committer at a time avoids git lock races: the session commits the
  notes with its next checkpoint, and during long runs the background saver does (§5).
- **Worktree agents** (`isolation: worktree`) write their notes to the main checkout's `.checkpoints/` folder, by the
  absolute path their prompt gives, so the saver sees them.
- **Agents without a write tool:** the advisor writes its notes with Bash, under `.checkpoints/` only. The built-in
  Explore and Plan agents can't write: give them single-unit work only, and they return their result whole.
- **The format:** the first line names the task. After each finished unit, a line or two: what is done, what was
  found and where (file:line, URL, command), what comes next. `DONE` on the last line when the work is finished.
- **Look first.** An agent started on a task reads its notes file before anything else:
  - the first line names another task: move the file into `old/` beside it and start fresh;
  - the notes end with `DONE`: give the answer from the notes without redoing the work;
  - otherwise: go on from where the notes stop, without redoing the units they mark done.
- **Paste this block into every agent prompt** (fill in the path):
  ```
  ## Saving your progress
  A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
  <path>/<agent>-<part>.md (make the folder; you may always write there). Look there first: if the first line names
  another task, move the file into <path>/old/ and start fresh; if the notes end with DONE, write your answer from
  them; otherwise go on from where they stop. Start a new file with one line naming your task. After each unit you
  finish (a source read, a file reviewed, a test group run), add a line or two: what is done, what you found and
  where, what comes next. End with DONE, then give your answer. Never commit or push.
  ```

## 5. The background saver (brief 2b rules 1 and 2)
During a workflow or any agent run longer than a few minutes, start the saver in the background (Bash
`run_in_background`, with the Bash `timeout` at its maximum, 7200000 ms; the default would stop it after 30 minutes), so
the agents' notes reach the remote even if the whole session is cut off:
`node .claude/scripts/checkpoint.mjs watch <task-slug> --trailer "<attribution line>"`
- Every 10 minutes (`--every`) it commits and pushes `.checkpoints/` if anything changed. It stops after 110 minutes
  (`--for`, under the 2-hour limit of a background command), when the branch changes, or when it is stopped, after a
  last save. Start it again for a longer run.
- It refuses `main`, a detached head, any branch outside `cs/` and `claude/`, and a task whose `.checkpoints/<slug>.md`
  is gone (so the finished task's last commit is made by hand and runs CI); it never forces a push; a refused push is
  reported and left alone; notes that look like a secret (a token, a key, credentials in a URL) are not committed. A
  commit that pushes only checkpoint files carries `[skip ci]`, decided by the script alone. In the background it never
  publishes code: with unpushed commits outside `.checkpoints/` on the branch it commits locally and skips the push.
- **Before a PR is marked ready:** stop the saver and make the last checkpoint commit. After that, nothing pushes to the
  PR's branch except a fix that goes back to review. Reviewer agents and the PR Advisor routine never push their notes
  to a PR's branch: that would change the head the advisor approved.

## 6. Resumable units (brief 2b rule 3)
- **Split long jobs into small units:** one source, one file, one test group, one area of the code; one agent per unit
  or per few units, never one agent for everything.
- **Make every step idempotent:** re-running a finished step does nothing. Check the notes (or the output file) before
  doing a unit; write each unit's output whole, under its own name, never appended a second time.

## 7. Workflows (brief 2b rules 2–4, §2c "no duplicate work")
- **Count what came back.** After every workflow, compare the items sent with the items returned, stage by stage. An
  item with no result (`null`) is not done: run it again on its own. A missing verdict never means "confirmed", and a
  missing reviewer never means "no findings".
- **A usage limit:** write the run id, the failed jobs and the reset time the error gives into the checkpoint
  ("Running work"). Launch nothing new. After the reset, resume with `resumeFromRunId`, the same script and the same
  arguments: finished agents come back from the cache, and cut-off agents go on from their notes.
- **A script edited while its run is unfinished:** keep the script as it was launched
  (`.checkpoints/<task-slug>/<script>-run<k>.js`) and resume from that copy, so the finished agents stay cached.
- **Never re-run an agent that finished.** Its result is in the workflow journal and in its notes.

## 8. Resuming (brief 2b rule 4)
1. Read the checkpoint on the task's branch. It is the authority on where the work stands; the approved plan or verdict
   is the authority on what may be done. A status comment or a message only points to it.
2. Check the branch matches it: `git log --oneline -5` shows the last SHA it records (or a later checkpoint commit).
   For code, run the gate.
3. Continue from "Next steps". Never restart a task from scratch when a checkpoint exists.
4. If the checkpoint is missing or doesn't match the branch, stop that task and write it under "Decisions needed";
   a night run then moves to its next item.

## 9. Finishing (brief 2b rule 5)
1. Fold the checkpoint's essentials into the report (`/advisor-report`): the steps with their SHAs, the key numbers,
   the decisions and the open findings.
2. Delete `.checkpoints/<task-slug>.md` and `.checkpoints/<task-slug>/` in the last commit before the PR is marked ready
   for review. That commit runs CI (no `[skip ci]`). Deleting them is not deleting data: the history keeps them.
3. A ready PR has no `.checkpoints/` files in its head (`git ls-tree -r --name-only HEAD -- .checkpoints` prints
   nothing; untracked notes don't count): the session checks it before marking the PR ready, and the advisor on review.
4. A review fix that takes more than one step turns the PR back into a draft, starts a new checkpoint, and deletes it
   again in its last commit before the PR is marked ready once more.
