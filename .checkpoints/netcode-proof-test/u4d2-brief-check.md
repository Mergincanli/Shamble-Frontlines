# test-engineer check of the U4d2 builder brief, rev 1

Brief: .checkpoints/netcode-proof-test/u4d2-brief.md (rev 1, drafted by the rust-reviewer; 719 lines, sha256 e5869dbf…,
unchanged through the check). Its notes: u4d2-brief-notes.md (220 lines). Read only.
Head at check start: 3032b88 (2026-11-12 08:15 UTC); at the end a7f3906 (autosave). spikes/ and docs/ at both equal
28a85e7 (`git diff --stat 28a85e7 HEAD -- spikes/ docs/` empty), whose only spikes/ change since adb1bab is scene.rs +7
(Scene::effect_rows, U4e's L3).
Memory: no test-engineer memory directory exists (.claude/ has no agent-memory); nothing to carry in.
Tools: no Edit tool in this run, so this file is rewritten whole with Write after each block.
Form: as u4e-brief-r3-check.md (plan, progress log, check tables, recheck rows, findings, verdict).
Runs: no cargo build, test, clippy, doc or check.mjs (W1-TARGET). Read-only commands only: git log/diff/show, sha256sum,
grep/sed reads, `rustc --print cfg` (three forms, below), `nm -D` on glibc, `node --version`. Web: two std source files
on raw.githubusercontent.com (rust-lang/rust master), read as data.

## Plan
1. Read the brief whole; section map; first stale spots.
2. Plan §5 U4d2 (:666-702), Lanes (:840-875), §4.1, §4.2, §4.11, §4.14, §4.15, §5 intro, §7; plan.md §4.1, §4.2, §4.5,
   §4.7. Check (1) coverage table.
3. Carries §U4d2 (:407-446), entry 34 (:62-69) with u4a-build.md and u4ab-*.md, the recordings section (:351-379).
4. Rulings: advisor-u4.md cond 2, 7, 8, 10 (b), 12, U4s cond 2, F2 (3); wave1 W1-TARGET, W1-PUB; close B1.2, B1.3, B2;
   df-ruling R1, R3, R4.1, R6, T4; j-r2 B2.3, B6; e-ruling F2-ROUTE, L4; and advisor-u4e-r1.md (new since the draft).
5. The rules every builder brief carries: the card's "Rules in force", history.md "Decisions taken", u4e-run.sh.
6. Check (4): recheck rows at the head.
7. Check (2) testability and check (5) questions and section 7.
8. Findings TB-n; verdict.

## Progress log
- [start] plan written.
- [block 1 done] brief read whole. Section map: header :1-11; 0 citations :13-41; 1 Start :45-97; 2 intro :101-109;
  2.1 hash-wasm :111-148; 2.2 loader :150-167; 2.3 npt-lab :169-202; 2.4 check.mjs :204-293; 2.5 manifests :295-306;
  2.6 tests :308-345; 2.7 bites :347-364; 2.8 seam :366-376; 2.9 pub :378-384; 3.1 carries :390-423; 3.2 :425-432;
  4 build rules :436-487; 5 end checks :491-504; 6 return :508-537; 7.1 facts :543-568; 7.2 :570-589; 8 Q1-Q11 :593-719.
- [block 2 done] plan read: §5 U4d2 :666-702 (Reviewers :701-702: rust-reviewer, engine-architect, test-engineer,
  infra-engineer); Lanes :840-875; §4.1 :178-203; §4.11 :392-407; §4.14 :476; §4.15 :478-504; §5 intro :507-512; §7
  :895-918; Q3 :981-985; Q9 :1002-1005; Q21 :1040-1042. plan.md §4.1 :392-393, §4.2 :435-447, §4.5 :531-551, §4.7 :617.
  Stamps hold: plan 686a583c…, plan.md 2bef127b…, carries cdab0240….
- [block 3 done] carries: §U4d2 = :407-446 (29, 41, 42, 52, 65, 66) at the head. Entry 34 met at U4a: u4a-build.md:125-126
  ("34 met by 4 listed fixes"), history.md:1335, u4ab-review-test.md:90, u4ab-review-rust.md:27; check.test.mjs pins
  STEPS.doc and the bites-only clippy summary today (:1617-1698). So carry 42 does not apply: agreed. U4d2's work is not
  a change to the synthetic recordings or the World's outcome on them (it edits no crates/sim file; the scratch plants
  are restored): 67, 71, 72 do not bind it: agreed.
- [block 4 done] rulings read; new fact: advisor-u4e-r1.md (written after the draft) T2 inserts carries 73-81 into
  u4s-carries.md §U4r (after 69), §U4f (after 46), §U4g (after 70) and §U4h (after 45), all above §U4d2's :407, in U4e's
  merge commit; T6 edits plan §4.1's rows. Its R6: "check.mjs ... keeps cargo's Compiling lines out of its output".
  No carry of that ruling is routed to U4d2.
