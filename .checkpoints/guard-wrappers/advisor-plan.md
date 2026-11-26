# advisor: plan review of guard-wrappers (names no form, no program and no finding detail: those stay in the handback)
- Read the public plan (57df2b8), the scratchpad detail, the guard, the test file's rows and mutant anchors, the hook
  entries in settings.json and the CLI version; reran the row scan (758 strings, 8 hits, as the plan states).
- Verdict: APPROVE with conditions. The findings and conditions are in the handback only; none is copied here until
  the fix is merged.

## Plan-change ruling (round 1's fix pass, before the copy goes over the real guard)
- Read the copy's guard diff against the real one, plan §8, and the new rows and mutant. Verdict: APPROVE with
  conditions (in the handback only). SA-D-DIRECT pre-ruled: approved if Mustafa says yes. No form is named here.

## Severity ruling on round 2 (no form named)
- R2-1 raised from Medium to Critical (reason in the handback: an agent's kit write passes with no refusal, the same
  class as SEC-U7B-1 and SA-d). Round 3 on what changed allowed. SA-g refinement may ride; R2-2 may not.

## Round 4 plan change (2026-11-26; on head 4002493, the copy hooks-r4b; no form named here)
Checked myself (function calls through the harness only, nothing run live): the copy's suite 1532/1532; red 29 with
9ad6bc9's guard and red 15 with hooks-r4's guard, both under the copy's tests; size against main c1da1bd guard +46/-14,
tests +103/-3 = 149 added (stop line 200); guard.mjs equal to the real one; the real guard equals 9ad6bc9; advisor.md
equal on branch and main. Differential sweep, 151,848 lines, session and agent: 9ad6bc9 to hooks-r4b only stricter, no
throw; against main, 6 forms deny to pass and 85 session deny to ask, all older than round 4 (ADV-R4-2). Every round-3
case file of security-auditor: ask in a session and refused for an agent on hooks-r4b. hooks-r4 reopened R2-1
(confirmed on the probe); hooks-r4b closes it. Every remaining agent-silent line with xargs in the sweep ends in an
exact read-only name with no replace string in its command words.

1. hooks-r4b's unwrap() change: APPROVE. It implements his R3-WAY (a) for every xargs the guard strips and closes
   R3-1 and R3-2; nothing looser than 9ad6bc9; R2-1 closed again. Condition: it is applied only together with item 2's
   addition, as one rebuilt copy.
2. The boundary: REVISE, split in two.
   (a) ADV-R4-1, Critical, open, older than this change-set: a spelling of xargs that hooks-r4b leaves silent for a
       session and an agent (detail in the handback only). It is the word in his line (a), so his line decides it, not
       round 4: close it in this build, ask-only (agents refused), with its rows, a silent control and a mutant.
       Rebuild as a new copy; red on hooks-r4b, green there; the differential rerun with xargs and the installed
       programs in its word lists (the sweep above missed this class for want of them), no line looser than hooks-r4b;
       the plan states whether the addition also asks when a read-only program follows (stricter than his sentence,
       never looser: allowed, but written down); my review of that copy before anything is applied (a plan-change
       review, not a round). Stays under the stop line. Silent on main too (probed), so older than this change-set.
   (b) INPUT-RUN, the rest of the scratchpad's boundary line (programs outside the guard's wrapper table that run
       their input): the named limit, L1's class (scripts and inline code are not inspected), a follow-up for the next
       kit change-set. Stated in class words only in the report and STATUS; listed as a security finding left open, so
       his go covers it knowingly.
