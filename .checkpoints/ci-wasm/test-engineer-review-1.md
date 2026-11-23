# test-engineer, review round 1 on PR #64 (cs/ci-wasm, head f54bd1c; code and CI state of ce5a7e7)

Question: IS IT PROVEN? Review only; exact replacement text with every finding; one severity scale.

## Plan
1. Read the record: plan.md rev 4 (s1 goal, s6 my follow-ups, s9 evidence + stop rule, s12 review order), the card
   ci-wasm.md "Current step" (run ids), my notes test-engineer-followups.md. Memory: none on disk (memory: local, empty).
2. Read the OTHER part first (plan s12): `.github/workflows/toolchain.yml` from the diff, the in-cut script from
   "Build and test with the network cut off".
3. Then my own part: the rest of `git diff origin/main HEAD -- .github .gitignore packages` (specs, build-checks test,
   tsconfigs, package files).
4. Logs (.gate/ci-wasm/logs/): map every claim of plan s1 to log lines in the cut (6 clean logs), and the 4 planted logs.
   Read stop-rule-check.py and what it asserts.
5. Q2: extract the readings lines of all six clean logs into a table (scratch script under .gate/ci-wasm/te-review-1/),
   margins against the specs' bounds (file:line), test durations against timeouts.
6. Q4: find build-checks.test.mjs in the runner log; re-run my seven mutants on scratch copies if cheap (node by path).
7. Q5: find the five type-check lines in the log; plant a type error in a scratch copy of the specs and run tsc by path.
8. Q6: check the planted runs against plan s9 "Planted changes" and "What this cannot show"; explain P-BROWSER's two
   failures from the log (the runner's default backend).
9. Q7: what is not proven; STATUS follow-ups.
10. Verdict + findings with severity, file:line, replacement text.

## Notes (a line after each block)
- B1 (record) done: plan rev 4 read (s1 goal, s6, s9 stop rule + plants + cannot-show); card "Current step" (runs: push 137974047720, pr 137974099620, takeback-browser 137981826960/137981878828, takeback-gh 138010909634/138010956116; plants 137977706424/137977761665 P-BROWSER, 138003254035/138003287607 P-GH); my followups notes. Memory: none on disk.
- B1 note: head moved during the review f54bd1c -> faae0b8 (e98b927 docs/DECISIONS.md, faae0b8 decisions-3.mjs); `git diff origin/main HEAD -- .github .gitignore packages` is the same at both (only docs/ and .checkpoints/ changed). Code/CI state = ce5a7e7 = 2a654f3 outside .checkpoints (checked: empty diff).
- B2 (workflow, infra's part) read: toolchain.yml:132-197 is the cut; new lines 171-196; bash -euo pipefail; every new line is a simple command or a `|| { ...; exit 1; }` guard, except 176 (grep || echo: printed, not judged, by design).
- B3 finding (Low, record): playwright.config.ts:26 sets `timeout: 90_000`; plan.md:87 and :157 say `timeout: 60_000` (commit 3cddb55's message says 90 s, the right value: a 60 s test limit would tie with report.ts's 60 s wait). Plan/report wording to follow the code.
- B3 obs (out of lane, for the advisor): all 92 branch commits carry "Co-Authored-By: Claude Fable 5.2"; CLAUDE.md "The main model and every agent stay Opus 5.5". Not verified what model the main session ran.
- B4 (push log read whole, 932 lines): checkout 2a654f3 (:110,:128); token Contents read/Metadata read (:23-24); gh 2.104.0 (:270), verify success (:276); install lines (:317-320); cut (:442) before all build/test lines; unit tests incl. scripts/build-checks.test.mjs (18 tests) (:753), client 49 passed (:760); builds (:834-869); Wasm sha 22dad2eb... (:871), registry path /home/runner/.cargo/... (:872); boundary block (:873-882) "boundary/sim-wasm.test.mjs (7 tests)" + "Tests 7 passed (7)" (:877,:880); type checks: only the echo (:884) and the next echo (:885), 7 s apart, no per-line output (tsc prints nothing on success; set -e makes the next echo the proof); Chrome 157.0.8103.78 (:887); Running 12 tests using 1 worker (:889); 12 passed (1.1m) (:915).
- B4 finding (Medium, evidence method): stop-rule-check.py:25 `Tests .*7 passed` is satisfied by packages/types' own "Tests 7 passed (7)" (push log :816, before the boundary block) -> that one of the 17 "yes" cannot fail; the boundary count must be anchored to its block (:873-884). By hand: the boundary line is present in the push log (:877, :880).
- B5 (readings) done: scratch readings.py on the 6 clean + 2 P-BROWSER logs (te-review-1/readings-clean.txt, readings-plant-browser.txt). All on image 20261116.133.1, Chrome 157.0.8103.78, 4 cores, AWS us-west-2 x5 / us-east-1 x3 (8 logs); no CPU model printed. Every bound wide on the runner: alloc frames 65-76 (>20), perFrame 47-61 (25..170), perBlock 487-523 (430..610, thinnest relative: 12% above the floor), rate <=16467 (<100000), blocks in window 243-244 (>200), end 340-343 (<420), drops 0; first page frames 29-30 (>3), frames-posts 0, |b-t| 0, wakes = ticks+1 (slack 9.4-10.3 vs 1.1t+2); hidden 0 ticks while hidden, since showing 42-51 vs most 57-60 (slack 9-15, >=2 by construction); notice 130-542 ms (20 s); clock 64 ticks/2 s (>=30). Durations: max 14.8 s (allocation) of 90 s = 16%; run 1.0-1.1 min of the 300 s global; job ~2.3-2.5 min of 10 (stop rule)/15.
- B6 (P-BROWSER) done: both logs fail tests 8 and 9 at skeleton.spec.ts:67 (-"drawCalls": 4 / +3), "2 failed", "10 passed (1.1m)", ##[error] exit 1, no retry line, inside the cut (:442 cut). Two, not one: render-counts.json has counts only for "WebGL 2 fallback" (WebGPU null), and the runner's default backend IS "WebGL 2 fallback" (readings: webgpuOffered true, adapter SwiftShader), so both tests compare the same planted baseline.
- B6 (P-GH) done: both logs: gh 2.104.0, then "attestation for 0.2.130 does not contain subject sha256:27c24e42..." and exit 1 at that step; no step group after it (only Post job cleanup). Proves: the check is live, its failure stops the job, contents:read suffices to load the statement. Not: the signature path (subject compared first; F-PGH).
- B7 (TE-NT6 plants) note: plants-subset-output.txt (container, 2026-11-19 20:00-20:08, head c934fc0 = after the edits): 10 cs2 plants all caught; 8b (hidden gap run on return) is caught at skeleton.spec.ts:102 (drops), so no plant reaches the changed bound at :104 (`most`). Finding (Low): the changed bound's own negative case is missing.
- B8 (RR-N4) done: rr-n4/run.mjs runs the repo's own test (unchanged; only its vitest import -> stand-in) on scratch copies of build-checks.mjs: clean PASS; M1-M7 (my seven) all FAIL as they must; X8-X14 (RUST_LOG, RUSTUP_TOOLCHAIN, CARGO_ENCODED_RUSTFLAGS, bare RUSTC, widened CARGO_.*, widened RUSTUP_.*, function returns []) all FAIL: 14 of 14 caught (rr-n4/output.txt). Runner log: push :753 "scripts/build-checks.test.mjs (18 tests)" (17 before + 1), :760 "Tests 49 passed (49)", inside the cut.
