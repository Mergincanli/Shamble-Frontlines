# docs-keeper, change-set 2's docs pass (PR #61): every line moved, copied or corrected in the shared record files

_Written 2026-11-19 11:18 UTC (date -u) by docs-keeper. Base: main b763ba3. Proof: `docs-keeper-move.mjs` (the moves, with
their checks) and `docs-keeper-blame-check.mjs` (every removed or changed line of the four files blamed on main:
this lane's, moved byte for byte, or a REC-OLD correction listed below; 0 others)._

## 1. Moved byte for byte by script (OLD-LINES: lines about merged pull requests), 9 lines in 2 blocks
- `docs/STATUS.md` line 29 (the row of #60, merged as b763ba3) -> `docs/STATUS_ARCHIVE.md` line 159, first row of
  "In flight" (sha256/16 6f5c161248a425d7 at both places). Then its own cells (this lane's line, written by #60's
  commits) were reshaped to the archive's four columns and updated to merged, as for #58's row: the PR cell dropped,
  "**the kit hold and two CI edits** (kit-hold-ci)" in bold, the follow-ups pointer changed from "the archive's
  'Follow-ups'" to "below", the merge cell added (b763ba3, 2026-11-18 20:30:46 UTC, his go of 20:20:09 UTC for
  e5ca735, CI green on main: gate run 900, toolchain run 198), "done: " plus what he accepted with the merge, waiting
  on: none.
- `docs/ROADMAP.md` lines 274-282 (a blank and STATUS-DIET's entry, the oldest of 13 in "Recent work") -> lines
  120-128 in "Build history", after kit-v3-5's folded entry (sha256/16 c3d7f2bec92a36bd at both places).

## 2. Copied by script (his words, never retyped), 1 quote
- His message of 2026-11-18, 20:44:29 UTC, "the zip saved and unpacked" (26 bytes, sha256 6616a05d...84585, checked),
  from `.checkpoints/cs2-first-page/mustafa-2026-11-18-zip-saved.md` into a new dated pointer of this lane in
  `docs/STATUS_ARCHIVE.md` ("Follow-ups", block "From change-set 1, Types (PR #58)", its last item), which closes
  the question STATUS's "Waiting on Mustafa" held.

## 3. Corrected under REC-OLD (a) (Mustafa, 2026-11-18, 20:20:09 UTC): rows of `docs/REPO_MAP.md` that describe code
that has changed, also rows another lane wrote; 8 rows, old and new text whole
The three his words name ("The three map rows for the files this pull request changed") are 3.6 to 3.8. The row of
`.claude/scripts/checkpoint.test.mjs` ("Its tests: `node --test .claude/scripts/checkpoint.test.mjs`") describes
only the command, which #60 did not change: left as it is. 3.1 to 3.5 are rows change-set 2's code made untrue.
Blame on main: Cargo.toml, package.json, packages/, .gitignore, checkpoint.mjs and toolchain.yml rows 550856a
(foundation-toolchain, #48); crates/ row 790ac31 (also #48); CHECKPOINTS.md row 9660197 (DOCS-54).

### 3.1 `Cargo.toml`
Old:

    | `Cargo.toml` | The v2 Rust workspace, a virtual one (ADR 0002 parts 2-4 and 7): members `crates/*`, resolver 3, edition 2024, `rust-version` equal to the pin, `publish = false`; `[workspace.dependencies]`, empty for now, where each shared version enters with the first crate that needs it; the workspace lints (`unsafe_code` denied, clippy's `all` at warn), which every crate opts into; overflow checks in every profile for every workspace crate (`[profile.release] overflow-checks = true`, dependencies at the release default, the register row OVERFLOW-ALL) |

New:

    | `Cargo.toml` | The v2 Rust workspace, a virtual one (ADR 0002 parts 2-4 and 7): members `crates/*`, resolver 3, edition 2024, `rust-version` equal to the pin, `publish = false`; `[workspace.dependencies]`, where each shared version enters with the first crate that needs it (so far ts-rs `=12.0.1` and wasm-bindgen `=0.2.130`); the workspace lints (`unsafe_code` denied, clippy's `all` at warn), which every crate opts into; overflow checks in every profile for every workspace crate (`[profile.release] overflow-checks = true`) and, by a named override each, for the six third-party crates compiled into our libraries and tests (cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared); `"*"` keeps the other dependencies (proc-macros, build scripts) at the release default (the register row OVERFLOW-ALL; a new third-party crate joins the list in the change that adds it) |

### 3.2 `crates/`
Old:

    | `crates/` | The v2 Rust crates (ADR 0002 part 3), each still an empty stub with one test: `sim` (the simulation: state, phases, tick, the navigation runtime), `protocol` (message types, the codec, decoders that never panic, the loopback transport), `room` (the room's match logic, with no I/O and no clock), `client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding, the aim cast, the block writer and its layout table), `sim-wasm` (the wasm-bindgen bindings the Worker loads; none until the boundary spike), `devtools` (the replay runner, hash comparer, tick bench, legacy-golden reader, headless protocol clients). The tests in `sim` and `protocol` pass only while overflow checks are on |

New:

    | `crates/` | The v2 Rust crates (ADR 0002 part 3): `sim` (the simulation: state, phases, tick, the navigation runtime; so far the world, its step and its state hash), `protocol` (message types, the codec, decoders that never panic, the loopback transport; so far the first eight message types), `room` (the room's match logic, with no I/O and no clock; still an empty stub with one test), `client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding, the aim cast, the block writer and its layout table; so far the layout table, the block writer and the local schedule), `sim-wasm` (the wasm-bindgen bindings the Worker loads, change-set 2), `devtools` (the replay runner, hash comparer, tick bench, legacy-golden reader, headless protocol clients; still an empty stub with one test). The tests in `sim` and `protocol` pass only while overflow checks are on |

### 3.3 `package.json`
Old:

    | `package.json` | The v2 npm workspaces (`packages/*`; ADR 0002 parts 2 and 3), private, exact versions only: TypeScript, Vitest and @playwright/test as dev dependencies, Node 22 in `engines`; no `"type"` field. The legacy toolkit keeps its own `tools/package.json` |

New:

    | `package.json` | The v2 npm workspaces (`packages/*`; ADR 0002 parts 2 and 3), private, exact versions only: TypeScript, Vitest, Vite and @playwright/test as dev dependencies, Node 22 in `engines`; no `"type"` field. The legacy toolkit keeps its own `tools/package.json` |

### 3.4 `packages/`
Old:

    | `packages/` | The v2 TypeScript packages (ADR 0002 part 3), each empty with one stub test (`npm test --workspaces`): `client` (`@shamble/client`, the player entry: input, camera, HUD, rendering), `sim-worker` (hosts `sim-wasm`, paces the tick, runs the local match), `net` (the browser's two transport backends, WebTransport and WebSocket), `types` (types generated from Rust), `e2e` (end-to-end tests with Playwright Test) |

New:

    | `packages/` | The v2 TypeScript packages (ADR 0002 part 3), each with its tests (`npm test --workspaces`, which CI's `toolchain` job runs: Vitest in `client`, `sim-worker`, `net` and `types`; Playwright's `node` project in `e2e`): `client` (`@shamble/client`, the player entry: the app shell, the overlay and rendering; change-set 2's page, below), `sim-worker` (hosts `sim-wasm`, paces the tick, runs the local match; below), `net` (the browser's two transport backends, WebTransport and WebSocket; still empty with one stub test), `types` (types generated from Rust), `e2e` (end-to-end tests with Playwright Test; below) |

### 3.5 `.gitignore`
Old:

    | `.gitignore` | node_modules; `.gate/` (generated output: renders, exports, builds); `dist/`, `*.log`, `.env*`, local Claude settings and agent memory; `/target/` (cargo's build folder) and `test-results/` (Playwright Test's output) |

New:

    | `.gitignore` | node_modules; `.gate/` (generated output: renders, exports, builds); `dist/`, `*.log`, `.env*`, local Claude settings and agent memory; `/target/` (cargo's build folder) and `test-results/` (Playwright Test's output); `packages/sim-worker/pkg/` (the Wasm and its glue, written by the build) and `packages/e2e/playwright-report/` |

### 3.6 `.claude/CHECKPOINTS.md`
Old:

    | `.claude/CHECKPOINTS.md` | The checkpoint system: never lose work to a usage limit. For every session: at most 3 agents at a time, workflow agents included (§4, §7); each agent writes its plan first and saves each block as it goes, and a restart trusts the blocks its notes mark done (§4); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/` (§4); the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); disk hygiene (§10); kept raw logs and the old build cache, removed only by `kept-logs.mjs` (§11) |

New:

    | `.claude/CHECKPOINTS.md` | The checkpoint system: never lose work to a usage limit. For every session: at most 3 agents at a time, workflow agents included (§4, §7); each agent writes its plan first and saves each block as it goes, and a restart trusts the blocks its notes mark done (§4); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/` (§4); the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); a dependency change held: the script never pushes a change to a manifest or a lockfile until `.checkpoints/<task-slug>/dependency-check.md` records security-auditor's answer on that content, and nothing that holds the change is built or pushed by hand meanwhile (§3, Mustafa's DEP-HOLD, PR #60); disk hygiene (§10); kept raw logs and the old build cache, removed only by `kept-logs.mjs` (§11) |

### 3.7 `.claude/scripts/checkpoint.mjs`
Old:

    | `.claude/scripts/checkpoint.mjs` | Commits and pushes `.checkpoints/` only: `save`, and `watch` (background saver) |

New:

    | `.claude/scripts/checkpoint.mjs` | Commits and pushes `.checkpoints/` only: `save`, and `watch` (background saver). Both hold a push that changes a manifest or a lockfile (`Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `npm-shrinkwrap.json`, in any folder) against the remote branch (or main's merge-base for a branch not pushed yet), unless `dependency-check.md` names each file's sha256 or the file equals `origin/main`'s; a held `save` commits locally and exits 1 (PR #60) |

### 3.8 `.github/workflows/toolchain.yml`
Old:

    | `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request: installs the pinned Rust toolchain with rustup, checks `rustc` against the pin, runs `cargo fetch --locked` and `npm ci` (the last steps with the network), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`, the release overflow tests of `sim` and `protocol`, the `wasm32-unknown-unknown` build of five crates, `npm ci --offline`, `npm test --workspaces`. Step 2.3 replaces it with the gate v2's workflow |

New:

    | `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request, with no cache restored or saved (`cache-mode: none`, PR #60): installs the pinned Rust toolchain with rustup, checks `rustc` against the pin, runs `cargo fetch --locked` and `npm ci` (the last steps with the network), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`; the generated types written again (`SHAMBLE_WRITE_TYPES=1`) and compared with git, failing on any change in `packages/types/src/generated`; the release overflow tests of `sim` and `protocol`, the `wasm32-unknown-unknown` build of five crates, `npm ci --offline`, `npm test --workspaces`; then `packages/types` type-checked with strict flags (`tsc --ignoreConfig`: the shipped files, then the shipped files and their tests with `--skipLibCheck`). Step 2.3 replaces it with the gate v2's workflow |

## 4. This lane's own lines, updated (RECORD-FILES: each pull request writes its own lines)
- `docs/STATUS.md`: line 5 (Last updated), 13 (Last merged: #60), 36-37 (Next, item 2's end), 40 (Waiting: his
  Brave check and go on #61, replacing #60's go and the zip question), 101-105 (Follow-ups: change-sets 0 and 1
  shortened, "the carried lines" now "the machine lines" since the SPLIT-25 pointer of 2cedb9c; #60's ids with
  his list for the quality gate plan; a pointer for #61). New: #61's row in "In flight".
- `docs/STATUS_ARCHIVE.md`: #60's block header in "Follow-ups" (in flight -> merged, with his words "The five
  Medium findings as follow-ups.", copied from his message's notes file and checked with grep -F; rewrapped). New:
  the dated pointer in change-set 0's block (SPLIT-25's pointer, 2cedb9c); the pointer in #58's block (section 2,
  plus: change-set 2 touches none of the three files with the two doc lines; the five change-set 2 lines its code has);
  the block "From change-set 2, the first page (PR #61)".
- `docs/ROADMAP.md`: #60's entry (title in flight -> MERGED; its last sentence "Next: ..." -> the merge line). New:
  change-set 2's entry, first under "Recent work" (12 entries after the fold).
- `docs/REPO_MAP.md`: the row of `crates/client-endpoint/src/layout.rs` (500757c, #58's docs pass; extended with
  `section`, `field` and `Section::align`). New: 21 rows (5 crate files, 15 package paths, the report's row).
