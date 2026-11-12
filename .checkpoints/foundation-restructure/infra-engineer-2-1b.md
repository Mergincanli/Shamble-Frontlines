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
- Merge order (git merge-tree, no worktree): #48 (d8186fd) into the PR head: clean, legacy/ intact, no shared file. #40 (a70ef74)
  into main 76b4f7a already conflicts (DECISIONS.md, STATUS.md); into the PR head: the same two conflicts, REPO_MAP and
  STATUS_ARCHIVE auto-merge. #51 adds no new conflict file for #40. #40 mergeable_state "dirty" now, so its pull_request CI does not run until resolved.
- Pre-move PR probe: unreferenced commit 454e2cd = 76b4f7a + one comment line in root shamble-game.js; merge-tree onto the PR head
  (722d2c1) puts the edit in legacy/shamble-game.js, no root copy; probe merge 6c6f72e in throwaway worktree .gate/infra-premove:
  `node tools/gate.mjs 722d2c1` (CI's PR base after the merge) GATE PASS, all 8 checks, "(files in legacy/) | head (files in legacy/)".
  Worktree removed afterwards (guard refused `--force`; removed the node_modules symlink, then plain remove).
- New commit 8b97b5b (autosave, [skip ci]) appeared during review; non-.checkpoints diff 722d2c1..8b97b5b empty.
- Ruleset on main (gh api rules/branches/main): required checks gate + smoke bound to integration 15368 (GitHub Actions),
  strict=false; allowed_merge_methods merge, squash, rebase. So a [skip ci] final head cannot merge; CHECKPOINTS.md:204 makes the
  checkpoint-deleting commit run CI. Not a finding.
- Repo public: standard runners free; billable 0 ms on #694; run ~60 s. Move changes cost 0.
- Playwright cache: key playwright-chromium-Linux-1.56.1 from tools/package-lock.json, untouched by the PR; hits on #674 and #694.
- gate.yml names no build path; .github untouched; nothing to change. PR Advisor's mergeAllowed (routine-guard.mjs:84-99) lists neither
  root nor legacy/ game files: unchanged reach.
- E8 at f0322d2 (kit lines included); later commits are docs only: gate result of a revert unchanged. Revert CI path: PR base
  origin/main (legacy/) vs merge-ref (root): move check sees no legacy/ folder; push to main BEFORE=M same.

## Findings
- F1 Low: the pure-revert window closes once #40 merges (shared DECISIONS/REPO_MAP/STATUS/STATUS_ARCHIVE; advisor.md:297-298
  exempts only STATUS and ROADMAP); report Risks :72-75 do not say so.
- F2 Low: merge method unstated; ruleset allows squash/rebase; E8 rehearsed a merge commit (`revert -m 1`); #33 was squashed.
- F3 Low: Windows step (report :124-128) expects "76b4f7a" literally; if #48/#40 land first the SHA differs.

## Verdict: APPROVE (three Lows, STATUS follow-ups or a wording touch if a fix pass happens anyway)

## Additions after the advisor-tool check
- #51: mergeable true, mergeable_state "blocked" at head 8b97b5b (draft; no checks on the [skip ci] head). No conflict with main.
- #40 checked at a70ef74 (my fetch advanced origin's ref 063af69..a70ef74); the four shared docs files hold there too.
  Seen: only push runs for #40's branch in the last 30 runs (#672, #673); "PR CI does not run while dirty" is read, not observed.
- Recommended order: #51, then #48 (clean onto the head), then #40 (already conflicts with main; resolves after either).
- Read, not run: the Windows step (report :124-128); the push path for a pre-move branch (gate.yml:42-48). tools/package* untouched,
  so the cached Chromium for Playwright 1.56.1 on his laptop still matches.
- Side effects: `git fetch origin claude/netcode-proof-test-plan-hcky5g` (remote-tracking ref only); unreferenced probe objects
  8f9119b (blob), 0754116 (tree), 454e2cd and 6c6f72e (commits), 7aca1f5 (tree): loose, reachable from no ref, removed by gc.
