# Question for Mustafa from the new lane-2 builder chat (step 2.6), 2026-11-26, after 17:40 UTC

## ORDER-WALL: is #72 the "first day-session merge" before the token sitting, or does the token sitting stay before #72's merge?
- Your message of 17:31 UTC: the token sitting waits until one day-session merge and one PR Advisor merge have gone
  through under MAIN-WALL; "Your own next merge from a day session counts as the first."
- This chat's only pull request is #72. Read literally, #72 merges to main first and the token comes after. Your start
  block (item 6) has it the other way: the token sitting (d), then the verifier and ready (e), then the merge (f).
  Taking main into #72's branch (851f004) does not count: it changed only lane 2's branch; main did not move.
- What merging #72 before the token would mean (the advisor agent's read of the code, notes advisor-head-851f004.md;
  not measured live):
  1. On main, for each pull request from this repository whose toolchain run passes: `deploy` fails at the token check
     (`hosting/preview.mjs:106-108`), the comment says "no new link this time: the upload failed" (`:430`), and the
     month's count rises by one each time (it counts GitHub's deployment records). The daily cleanup (`reconcile`) goes
     red every day at 04:41 UTC and after every close. Unless the `preview` workflow is switched off on main until the
     token sitting (your click: Actions, `preview`, "Disable workflow").
  2. `wall-test.yml` must leave the branch before any merge. Then the security-auditor's condition (d), re-running the
     environment check after the key is stored (a fresh run and a re-run), has no workflow file for the fresh run.
     Re-runs of old runs work up to 30 days after them, but whether a re-run of a merged pull request's run still starts
     and is still judged by the environment rule is not known. Condition (d) would need a new design and the
     security-auditor's word.
  3. The token's end date in STATUS (it was to ride #72) needs another docs change after the sitting; the month's count
     check (M9) must also subtract the failed attempts made before the token.
- Options:
  (a) Keep the start block's order: the token sitting before #72's merge. The first day-session merge under MAIN-WALL
      comes from another pull request merged from a day session on your go (another chat's). #72 waits for that, then
      the token sitting, then its merge.
  (b) The literal order: #72 merges first, as the first day-session merge (wall-test.yml out first; the verifier; the
      advisor agent's review; your go), with the `preview` workflow switched off on main right after the merge until
      the token sitting; condition (d) redesigned with the security-auditor before the sitting.
  (c) Something else, for example a small separate pull request from this chat as the first day-session merge (it
      needs your word for a second branch, since this chat pushes only to #72's branch).
- Recommendation: (a). Merging #72 without a key turns every pull request's preview into a failure comment and a red
  daily run, and it breaks the security-auditor's check of the wall after the key is stored, the check your sitting
  rests on. With (a) nothing in the approved design changes.
