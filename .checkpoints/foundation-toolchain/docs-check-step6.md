# docs-check-step6: adversarial fact check of docs-keeper's step 6 edits (PR #48)

Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (branch claude/laughing-pasteur-fwya24). No edits
to anything but this file.

## Plan
1. Mechanical checks: `git status --short` (only the four docs + .checkpoints/), `node .claude/scripts/repo-map-check.mjs`
   exit code, `wc -l docs/STATUS.md` (<= 120).
2. Read the sources: the report docs/reports/2026-11-12-foundation-2-2.md (Appendix A = Mustafa's words), the
   checkpoint .checkpoints/foundation-toolchain.md, findings-2-3.md, DECISIONS.md entries, the 2.1b report.
3. REPO_MAP.md: every new row against the file it describes (rust-toolchain.toml, Cargo.toml, Cargo.lock, crates/*,
   package.json, package-lock.json, packages/*, .gitignore, .github/workflows/toolchain.yml), ADR 0002 part numbers.
4. STATUS.md: SHAs, dates, run ids, PR states, "Waiting on Mustafa" wording (nothing recorded decided while open),
   the follow-up names vs findings-2-3.md, the removed lines (were they safe to drop?).
5. STATUS_ARCHIVE.md: #51 merged row (SHA, merged_at, parents, go time, CI runs; via gh/git), 2.1a closure note,
   the "From foundation 2.2" block item by item vs findings-2-3.md, [verified] claims spot-checked against files.
6. ROADMAP.md: the 2.2 entry, the MERGED retags (SHA/time/head/CI run), the folded "Build history" line vs the two
   deleted entries (byte-level: only phrases cut from them), the 8-12 count.
7. Any words given as Mustafa's: byte-identical to the report's Appendix A (or DECISIONS.md).

## Notes (a line per finding, as I go)
- [mech] repo-map-check: "repo map OK: 120 entries cover 107 required paths", exit 0. PASS.
- [mech] wc -l docs/STATUS.md = 120. PASS (at the limit).
- [mech] git status --short --untracked-files=all: only the four docs, docs-keeper-step6.md and this notes file. PASS.
- [REPO_MAP] rust-toolchain.toml, Cargo.toml, Cargo.lock, crates/ (6 crates, each one test, every crate `[lints] workspace = true`), package.json, package-lock.json, packages/ (5, each one test), .gitignore (+/target/, +test-results/), toolchain.yml (on push + pull_request, steps in order, every cargo build/test/fetch --locked, header "Step 2.3 replaces this file"): all match the files. All 7 new top-level paths mapped.
- [REPO_MAP] Low candidate: Cargo.toml row says `[workspace.dependencies]` "holds the shared versions" while the table is empty (report:18 says "an empty `[workspace.dependencies]`").
