# Skeptic r1, D11 (= RR-3, Low): default-features override in edition 2024 on 1.99.1, and feature unification across members

Plan: (1) read RR-3 in full and ADR 0002 part 3 (:212-217) plus the plan's 2.3/G5 text it leans on; (2) check the edition
guide's two quoted sentences against the source (web if reachable, else whatever the toolchain ships); (3) re-run the
repro offline in my own copy (.gate/skeptic-D11/, git archive 4d78ad6): a path crate depx with default = ["std"] in
[workspace.dependencies], sim takes { workspace = true, default-features = false }, then room takes { workspace = true };
cargo tree -e features per package and together, plus build output for warnings; (4) controls: the opposite direction,
member `features` keys, edition 2021, and a workspace entry with default-features = false; (5) judge the claim, the
severity, and the fix (completeness, what it breaks, a better one).

## Checks
