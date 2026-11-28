// rust-reviewer's scratch prototype, round 6 (4c design read, 2026-11-28): the sandbox's own body as game-designer
// read it (game-designer-read.md block 1, his Testlines code at be608cad), in the sandbox's 2.5D form: a body circle
// for walls and the step trigger, a foot circle for standing, heights from footprints, the exact arc with the tick's
// true top. Bodies: "3a" (0.15 m foot and the pull), "3b" (0.15 m foot, the full body on the step just taken), "2"
// (the flat cylinder of round 4 in the same 2.5D form: stands on the highest point under its 0.4 m disc, snap =
// step-up, no allowance). f32 throughout. Not product code. Build: rustc -O; run with the body as argument.

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

#[derive(Clone, Copy, PartialEq)]
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

fn clampf(v: f32, lo: f32, hi: f32) -> f32 { if v < lo { lo } else if v > hi { hi } else { v } }
fn nearest(b: &Block, x: f32, z: f32) -> (f32, f32) { (clampf(x, b.min[0], b.max[0]), clampf(z, b.min[2], b.max[2])) }
fn dist2(b: &Block, x: f32, z: f32) -> f32 { let (nx, nz) = nearest(b, x, z); (x - nx) * (x - nx) + (z - nz) * (z - nz) }
fn overlaps(b: &Block, x: f32, z: f32, r: f32) -> bool { dist2(b, x, z) < r * r }
/// Height of a block's top at a point of its footprint (clamped in).
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
/// The sandbox's heightNear (yard.ts:170-175): the height at the footprint's point nearest the centre.
fn height_near(b: &Block, x: f32, z: f32) -> f32 { let (nx, nz) = nearest(b, x, z); height_at(b, nx, nz) }
/// Body 2's flat disc: the highest point of the block under a disc of radius r (uphill side for a ramp).
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
fn floating(b: &Block, feet: f32) -> bool { b.min[1] > feet + BODY_H }

struct S {
    x: f32, z: f32, y: f32, vy: f32, grounded: bool,
    pull: Option<usize>, stepped: Option<usize>,
    steps: u32, falls_unjumped: u32, airborne_ticks: u32, max_float: f32, open_float: f32, fall_at: Option<(f32, f32)>,
}

fn eps(body: Body) -> f32 { if body == Body::Cyl { 0.0 } else { 0.005 } }

