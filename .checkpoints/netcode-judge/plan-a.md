# Part a's plan: MOVED (the judge and marks.json report a moved mark, or a moved part of one, as MOVED, never PASS)
2026-11-24, Netcode Builder 2. For the advisor agent's review; no judge or marks edit before its APPROVE.
Base: e2c0e896 (the netcode branch's head, this branch's first push). Paths below are under `spikes/netcode-proof/`.

## 1. Goal and sources
- **Goal (Mustafa's start block, 2026-11-24, part a):** the judge and marks.json report a moved pass mark, or a moved
  part of one, as MOVED and never as PASS; marks.json's text changes only as his MARK-NOTES answer allows.
- **Sources on the netcode branch, `.checkpoints/netcode-proof-test/`:** the design, replan-marks.md:193-214 (section 6);
  which marks and parts move, replan-marks.md:7-99 and moved-block.md (29 marks whole, 12 kept marks in part);
  MARK-NOTES, replan.md:106-111, his yes history.md:7595-7607 ("worded exactly as in replan.md's MARK-NOTES").
- **Change type:** logic (a judge rule) plus data (marks.json) and its contract text (marks/README.md). Risk tier 1.

## 2. What changes (files and symbols)
- **`crates/judge/src/marks.rs`** (about 90-130 lines with its unit tests):
  - new `pub struct Moved { to, why, by }` (serde, `deny_unknown_fields`; each of the three a non-empty string);
  - `moved: Option<Moved>` on `RawMark`, `RawPart`, `RawCheck`, `Bite`, and on the compiled `Mark`, `Part`, `Check`;
  - refused at load, each with its own message: an empty `to`, `why` or `by`; `moved` on an `outside` mark; `moved` on
    a mark and also on one of its parts, checks or bites (say it once); every part of a mark moved (move the mark);
    `moved` on a `report` entry; a precondition (`pre:<mark>`) that names a mark moved whole.
  - `SCHEMA` stays 1: the field is optional, and an older judge refuses the new file (unknown field), which is safe.
- **`crates/judge/src/eval.rs`** (about 90-140 lines):
  - `VerdictKind::Moved` ("MOVED"); `BiteStatus::Moved`; `moved: Option<Moved>` on `Verdict`, `PartOut`, `BiteOut`
    (left out of the JSON when absent); `moved_checks` on `Verdict` (name and where to);
  - `judge_mark`: a mark moved whole returns MOVED before any judging (person marks too), its line naming `to`, `why`,
    `by` and how many runs here list it ("counted, not judged"); `Kind::Outside` returns MOVED (section 5, question 2);
  - a moved part is never judged: its `PartOut` reads MOVED with no runs; a moved check is skipped everywhere checks
    are chosen (a part's runs, pooled checks, bite runs, a precondition's checks); a moved bite is never evaluated;
    `matched_parts` leaves moved parts out (bite runs and preconditions);
  - the headline order: FAIL, then INVALID, then NOT-RUN (kept parts and kept bites only), then MOVED when any part,
    check or bite is moved, then REPORTED, STAND-IN or PASS. So a kept failure, invalid run or missing run still
    shows, and nothing with a moved piece reads PASS, REPORTED or STAND-IN.
- **`crates/judge/src/lib.rs`**: the doc line that lists the verdicts gains MOVED. **`main.rs`**: no change.
- **`marks/marks.json`**, by a small script kept in the notes (`moved-table.mjs`: the table below as data; it parses,
  adds the fields and writes the file back; the file is exactly `JSON.stringify(x, null, 2)`, checked, so no other byte
  moves). sha256 before: c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9; after: in the report.
  - `moved` on 26 marks whole: PM-0001-25, -32, -33, -34, -35, -41, -42, -60, -43, -44, -45, -46, -61, -47, -48, -49,
    -50, -51, -52, -62, -53, -54, -55, PM-0006-15, -26, -27. The other 3 of the 29 (-39, -40, -63) are `outside`.
  - `moved` on parts of kept marks: -1 "RD", "MAC"; -3 "MP at P2"; -14 "MAC Safari"; -19 "ML Chrome"; -28 the six
    "wtransport: …" parts and "web-transport-quinn: MAC safari"; -29 "MAC-b safari"; -31 "ML Chrome over WebSocket at
    P2"; -36 "MAC-b safari loopback", "MAC-b safari eu-path"; PM-0006-14 "N=36", "N=18"; PM-0006-25 "MAC safari";
    and, by question 1, -19's and -31's "ML* Chrome (rough, reported)".
  - -58: `moved` on its check "0 corrections larger than 1 mm not traced to a late or lost command (PM-0001-41's
    count)" and on its bite `PM-0001-58/lt-tick-offset`.
  - -56 (MARK-NOTES 2; question 3): its 10-item check stays, marked moved; a new check "every kept item read once and
    written (4 items)", the same series and statistic, `>= 4`; its notes gain MARK-NOTES' sentence word for word.
  - -7 (MARK-NOTES 1): in its notes, the sentence about the history's size ("The history is sized for the largest cap
    the test uses … so -7 needs no re-run (plan §6.2).") is replaced by MARK-NOTES' sentence, word for word.
  - `to`, `why` and `by` come from replan-marks.md's table and moved-block.md; `by` is "NPT-TRIM, 2026-11-21 (the
    re-plan, approved 2026-11-22)". No pass number, operator, series or row hash changes; the report shows the diff.
- **`marks/README.md`** (about 25-40 lines): the `moved` field (2.1, 2.2, 2.3), how a verdict is reached (section 3),
  moved bites (section 5), MOVED in the verdict list (section 6), `outside` reading MOVED.
