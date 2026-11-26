# The token sitting: the one message to Mustafa (step 2.6), ready 2026-11-26; sent when he says the PR Advisor's merge
# under MAIN-WALL is through and main has been taken in (his words of 19:20 Amsterdam, item 3)

Sources: the security-auditor's conditions (a) to (e) (round 1, `round1-security-auditor.md`; build-notes B13), the
plan's clicks 6 to 8 (`plan.md` section 4), the runbook's "The environment page" and "The token"
(`docs/runbooks/previews.md`), and his start block, item 6d. Before sending: re-read the five approved sha256 and the
runbook's sections, and replace each {…} below.

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

**The token (Cloudflare, the previews account `Shamble previews`).**
1. Manage Account, Account API Tokens, Create Token, Custom token, named `shamble-previews-deploy`. It must be an account
   token, not one from your own profile.
2. Permissions: Account, Cloudflare Pages, Edit; and a second row: Account, Cloudflare Pages, Read.
3. Account: the summary must show only the previews account.
4. Client IP filtering: empty.
5. Expiration: a start date of today AND an end date 90 days on ({end date}), each in its own field. Never the
   single-date chip: it gave the hosting check a token that worked for one day.
6. Create, then copy the token.
7. Copy the Account ID from the previews account's overview page.

**(b) GitHub, the two secrets, only here.** Environments, `preview`, "Add environment secret", twice:
- name `CLOUDFLARE_API_TOKEN`, value: the token;
- name `CLOUDFLARE_ACCOUNT_ID`, value: the Account ID.
Never as a repository, Dependabot or Codespaces secret, and never as a variable: those reach every branch.

**(c) GitHub, nowhere else.** Settings, Secrets and variables: Actions, then Dependabot, then Codespaces. Send me one
screenshot of each page, showing no repository secret and no variable whose name starts with `CLOUDFLARE_`. If the
Actions page also lists environment secrets, the two under `preview` are the only `CLOUDFLARE_` names expected there.

Then tell me "secrets in", with the token's end date (a date only).

**(d) Me: the wall again, with the real key stored.** Within about 10 minutes I:
- push the commit that writes the token's end date into STATUS, with a reminder 14 days before it: a fresh run of the
  three planted jobs (the branch push, the pull request plainly, the pull request with no deployment record);
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
