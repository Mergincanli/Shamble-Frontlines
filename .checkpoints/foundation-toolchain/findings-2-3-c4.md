# Foundation 2.2: items added by the Fable advisor's full skeleton read (consultation 4, 2026-11-12), and the checks run

To merge into the report's lists and the archive's Follow-ups after the docs workflow (wf_29ac478b-0a7) is committed.

## Checked now
- 2.3's lint commands on the stubs: `cargo fmt --all --check` exit 0; `cargo clippy --workspace --all-targets --locked --
  -D warnings` clean on the host and on wasm32; with `-W clippy::pedantic` on `sim` and `protocol`:
  `should_panic_without_expect` on both overflow tests (fixed next: `expected = "attempt to add with overflow"`).
- The overflow tests pass under the `bench`, `test` and `dev` profiles locally (bench inherits release's package
  overrides); CI proves `release`.
- `.gitattributes` exists: `* text=auto eol=lf` (LF in the repo and the working tree), binaries marked.
- crates.io names (HTTP status of `https://crates.io/api/v1/crates/<name>`, 2026-11-12): `sim` 200 (0.13.1),
  `protocol` 200 (3.4.0), `room` 200 (0.0.0), `devtools` 200 (0.3.5), `client-endpoint` 404, `sim-wasm` 404.
- setup-node in `gate.yml`: the same inputs and no cache restored, on main (run 43226692719, gate and smoke jobs) and
  on a5823cf (run 43282089533): the root `package.json` changes nothing there today.
- Node resolution: with the root installed and `tools/` installed `--omit=optional` (the gate job's form),
  `require('playwright')` from `tools/smoke.mjs` resolves to the root's `node_modules/playwright` 1.65.0, not
  "Playwright is not installed"; CI's smoke job installs `tools/` in full (1.56.1 found first) and never installs the
  root; a machine set up per CLAUDE.md (`cd tools && npm ci`) finds the toolkit's own copy first.

## For the report's Risks
- **TOOLKIT-RESOLVES-UP**: the case above; low reach (only a `tools/` install without optional packages plus a root
  install); `tools/smoke.mjs` is a gate file, so a fix (for example checking that the resolved Playwright lies inside
  `tools/`) is 2.3's.
- The cut proves hermetic tests, not a sandbox: the steps after it run with the network, and it shares the process and
  mount namespaces (security-auditor's brief).

## For 2.3 (findings)
- **CRATE-NAMES**: four workspace crate names are taken on crates.io by other crates (`sim`, `protocol`, `room`,
  `devtools`): a dependency written without its `path`, or `cargo add sim`, would pull a stranger's crate; G5
  (cargo-deny) can ban those names from the registry (rust-reviewer's review asked).
- **CI-ENFORCED-LATER**: nothing in CI yet checks that a new crate opts into the workspace lint table, that a new crate
  has a test (cargo passes with 0 tests where npm fails), or that npm versions stay exact (E7 and E11's script goes with
  the notes).
- **WASM-ARTIFACT**: `sim-wasm` is a plain library, so the wasm32 step compiles the crates but emits no `.wasm` file;
  its crate type and its `unsafe_code` exception come with the bindings; the wasm32 build is dev-profile only (joins
  OVERFLOW-WASM).
- **TWO-PLAYWRIGHTS**: two Playwright versions now live in the repository (the toolkit's 1.56.1, the root's 1.65.0), so
  2.3's browser cache needs a key per version; no real browser has run inside the cut's privilege drop yet.
- **CARGO-NET-OFFLINE**: `CARGO_NET_OFFLINE=true` inside the cut would give fast, clear failures once third-party crates
  exist (a CI-job edit: it asks him).
- **CRATE-LICENCE**: the crates carry no `license` field, which cargo-deny's licence check reads.

## Report fix
- The kit table names line numbers from two trees: say which tree each is on (main 2aa5e9c before the edits; the
  branch after them: K3 at 22-23, K2a at 48, K2b at 55-57).