/// One tick, the sandbox's order as game-designer read it: jump, the arc, the move, the step-up (body circle), the
/// walls, standing and landing (foot circle), the snap or the fall, the pull.
fn tick(world: &[Block], s: &mut S, body: Body, wish: (f32, f32), speed: f32, jump: bool, v0: f32) {
    let e = eps(body);
    if jump && s.grounded { s.vy = v0; s.grounded = false; }
    let mut test_feet = s.y;
    if !s.grounded {
        let (y0, v) = (s.y, s.vy);
        s.y = s.y + v * DT - 0.5 * G * DT * DT;
        s.vy = v - G * DT;
        test_feet = if v > 0.0 && s.vy < 0.0 { y0 + v * v / (2.0 * G) } else if s.y > y0 { s.y } else { y0 };
    }
    let mv = speed * DT;
    s.x += wish.0 * mv;
    s.z += wish.1 * mv;
    // The pull (3a): toward the stepped block's nearest point until 0.9 x foot from it; walking back cancels it.
    if let Some(i) = s.pull {
        let (nx, nz) = nearest(&world[i], s.x, s.z);
        let (dx, dz) = (nx - s.x, nz - s.z);
        let d = (dx * dx + dz * dz).sqrt();
        let left = d - PULL_TO * FOOT;
        if left <= 0.0 || dx * wish.0 + dz * wish.1 < 0.0 { s.pull = None; } else {
            let step = if left < PULL_MIN { left } else { left * PULL_SHARE };
            s.x += dx / d * step; s.z += dz / d * step;
            if step == left { s.pull = None; }
        }
    }
    // The step-up: the body circle overlaps a block whose top is above the feet and within a step of the test feet.
    let mut best: Option<(usize, f32)> = None;
    for (i, b) in world.iter().enumerate() {
        if floating(b, s.y) || !overlaps(b, s.x, s.z, R) { continue; }
        let h = if body == Body::Cyl { height_max(b, s.x, s.z, R) } else { height_near(b, s.x, s.z) };
        let (nx, nz) = nearest(b, s.x, s.z);
        let toward = (nx - s.x) * wish.0 + (nz - s.z) * wish.1 > 0.0;
        if h > s.y + 1e-4 && h <= test_feet + STEP + e && toward && best.is_none_or(|(_, bh)| h > bh) { best = Some((i, h)); }
    }
    if let Some((i, h)) = best {
        s.y = h; s.vy = 0.0; s.grounded = true; s.steps += 1;
        if body == Body::A3 { s.pull = Some(i); }
        if body == Body::B3 { s.stepped = Some(i); }
        test_feet = h;
    }
    // Walls: a block higher than the test feet plus a step pushes the body circle out to R; no speed is kept (the
    // command sets it again each tick here).
    for b in world {
        if floating(b, test_feet) { continue; }
        let h = height_near(b, s.x, s.z);
        if h <= test_feet + STEP + e || !overlaps(b, s.x, s.z, R) { continue; }
        let (nx, nz) = nearest(b, s.x, s.z);
        let (dx, dz) = (s.x - nx, s.z - nz);
        let d = (dx * dx + dz * dz).sqrt();
        if d > 0.0 { s.x = nx + dx / d * R; s.z = nz + dz / d * R; } else {
            // Centre inside the footprint: out along the shallowest side.
            let pens = [(s.x - b.min[0] + R, -1.0, 0.0), (b.max[0] - s.x + R, 1.0, 0.0), (s.z - b.min[2] + R, 0.0, -1.0), (b.max[2] - s.z + R, 0.0, 1.0)];
            let mut m = pens[0];
            for p in pens { if p.0 < m.0 { m = p; } }
            s.x += m.1 * m.0; s.z += m.2 * m.0;
        }
    }
    // (3b): the stepped block is stood on with the full body until the foot overlaps it, or the body leaves it.
    if let Some(i) = s.stepped {
        if overlaps(&world[i], s.x, s.z, FOOT) || !overlaps(&world[i], s.x, s.z, R) { s.stepped = None; }
    }
    // Standing: the highest top under the standing circle within a step of the feet, else the ground at 0.
    let rad = match body { Body::Cyl => R, Body::A3 => if s.pull.is_some() { R } else { FOOT }, Body::B3 | Body::X3 => FOOT };
    let mut surf = 0.0_f32;
    for (i, b) in world.iter().enumerate() {
        let r_here = if body == Body::B3 && s.stepped == Some(i) { R } else { rad };
        if !overlaps(b, s.x, s.z, r_here) { continue; }
        let h = if body == Body::Cyl { height_max(b, s.x, s.z, r_here) } else { height_near(b, s.x, s.z) };
        if h <= s.y + STEP + e && h > surf { surf = h; }
    }
    if !s.grounded {
        if s.vy <= 0.0 && s.y <= surf { s.y = surf; s.vy = 0.0; s.grounded = true; }
        s.airborne_ticks += u32::from(!s.grounded);
    } else {
        let allow = if body == Body::Cyl { STEP } else { SNAP + (mv + FOOT) * slope_under(world, s.x, s.z) };
        if surf >= s.y || s.y - surf <= allow { s.y = surf; } else {
            s.grounded = false; s.vy = 0.0; s.falls_unjumped += 1;
            if s.fall_at.is_none() { s.fall_at = Some((s.x, s.z)); }
        }
    }
    // The float, measured once the tick's height is final: standing height above the slope under the centre.
    for b in world {
        if b.ramp.is_some() && s.x >= b.min[0] && s.x <= b.max[0] && s.z >= b.min[2] && s.z <= b.max[2] && s.grounded {
            let f = s.y - height_at(b, s.x, s.z);
            if f > s.max_float { s.max_float = f; }
            // On the open slope: the centre more than a body radius from the ramp's two ends.
            if let Some((a, _)) = b.ramp { let p = if a == 0 { s.x } else { s.z }; if p > b.min[a] + R && p < b.max[a] - R && f > s.open_float { s.open_float = f; } }
        }
    }
}

fn start(x: f32, z: f32, y: f32) -> S {
    S { x, z, y, vy: 0.0, grounded: true, pull: None, stepped: None, steps: 0, falls_unjumped: 0, airborne_ticks: 0, max_float: 0.0, open_float: 0.0, fall_at: None }
}

