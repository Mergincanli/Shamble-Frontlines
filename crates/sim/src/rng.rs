//! The generator of the simulation's random streams: `ChaCha` with 8 rounds (ADR 0004 part 8). It is our own code and
//! no library's (the answer RNG-SOURCE (c), which Mustafa confirmed on 2026-11-23: our own `ChaCha8`, checked against
//! the published test vectors), so the server and every browser run these same lines: plain 32-bit adds, xors and
//! rotations, with no processor-specific path that could make them disagree. The tests below hold those vectors.
//!
//! The layout is Bernstein's: sixteen 32-bit words, four constants, the 256-bit key, a 64-bit block counter, then
//! the 64-bit stream id. One key gives 2^64 streams that never overlap; the simulation gives each purpose its own.
//! Nothing here picks a stream or draws for a purpose yet: each purpose joins with its first draw.

use crate::SEED_BYTES;

/// "expand 32-byte k", as four little-endian words.
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];

/// Words per block.
const BLOCK_WORDS: u64 = 16;

// The key is eight words: a seed of another length would be cut short without a word (`new` pairs them up).
const _: () = assert!(SEED_BYTES == 32);

/// One random stream of a match: the match seed as the key, a stream id, and a position. There is no `Debug`: the
/// seed is never printed or logged (ADR 0004 part 8).
#[derive(Clone)]
pub struct ChaCha8 {
    key: [u32; 8],
    stream: u64,
    /// The index of the next word, counted from the stream's start.
    position: u64,
    /// The block the next word lies in, once it has been computed, with its number.
    block: Option<(u64, [u32; 16])>,
}

impl ChaCha8 {
    /// The stream `stream` of the key `seed`, at its start.
    #[must_use]
    pub fn new(seed: &[u8; SEED_BYTES], stream: u64) -> Self {
        let mut key = [0; 8];
        for (word, bytes) in key.iter_mut().zip(seed.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*bytes);
        }
        Self {
            key,
            stream,
            position: 0,
            block: None,
        }
    }

    /// The stream's id.
    #[must_use]
    pub const fn stream(&self) -> u64 {
        self.stream
    }

    /// The index of the next word. With the seed and the stream id it is the stream's whole state.
    #[must_use]
    pub const fn position(&self) -> u64 {
        self.position
    }

    /// Moves to the word `position`: the next draw is the word a fresh stream would give after that many draws.
    pub fn set_position(&mut self, position: u64) {
        self.position = position;
    }

    /// The next 32 bits.
    ///
    /// # Panics
    /// After 2^64 - 1 words: the position stops loudly and never wraps (ADR 0004 part 4).
    pub fn next_u32(&mut self) -> u32 {
        let number = self.position / BLOCK_WORDS;
        let block = match self.block {
            Some((held, block)) if held == number => block,
            _ => {
                let block = self.compute(number);
                self.block = Some((number, block));
                block
            }
        };
        let index = usize::try_from(self.position % BLOCK_WORDS).expect("an index below 16");
        self.position = self
            .position
            .checked_add(1)
            .expect("the random stream is at its last word and cannot advance");
        block[index]
    }

    /// The next 64 bits: two words in the order they are drawn, the first one low.
    ///
    /// # Panics
    /// As [`Self::next_u32`], at the stream's last word.
    pub fn next_u64(&mut self) -> u64 {
        let low = u64::from(self.next_u32());
        low | (u64::from(self.next_u32()) << 32)
    }

    /// Block `number` of this stream.
    fn compute(&self, number: u64) -> [u32; 16] {
        let (number, stream) = (u64::to_le_bytes(number), u64::to_le_bytes(self.stream));
        let half = |bytes: [u8; 8], at: usize| {
            u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
        };
        let mut start = [0; 16];
        start[..4].copy_from_slice(&CONSTANTS);
        start[4..12].copy_from_slice(&self.key);
        start[12..].copy_from_slice(&[
            half(number, 0),
            half(number, 4),
            half(stream, 0),
            half(stream, 4),
        ]);
        let mut x = start;
        // Eight rounds: four times a round down the columns, then one down the diagonals.
        for _ in 0..4 {
            quarter_round(&mut x, 0, 4, 8, 12);
            quarter_round(&mut x, 1, 5, 9, 13);
            quarter_round(&mut x, 2, 6, 10, 14);
            quarter_round(&mut x, 3, 7, 11, 15);
            quarter_round(&mut x, 0, 5, 10, 15);
            quarter_round(&mut x, 1, 6, 11, 12);
            quarter_round(&mut x, 2, 7, 8, 13);
            quarter_round(&mut x, 3, 4, 9, 14);
        }
        for (word, first) in x.iter_mut().zip(start) {
            *word = word.wrapping_add(first);
        }
        x
    }
}

