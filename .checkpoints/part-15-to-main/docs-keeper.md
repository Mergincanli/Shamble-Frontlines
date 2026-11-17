Task: part-15-docs-keeper (docs-keeper pass for the merge of #56 and change-set Part 15 to main; STATUS, ROADMAP, REPO_MAP, STATUS_ARCHIVE only; never commit)

## Plan (blocks, in order)
1. Read DECISIONS' two newest entries (2026-11-17 22:59:16 and 22:10:34 UTC) and note the binding words.
2. Read STATUS.md in full; mark lane 1's own lines vs other lanes' lines.
3. Read STATUS_ARCHIVE.md's "In flight" and "Follow-ups" structure (where merged rows go).
4. Read ROADMAP.md's "Recent work" and "v2, next" (change-set 0 entry form).
5. Read REPO_MAP.md's 2026-11-17 rows and the reports section.
6. Read Part 15's report "His answers" (docs/reports/2026-11-17-build-plan-slice-1-part-15.md) for the items that stay his.
7. Write STATUS edits (Last updated/merged, #56 row out by script, new row, Next, Waiting, Follow-ups); keep <= 120 lines.
8. Write ROADMAP edits (change-set 0 marked merged; one entry for Part 15).
9. Write REPO_MAP edits (two reports); run repo-map-check.
10. Proof: numstat; removed lines of STATUS and ROADMAP all lane 1's; line widths <= 120 outside tables.
11. DONE and answer.

## Notes
- Block 1 DONE. DECISIONS.md:130-154 (22:59:16 UTC, go on #56 merge; accepts PM-0005-54 "stands as written: a fail with
  today's rifle prototype (6 draws per pass against 4). It is not waived. The way out is my call, with the plan for the
  first-model change-set."; size; three unverified; checkpoint commits enter main's history (LOG-HISTORY); after: "Part 15
  with my six answers goes to main next, as a docs change of its own." ... "It also brings the status page and the roadmap
  up to date for this merge." / "Then change-set 1."). DECISIONS.md:155-187 (22:10:34 UTC, six answers all (a)).
  RECORD-FILES DECISIONS.md:171: "(a). Each pull request writes only its own lines there, in its docs pass, after taking
  main in. It does not reword, re-wrap or move another lane's lines." VFX-ROUTE :173 seventh package. VFX-FIX :177 "I pass
  the findings on." Both entries already in DECISIONS (b75ec0d). Branch claude/wonderful-hamilton-jdmlnf at b75ec0d; main
  at d7dad97. STATUS=120 lines now (at the cap).
- Coordinator message (mid-task): draft PR for this change-set exists: #57, draft, branch claude/wonderful-hamilton-jdmlnf,
  base main d7dad97. Use "#57" in STATUS's row and ROADMAP's entry, state "draft; in review".
- Block 2 DONE. STATUS lane 1 lines: L5 (Last updated), L13 (Last merged), L29 (#56 row), L35-37 (Next item 2), L40
  (Waiting #56), L41-42 (Waiting Part 15) = 9 lines. Others (never touch): L28 (#40 row), L43-44 (netcode Waiting
  bullet), all else incl. L21-23 (main/CI line, stale 82c605e: not mine), Follow-ups L66-100, Open risks. Follow-ups
  form: ids in STATUS, text in archive's live "Follow-ups". Budget: 111 fixed lines + mine <= 120 -> mine <= 9 lines:
  L5(1)+L13(1)+#57 row(1) leaves 6 for Next2 + Waiting + new Follow-ups bullet.
- FINDING (git blame STATUS): L35 (Next item 2's first line, "2. v2: the netcode proof test (#40); ... #52 merged; step")
  was written by DOCS-54 9660197 (the netcode lane's #55), NOT by cs0. cs0 (94aaae8/6a31650/4f20643) wrote only L5, L13,
  L29, L36, L37, L40, L41, L42 = 8 lines. So keep L35 byte for byte; the tail must continue "step" -> "2.3...".
  Budget: 112 fixed lines; my 8: L5, L13, #57 row, Next tail (>=1), Waiting (>=2), FU (>=1..2).
- Block 3 (archive structure) DONE. Archive "In flight" = 4 cells: Work | merged: PR #N (`branch` ...), sha, date ... |
  done: ... | none. Newest first, inserted after L158 (header L157-158). DOCS-54 wrote rows fresh (not line moves). #55
  (DOCS-54) has NO row in the archive: cs0 replaced it in STATUS without archiving (not mine to fix: netcode lane's).
  Live Follow-ups section L219-907; newest blocks at its end ("From KEPT-LOGS", "From the night-shift lines" ~L873-906),
  each "- **From X** (PR #N, merged as sha on date; ...; copied by script from ...):" then sub-bullets.
- Block 4 (ROADMAP) DONE. Recent work L122-248; cs0 entry L125-128 ("**Change-set 0 of the build plan [2026-11-17, PR
  #56: ready for review]** — ..."; last line ends "Two review rounds done (the verifier: PASS); then his go." and is
  135 cols, cs0's own). Form of merged: "[dates, MERGED]" + "Merged: PR #N, sha, date UTC ..., by merge commit, on his go
  after the advisor agent's APPROVE of <sha>; CI green on main (...)". 12 entries now (cs0 ... #43 at L248); adding #57
  makes 13 (>12) but brief says no fold: report it. "v2, next" pointer 2026-11-17 L102-105 is cs0's; still true, keep.
- Block 5 (REPO_MAP) DONE. Reports rows L73-80; cs0's rows L70, L71, L78, L79, L80 (L80: "| `docs/reports/2026-11-17-
  cs0-model-rules.md` | The report of change-set 0 of the build plan (PR #56) |"). Add two rows after L80. L6 "Last
  checked: 2026-11-17" already today's date: leave.
- Block 6 (Part 15 report) DONE. File docs/reports/2026-11-17-build-plan-slice-1-part-15.md (348 lines). L30-33 "Still
  his, from the text's "For Mustafa":" giving lane 2 this Part 15 and asking it for its list of needs from the block
  before change-set 4's plan is reviewed; telling lane 2 which chat on his PC makes the model files; having the VFX chat
  export at least one effect folder for the VFX proof, and later the effects for the slice's five weapons; with VFX-FIX
  (a), passing the digest's findings to the VFX chat ("I pass the findings on."). GUN-FILES: rule AX-R1 in
  asset-export-rules.md. VFX-ROUTE seventh package.
- Sources for the six follow-ups: O1 = asset-export-rules.md:316-345 (part 7; part 8 :357 "O1 and O2: Mustafa, with
  change-set M's plan"); O2 = :347-351; first real export = cs0 report risk 1 (:67, may fail PM-0005-49 or -56);
  coat = soldier-joint-list.md:195-197 (section 6; dropping it removes the four coat joints, before export); machine
  lines = build plan report "Since the plan" MACHINES row (:26, change-set 6's docs pass); "one change-set per step" =
  SPLIT-25 row (:23, change-set 2's docs pass); package list = ADR 0002:202-204 dated pointer (Part 15 to main).
- HEAD now 94c805c (checkpoint commit on b75ec0d). Main session commits checkpoints; my files untracked-modified only.
- Design (STATUS, 8 lane-1 lines -> 8): L5', L13', #57 row, L36' (one line, replaces L36+L37; L35 kept), Waiting 2
  lines (replace L40-42), new FU bullet 2 lines appended after L100. #56 row -> archive In flight top (after L158) by
  script: cells Work + State copied, "ready for review; " prefix dropped, merged-cell and done-tail new text. Archive
  Follow-ups: new block after L906 "From change-set 0 (PR #56)", items split by script from the row's "Open after it"
  text + a sixth (VFX-ROUTE package list). #55 row missing from archive: not mine (flag in answer).
- VERIFIED: d7dad97 committer date 2026-11-17 23:00:12 UTC (merged_at 23:00:13Z by REST); CI on main at d7dad97: gate
  run 856 (id 44195404107; jobs gate + smoke success), toolchain run 155 (id 44195404068), push, branch main. Runs 857/156
  are the branch claude/wonderful-hamilton-jdmlnf at the same SHA (not main). PR #57: draft, open, head
  claude/wonderful-hamilton-jdmlnf, base main d7dad97. ADR 0002's package list + dated pointer: "## Decision", part 3
  ("Crates, packages and who may depend on whom", :182; pointer :202-206). gh GraphQL refused (403): use gh api REST.
- ROADMAP blame: cs0 wrote L102-105, L125-130 (L130 = first line of DOCS-54's entry, cs0 marked it MERGED; netcode
  lane's entry: leave). REPO_MAP: cs0 rows L70, L71, L78-80.
- This change-set so far (d7dad97..HEAD, excl. checkpoints): DECISIONS +69 (two entries), ADR 0002 +5 (pointer), Part 15
  report (new, 348), build plan report (two pointers), asset-export-rules.md (AX-R1 gun-file line).
- Chosen STATUS layout (B): L5', L13', #57 row, Next tail 1 line (replaces L36-37), Waiting 2 lines (replace L40-42),
  FU 2 lines in words after L100. Widths measured in docs-keeper-drafts*.txt.
- Script written: docs-keeper-apply.mjs (+ docs-keeper-newtext.txt). Asserts every replaced line, refuses a second run
  (STATUS #57 row / archive "merged: PR #56 (" / REPO_MAP Part 15 row present). Dry run OK: STATUS 120 -> 120; archive
  1250 -> 1272; ROADMAP 248 -> 257; REPO_MAP 153 -> 155; 0 wide added lines; both quotes in DECISIONS and unbroken.
  Blocks 7-9 = running it (next).
- Blocks 7, 8, 9 (edits) DONE: script run, WRITTEN. numstat: REPO_MAP 2/0, ROADMAP 11/2, STATUS 8/8, ARCHIVE 22/0.
  STATUS 120 lines. Next: repo-map-check; block 10 proof (removed lines = lane 1's only; widths; review the diff).
- repo-map-check: one line only, "in the map but not in the repository: docs/reports/2026-11-17-part-15-second-lane.md"
  (exit 1; the report is written last). Expected.
- Block 10 (proof) DONE. STATUS removed lines = exactly old L5, L13, L29, L36, L37, L40, L41, L42; blame at HEAD: 94aaae8,
  94aaae8, 6a31650, 94aaae8, 94aaae8, 94aaae8, 4f20643, 4f20643 (all three commits in c94c81b..cd61b28 = #56). ROADMAP
  removed = old L125, L128 (both 6a31650). REPO_MAP and archive: 0 removed. Other lines kept in order: STATUS 113/113,
  ROADMAP 247/247; #40 row identical at L28; netcode Waiting bullet byte for byte at now L41-42; DOCS-54 entry byte for
  byte at now L139-147; v2-next pointer L102-105 identical. Added non-table lines <= 120 code points (ROADMAP new L125
  = 120 exactly). git diff --check clean. Paraphrases grounded: DECISIONS :282 (2.3 chat stopped), :536 (Slice 3).
- Open notes for the answer: ROADMAP Recent work now 13 entries (>12), no fold per brief. #55 (DOCS-54) has no archive
  "In flight" row (cs0 dropped it from STATUS in its pass; netcode lane's PR: not written by this lane). STATUS L22-23
  CI line still names 82c605e (DOCS-54's line, not mine). "Last checked: 2026-11-17" in REPO_MAP unchanged (today).
DONE
