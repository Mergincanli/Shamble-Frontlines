# Plan: main-wall (MAIN-WALL) · risk tier 2 (repository settings) · for the advisor agent, then Mustafa's go
Base: main 4a62873e. Branch: claude/confident-gates-tbnq38 (stands in for cs/main-wall). No code change: GitHub
settings by his clicks, a measurement on a throwaway repository, then docs. The gap's detail stays out of this file
(his 20:35 item 3): the design with its test detail, the test chat's brief and his test card are in `.gate/main-wall/`
(gitignored; copies in the session scratchpad); none is pushed. Sources read: the other lane's notes at 7aadfc17
(claude/optimistic-albattani-mdoonz; the branch has moved since).

## 1. Goal (his words, 2026-11-25 20:35, item 2, as recorded in DECISIONS)
A rule that only his account can update main, measured on a throwaway repository first. Day sessions, the PR Advisor
and his revert route must still merge. His account must not skip gate and smoke. Plan first, his clicks listed once.
The measurement runs in a separate chat holding only the throwaway repository, or by his clicks; never a second
repository in this chat (his 13:20 item 1, 15:05 item 6).

## 2. The rule (infra-engineer's candidate W1; sources: GitHub's ruleset docs, cited in the design)
- A second ruleset on main, `main-wall (only the owner updates main)`: Active, target "Include default branch", ONE
  rule, "Restrict updates". Its bypass list holds one entry, the "Repository admin" role: on a personal repository the
  owner, Mergincanli, is the only admin. "Restrict deletions" and "Block force pushes" are unticked here (the form
  ticks them by default): they stay in today's ruleset.
- Today's ruleset, "Shamble Ruleset 1" (pull request, gate, smoke, no deletion, no force push), stays exactly as it
  is, with its bypass list empty. GitHub applies every ruleset on a branch together, so gate and smoke stay required
  for every merge, his own included. Whether a bypass in one ruleset leaves the other in force is NOT in the docs: the
  first test decides it.
- Bypass mode, chosen by the measurement: "Exempt" preferred (his merges look as today, no bypass tick; cost: no
  bypass audit entry for his own merges); "For pull requests only" as the fallback, only if the tick it brings cannot
  pass a merge with red or pending checks; neither: stop and back to him.
- Rejected: one combined ruleset (its bypass would cover gate and smoke); classic "Restrict who can push" (offered on
  organization repositories only); the Claude app on the bypass list (not "only his account").
- What it cannot tell apart, said plainly: every Claude session acts as his account (through the Claude app), and so
  does anything else he authorises as himself. The wall separates his account from everything else; separating his
  own decision from a session's act stays with the guard, the pull-request review and his go.

## 3. The measurement (throwaway repository `wall-throwaway`, public, his account)
Run by a separate test chat that holds only `wall-throwaway` (opened by him, from the brief; one test on each of his
Claude accounts) and by his own clicks with screenshots. Results reach this chat by his paste.
Tests, ids only here (detail in the brief): T0 both rulesets read back; T1 isolation, decisive and first: a pull
request with red checks must not merge by any route, his web merge included (fail = the design is dead, no click on
the real repository); T2 session merges from both Claude accounts; T3 his web merge waits for gate and smoke; T4-T6
writes that are not his account are refused (T6 optional); T7 the revert route; T8 the audit record; T9 the default
branch stays; T11 optional, the PR Advisor's harness on the throwaway. Each refusal counts only when controls show it
came from the wall. Then the same set again for the fallback mode, only if needed.
T10 runs on the real repository after his click there: the next day-session merge and the next PR Advisor merge.

## 4. His clicks, listed once (each change with its undo; screen names to confirm on screen)
A. Throwaway repository:
1. Create `wall-throwaway` (public, with a README). Undo: Settings > Danger Zone > Delete.
2. Give the Claude app access to it (github.com Settings > Applications > Installed GitHub Apps > Claude > Configure).
   Undo: remove it there.
3. Open the test chat (a new Claude Code chat holding only `wall-throwaway`) and paste the brief; later the same on
   his other Claude account for T2. Undo: archive the chats.
4. Copy today's ruleset: real repository > Settings > Rules > Rulesets > "Shamble Ruleset 1" > Export ruleset (a
   download only); in `wall-throwaway` > New ruleset > Import > check it > Create. Undo: delete it there.
5. Add the wall in `wall-throwaway` (section 2's settings, mode Exempt; if "Exempt" or "Repository admin" is not
   offered: note it and tell the test chat, or stop). Undo: delete it, or Enforcement status Disabled.
