Task: security-auditor-review2 (round 2, the last planned round, cs1 "Types", foundation step 2.4): only what changed in my lane since round 1 (82142fa -> 65a7b13: 1a2ff13 fix pass + 65a7b13 follow-up), and whether my round-1 Lows S1-S3 are closed. Branch claude/wonderful-hamilton-jdmlnf, HEAD 554ebe1 (checkpoint commits; code = 65a7b13's), base main 22c6afa. Started 2026-11-18. I edit nothing but this file (and my agent memory).

Rules: cargo --locked (and --offline), no cargo update, no npm scripts/npx, nothing installed; no SHAMBLE_WRITE_TYPES in the main folder; trials only in `.gate/cs1-types/sa-plant` (git worktree add --detach, HEAD) with CARGO_TARGET_DIR=.gate/cs1-types/sa-target (my own, never main's target); put every file back, delete planted files/links one by one, `git worktree remove` without --force; never open spikes/ or uploads.

## Plan (blocks, in order)
- B0 Read: diff 82142fa..65a7b13 -- crates packages/types/src/layout-check.ts; fix-pass-1.md (done); plant.sh + plant-results.txt runs 2f, 6a-6e.
- B1 S1: read both write_generated + repository_root; trial in sa-plant: generated folder as link, parent as link, link inside, dangling link named like an output; plus canonicalize/TOCTOU and create_dir edge cases. Judge "refuse write mode when CI is set" not taken.
- B2 S2: read the `*/` line check; trial with planted docs: a doc line exactly `*/`, a block doc comment, #[doc = "..."], type-level doc, a field doc ending in `*/`.
- B3 S3: read layout-check.ts doc.
- B4 repository_root guard (point 4): env var effects, log/path leaks.
- B5 Other changed lines in my lane: scratch loop (AlreadyExists only, absolute base), entries_of, "no file of its own in generated/".
- B6 Verdict, findings ranked, memory update, DONE.

## Notes
### B0 Read: DONE
- Code at HEAD 554ebe1 == 65a7b13 for crates, packages, Cargo.toml/lock, rust-toolchain.toml (git diff --stat empty). Diff 82142fa..65a7b13 in my scope: 6 files +288/-120 (ts_export.rs, layout_ts.rs, layout.rs, lib.rs, messages.rs docs, layout-check.ts). fix-pass-1.md read; plant.sh runs 2f, 6a-6e read with results (all "as wanted"; plant run at 65a7b13, own target plant-target, 30 runs, 0 not as wanted).
- ts-rs facts re-read (registry src, 12.0.1): format_docs lib.rs:1188-1213 has TWO forms: one doc entry holding '\n' -> raw `/**{doc}*/` (lib.rs:1194); otherwise `/**` + per entry " *"+entry + "\n */" (:1197-1210). ts-rs-macros utils.rs:167-174 extract_docs takes every #[doc = expr] verbatim (no trim); field docs prefixed "\n" (named.rs:150-154). messages.rs: 52 `///` lines, no `/**`, no #[doc, no include_str (grep exit 1).
### B3 S3: CLOSED
- layout-check.ts:9-17 now says: drift check, not a security check; a passing block still has every count and offset checked against its length; never throws given an ArrayBuffer; the caller makes sure it has one. All three parts of my S3 met. `const header = BLOCK_LAYOUT.header` change: no effect in my lane.
### B1a S1 second clause (refuse write mode when CI is set), not taken: ACCEPTED
- The drafted CI edit (docs/reports/2026-11-18-cs1-types.md:271-279) runs plain `cargo test --workspace --locked` first, then write mode, then `git status --porcelain --untracked-files=all -- packages/types/src/generated` and fails on any output. The comparison is with the commit, so an injected SHAMBLE_WRITE_TYPES (e.g. a committed .cargo/config.toml [env]) cannot silence it: modified, deleted and new untracked files all show. A CI refusal would break that design (env -i passes CI). Condition: keep the git status form (git diff --exit-code alone misses new untracked files). Nit for that tier-2 PR: add --ignored (no ignore rule matches generated/ today: git check-ignore exit 1), so a later ignore line cannot hide a regenerated file.
