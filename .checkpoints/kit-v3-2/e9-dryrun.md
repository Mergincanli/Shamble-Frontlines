E9 for kit-v3-2: the new paste block works (a dry run), with condition 10's cut-off case. Main session, 2026-11-04.

## Plan
Blocks: 1 runs on the reviewed text (acfc17e); 2 the finding and its fix; 3 runs on the fixed text; 4 the record.
Each run: one general agent (opus, high); its prompt is a toy task plus CHECKPOINTS.md's paste block, filled in
(checked byte for byte against the file's block). Evidence per run: its tool calls, read from its transcript with a
small lister script, and each file's inode, size and mtime before and after (the cut state is set by hand).

## Two toys (a deviation from plan rev 2's E9, which names one: read three named files, note one fact from each)
- Write toy (runs 1-3): block N writes out/bN.txt ("one", "two", "three") and notes "bN written".
- Read toy, the plan's design (runs 4-6 and the fixed-text runs): block N reads src/crateN.txt, notes its fact, and
  writes it to out/factN.txt. The per-block output file is added so "written over, not appended" has a file to show.
- The cut leaves block 2's file half written ("tw", "Crate 2 ho") and deletes block 3's, so "written over" shows in
  the content, not only in the mtime.
- The toys ran in the scratchpad, outside the repository (the plan said .checkpoints/kit-v3-2/dryrun-<part>.md);
  this file is their record.

## Block 1: the reviewed text (acfc17e), runs 1-6, 21:42-21:51 UTC
| Run | Toy, case | Calls | What the tool calls show | Result |
|---|---|---|---|---|
| 1 | write, fresh | 3 | notes: task line, plan, a finding and a done line per block, DONE; all written in ONE shell call, in that order | PASS (the toy is too small to show saving across calls) |
| 2 | write, cut after block 2's finding | 3 | went on after block 1; b2 "tw" -> "two\n" (written over); b3 done; b1 not rewritten (inode 803332, mtime 21:42:26.913 kept); its last call read b1 (`od -c` of all three) | PARTIAL: block 1's file read |
| 3 | write, DONE | 2 | `ls -la` and the notes only; no file read, nothing written | PASS |
| 4 | read, fresh | 6 | plan first; then one call per block: source read, finding line, out file, done line (out mtimes 21:48:49, :53, :57) | PASS: shows saving as it goes |
| 5 | read, cut after block 2's finding | 4 | fact2 written over, fact3 done, fact1 not rewritten (inode 803342 kept); but call 2 read all three sources, src/crate1.txt included, and out/fact1.txt, "to check" | FAIL: condition 10 |
| 6 | read, DONE | 3 | no source read; read out/fact1-3 "to check they match the notes" | PARTIAL: letter met, intent not |

## Block 2: finding E9-1 and the fix
- E9-1: a restarted agent re-checks the blocks its notes mark done (runs 2, 5, 6). The text says where to go on, not
  that done blocks are trusted. Condition 10's "no Read of block 1's file" fails in run 5.
- Fix B (CHECKPOINTS.md §4, three places): the paste block gains "Trust the blocks they mark done: never reread,
  recheck or redo them (a later block may still read a file it needs)."; "Look first" gains the same rule for both
  resume cases; the rule bullet "a restart continues after the last complete block" says a restart trusts complete
  blocks, and why. The carve-out: a later block may still read a file it needs (a builder's block 2 may edit the file
  block 1 edited).
- Fix C, after B's runs: the paste block's last sentence becomes "End with DONE, then give your answer from your
  notes."; the format bullet adds "the answer is then given from the notes". Why: B's runs still printed block 1's
  output while showing results at the end.

## Block 3: the fixed texts, 21:54-22:01 UTC (three agents at a time)
| Run | Text | Toy, case | Calls | What the tool calls show | Result |
|---|---|---|---|---|---|
| rc1 | B | read, cut | 5 | read only src/crate2, out/fact2, src/crate3; fact2 over, fact3 done | PASS |
| rc2 | B | read, cut | 4 | no block 1 file read during the work; last call `head out/*.txt` printed fact1 | PARTIAL: output shown |
| rc3 | B | read, cut | 4 | no block 1 file read during the work; last call `cat` of fact1-3 | PARTIAL: output shown |
| rc4 | C | read, cut | 4 | no block 1 file read during the work; last call `cat out/*.txt` printed fact1 | PARTIAL: output shown |
| rc5 | C | read, cut | 4 | read only src/crate2 and src/crate3; showed only fact2 and fact3 | PASS |
| rc6 | C | read, cut | 4 | read only src/crate2, out/fact2, src/crate3; showed only fact2 and fact3 | PASS |
| rfresh | C | read, fresh | 6 | plan first; one call per block (out mtimes 21:58:03, :06, :10); showed its own outputs at the end | PASS |
| rdone | C | read, DONE | 2 | `ls -laR` and the notes only; no file read, nothing written | PASS |
| wcut | C | write, cut | 3 | b2 "tw" -> "two\n", b3 done, b1 kept (inode 803474); its last call printed b1-b3 | PARTIAL: output shown |
- Fix D, after C's runs: the last sentence becomes "End with DONE, then give your answer from your notes alone,
  without opening any file again."; the format bullet says "from the notes alone".

| Run | Text | Toy, case | Calls | What the tool calls show | Result |
|---|---|---|---|---|---|
| rc7 | D | read, cut | 5 | read only src/crate2, out/fact2, src/crate3; nothing shown at the end | PASS |
| rc8 | D | read, cut | 5 | read only src/crate2 and src/crate3; nothing shown at the end | PASS |
| wcut2 | D | write, cut | 3 | read only its notes; b2 written over, b3 done, b1 kept (inode 803533) | PASS |
| rdone2 | D | read, DONE | 2 | `ls -la` and the notes only; no file changed | PASS |
| rfresh2 | D | read, fresh | 6 | plan first; one call per block (out mtimes 22:01:15, :18, :21); nothing shown at the end | PASS |
In every run on every text: block 1 was never rewritten (same inode and mtime), block 2's half file was written over
(exact content, nothing appended), block 3 was done, and the notes end with DONE: the restart point was always right.
All 20 prompts were checked byte for byte against their prompt files.

## Block 4: the record
| Text | Restarts after a cut that opened none of block 1's files | DONE starts that answered from the notes only | Fresh starts: plan first, saved as they went |
|---|---|---|---|
| A (acfc17e, reviewed) | 0 of 2 (run 2 read b1 at the end; run 5 read src/crate1 and out/fact1 first) | 1 of 2 (run 6 read the outputs) | 2 of 2 (run 1 in one call, in order) |
| B (+ trust the done blocks) | 1 of 3 (rc2, rc3 printed fact1 at the end) | not run | not run |
| C (+ answer from the notes) | 2 of 4 (rc4, wcut printed block 1's output at the end) | 1 of 1 | 1 of 1 |
| D (+ alone, no file opened again; final) | 3 of 3 | 1 of 1 | 1 of 1 |
- Condition 10 on the plan's read design ("no Read of block 1's file", the named source): text A 0 of 1 (run 5);
  texts B-D 8 of 8 (rc1-rc8).
- Not verified: real (long) tasks, only toys; the "first line names another task" case (no run moved notes to
  old/; in wcut and wcut2 the first line named the same task with a shorter path, and both kept the file, rightly);
  workflow and worktree agents (only Agent-tool subagents ran); each case ran 1-3 times on the final text, and agents
  are random, so these are small samples.
- Cost: 20 runs; the 19 measured took 63.6-70.0 thousand tokens each (most of it the agent's fixed context) and
  8.6-26.0 s each.
DONE
