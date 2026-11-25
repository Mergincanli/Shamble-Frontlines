# Verifier notes: ADR 0006 header-row revision (diff 09faefa..1c45ed7)

Started 2026-11-25. Change type: decision-doc revision with research claims. Plan: plan.md revision 2.

## Plan
1. E7 and E3 claims: fetch each Cloudflare page myself; verdict PASS / FAIL / UNVERIFIABLE with the page's words.
2. Numbers and dates in E1, E2, E4, E6, E9, E10 against the merged report's Appendix A
   (round-a :664-678, bite/restore-production :803-826, dashboard :706-757, docs reading :1384-1388).
3. E7's outcome sentence against Appendix A under E1's new words (this row only).
4. DECISIONS: numstat 16/0; quoted block bytes and sha256; equality with transcript message 30f4ab9b-...
5. Gate: node tools/gate.mjs --no-smoke (smoke cannot start locally: Playwright wants chromium_headless_shell-1194,
   container has 1243; CI runs smoke).
Also: symbol diff (changed sentences) vs plan section 3; diff touches only the planned files.

## Results

### Item 5. Gate (done)
`node tools/gate.mjs --no-smoke`: GATE PASS, exit 0. [1] syntax 5/5; [2] concat PASS; [3] symbols +0 -0 (core 138 /
game 714 / editor 353), duplicates 0; [4] player build 51 refs guarded, 0 flagged; [5a] SKIP (--no-smoke); [5b] names 0
problems; [5c] layering 0; [5d] script order PASS. Base 09faefa (origin/main). Smoke not run locally: tools' Playwright
wants chromium_headless_shell-1194, container has 1243 (caller's statement; not re-tested). CI runs smoke on the head.

### Item 2. Numbers and dates vs Appendix A (done)
Report text, and cross-checked against the raw artifacts in .gate/preview-hosting-check/day1/*/headers.json
(script: verifier-scratch/headers_summary.py):
- round-a (:664-678; headers.json checked_at 2026-11-18T17:02:41.858Z, summary 3 rec / 5 pass / 1 fail): production
  one-off https://0354374e...pages.dev/ fail on 7 of 7 paths, only msg "production carries X-Robots-Tag noindex",
  x-robots-tag "noindex" on all 7; production own address pass, x-robots none. PASS.
- bite-production (:803-811; checked_at 18:11:38.137Z, 1 rec / 0 pass / 2 fail): own address and e9a37459 both fail 7/7,
  only the noindex msg. "the bite caught on production's own address" (:811). PASS (E2).
- restore-production (:812-820; checked_at 18:13:45.151Z, 1 rec / 1 pass / 1 fail): own address pass (no noindex);
  de1c3e5e one-off fail 7/7, only the noindex msg. PASS (E1's "0354374e and de1c3e5e ... all 7 paths").
- Dashboard readings (:706-757): first reading 17:19:10 / 17:21:40 / 17:23:36 UTC 2026-11-18, "No count of builds is
  shown" on Deployments, Metrics, overview; second reading 17:33:15 UTC 2026-11-18, "No count of builds is shown". Ten
  uploads 17:28:10-17:28:52 between them. Both on 2026-11-18: PASS (E3, E4, E10). "ten uploads": 10 listed. PASS.
- Docs reading (:1384-1388) under "After the verifier (2026-11-24, before the report)", agent "after 23:09 UTC": docs
  do not say. Date 2026-11-24: PASS (E3, E4).
- "written by no rule of the production build" (E1): report :668 "production's build writes no noindex, so the host
  added it"; own address of the same deployment carries none (artifact). PASS.

### Item 3. E7's outcome sentence, this row only (done)
Under E1's new words, every reading on Pages (artifacts):
- every header on both URL forms: .wasm application/wasm in all 34 readings of 23 links (round-a 6, uploads 20 readings
  of 11 links, bites 4, bite-production 2, restore 2); every status 200, 7 paths per reading. [corrected after the
  advisor: an earlier head -30 cut off the uploads' branch-link rows] Production one-off links fail on nothing but the noindex msg. PASS.
- production's own address no noindex: round-a and restore-production pass; x-robots none. PASS.
- previews noindex: round-a 4 links / 4 readings, uploads 11 links / 20 readings (= the report's 15 preview links, 24
  readings, :820-821), bites 4 / 4: x-robots "noindex" on every path. PASS.
- COOP/COEP: iso-on both links same-origin / require-corp on all 7 paths; iso-off and production none. PASS.
- both bites caught: bite-coep only "/worker.js COEP "" expected require-corp" on both links; bite-production noindex on
  production's own address. PASS.
- One-off X-Robots-Tag on production now "recorded, not judged": recorded noindex on 0354374e and de1c3e5e. PASS.
Verdict: "the revised header row is met by the readings already taken on Pages" holds for PM-0006-2 alone. The whole
check is not judged here (Safari's PM-0006-3 row stays open, report :789-795).

### Item 4. DECISIONS (done)
- `git diff --numstat 09faefa..1c45ed7 -- docs/DECISIONS.md`: 16 0. No removed lines. Placed first under "Decisions,
  newest first", above the 00:22:09 UTC entry.
- Transcript (python, verifier-scratch/dec_check.py): uuid 30f4ab9b-... found once, type user, timestamp
  2026-11-25T11:07:07.843Z; content is one text string: 741 bytes, sha256 ae5eb4f6...176eb.
- DECISIONS quoted block (5 lines, "> " stripped, joined by \n): 741 bytes, sha256 ae5eb4f6...176eb; string-equal to the
  message: True. PASS.
- Records in the entry: gh api pulls/66: merged_at 2026-11-25T11:10:11Z, merge_commit_sha 09faefa7f7cd...4737, head
  d7937de0aad4...b67c; git: 09faefa's second parent d7937de. Match. PASS.

### Item 1. E7 and E3 against Cloudflare's docs (done; fetched 2026-11-25 with curl, each page's index.md export,
into the scratchpad's cf/sN/page.md; the pages' front-matter "agent instructions" ignored as data)
- previews/custom-domains (dM 2026-11-08), "Protect Preview content" :113: "Cloudflare adds this header automatically
  to `workers.dev` Preview URLs, but not to custom domain Preview URLs." PASS. Also previews/ :47 "use its workers.dev
  URL, which includes an `X-Robots-Tag: noindex` response header".
- Silent on noindex for production's own version links: versions-and-deployments/version-urls/ (dM 2026-11-19): 0 hits
  for noindex/robots/search engine/indexing/crawler (outside the front matter); :46-48 "New versions are created when
  you run: wrangler deploy" (so production versions have Version URLs). Also 0 hits on versions-and-deployments/
  (dM 2026-11-19) and configuration/routing/workers-dev/ (dM 2026-11-06). PASS (silent).
- previews/configuration/ (dM 2026-09-22) :127: "`npx wrangler preview` uploads assets from the branch you are
  previewing." PASS ("its branch's files").
- previews/ (dM 2026-09-24) "Limits" :65-68: "| Previews per Worker | 100 | 500 |" (Free plan | Paid plans). PASS.
  "where Pages sets no cap": pages/platform/limits/ names retention only (30 days; 100 most recent kept, :46-53), no
  cap on preview count. PASS.
- delete: previews/ :75 "You can also delete Previews yourself with `npx wrangler preview delete --name
  <preview-name>`"; wrangler/commands/workers/ (dM 2026-10-26) :316-318 "## `preview delete`" / "Delete a Preview and
  all its deployments". PASS.
- configuration/cloudflare-access/ (dM 2026-11-02) :54 and :100 "Choose **Previews only** or **All traffic**";
  :37/:39 `all_preview_workers`, `preview_worker`. PASS.
- static-assets/headers/ (dM 2026-11-12) :58 "Using absolute URLs is supported"; :144 host placeholders; :170-175
  example `https://:version.:subdomain.workers.dev/*` X-Robots-Tag: noindex. PASS (host rules).
- platform/limits/ (dM 2026-11-19) :39 and :409 "Individual file size | 25 MiB | 25 MiB"; "month" 0 hits on the page;
  no deploy or upload cap row (sections listed: Versions :336-348 are retention only). platform/pricing/ (dM
  2026-11-18): no line on deploys, uploads, builds, previews or versions (one DO "versions of get()" hit, unrelated).
  PASS ("no monthly cap on deploys is named"). "as on Pages": pages/platform/limits/ :71 "The maximum file size for a
  single Cloudflare Pages site asset is 25 MiB." PASS.
- pages/ (dM 2026-08-25) :18-20 "Are you sure you want to use Pages? ... It is Cloudflare's primary platform for
  building applications. Start new projects with Workers." PASS.
- E3: pages/platform/limits/ (dM 2026-11-19) :26 "Each time you push new code to your Git repository, Pages will build
  and deploy your site." :31 "| Builds per month | 500 | 5,000 | 20,000 |"; no word on direct uploads. pages/get-started/
  direct-upload/ (dM 2026-04-21): no "count", "month" or "500"; "build" only for the user's own build command (:28, :74,
  :129). PASS (the docs do not say).

### Plan conformance ("symbols" = plan section 3's changed sentences; script verifier-scratch/plan_vs_head.py)
- Files: only docs/adr/0006 and docs/DECISIONS.md under docs/ (plus .checkpoints/ files, expected while draft).
- ADR diff: 5 lines removed, 24 added (hunks at base :138, :149, :1211, :1367, :1449 = E3, E7, E6, E1/E2/E4/E9, E10).
- E1, E2, E4, E6, E9, E10: planned After-text found verbatim (whitespace-normalised) in head. Match.
- E3: matches except one parenthesis: plan "(read 2026-11-24, and again for this revision) (PM-0006-7;" -> head
  "(read 2026-11-24 and 2026-11-25; PM-0006-7;". Same meaning; Low.
- E7: placed after the *Need:* line as planned; says none measured on Workers; production version links "silent";
  outcome under (a); no move decided. Deviations, Low: 12 lines (11 full + 1 short; checkpoint says 11) against the
  plan's "at most 8 lines"; opener form differs from the plan's quoted opener; Q1's alias Version URLs and production
  workers.dev URL not given separately (the plan asked "separately for preview and alias URLs and for workers.dev").
- E1's reason = option (a)'s words verbatim (report :284-285). E3's STEP4-COUNT clause fits his words (DECISIONS :193).
- Not applicable now: "no .checkpoints/ files when marked ready" (the PR is still a draft; they must go before ready).

### After the advisor (2026-11-25)
- advisor-plan.md read whole (47 lines, review of revision 1): no condition on E7's length or opener; Q4 only says the
  security-auditor is not needed unless E7 changes part 2's wall text or favours a move (it does neither). It records
  E9/E10 joining "as the advisor agent's reading" but not their exact words, so E9/E10 are checked against plan.md's
  copy only. No recorded approval of a 12-line E7, of its opener form, of the thinner Q1 split, or of E3's parenthesis.
- STEP4-COUNT date: DECISIONS :169 heads it "2026-11-24, 23:53:15 UTC"; his label "(25 November, 00:55 Amsterdam)".
  E3's "2026-11-25" follows his Amsterdam date; Low: the ADR does not say which clock.
- Gate tree: `git diff --stat 1c45ed7 HEAD -- . ':!.checkpoints'` and `git diff --stat HEAD -- . ':!.checkpoints'` are
  empty, so the gate's working tree equals 1c45ed7 outside .checkpoints/.
- Pages clauses in E7 with no cite in the paragraph: "as on Pages" (25 MiB) and "where Pages sets no cap": confirmed by
  me on pages/platform/limits/ :71 and :44-53 (retention only, no cap stated); not in infra-engineer's draft.
- "Silent": the nearest text is static-assets/headers/ :166-175, a `_headers` example the user adds
  (`https://:version.:subdomain.workers.dev/*` X-Robots-Tag: noindex); it says nothing of what the host sends.

### Verdict
FAIL, narrow (plan conformance only). Content: every sourced claim confirmed, every number and date traced, the outcome
sentence holds for PM-0006-2, DECISIONS byte-equal, gate PASS. But the final text departs from approved plan revision 2
with no recorded approval: E7 12 lines vs "at most 8 lines", a different opener, Q1's alias and production workers.dev
cases not given separately; E3's parenthesis reworded. Smallest fix: trim E7 to 8 lines with the planned opener and
restore E3's wording, or record a plan revision 3 covering E7 as built and E3's wording, approved by the advisor agent.

## Round 2 (2026-11-25; fix pass ab15d6d, head a4152ad; plan revision 3, advisor conditions in advisor-plan.md)
### Plan
1. Word for word: every changed sentence of `git diff 09faefa..ab15d6d -- docs/adr/0006...` against section 3 as
   revision 3 amends it: E7 = R3-1 block (15 lines), E3 R3-2, E1 R3-3, E1b R3-4, E4 R3-5, E11 R3-6, R3-10, R3-11; the
   unchanged E2, E6, E9, E10 still as revision 2. Script in verifier-scratch/.
2. Spot-checks: pages/platform/limits/ S23 sentence and the old "unlimited number of preview deployments" gone;
   workers/previews/ "deletes the oldest"; version-urls and workers-dev silent on noindex (address, workers.dev URL,
   version links, aliases); Pages 25 MiB.
3. Gate --no-smoke.
4. PM-0006-2 under E1's final words, against Appendix A (custom domain: the check had none).
### Results
