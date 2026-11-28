# Y' head read: the brief for B6 (advisor-stopb-plan.md's author), Q4 (f) of its "Ruling on the dist difference"
Status: SENT 02:5x UTC on 11-28 after build 2' ended equal (CI on Y''s save still running then).

## The brief (to B6: resumed if it answers, else a new advisor agent with this file as its whole brief)
Your Q4 (f): the head read of Y' with both new builds. Read by SHA only.
- Y' = commit 1ceecc321eaf194d59568fdab56644d1e9eb4f05 ("Stop B Y'"), pushed under the card save
  232cc8c9b53bb4216e7cf97d7b9baeb25928e24f; Y = 652e9513 (void, STOPB-8); Y0 = 87cb8677.
- The change: spikes/netcode-proof/ops/runs/r1-dist.sha256 line 10, the Wasm line, from
  2a97394b023c5f52e7c2dcbcbb28204dd9bcf23bc4c0acaaf5f3c9191e4c4551 to
  901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77, in place.
- The bars as printed (done lines in stopb-plan.md, section C, "Y' DONE"): `git diff --numstat 652e9513 <Y'> --
  spikes/netcode-proof` = "1 1" on the list file alone; `git diff --numstat 87cb8677 <Y'> -- spikes/netcode-proof` = "7
  0" on the list file alone; Y''s non-comment lines cmp-equal to .gate/stopb/build1p-dist.sha256; the advisor tool's
  extra bar: `git grep -n 2a97394b -- spikes/ docs/` prints nothing at Y'; the counted run
  .gate/stopb/yp-counted.log (`--only self,ops,redact,refs`, its diff hash = Y''s diff over the spike folder); the gate.
- Build 1' (.gate/stopb/build1p.log; worktree .gate/stopb-a at Y 652e9513, the tree before MAIN-70; cargo base
  scratchpad/npt-dist-a) and build 2' (.gate/stopb/build2p.log; worktree .gate/stopb-b at Y', the tree after MAIN-70's
  merge 2d379d9c; cargo base scratchpad/npt-dist-b), each with your Q2 prints: the GIT_* names printed and unset; `git
  rev-parse HEAD` failing in the cargo base and in wasm-bindgen-shared-0.2.130's registry folder; `producers` reading
  "wasm-bindgen 0.2.130"; and the lists: build1p-dist.sha256 and build2p-dist.sha256 equal byte for byte and equal to Y''s
  non-comment lines. The two builds sit on different trees (inside your ruling: Y or Y0 for build 1', Y' for build 2'),
  so their equality also shows MAIN-70's #79 is no dist input.
- Your Q4 (e) triggers, both cleared: MAIN-70 changed nothing under spikes/ (M70-3's third bar,
  `git diff --name-only e646ee59 2d379d9c -- spikes/ docs/design/ .checkpoints/` prints nothing:
  main-70-plan.md "Steps 2-5, done"), and main's 107 paths hold no kit or CI path (`.gate/main-70/main-names.txt`, a
  printing grep for `.claude/`, `.github/`, `tools/gate`, CLAUDE.md, AGENTS.md: nothing). So Y0 stands and X takes in no
  kit file.
- CI on Y''s save 232cc8c9: running at sending (02:5x UTC); the main session appends the four runs' lines here and
  tells you by message when they end. Write every other block meanwhile; your verdict line names CI's state as read.
  APPENDED 02:59 UTC: all four success at attempt 1 (check_suite.completed 02:59:03): gate push 46099627026
  (02:45:35-02:46:50) and pull request 46099631890 (02:45:39-02:46:47); toolchain push run 46099627427, job
  140838881813 (02:45:38-02:56:33, steps 11-12 skipped), pull request run 46099631886, job 140838895873
  (02:45:42-02:55:43, steps 11-12 success). Both toolchain jobs about 10-11 minutes against 25.
- Build 2' as done: stopb-plan.md section C, "Build 2' DONE" (exit 0 02:45:57-02:47:25; both lists equal; producers
  plain; the two git lines "ok"; 11 GIT_ names unset).
- One correction asked of you: advisor-stopb-plan.md:623 says every list so far carried a hash, "build 1, build 2, and
  U7's run 4". U7's run 4 Wasm is 901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77 (u7-report.md:141),
  build 1''s plain value, so run 4 carried none. Correct that line in your file (a dated correction note under it),
  and say whether run 4 counts as a third agreeing build (made by another builder on another path and day) for the
  report.
Rule: Y' stands (STOPB-8 resolved by the method), or what is owed. Write your blocks into advisor-stopb-plan.md under a
new heading "## Y' head read" (your plan first, then each block as it is done); end with "Decisions needed" and "For
Mustafa", then the last line `Status: COMPLETE <date and UTC time>`.
The same tool rules as before: Write and Edit only for files; git first, one git per call, no `git -C`; printing forms
only; his full-paths rule; names of environment variables only, never values; no GitHub writes.
