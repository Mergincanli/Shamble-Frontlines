// rust-reviewer's scratch prototype, round 8 (4c, block 9 of rust-reviewer-read.md, 2026-11-28): body (3a) rebuilt to
// his sandbox's rules as game-designer-read.md block 6 reads them (N16 to N24, G1 to G10; game-designer read the
// sandbox, this file did not), on block 6's case code (rr-body-proto-6.rs). f32 throughout. Not product code.
// Build: rustc -O --edition 2024; run: <bin> 3a b5|b6|b9 (env SETBACK_GUARD=1 for G6's guard).

const DT: f32 = 1.0 / 30.0;
const G: f32 = 24.0;
const R: f32 = 0.4; // body radius (his Export)
const FOOT: f32 = 0.15; // movement.ts:59-60
const STEP: f32 = 0.55;
const REACH: f32 = 3.071;
const BODY_H: f32 = 1.8;
const PULL_SHARE: f32 = 0.5; // movement.ts:61-63
const PULL_MIN: f32 = 0.03;
const PULL_TO: f32 = 0.9; // of the foot radius, movement.ts:172
const SNAP: f32 = 0.2; // movement.ts:65
const EASE_K: f32 = 0.373_75; // 1 - exp(-ln10 x (1/30) / 0.164), rust-reviewer-read F9 corrected
// The push, game-designer-read.md block 3 (push.ts at be608cad, by its reading).
const PUSH_RADIUS: f32 = 0.4;
const PUSH_STRENGTH: f32 = 8.0;
const HEIGHT_GAP: f32 = 1.0;
const SIDEWAYS: f32 = 0.7;
const KEEP_GOING: f32 = 0.5;
const MAX_STEP: f32 = 0.25;
const HEAD_ON_GATE: f32 = 0.2;
const KEEP_GATE: f32 = 0.1;
const SKIN: f32 = 0.001;
static TOWARD_START: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static LAND_PREV_ALL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static WALL_DISC: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Body { A3, B3, Cyl, X3 }

#[derive(Clone, Copy)]
struct Block { min: [f32; 3], max: [f32; 3], ramp: Option<(usize, bool)> } // (axis 0 or 2, rises toward +)

fn bx(min: [f32; 3], max: [f32; 3]) -> Block { Block { min, max, ramp: None } }
fn rp(min: [f32; 3], max: [f32; 3], axis: usize, up: bool) -> Block { Block { min, max, ramp: Some((axis, up)) } }

fn scene() -> Vec<Block> {
    vec![
        bx([-12.0, 0.0, -8.0], [-5.9, 2.6, -3.6]),
        rp([-12.0, 2.6, -8.0], [-5.9, 4.7, -5.8], 2, true),
        rp([-12.0, 2.6, -5.8], [-5.9, 4.7, -3.6], 2, false),
        bx([-4.4, 0.0, -7.4], [0.2, 2.6, -3.6]),
        rp([-4.4, 2.6, -7.4], [0.2, 4.3, -5.5], 2, true),
        rp([-4.4, 2.6, -5.5], [0.2, 4.3, -3.6], 2, false),
        bx([-6.0, 0.0, 3.0], [-2.9, 1.0, 3.45]),
        bx([30.0, 0.0, -10.0], [32.0, 0.55, -8.0]),
        bx([34.0, 0.0, -10.0], [36.0, 0.56, -8.0]),
        bx([30.0, 0.0, -4.0], [33.0, 2.9, -1.0]),
        bx([35.0, 0.0, -4.0], [38.0, 3.05, -1.0]),
        bx([40.0, 0.0, -4.0], [43.0, 3.1, -1.0]),
        bx([30.0, 0.0, 4.0], [30.2, 2.5, 10.0]),
        rp([41.0, 0.0, 4.0], [45.0, 1.0, 8.0], 0, true),
        bx([45.0, 0.0, 4.0], [49.0, 1.0, 8.0]),
        rp([42.0, 0.0, 10.0], [45.0, 1.5, 14.0], 0, true),
        bx([45.0, 0.0, 10.0], [49.0, 1.5, 14.0]),
    ]
}

/// A made map for the ceilings and the high drop (the test scene has no overhang): flat ground and five shapes.
fn ceiling_map() -> Vec<Block> {
    vec![
        bx([55.0, 2.0, -1.0], [57.0, 2.3, 1.0]),  // 0: a beam, bottom 2.0 (above the 1.8 m head on the ground)
        bx([60.0, 1.7, -1.0], [62.0, 2.0, 1.0]),  // 1: a low beam, bottom 1.7 (below the head)
        bx([65.0, 0.0, -1.0], [67.0, 0.55, 1.0]), // 2: a 0.55 m step ...
        bx([66.0, 2.2, -1.0], [68.0, 2.5, 1.0]),  // 3: ... with a beam over its far part, bottom 2.2 (< 0.55 + 1.8)
        bx([55.0, 0.0, 5.0], [57.0, 2.0, 7.0]),   // 4: a 2.0 m block for the high drop
    ]
}

fn clampf(v: f32, lo: f32, hi: f32) -> f32 { if v < lo { lo } else if v > hi { hi } else { v } }
fn nearest(b: &Block, x: f32, z: f32) -> (f32, f32) { (clampf(x, b.min[0], b.max[0]), clampf(z, b.min[2], b.max[2])) }
fn dist2(b: &Block, x: f32, z: f32) -> f32 { let (nx, nz) = nearest(b, x, z); (x - nx) * (x - nx) + (z - nz) * (z - nz) }
fn overlaps(b: &Block, x: f32, z: f32, r: f32) -> bool { dist2(b, x, z) < r * r }
fn height_at(b: &Block, x: f32, z: f32) -> f32 {
    match b.ramp {
        None => b.max[1],
        Some((a, up)) => {
            let p = if a == 0 { clampf(x, b.min[0], b.max[0]) } else { clampf(z, b.min[2], b.max[2]) };
            let len = b.max[a] - b.min[a];
            let frac = if up { (p - b.min[a]) / len } else { (b.max[a] - p) / len };
            b.min[1] + (b.max[1] - b.min[1]) * frac
        }
    }
}
fn height_near(b: &Block, x: f32, z: f32) -> f32 { let (nx, nz) = nearest(b, x, z); height_at(b, nx, nz) }
fn height_max(b: &Block, x: f32, z: f32, r: f32) -> f32 {
    match b.ramp {
        None => b.max[1],
        Some((a, up)) => {
            let s = if up { r } else { -r };
            let (px, pz) = if a == 0 { (x + s, z) } else { (x, z + s) };
            height_at(b, px, pz)
        }
    }
}
fn slope_under(world: &[Block], x: f32, z: f32) -> f32 {
    for b in world {
        if let Some((a, _)) = b.ramp {
            if x >= b.min[0] && x <= b.max[0] && z >= b.min[2] && z <= b.max[2] { return (b.max[1] - b.min[1]) / (b.max[a] - b.min[a]); }
        }
    }
    0.0
}

#[derive(Clone, Copy, Default)]
struct Cfg { ease: bool, head_stop: bool, land_prev: bool }

/// Block 5's floating test (bottom above the head), and with the head stop a tolerant one so a head resting on a
/// bottom is not a wall.
fn floating(b: &Block, feet: f32, cfg: Cfg) -> bool {
    if cfg.head_stop { b.min[1] >= feet + BODY_H - 1e-4 } else { b.min[1] > feet + BODY_H }
}

#[derive(Clone, Copy)]
struct S {
    x: f32, z: f32, y: f32, vy: f32, grounded: bool,
    pull: Option<usize>, stepped: Option<usize>,
    steps: u32, falls_unjumped: u32, airborne_ticks: u32, max_float: f32, open_float: f32, fall_at: Option<(f32, f32)>,
    // block 6
    cfg: Cfg, vx: f32, vz: f32, lvx: f32, lvz: f32, fall_from: Option<f32>, flickers: u32, head_hits: u32,
    max_drop: f32, wall_hits: u32, y_moved: f32, below_top: f32,
    // block 9
    pullv: Option<(f32, f32)>, pull_carry: f32, setbacks: u32, max_setback: f32, max_rise: f32,
}

fn eps(body: Body) -> f32 { if body == Body::Cyl { 0.0 } else { 0.005 } }

/// Block 9: the sandbox's rules as game-designer-read.md block 6 reads them (movement.ts and yard.ts at be608cad, by
/// its file:line; nothing of the sandbox's code seen here). Body (3a) only.
static SETBACK_GUARD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
const TINY: f32 = 1e-6;
// Block 10's switches: on = as game-designer's block 7 reads the code (OLD_HEAD=1, OLD_SETBACK=1, OLD_LANDING=1 give
// block 9's forms back); WALK_SWEEP=1 sweeps the walk's wall pass; PULL_CLIP=1 sweeps the pull.
static TRACE_BITS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static OLD_HEAD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static OLD_SETBACK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static OLD_LANDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static WALK_SWEEP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static PULL_CLIP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static OWN_TOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static HEAD_WIDTH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Headroom (N19, yard.ts:185-194): no floating block's underside strictly inside top .. top + 1.8 m over the body
/// circle (0.4 m less 1e-6) at (x, z).
fn headroom(world: &[Block], top: f32, x: f32, z: f32) -> bool {
    !world.iter().any(|b| b.min[1] > 0.0 && b.min[1] > top + TINY && b.min[1] < top + BODY_H - TINY && overlaps(b, x, z, R - TINY))
}

fn grade(r: &Block) -> f32 { match r.ramp { Some((a, _)) => (r.max[1] - r.min[1]) / (r.max[a] - r.min[a]), None => 0.0 } }

