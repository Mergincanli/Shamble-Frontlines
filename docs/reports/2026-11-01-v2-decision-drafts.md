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
12 conditions, word for word in appendix §10. Revision 5 (2026-11-03: his design answers, his answers to the
escalations, COMMUNITY-SERVERS, the last fix round) → APPROVE WITH CONDITIONS, 10 conditions, appendix §11. Revision 6
(2026-11-03: his Hetzner check, its corrections and his last answers) → APPROVE WITH CONDITIONS, amended twice as his
messages came in (16 conditions, then MACHINES-TIMING, LG-COUNT and the domain), appendix §12.

## What changed
- `docs/ENGINE_OVERVIEW.md` (147 lines; the diagram shows the parts, the boundary, the per-tick and per-frame flow and
  VISION_V2 §6's places) · `docs/SPIKE_PLAN.md` (10 spikes and the asset-pipeline research: order, sizes, pass marks;
  wave 1 one heavy spike at a time) · `docs/adr/README.md` (index, every decision the drafts ask for, 38 design
  questions, the loose-ends register; its pointers name OQ ids and parts, no line numbers).
- `docs/adr/0001`–`0006` (Proposed; signed off as the basis for the spikes): 116 numbered open questions (96 open after
  the stop's answers and Mustafa's later ones) plus one in SPIKE_PLAN, each with options and a decider, and 341 pass
  marks. Authors: netcode-architect, engine-architect, graphics-engineer, rust-reviewer, engine-architect +
  game-designer + graphics-engineer, infra-engineer; pass marks by test-engineer.
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
- **Scripts at fd8c6f8 (the drafts' last change before the stop):** 521 of 521 research ids disposed exactly once; 0
  retired glossary phrases in all nine documents; README's 94 pointers into the drafts on target; no line glued under a
  table; the write guard passes.
- **The stop's answers re-checked (2026-11-03):** five fresh verifiers judged every changed line against the words and
  the advisor's rulings, and rebuilt each document from the authors' blocks (byte for byte the commit): fixcheck-e1,
  -e2, -e3 and -e3b PASS WITH NOTES, -e2b FAIL (two medium items in ADR 0005: PM-0005-4's kinds against CD-2, and
  Appendix A's count); 205 claims judged, 193 confirmed; skeptics on three verdicts (two upheld, one partly). Every
  finding went to its own author in the last fix round (0aa262f, with Mustafa's later answers), and then verification
  stopped, as he asked. The session's scripts at 0aa262f: the write guard passes; 521 of 521 research ids; 0 retired
  glossary phrases; no line glued under a table, every pass-mark row 7 cells, new prose lines at most 120 characters;
  115 numbered open questions, 97 open, and 341 pass marks, numbered without gaps; 172 quotes credited to Mustafa or the
  Advisor on 2026-11-03, each found word for word in his three messages (two with their inner quotes made single, as
  quotes inside a quote), the other hits section names and labels; the labels as the advisor set them; README's 114
  pointers carry no line number and name ids and parts that exist. Appendix §7.4.
- **The Hetzner round (plan revision 6, 4dc30bc), scripts only (his stop):** 79 exact FIX blocks and 37 NOTEs from six
  authors, each placed by script inside its author's own text; a work list by script (107 places plus the advisor's
  list), each place given a FIX or a reasoned NOTE. After it: the write guard passes; 521 of 521 research ids; 0
  retired glossary phrases; tables clean, 341 pass-mark rows of 7 cells; no new prose line over 120 characters; 116
  numbered open questions, 96 open (ADR 0006: 22 and 17); 179 quotes credited to Mustafa or the Advisor on 2026-11-03
  found word for word in his ten messages (the misses are the NET-PROFILES nesting, section names, labels and two
  phrases quoted from Hetzner's pages); no CCX13 or DS role, no SH in any machine column or key, `nbg1`, `hel1`,
  Nuremberg and Helsinki only in OQ-0006-2's recorded options and his quoted words, every remaining CX33 a labelled
  placeholder, a record, an example or a "cannot be ordered in Falkenstein" note, and no address written as fixed.
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
Mustafa's answers, word for word: appendix §2; where each landed, by commit: appendix §9 (03ef222, 59d41e0, 0aa262f).
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
  CX33s; "Test PC: my own" is MP, never a measured result (plan revision 5 adds his stand-in for ML, for rough checks
  only; nothing stands in for RD or MAC); the WebSocket leg uses a trusted certificate
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

Later the same day (appendix §2, plan revision 5): Mustafa's design answers, recorded in README and STATUS as he
asked (carry slots, levelling, Banzai vs Riot Shield, upgrade visibility, spawn protection in part, fragments visual
only, the name "amplifications", a lean for weapon unlocks; still open: looted ammo, faction asymmetry, the war
around the matches); his answers to the four escalations (the reference machines answered in part, LOOK_V2 §8 kept
with Blender for the test models only, one cost cap per run, shamblefrontlines.com at Cloudflare); COMMUNITY-SERVERS,
a new open question in ADR 0006 (OQ-0006-21), the door kept open; and his stop on verification after this round.
Commit: 0aa262f (every author's blocks; appendix §9). Counts after it: 115 numbered open questions, 97 open; pass marks
341.

That evening (appendix §2, plan revision 6): his Hetzner console check, two corrections and four answers. The rig is
"a CCX23 for the game server and a CAX21 for the 48 simulated players", both in Falkenstein (`fsn1`), created fresh on
each run day; OQ-0006-1 and OQ-0006-2 are answered; the shared CX33 cannot be ordered in Falkenstein, so it is not
measured and a shared fleet (K3) waits; the metrics role runs on the CAX21, never on the game server; a run adds
CAX21s as its calibration needs, inside his cost cap (LG-COUNT); no pass mark's verdict rests on the Arm clients
agreeing with x86-64 or Wasm (ADR 0001); MACHINES-TIMING (c), with GitHub's macOS runners checked first as the netcode
proof test's Safari check; shamblefrontlines.com registered; a new open question, OQ-0006-22 (the staging server's x86
plan, before slice 3). Commit: 4dc30bc (every author's blocks). Counts after it: 116 numbered open questions, 96
open; pass marks 341.

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

## Notes left after verification
Mustafa: "anything left goes into the PR as notes." No verifier re-checked the last round's blocks; the scripts above
did. What the authors flagged and left, each for the drafts' next revision unless it says otherwise:
1. ADR 0003's look proof test needs "both reference machines (OQ-0003-1)" and names no Mac, though its rows PM-0003-1,
   -2, -3, -5, -6 and -16 name MAC for Safari; his answer puts the Mac among the machines he names before that test
   (graphics-engineer).
2. PM-0006-18's "until it is answered the plan's cost goes to Mustafa" now describes the time before his SPIKE-COST
   answer (infra-engineer; the row is test-engineer's, unchanged).
3. SPIKE_PLAN item 6, the preview hosting check, needs ML and MAC with the four browsers (as PM-0006-1 to -6 and -8 do)
   and may run before the machines are named; his MACHINES-TIMING answer settles only the netcode proof test's Safari
   check, so each later spike's plan decides its own Safari rows (engine-architect; the advisor's amendment 2).
4. COMMUNITY-SERVERS' "the game's version" is read as the protocol version that `Join` carries (the advisor's ruling);
   `Join` carries no build version, and adding one would be a new field, for later (netcode-architect).
5. The design answers are in README and STATUS only. ADR 0005, ADR 0001 part 12 (AB15's visibility) and ADR 0003 (the
   fragments) take them up at their next revision, and README's interface I-21 ("sent to their owner") and code J5
   change with them (the advisor's DRAFT-ENTRIES (a); engine-architect).
6. Two older notes stand (appendix §8): GLOSSARY's base text (lines 707–712) still gives the WebSocket leg a pinned
   self-signed certificate; OQ-0006-13 times its call "before the first client-host token".
7. GLOSSARY's base text (lines 227–228, 260 and 605, research, not edited) still names the shared CX33 measured beside
   the reference server, R6-HOST-OQ1 as open and a staging CX33; the drafts now say otherwise (plan revision 6).
8. The branch's checkpoint commits are unsigned ("Unverified" on GitHub): the checkpoint script does not sign, and
   amending one was refused as a destructive git action. The squash merge keeps them out of main.

## Risks
1. Size: about 11,000 lines of drafts. README's "Decisions the drafts ask for" is the way in.
2. The asset pipeline has no research yet (compression, LOD tool, export checks, budgets): open until ASSET-RESEARCH,
   whose export and slot-naming part runs before the look proof test.
3. PR #33's history holds 31 MB of a skeptic's build output (294b3cf): the squash merge keeps it out of main.

## Proposed next step
After the merge: the netcode proof test (SPIKE_PLAN item 1), as its own change-set branched from main, plan first. Its
plan checks GitHub's macOS runners as the Safari check first (MACHINES-TIMING) and puts its cost cap to Mustafa; with
security-auditor it settles the certificate method, how a session holds its SSH key, and how a session checks the
fresh VMs' host keys each run day.

## Decisions needed
None open from this PR: MACHINES-TIMING ((c), with the GitHub macOS runner check first) and LG-COUNT (a run adds
CAX21s as its calibration needs, inside your cost cap) are answered. OQ-0006-22, the staging server's x86 plan, is
yours (money) with the Advisor, but only before slice 3.

## For Mustafa
1. Borrow and name the laptop and the desktop: rough checks run first, then a short re-run once they're named. Whether
   you borrow the Mac before the netcode proof test or only for the look proof test depends on the GitHub runner check.
2. OK each run's cost cap when its plan reaches you.
3. Approve the `.github/` workflow for the Safari check, if the proof test's plan puts it in this repository.
4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat; it merges by squash only. Nothing to check in Brave: nothing a player sees changes.
