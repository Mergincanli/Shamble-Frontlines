// rust-reviewer scratch for PM-0004-21's hash report (change-set 4a). Not project code; no crate, no manifest.
// FNV-1a 64 (the same loop as crates/sim/src/state_hash.rs) against XXH64 written from its spec, the strongest
// replacement that needs no crate. Native: cost (per-call mean and p99, and a block mean without the timer), a
// serializer proxy, and a structure test of detection on realistic divergence patterns. Wasm: exports for Node.
// Native build: rustc --edition 2021 -C opt-level=3 -o <out>/bench rr_hash_bench.rs
// Wasm build:   rustc --edition 2021 -C opt-level=3 --target wasm32-unknown-unknown --crate-type cdylib -o <out>/bench.wasm rr_hash_bench.rs
#![allow(dead_code)]

use std::cell::RefCell;

const FNV_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

#[inline(never)]
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
    }
    hash
}

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

fn read64(b: &[u8], at: usize) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[at..at + 8]);
    u64::from_le_bytes(w)
}

fn read32(b: &[u8], at: usize) -> u32 {
    let mut w = [0u8; 4];
    w.copy_from_slice(&b[at..at + 4]);
    u32::from_le_bytes(w)
}

fn xround(acc: u64, input: u64) -> u64 {
    acc.wrapping_add(input.wrapping_mul(P2)).rotate_left(31).wrapping_mul(P1)
}

fn xmerge(acc: u64, val: u64) -> u64 {
    (acc ^ xround(0, val)).wrapping_mul(P1).wrapping_add(P4)
}

/// XXH64 (Cyan4973/xxHash, doc/xxhash_spec.md), seed 0. The length is mixed in as a u64 value, the same on wasm32.
#[inline(never)]
pub fn xxh64(input: &[u8]) -> u64 {
    let seed = 0u64;
    let len = input.len();
    let mut p = 0;
    let mut h: u64;
    if len >= 32 {
        let (mut v1, mut v2, mut v3, mut v4) = (
            seed.wrapping_add(P1).wrapping_add(P2),
            seed.wrapping_add(P2),
            seed,
            seed.wrapping_sub(P1),
        );
        while p + 32 <= len {
            v1 = xround(v1, read64(input, p));
            v2 = xround(v2, read64(input, p + 8));
            v3 = xround(v3, read64(input, p + 16));
            v4 = xround(v4, read64(input, p + 24));
            p += 32;
        }
        h = v1
            .rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18));
        h = xmerge(h, v1);
        h = xmerge(h, v2);
        h = xmerge(h, v3);
        h = xmerge(h, v4);
    } else {
        h = seed.wrapping_add(P5);
    }
    h = h.wrapping_add(len as u64);
    while p + 8 <= len {
        h = (h ^ xround(0, read64(input, p))).rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
        p += 8;
    }
    if p + 4 <= len {
        h = (h ^ u64::from(read32(input, p)).wrapping_mul(P1)).rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
        p += 4;
    }
    while p < len {
        h = (h ^ u64::from(input[p]).wrapping_mul(P5)).rotate_left(11).wrapping_mul(P1);
        p += 1;
    }
    h ^= h >> 33;
    h = h.wrapping_mul(P2);
    h ^= h >> 29;
    h = h.wrapping_mul(P3);
    h ^= h >> 32;
    h
}

/// The bytes the project's timing test hashes (hash_cost.rs:34, wasm.rs:68): (at * 31 % 251).
fn pattern(length: usize) -> Vec<u8> {
    (0..length).map(|at| (at * 31 % 251) as u8).collect()
}

