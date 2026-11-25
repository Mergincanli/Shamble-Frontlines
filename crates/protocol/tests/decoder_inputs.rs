//! "No decoder panics or over-allocates" (ADR 0001's pass mark PM-0001-34), begun: each of the five message types
//! takes 10 million random and 10 million mutated inputs, and its decoder never panics and never takes memory above
//! its caps. The mark names a fuzzing tool that has not joined; until it does, this run starts the mark and does not
//! pass it. The inputs come from a seeded generator of this file's own, so a failure names an input that can be made
//! again, and the memory is counted by a global allocator of this file's own, around the system's.
//!
//! The full count runs where debug assertions are off (`cargo test --release`, which keeps overflow checks on); a
//! debug run takes 100,000 of each kind, and each test's name says which count it ran. Two plants show that the
//! run bites: a decoder with one unchecked index, and one that takes memory for a length before checking it.
//!
//! A test binary of its own, because a program has one global allocator; the counts are kept per thread, because
//! the tests here run side by side.

// A global allocator cannot be written without `unsafe`; the workspace denies it everywhere else, and this file is
// a test that ships nowhere.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;
use std::panic::{AssertUnwindSafe, catch_unwind};

use protocol::codec::{
    Control, FLAG_OWN, KIND_INPUT, KIND_JOIN, KIND_JOIN_ACCEPTED, KIND_JOIN_REJECTED,
    KIND_SNAPSHOT, MAX_INPUT_BYTES, MAX_JOIN_BYTES, MAX_SNAPSHOT_BYTES, decode_control,
    decode_input, decode_snapshot, encode_control, encode_input, encode_snapshot,
};
use protocol::messages::{
    Command, Input, MAX_ACKS, MAX_MODE_ID_BYTES, MAX_TOKEN_BYTES, REDUNDANCY_CAP, SnapshotAck,
};

thread_local! {
    /// Calls on this thread that took memory (`alloc`, `alloc_zeroed`, `realloc`) since the last `reset`, and the
    /// largest size one of them asked for. A constant start value: the counter itself never allocates.
    static TAKEN: Cell<(u64, usize)> = const { Cell::new((0, 0)) };
}

fn count(size: usize) {
    // A thread that is ending may no longer have the counter; such a call is not a test's.
    let _ = TAKEN.try_with(|taken| {
        let (calls, largest) = taken.get();
        taken.set((calls.wrapping_add(1), largest.max(size)));
    });
}

fn reset() {
    TAKEN.set((0, 0));
}

struct Counting;

// SAFETY: every call is passed to the system allocator unchanged, with the caller's own arguments, so its contract
// is the system allocator's; the only thing added is a thread-local pair of numbers.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(layout.size());
        // SAFETY: the caller's contract for `alloc`, passed on.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count(layout.size());
        // SAFETY: the caller's contract for `alloc_zeroed`, passed on.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count(new_size);
        // SAFETY: the caller's contract for `realloc`, passed on.
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the caller's contract for `dealloc`, passed on.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// xorshift64* (Vigna, 2016): this test's own generator, seeded with a fixed number per message type. It has
/// nothing to do with the simulation's random streams.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number below `bound`, by multiply and shift.
    fn below(&mut self, bound: usize) -> usize {
        ((u128::from(self.next()) * bound as u128) >> 64) as usize
    }

    /// A number up to `max` that is often an extreme: 0, `max`, or one beside them.
    fn edgy(&mut self, max: usize) -> usize {
        match self.below(8) {
            0 => 0,
            1 => max,
            2 => 1.min(max),
            3 => max.saturating_sub(1),
            _ => self.below(max + 1),
        }
    }

    /// Adds `count` random bytes to `out`.
    fn bytes(&mut self, count: usize, out: &mut Vec<u8>) {
        let mut word = [0; 8];
        for at in 0..count {
            if at % 8 == 0 {
                word = self.next().to_le_bytes();
            }
            out.push(word[at % 8]);
        }
    }
}

/// A mode id of one of every length class, the longest two exactly at the cap, with characters of one to four
/// bytes.
fn mode_id(rng: &mut Rng) -> String {
    let (digits, accents) = ("0123456789".repeat(7), "é".repeat(30) + "🙂");
    let ids = ["", "a", "skeleton", "é-ß-中-€-🙂", &digits[..64], &accents];
    ids[rng.below(ids.len())].to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Join,
    JoinAccepted,
    JoinRejected,
    Input,
    Snapshot,
}

