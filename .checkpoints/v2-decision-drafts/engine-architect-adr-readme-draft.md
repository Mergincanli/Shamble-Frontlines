# Decision records (ADRs): index and register

**Date:** 2026-11-01 · **Author:** engine-architect (the synthesis, plan step 6). This file indexes the v2 decision
drafts and keeps their register: the decisions they ask for, the design questions the research left to Mustafa, the
loose ends and where each is resolved, the features whose need lies outside VISION_V2 §6, and the codes the drafts
cite. It decides nothing. The engine on one page: [ENGINE_OVERVIEW](../ENGINE_OVERVIEW.md); the spikes:
[SPIKE_PLAN](../SPIKE_PLAN.md); the template: [0000](0000-template.md).

The drafts replace the legacy entries ADR-001, ADR-002 and ADR-003 (`docs/STATUS.md`, "ADR-001..003 below"; see the
loose-ends register). Line numbers below are the drafts' at assembly (2026-11-01); the fix round may move them.

## The drafts
| ADR | Title | Author(s) | Status | Its spikes ([SPIKE_PLAN](../SPIKE_PLAN.md)) | Open questions |
|---|---|---|---|---|---|
| [0001](0001-multiplayer-netcode.md) | Multiplayer and netcode | netcode-architect; pass marks: test-engineer | Proposed | the netcode proof test; the 60 Hz margin run | 13 (OQ-0001-1 to -13) |
| [0002](0002-code-structure-build-toolchain.md) | Code structure, build and toolchain | engine-architect; pass marks: test-engineer | Proposed | the Rust ↔ TypeScript boundary spike; the foundation's acceptance checks (not a spike) | 7 (OQ-0002-1 to -7) |
| [0003](0003-rendering.md) | Rendering and presentation | graphics-engineer; pass marks: test-engineer | Proposed | the look proof test; the render budget bench | 15 (OQ-0003-1 to -15) |
| [0004](0004-simulation-core.md) | The simulation core | rust-reviewer; pass marks: test-engineer | Proposed | the sim core spike; shares in the netcode proof test, the boundary spike and the content and ability spike | 7 (OQ-0004-1 to -7) |
| [0005](0005-content-abilities-assets.md) | Content data, abilities and assets | engine-architect (frame, content data, asset pipeline); game-designer (ability and effect system); graphics-engineer (the asset pipeline's GPU rules); pass marks: test-engineer | Proposed | the content and ability spike; a share of the look proof test; the asset spike (after the asset-pipeline research); the Feel Lab test | 34 (CD1–CD8, AB1–AB13, AP1–AP3, AS1–AS6, OQ-0005-1 to -4) |
| [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed | the preview hosting check; operations marks in the netcode proof test; an optional share of the boundary spike | 12 (OQ-0006-1 to -12) |

Total: 88 open questions. None of the drafts is signed off; no spike starts before Mustafa and the Advisor approve them
(the drafts brief, §4).

## Decisions the drafts ask for
Every decision code and open question the drafts route to Mustafa or the Advisor, one row each. The first rows are the
ones the brief and the advisor's conditions name; the rest follow by draft. Open questions decided by a spike alone, or
by an agent, are in the drafts and in [SPIKE_PLAN](../SPIKE_PLAN.md), not here.

| Code or OQ | The question, in plain words | Decider | Where |
|---|---|---|---|
| TICK-60-HEADROOM (OQ-0001-1) | Is the 60 Hz margin measured against the 60 Hz server tick budget (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms plus every 60 Hz research mark? | Mustafa and the Advisor (confirm at the stop) | [ADR 0001](0001-multiplayer-netcode.md) part 2, line 840; [GLOSSARY §8](../research/GLOSSARY.md#the-60-hz-margin) |
| TICK-60-SETUP (OQ-0001-2) | How does the stress scene's sim load reach the 60 Hz margin run: (a) the sim core spike's sim inside the proof-test server (recommended), (b) a stand-in replaying measured step times, (c) wait until the real sim runs the stress scene? | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 846; SPIKE_PLAN item 2 |
| "100 ms" reading (OQ-0001-3) | Does the netcode proof test's "100 ms of simulated latency" mean profile P2 (100 ms round trip, with P2's jitter and loss), or 100 ms each way? | the Advisor (who wrote the definition) | [ADR 0001](0001-multiplayer-netcode.md) line 856 |
| ABL-PREDICT (OQ-0001-4) | Does the client predict the player's own abilities? Decided after the Feel Lab test on the Molotov and the Shield. | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 861; [ADR 0005](0005-content-abilities-assets.md) AB-9; SPIKE_PLAN item 10 |
| ASSET-RESEARCH (timing) | The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first spikes) so art production can start sooner? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 76 and the asset pipeline part; SPIKE_PLAN, "The asset-pipeline research" |
| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1148 |
| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1095 |
| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1107 |
| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1115 |
| Reference machines (OQ-0003-1) | Which laptop (the minimum) and which desktop (the recommended) are bought or borrowed as the two reference machines? | Mustafa and the Advisor (money and availability) | [ADR 0003](0003-rendering.md) line 706 |
| Spike order (decision map J6 (5)) | Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the rest accepted? | Mustafa and the Advisor | [SPIKE_PLAN](../SPIKE_PLAN.md), "The spikes, in the proposed order" |
| Trace reading (LE-G; R2-XU-OQ18) | Does a VISION_V2 §3 or §4 target count as a feature's need like LOOP_V2 does (written "need: VISION_V2 §4 only" and listed below)? The session read it so on 2026-10-31. | the Advisor | [ADR 0001](0001-multiplayer-netcode.md) Appendix A (R2-XU-OQ18); "Needs outside VISION_V2 §6" below |
| The design questions | The 38 research questions below that are pure design matters | Mustafa, or a later design session | "Design questions" below |
| OQ-0001-5 | The rewind cap: 200 or 250 ms, and the victim's side? | Mustafa and the Advisor | [ADR 0001](0001-multiplayer-netcode.md) line 867 |
| OQ-0001-7 | Do shots carry their moment inside a tick at 30 Hz? | the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late | [ADR 0001](0001-multiplayer-netcode.md) line 881 |
| OQ-0001-8 | How far does a new projectile move forward on spawn? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 886 |
| OQ-0001-9 | What else does lag compensation cover? | the netcode proof test for cover; for melee, a design call by Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 890 |
| OQ-0001-10 | Are weapon pickups predicted? | Mustafa | [ADR 0001](0001-multiplayer-netcode.md) line 896 |
| OQ-0001-11 | Room codes: length, alphabet and access. | Mustafa (what a host may set, R2-ROOM-OQ5) with the security review's options | [ADR 0001](0001-multiplayer-netcode.md) line 900 |
| OQ-0001-12 | Do far sound events reveal too much? | Mustafa with the game-designer | [ADR 0001](0001-multiplayer-netcode.md) line 905 |
| OQ-0002-1 | Does the v2 editor still open by double-clicking its HTML file? | the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change | [ADR 0002](0002-code-structure-build-toolchain.md) line 800 |
| OQ-0002-4 | Does production ever turn on cross-origin isolation, and can a later login popup live with it? | the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with ADR 0006 (R1-BND-OQ5, only if is | [ADR 0002](0002-code-structure-build-toolchain.md) line 836 |
| OQ-0002-5 | Does real Safari run on pull requests, and does the end-to-end suite need it? | infra-engineer's reading of the macOS runner's time and cost (ADR 0006 for costs; Mustafa for any paid plan), then this record | [ADR 0002](0002-code-structure-build-toolchain.md) line 848 |
| OQ-0002-6 | Which jump reach do the golden tests pin? | the game-designer with Mustafa, from the loop slice on (a rule question) | [ADR 0002](0002-code-structure-build-toolchain.md) line 859 |
| OQ-0003-2 | The ink's width, its inner lines and its weight on buildings. | Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick | [ADR 0003](0003-rendering.md) line 714 |
| OQ-0003-3 | Which colour output: authored colour as shown, or a tone-mapped output? | Mustafa, in the look proof test, side by side with legacy | [ADR 0003](0003-rendering.md) line 722 |
| OQ-0003-4 | Is one sun shadow map enough, or are cascades or another setup needed? | the look proof test (softness, Mustafa) and the render budget bench (the shadow row) | [ADR 0003](0003-rendering.md) line 728 |
| OQ-0003-5 | How are "warm sun with cool shadows" and moods per map lit? | Mustafa, in the look proof test, with each option's GPU time | [ADR 0003](0003-rendering.md) line 734 |
| OQ-0003-8 | How is ink line boil drawn? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 753 |
| OQ-0003-9 | How are vegetation and the lush countryside drawn, and in which tree style? | Mustafa (the style, a later design session); the render budget bench (the technique) | [ADR 0003](0003-rendering.md) line 758 |
| OQ-0003-10 | Does the frame rate follow a 120 or 144 Hz screen, or is it capped? | Mustafa (feel), on the render budget bench's numbers | [ADR 0003](0003-rendering.md) line 765 |
| OQ-0003-11 | Should v2 one day drop the WebGL 2 fallback, and how many players use it? | Mustafa | [ADR 0003](0003-rendering.md) line 771 |
| OQ-0003-12 | Is the pixel ratio capped at 1 on Low for HiDPI laptops? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 777 |
| OQ-0003-13 | Where does the audio listener sit under the v2 camera? | the camera framing first (R2-INT-OQ2, a design question), then Mustafa's listening check | [ADR 0003](0003-rendering.md) line 782 |
| OQ-0003-15 | The legacy look touches LOOK_V2 does not name: keep, change or drop each? | Mustafa, in the look proof test | [ADR 0003](0003-rendering.md) line 793 |
| OQ-0004-1 | How is bot behaviour structured? | a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set | [ADR 0004](0004-simulation-core.md) line 742 |
| OQ-0004-4 | What moves a building through its damage stages? | Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here) | [ADR 0004](0004-simulation-core.md) line 765 |
| OQ-0004-5 | Which weapons drop, and who may take them? | Mustafa with the game-designer; ADR 0005 holds the answers as data | [ADR 0004](0004-simulation-core.md) line 773 |
| OQ-0004-6 | What do bots and slot takeovers do with player records? | Mustafa with the game-designer | [ADR 0004](0004-simulation-core.md) line 784 |
| OQ-0005-CD1 | Do hand-edited content files need comments? | Mustafa, after a short trial in the content and ability spike | [ADR 0005](0005-content-abilities-assets.md) line 583 |
| OQ-0005-CD2 | What load time and Wasm size may content cost, and is JSON fast and small enough? | the content and ability spike's measurement, then the Advisor sets the budgets | [ADR 0005](0005-content-abilities-assets.md) line 589 |
| OQ-0005-CD3 | May production ever change numbers in the middle of a match? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 598 |
| OQ-0005-CD4 | In development, do simulation numbers apply at the next match, or inside a running match? | Mustafa and the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 606 |
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? | the faction design session (VISION_V2 §7, faction asymmetry; Mustafa) | [ADR 0005](0005-content-abilities-assets.md) line 618 |
| OQ-0005-CD6 | How are moment rules written? | the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004 | [ADR 0005](0005-content-abilities-assets.md) line 624 |
| OQ-0005-CD7 | What is the Feel Lab tool? | a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the greybox v0); ADR 0003 for its screen | [ADR 0005](0005-content-abilities-assets.md) line 632 |
| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1135 |
| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1163 |
| OQ-0005-AB7 | May a pick add raw power (more damage, more health), or only change how a unit or weapon plays? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1171 |
| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1180 |
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1188 |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1197 |
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1206 |
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1217 |
| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1226 |
| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling | [ADR 0005](0005-content-abilities-assets.md) line 1481 |
| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1489 |
| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept) | [ADR 0005](0005-content-abilities-assets.md) line 1624 |
| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review, on the text the authors write in the fix round (engine-architect for AP-2, AP-3, CD-6 and CD-13; the game-d | [ADR 0005](0005-content-abilities-assets.md) line 1833 |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa answers OQ-0005-AB4; the content data section's author aligns CD-6's wording in the fix round, so that it leaves AB4 open. Research | [ADR 0005](0005-content-abilities-assets.md) line 1852 |
| OQ-0005-3 | What does the content hash cover? | the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3) | [ADR 0005](0005-content-abilities-assets.md) line 1864 |
| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor (EDITOR_V2 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1876 |
| OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? | Mustafa (a console check in his own account; agents create no account), then a research read if (b) | [ADR 0006](0006-infrastructure-operations.md) line 598 |
| OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? | a measurement (the netcode proof test's setup or a small check before it), confirmed by Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 608 |
| OQ-0006-3 | Where does the lobby run, and what stores guest ids and the room directory? | the Advisor, from a short lobby read done with the security baseline (its key storage is item B5); Mustafa for any paid service. ADR 0001 ke | [ADR 0006](0006-infrastructure-operations.md) line 615 |
| OQ-0006-4 | Once the access wall is on, does it cover every preview, or only multiplayer previews and previews with unreleased art? | Mustafa (convenience against one more project to run) | [ADR 0006](0006-infrastructure-operations.md) line 626 |
| OQ-0006-6 | The security baseline: what it decides for each waiting item. | security-auditor, in its own fresh session (BASELINE-WHEN (b)); the Advisor and Mustafa for the policy calls | [ADR 0006](0006-infrastructure-operations.md) line 647 |
| OQ-0006-7 | Will any CI runner be paid for: GPU, macOS, self-hosted reference machines, or a cache above 10 GB? | Mustafa for any paid plan; ADR 0002 says which jobs need one; the security baseline weighs (c) | [ADR 0006](0006-infrastructure-operations.md) line 658 |
| OQ-0006-8 | What do Sentry and Grafana really cost, and how do logs and metrics reach Grafana Cloud? | Mustafa, with a read of sentry.io/pricing before he creates the Sentry organization; a read of Grafana's ingestion docs if M2 is wanted | [ADR 0006](0006-infrastructure-operations.md) line 668 |
| OQ-0006-9 | What does EU data-protection law require of us, and how long are match records kept? | Mustafa | [ADR 0006](0006-infrastructure-operations.md) line 677 |
| OQ-0006-10 | How many players, and when; which VAT and currency; is included traffic per VM, pooled or prorated? | Mustafa (his plans, his billing address, the first invoice) | [ADR 0006](0006-infrastructure-operations.md) line 686 |
| OQ-0006-12 | If production isolation is ever adopted: embeds, cross-origin popups, CORP and the login flow. | Mustafa with this record, only if production isolation is adopted (R1-BND-REC2 mark 5's test) | [ADR 0006](0006-infrastructure-operations.md) line 701 |
