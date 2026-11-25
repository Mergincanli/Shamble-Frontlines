//! The scripts the recordings are made from (ADR 0002 part 8). A recording is never captured from play: it is made
//! by a function here, with whole numbers only and no clock, so making it again is one exact command and gives the
//! same bytes on any machine. The committed files are these functions' output (`files`), checked on every test run.

use sim::{SEED_BYTES, TICK_HZ};

use crate::recording::{Header, Recording, Step, TEST_SEED_MARK};

/// The page's buttons, as `packages/client/src/input/keyboard.ts` numbers them.
const W: u32 = 1 << 0;
const A: u32 = 1 << 1;
const S: u32 = 1 << 2;
const D: u32 = 1 << 3;
const SPACE_BUTTON: usize = 4;
const SPACE: u32 = 1 << SPACE_BUTTON;
const SHIFT: u32 = 1 << 5;

/// The eight directions the move keys make, clockwise from forward.
const DIRECTIONS: [u32; 8] = [W, W | D, D, S | D, S, S | A, A, W | A];

/// A public test seed: `TESTSEED`, then the recording's name (at most 24 bytes of it), then zeros.
#[must_use]
pub fn test_seed(name: &str) -> [u8; SEED_BYTES] {
    let mut seed = [0; SEED_BYTES];
    seed[..8].copy_from_slice(&TEST_SEED_MARK);
    for (byte, letter) in seed[8..].iter_mut().zip(name.bytes()) {
        *byte = letter;
    }
    seed
}

/// A small generator for the scripts' own jitter (a linear congruential one, Knuth's constants): whole numbers
/// only, so the same on every machine. It is no part of the simulation.
struct Jitter(u64);

impl Jitter {
    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as u32
    }
}

/// A match of `steps` ticks of the loop change-set 3b closed, as a page would drive it. `salt` makes another
/// stream of the same kind.
///
/// What it holds: each of the eight directions held in turn, with a pause every fifth stretch and Shift over every
/// third; Space held for a while; a press counted on every key-down, and taps of Space at uneven gaps, two of them
/// in one tick and now and then six at once, which the endpoint's pacer has to spread; a page that is fast (several
/// posts between two ticks, so post numbers are skipped) and now and then slow (no new post, so the same one is
/// handed in again); host times a tick apart with up to 6 ms of jitter, never running backwards. The counts wrap
/// past 255 on the way.
#[must_use]
pub fn looped(name: &str, steps: u32, salt: u64) -> Recording {
    let mut jitter = Jitter(0x5348_4D42_4C52_4543 ^ salt);
    let turn = (salt % 8) as u32;
    let mut counts = [0u8; 16];
    let mut held_before = 0;
    let mut post = 0u32;
    let mut handed = Step::at(0.0);
    let mut made = Vec::with_capacity(steps as usize);
    for step in 0..steps {
        let stretch = step / 45;
        let mut held = if stretch % 5 == 4 {
            0
        } else {
            DIRECTIONS[((stretch + turn) % 8) as usize]
        };
        if stretch % 3 == 1 {
            held |= SHIFT;
        }
        if (600..700).contains(&step) {
            held |= SPACE;
        }
        // The page counts a press on every key-down.
        for (button, count) in counts.iter_mut().enumerate() {
            if held & !held_before & (1 << button) != 0 {
                *count = count.wrapping_add(1);
            }
        }
        held_before = held;
        let taps = match (step + turn) % 31 {
            3 | 8 | 17 | 19 | 20 => 1,
            25 => 2,
            _ => 0,
        };
        let burst = if step % 450 == 449 { 6 } else { 0 };
        counts[SPACE_BUTTON] = counts[SPACE_BUTTON].wrapping_add(taps + burst);
        // A slow page has no new post for this tick; a fast one has made one to three since the last.
        if step % 11 != 10 {
            post += 1 + jitter.next() % 3;
            handed.post = post;
            handed.held = held;
            let fours = counts.as_chunks::<4>().0;
            for (word, four) in handed.presses.iter_mut().zip(fours) {
                *word = u32::from_le_bytes(*four);
            }
        }
        let micros =
            u64::from(step + 1) * 1_000_000 / u64::from(TICK_HZ) + u64::from(jitter.next() % 6_001);
        made.push(Step {
            host_ms: micros as f64 / 1000.0,
            ..handed
        });
    }
    Recording {
        header: Header {
            seed: test_seed(name),
            start_tick: 0,
            first_sequence: 0,
            tick_rate: TICK_HZ as u16,
            content_hash: [0; 32],
            map_id: String::new(),
        },
        steps: made,
    }
}

/// The first recording: one minute of the loop.
#[must_use]
pub fn first_loop() -> Recording {
    looped("first-loop", 1_800, 0)
}

/// A script: it makes one recording, the same every time.
pub type Script = fn() -> Recording;

/// Every committed recording: its name and the script that makes it.
pub const RECORDINGS: [(&str, Script); 1] = [("first-loop", first_loop)];

/// The tick whose shared bytes each recording's word-size vector pins.
pub const VECTOR_TICK: u32 = 900;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_seed_is_the_mark_then_the_name() {
        let seed = test_seed("first-loop");
        assert_eq!(seed[..18], *b"TESTSEEDfirst-loop");
        assert!(seed[18..].iter().all(|byte| *byte == 0));
        // A long name is cut, never an overflow.
        assert_eq!(test_seed(&"n".repeat(40))[8..], [b'n'; 24]);
    }

    /// The script holds what its text says it holds: checked on the steps themselves.
    #[test]
    fn the_first_loop_holds_what_its_description_says() {
        let recording = first_loop();
        let steps = &recording.steps;
        assert_eq!(steps.len(), 1_800);
        assert!(recording.encode().is_ok());
        // Every direction and a pause are held at some step, and so are Shift and Space.
        for keys in DIRECTIONS.into_iter().chain([0]) {
            let seen = steps.iter().any(|step| step.held & 0xF == keys);
            assert!(seen, "move keys {keys:#b}");
        }
        for button in [SHIFT, SPACE] {
            assert!(steps.iter().any(|step| step.held & button != 0));
        }
        // Post numbers repeat (a slow page) and skip (a fast one), and never go back.
        let posts = || {
            steps
                .windows(2)
                .map(|pair| pair[1].post.wrapping_sub(pair[0].post))
        };
        assert!(posts().any(|gap| gap == 0) && posts().any(|gap| gap > 1));
        assert!(posts().all(|gap| gap <= 3));
        // Space's count moves by 2 and by 6 or more in one step somewhere, and wraps past 255.
        let space = |step: &Step| step.presses[1].to_le_bytes()[0];
        let jumps = || {
            steps
                .windows(2)
                .map(|pair| space(&pair[1]).wrapping_sub(space(&pair[0])))
        };
        assert!(jumps().any(|by| by == 2) && jumps().any(|by| by >= 6));
        assert!(
            steps
                .windows(2)
                .any(|pair| space(&pair[1]) < space(&pair[0]))
        );
        // Host times: a tick apart on average, jittered, never backwards.
        let gaps = || {
            steps
                .windows(2)
                .map(|pair| pair[1].host_ms - pair[0].host_ms)
        };
        assert!(gaps().all(|gap| gap > 27.0 && gap < 40.0));
        assert!(gaps().any(|gap| gap < 30.0) && gaps().any(|gap| gap > 36.0));
        // Another salt is another stream of the same kind.
        let other = looped("first-loop", 1_800, 3);
        assert_ne!(other.steps, recording.steps);
        assert_eq!(other.header, recording.header);
    }
}
