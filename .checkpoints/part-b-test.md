# Checkpoint: part-b-test
- **Goal:** a docs-only test PR for TEST-RUN part B2 (the PR Advisor routine's first real review, Mustafa
  2026-11-04), which also fixes the two accuracy nits the advisor found in #30's report · plan: below · verdict: pending
- **Branch:** `claude/sleepy-tesla-ofsxsr` (this cloud session's branch, standing in for `cs/part-b-test`; CLAUDE.md
  standing permission), from main e2b5799. Worktree `<scratchpad>/part-b-test`, local branch `part-b-test`, pushed with
  `git push origin HEAD:claude/sleepy-tesla-ofsxsr`; the session's main folder is never switched.
- **Last update:** 2026-11-04 12:00 Amsterdam

## Plan (revision 1)
- **Why:** part B2 needs one real, ready, docs-only pull request for the PR Advisor routine's first review. The test PR
  must be the only ready PR when B2 runs, so the docs pass stays a draft until then. It should be worth merging, so it
  fixes two accuracy nits from the advisor's 2026-11-04 review of #30 (comment 6472566955, findings 1 and 2).
- **The change:** `docs/reports/2026-10-27-kit-v3-1.md` only, two places in its "Update, 2026-11-04" section:
  1. Line 168, "Check 5a comes from CI's `smoke` job, green on every code commit of A1." This becomes: green on every A1
     code commit except 04da817. There the smoke step itself passed (09:30:25–41 UTC), but the next push (1c49be7)
     cancelled the job during its cleanup, so CI shows it as cancelled; gate passed. Smoke is green on the head 7a23132.
     The source is CI run 41807980703, jobs 126499726624 and 126499726876.
  2. Lines 257–258, the steps list. Item 7 becomes "04da817: this section, the docs wording and the `guard.mjs`
     comment". Item 8 becomes "1c49be7: the checkpoint deleted". Item 9 is new: "7a23132, the PR's last commit: the
     report's accuracy fix". Then a line: merged into main as e2b5799 on 2026-11-04, on Mustafa's "merge #30" after the
     advisor's APPROVE of 7a23132.
- **Not in it:**
  - STATUS and ROADMAP: the docs pass right after part B records this PR together with #33's and #30's merges and
    part B's results.
  - No rules file, no `.claude/`, no `.github/`, no gate file. Symbols +0 −0.
- **Evidence:** the gate (`--no-smoke`, GATE-SETUP (a)); the repo-map check; the diff is the two places above; CI's
  `gate` and `smoke` on the head.
- **Review and merge:** this PR is part B2's test subject, so the PR Advisor routine's "Run now" review is its review. It
  is in the ready class (docs only), so the routine merges it when it approves and the checks are green. That merge is
  what B2 tests. No day-session advisor reviews the PR first, so the routine's review is the first and independent.
- **Checkpoint:** this file is deleted in the last commit before the PR is marked ready (it runs CI).

