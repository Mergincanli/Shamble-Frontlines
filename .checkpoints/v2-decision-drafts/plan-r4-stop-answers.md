# Plan revision 4: record the stop's answers, close the change-set (v2-decision-drafts)

2026-11-03, about 12:34 Amsterdam. Branch claude/laughing-lovelace-ox61se (stands in for cs/v2-decision-drafts), draft
PR #33, head 4374a1c (CI green: gate and smoke), base main 9e7db28 (main has not moved).

## Goal
Record Mustafa's and the Advisor's answers to the stop (2026-11-03, word for word in
`inputs/STOP-ANSWERS-mustafa-2026-11-03.md`) in the drafts and the docs; close the change-set by CHECKPOINTS.md §9;
mark PR #33 ready; have the advisor review the final head; squash-merge only on Mustafa's go for that head (his words:
"once it's approved, squash-merge it so the 31 MB of scratch build output never reaches main's history").

## Change type, scope
Docs only: no code, no spike, symbols +0 −0. Touched: `docs/adr/0001`–`0006` (the answered questions, the parts they
settle, the header lines), `docs/adr/README.md` (the decisions table, the open count, the pointers), `docs/SPIKE_PLAN.md`,
`docs/ENGINE_OVERVIEW.md` only where a line states what an answer changes, STATUS, ROADMAP, REPO_MAP (docs-keeper), the
report and its appendix (the session), `.checkpoints/` (deleted in the last commit). Not touched: `docs/LOOK_V2.md`,
`docs/VISION_V2.md`, LOOP_V2, the research files and GLOSSARY.md (a verifier who finds a §8 entry the answers contradict
writes a NOTE for the Advisor; nothing edits it), CLAUDE.md, `.claude/`, the gate's files.

