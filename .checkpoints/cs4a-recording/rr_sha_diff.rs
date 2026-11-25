// rust-reviewer r1 scratch: devtools' own SHA-256 (the real file, brought in by path) against Python's hashlib,
// natively and as wasm32 (a 32-bit usize). Native: `rustc --edition 2024 -O` then run with no argument, it prints
// one "<length> <hex>" line per length for rr_sha_check.py. Wasm32: built as a cdylib, its two exports run in Node.
#![allow(dead_code)]

#[path = "/home/user/Shamble-Frontlines/.gate/lane1-takeover/crates/devtools/src/sha256.rs"]
mod sha256;

/// The message of `length` bytes the comparison uses: byte i is (i * 7 + i / 251) mod 256.
fn message(length: usize) -> Vec<u8> {
    (0..length).map(|i| (i.wrapping_mul(7).wrapping_add(i / 251)) as u8).collect()
}

/// The lengths compared: every length 0..=300 (every padding shape, twice over), and some longer ones.
fn lengths() -> Vec<usize> {
    let mut all: Vec<usize> = (0..=300).collect();
    all.extend([447, 448, 511, 512, 1000, 4095, 4096, 4097, 65_535, 65_536, 1_000_003]);
    all
}

/// All digests folded with FNV-1a 64 over their bytes, in order: one number per target to compare.
#[unsafe(no_mangle)]
pub extern "C" fn fold_all() -> u64 {
    let mut hash = 0xCBF2_9CE4_8422_2325u64;
    for length in lengths() {
        for byte in sha256::sha256(&message(length)) {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    hash
}

/// The first 8 bytes (big-endian) of the digest of `length` bytes of 'a': for a length past 2^29 bytes, whose
/// count of bits needs more than 32 bits.
#[unsafe(no_mangle)]
pub extern "C" fn long_a(length: u32) -> u64 {
    let digest = sha256::sha256(&vec![b'a'; length as usize]);
    u64::from_be_bytes(digest[..8].try_into().unwrap())
}

fn main() {
    for length in lengths() {
        println!("{length} {}", sha256::hex(&sha256::sha256(&message(length))));
    }
    println!("fold {:016x}", fold_all());
    if std::env::args().nth(1).as_deref() == Some("long") {
        println!("long {:016x}", long_a((1 << 29) + 1));
    }
}
