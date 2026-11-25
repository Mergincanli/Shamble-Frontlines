# STOP-4A: change-set 4a is at its stop line. Asked of Mustafa on 2026-11-25 (21:00 Amsterdam)

## Where 4a stands
- **Written so far: 3,030 lines** (1,523 code, 1,507 tests), counted by git against 4a's base (7670c7b), the
  recording's own data files left out.
- The three sizes of the plan, each named for what it is: **1,365** is the designs' own count (bottom-up, not
  multiplied); **2,450** is the planned size, which already holds the 1.8; **3,050** is the stop line (planned times
  1.25). So 4a is 2.2 times the designs' count, 1.24 times its planned size, and 20 lines under its stop line.
- **What is built and green on this machine:** the one visitor of the state; the shared hash on the players' file
  with its pin; the recording's format with its reader and writer; the replay runner; the comparer and the
  field-by-field difference; the scripts and the two tools (`make_recordings` with `--check`, `replay`); the first
  recording (1,800 steps) replayed natively in one process, in two more processes (with a 36,000-step scripted
  match), as two matches stepped in turn, in WebAssembly in Node on the players' file and on the test module in
  both profiles, and in Chromium in a Worker on the players' file: every one of the 1,800 shared hashes equal on
  every target.
- **What is left adds no product line of its own:** rust-reviewer's report on the hash's function (running); then
  the recording's files are committed; the three planted divergences (a script in the notes); CI on GitHub's
  runner; the reviewers (two rounds at most), the verifier, the docs pass, the report. **What the reviewers ask
  for will add lines**, and with 20 lines of room that crosses the stop line.

## Why it is larger than planned
- The designs' count for the code was 655; it is 1,523. The recording's format alone is 434 lines against 140
  planned (every refusal by name, the hashes file, the manifest's lines); the folder check with its "first step
  that differs" is 139 lines the design did not list apart; our own SHA-256 is 95 lines the design did not have
  (the hashes file names its recording by that digest and the manifest lists each file with it; the workspace has
  no crate for it, and 4a changes no manifest).
- The tests are 1,507 against 710: the Rust formatter spreads asserts over many lines (the format's unit tests are
  324 lines), and the TypeScript reader got its own unit tests (157 lines), because it is a second implementation
  of the format.

## The ladder, as the plan fixed it
- **Rung 1 (at 2,450 lines): 3B-HOSTILE-ALLOC moves to the link simulator's change-set.** It is not written, so it
  moves; this is the telling before it moves. (It is the counted "no allocation" for hostile bytes in the room and
  the endpoint, a follow-up of change-set 3b; the link simulator's change-set is its other named home.)
- **Rung 2 (at 2,750 lines): the 36,000-step run and the two matches stepped in turn move to 4c.** Both are already
  written and green (about 80 lines in all). Moving them now means deleting passing proofs, and 4a stays above
  2,750 either way.

## The decision: STOP-4A
What happens now that 4a is at its stop line? It matters because your rule is that a stop comes back to you at
its stop line, whatever is left.
- **(a) Finish 4a as it stands, with a new stop line of 3,400.** The 36,000-step run and the two matches stepped
  in turn stay (rung 2 is not applied to work that is done); 3B-HOSTILE-ALLOC moves as rung 1 says. The 350 lines
  are room for what two review rounds ask; nothing else is added. If the reviews need more, I come back again.
- **(b) Apply rung 2 to the letter, and a new stop line of 3,300.** The two-matches test and the 36,000-step run
  are taken out of 4a and built again in 4c. 4a shrinks by about 80 lines; two green proofs leave it for a while.
- **(c) Stop here and cut.** You name what leaves 4a. The never-moved list stays: the runner, the comparer, the
  first recording on its four runs, the one new number with its pin, one planted divergence.
- **Recommended: (a).** Every line left is a proof or a fix a reviewer asks for; deleting green tests to meet a
  number makes nothing safer.

## What I do while you decide
Only work that adds no product line: rust-reviewer's hash report, the planted divergences (a notes script), the
commit of the recording's files once the report is in, CI on GitHub. No reviewer round starts before your answer.

## What this says about the rest of Walk
The plan's ratio of 1.8 was short for this part (2.2 so far, before reviews). 4b, 4c and 4d were estimated the same
way. I change nothing in the plan now; 4a's report will give the measured ratio, and 4b's re-read by the advisor
agent uses it.
