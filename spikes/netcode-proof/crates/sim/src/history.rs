//! The ring of past hit volumes (u4-build-plan.md §4.2, §4.9; the advisor's Q2 ruling).

use crate::seam::HitVolumes;

/// The ring's bound in bytes: 1 MB (plan.md:459, E11), as 10^6 B, the megabyte PM-0001-7 judges (the advisor's
/// W1-U4D T-M2).
const MAX_RING_BYTES: usize = 1_000_000;

/// A fixed ring of [`HitVolumes`], one entry per tick, oldest evicted. The room owns its instance, pushes
/// `world.hit_volumes()` after each step and lends it to the next; the replay driver does the same, so a recording
/// needs no volumes. Its span is set at construction in milliseconds (U4h's test cap plus its margin:
/// u4-build-plan.md §4.9), turned into ticks by [`crate::ticks_for_ms`] alone, so no tick count is ever written for it
/// (A5); it never changes.
#[derive(Clone, Debug)]
pub struct HitHistory {
    /// The entries, allocated once; until the ring is full they are in push order.
    slots: Vec<HitVolumes>,
    /// Where the next push goes once the ring is full: the oldest entry.
    next: usize,
    capacity: usize,
}

impl HitHistory {
    /// An empty ring covering `span_ms` milliseconds: [`crate::ticks_for_ms`]`(span_ms)` ticks, rounded up, so any span
    /// above 0 holds at least one tick. `None` for a span of 0 ms, which would hold none, and for a span whose ring
    /// would exceed 1 MB (10^6 B, as PM-0001-7 judges it): capacity × [`HitVolumes::MAX_BYTES`], one tick's volumes
    /// at most (u4-build-plan.md §4.9: the hit history at most 1 MB; carry 14). At 30 Hz that is 220 ticks, a span of
    /// at most 7,333 ms.
    pub fn for_ms(span_ms: u32) -> Option<HitHistory> {
        let capacity = crate::ticks_for_ms(span_ms) as usize;
        if capacity == 0 || capacity > MAX_RING_BYTES / HitVolumes::MAX_BYTES {
            return None;
        }
        Some(HitHistory {
            slots: Vec::with_capacity(capacity),
            next: 0,
            capacity,
        })
    }

    /// The ticks the ring holds when full.
    pub fn capacity(&self) -> u32 {
        self.capacity as u32
    }

    /// The ticks it holds now.
    pub fn len(&self) -> u32 {
        self.slots.len() as u32
    }

    /// Whether it holds no tick yet.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Copies one tick's volumes in as the newest, evicting the oldest once the ring is full. Ticks come one at a time,
    /// in order. A full ring copies into the evicted entry with `clone_from`, so volumes whose `Clone` implements it
    /// field by field reuse that entry's storage.
    pub fn push(&mut self, volumes: &HitVolumes) {
        debug_assert!(
            self.newest()
                .is_none_or(|v| volumes.tick == v.tick.wrapping_add(1)),
            "hit volumes are pushed one tick at a time, in order"
        );
        if self.slots.len() < self.capacity {
            self.slots.push(volumes.clone());
        } else {
            self.slots[self.next].clone_from(volumes);
        }
        self.next += 1;
        if self.next == self.capacity {
            self.next = 0;
        }
    }

    /// The volumes of `tick`, if the ring holds it.
    pub fn get(&self, tick: u32) -> Option<&HitVolumes> {
        let newest = self.newest_index()?;
        let age = self.slots[newest].tick.wrapping_sub(tick) as usize;
        if age >= self.slots.len() {
            return None;
        }
        let at = if age <= newest {
            newest - age
        } else {
            newest + self.capacity - age
        };
        let volumes = &self.slots[at];
        (volumes.tick == tick).then_some(volumes)
    }

    fn newest(&self) -> Option<&HitVolumes> {
        self.newest_index().map(|i| &self.slots[i])
    }

    fn newest_index(&self) -> Option<usize> {
        if self.slots.is_empty() {
            None
        } else if self.next == 0 {
            Some(self.slots.len() - 1)
        } else {
            Some(self.next - 1)
        }
    }
}
