# Checkpoint: v2-decision-drafts
- **Goal:** the v2 decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`, the spike plan, the loose ends)
  from the drafts brief (`v2-decision-drafts/inputs/DRAFTS_BRIEF.md`), then a draft PR, the report and a stop for Mustafa
  and the Advisor · plan: `v2-decision-drafts/plan.md` (revision 3: the advisor's APPROVE of revision 1 with 19 conditions folded in revision 2, verdict verbatim in `advisor-verdicts.md` §1; revision 3 applies Mustafa's UPGRADE-SCOPE update)
- **Branch:** claude/laughing-lovelace-ox61se (the session's branch, standing in for cs/v2-decision-drafts) ·
  **Base:** origin/main 9e7db28 (PR #32's merge)
- **Last update:** 2026-11-03 06:25 Amsterdam

## Steps done
(Earlier steps below; the run of 2026-11-01:)
- 5bdad85 · ADR 0006 assembled (infra-engineer's draft + test-engineer's PM-0006-1..31), write guard PASS.
- 2a0e5f6 · ADR 0003 assembled (graphics-engineer's draft + PM-0003-1..43), write guard PASS.
- 6b175de · ADR 0001 assembled (netcode-architect's draft + PM-0001-1..56), write guard PASS.
- 789bedb · ADR 0002 assembled (engine-architect's draft + PM-0002-1..76), write guard PASS.
- d321a13 · ADR 0004 assembled (rust-reviewer's draft + PM-0004-1..72), write guard PASS.
- c70c6d2 · pass-mark fix round 1 (test-engineer; 13 blocks).
- 45c957f · ADR 0001 second fix round (consistency F1, F2, F3, F8, F10, F13, F14).
- 7da03b1 · ADR 0004 fix round (rust-reviewer's 11 exact replacements).
- 03df0ae · ADR 0001 fix round (netcode-architect's 10 exact replacements; verifier F2, F4–F10).
- 09803a9 · ADR 0006 fix round (infra-engineer's 22 exact replacements; verifier findings 1, 2, 4, 7–13).
- 80a27b3 · ENGINE_OVERVIEW, SPIKE_PLAN, docs/adr/README.md, GLOSSARY §8 (synthesis); 521/521 ids disposed.
- 549490f · docs-keeper: STATUS, ROADMAP, REPO_MAP.
- cf00def · the pass-mark files' cut marker removed from all six drafts (verifier 0006 finding 3); assemble.py fixed.
- 233f6f7 · ADR 0005 assembled (engine-architect's frame with the four sections byte for byte + PM-0005-1..46), write
  guard PASS. Draft PR #33 opened on it.
1. Step 0 (brief §0): `docs/LOOK_V2.md` added byte for byte (sha256 52b4fc06…) · 6617e25; the brief copied to `inputs/`;
   plan revision 1 written · d6c71e4.
2. Plan review: the advisor's APPROVE with 19 conditions (about 18:35); folded into plan revision 2 (no second review
   needed, per the verdict); `origin/main` fetched to 9e7db28 (condition 1); the id list rebuilt (REC 165: the two R3
   rows were references, condition 6) · 5ab5eae.
3. Mustafa's UPGRADE-SCOPE update (about 18:41, in the session's chat; it replaces the brief's upgrade note): saved
   verbatim to `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; plan revision 3 applies it (settled inputs, the ADR 0005
   paragraph, the map addendum's part (e), docs-keeper's STATUS record) · f927ab3.
