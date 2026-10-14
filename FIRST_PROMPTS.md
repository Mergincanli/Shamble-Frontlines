# First prompts for Claude Code

Paste these into Claude Code in the repo folder, in order. Each starts with a plan and waits for your approval.
Order: 0 (environment) → 1 (scale test) and 2 (build spike), in either order → 3 (architecture decisions).

---
## Prompt 0: Phase 0, environment and a provable baseline (normal effort, no ultracode)

```
/changeset Phase 0: agentic environment and a green, provable baseline

Read CLAUDE.md, docs/STATUS.md and docs/START_HERE.md first. The Advisor assembled this repo from the latest game
files, the docs and the toolkit, and wrote the .claude/ kit. Nothing has run on this Windows machine yet, and
tools/gate.mjs does not exist yet: you build it. No game behaviour changes in this change-set.

Plan these, then wait for my approval:
1. Toolkit portability: preview.js, battery.js, sim.js, aabbs.js and pageshot.js must run from this repo on Windows
   (no /home/claude paths; resolve dependencies relative to tools/). Add tools/package.json pinning three@0.128.0, gl,
   @napi-rs/canvas, pngjs, acorn@8, acorn-walk@8 and playwright, plus a short tools/README.md. Find out whether
   headless-gl (preview.js) runs natively here; if it fights you beyond a short attempt, stop and report a WSL
   fallback instead of forcing it.
2. pageshot.js: add a `headed` mode that renders with the real GPU and prints the WebGL renderer string, so every
   screenshot says whether it came from the RTX 5080 or from SwiftShader.
3. tools/gate.mjs: one command for checks 1–4 against a base git ref (default: merge-base with main): per-file
   node --check including each page's inline script; load-order concatenation for both pages; symbol conservation via
   battery.js (print +added / −dropped and cross-file duplicates); check4.flagged must be empty. Non-zero exit on failure.
4. Prove the gate bites (have the verifier agent check your work critically): PASS on the clean tree; then on a
   throwaway branch, each of these must FAIL on the right check: (a) a deleted function, (b) a new unguarded call from
   shamble-game.js into an editor-only function, (c) a syntax error, (d) a game function defined a second time in
   shamble-core.js. Delete the branch afterwards.
5. Prove the syntax hook: introduce a syntax error in shamble-game.js with the Edit tool, show the hook's report, revert.
6. CI: .github/workflows/gate.yml running `node tools/gate.mjs` on push and pull request (Node 22, read-only
   permissions, full git history so pull requests compare against their base branch; the gate only needs acorn, so
   skip optional packages and browsers). List anything I must click on GitHub.
7. Docs for the new workflow: rewrite docs/START_HERE.md §1 and §5 (git branches, the Edit tool, gate.mjs,
   tools/README.md; remove uploads, /mnt/user-data/outputs, present_files, Python heredocs and the dash shell). Keep
   the section numbers. In CLAUDE.md, drop "created in Phase 0" from the gate line once it works.
8. The multiplayer target changed: update CLAUDE.md, docs/STATUS.md and docs/DESIGN_NOTES.md to three teams (one per
   faction) of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly larger); design for 48+ players,
   bots fill empty slots.
9. Baseline evidence under .gate/baseline/: gate PASS; pageshot menu and pause renders in headed GPU mode at 1600x900;
   a preview.js menu render (if preview.js runs here); sim.js ×3. Record the numbers in docs/STATUS.md.
Finish with /advisor-report.
```

---
## Prompt 1: scale test, can the client handle 12v12v12 to 16v16v16 fights? (normal effort)
Run after Phase 0. Measurement only: nothing that changes gameplay gets merged.

```
/changeset Scale test: can the client render and simulate 12v12v12 to 16v16v16 fights?

Measurement only; no gameplay change is merged. Keep the harness in tools/ (or behind DEV) for later reuse.
Read CLAUDE.md, docs/STATUS.md and docs/AUDIT_BUGS_PERF.md first. Use the perf-auditor agent for method and review.
Starting fact (measured by the Advisor): one soldier is ~66 visible meshes + ~55 ink hulls, so ~120 draw calls before
the shadow pass; the busiest scene today is ~200 draw calls in total.

Plan, then wait for my approval:
1. A harness that spawns N soldiers (mixed classes and factions) in two layouts: "brawl" (as many as fit inside the
   gameplay camera's view, fighting) and "spread" (across the map). N = 7 (today), 16, 24, 36, 48.
2. For each N and layout measure: frame time with pageshot in headed GPU mode, and again with CDP CPU throttling ×4 as
   a weaker-laptop proxy; update() time split into AI / animation / FX / other; draw calls and triangles for the main
   and shadow passes; allocation MB/s.
3. Report a table, where it breaks, and the three highest-value fixes with estimated gains (for example merging each
   soldier's rigid parts per bone, distance LOD, cheaper ink for far soldiers, animation LOD, AI time-slicing).
   Do not implement them.
Finish with /advisor-report. This feeds ADR-001 (server and bandwidth budgets), ADR-003 and the character rework.
```

