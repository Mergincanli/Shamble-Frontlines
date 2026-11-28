//! His body against the static world, in the 2.5D form (rust-reviewer's block 11.3, block 10's measured form; his
//! sandbox's rules as game-designer's blocks 6 and 7 read them; SWEEP-ROUTE: our own sweeps, no library). The body is a
//! circle of `Rules::radius` on the ground plane, standing on a smaller foot circle (`Rules::foot_radius`), as tall as
//! `Rules::height`; a shape is its box's footprint with a height there (a box's top, or a ramp's slope). Everything is
//! `f32`, by additions, subtractions, multiplications, divisions, square roots and comparisons (ADR 0004 part 7), in a
//! fixed order, over the shapes in rising index order: the same bits on every target.
//!
//! The map's bounds take part as the design has them: the bottom is the ground (`StaticWorld::floor_y`), the four
//! sides are walls, the top is an underside for the head.

use content::map::{Form, Toward};

use crate::collision::{MAX_SHAPES, Solid, StaticWorld};
use crate::fmath::{clamp_f32, max_f32, min_f32};
use crate::rules::{HeadroomFeet, Rules};

/// A tolerance of a micrometre for the comparisons of heights and distances that the sandbox makes with one: a top
/// "above" the feet, an underside "inside" the headroom, a bottom "at" the feet (game-designer's N19, N20, G5).
pub const TINY: f32 = 1e-6;

/// How close counts as touching for the sweep: a body within 0.1 mm of a footprint's grown face, moving into it, is
/// in contact; moving along or away, it is not (block 10: push-out's own spot counts as touching).
pub const TOUCH: f32 = 1e-4;

/// The slope-head rule's two numbers, his sandbox's own (yard.ts:265 and :270-284 as game-designer reads them; no
/// field of the content carries them): how far below his feet the ramp's height near him may lie, and how level the
/// box's top must be with the ramp's.
pub const SLOPE_HEAD_BELOW: f32 = 0.05;
pub const SLOPE_HEAD_LEVEL: f32 = 0.01;

/// What the step counted, for a run's readout and the tests. Each count only ever rises, by wrapping: nothing in the
/// step reads a count (netcode-architect's Q-1), so a count's value can never change a result.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounts {
    /// Queries of the tree for the shapes near a soldier, and the shapes they found.
    pub queries: u32,
    pub candidates: u32,
    /// Sweeps of the body circle (each slide is one).
    pub sweeps: u32,
}

impl QueryCounts {
    fn add(counter: &mut u32, by: u32) {
        *counter = counter.wrapping_add(by);
    }
}

/// The axis a ramp rises along (0 for x, 2 for z) and whether it rises toward that axis's high side.
const fn rise(toward: Toward) -> (usize, bool) {
    match toward {
        Toward::PlusX => (0, true),
        Toward::MinusX => (0, false),
        Toward::PlusZ => (2, true),
        Toward::MinusZ => (2, false),
    }
}

/// A shape's ramp, if it is one: its rise as `rise` gives it.
const fn ramp_of(solid: &Solid) -> Option<(usize, bool)> {
    match solid.form {
        Form::Box => None,
        Form::Ramp { rises_toward } => Some(rise(rises_toward)),
    }
}

/// The point of a shape's footprint nearest (x, z).
fn nearest(solid: &Solid, x: f32, z: f32) -> (f32, f32) {
    (
        clamp_f32(x, solid.min[0], solid.max[0]),
        clamp_f32(z, solid.min[2], solid.max[2]),
    )
}

/// The squared distance from (x, z) to a shape's footprint: 0 inside it.
fn distance_sq(solid: &Solid, x: f32, z: f32) -> f32 {
    let (nx, nz) = nearest(solid, x, z);
    (x - nx) * (x - nx) + (z - nz) * (z - nz)
}

/// Whether a circle of radius `r` at (x, z) overlaps a shape's footprint (strictly: touching is not overlapping).
fn overlaps(solid: &Solid, x: f32, z: f32, r: f32) -> bool {
    distance_sq(solid, x, z) < r * r
}

/// A shape's height at (x, z), held into its footprint: a box's top; on a ramp, its slope's height along its rise.
fn height_at(solid: &Solid, x: f32, z: f32) -> f32 {
    match ramp_of(solid) {
        None => solid.max[1],
        Some((axis, up)) => {
            let along = if axis == 0 {
                clamp_f32(x, solid.min[0], solid.max[0])
            } else {
                clamp_f32(z, solid.min[2], solid.max[2])
            };
            let length = solid.max[axis] - solid.min[axis];
            let share = if up {
                (along - solid.min[axis]) / length
            } else {
                (solid.max[axis] - along) / length
            };
            solid.min[1] + (solid.max[1] - solid.min[1]) * share
        }
    }
}

