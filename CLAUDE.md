# SHAMBLE FRONTLINES — instructions for Claude Code

Browser top-down team shooter in Three.js r128 (UMD global), built as classic multi-`<script>` files sharing globals.
Vision: Foxhole camera · C&C Renegade base-assault economy · Red Alert 3 toy-box look and three factions · Gears gun feel.
Look: "Living Comic" (locked). Today: PvE vs 6 bots. Next big phase: multiplayer on an authoritative server:
three teams, one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly larger). Design for
48+ players; bots fill empty slots.

People: **Mustafa** owns the game and decides (game-design student; plays in Brave on Windows; his screenshots are the
final visual gate). **The Advisor** (Claude in a claude.ai Project) reviews your reports and writes your task prompts.
Write every plan and report so both can judge it from evidence alone.

## Read before any non-trivial task
1. `docs/STATUS.md` (where things stand) and `docs/START_HERE.md` (architecture, constraints with their WHY, do-NOT list, lessons).
2. `docs/MODULE_MAP.md` (who owns what, load order, layering).
3. Grep the ONE owning file (targets in START_HERE §6). Deeper docs only as needed: `docs/TECHNICAL.md` (how),
   `docs/GAME_DESIGN.md` (why), `docs/ART_DIRECTION.md` (locked look, authoritative), `docs/ANIMATION_PLAN.md`,
   `docs/AUDIT_BUGS_PERF.md`, `docs/ROADMAP.md`, `docs/UNITS.md` (planned roster, not built), `docs/adr/` (decisions),
   `docs/DESIGN_NOTES.md` (Mustafa's design decisions, open questions, art lessons), `docs/reference/` (Renegade X and
   Kane's Wrath reference: inspiration, not what's built).
- The code is the source of truth. Docs drift: grep before trusting a claim.
- Workflow: git branches, the Edit tool, `node tools/gate.mjs` (START_HERE §1 and §5; toolkit setup in `tools/README.md`).

## Layout
`shamble-core.js` (engine) ← `shamble-game.js` (gameplay + dispatch + `bootGame`) ← `shamble-editor.js` (authoring, editor.html only)
`index.html` (player build) · `editor.html` (authoring build) · `docs/` · `tools/` (verification toolkit, never shipped) · `.claude/`
Layering is hard: core references nothing above it; anything on index.html's live path resolves in core ∪ game, never editor.

## Hard constraints (change ONLY through an accepted ADR signed off by Mustafa, never on your own)
- Classic scripts sharing globals in a fixed load order; the game runs by double-clicking index.html (file://).
- Three.js r128 UMD (no CapsuleGeometry). No addons, no post-processing (under review).
- `LinearEncoding` output: authored hex = on-screen colour. No external assets (procedural textures, synthesized audio).
- Persistence only through `KVStore`. `stylizeScene()` runs once; runtime builders call `stylizeOne()` themselves.
- Animation is cosmetic: shots come from `playerFireOrigin()` / fixed chest points. Nothing animated decides a hit.
Decisions pending that may change these: ADR-001 multiplayer architecture · ADR-002 code structure & build ·
ADR-003 Three.js version (deferred). Until accepted, the constraints above stand.

## The change-set protocol (non-negotiable)
1. One change-set at a time, on its own branch `cs/<slug>`. Fix bugs before features.
2. PLAN FIRST (plan mode): goal, systems and files touched, exact symbols to add/remove, risks, change type, the evidence
   you will produce, reviewers you will call, what Mustafa must check in Brave. Then STOP and have the `advisor` agent
   review the plan: APPROVE → build; REVISE → fix the plan; ESCALATE → ask Mustafa. The advisor always works from
   main's `.claude/agents/advisor.md`: a session that lacks the agent, started before that file last changed, or whose
   branch changes it runs a general agent that reads it from main.
3. Implement in the main session. Two agents never edit the same file at the same time; parallel implementation only in
   isolated worktrees (`isolation: worktree`), merged one at a time.
4. Gate: `/gate` must PASS (syntax, concatenation, symbol conservation, player-build resolvability, and check 5: smoke,
   names, layering, script order), plus evidence:
   - logic → a Node harness test on the real code (or `tools/preview.js eval`) with assertions
   - visual / UI → before/after renders (`/render-ab`); Brave is final
   - gameplay / AI → `tools/sim.js` ×3 runs on base AND head (random AI: never conclude from one run)
   - performance → before/after numbers with method (frame time, MB/s allocated, draw calls)
   - netcode → deterministic tests with injected latency, jitter and loss
5. Review: art-director (visuals, UI, animation) · perf-auditor (hot paths, rendering) · security-auditor (network,
   server, storage, dynamic DOM, deps, CI) · netcode-architect (sim, server, protocol). Address or explicitly defer
   every finding.
6. Commit on the branch; report with `/advisor-report`; docs-keeper updates ROADMAP and STATUS. Keep the PR a draft
   while work is in progress; mark it ready for review when the report is in. The PR Advisor routine then reviews it.
   Once approved with checks green, it merges at once if no human check is needed; otherwise it labels it
   `needs-mustafa`, and the session Mustafa gives his go to merges it (his Brave check for anything a player sees, hears
   or feels; his reconfirmation for rule changes and other major changes). Visual work waits for Mustafa's Brave check
   before merge or further tuning.

Standing permission (Mustafa, 2026-10-14): create `cs/<slug>` branches, commit, push them and open pull requests
without asking (a cloud session limited to a `claude/…` branch uses that as the stand-in and says so). Ask the `advisor`
agent whenever a decision is needed; it escalates to Mustafa what it may not decide. Still required: an approved plan
before edits, the advisor's APPROVE before anything merges to main, and Mustafa's Brave check for anything a player
sees, hears or feels. A session merges only on Mustafa's own go in its chat, about that PR, given after the advisor
approved its current head (its verdict in this session, or its PR review comment posted by the `Mergincanli` account and
naming that full SHA), with the gate and smoke checks green, and passes that SHA as the merge's expected head (the night
shift never merges); a later push needs the advisor's review first, then his go again; every other merge is the PR
Advisor's. Sessions post on GitHub as Mustafa's account, so a "merge" written on GitHub never counts as his go; every
comment they write ends with the Claude Code footer.

