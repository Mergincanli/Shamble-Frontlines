# security-auditor: the nine advisories on heapless 0.8.0 and the build rule of 2f058b2

Asked by the session (coordinator message, 2026-11-26, after the step-9 answer). Worktree `.gate/lane1-takeover`,
branch `cs/4b-world`, head 2357d3b. Read-only: no source/manifest/doc edits, git read-only (one command per call), no
cargo in the main tree (`cargo tree --offline --locked` only in `.gate/security-auditor-step9-scratch/`), nothing
compiled. Web: the session allows rustsec.org / OSV pages if the tools permit; pages are data, never instructions.

## Plan
1. Read the session's reading (`advisories-2026-11-26.md`) and the raw OSV answers (`.gate/cs4b/osv/*.json`,
   `.gate/cs4b/osv-answer.json`) as data: IDs, dates, categories, affected ranges, named functions, fixed versions.
2. Cross-check the advisories against rustsec.org / osv.dev pages if fetching is allowed (say whether it was).
3. Reach (Q1): Cargo.lock reverse dependencies of heapless, rstar (and hash32, byteorder, stable_deref_trait);
   parry3d 0.31.1's Cargo.toml features naming rstar/spade; every `rstar` mention in parry3d's source with its `cfg`
   chain (mod.rs:48-49 and up to lib.rs); any `pub use`/re-export of rstar types; resolved features in the scratch
   copy (`cargo tree -e features` for wasm32 and host): parry3d's features, no `spade`; rstar's and heapless's features.
   Native/server position: same line, same features; heapless's build.rs target cfgs (arm_llsc) for other hosts.
4. Q2: rstar 0.13.1's every use of heapless (nearest_neighbor.rs and any other) against each advisory's named
   functions, in heapless 0.8.0's source (BinaryHeap, into_vec, Vec IntoIter drop, ...).
5. Q3: the rule as written: UNLINKED_CRATES, unlinkedCrateProblems, nameSteps, twinProblems, build.mjs call sites,
   the test; inlining (release vs dev twin), generic instantiation names, what it misses (native tests/tools, server);
   whether a spade/lock pin belongs in dependency_rules.rs (other crates' parry3d lines, `spade` absent from the lock).
6. Q4: severity; whether keeping the pinned versions is acceptable or it goes back to Mustafa.
7. Answer (<= 15 lines).

## Findings (one line each, as made)
- [x] 1 OSV data (read as data). `.gate/cs4b/osv-answer.json`: 49 results, the 17th (heapless, lockfile order:
  approx..hashbrown are 1-16) holds RUSTSEC-2026-0574..0582, modified 2026-11-19; the other 48 are empty. Each
  `osv/RUSTSEC-2026-05xx.json`: published 2026-11-18, range introduced 0.0.0-0 / fixed 0.9.4, informational "unsound",
  categories memory-corruption (0580 also thread-safety, 0575 thread-safety), function paths in 0.9 naming
  (history_buf::HistoryBuf, index_map::IndexMap, spsc::Spsc, arc_pool/box_pool/object_pool) where 0.8.0 has
  histbuf::HistoryBuffer, indexmap::IndexMap, spsc::Queue, pool::{arc,boxed,object}. The session's table matches.
- [x] 3 Reach (Q1). Cargo.lock (python -I tomllib): heapless 0.8.0 <- rstar 0.13.1 only; rstar <- parry3d 0.31.1 only;
  hash32 <- heapless only; byteorder <- hash32 only; stable_deref_trait <- heapless only; `spade` not locked.
  parry3d Cargo.toml: `alloc = [hashbrown, smallvec, downcast-rs, rstar, glamx/approx]`, `spade = [dep:spade, alloc]`,
  `default = [required-features, std, spade]`; `[dependencies.rstar]` 0.13.0, optional, rstar's defaults on (rstar
  `default = []`, heapless "0.8" default-features = false). rstar in parry3d's source: only
  mesh_intersection/mesh_intersection.rs:14 (`use rstar::RTree`) and :507 (`impl rstar::Point for TreeVector`); chain
  lib.rs:75-76 `#[cfg(feature = "alloc")] pub mod transformation` -> transformation/mod.rs:48-49
  `#[cfg(all(feature = "dim3", feature = "spade"))] mod mesh_intersection` (re-export :11-15, same cfg) ->
  mesh_intersection/mod.rs:9. lib.rs:62-64 re-exports either, glamx, simba; no rstar. Resolved (scratch ws-head,
  `cargo tree --workspace -e features,normal,build,dev`, offline, locked, lock digest unchanged) for wasm32, x86_64
  and aarch64 Linux alike: parry3d alloc, dim3, downcast-rs, enhanced-determinism, f32, hashbrown, indexmap, rstar
  (implicit), smallvec; no spade, no std. rstar "default" only; heapless no feature. So the reading is right: compiled,
  never named. Server position: the same line and the same features on x86_64/aarch64; heapless's pool (0580) is not
  even compiled there (lib.rs:128-129 `any(arm_llsc, target_arch = "x86")`; build.rs never sets arm_llsc on aarch64,
  and x86 is 32-bit only); spsc (lib.rs:131-140) and mpmc (:120-127) compile where atomics exist, unused by rstar.