impl Kind {
    /// The type's kind byte and its longest encoding.
    const fn wire(self) -> (u8, usize) {
        match self {
            Self::Join => (KIND_JOIN, MAX_JOIN_BYTES),
            Self::JoinAccepted => (KIND_JOIN_ACCEPTED, 75 + MAX_MODE_ID_BYTES),
            Self::JoinRejected => (KIND_JOIN_REJECTED, 2),
            Self::Input => (KIND_INPUT, MAX_INPUT_BYTES),
            Self::Snapshot => (KIND_SNAPSHOT, MAX_SNAPSHOT_BYTES),
        }
    }

    /// A random valid message of this type, written by hand from the wire table into `out`: counts, lengths and
    /// ticks at and between their extremes, every other byte random.
    fn valid(self, rng: &mut Rng, out: &mut Vec<u8>) {
        out.clear();
        out.push(self.wire().0);
        match self {
            Self::Join => {
                // Half the tokens are short, so that a mutation often lands on the length or the fields.
                let cap = [16, MAX_TOKEN_BYTES][rng.below(2)];
                let len = rng.edgy(cap);
                rng.bytes(2, out);
                out.extend_from_slice(&(len as u16).to_le_bytes());
                rng.bytes(len + 32, out);
            }
            Self::JoinAccepted => {
                let mode_id = mode_id(rng);
                rng.bytes(3, out);
                out.push(mode_id.len() as u8);
                out.extend_from_slice(mode_id.as_bytes());
                rng.bytes(64 + 4 + 2, out);
            }
            Self::JoinRejected => out.push(1 + rng.below(3) as u8),
            Self::Input => {
                let acks = rng.edgy(MAX_ACKS);
                out.push(acks as u8);
                rng.bytes(5 * acks, out);
                let commands = rng.edgy(REDUNDANCY_CAP);
                out.push(commands as u8);
                // The newest tick leaves room for every older one; sometimes exactly, so the oldest is tick 0.
                let gaps: Vec<u8> = (1..commands).map(|_| 1 + rng.edgy(254) as u8).collect();
                let back: u32 = gaps.iter().map(|gap| u32::from(*gap)).sum();
                let tick = back + rng.edgy((u32::MAX - back) as usize) as u32;
                if commands > 0 {
                    out.extend_from_slice(&tick.to_le_bytes());
                    rng.bytes(2 + 32, out);
                }
                for gap in gaps {
                    out.push(gap);
                    rng.bytes(32, out);
                }
            }
            Self::Snapshot => {
                let tick = rng.edgy(u32::MAX as usize) as u32;
                // No baseline at tick 0: there is no earlier tick to name.
                let flags = rng.below(16) as u8 & if tick == 0 { 14 } else { 15 };
                out.extend_from_slice(&tick.to_le_bytes());
                out.push(flags);
                rng.bytes(1, out);
                if flags & 1 != 0 {
                    let back = 1 + rng.edgy(tick.min(0xFFFF) as usize - 1) as u16;
                    out.extend_from_slice(&back.to_le_bytes());
                }
                rng.bytes(2 * (flags & 6).count_ones() as usize, out);
                rng.bytes(18 * usize::from(flags >> 3), out);
            }
        }
        assert!(out.len() <= self.wire().1, "{self:?}: too long");
    }
}

/// One to three changes to a valid message: a bit flipped, a byte replaced (often by 0, 255 or a small count), the
/// message cut, a byte put in, or bytes added at the end.
fn mutate(rng: &mut Rng, bytes: &mut Vec<u8>) {
    for _ in 0..=rng.below(3) {
        let at = rng.below(bytes.len().max(1));
        match rng.below(6) {
            0 if !bytes.is_empty() => bytes[at] ^= 1 << rng.below(8),
            1 if !bytes.is_empty() => bytes[at] = [0, 255, 1, 4, 5, 8, 9, 16][rng.below(8)],
            2 if !bytes.is_empty() => bytes[at] = rng.next() as u8,
            3 => bytes.truncate(at),
            4 => bytes.insert(at, rng.next() as u8),
            _ => {
                let more = 1 + rng.below(8);
                rng.bytes(more, bytes);
            }
        }
    }
}

