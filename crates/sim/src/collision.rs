//! The static world of a match: the map's solid shapes as the simulation holds them, the tree over them and the ray
//! cast (ADR 0004 part 10; PM-0004-32), and the map's bounds in whole centimetres, the box a command's aim must lie
//! in. It is built once from the map document when a match starts, never changes after, and is never part of a
//! snapshot: the same document gives the same world again.
//!
//! This is the one file that names the collision library (`crates/sim/tests/rules_scan.rs` holds that). What it
//! takes from the library is the tree: the binned build over the shapes' boxes, the walk over its nodes, and the
//! depth. The arithmetic of a ray is ours, for three reasons rust-reviewer measured before this was written
//! (2026-11-27, on the pinned library): the library's own ray search drops a shape that is exactly as far as the
//! best one so far, so of two shapes at one distance the tree's layout picks the winner; it turns a direction's zero
//! into a large number where infinity is meant, so a ray that lies in a box's face misses the box; and its shape
//! tests work around a shape's centre, where the corners round. A ray here is exact in a plain sense: one function
//! reads a box's own stored corners, for a tree node and for a box alike, by a subtraction and a division per axis.
//!
//! A shape's index is its rank by id (bytes), whatever order the document lists the shapes in: two documents that
//! hold the same shapes give the same world, byte for byte ([`StaticWorld::tree_bytes`]).

use content::map::{
    AIM_HEIGHT_MAX_CM, AIM_REACH_CM, BOUNDS_GRID, Form, MapDocument, Toward, on_grid,
};
use parry3d::bounding_volume::Aabb;
use parry3d::math::Vector;
use parry3d::partitioning::{Bvh, BvhBuildStrategy, TraversalAction};

use crate::aim::AimCm;
use crate::vec::V3;

/// The most shapes a static world holds: the map kind's own cap (`content::map::MAX_SHAPES`; a test ties the two).
pub const MAX_SHAPES: usize = 512;

/// The largest a ray's numbers may be (2^20): every part of its origin and of its direction, in metres, and its
/// limit, in t (the ray reaches `origin + direction * t`, so t is metres only for a unit direction). Far beyond any
/// map (the loader keeps every coordinate within 10,000 m), and small enough that no product a ray's test makes of
/// such a number and a shape's size leaves `f32`'s finite range: a ray beyond it gives no hit, and a longer limit,
/// an endless one included, reaches this far in t (rust-reviewer's rounds of 2026-11-27).
pub const RAY_REACH: f32 = 1_048_576.0;

/// The deepest tree a static world may have. The library's walks keep the nodes they still owe on a stack of 32
/// places that needs no allocation; a deeper tree would make a query allocate inside the step. A map the loader
/// takes can be deeper (a chain of 34 one-centimetre cubes, each twice as far as the last, is 34 deep), so this
/// is a refusal that happens, not a formality.
pub const MAX_TREE_DEPTH: u32 = 31;

/// Why a map document gives no static world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldError {
    /// More shapes than [`MAX_SHAPES`]: this many.
    TooManyShapes(usize),
    /// The map's bounds have a corner that is no finite `f32`, or no thickness on some axis once their corners are
    /// `f32`: the checks a shape gets.
    Bounds,
    /// The shape at this place in the document's list has a corner that is no finite `f32`.
    NotFinite(usize),
    /// The shape at this place in the document's list has no thickness on some axis once its corners are `f32`.
    NoThickness(usize),
    /// The tree over the shapes is this deep: deeper than [`MAX_TREE_DEPTH`].
    TooDeep(u32),
}

impl WorldError {
    /// The reason in plain words.
    #[must_use]
    pub fn message(self) -> String {
        match self {
            Self::TooManyShapes(found) => {
                format!(
                    "the map holds {found} shapes, and a static world holds at most {MAX_SHAPES}"
                )
            }
            Self::Bounds => "the map's bounds have a corner that is not a finite number, or no thickness on one of their axes".to_owned(),
            Self::NotFinite(place) => {
                format!("shape {place} of the map has a corner that is not a finite number")
            }
            Self::NoThickness(place) => {
                format!("shape {place} of the map has no thickness on one of its axes")
            }
            Self::TooDeep(depth) => format!(
                "the tree over the map's shapes is {depth} deep, and at most {MAX_TREE_DEPTH} is taken: its shapes are spread too unevenly"
            ),
        }
    }
}

/// One solid shape as the simulation holds it: its box by its lowest and highest corner, in `f32`, and what fills
/// the box. These corners, and no others derived from them, are what the tree is built from and what a ray reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Solid {
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub form: Form,
}

/// Where a ray first meets the static world: the shape (its rank by id) and how far along the ray, in multiples
/// of the direction it was cast with (the point is `origin + direction * t`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
    pub shape: u32,
    pub t: f32,
}

/// The static world.
#[derive(Debug, Clone)]
pub struct StaticWorld {
    solids: Vec<Solid>,
    tree: Bvh,
    depth: u32,
    bounds_cm: Option<[[i32; 3]; 2]>,
}

/// A corner as the simulation holds it: narrowed once, here, and a zero without its sign (a file cannot hold a
/// minus zero, a document built in code can, and the sign would reach the tree's bytes). A spawn's point is
/// narrowed by the same rule (`soldiers::spawn_points`).
pub(crate) fn narrow(corner: [f64; 3]) -> [f32; 3] {
    corner.map(|value| {
        let value = value as f32;
        if value == 0.0 { 0.0 } else { value }
    })
}

/// One face of the map's bounds in whole centimetres, from the document's own number (never from its `f32`): on the
/// bounds' grid its count by the loader's own rule (content's `on_grid`: 0.55 gives 55, where a plain rounding up of
/// 0.55 x 100 gives 56), and off that grid, which only a document built in code can be, the next whole centimetre
/// inward: up for a lower face (`lower`), down for an upper one. Off the grid the count over 100 is never the number
/// itself (that is what off the grid means), so each comparison is strict. A face beyond `i64` saturates; the aim's
/// reach holds it after.
fn face_cm(value: f64, lower: bool) -> i64 {
    if let Some(steps) = on_grid(value, BOUNDS_GRID) {
        return steps;
    }
    let per_metre = f64::from(BOUNDS_GRID.per_metre);
    let nearest = (value * per_metre).round();
    let steps = nearest as i64;
    if lower && nearest / per_metre < value {
        steps.saturating_add(1)
    } else if !lower && nearest / per_metre > value {
        steps.saturating_sub(1)
    } else {
        steps
    }
}