/// The slope-head rule (G4, yard.ts:270-284, as the reading gives it; the formula is my reading of its words): a box
/// (not a ramp, not floating) level within 1 cm with the top of a slope he stands on (the slope's surface near him
/// within 5 cm below his feet up to the slope's rise across his body above them), whose footprint meets that slope's
/// high end, is neither a step-up target nor a wall.
fn at_slope_head(world: &[Block], b: &Block, x: f32, z: f32, feet: f32) -> bool {
    if b.ramp.is_some() || b.min[1] > TINY { return false; }
    world.iter().any(|r| {
        let Some((a, up)) = r.ramp else { return false };
        if !overlaps(r, x, z, R) { return false; }
        let h = height_near(r, x, z);
        // "the slope's rise across the body": the body's radius (default) or its whole width (env HEAD_WIDTH=1).
        let across = if HEAD_WIDTH.load(std::sync::atomic::Ordering::Relaxed) { 2.0 * R } else { R };
        let top = if OLD_HEAD.load(std::sync::atomic::Ordering::Relaxed) { feet + across * grade(r) + 0.005 } else { feet + STEP + across * grade(r) + 0.005 };
        if h < feet - 0.05 || h > top { return false; }
        if (b.max[1] - r.max[1]).abs() > 0.01 { return false; }
        if !OLD_HEAD.load(std::sync::atomic::Ordering::Relaxed) {
            // yard.ts:281 as read: the slope's height at the box's point nearest him is at least the box's top - 0.55.
            let (bx_, bz_) = nearest(b, x, z);
            return height_at(r, bx_, bz_) >= b.max[1] - STEP;
        }
        let high = if up { r.max[a] } else { r.min[a] };
        let face = if up { b.min[a] } else { b.max[a] };
        let o = 2 - a; // the across axis
        (face - high).abs() <= TINY && b.min[o] < r.max[o] && b.max[o] > r.min[o]
    })
}

/// resolveWalls as the reading gives it (yard.ts:300-357, N20, G5, G6): returns the number of push-outs and set-backs.
fn walls(world: &[Block], s: &mut S, test_feet: f32, _e: f32) -> u32 {
    let mut n = 0;
    for b in world {
        if b.min[1] >= test_feet + BODY_H - TINY { continue; } // wholly above the head (:306)
        if !overlaps(b, s.x, s.z, R) { continue; }
        let h = height_near(b, s.x, s.z);
        // Assumed (the reading does not say): a top at or below the feet is never a wall, headroom or not
        // (env HEADROOM_ON_OWN_TOP=1 drops this, so the top he stands on can turn into a wall).
        if h <= test_feet + TINY && !OWN_TOP.load(std::sync::atomic::Ordering::Relaxed) { continue; }
        let no_wall = h <= test_feet + STEP + 0.005 && headroom(world, h, s.x, s.z) && b.min[1] <= test_feet + TINY;
        if no_wall || at_slope_head(world, b, s.x, s.z, test_feet) { continue; }
        n += 1;
        if let Some((a, up)) = b.ramp {
            let o = 2 - a;
            let (p, q) = if a == 0 { (s.x, s.z) } else { (s.z, s.x) };
            let within_width = q > b.min[o] && q < b.max[o];
            let over_slope = p >= b.min[a] && p <= b.max[a];
            let guard = SETBACK_GUARD.load(std::sync::atomic::Ordering::Relaxed);
            if within_width && (!guard || over_slope) {
                // Set back along the slope to where its height is feet + 0.55 (no 5 mm), only if further up (:325-332).
                let t = clampf((test_feet + STEP - b.min[1]) / (b.max[1] - b.min[1]), 0.0, 1.0);
                let len = b.max[a] - b.min[a];
                let c = if up { b.min[a] + t * len } else { b.max[a] - t * len };
                let further = if up { p > c } else { p < c };
                if further {
                    if a == 0 { s.x = c; } else { s.z = c; }
                    s.setbacks += 1;
                    if !OLD_SETBACK.load(std::sync::atomic::Ordering::Relaxed) {
                        // The set-back is a push like any other: the slide removes his speed up the slope.
                        let v = if a == 0 { &mut s.vx } else { &mut s.vz };
                        if (up && *v > 0.0) || (!up && *v < 0.0) { *v = 0.0; }
                    }
                    let moved = (p - c).abs();
                    if moved > s.max_setback { s.max_setback = moved; }
                }
                continue;
            }
        }
        let (nx, nz) = nearest(b, s.x, s.z);
        let (dx, dz) = (s.x - nx, s.z - nz);
        let d = (dx * dx + dz * dz).sqrt();
        let (ox, oz);
        if d > 0.0 { s.x = nx + dx / d * R; s.z = nz + dz / d * R; ox = dx / d; oz = dz / d; } else {
            let pens = [(s.x - b.min[0] + R, -1.0, 0.0), (b.max[0] - s.x + R, 1.0, 0.0), (s.z - b.min[2] + R, 0.0, -1.0), (b.max[2] - s.z + R, 0.0, 1.0)];
            let mut m = pens[0];
            for p in pens { if p.0 < m.0 { m = p; } }
            s.x += m.1 * m.0; s.z += m.2 * m.0;
            ox = m.1; oz = m.2;
        }
        // The speed that went into the wall is removed (movement.ts:147-157).
        let vn = s.vx * ox + s.vz * oz;
        if vn < 0.0 { s.vx -= vn * ox; s.vz -= vn * oz; }
    }
    n
}

/// The steepest ramp overlapping the 0.15 m foot circle, at any height (G8, yard.ts:286-294).
fn slope_under_foot(world: &[Block], x: f32, z: f32) -> f32 {
    world.iter().filter(|b| b.ramp.is_some() && overlaps(b, x, z, FOOT)).map(grade).fold(0.0, f32::max)
}

/// One tick in the sandbox's order (movement.ts:104-209 as read): jump; the arc with the test feet (G3); the head
/// bump (N18); the move; the walls; the step-up (G2, G5, G4); the pull (G1); standing; the snap or the fall.
fn tick(world: &[Block], s: &mut S, _body: Body, wish: (f32, f32), speed: f32, jump: bool, v0: f32) {
    let (x0, z0, y_start) = (s.x, s.z, s.y);
    if jump && s.grounded { s.vy = v0; s.grounded = false; }
    let mut test_feet = s.y;
    if !s.grounded {
        let (y0, v) = (s.y, s.vy);
        s.y = s.y + v * DT - 0.5 * G * DT * DT;
        s.vy = v - G * DT;
        // G3: the feet at the tick's end, except the tick that holds the arc's top (movement.ts:131).
        test_feet = if v > 0.0 && s.vy <= 0.0 { y0 + v * v / (2.0 * G) } else { s.y };
        // The head bump (N18, movement.ts:132-138, yard.ts:196-208): at the tick's start position, the lowest
        // underside of a floating block no lower than the head at the tick's start less 1 mm, over the body circle.
        let head0 = y0 + BODY_H - 0.001;
        let mut lowest = f32::INFINITY;
        for b in world {
            if b.min[1] > 0.0 && b.min[1] >= head0 && overlaps(b, x0, z0, R - TINY) && b.min[1] < lowest { lowest = b.min[1]; }
        }
        if lowest.is_finite() {
            let ceil = lowest - BODY_H;
            if s.y > ceil || test_feet > ceil {
                if s.y > ceil { s.y = ceil; }
                if s.vy > 0.0 { s.vy = 0.0; }
                if test_feet > ceil { test_feet = ceil; }
                s.head_hits += 1;
            }
        }
        if y0 - s.y > s.max_drop { s.max_drop = y0 - s.y; }
        s.y_moved = s.y;
        // Candidate fix (G9; the sandbox has the hole): in the vertical phase, before the walls, a top under the foot
        // crossed inside this falling tick lands him on it.
        if s.cfg.land_prev && s.vy <= 0.0 {
            let mut hit = f32::NEG_INFINITY;
            for b in world {
                let h = height_near(b, s.x, s.z);
                if overlaps(b, s.x, s.z, FOOT) && h <= y0 + 1e-4 && h > s.y && h > hit { hit = h; }
            }
            if hit.is_finite() { s.y = hit; s.vy = 0.0; s.grounded = true; test_feet = hit; }
        }
    }
    // The move (eased, or wish x speed), then the walls with the slide.
    if s.cfg.ease {
        s.vx += (wish.0 * speed - s.vx) * EASE_K;
        s.vz += (wish.1 * speed - s.vz) * EASE_K;
    } else {
        s.vx = wish.0 * speed;
        s.vz = wish.1 * speed;
    }
    if WALK_SWEEP.load(std::sync::atomic::Ordering::Relaxed) {
        let (mx, mz) = (s.vx * DT, s.vz * DT);
        sweep_walk(world, s, mx, mz, test_feet, true);
    } else {
        s.x += s.vx * DT;
        s.z += s.vz * DT;
    }
    s.wall_hits += walls(world, s, test_feet, 0.005);
    // The step-up (yard.ts:230-256, N17): the body circle overlaps, the top above the feet after the arc and within
    // test feet + 0.555, headroom, not from below, not at a slope's head, moving toward it or already over it.
    let mut best: Option<(usize, f32)> = None;
    for (i, b) in world.iter().enumerate() {
        if !overlaps(b, s.x, s.z, R) { continue; }
        let h = height_near(b, s.x, s.z);
        let (nx, nz) = nearest(b, s.x, s.z);
        let over = s.x >= b.min[0] && s.x <= b.max[0] && s.z >= b.min[2] && s.z <= b.max[2];
        let toward = (nx - s.x) * s.vx + (nz - s.z) * s.vz > 0.0 || over;
        if h > s.y + TINY && h <= test_feet + STEP + 0.005 && headroom(world, h, s.x, s.z) && b.min[1] <= s.y + TINY
            && !at_slope_head(world, b, s.x, s.z, test_feet) && toward && best.is_none_or(|(_, bh)| h > bh) { best = Some((i, h)); }
    }
    if let Some((i, h)) = best {
        s.y = h; s.vy = 0.0; s.grounded = true; s.steps += 1;
        // G1: the pull is fixed now, from his centre toward the block's nearest point, of length distance - 0.135.
        let (nx, nz) = nearest(&world[i], s.x, s.z);
        let (dx, dz) = (nx - s.x, nz - s.z);
        let d = (dx * dx + dz * dz).sqrt();
        let need = d - PULL_TO * FOOT;
        s.pullv = if need > 0.0 && d > 0.0 { Some((dx / d * need, dz / d * need)) } else { None };
    }
    // The pull, on top of the walk, after the walls, with no wall test: dropped when his velocity points against it;
    // half of what is left a tick, all of it once under 0.03 m (movement.ts:179-192).
    if let Some((px, pz)) = s.pullv {
        if px * s.vx + pz * s.vz < 0.0 { s.pullv = None; } else {
            let len = (px * px + pz * pz).sqrt();
            let (sx, sz) = if len < PULL_MIN { (px, pz) } else { (px * PULL_SHARE, pz * PULL_SHARE) };
            if std::env::var("TRACE9").is_ok() { println!("      pull from x {:.4} by {sx:.4} test_feet {test_feet:.3}", s.x); }
            if PULL_CLIP.load(std::sync::atomic::Ordering::Relaxed) { sweep_walk(world, s, sx, sz, test_feet, false); } else { s.x += sx; s.z += sz; }
            s.pull_carry += (sx * sx + sz * sz).sqrt();
            s.pullv = if len < PULL_MIN { None } else { Some((px - sx, pz - sz)) };
        }
    }
    // Standing: on the body circle while the pull runs, else the foot; the highest top within the feet + 0.555.
    let rad = if s.pullv.is_some() { R } else { FOOT };
    let mut surf = 0.0_f32;
    for b in world {
        if !overlaps(b, s.x, s.z, rad) { continue; }
        let h = height_near(b, s.x, s.z);
        let lim = if s.cfg.land_prev && !s.grounded { (s.y + STEP + 0.005).max(y_start + 1e-4) } else { s.y + STEP + 0.005 };
        if h <= lim && h > surf { surf = h; }
    }
    if !s.grounded {
        if (s.vy <= 0.0 || !OLD_LANDING.load(std::sync::atomic::Ordering::Relaxed)) && s.y <= surf { if surf > 0.0 && surf - s.y_moved > s.below_top { s.below_top = surf - s.y_moved; } s.y = surf; s.vy = 0.0; s.grounded = true; }
        s.airborne_ticks += u32::from(!s.grounded);
    } else {
        // The snap: 0.2 m plus (the tick's whole displacement + 0.15) x the steepest ramp under the foot (G8).
        let disp = ((s.x - x0) * (s.x - x0) + (s.z - z0) * (s.z - z0)).sqrt();
        let allow = SNAP + (disp + FOOT) * slope_under_foot(world, s.x, s.z);
        if surf >= s.y || s.y - surf <= allow { s.y = surf; } else {
            s.grounded = false; s.vy = 0.0; s.falls_unjumped += 1;
            if s.fall_at.is_none() { s.fall_at = Some((s.x, s.z)); }
            s.fall_from = Some(s.y);
        }
    }
    if s.grounded {
        if let Some(f) = s.fall_from.take() { if (s.y - f).abs() < 1e-3 { s.flickers += 1; } }
        if s.y - y_start > s.max_rise { s.max_rise = s.y - y_start; }
    }
    for b in world {
        if b.ramp.is_some() && s.x >= b.min[0] && s.x <= b.max[0] && s.z >= b.min[2] && s.z <= b.max[2] && s.grounded {
            let f = s.y - height_at(b, s.x, s.z);
            if f > s.max_float { s.max_float = f; }
            if let Some((a, _)) = b.ramp { let p = if a == 0 { s.x } else { s.z }; if p > b.min[a] + R && p < b.max[a] - R && f > s.open_float { s.open_float = f; } }
        }
    }
    // G10: the push reads each mover's velocity (after the slide).
    s.lvx = s.vx;
    s.lvz = s.vz;
    if TRACE_BITS.load(std::sync::atomic::Ordering::Relaxed) { eprintln!("T {speed} {:08x} {:08x} {:08x}", s.x.to_bits(), s.z.to_bits(), s.y.to_bits()); }
}

