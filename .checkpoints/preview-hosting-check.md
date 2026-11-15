# Checkpoint: preview-hosting-check
- **Goal:** the plan for the preview hosting check (docs/SPIKE_PLAN.md entry 6; ADR 0006 parts 1-3 and 13; pass marks
  PM-0006-1 to -10 and -32), by Mustafa's task (his words of 2026-11-14, sent 2026-11-15 15:33:37 UTC; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-15-task.md`, sha256 e86c6b53...4009). The plan only: no build, no
  secret, no edit outside this checkpoint. · plan: `.checkpoints/preview-hosting-check/plan.md` (not yet written)
- **Branch:** claude/optimistic-albattani-mdoonz (stands in for cs/preview-hosting-check) · **Base:** main
  4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-15 19:44 Amsterdam (18:44 UTC)

## Steps done
1. Orientation · the session's model read with get_session at 2026-11-15 15:36:35 UTC: `session_context.model`,
   `configured_model` and `external_metadata.last_served_model` all `claude-opus-5-5`; effort max; the branch at main
   4460ef5. The advisor tool's model, from the transcript: `/advisor fable` -> "Advisor set to Fable 5.2" (event
   9eb677a4-cd3d-4dff-8eab-c7ba5dd050e1, 2026-11-15 15:33:29.746 UTC); every assistant turn carries
   `"advisorModel": "claude-fable-5-2"` (first: event a1548b9c-0d4c-473d-9bad-97370d817f57, 15:33:47.219 UTC);
   ~/.claude/settings.json holds `"advisorModel": "fable"`.
2. Reading (no commit): CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS, DECISIONS (Queued; 2026-11-11 to 11-13 entries,
   the five moments at DECISIONS.md:470-482), SPIKE_PLAN (:1-160, entry 6 :416-438, the foundation :440-472), ADR 0006
   (parts 1-4 :128-222, part 13 :531-581, part 14 :582-587, Q1/Q2 :605-623, OQ-0006-4 :876-887, OQ-0006-13 :988-1006,
   spike 1 :1198-1222, the marks' key and PM-0006-1 to -10, -32 :1268-1376, "When a mark fails" :1401-1415, the
   hosting checklist :1436-1457, migration :1467-1500), ADR 0002 part 10 (:655-706) and PM-0002-65 to -68
   (:1331-1334), GLOSSARY §5 (:560-665), r5 Question 6 (F1-F20 :760-900; REC1-REC5 :1585-1650; OQ4-OQ6 :1855-1862);
   the first builder chat's plan at 68addf9 (Part 8 :836-876, Part 10 :880-912, decisions :1165-1268) and checkpoint;
   the netcode chat's infra digest (macOS runner Safari versions, read 2026-11-05); .github/workflows/gate.yml (pins).
3. Facts read for the plan (no commit): workflow runs started by sessions show `actor` and `triggering_actor`
   `Mergincanli` (gate run 43576714278, toolchain runs 43576714298 and 43536148433, read 2026-11-15 15:44:54 UTC with
   the GitHub MCP tool); the MCP tool `actions_run_trigger` offers run_workflow, rerun_workflow_run, rerun_failed_jobs,
   cancel_workflow_run, delete_workflow_run_logs: no method approves a pending deployment (its schema, loaded
   2026-11-15 15:44:43 UTC); the guard's merge check (routine-guard.mjs:1474-1491) refuses an MCP merge in any
   repository without green `gate` and `smoke` checks, for every session; npm's registry answers (wrangler latest
   4.162.0) and `git ls-remote` reads public GitHub repositories (actions/upload-artifact, download-artifact tags).
4. Mustafa's task message saved byte for byte by script (the file above; 5830 bytes).

5. The saver started (15:52 UTC) and the advisor tool's consultation 1 (15:49 UTC, below).
6. Doc reads by two agents, both DONE: infra-engineer (C1-C9, notes `infra-engineer-docs.md`, last hand-back after
   16:13 UTC) and security-auditor (G1-G10, notes `security-auditor-docs.md`, hand-back before 16:10 UTC).
