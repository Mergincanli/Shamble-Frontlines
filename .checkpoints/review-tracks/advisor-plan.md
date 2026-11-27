# Advisor plan review: review tracks and who merges (risk tier 2)

Reviewed: .gate/review-tracks/plan.md (487 lines, 2026-11-26 18:50), inputs-A.md, inputs-B.md, checkpoint.md.
Rules used: main's .claude/agents/advisor.md and CLAUDE.md (working copy equals origin/main 2671595f for CLAUDE.md and .claude/).
His words are named by code and DECISIONS line; only the plan and the kit are quoted.

**Verdict: REVISE** (4 High, 10 Medium, 5 Low). Every fix is plan text. None needs a new decision from him.

## Plan-review checklist (advisor.md, Plan review)
- Queue and vision: SPEED-ORDER (b) (DECISIONS:183-184) puts it here. No v2 engine feature is added.
- Risk tier 2 is named. The plan reviewers are security-auditor and the advisor agent (his item 5, DECISIONS:154). The build reviewers are tier 2's pair plus test-engineer and the verifier.
- Files are listed (CLAUDE.md, advisor.md, routine-guard.mjs and its test, settings.json, new track.mjs and track.test.mjs). Rule text is shown OLD/NEW, with gaps (M1).
- The evidence matches a tier-2 kit change: script tests, guard tests at main's counts plus new cases, replay on ten PRs, dry run, T-MARK.
- No Brave checklist is needed: nothing a player sees.
- Checkpoint: held in .gate/review-tracks/ rather than on the branch, so the lane branch does not move before #77 merges. This is a stated, temporary departure from CHECKPOINTS §1. Fine until #77 merges; then it moves to `.checkpoints/` on the branch.

## What I checked myself
- OLD quotes against the files, all exact:
  - CLAUDE.md:62, :83, :92-94, :103 (read whole), :107, :127-133;
  - advisor.md:37-45, :73-74, :92-104, :167-169, :178-182, :325-326;
  - routine-guard.mjs:1568 (the gate/smoke loop), :1072 (the PR Advisor runs no program), :79-100 (allow-list; DECISIONS.md is in DECISION_DOCS);
  - settings.json:75-90 (no merge_pull_request on the allow list).
- His words by line, each where the plan says: DECISIONS:115 (SOURCE-GO), :124-158 (brief), :134-135 (item 1, reply), :141 (LIGHT-DEF), :145 (EYES-KEEP), :154 (shape), :183-184 (SPEED-ORDER), :194-195, :208, :260, :430, :432, :3160 (12 Nov, inside the 15:35:17 UTC entry at :3137).
- Words-record hashes. The 1-byte SPEED-ORDER quote gives df7e70e5…0a5c (matches :183). The 17:27:17 UTC quote joined with \n gives 62 bytes and 4fa09f1d…e739 (matches).
- CI. toolchain.yml runs on every push and pull_request with no path filter, so requiring toolchain at merge is feasible.

## Not checked (and why)
- Timings: only #69's branch commits were re-derived; they agree with inputs-A's anchors. #65 and #68 are taken from inputs-A.
- Preview: 4 of 10 rows were checked from merge diffs. #76 (2671595f) and #66 (09faefa7) are docs only with DECISIONS (#66 also SPIKE_PLAN). #67 (c1da1bd2) has docs/adr. #70 (2483e9a7) has crates. The track and floor columns match, but #76 and #66 are "building chat" only if H3's broader fix finds no other quote of him needing the merge check.
- Reviewer picks: inputs-B's tables were read; pick.mjs was not run.
- Sizes: taken from the plan, not recounted.
- subscribe_pr_activity and send_later (4b): not checked.
- Whether his Advisor chat can post a PR comment: unknown. inputs-A Part 3 says #65 and #68 were merged by his Advisor thread, so it has some GitHub write access; a comment from it is not yet observed.
- Security-auditor's plan review: still running per checkpoint.md, not in the folder.

