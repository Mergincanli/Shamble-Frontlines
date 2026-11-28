#!/usr/bin/env bash
# U5's cargo-fuzz guard, rev 4 (the advisor's SEC-U5D-4 ruling, advisor-u5-early.md:316-338, condition S2; the
# dependency read, security-u5-deps.md:167-184; rev 1's holes GUARD-1 to GUARD-6, closed in rev 2, block B8; rev 2's
# Lows GUARD-7 and E1-E5, closed in rev 3, block B9; rev 3's R3-1 to R3-4, block B10, closed here: `--bite <id>` passed
# as NPT_BITE to the fuzz call alone; DEP-HOLD's `deleted  <path>` lines read and lines outside the spike skipped;
# every path crate of the fuzz tree whose manifest dependency-check.md does not list held to the copy's committed
# manifest, the copy's HEAD on the pushed branch; libFuzzer's numbers at most 9 digits, no 0 for -timeout or
# -rss_limit_mb).
# Every build-time cargo-fuzz call runs as the command of a u5-run.sh call, never directly:
#   bash u5-run.sh [-x] <worktree> <log> bash <this guard> <worktree> build [options] [target]
#   bash u5-run.sh [-x] <worktree> <log> bash <this guard> <worktree> run [options] <target> <corpus...> -- <libFuzzer flags>
# Allowed before `--`, and nothing else: -O, --release, -a, --debug-assertions, -s/--sanitizer address|none (a separate
# value, or --sanitizer=address|none), --features bites (or --features=bites); for run with --features bites only,
# `--bite <id>`: the guard's own option, never passed to cargo-fuzz; the id (PM-NNNN-N/<name>, lower-case letters,
# digits and -) reaches the fuzz call as NPT_BITE, and without it NPT_BITE is unset (U5-Q6 term 2: the runner sets it
# or clears it); one target, a [[bin]] of fuzz/Cargo.toml; for run, one or more corpus folders under <repo>/.gate/ or
# <worktree>/.gate/ (absolute, no ".."). Every other dash token (--fuzz-dir, --target-dir, -Z, -c, --careful,
# --build-std, attached forms such as -saddress) and every empty token is refused; build takes no `--`. After `--`
# (run only: libFuzzer's own flags) only these -name=value forms: -artifact_prefix= (required, under .gate/),
# -exact_artifact_path= (under .gate/), -max_total_time=, -runs=, -seed=, -max_len=, -malloc_limit_mb= (1 to 9
# digits), -timeout= and -rss_limit_mb= (1 to 9 digits, not 0: 0 turns libFuzzer's limit off), -print_final_stats=,
# -use_value_profile= and -detect_leaks= (0 or 1). A bare token (libFuzzer would take it as its first corpus, the one it
# writes into) is refused.
# Before the call, in this order: the runner's environment (CUSTOM_LIBFUZZER_PATH, CUSTOM_LIBFUZZER_STD_CXX, RUSTFLAGS,
# CARGO_ENCODED_RUSTFLAGS unset; CARGO_TARGET_DIR the shared target; CARGO_NET_OFFLINE=true); no cargo config file
# (.cargo/config.toml or .cargo/config) in any folder from the spike folder up to /, in /root/.cargo or in fuzz/.cargo,
# but the spike's own .cargo/config.toml at its known sha256 (build.target-dir only); 1.99.1 installed (so rustup never
# tries a download); every spike file dependency-check.md lists (read from the file, never retyped) has one of its
# listed sha256 under the worktree, or is absent where a `deleted  <path>` line lists it (DEP-HOLD's two line forms,
# .claude/CHECKPOINTS.md:64-66; lines for paths outside spikes/netcode-proof/ are skipped; any other line form is
# refused), and the fuzz manifest and lock have exactly one line each; `cargo +1.99.1 tree --locked --offline` on the
# fuzz manifest, with the call's features, passes; the copy's HEAD is on the pushed branch, and every path crate in that
# tree whose manifest dependency-check.md does not list has its manifest equal to HEAD's (git's blob ids, read with
# --no-filters); the nightly installed; the installed cargo-fuzz reports exactly "cargo-fuzz 0.13.2"; the same tree
# passes on the nightly. Every cargo call (both trees, the version, the fuzz call) runs under `env -i` with only PATH,
# HOME, CARGO_HOME, RUSTUP_HOME, CARGO_TARGET_DIR and CARGO_NET_OFFLINE set (and NPT_BITE for a bite run's fuzz call),
# and every git read under `env -i` with only PATH and HOME, so no CARGO_*, RUSTC*, RUSTUP_*, CUSTOM_LIBFUZZER_* or GIT_*
# setting from the runner's environment reaches them (the guard never lists that environment). Then `cargo
# +nightly-2026-11-01 fuzz <the args without --bite>` runs from the copy's spikes/netcode-proof, and every listed file
# and every held manifest is checked again.
# Plant test (shown once before the first cargo-fuzz call, inside one exclusive runner call):
#   bash u5-run.sh -x <worktree> <log> bash <this guard> --plant-test <worktree>
# appends one byte to fuzz/Cargo.toml, expects the hash check to refuse it, restores the file (a cp -p backup moved back,
# with a trap that moves it back if the script is stopped between the change and the restore) and checks every listed
# file again; it runs no cargo-fuzz.
# Exit codes: cargo-fuzz's own after a passed check; 64 a refused call (nothing ran); 65 a hash mismatch after the call
# (its outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor); 66 a failed
# plant test (not refused, or not restored; the backup's path is printed).
set -u
M=/home/user/Shamble-Frontlines
DC=$M/.checkpoints/netcode-proof-test/dependency-check.md
T=$M/.gate/shared-target
NIGHTLY=nightly-2026-11-01
FUZZ_VERSION=0.13.2
BRANCH_REF=refs/remotes/origin/claude/netcode-proof-test-plan-hcky5g
# The spike's .cargo/config.toml (build.target-dir only), read 2026-11-21: equal in the main folder and the builder's copy.
SPIKE_CFG_SHA=9ff1aa86e4f2366197a220b20e1c0e47f39c0575a08b55becb3d99a80db7e30a
TC=/root/.rustup/toolchains
CE=(env -i PATH=/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin HOME=/root CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup
    "CARGO_TARGET_DIR=$T" CARGO_NET_OFFLINE=true)
