# U4e builder brief rev 3: netcode-architect notes

Task: rev 3 of the U4e builder brief -> .checkpoints/netcode-proof-test/u4e-brief.md (from rev 2 at 0dccbf07:.checkpoints/npt-u4e-brief/u4e-brief-draft.md).
No local agent memory found (memory: local; no file). Fresh run (no earlier notes file, no u4e-brief.md).
Tools: no Edit tool in this run, so each notes update rewrites this file with Write. Two notes updates (after B1 and B3) slipped through the shell (a heredoc, then a python replace) before I caught it; the content is the same, and every later write is the Write tool's. The brief itself was written twice with Write (the second pass fixed eight cites; no shell write touched it).

## Plan (blocks)
B1. Read rev 2 draft + test-engineer check (git show 0dccbf07).
B2. Read rulings: advisor-u4-wave1.md W1-U4E (:437-498), W1-S1 (:45-111), W1-N2 (:305-352), W1-TARGET (:558-587); advisor-u4.md where rev 2 cites it.
B3. Read u4-build-plan.md §4.1-§4.15, §5 U4e, Lanes, §6, §7 (from file).
B4. Read u4s-carries.md §U4e (entries 21-27, 39, 49?, 50, 51); copy word for word (script check).
B5. Read catalogue docs/design/netcode-proof-messages.md own-state row; .claude/rules/netcode.md.
B6. Read the code at the branch head: crates/{sim,proto,server,room,client}, check.mjs; u4d-report.md.
B7. Read u4d-round2-netcode.md N2-L1, N2-L3(c); u4d-round2-security.md "Carries for U4e" (pending advisor-u4d-close.md).
B8. Walk the recheck table (rev 2 §7), row by row with file:line.
B9. Write rev 3 (change list; applied answers; build rules; carries; recheck table; day-run items; open questions).
B10. Script check carries word for word; final pass; handback.

## Progress
- B1 done: rev 2 (577 lines; sections 1-8; recheck rows R1-R28; questions Q1-Q11, Q7 and Q9 answered) and test-engineer's check read.
- B2 done: wave1 W1-S1 :45-111, W1-U4D :113-303, W1-N2 :305-351, W1-PUB :353-435, W1-U4E :437-497, W1-TARGET :558-586 (details in the brief's cites).
- B3 done: plan now 1,070 lines, sha256 a81b230f…; every line moved since df8975b (new lines used throughout the brief).
- B4 done: carries §U4e = :145-189 (21-27, 39, 50, 51; 49 is §U4r's; 40 is §U4r's with W1-U4E Q5's sentence).
- B5 done: catalogue own-state row :136 as built; header row :135; .claude/rules/netcode.md.
- B6 done: head 56214fa, spikes/ and docs/ identical to 39f4290. Code facts recorded in the brief's table (R1-R38).
- B7 done: N2-L1 :78, N2-L3 (c) :81-82, N2-M2 :75-77 (u4d-report.md:44-46), security's carries :76-88; advisor-u4d-close.md held only its plan: all marked pending.
- B8 done: 28 rows walked: changed R4, R6, R11, R15, R19, R20, R27, R28 (8); 20 hold; R29-R38 added (10). R24's diff run (git diff df8975b 39f4290 seam.rs grid.rs: 312+/31-), R11's git grep run at 39f4290.
- B9 done: u4e-brief.md written (719 lines): 31 changes (C1-C31); sections 0-8; Q1-Q11 ruled table; open: U4E-Q12 (-22/unbounded's planting unit), U4E-Q13 (owner-to-all's id).
- B10 done: carries block checked by script (python -I read of both files): lines 413-457 of the brief equal carries :145-189, EQUAL. Cite fixes verified (u4j-brief :46-48, check.mjs :1646/:1649, :637-644, plan.md :568-612, snapshot.rs :491-499 / :647-655). `<base>` kept as a placeholder (18 uses).
- DONE.
