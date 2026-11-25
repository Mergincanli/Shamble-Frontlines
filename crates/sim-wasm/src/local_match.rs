//! Local play's one step (ADR 0002 part 6): the client endpoint and the local server in one place, talking through
//! the real messages over a transport. Plain Rust with no bindings, so it runs natively in tests exactly as it
//! runs in the Worker, and over any backend of the transport interface.
//!
//! One step is one tick, in the draft's order: the endpoint builds the tick's command from the page's newest post
//! and sends it; the server takes what arrived, runs the tick and sends its snapshot; the endpoint takes the
//! snapshot; the block is written. So the command for a tick reaches the server before the server runs that tick,
//! and a press the page posted before a step is in the server's state, and in the block, after that step.

use client_endpoint::endpoint::{Endpoint, EndpointState};
use client_endpoint::post::Post;
use client_endpoint::schedule::LocalSchedule;
use protocol::codec::MAX_JOIN_BYTES;
use protocol::loopback::Loopback;
use protocol::transport::{Channel, Delivery, SendError, Side, Transport, TransportPair};
use room::Room;
use sim::{MatchSetup, TICK_HZ};

/// The content hash both sides hold while there is no content data; the content crate brings the real one.
pub const NO_CONTENT: [u8; 32] = [0; 32];

/// The loopback of a local match: each slot takes the longest message there is, and one step moves at most two
/// messages each way.
#[must_use]
pub fn loopback() -> Loopback {
    const SLOT_BYTES: u16 = {
        assert!(MAX_JOIN_BYTES <= u16::MAX as usize);
        MAX_JOIN_BYTES as u16
    };
    Loopback::new(SLOT_BYTES, 8)
}

/// One local match: the local server, the client endpoint and the link between them. It owns no state of the
/// exchange itself: the endpoint holds the page's post and writes the block, the room holds the world.
pub struct LocalMatch<P: TransportPair = Loopback> {
    room: Room,
    endpoint: Endpoint,
    link: P,
    schedule: LocalSchedule,
    /// The bytes that arrived and the bytes to send, each reused for every message.
    incoming: Vec<u8>,
    outgoing: Vec<u8>,
    /// Messages the link did not take; wraps.
    unsent: u32,
}

impl<P: TransportPair> LocalMatch<P> {
    /// A match over `link`, with the join already on its way; the endpoint's first command has `first_sequence`
    /// (0 in play). The block is whole from the start, and no tick is due before the first `rebase`. `None` when
    /// `pad_bytes` is more padding than the block writer takes.
    #[must_use]
    pub fn new(setup: &MatchSetup, first_sequence: u16, pad_bytes: u32, link: P) -> Option<Self> {
        let mut local = Self {
            room: Room::new(setup, NO_CONTENT),
            endpoint: Endpoint::new(NO_CONTENT, first_sequence, pad_bytes)?,
            link,
            schedule: LocalSchedule::new(0.0, TICK_HZ),
            incoming: Vec::with_capacity(MAX_JOIN_BYTES),
            outgoing: Vec::with_capacity(MAX_JOIN_BYTES),
            unsent: 0,
        };
        let joined = local.endpoint.join(&mut local.outgoing);
        let mut client = local.link.end(Side::Client);
        let sent = joined
            && client
                .send_reliable(Channel::Control, &local.outgoing)
                .is_ok();
        drop(client);
        local.unsent = u32::from(!sent);
        Some(local)
    }

    #[must_use]
    pub const fn room(&self) -> &Room {
        &self.room
    }

    #[must_use]
    pub const fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    /// Messages the link did not take; wraps. They are in the block's count of refused messages.
    #[must_use]
    pub const fn unsent(&self) -> u32 {
        self.unsent
    }

    /// The block's bytes. Their address does not change for the life of the match.
    #[must_use]
    pub fn block(&self) -> &[u8] {
        self.endpoint.block()
    }

    #[must_use]
    pub const fn block_len(&self) -> u32 {
        self.endpoint.block_len()
    }

    /// The page's newest post, as the host hands it over once per tick. The host keeps the newest by its number;
    /// the match passes on what it is given.
    pub const fn set_post(&mut self, post: Post) {
        self.endpoint.set_post(post);
    }

