Task: infra-engineer review of foundation 2.1a, PR #50, pinned 3c98ad5 vs main bb2d817 (CI and the gate on old bases).
Plan (blocks, in order):
B1 read diff of tools/gate.mjs, tools/smoke.mjs, village.py, README
B2 read .github/workflows/gate.yml (base selection, smoke job root, install flags)
B3 read e3-bases.md, e3-summary.md (evidence for bases)
B4 reproduce key cases in throwaway worktrees: 2.1b PR (root base -> legacy head) and revert (legacy base -> root head) with new gate; main's gate on legacy base
B5 --omit=optional / --no-smoke path and the guarded import
B6 Chromium pin, CI runs, skip-ci heads, last commit
B7 old bases 44446a5, 8e58937, exit-2 msg
B8 write verdict
Notes:
- B1 done. gate.mjs:64-70 guarded import of smoke.mjs runs findBuild before require of battery/acorn; smoke.mjs top-level imports are node builtins only, playwright is required lazily in runSmoke (smoke.mjs:141) -> folder rule applies under --omit=optional. die() = exit 2 (gate.mjs:40). basePrefix gate.mjs (BASE_PREFIXES ['legacy/','',OLD_PREFIX]) needs all five FILES (gate.mjs:37). Cosmetic: smoke.mjs:38-39 rel() strips repo-root URL, so under legacy/ console locations read legacy/... (fine).
- B2 done. gate.yml unchanged (git diff bb2d817 3c98ad5 -- .github empty). Gate job: head gate.mjs from working tree; base PR origin/$BASE_REF, push BEFORE if commit exists else merge-base origin/main; --no-smoke; npm ci --omit=optional. Smoke job: gate.mjs --smoke-only (findBuild picks folder). Next B3.
