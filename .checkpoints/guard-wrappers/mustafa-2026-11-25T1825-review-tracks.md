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