#!/usr/bin/env bash
# U5's cargo-fuzz guard, rev 2 (the advisor's SEC-U5D-4 ruling, advisor-u5-early.md:316-338, condition S2; the
# dependency read, security-u5-deps.md:167-184; rev 1's holes GUARD-1 to GUARD-6, security-u5-deps.md block B8, closed
# here: an allow-list before `--`, the run's outputs under .gate/, the offline and version checks, the plant test's trap).
# Every build-time cargo-fuzz call runs as the command of a u5-run.sh call, never directly:
#   bash u5-run.sh [-x] <worktree> <log> bash <this guard> <worktree> build [options] [target]
#   bash u5-run.sh [-x] <worktree> <log> bash <this guard> <worktree> run [options] <target> <corpus...> -- <libFuzzer flags>
# Allowed before `--`, and nothing else: -O, --release, -a, --debug-assertions, -s/--sanitizer address|none (a separate
# value, or --sanitizer=address|none), --features bites (or --features=bites); one target, a [[bin]] of fuzz/Cargo.toml;
# for run, one or more corpus folders under <repo>/.gate/ or <worktree>/.gate/ (absolute, no ".."). Every other dash
# token (--fuzz-dir, --target-dir, -Z, -c, --careful, --build-std, attached forms such as -saddress) is refused. After
# `--` (libFuzzer's own flags) run needs -artifact_prefix=<a folder under .gate/>/, and -exact_artifact_path= must be
# under .gate/ too.
# Before the call: the environment (CUSTOM_LIBFUZZER_PATH, CUSTOM_LIBFUZZER_STD_CXX, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS
# unset; no cargo config file names CUSTOM_LIBFUZZER; CARGO_TARGET_DIR the shared target; CARGO_NET_OFFLINE=true); the
# installed cargo-fuzz is 0.13.2; fuzz/Cargo.toml's and fuzz/Cargo.lock's sha256 equal dependency-check.md's lines (read
# from the file, never retyped); `cargo tree --locked --offline` on the fuzz manifest, with the call's features, passes on
# 1.99.1 and on the nightly. Then `cargo +nightly-2026-11-01 fuzz <the same args>` runs from the copy's
# spikes/netcode-proof, and both files are hashed again.
# Plant test (shown once before the first cargo-fuzz call, inside one exclusive runner call):
#   bash u5-run.sh -x <worktree> <log> bash <this guard> --plant-test <worktree>
# appends one byte to fuzz/Cargo.toml, expects the hash check to refuse, restores the file (a cp -p backup moved back,
# with a trap that moves it back if the script is stopped between the change and the restore) and checks both hashes
# equal dependency-check.md's again; it runs no cargo-fuzz.
# Exit codes: cargo-fuzz's own after a passed check; 64 a refused call (nothing ran); 65 a hash mismatch after the call
# (its outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor); 66 a failed
# plant test (not refused, or not restored; the backup's path is printed).
set -u
M=/home/user/Shamble-Frontlines
DC=$M/.checkpoints/netcode-proof-test/dependency-check.md
T=$M/.gate/shared-target
NIGHTLY=nightly-2026-11-01
FUZZ_VERSION=0.13.2
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
case "$W" in *..*) refuse "the worktree path holds '..' ($W)" ;; esac
shift
S=$W/spikes/netcode-proof
F=$S/fuzz
[ -f "$F/Cargo.toml" ] && [ -f "$F/Cargo.lock" ] || refuse "no fuzz/Cargo.toml or fuzz/Cargo.lock under $S"
[ -f "$DC" ] || refuse "no $DC"