    /// Runs one tick, writes its block, and returns the time (the host's milliseconds) at which the next tick is
    /// due. `_now_ms` is the host's time now; nothing reads it yet (the link simulator's clock will).
    ///
    /// # Panics
    /// When the local server refuses the join or the endpoint does not take its answer: both sides are one build,
    /// so that is a bug, and the match stops with the reason.
    pub fn step(&mut self, _now_ms: f64) -> f64 {
        let Self {
            room,
            endpoint,
            link,
            incoming,
            outgoing,
            unsent,
            ..
        } = self;
        let mut sent = |result: Result<(), SendError>| {
            *unsent = unsent.wrapping_add(u32::from(result.is_err()));
        };
        if endpoint.build_input(outgoing) {
            sent(link.end(Side::Client).send_unreliable(outgoing));
        }
        {
            let mut server = link.end(Side::Server);
            while let Some(delivery) = server.receive(incoming) {
                match delivery {
                    Delivery::Datagram => room.on_datagram(incoming),
                    Delivery::Reliable(Channel::Control) => {
                        if room.on_control(incoming, outgoing) {
                            sent(server.send_reliable(Channel::Control, outgoing));
                        }
                    }
                }
            }
            if room.step(outgoing) {
                sent(server.send_unreliable(outgoing));
            }
        }
        let mut client = link.end(Side::Client);
        while let Some(delivery) = client.receive(incoming) {
            match delivery {
                Delivery::Datagram => endpoint.on_datagram(incoming),
                Delivery::Reliable(Channel::Control) => {
                    let taken = endpoint.on_control(incoming).is_ok();
                    assert!(
                        taken && endpoint.state() == EndpointState::Joined,
                        "the local server's answer to the join was not an acceptance this build takes"
                    );
                }
            }
        }
        // In local play both sides are here, so the block's count of refused messages is everybody's.
        endpoint.write_block(room.counters().refused().wrapping_add(*unsent));
        self.schedule.ticked();
        self.schedule.next_due_ms()
    }

