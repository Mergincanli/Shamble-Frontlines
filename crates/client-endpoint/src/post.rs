//! The page's newest input, as the Worker's host hands it over once per tick. The page posts one per drawn frame
//! and the host keeps the newest; `number` counts the page's posts and is never the protocol's command sequence.
#![cfg_attr(
    not(test),
    deny(clippy::indexing_slicing, clippy::arithmetic_side_effects)
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

    /// Whether this post is newer than `other`: its number lies 1 to 2^31 - 1 ahead, around the 32-bit circle. The
    /// host keeps the newest post by this and never the last to arrive, so a post that comes late cannot roll the
    /// counts back.
    #[must_use]
    pub const fn newer_than(&self, other: &Self) -> bool {
        let ahead = self.number.wrapping_sub(other.number);
        ahead != 0 && ahead < 0x8000_0000
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

    #[test]
    fn a_post_is_newer_by_its_number_around_the_wrap() {
        let post = |number| Post {
            number,
            ..Post::NONE
        };
        assert!(post(1).newer_than(&post(0)));
        assert!(!post(0).newer_than(&post(1)));
        assert!(!post(5).newer_than(&post(5)));
        assert!(post(0).newer_than(&post(u32::MAX)));
        assert!(!post(u32::MAX).newer_than(&post(0)));
        assert!(post(0x7FFF_FFFF).newer_than(&post(0)));
        assert!(!post(0x8000_0000).newer_than(&post(0)));
    }
}
