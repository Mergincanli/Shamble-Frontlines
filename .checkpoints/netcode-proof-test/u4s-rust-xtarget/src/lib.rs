//! The wire grid (npt_sim::grid) on every integer of every field, every value near each grid point's quarter and half
//! steps, and every 2^shift-th f32 bit pattern (NaN, infinities and subnormals included), folded into one FNV-1a hash.

use npt_sim::grid::{self, Config, Field};

const FIELDS: [Field; 5] = [
    grid::POSITION_XY,
    grid::POSITION_Z,
    grid::YAW,
    grid::PITCH,
    grid::VELOCITY,
];

struct Fnv(u64, u64);

impl Fnv {
    fn u32(&mut self, v: u32) {
        for b in v.to_le_bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
        self.1 += 1;
    }
}

fn scale(f: Field) -> f64 {
    if f == grid::YAW {
        65_536.0 / core::f64::consts::TAU
    } else if f == grid::PITCH {
        511.0 / core::f64::consts::FRAC_PI_2
    } else {
        100.0
    }
}

/// (hash, values hashed)
pub fn run(shift: u32) -> (u64, u64) {
    let g = Config::default();
    let mut h = Fnv(0xcbf2_9ce4_8422_2325, 0);
    for f in FIELDS {
        let sc = scale(f);
        for k in f.min()..=f.max() {
            let x = g.from_grid(f, k);
            h.u32(x.to_bits());
            h.u32(g.to_grid(f, x) as u32);
            for q in -2i32..=2 {
                let at = ((f64::from(k) + f64::from(q) / 4.0) / sc) as f32;
                for d in [-2i32, -1, 0, 1, 2] {
                    let y = f32::from_bits(at.to_bits().wrapping_add(d as u32));
                    h.u32(g.to_grid(f, y) as u32);
                    h.u32(g.snap(f, y).to_bits());
                }
            }
        }
        let step = 1u64 << shift;
        let mut b = 0u64;
        while b < (1u64 << 32) {
            let x = f32::from_bits(b as u32);
            h.u32(g.to_grid(f, x) as u32);
            h.u32(g.snap(f, x).to_bits());
            b += step;
        }
    }
    (h.0, h.1)
}

/// The hash for wasm32 (Node reads an i64 as a BigInt).
#[no_mangle]
pub extern "C" fn xt_run(shift: u32) -> u64 {
    run(shift).0
}

/// The count of values hashed, for wasm32.
#[no_mangle]
pub extern "C" fn xt_count(shift: u32) -> u64 {
    run(shift).1
}
