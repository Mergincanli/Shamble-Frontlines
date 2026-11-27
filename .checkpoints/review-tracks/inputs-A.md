# Inputs A for change-set "review tracks and who merges"

Read-only research on main as fetched (origin/main). No tracked file edited, nothing committed or pushed.

## Part 1. Timings of #65 (stop 3a), #68 (stop 3b), #69 (guard fix)

### Method
- Merge commits found with `git log --merges --oneline origin/main | grep -E '#(65|68|69)'`:
  #65 = 28254a73 (2026-11-24 23:40:38 +0100 = 22:40:38 UTC), #68 = 3ab9eaec (2026-11-25 14:57:02 +0100 =
  13:57:02 UTC), #69 = 311c0484 (2026-11-25 17:34:47 +0100 = 16:34:47 UTC). GitHub's `merged_at` agrees to the
  second (22:40:39Z, 13:57:03Z, 16:34:48Z; `gh api repos/Mergincanli/Shamble-Frontlines/pulls/<n>`).
- Branch history of each: `git log --reverse --format='%h %ci | A %ai | %s' <merge>^1..<merge>^2` (author and
  committer times are identical on every commit; branch commits are in +0000, the merge commits in +0100: all
  times below are converted to UTC).
- The PR timelines (`gh api .../issues/<n>/timeline`) hold no useful ask time: each PR was opened late (#65 at
  21:18:58Z, #68 at 13:42:53Z, #69 at 15:14:35Z) and marked ready seconds before the merge (#65 22:40:26Z, #68
  13:56:47Z, #69 15:33:50Z); no GitHub reviews or comments exist on any of the three. So the clock comes from the
  commits plus the reports and checkpoint cards (`docs/reports/2026-11-24-cs3a-rules-boundaries.md`,
  `2026-11-25-cs3b-the-loop-closes.md`, `2026-11-25-cs3b-browser-check.md`, `2026-11-25-guard-env-fix.md`;
  the cs3a card at `git show 6491b839:.checkpoints/cs3a-rules-boundaries.md`).

### Definitions used
- **Building** = first code commit after the plan's approval -> the last planned build step (before review round 1
  starts). For #69, which has a single code commit, from the approval commit to that code commit.
