# test-engineer: review of change-set 4b-1 "Content" (is it proven?), one read on head d36e416

Started 2026-11-26. Worktree .gate/lane1-takeover, branch cs/4b-content, code head d36e416 (+ 495204a notes).
Read-only; notes only here; target folder .gate/test-engineer-4b1-target; plants only in .gate/test-engineer-4b1-scratch.
Memory: none local (as in test-engineer-testlist.md).

## Plan
1. Briefs (done): review-brief-4b1-test-engineer.md, review-brief-4b1.md (rules), review-brief-4b1-round2.md.
2. Card steps 9-17, 23, 25-27; test-list.md items for steps 1-5; plants-4b1.py; local-checks.sh.
3. Whole diff `git diff origin/main...HEAD -- crates packages`: file list, then per file.
4. Look-hardest list 1-10 in order: content_proofs.rs case() (circularity), vectors digests (regeneration),
   content_fuzz.rs (shares, seed, reach), limit cases + allocator + content.test.mjs ceilings, map.rs + aim_reach.rs,
   time unit + report, links check, rules_scan content scan, plants list (no plant / wrong check), CI vs by hand.
5. Run: cargo test (debug) for content + devtools in my target; selected tests; scratch plants in a copy where a
   point needs proof.
6. Findings as found (below), then the handback (<= 25 lines).

## Source notes (one line per file/source as read)

## Findings (one line per finding as found)
- Briefs, card steps 9-27, test-list.md, my own testlist read: read. Head d36e416 (+495204a), tree clean but the
  three reviewers' untracked notes.
- crates/content/src (json, number, read, write, bundle, error, map, movement, time): read whole. No assert!/split_at/
  remove/insert outside tests (awk scan). Duplicate keys: sort of (key, place), later named (json.rs:317-332).
- content_proofs.rs read whole: case() expectations are typed record strings + the generator's own layout model
  (File::lines, bundle_bytes, nest, echo, form, random_id); circular only where stated: number_text for values,
  Map::text through json::parse + canonical (content_proofs.rs:443-449); FIELDS ranges shared (data, not logic).
- verdicts(): got==expect asserted in every run (native and Wasm); digest folds bytes(<=256 KiB)+record+message
  (content_proofs.rs:1110-1121). Test pins seed 20261126 / 500 in code (tests/content_proofs.rs:81-83).
- content_fuzz.rs: SHARES [900,0,1000,5,700,1600,150,300,90]; stage map content_proofs.rs:1261-1284 lumps
  IdNotFileName with the map rules in stage 8, NumberNotCanonical with NotCanonical in stage 6.
- CI: toolchain.yml:162 release line has -p devtools -> the million run (cfg(not(debug_assertions))) DOES run in CI;
  only -p content's own unit tests and clippy are by hand (STATUS_ARCHIVE 4B-CI-CONTENT says the release line only;
  clippy (RR-2) not named there at 495204a).
- time: units_from_seconds tie pinned (0.5/3000*3000 == 0.5 exactly, python check; half-even would give 0);
  report's exact path tie only at 0.0005 s = 1.5 units (half-even also 2): not pinned against half-even.
- links: scan of files.rs counts "File::open(" and "fs::read(" only: read_to_string / OpenOptions not seen
  (recordings.rs:226-235). Node helper lstat-then-read (plain-file.ts:18-23), checked by path twice.
- rules_scan content: PANICS lacks assert!/assert_eq!/debug_assert; UFCS `Option::unwrap(x)` not seen (method rule
  needs a `.` before). Lint guard (clippy::panic) does not deny assert! either.
- Ran (my target, debug, --locked --offline): content 35 unit tests ok; devtools content_fuzz, content_proofs,
  time_unit, aim_reach, dependency_rules, recordings: all ok. Limit cases (debug): 16,383 keys 245,748 B in /
  1,540,074 held / 45 ms; one key x16,383 131,065 / 1,327,111 / 19 ms; 100,000 keys 800,001 / 1,196,041 / 18 ms;
  eight 1 MB files 8,006,189 / 9,011,083 / 1.13 s; fifteen texts 7,865,384 / 7,870,046 / 0.98 s. Bounds: held <=
  in + 4 MiB; time < 20 s (debug) or 2 s (release) per started MiB (content_fuzz.rs:98-99,127-128).
  Fuzz 50,000 debug: shares 1869/0/2122/11/1503/3289/395/614/194 vs minimums 900/0/1000/5/700/1600/150/300/90.
