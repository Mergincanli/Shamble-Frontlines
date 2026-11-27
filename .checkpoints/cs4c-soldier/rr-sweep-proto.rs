// rust-reviewer's scratch prototype (4c design read, 2026-11-28): our own capsule sweep against axis-aligned boxes,
// f32 throughout, + - * / sqrt and comparisons only. Not product code. Build: rustc -O, run, read the lines.
type V = [f32; 3];
const R: f32 = 0.4; // body radius
const H: f32 = 0.5; // half segment: (1.80 / 2) - 0.4
const SKIN: f32 = 0.002;
const STEP: f32 = 0.55;
const WALK_NY: f32 = 0.7;

fn sub(a: V, b: V) -> V { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn add(a: V, b: V) -> V { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn scale(a: V, s: f32) -> V { [a[0] * s, a[1] * s, a[2] * s] }
fn dot(a: V, b: V) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

/// Ray o + t d, t in [0, 1], into an AABB: entry t (0 if inside).
fn slab(min: V, max: V, o: V, d: V) -> Option<f32> {
    let (mut en, mut lv) = (0.0_f32, 1.0_f32);
    for a in 0..3 {
        if d[a] == 0.0 {
            if o[a] < min[a] || o[a] > max[a] { return None; }
        } else {
            let (p, q) = ((min[a] - o[a]) / d[a], (max[a] - o[a]) / d[a]);
            let (n, f) = if p > q { (q, p) } else { (p, q) };
            if n > en { en = n; }
            if f < lv { lv = f; }
            if en > lv { return None; }
        }
    }
    Some(en)
}

/// Ray into a sphere of radius rr at c: smallest t in [0, 1] (0 if inside).
fn sphere(c: V, rr: f32, o: V, d: V) -> Option<f32> {
    let m = sub(o, c);
    let cc = dot(m, m) - rr * rr;
    if cc <= 0.0 { return Some(0.0); }
    let a = dot(d, d);
    let b = dot(m, d);
    if b >= 0.0 || a == 0.0 { return None; }
    let disc = b * b - a * cc;
    if disc < 0.0 { return None; }
    let t = (-b - disc.sqrt()) / a;
    if t <= 1.0 { Some(if t < 0.0 { 0.0 } else { t }) } else { None }
}

/// Ray into the capsule of segment p..q (axis-aligned: axis ax) and radius rr.
fn capsule(p: V, q: V, ax: usize, rr: f32, o: V, d: V) -> Option<f32> {
    let (u, w) = match ax { 0 => (1, 2), 1 => (0, 2), _ => (0, 1) };
    let (mu, mw) = (o[u] - p[u], o[w] - p[w]);
    let (du, dw) = (d[u], d[w]);
    let a = du * du + dw * dw;
    let cc = mu * mu + mw * mw - rr * rr;
    let mut best: Option<f32> = None;
    let mut take = |t: f32| if best.is_none_or(|b| t < b) { best = Some(t) };
    if a > 0.0 {
        let b = mu * du + mw * dw;
        let disc = b * b - a * cc;
        if disc >= 0.0 {
            let t = if cc <= 0.0 { 0.0 } else { (-b - disc.sqrt()) / a };
            if (0.0..=1.0).contains(&t) && (cc <= 0.0 || b < 0.0) {
                let along = o[ax] + d[ax] * t;
                if along >= p[ax] && along <= q[ax] { take(t); }
            }
        }
    } else if cc <= 0.0 {
        let along = o[ax];
        if along >= p[ax] && along <= q[ax] { take(0.0); }
    }
    if let Some(t) = sphere(p, rr, o, d) { take(t); }
    if let Some(t) = sphere(q, rr, o, d) { take(t); }
    best
}

#[derive(Clone, Copy, Debug)]
struct Hit { t: f32, shape: usize, n: V, face: bool }

/// The capsule's centre o moving by d against box (min, max): first t and the normal.
fn sweep_box(min: V, max: V, o: V, d: V, rr: f32) -> Option<(f32, V, bool)> {
    let (lo, hi) = ([min[0], min[1] - H, min[2]], [max[0], max[1] + H, max[2]]);
    let mut best: Option<f32> = None;
    let mut take = |t: Option<f32>| if let Some(t) = t { if best.is_none_or(|b| t < b) { best = Some(t) } };
    for a in 0..3 {
        let (mut l, mut h) = (lo, hi);
        l[a] -= rr; h[a] += rr;
        take(slab(l, h, o, d));
    }
    for a in 0..3 {
        let (u, w) = match a { 0 => (1, 2), 1 => (0, 2), _ => (0, 1) };
        for cu in [lo[u], hi[u]] { for cw in [lo[w], hi[w]] {
            let mut p = [0.0; 3]; p[a] = lo[a]; p[u] = cu; p[w] = cw;
            let mut q = p; q[a] = hi[a];
            take(capsule(p, q, a, rr, o, d));
        }}
    }
    let t = best?;
    let at = add(o, scale(d, t));
    let cl = [at[0].clamp(lo[0], hi[0]), at[1].clamp(lo[1], hi[1]), at[2].clamp(lo[2], hi[2])];
    let g = sub(at, cl);
    let len = dot(g, g).sqrt();
    let n = if len > 0.0 { scale(g, 1.0 / len) } else { [0.0, 1.0, 0.0] };
    // Touching and moving along or away is no hit (a convex shape cannot be entered later on that line).
    if dot(n, d) >= 0.0 { return None; }
    let outside = (0..3).filter(|&a| at[a] < lo[a] || at[a] > hi[a]).count();
    Some((t, n, outside <= 1))
}

/// Variant A (round 4): a vertical cylinder of radius rr and half height hc against a box. Pieces: the box grown by hc
/// in y and by rr in x, the same grown by rr in z, and four vertical cylinders at its vertical edges.
fn sweep_box_cyl(min: V, max: V, o: V, d: V, rr: f32, hc: f32) -> Option<(f32, V, bool)> {
    let (lo, hi) = ([min[0], min[1] - hc, min[2]], [max[0], max[1] + hc, max[2]]);
    let mut best: Option<f32> = None;
    let mut take = |t: Option<f32>| if let Some(t) = t { if best.is_none_or(|b| t < b) { best = Some(t) } };
    for a in [0, 2] { let (mut l, mut h) = (lo, hi); l[a] -= rr; h[a] += rr; take(slab(l, h, o, d)); }
    for cx in [lo[0], hi[0]] { for cz in [lo[2], hi[2]] {
        take(capsule_vertical(cx, cz, lo[1], hi[1], rr, o, d));
    }}
    let t = best?;
    let at = add(o, scale(d, t));
    let n = if at[1] >= hi[1] { [0.0, 1.0, 0.0] } else if at[1] <= lo[1] { [0.0, -1.0, 0.0] } else {
        let g = [at[0] - at[0].clamp(lo[0], hi[0]), 0.0, at[2] - at[2].clamp(lo[2], hi[2])];
        let l = dot(g, g).sqrt(); if l > 0.0 { scale(g, 1.0 / l) } else { [0.0, 1.0, 0.0] } };
    if dot(n, d) >= 0.0 { return None; }
    Some((t, n, true))
}

/// A vertical cylinder at (cx, cz), y from y0 to y1, radius rr: ray entry t in [0, 1].
fn capsule_vertical(cx: f32, cz: f32, y0: f32, y1: f32, rr: f32, o: V, d: V) -> Option<f32> {
    let (mx, mz) = (o[0] - cx, o[2] - cz);
    let a = d[0] * d[0] + d[2] * d[2];
    let cc = mx * mx + mz * mz - rr * rr;
    let t = if cc <= 0.0 { 0.0 } else {
        if a == 0.0 { return None; }
        let b = mx * d[0] + mz * d[2];
        if b >= 0.0 { return None; }
        let disc = b * b - a * cc;
        if disc < 0.0 { return None; }
        (-b - disc.sqrt()) / a
    };
    if t > 1.0 { return None; }
    let y = o[1] + d[1] * t;
    if y >= y0 && y <= y1 { Some(t) } else { None }
}

fn sweep(world: &[(V, V)], o: V, d: V) -> Option<Hit> {
    let mut best: Option<Hit> = None;
    // A horizontal sweep treats a box whose top is no higher than the foot plus the skin as floor (round 2 of this
    // prototype: without it the round foot rides over every edge and a 0.56 m step is gained).
    let foot = o[1] - R - H;
    for (i, (mn, mx)) in world.iter().enumerate() {
        if d[1] == 0.0 && mx[1] <= foot + SKIN { continue; }
        let cyl = std::env::var("CYL").is_ok();
        let got = if cyl { sweep_box_cyl(*mn, *mx, o, d, R + SKIN, H + R + SKIN) } else { sweep_box(*mn, *mx, o, d, R + SKIN) };
        if let Some((t, n, face)) = got {
            if best.is_none_or(|b| t < b.t || (t == b.t && i < b.shape)) { best = Some(Hit { t, shape: i, n, face }); }
        }
    }
    best
}

/// One grounded horizontal move with up to four slides and the step-up; returns the new centre.
fn walk(world: &[(V, V)], c: V, mv: V) -> (V, bool) {
    let plain = |c: V, mv: V| {
        let (mut c, mut rest) = (c, mv);
        let mut stepped_wanted = false;
        for _ in 0..4 {
            if dot(rest, rest) == 0.0 { break; }
            match sweep(world, c, rest) {
                None => { c = add(c, rest); break; }
                Some(hit) => {
                    c = add(c, scale(rest, hit.t));
                    let mut n = hit.n;
                    // An edge or a corner is a wall to a horizontal move (round 3): only a plane can be walked up.
                    if n[1] < WALK_NY || !hit.face { stepped_wanted = true; n[1] = 0.0; let l = dot(n, n).sqrt(); if l > 0.0 { n = scale(n, 1.0 / l); } }
                    let left = scale(rest, 1.0 - hit.t);
                    rest = sub(left, scale(n, dot(left, n)));
                    rest[1] = 0.0;
                }
            }
        }
        (c, stepped_wanted)
    };
    let (slid, blocked) = plain(c, mv);
    if !blocked { return (slid, false); }
    let up = [0.0, STEP + SKIN, 0.0];
    let c1 = match sweep(world, c, up) { None => add(c, up), Some(h) => add(c, scale(up, h.t)) };
    let (c2, _) = plain(c1, mv);
    let down = [0.0, -(c1[1] - c[1]) - SKIN, 0.0];
    match sweep(world, c2, down) {
        Some(h) if h.n[1] >= WALK_NY => {
            let c3 = add(c2, scale(down, h.t));
            let far = |p: V| { let g = sub(p, c); g[0] * g[0] + g[2] * g[2] };
            if far(c3) > far(slid) { (c3, true) } else { (slid, false) }
        }
        _ => (slid, false),
    }
}

fn main() {
    let floor = ([-100.0, -1.0, -100.0], [100.0, 0.0, 100.0]);
    for (top, label) in [(0.55_f32, "0.55"), (0.56, "0.56"), (0.30, "0.30"), (0.05, "0.05")] {
        for speed in [6.3_f32, 9.576, 13.27] {
            for start in [27.0_f32, 27.07, 27.13] {
                let world = [floor, ([30.0, 0.0, -10.0], [32.0, top, -8.0])];
                let mut c = [start, SKIN + R + H, -9.0];
                let mv = [speed / 30.0, 0.0, 0.0];
                let mut stepped = false;
                for tick in 0..30 { let (n, s) = walk(&world, c, mv); stepped |= s; c = n; if std::env::var("TRACE").is_ok() && label == "0.55" && speed == 6.3 && start == 27.0 { println!("  tick {tick} x {:.4} foot {:.4} {s}", c[0], c[1] - R - H); } if c[0] > 31.0 { break; } }
                let foot = c[1] - R - H;
                println!("step {label} speed {speed} start {start}: x {:.4} foot {:.4} stepped {stepped}", c[0], foot);
            }
        }
    }
    // A tie: two boxes whose faces share the plane x = 30, the capsule between them in z.
    let world = [([30.0, 0.0, 0.0], [31.0, 2.0, 1.0]), ([30.0, 0.0, -1.0], [31.0, 2.0, 0.0])];
    let hit = sweep(&world, [28.0, 1.0, 0.0], [3.0, 0.0, 0.0]);
    let swapped = [world[1], world[0]];
    let hit2 = sweep(&swapped, [28.0, 1.0, 0.0], [3.0, 0.0, 0.0]);
    println!("tie: {hit:?} / swapped {hit2:?}");
    // The thin wall at the bayonet's 13.27 m/s, 10,000 moves from seeded starts: the centre never passes x = 30.1.
    let wall = [([30.0, 0.0, 4.0], [30.2, 2.5, 10.0])];
    let (mut s, mut through, mut hits) = (0x9e37_79b9_u32, 0, 0);
    for _ in 0..10_000 {
        s ^= s << 13; s ^= s >> 17; s ^= s << 5;
        let z = 4.5 + (s % 5000) as f32 / 1000.0;
        s ^= s << 13; s ^= s >> 17; s ^= s << 5;
        let x = 29.0 + (s % 600) as f32 / 1000.0;
        s ^= s << 13; s ^= s >> 17; s ^= s << 5;
        let ang = (s % 1000) as f32 / 1000.0 - 0.5;
        let d = [13.27 / 30.0, 0.0, 13.27 / 30.0 * ang];
        let o = [x, 1.0, z];
        let end = match sweep(&wall, o, d) { None => add(o, d), Some(h) => { hits += 1; add(o, scale(d, h.t)) } };
        if end[0] > 30.1 { through += 1; }
    }
    println!("thin wall: {through} of 10000 past the mid-plane, {hits} hits");
}
