//! The replay runner (ADR 0002 part 8; ADR 0004 part 9): it makes a recording's calls again, through the entry
//! points the players' file has (`Sim::create`, `rebase`, then per step `input` and `step`), and reads the state's
//! hashes after every step. It is plain Rust over `sim_wasm::Sim`, so the same code runs natively and inside the
//! test-only WebAssembly module; on the players' file itself the same calls are made from JavaScript, which can
//! read the shared hash only.

use std::fmt;

use sim::{StateSink, World};
use sim_wasm::{MODE_LOCAL, Sim};

use crate::recording::{Recording, Step, TEST_SEED_MARK, TickHashes};

/// Why a recording is not replayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    /// The recording holds something this build's entry points cannot take. It is refused whole: nothing that was
    /// recorded is ever silently left out of a replay.
    Cannot(&'static str),
    /// The match was not created.
    Create(String),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cannot(what) => write!(f, "this build cannot replay this recording's {what}"),
            Self::Create(why) => write!(f, "the match was not created: {why}"),
        }
    }
}

/// What this build's entry points cannot take yet, checked before the first call.
fn check(recording: &Recording) -> Result<(), ReplayError> {
    let header = &recording.header;
    let cannot = [
        (
            !header.seed.starts_with(&TEST_SEED_MARK),
            "seed: a replay takes a public test seed only",
        ),
        (
            header.start_tick != 0,
            "start tick: a match starts at tick 0",
        ),
        (
            header.first_sequence != 0,
            "first command sequence: a match starts at sequence 0",
        ),
        (
            header.content_hash != [0; 32] || !header.map_id.is_empty(),
            "content: this build takes none",
        ),
        (
            recording.steps.iter().any(|step| step.movement != 0),
            "move: the input call does not take it yet",
        ),
        (
            recording
                .steps
                .iter()
                .any(|step| step.ray_origin != [0; 3] || step.ray_direction != [0; 3]),
            "aim: the input call does not take it yet",
        ),
        (
            recording
                .steps
                .iter()
                .any(|step| step.view_tick != 0 || step.view_fraction != 0),
            "view time: the input call does not take it yet",
        ),
    ];
    match cannot.into_iter().find(|(found, _)| *found) {
        Some((_, what)) => Err(ReplayError::Cannot(what)),
        None => Ok(()),
    }
}

/// One replay under way: each `next` makes one step's calls and gives the hashes after it.
pub struct Replay<'a> {
    sim: Sim,
    steps: std::slice::Iter<'a, Step>,
}

impl<'a> Replay<'a> {
    /// Creates the match from the recording's seed and drops the time before its first step.
    ///
    /// # Errors
    /// A recording this build cannot replay, by the name of what it cannot take.
    pub fn new(recording: &'a Recording) -> Result<Self, ReplayError> {
        check(recording)?;
        let mut sim = Sim::create(&recording.header.seed, MODE_LOCAL, 0)
            .map_err(|error| ReplayError::Create(error.message()))?;
        if let Some(first) = recording.steps.first() {
            sim.rebase(first.host_ms);
        }
        Ok(Self {
            sim,
            steps: recording.steps.iter(),
        })
    }

    /// The match as it stands.
    #[must_use]
    pub const fn sim(&self) -> &Sim {
        &self.sim
    }

    fn world(&self) -> &World {
        self.sim.local().room().world()
    }
}

impl Iterator for Replay<'_> {
    type Item = TickHashes;

    fn next(&mut self) -> Option<TickHashes> {
        let step = self.steps.next()?;
        let [p0, p1, p2, p3] = step.presses;
        self.sim.input(step.post, step.held, p0, p1, p2, p3);
        self.sim.step(step.host_ms);
        let shared = self.sim.shared_hash();
        let world = self.sim.local().room().world();
        Some(TickHashes {
            tick: world.tick(),
            shared,
            whole: world.state_hash(),
        })
    }
}

/// Replays the whole recording: the hashes after each step.
///
/// # Errors
/// As `Replay::new`.
pub fn replay(recording: &Recording) -> Result<Vec<TickHashes>, ReplayError> {
    Ok(Replay::new(recording)?.collect())
}

