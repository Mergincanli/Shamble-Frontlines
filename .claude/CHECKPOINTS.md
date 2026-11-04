# Checkpoints: never lose work to a usage limit

A usage limit, a restart or a lost container can cut a run off mid-task. Without a record, the next run redoes the
work; with one, it loses at most one step. These rules come from the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §2b,
rules 1–5, and §2c), and from Mustafa's anti-waste rules of 2026-11-03 (§3 and §5: the saver check; §4: the agent
cap, plan first, saving each block, scratch files; §7: the cap in workflows). They hold for every session: day
sessions, the night shift and the PR Advisor. CLAUDE.md "Checkpoints" points here.

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
- <saver: background task id · its output file · started HH:MM · stops by HH:MM (110 minutes on) · last check
  HH:MM: ok, or what was wrong and what was done>
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
- **At every update, check the background saver** when agents or a workflow are running, and restart it if it has
  stopped (§5).

## 4. Subagents write as they go (brief 2b rule 2)
- **At most 3 agents at a time** (Mustafa, 2026-11-03), per session: a day session, a night-shift run or a PR Advisor
  run. Every running agent counts: subagents started with the Agent tool, background agents, and each agent a
  workflow has started; the session itself does not. More work waits its turn: a review round of 5 reviewers runs as
  3, then 2; a workflow script starts at most 3 `agent()` calls at once (§7). Why: every running agent spends the same
  usage limit, and a limit cuts off all of them at that moment, so fewer at once means less to redo.
