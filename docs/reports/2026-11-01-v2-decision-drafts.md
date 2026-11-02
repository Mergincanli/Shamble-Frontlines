# Advisor report: v2-decision-drafts (2026-11-01, branch claude/laughing-lovelace-ox61se, base main 9e7db28)
Draft PR [#33](https://github.com/Mergincanli/Shamble-Frontlines/pull/33). The branch stands in for
`cs/v2-decision-drafts` (this cloud session is limited to it). Appendix: `2026-11-01-v2-decision-drafts-appendix.md`
(the brief in full; Mustafa's words; the advisor's plan review and the plan's rules, word for word; the commits; every
finding and its outcome).

## Goal
The v2 decision drafts the Advisor's brief asks for: an engine overview, six decision drafts (ADR 0001–0006, status
Proposed), a spike plan and the loose ends, all checked against the research. Then a stop for Mustafa and the Advisor.
Nothing changes for players: these documents decide how v2 gets built.

## Outcome
{OUTCOME}

## Plan and approval
Plan revision 1 → the advisor's APPROVE with 19 conditions (2026-10-31), folded into revision 2; revision 3 applies
Mustafa's UPGRADE-SCOPE update. Rulings: ASSET-RESEARCH (a); TICK-60-HEADROOM read as "the 60 Hz margin" (to confirm
now); GLOSSARY §8 yes; the checkpoint kept at the stop. Each condition's place: appendix §6.

## What changed
- `docs/LOOK_V2.md` (byte copy) · `docs/ENGINE_OVERVIEW.md` (one page, Mermaid diagram) · `docs/SPIKE_PLAN.md`
  (10 spikes and the asset-pipeline research: order, sizes, pass marks by id) · `docs/adr/README.md` (index, the
  decisions the drafts ask for, 38 design questions, the loose-ends register, the codes the drafts cite).
- `docs/adr/0001`–`0006` (Proposed). Authors: netcode-architect, engine-architect, graphics-engineer, rust-reviewer,
  engine-architect + game-designer + graphics-engineer, infra-engineer; pass marks: test-engineer ({PM} marks).
- `docs/research/GLOSSARY.md` §8 (appended; lines 1–743 unchanged) · STATUS, ROADMAP, REPO_MAP (docs-keeper) · this
  report and its appendix. Symbols +0 −0 (gate check 3); no code, no spike.

## Evidence
- **Against the research** (plan step 7; one fresh verifier per draft and one for the synthesis): 3,922 claims, 3,847
  confirmed (98.1 %). Verdicts: 0001, 0002, 0003, 0004, 0005 and the synthesis PASS WITH NOTES; 0006 FAIL (3 defects),
  then fixed. Every "wrong" or "unsupported" verdict went to a skeptic first (four rounds): 24 upheld, 9 partly, 2
  refuted. Source checks: about 140 pages read with curl, allow-listed hosts only, by day. github.com answers 403 here,
  so its pages were read from raw.githubusercontent.com. Each verdicts file holds its fetch audit.
- **Against each other** (step 8): the consistency critic, 14 findings (3 high), all fixed by their owners; 489
  in-context glossary hits judged in the drafts and 44 in the synthesis. The completeness critic: GAPS, 16 items, all
  handled (appendix §5).
- **The fixes re-checked** (step 9): {FIXCOUNT} exact fix blocks in {ROUNDS} rounds, each applied only inside its
  author's own text. Verifiers fixcheck-a and fixcheck-b: PASS WITH NOTES (211 changed claims: 201 confirmed). Applying
  the fix files in commit order to the verified text gives the next commit byte for byte. fixcheck-c: {FIXC}.
- **Scripts at {HEAD}:** dispositions 521 of 521 research ids, each exactly once; glossary exact hits 0 in all nine
  documents; README's pointers into the drafts 69 of 69 on target; no line directly under a table; ENGINE_OVERVIEW
  139 lines; the write guard (only allowed paths; research files and LEGACY_INVENTORY unchanged) PASS on every commit.
