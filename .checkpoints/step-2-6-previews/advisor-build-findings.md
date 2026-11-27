# The advisor agent's verdicts on step 2.6's build findings (F1 to F4, and the order): its answer at 2026-11-25 20:27:50 UTC (this chat's record)

Its report, word for word as it came back (the advisor agent has no write tool; this file is its record).

**Advisor verdicts on step 2.6's build findings (F1 to F4, and the order of the rest)**

All four are handled correctly. F1 is approved, F2 goes to him, and F3 needs no word from him. F4's open question is now answered by GitHub's own docs. The order is approved, with one timing point taken from his own words (finding A below).

These are verdicts on how each finding is handled, not an approval of a head. I did not run the gate, and I edited nothing. My scratch copy is in `/tmp/claude-0/-home-user/a7e7f150-3195-52bd-a549-73b27426198b/scratchpad/adv-f1`.

**F1. `node --test hosting/` fails in job `tests`: APPROVE**
- **What I reproduced** (Node v22.23.4, scratch copy with the three approved texts placed):
  - the folder argument exits 1 with "Cannot find module '…/hosting'";
  - naming the file exits 0, with 117 tests and 117 passing;
  - a missing named file exits 1.
- **The one-line change, applied to the approved bytes:**
  - `diff` shows exactly one changed line (line 50);
  - 10,577 bytes (10,561 plus the 16 characters of `preview.test.mjs`);
  - sha256 `31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc`;
  - the G9 lint and all 117 tests pass with the new text in place.
- **Nothing is lost by naming the file:** the lint and the header-file test are suites inside `preview.test.mjs` (12 suites, one file).
- Rejecting the `"main"` field is right: it is outside his "for the upload tool only", and it would change the manifest M3 reads.
- **Conditions:**
  1. Compute the sha256 yourself from the exact bytes you will place. It must equal the hash above.
  2. Bring him the full text, that hash and the one-line diff together.
  3. Place no changed bytes under `.github/workflows/` before his yes.
  4. The draft PR's `tests` job on CI is the CI proof; record it.

**F2. The comment cannot report a missing build: ESCALATE, as you propose (recommend keep)**
- His go covered both the plan's section 2.7 and the texts, and they disagree. So it is his call, as a stated departure.
- **Wording fix (Low):** the checkpoint says `go` "stays unset". The code writes `go=false`, then exits 1 (`hosting/preview.mjs` :241-244). Say "`go` is not `true`, so `after` does not run".
- **The decision must say exactly what the comment covers:**
  - it can report: the upload's result and the header check;
  - it cannot report anything that fails or stops `check`: a missing build once the hand-over step is on main, a refused build, a GitHub read failure;
  - those show only as a red run with an `::error::` line under main's Actions tab, and the runbook names this.
- **The options:**
  - (a) keep the texts;
  - (b) change `after`'s `if` and `comment` so failures in `check` also reach the comment. That is a new text with a new sha256, the security-auditor's re-read, and a write-token job that runs in more cases.
- I recommend (a).

**F3. The pull-request number is matched in `comment`, not `read-url`: within the plan's intent, no word from him needed**
- Plan 2.8 line 212 says `read-url` "requires the link to begin with `https://pr-<number>.` on that subdomain". The code does exactly that (:336), against the project address `verify-token` saved (:308).
- `comment` adds the match with this pull request's number (:485). The test at :534-535 checks that a `pr-13` link is refused for pull request 12.
- A prefix case such as pull request 1 against `pr-12` is safe because the check includes the trailing dot. Adding a test for it is a Low.
- **Note (Low):** between `read-url` and `comment`, two things use a link not yet matched to the pull request: the `preview` deployment's environment URL and the no-secret header fetch. This is bounded, because Wrangler is handed `--branch pr-$PR`.
- **Condition:** name the split in the report so round 1's security-auditor reads it. Do not change the text for it now.

**F4. R-COMMENT-PERM: now settled by GitHub's docs, no text change**
- The text tool lost the per-endpoint box; the rendered page has it. https://docs.github.com/en/rest/issues/comments, fetched 2026-11-25, says for both "Create an issue comment" and "Update an issue comment":
  - token types include "GitHub App installation access tokens";
  - "The fine-grained token must have at least one of the following permission sets: "Issues" repository permissions (write) "Pull requests" repository permissions (write)".
  - "List issue comments" accepts Issues (read) or Pull requests (read).
