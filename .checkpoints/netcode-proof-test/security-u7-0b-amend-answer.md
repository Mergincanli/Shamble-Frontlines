# security-auditor: U7 Block 0b amendment (npt-link, npt-proto as dev-deps of npt-client-wasm), DEP-HOLD answer

Date: 2026-11-25. Effort xhigh. Read only; this file is my only write. No Edit tool in this agent: this file is
rewritten whole with Write after each check.

## Plan
1. Read DEP-HOLD rule (.claude/CHECKPOINTS.md :60-69), my earlier answer (security-u7-0b-answer.md), dependency-check.md,
   security-u7-deps.md items (7)-(10) and the binding check.
2. Hash both amended files in .gate/u7-amend; compare to the stated sha256 values.
3. Diff the amended Cargo.toml and Cargo.lock against the branch's copies (content, not just numstat).
4. Read workspace Cargo.toml (:34, :37): confirm npt-link and npt-proto are path entries, no registry/git source.
5. Read npt-link and npt-proto manifests and lock blocks: confirm they add nothing third-party to the lock.
6. Read check.mjs :56-78 (deps step): confirm it reads shipped (normal) edges only and the R4 rule for npt-client-wasm.
7. Read pump.rs :372-378 and tests/common/mod.rs to confirm the stated need.
8. Re-check my earlier conditions (Q1 condition 5: --locked) against the amendment.
9. Answer Q1-Q3, findings on CLAUDE.md scale, DONE.

## Notes (one line per check)
- N1 (plan step 1): CHECKPOINTS.md :60-69 read: the hold binds content by sha256 per file, released by a
  dependency-check.md line naming the answer. dependency-check.md :105-139 read: Block 0b's three lines (:119-121,
  client-wasm/Cargo.toml d8af72f7...000e, lock 55604a5f...9f5d) and the CLI addendum (:122-139, "every workspace cargo
  call stays --locked (Q1 condition 5)"). security-u7-0b-answer.md read in full: Q1 conditions 1-6, Q2 conditions 1-4,
  Q3, SEC-U7B-1 (open, Critical, the guard's env unwrap). security-u7-deps.md item (7) :175-182 ("exactly" three
  dev-dependencies; "Nothing else in either file"), item (8) :184-190, item (9) :192-199, binding check :281-316 read.
