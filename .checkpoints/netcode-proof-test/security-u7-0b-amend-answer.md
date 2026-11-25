# security-auditor: U7 Block 0b amendment (npt-link, npt-proto as dev-deps of npt-client-wasm), DEP-HOLD answer

Date: 2026-11-25. Effort xhigh. Read only; this file is my only write.

## Plan
1. Read DEP-HOLD rule (.claude/CHECKPOINTS.md :60-69), my earlier answer (security-u7-0b-answer.md), dependency-check.md.
2. Hash both amended files in .gate/u7-amend; compare to the stated sha256 values.
3. Diff the amended Cargo.toml and Cargo.lock against the branch's copies (content, not just numstat).
4. Read workspace Cargo.toml (:34, :37): confirm npt-link and npt-proto are path entries, no registry/git source.
5. Read npt-link and npt-proto manifests: confirm their own dependency sets add nothing third-party to the lock.
6. Read check.mjs :56-78 (deps step): confirm it reads shipped (normal) edges only and the R4 rule for npt-client-wasm.
7. Read pump.rs :372-378 and tests/common/mod.rs to confirm the stated need.
8. Re-check my earlier conditions (Q1 condition 5: --locked) against the amendment.
9. Answer Q1-Q3, findings on CLAUDE.md scale, DONE.

## Notes (one line per check)
