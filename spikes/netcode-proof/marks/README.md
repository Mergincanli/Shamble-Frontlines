# marks/: every pass mark of the netcode proof test, as data

`marks.json` holds every mark the test judges (plan §6.2), one entry per mark, with its draft row. `npt-judge`
reads it together with the runs' results logs (npt-log's JSON Lines) and writes one verdict per mark (plan §6.1).
This file is the contract between the two: the format of `marks.json`, the results-log record kinds the checks read,
and the rules the judge applies. It changes only together with npt-judge and its fixtures.

## 1. The results log (npt-log, format version 1)
One file per process and run, JSON Lines (UTF-8, one record per line, `\n` line ends). The first line is the
header; tick, second and event records follow in time order. Every record has a kind, `k`.

| `k` | Fields | Meaning |
|---|---|---|
| `header` | `schema` (1), `run`, `set`, `role`, `marks`, `machine`, `build`, `bite`, `layer`, `profile`, `seed`, `tick_hz`, `clients`, `scene`, `process`, `slot`, `date`, `params` | one per file, first; see below |
| `tick` | `t` (tick number), `ph` (phase), `v` (name → integer), `s` (name → list of integers), `h` (state hash, 16 hex digits, optional) | one per simulated tick that the process measured |
| `second` | `sec` (seconds since the log's start), `ph`, `v`, `s` | one per second per process |
| `event` | `ev` (name), `ph`, `t` (tick, optional), `ns` (monotonic ns since the log's start, optional), `f` (name → JSON value) | anything that happens once |
| `footer` | `records` (the count of records before it), `ended` (`clean`, `crash` or `stopped`) | one per file, last; a file without it was cut short |

- **`header`:**
  - `run` names the run; `set` names the run set that is judged together; `role` is the run's part in its set:
    `measured` (the default), `bite` (the bites build, `bite` naming the planted defect), `clean` (a clean reference
    run), `bracket-first` and `bracket-last` (PM-0006-13's lone room), or any role a mark's checks name (`n24`,
    `logging-off`, `cubic`, …);
  - `marks` lists the marks this run serves; the judge reads a run only for those marks;
  - `machine`: `role` (`RS`, `LG`, `MV`, `C`, `ML`, `MP`, `MAC`), `name` (the instance: `RS`, `LG1`, `LG2`, …),
    `plan`, `arch`, `cpu`, `vcpus`, `mem_mb`, `location`, `kernel`, `address` (a VM's day address; empty elsewhere:
    §10.6), `threads_per_core`, `smt` (sibling lists), `clock_res_ns`;
  - `build`: `commit`, `binary_sha256`, `features`, `profile`;
  - `bite`: the bite id, or `null` in every measured build (plan §4.7);
  - `layer` (`L1`, `L2`, `real`), `profile` (`P0` to `P4`, `B1`, `real-path`), `seed`, `tick_hz`, `clients`,
    `scene` (`SS`, `ES`, `TC`, `CO`, …), `process` (`room`, `bots`, `load`, `lab`, `browser`, …), `slot`;
  - `date`: the run's date (`YYYY-MM-DD`), passed in by the caller (npt-log reads no clock);
  - `params`: name → JSON value, every other setting a mark needs from the header (for example `sim`, which is
    `stress` or `stand-in`; `padding_target_ns`; the queue depths of PM-0001-23 and -24; `cmd_timing`, `A` or `B`).
- **`ph`**, the phase of a tick, second or event record: `warmup`, `measured` or `after`.
- **Names in `v` and `s`** are lowercase with underscores, and end in their unit: `_ns`, `_bytes`, `_bits`,
  `_permille`, or `_count`, or a 0/1 flag with no suffix. `SERIES.md` lists every name a mark reads, with its unit and
  the crate that writes it.
- **Events the judge reads for validity** (§4): `admit` (`session`, `in_roster`), `control` (`source`: `socket` or
  `session`; `kind`; `applied`; `rostered`; `cap_valid` for session controls; `peer_ok` for socket controls),
  `affinity` (`when`: `start` or `end`; `cpu`; `alone`; `sibling_free`), `person` (`who`, `mark`, `verdict`:
  `pass` or `fail`, `steps`).
- **The writer** never runs on the tick thread: the tick thread hands fixed-size measurements to a writer thread,
  which builds the records (so a record's maps never allocate on the tick).

## 2. `marks.json`
```json
{ "schema": 1, "drafts_commit": "<the commit whose draft rows the row hashes were taken from>", "marks": [ … ] }
```

### 2.1 A mark
| Field | Type | Meaning |
|---|---|---|
| `id` | string | `PM-0001-4`, `PM-0004-67`, `PM-0006-14`, … |
| `title` | string | the row's second cell, shortened if long |
| `draft` | object | `file` (repository path of the ADR) and `row_sha256` (SHA-256 of the row's whole line, without its line end). The drift test finds the row by its id and fails if the hash differs |
| `verdict` | string | the mark's kind: `pass` (pass numbers, judged by script), `reported` (no pass number), `person` (judged by a person), `outside` (not in this change-set: always NOT-RUN with `notes`) |
| `where` | list of strings | where and when it runs (plan §6.2's key: `C`, `R1` to `R7`, `ML*`, `MAC-r`, `MAC-b`, `MP`); informative |
| `parts` | list of parts | the evidence the mark needs (2.2); empty for `outside` |
| `validity` | list of strings | the validity rules of §4 that apply to its runs, beyond the two that apply to every run |
| `checks` | list of checks | the pass numbers (2.3); empty for `reported`, `person` and `outside` |
| `report` | list of statistics | what is reported beside the verdict (2.4) |
| `bites` | list of objects | `id` and `what`: each planted defect that must fail the mark first (§5) |
| `stand_in` | bool | true only for PM-0001-4: a pass on a lighter sim is STAND-IN (header `params.sim` = `stand-in`) |
| `arm_rule` | bool | true for PM-0001-17, -19, -48 and -54 (ADR 0001's Arm rule) |
| `readings` | list of strings | the plan §15 readings that shaped the entry (`R-6`, …) |
| `notes` | string | anything the data cannot say: the setup in short, a proposed reading, what is not yet built |

### 2.2 Parts
A part selects runs and says how many valid runs it needs:
```json
{ "name": "L1 at P2", "select": { "layer": "L1", "profile": "P2" }, "min": 3 }
```
- `select` matches header fields: `layer`, `profile`, `role`, `scene`, `tick_hz`, `clients`, `machine.name`,
  `machine.arch`, `process`, or `params.<name>`. A run belongs to the part if every listed field is equal. `role`
  defaults to `measured`; bite runs are never in a part (§5).
- `min` is the number of valid runs the part needs (3 for a V-S mark; ADR 0001's V-S).

### 2.3 Checks
```json
{ "name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 16670000 }
{ "name": "0 overruns", "series": "tick.v.overrun", "stat": "sum", "op": "==", "value": 0 }
{ "name": "within 10 % of the lone room", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "factor": "1.10",
  "against": { "roles": ["bracket-first", "bracket-last"], "combine": "mean" } }
{ "name": "hits registered", "series": "event.shot.view_hit", "stat": "share", "where": { "op": "==", "value": 1 },
  "op": ">=", "value": "0.99", "parts": ["L1 at P2"] }
{ "name": "sync cost linear", "figure": "sync_cost_ratio", "op": "<=", "value": "1.1" }
```
- **`series`**, where the values come from:
  - `tick.v.<name>`: one value per tick record that has the name;
  - `tick.s.<name>`: every sample of every tick record, flattened;
  - `second.v.<name>`, `second.s.<name>`: the same per second;
  - `event.<ev>.<field>`: the field of every event `ev` (numbers, or booleans as 0 and 1);
  - `header.params.<name>`: one value.
- **`window`**: `measured` (the default: records whose `ph` is `measured`), `all`, or `warmup`.
- **`stat`**: `count`, `sum`, `min`, `max`, `mean`, `p50`, `p90`, `p99`, `p999`, or `share` (the share of values
  meeting `where`). Percentiles are by nearest rank: the ⌈q × n⌉-th smallest value, counted from 1 (plan §6.1;
  36,000 values give the 35,640th for p99). An empty series makes the check fail, never pass.
- **`op`**: `<=`, `<`, `==`, `>=`, `>`. **`value`**: an integer in the series' unit, or a decimal string
  (`"0.999"`, `"1.10"`), which the judge reads exactly, as a fraction.
- **`against`** compares with other runs of the same set: the same statistic of the same series in the runs whose
  role is in `roles`, combined by `mean` or `median`. The check passes if this run's statistic `op` `factor` × that
  reference. A missing reference run makes the part NOT-RUN.
- **`figure`** names a computation the judge carries in code (ratios across scale steps, slopes, arithmetic against
  a formula). The judge lists its figures; a test fails if `marks.json` names one it lacks.
- **`from`** (optional) picks the run's logs a check reads, by header fields as in `select` (for example
  `{ "machine.role": "LG" }` or `{ "process": "bots" }`). By default a check reads every log of the run; a run is all
  the logs whose header has the same `run`.
- **`parts`** (optional) limits a check to the named parts; by default a check applies to every part.
- **`pooled`** (optional, `true`): the check is computed once over the values of all the part's runs together
  (PM-0006-19's pooled p99), not run by run.

### 2.4 Reported statistics
`{ "name": "tick p50", "series": "tick.v.work_ns", "stat": "p50" }`, or `{ "name": …, "figure": … }`. They are
written beside the verdict and never decide it.

## 3. How a verdict is reached
1. The judge reads every results log under the given folder and keeps, per mark, the runs whose header lists it.
2. Each run is checked against the validity rules (§4). An invalid run is neither a pass nor a fail (ADR 0001's
   V-S).
3. Each valid run is checked against every check (pooled checks once per part). A run fails if any check fails.
4. Per part: any valid run that failed makes the part FAIL; else any invalid run makes it INVALID; else fewer than
   `min` valid runs make it NOT-RUN; otherwise it passes.
5. Per mark: any part FAIL, or a bite run that passed (§5), makes the mark FAIL; else any part INVALID, or an invalid
   bite run, makes it INVALID; else any part NOT-RUN, or a bite with no run, makes it NOT-RUN (the verdict names what
   is missing); else PASS, or STAND-IN when `stand_in` is true and a run had `params.sim` = `stand-in`.
6. A `reported` mark comes out REPORTED with its statistics when every part has its valid runs; INVALID and NOT-RUN
   work as above. A `person` mark comes out PASS or FAIL from its `person` event, or NOT-RUN. An `outside` mark is
   always NOT-RUN.

## 4. Validity rules
Three rules hold for every run of every mark (plan §6.1, security F1, F3, N11):
- **complete:** every log of the run ends with its `footer`, unless its header has `params.killed` true (a room the
  run kills on purpose, as PM-0001-27 does); a cut log makes the run INVALID;
- **roster:** any `admit` event with `in_roster` false makes the run INVALID;
- **capability:** a `control` event that was `applied` with `cap_valid` false (session) or `peer_ok` false
  (socket), or one from a `rostered` session with `cap_valid` false, makes the run INVALID. A control-type message
  before `Join` closes that session as malformed and is counted, never INVALID.

The others apply where a mark lists them:
- **`V-S`** (ADR 0001, "Runs and validity"): both `affinity` events (`start`, `end`) say `alone`; the steal share
  of the measured seconds (`second.v.steal_permille` in RS's logs) is at most 10 ‰ (1 %) over the run; LG's clients
  sent their 30 inputs per second each within 1 % (`second.s.inputs_count`, one value per client, against
  `tick_hz`); each LG machine's CPU stayed under 80 % in every measured second (`second.v.cpu_permille` < 800 in
  every log whose `machine.role` is `LG`);
- **`PM-0006-13`** (ADR 0006's density rules): the two bracket runs' p99s lie within 5 % of each other, and LG's
  80 % rule; steal is recorded and reported, never a condition;
- **`lg-80`:** LG's 80 % rule alone;
- **`padding`** (PM-0006-14): every measured tick's `tick.v.padded_step_ns` is at least the header's
  `params.padding_target_ns`; a run whose padding fell short on any tick is INVALID (the busy-wait never ends early,
  so a shorter tick is a fault);
- **`late-zero`** (PM-0001-30): every command ran at its stamped tick, and `tick.v.late_count` sums to 0;
- **`reorder`** (a mark that names reordering, ADR 0001 part 13's key): the run's measured reorder share,
  `header.params.reorder_permille` or the event `reorder`, is above 0;
- **`clean-ref`** (a mark compared with "a clean run"): its 3 clean runs' p99s lie within 2.5 % of their median.

## 5. Bites first
Each bite of a mark needs at least one run with role `bite` whose header names that bite and lists the mark. That run
must fail at least one of the mark's checks: the checks of the parts whose `select` it matches (its role aside), or,
if it matches none, the checks that name no `parts`. A bite run that passes every such check makes the mark FAIL
("the bite did not fail: the check cannot be shown to work"). A bite with no run makes the mark NOT-RUN, naming the
bite. Bite runs follow the same validity rules, apart from `min`.

## 6. Verdicts (plan §6.1)
PASS · FAIL · INVALID (a validity rule failed: neither a pass nor a fail, ADR 0001:1497-1498) · STAND-IN (PM-0001-4
on a lighter sim) · REPORTED (no pass number) · NOT-RUN. Every verdict carries its detail: per part, the runs and
each check's value against its threshold.
