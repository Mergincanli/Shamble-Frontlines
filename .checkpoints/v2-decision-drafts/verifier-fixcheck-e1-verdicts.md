# verifier-fixcheck-e1 verdicts (ADR 0001, 0002, 0004; HEAD 59d41e0, base 4374a1c)

## Claims

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
END CLAIMS-HEADER (rows follow below in blocks)

## Rebuild (task 4)
- From 4374a1c, scratch copies in verifier-fixcheck-e1-scratch/docs/adr/, apply_fixes.py in this order:
  0001: stop-test-engineer-fixes.md (23 blocks for 0001, all ok), stop-netcode-architect-fixes.md (42, all ok).
  0004: stop-test-engineer-fixes.md (4, ok), stop-rust-reviewer-fixes.md (8, ok).
  0002: stop-test-engineer-fixes.md (0 for 0002), stop-engine-architect-a-fixes.md (13, ok), stop-test-engineer-0002-fixes.md (4, ok).
- cmp against HEAD 59d41e0: all three IDENTICAL, byte for byte.
END REBUILD
