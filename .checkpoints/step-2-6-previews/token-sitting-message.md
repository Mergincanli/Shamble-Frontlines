# The token sitting: the one message to Mustafa (step 2.6), sent 2026-11-27 after the advisor agent's APPROVE of
# 19034b7 (his words of 2026-11-27, 12:50 Amsterdam, item 3: "send me the token sitting as your one message, with
# today's dates filled in")

Sources: the security-auditor's conditions (a) to (e) (round 1, `round1-security-auditor.md`; build-notes B13), the
plan's clicks 6 to 8 (`plan.md` section 4), the runbook's "The environment page" and "The token"
(`docs/runbooks/previews.md`), his start block, item 6d; the advisor agent's review of 19034b7 (advisor-head-19034b7.md:
M-1, the Account ID goes in before the token is made, so the token is pasted at once and never overwritten on the
clipboard; L-C, the environment page re-checked after the secrets; L-D, both tabs of the Actions page; L-E, #72 must
not conflict when the fresh run is pushed).

---

**The token sitting (step 2.6, #72): about 15 minutes, in this order. Stop at any step that does not look as written,
and tell me before going on.**

The token goes from Cloudflare's page straight into GitHub's secret box and nowhere else: never into this chat, a file
or a note. I never see it; the first upload's log shows only whether it is active and its two dates.

**(a) GitHub, the wall first.** Shamble-Frontlines, Settings, Environments, `preview`. Reload the page, then send me one
screenshot showing:
- "Allow administrators to bypass configured protection rules": unticked;
- Deployment branches and tags: "Selected branches and tags", "1 branch and 0 tags allowed", the rule `main`, and no
  other rule;
- Required reviewers and Wait timer: off.
If any of it differs, stop here: no token is made.

**(b) The two secrets, only on that environment page, each with "Add environment secret".**
1. Cloudflare, the previews account `Shamble previews`: copy the Account ID from the account's overview page. GitHub,
   Environments, `preview`, "Add environment secret": name `CLOUDFLARE_ACCOUNT_ID`, value the Account ID.
2. Cloudflare, the previews account: Manage Account, Account API Tokens, Create Token, Custom token, named
   `shamble-previews-deploy`. It must be an account token, not one from your own profile.
   - Permissions: Account, Cloudflare Pages, Edit; and a second row: Account, Cloudflare Pages, Read.
   - Account: the summary must show only the previews account.
   - Client IP filtering: empty.
   - Expiration: start date 2026-11-27 AND end date 2027-02-25 (90 days), each in its own field. Never the
     single-date chip: it gave the hosting check a token that worked for one day.
   - Create, copy the token, and at once: GitHub, Environments, `preview`, "Add environment secret": name
     `CLOUDFLARE_API_TOKEN`, value the token. Copy nothing else in between.
3. Never as a repository, Dependabot or Codespaces secret, and never as a variable: those reach every branch.
4. Reload the environment page: the bypass box still unticked and the rule still `main` only. If anything changed,
   click "Save protection rules", reload, and check both again.

**(c) GitHub, nowhere else.** Settings, Secrets and variables: Actions (both tabs, Secrets and Variables), then
Dependabot, then Codespaces. Send me one screenshot of each, showing no repository secret and no variable whose name
starts with `CLOUDFLARE_`. If the Actions page also lists environment secrets, the two under `preview` are the only
`CLOUDFLARE_` names expected there.

Then tell me "secrets in".

**(d) Me: the wall again, with the real key stored.** Within about 10 minutes I:
- check that #72 does not conflict, then push the commit that writes the token's end date (2027-02-25) into STATUS,
  with a reminder from 2027-02-11, 14 days before (a real change, not skipped by CI): a fresh run of the three planted
  jobs (the branch push, the pull request plainly, the pull request with no deployment record);
- re-run the last earlier run of each (GitHub keeps re-runs open for 30 days);
- read, for all six jobs, GitHub's refusal line through the annotations API ("... is not allowed to deploy to preview
  due to environment protection rules.") and zero steps run; each also leaves a deployment record with status
  failure, as expected.
If any planted job runs a step, stop: delete both CLOUDFLARE_ secrets and revoke the token in Cloudflare (Manage
Account, Account API Tokens, the token, Delete), and we find out why before anything else.

**(e) Both of us: the throwaway out.** When I report all six refused:
- you: Environments, `preview`, the secret `WALL_TEST`, delete it;
- me: `.github/workflows/wall-test.yml` leaves the branch in one commit. It is a change under `.github/`, so the guard
  asks you before my edit; say yes to that one prompt.

After the sitting nothing is needed per preview. The first live link comes after #72 merges (your go, after your
Advisor's merge check), on the next pull request whose `toolchain` run passes.
