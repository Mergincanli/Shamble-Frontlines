# Step 8, pre-staged (the main session, 2026-11-25 06:3x UTC). Not the record: dependency-check.md gets the block
# below (filled by security's answer) only after security-u7-0b-answer.md ends DONE with Q2 (a), or after step 7.
#
# Order at step 8 (each one command; the hold reads the PUSHED commit's dependency-check.md, lines trimmed, each
# "<sha256>  <path>" with two spaces, the path from the repository root: checkpoint.mjs:17-24, :114-140):
# 1. Re-hash, must equal the lines below: `sha256sum spikes/netcode-proof/Cargo.toml spikes/netcode-proof/Cargo.lock
#    spikes/netcode-proof/crates/client-wasm/Cargo.toml`; `git status --short -- spikes/` shows those three only.
# 2. `node tools/gate.mjs --no-smoke` PASS.
# 3. `git add spikes/netcode-proof/Cargo.toml spikes/netcode-proof/Cargo.lock spikes/netcode-proof/crates/client-wasm/Cargo.toml`
# 4. `git commit -F <scratchpad>/u7-0b-commit-msg.txt` (subject, body, the two trailer lines).
# 5. Append the block to dependency-check.md by Edit (after its last line, one blank line before).
# 6. The save: `node .claude/scripts/checkpoint.mjs save netcode-proof-test -m "U7 Block 0b: DEP-HOLD released
#    (security-u7-deps.md, security-u7-0b-answer.md, dependency-check.md)" --trailer ... --trailer ...`; it must say
#    "saved and pushed"; if it says held, read its file list, fix the line, save again (never push by hand).
# 7. CI on the pushed head (gate, smoke, toolchain), then `<base>` for brief 3 = that head.

# DEP-HOLD release, U7's Block 0b (main's .claude/CHECKPOINTS.md §3; u7-brief.md rev 1.1 section 5)
# security-auditor's answers, read in full (each ends DONE): .checkpoints/netcode-proof-test/security-u7-deps.md
# (items (7)-(10); Block 0b CLEAR WITH CONDITIONS; its binding check :281-316) and
# .checkpoints/netcode-proof-test/security-u7-0b-answer.md (Q1 the CLI's one download: <answer>; Q2 this commit before
# the CLI: <answer>; Q3 the database-date form: <answer>). Written 2026-11-25 <time> UTC by the main session after the
# binding check's steps 0-6 passed (evidence u7-block0b.md): the manifests exactly as item (7) (+3; +6 -1); `cargo
# metadata --format-version 1 --offline` exit 0; the lock `4	0`, npt-client-wasm's block only, 252 packages; the five
# trees as written (wasm-bindgen features default and std only; one shipped dependent; 20 names; the three
# dev-dependencies); `cargo check --workspace --all-targets --locked` 0 warnings; check.mjs test and deps equal before
# and after (measured 861, bites 958); cargo-deny 0.22.0 "advisories ok, bans ok, sources ok" (the seven duplicate
# names of U6c's run); cargo-audit 0.23.0, 252 crate dependencies, 0 vulnerabilities, one allowed warning
# RUSTSEC-2026-0441; databases fetched 2026-11-25 05:44:45 and 05:44:47 UTC (newest entry 2026-11-24T22:47:44+01:00);
# redact 371 items, 0 findings. The CLI (step 7): <its lines, or "recorded in an addendum below after its install">.
bd1e23353bdcf6606256619217965bcbe3931f508fafc92300372a45417bf59a  spikes/netcode-proof/Cargo.toml
55604a5fae45e50ee0fb262d743bc9e967b7caa3b23482de3b61de4cb28b9f5d  spikes/netcode-proof/Cargo.lock
d8af72f767830e066b360725f200dbdf44b1df77d2c7e0bcb5150661c77c000e  spikes/netcode-proof/crates/client-wasm/Cargo.toml
