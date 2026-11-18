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
- Block 5 (Q4/Q5) evidence: the header says "The script trusts that record; the review of the pull request reads it."
  (checkpoint.mjs:22). No rule says so: advisor.md at 4877599 has no line on manifests, lockfiles or the record (grep
  "lockfile|manifest|depend|Cargo|package.json|supply": only the gate files list at :42), the rule line
  (CHECKPOINTS.md:60-69) names no reader, and CHECKPOINTS.md section 9.2 deletes `.checkpoints/<task-slug>/` (the
  record with it) in the last commit before the PR is marked ready; 9.1 folds "the steps with their SHAs, the key
  numbers, the decisions and the open findings", not the record. So at the ready head the record exists only in
  history. F3 (Medium, process; text he approved, so a follow-up, not a rewrite here).
- Source SRC-A (read 2026-11-18): GitHub Docs, "Workflow syntax for GitHub Actions",
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax, section
  `on.push.<branches|tags|branches-ignore|tags-ignore>`: with neither filter defined, "the workflow will run for events
  affecting either branches or tags." toolchain.yml:9-11 is `on: push:` with no filter, so a tag push runs it.
- F4 CONFIRMED (followtags.mjs on head.mjs): with `push.followTags=true`, an annotated tag on an intermediate commit
  that adds Cargo.lock (the head reverts it, so the tree diff is empty and nothing is held): "saved and pushed", exit 0;
  the origin received the tag and its tree holds Cargo.lock; by SRC-A that tag push starts toolchain.yml on that
  commit (fetch with network, then build). push() (checkpoint.mjs:171, pre-existing) has no --no-follow-tags; the hold
  checks only the pushed head's tree. This clone: push.followTags unset (git config exit 1). Needs a session to make
  an annotated tag AND set the config: by accident very unlikely. Low; follow-up.
