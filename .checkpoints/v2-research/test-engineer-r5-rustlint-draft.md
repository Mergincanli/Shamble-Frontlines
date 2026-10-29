# R5 draft, question 3: clippy and rustfmt (R5-RUSTLINT)

Brief question (quoted): "clippy and rustfmt". Task scope: clippy lint groups and specific lints worth denying
(determinism, panics, float use), workspace-level lint tables, rustfmt settings. All sources read 2026-10-28.
Note: the clippy lint list (clippy.rust-lang.org, where rust-lang.github.io/rust-clippy now redirects) is outside the
allow list; each lint below was read in clippy's own source on raw.githubusercontent.com (master, 2026-10-28).

## Findings

- **R5-RUSTLINT-F1. Only clippy's `correctness` group is an error by default; `style` warns; `restriction` must never be
  turned on as a whole.** Source: https://doc.rust-lang.org/clippy/lints.html (Clippy book; no version) · official · read
  2026-10-28 · confirmed in the source: "The `clippy::correctness` group is the only lint group in Clippy whose lints are
  deny-by-default"; `style` is "the most opinionated warn-by-default group in Clippy"; for `restriction`: "It is **not**
  recommended to enable the whole group, but rather cherry-pick lints that are useful"; "Clippy will produce a warning if
  it finds a `#![warn(clippy::restriction)]` attribute in your code!"
- **R5-RUSTLINT-F2. `pedantic` is stricter and `nursery` holds lints still being worked on.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "The `clippy::pedantic` group makes Clippy even more
  _pedantic_."; "The `clippy::nursery` group contains lints which are buggy or need more work."
- **R5-RUSTLINT-F3. `Cargo.toml` sets lint levels per tool, with a priority that orders groups and single lints; it
  applies only to our own packages.** Source: https://doc.rust-lang.org/cargo/reference/manifest.html ("The `[lints]`
  section") · official · read 2026-10-28 · confirmed in the source: "Override the default level of lints from different
  tools by assigning them to a new level in a table"; example `[lints.rust] unsafe_code = "forbid"`; "`priority` is a
  signed integer that controls which lints or lint groups override other lint groups:"; "lower (particularly negative)
  numbers have lower priority, being overridden by higher numbers"; "Cargo only applies these to the current package and
  not to dependencies."
- **R5-RUSTLINT-F4. A workspace defines lints once and each crate opts in.** Source:
  https://doc.rust-lang.org/cargo/reference/workspaces.html ("The `lints` table") · official · read 2026-10-28 · confirmed
  in the source: "The `workspace.lints` table is where you define lint configuration to be inherited by members of a
  workspace."; members write `[lints]` with `workspace = true`; "Respected as of 1.74".
- **R5-RUSTLINT-F5. clippy reads a `clippy.toml` found from the crate's own folder upward.** Source:
  https://doc.rust-lang.org/clippy/configuration.html · official · read 2026-10-28 · confirmed in the source: "a TOML file
  named `clippy.toml` or `.clippy.toml`"; the search starts at `CLIPPY_CONF_DIR`, then `CARGO_MANIFEST_DIR`, then "The
  current directory."; "Clippy will walk up the directory tree, searching each parent directory until it finds one".
  That a `clippy.toml` inside `crates/sim/` applies to `sim` alone follows from `CARGO_MANIFEST_DIR` coming first
  (inferred).
- **R5-RUSTLINT-F6. A `deny` level makes clippy exit with an error, which is what CI needs.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "An error causes Clippy to exit with an error code, so is most
  useful in scripts used in CI/CD."
- **R5-RUSTLINT-F7. `disallowed_methods` bans named functions and methods, including std ones such as
  `std::time::Instant::now`.** Sources:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/disallowed_items.rs (source) and
  https://doc.rust-lang.org/clippy/lint_configuration.html (`disallowed-methods`) · official · read 2026-10-28 · confirmed
  in the source: "Denies the configured methods and functions in clippy.toml" (group `style`,
  `#[clippy::version = "1.49.0"]`); the examples include `{ path = "std::time::Instant::now" }` and `{ path =
  "std::vec::Vec::leak", reason = "no leaking memory" }`; the configuration is "The list of disallowed methods, written as
  fully qualified paths." with optional `reason` and `replacement`. Whether a primitive's method (`f32::sin`) can be named
  is not shown (unverified).
