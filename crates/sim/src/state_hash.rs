//! The state hash's function: one fixed, unseeded hash over the canonical bytes of the whole state (ADR 0004 part 9).
//! Two runs of a match agree while their per-tick hashes agree. It finds divergence; it is not a security check, and
//! its value never leaves the Worker or the server, because it covers the match seed (ADR 0004 part 8).
//!
//! FNV-1a is the pick for the first, 36-byte world and is provisional: ADR 0004 part 9 leaves the function to the
//! build, and PM-0004-21's report (the function picked and its cost per tick) confirms or replaces it before the
//! first recording is committed.

const FNV_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

/// FNV-1a, 64 bits (RFC 9923), over `bytes` as one byte string. It reads bytes only, never a length or a `usize`, so
/// the digest is the same on 32-bit and 64-bit targets.
#[must_use]
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The test values of RFC 9923 for FNV-1a, 64 bits.
    #[test]
    fn fnv1a_64_matches_rfc_9923() {
        assert_eq!(fnv1a_64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a_64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a_64(b"foobar"), 0x8594_4171_f739_67e8);
    }

    /// Every byte and its place count: one changed byte, two swapped bytes and one byte more each change the digest.
    #[test]
    fn fnv1a_64_covers_every_byte_and_its_place() {
        let base = fnv1a_64(&[1, 2, 3, 4]);
        assert_ne!(base, fnv1a_64(&[1, 2, 3, 5]));
        assert_ne!(base, fnv1a_64(&[2, 1, 3, 4]));
        assert_ne!(base, fnv1a_64(&[1, 2, 3, 4, 0]));
        assert_ne!(base, fnv1a_64(&[1, 2, 3]));
    }
}
