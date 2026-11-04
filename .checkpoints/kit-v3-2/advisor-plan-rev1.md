# advisor: plan review of kit-v3-2 (A2) revision 1, on main 8bcc65e
## Plan
1. Words vs plan (a): compare queue item A.2 to plan §1-§4.
2. Agent cap vs CLAUDE.md ultracode/workflow (b).
3. Edits vs every file restating checkpoint rules (c): CHECKPOINTS.md, CLAUDE.md, NIGHT_SHIFT, agents, skills, docs.
4. TOY-BOX vs LOOK_V2 (d).
5. Reviewers and effort (e).
6. Gaps (f), verdict.
## Findings
- (a) words: queue A.2 read; plan §1 quotes it verbatim (checked char by char against queue file line 9). Block 1 done.
- (c) CHECKPOINTS.md old texts C1,C2,C5,C6,C8,C9,C10 match main lines 3-5,44-46,60-61,66-69,70-71,75,77-85. CLAUDE.md on main = worktree (8bcc65e).
- (d) LOOK_V2 exists (78 lines), §3 line 27 "audience is adults"; §8 has [OPEN] tree style; §1 names "Red Alert 3" as a look reference.
- (c) all old texts for T3,T4,T5,S1-S4,N1,K1 match main. 28 "CHECKPOINTS.md §N" citations confirmed; 9 sections.
- (c) saver log strings named in C11 exist (checkpoint.mjs:43,127-205). save stages `git add -A -- .checkpoints/` (:151): any file, binaries included -> C7 risk real.
- (c) gate scans only the five game files (gate.mjs:34): scratch .js in .checkpoints/ is not gated. routine-guard :469-471 refuses .gate/ and untracked scripts.
- (c) gap: START_HERE:112, tools/README:4, REPO_MAP:24 call .gate/ "scratch": left to docs-keeper as optional; reads against the new rule.
- (c) gap: plan risk 1 says the advisor checks the cap, but advisor.md:59 plan-review bullet is unchanged.
- (c) minor: C1 lists §4,§5 only, rules also land in §3 (C3) and §7 (C12). "block" (new) vs "unit" (§6, CLAUDE.md item 4): two words, no link.
- (d) art-director.md:27 "the upbeat tone": his words say "the art-director's matching lines" (plural); plan defers it to review.
- (d) T3 drops "readable", not a toy-box word; LOOK_V2 §5 keeps readability.
- (e) art-director, security-auditor, verifier, docs-keeper, advisor: all opus/high in frontmatter. high is right (not ADR/netcode).
- (f) E9 tests only the DONE resume; the cut-off resume ("continues after the last complete block") is untested.
- (b) AGENT-CAP framing correct; "per session" is itself a reading (usage limit is per account) and should be said in the decision.
- Blocks 1-6 done. Verdict below.

# Verdict: APPROVE, with conditions (build after Mustafa's answers to the decisions)

Plan kit-v3-2 (A2) revision 1, on main 8bcc65e. Checked by git, Read and Grep against main; no repository file edited.

## What I checked
- (a) His words: the plan quotes queue item A.2 word for word (plan:7-13 = queue file line 9). Each of his rules has a
  home: agent cap C4/C12/M1/M3/K1/N1; plan first C5/C6/C8/C10; block to disk, notes line per finding, restart after
  last complete block C5/C9/C10; scratch C7/C10/M1/M2/S1-S4; saver check C3/C11. Nothing is decided beyond them except
  where the plan says so: the 100 KB guide (plan:214, flagged as the plan's own number: I accept it as the definition
  of "small"), and the four readings put to him as decisions. Faithful.
