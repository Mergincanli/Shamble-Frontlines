# Question for Mustafa from the new lane-2 builder chat (step 2.6), 2026-11-26, after 15:25 UTC

## MERGE-74: take main in now, with #74's change under .github/, or wait for MAIN-WALL?
- What happened: main moved to 3b949c1 (#74, the CI Builder's hand-over step, merged 2026-11-26 15:24 UTC), and #72
  conflicts again (git's list: DECISIONS, REPO_MAP, ROADMAP, STATUS; toolchain.yml itself merges cleanly). While #72
  conflicts, GitHub starts no pull-request runs on it.
- #74 changes `.github/workflows/toolchain.yml` (the hand-over step, comments) besides docs and
  `packages/e2e/playwright.config.ts`. Taking main in brings that change into lane 2's branch, main's text byte for
  byte. #72's own diff against main would stay the three approved workflow files, unchanged by sha256.
- Your words of 15:05 Amsterdam, item 3: "Until then: nothing under .github/, no token sitting." The guard does not
  ask for a merge inside a worktree, so only your word covers a `.github/` file coming in through a merge.
- Options:
  (a) wait: take main in once, after MAIN-WALL merges (one merge and one review for both; #72 stays conflicting until
      then, which holds nothing back, since nothing on #72 moves before MAIN-WALL);
  (b) take main in now (main's toolchain.yml byte for byte, the docs by the same scripted method as before), and again
      after MAIN-WALL;
  (c) something else.
- Recommendation: (a). Nothing on #72 moves before MAIN-WALL, MAIN-WALL moves main again, and one merge saves a review
  round. If you pick (b), I merge at once and the advisor agent reviews the merge as before.
- For information: #74 hands the tested build over under the name `preview-site`, the name preview.yml downloads
  (line 93) and the script checks for (`hosting/preview.mjs:241`). So the CI half of your item 6g is merged; the first
  live link needs #72 merged with the token in place.
