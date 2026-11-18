# security-auditor, review round 1 of kit-hold-ci (PR #60)

Head reviewed: 4877599a9b85cfcc10b147a69e44a5276df5837e (branch claude/wonderful-hamilton-jdmlnf), base main 9b4b518.
The working tree's HEAD is 23e0547 (an autosave after 4877599; it adds only two notes files under .checkpoints/:
`git diff --stat 4877599 23e0547`). Script read with `git show 4877599:.claude/scripts/checkpoint.mjs`.
Started 2026-11-18 (after 18:04 UTC). Memory read: .claude/agent-memory-local/security-auditor/MEMORY.md.

## Threat model (what the hold is for, and against whom)
- Asset: CI (toolchain.yml) must not compile a dependency change before security-auditor has answered on it. CI starts on
  any push whose head commit lacks "[skip ci]"; checkpoint.mjs's foreground save pushes every local commit (cs1 miss 1).
- Actor: the session itself, by ACCIDENT (Mustafa's scope: the hold is a seatbelt against a save that pushes too early).
  Not in scope (accepted by him): a session acting by intent (plain git push, GitHub tools, a forged record).
- So the questions are: (1) does any path through save/watch push a changed manifest/lockfile (five names, any folder)
  whose content is neither recorded nor equal to origin/main at the same path; (2) does every error end in "held";
  (3) is behaviour for pushes without a manifest change unchanged; (4) does the record bind exactly one content to one
  path; (5) do the words claim what the code does; (6) do the CI edits open anything (security read only; infra-engineer
  owns the depth); (7) what the hold does not cover by accident.
- Fail-open candidates to probe: git output post-processing (run() trims stdout; UTF-8 decoding of -z output), pathspec
  magic in the first ls-tree (accepted, known), base selection (remote-tracking ref, merge-base, stale/missing refs),
  record parsing (CRLF, spaces, links, folders), type changes, races between commit and push.

## Plan
1. Read the diff, the script at the head, his three messages, the card, advisor notes, kit-fix2.diff, ci-evidence,
   infra-engineer's notes. DONE.
2. Run the 33 tests at the head; run the given probe2 on the head's script.
3. Scratch harness under the scratchpad (never this repository): one fixture per probe family; run the head's script copy.
   Families: odd paths (":" forms, leading/trailing whitespace, quotes, newline, non-ASCII, invalid UTF-8, case);
   renames/copies/merge; type changes (link, folder, submodule), mode, empty, large (>64 MiB); deleted+re-added;
   new branch; missing/stale origin/main; branch changed before push; record forms (CRLF, spaces, two spaces, dups,
   BOM, link, folder, secret-flagged, another slug); git failures.
4. Q3: behaviour for no-manifest pushes vs main's script (same harness, both scripts).
5. Q4/Q5: record soundness; read CHECKPOINTS.md section 3 and section 9, advisor.md for who reads the record.
6. Q6: read toolchain.yml whole at the head.
7. Findings with severity, then the handback.

## Findings (each written as found)
- Block 2 done: the 33 tests pass at the head (node v22.23.4, git 2.43.0; kit and CI files at the working tree's HEAD
  23e0547 equal 4877599). Scratch: /tmp/claude-0/-home-user-Shamble-Frontlines/f707c56b-26b4-5848-b1e1-237a99a6a881/scratchpad/sec-review1/
  (head.mjs = 4877599's script, sha256 fc5a4f02...; main.mjs = 9b4b518's script, 6bf8696d...).
- Lead F1 (to confirm end to end): run() (checkpoint.mjs:60-67, pre-existing helper) returns `out.trim()`. The new
  names call (checkpoint.mjs:123) reads `git diff -z` through it, so a FIRST path that starts with whitespace loses it:
  trimcheck.mjs printed raw " a/Cargo.lock\0.checkpoints/test.md\0" -> after trim+split ["a/Cargo.lock", ...]. JS trim()
  strips U+0009-000D, U+0020, U+00A0, U+1680, U+2000-200A, U+2028/2029, U+202F, U+205F, U+3000, U+FEFF. The NUL at the
  end protects the last path; only the first path (sorted by bytes) is hit.
