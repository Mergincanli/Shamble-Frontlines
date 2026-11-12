# docs-keeper, foundation 2.2 step 6 (PR #48): plan and progress

Role: main's `.claude/agents/docs-keeper.md` (read with `git show origin/main:...`). Worktree only:
`/home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt` (branch claude/laughing-pasteur-fwya24; code head a5823cf,
docs commit 6135579). Edits in docs/ only; never DECISIONS.md, docs/adr/, the report, CLAUDE.md, .claude/, .github/.
No commit, no push. His words: not quoted (pointers to DECISIONS.md entries and the report's Appendix A instead).

## Facts read (sources)
- #51: merged_at 2026-11-12T13:22:32Z (REST API, pulls/51); merge commit 2aa5e9c dated 13:22:31 UTC, parents 76b4f7a
  and 5d50bf2; head ref claude/clever-fermi-oudsfq; his go 13:18:42 UTC in session_019YNMUWdGJZoQFUuuvvBP1Y, quoted
  whole in the merge message; advisor agent's APPROVE of 5d50bf2; CI on the head runs 43130543086 and 43130550831 (merge
  message); CI on main: gate workflow push run 43226692719 (run #705), success (`gh run list --commit`).
- 2.1b's reviews (its report :55-80): round 1 (security-auditor APPROVE WITH LOWS, test-engineer and infra-engineer
  APPROVE), fix pass 43733e5, the verifier PASS on 43733e5, the advisor's REVISE on 3860e3f (two Mediums), the fix
  3828e6a, then APPROVE of 5d50bf2. 2.1b's report :92-93 closes 2.1a's IE-2, IE-3, IE-4, IE-7 and SA-L1.
- repo-map-check before edits: exit 1, 8 paths missing (toolchain.yml, Cargo.lock, Cargo.toml, crates/,
  package-lock.json, package.json, packages/, rust-toolchain.toml).
- Spot checks of findings-2-3.md: CHECKPOINTS.md:221 (in §10) has `git -C <path> status --short --ignored`;
  V2_PHASE0_BRIEF.md:20-21 "No engine code except throwaway spikes", :144 the Don't line "Write engine code in phase 0";
  checkpoint.mjs:168 adds `[skip ci]` when no code commit is ahead; the four stub tests import "./index.ts"; no
  tsconfig under packages/; ADR 0002 PM-0002-53 (4 of 4 builds), PM-0002-39 (G0 exit 2 under --prove), :642 and
  PM-0002-57 (one aggregate required check); toolchain.yml header: "Step 2.3 replaces this file with the gate v2's
  workflow"; .gitignore +2 (`/target/`, `test-results/`).

## Plan, one file at a time
1. docs/REPO_MAP.md: eight top-level entries (rust-toolchain.toml, Cargo.toml, Cargo.lock, crates/, package.json,
   package-lock.json, packages/, .github/workflows/toolchain.yml) from the files themselves; the `.gitignore` line gains
   `/target/` and `test-results/`; "Last checked" 2026-11-12. Then repo-map-check: exit 0.
2. docs/STATUS_ARCHIVE.md: (a) "In flight": #51's merged row at the top (facts above); (b) live "Follow-ups": a new
   block "From foundation 2.2" with the 14 findings for 2.3 and the 4 follow-ups, text from findings-2-3.md, tightened
   only; (c) 2.1a's "For 2.1b's plan" items marked closed by 2.1b (#51 merged; its report :92-93).
3. docs/STATUS.md (120 lines max, now 120): Last updated; Last merged = #51; CI on main 2aa5e9c; In flight: #51's row
   out, #48's row updated; Next item 2; Waiting on Mustafa (his go for #48, not #51); Follow-ups: the 2.2 block by short
   name; room made by dropping closed items (2.1a's five closed by 2.1b, R7's line-number note, now closed) and
   tightening the Waiting line. Count lines after.
4. docs/ROADMAP.md: a terse 2.2 entry at the top of Recent work; 2.1a and 2.1b entries marked merged (stale since
   #50 and #51 merged); Recent work at 14 entries -> fold the two oldest (TEST-RUN part B, docs after kit v3.1) into
   Build history (the role's 8-12 rule).
5. Re-run repo-map-check; wc -l STATUS.md; grep for stale lines elsewhere (left, reported).

## Progress
- plan written (this file).
