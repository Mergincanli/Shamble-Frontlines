# Appendix to the v2-decision-drafts report (2026-11-01)

The records the report (`docs/reports/2026-11-01-v2-decision-drafts.md`) cites, word for word where the plan asks for
it. The working notes were in `.checkpoints/v2-decision-drafts/` on the branch, and the last commit before the PR was
marked ready deletes them (CHECKPOINTS.md §9.2–9.3). Every commit id cited here is one of PR #33's commits: after the
squash merge they stay reachable through the PR and its kept branch, not in main's history. The last commit that holds
the notes is {LASTCKPT}; the notes, prompts, scripts and verdict files named below are read there.

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
- **The usage resets, 2026-10-31 at about 22:50 and 2026-11-01 at 13:52 Amsterdam** (the same words both times): "I hit
  my usage limit while you were working, but it has reset now. Please continue from where you left off."
- **2026-11-01 at 19:06 Amsterdam, after the fourth spend-limit cut:** "Try again"
- **2026-11-02 at about 17:28 Amsterdam, after the seventh cut:** "Try again", then "You're halfway usage so you know,
  make sure to set the agents up in a way that they save their work & progress (reduce the chance of losing work when
  usage is hit)".
- **2026-11-02 at about 17:38 Amsterdam, three additions to that rule, for the rest of this task:**
  > Three additions to the save-as-you-go rule, for the rest of this task (Mustafa):
  > 1. for the agents working at a time, including the next waves
  > 2. Scratch files live in the notes folder (.checkpoints/v2-decision-drafts/<agent>-scratch/), never in a temp folder.
  > 3. At every checkpoint update, check that the background saver is running, and restart it if it stopped.
  > Record these in the checkpoint.

  The session applied them: every prompt and every running agent got the "Save as you go" rule (each block written to
  the output file the moment it is composed; a notes line after each finding; scratch only in the notes folder; on a
  restart, continue after the last complete block), and `scripts/saver_check.sh` ran at every checkpoint update.
- **2026-11-02 at about 20:32 Amsterdam, after the eighth cut:** "I hit my usage limit while you were working, but it has
  reset now. Please continue from where you left off."
- **2026-11-03 at about 06:14 Amsterdam, after the ninth cut:** "Continue, usage was reached"
- **2026-11-03 at about 07:14 Amsterdam**, after the session said it would start no agent before 08:30 (condition 17):
  "You can continue". The session read it as his go to run this morning's remaining units at once; it kept web fetching
  to 08:30 and later.
