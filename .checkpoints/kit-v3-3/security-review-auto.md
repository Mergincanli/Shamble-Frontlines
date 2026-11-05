Automated security review of commit f22c4df (build steps 3-5), 2026-11-05 ~01:45 UTC: four findings, all in step 4's new
day-session code in .claude/hooks/routine-guard.mjs. Recorded here so step 4b fixes each one with a test.

1. HIGH, argument injection into a guard-run git command. switchWords (:731) keeps every word after `--` as a
   positional, and mainFolderMove (:762) passes pos[0] to `git diff --name-only origin/main <pos[0]> --`. A target
   word starting with `-`, such as `git switch -- --output=<path>`, makes the guard itself run git with that option,
   so the guard writes a file. Confirmed by reading :731 and :762 at f22c4df. Fix: never pass a command word that
   starts with `-` into a guard-run git command. Resolve the ref first, with
   `git rev-parse --verify --end-of-options <ref>^{commit}`, and pass only the resulting SHA. If it does not resolve,
   ask without running git.
2. MEDIUM, a ref-name ambiguity. The exemptions compare names: `pos[0] !== 'origin/main'`, and the merge forms
   `'--ff-only origin/main'` and `'origin/main'`. A local branch named `origin/main` (refs/heads/origin/main) is what
   git resolves first, so an unreviewed branch would pass as reviewed main. Fix: compare by commit. Resolve
   refs/remotes/origin/main explicitly, and accept an argument only when it resolves to the same SHA (or require the
   fully qualified ref).
3. MEDIUM, case-insensitive file systems. isWin/fold (:64-65) fold case only on win32, and macOS (darwin) file systems
   are case-insensitive by default too. Fix: fold on darwin as well, or detect case-insensitivity at runtime.
4. MEDIUM, incomplete cwd tracking. The shell check tracks `cd` (:484) but not `pushd`/`popd`, so
   `pushd <main folder> && git switch x` after a `cd` elsewhere runs unchecked, because the guard thinks the folder
   is elsewhere. Fix: treat pushd and popd like cd, and for any directory change it cannot follow, fail toward the
   ask by assuming the project folder.
DONE
