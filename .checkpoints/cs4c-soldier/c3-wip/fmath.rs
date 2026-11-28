//! The simulation's float helpers and its two maths routines (ADR 0004 part 7: "transcendental maths only via
//! pinned libm"). This is the one file of the simulation that names `libm`, and only by full paths, each in its own
//! function: the exponential in `share_per_tick`, the sine and cosine (one routine, `sincos`) in `facing_of`. The rule
//! scan holds it to that (`tests/rules_scan.rs`). Both routines are pure Rust, with no fused multiply-add, on every
//! target; they run while a match's constants are worked out, never in a tick, and their results are pinned by the
//! vectors (PM-0004-22).

/// Degrees to radians: pi over 180 as a double, the same bits as `f64::to_radians`'s own constant (pinned by a test).
pub const DEG_TO_RAD: f64 = core::f64::consts::PI / 180.0;

/// The smaller of two floats by one comparison: on equal values, two signed zeros among them, the first. (std's
/// `min` may give either zero, ADR 0004 part 7.)
#[must_use]
pub fn min_f32(a: f32, b: f32) -> f32 {
    if b < a { b } else { a }
}

/// The larger of two floats by one comparison: on equal values, two signed zeros among them, the first.
#[must_use]
pub fn max_f32(a: f32, b: f32) -> f32 {
    if b > a { b } else { a }
}

/// `x` held to `low ..= high` by comparisons: `x` itself when it is inside, a bound otherwise; when `low` is above
/// `high`, `low`. It never panics (std's `clamp` panics on such bounds, ADR 0004 part 7).
#[must_use]
pub fn clamp_f32(x: f32, low: f32, high: f32) -> f32 {
    if low > high || x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}

/// The share of a gap that an exponential approach at `rate_per_s` closes in one step of `dt_s` seconds:
/// 1 - exp(-rate x dt), worked out in f64 and narrowed once. The easing's rate is ln 10 over its time to a tenth of
/// the gap; the push's is its strength as the content writes it.
#[must_use]
pub fn share_per_tick(rate_per_s: f64, dt_s: f64) -> f32 {
    (1.0 - libm::exp(-(rate_per_s * dt_s))) as f32
}

/// The direction a heading in degrees faces, as (x, z): 0 degrees faces +z, 90 faces +x. The whole quarter turns are
/// taken out first, by comparisons, so 0, 90, 180 and 270 come out exact; what is left of the angle, exact too, goes
/// through `sincos` in f64 once and is narrowed once. A turn negates with `0.0 - v`, never `-v`, and the rest is
/// canonicalised with `+ 0.0`: a minus zero would be another bit pattern in the hashed state.
#[must_use]
pub fn facing_of(deg: f64) -> [f32; 2] {
    let (quarter, turned) = if deg < 90.0 {
        (0, 0.0)
    } else if deg < 180.0 {
        (1, 90.0)
    } else if deg < 270.0 {
        (2, 180.0)
    } else {
        (3, 270.0)
    };
    let rest = (deg - turned) + 0.0;
    let (sin, cos) = libm::sincos(rest * DEG_TO_RAD);
    let (sin, cos) = (sin as f32, cos as f32);
    match quarter {
        0 => [sin, cos],
        1 => [cos, 0.0 - sin],
        2 => [0.0 - sin, 0.0 - cos],
        _ => [0.0 - cos, sin],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The helpers: on two signed zeros each gives its first argument, by bits; a clamp whose bounds are the wrong
    /// way round gives the low bound; inside, a clamp gives `x` itself.
    #[test]
    fn the_helpers_give_one_defined_argument_and_never_panic() {
        let bits = |x: f32| x.to_bits();
        assert_eq!(bits(min_f32(0.0, -0.0)), bits(0.0));
        assert_eq!(bits(min_f32(-0.0, 0.0)), bits(-0.0));
        assert_eq!(bits(max_f32(0.0, -0.0)), bits(0.0));
        assert_eq!(bits(max_f32(-0.0, 0.0)), bits(-0.0));
        assert_eq!((min_f32(1.0, 2.0), min_f32(2.0, 1.0)), (1.0, 1.0));
        assert_eq!((max_f32(1.0, 2.0), max_f32(2.0, 1.0)), (2.0, 2.0));
        assert_eq!(clamp_f32(5.0, 1.0, 2.0), 2.0);
        assert_eq!(clamp_f32(-5.0, 1.0, 2.0), 1.0);
        assert_eq!(clamp_f32(1.5, 1.0, 2.0), 1.5);
        assert_eq!(clamp_f32(1.5, 2.0, 1.0), 2.0);
        assert_eq!(bits(clamp_f32(-0.0, 0.0, 1.0)), bits(-0.0));
    }

    /// The constant is pi over 180 to the bit (rust-reviewer B7-2: `f64::to_radians`'s own, measured).
    #[test]
    fn degrees_become_radians_by_the_pinned_constant() {
        assert_eq!(DEG_TO_RAD.to_bits(), 0x3f91_df46_a252_9d39);
    }

    /// The four quarter turns come out exact, by bits, and with no minus zero (B7-3): 90 degrees faces (1, 0).
    #[test]
    fn the_quarter_turns_are_exact_and_carry_no_minus_zero() {
        let bits = |[x, z]: [f32; 2]| [x.to_bits(), z.to_bits()];
        let cases: [(f64, [f32; 2]); 5] = [
            (0.0, [0.0, 1.0]),
            (-0.0, [0.0, 1.0]),
            (90.0, [1.0, 0.0]),
            (180.0, [0.0, -1.0]),
            (270.0, [-1.0, 0.0]),
        ];
        for (deg, facing) in cases {
            assert_eq!(bits(facing_of(deg)), bits(facing), "{deg}");
        }
    }

    /// Between the quarter turns: the correctly rounded f32 of the sine and cosine of the radians the route takes
    /// (`.checkpoints/cs4c-soldier/facing-ref-c3.py`, Python decimal at 60 digits). At 45, 120 and 240 degrees that is
    /// also the true angle's. At the largest double under 360 it is not: there the constant's own rounding (about
    /// 1.7e-16 at 90 degrees) outweighs a sine of 1e-15, and the true angle's x would be a68efa35 (-9.921e-16, against
    /// the route's -9.494e-16: a direction 4.3e-17 apart).
    #[test]
    fn between_the_quarter_turns_each_facing_is_the_route_s_correctly_rounded_one() {
        let below_360 = f64::from_bits(360.0_f64.to_bits() - 1);
        let cases: [(f64, [u32; 2]); 4] = [
            (45.0, [0x3f35_04f3, 0x3f35_04f3]),
            (120.0, [0x3f5d_b3d7, 0xbf00_0000]),
            (240.0, [0xbf5d_b3d7, 0xbf00_0000]),
            (below_360, [0xa688_d313, 0x3f80_0000]),
        ];
        for (deg, bits) in cases {
            let [x, z] = facing_of(deg);
            assert_eq!([x.to_bits(), z.to_bits()], bits, "{deg}");
        }
    }
}