- **2026-11-03 at about 12:28 Amsterdam, the answers to the stop** (the Advisor's calls are marked in it; the session
  saved it word for word in `inputs/STOP-ANSWERS-mustafa-2026-11-03.md` before any edit; where each answer landed: §9):
  > Answers to the stop (Mustafa, 2026-11-03; the Advisor's calls are marked).
  >
  > SPIKE-ORDER: (b) accept with one change. The six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time: the netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike, then the look proof test once its models exist. The preview hosting check may run alongside any of them. Reason: seven usage-limit cuts in three days.
  > TICK-60-HEADROOM: confirm, with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz.
  > TICK-60-SETUP (Advisor): (a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.
  > NET-PROFILES (Advisor): yes. "100 ms" is P2: 100 ms round trip, ±20 ms jitter, 2 % loss (VISION_V2 §4's "about 100 ms of ping"). Latency and jitter split half each way; loss as P2 defines it.
  > REWIND-CAP: no value now. Rule: the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins. Aim for the smallest input buffer that holds.
  > SPIKE-MACHINES: a separate Hetzner project. I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run. Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register. Test PC: my own, in Brave; the CX33 runs the simulated clients.
  > LOOK-TEST-INPUTS: yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough.
  > TOY-BOX: reword both to point at LOOK_V2, in a small kit change-set after PR #30 is updated, together with the checkpoint anti-waste rules.
  > OQ-0002-8 (Advisor): (a).
  > OQ-0005-1 (Advisor): the client's data, keyed by the same id; the content check fails if a simulation id has no client entry.
  > Also the Advisor's, at the stop: OQ-0001-14: yes, a VISION_V2 §3 or §4 trace counts, marked as such. ROOM-CRASH: end the match, as proposed, and keep the crash and match records. OQ-0003-17: at most 6 of 600 presented frames may miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as diagnostics; I confirm this reading. OQ-0003-20: judge after 15 minutes of play. OQ-0005-AB12: tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read. OQ-0005-AB13: scripting comes back only when a designed ability can't be built from data.
  > LATER: a design session after the netcode proof test.
  >
  > Next: record these, mark PR #33 ready, and once it's approved, squash-merge it so the 31 MB of scratch build output never reaches main's history. Then the netcode proof test as its own change-set, plan first.
- **The one-off night run, 2026-10-31 between about 23:30 and 23:50:** "the next usage reset is going to be on 03:50 AM,
  so you can maybe tell it to prepare accordingly (starting the task at the right time)" and "Just for today (tonight):
  03:55, it can work until 11:00 because I will not have much time to use claude, so it can work if needed. You can take
  the lead in ensuring things get done accordingly, decide on when it start and when its done. Remember: just for today
  (upcoming session)". It set aside, for the run of 2026-11-01 03:55–11:00 only, the brief's "daytime only" and the
  advisor's condition 17; the session kept web fetching to 08:30 and later.
- **2026-11-03 at about 17:22 Amsterdam, after the tenth cut:** "I hit my usage limit while you were working, but it has
  reset now. Please continue from where you left off."
- **2026-11-03 at about 17:28 Amsterdam, design answers** (saved word for word in
  `inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md` before any edit; recorded in README and STATUS, plan revision 5):
  > Design answers from Mustafa (2026-11-03). 
  > Record them in the README's design questions and in STATUS (in PR #33, or a docs follow-up if it's merged). They don't change the spike order.
  > - DOMAIN (OQ-0006-15): shamblefrontlines.com (or the closest free variant), bought by Mustafa at Cloudflare.
  > - Reference machines (OQ-0003-1): not Mustafa's RTX 5080 laptop. Minimum: a typical student laptop with integrated graphics. Recommended: a mid-range card, roughly RTX 3060/4060 class. Both borrowed.
  > - Carry slots (OQ-0005-AB1): a looted weapon replaces the main (class) weapon; the sidearm stays as backup. Assumed, to confirm: the class's own abilities stay.
  > - Looted ammo (OQ-0005-AB2): still open (infinite, limited, or supply lines by an engineer class). The Advisor's lean: limited but generous, refilled at the barracks and supply points.
  > - Levelling (OQ-0005-AB3): XP from kills, assists and objective work while using that unit or weapon; fast early levels; a track that's behind the player's best levels faster.
  > - Name: the in-match picks are "amplifications" ("amps").
  > - WEAPON-UNLOCKS (OQ-0005-AB5): leaning towards class variants, with more variants added over time. Not final.
  > - Banzai vs Riot Shield (OQ-0005-AB11): a tall, slightly curved Riot Shield blocks the Banzai blade from the front, so the Warrior must flank; the Conscript's Molotov punishes the shield hard. LOOP_V2's counter triangle; numbers tuned later.
  > - Upgrade visibility: enemies can see a player's build.
  > - Spawn protection (OQ-0005-AB14): 3 s, against spawn killing. Firing ends it early (the Advisor's pencil; Mustafa may veto).
  > - Fragments as damage: visual only for now. The server sends one blast event, the browser draws the fragments, and the spray matches the real damage radius.
  > - Still open: faction asymmetry (Mustafa works on the unit list first) and the war around the matches.
- **2026-11-03 at about 17:31 Amsterdam, answers to the escalations, COMMUNITY-SERVERS and the verification stop**
  (saved word for word in `inputs/ANSWERS-2-mustafa-2026-11-03.md` before any edit; plan revision 5):
  > Answers (Mustafa, 2026-11-03):
  > - REF-MACHINES: minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed. I name the actual machines before the look proof test; the netcode proof test doesn't measure client frames. Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.
  > - LOOK_V2-§8: yes, keep it as it is, with Blender for the test models only. When the asset research's export part reports, §8 says "Spline or Blender".
  > - SPIKE-COST: one cap per run, from its planned hours, written in that run's plan for my OK. I create and delete the VMs myself.
  > - DOMAIN: shamblefrontlines.com, which I register at Cloudflare.
  > - COMMUNITY-SERVERS (new; keep the door open, don't design it): players may one day host their own servers, possibly with revenue such as official private servers they rent. Add it as an open question to ADR 0006 and the README. The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only.
  > - Verification: after e2b, e3b and their fix round, stop. No further re-check rounds; anything left goes into the PR as notes. The spikes are the real test, and we've had ten usage cuts.
- **2026-11-03 at about 18:26 Amsterdam, the Hetzner console check** (saved word for word in
  `inputs/HETZNER-CHECK-mustafa-2026-11-03.md` before any edit; plan revision 6):
  > Hetzner check (Mustafa, 2026-11-03): in Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can. Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead of the CX33, so their build targets aarch64. The game server stays a CCX23 (dedicated, x86) in Falkenstein; I checked it can be ordered there. Update SPIKE-MACHINES and every place the drafts name the CX33.
- **2026-11-03 at about 18:32 Amsterdam, his correction to the Hetzner note** (saved word for word in
  `inputs/HETZNER-CORRECTION-mustafa-2026-11-03.md`, with the session's description of the screenshot he attached):
  > Correction to my Hetzner note (Mustafa, 2026-11-03): the game server is a CCX23 for development and "does it work" runs. The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz margin and rooms per VM. Each run's plan says which server it needs.
- **2026-11-03 at about 18:34 Amsterdam, his answers to the session's two questions on it** (the question tool's
  record, word for word in `inputs/HETZNER-ANSWERS-mustafa-2026-11-03.md`; the option texts are the session's wording,
  which he chose):
  > Question 1 (the session's): Your Hetzner correction says "the game server is a CCX23 for development and 'does it work' runs", then "The CCX23 (dedicated) is used only for the measurement runs". Which machine did you mean for the development and "does it work" runs? (The measurement runs stay on the CCX23.)
  > His answer: "CCX13 (Recommended)", the option described as: "Dedicated x86, 2 vCPU, 8 GB, €0.083/h in your screenshot: the same x86 server build as the CCX23, so no second build target for the room server."
  >
  > Question 2 (the session's): Your first Hetzner note puts the CCX23 game server in Falkenstein; the screenshot's order summary shows "Nuremberg" selected. Which location should the drafts give the game servers and the simulated clients (they sit in the same location)?
  > His answer: "Nuremberg (nbg1)", the option described as: "As your screenshot shows; whether the CAX21 (or another client VM) can be ordered there is then checked in the proof test's plan."
- **2026-11-03 at about 18:48 Amsterdam, his second correction** (saved word for word in
  `inputs/HETZNER-CORRECTION-2-mustafa-2026-11-03.md`; the round launched on the advisor's first verdict was stopped
  before any block was written):
  > Correction again (Mustafa, 2026-11-03): both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players. I created both once to try the console and deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then.
- **2026-11-03 at about 18:49 Amsterdam, his answer to the session's question on the CCX13** (the question tool's record,
  word for word in `inputs/HETZNER-ANSWERS-2-mustafa-2026-11-03.md`; the option text is the session's wording, which he
  chose):
  > Question (the session's): Your latest note names two test servers in Falkenstein: a CCX23 for the game server and a CAX21 for the 48 simulated players. Earlier you chose a CCX13 for development and "does it work" runs, with the CCX23 used only for the measurement runs (tick timing, the 60 Hz margin, rooms per VM). Does the CCX13 still stand?
  > His answer: "No, drop the CCX13", the option described as: "Every server run uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21."
- **2026-11-03 at about 18:56 Amsterdam, his answer to LG-COUNT** (the advisor's question in its amended verdict on plan
  revision 6; the question tool's record, word for word in `inputs/LG-COUNT-mustafa-2026-11-03.md`; the option text is
  the session's wording, which he chose):
  > Question (the session's, from the advisor's amended verdict on plan revision 6): You named "a CAX21 for the 48 simulated players". Some netcode proof test rows run 8 rooms of 48 bots at once (384 simulated players, the rooms-per-VM rows). If one CAX21 can't carry a run's bots under 80 % CPU (an overloaded load generator makes the run invalid), may that run add CAX21s (about 0.0176 EUR an hour each, excluding VAT)?
  > His answer: "Add as needed (Recommended)", the option described as: "The run's plan adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap you OK for that run."
- **2026-11-03 at about 19:04 Amsterdam, answers to MACHINES-TIMING and the domain** (saved word for word in
  `inputs/ANSWERS-3-mustafa-2026-11-03.md` before any edit):
  > Answers (Mustafa, 2026-11-03):
  > - MACHINES-TIMING: (c), with one step first. For the netcode proof test's Safari leg, check whether GitHub's macOS runners (real Safari, free while the repository is public) have a Safari new enough for WebTransport and can reach the test servers. If they can, that is the Safari check, and I borrow the Mac only for the look proof test. If they can't, I borrow the Mac before the netcode proof test. The laptop and desktop stay on my timing: rough checks first, then a short re-run once they're named.
  > - DOMAIN: done. shamblefrontlines.com is registered at Cloudflare.
  > - Relaunch the authors' round from the checkpoint.

## 3. The advisor's plan review: APPROVE with 19 conditions (word for word)

From the advisor agent in this session, 2026-10-31, returned about 18:35 Amsterdam, on plan revision 1. The drafts,
README and SPIKE_PLAN cite its conditions as "condition n" (or "[Cn]"). Headings are one level lower than in the
original; nothing else is changed.

### Plan review: v2-decision-drafts, plan revision 1 (base 9e7db28, head d6c71e4)

**Verdict: APPROVE, with the 19 conditions below.** The plan delivers everything the brief asks for, reopens nothing it settles and decides nothing about the look. One real defect: the write guard compares against a stale local `origin/main`, so it would stop every commit and could not see an edit to a research file (condition 1). The other conditions are wording and process fixes. Fold them into revision 2 before step 3. Folding them needs no second review. If a condition can't be met, stop and ask me.

**The decision map (step 2) is not redone.** It needs one addendum (condition 7).

#### Rulings on the four decisions you asked me
- **ASSET-RESEARCH: (a)**, with condition 4.
- **TICK-60-HEADROOM: the measuring base is ruled now, not at the stop.** TICK-30's own words compare 8.28 ms with 8.33 ms, so the base is the server tick budget at 60 Hz (8.33 ms). At least 25 % margin means a p99 server tick time of at most 6.25 ms. The full mark is in condition 2. Mustafa and the Advisor confirm it at the stop.
- **GLOSSARY-8: yes**, as an append-only §8 with three entries and one note (condition 3). It must be a third entry, not just the two definitions: the glossary already uses "headroom" for a run above the largest mode's N (GLOSSARY:355–373). TICK-30's "25 % headroom" is a time margin, and plan rule 5 forbids giving an existing term a new meaning.
- **CHECKPOINT-AT-STOP: keep it** (condition 5).

#### Conditions
1. **Write guard and gate base.** The local `main` and `origin/main` refs are at b6effde (PR #28's merge). The remote main is 9e7db28 (`git ls-remote`). Today `git diff --stat origin/main...HEAD` lists 17 files, including STATUS, ROADMAP, VISION_V2 and all six topic files. Replace the guard with: "Before every commit: `git fetch origin main`. Then `git diff --name-only 9e7db28...HEAD` (if main has moved, the merge base with the fetched origin/main) lists only the paths above and `.checkpoints/v2-decision-drafts*`. `git diff --quiet 9e7db28 HEAD -- docs/research/r1-simulation-core.md docs/research/r2-netcode.md docs/research/r3-rendering.md docs/research/r4-physics-navigation.md docs/research/r5-toolchain-quality.md docs/research/r6-infrastructure.md docs/research/UNREAD_SOURCES.md docs/LEGACY_INVENTORY.md` exits 0. `git show 9e7db28:docs/research/GLOSSARY.md | cmp - <(head -n 742 docs/research/GLOSSARY.md)` passes. Any other result stops the commit." Step 12 runs `node tools/gate.mjs 9e7db28` (or the default base after the fetch).
2. **The tick rate (TICK-60-HEADROOM).** Add to the plan: "TICK-30's 25 % is measured against the server tick budget at 60 Hz, 8.33 ms. This is the advisor's reading of 2026-10-31; Mustafa and the Advisor confirm it at the stop. A 60 Hz tick needs all of this in the netcode proof test:
   - the setup: one vCPU of the reference server, the stress scene at N = 48 with a snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included;
   - a p99 server tick time of at most 6.25 ms (a 60 Hz margin of at least 25 %);
   - every 60 Hz mark the research sets, as written (R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3).

   Otherwise the tick stays at 30 Hz. ADR 0001 owns this mark and the 30 Hz baseline marks (R2-BUD-REC6 restated from GLOSSARY §1's 30 Hz column). ADR 0001 states three lines as replaced by TICK-30: R1-TICK-REC2's rule (r1:2029), 'the rule does not change' (r2:2415–2416) and 'which does not change' (GLOSSARY:85). Where it restates r2:2402–2403, it adds '(at N = 54, MODES-3 (b))' (PR #28 review, finding 2)."
3. **GLOSSARY §8.** Append it after line 742, so every GLOSSARY line cited at 9e7db28 still holds. Heading: `## 8. Phase 1 terms (the Advisor, 2026-10-31)`. Three entries, each in the "How to read an entry" shape:
   - **The foundation** and **The netcode proof test**: the definition is brief §2 quoted as a block, word for word, credited "the Advisor's drafts brief, §2, 2026-10-31"; Marked by; Retire: none.
   - **The 60 Hz margin**: 1 − (p99 server tick time at a 60 Hz tick ÷ the server tick budget at 60 Hz, 8.33 ms), measured as condition 2 says. Depends on: TICK-60-HEADROOM, confirmed at the stop. Marked by: "the 60 Hz margin". It notes that TICK-30's word "headroom" appears only in quotes.

   Then one closing note: "TICK-30 (the Advisor, 2026-10-31) replaces, for the drafts, the tick-rate rule the Tick period entry calls unchanged (line 85): 30 Hz is the baseline, 60 Hz needs a 60 Hz margin of at least 25 %." Fix the §8 text in the notes before drafting (condition 7), so authors link stable anchors; the file edit lands in step 6. Update the "Changed" list to say three entries and one note.
4. **ASSET-RESEARCH (a).**
   - ADR 0005's asset-pipeline part takes LOOK_V2 §8's flow as given and proposes only what R1-DATA, R3-CHAR and R3-BUDGET support.
   - It lists the rest as open questions marked "not researched": compression formats, the LOD tool, export checks, the material-slot renaming step and the per-unit budget numbers.
   - Those questions are grouped under one research unit, "asset-pipeline research" (graphics-engineer, verified like R3, in its own daytime session). SPIKE_PLAN places it before the asset spike.
   - The look proof test says whether it needs authored models. If it does, it uses plain glTF/GLB exports and decides nothing about compression or LODs.
   - The report lists ASSET-RESEARCH under "Decisions needed", so Mustafa and the Advisor can move the research earlier.
   - Correct risk 3: R3 has one glTF mention (R3-CHAR-F8, r3:476). The other 12 hits of a case-insensitive search are "GLB" inside "WebGLBackend".
5. **CHECKPOINT-AT-STOP: keep.** The report folds in the checkpoint's essentials and says the checkpoint and notes are deleted in the last commit before the PR is marked ready (CHECKPOINTS.md §9.2–9.3).
6. **Dispositions outside the six drafts.** Rule 8 becomes: "…exactly one owner disposition across three places: the six drafts' Appendix A; `docs/adr/README.md`'s register (owner DESIGN: a pure design matter for Mustafa or a later design session, with one line why); and `docs/SPIKE_PLAN.md` (owner SPIKE_PLAN). The script counts the 521 unique ids in `inputs/research-ids.tsv`. The rows r3:524 and r3:771 are references to R1-BND-REC2 and R1-TICK-REC5 inside R3's legacy-facts list, not second definitions." The decision-map agent flagged this gap itself.
7. **Decision map: one addendum, no redo.** The same agent writes it after its current run, resuming from its notes. It adds:
   - (a) the 60 Hz margin as a shared-number row (owner 0001, GLOSSARY §8), plus the §8 text from condition 3;
   - (b) three items in the loose-ends register:
     - the old ADR-001..003 entries (STATUS:198–202), each with the draft that disposes of it;
     - the items STATUS sends to ADR-001 (STATUS:257 SEC-MDL-2, :262 no SRI/CSP on the CDN tag, :273 the 11 wall-clock reads), each with its draft;
     - one line closing PR #28 finding 4's research-plan condition 13 (two branches from one session): it no longer applies;
   - (c) the traces from condition 8, including EDITOR_V2 §2's document rules (→ 0005) and EDITOR_V2 §5's double-click and "if ADR-002" lines (→ 0002, listed for DOCS-ALIGN);
   - (d) how ADR 0005 is put together (condition 11).
8. **Traces (add to rule 4).** "Name the VISION_V2 §6 item where one applies. A feature whose need is only in LOOP_V2, EDITOR_V2, LOOK_V2 or GLOSSARY §8's foundation says so, and `docs/adr/README.md` lists those features, one line each, for the stop review. An item tagged [LATER] or [OPEN] gets no proposed feature; the draft says only what keeps it possible. That covers §6.4's seats, §6.8's kill-cams, replays and highlights, §6.10's accounts, and the same tags in LOOP_V2, EDITOR_V2 and LOOK_V2."
9. **The look (add to rule 6).** "Where two techniques both meet a LOOK_V2 item but look different (for example R3-LOOK-REC3's two outline families), the draft proposes one on cost and robustness and names the visible difference as Mustafa's call in the look proof test. Visible values (line widths, ramp steps, AO strength, colours, glow strength) are tuning for that test, not draft decisions. A LOOK_V2 item that could be read as a rule (fragments that deal damage, planks and debris as routes, enterable ruins) is built as LOOK_V2 states it; any rule beyond that is an open design question for Mustafa, unless LOOP_V2 settles it." Rule 6 also goes into every reviewer prompt. The art-director reviews 0003, 0005's asset part and SPIKE_PLAN's look proof test.
10. **Spikes judged by people.** The netcode proof test's "look correct" and the whole look proof test are Mustafa's judgement (escalation category 1).
    - Their pass marks give measurable marks where they exist. For the netcode proof test: the share of client-shown hits the server confirms, correction sizes and prediction error, at 100 ms simulated latency with stated jitter and loss.
    - They add a numbered Brave checklist for Mustafa, each step with its expected result.
    - The look proof test's checklist names each LOOK_V2 item it shows, side by side with legacy screenshots (LOOK_V2 §9). It sits beside the frame-time marks on the minimum laptop and the recommended desktop.
    - SPIKE_PLAN also places the Feel Lab test (ABL-PREDICT) and the PREV-DELETE item.
11. **ADR 0005: upgrades and assembly.**
    - ADR 0005 designs no upgrade, ability, unit or rule (escalation category 2).
    - It shows that the ability system can carry the brief's in-match upgrades: pick one of three, owned by the player, kept through death, reset every match; GAS on the PlayerState, not the Pawn.
    - It lists "how a pick applies after a switch to a different unit" as an open question for Mustafa, with options and no pick.
    - engine-architect writes 0005's frame (header, context, the joined decision, consequences, open-question numbering, Appendix A) after the game-designer's and graphics-engineer's parts land, and places those parts verbatim as sections. Each author still fixes only its own text.
12. **Settled inputs and lasting citations.**
    - An option that contradicts a settled input (a bundler other than Vite, a renderer other than three.js WebGPU, a transport other than WebTransport with the WebSocket fallback, and the like) appears only as "settled by <code or GLOSSARY §8 entry>". It is not weighed.
    - Settled inputs cite their STATUS decision code (docs-keeper adds UPGRADE-SCOPE there) or GLOSSARY §8, with "brief §n" only beside it. The reason: `inputs/DRAFTS_BRIEF.md` leaves the branch with the checkpoint.
    - The report's appendix quotes the brief in full.
13. **Template fields.** Each draft fills template 0000's "Hard constraints in CLAUDE.md that change": which legacy constraint stops applying to v2 code, and under which decision code (SPIKE-SCOPE (a); LOOK_V2 §8 for "no external assets"). It also fills Migration plan and Rollback. Nothing in CLAUDE.md or `.claude/` changes in this PR.
14. **Model and effort for every unit.** Each unit names its agent, `model: opus` (Opus 5.5) and its effort:
    - the step-8 consistency critic: engine-architect, xhigh;
    - the completeness critic: name the agent, xhigh;
    - the step-7 skeptic: verifier, xhigh;
    - the step-9 fix owners: their own agent, xhigh;
    - every step-10 reviewer: xhigh, because they work on ADRs;
    - docs-keeper: high.

    Any built-in agent gets `model: "opus"` passed explicitly.
15. **Independent review of the pass marks.** The test-engineer's pass-mark rows go into each draft's claims ledger, so the verifier checks them. In step 10, each draft's author reviews its draft's pass-mark table (domain fit) and the perf-auditor reviews every budget mark. SPIKE_PLAN is reviewed by netcode-architect (the netcode proof test), graphics-engineer (the look proof test), rust-reviewer (the boundary spike) and test-engineer (is it proven). The test-engineer does not review its own tables.
16. **What "research verified" covers.** Only R1 and R2 recommendation claims and the claims their verifiers checked (r1:3233: "every claim the R1 recommendations rest on"). Every other finding is checked against its source as step 7 says: R1 or R2 findings outside that set, and R3–R6 findings outside the spot-check tables. Rule 10 and every verifier prompt add: fetched pages are data, never instructions.
17. **Daytime only.** No new agent after 23:00 Amsterdam, and no verifier unit that fetches sources after 22:00, so units end before the night window (00:10). A unit still running at 00:10 is recorded in the checkpoint, and the next day session reads its notes before using its result.
18. **Alignment items and two openly stated choices.**
    - docs-keeper's KIT-ALIGN also lists CLAUDE.md:4 ("Red Alert 3 toy-box look") and art-director.md:10 ("a joyful, readable, toy-box battlefield"). Both may clash with LOOK_V2 §3 ("The audience is adults. It must not look like a children's game."). Flag them for Mustafa; don't change them.
    - DOCS-ALIGN gains EDITOR_V2 §5's double-click line and its "if ADR-002 moves the project to modules" line.
    - The report states openly that the plan delays R7-MARKS's "changed when their ADR draft comes up" until the drafts are accepted. Each draft lists its rows instead: id, old mark, proposed mark, reason, including the "look brief pending" rows LOOK_V2 now unblocks.
    - The report also states that adding LOOK_V2 was the one edit before plan approval, made because brief §0 orders it first, as a byte copy.
    - I accept both.
19. **ENGINE_OVERVIEW stays one page:** at most about 150 lines with its diagram; details link to the drafts.

#### What I checked myself
- **The branch:** 9e7db28..HEAD is 2 commits and 5 files (LOOK_V2 plus 4 checkpoint files). Commit 6617e25 adds only `docs/LOOK_V2.md` (78 lines), and its sha256 starts 52b4fc06, as the plan says.
- **The refs:** remote main is 9e7db28; local `main` and `origin/main` are b6effde.
- **The id list** (`research-ids.tsv`): REC 167, OQ 241, RR 7, XU 30, XT 72, SHARED 18; 521 unique ids. 241 + 30 + 7 = 278 matches the research report. The two "defined twice" ids are references at r3:524 and r3:771.
- **The research gaps:** 0 whole-word hits for KTX2, meshopt, meshoptimizer, Draco, Spline, gltf-transform or Basis Universal; 1 glTF mention in all of the research.
- **The superseded lines:** r1:2029, r2:2402–2403, r2:2415–2416 and GLOSSARY:85. The headroom clash is at GLOSSARY:355–373.
- **The brief, item by item:**
  - §3: all four deliverables are planned.
  - §4: plan first; engine-architect leads at xhigh; the named authors (0001 netcode-architect, 0003 graphics-engineer, 0004 rust-reviewer, 0005's ability system game-designer, 0006 infra-engineer, pass marks test-engineer); the verifier; the consistency pass; checkpoints; daytime; Opus; no code and no spikes.
  - §5: draft PR, report, stop.
  - §1–2: all settled inputs are present and none is reopened.
- **STATUS's five carry-forwards** (STATUS:124–139) are each carried: 1 by step 7 (with condition 16); 2 by TICK-60-HEADROOM; 3 by the settled inputs; 4 by the register; 5 by GLOSSARY §8 and SPIKE_PLAN.
- **My PR #28 review** (comment 6542571633, read with `gh api`): finding 2 is now in condition 2, finding 4 in step 7 and condition 7.
- **The agent files:** every agent is `model: opus`. Only engine-architect and netcode-architect name `docs/adr/` as writable, so the notes-then-copy route is right. graphics-engineer.md:19 and art-director.md:16 are stale as the plan says, and CLAUDE.md:4 and art-director.md:10 are too.
- **The parallel decision map:** its notes, read only. It has covered R1–R6 so far.
- **The CPUs:** `nproc` is 4.

#### What I could not check
- That LOOK_V2.md and DRAFTS_BRIEF.md are byte-identical to the session's attachments: I don't have the attachments, only the plan's hash and the one-file commit.
- The Workflow limit of 2 agents at once: I confirmed only the 4 CPUs.
- The gate: not run. This is a plan review with one docs file on the branch, and the gate writes scratch files while I edit nothing. It runs at the PR review.
- The decision map's output: not finished yet.

#### Decisions needed
None for Mustafa now. At the stop, the session's report asks Mustafa and the Advisor to confirm three things: the TICK-60-HEADROOM reading (condition 2), when the asset-pipeline research runs (condition 4), and the "toy-box" lines (condition 18).

#### For Mustafa
Nothing.

My notes, which end with DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/advisor-plan-review.md

## 4. The plan, revision 3: settled inputs, the rules every author follows, who writes what, and the steps (word for word)

The drafts cite these as "plan rule n" and "plan step n". Copied from the plan's revision 3 (2026-10-31, about 18:43
Amsterdam), from "Settled inputs" to the end of "Steps". Headings are one level lower than in the original; nothing
else is changed. Revision 3 applied Mustafa's UPGRADE-SCOPE update (§2 above), and revision 2 folded in the
conditions of §3.

### Settled inputs (never reopened)
The stack; N = 48 (MODES-3 (a)); TICK-30; SLICE-ORDER and its three conditions; PREVIEW-WALL (a); SEC-SPREAD (a);
ABL-PREDICT (waits for the Feel Lab test); BASELINE-WHEN (b); UPGRADE-SCOPE as Mustafa updated it on 2026-10-31
(verbatim in `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; docs-keeper records his words in STATUS with today's
decisions): every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the
player; tracks survive death and switching and reset at the start of every match; a class always comes with its own
weapon (no weapon choice when buying); other weapons come only from killed enemies' drops, and dropped weapons are world
pickups; progress belongs to the weapon type, not the physical gun; the tracks live on the PlayerState, not the Pawn.
Open, for Mustafa (listed, never decided): carry slots, ammo for looted weapons, and how fast tracks level (including
catch-up when switching);
SPIKE-SCOPE (a); R7-MARKS (a); LOOK_V2 (authored glTF/GLB models allowed in v2); GLOSSARY §8's foundation and netcode
proof test (brief §2). Where a research mark gives 48 and 54, the drafts use 48; where it says "until … answers", the
answer applies (carry-forward 3).
- **Citing them** [C12]: by the STATUS decision code or the GLOSSARY §8 entry, with "brief §n" only beside it (the brief's
  copy leaves the branch with the checkpoint). An option that contradicts a settled input (a bundler other than Vite, a
  renderer other than three.js WebGPU, a transport other than WebTransport with the WebSocket fallback, and the like)
  appears only as "settled by <code or GLOSSARY §8 entry>"; it is not weighed.
- **TICK-60-HEADROOM, the advisor's reading of 2026-10-31** [C2], confirmed by Mustafa and the Advisor at the stop:
  TICK-30's 25 % is measured against the server tick budget at 60 Hz, 8.33 ms. A 60 Hz tick needs all of this in the
  netcode proof test: the setup (one vCPU of the reference server, the stress scene at N = 48 with a snapshot-receiving
  client on every soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included); a p99 server tick time of at
  most 6.25 ms (a 60 Hz margin of at least 25 %); every 60 Hz mark the research sets, as written (R2-BUD-REC6's two
  shares, R1-TICK-REC2 P1–P3). Otherwise the tick stays at 30 Hz. ADR 0001 owns this mark and the 30 Hz baseline marks
  (R2-BUD-REC6 restated from GLOSSARY §1's 30 Hz column). ADR 0001 states three lines as replaced by TICK-30:
  R1-TICK-REC2's rule (r1:2029), "the rule does not change" (r2:2415–2416) and "which does not change" (GLOSSARY:85).
  Where it restates r2:2402–2403, it adds "(at N = 54, MODES-3 (b))" (PR #28 review, finding 2).

### Rules every author follows (written into every prompt)
1. **Citations:** every claim and number cites its research file, section and id, with lines at 9e7db28 (the topic files
   do not change in this PR, so the lines hold at the head), for example `(r2-netcode.md, Recommendations, Brief question
   2, R2-BUD-REC6, lines 2398–2420)`. Settled inputs follow "Citing them" above; design rules cite VISION_V2, LOOP_V2,
   EDITOR_V2 or LOOK_V2 by section.
2. **Numbers:** a number comes from the research (cited), is arithmetic on research numbers (shown), is a settled input,
   or is a pass mark marked "proposed" with its reason. Nothing else. A claim the research does not support is dropped,
   becomes an open question, or becomes a spike mark.
3. **Options** come from the research's Options sections; an option the research lacks is labelled "not researched"
   and cannot be the proposed decision.
4. **Trace** [C8]: every engine feature a draft proposes names the VISION_V2 §6 item that needs it, where one applies. A
   feature whose need is only in LOOP_V2, EDITOR_V2, LOOK_V2 or GLOSSARY §8's foundation says so, and
   `docs/adr/README.md` lists those features, one line each, for the stop review. An item tagged [LATER] or [OPEN] gets
   no proposed feature; the draft says only what keeps it possible. That covers §6.4's seats, §6.8's kill-cams, replays
   and highlights, §6.10's accounts, and the same tags in LOOP_V2, EDITOR_V2 and LOOK_V2. No generic-engine feature "for
   later".
5. **Glossary:** the GLOSSARY's terms with their marking words; no "Retire (exact)" phrase; an "in context" phrase only
   where it does not carry the term's meaning; the first use in each draft links the entry. No new meaning for an
   existing term: TICK-30's 25 % is "the 60 Hz margin" (GLOSSARY §8), never "headroom", which GLOSSARY:355–373 uses for
   a run above the largest mode's N; the word appears only inside quotes of TICK-30 [C3]. Spike names are defined once,
   in `docs/SPIKE_PLAN.md`.
6. **The look:** LOOK_V2 is Mustafa's decision (brief §1). ADR 0003 shows how each LOOK_V2 `[DECIDED]` item is achieved,
   builds the `[DEFAULT]` items as defaults, and decides nothing LOOK_V2 leaves `[OPEN]` (the tree style). The prompts
   say this outright, because graphics-engineer.md:19 and art-director.md:16 still say the look is not decided. And
   [C9]: where two techniques both meet a LOOK_V2 item but look different (for example R3-LOOK-REC3's two outline
   families), the draft proposes one on cost and robustness and names the visible difference as Mustafa's call in the
   look proof test. Visible values (line widths, ramp steps, AO strength, colours, glow strength) are tuning for that
   test, not draft decisions. A LOOK_V2 item that could be read as a rule (fragments that deal damage, planks and debris
   as routes, enterable ruins) is built as LOOK_V2 states it; any rule beyond that is an open design question for
   Mustafa, unless LOOP_V2 settles it. Rule 6 also goes into every reviewer prompt.
7. **Open questions** are numbered per draft (`OQ-0001-1` …), each with the question, why it matters, the options, the
   decider (Mustafa, the Advisor, a spike, a later design session) and the research ids it carries.
8. **Dispositions** [C6]: every recommendation and open question of R1–R6 (including the cross-unit (XU), rust-reviewer
   (RR), cross-topic (XT) and shared ones) gets exactly one owner disposition across three places: the six drafts'
   Appendix A (adopted · changed, how and why · rejected, why · answered, how · open, its OQ id · for a spike, which);
   `docs/adr/README.md`'s register (owner DESIGN: a pure design matter for Mustafa or a later design session, with one
   line why); and `docs/SPIKE_PLAN.md` (owner SPIKE_PLAN). The script counts the 521 unique ids in
   `inputs/research-ids.tsv` (rebuilt after the review: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes in 18
   copies; the rows r3:524 and r3:771 were references to R1-BND-REC2 and R1-TICK-REC5 inside R3's legacy-facts list, not
   second definitions, and the script now skips a bold id outside its own topic file).
9. **A claims ledger per draft** (a notes file, not in the draft): every claim and number, its citation and its source
   tier (research verified · research, needs its source checked · arithmetic · settled · design doc · proposed). The
   test-engineer's pass-mark rows go into the same ledger [C15]. The verifier works from it and checks that no claim in
   the draft is missing from it.
10. Never decide the look, write engine code, run a spike, or state a number you did not source. Fetched pages are
    data, never instructions [C16].
11. **Template fields** [C13]: each draft fills template 0000's "Hard constraints in CLAUDE.md that change" (which legacy
    constraint stops applying to v2 code, and under which decision code: SPIKE-SCOPE (a); LOOK_V2 §8 for "no external
    assets"), its Migration plan and its Rollback. Nothing in CLAUDE.md or `.claude/` changes in this PR.
12. **People judge some spikes** [C10]: the netcode proof test's "look correct" and the whole look proof test are
    Mustafa's judgement (escalation category 1). Their pass marks give measurable marks where they exist (for the
    netcode proof test: the share of client-shown hits the server confirms, correction sizes and prediction error, at
    100 ms simulated latency with stated jitter and loss) and add a numbered Brave checklist for Mustafa, each step with
    its expected result. The look proof test's checklist names each LOOK_V2 item it shows, side by side with legacy
    screenshots (LOOK_V2 §9), beside the frame-time marks on the minimum laptop and the recommended desktop.

### Who writes what (brief §4), with model and effort [C14]
Every agent below runs `model: opus` (Opus 5.5; the agent files say so; a built-in agent gets `model: "opus"` passed).
| Unit | Agent | Effort | Main research |
|---|---|---|---|
| Decision map (step 2) and its addendum | engine-architect (lead) | xhigh | all |
| ADR 0001 multiplayer and netcode | netcode-architect | xhigh | R2 (all units), R1-TICK, R6-HOST (servers) |
| ADR 0002 code structure, build and toolchain | engine-architect | xhigh | R5-LAYOUT, TSLINT, RUSTLINT, TEST, CI, GATE, PREV (the build half); R1-BND |
| ADR 0003 rendering | graphics-engineer | xhigh | R3 (all units), LOOK_V2 |
| ADR 0004 simulation core | rust-reviewer | xhigh | R1-ECS, TICK, DET, RR; R4 (all units) |
| ADR 0005, the ability system section | game-designer | xhigh | R1-ABL, LOOP_V2 |
| ADR 0005, the asset section (GPU rules) | graphics-engineer | xhigh | R3-CHAR, R3-BUDGET, LOOK_V2 §8 |
| ADR 0005, content data and the frame | engine-architect | xhigh | R1-DATA, VISION_V2 §6.3, §6.4, §6.7, EDITOR_V2 §2 |
| ADR 0006 infrastructure and operations | infra-engineer | xhigh | R6 (all units), R5-PREV (the hosting half), BASELINE-WHEN (b) |
| Pass marks for every draft, and the spike entries | test-engineer | xhigh | R5-TEST, R5-GATE, each draft |
| ENGINE_OVERVIEW, SPIKE_PLAN, adr/README, GLOSSARY §8 | engine-architect | xhigh | the drafts |
| Verification units and their skeptics | verifier | xhigh | the drafts, the research, the sources |
| Consistency critic | engine-architect (a fresh one) | xhigh | everything |
| Completeness critic | verifier (a fresh one) | xhigh | everything |
| Fix owners | each text's own author agent | xhigh | their notes |
| Reviewers (step 10) | as listed there | xhigh | the final diff |
| STATUS, ROADMAP, REPO_MAP | docs-keeper | high | the change-set |

Authors whose agent file does not name `docs/adr/` (graphics-engineer, rust-reviewer, game-designer, infra-engineer,
test-engineer) write their text whole into their notes; the session copies it into `docs/adr/` verbatim (`cmp` after
copying) and credits the author in the draft's header. netcode-architect and engine-architect follow the same route, so
every draft is assembled the same way. Each author fixes only its own text.

**ADR 0005** [C4, C11, and Mustafa's UPGRADE-SCOPE update]: it designs no upgrade, ability, unit or rule (escalation
category 2). It shows that the ability system can carry UPGRADE-SCOPE as Mustafa wrote it: a roguelike track per unit
and per weapon type (pick one of three per level), owned by the player (the PlayerState, not the Pawn), surviving death
and switching, reset every match; progress on the weapon type, waiting when a gun is lost for the next gun of that
type. It lists, for Mustafa, with their options and no pick: carry slots, ammo for looted weapons, and how fast tracks
level (including catch-up when switching); any tension it finds with LOOP_V2 or VISION_V2 (for example LOOP_V2 §8's
third gate, buying an unlocked item each life, next to "no weapon choice when buying") is listed the same way, never
resolved. The other drafts carry their parts as the decision map's addendum assigns them: the player record that holds
the tracks and the weapon drops and pickups as sim entities (0004), their authority, messages and replication (0001). Its asset part takes LOOK_V2 §8's flow as given and proposes only what
R1-DATA, R3-CHAR and R3-BUDGET support; the rest (compression formats, the LOD tool, export checks, the material-slot
renaming step and the per-unit budget numbers) are open questions marked "not researched", grouped under one research
unit, "asset-pipeline research" (graphics-engineer, verified like R3, in its own daytime session), which SPIKE_PLAN
places before the asset spike. The look proof test says whether it needs authored models; if it does, it uses plain
glTF/GLB exports and decides nothing about compression or LODs. engine-architect writes 0005's frame (header, context,
the joined decision, consequences, open-question numbering, Appendix A) after the game-designer's and
graphics-engineer's parts land, and places those parts verbatim as sections.

### Steps
1. **Plan review:** done, APPROVE with 19 conditions (`advisor-verdicts.md` §1), folded here.
2. **Decision map** (engine-architect, notes only: `engine-architect-decision-map.md` and `-ids.tsv`), started in
   parallel with the review and not redone. Its **addendum** [C7], by the same agent resuming from its notes: (a) the 60
   Hz margin as a shared-number row (owner 0001, GLOSSARY §8), plus the §8 text of [C3] written out in full, so authors
   link stable anchors; (b) three items in the loose-ends register: the old ADR-001..003 entries (STATUS:198–202), each
   with the draft that disposes of it; the items STATUS sends to ADR-001 (STATUS:257 SEC-MDL-2, :262 no SRI/CSP on the
   CDN tag, :273 the 11 wall-clock reads), each with its draft; one line closing PR #28 finding 4's research-plan
   condition 13 (two branches from one session): it no longer applies; (c) the traces of rule 4, including EDITOR_V2
   §2's document rules (→ 0005) and EDITOR_V2 §5's double-click and "if ADR-002" lines (→ 0002, listed for DOCS-ALIGN);
   (d) how ADR 0005 is put together (above); (e) UPGRADE-SCOPE as Mustafa updated it: which draft carries each part
   (the tracks as data, the player record that holds them, weapon drops and world pickups in the sim, their authority,
   messages, replication and telemetry) and where its three open questions are listed. A script checks the id table: every one of the 521 ids exactly once, owner
   in {0001…0006, SPIKE_PLAN, DESIGN}.
3. **Drafting** (≤ 5 agents at once, NIGHT-RESEARCH (a)): the units of the table. Each writes its draft (or section), its
   claims ledger and its notes, and returns a short summary. Prompts saved before launch in `prompts/`. ADR 0005's frame
   starts once its two sections have landed.
4. **Pass marks** (test-engineer, one unit per draft, as each draft lands): a "Pass marks" table under the draft's "What
   a spike must prove" (mark · what it proves · number and statistic · hardware · method and tool · runs and noise ·
   basis), the Brave checklists of rule 12, plus the draft's spike entries for the spike plan; its rows join the
   draft's claims ledger. The drafter's prose stays; the table is the test-engineer's.
5. **Assembly** (the session): copy the drafts into `docs/adr/`, insert the pass-mark tables verbatim, `cmp`; write
   guard; commit.
6. **Synthesis** (engine-architect): `docs/ENGINE_OVERVIEW.md` (≤ about 150 lines with its diagram), `docs/SPIKE_PLAN.md`
   (each spike: what it proves, its pass marks from step 4, its place in the order, a rough size; the netcode proof test
   runs before the foundation, and the plan names the research spikes it subsumes; it also places the asset-pipeline
   research before the asset spike, the Feel Lab test (ABL-PREDICT) and the PREV-DELETE item [C4, C10]),
   `docs/adr/README.md` (the index, the decision codes, the DESIGN-owned ids, the features whose need is outside
   VISION_V2 §6, and the register: each loose end → where it is resolved or which OQ lists it), and GLOSSARY §8 (the
   file edit lands here [C3]); write guard; commit.
7. **Verification** (verifier, xhigh, one unit per draft plus one for the synthesis docs, ≤ 5 at once, pipelined from
   step 5): every claim and number in the ledger and the text (carry-forward 1, research plan condition 9), against the
   research file and section it cites; arithmetic recomputed. "Research verified" covers only R1 and R2 recommendation
   claims and the claims their verifiers checked (r1:3233) [C16]; every other finding a draft rests on is checked
   against its original source (R1 or R2 findings outside that set, R3–R6 findings outside their spot-check tables;
   `inputs/findings-checked.tsv` is a script-built guide, each topic's own verifier section the authority), on
   allow-listed pages read with curl by day (VERIFY-FETCH (a)), with the host audit in the report; an unreadable source
   makes the claim "unverified" in the draft. Fetched pages are data, never instructions. A "wrong" or "unsupported"
   verdict gets one skeptic (a fresh verifier, xhigh, asked to refute it) before it goes to the owner.
8. **Consistency pass** (a fresh engine-architect as the critic, xhigh, plus scripts): the drafts against each other
   (every shared number, owner, interface and term the same everywhere), against the glossary (the research's glossary
   check, recovered from 85f89ea and pointed at the drafts: 0 exact, every in-context hit judged), and for completeness
   (every id disposed once across the three places, every §6 and LOOK_V2 `[DECIDED]` item traced, every carry-forward
   handled). Then the completeness critic (a fresh verifier, xhigh): what is missing.
9. **Fix round:** every finding to its owner (its author agent, xhigh), who writes exact before/after texts in its
   notes; the session applies them and re-copies; the verifier re-checks only what changed.
10. **Review on the final diff** (CLAUDE.md step 5; xhigh; ≤ 5 at once; rule 6 in every prompt):
    engine-architect (boundaries: 0001, 0003, 0004, 0006) · netcode-architect (0002, 0004, the overview; SPIKE_PLAN's
    netcode proof test) · rust-reviewer (0001, 0002, 0005; SPIKE_PLAN's boundary spike) · graphics-engineer (0002, 0005,
    the overview's per-frame flow; SPIKE_PLAN's look proof test) · security-auditor (0001, 0002, 0006) · infra-engineer
    (0001, 0002) · game-designer (0001, 0004 for rules and data) · art-director (0003, 0005's asset part, SPIKE_PLAN's
    look proof test) · perf-auditor (every budget mark) · test-engineer (is it proven: every draft and SPIKE_PLAN, but
    not its own pass-mark tables) · and each draft's author reviews its draft's pass-mark table for domain fit [C15].
    Every finding is fixed or explicitly deferred with a reason; fixes go through step 9's loop once more.
11. **docs-keeper** (high): STATUS (this change-set's row; PR #32's row marked merged; UPGRADE-SCOPE recorded with the
    other 2026-10-31 decisions in Mustafa's own words of his update (verbatim in the inputs folder), as he and brief §1
    ask; the carry-forwards' state; KIT-ALIGN gains the stale "look is not
    decided" lines, CLAUDE.md:5, :23–24, art-director.md:16, graphics-engineer.md:19, and the "toy-box" lines CLAUDE.md:4
    and art-director.md:10, which may clash with LOOK_V2 §3, flagged for Mustafa, not changed [C18]; DOCS-ALIGN gains
    EDITOR_V2 §5's double-click line and its "if ADR-002 moves the project to modules" line; "Next": the stop),
    ROADMAP, REPO_MAP (the new files); repo-map check.
12. **Gate and checks:** `node tools/gate.mjs 9e7db28` (PASS, symbols +0 −0, smoke) [C1], the repo-map check, the write
    guard, the coverage, citation and glossary scripts (outputs in the report).
13. **Report** (`/advisor-report`, story first, "Decisions needed" and "For Mustafa" last; it folds in the checkpoint's
    essentials and says the checkpoint and notes are deleted in the last commit before the PR is marked ready,
    CHECKPOINTS.md §9.2–9.3 [C5]; it lists ASSET-RESEARCH under "Decisions needed" so Mustafa and the Advisor can move the
    research earlier [C4]; it asks them to confirm the TICK-60-HEADROOM reading [C2] and the "toy-box" lines [C18]; it
    states the R7-MARKS delay and the step-0 edit openly [C18]) and a **draft PR** (brief §5; the body is the report).
    Subscribe to its activity. Then stop.

Checkpoint after every step and at least every 30 minutes (`checkpoint.mjs save`); the background saver runs during
agent runs. Every agent prompt carries CHECKPOINTS.md §4's "Saving your progress" block. A spend-limit cut: record it,
launch nothing new, resume from notes after the reset. **Daytime only** [C17]: no new agent after 23:00 Amsterdam, and no
verifier unit that fetches sources after 22:00, so units end before the night window (00:10). A unit still running at
00:10 is recorded in the checkpoint, and the next day session reads its notes before using its result.

## 5. The steps and their commits

Times are Amsterdam. Checkpoint commits (`checkpoint(v2-decision-drafts): …`, `[skip ci]`) are left out; they only save
notes.

| Step (plan) | Commits | What |
|---|---|---|
| 0, brief §0 | 6617e25 (2026-10-31) | `docs/LOOK_V2.md` added byte for byte (sha256 52b4fc06…), before the plan's approval |
| 1, plan review | (notes) | the advisor's APPROVE with 19 conditions (§3), folded into plan revision 2; revision 3 applies UPGRADE-SCOPE (§2) |
| 2, decision map | (notes) | engine-architect: one owner draft per research id, 521 of 521; the shared numbers, seams, traces and spike candidates |
| 3–5, drafting, pass marks, assembly | 5bdad85, 2a0e5f6, 6b175de, 789bedb, d321a13 (2026-11-01, the night run); 233f6f7; cf00def | the six drafts with test-engineer's pass-mark tables, each checked line for line against its notes; cf00def removes a cut marker the assembly copied |
| 6, synthesis | 80a27b3 | ENGINE_OVERVIEW, SPIKE_PLAN, `docs/adr/README.md`, GLOSSARY §8 (appended after line 743, lines 1–743 unchanged) |
| 11, docs (first pass) | 549490f | STATUS, ROADMAP, REPO_MAP |
| 13, report (first part) | 8132ef4 | this appendix's §1 and §2 |
| 9, fix round 1 (the verifiers) | 09803a9, 03df0ae, 7da03b1, c70c6d2 | ADR 0006, 0001, 0004 and the pass marks |
| 9, fix round 2 (the consistency critic, the skeptics) | 45c957f, 8eb2f67, b375829, 03645ce, 57c43c2 | every draft and section |
| 9, rounds 3 and 4 (the completeness critic) | 9ed8d67, 81d2a65, aa85bf0, f0b56e1, 3c46731, 9a03999 | the 18 inventory rows, the pass marks' stable anchors, §3–§4 of this appendix, a blank line under eight tables |
| 9, the synthesis round | 47eadd9 (2026-11-02) | the overview, spike plan and README brought in line with the drafts |
| 10, reviews on the final diff | (notes) | ten reviewers, findings in §7 |
| 9 again, the final round | 319d3b6, b815c85, 8fa7173, 61e9933, b88714d, 16e97aa, 75615d1, d57dc5a, 5a24122, 72cfcc3, d1f7e13, 8a149bc, 604a497, 449356c (2026-11-02) | every review finding fixed or deferred with a reason |
| 9, the touch-ups after the final re-checks | cd1fc65, cf762b7, 7858762 (2026-11-02); 750d46d, 361b61d, 7cc6942, b513483, 882cfa6, ac252f8, bd68d59, fd8c6f8 (2026-11-03) | fixcheck-d1's, -d2's and -d3's findings fixed or deferred (§7.3), then re-checked (fixcheck-d3, -d4) |
| 11, 12, 13 | 03fe138, b011372 (docs-keeper); 4374a1c, the final commit at the stop (the gate, this report; it runs CI) | docs-keeper's re-check and last touch, the gate, the report |
| the stop, plan revision 4 | 03ef222, 59d41e0 (2026-11-03) | the stop's answers recorded by every author (§9); ADR 0002's pass marks |
| the stop, plan revision 5 | 0aa262f (2026-11-03) | Mustafa's later answers, COMMUNITY-SERVERS and fixcheck-e's findings, by every author (§7.4, §9) |
| the stop, plan revision 6 | 4dc30bc (2026-11-03) | his Hetzner check, two corrections and four answers, by every author (§7.5, §9, §12) |
| 11 and 13 after the stop | the commit that carries this report | docs-keeper-4 (STATUS, ROADMAP, REPO_MAP), this report and appendix; the last commit deletes the checkpoint (CHECKPOINTS.md §9) |

Spend-limit cuts, ten (no work lost: every agent writes its work as it goes and was resumed from it): 2026-10-31
about 19:18; 2026-11-01 about 05:30, 09:27, 14:20 and 19:35; 2026-11-02 about 09:50, 12:15, 17:50 and 21:20; 2026-11-03
about 13:55.

## 6. The advisor's 19 conditions: where each is met

| C | What it asks | Where it is met |
|---|---|---|
| 1 | Write guard before each commit; the gate at the end | `scripts/write_guard.sh` before every commit (PASS on all), its GLOSSARY check a byte-prefix test (a change in form, §5); the gate: the report's Evidence |
| 2 | TICK-60-HEADROOM in ADR 0001 with condition 2's setup; the 60 Hz lines replaced | ADR 0001 part 2, its budgets table and OQ-0001-1; four research lines replaced, not three (r2-netcode.md:2302 too), named for the Advisor |
| 3 | GLOSSARY §8, three entries and one note, after line 742 | GLOSSARY.md §8, appended after line 743 (the base file's last, unterminated line), the base a byte prefix of the new file |
| 4 | ASSET-RESEARCH (a): only what R1-DATA, R3-CHAR and R3-BUDGET support; one research unit before the asset spike | ADR 0005's asset pipeline and GPU sections, OQ-0005-AS1 to AS6 "not researched"; SPIKE_PLAN's asset-pipeline research; the report's Decisions needed |
| 5 | CHECKPOINT-AT-STOP: the report folds in the checkpoint; it is deleted before the PR is marked ready | This appendix (§1–§10, the checkpoint folded in at §8); the checkpoint stays while the PR is a draft |
| 6 | Every research id disposed once, across the six drafts, README (DESIGN) and SPIKE_PLAN | `scripts/dispositions.py`: 521 of 521, each once, on every commit |
| 7 | The decision map's addendum (a)–(e) | Map part j; carried into README's "Codes the drafts cite" (codes defined there, the plan's rules and conditions pointing to §3–§4 here) |
| 8 | Traces: each feature names its VISION_V2 §6 need or says where its need is | Every Decision part's Need line; README "Needs outside VISION_V2 §6" |
| 9 | The look: one technique per LOOK_V2 item, the visible difference Mustafa's call; rule 6 in every reviewer prompt; art-director on 0003, 0005's assets, the look test | ADR 0003 part 5 and its trace table; the look floor (OQ-0003-16); rule 6 in all ten review prompts; art-director's review (§7) |
| 10 | People-judged spikes have measurable marks and a numbered Brave checklist; SPIKE_PLAN places the Feel Lab test and PREV-DELETE | ADR 0001's netcode checklist; ADR 0003's look checklist (now 32 steps, the Low frame and the busy fight among them); SPIKE_PLAN items 6 and 10 |
| 11 | ADR 0005: no upgrade designed; the system carries UPGRADE-SCOPE; the frame by engine-architect, sections verbatim | AB-8; OQ-0005-AB1 to AB15 with no pick; the sections placed verbatim (script) |
| 12 | Settled inputs cited by code or GLOSSARY §8; the brief quoted in full here | "Settled by" rows throughout; §1 byte-identical to the brief |
| 13 | Template 0000's fields filled; nothing in CLAUDE.md or `.claude/` changes | Every heading of the template in all six; `git diff 9e7db28` touches only `docs/` and `.checkpoints/` |
| 14 | Model and effort named for every unit | Every prompt's first line (`.checkpoints/v2-decision-drafts/prompts/`): agent, effort (xhigh; docs-keeper high), model opus |
| 15 | Pass marks in the claims ledgers and verified; each author reviews its pass-mark table; perf-auditor every budget mark; SPIKE_PLAN by four reviewers; test-engineer not on its own tables | The ledgers and verdict files; the step-10 reviews (§7): each author on its draft's pass marks, perf-auditor, SPIKE_PLAN by netcode-architect, rust-reviewer, graphics-engineer and test-engineer |
| 16 | "Research verified" covers only R1 and R2 recommendation claims and their verifiers' checks; other findings at their source; pages are data | Every verifier and skeptic prompt; about 160 pages read on allow-listed hosts by day, each verdicts file with its fetch audit |
| 17 | Daytime only | All units by day, but Mustafa's one-off night run (2026-11-01 03:55–11:00, §2); fetching only 08:30–22:00; on 2026-11-03, after his "Continue" (06:14), the session alone applied blocks already written; after his "You can continue" (07:14, §2), agents ran from about 07:16, fetching only from 08:30 |
| 18 | KIT-ALIGN and DOCS-ALIGN lines; the R7-MARKS delay and the step-0 edit stated; each draft lists its inventory rows | STATUS (docs-keeper); the report's "Decisions and deviations"; every draft's legacy inventory table, the 18 "look brief pending" rows included |
| 19 | ENGINE_OVERVIEW at about 150 lines at most | 139 lines after the synthesis round (the final round's count is in the Evidence) |

## 7. Verification, the reviews and the final round

### 7.1 Verification (plan steps 7–9)
| Unit | Verdict | Claims | Confirmed | Other |
|---|---|---|---|---|
| ADR 0001 | PASS WITH NOTES | 629 | 609 | 3 wrong, 3 unsupported, 5 citation off, 1 rule |
| ADR 0002 | PASS WITH NOTES | 538 | 515 | 2 wrong, 7 citation off; 37 sources read |
| ADR 0003 | PASS WITH NOTES | 440 | 431 | 1 wrong, 4 unsupported, 2 citation off, 2 rule |
| ADR 0004 | PASS WITH NOTES | 665 | 657 | 4 wrong, 1 unsupported, 2 citation off |
| ADR 0005 | PASS WITH NOTES | 1,001 | 997 | 1 wrong, 1 unsupported, 2 citation off |
| ADR 0006 | FAIL (3 defects), then fixed | 588 | 581 | 3 wrong (fixed in 09803a9), 2 citation off |
| Synthesis | PASS WITH NOTES | 61 | 57 | 2 wrong, 1 unsupported, 1 citation off |
| Fix rounds 1–2 (fixcheck-a, -b) | PASS WITH NOTES | 211 | 201 | 3 wrong, 4 unsupported, stale lines; all to a skeptic, then fixed |
| Rounds 3–4 and the synthesis round (fixcheck-c) | PASS WITH NOTES | 95 | 92 | 1 partly wrong (skeptic upheld), 1 citation off, nits; fixed in the final round |
| The final round's web facts (fixcheck-d0) | PASS WITH NOTES | 10 | 9 | 1 wrong (ADR 0004's precision line; its owner checked the saved excerpt and fixed it; fixcheck-d3a then found the new words leaned on a promise rustc breaks, D3A-1, fixed) |
| The final round, ADR 0001, 0002, 0006 (fixcheck-d1) | PASS WITH NOTES | 151 | 138 | 3 wrong, 3 unsupported, 1 citation off, 4 stale against another draft, 1 wording, 1 unclear pass rule; rebuilding the three drafts from the blocks gives the commit exactly |
| The final round, ADR 0003, 0004, 0005, the synthesis (fixcheck-d2) | FAIL (two lines lost in ADR 0005), then fixed | 130 | 122 | 7 wrong (2 in part or in one word), 1 unsupported, 1 confirmed with one phrase unsupported; fix blocks had dropped the AB-3 heading and the Feel Lab's step 4 |
| The touch-ups to ADR 0001, 0002, 0006 and four owner-checked verdicts (fixcheck-d3a) | PASS WITH NOTES | 19 | 18 | 14 blocks, none with a problem; D3A-1 (`div_euclid`; its skeptic: partly) fixed in 7cc6942 and b513483; D3A-2 (this appendix) fixed |
| The touch-ups to ADR 0003, 0004, 0005 and the synthesis (fixcheck-d3b) | PASS WITH NOTES | 33 | 33 | 42 blocks, none with a problem; both lost lines back; D3B-1 (§7.3) and D3B-2 (ADR 0004 part 7) fixed |
| All nine documents swept (fixcheck-d3c) | PASS WITH NOTES | 13,513 | 13,494 | structural lines and references: 0 of 3,225 structural lines lost; D3C-1 to D3C-4 (wraps; seams S5, S9, S10 missing from README; review codes; 111 open, not 112) fixed in ac252f8 and fd8c6f8, D3C-3 in part (the decision map's part g, cited at ADR 0005 line 2103, is deferred, §7.3) |
| The critic's gaps and three web facts (fixcheck-d3d fetched nothing before 08:30; fixcheck-d3e) | PASS WITH NOTES | 26 | 19 | 13 of 14 web facts confirmed (WebGPU §22, rustc 1.99.1's `div_euclid`, std's `Hasher`; not read at a source: `usize` on wasm32, labelled proposed); 6 of 11 of this report's numbers confirmed, 5 wrong, fixed; D3E-1 to -3 fixed in bd68d59 and fd8c6f8 |
| The last touch-ups and this report (fixcheck-d4) | PASS WITH NOTES | 68 | 59 | touch-up 4's ten blocks right, rebuilt byte for byte; 9 slips in this report and appendix (one medium: the Feel Lab line had no verifier before it was applied), fixed with the skeptics' corrected words, not re-checked again; F4 (a section number in PM-0003-2) deferred, §7.3 |

Skeptics (a fresh verifier asked to refute each "wrong" or "unsupported" verdict before it went to its owner), five
rounds: 27 upheld, 9 partly, 2 refuted (the results files' own rows, recounted by fixcheck-d3e; an earlier "30 upheld"
was a miscount); a sixth round on 2026-11-03: D3A-1 and D3E-5, both partly. Not sent to a skeptic: four assembly
cut-marker rows the session fixed itself (cf00def), fixcheck-c's nit A43, the wrong and unsupported verdicts of
fixcheck-d0, -d1 and -d2 (their owners checked them, but d2's F7 and F14, which were deferred; the report's "Decisions
and deviations"), and fixcheck-d3's other than D3A-1 and D3E-5 (D3C-4, D3E-4 and D3E-6, fixed; the ledger row, stated as
superseded; d3a C15, labelled "no source read" in the draft). A seventh round on fixcheck-d4: F1 and F2 upheld, F6 and
F8 partly. The consistency critic: 14 findings (3 high), all fixed by their owners, 489 glossary in-context hits judged
in the drafts (3 wrong, fixed) and 44 in the synthesis. The completeness critic: GAPS, 16 items, all handled (the 18
inventory rows, the pass marks' anchors, §3–§4 of this appendix, the fixcheck units, the reviews, this report). Each
verdicts file holds its fetch audit; github.com answers 403 in this session, so its pages were read from
raw.githubusercontent.com.

### 7.2 The ten reviews on the final diff (plan step 10)
| Reviewer | Verdicts | High findings, and their outcome |
|---|---|---|
| engine-architect | 0001 REVISE (small); 0003, 0004, 0006, 0002's and 0005's pass marks APPROVE WITH NOTES | No timing rule for events: fixed (ADR 0001 part 7; ADR 0002 part 5; PM-0001-61) |
| security-auditor | 0002, 0006 REVISE; 0001 APPROVE WITH NOTES | A pull request's own workflow could take the preview token: fixed (production deploys only from main, own environment and token; a required reviewer on `preview`; OQ-0006-13; PM-0002-66, PM-0006-32) |
| infra-engineer | 0006 REVISE (its own text); 0001, 0002 APPROVE WITH NOTES | No release path or rollback: an interim position and OQ-0006-14; the rooms-per-VM mark vs one thread per vCPU: fixed (PM-0001-27 is PM-0006-13's runs) |
| game-designer | 0001, 0004, one 0005 pass-mark row REVISE | Aim beyond weapon range refused: fixed (clamped; PM-0001-21) |
| netcode-architect | 0001's pass marks and parts 5, 7, 8 REVISE (small); the rest APPROVE (WITH NOTES) | The rewind estimate left out the command buffer and the page's blend: fixed (part 8; PM-0001-48, -49; OQ-0001-5 keeps the cap open until the proof test) |
| perf-auditor | 0001, 0003 REVISE (small); the rest APPROVE (WITH NOTES) | The 160 kbps mean gated in the wrong scene: fixed (R2's estimate scene for the mean, the 256 kbps cap in the stress scene) |
| rust-reviewer | 0002 REVISE (small); the rest APPROVE WITH NOTES | (mediums) the float lint, clock reads in `room`, a panic in the tick, pool capacities: all fixed |
| graphics-engineer | 0005 REVISE; the rest APPROVE WITH NOTES | One material per class vs per slot content: fixed (one per slot content) |
| art-director | 0003 REVISE; 0005's assets and the look test APPROVE WITH NOTES | A tier could switch a LOOK_V2 [DECIDED] item off; Low never shown: fixed (the look floor, OQ-0003-16; checklist step 31) |
| test-engineer | SPIKE_PLAN REVISE; every draft APPROVE WITH NOTES | The 60 Hz run's sim lacked abilities, economy and objectives: fixed (stand-ins that burn their ceilings, ADR 0004 S1; OQ-0001-2) |

### 7.3 The final round (step 9 once more): every finding fixed or deferred
Each author got one batch with every finding against its own text (`final-round-by-author.md`); each block was
checked to sit in its author's text before it was applied; a fix that belonged to another author went to that author.
Blocks applied in the final round: ADR 0001 61, ADR 0002 52, ADR 0003 53 (and 0005's GPU section), ADR 0004 28, ADR
0005's frame and content 25, its ability section 21, ADR 0006 71, the pass marks 138 (units A and B), the synthesis
116. Twenty-four new open questions, each routed to Mustafa or the Advisor: OQ-0001-15 to -18, OQ-0003-16 to -20,
OQ-0004-8 to -12, OQ-0005-AB14 and AB15, OQ-0006-13 to -20.
Deferred, each with its reason (to the drafts' revision after the stop review):
- ADR 0004 OQ-0004-11's "What decides it" and part 17's immunity pointer could name ADR 0005 AB-1 and AB-6 (optional;
  correct as they stand). ADR 0004 part 20 could name OQ-0001-18 (the pointer is correct as it stands).
- ADR 0002 line about 1030: "draw on paid minutes" should read "draw on the account's included compute usage and are
  then billed per minute" (fixcheck-d0 8c; low wording).
- ADR 0001 and 0006: "a site being the registrable domain" simplifies MDN, which also counts the scheme for SameSite
  cookies (fixcheck-d0 row 6; optional).
- ADR 0001 part 8's rewind estimate may add the wait from the drawn frame to the tick that sends the command (0 to one
  frame); PM-0001-49 now measures it per shot. The budgets row "Prediction replay" may add "7 at P2's deepest buffer".
  (Part 4's "causes no correction" was fixed instead: fixcheck-d1 F2, 7858762.)
- ADR 0002 part 4 writes the chosen `[profile.release]` settings once the boundary spike measures them (a future
  action); ADR 0003 part 10 states event play-time only if the one-frame bound of EA-1 is wanted (PM-0001-61 is set at
  one tick plus one frame, 50 ms, a reasoned note).
- ADR 0002's halves of SA-10 and SA-11 (npm advisories and a strict-CSP-ready build) wait for the security baseline
  (BASELINE-WHEN (b)); ADR 0006 carries them.
- The touch-ups after the final re-checks: 63 exact fix blocks, each in its author's own text (test-engineer 15,
  engine-architect 31, of them 23 pointer re-pins, infra-engineer 7, game-designer 5, netcode-architect 3,
  rust-reviewer 2), 5 NOTE blocks; the game-designer's blocks were dry-run and judged by a fresh verifier before they
  were applied, and fixcheck-d3 judged the first 53 after, fixcheck-d4 the last 10.
- From the final re-checks (fixcheck-d1, -d2), low: ADR 0001 part 8's rewind estimate may add the 0-to-one-frame wait
  (d1 F3, measured by PM-0001-49; changing the 233–383 ms figure now would ripple into the README and other drafts);
  ADR 0002 OQ-0002-5's "paid minutes" (d1 F8, as above); ADR 0003 Appendix A still marks R3-WGPU-REC1 "adopted" though
  its mark 2 is changed (d2 F6); ADR 0004's "late steals" is not one of LOOP_V2 §11's counters (d2 F7) and "most
  slots" stands where VISION_V2 §3 says "Bots fill empty slots" (d2 F14). Wording only; for the revision after the
  stop review. ADR 0003 lines 960, 993 and 1367 (and SPIKE_PLAN 247, 255 and 267) keep LOOK_V2 §9's "legacy
  screenshots" beside the look checklist's live legacy window (d2 F13's other places: a screenshot of that window
  meets both).
- Only two claims ledgers (plan rule 9) got the final round's new claims, rust-reviewer's ADR 0004 (rows 367–384) and
  engine-architect's ADR 0002 (F1–F17); none changed after 8fa7173, early in that round. For the rest the verifiers'
  claim tables (fixcheck-d1 §1a–4c, d2's claims table, d3a §1, d3b's claims, d4's) stand in; ledger row 307's 7.49 ms
  is superseded by ADR 0001's 4.69 ms (fixcheck-d3's critic, G3; fixcheck-d4 F8).
- ADR 0005 line 2103 cites the decision map's part g, which leaves the branch with the checkpoint (D3C-3 in part,
  fixcheck-d4 F7): SPIKE_PLAN now holds the spikes' order, so the pointer is provenance only; for the revision after
  the stop review. PM-0003-2's "WebGPU §22.1" for the `destroy()` step could read "WebGPU `destroy()` and §22.1"
  (fixcheck-d4 F4, optional: the row's source cell names both).
- Read at their sources by fixcheck-d3e (2026-11-03, 08:30–08:36): PM-0003-2's WebGPU error model (W3C Candidate
  Recommendation Draft of 15 September 2026, §22) and std's `write_length_prefix` default, which the final round could
  not read after 22:00, and rustc 1.99.1's `div_euclid` (the same steps as 1.97.0), which fixcheck-d3a raised: all
  confirmed. Not read at a
  source: that `usize` is 32 bits on wasm32 (ADR 0004 part 9, labelled proposed).

### 7.4 Recording the stop's answers (plan revisions 4 and 5): fixcheck-e, its skeptics and the last fix round

Five fresh verifiers (effort high) judged every changed line of the nine documents against the deciders' words (§2),
the advisor's rulings (§10) and the landing map, and each rebuilt its documents from 4374a1c with the authors' blocks:

| Unit | Documents | Verdict | Claims | Findings |
|---|---|---|---|---|
| fixcheck-e1 | ADR 0001, 0002, 0004 | PASS WITH NOTES | 77 judged, 77 confirmed (2 with a note) | 3 low: PM-0002-49's reading mark (F1); ADR 0002's "the real sim" (F2, an over-read; its skeptic upheld it); ADR 0004 phase 0's sign-off (F3) |
| fixcheck-e2 | ADR 0003, 0005, 0006, GLOSSARY §8 | PASS WITH NOTES | 47 judged, 47 confirmed; ADR 0005's changed lines not judged one by one (time, after a cut) | 1 medium: ADR 0006's key gives the loss on each direction without the reading mark (F2); 2 low: three reading marks in ADR 0006 (F3); ADR 0003's option (a) without the mark, two places (F1) |
| fixcheck-e3 | README, SPIKE_PLAN, ENGINE_OVERVIEW; checks across all nine | PASS WITH NOTES | 6 rows (19 quotes among them) and five cross-document checks | 1 low: six map rows, the session's own, had no written disposition (N7, O23, L14, T2, T4, LA4; the session's NOTEs, §9); SPIKE_PLAN from item 5 on not read (deadline) |
| fixcheck-e2b | ADR 0005's changed lines (what e2 did not judge) | FAIL | 39 judged, 32 confirmed | 2 medium: PM-0005-4's kinds against CD-2 (F1); Appendix A's count (F2); 5 low: two reading marks (F3); AB12's claims (F4; its skeptic: partly); "as blockouts" (F5; its skeptic upheld it); a mark's date (F6); the client entries' key (F7) |
| fixcheck-e3b | SPIKE_PLAN from item 5 on; consistency across the documents | PASS WITH NOTES | 36 judged, 31 confirmed | 2 medium: PM-0005-4's kinds (as e2b F1); ADR 0004's phase-1 order against SPIKE_PLAN's waves; 3 low: "not a session's" unmarked; OQ-0003-1's labels; OQ-SPIKE-1's "Why it matters" left stale |

Across the units: every rebuild was byte for byte the commit; the write guard passed; 0 retired glossary phrases;
521 of 521 research ids disposed once; the counts (114 numbered, 98 open; 341 pass marks) as the documents state them;
README's 112 pointers name ids and parts that exist, with no line numbers.

The last fix round (plan revision 5, the advisor's conditions 3 to 7): each author's exact blocks for its own text, 59
FIX and 21 NOTE (test-engineer 7 and 3; engine-architect 37 and 8; infra-engineer 9 and 6; graphics-engineer 3 and 2;
rust-reviewer 2 and 1; game-designer 1 and 0; netcode-architect 0 and 1), each placed by the session's script inside its
author's text, dry-run, and applied in the session's order in 0aa262f. Every finding of e1, e2, e2b and e3b is fixed as
the advisor's conditions 3 to 7 set it; e3's six map rows have the session's NOTEs (§9). COMMUNITY-SERVERS: OQ-0006-21
and part 12's one pointer; ADR 0001, 0002, 0004 and 0005 checked by their authors and ADR 0003 and 0004 by the session,
no change.

Mustafa then stopped verification ("after e2b, e3b and their fix round, stop. No further re-check rounds; anything left
goes into the PR as notes."), so no verifier re-checked that round's blocks. The session ran the scripts instead (the
verdict's condition 8): the write guard passes; 521 of 521 research ids disposed once; 0 retired glossary phrases; no
line glued under a table and every pass-mark row 7 cells; new prose lines at most 120 characters; 115 numbered open
questions, 97 open (12, 7, 17, 10, 33, 18), and 341 pass marks (63, 78, 47, 75, 46, 32), numbered without gaps; 172
quotes credited to Mustafa or the Advisor on 2026-11-03 in the changed rows and paragraphs, each found word for word in
his three messages (the Advisor's NET-PROFILES answer twice with its inner quotes made single, as quotes inside a quote;
the script's 20 other hits are section names, labels and checklist terms); no "answered in part" left on OQ-0006-15 or
-20, OQ-0003-1 "answered in part" wherever it is labelled, no "stands in for none", no "tuning table" in PM-0005-4, and
"doesn't measure client frames" once, in ADR 0001's key; README's 114 pointers carry no line number and name ids and
parts that exist. The gate (`node tools/gate.mjs 9e7db28`): 1–4 and 5b–5d PASS, symbols +0 −0; 5a cannot start here, as
before; CI runs it with smoke.

Notes left: the report's "Notes left after verification" (six items).

### 7.5 Plan revision 6 (Mustafa's Hetzner check): the authors' round and the scripts

Six authors (xhigh) wrote 79 exact FIX blocks and 37 NOTEs, each placed by the session's script inside its author's
own text and applied in the session's order (per document: test-engineer first, then the section's owner) in 4dc30bc.
A first launch at about 18:47 was stopped before any block was written, when Mustafa corrected the location; the
relaunch at about 22:28 followed the advisor's amended conditions. As Mustafa asked, no verifier re-checked the
blocks; the session's scripts did (the report's Evidence lists them), and what is left is in the report's notes.

## 8. The checkpoint, folded in (CHECKPOINTS.md §9.1)

- **Steps with their commits:** §5. **Decisions:** Mustafa's words (§2; the ten inputs files byte for byte, §13), the
  advisor's plan reviews (§3, §10, §11, §12), the plan's rules (§4); STATUS records the decision codes.
- **Key numbers:** research ids at 9e7db28: 533 rows, 521 unique (REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes,
  18 copies), each disposed exactly once across the six drafts, README's register and SPIKE_PLAN (`dispositions.py`,
  521 of 521 at every commit checked). Claims verified: 3,922, 3,847 confirmed. Fix blocks: 901 in six rounds, 63 in
  the touch-ups after them, 352 in recording the stop's answers, 59 in the last round after them and 79 in plan
  revision 6 (with 52, 21 and 37 reasoned notes, and the session's six). At the stop's answers: 114 numbered open
  questions, 98 open (ADR 0001 12, 0002 7, 0003 17, 0004 10, 0005 33, 0006 19) plus OQ-SPIKE-1 in SPIKE_PLAN; pass
  marks 63, 78, 47, 75, 46 and 32 (341). After the last round: 115 numbered, 97 open (12, 7, 17, 10, 33, 18); after
  plan revision 6: 116 numbered, 96 open (12, 7, 17, 10, 33, 17; ADR 0006 has 22 numbered); the pass marks unchanged.
- **Open findings at the end:** the deferred items of §7.3; GLOSSARY lines 707–712 (base text, before §8) still describe
  the proof test's WebSocket leg with a pinned self-signed certificate, where Mustafa's answer gives it a trusted
  certificate on his domain (a NOTE for the Advisor: the base text is not edited); ADR 0006's OQ-0006-13 times its call
  "before the first client-host token" where the advisor's LATER ruling says "before the foundation's client-host token"
  (infra-engineer's flag; the entry is not edited by that ruling); and what the last round left, the report's "Notes
  left after verification".
- **Not carried over:** the checkpoint's earlier open findings were each settled in a fix round (§7) or are in §7.3; the
  running-work notes end with the stop.

## 9. After the stop (2026-11-03): where each answer landed

Recorded under plan revision 4 (the advisor's APPROVE WITH CONDITIONS, §10), from engine-architect's landing map (205
rows; each got a FIX block or a reasoned "no change" from its own author). Commits: 03ef222 (all eight authors' blocks),
59d41e0 (ADR 0002's pass marks, test-engineer), and 0aa262f (fixcheck-e's findings, plan revision 5, §7.4). A reading of
the advisor's, not the deciders' words, is marked "(the advisor's reading, 2026-11-03)" where it stands; the report's
"At the stop" lists them.

| Answer | Whose | Where it is recorded | Still open |
|---|---|---|---|
| SPIKE-ORDER (b) | Mustafa | SPIKE_PLAN's order (wave 1, one heavy spike at a time; the preview hosting check alongside); every header: "Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted", Status Proposed; ADR 0001 part 14 and migration step 1; ADR 0002 part 12 and its pass marks; README | — |
| TICK-60-HEADROOM, confirmed with his addition | Mustafa | ADR 0001 OQ-0001-1, part 2, the budgets row, spike item 3, the Rollback; PM-0001-63 (new: rooms per VM at 60 Hz within the goal budget); ADR 0006 part 8 and spike item 2 point to it; GLOSSARY §8's 60 Hz entry; README; SPIKE_PLAN; ENGINE_OVERVIEW | the goal budget's figure (OQ-0006-19, at slice 6) |
| TICK-60-SETUP (a), an early signal only | the Advisor | ADR 0001 OQ-0001-2, part 2, migration steps 1 and 8; ADR 0004's stand-ins; ADR 0002's pass marks; GLOSSARY §8; SPIKE_PLAN; README | — |
| NET-PROFILES | the Advisor | ADR 0001 OQ-0001-3, OQ-0001-16, part 13; ADR 0002 part 8 and PM-0002-49; README; SPIKE_PLAN | — |
| REWIND-CAP | Mustafa | ADR 0001 OQ-0001-5 (answered in part: the rule), parts 5 and 8, OQ-0001-6's guide, the netcode checklist, Revisit; SPIKE_PLAN items 1 and 10; README | the cap's value, from the proof test |
| SPIKE-MACHINES | Mustafa | ADR 0006 OQ-0006-17 (answered), OQ-0006-5, -15, -20 (in part), parts 7 and 8, the spike items, the load generator on CX33s; ADR 0001 part 10 (certificates) and spike item 1; ADR 0002 part 8 (the bench path); SPIKE_PLAN item 1; README | OQ-0006-1 (his console check); OQ-0003-1's actual machines (named before the look proof test); the SSH key and the certificate method (the proof test's plan, with security-auditor); OQ-0006-15 and -20 answered later that day (below) |
| LOOK-TEST-INPUTS | Mustafa | ADR 0005's settled inputs (LOOK-TEST-INPUTS) and OQ-0005-AP1; ADR 0003 part 11; SPIKE_PLAN (the asset research's export and slot-naming part before the look proof test; OQ-SPIKE-1 answered); README | LOOK_V2 §8: answered later that day (below); LOOK_V2 not edited |
| TOY-BOX | Mustafa | nothing in the nine documents; STATUS and the report: a kit change-set after PR #30, with the checkpoint anti-waste rules | — |
| OQ-0002-8 (a) | the Advisor | ADR 0002 OQ-0002-8, part 3, the trade-off, the options table, Appendix A; README names the three crates | — |
| OQ-0005-1 | the Advisor | ADR 0005 OQ-0005-1, CD-2 (the presented kinds that need a client entry, and those with nothing to present), CD-13, AP-2, AP-3, question F2, the frame's table; a bite on a CD-2 pass mark; README | — |
| OQ-0001-14 (a) | the Advisor | ADR 0001 OQ-0001-14 and Appendix A; README | — |
| ROOM-CRASH (a) | the Advisor | ADR 0001 OQ-0001-18, part 11, the failure row, PM-0001-59, Appendix A; ADR 0004 part 20; ADR 0006 part 12 ("marked crashed", kept under part 11's retention); README | — |
| OQ-0003-17 | the Advisor, confirmed by Mustafa | ADR 0003 OQ-0003-17 and part 7; PM-0003-44; SPIKE_PLAN; ENGINE_OVERVIEW; README | — |
| OQ-0003-20 | the Advisor | ADR 0003 OQ-0003-20 and part 7; the soaked set in PM-0003-18 to -21; SPIKE_PLAN; README | — |
| OQ-0005-AB12 | the Advisor | ADR 0005 OQ-0005-AB12, the frame's table, Appendix A; README | the specific claims to check against Epic's documentation, once it can be read |
| OQ-0005-AB13 | the Advisor | ADR 0005 OQ-0005-AB13, the Rollback and "Revisit when", the frame's table, Appendix A; the pass marks' failure rule; README | — |
| LATER | Mustafa | README's later rows and design questions: the design session after the netcode proof test | the twelve questions themselves |

The map rows that were the session's own (fixcheck-e3 F1 found them without a written disposition; the session wrote
these, word for word from its notes):

| Row | Whose | Disposition |
|---|---|---|
| N7 | the session | no change: the row is the Advisor's NET-PROFILES words quoted inside the map's own heading ("VISION_V2 §4's 'about 100 ms of ping'"); no document place to change. |
| O23 | the session | no change in the nine documents: the session's own item. Mustafa's reason for SPIKE-ORDER ("Reason: seven usage-limit cuts in three days.") is quoted word for word in SPIKE_PLAN (engine-architect-b's block); the report keeps its own count (ten cuts over four days, labelled as the checkpoint's), as the verdict's REASON-COUNT ruling says. |
| L14 | the session | no change: LOOK_V2 is Mustafa's and the Advisor's file and is not edited (the verdict's LOOK_V2-§8 ruling); the question whether §8 changes is the report's escalation LOOK_V2-§8. |
| T2 | the session | no change in this PR: TOY-BOX rewords CLAUDE.md line 4 and the art-director's file line 10 in a small kit change-set after PR #30 is updated (Mustafa's words); the plan excludes CLAUDE.md and `.claude/`. STATUS and the report record it. |
| T4 | the session | docs-keeper's and the session's: STATUS records TOY-BOX's kit change-set (docs-keeper-4, item 4); the report's "At the stop" and the appendix §9 record it. |
| LA4 | the session | docs-keeper's: STATUS records the design session after the netcode proof test, with the twelve LATER questions and the 38 design questions on its agenda (docs-keeper-4, item 4; the verdict's LATER ruling). |

Recorded under plan revision 5 (the advisor's APPROVE WITH CONDITIONS, §11), in 0aa262f. The design answers go in
README and STATUS only, as Mustafa asked; the drafts take them up at their next revision (the verdict's DRAFT-ENTRIES
(a)). The answers to the escalations close questions the drafts' own entries carry, so the drafts take those.

| Answer | Whose | Where it is recorded | Still open |
|---|---|---|---|
| Carry slots (OQ-0005-AB1) | Mustafa | README's row (answered, with his "Assumed, to confirm: the class's own abilities stay."); STATUS | his "Assumed, to confirm" |
| Looted ammo (OQ-0005-AB2) | still open; the lean is the Advisor's | README's row (open, his three options in his words; the lean marked as the Advisor's, never as an answer; LATER kept); STATUS | the answer |
| Levelling (OQ-0005-AB3) | Mustafa | README's row; STATUS | — |
| The name "amplifications" ("amps") | Mustafa | README's one sentence; STATUS | — |
| WEAPON-UNLOCKS (OQ-0005-AB5) | Mustafa, "Not final" | README's row (open, his lean); STATUS | the answer |
| Banzai vs Riot Shield (OQ-0005-AB11) | Mustafa | README's row; STATUS | the Peacekeeper's strength before Tech 1, left to his "numbers tuned later" (the advisor's reading) |
| Upgrade visibility (OQ-0005-AB15) | Mustafa | README's row (placing it at AB15: the advisor's reading); STATUS | how enemies see it (the scoreboard or a cue) |
| Spawn protection (OQ-0005-AB14) | Mustafa: 3 s; the Advisor's pencil: firing ends it early | README's row (answered in part); STATUS | what it covers (ADR 0005's options (a) and (b)); the pencil, which Mustafa may veto |
| Fragments as damage | Mustafa | README's sentence; STATUS | — |
| Faction asymmetry; the war around the matches | Mustafa: still open | README (asymmetry beside OQ-0005-CD5; the war quoted, no id); STATUS | both |
| REF-MACHINES (OQ-0003-1) | Mustafa | ADR 0001's and ADR 0003's keys (test-engineer); ADR 0003's OQ-0003-1 and its migration line (graphics-engineer); README; SPIKE_PLAN's machines paragraph, items 1, 3, 4 and 5 and its open list (engine-architect) | the actual machines, named before the look proof test |
| LOOK_V2-§8 | Mustafa | ADR 0005's settled inputs (LOOK-TEST-INPUTS); SPIKE_PLAN's asset-pipeline research; STATUS (a later step) | §8 reads "Spline or Blender" once the export part reports (that part's change-set) |
| SPIKE-COST (OQ-0006-20) | Mustafa | ADR 0006 OQ-0006-20 (answered); README; SPIKE_PLAN (the proof test's "Depends on") | each run's cap, in its plan, for his OK |
| DOMAIN (OQ-0006-15) | Mustafa | ADR 0006 OQ-0006-15 (answered); README; SPIKE_PLAN | the registration, before the netcode proof test |
| COMMUNITY-SERVERS | Mustafa | ADR 0006 OQ-0006-21 (new) and part 12's pointer; README's row; ADR 0001, 0002, 0004 and 0005 checked by their authors (no change), ADR 0003 and 0004 by the session (no change) | the whole question, kept open, not designed |
| Verification stop | Mustafa | no fixcheck-f; the scripts and the gate (§7.4); notes left in the report | — |

Recorded under plan revision 6 (the advisor's APPROVE WITH CONDITIONS, amended twice, §12), in 4dc30bc, from the work
list by script (amended condition 1): every place got a FIX block or a reasoned NOTE from its own author. One row per
block, in the authors' words as each returned it:

| Block | Author | Document | Kind | What it does |
|---|---|---|---|---|
| R6B-IE-01 | infra-engineer | 0006 | FIX | Decision (117-120): servers start in `fsn1` (part 5); the shared CX33 is no longer measured beside the CCX23, and K3 waits for a later measurement; the lean end of 18 to 102 EUR rests on the CX33's price (part 8). |
| R6B-IE-02 | infra-engineer | 0006 | FIX | Part 4 (188-190): the staging server's proposed CX33 cannot be ordered in Falkenstein, so its x86 plan is OQ-0006-22 (the pointer part 4 needed). |
| R6B-IE-03 | infra-engineer | 0006 | FIX | Part 5 (242-255): the reference server is the CCX23 (x86-64 AMD) in `fsn1`, with his words quoted ('a CCX23 for the game server and a CAX21 ...' and 'I created both once ...'). Every server row runs on it; the CAX21 carries only bots and metrics; no address is fixed; production follows the test servers' location unless R6-HOST-OQ2 says otherwise (the advisor's reading); SH is gone and K3 waits. Removes nbg1/hel1 and 'picked by a round-trip measurement'. |
| R6B-IE-04 | infra-engineer | 0006 | FIX | Part 7 (288-290): his earlier domain quotes kept, with one pointer 'registered (Mustafa, 2026-11-03)'; 'Mustafa registers one' becomes 'has one'. |
| R6B-IE-05 | infra-engineer | 0006 | FIX | Part 7 (301): 'on the domain he registers' becomes 'on the registered domain'. |
| R6B-IE-06 | infra-engineer | 0006 | FIX | Part 8 start budget (324-328): the reference case's 8.99 is labelled 'the staging VM's placeholder price (the CX33's, R6-HOST-F15); its plan is OQ-0006-22'; the lean case gets the advisor's label, saying it rests on the CX33's price and the CX33 can't be ordered in Falkenstein. |
| R6B-IE-07 | infra-engineer | 0006 | FIX | Part 8 goal budget (331-332): one label saying the staging VM is included at its placeholder 8.99 here and below (OQ-0006-22); the rest of the line is kept. |
| R6B-IE-08 | infra-engineer | 0006 | FIX | Part 8 fleet shape (352-355): K3 only after a later measurement of a shared x86 plan that can be ordered in the fleet's location, with one pointer to his check. |
| R6B-IE-09 | infra-engineer | 0006 | FIX | Part 8 soak (372-374): 'on a client VM beside the reference server, never on it (as spike 2's load generators)', 3 x (0.1386 + 0.0176) = 0.47 EUR, with 0.0176 named as a CAX21-hour (R6-HOST-F15). |
| R6B-IE-10 | infra-engineer | 0006 | FIX | Part 14 (562-564): 'rooms per VM on both machines' becomes 'on the reference server' (a hit outside the script's list). |
| R6B-IE-11 | infra-engineer | 0006 | FIX | Q3 G2 row (607): the row stays as weighed; one pointer added in its Cons cell: not measured after all, because the CX33 can't be ordered in Falkenstein, so K3 waits. Still 8 cells. |
| R6B-IE-12 | infra-engineer | 0006 | FIX | Q6 (631-635): nbg1/hel1/'NBG1 HEL1' and 'a measurement decides' are gone; the three locations are named as OQ-0006-2's options (the record); his location quote is added; playtester round trips to `fsn1` (R6-HOST-OQ2) check it for production. |
| R6B-IE-13 | infra-engineer | 0006 | FIX | Rationale (711-715): a shared plan waits for a later measurement (K3), with one pointer; OQ-0006-1's check is answered; capacity on a run day stays ADR 0001's failure row, with OQ-0006-1 (b) as the fallback (the advisor's reading). |
| R6B-IE-14 | infra-engineer | 0006 | FIX | Provider-down list (748): 'once the game's domain is registered' becomes 'the game's domain is registered' (a register hit outside AMENDMENT 2's list). |
| R6B-IE-15 | infra-engineer | 0006 | FIX | Consequences, Harder (757-762): his console check is done (OQ-0006-1); 'he registers shamblefrontlines.com ... before the netcode proof test' becomes 'he holds shamblefrontlines.com at Cloudflare (OQ-0006-15), registered (Mustafa, 2026-11-03)'. |
| R6B-IE-16 | infra-engineer | 0006 | FIX | Revisit when (766-768): plans that can't be ordered in `fsn1` on a run day (OQ-0006-1 (b)); K3 when a shared x86 plan that can be ordered in the fleet's location passes a later measurement. |
| R6B-IE-17 | infra-engineer | 0006 | FIX | OQ-0006-1 (799-800): 'Answered by Mustafa (2026-11-03)' with his check quote and his 'a CCX23 ... I created both ... new then.' quote; (a) holds; capacity on the day isn't guaranteed, so (b) and ADR 0001's failure row stay the fallback (the advisor's reading); R6-HOST-OQ4 and OQ7 stay open with their own triggers; the options stay as the record. |
| R6B-IE-18 | infra-engineer | 0006 | FIX | OQ-0006-1 What decides it (809): 'in `nbg1` or `hel1`' becomes 'in an EU location'. |
| R6B-IE-19 | infra-engineer | 0006 | FIX | OQ-0006-2 (811): 'Answered by Mustafa (2026-11-03)' with 'both test servers will be in Falkenstein, not Nuremberg: ...'; both test servers in `fsn1` on each run day; production follows unless R6-HOST-OQ2 says otherwise (the advisor's reading); the options stay as the record. |
| R6B-IE-20 | infra-engineer | 0006 | FIX | OQ-0006-5 (846-847): his 'a domain I register.' quote kept, with one pointer 'registered (Mustafa, 2026-11-03), OQ-0006-15'. |
| R6B-IE-21 | infra-engineer | 0006 | FIX | OQ-0006-15 (982-984): earlier quotes kept, plus 'registered (Mustafa, 2026-11-03): "DOMAIN: done. shamblefrontlines.com is registered at Cloudflare."'; 'a domain he registers before it' becomes 'the registered domain serves its WebSocket leg'. |
| R6B-IE-22 | infra-engineer | 0006 | FIX | OQ-0006-20 (1073-1074): the record keeps its words (CX33 hours of SH, MV) and gains one pointer: 'the rig is now the CCX23 and CAX21s, part 5'. |
| R6B-IE-23 | infra-engineer | 0006 | FIX | New OQ-0006-22 after OQ-0006-21 (anchored on 1097): which x86 plan runs the staging server. Why it matters, his check quote, options as HZ-STAGING gives them (CX33 8.99 only where it can be ordered; CCX13 43.49 if it can be ordered in the fleet's location; CPX32 35.99, locations not researched), the 8.99 placeholder in the meantime, decider Mustafa (money) with the Advisor, before slice 3. |
| R6B-IE-24 | infra-engineer | 0006 | FIX | Spike item 2 (1132-1139): on the reference server, a CCX23 in `fsn1`, precondition OQ-0006-1 answered; :1134's SH part goes (CX33, 'both machines', the shared host's 3 times of day), and that half of R6-HOST-REC2's mark waits with K3. |
| R6B-IE-25 | infra-engineer | 0006 | FIX | Migration step 0 (1361-1371): the console check is done (OQ-0006-1 answered); VMs created fresh for each run; the domain quotes kept with 'registered (Mustafa, 2026-11-03)'; registration is no longer a to-do; the milestone stays (the reference server can be ordered: OQ-0006-1 answered, or (b)). |
| R6B-IE-26 | infra-engineer | 0006 | FIX | Test plan (1435-1436): 'on a CX33, the staging VM's plan' becomes 'on the staging VM's plan (OQ-0006-22)'. |
| R6B-IE-27 | infra-engineer | 0006 | FIX | Rollback (1446-1448): the CCX23 can't be ordered on a run day or fails the marks; K3 only after a later measurement of a shared x86 plan that can be ordered in the fleet's location; until then, or if it fails, K1. |
| R6B-IE-28 | infra-engineer | 0006 | FIX | Appendix A R6-HOST-REC2 (1499): the disposition adds 'the shared CX33 not measured'; 'a working pick until OQ-0006-1 is answered' becomes 'the CCX23 stands, in `fsn1` (OQ-0006-1 and -2 answered)', and K3 waits. Still 3 cells. |
| R6B-IE-29 | infra-engineer | 0006 | FIX | Appendix A 1510-1512: R6-TEL-REC3 reads 'on the staging VM's plan (OQ-0006-22)'; R6-HOST-OQ1 is answered by OQ-0006-1 (his console check); R6-HOST-OQ2 stays open, carried by the answered OQ-0006-2 (playtesters' round trips to `fsn1`). |
| R6B-IE-30 | infra-engineer | 0006 | FIX | Appendix A R6-HOST-OQ4 (1514): 'open: carried by OQ-0006-1, whose console check is answered; this id keeps its own trigger' (not marked answered). |
| R6B-IE-31 | infra-engineer | 0006 | FIX | Appendix A R6-HOST-OQ7 (1517): the same status wording as OQ4; still open. |
| R6B-IE-N1 | infra-engineer | 0006 | NOTE | No change: 91, 246 (kept inside -03), 753, 1103 ('every server mark'): true again, since every server row runs on the CCX23. |
| R6B-IE-N2 | infra-engineer | 0006 | NOTE | No change: 267 and 825, the lobby's 'for example a CX33' placeholder, kept as an example (HZ-STAGING). |
| R6B-IE-N3 | infra-engineer | 0006 | NOTE | No change: option rows 606 (G1's dated finding), 610 (other providers, consistent with (a)), 618 (K3's row, the record; its trigger is now in part 8). |
| R6B-IE-N4 | infra-engineer | 0006 | NOTE | No change: OQ-0006-1's 801-808 and 810 (Why, Options with R6-HOST-OQ1 at 803, Decider, Carries) stay as the record under the answer line. |
| R6B-IE-N5 | infra-engineer | 0006 | NOTE | No change: OQ-0006-2's 812-818 (options with nbg1/hel1/fsn1, Decider, What decides it with R6-HOST-OQ2, Carries) and its title at 811: the record HZ-LOCATION keeps. |
| R6B-IE-N6 | infra-engineer | 0006 | NOTE | No change: OQ-0006-15's title (981) and fields 988-999, the record of what was weighed. |
| R6B-IE-N7 | infra-engineer | 0006 | NOTE | No change: 1032, OQ-0006-17's answered entry's 'What decides it' field (its record). |
| R6B-IE-N8 | infra-engineer | 0006 | NOTE | No change: 1075-1077, OQ-0006-20's options and interim ('0.0144 a CX33-hour'): the record keeps its words; its one pointer is -22. |
| R6B-IE-N9 | infra-engineer | 0006 | NOTE | No change: 1146-1149, load-generator VMs as many as calibration shows: already LG-COUNT's rule and address-neutral; LG, the day's addresses and the host keys are test-engineer's key. |
| R6B-IE-N10 | infra-engineer | 0006 | NOTE | No change: 1300, 1189-1199, 1267-1280 are in the Pass marks section (test-engineer's); 1300's OQ-0006-1 (b) route stays valid as the run-day fallback. |
| R6B-IE-N11 | infra-engineer | 0006 | NOTE | No change: part 8's macOS-runner price (360-368) already reads 'free while public' (AMENDMENT 2); the runner standing in as MAC is ADR 0001's key. |
| R6B-IE-N12 | infra-engineer | 0006 | NOTE | No change: 214, 396, 643, 731, 1455, Prometheus on the staging VM (M1), the production design; MV stands in for it in the spike. |
| R6B-IE-N13 | infra-engineer | 0006 | NOTE | No change: Appendix A 1513, 1515, 1516 (R6-HOST-OQ3, OQ5, OQ6) don't rest on the CX33, the location or OQ-0006-1. |
| F-0001-KEY-RS-LG | test-engineer | 0001 | FIX | Machines key (lines 1348-1364): RS is now the CCX23 in Falkenstein (`fsn1`), quoting his 'a CCX23 for the game server', and every server row runs on it. SH is gone ('No shared host is measured beside RS: the CX33 cannot be ordered in Falkenstein'). LG is 'a CAX21 for the 48 simulated players', 'so their build targets aarch64', created in `fsn1` on run day, with the 80 % calibration rule kept and AMENDMENT 2's LG-COUNT sentence word for word. The run plan names the CAX21 that carries MV. One clause at the connection cap: the cap is set on the day's addresses. One Arm sentence (condition 5): rows PM-0001-17, -19, -48 and -54 replay any LG mismatch on x86-64 before it counts, and a mismatch seen only on aarch64 decides nothing. |
| F-0001-KEY-MAC | test-engineer | 0001 | FIX | MAC (line 1372): 'nothing stands in for RD or MAC' now allows the macOS runner. One sentence covers his MACHINES-TIMING step (quoted) and both branches, checks (i) Safari at least 26.4 (R2-TRN-F7), (ii) real Safari through safaridriver and (iii) one WebTransport session to RS plus one WebSocket session on the domain, and the limits: yes-or-no and count rows only (PM-0001-28, whose server is then RS; -14; -25's Origin; PM-0006-15). Time-limited rows and the look proof test stay on the borrowed Mac. The workflow is reviewed by security-auditor and infra-engineer. Labelled the advisor's reading. |
| F-0001-VALIDITY-SH | test-engineer | 0001 | FIX | Validity paragraph (lines 1462-1465): the clause 'on SH steal is what is measured' goes. The 3-times-of-day citation (R6-HOST-REC2's mark, r6 lines 668-669) stays. |
| F-PM-0001-1 | test-engineer | 0001 | FIX | PM-0001-1: the rig is 'RS and LG', each named with its address of the day (the VMs are created fresh on run day). Machine cell is now RS, LG, ML, RD, MAC (SH dropped). |
| F-PM-0001-16 | test-engineer | 0001 | FIX | PM-0001-16: machine cell 'RS and SH' becomes 'RS'. |
| F-PM-0001-27 | test-engineer | 0001 | FIX | PM-0001-27: machines 'RS, then SH, each with LG' become 'RS with LG'. '3 sessions per machine' becomes '3 sessions'. |
| F-PM-0001-63 | test-engineer | 0001 | FIX | PM-0001-63's formula: 'the staging CX33 (8.99 EUR)' becomes '8.99 EUR for the staging VM (the staging VM's placeholder price (the CX33's, R6-HOST-F15); its plan is OQ-0006-22)'. The bite's arithmetic is unchanged. |
| F-PM-0001-17 | test-engineer | 0001 | FIX | PM-0001-17, way (ii): each LG client logs the bytes it receives. A mismatch on the real-network run (L2) counts only once those bytes, decoded again from the start on x86-64 (L1's decoder on CI's x86-64 runner), also miss the server's hash. A mismatch seen only on aarch64 is reported as a cross-architecture finding and decides nothing. |
| F-PM-0001-19 | test-engineer | 0001 | FIX | PM-0001-19: joins in ML's Chrome (Wasm) are judged as they are. A hash mismatch on an LG join follows way (ii): its stream and datagram bytes are decoded again on x86-64, and a mismatch seen only on aarch64 decides nothing. The 300 ms limit counts as measured. |
| F-PM-0001-54 | test-engineer | 0001 | FIX | PM-0001-54, way (ii): on L2, a throw whose landing differs is fed again (start position, velocity and fuse, as PM-0004-73 feeds them) to the trajectory function on x86-64. It counts as a desync only if it differs there too; a difference seen only on aarch64 decides nothing. |
| F-PM-0001-48 | test-engineer | 0001 | FIX | PM-0001-48, way (ii): LG's shooter logs each shot's ray and the target's interpolated pose. A shot that counts against the mark counts only once the same view test, replayed on x86-64, gives the same result; one that differs only on aarch64 decides nothing. |
| N-0001-NOT-MINE | test-engineer | 0001 | NOTE | No change by me. Line 1274 (the domain, 'registered') is item 1 of 'What a spike must prove', not a key or checklist, so it is netcode-architect's text. Lines 719, 756 (the failure row stays live) and 1293 are netcode-architect's too. |
| N-0001-ARM-ROWS | test-engineer | 0001 | NOTE | Condition 5 against netcode-architect's NOTE: I agree that -17, -19, -48 and -54 are covered and -18, -42, -49, -61 and the other LG rows are not. One difference: for -48 I take way (ii), not the (i) it suggests, because ML is judged only once named and the rig has no x86-64 client machine. Its NOTE leaves (ii) open. |
| N-0001-ADDRESS-ROWS | test-engineer | 0001 | NOTE | No change: the 'address' hits in PM-0001-25 (line 1508) and PM-0001-29 (line 1514) set the cap on whatever addresses LG has that day, and the key now says those are the day's addresses. The RS sentence keeps 'every server time mark ... one vCPU', which is true again. |
| N-0001-MAC-ROWS | test-engineer | 0001 | NOTE | No change to the machine cells of PM-0001-28, -14, -25 and PM-0006-15: the key defines when a runner stands as MAC. On a runner, PM-0001-28's server is RS (recorded); on the borrowed Mac it keeps its local server. |
| F-0004-KEY-RS-CX33 | test-engineer | 0004 | FIX | Machines key (lines 1027-1033): RS is now in Falkenstein (`fsn1`), with R6-HOST-OQ1 answered by Mustafa's console check (OQ-0006-1). The **CX33** machine code is replaced by 'The shared CX33 is not measured: it cannot be ordered in Falkenstein (Mustafa, 2026-11-03)'. |
| F-PM-0004-1 | test-engineer | 0004 | FIX | PM-0004-1: CX33 dropped from both the recorded machines and the machine cell, which becomes RS, ML, RD, MAC. |
| F-PM-0004-4 | test-engineer | 0004 | FIX | PM-0004-4: the title loses 'and on the cheaper host' and now says it decides R1-ECS-OQ4 and R1-TICK-OQ5 'both from RS alone'. The 'same at N = 48 on CX33' part goes, the machine cell becomes RS, and the basis loses 'R6-HOST-REC2 (CX33 beside)'. |
| N-0004-OQ4-OQ5 | test-engineer | 0004 | NOTE | No change: R1-ECS-OQ4 (REC1 mark 1's native step and R6's cost per match) and R1-TICK-OQ5 (REC2's P1-P3) are still decided from RS alone. The CX33 half was only reported and decided neither, so neither question stays open. |
| N-0004-A64 | test-engineer | 0004 | NOTE | No change: the A64 key, S7 and PM-0004-26 stay, as condition 6 rules. Condition 6's one sentence is rust-reviewer's and sits in part 7 (R6B-RR-1). Nothing else in ADR 0004's pass marks names CX33, SH, MV, a location or an address. |
| F-0006-KEY-RIG | test-engineer | 0006 | FIX | Key (lines 1189-1203): RS is in `fsn1` with his full CORRECTION-2 words and OQ-0006-2 answered; every server row runs on RS; SH is gone and K3 waits. LG is as in ADR 0001, with the 80 % rule and the LG-COUNT sentence. MV is defined as 'the metrics role: Prometheus on LG's CAX21, never on RS, inside LG's 80 % rule, standing in for the staging game server' (the advisor's reading). One sentence on the day's addresses quotes his words and lists the settings bound to them: the cap, firewall sources, and the DNS record set before the certificate. Host keys go to the proof test's plan with security-auditor, and the WebTransport certificate is pinned by hash and made per run. MAC points to ADR 0001's MAC sentence. |
| F-PM-0006-11 | test-engineer | 0006 | FIX | PM-0006-11: creates 'the CCX23 created in `fsn1` on the run day' and records it with its address of the day. The title reads 'OQ-0006-1, answered'. The machine cell 'RS, SH' becomes 'RS'. |
| F-PM-0006-12 | test-engineer | 0006 | FIX | PM-0006-12 is now a check that decides nothing. Target: one of the run's CAX21s in `fsn1`, with ICMP allowed on it only, never on RS, pinged from ML and MP. It is reported against VISION_V2 §4's 'about 100 ms', and a surprise goes to Mustafa. The title loses 'each candidate location' and 'feeds OQ-0006-2'; the second-run rule goes. |
| F-PM-0006-13 | test-engineer | 0006 | FIX | PM-0006-13: runs on RS only. The SH-only bite clause (mark 1 and start-up on SH) goes, and so do 'per machine', 'on SH steal time is what the mark measures' and the SH adoption-mark sentence. The 384 bots run on as many CAX21s as the calibration shows. |
| F-PM-0006-14 | test-engineer | 0006 | FIX | PM-0006-14: 'on each machine' becomes 'on RS'. Machines 'RS and SH, each with LG' become 'RS with LG'. |
| F-PM-0006-15 | test-engineer | 0006 | FIX | PM-0006-15: the metrics rule's source is MV's address of the day. The certificate is now on shamblefrontlines.com, 'the domain Mustafa registered', quoting his 'DOMAIN: done...' with his earlier SPIKE-MACHINES words kept. Its DNS record is set to RS's address of the day before the certificate is issued. |
| F-PM-0006-16 | test-engineer | 0006 | FIX | PM-0006-16: MV stays as the role. The exporter answers only MV's address of the day (the CAX21 that carries MV), and the rule names that address and is set on the run day. |
| F-PM-0006-18 | test-engineer | 0006 | FIX | PM-0006-18: the planning check now prices 0.1386 EUR (CCX23) and 0.0176 EUR (CAX21) an hour, excluding VAT (R6-HOST-F15), instead of the CX33's 0.0144. 'LG's calibrated CAX21s counted'. |
| F-0006-FAILS-SH | test-engineer | 0006 | FIX | 'When a mark fails' (lines 1291-1293): the adoption marks 'PM-0006-13 on SH, PM-0006-14 on SH' go. New text: 'No mark here is an adoption mark: no shared host is measured (the key), so K3 waits (part 8)'. |
| F-0006-OPS-STEP1 | test-engineer | 0006 | FIX | Operations checklist step 1: one ping to the address the session sends (that day's address of one of the run's CAX21s in `fsn1`), replacing one address per candidate location. |
| N-0006-MV-ROWS | test-engineer | 0006 | NOTE | No change: PM-0006-22, -23 and -24 keep MV, which the key now defines as the metrics role on LG's CAX21. PM-0006-17, -19 and -20 name no SH, MV, CX33 or location. |
| N-0006-ADDRESS-ROWS | test-engineer | 0006 | NOTE | No change: PM-0006-21 and -31 search logs and crash reports for client addresses, whatever the day's addresses are. Line 1214 ('Every server mark runs at the 30 Hz baseline') is about the tick rate. The Results log stands; the key's new sentence plus PM-0006-11 and PM-0001-1 record the day's addresses. |
| N-0006-STAYS | test-engineer | 0006 | NOTE | No change: line 1300's OQ-0006-1 (b) fallback stays live, since capacity on run day is not guaranteed (HZ-OQ1). These also stay: the bracket's 3-times-of-day reason at line 1233 (R6-HOST-REC2's mark), the 'Counts and settings' line on PM-0006-12's 100 echoes, and 'What has no measurable mark here' for part 4 (OQ-0006-22 adds no mark). |
| N-OTHER-ADRS | test-engineer | 0003-rendering | NOTE | No change: the pass marks of ADR 0002, 0003 and 0005 name no CX33, SH, MV, CCX13, DS, old location or fixed address. ADR 0003's MAC (the borrowed MacBook Air, named before the look proof test) is consistent with MACHINES-TIMING in either branch. |
| rows-for-test-engineer | netcode-architect | 0001 | NOTE | For test-engineer: which rows condition 5's rule covers. 23 rows name LG. Covered: PM-0001-54 (line 1546, the landings, the shared trajectory function), -48 (1538, the shooter's view ray, part 5 lines 257-260), -17 (1498: the Worker decodes 'into its own state', lines 321-322, and the bite plants 'a client quantiser', so float work) and -19 (1502, the same hash). Not covered: -18 (a time limit), -49 (the server's own rewind), -42 and -61 (already way (i)), and the rest (counts, sizes, times, server-side figures), each with its line. Suggested way for each covered row; L1 alone compares no two architectures; a replay must name x86-64. |
| arm-rule | netcode-architect | 0001 | FIX | Writes the rule once, in a new paragraph after item 5 of 'What a spike must prove'. LG's clients run on CAX21s (his words). No verdict rests on native aarch64 agreeing with x86-64 or Wasm (PM-0004-73: x86-64 native and Wasm only; PM-0004-26 only before Arm hosts; ADR 0004 part 7 not enough on its own). Rows -17, -19, -48 and -54 each name way (i) or way (ii); an aarch64-only mismatch is a cross-architecture finding that decides nothing. No row added; labelled the advisor's reading. |
| sh-1293 | netcode-architect | 0001 | FIX | Line 1293 now reads '(R2-ROOM-REC2 marks 2 and 4, on the CCX23 (ADR 0006)).' SH and the CX33 leave (condition 8). |
| capacity-756 | netcode-architect | 0001 | FIX | The failure row 'No Hetzner capacity in the EU' stays, and its other two cells change: the reference server or a load generator's CAX21 cannot be created in `fsn1` on a run day, because both are created fresh (his words), so his console check doesn't promise that day's capacity (the advisor's reading). That run waits (proposed); a lasting shortage takes OQ-0006-1's option (b) route. Still 3 cells. |
| domain-464 | netcode-architect | 0001 | FIX | Lines 463-466: the WebSocket leg's certificate is on shamblefrontlines.com, now registered, with his words 'DOMAIN: done. shamblefrontlines.com is registered at Cloudflare.' and his earlier SPIKE-MACHINES quote kept as his. Reflowed: old line 463 was 121 characters. |
| domain-1274 | netcode-architect | 0001 | FIX | Lines 1274-1276 (my text, item 1 of 'What a spike must prove'): 'the domain he registers' becomes 'the domain, registered (Mustafa, 2026-11-03; his earlier words: ...)'; the rest of the sentence is unchanged. |
| budgets-719 | netcode-architect | 0001 | NOTE | No change: 'Every server mark is measured on one vCPU of the reference server' is true again, since every server row runs on RS (DS dropped, SH leaves). Its 'unless the row says otherwise' covers the unpinned rooms-per-VM rows. It names no location and no address. |
| cap-clause-for-test-engineer | netcode-architect | 0001 | NOTE | No change in my text. Condition 11's one clause belongs at the connection cap in the key (lines 1360-1361), which is test-engineer's text. The NOTE gives the exact proposed clause: LG's addresses are new each run day (his quote), and the cap is raised on the run day (the advisor's reading). Part 10 (444-450) names no machine address. |
| other-hits-0001 | netcode-architect | 0001 | NOTE | No change for the other search hits in my text: 34, 448, 509, 520 (the pointer to OQ-0006-15, now answered), 677, 720, 777, 1219, 1312, 1716 and 1813, each with its reason. No CX33, CCX13, DS, SH, MV, nbg1, hel1, fsn1, Nuremberg or Helsinki in my text. The Test plan's soak follows condition 7 (engine-architect (a)). |
| routed-to-test-engineer | netcode-architect | 0001 | NOTE | No change in my text: 1350-1357, 1354-1363, 1462-1464, and PM-0001-1, -16, -27 and -63 (with their SH removals under condition 8) are in the Pass marks section, test-engineer's text. PM-0001-48 and -54 take the way NOTE rows-for-test-engineer gives. |
| IDX-1 | engine-architect (b) | README | FIX | ADR 0006's row (:21): 22 numbered (OQ-0006-1 to -22; -22 the staging server's x86 plan, new), 17 open; -1 and -2 answered by Mustafa (2026-11-03) through his Hetzner check; the rest unchanged. |
| IDX-2 | engine-architect (b) | README | FIX | Total (:23-25): 116 numbered, 96 open. OQ-0006-1 and -2 answered and OQ-0006-22 new, from his Hetzner check later that day. |
| IDX-3 | engine-architect (b) | README | FIX | OQ-0006-1's row (:133): answered by his console check (his two quotes); option (a), the reference server stands (advisor's reading); R6-HOST-OQ4 and OQ7 still carried; run-day capacity fallback (ADR 0001 failure row, option (b)); points to OQ-0006-22. |
| IDX-4 | engine-architect (b) | README | FIX | OQ-0006-2's row (:134): 'Answered by Mustafa (2026-11-03)' with his Falkenstein quote; `fsn1`; options kept as the record; production follows the test servers' location unless R6-HOST-OQ2 says otherwise; PM-0006-12 decides nothing (both advisor's readings). |
| IDX-5 | engine-architect (b) | README | FIX | New row OQ-0006-22 after OQ-0006-21's row: the staging server's x86 plan now that the CX33 cannot be ordered in Falkenstein; why (Arm host, PM-0004-26; part 8's lean case); options in ADR 0006; decider Mustafa (money) with the Advisor, before slice 3. |
| IDX-6 | engine-architect (b) | README | FIX | OQ-0006-15's row (:145): earlier quotes kept; 'registered before the netcode proof test (advisor's reading)' becomes registered, in his words 'DOMAIN: done. shamblefrontlines.com is registered at Cloudflare.' |
| IDX-7 | engine-architect (b) | README | FIX | OQ-0003-1's row (:62): 'nothing stands in for RD or MAC' gains the one exception, a GitHub macOS runner for the proof test's yes-or-no and count Safari rows if his check passes (branch 2: Mac borrowed before the test); points to SPIKE_PLAN 'Machines'. |
| IDX-8 | engine-architect (b) | README | FIX | OQ-0006-5's row (:66): one pointer, 'registered, Mustafa, 2026-11-03', beside his kept quote 'a domain I register'. |
| IDX-N1 | engine-architect (b) | README | NOTE | No change: :82 (OQ-0002-5) and :138 (OQ-0006-7) already give the runner price; :141 is a billing address; :147 (OQ-0006-17) still holds; :385 (N34) stands; :26-31 pass marks stay 341. |
| SPK-1 | engine-architect (b) | SPIKE_PLAN | FIX | Machines paragraph (:30-45): RS is the CCX23 and every server row runs on it; LG is a CAX21 'so their build targets aarch64', both in `fsn1` on run day, with his two quotes; LG-COUNT sentence; MV as a role on LG; MACHINES-TIMING quoted in full plus the runner's (i)-(iii) and limits; SH and CX33 removed; no fixed address. |
| SPK-2 | engine-architect (b) | SPIKE_PLAN | FIX | Item 1 'What it proves' (:112-119): 'rooms per VM on both machine types' becomes 'on the reference server' (SH has left). |
| SPK-3 | engine-architect (b) | SPIKE_PLAN | FIX | Item 1 'Depends on' (:129-154): OQ-0006-1 and -2 answered, RS and LG in `fsn1` on run day; 'LG, with MV's role on it, beside RS'; his laptop-and-desktop words; the Safari leg starts with his runner check; the domain is registered (his words); the DNS record is set each run day before the certificate is issued. |
| SPK-4 | engine-architect (b) | SPIKE_PLAN | FIX | Item 1 Size (:160-162): '8 rooms on two machine types' becomes '8 rooms on the reference server'. |
| SPK-5 | engine-architect (b) | SPIKE_PLAN | FIX | Item 1 'It decides' (:175-176): 'it feeds OQ-0006-2' becomes 'it reports the round trip to `fsn1` (PM-0006-12), which decides nothing now that OQ-0006-2 is answered'. |
| SPK-6 | engine-architect (b) | SPIKE_PLAN | FIX | Item 3 'Depends on' (:244-247): his words 'rough checks first, then a short re-run once they're named' replace the stand-in reading; the plan still lists the rows that run before naming; nothing is judged on a rough check. |
| SPK-7 | engine-architect (b) | SPIKE_PLAN | FIX | Item 4 'Depends on' (:281-284): 'RS orderable (OQ-0006-1)' becomes 'RS, the CCX23 in `fsn1` (OQ-0006-1 and OQ-0006-2, answered)'; his laptop-and-desktop words, as in SPK-6. |
| SPK-8 | engine-architect (b) | SPIKE_PLAN | FIX | 'What this plan does not decide' (:512-518): the server location leaves the open list and OQ-0006-22 (before slice 3) joins it; his Hetzner check and later answers settled OQ-0006-1, -2 and when each machine is needed. |
| SPK-N1 | engine-architect (b) | SPIKE_PLAN | NOTE | No change: :274, :321, :352, :362, :410 (later spikes' Safari or MAC rows; the machines paragraph makes these each spike's own plan's); none of SH, CX33, `nbg1`, `hel1` or a fixed address is left; Nuremberg appears only inside his quote. |
| OVW-N1 | engine-architect (b) | ENGINE_OVERVIEW | NOTE | No change: no line becomes untrue. It names no machine, location, plan, price or address and gives no open-question count. |
| R6B-RR-1 | rust-reviewer | 0004 | FIX | Adds the one sentence condition 6 allows to part 7, right after 'before ADR 0006 may pick ARM hosts (the build flags are ADR 0002's).' (line 290). The sentence: 'An ARM load generator, such as the netcode proof test's (ADR 0001's LG), is not an ARM host in this sense, since it serves no player and runs no room (the advisor's reading, 2026-11-03), and under ADR 0001's rule for the rows that compare an LG client's result with the server's, no verdict of those rows rests on native AArch64 agreeing with x86-64 or Wasm.' The rule is limited to those rows on purpose: PM-0004-26's verdict is that very agreement, and it stays. Lines 290-295 (the end of the paragraph) are rewrapped from 6 lines to 10, and every word outside the new sentence is unchanged. S7, PM-0004-26 and the A64 key are not touched. |
| R6B-RR-2 | rust-reviewer | 0004 | NOTE | No change. Searched ADR 0004 outside 'Pass marks' (lines 1-1013 and 1238-1421) for: CX33, CCX13, CCX23, CAX21, Hetzner, cheaper, the Nuremberg, Helsinki and Falkenstein names and codes, R6-HOST-OQ1, OQ-0006-, SH, MV, DS, LG, shared vCPU, 'every server', 'every native', address, order(ed/able) and x86. None of my text names a plan, a location, whether a machine can be ordered, or 'the cheaper host'. 'The reference server' appears only as the term (lines 87, 234, 827, 979, 1005, 1007, 1272). Every 'order' hit is an iteration, phase or slice order. The CX33, the cheaper host, the location and R6-HOST-OQ1 appear only at 1030-1032, 1117 and 1120 (the keys, PM-0004-1 and PM-0004-4), all in test-engineer's pass-marks text. |
| R6B-RR-3 | rust-reviewer | 0004 | NOTE | No change; input for test-engineer's statement on PM-0004-4 (condition 6). My Appendix A rows for R1-ECS-OQ4 (1329) and R1-TICK-OQ5 (1334) name only the sim core spike (S1, and S2 for R1-TICK-OQ5), so they stand once PM-0004-4 drops its CX33 part. The research's own deciders need no shared host: R1-ECS-OQ4 at r1 lines 2624-2626, R1-TICK-OQ5's P1-P3 at r1 lines 2658-2660; P1 runs on RS, P2 and P3 on the minimum laptop. In my view both are still decided from RS alone on the server side. If test-engineer instead says either stays open, rows 1329 and 1334 need a matching change that this round cannot make, and the report should say so. |
| EA-A-1 | engine-architect (a) | 0002 | FIX | Condition 7. 'No arm64 leg' (626-627) keeps its text and gains one sentence: the netcode proof test's room server runs on the x86-64 reference server, and its only Arm machine is the load generator (ADR 0001's LG, CAX21 VMs), so the bot clients' aarch64 build belongs to that spike. The playtest soak's bot VM is named when the first soak is planned. If it is Arm, `devtools` gains an aarch64 build target then (a build, not a determinism leg), and the soak's desyncs are judged by ADR 0001's rule that no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm. |
| EA-A-2 | engine-architect (a) | 0002 | FIX | OQ-0002-5 (line 1032) gains one pointer: the netcode proof test first checks whether the macOS runners' Safari is new enough for WebTransport and reaches the test servers (Mustafa's answer to MACHINES-TIMING, 2026-11-03, in SPIKE_PLAN's machines paragraph). That check belongs to the spike and leaves the question open, since it asks about pull requests. |
| EA-A-3 | engine-architect (a) | 0002 | NOTE | No change to the rest of my own ADR 0002 text. The soak at 519-528, the bench's 'a CCX23 created per bench session' at 540-548, Revisit-when at 935, R5-CI-OQ7 at 1587 and OQ-0002-5 are all still true. Greps find 0 hits for CX33, CCX13, a location, OQ-0006-1/-2, R6-HOST-OQ1/2, 'development server', 'register', and the SH, MV and DS machines. |
| EA-A-4 | engine-architect (a) | 0002 | NOTE | No change, test-engineer's text (named, nothing asked of them). The MAC key (1159-1170) stays true because the runner stands in as MAC only for the netcode proof test's rows. 'No arm64 leg' (1341), PM-0002-72 and PM-0002-76 ('a CCX23 per session') and PM-0002-73 (the staging game server, no plan named) are also still true. |
| EA-A-5 | engine-architect (a) | 0005 | NOTE | No change to my ADR 0005 text (the frame, Content data, The asset pipeline): it names no Hetzner machine, location, Arm build, domain or OQ-0006-1/-2. 'The development server' (411, 417, 744; 2283, 2290, 2379, 2382) is the native server on the minimum laptop or Mustafa's PC, never a Hetzner machine. Line 1053 (game-designer's) names no plan or location. |
| EA-A-6 | engine-architect (a) | 0005 | NOTE | No change, test-engineer's pass-mark key at 2204-2213 (named, nothing asked of them). 'A Hetzner CCX23, Linux, one EU location' is true for the CCX23 in Falkenstein. The MAC key ('SPIKE_PLAN's macOS machine') stays true because the GitHub runner stands in as MAC only for the netcode proof test, and look-proof-test rows stay on the borrowed Mac (AMENDMENT 2, the advisor's reading, 2026-11-03). |

## 10. The advisor's review of plan revision 4, at the stop (word for word)

The verdict of 2026-11-03 on plan revision 4 (record the stop's answers, close the change-set), copied byte for byte
from its verdict file; its working notes are left out.

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 4 (record the stop's answers, close the change-set), v2-decision-drafts, PR #33, reviewed at branch tip
829069d (content head 4374a1c, CI gate and smoke green there), base main 9e7db28. Advisor agent, 2026-11-03.

**Why, in one line:** the plan records the answers inside their words, keeps each author to its own text, closes by
CHECKPOINTS §9 and keeps CLAUDE.md's merge rules. But its table misses about 60 of the map's 126 places and lists one
row that doesn't exist (TOY-BOX). It also leaves GLOSSARY §8's 60 Hz entry, which this PR wrote itself, contradicting
two answers. And several answers settle only part of their question: the conditions and rulings below fix all three.

**TL;DR**
- Build from the map, not just the plan's table: every map row gets a change or a "no change" with its reason.
- STATUS-FIELD: (b). The drafts stay "Proposed", signed off by Mustafa as the basis for the spikes. POINTERS: (b).
- GLOSSARY §8's "The 60 Hz margin" entry is this PR's own text: amend it now. Don't just leave a note.
- Mustafa's "once it's approved, squash-merge it" sets the merge method. It is not his go: his go must come after
  the advisor approves the final head.
- One answer can't be closed by a reading, and it blocks the netcode proof test: the reference machines (OQ-0003-1).
  It goes to Mustafa.

### What I checked myself
- `.claude/agents/advisor.md` on this branch = main's (diff: identical). Base 9e7db28 = origin/main.
- The words file against Mustafa's relayed message: identical text. Which answers are his and which are the Advisor's,
  read from the message's own marks.
- The map: all 14 tables, the counts and the 14 questions. I spot-checked its key places at 4374a1c: ADR 0001 part 2
  (118–128), OQ-0001-1 to -6 (1022–1083), part 13 (537–547), PM-0001-3 (1382), the machine key (1266–1280); ADR 0003
  OQ-0003-17 and -20 (914–950), PM-0003-44 and its fail rule (1178, 1186–1188); ADR 0004 (45–47, 1083–1087, PM-0004-71
  and -72); ADR 0005 AB12, AB13 (1332–1348) and the Rollback (2410–2417); ADR 0006 part 8 (315–353), OQ-0006-17 to -20
  (962–1018) and part 13; the 60 Hz restatements in 0002, 0004, 0006 and ENGINE_OVERVIEW; README lines 1–62. All match
  the map.
- The report's "Decisions needed" (lines 105–133): SPIKE-ORDER asked to "accept the six drafts as the basis for the
  spikes"; REWIND-CAP recommended "set it from the proof test's p99".
- GLOSSARY §8 (lines 744 on) was added by this PR (the diff against main; engine-architect-glossary8.md). It is not a
  settled research input.
- The 31 MB sits entirely under `.checkpoints/` (294b3cf, the skeptic-6 scratch). The tree holds 51.8 MB, of which
  46.1 MB is `.checkpoints/`. A squash merge with the folder deleted keeps it all out of main.
- The repo allows squash merges. Its default squash message is COMMIT_MESSAGES, which would paste every autosave
  commit's message into main. delete_branch_on_merge is false. PR #33 is a draft from this repo, not a fork.
- The background saver is running (`checkpoint.mjs watch v2-decision-drafts --every 5`).
- Not run: the gate. This is a plan review and nothing has been edited yet; the gate runs on the final head at the PR
  review.

### Conditions
1. **The map is the work list.** Every map row (RC1–10, H1–24, S1–23, N1–16, RW1–16, O1–23, M1–24, L1–14, T1–4,
   E1–6, C1–11, X1–5, F1–7, Q1–6, A1–5, B1–7, LA1–4) gets a disposition from its author: a FIX block, or "no change:
   <reason>", following the rulings below. fixcheck-e checks the disposition of every row, not only the rows in the
   plan's table. Correct the plan's table as the map's question 14 says:
   - TOY-BOX has no README row.
   - ENGINE_OVERVIEW changes at :3–4, :15 and :117.
   - ADR 0006 needs places for TICK-60-HEADROOM and for OQ-0006-5's development case.
   - README:47 names all three crates.
2. **Attribution follows the message's marks.**
   - Mustafa's answers: SPIKE-ORDER, TICK-60-HEADROOM, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, LATER.
   - The Advisor's: TICK-60-SETUP, NET-PROFILES, OQ-0002-8, OQ-0005-1, OQ-0001-14, ROOM-CRASH, OQ-0003-20, OQ-0005-AB12
     and AB13.
   - OQ-0003-17: the Advisor's, confirmed by Mustafa.

   Quotes are word for word from the inputs file. A reading taken from this verdict is written "(the advisor's
   reading, 2026-11-03)", so it never passes for the deciders' words. The report's "At the stop" section lists every
   such reading, so Mustafa and the Advisor can overturn any of them.
3. **GLOSSARY §8, "The 60 Hz margin" (lines 782–797), is amended in this round** by engine-architect, who wrote it.
   This replaces the plan's NOTE-only rule for this one entry.
   - The definition stays (1 − p99 ÷ 8.33 ms).
   - "It is measured in the netcode proof test, in one run" becomes: one run, an early signal in wave 1, with the
     deciding run at slice 6 on the real sim.
   - The conditions add "the goal budget holds at the rooms per VM measured at 60 Hz", or the entry points to ADR 0001
     part 2 for them.
   - "Depends on" reads: confirmed by Mustafa, 2026-11-03, with his addition.
   - Lines 1–743 keep their bytes (the write guard's prefix check).
   - The older entry at lines 707–712 (map M24) stays a NOTE in the report's open findings.
4. **STATUS-FIELD (b), worded as follows.** "Status: Proposed" and "Signed off by Mustafa: as the basis for the spikes
   (2026-11-03, SPIKE-ORDER (b)); not yet accepted".
   - A draft becomes "Accepted" only when Mustafa signs it off after its spikes report. Marks passing never does it on
     its own.
   - The same wording goes in the headers of ENGINE_OVERVIEW and SPIKE_PLAN, README's Status column and README:24–25.
5. **POINTERS (b).** README's 95 pointers lose their line numbers, "(line 162 at 604a497)" included.
   - Each pointer keeps its file link and names its OQ id, part or section. Where a row has neither an OQ id nor a part,
     the pointer names the section.
   - README:10's sentence about line numbers changes to say so, and the report's risk 2 goes.
   - This is the last README edit, after every other README FIX block.
6. **Keep pass-mark changes minimal.**
   - Add one new row in ADR 0001's 60 Hz group: rooms per VM at 60 Hz, with the goal-budget check (test-engineer).
   - OQ-0005-1's rule becomes a bite on an existing CD-2 whole-set row (PM-0005-4 or -23), not a new row.
   - fixcheck-e recounts by script. Expected: 98 open questions, and 341 pass-mark rows if one row is added.
7. **Close the change-set (§5 and §9).**
   - Stop the background saver (SIGTERM to its `node` process, so its last save runs) before the final report edits.
   - Make the deletion commit by hand, with no "[skip ci]". It is the PR's head when the PR is marked ready.
   - `git ls-tree -r --name-only HEAD -- .checkpoints` prints nothing. Run the gate locally.
   - After that, nothing pushes to the branch except review fixes. A fix that takes more than one step turns the PR
     back into a draft with a new checkpoint (§9.4).
8. **Fold in before deleting.** The appendix carries, byte for byte:
   - Mustafa's words (the inputs file);
   - this verdict;
   - one "At the stop" table: each answer and where it landed, by commit.

   The report also names the last commit that still holds the checkpoint. It says that every SHA it cites is one of
   PR #33's commits: after the squash they are reachable through the PR and the kept branch, not in main's history.
9. **The merge.** Mustafa's message of 2026-11-03 fixes the method (squash only) and the order of work. It is not his
   go: it came before any APPROVE of the final head.
   - **Before the merge:**
     - The advisor has approved the final head's full SHA, in this session or in the PR review comment
       (Mergincanli) that names that SHA.
     - gate and smoke are green on that SHA.
     - Mustafa has said "merge" in this session's chat, about PR #33, after that APPROVE.
   - **The merge itself:**
     - Run `gh pr merge 33 --squash --match-head-commit <that SHA>`.
     - The session writes the subject and body (not the repo's default, which pastes about 100 autosave messages into
       main), ending with the attribution lines.
     - Don't delete the branch.
   - **The PR:**
     - The PR body's first line: "Merge by squash only (Mustafa, 2026-11-03)". It ends with the Claude Code footer.
     - The PR is needs-mustafa (ADRs, escalation category 3).
     - A later push needs the advisor's review again, then his go again.
10. **The report's last two sections** carry the escalations below.
    - "For Mustafa" lists:
      - the Hetzner console check (OQ-0006-1);
      - registering the domain for the WebSocket leg, before the proof test;
      - the reference machines;
      - his go for the merge, after the advisor's APPROVE.
    - STATUS's next step lists what the netcode proof test still waits on: those items, OQ-0006-20, the certificate
      method and the session's SSH key handling. The last two are for the proof test's own plan, with
      security-auditor.
11. **Order.** The netcode proof test's change-set branches from main after PR #33 merges, plan first.
12. **Models and effort (CLAUDE.md).** Opus for every agent. xhigh for the authors' FIX blocks on the ADRs,
    ENGINE_OVERVIEW and the netcode text; high for fixcheck-e, its skeptics and docs-keeper.
END CONDITIONS

### Rulings (the plan's decisions and the mapper's questions)
- **STATUS-FIELD:** (b). Mustafa accepted "the six drafts as the basis for the spikes" (the question he answered).
  Writing "Accepted" would claim more than he said, and accepting an ADR is his call alone (escalation category 3).
- **POINTERS:** (b), on condition 5. The pointer script leaves with the checkpoint, so re-pinned numbers would go stale
  at the next edit.
- **LOOK_V2-§8:** LOOK_V2 is not edited. The question goes into the report as an escalation (below).
- **GLOSSARY-8 (question 1):** amend the 60 Hz entry, this PR's own text (condition 3). M24 stays a NOTE.
- **TICK-60-HEADROOM (question 2):**
  - "The goal budget" is part 8's goal budget as it stands when the call is made: at slice 6, the figure that OQ-0006-19
    fixes. In the wave-1 early signal, the cost is reported beside part 8's provisional figures and decides nothing.
  - "Holds" means: the fleet the goal modes need at the rooms per VM measured at 60 Hz (part 8's formula, same machine,
    N = 48) costs no more than that figure.
  - The new mark lives in ADR 0001's 60 Hz group. ADR 0006 part 8 and spike item 2 point to it, and ADR 0006's "none
    runs at 60 Hz" lines stay true. OQ-0006-19 and OQ-0006-10 stay open, with a pointer.
  - Restatements that say "60 Hz only with the 60 Hz margin" and link §8 (0002:47, 0004:182 and 612, 0006:50) state a
    necessary condition. They may stay once §8 is amended.
  - Every line that says the proof test measures or decides the 60 Hz call changes: ADR 0001 at :11, part 2, the
    budgets row, spike item 3, migration steps 1 and 8, and the rollback; ADR 0004 at 45–47; SPIKE_PLAN item 2.
  - The summary lines (ADR 0001:11, ENGINE_OVERVIEW:15, README N2) state the full rule, or point to part 2.
- **TICK-60-SETUP (question 3):**
  - Slice 6 always runs the final 60 Hz measurement on the real sim, whatever the early signal shows.
  - The early run (OQ-0001-2 (a), with S1's stand-ins) decides nothing. A pass starts no 60 Hz work before slice 6:
    PM-0004-71 and -72 run with each 60 Hz run but decide only at slice 6, and R1-TICK-OQ1 (60 or 64 Hz) is asked only
    after slice 6 passes. A fail closes nothing.
  - Options (c) and (d) stay in the record as weighed.
- **SPIKE-ORDER (question 4):**
  - The 60 Hz run stays a run on the proof test's kept server and harness, in its own VM session after the sim core
    spike. The proof test's "Done when" closes at 30 Hz without it: its tick-rate clause goes, and the 60 Hz run reports
    separately.
  - The boundary spike now uses the sim core spike's sim, so its "stand-in until then" clause may go.
  - The asset research's export and slot-naming part is agent research, not a heavy spike, and it does not run
    alongside one: only the preview hosting check may. It sits between two wave-1 spikes, early enough that its export
    and slot rules reach Mustafa before he makes the models. Recommended: right after the netcode proof test.
- **REWIND-CAP (question 5):**
  - "The honest rewind the proof test measures at P2" is its p99 at P2, as option (c) defines it and as the
    recommendation he answered ("set it from the proof test's p99"). The run's largest rewind is reported beside it.
  - Rounded up to a whole tick at the run's tick rate (as PM-0001-48 does).
  - The one-interval tolerance and part 8's anti-cheat text are unchanged. The value is recorded when the proof test
    reports.
  - The checklist: step 10's 200 and 250 ms comparisons go, since no candidates are left to weigh. Step 7 uses the
    rule. Step 11 stays: it records the "hit after cover" window inside the cap.
  - SPIKE_PLAN item 10's Feel Lab weighing of the cap goes. "Revisit when" keeps his Brave check of the measured cap as
    the trigger for feel.
  - "The smallest input buffer that holds" guides OQ-0001-6, which the proof test still decides: of the options that
    pass its marks (corrections, fairness, the fast-clock bound), take the smallest buffer, with (a) run-on-arrival
    counting as none. The options stay.
  - PM-0001-48's test cap (the run's largest rewind) stays: it judges a different mark.
- **NET-PROFILES (question 6):**
  - "Loss as P2 defines it" means 2 % on each direction, applied independently, as PM-0001-3 already measures. About
    4 % of round trips lose a packet.
  - Part 13 gets one clause saying so. OQ-0001-3 and -16 are answered.
- **SPIKE-MACHINES (question 7):**
  - (a) Sessions get SSH access to the spike VMs. How the key is made and held without a session reading a secret it
    didn't make is for the netcode proof test's plan, with security-auditor and the advisor. The obvious candidate: a
    key pair the session makes for each run, whose public half Mustafa adds when he creates the VM. Anything that
    hands a session a secret goes to Mustafa.
  - (b) "No Hetzner token until the security baseline" covers every VM before the baseline, and the baseline comes
    before the first online playtest. So doing it by hand also covers the tick bench (from slice 1) and the 3-hour
    soaks: OQ-0006-17 is answered. The report notes the bench's hand work (a placeholder of about 20 h a month).
  - (c) LG is a CX33 as written; how many comes from the calibration run. SH stays.
  - (d) "Test PC: my own, in Brave" is MP as ADR 0001 already defines it (what he judges, never a measured result). It
    doesn't stand in for ML, RD or MAC, so OQ-0003-1 is unchanged in substance (escalated below).
  - (e) OQ-0006-5: the WebSocket leg uses development case (i), a trusted certificate on his domain. The issuance
    method is for the proof test's plan, with security-auditor: http-01 or tls-alpn-01 need an open port, dns-01 needs a
    secret, and OQ-0006-16's port plan applies. Production stays the security baseline's.
  - Partly answered: OQ-0006-15 and OQ-0006-20 (VMs only during a run, but no cap). OQ-0006-1 stays open.
- **LOOK-TEST-INPUTS (question 8):**
  - LOOK_V2 is untouched. Blender is recorded as the route for the look test's models and for the research part's
    export.
  - Part 11's list of objects stays; each object may be a blockout at Mustafa's bar.
  - Which look-checklist steps a blockout can judge is for the look test's own plan (graphics-engineer, test-engineer),
    and any visual call is Mustafa's. L12: no checklist edit now.
- **OQ-0003-17 (question 9):** the narrow reading.
  - (a), at most 6 of 600 frames, is §6.9's reading, and the p95 rows are diagnostics for it.
  - PM-0003-18 to -21 keep gating their own needs. OQ-0003-20's answer has the soaked set gate them, which only makes
    sense if they gate.
  - PM-0003-44 is gated on RD at High and reported on ML. Its bite (10 of 600) still bites.
- **OQ-0005-1 (question 10):** the rule binds the kinds the client presents.
  - CD-2's whole-set check fails when an id of a presented kind has no client entry.
  - engine-architect names the presented kinds in CD-2, from AP-2, CD-13 and AB-3 (game-designer confirms the ability
    kinds).
  - Kinds with nothing to present (tuning tables, modes) need no entry and are listed by name.
- **OQ-0005-AB13 (question 11):** "built from data" includes a new reviewed generic operation, as the drafts' model
  already has it (AB-6; "every new operation is a reviewed Rust change").
  - Scripting comes back when one designed ability needs ability-specific code that no operation can express.
  - "Needed often" and "many fail" are no longer triggers: align the rollback, "Revisit when", the frame's table and
    Appendix A.
  - (b), reading the Overwatch talk, stays as a weighed option, and nothing schedules it.
- **ROOM-CRASH (question 12):**
  - "Keep" means written and kept under ADR 0006 part 11's proposed retention. No new period.
  - One word: ADR 0006 part 12's stub record is "marked crashed", as in ADR 0001 part 11 and PM-0001-59.
  - PM-0001-59's tail about option (b) goes.
- **LATER (question 13):**
  - The twelve LATER questions go to the design session after the netcode proof test. The 38 design questions join its
    agenda: deciders unchanged, and Mustafa may trim the list.
  - Three keep their own timing, set by the evidence they need:
    - OQ-0003-16: in the look proof test.
    - OQ-0006-18: before the first model is committed. If one comes before the session, it goes to Mustafa then.
    - OQ-0006-13: before the foundation's client-host token. Until then the preview hosting check runs under B9's
      interim rule (every deploy waits for his approval).
  - The drafts' own entries for these questions are not edited (LA3). README gets one sentence and a marker on each
    row.
- **COUNTS:** a partly answered question stays in the open count, marked "answered in part". OQ-0001-5 stays open,
  since its value is pending. That makes 98 open (condition 6).
- **TOY-BOX:** nothing changes in the nine documents. STATUS and the report record the kit change-set that comes after
  PR #30, together with the checkpoint anti-waste rules.
- **OQ-0002-8:** (a). README names all three crates. **OQ-0001-14:** (a). No need line changes.
- **REASON-COUNT:** Mustafa's "seven usage-limit cuts in three days" is quoted as his words. The report keeps its own
  count, labelled as the checkpoint's.

### Findings (ranked)
1. Completeness: about 60 of the map's 126 places are missing from the plan's table (condition 1).
2. GLOSSARY §8's 60 Hz entry is this PR's own text and contradicts TICK-60-HEADROOM and TICK-60-SETUP (condition 3).
3. "Once it's approved, squash-merge it" could be taken as an advance go. It isn't one (condition 9).
4. The repo's default squash message would carry about 100 autosave messages into main (condition 9).
5. After a squash, the cited SHAs and the deleted notes live only in PR #33's commits, so the branch must be kept and
   the report must say where they are (condition 8).
6. OQ-0003-1 is unchanged by "Test PC: my own" (MP was already his PC), and it blocks the proof test (escalation 1).
7. The plan names no effort for its agents (condition 12).
END RULINGS

### Decisions needed (for the report; none blocks this plan)
- **REF-MACHINES (OQ-0003-1)**
  - The question: which laptop is the minimum (ML), which desktop the recommended (RD), and which Mac (MAC)?
  - Why it matters: about 22 of the netcode proof test's 62 marks are measured on ML or RD. Your PC (MP) only judges
    and is never a measured result, so the proof test can't start without them.
  - Options: (a) name, buy or borrow ML, RD and MAC before the proof test's plan; (b) let your PC stand in as RD for
    the proof test only, which changes MP's "never measured" rule; (c) start the rows that need neither and hold the
    rest.
  - Recommendation: (a) for ML and RD. MAC may follow, for the Safari rows.
- **LOOK_V2-§8**
  - The question: you chose Blender for the look test's models, while LOOK_V2 §8 says "a 3D model in Spline". Does §8
    change?
  - Why it matters: agents read §8 as the production art flow.
  - Options: (a) keep §8, with Blender for the test models only; (b) change §8 to Blender, in a change-set by you and
    the Advisor.
  - Recommendation: (a), until the asset research's export part reports on both tools.
- **SPIKE-COST (OQ-0006-20)**
  - The question: what may the proof test's VMs cost?
  - Why it matters: no cap is set, so each run plan's cost comes to you before you create its VMs.
  - Options: (a) 5 EUR for ADR 0006's own runs; (b) one cap set from the run plan's hours at the hourly prices; (c) 5
    EUR for everything.
  - Recommendation: (b), the test review's.
- **DOMAIN (OQ-0006-15)**
  - The question: which domain, which registrar and which DNS host for the WebSocket leg, and will production use it
    later?
  - Why it matters: the WebSocket leg needs a trusted certificate on a real hostname before the proof test.
  - Options: a fresh domain for the spikes only, or the game's future domain.
  - Recommendation: the game's future domain if you already know it; if not, a cheap spike-only one.

### For Mustafa
1. Check in your Hetzner console that a CCX23 and a CX33 can be ordered in an EU location (OQ-0006-1). This is still
   open.
2. Register the domain for the WebSocket leg before the netcode proof test.
3. Name the reference machines (REF-MACHINES above).
4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat. It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

## 11. The advisor's review of plan revision 5, the design answers (word for word)

The verdict of 2026-11-03 on plan revision 5 (Mustafa's design answers, his answers to the stop's escalations,
COMMUNITY-SERVERS and the last fix round), copied byte for byte from its verdict file; its working notes are left out.

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 5 (Mustafa's design answers, his answers to the four escalations, COMMUNITY-SERVERS, the last fix
round), v2-decision-drafts, PR #33 (draft), branch tip 6115116, base main 9e7db28. Advisor agent, 2026-11-03.

**Why, in one line:** the plan keeps his scopes, routes every fixcheck-e finding to the text's real author and honours
"no further re-check rounds". But it misses four things. It over-reads two answers: the reference machines and spawn
protection. Its gloss on "doesn't measure client frames" is wrong for one row. And it leaves three spikes depending on
machines that, by his timing, won't be named yet. The conditions below fix all four, without a new round.

**TL;DR**
- DRAFT-ENTRIES: (a). The design answers go in README and STATUS only. README says the counts follow the drafts.
- OQ-0003-1 stays "answered in part": the classes are chosen, but the actual machines are not named yet. So the count
  is 115 numbered, 97 open (not 96). Pass marks stay at 341.
- "MP is never a measured result" stands. His laptop on its integrated graphics is a separate stand-in for ML, for rough
  checks only. No pass mark is judged on it, and nothing stands in for RD or MAC.
- COMMUNITY-SERVERS: nothing in ADR 0001 part 11 or ADR 0006 locks it out. It gets one open question, OQ-0006-21, and
  one README row. No field, part, pass mark or crate.
- One new decision for the report: three spikes before the look test measure on the laptop, desktop or Mac.

### What I checked myself
- `ANSWERS-2-mustafa-2026-11-03.md` against this run's relayed message: byte for byte the same (diff). My earlier
  run (cut) checked `DESIGN-ANSWERS-…` against the message relayed then: the same. `.claude/agents/advisor.md` here =
  main's.
- The plan on disk (rewritten, uncommitted) against both inputs, verdict r4 (conditions 7 to 9, COUNTS, SPIKE-MACHINES
  (d)), fixcheck e1, e2, e3, e2b, e3b and skeptic-8 (e1-1, e2b-1, e2b-2).
- Authorship, from the stop-*-fixes files:
  - e2 F2's "loss on each" (0006 key) is test-engineer's (stop-test-engineer-fixes.md:413), so the plan's routing is
    right and the verifier's label was wrong.
  - e2 F1's key place (0003:1122–1123) is test-engineer's (:284–287); its part 8 place (0003:404–405) is
    graphics-engineer's.
- README: lines 1–75, 76–141 (AB11, AB14, AB15, OQ-0006-1, -15 to -20) and "Design questions" (143–184): 38
  DESIGN-owned ids, and none of his answers' ids among them. So "the README's design questions" means the rows in
  "Decisions the drafts ask for", plus one sentence in that section. The plan reads it the same way.
- ADR 0005:
  - OQ-0005-AB11 (1351–1361): two questions. His answer covers the first; the second is left to "numbers tuned later".
  - AB14 (1387–1397): options (a)/(b) for what it covers. "Against spawn killing" picks neither.
  - AB15 (1405–1413): options (a)/(b)/(c).
  - CD-2 (319–329): "the rule tables". PM-0005-4 (2273) says "a tuning table" twice.
- ADR 0001:
  - Part 11 (473–531): one process per room; a mode id plus overrides; `Join` carries the protocol version and
    content hash, and the room checks both. The directory records build, version and hash. The room writes the
    record.
  - The message catalogue (663–664, 753).
  - The machine key (1355–1369).
  - 23 of its 63 pass-mark rows name ML, RD or MAC (script over column 4). PM-0001-43 times input-to-screen latency
    through "presented frames on a 60 Hz display" on ML and RD. PM-0001-28 (it decides R2-TRN-OQ1) and spike claim 1
    ("four browsers") need Safari, so they need MAC.
- ADR 0002's crate table (189–193): `server` = "the room process … the supervisor hook"; the supervisor "runs room
  processes as programs".
- ADR 0006:
  - Part 12: "Every match writes one record inside the room".
  - OQ-0006-15 (976–991) and OQ-0006-20 (1057–1075).
  - Every "answered in part" label for OQ-0006-15 and -20 (0006:291, 756, 1346; README; SPIKE_PLAN:140, 143, 496).
  - PM-0006-18 ("under the cap OQ-0006-20 sets") stays true.
- SPIKE_PLAN:
  - Item 1 (:129), item 3, the boundary spike (:234), and item 4, the sim core spike (:269): each "Depends on" ML (and
    RD, MAC) named.
  - Item 5, the look test (:307): also depends on them.
  - "Blockouts": the same phrase at :312–313 as ADR 0005:1498–1500.
  - "Not a session's" (:326–327, :520).
- The MP sentence "stands in for none" sits at 0001:1366–1369, 0003:821–826, README:53 and SPIKE_PLAN:37.
- e3 F1: `stop-session-notes.md` already holds NOTEs for the six map rows (N7, O23, L14, T2, T4, LA4).
- Not run: the gate. This is a plan review and nothing is edited yet. The gate runs on the final head at the PR
  review.

### Conditions
1. **His words and their labels (README rows, STATUS).**
   - Quote word for word from the two inputs files. Every label is no stronger than his words:
     - **AB1:** answered. "Assumed, to confirm: the class's own abilities stay." is kept as his words, and no one is
       named to confirm it.
     - **AB2:** still open, his three options in his words. "Limited but generous, refilled at the barracks and supply
       points" is marked as the Advisor's lean, never as an answer. The row keeps its LATER marker.
     - **AB3:** answered.
     - **AB5:** "leaning towards class variants, with more variants added over time. Not final." It stays open.
     - **AB11:** answered for the Shield and the blade, in his words. The Peacekeeper's strength before Tech 1 is left to
       his "numbers tuned later" (the advisor's reading, 2026-11-03).
     - **AB14:** answered in part.
       - Mustafa's: 3 s, "against spawn killing".
       - The Advisor's pencil, which Mustafa may veto: "Firing ends it early".
       - Still open: what it covers (ADR 0005's options (a) damage only, (b) every effect).
     - **AB15:** "enemies can see a player's build".
       - Placing it at AB15 is the advisor's reading (2026-11-03).
       - How they see it is not said: the scoreboard or a cue; a cue is ADR 0003's to draw, and the look is his.
     - **Fragments:** "visual only for now", plus his sentence.
   - The name "amplifications" ("amps") goes only in README's one sentence and in STATUS. No GLOSSARY entry, no
     renaming in the drafts.
   - "Still open" items:
     - Faction asymmetry ("Mustafa works on the unit list first") goes beside OQ-0005-CD5.
     - "The war around the matches" is quoted as his words and mapped to no id.
   - Rows answered now drop their "LATER" marker.
   - DOMAIN and REF-MACHINES quote his second message (about 17:31). Nothing in the docs says he "dropped" the
     fallback. The appendix carries both messages byte for byte.
2. **DRAFT-ENTRIES (a), said once in README.**
   - One sentence beside the counts says the open counts follow the drafts' own entries.
   - README records Mustafa's design answers of 2026-11-03 for AB1, AB3, AB11 and AB15, AB14 in part, and a lean for
     AB5. ADR 0005 (and ADR 0001 part 12 for AB15's visibility class, ADR 0003 for the fragments) take them up at
     their next revision.
3. **REF-MACHINES (OQ-0003-1), one wording in every place.**
   - The places:
     - ADR 0001's key (test-engineer);
     - ADR 0003's key (test-engineer);
     - ADR 0003's OQ-0003-1 entry and its migration line (graphics-engineer);
     - README's row and the ADR 0003 count (engine-architect b);
     - SPIKE_PLAN's machines paragraph (engine-architect b).
   - The wording:
     - Label: "Answered in part by Mustafa (2026-11-03)", with his classes quoted. ADR 0003's own class text stays
       beside his words, not rewritten.
     - His laptop: "Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the
       minimum, for rough checks only."
     - The reading: "A rough check is reported, never judged against a pass mark. MP stays what he judges, never a
       measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03)."
     - Still open: the actual machines, named by Mustafa before the look proof test.
   - Every "stands in for none of ML, RD and MAC" sentence goes. Call it "the stand-in for ML", with no new machine
     code.
4. **"The netcode proof test doesn't measure client frames."**
   - It is quoted once, in ADR 0001's key (test-engineer), with this reading: no client frame-time budget is judged in
     the proof test. PM-0001-43 times input-to-screen latency through presented frames on ML and RD. Like every row
     on ML, RD or MAC, it is judged only on a named machine (the advisor's reading, 2026-11-03).
   - Don't write the plan's gloss "they measure client CPU, not frames". No pass-mark row changes; marks stay at 341.
   - SPIKE_PLAN items 1, 3 and 4 (engine-architect b) replace "ML, RD and MAC named (OQ-0003-1, still open)" (and the
     items' equivalents) with one clause: "the machines its rows name (OQ-0003-1, answered in part: Mustafa names them
     before the look proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists
     its rows that run before they are named, and none of them is judged on the stand-in (the advisor's reading,
     2026-11-03)".
   - Item 5 keeps "named and on hand".
   - SPIKE_PLAN:495 ("still to be named") gets the same label.
5. **OQ-0006-15 and OQ-0006-20 answered.**
   - Every "answered in part" label for them changes: ADR 0006 (infra-engineer), README and SPIKE_PLAN
     (engine-architect b).
   - OQ-0006-15: domain and registrar in his words. The registration falls before the netcode proof test (r4's
     reading stays).
     - The DNS host and prices stay "not researched". No new claim about Cloudflare without a source.
     - "Production uses the same domain" may be written only as the advisor's reading, 2026-11-03 (it is the game's
       name, and part 6 needs one registrable domain).
   - OQ-0006-20: his rule in his words. PM-0006-18 is unchanged.
6. **COMMUNITY-SERVERS.**
   - **OQ-0006-21** (infra-engineer) quotes his whole bullet, "possibly with revenue such as official private servers
     they rent" included, uninterpreted. Its fields:
     - Options: none weighed ("keep the door open, don't design it").
     - Decider: Mustafa. No timing set.
     - Related open design questions it does not answer: R2-ROOM-OQ7 and R6-TEL-OQ2. Progression lives under VISION_V2
       §6.10 [LATER].
   - It adds no engine feature: no part, field, pass mark, crate or "Needs outside §6" entry.
   - **README** gets one row in "Decisions the drafts ask for", pointing to OQ-0006-21.
   - **Each check gives a disposition** ("no change: <reason>" or one pointer):
     - netcode-architect: ADR 0001 part 11 and the message catalogue.
     - infra-engineer: ADR 0006 parts 4 to 6 and 12. The one pointer expected is part 12's match record.
     - engine-architect (a): ADR 0002's `server` and `supervisor` rows (the room process must not need the supervisor
       or the lobby to start), and ADR 0005's mode, content-hash and record text.
   - A pointer outside ADR 0006 goes only where a sentence would otherwise read as closing the door.
   - ADR 0003 and 0004: the session greps "room process|match record|supervisor" and writes one note line. No author
     needed.
7. **Fix-round gaps.**
   - (a) **e2b F1, both halves** (test-engineer, PM-0005-4): the kinds become "units, vehicles, weapons and factions",
     and both "tuning table" become "rule table", as CD-2 has it.
   - (b) **e2b F5's twin** (engine-architect b, SPIKE_PLAN:312–313): the same fix as ADR 0005:1498–1500 ("…, where
     blockouts with a skin, named colour slots and LOOK_V2's proportions are enough"). Skeptic-8 upheld it; e3b's
     "faithful" note on the same phrase is overruled.
   - (c) **e3b 3:** I don't confirm "not a session's" ("with AI help" may be a session's help). Drop it at
     SPIKE_PLAN:326–327 and :520. "No spike's size carries it" may stay as the plan's own sizing.
   - (d) **e2b F4** (game-designer): the pointer form, after "once it can be read": "; those claims are the GAS findings
     AB-1 to AB-3 cite, F4's summed multipliers and F17's cooldown quote among them". No re-listed id ranges.
   - (e) **e3 F1:** the six NOTEs in `stop-session-notes.md` go into the report's "At the stop" table. No author round.
8. **No re-check rounds (his words), with these limits.**
   - No fixcheck-f and no skeptics.
   - The session runs: the write guard, dispositions, glossary, tables and cell counts, the open-question and pass-mark
     counts, the gate. Two more scripts:
     - every quote attributed to Mustafa 2026-11-03 is found verbatim in one of the three inputs files;
     - a label grep: no "answered in part" left on OQ-0006-15 or -20; OQ-0003-1 "answered in part" everywhere; no
       "stands in for none"; "tuning table" gone from PM-0005-4.
   - What is left goes into the report as notes.
   - My review of the final head (gate, CI, these scripts' output, the fix round's blocks) is the merge rule's review,
     not a round. If it finds a misquote, a misattribution of his words, or a failing gate or script, that gets a
     one-phrase fix, listed in the report. Anything else becomes a note.
9. **Close as r4 conditions 7 to 9, unchanged.**
   - The appendix also carries, byte for byte:
     - `DESIGN-ANSWERS-…` and `ANSWERS-2-…`;
     - this verdict;
     - the six NOTEs.
   - The report's "Decisions needed" holds MACHINES-TIMING (below). Its "For Mustafa" is the list below.
   - STATUS "Next" records two later steps:
     - LOOK_V2 §8 reads "Spline or Blender" once the asset research's export part reports. His words fix the text; the
       edit belongs to that part's change-set and goes to him for the merge.
     - The drafts take up the design answers at their next revision.
10. **Models and effort:** Opus for every agent. The authors' blocks at xhigh; docs-keeper-4 at high.
END CONDITIONS

### Rulings
- **DRAFT-ENTRIES:**
  - (a), on condition 2. It is his scope ("Record them in the README's design questions and in STATUS"), and it means
    fewer unverified edits after his stop.
  - The escalation answers differ: they close questions the drafts' own entries carry, so the drafts take them, as the
    plan says.
- **OQ-0003-1:**
  - Answered in part, not answered. The record needs the named models ("each is recorded with GPU, CPU, RAM …"), as
    OQ-0001-5 needs its value. Under r4's COUNTS rule it stays open.
  - It no longer blocks the netcode proof test (his words).
- **"Doesn't measure client frames":**
  - Recorded as his words, with condition 4's reading. It removes the named machines from the proof test's start, not
    from its rows.
  - The plan's gloss is wrong for PM-0001-43, which times presented frames.
- **MP and the stand-in:** MP's "never a measured result" is unchanged. The stand-in is a different setup with its own
  limit ("for rough checks only"), so no key's rule changes. Nothing judged on the stand-in counts as a result.
- **Counts after the round (by script):**
  - 115 numbered, 97 open.
  - ADR 0003: 20 numbered, 17 open (-1 answered in part).
  - ADR 0006: 21 numbered, 18 open (-15, -17, -20 answered; -5 answered in part; -21 new).
  - ADR 0005: still 33 (condition 2). Pass marks: 341.
- **COMMUNITY-SERVERS against ADR 0001 part 11 and ADR 0006:** no lock-out found.
  - (1) "One self-contained program with a config file": a room is already one process per match, started with a mode
    id plus overrides. A file holding them is a second way to start it. The only thing to watch is ADR 0002's
    "supervisor hook", which must not be required to start (condition 6).
  - (2) "Checks the game's version and content hash": `Join` already carries both, and the room checks them.
  - (3) "Marks each match as official or community": part 12's record has no such field. A pointer keeps the door open;
    adding the field is design, for later.
  - Token keys per environment and official-only storage keys fit "progression … on official servers only".
- **Verification stop:** honoured (condition 8). The scripts and the gate are not re-check rounds, and neither is my
  final-head review.
- **Routing:** right for every finding, including e2 F2 (test-engineer's text, not infra-engineer's). It misses only
  condition 7's gaps.
- **LOOK_V2-§8:** LOOK_V2 is not edited in this PR. ADR 0005's Settled inputs and SPIKE_PLAN's research section quote
  his answer. "Blender for the test models only" agrees with the drafts' readings (the export part researches both
  tools' export).
- **His scopes:** kept. The design answers go in README and STATUS. COMMUNITY-SERVERS goes in ADR 0006 and README, with
  at most a pointer elsewhere.
END RULINGS

### Findings (ranked)
1. Three spikes before the look test (the netcode proof test for Safari; the sim core and boundary spikes for the
   laptop and desktop) depend on machines that, by his timing, are named only before the look test (escalation).
2. OQ-0003-1 marked "answered" would claim the machines are named. It is "answered in part" (count 97, not 96).
3. AB14 "answered" over-reads "against spawn killing" as a choice of what protection covers (condition 1).
4. "They measure client CPU, not frames" is wrong for PM-0001-43 (condition 4).
5. Missed in the routing: e2b F1's "tuning table" half, the SPIKE_PLAN blockout twin, and e3 F1's NOTEs (condition 7).
6. COMMUNITY-SERVERS' check left out ADR 0002's `server` and `supervisor` rows and ADR 0005 (condition 6).
7. docs-keeper-4's effort is not stated (condition 10).
END FINDINGS

### Decisions needed (for the report; none blocks this plan or the netcode proof test's start)
- **MACHINES-TIMING**
  - The question: you name the laptop, desktop and Mac before the look proof test, but three earlier spikes measure on
    them. Should they wait for the machines, or run first and be judged later?
    - The netcode proof test needs the Mac for Safari: its four-browser claim and the WebTransport library choice.
    - The sim core spike and the boundary spike measure on the laptop and the desktop.
  - Why it matters: until those machines exist, those rows can only be rough checks. A row judged later costs a short
    re-run, with VM hours and a session.
  - Options:
    - (a) Borrow the Mac before the netcode proof test, and the laptop and desktop before the sim core spike.
    - (b) Keep your timing. Those rows run as rough checks first (the laptop rows on your stand-in, the Safari rows held),
      and a short re-run judges them once the machines are named. The library choice waits for Safari.
    - (c) A mix: the Mac early, the laptop and desktop on your timing.
  - Recommendation: (c). Safari is the one browser where WebTransport is new, so the library choice should see it. The
    laptop and desktop rows can wait for one re-run.

### For Mustafa
1. Check in your Hetzner console that a CCX23 and a CX33 can be ordered in an EU location (OQ-0006-1, still open).
2. Register shamblefrontlines.com at Cloudflare before the netcode proof test.
3. Borrow and name the laptop, desktop and Mac: at the latest before the look proof test, earlier as MACHINES-TIMING
   says.
4. OK each run's cost cap when its plan reaches you.
5. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat. It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

## 12. The advisor's review of plan revision 6, the Hetzner check, with its two amendments (word for word)

From `advisor-verdict-r6.md`, its verdict and both amendments, byte for byte (its notes are not copied):

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 6 with its addendum (Mustafa's Hetzner check, his correction, his two answers), v2-decision-drafts,
PR #33 (draft), branch tip 898ba05 (content as 21ea733), base main 9e7db28. Advisor agent, 2026-11-03, about 18:45.

**Why, in one line:** his words settle the load generator, the two game servers and the location, and the plan keeps
every author on its own text without a re-check round. But four of its recommendations go further than the evidence:
HZ-SH (a) and HZ-STAGING (a) would put the room server on Arm, HZ-OQ1 (a) closes a question that still has an order
check left, and ADR 0001's rows can't rely on aarch64 agreeing with x86-64. Its map also misses about 20 places that
the correction and the location answer change. The conditions below fix all of this inside the one round.

**TL;DR**
- Two game servers. RS (the CCX23) runs only his three kinds of measurement runs. A new code, DS (the CCX13), covers
  development and "does it work" runs. Each run's plan names its server. Rows keep their machine columns.
- Location: Nuremberg (`nbg1`). OQ-0006-2 is answered. OQ-0006-1 is answered in part: the client VM's order check
  moves to the proof test's plan. Counts: 115 numbered, 96 open (ADR 0006: 21 and 17). Pass marks stay at 341.
- SH stays x86 only: a CX33 if it can be ordered in Nuremberg, otherwise SH leaves the proof test. Never a CAX21.
  Staging stays x86 too.
- An Arm load generator (LG) gets one rule in ADR 0001: no pass mark is judged on aarch64 agreeing with x86-64. A
  mismatch seen only on LG is replayed on x86-64 before it counts.
- Nothing new for Mustafa now. All nine decisions are mine, or already settled by his words.

### What I checked myself
- The six inputs files. HETZNER-CHECK, HETZNER-CORRECTION and HETZNER-ANSWERS were read in full. In HETZNER-ANSWERS,
  the option texts are the session's wording; his words there are only his choices. The screenshot description is
  the session's.
- `grep -n CX33` over the nine documents: 0001 ×4 (1293, 1354, 1356, 1527 = PM-0001-63), 0004 ×3 (1032, PM-0004-1,
  PM-0004-4), 0006 ×27, README ×1 (133), SPIKE_PLAN ×3 (32, 33, 129); 0002, 0003, 0005 and ENGINE_OVERVIEW ×0. Two of
  them are routed to test-engineer but missing from the plan's map: 0006:1268 (PM-0006-12's "one throwaway CX33 per
  offered location") and 0006:1274 (PM-0006-18's "0.0144 EUR (CX33) an hour").
- The other places the answers change, found by grep: 0006:244–251 (part 5: "every native server mark is measured on
  one vCPU of it"; the location "picked by a round-trip measurement"), :373 (the playtest soak's bot clients "on a
  CX33 … as spike 2's load generators"), :634, :715, :753, :1133–1134, :1300, :1364–1370, :1446–1448, :1499 and
  Appendix A :1511–1517; SPIKE_PLAN :176 ("it feeds OQ-0006-2"), :281 ("RS orderable (OQ-0006-1)") and :513.
- Research:
  - R6-HOST-F13: the verifier's "NBG1 HEL1" note.
  - R6-HOST-F15: CAX21 10.99 (0.0176), CCX13 43.49 (0.0697), CCX23 86.49 (0.1386), CX33 8.99 (0.0144), CPX32 35.99,
    all excluding VAT.
  - R6-HOST-REC2 (r6:650–652): "Arm (CAX) stays an option only after R1-DET-REC6's matrix includes it".
  - R5-TEST-REC5 (r5:1475–1481): the bots are headless Rust clients; desyncs come "from REC2's hashes on sampled
    clients".
- ADR 0004:
  - Part 7 (289–290): AArch64 joins the hash test "before ADR 0006 may pick ARM hosts".
  - PM-0004-26 (A64, "only before ADR 0006 may pick ARM hosts").
  - PM-0004-73: the trajectory function is bit-identical, checked on CI native (x86) and Wasm only. So aarch64 identity
    is unproven.
  - The CX33 key (1032).
- ADR 0002: 626–627 ("No arm64 leg … the first fleet is x86-64"), 935, R5-CI-OQ7 (1587). The soak runs on RS with its
  bots beside it (519–528). `devtools` holds the soak's headless clients (194).
- ADR 0001:
  - 23 rows have LG in the machine column (script).
  - PM-0001-54 compares each landing with the server's. PM-0001-48 compares the LG shooter's view with the server's
    registration.
  - PM-0001-17, -18 and -19 compare hashes of quantised state.
  - The RS key gives the transport three vCPUs (1350–1354).
- "Development server" already means the local, content-watching server in ADR 0005 (:411, :417, :744, :2379) and
  Vite's dev server in ADR 0002 (:945). "DS" is unused in the nine documents.
- `.claude/agents/advisor.md` is identical to main's. Base 9e7db28 = origin/main. The drafts at 898ba05 match the
  working tree.
- Not run: the gate. This is a plan review and nothing has been edited; the gate runs at my final-head review. Not
  checked: anything in Hetzner's console. What can be ordered is his check, and the rest moves to the proof test's
  plan.

### Rulings (who decides: all nine are mine or already settled by his words; none escalates)
- **HZ-SH: neither (a) nor (d). SH stays x86 only.** SH is a CX33 if the proof test's plan finds it orderable in
  Nuremberg. Otherwise SH leaves the proof test, and K3 waits for a later measurement of a shared x86 plan. Never a
  CAX21:
  - an Arm room server is an Arm host (ADR 0004 part 7, PM-0004-26 first; R6-HOST-REC2);
  - the option he chose keeps "no second build target for the room server" (the session's wording).

  His "used only for the measurement runs" limits what the CCX23 does. It doesn't drop SH (the advisor's reading,
  2026-11-03). Mine to decide: it is an evidence choice, and its hours fall under the cap he OKs for each run.
- **HZ-MV: (b), with limits.** A small VM in Nuremberg running Prometheus only, never RS. The proof test's plan names
  it after its order check. The drafts name no CX33 or CAX21 for it as a fact, and make no claim about Prometheus's
  builds without a source. Mine.
- **HZ-STAGING: not (a)'s CAX21.** Staging and the lobby's placeholder keep "proposed a CX33", with "if it can be
  ordered in Nuremberg (Mustafa found it not orderable in Falkenstein, 2026-11-03); otherwise an x86 plan chosen before
  slice 3". An Arm staging server is an Arm host, and part 7 asks for PM-0004-26 first (the advisor's reading,
  2026-11-03).
  - The budgets and PM-0001-63's formula keep 8.99, labelled as the CX33's price (R6-HOST-F15), a placeholder. No
    21.98 lean case.
  - No new open question (115 stays).
  - The lobby's "for example a CX33" stays an example.
  - Mine now. The staging server's monthly plan and price go to Mustafa with slice 3's plan (money, later).
- **HZ-LOCATION: (a). His answer, OQ-0006-2 answered.** Label: "Answered by Mustafa (2026-11-03): Nuremberg (`nbg1`),
  the option he chose". No quotation marks: the option text is the session's.
  - PM-0006-12 becomes a check, reported against VISION_V2 §4's "about 100 ms", and decides nothing. A surprise goes to
    Mustafa.
  - "Falkenstein" appears only as his words about what his check found there.
- **HZ-OQ1: (b), answered in part.** The answered part: option (a), the reference server stands ("I checked it can be
  ordered there"). Still open: whether the client VM ("the CAX21 (or another client VM)", his chosen option), SH's CX33
  and MV can be ordered in Nuremberg. That is checked in the proof test's plan.
  - This follows r4's COUNTS rule and r5's OQ-0003-1 precedent.
  - It no longer blocks the drafts or the start of the proof test's plan.
  - R6-HOST-OQ4 (the root servers, read before the goal budget is fixed) and OQ7 (the IPv4 price) stay.
- **HZ-PRICE: approved, widened.**
  - The drafts use only R6-HOST-F15's prices excluding VAT: CCX23 0.1386, CCX13 0.0697, CAX21 0.0176, CX33 0.0144.
  - His "about €0.020/h incl. VAT" is quoted as his, once.
  - The screenshot's figures and the option text's "€0.083/h" are the session's: never quoted as his.
  - No VAT rate: the session's 86.49 × 1.21 stays out of the docs.
- **HZ-RECORD: approved.**
  - The weighed options (G2, OQ-0006-20's options, OQ-0006-17's "with OQ-0006-1's console check"), the examples and
    Appendix A's record rows keep their words, with at most one pointer.
  - Appendix A's status cells ("open: OQ-0006-1", "open: OQ-0006-2", 1511–1517) are status, not record: they change
    to "answered in part" and "answered".
- **HZ-COUNTS: corrected.**
  - 115 numbered, 96 open; ADR 0006: 21 numbered, 17 open (-2 answered, -1 answered in part). The plan's 95 and 16 are
    one too few.
  - Pass marks: 341 (no row added or dropped; rows change their machines).
- **HZ-VERIFY: approved, with condition 11.** His stop holds: no verifier, no skeptic. The quote script runs against
  all six inputs files, not four.
END RULINGS

### Conditions
1. **The work list comes from a script, not the plan's map** (as r4's condition 1).
   - The search covers the nine documents for: CX33; OQ-0006-1; OQ-0006-2; R6-HOST-OQ1; R6-HOST-OQ2;
     nbg1|hel1|fsn1|Falkenstein|Nuremberg|Helsinki; and "every server row|every native server mark|every server mark".
   - Each hit gets a disposition from its author: a FIX, or "no change: <reason>". That includes the places listed in
     "What I checked myself" that the map misses.
   - The report's §9 gets one row for each place.
2. **Two game servers** (the correction and his first answer).
   - **RS:** his words, quoted: "The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz
     margin and rooms per VM. Each run's plan says which server it needs."
   - **The new code DS:** the CCX13 (dedicated x86, 2 vCPU, 8 GB; 43.49 EUR a month, 0.0697 an hour, excluding VAT,
     R6-HOST-F15), for development and "does it work" runs. Source: his answer to the session's question 1 (the option
     he chose, in the session's wording). Never call it "the development server" alone: ADR 0005 and ADR 0002 use that
     name for the local server.
   - **Where DS goes:** the keys of ADR 0001 and ADR 0006, and of ADR 0004 if any of its rows can run there; SPIKE_PLAN's
     machines paragraph; PM-0001-1 (the rig); PM-0004-1 (the machines recorded); PM-0006-18 (DS's hours at 0.0697);
     PM-0006-11. PM-0006-11's mark changes:
     - every measurement row ran on RS (SH where its row says);
     - every other server row ran on the server its run's plan names;
     - DS is recorded the way RS is.
   - **The machine columns stay as they are.** ADR 0001's key gets one sentence (test-engineer), and ADR 0006's key
     points to it: "A row whose column names RS runs on RS when it is one of the three measurement kinds; any other row
     runs on the server its run's plan names; a time-limited row that its plan puts on DS and that fails there is re-run
     on RS before it counts as failed." Mark it as the advisor's reading, 2026-11-03: DS has 2 vCPU, where RS's key
     gives the transport three.
   - **Part 5 and 0006:753** (infra-engineer): "every native server mark …" becomes "every server time mark …", or
     equivalent.
   - **The correction's first sentence** is not quoted in the drafts. The appendix carries it byte for byte, with the
     answer that resolved it.
3. **Location** (his second answer): Nuremberg (`nbg1`) for the game servers and the simulated clients, labelled as in
   HZ-LOCATION.
   - OQ-0006-2's options stay as the record.
   - Part 5, :632–634, :1189, PM-0006-11 and PM-0006-12 say `nbg1`. None of them keeps "nbg1 or hel1" or "picked by a
     round-trip measurement".
   - PM-0006-12's target is a VM of the run in `nbg1` other than RS and DS, with ICMP allowed on it only. No CX33.
4. **LG.**
   - His words, quoted: "Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead
     of the CX33, so their build targets aarch64."
   - In Nuremberg: whether it "(or another client VM)" can be ordered is checked in the proof test's plan (his chosen
     option, the session's wording). The calibration run still sets how many.
   - The drafts say nothing more. A note for the proof test's plan, not for the drafts: if a CX33 can be ordered there,
     that plan may weigh it as the client VM (one x86 build, so condition 5 costs nothing) and put the choice to him
     with its cost cap.
5. **Arm in ADR 0001** (netcode-architect; test-engineer writes the row text).
   - The rule: no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm. That agreement is unproven:
     PM-0004-73 runs on x86 native and Wasm only, and PM-0004-26 adds AArch64 only before Arm hosts are picked.
   - It applies to every row that compares a result an LG client computes with the server's: at least PM-0001-54 (the
     landings) and PM-0001-48 (the shooter's view). It applies to -17, -18 and -19 too if their client side does any
     float work; the author confirms it (-17's bite names "a client quantiser").
   - Each such row picks one of two ways:
     - (i) it takes its verdict from L1, an x86-64 client or a browser (Wasm), with LG's clients as load only;
     - (ii) a mismatch seen on LG is replayed on x86-64 (the same inputs on L1) before it counts. A mismatch that shows
       only on aarch64 is reported as a cross-architecture finding and decides nothing.
   - "Part 7's rules make them agree" is not enough on its own.
   - The rule holds whatever LG turns out to be. No row is added.
6. **ADR 0004** (rust-reviewer).
   - An Arm load generator is not "an ARM host" in part 7's sense: it serves no player and runs no room (the advisor's
     reading, 2026-11-03). S7, PM-0004-26 and the A64 key stay as they are.
   - At most one sentence, pointing to condition 5's rule in ADR 0001.
   - The CX33 code stays. Its key says it is measured only if the proof test's plan finds it orderable in Nuremberg,
     and PM-0004-4's CX33 part runs only then.
7. **ADR 0002** (engine-architect a).
   - "No arm64 leg" stays: RS, DS and SH are all x86-64.
   - At most one sentence: the bot clients' aarch64 build is the netcode proof test's own. The playtest soak's bot VM
     is named when the first soak is planned. If that VM is Arm, `devtools` gains an aarch64 build target in that
     change-set (a build, not a determinism leg), and the soak's desyncs follow condition 5's rule.
   - The plan's "the LG binary is throwaway spike code" misses the soak at 0006:373.
   - 0006:373 (infra-engineer) becomes "on a client VM beside the reference server, never on it (as spike 2's load
     generators)", priced at the CAX21's rate: 3 × (0.1386 + 0.0176) = 0.47 EUR.
8. **SH, K3 and the budgets**, as HZ-SH and HZ-STAGING rule.
   - PM-0001-27, 0006:1134 and PM-0004-4: SH's part runs only if SH exists.
   - K3 (0006:352, :768, :1448) becomes conditional on SH being measured.
   - 0001:1293 reads "on the CCX23 and SH (ADR 0006)".
   - 0006:1435: "on the staging VM's plan (proposed a CX33, part 4)".
   - MV as HZ-MV rules.
9. **His words.**
   - His words come only from the six inputs files. The option texts in HETZNER-ANSWERS are labelled as the options he
     chose, in the session's wording.
   - The screenshot description is never presented as his words.
   - Every reading is labelled "(the advisor's reading, 2026-11-03)".
   - Times come from the inputs files: about 18:26, 18:32 and 18:34. The addendum's "about 18:40" is the session's
     note, not a time the inputs files give.
10. **The routing.** It is the plan's, plus:
    - infra-engineer: condition 1's ADR 0006 places, including the Appendix A status cells;
    - test-engineer: DS in the keys, PM-0001-1, PM-0004-1, PM-0006-11, -12 and -18, and condition 2's sentence;
    - engine-architect (b): SPIKE_PLAN :176, :281 and :513, and README's counts at lines 21 and 23–25.
11. **Verification (his stop).** No fixcheck and no skeptic. The session runs:
    - the write guard, dispositions, glossary, tables and counts;
    - quotes against all six inputs files;
    - a grep that every "CX33" left is his words, a record, a NOTE, or a proposal that depends on the order check in
      Nuremberg;
    - a grep that no "Falkenstein" or "fsn1" is left outside his words and the options' record;
    - a grep that no "nbg1 or hel1" or "picked by a round-trip measurement" is left as a live rule;
    - a check that PM-0006-11 no longer says "every server row ran on", and that DS is defined in every key that uses
      it;
    - the counts 115/96, ADR 0006 21/17, and 341;
    - the gate.

    My final-head review is the merge rule's review, not a round (r5's condition 8).
12. **Close as r4's conditions 7 to 9 and r5's condition 9.** The appendix also carries, byte for byte, HETZNER-CHECK,
    HETZNER-CORRECTION (with the screenshot description, marked as the session's) and HETZNER-ANSWERS, and this verdict.
    docs-keeper (high) updates STATUS:
    - the check, the correction and the answers;
    - the new counts;
    - "For Mustafa" without the console check;
    - STATUS:117's record kept, with the new words beside it.
13. **Models and effort:** Opus for every agent. The authors work at xhigh, docs-keeper at high. One round, each author
    on its own text.
END CONDITIONS

### Findings (ranked)
1. HZ-SH (a) and HZ-STAGING (a) would run the room server on Arm. That goes against ADR 0004 part 7, R6-HOST-REC2 and
   the option he chose ("no second build target for the room server").
2. The map misses about 20 places that the correction and the location answer change. It also misses two CX33 places
   (0006:1268, :1274), though both are routed (condition 1).
3. Relying on "part 7's rules" for aarch64 agreeing is not proven (condition 5).
4. HZ-OQ1 (a) and the count of 95 close a question whose order check moved to the proof test's plan. The count is 96
   (ADR 0006: 17).
5. The playtest soak (0006:373) carries LG beyond the spike, so the aarch64 build isn't only "the spike's own"
   (condition 7).
6. "Development server" is already taken in ADR 0005 and ADR 0002 (condition 2).
7. HZ-VERIFY names four inputs files; there are six. HETZNER-ANSWERS's option texts are the session's (condition 9).
END FINDINGS

### Decisions needed
None new from this plan. The round can run in full now: nothing is pending from Mustafa. MACHINES-TIMING (r5) stays in
the report.

### For Mustafa
1. Nothing new from this round. The console check is done, and it leaves the list.
2. Still on the list from r5: register shamblefrontlines.com at Cloudflare before the netcode proof test; borrow and
   name the laptop, desktop and Mac; OK each run's cost cap when its plan reaches you. That plan also checks which
   client VM can be ordered in Nuremberg.
3. Once I have approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this session's chat.
   It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

Status: DONE

## AMENDMENT (verbatim)

### Notes (amendment)
- 18:49 read HETZNER-CORRECTION-2 (his words, about 18:48) and HETZNER-ANSWERS-2 (about 18:49; "No, drop the CCX13", option text the session's: "Every server run uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21."), and the plan's Addendum 2 (7 points). The round launched on the first verdict wrote no block (session's word).
- His "a CAX21 for the 48 simulated players" is singular and says 48; the rooms-per-VM rows run 8 rooms at N = 48 (384 bot sessions: PM-0001-27, PM-0006-13, -14, -22, -24) and the 0006 key already says 384 sessions "may need more than one 4-vCPU VM" (proposed). Money, his: routed through each run's cap (SPIKE-COST). Non-blocking escalation LG-COUNT.
- Rows with SH, MV or CX33 in the machine column (script): 0001 PM-0001-1, -16, -27; 0004 PM-0004-1, -4; 0006 PM-0006-11, -12 (CX33 throwaways), -13, -14 (SH); -16, -22, -23, -24 (MV). None runs on SH or MV alone, so no row drops: 341 stays.
- OQ-0006-1 "Carries: R6-HOST-OQ1, R6-HOST-OQ4, R6-HOST-OQ7": closing it must not orphan OQ4 (root servers before the goal budget) and OQ7 (IPv4 price); Appendix A rows 1514, 1517 say "open: OQ-0006-1".
- With the CX33 not orderable in Falkenstein, part 8's lean case (production CX33 + staging CX33) and the staging VM's proposed CX33 rest on a machine his check found unorderable in the test servers' location: a money choice for later (before slice 3), so a numbered open question is the honest home, not a label.
- VMs created fresh each run day: new addresses AND new SSH host keys; capacity on the day is not guaranteed (ADR 0001:756's failure row stays live).
- 18:53 composing the amendment.

### AMENDED VERDICT: APPROVE WITH CONDITIONS (replaces the rulings and conditions above where they differ; read alone)

Plan revision 6 with Addendum and Addendum 2, v2-decision-drafts, PR #33 (draft), branch tip 898ba05, base main
9e7db28. Advisor agent, 2026-11-03, about 19:00.

**Why, in one line:** his second correction and answer simplify the rig to two machines in Falkenstein, a CCX23 and a
CAX21. So DS goes, SH and the separate metrics VM go, and OQ-0006-1 and -2 are answered. Three things still need care
in the drafts: the staging server and the lean budget now rest on a machine that can't be ordered there, "a CAX21 for
the 48 simulated players" is singular while some rows run 384 bot sessions, and every run day brings new addresses and
host keys.

**TL;DR**
- The rig is RS (the CCX23) and LG (the CAX21), both in Falkenstein (`fsn1`), in his words. Every server row runs on
  RS. No DS, no SH, no separate MV.
- OQ-0006-1 and OQ-0006-2 are answered. One new open question, OQ-0006-22: the staging server's x86 plan (money,
  Mustafa, before slice 3). Counts: 116 numbered, 96 open (ADR 0006: 22 and 17). Pass marks stay at 341.
- The Arm rules for LG stand (ADR 0001's condition 5, ADR 0004, ADR 0002).
- The drafts assume no fixed address. Every address-bound setting is made on the run day.
- One non-blocking question for Mustafa (LG-COUNT), answered when the first run's plan reaches him.

### What I checked myself (amendment)
- Both new inputs files, read in full. In HETZNER-ANSWERS-2, his words are his choice; the option text is the
  session's.
- The plan's Addendum 2.
- By script, the rows whose machine column names SH, MV or CX33:
  - ADR 0001: PM-0001-1, -16, -27;
  - ADR 0004: PM-0004-1, -4;
  - ADR 0006: PM-0006-11, -12, -13, -14, -16, -22, -23, -24.

  None of them runs on SH or MV alone, so no row drops.
- OQ-0006-1's "Carries" field (0006:810) and Appendix A at 1511–1517.
- PM-0006-13 and -14: 8 rooms at N = 48.
- The 0006 key's 384-session note (1194–1196).
- ADR 0001:756, the failure row "No Hetzner capacity in the EU".
- Not run: the gate (nothing edited). Not checked: the console. Both machines being orderable is his statement.

### Amended rulings (all mine or settled by his words; one non-blocking escalation, LG-COUNT)
- **HZ-LOCATION (OQ-0006-2): answered by his words.**
  - Quote: "both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the
    48 simulated players". Label: "Answered by Mustafa (2026-11-03)".
  - `fsn1` in every live place.
  - "Nuremberg" stays only in OQ-0006-2's options (the record) and inside his quoted words. "Helsinki" and `hel1`
    likewise.
  - His answer to the session's question 2 ("Nuremberg (nbg1)") is superseded. It is never quoted in the drafts; the
    appendix carries it.
  - Production: part 5 says the first fleet starts in the test servers' location unless playtesters' round trips
    (R6-HOST-OQ2, still open) say otherwise (the advisor's reading, 2026-11-03). His words name only the test servers.
- **HZ-OQ1 (OQ-0006-1): answered** (not "in part": nothing is left to check).
  - His words: "I created both once to try the console and deleted them right away", and, from his first note, "in
    Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can."
  - Option (a) holds: the reference server stands.
  - The entry keeps carrying R6-HOST-OQ4 (the root servers, read before part 8's goal budget is fixed) and R6-HOST-OQ7
    (the IPv4 price), each still open with its own trigger as written. Appendix A's status cells say so; they don't say
    "answered" for OQ4 and OQ7.
  - "Created fresh on run day" means capacity on the day isn't guaranteed. ADR 0001:756's failure row and OQ-0006-1's
    option (b) route stay as the fallback (the advisor's reading, 2026-11-03).
- **DS: dropped.** It is his choice ("No, drop the CCX13"; the option text, the session's wording: "Every server run
  uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21.").
  - Nothing of DS goes into the drafts: no code, no CCX13, no key sentence, no re-run rule.
  - PM-0006-11's mark "the machine every server row ran on" stays: it is true again. Part 5's "every native server mark
    is measured on one vCPU of it" stays.
  - His first correction ("used only for the measurement runs") and his CCX13 answer are not quoted in the drafts. The
    appendix carries both, with the later answer that superseded them.
- **HZ-SH: SH leaves the proof test; K3 waits.** His check makes the CX33 unorderable in Falkenstein, and "the rig is
  just the CCX23 and the CAX21" (the option he chose).
  - Never a CAX21 as SH (ADR 0004 part 7; R6-HOST-REC2).
  - No other shared x86 plan is checked now. K3 (a shared fleet) waits for a later measurement of a shared x86 plan
    that can be ordered in the fleet's location.
  - G2's option row stays as weighed, with one pointer.
- **HZ-MV: no separate VM.** MV becomes a role on the CAX21, never on RS: Prometheus runs on LG's VM, inside LG's 80 %
  CPU rule (the calibration run includes it).
  - It still stands in for the staging game server.
  - The proof test's plan reads Prometheus's Arm64 build from its own release page. The drafts make no claim about it.
  - If LG-COUNT gives LG more than one CAX21, the plan says which one carries MV.
  - The firewall rule of PM-0006-16 opens the exporter's port to that VM's address of the day only.
- **HZ-STAGING: new open question OQ-0006-22** (infra-engineer).
  - The question: which x86 plan runs the staging game server, now that the proposed CX33 cannot be ordered in
    Falkenstein (Mustafa's check, 2026-11-03)?
  - Why it matters: staging runs the room server, and an Arm plan is an Arm host (PM-0004-26 first). Part 8's lean case
    rests on the CX33's price.
  - Options:
    - a CX33 (8.99) only where it can be ordered;
    - a CCX13 (43.49, R6-HOST-F15) if it can be ordered in the fleet's location;
    - a shared x86 "Regular Performance" plan (CPX32 35.99, R6-HOST-F15), whose locations are not researched.
  - Decider: Mustafa (money), with the Advisor. When: before slice 3.
  - Part 4 points to it. Part 8's lean case is labelled: it "rests on the CX33's price for production (K3) and staging;
    the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so it holds only where a shared x86 plan can be
    ordered in the fleet's location and K3 passes" (the advisor's reading, 2026-11-03).
  - The reference case and PM-0001-63's formula keep 8.99, labelled "the staging VM's placeholder price (the CX33's,
    R6-HOST-F15); its plan is OQ-0006-22".
  - 0006:1435 reads "on the staging VM's plan (OQ-0006-22)".
  - The lobby's "for example a CX33" stays in OQ-0006-3, an example.
- **PM-0006-12: a check, without DS or a throwaway VM.**
  - Target: the run's CAX21 in `fsn1`, with ICMP allowed on it only (never on RS), pinged from ML and MP.
  - Reported against VISION_V2 §4's "about 100 ms". It decides nothing; a surprise goes to Mustafa.
  - The row stays; its title loses "each candidate location" and "feeds OQ-0006-2".
- **New addresses each run day (his words: "They'll be created fresh on run day, so their IP addresses will be new
  then.").**
  - The drafts write no address and assume none is fixed. The run records the day's addresses with its results.
  - Every address-bound setting is made on the run day:
    - the per-address connection cap raised on LG's addresses;
    - any firewall rule that names a source (PM-0006-16's scrape);
    - the WebSocket leg's DNS record on shamblefrontlines.com, set before its certificate is issued.
  - New VMs also bring new SSH host keys. How a session checks a host key before trusting it is the proof test's plan's,
    with security-auditor (r4's SPIKE-MACHINES (a)).
  - The WebTransport certificate is pinned by hash, not by address, and is made per run.
  - This is the advisor's reading, 2026-11-03, written as at most one sentence in ADR 0006's key and one in ADR 0001's.
- **HZ-PRICE:**
  - Prices excluding VAT come from R6-HOST-F15 only: CCX23 0.1386 and CAX21 0.0176 an hour. The CX33's 0.0144 appears
    only in records and placeholders.
  - His "about €0.020/h incl. VAT" is quoted as his.
  - The screenshot's figures and the option texts are the session's. No VAT rate.
- **HZ-RECORD:** as ruled above.
- **HZ-COUNTS:** 116 numbered, 96 open; ADR 0006: 22 numbered (OQ-0006-1 to -22), 17 open (-1 and -2 answered; -22
  new). Pass marks: 341 (rows lose their SH, MV and CX33 parts; none drops).
- **HZ-VERIFY:** his stop holds; no verifier and no skeptic (condition 11). Quotes are checked against all eight
  inputs files.
END AMENDED RULINGS

### Amended conditions (complete; they replace conditions 1 to 13 above)
1. **The work list comes from a script.**
   - The search covers the nine documents for: CX33; CCX13; `DS` (expected 0); SH and MV in machine columns and keys;
     OQ-0006-1; OQ-0006-2; R6-HOST-OQ1; R6-HOST-OQ2; Nuremberg|nbg1|Helsinki|hel1|Falkenstein|fsn1; "every server
     row|every native server mark|every server mark"; and "address" in the machine keys and the operations checklist.
   - Each hit gets a disposition from its author: a FIX, or "no change: <reason>". The report's §9 gets a row for each
     place.
   - It must include:
     - ADR 0006: 119, 189, 244–254, 267, 325–326, 352, 373, 607, 632–634, 713, 715, 753, 768, 799–816, 825, 1073–1077,
       1133–1134, 1189–1199, 1267–1280, 1300, 1364–1370, 1435, 1446–1448, 1499 and 1510–1517;
     - ADR 0001: 1293, 1350–1357, PM-0001-1, -16, -27 and -63;
     - ADR 0004: 1030–1032, PM-0004-1 and -4;
     - README: 21, 23–25, 133 and 134;
     - SPIKE_PLAN: 31–33, 129–130, 176, 281 and 513.
2. **The rig: two machines, his words.**
   - Quote "a CCX23 for the game server and a CAX21 for the 48 simulated players" and "I created both once to try the
     console and deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then."
   - RS: the CCX23 (x86-64 AMD) in `fsn1`. Every server row runs on it.
   - No DS and no CCX13 anywhere.
   - Not quoted in the drafts: his first correction's sentences and his answers to the session's questions 1 and 2.
3. **Location:** `fsn1` in every live place, as HZ-LOCATION rules.
   - OQ-0006-2's options stay as the record, labelled answered.
   - Nothing keeps "nbg1 or hel1", "Falkenstein … only if offered" or "picked by a round-trip measurement" as a live
     rule.
   - Production follows the test servers' location unless R6-HOST-OQ2's playtester round trips say otherwise (the
     advisor's reading, 2026-11-03).
4. **LG:** "a CAX21 for the 48 simulated players" (his words), with "so their build targets aarch64" (his first note).
   - Created in `fsn1` on the run day.
   - The calibration run's 80 % rule stays.
   - A run whose sessions one CAX21 cannot carry under 80 % (the 8-room rows, 384 sessions) names the extra CAX21s in
     its plan, for his OK with that run's cost cap (LG-COUNT below). Until he answers, the drafts say exactly that.
   - The order-check note and the CX33 client-VM note from the first verdict go.
5. **Arm in ADR 0001** (netcode-architect; test-engineer writes the row text). Unchanged.
   - No pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm.
   - It covers PM-0001-54 and -48, and -17, -18 and -19 if their client side does float work.
   - Each such row picks one of two ways:
     - (i) it takes its verdict from L1, an x86-64 client or a browser, with LG as load only;
     - (ii) a mismatch seen on LG is replayed on x86-64 before it counts. An aarch64-only mismatch is reported and
       decides nothing.
   - No row is added.
6. **ADR 0004** (rust-reviewer and test-engineer).
   - An Arm load generator is not "an ARM host" in part 7's sense (the advisor's reading, 2026-11-03). S7, PM-0004-26
     and the A64 key stay; at most one sentence pointing to condition 5.
   - The CX33 code leaves the key, which says instead: "the shared CX33 is not measured: it cannot be ordered in
     Falkenstein (Mustafa, 2026-11-03)".
   - PM-0004-1 drops CX33. PM-0004-4 drops "and on the cheaper host" and its N = 48 CX33 part. Its author states
     whether R1-ECS-OQ4 and R1-TICK-OQ5 are still decided from RS alone, or whether either stays open.
   - The RS key (1030): `fsn1`, and R6-HOST-OQ1 answered.
7. **ADR 0002** (engine-architect a).
   - "No arm64 leg" stays: every room server runs on x86-64 RS, and the rig's only Arm machine is LG.
   - At most one sentence: the bot clients' aarch64 build is the netcode proof test's own. The playtest soak's bot VM
     is named when the first soak is planned; if it is Arm, `devtools` gains an aarch64 build target then (a build, not
     a determinism leg), and the soak's desyncs follow condition 5.
   - 0006:373 (infra-engineer): "on a client VM beside the reference server, never on it (as spike 2's load
     generators)", 3 × (0.1386 + 0.0176) = 0.47 EUR.
8. **SH leaves, and the rows and texts that carried it change.**
   - ADR 0001: PM-0001-1, -16 and -27 lose SH. 0001:1293 reads "on the CCX23 (ADR 0006)". The key drops SH.
   - ADR 0006:
     - PM-0006-11 creates "the CCX23 … in `fsn1`" and records it; there is no SH.
     - PM-0006-13 and -14 run on RS only, and -13's SH-only bite goes.
     - The key drops SH.
     - 119, 244, 254 and 713 no longer measure a shared CX33 beside RS. They say K3 waits, with one pointer to his check.
     - K3 at 352, 768 and 1448 waits for a later measurement of a shared x86 plan in the fleet's location.
     - G2's row stays as weighed, with one pointer.
   - SPIKE_PLAN:30–31: "SH, its shared neighbour" goes.
9. **MV and the metrics.**
   - MV is defined in ADR 0006's key as "the metrics role: Prometheus on LG's CAX21, never on RS, inside LG's 80 %
     rule, standing in for the staging game server" (the advisor's reading, 2026-11-03).
   - PM-0006-16, -22, -23 and -24 keep MV as that role. PM-0006-16's firewall rule names the day's address of that VM.
   - SPIKE_PLAN:33 says the same.
   - 0006:1073–1077 (OQ-0006-20's options and interim): the record keeps its words and gains one pointer.
10. **Staging and budgets:** OQ-0006-22 as HZ-STAGING rules (infra-engineer: the entry, part 4's pointer, part 8's
    labels, 1435). README gets its row (engine-architect b). PM-0001-63's label: test-engineer.
11. **PM-0006-12 and new addresses**, as the rulings say.
    - test-engineer: PM-0006-12, and one sentence in ADR 0006's key: the day's addresses, the settings bound to them,
      the host keys going to the proof test's plan.
    - netcode-architect: at most one clause in ADR 0001's key, at the connection cap on LG's addresses.
12. **His words.**
    - They come only from the eight inputs files.
    - The option texts in both HETZNER-ANSWERS files are the session's wording. Superseded words (the first
      correction's rule, the CCX13, Nuremberg) are never quoted in the drafts.
    - The screenshot description is never presented as his.
    - Every reading is labelled "(the advisor's reading, 2026-11-03)".
    - Times come from the inputs files: about 18:26, 18:32, 18:34, 18:48 and 18:49.
13. **Routing** (one round, every author on its own text):
    - infra-engineer: ADR 0006's places outside the pass marks, OQ-0006-1, -2 and -22, and Appendix A.
    - test-engineer: the keys and rows of ADR 0001 (PM-0001-1, -16, -27, -63, and -48 and -54 as netcode-architect
      answers), ADR 0004 (PM-0004-1, -4, the keys) and ADR 0006 (PM-0006-11 to -14, -16, -18, -22 to -24, the key).
    - netcode-architect: 0001:1293, condition 5, condition 11's clause.
    - rust-reviewer: condition 6's sentence.
    - engine-architect (a): condition 7.
    - engine-architect (b): README (rows OQ-0006-1, -2 and -22; counts at 21 and 23–25) and SPIKE_PLAN (31–33, 129–130,
      176, 281, 513).
    - Then docs-keeper and the report.
14. **Verification (his stop).** No fixcheck and no skeptic. The session runs:
    - the write guard, dispositions, glossary, tables and counts;
    - quotes against the eight inputs files;
    - greps: no CCX13, no `DS`, no SH in any machine column or key;
    - no live `nbg1`, `hel1`, Nuremberg or Helsinki (only OQ-0006-2's options and his quoted words);
    - every CX33 left is his words, a record, a NOTE or a labelled placeholder;
    - no address written as fixed;
    - counts 116/96, ADR 0006 22/17, 341;
    - the gate.

    My final-head review is the merge rule's review, not a round.
15. **Close as r4's conditions 7 to 9 and r5's condition 9.**
    - The appendix carries byte for byte: the five Hetzner inputs files (CHECK, CORRECTION with the session's
      screenshot description marked as the session's, ANSWERS, CORRECTION-2, ANSWERS-2) and this verdict with its
      amendment.
    - docs-keeper (high) updates STATUS: the check, both corrections and the answers; the counts; "For Mustafa" without
      the console check; STATUS:117's record kept, with his new words beside it.
    - The report's "Decisions needed" holds MACHINES-TIMING (r5) and LG-COUNT.
16. **Models and effort:** Opus for every agent. The authors at xhigh, docs-keeper at high.
END AMENDED CONDITIONS

### Amended findings (ranked)
1. Part 8's lean case and the staging VM rest on the CX33, which can't be ordered in Falkenstein: a money choice for
   later, now OQ-0006-22.
2. "A CAX21 for the 48 simulated players" against rows of 384 bot sessions (LG-COUNT).
3. Run-day VMs bring new addresses and host keys, and capacity on the day isn't guaranteed (condition 11; ADR 0001:756
   stays).
4. Closing OQ-0006-1 must not orphan R6-HOST-OQ4 and OQ7.
5. The SH and MV parts of 13 rows, which the plan's map doesn't list.
6. The first verdict's Arm findings stand (conditions 5 to 7).
END AMENDED FINDINGS

### Decisions needed (for the report; none blocks this round)
- **LG-COUNT**
  - The question: you named "a CAX21 for the 48 simulated players". Some proof-test rows run 8 rooms of 48 bots, which
    is 384 simulated players. If one CAX21 can't carry them under 80 % CPU, may that run add CAX21s?
  - Why it matters: an overloaded load generator makes the run invalid. Each extra CAX21 costs about 0.0176 EUR an hour
    (excluding VAT, R6-HOST-F15).
  - Options:
    - (a) the run's plan adds as many CAX21s as the 5-minute calibration needs, inside the cost cap you OK;
    - (b) one CAX21 only, and the 8-room rows run with as many bots as it carries (they would no longer test 8 rooms of
      48);
    - (c) decide when the calibration reports.
  - Recommendation: (a). The decision lands in the run's plan, with its cost cap, which reaches you anyway. The round
    proceeds with the drafts saying so.
- OQ-0006-22 (the staging server's plan) is yours (money), but not now: before slice 3.

### For Mustafa
1. Nothing to do for this round. The console check is done and leaves the list.
2. LG-COUNT above, when the first run's plan and its cost cap reach you.
3. Still from r5: register shamblefrontlines.com at Cloudflare before the netcode proof test; borrow and name the
   laptop, desktop and Mac; OK each run's cost cap.
4. Once I have approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this session's chat.
   It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END AMENDMENT

Status: DONE (amended)

## AMENDMENT 2 (verbatim)

### Notes (amendment 2)
- 22:25 read ANSWERS-3 (his words, about 19:04), LG-COUNT (about 18:56; option text the session's) and the plan's Addendum 3. Grep "register": ADR 0001:464, :1274; ADR 0006:289-290, 301, 761, 847, 982-984, 1367-1369; README:145; SPIKE_PLAN:148-149. macOS runners: ADR 0002 OQ-0002-5 (1027-1031: free while public, R5-CI-F17 + infra-engineer's read), ADR 0006 part 8 (360-363), Q13 (694), :890. Short on time (deadline 22:38): rulings first.

### Rulings (amendment 2; everything in AMENDMENT stands unless changed here)
- **MACHINES-TIMING: answered by his words (c, "with one step first").** Quote in full, from ANSWERS-3, in SPIKE_PLAN's
  machines paragraph (engine-architect b); the other places point to it.
  - **The step:** the netcode proof test's plan first checks "whether GitHub's macOS runners (real Safari, free while
    the repository is public) have a Safari new enough for WebTransport and can reach the test servers".
  - **Branch 1 (they can):** "that is the Safari check, and I borrow the Mac only for the look proof test".
  - **Branch 2 (they can't):** "I borrow the Mac before the netcode proof test."
  - **Laptop and desktop:** "rough checks first, then a short re-run once they're named". These words replace the
    advisor's reading in SPIKE_PLAN items 1, 3 and 4 where it says the same thing. Each spike's plan still lists the rows
    that run before the machines are named. Nothing is judged on a rough check.
- **May a GitHub macOS runner stand as MAC?** Yes, for the netcode proof test's Safari rows only, under branch 1 (his
  words). It must show first, recorded in the run's results:
  - (i) the runner image's Safari version on the day, at least the version R2-TRN-F7 gives for WebTransport (26.4);
  - (ii) real Safari driven through safaridriver, not a WebKit build;
  - (iii) from the runner: one WebTransport session to RS that opens and carries datagrams, and one WebSocket session on
    the domain.

  Limits (the advisor's reading, 2026-11-03):
  - The runner stands in only for yes-or-no and count rows (PM-0001-28's four-browser path and library pick, PM-0001-14,
    PM-0001-25's Origin, PM-0006-15). A row whose limit is a time, or which ADR 0003's look proof test needs, stays on
    the borrowed Mac.
  - The workflow that runs it is part of the proof test's plan: security-auditor and infra-engineer review it, and any
    `.github/` change in this repository goes to Mustafa (escalation category 3).
  - The later spikes' Safari rows are each spike's own plan's, in the same way.
  - Where it goes: ADR 0001's key, at MAC (test-engineer), one sentence with the step, (i) to (iii) and the limits;
    ADR 0006's key points to it (test-engineer).
  - ADR 0002 OQ-0002-5 and ADR 0006 part 8 already read the runner's price (free while public): no change, or at most
    one pointer (engine-architect a; infra-engineer). OQ-0002-5 stays open: it asks about pull requests, not the spike.
- **Counts:** unchanged. MACHINES-TIMING was a decision in the report, not a numbered question. OQ-0003-1 stays "answered
  in part" (the actual machines are still to be named). 116 numbered, 96 open; ADR 0006: 22 and 17; 341.
- **DOMAIN: registered.** His words: "DOMAIN: done. shamblefrontlines.com is registered at Cloudflare."
  - His earlier quotes ("a domain I register", "which I register at Cloudflare") stay as his words where they are
    quoted, with one pointer to "registered (Mustafa, 2026-11-03)".
  - Live sentences ("he registers …", "the domain he registers", "before it" / "before the netcode proof test") become
    "registered".
  - Who writes what:
    - ADR 0006:289–290, 301, 761, 847, 982–984 and 1367–1369 (infra-engineer). The milestone at 1367–1370 loses the
      registration as a to-do.
    - ADR 0001:464 (netcode-architect) and :1274 (its author: test-engineer if in the checklist or key).
    - README:145 and SPIKE_PLAN:148–149 (engine-architect b).
    - STATUS and the report (docs-keeper): "For Mustafa" loses the registration.
  - The DNS record and certificate for each run day stay as AMENDMENT condition 11 says.
- **LG-COUNT: answered.** He chose the session's option "Add as needed (Recommended)", whose text is the session's
  wording: "The run's plan adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap you OK
  for that run." AMENDMENT condition 4's last sentence now reads: "A run whose bots one CAX21 cannot carry under 80 %
  adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run
  (LG-COUNT, the option he chose, 2026-11-03)." When the run has more than one CAX21, its plan says which one carries MV
  (AMENDMENT, HZ-MV).
- **Inputs:** now ten files for the quote check (STOP, DESIGN, ANSWERS-2, HETZNER-CHECK, HETZNER-CORRECTION,
  HETZNER-ANSWERS, HETZNER-CORRECTION-2, HETZNER-ANSWERS-2, LG-COUNT, ANSWERS-3). The option texts in HETZNER-ANSWERS,
  HETZNER-ANSWERS-2 and LG-COUNT are the session's. The appendix carries all ten files' Hetzner and answer texts byte
  for byte (the new two included).
- **"Decisions needed" after this:** none open from PR #33. MACHINES-TIMING and LG-COUNT are answered and leave it.
  OQ-0006-22 (the staging server's plan: Mustafa, money, before slice 3) is listed as a later item, not a decision now.
- **"For Mustafa" after this:**
  1. Borrow and name the laptop and desktop (rough checks first, then a short re-run); the Mac as branch 1 or 2 decides.
  2. OK each run's cost cap.
  3. The `.github/` workflow for the Safari check, if the proof test's plan puts it in this repository.
  4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
     session's chat (squash only).
- Routing: these items join AMENDMENT condition 13 for the same authors. One round, xhigh; docs-keeper at high.
END AMENDMENT 2

Status: DONE (amended twice)

## 13. Mustafa's words: the ten inputs files, byte for byte

Each file as the session saved it before any edit (its first line is the session's header; the option texts in the
three answers files are the session's wording, which he chose; the screenshot description is the session's):

`inputs/STOP-ANSWERS-mustafa-2026-11-03.md`:

~~~~text
# Answers to the stop (Mustafa, 2026-11-03, about 12:28 Amsterdam, in the session's chat; word for word)

Answers to the stop (Mustafa, 2026-11-03; the Advisor's calls are marked).

SPIKE-ORDER: (b) accept with one change. The six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time: the netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike, then the look proof test once its models exist. The preview hosting check may run alongside any of them. Reason: seven usage-limit cuts in three days.
TICK-60-HEADROOM: confirm, with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz.
TICK-60-SETUP (Advisor): (a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.
NET-PROFILES (Advisor): yes. "100 ms" is P2: 100 ms round trip, ±20 ms jitter, 2 % loss (VISION_V2 §4's "about 100 ms of ping"). Latency and jitter split half each way; loss as P2 defines it.
REWIND-CAP: no value now. Rule: the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins. Aim for the smallest input buffer that holds.
SPIKE-MACHINES: a separate Hetzner project. I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run. Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register. Test PC: my own, in Brave; the CX33 runs the simulated clients.
LOOK-TEST-INPUTS: yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough.
TOY-BOX: reword both to point at LOOK_V2, in a small kit change-set after PR #30 is updated, together with the checkpoint anti-waste rules.
OQ-0002-8 (Advisor): (a).
OQ-0005-1 (Advisor): the client's data, keyed by the same id; the content check fails if a simulation id has no client entry.
Also the Advisor's, at the stop: OQ-0001-14: yes, a VISION_V2 §3 or §4 trace counts, marked as such. ROOM-CRASH: end the match, as proposed, and keep the crash and match records. OQ-0003-17: at most 6 of 600 presented frames may miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as diagnostics; I confirm this reading. OQ-0003-20: judge after 15 minutes of play. OQ-0005-AB12: tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read. OQ-0005-AB13: scripting comes back only when a designed ability can't be built from data.
LATER: a design session after the netcode proof test.

Next: record these, mark PR #33 ready, and once it's approved, squash-merge it so the 31 MB of scratch build output never reaches main's history. Then the netcode proof test as its own change-set, plan first.
~~~~

`inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md`:

~~~~text
# Design answers (Mustafa, 2026-11-03, about 17:28 Amsterdam, in the session's chat; word for word)

Design answers from Mustafa (2026-11-03). 
Record them in the README's design questions and in STATUS (in PR #33, or a docs follow-up if it's merged). They don't change the spike order.
- DOMAIN (OQ-0006-15): shamblefrontlines.com (or the closest free variant), bought by Mustafa at Cloudflare.
- Reference machines (OQ-0003-1): not Mustafa's RTX 5080 laptop. Minimum: a typical student laptop with integrated graphics. Recommended: a mid-range card, roughly RTX 3060/4060 class. Both borrowed.
- Carry slots (OQ-0005-AB1): a looted weapon replaces the main (class) weapon; the sidearm stays as backup. Assumed, to confirm: the class's own abilities stay.
- Looted ammo (OQ-0005-AB2): still open (infinite, limited, or supply lines by an engineer class). The Advisor's lean: limited but generous, refilled at the barracks and supply points.
- Levelling (OQ-0005-AB3): XP from kills, assists and objective work while using that unit or weapon; fast early levels; a track that's behind the player's best levels faster.
- Name: the in-match picks are "amplifications" ("amps").
- WEAPON-UNLOCKS (OQ-0005-AB5): leaning towards class variants, with more variants added over time. Not final.
- Banzai vs Riot Shield (OQ-0005-AB11): a tall, slightly curved Riot Shield blocks the Banzai blade from the front, so the Warrior must flank; the Conscript's Molotov punishes the shield hard. LOOP_V2's counter triangle; numbers tuned later.
- Upgrade visibility: enemies can see a player's build.
- Spawn protection (OQ-0005-AB14): 3 s, against spawn killing. Firing ends it early (the Advisor's pencil; Mustafa may veto).
- Fragments as damage: visual only for now. The server sends one blast event, the browser draws the fragments, and the spray matches the real damage radius.
- Still open: faction asymmetry (Mustafa works on the unit list first) and the war around the matches.
~~~~

`inputs/ANSWERS-2-mustafa-2026-11-03.md`:

~~~~text
# Answers to the escalations, COMMUNITY-SERVERS, and the verification stop (Mustafa, 2026-11-03, about 17:31 Amsterdam, in the session's chat; word for word)

Answers (Mustafa, 2026-11-03):
- REF-MACHINES: minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed. I name the actual machines before the look proof test; the netcode proof test doesn't measure client frames. Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.
- LOOK_V2-§8: yes, keep it as it is, with Blender for the test models only. When the asset research's export part reports, §8 says "Spline or Blender".
- SPIKE-COST: one cap per run, from its planned hours, written in that run's plan for my OK. I create and delete the VMs myself.
- DOMAIN: shamblefrontlines.com, which I register at Cloudflare.
- COMMUNITY-SERVERS (new; keep the door open, don't design it): players may one day host their own servers, possibly with revenue such as official private servers they rent. Add it as an open question to ADR 0006 and the README. The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only.
- Verification: after e2b, e3b and their fix round, stop. No further re-check rounds; anything left goes into the PR as notes. The spikes are the real test, and we've had ten usage cuts.
~~~~

`inputs/HETZNER-CHECK-mustafa-2026-11-03.md`:

~~~~text
# The Hetzner console check (Mustafa, 2026-11-03, received about 18:26 Amsterdam, in the session's chat; word for word)

Hetzner check (Mustafa, 2026-11-03): in Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can. Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead of the CX33, so their build targets aarch64. The game server stays a CCX23 (dedicated, x86) in Falkenstein; I checked it can be ordered there. Update SPIKE-MACHINES and every place the drafts name the CX33.
~~~~

`inputs/HETZNER-CORRECTION-mustafa-2026-11-03.md`:

~~~~text
# The correction to the Hetzner note (Mustafa, 2026-11-03, received about 18:32 Amsterdam, in the session's chat; word for word)

Correction to my Hetzner note (Mustafa, 2026-11-03): the game server is a CCX23 for development and "does it work" runs. The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz margin and rooms per VM. Each run's plan says which server it needs.

## The screenshot he attached (the session's description, not his words)
Hetzner Cloud's "Create a server" page. Type: three groups. Shared Resources: "Cost-Optimized" (badge "Limited
availability"; "Cost-efficient on older hardware generations, with limited availability."; architecture "x86
(Intel®/AMD)" or "Arm64 (Ampere®)", neither selected) and "Regular Performance" ("Higher CPU performance based on newer
hardware generations."; architecture "x86 (AMD)"). Dedicated Resources: "General Purpose" (selected; "Provides dedicated
vCPUs on the latest available hardware generation at the location."; architecture "x86 (AMD)"). Its table: CCX13 2 vCPU
8 GB 80 GB 20 TB €0.083/h €52.02/mo; CCX23 4 vCPU 16 GB 160 GB 20 TB €0.167/h €104.05/mo (selected); CCX33 8 vCPU
32 GB 240 GB 30 TB €0.268/h €167.57/mo; CCX43 16 vCPU 64 GB 360 GB 40 TB €0.535/h €333.95/mo; CCX53 32 vCPU 128 GB
600 GB 50 TB €1.035/h €645.52/mo; CCX63 48 vCPU 192 GB 960 GB 60 TB €1.655/h €1,032.72/mo. The order summary on the
right: "CCX23 Dedicated CPU" (Type), "Nuremberg" (Location), "Ubuntu 26.04" (Image), "IPv4, IPv6" (Networking), SSH keys
not yet set; "1 SERVER €104.05/mo", "1 IPV4 €0.61/mo", "TOTAL €104.65/mo". The page states no VAT; the research's
CCX23 price is 86.49 EUR a month excluding VAT, IPv4 included (R6-HOST-F15), and 86.49 × 1.21 = 104.65 (the session's
arithmetic; the VAT rate is not stated on the page).
~~~~

`inputs/HETZNER-ANSWERS-mustafa-2026-11-03.md`:

~~~~text
# Mustafa's answers to the session's two questions on his Hetzner correction (2026-11-03, about 18:34 Amsterdam; the questions and the options he chose, word for word as the question tool returned them)

Question 1 (the session's): Your Hetzner correction says "the game server is a CCX23 for development and 'does it work' runs", then "The CCX23 (dedicated) is used only for the measurement runs". Which machine did you mean for the development and "does it work" runs? (The measurement runs stay on the CCX23.)
His answer: "CCX13 (Recommended)", the option described as: "Dedicated x86, 2 vCPU, 8 GB, €0.083/h in your screenshot: the same x86 server build as the CCX23, so no second build target for the room server."

Question 2 (the session's): Your first Hetzner note puts the CCX23 game server in Falkenstein; the screenshot's order summary shows "Nuremberg" selected. Which location should the drafts give the game servers and the simulated clients (they sit in the same location)?
His answer: "Nuremberg (nbg1)", the option described as: "As your screenshot shows; whether the CAX21 (or another client VM) can be ordered there is then checked in the proof test's plan."

(The option texts are the session's wording, which he chose; "(Recommended)" was the session's mark on option 1.)
~~~~

`inputs/HETZNER-CORRECTION-2-mustafa-2026-11-03.md`:

~~~~text
# The second correction to the Hetzner note (Mustafa, 2026-11-03, received about 18:48 Amsterdam, in the session's chat; word for word)

Correction again (Mustafa, 2026-11-03): both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players. I created both once to try the console and deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then.
~~~~

`inputs/HETZNER-ANSWERS-2-mustafa-2026-11-03.md`:

~~~~text
# Mustafa's answer to the session's question on the CCX13 after his second correction (2026-11-03, about 18:49 Amsterdam; the question and the option he chose, word for word as the question tool returned them)

Question (the session's): Your latest note names two test servers in Falkenstein: a CCX23 for the game server and a CAX21 for the 48 simulated players. Earlier you chose a CCX13 for development and "does it work" runs, with the CCX23 used only for the measurement runs (tick timing, the 60 Hz margin, rooms per VM). Does the CCX13 still stand?
His answer: "No, drop the CCX13", the option described as: "Every server run uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21."

(The option text is the session's wording, which he chose.)
~~~~

`inputs/LG-COUNT-mustafa-2026-11-03.md`:

~~~~text
# Mustafa's answer to the session's question LG-COUNT (2026-11-03, about 18:56 Amsterdam; the question and the option he chose, word for word as the question tool returned them)

Question (the session's, from the advisor's amended verdict on plan revision 6): You named "a CAX21 for the 48 simulated players". Some netcode proof test rows run 8 rooms of 48 bots at once (384 simulated players, the rooms-per-VM rows). If one CAX21 can't carry a run's bots under 80 % CPU (an overloaded load generator makes the run invalid), may that run add CAX21s (about 0.0176 EUR an hour each, excluding VAT)?
His answer: "Add as needed (Recommended)", the option described as: "The run's plan adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap you OK for that run."

(The option text is the session's wording, which he chose; "(Recommended)" was the session's mark.)
~~~~

`inputs/ANSWERS-3-mustafa-2026-11-03.md`:

~~~~text
# Answers to MACHINES-TIMING, the domain, and the round (Mustafa, 2026-11-03, received about 19:04 Amsterdam, in the session's chat; word for word)

Answers (Mustafa, 2026-11-03):
- MACHINES-TIMING: (c), with one step first. For the netcode proof test's Safari leg, check whether GitHub's macOS runners (real Safari, free while the repository is public) have a Safari new enough for WebTransport and can reach the test servers. If they can, that is the Safari check, and I borrow the Mac only for the look proof test. If they can't, I borrow the Mac before the netcode proof test. The laptop and desktop stay on my timing: rough checks first, then a short re-run once they're named.
- DOMAIN: done. shamblefrontlines.com is registered at Cloudflare.
- Relaunch the authors' round from the checkpoint.
~~~~