- No .github/dependabot.yml (ls-tree .github: gate.yml, toolchain.yml only): no bot pushes dependency changes.
- Q6 (CI, security only): cache-mode: none closes my A1 cache channel (cross-run poisoning). The new lines run inside
  the cut as uid 1001 with no network, on code that already runs there: write mode (ts_export.rs:76-104,
  layout_ts.rs:~45-83) writes only <Name>.ts / layout.ts and deletes only .ts entries in its own folder after
  canonicalizing, so it cannot plant the JavaScript that post steps run; any build script or test in the first
  `cargo test` already had that reach. git status reads only the checkout (persist-credentials: false, no token in
  .git/config); tsc from the lockfile's typescript 6.0.3 (`npx --no`) executes no project code; --ignoreConfig keeps a
  committed tsconfig out. Nothing new opened. Unchanged residual: checkout's and setup-node's post steps run after the
  cut with network and the read-only token, executing files uid 1001 can overwrite (CI-CUT part 2). One premise moves:
  my accepted Low S1b (a committed crate-folder link moves write mode's root outside the tree) was accepted as needing
  "a manual write run"; CI now runs write mode on every push. Bounded by the throwaway VM and the cut, and test code
  has the same reach, so no change asked; one report line.
- The five names are the right chokepoint for FETCHED code because CI fetches with `cargo fetch --locked` and
  `npm ci --ignore-scripts` (toolchain.yml:57-60) and builds with --locked / npm ci --offline: a manifest change that
  needs new packages fails unless Cargo.lock / package-lock.json change too (and those are held). A workflow edit
  (not held) could drop --locked; .cargo/config.toml `paths` overrides swap a locked crate for committed source
  without a lock change (repository code, not fetched).
- Source SRC-B (read 2026-11-18): The Cargo Book, "cargo fetch", https://doc.rust-lang.org/cargo/commands/cargo-fetch.html,
  --locked: "Cargo will exit with an error when either of the following scenarios arises: The lock file is missing.
  Cargo attempted to change the lock file due to a different dependency resolution."
- Source SRC-C (read 2026-11-18): npm Docs, "npm ci" (v11), https://docs.npmjs.com/cli/v11/commands/npm-ci: if the lock
  and package.json do not match, "npm ci will exit with an error, instead of updating the package lock"; "It will never
  write to package.json or any of the package-locks: installs are essentially frozen."
  -> With SRC-B/C, the root lockfiles are the chokepoint for new fetched code in CI as written today.
- replace.mjs (intent-only route): a local refs/replace on the dependency commit did not reach the hold: read-tree HEAD
  reads through the replacement, so the pushed head had no Cargo.lock (origin's tree: no lockfile). Not in the list.
- This clone is shallow (`git rev-parse --is-shallow-repository`: true; fetch refspec +refs/heads/*). A new branch
  whose fork point lies outside the fetched history fails merge-base -> held "could not be read" (reasoned, not
  tried; fail-safe; same message issue as F5a).

## Findings (final, ranked; all below High, so by his words of 18:52 each is a follow-up)
- F1 Medium. checkpoint.mjs:123 and :127 split the name list that run() (:60-67) returns TRIMMED (:63) and UTF-8
  DECODED (:62), so the path checked is not always the path pushed. Repro (harness '^ws-'): main holds a/Cargo.lock=C;
  the push adds " a/Cargo.lock" (root folder " a"; also \t \n \r \v \f) with new content, no record: "saved and
  pushed", exit 0. Same for " crates/sim/Cargo.toml" with a new dependency; one record line for a/Cargo.lock also
  released " a/Cargo.lock"; and (F1b) a folder "\xff" whose U+FFFD twin on main holds the same content. Reach: only
  the first changed path (byte order) and only ASCII whitespace in practice (or a non-UTF-8 name with a U+FFFD twin);
  no such folder exists; workspaces are crates/* and packages/* (Cargo.toml:3, package.json "workspaces"), so CI would
  not build such a folder without a root manifest change, which is held. Same class and reach as the ":" hole fixed by
  KIT-FIX. Follow-up line: "checkpoint.mjs: read the hold's -z name list raw (no trim, no UTF-8 decoding: split the
  Buffer on NUL and pass each path back as given, or hold any name that changes when trimmed or re-encoded), with a test
  for a root folder ' a' beside main's a/Cargo.lock."
- F3 Medium. checkpoint.mjs:22 says "The script trusts that record; the review of the pull request reads it." No rule
  makes any review read it (advisor.md: no line; CHECKPOINTS.md:60-69 names no reader) and CHECKPOINTS.md section 9.2
  deletes .checkpoints/<slug>/, the record with it, before the PR is marked ready (9.1 does not fold the record). A
  record written too early, or for other content, goes unchecked. Text he approved: follow-up line: "advisor.md PR
  review and CHECKPOINTS section 9: a PR that changes a manifest or lockfile quotes each dependency-check record (with
  the commit it was pushed in) and the security-auditor answer it names in its ready report; the review checks every
  changed manifest at the head against those lines or main."
- F2 Low. checkpoint.mjs:125-126 read the record with `git show <rev>:<path>` whatever the entry is: a LINK whose target
  text is a release line, or a FOLDER holding an entry named as one, releases (harness: both pushed). Intent only (the
  record is trusted). Follow-up: "read the record only when ls-tree shows a plain file (100644/100755)."
- F4 Low. push() (checkpoint.mjs:171, pre-existing) sends followed tags when push.followTags=true: an annotated tag on an
  intermediate commit that adds Cargo.lock went out with the head that reverted it (followtags.mjs), and toolchain.yml
  (:9-11, no filters) runs on tag pushes (SRC-A). Needs a tag plus the config (unset here). Follow-up: "push() passes
  --no-follow-tags, so a save publishes only the checked head."
- F5 Low, wording (rule line and header; the code is fail-safe in each):
  a. "(what this push changes could not be read)" (:124) comes with the advice to write dependency-check.md (:221-223),
     which cannot release it; the way out is `git fetch origin main` (or pushing the branch once by hand).
  b. "as sha256sum prints it from the repository root" (CHECKPOINTS.md:65-66, checkpoint.mjs:21): the script hashes the
     blob in the commit (:136-138); sha256sum reads the working file and escapes paths with "\" or a newline (tried:
     "\<hash>  a\\b/Cargo.lock"), so such lines never match. Exact form: `git show <commit>:<path> | sha256sum`.
  c. "or with main for a branch not yet pushed" (CHECKPOINTS.md:62-63) is the merge-base with origin/main (:122).
  d. "never pushes" (CHECKPOINTS.md:60-61; checkpoint.mjs:17-18): not in F1's corners.
  e. Links, submodules and a file-to-folder change can never be released by a record (:118, :129-132); the rule line
     does not say so.
- Pre-existing, not this PR's lines: onlyCheckpointsAhead (:108-113) uses rename detection, so a code file moved into
  .checkpoints/ goes out with [skip ci] (both scripts, harness q3; advisor E7).
- Info: S1b premise (CI now runs write mode on every push), see the Q6 block. No change asked.

## Answers, one line each
1. Yes, in two corners only (F1): a manifest under a root folder whose name starts with ASCII whitespace, and a
   non-UTF-8 folder with a same-content U+FFFD twin on main. Every other probe (about 85 runs: ":" and glob forms,
   spaces, quotes, newline, non-ASCII, case, renames, copies, merges, link/folder/submodule, mode, empty, 65 MiB,
   delete+re-add, new branch, missing origin/main, record forms, links, other slug, races, git failures) held, or
   released only what his rule releases.
2. Every git failure tried ends held (rev-parse, names diff, record read, both ls-trees, cat-file, merge-base, a hang
   killed after 5 s). Nothing fails open on an error; F1 is a wrong path, F4 a ref the hold does not look at.
3. No change for pushes without a manifest change (outcome, exit code, [skip ci], secret scan, watch exits, the last
   save on SIGTERM: identical on main's and the head's script), except a new branch in a clone with no origin/main or
   with unrelated history: now held "could not be read" (fail-safe; misleading advice, F5a). The [skip ci] quirk for
   a file moved into .checkpoints/ exists in both.
4. Sound in binding: the script hashes the blob in the commit to push, so one line releases one path at one content and
   any later change re-holds; exceptions are F1's corners and the accepted ":" reading. The reviewer must read every
   changed manifest at the head against main or a line, the record from history (deleted at 9.2) as a plain file, the
   named security-auditor notes and their timing, and whether any manifest change reached GitHub before its record
   (plain pushes are not held). The rule line says how to write the record, not who checks it (F3).
5. More than the code: "never pushes" (F1), "as sha256sum prints it" (escaped paths, working file vs blob), "the
   review of the pull request reads it" (F3). Less: links, submodules and folders can never be released; "with main"
   is the merge-base; unreadable bases hold code-only pushes too.
6. Nothing new opened: cache-mode none closes the cross-run cache channel; the new lines run inside the cut on code
   that already runs there and cannot reach the post steps' JavaScript; the residual is CI-CUT part 2 (post steps
   after the cut). The S1b premise ("a manual write run") no longer holds; bounded by the VM and the cut.
7. See "Not covered" below.

## Not covered (accidental routes; for the report and the gate v2 list; no design)
- A plain git push or the GitHub tools; above all pushing code by hand while a held checkpoint commit sits under it
  (no lasting flag; for the saver the only sign is one line in a background log; held notes stay local).
- Running the change before any push: npm install (install scripts), cargo build/test (build scripts, proc macros),
  npx / cargo install (no manifest changes at all). The hold only times the push.
- Files outside the five names that change what CI fetches or runs: workflows and actions, rust-toolchain.toml,
  .cargo/config.toml (paths overrides, source replacement), .npmrc, .gitmodules, vendored source; and dropping
  --locked / npm ci in a workflow (he declined a longer list now).
- Other lockfile names and case variants (yarn.lock, pnpm-lock.yaml, cargo.lock, Package.json): not read by Linux CI
  today; read by other tools or on a case-insensitive checkout.
- Intermediate commits: add-then-remove in one push goes out unheld; CI builds only pushed heads, but a followed tag
  makes it build that commit (F4).
- The clone's view: a stale origin/<branch> (remote branch deleted or reset without a fetch) re-pushes what that view
  held; a stale origin/main releases what main had at the last fetch.
- Branches without this script: a branch from before the merge (or not taking main in) runs its own older
  checkpoint.mjs; a saver started before keeps the old code until it stops (at most 115 min).
- The record is trusted and unreviewed by rule (F3), may be a link or folder (F2), and releases content, not a change
  (content checked against an older main stays released after main moves).
- Misread paths: F1's corners; ":" paths (accepted, errs to hold).
