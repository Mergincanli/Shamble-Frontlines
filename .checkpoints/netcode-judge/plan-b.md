# Part b's plan: U6j (the judge learns 19 figures; three waiting judge changes; two standing items first)
2026-11-24, Netcode Builder 2. For the advisor agent's review; no judge or marks edit for part b before its APPROVE.
Base: part a's head 0e0f654776fc0e10e153875d59b33869e6179a88. Paths are under `spikes/netcode-proof/`.
**The facts this plan rests on are in `figures-b.md`** (test-engineer's spec sheet, 358 lines, every claim cited): one
block per figure (A1-A11), the three changes (B1-B3), the two standing items (C1-C2), sizes and order (D), the rulings
needed (E). This plan says what is built, in which order, how it is proven, and what needs a ruling; it repeats none
of the sheet.

## 1. Goal and sources
- **Goal (Mustafa's start block, part b; his FIGURES answer of 2026-11-24):** the judge learns the 19 figures that
  marks.json names and it cannot compute yet (figures.rs's `planned(.., "U6", ..)` less `rtt_agreement_p99_ns`, whose
  mark PM-0001-33 moved), plus three waiting judge changes. His U6-JUDGE (b) text (u6-brief.md:160-178): "This changes
  the judge's code and those two texts, not marks.json, a pass mark or a number." So **marks.json is not edited in
  part b**; the two texts are `marks/SERIES.md` (only the rows these figures need) and `marks/README.md`.
- **Change type:** logic (computations on results logs). Risk tier 1. Fence, checks and review as part a's.

## 2. What is built, in nine blocks (each compiles with its tests, clippy and fmt before the next; sizes: sheet D)
1. **J0, the two standing items for "the judge's next change"** (sheet C1, C2; about 285 lines): u4s-carries.md entry
   64 (a) to (f), each test shown failing under its plant and passing restored; JDG-ONLY-1: `--only` judges the named
   marks against the whole marks file (`judge_only`; main.rs drops its `retain`), with a command-line test on the real
   marks.json and the PM-0001-4 fixture.
2. **The plumbing three figures need** (sheet 0.1; about 285): `Compute` takes `&Inputs { run, set, parts }`, `parts`
   being each kept part's valid runs from a first pass in `judge_mark`; `Lack::Fail(why)` (a failure with no number);
   a detail text beside a reported value ("the arithmetic shown"). The 8 built figures move to `Inputs` unchanged;
   all 98 tests keep their results.
3. **C-U6-9, C-U6-10 and `alloc_twin_diff_count`** (sheet B1, B2, A11; about 413): -30's twin form in
   `twin_hash_mismatch_count` (both measured, each naming the other, `params.transport` wt against ws; every other
   measured twin stays a gap) with fixtures under `crates/judge/fixtures/pm-0001-30/`; the planted-format bite run's
   logging-off twin accepted in `twin_of` for that pair only (C4's role list unchanged everywhere else).
4. **C-U6-11** (sheet B3; about 220): the coverage rule's spans per session, cut at the room log's `admit` events; a
   session that ends early may leave room hashes unresolved only up to a recorded `session_end`; every PM-0001-17
   fixture keeps its verdict. README section 1's warm-up sentence and SERIES.md's coverage clause are reworded.
5. **`down_client_mean_max_bits`, `down_mean_ipv6_bits`, `up_client_mean_max_bits`, `bulk_control_p99_rise_ns`,
   `one_shot_dropped_count`** (A2, A3, A7; about 344).
6. **`rss_slope_bytes_per_10min`, `queue_growth_count`, `session_memory_growth_count`** (A4, A5; about 292): the
   least-squares slope as one exact fraction in i128; the growth rule by time windows.
7. **`sync_unit_cost`, `sync_cost_ratio`, `kill_p99_ratio`** (A1, A6; about 309).
8. **`r2_down_formula_bits`, `r2_egress_formula_bytes`, `egress_interface_bytes`,
   `egress_metric_vs_interface_ratio`, `egress_vs_r2_ratio`** (A2, A9; about 304), with R2's own counts as the golden
   case (3653064/25 bits; 3,156,247,296 B).
9. **`crash_close_max_ns`, `logging_cost_ratio`, the texts** (A8, A10; about 299): SERIES.md's rows (the twin row's
   two forms, the coverage clause, `room_end`'s `slot`, the L2 home of `admit` and `session_end`, the :284 citation),
   README.md, fixtures/README.md; figures.rs's list (19 `planned` become built; `rtt_agreement_p99_ns` stays planned,
   named as moved).
- **Symbols:** figures.rs: `Inputs`, `Lack::Fail`, 19 `fn`s and a few shared helpers (`slot_mean_max`, `growth_count`,
  `r2_bits`, a per-session span in `coverage`); eval.rs: the first pass over parts, `judge_only`, `Outcome` for
  `Lack::Fail`, the report's detail text, `summary_line`'s escape (lib.rs); main.rs: `--only` through `judge_only`.
- **Not touched:** marks.json; every other crate; check.mjs; ops/; manifests and lockfiles.

## 3. Tests and evidence
- **Every computation: pass, fail and gap cases on made-up logs** (his rule), as the sheet lists per figure, plus the
  waiting case where a figure compares with a run not in the set yet, and an empty comparison where one exists. One
  planted defect per figure at least, each failing a named test and taken back (the sheet names them); entry 64's
  items by their own plants. The exact-arithmetic cases: R2's golden counts; a slope over a 2-hour 1 GB series (no
  overflow).
- **No writer exists yet for 16 of the input series** (sheet's head): every figure is proven on made-up logs only, and
  the report says so per figure. The first real logs come with the netcode chat's later units and R2, R3, R5.
- **Checks on the final head, in this container:** as part a's (`cargo test -p npt-judge --locked --no-fail-fast`,
  clippy with warnings as errors, `cargo fmt --check`, `node spikes/netcode-proof/check.mjs` whole); GitHub's gate and
  toolchain green on the final code commit, pushed without a skip marker. After each block: the judge's tests, clippy,
  fmt, a commit and a push, and the card.
- **Review, one round on the final diff:** test-engineer and rust-reviewer at xhigh, each reading every judge line;
  test-engineer runs the tests itself. One fix pass. The advisor's read of the final head. No separate verifier agent
  (part a's ruling 4, if you confirm it for part b).
- **Size:** estimate about 2,780 added lines (sheet D: range 2,400 to 3,300; at the units' past growth up to about
  4,950). The stop is 4,900 (`git diff --numstat 0e0f6547 -- spikes/netcode-proof/`, added). Counted after every
  block; cuts in this order if a block runs over: the detail text (values only); the kill-record check deferred by
  name; one plant per figure. At the stop: cut, and three lines to Mustafa.

## 4. Rulings asked of the advisor (the sheet's E, each with its lean there; decide, or say it is Mustafa's)
1. **Part a's head and entry 64's "before any other edit to crates/judge/src":** part a edited the judge's source
   without entry 64's items or JDG-ONLY-1 (my miss; the hand-off named JDG-ONLY-1). Lean: part a's head stands;
   both land first in part b, inside entry 64's "at the latest before U6 builds -19's figure" and JDG-ONLY-1's
   "before PR #40 is marked ready at the latest". Or must they be added to part a before it merges?
2. **The sheet's rulings, at their leans:** SIG (two-pass `Inputs`), REF-MIN (waiting), FAILNUM (`Lack::Fail`), ARITH
   (a detail text), TWIN-ROLE (A), C-U6-9's data-defined form, C-U6-11's per-session spans with `session_end`,
   KILL-EVENT (phase `after` plus the killed run present), CLIENT-ID (`room_end` carries `slot`), EGRESS-SPAN
   (x 3,600 as written, the real span printed), QWIN and HALF (by time), BULK-WIN (measured), IPV6-WHEN (always),
   ON-POOL-SET (this set), ONE-SIDED (counts).
3. **R2-INPUTS (Medium, the one I cannot lean on alone):** R2's formula has a soldier term and a vehicle term; SERIES.md
   names one count, `tick.s.relevant_count`, and the ADR 0006 row says "the run's P, V, shots and projectile spawns".
   Options in the sheet's A2 item 7. Do you rule, or does netcode-architect (one agent, before block 8)?
4. **C4-KILL (scope):** README 2.5 waits on a judge change "built at U6" (the `killed` exemption holds only if
   npt-super's log records that kill); the hand-off does not list it and no kill record is named yet. Lean: ask the
   netcode chat on the board for the record's name and fields; build it in part b if they exist by block 7, else
   defer it by name in the report.
5. **Two writer-side needs for the netcode chat** (information for it, posted on the board; SERIES.md rows in my
   fence): `room_end` carries `slot`; on L2 `admit` and `session_end` are in the room log. Lean: SERIES.md says so,
   and the report names both as things its later units must write.
6. **Rule::LoggingOff skips the every-run rules on its logging-off runs** (sheet A10 item 7, a Medium that was there
   before): fix it in block 9 (lean), or defer by name.