- **Plan first, then save as you go** (Mustafa, 2026-11-03), never only in context. Before it writes anything else, an
  agent writes its plan into its notes: the blocks it will do, in order (a block is one unit of §6: a source's
  summary, a reviewed file, a test group's result, a section of a deliverable). Then:
  - **each finished block goes to disk at once,** into its file (the notes, or the deliverable), never held for the end;
  - **a notes line after each finding,** as soon as it is found: what, and where (file:line, URL, command);
  - **a restart continues after the last complete block.** A block is complete when its notes line says so. A restart
    trusts complete blocks and never rereads, rechecks or redoes them: that would spend the saved work again (a later
    block may still read a file it needs). A block without a done line is redone whole and written over the partial
    one, never appended (§6).
  Example: a research agent writes each source's summary (URL, what it says, how sure) as soon as it has read the
  source, then a notes line naming that source done.
- **Agents write; they never commit or push.** One committer at a time avoids git lock races: the session commits the
  notes with its next checkpoint, and during long runs the background saver does (§5).
- **Worktree agents** (`isolation: worktree`) write their notes to the main checkout's `.checkpoints/` folder, by the
  absolute path their prompt gives, so the saver sees them.
- **Agents without a write tool:** the advisor writes its notes with Bash, under `.checkpoints/` only, in a day session;
  in a routine the guard refuses shell file tools, so the advisor returns its notes in its report (advisor.md, "Your
  notes"; a known limit until A3, the guard's follow-ups). The built-in Explore and Plan agents can't write: give them
  single-unit work only, and they return their result whole. Plan first holds for them too: the advisor writes its
  plan as the first lines of its notes (with Bash, in a day session) or as the first part of its report (in a
  routine, where the routine's own session saves it with the Write tool); for Explore and Plan, the caller's prompt is
  the plan, and the caller saves the result to the notes as soon as it comes back.
- **Scratch files** (Mustafa, 2026-11-03) live in the notes folder, `.checkpoints/<task-slug>/`: text and small
  scripts only (a list, a draft, a short harness; as a guide, under 100 KB a file). Never build output or binaries
  there (compiled code, Wasm, renders and screenshots, archives, whole-run logs, copies of the tree): the saver pushes
  the notes folder to the public repository (no secrets, §1), and a file stays in its history even after the folder
  is deleted.
  Builds and binaries go to the temp folder `.gate/<task-slug>/` (gitignored, never committed), or where a tool puts
  its own output (the gate's temporary folder; `/render-ab`'s `.gate/base` and `.gate/renders/`). In a routine a script
  in the notes folder is a record only: routines run only main's tracked scripts (`.claude/hooks/routine-guard.mjs`,
  `interpreter`).
- **The format:** the first line names the task; the plan follows, one line per block. Then, as the work goes: a
  line for each finding (what, where), and after each finished block a line naming it done and what comes next.
  `DONE` on the last line when the work is finished; the answer is then given from the notes alone.
- **Look first.** An agent started on a task reads its notes file before anything else:
  - the first line names another task: move the file into `old/` beside it and start fresh;
  - the notes end with `DONE`: give the answer from the notes without redoing the work;
  - otherwise: go on after the last block the notes mark done, following the plan at the top; redo a block they
    don't mark done, written over, never appended.
  In both of the last two cases, trust the blocks the notes mark done: never reread, recheck or redo them (a later
  block may still read a file it needs).
- **Paste this block into every agent prompt** (fill in <path> and <task-slug>):
  ```
  ## Saving your progress
  A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
  <path>/<agent>-<part>.md (make the folder; you may always write there). Look there first: if the first line names
  another task, move the file into <path>/old/ and start fresh; if the notes end with DONE, write your answer from
  them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck
  or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then
  your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section
  written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what,
  where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written
  over, never appended. Scratch files go in <path>/: text and small scripts only; builds and binaries go under
  .gate/<task-slug>/. This folder is pushed to a public repository: never put a secret, token, environment value,
  request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret
  check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer
  from your notes alone, opening no other file. Never commit or push.
  ```

## 5. The background saver (brief 2b rules 1 and 2)
During a workflow or any agent run longer than a few minutes, start the saver in the background (Bash
`run_in_background`, with the Bash `timeout` at its maximum, 7200000 ms; the default would stop it after 30 minutes), so
the agents' notes reach the remote even if the whole session is cut off:
`node .claude/scripts/checkpoint.mjs watch <task-slug> --trailer "<attribution line>"`
- Every 10 minutes (`--every`) it commits and pushes `.checkpoints/` if anything changed. It stops after 110 minutes
  (`--for`, under the 2-hour limit of a background command), when the branch changes, or when it is stopped, after a
  last save. Start it again for a longer run. To stop it, stop its background task with the TaskStop tool, so its last
  save runs. (A routine has no `kill`/`pkill`; the guard refuses them — stop the task the harness way.)
- **Check it at every checkpoint update, and restart it when it has stopped** (Mustafa, 2026-11-03), while agents or
  a workflow run:
  - running: the harness has not reported its background task as ended, and its first line (`watching .checkpoints/
    on <branch> every 10 min for 110 min`, with its time) is less than 110 minutes old;
  - working: the latest lines of its output (read the output file the Bash call named, with the Read tool: routines
    have no `cat`) say `saved and pushed` or `nothing to save`. Any other line is dealt with now and goes under "Open
    findings":
    - `NOT pushed` (unpushed commits outside `.checkpoints/`): push only once those commits are ready to publish (gate
      run, diff read for secrets: the saver scans notes only), then run `save`;
    - `push refused`: leave it alone (below); fetch and look at what moved on the remote; never force, rebase or reset;
    - `push failed`: run `save` once more;
    - `NOT committed`: fix or remove the flagged note; under "Open findings" name the file and the kind only, never the
      value;
    - `WARNING`: do what its line says (the index's `.checkpoints/` entries are behind HEAD);
  - stopped with `time is up`, `interrupted` or `terminated`: start it again with the same command, on the task's
    branch. Stopped because the branch changed or the checkpoint is gone: find out why first; a finished task's saver
    stays stopped;
  - one saver per task: never start a second while one runs. Record it under "Running work".
  The PR Advisor runs no saver: it runs no scripts and never pushes notes (`routine-guard.mjs`, `interpreter`).
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
- **3 agents at a time** (§4): a workflow script starts at most 3 `agent()` calls at once (for example, its items in
  groups of 3), and the session starts no other agent while they run.
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
