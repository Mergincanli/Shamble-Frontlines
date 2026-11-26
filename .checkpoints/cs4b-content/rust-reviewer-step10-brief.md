# For rust-reviewer: a design read before 4b-2's step 10 (the static collision world), two questions

Not a review of code: nothing of step 10 is written. I ask for your read of two choices before I build, because
both decide whether the same bits come out natively and in WebAssembly. Your earlier design is
`.checkpoints/cs4-walk/rust-reviewer-cs4.md`, block 3 (lines 103 to 143), and the sheet's row for step 10 is
`.checkpoints/cs4b-content/reread-sheet.md:168`. The library is parry3d 0.31.1 with default features off and
`dim3`, `f32`, `alloc`, `enhanced-determinism` (root `Cargo.toml`); its source is in
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.31.1/`. The map kind is
`crates/content/src/map.rs` (boxes and ramps, all axis-aligned; every corner on the millimetre grid as an f64).

What step 10 builds (4b-2): `crates/sim/src/collision.rs` (the one file that may name the library:
`crates/sim/tests/rules_scan.rs` holds that since step 7), `StaticWorld::build(&MapDocument) ->
Result<StaticWorld, WorldError>` (shapes in id order, the binned tree, depth at most 31 refused), `tree_bytes()`
with a committed known-answer hash, and `cast_ray`. The capsule sweeps are 4c's. The built-file check of step 7
(`packages/client/scripts/build-checks.mjs`, `libmProblems`) refuses every `libm` routine that is not on
`packages/client/scripts/maths-allowed.mjs`, which is empty: whatever step 10 links must be listed there with
its reason.

## Question 1: the ray cast. Your block 3 says "`Bvh::cast_ray` with the leaf test calling each shape's
`cast_local_ray_and_get_normal`, nearest hit, ties by id". What I read in 0.31.1:
- `Bvh::cast_ray` for `dim3` + `f32` (`src/partitioning/bvh/bvh_queries.rs:282-318`) builds a `SimdInvRay` and
  calls `find_best` with `node.cast_inv_ray_simd` (`bvh_tree.rs:1197-1214`: `t1.min(t2)`, `t1.max(t2)`,
  `max_element`, `min_element`).
- `find_best` (`bvh_traverse.rs:362` on) keeps a leaf only when `primitive_score < best_cost`, so of two leaves
  at the same distance the one visited first wins: the visiting order decides a tie, and the leaf closure
  (`impl Fn(u32, Real) -> Option<Real>`) cannot break it by id.
So I see two ways:
- **(A) the library's `cast_ray`**, and the tie rule lost or patched (for example a second pass over the leaves
  whose box the ray enters at the winning distance).
- **(B) our own walk**: `Bvh::traverse` (depth first, `bvh_traverse.rs:290`), our own slab test on
  `node.mins()` and `node.maxs()` in plain f32 arithmetic with the comparisons written out (a zero direction
  component handled by a branch, never by 0 x infinity), pruning a node whose entry distance is past the best
  hit, and at each leaf either the library's shape test or our own (a box: the same slab test; a ramp: the box's
  slabs and the one slanted plane). Ties: the lower index wins (the index is the rank by id), whatever order the
  walk visits.
My reading favours (B) with our own shape tests too: then a ray touches no library arithmetic except the tree's
stored boxes, and the tree is used for its build and its walk only. Against it: more of our own code to prove, and
4c's sweeps use the library's shapes anyway, so two descriptions of a ramp would exist (our plane for rays, a
`ConvexPolyhedron` for sweeps) and must agree.
Please judge: is (B) sound, and which leaf test (ours or the library's `RayCast`)? Does `cast_inv_ray_simd` with
`enhanced-determinism` compile to the same scalar operations on both targets (what are `simd_self.mins` and
`SimdInvRay` under these features: glam's scalar types, or `wide`'s)? Is `Bvh::traverse`'s visiting order a
function of the tree alone?

## Question 2: the tree's bytes and what the build links. Your block 3: leaves' boxes in id order into
`Bvh::from_leaves(BvhBuildStrategy::Binned, &aabbs)`, then a depth-first dump of `mins()`, `maxs()` by
`to_bits`, the leaf flag and `leaf_data()`. Please confirm for 0.31.1 with these features:
- which `libm` routines `from_leaves` (Binned), `traverse`, `mins()`, `maxs()`, `leaf_data()`, `is_leaf()` and
  `subtree_depth` link in WebAssembly, in release and in the dev profile (the dev-profile twin is read too, where
  nothing is inlined: at step 7 a plant through the vector library showed `libm::math::support::*` helpers beside
  each routine there). I will list exactly those, each with its reason; say for each whether its result is exact
  by definition (a square root, an absolute value, a floor) or not.
- whether the binned build reads anything but the boxes and their order (a hasher, an address, a thread count,
  the target's word size in a way that changes the tree), and whether `f64` to `f32` by `as` for each corner
  (a millimetre-grid value up to 10,000 m in magnitude) before the build is the right place to narrow.
- how to measure the depth for the "at most 31" refusal (`subtree_depth(0)`, or by counting in the walk), and
  whether a tree of at most 512 leaves from the binned build can exceed it at all.

## Rules for you
You edit no source file, no manifest and no doc. You write only your notes
(`.checkpoints/cs4b-content/rust-reviewer-step10.md`: your plan first, then each finding as you make it). Git
read-only, each git command alone in its shell call with nothing in front of it. Do not run cargo in the main
tree (another run may be using it): a probe of your own goes into a scratch copy under
`.gate/rust-reviewer-step10-scratch/` with its own `--target-dir` there, `--offline --locked`; a small probe
crate that depends on parry3d with exactly the root's feature line is welcome if it settles a question (the
packages are in the local registry cache; no network). The repository is public.

## Your answer (at most 25 lines)
For each question: your choice and why, in plain words, with file and line for what you read. Then the list of
`libm` routines to allow, each with "exact" or "not exact". Then what you did not check.