- **Gate** (`node tools/gate.mjs 9e7db28`): {GATE}. CI's gate with smoke: green on every pushed fix commit.

## Reviews
Ten reviewers on the final diff (plan step 10, xhigh, the look rule in every prompt; each author also judged the
pass marks of its own draft). Verdicts, then the high findings; every finding is in the appendix §7 with its outcome.
- **engine-architect** (boundaries): 0001 REVISE (small), 0003, 0004, 0006 and two pass-mark sets APPROVE WITH NOTES.
  High: no timing rule for events (remote deaths and blasts would play before the delayed poses they belong to).
- **security-auditor:** 0002 and 0006 REVISE, 0001 APPROVE WITH NOTES. High: a pull request runs its own branch's
  workflow, so a branch could take the preview hosting token, which can also publish production.
- **infra-engineer:** 0006 REVISE (its own text, said so), 0001 and 0002 APPROVE WITH NOTES. High: no release path or
  rollback for production; the rooms-per-VM mark contradicts ADR 0001's one-thread-per-vCPU rule.
- **game-designer:** 0001, 0004 and one 0005 pass-mark row REVISE. High: the server would refuse an aim point beyond the
  weapon's range, so almost every Banzai blade swing and Molotov throw (clamp instead).
- **netcode-architect:** 0001's pass marks and its own parts 5, 7, 8 REVISE (small); the rest APPROVE (WITH NOTES).
  High: the rewind estimate leaves out the command buffer and the page's blend (an honest shot at 100 ms needs 233–383
  ms, above the 200 ms cap).
- **perf-auditor:** 0001 and 0003 REVISE (small), the rest APPROVE (WITH NOTES). High: the 160 kbps mean is gated in the
  stress scene, which costs 157–182 kbps on the research's own sizes.
- **graphics-engineer:** 0005 REVISE; 0002, 0003's pass marks, the overview and the look proof test APPROVE WITH NOTES.
  High: "one material per class" against "one material per slot content" (one material per content key).
- **rust-reviewer:** {RR}
- **art-director:** {AD}
- **test-engineer:** {TE}
All of them: {REVIEW_OUTCOME}

## Decisions and deviations
- **Openly stated (condition 18):** LOOK_V2 was added before the plan's approval (brief §0 asked for it first). The
  plan delays R7-MARKS's "changed when their ADR draft comes up" until the drafts are accepted: each draft lists its
  rows instead (id, old mark, proposed mark, reason), the "look brief pending" rows included (18 more added in round 3).
- **Different from the plan, in form:** the write guard checks the GLOSSARY base as a byte prefix (the base file has no
  final newline, so `head -n 742` cannot work); the session put a blank line under eight tables (ADR 0001 five, 0002
  three; GitHub would show the line as a table row): no word changed, 0001's author proposed it, 0002's had no round.
