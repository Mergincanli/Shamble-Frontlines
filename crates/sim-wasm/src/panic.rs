//! The panic hook that keeps a trap's message (ADR 0004 part 20's check; ADR 0002 part 5). A panic in the Wasm ends
//! in a trap, and the host then sees only the engine's own words for it ("unreachable"). This hook writes the
//! panic's own text, its place and its message, into one fixed buffer first; the host reads the buffer after the
//! trap, through a fresh view of the memory and with no further call into the Wasm (`panic_buffer` is asked once,
//! when the Worker starts).
//!
//! The buffer describes itself, so the host copies no size from here: bytes 0 to 3 hold the text's length (a `u32`,
//! little-endian; 0 while nothing has panicked), bytes 4 to 7 how many bytes of text fit, and the text follows as
//! UTF-8, cut at a character's end when it is too long. The hook's own write allocates nothing, and each byte is an
//! atomic, so there is no `unsafe` here and no lock that a trap could leave taken.
//!
//! What it does not show: a stop for want of memory. The standard library does not run a panic hook for a failed
//! allocation (it prints and aborts), so the buffer stays empty and the host falls back to the trap's own text.
//! And a panic inside the hook itself aborts without running it again: the length then stays 0 as well.

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicU8, Ordering};
use wasm_bindgen::prelude::*;

/// The buffer's whole length in bytes.
pub const BUFFER_BYTES: usize = 1024;
/// The two length words before the text.
pub const HEADER_BYTES: usize = 8;

static BUFFER: [AtomicU8; BUFFER_BYTES] = [const { AtomicU8::new(0) }; BUFFER_BYTES];

/// Writes text into a buffer's text part and counts what it wrote. Once a piece did not fit, nothing more is taken.
struct Text<'a> {
    buffer: &'a [AtomicU8],
    length: usize,
    full: bool,
}

impl Write for Text<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let room = self.buffer.len().saturating_sub(HEADER_BYTES + self.length);
        let mut take = if self.full { 0 } else { room.min(text.len()) };
        while !text.is_char_boundary(take) {
            take -= 1;
        }
        self.full |= take < text.len();
        let place = &self.buffer[HEADER_BYTES + self.length..];
        for (cell, byte) in place.iter().zip(&text.as_bytes()[..take]) {
            cell.store(*byte, Ordering::Relaxed);
        }
        self.length += take;
        Ok(())
    }
}

fn store_word(cells: &[AtomicU8], value: usize) {
    let bytes = u32::try_from(value).unwrap_or(u32::MAX).to_le_bytes();
    for (cell, byte) in cells.iter().zip(bytes) {
        cell.store(byte, Ordering::Relaxed);
    }
}

/// Puts `text` into `buffer` in the layout the module's doc gives. The length is stored last, so a reader never
/// finds a length for a text that is half written.
fn record(buffer: &[AtomicU8], text: fmt::Arguments<'_>) {
    store_word(&buffer[..4], 0);
    store_word(&buffer[4..HEADER_BYTES], buffer.len() - HEADER_BYTES);
    let mut writer = Text {
        buffer,
        length: 0,
        full: false,
    };
    // The writer itself never fails; a `Display` that does leaves what was written so far.
    let _ = writer.write_fmt(text);
    store_word(&buffer[..4], writer.length);
}

/// Installs the hook. The glue runs this when the module is instantiated, before any other export can run. It is
/// no export of the glue's own (`private`), and the host has no reason to call it; the Wasm's raw exports still
/// hold it, and a call there would only install the same hook again.
#[wasm_bindgen(start, private)]
pub fn start() {
    store_word(&BUFFER[4..HEADER_BYTES], BUFFER_BYTES - HEADER_BYTES);
    std::panic::set_hook(Box::new(|info| record(&BUFFER, format_args!("{info}"))));
}

/// Where the buffer lies in linear memory. It never moves, so the host asks once, at its start.
#[wasm_bindgen]
#[must_use]
pub fn panic_buffer() -> *const u8 {
    BUFFER.as_ptr().cast()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(count: usize) -> Vec<AtomicU8> {
        (0..count).map(|_| AtomicU8::new(0xEE)).collect()
    }

    /// What a host reads: the two words, then that many bytes of text.
    fn read(buffer: &[AtomicU8]) -> (usize, usize, String) {
        let bytes: Vec<u8> = buffer
            .iter()
            .map(|cell| cell.load(Ordering::Relaxed))
            .collect();
        let word = |at: usize| {
            u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize
        };
        let text = String::from_utf8(bytes[HEADER_BYTES..HEADER_BYTES + word(0)].to_vec())
            .expect("whole characters");
        (word(0), word(4), text)
    }

    #[test]
    fn a_text_that_fits_is_kept_whole_with_its_length_and_the_room() {
        let buffer = cells(HEADER_BYTES + 32);
        record(
            &buffer,
            format_args!("panicked at {}:{}:\n{}", "world.rs", 42, "boom"),
        );
        assert_eq!(
            read(&buffer),
            (29, 32, "panicked at world.rs:42:\nboom".to_owned())
        );
        // A second text replaces the first, a shorter one too.
        record(&buffer, format_args!("again"));
        assert_eq!(read(&buffer), (5, 32, "again".to_owned()));
    }

    #[test]
    fn a_text_that_is_too_long_is_cut_at_a_character_s_end_and_nothing_follows_the_cut() {
        // "é" is two bytes: ten bytes of room hold "aaaaaaaaa" and not half of the "é" after it, and not the "b"
        // that would still fit. Three pieces at run time (the compiler joins pieces it knows into one).
        let buffer = cells(HEADER_BYTES + 10);
        let (first, second, third) = std::hint::black_box(("aaaaaaaaa", 'é', "b"));
        record(&buffer, format_args!("{first}{second}{third}"));
        assert_eq!(read(&buffer), (9, 10, "aaaaaaaaa".to_owned()));
        // Exactly full, and one byte more.
        record(&buffer, format_args!("{}", "0123456789"));
        assert_eq!(read(&buffer), (10, 10, "0123456789".to_owned()));
        record(&buffer, format_args!("{}{}", "0123456789", "x"));
        assert_eq!(read(&buffer), (10, 10, "0123456789".to_owned()));
        // Nothing is written past the buffer's end or before its text.
        let wider = cells(HEADER_BYTES + 4);
        record(&wider[..HEADER_BYTES + 2], format_args!("abcd"));
        assert_eq!(wider[HEADER_BYTES + 2].load(Ordering::Relaxed), 0xEE);
    }

    #[test]
    fn the_hook_writes_a_panic_s_place_and_message_into_the_module_s_buffer() {
        start();
        assert!(std::panic::catch_unwind(|| panic!("planted for the hook's test: {}", 7)).is_err());
        let _ = std::panic::take_hook();
        let (length, room, text) = read(&BUFFER);
        assert_eq!((room, length), (BUFFER_BYTES - HEADER_BYTES, text.len()));
        assert!(
            text.starts_with("panicked at crates/sim-wasm/src/panic.rs:"),
            "{text}"
        );
        assert!(
            text.ends_with(":\nplanted for the hook's test: 7"),
            "{text}"
        );
        assert_eq!(panic_buffer(), BUFFER.as_ptr().cast());
    }
}
