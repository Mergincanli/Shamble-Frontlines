# SHAMBLE FRONTLINES v2, phase 0: prepare (brief for Claude Code)

From Mustafa and the Advisor. Read all of it before acting. Attached: `VISION_V2.md`.

## 0. Where we are
- **The decision:** SHAMBLE FRONTLINES is rebuilt as v2:
  - a **Rust simulation core**, compiled to WebAssembly for the browser and to native code for the server;
  - a **TypeScript client** on the latest three.js with the **WebGPU** renderer (WebGL 2 fallback);
  - **our own netcode**: server authority, WebTransport with a WebSocket fallback.
- **The legacy game is frozen.** Critical fixes only. It moves to `legacy/` in phase 1.
- **The design inputs:**
  - `docs/LOOP_V2.md` (the core loop) and `docs/EDITOR_V2.md` (plus its inventory);
  - the attached `VISION_V2.md`: the north star, modes and scale, targets, and what the engine must support.
- **The look is not decided yet.** Mustafa and the Advisor are writing the look brief. You research the technical
  options, but never decide the look.

## 1. The order of work
- **We build a game-specific engine.** Every engine feature exists because VISION_V2 §6 needs it. No generic-engine
  features "for later".
- **Phase 0 (this brief):** process upgrades, then research, then decision drafts. **No engine code** except throwaway
  spikes, and those only after the decisions are approved.
- **Phase 1:** the repository restructure, toolchain, CI gate v2, then the first playable slice.
- **Then:** playable slices, as in VISION_V2.

## 2. Step 1: process upgrades (one PR: kit-v3-0)
This PR changes the rules, so **Mustafa merges it himself.**

### 2a. Model policy (quality first)
- **Opus 5.5 for everything that thinks:** architecture, code, reviews, research, plans and reports. That includes
  every subagent. Switch `verifier` and `docs-keeper` from Sonnet to Opus.
- **Effort levels:** high by default; xhigh for architecture decisions and netcode.
- **Sonnet 5.5 only for purely mechanical chores, and only when it's genuinely necessary:** file moves, formatting,
  link checks, summarizing logs. Opus remains the preference even there.
- Write this policy into `CLAUDE.md`.

### 2b. A checkpoint system: never lose work to a usage limit
In another project, runs that hit the usage limit lost their in-progress work and had to redo it after the reset. A
checkpoint system fixed it. If you can access Mustafa's `unit-forge` repository, study how it did this and adapt it.
Otherwise, build it from these rules:
1. **Every task of more than one step keeps a checkpoint file** in its branch: `.checkpoints/<task-slug>.md`.
   - It records: the goal; the steps done, with commit SHAs; the current step; the next steps; open findings; key
     numbers; and decisions taken.
   - Update it after every step, and at least every 30 minutes.
   - **Commit and push it** each time, so a cut-off loses at most one step.
2. **Subagents write as they go.** They save partial results to `.checkpoints/<task-slug>/<agent>-<part>.md` before
   moving on, never only in their own context. Example: a research agent saves each source summary as soon as it's
   done.
3. **Resumable units:** split long jobs into small units (one source, one file, one test group). Make every step
   idempotent, so re-running a finished step does nothing.
4. **Resuming:** read the checkpoint, check the branch matches it, then continue from "next steps". Never restart a
   task from scratch when a checkpoint exists.
5. **Finishing:** fold the checkpoint's essentials into the report, and delete `.checkpoints/<task-slug>*` in the last
   commit before the PR is marked ready.
- Document the system in `.claude/CHECKPOINTS.md`, and point to it from `CLAUDE.md`.

### 2c. Efficiency rules: cut waste, never quality
- **Don't rediscover what's already known.** Keep a short, current map of the repository and the docs, so sessions and
  agents read the map instead of exploring from scratch.
- **Focused subagents:**
  - give each one a tight brief;
  - have it write details to files and return a short, structured summary;
  - keep the main context clean.
- **Review once, on the final diff:** run reviewer agents on a change-set's final diff, not on every small commit.
- **Stop on blockers:** when something needs a decision, write it down and move on to other approved work. Never burn
  tokens guessing.
- **No duplicate work:** a task that has a checkpoint is resumed, never started again.
- **Explicitly not allowed:** skipping verification, shrinking evidence, or using a weaker model to save tokens.

