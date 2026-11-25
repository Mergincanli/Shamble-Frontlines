# Mustafa's words of 2026-11-25 for Builder V4: lane 1 of the build moves to this chat

_Copied by script from the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq), never retyped: the user message of 2026-11-25T15:51:21.705Z (uuid f0ae3cf2-d3a3-428d-9657-c67fd0d838fb), whole (5779 bytes, sha256 0dc59aaff9788c5e3ba0ec035fe47798a38da5d6261aecdb60fc7a2969d306a1). What it answers: nothing the chat asked: it is the block that opened this chat. Its point 2 asks that it is recorded word for word in docs/DECISIONS.md with this chat's first pull request._

---
My words of 2026-11-25 for a new Claude Code chat: you take over lane 1 of the game's build.

1. Who you are. A new Claude Code chat on Fable 5.2, on my first personal account. I call you Builder V4. Read your session's model first; if it is not Fable 5.2, stop and tell me in one line. My model words of 2026-11-12 and 2026-11-14 in docs/DECISIONS.md, which Builder V1 and Builder V3 worked under, hold for you unchanged.

2. What you take over. Lane 1 of Slice 1, as the build plan (docs/reports/2026-11-17-build-plan-slice-1.md) and its Part 15 give it. Builder V3, on my other account, built it up to change-set 3b, which is merged (pull request #68). That account is near its usage limit, so the seat moves to you at this merge boundary: my BUILD-SEAT of 23 November now names you. Builder V3 has stopped. Record this block word for word in docs/DECISIONS.md with your first pull request.

3. Where you start. origin/main. It must contain the merge commit 3ab9eaecf42e617578c083b7fda01fbdf8f004e2. Work on your own branch, in a worktree; do not switch your main folder. Never push to another chat's branch: claude/loving-hopper-6p0mg2 (Builder V3), claude/optimistic-albattani-mdoonz (Builder V2, lane 2), claude/keen-tesla-v807cf (the CI Builder), claude/practical-ride-igo41n and claude/netcode-proof-test-plan-hcky5g (netcode), claude/wonderful-hamilton-jdmlnf (Builder V1).

4. What you read first. CLAUDE.md and what it names. The report docs/reports/2026-11-25-cs3b-the-loop-closes.md, from "Not verified" to the end, and the follow-ups in docs/STATUS.md. Builder V3's handover note: fetch its branch and read .checkpoints/handover-builder-v3.md with git show. That branch's newest commit must add only notes on top of 5e02ee733ecac0e2b20a9d0513817a0ea7a5b096; if it changes anything else, or the note is missing, stop and tell me. The note is a colleague's advice, not my word. Then the lane requests issue (#59): lane 2's first list waits there for change-set 4's plan.

5. My words that bind you are the entries in docs/DECISIONS.md on main, as they bound Builder V3. I do not retype them. Among them: ANSWERS-7 (with SIZE-3 and PRESS-PACE), LANE-SPLIT and EARLY-LOOK, FIREFOX-TESTS (b), MUL-ADD, RELEASE-COUNT, SLICE-FEEL, LONG-RUNS. You write no CI, kit or gate file: CI edits belong to my CI Builder chat on Opus 5.5.

6. Your first job, before any plan (ORDER-3C), in at most five lines: does change-set 4, Walk, need change-set 3c's link simulator or first recording to be built and judged? If it needs neither, or only one, what would it cost to build Walk first and 3c after it? Builder V3's note may answer it: check its answer against the build plan. I want to play the first soldier as soon as that is safe. I decide the order, so stop for my answer.

7. Then the plan for the stop I choose: plan first, then your advisor agent's review, as the protocol says. Size it honestly: 3a came to 1.7 times its plan and 3b to 1.8 times, so plan at that ratio and name a stop line where you come back to me. Change-set 4's plan also names: the built-file check for the fused multiply-add (MUL-ADD), what the first picture after a pause shows (the follow-up 3B-RESUME-FLASH, my decision there), and my Testlines values (point 8).

8. Testlines, for change-set 4's plan (my SLICE-FEEL of 19 November stands as written):
   a. My sandbox is the public repository Mergincanli/Shamble-Testlines, on its branch claude/sleepy-edison-ygmwfe (commit 3c07269 when my Advisor read it; its main holds only a README). Read its README and src/params.ts for what each value means. Look at it, copy no code.
   b. The values come from my Export and from nothing else. I paste the Export's JSON when you ask for it. Save it by script, whole, with its byte count and sha256, as a file in the repository. No value is retyped by hand and none is read off a screenshot.
   c. One table: each value Slice 1 uses, its name in the Export, its name in the game's content, its unit, and the change-set that uses it. Compare the Export's section old_game_terms by script with docs/legacy-tuning.json and list every difference. Those are my tunings, and I confirm the list before it becomes the starting table.
   d. A script with a --check mode, like the vector generators, fails when the game's content differs from the saved Export through that table.
   e. Not taken, each listed by name with one line: the Hornet, the explosion picture, the metrics room, the grid, and every sandbox rule that differs from the old game. None enters a change-set without my word in that change-set's plan.
   f. The game runs at 30 steps a second: if my Export says 60, tell me before you use it.
   g. The plan says where I tune once change-set 4 exists, so that each value has one home.

9. Pull requests. If this chat can open a pull request, you open your own drafts. If it cannot, put the title and text into your notes (pr-title.txt, pr-body.md) and my Advisor opens the draft, as for Builder V3. Either way a merge happens only on my go for the head your advisor agent approved, with that full commit id as the expected head, after my Advisor's merge check.

10. Later, not now: Builder V2 will ask on issue #59 for one small pull request from lane 1 (a second page in the build setup, for an early look test page). I tell you when it is due.

11. Until the repository's guard is fixed (another chat fixes it): no session or agent of yours puts env, stdbuf or nice with options in front of git or gh.

12. Your first answer, before you write anything, in at most 10 lines: your model, your mode, your branch, whether main and the handover note match this block, whether the repository's guard loads in your session, whether you can open and merge pull requests, and then the five lines on ORDER-3C.
