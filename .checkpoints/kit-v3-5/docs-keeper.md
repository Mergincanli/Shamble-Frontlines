# docs-keeper: kit-v3-5 docs pass (STATUS, ROADMAP, kit-v3-4 report correction, REPO_MAP/START_HERE check)

## Plan (blocks, in order)
- B1 read sources (checkpoint, plan-rev1..3, advisor-plan-rev1 rev3 check E1-E5, build.md, Mustafa's five files)
- B2 read STATUS, ROADMAP, the kit-v3-4 report; list stale lines
- B3 STATUS edits (#44 merged; kit-v3-5 in flight; PR Advisor paused; F1-F5; known limits; queue in his words; record; E5 line)
- B4 ROADMAP entry
- B5 kit-v3-4 report correction section (append only)
- B6 REPO_MAP / START_HERE check for false facts
- B7 checks: quotes byte-equal (script), repo-map-check, no added line >120 chars outside tables; changed-line list
- B1 done (sources read). Findings: #44 merged_at 2026-11-09T18:21:13Z (gh), merge commit dated 18:21:12 UTC (git
  log 44446a5): the brief's "18:21:12" is the commit date; write both, as STATUS does for #30. Run 42673290304
  attempt 2: success, 2026-11-10T06:37:11Z (gh). PR #46: draft, head 8378ef8 (gh).
- Kit-v3-4 report Lows verified: 82 rows, first-word counts 37/26/19; mixed rows 32, 39, 46, 47, 51, 52, 55 (with
  "/"), 76 and 77 ("than"); ddb06a5 (H1 ruling ckpt, between 20b4be5 and 3d39527) and 2894a1f (rows ruling ckpt,
  after d96e215) are on main, missing from Appendix D (report :251-259).
- Advisor's #44 final-head review read (comment 6740050583, gh): findings 1 Critical agent memory, 2-4 Lows.
- B2 done. Stale lines found: STATUS:3 (tail: "In flight: kit-v3-4 ... his go"), :41-45 (In flight kit-v3-4, "a draft"),
  :46-51 (Queue holds nothing), :76-78 (PR Advisor "back on", dated 2026-11-04), :114 (row: in flight), :560-584
  (F1-F5; F1 "open GUARD-RENAME"), :587-589 (queue "Nothing of his is left"), :793 (`enabled: true`), :1032 (CI on
  main row: af8393e). F3: register citations at STATUS :116 (README:160), :117 (:119), :465 (:294-298), :466 (:119),
  :484 (:202), :485 (:192-193), :492 (:160), :495 (:408), :545 (:408) are all +1 since #45's row at README:88
  (checked: each cited row now one line down; HEAD vs 626df67 register diff is 3+/3-, no shift). ROADMAP: 12 recent
  entries (:95-:117); #44 entry :95 says "waiting on his go"; #45 has no entry (not mine to add; noted).
- Guard verified at 9bacf47: MERGE_PLACES routine-guard.mjs:84; RULES_NAMES :85; GATE_NAMES :86; DECISION_DOCS
  :87-88 (VISION_V2, LOOP_V2, LOOK_V2, EDITOR_V2, V2_PHASE0_BRIEF, DESIGN_NOTES, ART_DIRECTION, GAME_DESIGN, UNITS,
  DECISIONS); mergeAllowed :89-100; GATE_FILES :28-29 incl. npm-shrinkwrap; protectedRel :39-40; cap :1494; @tsv :1495;
  total_count :1482. Kit commit stat: advisor.md, routine-guard.mjs, its tests, CLAUDE.md (152+, 28-).
- His saved words committed at 8378ef8, equal to the worktree copies.
- B3 done: STATUS edited by script (scratchpad dk5/apply_status.py; quotes generated from the saved text blocks, never
  retyped), plus two Edit fixes (", ADR 0001's" list comma; A4's freeze sentence) and one pointer line. Changed lines
  and sources:
  - :3-26 Last updated (rewrapped, words kept; date 2026-11-10; #44 merged; kit-v3-5 in flight): gh pulls/44, git log
    44446a5, PR #46 draft (gh), kit commit 9bacf47.
  - Phase: kit-v3-4 done entry after vfx-xp-notes; In flight = kit-v3-5; Queue = his order: gh (merged_at 18:21:13Z,
    run 42673290304 attempts 1 failure/2 success), comments 6740050583 and 6740560231 (gh), queue/answers/second files.
  - Setup step 4: "Paused again by Mustafa, 2026-11-09 18:22 UTC": checkpoint [assumed: not re-read via routines API].
  - In flight table: new kit-v3-5 row (checkpoint, build.md, plan-rev3, advisor rev3 check, guard code :28-100);
    kit-v3-4 row merged + final-head reviews (gh comments).
  - Decisions pending: new "Taken by Mustafa, 2026-11-09 and 2026-11-10, for the record" (GUARD-RENAME (c) + go,
    ORDER/SA5-RIDE/allow-list/limits, SPLIT/SCOPE; E5 line): the saved files; E5 from advisor rev3 check, verified
    by git diff of 4c6db5b, 69ea88e, 626df67 (each touches docs/adr/README.md).
  - F1 carried out; F2 and F5 "Waits for the foundation's gate step (KIT-FREEZE-2, 2026-11-10)."; F3 fixed (nine
    citations +1); F4 carried out (CLAUDE.md:98 row 0, :103 line under the table [verified]).
  - Queued, in Mustafa's words: KIT-FREEZE-2 + FREEZE-FDN, STATUS-DIET, LANE-RULES + Models addition + ACCOUNTS, unit
    walk, Slice 1 note; conflict answer from plan-rev2 §4 [assumed: plan text, not the chat record].
  - Routines PR Advisor: enabled false, paused 2026-11-09 18:22 UTC (checkpoint) [assumed]; "trigger has no MCP
    connections (its GitHub tools come from the environment)": plan-rev1 G [assumed, not re-read].
  - Open bugs and risks: kit-v3-5 known limits (plan-rev3 §2, advisor E4, plan-rev2 §4; kitDir :147 [verified]).
  - Health: CI on main row -> 44446a5 (gh runs API).
  - Also: STATUS "next kit change-set" bracket (A3 fix-round items) now says it is kit-v3-5 with F1 and F4 only, the
    open items waiting under KIT-FREEZE-2 (his queue words); F5's "(next kit pass, with F4" -> "left out of kit-v3-5".
- B4 done: ROADMAP: new kit-v3-5 entry (in progress); kit-v3-4 entry MERGED (44446a5, gh) and rewrapped, "Next" line
  replaced by GUARD-RENAME (c); oldest entry (Docs after v2 research, PR #31) folded into Build history; 12 entries.
- B5 done: kit-v3-4 report: "## Correction, 2026-11-10, after the merge" appended, 20 lines added, 0 removed (git
  numstat). Sources: comment 6740050583 findings 1-4 (gh); recount of Appendix C; git log 626df67..44446a5.
- B6 done: REPO_MAP (:24 .gitignore, :108-111 hooks lines, :133-134 tools package files) and START_HERE (no guard or
  merge-rule claims; grep guard/protected/merge check/AGENTS/CLAUDE.local/shrinkwrap/allow-list): nothing false; no
  file added, moved or removed by the change-set (tools/npm-shrinkwrap.json does not exist; only listed in the guard).
- B7 done: verify.py (scratchpad dk5): 16 quote paragraphs in 9 blocks byte-equal (whitespace-joined) to the saved
  text blocks, each a run of whole source lines; 4 inline quotes (7 occurrences) found in the sources; 0 removed
  quote lines; 0 added lines over 120 outside tables; repo-map-check OK (114 entries, 101 paths).
- Not made true / caveats: merged_at is 18:21:13Z (brief said 18:21:12, the commit date): both written. Red count:
  47 of 1086 tests (checkpoint) vs 43 rows (kit commit message): STATUS/ROADMAP say 47 of 1086 tests. Routine state
  (paused, no MCP connections) not re-read through the routines API. STATUS grows to 1208 lines (STATUS-DIET queued).
DONE