/// A shape's height at the point of its footprint nearest (x, z).
fn height_near(solid: &Solid, x: f32, z: f32) -> f32 {
    let (nx, nz) = nearest(solid, x, z);
    height_at(solid, nx, nz)
}

/// A ramp's grade: its rise over its run (0 for a box).
fn grade(solid: &Solid) -> f32 {
    match ramp_of(solid) {
        None => 0.0,
        Some((axis, _)) => (solid.max[1] - solid.min[1]) / (solid.max[axis] - solid.min[axis]),
    }
}

/// The shapes near one soldier for one step, found once: the static world's shapes by the indices of the tree's
/// candidates for a box that holds everything the step can reach (rising order), and the bounds.
pub struct Near<'w> {
    solids: &'w [Solid],
    indices: &'w [u32],
    floor: f32,
    bounds: [[f32; 3]; 2],
}

impl<'w> Near<'w> {
    /// The shapes of `statics` whose boxes meet the column from `low` to `high` on x and z (the bounds' whole height),
    /// found through the tree into `found`.
    pub fn around(
        statics: &'w StaticWorld,
        low: [f32; 2],
        high: [f32; 2],
        found: &'w mut [u32; MAX_SHAPES],
        counts: &mut QueryCounts,
    ) -> Self {
        let bounds = statics.bounds_m();
        let (min, max) = (
            [low[0], bounds[0][1], low[1]],
            [high[0], bounds[1][1], high[1]],
        );
        let indices = statics.candidates(min, max, found);
        QueryCounts::add(&mut counts.queries, 1);
        QueryCounts::add(&mut counts.candidates, indices.len() as u32);
        Self {
            solids: statics.solids(),
            indices,
            floor: bounds[0][1],
            bounds,
        }
    }

    /// Every shape of `statics`, in index order: the tests' control for the tree's candidates.
    #[cfg(test)]
    pub fn every(statics: &'w StaticWorld, all: &'w [u32]) -> Self {
        let bounds = statics.bounds_m();
        Self {
            solids: statics.solids(),
            indices: all,
            floor: bounds[0][1],
            bounds,
        }
    }

    /// The ground's height.
    #[must_use]
    pub const fn floor(&self) -> f32 {
        self.floor
    }

