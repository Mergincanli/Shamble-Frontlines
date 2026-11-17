# Advisor: C62H-BOUND report check (P6, order item 11)

Head under check: 3626a377f95c0a9dc0edb3bce49227f73caf733d on claude/netcode-proof-test-plan-hcky5g.
Role: origin/main:.claude/agents/advisor.md, read whole. Read only; this file is my only write.

## Plan
1. Confirm the head SHA and its parents; the commit list a5baab8..3626a37 and 7526ffe.
2. Append-only check: git diff a5baab8 3626a37 -- u4r-report.md; sha256 of the file at a5baab8 and at the head; line counts.
3. The (c) diff: git show 7526ffe -- u4r-report.md against the_c_lines (P3 line, P2 line, note line); B1 lines :1034 and :1166 unchanged.
4. Read the section (u4r-report.md 1201-1764) against order items 4-12 and P1-P7, item by item.
5. Spot-check the section's key numbers against sources (runs.txt, verdict.json, verdict-bites.json, merge logs, review JSONs).
6. The six Lows: read verify-r2-full.json; diff a5baab8..head on the draft (+18 -12); judge each against the Low text and source.
7. Draft-to-report binding: the draft at head sha256 = e4af2d54...; the insert script; the section equals the draft.
8. P6: git diff --quiet ddd38ba 3626a37 -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'; exit code.
9. CI ruling (item 10): the [skip ci] rule in .claude/CHECKPOINTS.md section 5; what changed after a59e0ee.
10. C62HB-Q4 (b): read the design's C62HB-Q4; confirm or give mine.
11. Verdict, findings, conditions, to_mustafa.

