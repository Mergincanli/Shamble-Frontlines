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
- plan written (this file). Advisor tool consulted once before the edits: fold ROADMAP's two oldest from their own
  phrases; mark 2.1a/2.1b merged (tag and closing sentence only); STATUS cuts as planned, spare cut :72; no new Open
  risks (PIN-GUARD carries the pins risk; the sudo risk stays in the report: named in the return); nothing open may
  read as decided; archive items self-contained; S1-S5 defined; no lockfile counts in the map; drift grep, report only.
  #51's head runs verified: 43130543086 (push #701) and 43130550831 (pull_request #702), success.
- DONE docs/REPO_MAP.md: "Last checked" 2026-11-12; seven top-level entries after `legacy/` (rust-toolchain.toml,
  Cargo.toml, Cargo.lock, crates/, package.json, package-lock.json, packages/); `.gitignore` line + `/target/` and
  `test-results/`; `.github/workflows/toolchain.yml`. repo-map-check: "repo map OK: 120 entries cover 107 required
  paths", exit 0.
- DONE docs/STATUS_ARCHIVE.md: "In flight" #51's merged row at the top (:159); 2.1a's "For 2.1b's plan" marked closed
  by #51 (:509-510); live "Follow-ups" gains "From foundation 2.2" (:551-614): 14 items for 2.3 and 4 for later steps,
  each self-contained, codes defined, his words only pointed at (DECISIONS.md entries by time, Appendix items).
- DONE docs/STATUS.md: Last updated 2026-11-12 (main 2aa5e9c); Last merged = #51 (13:22:32 UTC, APPROVE of 5d50bf2);
  CI on main 2aa5e9c run 43226692719; In flight: #51 out, #48 updated (steps 1-5, docs pass done; next Fable read,
  review round and verifier, report, advisor's review, his go; tier 2); Next item 2; Waiting: his go for #48 with K1 as
  written and Q2.2-a (waiting, not decided); Follow-ups: 2.1a's five closed items dropped, the 2.2 block by short name;
  Open risks: R7's closed line-number note dropped (its text stays in the archive). 120 lines; lines 11-12 and 18-19
  (his quotes) untouched.
- DONE docs/ROADMAP.md: 2.2's entry at the top of Recent work (:113-123); 2.1b and 2.1a retagged MERGED with a closing
  "Merged:" sentence (verified facts: 2aa5e9c 13:22 UTC, APPROVE of 5d50bf2, run 43226692719; 76b4f7a 18:38 UTC,
  APPROVE of f6becf5, run 43079707687); the two oldest entries (TEST-RUN part B, docs after kit v3.1) deleted by line
  range (sed 195,198d) and folded into one Build history line made of their own phrases (:74-80). Recent work: 12.
- DONE final checks: repo-map-check "repo map OK: 120 entries cover 107 required paths", exit 0; STATUS.md 120 lines;
  `git status`: only the four docs and this notes file changed (no commit by me; the saver's autosaves cover
  .checkpoints/ only). Drift grep (START_HERE, TECHNICAL, MODULE_MAP, README.md, tools/README.md, SPIKE_PLAN): no claim
  2.2 made untrue in docs/ (START_HERE has its "Legacy doc" banner; TECHNICAL:9-12 and MODULE_MAP:11 describe legacy;
  tools/README.md:32 and START_HERE:87 describe gate.yml's two jobs, still true). Left, for the caller: README.md:16-26
  (its layout table has no v2 workspace paths; outside docs/), STATUS:21 and :47 (his Windows run, possibly done in
  2.1b's A3, unverified), STATUS:57-67 (Routines read 2026-11-10), STATUS:93-95 (2.1b's follow-ups have no archive
  block), ROADMAP:3 (banner without `legacy/`).
- DONE after the second advisor consultation: the checkpoint at HEAD gained consultation 4 (the Fable advisor's full
  skeleton read done; a re-check of changed files before ready), so STATUS's #48 row and ROADMAP's 2.2 "Next" now name
  the re-check, not the read; findings-2-3.md unchanged (autosave 8b33e3e, same text); `findings-2-3-c4.md` exists
  (not in the brief: not taken, named in the return); the archive block's intro says "is to take ... at step 7" and
  gives the [verified]/[assumed] split. Re-checked: no new line over 120; STATUS 120 lines; repo-map-check exit 0.
  Ready to hand back.
