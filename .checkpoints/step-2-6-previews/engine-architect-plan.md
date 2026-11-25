# engine-architect: departure read of step 2.6's plan (revision 1) against ADR 0002 part 10

Date: 2026-11-25. Read only; nothing committed. Reader: engine-architect (ADR 0002's owner).
Read: plan.md sections 2.1, 2.5, 3, 5, 6 (and 7, 10, 12 for cross-checks); infra-engineer-design.md D1, D5, D6 and the
"Revision after the advisor call" section (:718-793, overrides); ADR 0002 :3, :61-101, :640-712, :887-889;
r5-toolchain-quality.md R5-PREV-F1 to F3 (:765-782); toolchain.yml; packages/client/vite.config.mjs,
scripts/build.mjs, scripts/build-checks.mjs; Part 15 (docs/reports/2026-11-17-build-plan-slice-1-part-15.md) :23,
:66-103, :160-180, :304; DECISIONS :455-461, :507, :1365, :1887-1900; GLOSSARY :652-658.
Not read: his order of 2026-11-25 18:15 (not in DECISIONS yet; plan U3, :293-296).

## Context
- ADR 0002 is Proposed; only part 1 is accepted (ADR :3). Departures are from a proposal, to its next revision.

## Q1. Fair representation; does workflow_run + main-only environment answer finding 10?
- Fair in substance: section 6 item 2 (:222-226) quotes finding 10 (:82-83); 2.1 (:58-62) says plainly that the
  wall for preview.yml's own text is the kit, not GitHub. Missing: the options-table row P3 (ADR :889, "rejected for
  now", reason R5-PREV-F3), and that R5-PREV-F3's own quotes (r5 :775-782) state the conditions for safe use
  ("treat artifacts uploaded from other workflows with caution"; "must not explicitly check out untrusted code"),
  which this design meets. The draft rejected P3 on capability, not on a shown path.
- Three surfaces, against P1:
  1. Workflow text that reaches the token: now only main's (GitHub-enforced by the main-only rule, pending M1);
     main's review itself is kit-enforced (plan :58-62). Better than P1 without the reviewer.
  2. The artifact as untrusted data: NOT new; P1's deploy job also deployed a PR-built dist. Strengthened here
     (refuse list in a no-secret job; token job runs nothing of it).
  3. PR-controlled event strings (branch name, title, etc.) inside a main-context run: NEW against P1. D1 :390-391
     has the rule (env: only; PR number ^[0-9]+$, SHA ^[0-9a-f]{40}$); plan 2.1 and M2 do not. -> Medium F2.
- Verdict on Q1: it closes the gap the draft names without the click; the last wall moves to main's review. I accept
  the departure for static previews.
- Not a precedent: "runs nothing from the artifact" holds for static files only. ADR 0002 :686-689 (multiplayer
  delivery job: PR-built native code to the staging server, "the same required reviewer as `preview`") and :643-646
  (Sentry debug-file job "under that job's guard ... `preview`'s required reviewer") both lean on the reviewer this
  plan drops; section 6 does not list them. -> Low F5.

## Q2. hosting/_headers replacing the build's
- Today nothing breaks: the build writes no `_headers` (no packages/client/public/; buildPage writes none,
  build.mjs:185-222); isolation headers exist only on Vite's dev and preview servers (vite.config.mjs:7-12, :36, :39).
- Dependency direction improves: the client build stays host-agnostic; the host-specific file lives with hosting.
- Why the draft put it in the build: the header set depends on build properties: the isolation flag (ADR :683-684),
  CSP later (GLOSSARY :653-655), and hashed names under /assets/* (D5 :555; that every file there is hashed is D5's
  reading, not checked by me).
- Defect: a SILENT replace (plan :54, :110) would discard a generator lane 1 builds by following part 10 as written,
  and nothing tells lane 1. Fix: refuse (fail check, visibly) when the artifact carries `_headers`, and one #59 line
  that part 10's generator is parked until the revision. Plan 2.5 also lacks D5's "non-isolated builds only" line
  (:556-557). -> Medium F3, Low F6.

## Q3. The hand-over step in toolchain.yml
- Ownership settled by his words: DECISIONS :461 ("The CI edit that hands the tested build to the preview job is the
  CI Builder chat's") and :507 ("Lane 1's CI edits now go to a separate chat, CI Builder"); Part 15 :174 keeps the
  file lane 1's. The plan writes a request, not an edit. No ownership problem.
- Boundary: the artifact is ADR 0002's interface I-13 (ADR :101); its contract (workflow name, path, artifact name,
  event, same-repo, layout, retention) belongs in the revision.
- Fail-soft forever: a missing artifact ends in a green notice (plan :216-217); toolchain.yml:3 says step 2.3
  replaces the file. After both merges a rename would stop previews silently. Fix: fail-soft only until the
  producer step is on main; then a missing `preview-site` on a successful same-repo pull_request run turns `check`
  red. -> Medium F4.
- "Tested build" wording: the browser tests run on .gate/dist-development (toolchain.yml:194-195); dist-production
  passed the build checks (build.mjs:215-221) and the earlier tests, not the browser tests. Say so (section 5, 9).
  The plan dropped D6's producer-side comment (D6 :585-589) naming preview.yml. -> Low F6.

## Q4. Crossings of lane 1's folders and files
- hosting/package.json and hosting/package-lock.json authored by lane 2 (plan :133-134, :249-250, :289) against:
  Part 15 condition 2 (:72-74: "Build files and dependencies stay with lane 1: every `Cargo.toml` and
  `package.json`, both lockfiles ... When lane 2 needs a dependency it asks (15.3); lane 1 adds it"), accepted by him
  (Part 15 :23, :304: "The four conditions of 15.1 hold"); and his words to lane 2 of 2026-11-18 (DECISIONS :1365):
  "You own no manifest and no lockfile, so it should never hold you; if it does, stop and tell me." The plan plans
  to trigger that hold as a routine step. His 2026-11-16 item 3 (DECISIONS :1900) asks for a locked install planned
  and brought with step 2.6's plan; it names no author. Departure 6 (:234, :140-142) frames this as Part 15 :103's
  "a new file" for the advisor agent to judge; section 12 has no entry. Routing, not concealment: the folder is shown,
  the condition is never named. -> High F1.
  Options for him: (a) lane 1 (via CI Builder) adds the two files at lane 2's #59 request (condition 2 as written;
  one more PR on the path); (b) his word that lane 2 owns a host-tool lockfile outside the npm workspace (root
  package.json:9-11 lists only packages/*, so the game's npm ci never installs it; tools/ has its own lockfile as
  precedent). Architecturally either keeps Wrangler's tree out of the game's workspace; the author is his call.
- Plan :144 "Not touched" omits packages/, which Part 15 :103 lists. -> Low F7.
- Read-only dependencies on lane 1's build layout (assets/, Worker file, .wasm, 25 MiB rule duplicated) are fine as
  declared reads, not crossings; they are the I-13 contract.

## Findings, ranked (CLAUDE.md scale)
- F1 High: lockfile/manifest authorship (Q4) not named as a departure from condition 2 and :1365, not put to him.
- F2 Medium: event-string rule and a file test with a bite missing from plan 2.1 and M2 (Q1).
- F3 Medium: silent `_headers` replace; refuse visibly and tell lane 1 on #59 (Q2).
- F4 Medium: permanent fail-soft on a missing artifact (Q3).
- F5 Low: section 6 misses ADR 0002 :643-646 and :686-689; add P3 row :889 and F3's conditions to departure 2.
- F6 Low: copy D1/D5/D6 lines into the plan: full refuse list (D1 :394-397: `_redirects`, dot segments, 20,000
  files; plan :52-53 has fewer), D5 non-isolated line (:556-557), D6 step comment (:585-589), "tested build" wording.
- F7 Low: plan :144 add packages/.

## Additions after the final advisor call (2026-11-25)
- Q1, the draft's own escape clause: ADR 0002 :672-675 keeps the reviewer "unless OQ-0006-13 puts production where
  the preview token cannot reach it". The plan's ACCOUNT option (b), a previews-only Cloudflare account (plan
  :323-328; design :762-764), meets that clause: with (b), dropping the reviewer is inside the draft's own text and
  workflow_run is a second wall; with (a), workflow_run and main's review carry the whole weight. Section 6 items 1
  and 2 never cite the clause (added to F5). My acceptance of departure 2 is tied to his ACCOUNT answer: firm with
  (b); with (a), acceptable only while no production exists (plan :55-57).
- Departure 7 (job split, plan :235): accepted. The token job holds `contents: read` only (design :398) against the
  draft's one job with four permissions (ADR :663-664); the comment replaces the environment URL (design :402-406).
  Least privilege, an improvement.
- F3 and F4 reworded: a failed `check` in a workflow_run run may not show on the pull request (design :404-406
  infers the run carries main's commit). So each fix is "fails, and the plan says where that is seen" (the pull
  request's comment written by a no-secret step on failure, or the runbook naming the Actions page).
- F1 dates: his locked-install words (DECISIONS :1900) are of 2026-11-16, to lane 2; Part 15's conditions (accepted
  2026-11-17, Part 15 :23) and his sentence at :1365 (2026-11-18) came after. The plan does go back to him; the
  defect is that the question is unnamed and handed to the advisor agent (departure 6). It closes once section 12
  carries it as a decision.
- Not read by me: #66's report (the measured refusal at :113-114 is the plan's cite), ADR 0006 part 13, his order of
  2026-11-25 18:15.
