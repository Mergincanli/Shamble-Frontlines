# Skeptic r1, D10 (RR-2, Low): workspace lint opt-in vs pedantic on three crates only

Plan: read RR-2's full text from review-r1-findings.json; read ADR 0002 part 7 (words) and the head's Cargo.toml files (workspace + each crate) at 4d78ad6; check Cargo's documented [lints] / lints.workspace semantics (can a crate that sets `lints.workspace = true` add or override lints locally?); re-run cargo in a copy under .gate/skeptic-D10/ to show what the head actually enforces (pedantic on which crates); judge holds/severity; judge the proposed fix and what it would break.

## Checks
