# U1's open questions, for the advisor (2026-11-06)

The main session's one request for rulings at U1's end. Three sources. Each item has the session's recommendation; the
ones the session settled itself as the owner of a format (no mark's number or meaning moves) are marked SETTLED, for
the record. Everything else asks for a ruling, or for escalation to Mustafa where it is his.

Committed state: branch `claude/netcode-proof-test-plan-hcky5g` at 5ac4042 (U1 part 2c). PR #41 approved at 1b4094a,
not merged yet (Mustafa's go). Files: `spikes/netcode-proof/marks/{README.md,marks.json,SERIES.md}`,
`docs/design/netcode-proof-messages.md`, `spikes/netcode-proof/crates/judge/`, `spikes/netcode-proof/check.mjs`,
`spikes/netcode-proof/ops/operator/redact-check.mjs`.

## A. The message catalogue (netcode-architect; `docs/design/netcode-proof-messages.md` §8)
- **A1 (its Q1), the overlay's status.** No room → client type carries the overlay's room-side values or the form's
  "saved". Options: (a) a 15th type `SessionStatus` (owner, capability holders only, on change, at most 1 a second);
  (b) a spike-only owner group in `Snapshot`. Recommendation: (a), so the measured snapshot path stays as the draft has
  it and bots never receive it. It adds a type plan §4.7 does not list (a plan-text change).
- **A2 (Q2), start-up settings.** OQ-0001-6's option and the congestion controller (PM-0001-24) are on neither control
  list. Recommendation: start-up settings in the room unit's configuration, recorded in every run header, never changed
  in a run.
- **A3 (Q3), timing Open (PM-0001-16).** npt-super may not depend on npt-proto. Recommendation: systemd's readiness
  notice (`Type=notify`, ready at Open), verified at U8.
- **A4 (Q4), "none" or `authority-only`.** ADR 0001's state table writes the never-sent class as "none" (line 638); the
  catalogue uses `authority-only`. Recommendation: the audit accepts only `authority-only`, and the close's ADR 0001
  results note the word (no draft edit now).
- **A5 (Q5), omitted rows' words.** SETTLED: the U1 audit checks Direction and Channel on built rows only (as the
  catalogue's §6 item 3 says).
- **A6 (Q6), fuzzing the sockets.** PM-0001-34 fuzzes "every message type of the catalogue". Recommendation: yes, the
  room-wide controls get one fuzz target (they are parsed panic-free, plan §10.2).
- **A7 (Q7), one capability per what.** Plan §10.2 says "a per-run capability"; the catalogue reads one per browser
  slot. Recommendation: per browser slot (it binds to one session, and a run can have several browser legs).
- **A8 (Q8), resync and the event queue.** A `FullState` has no event section, and a resync clears the client's
  unacknowledged events (the hit markers of that window are lost). Recommendation: confirm; PM-0001-22's "resynced"
  path rests on it.
- **A9 (Q9), the draft's `Input` limit.** ADR 0001:443-444 sizes it as command size × the cap, leaving no room for the
  header; the catalogue uses 6 + 8 × 20 = 166 B. Recommendation: a reading now ("plus the header"), written into ADR
  0001 at the close, or by a later R-PR if a run needs it first.
- **A10 (found by netcode-architect).** ADR 0001's doubled "The profile changes from the overlay": fixed in PR #41
  (1b4094a), approved. Done.

## B. marks.json (test-engineer; its notes `test-engineer-marks.md`, the registry `marks/SERIES.md`)
SETTLED by the session (format only; already in the committed README, judge and marks.json):
- B1 (its Q1): `select` takes `machine.role`; B2 (Q2): a per-log field matches a run when any one of its logs has it;
  B3 (Q3): the units `_um` and `_microdeg`; B4 (Q5, Q24): a report may name its parts or be pooled, and without parts
  it is computed in every part; B5 (Q6): `RD` is a machine role; `ML`/`ML*`, `MAC-r`/`MAC-b` as machine names.
- B6 (Q31): a bite run whose header matches no part has no checks to fail and comes out undecided (the mark NOT-RUN,
  never PASS). The run plans (U8) give each bite run the header of the part it bites. No judge change.

For a ruling (each a reading of a mark or a verdict rule; none moves a number):
- **B7 (Q4), validity per part.** Some rules hold for some parts only (-58's `reorder` at P3; -25's V-S and clean-ref
  for the flood parts, not the Origin parts). Recommendation: an optional `validity` on a part, beside the mark's.
- **B8 (Q7), preconditions.** -2's load is not chained into -4 to -27, so a run whose load fell short could pass -4.
  Recommendation: a rule `pre:PM-0001-2`: the run must pass -2's checks, else it is INVALID (it did not run the stress
  scene), listed on every mark -2 is a precondition of.
- **B9 (Q9), setup checks.** Setup conditions (-17, -23, -24, -48, -58, PM-0006-25, -27) are checks today, so a wrong
  setup FAILs. Recommendation: a check flag `setup: true` whose failure makes the run INVALID (it did not test the
  mark), as ADR 0001's V-S treats a run that broke its conditions.
- **B10 (Q8), bite sizes.** Rules B-S and B-N size the bite; nothing checks the size. Recommendation: the bite run's
  header carries `params.bite_size_ns` or `params.bite_pad_bytes`, reported beside the bite (not judged).
- **B11 (Q10-Q15, Q18-Q23, Q26-Q29), readings of single marks** (statistics the rows do not name): -3's jitter as
  both ends of the range and the uniform's p90, each within 5 % of the half jitter (Q10); -12's and -13's "per client"
  means as the largest client mean (Q11); -19's 100 joins as LG's, ML Chrome at least 1 (Q12); -25's IPv6 /64 part
  NOT-RUN without IPv6 ("recorded if it has not", Q13); -25's "closed 2 s after accept" as at most 2 s exactly, no
  tolerance (Q14); -27's "within 5 %" after the SIGKILL as not more than 5 % worse, and PM-0006-13's bite run serving
  -27 too (Q15); -28's control as a role `control` part with sum == 0 (Q18); -42 judged at P3 under both options, P2
  and P4 reported (Q19); -48's false-hit share over view misses (Q20); -49's pong-delayer cheat in the rule-cap runs,
  not a bite (Q21); PM-0004-67's jitter from the first 36,000 measured ticks of each S30, needing a first-N window
  (Q22); -56's item (1) needs -11 and -25 run with both WebTransport crates (Q23: the run plans must cover it);
  PM-0006-19 as the figure `logging_cost_ratio`, and its own validity rule (each "off" run's p99 within 1 % of the
  pooled "off" p99, Q26-Q27); PM-0006-18's numbers in the person event (Q28); PM-0006-11's sibling check excluding
  the unpinned density runs (Q29). Recommendation: approve as drafted; the session adds the first-N window (Q22) and
  the PM-0006-19 rule (Q27) to the judge with tests.
- **B12 (Q16), strings in the verdict.** Negotiated draft, browser versions, the certificate, the clock resolution
  are strings no statistic reads. Recommendation: a report may name `header.params.<name>` or `header.machine.<field>`
  with stat `value`, printing it beside the verdict.
- **B13 (Q17), -30's second bite expects INVALID.** Recommendation: a judge fixture, not a bite (a bite must FAIL).
- **B14 (Q25), the times-of-day rule's input.** Recommendation: `params.started_utc` in every header (the run's UTC
  start, from the caller), which redact-check passes.
