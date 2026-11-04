# builder: kit-v3-2 (A2) build in worktree scratchpad/a2 (main 8bcc65e + checkpoint commits); no commit
## Plan (blocks, in order)
1. plan-rev2.md: rev1 + 11 conditions + Mustafa's answers (all recommendations), condition map.
2. CHECKPOINTS.md: C1 (with cond 3), C2, C3, C4, C5 (with cond 4), C6, C7, C8, C9, cond 5 (paste-block line), C10, C11, C12.
3. CLAUDE.md: M1, M2, M3, T1, T2.
4. art-director.md: T3 (cond 9), T4, cond 8 (line 27). graphics-engineer.md: T5.
5. verifier.md S1, S2; perf-auditor.md S3; rust-reviewer.md S4.
6. changeset SKILL.md K1; NIGHT_SHIFT.md N1; advisor.md cond 2; tools/README.md:4 (cond 6).
7. Evidence: E1-E8, E10 (greps, E3 table, E4 two-way table, E7 citations, E8 widths), repo-map-check, gate --no-smoke, node --test.
## Log
- baseline before edits: see E7 line below once taken.
- block 1 done: plan-rev2.md written (rev1 + 11 conditions + answers; §0 condition map; new edits C9a, T6, AD1, D1; E10 defined). next: block 2 CHECKPOINTS.md
- block 2 done: CHECKPOINTS.md C1(cond3) C2 C3 C4 C5(cond4) C6 C7 C8 C9 C9a(cond5) C10 C11 C12 = 13 edits applied exactly. next: block 3 CLAUDE.md
- block 3 done: CLAUDE.md M1 M2 M3 T1 T2 = 5 edits applied exactly. next: block 4 art-director + graphics-engineer
- block 4 done: art-director T3(cond9) T4 T6(cond8) = 3 edits; graphics-engineer T5 = 1 edit (rewrapped, words as plan). next: block 5
- block 5 done: verifier S1 S2 = 2; perf-auditor S3 = 1; rust-reviewer S4 = 1 (line 13 split before 'baselines.' to stay <=120). next: block 6
- block 6 done: SKILL K1 =1; NIGHT_SHIFT N1 =1 (item 7 rewrapped, one line longer); advisor AD1 =1; tools/README D1 =1 (line 4 now <=120). All edits applied. next: block 7 evidence
- E8 PASS: 111 added lines, max 120 chars (python, chars; awk counts bytes and flags NIGHT_SHIFT:44 at 121 bytes = 119 chars + §).
- E1 PASS: every phrase once in CHECKPOINTS.md; `.gate/<task-slug>/` with backticks once (C7, line 95), plain in paste block (line 118); "then\n  your plan" at 113-114.
- E2 PASS for CLAUDE.md + .claude/: 0 hits on all 10 old texts (incl. upbeat, cond 7). "toy-box" elsewhere: README.md:4, GAME_DESIGN.md:14,17 (expected) plus records quoting it: STATUS.md:360-361 (KIT-ALIGN list, docs-keeper's), docs/reports/2026-11-01-* (history). "toy-soldier" hits = legacy death fall, unrelated.
- E7 PASS: 9 sections, headings unchanged (0 '## ' lines in diff); 28 main citations all still present (diff before/after: only +1 new, CLAUDE.md M3 `.claude/CHECKPOINTS.md` §4; K1's new §4 pointer wraps across a line so the one-line grep misses it); 'Checkpoints" N' cites: 0.
- E10 PASS: all condition texts found once; 0 "if AGENT-CAP/SCRATCH-HOME/SAVER-CHECK/LOOK-LINES" in the repo diff.
- E6 file list (diff vs 8bcc65e, .checkpoints aside): exactly the 11 planned files.
- E3 table (144 grep hits; one row per rule; "block" hits for VERDICT BLOCK, SIM block, warm-up block, blockers = unrelated):
  | Rule | Hits | Verdict |
  |---|---|---|
  | Agent cap 3 | CHECKPOINTS:66-70,159-160; CLAUDE:150,225; changeset SKILL:25; NIGHT_SHIFT:44; advisor:59 | agrees/restates |
  | "parallel" / "one at a time" | CLAUDE:56,63,211-212; 7 agent files "merged one at a time"; CHECKPOINTS:80; NIGHT_SHIFT:4 | agrees (worktree merges, one committer, one night run) |
  | Plan first + block saving | CHECKPOINTS:71-78,87-90,99-118; CLAUDE:150-152; CLAUDE:57 / START_HERE:30 (change-set PLAN FIRST, other sense) | agrees |
  | Scratch in notes, builds in .gate/ | CHECKPOINTS:91-98,117-118; CLAUDE:153-154,217-218; verifier:10,25; perf-auditor:11; rust-reviewer:13; tools/README:4-5 (D1); render-ab SKILL:8,14; tools/README:27,56,58 (tool output); settings.json:49-50,71,73 (allow-list, mechanism) | agrees |
  | Scratch in .gate/ (old) | docs/START_HERE.md:112 "Scratch output goes under `.gate/`"; docs/REPO_MAP.md:24 (not in the grep set) | CONTRADICTS until docs-keeper (cond 6) |
  | Saver check | CHECKPOINTS:49-50,62-63,122,135-142,148; CLAUDE:154-155,185; NIGHT_SHIFT:43-44,57 | agrees |
  | Code (guard.mjs, routine-guard.mjs/.test, settings.test, checkpoint.mjs/.test) | 40 hits | mechanism, unchanged; agrees |
- E4 two-way table: items -> sections: 1 -> §1-§3; 2 -> §4 (cap, plan first, blocks, look first) + §7 (cap in workflows); 3 -> §4 (scratch, never commit) + §3/§5 (saver, check); 4 -> §6; 5 -> §8; 6 -> §9. New rules -> items: C3 -> 3; C4 -> 2; C5 -> 2; C7 -> 3; C11 -> 3; C12 -> 2. Every cell filled: PASS.
- E5 PASS: repo-map-check "113 entries cover 100 required paths", exit 0.
- E6 PASS: gate --no-smoke GATE PASS, symbols +0 -0 (base 8bcc65e via origin/main); node --test 339/339 pass.
- E9: not run (main session).
DONE
