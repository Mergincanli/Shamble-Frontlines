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
DONE, and past the stop. Every deliverable is in; each was verified against the research, checked for consistency and
completeness, and reviewed by ten domain reviewers, and every finding was fixed or deferred with a reason. On
2026-11-03 Mustafa and the Advisor answered the stop: the six drafts are signed off as the basis for the spikes, not
yet accepted (Status stays Proposed; a draft becomes Accepted only when Mustafa signs it off after its spikes report).
Their answers are recorded in the drafts. The PR is ready for review; it merges by squash only, on Mustafa's go after
the advisor approves its final head.

## Plan and approval
Plan revision 1 → the advisor's APPROVE with 19 conditions (2026-10-31), folded into revision 2; revision 3 applies
Mustafa's UPGRADE-SCOPE update. Each condition's place: appendix §6. Rulings: ASSET-RESEARCH (a); TICK-60-HEADROOM read
as "the 60 Hz margin" (confirmed at the stop, with Mustafa's addition); GLOSSARY §8 yes; the checkpoint kept at the
stop. Revision 4 (2026-11-03: record the stop's answers, close the change-set) → the advisor's APPROVE WITH CONDITIONS,
12 conditions, word for word in appendix §10.

## What changed
- `docs/ENGINE_OVERVIEW.md` (147 lines; the diagram shows the parts, the boundary, the per-tick and per-frame flow and
  VISION_V2 §6's places) · `docs/SPIKE_PLAN.md` (10 spikes and the asset-pipeline research: order, sizes, pass marks;
  wave 1 one heavy spike at a time) · `docs/adr/README.md` (index, every decision the drafts ask for, 38 design
  questions, the loose-ends register; its pointers name OQ ids and parts, no line numbers).
- `docs/adr/0001`–`0006` (Proposed; signed off as the basis for the spikes): 114 numbered open questions (98 open after
  the stop's answers) plus one in SPIKE_PLAN, each with options and a decider, and 341 pass marks. Authors:
  netcode-architect, engine-architect, graphics-engineer, rust-reviewer, engine-architect + game-designer +
  graphics-engineer, infra-engineer; pass marks by test-engineer.
- `docs/LOOK_V2.md` (byte copy) · `docs/research/GLOSSARY.md` §8 (appended; its 60 Hz entry amended at the stop) ·
  STATUS, ROADMAP, REPO_MAP · this report and its appendix. Symbols +0 −0 (gate check 3); no code, no spike.

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

## At the stop (2026-11-03)
Mustafa's answers, word for word: appendix §2; where each landed, by commit: appendix §9 (03ef222, 59d41e0{STOPFIX}).
Mustafa's: SPIKE-ORDER (b), wave 1 one heavy spike at a time; TICK-60-HEADROOM confirmed, with the goal budget holding
at the rooms per VM measured at 60 Hz (new mark PM-0001-63); REWIND-CAP's rule (no value yet); SPIKE-MACHINES;
LOOK-TEST-INPUTS (Blender); TOY-BOX (a kit change-set after PR #30); LATER (a design session after the netcode proof
test). The Advisor's: TICK-60-SETUP (a), an early signal only, slice 6 deciding; NET-PROFILES (P2); OQ-0002-8 (a);
OQ-0005-1; OQ-0001-14; ROOM-CRASH; OQ-0003-20; OQ-0005-AB12; AB13; and OQ-0003-17, confirmed by Mustafa.

The advisor's readings, marked "(the advisor's reading, 2026-11-03)" where they stand; each can be overturned:
- 60 Hz: "the goal budget" is ADR 0006 part 8's figure at the call (OQ-0006-19's at slice 6; in wave 1 reported only);
  "holds" means the goal-mode fleet at the rooms per VM measured at 60 Hz costs no more; slice 6 always runs the final
  measurement, and a pass in wave 1 starts no 60 Hz work.
- Spike order: the 60 Hz run uses the proof test's kept server and harness after the sim core spike; the proof test
  closes at 30 Hz without it; the boundary spike uses the sim core spike's sim; the asset research's export part runs
  between two wave-1 spikes (recommended: right after the proof test).
- The rewind cap: the honest rewind is its p99 at P2, rounded up to a whole tick; the checklist's 200/250 ms comparisons
  and the Feel Lab's weighing of the cap go; "the smallest input buffer that holds" guides OQ-0001-6.
- P2's loss: 2 % on each direction, independently.
- Machines: "no token until the security baseline" covers the tick bench and the 3-hour soaks; the load generator is
  CX33s; "Test PC: my own" is MP and stands in for no reference machine; the WebSocket leg uses a trusted certificate
  on his domain, the method in the proof test's plan.
- The look test: blockouts may stand for part 11's objects; no checklist edit now; Blender for the models and the
  export route.
- OQ-0003-17 and -20: the narrow reading (the frame rows keep gating; PM-0003-44 gated on RD at High); the soaked set
  gates on ML.
- OQ-0005-1: the presented kinds that need a client entry are unit, vehicle, weapon and faction; AB13: a new reviewed
  operation still counts as built from data.
- ROOM-CRASH: "keep" means under ADR 0006 part 11's proposed retention; one word, "marked crashed".
- LATER: the twelve questions and the 38 design questions go to that session; OQ-0003-16, OQ-0006-18 and OQ-0006-13
  keep their own timing.
- Counts: a partly answered question stays open (OQ-0001-5 among them), so 98 are open.

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
  let this morning's last units start before 08:30 (fetching still from 08:30); ten spend-limit cuts over four days,
  each resumed from the agents' saved work; from 2026-11-02 his save-as-you-go rules (appendix §2).

## Risks
1. Size: about 11,000 lines of drafts. README's "Decisions the drafts ask for" is the way in.
2. The asset pipeline has no research yet (compression, LOD tool, export checks, budgets): open until ASSET-RESEARCH,
   whose export and slot-naming part runs before the look proof test.
3. PR #33's history holds 31 MB of a skeptic's build output (294b3cf): the squash merge keeps it out of main.

## Proposed next step
After the merge: the netcode proof test (SPIKE_PLAN item 1), as its own change-set branched from main, plan first; it
waits on the items in "For Mustafa" and, for its own plan with security-auditor, on the certificate method and how a
session holds its SSH key.

## Decisions needed
Escalated by the advisor at the stop (appendix §10); none blocks this PR.
- **REF-MACHINES** (OQ-0003-1) · Which laptop is the minimum (ML), which desktop the recommended (RD), and which Mac
  (MAC)? · About 22 of the proof test's marks are measured on ML or RD, and your PC (MP) is never a measured result ·
  (a) name, buy or borrow ML, RD and MAC before the proof test's plan; (b) your PC as RD for the proof test only, which
  changes MP's rule; (c) start the rows that need neither · Recommend (a) for ML and RD; MAC may follow.
- **LOOK_V2-§8** · You chose Blender for the look test's models, while LOOK_V2 §8 says Spline: does §8 change? · Agents
  read §8 as the production art flow · (a) keep §8, Blender for the test models only; (b) change §8, in a change-set by
  you and the Advisor · Recommend (a), until the asset research's export part reports.
- **SPIKE-COST** (OQ-0006-20) · What may the proof test's VMs cost? · With no cap, each run plan's cost comes to you
  before you create its VMs · (a) 5 EUR for ADR 0006's own runs; (b) one cap from the run plan's hours; (c) 5 EUR for
  everything · Recommend (b).
- **DOMAIN** (OQ-0006-15) · Which domain, registrar and DNS host for the WebSocket leg, and will production use it? ·
  The WebSocket leg needs a trusted certificate on a real hostname before the proof test · a spike-only domain, or the
  game's future one · Recommend the game's future domain if you know it; if not, a cheap spike-only one.

## For Mustafa
1. In your Hetzner console: can a CCX23 and a CX33 be ordered in an EU location (OQ-0006-1)? Still open.
2. Register the domain for the WebSocket leg before the netcode proof test (DOMAIN above).
3. Name the reference machines (REF-MACHINES above).
4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat; it merges by squash only. Nothing to check in Brave: nothing a player sees changes.

