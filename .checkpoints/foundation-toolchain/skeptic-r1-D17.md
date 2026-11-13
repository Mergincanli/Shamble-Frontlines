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