---
## Prompt 2: build spike, evidence for the ADR-002 discussion (throwaway branch, never merged)

```
/changeset Spike (throwaway, never merged): what would ES modules + a bundler + TypeScript cost us?

Evidence for ADR-002 only. Work on a throwaway branch; no gameplay changes; do not merge.
1. Scan shamble-core.js, shamble-game.js and shamble-editor.js for what breaks as ES modules: implicit globals,
   sloppy-mode behaviour (modules are strict), inline HTML handlers (onclick="…") that need globals, load-order
   assumptions, and the cross-file symbol graph from tools/battery.js. Estimate the conversion size per file.
2. Convert ONE self-contained slice to a TypeScript module (proposal: the fenced SIM block in core), shim it back to
   the globals the rest of the code expects, and produce a double-clickable dist/index.html with Vite or esbuild,
   with three.js r128 bundled from npm so it also works offline. Prove: double-click works with the network off; the
   dev server works; /render-ab shows identical pixels against main; the gate and sim.js ×3 still pass on the source.
3. Report in plain language: what Mustafa's day-to-day workflow becomes, what broke, the effort estimate for the full
   migration, the risks (including saves: localStorage under file:// and under http://localhost are different
   origins, so rank, maps and prefabs would need export/import), and your recommendation.
Finish with /advisor-report.
```

---
## Prompt 3: the architecture decisions (ultracode, its own session)
Run after Prompts 1 and 2. Use `/model fable` (check the picker for a "Requires usage credits" label) with ultracode.

```
ultracode: Draft ADR-001 (multiplayer architecture) and ADR-002 (code structure & build) for SHAMBLE FRONTLINES.
Do not modify source code. Write only docs/adr/0001-multiplayer-architecture.md and
docs/adr/0002-code-structure-and-build.md in the format of docs/adr/0000-template.md, Status: Proposed.

Inputs: the scale-test and build-spike reports in docs/reports/, plus CLAUDE.md, docs/START_HERE.md, MODULE_MAP.md,
TECHNICAL.md §4 (SIM core), GAME_DESIGN.md, AUDIT_BUGS_PERF.md, ROADMAP.md and the code.

Author: the netcode-architect agent. Before finalizing, have independent agents attack each draft from these angles
and revise: (a) netcode correctness and game feel at 50–150 ms latency with 2 % packet loss; (b) security and
anti-cheat (security-auditor); (c) migration risk and effort from the current code: three global-scope files, the
fenced SIM block, FIXED_DT, makeInputCommand, playerFireOrigin, ANIM v2's cosmetic rule, game state living inside
THREE objects; (d) performance at full scale: client frame time with many soldiers in view, server tick budget with
48+ players plus bots, bandwidth per client; (e) keeping double-click single-player play against bots; (f) the effect
on agent-driven development (type safety, testability, size of reviews).

Targets from Mustafa: three teams, one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16,
possibly larger): design for 48+ players. Bots fill empty slots; team count and size are data, not code. Cosmetic
skins come later: keep ownership server-side and never let a skin change team colours. Milestones: 2–8 players
first, then 16, then full scale, with bot-driven load tests at each step. Offline play against bots stays. EU hosting,
student budget, Windows development machine.

Starting recommendation to test, not to rubber-stamp: an authoritative Node server; one fixed-tick simulation
extracted from the current code and shared by client and server; binary WebSocket transport behind an interface with
interest management; ES modules + a bundler + TypeScript for shared and server code while still producing a
double-clickable build; the Three.js upgrade deferred to ADR-003.

Mustafa asked to talk ADR-002 through: open it with a plain-language summary (what changes in his day, what stays the
same, what it costs, and the no-build alternative with its costs). Each ADR must contain the options with trade-offs,
the strongest objection raised against the chosen option, a phased migration plan with verifiable milestones (the game
stays playable at every step) and a test plan with a network simulator. Finish with /advisor-report: the
recommendation, the strongest objections, and open questions for Mustafa.
```

---
## Queue: small change-sets worth doing early (one at a time, each with /changeset)
1. Editor: hoist `refreshPrefabList` out of `bootEditor()` so opening the editor stops throwing.
2. Main menu: skip the 3D render while the opaque menu covers the screen (measure GPU/CPU before and after).
3. First-deploy tutorial → the clipboard's field-manual mode (one controls list; removes the dead V / B / 1–5 keys).
4. UI transplant step 2: service record + end of round. Then step 3: armory + scoreboard (dark field-kit family).

## Template for every future task
```
/changeset <one sentence: what, and for whom>
Why: <player-facing reason, or the bug with its evidence>
Scope: in: … / out: …
Constraints: <anything beyond CLAUDE.md>
Done when: <observable acceptance criteria>
Evidence: <what proves it> · Brave: <what I will screenshot>
```
