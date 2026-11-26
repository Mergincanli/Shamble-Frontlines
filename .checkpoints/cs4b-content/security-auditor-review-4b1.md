# security-auditor: change-set 4b-1 "Content", review round 1

Date 2026-11-26. Worktree .gate/lane1-takeover, branch cs/4b-content, head ef2edec (main 3b949c1 merged in).
Read-only; notes and scratch only (scratch: .gate/security-auditor-4b1-scratch/, target:
.gate/security-auditor-4b1-target/). Agent memory: none exists (checked .claude/agent-memory* in both checkouts).

## Plan
1. Context: card .checkpoints/cs4b-content.md (steps 9-17), reread-sheet B.1/B.4, test-list, dependency-check.md.
2. Diff inventory: git diff origin/main...HEAD --stat -- crates packages; plus manifests/lockfiles vs origin/main.
3. Share 1, loader as input boundary (crates/content/src): caps (bundle 8 MiB, file 1 MiB decoded, 16 files,
   16,384 values, depth 32, numbers <= 25 bytes, keys/ids <= 64), bounded work (dup-key search, canonical compare,
   path stack, field path per value, Object::take linear search), folder_and_id path rules, refusal echo (<= 64 B,
   \xNN outside printable ASCII and backslash) and message cap (512 B), no fallback, no unsafe, no panic.
   Map each to S-must 3-8, 11 and S-should 13-16: met (test) or missing.
4. Share 2, fuzz + classes: content_proofs.rs, tests/content_fuzz.rs, tests/content_proofs.rs; counting allocator
   unsafe; memory ceilings; time bounds.
5. Share 3, file tools/readers: legacy-extract.mjs, corpus-texts.mjs, legacy-duration-paths.txt, files.rs,
   plain-file.ts + users, recordings.rs scan, time_report.rs, SHAMBLE_WRITE_VERDICTS.
6. Share 4, deps: Cargo.lock, Cargo.toml(s), package.json(s), package-lock vs origin/main and step 1's batch;
   node-names.d.ts.
7. Share 5, browser side: content.spec.ts, content.test.mjs, .gate/rules-module; shipped code cannot reach it; no
   CSP/server/page change.
8. Share 6, departures 3, 6, 9, 11, 12.
9. Hostile inputs of my own in a scratch test (copy of the crate under scratch, own target): deep nesting in a
   file's text, 1 MB of escapes, path with NUL / backslash, Windows device names as ids, 1 MiB of quotes in text,
   wide object (Object::take cost), long field paths.
10. Final: <= 25 lines in the brief's shape.

## Findings (one line each, as made)