- **`crates/judge/fixtures/README.md`**: PM-0006-14's `base` row (NOT-RUN becomes MOVED). **`marks/SERIES.md`**: none.
- **Not touched:** every other crate, `check.mjs`, `ops/`, manifests and lockfiles (none needed).

## 3. Tests (made-up logs; `crates/judge/tests/engine.rs`, marks.rs's unit tests; about 300-450 lines)
1. A mark moved whole: all-pass runs read MOVED; failing runs read MOVED too (not judged); a moved person mark reads
   MOVED; the line names where it went.
2. A part moved: kept parts passing reads MOVED; a kept part failing reads FAIL; a kept part with a gap reads
   INVALID; a kept part unrun reads NOT-RUN; a moved part's own failing or gapped runs change nothing.
3. A check moved: its failing value and its gap change nothing; the rest passing reads MOVED.
4. A bite moved: unrun, it does not hold the mark at NOT-RUN; a run of it that would be "missed" does not fail.
5. MOVED outranks REPORTED and STAND-IN; `outside` reads MOVED.
6. A precondition skips the precondition mark's moved checks and moved parts.
7. Load refusals, each by its message: the six of section 2.
8. The property, on the real marks.json and the committed fixtures that read PASS today (PM-0001-4 `base`,
   PM-0006-11 `base`): `moved` added at every position in turn (the mark, each part, each check, each bite) never
   reads PASS; a position the loader refuses is skipped and counted, and the test fails if none was accepted.
9. The real marks.json: all 26 + 3 read MOVED on an empty log set; every kept mark with a moved piece cannot read
   PASS; PM-0006-14's `base` fixture reads MOVED (its test updated); the existing 85 tests still pass, except the
   three that assert `outside` or PM-0006-14 reading NOT-RUN, which change to MOVED (cli.rs, engine.rs, fixtures.rs).
10. Two planted defects, each run once and taken back, each failing a named test: the early return for a moved mark
    removed; MOVED ranked above FAIL. Their output goes in the report.
- **Not possible here:** the re-plan's real-data line (re-judge `results/u4f/b57221ed139d/pm-0001-47/`): the raw logs
  are not in the repository (only runs.txt, verdict.json and the sha256 list). The report gives the netcode chat the
  one command to run it at its merge.

## 4. Evidence, reviews, size
- **Checks on the final head, in this container (his JUDGE-CHECKS answer), with the exact commands in the report:**
  `cargo test -p npt-judge --locked`; `cargo clippy -p npt-judge --all-targets --locked -- -D warnings`; `cargo fmt
  --check`; then `node spikes/netcode-proof/check.mjs` whole (fmt, clippy, doc, test on both builds, deps, wasm, refs,
  marks, self, redact). GitHub's two checks (gate, toolchain) stay green on the pushed head. No workflow file.
- **Review, one round:** test-engineer and rust-reviewer at xhigh, both at once (2 agents), each reading every changed
  judge line and marks.json's diff; test-engineer runs the tests itself. One fix pass. Then the advisor's read of the
  final head, and the commit id to Mustafa. marks.json's sha256 before and after in the report's own line.
- **Size:** estimate 800-1,000 added lines (marks.rs 110, eval.rs 120, tests 380, README 35, marks.json about 280 as
  data at 5 lines per label); the stop is 1,440 (`git diff --numstat e2c0e896 -- spikes/netcode-proof/`, added).
- **Risks:** (1) his text in marks.json: only the two MARK-NOTES notes and the labels; the script's output diffed line
  by line. (2) Other code reading the verdict words: none in this tree compares them (grep of `ops/`, `check.mjs`);
  U8's run package is not merged here, so the netcode chat is asked on the board. (3) 12 kept marks can at best read
  MOVED from now on (a part of each moved): this is the re-plan's rule, said plainly in the report.

## 5. Questions for the advisor (each with my lean)
1. **The rough-laptop twin parts of -19 and -31.** Their "ML Chrome" parts move with the human run (R7). Each also has
   a part "ML* Chrome (rough, reported)" with `min: 1`, whose only run was the same R7 leg on the stand-in laptop. Left
   kept, it holds -19 and -31 at NOT-RUN for ever (R3 runs no browser). The table names only the "ML Chrome" part.
   Lean: the rough twin moves with it, same destination. Is that inside RE-PLAN-OK, or his to answer?
2. **The three `outside` marks (-39, -40, -63).** The re-plan maps `outside` to MOVED (one arm). Their notes end
   "Always NOT-RUN here.", which would then be wrong, and MARK-NOTES does not name those notes. Options: (a) map to
   MOVED and change that one sentence to "Always MOVED here." in the three notes; (b) map to MOVED and leave the notes;
   (c) leave `outside` reading NOT-RUN. Lean: (a) if you rule the sentence follows from the approved re-plan; else (c)
   now and (a) as a one-letter question to him.
3. **-56's form.** Keep the 10-item check as a moved check and add the 4-item check (lean: the old bar travels as
   data, and -56 reads MOVED, as moved-block lists it among the kept marks with moved parts), or replace 10 by 4.
4. **Kit steps for this unit** (it merges into the netcode branch, not main; PR #40 has its own close). Lean: no
   separate verifier agent (his block names one round of two reviewers; test-engineer runs the tests itself); the
   advisor reads the final head; the gate is GitHub's gate check on each push.
5. **The real-data line** (section 3, last bullet): skipped here, handed to the netcode chat. Lean: accept.
