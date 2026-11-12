#!/usr/bin/env bash
# rust-reviewer, review round 1 of PR #48: reproduces RR-1 (sim's release overflow override does not reach sim code
# that another crate code-generates). Usage, from the branch worktree:
#   bash .checkpoints/foundation-toolchain/rust-reviewer-overflow-xcrate.sh /abs/path/to/empty/dir
# Needs: the pinned toolchain (1.99.1) installed; no network (path crates only).
set -euo pipefail
out=${1:?an absolute path to an empty folder}
head=4d78ad61e652f2e35ddf51c49557e725420248c8
mkdir -p "$out"
git archive "$head" rust-toolchain.toml Cargo.toml Cargo.lock crates | tar -x -C "$out"
cp "$out/Cargo.toml" "$out/Cargo.toml.orig"

cat >> "$out/crates/sim/src/lib.rs" <<'EOF'

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

cat > "$out/crates/room/src/lib.rs" <<'EOF'
//! scratch: sim's arithmetic called from another crate.

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
}
EOF
printf '\n[dependencies]\nsim = { path = "../sim" }\n' >> "$out/crates/room/Cargo.toml"

cd "$out"
cargo generate-lockfile --offline
echo "== test profile (expected: 5 of 5 panic)"
cargo test -p room --no-fail-fast 2>&1 | grep -E '^test tests' || true
echo "== release, the head's per-package overrides (seen 2026-11-12: only never_inline panics; 4 FAILED)"
cargo test --release -p room --no-fail-fast 2>&1 | grep -E '^test tests' || true

# Fix candidate A: every workspace crate checks overflow in release; non-members keep their default.
python3 - "$out/Cargo.toml" <<'EOF'
import sys
p = sys.argv[1]
s = open(p + '.orig').read()
s = s[:s.index('# Overflow checks')] + '[profile.release]\noverflow-checks = true\n\n[profile.release.package."*"]\noverflow-checks = false\n'
open(p, 'w').write(s)
EOF
echo "== release, fix candidate A (seen 2026-11-12: 5 of 5 panic)"
cargo test --release -p room --no-fail-fast 2>&1 | grep -E '^test tests' || true
cp "$out/Cargo.toml.orig" "$out/Cargo.toml"