/// The map's bounds in whole centimetres, held to the aim's reach (x and z within `AIM_REACH_CM` either way, y from 0
/// up to `AIM_HEIGHT_MAX_CM`: content's map.rs), as its lower corner and then its upper one. Every map the loader takes
/// lies inside the reach already, so there this is the bounds themselves; a map built in code past the reach keeps the
/// part inside it. `None` when that part holds no whole centimetre on some axis.
fn aim_box(min: [f64; 3], max: [f64; 3]) -> Option<[[i32; 3]; 2]> {
    let reach = [
        (-AIM_REACH_CM, AIM_REACH_CM),
        (0, AIM_HEIGHT_MAX_CM),
        (-AIM_REACH_CM, AIM_REACH_CM),
    ];
    let mut held = [[0; 3]; 2];
    for (axis, (least, most)) in reach.into_iter().enumerate() {
        let low = i64::max(face_cm(min[axis], true), i64::from(least));
        let high = i64::min(face_cm(max[axis], false), i64::from(most));
        if low > high {
            return None;
        }
        // Both lie within the reach now, which `i32` holds exactly.
        held[0][axis] = low as i32;
        held[1][axis] = high as i32;
    }
    Some(held)
}

/// Where a ray enters a box, between `t = 0` and `limit`, or nothing when it misses it there. One subtraction and
/// one division per axis, on the box's corners as they are stored; a direction of zero on an axis is a plain
/// question (is the origin between the two faces, the faces included) and never a multiplication of zero by
/// infinity. A ray that only touches a face or an edge meets the box.
fn enter_box(
    min: [f32; 3],
    max: [f32; 3],
    origin: [f32; 3],
    direction: [f32; 3],
    limit: f32,
) -> Option<(f32, f32)> {
    let (mut enter, mut leave) = (0.0_f32, limit);
    for axis in 0..3 {
        if direction[axis] == 0.0 {
            if origin[axis] < min[axis] || origin[axis] > max[axis] {
                return None;
            }
        } else {
            let first = (min[axis] - origin[axis]) / direction[axis];
            let second = (max[axis] - origin[axis]) / direction[axis];
            let (near, far) = if first > second {
                (second, first)
            } else {
                (first, second)
            };
            if near > enter {
                enter = near;
            }
            if far < leave {
                leave = far;
            }
            if enter > leave {
                return None;
            }
        }
    }
    Some((enter, leave))
}

/// Where a ray enters a ramp: inside its box (never before the box's own entry) and under its slope. The slope is
/// the plane through the box's low edge on the side the ramp rises from and its high edge on the side it rises
/// toward; a point is under it when `height above the box's floor * length <= way along the rise * height`.
fn enter_ramp(
    solid: &Solid,
    toward: Toward,
    origin: [f32; 3],
    direction: [f32; 3],
    limit: f32,
) -> Option<f32> {
    let (mut enter, mut leave) = enter_box(solid.min, solid.max, origin, direction, limit)?;
    // The axis the ramp rises along, and whether it rises toward that axis's high side.
    let (axis, up) = match toward {
        Toward::PlusX => (0, true),
        Toward::MinusX => (0, false),
        Toward::PlusZ => (2, true),
        Toward::MinusZ => (2, false),
    };
    let length = solid.max[axis] - solid.min[axis];
    let height = solid.max[1] - solid.min[1];
    // How far along the rise the origin is, and how fast the ray moves along it.
    let (along, along_rate) = if up {
        (origin[axis] - solid.min[axis], direction[axis])
    } else {
        (solid.max[axis] - origin[axis], -direction[axis])
    };
    // Above the slope when positive, on it when zero; `rate` is how that changes along the ray.
    let above = (origin[1] - solid.min[1]) * length - along * height;
    let rate = direction[1] * length - along_rate * height;
    // A ramp whose numbers leave `f32`'s finite range gives no hit, never one made of an infinity or a NaN (an
    // infinity over an infinity is a NaN, which passes every comparison below, and a number over an infinity a zero
    // of either sign). Only a document no loader takes can hold such a ramp: its corners stay within 10,000 m.
    if !(above.is_finite() && rate.is_finite()) {
        return None;
    }
    if rate == 0.0 {
        if above > 0.0 {
            return None;
        }
    } else {
        let crossing = above / -rate;
        if rate < 0.0 {
            // The ray comes down through the slope: it is inside from the crossing on.
            if crossing > enter {
                enter = crossing;
            }
        } else if crossing < leave {
            // The ray goes up through the slope: it is inside up to the crossing.
            leave = crossing;
        }
        if enter > leave {
            return None;
        }
    }
    Some(enter)
}

impl StaticWorld {
    /// Builds the static world of a map. The loader has already held a content file to more than this (its grid,
    /// its bounds, its ids); these are the checks a document needs whoever made it, a test included.
    ///
    /// # Errors
    /// Too many shapes, bounds that are not finite or have no thickness, a corner that is not finite, a shape without
    /// thickness, or a tree that is too deep.
    pub fn build(map: &MapDocument) -> Result<Self, WorldError> {
        if map.shapes.len() > MAX_SHAPES {
            return Err(WorldError::TooManyShapes(map.shapes.len()));
        }
        // The bounds get the checks a shape gets, on their corners as `f32`; the box an aim must lie in is taken
        // from the document's own numbers.
        let (low, high) = (narrow(map.bounds.min), narrow(map.bounds.max));
        if !low.iter().chain(&high).all(|corner| corner.is_finite())
            || (0..3).any(|axis| low[axis] >= high[axis])
        {
            return Err(WorldError::Bounds);
        }
        let bounds_cm = aim_box(map.bounds.min, map.bounds.max);
        let mut order: Vec<usize> = (0..map.shapes.len()).collect();
        order.sort_by(|a, b| {
            map.shapes[*a]
                .id
                .as_bytes()
                .cmp(map.shapes[*b].id.as_bytes())
        });
        let mut solids = Vec::with_capacity(order.len());
        for place in order {
            let shape = &map.shapes[place];
            let (min, max) = (narrow(shape.min), narrow(shape.max));
            if !min.iter().chain(&max).all(|corner| corner.is_finite()) {
                return Err(WorldError::NotFinite(place));
            }
            if (0..3).any(|axis| min[axis] >= max[axis]) {
                return Err(WorldError::NoThickness(place));
            }
            solids.push(Solid {
                min,
                max,
                form: shape.form,
            });
        }
        let corner = |at: [f32; 3]| Vector::new(at[0], at[1], at[2]);
        let boxes: Vec<Aabb> = solids
            .iter()
            .map(|solid| Aabb::new(corner(solid.min), corner(solid.max)))
            .collect();
        let tree = Bvh::from_leaves(BvhBuildStrategy::Binned, &boxes);
        let depth = tree.subtree_depth(0);
        if depth > MAX_TREE_DEPTH {
            return Err(WorldError::TooDeep(depth));
        }
        Ok(Self {
            solids,
            tree,
            depth,
            bounds_cm,
        })
    }

    /// The map's bounds in whole centimetres, held to the aim's reach: the box [`StaticWorld::holds_cm`] reads, its
    /// lower corner and then its upper one (x, y, z), both faces inside it. `None` when no whole centimetre of the
    /// reach lies within the bounds on some axis, which only a document built in code can give: then no aim is held.
    #[must_use]
    pub const fn bounds_cm(&self) -> Option<[[i32; 3]; 2]> {
        self.bounds_cm
    }

