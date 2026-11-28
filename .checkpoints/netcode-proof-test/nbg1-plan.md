# nbg1: R1's location stand-in, rehearsed (DRAFT 0.1, 18:5x UTC on 11-28; one screen)
Status: drafted on his "Continue" (18:33 UTC, history.md:8274-8288) in the advisor tool's order. It runs only on his
word: his create-form look (Stop B's message, For Mustafa 2) shows CCX23 or CAX21 not offered in `fsn1` and he picks
`nbg1` (R1.md:62-63: "else `nbg1` at the same prices"). Until then notes and a dry run only: no commit in spikes/
(re-fill trigger 1, stopb-plan.md A4). Dry run: below, "Dry run". B6's check: advisor-stopb-plan.md "## nbg1 plan read".

## What nbg1 changes: five lines in three files, all in place, net 0
| Line | From -> to | Why |
|---|---|---|
| ops/runs/r1.sh:34 | `LOCATION=fsn1` -> `LOCATION=nbg1` (its comment kept) | the fill (INF-SB-3); r1.sh uses it only to label machine records: RS's npt-super header (:265), LG's npt-load settings (:327-348) |
| ops/tests/r1sh.test.mjs:199 | `location: "fsn1"` -> `"nbg1"` | the pin on LG's record, built by r1.sh's own load_settings (:193) |
| ops/runs/R1.md:63 | "this plan's `fsn1`" -> "`nbg1`" | the fill named |
| ops/runs/R1.md:114 | "CAX21, `fsn1`," -> "`nbg1`," | his create step |
| ops/runs/R1.md:135 | "Hetzner `fsn1` ... bound to `fsn1`" -> `nbg1` twice | the provider-down line |
He creates both VMs by hand (R1.md:114), so no script creates anything in a location. No judge rule reads the value:
npt-judge asks only that it is not empty (figures.rs:939-950).

## Left as they are (named in R1's report as stale words if nbg1 is chosen)
- R1.md:62-63's "offered in `fsn1`, else `nbg1`": the Stop B procedure, still true.
- marks.json:11310 and :11369, the notes of the machine mark and of PM-0006-12's ping ("to one of the run's CAX21s in
  fsn1"): text no rule judges; marks.json is not edited for a location.
- Test data naming fsn1 (crates/log/src/lib.rs:815, :825; crates/judge/tests/engine.rs:92; crates/bots/src/load_tests.rs:16;
  ops/tests/wrappers.test.sh:132; 27 judge fixture files): sample values no run reads.
- ADR 0006:118, :273, ADR 0001:778, :1415, :1425, ADR 0004:1033 and SPIKE_PLAN.md:32, :209, :264, :373 record his
  answer of 2026-11-03 (fsn1). His nbg1 word goes to history.md and, by the docs lane, to DECISIONS.md; this lane edits
  no ADR.

## The chain if he picks nbg1 (stopb-plan.md C1-D3 with one change; about an hour, no money)
1. **N, the new Y0:** the five lines by Edit in the main folder, red first (r1.sh:34 alone turns r1sh.test.mjs:199 red,
   then the pin). Bars: `git diff --name-only HEAD` exactly r1.sh, r1sh.test.mjs, R1.md; `--numstat` `1 1`, `1 1`,
   `3 3`; the diff's sha256 equal to the dry run's; the counted run (main-run.sh, `--only self,ops,redact,refs`); the
   gate `--no-smoke`; one commit, pushed by the save; CI green.
2. **The dist list at N:** build 1 in .gate/stopb-a and build 2 in .gate/stopb-b, each first moved to N detached, with
   C3's recipe, CARGO_HOMEs outside every repository as at Y', the producers bar, both git lines and the GIT_ names
   unset (stopb-plan.md C2-C3, the Y' lines). Each list cmp-equal to r1-dist.sha256's committed non-comment lines:
   then **Y = N** and no list commit. That is the one change to C1's order, for B6: N touches no file under web/ or
   crates/ and no Cargo or npm file, so the dist cannot move, and the two builds prove it. A difference goes to B6 with
   both lists, before X.
3. **X'', the new X:** rs.yaml:69 and lg.yaml:52, Y' -> N in place. Bars: name-only exactly the two cloud-init files
   (R1.md's terms link is in already), `--numstat` `1 1` each, hunks printed; `git show X'':` of both holds N; rs.yaml
   under 32 KiB; the counted run; the gate; pushed by the save; CI green; B6's head read; both raw links fetched (200)
   and equal to `git show X'':`.
4. **To him:** the two new Cloud config links; his fixed addresses made in `nbg1` (R1.md:64 and :135: they are bound
   to their location; an fsn1 pair, if made, deleted by him); the rest of the Once list unchanged.

## Held, not sent: if no location offers CAX21
R1.md has no stand-in for the load machine (Stop B's message, For Mustafa 2) and its rule for refused VMs is "R1 waits"
(:135). Held question, sent only if his look reports it: R1 waits for capacity day by day (R2-R5 move with it), or LG
on another server type (a new plan: lg.yaml and the bots' build change, its own check). No lean until his look and
the price pages are read.

## Dry run: DONE 18:4x-18:50 UTC on 11-28 (logs in /home/user/Shamble-Frontlines/.gate/nbg1/; nbg1-dry.mjs)
- Worktree .gate/nbg1-dry, detached at X 6e623788; ops/browser installed in the lane's form (npm-ci.log, 167 packages).
- Red first: at X, r1sh.test.mjs 23 of 23 (t0-base.log); r1.sh:34 alone, 22 of 23, the one failure item 5 at
  r1sh.test.mjs:199:12, actual `nbg1` against the pinned `fsn1` (t1-red.log); with the pin, 23 of 23 (t2-green.log).
- All five lines: `node check.mjs --only self,ops,redact,refs` exit 0 (dry-counted.log): self 88 of 88; ops 104 of 104
  and wrappers.test.sh 315 ok, 0 failed; redact 11 of 11, 464 items, 0 findings; refs 2,245 files.
- `git diff --numstat HEAD -- spikes/netcode-proof` in the worktree: R1.md `3 3`, r1.sh `1 1`, r1sh.test.mjs `1 1`;
  the diff's sha256 f75d8e481784ffedb24b144fcb6e4a1c4ac8e2f693df547dc388c441afd94938, 5,557 bytes (dry-show.txt).
  N's counted run must print the same in main-run.sh's `diff` line (the spike folder at HEAD equals X's).
- fsn1 left in ops/ after the five lines (a grep): r1.sh:34's comment ("fsn1 or nbg1"), R1.md:63's procedure
  ("offered in `fsn1`, else `nbg1`") and wrappers.test.sh:132's sample header, whose 315 checks pass unchanged.
- Undone by the script (no diff, status clean), then `git worktree remove` in the plain form; free disk 5,819 MiB.
- Not rehearsed: step 2's two builds (they need N's commit) and step 3 (it needs N's SHA); both repeat Y''s and X's
  recorded runs (stopb-plan.md's Y' lines, D2's done line).
