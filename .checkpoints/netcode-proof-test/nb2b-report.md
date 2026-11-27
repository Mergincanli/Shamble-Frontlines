# NB2-ONEWAY, Builder 2's one-way delay change merged into the netcode branch: the merge report
Merged on 2026-11-27 by the main session: merge cb600edee64b74a7e3bf82bd85791b61058d28bd (parents H0 d24340b7 and
Builder 2's tip 29a901994f5b65c23a17162422770aaacc0ca44e; code commit c56afd07002f6c13af418c8f016d6087fe806813), pushed
in b81061cf. Plan nb2b-merge-plan.md (a delta on nb2-merge-plan.md), approved with conditions NB2B-1 to NB2B-11
(advisor-nb2b-merge.md). The advisor agent read the pushed head (advisor-um4-nb2b-head.md, block 2: APPROVE WITH
FINDINGS, two Lows). His word: history.md:8138-8139 (12:25 UTC, items 2 and 3). His "yes" to the named command:
history.md:8219-8228 (17:03:02 UTC). NB2's own report goes with this one, in its own block (nb2-report.md; NB2B-9).

## What it is
- **The rule** (Builder 2's words for his NPT-L2-ONEWAY (a), q-oneway-merge.md:11-14):
  - On the real network (L2), the judge reads PM-0001-3's one-way delay checks against the written bound minus half
    the leg's own measured base round trip, and prints the bound it used.
  - No tolerance changes. The round-trip checks, the packet counts, the reports and every run on the model (L1) are
    untouched.
- **Size:** 12 files, +665/-17.
  - Six spike files under crates/judge/ and marks/ (+379/-6): eval.rs, marks.rs, engine.rs, README.md, SERIES.md and
    marks.json.
  - Six of Builder 2's notes: five added, one modified.
  - No manifest, lockfile, kit, CI or gate file among the 12 paths (`git diff --name-status 591e2cb T`).
- **marks.json:** one field changed, PM-0001-3's notes (his two sentences, copied by script). Its sha256 is
  a41ee583…24f2, his figure (history.md:8138). No pass mark, number or tolerance changes.

## Evidence on the merged tree
- **The merge brings exactly the tip's change:**
  - its tree is dd5973f4, the trial merge's (NB2B-3);
  - H0..merge and 591e2cb..T give the same 12 paths and counts, and their diffs hash equal (4186a375…15dc);
  - T..merge over the judge, marks and Builder 2's notes prints nothing (its control listed three marks files);
  - marks.json on disk hashes a41ee583….
- **His bar, the whole check** (check.log; 17:20:52-17:45:01 UTC; HEAD cb600ede, tree clean): 13 of 13 steps.
  - test 935 and 1,032 (UM-4's 931 and 1,028 + 4 each), 0 failed, 0 skipped.
  - refs 2,171 = 2,166 + the five added notes. redact 464 items = the base's 464 + 0 (NB2B-1), 0 findings.
  - marks 81, rows unchanged; self 85; web 70 of 70; ops 104 of 104 and "315 ok, 0 failed".
  - wasm 27,000 ticks, 0 mismatching.
- **The judge's own tests** (judge-tests.log): 136 passed (19 + 4 + 87 + 26), 0 failed. These are the tests his main
  Advisor did not run (history.md:8138).
- **The -47 re-judge**, as in the last merge: MOVED on both calls, exit 0. Both out files read "verdict": "MOVED".
- **Gate:** `node tools/gate.mjs --no-smoke`: GATE PASS, base b8f02b0. The output is kept whole (gate.txt, NB2B-6).
- **CI on b81061cf:** all four runs success at attempt 1.
  - gate: push 46044847948, pull request 46044855530;
  - toolchain: push 46044848099, pull request 46044855642.
  - PR #40 reads "blocked" (a draft), not "dirty": no conflict.
- **Two facts of the run, both accepted by the head read:**
  - **H0 (NB2BH-1):** d24340b7 is the saver's own save when it was stopped ("autosave (terminated)"). It carried the
    card's resume line and his reply's entry, so the explicit save found nothing to save. It is still the pushed
    commit that carries the resume line (`git ls-remote` = `git rev-parse HEAD`).
  - **The start time (NB2BH-2):** the whole check started at 17:20:52 (its log's lock line), after a worker restart.
    It did not start at 17:07, as first written. Nothing ran on the tree in between.

## Reviews: what covered each part (his rule of 2026-11-15, history.md:2125-2127)
- **1a637408, the one-way change (591e2cb..1a637408):** the netcode-architect's read, his "one reviewer's read".
  - Its file is on the branch: .checkpoints/netcode-judge/netcode-architect-oneway.md, whose line 1 names that range.
    Builder 2's card: netcode-judge.md:230-232.
  - APPROVE WITH FINDINGS, five Lows. Lows 1 to 4 were taken in the fix pass; Low 5 is a follow-up (his item 4,
    history.md:8140).
- **c56afd07, the fix pass (1a637408..c56afd07):**
  - the test-engineer's read in this lane (review-um4-nb2b-test.md, block 2; NB2B-8): APPROVE WITH FINDINGS, four
    Lows (T-6 to T-9);
  - the advisor agent's APPROVE of c56afd07, by Builder 2's record only (netcode-judge.md:240-241,
    q-oneway-merge.md:29). I have not seen its file.
  - His main Advisor checked c56afd07 (history.md:8138).
- **c56afd07..29a90199:** Builder 2's notes only. `git diff --name-only c56afd07 T -- …judge …marks` printed nothing;
  its control over 1a637408..c56afd07 listed eval.rs, engine.rs and README.md.
- **NB2-C10's M1** (figures.rs:722-724) is not touched by this tip. It stays Builder 2's forward fix (nb2-report.md).

## Findings and where they go
- **T-6 to T-9** (Low; review-um4-nb2b-test.md:211-241). Builder 2's lane, as forward fixes:
  - T-6: a stale reason left in a test (tests/engine.rs:5218);
  - T-7: the netting of `max` is claimed, but no test exercises it;
  - T-8: the plants' re-run after the fix pass has no kept output;
  - T-9 (pre-existing in 1a637408): a bound in force of exactly 0 is never tested.
  - All four were handed to Mustafa to paste into Builder 2's chat (the main session's chat message of about 18:1x UTC,
    its For Mustafa item 2).
- **NB2BH-1 and NB2BH-2** (Low, form and record). For the next plans of this form:
  - define H0 as the pushed commit that carries the resume line, whichever save made it;
  - done lines take their times from the log, after the event.

## Not verified
- **T-7:** the netting of `max` (README 2.3, marks.rs:308) has no test. It stands by reading only.
- **T-8:** the claim that thirteen planted defects each fail a test (c56afd07's message; q-oneway-merge.md:38) has no kept
  output. It stands by reading only.
- **The advisor agent's APPROVE of c56afd07** rests on Builder 2's record only.
- **No real L2 log was judged under the new rule in this merge.** Its tests use made-up logs, which the netcode-architect
  found look like the probe's (netcode-judge.md:230-232). The -47 re-judge reads MOVED before any check.
- **The smoke test** was skipped in the local gate (`--no-smoke`). The gate workflow's run on b81061cf read success.
- **PM-0001-57** is still not re-judged (nb2-report.md, NB2-C11).

## Decisions needed
None.

## For Mustafa
1. If not yet pasted: the block for Builder 2's chat with T-6 to T-9 (the main session's chat message of about 18:1x
   UTC, its For Mustafa item 2).
