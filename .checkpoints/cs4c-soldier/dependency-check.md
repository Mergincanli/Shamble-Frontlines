# Dependency check: cs4c-soldier (libm in `sim`; `content` in the tests of `room` and `client-endpoint`)

security-auditor's answer on exactly this content: `.checkpoints/cs4c-soldier/security-auditor-deps-4c.md` (APPROVE,
2026-11-28; it names the same five digests, and the patch `deps-4c.diff` at 4865c3f9b8c52d4e223ffd77ddce718f0b3956c230d25392bc11620c0ac7e710,
which rebuilds exactly these five from 8bee8365). No package from a registry, git or another source is added, no
locked version moves (56 packages before and after; three dependency-list lines in Cargo.lock), no build script;
libm's features are `default` and `arch` before and after.

The batch is a patch in the notes, not applied: it is applied in the step that first needs it, after SIZE-4C. The
commit that applies it also holds (the answer's conditions, none of which this file checks):
- F1: `crates/devtools/tests/dependency_rules.rs` updated and tightened in the same commit (sim's exact libm line; the
  root's `libm = "=0.2.16"` line to the byte; Cargo.lock's libm versions exactly ["0.2.16"]; only `sim` among our
  manifests names libm);
- F2: devtools' `tests/layering.rs`, which the room's and the client endpoint's comments name, lands in the same
  commit; the root comment's "on the built-file check's reviewed list" becomes true when the exp entry lands.
If any of these five files differs when the batch is applied (main moved them, or the comments are reworded), the
digests below no longer match and the batch goes to security-auditor again.

099d224b85191d65ebd2f410c8e0c9a5bc5ef91d2def7ea136ed46aec56a85f5  Cargo.toml
b7c6ee5de5fe0a89ba819a1f67159d481763758bedfa67f607401a5a25468910  Cargo.lock
c7432b0346817b407417e38b264c462450c8168694b0f39a84b02218288a65c8  crates/sim/Cargo.toml
79b829a9ee954108082b0ec83aeeb1c4553fe67806b0c3dfc69f4c5d50319c5c  crates/room/Cargo.toml
22e2c5bf5795e9a93584dbf23abb4dc06645121d98563e5ca560f6b3790a58f9  crates/client-endpoint/Cargo.toml
