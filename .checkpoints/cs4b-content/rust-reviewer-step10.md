# rust-reviewer: design read before 4b-2 step 10 (static collision world), 2026-11-26
Worktree .gate/lane1-takeover, branch cs/4b-world, head 54e89d2. Brief: rust-reviewer-step10-brief.md.
Not a code review: nothing of step 10 exists. Two questions: Q1 the ray cast (A library cast_ray vs B own walk; leaf
test ours or library's), Q2 tree bytes + what the build links (libm list, binned build inputs, narrowing, depth).

## Plan
1. Memory check; sheet row reread-sheet.md:168; root Cargo.toml feature line; Cargo.lock versions of parry3d, glam,
   glamx, simba, wide, libm; crates/content/src/map.rs (the map kind); build-checks.mjs libmProblems +
   maths-allowed.mjs (what the check refuses and how it reads names: release twin, dev twin).
2. Q1 reading in parry3d-0.31.1: bvh_queries.rs cast_ray (282-318), SimdInvRay; bvh_tree.rs cast_inv_ray_simd
   (1197-1214), BvhNode layout, mins/maxs; bvh_traverse.rs traverse (290) + find_best (362); glam Vec3A under
   scalar-math (glamx re-export; f32.rs cfg), its min/max/min_element/max_element; Cuboid/ConvexPolyhedron RayCast
   paths (ray_cuboid / ray_aabb / ray_support_map) for operations used.
3. Q2 reading: bvh_binned_build.rs (from_leaves, Binned), bvh_tree.rs from_leaves/new, subtree_depth, leaf_data,
   is_leaf; any hasher/address/thread/usize use; f32 grid narrowing math.
4. Probe crate in .gate/rust-reviewer-step10-scratch/ (path deps from registry copy? no: registry dep with root's
   feature line, its own lockfile copied from the worktree's Cargo.lock so versions match, --offline --locked):
   exports calling from_leaves(Binned), traverse, mins/maxs/leaf_data/is_leaf/subtree_depth, cast_ray; build
   wasm32 release + dev; list imported/defined function names (libm::*), count. Second variant with Cuboid RayCast
   and ConvexPolyhedron RayCast to see what the library leaf test links.
5. Depth: can 512 leaves from binned build exceed 31? read the binned split (bins, fallback to median split?).
6. Answer (<= 25 lines) + not checked. DONE marker.

## Findings (a line each, as found)
