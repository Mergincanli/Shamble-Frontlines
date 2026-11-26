//! SHA-256 (FIPS 180-4), for the tools' own files: a recording's hashes file names the recording it belongs to by
//! this digest, and the recordings' manifest lists each file with it, so `sha256sum` can check a line by hand. It
//! is written here because the workspace has no crate for it and a tool is no reason to bring one in. Tools and
//! tests only: nothing a player gets holds it.

/// The round constants: the first 32 bits of the fractional parts of the cube roots of the first 64 primes.
#[rustfmt::skip]
const K: [u32; 64] = [
    0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4, 0xab1c_5ed5,
    0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe, 0x9bdc_06a7, 0xc19b_f174,
    0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc, 0x2de9_2c6f, 0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da,
    0x983e_5152, 0xa831_c66d, 0xb003_27c8, 0xbf59_7fc7, 0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967,
    0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc, 0x5338_0d13, 0x650a_7354, 0x766a_0abb, 0x81c2_c92e, 0x9272_2c85,
    0xa2bf_e8a1, 0xa81a_664b, 0xc24b_8b70, 0xc76c_51a3, 0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070,
    0x19a4_c116, 0x1e37_6c08, 0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
    0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb, 0xbef9_a3f7, 0xc671_78f2,
];

/// The start values: the first 32 bits of the fractional parts of the square roots of the first 8 primes.
#[rustfmt::skip]
const START: [u32; 8] = [
    0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a, 0x510e_527f, 0x9b05_688c, 0x1f83_d9ab, 0x5be0_cd19,
];

fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for (word, four) in w.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_be_bytes(*four);
    }
    for i in 16..64 {
        let (a, b) = (w[i - 15], w[i - 2]);
        let s0 = a.rotate_right(7) ^ a.rotate_right(18) ^ (a >> 3);
        let s1 = b.rotate_right(17) ^ b.rotate_right(19) ^ (b >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (k, w) in K.iter().zip(w) {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choose = (e & f) ^ (!e & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(choose)
            .wrapping_add(*k)
            .wrapping_add(w);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        (h, g, f, e) = (g, f, e, d.wrapping_add(t1));
        (d, c, b, a) = (c, b, a, t1.wrapping_add(s0.wrapping_add(majority)));
    }
    for (word, add) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *word = word.wrapping_add(add);
    }
}

/// The SHA-256 digest of `message`.
#[must_use]
pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut state = START;
    let (blocks, rest) = message.as_chunks::<64>();
    for block in blocks {
        compress(&mut state, block);
    }
    // The end: what is left of the message, one 0x80, zeros, and the message's length in bits as a big-endian
    // `u64`; in one block when what is left is shorter than 56 bytes, else in two.
    let mut end = [0u8; 128];
    end[..rest.len()].copy_from_slice(rest);
    end[rest.len()] = 0x80;
    let end_len = if rest.len() < 56 { 64 } else { 128 };
    let bits = (message.len() as u64).wrapping_mul(8);
    end[end_len - 8..end_len].copy_from_slice(&bits.to_be_bytes());
    for block in end[..end_len].as_chunks::<64>().0 {
        compress(&mut state, block);
    }
    let mut digest = [0; 32];
    for (four, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(state) {
        *four = word.to_be_bytes();
    }
    digest
}

/// `bytes` as lower-case hex, two digits a byte.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        // Writing into a `String` cannot fail.
        let _ = write!(text, "{byte:02x}");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FIPS 180-4's own examples, and answers from a second implementation (Python's hashlib) at every length where
    /// the end changes shape: 55 and 56 bytes left, 63 and 64, and the same one block further on.
    #[test]
    fn sha256_matches_the_standard_s_examples_and_a_second_implementation() {
        let of = |message: &[u8]| hex(&sha256(message));
        assert_eq!(
            of(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            of(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            of(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        let a = |count: usize| of(&vec![b'a'; count]);
        assert_eq!(
            a(55),
            "9f4390f8d30c2dd92ec9f095b65e2b9ae9b0a925a5258e241c9f1e910f734318"
        );
        assert_eq!(
            a(56),
            "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a"
        );
        assert_eq!(
            a(63),
            "7d3e74a05d7db15bce4ad9ec0658ea98e3f06eeecf16b4c6fff2da457ddc2f34"
        );
        assert_eq!(
            a(64),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
        assert_eq!(
            a(119),
            "31eba51c313a5c08226adf18d4a359cfdfd8d2e816b13f4af952f7ea6584dcfb"
        );
        assert_eq!(
            a(120),
            "2f3d335432c70b580af0e8e1b3674a7c020d683aa5f73aaaedfdc55af904c21c"
        );
        assert_eq!(
            a(1_000_000),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }
}
