# Seat T (test-engineer): UM-4 and NB2B-8 proof review

Brief: launch-1750-msgs.md section T. Read only. Branch claude/netcode-proof-test-plan-hcky5g, head b81061cf.

## Plan
1. Block 1 (UM-4, commit 8fb32bc8):
   a. `git show 8fb32bc8 -- spikes/netcode-proof/check.mjs spikes/netcode-proof/check.test.mjs`: opsStep, OPS_MISSING,
      OPS_BROWSER_MODULES, the five new tests.
   b. um4-plan.md rev 0.3 + done lines; advisor-um4-plan.md UM4-1..UM4-8.
   c. Logs .gate/um4/absent.log, ops-pass.log, counted.log (+ light-base.log, commit-msg.txt).
   d. Questions: each failure branch has a test that fails by name if its guard is removed; any test leaning on the real
      ops/browser/node_modules (STEPS.ops reaching opsStep without an injected `present`); the glob vs u8-brief.md:524;
      the wrappers' last-line parse when stdout ends with other output; CI never runs check.mjs: does the local run
      suffice.
   e. Run `node --test spikes/netcode-proof/check.test.mjs` from the repo root (allowed); mutation by reading only
      (no edits to shipped files; any scratch copy lives in the scratchpad).
2. Block 2 (NB2B-8): `git diff 1a637408 c56afd07 -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`:
   each change proven by a test in tests/engine.rs; marks/README.md 2.3 and marks.json PM-0001-3 notes vs eval.rs and
   marks.rs. NB2-C10 M1 (figures.rs:722-724) excluded.
3. Verdicts, severity counts, first action. Last line "Status: COMPLETE <UTC time>".

No cargo, no check.mjs.