fn main() {
    let body = match std::env::args().nth(1).as_deref() { Some("3a") => Body::A3, Some("3b") => Body::B3, Some("3x") => Body::X3, _ => Body::Cyl };
    let world = scene();
    let v0 = (2.0 * G * (REACH - STEP)).sqrt();
    let speeds = [6.3_f32, 9.576, 13.27];
    // 1. The 0.55 and 0.56 steps (x 30 and 34 faces, z -9), walking +x.
    for (face, top) in [(30.0_f32, 0.55_f32), (34.0, 0.56)] {
        // Starts inside the 2 m gap before each step (the 0.55 step's top ends at x 32): the body 1.2, 1.13, 1.07 m short.
        for sp in speeds { for d in [1.2_f32, 1.13, 1.07] {
            let mut s = start(face - R - d, -9.0, 0.0);
            let mut on_top_at = None;
            for t in 0..60 {
                tick(&world, &mut s, body, (1.0, 0.0), sp, false, v0);
                if std::env::var("TRACE").is_ok() && sp == 6.3 && top == 0.55 { println!("  t {t} x {:.4} y {:.4} grounded {} pull {:?} stepped {:?}", s.x, s.y, s.grounded, s.pull, s.stepped); }
                if on_top_at.is_none() && s.x >= face + 1.0 { on_top_at = Some((t, s.y, s.grounded)); break; }
            }
            let pass = s.falls_unjumped == 0 && if top < 0.555 { matches!(on_top_at, Some((_, y, true)) if y == top) } else { on_top_at.is_none() && s.y == 0.0 && s.x <= face - R + 1e-4 };
            println!("step {top} speed {sp} start {:.2}: {} x {:.4} feet {:.4} steps {} unjumped-falls {} airborne-ticks {}",
                face - R - d, if pass { "PASS" } else { "FAIL" }, s.x, s.y, s.steps, s.falls_unjumped, s.airborne_ticks);
        }}
    }
    // 1b. Where the bridge matters: a slow walk (1.0 and 2.0 m/s: the easing's first ticks from rest), and a walk that
    // stops the tick the step-up fires (then 20 ticks with no key). Pass: on the 0.55 top, grounded, no fall.
    for sp in [1.0_f32, 2.0, 6.3] { for d in [1.2_f32, 1.13, 1.07] { for stop in [false, true] {
        if !stop && sp == 6.3 { continue; }
        let mut s = start(30.0 - R - d, -9.0, 0.0);
        let mut stopped_at = None;
        for t in 0..400 {
            let wish = if stopped_at.is_some() { (0.0, 0.0) } else { (1.0, 0.0) };
            tick(&world, &mut s, body, wish, sp, false, v0);
            if stop && stopped_at.is_none() && s.steps > 0 { stopped_at = Some(t); }
            if let Some(t0) = stopped_at { if t >= t0 + 20 { break; } }
            if !stop && s.x >= 31.0 { break; }
        }
        let pass = s.y == 0.55 && s.grounded && s.falls_unjumped == 0;
        println!("slow step 0.55 speed {sp} start {:.2} {}: {} x {:.4} feet {:.4} centre past the face {:.4} steps {} unjumped-falls {}",
            30.0 - R - d, if stop { "stop at touch" } else { "walk on" }, if pass { "PASS" } else { "FAIL" }, s.x, s.y, s.x - 30.0, s.steps, s.falls_unjumped);
    }}}
    // 2. The 3.05 and 3.10 ledges (x 35 and 40 faces, z -2.5), a jump at tick 0 from d metres before the body touches.
    for (face, top) in [(35.0_f32, 3.05_f32), (40.0, 3.1)] {
        for sp in speeds { for d in [0.3_f32, 0.8, 1.2] {
            let mut s = start(face - R - d, -2.5, 0.0);
            let mut first_top = None;
            for t in 0..90 {
                tick(&world, &mut s, body, (1.0, 0.0), sp, t == 0, v0);
                if first_top.is_none() && s.grounded && s.y == top { first_top = Some(t); }
                if s.x >= face + 1.2 { break; }
            }
            let caught = s.y == top && s.grounded;
            let pass = s.falls_unjumped == 0 && if top < 3.076 { caught } else { !caught && first_top.is_none() };
            println!("ledge {top} speed {sp} jump {d} m out: {} feet {:.4} x {:.3} on-top-tick {:?} unjumped-falls {} airborne-ticks {}",
                if pass { "PASS" } else { "FAIL" }, s.y, s.x, first_top, s.falls_unjumped, s.airborne_ticks);
        }}
    }
    // 3. The ramps: up from x 38 onto the platform, then down from x 47.
    for (name, z, top) in [("gentle 25%", 6.0_f32, 1.0_f32), ("steep 50%", 12.0, 1.5)] {
        for sp in speeds { for x0 in [38.0_f32, 38.07, 38.13] {
            let mut s = start(x0, z, 0.0);
            for _ in 0..90 { tick(&world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.x >= 47.0 { break; } }
            let up_ok = s.y == top && s.grounded;
            let (up_float, up_open, up_air, up_falls) = (s.max_float, s.open_float, s.airborne_ticks, s.falls_unjumped);
            let mut d = start(47.0 + (x0 - 38.0), z, top);
            for _ in 0..90 { tick(&world, &mut d, body, (-1.0, 0.0), sp, false, v0); if d.x <= 39.0 { break; } }
            let down_ok = d.y == 0.0 && d.grounded;
            println!("ramp {name} speed {sp} start {x0}: up {} (float open {up_open:.4} / at the seams {:.4} m, airborne ticks {up_air}, falls {up_falls}) down {} (float open {:.4} / seams {:.4} m, airborne ticks {}, falls {})",
                if up_ok { "PASS" } else { "FAIL" }, up_float, if down_ok { "PASS" } else { "FAIL" }, d.open_float, d.max_float, d.airborne_ticks, d.falls_unjumped);
        }}
    }
    // 4. The roofs: a jump onto the house's north edge (2.6 m), then walking +z up the roof toward the ridge.
    for (name, x, face, ridge) in [("house a 95%", -9.0_f32, -8.0_f32, 4.7_f32), ("house b 89%", -2.0, -7.4, 4.3)] {
        for sp in speeds { for d in [0.5_f32, 1.0, 1.5] {
            let mut s = start(x, face - R - d, 0.0);
            let mut highest = 0.0_f32;
            let mut gained_edge = false;
            for t in 0..120 {
                tick(&world, &mut s, body, (0.0, 1.0), sp, t == 0, v0);
                if s.grounded && s.y >= 2.6 { gained_edge = true; }
                if s.grounded && s.y > highest { highest = s.y; }
                if s.z >= face + 2.1 { break; }
            }
            println!("roof {name} speed {sp} jump {d} m out: edge gained {gained_edge}, highest standing {:.3} m (ridge {ridge}), float open {:.4} / seams {:.4}, unjumped-falls {}",
                highest, s.open_float, s.max_float, s.falls_unjumped);
        }}
    }
    // 6. The thin wall (x 30 to 30.2, z 4 to 10) at 13.27 m/s, 10,000 seeded starts: the 2.5D push-out runs after the
    // move, with no sweep, so this counts the centres that end past the wall's mid-plane (x 30.1).
    let (mut seed, mut through) = (0x9e37_79b9_u32, 0);
    for _ in 0..10_000 {
        let mut next = || { seed ^= seed << 13; seed ^= seed >> 17; seed ^= seed << 5; seed };
        let x = 29.0 + (next() % 600) as f32 / 1000.0;
        let z = 4.5 + (next() % 5000) as f32 / 1000.0;
        let side = (next() % 1000) as f32 / 1000.0 - 0.5;
        let len = (1.0 + side * side).sqrt();
        let mut s = start(x, z, 0.0);
        tick(&world, &mut s, body, (1.0 / len, side / len), 13.27, false, v0);
        if s.x > 30.1 { through += 1; }
    }
    println!("thin wall at 13.27 m/s: {through} of 10000 past the mid-plane");
    // 5. Walking off an edge: the gentle platform's far side (x 49, 1.0 m drop) and the 0.55 step's far side (x 32).
    for (name, x0, z, y, edge) in [("platform 1.0 m", 47.0_f32, 6.0_f32, 1.0_f32, 49.0_f32), ("step 0.55 m", 31.0, -9.0, 0.55, 32.0)] {
        for sp in speeds {
            let mut s = start(x0, z, y);
            for _ in 0..30 { tick(&world, &mut s, body, (1.0, 0.0), sp, false, v0); if s.x > edge + 1.5 { break; } }
            let at = s.fall_at.map(|(x, _)| x - edge);
            println!("off {name} speed {sp}: {} (centre past the edge when it started: {:?} m), feet now {:.3}",
                if s.fall_at.is_some() { "fell" } else { "snapped down" }, at, s.y);
        }
    }
}
