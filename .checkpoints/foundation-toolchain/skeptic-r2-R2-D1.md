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
- K2 (a) control and mechanism: scripts/run-a2.sh, run-a3.sh -> run-a2.txt, run-a3.txt. C3 no dependency (room's
  manifest without extsum), route A + "s" and env 0, 1, z: member_total panics (checked) in all four, so std alone does
  not supply this instance. Symbol view (nm on the rlibs, room's LIB naming extsum::dep_total): at "s" room's object
  has `U <Copied<slice::Iter<u8>> as Iterator>::sum::<u8>` whose v0 name ends `...Iterator3sumhECsfHlATH1EdgS_6extsum`
  (instantiating crate: extsum), and extsum's rlib defines (T) it, plus `<u8 as Sum>::sum::<..>` and its fold; at 3
  room defines member_total locally and references `core::panicking::panic_const::panic_const_add_overflow`, extsum
  exports only dep_total. Caveat found on the way: when only room's `#[cfg(test)]` code names extsum, room's lib build
  does not load extsum and defines (T) its own checked copy, so "naming the dependency" is per compilation.
- K3 (b) rustc 1.99.1 sources fetched myself (raw.githubusercontent.com/rust-lang/rust/1.99.1/..., HTTP 200, into
  rustc-src-dl/): rustc_session/src/config.rs:1521-1529 share_generics(): `None => match self.optimize { OptLevel::No
  | OptLevel::Less | OptLevel::Size | OptLevel::SizeMin => true, OptLevel::More | OptLevel::Aggressive => false }`
  (-Zshare-generics overrides). rustc_middle/src/ty/instance.rs:214-254 upstream_monomorphization: local def -> None;
  `!share_generics() && inline != InlineAttr::Never` -> None; non-generic -> None; else
  `upstream_monomorphizations_for(def)`. Export side, rustc_codegen_ssa/src/back/symbol_export.rs:299-397: an rlib
  exports its generic instances (local_crate_exports_generics, context.rs:1415-1439: rlib/dylib yes; bin, cdylib,
  staticlib no) when share_generics() is on, or for #[inline(never)] defs even when off (:389-397). Tie-break,
  :498-556 upstream_monomorphizations_provider: over every loaded crate (`tcx.crates(())`, transitive ones included),
  several exporters of one instance -> the lowest stable_crate_id wins (:546-551), i.e. an arbitrary hash order, so a
  checked member's exported copy is no protection against a dependency's unchecked one downstream. The reviewer's C14
  lines match.
  (Correction to my K3 line: "no protection downstream" is read from the tie-break source, not run.)
- K4 (a, the trigger) scripts/run-a4.sh -> run-a4.txt: sim gets `#[inline] pub fn sim_total(&[u8]) -> u8` (the same
  sum); room depends on sim AND extsum, extsum is NOT in sim's dependency tree. Route A, env opt-level "s":
  sim::sim_total(200+100) called from room RETURNS 44 (wrapped); opt 3: panics; control at "s" with room depending on
  sim alone: panics. So at a sharing opt-level a third-party crate used by any workspace crate (here room) reaches
  sim's own code compiled in that crate, the RR-1 class again: OVERFLOW-PER-CRATE's trigger "the first such crate
  that enters sim's dependency tree" is too narrow, as RR2-1's fix (1) says.
