# Skeptic, review round 2, item R2-D1 (Medium; merges RR2-1, TE2-1, RR2-2), on 4bccf55

Plan: read the finding texts (review-r2-findings.md, rust-reviewer-review-r2.md :157-230, the ruling's D1 conditions 1
and 6 and :163-165); (a) my own route-A repro at opt-level "s" of RR2-1 (member + path dep outside the workspace, both
calling iter().sum() on u8) with controls (no dep; dep at opt-level 3; dep at "s" without sum); (b) read rustc 1.99.1's
share_generics() and upstream_monomorphization source at the tag; (c) my own repro of TE2-1's T: Add case at the shipped
opt-level 3; (d) RR2-2's membership rule against the Cargo book profiles page and cargo source; (e) severity with zero
reach today; (f) exact replacement text for Cargo.toml:21-24 and OVERFLOW-PER-CRATE's trigger. Own folder:
/home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D1/.

## Checks
- K0 read: review-r2-findings.md (R2-D1 row, TE2-1 full text); rust-reviewer-review-r2.md C1-C20 and RR2-1/RR2-2
  (:160-190); its share-controls.sh/.txt, share-more.sh/.txt, star.sh, star2-4bccf55.txt; test-engineer's
  orig-after/ext/extdep (add_trait, T: Add) and room probe; advisor-ruling-d1.md conditions 1, 3, 6 and :163-165
  ("Dependency GENERIC code instantiated in a workspace crate is untested. Wording for the row and comment:
  'dependencies keep the release default setting'"). The shipped comment at 4bccf55 Cargo.toml:21-24 adds ", unchecked".
  Records repeating the claim at 4bccf55: README:234 (OVERFLOW-ALL), report :19-20, :76-77, :254-257, REPO_MAP:16,
  STATUS_ARCHIVE:779-784 (OVERFLOW-PER-CRATE: "The first such crate that enters sim's dependency tree decides").
- K1 (a) own repro, my folder (archive of 4bccf55: rust-toolchain.toml Cargo.toml Cargo.lock crates; no tracked
  .cargo/ path), rustc/cargo 1.99.1, scripts/setup-a.sh + scripts/run-a.sh -> run-a.txt. room (member) gets
  #[inline(never)] member_total(&[u8]) = v.iter().copied().sum(); extsum (path crate OUTSIDE the ws root, own
  [workspace]) the same body as dep_total; input [200, 100] (300 -> 44), control [1, 2] = 3. Results:
  R0 route A + `opt-level = "s"` written into [profile.release]: flags room `opt-level=s -C overflow-checks=on`,
  extsum `opt-level=s`, no overflow flag; member_total(200+100) RETURNS 44 (wrapped). HOLDS.
  C1 route A as shipped (3): member_total panics (checked). C2 profile "s" + [profile.release.package.extsum]
  opt-level = 3: member_total panics (the dependency must also share). Matrix by env override: opt 0, 1, s, z wrap;
  2, 3 panic. dep_total wraps in every run (expected: "*").
