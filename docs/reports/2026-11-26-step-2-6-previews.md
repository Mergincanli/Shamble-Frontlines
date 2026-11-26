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
the first live link (it needs the CI Builder's hand-over step, asked on #59). Stopped until his GUARD-OFF answer.
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
  ROADMAP (lane 2's lines; change-set 0's merged ROADMAP entry moved byte for byte into Build history by script, under
  his OLD-LINES (a)).
- No game file, no `.claude/`, no gate file. Gate check 3: +0 -0 symbols.
## Evidence
- Gate PASS (checks 1-4, 5b-5d; 5a smoke cannot start locally: the container's Chromium does not match tools'
  Playwright). On GitHub at 84a3ea44: gate, smoke, toolchain green; preview/tests "# tests 175", "# pass 175" (job
  139612150805). `wall-test` red by design (its planted jobs are refused).
- Mutation sweeps: preview.mjs 32/32 and the test-engineer's 22/25 (its 3 judged equivalent); the lint 18/18.
- M1 (the wall in front of the token): try 1 failed (the bypass box was not saved); try 2 passed: all three planted
  jobs refused before any step, the refusal line read through the annotations API (B12).
- npm 10.9.9 measured: `--no-ignore-scripts`, `--ignore-scripts=false`, `--ignore-scripts false` turn scripts on again.
## Reviews
- Round 1 (head 8fef0f6): infra REVISE (runbook: renewal order, re-run limits, end date in STATUS): fixed. Security
  APPROVE, with conditions for the token sitting (below). Test REVISE (TE-A to TE-J): fixed in de6e4b58 and 6fd0375d.
- Round 2: infra APPROVE (two Lows fixed); test REVISE narrow (N1 a regression in the npm rule, N2 the token's places
  per job): fixed in 80a9b6a8; security APPROVE (two Mediums: YAML anchors, YAML-continued npm lines): fixed in
  84a3ea44. Follow-ups to STATUS: six unlinted guards (WRANGLER_LOG, metrics, job permissions, github-script, the
  download path, the upload's working folder), code beside the token (shell, NODE_OPTIONS/BASH_ENV, containers),
  check-headers one file at a time; `preview / tests` is not a required check and the gate does not read `hosting/`.
## Decisions and deviations
- F2: the comment cannot report a missing build (his PREVIEW-MISSING-BUILD-COMMENT (a): the texts stay). F3: the PR
  number is matched in `comment`, not `read-url`. The month's count is an upper bound (deploy attempts), not exact as
  plan 2.3 said. `wall-test.yml` stays until the token sitting (M1's re-runs need it).
- Process, from the record (B15, B16): 53 breaches of his interim git rule (a floor); three main-folder merges outside
  the allowed forms (2026-11-18, 11-24, 11-25 21:00 UTC), each bringing only main's files among protected ones; the
  guard absent since 2026-11-15; whether an ask in auto mode reaches him is open. The advisor agent: ESCALATE.
## Risks
1. A guard that seems on but is not (this chat), and asks that may not reach him in auto mode (kit-wide).
2. The token sits behind the environment rule alone until the Guard fix chat's protection for main is merged.
3. Unproven until live: M5 to M11 (the first link, the comment, the headers served, reconcile, the count).
## Proposed next step
After his GUARD-OFF answer: the token sitting with the security-auditor's conditions (a reload and screenshot of the
environment page; both secrets by "Add environment secret"; no CLOUDFLARE_ secret elsewhere; M1 re-run; WALL_TEST and
`wall-test.yml` removed), then the verifier, ready, and his go.
## Decisions needed
1. GUARD-OFF: how the remaining sensitive steps run. (a) a new chat with only this repository, after this report; its
   first act shows a guard prompt while he watches; (b) this chat, asking him before each step. Recommend (a).
2. MAIN-WALL-AGAIN: does his MAIN-WALL (b) stand, given the guard did not run? (a) until the token sitting, where his
   rule asks him again; (b) withdraw now. Recommend (a).
3. GUARD-KIT: a plan-first kit change-set for the Guard fix chat: show at start whether the guard is loaded, test
   whether an ask in auto mode reaches him, no second repository in a chat that writes this one. Recommend yes.
4. GIT-IN-NODE: does his git rule cover git inside scripts? (a) main's kit scripts may; a chat's own scripts never;
   (b) never; (c) always. Recommend (a).
5. TAG-CHECK: M1 did not measure a tag push. (a) record it as an unmeasured residual; (b) measure it with a tag job
   (a new text and his yes). Recommend (a): the rule admits branch `main` only, and "0 tags allowed".
## For Mustafa
1. Answer GUARD-OFF, MAIN-WALL-AGAIN, GUARD-KIT and GIT-IN-NODE (the questions file at commit 702f42c0 holds these
   four in full) and TAG-CHECK (in this report only).
2. Nothing to click now; the token sitting waits for the Guard fix chat's protection for main.
