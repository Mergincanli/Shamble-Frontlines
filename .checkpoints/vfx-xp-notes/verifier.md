# Verifier notes: vfx-xp-notes (2026-11-07)

Role read from `origin/main:.claude/agents/verifier.md` and `origin/main:CLAUDE.md`. Worktree at e967196, base main
4c6db5b. Docs only; symbols expected +0 −0. Scratch under `.gate/vfx-verifier/`.

## Plan
1. E1: run `e-checks.py 4c6db5b`; check its method against mustafa-words.md; words file against the chat events
   (get_event if available, else a79fef1's copy); STATUS: VFX queue entry gone + pointer, A4 entries unchanged vs
   4c6db5b, comic-flavour line quoted once.
2. E2/E3: own diff of EDITOR_V2 and VISION_V2 vs 4c6db5b; tag counts recomputed independently.
3. E4: every line citation into EDITOR_V2 and VISION_V2 (incl. ADR 0005:27, :48-50, :2550, R1's record) checked at
   this head, or recorded as a follow-up with true numbers.
4. E5: STATUS, ROADMAP, B.2 report facts: merge commit via git; merged_at, comment 6698888516, CI runs (C10), #40 at
   0588a17 (C9) via read-only GitHub API if available.
5. E6: gate.mjs 4c6db5b --no-smoke; repo-map-check.
6. E7: changed file list.
7. C1-C10 each with file:line.
8. Changed lines: untrue, stale, unclear; widths over 120 characters (outside STATUS:3, table rows, ROADMAP entries).

## Results
### E1 (his words)
- `e-checks.py 4c6db5b`: RESULT PASS, 30 PASS 0 FAIL.
- Method: `joined()` strips tags and joins wrapped lines AND bullet boundaries, so on its own it would pass a line
  split across two bullets, and it does not check each line carries its tag (E3 counts cover the tags only in total).
  My stricter check (in memory): EDITOR_V2 §7 has exactly 10 bullets, each equal to "`[DECIDED]` " + his line, in his
  order: TRUE. VISION_V2:71-75: "`[DECIDED]` " + rule and "`[OPEN]` Open, ..." exact: TRUE.
- get_event 0f5f8a45 (2026-11-06T23:27:12Z) and 634a1bb3 (23:50:53Z): equal mustafa-words.md's two blocks byte for
  byte: TRUE both. a79fef1's saved copy contains the VFX block: TRUE.
### E5 (facts, read-only GitHub API with gh, and git)
- git: 4c6db5b parents ee736ec e223ecc, committed 2026-11-07 00:57:34 +0100 (23:57:34 UTC). API PR #42: merged_at
  2026-11-06T23:57:35Z, merge_commit_sha 4c6db5b, head e223ecc. CONFIRMED.
- Comment 6698888516 (Mergincanli, 23:47:12Z): "APPROVE WITH NOTES, head e223ecc..." with four Low notes. STATUS:33
  says "APPROVE WITH NOTES" CONFIRMED (the checkpoint's "APPROVE" is shorter but not wrong).
- Run 42315225217: push on 4c6db5b, gate, success. CONFIRMED.
- C10: run 42313665674 (pull_request, e223ecc): attempt 1 smoke step "Install Chromium's system libraries (browser
  restored from the cache)" 23:42:39-23:49:13 = 6 min 34 s, cancelled; attempt 2 23:49:58-23:56:47 = 6 min 49 s,
  success, then "Smoke (check 5a)" passed. Push run 42313658977 same step 23:42:32-23:42:43 = 11 s. All CONFIRMED.
- C9: at 0588a17, #40's STATUS :3 ("its readings PR #41 (needs-mustafa: waiting for his go)"), :31 ("PR #41 waits
  for his go"), :173 ("Open: his go to merge PR #41") are stale as the note says. CONFIRMED. (#40's head is now
  fe3ce62, not 10b1a53 as the advisor read; the correction names 0588a17 as "the head the note checked": true.)
- B.2 report corrections :71-82: note 1 (STATUS:500 "Next after B.2" in the VFX entry, gone with it): true; note 2
  (:3, :31, :173 at 0588a17): true; note 3 (Risks 1, report:53, "at the level of principle only; which units are
  shared stays open"): quote exact; note 4 (e-checks.py at 5c0999f, old C9 rule; Appendix B title "VISION_V2:36-37
  at ee736ec", report:83 at e223ecc): true.
- ROADMAP :61 (docs-after-r7 folded into R7's entry), :74, :94, :96: true against git and the API.
### STATUS queue
- The VFX queue entry (base :498-520) removed; pointer at head :550-551. A4's entries (head :523-548) byte-identical
  to base :521-546 (no diff hunk touches them). "Comic flavour: ink-outlined smoke" appears once in STATUS at base
  (the queue quote) and once at head (the LOOK follow-up, :512): intended.
### E4 (line citations into EDITOR_V2 and VISION_V2)
- The script's pattern `(EDITOR|VISION)_V2(\.md)?:[0-9]` misses "(lines A–B)" and "EDITOR_V2 :N" forms; I grepped
  those too (whole repo but reports, then reports).
- VISION_V2 at head: §2 :27, §3 :45, §4 :54, §5 :60, §6 :80 (last item :99), §7 :101; base had §6 :75, §7 :96.
  ADR 0005:27 "62–81", :48 "25–40", :49 "42–49", :50 "51–55", :2550 "25–81" unedited. STATUS:488-491's numbers at
  this head (80–99, 27–43, 45–52, 54–58, 27–99): all TRUE. The 100 ms line is VISION_V2:58: STATUS:501 TRUE.
- ADR 0005:2037, :2038 (and its CD text) and :2550-2551 cite EDITOR_V2 "lines 4–5": unmoved (header line goes at :7):
  TRUE. LEGACY_INVENTORY:324 cites EDITOR_V2 by § only. ROADMAP:104's "VISION_V2 lines 36-37, 59 and 86 (as numbered
  before B.2)": dated, fine.
- STATUS:660 EDITOR_V2:94 ("It still runs by double-clicking editor.html") and :95 ("if ADR-002 moves..."): TRUE.
- STATUS:488, :495 "VISION_V2:71-75": TRUE (the XP block is :71-75).
- NOT RECORDED: `docs/research/r1-simulation-core.md:3631` cites `EDITOR_V2.md:39` (af2560c's "Validation panel"
  line); it was :47 at base (the B.1 report:51-52 says so, dated) and is :48 at this head. No STATUS follow-up names
  it; STATUS:500-502 names only R1's :3301 and :3535. (Finding.)
- Other citations in the changed lines, checked at this head: LOOP_V2:242 (weapon XP `[DECIDED]`), :237-238 (tracks
  reset each match), :140 ("the existing career rank"); DESIGN_NOTES:23 ("account XP"); LEGACY_INVENTORY:302 (GE15);
  ADR 0006:474 ("private flag"), :475 ("humans and bots per team"); LOOK_V2:60 (§7 Juice), :63 (`[DEFAULT]` "Comic
  impact touches for big moments," used sparingly); README:69 (AB3), :104, :122, :160, :183 (the four rows), :408 (J3,
  "no feature for `[LATER]` or `[OPEN]` items"); VISION_V2 §6.3 (:87, "ability and effect system"). All TRUE.
### E2, E3
- My own diff: EDITOR_V2 = one insert at :7 (header line) and one at :114-135 (blank + §7); VISION_V2 = one insert at
  :71-75; README = four rows changed (:104, :122, :160, :183), each an appended clause, old text kept. Headings of
  EDITOR_V2 unchanged, §7 last. Tags: EDITOR_V2 `[DECIDED]` 11→21, `[LATER]` 2→3; VISION_V2 `[DECIDED]` 12→13,
  `[OPEN]` 3→4; others unchanged; as the checkpoint states. LOOK_V2 untouched.
### E6, E7, widths, CI
- `node tools/gate.mjs 4c6db5b --no-smoke`: GATE PASS; [1] 5/5, [2] PASS, [3] +0 −0, [4] 51 guarded 0 flagged,
  [5b] [5c] [5d] PASS, [5a] skipped. `repo-map-check.mjs`: OK, 113 entries cover 100 required paths.
- `git diff --name-only 4c6db5b...HEAD`: the six docs (EDITOR_V2, VISION_V2, adr/README, STATUS, ROADMAP, the B.2
  report) and 10 files under `.checkpoints/vfx-xp-notes*`. Nothing else.
- Added lines over 120 characters: only STATUS:3, :106, :107 (table rows), README :104/:122/:160/:183 (table rows),
  ROADMAP :61/:74/:94/:96 (entries): all exempt.
- CI on e967196: pull_request run 42317256265 and push run 42317250184, gate workflow, success.

## The advisor's conditions
- C1 MET (EDITOR_V2:115-135, VISION_V2:71-75; byte-checked above). The backtick note ("his `[DECIDED]` was typed
  without backticks") belongs to the report: PENDING (report).
- C2 MET: STATUS:505-510 lists LOOP_V2:242, :237-238, README:69, DESIGN_NOTES:23, LEGACY_INVENTORY:302, LOOP_V2:140,
  maps none; ROADMAP:94 "none mapped". No changed doc line repeats the false premise.
- C3: STATUS:509-510 points at the report's "Decisions needed" (XP-TERM). The entry itself: PENDING (report).
- C4 MET: README:183 (rule quoted; "Which progression 'XP' means, and telemetry, stay open": Service points left open
  in substance), :122 ("Whether it and the Feel Lab are one tool is not said"), :160 ("and (2026-11-06) his XP rule
  for private (rented) matches"), :104 (his Cost and quality line, decides nothing). No ADR file changed.
- C5: STATUS:511-515 states the tension, edits no LOOK_V2 line, style guide "a follow-up only, not queued". The
  plain question with options: PENDING (report).
- C6 MET: EDITOR_V2:7 dated 2026-11-07 with "item of 2026-11-06"; STATUS:660 :94/:95; STATUS:488-491 ranges true.
- C7 MET: STATUS:516-520 (J3, README:408; §6.3 is gameplay effects, not cited as the need). It omits the advisor's
  "today's visual effects trace to LOOP_V2 §7, LOOK_V2 §3/§7 and §6.9 (ADR 0003 part 5.k)" (finding 5, optional).
- C8 MET: STATUS:483-486 (no field now; ADR 0006:474-475), :495-497 ("bears on it and does not answer it").
- C9 MET: B.2 report:75-76 (:3, :31, :173 at 0588a17), confirmed at 0588a17.
- C10 MET: STATUS:790-794, every number confirmed with the API; STATUS:107 "stalled once"; ROADMAP:96 "stalled ...
  and passed on a re-run". No `.github/` change.

## Verdict: PASS WITH NOTES
His words are byte for byte (against the chat events), the scope is the six docs plus notes, the gate passes +0 −0,
every merge and CI fact is confirmed, and C1-C10 are met in the diff except the parts only the report can meet.

## Findings
1. Low. R1's research record `docs/research/r1-simulation-core.md:3631` cites `EDITOR_V2.md:39` (the "Validation
   panel" line at af2560c). It was :47 at base and this pass's header line moves it to :48. STATUS's research-citation
   follow-up (STATUS:500-502) names only R1's :3301 and :3535, so the brief's "right at this head or recorded with
   true numbers" is not met for this one. Fix: add to STATUS:500-502 "and `:3631` cites `docs/EDITOR_V2.md:39` (the
   validation panel; :47 after B.1, :48 at d5e2c62)".
2. Low. STATUS:106 says "four follow-ups 'From vfx-xp-notes'", but STATUS:503-520 holds three items (XP-TERM, LOOK,
   the need rule; the style guide is a sentence inside LOOK). Fix: "three follow-ups", or name the style guide as the
   fourth as ROADMAP:94 does.
3. Low. STATUS:3 says B.2 merged "after the advisor's APPROVE of e223ecc"; STATUS:33, :107, ROADMAP:96 and the comment
   (6698888516) say "APPROVE WITH NOTES". Fix: "APPROVE WITH NOTES" at STATUS:3.
4. Low (evidence method). `e-checks.py` E4's pattern `(EDITOR|VISION)_V2(\.md)?:[0-9]` misses "(lines A–B)" and
   "EDITOR_V2 :N" forms, and its E1 `joined()` would pass a line split across two bullets. My wider checks found
   nothing wrong beyond finding 1. Fix: say so in the report's Evidence (or widen the pattern and add the
   per-bullet check).
5. Low, optional. C7's background sentence (today's visual effects trace to LOOP_V2 §7, LOOK_V2 §3/§7 and VISION_V2
   §6.9, ADR 0003 part 5.k) is not in STATUS:516-520. Fix: append it, or leave it to the report.
- Pending, not findings: C1's backtick note, C3's XP-TERM entry with options (a)-(d), and the LOOK question (C5) go
  in the report; `.checkpoints/vfx-xp-notes*` must be deleted in the last commit before the PR is marked ready.
- Not verified: nothing player-facing; no Brave check applies. Smoke (5a) skipped locally; CI's smoke ran green on
  e967196.
- Head note: while I worked, the session committed a814ce9 and 08ac3fb (notes only: art-director.md,
  game-designer.md and this file's plan). `git diff e967196 08ac3fb -- docs` is empty, so this review holds for both.
