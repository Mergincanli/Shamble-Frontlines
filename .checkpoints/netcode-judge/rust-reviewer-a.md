# rust-reviewer, part a (MOVED) review round, diff e2c0e896..b22c30cd

## Plan
1. Read plan-a.md sections 2 and 6, marks/README.md 2.6, 3, 5, 6.
2. Read the full diff of crates/judge (marks.rs, eval.rs, lib.rs, tests).
3. Read surrounding unchanged code: eval.rs judge_mark, eval_part, eval_bite, judge_run, Rule::Pre, report_values,
   matched_parts, applies; marks.rs compile_mark, compile_check, compile_report, parse; practice.rs, figures.rs.
4. Grep every match over VerdictKind/BiteStatus and every iteration of parts/checks/bites.
5. Run clippy, fmt --check, tests, Cargo diff stat.
6. Write findings here, one line each, then hand back.

## Findings (one line each, as found)
- RAN (19:2x UTC): clippy -D warnings exit 0; fmt --check exit 0; tests 17+0+3+55+23 = 98 passed, 0 failed; Cargo.toml/lock diff empty.
- Read src diff whole: eval.rs, marks.rs, lib.rs. Determinism: only BTreeMap/Vec order; ratio.rs, stat_of, nearest_rank untouched.
- CANDIDATE Low: loader refuses "every part moved" but not "every check moved" nor a kept part left with no kept check (judged on validity alone; mark still reads MOVED, never PASS).
- CANDIDATE Low: expect-invalid bite whose rule lives only on a moved part: rules_of(kept) drops it, run reads Missed -> FAIL. Unreachable in real file (-30 has no moved part).
- CANDIDATE Low: practice::measured_of (practice.rs:406-422) picks measured runs by set/layer/profile/transport, not by part, so a moved part's runs on a ws-practice mark could still feed a kept bite's practice lookup. Unreachable (-18 has no moved part); loader could refuse moved part on a ws-practice mark.
- EVIDENCE A/B (built base e2c0e896 judge from git archive into .gate/rr-judge-base, target .gate/rr-base-target; head binary .gate/netcode-proof-test/target/debug/npt-judge; script scratchpad ab.mjs/ab2.mjs): 42 log folders (every fixture case folder + the whole fixtures tree). OLD marks.json on both judges: JSON byte-identical except PM-0001-39/-40/-63 verdict word NOT-RUN -> MOVED (intended, condition 7). Base+old vs head+new: only the 26 whole, 3 outside and 12 kept-with-moved-piece marks differ; PM-0006-14 INVALID stays INVALID (kept INVALID not hidden).
- Real file: kept checks whose every named part is moved (never evaluated, not labelled): -3 two "MP: server-measured RTT ..." checks, -19 "setup: at least 2 Worker traps injected" and "one timed rejoin per Worker trap"; reports -3 "MP: his own base RTT", -31 "ML*: p99 extra delay". Harmless for the verdict; Low (loader could say so).
- Fresh clippy (separate target .gate/rr-clippy-target, --offline): exit 0. rustdoc -D warnings on npt-judge: exit 0.
- Low: marks.rs RawPart.moved doc says "its runs are counted, never judged"; a moved part's runs are not counted anywhere (only a mark moved whole counts its runs). Drop "counted".
- Low: fixtures.rs a_label_anywhere...: assert_eq!(moved + missed + refused, positions.len()) cannot fail (each iteration bumps one counter or panics).
- Low: why lines of a kept mark reading MOVED: the generic part loop (eval.rs:1794-1798) appends each moved part's full line after the summary; moved checks and bites get none. Harmless, inconsistent.
- Low: kept_parts closure can retain in place (one Vec less per bite run, about 3 lines fewer).
- Note: outside marks read MOVED while their notes end "Always NOT-RUN here." (advisor condition 7: notes wait for Mustafa, OUTSIDE-NOTES). Not a Rust defect.

## Final (complete; the hand-back text is this block)
VERDICT: APPROVE WITH FINDINGS. No Critical, High or Medium. Seven Low, of which L1 and L2 are latent loader gaps
(unreachable with the committed marks.json) worth the small fix; the rest are optional or a STATUS follow-up.
L1 practice.rs:406-422 measured_of + marks.rs:933-954: moved part on a ws-practice mark can change a kept bite's verdict. Fix: refuse it in compile_mark.
L2 marks.rs:969-977: kept expect-invalid bite whose rule only a moved part names -> FAIL by absence. Fix: count only kept parts' rules for a kept bite.
L3 marks.rs (RawPart.moved doc, about line 99): "its runs are counted" is false for a part.
L4 eval.rs:1438-1444 + 1805: a kept bite whose runs were all left out reads "no run yet" though runs exist (moved parts). Wording; follow-up.
L5 fixtures.rs (a_label_anywhere...): tautological assert_eq!(moved + missed + refused, positions.len()).
L6 eval.rs:1794-1798: MOVED why list carries moved parts' full lines only (not checks or bites). Optional.
L7 eval.rs:1423-1431: kept_parts can retain in place (-3 lines, one Vec less). Optional.
Info: kept checks/reports whose every part moved are inert and unnamed (-3 x2 checks + 1 report, -19 x2 checks, -31 1 report); marks.rs:1050-1055 checks pre: rules on moved parts and moved-whole marks too (over-strict, harmless).