// ---------------------------------------------------------------- Wasm exports (timed from Node)
thread_local! {
    static BUF: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn fill(length: u32) {
    BUF.with(|b| *b.borrow_mut() = pattern(length as usize));
}

#[no_mangle]
pub extern "C" fn fnv_once() -> u64 {
    BUF.with(|b| fnv1a_64(std::hint::black_box(&b.borrow())))
}

#[no_mangle]
pub extern "C" fn xxh_once() -> u64 {
    BUF.with(|b| xxh64(std::hint::black_box(&b.borrow())))
}

#[no_mangle]
pub extern "C" fn fnv_rounds(rounds: u32) -> u64 {
    BUF.with(|b| {
        let b = b.borrow();
        (0..rounds).fold(0u64, |f, r| f.rotate_left(1) ^ fnv1a_64(std::hint::black_box(&b)) ^ u64::from(r))
    })
}

#[no_mangle]
pub extern "C" fn xxh_rounds(rounds: u32) -> u64 {
    BUF.with(|b| {
        let b = b.borrow();
        (0..rounds).fold(0u64, |f, r| f.rotate_left(1) ^ xxh64(std::hint::black_box(&b)) ^ u64::from(r))
    })
}

#[no_mangle]
pub extern "C" fn digest_check() -> u32 {
    // 1 when both functions give the native digests below on wasm32 too (the word-size check for this scratch).
    let fnv_ok = fnv1a_64(b"foobar") == 0x8594_4171_f739_67e8 && fnv1a_64(&pattern(16_384)) == FNV_16K;
    let xxh_ok = xxh64(b"Nobody inspects the spammish repetition") == 0xfbce_a83c_8a37_8bf1 && xxh64(&pattern(16_384)) == XXH_16K;
    u32::from(fnv_ok && xxh_ok)
}

// Filled in from the native run (printed there); the Wasm export compares against them.
const FNV_16K: u64 = 0x828c_6038_8bc7_6450;
const XXH_16K: u64 = 0xc5c2_328f_f0a3_6552;

// ---------------------------------------------------------------- native: cost
#[cfg(not(target_arch = "wasm32"))]
fn timed(rounds: usize, mut call: impl FnMut() -> u64) -> (f64, u128, f64) {
    use std::time::Instant;
    let mut nanos = Vec::with_capacity(rounds);
    let mut folded = 0u64;
    for _ in 0..rounds {
        let start = Instant::now();
        folded ^= std::hint::black_box(call());
        nanos.push(start.elapsed().as_nanos());
    }
    nanos.sort_unstable();
    let mean = nanos.iter().sum::<u128>() as f64 / rounds as f64;
    // The block mean: the same calls with one timer pair around all of them.
    let start = Instant::now();
    for _ in 0..rounds {
        folded ^= std::hint::black_box(call());
    }
    let block = start.elapsed().as_nanos() as f64 / rounds as f64;
    std::hint::black_box(folded);
    (mean, nanos[rounds * 99 / 100], block)
}

/// A proxy of world.rs's Canonical sink for a soldier table: 4-byte fields through extend_from_slice into a reused Vec.
#[cfg(not(target_arch = "wasm32"))]
#[inline(never)]
fn serialize(tick: u32, rows: &[[u32; 16]], out: &mut Vec<u8>) {
    out.clear();
    out.reserve(4 + rows.len() * 64);
    out.extend_from_slice(&u32::to_le_bytes(tick));
    for row in rows {
        for &field in row {
            out.extend_from_slice(&u32::to_le_bytes(field));
        }
    }
}

// ---------------------------------------------------------------- native: structure test
/// A deterministic generator for the synthetic state (splitmix64).
#[cfg(not(target_arch = "wasm32"))]
struct Mix(u64);
#[cfg(not(target_arch = "wasm32"))]
impl Mix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn f32_in(&mut self, lo: f32, hi: f32) -> u32 {
        let unit = (self.next() >> 40) as f32 / (1u64 << 24) as f32;
        (lo + (hi - lo) * unit).to_bits()
    }
}

