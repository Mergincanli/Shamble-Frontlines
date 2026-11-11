# U4d round 2 review: rust-reviewer (read only)

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597, uncommitted). Round 1 state: .gate/u4d-prefix.
Round 1 notes: u4d-review-rust.md (M1 F-movers, M2 F-claimed, L1-L9, N1-N5).

## Plan
1. Read rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304), W1-PUB (:353-436).
2. Read fixer's record (u4d-build.md "Fix pass, round 1") and s1-proto.md.
3. diff -ru u4d-prefix sim vs u4d-s1 sim; read the changed regions; check sha256 list.
4. git diff 21a8597 -- crates/proto (snapshot.rs, error.rs, tests).
5. Per round-1 finding: CLOSED / NOT CLOSED with file:line.
6. New checks: determinism, panics (debug_assert in record()), allocation (load_movers, decode cap), unsafe, clippy/clock bans, bites per instance, public items (A19) each needed now; proto Section/Groups/carries() private; new params.
7. Open points A/B/C weigh. Verdict.

## Progress