- (b) AGENT-CAP: CLAUDE.md:217-220 asks for workflows on wide work and lets agents "read and analyse in parallel";
  changeset SKILL.md:25 runs reviewers "in parallel"; V2_PHASE0_BRIEF.md:113 (the Advisor's) runs research "in
  parallel". The cap collides with all three, and the plan names all three. Framing is correct and the options are
  honest. One reading is folded in silently: "per session". His "for every session" most plainly means "the rule holds
  in every session", and the usage limit is per account, so "3 across all sessions" is also a reading; per session is
  the only one a session can keep (it cannot see another session's agents). Say so in the decision (condition 1).
  Recommendation: A. It is his words read literally, it serves their purpose (a cut-off loses at most 3 agents' work),
  and it matches his own earlier choice "wave 1 runs one heavy spike at a time" (SPIKE-ORDER, STATUS). Workflows keep
  what they are for besides width (no input mid-run, cached resume); they just run 3 wide.
- (c) Every old text in C1-C12, M1-M3, T1-T5, S1-S4, K1, N1 matches main 8bcc65e at the cited lines. 28
  "CHECKPOINTS.md §N" citations on main and 9 sections (E7's numbers reproduce). The saver stages `git add -A --
  .checkpoints/` (checkpoint.mjs:151): any file, binaries included, so C7's warning is real; the gate scans only the
  five game files (gate.mjs:34), so a scratch .js in the notes folder is not gated; routines refuse `.gate/` and
  untracked scripts (routine-guard.mjs:469-471), so "a record only" is right. C11's log strings all exist
  (checkpoint.mjs:43,127-205). Gaps: conditions 3-7.
- (d) TOY-BOX: T1, T2, T4, T5 only name docs/LOOK_V2.md as Mustafa's and the Advisor's brief and keep "never decide
  the look"; T4 keeps LOOK_V2's `[OPEN]` items open (LOOK_V2.md:73 tree style). None states or narrows the look.
  Two fixes: line 27's "upbeat tone" is one of "the art-director's matching lines" (his plural; GAME_DESIGN.md:28
  "upbeat" sits in the same cluster as :17 "joyful … toy-box"), so it belongs in A2, not in a review question
  (condition 8); and T3 drops "readable", which is not a toy-box word and which LOOK_V2 §5 keeps (condition 9).
  LOOK-LINES: STATUS:358-363 lists those four lines for Mustafa (KIT-ALIGN), so offering them as a decision is right.
  Recommendation: B (otherwise CLAUDE.md:5 says the brief is still being written right after line 4 points at it).
- (e) Reviewers: art-director (the look wording), security-auditor (what reaches a public repository), verifier
  (E1-E9), docs-keeper last, advisor on plan and final diff. All `model: opus`, `effort: high` in their frontmatter;
  high is right (not an ADR, not netcode). Wave 1 is exactly 3, so the plan obeys its own rule. Not calling the
  others is right: no code, engine, CI or rules of play.
- (f) Missing: conditions 2, 6, 10.

## Conditions
1. **AGENT-CAP's text for Mustafa** gets one line: "Per session is itself a reading: the usage limit is per account,
   so 'at most 3 across all sessions at once' is the other one; a session can't see another session's agents, so only
   per session can be kept. Two sessions at once are rare (the night shift runs 00:10-08:30, the PR Advisor three
   times a day with one agent)."
2. **The advisor's plan review checks the cap.** Risk 1 says the advisor checks it, but advisor.md:59 is unchanged.
   Add one clause there (a rules file, inside this kit change-set; it is not A4's review policy): `- Multi-step work
   keeps a checkpoint (\`.claude/CHECKPOINTS.md\`), runs at most 3 agents at a time (§4), and its agents' model and
   effort follow …`. Add advisor.md to E6's file list. Under AGENT-CAP B or C, word the clause to match.
3. **C1** names every section the new rules land in: "(§3 and §5: the saver check; §4: the agent cap, plan first,
   saving each block, scratch files; §7: the cap in workflows)".
4. **One word for one thing.** C5 defines the block as §6's unit: "(a block is one unit of §6: a source's summary, a
   reviewed file, a test group's result, a section of a deliverable)". §6 and CLAUDE.md item 4 keep "units".
5. **The paste-block instruction** (CHECKPOINTS.md:76) reads "(fill in <path> and <task-slug>)", since C10 adds a
   second placeholder.
6. **The docs that call `.gate/` "scratch"** (docs/START_HERE.md:112, tools/README.md:4, docs/REPO_MAP.md:24) are
   reworded by docs-keeper in this change-set, not "may": "generated output (renders, exports, builds) goes under
   `.gate/` (gitignored)". E3 counts them as "contradicts" until then.
7. **E2** also greps 0 hits for "upbeat" in CLAUDE.md and .claude/ (after condition 8).
8. **art-director.md:27**, part of TOY-BOX: "and the upbeat tone" becomes "and the tone (legacy: Living Comic,
   ART_DIRECTION; v2: docs/LOOK_V2.md)". The art-director review still reads it; it is no longer the review's question.
9. **T3 keeps "readable":** "You are the art director of SHAMBLE FRONTLINES, a readable battlefield seen from an
   overhead camera. Its look: …" (rest as planned).
10. **E9 also tests the cut-off case**, the heart of his rule: a third start from a notes file cut by hand after
    block 2's finding line but before its done line must redo block 2 (written over, not appended), do block 3, and
    not touch block 1 (its tool calls show no Read of block 1's file). The DONE case: the tool calls show no Read of the
    three files.
11. Build only after Mustafa answers the four decisions (or says "recommendations"); then the final diff comes back to
    me (day session), and the PR is `needs-mustafa` (it changes `.claude/` and CLAUDE.md).

## What I couldn't check
- That a workflow script can hold its `agent()` calls to 3 at a time with plain script logic: I read no workflow API
  here; the build's first workflow after A2 shows it. The gate and E1-E10 run on the build, not on a plan.

## Decisions needed
- **AGENT-CAP — what "at most 3 agents at a time" counts.** Why: today workflows and 5-reviewer rounds run wider, and
  CLAUDE.md asks for workflows on wide work. Options: A every running agent in a session, a workflow's agents
  included (the session itself not); B only agents started with the Agent tool, workflows apart; C only agents that
  write more than their notes. Also: the count is per session (a session can't see another's agents), not across all
  sessions at once. Recommendation: A, per session: your words read literally, and a cut-off then loses at most 3
  agents' work. Cost: wide work takes longer by the clock, not in quality.
- **SCRATCH-HOME — where scratch files and builds go.** Why: today's rules put scratch in `.gate/`, which is never
  pushed, so a lost container loses it. Options: A text and small scripts in the notes folder, builds and binaries in
  `.gate/<task-slug>/`; B scripts that get run stay in `.gate/` (fewer permission prompts, but against your words);
  C the session's scratchpad folder as the temp folder. Recommendation: A, your words; choosing `.gate/<task-slug>/`
  as "the temp folder" is a technical choice I can take if you prefer not to.
- **SAVER-CHECK — when the background saver is checked and restarted.** Why: "at every checkpoint update" could mean
  one saver running through every task. Options: A at every update while agents or a workflow run; B always, from a
  task's first checkpoint. Recommendation: A: with nothing running, the update's own save has already pushed
  everything.
- **LOOK-LINES — reword only the two toy-box lines, or also the four "the look is not decided" lines beside them.**
  Why: otherwise CLAUDE.md says the look brief is still being written, one line after pointing at it. Options: A the
  toy-box lines only (the four wait for the later alignment pass); B also CLAUDE.md:5-6 and :23-24,
  art-director.md:16-18 and graphics-engineer.md:19-20, each only pointing at LOOK_V2 and keeping "never decide the
  look". Recommendation: B.

## For Mustafa
1. Answer AGENT-CAP, SCRATCH-HOME, SAVER-CHECK and LOOK-LINES, or say "recommendations".
2. Later, after the advisor approves the final diff: your go to merge (a rules change), in the session's chat.
3. Brave: nothing (no player-facing change).
DONE