4. Decision map (step 2) done; coverage script: 521 of 521 ids, one owner each; addendum sent; drafting started
   (0003, 0006, 0005's ability section) · b2719db.
5. The map's addendum done (part j); 0001 and 0004 launched · deeea0d.
6. Spend-limit cut (about 19:18) recorded; resumed after the reset (22:52); 0006 and 0005's ability section finished;
   0002 and 0005's content part launched before 23:00 · this commit.

## Rules for the rest of this task (Mustafa, 2026-11-02, in the session's chat)
About 17:28: "You're halfway usage so you know, make sure to set the agents up in a way that they save their work &
progress (reduce the chance of losing work when usage is hit)". About 17:38, three additions, word for word:
"1. for the agents working at a time, including the next waves
2. Scratch files live in the notes folder (.checkpoints/v2-decision-drafts/<agent>-scratch/), never in a temp folder.
3. At every checkpoint update, check that the background saver is running, and restart it if it stopped.
Record these in the checkpoint."
How the session applies them: `prompts/_save-as-you-go.txt` is in every prompt still to launch and is sent to every
agent running (each block written the moment it is composed; a notes line after each finding; scratch only in
`.checkpoints/v2-decision-drafts/<unit>-scratch/`, kept to what a restart needs; on a restart, continue after the last
complete block); `scripts/saver_check.sh` runs at every checkpoint update, and the saver is restarted
(`checkpoint.mjs watch v2-decision-drafts --every 5`, in the background) whenever it reports "NOT running".

## Current step
Ninth spend-limit cut, 2026-11-02 about 21:20 Amsterdam ("resets 12:30am (UTC)", 01:30 Amsterdam): the touch-ups
after fixcheck-d2 were running; game-designer's touch2 (agent a62575dec0f1fe2c5) failed at 21:19 with its notes saying
F1b, F8, F9, F10 all hold, plus two lost lines of its own (AB-1's "Why" line, dropped by its RR-4 block; 0005:883's
dangling "names, with no generic-engine feature."), and no block written yet; test-engineer's touch-up had written all
11 blocks, only its final checks were missing. Mustafa, 2026-11-03 about 06:14 Amsterdam, in the session's chat:
"Continue, usage was reached". The saver was restarted (rule 3). Done since: 750d46d, the test-engineer touch-up's 11
blocks (fixcheck-d1 F6, F7, F12; fixcheck-d2 F1a, F2, F4, F5 ×3, F12, F13), each checked by the session against its
source (LOOK_V2 lines 16, 50–52, 63; `docs/AUDIT_BUGS_PERF.md` line 27; the arithmetic) and inside "### Pass marks";
checks after it: write guard PASS, 521/521 disposed, glossary exact 0, pointers 94/94, glued 0, 340 PM rows of 7 cells.
No agent runs inside the night window (condition 17: units end before 00:10; the window ends 08:30). Next, at 08:30:
resume game-designer's touch2 (F1b, F8, F9, F10, AB-1's "Why" line, the 0005:883 dangle); apply; if 0005's line count
changes, the synthesis author re-pins README's pointers into 0005 (draft_pointers.py "off 0"); then a short re-check
of the touch-ups (fixcheck-d3: 750d46d, the game-designer's commit, the re-pin; no fetching), the gate, the report's
and appendix's placeholders, the PR body, and a final commit that runs CI.

Before (2026-11-02 evening):
Eighth spend-limit cut, 2026-11-02 about 17:50 Amsterdam ("resets 7:30pm (UTC)", 20:30 Amsterdam): test-engineer A (29
FIX written) and B (30 FIX, 8 NOTE written) failed with HTTP 429; the saver had stopped (rule 3: restarted). Mustafa,
about 20:32, in the session's chat: "I hit my usage limit while you were working, but it has reset now. Please
continue from where you left off." Applied before the cut: d57dc5a (ADR 0005's ability section, 21 blocks), 5a24122
(ADR 0001 follow-up b), 72cfcc3 (ADR 0005 frame follow-up b): all decision text final; CI green on every one. Resumed
about 20:33: test-engineer A and B (deadline 21:15); launched: fixcheck-d0 (the final round's web facts, fetches before
22:00). Next: the synthesis (last author round, after A and B), fixcheck-d1 and -d2 (no fetching), docs-keeper, the
report and the PR body. No new agent after 23:00.

Before (2026-11-02 late afternoon):
Seventh spend-limit cut, 2026-11-02 about 12:15 Amsterdam ("resets 2:20pm (UTC)", 15:20 Amsterdam): netcode-architect's
final batch (written, 57 FIX and 39 NOTE, in proofreading), game-designer's (just started) and rust-reviewer's follow-up
b (just started) failed with HTTP 429; the session's worker process also restarted. Mustafa, about 17:28 Amsterdam, in
the session's chat: "Try again" and "You're halfway usage so you know, make sure to set the agents up in a way that they
save their work & progress (reduce the chance of losing work when usage is hit)". Done: every remaining prompt and every
resume message now carries `prompts/_save-as-you-go.txt` (each block written to the output file the moment it is
composed; a notes line after each finding; on a restart, continue after the last complete block); the background saver
restarted (every 5 minutes). Applied since the cut: b88714d (ADR 0005's frame and content batch, 21 blocks; ADR 0006's
follow-up b, 11 blocks). Resumed about 17:30: netcode-architect (finish proofreading), game-designer, rust-reviewer
follow-up b. Next: test-engineer A (after 0001) and B (after the ability section and 0004 b), the synthesis (last),
fixcheck-d (two units), docs-keeper, the report.

Before (2026-11-02 midday):
2026-11-02 about 11:15 Amsterdam. All ten step-10 reviews are in (review-*.md), with fixcheck-a, -b, -c (PASS WITH
NOTES) and skeptics 4 and 5 (all upheld). REVISE verdicts: 0001 (engine-architect small, game-designer, netcode-
architect small, perf-auditor small), 0002 (security-auditor, rust-reviewer small), 0003 (perf-auditor small,
art-director), 0004 (game-designer small), 0005 (graphics-engineer; one ability pass-mark row, game-designer), 0006
(security-auditor, infra-engineer), SPIKE_PLAN (test-engineer); everything else APPROVE or APPROVE WITH NOTES. Every
finding is sorted by author in `final-round-by-author.md`. The final round (one batch per author; prompts
`prompts/final-*.txt` from `_final-round-TEMPLATE.txt`): wave 1 running since about 11:14 (netcode-architect
ac406e7aab2effb19, infra-engineer ad9e6818495220536, rust-reviewer a9ed1c468597c7711, engine-architect 0002
a9eb5c3cc8af03805, graphics-engineer aa8bf97f5e6ca4e8d; all base 47eadd9); next engine-architect 0005 and
game-designer (base 47eadd9), then test-engineer A and B (base: the commit after wave 1), then the synthesis (last),
then fixcheck-d, docs-keeper (`prompts/docs-keeper-2.txt`), the report.

Before (2026-11-02 morning, the sixth cut):
Sixth spend-limit cut, 2026-11-02 shortly after 09:48 Amsterdam ("resets 9:10am (UTC)", 10:10 Amsterdam): the reviews
by test-engineer, art-director, graphics-engineer and rust-reviewer and verifier fixcheck-c failed with HTTP 429 (notes
kept). Done before the cut, 2026-11-02: fixcheck-b and fixcheck-a (PASS WITH NOTES), skeptic-4 (5 of 5 upheld), the
synthesis round (47eadd9, 53 blocks; CI green), the reviews by engine-architect (0001 REVISE small), security-auditor
(0002, 0006 REVISE), infra-engineer (0006 REVISE), game-designer (0001, 0004, one 0005 row REVISE), netcode-architect
(0001 REVISE small) and perf-auditor (0001, 0003 REVISE small); every finding logged per author in
`final-round-findings.md`. Resumed about 11:07 Amsterdam: the five cut agents from their notes; then the final fix
round (`prompts/_final-round-TEMPLATE.txt`, one batch per author), fixcheck-d, docs-keeper (`prompts/docs-keeper-2.txt`),
the report.

Before this cut (2026-11-02 morning):
Fifth spend-limit cut, 2026-11-01 about 19:35 Amsterdam ("your 5-hour limit resets 11pm (UTC)", 00:00 Amsterdam): all
five running agents failed with HTTP 429 (fixcheck-a: 0001 and 0003 done, 0006 left; fixcheck-b: all checks done, its
verdicts file not yet written; the synthesis round and the reviews by engine-architect and security-auditor barely
started). The session was idle through the night window (00:10–08:30: no agent). CI green on every commit pushed that
evening (03645ce, 57c43c2, 9ed8d67, 81d2a65, aa85bf0, 3c46731, 9a03999: seven check-suite notifications, read
2026-11-02 09:14). Resumed 2026-11-02 at about 09:15 Amsterdam (daytime: fetching 08:30–22:00, no new agent after
23:00): the five cut agents, each from its notes. The report's draft (`docs/reports/2026-11-01-v2-decision-drafts.md`)
was not yet committed at the cut; committed with the next batch.

Before the cut (2026-11-01 evening):
Evening of 2026-11-01 (daytime rules: fetching until 22:00, no new agent after 23:00). Resumed at 19:07 after the
fourth cut. Applied since (each block checked to sit in its author's own text; guard, dispositions 521/521 and
glossary exact 0 before every commit): 03645ce (0002 round 2, 24 of 25 blocks; game-designer F14, F2), 57c43c2
(test-engineer round 2, 42 blocks; engine-architect 0005 round 2, 25 blocks), 9ed8d67 (game-designer X2;
infra-engineer F13 ×2), 81d2a65 (test-engineer round 3: PM-0002-3, -29, -31, 0006 rows; its NOTE deleted the note the
held engine-architect block would have replaced, so that block is not applied; 0005 inventory rows GW6–GW8, GW11, GE7;
appendix §2 additions, §3 the advisor's verdict and §4 the plan's rules, word for word, script-checked), aa85bf0
(netcode-architect: the stack's citations at 0001:40 and :718; the session's blank line under eight tables, 0001 ×5
and 0002 ×3, formatting only), f0b56e1 (0003 inventory rows ×8), 3c46731 (0004 inventory rows ×4; 0005 GW10: all 18
rows placed), 9a03999 (test-engineer round 4: 0001's 36 pointers into its own lines now name parts and rows; 0003
checklist step 29 shows GW5). The completeness critic: VERDICT GAPS, 16 items (`verifier-completeness-findings.md`);
handled so far: 4 (appendix), 5 (18 rows), 6 (two PM texts), 8 (0006:550), 14 (0001:40/:718; appendix dates); in
progress: 1 (fixcheck-a/-b), 2 (reviews), 3 (report), 7 (docs-keeper), 9 (citation flags, in the fixchecks), 10
(fixcheck-b's source checks), 11 and 15 (the synthesis round), 12 (scripts re-run at the end), 13 (gate at the end).
Running at about 19:31: verifier fixcheck-b (abd52b53795e379d5), verifier fixcheck-a (aca66e0bf178f03c4), the
synthesis round (a578d816282fbf38b; `prompts/fix2-engine-architect-synthesis.txt` + `-addendum.txt`; base 3c46731),
review engine-architect (aef73f24d3bbddc99), review security-auditor (ae2858d201b93973c). Waiting: reviews
infra-engineer, game-designer, perf-auditor (wave 1), then netcode-architect, rust-reviewer, graphics-engineer,
art-director, test-engineer (wave 2, after the synthesis round): `prompts/review-*.txt`. The report's draft is in
`docs/reports/2026-11-01-v2-decision-drafts.md` (Outcome, Evidence and Reviews still placeholders).

Earlier (kept for the record):
Fourth spend-limit cut, about 14:20 Amsterdam ("resets 5:50pm (UTC)", 18:50 Amsterdam): game-designer's 0005 round,
engine-architect's 0002 and 0005 rounds, test-engineer's second pass-mark round and the completeness critic failed
with HTTP 429 (notes kept). Applied before the cut: 0004's second round (8eb2f67; 14 of 16 blocks, F4 and F2-PMtext
held for test-engineer in `rust-reviewer-fixes-2-held-for-test-engineer.md`), 0003's round and 0005's GPU section
(b375829; 45 blocks); CI green on every fix commit. Mustafa, 19:06: "Try again". Resumed at about 19:07 (daytime:
fetching until 22:00, no new agent after 23:00).

Third spend-limit cut at about 09:27 Amsterdam ("your 5-hour limit resets 12:50pm (UTC)", 13:50 Amsterdam): the
consistency critic, the synthesis' verifier, verifier 0005 and netcode-architect's 0001 fix round failed with HTTP 429
(their notes hold their progress); verifier 0003 finished first (PASS WITH NOTES: 431 of 440 confirmed; 1 wrong, 4
unsupported, 2 citation off, 2 rule findings; 24 fetches, all 200). The session itself was cut too; the 10:40 wrap-up
reminder fired at 10:40 but reached the session only at 13:52 and is superseded. Mustafa, 13:52, in the session's
chat: "I hit my usage limit while you were working, but it has reset now. Please continue from where you left off."
His one-off night exception ended at 11:00; the normal rules apply again (daytime: fetching until 22:00, no new agent
after 23:00). Resumed at about 13:53: the four cut agents, and rust-reviewer for 0004's fixes (5 running). Prepared:
`prompts/verifier-skeptic-2.txt` (0002's and 0003's wrong, unsupported and rule findings): launched about 13:57.
0001's fixes applied (03df0ae: 10 blocks for F2, F4–F10; F8 was a citation inside test-engineer's pass-mark text that
the session assigned to the drafter by mistake: a stale line range replaced by the section's name, accepted and said so
in the report; F1 waits for test-engineer). Prepared: `prompts/fix-passmarks-test-engineer.txt` (the pass-mark rows of
0001, 0002, 0003, 0004, 0006). 0004's fixes applied (7da03b1: 11 blocks; rows 166, 39, 182, 253, N1, N3, N6; row 38
refuted, row 20 holds; the per-fragment damage question added "not yet in that register": the README's owner adds it
at the next synthesis touch). test-engineer launched for the pass-mark rows (about 14:00). Verifier 0005 done: PASS
WITH NOTES (997 of 1,001 confirmed; G10 wrong: BatchedMesh has `setColorAt`, not `instanceColor` (GPU section); A246
unsupported: AB5's "ADR 0006 cites this" (ability section, or a cite in 0006 part 12); F111/F112: OQ-0005-1/-2, the
sections' own disagreements; G22: "≤ 800 draw calls" without its hull-at-Low condition; condition 12: the GPU options
row "a look baked into each asset" should read "settled by LOOK_V2 §8"; C53: STATUS line 257 is now 314 (cite by name);
X2: a misquote of r1:1798; 8 fetches, all 200). All six drafts are now verified. The completeness critic launched (about
14:03). The 0005 fix round waits for the consistency critic, so each author gets one batch. The synthesis' verifier
done: PASS WITH NOTES (57 of 61; row 37 wrong: the overview's second §6.4 primitive is the leech trigger, not marking;
row 33 unsupported: "tools may depend on everything" vs navbake's sim + content; row 53: STATUS line numbers moved with
549490f (cite by name); row 54: the needs register misses R2-BUD-REC4 and over-includes the roof fade; drift after the
fix rounds: OQ-0001-14 (count 89), OQ-0006-5's Advisor part, 0004's fragment question not in the design register; S11
dangling; SPIKE_PLAN's "seven targets" lists six; OQ-0003-6 listed once of twice). The third skeptic (0005 G10, A246;
overview 37, 33; README 54) launched about 14:06. The consistency critic done (14 findings: F1, F3, F8 high; F2, F4–F7,
F9–F11 medium; F12–F14 low; `engine-architect-consistency-findings.md`, each with owners and a proposed outcome; all
489 glossary in-context hits judged: 3 wrong (F5's 2 + 1)). The second skeptic done (0002: row 36, F-2 upheld; F-3,
F-5, F-8 partly; F-9 refuted. 0003: 211, X3, X6, 73 upheld; X5, 100, X2 partly). Second fix round (consistency + the
skeptics' outcomes), format `prompts/fix-round-2-TEMPLATE.txt` (FIX blocks with a FILE line; NOTE blocks to put to the
Advisor): sent to netcode-architect (0001) and graphics-engineer (0003 + 0005 GPU) about 14:11; saved for rust-reviewer
(0004), engine-architect (0002), engine-architect (0005 frame + content), game-designer (0005 abilities), engine-architect
(synthesis): `prompts/fix2-*.txt`, sent as slots free. test-engineer's pass-mark round (running) also gets the critic's
F3 (0001 rows: two decimals), F7 (0004 rows) and F13 rows as a follow-up. The third skeptic done: G10, A246, row 33,
row 54 upheld, row 37 partly (its texts in `verifier-skeptic-3-results.md`; the roof fade stays in the register as
"LOOK_V2 §4, beside §6.2"). rust-reviewer's second round sent about 14:13. 0001's second round applied (45c957f: 15 blocks, no NOTE; its PM rows
-4, -5, -39, -40 still carry 16.7 / 8.3 / 4.2 ms: test-engineer's follow-up). engine-architect's 0002 round sent about
14:15. test-engineer's first pass-mark round applied (c70c6d2: 13 blocks in 0001, 0002, 0003, 0004, 0006; the glossary's
exact hits are now 0 in every document; 521/521 still); its second round sent about 14:18
(`prompts/fix2-test-engineer.txt`: the critic's F3, F7, F13, PM-0002-42, a row for 0001's restated R2-BUD-REC2 mark).
Waiting for slots: `prompts/fix2-engine-architect-0005.txt`, `prompts/fix2-game-designer.txt`,
`prompts/fix2-engine-architect-synthesis.txt` (the synthesis last, after the drafts' rounds, so its counts are final).
Then: the fix rounds for 0002 (engine-architect), 0003 (graphics-engineer), 0005 (its authors, after its verifier),
the pass-mark rows (test-engineer: 0001 F1 "whole tick", 0006 findings 5–6, 0004 TE69, and the rest), the completeness
critic, the reviews (step 10), docs-keeper's re-check of STATUS, the gate, the report, the PR body.

## Next steps (evening of 2026-11-01)
1. Apply each small round as it returns: dry run, check the blocks sit in the author's own text, apply, guard,
   dispositions, glossary; commit; push.
2. Verifier fixcheck-a (`prompts/verifier-fixcheck-a.txt`: the fix texts of 0001, 0003 and 0006; 0001's and 0006's
   citation flags) as soon as a slot frees.
3. Synthesis round 2 (`prompts/fix2-engine-architect-synthesis.txt` plus an addendum: the completeness critic's items
   5, 11, 15 and the authors' notes for README and SPIKE_PLAN) to the synthesis author, after the small rounds.
4. Verifier fixcheck-c on the synthesis round and the small rounds' texts, plus 0002's citation flags.
5. Step 10 reviews (`prompts/review-*.txt`, one template), ≤ 5 at once as slots free; every finding fixed or deferred
   with a reason.
6. docs-keeper: STATUS (the carry-forward states, the stale ADR-001 lines, the pointer to the verdict), the repo map.
7. The report, plus appendix §3 (the advisor's verdict §1, word for word) and §4 (the plan's settled inputs and rules
   1–12, word for word); a final commit that runs CI; PR #33's body; the PR stays a draft.
If time or the spend limit runs short, finish in this order: report and appendix > re-verification > reviews.
Whatever is not done is stated plainly in the report.

## Open findings
- The research has no asset-pipeline findings (ASSET-RESEARCH (a), ruled; asset-pipeline research unit before the asset
  spike).
- For the synthesis and the consistency pass: drafts cite the decision map's ids (interfaces I-n, research gaps G-n, loose
  ends LE-x, J-n), and the map is a checkpoint file deleted before the PR is marked ready: carry the needed ones into
  `docs/adr/README.md` (the register) or reword them (graphics-engineer's and rust-reviewer's notes flag it).
- Map N48 says 32 audio voices on both tiers; r3:1246 says 32 on Low and 48 on High; ADR 0003 follows the research
  (graphics-engineer). The map's part g had spikes decide R3-BUDGET-OQ2, OQ3, OQ6; ADR 0003 makes them open questions
  (OQ-0003-10, -12, -4).
- ADR 0006: ids its author thinks belong elsewhere (R5-PREV-REC3's jobs → 0002; R1-RR-OQ7 and R5-GATE-OQ4's CI side →
  0002; R2-ROOM-OQ11 → 0001); goal costs shown at two densities (8 rooms per VM: about 361 EUR; 4 per VM: about 534 EUR);
  `inputs/findings-checked.tsv` misses R6-HOST-F8 and F20, which R6's second pass confirmed (r6:1170): the topic's own
  section is the authority (the verifier prompt says so).
- ADR 0005's ability section: AB11 (on legacy numbers the Banzai blade kills through the Riot Shield) is listed for
  Mustafa; AB12 and AB13 are for the Advisor.
- ADR 0004: uses the map's interface and seam codes (I-1, I-9, I-10, I-12, I-16, I-17, I-19, I-20, I-21, S1): they need a
  lasting home (ENGINE_OVERVIEW or adr/README) or removal; OQ-0004-5 (drops and pickups) and OQ-0004-6 (bots' player
  records) may sit better with ADR 0005's upgrade questions (consistency pass); R1-XU-OQ6 disposed as answered.
- ADR 0001: R1-TICK-OQ1 (60 or 64 Hz) and R2-INT-OQ5 may belong to 0004 (consistency pass); it replaces four lines
  under TICK-30 (r1:2029/2061–2062, R2-BUD-REC1's tick half at r2:2302, r2:2415–2416, GLOSSARY:85), one more than
  condition 2 lists: the verifier checks it.
- Pass marks 0003 (test-engineer), for the 0003 author's fix round: (a) vsync makes the desktop's median ≤ 14 ms
  unpassable and the laptop's median ≤ 28 ms a hidden 60 fps demand; "WebGPU ≤ 1.1 × WebGL 2" and the overlay mark can't
  fail at vsync (proposed: measure uncapped; the browser switch is not researched); (b) the overlay's 1 % mark is below
  the harness's own 3 % spread; (c) sub-ms marks sit near the 100 µs timer step; (d) the first-sight stutter check passes
  if the browser cached shaders; (e) "zero bytes per frame" needs a planted allocation to prove the method; (f) the look
  proof test needs a synthetic recording for its backend marks; (g) the Safari 26 marks need a Mac the draft never names.
  The page's cost of the sim gates twice (R3 p99 in the bench, PM-20; R1-BND-REC1's p95 in 0002's boundary spike). The
  map's part g put R3-MEASURE-REC1/REC4 in the look test, the draft in the bench (the draft holds). The map's N12 cites
  r1:2258–2260; it is 2259–2261.
- ADR 0005 content (engine-architect): CD-7 changes R1-DATA-REC5 (in development, sim numbers apply from the next match,
  proposed; OQ-0005-CD4), so ADR 0001's `ContentReload` message and ADR 0004 part 16 ("a development reload swaps them
  between ticks") hold only under the mid-match option: consistency pass. Legacy rows ED5 and ED7 (Character Lab, Anim
  Lab) are listed by no draft. Its "For the frame" carries the record-level parts.
- Pass marks 0001 (test-engineer), for the 0001 author's fix round: the 160 kbps mean rests on R2's estimate with 32
  live projectiles, the stress scene has 300 (about 162–192 kbps with E3; with E3b the projectiles alone need 1,650 B
  per snapshot, over the 1,067 B cap), so the Rollback's "E3b about +42 kbps" holds only at 32: the draft must name the
  scene; "join within 300 ms at P2" needs a start point and a loss rule; "RTT within ±5 ms" is R5-TEST-REC4's at a fixed
  100 ms, not "at P2"; the throwaway sim has no vehicles, projectiles or effects and nothing sets the portal-change
  rate (stand-ins proposed); "What a spike must prove" omits R2-SYNC-REC7 mark 3 though Appendix A sends R2-SYNC-OQ5 to
  the spike; the tick-wrap check runs only at 60 Hz; R2-ROOM-REC2's "rooms of bots" would leave out encoding and
  transport work.
- Pass marks 0002 (test-engineer), for the 0002 author's fix round: "at most 4 Wasm calls per tick" vs the draft's own
  5–6 for the online client (gated as control-API calls, glue reported: the drafter confirms the reading); R1-BND-REC1
  mark 1 needs vsync off and a defined no-sim baseline (fixed in the method); block latency within one frame cannot fail
  on a light page (page loaded to 16 ms); "heap growth" by allocation sampling measures a rate, Chrome only; R5's L1 mark
  at P1's 1 % loss misses its 5 % band about 1 seed in 9 at 100,000 packets; server tick time in the nightly soak needs
  the reference server's vCPU; gaps: R1-DATA-OQ8 had no spike item (PM-0002-31 added), the map lists R1-BND-REC6 under the
  netcode proof test but 0002's spike item 6 holds it, no limit for the overflow checks' cost, no Mac and no 120/144 Hz
  displays named, B2 vs B3 only on an isolated build.
- Pass marks 0004 (test-engineer), for the 0004 author's fix round: "Wasm ≤ 2 × native" names no machine (both measured
  on the laptop); S7's planted divergences could pass without catching anything (now triggered at a known tick with a
  known-different value); per-phase times need clock reads S4 forbids inside the sim (phases called one by one from the
  bench); S2 cannot pass until R1-TICK-OQ7 sets N; the "at 30 and 60 Hz" halves of S3, S4, S9, S11, S13 moved into the
  60 Hz set (rust-reviewer confirms); S17 placed in the netcode proof test but ADR 0001 never mentions R1-TICK-REC4; the
  map puts R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4 in the sim core spike but no draft's table names them (SPIKE_PLAN
  places them); S9's "largest planned map" read as part 7's 2 km bound; PM-0004-2 checks the stress scene's load every
  tick; S20's online recovery needs the proof test's server.