    /// The shapes, in rising index order, each with its index.
    fn shapes(&self) -> impl Iterator<Item = (u32, &'w Solid)> + '_ {
        let solids = self.solids;
        self.indices
            .iter()
            .filter_map(move |&at| solids.get(at as usize).map(|solid| (at, solid)))
    }

    /// Whether a shape floats: its bottom above the ground.
    fn floats(&self, solid: &Solid) -> bool {
        solid.min[1] > self.floor
    }
}

/// Headroom over a top (game-designer's N19, the sandbox's yard.ts:185-194): no floating shape's underside lies
/// strictly between `top` and `top` plus his height over his body circle (its radius less a micrometre) at (x, z).
#[must_use]
pub fn headroom(near: &Near, rules: &Rules, top: f32, x: f32, z: f32) -> bool {
    !near.shapes().any(|(_, solid)| {
        near.floats(solid)
            && solid.min[1] > top + TINY
            && solid.min[1] < top + rules.height - TINY
            && overlaps(solid, x, z, rules.radius - TINY)
    })
}

/// The slope-head rule (G4, the sandbox's yard.ts:264-281 as game-designer reads it): a resting box, level within 1 cm
/// with the top of a ramp under his body circle whose height near him lies in feet - 0.05 up to feet + the step-up +
/// his radius times its grade + the allowance, and where the ramp's height at the box's nearest point is at least the
/// box's top less the step-up, is neither a wall nor a step target.
#[must_use]
pub fn at_slope_head(near: &Near, rules: &Rules, solid: &Solid, x: f32, z: f32, feet: f32) -> bool {
    if ramp_of(solid).is_some() || near.floats(solid) {
        return false;
    }
    near.shapes().any(|(_, ramp)| {
        if ramp_of(ramp).is_none() || !overlaps(ramp, x, z, rules.radius) {
            return false;
        }
        let height = height_near(ramp, x, z);
        let top = feet + rules.step_up + rules.radius * grade(ramp) + rules.step_allowance;
        if height < feet - SLOPE_HEAD_BELOW || height > top {
            return false;
        }
        if (solid.max[1] - ramp.max[1]).abs() > SLOPE_HEAD_LEVEL {
            return false;
        }
        let (bx, bz) = nearest(solid, x, z);
        height_at(ramp, bx, bz) >= solid.max[1] - rules.step_up
    })
}

/// Whether a shape is a wall for him with his feet at `feet` and his centre at (x, z) (game-designer's N20, G5, the
/// sandbox's yard.ts:300-357): not when it lies wholly above his head; under HEADROOM-FEET (a) only, not when its top
/// near him is at or below his feet; not when its top near him is within the step (the step-up plus the allowance)
/// with headroom over it and its bottom at his feet or lower; not at a slope's head.
#[must_use]
pub fn is_wall(near: &Near, rules: &Rules, solid: &Solid, x: f32, z: f32, feet: f32) -> bool {
    if solid.min[1] >= feet + rules.height - TINY {
        return false;
    }
    let top = height_near(solid, x, z);
    if rules.headroom_feet == HeadroomFeet::OwnTopNeverWall && top <= feet + TINY {
        return false;
    }
    let walkable = top <= feet + rules.step_up + rules.step_allowance
        && headroom(near, rules, top, x, z)
        && solid.min[1] <= feet + TINY;
    !walkable && !at_slope_head(near, rules, solid, x, z, feet)
}

/// Where a circle of radius `r` moving from (px, pz) by (dx, dz) first touches a footprint (`low` to `high` on x and
/// z), as the share t of the move and the contact's outward normal; nothing when it does not within the move. A circle
/// already touching (within `TOUCH`) that moves along or away is no hit, and one moving into the face is hit at t = 0
/// with the face's own normal (axis-aligned) or, at a corner, the radial one; a centre inside the footprint gives
/// nothing (the sweep does not push out). A start exactly on the grown face is a contact (the slab test's "a >= t0").
fn sweep_footprint(
    low: [f32; 2],
    high: [f32; 2],
    r: f32,
    (px, pz): (f32, f32),
    (dx, dz): (f32, f32),
) -> Option<(f32, f32, f32)> {
    let (qx, qz) = (clamp_f32(px, low[0], high[0]), clamp_f32(pz, low[1], high[1]));
    let (ex, ez) = (px - qx, pz - qz);
    let gap = (ex * ex + ez * ez).sqrt();
    if gap <= r + TOUCH {
        if gap > 0.0 {
            let sign = |v: f32| if v < 0.0 { -1.0 } else { 1.0 };
            let (nx, nz) = if ex == 0.0 {
                (0.0, sign(ez))
            } else if ez == 0.0 {
                (sign(ex), 0.0)
            } else {
                (ex / gap, ez / gap)
            };
            if dx * nx + dz * nz >= 0.0 {
                return None;
            }
            return Some((0.0, nx, nz));
        }
        return None;
    }
    let (grown_low, grown_high) = ([low[0] - r, low[1] - r], [high[0] + r, high[1] + r]);
    let (start, step) = ([px, pz], [dx, dz]);
    let (mut enter, mut leave, mut axis) = (0.0_f32, 1.0_f32, 2_usize);
    for k in 0..2 {
        if step[k] == 0.0 {
            if start[k] < grown_low[k] || start[k] > grown_high[k] {
                return None;
            }
            continue;
        }
        let (mut near_t, mut far_t) = (
            (grown_low[k] - start[k]) / step[k],
            (grown_high[k] - start[k]) / step[k],
        );
        if near_t > far_t {
            core::mem::swap(&mut near_t, &mut far_t);
        }
        if near_t > enter || (axis == 2 && near_t >= enter) {
            enter = near_t;
            axis = k;
        }
        if far_t < leave {
            leave = far_t;
        }
        if enter > leave {
            return None;
        }
    }
    if axis == 2 {
        return None;
    }
    let (hx, hz) = (px + dx * enter, pz + dz * enter);
    if (hx >= low[0] && hx <= high[0]) || (hz >= low[1] && hz <= high[1]) {
        let away = |v: f32| if v < 0.0 { 1.0 } else { -1.0 };
        let normal = if axis == 0 {
            (away(dx), 0.0)
        } else {
            (0.0, away(dz))
        };
        return Some((enter, normal.0, normal.1));
    }
    // The grown box's corner region: the circle meets the footprint's corner.
    let cx = if hx < low[0] { low[0] } else { high[0] };
    let cz = if hz < low[1] { low[1] } else { high[1] };
    let (fx, fz) = (px - cx, pz - cz);
    let a = dx * dx + dz * dz;
    let b = 2.0 * (fx * dx + fz * dz);
    let c = fx * fx + fz * fz - r * r;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let t = (-b - disc.sqrt()) / (2.0 * a);
    if !(0.0..=1.0).contains(&t) {
        return None;
    }
    Some((t, (fx + dx * t) / r, (fz + dz * t) / r))
}

/// What one sweep's contact is against: a shape (by index), or one of the bounds' four sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Against {
    Shape(u32),
    Side,
}

