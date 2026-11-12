//! The per-client event queue (u4-build-plan.md §4.7; catalogue §3's gameplay-event row): the relevant gameplay
//! events not yet acknowledged, each stamped with the tick it ran and its 32-bit number, resent until acknowledged,
//! bounded by a cap held as data.

use std::collections::VecDeque;

use npt_sim::SimEvent;

/// One queued event: its 32-bit number, the tick it ran (`TickInput::tick`: carry 23) and the event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Queued {
    pub(crate) number: u32,
    pub(crate) tick: u32,
    pub(crate) event: SimEvent,
    /// How many snapshots have carried it.
    pub(crate) sent: u32,
}

/// The oldest age an event's section can code: 8 bits (snapshot.rs's wire text; carry 50).
pub(crate) const MAX_EVENT_AGE: u32 = u8::MAX as u32;

/// One client's queue of unacknowledged gameplay events (u4-build-plan.md §4.7). Its cap is data: the room's setting,
/// provisional until U4f measures the rate (U4E-Q6 (a): 1,024 from net F121's example). An overflow is the room's
/// signal to force one resync; under PM-0001-22/unbounded (bites builds only) the queue keeps every event past its cap.
#[derive(Clone, Debug)]
pub struct EventQueue {
    cap: u32,
    pending: VecDeque<Queued>,
    unbounded: bool,
    high_water: u32,
}

/// Why [`EventQueue::push`] took no event: the queue was at its cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Overflow;

impl EventQueue {
    /// The provisional cap (U4E-Q6 (a): net F121's example, above the about 400 a B1 burst adds); U4f records §4.7's
    /// value before -22's runs.
    pub(crate) const PROVISIONAL_CAP: u32 = 1_024;

    pub(crate) fn new(cap: u32, unbounded: bool) -> EventQueue {
        EventQueue {
            cap,
            pending: VecDeque::new(),
            unbounded,
            high_water: 0,
        }
    }

    /// Queues one event, oldest first; at the cap it takes none and reports the overflow (but under the -22 bite).
    pub(crate) fn push(&mut self, q: Queued) -> Result<(), Overflow> {
        if self.pending.len() >= self.cap as usize && !self.unbounded {
            return Err(Overflow);
        }
        self.pending.push_back(q);
        self.high_water = self.high_water.max(self.len());
        Ok(())
    }

    /// The events queued now.
    pub(crate) fn len(&self) -> u32 {
        self.pending.len() as u32
    }

    /// The most it has held.
    pub(crate) fn high_water(&self) -> u32 {
        self.high_water
    }

    /// The cap less the length, signed: below 0 only under the -22 bite (SERIES.md's
    /// `tick.v.ack_starved_headroom_count`).
    pub(crate) fn headroom(&self) -> i64 {
        i64::from(self.cap) - i64::from(self.len())
    }

    /// Removes the acknowledged events (each number once; the queue is in number order, wrap-aware).
    pub(crate) fn ack(&mut self, numbers: &[u32]) {
        if numbers.is_empty() {
            return;
        }
        self.pending.retain(|q| !numbers.contains(&q.number));
    }

    /// Drops every event that could no longer be coded at tick `now` (older than 255 ticks: carry 50) and returns
    /// how many.
    pub(crate) fn expire(&mut self, now: u32) -> u32 {
        let before = self.pending.len();
        self.pending
            .retain(|q| now.wrapping_sub(q.tick) <= MAX_EVENT_AGE);
        (before - self.pending.len()) as u32
    }

    /// A full state clears the client's unacknowledged events (catalogue Q8).
    pub(crate) fn clear(&mut self) {
        self.pending.clear();
    }

    /// The queue, oldest first.
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut Queued> {
        self.pending.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(n: u32, tick: u32) -> Queued {
        Queued {
            number: n,
            tick,
            event: SimEvent::Kill {
                killer: 1,
                victim: 2,
            },
            sent: 0,
        }
    }

    #[test]
    fn the_cap_refuses_at_its_value_and_the_headroom_falls_to_zero_never_below() {
        let mut q = EventQueue::new(3, false);
        for n in 0..3 {
            assert_eq!(q.push(hit(n, 10)), Ok(()));
        }
        assert_eq!(q.headroom(), 0);
        assert_eq!(q.push(hit(3, 10)), Err(Overflow));
        assert_eq!((q.len(), q.high_water(), q.headroom()), (3, 3, 0));
        q.ack(&[1]);
        assert_eq!(q.len(), 2);
        q.clear();
        assert_eq!((q.len(), q.high_water()), (0, 3));
    }

    /// Carry 50's edge: an event at age 255 is kept (it can still be sent), none at 256.
    #[test]
    fn carry_50_an_event_at_age_255_is_kept_and_at_256_dropped_and_counted() {
        let mut q = EventQueue::new(8, false);
        q.push(hit(0, 1_000)).expect("room");
        assert_eq!(q.expire(1_000 + 255), 0);
        assert_eq!(q.len(), 1);
        assert_eq!(q.expire(1_000 + 256), 1);
        assert_eq!(q.len(), 0);
    }
}