- Pass marks 0005 (test-engineer), for the 0005 authors' fix round: "0 accepted invalid" is met by a loader that
  refuses everything (a labelled corpus with valid controls added); CD-1's "written back as the same text" may fail on
  serde_json's number format (not researched; the spike's first run settles it); R1-ABL-REC2's 1e-6 tolerance read as
  stored values, shots within one tick; "zero matches for the ids" cannot prove no ability-specific code (a rename test
  added); fuzzed definitions that load run 300 ticks each; OQ-0005-AP2 needs the look proof test to port the kit
  builders, which 0003's spike entry does not list; the glTF dependency check sits in the asset spike in one place and
  the foundation in another; "every legacy number present once" vs CD-10's drops (read as mapped once or dropped with a
  reason); CD-10's "value" read as each leaf (725 outside the look and state groups, 2,011 in the look group).
- docs-keeper's notes for later: STATUS points to `advisor-verdicts.md` §1, which goes with the checkpoint (move the
  pointer to the report's appendix before ready); four STATUS statements rest on the README draft or unfinished work
  (re-check before ready); STATUS is now 345 lines (was 288): trimming old legacy rows is a separate cleanup.
- Write guard: condition 1's `cmp - <(head -n 742 GLOSSARY.md)` fails on an unchanged file, because the base GLOSSARY ends
  without a final newline (742 newlines, then an unterminated last line); `scripts/write_guard.sh` checks the exact
  intent instead (the base file is a byte prefix of today's), so §8 can be appended and nothing above it may change.
  Report it as a deviation in form, not in intent.
- Pass marks 0006 (test-engineer), for the 0006 author's fix round: PREV-DELETE's "or the remaining case is written
  down" cannot fail (read it as a fail with its reason); with the access policy on, leftover links pass trivially (run
  with the policy off); Pages has a branch link plus one per deployment (all checked); the `wss` fallback cannot use the
  pinned self-signed certificate (the WebSocket constructor takes no hashes, R2-ROOM-F12); "nothing else" in the
  firewall leaves no admin or deploy path; rooms per VM on the throwaway sim stands for the real one only if its tick
  costs at least as much; the egress mark's reference must be recomputed from the counts run (no vehicles: 112.3 vs
  146.1 kbps); "under 5 EUR" is 36 CCX23-hours; the CX33 start-up under 2 s restored; "with interest on" needs the
  filter in the proof test's sim; the access-wall login check's place differs between the map and the draft.
