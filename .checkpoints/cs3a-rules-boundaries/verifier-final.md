# verifier-final: cs3a on code head 8619d539 (base main 1ec6f6c)

## Plan
1. Gate: `node tools/gate.mjs --no-smoke` in the shared tree (read-only); CI gate run 45435601291 via gh api (read).
2. CI toolchain job 138592422504 log: every count vs the card; run 45434633338 on a1b2e802 red for the card's reason, attempts=1.
3. Own copy `.gate/verifier-cs3a/` (worktree --detach at 8619d539, node_modules linked): run full-run.sh's steps; compare counts + Wasm sha256; both vector generators --check.
4. Plants: >= 20 of plants.py, all steps, both dev-server ones, the Wasm ones; failure = the named one?
5. Claims a-f against sources (mul_add pins + native exactness; Rust doc quote; ChaCha TC1-TC8 + sha256; size 3,561 < 4,100; ADR 0004 part 7 vs ALLOWED; no manifest/lock change since 0412814; no .github/.claude/gate/CLAUDE.md in diff).
6. Not-verified items stated and not claimed.

## Results (as they come)
- 16:24 shared tree state: HEAD f153ddf4 (notes after 8619d539); working tree has uncommitted docs/DECISIONS.md (+132, builder's docs pass, 16:23) - not in the verified diff.
- 1. gate (shared tree, --no-smoke): GATE PASS, exit 0; base 1ec6f6c; [1] 5/5 [2] PASS [3] +0 -0 core 138/game 714/editor 353 [4] 51 guarded 0 flagged [5b-d] PASS.
- 5f(part): diff 1ec6f6c..8619d539 minus .checkpoints = 56 files; none under .github/ .claude/ tools/gate.mjs docs/adr docs/research, no CLAUDE.md/AGENTS.md (grep exit 1).
- 1. CI gate run 45435601291: head 8619d539, attempt 1, success; job gate 138592423955 "GATE PASS" (base = push's before a1b2e802, not main); smoke job 138592424548 [5a] PASS index 8/8, editor 5/5, 0 errors.
- 2. toolchain run 45435601223 attempt 1 success, job 138592422504 16:18:37-16:21:42, image 20261116.133.1, Chrome 157.0.8103.78; log (1083 lines) byte-equal to builder's .gate/cs3a/ci-8619d539-toolchain.log (sha256 37ef6b7a...).
  cargo debug run1 (l.466-577): 15,3,2,2,1,15,5,1,9 (+devtools lib 0); run2 (l.616-728): same; release (l.777-822): protocol 2, sim 15, rules_scan 5, state_hash_alloc 1. 38 "test result: ok", 0 FAILED, 0 nonzero ignored.
  npm: client 70 (7 files) l.871; e2e node "Running 1 test" "1 passed" l.880-882; net 1 l.893; sim-worker 42 l.913; types 7 l.928.
  sha256 383f53039ca93651f3d1655deb956040f688b45b0a9d347199c6f08f9f833963 l.1017; "no registry path in the Wasm" l.1018; boundary 2 files 17 tests (9+8) l.1023-1027; "Running 14 tests" l.1036, "14 passed (1.1m)" l.1066. No skipped/flaky line. ALL MATCH the card.
- 2b. run 45434633338 (a1b2e802): failure, run_attempt 1, attempts/2 = 404 (not re-run); job 138588985695 16:13:42-16:16:23; 2 failed = isolation.spec.ts:21:5 dev server flag on/off at line 54 `.not.toBe(200)` for packages/sim-worker/pkg-rules/release/devtools_bg.wasm; "12 passed (1.1m)". Matches card step 12.