/// `ChaCha`'s quarter round on four words of the block. The additions wrap on purpose: they are sums modulo 2^32.
fn quarter_round(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(16);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(12);
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(8);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(7);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The published test vectors for 8 rounds and a 256-bit key, TC1 to TC8 of the Internet-Draft
    /// "draft-strombergson-chacha-test-vectors-01" (J. Strombergson, December 2013;
    /// www.ietf.org/archive/id/draft-strombergson-chacha-test-vectors-01.txt): the key, the IV, and the first two
    /// keystream blocks. Copied from the draft's text by a script, never typed. The text they were copied from, as
    /// fetched on 2026-11-24: 76,754 bytes, sha256
    /// e0a49fddc11091989b018b0d4c7ffb73c44f849255659b938b464c65d1ab4d55.
    #[rustfmt::skip]
    const PUBLISHED: [(&str, &str, &str); 8] = [
        // TC1
        ("0000000000000000000000000000000000000000000000000000000000000000", "0000000000000000", concat!(
            "3e00ef2f895f40d67f5bb8e81f09a5a12c840ec3ce9a7f3b181be188ef711a1e",
            "984ce172b9216f419f445367456d5619314a42a3da86b001387bfdb80e0cfe42",
            "d2aefa0deaa5c151bf0adb6c01f2a5adc0fd581259f9a2aadcf20f8fd566a26b",
            "5032ec38bbc5da98ee0c6f568b872a65a08abf251deb21bb4b56e5d8821e68aa",
        )),
        // TC2
        ("0100000000000000000000000000000000000000000000000000000000000000", "0000000000000000", concat!(
            "cf5ee9a0494aa9613e05d5ed725b804b12f4a465ee635acc3a311de8740489ea",
            "289d04f43c7518db56eb4433e498a1238cd8464d3763ddbb9222ee3bd8fae3c8",
            "b4355a7d93dd8867089ee643558b95754efa2bd1a8a1e2d75bcdb32015542638",
            "291941feb49965587c4fdfe219cf0ec132a6cd4dc067392e67982fe53278c0b4",
        )),
        // TC3
        ("0000000000000000000000000000000000000000000000000000000000000000", "0100000000000000", concat!(
            "2b8f4bb3798306ca5130d47c4f8d4ed13aa0edccc1be6942090faeeca0d7599b",
            "7ff0fe616bb25aa0153ad6fdc88b954903c22426d478b97b22b8f9b1db00cf06",
            "470bdffbc488a8b7c701ebf4061d75c5969186497c95367809afa80bd843b040",
            "a79abc6e73a91757f1db73c8eacfa543b38f289d065ab2f3032d377b8c37fe46",
        )),
        // TC4
        ("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "ffffffffffffffff", concat!(
            "e163bbf8c9a739d18925ee8362dad2cdc973df05225afb2aa26396f2a9849a4a",
            "445e0547d31c1623c537df4ba85c70a9884a35bcbf3dfab077e98b0f68135f54",
            "81d4933f8b322ac0cd762c27235ce2b31534e0244a9a2f1fd5e94498d47ff108",
            "790c009cf9e1a348032a7694cb28024cd96d3498361edb1785af752d187ab54b",
        )),
        // TC5
        ("5555555555555555555555555555555555555555555555555555555555555555", "5555555555555555", concat!(
            "7cb78214e4d3465b6dc62cf7a1538c88996952b4fb72cb6105f1243ce3442e29",
            "75a59ebcd2b2a598290d7538491fe65bdbfefd060d88798120a70d049dc2677d",
            "d48ff5a2513e497a5d54802d7484c4f1083944d8d0d14d6482ce09f7e5ebf20b",
            "29807d62c31874d02f5d3cc85381a745ecbc60525205e300a76961bfe51ac07c",
        )),
        // TC6
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaa", concat!(
            "40f9ab86c8f9a1a0cdc05a75e5531b612d71ef7f0cf9e387df6ed6972f0aae21",
            "311aa581f816c90e8a99de990b6b95aac92450f4e112712667b804c99e9c6eda",
            "f8d144f560c8c0ea36880d3b77874c9a9103d147f6ded386284801a4ee158e5e",
            "a4f9c093fc55fd344c33349dc5b699e21dc83b4296f92ee3ecabf3d51f95fe3f",
        )),
        // TC7
        ("00112233445566778899aabbccddeeffffeeddccbbaa99887766554433221100", "0f1e2d3c4b5a6978", concat!(
            "db43ad9d1e842d1272e4530e276b3f568f8859b3f7cf6d9d2c74fa53808cb515",
            "7a8ebf46ad3dcc4b6c7dadde131784b0120e0e22f6d5f9ffa7407d4a21b695d9",
            "c5dd30bf55612fab9bdd118920c19816470c7f5dcd42325dbbed8c57a56281c1",
            "44cb0f03e81b3004624e0650a1ce5afaf9a7cd8163f6dbd72602257dd96e471e",
        )),
        // TC8
        ("c46ec1b18ce8a878725a37e780dfb7351f68ed2e194c79fbc6aebee1a667975d", "1ada31d5cf688221", concat!(
            "838751b42d8ddd8a3d77f48825a2ba752cf4047cb308a5978ef274973be374c9",
            "6ad848065871417b08f034e681fe46a93f7d5c61d1306614d4aaf257a7cff08b",
            "16f2fda170cc18a4b58a2667ed962774af792a6e7f3c77992540711a7a136d7e",
            "8a2f8d3f93816709d45a3fa5f8ce72fde15be7b841acba3a2abd557228d9fe4f",
        )),
    ];

    fn bytes(hex: &str) -> Vec<u8> {
        let digit =
            |index: usize| u8::from_str_radix(&hex[index..index + 2], 16).expect("two hex digits");
        (0..hex.len()).step_by(2).map(digit).collect()
    }

    fn stream_of(key_hex: &str, iv_hex: &str) -> ChaCha8 {
        let key: [u8; SEED_BYTES] = bytes(key_hex).try_into().expect("a 32-byte key");
        // The draft's 64-bit IV fills the two words this layout gives the stream id, in the same byte order.
        let iv: [u8; 8] = bytes(iv_hex).try_into().expect("an 8-byte IV");
        ChaCha8::new(&key, u64::from_le_bytes(iv))
    }

    #[test]
    fn the_keystream_is_the_published_one() {
        for (case, (key, iv, keystream)) in PUBLISHED.iter().enumerate() {
            let mut stream = stream_of(key, iv);
            let drawn: Vec<u8> = (0..32)
                .flat_map(|_| u32::to_le_bytes(stream.next_u32()))
                .collect();
            assert_eq!(drawn, bytes(keystream), "TC{} differs", case + 1);
            assert_eq!(stream.position(), 32);
        }
        // The eight cases are not one case eight times.
        let firsts: Vec<&str> = PUBLISHED
            .iter()
            .map(|(_, _, keystream)| &keystream[..16])
            .collect();
        assert!((1..8).all(|case| !firsts[..case].contains(&firsts[case])));
    }

    #[test]
    fn a_position_is_the_whole_place_in_a_stream() {
        let (key, iv, _) = PUBLISHED[7];
        let mut fresh = stream_of(key, iv);
        let words: Vec<u32> = (0..100).map(|_| fresh.next_u32()).collect();
        // From any word, also in the middle of a block, across a block's end, and backwards.
        for start in [0, 1, 15, 16, 17, 31, 32, 63, 99, 5] {
            let mut moved = stream_of(key, iv);
            moved.set_position(start);
            let index = usize::try_from(start).expect("a small index");
            assert_eq!(moved.next_u32(), words[index], "word {start}");
            assert_eq!(moved.position(), start + 1);
        }
        // A copy made in mid-stream goes on as the original does.
        let mut copy = fresh.clone();
        assert_eq!(
            (copy.next_u32(), copy.stream()),
            (fresh.next_u32(), fresh.stream())
        );
    }

    #[test]
    fn sixty_four_bits_are_two_words_the_first_one_low() {
        let (key, iv, _) = PUBLISHED[7];
        let (mut pairs, mut words) = (stream_of(key, iv), stream_of(key, iv));
        for _ in 0..20 {
            let (low, high) = (u64::from(words.next_u32()), u64::from(words.next_u32()));
            assert_eq!(pairs.next_u64(), low | (high << 32));
        }
        assert_eq!(pairs.position(), 40);
        // From an odd word, so that a pair lies across a block's end (words 15 and 16), and from a block's last word.
        for start in [1, 15, 31] {
            pairs.set_position(start);
            words.set_position(start);
            for _ in 0..12 {
                let (low, high) = (u64::from(words.next_u32()), u64::from(words.next_u32()));
                assert_eq!(pairs.next_u64(), low | (high << 32), "from word {start}");
            }
            assert_eq!(pairs.position(), start + 24);
        }
    }

    #[test]
    fn streams_of_one_key_differ_and_each_half_of_the_id_counts() {
        let seed = [7; SEED_BYTES];
        let first = |stream: u64| ChaCha8::new(&seed, stream).next_u64();
        let seen = [first(0), first(1), first(1 << 32), first(u64::MAX)];
        assert!((1..4).all(|index| !seen[..index].contains(&seen[index])));
        assert_ne!(first(0), ChaCha8::new(&[8; SEED_BYTES], 0).next_u64());
    }

    #[test]
    fn the_block_counter_carries_into_its_high_word() {
        // Block 2^32 is not block 0 again: the counter is 64 bits wide.
        let mut stream = ChaCha8::new(&[0; SEED_BYTES], 0);
        let start = stream.next_u32();
        stream.set_position((1 << 32) * BLOCK_WORDS);
        assert_ne!(stream.next_u32(), start);
        // A known answer from a second implementation (the reference script beside the vectors): block 2^32 - 1.
        stream.set_position(((1 << 32) - 1) * BLOCK_WORDS);
        assert_eq!(
            (stream.next_u32(), stream.next_u32()),
            (0x37b7_a234, 0xec19_1d75)
        );
    }

    #[test]
    #[should_panic(expected = "the random stream is at its last word")]
    fn the_position_stops_loudly_and_never_wraps() {
        let mut stream = ChaCha8::new(&[0; SEED_BYTES], 0);
        stream.set_position(u64::MAX);
        let _ = stream.next_u32();
    }
}