/// The first contact of his circle moving by (mx, mz) from (x, z) with the walls (`is_wall` at `feet` and at the
/// start) and the bounds' four sides: its share of the move, its normal and what it is. Ties go to the lower shape
/// index, and a side only wins a tie against no shape. A ramp met within its width is left to the set-back, unless
/// he comes from beyond its high end; its sides are swept like any wall (block 10).
fn first_contact(
    near: &Near,
    rules: &Rules,
    (x, z): (f32, f32),
    (mx, mz): (f32, f32),
    feet: f32,
) -> Option<(f32, f32, f32, Against)> {
    let mut best: Option<(f32, f32, f32, Against)> = None;
    for (at, solid) in near.shapes() {
        if !is_wall(near, rules, solid, x, z, feet) {
            continue;
        }
        if let Some((axis, up)) = ramp_of(solid) {
            let across = 2 - axis;
            let (along, side) = if axis == 0 { (x, z) } else { (z, x) };
            let within_width = side > solid.min[across] && side < solid.max[across];
            let beyond_high = if up {
                along >= solid.max[axis]
            } else {
                along <= solid.min[axis]
            };
            if within_width && !beyond_high {
                continue;
            }
        }
        let (low, high) = ([solid.min[0], solid.min[2]], [solid.max[0], solid.max[2]]);
        if let Some((t, nx, nz)) = sweep_footprint(low, high, rules.radius, (x, z), (mx, mz))
            && best.is_none_or(|(bt, ..)| t < bt)
        {
            best = Some((t, nx, nz, Against::Shape(at)));
        }
    }
    // The bounds' four sides: his centre stays at least his radius inside each.
    let [low, high] = near.bounds;
    let sides = [
        (x, mx, low[0] + rules.radius, -1.0_f32, 0.0_f32),
        (x, mx, high[0] - rules.radius, 1.0, 0.0),
        (z, mz, low[2] + rules.radius, 0.0, -1.0),
        (z, mz, high[2] - rules.radius, 0.0, 1.0),
    ];
    for (at, by, limit, ox, oz) in sides {
        // Outward along the side's normal: (ox, oz) points out of the bounds; moving out past the limit is a contact.
        let outward = by * (ox + oz);
        if outward <= 0.0 {
            continue;
        }
        let room = (limit - at) * (ox + oz);
        let t = if room <= 0.0 { 0.0 } else { room / outward };
        if t <= 1.0 && best.is_none_or(|(bt, ..)| t < bt) {
            best = Some((t, -ox, -oz, Against::Side));
        }
    }
    best
}

