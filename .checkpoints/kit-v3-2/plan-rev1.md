# Plan: kit-v3-2 (A2 on Mustafa's list of 2026-11-03), revision 1

- **Change-set:** kit-v3-2 · branch `cs/kit-v3-2` from main 8bcc65e (the worktree's local branch is named `kit-v3-2`
  today: create `cs/kit-v3-2` before the first checkpoint save, because `checkpoint.mjs` refuses any branch outside
  `cs/` and `claude/`, `checkpoint.mjs:24`; a cloud session limited to a `claude/…` branch uses that and says so).
- **Kind:** a rules (kit) change-set: it needs Mustafa's go before merge. No code, no symbols.
- **Input, his words** (queue/mustafa-2026-11-03-list.md, item A.2): "kit-v3-2, into .claude/CHECKPOINTS.md for every
  session: at most 3 agents at a time; every agent writes its plan into its notes before writing, then saves as it
  goes (each finished block to disk at once, a notes line after each finding, a restart continues after the last
  complete block); scratch files live in the notes folder, text and small scripts only, never build output or
  binaries (builds go to a temp folder); the background saver is checked and restarted at every checkpoint update.
  Also TOY-BOX: reword CLAUDE.md's "Red Alert 3 toy-box look" and the art-director's matching lines to point at
  LOOK_V2."
- **Earlier decision it carries out:** TOY-BOX (Mustafa, 2026-11-03, STATUS:199-200): "reword both to point at
  LOOK_V2, in a small kit change-set after PR #30 is updated, together with the checkpoint anti-waste rules."

