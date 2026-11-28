# security-auditor: dependency hold for 4c, second answer (reworded root comment, deps-4c-v2.diff)

Builds on security-auditor-deps-4c.md (APPROVE with F1, F2, item 3). Read only; offline cargo read commands only.

## Plan
1. Digests: `sha256sum -c deps-4c-v2.sha256` (five files in `.gate/deps-4c`, and the patch); diff v1 vs v2 patch: only the
   root comment's lines differ; patch applied to head copies rebuilds exactly the five digests; `git apply --check`
   on the branch head a4e93679; branch manifests equal to the patch's base.
2. Re-run `cargo metadata --offline --locked` and the libm feature tree in the scratch worktree (unchanged lock: nothing moves).
3. F1, F2 re-read against the new comment.
4. Item 3 corrected by RR1 (scan rule for libm as a collision-library-like rule) and RR4 (sincos, rem_pio2, rem_pio2_large, exp);
   sincos's code read (cached libm 0.2.16 source) for anything new: feature/arch overrides, fma, get_unchecked, panics.
5. Answer, DONE.

## Notes
- [x] 1 digests. `sha256sum -c deps-4c-v2.sha256` in `.gate/deps-4c`: five OK (root fc436ec3..., lock b7c6ee5d...,
  sim c7432b03..., room 79b829a9..., client-endpoint 22e2c5bf...); the patch itself 0edaf8f3... (sha256sum of the file
  in the notes folder; the sha file names it by a branch-relative path, so -c from the scratch worktree cannot open it).
  `diff deps-4c.diff deps-4c-v2.diff`: only the root hunk header (+6,11 -> +6,12) and the comment's 3 lines -> 4 lines;
  the `libm = "=0.2.16"` line and every other hunk byte-identical. Patch applied with `patch -p1` to `git show
  a4e93679:<path>` copies (scratchpad v2/) rebuilds exactly the five digests: no sixth file. The five manifests are
  identical at 8bee8365 and a4e93679 (`git diff --stat` empty). `git apply --check` on the branch (head a4e93679): OK.
  Patch pure ASCII, no CR. Branch tracked changes: only .checkpoints (rust-reviewer's notes).
- [x] 2 cargo. `cargo metadata --offline --locked` in scratch: exit 0. `cargo tree -e features -i libm` in scratch =
  the stored after file modulo the worktree path; libm features still `default` + `arch`. Lock digest unchanged, so the
  package set, players' Wasm view and override coverage of the first answer stand unchanged.