/// The walk's wall pass as a sweep (block 10): the move swept against the shapes that are walls for him at the
/// tick's start (block 9's wall test), stopped at contact with no skin and the contact axis put exactly on the
/// grown face (so that a face contact gives push-out's own bits), one slide; ramps whose width holds his centre at
/// either end are left to the push-out and set-back after it. `slide_speed` removes the speed into the wall.
fn sweep_walk(world: &[Block], s: &mut S, mut mx: f32, mut mz: f32, test_feet: f32, slide_speed: bool) {
    for _ in 0..2 {
        if mx == 0.0 && mz == 0.0 { break; }
        let mut best: Option<(f32, f32, f32, usize)> = None;
        for (i, b) in world.iter().enumerate() {
            if b.min[1] >= test_feet + BODY_H - TINY { continue; }
            let h = height_near(b, s.x, s.z);
            if h <= test_feet + TINY && !OWN_TOP.load(std::sync::atomic::Ordering::Relaxed) { continue; }
            let no_wall = h <= test_feet + STEP + 0.005 && headroom(world, h, s.x, s.z) && b.min[1] <= test_feet + TINY;
            if no_wall || at_slope_head(world, b, s.x, s.z, test_feet) { continue; }
            if let Some((a, _)) = b.ramp {
                let o = 2 - a;
                let (q0, q1) = if a == 0 { (s.z, s.z + mz) } else { (s.x, s.x + mx) };
                if (q0 > b.min[o] && q0 < b.max[o]) || (q1 > b.min[o] && q1 < b.max[o]) { continue; }
            }
            if std::env::var("TRACE9").is_ok() && !slide_speed { println!("        sweep cand {i} from {:.4} by {mx:.4}: {:?}", s.x, sweep_rect0(b, s.x, s.z, mx, mz)); }
            if let Some((t, nx, nz)) = sweep_rect0(b, s.x, s.z, mx, mz) { if best.is_none_or(|bb| t < bb.0) { best = Some((t, nx, nz, i)); } }
        }
        match best {
            None => { s.x += mx; s.z += mz; break; }
            Some((t, nx, nz, i)) => {
                let b = world[i];
                if nz == 0.0 && (nx == 1.0 || nx == -1.0) { s.x = if nx > 0.0 { b.max[0] + R } else { b.min[0] - R }; s.z += mz * t; }
                else if nx == 0.0 && (nz == 1.0 || nz == -1.0) { s.z = if nz > 0.0 { b.max[2] + R } else { b.min[2] - R }; s.x += mx * t; }
                else { s.x += mx * t; s.z += mz * t; }
                let (rx, rz) = (mx * (1.0 - t), mz * (1.0 - t));
                let vn = rx * nx + rz * nz;
                mx = rx - vn * nx; mz = rz - vn * nz;
                if slide_speed { let v = s.vx * nx + s.vz * nz; if v < 0.0 { s.vx -= v * nx; s.vz -= v * nz; } }
            }
        }
    }
}

/// Block 6's swept circle against a footprint with no skin: touching (within 0.1 mm, so push-out's own spot counts) and moving along or away is no hit.
fn sweep_rect0(b: &Block, px: f32, pz: f32, dx: f32, dz: f32) -> Option<(f32, f32, f32)> {
    let (qx, qz) = nearest(b, px, pz);
    let (ex, ez) = (px - qx, pz - qz);
    let d0 = (ex * ex + ez * ez).sqrt();
    if d0 <= R + 1e-4 {
        if d0 > 0.0 {
            // On a face, the face's own normal (axis-aligned), so the slide is exact; at a corner, the radial one.
            let (nx, nz) = if ex == 0.0 { (0.0, ez.signum()) } else if ez == 0.0 { (ex.signum(), 0.0) } else { (ex / d0, ez / d0) };
            if dx * nx + dz * nz >= 0.0 { return None; }
            return Some((0.0, nx, nz));
        }
        return None;
    }
    let (lo, hi) = ([b.min[0] - R, b.min[2] - R], [b.max[0] + R, b.max[2] + R]);
    let (p, d) = ([px, pz], [dx, dz]);
    let (mut t0, mut t1, mut axis) = (0.0_f32, 1.0_f32, 2usize);
    for k in 0..2 {
        if d[k] == 0.0 { if p[k] < lo[k] || p[k] > hi[k] { return None; } continue; }
        let (mut a, mut c) = ((lo[k] - p[k]) / d[k], (hi[k] - p[k]) / d[k]);
        if a > c { std::mem::swap(&mut a, &mut c); }
        if a > t0 || (axis == 2 && a >= t0) { t0 = a; axis = k; }
        if c < t1 { t1 = c; }
        if t0 > t1 { return None; }
    }
    if axis == 2 { return None; }
    let (hx, hz) = (px + dx * t0, pz + dz * t0);
    if (hx >= b.min[0] && hx <= b.max[0]) || (hz >= b.min[2] && hz <= b.max[2]) {
        let (nx, nz) = if axis == 0 { (-d[0].signum(), 0.0) } else { (0.0, -d[1].signum()) };
        return Some((t0, nx, nz));
    }
    let cx = if hx < b.min[0] { b.min[0] } else { b.max[0] };
    let cz = if hz < b.min[2] { b.min[2] } else { b.max[2] };
    let (fx, fz) = (px - cx, pz - cz);
    let a = dx * dx + dz * dz;
    let bq = 2.0 * (fx * dx + fz * dz);
    let cq = fx * fx + fz * fz - R * R;
    let disc = bq * bq - 4.0 * a * cq;
    if disc < 0.0 { return None; }
    let t = (-bq - disc.sqrt()) / (2.0 * a);
    if !(0.0..=1.0).contains(&t) { return None; }
    Some((t, (fx + dx * t) / R, (fz + dz * t) / R))
}

/// A body inside a solid: the centre inside a footprint with the feet more than 1 mm below the top there (and the
/// body's height range meeting the shape's), or the body circle more than 1 mm into a shape that is a wall for it.
fn embedded(world: &[Block], s: &S, body: Body) -> bool {
    let e = eps(body);
    for b in world {
        if b.min[1] >= s.y + BODY_H { continue; }
        let inside = s.x > b.min[0] + 1e-4 && s.x < b.max[0] - 1e-4 && s.z > b.min[2] + 1e-4 && s.z < b.max[2] - 1e-4;
        if inside && s.y < height_at(b, s.x, s.z) - 1e-3 && s.y + BODY_H > b.min[1] { return true; }
        let h = height_near(b, s.x, s.z);
        if h > s.y + STEP + e && dist2(b, s.x, s.z) < (R - 1e-3) * (R - 1e-3) && s.y < h { return true; }
    }
    false
}