/// The one sweep (11.3): his circle moved by (mx, mz) from his centre against the walls at `feet` and the bounds'
/// sides, stopped at the first contact with no skin, a face contact put exactly on the grown face, then one slide of
/// what is left along it. With `slide_speed`, the part of his speed (vx, vz) into each contact is removed. Returns the
/// new centre.
pub fn sweep_circle(
    near: &Near,
    rules: &Rules,
    counts: &mut QueryCounts,
    centre: (f32, f32),
    move_by: (f32, f32),
    feet: f32,
    speed: Option<&mut (f32, f32)>,
) -> (f32, f32) {
    let (mut x, mut z) = centre;
    let (mut mx, mut mz) = move_by;
    let mut speed = speed;
    for _ in 0..2 {
        if mx == 0.0 && mz == 0.0 {
            break;
        }
        QueryCounts::add(&mut counts.sweeps, 1);
        match first_contact(near, rules, (x, z), (mx, mz), feet) {
            None => {
                x += mx;
                z += mz;
                break;
            }
            Some((t, nx, nz, against)) => {
                let face = |solid: Option<&Solid>| {
                    // On a face of a shape (an axis-aligned normal), the contact axis sits exactly on its grown face.
                    solid.map(|solid| {
                        if nz == 0.0 && (nx == 1.0 || nx == -1.0) {
                            (
                                Some(if nx > 0.0 {
                                    solid.max[0] + rules.radius
                                } else {
                                    solid.min[0] - rules.radius
                                }),
                                None,
                            )
                        } else if nx == 0.0 && (nz == 1.0 || nz == -1.0) {
                            (
                                None,
                                Some(if nz > 0.0 {
                                    solid.max[2] + rules.radius
                                } else {
                                    solid.min[2] - rules.radius
                                }),
                            )
                        } else {
                            (None, None)
                        }
                    })
                };
                let solid = match against {
                    Against::Shape(at) => near.solids.get(at as usize),
                    Against::Side => None,
                };
                let (on_x, on_z) = face(solid).unwrap_or((None, None));
                x = on_x.unwrap_or(x + mx * t);
                z = on_z.unwrap_or(z + mz * t);
                if against == Against::Side {
                    // A side holds his centre exactly at its limit.
                    let [low, high] = near.bounds;
                    if nx > 0.0 {
                        x = low[0] + rules.radius;
                    } else if nx < 0.0 {
                        x = high[0] - rules.radius;
                    } else if nz > 0.0 {
                        z = low[2] + rules.radius;
                    } else if nz < 0.0 {
                        z = high[2] - rules.radius;
                    }
                }
                let (rx, rz) = (mx * (1.0 - t), mz * (1.0 - t));
                let into = rx * nx + rz * nz;
                (mx, mz) = (rx - into * nx, rz - into * nz);
                if let Some(speed) = speed.as_deref_mut() {
                    let into = speed.0 * nx + speed.1 * nz;
                    if into < 0.0 {
                        *speed = (speed.0 - into * nx, speed.1 - into * nz);
                    }
                }
            }
        }
    }
    (x, z)
}

/// The set-back (game-designer's N20 and G6, the sandbox's movement.ts:147-157 and yard.ts:325-332): each ramp that
/// is a wall for him at `feet`, with his centre within its width and over its slope, sets him back along the slope
/// to where its height is `feet` plus the step-up (from its own low height; no allowance), only if he is further up;
/// his speed up the slope is removed. Returns the new centre.
#[must_use]
pub fn set_back(
    near: &Near,
    rules: &Rules,
    centre: (f32, f32),
    feet: f32,
    speed: &mut (f32, f32),
) -> (f32, f32) {
    let (mut x, mut z) = centre;
    for (_, solid) in near.shapes() {
        let Some((axis, up)) = ramp_of(solid) else {
            continue;
        };
        if !overlaps(solid, x, z, rules.radius) || !is_wall(near, rules, solid, x, z, feet) {
            continue;
        }
        let across = 2 - axis;
        let (along, side) = if axis == 0 { (x, z) } else { (z, x) };
        let within_width = side > solid.min[across] && side < solid.max[across];
        let over_slope = along >= solid.min[axis] && along <= solid.max[axis];
        if !(within_width && over_slope) {
            continue;
        }
        let share = clamp_f32(
            (feet + rules.step_up - solid.min[1]) / (solid.max[1] - solid.min[1]),
            0.0,
            1.0,
        );
        let length = solid.max[axis] - solid.min[axis];
        let to = if up {
            solid.min[axis] + share * length
        } else {
            solid.max[axis] - share * length
        };
        let further = if up { along > to } else { along < to };
        if further {
            if axis == 0 {
                x = to;
            } else {
                z = to;
            }
            let up_slope = if axis == 0 { &mut speed.0 } else { &mut speed.1 };
            if (up && *up_slope > 0.0) || (!up && *up_slope < 0.0) {
                *up_slope = 0.0;
            }
        }
    }
    (x, z)
}

/// The step-up target (game-designer's N17 and G2, the sandbox's yard.ts:230-256): of the shapes his body circle
/// overlaps whose top near him is above his feet (`feet` after the arc), within the step from the test feet, with
/// headroom over it, its bottom at his feet or lower, not at a slope's head, and toward which he moves (or over whose
/// footprint his centre already is), the highest: its index and its top.
#[must_use]
pub fn step_up_target(
    near: &Near,
    rules: &Rules,
    (x, z): (f32, f32),
    (vx, vz): (f32, f32),
    feet: f32,
    test_feet: f32,
) -> Option<(u32, f32)> {
    let mut best: Option<(u32, f32)> = None;
    for (at, solid) in near.shapes() {
        if !overlaps(solid, x, z, rules.radius) {
            continue;
        }
        let top = height_near(solid, x, z);
        let (nx, nz) = nearest(solid, x, z);
        let over = x >= solid.min[0] && x <= solid.max[0] && z >= solid.min[2] && z <= solid.max[2];
        let toward = (nx - x) * vx + (nz - z) * vz > 0.0 || over;
        let target = top > feet + TINY
            && top <= test_feet + rules.step_up + rules.step_allowance
            && headroom(near, rules, top, x, z)
            && solid.min[1] <= feet + TINY
            && !at_slope_head(near, rules, solid, x, z, test_feet)
            && toward;
        if target && best.is_none_or(|(_, best_top)| top > best_top) {
            best = Some((at, top));
        }
    }
    best
}

