# spikes/netcode-proof: the netcode proof test (throwaway)

The netcode proof test of `docs/SPIKE_PLAN.md` item 1: a throwaway spike (SPIKE-SCOPE (a)), exempt from the legacy
hard constraints, and kept on main afterwards for the 60 Hz margin run and the boundary spike (plan §4.9). Nothing
outside `spikes/` may reference it.

- **The plan** (approved 2026-11-06 by the advisor and Mustafa) lives in `.checkpoints/netcode-proof-test/plan.md`
  while the change-set runs, and is folded into its report at the close.
- **Layout** (plan §4.1): one Cargo workspace pinned to Rust 1.99.1 (`rust-toolchain.toml`), 15 crates under
  `crates/`, the client harness under `web/`, the run package under `ops/`, the marks as data in `marks/marks.json`,
  committed verdicts under `results/`.
- **Dependency rules** (plan §4.2): the leaf crates (npt-link, npt-l1, npt-sim, npt-proto, npt-room, npt-client,
  npt-log) build for wasm32 and depend on no transport crate or async runtime; npt-client never depends on npt-net;
  nothing depends on npt-lab.

## Commands (run from this folder; the toolchain file picks Rust 1.99.1)
- **The test command, every unit:** `node check.mjs` (fmt, clippy host and wasm32 with warnings as errors, the tests,
  the dependency rules of plan §4.2 from `cargo metadata`, the no-reference grep, its own tests, redact-check);
  `--only <step,...>` runs some
- Build and test: `cargo build --locked --workspace` and `cargo test --locked --workspace`
- Lints, host and wasm32: `cargo clippy --locked --workspace --all-targets` and
  `cargo clippy --locked --target wasm32-unknown-unknown -p npt-link -p npt-l1 -p npt-sim -p npt-proto -p npt-room -p npt-client -p npt-log`
- Format check: `cargo fmt --all --check` (rustfmt.toml: ADR 0002 part 7's stable options)
- Judge: `cargo run --locked -p npt-judge -- --marks marks/marks.json --logs <folder> [--out verdicts.json]`;
  `marks/README.md` is the contract between the results log, `marks.json` and the judge
- redact-check (plan §10.6; Node 22, also on Windows): `node ops/operator/redact-check.mjs <file or folder>...`,
  `--staged` before a commit, `--pre-push` as a git hook; its tests: `node --test ops/operator/redact-check.test.mjs`

Status: unit U1, second part (npt-log, npt-judge, redact-check, the test command and its dependency checks, the
message catalogue `docs/design/netcode-proof-messages.md` with its class audit test; marks.json next). Each unit's
state is in `.checkpoints/netcode-proof-test.md`.
