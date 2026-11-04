# infra-engineer review: docs-after-kit-v3-1 (PR #35, head 603f53b on main 868e165)

Memory: no agent-memory folder found (checked .claude/ in repo and worktree).

## Plan
1. Read the diff for ADR 0006 (OQ-0006-23), adr/README.md register row.
2. Compare OQ-0006-23 shape with OQ-0006-21; check related list exists (OQ-0006-3, -12, -15, -21, OQ-0002-9).
3. Check OQ-0006-23 makes no claim about iframe storage/cookies/headers/hosting.
4. Scan ADR 0006 for hosting/headers/CSP/preview statements that contradict launching inside a third-party iframe (frame-ancestors, X-Frame-Options, COOP/COEP, storage).
5. STATUS and ROADMAP routine and CI statements: PR Advisor schedule-only trigger (+ by hand / API, never PR events); kit-v3-1 report line 169 (two smoke runs on 04da817).
6. Plan/approval conditions that touch my scope.

## Progress
(filled as I go)
- Read: diff for 0006, README, kit-v3-1 report; plan (checkpoint) and advisor-plan-rev1 (16 conditions; cond. 8 "web portal", cond. 9 related OQ-0006-3/-15, no storage/cookie claims).
- OQ-0006-23 (0006:1149-1164) shape matches OQ-0006-21 (0006:1123-1136): heading, verbatim quote (matches checkpoint:202-206), Why, Options none, Decider, What decides, Carries+Related. Related list has -12, -21, -3, -15, OQ-0002-9: all present.
- CANDIDATE F1: 0006:1156-1157 "Why it matters: his words above: web-portal launches would reach the same client host, game servers and player pool" -> "the same client host" is not in his words ("sharing our servers and player pool") and is an unresearched hosting claim. Same in README register row (OQ-0006-23 row).
- CANDIDATE F2: 0006:278 part 6 "The page and the lobby sit on one registrable domain, or the guest cookie is never sent (part 13, B4)" and 0006:534 B4 (SameSite=Strict HttpOnly cookie on lobby origin) assume a top-level page on our domain -> tension with launch inside third-party iframe. Related list names OQ-0006-3 (server store) but not B4/OQ-0006-6 (client-side guest secret). Flag only.
- No frame-ancestors / X-Frame-Options in 0006 (grep). Production header set: no COOP/COEP (0006:132-133, PM-0006-2 at :1337) -> no framing block. CSP not written (B3 :533) -> no contradiction; frame-ancestors would be decided there.
- SOURCE (read 2026-11-04): MDN "Third-party cookies" guide, https://developer.mozilla.org/en-US/docs/Web/Privacy/Guides/Third-party_cookies (last modified 2026-06-15): "Cookies must have the SameSite=None attribute set to be used in a cross-site context"; SameSite=Strict "effectively blocks third-party cookies from being sent in any circumstances"; to embed cross-site content in iframes relying on cookies "you need to explicitly set SameSite=None". Firefox Total Cookie Protection by default (separate jar per site); Safari ITP by default; Brave blocks tracking cookies by default; Chrome does not block by default. Supports F2 (B4's SameSite=Strict cookie not sent from a web portal's iframe). Not for the ADR text (not researched there) - review evidence only.
- SOURCE (read 2026-11-04): MDN Set-Cookie: Lax excludes "navigations inside <iframe> elements" and fetch(); Strict "only for requests originating from the same site that set the cookie".
- SOURCE (search snippet only, 2026-11-04, NOT read): npm @poki/cli (https://npmjs.org/package/@poki/cli) "upload game builds directly from your terminal or CI-pipeline" -> Poki takes uploaded builds (so a web-portal launch may not use our client host). Low confidence; used only to show F1's "same client host" may be false, F1 stands on "unsupported" alone.
- README:156 register row repeats F1 ("Web-portal launches would reach the same client host"). README counts 23/18, total 118/98: correct (0002 has 9 headings, 0006 has 23).
- CI facts (gh run view, read 2026-11-04): push run 41807980703 (event push, head 04da817, created 09:29:56Z): smoke 126499726624 cancelled (completed 09:30:45Z), gate 126499726876 success. PR run 41807986424 (pull_request, created 09:29:58Z): gate 126499746076 success, smoke 126499746906 cancelled (09:30:41Z). 1c49be7 runs: push 41808072877 created 09:30:32Z, PR 41808080671 created 09:30:35Z. gate.yml:15-17 concurrency group gate-${workflow}-${ref}, cancel-in-progress true -> push and PR runs in separate groups, each cancelled by 1c49be7's own run. Evidence file lines 5-10 give the step times: report's "09:30:25–41 push, 09:30:23–38 PR" correct.
- CANDIDATE F4 (nit): report:169-171 after "in both runs", "The job is shown as cancelled ... cancelled the run in progress; gate passed" is singular; both smoke jobs cancelled, both gates passed.
- CANDIDATE F3 (minor): STATUS:129-130 "runs only on a schedule or an API trigger, never on pull-request events" omits "by hand (Run now)", which B2 used (STATUS:418); the one wording (CLAUDE.md:138-140, STATUS:40-42, :403-406) says schedule only + by hand or API + never PR events.
- CANDIDATE F5 (nit): ROADMAP:94 "(the PR Advisor runs no scripts)" omits "installs nothing" (guard:399 npm only for night-shift; CLAUDE.md:135-136 says both).
- CANDIDATE F6 (nit, outside scope): STATUS:125-126 points to `.checkpoints/docs-after-kit-v3-1.md`, which the last commit deletes (CHECKPOINTS §9; plan Next steps 3) -> repoint in that commit.
- SOURCE (read 2026-11-04): MDN CSP frame-ancestors (last modified 2026-08-27): "specifies valid parents that may embed a page using <frame>, <iframe>, <object>, or <embed>"; 'none' "similar to X-Frame-Options: deny". -> B3's future CSP (0006:533, "a strict policy stays possible") is where framing by web portals is allowed or refused. Not written yet: no contradiction, a pointer only.
- Other checks: 0006:194 "production servers refuse preview origins" (a deny of preview origins, not an allow-list of ours): no contradiction. ADR 0001:451 "an Origin allow-list answered with 403 on both transports" and 0001:676 lobby "guest create (sets the cookie)", "Origin check": ADR 0001's (netcode-architect), pointers only. 0006:810 CD2 "client storage keeps settings and a guest cache": inside an iframe that storage is keyed by the top-level site (not researched in the drafts; not claimed). Access wall/previews (part 2) and noindex: previews only, no bearing on production embedding.
- Night shift next run 2026-11-04T23:10Z = 00:10 CET (Nov, UTC+1): STATUS 2026-11-05 00:10 correct. PR Advisor next 18:45 UTC = 19:45 CET: correct.
- SOURCE (read 2026-11-04): draft-ietf-httpbis-rfc6265bis-22 (2025-12-01), §5.2.1 Document-based requests: for a nested document, the site for cookies is the top-level origin only if it is same-site with the document's origin and every ancestor's; otherwise "an origin set to an opaque origin" -> requests from a cross-site iframe are cross-site: SameSite=Strict/Lax cookies not attached. Confirms F2 with high confidence.
- F1 -> major; F2 -> major (flag + pointer only, no design).
- CI on 603f53b: push run 41845554101 and PR run 41845561686, gate and smoke success (gh run view). Head 16c9a85 is [skip ci]: no runs (expected; the final commit runs CI, plan Next steps 3).
- V2_PHASE0_BRIEF:103 "effort high", :108 "effort high. Keep its triggers.": STATUS:44-46 cites them right.
- OQ-0002-9 (0002:1108-1109) describes OQ-0006-23 as "launches that share the servers and the player pool" (no client host) -> supports F1.

## Verdict: PASS WITH NOTES (my scale: REVISE, two small text fixes; no blocker)
1. F1 major: 0006:1156-1157, README:156 "the same client host" - unsupported, attributed to his words.
2. F2 major: 0006:1161-1164 related list misses part 6 (0006:278-279) and part 13 B4 (0006:534) (B3 :533 too); flag + pointer only.
3. F3 minor: STATUS:129-130 trigger wording omits "by hand".
4. F4 nit: report:169-171 singular job/run after "both runs".
5. F5 nit: ROADMAP:94 omits "installs nothing".
6. F6 nit: CLAUDE.md:131-132 "Each run's session record" from one record per routine.
7. F7 nit: STATUS:125-126 and :396 interim pointers (checkpoint file, condition 15 re-read) must be updated in the last commit.
