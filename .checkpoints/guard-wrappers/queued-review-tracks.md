# Queued: the change-set after guard-wrappers, "the review tracks and who merges" (risk tier 2)
Received in the Guard fix chat, 2026-11-25, as a paste headed "FOR THE GUARD FIX CHAT. From Musti, written with my
Advisor, 25 November, 18:25 Amsterdam." Plan first; no rule file edited before his go on the plan; it waits until
the second guard change-set (guard-wrappers) is done. This note is a working summary for resuming, NOT the record of
his words: the change-set saves his quoted words by script into docs/DECISIONS.md, word for word, from his paste.

Its items, in short (the paste itself is the authority):
1. His words to record (his walk of 25 November; his reply of 25 November, evening), with his meaning: "your review"
   = his Advisor's merge check; V4 = Builder V4, lane 1's builder, and the same for every builder chat; "what I see
   on screen" = today's "anything a player sees, hears or feels".
2. Why: his Advisor's timings of #65, #68, #69 (building / checks / waiting for him); measure them before quoting.
3. The plan proposes the smallest change to CLAUDE.md and advisor.md: (a) the track worked out by a script from the
   files touched, in doubt full; (b) LIGHT-DEF (a); (c) a fifth full-track exception: the rule files; (d) CORE-REVIEWERS
   (a), with a rule for the at-most-two extra reviewers and a check of each High of 3a and 3b; reviewer effort as his
   words of 12 November; (e) who merges (the building chat, on gate + smoke + toolchain green, the advisor agent's
   approval, his Advisor's merge check, the id as expected head; a later push needs all again); (f) EYES-KEEP (a);
   (g) one table of every needs-mustafa reason: stays, moves to his Advisor's check, or goes, one line why each.
4. Plain answers: (a) how his Advisor's merge check reaches the building chat without his paste (one PR comment naming
   the full id), and what stops a chat writing it itself (same GitHub user), and what catches it afterwards; (b) how
   the building chat is woken; (c) what the PR Advisor routine still does, and the risk-tier table's changes; (d) how a
   wrong change that shows nothing is caught after the merge (rollback rule unchanged).
5. Shape: every changed sentence old beside new; every word of his changed or narrowed, listed; security-auditor and
   the advisor agent on the plan; sized bottom-up (rule text, script, tests; not docs or report); stop line twice the
   size; above 150 added lines, a smaller version beside it.
6. Until merged, today's rules hold; until guard-wrappers is merged, git and gh always start their own command.
End with "Decisions needed" and "For Mustafa".

## His paste, copied whole and unchanged (so a compacted session still has it; the DECISIONS record is made from his
## paste by the change-set itself, and "saved by script" is a question for its plan: no words-copying script exists
## under .claude/scripts or tools/ today)
----- begin paste -----
FOR THE GUARD FIX CHAT. From Musti, written with my Advisor, 25 November, 18:25 Amsterdam.

Your next change-set after the second guard change-set: the review tracks and who merges. Risk tier 2. Plan first, and no rule file is edited before my go on the plan. If the second guard change-set has not started, it still goes first.

1. My words, saved by script and recorded word for word in docs/DECISIONS.md with this change-set.
From my walk of 25 November:
"Light review track by default. Exceptions that always get the full checks: netcode, the Rust core, server code, anything anti-cheat."
"The rule lives in the repo files, not in a label someone picks."
"Less process on small changes, so I see visible progress sooner."
From my reply of 25 November, evening:
"My eyes only where it shows: I only check pull requests that change what I see on screen. Everything else merges on green checks plus your review; V4 merges its own."
What I mean by it: "your review" is my Advisor's merge check. V4 is Builder V4, lane 1's builder chat, and the same holds for every builder chat. "What I see on screen" is today's "anything a player sees, hears or feels".

2. Why. Measured by my Advisor from commit times on the last three merged pull requests. Stop 3a (#65): building 2.4 hours, checks 4.0, waiting for me 4.9. Stop 3b (#68): 1.1, 4.9 and 9.2. The guard fix (#69): 0.3, 1.0 and 1.2. Measure them yourself before you quote them.

3. What the plan proposes, as the smallest change to CLAUDE.md and the advisor agent's file that does it:
a. The track of a pull request is worked out by a script from the files it touches, never picked by a person or a chat. In doubt it is the full track.
b. LIGHT-DEF (a), what "light" means: the plan and the advisor agent's read of it stay; the gate and CI stay; one review round by the one reviewer that fits, at the repository's default effort; the verifier on the final result; a short report; no second round unless a High or Critical is open.
c. A fifth exception that always gets the full checks: the rule files themselves (CLAUDE.md, .claude/, .github/, the gate files, the guard).
d. CORE-REVIEWERS (a), what "full" means for the Rust core, and for netcode and server code in the game: the test-engineer always, plus at most two more picked by a rule from what the piece touches. The plan writes that rule. At most two rounds, as today. Show whether each High of stops 3a and 3b would have been found or missed under it. The effort of these reviewers stays as in my words of 12 November.
e. Who merges. A pull request that changes nothing a player sees, hears or feels, and meets nothing in f, is merged by the chat that built it, with no go from me, when all of these hold for the same full commit id: gate, smoke and toolchain green; the advisor agent's approval; my Advisor's merge check; and that id passed as the merge's expected head. A later push needs all of them again.
f. EYES-KEEP (a). Three things keep my go even when nothing shows: anything that costs money or cannot be undone; the rule files in c; and a decision of mine that is changed or newly set (a decision draft accepted, a design or look rule, a pass mark). A record of words I typed, copied by script, is not in this list.
g. One table of every reason that makes a pull request "needs-mustafa" today: it stays with me, it moves to my Advisor's merge check, or it goes, each with one line why. Nothing leaves that table silently.

4. What the plan answers plainly:
a. How my Advisor's merge check reaches the building chat without my paste. My Advisor proposes one comment on the pull request that names the full commit id and what was checked. And what stops a chat from writing that line for itself: all sessions post as the same GitHub user, so if nothing can stop it, say so, and say what catches it afterwards. My Advisor proposes to compare every merge on main with its own checks at each board check.
b. How the building chat is woken when that check is in, without me.
c. What the PR Advisor routine still does, and what changes in the risk tier table.
d. How a wrong change that shows nothing is caught after the merge. The rollback rule stays as it is.

5. Shape. Every changed sentence shown old beside new. Every word of mine that it changes or narrows, listed. The security-auditor and the advisor agent on the plan. Size it bottom-up: rule text, script and tests count, docs and the report do not, as in CAP (a). The stop line is twice that size. If it is more than 150 added lines, show me a smaller version beside it.

6. Until this change is merged, today's rules hold in every chat. Until the second guard fix is merged: git and gh always start their own command, with no other program in front of them.

End with "Decisions needed" and "For Mustafa".
----- end paste -----
