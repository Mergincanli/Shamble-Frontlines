# Skeptic, review round 2, item R2-D1 (Medium; merges RR2-1, TE2-1, RR2-2), on 4bccf55

Plan: read the finding texts (review-r2-findings.md, rust-reviewer-review-r2.md :157-230, the ruling's D1 conditions 1
and 6 and :163-165); (a) my own route-A repro at opt-level "s" of RR2-1 (member + path dep outside the workspace, both
calling iter().sum() on u8) with controls (no dep; dep at opt-level 3; dep at "s" without sum); (b) read rustc 1.99.1's
share_generics() and upstream_monomorphization source at the tag; (c) my own repro of TE2-1's T: Add case at the shipped
opt-level 3; (d) RR2-2's membership rule against the Cargo book profiles page and cargo source; (e) severity with zero
reach today; (f) exact replacement text for Cargo.toml:21-24 and OVERFLOW-PER-CRATE's trigger. Own folder:
/home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D1/.

## Checks