under_gate() {
  case "$1" in *..*) return 1 ;; esac
  case "$1" in "$M"/.gate/*|"$W"/.gate/*) return 0 ;; *) return 1 ;; esac
}

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
  trap 'mv -f "$backup" "$F/Cargo.toml" 2>/dev/null' EXIT
  trap 'mv -f "$backup" "$F/Cargo.toml" 2>/dev/null; echo "PLANT TEST STOPPED (restore attempted from $backup)"; exit 66' INT TERM HUP
  printf ' ' >> "$F/Cargo.toml"
  if hashes_match; then planted="NOT REFUSED"; else planted="REFUSED"; fi
  echo "plant: fuzz/Cargo.toml with one byte appended is $have_toml; the hash check $planted it"
  mv -f "$backup" "$F/Cargo.toml"
  mvrc=$?
  trap - EXIT INT TERM HUP
  if [ $mvrc -eq 0 ] && hashes_match; then restored="restored"; else restored="NOT RESTORED (backup $backup)"; fi
  echo "after restore: fuzz/Cargo.toml $have_toml, fuzz/Cargo.lock $have_lock: $restored"
  if [ "$planted" = "REFUSED" ] && [ "$restored" = "restored" ]; then
    echo "PLANT TEST PASSED"
    exit 0
  fi
  echo "PLANT TEST FAILED"
  exit 66
fi

[ $# -ge 1 ] || refuse "usage: bash u5-fuzz-guard.sh <worktree> <build|run> [options] [target] [corpus...] [-- <libFuzzer flags>]"
orig=("$@")
sub="$1"
shift
case "$sub" in
  build|run) ;;
  *) refuse "subcommand '$sub': only build or run" ;;
esac
target=""
features=""
corpora=()
lf=()
seen_dd=0
while [ $# -gt 0 ]; do
  a="$1"
  shift
  if [ $seen_dd -eq 1 ]; then
    lf+=("$a")
    continue
  fi
  case "$a" in
    --) seen_dd=1 ;;
    -O|--release|-a|--debug-assertions) ;;
    -s|--sanitizer)
      [ $# -ge 1 ] || refuse "'$a' needs a value"
      v="$1"
      shift
      case "$v" in address|none) ;; *) refuse "sanitizer '$v': only address or none" ;; esac ;;
    --sanitizer=address|--sanitizer=none) ;;
    --features)
      [ $# -ge 1 ] || refuse "'$a' needs a value"
      v="$1"
      shift
      [ "$v" = "bites" ] || refuse "features '$v': only bites"
      features=bites ;;
    --features=bites) features=bites ;;
    -*) refuse "option '$a' is not on the allow-list (-O, --release, -a, --debug-assertions, -s/--sanitizer address|none, --features bites)" ;;
    *)
      if [ -z "$target" ]; then
        target="$a"
      elif [ "$sub" = "run" ]; then
        corpora+=("$a")
      else
        refuse "build takes one target (extra: '$a')"
      fi ;;
  esac
done
if [ -n "$target" ]; then
  case "$target" in *[!a-z0-9_]*) refuse "target '$target': letters, digits and _ only" ;; esac
  grep -qE "^name = \"$target\"$" "$F/Cargo.toml" || refuse "target '$target' is not a [[bin]] name in fuzz/Cargo.toml"
fi
if [ "$sub" = "run" ]; then
  [ -n "$target" ] || refuse "run needs a target"
  [ ${#corpora[@]} -ge 1 ] || refuse "run needs a corpus folder under .gate/ (else cargo-fuzz writes into fuzz/corpus/)"
  for c in "${corpora[@]}"; do
    under_gate "$c" || refuse "corpus '$c' is not an absolute path under $M/.gate/ or $W/.gate/"
  done
  ap=""
  for x in "${lf[@]+"${lf[@]}"}"; do
    case "$x" in
      -artifact_prefix=*) ap="${x#-artifact_prefix=}" ;;
      -exact_artifact_path=*) under_gate "${x#-exact_artifact_path=}" || refuse "'$x' is not under .gate/" ;;
    esac
  done
  [ -n "$ap" ] || refuse "run needs -artifact_prefix=<a folder under .gate/>/ after -- (else artifacts go to fuzz/artifacts/)"
  under_gate "$ap" || refuse "-artifact_prefix '$ap' is not an absolute path under .gate/"
fi

[ -z "${CUSTOM_LIBFUZZER_PATH+x}" ] || refuse "CUSTOM_LIBFUZZER_PATH is set"
[ -z "${CUSTOM_LIBFUZZER_STD_CXX+x}" ] || refuse "CUSTOM_LIBFUZZER_STD_CXX is set"
[ -z "${RUSTFLAGS+x}" ] || refuse "RUSTFLAGS is set"
[ -z "${CARGO_ENCODED_RUSTFLAGS+x}" ] || refuse "CARGO_ENCODED_RUSTFLAGS is set"
[ "${CARGO_TARGET_DIR:-}" = "$T" ] || refuse "CARGO_TARGET_DIR is '${CARGO_TARGET_DIR:-}', not the shared target $T"
[ "${CARGO_NET_OFFLINE:-}" = "true" ] || refuse "CARGO_NET_OFFLINE is '${CARGO_NET_OFFLINE:-}', not true"
for cfg in "$F/.cargo/config.toml" "$F/.cargo/config" "$S/.cargo/config.toml" "$S/.cargo/config" \
           "$W/.cargo/config.toml" "$W/.cargo/config" "${HOME:-/root}/.cargo/config.toml" "${HOME:-/root}/.cargo/config"; do
  if [ -f "$cfg" ] && grep -q "CUSTOM_LIBFUZZER" "$cfg"; then
    refuse "$cfg names CUSTOM_LIBFUZZER"
  fi
done
fv=$(cargo "+$NIGHTLY" fuzz --version 2>/dev/null) || refuse "cargo +$NIGHTLY fuzz --version failed (is cargo-fuzz installed?)"
case "$fv" in *"cargo-fuzz $FUZZ_VERSION"*) ;; *) refuse "cargo-fuzz is '$fv', not $FUZZ_VERSION" ;; esac
hashes_match || refuse "fuzz/Cargo.toml $have_toml / fuzz/Cargo.lock $have_lock differ from dependency-check.md ($want_toml / $want_lock)"
tf=()
[ -n "$features" ] && tf=(--features "$features")
cargo +1.99.1 tree --locked --offline "${tf[@]+"${tf[@]}"}" --manifest-path "$F/Cargo.toml" >/dev/null \
  || refuse "cargo +1.99.1 tree --locked --offline ${tf[*]+"${tf[*]}"} failed on $F/Cargo.toml"
cargo "+$NIGHTLY" tree --locked --offline "${tf[@]+"${tf[@]}"}" --manifest-path "$F/Cargo.toml" >/dev/null \
  || refuse "cargo +$NIGHTLY tree --locked --offline ${tf[*]+"${tf[*]}"} failed on $F/Cargo.toml"
echo "GUARD PASSED: $fv; fuzz/Cargo.toml $have_toml, fuzz/Cargo.lock $have_lock; running cargo +$NIGHTLY fuzz ${orig[*]}"
cd "$S" || refuse "cannot cd to $S"
cargo "+$NIGHTLY" fuzz "${orig[@]}"
rc=$?
if ! hashes_match; then
  echo "GUARD VOID: after the call fuzz/Cargo.toml is $have_toml and fuzz/Cargo.lock is $have_lock (dependency-check.md: $want_toml / $want_lock); the call's outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor"
  exit 65
fi
echo "GUARD AFTER: both fuzz files unchanged; cargo-fuzz exit $rc"
exit $rc