    /// Whether an aim lies in [`StaticWorld::bounds_cm`], both faces included: never on a world with no such box, and
    /// never "no aim yet" ([`AimCm::NONE`]), whose x lies past the aim's reach on every map.
    #[must_use]
    pub fn holds_cm(&self, aim: AimCm) -> bool {
        let at = [i32::from(aim.x), i32::from(aim.y), i32::from(aim.z)];
        self.bounds_cm.is_some_and(|[low, high]| {
            (0..3).all(|axis| low[axis] <= at[axis] && at[axis] <= high[axis])
        })
    }

    /// The shapes, by index: their rank by id.
    #[must_use]
    pub fn solids(&self) -> &[Solid] {
        &self.solids
    }

    /// How deep the tree is: 0 for no shape, 1 for one.
    #[must_use]
    pub const fn tree_depth(&self) -> u32 {
        self.depth
    }

    /// The tree as bytes, for the proof that every target builds the same one. Its nodes in the order the
    /// library's walk visits them (depth first); each node is its box's six `f32` by their bits (the low corner,
    /// then the high one; little-endian), one byte that says whether it is a leaf, and the leaf's shape index (all
    /// ones for a node that is none).
    #[must_use]
    pub fn tree_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.tree.traverse(|node| {
            for value in node
                .mins()
                .to_array()
                .into_iter()
                .chain(node.maxs().to_array())
            {
                bytes.extend_from_slice(&u32::to_le_bytes(value.to_bits()));
            }
            bytes.push(u8::from(node.is_leaf()));
            bytes.extend_from_slice(&u32::to_le_bytes(node.leaf_data().unwrap_or(u32::MAX)));
            TraversalAction::Continue
        });
        bytes
    }

    /// The first shape a ray meets between its origin and `origin + direction * limit`, or nothing. Of two shapes
    /// met at the same `t`, the one with the lower index wins: the answer is a property of the shapes and never
    /// of the tree's layout. A ray that starts inside a shape meets it at 0. It allocates nothing.
    ///
    /// The walk visits a node only while its box is entered no later than the best hit so far (a later node
    /// cannot hold a better hit: a node's box holds its children's, and a shape is met no earlier than its own
    /// box). A ray whose origin or direction holds a number beyond [`RAY_REACH`] (one that is not finite included)
    /// gives no hit; a limit beyond it reaches [`RAY_REACH`] in t.
    #[must_use]
    pub fn cast_ray(&self, origin: V3, direction: V3, limit: f32) -> Option<RayHit> {
        let (origin, direction) = (origin.to_array(), direction.to_array());
        // Written out, so that a NaN fails it as well.
        let within = origin
            .iter()
            .chain(&direction)
            .all(|value| -RAY_REACH <= *value && *value <= RAY_REACH);
        if !within || limit.is_nan() || limit < 0.0 || direction == [0.0; 3] {
            return None;
        }
        let limit = if limit > RAY_REACH { RAY_REACH } else { limit };
        let mut best: Option<RayHit> = None;
        self.tree.traverse(|node| {
            let reach = best.map_or(limit, |hit| hit.t);
            let (min, max) = (node.mins().to_array(), node.maxs().to_array());
            if enter_box(min, max, origin, direction, reach).is_none() {
                return TraversalAction::Prune;
            }
            if let Some(shape) = node.leaf_data() {
                let met = self
                    .solids
                    .get(shape as usize)
                    .and_then(|solid| match solid.form {
                        Form::Box => enter_box(solid.min, solid.max, origin, direction, reach)
                            .map(|(enter, _)| enter),
                        Form::Ramp { rises_toward } => {
                            enter_ramp(solid, rises_toward, origin, direction, reach)
                        }
                    });
                if let Some(t) = met
                    && best.is_none_or(|hit| t < hit.t || (t == hit.t && shape < hit.shape))
                {
                    best = Some(RayHit { shape, t });
                }
            }
            TraversalAction::Continue
        });
        best
    }
}

#[cfg(test)]
mod tests {
    use content::map::{Bounds, Shape, Spawn};
    use parry3d::query::{Ray, RayCast};
    use parry3d::shape::ConvexPolyhedron;

    use super::*;

    fn cube(id: &str, min: [f64; 3], max: [f64; 3]) -> Shape {
        Shape {
            id: id.to_owned(),
            min,
            max,
            form: Form::Box,
        }
    }

    fn ramp(id: &str, min: [f64; 3], max: [f64; 3], rises_toward: Toward) -> Shape {
        Shape {
            id: id.to_owned(),
            min,
            max,
            form: Form::Ramp { rises_toward },
        }
    }

    /// A document as a test builds one: it never passed the loader, and `build` must hold it all the same.
    fn map(shapes: Vec<Shape>) -> MapDocument {
        let bounds = Bounds {
            min: [-327.0, 0.0, -327.0],
            max: [327.0, 655.0, 327.0],
        };
        MapDocument {
            id: "test".to_owned(),
            bounds,
            spawns: Vec::new(),
            shapes,
            regions: Vec::new(),
        }
    }

    fn world(shapes: Vec<Shape>) -> StaticWorld {
        StaticWorld::build(&map(shapes)).expect("a world")
    }

    fn cast(world: &StaticWorld, origin: [f32; 3], direction: [f32; 3]) -> Option<(u32, f32)> {
        let v = |at: [f32; 3]| V3::new(at[0], at[1], at[2]);
        world
            .cast_ray(v(origin), v(direction), 1000.0)
            .map(|hit| (hit.shape, hit.t))
    }

