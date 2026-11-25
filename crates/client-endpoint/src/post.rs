//! The page's newest input, as the Worker's host hands it over once per tick. The page posts one per drawn frame
//! and the host keeps the newest by its number, so a post that comes late cannot roll the counts back; the
//! endpoint takes what the host hands it. `number` counts the page's posts and is never the protocol's command
//! sequence.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects
    )
)]

use sim::BUTTONS;

/// One post: which buttons are held, and the page's running count of presses per button, each wrapping at 256. A
/// post that is skipped loses no press, because the next one's counts still hold it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Post {
    pub number: u32,
    pub buttons: u16,
    pub presses: [u8; BUTTONS],
}

impl Post {
    /// Before the page's first post: nothing held, no press.
    pub const NONE: Self = Self {
        number: 0,
        buttons: 0,
        presses: [0; BUTTONS],
    };

    /// From the plain numbers that cross the bindings: the post number, the held buttons in the low 16 bits (the
    /// rest is ignored), and the 16 counts packed four to a word, button `k` in byte `k % 4` of word `k / 4`.
    #[must_use]
    pub fn from_words(number: u32, held: u32, counts: [u32; 4]) -> Self {
        let mut presses = [0; BUTTONS];
        for (four, word) in presses.as_chunks_mut().0.iter_mut().zip(counts) {
            *four = word.to_le_bytes();
        }
        let [low, high, ..] = held.to_le_bytes();
        Self {
            number,
            buttons: u16::from_le_bytes([low, high]),
            presses,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_words_unpack_to_each_button_s_count_and_the_held_bits() {
        let post = Post::from_words(7, 0xFFFF_8001, [0x0403_0201, 0x0807_0605, 0, 0xFF00_0000]);
        assert_eq!(post.number, 7);
        assert_eq!(post.buttons, 0x8001);
        assert_eq!(
            post.presses,
            [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 255]
        );
        assert_eq!(Post::from_words(0, 0, [0; 4]), Post::NONE);
    }
}