## 1. Goal
1. Put Mustafa's four checkpoint rules into `.claude/CHECKPOINTS.md`, where every session (day sessions, the night
   shift, the PR Advisor) and every agent prompt already points, and keep every file that restates those rules in step
   with it (CLAUDE.md "Checkpoints", "Agents", "Ultracode and workflows"; the agent files that restate where scratch
   goes; the `changeset` skill; NIGHT_SHIFT.md's saver pointer).
2. TOY-BOX: CLAUDE.md:4's "Red Alert 3 toy-box look" and art-director.md:10's "a joyful, readable, toy-box
   battlefield" point at `docs/LOOK_V2.md` instead, without saying anything about the look itself.

Why (the player-facing reason is indirect): parallel agents spend one usage limit, and a limit cuts off every agent
running at that moment; work held only in context, or saved only at the end, is lost with it. The rules make a cut-off
cost at most one block per agent. TOY-BOX: LOOK_V2 §3 says `[DECIDED]` "The audience is adults. It must not look like a
children's game." (LOOK_V2.md:27), and CLAUDE.md, which every session reads first, still says "toy-box look".

## 2. Out of scope
- **A3, the guard's follow-ups** (KIT-FOLLOWUPS (a), STATUS:167-170 and 502-512): ask-first for kit and gate edits in
  day sessions, the URL-escaped `sk-` gap in `checkpoint.mjs`, read-only GitHub Actions tools for pr-advisor (the
  "New" item in a2-items.md now belongs to A3). Also no `git rev-list` change.
- **A4, the review policy** (a4-spec-2026-11-04.md): risk tiers, the review budget, rollback, per-tier checklists.
  A2 changes nothing in advisor.md, protocol steps 5-6 or the merge paragraph. The agent cap (decision AGENT-CAP below)
  touches how many reviewers run at once, not which reviewers or how many rounds: that stays A4's.
- **No tool and no code:** no change to `checkpoint.mjs`, the guard, `settings.json` or `.gitignore`. Nothing enforces
  the scratch rule mechanically in A2 (a size or binary check in the saver would be a code change: a possible later
  item, noted under Risks).
- **The rest of KIT-ALIGN** (STATUS:353-361: team sizes, the security baseline, preview privacy, server-only spread):
  untouched. Only the "look is not decided" lines are offered, as decision LOOK-LINES, because they sit beside the
  TOY-BOX lines and would contradict the new pointer.
- **Docs the session does not edit:** `docs/V2_PHASE0_BRIEF.md:113` ("One Opus research agent per topic … running in
  parallel": the research is done, but the line would read against the cap) is the Advisor's document: listed for
  the Advisor, not changed. `docs/GAME_DESIGN.md:14,17,28,50-51` (toy-box and joyful, the legacy design doc) and
  `README.md:4` ("Red Alert 3's toy-box look") are outside Mustafa's two named files: listed as a follow-up for
  docs-keeper and Mustafa (README is the public face), not changed here.
- No game file, no page, nothing a player sees. Nothing for Brave.

## 3. Decisions needed before the build (Mustafa's; the advisor may recommend)
The plan below is written for the recommended option of each. Where an option changes the text, the alternative text
is given next to the edit (marked "if AGENT-CAP B", and so on), so his answer needs no new plan.

**AGENT-CAP: what does "at most 3 agents at a time" count?**
- Why it matters: CLAUDE.md "Ultracode and workflows" (CLAUDE.md:217-220) asks for workflows on wide work, and a
  workflow runs many agents at once; today's review rounds ran 5 reviewers at once, and the changeset skill says
  "Call the reviewers … in parallel" (`.claude/skills/changeset/SKILL.md:25`). The answer decides whether those slow down.
- Options:
  - **A. Every running agent, per session or routine:** Agent-tool subagents, background agents and each agent a
    workflow has started (the session itself doesn't count). A 5-reviewer round runs as 3, then 2; a workflow script
    starts at most 3 `agent()` calls at once (plain script logic, for example groups of 3; no new tool). Cost: wide
    work takes longer by the clock. Gain: it matches his words "for every session" and the rule's purpose: a usage
    limit cuts off at most 3 agents' work, and 3 agents spend the limit more slowly than 10.
  - **B. Only Agent-tool subagents:** workflows keep their own width. Keeps ultracode as it is, but the widest burns
    of the limit (workflows) stay uncapped, which undercuts the purpose.
  - **C. Only agents that write beyond their notes** (deliverables, docs, worktree builds); read-only reviewers
    uncapped. After this change-set every agent writes at least its plan and notes, so "writes" has to be redefined
    as "beyond notes", and reviewers (the bulk of today's 5-at-once rounds) stay uncapped.
- Recommendation: **A**, with "per session" written out (two sessions each run their own 3; the night shift and the PR
  Advisor are sessions). It costs wall time, not quality; wide work still runs as workflows, just 3 wide.

**SCRATCH-HOME: where do scratch files and builds go?**
- Why it matters: today the rules say the opposite of his words: CLAUDE.md:213 "scratch under `.gate/`",
  verifier.md:10 "Scratch output goes only under `.gate/`", verifier.md:25 "write under .gate/ a minimal harness",
  perf-auditor.md:11 "scratch harnesses go under .gate/", and `settings.json:49-50` lets `node .gate/*.mjs|*.js` run
  without a prompt. `.gate/` is gitignored (`.gitignore:2`), so a cut-off container loses it; the notes folder is pushed
  by the saver, so it survives, which is the reason his rule puts scratch there.
- Options:
  - **A. Literal:** scratch text and small scripts in the notes folder `.checkpoints/<task-slug>/`; builds, binaries
    and generated output under `.gate/<task-slug>/` ("the temp folder"), or where a tool already puts its own output
    (the gate's `mkdtemp` folder, `tools/gate.mjs:105`; `/render-ab`'s `.gate/base` and `.gate/renders/<branch>/`,
    `.claude/skills/render-ab/SKILL.md:8,14`). The lines above change to match. Cost: in a day session `node
    .checkpoints/<slug>/x.mjs` is not on the allow list, so it asks first (or goes to the classifier in auto mode);
    in a routine it is refused (untracked, `routine-guard.mjs:471`), so a script there is a record only.
  - **B. Split by use:** text scratch in the notes folder; scripts that get run stay in `.gate/` (allow-listed). Fewer
    prompts, but a harness is lost with the container, against his words "scratch files live in the notes folder".
  - **C. The scratchpad folder** as the temp folder: session-specific, outside the project, writable by routines
    (`routine-guard.mjs:297`); but its path differs per session, agents' prompts would need it, and the tools and
    skills already write to `.gate/`.
- Recommendation: **A**, `.gate/<task-slug>/` as the temp folder (no new tool, no new folder kind, already
  gitignored, writable by both routines, `routine-guard.mjs:292`). An allow-list entry for `node .checkpoints/*` is a
  permissions change for a later kit change-set with security-auditor's review, not A2.

**SAVER-CHECK: when is the background saver checked and restarted?**
- Why it matters: §5 starts the saver only "during a workflow or any agent run longer than a few minutes"
  (CHECKPOINTS.md:88). "Restarted at every checkpoint update" could also mean keeping one running for the whole task.
- Options: **A.** at every checkpoint update while agents or a workflow run (with nothing running, the update's own
  `save` has already pushed everything); **B.** always, from the task's first checkpoint until the saver is stopped
  before the PR is marked ready.
- Recommendation: **A**. B adds a background command (2-hour limit, restarted every 110 minutes) to every
  multi-step task for no saved work.

**LOOK-LINES: reword only the two TOY-BOX lines, or also the four "look is not decided" lines?**
- Why it matters: CLAUDE.md:5-6 says "the v2 look is not decided (Mustafa and the Advisor write the look brief; never
  decide it)", right after line 4. Once line 4 points at LOOK_V2, line 5 says the brief is still being written. The
  same holds for CLAUDE.md:23-24, art-director.md:16 ("Until it exists") and graphics-engineer.md:19. STATUS:358-363
  lists them as "flagged for Mustafa, not changed" (KIT-ALIGN).
- Options: **A.** TOY-BOX's two lines only; the four stay for KIT-ALIGN, contradicting the new pointer until then.
  **B.** TOY-BOX plus the four lines, each reworded only to point at LOOK_V2 and to keep "never decide the look";
  the rest of KIT-ALIGN stays out.
- Recommendation: **B**. Both texts are below (edits T1-T4 for B; T1a and T2a for A). Neither decides anything about
  the look: each line names LOOK_V2 as Mustafa's and the Advisor's brief and keeps the agents off deciding it.

## 4. The edits, file by file
Every old text below is copied from main 8bcc65e. Line wraps follow the files' 120-column style; the Edit tool makes
each change (unique matches). Section numbers in CHECKPOINTS.md do not change: §3, §4, §5, §8 and §9 are cited by
number in CLAUDE.md:81, NIGHT_SHIFT.md:27,43, thirteen agent files, the advisor-report skill and docs (grep in §6, E7).

### 4.1 `.claude/CHECKPOINTS.md`
**C1, the opening paragraph (lines 3-5).** Old:
```
work; with one, it loses at most one step. These rules come from the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §2b,
rules 1–5, and §2c). CLAUDE.md "Checkpoints" points here.
```
New:
```
work; with one, it loses at most one step. These rules come from the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §2b,
rules 1–5, and §2c), and from Mustafa's anti-waste rules of 2026-11-03 (§4: the agent cap, plan first, saving each
block, scratch files; §5: the saver check). They hold for every session: day sessions, the night shift and the PR
Advisor. CLAUDE.md "Checkpoints" points here.
```

**C2, the template's "Running work" (lines 44-46).** Old:
```
## Running work
- <workflow run id · script path (the copy as launched) · arguments or the file holding them · notes folder ·
  launched at · status (running, done, failed: reason and reset time)>
```
New:
```
## Running work
- <workflow run id · script path (the copy as launched) · arguments or the file holding them · notes folder ·
  launched at · status (running, done, failed: reason and reset time)>
- <saver: background task id · its output file · started HH:MM · stops by HH:MM (110 minutes on) · last check
  HH:MM: ok, or what was wrong and what was done>
```

**C3, §3, a new last bullet after "Each step ends with a three-part summary…" (line 56-57).** Insert after
`  next.`:
```
- **At every update, check the background saver** when agents or a workflow are running, and restart it if it has
  stopped (§5).
```
(If SAVER-CHECK B: "**At every update, check the background saver** and restart it if it has stopped (§5).")

**C4, §4, a new first bullet, before "Never only in context" (line 60).** Insert:
```
- **At most 3 agents at a time** (Mustafa, 2026-11-03), per session: a day session, a night-shift run or a PR Advisor
  run. Every running agent counts: subagents started with the Agent tool, background agents, and each agent a
  workflow has started; the session itself does not. More work waits its turn: a review round of 5 reviewers runs as
  3, then 2; a workflow script starts at most 3 `agent()` calls at once (§7). Why: every running agent spends the same
  usage limit, and a limit cuts off all of them at that moment, so fewer at once means less to redo.
```
(If AGENT-CAP B: "Subagents started with the Agent tool count, background ones included; a workflow's agents do not
(a workflow is wide work, CLAUDE.md "Ultracode and workflows"), but the session starts no Agent-tool subagent while a
workflow runs." If AGENT-CAP C: "Agents that write anything beyond their notes count (deliverables, docs, builds in a
worktree), whether started by the Agent tool or a workflow; agents that only read and write notes do not.")

**C5, §4 "Never only in context" (lines 60-61).** Old:
```
- **Never only in context.** An agent saves each finished unit to its notes file before it moves on. Example: a
  research agent writes each source's summary (URL, what it says, how sure) as soon as it has read the source.
```
New:
```
- **Plan first, then save as you go** (Mustafa, 2026-11-03), never only in context. Before it writes anything else, an
  agent writes its plan into its notes: the blocks it will do, in order (a block is one finished piece of work: a
  source's summary, a reviewed file, a test group's result, a section of a deliverable). Then:
  - **each finished block goes to disk at once,** into its file (the notes, or the deliverable), never held for the end;
  - **a notes line after each finding,** as soon as it is found: what, and where (file:line, URL, command);
  - **a restart continues after the last complete block.** A block is complete when its notes line says so; a block
    without one is redone whole and written over the partial one, never appended (§6).
  Example: a research agent writes each source's summary (URL, what it says, how sure) as soon as it has read the
  source, then a notes line naming that source done.
```

**C6, §4 "Agents without a write tool" (lines 66-69).** Old:
```
  notes"; a known limit until A3, the guard's follow-ups). The built-in Explore and Plan agents can't write: give them
  single-unit work only, and they return their result whole.
```
New:
```
  notes"; a known limit until A3, the guard's follow-ups). The built-in Explore and Plan agents can't write: give them
  single-unit work only, and they return their result whole. Plan first holds for them too: the advisor writes its
  plan as the first lines of its notes (with Bash, in a day session) or as the first part of its report (in a
  routine, where the routine's own session saves it with the Write tool); for Explore and Plan, the caller's prompt is
  the plan, and the caller saves the result to the notes as soon as it comes back.
```

**C7, §4, a new bullet after "Agents without a write tool", before "The format".** Insert:
```
- **Scratch files** (Mustafa, 2026-11-03) live in the notes folder, `.checkpoints/<task-slug>/`: text and small
  scripts only (a list, a draft, a short harness; as a guide, under 100 KB a file). Never build output or binaries
  there (compiled code, Wasm, renders and screenshots, archives, whole-run logs, copies of the tree): the saver pushes
  the notes folder to the public repository, and a file stays in its history even after the folder is deleted.
  Builds and binaries go to the temp folder `.gate/<task-slug>/` (gitignored, never committed), or where a tool puts
  its own output (the gate's temporary folder; `/render-ab`'s `.gate/base` and `.gate/renders/`). In a routine a script
  in the notes folder is a record only: routines run only main's tracked scripts (`.claude/hooks/routine-guard.mjs`,
  `interpreter`).
```
(If SCRATCH-HOME B: add "A script that gets run may stay under `.gate/<task-slug>/`, where a day session runs it
without a prompt." If C: "the temp folder" becomes "the session's scratchpad folder, given by absolute path in each
agent's prompt".) The 100 KB guide is this plan's number, not Mustafa's: the advisor or he may change it.

**C8, §4 "The format" (lines 70-71).** Old:
```
- **The format:** the first line names the task. After each finished unit, a line or two: what is done, what was
  found and where (file:line, URL, command), what comes next. `DONE` on the last line when the work is finished.
```
New:
```
- **The format:** the first line names the task; the plan follows, one line per block. Then, as the work goes: a
  line for each finding (what, where), and after each finished block a line naming it done and what comes next.
  `DONE` on the last line when the work is finished.
```

**C9, §4 "Look first", third case (line 75).** Old:
```
  - otherwise: go on from where the notes stop, without redoing the units they mark done.
```
New:
```
  - otherwise: go on after the last block the notes mark done, following the plan at the top; redo a block they
    don't mark done, written over, never appended.
```

**C10, §4 the paste block (lines 77-85).** Old:
```
  ## Saving your progress
  A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
  <path>/<agent>-<part>.md (make the folder; you may always write there). Look there first: if the first line names
  another task, move the file into <path>/old/ and start fresh; if the notes end with DONE, write your answer from
  them; otherwise go on from where they stop. Start a new file with one line naming your task. After each unit you
  finish (a source read, a file reviewed, a test group run), add a line or two: what is done, what you found and
  where, what comes next. End with DONE, then give your answer. Never commit or push.
```
New:
```
  ## Saving your progress
  A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
  <path>/<agent>-<part>.md (make the folder; you may always write there). Look there first: if the first line names
  another task, move the file into <path>/old/ and start fresh; if the notes end with DONE, write your answer from
  them; otherwise go on after the last block they mark done. Start a new file with one line naming your task, then
  your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section
  written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what,
  where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written
  over, never appended. Scratch files go in <path>/: text and small scripts only; builds and binaries go under
  .gate/<task-slug>/. End with DONE, then give your answer. Never commit or push.
```

**C11, §5, a new bullet after "Every 10 minutes…" (after line 95).** Insert:
```
- **Check it at every checkpoint update, and restart it when it has stopped** (Mustafa, 2026-11-03), while agents or
  a workflow run:
  - running: the harness has not reported its background task as ended, and its first line (`watching .checkpoints/
    on <branch> every 10 min for 110 min`, with its time) is less than 110 minutes old;
  - working: the latest lines of its output (read the output file the Bash call named, with the Read tool: routines
    have no `cat`) say `saved and pushed` or `nothing to save`. A line with `NOT pushed`, `push refused`, `push
    failed`, `NOT committed` or `WARNING` is dealt with now (push with `save`, or fix the flagged note) and goes under
    "Open findings";
  - stopped with `time is up`, `interrupted` or `terminated`: start it again with the same command, on the task's
    branch. Stopped because the branch changed or the checkpoint is gone: find out why first; a finished task's saver
    stays stopped;
  - one saver per task: never start a second while one runs. Record it under "Running work".
  The PR Advisor runs no saver: it runs no scripts and never pushes notes (`routine-guard.mjs`, `interpreter`).
```
(If SAVER-CHECK B: the first line ends "… when it has stopped, from the task's first checkpoint until the saver is
stopped before the PR is marked ready:", and §5's opening "During a workflow or any agent run longer than a few
minutes, start the saver" becomes "From a task's first checkpoint, start the saver".)

**C12, §7 Workflows, a new first bullet (before "Count what came back", line 112).** Insert:
```
- **3 agents at a time** (§4): a workflow script starts at most 3 `agent()` calls at once (for example, its items in
  groups of 3), and the session starts no other agent while they run.
```
(If AGENT-CAP B: no C12. If C: "a workflow's agents that write beyond their notes count toward the cap of 3 (§4).")

### 4.2 `CLAUDE.md`
**M1, "Checkpoints" items 2-5 (lines 150-155): item 2 grows and splits in two; 3-5 become 4-6.** Old:
```
2. Subagents write as they go, to `.checkpoints/<task-slug>/<agent>-<part>.md`, never only in their context. They never
   commit; the session does, and during long runs the background saver (`checkpoint.mjs watch`) does too.
3. Long jobs are split into small, idempotent units (one source, one file, one test group).
4. Resuming: read the checkpoint, check the branch matches it, continue from "Next steps".
5. Finishing: fold the checkpoint into the report and delete `.checkpoints/<task-slug>*` in the last commit before the
   PR is marked ready.
```
New:
```
2. At most 3 agents at a time, per session, workflow agents included. Each writes its plan first into
   `.checkpoints/<task-slug>/<agent>-<part>.md`, then saves as it goes, never only in its context: each finished block
   to disk at once, a notes line after each finding; a restart continues after the last complete block.
3. Scratch files live in that notes folder: text and small scripts only. Builds and binaries go under `.gate/`
   (gitignored). Agents never commit; the session does, and during long runs the background saver
   (`checkpoint.mjs watch`) does too, checked and restarted at every checkpoint update.
4. Long jobs are split into small, idempotent units (one source, one file, one test group).
5. Resuming: read the checkpoint, check the branch matches it, continue from "Next steps".
6. Finishing: fold the checkpoint into the report and delete `.checkpoints/<task-slug>*` in the last commit before the
   PR is marked ready.
```
(No file cites these items by number: `grep -rn 'Checkpoints" [0-9]\|Checkpoints item'` finds none on main. If
AGENT-CAP B: "At most 3 Agent-tool subagents at a time, per session (workflows apart)". If C: "At most 3 agents that
write beyond their notes at a time, per session".)

**M2, "Agents", the write-scope sentence (lines 212-214).** Old:
```
- Agents never edit source, except build work the main session assigns them in an isolated worktree; they write only
  their notes under `.checkpoints/<task-slug>/`, scratch under `.gate/`, and the docs their own agent file names
```
New:
```
- Agents never edit source, except build work the main session assigns them in an isolated worktree; they write only
  their notes and scratch files (text and small scripts) under `.checkpoints/<task-slug>/`, builds and binaries under
  `.gate/`, and the docs their own agent file names
```
(If SCRATCH-HOME B: "…their notes and text scratch under `.checkpoints/<task-slug>/`, scripts they run and builds
under `.gate/`, …".)

**M3, "Ultracode and workflows" (lines 218-220).** Old:
```
verification sweeps, research. Workflows take no input mid-run, so each approval gate is its own run. Agents may read
and analyse in parallel; edits to the three engine files stay sequential. Try a small slice first.
```
New:
```
verification sweeps, research. Workflows take no input mid-run, so each approval gate is its own run. Agents may read
and analyse in parallel, at most 3 at a time, a workflow's agents included (`.claude/CHECKPOINTS.md` §4); edits to the
three engine files stay sequential. Try a small slice first.
```
(If AGENT-CAP B: "…in parallel (a workflow's agents are outside the cap of 3 Agent-tool subagents, §4); …". If C:
"…in parallel; at most 3 at a time that write beyond their notes (§4); …".)

**T1, line 4 and the "Look:" sentence (lines 4-6), TOY-BOX + LOOK-LINES B.** Old:
```
Vision: Foxhole camera · C&C Renegade base-assault economy · Red Alert 3 toy-box look and three factions · Gears gun feel.
Look: "Living Comic" is locked for the legacy build; the v2 look is not decided (Mustafa and the Advisor write the look
brief; never decide it). Today: PvE vs 6 bots. Next big phase: multiplayer on an authoritative server
```
New:
```
Vision: Foxhole camera · C&C Renegade base-assault economy · Red Alert 3's three factions · Gears gun feel.
Look: "Living Comic" is locked for the legacy build; the v2 look brief is `docs/LOOK_V2.md`, Mustafa's and the
Advisor's (never decide the look). Today: PvE vs 6 bots. Next big phase: multiplayer on an authoritative server
```
**T1a, if LOOK-LINES A** (line 4 only; line 5 left as it is): `Red Alert 3 toy-box look and three factions` becomes
`Red Alert 3's three factions · v2 look brief: docs/LOOK_V2.md` (the rest of line 4 unchanged).

**T2, "Design inputs" (lines 23-24), LOOK-LINES B only.** Old:
```
- **Design inputs:** `docs/LOOP_V2.md`, `docs/EDITOR_V2.md`, `docs/VISION_V2.md`. The look is not decided: research the
  technical options, never decide the look.
```
New:
```
- **Design inputs:** `docs/LOOP_V2.md`, `docs/EDITOR_V2.md`, `docs/VISION_V2.md`, and the look brief `docs/LOOK_V2.md`
  (Mustafa's and the Advisor's): research the technical options for it, never decide the look.
```

### 4.3 `.claude/agents/art-director.md`
**T3, line 10 (TOY-BOX).** Old:
```
You are the art director of SHAMBLE FRONTLINES: a joyful, readable, toy-box battlefield seen from an overhead camera.
```
New:
```
You are the art director of SHAMBLE FRONTLINES, a battlefield seen from an overhead camera. Its look: Living Comic for
the legacy build (below); for v2, the look brief docs/LOOK_V2.md, Mustafa's and the Advisor's.
```
(Under LOOK-LINES A as well: T3 is TOY-BOX itself. "Readable" moves out of this line only: readability stays the
first pillar in line 12, "docs/GAME_DESIGN.md §2–3 (pillars: readability first)".)

**T4, line 16-18 (the v2 paragraph), LOOK-LINES B only.** Old:
```
**v2:** the v2 look is not decided: Mustafa and the Advisor are writing the look brief. Until it exists, review the
rendering research (R3) only for what each technique can and can't render and how it reads from the gameplay camera;
never choose the look. Living Comic and the values below stay the reference for the legacy build.
```
New:
```
**v2:** the v2 look brief is docs/LOOK_V2.md, written by Mustafa and the Advisor. Review v2 rendering work
against it: what each technique can and can't render, and how it reads from the gameplay camera; its `[OPEN]` items
stay open, and you never choose the look. Living Comic and the values below stay the reference for the legacy build.
```
Not changed: line 27's "the upbeat tone" sits in the legacy procedure (Living Comic, ART_DIRECTION), which LOOK-LINES
does not touch; the art-director review (§7) is asked whether it reads as a v2 rule and needs a later pointer.

### 4.4 `.claude/agents/graphics-engineer.md` (LOOK-LINES B only)
**T5, lines 19-21.** Old:
```
**The look is not decided.** Mustafa and the Advisor are writing the look brief. You find out what each technique can
and can't do, what it costs and how it fails; you never choose the look. Candidate techniques for a comic look: toon
```
New:
```
**The look brief is docs/LOOK_V2.md** (Mustafa's and the Advisor's). You find out what each technique can
and can't do toward it, what it costs and how it fails; you never choose the look. Candidate techniques for a comic look: toon
```
(The second line is rewrapped at 120 columns in the build; the words are as above.)

### 4.5 The agent files that restate where scratch goes (SCRATCH-HOME A)
- **S1, `.claude/agents/verifier.md:10`.** Old: `You never edit source files or docs. Scratch output goes only under
  \`.gate/\` (gitignored); your notes go only under` → New: `You never edit source files or docs. Builds and generated
  output go only under \`.gate/\` (gitignored); scratch text and small scripts go in your notes folder; your notes go
  only under` (rewrapped).
- **S2, `.claude/agents/verifier.md:25`.** Old: `- logic: run (or write under .gate/) a minimal harness on the real code
  and assert the behaviour.` → New: `- logic: run (or write in your notes folder) a minimal harness on the real code and
  assert the behaviour.`
- **S3, `.claude/agents/perf-auditor.md:11`.** Old: `(scratch harnesses go under .gate/; your notes only under` → New:
  `(small scratch harnesses go in your notes folder, captures and builds under .gate/; your notes only under`.
- **S4, `.claude/agents/rust-reviewer.md:13`.** Old: `.checkpoints/ (your notes) and .gate/ (scratch builds).` → New:
  `.checkpoints/ (your notes and scratch files) and .gate/ (builds).`
(If SCRATCH-HOME B: S2 and S3 stay as they are; S1 keeps "scripts you run" under `.gate/`.)

### 4.6 `.claude/skills/changeset/SKILL.md` (AGENT-CAP A or C)
**K1, line 25.** Old: `6. **Review.** Call the reviewers that match the change type, in parallel, once, on the final diff`
→ New: `6. **Review.** Call the reviewers that match the change type, in parallel (at most 3 at a time,
\`.claude/CHECKPOINTS.md\` §4), once, on the final diff` (rewrapped). If AGENT-CAP B: no K1.

### 4.7 `.claude/NIGHT_SHIFT.md`
**N1, line 43.** Old: `push the checkpoint after every step (CHECKPOINTS.md §3; the background saver during long agent runs, §5). Only`
→ New: `push the checkpoint after every step (CHECKPOINTS.md §3; during long agent runs the background saver, checked
at every update, §5; at most 3 agents at a time, §4). Only` (rewrapped). Line 2 already says "Follow … CHECKPOINTS.md
for every item", so N1 is a pointer for consistency, not a new rule.

### 4.8 Files read and left unchanged (consistent after the edits)
- `.claude/agents/advisor.md:146-151` "Your notes": the routine-returns-notes route stays; C6 adds plan-first for it in
  CHECKPOINTS.md, with the same route.
- The other ten agent files cite "§4, saved as you go" or "save each source's summary as soon as you have read it":
  both still true under C5. No edit (one pointer each would be churn; the paste block C10 carries the new rules into
  every agent prompt).
- `.claude/skills/advisor-report/SKILL.md:10` (§9), `.claude/skills/render-ab/SKILL.md:8,14` (`.gate/base`,
  `.gate/renders/`, which C7 names), `docs/research/README.md:50` (§4, still true): unchanged.
- `docs/REPO_MAP.md`, `docs/STATUS.md`, `docs/ROADMAP.md`: docs-keeper's, at the end (§7). REPO_MAP:24
  ("`.gate/` scratch") may become "`.gate/` builds and scratch output": docs-keeper's call.

## 5. Symbols, change type, risks
- **Symbols:** none. No game file, page, tool, hook, script or setting changes; the gate's symbol check must read
  +0 -0.
- **Change type:** rules (kit) and agent instructions; docs at the end by docs-keeper. Not visual, not gameplay, not
  netcode. Evidence is the greps and checks in §6.
- **Risks and how they are contained:**
  1. **Wide work slows down (AGENT-CAP A).** A 5-reviewer round takes two waves; a 12-item workflow takes four. Wall
     time, not quality. Nothing enforces the cap (no guard rule, no workflow setting is assumed): it is a written rule,
     checked by the advisor on review from the checkpoint's "Running work".
  2. **Scripts in the notes folder reach the public repository.** The saver pushes `.checkpoints/`; §1 already bans
     secrets there and the saver's secret scan catches common keys, but the `%XX`-escaped `sk-` gap is still open until
     A3 (STATUS:505-509). C7 keeps scripts small and text only; the security-auditor review (§7) checks the wording.
  3. **Prompts for scripts in the notes folder (SCRATCH-HOME A).** `node .checkpoints/<slug>/x.mjs` is not on the allow
     list (settings.json:49-50 allow `.gate/` scripts only): a day session asks first or the classifier decides. A2 adds
     no allow-list entry (out of scope); noted as a possible later kit item with security-auditor.
  4. **Binaries can still land in the notes folder** (no mechanical check): once pushed, they stay in history. C7 says
     why; the session's `save` and the advisor's review see `git diff --stat` of checkpoint commits. A size or type
     check in `checkpoint.mjs` would be code: a possible later item, not A2.
  5. **The saver check needs its output file.** After a long run or a compaction the path can be lost: C2 records the
     task id and output file under "Running work". If the file can't be read, the safe move is the one §5 allows:
     stop the task with TaskStop (its last save runs) and start a new saver.
  6. **Two savers for one task** (a restart while one still runs): `save` updates the branch only if it has not moved
     (`checkpoint.mjs`, "moved while saving"), so the clash fails safe, but it is noise. C11 says one per task.
  7. **TOY-BOX wording read as deciding the look.** T1-T5 only name LOOK_V2 as Mustafa's and the Advisor's brief and
     keep "never decide the look"; art-director reviews them against LOOK_V2 (§7). Under LOOK-LINES A, CLAUDE.md:5
     keeps saying the brief is being written, which contradicts line 4 until KIT-ALIGN.
  8. **Renumbering CLAUDE.md's "Checkpoints" list (M1).** No file cites those items by number (E7 re-checks); the
     CHECKPOINTS.md section numbers, which are cited, do not change.
  9. **The rules are protected files.** The routine guard refuses edits to `.claude/` and `CLAUDE.md`
     (`routine-guard.mjs:26-37`): A2 is built in a day session only, never queued for the night.

## 6. Evidence (run by the session, then re-run by the verifier on the final diff)
- **E1, every rule landed:** `grep -n` in `.claude/CHECKPOINTS.md` finds each once: "At most 3 agents at a time",
  "Plan first, then save as you go", "each finished block goes to disk at once", "a notes line after each finding",
  "a restart continues after the last complete block", "Scratch files", "`.gate/<task-slug>/`", "Check it at every
  checkpoint update", "3 agents at a time** (§4)" (C12), "saver: background task id" (C2), and the new paste block's
  "then\n  your plan" and "Scratch files go in <path>/".
- **E2, the old texts are gone:** `grep -rn` over `CLAUDE.md .claude/` finds 0 hits for "toy-box", "joyful", "go on
  from where the notes stop", "go on from where they stop", "scratch under \`.gate/\`", "Scratch output goes only
  under", "write under .gate/"; and, under LOOK-LINES B, 0 hits for "look is not decided" and "are writing the look
  brief". Expected remaining "toy" hits elsewhere: README.md:4 and docs/GAME_DESIGN.md only (listed follow-ups).
- **E3, consistency across the files that restate the rules:** a table, one row per rule, listing every hit of
  `grep -rn -i "at a time\|in parallel\|scratch\|\.gate/\|saver\|plan first\|block" CLAUDE.md .claude/ docs/START_HERE.md
  docs/research/README.md tools/README.md` with "agrees / restates / contradicts". No "contradicts" row may remain
  (CLAUDE.md:62-63 and 207-208, "parallel implementation … merged one at a time", are about worktrees and agree).
  `tools/README.md:4` and `docs/START_HERE.md:112` ("scratch output goes under `.gate/`") are tool output, which C7
  puts under `.gate/`: they agree; docs-keeper may sharpen them.
- **E4, CLAUDE.md's short "Checkpoints" list matches CHECKPOINTS.md:** a two-way table: each of items 1-6 to the
  CHECKPOINTS.md section that holds it, and each new CHECKPOINTS.md rule (C3, C4, C5, C7, C11, C12) to the item that
  summarises it. Every cell filled.
- **E5, the repo map:** `node .claude/scripts/repo-map-check.mjs` exits 0 (no file added or removed).
- **E6, the gate:** `node tools/gate.mjs` PASS against main 8bcc65e, symbols +0 -0, smoke green; plus
  `node --test .claude/scripts/checkpoint.test.mjs .claude/hooks/routine-guard.test.mjs .claude/hooks/settings.test.mjs`
  green (untouched; proves nothing slipped into them), and `git diff --stat main` lists exactly CHECKPOINTS.md,
  CLAUDE.md, NIGHT_SHIFT.md, the skill and the agent files of §4 (art-director, verifier, perf-auditor, rust-reviewer,
  and graphics-engineer under LOOK-LINES B).
- **E7, section numbers and citations:** `grep -c '^## [0-9]' .claude/CHECKPOINTS.md` is 9, and the list of
  "CHECKPOINTS.md §N" citations over the repo is the same before and after (28 hits on main); `grep -rn
  'Checkpoints" [0-9]\|Checkpoints item'` still finds none.
- **E8, line width:** no changed line over 120 columns (`awk 'length > 120'` on the changed files, compared with main,
  where some long lines already exist).
- **E9, the new paste block works (a dry run):** one general agent (opus, high) gets a 3-block task (read three named
  files, note one fact from each) with the C10 block; its notes file must show the task line, the plan, a finding line
  and a done line per block, and DONE. A second start with the same notes must answer from them without re-reading.
  Its notes go to `.checkpoints/kit-v3-2/dryrun-<part>.md`, deleted with the checkpoint.

## 7. Reviewers (model and effort per CLAUDE.md "Models and effort"; at most 3 at a time, as the new rule asks)
- **advisor** (opus, high; from main's `.claude/agents/advisor.md`): this plan now; then the final diff, after wave 1's
  fixes. APPROVE, REVISE or ESCALATE.
- **Wave 1 on the final diff, three at once:**
  - **art-director** (opus, high): T1-T5 against LOOK_V2 §1-§9: does any line state or narrow the look? Is line 27's
    "upbeat tone" a v2 rule in disguise?
  - **security-auditor** (opus, high), narrow: C7 and C10 (what reaches the public repository: scripts and notes,
    the 100 KB guide, binaries), C11 (the saver check: no new stop route beyond TaskStop, nothing that widens a
    routine's permissions). Called because the change sets what is pushed to a public repository (CLAUDE.md step 5,
    "storage"), and A4's proposed tier 2 puts rules changes with security-auditor.
  - **verifier** (opus, high): E1-E9, PASS or FAIL with proof.
- **docs-keeper** (opus, high), last, after the report: STATUS (the queue, TOY-BOX done, KIT-ALIGN's list without the
  look lines under LOOK-LINES B, the follow-ups below), ROADMAP, REPO_MAP (line 24 optional).
- Not called: engine-architect, netcode-architect, perf-auditor, graphics-engineer, rust-reviewer, test-engineer,
  infra-engineer, game-designer: no code, no engine, no CI, no rules of play. (graphics-engineer.md's T5 is a pointer;
  art-director covers its wording.)

## 8. Build order (after the advisor's APPROVE and Mustafa's answers)
1. `cs/kit-v3-2` from main 8bcc65e; checkpoint `.checkpoints/kit-v3-2.md`; save.
2. CHECKPOINTS.md C1-C12 · gate · save.
3. CLAUDE.md M1-M3, T1-T2 · gate · save.
4. art-director T3-T4, graphics-engineer T5, verifier S1-S2, perf-auditor S3, rust-reviewer S4 · gate · save.
5. changeset skill K1, NIGHT_SHIFT N1 · gate · save.
6. E1-E9 · save. 7. Wave 1 reviews, fixes, the advisor on the final diff. 8. `/advisor-report`, the checkpoint folded
   in and deleted in the last commit, PR marked ready, `needs-mustafa` (a rules change: his go). 9. docs-keeper.

## 9. What Mustafa checks in Brave
Nothing: no player-facing change.

## 10. Follow-ups noticed (not in A2)
- `README.md:4` ("Red Alert 3's toy-box look") and `docs/GAME_DESIGN.md:14,17,28,50-51` (toy-box, joyful: the legacy
  design doc) still say toy-box. README is the public face: a docs-pass item for Mustafa to place.
- `docs/V2_PHASE0_BRIEF.md:113` ("running in parallel") reads against the cap: for the Advisor (DOCS-ALIGN).
- A size or binary check for `.checkpoints/` in `checkpoint.mjs`, and an allow-list entry for `node .checkpoints/*`:
  possible later kit items, with security-auditor.

## Decisions needed
- **AGENT-CAP:** what "at most 3 agents at a time" counts. Why: workflows and 5-reviewer rounds run wider today.
  Options: A every running agent per session, workflow agents included · B Agent-tool subagents only · C only agents
  that write beyond their notes. Recommendation: A.
- **SCRATCH-HOME:** where scratch and builds go. Why: today's rules put scratch in `.gate/`, which a lost container
  loses. Options: A scratch text and small scripts in the notes folder, builds and binaries in `.gate/<task-slug>/` ·
  B scripts that run stay in `.gate/` · C the session's scratchpad as the temp folder. Recommendation: A.
- **SAVER-CHECK:** when the saver is checked and restarted. Why: "at every checkpoint update" could mean a saver for
  every task. Options: A while agents or a workflow run · B always, from the first checkpoint. Recommendation: A.
- **LOOK-LINES:** reword only the two TOY-BOX lines, or also the four "look is not decided" lines beside them. Why:
  otherwise CLAUDE.md:5 says the brief is still being written, right after line 4 points at it. Options: A TOY-BOX
  only · B TOY-BOX plus CLAUDE.md:5-6, CLAUDE.md:23-24, art-director.md:16-18, graphics-engineer.md:19-20, as pointers
  only. Recommendation: B.

## For Mustafa
1. Answer AGENT-CAP, SCRATCH-HOME, SAVER-CHECK and LOOK-LINES (or say "recommendations").
2. After the advisor approves the final diff: your go to merge (a rules change).
3. Brave: nothing.
