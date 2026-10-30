# Advisor verdict on plan-amendments-2 (verbatim)

The advisor agent, workflow run wf_328b784b-e78 (prompt `prompts/advisor-amendments-2-review.txt`), returned 2026-10-30 about 17:20 UTC. Notes: `advisor-amendments-2-review.md`.

**Verdict: APPROVE, with 13 conditions.** The amendment carries out the Advisor's five decisions of 2026-10-30 faithfully and fits the approved plan, the first amendments (8 conditions), CLAUDE.md and CHECKPOINTS.md. The conditions tighten how it is run. None needs a new plan round: apply them in the amendment file and in the prompts before launch.

**TL;DR**
- The resolve chain is right: an engine-architect proposal per contradiction, my review, the owners apply, a verifier checks the diffs. It needs a third outcome ("one shared open question"), a set order between items, and a way for an owner to object.
- Of the 12 contradictions, I expect to decide 7 myself. 1 gets one shared wording now and Mustafa's call after a Feel Lab test. 2 are Mustafa's now. 2 belong to the Advisor's deferred security baseline.
- Fetching should be narrower than written: only the step-3d verifier needs to fetch, under stricter curl rules.
- The unread-sources list can be built mechanically if every cell cites its basis and a script proves no source was dropped.
- NS-2: listing the new routine for Mustafa is the right fallback. Nothing else is needed tonight.

