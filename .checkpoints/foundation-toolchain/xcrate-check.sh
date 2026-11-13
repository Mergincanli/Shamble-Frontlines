#!/usr/bin/env bash
# Condition 7b of the advisor agent's ruling D1 (2026-11-13): the local cross-crate run on a given commit, applying
# nothing to its Cargo.toml (adapted from rust-reviewer's rust-reviewer-overflow-xcrate.sh, review round 1, RR-1).
# Usage, from the branch worktree:  bash .checkpoints/foundation-toolchain/xcrate-check.sh <commit> <empty abs dir>
# It copies the commit's Rust files, adds five sim functions (plain, #[inline], generic, Tick::next, #[inline(never)])
# and five room tests that call them with an overflowing value (each #[should_panic(expected = "attempt to add with
# overflow")]), a crate outside the workspace that room calls, and a wasm32 cdylib; then it reports: the test, release,
# bench and an inheriting custom profile; rustc's overflow flag per crate; and the wasm32 dev and release builds'
# behaviour in Node. Needs the pinned toolchain (1.99.1) and Node; no network (path crates only).
set -euo pipefail
commit=${1:?a commit}; out=${2:?an absolute path to an empty folder}
mkdir -p "$out/ws" "$out/ext/extdep/src"
git archive "$commit" rust-toolchain.toml Cargo.toml Cargo.lock crates | tar -x -C "$out/ws"
cd "$out/ws"
cat >> crates/sim/src/lib.rs <<'EOF'

/// Not inline, not generic.
pub fn add_plain(a: u8, b: u8) -> u8 {
    a + b
}

/// `#[inline]`.
#[inline]
pub fn add_inline(a: u8, b: u8) -> u8 {
    a + b
}

/// Generic over an unused type.
pub fn add_generic<T>(a: u8, b: u8, _t: T) -> u8 {
    a + b
}

/// A tick counter.
pub struct Tick(pub u32);

impl Tick {
    /// The next tick.
    #[inline]
    #[must_use]
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Never inlined: the control.
#[inline(never)]
pub fn add_never(a: u8, b: u8) -> u8 {
    a + b
}
EOF
cat > crates/room/src/lib.rs <<'EOF'
//! scratch: sim's arithmetic, and a crate outside the workspace, called from room.

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn plain_from_room() {
        let _ = sim::add_plain(black_box(u8::MAX), black_box(1));
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn inline_from_room() {
        let _ = sim::add_inline(black_box(u8::MAX), black_box(1));
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn generic_from_room() {
        let _ = sim::add_generic(black_box(u8::MAX), black_box(1), ());
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn tick_next_from_room() {
        let _ = black_box(sim::Tick(black_box(u32::MAX))).next();
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn never_inline_from_room() {
        let _ = sim::add_never(black_box(u8::MAX), black_box(1));
    }

    #[test]
    fn outside_crate_wraps_in_release_only() {
        // A crate outside the workspace keeps the release default ("*"): it wraps in release, panics in dev.
        let r = std::panic::catch_unwind(|| extdep::add(black_box(u8::MAX), black_box(1)));
        println!("extdep::add(255, 1) -> {r:?}");
    }
}
EOF
printf '\n[dependencies]\nsim = { path = "../sim" }\nextdep = { path = "../../../ext/extdep" }\n' >> crates/room/Cargo.toml
cat > ../ext/extdep/Cargo.toml <<'EOF'
[package]
name = "extdep"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]
EOF
printf 'pub fn add(a: u8, b: u8) -> u8 {\n    a + b\n}\n' > ../ext/extdep/src/lib.rs
mkdir -p crates/wasmcheck/src
cat > crates/wasmcheck/Cargo.toml <<'EOF'
[package]
name = "wasmcheck"
version = "0.0.0"
edition = "2024"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
sim = { path = "../sim" }
EOF
cat > crates/wasmcheck/src/lib.rs <<'EOF'
#[unsafe(no_mangle)]
pub extern "C" fn plain(a: u8, b: u8) -> u8 {
    sim::add_plain(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn inline(a: u8, b: u8) -> u8 {
    sim::add_inline(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn generic(a: u8, b: u8) -> u8 {
    sim::add_generic(a, b, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn tick(a: u32) -> u32 {
    sim::Tick(a).next().0
}
EOF
cat >> Cargo.toml <<'EOF'

[profile.relcustom]
inherits = "release"
EOF
export RUSTUP_AUTO_INSTALL=0
cargo generate-lockfile --offline -q
for prof in test release bench relcustom; do
  echo "== profile $prof"
  if [ "$prof" = test ]; then flag=""; else flag="--profile $prof"; fi
  cargo test $flag -p room --no-fail-fast -- --nocapture --test-threads=1 2>&1 | grep -E '^test tests|extdep::add' | sort || true
done
echo "== rustc's overflow flag per crate (release, -v, after a clean)"
cargo clean -q
cargo build --release -v -p room -p sim-wasm 2>&1 | grep -E 'Running `.*rustc --crate-name (sim|room|sim_wasm|extdep) ' \
  | while read -r l; do c=$(echo "$l" | sed -E 's/.*--crate-name ([a-z_]+) .*/\1/'); \
      if echo "$l" | grep -q 'overflow-checks=on'; then echo "$c: -C overflow-checks=on"; else echo "$c: no overflow flag"; fi; done | sort -u
echo "== wasm32 cdylib in Node: dev, then release (each call with an overflowing value)"
for p in dev release; do
  cargo build --profile "$p" -p wasmcheck --target wasm32-unknown-unknown -q
  dir=$([ "$p" = dev ] && echo debug || echo release)
  node -e '
    const fs = require("fs"); const p = process.argv[1];
    WebAssembly.instantiate(fs.readFileSync(p), {}).then(({ instance: i }) => {
      const e = i.exports; const calls = [["plain", 255, 1], ["inline", 255, 1], ["generic", 255, 1], ["tick", 4294967295]];
      for (const [n, ...a] of calls) { try { console.log(process.argv[2], n, "returns", e[n](...a)); } catch (x) { console.log(process.argv[2], n, "traps:", x.message); } }
    });' "target/wasm32-unknown-unknown/$dir/wasmcheck.wasm" "$p"
done