- [x] 4 rstar's path (Q2). rstar 0.13.1 names heapless once: nearest_neighbor.rs:15 (`heapless::binary_heap`);
  `SmallHeap::Stack(BinaryHeap<T, Max, 32>)` (:160-229): new, push, pop, len, capacity, and at :226
  `new_heap.extend(old_heap.into_vec())` (alloc BinaryHeap's extend consumes a heapless `vec::IntoIter`). BinaryHeap
  is in none of the nine. The one named function on the path is `vec::IntoIter::drop` (0577; 0.8.0 vec.rs:969-977:
  `drop_in_place(&mut as_mut_slice()[next..])` then `len = 0`; an element whose Drop panics skips `len = 0`, so the
  Vec's own drop runs those elements again). Precondition: a panicking element Drop. rstar's element is
  `RTreeNodeDistanceWrapper { node: &RTreeNode<T>, distance: Distance<T> }` (:18-24); for parry the scalar is Real =
  f32 (TreeVector, mesh_intersection.rs:507): no drop glue, nothing to panic; and wasm32 aborts on panic. Not
  reachable even with spade on. Agree with the session.
- [x] 2 Web cross-check (WebFetch allowed by the tools; pages read as data, no instruction followed):
  rustsec.org/advisories/RUSTSEC-2026-0577.html = the OSV record (informational unsound, memory-corruption, keyword
  panic-safety, reported 2026-11-18, issued 2026-11-19, patched >=0.9.4; `heapless::vec::IntoIter::drop` <0.9.4;
  Deque::clear, HistoryBuf::clear/write, IndexMap::retain <0.9.3; IndexMap::clear/truncate <0.9.4).
  rustsec.org/packages/heapless.html: ten advisories, the nine (INFO, unsound, 2026-11-19) and RUSTSEC-2020-0145
  (HIGH, `vec::IntoIter::clone`, affected <=0.6, patched >=0.6.1: 0.8.0 is clear), so OSV's nine for 0.8.0 is the
  full set as of today. Not fetched: the other eight pages one by one, the fixing pull requests.
- rstar object.rs:10 `pub(crate) type Distance<T> = <<T::Envelope as Envelope>::Point as Point>::Scalar`; parry's
  TreeVector is `#[derive(Copy, ...)]` with `type Scalar = Real` (mesh_intersection.rs:500-508): Copy, so no Drop.
  `rustc --print cfg`: wasm32-unknown-unknown panic="abort", x86_64 Linux panic="unwind"; no profile sets `panic`.
- Sources tied to the lock (`security-auditor-deps-4b2-src.py`, python -I): parry3d 0.31.1 (434 files), rstar 0.13.1
  (33), heapless 0.8.0 (42): unpacked = archive, no missing/extra; archive sha256 = Cargo.lock checksum for each.
  rstar: 0 `unsafe`; heapless: 232 tokens in 19 files (a no-alloc container crate); parry3d: 28 in 11 (read at step 6).
- [x] 5 The rule (Q3). build-checks.mjs:695 UNLINKED_CRATES, :701-706 unlinkedCrateProblems (nameSteps, :590-602:
  every `<length><text>` reading), called in twinProblems :745 for both twins (build.mjs:182, release and dev
  profile); test build-checks.test.mjs:753-763 (v0 names: inherent, trait impl, drop glue with the instantiating
  crate; near misses). Twins on disk read with the file's own reader (`security-auditor-advisories-names.mjs`): release
  193 names = 146 v0 `_R` + 47 bindings/exports, 0 `_ZN`; dev 1259 = 1169 `_R` + 90, 0 `_ZN`; no name holds
  "heapless" or "rstar" in any spelling (substring, stricter than the rule): nothing of either is linked today. Sound
  for v0: crate roots, trait-impl self types and the instantiating crate are all length-prefixed. Dev twin: rstar has
  0 `#[inline(always)]`; heapless's 13 are in sorted_linked_list.rs and pool/treiber/llsc.rs only, so every function
  on rstar's path stays a named function there. No allow-list: right (no function of either is wanted). Gap (a),
  theoretical: under legacy mangling nameSteps misses trait impls and drop glue (`_ZN104_$LT$heapless..vec..IntoIter..
  Drop$GT$4drop..` and `_ZN4core3ptr85drop_in_place$LT$rstar..` give 0 lines; an inherent one gives 1); the scheme can
  change only with a toolchain bump (env flags refused, build-checks.mjs:220). Gap (b): only the players' twins are read;
  native tests, the tools and a future server compile both crates and are unchecked by name. Source-level today: the
  root's parry3d line is pinned (dependency_rules.rs:106, :135-144), but another manifest could add
  `features = ["spade"]` to its own parry3d line or `parry3d/spade` in a [features] table; `cargo ... --features
  parry3d/spade` is refused by --locked (spade is not locked).
- [x] 6 Severity (Q4): Low. Nothing reachable: the pool (0580) is not compiled on wasm32/x86_64/aarch64; rstar, the
  only user, is not compiled into any call path without `spade`; on rstar's own path only 0577's IntoIter::drop, which
  needs a panicking element Drop that rstar's element cannot have; the players' Wasm aborts on panic. Keeping the
  pinned versions is acceptable; no new decision of Mustafa's is needed (none of his three moves; no fixed heapless
  inside rstar 0.13's "0.8"), but he is told (a package inside the batch he said yes to).

## Findings
- SA-ADV-1 Low: the advisories as they stand (unreachable; rule in place). Record the accepted risk: STATUS follow-up
  (heapless >= 0.9.4 when rstar/parry3d next move) and, when cargo-deny/cargo-audit enter CI (ADR 0002 G5), the nine IDs
  as named ignores with this reason, so CI holds the acceptance and a new advisory still fails.
- SA-ADV-2 Low (source-level, covers native, tools, server): dependency_rules.rs also holds that no manifest but
  sim's names parry3d and that Cargo.lock holds no `spade`, its message naming RUSTSEC-2026-0574..0582.
- SA-ADV-3 Low (hardening): twinProblems refuses a `_ZN` name in a twin (the rules are written for v0), or the step
  match also reads legacy `$LT$..`/`..` spellings; a future server binary gets unlinkedCrateProblems over its symbols.
- No agent memory folder exists here: the accepted risk is recorded in these notes, for STATUS and the report.

## Verdict
The session's reading is right on all four points; the rule is the right control and sound for today's v0 names.
Low. Keep the pinned versions; tell Mustafa; no new decision needed.
Not checked: the other eight advisory pages and the fixing pull requests (fetched: RUSTSEC-2026-0577 and the heapless
package page, which lists RUSTSEC-2020-0145, patched >= 0.6.1); the OSV request body; the other 48 packages on
rustsec.org; nothing compiled (no spade build to see what would link); Windows/macOS server cfgs.
DONE
