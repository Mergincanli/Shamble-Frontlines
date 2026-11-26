# docs-keeper: guard-wrappers docs pass (worktree .gate/merge-main; ids only, no forms)
Plan (blocks):
K1 DECISIONS: 8 entries at top of "Decisions, newest first" (by script from mustafa-*.md, byte-checked)
K2 DECISIONS "Queued": MAIN-WALL; guard change-set (GUARD-KIT + R4-1 fix); review tracks + SOURCE-GO (a); MAIN-WALL measurement rule
K3 STATUS: outcome, R4-1 open line, interim rule, queue, follow-up pointer, #69 follow-ups (GEF-SAME, SA-R1-3 closed; CAP closed), #70 merged row moved to archive by script; <=120 lines
K4 STATUS_ARCHIVE: follow-ups block, open risks (R4-1, INPUT-RUN, class words), advisor rulings, In flight
K5 ROADMAP entry; #69 entry marked merged
K6 REPO_MAP: report line (+ guard-env-fix report line, missing); repo-map-check
Notes:
- Orientation done: 8 message files match bytes and sha256 given by the caller.
- K1 done: 8 entries inserted by timestamp (top: 14:09:28, 12:24:26, 23:01:07; others interleaved); round trip byte-equal for all 8.
- K2 done: Queued group added before the pointers line (53 lines), each quoted line a whole source line.
- K3a: STATUS lines moved by script to archive (#70 row; 3b, 4a, ci-after-3a follow-up bullets); 11 removed, all byte-equal in archive.
- K3b: STATUS edited (phase bullet with R4-1 class line and interim rule by script; in-flight row; Next 4; waiting; follow-ups; risks).
- K3c: #67's follow-ups bullet (3 lines) moved by script to the archive.
- K3 done: STATUS 120 lines; CI on 2483e9a read (gate, smoke, toolchain success); kit suites run here 1551/127/33.
- K4 done: archive follow-ups block, advisor rulings, open risks entry, two Health rows.
- K5 done: ROADMAP entry added (12 in Recent work); #69 entry marked MERGED.
- K6 done: REPO_MAP report lines (guard-env-fix, guard-wrappers); map check run.
- Final checks: STATUS 120; map OK; 11 <SHA-NOTES> placeholders (DECISIONS 9, archive 2); 8 entries byte-equal; deny-list grep clean.
- After advisor review: wording fixes (no merge claimed; closed-for-forms-run; placeholders all parenthesised, 11); lane-2 stale lines fixed; checks re-run: 8 entries equal, 14 moved lines verbatim, STATUS 120, map OK. DONE