/// What a run saw: how many random and mutated inputs it ran, how many of each were taken as messages, and the
/// largest single request for memory any decode made.
#[derive(Debug, Default)]
struct Tally {
    ran: [u64; 2],
    taken: [u64; 2],
    largest: usize,
}

/// What one decode gave: taken or refused, or why the run stops at this input.
type Verdict = Result<bool, String>;

/// The 4,096 valid messages a type's mutated inputs start from, and the generator as it stands after making them.
/// One fixed seed per message type; a failure's input number then names the same bytes on every machine.
fn pool(kind: Kind) -> (Rng, Vec<Vec<u8>>) {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ u64::from(kind.wire().0) << 32);
    let mut bytes = Vec::new();
    let mut valid = || {
        kind.valid(&mut rng, &mut bytes);
        bytes.clone()
    };
    let pool = (0..4096).map(|_| valid()).collect();
    (rng, pool)
}

/// Runs `decode` on `per_half` random inputs and then `per_half` mutated ones of `kind`, each inside a fresh count
/// of the memory it takes, and stops at the first input on which it panics or gives a reason to stop: the error is
/// that input's number and the reason. A random input is random bytes of a random length up to a little over the
/// type's longest message, half of them with the type's kind byte first; a mutated input is one of the pool's
/// valid messages, changed.
fn run(
    kind: Kind,
    per_half: u64,
    mut decode: impl FnMut(&[u8]) -> Verdict,
) -> Result<Tally, (u64, String)> {
    let (kind_byte, longest) = kind.wire();
    let (mut rng, pool) = pool(kind);
    let mut bytes = Vec::with_capacity(2 * longest + 64);
    let mut tally = Tally::default();
    let at = Cell::new(0);
    let ran = catch_unwind(AssertUnwindSafe(|| {
        for input in 0..2 * per_half {
            at.set(input);
            let half = usize::from(input >= per_half);
            bytes.clear();
            if half == 1 {
                bytes.extend_from_slice(&pool[rng.below(pool.len())]);
                mutate(&mut rng, &mut bytes);
            } else {
                let len = rng.below(longest + 9);
                rng.bytes(len, &mut bytes);
                if rng.below(2) == 0 && !bytes.is_empty() {
                    bytes[0] = kind_byte;
                }
            }
            reset();
            let taken = decode(black_box(&bytes))?;
            tally.largest = tally.largest.max(TAKEN.get().1);
            tally.ran[half] += 1;
            tally.taken[half] += u64::from(taken);
        }
        Ok(())
    }));
    match ran {
        Ok(Ok(())) => Ok(tally),
        Ok(Err(why)) => Err((at.get(), why)),
        Err(_) => Err((at.get(), "the decoder panicked".to_owned())),
    }
}

/// A check that stops the run with its reason.
fn hold(holds: bool, why: impl FnOnce() -> String) -> Result<(), String> {
    if holds { Ok(()) } else { Err(why()) }
}

/// The real decoders as the room and the client use them, each decode judged. A control message that is refused
/// takes no memory at all; a `Join` takes its token's bytes once and a `JoinAccepted` its mode id's, each within
/// its cap. The one `Input`, reused for every message, takes memory at most twice in a whole run, each time
/// exactly one list's cap. The snapshot decoder never takes memory. Whatever is taken encodes back to its bytes.
struct Decoders {
    kind: Kind,
    into: Input,
    scratch: Vec<u8>,
    input_calls: u64,
}

impl Decoders {
    fn new(kind: Kind) -> Self {
        let (acks, commands) = (Vec::new(), Vec::new());
        Self {
            kind,
            into: Input { acks, commands },
            scratch: Vec::with_capacity(2 * MAX_JOIN_BYTES),
            input_calls: 0,
        }
    }

