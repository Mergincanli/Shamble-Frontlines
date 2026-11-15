# advisor: refs clash (check.mjs referenceProblems vs PR #53's kept-logs kit files)

## Plan
1. Read advisor.md from main (done), the two kit files on origin/main, CHECKPOINTS.md section 11, check.mjs referenceProblems
   and its tests, plan.md 4.2 and 4.9, history.md 2131-2175, c62h-bound-freeze.txt.
2. Q1: purpose vs letter: what each kit file needs from spikes/.
3. Reproduce the failure on a scratch worktree under .gate/ (merge origin/main, no commit), run --only refs.
4. Q2: weigh fixes (a) exact two-path skip, (b) broader skip, (c) others; write exact old/new lines; test the fix in the
   scratch worktree (the test plus the planted bug).
5. Q3: Mustafa's word? Q4: order. Q5: deletion follow-up.
6. Return structured output.

## Notes
- Read: advisor.md (main), kept-logs.mjs and kept-logs.test.mjs (origin/main 53272a1), CHECKPOINTS.md §11, check.mjs
  :1836-1853 and :2612-2618, check.test.mjs :1560-1580, plan.md §4.2 :451-453 and §4.9 :656-664, ADR 0002:145-146,
  history.md :2131-2175, c62h-bound-freeze.txt, the card :575-700.
- Q1: kept-logs.mjs names spikes/ twice (:8 comment, :37 RESULTS = 'spikes/netcode-proof/results/'): checkSet requires
  --results under that prefix, the folder to exist, results committed in the ref, nothing uncommitted; list writes
  raw-logs.sha256 there; pack reads it. No import, no spike code run, no spike source read. kept-logs.test.mjs: the
  string in scratch repos only (:24, :207-210, :300; :214 is 'spikes' without slash). Purpose of §4.9 (ADR 0002:145:
  nothing in crates/ or packages/ imports from spikes/) intact: clash with the grep's letter only.
- Precedent: plan §4.2 :451-453 already plans a path-named exception (W-PR's workflow file, "in the same change").
- C62H-BOUND design: "No change to ... check.mjs"; no file overlap with the fix. Its order step 5: KEPT-LOGS' pack
  first if free space is low; his C62H-BOUND-YES re-judges the kept P0 set's logs after the build.
- Leaning: (c) = (a) tightened: the two full paths may name only spikes/netcode-proof/results/; FIX WITHOUT HIS WORD.
- Precedent for the advisor amending the approved plan: plan.md :18-19 "Revision 3.3 ... the units' rulings folded in,
  with Mustafa's approval of 3.1 standing. It changes no mark, run or cost." This fix changes no mark, run or cost.
- Verified in memory (no worktree made, no repo file written): scratchpad refs-clash/{edits.txt (sha256 after reflow
  in the result), lib.mjs, verify.mjs, run-tests.mjs}. HEAD adb8abe (spike files = bedca79), main 53272a1, base 4d21c1a.
  Edits: check.mjs 3, check.test.mjs 1, plan.md 2, each old text found once; both .mjs parse (acorn 8.18.0).
  Merged tree 1268 files: refs before = exactly the two kit files; after = []; HEAD alone after = []. Both real kit
  files: no spikes/ left once spikes/netcode-proof/results/ is taken out. No other file outside the skips names spikes/.
  New test + old test: pass (2/2). Plants: P1 broad skip (.claude/scripts/) fails the new test (checkpoint.mjs and
  kept-logs.mjs.orig not flagged); P2 whole-file skip fails it (kit files naming check.mjs not flagged); P3 no
  exception fails it (kit files flagged). Old test passes under all three.
- Known limit, stated in the comment: a `..` after the results prefix is not read (the kit test names one on purpose).
- Order: fix commit first (own commit; the advisor checks its head), then the merge commit alone (merge_main_53.py),
  then check.mjs whole + gate, push, the advisor on the head; then packs (not during a build; not the P0 set C62H-BOUND
  re-judges until that judge call); C62H-BOUND's build from the merged head (its brief may be written now).
- Deletion: list/pack then refuse ("does not exist"/no committed results), cache unaffected, tests still pass
  (scratch repos), archives stay with their lists in git at the closing SHA; the deleting change-set proposes §11's
  raw-log part's fate to Mustafa (kit change, his yes).
- DONE: result returned.
