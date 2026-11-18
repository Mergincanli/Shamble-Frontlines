# Verifier notes: kit-hold-ci (PR #60)

Branch `claude/wonderful-hamilton-jdmlnf`, code head 4877599a9b85cfcc10b147a69e44a5276df5837e, base main 9b4b518.
Working-tree HEAD at start: 308286d (later commits touch only `.checkpoints/`, to be confirmed).
Scratch: `.gate/kit-hold-ci/verifier/` and the session scratchpad. I edit nothing else; I never run `save`/`watch`.

## Plan
1. Gate: `node tools/gate.mjs --no-smoke`; `node .claude/scripts/repo-map-check.mjs`.
2. Kit tests with counts: checkpoint.test (33), routine-guard.test (1107), settings.test (127), kept-logs.test (10/9/0);
   `git diff --stat origin/main -- .claude/hooks .claude/scripts/kept-logs*`.
3. Cross-run: new tests vs main's script (expect 27 pass, 6 fail = tests 28-33); main's tests vs new script (expect 27 pass).
4. Texts: sha256 of Mustafa's three files; kit-edit.diff at 3ebe654a (262 lines, sha 70756f3d...) applied to main = as-shown/;
   kit-change.diff = diff -u as-shown -> head, adds only, count 75 (6/67/2); kit-fix2.diff at f7e232fb (45 lines, sha 9197f070...)
   = git diff 415bd73 4877599 -- .claude/; toolchain.yml diff = report blocks 1-3; numstat 47/224/10/12 = 293/2.
5. Plant: `git diff 94c197f 415bd73 -- . ':!.checkpoints'` empty; generated files mode 100644 at HEAD.
6. Replay: replay-miss.sh with new script (exit 1, names 3 files, nothing pushed) and main's (exit 0, pushes).
7. Probe: probe-colon-folder.mjs on new script (held) and on 415bd73's (RELEASED).
8. CI claims: WebFetch public run/job pages; else consistency only.
9. Draft sentences: "What went wrong on the way", "What changed", "Evidence", "Not covered, and not verified" vs card, notes, git,
   Mustafa's three files word for word.

## Results (one block per check, written as each finishes)