    /// A yard of the kinds of shape a map has: two boxes that share a face's plane, a low step, a far post and
    /// one ramp toward each side.
    fn yard() -> Vec<Shape> {
        vec![
            cube("b_left", [-1.0, 0.0, 0.0], [0.0, 1.0, 1.0]),
            cube("a_right", [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
            cube("step", [4.0, 0.0, 4.0], [5.0, 0.55, 5.0]),
            cube("post", [40.0, 0.0, -30.0], [40.2, 1.5, -29.8]),
            ramp("ramp_px", [10.0, 0.0, 0.0], [14.0, 1.0, 2.0], Toward::PlusX),
            ramp(
                "ramp_mx",
                [10.0, 0.0, 4.0],
                [14.0, 1.0, 6.0],
                Toward::MinusX,
            ),
            ramp(
                "ramp_pz",
                [20.0, 0.0, 10.0],
                [22.0, 1.0, 14.0],
                Toward::PlusZ,
            ),
            ramp(
                "ramp_mz",
                [24.0, 0.0, 10.0],
                [26.0, 1.0, 14.0],
                Toward::MinusZ,
            ),
        ]
    }

    #[test]
    fn a_shapes_index_is_its_rank_by_id_whatever_the_documents_order() {
        let first = world(yard());
        let ids = [
            "a_right", "b_left", "post", "ramp_mx", "ramp_mz", "ramp_px", "ramp_pz", "step",
        ];
        assert_eq!(first.solids().len(), ids.len());
        assert_eq!(first.solids()[0].max, [1.0, 1.0, 1.0]);
        assert_eq!(
            first.solids()[5].form,
            Form::Ramp {
                rises_toward: Toward::PlusX
            }
        );
        // The same shapes turned around, and rotated by three: the same solids and the same tree, byte for byte.
        let mut turned = yard();
        turned.reverse();
        let mut rotated = yard();
        rotated.rotate_left(3);
        for other in [world(turned), world(rotated)] {
            assert_eq!(other.solids(), first.solids());
            assert_eq!(other.tree_bytes(), first.tree_bytes());
            assert_eq!(other.tree_depth(), first.tree_depth());
        }
        assert_eq!(MAX_SHAPES, content::map::MAX_SHAPES as usize);
    }

    #[test]
    fn the_trees_bytes_hold_every_node_and_each_leaf_holds_its_shapes_own_corners() {
        let built = world(yard());
        let bytes = built.tree_bytes();
        // A node is 24 bytes of corners, a leaf flag and an index.
        assert_eq!(bytes.len() % 29, 0);
        let mut seen = Vec::new();
        for node in bytes.as_chunks::<29>().0 {
            let number = |at: usize| {
                f32::from_bits(u32::from_le_bytes([
                    node[at],
                    node[at + 1],
                    node[at + 2],
                    node[at + 3],
                ]))
            };
            let index = u32::from_le_bytes([node[25], node[26], node[27], node[28]]);
            assert_eq!(node[24] == 1, index != u32::MAX);
            if node[24] == 1 {
                let solid = built.solids()[index as usize];
                assert_eq!([number(0), number(4), number(8)], solid.min);
                assert_eq!([number(12), number(16), number(20)], solid.max);
                seen.push(index);
            }
        }
        seen.sort_unstable();
        assert_eq!(seen, (0..8).collect::<Vec<u32>>());
        // No shape, one shape, two shapes: the small trees the library lays out in its own way.
        let none = world(Vec::new());
        assert_eq!((none.tree_depth(), none.tree_bytes().len()), (0, 0));
        assert_eq!(cast(&none, [0.0, 5.0, 0.0], [0.0, -1.0, 0.0]), None);
        let one = world(vec![cube("only", [2.0, 0.0, 2.0], [3.0, 1.0, 3.0])]);
        assert_eq!((one.tree_depth(), one.tree_bytes().len()), (1, 29));
        assert_eq!(
            cast(&one, [2.5, 5.0, 2.5], [0.0, -1.0, 0.0]),
            Some((0, 4.0))
        );
        // A ray through the map's origin meets nothing there: the one-shape tree holds no second, empty box.
        assert_eq!(cast(&one, [0.0, 5.0, 0.0], [0.0, -1.0, 0.0]), None);
        let two = world(yard().into_iter().take(2).collect());
        assert_eq!((two.tree_depth(), two.tree_bytes().len()), (2, 58));
    }

    #[test]
    fn a_minus_zero_in_a_document_leaves_no_sign_in_the_world() {
        let plain = world(vec![
            cube("a", [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
            cube("b", [-2.0, 0.0, 0.0], [0.0, 1.0, 1.0]),
        ]);
        let signed = world(vec![
            cube("a", [-0.0, -0.0, -0.0], [1.0, 1.0, 1.0]),
            cube("b", [-2.0, -0.0, -0.0], [-0.0, 1.0, 1.0]),
        ]);
        assert_eq!(signed.tree_bytes(), plain.tree_bytes());
        let zero = |world: &StaticWorld| {
            world
                .solids()
                .iter()
                .flat_map(|solid| solid.min.into_iter().chain(solid.max))
                .filter(|value| *value == 0.0)
                .map(f32::to_bits)
                .collect::<Vec<u32>>()
        };
        assert_eq!(zero(&signed), vec![0; 6]);
        // A number too small for `f32` narrows to zero, and its sign goes the same way.
        let tiny = world(vec![cube("a", [-1e-60, 0.0, 0.0], [1.0, 1.0, 1.0])]);
        assert_eq!(tiny.solids()[0].min[0].to_bits(), 0);
    }

    /// The low corners, in millimetres, of 34 one-centimetre cubes whose tree is a chain 34 deep: each cube lies so
    /// much farther out than the ones before it that the build splits it off alone (found by rust-reviewer's search
    /// on the pinned library, 2026-11-27). Every cube is on the map's grid and inside the aim's reach: a map the
    /// loader takes.
    #[rustfmt::skip]
    const CHAIN: [[i32; 3]; 34] = [[-5, 327665, -5], [-5, 327666, -5], [-5, 327667, -5], [-5, 327663, -5], [-5, 327671, -5], [-14, 327665, -5], [4, 327665, -5], [-5, 327665, -14], [-5, 327665, 4], [-5, 327648, -5], [-5, 327746, -5], [-5, 327530, -5], [-162, 327665, -5], [-5, 327665, -162], [189, 327665, -5], [-5, 327665, 189], [-5, 328456, -5], [-5, 326420, -5], [-1272, 327665, -5], [-5, 327665, -1272], [1924, 327665, -5], [-5, 327665, 1924], [-5, 335079, -5], [-5, 316071, -5], [-11621, 327665, -5], [-5, 327665, -11621], [18096, 327665, -5], [-5, 327665, 18096], [-5, 396773, -5], [-5, 219673, -5], [-108019, 327665, -5], [-5, 327665, -108019], [168719, 327665, -5], [-5, 327665, 168719]];

    fn chain(count: usize) -> Vec<Shape> {
        let metres = |millimetres: i32| f64::from(millimetres) / 1000.0;
        let cubes = CHAIN.iter().take(count).enumerate();
        cubes
            .map(|(at, low)| {
                cube(
                    &format!("cube_{at:02}"),
                    low.map(metres),
                    low.map(|value| metres(value + 10)),
                )
            })
            .collect()
    }

    #[test]
    fn a_document_that_gives_no_world_is_refused_by_name() {
        let refused = |shapes: Vec<Shape>| StaticWorld::build(&map(shapes)).expect_err("no world");
        let many = |count: usize| {
            (0..count)
                .map(|at| {
                    cube(
                        &format!("s{at:03}"),
                        [at as f64, 0.0, 0.0],
                        [at as f64 + 0.5, 1.0, 1.0],
                    )
                })
                .collect::<Vec<Shape>>()
        };
        assert_eq!(refused(many(513)), WorldError::TooManyShapes(513));
        assert_eq!(world(many(512)).solids().len(), 512);
        // The place named is the shape's place in the document, not its rank.
        let unit = || cube("z", [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e39, -1e39] {
            assert_eq!(
                refused(vec![unit(), cube("a", [bad, 0.0, 0.0], [1.0, 1.0, 1.0])]),
                WorldError::NotFinite(1),
                "{bad}"
            );
            assert_eq!(
                refused(vec![unit(), cube("a", [0.0, 0.0, 0.0], [1.0, bad, 1.0])]),
                WorldError::NotFinite(1),
                "{bad}"
            );
        }
        // No thickness: the same corner twice, the corners the wrong way round, and two doubles one `f32` apart.
        assert_eq!(
            refused(vec![cube("a", [0.0, 0.0, 0.0], [1.0, 0.0, 1.0])]),
            WorldError::NoThickness(0)
        );
        assert_eq!(
            refused(vec![
                unit(),
                unit(),
                cube("a", [2.0, 0.0, 0.0], [1.0, 1.0, 1.0])
            ]),
            WorldError::NoThickness(2)
        );
        assert_eq!(
            refused(vec![cube(
                "a",
                [100.0, 0.0, 0.0],
                [100.000_000_1, 1.0, 1.0]
            )]),
            WorldError::NoThickness(0)
        );
        // The deep chain: 31 cubes are taken at depth 31, one more is refused, and so is the whole chain.
        assert_eq!(world(chain(31)).tree_depth(), 31);
        assert_eq!(refused(chain(32)), WorldError::TooDeep(32));
        assert_eq!(refused(chain(34)), WorldError::TooDeep(34));
        assert_eq!(
            WorldError::TooDeep(34).message(),
            "the tree over the map's shapes is 34 deep, and at most 31 is taken: its shapes are spread too unevenly"
        );
        assert_eq!(
            WorldError::TooManyShapes(513).message(),
            "the map holds 513 shapes, and a static world holds at most 512"
        );
    }

    /// A document with these bounds, one spawn at their lower corner and no shape: the loader's own rules
    /// (`content::map::check`) can judge it.
    fn bounded(min: [f64; 3], max: [f64; 3]) -> MapDocument {
        MapDocument {
            id: "bounded".to_owned(),
            bounds: Bounds { min, max },
            spawns: vec![Spawn {
                id: "a".to_owned(),
                at: min,
                facing_deg: 0.0,
            }],
            shapes: Vec::new(),
            regions: Vec::new(),
        }
    }

    fn box_of(map: &MapDocument) -> Option<[[i32; 3]; 2]> {
        StaticWorld::build(map).expect("a world").bounds_cm()
    }

    /// The committed test scene's bounds (content/maps/test-scene.json), in metres and in centimetres.
    const SCENE: ([f64; 3], [f64; 3]) = ([-32.0, 0.0, -24.0], [80.0, 20.0, 24.0]);
    const SCENE_CM: [[i32; 3]; 2] = [[-3_200, 0, -2_400], [8_000, 2_000, 2_400]];

    #[test]
    fn bounds_on_the_grid_are_their_own_count_of_whole_centimetres() {
        // The made-up test map as the loader reads it.
        let set = content::load(content::bundle::test_bundle().as_bytes())
            .expect("the test bundle loads");
        let loaded = set.map("test-scene").expect("its map");
        assert_eq!(
            box_of(loaded),
            Some([[-5_000, 0, -5_000], [5_000, 2_000, 5_000]])
        );
        // The test scene's bounds, the reach's own edges, and lower faces at 0.55 and -0.55: 0.55 x 100 is not 55 to
        // the last bit, so a plain rounding up would put the face on 56.
        let face = 0.55_f64;
        assert_eq!((face * 100.0).ceil(), 56.0);
        #[rustfmt::skip]
        let cases = [
            (SCENE.0, SCENE.1, SCENE_CM),
            ([-327.67, 0.0, -327.67], [327.67, 655.35, 327.67], [[-32_767, 0, -32_767], [32_767, 65_535, 32_767]]),
            ([0.55, 0.55, -0.55], [1.0, 1.0, 1.0], [[55, 55, -55], [100, 100, 100]]),
        ];
        for (min, max, expected) in cases {
            let map = bounded(min, max);
            content::map::check(&map).expect("the loader takes it");
            assert_eq!(box_of(&map), Some(expected), "{min:?} {max:?}");
            // Each face is the loader's own count, exactly.
            for axis in 0..3 {
                let count = |value: f64| on_grid(value, BOUNDS_GRID);
                assert_eq!(count(min[axis]), Some(i64::from(expected[0][axis])));
                assert_eq!(count(max[axis]), Some(i64::from(expected[1][axis])));
            }
        }
    }

    #[test]
    fn bounds_off_the_grid_take_the_next_whole_centimetre_inward() {
        // Documents built in code, which the loader refuses: every face lies inward of the document's own number by
        // less than 1 cm, judged in f64 against that number.
        #[rustfmt::skip]
        let cases = [
            ([-32.004, 0.005, -24.0], [80.004, 20.001, 24.0], [[-3_200, 1, -2_400], [8_000, 2_000, 2_400]]),
            ([-0.015, 0.009, 0.001], [0.019, 0.011, 0.999], [[-1, 1, 1], [1, 1, 99]]),
        ];
        let metres = |cm: i32| f64::from(cm) / 100.0;
        for (min, max, expected) in cases {
            let map = bounded(min, max);
            assert!(content::map::check(&map).is_err(), "{min:?} {max:?}");
            let held = box_of(&map).expect("a box");
            assert_eq!(held, expected, "{min:?} {max:?}");
            for axis in 0..3 {
                let (low, high) = (held[0][axis], held[1][axis]);
                assert!(
                    metres(low) >= min[axis] && metres(low - 1) < min[axis],
                    "{axis}: {low}"
                );
                assert!(
                    metres(high) <= max[axis] && metres(high + 1) > max[axis],
                    "{axis}: {high}"
                );
            }
        }
    }

    #[test]
    fn the_aims_box_lies_inside_the_bounds_and_the_reach_and_a_map_past_the_reach_still_builds() {
        let reach = [
            [-AIM_REACH_CM, 0, -AIM_REACH_CM],
            [AIM_REACH_CM, AIM_HEIGHT_MAX_CM, AIM_REACH_CM],
        ];
        // A 2 km map built in code (0 to 2,000 m on x and z) builds, and its box is the part inside the reach; so is a
        // map reaching past the reach on every side, below y 0 included.
        #[rustfmt::skip]
        let cases = [
            ([0.0, 0.0, 0.0], [2_000.0, 20.0, 2_000.0], [[0, 0, 0], [32_767, 2_000, 32_767]]),
            ([-2_000.0, -5.0, -2_000.0], [2_000.0, 700.0, 2_000.0], reach),
            ([-32.004, 0.005, -24.0], [80.004, 20.001, 24.0], [[-3_200, 1, -2_400], [8_000, 2_000, 2_400]]),
            (SCENE.0, SCENE.1, SCENE_CM),
        ];
        for (min, max, expected) in cases {
            let held = box_of(&bounded(min, max)).expect("a box");
            assert_eq!(held, expected, "{min:?} {max:?}");
            for axis in 0..3 {
                let (low, high) = (held[0][axis], held[1][axis]);
                assert!(reach[0][axis] <= low && low <= high && high <= reach[1][axis]);
                assert!(
                    f64::from(low) / 100.0 >= min[axis] && f64::from(high) / 100.0 <= max[axis]
                );
            }
        }
        // Bounds wholly past the reach on one axis, or holding no whole centimetre on one, give no box, and the world
        // still builds.
        for (min, max) in [
            ([400.0, 0.0, 0.0], [500.0, 10.0, 10.0]),
            ([0.0, 700.0, 0.0], [1.0, 800.0, 1.0]),
            ([0.0, 0.0, -0.009], [1.0, 1.0, -0.001]),
        ] {
            assert_eq!(box_of(&bounded(min, max)), None, "{min:?} {max:?}");
        }
    }

    #[test]
    fn a_face_moved_by_one_centimetre_moves_that_face_of_the_box_alone() {
        let metres = |cm: i32| f64::from(cm) / 100.0;
        for side in 0..2 {
            for axis in 0..3 {
                for by in [-1, 1] {
                    let mut moved_cm = SCENE_CM;
                    moved_cm[side][axis] += by;
                    if moved_cm[0][1] < 0 {
                        continue;
                    }
                    let (mut min, mut max) = SCENE;
                    let face = if side == 0 { &mut min } else { &mut max };
                    face[axis] = metres(moved_cm[side][axis]);
                    let map = bounded(min, max);
                    content::map::check(&map).expect("the loader takes it");
                    assert_eq!(box_of(&map), Some(moved_cm), "{side} {axis} {by}");
                }
            }
        }
    }

    #[test]
    fn an_aim_is_held_on_each_face_of_the_box_and_refused_one_centimetre_past_it() {
        let build = |min, max| StaticWorld::build(&bounded(min, max)).expect("a world");
        let aim = |x: i16, y: u16, z: i16| AimCm { x, y, z };
        let scene = build(SCENE.0, SCENE.1);
        // Each face and the two far corners are held; one centimetre past each face is not (y 0 has nothing below it
        // in an aim's type: the raised map below has its lower y face at 2 m).
        for held in [
            aim(-3_200, 10, 0),
            aim(8_000, 10, 0),
            aim(0, 0, 0),
            aim(0, 2_000, 0),
            aim(0, 10, -2_400),
            aim(0, 10, 2_400),
            aim(-3_200, 0, -2_400),
            aim(8_000, 2_000, 2_400),
        ] {
            assert!(scene.holds_cm(held), "{held:?}");
        }
        for past in [
            aim(-3_201, 10, 0),
            aim(8_001, 10, 0),
            aim(0, 2_001, 0),
            aim(0, 10, -2_401),
            aim(0, 10, 2_401),
        ] {
            assert!(!scene.holds_cm(past), "{past:?}");
        }
        let raised = build([-1.0, 2.0, -1.0], [1.0, 4.0, 1.0]);
        assert!(raised.holds_cm(aim(0, 200, 0)) && raised.holds_cm(aim(0, 400, 0)));
        assert!(!raised.holds_cm(aim(0, 199, 0)) && !raised.holds_cm(aim(0, 401, 0)));
        // "No aim yet", and any aim with the type's least x, is held on no map, the widest one included.
        let widest = build([-327.67, 0.0, -327.67], [327.67, 655.35, 327.67]);
        assert!(widest.holds_cm(aim(-32_767, 65_535, 32_767)));
        for world in [&scene, &raised, &widest] {
            assert!(!world.holds_cm(AimCm::NONE));
            assert!(!world.holds_cm(aim(i16::MIN, 10, 0)));
        }
        // A world with no box holds no aim at all.
        let past = build([400.0, 0.0, 0.0], [500.0, 10.0, 10.0]);
        assert_eq!(past.bounds_cm(), None);
        assert!(!past.holds_cm(aim(32_767, 0, 0)) && !past.holds_cm(aim(0, 0, 0)));
    }

    #[test]
    fn bounds_that_are_not_finite_or_flat_are_refused() {
        let refused = |min, max| StaticWorld::build(&bounded(min, max)).expect_err("no world");
        let (min, max) = SCENE;
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e39, -1e39] {
            for axis in 0..3 {
                let (mut low, mut high) = (min, max);
                low[axis] = bad;
                assert_eq!(refused(low, max), WorldError::Bounds, "{bad} {axis}");
                high[axis] = bad;
                assert_eq!(refused(min, high), WorldError::Bounds, "{bad} {axis}");
            }
        }
        // No thickness: one face twice, the faces the wrong way round, and two doubles one `f32` apart.
        assert_eq!(
            refused([0.0, 1.0, 0.0], [1.0, 1.0, 1.0]),
            WorldError::Bounds
        );
        assert_eq!(
            refused([0.0, 0.0, 2.0], [1.0, 1.0, 1.0]),
            WorldError::Bounds
        );
        assert_eq!(
            refused([100.0, 0.0, 0.0], [100.000_000_1, 1.0, 1.0]),
            WorldError::Bounds
        );
        assert_eq!(
            WorldError::Bounds.message(),
            "the map's bounds have a corner that is not a finite number, or no thickness on one of their axes"
        );
    }

    #[test]
    fn a_ray_meets_the_first_shape_on_its_way_and_the_lower_index_of_two_at_one_distance() {
        let built = world(yard());
        // Straight down onto a box, onto the low step, and past everything.
        assert_eq!(
            cast(&built, [0.5, 10.0, 0.5], [0.0, -1.0, 0.0]),
            Some((0, 9.0))
        );
        assert_eq!(
            cast(&built, [4.5, 10.0, 4.5], [0.0, -1.0, 0.0]),
            Some((7, 9.45))
        );
        assert_eq!(cast(&built, [7.0, 10.0, 7.0], [0.0, -1.0, 0.0]), None);
        // Along x through both boxes: the nearer one, from either side.
        assert_eq!(
            cast(&built, [-5.0, 0.5, 0.5], [1.0, 0.0, 0.0]),
            Some((1, 4.0))
        );
        assert_eq!(
            cast(&built, [5.0, 0.5, 0.5], [-1.0, 0.0, 0.0]),
            Some((0, 4.0))
        );
        // The two boxes share the plane x = 0. A ray down that plane meets both at one distance: the lower index,
        // "a_right", wins. The library's own search loses this ray altogether (the direction's zero on x).
        assert_eq!(
            cast(&built, [0.0, 10.0, 0.5], [0.0, -1.0, 0.0]),
            Some((0, 9.0))
        );
        // The same with a minus zero on x, in the direction and in the origin: a zero is a zero, whatever its sign.
        assert_eq!(
            cast(&built, [0.0, 10.0, 0.5], [-0.0, -1.0, 0.0]),
            Some((0, 9.0))
        );
        assert_eq!(
            cast(&built, [-0.0, 10.0, 0.5], [-0.0, -1.0, -0.0]),
            Some((0, 9.0))
        );
        assert_eq!(
            cast(&built, [0.0, 0.5, -3.0], [0.0, 0.0, 1.0]),
            Some((0, 3.0))
        );
        // A ray in a face's plane, an edge, a corner: touching is meeting.
        assert_eq!(
            cast(&built, [1.0, 10.0, 0.5], [0.0, -1.0, 0.0]),
            Some((0, 9.0))
        );
        assert_eq!(
            cast(&built, [1.0, 10.0, 1.0], [0.0, -1.0, 0.0]),
            Some((0, 9.0))
        );
        assert_eq!(cast(&built, [1.000_001, 10.0, 0.5], [0.0, -1.0, 0.0]), None);
        // From inside a shape: at once. The direction's length scales `t`; the point stays.
        assert_eq!(
            cast(&built, [0.5, 0.5, 0.5], [0.0, 1.0, 0.0]),
            Some((0, 0.0))
        );
        assert_eq!(
            cast(&built, [0.5, 10.0, 0.5], [0.0, -4.0, 0.0]),
            Some((0, 2.25))
        );
        // The far post, from the yard's other end, on a slanted way.
        assert_eq!(
            cast(&built, [40.1, 1.0, 10.0], [0.0, 0.0, -1.0]),
            Some((2, 39.8))
        );
        let slanted = cast(&built, [0.1, 21.0, -9.9], [2.0, -1.0, -1.0]).expect("the post");
        assert_eq!(slanted.0, 2);
        assert!((slanted.1 - 19.95).abs() < 1e-4, "{slanted:?}");
    }

    #[test]
    fn the_limit_and_numbers_that_are_none_are_held() {
        let built = world(yard());
        let v = |x: f32, y: f32, z: f32| V3::new(x, y, z);
        let down = v(0.0, -1.0, 0.0);
        // The box's top is 9 away: a limit of exactly 9 reaches it, one step of an `f32` less does not.
        assert_eq!(
            built.cast_ray(v(0.5, 10.0, 0.5), down, 9.0),
            Some(RayHit { shape: 0, t: 9.0 })
        );
        assert_eq!(
            built.cast_ray(
                v(0.5, 10.0, 0.5),
                down,
                f32::from_bits(9.0_f32.to_bits() - 1)
            ),
            None
        );
        assert_eq!(
            built.cast_ray(v(0.5, 0.5, 0.5), down, 0.0),
            Some(RayHit { shape: 0, t: 0.0 })
        );
        for limit in [-1.0, -0.001, f32::NAN, f32::NEG_INFINITY] {
            assert_eq!(
                built.cast_ray(v(0.5, 10.0, 0.5), down, limit),
                None,
                "{limit}"
            );
        }
        assert_eq!(
            built.cast_ray(v(0.5, 10.0, 0.5), down, f32::INFINITY),
            Some(RayHit { shape: 0, t: 9.0 })
        );
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(built.cast_ray(v(bad, 10.0, 0.5), down, 100.0), None);
            assert_eq!(
                built.cast_ray(v(0.5, 10.0, 0.5), v(0.0, bad, 0.0), 100.0),
                None
            );
        }
        assert_eq!(
            built.cast_ray(v(0.5, 0.5, 0.5), v(0.0, 0.0, 0.0), 100.0),
            None
        );
        assert_eq!(
            built.cast_ray(v(0.5, 0.5, 0.5), v(0.0, -0.0, 0.0), 100.0),
            None
        );
    }

    /// A ray's numbers stay within [`RAY_REACH`] (rust-reviewer's round of 2026-11-27): within it the verdict does
    /// not depend on the direction's length; beyond it there is no hit, never one made of an overflow; and a ramp
    /// whose own numbers overflow gives no hit either.
    #[test]
    fn a_ray_beyond_the_reach_and_a_ramp_beyond_f32_give_no_hit() {
        let built = world(yard());
        let v = |x: f32, y: f32, z: f32| V3::new(x, y, z);
        // The yard's shapes by id: a_right, b_left, post, ramp_mx, ramp_mz, ramp_px, ramp_pz, step.
        let ramp_px = 5;
        // rust-reviewer's probe: from the top of the box of the ramp rising toward +x, over its slope, up and away.
        let over = [10.5, 1.0, 1.0];
        for scale in [1.0, 1e5] {
            assert_eq!(
                cast(&built, over, [2.0 * scale, 3.0 * scale, 0.0]),
                None,
                "{scale}"
            );
        }
        // Down onto the slope (its height at x = 13 is 0.75): the same shape, the point the same, `t` by the length.
        assert_eq!(
            cast(&built, [13.0, 3.0, 1.0], [0.0, -1.0, 0.0]),
            Some((ramp_px, 2.25))
        );
        assert_eq!(
            cast(&built, [13.0, 3.0, 1.0], [0.0, -1e5, 0.0]),
            Some((ramp_px, 2.25e-5))
        );
        // Beyond the reach: no hit, where the overflow gave the box's entry as a hit (`inf - inf` is NaN).
        assert_eq!(cast(&built, over, [2e38, 3e38, 0.0]), None);
        assert_eq!(cast(&built, [13.0, 3.0, 1.0], [0.0, -2e6, 0.0]), None);
        let edge = cast(&built, [13.0, 3.0, 1.0], [0.0, -RAY_REACH, 0.0]).expect("at the reach");
        assert_eq!(edge.0, ramp_px);
        // The next float above the reach, by its bits (ADR 0004 part 7's list holds `to_bits`, not `next_up`).
        let beyond = f32::from_bits(RAY_REACH.to_bits() + 1);
        assert_eq!(cast(&built, [13.0, 3.0, 1.0], [0.0, -beyond, 0.0]), None);
        assert_eq!(
            built.cast_ray(v(beyond, 3.0, 1.0), v(0.0, -1.0, 0.0), 10.0),
            None
        );
        // A direction so short that no shape is within reach: an endless limit reaches `RAY_REACH`, not infinity, so
        // the box ahead is no hit at an endless `t`.
        assert_eq!(
            built.cast_ray(v(-20.0, 0.5, 0.5), v(1e-45, 0.0, 0.0), f32::INFINITY),
            None
        );
        assert_eq!(
            built.cast_ray(v(-20.0, 0.5, 0.5), v(1.0, 0.0, 0.0), f32::INFINITY),
            Some(RayHit { shape: 1, t: 19.0 })
        );
        // A ramp as large as `f32` goes: its length and height overflow, and the ray from its middle gets no hit.
        let huge = world(vec![ramp(
            "huge",
            [-3e38, -3e38, -3e38],
            [3e38, 3e38, 3e38],
            Toward::PlusX,
        )]);
        assert_eq!(
            huge.cast_ray(v(0.0, 0.0, 0.0), v(0.0, -1.0, 0.0), 100.0),
            None
        );
        // Finite terms whose sums overflow (rust-reviewer's round 2): above and rate both infinite give a NaN
        // crossing, rate alone a -0.0 one; each gave the box's entry as a hit, and the truth is none.
        let tall = world(vec![ramp(
            "tall",
            [0.0, -1e33, -1.0],
            [1e33, 1e33, 1.0],
            Toward::PlusX,
        )]);
        assert_eq!(
            tall.cast_ray(v(0.0, 0.0, 0.0), v(0.0, -RAY_REACH, 0.0), 100.0),
            None
        );
        let long = world(vec![ramp(
            "long",
            [0.0, 0.0, -1.0],
            [1e33, 1.0, 1.0],
            Toward::PlusX,
        )]);
        assert_eq!(
            long.cast_ray(v(0.0, 0.5, 0.0), v(0.0, RAY_REACH, 0.0), 100.0),
            None
        );
    }

    #[test]
    fn a_ramp_is_met_on_its_slope_its_faces_and_its_floor_toward_each_of_the_four_sides() {
        let built = world(yard());
        // Toward +x: 4 long, 1 high, from x = 10 to x = 14. Straight down, the slope is a quarter of the way along.
        assert_eq!(
            cast(&built, [12.0, 10.0, 1.0], [0.0, -1.0, 0.0]),
            Some((5, 9.5))
        );
        assert_eq!(
            cast(&built, [11.0, 10.0, 1.0], [0.0, -1.0, 0.0]),
            Some((5, 9.75))
        );
        assert_eq!(
            cast(&built, [14.0, 10.0, 1.0], [0.0, -1.0, 0.0]),
            Some((5, 9.0))
        );
        assert_eq!(
            cast(&built, [10.0, 10.0, 1.0], [0.0, -1.0, 0.0]),
            Some((5, 10.0))
        );
        // Level at half its height: from the low side the slope is met halfway along; from the high side, the face.
        assert_eq!(
            cast(&built, [5.0, 0.5, 1.0], [1.0, 0.0, 0.0]),
            Some((5, 7.0))
        );
        assert_eq!(
            cast(&built, [20.0, 0.5, 1.0], [-1.0, 0.0, 0.0]),
            Some((5, 6.0))
        );
        // Over the slope and never under it: no meeting. Up from under the floor: the floor.
        assert_eq!(cast(&built, [9.0, 0.8, 1.0], [1.0, 0.1, 0.0]), None);
        assert_eq!(
            cast(&built, [13.0, -3.0, 1.0], [0.0, 1.0, 0.0]),
            Some((5, 3.0))
        );
        // Inside the box and above the slope, going up: nothing. Going down: the slope. Under the slope: at once.
        assert_eq!(cast(&built, [11.0, 0.9, 1.0], [0.0, 1.0, 0.0]), None);
        assert_eq!(
            cast(&built, [11.0, 0.9, 1.0], [0.0, -1.0, 0.0]).map(|(shape, _)| shape),
            Some(5)
        );
        assert_eq!(
            cast(&built, [13.0, 0.25, 1.0], [0.0, 1.0, 0.0]),
            Some((5, 0.0))
        );
        // Along the slope's own plane, on it: touching is meeting.
        assert_eq!(
            cast(&built, [10.0, 0.0, 1.0], [4.0, 1.0, 0.0]),
            Some((5, 0.0))
        );
        // The other three sides: the same wedge turned. Each is high on the side it rises toward.
        assert_eq!(
            cast(&built, [11.0, 10.0, 5.0], [0.0, -1.0, 0.0]),
            Some((3, 9.25))
        );
        assert_eq!(
            cast(&built, [13.0, 10.0, 5.0], [0.0, -1.0, 0.0]),
            Some((3, 9.75))
        );
        assert_eq!(
            cast(&built, [21.0, 10.0, 13.0], [0.0, -1.0, 0.0]),
            Some((6, 9.25))
        );
        assert_eq!(
            cast(&built, [21.0, 10.0, 11.0], [0.0, -1.0, 0.0]),
            Some((6, 9.75))
        );
        assert_eq!(
            cast(&built, [25.0, 10.0, 11.0], [0.0, -1.0, 0.0]),
            Some((4, 9.25))
        );
        assert_eq!(
            cast(&built, [25.0, 10.0, 13.0], [0.0, -1.0, 0.0]),
            Some((4, 9.75))
        );
    }

    /// The same six corners as the library's own shape: the wedge 4c's sweeps will use. Our ray's distance and the
    /// library's agree within a tenth of a millimetre wherever both meet the ramp, and neither meets it alone
    /// except within that margin of its edge.
    #[test]
    fn our_ramp_is_the_librarys_wedge_within_a_tenth_of_a_millimetre() {
        let solid = Solid {
            min: [2.0, 0.0, 0.0],
            max: [6.0, 1.0, 2.0],
            form: Form::Ramp {
                rises_toward: Toward::PlusX,
            },
        };
        let p = |x: f32, y: f32, z: f32| Vector::new(x, y, z);
        let ([x0, y0, z0], [x1, y1, z1]) = (solid.min, solid.max);
        let points = vec![
            p(x0, y0, z0),
            p(x1, y0, z0),
            p(x1, y1, z0),
            p(x0, y0, z1),
            p(x1, y0, z1),
            p(x1, y1, z1),
        ];
        let faces = [
            [0, 2, 1],
            [3, 4, 5],
            [0, 1, 4],
            [0, 4, 3],
            [1, 2, 5],
            [1, 5, 4],
            [0, 3, 5],
            [0, 5, 2],
        ];
        let wedge = ConvexPolyhedron::from_convex_mesh(points, &faces).expect("a wedge");
        let (mut both, mut widest) = (0_u32, 0.0_f32);
        for step in 0..2000_u32 {
            // Rays from above and from the sides, on a centimetre grid, some straight and some slanted.
            let (a, b) = (
                (step % 50) as f32 * 0.1 + 1.5,
                (step / 50) as f32 * 0.05 + 0.01,
            );
            let cases = [
                ([a, 5.0, b], [0.0, -1.0, 0.0]),
                ([a, 5.0, b], [0.3, -1.0, 0.1]),
                ([-3.0, b * 0.6, 1.0], [1.0, 0.05, 0.0]),
                ([9.0, b * 0.6, 1.0], [-1.0, 0.02, 0.01]),
            ];
            for (origin, direction) in cases {
                let ours = enter_ramp(&solid, Toward::PlusX, origin, direction, 100.0);
                let ray = Ray::new(
                    p(origin[0], origin[1], origin[2]),
                    p(direction[0], direction[1], direction[2]),
                );
                let theirs = wedge.cast_local_ray(&ray, 100.0, true);
                if let (Some(ours), Some(theirs)) = (ours, theirs) {
                    both += 1;
                    let apart = (ours - theirs).abs();
                    if apart > widest {
                        widest = apart;
                    }
                }
            }
        }
        assert!(both > 3000, "only {both} rays met the ramp both ways");
        assert!(
            widest < 1e-4,
            "our ramp and the library's wedge are {widest} m apart"
        );
    }
}