- So `pull-requests: write` alone is enough on the docs, and `issues: write` stays out.
- **Conditions:**
  1. Add the quote, its address and the date to `build-notes.md` B3, replacing "NOT settled".
  2. In the report, keep only "the first run confirms it (M5/M9)".
  3. Give him one line on the fallback if it is ever refuted: a tier-2 fix adding `issues: write` to `after`, with a new sha256 and his yes.

**The order: APPROVE, with conditions**
The order is: U1 part 2 held until M3, U3 meanwhile, U2 after his yes, then U5 and his sitting with M1.
1. **No push before M3 is recorded.** U3 (8a8531a) now sits on top of the held 1f4fd0d. The remote is still at e5e385c, so nothing has gone out.
   - The background saver (running) holds itself: its code at :10 and :17-23 skips the push when there are unpushed commits outside `.checkpoints/` or an unrecorded dependency change.
   - So the only risk is a manual `git push`: make none until M3's answer is recorded.
2. **Finding A (Medium): the security-auditor's read of `preview.yml` must come before his sitting.**
   - His item 6 (DECISIONS.md, line 153 on the branch) says the security-auditor reads `preview.yml` "in round 1", and "If a review changes a text, bring me the new text and its sha256 before my sitting."
   - Plan U7 puts round 1 after U5, his sitting. Then a change from round 1 could not reach him before the sitting.
   - **Condition:** before the one message goes, have the security-auditor read `preview.yml` as it will stand (the F1 line included), together with the token check's added duty in code and F3's split. Then he is brought one final text, not two. The full round 1 on the final diff still runs at U7; any later change goes back to him with a new sha256 (plan U7).
3. **U2:** place `preview-close.yml` and `wall-test.yml` byte for byte (c2f1fa90…, 3af06706…) and `preview.yml` at the new hash. Check all three hashes after placing them.

**Other findings**
- **Low, codes:** F3 and F4 clash with engine-architect's earlier F3 and F4 in the same notes. In his message, use distinct codes and explain each where it appears.
- **Low, red until U2:** in the real tree, before the texts are placed, `node --test hosting/preview.test.mjs` fails with ENOENT on `.github/workflows/preview.yml`. That is loud, not a silent skip. Until U2, the branch's own test file is red if anyone runs it; say so in the report.

**Decisions needed**
- **PREVIEW-MISSING-BUILD-COMMENT:** should the preview comment also report problems that stop the build check (a missing build once CI hands one over, a refused build)?
  - Why it matters: plan section 2.7 promised it, but the approved workflow texts cannot do it.
  - Option (a): keep the texts. Those problems show only as a red run under main's Actions tab, and the runbook says so.
  - Option (b): change the texts so the comment covers them. That means a new sha256, a security re-read, and a job with write access running more often.
  - My recommendation: (a).

**For Mustafa**
1. Answer PREVIEW-MISSING-BUILD-COMMENT.
2. Say yes or no to the new `preview.yml` (one line changed, sha256 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc, the full text brought to you), after the security-auditor's read is in.

## What this chat did with it (2026-11-25, 20:28 to 20:35 UTC)
- F1: the sha256 computed from the exact bytes (texts/preview-proposed.yml): 31119c06..., equal. Nothing placed.
- F2: the checkpoint's wording corrected ("go is not true, so after does not run"); the decision carries the coverage
  lines; the runbook names the red run.
- F3: the prefix test added (pull request 1 against pr-12's link); its first version passed for the wrong reason (the
  fake GitHub refused pull request 1), found by the mutation sweep and fixed: 29 of 29 mutations caught.
- F4: build-notes B3 replaced by the quotes, re-read by this chat with curl on the rendered pages.
- Finding A: the security-auditor's read of the proposed preview.yml, the token check's added duty and F3's split comes
  before the message to him.
- Codes: the message uses NEW-TEXT and PREVIEW-MISSING-BUILD-COMMENT, each explained where it appears.
