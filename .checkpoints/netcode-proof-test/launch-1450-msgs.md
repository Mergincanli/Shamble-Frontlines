# The 14:50 UTC window (2026-11-26): the order and the messages, drafted 12:5x UTC by the main session
LEAN-MODE: new agents, and resumed agents with new work, only 14:50-15:20 UTC; at most 3 at once. Step 0: df, `git
rev-parse HEAD` (fill `<head>`), no merge of mine in progress (else the launches wait for its push, inside 15:20).
- **Order:** A and B and C together at 14:5x. When B ends (if before 15:20, his S14-PUSH answered and B's verdict
  APPROVE or conditions applied word for word), D in B's slot. If Builder 2's merge reached its step 9 before 14:50,
  NB2-C10's reviewer read (one reviewer agent on 72651a3c's last 67 judge lines: advisor-nb2-merge.md:121-126) takes
  D's slot, and D goes to 19:50.
- Every message carries, word for word: "git and gh always start their own command, with no other program in front of
  them." and "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo
  fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with
  Write." A, B and C: read only, except their verdict files; no cargo; no commits (the main session saves); never
  switch the main folder; never `cd` into another worktree.

## A. To the advisor a0530735048f1620a (summary: "U7's snapshot read")
Your read of U7's fix-pass snapshot, as your advisor-u7-round.md section 4 set it (not a round): each item's red and
green, its plants, run 4's marks, F3's press counting for the netcode question, F3's shortfall against the brief, and
the builder's four process slips. Inputs: wip/u7-build-rs.patch (sha256 6535c181…, 267,337 B, 43 files on 80b6748),
the notes wip/u7-build-rs-notes-1055.md (a65282fa…), the logs under .gate/u7/ and the builder's worktree's
.gate/u7/t5ii/ (run 3: 2026-11-25T19-53-08-358Z; run 4: 2026-11-26T10-48-42-618Z; read only).
New since your ruling:
1. His answers, word for word in history.md:8068-8088: U7-MARK9 (a) and U7-MARK8B (a). Their record is u7-brief.md's
   addendum under :199. Check its wording against his words and the two runs' summary.json files.
2. Mark 8: the driver reports it "as observed", as your brief asked. Is a hand read against the restated bar (run 4:
   1.56 s normal, 4.07 s planted) enough for U7's report, or must the driver judge it? Rule.
3. Mark 9's stand-in includes F3's legs, whose plant the brief's leg cannot catch (only the builder's added narrow leg
   does). Answer by name: is F3 as built a sound stand-in for mark 9 under his (a)?
4. A cross-check nobody has run: the snapshot changes crates/bots/src/load.rs and lib.rs (npt-load's LoadSettings,
   `deny_unknown_fields`). U8's r1.sh writes load_settings' 24 keys and machine's 5 (r1.sh:340-348), and U8's tests
   read none of U7's files. Compare the hunk with those keys: M5's class of defect, which only R1 would show.
5. U7's merge plan (u7-merge-plan.md, U8's M0-M9 form) if it is in the folder when you start; else I bring it at 19:50.
Write .checkpoints/netcode-proof-test/advisor-u7-snapshot.md: a verdict (MERGE, or FIX with items), the findings with
their severity, and U7's merge's conditions. End your turn as soon as the verdict is in the file.

## B. To the advisor a3de649cd25e474df (summary: "U8-F1's brief check")
Your one check of U8-F1's brief, u8-f1-brief.md (written from your advisor-u8-head.md), against his RESULTS-PUSH
answer, word for word in history.md:8068-8088: "RESULTS-PUSH: (b). Only numbers leave the server and the push stays
summaries. U8's new stop is 6,230 lines." Check: the 41-line budget and its carry order; the fresh worktree (U8's
builder's own cannot fast-forward: ffcheck found 44 ADDED paths present); item 1's prints (numbers only; are the two
policy strings inside his words?); item 1c's list and its `<S14>` fill (S14-PUSH: his answer `<answer, or none yet>`);
item 2's test design; the round. Also rule: may item 2's four-way proof be a scratch run under .gate/, recorded in the
builder's notes and not counted in the 41, so that the one PROMPT-R1.md line always ships (your head read says the
builder "proves it on a throwaway repository", not that the proof is a committed test)? It may be 15-20 lines alone;
if yes, the line is never carried, only its test. S14-PUSH's options for `<S14>`: (a) s14/ whole; (b) a derived
summary (a new item, re-checked); (c) neither (S14 NOT-RUN at R1). Write
.checkpoints/netcode-proof-test/advisor-u8-f1-brief.md: APPROVE, APPROVE WITH CONDITIONS (each worded to apply word
for word) or REVISE. End your turn at once: the builder takes your slot.

## C. To a new advisor (general-purpose, opus, effort xhigh; summary: "MAIN-68's plan, then the take-in recipe")
You are the project's advisor agent: first read `git show origin/main:.claude/agents/advisor.md` whole and act by it.
Two recorded checks, in this order, each in its own file:
1. MAIN-68's plan (main-68-plan.md): taking main 2483e9a (#70, change-set 4a) into this branch, in MAIN-66's approved
   form (main-66-plan.md with advisor-main-66.md's M66-1 to M66-5) as MAIN-67 applied it (main-67-plan.md;
   advisor-main-67-head.md approved its pushed head). Re-derive its facts read only (`git diff --name-status
   07d673b origin/main`, `git merge-tree --write-tree --name-only HEAD origin/main`, main's CI by gh) and answer its
   Q1-Q4. Write advisor-main-68.md: APPROVE, APPROVE WITH CONDITIONS or REVISE.
2. LIGHT-TAKEIN's recipe (light-takein-recipe.md), his LIGHT-TAKEIN (a) word for word in history.md:8048-8066:
   "Write the recipe once and have the advisor check it." Answer its Q1-Q4. Write advisor-light-takein.md.

## D. U8-F1's builder (a new general-purpose agent, opus, effort xhigh, isolation: worktree)
u8-f1-brief.md as checked (B's conditions applied word for word), `<base>` and `<S14>` filled; sent in full.