## Reporting to Mustafa
- Story first, asks last. Every chat update and every report ends with two sections, in
  this order:
  1. **Decisions needed:** one entry per decision, fully self-contained: a short code, the
     question in plain words, why it matters (one line), the options, and your
     recommendation. Never use a code (D1, K2, NC-4…) that isn't explained right there.
  2. **For Mustafa:** his actions as a numbered list (Brave checks with their steps,
     merges to approve), or "nothing".
- Plain words first; internal names (functions, review ids) only in brackets after them.
- Keep the top short: what happened, in a few lines.

## Night shift
- Approved work that can run unattended goes into the Night queue (issue #16) the day it
  is approved, with a link to its plan or verdict.
- Anything that needs Mustafa (Brave checks, merges, decisions) never goes in the queue:
  it goes in "For Mustafa".

## Commands
- Setup (once per machine, Node 22.13+): `cd tools && npm ci`, then inside tools/: `npx --no -- playwright install chromium`
  (the gate's smoke test 5a and pageshot need it)
- Gate: `node tools/gate.mjs [baseRef] [--allow-drop a,b] [--no-smoke]` (checks 1–4 and 5a–5d vs the base ref, default:
  merge-base with main or origin/main, whichever is newer; CI runs it)
- Official battery (JSON): `node tools/battery.js <dir>`
- 3D render: `node tools/preview.js <dir> shot <out.png> <x> <z>` · `menu <out.png>` · `eval <snippet.js>` (Linux: prefix `xvfb-run -a`)
- Headless match: `node tools/preview.js <dir> eval tools/sim.js [--map=tri]` · reachability: `eval tools/reach.js [--map=tri]` ·
  bot scaling: `node --expose-gc tools/preview.js <dir> eval tools/perfbots.js --map=tri --pre=<file.js>` (`--map=tri`: the three-team arena)
- Real pages (DOM + HUD + WebGL): `node tools/pageshot.js <dir> index.html <outPrefix> [--headed] <steps…>` (`--headed`: real GPU; every shot prints its WebGL renderer)
- Map layout validation: `tools/aabbs.js` + `tools/village.py`

## Code rules (file-specific traps load from .claude/rules/)
- Edit with the Edit tool (unique matches). Multi-site mechanical edits: an AST script (acorn), never blind regex.
- Never append a `//` comment inside a dense one-liner: it swallows the rest of the line.
- Place code by its references, not its neighbourhood (the placeCursor incident). Respect temporal dead zones at load.
- No allocation per frame in hot paths. Pool FX, dispose GPU resources on teardown, never dispose pooled/shared materials.
- Dynamic or player-provided text: `textContent`, never `innerHTML`.
- Both pages share head and DOM: edit both identically. War-room UI names (classes, tokens, keyframes) use `wr-`.
- Reports: plain language, scannable, decimals with periods, honest about what was NOT verified.

## Agents (.claude/agents/)
advisor · verifier · art-director · perf-auditor · security-auditor · netcode-architect · docs-keeper. Reviewers are
read-only and must cite evidence (file:line, numbers, render paths). Weigh their findings; they are not orders.

## Ultracode and workflows
Use them for WIDE work: audits, large migrations (on isolated copies), multi-angle plans with adversarial review,
verification sweeps, research. Workflows take no input mid-run, so each approval gate is its own run. Agents may read
and analyse in parallel; edits to the three engine files stay sequential. Try a small slice first.

## Never
- change a hard constraint without an accepted ADR · edit before the plan is approved · merge visual work before the Brave check
- commit secrets or read .env/secrets · force-push, hard-reset or rewrite history · delete user data or saves
- claim something works without evidence · trust a doc over the code · run unreviewed code fetched from the internet
