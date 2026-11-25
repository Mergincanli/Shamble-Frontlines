# Mustafa's answer FIREFOX-GL (a), with the red-run rule, the wait for #68 and the guard rule

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-25T13:54:38.922Z (uuid 641bd3ba-3dbd-451d-919e-46ebee973577), whole (1681 bytes, sha256 1c45836da10b4f8daadb42874fff9d72eb340e8c869564f8c29a9c44cb7ad7eb). What it answers: the chat's question FIREFOX-GL (question-2026-11-25-firefox-gl.md at 0a4e32a): how Firefox enters CI; (a) the full Firefox run with the libEGL line, the first GitHub run as the drawing tests' trial, (b) the rule tests only, (c) more digging; recommended (a)_

---
For the CI Builder, from me, written with my main Advisor (25 November, 14:55 Amsterdam).

1. FIREFOX-GL (how Firefox enters CI): (a). Ask 2 brings the full Firefox run with the read-only line about libEGL.so.1, and the first GitHub run is the trial of the drawing tests.
2. Decided now, so that a red run needs no new question: if on GitHub's runner the Firefox drawing tests fail with the same line as in your container (the page could not start drawing because Firefox gave it no WebGL), change the Firefox entry in the test configuration to the rule tests only (your option (b)), name the Firefox drawing tests as a follow-up, and tell me in three lines. Chrome's drawing tests stay as they are. Any other red result, or any other cause, comes back to me first with options, and you fix nothing.
3. No install line for the runner and no Firefox setting that switches off its own graphics refusal (such as webgl.force-enabled) without my word: each is its own question to me, in that follow-up.
4. Main has moved: #66 is merged, and change-set 3b (pull request #68) merges today and adds browser tests. Before ask 2, wait until main shows the merge of #68, take that main into your branch in your worktree, and run your test lists and timings again on it. If #68 is not merged by Thursday 26 November, 12:00 Amsterdam, tell me in one line and go on from today's main.
5. The two library files did not help: delete the folder .gate/egl now and say so.
6. Until the repository's guard is fixed (a finding of today, another chat fixes it): no session or agent of yours puts env, stdbuf or nice with options in front of git or gh.

Answer in at most 6 lines. Plain words, as before.
