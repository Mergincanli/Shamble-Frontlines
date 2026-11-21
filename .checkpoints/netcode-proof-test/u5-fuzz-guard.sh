#!/usr/bin/env bash
# U5's cargo-fuzz guard (the advisor's SEC-U5D-4 ruling, advisor-u5-early.md:316-338, condition S2; the dependency read,
# security-u5-deps.md:167-184). Every build-time cargo-fuzz call runs as the command of a u5-run.sh call, never directly:
#   bash u5-run.sh [-x] <worktree> <log> bash <this guard> <worktree> <build|run> <cargo-fuzz args...>
# Before the call it checks: fuzz/Cargo.toml's and fuzz/Cargo.lock's sha256 equal dependency-check.md's lines (read from
# the file, never retyped); `cargo +1.99.1 tree --locked --offline` resolves the fuzz manifest; CUSTOM_LIBFUZZER_PATH,
# CUSTOM_LIBFUZZER_STD_CXX, RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS are unset; the subcommand is build or run; before `--`
# no refused option (--build-std, -Zbuild-std, --careful, --target-dir, a sanitizer other than address or none);
# CARGO_TARGET_DIR is the shared target. Then it runs `cargo +nightly-2026-11-01 fuzz <subcommand> <args...>` from the
# copy's spikes/netcode-proof and hashes both files again.
# Plant test (shown once before the first cargo-fuzz call, inside one exclusive runner call):
#   bash u5-run.sh -x <worktree> <log> bash <this guard> --plant-test <worktree>
# appends one byte to fuzz/Cargo.toml, expects the hash check to refuse, restores the file (cp -p, then mv) and checks
# both hashes equal dependency-check.md's again; it runs no cargo-fuzz.
# Exit codes: cargo-fuzz's own after a passed check; 64 a refused call (nothing ran); 65 a hash mismatch after the call
# (its outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor); 66 a failed
# plant test (not refused, or not restored).
set -u
M=/home/user/Shamble-Frontlines
DC=$M/.checkpoints/netcode-proof-test/dependency-check.md
T=$M/.gate/shared-target
NIGHTLY=nightly-2026-11-01
refuse() { echo "GUARD REFUSED: $*"; exit 64; }

plant=0
if [ "${1:-}" = "--plant-test" ]; then
  plant=1
  shift
