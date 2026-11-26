# security-auditor: tier-2 review of the preview hand-over texts (worktree .gate/ci4, head 13ce313, main 2483e9a)

## Plan
1. Rebuild toolchain.yml and playwright.config.ts with `patch` in the scratchpad; compare sha256 with the stated values.
2. A: `git ls-remote` for upload-artifact v7 tags myself; read action.yml, src/shared/search.ts,
   src/upload/input-helper.ts at the clone (HEAD check), plus how the token is used (dist).
3. B: (i) absolute paths, (ii) no script step, (iii) action code / post steps in the same-uid work folder.
   Check the runner layout claims (_actions folder, ownership) against docs where possible; mark inferred.
4. C: GUARD-STEP; D: HIDDEN-FILES; E: exposure, `if:`, comments' truth; lane 2's preview.yml if readable.
5. Findings, one line each as found, then a summary.

## Notes (as found)
