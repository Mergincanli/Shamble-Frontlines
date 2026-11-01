# Appendix to the v2-decision-drafts report (2026-11-01)

The records the report (`docs/reports/2026-11-01-v2-decision-drafts.md`) cites, word for word where the plan asks for
it. The working notes are in `.checkpoints/v2-decision-drafts/` on the branch (kept while the PR is a draft; deleted in
the last commit before it is marked ready, CHECKPOINTS.md §9.2–9.3).

## 1. The drafts brief, in full (the advisor's condition 12)

From Mustafa and the Advisor, 2026-10-31, the attachment of the session's first message (byte-identical copy in the
checkpoint's `inputs/DRAFTS_BRIEF.md`, sha256 c5172025…):

> # Decision drafts: brief for a fresh Claude Code session
>
> From Mustafa and the Advisor, 31 October 2026. Attached: `LOOK_V2.md`.
>
> ## 0. Where we are
> - **v2 phase 0.**
> - **Done:** kit-v3-0, R7 (the legacy inventory), R1–R6 research (PR #28), the docs follow-up (PR #31) and the handover
>   (PR #32, main `9e7db28`).
> - **Next, this brief:** the engine overview and six decision drafts. Then a stop for Mustafa and the Advisor.
>
> **Read first:**
> - `CLAUDE.md`;
> - `docs/STATUS.md`: its inputs and its five carry-forwards;
> - `docs/V2_PHASE0_BRIEF.md`, §4;
> - `docs/VISION_V2.md`, `docs/LOOP_V2.md`, `docs/EDITOR_V2.md`;
> - every topic in `docs/research/`, plus `GLOSSARY.md`;
> - `docs/LEGACY_INVENTORY.md`;
> - the attached `LOOK_V2.md`: add it to `docs/` first.
>
> Use the glossary's terms. Don't invent new meanings for them.
>
> ## 1. Settled inputs: don't reopen these
> - **The stack:**
>   - a Rust simulation core (Wasm in the browser, native on the server);
>   - a TypeScript client on three.js WebGPU, with a WebGL 2 fallback;
>   - our own netcode with server authority, over WebTransport with a WebSocket fallback.
> - **Size:** the biggest match is 24v24, so 48 players (MODES-3).
> - **Server tick:** 30 Hz is the baseline. 60 Hz only if the netcode proof test shows at least 25 % headroom.
> - **Slice order:**
>   1. move & shoot;
>   2. bots and three teams, played locally;
>   3. online;
>   4. the loop;
>   5. Octopon-3 and Editor v2;
>   6. scale to 48.
> - **Conditions on that order:**
>   - From slice 1, the game runs as a client plus a local server: the simulation in a Web Worker, talking through the
>     network's own messages, with no offline-only shortcuts.
>   - Movement and shooting are input-driven and replayable from day one.
>   - The netcode proof test runs before the foundation.
> - **Other settled answers:**
>   - preview links are private (PREVIEW-WALL);
>   - weapon spread is known only to the server (SEC-SPREAD);
>   - ability prediction is decided after a Feel Lab test (ABL-PREDICT);
>   - the security baseline stays open, with temporary positions (BASELINE-WHEN);
>   - in-match upgrade picks belong to the player: they survive death and reset at the start of every match
>     (UPGRADE-SCOPE, Mustafa, 2026-10-31). Record it with the other 2026-10-31 decisions in STATUS.
> - **Spikes** live in `spikes/`, are throwaway, and are exempt from the legacy hard constraints (SPIKE-SCOPE).
> - **The look** is `docs/LOOK_V2.md`. Authored 3D models (glTF/GLB) are allowed in v2.
>
> ## 2. Definitions: the Advisor's answer to the two open terms
> - **The foundation.** Phase 1's work before slice 1's gameplay:
>   - the repository restructure, moving the old game to `legacy/`;
>   - the toolchain: a Cargo workspace plus TypeScript with Vite;
>   - CI gate v2, and private preview links;
>   - TypeScript types generated from the Rust code;
>   - the client + local server skeleton: the simulation in a Web Worker, behind the same message protocol the network uses.
>
>   It ends when an empty scene runs through the full stack in the browser, with CI green and a preview link.
> - **The netcode proof test.** A throwaway spike that proves the online path:
>   - the Rust simulation runs natively as a server, and as Wasm in the client;
>   - they connect over WebTransport, with the WebSocket fallback;
>   - 48 simulated clients run at a 30 Hz tick, with tick time, bandwidth per client and memory all within the budgets that
>     ADR 0001 sets;
>   - one human client at 100 ms of simulated latency sees prediction, reconciliation and lag-compensated hits that look
>     correct.
>
> ## 3. Deliverables
> 1. **`docs/ENGINE_OVERVIEW.md`:** the engine on one page, with a diagram showing:
>    - the parts, and the Rust ↔ TypeScript boundary;
>    - the data flow per tick and per frame;
>    - where each requirement in VISION_V2 §6 lives.
> 2. **Six decision drafts, `docs/adr/0001`–`0006`,** using template 0000:
>    - 0001: multiplayer and netcode;
>    - 0002: code structure, build and toolchain;
>    - 0003: rendering. It must meet LOOK_V2, and its look proof test is judged by Mustafa in the browser.
>    - 0004: the simulation core: ECS, identical results on every machine, physics, navigation;
>    - 0005: content data, the ability system and the asset pipeline. Its ability system must also carry **in-match
>      upgrades** (pick one of three, still being designed). Their owner is settled (UPGRADE-SCOPE, §1): the player, not the
>      unit; in Unreal terms, GAS on the PlayerState, not the Pawn. How a pick applies after a switch to a different unit
>      is still open: list it as an open question;
>    - 0006: infrastructure and operations.
>
>    **Each draft contains:**
>    - the context, and the options with their trade-offs;
>    - the proposed decision and its consequences;
>    - the open questions;
>    - **what its spike must prove,** with measurable pass marks: numbers, hardware and method.
>
>    Every claim cites the research file and section it rests on.
> 3. **A spike plan:** every spike (at least the netcode, look and Rust ↔ TypeScript boundary spikes), with its pass marks,
>    its place in the order, and a rough size.
> 4. **Loose ends:** resolve the five STATUS carry-forwards and the critic's remaining contradictions, or list each one as
>    an open question.
>
> ## 4. How
> - **Plan first.** The advisor agent reviews the plan before any drafting.
> - **`engine-architect` leads,** at xhigh effort. Each draft's domain agent writes it:
>   - 0001: netcode-architect;
>   - 0003: graphics-engineer;
>   - 0004: rust-reviewer;
>   - 0005's ability system: game-designer;
>   - 0006: infra-engineer;
>   - the pass marks: test-engineer.
> - **Checks:** the verifier checks every draft's key claims against the research. A consistency pass then checks the
>   drafts against each other and against the glossary.
> - Checkpoints as usual, daytime only, Opus 5.5.
> - **No engine code, and no spikes yet.**
>
> ## 5. Stop
> Open a draft PR with the overview, the six drafts and the spike plan. Report per `CLAUDE.md`, then stop for Mustafa and the
> Advisor.

## 2. Mustafa's decisions in the session's chat (word for word)

- **UPGRADE-SCOPE update, 2026-10-31 about 18:41 Amsterdam** (it replaces the brief's upgrade note):
  > Update from Mustafa (2026-10-31). It replaces any earlier upgrade note from today.
  >
  > UPGRADE-SCOPE, a settled input for ADR 0005 (record it in STATUS with today's decisions):
  > - Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player.
  > - Tracks survive death and switching (switch back and the track is still there). They reset at the start of every match.
  > - A class always comes with its own weapon: no weapon choice when buying.
  > - Other weapons come only from killed enemies' drops; dropped weapons are world pickups.
  > - Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type.
  > - In Unreal terms, the tracks live on the PlayerState, not the Pawn.
  >
  > Still open; list them as open questions, don't decide them: carry slots, ammo for looted weapons, and how fast tracks level (including catch-up when switching).
- **The usage reset, 2026-10-31 late evening:** "I hit my usage limit while you were working, but it has reset now.
  Please continue from where you left off."
- **The one-off night run, 2026-10-31 between about 23:30 and 23:50:** "the next usage reset is going to be on 03:50 AM,
  so you can maybe tell it to prepare accordingly (starting the task at the right time)" and "Just for today (tonight):
  03:55, it can work until 11:00 because I will not have much time to use claude, so it can work if needed. You can take
  the lead in ensuring things get done accordingly, decide on when it start and when its done. Remember: just for today
  (upcoming session)". It set aside, for the run of 2026-11-01 03:55–11:00 only, the brief's "daytime only" and the
  advisor's condition 17; the session kept web fetching to 08:30 and later.
