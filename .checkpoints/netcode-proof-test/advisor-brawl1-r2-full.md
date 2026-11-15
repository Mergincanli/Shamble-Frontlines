# Advisor: the ruling on BRAWL-1's review round 2 (workflow wf_d32decd1-87b), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-r2's StructuredOutput result). The round: brawl1-review-r2-full.md. Its notes: advisor-brawl1-r2.md. Nothing below is the main session's.

**Snapshot:** wip/brawl1.patch, sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f. I hashed it with sha256sum. It equals the copy committed at 888ecbb and the copy at the branch head, and no later commit touches it. The read copy is /home/user/Shamble-Frontlines/.claude/worktrees/brawl1: HEAD 352a245, the same 10 files modified, nothing untracked. Its `git diff HEAD | sha256sum` gives c2561b6c. By numstat against 352a245: 1,139 lines added and 127 deleted, 1,266 changed. Round 1's copy, /home/user/Shamble-Frontlines/.gate/brawl1-snap1, still gives ee2973aa. I changed nothing in either copy.

## Summary

Round 2 is closed and the merge step can go ahead. Both seats (test-engineer and rust-reviewer) reviewed snapshot c2561b6c and approved with findings. Every round-1 finding is fixed or deferred as ruled. They found 4 Lows, and no Critical, High or Medium, so round 3 does not open.

I checked each Low in the code and all four hold:
- TE-B1-R2-1 is real: a leash that updates its state from a dead soldier's position still passes the unit test, because the dead input at (1450, 0) lies past the outer radius. I re-ran the walk sequence (phase 7: command 10 walks [-117,-49], command 12 walks [-89,-89]).
- TE-B1-R2-2: the runner's "untracked" field hashes the whole porcelain list (modified files too). It reads 17cf51d7 on a tree with nothing untracked.
- RR2-B1-1: (-32,768, -32,768) gives exactly 2^31, so "below" should be "at most". This was my own ruled wording.
- RR2-B1-2: the report says Server::new's caller is at :390; grep finds it at :389.

Three are follow-ups and one is report words. None changes the bytes that merge.

Copy 2's deviations all hold in the code, and I accept them, LOGS0-RECORD as option (a). Two of them correct my own errors: the ruled `--logs 0` line could not parse, and `Some(&dir)` would not compile.

What I checked myself:
- the interdiff, read whole: the library changes are comments plus the one `fn new`;
- the test counts: +3 tests and none removed (bots 3 to 4, endpoint.rs 14 to 15, logs.rs 2 to 3);
- every log's header and exit codes, and the four plant failure lines (bots :700 in both builds and :750, logs :924, endpoint :446 and :483);
- the 13 printed lines of the sim-only test, which I diffed myself after the stated normalisation: identical to round 1;
- the hashes of det-u4's regenerated files (1ec25de7 and 6abf7643) and of both manifests (a2faf3ea and 5ccf57f0).

My own run is /home/user/Shamble-Frontlines/.gate/u4r/brawl1-advisor-r2/adv-r2-tests.log. Its header shows c2561b6c and every command exited 0: fmt clean, bots brawl1 tests 3 passed and 1 ignored in both builds, client brawl1 tests 2 passed, and the lab figure test passed in both builds.

Both resume-part errors are real and had no effect. The "section 4" slip started in my own round-1 ruling.

M1-M7 go ahead on c2561b6c, with six amendments A1-A6 (see the merge step). Nothing for Mustafa.

One slip of mine: I appended three progress lines to my notes file with a shell `cat >>` redirect, which my limits forbid. Only that file was touched, its content is what I would have written with Edit, and the slip is recorded in the file. Every later write used Edit.

**Round 3:** does not open

## Each finding

### TE-B1-R2-1 (Low; follow-up)

- **Home:** A STATUS follow-up named in the report. It rides the next lane that edits npt-bots' lib.rs, together with RR2-B1-1, and lands at U6's bots'-view item at the latest, beside TE-B1-8's glue test. The fix, as the seat wrote it: at bots lib.rs:404 the dead input `give(Some(([1_450, 0, 150], true)))` becomes `give(Some(([0, 0, 150], true)))`, with both messages unchanged. It does not ride the merge: M1 needs the merged bytes to equal c2561b6c.
- **Why:** Confirmed. The dead input at (1450, 0) is past the 1,400 cm outer radius, so a leash that recomputes `steering` from a dead soldier's position keeps steering true, and the check at command 12 passes. My own splitmix64 gives phase 7: command 10's walk is [-117,-49] and command 12's is [-89,-89]. With the dead soldier at (0, 0), that leash sets steering false and returns the walk [-89,-89] instead of the steer [-127,0] at command 12, so the check fails. 'Dead: the walk' still holds for the real code. The gap is test-only, the mutant is close to the real code (respawns land about 10 m out), and the budget makes a round-2 Low a follow-up.