fn start(x: f32, z: f32, y: f32) -> S {
    if TRACE_BITS.load(std::sync::atomic::Ordering::Relaxed) { eprintln!("S"); }
    S { x, z, y, vy: 0.0, grounded: true, pull: None, stepped: None, steps: 0, falls_unjumped: 0, airborne_ticks: 0, max_float: 0.0, open_float: 0.0, fall_at: None,
        cfg: Cfg { land_prev: LAND_PREV_ALL.load(std::sync::atomic::Ordering::Relaxed), ..Cfg::default() }, vx: 0.0, vz: 0.0, lvx: 0.0, lvz: 0.0, fall_from: None, flickers: 0, head_hits: 0, max_drop: 0.0, wall_hits: 0, y_moved: 0.0, below_top: 0.0,
        pullv: None, pull_carry: 0.0, setbacks: 0, max_setback: 0.0, max_rise: 0.0 }
}

// ---------------------------------------------------------------------------------------------------------------
// The push.
#[derive(Clone, Copy, PartialEq)]
enum WallRule { PushOut, Sweep }

#[derive(Default, Clone, Copy)]
struct PushStats { pairs: u32, capped: u32, kept: u32, most: f32, turned: u32 }

/// A swept circle (radius R) from (px, pz) by (dx, dz) against a block's footprint: the first contact's share of the
/// move and its outward normal. Touching (within the skin) and moving along or away is no hit.
fn sweep_rect(b: &Block, px: f32, pz: f32, dx: f32, dz: f32) -> Option<(f32, f32, f32)> {
    let (qx, qz) = nearest(b, px, pz);
    let (ex, ez) = (px - qx, pz - qz);
    let d0 = (ex * ex + ez * ez).sqrt();
    if d0 < R + SKIN {
        if d0 > 0.0 { let (nx, nz) = (ex / d0, ez / d0); if dx * nx + dz * nz >= 0.0 { return None; } return Some((0.0, nx, nz)); }
        return None; // centre inside the footprint: left to the push-out
    }
    let (lo, hi) = ([b.min[0] - R, b.min[2] - R], [b.max[0] + R, b.max[2] + R]);
    let (p, d) = ([px, pz], [dx, dz]);
    let (mut t0, mut t1, mut axis) = (0.0_f32, 1.0_f32, 2usize);
    for k in 0..2 {
        if d[k] == 0.0 { if p[k] < lo[k] || p[k] > hi[k] { return None; } continue; }
        let (mut a, mut c) = ((lo[k] - p[k]) / d[k], (hi[k] - p[k]) / d[k]);
        if a > c { std::mem::swap(&mut a, &mut c); }
        if a > t0 { t0 = a; axis = k; }
        if c < t1 { t1 = c; }
        if t0 > t1 { return None; }
    }
    if axis == 2 { return None; }
    let (hx, hz) = (px + dx * t0, pz + dz * t0);
    let in_x = hx >= b.min[0] && hx <= b.max[0];
    let in_z = hz >= b.min[2] && hz <= b.max[2];
    if in_x || in_z {
        let (nx, nz) = if axis == 0 { (-d[0].signum(), 0.0) } else { (0.0, -d[1].signum()) };
        return Some((t0, nx, nz));
    }
    let cx = if hx < b.min[0] { b.min[0] } else { b.max[0] };
    let cz = if hz < b.min[2] { b.min[2] } else { b.max[2] };
    let (fx, fz) = (px - cx, pz - cz);
    let a = dx * dx + dz * dz;
    let bq = 2.0 * (fx * dx + fz * dz);
    let cq = fx * fx + fz * fz - R * R;
    let disc = bq * bq - 4.0 * a * cq;
    if disc < 0.0 { return None; }
    let t = (-bq - disc.sqrt()) / (2.0 * a);
    if !(0.0..=1.0).contains(&t) { return None; }
    Some((t, (fx + dx * t) / R, (fz + dz * t) / R))
}

/// The plan's rule for the push: the pushed move swept against the walls, stopped a skin short, one slide.
fn sweep_move(world: &[Block], s: &mut S, mut mx: f32, mut mz: f32, e: f32) {
    for _ in 0..2 {
        let len = (mx * mx + mz * mz).sqrt();
        if len < 1e-7 { break; }
        let mut best: Option<(f32, f32, f32)> = None;
        for b in world {
            if floating(b, s.y, s.cfg) || height_near(b, s.x, s.z) <= s.y + STEP + e { continue; }
            if let Some(h) = sweep_rect(b, s.x, s.z, mx, mz) { if best.is_none_or(|bb| h.0 < bb.0) { best = Some(h); } }
        }
        match best {
            None => { s.x += mx; s.z += mz; break; }
            Some((t, nx, nz)) => {
                let go = (t - SKIN / len).max(0.0);
                s.x += mx * go; s.z += mz * go;
                let (rx, rz) = (mx * (1.0 - go), mz * (1.0 - go));
                let vn = rx * nx + rz * nz;
                mx = rx - vn * nx; mz = rz - vn * nz;
            }
        }
    }
}

