// rust-reviewer 4b-2: copy of collision.rs enter_box/enter_ramp (c442a80, lines 107-191) with Toward/Solid stubs.
#![allow(dead_code)]
#[derive(Clone, Copy)] enum Toward { PlusX, MinusX, PlusZ, MinusZ }
#[derive(Clone, Copy)] enum Form { Box }
struct Solid { min: [f32; 3], max: [f32; 3], form: Form }
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


fn main() {
    // The yard's ramp_px: [10,0,0]..[14,1,2], rising toward +x.
    let s = Solid { min: [10.0, 0.0, 0.0], max: [14.0, 1.0, 2.0], form: Form::Box };
    // Control: a ray passing over the low end (y = 0.8 at x = 10..11, above the slope there: slope y = (x-10)/4).
    println!("control: level ray at y 0.9 over x 10..13.6 -> {:?}", enter_ramp(&s, Toward::PlusX, [5.0, 0.9, 1.0], [1.0, 0.0, 0.0], 100.0));
    // A huge but finite direction: rate = dy*4 - dx*1 with dy, dx near f32::MAX -> inf - inf = NaN.
    let big = 3.0e38_f32;
    let d = [big, big * 0.5, 0.0];
    // The ray from (9.9, 0.95, 1) goes up and right: it is above the slope everywhere inside the box.
    println!("huge direction, over the slope: box {:?} ramp {:?}", enter_box(s.min, s.max, [9.9, 0.95, 1.0], d, 1.0), enter_ramp(&s, Toward::PlusX, [9.9, 0.95, 1.0], d, 1.0));
    // A ramp 4 long and 2 high, [10,0,0]..[14,2,2]; origin inside its box, above the slope (slope y at x 10.5 is
    // 0.25), going up steeper than the slope: it never goes under it. Unit-size direction, then the same direction
    // scaled near f32::MAX (finite), where rate = dy*4 - dx*2 = inf - inf = NaN.
    let t = Solid { min: [10.0, 0.0, 0.0], max: [14.0, 2.0, 2.0], form: Form::Box };
    println!("tall ramp, unit direction (2,3,0): {:?}", enter_ramp(&t, Toward::PlusX, [10.5, 1.0, 1.0], [2.0, 3.0, 0.0], 100.0));
    println!("tall ramp, direction (2e38,3e38,0): {:?}", enter_ramp(&t, Toward::PlusX, [10.5, 1.0, 1.0], [2.0e38, 3.0e38, 0.0], 100.0));
    let d2 = [1.0, 0.5, 0.0];
    println!("same ray, unit-ish direction: ramp {:?}", enter_ramp(&s, Toward::PlusX, [9.9, 0.95, 1.0], d2, 100.0));
    // A tiny direction with an infinite limit: a hit at t = inf.
    println!("tiny direction, infinite limit: box {:?}", enter_box([0.0;3], [1.0;3], [-5.0, 0.5, 0.5], [1e-45, 0.0, 0.0], f32::INFINITY));
}