## Findings
### High
- H1 MOVED-SIGNALS-UNCHECKED: 3g moves rows 8-11 (major change, accepted regression, open security finding below High, headless doubt) to his Advisor's merge check. The new `ready` (advisor.md:92-104 NEW) admits those PRs, and 4c keeps the PR Advisor's allow-listed `ready` merges "as today". Nothing stops the routine merging such a PR (a tools/ refactor, an open Medium security finding) with no merge check. That is beyond his item 1 (DECISIONS:134-135, the merge check as the condition) and looser than today, where these are needs-mustafa. Fix: the PR Advisor merges without the merge check only a `ready` PR whose "For the merge check" list is empty. Any signal means it waits for the merge check (step 6 NEW and 4c).
- H2 FORGERY-OVERCLAIM (4a; risk 1): a forged merge-check comment is a `Mergincanli` comment naming the head, the same as a real one. The PR Advisor's after-check can catch only a missing approval or a missing check, never a forged one. Only his Advisor, comparing against its own record of the checks it gave, catches a forgery, at whatever cadence he runs board checks, not "up to about 6 h". MERGE-MARK's recommendation rests on this. Fix: say it plainly in 4a, in risk 1 and in MERGE-MARK's text.
- H3 WORDS-ORIGIN:
  - The problem: the byte count and sha256 are written by the same chat as the quote, so a match proves self-consistency, not origin. Under the plan, a matching entry reaches no one's eyes (only "other added text" goes to the merge check). Misattributed words of his could merge with neither his go nor his Advisor's look. Today every such change needs his go (advisor.md:61-64), and misrecording his words is a High on the scale.
  - Wider: advisor.md:61-62 makes any doc that quotes him a decision doc. The plan's exemption and its checks cover DECISIONS.md only. A quote of him in a report, SPIKE_PLAN.md or UNITS.md is neither a go floor nor a merge-check signal, and inside the allow-list (docs/reports, SPIKE_PLAN) the routine could merge it unseen.
  - Fix, part 1: every added DECISIONS entry, quote and header, is listed whole under "For the merge check".
  - Fix, part 2: when the entry names a saved file at a commit, track.mjs compares the quote byte for byte with `git show <commit>:<file>`.
  - Fix, part 3: any other added quote of his, a `[DECIDED]` line, or an answer to one of his codes, in any doc, is a merge-check signal (so H1 holds such a PR), or a go floor when it sets a decision.
  - Fix, part 4: the "His go keeps" text calls the hash an integrity check, not proof of origin.
- H4 WHICH-APPROVAL: row 12 "goes" only because the building chat's advisor agent runs the day-session verification (gate, sim, render, repro). The NEW merge sentence (CLAUDE.md:127-133) also accepts the routine's PR review comment as the approval for way (1). So a game-code PR, or one whose claim needs a run, could merge on a headless approval nobody reproduced. That is looser than today. An in-session verdict also leaves nothing on GitHub for the after-check. Fix:
  - for way (1), a PR with any step-2 "verify in a day session" trigger needs the in-session advisor agent's APPROVE;
  - the building chat posts that verdict on the PR (full SHA, one line of what was run) before merging.

### Medium
- M1 SWEEP: sentences that stay and contradict the NEW text have no OLD/NEW (item 5, DECISIONS:154). They are advisor.md:32-33 (authority), :64 (decision docs "always needs-mustafa"), :71 ("`ready`, yours"), :174 (headless doubt to his checklist), :265-268 (open security finding at any severity), :304-305 (revert "verify in a day session"); security-auditor.md:54; NIGHT_SHIFT.md:77 ("merges to approve"). Show OLD/NEW for each, or mark it "stays" with the reason.
- M2 CLAUDE-107-PARAPHRASE: the NEW :107 is looser than "His go keeps" item (4), which also covers ADRs and the register, hard constraints, and design or look docs beyond a pointer. A session reading CLAUDE.md alone gets the looser rule. Make :107 point to "His go keeps".
- M3 OWN-MERGE: the tier-1 column NEW sends allow-listed `ready` PRs to the PR Advisor only, so the chat that built a tools/ PR may not merge its own. That narrows his item 1 (DECISIONS:134-135, the builder chat merging its own), is not listed in section 5, and adds up to 6 h of waiting. Let way (1) cover every `ready` PR (whichever merges first), and keep H1 for the routine.
- M4 NARROWED-UNLISTED (section 5):
  - (a) "His go keeps" holds more than EYES-KEEP's list (DECISIONS:145): secrets and infrastructure, TOOL-SOURCE, stop lines, Critical or High after round 3, revert past a required check, forks. Each is stricter and rests on other words of his (A4, TOOL-SOURCE, CAP), but it narrows item 1's "everything else merges" sentence (DECISIONS:134), so list it.
  - (b) Item 9 reads A4's tier-0 auto-merge as standing, while item 7 reads newer words as winning. State why both hold: A4 is untouched, and his item 4c (DECISIONS:151) asks what the routine still does.