- **R5-RUSTLINT-F8. `disallowed_types` bans named types, such as std collections.** Sources: the same two pages ·
  official · read 2026-10-28 · confirmed in the source: "Denies the configured types in clippy.toml." (`style`,
  `1.55.0`); examples `"std::collections::BTreeMap"` and `{ path = "std::net::TcpListener" }`; "The list of disallowed
  types, written as fully qualified paths."
- **R5-RUSTLINT-F9. `iter_over_hash_type` flags `HashMap` and `HashSet` in `for` loops, and only there.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/iter_over_hash_type.rs · official
  (source) · read 2026-10-28 · confirmed in the source: "This is a restriction lint which prevents the use of hash types
  (i.e., `HashSet` and `HashMap`) in for loops."; "Because hash types are unordered, when iterated through such as in a
  `for` loop, the values are returned in an undefined order."; `#[clippy::version = "1.76.0"]`; the check starts with
  `if let Some(for_loop) = ForLoop::hir(expr)`, so an iterator chain such as `map.values().map(...).sum()` outside a
  `for` is not flagged (confirmed in the source lines).
- **R5-RUSTLINT-F10. Two nursery lints rewrite float expressions into `mul_add` and other methods.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/floating_point_arithmetic/mod.rs ·
  official (source) · read 2026-10-28 · confirmed in the source: `suboptimal_flops` "Looks for floating-point expressions
  that can be expressed using built-in methods to improve both accuracy and performance." (its example turns `a * 2.0 +
  4.0` into `a.mul_add(2.0, 4.0)`); `imprecise_flops` "Looks for floating-point expressions that can be expressed using
  built-in methods to improve accuracy at the cost of performance." (`powf(1.0 / 3.0)` into `cbrt()`); both `nursery`,
  `1.43.0`. Consequence (inferred): applying either changes rounding, so it changes state hashes and golden values, and
  `cbrt`, `ln_1p` and `exp_m1` are std transcendental calls R1-DET-REC2 bans in the sim.
- **R5-RUSTLINT-F11. `unwrap_used` and `expect_used` are restriction lints; clippy can exempt tests.** Sources:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/methods/mod.rs (read at offsets 0 and
  100000) and https://doc.rust-lang.org/clippy/lint_configuration.html · official · read 2026-10-28 · confirmed in the
  source: `unwrap_used` "Checks for `.unwrap()` or `.unwrap_err()` calls on `Result`s and `.unwrap()` call on
  `Option`s."; `expect_used` "Checks for `.expect()` or `.expect_err()` calls on `Result`s and `.expect()` call on
  `Option`s." (both `restriction`, `1.45.0`); `allow-unwrap-in-tests`: "Whether `unwrap` should be allowed in test
  functions or `#[cfg(test)]`" (default `false`), and the same for `expect`, `panic` and `indexing_slicing`.
- **R5-RUSTLINT-F12. `indexing_slicing` flags indexing that can panic.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/indexing_slicing.rs · official
  (source) · read 2026-10-28 · confirmed in the source: "Checks for usage of indexing or slicing that may panic at
  runtime."; "To avoid implicit panics from indexing and slicing." (`restriction`, "pre 1.29.0").
- **R5-RUSTLINT-F13. Overflow checks are on in dev builds and off in release builds by default; a profile can be
  overridden per package.** Source: https://doc.rust-lang.org/cargo/reference/profiles.html · official · read 2026-10-28
  · confirmed in the source: "When overflow-checks are enabled, a panic will occur on overflow."; `overflow-checks = true`
  under `[profile.dev]`, `false` under `[profile.release]`; "Profile settings can be overridden for specific packages and
  build-time crates."; "Overrides cannot specify the `panic`, `lto`, or `rpath` settings." That `overflow-checks` may be
  set per package follows from that list (inferred).