- MEASURED (scratch copy .gate/test-engineer-4b1-scratch/ws, extra test te_tally.rs, release, seed 4, 200,000):
  stage 8 = 3,878 = IdNotFileName 3,453 (172.6 in 10,000; from fuzz_input's fixed rename to movement/another.json)
  + map rules 425 (OffGrid 193, TooThin 132, OutsideBounds 95, BeyondAim 5, DuplicateId 0). Minimum 90 is met by
  IdNotFileName alone: the share cannot see the map rules vanish. Stage 6: NumberNotCanonical alone 211 > 150.
- SCRATCH PLANTS (te_plants.py on the copy; the tree untouched): 1 node cap +1 -> CAUGHT by class "more values
  than a document holds"; 2 files cap +1 -> CAUGHT by "more files than a bundle holds"; 3 OutsideBounds lax on the
  high side -> CAUGHT by "a map's box ... outside the bounds" (shapes[0].max[1]); 4 duplicate-key search made
  quadratic -> NOT CAUGHT: at_its_caps passes, 16,383 distinct keys 45 ms -> 1.82 s debug (bound 20 s), 442 ms
  release (bound 2 s); 5 no map in any fuzz input -> NOT CAUGHT: fifty_thousand passes, stage 8 at 199 in 10,000
  (IdNotFileName alone).
- FINDING F1 (Medium): stage shares cannot see the map kind leave the fuzz run (content_fuzz.rs:136,
  content_proofs.rs:1265-1267 lumps IdNotFileName with the map rules). Fix: own stage or own minimum for the map's
  rules (~10 in 10,000; measured 21) and for map files taken. Before 4b-2's first map file.
- FINDING F2 (Medium/Low): time ceilings cannot see a quadratic duplicate-key search (S-must 3's O(k log k)
  unproven); content_fuzz.rs:95-99 and content.test.mjs:21-24 claim "a loader whose time grows faster than its
  input does not [pass]". Fix: a ratio test (distinct keys vs an array of the same node count) or drop the claim.
- SCRATCH WASM (my own cargo build of devtools for wasm32 from the copy + the checked wasm-bindgen into scratch;
  te_wasm_memory.mjs repeats content.test.mjs's calls in order): clean release 49,479,680 B after the limits
  (2,228,224 after corpora, 11,403,264 after verdicts). With plant b (String::new(), SA-1's doubling): release
  49,479,680 B (identical), dev 49,545,216 B: both under 64 MiB. The high water is set by the eight-file case,
  which runs before the fifteen-text case; the doubled strings (about 23.6 MB live) fit in memory already grown.
- FINDING F3 (Medium): the Node and browser memory ceilings cannot see SA-1's defect (content.test.mjs:18-20,51;
  content.spec.ts:81-82); only the native counting allocator does (plants-4b1.py plants b against CAPS only). The
  round-2 brief says the fifth case runs "native counting allocator, Node, browsers". Fix: say Wasm checks verdict
  and time only, or measure each heavy case in a fresh instance (case 4 alone: about 24 MB vs 16 MB clean).
- F3 refined: a fresh release instance running ONLY the fifteen-text case (copy's limits_check filtered to case 4)
  grows 1,114,112 -> 38,731,776 B clean AND with the doubling plant (identical): the generator's own construction
  (fifteen 512 KiB texts + bundle_bytes' growing Vec) leaves enough freed memory to absorb the doubled strings. So
  in Wasm the ceiling measures the module (generator included), not the loader; "a fresh instance per case" would
  not fix it. Fix: reword (memory per case is native only), or a counting allocator in the test module.
- F2 Wasm numbers (scratch builds, content_limits all five cases): clean release 270 ms, quadratic 854 ms (limit
  5,000); clean dev 3,308 ms, quadratic 6,093 ms (limit 60,000): passes everywhere. Ratio idea: native debug case 0
  vs case 1 is 45/19 ms = 2.4x clean, 1,823/15 ms = 120x quadratic.

## Ranked findings (final)
- F1 Medium | content_fuzz.rs:136 + content_proofs.rs:1265-1267 | stage 8's share met by IdNotFileName alone; map
  rules 21 in 10,000 (DuplicateId 0, BeyondAim 5 in 200,000) unguarded | tally + plant 5 | own share for map rules
  (~10) and map files taken | can wait, best before 4b-2 step 8.
- F2 Medium | content_fuzz.rs:95-99,128; content.test.mjs:21-24 | time limits cannot see a quadratic dup search |
  plant 4 native + Wasm numbers | ratio check or drop the claim | can wait.
- F3 Medium | content.test.mjs:18-20,51; content.spec.ts:81-82 | Wasm memory ceilings blind to SA-1's doubling |
  49,479,680 = 49,479,680; case 4 alone 38,731,776 both | reword / count in-module | can wait (High if the report
  counts them as memory evidence).
- L1 Low | time_report.rs:46-51, time_unit.rs:92-95 | exact path's tie not pinned vs half-even | 1.5 units both 2 |
  add 0.0015 s (4.5 -> 5) | can wait.
- L2 Low | recordings.rs:218-220,228-234; files.rs:100-101 | scan misses read_to_string/OpenOptions, same-line only;
  doc says every new reader reads through read_plain, dependency_rules.rs:42 and rules_scan.rs:590 do not | read |
  widen the count, narrow the doc | can wait.
- L3 Low | rules_scan.rs:508-515,559 | assert!/assert_eq!/unwrap_err/expect_err and UFCS Option::unwrap not seen |
  read | add them + bites | can wait.
- L4 Low | error.rs:424-429; tests/content_proofs.rs:76-79 | only one sentence pinned; others only by the digest
  that SHAMBLE_WRITE_VERDICTS=1 rewrites | read | one sentence per Kind | can wait.
- L5 Low (wording) | review-brief-4b1-round2/test-engineer item 10 | the million run is in CI (toolchain.yml:162,
  -p devtools release); by hand: -p content release and clippy; 4B-CI-CONTENT names no clippy step | read | fix
  the report's words and the follow-up | can wait.
DONE 2026-11-26.