**What I checked myself**
- **The plans:** `plan-amendments-2.md` against the Advisor's words in the checkpoint, read word for word. Also `plan.md`, `plan-amendments.md` and my earlier verdict.
- **The critic's list:** 41 items (12 contradictions, 13 hidden dependencies, 15 gaps, 1 wrong cross-reference). The plan's "other 29" is correct. Four of the critic's quotes match the files (r1:2012-2018, r2:2376-2380, r2:2731-2734, r5:1315-1319).
- **Not-read tables:** there are 8, not 6 (R5 has three). Some rows have no URL (R1's rand book row; R5's "Netlify CLI deploy…"). R3, R4 and R6 have no unit column. All six topics have a "Recommendations that hinge on a source not read" table.
- **SEC-SPREAD** is already Mustafa's open call (r2:2893-2899, r2:3572).
- **R2-BUD-REC6** says "no source read gives a tick budget", so the budget shares are proposals and choosing among them is an engineering call.
- **The fetch audit** (`wave1-tally.md:43-54`) covers the requested hosts only, not redirect targets. The one off-list host is api.github.com (the #29 review).
- **Code clash:** "NS-2" was already used for an earlier decision, deleting the three old routines (`LEGACY_INVENTORY.md` KT14).
- **Term counts per topic** show how big the glossary job is (for example "tick budget": 2/10/0/1/0/3 across R1–R6).

**Conditions**
1. **A third outcome.** Besides "resolved" and "needs a decision", add "aligned": every affected topic states the same open question, in the same words, with one decider (a named spike and pass mark, or a person) and a decision code. The report counts settled, aligned and escalated items separately.
2. **Order and dependencies.** Step 3a does #10, #11, #9 and their glossary terms first, then #1; #19 before #21 and #22; #5 together with #6. A resolution that depends on an escalated item must hold under every option (write it in terms of "the largest mode's N"). A glossary entry that depends on an escalated item states both readings and the code, and picks neither.
3. **Step 3a works in small units** (CHECKPOINTS §6): each term and each contradiction is its own unit, written whole. `resolutions.md` gives, per item, both sides' words with their ids, the exact before/after text per file, and the basis ids.
4. **Numbers.** Every changed number either comes from the files (with its id), is arithmetic on them shown in `resolutions.md`, or is a pass mark labelled "proposed" with its reason. No new sourced fact.
5. **Step 3b effort.** I run at xhigh: tick budget, prediction and the random streams are netcode and architecture, and the plan leaves this effort unstated. The verdict goes to `advisor-resolutions-verdict.md`. The session may ask the decisions below now, without waiting for step 3a.
6. **Applying (step 3c).**
   - An owner who finds an approved resolution wrong for its file does not apply it. It logs OBJECT with the reason, and I decide.
   - Owners change nothing beyond the approved text, the glossary terms and nits. Quoted source text never changes.
   - The session records the SHA before step 3c and commits each owner's file as it comes back, after the write-guard check.
7. **Fetching (VERIFY-FETCH (a)).**
   - Steps 3a and 3c fetch nothing, since they may add no sources. Only step 3d fetches.
   - curl rules:
     - GET over https only (`--proto =https`), with no data, upload or method flags.
     - No `-L`: a redirect is recorded, and followed only as a new fetch when its target host is on the allow list.
     - No auth headers, cookies, netrc or config files; time and size caps.
     - Output with `-o` into a fresh empty folder in the scratchpad. Downloads are never run; Python on them only with `-I`.
     - Record the HTTP code and the final URL.
   - The host audit lists requested and final hosts, by tool.
   - The report says past curl use is covered by (a), except the 3 api.github.com calls of the #29 review. That host is off the allow list, and that was a PR review, not research.
8. **UNREAD_SOURCES.md.**
   - **Inputs:** all 8 not-read tables plus the 6 hinge tables. A URL-matching rule is stated (scheme, `www.`, trailing slash). Rows without a URL stay as their own rows, in the topic's words.
   - **Brief question:** from the unit column, or for R3, R4 and R6 from the per-question draft that names the URL.
   - **"Needed for" cells:** each cites topic ids, or quotes the notes with the file at a commit SHA (the notes folder is deleted at the end), or says "no tie found in the file or notes". "Search lead" only where the topic's own words say so.
   - **Check:** a script shows every source URL appears exactly once; the counts go in the report.
   - **Timing:** build and push the file first, so the Advisor can start looking for copies. Re-run the script and refresh the ids after step 3c. Step 3d samples 10 rows.
9. **The glossary must be checkable.**
   - Each entry gives the term, its one definition, the words that mark it, the synonyms to retire, and the ids that use it in each topic.
   - After step 3c, a search script lists every hit of each term and each retired synonym per topic. It must find zero retired synonyms outside quoted source text.
   - Each topic header links GLOSSARY.md, and so does the first use of each term. The "used in" lists are refreshed from the script's output, which goes in the report.
10. **Step 3d in three units, at most 4 agents at once:** R2 at xhigh (as every earlier R2 check), R1 + R4 at high, R3 + R5 + R6 at high.
    - In R3–R6, every changed recommendation gets at least one basis claim checked at its source, unless its verifier table already confirms one.
    - In R1 and R2, every changed basis claim not already confirmed in its own topic is checked.
    - The "N of M" labels are recounted.
    - A value that changes because of a definition is treated as a recommendation change (as the plan already says) and checked here.
11. **The other 29 items.** Rank 41 (the wrong cross-reference) is fixed in R5 as a nit: cite R1-BND-REC2 mark 5, rather than filing it as an open question. Items a resolution settles are marked "settled by resolution <rank>". Each shared open question carries its decision code, so a search finds every copy.
12. **The report and evidence.**
    - The 12 resolutions and my per-item verdicts go into the report word for word, because `resolutions.md` is deleted with the checkpoint.
    - Add short risk and evidence lines to the amendment: a spend cut in the middle of step 3c, a resolution taking Mustafa's call, R1 or R2 losing its "verified" label. Evidence: the script outputs, the verifier tables, the gate, the repo-map check.
    - Answers from Mustafa or the Advisor that arrive before the PR is marked ready are applied the same way (owner applies, verifier checks); later ones go to the synthesis.
13. **NS-2.**
    - Call it "NS-2 (2026-10-30, a fresh session per run)", because of the code clash above.
    - Mustafa's item lists the exact settings, with the one-line prompt copied word for word from the old routine.
    - It also gives the order: create the new routine, then disable the old one in the same sitting. Two enabled routines would both work the queue, posting as the same account, and the busy lock can't tell them apart. Delete the old one after one good night.
    - The session then reads both routines back to confirm only one is enabled.

**Nits**
- The write-guard stand-in should also allow `.checkpoints/v2-research.md`: the checkpoint file sits beside the folder, not inside it.
- Until the WINDOW-0730 rules PR lands, NIGHT_SHIFT.md still lets a night run work until 08:30. Research launched between 07:30 and 08:30 should count that run in the 4-at-once limit.

**Who decides each contradiction (by the critic's rank)**

| Rank | What clashes | Who decides |
|---|---|---|
| 1 | What the tick budget covers, and its shares | Me (engineering), after #9/#10/#11. The 60-or-30 Hz rule stays as it is. |
| 5 | Whether the client predicts the player's own abilities | Shared wording now (R1-ABL-REC4 depends on R2-SYNC-OQ10). The final call is Mustafa's (game feel), after a Feel Lab test. Nothing to decide now. |
| 6 | Spread random stream: in the sim state or server-only | Me for the design (R1 adopts R2's server-only stream, kept out of the client's state and hash). The tracer-against-cheating trade-off stays Mustafa's existing open decision SEC-SPREAD. |
| 9 | The stress scene (vehicles, projectiles) | Me: one proposed worst-case scene. Caps later (R2-BUD-OQ5). |
| 10 | Mode sizes: totals or per team | Mustafa now (MODES-3 below). |
| 11 | One core or one vCPU as the measuring unit | Me. |
| 17 | Who owns the dependency audit in CI | The Advisor (BASELINE-WHEN below). |
| 19 | Previews on Cloudflare Workers or Pages | Me. Whether Pages can delete a branch's latest preview (R5-PREV-OQ2) becomes a shared spike item. |
| 21 | Login wall on previews | Mustafa (privacy against convenience), now (PREVIEW-WALL below). |
| 22 | Isolation headers on every preview, or behind a flag | Me. |
| 23 | One wildcard certificate (one private key on every server) | The Advisor (BASELINE-WHEN below). |
| 29 | HashMap: lookups allowed, or the type banned | Me. |

**What I could not check**
- The routines: no routine tools here. That covers the cron being back on 0-7, and the test routine created and then deleted (`sources: []`).
- Issue #16's edit: no network reads, as instructed.
- The agent transcripts: I relied on `wave1-tally.md`'s audit.
- Whether this harness puts any permission rule on curl.
- I did not run the gate: this is a plan review and nothing was edited.

I edited nothing and committed nothing. My notes, ending in DONE, are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-amendments-2-review.md`.

## Decisions needed
- **MODES-3 (Mustafa): how should VISION_V2 §3's starting sizes be read?** The text says "two teams with 2–8 players, and three teams with 6–18 players".
  - **Why it matters:** it sets the largest mode that every pass mark, the offline test and the server cost rows use (48 players in R1, 54 in R2).
  - **Options:**
    - (a) Totals: 1v1 to 4v4, and 2v2v2 to 6v6v6. The largest mode is then the goal, 24v24 = 48; the three-team goal is 12v12v12 = 36.
    - (b) Per team: up to 3×18 = 54 players.
  - **Recommendation:** (a). Under (b), the "start small" modes would be bigger than the 12v12v12 goal. A headroom run above 48 can be labelled as headroom ("design for 48+").
- **PREVIEW-WALL (Mustafa): should per-PR preview links that talk to the staging game server be private?** Private means behind a Cloudflare Access login on your account; otherwise anyone with the link can open them.
  - **Why it matters:** R5 turns the wall on once a preview reaches a real server, and R6 leaves it optional. It trades one login per Brave check against strangers reaching unreleased work and the staging server.
  - **Options:**
    - (a) Private from the first preview that reaches the staging server.
    - (b) Open, since previews hold no secrets.
    - (c) Decide after the security baseline.
  - **Recommendation:** (a).
- **BASELINE-WHEN (the Advisor, relayed by Mustafa): research the deferred security baseline before ADR 0006, or list it open there?** Two contradictions turn on it: who owns the dependency audit in CI and whether it blocks a PR (#17), and one shared certificate key on every game server (#23). Four more R2 and R5 items wait on it (critic item 18).
  - **Why it matters:** until it is answered, those items can only become shared open questions.
  - **Options:**
    - (a) Research it now: one security-auditor topic, before the drafts.
    - (b) List it open in 0006, with interim positions: R5's scheduled advisories check stays the mechanism, and the baseline sets the policy later. R2's one certificate per hostname stays, and R6's wildcard becomes an option the baseline weighs.
    - (c) Leave both as one shared open question.
  - **Recommendation:** (b). It settles the topics' wording now without deciding the baseline, and the safer certificate choice weakens nothing.

## For Mustafa
1. NS-2: on the Routines page, create the night-shift routine that starts a fresh session on every run. Use the session's exact settings: the same one-line prompt, the repository, the GitHub connector, Opus 5.5, `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`. In the same sitting, disable the old "Night shift" routine (trig_019LtRqmAdzddyk6K3e3kDJz), and delete it after one good night. If you can't do it before 00:10, the old routine runs tonight as before, which is fine.
2. Answer MODES-3 and PREVIEW-WALL (now, or when step 3b brings them back), and pass BASELINE-WHEN to the Advisor.
3. Later: say "merge #28" once the advisor approves its head.
