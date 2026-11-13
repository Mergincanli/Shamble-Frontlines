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
  staticlib no) when share_generics() is on, or for #[inline(never)] defs even when off (:389-398). Tie-break,
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
- K5 (c) scripts/run-c.sh -> run-c.txt: extgen (path crate OUTSIDE the root) with plain, trait_add<T: Add>,
  trait_sum<T: Sum>, concrete_in_generic<T> (concrete u8 `+`), and holds_u16 (extgen itself instantiates
  trait_add::<u16>); room calls each with an overflowing input. Route A as shipped (opt 3; flags: room
  -C overflow-checks=on, extgen none): trait_add::<u8> PANICS, trait_sum::<u8> PANICS, trait_add::<u16> PANICS;
  plain and concrete_in_generic wrap (0). Test profile: all five panic. Control trait_add::<u8>(1+2) = 3 (my label says
  "(wrapped)" for every Ok; 3 is the right sum). TE2-1 HOLDS at the shipped opt-level: a dependency's arithmetic that
  goes through a trait (Add, Sum) and that our crate instantiates is checked in release, so ", unchecked" is wrong
  there (more checks, not fewer). At env "s": trait_add::<u16> (an instance extgen already holds) WRAPS from room
  (room reuses extgen's copy), trait_add::<u8> still panics: TE2-1's case flips back at sharing opt-levels.
- K6 (d) Cargo book 1.99.1, fetched myself (doc.rust-lang.org/1.99.1/cargo/reference/{profiles,workspaces}.html, 200):
  profiles: 'To override the settings for all dependencies (but not any workspace member), use the "*" package name';
  precedence (first match wins): named package, then '"*" - For any non-workspace member', build-override, the
  profile, defaults. "Overrides and generics" even names this mechanism: "rustc has some optimizations where it will
  attempt to share monomorphized generics between crates. If the opt-level is 2 or 3, then a crate will not use
  monomorphized generics from other crates, nor will it export locally defined monomorphized items". workspaces: "All
  path dependencies residing in the workspace directory automatically become members"; `exclude` removes them. cargo
  source: the crate cargo-0.100.0 from static.crates.io (read only, never built), .cargo_vcs_info.json sha1
  5f94df4789f005f9a352888e8355ffc645b7ed0e = local cargo 1.99.1 (5f94df478): src/workspace/profiles.rs:496-538
  merge_toml_overrides: "*" merged only `if !is_member` (:510-514), then the one named spec (:515-536, later merge
  wins); src/workspace/workspace.rs:693-695 is_member = member_ids.contains; :943-997 find_path_deps: a path dependency
  whose folder is under the workspace root (or whose own root search lands here) and is not excluded becomes a member
  (:957-975); only path sources are followed (:986). RR2-2 HOLDS as a description; the shipped words "crates outside
  the workspace" are right in Cargo's terms (non-members), but ", so dependencies keep the release default" is wrong
  for an in-repo path dependency (a member, so checked). More checks, not fewer; reach zero. Low. I did not re-run the
  star.sh behaviour; the reviewer's star2-4bccf55.txt shows inside -> member, checked; patched/excluded -> unchecked.
- K7 sources for the class: library/core/src/iter/traits/iterator.rs:3664-3681 (1.99.1) Iterator::sum and ::product
  carry no #[inline]; accum.rs:47-91 the integer Sum/Product impls (no #[inline]) fold with a
  `#[rustc_inherit_overflow_checks]` closure; neither file has #[inline(never)], so opt 2 and 3 do not share them.
- K8 (f support) scripts/run-f.sh -> run-f.txt, route A, env "s" and "z": F1 + [profile.release.package.extsum]
  overflow-checks = true -> member_total panics at both; F2 "*" table removed -> member_total panics at both. Both
  closing options in the records close RR2-1 (and C2 above: the dependency at opt-level 3 closes it too).

## Draft verdict (before the advisor consult)
- Holds, Medium. RR2-1 reproduced (K1-K4: room's own sum and sim's inline code compiled in room wrap at 0, 1, "s",
  "z" in a release profile when a non-member dependency that room loads holds the same core instance; 2 and 3
  checked; no dependency, or the dependency at 3, or a named override, or no "*" -> checked; mechanism shown in the
  symbols and in rustc 1.99.1's source). TE2-1 reproduced at opt 3 (K5). RR2-2 confirmed in the Cargo book and cargo's
  source (K6), a wording point (Cargo's "outside the workspace" = non-member is right; "dependencies" is not = "non-
  members" for an in-repo path crate). Medium: zero reach today (no dependency; release at 3; "*" only in release),
  but ADR 0002 part 4 and PM-0002-29 plan "s"/"z" for the sim-wasm file with parry3d, and wasm-bindgen (first planned
  third-party crate) enters sim-wasm, outside sim's tree, which the trigger misses (K4); tested workarounds exist.
- Comment draft: comment-draft.txt in my folder (4 lines, 114-115 columns).
