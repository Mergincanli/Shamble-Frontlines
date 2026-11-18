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
- F1 CONFIRMED (harness.mjs '^ws-' on head.mjs): RELEASED with no record (exit 0, "saved and pushed") for a root folder
  named " a", "\ta", "\na", "\ra", "\va", "\fa" holding Cargo.lock=D when main holds a/Cargo.lock=C (unchanged in the
  push); also " crates/sim/Cargo.toml" with a new [dependencies] line beside main's crates/sim/Cargo.toml. One record
  line for a/Cargo.lock (content E) also released the changed " a/Cargo.lock" (one line, two paths and contents).
  Held: a U+00A0 folder (sorts after .checkpoints/, so not first) and " x/Cargo.lock" when x/Cargo.lock does not exist
  (but the message names "x/Cargo.lock", and the true sha256sum line cannot release it; only a false
  "deleted  x/Cargo.lock" line would). Cause: checkpoint.mjs:123 + :127 split run()'s trimmed output (run(): :60-67).
- Block 3a done (harness, head.mjs; 27 scenarios): HELD as intended, no record: root folder ":" and ":a", "::a",
  ":(top)a", ":(literal)a", ":(top,literal)a", ":!a", ":(exclude)a", ":(glob)a", ":(icase)a", "*", "?", "[a]" each
  beside a same-content twin on main; ":a" with its true sha256sum line (known: errs towards holding); rename
  a/Cargo.lock -> b/Cargo.lock (both named); copy to b/Cargo.lock; merge of another unpushed branch with a lockfile;
  file -> link (record lines for target, link text and "deleted" all refused); file -> folder (unreleasable even by
  "deleted"); gitlink (held even with "deleted"); empty lockfile; a 65 MiB lockfile with its true line (maxBuffer:
  held, unreleasable). PUSHED as intended: take-main-in merge; mode-only change with main's blob (accepted); empty
  lockfile with e3b0... line; deleted+re-added same content and added+removed (tree to tree: no change, [skip ci]:
  the intermediate commit is published but CI builds only the head).
- Block 3b done (24 scenarios): F1b CONFIRMED, same cause class as F1: run() decodes stdout as UTF-8
  (checkpoint.mjs:62 `encoding: 'utf8'`), so a path byte 0xff becomes U+FFFD. Main holds "�/Cargo.lock"=C, the
  push adds "\xff/Cargo.lock"=D with no record: RELEASED (ls-tree reads the U+FFFD twin, equal to main). Alone, the
  \xff folder is held, but the true sha256sum line cannot release it (the message prints U+FFFD).
- Record forms: CRLF, trailing spaces/tab, leading spaces, BOM, duplicates, a path with two spaces, é/ folder: released
  as meant. One space, "hash *path" (-b), uppercase hex, "./" prefix: held. Another slug's record: held. The notes
  folder as a link: held. A record that is a LINK whose target text is a release line, or a FOLDER holding an entry
  named as a release line: RELEASED (git show prints the link text / the tree listing). Intent-only (the record is
  trusted by design), but a PR reviewer must check the record is a plain file. L-level.
- Bases: new branch + origin/main present: held as meant. New branch with origin/main deleted, or with unrelated
  history (orphan), code only: held with "(what this push changes could not be read)" and the advice to write
  dependency-check.md, which cannot release it (no names to match); main's script pushed these. Fail-safe; message
  wording Low (the way out is `git fetch origin main`). Branch on the remote + origin/main deleted + main's lockfile
  taken in: held (no main release without the ref): fail-safe.
- Backslash folder a\b/Cargo.lock: sha256sum prints "\<hash>  a\\b/Cargo.lock" (escaped, leading backslash), so the
  rule's "as sha256sum prints it" cannot release it: held. Same for a newline in the path (split on \n). Fail-safe; the
  rule line is not exact for such paths (Low). Case variants cargo.lock / Package.json: pushed (by design, five exact
  names; Linux CI reads neither). A note named .checkpoints/test/Cargo.toml: held (any folder includes the notes).
- Block 3c done (git failures through a PATH shim, shim/git): with a lockfile NOT on main and no record, every failure
  ends HELD: rev-parse of the remote branch (falls back to the merge-base), the names diff ("could not be read"),
  git show of the record, the first ls-tree, the literal ls-tree, cat-file, merge-base (new branch); a hang in
  cat-file killed after 5 s: nothing pushed. With the lockfile equal to main, failures of rev-parse, git show and
  cat-file still release it (the main rule needs none of them): correct. No fail-open from a git error found.
- Block 4 done (Q3, harness '^(q3|...)' on main.mjs and head.mjs): identical outcome, exit code and [skip ci] for:
  notes only (pushed, skip ci), code ahead foreground (pushed, no skip), new branch notes only (pushed), a secret-
  flagged note beside a good one (pushed, exit 1, "flagged notes left out"), nothing to save (exit 0), watch with code
  ahead + SIGTERM (not pushed, exit 0), watch SIGTERM before the first tick (last save pushed, exit 0). The only
  difference: a new branch whose clone has no refs/remotes/origin/main, or with unrelated history: main's script
  pushes, the head holds ("could not be read", exit 1). Fail-safe; the first is known (advisor), the orphan case is
  the same class. Pre-existing in BOTH: a code file moved into .checkpoints/ is pushed WITH [skip ci]
  (onlyCheckpointsAhead :111 uses `git diff --name-only` with rename detection on; advisor's E7). Not this PR's lines.
- Race (race.mjs on head.mjs): 45 foreground saves in 15 rounds of 3, racing watch --every 0.01, notes changing:
  every save exit 1, the origin never received Cargo.lock. Record written: the saver alone did not push in 3 s (code
  ahead: "push them yourself"); one foreground save then pushed (exit 0). The pushed SHA is always the checked one
  (push(branch, commit.out), :230), so a race can only choose between checked commits.
