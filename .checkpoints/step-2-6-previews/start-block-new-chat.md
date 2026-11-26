# Start block for the new lane-2 builder chat (step 2.6), written by Builder V2 at Mustafa's request (2026-11-26)

For Mustafa: start a new Claude Code chat with only `Mergincanli/Shamble-Frontlines` attached, on Opus 5.5, and paste
everything below the line as your message.

---

For the new lane-2 builder, from Mustafa (written by Builder V2 at my request, 26 November).

You take over foundation step 2.6, a preview link for every pull request, from Builder V2
(session_019YNMUSYpJGi1xxCjKgr4e5). Builder V2 had two repositories, so the guard never loaded there; you have one.

1. First act, while I watch: show that the guard's ask reaches me. In your main folder run exactly
   `git merge --ff-only HEAD~1` as its own command. The guard asks me before any main-folder merge of anything but
   origin/main; I should see a prompt, and I will answer "no". If it ran anyway it would change nothing (HEAD~1 is
   already inside HEAD), and that would show the ask did not reach me. If no prompt reaches me, or the auto-mode
   classifier refuses it first, tell me, do no sensitive step, and ask me in chat before each sensitive step instead.
2. The branch: PR #72's branch `claude/optimistic-albattani-mdoonz`. I allow you to commit and push there; Builder V2
   stops pushing to it once you tell me you have taken over. Work on it in a worktree
   (`git worktree add .gate/step-2-6 claude/optimistic-albattani-mdoonz` after `git fetch origin
   claude/optimistic-albattani-mdoonz`, each its own command), never by switching your main folder. Push nowhere else.
3. Read first: CLAUDE.md; on that branch the checkpoint `.checkpoints/step-2-6-previews.md`, the build notes
   `.checkpoints/step-2-6-previews/build-notes.md` (B12 to B16), the report `docs/reports/2026-11-26-step-2-6-previews.md`
   and `docs/runbooks/previews.md`; in `docs/DECISIONS.md` my entries of 25 and 26 November for step 2.6.
4. Done: plan revision 2 (c40f3f9) and my go of 25 November; the request to the CI Builder on #59; `hosting/` (the
   script, 175 tests, the header file, Wrangler's manifest and lockfile); the three workflow texts byte for byte
   (9ded5a87); draft PR #72; my sitting (the previews account, the project, the environment `preview` limited to
   `main`, its bypass box saved unticked); M1 passed on its second try; review rounds 1 and 2 closed; the report and
   the docs pass; the advisor agent's reviews of the heads. Code head 84a3ea44; the branch head is Builder V2's last
   push.
5. The approved texts and their sha256 (check them before and after every step):
   - `.github/workflows/preview.yml` 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc (my NEW-TEXT (a))
   - `.github/workflows/preview-close.yml` c2f1fa90511db0f558d4e9287d64e189877f8cbc03eb27e6efbe08787fe20e66
   - `.github/workflows/wall-test.yml` 3af067068060971beba8444346fdb836b7779a41403ac09d25e43027e8b18507 (throwaway:
     it leaves at the token sitting)
   - `hosting/package.json` b47551b125f78a667a031dd0718a025981c1395c036be3c330b5239aef7b9726 and
     `hosting/package-lock.json` df24ffc392062d041d4be8f9ac5b51be7a6696e0c5a8cbfc3bac0dc07a4924fc (the
     security-auditor's dependency check)
6. Left, in order:
   a. The guard check (item 1); then tell me in one line that you have taken over, with its result.
   b. Wait for the Guard fix chat's protection for main (my TOKEN-TONIGHT (b), MAIN-WALL-AGAIN (a)). If it is not
      merged by my click on the real token, ask me again first (my MAIN-WALL (b)).
   c. The token sitting: one message to me, in this order (the security-auditor's conditions): (a) reload the
      `preview` environment page and send a screenshot (bypass box unticked, "Selected branches and tags", "1 branch
      and 0 tags allowed", `main`, no custom rule); (b) both secrets only by "Add environment secret"; (c) after the
      secrets, the Actions, Dependabot and Codespaces secret pages show no CLOUDFLARE_ name (screenshots); (d) you
      re-run M1, a fresh run and a re-run: the refusal line through the annotations API and zero steps for all three
      jobs, a deployment record with status failure expected; (e) delete the secret WALL_TEST, and `wall-test.yml`
      leaves the branch (a change under `.github/`: the guard asks me). The token's start and end dates go in their own
      fields, never the single-date chip, 90 days apart; its end date goes into STATUS with a reminder 14 days before.
   d. The verifier on the final diff; fold the checkpoint into the report and delete `.checkpoints/step-2-6-previews*`
      in the last commit before ready; a head without [skip ci]; the advisor agent's review of that head; ready.
   e. My go after my Advisor's merge check; merge with the approved head as the expected head.
   f. After this PR and the CI Builder's hand-over step (#59) are both merged: M5 to M11 (the first live link and its
      comment, the served headers, reconcile, the month's count against Cloudflare's list minus the wall-test records).
7. Rules: git and gh always start their own command, with no other program in front of them. GIT-IN-NODE (a): main's
   tracked kit scripts may run git; a script you write never runs git and never writes a protected file. In the main
   folder only `git switch -c <new> origin/main`, `git merge --ff-only origin/main` and `git pull --ff-only origin
   main`. Never push to another chat's branch; never touch spikes/. The token goes from Cloudflare's page into
   GitHub's secret box and nowhere else. Keep the environment wall's detail out of every pushed file. No account ID,
   e-mail address, Windows user name, Access ID or `kid` value in repository files. When you wait on me, save the full
   question as a file in your notes, push it, and name the file and the commit in your message.
