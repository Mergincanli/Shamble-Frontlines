# Advisor: U7's builder fast-forward and the keep-list (10:50 UTC window, 2026-11-27)
Advisor agent, day session, effort xhigh, read only (no cargo, no git write). Acting by main's advisor.md, re-read by
`git show origin/main:.claude/agents/advisor.md` (unchanged on main since c9bd268, 2026-11-12).

## Plan
1. Read C in launch-1050-msgs.md, the last M9 line, CLAUDE.md's protocol item 3 (on main).
2. Re-check by output-carrying forms: the 45 paths' blob ids against the worktree (hash-object vs ls-tree), the kit
   paths 80b6748..0efbbef and b9aad01..0efbbef, the earlier fast-forward's kit paths (precedent), ffcheck.
3. Rule block 1 (first line go / fallback (b) / his), then block 2, then Decisions needed, For Mustafa, Status.

## Notes
- C (launch-1050-msgs.md:111-136): the 45 staged paths are two notes under .checkpoints/netcode-proof-test/ and 43
  under spikes/netcode-proof/; none is a kit or gate path.
- The fast-forward's kit paths, nested names included (`git diff --name-only 80b6748 0efbbef --` with `:(glob)**/`
  pathspecs for CLAUDE.md, CLAUDE.local.md, AGENTS.md, .claude/, .github/ and tools/gate*): .claude/hooks/routine-guard.mjs,
  .claude/hooks/routine-guard.test.mjs, .github/workflows/toolchain.yml. Nothing else. Re-run against the guard's own
  list (origin/main:.claude/hooks/routine-guard.mjs:28-47, `protectedRel`: .claude/, .github/, .git, the gate files in
  tools/, and CLAUDE.md, CLAUDE.local.md, AGENTS.md, .npmrc, .mcp.json at any depth), with `.claude/`, `.github/`,
  `tools/` and `:(glob)**/` for the five names: the same three paths.
- Their blob ids, `git rev-parse` at b9aad01 (main) and at 0efbbef (the target), in pairs: 4bb64117… = 4bb64117…,
  9a9ffd0b… = 9a9ffd0b…, 4f2e88d9… = 4f2e88d9…; `git diff --name-only b9aad01 0efbbef -- <kit pathspec>` printed nothing.

## U7's builder fast-forward, ruled
go

Why. His rule (main's CLAUDE.md, protocol item 3) bounds the edit it covers: "an edit to a kit or gate file (…) made
with a file tool or a shell write asks Mustafa first, in any worktree of this repository (…); agents and teammates may
not edit them". "Them" is that same edit. C's step 2 (`git merge --ff-only 0efbbef…`) is a git command, neither a file
tool nor a shell write, and no byte of a kit file is authored in it: the three kit files it checks out are, blob for
blob, main's reviewed files (the pairs above).
This is the reading already in force, not a wider one. Main's reviewed guard says the same in its own header and code:
- origin/main:.claude/hooks/routine-guard.mjs:11-12: "day sessions only (…): file-tool and shell writes to kit and gate
  files ask first, and so do the git and gh commands that would move the main folder off main's reviewed rules; a
  subagent (agent_id set) is refused both". Two classes, and the git class is the one that moves the main folder off
  main's rules.
- :1390-1392: `function mainFolderMove(…)` returns no decision unless the command's git top is the project folder
  (`if (!sameDir(gitTop(vcwd, io), io.projectDir, io)) return null;`); :1444 refuses a subagent's move of the main
  folder ("A subagent never moves the main folder").
- origin/main:CLAUDE.md:301 (the Never bullet): a worktree is where off-main work goes, and in the main folder "Still
  allowed there (main's reviewed rules)" names `git merge --ff-only origin/main`: a fast-forward onto main's reviewed
  kit files is the form his rules treat as safe.
C's merge runs in the builder's worktree, not the main folder, and moves that worktree onto main's kit bytes (from
80b6748's older ones), so it meets neither class. If the guard reads the worktree as the subagent's project folder, its
:1427-1428 rule refuses a merge whose target is not origin/main; C already says a refusal stops there, quoted, and
fallback (b) holds. The builder's earlier fast-forwards (9177fb5..80b6748 and 577816c..7e2d6f3, each bringing
.github/workflows/toolchain.yml) count only as "nothing refused then", not as a ruling.
Conditions: none. C goes as written (no command added, target unchanged).
One line for the day message (a record, not a decision): "U7's builder fast-forward brought main's three kit files,
blob for blob, into its own worktree by git, not by a file tool or shell write; ruled outside the kit ask
(advisor-u7-ff.md)."

## The keep-list, ruled
accepted

Checked here, output-carrying forms only (11:0x UTC):
- `sha256sum` of u7-kept-list.md and of the tool's saved output
  /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tool-results/bu4z9dn2h.txt
  (the only saved output of 59,810 B = 58.4 KiB) printed the same hash twice:
  f6bb877246a1145d2b44728342323eb019e71a22df31fe6bdeaf55e744eb620d.
- `wc -l` of the list: 332. `find .gate/u7/kept-agent-a9873 -type f | wc -l`: 332.
- `sha256sum -c u7-kept-list.md | grep -c ": OK$"`: 332, so every kept file still matches its line today.
Why accepted: my form asked for the list "written by Write" so that no shell redirect authors a file. The `cp` copied
the command's own output byte for byte (the hash pair above), so the list says exactly what `sha256sum` printed, no
more, and no word in it was written by the session. The departure is from the form's letter only, and it is already
recorded (u7-merge-plan.md:258-260, "copied by `cp` … not rewritten by Write: the same bytes"). This file now holds
the proof of "the same bytes" those lines claim.
Owed: nothing. Nothing owed needs the builder's worktree: the list covers the kept copy under .gate/u7/kept-agent-a9873/,
not the worktree, so the plain remove after C may go once C's step 3 prints nothing. One Low, not owed: the saved
output sits in this session's tool-results folder and can be cleared; the list on the lane is the durable copy.

## Decisions needed
None. Block 1 rests on the kit rule's own words and on the reading main's reviewed guard already uses
(routine-guard.mjs:11-12), not on a wider reading; block 2 departs from my form's letter only.

## For Mustafa
Nothing.

Status: COMPLETE 2026-11-27T11:05Z