### TE-B1-R2-2 (Low; follow-up)

- **Home:** A main-session runner follow-up, outside the change set. It comes after M7's last re-run set (so every BRAWL-1 log has one header form) and before U4g's first run. Form, in both u4r-run.sh:67 and main-run.sh:42: `untracked $(git ls-files -z --others --exclude-standard -- . | xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)`, saved as a .diff beside each runner with `bash -n` silent (INF2-4's form). Until then the report reads the field as 'the porcelain status list's hash'.
- **Why:** Confirmed. The line hashes `git status --porcelain --untracked-files=all -- .`, so it covers modified files too. In the builder's copy, with 10 modified files and nothing untracked, it reads 17cf51d7. The field is my ruled FP-0 form. M3's expectation is unaffected: in the main folder's spikes/netcode-proof both values read e3b0 today, and on a clean committed tree e3b0 there means no modified and no untracked file. I prefer the form above to the seat's `ls-files | sha256sum`: it also hashes the untracked files' contents (the seat noted no hash does today). I ran it read-only: e3b0 in both folders.

### RR2-B1-1 (Low; follow-up)

- **Home:** A STATUS follow-up named in the report. It rides with TE-B1-R2-1 (same file): at bots lib.rs:132, 'so x² + y² stays below 2^31' becomes 'so x² + y² is at most 2^31'. It does not ride the merge.
- **Why:** Confirmed. POSITION_XY's min is -32,768 (sim grid.rs:106-111), and quant.rs:212 accepts k equal to the min, so (-32,768, -32,768) gives exactly 2^31. The words are mine (FP-5 (c)). Copy 2 wrote them as ruled and flagged them, which was right. The arithmetic is i64, so nothing overflows. Doc only.

### RR2-B1-2 (Low; recorded)

- **Home:** The report fold (BRAWL-1's section of U4r's report): in FP-6 (a), 'Its only caller is :390.' becomes 'Its only caller is :389.' This is .checkpoints/ text, outside M1's spikes/ comparison.
- **Why:** Confirmed. In the read copy, `grep -n Server::new` prints only scenario.rs:389, with `fn new` at :396. The rewrap of Server::build's doc removed one line above it. Report words only.

### ADV-R1-1 (Low; recorded)

- **Home:** Report words, beside ADV-BP-1 to ADV-BP-4 (my corrections, in my words).
- **Why:** My error. Two lines in FP-3 could not run as I wrote them. In step 7, `settings(&dir, "--profile P0 --seed 1 --logs 0")` cannot parse, because parse refuses `--logs 0` without `--record` (scenario.rs:1485-1486). In step 3, `Some(&dir)` is an Option<&Scratch>, which does not coerce to Logs::open's Option<&Path> (logs.rs:197-202; Scratch derefs to PathBuf, scenario.rs:1712-1719). I did not check either against the code. Copy 2's `--record <scratch>/brawl1.rec` and `Some(dir.as_path())` are the right repairs.

### ADV-R1-2 (Low; recorded)

- **Home:** Report words, beside ADV-R1-1.
- **Why:** My error. Step 1 of my round-1 ruling (advisor-brawl1-r1-full.md:16) named 'the rules of brawl1-brief.md section 4'. The brief's section 4 is 'What you hand over' (:216). The rules are in section 1 (runner, -x, rule 6's disk floors), section 2 (limits) and section 5 (stops). It did no harm: step 1 listed the rules themselves, and copy 2 followed that list.

## Copy 2's deviations

