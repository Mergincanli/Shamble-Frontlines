# Advisor: NB2-ONEWAY merge plan check (nb2b-merge-plan.md), 2026-11-27
Read only: no cargo, no git write, no fetch. Rules: `git show origin/main:.claude/agents/advisor.md`.
Builder 2's tip at sending: 29a901994f5b65c23a17162422770aaacc0ca44e. Main at sending: b8f02b0 (#78; MAIN-69 owed).

## My plan (blocks, each written when done)
1. Rules and words: advisor.md on main (changed since my last read?); his word of 12:25 (history.md:8133-8151) and of
   13:55 (history.md:8163-8186), word for word.
2. The plan: read whole; its "Since the draft" section and Q1-Q4; its sha256.
3. The tip's facts, by SHA: the tip's ancestry against the merged 48efc9c (Builder 2 took the merged head in?); the
   merge base with HEAD; the diff merge-base..tip (paths, stat, modes, guard pathspec, manifests); which commits are
   code and which notes; the trial merge on HEAD; overlap with this branch since the base.
4. What the code change touches (crates/net? the probe's one-way checks; marks/SERIES?) and whether the "same steps and
   bars" fit: the counts, refs, redact; the re-judge (does a -47-style re-judge apply?); the reviewers' reads (his
   2026-11-15 rule if the judge or marks.json changes).
5. The order: MAIN-69 first; "never overlap"; R1's path; the gap.
6. Q1-Q4.
7. Verdict, conditions, findings, Decisions needed, For Mustafa, Status line.

## Blocks

### Block 1: the rules and his words (done)
- advisor.md on main: last changed c9bd268 (11-12), the version this seat read and follows.
- Plan nb2b-merge-plan.md: sha256 3f2ea2e468b3918969b4f1e44c36a904b87b1bf99da25aa51f703418352f1641 (109 lines,
  "Since the draft" and Q4 included).
- His 12:25 word (history.md:8133-8151) read word for word. The plan's quote of item 3 is exact. Item 2's facts are
  re-read in block 2.
- His 13:55 word (history.md:8163-8186) read word for word: item 1's limit "do not start a step that must not be cut,
  such as a merge's no-save stretch, unless the usage left covers it"; item 4 "name the command as soon as you are
  ready for it".
- His 2026-11-25 words that this unit rests on:
  - his message to Builder 2, item 4 (T:.checkpoints/netcode-judge/mustafa-2026-11-25.md): "plan first, your advisor
    agent's review, one reviewer's read, ready before R1's report. Take the netcode branch's merged head in first.";
  - his answer (a) in this chat (history.md:7906).
- HEAD ccbd53453bf0 = origin; main b8f02b0149cc; T 29a901994f5b present locally.

### Block 2: the tip's facts, by SHA (done)
- **T's history:** 48efc9c..T is this branch's own commits to 591e2cb (Builder 2 fast-forwarded to it), then Builder
  2's nine:
  - notes: 942855b, 4dfdbed, befd4e0, 6ceaa07, f87b8f8, df47b19, 29a9019;
  - code: 1a63740 (the build) and c56afd0 (the fix pass).
  - `git merge-base HEAD T` = 591e2cb0ab07d8483290c98d8f6ab8595a2621e3.
- **His condition holds:** `git rev-parse c56afd0` = c56afd07002f6c13af418c8f016d6087fe806813, his code commit. The
  three commits after it: f87b8f8 (M .checkpoints/netcode-judge.md), df47b19 (the same), 29a9019 (M netcode-judge.md,
  A netcode-judge/q-oneway-merge.md).
- **591e2cb..T** (`--numstat`, `--raw`): 12 paths, +665/-17, all 100644. Notes: netcode-judge.md 54/11 M, and five A
  (na-plant.sh 30, netcode-architect-oneway.md 42, oneway-note.mjs 44, plan-oneway.md 71, q-oneway-merge.md 45). Spike,
  all M: eval.rs 70/2, marks.rs 28/0, tests/engine.rs 263/0, marks/README.md 14/0, SERIES.md 3/3, marks.json 1/1
  (+379/-6, as the plan says).