    /// Drops the time up to `now_ms` (at the start, after a pause, or after a stall longer than one wake may catch
    /// up) and returns when the next tick is due: one tick period later.
    pub fn rebase(&mut self, now_ms: f64) -> f64 {
        self.schedule.rebase(now_ms);
        self.schedule.next_due_ms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::wrap::press_edges;
    use sim::BUTTONS;

    fn local(start_tick: u32, first_sequence: u16) -> LocalMatch {
        let seed = [7; sim::SEED_BYTES];
        let setup = MatchSetup { seed, start_tick };
        LocalMatch::new(&setup, first_sequence, 0, loopback()).expect("no padding is allowed")
    }

    /// A post with these buttons held and button 0 pressed this many times.
    fn post(number: u32, buttons: u16, first_presses: u8) -> Post {
        let mut presses = [0; BUTTONS];
        presses[0] = first_presses;
        Post {
            number,
            buttons,
            presses,
        }
    }

    /// Nothing was refused, lost, doubled, late, repeated or skipped on either side.
    fn clean(local: &LocalMatch) {
        let (room, readout) = (local.room().counters(), local.endpoint().readout());
        let room = [
            room.refused(),
            room.late,
            room.missing,
            room.duplicates,
            room.stale,
            room.queue_full,
        ];
        assert_eq!(room, [0; 6]);
        let endpoint = [
            readout.refused,
            readout.late,
            readout.unsent,
            readout.presses_lost,
            readout.presses_doubled,
        ];
        assert_eq!(endpoint, [0; 5]);
        assert_eq!(local.endpoint().pacer().refused(), (0, 0));
        assert_eq!(local.unsent(), 0);
    }

    #[test]
    fn the_join_crosses_in_the_first_step_and_a_post_reaches_the_server_within_its_step() {
        let mut local = local(40, 0);
        assert_eq!(local.endpoint().state(), EndpointState::Joining);
        local.step(0.0);
        assert_eq!(local.endpoint().state(), EndpointState::Joined);
        let readout = *local.endpoint().readout();
        assert_eq!((readout.tick, readout.sent, readout.received), (41, 0, 1));
        // What the page posted before a step is in the server's state after that step, and in the block: the
        // command for tick 42 reached the server before it ran tick 42.
        local.set_post(post(5, 0b11, 1));
        local.step(1.0);
        let world = local.room().world();
        let server = (
            world.tick(),
            world.player().buttons,
            world.player().presses[0],
        );
        assert_eq!(server, (42, 0b11, 1));
        let readout = *local.endpoint().readout();
        assert_eq!((readout.tick, readout.sent, readout.received), (42, 1, 2));
        let echo = (
            readout.post_used,
            readout.echo_buttons,
            readout.echo_presses[0],
        );
        assert_eq!(echo, (5, 0b11, 1));
        assert_eq!(local.block()[..4], 42u32.to_le_bytes());
        let counters = local.room().counters();
        assert_eq!((counters.joins, counters.commands_run), (1, 1));
        clean(&local);
    }

    /// ADR 0003's milestone, second half: every tick runs on the last post handed over before it. A page faster
    /// than the tick posts two or three times before a step, a slower one not at all: each tick's command is
    /// built from the last one. (Which post is the newest when they arrive out of order is the host's to say:
    /// `packages/sim-worker/src/host.ts` and its test.)
    #[test]
    fn every_tick_runs_the_last_post_handed_over() {
        let mut local = local(0, 0);
        local.step(0.0);
        let mut number = 0u32;
        for tick in 0..1800u32 {
            let posts = if tick % 7 == 3 { 0 } else { 1 + tick % 3 };
            for _ in 0..posts {
                number += 1;
                local.set_post(post(number, (number % 4) as u16, 0));
            }
            local.step(f64::from(tick));
            let readout = local.endpoint().readout();
            let ran = (readout.post_used, readout.echo_buttons);
            assert_eq!(ran, (number, (number % 4) as u16), "tick {tick}");
        }
        assert_eq!(local.room().counters().commands_run, 1800);
        clean(&local);
    }

    /// Ten thousand presses on four buttons, posted twice a tick as a page at 60 frames a second does, some twice
    /// on one button inside one tick: the server applies each exactly once. Counted on the server's own state,
    /// tick by tick, while each button's 8-bit counter wraps about ten times.
    #[test]
    fn ten_thousand_presses_reach_the_server_none_lost_none_doubled() {
        let mut local = local(0, 0);
        local.step(0.0);
        let (mut made, mut applied) = ([0u32; 4], [0u32; 4]);
        let (mut counts, mut before) = ([0u8; BUTTONS], [0u8; BUTTONS]);
        let (mut random, mut number, mut steps) = (0x9E37_79B9_u32, 0u32, 0u32);
        while made.iter().sum::<u32>() < 10_000 || local.endpoint().pacer().waiting() > 0 {
            for _ in 0..2 {
                random = random.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let button = (random >> 30) as usize;
                if made.iter().sum::<u32>() < 10_000 && (random >> 24) & 3 != 0 {
                    made[button] += 1;
                    counts[button] = counts[button].wrapping_add(1);
                }
                number += 1;
                local.set_post(Post {
                    number,
                    buttons: (random >> 8) as u16 & 0xF,
                    presses: counts,
                });
            }
            local.step(f64::from(steps));
            steps += 1;
            let now = local.room().world().player().presses;
            for (applied, (old, new)) in applied.iter_mut().zip(before.iter().zip(now)) {
                *applied += u32::from(press_edges(*old, new));
            }
            before = now;
        }
        assert_eq!(made.iter().sum::<u32>(), 10_000);
        assert_eq!(applied, made);
        assert!(made.iter().all(|made| *made > 2000), "{made:?}");
        clean(&local);
    }

    /// ADR 0001's counter wraps: 600 ticks from three ticks before a multiple of 65,536, with the first command
    /// three before the sequence's wrap, a press every tick. Nothing is refused or lost, and a second run gives
    /// the same state hash at every tick.
    #[test]
    fn the_tick_and_the_sequence_cross_their_wraps_and_two_runs_agree() {
        let run = || {
            let mut local = local(131_069, 65_533);
            let (mut scratch, mut hashes) = (Vec::new(), Vec::new());
            let mut presses = [0u8; BUTTONS];
            for step in 0..600u32 {
                let button = step as usize % BUTTONS;
                presses[button] = presses[button].wrapping_add(1);
                let (number, buttons) = (step, step as u16);
                local.set_post(Post {
                    number,
                    buttons,
                    presses,
                });
                local.step(f64::from(step));
                hashes.push(local.room().world().state_hash(&mut scratch));
            }
            let world = local.room().world();
            assert_eq!((world.tick(), world.player().presses), (131_669, presses));
            assert_eq!(local.room().counters().commands_run, 599);
            clean(&local);
            hashes
        };
        let (first, second) = (run(), run());
        assert_eq!(first, second);
        let mut distinct = first.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(distinct.len(), 600);
    }

    /// The host's time is not read yet: a time that is no number, or one that goes back, changes no tick.
    #[test]
    fn a_step_is_one_tick_whatever_time_the_host_says_and_too_much_padding_is_refused() {
        let mut local = local(0, 0);
        for now_ms in [5.0, 4.0, f64::NAN, 5.0, f64::INFINITY] {
            local.step(now_ms);
        }
        assert_eq!(local.room().world().tick(), 5);
        assert_eq!(local.block()[..4], 5u32.to_le_bytes());
        clean(&local);
        let seed = [7; sim::SEED_BYTES];
        let setup = MatchSetup {
            seed,
            start_tick: 0,
        };
        let pad = client_endpoint::block::MAX_PAD_BYTES + 1;
        assert!(LocalMatch::new(&setup, 0, pad, loopback()).is_none());
    }

    /// A link that takes nothing: nothing panics, the join never arrives, so the room has no player to send to,
    /// and the one message that could not be sent is in the block's count of refused messages.
    #[test]
    fn a_link_that_takes_nothing_is_counted_in_the_block() {
        let seed = [7; sim::SEED_BYTES];
        let setup = MatchSetup {
            seed,
            start_tick: 0,
        };
        // Slots of one byte: no message fits.
        let mut local = LocalMatch::new(&setup, 0, 0, Loopback::new(1, 8)).expect("no padding");
        assert_eq!(local.unsent(), 1);
        for step in 0..10 {
            local.step(f64::from(step));
        }
        assert_eq!(local.endpoint().state(), EndpointState::Joining);
        assert_eq!((local.unsent(), local.room().counters().joins), (1, 0));
        assert_eq!(local.block()[36..40], 1u32.to_le_bytes());
    }
}