1. **What:** LOGS0-RECORD: the figure test's no-log case adds `--record <scratch>/brawl1.rec` to the ruled `--profile P0 --seed 1 --logs 0` (logs.rs, near the end of the new test).
   - **Ruling:** Accepted, option (a). The answer to copy 2's question is (a): keep it as built.
   - **Why:** Checked in the code. parse refuses `--logs 0` without `--record` (scenario.rs:1485-1486; fp-check-1 failed on exactly that). The path is only stored (:1456, :1592), and only `run` reads it (:906 recording_on, :1205 writer). summary_line (:1612-1691) reads `s.logs` and never `s.record`. The test passes neither --every-step nor --lead, so :1494's refusal does not apply. The added flag therefore cannot change the branch under test, and it is the smallest line that gives a no-log Settings. The ruled line was my error (ADV-R1-1).
2. **What:** soldiers()'s ruled sentence names 'client lib.rs:250-251' instead of '247-248'.
   - **Ruling:** Accepted.
   - **Why:** In the read copy, client lib.rs:250-251 hold SOLDIER_FLAGS and SOLDIER_DEAD. FP-4 (d)'s added doc lines, in the same pass, moved them there. My numbers were the pre-edit lines, so keeping them would have left the doc wrong.
3. **What:** Logs::open gets `Some(dir.as_path())` instead of the ruled `Some(&dir)`.
   - **Ruling:** Accepted, and required.
   - **Why:** Logs::open takes `Option<&std::path::Path>` (logs.rs:197-202). `&dir` is `&Scratch`, which derefs to PathBuf (scenario.rs:1712-1719), and deref coercion does not reach inside an Option, so the ruled form would not compile. It is the same path. My error (ADV-R1-1).
4. **What:** 'x² + y² stays below 2^31' written as ruled, though it is off at (-32,768, -32,768).
   - **Ruling:** Accepted as built: writing the ruled words and flagging them was right.
   - **Why:** The words are mine. Their correction is RR2-B1-1's follow-up. Nothing overflows (i64).
5. **What:** Small additions beyond the ruling: doc clauses in test-module docs (the unit test, printed(), Record, the short copy, the client read test); the logs.rs //! paragraph rewrapped from line 4; a two-line comment above the RR-B1-4 cases; two `out.hashes.len() == 1` 'filed' asserts in the wrap test.
   - **Ruling:** Accepted.
   - **Why:** I read the interdiff from round 1's copy to round 2's whole. The rewrap keeps the old words and adds only EA-B1-4's ruled sentence. The comment's arithmetic is true (143,165,566 x 30 + 300 = 4,294,967,280). The asserts only strengthen the test. All of it is test or doc. Outside the test modules, the only changed lines are comments plus `fn new` (scenario.rs:396), as both seats found and I confirmed.
6. **What:** fp-check-1 failed (fmt hunks; the figure test's parse panic). The code was fixed by Edit, and fp-check-2 then passed.
   - **Ruling:** Accepted. It is not a retry.
   - **Why:** fp-check-1 ran cargo commands, not check.mjs, on a different tree (header diff 2a62ecdc, not c2561b6c). The failure was the new test's own code, so the run was a pre-fix check of new code, not a re-run of a result into a pass. No check.mjs step failed anywhere: final-fp-check.log passed all 11 steps at c2561b6c.

## The main session's resume-part errors

Both are real, and neither had an effect.

(a) Line count. The resume part (brawl1-copy2-prompt.md:17) gave advisor-brawl1-r1-full.md as '249 lines'. The file has 460 lines: wc -l and my Read agree. The sha256 it gave, 86bec63b..., is right. Copy 2 treated the hash as the file's identity, noted the mismatch (brawl1-build.md:401-403) and went on correctly. The report records this as a main-session slip in copy 2's resume part, with no effect, and the lesson: take a file's line count from `wc -l` on the file, or leave it out, since the sha256 is the identity.

(b) "Section 4". The resume part (prompt :16) repeated 'the brief's section 4 (its rules hold throughout: the ruling's step 1)'. The words began in my own ruling: step 1 says 'The rules of brawl1-brief.md section 4' (advisor-brawl1-r1-full.md:16). The brief's section 4 is 'What you hand over' (:216). The rules are in section 1 (runner, -x, rule 6's disk floors, :32-52), section 2 (limits, :54-67) and section 5 (stops, :230-241). The same resume part has a second instance that copy 2 did not report: prompt :19 'Keep section 4 rule 6's floors', but rule 6 is section 1, item 5. No effect: step 1 listed the rules themselves, copy 2 read the brief whole (brawl1-build.md:407-411), and it followed every listed rule. The report records it as my error, ADV-R1-2 (beside ADV-BP-1 to ADV-BP-4), repeated twice by the main session in the resume part. Future resume parts cite brief sections 1, 2 and 5 for the rules.

## The merge step

GO. M1-M7 of my round-1 ruling go ahead on snapshot c2561b6c with six amendments. Round 2 is closed, nothing triggers a push, and the base holds: spikes/netcode-proof's tree is abad0635 at both 352a245 and the branch head, and every commit since 352a245 touches only .checkpoints/.

A1. Nothing from round 2 rides. The merged spikes/ bytes are exactly c2561b6c's (M1's cmp and the tree fingerprint). TE-B1-R2-1 and RR2-B1-1 wait for their home. RR2-B1-2 is report text under .checkpoints/.

