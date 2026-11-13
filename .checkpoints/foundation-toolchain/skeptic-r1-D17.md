# Skeptic r1: D17 (= TE-3, Low): two checks never seen red in CI

Plan: (1) confirm the code under review equals 4d78ad6; (2) re-read toolchain.yml :46-52 (pin check) and :76-81 (probe)
and the report's claims about them (the "not proven" list, "Findings for 2.3", the plan's Part 7 sentence); (3) re-pull
the CI logs of every toolchain run and count probe outcomes and pin-check reds; (4) re-run the branches locally on a copy
in .gate/skeptic-D17/ (TE's drive.py, then my own mutations); (5) check PM-0002-52's wording; (6) judge holds, severity,
and the fix.

## Checks
- C1 code under review: `git diff --stat 4d78ad6 HEAD` touches only `.checkpoints/` (27 files), so toolchain.yml at HEAD
  is 4d78ad6's. toolchain.yml:46-53 (pin check) and :76-81 (probe) read as TE-3 quotes them.
- C2 the plan's sentence exists: plan.md:492-493 "The control and probe inside E2's one green run already show that the
  probe bites." (Part 7, the planted-failures paragraph). Plan 4.4 says what proves the cut: M1a, M1b.
- C3 PM-0002-52 (ADR 0002:1318): "at least one mutation and one control per check id (G0-G12; G13 from slice 2)", over
  "every mutation of PM-0002-39 to PM-0002-51". TE's paraphrase is accurate. Note: the cut is PM-0002-37 and the
  version-equality half of the pin check is PM-0002-38, both outside 39..51; the "never uses another Rust" half sits in
  PM-0002-39 (G0). So PM-0002-52 asks one mutation per check id, not one per branch.
- C4 the report has no "not proven" section; its claims touching the two checks: :26 (describes the pin check), :35
  "PM-0002-37 is proven, not partly: on every run the probe ended with curl exit 7 inside the cut, and a network use
  planted in each half went red there (M1a, M1b)", E2 (:96, green path only), M2 (:99, missing file only), :200 "2.2's
  job already fails closed without the file (M2)". None claims the probe's rc-0 branch or the pin check's :47-52
  branches were seen red.
- C5 TE's harness is faithful: toolchain.yml:46-53 de-indented is byte-identical to te-r1/branches/pincheck.sh; :76-81
  equals probe.sh plus one marker line ("AFTER-PROBE"). Shells match the job's (`bash --noprofile --norc -e -o pipefail`
  for the step, as GitHub runs `shell: bash`; `-euo pipefail` for the inner bash).
- C6 re-ran TE's drive.py from a copy (.gate/skeptic-D17/te-copy/): 17 of 17 PASS, the same lines TE quotes (S1-S10,
  probe rc 0 -> exit 1 "still reachable", rc 7/28 -> pass, rc 6/35/127 -> exit 1 "probe inconclusive"). So both checks
  read correctly with fakes; this is not CI.