fi
W="${1:-}"
case "$W" in
  "$M"/.claude/worktrees/*) ;;
  *) refuse "the worktree must be under $M/.claude/worktrees/ (got: $W)" ;;
esac
shift
S=$W/spikes/netcode-proof
F=$S/fuzz
[ -f "$F/Cargo.toml" ] && [ -f "$F/Cargo.lock" ] || refuse "no fuzz/Cargo.toml or fuzz/Cargo.lock under $S"
[ -f "$DC" ] || refuse "no $DC"

want_toml=$(grep -E '^[0-9a-f]{64}  spikes/netcode-proof/fuzz/Cargo\.toml$' "$DC" | cut -d' ' -f1)
want_lock=$(grep -E '^[0-9a-f]{64}  spikes/netcode-proof/fuzz/Cargo\.lock$' "$DC" | cut -d' ' -f1)
[ "$(printf '%s\n' "$want_toml" | grep -c .)" = 1 ] || refuse "dependency-check.md must hold exactly one fuzz/Cargo.toml line"
[ "$(printf '%s\n' "$want_lock" | grep -c .)" = 1 ] || refuse "dependency-check.md must hold exactly one fuzz/Cargo.lock line"

hashes() {
  have_toml=$(sha256sum "$F/Cargo.toml" | cut -d' ' -f1)
  have_lock=$(sha256sum "$F/Cargo.lock" | cut -d' ' -f1)
}
hashes_match() {
  hashes
  [ "$have_toml" = "$want_toml" ] && [ "$have_lock" = "$want_lock" ]
}

if [ $plant -eq 1 ]; then
  hashes_match || refuse "plant test needs a clean start: fuzz/Cargo.toml $have_toml, fuzz/Cargo.lock $have_lock"
  backup=$(mktemp "$M/.gate/u5/guard-plant.XXXXXX") || refuse "cannot make the backup"
  cp -p "$F/Cargo.toml" "$backup" || refuse "cannot back up fuzz/Cargo.toml"
  printf ' ' >> "$F/Cargo.toml"
  if hashes_match; then planted="NOT REFUSED"; else planted="REFUSED"; fi
  echo "plant: fuzz/Cargo.toml with one byte appended is $have_toml; the hash check $planted it"
  mv -f "$backup" "$F/Cargo.toml"
  if hashes_match; then restored="restored"; else restored="NOT RESTORED"; fi
  echo "after restore: fuzz/Cargo.toml $have_toml, fuzz/Cargo.lock $have_lock: $restored"
  if [ "$planted" = "REFUSED" ] && [ "$restored" = "restored" ]; then
    echo "PLANT TEST PASSED"
    exit 0
  fi
  echo "PLANT TEST FAILED"
  exit 66
fi

[ $# -ge 1 ] || refuse "usage: bash u5-fuzz-guard.sh <worktree> <build|run> <cargo-fuzz args...>"
sub="$1"
case "$sub" in
  build|run) ;;
  *) refuse "subcommand '$sub': only build or run" ;;
esac
prev=""
for a in "$@"; do
  [ "$a" = "--" ] && break
  case "$a" in
    --build-std*|-Zbuild-std*|--careful*) refuse "option '$a' builds std's own crates, outside both locks" ;;
    --target-dir*) refuse "option '$a': the target dir is the shared target" ;;
    --sanitizer=*) case "${a#--sanitizer=}" in address|none) ;; *) refuse "sanitizer '${a#--sanitizer=}'" ;; esac ;;
  esac
  case "$prev" in
    --sanitizer|-s) case "$a" in address|none) ;; *) refuse "sanitizer '$a'" ;; esac ;;
    -Z) case "$a" in build-std*) refuse "option '-Z $a'" ;; esac ;;
  esac
  prev="$a"
done
[ -z "${CUSTOM_LIBFUZZER_PATH+x}" ] || refuse "CUSTOM_LIBFUZZER_PATH is set"
[ -z "${CUSTOM_LIBFUZZER_STD_CXX+x}" ] || refuse "CUSTOM_LIBFUZZER_STD_CXX is set"
[ -z "${RUSTFLAGS+x}" ] || refuse "RUSTFLAGS is set"
[ -z "${CARGO_ENCODED_RUSTFLAGS+x}" ] || refuse "CARGO_ENCODED_RUSTFLAGS is set"
[ "${CARGO_TARGET_DIR:-}" = "$T" ] || refuse "CARGO_TARGET_DIR is '${CARGO_TARGET_DIR:-}', not the shared target $T"
hashes_match || refuse "fuzz/Cargo.toml $have_toml / fuzz/Cargo.lock $have_lock differ from dependency-check.md ($want_toml / $want_lock)"
cargo +1.99.1 tree --locked --offline --manifest-path "$F/Cargo.toml" >/dev/null || refuse "cargo tree --locked --offline failed on $F/Cargo.toml"
echo "GUARD PASSED: fuzz/Cargo.toml $have_toml, fuzz/Cargo.lock $have_lock; running cargo +$NIGHTLY fuzz $*"
cd "$S" || refuse "cannot cd to $S"
cargo "+$NIGHTLY" fuzz "$@"
rc=$?
if ! hashes_match; then
  echo "GUARD VOID: after the call fuzz/Cargo.toml is $have_toml and fuzz/Cargo.lock is $have_lock (dependency-check.md: $want_toml / $want_lock); the call's outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor"
  exit 65
fi
echo "GUARD AFTER: both fuzz files unchanged; cargo-fuzz exit $rc"
exit $rc