    fn decode(&mut self, bytes: &[u8]) -> Verdict {
        // Whether the bytes were taken, and whether what was taken has a wire form again, now in `scratch`.
        let (taken, encoded) = match self.kind {
            Kind::Input => self.input(bytes)?,
            Kind::Snapshot => {
                let snapshot = decode_snapshot(bytes);
                let (calls, largest) = TAKEN.get();
                let why = || format!("a snapshot took memory {calls} times, up to {largest} bytes");
                hold(calls == 0, why)?;
                let encoded = |snapshot| encode_snapshot(&snapshot, &mut self.scratch).is_ok();
                (snapshot.is_ok(), snapshot.is_ok_and(encoded))
            }
            _ => self.control(decode_control(bytes).ok())?,
        };
        let why = || "a message that was taken does not encode back to its bytes".to_owned();
        hold(!taken || (encoded && self.scratch == bytes), why)?;
        Ok(taken)
    }

    fn control(&mut self, control: Option<Control>) -> Result<(bool, bool), String> {
        let (calls, largest) = TAKEN.get();
        let (held, cap) = match &control {
            Some(Control::Join(join)) => (join.token.len(), MAX_TOKEN_BYTES),
            Some(Control::JoinAccepted(accepted)) => (accepted.mode_id.len(), MAX_MODE_ID_BYTES),
            _ => (0, 0),
        };
        hold(held <= cap, || {
            format!("a field of {held} bytes is above its cap of {cap}")
        })?;
        let once = calls == u64::from(held > 0) && largest == held;
        hold(once, || {
            format!(
                "a message holding {held} bytes took memory {calls} times, up to {largest} bytes"
            )
        })?;
        let encoded = |control| encode_control(&control, &mut self.scratch).is_ok();
        Ok((control.is_some(), control.is_some_and(encoded)))
    }

    fn input(&mut self, bytes: &[u8]) -> Result<(bool, bool), String> {
        let taken = decode_input(bytes, &mut self.into).is_ok();
        let (calls, largest) = TAKEN.get();
        self.input_calls += calls;
        let caps = [
            MAX_ACKS * size_of::<SnapshotAck>(),
            REDUNDANCY_CAP * size_of::<Command>(),
        ];
        let sized = calls == 0 || caps.contains(&largest);
        hold(sized, || {
            format!("an input took {largest} bytes, which is neither list's cap ({caps:?})")
        })?;
        let calls = self.input_calls;
        hold(calls <= 2, || {
            format!("the reused input took memory {calls} times")
        })?;
        let (acks, commands) = (self.into.acks.len(), self.into.commands.len());
        hold(taken || acks + commands == 0, || {
            "a refused input left something behind".to_owned()
        })?;
        let capped = acks <= MAX_ACKS && commands <= REDUNDANCY_CAP;
        hold(capped, || "an input above a cap was taken".to_owned())?;
        Ok((
            taken,
            taken && encode_input(&self.into, &mut self.scratch).is_ok(),
        ))
    }
}

/// The whole run of one message type on the real decoders, with what the run must show.
fn whole_run(kind: Kind, per_half: u64) {
    let mut decoders = Decoders::new(kind);
    // The pool the mutated inputs start from: each of its messages is taken as it stands.
    for valid in pool(kind).1 {
        reset();
        assert_eq!(decoders.decode(&valid), Ok(true), "{kind:?}: {valid:?}");
    }
    let tally = match run(kind, per_half, |bytes| decoders.decode(bytes)) {
        Ok(tally) => tally,
        Err((input, why)) => panic!("{kind:?}: stopped at input {input}: {why}"),
    };
    assert_eq!(tally.ran, [per_half, per_half]);
    // The run is not empty-handed: of the mutated messages at least one in a thousand is still taken and at least
    // one in a thousand refused, and random bytes are refused nearly always.
    let [random_taken, mutated_taken] = tally.taken;
    let fair = mutated_taken * 1000 >= per_half && (per_half - mutated_taken) * 1000 >= per_half;
    assert!(
        fair,
        "{kind:?}: {mutated_taken} of {per_half} mutated inputs were taken"
    );
    assert!(
        random_taken * 20 < per_half,
        "{kind:?}: random bytes were taken too often"
    );
    let largest = tally.largest;
    let cap = match kind {
        Kind::Join => MAX_TOKEN_BYTES,
        Kind::JoinAccepted => MAX_MODE_ID_BYTES,
        Kind::JoinRejected | Kind::Snapshot => 0,
        Kind::Input => REDUNDANCY_CAP * size_of::<Command>(),
    };
    assert!(
        largest <= cap,
        "{kind:?}: {largest} bytes taken at once, above {cap}"
    );
    println!(
        "{kind:?}: {per_half} random inputs ({random_taken} taken) and {per_half} mutated inputs \
         ({mutated_taken} taken): 0 panics, at most {largest} bytes taken at once (the cap: {cap})"
    );
}

