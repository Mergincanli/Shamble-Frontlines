# U8 last round: netcode seat, NC-R2-1's fix (pass 3 item 1) only

Plan: (1) check snapshot hash; read advisor-u8-r2.md section 3 item 1 and :126-139; u8-build.md "## Pass 3" item 3.1.
(2) interdiff r1.sh (and any other file item 1 touches) old (.gate/u8-r3-old) vs new; isolate item 1's hunks.
(3) judge: cut log kept as whole-line prefix; judge unchanged; run INVALID; LG's own end awaited; one LG fetch per block.
(4) the coordinator's npt-judge run (.gate/u8-p3/judge.log, verdicts.json). (5) new findings; verdict.

Status: all parts done (resumed after the 17:33-21:10 UTC spend-limit cut; nothing in either tree changed).

## Notes
- **Hashes:** wip/u8-build.patch 0e7fbab2...b15 (matches; re-checked after the cut); advisor-u8-r2.md d107cbe3...a784
  (matches); u8-build.md 9f9a9478 (read "## Pass 3", item 3.1, and the later items' notes only to separate hunks).
- **The judge is unchanged:** the patch's 43 `diff --git` entries all sit under spikes/netcode-proof/ops/; none touches
  crates/.
- **Item 1's hunks in r1.sh** (W = .claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof/ops/runs/r1.sh):
  - save_log (:401-433);
  - keep through save_log (:438-443);
  - lg_keep (:446-452);
  - headers, calibrate and ds call lg_keep (:563, :661, :688);
  - the probe block (:577-590): LGK set after `lg start`; the probe's exit; room_down; stat-after;
    `load_wait 02 120`; lg_keep. There is no `lg stop` in the block; load_wait's abort (:399) is the only stop path;
  - the collect comment (:765-767) and R1.md section 8's line (:310-312).
  - Out of my scope but shared: down_all's LGK path (:96-98, item 3.8) calls the same lg_keep.
- **save_log, case by case** (python, :413-432):
  - Only newline-ended lines are considered (`split(b"\n")[:-1]`); the prefix ends at the first whole line that is
    not one JSON object.
  - A whole log, or one cut at a line end, is copied with copy2. read_log reads the latter with no footer, so the
    complete rule makes its run INVALID.
  - A cut inside a line keeps its whole lines as `<name>` (each parses), the original as `<name>.raw`, and a line in
    cut.txt.
  - An empty file, a cut first line, or a first line that is not a header goes to `<name>.empty`.
  - judge/src/lib.rs:27 reads only `*.jsonl`, so `.raw` and `.empty` are never read.
  - Records never span lines: serde_json escapes control characters (M log/src/lib.rs:602-605), so the newline split
    is exact.
- **LG's own end in the probe block:** LG runs PROBE_TOTAL_S + 60 = 792 s from its start; the probe exits at about
  732-740 s. load_wait's 120 polls (each an ssh plus 1 s, about 130-160 s) cover the remaining ~55 s, or about 95 s if
  the probe ends early.
  - If the probe dies very early, load_wait expires, its abort stops LG, and the cut copy is kept as a prefix. That
    makes only the npt-super-plus-LG run (R1-probe-02, process super and load) INVALID; that run is in no part of -3.
    The legs' own runs (R1-probe-02-<leg>) are untouched.
- **One LG fetch per block:** `lg "npt-log` appears only in lg_keep (:449). lg_keep runs once per LG block and clears
  LGK first, so down_all's EXIT path fetches only when a block died between LG's start and its lg_keep. That path
  stops LG first and saves the cut copy through save_log.
- **Tests read:** r1sh.test.mjs:635-697.
  - Six save_log forms: whole; cut at a line end; cut inside a line; empty; no header; cut inside the header. Each
    kept line is checked with JSON.parse.
  - The probe trace: `load_wait 02 120` comes after `lg start 02` and before `lg npt-log 02`, and there is no
    `lg stop 02`. load-02.jsonl holds the 3 whole lines, `.raw` holds the cut copy, and cut.txt names it.
  - The results tree reads whole under the stand-in, while the copy as fetched fails at line 4.
  - The plant (the probe's fetch back to a direct `> load-02.jsonl`) fails 3 tests by the builder's record. I did not
    re-run it.
- **The main session's real npt-judge run** (.gate/u8-p3/judge.log, 17:08 UTC at 627401d) over
  .gate/u8-p3-judge-tree:
  - It exits 0. PM-0001-2 and PM-0001-4 read INVALID because "run s30-2: complete: the bots log on LG1 has no footer
    (cut short)". load-07.jsonl.empty and s30-2-lg.jsonl.raw were not read.
  - The tree's cut.txt names both files. This is the ruling's proof that the builder could not give.

## NC-R2-1: CLOSED
Every part of the ruling's wording holds in W and in the judge's real run:
- A cut log is kept as its whole-line prefix, every line parsing, with the original beside it as `.raw`; an empty copy
  goes to `.empty`.
- The judge is unchanged, and the run holding the cut copy reads INVALID by the complete rule while the whole-tree
  call exits 0.
- The probe block waits for LG's own end (`load_wait 02 120`) before any `lg stop`, which is now only load_wait's abort.
- Each LG block fetches LG's log once, through lg_keep. The room, npt-super and leg logs also pass through save_log,
  so a room killed at npt-super's deadline no longer breaks the whole-tree read (ADV-SB-1's case).

## Findings
| Id | Sev | Where (W, ops/runs/r1.sh) | What fails | Smallest fix |
|---|---|---|---|---|
| NC-R3-1 | Low | :438-443, :446-452 (keep, lg_keep) | keep's old `cp -p ... \|\| true` ignored a copy failure. save_log's python is now unguarded under `set -e`: one unreadable log, or a full disk, ends the block inside keep. For the probe, that skips the remaining legs' copies, probe_presence and INVALID-presence.txt for that attempt; the block reads stopped and a new attempt may run, so nothing is judged wrongly. | Either `save_log ... \|\| printf '...: not kept\n' >> "$d/cut.txt"` per call, or keep the stop and say in R1.md that a log that cannot be kept stops its block. |

A note, below Low and not a finding: a log cut between a record's closing brace and its newline would be treated as
cut inside a line, so save_log would drop that last record, even when it is a footer that read_log would have kept.
The writer sends the record and its newline as two writes into its BufWriter (M log/src/lib.rs:602-605), so that byte
boundary can reach the file only when the buffer fills exactly at the record's end and the process dies before its
next write. Even then the result is only a stricter INVALID.

Count: 0 Critical, 0 High, 0 Medium, 1 Low. Nothing here stops U8.

## What I did not verify
- I ran no test, no plant and no npt-judge. The judge's result is the main session's run (judge.log above), and the
  plant failures are the builder's record.
- U6c's npt-load is not built: its log writer, its footer at the end of its duration, and its TERM behaviour are read
  from the briefs.
  - The probe block's normal path no longer depends on TERM.
  - A headers run whose LG log lacks a footer at a normal end would read INVALID by the complete rule. That is U6c's
    carried term (advisor-u8-stopb.md:214-216), not this item.
- No VM: the ssh times behind load_wait's 120-poll bound are estimates.
- Items 3.2-3.8's hunks were not reviewed (outside this seat's scope), apart from down_all's shared use of lg_keep.

DONE
