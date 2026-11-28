# His answers to the thirteen questions before 4c's steps, and how the scene and the soldier share one branch (2026-11-28, in the chat of Builder V6)

_Copied by script from the chat's transcript (session_01WLuqnt1v3UNwSqbJsJre7u), never retyped: the user message of 2026-11-28T11:43:45.498Z (uuid e5c77df2-375c-4f7b-8084-ec59ef7c41ff), whole (4516 bytes, sha256 3b14fb63513e01f0aa15e0325a6813978f8a78608af2df1110575eecb7043f02). What it answers: the thirteen questions of Builder V6 (the question file .checkpoints/cs4c-soldier/question-2026-11-28-before-4c.md at 28690f1c: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SIZE-4C, BOARD-WORDS, PASTE-BREAKS, DESIGN-CLASH, STREET-GREY, BODY-SHAPE, DEPART-4C, FALL-STEP, DIGEST-NOTE) and its three asks in the chat (reading his Testlines repository, the CI Builder's Chromium time, the no-aim-yet line he may veto). The transcript holds the same 4,516 bytes twice, 37 seconds apart with the same parent entry (uuids 17b6bde5 at 11:43:08 UTC and e5c77df2 at 11:43:45 UTC): one message handed to the chat twice when the session restarted; the first led to no action, and this is the second, the one the chat answered_

---
FOR BUILDER V6. From Mustafa, 28 November, 12:10 Amsterdam (written with my Advisor). My answers to your 13 questions. I am away until Sunday 29 November, 12:00. Build on without me.

1. MOVE-RULES: (a), all rows, as the game's starting rules. Each one stays mine to change after I have played. Rows 6 and 7 are confirmed again at 4d. Row 9 (he walks any slope, the roofs included): as my sandbox for now, recorded as open and not as decided. I decide a slope limit after I have walked the roofs in the early look. Build nothing that depends on roofs being walkable or not.
2. NO-SOURCE: (b). The game starts from my sandbox's numbers, each marked "not from his Export". I may make them sliders in Testlines later, and a later Export then replaces them the same way.
3. RESUME-FLASH: (b).
4. TUNE-HOME: (b).
5. SIZE-4C: (b). The scene alone first, then the soldier as one pull request with the early look inside it. The soldier's stop line is 18,700. If the count reaches it before the early look is ready, do not stop: tell me in two lines in your next report and build on to the early look. Past the line, nothing after the early look starts without my word.
6. HOW (b) WORKS WITH YOUR ONE BRANCH, so that you never wait for me:
   a. Finish the scene now: its reviews, the verifier, its docs commit, the report, the notes out of the head, the advisor agent on that head. Put the pull request's title and text in the notes. Tell me the scene's final head with its full id, and the steps to see the scene in Brave.
   b. Then build the soldier on the same branch, on top of that head, and push as usual.
   c. On Sunday my Advisor points a branch of its own at exactly the scene's final head and opens the scene's pull request from that branch. Your pushes then do not move that pull request's head. I do my Brave check on that head and give my go for it.
   d. If I want something changed in the scene after my Brave check, it goes into the soldier's pull request. When the scene's merge is on main, take main in once.
7. BOARD-WORDS: (b).
8. PASTE-BREAKS: (a).
9. DESIGN-CLASH: (a) at both lines. At LOOP_V2's "uplinks replace silos" line the pointer reads: "(2026-11-27: §14's resource silo is a different thing, a part of the harvester building with a small passive income; see §14)".
10. STREET-GREY: (b). I look at it at the scene's Brave check.
11. BODY-SHAPE: (3a).
12. DEPART-4C: (a), all three, on these terms.
   a. SWEEP-ROUTE: yes. The collision library stays the check on our own sweeps for every shape the game uses, the ramp included, before the movement step counts as done. The report shows the old and the new wording of PM-0004-20 and PM-0004-35 side by side, and the new wording asks no less than the old about what the players' file does.
   b. PM-0004-60: 2,000,000 per CI run now. The mark is not met by that: it stays open in STATUS until the nightly job runs its 10 million, named with the change-set that brings it.
   c. PM-0004-20's 48-soldier run: not met by the one-soldier recording. It stays open in STATUS, named with the first build that can show it.
13. FALL-STEP: (c), by reading. The game does what my sandbox does. If the sandbox's code does not settle it, take (a).
14. DIGEST-NOTE: (a), leave it. No history is rewritten.
15. TESTLINES: yes, you may read my Testlines repository by its public address, read only. Copy no code from it: numbers and behaviour only. Do not attach it to this chat as a second repository. That also settles the head bump and the ramp met from the side.
16. "No aim yet" for a new soldier: fine. I look at it in the early look.
17. CHROME-TIME: the CI Builder is parked until Tuesday 1 December, 20:00, and raises the Chromium run's limit then. Until then never cut or thin a test to fit the 5 minutes. If a run stops at the limit, re-run it once. If it stops again, tell me in two lines and build on. A head that goes to the advisor agent needs a green run.
18. WHILE I AM AWAY: save and push at least every 25 minutes. A new question that needs me goes into your question file with your recommendation, and you move on to the work of 4c that does not hang on it. The advisor agent decides what it may decide.
19. Rules unchanged: git and gh always start their own command, with no other program in front of them, one git per shell call. In a shell command, spell out every file you write with its full path: no variable, substitution or wildcard in the name of a file that is written, moved, copied or deleted, or use the Write and Edit tools.