/// A reader of the state that writes one `name = value` line per field. The seed's bytes are never written.
struct Lines(String);

impl Lines {
    fn line(&mut self, name: &str, value: fmt::Arguments<'_>) {
        use fmt::Write;
        // Writing into a `String` cannot fail.
        let _ = writeln!(self.0, "{name} = {value}");
    }
}

impl StateSink for Lines {
    fn u16(&mut self, name: &'static str, value: u16) {
        self.line(name, format_args!("{value}"));
    }

    fn u32(&mut self, name: &'static str, value: u32) {
        self.line(name, format_args!("{value}"));
    }

    fn bytes<const N: usize>(&mut self, name: &'static str, value: &[u8; N]) {
        self.line(name, format_args!("{value:?}"));
    }

    fn authority(&mut self) {
        self.0.push_str("-- the authority's part --\n");
    }

    fn secret<const N: usize>(&mut self, name: &'static str, _: &[u8; N]) {
        self.line(name, format_args!("({N} bytes, never shown)"));
    }
}

/// The state as lines, one field each, in the state's own order.
#[must_use]
pub fn state_lines(world: &World) -> String {
    let mut lines = Lines(String::new());
    world.visit(&mut lines);
    lines.0
}

/// Replays until the world is at `tick` and hands it to `read`; `None` when the recording never reaches that tick.
fn at<T>(
    recording: &Recording,
    tick: u32,
    read: impl Fn(&World) -> T,
) -> Result<Option<T>, ReplayError> {
    let mut replay = Replay::new(recording)?;
    loop {
        if replay.world().tick() == tick {
            return Ok(Some(read(replay.world())));
        }
        if replay.next().is_none() {
            return Ok(None);
        }
    }
}

/// The state's lines when the world is at `tick` (0: before the first step).
///
/// # Errors
/// As `Replay::new`.
pub fn state_at(recording: &Recording, tick: u32) -> Result<Option<String>, ReplayError> {
    at(recording, tick, state_lines)
}