- [block 5 done] rules in force: card :146-185 now (was :137-176 at cea8209): TE2-5 :171-174, B6 :175-176, T4 item 1
  :178-180, item 2 :181-184, "carries word for word" :167-168. The brief's card cites are those of cea8209 (git show
  cea8209: :162, :166, :169, :172): all moved by 9 lines. history.md:291-293 holds P2's words at the head; "Decisions
  taken" is newest first (card :190), so those lines move with the next decision. u4e-run.sh read (:16-17 exports,
  :30-32 log folder, :58-62 the touch loop).
- [block 6 done] recheck rows (below).
- [block 7 done] checks (2), (3), (5); findings; verdict.

### Check (1): plan §5 U4d2 against the brief
| Plan item | Brief | Testable as worded? |
|---|---|---|
| hash-wasm cdylib, npt-sim only, bites forwards (:667-670) | 2.1 :112-116 | yes |
| seven exports, integers only (:668-669) | 2.1 :117-139; Q3, Q7 | yes; behaviour at NeedMore, Done, Failed left to the builder's doc and h3 (fine) |
| Node loader, WebAssembly.instantiate, streams (:670-671) | 2.2 | yes |
| npt-lab `hash`, release, two native legs, first tick and field diff (:670-672) | 2.3 | yes; per-leg timing conflicts with "reads no clock" (TB-9) |
| HashCompare, two lists, U4r reuse (:672-673; advisor cond 2) | 2.3 :170-174, h4 | yes |
| wasm step, §4.5's flags, Node run, runner note, wall time, 5 min (:675-676) | 2.4 (a); Q2 | pass rule for the bites half missing (TB-2); self-test negatives missing (TB-6); provenance (TB-8); timing method (TB-9) |
| features comparison like for like (:677-678; A12) | 2.4 (b); Q6 | real-tree run vacuous-capable; live plant wrongly ruled out (TB-11) |
| build-flag check, read-back fails, self-test bite (:679-683; cond 7 (a)) | 2.4 (c); Q4 | Q4 (a)'s key property has no test that can fail (TB-3); no control for the scan (TB-16) |
| hash-wasm's rules, LEAF, WASM_CRATES, deps rule (:684-687) | 2.4 (d) | yes |
| third exception: invariant, test, UNSAFE_EXCEPTIONS, plan.md §4.2 (:688-693; cond 7 (b), (c)) | 2.1 :140-146, 2.4 (e); Q7, Q8 | yes; re-pins are expected failures the stop clause forbids (TB-7) |
| 0 mismatches over 27,000 ticks (:694) | h1 | yes, with exact counts (TB-6) |
| PM-0004-21 vectors (:694-695; T22) | h2; Q11 | the word-size plant's site is wrong (TB-4); one-vector reading needs its reason (TB-18) |
| bite std sin (:696-697) | 2.7 row 2; Q5 | no committed expected tick and no step verdict (TB-2) |
| bite hash-map, "caught, or not catchable on wasm32" (:697-698) | 2.7 row 1 | "not catchable on wasm32" cannot be decided with one Wasm leg (TB-5) |
| bite extra feature (PM-0004-74) | 2.7 row 3 | unit only; live plant possible in lane (TB-11) |
| bite each flag | 2.7 row 4 | live plant cannot tell the read-back from the scan (TB-3) |
| bite stray unsafe; export not on the list | 2.7 rows 5, 6 | yes |
| Reviewers (:701-702) | not in the brief | TB-13 |
| §5 intro (deltas, clippy, fmt, lock by named deps, pub items) | 4, 5, 6 | yes |
| carries 29, 41, 42, 52, 65, 66 | 3.1 placeholder P1 + notes | P1 by line range will drift at U4e's merge (TB-1) |
| advisor cond 2, 7, 8, 10 (b) | 2.3, 2.4, 2 intro, 4 | yes |
| W1-TARGET items 1-3 | 4 rules 1-6 | item 1's "the touch" omitted (TB-20); item 2 for release builds cannot hold (TB-8) |
| T4 items 1, 2; TE2-5; B6; P2; RM | 4, 3.2 | lines drifted (TB-1); "expected failures: none" wrong and wasm step not covered (TB-7); RM: not applicable, argued (agreed) |

### Check (3): the rules every builder brief carries
- T4 item 1 sentence: `<<P3>>` card :169-171: at the head those lines hold the end of the U4 bullet, the U4j ruling's
  heading and TE2-5's first line; the sentence is at :178-180 (df-ruling :462-464 holds it, stable). TB-1.
