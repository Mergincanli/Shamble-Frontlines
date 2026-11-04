# verifier: fix check for docs-after-kit-v3-1 (PR #35), commit 884d33b vs 603f53b

## Plan
1. Extract the 30 upheld findings from tasks/wrpsw8dyr.output (done: scratchpad/findings.txt).
2. Read diff 603f53b..884d33b (excluding .checkpoints).
3. Per finding: resolved/partly/not with file:line at head 884d33b.
4. Re-check primary facts: gate.yml concurrency; gate jobs 126499726876 / 126499746076; checkpoint.mjs:30;
   PR #30 merged_at; routine-guard.mjs:446/:458; ADR 0004 W1/W4 + part 5; ADR 0006 part 13 B3/B4.
5. INFRA-2 (refuted): does the bare pointer in OQ-0006-23 break advisor condition 9?
6. New defects: contradictions (effort, trigger wording, A2/A3/A4, "web portal"), line widths, gate --no-smoke,
   repo-map-check.

## Verdicts (filled as I go)
- Primary facts (2026-11-04, gh api): PR #30 merged_at 2026-11-04T10:21:56Z (merge e2b5799). Gate jobs 126499726876
  (push run 41807980703) and 126499746076 (PR run 41807986424): both conclusion success, Gate step success. Smoke jobs
  126499726624 / 126499746906 cancelled, their Smoke step success (09:30:25-41, 09:30:23-38). 1c49be7 runs: push
  41808072877 created 09:30:32, pull_request 41808080671 created 09:30:35. gate.yml:15-17 group
  gate-${workflow}-${ref}, cancel-in-progress true (push ref and PR merge ref are different groups).
- checkpoint.mjs:30 regex re-tested: only %3D/%3d caught; %20 %2C %3A %22 %26 %2F %0A %2b %5C %7E missed. STATUS wording right.
- routine-guard.mjs:445 node/python -> interpreter; :446 date -> dateCheck; :458 pr-advisor "runs no program" inside
  interpreter(); :594 GITHUB_READ. guard.mjs:8-9 "A session keeps the hook settings it loaded while a branch switch...".
- ADR 0004:647-655 table: W1 and W4 Cost "None"; part 5 (0004:209-213) keeps W2 proposed, W3 measured alternative. EA-2 fix right.
- ADR 0006:533 B3 = content security policy; :534 B4 = where the guest secret lives; OQ-0006-6 (:914) = security
  baseline's waiting items. New related-list pointer (0006:1163-1165) names them by their row titles only.
- Gate (working tree = 884d33b + uncommitted STATUS:129 repoint + untracked report draft; docs only): GATE PASS, +0 -0,
  5a skipped. repo-map-check: OK 113/100.
- NOTE: the main session is editing the worktree during this check (STATUS:129 repointed to
  docs/reports/2026-11-04-docs-after-kit-v3-1.md "Mustafa's words"; report draft untracked). I judge 884d33b as committed.

## Per finding (at 884d33b)
SA-1 resolved STATUS:463-466 | SA-2 resolved advisor.md:68-69 | SA-3/INFRA-6/V1/CC-3 resolved CLAUDE.md:131-134
SA-4 resolved CLAUDE.md:139-143 (guard.mjs:8-9; B1 at 13:20 UTC off its 08:41 UTC cron, so not scheduled; B2 manual)
SA-5 resolved advisor.md:84 | SA-6 resolved STATUS:90 | SA-7/INFRA-3/V3 resolved STATUS:134-135
EA-1/CC-1 resolved GLOSSARY.md:641-642 (pointer; report still must list GLOSSARY.md) | EA-2 resolved 0002:1102-1104
EA-3/INFRA-1 resolved 0006:1156-1157, README:156 | EA-4 resolved 0002:1099-1100 | EA-5 resolved 0002:367
INFRA-4 resolved report:170-172 | INFRA-5 resolved ROADMAP:94 | INFRA-7 partly (STATUS:129 names "Mustafa's answers",
report section is "Mustafa's words"; uncommitted fix in progress), :400-401 resolved
V2 partly (same pointer) | V4 resolved LEGACY_INVENTORY:424 | V5 resolved STATUS:16-17, :90 | V6 resolved STATUS:400-401,
:229, LEGACY_INVENTORY:54-55 | CC-2 resolved CHECKPOINTS.md:66-68 | CC-4 partly (row :83 resolved; pointer :129 as V2)
CC-5 resolved STATUS:468-469 | CC-6 resolved STATUS:130-131
INFRA-2: pointer does not break condition 9 (titles only, no iframe/cookie statement); must be disclosed (report:69 does).
## New defects
- minor: STATUS:22-26 A4 review-policy queue entry: not asked by any finding, no source in repo (checkpoint:99 only says
  queued; report draft quotes none of his A4 words). Fix: quote his A4 words with date in the report's Mustafa's words.
- nit: STATUS:83 "each fixed in the last docs commit" -> name 884d33b (later docs commits follow).
- nit: ragged mid-paragraph wraps advisor.md:69, CHECKPOINTS.md:68, STATUS:402 (all <120). Reflow.
- nit: condition 15 asks the re-read right before the final docs commit; more docs commits follow 884d33b.
- nit (optional): report:170-171 PR run was cancelled by 1c49be7's pull_request run (09:30:35), not the 09:30:32 push run.
DONE
