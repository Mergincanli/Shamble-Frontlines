# Mustafa's CI-TIME (a): toolchain's timeout-minutes 15 becomes 25

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-27T12:25:12.665Z (uuid 892aacd7-4da9-405e-84d3-3090f2316f54), whole (1280 bytes, sha256 cbff88724bcb157fd0d69efbc92fa5837f01f0e8dd9c9bc9aab47d3e0f37ee06). What it answers: nothing the chat asked: one small job, the job's time limit raised from 15 to 25 minutes, the exact text approved in advance_

---
FOR THE CI BUILDER. From Mustafa, 27 November, 12:50 Amsterdam (written with my Advisor).

1. One small job, CI-TIME. `.github/workflows/toolchain.yml` line 38 on main says `timeout-minutes: 15`. Builder V4's newest head (5c6738a) took 13 minutes, and the rest of Walk adds tests. A run that passes the limit fails with no defect.
2. CI-TIME: (a). That one line becomes `timeout-minutes: 25`. This exact text has my yes, so do not ask me again for the text. Nothing else in the file changes, and the 5-minute stop for Firefox stays.
3. First fast-forward your main folder to main (b9aad01954be666340a7ec5c6e9fc9353b11abf1) so the newest guard loads. Then a plan of a few lines for the advisor agent, the change, and the security-auditor's read (risk tier 2).
4. Open the draft pull request yourself if this chat can. If it cannot, put the title and text in your notes and my Advisor opens it.
5. When the advisor agent and the security-auditor approved the head and gate, smoke and toolchain are green on it, tell me its full id. On my go for that id you mark it ready and merge it with that id as the expected head. If GitHub refuses the merge, tell me and work around nothing.
6. Rule unchanged: git and gh always start their own command, with no other program in front of them.