## Steps done
1. Plan revision 1 (3d2e15c); the advisor: APPROVE with 7 conditions (`part-b-test/advisor-plan-verdict.md`).
2. The report corrections, conditions 1–3 (the next commit after this one's parent); gate `--no-smoke` PASS (1–4, 5b–5d, symbols +0 −0); repo map OK.

## Current step
Advisor plan check (revision 1).

## Next steps
1. APPROVE → edit the two places → gate → commit → push → delete this checkpoint (last commit, runs CI) → PR ready →
   tell Mustafa the PR number for B1/B2.

## Queue after this (Mustafa, 2026-11-03 and 2026-11-04; one change-set at a time, plan first)
1. **Part B:** Mustafa runs B1 (the Guard test routine in the pr-advisor environment, the 14-step prompt) and B2 ("Run
   now" on the PR Advisor routine). The session reads both transcripts and checks every row.
2. **The docs pass** (rules, so it needs Mustafa's go):
   - CLAUDE.md's routine shell list names `date`, matching the guard (Mustafa).
   - The rule: the PR Advisor runs only on a schedule or API trigger, never on PR events (Mustafa, PR-HOME-REPLACE (a)).
   - STATUS and ROADMAP:
     - #33 merged (e4d8b22) and #30 merged (e2b5799), with the PR Advisor's nit "a draft while A1 runs";
     - the two routine environments exist, part A passed, part B's result;
     - PR-HOME replaced by the schedule trigger, with no setup script;
     - the PR Advisor routine's new prompt;
     - the advisor's optional KT14 nit in LEGACY_INVENTORY.
3. **B3, docs only, after part B, plan first:** record Mustafa's 2026-11-04 design decisions in LOOP_V2 (rules) and
   UNITS.md (units and factions), marked decided or idea exactly as in his message (word for word below).
4. **A2 (kit-v3-2), B1 (EDITOR_V2 "Plan view: draw to build"), B2 (VISION_V2 "Community servers (later)").** Their
   wording is in Mustafa's 2026-11-03 message, in the frozen session's transcript (session_0173vVo5sT21uP4DcQtKvmAU).

## Decisions taken
- **PR-HOME-REPLACE (a), no setup script** (Mustafa, 2026-11-04).
  - The PR Advisor routine has a schedule trigger only, `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`. The GitHub PR
    trigger is removed. It stays paused until TEST-RUN part B passes.
  - Why: per the official docs (verified at their source 2026-11-04), a setup script runs only when the environment's
    cache is built, and a PR-event run starts from the PR's head commit. A schedule or API run starts from the default
    branch.
- **TEST-RUN part A passed** (Mustafa, 2026-11-04; the Guard test routine on routine-night-shift).
  - git status and the Read tool ran.
  - echo, cat, curl and `git push --dry-run origin HEAD:main` were refused, each with the right message.
  - Both routine environments exist (SHAMBLE_ROLE=night-shift and =pr-advisor), and both routines point at them.
- **PR #30 merged** as e2b5799 (2026-11-04 10:21:56 UTC) on Mustafa's "merge #30". Main's tree equals 7a23132; CI is
  green on main.

## B3: Mustafa's 2026-11-04 message, word for word
B3, a docs-only change-set after part B, plan first: record Mustafa's design decisions from his 2026-11-04 walk in LOOP_V2 (rules) and UNITS.md (units and factions), marked decided or idea exactly as below.
Decided:
- Looting keeps the class ability (a Conscript with a looted shotgun still throws Molotovs). Rare strong combos are welcome; watch for any combo that becomes a must-have.
- Spawn protection: 3 s; firing ends it early.
- Ammo: limited but generous, refilled at the barracks and at supply points (keeps an engineer or supply role possible later).
- Rejoining keeps a player's amp tracks; bots can loot weapons and pick amps.
- Amps (amplifications) are on weapons only. No class perks and no learn-by-doing layer. Account progression gives breadth only (class variants, cosmetics), never power.
- Active reload on every gun, gentle: a perfect reload is faster, with little or no damage bonus; a miss is a normal reload, no jam. Amps can widen the sweet spot.
- Every gun gets a short sprint-to-fire delay, longer for heavy guns.
- Picking an amp: when a gun levels up, three cards wait at the screen's edge; pick with Alt+1/2/3. Rebindable, and tested early in Brave and Firefox, because Alt can steal browser focus or switch tabs; fallback: hold B + 1/2/3.
- The Molotov in v2 is a real fire patch that denies an area, not a grenade.
- Tier-one weapon ladders (first sketch, to iterate):
  - Conscript AMK (bolt action): L1 faster reload; L2 incendiary rounds (a small fire on hit). Suppression parked; if ever, a small screen-edge effect that doesn't stack.
  - Peacekeeper shotgun (shells load one by one): L1 faster reload (two shells at a time); L2 a choice of buckshot or slugs (slugs hit hard and fire slowly, and still lose a long-range duel to the rifle); L3 fork: buckshot leads to a sawed-off (lighter, faster), slugs to a longer barrel (steadier aim).
  - Imperial Warrior SMG (high recoil, strong close, falls off at mid range): L1 faster sprint-to-fire; L2 hollow points (harder close, even faster falloff); L3 drum mag. Dual-wield parked for a tier-two SMG or a variant. Its amps never add range.
- Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two specialists and vehicles are unique). Personalities: Soviets heavy and brutal, Allies precise and technical, Empire fast and tricky.
- Infiltrators (tier two, every faction): one shared core, where entering an enemy building does the same for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus a faction verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB agent interrogates a downed enemy to briefly reveal the enemy team on the map.
Ideas (sketches to iterate):
- Soviets: a crewed walker; the pilot's leap stuns infantry and the gunners finish them (every seat has a verb).
- Allies: a tier-two Cryo specialist: the freeze builds up visibly, then it leaps and shatters the target; it's exposed while it channels. An IFV whose turret weapon depends on the passenger's class (Red Alert 3's Multigunner as the reference).
- Empire: a hover skimmer as its first vehicle (fast, crosses water, sideways dash, rides just above the ground; no true flight under the top-down camera), and a disguise transport at tier two.
- Weapon weight could set movement speed.
Open: the war around the matches, in its own design session.

## Running work
- none