## Where each answer lands (owner)
A mapper (step 2) checks this map against all nine documents and adds every place it misses.
| Answer | Lands in | Author |
|---|---|---|
| SPIKE-ORDER (b) | SPIKE_PLAN's order: wave 1 one heavy spike at a time (netcode proof test, sim core spike, then the 60 Hz run, boundary spike, look proof test once its models exist), the preview hosting check alongside any; README's SPIKE-ORDER and "Spike order (J6 (5))" rows; each ADR's header line "Signed off by Mustafa" (see STATUS-FIELD) | engine-architect; each author its header |
| TICK-60-HEADROOM, with the addition | ADR 0001 OQ-0001-1, part 2, the budgets table: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz; the 60 Hz pass marks (a rooms-per-VM measure at 60 Hz) | netcode-architect; test-engineer |
| TICK-60-SETUP (a), an early signal only | ADR 0001 OQ-0001-2 and part 2; ADR 0004's stand-ins (S1) where they call the run a decision; SPIKE_PLAN's 60 Hz run; the final 60 Hz call is measured again with the real sim at slice 6 | netcode-architect; rust-reviewer; engine-architect; test-engineer |
| NET-PROFILES | ADR 0001 OQ-0001-3, OQ-0001-16, the profile table: P2 = 100 ms round trip, ±20 ms jitter, 2 % loss; latency and jitter half each way; loss as P2 defines it; pass marks that call these "proposed" | netcode-architect; test-engineer |
| REWIND-CAP | ADR 0001 OQ-0001-5 and part 8: no value now; the cap covers the honest rewind measured at P2, rounded up; beyond it the victim's view wins; the smallest input buffer that holds; PM-0001-48, -49 as needed | netcode-architect; test-engineer |
| SPIKE-MACHINES | ADR 0006 OQ-0006-17 (answered), OQ-0006-15 (in part: a domain Mustafa registers, for the WebSocket leg), OQ-0006-20 (in part: VMs exist only during a run; no cap value given), OQ-0006-1 (still open: his console check), the spike-VM part; ADR 0001's certificates (a private hash-pinned certificate for WebTransport; the WebSocket leg on his domain); SPIKE_PLAN item 1's machines (the CX33 runs the simulated clients; the test PC is his own, in Brave); OQ-0003-1 (in part: the proof test's test PC; ML and RD still to be named) | infra-engineer; netcode-architect; engine-architect; test-engineer; graphics-engineer |
| LOOK-TEST-INPUTS | SPIKE_PLAN: the asset research's export and slot-naming part runs before the look proof test; OQ-SPIKE-1: Mustafa makes the test models with AI help in Blender (Spline's skins and named slots are unproven); blockouts with a skin, named colour slots and LOOK_V2's proportions are enough; ADR 0005's asset pipeline questions that bear on the route (OQ-0005-AS3, AS4); ADR 0003 part 11's inputs | engine-architect; graphics-engineer |
| TOY-BOX | README's row: reword CLAUDE.md line 4 and the art-director's file line 10 to point at LOOK_V2, in a small kit change-set after PR #30 is updated, with the checkpoint anti-waste rules (not this PR) | engine-architect; docs-keeper |
| OQ-0002-8 (a) | ADR 0002 OQ-0002-8; the three "not researched" crates stay labelled, now accepted | engine-architect |
| OQ-0005-1 | ADR 0005 OQ-0005-1: presentation in the client's data, keyed by the same id; the content check fails if a simulation id has no client entry (the whole-set check); a pass mark bite | engine-architect; test-engineer |
| OQ-0001-14 | ADR 0001 OQ-0001-14: a VISION_V2 §3 or §4 trace counts, marked as such; README "Needs outside VISION_V2 §6", J3 | netcode-architect; engine-architect |
| ROOM-CRASH (OQ-0001-18) | ADR 0001 OQ-0001-18 and its failure row: end the match, keep the crash and match records; ADR 0006 where it keeps those records | netcode-architect; infra-engineer |
| OQ-0003-17 | ADR 0003 OQ-0003-17, part 7; PM-0003-44: at most 6 of 600 presented frames miss a refresh with the very large units on screen (RD, High); the p95 rows stay as diagnostics | graphics-engineer; test-engineer |
| OQ-0003-20 | ADR 0003 OQ-0003-20, part 7; the soaked set's rows: judged after 15 minutes of play | graphics-engineer; test-engineer |
| OQ-0005-AB12, AB13 | ADR 0005's ability section | game-designer |
| LATER | README's later rows: a design session after the netcode proof test; STATUS | engine-architect; docs-keeper |

## Rules for this round (every author)
1. Record only what the words say. Each answered entry quotes the answer: "Answered by Mustafa (2026-11-03): …" or
   "Answered by the Advisor (2026-11-03, Mustafa's message): …". Where an answer settles part of a question, say what
   stays open and who decides it.
2. Keep each question's options (the record of what was weighed); "proposed" becomes "decided" only where the answer
   settles it.
3. Each author edits only its own text (pass marks and the Brave checklists: test-engineer; ADR 0005's sections as
   before); a block in another author's text goes to that author.
4. The plan's rules 1–12 (appendix §4) still hold: sources, no GLOSSARY "Retire (exact)" phrase, no sentence under a
   table, nothing decided about the look beyond LOOK_V2, no settled input reopened.
5. Save as you go (Mustafa's rules of 2026-11-02, in every prompt); no binaries and no file over 1 MB in scratch.

## Decisions this plan asks for
- **STATUS-FIELD** (the advisor's, or Mustafa's): what each ADR's header says now. (a) "Status: Accepted (2026-11-03,
  the basis for the spikes)" and "Signed off by Mustafa: yes"; (b) "Status: Proposed" kept and "Signed off by Mustafa:
  as the basis for the spikes (2026-11-03, SPIKE-ORDER (b))", Accepted once a draft's spikes pass their marks.
  Recommend (b): the pass marks decide, and each draft's "When a mark fails" reopens the parts it names.
