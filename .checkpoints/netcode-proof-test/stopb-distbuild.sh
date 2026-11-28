#!/usr/bin/env bash
# Stop B's dist build (stopb-plan.md rev 0.3, C3; B6's ruling on the dist difference, advisor-stopb-plan.md): rs.yaml's
# dist lines (:193 rustup toolchain install, :194 wasm-bindgen-cli, :195 cargo fetch, :198-200 web/'s npm ci and
# build.mjs), word for word but for two named deviations (`npm ci` gains --no-audit --no-fund; `rustup toolchain install`
# gains --no-self-update), in a worktree under .gate/, with CARGO_HOME, the wasm-bindgen root and its target OUTSIDE any
# git repository, as on RS (STOPB-8: wasm-bindgen-shared's build.rs embeds `git rev-parse HEAD` of the folder holding
# its registry source into the Wasm's `producers` section). Not :196-197 (RS's binaries).
# Usage: bash stopb-distbuild.sh <worktree under .gate/> <log> <list out> <cargo base outside the repository>
set -euo pipefail
WT=$1 LOG=$2 LIST=$3 X=$4
case $WT in /home/user/Shamble-Frontlines/.gate/*) ;; *) echo "STOP: the worktree must be under .gate/"; exit 2 ;; esac
case $X in /home/user/Shamble-Frontlines|/home/user/Shamble-Frontlines/*) echo "STOP: the cargo base must be outside the repository"; exit 2 ;; /*) ;; *) echo "STOP: the cargo base must be absolute"; exit 2 ;; esac
SPIKE=$WT/spikes/netcode-proof
mkdir -p "$X"
exec > >(tee -a "$LOG") 2>&1
LOCK=/home/user/Shamble-Frontlines/.gate/shared-target/.lane.lock
exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }
flock -x 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
echo "=== $(date -u +%T) UTC: lock held; worktree $WT; cargo base $X"
echo "free MiB before: worktree disk $(df -m "$WT" | tail -1 | awk '{print $4}'), cargo base disk $(df -m "$X" | tail -1 | awk '{print $4}')"
if (cd "$X" && git rev-parse HEAD > /dev/null 2>&1); then echo "STOP: the cargo base $X is inside a git repository"; exit 4; fi
echo "git rev-parse HEAD fails in the cargo base: ok"
for v in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS RUSTC_WRAPPER CARGO_TARGET_DIR CARGO_NET_OFFLINE RUSTUP_TOOLCHAIN NPT_WASM_BINDGEN CARGO_HOME; do
  if [ -n "${!v+x}" ]; then echo "STOP: $v is set"; exit 3; fi
done
if env | cut -d= -f1 | grep -Eq '^CARGO_(BUILD|PROFILE)_'; then echo "STOP: a CARGO_BUILD_ or CARGO_PROFILE_ variable is set"; exit 3; fi
gitvars=$(env | cut -d= -f1 | grep -E '^GIT_' || true)
for v in $gitvars; do echo "unset: $v (name only)"; unset "$v"; done
echo "GIT_ variables: ${gitvars:+unset above}${gitvars:-none set}"
if [ -n "${RUSTUP_HOME+x}" ]; then echo "build variables: none set; RUSTUP_HOME is set (the container toolchain home)"; else echo "build variables: none set"; fi
export CARGO_HOME=$X/cargo
echo "tools: $(rustup --version 2>/dev/null | head -1); node $(node --version); npm $(npm --version)"
cd "$SPIKE"
# Deviation 2: --no-self-update (the container's rustup lives outside this build's CARGO_HOME; the flag also keeps the
# container's rustup at its pinned 1.29.3). The toolchain installed (1.99.1, rust-toolchain.toml) is the same.
rustup toolchain install --no-self-update
echo "rustc: $(rustc --version)"
CARGO_TARGET_DIR="$X/wbg-target" cargo install wasm-bindgen-cli --version 0.2.130 --locked --root "$X/wbg"
WS=$(ls -d "$CARGO_HOME"/registry/src/*/wasm-bindgen-shared-0.2.130 | head -1)
if (cd "$WS" && git rev-parse HEAD > /dev/null 2>&1); then echo "STOP: git rev-parse HEAD succeeds in $WS"; exit 4; fi
echo "git rev-parse HEAD fails in wasm-bindgen-shared's own folder ($WS): ok"
cargo fetch --locked
export PATH="$X/wbg/bin:$PATH"
echo "wasm-bindgen: $(command -v wasm-bindgen) $(wasm-bindgen --version)"
cd web
npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund
npm_config_ignore_scripts=true CARGO_NET_OFFLINE=true node build.mjs
P=$(node -e 'const b=require("fs").readFileSync(process.argv[1]);const i=b.indexOf(Buffer.from("wasm-bindgen"),b.indexOf(Buffer.from("producers")));const n=b[i-1];process.stdout.write(b.slice(i,i+12).toString()+" "+b.slice(i+13,i+13+b[i+12]).toString())' dist/npt_client_wasm_bg.wasm)
echo "producers: $P"
[ "$P" = "wasm-bindgen 0.2.130" ] || { echo "STOP: the producers entry is not plain \"wasm-bindgen 0.2.130\""; exit 6; }
(cd dist && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum) > "$LIST"
echo "--- the list ($(wc -l < "$LIST") files), rs.yaml:221's form:"
cat "$LIST"
echo "free MiB after: worktree disk $(df -m "$WT" | tail -1 | awk '{print $4}'), cargo base disk $(df -m "$X" | tail -1 | awk '{print $4}')"
echo "=== $(date -u +%T) UTC: done"
