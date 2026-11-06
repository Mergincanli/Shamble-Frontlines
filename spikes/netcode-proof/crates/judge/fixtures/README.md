# npt-judge's fixtures: fixed results logs that must come out PASS, FAIL and INVALID (plan §6.1)

Each mark has a `base/` folder of results logs (npt-log's format, `marks/README.md` §1) and one folder per case that
holds only the files it changes: a case file replaces the base file of the same name. `tests/fixtures.rs` judges
each case with the real `marks/marks.json` and checks the verdict and its reason. Addresses are documentation ones
(RFC 5737), so redact-check passes them.

| Mark | Case | What changes | Verdict |
|---|---|---|---|
| PM-0001-4 | `base` | three S30 runs at P2 (room on RS, bots on LG), all under 16.67 ms, V-S held; the B-S bite run fails | PASS |
| PM-0001-4 | `fail` | run 2's slowest measured tick is 16.68 ms, so its p99 (the 3rd of 3) is over 16.67 ms | FAIL |
| PM-0001-4 | `invalid` | run 3's RS steal averages 11.5 ‰, over V-S's 1 % | INVALID |
| PM-0006-14 | `base` | the N = 48 session: both bracket runs and one count run padded to 8.33 ms on every tick; N = 36 and 18 not run | NOT-RUN |
| PM-0006-14 | `short` | one measured tick of the count run padded to 8.329 ms, short of 8.33 ms | INVALID |

PM-0006-14 has no pass number, so it is never PASS or FAIL: with all three N run and padded right it is REPORTED
(the engine tests cover that path); its fixture proves that short padding comes out INVALID (ADR 0006's PM-0006-14 row:
"npt-judge's fixtures prove that short padding comes out INVALID").