- **B15 (Q30), per-client hashes per tick** (-17, -19, -58). Recommendation: a tick record field `hs` (name → list of
  16-hex-digit strings), read only by figures.

## C. The judge's and the tools' own readings (the main session's), for confirmation
- **C1, short padding (PM-0006-14):** a run is INVALID if any measured tick's padded step is under
  `params.padding_target_ns` (8,330,000), since the busy-wait never ends early; the p99 is reported. The row says "Each
  room's padded step time p99 is recorded, so a run whose padding fell short is invalid".
- **C2, a gap is INVALID:** a series a check needs with no values makes the run INVALID, never PASS or FAIL.
- **C3, V-S's clauses:** steal as the mean of RS's per-second steal permille over the measured seconds, at most 10;
  LG's inputs per client over the measured window (total ÷ seconds within 1 % of `tick_hz`), not second by second;
  LG's CPU under 800 permille in every measured second; the affinity read at start and end, alone and sibling-free.
- **C4, complete logs:** every log of a run ends with its footer, unless the header says `params.killed` (the
  SIGKILL'd room of -27); a cut log makes the run INVALID.
- **C5, verdict order:** a valid failing run, or a bite that passed, makes FAIL; else an invalid run INVALID; else a
  missing part, bite or figure NOT-RUN; every valid bite run must fail.
- **C6, token prefixes (redact-check, for U6 to mint):** `nptj_` join, `nptjr_` the runner's public join token
  (allowed), `nptc_` capability, `nptb_` bind secret; each the prefix plus at least 22 base64url characters.
- **C7, redact-check's allow-list:** loopback, unspecified, broadcast, the documentation ranges, the cloud metadata
  address (169.254.169.254, fe80::a9fe:a9fe) and Cloudflare's resolvers (1.1.1.1, 1.0.0.1); everything else needs
  the run's VM list or `--allow`.
- **C8, surfaces:** npt-log adds to plan §4.7's list the field types Machine, Build, Phase, Ended and Footer, and the
  reader (read_log, Log, ReadError); npt-judge is a library (marks, eval, ratio, figures) for its own binary and tests.
- **C9, files outside §4.1's layout:** `check.mjs` and `check.test.mjs` (the test command), `rustfmt.toml` (ADR 0002
  part 7's options), `marks/README.md` and `marks/SERIES.md`, `crates/judge/fixtures/`.
- **C10, the dependency rules** are read with every feature and target; dev dependencies do not count (they never
  ship), except for "nothing depends on npt-lab", which counts every kind.
- **C11, the no-reference grep** skips docs/, .checkpoints/ and Markdown (prose may name the folder). W-PR's workflow
  file (U8) will reference the spike and needs a named exception then.
- **C12, marks.json and SERIES.md were machine-copied** from the drafts (one node command, main folder, no cd), not
  retyped through the Write tool: an exception to the session's Write/Edit rule, for byte-exact data.