/// 48 soldiers of 16 four-byte fields: generation, placement, team, flags as small integers; position, velocity,
/// facing, aim, stamina, recovery and four more as plausible f32 bit patterns.
#[cfg(not(target_arch = "wasm32"))]
fn soldier_table() -> Vec<[u32; 16]> {
    let mut mix = Mix(0x5348_414D_424C_4521);
    (0..48u32)
        .map(|id| {
            let mut row = [0u32; 16];
            row[0] = 1 + id % 3; // generation
            row[1] = id; // placement
            row[2] = id % 2; // team
            row[3] = (mix.next() & 0x7) as u32; // flags
            row[4] = mix.f32_in(-120.0, 120.0); // position x
            row[5] = mix.f32_in(0.0, 4.0); // position y
            row[6] = mix.f32_in(-120.0, 120.0); // position z
            row[7] = mix.f32_in(-6.0, 6.0); // velocity x
            row[8] = mix.f32_in(-1.0, 1.0); // velocity y
            row[9] = mix.f32_in(-6.0, 6.0); // velocity z
            row[10] = mix.f32_in(-3.14159, 3.14159); // facing
            row[11] = mix.f32_in(-1.5, 1.5); // aim pitch
            row[12] = mix.f32_in(0.0, 100.0); // stamina
            row[13] = mix.f32_in(0.0, 1.0); // recovery
            row[14] = if id % 5 == 0 { 0 } else { mix.f32_in(0.0, 2.0) }; // a timer, often zero
            row[15] = 0; // a field still zero
            row
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn count_pairs(mut keys: Vec<u64>) -> u64 {
    keys.sort_unstable();
    let (mut pairs, mut run) = (0u64, 1u64);
    for w in keys.windows(2) {
        if w[0] == w[1] {
            run += 1;
        } else {
            pairs += run * (run - 1) / 2;
            run = 1;
        }
    }
    pairs + run * (run - 1) / 2
}

#[cfg(not(target_arch = "wasm32"))]
struct Tally {
    base: (u64, u64),
    made: [u64; 6],
    eq_base: [(u32, u32); 6],
    all: Vec<(u64, u64)>,
    e_only: Vec<(u64, u64)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Tally {
    fn add(&mut self, class: usize, v: &[u8]) {
        let pair = (fnv1a_64(v), xxh64(v));
        self.made[class] += 1;
        self.eq_base[class].0 += u32::from(pair.0 == self.base.0);
        self.eq_base[class].1 += u32::from(pair.1 == self.base.1);
        self.all.push(pair);
        if class == 4 {
            self.e_only.push(pair);
        }
    }
}

/// Distinct inputs only: identical byte strings made by two classes (a bit-0 flip that equals a +1 ulp, say) give
/// the same (fnv, xxh) pair and are one input; distinct inputs with both 64-bit digests equal are not expected.
#[cfg(not(target_arch = "wasm32"))]
fn distinct(set: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut v = set.to_vec();
    v.sort_unstable();
    v.dedup();
    v
}

#[cfg(not(target_arch = "wasm32"))]
fn structure_test() {
    let tick = 9_000u32;
    let base = soldier_table();
    let mut bytes = Vec::new();
    serialize(tick, &base, &mut bytes);
    let fields = base.len() * 16;
    let names = [
        "A one bit flipped",
        "B one field +-1 ulp",
        "C two rows swapped",
        "D two fields +-1 ulp",
        "E two sign bits flipped",
        "F row removed/zeroed/copied",
    ];
    let mut t = Tally {
        base: (fnv1a_64(&bytes), xxh64(&bytes)),
        made: [0; 6],
        eq_base: [(0, 0); 6],
        all: Vec::with_capacity(1_000_000),
        e_only: Vec::with_capacity(300_000),
    };
    let mut rows = base.clone();
    let mut out = Vec::with_capacity(bytes.len());
    let mut emit = |t: &mut Tally, class: usize, change: &dyn Fn(&mut Vec<[u32; 16]>)| {
        rows.clone_from(&base);
        change(&mut rows);
        serialize(tick, &rows, &mut out);
        assert_ne!(out, bytes, "a variant must differ from the base");
        t.add(class, &out);
    };
    // A: every single-bit flip of the canonical bytes.
    let mut flipped = bytes.clone();
    for at in 0..bytes.len() {
        for bit in 0..8 {
            flipped[at] ^= 1 << bit;
            t.add(0, &flipped);
            flipped[at] ^= 1 << bit;
        }
    }
    // B: every field moved by one unit in the last place (+1 and -1 on its bits).
    for f in 0..fields {
        for d in [1u32, u32::MAX] {
            emit(&mut t, 1, &|r| r[f / 16][f % 16] = r[f / 16][f % 16].wrapping_add(d));
        }
    }
    // C: every row swapped with every other (a wrong iteration order).
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            emit(&mut t, 2, &|r| r.swap(i, j));
        }
    }
    // D: two fields each moved by one ulp, every pair, in both relative directions.
    for f in 0..fields {
        for g in f + 1..fields {
            for dg in [1u32, u32::MAX] {
                emit(&mut t, 3, &|r| {
                    r[f / 16][f % 16] = r[f / 16][f % 16].wrapping_add(1);
                    r[g / 16][g % 16] = r[g / 16][g % 16].wrapping_add(dg);
                });
            }
        }
    }
    // E: two sign bits flipped, every pair (differences only in bit 7 of a byte: FNV's weakest class).
    for f in 0..fields {
        for g in f + 1..fields {
            emit(&mut t, 4, &|r| {
                r[f / 16][f % 16] ^= 0x8000_0000;
                r[g / 16][g % 16] ^= 0x8000_0000;
            });
        }
    }
    // F: a row removed and the table compacted (last row zero), a row zeroed, a row copied onto the next.
    for i in 0..base.len() {
        emit(&mut t, 5, &|r| {
            r.remove(i);
            r.push([0; 16]);
        });
        emit(&mut t, 5, &|r| r[i] = [0; 16]);
        if i + 1 < base.len() {
            emit(&mut t, 5, &|r| r[i + 1] = r[i]);
        }
    }

    println!("\nstructure test: base state {} bytes (tick + 48 rows x 64 bytes)", bytes.len());
    println!("  class                         variants  equal-to-base (fnv, xxh)");
    for (k, name) in names.iter().enumerate() {
        println!("  {name:28} {:>9}  {:?}", t.made[k], t.eq_base[k]);
    }
    let base_pair = t.base;
    let all = distinct(&[&t.all[..], &[base_pair]].concat());
    let e_only = distinct(&[&t.e_only[..], &[base_pair]].concat());
    for (label, set) in [("all classes + base", &all), ("class E + base", &e_only)] {
        let n = set.len() as f64;
        let expect32 = n * (n - 1.0) / 2.0 / 4_294_967_296.0;
        let expect25 = n * (n - 1.0) / 2.0 / 33_554_432.0;
        let expect64 = n * (n - 1.0) / 2.0 / 18_446_744_073_709_551_616.0;
        let pick = |f: &dyn Fn(&(u64, u64)) -> u64| count_pairs(set.iter().map(f).collect());
        println!(
            "  {label}: N = {} distinct inputs; colliding pairs (a random function expects {expect32:.1} at 32 bits, {expect25:.0} at 25, {expect64:.1e} at 64)",
            set.len()
        );
        println!(
            "    fnv full 64: {}   fnv top 32: {}   fnv low 32: {}   fnv low 7 bits equal in all: {}",
            pick(&|p| p.0),
            pick(&|p| p.0 >> 32),
            pick(&|p| p.0 & 0xFFFF_FFFF),
            set.iter().all(|p| p.0 & 0x7F == set[0].0 & 0x7F)
        );
        println!(
            "    xxh full 64: {}   xxh top 32: {}   xxh low 32: {}",
            pick(&|p| p.1),
            pick(&|p| p.1 >> 32),
            pick(&|p| p.1 & 0xFFFF_FFFF)
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Known answers: FNV from RFC 9923 (as state_hash.rs's test); XXH64 seed 0 from the reference's published values.
    assert_eq!(fnv1a_64(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(fnv1a_64(b"foobar"), 0x8594_4171_f739_67e8);
    let xxh_known: [(&[u8], u64); 4] = [
        (b"", 0xef46_db37_51d8_e999),
        (b"a", 0xd24e_c4f1_a98c_6e5b),
        (b"abc", 0x44bc_2cf5_ad77_0999),
        (b"Nobody inspects the spammish repetition", 0xfbce_a83c_8a37_8bf1),
    ];
    for (input, want) in xxh_known {
        println!("xxh64({:?}) = {:016x} want {want:016x} {}", String::from_utf8_lossy(input), xxh64(input), if xxh64(input) == want { "ok" } else { "BAD" });
    }
    println!("fnv1a_64(pattern 16384) = {:016x}; xxh64(pattern 16384) = {:016x}", fnv1a_64(&pattern(16_384)), xxh64(&pattern(16_384)));
    if args.iter().any(|a| a == "structure") {
        structure_test();
    }
    if args.iter().any(|a| a == "cost") {
        let rounds = 20_000;
        println!("\ncost, native release (opt-level 3), {rounds} calls per line: per-call mean / p99 (timer included), block mean (one timer pair)");
        for length in [22usize, 54, 3_076, 4_096, 16_384, 65_536] {
            let bytes = pattern(length);
            let (m, p, blk) = timed(rounds, || fnv1a_64(std::hint::black_box(&bytes)));
            let (xm, xp, xblk) = timed(rounds, || xxh64(std::hint::black_box(&bytes)));
            println!(
                "  {length:>6} B  fnv1a_64 {m:>9.1} / {p:>7} / {blk:>9.1} ns   xxh64 {xm:>8.1} / {xp:>6} / {xblk:>8.1} ns"
            );
        }
        // The serializer proxy: what writing the bytes costs beside hashing them.
        for rows_n in [48usize, 256] {
            let rows: Vec<[u32; 16]> = (0..rows_n).map(|i| soldier_table()[i % 48]).collect();
            let mut out = Vec::new();
            serialize(1, &rows, &mut out);
            let len = out.len();
            let (sm, sp, sb) = timed(rounds, || {
                serialize(std::hint::black_box(9), &rows, &mut out);
                out.len() as u64
            });
            let (fm, fp, fb) = timed(rounds, || {
                serialize(std::hint::black_box(9), &rows, &mut out);
                fnv1a_64(&out)
            });
            println!(
                "  serializer proxy {len:>6} B ({rows_n} rows x 16 fields): write {sm:>8.1} / {sp:>6} / {sb:>8.1} ns; write + fnv1a_64 {fm:>8.1} / {fp:>6} / {fb:>8.1} ns"
            );
        }
    }
}
