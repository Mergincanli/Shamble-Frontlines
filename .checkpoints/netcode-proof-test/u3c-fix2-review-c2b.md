# test-engineer review: U3c last fix round, C2b (b5e4043)

Started 2026-11-08 (resumed after a usage limit; no earlier notes for this review existed). Branch
claude/netcode-proof-test-plan-hcky5g in the main folder, read only. Scratch copy under the session scratchpad
(c2b-review/), never the main tree. CARGO_TARGET_DIR for anything in the main folder: .gate/u3c-review-target.
No iptables.

Brief: advisor-u3c-plan.md "Side rulings: U3c's last fix round" (2): (a) C2R-5 self-test + honest comments
(C2R-6 / REG-2); (b) refusals RUSTC, CARGO_BUILD_RUSTC, workspace wrapper, config rustc / rustc-workspace-wrapper,
include, runners, each with a test, none refusing this container's env; forbid backstop + exceptionLintProblems,
kept only if 9/9; (c) C2R-9, C2R-10; (d) known limits documented. Fix notes: u3c-fix2-c2b.md.

## Plan
1. Read C2R-1..10 / REG-2 / REG-3 text (u3c-fix-review-result.txt :200-330) so each item's bar is known.
2. Read `git show b5e4043` (Cargo.toml, both exception manifests, check.mjs, check.test.mjs, tcpinfo, python).
3. Per item: locate the test; revert the fix in a scratch copy; run the test; record bite or no bite.
4. False positives: this container's env (env | grep CARGO/RUST), ~/.cargo/config.toml, a sccache-style setup,
   lowercase/alternate spellings of env names, CARGO_TARGET_<triple>_LINKER (must stay allowed?).
5. Backstop defeat: can the exception crates' own lint tables be changed in a way exceptionLintProblems accepts but
   that weakens the backstop (e.g. a priority key, a lint group, `unsafe_op_in_unsafe_fn`, a [lints] workspace=true
   alongside own tables, a [lints.rustdoc] table, a cfg'd target table)? Also: a non-exception crate dropping
   `workspace = true`, crate-level `#![allow(unsafe_code)]` under forbid, RUSTFLAGS --cap-lints / -A unsafe_code.
6. Reproduce check.mjs 9/9 in a scratch copy.
7. Verdict + table.

## Findings (as I go)