- **Slips and hand-overs:** ADR 0001's F8 (a citation in test-engineer's text) went to netcode-architect by mistake
  (a stale line range replaced by the section's name; kept). Fixes that touched another author's text were held and
  passed on: rust-reviewer's two blocks (test-engineer wrote its own) and one engine-architect block (superseded).
- **For the Advisor:** ADR 0001 replaces four research lines under TICK-30, not three (r2-netcode.md:2302 too).
  Mustafa's UPGRADE-SCOPE update answers brief §3's "how a pick applies after a switch": tracks survive switching.
- **Time:** Mustafa's one-off night run (2026-11-01 03:55–11:00; web fetching kept to 08:30 and later); four
  spend-limit cuts (2026-10-31 19:18; 2026-11-01 05:30, 09:27, 14:20), each resumed from the agents' notes.

## Risks
1. Pointers into the drafts by line number drift with every fix: README's are re-pinned and checked by
   `scripts/draft_pointers.py`; the pass marks now point at parts and rows instead. {RISK1}
2. Claims on sources nobody could read (github.com answers 403 in this session) stay marked "unverified"; the
   decisions on them carry spike marks. {RISK2}
3. The asset pipeline has no research yet: compression, LOD tool, export checks and per-unit budgets are open
   questions marked "not researched" until the asset-pipeline research runs (ASSET-RESEARCH).
4. Size: about 10,000 lines of drafts. README's "Decisions the drafts ask for" is the way in.

## Proposed next step
After Mustafa's and the Advisor's review: the netcode proof test (SPIKE_PLAN item 1), as its own change-set, plan first.

## Decisions needed
- **SPIKE-ORDER** · Accept the drafts as the basis for the spikes, and SPIKE_PLAN's order? · Phase 0 allows spikes
  only after you both approve (CLAUDE.md) · (a) accept; (b) accept with changes; (c) send back · Recommend (a){DEC1}.
- **TICK-60-HEADROOM** (ADR 0001, OQ-0001-1) · Confirm: a 60 Hz tick needs the p99 server tick time at most 6.25 ms
  against 8.33 ms (the 60 Hz margin, 25 %), measured with 48 players on one vCPU of the reference server over 30
  minutes? · Decides whether v2 ever ticks at 60 Hz · confirm, or another base for the 25 % · Recommend confirm.
- **TICK-60-SETUP** (OQ-0001-2, the Advisor) · Where does that 60 Hz run get its simulation load before the real
  simulation exists? · (a) the sim core spike's simulation inside the proof-test server; (b) a stand-in replaying
  measured step times; (c) wait for the real simulation · Recommend (a), as ADR 0001 does.
- **LATENCY-100** (OQ-0001-3, the Advisor) · Is the netcode proof test's "100 ms of simulated latency" research
  profile P2 (100 ms round trip with P2's jitter and loss) or 100 ms each way? · It sets the test's hardest case ·
  Recommend P2 (VISION_V2 §4's "about 100 ms of ping").
- **ASSET-RESEARCH** · Run the asset-pipeline research (one daytime research session, no code) earlier, beside the
  first spikes, instead of just before the asset spike? · Art production waits on it · Recommend earlier.
- **TOY-BOX** · CLAUDE.md line 4 ("Red Alert 3 toy-box look") and the art-director's file line 10 ("a joyful,
  readable, toy-box battlefield") may clash with LOOK_V2 §3 ("It must not look like a children's game") · Agents read
  them as the look · keep; reword to point at LOOK_V2; remove · Recommend reword, in a small kit change-set.
- **OQ-0002-8** (the Advisor) · ADR 0002 proposes a `room` and a `navbake` crate that no research option offers,
  because the local server in the browser (SLICE-ORDER) forces it. Allowed, labelled "not researched"? · (a) yes,
  the foundation's first milestone tests it; (b) a short layout research first · Recommend (a).
- **OQ-0005-1** (the Advisor) · Do model ids and faction colours sit on the simulation's definitions, or in client
  definitions keyed by the same id? · It decides what the simulation's Wasm carries · Recommend the client
  definitions (ADR 0005's lean).
- **HARDWARE** (OQ-0006-1, OQ-0003-1) · Can you order Hetzner's CCX23 and CX33 in an EU location, and which laptop
  (minimum) and desktop (recommended) are the reference PCs? · The proof tests run on them · Recommend the Hetzner
  console check first; agents make no account.
- **DESIGN** · Your design questions (README: UPGRADE-SCOPE's carry slots, looted ammo and levelling; how unlocked
  weapons reach a match; Banzai blade against Riot Shield; 38 research questions) · None blocks the first spikes ·
  Recommend one design session after the netcode proof test.

## For Mustafa
1. Read `docs/adr/README.md`, "Decisions the drafts ask for", and answer the decisions above in this chat.
2. In your Hetzner console: can you order a CCX23 and a CX33 in an EU location (OQ-0006-1)?
3. Nothing to check in Brave (nothing a player sees changes) and nothing to merge: the PR stays a draft for your and
   the Advisor's review.
