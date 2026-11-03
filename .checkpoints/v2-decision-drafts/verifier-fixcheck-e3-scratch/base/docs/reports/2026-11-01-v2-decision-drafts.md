# Advisor report: v2-decision-drafts (2026-11-03, branch claude/laughing-lovelace-ox61se, base main 9e7db28)
Draft PR [#33](https://github.com/Mergincanli/Shamble-Frontlines/pull/33). The branch stands in for
`cs/v2-decision-drafts` (this cloud session is limited to it). The details are in the appendix,
`2026-11-01-v2-decision-drafts-appendix.md`: the brief, Mustafa's words, the advisor's plan review and the plan's rules
(word for word), the commits, the 19 conditions, and every verification, review, finding and outcome.

## Goal
The v2 decision drafts of the Advisor's brief: an engine overview, six decision drafts (ADR 0001–0006, Proposed), a
spike plan and the loose ends, all checked against the research, then a stop for Mustafa and the Advisor. Nothing
changes for players: these documents decide how v2 gets built.

## Outcome
DONE, at the stop. Every deliverable is in. Each was verified against the research, checked for consistency and
completeness, and reviewed by ten domain reviewers, and every finding was fixed or deferred with a reason. The PR stays
a draft.

## Plan and approval
Plan revision 1 → the advisor's APPROVE with 19 conditions (2026-10-31), folded into revision 2; revision 3 applies
Mustafa's UPGRADE-SCOPE update. Each condition's place: appendix §6. Rulings: ASSET-RESEARCH (a); TICK-60-HEADROOM read
as "the 60 Hz margin" (to confirm now); GLOSSARY §8 yes; the checkpoint kept at the stop.

## What changed
- `docs/ENGINE_OVERVIEW.md` (144 lines; the diagram shows the parts, the boundary, the per-tick and per-frame flow and
  VISION_V2 §6's places) · `docs/SPIKE_PLAN.md` (10 spikes and the asset-pipeline research: order, sizes, pass marks)
  · `docs/adr/README.md` (index, every decision the drafts ask for, 38 design questions, the loose-ends register).
- `docs/adr/0001`–`0006` (Proposed): 114 numbered open questions (111 open) plus one in SPIKE_PLAN, each with options
  and a decider, and 340 pass marks. Authors: netcode-architect, engine-architect, graphics-engineer, rust-reviewer,
  engine-architect + game-designer + graphics-engineer, infra-engineer; pass marks by test-engineer.
- `docs/LOOK_V2.md` (byte copy) · `docs/research/GLOSSARY.md` §8 (appended) · STATUS, ROADMAP, REPO_MAP · this report
  and its appendix. Symbols +0 −0 (gate check 3); no code, no spike.

## Evidence
- **Against the research:** 3,922 claims, 3,847 confirmed (98.1 %); 0006's first verdict FAIL, fixed. A skeptic
  re-checked first every "wrong" or "unsupported" verdict that the drafts' verifiers and fixcheck-a to -c sent to an
  owner (five rounds: 27 upheld, 9 partly, 2 refuted); on 2026-11-03 six later verdicts got skeptics too (fixcheck-d3's
  D3A-1 and D3E-5, fixcheck-d4's F1, F2, F6 and F8: 2 upheld, 4 partly; the appendix §7.1 lists the ones that did not).
  About 160 source pages read, allow-listed hosts only, by day, each with a fetch audit.
- **Against each other:** the consistency critic's 14 findings, all fixed; the completeness critic's 16 gaps, all
  closed (the 18 legacy inventory rows, the pass marks' anchors, the appendix's §3–§4, the re-checks, the reviews).
- **The fixes re-checked:** 901 exact fix blocks in six rounds and 63 in the touch-ups after them, each applied only
  inside its author's own text (a block in another author's text went to that author). Re-checks: fixcheck-a, -b, -c and
  -d1 PASS WITH NOTES (d1: rebuilding ADR 0001, 0002 and 0006 from the blocks gives the commit exactly; its three medium
  items fixed); d0, the final round's web facts: 9 of 10 confirmed, the tenth fixed; d2 FAIL, as fix blocks had dropped
  ADR 0005's AB-3 heading and a Feel Lab step (both restored; a fresh verifier dry-ran the AB-3 block before it was
  applied, and fixcheck-d3b judged both after); d3, on every touch-up: PASS WITH NOTES in units a, b, c and e (unit d
  ran at 08:07, before the fetch window, and judged none of its three facts; e read them; 0 of 3,225 structural lines
  lost across the nine documents; the two web facts the final round could not read and a third d3a raised, confirmed at
  their sources; each finding fixed by its author or deferred with a reason, appendix §7.3); a last check of those fixes
  and of this report's numbers: PASS WITH NOTES (fixcheck-d4: 68 claims, 59 confirmed; the ten newest blocks right; nine
  slips in this report and its appendix, one medium, fixed with its skeptics' corrected words and not re-checked again;
  one optional section number deferred).
