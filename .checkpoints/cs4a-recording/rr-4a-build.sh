#!/usr/bin/env bash
# rust-reviewer r1: the players' Wasm at base (c1da1bd) and head, built as build.mjs builds it, each in its own copy
# and target folder; then the checked wasm-bindgen tool, --target web.
set -euo pipefail
export CARGO_ENCODED_RUSTFLAGS="--remap-path-prefix=/root/.cargo/registry/src=/cargo/registry/src"
TOOL=/home/user/Shamble-Frontlines/.gate/lane1-takeover/.gate/wb-tool/wasm-bindgen
for side in rr-4a-base rr-4a; do
  dir=/home/user/Shamble-Frontlines/.gate/lane1-takeover/.gate/$side
  (cd "$dir" && cargo build --locked --offline --release --target wasm32-unknown-unknown -p sim-wasm --target-dir "$dir/target")
  rm -rf "$dir/pkg-out"; mkdir -p "$dir/pkg-out"
  (cd "$dir" && "$TOOL" --target web --out-dir "$dir/pkg-out" "$dir/target/wasm32-unknown-unknown/release/sim_wasm.wasm")
  stat -c "$side raw %s bytes %n" "$dir/target/wasm32-unknown-unknown/release/sim_wasm.wasm" "$dir/pkg-out/sim_wasm_bg.wasm"
  sha256sum "$dir/pkg-out/sim_wasm_bg.wasm"
done
