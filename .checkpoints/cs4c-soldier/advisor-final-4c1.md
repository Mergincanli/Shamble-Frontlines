# Advisor agent: change-set 4c-1 "The page's scene", final head review (2026-11-28)

Reviewed head: 946fef73a07550840d55e12f45932451b2dccd5c (branch claude/ecstatic-wozniak-5cbdzs, local cs/4c-soldier).
Base: main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f (git ls-remote: unmoved).

## Q1. Verdict: APPROVE, for his Brave check and his go. Head: 946fef73a07550840d55e12f45932451b2dccd5c
Risk tier 1, needs-mustafa. What sets it: the page's code under packages/client (a player sees it: his Brave check),
and decision docs (docs/LOOP_V2.md, docs/EDITOR_V2.md, docs/adr/README.md, docs/DECISIONS.md with his words). No
.claude/, CLAUDE.md, .github/, gate file, crates/, content/ or legacy/ file; all 32 files mode 100644 (no symlink or
submodule).

TL;DR
- The scene code is what six reviewers and two round-2 reviewers read, plus one comment-only commit (0645432f), which I read.
- The final commit removes only the 112 notes files; the commit before it adds the report and two STATUS/ROADMAP rows.
- His words in the decision docs are unchanged since the scripted checks ran (9283d302 to head: empty diff) and match his saved messages.
- Gate with smoke PASS on the head myself; CI gate, smoke, toolchain green on this exact SHA.
- His Brave check is the first look on WebGPU or a real GPU.