- ADR 0005's frame (engine-architect): sections disagree on model ids and faction colours in sim definitions (AP-2 and
  CD-13 vs CD-6, AB-3/AB-4 and inventory row CD13): OQ-0005-1, the frame leans to client definitions keyed by the same
  id; CD-6 "a weapon definition is a weapon type" pre-empts OQ-0005-AB4 (b): OQ-0005-2; frame-only OQ-0005-3 (what the
  content hash covers) and OQ-0005-4 (Character Lab and Anim Lab, rows ED5 and ED7); AB5 says "ADR 0006 cites this", 0006
  does not; the author line follows the brief (0002 also credits test-engineer: align at assembly); spike item 4 (map
  migration) moved to slice 2.
- ADR 0002 (engine-architect): its ledger adds two tiers outside the plan's six ("cross-draft" 18, "inventory" 10; the
  verifier may fold them); ids its author thinks belong elsewhere: R5-TEST-OQ3 (DESIGN), R5-TEST-OQ2 (0001), R5-PREV-OQ5
  and OQ6 (0006); judgement calls marked: the `room` and `navbake` crates, the multiplayer-preview delivery shape, the
  wasm-bindgen CLI over wasm-pack, two labelled choices (a second production-isolation trigger, a `forceWebGL` CI run);
  loopback in the protocol crate (aligned with 0001); no legacy row's mark changes.