- **R5-RUSTLINT-F14. rustfmt reads `rustfmt.toml`; its edition, style edition, width and line-ending options are stable;
  import grouping is not.** Source: https://raw.githubusercontent.com/rust-lang/rustfmt/master/Configurations.md
  (master; the rendered page at rust-lang.github.io/rustfmt returned only a template) · official · read 2026-10-28 ·
  confirmed in the source: "You can create a TOML file called `rustfmt.toml` or `.rustfmt.toml`, place it in the
  project"; `style_edition` "Controls the edition of the [Rust Style Guide] to use for formatting ([RFC 3338])" (default
  "2015", stable); `edition` "Specifies which edition is used by the parser." (default "2015", stable); `max_width`
  "Maximum width of each line" (100, stable); `newline_style` "Unix or Windows line endings" (default "Auto", stable);
  `imports_granularity` and `group_imports` are not stable (tracking issues #4991, #5083); `unstable_features` "Enable
  unstable features on the unstable channel."
- **R5-RUSTLINT-F15. clippy adds lints release by release.** Source: the `disallowed_items.rs` source above · official ·
  read 2026-10-28 · confirmed in the source: `disallowed_fields` carries `#[clippy::version = "1.95.0"]` while
  `disallowed_methods` carries `1.49.0`. Consequence (inferred): a toolchain upgrade can add warnings, so under
  `-D warnings` the toolchain pin (R5-LAYOUT-F6) is what keeps the gate's verdict stable.

### Legacy facts used
- **KT13** (redesign): the netcode rule's principles for the Rust core: no wall-clock reads in the tick, seeded RNG,
  deterministic iteration (no HashMap order); "identical in Wasm and native".
- **TL13**: the legacy game reads `performance.now()` / `Date.now()` in 11 places (STATUS:111), which is why same-seed
  legacy matches differ; the lint must stop the same thing in the sim.
- **CS1**: the legacy loop is variable-step, though the docs called it deterministic (code contradicts docs).
- Wave 1 cited, not restated: R1-DET-REC2 (float rules; R5 picks the lint or build check that fails on a std
  transcendental call, `algebraic_*` or `relaxed-simd`), R1-ECS-REC2 mark 2 (a CI check finds zero std HashMap or
  HashSet iteration in the sim's outcome code; tooling is R5's), R1-RR-OQ3 (panics reachable from content and input),
  R1-RR-OQ4 (overflow differs between profiles; ADR-0002 with R5), R1-RR-OQ6 (unsafe for the output block),
  R2-TRN-REC4 (decoders never panic).

## Options

| Option | Shape | Catches | Cost and risk |
|---|---|---|---|
| A. Defaults plus `-D warnings` | clippy's default groups, every warning an error in the gate | correctness, suspicious, style, complexity, perf | nothing for determinism or panics |
| B. A. plus a workspace lint table and per-crate tiers | `[workspace.lints]` for all crates; a `clippy.toml` and extra `[lints]` in `sim`, `protocol`, `content` | determinism (F7–F9), panics in decoders and the content loader (F11, F12) | per-crate lint lists to keep; `restriction` picks need reviewed `#[allow]`s with reasons |
| C. B. plus `pedantic` everywhere | as B, plus pedantic | more style and edge cases (F2) | noise in tools and tests; under `-D warnings` every pedantic warning blocks |

For float use, a ban on all float arithmetic (a "no float" lint) does not fit: wave 1 keeps IEEE floats under rules
(R1-DET-REC7), so the lints target the banned functions, not floats.

## Recommendations

- **R5-RUSTLINT-REC1. One workspace lint table, every crate opts in, and the gate turns warnings into errors (option
  B).** `[workspace.lints.rust]`: `unsafe_code = "deny"` (an exception only in the module R1-RR-OQ6 may need, with a
  written invariant, a test and an `#[allow]` that gives its reason). `[workspace.lints.clippy]`: `all = { level =
  "warn", priority = -1 }`. The gate runs `cargo clippy --workspace --all-targets --locked -- -D warnings`, once for the
  host and once with `--target wasm32-unknown-unknown` for the crates that build for the browser. `pedantic` goes only on
  `sim`, `protocol` and `content`, at warn under the same `-D warnings`.
  Reason: only correctness denies by default (F1), so without `-D warnings` the rest only prints (F6); one table keeps
  the crates alike (F3, F4); pedantic is worth its noise in the outcome-deciding crates only (F2); `cfg` differences
  between targets mean one clippy run cannot see both builds (inferred).
  Rests on: R5-RUSTLINT-F1, F2, F3, F4, F6; R1-RR-OQ6 (cited).
  Serves: none directly (toolchain).
  A spike must prove (proposed): a planted correctness bug, a planted `style` warning and a planted `unsafe` block each
  fail the gate; a crate whose `Cargo.toml` lacks `[lints] workspace = true` fails a gate check of its own; clippy on the
  whole workspace takes under 3 min cold and under 30 s warm on the CI runner.
- **R5-RUSTLINT-REC2. The determinism lint set for `sim` (and for the parts of `content` the sim calls).** In
  `crates/sim/clippy.toml`: `disallowed-types` = `std::collections::HashMap`, `std::collections::HashSet` (replacement
  `BTreeMap` or an index-ordered `Vec`), `std::time::Instant`, `std::time::SystemTime`; `disallowed-methods` =
  `std::time::Instant::now`, `std::time::SystemTime::now`, std's transcendental float methods and the `algebraic_*`
  methods (if clippy can name them: OQ1), each with a `reason` that points to R1-DET-REC2 or KT13. In `sim`'s
  `[lints.clippy]`: `disallowed_types` and `disallowed_methods` at `deny`, `iter_over_hash_type = "deny"` as a backstop,
  `suboptimal_flops` and `imprecise_flops` explicitly `allow` with a comment, and never `cargo clippy --fix` on `sim`.
  The `relaxed-simd` target feature is not a lint: the gate reads the sim's build flags and fails if it appears.
  Reason: `iter_over_hash_type` alone misses iterator chains (F9), so the type ban is the main check for R1-ECS-REC2's
  mark 2 and the `for` lint only a second line; `Instant::now` is clippy's own example of a bannable function (F7) and is
  exactly KT13's wall-clock rule; the per-crate `clippy.toml` keeps the bans out of the server and tools, which need
  clocks (F5); the two float lints would rewrite sim maths into fused or banned calls (F10).
  Rests on: R5-RUSTLINT-F5, F7, F8, F9, F10; legacy KT13, TL13; R1-DET-REC2, R1-ECS-REC2 (cited).
  Serves: §6.2 (one sim on client and server), §6.6 and §6.8 (telemetry and moments computed from re-runnable matches).
  A spike must prove (proposed): each planted violation fails the gate on clippy, with the reason in the message: a
  `HashMap` field in a sim struct; `map.values().sum()` over a std `HashMap` (missed by F9's lint, caught by the type
  ban); `Instant::now()` in a tick function; `x.sin()` on an `f32` (if OQ1 says clippy cannot name it, the fallback
  check of OQ1 must catch it); the same code in `server` passes (the bans are per crate).
- **R5-RUSTLINT-REC3. No implicit panics in the code that reads untrusted bytes.** In `protocol` (decoders) and in
  `content`'s loader: `unwrap_used`, `expect_used` and `indexing_slicing` at `deny`, with `allow-unwrap-in-tests`,
  `allow-expect-in-tests` and `allow-indexing-slicing-in-tests` set to `true` in those crates' `clippy.toml`. The sim
  itself keeps them at warn under pedantic review, since its indices come from its own pools.
  Reason: wave 1 requires decoders that never panic and fuzzing with zero panics (R2-TRN-REC4, R1-RR-OQ3); these lints
  find the common panic sources at review time (F11, F12), and the fuzzing (R5-TEST-REC4) finds the rest; tests may
  unwrap (F11).
  Rests on: R5-RUSTLINT-F11, F12; R2-TRN-REC4, R1-RR-OQ3 (cited).
  Serves: §6.1 (rooms and private matches stay up when a client sends bad bytes).
  A spike must prove (proposed): a planted `.unwrap()` and a planted `buf[i]` in a decoder fail clippy; the same in a
  `#[test]` pass; R2-TRN-REC4's fuzz mark (10 million inputs per message type, 0 panics) holds.
- **R5-RUSTLINT-REC4. One overflow rule on every target: overflow checks on for `sim` and `protocol` in every profile,
  and explicit `wrapping_*` or `checked_*` for counters.** `[profile.release.package.sim] overflow-checks = true` (and the
  same for `protocol`); counters and tick numbers use explicit operations, as R1-DET-REC2 already asks.
  Reason: dev and release differ by default (F13), so a debug client and a release server can disagree or one can panic
  (R1-RR-OQ4); a per-package override is allowed for this setting (F13, inferred), so the rest of the workspace keeps the
  release default.
  Rests on: R5-RUSTLINT-F13; R1-RR-OQ4, R1-DET-REC2 (cited).
  Serves: §6.2.
  A spike must prove (proposed): a planted `u16` counter that overflows with `+` panics in the dev and the release test
  builds, native and Wasm alike; the same counter with `wrapping_add` gives the same value on all four; the tick cost of
  the checks is reported with method (R1-TICK's bench), and the ADR sets the limit.
- **R5-RUSTLINT-REC5. rustfmt with stable options only, and the gate in check mode.** `rustfmt.toml` at the root:
  `edition = "2024"`, `style_edition = "2024"`, `newline_style = "Unix"`; everything else default (`max_width` 100). No
  unstable options, so the formatter runs on the pinned stable toolchain. The gate runs `cargo fmt --all --check`.
  Reason: both editions default to 2015 when not set (F14), so writing them pins the style; `newline_style` carries
  KT19's LF rule for Mustafa's Windows machine; the import-grouping options are unstable (F14).
  Rests on: R5-RUSTLINT-F14; legacy KT19.
  Serves: none (toolchain).
  A spike must prove (proposed): a planted misformat and a file saved with CRLF endings each fail `cargo fmt --all
  --check`; the formatted tree is byte-identical on Windows and Linux.
- **R5-RUSTLINT-REC6. The toolchain pin includes clippy and rustfmt; a toolchain upgrade is its own change-set.** The
  change-set reports the new warnings and fixes them or allows them with reasons.
  Reason: lints arrive with releases (F15) and `-D warnings` turns them into failures; the pin (R5-LAYOUT-REC1) keeps
  the verdict stable between runs.
  Rests on: R5-RUSTLINT-F15; R5-LAYOUT-F6.
  Serves: none (toolchain).
  A spike must prove (proposed): CI prints `cargo clippy --version` and `rustfmt --version` equal to the pin; the same
  commit gives the same lint result on two CI runs a week apart.

## Open questions
- **R5-RUSTLINT-OQ1. Can `disallowed-methods` name std's float methods (`f32::sin`, `f64::atan2`, `f32::algebraic_add`)?**
  The read examples name only paths like `std::vec::Vec::leak` (F7). If it cannot, R1-DET-REC2's ban needs another
  check: a reviewed text or syntax scan of the sim crate for those method names (no tool for it was read). Decides it: the
  spike's planted `x.sin()`.
- **R5-RUSTLINT-OQ2. The `panic` and `arithmetic_side_effects` lints were not read.** Their source files were not at the
  paths tried (HTTP 404). They are candidates for REC3 and REC4. Decides it: reading their source or clippy's lint list.
- **R5-RUSTLINT-OQ3. Hash maps from dependencies.** REC2 bans std's `HashMap`; a map type from another crate (for
  example a hasher-backed map re-exported by a dependency) is not covered by F9's lint or the std paths. Decides it:
  rust-reviewer lists the map types of the sim's dependency tree and adds them to `disallowed-types`.
- **R5-RUSTLINT-OQ4. Overflow checks cost on the hot tick.** Not measured. Decides it: REC4's bench.
- **R5-RUSTLINT-OQ5. Allowed HashMap use inside the sim for non-outcome code.** R1-ECS-REC2 limits the ban to outcome
  code; REC2 bans the type crate-wide. Decides it: engine-architect, when the sim's modules exist; an `#[allow]` with a
  reason is the escape.

| Recommendation that hinges on a source not read | Source | Reason not read |
|---|---|---|
| REC2, REC3 (lint names and groups) | https://clippy.rust-lang.org/stable/index.html | outside the allow list (read in clippy's source instead) |
| REC3, REC4 (`panic`, `arithmetic_side_effects`) | their clippy source files | HTTP 404 at the paths tried; OQ2 |
| REC5 (rendered rustfmt options) | https://rust-lang.github.io/rustfmt/ | the page returns a template without content (read Configurations.md instead) |

## Sources read (R5-RUSTLINT)
S1 (workspaces, lints table), S29, S30, S31, S32, S33, S34, S35, S36, S37, S38, S39, S40 (notes file).
Not read: clippy.rust-lang.org (outside the allow list); arithmetic_side_effects.rs, disallowed_methods.rs (404);
rust-lang.github.io/rustfmt (template only).

## Cross-topic notes
- rust-reviewer: OQ1, OQ3, the unsafe exception (R1-RR-OQ6).
- engine-architect (ADR-0002): REC4's profile rule; OQ5.