/// The shared state's canonical bytes when the world is at `tick`: what the word-size vector pins.
///
/// # Errors
/// As `Replay::new`.
pub fn shared_bytes_at(recording: &Recording, tick: u32) -> Result<Option<Vec<u8>>, ReplayError> {
    at(recording, tick, |world| {
        let mut bytes = Vec::new();
        world.write_shared_state(&mut bytes);
        bytes
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::{Column, field_diff, first_difference};
    use crate::recording::Header;
    use crate::scripts::test_seed;

    /// Twelve steps: button 0 held on the even ones, and pressed once on each.
    fn small(name: &str) -> Recording {
        let step = |index: u32| Step {
            post: index + 1,
            held: (index + 1) % 2,
            presses: [index + 1, 0, 0, 0],
            ..Step::at(f64::from(index) * 33.0)
        };
        Recording {
            header: Header {
                seed: test_seed(name),
                start_tick: 0,
                first_sequence: 0,
                tick_rate: 30,
                content_hash: [0; 32],
                map_id: String::new(),
            },
            steps: (0..12).map(step).collect(),
        }
    }

    #[test]
    fn a_replay_gives_the_same_hashes_every_time_and_the_seed_only_in_the_whole_column() {
        let (a, b) = (small("a"), small("b"));
        let first = replay(&a).expect("it replays");
        assert_eq!(first.len(), 12);
        assert!((1u32..).zip(&first).all(|(tick, line)| line.tick == tick));
        assert_eq!(replay(&a).as_ref(), Ok(&first));
        // Another seed: the same shared hashes, other whole hashes, from the first step on.
        let other = replay(&b).expect("it replays");
        assert_eq!(first_difference(&first, &other, Column::Shared), None);
        let whole = first_difference(&first, &other, Column::Whole).expect("the seeds differ");
        assert_eq!(whole.step, 1);
        // Step by step it is the same run, and the room refused nothing.
        let mut stepped = Replay::new(&a).expect("it replays");
        assert_eq!(stepped.next(), Some(first[0]));
        assert_eq!(stepped.by_ref().last(), Some(first[11]));
        // The first step carries the join, so the first command runs in the second.
        assert_eq!(stepped.sim().local().room().counters().refused(), 0);
        assert_eq!(stepped.sim().local().room().counters().commands_run, 11);
    }

    #[test]
    fn what_this_build_cannot_replay_is_refused_by_name_before_any_call() {
        let refused = |change: &dyn Fn(&mut Recording)| {
            let mut recording = small("a");
            change(&mut recording);
            Replay::new(&recording).err().map(|error| error.to_string())
        };
        let cannot = |what: &str| Some(format!("this build cannot replay this recording's {what}"));
        assert_eq!(
            refused(&|r| r.header.start_tick = 1),
            cannot("start tick: a match starts at tick 0")
        );
        assert_eq!(
            refused(&|r| r.header.first_sequence = 1),
            cannot("first command sequence: a match starts at sequence 0")
        );
        let content = cannot("content: this build takes none");
        assert_eq!(refused(&|r| r.header.content_hash[31] = 1), content);
        assert_eq!(refused(&|r| r.header.map_id = "yard".to_owned()), content);
        let aim = cannot("aim: the input call does not take it yet");
        assert_eq!(
            refused(&|r| r.steps[11].movement = 1),
            cannot("move: the input call does not take it yet")
        );
        assert_eq!(refused(&|r| r.steps[0].ray_origin[2] = 1), aim);
        assert_eq!(refused(&|r| r.steps[5].ray_direction[0] = 1), aim);
        let view = cannot("view time: the input call does not take it yet");
        assert_eq!(refused(&|r| r.steps[3].view_tick = 1), view);
        assert_eq!(refused(&|r| r.steps[3].view_fraction = 1), view);
        // A recording built in memory with a seed that is no test seed: refused like the bytes of one.
        assert_eq!(
            refused(&|r| r.header.seed = [7; 32]),
            cannot("seed: a replay takes a public test seed only")
        );
        assert_eq!(refused(&|_| ()), None);
    }

    /// The live host drops time when it starts, after a stall and on resume (`rebase`), and a recording holds none
    /// of those calls. That is sound only while a rebase moves the schedule and nothing of the state: with one
    /// before every step, every hash is the same.
    #[test]
    fn a_rebase_before_every_step_changes_no_hash() {
        let recording = small("a");
        let plain = replay(&recording).expect("it replays");
        let mut rebased = Replay::new(&recording).expect("it replays");
        let mut got = Vec::new();
        for step in &recording.steps {
            rebased.sim.rebase(step.host_ms + 5_000.0);
            got.extend(rebased.next());
        }
        assert_eq!(got, plain);
    }

    #[test]
    fn the_state_s_lines_name_every_field_and_never_show_the_seed() {
        let recording = small("a");
        let lines = state_at(&recording, 3)
            .expect("it replays")
            .expect("tick 3 is reached");
        // Two commands have run by tick 3 (the first step carried the join), each with one press of button 0.
        assert_eq!(
            lines,
            "tick = 3\nplayer.buttons = 1\nplayer.presses = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]\n\
             -- the authority's part --\nseed = (32 bytes, never shown)\n"
        );
        // No byte of the seed is in the text: not as letters, not as numbers.
        let seed_as_numbers = format!("{:?}", &TEST_SEED_MARK[..]);
        assert!(
            !lines.contains("TESTSEED")
                && !lines.contains(seed_as_numbers.trim_matches(['[', ']']))
        );
        // Before the first step, and past the last.
        let start = state_at(&recording, 0)
            .expect("it replays")
            .expect("the start");
        assert!(start.starts_with("tick = 0\nplayer.buttons = 0\n"));
        assert_eq!(state_at(&recording, 13), Ok(None));
        // The shared bytes: the tick, the buttons, the sixteen counts.
        let bytes = shared_bytes_at(&recording, 3)
            .expect("it replays")
            .expect("tick 3 is reached");
        assert_eq!(bytes, [&[3, 0, 0, 0, 1, 0, 2][..], &[0; 15]].concat());
        // Two runs that differ in one held button differ in that field, by name.
        let mut other = small("a");
        other.steps[2].held = 3;
        let got = state_at(&other, 3)
            .expect("it replays")
            .expect("tick 3 is reached");
        assert_eq!(
            field_diff(&lines, &got),
            ["player.buttons: expected 1, got 3"]
        );
    }
}