- C7 CI, all 40 toolchain runs on the branch (API list: 22 success, 14 failure, 4 cancelled; one job each; logs pulled
  into .gate/skeptic-D17/ci-data/, tallied by scripts/analyze.py, the script's own cyan echo lines left out): 30 jobs
  reached the cut (22 green + M1a, M1b, M4, M5 x2 each); in all 30 "control: connected to" and "the network is cut: no
  TCP connection"; 0 "still reachable", 0 "probe inconclusive", 0 "control failed". TE's "0 over all 30 runs" holds.
- C8 pin check in CI: 34 jobs "rustc equals the pin" (step green), 2 red, both M2 (0d3ec36: job 130098974545 push, and
  130096054588 pull_request; TE named only the first): "sed: can't read rust-toolchain.toml" then "Process completed
  with exit code 2" (log :187-188), nothing from :47-52 printed; the install step had fallen back to "stable ... it's
  the default toolchain" (:172-173). 0 lines of "not the pin", "no single exact", "RUSTUP_TOOLCHAIN is set", "does not
  come from". The 4 cancelled jobs stopped at "Set up job" (3 have no log: BlobNotFound). TE's claim holds.
- C9 no run went without `--net`: `git log -- .github/workflows/toolchain.yml` lists only fb17451 (K1) and fd14855 (K1b);
  `git diff fb17451 4d78ad6` on it is the one K1b line; no planted commit touched it. So no run could take the rc-0
  branch, nor set RUSTUP_TOOLCHAIN or an override.
- C10 TE's exact grep on my own logs: `grep -h -E 'still reachable|probe inconclusive' log-*.txt | grep -v '36;1m' | wc -l`
  = 0. The 30 cut lines all say "(curl exit 7)"; inner message "curl: (7) Failed to connect to github.com" x30 (so the
  report's ":35 on every run the probe ended with curl exit 7" holds too).
- C11 durable docs: ROADMAP:118-120 ("checks rustc against the pin ... (a TCP probe must fail inside); it is green on the
  build's heads, and seven planted failures each went red"), STATUS_ARCHIVE:586-589 (G0: "fails closed without the file
  (M2 ...)"), REPO_MAP:146: none claims the probe's rc-0 branch or the pin check's :47-52 branches seen red. The only
  overstatement is the plan's own sentence (C2), in a checkpoint file deleted before ready; the report does not carry it
  (grep "bites" in the report: only L1).
- C12 what carries PM-0002-37: M1a (job 130094623250, log :271-282: "failed to lookup address information: Temporary
  failure in name resolution", exit 101) and M1b (job 130095416867, :376-392: "fetch failed ... EAI_AGAIN github.com",
  exit 1) went red inside the cut, both on DNS; the probe's exit 7 after a connected control on all 30 runs covers TCP.
  The rc-0 branch is a regression guard for later heads, not part of this head's proof. So "not a gap in 2.2's
  evidence" holds.
- C13 ADR 0002 part 9 (:567-587): the pin check's job is G0's ("toolchain equals rust-toolchain.toml ... exit 2 on any
  setup failure"); `--prove` "applies each planted defect in a throwaway worktree", at least one mutation and one
  control per check id; PM-0002-39 (:1305) already names G0's mutations (file deleted, Cargo.toml without lock, missing
  browser; control: a consistent bump). The cut has no G-check: PM-0002-37 is a one-run CI mark (:1303, "1 run").
  So (a) PM-0002-52's per-check-id minimum is met for G0 by the file deletion alone, which dies before :47-52 (M2);
  (b) a `--prove` mutation can reach the probe only if 2.3 moves the cut into the gate; otherwise it is a planted CI
  run like 2.2's M-runs. Both bear on the fix's wording, not on whether the finding holds.
- C14 the report's "Findings for 2.3" table (:156) keeps a pointer only: "their full text: docs/STATUS_ARCHIVE.md,
  'Follow-ups', 'From foundation 2.2'" (:551-631 holds one bullet per row). D7 (= IE-3) is the finding that carries the
  cut into 2.3 at all.

## Verdict draft
Holds as stated, Low. Every factual claim re-ran: 0 of 30 cut runs took the rc-0 or inconclusive branch; the only
pin-check reds are M2's two runs, dead at :46's sed (exit 2) before :47-52; the report claims neither proven; both read
correctly (17 of 17 with fakes, harness faithful). No defect in the checks and no evidence that does not hold, so not
Medium or High. Fix: right in substance (a 2.3 line; no CI edit in 2.2, as the review budget and the K1 ask require);
incomplete in three small ways: mirror the full text in STATUS_ARCHIVE "From foundation 2.2"; put the probe half in D7's
row and name the proof as "a red run with the cut removed (a `--prove` mutation if 2.3 moves the cut into the gate, else
a planted CI run)"; do not cite PM-0002-52 as requiring these (its G0 minimum is met by the file deletion, which never
reaches :47-52): say they go beyond it. The step-7 fold must not carry plan.md:492-493's "already show that the probe
bites" into the report. Breaks nothing (docs only, no word of Mustafa's).

## Checks after the advisor tool's read
- C15 re-run attempts: `run_attempt` is 1 for all 40 runs (runs-p1.json), so the jobs endpoint's default (latest
  attempt) saw every job; no flaky red hidden by a re-run. M2's push job 130098974545 was created 20:13:07 for a run
  created 20:08:06: it queued behind the cancelled 5c64404 job 130096016288 (20:08:06 to 20:13:07, no steps). Attempt 1.
- C16 IE-3's full text (findings JSON, value.findings IE-3): fix "CUT-CARRY: the gate v2 workflow keeps the cut (the
  control, the probe, curl exit 7 or 28 only) around its build and test jobs, or 2.3's plan drops it by decision". D7's
  skeptic (skeptic-r1-D7.md:92-108): holds, Low; CUT-CARRY in three places (the report's table, STATUS_ARCHIVE's full
  text, STATUS.md :96-99's names). STATUS.md:96-99 confirmed: the 2.2 names list (G0 is on it).
- C17 shell flags from the log, not memory: head job 131510223737 log :185, the pin-check step: "shell: /usr/bin/bash
  --noprofile --norc -e -o pipefail {0}", the flags TE's drive.py passes.

## Verdict (final)
Holds, Low. The facts all re-ran (C6-C10, C15, C17); the report and durable docs claim neither red branch (C4, C11);
2.2's proof of PM-0002-37 does not rest on the rc-0 branch (C12). Caveat on "as stated": the "2.3 --prove item" label
fits the pin check (G0) but is not established for the probe (the ADR gives the cut no check id, C13).
Fix: right in substance; docs only; no CI edit in 2.2; breaks nothing. Placement and wording: (1) a new 2.3 name goes in
three places (report :156 table, STATUS_ARCHIVE :551-631, STATUS.md :96-99), or better, no new name: the pin-check
mutation into the existing G0 row, the probe's red run into CUT-CARRY if D7's row is adopted (else its own row);
(2) for the probe name the proof, not `--prove`: a red run with `--net` removed, ending "still reachable" (a `--prove`
mutation only if 2.3 moves the cut into the gate, else a planted CI run like M1a-M6); (3) PM-0002-52 is cited
accurately, but its G0 minimum is already met by the file deletion, which dies at :46 before :47-52, so these go beyond
it; (4) the step-7 fold must not carry plan.md:492-493's "already show that the probe bites".