- T4 item 2 stop clause: `<<P4>>` card :172-175: now :181-184 (df-ruling :465-468). `<crate>` given as a note beside the
  copy, not substituted; "expected failures: none" wrong. TB-1, TB-7.
- TE2-5 rule: `<<P5>>` card :162-165: now :171-174 (j-r2 :168-171). TB-1.
- Builders' limits: `<<P2>>` history.md:291-293: holds at the head; newest-first file. TB-1.
- Runner and lock: `<<P7>>` (u4e-run.sh with .gate/u4d2/): sound; the runner takes -x per call and touches first.
- Plants by Edit (B6): carried, cite :166-167 now :175-176. TB-1.
- RM: 3.2 argues not applicable (no crates/sim change; plants restored): agreed.

### Check (4): recheck rows at the head (spikes/ = 28a85e7)
| # | Item | At the head | Bearing |
|---|---|---|---|
| C1 | `Replayer` | replay.rs:621-770; new :647, with_bite :662-670 (bites only; None for an unknown id), feed :673-675, state_hash/dump return Option (None before the header) :683-690, step :693-706 (drain past 64 KiB :701-704), header and World built in try_step :709-736, 0 ticks → Done | R1, R7 hold |
| C2 | header layout | replay.rs:8-15: words 0 magic, 1 version, 2 scene, 3-4 seed, 5 TICK_HZ, 6 first tick, 7 count, 8 span, 9 generator, 10-14 pools | Q3's "words 2 to 4" holds |
| C3 | GENERATOR, synthetic | replay.rs:288 `GENERATOR: u32 = 2`; synthetic(scene: &Scene, seed, first_tick, ticks, span_ms) :367-373; sets generator: GENERATOR (:602) | R2, R3 hold |
| C4 | World bites | world.rs:585-606 (enum, BITE_IDS), hash_map_order :608-623 (64 keys, FNV fold), std_sin :626-635, BITES :934-936, with_bite :940-966, bite_order_digest :968-973, sinf :994-1000 | R5 holds |
| C5 | the walk | write_state world.rs:1926; dump :1018-1022 calls it (:1020); state_hash :2141-2145 calls it (:2143); the fold is written last as `wide("bite", 0, "hash_map_order")` (:2017-2019, bites only); vehicles' pos as float bits | TB-4: the plant site is write_state, not dump |
| C6 | vehicle loops | place_vehicles world.rs:1216-1228: along = (elapsed + phase) mod period, theta = TAU × along / period; period = ticks_for_ms(60,000 + 5,000 v) (:794) | R6 and Q5's "every input met in 9,000 ticks" hold |
| C7 | StateDump | hash.rs:66-127: words :93, hash :98, field :107, first_difference(&[u32]) :115; `Fnv` pub(crate) :33 | a Wasm dump's words are enough for the field diff; the only public hash route is over words (TB-18) |
| C8 | D-2, D-7 | sim/tests/replay.rs:21-29 (the three recordings), :144-171 (Leg::spawn via current_exe), D-2 hash-map rule :284-309 (digests equal → no tick differs; else (0, arm, `bite[0].hash_map_order`)), D-7 :311-390 (initial dumps :320-325; recording pins :382-389) | R4 holds; D-2's verdict rule is what the step's bites half should state (TB-2) |
| C9 | npt-lab | lib.rs 6 lines, doc only; main.rs: dispatch probe only :186-213, params :116, summary :167-183 (prints `fate_log_sha256[..16]`), defaults :73-75, tests :215-; lab/Cargo.toml: serde_json, sha2; no HashMap, HashSet, Instant, SystemTime in lab/src (grep) | R21 holds |
| C10 | carry 66 | l1/src/lib.rs:351 for_each_canonical; tag v2 :329; l1/tests/l1.rs:481 v1_datagram_fnv, :518 the test; U2's raw logs (.gate/…/c-u2-pm-0001-3-81c6d01e371b/l1-{p2,b1,p4}-s1.jsonl) hold the three full values the brief gives (grep of `fate_log_sha256`) | 3.1's note holds |
| C11 | -3 script | ops/runs/c-u2-pm-0001-3.mjs:43-45 refuses a dirty tree; :55 `built` in the main folder's target; :65-72 probe lines as printed; :24-29 a JS copy of PM-0001-3's bite ids | R22 holds; the id-in-JS precedent (TB-12) |
| C12 | check.mjs constants | LEAF :31; UNSAFE_EXCEPTIONS :169; EXCEPTION_LEVELS :171; TCPINFO :173-174; tcpinfo rule :107-121; lab rule :152-157; edition note :197-206; PLACED_ALLOW :208-216; itemAfter :301-312; fnBodies :364-380; tcpinfoProblems :394-452; unsafeProblems :489-526 (message from UNSAFE_EXCEPTIONS :493); bite home :689-758; exceptionLintProblems :1023-1052; layoutProblems :1066-1092; cargoConfigProblems :1250-1284 (splits :1259, :1270; runner :1263, :1281); run :1398-1403; cargoCounts :1411-1421; WASM_CRATES/ARGS :1438-1439; lint/doc/test both builds :1456-1500; note :1527-1529; DEPS_CHECKED :1531-1550; depsStep :1563-1590; STEPS :1594-1627; main :1633-1654 (--only :1636-1638) | section 0's table holds; `--only` exists (useful for bite runs) |
| C13 | check.test.mjs | deps fixture :1094-1165 (summary pin :1099); the "only npt-client-wasm and npt-tcpinfo may" pins at :288, :486, :508, :1141; steps pin :1617-1658; 52 tests | TB-7: four pins fail when UNSAFE_EXCEPTIONS gains a third |
| C14 | manifests | Cargo.toml members :6-23, libm comment :50-53, lints comment :73-92, forbid :93, clippy :95-99, release overrides :108-112; proto/Cargo.toml:13-14 bites comment (stale); tcpinfo/Cargo.toml:9-23 lints form, :31-32 "a feature is not recorded in Cargo.lock"; server/Cargo.toml:22 bites = sim, proto, room; :29 npt-net | carries 29, 41 cites hold; TB-11's plant needs no lock change |
| C15 | rustc read-back (1.99.1) | x86-64: fxsr, sse, sse2, panic unwind; wasm32: bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext, panic abort, pointer width 32; `-C target-cpu=haswell` (x86-64) adds `target_feature="fma"`; `-C target-cpu=bleeding-edge` (wasm32) adds simd128 and relaxed-simd; `-C target-feature=+fma` on wasm32 only warns | R9 holds; TB-3's live plants measured |
| C16 | glibc 2.39 | `nm -D`: `W step@GLIBC_2.2.5`, `W advance@GLIBC_2.2.5`; no other export name | R12 holds |
| C17 | std on wasm32-unknown-unknown | library/std/src/sys/random/mod.rs routes `all(target_family = "wasm", target_os = "unknown")` to `unsupported`, whose `hashmap_random_keys` takes a stack and a heap address ("Use allocation addresses for a bit of randomness") | TB-5 |
| C18 | ADR rows | 0004:1142 PM-0004-21 (three injections; word-size bite; "armed at a known tick T with a value that differs"); 0004:1184 PM-0004-74 (control: a devtools-only feature passes); 0002:256-258 flags; :264-265 exception rule; :500-506 suite's tools | the brief's cites hold |
| C19 | U4e's patch | wip/u4e.patch: crates/client, crates/room, crates/server only (23 files) | R17 holds today |
| C20 | runner | u4e-run.sh: -x or -s per call; idle wait; 4 GB check; touch first and every 2 s (:58-62); provenance appended (:91-97) | TB-8, TB-9 |
- [R done] 20 rows; code cites in the brief hold; the drift is in the living notes files (card; carries at U4e's merge).

## Findings (severity on the one scale; brief line; fix)
High: TB-1 placeholders by line range in living files (drifted now; carries will drift at U4e's merge); TB-2 the
bites half has no verdict rule for std-sin and no home for the scan's result; TB-3 the read-back's Q4 (a) property has
no test that can fail.
Medium: TB-4 the usize plant's site; TB-5 hash-map "not catchable on wasm32" undecidable with one Wasm leg; TB-6 the
step's self-test negatives; TB-7 TE2-5's "expected failures: none" and the wasm step; TB-8 release-build provenance;
TB-9 timing method; TB-10 h7 has no exit code; TB-11 features: no non-vacuity, live plant possible.
Low: TB-12 ids in JS; TB-13 reviewers and condition 12's cite; TB-14 main-run.sh "own cache"; TB-15 h3's bites case;
TB-16 scan control; TB-17 the lock command; TB-18 one-vector reason and a text vector file; TB-19 carry 65's cases;
TB-20 lock wording.
(Full text of each in the return to the workflow.)

## Check (5): what is left to the main session and the advisor
- Q1-Q11 are all real and well posed, each with why, options and a recommendation. Q5 (a) needs the step's verdict and
  the scan result's home added (TB-2); Q11 (b) needs the plant's site corrected (TB-4); Q4 (a) needs its live proof
  (TB-3). No question is missing beyond those completions.
- 7.2 item 1: main-run.sh runs on the shared target, not "its own cache" (TB-14). Item 3: the copy by line ranges cannot
  catch a moved range (TB-1). Missing from 7.2: the review panel (TB-13); re-stamping the plan, carries and card at
  launch (TB-1).

## Verdict
REVISE: three Highs, each fixed by text; no ruling needs to change. Scope creep: none found.
- [done] verdict written.
