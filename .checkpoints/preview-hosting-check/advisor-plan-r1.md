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