## Log (one line per check)
- 1. Head 3626a37 parent 8c72ab7 parent a5baab8; since a59e0ee (last CI run) 18 commits, all [skip ci], and every changed path is under .checkpoints/ (git diff --name-only --no-renames; --raw: all 100644, no symlink or submodule).
- 2. Append-only: diff a5baab8..head on u4r-report.md is one hunk @@ -1197,3 +1197,568 @@ with no removed line. sha256 777ee782... (1199 lines) -> c5dee3fa... (1764 lines). Line 1200 blank, 1201 the heading. tail -n +1201 sha256 = e4af2d54... = the draft at head. Draft at a5baab8 = 5d8fb338... (the verified one). Insert script checks the before-sha, draft sha, prefix, cited lines 997/1008/1035/1167.
- 3. (c) diff 7526ffe: +3 -2 on u4r-report.md: the old :996 and :1007 replaced, the note added at :990. My python read of the ruling's the_c_lines: each of the three texts equals exactly one whole line (997, 1008, 990) at 7526ffe and at the head. Report at 7526ffe sha256 777ee782 (= a5baab8's). B1 lines :1035/:1167 (were :1034/:1166) byte-equal before and after. The script checks A/B 0/0/0, tallies, resyncs, verdict path, bites 48/48 against the files and exits 1 on difference. P2 met.
- 4. Read the section 1201-1764 whole (18 numbered parts + Decisions needed + For Mustafa).
- 8. P6: git diff --quiet ddd38ba 3626a37 -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results' -> exit 0. Only adds under spikes/ since ddd38ba: the 8 results files (P0 rejudge 3, P2 3, P3 2). Brief sha256 at head 7936074c... = C8.
- 5a. runs.txt P3 (head): S/L/B, E1/E2/N, incomplete, resyncs, stop, fate logs, hashes all equal the report's section 10. P2 (head) incl. both bite lines: all equal (bite whole-as-lost hashes 861,948, incomplete 17,308, N 48; link-lost-overstated equal to s1). L/S 2.00/1.98/2.03 % right. marks c1935a5b, judge d8b8c91e, lab 98c3967e in both runs.txt.
- 5b. verdict.json P2 and P3: -2/-17/-57 PASS, each run valid, the three -17 checks 0, no gaps key. verdict-bites P2: whole-as-lost A 48 fail, B 0, mism 0; link-lost-overstated B 48 fail, A 0, mism 0; tick16, client-rounding no runs.
- 5c. P0 re-judge files: verdict e5ed24e2, bites 13b6b107; earlier 93509dfd7fd1/P0 verdict e926b812, bites b472784f: equal to section 8. cb37f56, daec788, 8ca774a exist with those subjects/files.
- 5d. Size: numstat 044fab1..561bf42 (results excluded) 14 files +904 -63 = 967; per file all 14 equal section 1; cumulative 499 / 821 / 920 / 967 at 54046d4 / b423a7d / b01d3ac / 42b9646 (steps 499, 322, 99, 47). 212 over 755, 92 over 875, 83 under 1,050; paste:87 has 545-755.
- 5e. All 13 commit SHAs named exist with the right subjects and times. pre-P3 log: header free 5306, df 5,563,871,232 B = 5,306.1 MiB (report's correction to 5,306 is right). P2: runner line 1 free 6057 (report's 6,057), pre-P2 header 6056; both >= 5,500.
- 5f. Review JSONs 3bf8888c / 5adfdf18 / 3f39ade6; every deferred Low's file:line in section 15 equals its JSON's 'where'. Taker pooled ("the next npt-judge or npt-lab change, U4f, U4g or JDG-ONLY-1") = the merge-stop ruling's own wording and card :3776-3779 at 67de11e. U3C-HELD-1 :3760-3773, CHECK-NFF-1 :3774-3775 right.
- 5g. history.md 2127 (13:35 rule), 2143-2146 (C62H-BOUND (a); :2146 (c) lines), 2172 (size), 2174 (P0 re-judge), 2175 (stop rules), 2187 (21:50 command forms): all right.
- 5h. P4: c62h-bound-build.md:515-533 hold P3's lines + the ruling; present at 9a0e687 (21:55:59), before the draft's first commit b3fb754 (22:11). :428-435, :460-472, :474-489, :422-423 right.
- 5i. Ruling hashes: P3 ruling facf298e (71 lines), head check a64259f5, merge-stop 55ad3561: right. merge.log:51-52 name the test and 429/1; merge-2.log :12 552/632, :19 all 11, :30 det-u4 215.5 s (200.8+2.5+0.9 = 204.2), :54-55 SS-P0-s4 pin, :57 real 1m55.935s. 429+1+202 = 632. No "recorded as incomplete", no "not at all", no placeholder in the report.
- 5j. P5 on disk now: u4r-P0-93509dfd7fd1, u4r-P0-051995db9cea, u4r-P2-fc411116c1f1, u4r-P3-fd4612050cd6, both c-u2 folders present; u4r-P2-ab10f058a828 gone (the one packed).
- 6. Six Lows vs diff a5baab8..head on the draft (+18 -12): L1 date "of 2026-11-15" right (history.md:2113 heading 2026-11-15 13:35 Amsterdam). L2 P6 wording right (my run: e2d06ec exit 0, a5baab8 exit 0; e2d06ec carries 3b826b4's draft, i.e. round 1's text, e0fd859 changed it after). L3 c-u2 folders added, right (P5). L4 PR #54 "waits for your go", 22:09 Amsterdam, 22:11 UTC (23:11 Amsterdam): right (card 67de11e :164-180). L5 5,306 with the log named: right. L6 draft :115, :213 rewrapped, no draft line > 120 now; the notes' :243 (156 chars) left as one line by design (the card says rewrapping would move the cited lines): right call, recorded.
- 9. CI: gh run list on a59e0ee: gate + toolchain success on push and pull_request (43961950359, 43961950485, 43961962912, 43961962919); gate runs' jobs gate and smoke success (both). No run on 3626a37 ([skip ci]). git diff --quiet a59e0ee 3626a37 -- . ':(exclude).checkpoints' exit 0; neither workflow nor gate.mjs/smoke.mjs reads .checkpoints. CI on ddd38ba: 4 runs success (43911726413, 43911714149, 43911727938, 43911713654) - now read by me.
- Note: PR #40's head has moved to bf752f3 (cd91771, 7cce5af, bf752f3: card + my notes only, [skip ci]); u4r-report.md and spikes unchanged after 3626a37.