- **Checks** = end of building -> the last review / verifier / advisor / docs commit before the head that went to
  Mustafa (the browser-check zip for #65 and #68; the final head for #69).
- **Waiting** = that commit -> the merge (GitHub `merged_at`). Split below into "waiting for his check/answer" and
  "after his answer to merge" (re-review of the last head, his tap).

### The anchor commits (UTC)
| | #65 stop 3a | #68 stop 3b | #69 guard fix |
|---|---|---|---|
| His go / plan approval | his go 11:03:53; advisor APPROVE commit f4a38f94 11:25:09 | advisor APPROVE 8e3c0a75 23:12:13; his go 23:24:50 | plan 20902d9b 14:18:10; APPROVE c11b68fe 14:22:29 |
| First code commit | a1c59e87 11:35:59 (step 1) | 1ee3e918 23:41:36 (manifests) | 21615cb3 14:39:18 (the edit) |
| Code finished (last planned step) | 5c923119 13:55:51 (step 8) | bbbf09e4 00:44:32 (step 6; card eb0d2314 00:45:02) | 21615cb3 14:39:18 (fix rows c3827f40 15:01:43) |
| Last code-changing commit | 8619d539 16:18:23 (8d277cf2 17:38:35 comments only) | 6abf3d2a 03:08:26 | c3827f40 15:01:43 |
| Last pre-wait commit (head to him) | 4de01a0c 17:46:13 (advisor APPROVE 8d277cf2 17:45:01; zip sent ~17:42, ec1a7d26) | 7dc6e1d6 04:22:54 "waiting for Mustafa" (advisor APPROVE for the zip 01bea7fe 04:22:18) | 08bb20af 15:24:16 (report + docs; = final head) |
| His answer | browser check 21:48:49 | RELEASE-COUNT + Brave 11:38:37; Firefox 12:14:23 | none needed before merge go |
| Final head committed | 1624f084 21:53:38 | 5e02ee73 12:40:59 (merge of main #66 in) | 08bb20af 15:24:16 |
| His merge tap / go | 22:40:00 (his paste of the approved head 22:28:40) | 13:56:26 | not recorded in git or the report |
| Merged | 22:40:39 | 13:57:03 | 16:34:48 |

### Table: his number vs mine (hours)
| PR | Before first code (mine only) | Building (his / mine) | Checks (his / mine) | Waiting (his / mine) | Whole branch: first branch commit -> merge (his sum / mine) |
|---|---|---|---|---|---|
| #65 stop 3a | 0.18 (f4a38f94 11:25:09 -> 11:35:59) | 2.4 / **2.33** | 4.0 / **3.84** | 4.9 / **4.91** | 11.3 / **11.26** |
| #68 stop 3b | 0.87 (51daf6c9 22:49:37 -> 23:41:36) | 1.1 / **1.05** | 4.9 / **3.64** | 9.2 / **9.57** | 15.2 / **15.12** |
| #69 guard fix | 0.07 plan -> approval (14:18:10 -> 14:22:29) | 0.3 / **0.28** (approval -> code) | 1.0 / **0.75** | 1.2 / **1.18** | 2.5 / **2.28** |

Consistent variant of building (approval commit -> last planned build step, same rule for all three): **2.51 / 1.54 / 0.28 h**
(f4a38f94 11:25:09 -> 13:55:51; 8e3c0a75 23:12:13 -> 00:44:32; c11b68fe 14:22:29 -> 14:39:18).
My four cells add up to the whole-branch span (0.18+2.33+3.84+4.91 = 11.26; 0.87+1.05+3.64+9.57 = 15.13;
0.07+0.28+0.75+1.18 = 2.28).

One-line reasons for differences:
- #65 building: within rounding; if building starts at the approval commit 11:25:09 instead, it is 2.51.
  Checks 4.0 vs 3.84: his split point is ~10 min earlier (e.g. step 7, 13:50:40 gives 3.93) or the ask taken a few
  minutes later; the waiting cell agrees.
- Whole-branch totals agree within rounding for #65 (11.3 vs 11.26) and #68 (15.2 vs 15.12): his three cells
  appear to cover the whole branch, the leg before the first code commit included (0.18 h for #65, 0.87 h for #68:
  the 3b chat re-read the plan, got the advisor agent's APPROVE and his go after a compact), which my definitions
  leave out of building.
- #68 checks 4.9 vs 3.64 and waiting 9.2 vs 9.57: counted back from the merge, his waiting starts about 04:45 UTC,
  about 22 min after the last pre-wait commit (04:22:54) and about 20 min after the advisor agent's APPROVE for the
  zip ("about 04:25 UTC" in the report); no commit or report time marks 04:45 (the chat's ask to him may have gone
  then; not recorded). Counted forward from the first branch commit, his building ends about 23:55, at the steps 1-3
  checkpoint (5c582ddd 23:57:02), before steps 4-6, so his "checks" also hold steps 4-6 (about 0.8 h of build).
  So his split points are **not reproducible from any commit**, though the whole span matches. Most of the wait
  (7.26 h) is the night before his Brave report (04:22 UTC is 05:22 Amsterdam).
- #69 checks 1.0 vs 0.75: his sum (2.5) overshoots the whole-branch span (2.28, plan commit to merge) beyond
  rounding, so at least one of his cells is too long; checks measured from the approval (14:22:29 -> 15:24:16 =
  1.03) would explain it if building were then counted twice.

### Waiting, split
| PR | Last pre-wait commit -> his answer | His answer -> final head committed | Final head -> merge | What the end step held |
|---|---|---|---|---|
| #65 | 4.04 h (to the browser check 21:48:49) | 0.08 h | 0.78 h | record his check, fold checkpoint; advisor + verifier read 1624f084; his paste 22:28:40; tap 22:40:00 |
| #68 | 7.26 h (to Brave 11:38:37; Firefox +0.60 h) | 0.44 h (incl. merging main's #66 in at 12:40:59) | 1.27 h | advisor read of 5e02ee73; tap 13:56:26 |
| #69 | no check asked; head = final | 0 | 1.18 h | advisor read of 08bb20af (per the report's For Mustafa); ready 15:33:50; merge (who pressed it not recorded) |

Notes: in both 3a and 3b, after his browser check a new head was committed (record + checkpoint folded, and for 3b a
merge of main), which needed its own advisor read and his go for that SHA: the "after his answer" leg is 0.9 h (3a)
and 1.7 h (3b). The browser check is the dominant part of the wait for 3a/3b; for #69 (tier 2, no visual) the whole
wait is the re-review + his go.

## Part 2. Every reason a pull request needs Mustafa today

Sources read: `CLAUDE.md`, `.claude/agents/advisor.md`, `.claude/NIGHT_SHIFT.md`, `.claude/CHECKPOINTS.md`,
`.claude/hooks/routine-guard.mjs` (merge rule), `.claude/settings.json` (deny list), on origin/main 2671595f.
Line numbers checked with `sed -n`/`grep -n` against origin/main 2671595f as last fetched (the working tree, HEAD
4e8a87e8, differs from it only in `docs/TECHNICAL.md`, one line: noted where cited). "Dup" marks the same reason stated in another place.

### A. Reasons that make a PR needs-mustafa (his go before merge)
| # | Reason (paraphrase) | Where (file:line) | Same reason also at (dup) |
|---|---|---|---|
| 1 | **Player sees, hears or feels it** (visuals, UI, animation, audio, feel, balance, controls): his Brave check, then his go | advisor.md:37 (escalation cat. 1) | CLAUDE.md:93-95 (step 6), :127-128 (standing permission), :292 (Never: merge visual work before Brave check); advisor.md:49-50 (his A4 words "Brave check when players see or hear it"), :71-72 (tier 1), :95 (class map), :193-195 (browser checklist), :235 (tier-1 checklist); NIGHT_SHIFT.md:77 |
| 2 | **Design decision the docs don't settle** (new rules, units, mechanics, economy beyond tuning, art direction / the look) | advisor.md:38 (cat. 2) | advisor.md:110 (the look is his); CLAUDE.md:5-6, :24-25 (look brief: never decide the look) |
| 3 | **Rule files and other tier-2 paths** (hard constraints, ADRs, CLAUDE.md, `.claude/`, `.github/`, security rules, gate files; also `CLAUDE.local.md`/`AGENTS.md` anywhere, agent memory, any gitignored path, and a file renamed or moved from outside tier 0) = risk tier 2, "always Mustafa's go"; kit/CI/gate merge only from a day session | advisor.md:39-43 (cat. 3), :64-66 (tier-0 exclusions that are tier 2), :75-86 (tier 2) | CLAUDE.md:102, :104 (tier table), :94 ("reconfirmation for rule changes"); advisor.md:50-51 (his A4 words), :239-245 (tier-2 checklist), :325 (Never); dup by mechanism: #13, #14, #15 below |
| 4 | **Secrets and infrastructure** (adds/moves/reads a secret; hosting, servers, deploys, domains, repo or CI settings) = tier 2 | advisor.md:76-78 | CLAUDE.md:104 ("secrets, infrastructure"); infra-engineer.md:40 (accounts, services, secrets, repo/CI settings "are Mustafa's") |
| 5 | **Hard-constraint change**: needs an ADR accepted by him on main first | CLAUDE.md:48 | CLAUDE.md:291 (Never); advisor.md:324 (Never) |
| 6 | **Decision docs**: a doc that records his words or decisions (quote, `[DECIDED]`, answer to his code), ADRs and register (`docs/adr/`), design and look docs = tier 1, "always needs-mustafa" | advisor.md:61-64 | CLAUDE.md:102 (tier 0 excludes them), :107 ("decision docs always need his go"); advisor.md:95, :178; routine-guard.mjs:87-88, :98 (DECISIONS.md and the design docs refused by the merge allow-list). Note: every change-set whose docs pass copies his words into `docs/DECISIONS.md` trips this (cs3a report :6-7, cs3b report :5-6 both say so) |
| 7 | **Irreversible or costs money** (tier 3): agents only propose; his decision of the act, then his go | advisor.md:44-45 (cat. 4), :87-90 | CLAUDE.md:105; advisor.md:246-248 (checklist), :331 (Never) |
| 8 | **Major change worth a second opinion**: new system or mechanic, large refactor of an engine file (his reconfirmation) | advisor.md:102, :178-180 | CLAUDE.md:94 ("other major changes"); advisor.md:3, :196 ("Reconfirm the merge") |
| 9 | **A measured regression the PR accepts** | advisor.md:102 | advisor.md:180 |
| 10 | **A security finding left open**, at any severity (a STATUS follow-up counts as open) | advisor.md:102-104 | advisor.md:180-181, :266-268 (budget never cuts it); security-auditor.md:54. Example: #69's report "open security findings keep this needs-mustafa" |
| 11 | **A doubt the checks can't settle headless; when unsure, or both lists fit** | advisor.md:104 | advisor.md:181-182; :174 ("can't be verified headless → Mustafa's checklist") |
| 12 | **"Verify in a day session"**: the PR changes `shamble-*.js` or the pages, or its headline claim needs a sim, render, measurement or bug-repro the routine can't run | advisor.md:100-101 | advisor.md:166-169 |
| 13 | **CI's green does not stand**: the PR touches `.github/` or a gate file, so CI ran the PR's own gate | advisor.md:100 | advisor.md:164-167. Dup of #3 in effect (such a PR is tier 2 anyway) |
| 14 | **Step 0, self-referential rules**: the diff touches `.claude/`, `CLAUDE.md` or `.github/` → day-session advisor review | advisor.md:98 | advisor.md:139-141, :239. Dup of #3 in effect |
| 15 | **Outside the guard's merge allow-list**: the PR Advisor may merge only plain paths in `docs/`, `tools/`, `README.md`, `FIRST_PROMPTS.md` with no rules, gate or decision-doc name; anything else (e.g. all v2 code: `crates/`, client, Worker) merges from a day session, and a session merges only on his go | routine-guard.mjs:79-100 (`mergeAllowed`), :1580-1590 (merge check) | advisor.md:72-74, :80-86, :203; CLAUDE.md:103 ("outside the guard's allow-list: from a day session"), :128-132 ("A session merges only on Mustafa's own go ... every other merge is the PR Advisor's"). Structural: this makes a `ready` tier-1 code PR still wait for his go |
| 16 | **A guard refusal blocks a routine step** → that step is needs-mustafa | advisor.md:98-99 | advisor.md:141-143 |
| 17 | **Critical or High still open after review round 3** → stops the change-set for him | CLAUDE.md:111-112 | advisor.md:262-263 |
| 18 | **Revert that drops game symbols** → CI's gate fails by design; needs-mustafa (a fix forward, or his own way past the required check) | advisor.md:300-303 | Other revert cases are instances of #12 (revert of a game-file merge, advisor.md:304-305) and #3 (revert of a kit/CI/gate merge on his go, :305; ask him before `git revert` on kit files, :285-287) |
| 19 | **PR from a fork**: skipped, "tell Mustafa"; the guard refuses a fork merge | advisor.md:153 | routine-guard.mjs:1564 |

Re-trigger (not a new reason): **a later push** needs the advisor's review, then his go again: CLAUDE.md:131-132;
advisor.md:204, :256. Go validity: his own words in the merging session's chat, about that PR, after the advisor
approved that head; a GitHub "merge" never counts: CLAUDE.md:128-133; advisor.md:205-208.

**Count:** 19 rows; #13 and #14 are the same reason as #3 by effect, so **17 distinct reasons** (15 everyday, plus
two edge cases: #18 symbol-dropping revert, #19 fork). Of the 17, the tier/category reasons are #1-#7; the
"second opinion" reasons #8-#11; the seat/mechanics reasons #12, #15, #16; process #17; edge #18, #19.

### B. Mustafa gates that are not PR-merge triggers (for completeness)
- Plan review ESCALATE (a decision only he can make): advisor.md:36, :128; CLAUDE.md:62-63.
- Phase 0: no spikes / engine code until he and the Advisor approve the drafts: CLAUDE.md:21-23; advisor.md:107-108;
  NIGHT_SHIFT.md:57-58.
- Kit or gate file edits in a day session ask him first (edit time, not merge): CLAUDE.md:67-71;
  routine-guard.mjs:11-13.
- Night shift never merges; anything needing him goes to "For Mustafa", never the queue: NIGHT_SHIFT.md:63;
  CLAUDE.md:165-166; routine-guard.mjs:1561.
- DEP-HOLD: a manifest/lockfile change is held for security-auditor's answer, not for him: CHECKPOINTS.md:60-69.
- Main broken: hold every other PR until the revert merges (no Mustafa step unless no session can work: he opens
  the revert): advisor.md:277-280, :306.

### C. In his recorded words (docs/DECISIONS.md), not in the kit files
These bind sessions today but are not among the kit's needs-mustafa triggers; the plan's table (his item 3g) should
decide where each goes.
- **"My Advisor's merge check comes before my go"**: DECISIONS.md:260 (item 7), :432 ("A merge is on my go for the
  approved head, after my Advisor's merge check"), :225 (#73's go), :476. In practice the #65 and #68 merges were
  done by his main Advisor thread (a claude.ai project session) on his tap on a decision card, with the expected
  head (merge commit messages of 28254a73 and 3ab9eaec).
- **TOOL-SOURCE**: any new download, file, version or checksum comes back to him before use: DECISIONS.md:2220,
  :1348.
- **Size stop lines**: a change-set reaching its stop line goes to him (e.g. STOP-4A): DECISIONS.md:412-431; and
  CAP / the 150-line smaller-version rule for this change-set: DECISIONS.md:154.
- **SOURCE-GO (a)**, to be added by this change-set (not in force as a kit rule yet): "a change under
  content/source/ needs my go": DECISIONS.md:115, :122-123.
- **EYES-KEEP (a)** (his brief, proposal for this change-set): money/irreversible, the rule files, and a decision of
  his changed or newly set keep his go; "A record of words I typed, copied by script, is not in this list":
  DECISIONS.md:145. This removes #6's most frequent trip (the DECISIONS.md script copies).

## Part 3. Merge mechanics today

- **Who may merge what.**
  - The PR Advisor routine: `ready` PRs (tier 0, or tier 1 with nothing for human eyes and no trigger) at once
    when approved with checks green, and needs-mustafa PRs after his go, but only if every path passes the guard's
    allow-list; otherwise "it merges from a day session": advisor.md:33-34, :91-94, :200-204; CLAUDE.md:91-95,
    :102-103.
  - Day sessions: only on his own go in that chat, about that PR and its current head, after the advisor approved
    that head (its verdict in the session, or its PR review comment by `Mergincanli` naming the full SHA), gate and
    smoke green, the SHA passed as expected head: CLAUDE.md:128-133. Tier 2 kit/CI/gate: only from a day session
    (CLAUDE.md:104; advisor.md:79-80).
  - The night shift: never (NIGHT_SHIFT.md:63; CLAUDE.md:131; guard routine-guard.mjs:1561).
  - Mustafa himself: under MAIN-WALL (2026-11-26, about 17:15 UTC) only his account updates main, and every merge
    (his included) still needs a PR with `gate` and `smoke` green; every session acts as his account, so the wall
    separates his account from outsiders, not his decisions from a session's acts (docs/TECHNICAL.md:234-241,
    STATUS.md:25). origin/main's TECHNICAL.md:241 still says one day-session merge and one PR Advisor merge under the
    wall are open (T10); the working tree's later version of that line records #76 merged through the wall by a day
    session (2026-11-26 18:00:25 UTC, 2671595f); the PR Advisor's first merge under the wall is unmeasured in both.
- **How the PR Advisor decides and labels** (advisor.md:130-215): step 0 seat check (`SHAMBLE_ROLE=pr-advisor`,
  detach to origin/main; a diff touching `.claude/`, `CLAUDE.md`, `.github/` → needs-mustafa); step 1 load (skip
  drafts, forks; re-review only what changed); step 2 verify (reads CI's `gate`/`smoke` check runs on the SHA; that
  green stands only if no `.github/`/gate file changed; game files or claims needing sim/render → needs-mustafa);
  step 3 vision; step 4 classify (tier first, then needs-mustafa vs ready); step 5 ONE review comment (verdict +
  full SHA, tier, TL;DR, what it verified, findings, Decisions needed, For Mustafa: browser checklist or
  "Reconfirm the merge of #n"); step 6 act: labels `ready-to-merge` (and merges), `needs-mustafa` (notify him; merge
  after his go if the allow-list admits it), `changes-requested` on REVISE/BLOCK. The guard limits its GitHub
  writes to PR comments and label-only `issue_write` (routine-guard.mjs:1546-1553). Nothing in the guard reads a
  label: labels are bookkeeping, not enforcement.
- **Schedule** (STATUS.md:75-79): PR Advisor `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *` (07:45, 13:45, 19:45
  Amsterdam), schedule only, never pull-request events, Opus 5.5 at max effort; Night shift `10 0-6 * * *`
  (NIGHT_SHIFT.md:2-3 says runs to 07:10, a doc drift against STATUS's `0-6`; the archive, STATUS_ARCHIVE.md:2153,
  has `0-7`). Last read 2026-11-10. CLAUDE.md:196-198: started by hand or API too, never by PR events.
- **What the guard checks at a merge** (`mergeCheck`, routine-guard.mjs:1556-1593; applies to every session):
  `expectedHeadSha` present and a full 40-hex SHA (:1558); for routines, the repository is this one (:1560); the
  night shift is refused (:1561); the PR's head equals that SHA (:1563); not a fork (:1564); the check-run list read
  whole (:1565-1567); `gate` and `smoke` both present, completed and green (:1568-1573; `toolchain` is not
  required by the guard); for the PR Advisor only: the changed-file list read whole (<3000, both names of a
  rename/copy) and every name passes `mergeAllowed` (:1574-1590). The check fails closed (:1507). Also refused for
  every session: `gh pr merge` (:1452; settings.json:229), GitHub auto-merge (:1505; settings.json:258), any
  `gh api` write (:1466). In a day session `merge_pull_request` is not on the allow list (settings.json:79-87), so
  the normal permission prompt / classifier applies. The guard does not check the advisor's approval, his go, or
  labels: those are prose rules (CLAUDE.md:128-133; advisor.md:205-210).
- **Observed on the three PRs measured**: none was reviewed or labelled by the PR Advisor routine (no reviews, no
  comments, no labels on #65, #68, #69 via `gh api`); each PR was opened late and marked ready seconds before its
  merge; #65 and #68 were merged by his main Advisor thread (claude.ai project) on his tap with the expected head,
  #69 by the `Mergincanli` account (every session and he act as it); the report asked for his "merge #69" in a day session's chat, but who pressed the merge is recorded neither in the merge message nor in the report (DECISIONS.md:655 only records that it is merged).