- M5 DOCS-NO-ROUND: "a docs-only PR gets no review round" is wider than tier 0. ADR drafts, research topics and decision docs would lose step 5's reviewer, though LIGHT-DEF (DECISIONS:141) keeps one round. Limit it to risk tier 0. Other docs get the one reviewer that fits, plus the verifier for research.
- M6 FEEL-PATHS: eyesPaths has nothing for feel or balance, though escalation category 1 includes game feel and balance. Add crates/sim/, packages/sim-worker/ and content/, with identical sims (sim.js or replay hashes) as the evidence of no change, else his check.
- M7 MODES-AND-BASE: track.mjs reads names only. Read `--raw` modes too: a symlink (120000) or submodule (160000) anywhere means full. Print the base and head SHAs in the JSON, so the chat, the advisor agent and the routine compare like with like (origin/main moves).
- M8 AFTER-CHECK-DESIGN: "since your last run without his go" is not knowable to the routine, which has no state and cannot see his go. Make it stateless:
  - every merge on main the PR Advisor did not make needs an APPROVE and a merge-check comment naming the merged head;
  - the routine marks each checked merged PR with one after-check line, and checks the unmarked ones.
- M9 4A-PASTE: say plainly that his item 4a (DECISIONS:149, no paste) holds only if his Advisor chat can post on the PR. Until T-MARK item 4 shows that, the honest answer is one paste per PR (still fewer steps than today's paste, go and tap). Measure T-MARK without manual steps from him: use comments he already typed on the web, if any, and the phone step only if MERGE-MARK (b) is chosen. Otherwise drop "For Mustafa" item 2.
- M10 NIGHT-PRS: name who merges a night-shift PR (the night shift never merges). It is the day chat on the same lane that resumes the branch, which WAKE-END decides.

### Low
- L1: "13 found, 0 missed" should read "11 Highs plus 2 candidates found, 0 missed" (inputs-B's count).
- L2: cite TOOL-SOURCE's general rule (DECISIONS:1348) beside the one-file answer (:2220). track.mjs could print changed lockfiles, manifests, toolchain pins and workflow URLs as a judged TOOL-SOURCE prompt.
- L3: the light reviewer needs a fallback when no owner row matches (for example test-engineer).
- L4: inputs-B Part 4.4's dynamic-DOM trigger for security-auditor (innerHTML and similar under packages/) is a cheap, deterministic addition. Take it or defer it in writing.
- L5: recount sizes and stop lines after these fixes, and keep the smaller version beside it if still above 150.

## Answers to the five questions
1. Coverage. 3a-g, 4a-d and item 5's shape are all addressed. The gaps are M1, M4, H2 and M9. The sizes follow CAP (a) as stated (290/580, smaller 150/300; not recounted). Every OLD quote and DECISIONS line I spot-checked (listed above) is exact.
2. Loosening: H1, H3 and H4 go beyond his words, and M2 does in CLAUDE.md's wording. On the points you asked about:
   - CLAUDE.md's merge sentence: H4.
   - Tier 1's merge column: H1 and M3.
   - Decision docs: M2, M1 (advisor.md:64) and H3 (quotes outside DECISIONS).
   - EYES-KEEP's last sentence: H3.
   - LIGHT-DEF's effort: sound. His item 3b sets the default effort for light and 3d keeps the 12 November effort for full (DECISIONS:141, :143), and the verifier stays at max.
   - PR Advisor's own merges: fine for empty-signal `ready` PRs only (H1).
   No misquotes found in the plan.
3. Design:
   - The script is deterministic: in doubt full, no merge base means exit 2, rename-safe, lower case. Add M7.
   - The go floors are sound, and honest about what stays judged.
   - Reviewer picks follow inputs-B (not re-run).
   - Merge conditions are sound once H4 is fixed.
   - MERGE-MARK's honesty needs H2.
   - The after-check needs M8.
   - The toolchain guard line is correct, and the fixture default needs the toolchain row.
4. Decisions: ten is too many. Fold four into PLAN-GO as defaults his words already answer, keep four as his, drop two (list below).
5. Missing:
   - SOURCE-GO lands well: item 5, category 5, a go floor, and content/ is full because it is in no list.
   - TOOL-SOURCE: L2.
   - Night shift: M10.
   - CHECKPOINTS §9: fine as written (fold before the head review).
   - Security-auditor's plan review must be in, and its findings answered, before he is asked (his item 5).

## Proposed decision list for him (4 instead of 10)
- PLAN-GO: (a) full, about 290 lines; (b) smaller, about 150; (c) revise. Recommend (a).
  - (a) carries these defaults, which his own words answer. He can override any by naming it:
    - TRACK-DOUBT (a): his item 3a, in doubt full (DECISIONS:140);
    - REVIEWER-SLOTS (a): his item 3d leaves the rule to the plan (DECISIONS:143);
    - LIGHT-EFFORT (a): his item 3b, default effort (DECISIONS:141), with the verifier at max;
    - GUARD-TOOLCHAIN (a): his item 3e names toolchain green (DECISIONS:144).
  - Under (b), the smaller version must still keep the fixes for H1-H4 and M8 in prose: no hash check, so every DECISIONS entry goes to the merge check (H3); the in-session approval (H4); a stateless after-check by his Advisor only. It drops only the script's reviewer picks, the hash check and toolchain in the guard (so GUARD-TOOLCHAIN becomes (b) under it).
- MERGE-MARK, restated per H2: (a) a written rule plus his Advisor's comparison, which is his brief's own proposal (item 4a, DECISIONS:149) and the only thing that catches a forgery; (b) a guard-enforced unmarked comment, after T-MARK. Recommend (a).
- MERGE-PROMPT: (a) put the merge tool on the allow list; (b) keep the prompt. The allow line also removes the prompt on merges that need his go, and the guard checks neither his go nor the approval. Recommend (a) if security-auditor's plan review finds no blocker, else (b).
- WAKE-END: the plan's own section 5 item 8 calls (b) a widening of his item 1 (DECISIONS:134-135, the builder chat merging its own), so it is his. Recommend (b), limited to a chat on the same lane that re-checks every condition. (a) is the default if he does not answer.
- Settled here, not asked:
  - PRADV-TRACK (a), as a technical choice inside his item 3a. The routine's reading of the path table is a cross-check only and never sets the track: the script's output, run by the building chat and re-run by the advisor agent, sets it, and a mismatch is a finding.
- Dropped:
  - ACCT-LINES: outside his brief, and already recorded as a follow-up (the 17:11:23 UTC entry header, DECISIONS:192). Ask it in a later batch.

---

# Re-review of the revised plan (plan.md, 670 lines, 2026-11-26 19:20)

**Verdict: APPROVE, on conditions C1 and C2, before the plan goes to Mustafa.**
- C1 is one paragraph of plan text. C2 is security-auditor's read of the new guard parts.
- C3 and C4 may be carried out at build.
- No new advisor plan review is needed unless C2 changes the design.

## Checked myself
- OLD quotes in the revised sweep match main exactly: advisor.md:33-34, :63-64, :174, :200, :266-268, :304-305; security-auditor.md:54. CLAUDE.md and .claude/ equal origin/main.
- My conditions:
  - H1: met (3e "The PR Advisor's own merges"; advisor.md :92-104, :200, Never).
  - H2: met (4a, 4d, risk 1, after-check text: "never a forged one").
  - H3: met. Integrity, not origin; each entry's sha256 is named in the check; quotes outside DECISIONS are listed; an entry the check does not name needs his go.
  - H4: met (way (1) needs the in-session APPROVE, posted).
  - M1-M7, M9 and M10: met.
  - M8: met by another route (see Departures).
  - L1-L5: met.
- Section 5 lists every narrowing, including the kit rule on security findings (item 13) and the light security trigger (item 14).
- No quote of his is misquoted. The quotes in the decisions (DECISIONS:140, :141, :143, :144) match.

## Departures: all acceptable
- SA's OLD/NEW texts are in the plan's own wording. Fine, provided security-auditor confirms them (C2).
- The after-check uses a fixed 24-hour look-back, not marks. Fine; risk 10 is honest. See C4.
- The guard reads the track table from disk. Same trust as the guard file, and the protected-path test runs first. See C3.
- The prompt skip reads file names only, and content floors go to his Advisor's PASS. Fine: the PASS is the condition his item 3e names, and 3e says so plainly.
- WAKE-END, narrowed (b) with (a) as the default. Fine, and correctly left to him.

## Conditions
- **C1 (High; a rule loosened if left): the PASS line must mean way (1) only.**
  - The conflict: the plan keeps DECISIONS:260 (his Advisor's merge check comes before his go) for PRs that keep his go (3g, "Four newer rules"). But 3e's allow path, and way (1) itself, treat any exact PASS comment as the last condition.
  - The risk: if his Advisor posts the same line on a PR that keeps his go, the guard skips the prompt whenever no protected, floor or eyes path is named. A judged floor (a visible change outside the eyes paths, a decision set in a light file) could then merge with no go and no tap.
  - Fix, in 3e, 4a, "Four newer rules" and the CLAUDE.md NEW sentence: his Advisor writes `Advisor merge check: PASS …` only when it finds nothing in "His go keeps". For a PR that keeps his go, its check uses a first line the guard and way (1) never accept (for example `Advisor merge check: FOR HIS GO #<pr> <full sha>`), or stays in his chat as today.
- **C2 (process, not a round): security-auditor reads what is new since its review and confirms SA-RT-1 to SA-RT-12 are met in the plan's wording, before he is asked.**
  - What is new: GUARD-MARK, MERGE-PROMPT (c)'s allow path, the allow output in guard.mjs, the disk-table read, the subagent-merge refusal.
  - Why: (c) is the most security-sensitive part, and it postdates SA's review. A risk-tier-2 change needs security-auditor on what it actually proposes.
- **C3 (Low, at build):** the allow path stays silent (that is, prompts) when the disk copy of track.mjs differs from origin/main's blob. A cheap fail-safe for the disk-table departure: a stale lane branch then never decides an allow.
- **C4 (Low, at build):** the after-check counts a merge as the routine's own only when its merge note names the merged head and every changed file passes the allow-list. Otherwise it checks the merge. Merge notes are comments any session can imitate.

## Decisions for him: three, not four
- Fold MERGE-MARK (a) into PLAN-GO's defaults. It is his brief's own approach (item 4a, DECISIONS:149) plus the guard's hardening. (b) is a later follow-up gated on T-MARK's data, asked only then.
- Keep MERGE-PROMPT (recommend (c), after C2) and WAKE-END (recommend narrowed (b); (a) by default). Both are self-contained, and both recommendations are sound.
- PLAN-GO's (b) text correctly says MERGE-PROMPT is moot under it.

---

# Quick look: MERGE-PROMPT (d) update (plan.md, 705 lines, 2026-11-26 21:10). Not a review round.

**Verdict: (d) is recorded correctly, and nothing is loosened beyond it. Ready to build once #77 has merged, on conditions Q1-Q3. Q4 is a recommendation for security-auditor.**

## Checked
- **His answers.** Only his typed strings are quoted: "PLAN-GO: a / MERGE-PROMPT: A / WAKE-END: a", then "d". The text of (d) is labelled as offered to him, not his words (plan :25, :487, :671-674). That is the right form.
  - Not verified: his messages themselves. They are in the main session's chat, and no saved copy exists yet (Q3).
- **(d) as built.** The plan carries the four guard checks:
  - the expected head is the current full head;
  - gate, smoke and toolchain are green;
  - the caller is not a subagent;
  - an exact, unedited PASS from `Mergincanli`, posted after the gate run.
  There is no file-list check. A guard that fails to load, or an error in the new checks, ends in the prompt. A failed base check stays a refusal.
- **"His go keeps" is not dropped.** It is routed through PASS: his Advisor writes PASS only after his Brave check or his yes (3e, "What PASS means"; the CLAUDE.md NEW sentence; section 5, items 1-4).
- **Honesty.** Risk 1 states the lost second layer plainly, including that nothing technical catches a real PASS written in error. GUARD-MARK, H1-H4 and the PR Advisor's own merge limits are unchanged. C1 is correctly marked superseded, and FOR HIS GO remains only as a status line that is never accepted.
- **Misquotes:** none found.

## Conditions before the build
- **Q1: security-auditor reads the (d) update.** This is a tier-2 rule, and the plan's own status line says so (:703-705). Security-auditor's APPROVE leaned on the file-list and protected-path test that (d) removes.
- **Q2 (Medium, text only): give advisor.md:205-208 an OLD/NEW.**
  - Today that sentence says his go counts only in the merging session's chat, and never through a go a session relays.
  - Under (d), his go reaches a way-(1) merge through his Advisor's PASS, which is a relay he chose.
  - Section 5 item 11 says "unchanged for way (2)", but the advisor.md text itself would still contradict CLAUDE.md's way (1). Scope it to way (2) and the routine, and add one clause: under way (1) his go reaches the merge only through his Advisor's PASS (his MERGE-PROMPT (d)), never through text a building session writes.
- **Q3: save his answers by script before the build.** That is the three messages, plus the option text of (d) as shown to him, in `.checkpoints/review-tracks/`. The docs pass can then record them in DECISIONS with byte count, sha256 and the saved-file compare that the plan's own track.mjs will check.

## Recommendation for security-auditor (not a condition)
- **Q4:** (d) raises the stakes of a forged PASS: a forged one could now merge a rule-file or visible PR. The one open route is Bash: `gh pr comment` and `gh issue comment` meet only a prompt, or Auto mode's classifier.
  - Deny them in settings.json. Sessions comment through the GitHub tool, where GUARD-MARK scans every write.
  - This costs one more ask at build. It closes the plain Bash route; the disguised forms stay open until R4-1's fix.
  - Security-auditor rules on it at its (d) read.

---

# Pre-apply review (build copy .gate/review-tracks/build/, 2026-11-27)

**Verdict: REVISE** (4 Medium, 7 Low; no Critical or High). Fixes are text in the copy; re-check only the changed hunks.
- Matches the plan: CLAUDE.md :62, :83, :92-94, :103, :107, Review tracks bullet, merge sentence and rest of paragraph;
  advisor.md :33-34, cat. 5, :63-64, :71-74, classes, "His go keeps", plan-review line, evidence sentence, :167-169,
  :174, :178-184, step 5 item, :200, After-check, merge check counts, kit tests line, :266-268, :304-305, Never;
  security-auditor.md:54; guard: GUARD-MARK (MCP + SA-RT3-1 gh forms), subagent merge, toolchain, passAllows (d), allow
  output day-PreToolUse only. Nothing loosened beyond his words; no misquote.
- Size: 585 added (39 CLAUDE.md + 546 kit), stop line about 980.
- Suites in copy: guard 1575 (main 1551), track 61, checkpoint 33; settings S4 fails in copy and in basecopy alike
  (copy is not a git main folder), 127/127 on the real repo.
- PA-M1 advisor.md "Mustafa's go": first sentence still unscoped (Q2 half met).
- PA-M2 advisor.md step 5: no "For the merge check" slot in the comment, though H1's routine merge depends on it.
- PA-M3 track.mjs plan gaps: changed or removed existing DECISIONS entries not flagged; other added DECISIONS text not
  listed; sha256 missing from forMergeCheck; XMLHttpRequest missing from the triggers; V2_PHASE0_BRIEF missing from (4).
- PA-M4 track.mjs case-twin: missed when the changed path sorts after its twin (main() :144).
- PA-L1 gh -F attached form (-Fnote.md) passes the body-file refusal (falls to a prompt).
- PA-L2 no test for the paginated comment read (plan evidence lists a second-page PASS).
- PA-L3 advisor.md Never and step 2 still say gate and smoke; the guard now needs toolchain too.
- PA-L4 advisor.md "Steps 5 and 6": a subagent's merge is now refused; say so.
- PA-L5 internal id "(H1)" in advisor.md rule text.
- PA-L6 GUARD-MARK will refuse this change-set's own MCP PR body or report if it quotes the line (risk 10).
- PA-L7 SA-RT2-1 guard-file half: not built; stays for security-auditor's build review.
- PA-M5 passAllows splits the body on \n only: a web-typed multi-line PASS (CRLF) never matches, so the allow would
  never fire on his paste (fail-safe: prompt). Not observed: PRs #65-#77 carry no web-typed comment (all app `claude`);
  the real --paginate/@json read parses (u, t, c, m, b) on #66, #67, #70, #77. Fix touches SA-RT2-3 wording: SA rules.
- Departures (accepted, record in the report): merge_pull_request outside the GUARD-MARK scan; security triggers on every
  path; "no saved file named" is a go floor; SA-RT3-1 built (Bash plain forms now refused); "Main's" and "as above".
- No case-only twins on origin/main (378 paths). main()'s git half is covered by exit-code tests only.
- Groups: restores the plan PA-M1, M3, M4, L1, L2; new rule text to show him as additions PA-M2, M5, L3, L4, L5.

---

# Re-check of the fix pass (changed hunks only, 2026-11-27). Not a new review round.

**Verdict: REVISE, track.mjs only** (1 High, 2 Medium, 2 Low, 1 for security-auditor to rate). The rest of the fix
pass is approved: advisor.md, guard.mjs, routine-guard.mjs and both test files. This is the last fix pass under the
budget: after it, only a Critical or High reopens.
- Run myself, in the copy: track 82/82, routine-guard 1581/1581, checkpoint 33/33, kept-logs 9 pass (10 tests, 0 fail).
  Skimmed the guard test diff: paginate+filter fake, CRLF allow, lone CR, Bot, two gate runs, plugin_ server, Miro,
  mutants. Not run: fix-mutate, fix-hist, settings S4 (copy).
- Met: PA-M1, PA-M2, PA-M4 (TE-B1 form), PA-M5, PA-L1..L5; SA-PA-1..9 as described; TE-B9 and SA-PA-8 text present;
  no internal ids in rule text. PA-M3: not met in full (RC-H1, RC-M1).
- RC-H1 (High) the saved-file pattern needs ")" right after the commit; real entries write "(readable at commit <sha>
  of the chat's branch `...`)". All 5 "no saved file named" results of the 20-entry run do name a saved file
  (90efc3d2, 809fbcd0, d736715d, 83ce083b, 7d208dc5; 247437b9 too). The compare is skipped and the result is wrong.
  Both commits are reachable from origin/main. Fix the pattern, test it with those real entries, re-run the 20.
- RC-M1 (Medium) core() keeps only an entry's first line, but headers wrap. Changing an old entry's stated sha256 or
  saved-file commit gives no floor (reproduced on main's DECISIONS). otherAdded() is true for every newly added wrapped
  entry (2542b1a0, 89075745), so that merge-check line is always on. core() came from my own pre-apply review. Fix:
  header = lines up to the first quote line, bounded at a blank line or heading; same core for otherAdded(); replay on
  the fix-hist PRs and the 20 entries, with no new false floors.
- RC-M2 (Medium) twins(): a folder-only twin (docs/x/a.md beside docs/X/b.md) is not caught. Fix: floor only the changed
  path whose folder prefix has a lower-case twin in the head's tree; never pull a sibling folder into the files.
- RC-L1 (Low) ignoredByBase() also reads the local info/exclude and core.excludesFile (scratch repo: two extra floors).
  It only adds floors, but the "same under any config" claim is off.
- RC-L2 (Low) numstat varies with diff.algorithm (2 of the last 15 main merges, by 2-4 lines): pin it.
- RC-S1 (security-auditor rates; outside the changed hunks) a consistency gap in gh(); detail to the caller only.
- Twins ruling: TE-B1's form is accepted. The floor concerns the change's own files; my PA-M4 version was overbroad.
- Binary floors (#68, #70): information for him in the report, not a decision.
- Size 931 vs about 980: fine. The fixes must stay under it, or it is his call (His go keeps, 6).
- ADDITIONS-OK stays the one decision. Its list grows by SA-PA-8, TE-B9 and the guard widenings (stricter), plus
  WORDS-NEWLINE: a second accepted quote form, not in plan 3f's letter, named as a tolerance (it proves the same words).

---

# Re-check of the second fix pass (changed hunks only, 2026-11-27). Not a new review round.

**Verdict: REVISE, one guard hunk** (plus one Medium riding the same pass); the rest approved.
- Run myself in the copy: track 85/85, routine-guard 1581/1581, checkpoint 33/33, kept-logs 9 pass (10 tests, 0 fail);
  fix-words20 999: 44 ok / 65 / 52 / 1 (as stated); fix-hist: 11/11 same, #65 stops, floors as stated; guard probe.
- Met: RC-H1, RC-M2, RC-L1, RC-L2 (pinned; mutant survives: Low), SA-RC-1..4, SA-RC-6, advisor.md gloss (matches the
  guard's first-line match). RC-M1: met for headers; see RC2-M1.
- RC2-C1 (Critical by class, same class as SA-RC-5, ends at a prompt): SA-RC-5 only partly met. Detail to the caller.
- RC2-M1 (Medium): existing-entry floor misses a removed quote line after a left-out note (2 entries on main). Rides
  the pass.
- RC2-L1 (Low): diff-algorithm pin untested. Follow-up.
- Third saved-file shape: accepted as a named tolerance in ADDITIONS-OK (24 saved files hold a "---" line).
- Size: 1003 plus the last fix, past about 980: SIZE-LINE to him, recommend (a) go as is.
- Not run: settings S4 (copy), security-auditor's probes (its own re-check).

---

# Re-check of the third fix pass (changed hunks only, 2026-11-27). Not a new review round.

**Verdict: APPROVE of these hunks; the copy is ready for the applies.**
- Run myself in the copy: routine-guard 1581/1581, track 85/85, checkpoint 33/33, kept-logs 9 pass (10 tests, 0 fail).
  Read mcp(), kept()/parts(), the saved-file header regex, git()/tg() env and the usage line; new tests at
  routine-guard.test.mjs 3917-3925, 4032-4033, mutants 4062-4063; track.test.mjs 163, 191-192. Not run: fix-mutate,
  fix-words20, fix-hist, sa-recheck7/8 (taken from the caller and security-auditor).
- RC2-C1: met. Merge and auto-merge refused on every server but exact `github` (case too); subagent merge refused on
  any connector before mergeCheck; `/github/i` holds only the comment-edit check.
- RC2-M1: met. kept() = header + every `>` line; quoteOf/new-entry/otherAdded unchanged (first run).
- SA-RC-8: met. Header slice before the first "\n---\n" must be exactly "# title, blank, _Copied by script..._"; JS `$`
  without m is end of input; tail-only quote floors.
- SA-RC-7: met. GIT_GRAFT_FILE=/dev/null in git(); tg() runs in a history-less temp repo (no graft reach). Usage line ok.
- RC3-L1 (Low, follow-up, no push): the comment-edit check keys on /github/i, so a GitHub server under a name without
  "github" edits a comment unchecked in a day session (routines refuse other servers). Fail-safe side: an edited PASS
  no longer counts. MERGE-SCOPE class.
- Open with him: SIZE-LINE (1019 vs about 980, recommend (a)), ADDITIONS-OK.
