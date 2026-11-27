# infra-engineer: step 2.6 round-2 review (what changed only), PR #72, diff 8fef0f6..6fd0375d

Started 2026-11-25. Scope: docs/runbooks/previews.md, hosting/preview.mjs (3 changes), hosting/preview.test.mjs.

## Checks (one line each)
- preview.yml sha256 31119c06e7d0... unchanged (measured); no .github/ diff in the range.
- Tests: `node --test hosting/preview.test.mjs` 148 pass, 0 fail (run here).
- Runbook messages vs script: "does not hand one over yet" :247, "could not be fetched" :244, "the token is not active"
  :291, "the project ... could not be read" :301, "production branch is" :307, "nothing to deploy" :223,
  "could not all be read ... nothing deleted" :582: all match.
- "checked build kept one day" = preview.yml:110 retention-days: 1. Matches.
- 30-day re-run limit, re-run keeps original SHA and actor, 60-day schedule auto-disable, Disable workflow path: match
  the GitHub pages read 2026-11-25 (round-1 notes).
- Rebuild step 1 eligibility (7 days, Super Administrator, five extra Free accounts) = design notes :736-738 (Cloudflare
  docs read 2026-11-25); step 2 path = build-notes B9 :166-172. Match.
- Privacy paragraph: the file test exists (preview.test.mjs:941-977, PHC-PREVIEW-NOINDEX). Matches.
- Regex: backreference refuses mismatched quotes; tests plant quoted, comment, padded (pass) and -checked, mismatched,
  -sites (refused). `preview-site#x` (no space) would pass where YAML reads it as one scalar: negligible.
- Dedup: filter keeps the first of each string id; items with no string id kept (unchanged behaviour); tested with a
  record on two pages, deleted once.
- NEW (Low): preview.mjs:291 and :295 tell the reader "the runbook: clicks 6 and 8"; the runbook has no clicks, it
  has "The token", "To renew" (clicks 6/8 live in plan.md, deleted at ready). Pre-existing text, made visible by the
  rewrite. Fix: "the runbook: The token, To renew" (a one-line text change plus any pinned test), or a follow-up.
- NEW (Low): Rebuild step 4 sends to "To renew, steps 2 to 4"; step 5's "write the end date in STATUS" is skipped there
  (the token bullet says it, so a careful reader still gets it); step 3 says "Replace" where a rebuild "adds".
- STATUS has no token end date yet (no live token: the token sitting is ahead). M3 closed in the runbook; the write
  itself belongs to the token sitting / docs pass.

## Verdict: APPROVE. M1, M2, M3 closed; L1, L2, L3, L5, L6 (regex), L7, the noindex note closed; L4 and L6's #59 note
deferred, agreed. Two new Lows (above), neither blocks.

## Extra checks (after the advisor tool's review)
- Count: reconcile names `preview` with `deployment: false` (preview.yml:237-239), so only deploy jobs make records
  (preview.yml:119-121); monthCount (preview.mjs:456-474) counts all records of `preview`. "Deploy attempts" holds;
  that deployment:false makes no record is from the docs, not measured (M9 compares).
- Read 2026-11-25, docs.github.com re-run page: "Re-runs use the privileges of the actor who initially triggered the
  workflow, not the privileges of the actor who initiated the re-run." Runbook :28 matches.
- Read 2026-11-25, docs.github.com permissions-required-for-fine-grained-PATs: POST issues/{n}/comments and PATCH
  issues/comments/{id} listed under both Issues write and Pull requests write. The `after` job holds pull-requests:
  write (preview.yml:187-189). Runbook :61-62 matches (the page covers fine-grained tokens; GITHUB_TOKEN by analogy:
  fairly sure).
- Conditions: the 14-day warning fires when expires_on is under 14 days away (preview.mjs:293-296); the
  production-branch refusal fires on a missing branch or a pr-<n> name (:306-308). Runbook matches.
- Working tree vs reviewed head: `git diff 6fd0375d HEAD -- hosting/ docs/runbooks/ .github/` empty: tests ran on the
  reviewed code.
