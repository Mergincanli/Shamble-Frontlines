docs-keeper-cs1: the docs pass of change-set 1 "Types" (foundation step 2.4), PR #58, lane 1; edits only STATUS,
STATUS_ARCHIVE, ROADMAP, REPO_MAP; never commits.

## Plan (blocks, in order)
B0. Main check: origin/main must be 22c6afa (else stop, edit nothing).
B1. Read DECISIONS: RECORD-FILES (2026-11-17 22:10:34 UTC) and OLD-LINES (2026-11-18 11:08:25 UTC), word for word.
B2. Read the report docs/reports/2026-11-18-cs1-types.md (Follow-ups, Two misses, decisions asked).
B3. Read STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP; blame each; list this lane's commits (#56, #57, #58).
B4. Moves under OLD-LINES (merged PRs only): pick lines, write the byte-copy script, run it, list every moved line in
    docs-keeper-moved-lines.md (file, old line, new place, first words). Script beside it.
B5. STATUS edits in this lane's own lines: Last merged (#57 + pointer to ROADMAP's list), Next ("the netcode proof
    test's result"; then change-set 2), In flight (#58), Waiting on Mustafa, Follow-ups, Open risks.
B6. STATUS_ARCHIVE edits: #57's merged row, follow-up texts, health if it changed.
B7. ROADMAP: entry for #57 (merged) and #58 as the repo's style has it; keep 8-12 entries (fold oldest by script).
B8. REPO_MAP: new paths, changed descriptions, the regenerate command, the target-folder rule; stale lines of other
    lanes listed, not touched.
B9. Checks: repo-map-check, wc -l STATUS (<= 120), line widths (<= 120), RECORD-FILES blame proof (command + result).
B10. DONE and the answer.

## Notes
- B0 done: origin/main = 22c6afa0341c36c40479f671736a569adfdb2663, merge-base(HEAD, origin/main) = 22c6afa. Next B1.
- B1 done: RECORD-FILES (DECISIONS.md:202) "Each pull request writes only its own lines there, in its docs pass, after
  taking main in. It does not reword, re-wrap or move another lane's lines." OLD-LINES (DECISIONS.md:155) "(a), for
  lines about merged pull requests. A docs pass of either lane may move those lines by script, byte for byte, never
  reworded, also when another lane wrote them. Its report lists each line it moved. Every other line stays with the
  lane that wrote it". Next B2.
- B2 done: report read (309 lines). Follow-ups (report 195-223): CI-CUT; security check (licence, rust 1.78.0 vs 1.88.0,
  cut stops IP only); cs2 (exports entry, by-name handle, Section::align, native-only export tests, layout check per
  Worker start); cs3 (provisional list, decoder caps, redacting Debug on Join); cs4 (row counts and offsets, padding
  owner, read hash at 4 bytes, content export test folder rules); round 2 Lows (guards untested, path-string compare,
  doc-comment check, generated/ link or third folder, tag-byte pair, case-insensitive FS); 2.3 chat (formatter and linter
  skip generated/**, gate regeneration own build folder); drafts' next revision (who reads message types in TS; ADR 0002
  pointer). Dependency hold: report 32-35. Decisions: DEP-HOLD, CI-TYPES, CI-CUT, LANE-ISSUE (report 235-262). Risks
  (report 226-229): provisional widths; type error caught by hand only; 2.3 chat's settings differ. Next B3.
- B3 finding: this lane's commits = #56 (c94c81b..cd61b28), #57 (d7dad97..f26ac5b), #58 (22c6afa..HEAD). STATUS blame:
  own (#57) lines 5 (1aaa067), 13 (d5529ca), 29 (d5529ca, #57's row), 36 (d5529ca), 39-40 (1aaa067), 99-100 (1aaa067).
  All other lines: 3f98c0b/554face/bfd719b (#52, merged 4460ef5) and 9660197 (#55, merged c94c81b): not this lane's.
- B3 finding: #57's pass treated #52/#55 lines as another lane's ("Left as found", part-15 report :59); follow that.
- B3 finding: old Last merged pointer was "every merge: ROADMAP." (`git show d7dad97:docs/STATUS.md` :13).
- B3 finding: archive "In flight" = 4 columns (Work | Branch | State | Waiting on), newest first at :159 (#56's row,
  1aaa067, own). STATUS's table has 5 (PR first). #57's row (STATUS :29, own) moves by script, then its cells edited.
- B3 finding: security check's Lows: L1 licence, L2 Rust 1.78.0 not 1.88.0, L4 cut stops IP only; L3 CLOSED
  (security-auditor-review1.md:27).
- B3 finding: ROADMAP Recent work = 13 entries (#57 125-129 own; #56 131-137 own; #55 139-147; #54 149-154; #53
  156-162; #52 164-172; #48 174-189; #51 191-200; #50 202-217; #49 219-226; #46 228-239; #44 241-255; #43 257, one
  line). Build history folds: #37-#42 as "- " bullets at 85-88 (88 by 94aaae8, #56, prefixed "- " before OLD-LINES).
  Plan: fold #43 and #44 byte for byte after line 88, blank lines around (no byte of a moved line changes).
- B3 finding: STATUS movable lines about merged PRs are few: #57's row (own); the STATUS-DIET pointer table 113-120
  (#49's move; referenced by line 85 "the table below": leave a one-line pointer of mine). Open follow-up ids (83-98)
  stay: docs-keeper keeps open follow-ups by id in STATUS.
- B3 finding: REPO_MAP :18 (`crates/` "each still an empty stub", 790ac31) and :21 (`packages/` "each empty with one
  stub test", 550856a) are #48's lines (merged 4d21c1a): stale now, not this lane's: leave and list. :6 "Last checked:
  2026-11-17" is #55's (9660197): leave. Own REPO_MAP lines: 70-71, 78-80 (94aaae8, #56), 81-82 (1aaa067, #57).
- B3 verified against the code (daf3d98): messages.rs has 8 pub structs (Join, JoinAccepted, Input, SnapshotAck,
  Command, AimPoint, ViewTime, Snapshot); lib.rs:6-7 `#[cfg(test)] mod ts_export`; client-endpoint lib.rs:6-8 `pub mod
  layout` + `#[cfg(test)] mod layout_ts`; layout.rs:61 BLOCK_LAYOUT, :69 LAYOUT_HASH, :86 layout_hash; generated/
  protocol: 8 files, ts-rs header "Do not edit this file manually."; generated/block/layout.ts: "Generated by the test
  generated_layout_is_current ... Do not edit", LAYOUT_HASH = 0x2df78c30; index.ts re-exports the 8 types, BLOCK_LAYOUT,
  LAYOUT_HASH, checkLayoutHash, LayoutCheck; layout-check.ts:18 checkLayoutHash; Cargo.toml:13 ts-rs =12.0.1 default
  features off; protocol Cargo.toml:9-10 dev-dependency; SHAMBLE_WRITE_TYPES=="1" (layout_ts.rs:106); build-folder
  rule: ts_export.rs:15-27 and layout_ts.rs:45-46 (CARGO_MANIFEST_DIR built vs run). Map check baseline: OK, 129 entries
  cover 109 required paths. HEAD moved to daf3d98 (main session's saver; the report committed in 5f1b58f, unchanged:
  309 lines). B3 done. Next B4 (moves).
- B4 finding (GitHub REST, read-only): PR #57 merged_at 2026-11-18T11:09:31Z, merge 22c6afa, head f26ac5b, by
  Mergincanli. Runs on SHA 22c6afa: gate 866 (id 44292101495; jobs gate + smoke success) and toolchain 165 (id
  44292101705; success) are head_branch MAIN (push); gate 867 (44292137403) and toolchain 166 (44292137394) are
  head_branch claude/wonderful-hamilton-jdmlnf (push of the same commit). The caller's "CI on main: gate 867,
  toolchain 166" is off by one: record main's 866/165 and tell the caller.
- B4 finding: #57's PR body :20 "The advisor agent: APPROVE of f26ac5b... (2026-11-18, about 00:17 UTC)"; the verifier
  PASS (seven Low, all fixed); body :24-30 three Lows (1 size recount 223/18 vs 220/15; 2 Last merged pointer "every
  merge: ROADMAP"; 3 Next "Slice 3 waits for #40" vs the proof test's result), "change-set 1's docs pass takes them".
- B4 decision: line budget. STATUS 120; my net adds ~+7 (Last merged 1 line incl. "every merge: ROADMAP"; Next +1;
  Waiting +3; cs1 follow-ups +3; #58's row replaces #57's). Room: move #57's row (own) to the archive's In flight
  (then its cells edited: own line) and move STATUS-DIET's pointer table (STATUS 113-120, #52's lines; it records
  where merged PR #49 moved STATUS's sections) byte for byte to the archive's end, with a one-line pointer of mine in
  STATUS (line 85's "the table below" still finds it). Open follow-up ids (83-98) stay (docs-keeper.md: by id).
  ROADMAP: fold #43 (257) and #44 (241-255) byte for byte after line 88, blank separators, no "- " prefix -> 12 entries
  after #58's entry. Script: .checkpoints/cs1-types/docs-keeper-move.mjs; list: docs-keeper-moved-lines.md.
  pre-move blob: docs/STATUS.md worktree 127df6603af418ebe58385fda96b526ed32fe7bb head 127df6603af418ebe58385fda96b526ed32fe7bb main 127df6603af418ebe58385fda96b526ed32fe7bb
  pre-move blob: docs/STATUS_ARCHIVE.md worktree 4d284ef381a840e6a90a1333f8fb14bb7bc40510 head 4d284ef381a840e6a90a1333f8fb14bb7bc40510 main 4d284ef381a840e6a90a1333f8fb14bb7bc40510
  pre-move blob: docs/ROADMAP.md worktree ce0626780934b0fb48966d6106f915f983407a50 head ce0626780934b0fb48966d6106f915f983407a50 main ce0626780934b0fb48966d6106f915f983407a50
  pre-move blob: docs/REPO_MAP.md worktree 9e0117f219aa495366e3d51cb050338e45f467ed head 9e0117f219aa495366e3d51cb050338e45f467ed main 9e0117f219aa495366e3d51cb050338e45f467ed
- B4 done: docs-keeper-move.mjs --write: 9 checks ok; STATUS 120 -> 111, archive 1272 -> 1282, ROADMAP 257 -> 258.
  Moved: STATUS:29 -> archive:159; STATUS:113-120 -> archive:1275-1282; ROADMAP:257 -> :90; ROADMAP:241-255 -> :92-106.
  Pure-move check (result: removed non-blank 25 = added non-blank 25, sorted lists equal; blanks -2 +4):
    git diff -U0 -- docs/STATUS.md docs/STATUS_ARCHIVE.md docs/ROADMAP.md | node -e 'let rem=[],add=[];for(const l of
    require("fs").readFileSync(0,"utf8").split("\n")){if(l.startsWith("---")||l.startsWith("+++"))continue;
    if(l.startsWith("-"))rem.push(l.slice(1));else if(l.startsWith("+"))add.push(l.slice(1));}const ne=a=>a.filter(
    x=>x!=="");const r=ne(rem).sort(),a=ne(add).sort();console.log(r.length,a.length,JSON.stringify(r)===JSON.stringify(a))'
  List: docs-keeper-moved-lines.md. Next B5 (STATUS own lines).
- B5 done: STATUS own-line edits: :5 Last updated; :13 Last merged (#57, 22c6afa, 11:09:31 UTC, his go; CI green on
  main; "every merge: ROADMAP." back: Low 2 of #57's final head); #58's row (new :29; holds "Open after it, for
  Follow-ups once merged" as #56's row did; ready when the verifier and the advisor agent are through); Next :36-37
  ("Slice 3 waits for the netcode proof test's result": Low 3; then change-set 2, its plan first, after his go on #58);
  Waiting: #58's go, the test zip, DEP-HOLD, CI-TYPES, CI-CUT, LANE-ISSUE (4 lines) + Part 15's items in their words (2
  lines); Follow-ups: the dependency hold (2 lines); pointer heading for the moved table (last line). STATUS = 120 lines.
  Over 120 chars: :23 (#55's line, URL), table rows :28 (#40) and :29 (#58; one line per PR). Next B6 (archive).
- B6 done: archive :159 (#57's moved row, own) edited into four cells (5 pipes, like the header): bold title; Branch
  cell "merged: PR #57 (...), 22c6afa, 2026-11-18 11:09:31 UTC (commit date and merged_at), by merge commit, on his go
  of 11:08:25 UTC for head f26ac5b..., after the advisor agent's APPROVE of that head (about 00:17 UTC); CI green on main
  (gate 866 with smoke, id 44292101495; toolchain 165, id 44292101705)"; State "done: report; reviews; what he accepted
  (DECISIONS:148-153, paraphrased, no quotes); OLD-LINES; the three Lows (two fixed here, the size recount left to #58's
  report); its merge recorded by #58"; Waiting "none". Archive end: a note line of mine after the moved table ("this
  file" = STATUS). Not touched: Health "CI on main" row (3f98c0b, #52; dated 2026-11-10): stale, listed. #58's
  follow-ups and risks go to the archive when it merges (as #56's did). Next B7 (ROADMAP).
- B7 done: ROADMAP: new #58 entry (144-150, 7 lines: draft; two review rounds; the report tells two misses first;
  next the verifier, the advisor agent, his go); #57's entry (own) -> [2026-11-17 to 11-18, MERGED] (title line was
  122 chars, now 113), its 145-147 unchanged, its last line replaced by 4 lines (the verifier PASS; report; merged PR
  #57, 22c6afa, 11:09:31 UTC, on his go after the advisor agent's APPROVE of f26ac5b; CI green on main gate 866 with
  smoke, toolchain 165; OLD-LINES pointer). Recent work = 12 entries. Folded #43 (:90) and #44 (:92-106) keep their
  bytes (two lines over 120 inside them: 1913 and 122 chars, not re-wrapped). Not touched: ROADMAP :120 "v2, next"
  (#55's, 9660197). Next B8 (REPO_MAP).
- B8 done: REPO_MAP: 10 new rows of mine, nothing removed or changed: after `crates/` (4: messages.rs, ts_export.rs,
  layout.rs, layout_ts.rs), after `packages/` (5: generated/ with the regenerate command and the target-folder rule,
  generated/protocol/, generated/block/layout.ts, layout-check.ts, index.ts), after #57's report (1: the cs1 report).
  Verified: layout.rs:61-66 (header stride 8, tick u32 at 0, layout_hash u32 at 4), :74-82 FNV-1a 32 bits;
  Cargo.toml [workspace.dependencies] holds only ts-rs; index.test.ts tests the exports. Map check: "repo map OK: 139
  entries cover 109 required paths", exit 0. Stale, other lanes', left: REPO_MAP :6 "Last checked: 2026-11-17" (#55,
  9660197); :16 `Cargo.toml` "[workspace.dependencies], empty for now" (#48, 550856a; now holds ts-rs); :18 `crates/`
  "each still an empty stub with one test" (#48, 790ac31); :21 `packages/` "each empty with one stub test" (#48,
  550856a). Next B9 (checks and the blame proof).
- B9 done: checks after all edits. STATUS 120 lines; no own prose line over 120 chars in the four files (table rows
  and moved lines aside); STATUS #58 row 6 pipes (5 cells), archive #57 row 5 pipes (4 cells); repo-map-check "repo map
  OK: 139 entries cover 109 required paths" exit 0; RECORD-FILES proof: node .checkpoints/cs1-types/docs-keeper-blame-
  check.mjs -> "34 removed lines checked; 0 not this lane's and not moved" (own 8, moved 24, blank 2), exit 0. Added
  "OLD-LINES, who may fold old lines in the shared record files" where my lines name it (STATUS row, archive row,
  ROADMAP #57 entry). numstat: STATUS +14 -14 (120), archive +13 -0 (1285), ROADMAP +33 -20 (270), REPO_MAP +10 -0
  (165). Untouched: DECISIONS, the report, ADRs, code, kit. origin/main still 22c6afa (fetched again).
- For the main session after the verifier: nothing of mine has a gap; the report's <<DOCS-KEEPER>> line (report :73)
  is the main session's to fill from this pass. Tell the caller: main's CI on 22c6afa is gate 866 / toolchain 165
  (867/166 ran for this branch's push of the same commit); if the verdict makes #58 "ready", only the #58 row's State
  cell and the ROADMAP #58 entry's last sentence change.
DONE