A2. M3 runs on the committed merge. Its header's HEAD is the merge commit, or a later commit whose changes since touch only .checkpoints/. The header line must read `diff e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855; untracked e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Read the 'untracked' field as the hash of the porcelain status list (TE-B1-R2-2): e3b0 there means no modified and no untracked file under spikes/netcode-proof, which is stronger than its name. Any other value voids it as the counted run, and it comes to me whole with no re-run until I rule. A diff of c2561b6c would mean M3 ran on the uncommitted patch. I checked today: in the main folder's spikes/netcode-proof both values read e3b0. det-u4 and the regeneration write under the root .gate/, which is gitignored.

A3. M3's figures, stated:
- check.mjs passes all 11 steps, with 538 and 616 tests, 0 failed, 0 skips, and self 77 of 77.
- `uptime` comes first, then det-u4 PASS, reproducing SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e, with a non-cargo sum under 240 s.
- The precheck regeneration, into new folders under .gate/, prints 'regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's'.
- The full sim-only test, by name in release, prints nine lines byte-identical to final-fp-regen-tests.log's nine full-test lines, dead figures included.
- Both manifests stay a2faf3ea... and 5ccf57f0....
Any pin mismatch: stop, no retry, and it comes to me whole.

A4. Disk inside the call (rule 6 applies to every build in a call; the runner checks 4 GiB only at the call's start). Add `df -B1 /home/user` to M3's sequence after check.mjs and again after det-u4, so the free space before each later release build is on record. A reading under 4 GiB before a release step is a floor finding: reported whole, never cleaned around, and it comes to me before M4. The expected numbers:
- The shared target holds 6,360 MiB now (debug 5,925, release 250, wasm32 172, doc 15).
- After M2's (B) and (C), about 12,955 MiB should be free.
- The builder's full set occupies about 6.4 GiB; my round-1 estimate was 8.2 to 9 GiB. So expect about 3.9 to 6.6 GiB free at M3's end.
- The shared target's old `tmp` folder (present since Nov 9 and left by the 2026-11-14 cleans) is not a stop.

A5. M5: the merge commit's message carries no `[skip ci]`, and CI is read on that commit. Every commit between it and the head M6 reviews touches only .checkpoints/ (git diff --name-only), as at U4r's merge.

A6. The runners stay as they are (u4r-run.sh 2f754d12, main-run.sh f0b7a3d0) until after M7's last re-run set, so all BRAWL-1 logs have one header form. TE-B1-R2-2's fix comes after that.

M2, M4, M6 and M7 stand as written. M2: the part-cleans (B) and (C) only, as clean-shared-B.log and clean-shared-C.log ran them, with no lane and no agent running. M7 repeats (B) and (C), with df before and after, before the two 10-minute evidence runs and the re-run sets, each set's results committed before the next.

## The report words

For BRAWL-1's section of U4r's report (where its round 1 sits), recorded word for word by script:

"Review round 2 (workflow wf_19a5485a-e64; test-engineer and rust-reviewer at effort xhigh, in one batch) reviewed snapshot c2561b6c (wip/brawl1.patch, 1,266 changed lines; packet e50a22f7). Both seats: APPROVE WITH FINDINGS. Every round-1 finding is fixed as ruled or deferred as ruled: TE-B1-1 to TE-B1-8, RR-B1-1 to RR-B1-7, and EA-B1-2 checked by rust-reviewer.

What the seats checked:
- They rebuilt each plant's planted sha256 without writing anything: P1 77be9b00, P2 b902a7ee, P3 7a77c71a, P4 6aeeec0a.
- Outside the test modules, the library diff is comments plus one visibility change (scenario.rs:396).
- Their own runs (r2-tests.log, lint-tests.log; header c2561b6c): every command exited 0. fmt and clippy are clean in both builds.

Round 2 found four Lows and no Critical, High or Medium, so round 3 did not open. Two rounds were used; the second was the last.
- TE-B1-R2-1: the kept-state check cannot catch a leash that updates its state from a dead soldier's position. Follow-up, test only: move the dead input from (1450, 0) to (0, 0) at bots lib.rs:404.
- RR2-B1-1: toward_centre's doc should say 'x² + y² is at most 2^31', not 'stays below' (bots lib.rs:132; the words were the advisor's). Follow-up, riding with TE-B1-R2-1. Home for both: the next lane that edits npt-bots' lib.rs, U6's bots'-view item at the latest.
- TE-B1-R2-2: the runners' 'untracked' field hashes the whole porcelain status list, modified files included. Follow-up, a runner edit outside the change set, after BRAWL-1's last re-run set and before U4g's first run, in the form `untracked $(git ls-files -z --others --exclude-standard -- . | xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)`, with a .diff and `bash -n`. Until then, every BRAWL-1 log's 'untracked' value is the porcelain list's hash: e3b0 means no modified and no untracked file under spikes/netcode-proof, and 17cf51d7 is the builder's ten modified files.
- RR2-B1-2: Server::new's only caller is scenario.rs:389, not :390. Corrected in this report.

