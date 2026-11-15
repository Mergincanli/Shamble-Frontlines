Task: advisor plan review r1 of preview-hosting-check plan.md (revision 1, commit 27e9f50), one recorded review, verdict APPROVE/REVISE/ESCALATE.
Plan of blocks:
B1 read Mustafa's task + notes
B2 read plan.md (all parts)
B3 read checkpoint preview-hosting-check.md
B4 read sources: SPIKE_PLAN entry 6 + :12, ADR 0006 cited ranges, ADR 0002 part 10 + PM-0002-66, DECISIONS (advisor tool moments)
B5 spot-check Parts 3/4 claims against notes files
B6 review tr-draft workflows (Part 7)
B7 rulings Q1-Q6 + other checklist items
B8 verdict
Notes:
B1 done (task: 8 items; never touch spikes/; no edit outside checkpoint; review plan). Next B2.
F-cand1 (Medium?): Part 1.1 PM-0006-3 row says Safari "on two images"; browsers.yml (7.4) has only macos-26. Inconsistent.
F-cand2 (Medium?): browsers.yml comment: "three links (and the planted no-cache preview)" - bite-coep preview (PM-0006-3 bite: missing COEP must give false) not named; links.json content unspecified. Check ADR PM-0006-3 bite.
F-cand3 (Low/Med): Sitting 3 reviewer-off window: any other preview run (PR synchronize) would get token unapproved; plan should say session pushes nothing else and no run waits during that window.
F-cand4 (Low): probe.yml (main context, read-only token, no secret) unzips PR-run artifacts; data only; unzip traversal; Low.
Note: deploy job no checkout, functions dir check harmless (cwd has no functions/). Plan job output from PR code -> matrix -> env only. OK.
B2 done (plan read in full). Next B3 checkpoint.
F-cand5 (Low): checkpoint header "Last update 16:47 Amsterdam (15:47 UTC)" stale vs entries to 16:28 UTC; step 8 says plan 1,183 lines, file is 1,223 after consultation-2 fixes (27e9f50). Records otherwise consistent: slip recorded honestly with times (0290625 16:16:22, d8451dc 16:21:42, consultation 2 16:22:00).
F-cand6 (Low): plan Part 8 "advisor tool ... always before a step is saved as done" silent on the recorded slip; closing message must carry it (checkpoint says it will).
B3 done. Next B4 sources.
B4 partial: sources read (SPIKE_PLAN :12, :45-70, :416-438; ADR 0006 :1198-1222, :1277-1376, :1401-1415, :1436-1457; 0002:1331-1334; GLOSSARY §5; r5 OQ6/F18).
F-noindex (Medium): Part 2 preview mode writes no noindex rule (relies on host default) though ADR 0006 part 1 (:130-133) says the build's generated _headers carries the preview set (= production + noindex, GLOSSARY §5). Unlisted departure; and the plan does not say whether a link form missing the host's noindex FAILS PM-0006-2 or is recorded. Fix: list in 1.1 and state outcome, or write noindex in preview modes.
F-browsers (Medium): browsers run never placed in the calendar (Part 3 C / Part 5); the bites' Pages branches unnamed; links.json = only project address; browsers.yml comment names 3 links + no-cache, not bite-coep -> PM-0006-3's bite (bite-coep must give false in browsers) may go unmeasured. Fix: name branches per set, list links incl bite-coep, sequence browsers runs before the bite branches change.
Departures ruling: X-Check-Rule = observation, no mark change (allowed; check-headers must tolerate extra header). Session's 24h probe = extra reading, allowed. PM-0006-10 route = allowed (mark asks a screenshot, any view). PM-0002-66 bite = PM-0002-66's own bite, names TR (0002:1332) -> allowed; report must not claim PM-0002-66 passed whole.
(guard refused two notes writes from this agent: relative path, then a variable target; route: literal absolute path)
B4 done (DECISIONS :470-482 five moments read). B5 done: spot-checked vs notes - G1 quotes (secrets wait for reviewer; prevent self-review; Protected-branches-only lets all deploy; bypass on by default), G4/G5 (GITHUB_REF refs/pull/N/merge; merged close = base ref; payload "empty for merged"), fork secrets, redaction not guaranteed, G9 (Pages Write single perm; DNS/Workers separate = inference, labelled); C1-C7 (direct upload path, 00:00 UTC TTL, account tokens for CI, latest-deploy sentence vs force, 1010, serving-pages noindex/404, 25 MiB in source, agent delegation read at 4.162.0 only). All backed; "not read today" markings honest (A1, A3 menu, D5, E1/E2). Over-claim (Low): Part 3 A4 "it never redirects a project that already exists (Wrangler's source)" was read at 4.162.0; 4.156.0's file exists but contents not read (C3 note line 95).
B6 done: embedded YAML == tr-draft files (script), sha256 match; pins = gate.yml (checkout, setup-node). PR values reach shell only via env (if: expressions and with: names aside). No checkout in token jobs. 
F-npx (Medium, security): deploy.yml upload step and close.yml tomb step run `npx --yes wrangler@4.156.0` WITH CLOUDFLARE_API_TOKEN in env: npm install lifecycle scripts of wrangler's unlocked deps (esbuild/workerd postinstall) run holding the token. Fix in Part 7 text: install wrangler in an earlier step with no secret in env (npm install --prefix $RUNNER_TEMP/w wrangler@4.156.0, optionally --ignore-scripts), then call that binary in the token step. Plan's Part 4 item 2 names the residual but not this cheaper narrowing.
F-prod-oneoff (Medium): session's own held-back fix: host may add noindex on production's one-off link -> PM-0006-2 "production none" on both URL forms. Ruling: pre-register: record per link form; if seen, it is the row's letter not met, reported and sent to the draft's author by :1409-1414; the check never re-reads it as a pass.
F-reviewer-window (Low): sitting 3 bite window: nothing else pushed or pending on preview; reviewer restore proven by next waiting run.
F-nested-github (Low): tr-draft/.github/workflows/ path in this repo's pushed notes may trip path rules (.github/ substring) in reviews; harmless to GitHub; rename to tr-draft/workflows/ next save, or note it.
F-preview-start-date (Low): Part 0/1 PREVIEW-START dated 2026-11-15; by note 1 it is 2026-11-14 (session already lists it).
Q1 ruling: :12 not binding against a separate TR: ADR 0006 :1280-1281 defines TR for this very spike (Mustafa names it), 0002:1332 says "the test repository ... never against the game's own production"; SPIKE_PLAN "decides nothing new" (:7-9) and its entry 6 depends on TR. :12 governs where spike code lives inside this repo. No stop. TR's place stays Mustafa's (TR-WHERE).
Q3 ruling: check = risk tier 2 (secret, CI, infrastructure) with tier-3 parts (ACCESS-CARD payment details; TR-END deletions; Cloudflare project deletion) decided and done by Mustafa only. Closing docs PR = tier 1 needs-mustafa (records his words/verdicts); tier 2 does not carry over (its files/effects add, move or read no secret, change no infrastructure); becomes tier 2 if it touches .github/ or carries any secret-bearing value.
B7 done. Next B8 verdict: APPROVE with conditions (no Critical/High).
Advisor-tool consult (after B7): added checks below.
F-winshell (Medium): browsers.yml run step on windows-2025 has no shell: -> PowerShell; "$BROWSERS" there is a PS variable, not the env var -> --browsers empty, Chrome/Edge/Firefox rows of PM-0006-3/-4 never run. Fix: `shell: bash` on that step.
F-node (Low): plan/build/verify/probe run node without setup-node; Part 2 says Node 22; add setup-node (gate.yml pin) or record the image's node.
F-decisions (Medium): Part 8's closing change-set lists STATUS, ROADMAP, SPIKE_PLAN pointer, not docs/DECISIONS.md: his seven answers (and PREVIEW-START) must be recorded word for word; MACHINES (a) rows rest on it.
F-prod-oneoff consequence: a letter-fail of PM-0006-2 holds the check's end and step 2.6 until the draft is revised (:1401 every mark must pass); Part 5 and Part 9 must say so.
Exact replacement lines (for conditions):
 deploy.yml, new step before `- id: upload`:
      - name: Install Wrangler (no secret in this step)
        run: npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
   and in the upload step: `npx --yes "$WRANGLER" pages deploy` -> `"$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy`
 close.yml: the same install step (with the tomb step's `if:`) before `- id: tomb`, and the same binary in the tomb step.
 browsers.yml: `shell: bash` on the "Isolation (PM-0006-3)..." step; header comment "opens the three links (and the planted no-cache preview)" -> "opens the three links, the planted missing-COEP preview and the planted no-cache preview".
 F-noindex one choice: preview, preview-isolated and the preview bite modes write `X-Robots-Tag: noindex` on `/*` (the draft's generated preview set, ADR 0006 part 1); check-headers accepts a value that contains noindex (once or repeated); the host's own default is recorded from what comes back.
Q2 ruling (his three): MACHINES his: Safari on a macOS runner has a route (key :1306-1310 + SPIKE_PLAN "each spike's own plan's, in the same way"), with the version recorded and real Safari; the Windows runner for ML has none (his laptop "rough checks only... never judged against a pass mark"): new, his; the four scripts are not hardware results (:1367). TOKENS his (departs from part 13 interim (a)); TOMBSTONE his (twin cases are the faithful form); both branches costed in each.
Q3 add: close.yml's forced deletions are not tier 3: they delete only the check's own preview uploads (PROJECT fixed, env=preview), the mechanism PM-0006-6 prescribes, each run behind his approval.
Checklist: reviewers (infra, security-auditor, test-engineer; verifier on evidence) right for tier 2; effort high fits CLAUDE.md, his "max" names game and engine code, and the plan asks him; Brave checklist present (Part 3 C/D); <=3 agents; checkpoint kept.
Not verified by me: artifact-action SHAs (checkpoint's ls-remote), runner labels windows-2025/macos-26 and Safari on macos-26, no source page re-read (notes only), Wrangler 4.156.0 behaviour.
Session already lists (confirmed, not new): one macOS image; PREVIEW-START date 2026-11-14; production one-off noindex reading.
VERDICT: APPROVE with conditions: (1) fix A1-A6 in revision 2; (2) tr-draft updated, re-embedded, re-hashed, re-parsed; (3) the closing message tells Mustafa which Part 7 lines changed after this review; U4 (security-auditor, infra-engineer) and step 6 read them before anything runs; (4) Lows ride along.
DONE
