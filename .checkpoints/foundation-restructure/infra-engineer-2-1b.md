# infra-engineer, review of foundation 2.1b (PR #51), round 1

Head seen: 722d2c1 (checkpoint-only commit on top of dc8582a; non-.checkpoints diff identical to dc8582a).
Base: 76b4f7a.

## Plan
1. gate.yml base logic (push BEFORE / PR origin/$BASE_REF) on the move and on the revert (E8).
2. gate.mjs move check, BASE_PREFIXES/basePrefix; smoke.mjs findBuild; how a pre-move PR still gates.
3. CI run headers (#674/#675 ... #681/#682) via gh.
4. Playwright cache key/paths in gate.yml vs the move.
5. Merge order with PR #40 and #48 (files overlap).
6. Windows step in gate.yml (`node tools/gate.mjs origin/main`).
7. Cost/runtime per run; rollback.

## Findings (a line each, as found)
- (guard) refused: `gh api ... > .gate/infra-2-1b/job-$j.log` in a loop (variable redirect target). Went on with pipes.
- CI headers checked from job logs: #674 push gate "base 2f4b974 (files at the root) | head = working tree (files in legacy/)" PASS;
  #675 PR gate on refs/pull/51/merge 692a9f2, "base origin/main = 76b4f7a (files at the root) | head (files in legacy/)" PASS;
  #674 smoke "head (files in legacy/)" 8/8 5/5, cache hit playwright-chromium-Linux-1.56.1; #694 (bf41334) gate+smoke same, cache hit.
- Last CI'd commit bf41334 (#693/#694); dc8582a and 722d2c1 are [skip ci] checkpoint commits; diff bf41334..HEAD outside .checkpoints empty.
- Run times: gate job ~10 s, smoke ~45-57 s; whole run ~1 min. Move changes neither.
