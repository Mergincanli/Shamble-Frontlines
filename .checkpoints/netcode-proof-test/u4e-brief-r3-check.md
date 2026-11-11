# test-engineer check of the U4e builder brief, rev 3

Brief: .checkpoints/netcode-proof-test/u4e-brief.md (rev 3, 719 lines at start of check). Read only.
Head at check start: dad9566 (spikes/ and docs/ to be confirmed equal to 39f4290).
Memory: no test-engineer memory directory exists (checked .claude/ and /root/.claude); nothing to carry in.

## Plan
1. Read the brief whole; note its section map (line ranges).
2. Re-read u4-build-plan.md §5 U4e, §4.1-§4.15 (the parts U4e touches), Lanes, §6, §7 from the file.
   Check (1): every §5 U4e test, negative case, bite and carry present and testable as worded. Table.
3. Check (2): script compare of the brief's carries block against u4s-carries.md §U4e (byte-for-byte; scratch script in this folder).
4. Check (3): advisor-u4-wave1.md W1-U4E :437-498 (Q1-Q11), W1-S1 :45-111, W1-N2 :305-352, W1-TARGET :558-587; each answer vs the brief.
5. Check (4): >=10 recheck rows of my own against the merged code (spikes/netcode-proof/crates/*, check.mjs), file:line.
6. Check (5): missing / untestable / contradicting / scope creep.
7. Check (6): build rules carry W1-TARGET.
8. Pending-ruling carries (U4d round 2) marked pending, never settled.
9. Findings with severity and exact fix; verdict.

## Progress log
- [start] plan written.
- [block 1 done] brief read whole (719 lines). Head dad9566; `git diff --stat 39f4290 HEAD -- spikes/ docs/` empty; plan 1,070 lines sha256 a81b230f (matches the brief's stamp). Plan §5 U4e read at :703-734; Lanes :836-871; §6 :873-889; §7 :891-914. Next: plan §4.1-§4.15.
