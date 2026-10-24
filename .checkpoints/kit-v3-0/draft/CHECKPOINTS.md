# Checkpoints: never lose work to a usage limit

A usage limit, a restart or a lost container can cut a run off mid-task. Without a record, the next run redoes the
work. With one, it loses at most one step. This system is adapted from Unit Forge's research runbook (its progress
notes, state file and usage-limit rules) to the v2 brief's five rules. CLAUDE.md points here.

## 1. Who keeps one, and where
- **Every task of more than one step** keeps a checkpoint on its own branch: `.checkpoints/<task-slug>.md`. A task is a
  change-set, a research topic, a night-shift item, a review of a large diff, or any workflow run. The slug is the
  branch's slug (`cs/kit-v3-0` → `kit-v3-0`); a cloud session on a `claude/…` branch uses the task's name.
- **Subagents keep notes** in `.checkpoints/<task-slug>/<agent>-<part>.md`, one file per agent and part (for example
  `.checkpoints/r3-rendering/graphics-engineer-outlines.md`).
- Checkpoints are committed to a public repository: no secrets, tokens, credentials or raw network payloads in them.

## 2. The checkpoint file
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
  launched at · status (running, done, failed: reason, reset time)>
```

## 3. Keeping it current
- **Update it after every step, and at least every 30 minutes** while a step runs.
- **Commit and push it each time**, so a cut-off loses at most one step. Commit the step's work first, then record
  that commit's SHA in the checkpoint and commit the checkpoint.
- One command does both for the checkpoint folder alone, without touching anything else you staged:
  `node .claude/scripts/checkpoint.mjs save <task-slug> -m "<step>" --trailer "<attribution line>"` (one `--trailer`
  per attribution line the session's instructions give).

## 4. Subagents write as they go
- **Never only in context.** An agent saves each finished unit to its notes file before it moves on. Example: a
  research agent writes each source's summary (URL, what it says, how sure) as soon as it has read the source.
- **Agents write; they never commit or push.** One committer at a time avoids git lock races. The main session
  commits the notes with its next checkpoint, and during long runs a background saver does it (below).
- **Worktree agents** (`isolation: worktree`) write their notes to the main checkout's `.checkpoints/` folder, by the
  absolute path their prompt gives, so the saver sees them.
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

## 5. The background saver
During a workflow or any agent run longer than a few minutes, start the saver in the background (Bash
`run_in_background`), so agents' notes reach the remote even if the whole session is cut off:
`node .claude/scripts/checkpoint.mjs watch <task-slug> --trailer "<attribution line>"`
- Every 10 minutes (`--every`) it commits and pushes `.checkpoints/` if anything changed; it stops after 110 minutes
  (`--for`, under the 2-hour limit of a background command), when the branch changes, or when it is stopped, after a
  last save. Start it again for a longer run.
- It refuses `main`, a detached head and any branch outside `cs/` and `claude/`; it never forces a push; a refused
  push is reported and left alone. A commit that pushes only checkpoint files carries `[skip ci]`.

## 6. Resumable units
- **Split long jobs into small units:** one source, one file, one test group, one area of the code. One agent per unit
  or per few units, never one agent for everything.
- **Make every step idempotent:** re-running a finished step does nothing. Check the notes (or the output file) before
  doing a unit; write each unit's output whole, under its own name, never appended a second time; name outputs after
  the unit, not the run.

## 7. Workflows
- **Count what came back.** After every workflow, compare the items sent with the items returned, stage by stage. An
  item with no result (`null`) is not done: run it again on its own. A missing verdict never means "confirmed", and a
  missing reviewer never means "no findings".
- **A usage limit:** write the run id, the failed jobs and the reset time the error gives into the checkpoint
  ("Running work"). Launch nothing new. After the reset, resume with `resumeFromRunId`, the same script and the same
  arguments; finished agents come back from the cache and cut-off agents go on from their notes.
- **A script edited while its run is unfinished:** keep the script as it was launched (`.checkpoints/<task-slug>/
  <script>-run<k>.js`) and resume from that copy, so the finished agents stay cached.
- **Never re-run an agent that finished.** Its result is in the workflow journal and its notes.

## 8. Resuming
1. Read the checkpoint on the task's branch. It is the authority; a status comment or a message only points to it.
2. Check the branch matches it: `git log --oneline -5` shows the last SHA it records (or a later checkpoint commit).
   For code, run the gate.
3. Continue from "Next steps". Never restart a task from scratch when a checkpoint exists.
4. If the state is unclear (a step half done, the SHA missing), redo only that step, and say so in the checkpoint and
   the report.

## 9. Finishing
1. Fold the checkpoint's essentials into the report (`/advisor-report`): the steps with their SHAs, the key numbers,
   the decisions and the open findings.
2. Delete `.checkpoints/<task-slug>.md` and `.checkpoints/<task-slug>/` in the last commit before the PR is marked
   ready for review. That commit runs CI (no `[skip ci]`).
3. A ready PR has no `.checkpoints/` files: the verifier and the advisor check it.