/// The pull a step-up fixes (G1): from his centre toward the stepped shape's nearest point, of length the distance
/// less `pull_to_foot` times his foot's radius; none when that is not positive.
#[must_use]
pub fn pull_toward(near: &Near, rules: &Rules, shape: u32, (x, z): (f32, f32)) -> [f32; 2] {
    let Some(solid) = near.solids.get(shape as usize) else {
        return [0.0, 0.0];
    };
    let (nx, nz) = nearest(solid, x, z);
    let (dx, dz) = (nx - x, nz - z);
    let distance = (dx * dx + dz * dz).sqrt();
    let need = distance - rules.pull_to_foot * rules.foot_radius;
    if need > 0.0 && distance > 0.0 {
        [dx / distance * need, dz / distance * need]
    } else {
        [0.0, 0.0]
    }
}

/// The highest top he can stand on (the sandbox's movement.ts:194-209): over the circle of radius `r` at (x, z), the
/// highest top within `limit`, the ground included.
#[must_use]
pub fn standing_height(near: &Near, (x, z): (f32, f32), r: f32, limit: f32) -> f32 {
    let mut surface = near.floor;
    for (_, solid) in near.shapes() {
        if !overlaps(solid, x, z, r) {
            continue;
        }
        let top = height_near(solid, x, z);
        if top <= limit && top > surface {
            surface = top;
        }
    }
    surface
}

/// The steepest ramp under his foot circle at any height (G8, the sandbox's yard.ts:286-294): its grade, 0 for none.
#[must_use]
pub fn steepest_under_foot(near: &Near, rules: &Rules, (x, z): (f32, f32)) -> f32 {
    let mut steepest = 0.0_f32;
    for (_, solid) in near.shapes() {
        if ramp_of(solid).is_some() && overlaps(solid, x, z, rules.foot_radius) {
            steepest = max_f32(steepest, grade(solid));
        }
    }
    steepest
}

/// The head bump (game-designer's N18, the sandbox's movement.ts:132-138): at the tick's start position, the lowest
/// underside over his body circle (its radius less a micrometre) that is no lower than his head at the tick's start
/// less 1 mm: a floating shape's bottom, or the bounds' top. The height his feet may reach: that underside less his
/// height; none when there is no such underside.
#[must_use]
pub fn ceiling_for_feet(near: &Near, rules: &Rules, (x, z): (f32, f32), feet_at_start: f32) -> Option<f32> {
    let head = feet_at_start + rules.height - 0.001;
    let mut lowest: Option<f32> = None;
    for (_, solid) in near.shapes() {
        let underside = solid.min[1];
        if near.floats(solid)
            && underside >= head
            && overlaps(solid, x, z, rules.radius - TINY)
            && lowest.is_none_or(|low| underside < low)
        {
            lowest = Some(underside);
        }
    }
    let top = near.bounds[1][1];
    if top >= head {
        lowest = Some(lowest.map_or(top, |low| min_f32(low, top)));
    }
    lowest.map(|underside| underside - rules.height)
}

/// The landing crossing (G9, in the vertical phase, before the walls): of the tops under his foot circle that his feet
/// crossed falling inside this tick (at or below where they were, `from` plus 0.1 mm, and above where they are, `to`),
/// the highest; the ground counts. None when he crossed none.
#[must_use]
pub fn landing_crossed(near: &Near, rules: &Rules, (x, z): (f32, f32), from: f32, to: f32) -> Option<f32> {
    let mut hit: Option<f32> = None;
    let floor = near.floor;
    if floor <= from + 1e-4 && floor > to {
        hit = Some(floor);
    }
    for (_, solid) in near.shapes() {
        let top = height_near(solid, x, z);
        if overlaps(solid, x, z, rules.foot_radius)
            && top <= from + 1e-4
            && top > to
            && hit.is_none_or(|best| top > best)
        {
            hit = Some(top);
        }
    }
    hit
}