3. Hunk order: APPROVE with conditions; the hunk list and order are confirmed at my review of the rebuilt copy (item
   2a's addition lies outside unwrap()). (1) Before the first Edit, the real guard and test file still equal
   9ad6bc9's. (2) Every Edit leaves the guard loadable and consistent: a name declared in one hunk and used in another
   goes in the same Edit (so unwrap()'s changed lines are one Edit), new text byte for byte from the approved copy;
   a throw fails open in a day session (read in guard.mjs and the settings bootstrap). (3) The comment Low (ADV-R4-3)
   rides as its own comment-only Edit. (4) Then the test file in a few Edits; each Edit asked of Mustafa with its
   exact text. (5) After the last Edit both real files byte-equal the copy (cmp), then the kit's three test files,
   then the gate. (6) No commit between the first guard Edit and the last test Edit; guard and tests in ONE commit
   (the saver stages only .checkpoints/, read in its script).
4. Main moved: REVISE as proposed. A fast-forward of the main folder to a SHA built in a worktree is not one of the
   three main-folder moves CLAUDE.md's Never line allows, and the advisor cannot widen it. Approved instead: the main
   folder does not move; the merge of origin/main, docs-keeper's edits, the report, the checkpoint deletion and the
   push are all done from one separate worktree. Checks on that head: against origin/main the kit, CI and gate paths
   differ only by the two hook files; the hook files are byte-identical to round 4's reviewed head; #71 touched no
   .claude/ or gate file (checked: git diff --raw c1da1bd origin/main). Then my review of that pushed head, his
   Advisor's merge check, then his go on that full head id. Snag: the lane branch is checked out in the main folder,
   so that worktree is detached, and the checkpoint saver refuses a detached head: stop the watcher, commit plainly
   there, push to the lane branch by name. Whether the guard lets that push through from the worktree is unverified;
   a refusal there means ESCALATE. If the session prefers the one fast-forward, ESCALATE that single command instead.
5. Queued notes: APPROVE with conditions. (1) The report names the full 40-character SHA of the last commit holding
   the notes and their paths, checked with git show after the deletion is pushed. (2) STATUS gets one follow-up
   pointer (MAIN-WALL next, the review tracks, INPUT-RUN, his item-4 follow-ups, ADV-R4-2 to -4) naming the report.
   (3) The scratchpad copy is a convenience only (it can be cleared); it is not counted. (4) The PR merges with a merge
   commit (as #71), so that SHA stays in main's history.

Findings: ADV-R4-1 Critical (item 2a, fix in this build). ADV-R4-2 Low: plan §8's "no deny or ask became a pass"
against main does not hold on the larger sweep; the 6 forms cannot start their program (checked), no effect; record
in the report, follow-up, no push. ADV-R4-3 Low: the option table's header comment still describes the old xargs rule;
rides item 3. ADV-R4-4 Low: the replace-string check relies on a property that holds today only because another rule
asks first; follow-up. The detail of every id is in the handback and the scratchpad, never here.

## Round 4 plan change: hooks-r4c (2026-11-26; branch head 450a46e; a plan-change review, not a round; no form named)
Checked myself: the real hook files still equal 9ad6bc9's; guard.mjs unchanged; the copy's suite 1551/1551; red 17
with hooks-r4b's guard and red 45 with 9ad6bc9's guard, each under the copy's tests (red9, red10: their guard files
hash-equal to those two); size against main guard +55/-14, tests +111/-3 = 166 added (stop line 200); the wide
differential rerun (253,080 lines, session and agent): hooks-r4b to hooks-r4c only stricter, no throw; against main,
looser only in ADV-R4-2's set (6 deny to pass, 85 session deny to ask), nothing new. The guard diff against hooks-r4b
is the rule U precondition named once (same expression), the ADV-R4-1 check after the gw block (adds an ask or an
agent refusal only, no early pass), and the ADV-R4-3 comment. The new reason reuses unwrap()'s reason text, as the
front check already does. Its silence when a read-only program follows matches his line (a) exactly; recorded.

Verdict: APPROVE hooks-r4c (sha256 prefixes: guard b1ab3110, tests 7c0f0f36). ADV-R4-1 and ADV-R4-3 closed in the
copy. ADV-R4-2 and ADV-R4-4 stay Low follow-ups; INPUT-RUN stays the named limit.

Hunk list and order, confirmed (guard: three hunks against 9ad6bc9; tests: four):
1. Guard, one Edit: unwrap()'s changed lines, declarations and every use together.
2. Guard, one Edit: rule U's block from the precondition line through the end of the new ADV-R4-1 block, the unchanged
   gw block inside it (the new check uses the precondition).
3. Guard, one comment-only Edit: ADV-R4-3.
4. Tests: the rows block, the R2-1 row, the mutant block. The re-pointed SAME-GAP anchor lies about 14 lines above the
   mutant block: one Edit spanning both or its own fourth Edit, either is fine.
After each guard Edit the guard stays loadable and no looser than 9ad6bc9 (the state after 1 is hooks-r4b's logic).
Conditions: the earlier ones stand (the real files equal 9ad6bc9's before the first Edit; text byte for byte from the
copy; each Edit asked of Mustafa with its exact text; no commit until the last test Edit; cmp both files against the
copy; the three kit suites with the same counts as the copy and main plus the new rows; the gate --no-smoke, CI runs
smoke; ONE commit). Round 4 (security-auditor) then reviews what changed since round 3 on that commit's head.