6. His tests from his test card (handed to him in chat with the brief, not pushed: steps, expected result and stop
   line for T1a in each mode, T3a-c, T6, T7 and T9): one screenshot each.
7. Only if needed: switch the wall to "For pull requests only"; re-run. Undo: set it back.
8. Optional T11: a test routine pointed at `wall-throwaway` only. Undo: delete it.
9. Paste the test chat's report and his screenshots here. Keep `wall-throwaway` until T10 passed, then delete it and
   remove the app's access.
B. Real repository (only after the measurement passed, the advisor agent reviewed the results, and his go):
10. Read and screenshot, changing nothing: Deploy keys, Collaborators, Installed GitHub Apps and the Claude app's
    "Administration" level.
11. Add the wall exactly as click 5, in the chosen mode. Do not open or edit "Shamble Ruleset 1". Undo (the rollback,
    one settings switch, needs no merge, so a wrong wall cannot trap main): the wall > Enforcement status: Disabled.
12. A screenshot of both rulesets; then T10 (this chat's next merge and the PR Advisor's next merge). If a merge is
    refused: click 11's undo.

## 5. Evidence, reviews, docs, size
- Evidence: the test chat's report (merge SHAs, run links, the rule record) and his screenshots, read against the
  expected results; the real repository's screenshots and T10's two merges. This chat does not read rulesets: its
  `gh` is for search only (the session's rule), so T10's read-back is his screenshot of both rulesets (click 12) and
  click 4's fallback is his setting the boxes by hand from that screenshot. The design's two read-only `gh api` reads
  (the repository and ruleset 25069670, by infra-engineer) were a departure from that rule; the report records it.
- Reviews: the advisor agent on this plan, on the measurement results before click B, and on every pushed head;
  security-auditor on the design, the brief and his test card BEFORE anything goes to him, and on the results before
  click 11, each finding addressed or deferred in writing (tier 2); infra-engineer is the designer.
- Docs (one small pull request after click 12, which is also T10's day-session merge): STATUS, the archive, ROADMAP,
  and a short "main's wall" note in TECHNICAL (what each ruleset does, the rollback click, what it cannot tell apart);
  guard-wrappers' follow-ups from the advisor agent's final-head review of #73 ride in it, all docs only: F2 (STATUS
  and the archive say "no pull request yet"), F3 (the report's base line and one red count), F4 (a count word in
  DECISIONS), F7 (the report's "never run live" line); F5 (the pull request text's footer) needs no file change; F6 was
  accepted. No code, no workflow on the real repository, zero CI minutes. STATUS marks T10's PR Advisor half open
  until its first merge.
- Change-sets: two, as he asked (20:35): MAIN-WALL alone, and the rule-file plan (the review tracks and who merges,
  with SOURCE-GO (a)) as its own change-set later; the wall is built first, and the review tracks' merge-on-green rule
  takes effect only after click 11 and T10 have passed. Between them, in his order (15:05 item 4): one guard
  change-set with R4-1's fix and GUARD-KIT (its parts ride there, as his 15:05 order replaced his 13:20 item 3). The
  other lane's token sitting waits for click 11 and T10.
- R4-1 and this order (his 15:05 item 4): security-auditor was asked on 2026-11-26 and answered that R4-1 can wait
  behind MAIN-WALL (branch protection, the pull-request review and his go keep it off main); so nothing goes to him
  before the measurement on that point.
- The test chat runs without the guard: the throwaway repository has no repository hooks, so its only limits are the
  brief's words and its binding to one repository.

## 6. Risks (ranked; full list in the design)
1. High, stop-gated: a bypass in the wall reaches today's ruleset, so his account and every session could merge
   without gate and smoke. Control: T1 and T3 first; a failure stops the plan.
2. High, by nature: the wall cannot tell a session from him (section 2). Control: say so in STATUS and TECHNICAL.
3. Medium: the PR Advisor's runner differs from a day chat. Control: T11 or T10; if refused, it reports, a day
   session merges on his go, or he disables the wall.
4. Medium: the wall does not protect itself; his account can change any ruleset. Control: click 10 records the Claude
   app's Administration level; a day session asks him for a ruleset screenshot at the start of any kit, CI or gate
   change-set.
5. Medium (Exempt only): no bypass audit entry for his merges; each pull request's history still shows them.
6. Low: GitHub changes rulesets (it did in 2025-09 and 2026-05). Control: T0's read-back is cheap to repeat.
What it does not cover: who acts inside his account; what gate and smoke check; branches other than main; settings,
secrets and environments; the preview token (its own environment protection; the wall is its precondition).
