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

## Progress log
- [start] plan written.
- [block 1 done] brief read whole (719 lines). Plan 1,070 lines sha256 a81b230f (matches the brief's stamp). Plan §5 U4e :703-734; Lanes :836-871; §6 :873-889; §7 :891-914; §4.1-§4.15 :177-503.
- [own slip] block 1's line was first appended with a shell heredoc, against the task's "Write and Edit tools only"; content unchanged; every later write is by Write.
- [block 2 done] wave1 read: W1-S1 :45-111, W1-N2 :305-351, W1-PUB :353-435, W1-U4E :437-497, W1-TARGET :558-586.
- [block 3 done] check (2) by script (scratchpad carry_cmp.py, python3 -I): carries §U4e = lines 145-189 (45 lines); brief block = 413-457 (45 lines); equal True; sha256 both b8399fbb…e89b1c; 0 diff lines; entries 21-27, 39, 50, 51 in both. 49 is §U4r (:200-203). CHECK (2): PASS against the file as it stands.
- [block 4 done] every quoted phrase in the brief checked by script (scratchpad quote_cmp.py) against wave1, advisor-u4, plan, carries, catalogue, u4d-round2-{netcode,security}, plan.md, SERIES.md, u4d-report: 36 quotes of 18+ chars, 27 found verbatim; the 9 misses are templates (5), the brief's own words (1), bold/inner-quote formatting only (2: :161 W1-S1, :266 Q2), and one glyph: :495 "≥ 99 %" where u4d-round2-netcode.md:76 has ">= 99 %" (Low).
- [block 5 done] advisor-u4d-close.md read (B1.3 :165-180, B2 :229-308, B3 F3/F8 :321-361). THE PENDING ITEMS ARE NOW RULED (file 12:36; commit dad9566 "the advisor's Part B done"): carry 51's body reworded (main session adds `Scene::effect_rows()` when U4e's build starts; :233-244); new §U4e carries 53 (N2-L1 + security bullet 1, test: every client of an SS room, 48 clients, >=300 ticks, a respawn included), 54 (OutOfRange: skip, count, log, test), 55 (Team/Public rows), 56 (N2-L3 (c): report the own-state section's bytes, SS and ES), 57 (U4E-Q6 room half: overflow resync counted per client with its tick); N2-M2's U4e option REJECTED (B1.3 :170: U4d-F fixes ES's scene; U4e need not wait, :120-121); carry 40 reworded (F3); carry 58 (U4r) homes Q6's conditions. u4s-carries.md not yet edited (main session's step). => brief 3.2, 7.2 items 2-3, 2.7, R31, e12, return list 6 must fold these in (finding F1).
- [block 6 done] manifests read: room/Cargo.toml:15 bites = sim+proto; server/Cargo.toml:22 bites = sim+proto+room (NOT npt-client), deps link/proto/room/sim/net/log, dev npt-client :32-35; client/Cargo.toml:15 bites = sim+proto, deps link/proto/sim. So e7/2.5 in npt-server's tests can reach Room, Decoder, ClientEndpoint, Loopback, World with no manifest edit. Note: npt-client/bites only on under --workspace --all-features (finding, Low).
- [block 7 done] the bite-proof form. check.mjs testBothBuilds (:1493-1501) fails the step if any bites-build test fails; brief 2.6 (:354-356), 2.5/e5b (:306-308, :361) and return item 4 (:588-589) ask for a test that "fails by name in the bites build" and passes in the clean build, while section 5 asks check.mjs twice, every step passing; section 4 (:549-551) says a bite run needs no planted copy. Wave 1's form is two parts: a bites-build test that PASSES by asserting the check fails on the bitten instance (proto/tests/bites.rs:1-3, client/tests/bites.rs:7-36), plus in-place scratch switches of the named test to the bite, sha256-restored, showing the test failing by name (u4c-build.md:28-32, :66; u4d-report.md:19). Finding F2 (High).

## Check (1): §5 U4e (plan :703-734) against the brief
| Plan item | Brief | Testable as worded? |
|---|---|---|
| catalogue item 4 | e1 (2.4) | yes |
| forced group split | e2 | yes |
| lost group unlogged+counted; refused -> sentinel | e3 | yes |
| baseline rule; ring age edges | e4 | yes |
| 1 s gap evicting B | e5a/e5b | e5a yes; e5b's "fails by name under no-fallback" clashes with check.mjs (F2) |
| window counting the pump's bytes | e6 | yes |
| mirror = client hash, Loopback P0, 1,000 ticks | e7 | weak: nothing requires the run to reach priority skips, group splits or events (finding) |
| Send asserts | e8 | yes |
| 5 JoinRejected reasons | e9 | yes |
| datagram before Join dropped+counted | e9 | yes |
| control before Join: malformed, counted, never INVALID | e9 | yes |
| Input bucket 8 / 60 a second; Ping 10 a second, fake clock | e9 | yes |
| 2 s Join deadline, fake clock | e9 | yes |
| second Resync dropped+counted | e9 | yes |
| client wrong channel dropped+counted | e9 | yes |
| -18 end to end | 2.5 | yes for the clean half; the bite half as F2 |
| Bites line (12 names) | 2.6 table, all 12 present (tick16 as both -17 and -58) | each row's proof as F2 |
| Marks moved: none | 2 | n/a |

(working notes continue below as blocks finish)
