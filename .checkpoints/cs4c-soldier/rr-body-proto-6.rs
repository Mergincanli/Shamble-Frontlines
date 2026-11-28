// rust-reviewer's scratch prototype, round 7 (4c, block 6 of rust-reviewer-read.md, 2026-11-28). Block 5's model
// (rr-body-proto.rs, the sandbox's body as game-designer-read.md block 1 reads it, in 2.5D) carried over unchanged
// when every new option is off ("b5" reproduces block 5's output byte for byte), plus: easing (k a tick toward the
// wish, up = stop, air 100 %), the soft push as game-designer-read.md block 3 reads it (two wall rules), a head stop
// (a candidate ceiling rule, not the sandbox's), a landing that catches a top crossed inside the tick (a candidate
// fix), and counters for flickers, bodies inside a solid and bounces. f32 throughout. Not product code.
// Build: rustc -O; run: <bin> <body: 3a|3b|2|3x> <group: b5|b6>.

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
}

fn eps(body: Body) -> f32 { if body == Body::Cyl { 0.0 } else { 0.005 } }

/// Block 5's wall rule: a block higher than the test feet plus a step pushes the body circle out to R. With easing the
/// speed into the wall is removed. Returns the number of push-outs.
fn walls(world: &[Block], s: &mut S, test_feet: f32, e: f32) -> u32 {
    let mut n = 0;
    for b in world {
        if floating(b, test_feet, s.cfg) { continue; }
        let h = if WALL_DISC.load(std::sync::atomic::Ordering::Relaxed) { height_max(b, s.x, s.z, R) } else { height_near(b, s.x, s.z) };
        if h <= test_feet + STEP + e || !overlaps(b, s.x, s.z, R) { continue; }
        n += 1;
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
        if s.cfg.ease { let vn = s.vx * ox + s.vz * oz; if vn < 0.0 { s.vx -= vn * ox; s.vz -= vn * oz; } }
    }
    n
}