/// One push pass over all pairs, in slot order, the sums applied after (block 3 items 1 to 9). The perpendicular for
/// the head-on turn is side x (-n.z, n.x) (the reading does not say which; the choice only mirrors the turn). Two
/// bodies exactly on one spot take n = +x from the lower slot (the sandbox's per-pair angle is trigonometry).
/// Standing is not run again after the push (the reading does not say; the next tick's own standing applies).
fn push(world: &[Block], ss: &mut [S], body: Body, rule: WallRule, st: &mut PushStats) {
    let share = 1.0 - (-PUSH_STRENGTH * DT).exp();
    let min = 2.0 * PUSH_RADIUS;
    let n = ss.len();
    let mut sums = [(0.0_f32, 0.0_f32); 4];
    for i in 0..n { for j in (i + 1)..n {
        let (a, b) = (ss[i], ss[j]);
        if (a.y - b.y).abs() >= HEIGHT_GAP { continue; }
        let (dx, dz) = (b.x - a.x, b.z - a.z);
        let d2 = dx * dx + dz * dz;
        if d2 >= min * min { continue; }
        let dist = d2.sqrt();
        let (mut nx, mut nz) = if dist < 1e-6 { (1.0, 0.0) } else { (dx / dist, dz / dist) };
        let (rvx, rvz) = (a.lvx - b.lvx, a.lvz - b.lvz);
        let rel = (rvx * rvx + rvz * rvz).sqrt();
        if rel > HEAD_ON_GATE {
            let head = (nx * rvx + nz * rvz).abs() / rel;
            let cross = nx * rvz - nz * rvx;
            let side = if cross < 0.0 { -1.0 } else { 1.0 };
            let (tx, tz) = (-nz * side, nx * side);
            nx += tx * SIDEWAYS * head; nz += tz * SIDEWAYS * head;
            let l = (nx * nx + nz * nz).sqrt();
            nx /= l; nz /= l;
            if head > 0.0 { st.turned += 1; }
        }
        let m = (min - dist) * share * 0.5;
        sums[i].0 -= nx * m; sums[i].1 -= nz * m;
        sums[j].0 += nx * m; sums[j].1 += nz * m;
        st.pairs += 1;
    }}
    for i in 0..n {
        let (mut px, mut pz) = sums[i];
        let s = &mut ss[i];
        let sp = (s.lvx * s.lvx + s.lvz * s.lvz).sqrt();
        if sp > KEEP_GATE {
            let (ux, uz) = (s.lvx / sp, s.lvz / sp);
            let along = px * ux + pz * uz;
            let limit = -KEEP_GOING * sp * DT;
            if along < limit { px -= ux * (along - limit); pz -= uz * (along - limit); st.kept += 1; }
        }
        let len = (px * px + pz * pz).sqrt();
        if len > MAX_STEP { px *= MAX_STEP / len; pz *= MAX_STEP / len; st.capped += 1; }
        let len = (px * px + pz * pz).sqrt();
        if len > st.most { st.most = len; }
        if len == 0.0 { continue; }
        match rule {
            WallRule::PushOut => { s.x += px; s.z += pz; let y = s.y; walls(world, s, y, eps(body)); }
            WallRule::Sweep => { sweep_move(world, s, px, pz, eps(body)); }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
fn block5(world: &[Block], body: Body, v0: f32) {
    let speeds = [6.3_f32, 9.576, 13.27];
    for (face, top) in [(30.0_f32, 0.55_f32), (34.0, 0.56)] {
        for sp in speeds { for d in [1.2_f32, 1.13, 1.07] {
            let mut s = start(face - R - d, -9.0, 0.0);
            let mut on_top_at = None;
            for t in 0..60 {
                tick(world, &mut s, body, (1.0, 0.0), sp, false, v0);
                if on_top_at.is_none() && s.x >= face + 1.0 { on_top_at = Some((t, s.y, s.grounded)); break; }
            }
            let pass = s.falls_unjumped == 0 && if top < 0.555 { matches!(on_top_at, Some((_, y, true)) if y == top) } else { on_top_at.is_none() && s.y == 0.0 && s.x <= face - R + 1e-4 };
            println!("step {top} speed {sp} start {:.2}: {} x {:.4} feet {:.4} steps {} unjumped-falls {} airborne-ticks {}",
                face - R - d, if pass { "PASS" } else { "FAIL" }, s.x, s.y, s.steps, s.falls_unjumped, s.airborne_ticks);
        }}
    }
    for sp in [1.0_f32, 2.0, 6.3] { for d in [1.2_f32, 1.13, 1.07] { for stop in [false, true] {
        if !stop && sp == 6.3 { continue; }
        let mut s = start(30.0 - R - d, -9.0, 0.0);
        let mut stopped_at = None;
        for t in 0..400 {
            let wish = if stopped_at.is_some() { (0.0, 0.0) } else { (1.0, 0.0) };
            tick(world, &mut s, body, wish, sp, false, v0);
            if stop && stopped_at.is_none() && s.steps > 0 { stopped_at = Some(t); }
            if let Some(t0) = stopped_at { if t >= t0 + 20 { break; } }
            if !stop && s.x >= 31.0 { break; }
        }
        let pass = s.y == 0.55 && s.grounded && s.falls_unjumped == 0;
        println!("slow step 0.55 speed {sp} start {:.2} {}: {} x {:.4} feet {:.4} centre past the face {:.4} steps {} unjumped-falls {}",
            30.0 - R - d, if stop { "stop at touch" } else { "walk on" }, if pass { "PASS" } else { "FAIL" }, s.x, s.y, s.x - 30.0, s.steps, s.falls_unjumped);
    }}}
    for (face, top) in [(35.0_f32, 3.05_f32), (40.0, 3.1)] {
        for sp in speeds { for d in [0.3_f32, 0.8, 1.2] {
            let mut s = start(face - R - d, -2.5, 0.0);
            let mut first_top = None;
            for t in 0..90 {
                tick(world, &mut s, body, (1.0, 0.0), sp, t == 0, v0);
                if first_top.is_none() && s.grounded && s.y == top { first_top = Some(t); }
                if s.x >= face + 1.2 { break; }
            }
            let caught = s.y == top && s.grounded;
            let pass = s.falls_unjumped == 0 && if top < 3.076 { caught } else { !caught && first_top.is_none() };
            println!("ledge {top} speed {sp} jump {d} m out: {} feet {:.4} x {:.3} on-top-tick {:?} unjumped-falls {} airborne-ticks {}",
                if pass { "PASS" } else { "FAIL" }, s.y, s.x, first_top, s.falls_unjumped, s.airborne_ticks);
        }}
    }
    for (name, z, top) in [("gentle 25%", 6.0_f32, 1.0_f32), ("steep 50%", 12.0, 1.5)] {
        for sp in speeds { for x0 in [38.0_f32, 38.07, 38.13] {
            let mut s = start(x0, z, 0.0);
            for _ in 0..90 { tick(world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.x >= 47.0 { break; } }
            let up_ok = s.y == top && s.grounded;
            let (up_float, up_open, up_air, up_falls) = (s.max_float, s.open_float, s.airborne_ticks, s.falls_unjumped);
            let mut d = start(47.0 + (x0 - 38.0), z, top);
            for _ in 0..90 { tick(world, &mut d, body, (-1.0, 0.0), sp, false, v0); if d.x <= 39.0 { break; } }
            let down_ok = d.y == 0.0 && d.grounded;
            println!("ramp {name} speed {sp} start {x0}: up {} (float open {up_open:.4} / at the seams {:.4} m, airborne ticks {up_air}, falls {up_falls}) down {} (float open {:.4} / seams {:.4} m, airborne ticks {}, falls {})",
                if up_ok { "PASS" } else { "FAIL" }, up_float, if down_ok { "PASS" } else { "FAIL" }, d.open_float, d.max_float, d.airborne_ticks, d.falls_unjumped);
        }}
    }
    for (name, x, face, ridge) in [("house a 95%", -9.0_f32, -8.0_f32, 4.7_f32), ("house b 89%", -2.0, -7.4, 4.3)] {
        for sp in speeds { for d in [0.5_f32, 1.0, 1.5] {
            let mut s = start(x, face - R - d, 0.0);
            let mut highest = 0.0_f32;
            let mut gained_edge = false;
            for t in 0..120 {
                tick(world, &mut s, body, (0.0, 1.0), sp, t == 0, v0);
                if s.grounded && s.y >= 2.6 { gained_edge = true; }
                if s.grounded && s.y > highest { highest = s.y; }
                if s.z >= face + 2.1 { break; }
            }
            println!("roof {name} speed {sp} jump {d} m out: edge gained {gained_edge}, highest standing {:.3} m (ridge {ridge}), float open {:.4} / seams {:.4}, unjumped-falls {}",
                highest, s.open_float, s.max_float, s.falls_unjumped);
        }}
    }
    let (mut seed, mut through) = (0x9e37_79b9_u32, 0);
    for _ in 0..10_000 {
        let mut next = || { seed ^= seed << 13; seed ^= seed >> 17; seed ^= seed << 5; seed };
        let x = 29.0 + (next() % 600) as f32 / 1000.0;
        let z = 4.5 + (next() % 5000) as f32 / 1000.0;
        let side = (next() % 1000) as f32 / 1000.0 - 0.5;
        let len = (1.0 + side * side).sqrt();
        let mut s = start(x, z, 0.0);
        tick(world, &mut s, body, (1.0 / len, side / len), 13.27, false, v0);
        if s.x > 30.1 { through += 1; }
    }
    println!("thin wall at 13.27 m/s: {through} of 10000 past the mid-plane");
    for (name, x0, z, y, edge) in [("platform 1.0 m", 47.0_f32, 6.0_f32, 1.0_f32, 49.0_f32), ("step 0.55 m", 31.0, -9.0, 0.55, 32.0)] {
        for sp in speeds {
            let mut s = start(x0, z, y);
            for _ in 0..30 { tick(world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.x > edge + 1.5 { break; } }
            let at = s.fall_at.map(|(x, _)| x - edge);
            println!("off {name} speed {sp}: {} (centre past the edge when it started: {:?} m), feet now {:.3}",
                if s.fall_at.is_some() { "fell" } else { "snapped down" }, at, s.y);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
fn pf(b: bool) -> &'static str { if b { "PASS" } else { "FAIL" } }

fn block6(world: &[Block], body: Body, v0: f32) {
    let speeds = [6.3_f32, 9.576, 13.27];
    let mut tally: Vec<(&str, u32, u32)> = Vec::new();
    let count = |name: &'static str, pass: bool, tally: &mut Vec<(&'static str, u32, u32)>| {
        if let Some(t) = tally.iter_mut().find(|t| t.0 == name) { t.1 += 1; t.2 += u32::from(pass); } else { tally.push((name, 1, u32::from(pass))); }
    };

    // A. Ramps met from their sides, along z, at several x. Outcome expected from the height at the contact: on the
    // slope when it is within the step (3a/3b: the height under the centre, the reading's heightNear; body 2: the
    // disc's highest point), else stopped at the side by the push-out stand-in (the sandbox's set-back along the slope
    // is not run: the reading gives no formula). Body 2 between its two heights may do either: no embed is the test.
    println!("# A. ramps from their sides (wall rule: block 5's push-out, a stand-in for the set-back)");
    let gentle = 13usize; let steep = 15usize;
    let cases: [(&str, usize, f32, f32, &[f32]); 4] = [
        ("gentle from -z", gentle, 4.0, 1.0, &[41.2, 42.0, 42.9, 43.0, 43.1, 43.6, 44.4, 45.0]),
        ("gentle from +z", gentle, 8.0, -1.0, &[41.2, 42.0, 42.9, 43.0, 43.1, 43.6, 44.4, 45.0]),
        ("steep from -z", steep, 10.0, 1.0, &[42.2, 42.6, 42.8, 42.9, 43.0, 43.2, 43.6, 44.4, 45.0]),
        ("steep from +z", steep, 14.0, -1.0, &[42.2, 42.6, 42.8, 42.9, 43.0, 43.2, 43.6, 44.4, 45.0]),
    ];
    for (name, ri, face, dirz, xs) in cases {
        let b = world[ri];
        // The gap between the ramps (z 8 to 10) is 2 m: starts there sit 0.37 to 0.5 m short of the face.
        let ds: [f32; 3] = if (name.contains("gentle") && dirz < 0.0) || (name.contains("steep") && dirz > 0.0) { [0.5, 0.43, 0.37] } else { [1.2, 1.13, 1.07] };
        for &x in xs { for sp in speeds { for d in ds {
            let z0 = face - dirz * (R + d);
            let mut s = start(x, z0, 0.0);
            let mut emb = 0u32;
            for _ in 0..40 {
                tick(world, &mut s, body, (0.0, dirz), sp, false, v0);
                emb += u32::from(embedded(world, &s, body));
                if (s.z - face) * dirz >= 1.0 { break; }
            }
            let h_near = height_at(&b, x, 0.0);
            let h_body = if body == Body::Cyl { height_max(&b, x, 0.0, R) } else { h_near };
            let lim = STEP + eps(body);
            let on = s.grounded && (s.z - face) * dirz > 0.0 && s.y > 0.0;
            let stopped = s.y == 0.0 && ((face - s.z) * dirz - R).abs() < 1e-3;
            let expect_on = h_body <= lim;
            let expect_wall = h_near > lim;
            let stand = if body == Body::Cyl { height_max(&b, s.x, s.z, R) } else { height_at(&b, s.x, s.z) };
            let outcome_ok = if expect_on { on && s.y == stand } else if expect_wall { stopped } else { on || stopped };
            let pass = outcome_ok && emb == 0 && s.flickers == 0 && s.falls_unjumped == 0;
            count("A ramps from the side", pass, &mut tally);
            println!("side {name} x {x} speed {sp} start {:.2}: {} {} feet {:.4} (slope at centre {:.4}, under the disc {:.4}) z {:.4} embedded-ticks {emb} flickers {} unjumped-falls {} steps {}",
                (face - s.z).abs(), pf(pass), if on { "on the slope" } else if stopped { "stopped at the side" } else { "neither" }, s.y, h_near, h_body, s.z, s.flickers, s.falls_unjumped, s.steps);
        }}}
    }
    // A2. Walking off a ramp's side (start on the slope at z 6 or 12, walk out across the side): fall or snap.
    println!("# A2. off a ramp's side");
    for (name, ri, z0, dirz, face) in [("gentle off -z", gentle, 6.0_f32, -1.0_f32, 4.0_f32), ("steep off +z", steep, 12.0, 1.0, 14.0)] {
        let b = world[ri];
        for x in [41.6_f32, 42.6, 43.6, 44.6] { for sp in speeds {
            let h = if body == Body::Cyl { height_max(&b, x, z0, R) } else { height_at(&b, x, z0) };
            let mut s = start(x, z0, h);
            let mut emb = 0u32;
            for _ in 0..60 { tick(world, &mut s, body, (0.0, dirz), sp, false, v0); emb += u32::from(embedded(world, &s, body)); if (s.z - face) * dirz >= 1.5 && s.grounded { break; } }
            let pass = s.grounded && s.y == 0.0 && emb == 0 && s.flickers == 0;
            count("A2 off a ramp's side", pass, &mut tally);
            println!("off-side {name} x {x} speed {sp}: {} from {h:.4}: {} (centre past the side when it started: {:?}) airborne-ticks {} embedded-ticks {emb} flickers {}",
                pf(pass), if s.fall_at.is_some() { "fell" } else { "snapped down" }, s.fall_at.map(|(_, z)| (z - face) * dirz), s.airborne_ticks, s.flickers);
        }}
    }

    // B. Landings: off the 2.9 and 3.05 m ledges, the two platforms and the 0.55 m step, 8 directions, walking off
    // or jumping, three speeds; after landing, 15 ticks standing still. Pass: lands on a top or the ground at the
    // standing height there, stays there every tick (no bounce, no fall), no flicker, never inside a solid.
    println!("# B. landings");
    let tops: [(&str, f32, f32, f32); 5] = [("ledge 2.9", 31.5, -2.5, 2.9), ("ledge 3.05", 36.5, -2.5, 3.05), ("platform 1.0", 47.0, 6.0, 1.0), ("platform 1.5", 47.0, 12.0, 1.5), ("step 0.55", 31.0, -9.0, 0.55)];
    let dirs = [(1.0_f32, 0.0_f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (0.7071, 0.7071), (0.7071, -0.7071), (-0.7071, 0.7071), (-0.7071, -0.7071)];
    let mut land_hist: Vec<(String, u32)> = Vec::new();
    let mut worst_drop = 0.0_f32;
    let mut worst_below = 0.0_f32;
    for (name, x0, z0, y0) in tops { for jump in [false, true] { for sp in speeds { for dir in dirs {
        let mut s = start(x0, z0, y0);
        let (mut landed_at, mut emb, mut bounce) = (None, 0u32, 0u32);
        let mut was_air = false;
        let mut after = 0;
        for t in 0..120 {
            let still = landed_at.is_some();
            let y_before = s.y;
            tick(world, &mut s, body, if still { (0.0, 0.0) } else { dir }, sp, jump && t == 0, v0);
            emb += u32::from(embedded(world, &s, body));
            if !s.grounded { was_air = true; }
            if let Some((_, ly)) = landed_at {
                after += 1;
                if !s.grounded || s.y != ly || s.y != y_before { bounce += 1; }
                if after >= 15 { break; }
            } else if was_air && s.grounded { landed_at = Some((t, s.y)); }
        }
        let walked_down = landed_at.is_none() && s.airborne_ticks == 0;
        let pass = (landed_at.is_some() || walked_down) && bounce == 0 && emb == 0 && s.flickers == 0;
        count("B landings", pass, &mut tally);
        if s.below_top > worst_below { worst_below = s.below_top; }
        if s.max_drop > worst_drop { worst_drop = s.max_drop; }
        let key = if walked_down { format!("{name} -> walked down, never airborne") } else { format!("{name} -> {:.2}", landed_at.map_or(-1.0, |l| l.1)) };
        if let Some(h) = land_hist.iter_mut().find(|h| h.0 == key) { h.1 += 1; } else { land_hist.push((key, 1)); }
        if !pass { println!("land {name} jump {jump} speed {sp} dir {dir:?}: FAIL landed {landed_at:?} bounce-ticks {bounce} embedded-ticks {emb} flickers {} airborne {}", s.flickers, s.airborne_ticks); }
    }}}}
    for h in &land_hist { println!("land outcome {}: {} runs", h.0, h.1); }
    println!("land: largest drop in one airborne tick {worst_drop:.4} m; deepest moved feet below a top (not the ground) in its landing tick {worst_below:.4} m (the landing test reaches {STEP} m above the feet)");
    // Air time against the arc: walking off the 2.9 m ledge to the ground and jumping off it.
    for jump in [false, true] {
        let mut s = start(31.5, -2.5, 2.9);
        let mut t_land = None;
        for t in 0..90 { tick(world, &mut s, body, (-1.0, 0.0), 6.3, jump && t == 0, v0); if t_land.is_none() && s.grounded && s.y == 0.0 && s.airborne_ticks > 0 { t_land = Some(t); break; } }
        let exact = if jump { (v0 + (v0 * v0 + 2.0 * G * 2.9).sqrt()) / G } else { (2.0 * 2.9 / G).sqrt() };
        println!("air time off the 2.9 ledge walking -x at 6.3, jump {jump}: airborne ticks {} landing tick {t_land:?}; exact fall time {:.4} s = {:.2} ticks (+ the walk to the edge)", s.airborne_ticks, exact, exact / DT);
    }
    // B2. The high drop (made map, block 4 there: top 2.0 m): a fall from 8.0 to 9.0 m in 0.1 m steps, onto the top.
    println!("# B2. a fall from 10 to 18 m onto a 2.0 m top (made map), the reading's landing and the candidate fix");
    let cmap = ceiling_map();
    for land_prev in [false, true] {
        let mut miss = 0;
        for k in 0..21 {
            let mut s = start(56.0, 6.0, 10.0 + k as f32 * 0.4);
            s.grounded = false; s.cfg.land_prev = land_prev;
            for _ in 0..60 { tick(&cmap, &mut s, body, (0.0, 0.0), 0.0, false, v0); if s.grounded { break; } }
            if s.y != 2.0 { miss += 1; }
        }
        println!("high drop from 10 to 18 m, land on a top crossed in the tick {land_prev}: {miss} of 21 missed the top (fell inside it and were put out to the ground)");
    }

    // C. The snap's numbers: the allowance and the drop a tick, down the 0.55 m step and down the two ramps.
    println!("# C. the snap");
    for sp in speeds {
        let mv = sp * DT;
        if body == Body::Cyl {
            println!("snap speed {sp}: allowance {STEP} everywhere (snap = step-up); a ramp's drop a tick 25 % {:.4}, 50 % {:.4}", mv * 0.25, mv * 0.5);
        } else {
            println!("snap speed {sp}: allowance flat {SNAP}, 25 % {:.4} (drop a tick {:.4}), 50 % {:.4} (drop {:.4}), the 0.55 step 0.55 > {SNAP}: a fall",
                SNAP + (mv + FOOT) * 0.25, mv * 0.25, SNAP + (mv + FOOT) * 0.5, mv * 0.5);
        }
        let mut s = start(31.0, -9.0, 0.55);
        for _ in 0..40 { tick(world, &mut s, body, (0.0, -1.0), sp, false, v0); if s.z < -12.0 { break; } }
        println!("  down the 0.55 step at {sp}: {} airborne ticks {} flickers {} feet {:.3}", if s.fall_at.is_some() { "fell" } else { "snapped" }, s.airborne_ticks, s.flickers, s.y);
        for (name, z, top) in [("gentle", 6.0_f32, 1.0_f32), ("steep", 12.0, 1.5)] {
            let mut s = start(47.0, z, top);
            let mut margin = f32::INFINITY;
            let mut biggest = 0.0_f32;
            for _ in 0..90 {
                let y0 = s.y;
                tick(world, &mut s, body, (-1.0, 0.0), sp, false, v0);
                if s.grounded && y0 > s.y {
                    let allow = if body == Body::Cyl { STEP } else { SNAP + (mv + FOOT) * slope_under(world, s.x, s.z) };
                    biggest = biggest.max(y0 - s.y);
                    margin = margin.min(allow - (y0 - s.y));
                }
                if s.x <= 39.0 { break; }
            }
            let pass = s.grounded && s.y == 0.0 && s.airborne_ticks == 0;
            count("C snap down the ramps", pass, &mut tally);
            println!("  down the {name} ramp at {sp}: {} largest drop a tick {biggest:.4}, smallest margin under the allowance {margin:.4}, airborne ticks {}", pf(pass), s.airborne_ticks);
        }
    }

    // D. Easing's first ticks: from rest at the 0.55 and 0.56 steps (0.05 to 1.2 m from the touch), walk on, or let
    // go the tick the step fires (the speed eases down); and the 3.05 / 3.10 ledges jumped from rest.
    println!("# D. easing (k {EASE_K} a tick, up = stop, air 100 %)");
    for (face, top) in [(30.0_f32, 0.55_f32), (34.0, 0.56)] { for d in [0.05_f32, 0.3, 0.6, 1.2] { for sp in speeds { for letgo in [false, true] {
        let mut s = start(face - R - d, -9.0, 0.0);
        s.cfg.ease = true;
        let mut gone = None;
        let mut emb = 0;
        for t in 0..90 {
            let wish = if gone.is_some() { (0.0, 0.0) } else { (1.0, 0.0) };
            tick(world, &mut s, body, wish, sp, false, v0);
            emb += u32::from(embedded(world, &s, body));
            if letgo && gone.is_none() && s.steps > 0 { gone = Some(t); }
            if let Some(t0) = gone { if t >= t0 + 30 { break; } }
            if !letgo && s.x >= face + 1.0 { break; }
            if top > 0.555 && t > 40 { break; }
        }
        let pass = emb == 0 && s.falls_unjumped == 0 && if top < 0.555 { s.y == top && s.grounded } else { s.y == 0.0 && s.steps == 0 && s.x <= face - R + 1e-4 };
        count("D easing at the steps", pass, &mut tally);
        if !pass || d == 0.05 { println!("ease step {top} from {d} m speed {sp} {}: {} x {:.4} feet {:.4} speed now {:.4} steps {} unjumped-falls {} flickers {}",
            if letgo { "let go at the step" } else { "walk on" }, pf(pass), s.x, s.y, s.vx, s.steps, s.falls_unjumped, s.flickers); }
    }}}}
    for (face, top) in [(35.0_f32, 3.05_f32), (40.0, 3.1)] { for d in [0.05_f32, 0.3, 0.8, 1.2] { for sp in speeds {
        let mut s = start(face - R - d, -2.5, 0.0);
        s.cfg.ease = true;
        for t in 0..90 { tick(world, &mut s, body, (1.0, 0.0), sp, t == 0, v0); if s.x >= face + 1.2 { break; } }
        let caught = s.y == top && s.grounded;
        let pass = s.falls_unjumped == 0 && if top < 3.076 { caught } else { !caught };
        count("D easing at the ledges", pass, &mut tally);
        println!("ease ledge {top} jump from rest {d} m out speed {sp}: {} feet {:.4} x {:.3}", pf(pass), s.y, s.x);
    }}}

    // E. Ceilings (made map; the reading shows no head rule: these runs show cost only, never a result).
    println!("# E. ceilings (made map; C1 = no head rule with block 5's floating test, C2 = a head stop; cost only)");
    for (cname, head) in [("C1", false), ("C2", true)] {
        for sp in speeds {
            // walk under the 2.0 m beam, into the 1.7 m beam, and up the step under the 2.2 m beam
            for (what, x0, end) in [("under the 2.0 beam", 52.0_f32, 58.5_f32), ("into the 1.7 beam", 58.0, 63.0), ("up the step under the 2.2 beam", 63.0, 69.0)] {
                let mut s = start(x0, 0.0, 0.0);
                s.cfg.head_stop = head;
                let mut emb = 0;
                for _ in 0..60 { tick(&cmap, &mut s, body, (1.0, 0.0), sp, false, v0); emb += u32::from(embedded(&cmap, &s, body)); if s.x >= end { break; } }
                println!("ceiling {cname} {what} at {sp}: x {:.4} z {:.4} feet {:.4} head {:.4} embedded-ticks {emb} wall-pushes {}", s.x, s.z, s.y, s.y + BODY_H, s.wall_hits);
            }
            // jump standing still under the 2.0 beam, and jump while walking under it
            for (what, walk) in [("jump standing under the 2.0 beam", false), ("jump walking under the 2.0 beam", true)] {
                let mut s = start(if walk { 54.0 } else { 56.0 }, 0.0, 0.0);
                s.cfg.head_stop = head;
                let (mut emb, mut top, mut maxdz) = (0, 0.0_f32, 0.0_f32);
                for t in 0..60 {
                    tick(&cmap, &mut s, body, if walk { (1.0, 0.0) } else { (0.0, 0.0) }, sp, t == 0, v0);
                    emb += u32::from(embedded(&cmap, &s, body));
                    if std::env::var("TRACE6").is_ok() && walk && sp == 13.27 { println!("    trace {cname} t {t} x {:.4} y {:.4} vy {:.3} grounded {} emb {}", s.x, s.y, s.vy, s.grounded, embedded(&cmap, &s, body)); }
                    top = top.max(s.y); maxdz = maxdz.max(s.z.abs());
                    if s.grounded && t > 0 { break; }
                }
                println!("ceiling {cname} {what} at {sp}: highest feet {top:.4} (head {:.4}; beam bottom 2.0) ended x {:.4} z {:.4} feet {:.4} moved along x {:.4} sideways at most {maxdz:.4} head-hits {} embedded-ticks {emb}", top + BODY_H, s.x, s.z, s.y, s.x - if walk { 54.0 } else { 56.0 }, s.head_hits);
            }
        }
    }

    // G. Block 5's ledge jumps and the beam again, counting ticks inside a solid, with and without the candidate fix.
    println!("# G. ledges with the inside counter");
    let fix_before = TOWARD_START.load(std::sync::atomic::Ordering::Relaxed);
    for fix in [false, true] {
        TOWARD_START.store(fix, std::sync::atomic::Ordering::Relaxed);
        for (face, top) in [(30.0_f32, 2.9_f32), (35.0, 3.05), (40.0, 3.1)] {
            let mut z = 0.0_f32;
            for sp in speeds { for d in [0.05_f32, 0.3, 0.8, 1.2] {
                let mut s = start(face - R - d, -2.5, 0.0);
                let mut emb = 0;
                for t in 0..90 { tick(world, &mut s, body, (1.0, 0.0), sp, t == 0, v0); emb += u32::from(embedded(world, &s, body)); if s.x >= face + 1.2 { break; } }
                let caught = s.y == top && s.grounded;
                let pass = emb == 0 && s.falls_unjumped == 0 && if top < 3.076 { caught } else { !caught };
                count(if fix { "G ledges, toward from the tick's start" } else { "G ledges, block 5's toward" }, pass, &mut tally);
                if emb > 0 { z += 1.0; println!("ledge {top} speed {sp} jump {d} m out, fix {fix}: {} caught {caught} inside-ticks {emb}", pf(pass)); }
            }}
            println!("ledge {top} fix {fix}: {z} of 12 runs spent a tick inside the block");
        }
        let cmap = ceiling_map();
        for sp in speeds { for head in [false, true] {
            let mut s = start(54.0, 0.0, 0.0);
            s.cfg.head_stop = head;
            let mut emb = 0;
            for t in 0..60 { tick(&cmap, &mut s, body, (1.0, 0.0), sp, t == 0, v0); emb += u32::from(embedded(&cmap, &s, body)); if s.grounded && t > 0 { break; } }
            println!("beam jump walking {sp} head-stop {head} fix {fix}: ended x {:.4} feet {:.4} inside-ticks {emb}", s.x, s.y);
        }}
    }
    TOWARD_START.store(fix_before, std::sync::atomic::Ordering::Relaxed);

    // F. The push, game-designer-read.md block 3 (velocities: this tick's displacement / dt, after the walls).
    println!("# F. the push (share {:.5} a tick)", 1.0 - (-PUSH_STRENGTH * DT).exp());
    struct P { name: &'static str, starts: Vec<(f32, f32, f32)>, wishes: Vec<(f32, f32)>, speed: f32, ticks: u32 }
    let cases = vec![
        P { name: "head-on 6.3", starts: vec![(60.0, 0.0, 0.0), (64.0, 0.0, 0.0)], wishes: vec![(1.0, 0.0), (-1.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "head-on 13.27", starts: vec![(60.0, 0.0, 0.0), (66.0, 0.0, 0.0)], wishes: vec![(1.0, 0.0), (-1.0, 0.0)], speed: 13.27, ticks: 30 },
        P { name: "glancing 0.5 m", starts: vec![(60.0, 0.0, 0.0), (64.0, 0.5, 0.0)], wishes: vec![(1.0, 0.0), (-1.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "one standing", starts: vec![(60.0, 0.0, 0.0), (63.0, 0.0, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "both standing 0.3 m apart", starts: vec![(60.0, 0.0, 0.0), (60.3, 0.0, 0.0)], wishes: vec![(0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "both standing on one spot", starts: vec![(60.0, 0.0, 0.0), (60.0, 0.0, 0.0)], wishes: vec![(0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "into a wall (3.10 ledge face x 40)", starts: vec![(37.0, -2.5, 0.0), (39.6, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "into a wall, mate 0.15 m clear", starts: vec![(37.0, -2.5, 0.0), (39.45, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "into a wall corner, mate 0.1 m clear of the corner", starts: vec![(37.0, -4.6, 0.0), (39.55, -4.45, 0.0)], wishes: vec![(0.9806, 0.1961), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "into a wall, glancing", starts: vec![(37.0, -2.2, 0.0), (39.6, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "across the 0.55 step, walker below", starts: vec![(28.0, -9.0, 0.0), (30.3, -9.0, 0.55)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "across the 0.55 step, walker above", starts: vec![(31.5, -9.0, 0.55), (29.6, -9.0, 0.0)], wishes: vec![(-1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "at the 1.0 m gap (platform edge x 49)", starts: vec![(51.0, 6.0, 0.0), (48.8, 6.0, 1.0)], wishes: vec![(-1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "three meet", starts: vec![(60.0, 0.0, 0.0), (64.0, 0.0, 0.0), (62.0, 2.0, 0.0)], wishes: vec![(1.0, 0.0), (-1.0, 0.0), (0.0, -1.0)], speed: 6.3, ticks: 40 },
        P { name: "three standing close", starts: vec![(60.0, 0.0, 0.0), (60.3, 0.0, 0.0), (60.15, 0.25, 0.0)], wishes: vec![(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "three into a wall", starts: vec![(37.0, -2.5, 0.0), (39.6, -2.5, 0.0), (38.8, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
    ];
    let perms: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut po_digest = 0u64;
    for c in &cases {
        for rule in [WallRule::PushOut, WallRule::Sweep] {
            let n = c.starts.len();
            let mut finals: Vec<Vec<(u32, u32, u32)>> = Vec::new();
            let mut first: Option<String> = None;
            for perm in perms.iter().filter(|p| n == 3 || (p[2] == 2 && (p[0] < 2))) {
                let order: Vec<usize> = perm.iter().copied().filter(|&k| k < n).collect();
                let mut ss: Vec<S> = order.iter().map(|&k| { let (x, z, y) = c.starts[k]; let mut s = start(x, z, y); s.grounded = true; s }).collect();
                let wishes: Vec<(f32, f32)> = order.iter().map(|&k| c.wishes[k]).collect();
                let mut st = PushStats::default();
                let (mut mind, mut overlap_ticks, mut emb, mut stalls) = (f32::INFINITY, 0u32, 0u32, 0u32);
                for _ in 0..c.ticks {
                    for (k, s) in ss.iter_mut().enumerate() {
                        let x0 = s.x;
                        tick(world, s, body, wishes[k], c.speed, false, v0);
                        if wishes[k] != (0.0, 0.0) && (s.x - x0) * wishes[k].0 <= 0.0 && wishes[k].0 != 0.0 { stalls += 1; }
                    }
                    push(world, &mut ss, body, rule, &mut st);
                    for i in 0..n { emb += u32::from(embedded(world, &ss[i], body)); for j in (i + 1)..n {
                        let d = ((ss[i].x - ss[j].x).powi(2) + (ss[i].z - ss[j].z).powi(2)).sqrt();
                        mind = mind.min(d); if d < 2.0 * PUSH_RADIUS - 1e-4 { overlap_ticks += 1; }
                    }}
                }
                // back to the canonical order for the comparison
                let mut fin = vec![(0, 0, 0); n];
                for (pos, &k) in order.iter().enumerate() { fin[k] = (ss[pos].x.to_bits(), ss[pos].z.to_bits(), ss[pos].y.to_bits()); }
                finals.push(fin);
                if first.is_none() {
                    let mut ends = String::new();
                    for k in 0..n { let pos = order.iter().position(|&o| o == k).unwrap(); let s = &ss[pos]; ends += &format!(" [{k}] ({:.3}, {:.3}, y {:.2}{})", s.x, s.z, s.y, if s.falls_unjumped > 0 { ", fell" } else { "" }); }
                    let mut endd = f32::INFINITY;
                    for i in 0..n { for j in (i + 1)..n { endd = endd.min(((ss[i].x - ss[j].x).powi(2) + (ss[i].z - ss[j].z).powi(2)).sqrt()); } }
                    first = Some(format!("min distance {mind:.4} end distance {endd:.4} overlapping pair-ticks {overlap_ticks} pushes {} turned {} kept-going {} capped {} largest push {:.4} walker stalled ticks {stalls} embedded-ticks {emb};{ends}",
                        st.pairs, st.turned, st.kept, st.capped, st.most));
                }
            }
            let same = finals.iter().all(|f| *f == finals[0]);
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            for &(a, b, c3) in &finals[0] { for w in [a, b, c3] { h ^= u64::from(w); h = h.wrapping_mul(0x0100_0000_01b3); } }
            if rule == WallRule::PushOut { po_digest = h; } else { println!("push {} push-out vs sweep final bits equal: {}", c.name, po_digest == h); }
            println!("push {} [{}]: {} | slot orders {} bit-identical: {same} | final-bits digest {h:016x}", c.name, if rule == WallRule::PushOut { "push-out" } else { "sweep" }, first.unwrap(), finals.len());
        }
    }
    println!("# tally (case group: runs, passes)");
    for t in &tally { println!("tally {}: {} runs, {} pass", t.0, t.1, t.2); }
}

// ---------------------------------------------------------------------------------------------------------------
/// Block 9's own cases: the pull's carry (G1), the set-back with and without G6's guard, headroom and the step from
/// below (G5), and the walk-off catch phases (G3). Made maps where the test scene has no case.
fn block9(world: &[Block], body: Body, v0: f32) {
    let speeds = [6.3_f32, 9.576, 13.27];
    // H1. The pull's carry at the 0.55 step, and a 0.2 m deep step against a 3 m wall (the pull has no wall test).
    println!("# H1. the pull (G1)");
    for sp in speeds { for d in [1.2_f32, 1.13, 1.07] {
        let mut s = start(30.0 - R - d, -9.0, 0.0);
        let mut ticks = 0;
        for _ in 0..60 { tick(world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.pullv.is_some() { ticks += 1; } if s.x >= 31.0 { break; } }
        println!("pull at the 0.55 step speed {sp} start {:.2}: carried {:.4} m beyond the walk over {ticks} ticks after the step tick, feet {:.2}", 30.0 - R - d, s.pull_carry, s.y);
    }}
    let pull_map = vec![bx([65.0, 0.0, -1.0], [65.2, 0.55, 1.0]), bx([65.2, 0.0, -1.0], [66.0, 3.0, 1.0])];
    for sp in speeds { for d in [1.2_f32, 1.13, 1.07] {
        let mut s = start(65.0 - R - d, 0.0, 0.0);
        let (mut emb, mut deepest) = (0, 0.0_f32);
        for _ in 0..40 {
            tick(&pull_map, &mut s, body, (1.0, 0.0), sp, false, v0);
            if embedded(&pull_map, &s, body) { emb += 1; deepest = deepest.max(s.x - (65.2 - R)); }
            if std::env::var("TRACE9").is_ok() && sp == 6.3 && d == 1.2 { println!("    t x {:.4} y {:.3} grounded {} pull {:?} inside {}", s.x, s.y, s.grounded, s.pullv, embedded(&pull_map, &s, body)); }
        }
        println!("pull onto a 0.2 m deep step before a 3 m wall speed {sp} start {:.2}: ended x {:.4} feet {:.2}, ticks inside the wall {emb} (deepest {deepest:.4} m)", 65.0 - R - d, s.x, s.y);
    }}
    // H2. The set-back: a freestanding ramp (x 60 to 69, z 5 to 9, 0 to 2.5 m rising +x: the sandbox's yard ramp's
    // size) met from beyond its high end; a fast fall onto it; and a 300 % ramp walked up at sprint.
    println!("# H2. the set-back (guard {})", SETBACK_GUARD.load(std::sync::atomic::Ordering::Relaxed));
    let sb = vec![rp([60.0, 0.0, 5.0], [69.0, 2.5, 9.0], 0, true), rp([60.0, 0.0, 12.0], [61.0, 3.0, 14.0], 0, true)];
    for sp in speeds {
        let mut s = start(70.2, 7.0, 0.0);
        let mut emb = 0;
        for _ in 0..20 { tick(&sb, &mut s, body, (-1.0, 0.0), sp, false, v0); emb += u32::from(embedded(&sb, &s, body)); }
        println!("beyond the high end, walking -x at {sp}: ended x {:.4} feet {:.4}, largest set-back in one tick {:.4} m, set-backs {}, ticks inside {emb}", s.x, s.y, s.max_setback, s.setbacks);
    }
    let mut miss = 0; let mut worst = 0.0_f32;
    for k in 0..21 {
        let mut s = start(64.5, 7.0, 8.0 + k as f32 * 0.4);
        s.grounded = false;
        for _ in 0..80 { tick(&sb, &mut s, body, (0.0, 0.0), 0.0, false, v0); if s.grounded { break; } }
        let want = height_at(&sb[0], s.x, s.z);
        if !(s.grounded && (s.y - want).abs() < 1e-3) { miss += 1; }
        worst = worst.max(s.max_setback);
    }
    println!("fall from 8 to 16 m onto the slope at x 64.5: {miss} of 21 not standing on the slope; largest set-back {worst:.4} m");
    for sp in speeds {
        let mut s = start(58.0, 13.0, 0.0);
        let mut top = 0.0_f32;
        for _ in 0..40 { tick(&sb, &mut s, body, (1.0, 0.0), sp, false, v0); if s.grounded { top = top.max(s.y); } if s.x >= 60.95 { break; } }
        println!("300 % ramp walked up at {sp}: highest standing {top:.4} m (top 3.0), largest rise in one tick {:.4} m, set-backs {}", s.max_rise, s.setbacks);
    }
    // H3. G5: a 0.55 step with a beam (bottom 2.2 m) over its near edge; a floating plank (0.3 to 0.5 m) met from
    // the ground; and the block 6 layout with the beam 1 m clear of the step's edge.
    println!("# H3. headroom and the step from below (G5)");
    let hm = vec![bx([70.0, 0.0, -5.0], [72.0, 0.55, -3.0]), bx([69.8, 2.2, -5.0], [72.0, 2.5, -3.0]), bx([74.0, 0.3, -5.0], [76.0, 0.5, -3.0]),
                  bx([65.0, 0.0, -1.0], [67.0, 0.55, 1.0]), bx([66.0, 2.2, -1.0], [68.0, 2.5, 1.0])];
    for sp in speeds {
        for (what, x0, z0, end) in [("step under a beam over its edge", 68.0_f32, -4.0_f32, 71.0_f32), ("floating plank 0.3 to 0.5 m", 72.5, -4.0, 75.0), ("step with the beam 1 m in", 63.0, 0.0, 69.0)] {
            let mut s = start(x0, z0, 0.0);
            let mut emb = 0;
            for _ in 0..40 { tick(&hm, &mut s, body, (1.0, 0.0), sp, false, v0); emb += u32::from(embedded(&hm, &s, body)); if s.x >= end { break; } }
            println!("{what} at {sp}: ended x {:.4} feet {:.4}, steps {}, ticks inside {emb}, unjumped falls {}, flickers {}, pushed out {}", s.x, s.y, s.steps, s.falls_unjumped, s.flickers, s.wall_hits);
        }
    }
    // H4. G3: walking off the 0.55 step toward the 0.56 step 2 m away, and off the 3.05 ledge toward the 3.10 ledge,
    // over 30 phases of one tick's move.
    println!("# H4. walk-off catches (G3), 30 phases each");
    for (name, x_top, z, y, top2) in [("0.55 step -> 0.56 step", 31.0_f32, -9.0_f32, 0.55_f32, 0.56_f32), ("3.05 ledge -> 3.10 ledge", 36.5, -2.5, 3.05, 3.1), ("2.9 ledge -> 3.05 ledge", 31.5, -2.5, 2.9, 3.05)] {
        for sp in speeds {
            let mut caught = 0;
            for k in 0..30 {
                let mut s = start(x_top + sp * DT * k as f32 / 30.0, z, y);
                for _ in 0..60 { tick(world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.grounded && s.airborne_ticks > 0 { break; } }
                if s.y == top2 { caught += 1; }
            }
            println!("{name} at {sp}: caught in {caught} of 30 phases");
        }
    }
}

fn main() {
    let body = match std::env::args().nth(1).as_deref() { Some("3a") => Body::A3, Some("3b") => Body::B3, Some("3x") => Body::X3, _ => Body::Cyl };
    if std::env::args().nth(1).as_deref() == Some("2w") { WALL_DISC.store(true, std::sync::atomic::Ordering::Relaxed); }
    for (k, f) in [("OLD_HEAD", &OLD_HEAD), ("OLD_SETBACK", &OLD_SETBACK), ("OLD_LANDING", &OLD_LANDING), ("WALK_SWEEP", &WALK_SWEEP), ("PULL_CLIP", &PULL_CLIP), ("TRACE_BITS", &TRACE_BITS)] {
        if std::env::var(k).is_ok() { f.store(true, std::sync::atomic::Ordering::Relaxed); }
    }
    if std::env::var("HEADROOM_ON_OWN_TOP").is_ok() { OWN_TOP.store(true, std::sync::atomic::Ordering::Relaxed); }
    if std::env::var("HEAD_WIDTH").is_ok() { HEAD_WIDTH.store(true, std::sync::atomic::Ordering::Relaxed); }
    if std::env::var("SETBACK_GUARD").is_ok() { SETBACK_GUARD.store(true, std::sync::atomic::Ordering::Relaxed); }
    if std::env::var("FIX_LAND").is_ok() { LAND_PREV_ALL.store(true, std::sync::atomic::Ordering::Relaxed); }
    if std::env::var("FIX_TOWARD").is_ok() { TOWARD_START.store(true, std::sync::atomic::Ordering::Relaxed); }
    let world = scene();
    let v0 = (2.0 * G * (REACH - STEP)).sqrt();
    match std::env::args().nth(2).as_deref() { Some("b5") => block5(&world, body, v0), Some("b9") => block9(&world, body, v0), _ => block6(&world, body, v0) }
}