7. Draft workflows in `tr-draft/` (not a build: text for the plan's Part 7), each parsed as YAML with PyYAML 6.0.1:
   deploy.yml (172 lines; the pull-request and by-hand deploys merged into one file), close.yml (93), probe.yml (63),
   browsers.yml (48); the two token-test variants (since moved: `tr-draft/workflows/` and `tr-draft/b9/`, finding A12).

8. plan.md revision 1 written (1,183 lines at d8451dc, 1,223 after consultation 2's fixes at 27e9f50; every embedded
   workflow block checked byte for byte against its draft file and its sha256): Parts 0-9, seven decisions, For
   Mustafa, the summary.

9. The advisor agent's plan review of revision 1 at 27e9f50 (launched about 16:28 UTC; hand-back 16:39:16 UTC; notes
   `advisor-plan-r1.md`, ending DONE): **APPROVE with conditions**, no Critical or High; Medium A1-A6 (A1 Wrangler
   installed outside the token step; A2 bash on Windows; A3 the bite-coep link and the browser run's place; A4 the
   preview builds write noindex; A5 production's one-off link; A6 his answers into DECISIONS.md), Low A7-A13. Rulings:
   TR-WHERE (a) breaks no rule (SPIKE_PLAN :12 is about where spike code lives in this repository), the four additions
   change no mark, the check is risk tier 2 (its tier-3 acts are his), the closing docs PR tier 1.
10. Revision 2 written and, after consultation 3 and its two additions, saved as done: all of A1-A13 folded in; tr-draft moved to `tr-draft/workflows/` and
   `tr-draft/b9/`; Part 7 regenerated from the files (deploy.yml 185 lines, close.yml 98, probe.yml 66, browsers.yml 51;
   every block byte-equal to its file, every sha256 and both variants' hashes checked, all parsed with PyYAML); Part
   7.0 lists the lines changed since 27e9f50 as diffs.

13. The build, on the test repository's branch `setup` (pushed without a pull request; 0 workflow runs on the
   repository after the last push, read with the GitHub MCP tool):
   - U1 fe8116d: the test site and tools/build.mjs (223 lines; plan about 190); all seven modes built locally.
   - U2 ba3946e: tools/lib.mjs, check-headers.mjs, check-sizes.mjs, probe.mjs, sets.mjs + sets.json, links.json,
     wdio/run.mjs, wdio/package.json + package-lock.json (WebdriverIO 10.1.0, published 2026-10-18, the newest at least
     14 days old; the lockfile made with --package-lock-only, 164 packages, all with integrity), README.md text (494
     lines; plan about 520). WebdriverIO's option names read from its own type files (wdio:enforceWebDriverClassic,
     wdio:chromedriverOptions.binary and the like; createWindow, switchToWindow, closeWindow).
   - U3 1b37fae: the four workflows from the plan at 2a2f426, byte for byte; sha256 prefixes 333e5a592a05,
     98b42b8e59cf, f24ad6e81daf, 6791076916ae, equal to his approved ones (checked by script before the commit).
   - Local proofs (they prove the scripts, not Cloudflare; scripts in `local-proof/`): proof.mjs 20 of 20 as expected
     (check-headers passes production, preview and preview-isolated on both links, and fails bite-coep only on
     /worker.js's COEP, bite-nocache only on the content file's Cache-Control, bite-noindex on noindex, and production's
     one-off link when the host's noindex is emulated; probe sorts 404, tombstone, wall, served, the 1010 block and a
     503 as planned); browser-proof.mjs in the preinstalled Chromium 153.0.8010.12: 5 of 5 (isolation true/true only on
     preview-isolated, the WebAssembly runs, the content file's second visit 0 bytes on production and 65,836 bytes on
     bite-nocache; on bite-coep Chromium refuses the worker's script, so the bite is caught).

15. The review round (workflow wf_c4580fcf-ece, 3 of 3 returned; notes `infra-engineer-review.md`,
   `security-auditor-review.md`, `test-engineer-review.md`): two Highs, the same fault found twice (F1/TE-1: Pages
   redirects .html paths, so every link would have failed on /index.html and /404.html), Mediums F2, SA-R3, SA-R4,
   TE-2 to TE-6, Lows. The security-auditor's ruling on his condition 2: remove `deployments: write`.
   - Fix pass 2df0112 on `setup` (scripts and page only; the four approved workflows unchanged, hashes rechecked):
     F1/TE-1, F4, SA-R7, TE-2, TE-3, TE-4, TE-5, TE-6, TE-8, TE-9, TE-10. Re-proven locally: proof.mjs 29 of 29 with a
     server that redirects .html paths as Pages does; the old check-headers (1b37fae) fails there with 308 on
     /index.html and /404.html (`.gate/preview-hosting-check/old-demo/demo.mjs`); browser-proof 5 of 5.
   - Proposed for his yes, one batched diff (`tr-draft/proposed/`): deploy.yml without `deployments: write` (SA-R1) and
     with the record artifact kept 14 days (F2), new sha256 29593af18d34...; close.yml's tombstone record kept 14 days,
     new sha256 3afdf389d636...; the token-test variants unchanged (fd3dc4c6029b, 8b745631a278). Not proposed: SA-R2
     (`actions: read`, deferred to step 2.6).
   - For him: SA-R3 (the install step keeps the token out of the scripts' environment, not out of their reach, since
     the runner has passwordless sudo: his acceptance asked); sitting 3 gains SA-R4 (after the reviewer is ticked
     again, cs/b9-edit re-run once: it must wait, and he rejects it) and SA-R8 (a screenshot of the empty "Repository
     secrets" list).
   - Results-log procedures (no code): F3 and TE-7 (each close log's "Uploads found" against the records), TE-11 (each
     SIZES line, the tombstone's size), TE-12 (the page's resource list against the manifest), TE-3's observed form of
     the bite (Chromium refuses the worker), TE-13 (root-cause a non-zero second visit). Deferred: F5, F6, SA-R5, SA-R6.

17. Round 2 (workflow wf_1f145f55-1c2, 2 of 2 returned; notes `infra-engineer-review-2.md`, `test-engineer-review-2.md`):
   both **APPROVE**, no Critical, High or Medium on 1b37fae..2df0112; F1/TE-1 confirmed fixed (both reran proof.mjs 29
   of 29 and browser-proof 5 of 5). Lows: infra L1 to L5 and test R2-1 to R2-6, follow-ups by the review budget (a Low
   triggers no push on its own). To ride the next push (with the approved workflow diff): R2-2 (done==='error' counts
   as not as expected), R2-4 (no sleep after the last manifest try), R2-1 (a proof case for the marker half). For the
   results log: L3 (index.html and 404.html not fetched, and why), L5 (an earlier run's verify reads a multi-push
   branch link as stale: an expected red). Kept as STATUS follow-ups: L1, L2, L4, R2-3, R2-5, R2-6.

## Current step
16. Waiting for his yes on the batched diff and his SA-R3 answer; round 2 (on what changed, 1b37fae..2df0112) and the
   advisor agent's review of the final head next. Earlier step, kept for the record:
14. The review round (U4): get_session at about 21:12 UTC read 717,475 of 1,000,000 tokens (72%; automatic compaction
   at 901,600), so the review goes ahead and he is asked to compact before Day 1's runs. The round runs as one workflow
   of three agents (ultracode is on; CLAUDE.md's cap of 3): infra-engineer, security-auditor (his condition 2 on
   `deployments: write`), test-engineer, each at effort high, notes `<agent>-review.md` in this folder.
   - At his request ("Clone the repository Mergincanli/shamble-preview-check into this session"), the session attached
     it with add_repo (push access, as plan A3 step 4 says) and cloned it shallowly to /home/user/shamble-preview-check
     (outside this repository's folder), add_repo at 20:01 UTC and the clone right after (transcript times). The repository is empty: no commit, no branch (`git ls-remote`
     lists nothing; the clone says "No commits yet"). Plan A1's "Add a README file" was not ticked, so there is no
     `main` for pull requests; the deny list forbids the session to push any `main`, so he adds the README on GitHub.
     register_repo_root waits for the first commit (its condition: `rev-parse HEAD` succeeds).
   - He then added README.md ("I have pushed the readme.md file"): `main` is 4a67b83 "Create README.md"
     (4a67b83b9ed672c81e1a6e3067f916841c02b4a9), fetched shallowly and fast-forwarded onto the empty local `main` (no
     reset); register_repo_root called for /home/user/shamble-preview-check. He asked for the rest of the setup in plain
     words; the steps sent: A2, A3 (SR-READ), A4, then wait for the session's reply, then A5 to A7.
   - His answers (his message stamped 20:42:20 UTC, abaac228-81c6-4e33-808e-40fc63e8499f): A4 done, the test site is up at https://shamble-preview-check.pages.dev/;
     his Cloudflare account holds nothing besides the domain shamblefrontlines.com ("this is the first time that I've
     been using cloudflare"); on 1b: "I had to give proper access for read and write (confirm), and have done so for
     Claude", which names no permission, so SR-READ is still open. The app's public page
     (https://github.com/apps/claude, read by the web-fetch agent) lists no permissions. Asked him for the exact
     "Read and write access to ..." lines on the app's Configure page; he waits at A5 (step 3a, the token) until then.
   - His next message (stamped 20:53:10 UTC, 0e69957a-4b89-452f-91ea-630a03b3a35a): "setup done" and a screenshot of
     the Claude app's Configure page. Transcribed from the screenshot: "Read access to Dependabot alerts,
     administration, commit statuses, merge queues, and metadata"; "Read and write access to actions, checks, code,
     discussions, issues, pull requests, repository creation, repository hooks, and workflows"; Repository access:
     "All repositories". **SR-READ: neither administration nor deployments has write access** (administration
     read-only; deployments absent), so his thing 1 does not stop the build. Approving a pending deployment needs
     Deployments write and changing an environment needs Administration write (security-auditor's G1b and G3 notes),
     neither of which the app holds. His "setup done" is read as A2 and A5 to A7 done (to confirm in the reply).
     The screenshot itself stays out of the repository (a binary); its text is above.

## Next steps
1. Round 2: DONE, both APPROVE (step 17). Its three ride-along Lows go into the next push.
2. His yes on the batched diff (`tr-draft/proposed/deploy.yml` sha256 29593af18d34..., `tr-draft/proposed/close.yml`
   sha256 3afdf389d636...; the exact diff sent in chat at about 21:47 UTC). On his yes: copy both files byte for byte
   into /home/user/shamble-preview-check/.github/workflows/, check the hashes, commit on `setup`, push.
3. His answers: SA-R3 (accept the install-step residual risk for this check, or not), the screenshots of `preview`,
   `production` and Actions → General, and the token's end date (asked at 20:56 UTC).
4. The advisor agent's review of the final `setup` head (after 1 to 3).
5. The results-log header (PM-0006-1), then the first list for him (his condition 3): the setup pull request's two
   runs (its deploy, then its merge's cleanup), in order, with the sitting's length; then open the setup pull request.
6. Then Day 1's sittings as plan Part 3 C, with sitting 3's two additions (SA-R4, SA-R8), each preceded by its list.
   Before Day 1: ask him to compact if the chat is near full (it was 82% at 21:44 UTC).

## Open findings
- Refused by the guard, 2026-11-15 15:54:02 UTC: `git -C <dir> fetch ...` ("git: no option before the subcommand (run
  git from the folder, without -C, -c or --git-dir)"). Route taken: `cd` into each scratch folder under
  `.gate/preview-hosting-check/tags/` and ran git there (a shallow fetch of two tags to read their dates; nothing run).

- Process slip (his words of 2026-11-15: consult the advisor tool "always before you save a step as done and never
  after"): steps 5-7 were saved as done in the save of 16:16:22 UTC (0290625) and step 8 in the save of 16:21:42 UTC
  (d8451dc), with no consultation before them; consultation 2 followed at 16:22:00 UTC. From here on, each step is
  saved as done only after its consultation. To be told to him once, in the closing message.

- Refused by the guard while the advisor agent wrote its notes (its hand-back): a relative path, then a variable target;
  its route: a literal absolute path.
- Ultracode: switched on at 16:39:16 UTC (the "ultra_effort_enter" notice in the transcript, delivered with the advisor
  agent's hand-back). Revision 2 was made without a workflow: his task says "The plan gets no review rounds and no
  verifier", the fixes were the advisor agent's exact lines, and CLAUDE.md caps agents at 3. The build will use
  workflows where they fit (e.g. U4's review round of three).

## Facts for the plan (read by the session)
- Action pins: actions/checkout v7.0.1 3d3c42e5aac5ba805825da76410c181273ba90b1 and actions/setup-node v7.0.0
  820762786026740c76f36085b0efc47a31fe5020 (as gate.yml pins them); actions/upload-artifact v7.0.2
  cf430e030ddbb5b0abf93d22962f4752f3646cd9 (commit 2026-10-07); actions/download-artifact v8.0.2
  9000827ccba6bdab643e8b6fd33ac0654aef8333 (commit 2026-10-07): read with `git ls-remote` and a shallow tag fetch,
  2026-11-15 15:54-15:56 UTC.
- The deny list (.claude/settings.json) refuses `git push * main`, `git push --delete`, `gh api` with -X, --method,
  -f, -F, --field, --raw-field or --input, `gh pr merge`, and `npx *`: a session here cannot push a test repository's
  main, delete a remote branch, approve a pending deployment through gh, or run wrangler locally.

## Key numbers
- (none yet)

## Decisions taken
- His task (the file above): the plan for the preview hosting check; PREVIEW-START answered "it starts now, on the
  infra draft as proposed"; main model Opus 5.5, Fable 5.2 on the advisor tool; agents Opus 5.5, max effort for those
  that review, verify or design game and engine code; never push to another chat's branch; never touch spikes/.
- His three notes (his words of 2026-11-15, 17:00 Amsterdam, queued 16:03:05 UTC; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-15-notes.md`, 914 bytes): every "today" in the task message means
  2026-11-14; the first builder chat gets his go on Tuesday 2026-11-17, so its change-set 0 and Part 15 do not exist
  yet, and his decision on two builder chats stands on the task message; a third chat builds Shamble VFX in its own
  repository (Mergincanli/ShambleVFX): not this lane, never touched. The task is unchanged.

- **His answers and his go** (his words written 2026-11-15, 18:35 Amsterdam, user message
  c7a8e93b-1b32-4886-8330-e14ada13e8a7 stamped 18:42:30 UTC; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-15-go.md`, 2589 bytes, sha256 a627d4bce60b...): TR-WHERE (a),
  TOKENS (a), REVIEWER (a), MACHINES (a), TOMBSTONE (a), ACCESS-CARD (a), TR-END (a); his go on the plan as written,
  the build once he has said "repository ready"; his yes to Part 7's six texts exactly as at 2a2f426 (sha256 prefixes
  333e5a592a05, 98b42b8e59cf, f24ad6e81daf, 6791076916ae, fd3dc4c6029b, 8b745631a278: checked by script against the
  files at 2a2f426 on 2026-11-15, all six match); the acts allowed in the test repository; merges, approvals and
  settings his; nothing in Shamble-Frontlines beyond the checkpoint and the report's pull request; his three things
  (SR-READ decides before the build; security-auditor rules on `deployments: write`; one list before every sitting).

## Advisor tool consultations (his words of 2026-11-11: when, the question, what changed)
1. 2026-11-15 15:49:20 UTC (called; answer 15:51:36 UTC; transcript times), moment 1 · Q: is the approach sound (a
   separate test repository; one Pages Edit token in `preview` only, a reviewer with self-review off; probes and
   browser rows on GitHub runners; no custom domain)? · Changed: the test repository's place, the runner stand-ins and
   the one-token choice become decisions with both branches costed (the `spikes/` tension put to the advisor agent by
   name); a mark-by-mark table (as drafted / as planned / departure and who decides); every click read from a cited
   page first (else "if your screen differs, stop and tell me"); the reviewer gate's clicks and sittings counted and the
   close-time vs close-job time both recorded; the 24 h reading not on cron alone; asks listed (push to named branches
   there, his merges and branch deletion, the token straight from Cloudflare into GitHub's secret box); the saver
   before agents; no action toward the test repository this turn.
2. 2026-11-15 16:22:00 UTC (called; transcript time), moment 5 for plan revision 1 (late: see Open findings) · Q: is
   revision 1 ready for the advisor agent? · Changed: the `setup` branch is pushed without a pull request and opened
   only after "setup done" and his yes, so no workflow starts before `preview` is protected; the setup pull request's
   deploy and its merge's cleanup become two named runs (a pre-flight of the merged-close path), added to the count;
   Part 4's two over-claims hedged (reading a secret back; a custom domain on his zone) and the summary's "nothing
   outside Pages"; the check's own risk tier stated for the advisor agent to rule; the agent's brief gives questions,
   not conclusions; the saver checked first.
3. 2026-11-15 16:45:35 UTC (called; transcript time), moment 5 for revision 2, before it was saved as done · Q: may
   revision 2 be marked done? · Changed: the rehearsal (the setup pull request's two runs) declared outside the marks,
   with the results-log header written before it (Part 3 B, Table 1.1 PM-0006-1), as the session's reading for him to
   overrule; "For Mustafa" item 3 widened from pushes to every act in the test repository (opening and closing pull
   requests, starting and re-running runs); revision 1's leftovers grepped (none: `npx` appears only in Part 7.0's
   removed lines); the advisor-model line and the fullness figure kept for the closing message.

4. 2026-11-15 20:54:47 UTC (called; transcript time), moments 1 and 3, before the build · Q: is the build's approach
   sound (branch `setup`, U1-U3, local checks, push without a pull request, the review round of three)? · Changed: commit
   and push `setup` after each unit, recording each SHA (the saver does not cover that clone); git run from inside the
   clone (no -C or -c); the scripts' interfaces taken from the six approved texts first (records flat in verify, one
   folder deeper in probe; no records must not crash verify); every script keeps "no reading" apart from "measured and
   failed" (retries after a deploy, transport errors and 5xx are no reading, an empty marker never matches); each bite
   proven locally against a small `_headers` server and the page opened in the preinstalled Chromium (proves the
   scripts, not Cloudflare); WebdriverIO's options read from the pinned package itself; "setup done" confirmed by
   screenshots of both environments and the Actions page, plus the token's end date, before the setup pull request;
   every change to an approved text batched into one diff for one new yes; get_session read after `setup` is pushed.

## Running work
- saver: background task bc4g68il4 · output /tmp/claude-0/-home-user-Shamble-Frontlines/a7e7f150-3195-52bd-a549-73b27426198b/tasks/bc4g68il4.output ·
  started 15:52 UTC · stops by 17:42 UTC (110 min) · last check: just started.
- agent infra-engineer (Opus by its file, effort high by its file): C1-C8 and the follow-up C9 · DONE.
- agent security-auditor (Opus by its file, effort high by its file): G1-G10 · DONE.
- saver last check 16:27 UTC: the process is alive (pid 1609); its log shows "saved and pushed" at 16:02 and 16:12
  UTC and nothing for the cycle due about 16:22 (the session's own save of 16:21:42 UTC came just before it); to be
  checked again at the next update.
- workflow wf_c4580fcf-ece (task wbgd5v14u, round 1) · script `workflow-runs/review-round-run1.js` · DONE, 3 of 3.
- workflow wf_1f145f55-1c2 (task wo369ci0q, round 2) · script `workflow-runs/review-round-2-run1.js` · DONE, 2 of 2,
  both APPROVE.
- agent advisor (Opus by its file, effort high by its file): the plan review of revision 1 at 27e9f50 · DONE
  (APPROVE with conditions).
- saver: stopped with TaskStop at about 16:48 UTC after the plan was saved as done (its log: "saved and pushed" at
  16:02, 16:12, 16:32 and 16:42 UTC). Start it again with the same command when agents run.
- Facts read by the session meanwhile: read_documentation `github.access` (15:52 UTC: a repository becomes attachable
  once the Claude GitHub App can reach it; add_repo attaches it); list_repos (15:52 UTC: five public repositories of
  his, all `can_push: true`; the app's repository-access mode is not shown).