## Key numbers
- Script checks on the six assembled drafts (about 09:05; outputs in `consistency-scripts.txt`): dispositions 483 of
  521, each once, all in the map's draft (the 38 DESIGN ids wait for the README); citations: 3,355 research ids cited,
  0 that exist nowhere, 75 line ranges flagged (many false: groups citing two files); glossary (the research's own check,
  pointed at the drafts): 2 exact hits ("whole tick" in PM-0001-4 and PM-0001-39, test-engineer's rows: fix to "the
  server tick time measured directly"), 489 in-context hits to judge (244 of them "the largest mode, and its N").
- Dispositions across the six drafts (23:26; 0005 from its three sections' Appendix A rows): 483 of 521 ids, each
  exactly once, every one in the draft the map assigns; the 38 not yet disposed are exactly the map's DESIGN-owned ids
  (they go into docs/adr/README.md's register at step 6).
- Early script checks on the finished parts (22:57): `dispositions.py` parses 0003's 71 and 0006's 65 Appendix A rows,
  0 duplicates, 0 in the wrong draft; `cite_check.py` (a coarse pre-check: it reads a citation group's line range
  against the first file named in it, so groups citing two files give false hits) flags 7 ranges in 0003, 16 in 0006
  (3 of them clearly r2 ranges inside an r6 group), 2 in the ability section, 0 in the GPU section: hints for the
  verifiers, not findings.
- Research ids defined in r1–r6 at 9e7db28 (`inputs/research-ids.tsv`, built by script, rebuilt after the review): 533
  rows, 521 unique ids: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes (18 copies). Matches the research report's
  165 recommendations and 278 own open questions (241 + 30 + 7).
- Findings checked by each topic's verifier (`inputs/findings-checked.tsv`, script-built guide): R1 172/185, R2 136/152,
  R3 44/92, R4 49/62, R5 49/117, R6 18/42.

## Decisions taken
- 2026-10-31 between about 23:30 and 23:50, Mustafa (in the session's chat), word for word: "the next usage reset is going to be on 03:50
  AM, so you can maybe tell it to prepare accordingly (starting the task at the right time)" and "Just for today
  (tonight): 03:55, it can work until 11:00 because I will not have much time to use claude, so it can work if needed.
  You can take the lead in ensuring things get done accordingly, decide on when it start and when its done. Remember:
  just for today (upcoming session)". It sets aside, for the run of 2026-11-01 03:55–11:00 only, the brief's "daytime
  only" and the advisor's condition 17 (no new agent between 23:00 and 08:30). The session keeps web fetching to 08:30
  and later, since daytime-only exists for unattended fetching before the write guard (DOMAINS-ACCEPT (a),
  NIGHT-RESEARCH (a)). Every other condition holds. After this run the normal rules apply again.
- 2026-10-31: the drafts brief (Mustafa and the Advisor) and LOOK_V2 (attachments of the session's first message).
- 2026-10-31 about 18:41, Mustafa: the UPGRADE-SCOPE update (tracks per unit and per weapon type, pick one of three per
  level, owned by the player, surviving death and switching, reset every match; a class comes with its own weapon;
  other weapons only from enemy drops, as world pickups; progress on the weapon type; on the PlayerState). Open for him:
  carry slots, ammo for looted weapons, track levelling speed with catch-up.
- 2026-10-31 about 18:35, the advisor: plan APPROVE with 19 conditions; rulings ASSET-RESEARCH (a), TICK-60-HEADROOM
  ruled now (base 8.33 ms, p99 ≤ 6.25 ms; confirmed at the stop), GLOSSARY-8 yes (three entries and a note),
  CHECKPOINT-AT-STOP keep.

## Running work
- advisor plan review: done (APPROVE), notes `advisor-plan-review.md` (DONE).
- engine-architect decision map: done about 18:45 (521 of 521 ids, one owner each: 0001 127, 0002 104, 0003 71,
  0004 82, 0005 34, 0006 65, DESIGN 38); its addendum (part j, from line 1032) done about 18:52, owners unchanged,
  coverage re-run 521/521; GLOSSARY §8 text in part J1.
- Every drafting unit finished by 23:25 (notes DONE). The run's saver: `checkpoint.mjs watch` every 5 minutes, started
  03:56. Wave A (launched about 03:57, background agents, opus, xhigh): engine-architect (0005 frame; notes
  `engine-architect-adr0005-frame-notes.md`), test-engineer ×4 (notes `test-engineer-passmarks-000N-notes.md` for 0001,
  0002, 0004, 0006).
- Scheduled tasks (send_later into this session, human_request): trig_01BH1UStnQLgHuzsethFGmTU fired 2026-11-01 03:55
  Amsterdam (02:55 UTC): the run's start (done); trig_01WnvjBEFsLQPXjoj7WdwN7d fires 10:40 Amsterdam (09:40 UTC): the wrap-up.
- Ready for the run: pass-mark prompts filled and saved for 0001, 0002, 0004, 0005, 0006
  (`prompts/test-engineer-passmarks-000N.txt`); the verifier template now allows fetching only between 08:30 and 22:00.
- Drafting, launched about 18:48 (background agents, effort xhigh, opus): graphics-engineer (0003 + 0005 asset section;
  notes `graphics-engineer-adr0003-notes.md`), infra-engineer (0006; `infra-engineer-adr0006-notes.md`), game-designer
  (0005 ability section; `game-designer-adr0005-notes.md`). Prompts in `prompts/`.
- checkpoint saver: `checkpoint.mjs watch` ran 18:22–20:12 and ended (its 110 minutes); restarted at about 22:55.
- Spend-limit cut, 2026-10-31 about 19:18 Amsterdam: agents netcode-architect (0001), infra-engineer (0006),
  rust-reviewer (0004), graphics-engineer (0003, 0005 asset section), game-designer (0005 ability section) failed with
  HTTP 429 "org's monthly spend limit … your 5-hour limit resets 9:50pm (UTC)"; each resumed from its notes by message
  after the reset.
