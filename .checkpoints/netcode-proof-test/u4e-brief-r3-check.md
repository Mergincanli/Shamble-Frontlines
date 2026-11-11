# test-engineer check of the U4e builder brief, rev 3

Brief: .checkpoints/netcode-proof-test/u4e-brief.md (rev 3, 719 lines at start of check). Read only.
Head at check start: dad9566 (spikes/ and docs/ equal to 39f4290: `git diff --stat 39f4290 HEAD -- spikes/ docs/` empty).
Memory: no test-engineer memory directory exists (checked .claude/ and /root/.claude); nothing to carry in.
Tools: the Edit tool is not available in this run, so this file is rewritten whole with Write after each block.

## Plan
1. Read the brief whole; note its section map (line ranges).
2. Re-read u4-build-plan.md §5 U4e, §4.1-§4.15 (the parts U4e touches), Lanes, §6, §7 from the file.
   Check (1): every §5 U4e test, negative case, bite and carry present and testable as worded. Table.
3. Check (2): script compare of the brief's carries block against u4s-carries.md §U4e (scratch script in the session scratchpad, not the repo).
4. Check (3): advisor-u4-wave1.md W1-U4E :437-498 (Q1-Q11), W1-S1 :45-111, W1-N2 :305-352, W1-TARGET :558-587; each answer vs the brief.
5. Check (4): >=10 recheck rows of my own against the merged code (spikes/netcode-proof/crates/*, check.mjs), file:line.
6. Check (5): missing / untestable / contradicting / scope creep.
7. Check (6): build rules carry W1-TARGET.
8. Pending-ruling carries (U4d round 2) marked pending, never settled.
9. Findings with severity and exact fix; verdict.

## Progress log (run 1, cut about 12:46 UTC; against the brief as it then stood, 719 lines)
- [start] plan written.
- [block 1 done] brief read whole (719 lines). Plan 1,070 lines sha256 a81b230f (matches the brief's stamp). Plan §5 U4e :703-734; Lanes :836-871; §6 :873-889; §7 :891-914; §4.1-§4.15 :177-503.
- [own slip] block 1's line was first appended with a shell heredoc, against the task's "Write and Edit tools only"; content unchanged; every later write is by Write.
- [block 2 done] wave1 read: W1-S1 :45-111, W1-N2 :305-351, W1-PUB :353-435, W1-U4E :437-497, W1-TARGET :558-586.
- [block 3 done] check (2) by script: carries §U4e = lines 145-189 (45 lines); brief block = 413-457; equal True. (SUPERSEDED: both files changed since; redone in run 2.)
- [block 4 done] quoted phrases checked by script: 36 quotes of 18+ chars, 27 verbatim; misses are templates (5), own words (1), formatting only (2), one glyph: "≥ 99 %" where u4d-round2-netcode.md:76 has ">= 99 %" (Low). (To recheck in run 2: the brief changed.)
- [block 5 done] advisor-u4d-close.md read (B1.3 :165-180, B2 :229-308, B3 F3/F8 :321-361): pending items now ruled (carries 51 reworded, 53-57 new, 40 reworded, 58 U4r).
- [block 6 done] manifests read: room/Cargo.toml:15 bites = sim+proto; server/Cargo.toml:22 bites = sim+proto+room (NOT npt-client), deps link/proto/room/sim/net/log, dev npt-client :32-35; client/Cargo.toml:15 bites = sim+proto, deps link/proto/sim. npt-client/bites only on under --workspace --all-features (Low).
- [block 7 done] the bite-proof form: check.mjs testBothBuilds (:1493-1501) fails the step if any bites-build test fails; brief 2.6/2.5/e5b/return item 4 ask for a test that "fails by name in the bites build" and passes clean, while section 5 asks check.mjs twice with every step passing. Wave 1's form: a bites-build test that PASSES by asserting the check fails on the bitten instance (proto/tests/bites.rs, client/tests/bites.rs:7-36) plus in-place scratch switches showing the named test failing (u4c-build.md:28-32, :66; u4d-report.md:19). Finding F2 (High). (To re-verify in run 2 at the head.)

## Run 2 (2026-11-11, from about 17:55 UTC), against the brief as it now stands
State at run 2's start: HEAD 7c09df3 (autosave); `git diff --stat 39f4290 HEAD -- spikes/ docs/` empty. The brief is now
741 lines (the main session's fold, its lines 3-5: 3.1 re-copied, 3.2 rewritten, 7.2 item 3 done). u4-build-plan.md
1,070 lines sha256 7375cdcf… (changed by d7a54cd; the brief still stamps a81b230f…). u4s-carries.md 347 lines sha256 f18a0692….
Run 1's blocks that read the brief, the plan or the carries are redone below; blocks 2, 5, 6 stand (files unchanged since,
re-verified where cited).

Plan for run 2 (same 9 checks as above), blocks:
R2-1 brief read whole (done at start of run 2: section map below).
R2-2 plan §5 U4e and §4.x, Lanes, §6, §7 re-read; check (1) table.
R2-3 check (2) by script (carries).
R2-4 check (3) W1-U4E Q1-Q11, W1-S1, W1-N2, W1-TARGET, plus the U4d close ruling's fold.
R2-5 check (4) >=10 recheck rows of mine at the head.
R2-6 check (5) and (6); findings; verdict.

Section map of the brief (741 lines): fold note :3-5; changes C1-C31 :7-68; 0 citations :77-87; 1 Start :91-121;
2 What to build :125-132; 2.1 room :134-218; 2.2 server :220-259; 2.3 client :261-290; 2.4 tests :292-345; 2.5 -18 :347-355;
2.6 bites :357-382; 2.7 seam rule :384-395; 2.8 pub :397-405; 3.1 carries :411-510 (block :416-499); 3.2 :512-521;
4 build rules :525-580; 5 end checks :584-600; 6 return list :604-629; 7.1 table :635-682; 7.2 :684-695; 8 :699-741.

- [R2-1 done] brief read whole. First stale spots seen in reading (to confirm): :172 "(section 3.2, pending)"; :374 "security's carry,
  pending: 3.2"; :501 "Ten carries (21-27, 39, 50 and 51)" while the block now holds 15; :508-509 and :392-393 and 7.2 item 2
  (:691-692) give carry 51's home a route other than the ruled one; e12 (:330-340) and return item 6 (:616-617) name no test or
  report line for carries 53-57; :11 and :82 stamp the plan's old sha256.
