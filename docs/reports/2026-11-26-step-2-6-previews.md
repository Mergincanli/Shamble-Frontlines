# Advisor report: step-2-6-previews (2026-11-26, branch claude/optimistic-albattani-mdoonz, PR #72, base main 07d673b0)
The branch stands in for `cs/step-2-6-previews` (a cloud chat limited to a `claude/…` branch). Code head 84a3ea44.
B-numbers point to the build notes, `.checkpoints/step-2-6-previews/build-notes.md` at commit 2fd80d69 (B1 to B16);
they leave the branch at "ready" and stay readable at that commit.

**Correction first.** On 2026-11-25 at 17:15:56 UTC this chat told Mustafa "Guard: yes. The hooks load the guard".
Wrong: since 2026-11-15 20:01:46 UTC the chat has two repositories, and "only a session with one repository reads" the
repository's settings (the platform's page; CLAUDE.md says the same), so the guard has not run here since (B16).

## Goal
Foundation step 2.6: every pull request from this repository gets a public, unlisted preview link of its tested build
on Cloudflare Pages, with one comment holding the link, and no click per preview.
## Outcome
PARTIAL. Built, reviewed in two rounds, green in CI. Not yet done: the token sitting, the throwaway `wall-test.yml` out,
the first live link (it needs the CI Builder's hand-over step, asked on #59). His answers of 2026-11-26 (12:21 UTC):
a new chat with only this repository takes over these steps; until then no .github/ or kit change, no main-folder
move, no merge.
## Plan and approval
Plan revision 2 (c40f3f9): a `workflow_run` workflow on main's files, an environment `preview` limited to `main`, a
previews-only Cloudflare account, guards G1 to G9. The advisor agent approved; his go 2026-11-25 19:36:53 UTC: TRIGGER
(a), ACCOUNT (b), LANE2-LOCKFILE (b), DEPARTURES (a), MAIN-WALL (b, the risk accepted until the token click), the three
texts by sha256. Conditions met: the security-auditor read preview.yml before his yes (no change); his NEW-TEXT (a)
changed line 50 (`node --test hosting/preview.test.mjs`; `hosting/` fails on Node 22): preview.yml is now 31119c06.
## What changed
- `.github/workflows/preview.yml` (31119c06), `preview-close.yml` (c2f1fa90), `wall-test.yml` (3af06706, throwaway): his
  approved texts, equal by sha256 (the security-auditor, round 2). Placed by a Python script (9ded5a87).
- `hosting/preview.mjs`: one script, eight commands, Node built-ins only. `hosting/preview.test.mjs`: 175 tests (G9's
  lint of preview.yml included). `hosting/_headers`; `hosting/package.json` + lockfile: Wrangler 4.164.0 (DEP-HOLD:
  the security-auditor's APPROVE on both sha256).
- `docs/runbooks/previews.md`; `docs/DECISIONS.md` (his six messages, by script); `docs/REPO_MAP.md` rows; STATUS and
  ROADMAP (lane 2's lines; change-set 0's merged entry, ROADMAP lines 371-377, moved byte for byte to Build history
  lines 202-208 by script, under his OLD-LINES (a)).
- No game file, no `.claude/`, no gate file. Gate check 3: +0 -0 symbols.
## Evidence
- Gate PASS (checks 1-4, 5b-5d; 5a smoke cannot start locally: the container's Chromium does not match tools'
  Playwright). On GitHub at 84a3ea44: gate, smoke, toolchain green; preview/tests "# tests 175", "# pass 175" (job
  139612150805). `wall-test` red by design (its planted jobs are refused).
- Mutation sweeps: preview.mjs 32/32 and the test-engineer's 22/25 (its 3 judged equivalent); the lint 18/18.
- M1 (the wall in front of the token): try 1 failed (likely cause: the bypass box was not saved); try 2 passed: all
  three planted jobs refused before any step (B12), the refusal line read through the annotations API (B13).
- npm 10.9.9 measured: `--no-ignore-scripts`, `--ignore-scripts=false`, `--ignore-scripts false` turn scripts on again.
## Reviews
- Round 1 (head 8fef0f6): infra REVISE (runbook: renewal order, re-run limits, end date in STATUS): fixed. Security
  APPROVE, with conditions for the token sitting (below). Test REVISE (TE-A to TE-J): fixed or moved (de6e4b58,
  6fd0375d).
- Round 2: infra APPROVE (two Lows fixed); test REVISE narrow (N1 a regression in the npm rule, N2 the token's places
  per job): fixed in 80a9b6a8; security APPROVE (two Mediums: YAML anchors, YAML-continued npm lines): fixed in
  84a3ea44. Follow-ups to STATUS: six unlinted guards (WRANGLER_LOG, metrics, job permissions, github-script, the
  download path, the upload's working folder), code beside the token (shell, NODE_OPTIONS/BASH_ENV, containers),
  check-headers one file at a time; `preview / tests` is not a required check and the gate does not read `hosting/`.
## Decisions and deviations
- F2: the comment cannot report a missing build (his PREVIEW-MISSING-BUILD-COMMENT (a): the texts stay). F3: the PR
  number is matched in `comment`, not `read-url`. The month's count is an upper bound (deploy attempts), not exact as
  plan 2.3 said. `wall-test.yml` stays until the token sitting (M1's re-runs need it).
- Untested, by his TAG-CHECK (a): a tag push against the environment rule. The reason: the rule admits the branch
  `main` only ("1 branch and 0 tags allowed"), so a tag is outside it, and a tag test would need a new workflow text.
- Process, from the record (B15, B16): 53 breaches of his interim git rule (a floor), and three main-folder merges
  outside the allowed forms (2026-11-18, 11-24, 11-25 21:00 UTC), each bringing only main's files among protected ones.
- The guard item: the guard absent from this chat since 2026-11-15, and whether an ask in auto mode reaches him open.
  The security-auditor rates it High for the kit (Critical if the hooks ran and an ask was settled without him),
  outside this diff and not a follow-up. The advisor agent: ESCALATE; his answers of 2026-11-26 settle it (below).
- Left, by the advisor agent's review (D1): lane 2's STATUS lines are long single lines, as other lanes' are.
## Risks
1. A guard that seems on but is not (this chat), and asks that may not reach him in auto mode (kit-wide): his
   GUARD-KIT (a) gives it to the Guard fix chat after its protection for main.
2. No token exists yet (his TOKEN-TONIGHT (b)): it waits for the Guard fix chat's protection for main, and the
   environment rule (M1) must still hold when it comes.
3. Unproven until live: M5 to M11 (the first link, the comment, the headers served, reconcile, the count).
## Proposed next step
The new chat with only this repository (his GUARD-OFF (a)), from Builder V2's start block: the guard check first;
then, once the Guard fix chat's protection for main is merged, the token sitting with the security-auditor's conditions
(a reload and screenshot of the environment page; both secrets by "Add environment secret"; no CLOUDFLARE_ secret
elsewhere; M1 re-run; WALL_TEST and `wall-test.yml` removed); then the verifier, ready, and his go.
## Decisions needed
None open. His answers of 2026-11-26 (`docs/DECISIONS.md`): GUARD-OFF (a), a new chat with only this repository takes
over the sensitive steps; MAIN-WALL-AGAIN (a), it stands until the token sitting; GUARD-KIT (a), the Guard fix chat
after its protection for main; GIT-IN-NODE (a), main's tracked kit scripts may run git, a chat's own scripts never run
git and never write a protected file; TAG-CHECK (a), untested, with the reason above.
## For Mustafa
1. Start a new Claude Code chat on your personal account (Builder V2's: one account per branch), with only
   Mergincanli/Shamble-Frontlines, on Opus 5.5, and paste the start block
   (`.checkpoints/step-2-6-previews/start-block-new-chat.md`); Builder V2 names the branch head when it hands it over.
2. Nothing to click until the token sitting, which waits for the Guard fix chat's protection for main.
