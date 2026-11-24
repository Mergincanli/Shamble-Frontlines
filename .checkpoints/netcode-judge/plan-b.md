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

### Block 2's design, fixed before its build (so a cut resumes on the same lines)
- **figures.rs:** `pub struct Inputs<'r, 'a> { run: &'r Run<'a>, set: &'r [&'r Run<'a>], parts: &'r Parts<'r, 'a> }`
  with `type Parts = BTreeMap<String, Vec<&Run>>` (part name to its valid runs); `Compute = fn(&Inputs) ->
  Result<Ratio, Lack>`. The 8 built functions keep their `(run, set)` signatures; the table calls them through
  non-capturing closures (`Some(|i| fate_log_twin_equal(i.run, i.set))`). `Lack::Fail(String)` maps to a new
  `Missing::Fail`. `Figure` gains `detail: Option<fn(&Inputs) -> Option<String>>` (`None` on the 8 and on `planned`);
  `compute(name, &Inputs)`.
- **eval.rs:** `Ctx` gains `parts: &Parts` and derives `Clone, Copy`; `judge_only` builds it with an empty map;
  `judge_mark` makes a first pass over the kept parts (members by `part_select`; valid = `validity(..)` holds and
  every setup check of the part passes, both evaluated with empty parts), then judges with `Ctx { parts: &valid,
  ..*ctx }`. `measure` takes the parts (a figure gets `Inputs`; `reference` passes none); `Rule::Pre` evaluates the
  precondition's checks with empty parts (they are another mark's). `eval_check`: `Missing::Fail(why)` gives
  `Outcome::Fail` with no value and the reason as its note; `fail_lines` prints such a line as
  `"{part}: run {id}: {check}: {reason} (needs {needs})"` in place of `= ?, needs`. `report_values`: a figure with a
  `detail` prints `"<value> (<detail>)"`.
- **Tests (engine.rs):** a test-only figure is not possible (the table is const), so block 2 is proven by all 100
  tests passing unchanged; `Lack::Fail`'s line and the detail text get their tests with their first users
  (`crash_close_max_ns` in block 9, the R2 formula in block 8), and `Inputs.parts` with `sync_cost_ratio` (block 7)
  and `logging_cost_ratio` (block 9). To honour condition 10 early, block 3's `alloc_twin_diff_count` does not need
  it, so the exact-line test lands with block 9 at the latest; if block 9 is cut, `Lack::Fail` goes with it.

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

## 5. The advisor agent's verdict (2026-11-24, about 21:35 UTC, on 80ffa68): APPROVE with 11 conditions
Each binds the build (my wording, shortened). Building on made-up logs before the writers exist is the right reading
of his U6-JUDGE (b) text; the SERIES.md wording for later writers stays inside "the rows your figures need".
1. Part a's head stands. Entry 64 (a) to (f) and JDG-ONLY-1 are block 1, before any other part-b edit to
   crates/judge/src. Until part b merges, the netcode chat keeps JDG-ONLY-1's interim rule (every `--only` naming a
   mark with `pre:PM-0001-2` also names PM-0001-2). The report records the ordering miss in one line.
2. Block 2 changes no existing test's expected result: all 98 pass unchanged before any figure is added.
   `Inputs.parts` holds only kept parts' valid runs (validity rules and setup checks held), never a moved part's.
3. R2-INPUTS goes to netcode-architect before block 8 (one agent, xhigh). Its brief: ADR 0006:1387 ("the run's P, V,
   shots and projectile spawns") governs PM-0006-17's check; R2's formula is E3 at docs/research/r2-netcode.md:
   1763-1765 (P players and bots, V vehicles, every entity in view); it changes no marks.json text or number; it may
   fix the meaning of `tick.s.relevant_count` or add one input row in SERIES.md as a row the figures need, with a
   board note for the writer; if the ADR row and SERIES.md:284-285 cannot both hold, back to the advisor first.
4. Figures that implement a reading still marked "(question)" or "proposed" (marks.json's notes for -12, -13, -15,
   -24, -27, PM-0006-19; SERIES.md:287's slope method) are built on the written reading, and the report lists them
   as readings still open for the readings change before R5, never as settled.
5. C-U6-11 states and tests two cases: a slot with no `admit` event keeps today's single span (a mid-run joiner
   with no admit record still fails coverage); a run with no `session_end` keeps today's verdict (a loss-burst gap
   stays INVALID).
6. KILL-EVENT at its lean (phase `after`, the killed run present in the set). SERIES.md's -27 row and the report
   state the writer-side need: the run script kills the room at the end of the measured window, so `after` begins
   at the kill. `kill_p99_ratio` is built last, after the logging-cost figure (his undecided R5 option could move it).
7. C4-KILL is built only if the netcode chat names npt-super's kill record by the time block 7 starts; else deferred
   by name in the report and on the card, home "before R5's -27 and PM-0006-22 runs"; the report says today's
   exemption of any `killed` log stays open.
8. The two writer-side SERIES.md changes (`room_end` gains `slot`; on L2 `admit` and `session_end` also in the room
   log) are posted on the board before block 4, worded as additions that keep every existing writer's rows valid, in
   their own rows (U6c's merge adds 51 rows to SERIES.md).
9. Rule::LoggingOff's skip of the every-run rules on logging-off runs is fixed in block 9, with a test and a planted
   defect; the report names it as a fix outside the U6j list.
10. `Lack::Fail`'s failure line prints its reason, and a test asserts the exact line (for `crash_close_max_ns`, the
    client with no `room_end`); today `fail_lines` prints only "= ?, needs …".
11. Per figure, the report says where its input series has no writer yet (16 series) and that it is proven on
    made-up logs only; the netcode chat's later units and R2, R3, R5 give the first real logs.
The six rulings: (1) part a stands; (2) the sheet's list all at their leans, with conditions 5, 6, 7 and 10;
(3) R2-INPUTS to netcode-architect; (4) C4-KILL as leaned; (5) the writer-side needs are inside the fence;
(6) Rule::LoggingOff fixed in block 9. Kit steps as part a's (one round, no separate verifier, the advisor reads the
final head; the size counts from 0e0f6547 against 4,900). So the build order's last two blocks become: block 7
`sync_unit_cost` and `sync_cost_ratio`; block 9 `crash_close_max_ns`, `logging_cost_ratio`, the LoggingOff fix, the
texts, then `kill_p99_ratio` last.
