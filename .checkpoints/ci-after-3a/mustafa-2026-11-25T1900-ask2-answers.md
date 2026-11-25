# Mustafa's answers to ask 2: CI-WRITE-2 (a), FF-STOP (a), FF-FILEURL (a), GUARD-LOAD (a)

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-25T18:00:43.219Z (uuid 3d775773-0e83-4611-95f3-04d6845b6436), whole (1681 bytes, sha256 a7bbf87d193fa1f85e5adcb88522780be638494f1a2026da37e2ce8b565c23a2). What it answers: the chat's ask 2 (question-2026-11-25-ask2.md at e12bb44): CI-WRITE-2, write the two texts exactly; FF-STOP, keep the 5-minute stop; FF-FILEURL, file-url.spec.ts in Firefox with its comment; GUARD-LOAD, bring the main folder to main; each recommended (a)_

---
FOR THE CI BUILDER. From Musti, 25 November, 19:00 Amsterdam. My answers to ask 2.

1. CI-WRITE-2 (a): yes, write both texts exactly as saved at e12bb442 (draft-toolchain.diff, sha256 55bdbd69a9c4e06609a2ef3e6547e0a1562d7217f3bb4feb80a2a8b4362e424f; draft-playwright-config.diff, sha256 7f79116888311c52e29b14839ff3fa72573e02bde96e4fa0becba8170d433dbe). After the write the workflow must be sha256 01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742 and the test configuration fed4b79d38cbbbf15cd69b0225e576dbfce880f9a69292096e74b1619b9e43ed. My Advisor rebuilt both from your drafts on main and got these two values. If either differs after your write, stop and show me the difference.
2. FF-STOP (a): keep the 5-minute stop for the first run.
3. FF-FILEURL (a): yes, with the comment that says so.
4. GUARD-LOAD (a): yes, git merge --ff-only origin/main in your main folder, alone in its command. Tell me in one line that the fixed guard loads.
5. Main is c1da1bd24faae21794195e2384895460753a41eb since 18:12 (the merge of #67, docs only; the workflow and the test configuration are unchanged on it). Take it in once before your docs pass.
6. The first run on GitHub, as in my FIREFOX-GL (a): if the Firefox drawing tests fail on the page's "no WebGL" line, the firefox entry runs the rule tests only, by a change to the test configuration alone, and the drawing tests become a follow-up. Any other red, a time-out included, comes to me first.
7. Then as agreed: the pull request's title and text into your notes, and my Advisor opens the draft. Until the second guard fix is merged: git and gh always start their own command, with no other program in front of them.