/// Each message type's run, under the name of the count it runs: the full count where debug assertions are off,
/// the short one where they are on.
macro_rules! runs {
    ($($kind:ident: $full:ident, $short:ident;)*) => {$(
        #[cfg(not(debug_assertions))]
        #[test]
        fn $full() {
            whole_run(Kind::$kind, 10_000_000);
        }

        #[cfg(debug_assertions)]
        #[test]
        fn $short() {
            whole_run(Kind::$kind, 100_000);
        }
    )*};
}

runs! {
    Join: join_10_million_random_and_10_million_mutated_inputs, join_100_thousand_random_and_100_thousand_mutated_inputs;
    JoinAccepted: join_accepted_10_million_random_and_10_million_mutated_inputs, join_accepted_100_thousand_random_and_100_thousand_mutated_inputs;
    JoinRejected: join_rejected_10_million_random_and_10_million_mutated_inputs, join_rejected_100_thousand_random_and_100_thousand_mutated_inputs;
    Input: input_10_million_random_and_10_million_mutated_inputs, input_100_thousand_random_and_100_thousand_mutated_inputs;
    Snapshot: snapshot_10_million_random_and_10_million_mutated_inputs, snapshot_100_thousand_random_and_100_thousand_mutated_inputs;
}

/// The control: the counter sees a request for memory, with its size, and sees nothing when nothing is taken.
#[test]
fn the_counting_allocator_sees_each_request_and_its_size() {
    reset();
    let seen = black_box(Vec::<u8>::with_capacity(4096));
    assert_eq!(TAKEN.get(), (1, 4096));
    let mut grown = black_box(Vec::<u8>::with_capacity(16));
    grown.extend_from_slice(black_box(&[7; 5000]));
    let (calls, largest) = TAKEN.get();
    assert!(
        calls == 3 && largest >= 5000,
        "{calls} calls, the largest {largest}"
    );
    drop((seen, grown));
    reset();
    let sum: u32 = black_box([1u8, 2, 3])
        .iter()
        .map(|byte| u32::from(*byte))
        .sum();
    assert_eq!((sum, TAKEN.get()), (6, (0, 0)));
}

/// The plant the mark names: one decoder with an unchecked index. Here the snapshot decoder reads the own part's
/// first counter at its place whenever the flag says the part is there, without asking whether the message is that
/// long. The run finds it, and says at which input.
#[test]
fn a_planted_unchecked_index_is_found_within_the_run() {
    let mut decoders = Decoders::new(Kind::Snapshot);
    let planted = |bytes: &[u8]| {
        if let [KIND_SNAPSHOT, _, _, _, _, flags, ..] = bytes
            && flags & FLAG_OWN != 0
        {
            let own_at = 7 + 2 * (flags & 7).count_ones() as usize;
            black_box(bytes[own_at + 2]);
        }
        decoders.decode(bytes)
    };
    let (input, why) = run(Kind::Snapshot, 100_000, planted).expect_err("the plant is found");
    assert_eq!(why, "the decoder panicked");
    println!("the planted unchecked index: found at input {input}");
}

/// The second plant: the join decoder takes room for the token as soon as it has read the token's length, before it
/// checks the length against the cap or against the bytes that are there. The run finds it, and says at which input.
#[test]
fn a_planted_over_allocation_is_found_within_the_run() {
    let mut decoders = Decoders::new(Kind::Join);
    let planted = |bytes: &[u8]| {
        if let [KIND_JOIN, _, _, low, high, ..] = bytes {
            let claimed = usize::from(u16::from_le_bytes([*low, *high]));
            black_box(Vec::<u8>::with_capacity(claimed));
        }
        decoders.decode(bytes)
    };
    let (input, why) = run(Kind::Join, 100_000, planted).expect_err("the plant is found");
    assert!(why.contains("took memory"), "{why}");
    println!("the planted over-allocation: found at input {input} ({why})");
}