- **POINTERS** (the advisor's): README's 94 "[ADR 000N](…) line L" pointers. (a) re-pin them after this round with
  `scripts/draft_pointers.py`; (b) drop the line numbers (every row already names its OQ id), so later edits cannot move
  them. Recommend (b): the script leaves the branch with the checkpoint, and the report's risk 2 goes away.
- **LOOK_V2-§8** (for the report, not this PR): Mustafa's Blender route for the look test's models differs from LOOK_V2
  §8's Spline flow. The drafts record his answer for the test models and do not edit LOOK_V2 (his and the Advisor's
  file); the report asks whether §8 changes.
- Mustafa's reason "seven usage-limit cuts in three days" is quoted as his words; the report's own count stays nine.

## Steps
1. (session) This plan; the words saved verbatim; the background saver restarted (rule 3).
2. (workflow) A mapper (engine-architect, read-only) lists every place in the nine documents each answer touches, by
   author; then the advisor reviews this plan with the map: APPROVE, REVISE or ESCALATE.
3. (workflow, after APPROVE) The authors write exact FIX blocks for their own text, from the map; the session applies
   each after the ownership and line checks; then the pointer step (POINTERS) by engine-architect.
4. (workflow) fixcheck-e: a fresh verifier checks each recorded answer against the words file (faithful, partial
   answers flagged), rebuilds the drafts from the blocks, checks consistency (P2's values, the cap rule, the 60 Hz rule,
   wave 1's order the same everywhere) and the counts (open questions after the answers), runs the scripts; a skeptic per
   wrong or unsupported verdict; owners fix; the session applies.
5. (docs-keeper) STATUS: the answers recorded (pointing to the appendix for the words), the next step (the netcode
   proof test, its own change-set, plan first), wave 1 one heavy spike at a time, TOY-BOX's kit change-set after PR #30,
   LATER's design session; ROADMAP; REPO_MAP (the checkpoint folder leaves the branch).
6. (session) The report and appendix: §2 the words; an "At the stop" section (each answer and where it landed, by
   commit); the checkpoint's essentials folded in (steps with SHAs, key numbers, decisions, open findings); PR #33's body
   = the report, with "Merge by squash only (Mustafa, 2026-11-03)" on top.
7. (session) The last commit deletes `.checkpoints/v2-decision-drafts.md` and `.checkpoints/v2-decision-drafts/` and
   runs CI; `git ls-tree -r --name-only HEAD -- .checkpoints` prints nothing; the gate; then mark PR #33 ready.
8. (advisor) Reviews the final head: the PR's diff against 9e7db28, the report, CI, no `.checkpoints/` files.
9. (Mustafa) His go in this chat for that head's SHA; the session squash-merges with that SHA as the expected head. A
   later push needs the advisor's review again, then his go again.

## Evidence, reviewers, Mustafa's checks
- Evidence: the map, fixcheck-e's verdict file, the scripts (write guard, dispositions 521/521, glossary, tables,
  counts), the gate, CI on the last commit.
- Reviewers: the advisor (this plan and the final head); fixcheck-e. No domain reviewer: the answers are the deciders'.
- Mustafa: nothing in Brave (nothing a player sees changes); his go for the merge.
- Night queue: nothing (the next spike needs its own plan and Mustafa's VMs).

## Risks
1. Over-reading an answer (OQ-0003-1, OQ-0006-1, -15, -20 are answered only in part): rule 1, and fixcheck-e checks
   every recorded answer against the words.
2. Line pointers move with every edit: POINTERS.
3. Deleting the checkpoint takes the notes, prompts and scripts off the tree: they stay in PR #33's commits (GitHub
   keeps a PR's commits after a squash merge); the appendix names the last commit that has them, and README's sentence
   on review codes says "PR #33's commits" instead of "the branch's history".
4. A merge that is not a squash would bring 294b3cf's 31 MB into main's history: the PR body says squash only.
5. Usage cuts: the save-as-you-go rules; small units; the saver on.
