# The main session's checks of the verifier's H1 (2026-11-09, head 198c075 + notes f1dfa2e)

git 2.43.0. Scratch repos under the session's scratchpad (`pidcheck1`-`pidcheck5`), outside the repo.

1. **H1 holds (own script, no binary file).** A fake revert of a merge that added `"Bash(curl:*)"` to a deny list,
   which also turns `"Bash(rm -rf:*)"` into `"Bash(rm-rf:*)"`: `--stable` inverse 0311f7f6d472, fake 0311f7f6d472
   (equal); `--verbatim` inverse 0d1d569e0305, fake 568708cb9075 (different). Trees differ. git refuses
   `--stable --verbatim` together ("--verbatim is incompatible with --stable").
2. **The verifier's `pid.sh`, re-run as it stands now:** its first "tamper" also changes a binary file, so its ids
   differ from the inverse under both flags; its "tamper" and "tamper2" (binary change only) share the `--stable` id
   3ddce8883e42 and differ under `--verbatim` (a2e97545dfc8, 67360025d966). Same conclusion; the numbers in
   `verifier-fixpass.md` (50afd389efe7 "both") come from an earlier version of the script.
3. **`pid2.sh` re-run:** an honest `git revert -m 1` after a later merge shifted STATUS lines: `--stable` and
   `--verbatim` both give equal ids (8178535f5d4a; bfb4ad1eacae) while the plain diffs differ. `--verbatim` keeps the
   offsets ignored.
4. **Binary files, checked and not needed:** for a binary file, patch-id hashes only the `index` line's blob ids,
   abbreviated. A crafted blob whose id shares the original's first 4 hex characters, under `core.abbrev=4`, still
   gave a different `--verbatim` id (inverse e07df547285a, fake 40fe9569c73d): git lengthens an abbreviation until it
   is unique among the objects it has, and the advisor's clone holds both blobs. `--full-index` is not needed. The
   repo tracks no binary file today (`git ls-files`, 0 matches for images, audio, wasm, zip).
5. **File order:** `--verbatim` implies `--stable` (file order does not change the id: sorted and reordered patches
   give 5a20d1e3a057 both; `--unstable` differs). The verifier's "order-sensitive" aside is wrong, and harmless: both
   sides are `git diff` output in the same path order.

Fix as the verifier gives it: `--verbatim` in place of `--stable` on both sides of advisor.md:285-286.