GE=(env -i PATH=/usr/bin:/bin HOME=/root)
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

# dependency-check.md: every line is blank, a '#' comment, '<sha256>  <path>' or 'deleted  <path>'. Lines for paths
# outside spikes/netcode-proof/ are skipped; a spike path must be plain (letters, digits, _ . / -, no "..").
dc_lines=()
dc_paths=()
dc_skipped=0
while IFS= read -r l || [ -n "$l" ]; do
  [[ "$l" =~ ^[[:space:]]*$ ]] && continue
  [[ "$l" =~ ^[[:space:]]*# ]] && continue
  [[ "$l" =~ ^([0-9a-f]{64}|deleted)\ \ (.+)$ ]] \
    || refuse "dependency-check.md line '$l' is not '<sha256>  <path>' or 'deleted  <path>'"
  p="${BASH_REMATCH[2]}"
  case "$p" in
    spikes/netcode-proof/*) ;;
    *) dc_skipped=$((dc_skipped + 1)); continue ;;
  esac
  [[ "$p" =~ ^spikes/netcode-proof/[A-Za-z0-9_./-]+$ ]] || refuse "dependency-check.md line '$l': the path is not plain"
  case "$p" in *..*) refuse "dependency-check.md line '$l' holds '..'" ;; esac
  dc_lines+=("$l")
  case " ${dc_paths[*]+"${dc_paths[*]}"} " in *" $p "*) ;; *) dc_paths+=("$p") ;; esac
done < "$DC"
n_toml=0
n_lock=0
for l in "${dc_lines[@]+"${dc_lines[@]}"}"; do
  case "$l" in
    *"  spikes/netcode-proof/fuzz/Cargo.toml") n_toml=$((n_toml + 1)) ;;
    *"  spikes/netcode-proof/fuzz/Cargo.lock") n_lock=$((n_lock + 1)) ;;
  esac
done
[ $n_toml -eq 1 ] || refuse "dependency-check.md must hold exactly one fuzz/Cargo.toml line (holds $n_toml)"
[ $n_lock -eq 1 ] || refuse "dependency-check.md must hold exactly one fuzz/Cargo.lock line (holds $n_lock)"

# Each listed spike path passes when its file under the worktree has one of the sha256 listed for that path, or is
# absent where a `deleted` line lists it (DEP-HOLD's exact-line rule).
hashes_match() {
  local p have l ok
  bad=""
  for p in "${dc_paths[@]}"; do
    ok=0
    if [ -e "$W/$p" ] || [ -L "$W/$p" ]; then
      have=$(sha256sum "$W/$p" 2>/dev/null | cut -d' ' -f1)
      for l in "${dc_lines[@]}"; do
        [ "$l" = "$have  $p" ] && ok=1
      done
      [ $ok -eq 1 ] || bad="$bad $p (${have:-unreadable})"
    else
      for l in "${dc_lines[@]}"; do
        [ "$l" = "deleted  $p" ] && ok=1
      done
      [ $ok -eq 1 ] || bad="$bad $p (missing)"
    fi
  done
  [ -z "$bad" ]
}

# git reads inside the copy (run from $S, after the cd below), with no GIT_* setting from the runner's environment.
file_blob() { "${GE[@]}" git hash-object --no-filters -- "$1" 2>/dev/null; }
head_blob() { "${GE[@]}" git rev-parse --verify --quiet "HEAD:$1" 2>/dev/null; }
held_paths=()
held_blobs=()
held_match() {
  local i have
  bad=""
  for i in "${!held_paths[@]}"; do
    have=$(file_blob "$W/${held_paths[$i]}")
    [ "$have" = "${held_blobs[$i]}" ] || bad="$bad ${held_paths[$i]} (${have:-unreadable})"
  done
  [ -z "$bad" ]
}

if [ $plant -eq 1 ]; then
  hashes_match || refuse "plant test needs a clean start; differing from dependency-check.md:$bad"
  backup=$(mktemp "$M/.gate/u5/guard-plant.XXXXXX") || refuse "cannot make the backup"
  cp -p "$F/Cargo.toml" "$backup" || refuse "cannot back up fuzz/Cargo.toml"
  trap 'mv -f "$backup" "$F/Cargo.toml" 2>/dev/null' EXIT
  trap 'mv -f "$backup" "$F/Cargo.toml" 2>/dev/null; echo "PLANT TEST STOPPED (restore attempted from $backup)"; exit 66' INT TERM HUP
  printf ' ' >> "$F/Cargo.toml"
  planted="NOT REFUSED"
  if ! hashes_match; then
    case "$bad" in *" spikes/netcode-proof/fuzz/Cargo.toml ("*) planted="REFUSED" ;; esac
  fi
  echo "plant: fuzz/Cargo.toml with one byte appended; the hash check $planted it (differing:${bad:- none})"
  mv -f "$backup" "$F/Cargo.toml"
  mvrc=$?
  trap - EXIT INT TERM HUP
  if [ $mvrc -eq 0 ] && hashes_match; then restored="restored"; else restored="NOT RESTORED (backup $backup)"; fi
  echo "after restore: all ${#dc_paths[@]} listed files checked, differing:${bad:- none}: $restored"
  if [ "$planted" = "REFUSED" ] && [ "$restored" = "restored" ]; then
    echo "PLANT TEST PASSED"
    exit 0
  fi
  echo "PLANT TEST FAILED"
  exit 66
fi

[ $# -ge 1 ] || refuse "usage: bash u5-fuzz-guard.sh <worktree> <build|run> [options] [--bite <id>] [target] [corpus...] [-- <libFuzzer flags>]"
sub="$1"
shift
case "$sub" in
  build|run) ;;
  *) refuse "subcommand '$sub': only build or run" ;;
esac
pass=("$sub")
target=""
features=""
bite=""
corpora=()
lf=()
seen_dd=0
while [ $# -gt 0 ]; do
  a="$1"
  shift
  [ -n "$a" ] || refuse "an empty argument"
  if [ $seen_dd -eq 1 ]; then
    lf+=("$a")
    pass+=("$a")
    continue
  fi
  case "$a" in
    --)
      [ "$sub" = "run" ] || refuse "build takes no '--' (libFuzzer flags are for run only)"
      seen_dd=1
      pass+=("$a") ;;
    --bite)
      [ $# -ge 1 ] || refuse "'--bite' needs a bite id"
      v="$1"
      shift
      [ -z "$bite" ] || refuse "one --bite only"
      [[ "$v" =~ ^PM-[0-9]{4}-[0-9]{1,3}/[a-z0-9][a-z0-9-]{0,63}$ ]] \
        || refuse "bite id '$v' is not PM-NNNN-N/<name> (lower-case letters, digits and -)"
      bite="$v" ;;
    -O|--release|-a|--debug-assertions) pass+=("$a") ;;
    -s|--sanitizer)
      [ $# -ge 1 ] || refuse "'$a' needs a value"
      v="$1"
      shift
      case "$v" in address|none) ;; *) refuse "sanitizer '$v': only address or none" ;; esac
      pass+=("$a" "$v") ;;
    --sanitizer=address|--sanitizer=none) pass+=("$a") ;;
    --features)
      [ $# -ge 1 ] || refuse "'$a' needs a value"
      v="$1"
      shift
      [ "$v" = "bites" ] || refuse "features '$v': only bites"
      features=bites
      pass+=("$a" "$v") ;;
    --features=bites) features=bites; pass+=("$a") ;;
    -*) refuse "option '$a' is not on the allow-list (-O, --release, -a, --debug-assertions, -s/--sanitizer address|none, --features bites, --bite <id>)" ;;
    *)
      if [ -z "$target" ]; then
        target="$a"
      elif [ "$sub" = "run" ]; then
        corpora+=("$a")
      else
        refuse "build takes one target (extra: '$a')"
      fi
      pass+=("$a") ;;
  esac
done
if [ -n "$bite" ]; then
  [ "$sub" = "run" ] || refuse "--bite is for run only"
  [ "$features" = "bites" ] || refuse "--bite needs --features bites"
fi
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
      -max_total_time=*|-runs=*|-seed=*|-max_len=*|-malloc_limit_mb=*)
        [[ "${x#*=}" =~ ^[0-9]{1,9}$ ]] || refuse "libFuzzer flag '$x': the value must be 1 to 9 digits" ;;
      -timeout=*|-rss_limit_mb=*)
        [[ "${x#*=}" =~ ^[1-9][0-9]{0,8}$ ]] \
          || refuse "libFuzzer flag '$x': the value must be 1 to 9 digits and not 0 (0 turns the limit off)" ;;
      -print_final_stats=0|-print_final_stats=1|-use_value_profile=0|-use_value_profile=1|-detect_leaks=0|-detect_leaks=1) ;;
      *) refuse "libFuzzer flag '$x' is not on the allow-list (-artifact_prefix=, -exact_artifact_path=, -max_total_time=, -runs=, -seed=, -max_len=, -malloc_limit_mb=, -timeout=, -rss_limit_mb=, -print_final_stats=, -use_value_profile=, -detect_leaks=)" ;;
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
cd "$S" || refuse "cannot cd to $S"
SP=$(pwd -P)
d=$SP
while :; do
  for n in config.toml config; do
    cfg="${d%/}/.cargo/$n"
    if [ -e "$cfg" ] || [ -L "$cfg" ]; then
      if [ "$cfg" = "$SP/.cargo/config.toml" ] && [ -f "$cfg" ] && [ ! -L "$cfg" ] \
         && [ "$(sha256sum "$cfg" | cut -d' ' -f1)" = "$SPIKE_CFG_SHA" ]; then
        :
      else
        refuse "cargo config $cfg is not allowed (only $SP/.cargo/config.toml at sha256 $SPIKE_CFG_SHA)"
      fi
    fi
  done
  [ "$d" = / ] && break
  d=$(dirname "$d")
done
for cfg in /root/.cargo/config.toml /root/.cargo/config "$F/.cargo/config.toml" "$F/.cargo/config"; do
  if [ -e "$cfg" ] || [ -L "$cfg" ]; then
    refuse "cargo config $cfg exists"
  fi
done
[ -d "$TC/1.99.1-x86_64-unknown-linux-gnu" ] || refuse "toolchain 1.99.1 is not installed under $TC"
hashes_match || refuse "differing from dependency-check.md:$bad"
tf=()
[ -n "$features" ] && tf=(--features "$features")
tree_out=$("${CE[@]}" cargo +1.99.1 tree --locked --offline --prefix none --format '{p}' -e normal,build \
  "${tf[@]+"${tf[@]}"}" --manifest-path "$F/Cargo.toml" 2>/dev/null) \
  || refuse "cargo +1.99.1 tree --locked --offline ${tf[*]+"${tf[*]}"} failed on $F/Cargo.toml"
"${GE[@]}" git merge-base --is-ancestor HEAD "$BRANCH_REF" 2>/dev/null \
  || refuse "the copy's HEAD is not on the pushed branch ($BRANCH_REF)"
mapfile -t graph_dirs < <(printf '%s\n' "$tree_out" | grep -oE '\(/[^)]*\)' | tr -d '()' | sort -u)
[ ${#graph_dirs[@]} -ge 1 ] || refuse "the fuzz tree names no path crate"
n_listed=0
for gd in "${graph_dirs[@]}"; do
  case "$gd" in "$W"/*) ;; *) refuse "path crate $gd is outside the copy" ;; esac
  case "$gd" in *..*) refuse "path crate $gd holds '..'" ;; esac
  rp="${gd#"$W"/}/Cargo.toml"
  case " ${dc_paths[*]} " in *" $rp "*) n_listed=$((n_listed + 1)); continue ;; esac
  hb=$(head_blob "$rp") || hb=""
  [ -n "$hb" ] || refuse "$rp is neither listed in dependency-check.md nor in the copy's HEAD"
  fb=$(file_blob "$W/$rp")
  [ "$fb" = "$hb" ] || refuse "$rp is not listed in dependency-check.md and differs from the copy's HEAD (file ${fb:-unreadable}, HEAD $hb)"
  held_paths+=("$rp")
  held_blobs+=("$hb")
done
echo "GUARD GRAPH: ${#graph_dirs[@]} path crates; $n_listed listed in dependency-check.md; ${#held_paths[@]} held to the copy's HEAD:${held_paths[*]+ ${held_paths[*]}}"
[ -d "$TC/$NIGHTLY-x86_64-unknown-linux-gnu" ] || refuse "toolchain $NIGHTLY is not installed under $TC"
fv=$("${CE[@]}" cargo "+$NIGHTLY" fuzz --version 2>/dev/null) || refuse "cargo +$NIGHTLY fuzz --version failed (is cargo-fuzz installed?)"
[ "$fv" = "cargo-fuzz $FUZZ_VERSION" ] || refuse "cargo-fuzz reports '$fv', not exactly 'cargo-fuzz $FUZZ_VERSION'"
"${CE[@]}" cargo "+$NIGHTLY" tree --locked --offline "${tf[@]+"${tf[@]}"}" --manifest-path "$F/Cargo.toml" >/dev/null \
  || refuse "cargo +$NIGHTLY tree --locked --offline ${tf[*]+"${tf[*]}"} failed on $F/Cargo.toml"
fe=("${CE[@]}")
[ -n "$bite" ] && fe+=("NPT_BITE=$bite")
echo "GUARD PASSED: $fv; all ${#dc_paths[@]} listed spike files match ($dc_skipped lines outside the spike skipped); ${#held_paths[@]} held manifests equal the copy's HEAD; NPT_BITE ${bite:-unset}; running cargo +$NIGHTLY fuzz ${pass[*]}"
"${fe[@]}" cargo "+$NIGHTLY" fuzz "${pass[@]}"
rc=$?
if ! hashes_match; then
  echo "GUARD VOID: after the call these differ from dependency-check.md:$bad; the call's outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor"
  exit 65
fi
if ! held_match; then
  echo "GUARD VOID: after the call these held manifests differ from the copy's HEAD:$bad; the call's outputs are void: the main session cleans the fuzz builds' folder and the case goes to the advisor"
  exit 65
fi
echo "GUARD AFTER: all ${#dc_paths[@]} listed files and ${#held_paths[@]} held manifests unchanged; cargo-fuzz exit $rc"
exit $rc