fn tick(world: &[Block], s: &mut S, body: Body, wish: (f32, f32), speed: f32, jump: bool, v0: f32) {
    let e = eps(body);
    let (x_start, z_start, y_start) = (s.x, s.z, s.y);
    if jump && s.grounded { s.vy = v0; s.grounded = false; }
    let mut test_feet = s.y;
    if !s.grounded {
        let (y0, v) = (s.y, s.vy);
        s.y = s.y + v * DT - 0.5 * G * DT * DT;
        s.vy = v - G * DT;
        test_feet = if v > 0.0 && s.vy < 0.0 { y0 + v * v / (2.0 * G) } else if s.y > y0 { s.y } else { y0 };
        // Candidate C2 (not the sandbox's): the head stops at the lowest bottom it would cross this tick.
        if s.cfg.head_stop && test_feet > y0 {
            let mut lowest = f32::INFINITY;
            for b in world {
                if overlaps(b, s.x, s.z, R) && b.min[1] >= y0 + BODY_H - 1e-4 && b.min[1] < test_feet + BODY_H && b.min[1] < lowest { lowest = b.min[1]; }
            }
            if lowest.is_finite() { s.y = lowest - BODY_H; if s.vy > 0.0 { s.vy = 0.0; } test_feet = s.y.max(y0); s.head_hits += 1; }
        }
        if y0 - s.y > s.max_drop { s.max_drop = y0 - s.y; }
        s.y_moved = s.y;
    }
    // The move: block 5's (wish x speed) or, with easing, the eased velocity.
    let (mv, dir);
    if s.cfg.ease {
        s.vx += (wish.0 * speed - s.vx) * EASE_K;
        s.vz += (wish.1 * speed - s.vz) * EASE_K;
        if wish == (0.0, 0.0) && s.vx * s.vx + s.vz * s.vz < 1e-8 { s.vx = 0.0; s.vz = 0.0; }
        let (dx, dz) = (s.vx * DT, s.vz * DT);
        mv = (dx * dx + dz * dz).sqrt();
        dir = (s.vx, s.vz);
        s.x += dx;
        s.z += dz;
    } else {
        mv = speed * DT;
        dir = wish;
        s.x += wish.0 * mv;
        s.z += wish.1 * mv;
    }
    if let Some(i) = s.pull {
        let (nx, nz) = nearest(&world[i], s.x, s.z);
        let (dx, dz) = (nx - s.x, nz - s.z);
        let d = (dx * dx + dz * dz).sqrt();
        let left = d - PULL_TO * FOOT;
        if left <= 0.0 || dx * dir.0 + dz * dir.1 < 0.0 { s.pull = None; } else {
            let step = if left < PULL_MIN { left } else { left * PULL_SHARE };
            s.x += dx / d * step; s.z += dz / d * step;
            if step == left { s.pull = None; }
        }
    }
    let mut best: Option<(usize, f32)> = None;
    for (i, b) in world.iter().enumerate() {
        if floating(b, s.y, s.cfg) || !overlaps(b, s.x, s.z, R) { continue; }
        let h = if body == Body::Cyl { height_max(b, s.x, s.z, R) } else { height_near(b, s.x, s.z) };
        let (nx, nz) = nearest(b, s.x, s.z);
        let toward = (nx - s.x) * dir.0 + (nz - s.z) * dir.1 > 0.0;
        if h > s.y + 1e-4 && h <= test_feet + STEP + e && toward && best.is_none_or(|(_, bh)| h > bh) { best = Some((i, h)); }
    }
    if let Some((i, h)) = best {
        s.y = h; s.vy = 0.0; s.grounded = true; s.steps += 1;
        if body == Body::A3 { s.pull = Some(i); }
        if body == Body::B3 { s.stepped = Some(i); }
        test_feet = h;
    }
    s.wall_hits += walls(world, s, test_feet, e);
    if let Some(i) = s.stepped {
        if overlaps(&world[i], s.x, s.z, FOOT) || !overlaps(&world[i], s.x, s.z, R) { s.stepped = None; }
    }
    let rad = match body { Body::Cyl => R, Body::A3 => if s.pull.is_some() { R } else { FOOT }, Body::B3 | Body::X3 => FOOT };
    // Candidate fix (not the sandbox's, by its reading): while falling, a top crossed inside the tick also counts.
    let lim = if s.cfg.land_prev && !s.grounded { (s.y + STEP + e).max(y_start + 1e-4) } else { s.y + STEP + e };
    let mut surf = 0.0_f32;
    for (i, b) in world.iter().enumerate() {
        let r_here = if body == Body::B3 && s.stepped == Some(i) { R } else { rad };
        if !overlaps(b, s.x, s.z, r_here) { continue; }
        let h = if body == Body::Cyl { height_max(b, s.x, s.z, r_here) } else { height_near(b, s.x, s.z) };
        if h <= lim && h > surf { surf = h; }
    }
    if !s.grounded {
        if s.vy <= 0.0 && s.y <= surf { if surf > 0.0 && surf - s.y_moved > s.below_top { s.below_top = surf - s.y_moved; } s.y = surf; s.vy = 0.0; s.grounded = true; }
        s.airborne_ticks += u32::from(!s.grounded);
    } else {
        let allow = if body == Body::Cyl { STEP } else { SNAP + (mv + FOOT) * slope_under(world, s.x, s.z) };
        if surf >= s.y || s.y - surf <= allow { s.y = surf; } else {
            s.grounded = false; s.vy = 0.0; s.falls_unjumped += 1;
            if s.fall_at.is_none() { s.fall_at = Some((s.x, s.z)); }
            s.fall_from = Some(s.y);
        }
    }
    if s.grounded {
        if let Some(f) = s.fall_from.take() { if (s.y - f).abs() < 1e-3 { s.flickers += 1; } }
    }
    for b in world {
        if b.ramp.is_some() && s.x >= b.min[0] && s.x <= b.max[0] && s.z >= b.min[2] && s.z <= b.max[2] && s.grounded {
            let f = s.y - height_at(b, s.x, s.z);
            if f > s.max_float { s.max_float = f; }
            if let Some((a, _)) = b.ramp { let p = if a == 0 { s.x } else { s.z }; if p > b.min[a] + R && p < b.max[a] - R && f > s.open_float { s.open_float = f; } }
        }
    }
    s.lvx = (s.x - x_start) / DT;
    s.lvz = (s.z - z_start) / DT;
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
    S { x, z, y, vy: 0.0, grounded: true, pull: None, stepped: None, steps: 0, falls_unjumped: 0, airborne_ticks: 0, max_float: 0.0, open_float: 0.0, fall_at: None,
        cfg: Cfg::default(), vx: 0.0, vz: 0.0, lvx: 0.0, lvz: 0.0, fall_from: None, flickers: 0, head_hits: 0, max_drop: 0.0, wall_hits: 0, y_moved: 0.0, below_top: 0.0 }
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
        for _ in 0..40 { tick(world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.x > 34.0 { break; } }
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
                    top = top.max(s.y); maxdz = maxdz.max(s.z.abs());
                    if s.grounded && t > 0 { break; }
                }
                println!("ceiling {cname} {what} at {sp}: highest feet {top:.4} (head {:.4}; beam bottom 2.0) ended x {:.4} z {:.4} feet {:.4} sideways at most {maxdz:.4} head-hits {} embedded-ticks {emb}", top + BODY_H, s.x, s.z, s.y, s.head_hits);
            }
            if !head { break; }
        }
    }

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
        P { name: "into a wall, glancing", starts: vec![(37.0, -2.2, 0.0), (39.6, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "across the 0.55 step, walker below", starts: vec![(28.0, -9.0, 0.0), (30.3, -9.0, 0.55)], wishes: vec![(1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "across the 0.55 step, walker above", starts: vec![(31.5, -9.0, 0.55), (29.6, -9.0, 0.0)], wishes: vec![(-1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "at the 1.0 m gap (platform edge x 49)", starts: vec![(51.0, 6.0, 0.0), (48.8, 6.0, 1.0)], wishes: vec![(-1.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 40 },
        P { name: "three meet", starts: vec![(60.0, 0.0, 0.0), (64.0, 0.0, 0.0), (62.0, 2.0, 0.0)], wishes: vec![(1.0, 0.0), (-1.0, 0.0), (0.0, -1.0)], speed: 6.3, ticks: 40 },
        P { name: "three standing close", starts: vec![(60.0, 0.0, 0.0), (60.3, 0.0, 0.0), (60.15, 0.25, 0.0)], wishes: vec![(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
        P { name: "three into a wall", starts: vec![(37.0, -2.5, 0.0), (39.6, -2.5, 0.0), (38.8, -2.5, 0.0)], wishes: vec![(1.0, 0.0), (0.0, 0.0), (0.0, 0.0)], speed: 6.3, ticks: 60 },
    ];
    let perms: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
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
            println!("push {} [{}]: {} | slot orders {} bit-identical: {same}", c.name, if rule == WallRule::PushOut { "push-out" } else { "sweep" }, first.unwrap(), finals.len());
        }
    }
    println!("# tally (case group: runs, passes)");
    for t in &tally { println!("tally {}: {} runs, {} pass", t.0, t.1, t.2); }
}

fn main() {
    let body = match std::env::args().nth(1).as_deref() { Some("3a") => Body::A3, Some("3b") => Body::B3, Some("3x") => Body::X3, _ => Body::Cyl };
    if std::env::args().nth(1).as_deref() == Some("2w") { WALL_DISC.store(true, std::sync::atomic::Ordering::Relaxed); }
    let world = scene();
    let v0 = (2.0 * G * (REACH - STEP)).sqrt();
    match std::env::args().nth(2).as_deref() { Some("b5") => block5(&world, body, v0), _ => block6(&world, body, v0) }
}
