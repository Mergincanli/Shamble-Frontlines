# Dependency check: cs4c-soldier (libm in `sim`; `content` in the tests of `room` and `client-endpoint`)

security-auditor's answer on exactly this content: `.checkpoints/cs4c-soldier/security-auditor-deps-4c-v2.md` (APPROVE,
2026-11-28; it names the same five digests, and the patch `deps-4c-v2.diff` at
0edaf8f3ff1610f6a29ae71ec22ae4d9028157d5d3e6e34672e9c723ce7c8b7b, which rebuilds exactly these five from a4e93679).
It builds on its first answer, `security-auditor-deps-4c.md`, on the first version (`deps-4c.diff`): the reworded
batch differs from it only in the root Cargo.toml's comment, which now names the sine and cosine (one routine,
`sincos`) for the spawns' facings beside the exponential (FACING-SPAWN (a)). No package from a registry, git or another
source is added, no locked version moves (56 packages before and after; three dependency-list lines in Cargo.lock), no
build script; libm's features are `default` and `arch` before and after; `sincos` needs no feature.

The batch is a patch in the notes, not applied: it is applied in the step that first needs it, after SIZE-4C. The
commit that applies it also holds (the answers' conditions, none of which this file checks):
- F1 (Medium): `crates/devtools/tests/dependency_rules.rs` updated and tightened in the same commit (sim's exact libm
  line, which also stops a renamed dependency such as `m = { package = "libm" }` getting past the scan; the root's
  `libm = "=0.2.16"` line to the byte; Cargo.lock's libm versions exactly ["0.2.16"]; only `sim` among our manifests
  names libm);
- F2 (Low): devtools' `tests/layering.rs`, which the room's and the client endpoint's comments name; and, for the root
  comment's words, one `maths-allowed.mjs` entry with its reason for each libm path the players' real build prints
  (expected `libm::math::exp`, `libm::math::sincos`, `libm::math::rem_pio2`, `libm::math::rem_pio2_large`; each an
  exact module path, never a bare `libm::math`), the exp and sincos vectors, and `maths-allowed.mjs:9-10` rewritten;
- the answer's item 3: the sim's source scan holds libm to the collision library's rule, stricter (named in one sim
  file, never imported, full-path calls only, one allowance per function and routine, every other libm path refused,
  with the bites and controls the answer lists), in this same commit, since it is what lets sim name libm at all;
  content's refusal of facings outside [0, 360) stays with its tests (it keeps `rem_pio2_large`, the one target-
  dependent routine, unreachable).
If any of these five files differs when the batch is applied (main moved them, or a comment is reworded), the digests
below no longer match and the batch goes to security-auditor again.

fc436ec36d3d54808012df5f6d830a11f80fb2e2272c8f9c0ec6975da35a49a8  Cargo.toml
b7c6ee5de5fe0a89ba819a1f67159d481763758bedfa67f607401a5a25468910  Cargo.lock
c7432b0346817b407417e38b264c462450c8168694b0f39a84b02218288a65c8  crates/sim/Cargo.toml
79b829a9ee954108082b0ec83aeeb1c4553fe67806b0c3dfc69f4c5d50319c5c  crates/room/Cargo.toml
22e2c5bf5795e9a93584dbf23abb4dc06645121d98563e5ca560f6b3790a58f9  crates/client-endpoint/Cargo.toml
