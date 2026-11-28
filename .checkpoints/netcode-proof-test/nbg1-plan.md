# nbg1: R1's location stand-in, rehearsed (rev 0.2, 19:0x UTC on 11-28; B6's NB-1 to NB-4 applied; one screen)
Status: drafted on his "Continue" (18:33 UTC, history.md:8274-8288) in the advisor tool's order. It runs only on his
word: his create-form look (Stop B's message, For Mustafa 2) shows CCX23 or CAX21 not offered in `fsn1` and he picks
`nbg1` (R1.md:62-63: "else `nbg1` at the same prices"). Until then notes and a dry run only: no commit in spikes/
(re-fill trigger 1, stopb-plan.md A4). Dry run: below, "Dry run". B6's check: APPROVE WITH CONDITIONS NB-1 to NB-4,
all Low (advisor-stopb-plan.md:957-1087, COMPLETE 18:57 UTC), applied below.

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
   gate `--no-smoke`; the location grep repeated at N, `git grep -n -i fsn1 N -- spikes/netcode-proof/ops`, showing
   only r1.sh:34's comment, R1.md:63's procedure and wrappers.test.sh:132 (NB-1); one commit, pushed by the save; CI
   green.
2. **The dist list at N:** build 1 in .gate/stopb-a and build 2 in .gate/stopb-b, each first moved to N detached, with
   C3's recipe, CARGO_HOMEs outside every repository as at Y', the producers bar, both git lines and the GIT_ names
   unset (stopb-plan.md C2-C3, the Y' lines); each build log prints `git rev-parse HEAD` from inside its worktree at
   the top (NB-1, YPH-1). Each list cmp-equal to r1-dist.sha256's committed non-comment lines: then **Y = N** and no
   list commit, the one change to C1's order (B6: sound; N touches no file under web/ or crates/ and no Cargo or npm
   file, build.mjs reads no ops/ path and no git state). C1's Y0-to-Y bar becomes `git diff --name-only 1ceecc32 N --
   spikes/netcode-proof` = exactly r1.sh, r1sh.test.mjs and R1.md. A difference goes to B6 with both lists, before X.
3. **X'', the new X:** rs.yaml:69 and lg.yaml:52, Y' -> N in place. Bars: name-only exactly the two cloud-init files
   (R1.md's terms link is in already), `--numstat` `1 1` each, hunks printed; `git show X'':` of both holds N; rs.yaml
   under 32 KiB; the counted run; the gate; the kit compare `git diff --name-only X'' origin/main -- CLAUDE.md .claude
   .github` empty and main's tip (A4) (NB-2); pushed by the save; CI green; B6's one head read of N, both builds and
   X'' before the links go to him (NB-2); both raw links fetched (200) and equal to `git show X'':`.
4. **To him:** the two new Cloud config links; his fixed addresses made in `nbg1` (R1.md:64 and :135: they are bound
   to their location; an fsn1 pair, if made, deleted by him); the rest of the Once list unchanged. The nbg1 prices his
   create form shows are recorded: if they differ from the price service's (EUR 0.1386 and 0.0176 an hour,
   stopb-plan.md D4's fourth read, equal to fsn1's), the cap is re-summed before his OK (NB-3).

## Held, not sent: if no location offers CAX21
R1.md has no stand-in for the load machine (Stop B's message, For Mustafa 2) and its rule for refused VMs is "R1 waits"
(:135). Held until his look reports no location offering CAX21 (NB-4; B6's item 5, advisor-stopb-plan.md:1044-1056):
- (a) R1 waits for CAX21 capacity, day by day; R2, R3, R5 and the close move with it.
- (b) LG on an x86 type in RS's location: lg.yaml's aarch64 rustup archive and its pinned sha256 change (:137-138), and
  r1sh.test.mjs:199's `plan: "CAX21"`: a new Y, two builds and a new X; R1.md's LG rows and the cap re-summed at that
  type's price; a new plan with its own check.
- (c) Both servers in another location that offers both types (the pages list HEL1 for both): the nbg1 chain under
  another name, his addresses made there; his home path (-36, the ping) is read farther away, rough readings that
  judge nothing. R1.md:63 names only nbg1, so a third location is his word.
- Lean, conditional: (c) if his look finds both types offered somewhere at the listed prices (labels only); else (a)
  for up to two days, then (b). Final once his look is read.

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