- **Scripts at fd8c6f8 (the drafts' last change):** 521 of 521 research ids disposed exactly once; 0 retired glossary
  phrases in all nine documents; README's 94 pointers into the drafts on target; no line glued under a table; the write
  guard passes.
- **Gate** (`node tools/gate.mjs 9e7db28`): 1–4 and 5b–5d PASS, symbols +0 −0; 5a smoke cannot start here (Playwright
  wants a browser revision this container lacks, and installing one is not allowed); CI's gate with smoke: 77 runs on
  this branch before the final commit, 70 passed, 7 cancelled when a newer push replaced them, 0 failed (`[skip ci]`
  commits skip it; the final commit runs it, and its result is in the PR). Repo-map check: OK.

## Reviews
Ten reviewers on the final diff, the look rule in every prompt; each author also judged its own draft's pass marks.
REVISE verdicts: 0001 (four reviewers), 0002 (two), 0003 (two), 0004, 0005, 0006 (two), SPIKE_PLAN; everything else
APPROVE (WITH NOTES). Every high finding is fixed. Among them: remote events now play on their entity's timeline, the
aim point is clamped instead of refused, the rewind estimate counts the input buffer, the preview token can no longer
reach production, a look floor stops any quality tier switching a LOOK_V2 [DECIDED] item off, and the 60 Hz run burns
the time of the sim parts it lacks. The final round added 24 open questions; a few low items are deferred to the
revision after this stop, each with its reason (appendix §7.3).

## Decisions and deviations
- **Openly stated (condition 18):** LOOK_V2 was added before the plan's approval (brief §0 asked for it first). The
  plan delays R7-MARKS's "changed when their ADR draft comes up" until the drafts are accepted; each draft lists its
  rows instead, the "look brief pending" rows included.
- **In form:** the write guard checks the GLOSSARY base as a byte prefix (the base file has no final newline); the
  session put a blank line under eight tables (formatting only; GitHub would show the line as a table row). In the final
  round only two claims ledgers (plan rule 9) got its new claims, ADR 0004's and ADR 0002's (last change 8fa7173, early
  in that round); the others were not updated in it: the verifiers' claim tables (fixcheck-d1, -d2, -d3, -d4) stand in
  for them, and ledger row 307's 7.49 ms is superseded by ADR 0001's 4.69 ms.
- **Slips and hand-overs:** one citation in test-engineer's text went to netcode-architect by mistake (kept, a stale
  line range replaced by the section's name). Blocks touching another author's text were held and handed on. At the end,
  for time, the owners checked the wrong and unsupported verdicts of fixcheck-d0, -d1 and -d2 against their sources
  instead of a fresh skeptic (1, 6 and 7 rows; d2's other two, F7 and F14, wording, were deferred, appendix §7.3);
  fixcheck-d3 then judged every fixed text and both deferrals against their sources, and four of those verdicts on their
  own (d0 row 9; d1 F1, F2, F10): every verdict holds, and every fix is right but d0 row 9's wording, which leaned on a
  precision promise the toolchain breaks (D3A-1, fixed after a skeptic). One skeptic's scratch put 31 MB of build output
  into a checkpoint commit (gone from the tree; history is never rewritten).
- **For the Advisor:** ADR 0001 replaces four research lines under TICK-30, not three (r2-netcode.md:2302 too).
  Mustafa's UPGRADE-SCOPE update answers brief §3's "how a pick applies after a switch": tracks survive switching. ADR
  0004 part 7: std documents `div_euclid` as exactly rounded, but rustc 1.97.0 and the pinned 1.99.1 return 3 where the
  exact quotient is 2 (a shared error, so a same-bits test passes it); both stay off the float list until PM-0004-22's
  new correctly-rounded-reference clause passes.
- **Time:** Mustafa's one-off night run (2026-11-01 03:55–11:00), and his "You can continue" (2026-11-03 07:14), which
  let this morning's last units start before 08:30 (fetching still from 08:30); nine spend-limit cuts over three days,
  each resumed from the agents' saved work; from 2026-11-02 his save-as-you-go rules (appendix §2).

## Risks
1. Size: about 11,000 lines of drafts. README's "Decisions the drafts ask for" is the way in.
2. Line pointers into the drafts move with every edit: re-pin them with `scripts/draft_pointers.py` after any change.
3. The asset pipeline has no research yet (compression, LOD tool, export checks, budgets): open until ASSET-RESEARCH.

## Proposed next step
After Mustafa's and the Advisor's review: the netcode proof test (SPIKE_PLAN item 1), as its own change-set, plan first.

## Decisions needed
These start the first spike; README's "Decisions the drafts ask for" holds every other one, each with its options.
- **SPIKE-ORDER** · Accept the six drafts as the basis for the spikes, and SPIKE_PLAN's order? · Phase 0 allows spikes
  only after you both approve (CLAUDE.md) · (a) accept; (b) accept with changes; (c) send back · Recommend (a).
- **TICK-60-HEADROOM** (OQ-0001-1) · Confirm: 60 Hz only if the p99 server tick time stays at or below 6.25 ms against
  8.33 ms (the 60 Hz margin, 25 %), with 48 players, one vCPU of the reference server, 30 minutes? · Decides whether v2
  ever ticks at 60 Hz · confirm, or another base for the 25 % · Recommend confirm.
- **TICK-60-SETUP** (OQ-0001-2, the Advisor) · Where does the 60 Hz run get its sim load before the real sim exists? ·
  (a) the sim core spike's sim plus stand-ins that burn the time of the parts it lacks; (b) replayed step times; (c)
  wait for the real sim · Recommend (a): it keeps the one-run rule and errs towards 30 Hz.
- **NET-PROFILES** (OQ-0001-3 and -16, the Advisor) · Is "100 ms of simulated latency" research profile P2 (100 ms round
  trip with P2's jitter and loss), and does each profile split latency and jitter half each way? · They set the proof
  test's hardest case · Recommend P2, split half each way (proposed values).
- **REWIND-CAP** (OQ-0001-5) · With the input buffer counted, an honest shot at P2 needs 233–383 ms of rewind, above
  the 200 ms cap · Fixes how far the server rewinds · Recommend no value now: set it from the proof test's p99.
- **SPIKE-MACHINES** (OQ-0006-1, -17, -20; OQ-0003-1; OQ-0006-15) · The proof test needs Hetzner VMs. Who creates
  them, under what bill cap, and with which reference PCs and certificate? · Nothing runs without them · Recommend a
  separate Hetzner project with its own token, a cap you set, and a private test certificate authority until a domain.
- **LOOK-TEST-INPUTS** (ASSET-RESEARCH timing; OQ-SPIKE-1) · Run the asset research's export and slot-naming slice
  before the look proof test, and name who makes its models? · Art waits on it · Recommend yes, before the look test.
- **TOY-BOX** · CLAUDE.md line 4 ("Red Alert 3 toy-box look") and the art-director's file line 10 may clash with LOOK_V2
  §3 ("It must not look like a children's game") · Agents read them as the look · keep, reword, remove · Recommend
  reword to point at LOOK_V2, in a small kit change-set.
- **OQ-0002-8** (the Advisor) · May ADR 0002 propose the `room`, `navbake` and `client-endpoint` crates, which no
  research option offers but a settled input forces (labelled "not researched")? · (a) yes, the foundation tests them;
  (b) a layout research first · Recommend (a).
- **OQ-0005-1** (the Advisor) · Do model ids and faction colours sit in the sim's definitions, or in client definitions
  keyed by the same id? · It decides what the sim's Wasm carries · Recommend client definitions.
- **LATER** · Game rules (steals, carried caches, rejoins, full pools, bots and pickups, spawn protection, who sees
  picks; UPGRADE-SCOPE's three), the look's tier floor, the deploy-job guard, releases, unreleased art · None blocks the
  proof test · Recommend a design session after it; README gives each its options and a recommendation.

## For Mustafa
1. Read `docs/adr/README.md`, "Decisions the drafts ask for", and answer the decisions above in this chat.
2. In your Hetzner console: can you order a CCX23 and a CX33 in an EU location (SPIKE-MACHINES)?
3. Nothing to check in Brave (nothing a player sees changes) and nothing to merge: the PR stays a draft for your and
   the Advisor's review.
