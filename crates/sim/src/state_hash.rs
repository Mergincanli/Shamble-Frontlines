//! The state hash's function: FNV-1a 64 (RFC 9923), fixed and unseeded, over the canonical bytes (ADR 0004 part 9). It
//! finds divergence between two runs, tick by tick; it is not a security check. Over the whole state it covers the
//! match seed, so that value never leaves the Worker or the server (part 8); over the shared state it covers no seed
//! byte and is the one number a replay reads from the players' file. Kept by PM-0004-21's report (change-set 4a): at
//! 16 KiB it takes 20 us natively and in WebAssembly, 0.06% of a tick, and a change in one byte is always caught.
//! Revisit when it first runs on a live tick or the shared state passes 64 KiB (then XXH64: 13 to 16 times faster).

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
        // The RFC's fourth string is the only one with bytes above 0x7F.
        assert_eq!(fnv1a_64(b"Hello!\x01\xFF\xED"), 0xbd51_ea70_94ee_6fa1);
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