## Notes (one line per finding as it lands)
- 18:00Z draft of block 1 saved; extra checks (last-line mutant, skip grep, nb2b check.log) still running.
- Mutation harness: scratchpad mutate-ops.mjs (copies check.mjs, check.test.mjs and crates/hash-wasm/loader.mjs to a
  scratch folder, one exact once-only replacement per mutant, runs `node --test --test-name-pattern="the ops step"`).
  No repository file touched. Results: M0 control 5/5; M1 (drop check.mjs:2696, node exit guard) SURVIVES 5/5; M2 (drop
  :2698, zero count) killed by :2033; M3 (drop :2700, wrappers exit guard) SURVIVES 5/5; M4 (`w.out` for `w.stdout ||
  w.out`, :2702) SURVIVES; M5 (drop N > 0, :2704) killed by :2043; M6 (drop :2694, present) killed by :2052; M7
  (STEPS.ops passes `() => true`, :2740) SURVIVES; M8 (OPS_BROWSER_MODULES = ops/browser, :2684) SURVIVES; M9 (last-line
  regex accepts any failed count, :2703) killed by :2043. W1 (outside the diff: drop webStep's node exit guard) SURVIVES.
- Node glob (scratch globexp.sh, dummy files, Node 22.23.4): `node --test "ops/**/*.test.mjs"` ran ops/top and ops/a/deep
  only; it did not enter ops/node_modules, ops/a/node_modules/pkg or ops/.hidden.
- Counted run tied to the commit: `git diff 65b2f296 8fb32bc8 -- .` from spikes/netcode-proof hashes to 09c26ef2…b858,
  the value absent.log, ops-pass.log and counted.log print; untracked e3b0c442… = sha256 of empty input.
- `node --test spikes/netcode-proof/check.test.mjs` at b81061cf: 85 tests, 85 pass, 0 fail, 0 skipped.
- 18:0xZ round 2 (scratchpad mutate-ops2.mjs): M10 (summary accepted on any line, :2702) SURVIVES; with stand-ins that
  pass the neighbouring guards (node `# tests 104 / # fail 1` exit 1; wrappers `315 ok, 0 failed` exit 1, replacing
  :2024's "boom"), M1 and M3 are KILLED by :2021 and the control stays 5/5. No trap in wrappers.test.sh or npt-lib.sh;
  no skip/todo form in ops tests. .gate/nb2b-merge/check.log:16 (HEAD cb600ede, clean tree): ops 104 of 104, 315 ok.
- Checked and dropped (not a finding): c56afd07's "Co-Authored-By: Claude Fable 5.2" trailer agrees with his model
  words for builder chats on his personal account (docs/DECISIONS.md:2835, :3388).

## Block 1: UM-4 (done 18:0xZ)
Verdict: APPROVE WITH FINDINGS (1 Medium, 4 Low). The step's code is right; two of its five guards have no test that
fails if the guard is removed.

| Claim (8fb32bc8, um4-plan.md) | Proof | Gap |
|---|---|---|
| Missing install: fails, one line naming it, runs nothing (UM-4 (d)) | check.test.mjs:2052 (M6 killed); absent.log real FAIL, "(1 lines)", 0.0 s, exit 1 | real `present` wiring only by the logs (T-4) |
| Node tests failing fails the step | code read, check.mjs:2696 | test :2021 cannot fail without it (M1 survives): T-1 |
| No Node test ran fails it (UM4-1) | :2033, both "# tests 0" and no count (M2 killed) | none |
| Wrappers failing fails it | code read, :2700 | M3 survives: T-1 |
| Last line `wrappers.test.sh: N ok, 0 failed`, N > 0 (UM4-1) | :2043 (M5, M9 killed) | stdout-only read and "last" untested (M4, M10): T-2 |
| Both commands' program, args, folder pinned (UM4-2) | :2009-2019 deepEqual of [cmd, args, cwd] | none |
| Order: nothing after the failing part runs | :2021's `calls.length === at + 1` | none |
| "ops" in STEPS after "web"; header names it | check.test.mjs:2904; check.mjs:22-23 | none |
| Glob selects U8's tests (u8-brief.md:305, :334, :524) | 9 files; static 92 + 10 + 2 = 104 = counted.log; Node skips node_modules (scratch run) | none |
| Whole run 13 of 13, cold; bars of B's C1 rule | counted.log; diff hash 09c26ef2 = `git diff 65b2f296 8fb32bc8 -- .`; self 85 = 80 + 5; refs, marks, redact = light-base.log | no later-head net (T-5) |
| "Five new tests, each red by its own name first" (commit message) | um4-plan.md:140-150 | holds for three, not five: T-1 |

### Findings, ranked
**T-1 (Medium): the "either part failing" test cannot fail if either exit guard is removed.**
check.test.mjs:2021-2030 against check.mjs:2696 and :2700.
- Mutants M1 (drop :2696) and M3 (drop :2700) leave all 5 ops tests green. The shipped stand-in at :2024 (`out: "boom\n"`)
  trips the zero-count guard (:2698) or the last-line guard (:2704) instead. :2029 then checks only the prefix
  ("the ops step's node tests", "the ops step's wrappers test"), and both messages share it.
- Real reach of M1: a Node run that exits 1 with `# tests 104 ... # fail 1` passes :2698. The wrappers then pass, so the
  step PASSES with a failing ops test. That is the failure the step exists to stop.
- The test was never red. um4-plan.md:146 says "the failing-part guard (passes on B's step)", and :2009 arrived green
  with the step (B, :143-145). The commit message's "Five new tests, each red by its own name first, then green"
  (8fb32bc8; .gate/um4/commit-msg.txt:7) holds for three of the five (zero count, last line, missing folder), not five.
- Why not High: both guards are present and correct at 8fb32bc8 (read), and two whole runs passed (counted.log;
  .gate/nb2b-merge/check.log:16). The result is right today; the gap is the regression net and an overstated summary.
- Fix: stand-ins that pass the neighbouring guards. Node fails with `# tests 104\n# pass 103\n# fail 1\n`, status 1;
  the wrappers fail with `wrappers.test.sh: 315 ok, 0 failed\n`, status 1. Assert each part's exact message
  (`/^the ops step's node tests \(103 of 104 tests passed\) failed/`, `/^the ops step's wrappers test failed/`). Shown in
  scratch: this kills M1 and M3, and the control stays 5 of 5. Leave the commit message's sentence out of the report.
- Carry, outside UM-4's diff: the web step's test (check.test.mjs:1985-1995, stand-in :1988) has the same shape. W1
  (drop webStep's node exit guard) survives too. Same fix; it does not move this verdict.

**T-2 (Low): the stdout-only read and the "last line" rule have no case where they matter.** check.mjs:2702-2704;
check.test.mjs:2007, :2024, :2045 (every stand-in sets `out` equal to `stdout`).
- M4 (read `w.out`) survives.
- M10 (accept the summary on any line) survives: no tail is a valid summary followed by more output.
- The brief's question, answered by reading: the parse holds. Output after the summary makes the step FAIL with "its
  last line: <that line>", which is the safe direction. stderr cannot interfere (stdout is read).
- Fix: two cases. A summary on stdout with stderr noise after it in `out` must pass. A summary followed by `extra\n`
  must fail.

**T-3 (Low): a skipped or todo ops test passes the step.** check.mjs:2696-2698, :2705. The step reads `# tests`, never
`# skipped` or `# todo`. A skip would show only as "N of 104" in the summary, for a human to read against the bar.
- Today: no skip or todo form in ops/tests/*.mjs or redact-check.test.mjs (grep).
- The web step does the same.
- Forward only: fail when `# pass` is below `# tests`, or say why not.

**T-4 (Low): the real `present` and its path are proven only by the logs.**
- M7 (STEPS.ops passes `() => true`, check.mjs:2740) and M8 (OPS_BROWSER_MODULES = ops/browser, :2684) both survive.
  check.test.mjs:2010 checks `STEPS.ops.length === 0` only.
- absent.log (FAIL) and ops-pass.log (PASS) prove both sides of the real check. The diff hash ties both logs to the
  commit.
- If either regressed, the step would fail on browser.test.mjs through :2696 (safe, with a worse message), unless T-1's
  M1 had landed too.
- Fix (optional): `assert.equal(OPS_BROWSER_MODULES, join(HERE, "ops", "browser", "node_modules"))`.

**T-5 (Low, forward, infra-engineer's): no net for later heads.** No workflow names spikes/, check.mjs or netcode-proof
(`git grep` over .github/ at HEAD: no match; gate.yml names gate.mjs).
- Does the local run suffice? For UM-4 at 8fb32bc8, yes:
  - the counted run is cold and whole (13 of 13), on exactly the committed content (09c26ef2);
  - the step's files and ops/ are unchanged since (`git diff --stat 8fb32bc8 HEAD`: empty);
  - the merge head cb600ede re-showed the line on a clean tree (.gate/nb2b-merge/check.log:16).
- For later heads, no: check.test.mjs (85 tests, about 3 s, no install, no cargo) could run in CI as it stands. The ops
  tests would need ops/browser's install there.

### The brief's questions, answered
- Does each failure branch have a test that fails by name if its guard is removed?
  - Yes for three of five: present (:2694, by :2052), zero count (:2698, by :2033), and the last line (:2703-2704,
    regex and N > 0, by :2043).
  - No for the two exit guards (:2696, :2700): T-1.
- Does any test lean on the real ops/browser/node_modules? No.
  - opsStep is always called with a stand-in `present` (:2011, :2026, :2036, :2046, :2054).
  - STEPS.ops is never called, only its `.length` read (:2010).
  - The STEPS driver at :1919-1935 runs fmt, clippy, clippy-wasm32 and test only. main() gets stand-in steps only
    (:2694, :2702).
  - Shown: the five ops tests pass in a scratch copy that has no ops/ folder at all.
- Does the glob select what u8-brief.md:524 asks? Yes.
  - :524 asks for "the `ops` step in check.mjs". :305 names ops/tests/*.test.mjs under node --test, and :334 names
    `node --test ops/`.
  - The glob selects nine files: those eight, plus ops/operator/redact-check.test.mjs (inside :334's ops/, and run again
    by the redact step, as the advisor accepted). helpers.mjs is not a test file and is rightly left out.
  - A static count gives 92 declarations, plus 10 from cloudinit's two-role loop (11 tests, 1 rs-only), plus 2 from
    firewall's two-entry table, which makes 104. That equals counted.log's 104.
  - Node's test glob enters no node_modules or dot folder at any depth (scratch run on Node 22.23.4).
    ops/browser/node_modules holds 0 *.test.mjs. The plan's unread point (um4-plan.md:47) is now read.
- Does the last-line parse hold if stdout ends with other output? Yes, it fails safe. It is untested: T-2.
- CI never runs check.mjs: T-5. It suffices for this commit; it is no net for later heads.
- UM4-1 to UM4-3 are in the code and the logs: the zero-count and last-line tests went red by name (um4-plan.md:147),
  the form and pins match UM4-2, and 2b is absent.log. UM4-4 to UM4-8 are process; their done lines are at
  um4-plan.md:133-139.

## Block 2: NB2B-8, Builder 2's fix pass (1a637408..c56afd07; done 18:06Z)
Read only: no cargo. Every "killed" below is by reading the shipped code against the test, not by a run.
- The range is what landed: `git diff c56afd07 cb600ede -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`
  prints nothing. The fix pass's three files: eval.rs (+5/-5), tests/engine.rs (+26/-15), marks/README.md (+2/-2).
  marks.json is untouched: sha256 a41ee583…a4f2 = the card's (netcode-judge.md:224).
- The judge's own run on the merge head (.gate/nb2b-merge/judge-tests.log: HEAD cb600ede, clean tree, `cargo test
  --locked -p npt-judge --no-fail-fast`): 19 + 4 + 87 + 26 = 136 passed, 0 failed. The four `oneway_` tests are `ok` by
  name. engine.rs holds 87 `#[test]`, which matches the 87 binary. The fix pass adds no test function: it widens three
  and edits one, so the count stays 136.
- The whole check on cb600ede (.gate/nb2b-merge/check.log): 13 of 13; test 935 and 1,032 (UM-4's 931 and 1,028 plus
  the four oneway tests); marks 81, rows unchanged.

Verdict: APPROVE WITH FINDINGS (4 Low). Each changed line is pinned by an exact assertion through the shipped judge.
README 2.3 and PM-0001-3's notes match eval.rs and marks.rs.

| Change (c56afd07) | Proof (tests/engine.rs) | Gap |
|---|---|---|
| L1, plant h (drop Stat::Min\|Max, marks.rs:308): a `min` row "least" (:5076) | :5132 all seven Pass (the 19.7 ms least fails the written 19.75 ms, passes 19.45 ms) and :5135 its needs exact: Min un-netted fails both | Max alone still survives: T-7 |
| L1, plant f (drop the carries-the-series filter, eval.rs:485-486): a second L2 log, `load`, no seconds, no base (:5138-5142) | :5142 all Pass. Under f the load log reaches :491, so all five one-way checks gap. The check has no `from` (log_matches, eval.rs:108-111, is empty, so it matches all); run_matches keeps run p1 | none |
| L2, plant c (whole base): one real-marks bound to the letter (:5299-5300) | count of `>= 19550000 (the written 19750000 less half the base round trip 400000)` == 2 (one per direction); a whole base gives 19350000 | none |
| L3 wording, eval.rs:471-472 (doc) and :515-516 (message); README:198 | :5221-5226 pins the new message exactly | stale test comment: T-6 |
| L4 wording, eval.rs:584 "the written {t}"; README:194 | `net()` (:5120) in every exact needs assert (:5133-5135, base 0 at :5150-5153, odd base at :5167-5171) and the real-marks pin | none |
| Gap test widened to indices [0, 1, 2, 3, 6] (:5175-5187) | superset of the old `[..4]`; the count picks by `logs.len()` (1 or 2 in every call) | none |
| "thirteen planted defects each fail a test" (c56afd07 message; netcode-judge.md:237) | no kept output in this repo | T-8 |

### README 2.3 and PM-0001-3's notes against the code
- Which checks are netted: a check with a `value` (Threshold::Value), over `second.s.probe_up_delay_ns` or
  `…down…` (Source::SecondS in ONE_WAY_SERIES, marks.rs:299), with stat mean, min, max or a percentile
  (marks.rs:308). This is eval.rs:477-481 and marks.rs:304-313. Match.
- Which logs it reads: a log on L2 that carries the series (eval.rs:485-486). Match.
- The arithmetic: the written bound minus half the base, exact (`Ratio::new(base, 2)`, eval.rs:512). The odd base is
  tested at :5167-5171 (19449999.5). Match.
- The printed bound: README's example `>= 23550000 (the written 23750000 less half the base round trip 400000)` is
  eval.rs:584's format through `needs_text`, and 23,750,000 - 200,000 = 23,550,000. Match.
- Nothing else moves:
  - the round trip and the count, needs[4..6] as written (:5137);
  - L1 never netted (:5144-5149);
  - reports: they go through report_values, never eval_check (netcode-architect-oneway.md:24; not re-read by me).
  Match.
- The gaps: no param, a non-whole or negative param ("600000", -2, 600000.5), logs that disagree, and half a base
  above the bound (net < 0, eval.rs:513). Each has a test at :5193-5226. Match.
- Plain check only: the loader refuses `pooled` and `against`, tested at :5252-5266. Match.
- The Fail and Empty branches of eval_check (:563-576) print the written bound. Measure::Series never returns those
  (only figures do: figures.rs:1985-1986), so a netted check never reaches them. No mismatch.
- PM-0001-3's notes quote his two sentences and point to README 2.3. "Minus half the measured base round trip; no
  tolerance changes": each bound of the series moves by the same half (both ends together). "The round-trip checks stay
  as they are": probe_rtt_ns is not in ONE_WAY_SERIES. "The leg's own measured value": the base is read from each log's
  header params. All three match.
  - The notes say "the profile's one-way delay" (his words), where README now says "the written" bound (L4). These do
    not conflict: the notes name what is subtracted from, and README names the number printed.

### Findings, ranked (all Low)
**T-6 (Low): the stale reason left in the test.** tests/engine.rs:5218 still reads "Half the base above a written bound:
the model adds no delay there, so that check has no bound to read". That is the reasoning L3 called false
(netcode-architect-oneway.md:33-35), and it sits directly above the `far` message it pins (:5221-5222). Grep over
crates/judge and marks: this is the only occurrence left. Fix: "…: the bound in force would be below 0 ns."

**T-7 (Low): the netting of `max` is claimed but never exercised.**
- README 2.3 and marks.rs:308 include Stat::Max. No test row and no real check uses it:
  - one_way_mark (:5069-5077) has mean, p001, the rtt mean, count and min;
  - PM-0001-3's real checks use mean, p001, p999 and p90 only (read from marks.json).
- A plant that drops only `Stat::Max` survives (read): the reviewer's plant h dropped Min and Max together, and the new
  min row kills only the Min half.
- Fix: a row (UP, "max", ">=", 29_750_000), with its needs asserted exact (the gap test's index list gains it). The
  stamps' max of 29.7 ms fails the written bound and passes the bound in force, 29,450,000.

**T-8 (Low): the plants' re-run after the fix pass has no kept output.**
- "Thirteen planted defects (eleven of mine, two of the reviewer's that had survived), each failing a test"
  (q-oneway-merge.md:38; the commit message). na-plant.sh prints to its terminal only, and no plant log exists under
  .gate/nb2-merge or .gate/nb2b-merge.
- By reading, the new asserts kill c, f and h's Min half (table above). The claim stands by reading, not by a record.
- Forward: a plant run's output kept beside the judge's log, as UM-4 keeps its logs.

**T-9 (Low, pre-existing in 1a637408, sharpened by L3): the bound-of-0 boundary is untested.**
- eval.rs:513 is `net < Ratio::int(0)`, and README now says the gap is "below 0 ns", so a bound in force of exactly 0 is
  read, not gapped.
- No test has net = 0. The bases used are 0, 400,000, 600,000, 600,001, 600,002 and 40,000,000; none is twice a
  netted written bound (39.5, 40.5, 47.5 or 52.5 ms). So `<=` in place of `<` survives (read).
- Fix: one log with base 39,500,000 (twice the written 19,750,000): "end low" reads `>= 0 (…)` and is not a Gap.

Not re-reported, as the brief says: NB2-C10's M1 (figures.rs:722-724) and the reviewer's L5 (the P0 part's label),
both Builder 2's follow-ups.

## Summary
- Block 1 (UM-4): APPROVE WITH FINDINGS. Block 2 (NB2B-8): APPROVE WITH FINDINGS.
- Counts: Critical 0, High 0, Medium 1 (T-1), Low 8 (T-2 to T-9).
- First to act on: T-1. Give check.test.mjs:2021-2030 stand-ins that pass the zero-count and last-line guards, and
  assert each part's exact message, so dropping check.mjs:2696 or :2700 turns a named test red. Do not carry the
  commit message's "each red by its own name first" into the report.
- Not verified here: no cargo, plant or check.mjs run of mine. Block 2's kills are by reading. Block 1's mutants ran
  on scratch copies under the scratchpad (mutate-ops.mjs, mutate-ops2.mjs, globexp.sh), filtered to the ops tests.

Status: COMPLETE 2026-11-27T18:06Z
