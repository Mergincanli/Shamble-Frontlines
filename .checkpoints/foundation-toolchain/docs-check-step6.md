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
- [facts] #51: merged_at 2026-11-12T13:22:32Z (REST), merge commit 2aa5e9c dated 13:22:31 UTC, parents 76b4f7a + 5d50bf2, head ref claude/clever-fermi-oudsfq; go 13:18:42 UTC, session_019YNMUWdGJZoQFUuuvvBP1Y, APPROVE of 5d50bf2 (merge msg); head runs 43130543086 (#701 push) + 43130550831 (#702 pull_request) success; main run 43226692719 (#705, push, 2aa5e9c) success. PR #50: 76b4f7a, 18:38:28Z, parents bb2d817 + f6becf5, main run 43079707687 success. PR #48 draft. SHAs c090843 1e6f84e b6b3b81 43733e5 3860e3f 3828e6a in main. All PASS.
- [facts] 2.1b report :55-80 (reviews) and :92-95 (IE-2/3/4/7, SA-L1 closed) back the archive's #51 row and the 2.1a closure note. PASS.
- [facts] DECISIONS.md entries at 00:07:48, 16:31:45, 15:35:17, 16:28:28, 17:23:21, 13:18:42 exist; Queued has the advisor-tool and models items. PASS.
- [facts] CHECKPOINTS.md:221 (§10) `git -C <path> status --short --ignored`; brief :20-21 and :144 as cited; checkpoint.mjs:168 `[skip ci]`; ADR 0002 PM-0002-39/-53/-57 (:1305/:1319/:1323), :642, part 7 (:416-420 tsc -b), part 9 G0 (:569) and G3 (:572). PASS.
- FINDING (Medium) archive G0 item: "G0's exit-2 mark moved to step 2.3" + "is proven by 2.3's G0" state the move as done/proof as present; the register row :95 says "His go for this row: PR #48's merge" (A2: "I will give my go on it in the pull request"). Omits his pending go.
- FINDING (Low?) archive BRIEF-STALE: "it is the claude.ai Advisor's file" but the brief's line 3 says "From Mustafa and the Advisor." (CLAUDE.md:24-25 calls only LOOK_V2 "Mustafa's and the Advisor's"). Check REPO_MAP's brief row.
- [facts] PIN-GUARD: protectedRel (routine-guard.mjs:35-48, hooks equal origin/main) false for the 5 root pins; NODE-26: GATE_FILES (:28-29) holds tools/package.json + tools/package-lock.json. PASS.
- [facts] SETUP-NODE-CACHE ([assumed] in the doc): job 131405738690's log line 155 "package-manager-cache: true". PASS.
- [facts] ROADMAP: Recent work 12 entries (role file "Keep 8-12", origin/main docs-keeper.md:22-23); fold's phrases cut from the two deleted entries; placed in date order after Kit v3.1. CI green on every build head incl. 6135579 (toolchain 43289800904/43289812375). His order 2026-11-11 = "#49, then #47, then 2.1b, then #48" (DECISIONS.md:409-418). PASS.
- [facts] R7 line-number note dropped from STATUS: archive :914-916 keeps it as a resolved item ([verified 2026-11-11]). PASS.
- [facts] Quoted strings in added lines: none are Mustafa's; "no option before the subcommand" = routine-guard.mjs:1046; "deleting the toolchain file fails G0 with exit 2" = ADR 0002:1454-1455 (wrapped); MANUAL-MODE's "in Accept edits or Manual mode" = CLAUDE.md step 3. All table rows keep their pipe counts. Fold's dropped facts survive (DECISIONS.md:710-714, STATUS:62). PASS.
- [facts] BRIEF-STALE's "no session edits it" matches the archive's DOCS-ALIGN (:480: "the Advisor's documents; the session does not edit them"); EMPTY-SLOTS = plan Part 3 (:250-263); no tsconfig tracked. PASS (ownership wording: Low below).
- FINDING (Medium) STATUS:29 "the docs pass done": findings-2-3-c4.md (header: "To merge into the report's lists and the archive's Follow-ups after the docs workflow (wf_29ac478b-0a7) is committed") is not yet in the archive or STATUS; checkpoint Current step still has the fact check and the E8/E9 re-runs; the report's risks (sudo on the runner; c4's TOOLKIT-RESOLVES-UP) are in neither "Open risks".
- FINDING (Medium) STATUS:29 "(its full read of the skeleton is done: his item 4 of 2026-11-12)": A9 item 4 = "At your moment 5, before the PR is marked ready, have the Fable advisor read the built skeleton as a whole ... Fix what it finds within the review budget, or record it in the report." The read was early (consultation 4, before the review round), the read before ready is narrowed to changed files (checkpoint :342), and c4's items are not fixed or recorded yet. Row asserts his item met.
- FINDING (Medium) archive:586-587 G0 (as above).
- FINDING (Low) REPO_MAP:16 `[workspace.dependencies]` "holds the shared versions" (empty; report:18).
- FINDING (Low) archive:618 BRIEF-STALE "the claude.ai Advisor's file" vs brief:3 "From Mustafa and the Advisor."
- FINDING (Low) archive:556-557 "GIT-C, ADVISOR-TOOL-WORDS and MODELS-2026-11-12 (their "Queued" items)": GIT-C has no Queued item (checked against CHECKPOINTS.md:221).
- No High found. Next: advisor consult, then the StructuredOutput.
- [sweep] Every backticked path-like token in the added lines exists (non-paths: gitignore patterns, globs, branch and package names, file://, ./index.ts, short names gate.yml/toolchain.yml). Every file:line cite verified (:221, :642, checkpoint.mjs:168, brief :20-21 and :144); other ":NN" hits are clock times. PASS.
- [sweep] Follow-up names: archive :551-618, STATUS :96-99 and findings-2-3.md each hold the same 18 (14 for 2.3 + 4 later). PASS.
- [add to Medium 1] consultation 4 (checkpoint :333-342) schedules after this docs commit a code change (`expected` on both overflow tests) and planted failure M6 (L2); neither STATUS:29's Waiting-on nor ROADMAP:123's Next names them.
- [add to Medium 3] findings-2-3.md item 8 has the same "is proven by" wording; step 7's report list is built from it.
- [context, not judged] report's risks (sudo on the runner; TOOLKIT-RESOLVES-UP) absent from STATUS "Open risks" (docs-keeper disclosed). STATUS:21 and :47 (his Windows run) unchanged lines, not judged.
- DONE: 3 Medium, 3 Low, no Critical or High. repoMapExit 0, statusLines 120.