- **The kit pathspec** (main-61-plan.md:37-40, plus tools, Cargo.toml, Cargo.lock, package*.json and rust-toolchain*)
  over 591e2cb..T prints nothing. The control (the same form plus spikes/netcode-proof/marks) prints the three marks
  files.
- **Ignore rules:** `git check-ignore --no-index -v -n` on the five new notes prints "::" for each: none is ignored.
- **marks.json:** T's blob is 84e382e7972aba928f6a9587bc74631783627794. One hunk (@@ -2368 +2368 @@): PM-0001-3's
  "notes" gains a sentence quoting him twice, both quotes exact (his item 4 of 25 Nov to Builder 2; history.md:7906).
- **This branch since 591e2cb:**
  - in crates/judge, marks and netcode-judge*: nothing (the control, ops/operator, lists PROMPT-R1.md and rules.md);
  - non-notes commits: 6b4a0d3 (MAIN-68), 6968215 (U7), ddb06ff (U8-F1), d8b57b8 (the operator rewordings).
- **Trial merge** on HEAD ccbd534: one line, tree f164a5794880205a428b80975765107ec4214c5e, no conflict (the plan's
  147dec96 was on 7b50a66).
- **What the code does:**
  - `one_way_bound` (eval.rs) returns None for every check except a statistic of place over probe_up_delay_ns or
    probe_down_delay_ns. For such a check it reads `params.base_rtt_ns` from each L2 log that carries the series.
  - marks.rs refuses a pooled or "against" one-way check.
  - judge_mark's early return for a moved mark is untouched, so the -47 re-judge reads MOVED exactly as at 48efc9c.
  - SERIES.md's three rows keep their names.
- **The writer:** HEAD's L2 probe already puts `base_rtt_ns` in its header (net/src/l2/socket.rs:338;
  server/src/probe_tests.rs:359 asserts it), so R1's L2 probe logs carry what the judge now reads.
- **Builder 2's card at T (steps 31-35):**
  - the netcode-architect's read on 1a637408 (APPROVE WITH FINDINGS, five Low);
  - the fix pass c56afd07, read by the advisor agent (APPROVE);
  - the whole check 11 of 11, test 912 and 1009 (908 and 1005 at 591e2cb's code: +4 each); 136 judge tests.
  - The fix pass itself (`--numstat 1a637408 c56afd07`): eval.rs 5/5, engine.rs 26/15, README.md 2/2.
- **Main's move** b9aad01..b8f02b0: M .github/workflows/toolchain.yml, M docs/DECISIONS.md, REPO_MAP.md, ROADMAP.md,
  STATUS.md, A docs/reports/2026-11-27-ci-time.md (#78). Trial merge of b8f02b0 into HEAD: one tree line ea1b2f9e, no
  conflict. PR #40 (gh): head ccbd534, mergeable true, mergeable_state "blocked" (a draft).
- **check.mjs at HEAD:** 12 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, web, redact).
  The redact step scans `HERE` = spikes/netcode-proof only (check.mjs:2717).
- **UM-4's plan (rev 0.3):** step 0 pins origin/main = b9aad01 (:66, a CI move stops). Step 5 holds test, web and wasm
  strictly to U8-F1's M5 with the proviso `git diff --name-only ddb06ff HEAD -- spikes/` (:104-108). Its build waits
  for this file (launch-1550-msgs.md:18).
- **U8-F1's M5** (.gate/u8-f1-merge/m5-check.log): 12 of 12 steps; test 931 and 1028; ops tests 104 of 104 and
  "wrappers.test.sh: 315 ok, 0 failed" beside the run; self 80 of 80; web 70 of 70; redact 464 items; refs 2145.
- **The card (HEAD :9):**
  - NB2-C10 DONE (review-nb2-c10.md: APPROVE WITH FINDINGS, M1 Medium at figures.rs:722-724, Builder 2's to fix; T
    does not touch figures.rs);
  - "Still open from NB2's merge: NB2's report …, then -47's pack".
  - `ls -d` lists the -47 folder's measured and bites (unpacked).