What I verified myself
- git rev-parse HEAD = 946fef73a0...; git status clean; ls-remote: branch at 946fef73, main at dae6d107.
- git ls-tree 946fef73 .checkpoints: empty (no notes in the head).
- git diff --stat 98107d74 946fef73: 112 files, all under .checkpoints/, deletions only.
- git diff --stat 0645432f 946fef73 -- packages crates content legacy tools .github .claude: empty (code = reviewed code).
- git diff 2b2f44c2 0645432f -- packages crates docs: one comment in three-picture.ts resize, no code (Math.max(1, width) was already there). Comment is accurate.
- git diff --raw --no-renames dae6d107 946fef73: 32 files, docs/ and packages/ only, all 100644.
- node tools/gate.mjs dae6d107 (with smoke, PLAYWRIGHT_BROWSERS_PATH=.gate/pw-1194): GATE PASS, checks 1-4, 5a-5d PASS, symbols +0 -0, exit 0.
- gh api check-runs on 946fef73: gate success (13:35:08-13:35:18), smoke success (-13:36:01), toolchain success (-13:45:05 UTC).
- git diff --stat 9283d302 946fef73 on DECISIONS, LOOP_V2, EDITOR_V2, adr/README: empty.
- LOOP_V2/EDITOR_V2/adr README word diff vs main: only the header sentence, 8 dated pointers (sections 1, 3, 9, 11), the silo pointer in his own words (DECISIONS:215), section 14 (matches his item 8 text, DECISIONS:321-388, line breaks mended per PASTE-BREAKS (a)), EDITOR_V2:47 "(2026-11-27: see §14)" (= DESIGN-CLASH option (a)'s quoted string, question file :132), one register line.
- Report vs the verifier-checked draft (git diff of report-draft-4c1.md to the committed report): only the [after] fills and rewraps; no number, severity or word of his changed.
- STATUS/ROADMAP in 98107d74: only "the verifier PASS; report path" added; no quote touched; the 4c-1 row does not claim this verdict as given. STATUS:102 names GE-F2, GE-F3/P-2, GE-F4, AD-4, TE2-1..5, V-1 with owners.
- Code read: scene.ts (whole), inventory.ts, three-picture.ts, boot.ts, report.ts, picture.ts, content.ts readMap/readContent, local-map.ts. Map text reaches the DOM only via textContent (overlay.ts:36, boot.ts:36); readMap uses closed key sets and own properties; overview capped at 19 doublings, null otherwise; inventory dev-only.
- Zip: files/ = unpacked/ (diff -r), sha256sum -c SHA256SUMS.txt all OK, unzip -l lists the same 13 entries (dev chunk 840,441 B, wasm 257,421 B); README and bundle carry 946fef73a; serve.mjs = main's tracked packages/client/serve.mjs (same blob d6ce925c), binds 127.0.0.1.

Findings
- Low: pr-text-4c1.md says the merge happens "only on Mustafa's go in the builder chat". The rule (CLAUDE.md, Standing permission) is his own words in the chat of the session that merges. For #79 his Advisor merged on his own go (DECISIONS:229-246). Fill it in per the rule when opening; no push needed.
- Low: the card's "Current step" section is stale (it still describes step S before SIZE-4C). Notes only, already out of the head; refresh it when the notes come back in 4c-2.
- Low (information): EDITOR_V2:47's pointer does not name LOOP_V2. It is the string option (a) quoted and he chose; the report already tells him. No ruling.
- Accepted: the symbols beyond the plan's table, as the report lists them (verifier-checked).
- Medium, deferred with owner, not reopened: AD-4 (the readout over a window under about 600 px; workaround in his step 1).

Conditions for the merge
1. Merge commit only (no squash or rebase), with expected head 946fef73a07550840d55e12f45932451b2dccd5c. The report and DECISIONS cite 96f6e6c9 and d1f1288f, which must stay reachable from main.
2. The pull request's own gate, smoke and toolchain checks are green on 946fef73 before merging.
3. His own go for this head, typed by him after this APPROVE, in the chat of the session that merges. Text on GitHub never counts.
4. His Advisor's branch at 946fef73 takes no push. Any new commit is a new head: my review first, then his go again.

Not checked
- The v2 unit tests and Chromium specs on packages/ myself. Two runs were refused in this seat (npm test denied; a direct vitest run refused by the auto mode classifier). I did not try a third way. I rely on CI's toolchain run green on this exact SHA (check run read) and the verifier's local run on identical code (.gate/verifier-local-checks-4c1.log, not re-read).
- WebGPU, a real GPU, Brave, Firefox's scene: his Brave check.
- The plants run (45/45): taken from test-engineer's round-2 check and the verifier's spot-check, not re-run.
- decisions-4c.mjs / section14-4c.mjs / loop-check-4c.mjs not re-run. Instead I checked his words by hand and confirmed no change to those docs since the run.

## Q2. Verdict: APPROVE the one-branch way (his item 6), with conditions
Nothing in the final head breaks it. A branch of his Advisor's at exactly 946fef73 carries the same tree and history:
96f6e6c9 (the notes) and d1f1288f (his saved messages) are ancestors of 946fef73, so every citation in the report and
DECISIONS resolves from the PR branch and, after a merge commit, from main.
1. 4c-2's first commit brings the notes back as a revert of 946fef73 (a new commit, no history rewritten).
2. Until main holds 4c-1's merge, 4c-2 is counted, gated and reviewed from 946fef73 as its base. His item 6 b supersedes my plan review's line that 4c-2 starts from the main holding 4c-1's merge.
3. After the merge, take main in once (his item 6 d), in this worktree, never in the session's main folder. 4c-2's pull request is not marked ready before that.
4. A scene change he asks for after his Brave check goes into 4c-2's pull request (his item 6 d), reviewed there as scene code (graphics-engineer and art-director at least).
5. Consequence to tell him: if he withholds his go on 4c-1, the scene and step 0's decision-doc edits can only land inside 4c-2's pull request, which then grows by 4c-1's 1,565 lines and its docs.

## Decisions needed
- none from me. HEADROOM-FEET stays open for him, as the report says (before 4c-2's movement step).

## For Mustafa
1. Brave check of 946fef73 (the zip shamble-v2-04c1-the-scene.zip). Confirm you have this version: the README's first line names commit 946fef73a, and a readout line ends with "commit: 946fef73a". Then the README's steps 1 to 6, each with its expected result.
2. After your Advisor opens the pull request from its own branch at 946fef73 and its checks are green: if every step passed, say "merge" for that pull request, in your own words, in the chat of the session that merges. It merges by merge commit with 946fef73 as the expected head.
3. If you hold back the go: the scene and its doc edits then ride in 4c-2's pull request.
