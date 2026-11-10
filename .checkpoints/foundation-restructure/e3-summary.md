# foundation-restructure 2.1a: E3, the mutation proof (2026-11-10, 11:43-13:2x Amsterdam)

Run: `node .checkpoints/foundation-restructure/e3-run.mjs` over `e3-mutations.json` (31 entries, drafted by an Opus
agent from `docs/reports/2026-10-13-phase-0.md:23-27` and `docs/reports/2026-10-14-gate-check-5.md:28-41`; every
`find` unique at 44446a5). Each entry planted in three throwaway worktrees built from commits, the full gate run with
the smoke against base 44446a5 (`--json`), the failing check ids and exit code recorded, the file restored (all three
worktrees clean after the run). Raw results: `e3-results.json`; log `.gate/e3-run1.log`.
- **old** = `.gate/fdn-old` at 44446a5: OLD's own gate, files at the root
- **new** = `.gate/fdn-new` at dd12aeb: the edited gate, files at the root
- **newleg** = `.gate/fdn-newleg` at 9af08f7: the edited gate, files in `legacy/` (dd12aeb plus a local commit of the move)

**Result: 31 of 31 PASS.** For every mutation the three failing sets are equal and not empty; every control exits 0 on
all three (PC1 judged on check 3: the Phase 0 gate had checks 1-4 only, and `refreshRect` is still called at boot).

| Id | What | Report's checks | Failing on old = new = newleg | Exit |
|---|---|---|---|---|
| P1 | `refreshRect` deleted | 3 | 3, 5a, 5b ("DROPPED: refreshRect") | 1 |
| P2 | `bootEditor();` in `bootGame` | 4 | 4, 5a ("FLAGGED bootEditor ... in bootGame") | 1 |
| P3 | a syntax error | 1 | 1, 2, 3, 4, 5a, 5b (shamble-game.js line 26) | 1 |
| P4 | `wrType` copied into core | 3 | 3, 5c ("CROSS-FILE DUPLICATES: wrType (core, game)") | 1 |
| M1-M5 | TDZ; throw in deploy; movement dead; a WebSocket; a file read outside | 5a | 5a | 1 |
| M6 | 2026-10-14's editor file (5858864) | 5a, 5b | 3, 5a, 5b | 1 |
| M7 | `refreshMapList` re-nested | 5a, 5b | 3, 5a, 5b | 1 |
| M8, M9 | core/game tags swapped; `defer` on game | 5a, 5d | 5a, 5d | 1 |
| M10-M18 | handler names, typos, implicit global, bare `localStorage`/`status`, CapsuleGeometry, const writes | 5b | 5b | 1 |
| M19 | unquoted `onclick=noSuchFn()` | 5b + 5d (with M20) | 5b | 1 |
| M20 | unquoted `<script src=shamble-editor.js>` | 5b + 5d (with M19) | 5d | 1 |
| M21 | core calls game | 5c | 5c | 1 |
| M22 | `type="text/ecmascript"` editor script | 5d | 5d | 1 |
| C1-C3 | valid handlers; commented tag etc.; endless animation | none | none | 0 |
| PC1 | P1 + `--allow-drop refreshRect` | none (checks 1-4) | 5a, 5b; check 3 PASS ("dropped (allowed by --allow-drop)") | 1 |
| PC2 | a pure move (`_aLerp` game → core) | none | none ("moved: _aLerp (game -> core)") | 0 |

Notes:
- The Phase 0 entries fail on the report's named check, for the report's reason, on all three gates; the extra failing
  checks (5a, 5b, 5c...) are checks that did not exist in Phase 0 (the base is today's main, not the reports').
- M6 and M7 also fail check 3 because on today's base the editor's three functions are top level (they were nested in
  the 2026-10-14 base): predicted by the agent before the run.
- M19 and M20: the report lists them together under "5b + 5d"; run one at a time, M19 fails 5b and M20 fails 5d, the
  same on all three gates. A note on the report's grouping, not a fail.
- Local smoke on Chromium 1243 through `.gate/pw-browsers` on all three columns (the comparison is like for like); CI's
  run 42801926823 on 9ae4037 (dd12aeb inside) ran gate and smoke with the pinned Chromium: both success.
