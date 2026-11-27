#!/usr/bin/env bash
# Stop B's dist build (stopb-plan.md rev 0.3, C3): rs.yaml's dist lines (:193 rustup toolchain install, :194
# wasm-bindgen-cli, :195 cargo fetch, :198-200 web/'s npm ci and build.mjs), word for word but for the named
# deviation (`npm ci` gains --no-audit --no-fund, the lane's form), run in a worktree with its own CARGO_HOME,
# wasm-bindgen root and targets under the worktree's own .gate/ (removed with it). Not :196-197 (RS's binaries).
# Usage: bash stopb-distbuild.sh <worktree, absolute, under .gate/> <log, absolute> <list out, absolute>
set -euo pipefail
WT=$1 LOG=$2 LIST=$3
case $WT in /home/user/Shamble-Frontlines/.gate/*) ;; *) echo "STOP: the worktree must be under .gate/"; exit 2 ;; esac
SPIKE=$WT/spikes/netcode-proof
S=$WT/.gate
mkdir -p "$S"
exec > >(tee -a "$LOG") 2>&1
LOCK=/home/user/Shamble-Frontlines/.gate/shared-target/.lane.lock
exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }
flock -x 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
echo "=== $(date -u +%T) UTC: lock held; worktree $WT"
echo "free MiB before: $(df -m "$WT" | tail -1 | awk '{print $4}')"
for v in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS RUSTC_WRAPPER CARGO_TARGET_DIR CARGO_NET_OFFLINE RUSTUP_TOOLCHAIN NPT_WASM_BINDGEN CARGO_HOME; do
  if [ -n "${!v+x}" ]; then echo "STOP: $v is set"; exit 3; fi
done
if env | cut -d= -f1 | grep -Eq '^CARGO_(BUILD|PROFILE)_'; then echo "STOP: a CARGO_BUILD_ or CARGO_PROFILE_ variable is set"; exit 3; fi
if [ -n "${RUSTUP_HOME+x}" ]; then echo "build variables: none set; RUSTUP_HOME is set (the container toolchain home)"; else echo "build variables: none set"; fi
export CARGO_HOME=$S/cargo
echo "tools: $(rustup --version 2>/dev/null | head -1); node $(node --version); npm $(npm --version)"
cd "$SPIKE"
# Deviation 2 (named in stopb-plan.md C3): --no-self-update. The container's rustup lives outside this build's
# CARGO_HOME (on RS, rustup-init puts it in $HOME/.cargo), so its self-update step fails here; the flag also keeps
# the container's rustup at its pinned 1.29.3. The toolchain installed (1.99.1, rust-toolchain.toml) is the same.
rustup toolchain install --no-self-update
echo "rustc: $(rustc --version)"
CARGO_TARGET_DIR="$S/wbg-target" cargo install wasm-bindgen-cli --version 0.2.130 --locked --root "$S/wbg"
cargo fetch --locked
export PATH="$S/wbg/bin:$PATH"
echo "wasm-bindgen: $(command -v wasm-bindgen) $(wasm-bindgen --version)"
cd web
npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund
npm_config_ignore_scripts=true CARGO_NET_OFFLINE=true node build.mjs
(cd dist && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum) > "$LIST"
echo "--- the list ($(wc -l < "$LIST") files), rs.yaml:221's form:"
cat "$LIST"
echo "free MiB after: $(df -m "$WT" | tail -1 | awk '{print $4}')"
echo "=== $(date -u +%T) UTC: done"
