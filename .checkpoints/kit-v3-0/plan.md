# Plan: kit-v3-0 (v2 phase 0, step 1: process upgrades)

Brief: `V2_PHASE0_BRIEF.md` §2 (from Mustafa and the Advisor; attached in the session, proposed for `docs/` below).
Branch: `claude/bold-faraday-i49gug` (this cloud session may push only there; CLAUDE.md's stand-in rule for `cs/kit-v3-0`).
Base: main 48ffb51. Change type: kit / rules + docs, plus one small kit script (logic). No game code: symbols +0 −0.
PR class: rules (needs-mustafa). Mustafa gave his go in this session's chat before the PR existed ("Once kit-v3-0 PR is
up, you can merge it yourself"); the session merges only after the advisor's APPROVE of the final head with `gate` and
`smoke` green, passing that SHA as the expected head.

## Goal
Make the crew ready for the v2 rebuild before any research starts: Opus everywhere with stated effort, work that
survives a usage limit (checkpoints), less waste without less quality, a night shift on the new window, and six new
agents for the new stack. Player-facing effect: none.

## Out of scope
- Any engine or game code; the legacy game is frozen (brief §0, §5).
- The routines (brief §3): set up after this PR merges, then reported.
- Research R1–R6 (start after the merge); R7 has its own plan (it may start now, brief §7.3).
- Path-scoped rules for v2 code (`.claude/rules/netcode.md` paths, `security-patterns.json`): phase 1 defines the
  folders, so those files change with the repository restructure.
- STATUS and ROADMAP: open PR #25 (docs-after-kit-v2-2) edits both files. The docs pass after this merge records
  kit-v3-0 (the kit-v2-2 precedent, its report "Decisions and deviations").

## What changes, file by file
### Commit 1 (done locally, docs only): `docs/VISION_V2.md`
Byte-identical copy of the attachment (cmp equal; SHA-256 22664025…3ab).

### Commit 2: `docs/V2_PHASE0_BRIEF.md` (proposed addition, docs only)
The brief itself, byte-identical. Why: R1–R6 start in later sessions (and possibly the night shift), and the PR
Advisor reviews their PRs; both need the topic questions, the ADR list and the "Don't" list. Without it in the repo
every session must be re-briefed. The advisor decides: keep (recommended) or drop.

### Commit 3: kit-v3-0
1. **`CLAUDE.md`** (sections added or edited; everything else byte-identical):
   - Top: one paragraph "v2 rebuild": the decision (Rust sim core to Wasm + native, TypeScript client on the latest
     three.js WebGPURenderer with WebGL 2 fallback, our own netcode over WebTransport with a WebSocket fallback);
     legacy frozen (critical fixes only, moves to `legacy/` in phase 1); phase 0 = process, research, decision drafts,
     no engine code; the look is not decided (never decide it); design inputs LOOP_V2, EDITOR_V2, VISION_V2, the brief.
   - "Read before any non-trivial task": `docs/REPO_MAP.md` becomes the first stop ("read the map, don't explore"),
     plus VISION_V2 and the brief in the deeper-docs list.
   - "Hard constraints" heading gains "(the legacy game)"; the constraint lines themselves are unchanged. "Decisions
     pending" line: the phase 0 drafts `docs/adr/0001`–`0006` (brief §4) take over ADR-001..003.
   - New section **"Models and effort"** (brief 2a): Opus 5.5 for everything that thinks, every subagent included
     (agent files pin `model: claude-opus-5-5`; built-in agents such as Explore, Plan and general-purpose get
     `model: "opus"` passed explicitly, because Explore can default to a smaller model; workflow agents inherit the
     session's Opus); effort `high` by default, `xhigh` for architecture decisions (ADRs, ENGINE_OVERVIEW, the
     synthesis) and netcode, always stated explicitly (the Opus 5.5 default effort is medium); Sonnet 5.5 only for
     purely mechanical chores when genuinely necessary (file moves, formatting, link checks, log summaries), Opus
     still preferred; never a weaker model to save tokens.
   - New section **"Checkpoints"** (brief 2b): five one-line rules and a pointer to `.claude/CHECKPOINTS.md`.
   - New section **"Efficiency: cut waste, never quality"** (brief 2c): read the map; focused subagents (tight brief,
     details to files, short structured summary back); review once on the final diff; stop on blockers (write the
     decision down, move to other approved work); no duplicate work (a checkpointed task is resumed); explicitly not
     allowed: skipping verification, shrinking evidence, a weaker model.
   - Protocol step 4: a "research" evidence line (cited sources, the verifier's spot-check of key claims). Step 5:
     the new reviewers by area. Step 6: the checkpoint is folded into the report and deleted before "ready".
   - "Agents": the crew v2 list with one line each, and the research owner per topic (R1 engine-architect, R2
     netcode-architect, R3 graphics-engineer, R4 engine-architect, R5 test-engineer with infra-engineer on CI, R6
     infra-engineer with security-auditor on the security baseline, R7 the main session; the verifier checks claims).
   - "Never": restart a task that has a checkpoint; a weaker model to save tokens.
   - "Commands": the checkpoint script (below) and its test.
2. **`.claude/CHECKPOINTS.md`** (new): the system, adapted from Unit Forge's research runbook ("Progress notes",
   "State file", "Usage limits") to the brief's five rules:
   - when (every task of more than one step), where (`.checkpoints/<task-slug>.md` on the task's branch), the template
     (goal; branch and base; steps done with SHAs; current step; next steps; open findings; key numbers; decisions;
     running work: workflow run ids, script paths, arguments, agent notes folders; last update in Amsterdam time);
   - update after every step and at least every 30 minutes, commit and push each time;
   - subagent notes `.checkpoints/<task-slug>/<agent>-<part>.md`: first line names the task, a line or two per
     finished unit (what, evidence, next), `DONE` at the end; look first (another task's notes move to `old/`; DONE
     means answer from the notes); agents write, never commit (no git races); worktree agents write to the main
     checkout's path given in their prompt; a ready-to-paste "Saving your progress" block for prompts;
   - resumable units: one source, one file, one test group; idempotent steps (check the notes first, outputs written
     whole per unit, never appended twice);
   - workflows: count what came back (null = not done, rerun alone); on a usage limit record the run id and the reset
     time, launch nothing new, resume after the reset with `resumeFromRunId`, the same script (the launched copy if it
     changed) and arguments; never re-run a finished agent;
   - resuming (read, check branch and last SHA, run the gate on code, continue from "next steps"; never restart);
     finishing (fold into the report, delete `.checkpoints/<slug>*` in the last commit before ready);
   - safety: the repository is public, so no secrets, tokens or raw payloads in checkpoints; the checkpoint on the
     branch is the authority, a comment only points to it.
3. **`.claude/scripts/checkpoint.mjs`** (new, Node 22, no dependencies, about 120 lines): pushes the notes of agents
   that never commit, so a cut-off loses minutes, not a workflow.
   - `save <slug> -m <subject> [--trailer <line>]…`: stage `.checkpoints/` only, commit only that path (`--only`:
     anything else the session staged stays staged), push the current branch.
   - `watch <slug> [--every <min>=10] [--for <min>=110] [--trailer <line>]…`: run in the background during a workflow;
     every N minutes, if `.checkpoints/` changed, `save`; stops after `--for` or when the branch changes.
   - Guards: only branches matching `^(cs|claude)/[A-Za-z0-9._/-]+$`; never `main`, never a detached head; never
     force; a refused push is reported and left alone; retries a push 4 times (2, 4, 8, 16 s); a busy index lock is
     retried later; commit subjects carry `[skip ci]` so autosaves don't run CI (the final commit, which deletes the
     checkpoint, runs CI).
4. **`.claude/scripts/checkpoint.test.mjs`** (new, `node --test`): a scratch git repo with a bare remote proves: only
   `.checkpoints/` is committed; other staged work stays staged; the push lands on the branch; `main`, a detached head
   and a non-`cs`/`claude` branch are refused; a rejected push is never forced; `watch` stops when the branch changes.
5. **`.claude/settings.json`**: allow `Bash(node .claude/scripts/checkpoint.mjs *)`, `Bash(node --test .claude/scripts/*)`,
   `WebSearch`, `WebFetch` (research agents and the night shift's research items need them without prompts). The
   deny list is unchanged. Security review weighs the web tools.
6. **`.claude/NIGHT_SHIFT.md`** (brief 2d), same structure and rule numbers where possible:
   - one routine starting at least hourly from 00:10 to 07:40 Amsterdam; window 00:10–08:30 (stop silently outside);
     from 07:45 start or resume no item; report done by 08:20; stop by 08:30;
   - status comment: "… · Checkpoint: <branch>:.checkpoints/<slug>.md · …"; busy (< 45 min) stops silently; stale
     running status (> 45 min) resumes from that checkpoint per CHECKPOINTS.md; only status comments by the
     `Mergincanli` account count (fixes STATUS's open risk "rules 3-4 trust any comment on the public issue");
   - focused agents per item type: research → the matching domain agent, then the verifier checks the claims; build →
     the main session builds, then the matching reviewers; docs → docs-keeper;
   - kept: the lock, silent stops, one report comment per night updated after every finished item, the absolute
     rules, the reporting format, cut-off pick-up from an earlier night.
7. **Six new agents** (`.claude/agents/`, same style as the existing ones; all `model: claude-opus-5-5`, `memory:
   project`; effort high, engine-architect xhigh). Tools: Read, Grep, Glob, Bash, Write, WebSearch, WebFetch; each
   writes only to its named folders and `.checkpoints/`; never source code (the main session builds):
   - `engine-architect`: module boundaries, the Rust ↔ TypeScript boundary, data flow, dependency rules; R1, R4, the
     synthesis (`docs/ENGINE_OVERVIEW.md`, ADR drafts 0001–0006 with spike pass marks). Writes docs/research/,
     docs/adr/, docs/ENGINE_OVERVIEW.md.
   - `graphics-engineer`: WebGPU and TSL, the render pipeline, outline pass, LOD and instancing, the GPU budget (VISION
     §4 targets), Web Audio for R3; researches techniques, never decides the look. Writes docs/research/.
   - `rust-reviewer`: Rust safety, speed, identical results on Wasm and native, Wasm size; reviews and researches
     (crate choices, determinism options). Writes .checkpoints/ only.
   - `test-engineer`: unit and property tests, golden tests against legacy behaviour, network and load tests; R5 and
     gate v2. Writes docs/research/.
   - `infra-engineer`: CI, preview links per PR, hosting, servers, monitoring, costs; R6. Writes docs/research/.
   - `game-designer`: rules against LOOP_V2 and VISION_V2, balance math, tuning tables; used from the loop slice on.
     Writes docs/design/.
8. **Existing agents updated for the new stack** (changes only, the rest kept):
   - `verifier`: sonnet → Opus, medium → high; adds research claim spot-checks (open the cited source, verdict
     confirmed / wrong / unsupported, with the quote's location) and WebSearch, WebFetch; checks that a ready PR has
     no `.checkpoints/` left. Gate v2 arrives in phase 1.
   - `docs-keeper`: sonnet → Opus, medium → high; keeps `docs/REPO_MAP.md` current; v2 docs (VISION_V2, research
     index, ADR drafts) in STATUS.
   - `netcode-architect`: the v2 stack (Rust server sharing the Rust sim; WebTransport with WebSocket fallback; tick
     rate decided by R1/R2; Web Worker client sim); R2 and the netcode ADR draft; adds docs/research/ to its writes and
     WebSearch, WebFetch. Legacy code is reference, not the base.
   - `perf-auditor`: the v2 budgets from VISION §4 (integrated-graphics laptop ≥ 30 fps Low in the largest mode;
     mid-range GPU 60 fps High; the spectacle budget §6.9; server tick and bandwidth from the ADRs; Wasm size) and v2
     methods (WebGPU timestamp queries, browser profiles, cargo bench); legacy traps kept for the legacy build.
   - `security-auditor`: the v2 threat surface (Rust server: panics as denial of service, `unsafe`, deserialisation
     limits; WebTransport and WebSocket: origin, certificates, datagram size, rate limits; CSP with
     `wasm-unsafe-eval`, no `unsafe-eval`; cargo audit / cargo deny / npm audit; preview links); R6 security baseline.
   - `art-director`: the v2 look is not decided; until the look brief exists it reviews R3's technique research only
     for what each option can and can't render, never choosing; Living Comic stays the reference for the legacy build.
   - `advisor`: v2 sources of truth (VISION_V2, the brief, docs/research/, ENGINE_OVERVIEW, ADR drafts); phase 0 rules
     (no engine code, legacy frozen except critical fixes, the look is Mustafa's); plan review also checks the model
     policy and a checkpoint for multi-step work; PR review also checks cited sources and the verifier's spot-check on
     research PRs, and that no `.checkpoints/` is left on a ready PR. Escalation categories, merge rules and "Never"
     unchanged.
9. **Skills**: `changeset` gains "start the checkpoint" (step 1) and the model and effort rule for its agents;
   `advisor-report` gains "fold the checkpoint in; confirm it is deleted".
10. **`.claude/claude-security-guidance.md`**: the context line says the planned server is a Rust server over
    WebTransport with a WebSocket fallback (was Node over WebSockets). Rules unchanged.
11. **`docs/REPO_MAP.md`** (new, one screen): every top-level path, every doc with its purpose and when to read it,
    `.claude/` contents, tools, CI. Generated from the real tree, then checked by a script (every path in the map
    exists; every tracked top-level path, doc and agent is in the map).
12. **`docs/research/README.md`** (new, short): the shape every `docs/research/<topic>.md` takes (findings with
    sources, options with trade-offs, recommendation, open questions, the verifier's spot-check table).
13. **`docs/reports/2026-10-24-kit-v3-0.md`**: the advisor report (with the checkpoint folded in).

## Symbols
No game code: gate symbols +0 −0. No `shamble-*.js`, page or gate file is touched.

## Risks and how they're contained
1. The new rules contradict old ones (merge rules, night-shift rules, read-only reviewers). Contained: a trace table
   from every brief requirement to its file and line, plus a contradiction sweep over CLAUDE.md, advisor.md,
   NIGHT_SHIFT.md and the skills (workflow below).
2. `checkpoint.mjs` pushes from a script, outside the Bash permission rules. Contained: hard guards (branch pattern,
   never main, never force, `.checkpoints/` only), the test file, and a security-auditor review.
3. Web tools in `settings.json` widen what runs without a prompt (prompt injection from fetched pages). Contained:
   pages are data (CLAUDE.md "Never"), agents have no secrets to leak, the security-auditor weighs it.
4. `[skip ci]` autosaves leave a head without checks. Contained: the final commit (checkpoint deleted) is never
   skip-ci, and main's protection requires `gate` and `smoke` on the PR head anyway.
5. Pinning `claude-opus-5-5` instead of the `opus` alias: a newer Opus needs a policy edit. That is the brief's intent
   (Opus 5.5); noted in CLAUDE.md.
6. The night-shift window and the routine change together; the routine is set up only after the merge, so main and
   the routine never disagree for long. Tonight's clock change (25 October, 03:00 → 02:00) is covered by
   `CRON_TZ=Europe/Amsterdam` plus the busy lock.
7. Reviewer agents gaining Write (art-director, perf-auditor, security-auditor, verifier, rust-reviewer): scoped by
   instruction to `.checkpoints/` (and `.gate/` scratch), so partial results survive a cut-off (brief 2b rule 2); they
   still never edit source or docs. The advisor stays without Write: its verdicts are short and returned whole.

## Evidence I'll produce
- Gate PASS, checks 1–4 and 5a–5d, symbols +0 −0.
- `node --test .claude/scripts/checkpoint.test.mjs`: all assertions pass, plus a mutation (guard removed → a test
  fails) to prove it bites.
- A brief trace table: every requirement of brief §2a–2e and §7 → file:line in the diff.
- A contradiction sweep (workflow: independent agents read the final files and try to find a rule that conflicts with
  another, or a brief requirement that is missing; findings verified before fixing).
- REPO_MAP check script output; every `docs/` link in new files resolves.
- `cmp` for the two copied docs.
- Frontmatter check: every agent has model claude-opus-5-5 and an effort line; names match file names.

## Reviewers
- advisor (a general agent reading main's `.claude/agents/advisor.md`, since this branch changes that file): this
  plan, then the PR's final head.
- security-auditor: the final diff (script, settings, web tools, night-shift trust rule, advisor.md).
- verifier: gate, trace table, claims in the report.
- Not called: art-director, perf-auditor, netcode-architect (no visuals, hot paths or protocol change; their own files
  change only in text the security and advisor reviews read).

## What Mustafa checks in Brave
Nothing (no player-facing change). His go for this rules PR is already in this chat; the session still waits for the
advisor's APPROVE of the final head and green `gate` and `smoke` before merging.

## Order of work
1. Advisor review of this plan (and of the R7 plan, separately).
2. Commit 1–2 (docs), then kit-v3-0 in small commits, checkpoint pushed after each.
3. Test, gate, trace table, contradiction sweep; fix.
4. Security review; fix or defer in writing.
5. Report; delete the checkpoint; open the PR (draft until the report is in, then ready); advisor PR review; merge.
6. Then: routines (brief §3), R7 continues on the same branch after the merge.