### 2d. Night shift v2 (update `.claude/NIGHT_SHIFT.md`)
- **The window is now 00:10–07:30, Europe/Amsterdam** (ends when Mustafa's day starts):
  - start no new item after 06:45;
  - have the report done by 07:20;
  - the 07:10 wake-up is the last one (finish only); stop by 07:30.
- **Handoff runs through checkpoints.** The status comment on the Night queue points to the active checkpoint file. A
  run that finds a stale "running" status (older than 45 minutes) resumes from that checkpoint.
- **Focused agents per item type:**
  - research items → the matching domain agent researches, and the verifier checks the claims;
  - build items → the main session builds, then the matching reviewers check;
  - docs items → docs-keeper.
- Keep everything else: the lock, silent stops, one report comment per night updated after every finished item, the
  absolute rules, and the reporting format.

### 2e. Crew v2: new agents, all on Opus
Add these, in the same style as the existing agents:

| Agent | Owns |
|---|---|
| `engine-architect` | Module boundaries, the Rust ↔ TypeScript boundary, data flow, dependency rules |
| `graphics-engineer` | WebGPU and shaders (TSL), the render pipeline, the outline pass, LOD and instancing, the GPU budget |
| `rust-reviewer` | Rust quality: safety, speed, identical results everywhere, Wasm size |
| `test-engineer` | Test strategy: unit and property tests, tests against legacy behaviour, network and load tests |
| `infra-engineer` | CI, preview links per PR, hosting, servers, monitoring |
| `game-designer` | Checks rules against LOOP_V2 and VISION_V2, balance math, tuning tables. Used from the loop slice onward |

- Update the existing agents for the new stack.
- They review and research; the main session builds. When work runs in parallel, agents build in isolated copies.

## 3. Step 2: the routines (you set them up yourself)
Once kit-v3-0 is merged:
1. **Replace the old night-shift routines with one "Night shift" routine:**
   - Schedule triggers **at least hourly from 00:10 to 07:10, Amsterdam time** (`10 0-7 * * *`). Use a cron schedule. Hourly starts
     plus the window check also cope with the clock changes.
   - Model Opus 5.5, effort high.
   - This repository attached; no connectors.
   - The prompt: *"You are the night shift for SHAMBLE FRONTLINES. Read .claude/NIGHT_SHIFT.md from the default branch
     and follow it exactly, together with CLAUDE.md."*
   - If you can't delete the old routines, list them for Mustafa to remove.
2. **The PR Advisor routine:** check that it runs on Opus 5.5, effort high. Keep its triggers.
3. **Report the final schedule:** each routine, its triggers in Amsterdam time, and its model.

## 4. Step 3: research, before any engine code
**Method:**
- One Opus research agent per topic (the matching domain agent), running in parallel, each with checkpoints.
- Cite sources. Prefer official docs, specifications, and talks or papers from shipped games.
- The verifier spot-checks every key claim against its source.
- Each topic ends with `docs/research/<topic>.md`: findings, options with trade-offs, a recommendation, and the open
  questions.

| # | Topic | Questions to answer |
|---|---|---|
| R1 | Simulation core | ECS or plain data (bevy_ecs, hecs, custom); a fixed tick rate; **identical results between Wasm and native** (floats, Rapier's determinism options); the Rust ↔ TypeScript boundary (shared memory, wasm-bindgen, generated TypeScript types); running the simulation in a Web Worker; a **data-driven ability and effect system** in the spirit of UE's Gameplay Ability System; data formats and hot reload of tuning |
| R2 | Netcode | Snapshots, client prediction, interpolation and lag compensation for 48+ players; tick rate and bandwidth budgets; interest management for a top-down camera; the WebTransport server stack in Rust plus the WebSocket fallback; rooms and private matches; learn from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3) |
| R3 | Rendering and presentation | three.js WebGPURenderer with TSL; the WebGL 2 fallback; candidate techniques for a comic look (toon ramps, screen-space outlines, ambient occlusion, emissive bloom); characters (skinning, instancing, LOD); budgets for the minimum and recommended hardware; how we measure it; spatial audio with Web Audio |
| R4 | Physics and navigation | Rapier 3D or custom; navigation meshes in Rust; bots moving between floors; line of sight across heights; obstacles placed mid-match (for later building) |
| R5 | Toolchain and quality | Repository layout (a Cargo workspace plus TypeScript with Vite); strict TypeScript and lint rules; clippy and rustfmt; tests (cargo test, property tests, Vitest, Playwright, golden tests against legacy); CI with Rust caching; **preview links per PR**; the gate v2 that replaces today's gate |
| R6 | Infrastructure | Hosting for previews and EU game servers; costs; logs, metrics and crash reports; balance telemetry (VISION_V2 §6.6); security baseline (content security policy, dependency audits, anti-cheat basics) |
| R7 | Legacy harvest | Write `docs/LEGACY_INVENTORY.md`. Every system of the current game: what it does, where it lives, the evidence and numbers we have, and a pre-marked recommendation (port to TypeScript / rewrite in Rust / redesign / drop) with a reason. Extract the tuning numbers into data tables. **Do this one first:** it's small, and the others use it |

**Synthesis:** `engine-architect` combines the findings into:
- `docs/ENGINE_OVERVIEW.md`: the engine in one page, with a diagram;
- **decision drafts** `docs/adr/0001`–`0006`: multiplayer and netcode; code structure, build and toolchain;
  rendering; the simulation core (ECS, determinism, physics, navigation); content data and the ability system;
  infrastructure and operations.

Each draft states:
- the context and the options;
- the proposed decision and its consequences;
- **what a spike must prove**, with measurable pass marks (for example: 48 simulated players at a 30 Hz server tick
  within X ms; the look test at 60 fps on the recommended hardware).

**Then stop.** The drafts are reviewed by Mustafa and the Advisor before any spike starts.

## 5. Don't
- Write engine code in phase 0 (spikes come after the decisions are approved).
- Change the legacy game, except for critical fixes.
- Decide the look.
- Merge rule changes or decision drafts without Mustafa.

## 6. Reporting
Use the reporting rule in `CLAUDE.md`: the TL;DR first; "Decisions needed" and "For Mustafa" last. At each step's end,
a short summary covering: what's done, what's checkpointed, and what's next.

## 7. Start
1. Add the attached `VISION_V2.md` as `docs/VISION_V2.md` (docs only).
2. Plan kit-v3-0 (step 1) and show the plan. Then build it and open the PR for Mustafa.
3. R7 (the legacy inventory) may start right away, on checkpoints. The other research topics start once kit-v3-0 is
   merged, so they use the new rules.
