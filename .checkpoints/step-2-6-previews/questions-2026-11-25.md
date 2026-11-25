# Questions for Mustafa, with step 2.6's plan (lane 2, Builder V2), 2026-11-25

Saved under his rule of 2026-11-25, 01:35 Amsterdam, item 6: "From now on, when you wait on me: save the full question as
a file in your notes, push it, and name the file and the commit in your message."

The plan is `.checkpoints/step-2-6-previews/plan.md`, revision 2.
- The advisor agent approved it with four text conditions; the security-auditor approved it for him with three text
  points. All seven are in.
- engine-architect, the owner of ADR 0002, accepts the departure from its draft.
- His go on the plan covers the three workflow texts in its Appendix, by their sha256:
  - `preview.yml` c66c0b0a9351a528695908c57f7256d2cd18713219c5698a17eaebaac469c28e
  - `preview-close.yml` c2f1fa90511db0f558d4e9287d64e189877f8cbc03eb27e6efbe08787fe20e66
  - `wall-test.yml` 3af067068060971beba8444346fdb836b7779a41403ac09d25e43027e8b18507 (a throwaway)
- Nothing is built before his go.

Settled without him, by the advisor agent:
- merge, then fix on main;
- the link shows the pull-request run's build;
- main's rule has no bypass actors (read);
- the token's end date is 90 days.

## What waits on him
1. **His answers to the five decisions below, and his go on the plan.** Step 2.6's first edit also waits for his go on
   the revised header row, which he gave with #67's merge.
2. **TRIGGER: how a preview gets the deploy token without his click.**
   - (a) The preview workflow starts after the tested build finishes, always from main's copy of its file. The token
     sits in an environment only `main` may use, with no reviewer.
   - (b) The ordinary pull-request run with no reviewer: any branch could take the token.
   - (c) Keep a reviewer, which is his click per preview, refused by his words of 2026-11-19.
   - Recommendation: (a). It departs from ADR 0002 part 10's "no `workflow_run`" and from B9's interim reviewer
     (decision DEPARTURES), and engine-architect accepts the departure.
3. **ACCOUNT: where the previews live.** A Cloudflare token reaches its whole account; no single-project token exists.
   - (a) His current Cloudflare account.
   - (b) A new free account for previews only: three more clicks in the same sitting. A leaked token then reaches
     previews only, never his domain or a future production.
   - Recommendation: (b).
4. **LANE2-LOCKFILE: who writes the lockfile that pins the upload tool (Wrangler)?** His Part 15 keeps every
   `package.json` and lockfile with lane 1, and his words of 2026-11-18 to lane 2 read "You own no manifest and no
   lockfile".
   - (a) Lane 1 adds the two files on a request on issue #59: its own small pull request, about 1 to 2 working days.
     Step 2.6's merge waits for it.
   - (b) Lane 2 owns this one lockfile, for the upload tool only, outside the game's npm workspace.
   - Recommendation: (b), for speed. Either way the tool stays out of the game's install, and the security-auditor
     checks the exact files before they are pushed.
5. **DEPARTURES: may the nine departures from the drafts and the build plan stand** (plan section 6)? No draft is
   edited; each goes to the drafts' next revision as a follow-up.
   - (a) All stand.
   - (b) He names those that go back to the drafts first.
   - Recommendation: (a).
6. **MAIN-WALL: a gap in main's rule that predates step 2.6** (the security-auditor's Critical; its detail is with Mustafa, kept out of pushed files by his words of 2026-11-25, 20:35 Amsterdam).
   - (a) First, a main rule that lets only his account update main. It is its own small change-set, measured on a
     throwaway repository: day sessions, the PR Advisor and his Revert route must still merge, and his account must
     not skip `gate` and `smoke`. The token clicks then move to a short second sitting.
   - (b) He accepts the risk in writing for now, with (a) as the next change-set. This is available only with
     ACCOUNT (b).
   - The security-auditor recommends (a). The advisor agent finds (b) proportionate with ACCOUNT (b).
   - This chat recommends (b) with ACCOUNT (b), and (a) as the next change-set either way. This chat changed its
     recommendation from (a) to (b) after both reviewers had read the plan, and neither has read the change.
   - Ask: does MAIN-WALL belong with the guard-fix chat?
7. **The test repository's clean-up, at his own timing.** Switch off the hourly checker before 2 December, then archive
   the repository, then delete the old test project and its Access application.