The advisor's round-2 ruling (advisor-brawl1-r2.md):
- It checked every finding and every deviation in the code.
- It read the interdiff whole and counted +3 tests with none removed.
- It diffed the sim-only test's 13 printed lines against round 1's itself, after the stated normalisation: identical.
- It hashed det-u4's regenerated recordings (1ec25de7, 6abf7643) and both manifests (a2faf3ea, 5ccf57f0).
- Its own run (.gate/u4r/brawl1-advisor-r2/adv-r2-tests.log, header c2561b6c): fmt clean, and the BRAWL-1 tests pass in both builds.

Copy 2's deviations are all accepted:
- LOGS0-RECORD, option (a): the no-log case keeps `--record <scratch>/brawl1.rec`, because parse refuses `--logs 0` alone and summary_line reads only `s.logs`.
- soldiers()'s doc names client lib.rs:250-251.
- Logs::open gets `Some(dir.as_path())`.
- The extra doc and test lines.

The advisor's corrections, in its words, beside ADV-BP-1 to ADV-BP-4:
- ADV-R1-1: 'Two lines in FP-3 could not run as I wrote them: `--logs 0` without `--record` cannot parse, and `Some(&dir)` would not compile; copy 2's repairs are right.'
- ADV-R1-2: 'Step 1 named the brief's section 4 for the rules; they are in sections 1, 2 and 5.'

Process slips, none with an effect:
- The main session's resume part for copy 2 gave the round-1 ruling as 249 lines; it has 460, and its sha256 was right.
- The resume part repeated ADV-R1-2's 'section 4' twice: at :16, and at :19 as 'section 4 rule 6'.
- The advisor appended three lines to its own notes file with a shell redirect, against its write rule. Content unaffected.

The merge step goes ahead on c2561b6c, M1 to M7 with amendments A1 to A6 (the ruling's merge step). The counted run must show the empty-input sha256 e3b0c442... for both diff and untracked, and must reproduce all three pins."

Then the merge record (M1 to M7, with each figure) follows once the merge is done.

## For Mustafa

BRAWL-1, the fix that keeps the test bots out of buildings, passed its second and last review round. Both reviewers approved. They found four small items: one test check that could be stricter, a log-header label that reads wrongly, one comment that is off at a single corner point, and one line number in the report. None changes what the code does, and none changes the code that goes in. Each goes on the follow-up list or into the report.

I checked everything myself: the tests pass, the three pinned recordings come out byte for byte the same, and the bot figures equal round 1's. Two errors in my own earlier instructions came to light and are recorded. The builder fixed both correctly.

The change now goes onto the test branch through the agreed steps: build caches cleared by the approved method, one counted run that must reproduce the three pins, the gate, CI, and my review of the pushed version.

Decisions needed: none. The kept-logs decision (KEPT-LOGS) is still open with you, as raised at the plan review; nothing here changes it.

For Mustafa: nothing. This is test-branch spike code, not a merge to main. Nothing a player sees, hears or feels changes.
