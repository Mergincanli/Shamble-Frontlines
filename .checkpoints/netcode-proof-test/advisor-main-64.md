# Advisor: MAIN-64 plan check (main 1ec6f6c into PR #40's branch)
2026-11-24, 10:3x-10:5x UTC. Read only. Acting by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`,
read whole). Plan: main-64-plan.md (85 lines, sha256 ed31452e…db777 on disk, equal to the brief's). Binding form:
main-61-plan.md, advisor-main-61.md C1-C6 (:19-104), advisor-main-61-head.md. One check (NPT-TRIM, history.md:7591).

## 1. Verdict: APPROVE WITH CONDITIONS M64-1 to M64-4
The take-in is due under MAIN-52 (history.md:2055-2057: "Take main in when main changes code, CI or rules"): #64
changes CI and code. Every fact I re-ran holds (section 3). The STATUS.md rule is right. What is missing is the
numbers: the plan says C1-C6 are "kept whole" and that "only the facts and numbers below differ", but leaves C1-C6's
f5030a3-era values in place. Read literally, C1 (c), C2 (b) and C4 would stop the take-in with a merge in progress or
committed. M64-1 gives them. Applied word for word, the conditions need no new review; step 7 stays.
No Critical or High.

## 2. Conditions
- **M64-1 (Medium): the numbers C1-C6 bind with, for this take-in.** Use these values in place of MAIN-61's (cited at
  advisor-main-61.md:19-104; the plan's :9 and :44-68 leave them unstated):
  - C1 (a), (b): the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with M0
    items, 0 findings). 5d must equal those word for word, except HEAD and refs, which must be N0 + 3. Not "78" or "10" from
    MAIN-61: the spike has changed since then.
  - C1 (c): docs/ prints the same six names. `git diff --name-only H0 HEAD` equals `git diff --name-only f5030a3
    origin/main` (19 names), and the shortstat reads "19 files changed, 2174 insertions(+), 99 deletions(-)".
  - C2 (a): "beyond main's 19". The allowance covers the card, the netcode seat's u6c-review-netcode.md and this file.
  - C2 (b): `git diff --cached --shortstat H0` prints "19 files changed, 2174 insertions(+), 99 deletions(-)".
  - C2 (c), C3: "builder 4" means the netcode seat (aa64bb15ad4467641) and its runner (u6c-run.sh:54-55, `-s` or
    `-x` on the same lock). Its handback is only read until the push. The check-in trig_01EKqNqwZrQTVJ6caAyRrug5
    gets card edits only.
  - C4: origin/main must be 1ec6f6c3852d2922989f299bfc053c403d0e52d3. Main's CI is three check runs (gate, smoke,
    toolchain, from the push event only; #64's PR runs were on its head 8aadfb0), all completed with success.
  - C5 (a): the resume line reads "MAIN-64 approved (advisor-main-64.md)", with 1ec6f6c3852d2922989f299bfc053c403d0e52d3
    in both places, and "The netcode seat's notes" in place of "Builder 4's notes".
  - C6: step 3's block is merged lines 24-31 (`<<<<<<< HEAD` :24, `=======` :27, `>>>>>>> origin/main` :31). Keep
    :25 (ours #40), :29 (#64) and :30 (#63, #62). Drop :26 (#61) and :28 (main's 2026-11-14 #40). Step 5b's expected
    line is "repo map OK: 173 entries cover 110 required paths".
- **M64-2 (Medium): the card stays at or under his 200-line cap** (TEST-FIRST rule 3, history.md:2265). The card is at
  200 lines now, and MAIN-61's take-in broke the cap (ADV-M61H-1, advisor-main-61-head.md:171). So the resume line
  replaces the "My step" lines (card :155-160) rather than adding to them. After the push, one done line remains (merge
  SHA, pushed SHA, CI). Step details go to history.md by Edit, with no new trim script (NPT-TRIM, history.md:7592).
  `wc -l` on the card before step 6's save must show 200 or fewer.
- **M64-3 (Low): the saver's window.** b1bd3pcdl runs until about 11:50 UTC (card :152). If it has already ended
  when step 1 starts, its completion notice plus an empty `pgrep -af "checkpoint.mjs watch"` is C5 (b)'s proof, and
  there is no TaskStop. C5 (c) and the restart at step 6 stay as they are.
- **M64-4 (Low): text.**
  - Step 3's bar is "exactly 120" (main's 120 lines, with one row swapped), not "at most 120" (plan :58).
  - Step 4 keeps MAIN-61's check that `git diff origin/main HEAD -- docs/STATUS.md` shows the #40 row alone.
  - Name the risk tier (plan :81): tier 1 (a spike PR). After the merge, .github/ and .gitignore equal main's, and C1
    (c)'s first check proves it.
  - Step 7 compares toolchain's `test result`, `Test Files` and Playwright pass lines with main's run 45373056953, as
    at MAIN-61 (advisor-main-61-head.md:119-122). It is written short (NPT-TRIM).

## 3. What I checked (read-only git and gh; merge-tree wrote objects only)
- origin/main 1ec6f6c3852d…; one first-parent merge since f5030a3 (#64, merged 10:15:31 UTC, gh). Merge base f5030a3.
  HEAD moved 2f6001f → 1a89aca during this check (saves only; `git diff --name-only a4186ca HEAD` outside
  .checkpoints/: empty). PR #40: draft, mergeable false, "dirty".
- `git diff --name-status f5030a3 origin/main`: 19 paths, 3 A and 16 M, +2,174/-99, as stated. It includes
  docs/VISION_V2.md (a decision doc; main's reviewed text, not touched by this branch).
- Guard: the full pathspec on `origin/main...HEAD` prints nothing. On `HEAD...origin/main` it prints only
  .github/workflows/toolchain.yml. routine-guard.mjs:1342-1352 reads the branch side only and needs the merge alone,
  after a fetch in its own call. So the plan's claim holds.
- merge-tree (tree 8633cf7 now; its STATUS.md blob equals f4dce49's, 9c45815): one conflict, docs/STATUS.md. Outside
  spikes/, .checkpoints/ and docs/ it equals main (exit 0). docs/ differs from main in the six names only.
- STATUS.md: the branch changed only the #40 row since f5030a3. Main's #40 row is the base's. Ours = #40 and #61;
  theirs = #40, #64 and "#63, #62". The rule keeps both sides' words with #61 out. H0 vs main is 39/39, so 38/38
  after the swap (main's own is 38/38). Main's file has 120 lines. No other marker-like line exists in docs, crates,
  packages, .github or .gitignore.
- The auto-merged docs. DECISIONS.md: patch-id equal (f0822ab), so every word of his is as main has it.
  STATUS_ARCHIVE.md and REPO_MAP.md: the same +/- lines, with only the hunk context moved. The archive's two
  insertions land under "Open bugs and risks" and "Health". REPO_MAP's three land in the root table, the docs/
  table, and after the tools/ table.
- .gitignore `dist-*/`: no tracked path matches. No dist-* folder exists on disk outside .git, .gate and node_modules.
  The wip patches create no path under one ("dist-accept" is a word; r1-dist.sha256 is a file). The light run's
  hashes cover spikes/netcode-proof only (main-run.sh:42).
- toolchain.yml (+97): on push and pull_request, contents read. The new steps read packages/ and the root workspaces
  only. No step reads spikes/ or .checkpoints/. Main's code has no `spikes/` string. Main changes no manifest, ADR,
  .claude/ or tools/ file, so DEP-HOLD holds nothing, the marks are as before and there are 33 checkpoint tests.
- CI on 1ec6f6c: gate, smoke and toolchain check runs, all success, 10:15:38-10:17:40 UTC. The plan's 10:15:35-:41 are
  the runs' bounds (not material).
- Repo map: 170 entries on HEAD (matches the tool's "170 / 110" now). Merged REPO_MAP: 173 by the tool's regex. The
  new files fall under existing required entries (docs/reports/, packages/).
- The netcode seat: its notes are tracked and rewritten by Write (u6c-review-netcode.md:7). It runs cargo through
  u6c-run.sh (rev-netcode-1.log), which takes the shared lock. No cargo is running now. Free space 9,810 MiB.
- Citations: MAIN-52 (history.md:2055-2057), MAIN-61's record (:6955-6993), NPT-TRIM (:7561-7593): all as cited.

## 4. What I did not check
- I ran no merge, gate, light run or checkpoint test, and checked no resolved STATUS.md file. The 38/38 figure and the
  "110 required" are worked out, not run.
- I did not read the netcode seat's brief beyond its notes file, or what it reads from the main folder.
- CI on the pushed head: that is step 7.

DONE
