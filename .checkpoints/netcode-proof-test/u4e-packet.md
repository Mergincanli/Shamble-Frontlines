# Review packet: U4e round 1 (the room and the codec path: npt-room, npt-server's library, npt-client; with L3's `Scene::effect_rows`)

_Cut by `review_packet.py` from `u4e-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Base: 1e8c188 on the spike branch, the commit before L3's carry-51 commit 28a85e7 (the advisor's U4e brief ruling, L3: the review reads its diff from the commit before the carry-51 commit). The build's `<base>` is 28a85e7; the worktree's HEAD is 97bb9d3, whose spikes/ equals 28a85e7's.
- U4e's copy: `.claude/worktrees/wf_0fa6595a-3e2-1` (branch worktree-wf_0fa6595a-3e2-1 at 97bb9d3; the change is uncommitted there). The same change as a patch: `.checkpoints/netcode-proof-test/wip/u4e.patch` (committed at db49108).
- The diffs below compare the base copy `.gate/u4e-prefix` (a worktree at 1e8c188) with U4e's copy; new test folders are compared with the empty folder `.gate/u4e-empty`. The tests' diffs are in the side file `.checkpoints/netcode-proof-test/u4e-packet-tests.md`.
- The build's runs are under `.gate/u4e/` (logs written by the runner `.checkpoints/netcode-proof-test/u4e-run.sh`): the two counted check.mjs runs b11-counted1.log and b11-counted2.log, the `--list` b11-list.log, and the scratch-switch runs s01 to s15.

## What changed (the diffs)

### npt-sim's scene.rs (L3's commit: `Scene::effect_rows`)
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/sim/src/scene.rs .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/sim/src/scene.rs`:  1 file changed, 7 insertions(+)._

````diff
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/sim/src/scene.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/sim/src/scene.rs
index 4ed34ba..f43dcf3 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/sim/src/scene.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/sim/src/scene.rs
@@ -318,6 +318,13 @@ impl Scene {
         self.health
     }
 
+    /// Effect rows per soldier: 8 in the stress scene, 0 in the others. The World holds `slots() * effect_rows()` of
+    /// them, and effect row `i` belongs to soldier slot `i / effect_rows()`. This is that mapping's one public home
+    /// (carry 51): a crate that needs an effect row's owner reads it here and never copies the 8.
+    pub fn effect_rows(&self) -> u8 {
+        self.effect_rows
+    }
+
     /// The same scene with other CPU stand-ins (`StandIns::OFF` on L1: §4.13). The state is the same either way.
     pub fn with_stand_ins(self, stand_ins: StandIns) -> Scene {
         Scene { stand_ins, ..self }
````

### npt-room's sources (crates/room/src)
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/room/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src`:  9 files changed, 2987 insertions(+)._

````diff
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/ack.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/ack.rs
new file mode 100644
index 0000000..b93e400
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/ack.rs
@@ -0,0 +1,292 @@
+//! The AckRing (u4-build-plan.md §4.5; plan §4.7): per client, every snapshot sent in the last
+//! `npt_sim::ACK_RING_TICKS` ticks with what its groups acknowledged, and the client's view after each, the baseline a
+//! later snapshot is coded against.
+
+use npt_proto::Wrap;
+use npt_sim::{EntityKind, EntityRecord};
+
+/// A client's view at one tick as the room means it to hold it after decoding that tick: the entity set and the
+/// own-state section, each sorted by (kind, index), every record as the wire carries it (`area` and `raw` 0).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct Frame {
+    pub(crate) entities: Vec<EntityRecord>,
+    pub(crate) own: Vec<EntityRecord>,
+}
+
+/// The sort and lookup key of a frame's records.
+pub(crate) fn key(r: &EntityRecord) -> (EntityKind, u16) {
+    (r.kind, r.index)
+}
+
+/// The record of (kind, index) in a sorted list.
+pub(crate) fn find(list: &[EntityRecord], k: (EntityKind, u16)) -> Option<&EntityRecord> {
+    list.binary_search_by_key(&k, key).ok().map(|i| &list[i])
+}
+
+/// One sent tick: its group count, the groups acknowledged, the view it gives, and each group's gameplay event numbers.
+#[derive(Clone, Debug, Default)]
+pub(crate) struct Sent {
+    pub(crate) tick: u32,
+    pub(crate) valid: bool,
+    pub(crate) groups: u8,
+    pub(crate) acked: u8,
+    pub(crate) frame: Frame,
+    /// (group, event number) for each gameplay event the tick carried.
+    pub(crate) events: Vec<(u8, u32)>,
+}
+
+impl Sent {
+    fn full_mask(&self) -> u8 {
+        ((1u16 << self.groups) - 1) as u8
+    }
+
+    fn complete(&self) -> bool {
+        self.valid && self.groups > 0 && self.acked == self.full_mask()
+    }
+}
+
+/// What an acknowledgement named (the room's check "the acknowledgement names a sent snapshot": catalogue :87).
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) enum Acked {
+    /// A snapshot (or full state) the ring holds; its groups were marked.
+    Held,
+    /// A tick inside the span the room has sent, older than the ring: sent, and too old to be a baseline.
+    Old,
+    /// No snapshot sent to this client has that tick.
+    NeverSent,
+}
+
+/// Per client, the ticks sent in the last `npt_sim::ACK_RING_TICKS` ticks (u4-build-plan.md §4.5): ages 0 to 31 kept
+/// at 30 Hz, age 32 evicted (age = the tick being coded less the sent tick). Baseline B is the newest sent tick whose
+/// every group the client acknowledged; a partly acknowledged tick is never a baseline, and a B older than the ring
+/// means coding against the per-type defaults.
+#[derive(Clone, Debug)]
+pub struct AckRing {
+    entries: Vec<Sent>,
+    newest: Option<u32>,
+    first: Option<u32>,
+    last_acked: Option<u32>,
+    /// Under PM-0001-18/no-fallback (bites builds only): the last acknowledged view, kept when its entry is evicted.
+    keep_stale: bool,
+    stale: Option<Sent>,
+}
+
+impl AckRing {
+    /// The ring's length in ticks, `npt_sim::ACK_RING_TICKS` by path (32 at 30 Hz).
+    pub(crate) fn len() -> u32 {
+        npt_sim::ACK_RING_TICKS
+    }
+
+    pub(crate) fn new() -> AckRing {
+        AckRing {
+            entries: vec![Sent::default(); AckRing::len() as usize],
+            newest: None,
+            first: None,
+            last_acked: None,
+            keep_stale: false,
+            stale: None,
+        }
+    }
+
+    /// Keep the last acknowledged view past its eviction (the -18 bite's encoder codes against it).
+    pub(crate) fn keep_stale(&mut self) {
+        self.keep_stale = true;
+    }
+
+    fn slot(tick: u32) -> usize {
+        (tick % AckRing::len()) as usize
+    }
+
+    /// Records a sent tick (a snapshot's groups, or a full state as one group): its group count, the view it gives
+    /// and its gameplay events.
+    pub(crate) fn record(&mut self, tick: u32, groups: u8, frame: Frame, events: Vec<(u8, u32)>) {
+        let i = AckRing::slot(tick);
+        let old = &mut self.entries[i];
+        if self.keep_stale && old.valid && Some(old.tick) == self.last_acked {
+            self.stale = Some(core::mem::take(old));
+        }
+        self.entries[i] = Sent {
+            tick,
+            valid: true,
+            groups,
+            acked: 0,
+            frame,
+            events,
+        };
+        let w = Wrap::default();
+        if self.newest.is_none_or(|n| w.after32("tick", n, tick)) {
+            self.newest = Some(tick);
+        }
+        if self.first.is_none() {
+            self.first = Some(tick);
+        }
+    }
+
+    /// The newest tick sent, if any.
+    pub(crate) fn newest(&self) -> Option<u32> {
+        self.newest
+    }
+
+    /// The entry the ring holds for `tick`, if any.
+    fn held(&self, tick: u32) -> Option<&Sent> {
+        let e = &self.entries[AckRing::slot(tick)];
+        let newest = self.newest?;
+        (e.valid && e.tick == tick && newest.wrapping_sub(tick) < AckRing::len()).then_some(e)
+    }
+
+    /// Marks `mask`'s groups of `tick` acknowledged, pushing the gameplay event numbers they carried onto `events`.
+    pub(crate) fn ack(&mut self, tick: u32, mask: u8, events: &mut Vec<u32>) -> Acked {
+        let w = Wrap::default();
+        if self.held(tick).is_some() {
+            let e = &mut self.entries[AckRing::slot(tick)];
+            let new = mask & e.full_mask() & !e.acked;
+            e.acked |= new;
+            for &(g, n) in &e.events {
+                if new & (1 << g) != 0 {
+                    events.push(n);
+                }
+            }
+            if e.complete() && self.last_acked.is_none_or(|l| w.after32("tick", l, tick)) {
+                self.last_acked = Some(tick);
+            }
+            return Acked::Held;
+        }
+        match (self.first, self.newest) {
+            (Some(f), Some(n)) if !w.after32("tick", tick, f) && !w.after32("tick", n, tick) => {
+                // Inside the sent span: older than the ring (a tick in the ring's span it lacks was never sent).
+                if n.wrapping_sub(tick) >= AckRing::len() {
+                    Acked::Old
+                } else {
+                    Acked::NeverSent
+                }
+            }
+            _ => Acked::NeverSent,
+        }
+    }
+
+    /// Whether the ring holds `tick` with every group acknowledged.
+    pub(crate) fn acked(&self, tick: u32) -> bool {
+        self.held(tick).is_some_and(Sent::complete)
+    }
+
+    /// Baseline B for coding tick `now`: the newest sent tick whose every group was acknowledged, at age 1 to
+    /// `len() - 1` (age `len()` is evicted). `None`: code against the per-type defaults.
+    pub(crate) fn baseline(&self, now: u32) -> Option<&Sent> {
+        let mut best: Option<&Sent> = None;
+        for e in &self.entries {
+            let age = now.wrapping_sub(e.tick);
+            if !e.complete() || age == 0 || age >= AckRing::len() {
+                continue;
+            }
+            if best.is_none_or(|b| now.wrapping_sub(b.tick) > age) {
+                best = Some(e);
+            }
+        }
+        best
+    }
+
+    /// The newest tick ever fully acknowledged (the last baseline the client confirmed), held or evicted.
+    pub(crate) fn last_acked(&self) -> Option<u32> {
+        self.last_acked
+    }
+
+    /// Whether the last acknowledged baseline has left the ring at tick `now` (u4-build-plan.md §4.5's
+    /// `event.burst_eviction`, read at the first tick after a burst).
+    pub(crate) fn evicted(&self, now: u32) -> bool {
+        self.last_acked
+            .is_some_and(|l| now.wrapping_sub(l) >= AckRing::len())
+    }
+
+    /// The last acknowledged view past its eviction (kept only under the -18 bite).
+    pub(crate) fn stale(&self) -> Option<&Sent> {
+        self.stale
+            .as_ref()
+            .filter(|s| Some(s.tick) == self.last_acked)
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    fn sent(ring: &mut AckRing, tick: u32, groups: u8) {
+        ring.record(tick, groups, Frame::default(), vec![(0, tick)]);
+    }
+
+    /// e4: the baseline rule and the ring's age edges (T15; u4-build-plan.md §4.5): age 31 kept, age 32 evicted; a
+    /// partly acknowledged tick is never a baseline.
+    #[test]
+    fn e4_the_baseline_is_the_newest_fully_acked_tick_age_31_kept_age_32_evicted() {
+        let len = AckRing::len();
+        assert_eq!(len, npt_sim::ack_ring_ticks_at(npt_sim::TICK_HZ));
+        let start = 1_000u32;
+        let mut ring = AckRing::new();
+        let mut events = Vec::new();
+        sent(&mut ring, start, 2);
+        // Group 0 alone: partly acknowledged, never a baseline.
+        assert_eq!(ring.ack(start, 0b01, &mut events), Acked::Held);
+        assert!(ring.baseline(start + 1).is_none());
+        assert_eq!(ring.ack(start, 0b10, &mut events), Acked::Held);
+        assert_eq!(ring.baseline(start + 1).map(|s| s.tick), Some(start));
+        // Age 31 kept, age 32 evicted.
+        assert_eq!(ring.baseline(start + len - 1).map(|s| s.tick), Some(start));
+        assert!(!ring.evicted(start + len - 1));
+        assert!(ring.baseline(start + len).is_none());
+        assert!(ring.evicted(start + len));
+        // A newer partly acknowledged tick never displaces the older complete one.
+        sent(&mut ring, start + 3, 3);
+        ring.ack(start + 3, 0b011, &mut events);
+        assert_eq!(ring.baseline(start + 4).map(|s| s.tick), Some(start));
+        ring.ack(start + 3, 0b100, &mut events);
+        assert_eq!(ring.baseline(start + 4).map(|s| s.tick), Some(start + 3));
+        // Each group's events come back once, when that group is first acknowledged.
+        assert_eq!(events, [start, start + 3]);
+    }
+
+    #[test]
+    fn an_ack_names_a_held_tick_an_old_sent_one_or_one_never_sent() {
+        let len = AckRing::len();
+        let mut ring = AckRing::new();
+        let mut events = Vec::new();
+        assert_eq!(ring.ack(5, 1, &mut events), Acked::NeverSent);
+        for t in 100..100 + 2 * len {
+            if t != 100 + 2 * len - 3 {
+                sent(&mut ring, t, 1);
+            }
+        }
+        let newest = 100 + 2 * len - 1;
+        assert_eq!(ring.ack(newest, 1, &mut events), Acked::Held);
+        assert_eq!(
+            ring.ack(newest - 2, 1, &mut events),
+            Acked::NeverSent,
+            "a skipped tick in the ring"
+        );
+        assert_eq!(ring.ack(100, 1, &mut events), Acked::Old);
+        assert_eq!(
+            ring.ack(99, 1, &mut events),
+            Acked::NeverSent,
+            "before the first"
+        );
+        assert_eq!(
+            ring.ack(newest + 1, 1, &mut events),
+            Acked::NeverSent,
+            "ahead of the newest"
+        );
+    }
+
+    #[test]
+    fn the_ring_works_across_the_32_bit_tick_wrap() {
+        let mut ring = AckRing::new();
+        let mut events = Vec::new();
+        let start = u32::MAX - 2;
+        for k in 0..6u32 {
+            sent(&mut ring, start.wrapping_add(k), 1);
+        }
+        assert_eq!(ring.ack(start.wrapping_add(4), 1, &mut events), Acked::Held);
+        assert_eq!(
+            ring.baseline(start.wrapping_add(6)).map(|s| s.tick),
+            Some(start.wrapping_add(4))
+        );
+        assert_eq!(ring.newest(), Some(start.wrapping_add(5)));
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/encoder.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/encoder.rs
new file mode 100644
index 0000000..171e9e7
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/encoder.rs
@@ -0,0 +1,350 @@
+//! The per-client encoder (u4-build-plan.md §4.4-§4.6; catalogue §3): the 256 kbps window and its remainder after the
+//! pump's bytes, groups with index and count that each decode alone against one baseline, the own-state section's
+//! filter, the mirror of what the client will hold (§4.6's server side), and the exact wire size of each item, so a
+//! group is filled to its limit before it is encoded. Every record goes out through npt-proto's `EntityDelta` and a
+//! `Quant` passed in; the field-group layout is npt-proto's `Snapshot::GROUPS` (the encoder builds none of its own).
+
+use npt_proto::{EntityDelta, Limits, Quant};
+use npt_sim::{EntityKind, EntityRecord, SimEvent};
+
+use crate::ack::{Frame, find, key};
+
+/// The window: 256 kbps of snapshot datagrams per client (catalogue :88: "about 1,067 B" an interval at 30 Hz),
+/// counted as UDP payload plus 28 B each.
+pub(crate) const WINDOW_BYTES_PER_SECOND: u64 = 256_000 / 8;
+/// What each datagram costs the window beyond its payload (catalogue :88; u4-build-plan.md §4.5).
+pub(crate) const DATAGRAM_OVERHEAD: u32 = 28;
+/// A snapshot group's bytes before its body: the type byte, then the header and the body's three counts (tick 32,
+/// baseline age 16, two sequences 16 each, flags 8, group 8; counts 8, 16 and 8: snapshot.rs's wire text).
+pub(crate) const GROUP_HEAD_BYTES: u32 = 1 + (32 + 16 + 16 + 16 + 8 + 8 + 8 + 16 + 8) / 8;
+/// An entity's kind on the wire: enough bits for every position in `EntityKind::ALL` (snapshot.rs's `KIND_BITS`).
+pub(crate) const KIND_BITS: u32 = usize::BITS - (EntityKind::ALL.len() - 1).leading_zeros();
+
+/// The per-tick budget of tick `k` of a window at `tick_hz` ticks a second: the bytes from second-fraction `k` to
+/// `k + 1`, so the fractional remainders carry and one second's budgets sum to exactly 32,000 B at any rate (the F2
+/// ruling: the rate is an argument; production callers pass `npt_sim::TICK_HZ`).
+pub(crate) fn tick_budget(tick_hz: u32, k: u64) -> u32 {
+    let hz = u64::from(tick_hz.max(1));
+    let at = |k: u64| (k % hz) * WINDOW_BYTES_PER_SECOND / hz;
+    let whole = |k: u64| (k / hz) * WINDOW_BYTES_PER_SECOND;
+    ((whole(k + 1) + at(k + 1)) - (whole(k) + at(k))) as u32
+}
+
+/// The bits of `kind`'s whole layout.
+fn layout_bits(kind: EntityKind) -> u32 {
+    kind.layout().iter().map(|f| f.bits()).sum()
+}
+
+/// The bits one delta takes on the wire (snapshot.rs's wire text): kind, index 16, operation 2, then an add's
+/// generation 16 and every field, an update's mask and each changed field, a removal nothing.
+pub(crate) fn delta_bits(d: &EntityDelta) -> u32 {
+    KIND_BITS
+        + 16
+        + 2
+        + match d {
+            EntityDelta::Add(r) => 16 + layout_bits(r.kind),
+            EntityDelta::Update { kind, changed, .. } => {
+                let layout = kind.layout();
+                layout.len() as u32
+                    + layout
+                        .iter()
+                        .enumerate()
+                        .filter(|(i, _)| changed & (1 << i) != 0)
+                        .map(|(_, f)| f.bits())
+                        .sum::<u32>()
+            }
+            EntityDelta::Remove { .. } => 0,
+        }
+}
+
+/// The bits one event takes: number 16, age 8, kind 2, then its fields.
+pub(crate) fn event_bits(e: &SimEvent) -> u32 {
+    16 + 8
+        + 2
+        + match e {
+            SimEvent::Shot { .. } => 8 + 16,
+            SimEvent::Hit { .. } => 8 + 16 + 8 + 16,
+            SimEvent::Kill { .. } => 8 + 8,
+        }
+}
+
+/// A group's bytes for a body of `bits`.
+pub(crate) fn group_bytes(bits: u32) -> u32 {
+    GROUP_HEAD_BYTES + bits.div_ceil(8)
+}
+
+/// One group being filled.
+#[derive(Clone, Debug, Default)]
+pub(crate) struct Group {
+    pub(crate) own: Vec<EntityDelta>,
+    pub(crate) entities: Vec<EntityDelta>,
+    pub(crate) events: Vec<(u16, u8, SimEvent)>,
+    /// Each event's 32-bit number, in `events`' order.
+    pub(crate) numbers: Vec<u32>,
+    /// The 32-bit numbers of the gameplay events it carries (acknowledged with the group).
+    pub(crate) gameplay: Vec<u32>,
+    pub(crate) bits: u32,
+}
+
+/// Which section an item goes to.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) enum Section {
+    Own,
+    Entities,
+    Events,
+}
+
+/// Fills groups in order, each at most `limit` bytes, within the window's remainder (each group costing its bytes plus
+/// 28 B), at most `Limits::GROUPS` of them; the own-state section goes into group 0 alone.
+#[derive(Clone, Debug)]
+pub(crate) struct Packer {
+    pub(crate) groups: Vec<Group>,
+    limit: u32,
+    left: u32,
+}
+
+impl Packer {
+    /// Group 0, if the remainder `budget` holds an empty group's bytes and overhead within `limit`.
+    pub(crate) fn new(limit: u32, budget: u32) -> Option<Packer> {
+        let cost = DATAGRAM_OVERHEAD + group_bytes(0);
+        if group_bytes(0) > limit || cost > budget {
+            return None;
+        }
+        Some(Packer {
+            groups: vec![Group::default()],
+            limit,
+            left: budget - cost,
+        })
+    }
+
+    fn count_ok(g: &Group, section: Section) -> bool {
+        match section {
+            Section::Own => g.own.len() < usize::from(u8::MAX),
+            Section::Entities => g.entities.len() < usize::from(u16::MAX),
+            Section::Events => g.events.len() < usize::from(u8::MAX),
+        }
+    }
+
+    /// Finds room for an item of `bits` in `section`: the last group if it fits there, else a new group (never for
+    /// the own-state section); the group's index, or `None` (skipped).
+    pub(crate) fn place(&mut self, bits: u32, section: Section) -> Option<usize> {
+        let last = self.groups.len() - 1;
+        let g = &self.groups[last];
+        let now = group_bytes(g.bits);
+        let after = group_bytes(g.bits + bits);
+        let own_ok = section != Section::Own || last == 0;
+        if own_ok && Packer::count_ok(g, section) && after <= self.limit && after - now <= self.left
+        {
+            self.left -= after - now;
+            self.groups[last].bits += bits;
+            return Some(last);
+        }
+        let fresh = group_bytes(bits);
+        if section == Section::Own
+            || self.groups.len() >= Limits::GROUPS
+            || fresh > self.limit
+            || DATAGRAM_OVERHEAD + fresh > self.left
+        {
+            return None;
+        }
+        self.left -= DATAGRAM_OVERHEAD + fresh;
+        self.groups.push(Group {
+            bits,
+            ..Group::default()
+        });
+        Some(self.groups.len() - 1)
+    }
+}
+
+/// Inserts or replaces `r` in a sorted list.
+fn upsert(list: &mut Vec<EntityRecord>, r: EntityRecord) {
+    match list.binary_search_by_key(&key(&r), key) {
+        Ok(i) => list[i] = r,
+        Err(i) => list.insert(i, r),
+    }
+}
+
+/// `list` after `d`, each update read against `base`'s record (every group decodes alone against one baseline).
+pub(crate) fn apply(list: &mut Vec<EntityRecord>, base: &[EntityRecord], d: &EntityDelta) {
+    match d {
+        EntityDelta::Add(r) => {
+            if let Ok(Some(r)) = d.apply(None) {
+                upsert(list, r);
+            } else {
+                upsert(list, *r);
+            }
+        }
+        EntityDelta::Update { kind, index, .. } => {
+            if let Ok(Some(r)) = d.apply(find(base, (*kind, *index))) {
+                upsert(list, r);
+            }
+        }
+        EntityDelta::Remove { kind, index } => {
+            if let Ok(i) = list.binary_search_by_key(&(*kind, *index), key) {
+                list.remove(i);
+            }
+        }
+    }
+}
+
+/// The view a client holds after decoding every group: the baseline frame with each group's deltas.
+pub(crate) fn view(base: &Frame, groups: &[Group]) -> Frame {
+    let mut f = base.clone();
+    for g in groups {
+        for d in &g.own {
+            apply(&mut f.own, &base.own, d);
+        }
+        for d in &g.entities {
+            apply(&mut f.entities, &base.entities, d);
+        }
+    }
+    f
+}
+
+/// The encoder's per-client state: the window's tick count and the `Quant` it codes with (npt-proto's, chosen per
+/// instance through `Quant::with_bite` in a bites build: U4E-Q4).
+#[derive(Clone, Debug)]
+pub struct Encoder {
+    tick_hz: u32,
+    k: u64,
+    pub(crate) quant: Quant,
+}
+
+impl Encoder {
+    pub(crate) fn new(tick_hz: u32, quant: Quant) -> Encoder {
+        Encoder {
+            tick_hz,
+            k: 0,
+            quant,
+        }
+    }
+
+    /// This tick's budget less the pump's bytes since the last tick (u4-build-plan.md §4.5: "the window counts them,
+    /// and the encoder's budget is what remains"); the window moves on one tick.
+    pub(crate) fn remainder(&mut self, server_bytes: u32) -> (u32, u32) {
+        let budget = tick_budget(self.tick_hz, self.k);
+        self.k += 1;
+        (budget, budget.saturating_sub(server_bytes))
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use npt_proto::{Direction, Message, Snapshot, decode};
+
+    /// e10's window pin (the F2 ruling, advisor :446-447): one second's per-tick budgets sum to 32,000 B at 30 and at
+    /// 60 Hz (32,000 / 30 is not a whole number of bytes: the remainders carry).
+    #[test]
+    fn e10_the_window_sums_to_32_000_b_a_second_at_30_and_60_hz() {
+        for hz in [30u32, 60] {
+            for second in [0u64, 1, 7] {
+                let sum: u64 = (0..u64::from(hz))
+                    .map(|i| u64::from(tick_budget(hz, second * u64::from(hz) + i)))
+                    .sum();
+                assert_eq!(sum, 32_000, "{hz} Hz, second {second}");
+            }
+        }
+        assert_eq!((tick_budget(30, 0), tick_budget(30, 2)), (1_066, 1_067));
+        assert_eq!((tick_budget(60, 0), tick_budget(60, 2)), (533, 534));
+        assert_eq!(tick_budget(npt_sim::TICK_HZ, 0), tick_budget(30, 0));
+    }
+
+    /// The encoder's size arithmetic equals npt-proto's encoding, so a group filled to its limit encodes at it.
+    #[test]
+    fn the_size_arithmetic_equals_the_encoding_for_every_kind_and_delta_form() {
+        let mut own = Vec::new();
+        let mut entities = Vec::new();
+        let mut bits = 0;
+        for (i, kind) in EntityKind::ALL.into_iter().enumerate() {
+            let mut r = EntityRecord {
+                kind,
+                index: i as u16,
+                generation: 3,
+                area: 0,
+                grid: [0; EntityRecord::FIELDS],
+                raw: [0; EntityRecord::FIELDS],
+            };
+            for (g, f) in r.grid.iter_mut().zip(kind.layout()) {
+                *g = f.max();
+            }
+            let mut base = r;
+            base.grid[0] = kind.layout()[0].min();
+            let add = EntityDelta::Add(r);
+            let update = EntityDelta::between(&r, Some(&base)).expect("one field changed");
+            let remove = EntityDelta::Remove {
+                kind,
+                index: i as u16,
+            };
+            let to = if matches!(kind, EntityKind::Effect | EntityKind::OwnState) {
+                &mut own
+            } else {
+                &mut entities
+            };
+            for d in [add, update, remove] {
+                bits += delta_bits(&d);
+                to.push(d);
+            }
+        }
+        let events = vec![
+            (1, 0, SimEvent::Shot { slot: 1, seq: 2 }),
+            (
+                2,
+                3,
+                SimEvent::Hit {
+                    shooter: 1,
+                    seq: 2,
+                    victim: 3,
+                    damage: 4,
+                },
+            ),
+            (
+                3,
+                255,
+                SimEvent::Kill {
+                    killer: 1,
+                    victim: 2,
+                },
+            ),
+        ];
+        bits += events.iter().map(|e| event_bits(&e.2)).sum::<u32>();
+        let s = Snapshot {
+            tick: 70_000,
+            baseline_age: 2,
+            own,
+            entities,
+            events,
+            ..Snapshot::default()
+        };
+        let msg = Message::Snapshot(s);
+        let mut out = Vec::new();
+        Quant::default().encode(&msg, &mut out).expect("encodes");
+        assert_eq!(out.len() as u32, group_bytes(bits));
+        // npt-room does not depend on npt-link: the snapshot's channel is read from its row of `Limits::TYPES`.
+        let (name, datagram, _, _) = Limits::TYPES[1];
+        assert_eq!(name, "Snapshot");
+        assert_eq!(decode(Direction::RoomToClient, datagram, &out), Ok(msg));
+    }
+
+    #[test]
+    fn the_packer_keeps_the_own_section_in_group_0_and_opens_groups_to_the_limit() {
+        let mut p = Packer::new(100, 1_000).expect("room for group 0");
+        assert_eq!(p.place(8 * 50, Section::Own), Some(0));
+        assert_eq!(p.place(8 * 20, Section::Entities), Some(0));
+        // 17 + 70 + 40 = 127 > 100: a new group.
+        assert_eq!(p.place(8 * 40, Section::Entities), Some(1));
+        assert_eq!(
+            p.place(8 * 10, Section::Own),
+            None,
+            "own never leaves group 0"
+        );
+        // 1,000 - (28 + 17 + 50 + 20) - (28 + 17 + 40) = 800 left; a group of 90 B costs 118: six more, then the cap
+        // of 8 groups.
+        let mut placed = 0;
+        while p.place(8 * 73, Section::Entities).is_some() {
+            placed += 1;
+        }
+        assert_eq!(p.groups.len(), Limits::GROUPS.min(2 + placed));
+        assert!(Packer::new(10, 1_000).is_none() && Packer::new(1_000, 44).is_none());
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/events.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/events.rs
new file mode 100644
index 0000000..4812649
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/events.rs
@@ -0,0 +1,146 @@
+//! The per-client event queue (u4-build-plan.md §4.7; catalogue §3's gameplay-event row): the relevant gameplay
+//! events not yet acknowledged, each stamped with the tick it ran and its 32-bit number, resent until acknowledged,
+//! bounded by a cap held as data.
+
+use std::collections::VecDeque;
+
+use npt_sim::SimEvent;
+
+/// One queued event: its 32-bit number, the tick it ran (`TickInput::tick`: carry 23) and the event.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) struct Queued {
+    pub(crate) number: u32,
+    pub(crate) tick: u32,
+    pub(crate) event: SimEvent,
+    /// How many snapshots have carried it.
+    pub(crate) sent: u32,
+}
+
+/// The oldest age an event's section can code: 8 bits (snapshot.rs's wire text; carry 50).
+pub(crate) const MAX_EVENT_AGE: u32 = u8::MAX as u32;
+
+/// One client's queue of unacknowledged gameplay events (u4-build-plan.md §4.7). Its cap is data: the room's setting,
+/// provisional until U4f measures the rate (U4E-Q6 (a): 1,024 from net F121's example). An overflow is the room's
+/// signal to force one resync; under PM-0001-22/unbounded (bites builds only) the queue keeps every event past its cap.
+#[derive(Clone, Debug)]
+pub struct EventQueue {
+    cap: u32,
+    pending: VecDeque<Queued>,
+    unbounded: bool,
+    high_water: u32,
+}
+
+/// Why [`EventQueue::push`] took no event: the queue was at its cap.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) struct Overflow;
+
+impl EventQueue {
+    /// The provisional cap (U4E-Q6 (a): net F121's example, above the about 400 a B1 burst adds); U4f records §4.7's
+    /// value before -22's runs.
+    pub(crate) const PROVISIONAL_CAP: u32 = 1_024;
+
+    pub(crate) fn new(cap: u32, unbounded: bool) -> EventQueue {
+        EventQueue {
+            cap,
+            pending: VecDeque::new(),
+            unbounded,
+            high_water: 0,
+        }
+    }
+
+    /// Queues one event, oldest first; at the cap it takes none and reports the overflow (but under the -22 bite).
+    pub(crate) fn push(&mut self, q: Queued) -> Result<(), Overflow> {
+        if self.pending.len() >= self.cap as usize && !self.unbounded {
+            return Err(Overflow);
+        }
+        self.pending.push_back(q);
+        self.high_water = self.high_water.max(self.len());
+        Ok(())
+    }
+
+    /// The events queued now.
+    pub(crate) fn len(&self) -> u32 {
+        self.pending.len() as u32
+    }
+
+    /// The most it has held.
+    pub(crate) fn high_water(&self) -> u32 {
+        self.high_water
+    }
+
+    /// The cap less the length, signed: below 0 only under the -22 bite (SERIES.md's
+    /// `tick.v.ack_starved_headroom_count`).
+    pub(crate) fn headroom(&self) -> i64 {
+        i64::from(self.cap) - i64::from(self.len())
+    }
+
+    /// Removes the acknowledged events (each number once; the queue is in number order, wrap-aware).
+    pub(crate) fn ack(&mut self, numbers: &[u32]) {
+        if numbers.is_empty() {
+            return;
+        }
+        self.pending.retain(|q| !numbers.contains(&q.number));
+    }
+
+    /// Drops every event that could no longer be coded at tick `now` (older than 255 ticks: carry 50) and returns
+    /// how many.
+    pub(crate) fn expire(&mut self, now: u32) -> u32 {
+        let before = self.pending.len();
+        self.pending
+            .retain(|q| now.wrapping_sub(q.tick) <= MAX_EVENT_AGE);
+        (before - self.pending.len()) as u32
+    }
+
+    /// A full state clears the client's unacknowledged events (catalogue Q8).
+    pub(crate) fn clear(&mut self) {
+        self.pending.clear();
+    }
+
+    /// The queue, oldest first.
+    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut Queued> {
+        self.pending.iter_mut()
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    fn hit(n: u32, tick: u32) -> Queued {
+        Queued {
+            number: n,
+            tick,
+            event: SimEvent::Kill {
+                killer: 1,
+                victim: 2,
+            },
+            sent: 0,
+        }
+    }
+
+    #[test]
+    fn the_cap_refuses_at_its_value_and_the_headroom_falls_to_zero_never_below() {
+        let mut q = EventQueue::new(3, false);
+        for n in 0..3 {
+            assert_eq!(q.push(hit(n, 10)), Ok(()));
+        }
+        assert_eq!(q.headroom(), 0);
+        assert_eq!(q.push(hit(3, 10)), Err(Overflow));
+        assert_eq!((q.len(), q.high_water(), q.headroom()), (3, 3, 0));
+        q.ack(&[1]);
+        assert_eq!(q.len(), 2);
+        q.clear();
+        assert_eq!((q.len(), q.high_water()), (0, 3));
+    }
+
+    /// Carry 50's edge: an event at age 255 is kept (it can still be sent), none at 256.
+    #[test]
+    fn carry_50_an_event_at_age_255_is_kept_and_at_256_dropped_and_counted() {
+        let mut q = EventQueue::new(8, false);
+        q.push(hit(0, 1_000)).expect("room");
+        assert_eq!(q.expire(1_000 + 255), 0);
+        assert_eq!(q.len(), 1);
+        assert_eq!(q.expire(1_000 + 256), 1);
+        assert_eq!(q.len(), 0);
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/input.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/input.rs
new file mode 100644
index 0000000..e16f4d2
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/input.rs
@@ -0,0 +1,280 @@
+//! The input buffer under OQ-0001-6's option B (ADR 0001:1156-1162: "buffer them and run exactly one per client per
+//! tick, repeating the last on a miss, late commands dropped and catch-up limited to the redundancy window"), and the
+//! two room-side command checks PM-0001-58's counters pass through (u4-build-plan.md §4.14 :467-468; plan.md:514): the
+//! sequence-forward compare and the press-edge compare, each through npt-proto's `Wrap` with its counter's name.
+//! Option A, and every other command check with its -21 counter, are U4f's.
+
+use npt_proto::Wrap;
+use npt_sim::Command;
+
+/// The sequence-forward compare's counter name in `Wrap` (PM-0001-58/lt-cmd-seq plants a plain `<` on it).
+pub(crate) const CMD_SEQ: &str = "cmd_seq";
+/// The press-edge compare's counter name (PM-0001-58/lt-press).
+pub(crate) const PRESS: &str = "press";
+
+/// One client's input buffer (option B) with its two checks' state.
+#[derive(Clone, Debug, Default)]
+pub struct InputBuffer {
+    /// Accepted commands not yet run, at most [`InputBuffer::CMD_CATCHUP`], in stamped-tick order.
+    buffered: Vec<Command>,
+    /// The last command run: option B runs it again on a tick with none stamped for it.
+    last_run: Option<Command>,
+    /// The server tick the last `take` ran.
+    ran_through: Option<u32>,
+    /// The newest sequence received (the forward compare's reference).
+    newest_seq: Option<u16>,
+    /// The last command accepted (the press compare's reference).
+    accepted: Option<Command>,
+    /// How many ticks ahead the newest buffered command was at the last `take` (`buffer_depth_ticks`).
+    depth: u32,
+}
+
+/// What the buffer did with one received command.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) enum Offered {
+    /// Accepted and buffered for its tick.
+    Buffered,
+    /// Its sequence is not after the newest received (a redundant copy): not run again.
+    Repeat,
+    /// A press counter moved backwards or by more edges than the ticks the command covers: refused.
+    PressRefused,
+    /// Stamped for a tick already run, or one already buffered: it can never run.
+    Late,
+    /// The buffer holds [`InputBuffer::CMD_CATCHUP`] commands already.
+    Full,
+}
+
+/// What `take` ran for one tick.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) struct Taken {
+    /// The command run, if the client has sent any yet.
+    pub(crate) command: Option<Command>,
+    /// Whether it is the last command run again (no command was stamped for this tick).
+    pub(crate) repeated: bool,
+    /// Buffered commands stamped before this tick, dropped as late.
+    pub(crate) late: u32,
+}
+
+impl InputBuffer {
+    /// Commands per client the buffer holds ahead, the redundancy cap (u4-build-plan.md §4.3; carry 27): one `Input`
+    /// carries at most this many, so a client never needs more buffered. Not derived from the tick rate.
+    pub const CMD_CATCHUP: usize = npt_proto::Limits::REDUNDANCY_CAP;
+
+    /// How many ticks ahead of the tick last run the newest buffered command was (0: none ahead).
+    pub fn buffer_depth_ticks(&self) -> u32 {
+        self.depth
+    }
+
+    /// The newest sequence received, which a snapshot's header acknowledges (it ends the client's repeats).
+    pub(crate) fn newest_received(&self) -> Option<u16> {
+        self.newest_seq
+    }
+
+    /// The last command run, whose sequence a snapshot's header names (where the client's replay starts).
+    pub(crate) fn last_run(&self) -> Option<&Command> {
+        self.last_run.as_ref()
+    }
+
+    /// The checks and the buffer for one received command (its `slot` already the session's). A counter that wrapped
+    /// on an accepted command pushes its name onto `wraps` (for -58's `event.counter_wrap`).
+    pub(crate) fn offer(&mut self, c: Command, w: Wrap, wraps: &mut Vec<&'static str>) -> Offered {
+        if let Some(n) = self.newest_seq {
+            if !w.after16(CMD_SEQ, n, c.seq) {
+                return Offered::Repeat;
+            }
+            if c.seq < n {
+                wraps.push(CMD_SEQ);
+            }
+        }
+        self.newest_seq = Some(c.seq);
+        if let Some(prev) = &self.accepted {
+            // The ticks the command covers: from the last accepted command's tick to its own.
+            let covered = (c.tick.wrapping_sub(prev.tick) as i32).max(0) as u32;
+            let mut wrapped = false;
+            for (&a, &b) in prev.presses.iter().zip(&c.presses) {
+                if a == b {
+                    continue;
+                }
+                if !w.after8(PRESS, a, b) || u32::from(b.wrapping_sub(a)) > covered {
+                    return Offered::PressRefused;
+                }
+                wrapped |= b < a;
+            }
+            if wrapped {
+                wraps.push(PRESS);
+            }
+        }
+        if self
+            .ran_through
+            .is_some_and(|r| !w.after32("tick", r, c.tick))
+            || self.buffered.iter().any(|b| b.tick == c.tick)
+        {
+            return Offered::Late;
+        }
+        if self.buffered.len() >= InputBuffer::CMD_CATCHUP {
+            return Offered::Full;
+        }
+        let at = self
+            .buffered
+            .iter()
+            .position(|b| w.after32("tick", c.tick, b.tick))
+            .unwrap_or(self.buffered.len());
+        self.buffered.insert(at, c);
+        self.accepted = Some(c);
+        Offered::Buffered
+    }
+
+    /// The command for tick `tick` (option B): the one stamped for it, else the last command run again; buffered
+    /// commands stamped before it are dropped as late.
+    pub(crate) fn take(&mut self, tick: u32) -> Taken {
+        let w = Wrap::default();
+        self.ran_through = Some(tick);
+        let before = self.buffered.len();
+        self.buffered.retain(|c| !w.after32("tick", c.tick, tick));
+        let late = (before - self.buffered.len()) as u32;
+        let (command, repeated) = if self.buffered.first().is_some_and(|c| c.tick == tick) {
+            let c = self.buffered.remove(0);
+            self.last_run = Some(c);
+            (Some(c), false)
+        } else {
+            (self.last_run, self.last_run.is_some())
+        };
+        self.depth = self
+            .buffered
+            .last()
+            .map_or(0, |c| c.tick.wrapping_sub(tick));
+        Taken {
+            command,
+            repeated,
+            late,
+        }
+    }
+}
+
+// Carry 27 (u4-build-plan.md §4.3; ADR 0001:238-239): the catch-up cap is no larger than the redundancy cap.
+const _: () = assert!(InputBuffer::CMD_CATCHUP <= npt_proto::Limits::REDUNDANCY_CAP);
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    fn cmd(seq: u16, tick: u32, presses: [u8; 4]) -> Command {
+        Command {
+            slot: 3,
+            tick,
+            seq,
+            presses,
+            ..Command::default()
+        }
+    }
+
+    /// Carry 27: the const assert holds `CMD_CATCHUP` to the redundancy cap, and it is that cap.
+    #[test]
+    fn carry_27_cmd_catchup_is_the_redundancy_cap() {
+        assert_eq!(InputBuffer::CMD_CATCHUP, npt_proto::Limits::REDUNDANCY_CAP);
+        let mut b = InputBuffer::default();
+        let w = Wrap::default();
+        let mut wraps = Vec::new();
+        for k in 0..InputBuffer::CMD_CATCHUP as u16 {
+            assert_eq!(
+                b.offer(cmd(k, 10 + u32::from(k), [0; 4]), w, &mut wraps),
+                Offered::Buffered
+            );
+        }
+        assert_eq!(b.offer(cmd(99, 99, [0; 4]), w, &mut wraps), Offered::Full);
+    }
+
+    /// e9's first room-side negative: a repeated command is not run twice.
+    #[test]
+    fn e9_a_repeated_command_is_not_run_twice() {
+        let mut b = InputBuffer::default();
+        let w = Wrap::default();
+        let mut wraps = Vec::new();
+        assert_eq!(
+            b.offer(cmd(7, 10, [0; 4]), w, &mut wraps),
+            Offered::Buffered
+        );
+        assert_eq!(b.offer(cmd(7, 10, [0; 4]), w, &mut wraps), Offered::Repeat);
+        assert_eq!(b.offer(cmd(6, 9, [0; 4]), w, &mut wraps), Offered::Repeat);
+        let t = b.take(10);
+        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), false));
+        // The next tick has none stamped: option B runs the last command again, flagged as a repeat.
+        let t = b.take(11);
+        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), true));
+    }
+
+    /// e9's second room-side negative: two press edges for one button inside one covered tick are refused; one edge per
+    /// covered tick passes, and a counter moving backwards is refused.
+    #[test]
+    fn e9_two_press_edges_inside_one_covered_tick_are_refused() {
+        let mut b = InputBuffer::default();
+        let w = Wrap::default();
+        let mut wraps = Vec::new();
+        assert_eq!(
+            b.offer(cmd(1, 10, [0; 4]), w, &mut wraps),
+            Offered::Buffered
+        );
+        assert_eq!(
+            b.offer(cmd(2, 11, [2, 0, 0, 0]), w, &mut wraps),
+            Offered::PressRefused
+        );
+        assert_eq!(
+            b.offer(cmd(3, 11, [1, 0, 0, 0]), w, &mut wraps),
+            Offered::Buffered
+        );
+        // Covering two ticks, two edges pass.
+        assert_eq!(
+            b.offer(cmd(4, 13, [3, 0, 0, 0]), w, &mut wraps),
+            Offered::Buffered
+        );
+        assert_eq!(
+            b.offer(cmd(5, 14, [2, 0, 0, 0]), w, &mut wraps),
+            Offered::PressRefused
+        );
+        assert!(wraps.is_empty());
+    }
+
+    /// e9's "each counter accepted across its wrap in the clean build": the sequence and a press counter cross their
+    /// wraps from injectable start values a few steps below, each wrap reported once by name.
+    #[test]
+    fn e9_the_sequence_and_the_press_counters_are_accepted_across_their_wraps() {
+        let mut b = InputBuffer::default();
+        let w = Wrap::default();
+        let mut wraps = Vec::new();
+        let (seq0, press0) = (u16::MAX - 2, u8::MAX - 1);
+        for k in 0..6u16 {
+            let seq = seq0.wrapping_add(k);
+            let press = press0.wrapping_add(k as u8);
+            let got = b.offer(
+                cmd(seq, 100 + u32::from(k), [press, 0, 0, 0]),
+                w,
+                &mut wraps,
+            );
+            assert_eq!(got, Offered::Buffered, "step {k}");
+            b.take(100 + u32::from(k));
+        }
+        assert_eq!(wraps, [PRESS, CMD_SEQ]);
+    }
+
+    #[test]
+    fn late_commands_are_dropped_and_the_depth_is_measured() {
+        let mut b = InputBuffer::default();
+        let w = Wrap::default();
+        let mut wraps = Vec::new();
+        for (seq, tick) in [(1, 10), (2, 11), (3, 13)] {
+            assert_eq!(
+                b.offer(cmd(seq, tick, [0; 4]), w, &mut wraps),
+                Offered::Buffered
+            );
+        }
+        let t = b.take(11);
+        assert_eq!((t.command.map(|c| c.seq), t.late), (Some(2), 1));
+        assert_eq!(b.buffer_depth_ticks(), 2);
+        assert_eq!(b.offer(cmd(4, 11, [0; 4]), w, &mut wraps), Offered::Late);
+        assert_eq!(
+            b.offer(cmd(5, 13, [0; 4]), w, &mut wraps),
+            Offered::Late,
+            "tick 13 is buffered"
+        );
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/interest.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/interest.rs
new file mode 100644
index 0000000..85a8ad7
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/interest.rs
@@ -0,0 +1,245 @@
+//! Interest (catalogue §3: "the spatial groups are packed by the priority accumulator under the per-client cap"; part 9):
+//! [`InterestGrid`] finds what lies near a point, in integer centimetres; [`AreaFlood`] says which areas a client's
+//! area connects to through the open portals; [`Priority`] orders what a snapshot carries. Float-free
+//! (u4-build-plan.md §4.11).
+
+use std::collections::BTreeMap;
+
+use npt_sim::{Areas, EntityKind, Portals};
+
+/// A uniform grid over the map's extent in the position grid's centimetres: each cell lists the records whose point
+/// lies in it, rebuilt each tick.
+#[derive(Clone, Debug)]
+pub struct InterestGrid {
+    min: [i32; 2],
+    cell: i32,
+    n: [usize; 2],
+    cells: Vec<Vec<u32>>,
+}
+
+impl InterestGrid {
+    /// A grid over `extent` (`[x_min, y_min, x_max, y_max]`, cm) of square cells `cell_cm` wide.
+    pub(crate) fn new(extent: [i32; 4], cell_cm: i32) -> InterestGrid {
+        let cell = cell_cm.max(1);
+        let span =
+            |lo: i32, hi: i32| ((i64::from(hi) - i64::from(lo)) / i64::from(cell) + 1) as usize;
+        let n = [span(extent[0], extent[2]), span(extent[1], extent[3])];
+        InterestGrid {
+            min: [extent[0], extent[1]],
+            cell,
+            n,
+            cells: vec![Vec::new(); n[0] * n[1]],
+        }
+    }
+
+    /// The cell a point falls in, a point outside the extent taken into its nearest edge cell.
+    fn cell_of(&self, at: [i32; 2]) -> [usize; 2] {
+        let c = |v: i32, min: i32, n: usize| {
+            let k = (i64::from(v) - i64::from(min)).div_euclid(i64::from(self.cell));
+            k.clamp(0, n as i64 - 1) as usize
+        };
+        [
+            c(at[0], self.min[0], self.n[0]),
+            c(at[1], self.min[1], self.n[1]),
+        ]
+    }
+
+    /// Empties every cell, keeping their storage.
+    pub(crate) fn clear(&mut self) {
+        for c in &mut self.cells {
+            c.clear();
+        }
+    }
+
+    /// Files record `id` at `at`.
+    pub(crate) fn insert(&mut self, at: [i32; 2], id: u32) {
+        let [x, y] = self.cell_of(at);
+        self.cells[y * self.n[0] + x].push(id);
+    }
+
+    /// Every record filed in a cell that the square of half-width `radius` round `at` touches (the caller checks the
+    /// exact distance).
+    pub(crate) fn near(&self, at: [i32; 2], radius: i32, out: &mut Vec<u32>) {
+        out.clear();
+        let lo = self.cell_of([at[0].saturating_sub(radius), at[1].saturating_sub(radius)]);
+        let hi = self.cell_of([at[0].saturating_add(radius), at[1].saturating_add(radius)]);
+        for y in lo[1]..=hi[1] {
+            for x in lo[0]..=hi[0] {
+                out.extend_from_slice(&self.cells[y * self.n[0] + x]);
+            }
+        }
+    }
+}
+
+/// Which areas each area connects to through the portals open now (u4-build-plan.md §4.2, A2): re-flooded when the
+/// portal states change, from npt-sim's plain-data static graph.
+#[derive(Clone, Debug)]
+pub struct AreaFlood {
+    joins: Vec<[u16; 2]>,
+    areas: usize,
+    open: Vec<bool>,
+    /// `reach[a * areas + b]`: area b connects to area a.
+    reach: Vec<bool>,
+}
+
+impl AreaFlood {
+    pub(crate) fn new(areas: &Areas, portals: &Portals) -> AreaFlood {
+        let n = areas.rects.len().max(1);
+        let mut f = AreaFlood {
+            joins: portals.joins.clone(),
+            areas: n,
+            open: Vec::new(),
+            reach: vec![false; n * n],
+        };
+        f.flood(&vec![false; portals.joins.len()]);
+        f
+    }
+
+    /// Re-floods when `open` differs from the states it last flooded with; returns whether it did.
+    pub(crate) fn update(&mut self, open: &[bool]) -> bool {
+        if open == self.open.as_slice() {
+            return false;
+        }
+        self.flood(open);
+        true
+    }
+
+    fn flood(&mut self, open: &[bool]) {
+        self.open.clear();
+        self.open.extend_from_slice(open);
+        let n = self.areas;
+        self.reach.iter_mut().for_each(|r| *r = false);
+        let mut stack = Vec::new();
+        for start in 0..n {
+            self.reach[start * n + start] = true;
+            stack.clear();
+            stack.push(start);
+            while let Some(a) = stack.pop() {
+                for (p, [x, y]) in self.joins.iter().enumerate() {
+                    if !open.get(p).copied().unwrap_or(false) {
+                        continue;
+                    }
+                    let (x, y) = (usize::from(*x), usize::from(*y));
+                    let other = if x == a {
+                        y
+                    } else if y == a {
+                        x
+                    } else {
+                        continue;
+                    };
+                    if other < n && !self.reach[start * n + other] {
+                        self.reach[start * n + other] = true;
+                        stack.push(other);
+                    }
+                }
+            }
+        }
+    }
+
+    /// Whether area `to` connects to area `from` now (an area outside the graph connects to nothing).
+    pub(crate) fn reaches(&self, from: u16, to: u16) -> bool {
+        let (a, b) = (usize::from(from), usize::from(to));
+        a < self.areas && b < self.areas && self.reach[a * self.areas + b]
+    }
+
+    /// Whether portal `p` lies in an area `from` connects to: a portal counts as inside both areas it joins
+    /// (catalogue §3's world-pieces row).
+    pub(crate) fn sees_portal(&self, from: u16, p: usize) -> bool {
+        self.joins
+            .get(p)
+            .is_some_and(|[x, y]| self.reaches(from, *x) || self.reaches(from, *y))
+    }
+}
+
+/// The priority accumulator (catalogue §3, part 9 (d)): own state always (outside it), the nearest 8 enemies in every
+/// snapshot, and every relevant entity with a change at least every 4 snapshots; this holds, per entity, how many
+/// snapshots in a row skipped its change.
+#[derive(Clone, Debug, Default)]
+pub struct Priority {
+    skipped: BTreeMap<(EntityKind, u16), u32>,
+}
+
+impl Priority {
+    /// The nearest enemies every snapshot carries.
+    pub(crate) const NEAREST_ENEMIES: usize = 8;
+    /// Every relevant changed entity goes out at least once in this many snapshots, when the budget allows.
+    pub(crate) const EVERY: u32 = 4;
+
+    /// How many snapshots in a row skipped `id`'s change.
+    pub(crate) fn skipped(&self, id: (EntityKind, u16)) -> u32 {
+        self.skipped.get(&id).copied().unwrap_or(0)
+    }
+
+    /// `id`'s change was sent, or it had none.
+    pub(crate) fn sent(&mut self, id: (EntityKind, u16)) {
+        self.skipped.remove(&id);
+    }
+
+    /// `id`'s change was skipped this snapshot.
+    pub(crate) fn skip(&mut self, id: (EntityKind, u16)) {
+        *self.skipped.entry(id).or_insert(0) += 1;
+    }
+
+    /// Forget every entity no longer relevant (`keep` says which are).
+    pub(crate) fn retain(&mut self, mut keep: impl FnMut((EntityKind, u16)) -> bool) {
+        self.skipped.retain(|k, _| keep(*k));
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn the_grid_finds_what_lies_in_the_touched_cells_and_clamps_points_outside() {
+        let mut g = InterestGrid::new([-1_000, -1_000, 1_000, 1_000], 500);
+        g.insert([0, 0], 1);
+        g.insert([900, 900], 2);
+        g.insert([50_000, 0], 3);
+        let mut out = Vec::new();
+        g.near([10, 10], 100, &mut out);
+        assert_eq!(out, [1]);
+        g.near([1_000, 0], 100, &mut out);
+        assert!(out.contains(&3) && !out.contains(&1));
+        g.clear();
+        g.near([10, 10], 100, &mut out);
+        assert!(out.is_empty());
+    }
+
+    #[test]
+    fn the_flood_follows_open_portals_and_refloods_only_on_a_change() {
+        let areas = Areas {
+            rects: vec![[0; 4]; 4],
+        };
+        let portals = Portals {
+            joins: vec![[0, 1], [0, 2], [2, 3]],
+        };
+        let mut f = AreaFlood::new(&areas, &portals);
+        assert!(f.reaches(1, 1) && !f.reaches(1, 0));
+        assert!(f.update(&[true, false, true]));
+        assert!(!f.update(&[true, false, true]));
+        assert!(f.reaches(1, 0) && !f.reaches(1, 2) && f.reaches(2, 3));
+        assert!(
+            f.sees_portal(1, 1),
+            "portal 1 joins area 0, which area 1 reaches"
+        );
+        assert!(!f.sees_portal(3, 0));
+        assert!(f.update(&[true, true, true]));
+        assert!(f.reaches(1, 3) && f.reaches(3, 1));
+        assert!(!f.reaches(9, 0));
+    }
+
+    #[test]
+    fn priority_counts_skips_in_a_row_and_forgets_on_a_send() {
+        let mut p = Priority::default();
+        let id = (EntityKind::Projectile, 4);
+        p.skip(id);
+        p.skip(id);
+        assert_eq!(p.skipped(id), 2);
+        p.sent(id);
+        assert_eq!(p.skipped(id), 0);
+        p.skip(id);
+        p.retain(|k| k != id);
+        assert_eq!(p.skipped(id), 0);
+    }
+}
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/room/src/lib.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/lib.rs
index c6f2373..f66dce9 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/room/src/lib.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/lib.rs
@@ -3,3 +3,44 @@
 //! Slots and roster, every tick-counted command check, the input buffer under both OQ-0001-6 options, interest, hit
 //! history and the rewind query, per-client encoding and the event queue; it accepts only decoded messages and talks
 //! to the sim through one seam (`SimCore`). No I/O, no async runtime, no clock; it builds for wasm32. Built in unit U4.
+//!
+//! U4e (u4-build-plan.md §5 U4e) builds [`Room`] over the seam alone, with its [`Roster`] and constant-time token
+//! compare, each client's [`ClientSlot`] ([`AckRing`], [`Encoder`], [`EventQueue`], [`InputBuffer`] under option B,
+//! [`Priority`]), the room's [`InterestGrid`] and [`AreaFlood`], [`rewind_at`]'s integer half, and the room-side checks
+//! PM-0001-58's counters pass through. Every value is an integer: the room holds no float (u4-build-plan.md §4.11; the
+//! lint below and check.mjs's float grep).
+#![deny(clippy::float_arithmetic)]
+
+mod ack;
+mod encoder;
+mod events;
+mod input;
+mod interest;
+mod rewind;
+mod room;
+mod roster;
+
+pub use ack::AckRing;
+pub use encoder::Encoder;
+pub use events::EventQueue;
+pub use input::InputBuffer;
+pub use interest::{AreaFlood, InterestGrid, Priority};
+pub use rewind::rewind_at;
+pub use room::{ClientCounters, ClientSlot, Room};
+pub use roster::Roster;
+
+/// The bite ids npt-room plants (bites builds only; U4E-Q3 (a): a bites-only list of the room's own ids), each chosen
+/// per instance through [`Room::with_bite`] (u4-build-plan.md §4.11): -18's baseline choice, -58's three room-side
+/// compares, -62's event routing, -22's event-queue cap, and catalogue item 5's class filter, whose id is outside the
+/// `PM-<ADR>-<row>/<name>` form (U4E-Q13 (a): no mark judges it). npt-proto's ids reach the room's encoder through
+/// `npt_proto::Quant::with_bite`, never spelled here.
+#[cfg(feature = "bites")]
+pub const BITES: [&str; 7] = [
+    "PM-0001-18/no-fallback",
+    "PM-0001-58/lt-ack",
+    "PM-0001-58/lt-cmd-seq",
+    "PM-0001-58/lt-press",
+    "PM-0001-62/server-echo",
+    "PM-0001-22/unbounded",
+    "catalogue-5/owner-to-all",
+];
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/rewind.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/rewind.rs
new file mode 100644
index 0000000..9a561d6
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/rewind.rs
@@ -0,0 +1,33 @@
+//! `rewind_at`, the integer half (u4-build-plan.md §4.9 :348-350; §4.2 :219-223): a command's view time to the
+//! [`Rewind`] its shot sees the past through. The tolerance and the cap are U4h's.
+
+use npt_sim::{Command, CoverVariant, HitHistory, Rewind};
+
+/// Whether `history` holds every tick `r` reads: `tick_a`, and `tick_a + 1` (wrapping) when `fraction` is above 0
+/// (seam.rs's rule for a Rewind the lent ring holds).
+fn holds(history: &HitHistory, r: &Rewind) -> bool {
+    history.get(r.tick_a).is_some()
+        && (r.fraction == 0 || history.get(r.tick_a.wrapping_add(1)).is_some())
+}
+
+/// The [`Rewind`] for `command`'s claimed view time (`view_tick`, `view_fraction`: seam.rs) against the room's ring,
+/// with the room's cover variant; `None` when the ring does not hold that view time (it lacks `view_tick`, or the
+/// fraction is above 0 and it lacks `view_tick + 1`). By U4E-Q5 (a) the command then runs with no Rewind and the room
+/// counts it (a refused Rewind runs as none in the sim, so this is one rule); U4h replaces it with the tolerance and
+/// the cap. So it never names `tick_a + 1` beyond the newest tick pushed when `fraction > 0` (carry 22).
+pub fn rewind_at(command: &Command, history: &HitHistory, cover: CoverVariant) -> Option<Rewind> {
+    let r = Rewind {
+        tick_a: command.view_tick,
+        fraction: command.view_fraction,
+        cover,
+    };
+    if !holds(history, &r) {
+        return None;
+    }
+    // The sim refuses and counts a Rewind its ring does not hold (seam.rs); a correct room never sends one (§4.2).
+    debug_assert!(
+        holds(history, &r),
+        "rewind_at names a tick the ring does not hold"
+    );
+    Some(r)
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/room.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/room.rs
new file mode 100644
index 0000000..3075ceb
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/room.rs
@@ -0,0 +1,1413 @@
+//! `Room<S: SimCore>` (u4-build-plan.md §4.1, §4.2; plan.md:596-598): the sim driven through the seam alone, its hit
+//! history, the roster, and per client its [`ClientSlot`]. Each tick it fills one `TickInput` (the commands in the
+//! canonical order, each with its Rewind, and the tick's scripted spawns, teleports and attribute changes), steps,
+//! copies the step's events into its stamped list (carry 26) before it reads anything else, pushes the hit volumes,
+//! re-floods on a portal change, keeps the reports for the logs, and codes each client's snapshot.
+
+use npt_proto::{
+    EncodeError, EntityDelta, FullState, Input, Join, JoinAccepted, JoinRejected, Kick, Limits,
+    Message, Quant, Snapshot, Wrap, client_hash,
+};
+use npt_sim::{
+    AttributeChange, Command, CoverVariant, EntityKind, EntityRecord, Greybox, HitHistory,
+    InputReports, Placement, Rewind, Scene, SimCore, SimEvent, TickInput,
+};
+
+use crate::ack::{AckRing, Acked, Frame, find, key};
+use crate::encoder::{
+    DATAGRAM_OVERHEAD, Encoder, Packer, Section, delta_bits, event_bits, group_bytes, view,
+};
+use crate::events::{EventQueue, Queued};
+use crate::input::{CMD_SEQ, InputBuffer, Offered, PRESS};
+use crate::interest::{AreaFlood, InterestGrid, Priority};
+use crate::rewind::rewind_at;
+use crate::roster::Roster;
+
+/// The ack rebuild's counter name in `Wrap` (PM-0001-58/lt-ack plants a plain `<` on it).
+pub(crate) const ACK: &str = "ack";
+/// The relevance radius round a client's soldier, cm (plan silent on its value: provisional data, 64 m).
+const RADIUS_CM: i32 = 6_400;
+/// The interest grid's cell, cm.
+const CELL_CM: i32 = 1_600;
+/// The hit history's span, ms: provisional until U4h measures its test cap (u4-build-plan.md §4.9).
+const HISTORY_MS: u32 = 2_000;
+
+/// The room's planted defects (bites builds only), chosen per instance by [`Room::with_bite`].
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum RoomBite {
+    NoFallback,
+    LtAck,
+    LtCmdSeq,
+    LtPress,
+    ServerEcho,
+    Unbounded,
+    OwnerToAll,
+}
+
+#[cfg(feature = "bites")]
+impl RoomBite {
+    fn of(id: &str) -> Option<RoomBite> {
+        let all = [
+            RoomBite::NoFallback,
+            RoomBite::LtAck,
+            RoomBite::LtCmdSeq,
+            RoomBite::LtPress,
+            RoomBite::ServerEcho,
+            RoomBite::Unbounded,
+            RoomBite::OwnerToAll,
+        ];
+        crate::BITES.iter().position(|b| *b == id).map(|i| all[i])
+    }
+}
+
+/// Each client's running counts, for U4r's logs and the -21 and -22 counters U4f adds (U4E-Q7, U4E-Q11).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub struct ClientCounters {
+    /// Ticks a snapshot went out (one or more groups).
+    pub snapshots: u64,
+    /// Snapshot datagrams (groups) sent.
+    pub groups: u64,
+    /// Ticks sent as 2 or more groups.
+    pub split_ticks: u64,
+    /// Full states sent (at join, on a `Resync`, on an overflow).
+    pub full_states: u64,
+    /// Snapshots coded against the per-type defaults (`baseline_age` 0).
+    pub defaults_coded: u64,
+    /// Snapshots coded against a baseline older than the ring (only under PM-0001-18/no-fallback).
+    pub stale_coded: u64,
+    /// Changed entities the priority accumulator held at the baseline's value, summed over ticks.
+    pub skipped: u64,
+    /// Ticks with no snapshot because the remainder could not hold the own-state group (plan silent on the case).
+    pub budget_short: u64,
+    /// Ticks whose snapshot the encoder refused, by `EncodeError` kind (carry 54; the advisor's A4): out of range ...
+    pub refused_out_of_range: u64,
+    /// ... too many ...
+    pub refused_too_many: u64,
+    /// ... too large ...
+    pub refused_too_large: u64,
+    /// ... not canonical.
+    pub refused_not_canonical: u64,
+    /// Relevant gameplay events queued (U4E-Q6 condition 1).
+    pub events_queued: u64,
+    /// Gameplay events carried for the first time.
+    pub events_sent: u64,
+    /// Gameplay events carried again (resent until acknowledged).
+    pub events_resent: u64,
+    /// Gameplay events dropped from the resend at age 256, before the overflow resync came (carry 50).
+    pub events_expired: u64,
+    /// Cosmetic events sent (once each).
+    pub cosmetic_sent: u64,
+    /// Cosmetic events the remainder had no room for (never resent).
+    pub cosmetic_skipped: u64,
+    /// Overflow resyncs (carry 57); the tick of each is [`ClientSlot::overflow_resync`] on that tick.
+    pub overflow_resyncs: u64,
+    /// `Resync`s answered with a full state.
+    pub resyncs: u64,
+    /// A second `Resync` before the full state's acknowledgement: dropped (catalogue :90).
+    pub resyncs_dropped: u64,
+    /// `Kick(overflow)`s sent: the client was dropped (§4.7).
+    pub kicked: u64,
+    /// Commands accepted into the input buffer.
+    pub commands_buffered: u64,
+    /// Commands whose sequence was not after the newest received: not run again.
+    pub commands_repeat: u64,
+    /// Commands refused by the press-edge compare.
+    pub press_refused: u64,
+    /// Commands stamped for a tick already run (or already buffered): dropped as late.
+    pub late: u64,
+    /// Commands refused because the buffer held `CMD_CATCHUP` already.
+    pub buffer_full: u64,
+    /// Datagrams whose acknowledgement named no snapshot sent to this client: their commands refused.
+    pub ack_refused: u64,
+    /// The commands those datagrams carried.
+    pub ack_refused_commands: u64,
+    /// Acknowledgements of a sent tick older than the ring.
+    pub acks_old: u64,
+    /// Commands run at the tick stamped on them.
+    pub commands_run: u64,
+    /// Ticks on which option B ran the last command again (none was stamped for the tick).
+    pub commands_rerun: u64,
+    /// Commands holding or pressing fire whose view time the ring did not hold: run with no Rewind (U4E-Q5 (a)),
+    /// a replacement in -30's setup condition (carry 40).
+    pub rewinds_refused: u64,
+}
+
+/// What one tick did for one client (the per-tick values U4r's logs write).
+#[derive(Clone, Debug, Default)]
+struct TickValues {
+    server_hash: Option<u64>,
+    burst_eviction: bool,
+    baseline_age: Option<u16>,
+    groups: u8,
+    bytes: u32,
+    budget: u32,
+    remainder: u32,
+    own_bits: u32,
+    own_state_bits: u32,
+    own_state_changed: bool,
+    overflow_resync: bool,
+    headroom: i64,
+    wraps: Vec<&'static str>,
+    rewinds_refused: Vec<u16>,
+    skipped: u32,
+}
+
+/// One client's slot in the room: its session's ring, queue, buffer, encoder and priorities, its outbox, and what the
+/// last tick did for it.
+#[derive(Clone, Debug)]
+pub struct ClientSlot {
+    slot: u8,
+    team: u8,
+    ack: AckRing,
+    queue: EventQueue,
+    input: InputBuffer,
+    encoder: Encoder,
+    priority: Priority,
+    /// A full state is due this tick: `Some(forced)`, forced by an overflow.
+    full_state_due: Option<bool>,
+    /// The full state sent and not yet acknowledged: its tick, and whether an overflow forced it.
+    in_flight: Option<(u32, bool)>,
+    kicked: bool,
+    server_bytes: u32,
+    max_datagram: usize,
+    datagrams: Vec<Vec<u8>>,
+    control: Vec<Message>,
+    counters: ClientCounters,
+    last_ack_hi: Option<u16>,
+    pending_wraps: Vec<&'static str>,
+    /// The fire press counter of the last command run (a fire edge asks for a Rewind).
+    fire_presses: Option<u8>,
+    t: TickValues,
+    relevant: Vec<usize>,
+    sees: Vec<bool>,
+    nearest: Vec<u16>,
+    cosmetic: Vec<(u32, SimEvent)>,
+}
+
+impl ClientSlot {
+    /// The slot.
+    pub fn slot(&self) -> u8 {
+        self.slot
+    }
+
+    /// The running counts.
+    pub fn counters(&self) -> &ClientCounters {
+        &self.counters
+    }
+
+    /// The last tick's server hash (marks/README.md §1's `server_hash`): `None` when the room sent this client
+    /// nothing at that tick (carry 61 (a)).
+    pub fn server_hash(&self) -> Option<u64> {
+        self.t.server_hash
+    }
+
+    /// Whether the client's last acknowledged baseline had left the ring at the last tick (u4-build-plan.md §4.5;
+    /// SERIES.md's `event.burst_eviction.evicted`, written at the first tick after a burst).
+    pub fn burst_eviction(&self) -> bool {
+        self.t.burst_eviction
+    }
+
+    /// The last tick's snapshot's baseline age (0: the per-type defaults); `None` when none was sent.
+    pub fn baseline_age(&self) -> Option<u16> {
+        self.t.baseline_age
+    }
+
+    /// The last tick's snapshot groups.
+    pub fn groups(&self) -> u8 {
+        self.t.groups
+    }
+
+    /// The last tick's snapshot bytes, payload plus 28 B each.
+    pub fn bytes(&self) -> u32 {
+        self.t.bytes
+    }
+
+    /// The last tick's window budget, and its remainder after the pump's bytes (the encoder's budget: §4.5).
+    pub fn budget(&self) -> (u32, u32) {
+        (self.t.budget, self.t.remainder)
+    }
+
+    /// The newest tick whose every group the client acknowledged (baseline B's tick while the ring holds it), held or
+    /// evicted.
+    pub fn last_acked(&self) -> Option<u32> {
+        self.ack.last_acked()
+    }
+
+    /// The own-state section's bits in the last tick's snapshot (carry 56), and the `OwnState` record's share of
+    /// them, and whether that record changed against the baseline.
+    pub fn own_section(&self) -> (u32, u32, bool) {
+        (
+            self.t.own_bits,
+            self.t.own_state_bits,
+            self.t.own_state_changed,
+        )
+    }
+
+    /// Whether the last tick forced an overflow resync (carry 57: its tick is that tick).
+    pub fn overflow_resync(&self) -> bool {
+        self.t.overflow_resync
+    }
+
+    /// The counters that wrapped since the tick before, by their `Wrap` names (-58's `event.counter_wrap`).
+    pub fn wraps(&self) -> &[&'static str] {
+        &self.t.wraps
+    }
+
+    /// The sequences of the last tick's commands that holding or pressing fire got no Rewind (U4E-Q5 (a); carry 40's
+    /// per-shot log).
+    pub fn rewinds_refused(&self) -> &[u16] {
+        &self.t.rewinds_refused
+    }
+
+    /// The changed entities the last tick's snapshot held at the baseline's value.
+    pub fn skipped(&self) -> u32 {
+        self.t.skipped
+    }
+
+    /// The gameplay events queued now and the most queued at once (U4E-Q6 condition 1).
+    pub fn queue(&self) -> (u32, u32) {
+        (self.queue.len(), self.queue.high_water())
+    }
+
+    /// The event queue's cap less its length, signed, at its lowest during the last tick (before an overflow resync
+    /// cleared it): SERIES.md's `tick.v.ack_starved_headroom_count` (U4E-Q12 (c)); below 0 only under
+    /// PM-0001-22/unbounded.
+    pub fn headroom(&self) -> i64 {
+        self.t.headroom
+    }
+
+    /// The input buffer's depth, in ticks ahead, at the last tick (`buffer_depth_ticks`).
+    pub fn buffer_depth_ticks(&self) -> u32 {
+        self.input.buffer_depth_ticks()
+    }
+
+    /// Whether the room dropped the client (`Kick(overflow)`).
+    pub fn kicked(&self) -> bool {
+        self.kicked
+    }
+
+    fn begin_tick(&mut self) {
+        self.t.server_hash = None;
+        self.t.burst_eviction = false;
+        self.t.baseline_age = None;
+        self.t.groups = 0;
+        self.t.bytes = 0;
+        self.t.budget = 0;
+        self.t.remainder = 0;
+        self.t.own_bits = 0;
+        self.t.own_state_bits = 0;
+        self.t.own_state_changed = false;
+        self.t.overflow_resync = false;
+        self.t.skipped = 0;
+        self.t.rewinds_refused.clear();
+        self.t.wraps.clear();
+        self.t.wraps.append(&mut self.pending_wraps);
+        self.cosmetic.clear();
+    }
+
+    fn kick(&mut self, roster: &mut Roster) {
+        self.control.push(Message::Kick(Kick::Overflow));
+        self.kicked = true;
+        self.counters.kicked += 1;
+        roster.leave(self.slot);
+    }
+}
+
+/// `r` as the wire carries it: `area` and `raw` 0.
+fn wire(r: &EntityRecord) -> EntityRecord {
+    EntityRecord {
+        area: 0,
+        raw: [0; EntityRecord::FIELDS],
+        ..*r
+    }
+}
+
+/// A record's point for interest (cm): a soldier's or hull's position, a trajectory's start; none for the rest.
+fn point(r: &EntityRecord) -> Option<[i32; 2]> {
+    match r.kind {
+        EntityKind::Soldier | EntityKind::Vehicle => Some([r.grid[0], r.grid[1]]),
+        EntityKind::Projectile => Some([r.grid[1], r.grid[2]]),
+        _ => None,
+    }
+}
+
+fn dist2(a: [i32; 2], b: [i32; 2]) -> i64 {
+    let (x, y) = (
+        i64::from(a[0]) - i64::from(b[0]),
+        i64::from(a[1]) - i64::from(b[1]),
+    );
+    x * x + y * y
+}
+
+/// Sorts a tick's commands into the canonical order (seam.rs `TickInput::commands`; carry 21): slot ascending, then
+/// command tick ascending within a slot, wrap-aware, so every arrival order gives one `TickInput`.
+pub(crate) fn canonical(commands: &mut [(Command, Option<Rewind>)]) {
+    commands.sort_by(|(a, _), (b, _)| {
+        a.slot
+            .cmp(&b.slot)
+            .then_with(|| (b.tick.wrapping_sub(a.tick) as i32).cmp(&0).reverse())
+    });
+}
+
+/// The tick's records by (kind, index), and the records interest places by a point or by their hull or area.
+#[derive(Clone, Debug, Default)]
+struct Index {
+    by_key: Vec<((EntityKind, u16), usize)>,
+    parts: Vec<usize>,
+    pieces: Vec<usize>,
+}
+
+impl Index {
+    fn get(&self, k: (EntityKind, u16)) -> Option<usize> {
+        self.by_key
+            .binary_search_by_key(&k, |e| e.0)
+            .ok()
+            .map(|i| self.by_key[i].1)
+    }
+}
+
+/// The room (plan.md:596-598): generic over npt-sim's `SimCore`; it owns its `HitHistory`, its roster and a
+/// [`ClientSlot`] per joined client.
+#[derive(Clone, Debug)]
+pub struct Room<S: SimCore> {
+    sim: S,
+    history: HitHistory,
+    roster: Roster,
+    clients: Vec<Option<ClientSlot>>,
+    teams: Vec<u8>,
+    effect_rows: u16,
+    mode_id: u16,
+    content_hash: [u8; 32],
+    mode_hash: [u8; 32],
+    tick_hz: u32,
+    next_tick: u32,
+    next_event: u32,
+    event_cap: u32,
+    event_cap_provisional: bool,
+    cover: CoverVariant,
+    grid: InterestGrid,
+    flood: AreaFlood,
+    quant: Quant,
+    #[cfg(feature = "bites")]
+    bite: Option<RoomBite>,
+    input: TickInput,
+    stamped: Vec<(u32, u32, SimEvent)>,
+    reports: InputReports,
+    index: Index,
+}
+
+impl<S: SimCore> Room<S> {
+    /// A room over `sim` running `scene`, its static graph read from `greybox` (`areas()` and `portals()`: the
+    /// tests pass `World::greybox()`), a roster of `tokens` admitting at most `n` sessions, at `tick_hz` ticks a second
+    /// (production callers pass `npt_sim::TICK_HZ`: the window's rate is an argument, the F2 ruling). Its first tick
+    /// is the one after the tick `sim`'s hit volumes name (a new `World`'s name the tick before its first step).
+    ///
+    /// # Panics
+    /// If a hit history of 2 s (`HISTORY_MS`, provisional until U4h) does not fit npt-sim's 1 MB bound (it does at
+    /// every rate npt-sim allows).
+    pub fn new(
+        sim: S,
+        scene: &Scene,
+        greybox: &Greybox,
+        tokens: Vec<[u8; 16]>,
+        n: usize,
+        tick_hz: u32,
+    ) -> Room<S> {
+        let next_tick = sim.hit_volumes().tick.wrapping_add(1);
+        let areas = greybox.areas();
+        let extent = areas.rects.first().copied().unwrap_or([0, 0, 1, 1]);
+        let mut flood = AreaFlood::new(areas, greybox.portals());
+        flood.update(&sim.portal_states().open);
+        let slots = usize::from(scene.slots());
+        Room {
+            history: HitHistory::for_ms(HISTORY_MS)
+                .expect("a 2 s hit history fits npt-sim's 1 MB bound"),
+            roster: Roster::new(tokens, n),
+            clients: vec![None; slots.max(1)],
+            teams: (0..scene.slots())
+                .map(|s| scene.team(s).unwrap_or(0))
+                .collect(),
+            effect_rows: u16::from(scene.effect_rows()),
+            mode_id: u16::from(scene.id()),
+            content_hash: [0; 32],
+            mode_hash: [0; 32],
+            tick_hz,
+            next_tick,
+            next_event: 0,
+            event_cap: EventQueue::PROVISIONAL_CAP,
+            event_cap_provisional: true,
+            cover: CoverVariant::Rewound,
+            grid: InterestGrid::new(extent, CELL_CM),
+            flood,
+            quant: Quant::default(),
+            #[cfg(feature = "bites")]
+            bite: None,
+            input: TickInput::default(),
+            stamped: Vec::new(),
+            reports: InputReports::default(),
+            index: Index::default(),
+            sim,
+        }
+    }
+
+    /// The same room with the event queue's cap set as data (for the run's header; a test may set a small one). The
+    /// cap then is no longer the provisional one.
+    pub fn with_event_cap(mut self, cap: u32) -> Room<S> {
+        self.event_cap = cap;
+        self.event_cap_provisional = false;
+        self
+    }
+
+    /// The same room with its 32-bit event counter started at `start` (-58's injectable start: carry 23).
+    pub fn with_event_start(mut self, start: u32) -> Room<S> {
+        self.next_event = start;
+        self
+    }
+
+    /// The same room with its content and mode hashes for `JoinAccepted` (the scene data and the build).
+    pub fn with_hashes(mut self, content_hash: [u8; 32], mode_hash: [u8; 32]) -> Room<S> {
+        self.content_hash = content_hash;
+        self.mode_hash = mode_hash;
+        self
+    }
+
+    /// The same room with a cover variant for every Rewind (R-10's two; the room's setting).
+    pub fn with_cover(mut self, cover: CoverVariant) -> Room<S> {
+        self.cover = cover;
+        self
+    }
+
+    /// The same room with one planted defect by its id (bites builds only; u4-build-plan.md §4.11: per instance): one
+    /// of [`crate::BITES`], or one of npt-proto's, whose `Quant` the room's encoder then codes with (U4E-Q4: the room
+    /// forwards the id, never spells it); `None` for an id neither plants.
+    #[cfg(feature = "bites")]
+    pub fn with_bite(mut self, bite: &str) -> Option<Room<S>> {
+        if let Some(b) = RoomBite::of(bite) {
+            self.bite = Some(b);
+            return Some(self);
+        }
+        self.quant = Quant::with_bite(bite)?;
+        Some(self)
+    }
+
+    fn bitten(&self, b: RoomBite) -> bool {
+        #[cfg(feature = "bites")]
+        {
+            self.bite == Some(b)
+        }
+        #[cfg(not(feature = "bites"))]
+        {
+            let _ = b;
+            false
+        }
+    }
+
+    /// The counter one of -58's room bites compares with a plain `<` (u4-build-plan.md §4.14 :467, :468, :471): each id
+    /// maps to one `Wrap` counter name.
+    fn plain_counter(&self) -> Option<&'static str> {
+        [
+            (RoomBite::LtAck, ACK),
+            (RoomBite::LtCmdSeq, CMD_SEQ),
+            (RoomBite::LtPress, PRESS),
+        ]
+        .into_iter()
+        .find(|(b, _)| self.bitten(*b))
+        .map(|(_, counter)| counter)
+    }
+
+    /// The `Wrap` the room's compares run through: the rule, or under one of -58's room bites a plain `<` on its one
+    /// counter.
+    fn wrap(&self) -> Wrap {
+        let plain = self.plain_counter();
+        #[cfg(feature = "bites")]
+        if let Some(counter) = plain {
+            return Wrap::plain_lt(counter);
+        }
+        let _ = plain;
+        Wrap::default()
+    }
+
+    /// The event queue's cap, and whether it is still the provisional value (U4E-Q6 (a); carry 57: for the run's
+    /// header).
+    pub fn event_cap(&self) -> (u32, bool) {
+        (self.event_cap, self.event_cap_provisional)
+    }
+
+    /// The sim.
+    pub fn sim(&self) -> &S {
+        &self.sim
+    }
+
+    /// The roster.
+    pub fn roster(&self) -> &Roster {
+        &self.roster
+    }
+
+    /// The tick the last [`Room::tick`] ran (the one before the first, before any).
+    pub fn last_tick(&self) -> u32 {
+        self.next_tick.wrapping_sub(1)
+    }
+
+    /// The last tick's `TickInput` (what a recording holds: the commands in the canonical order with their Rewinds).
+    pub fn tick_input(&self) -> &TickInput {
+        &self.input
+    }
+
+    /// The last step's clamp reports, refusal records and running counters, read through `SimCore::reports` and kept
+    /// for the logs, never in a hash.
+    pub fn reports(&self) -> &InputReports {
+        &self.reports
+    }
+
+    /// A client's slot, while it is joined (or kicked and not yet gone).
+    pub fn client(&self, slot: u8) -> Option<&ClientSlot> {
+        self.clients.get(usize::from(slot)).and_then(Option::as_ref)
+    }
+
+    /// The datagrams (snapshot groups) the last tick coded for `slot`, in group order.
+    pub fn take_datagrams(&mut self, slot: u8) -> Vec<Vec<u8>> {
+        self.clients
+            .get_mut(usize::from(slot))
+            .and_then(Option::as_mut)
+            .map(|c| core::mem::take(&mut c.datagrams))
+            .unwrap_or_default()
+    }
+
+    /// The reliable messages for `slot` (`JoinAccepted`, `FullState`, `Kick`), each for the channel its row in
+    /// `Limits::TYPES` names.
+    pub fn take_control(&mut self, slot: u8) -> Vec<Message> {
+        self.clients
+            .get_mut(usize::from(slot))
+            .and_then(Option::as_mut)
+            .map(|c| core::mem::take(&mut c.control))
+            .unwrap_or_default()
+    }
+
+    /// The tick's scripted spawns, teleports and attribute changes (integers only: seam.rs), for the next step.
+    pub fn script(
+        &mut self,
+        spawns: &[Placement],
+        teleports: &[Placement],
+        attributes: &[AttributeChange],
+    ) {
+        self.input.spawns.clear();
+        self.input.spawns.extend_from_slice(spawns);
+        self.input.teleports.clear();
+        self.input.teleports.extend_from_slice(teleports);
+        self.input.attributes.clear();
+        self.input.attributes.extend_from_slice(attributes);
+    }
+
+    /// A `Join` the pump passed (its version and content hash already checked there): the roster's checks, then a
+    /// fresh slot with `JoinAccepted` and a full state to follow. Returns the slot and whether it replaced a live
+    /// session (whose connection then gets `Kick(replaced)` from the caller).
+    ///
+    /// # Errors
+    /// The roster's [`JoinRejected`] reason: bad token, bind mismatch, roster full.
+    pub fn join(&mut self, join: &Join) -> Result<(u8, bool), JoinRejected> {
+        let a = self.roster.admit(&join.token, &join.bind)?;
+        let i = usize::from(a.slot);
+        if i >= self.clients.len() {
+            self.roster.leave(a.slot);
+            return Err(JoinRejected::RosterFull);
+        }
+        let mut ack = AckRing::new();
+        if self.bitten(RoomBite::NoFallback) {
+            ack.keep_stale();
+        }
+        let mut c = ClientSlot {
+            slot: a.slot,
+            team: self.teams.get(i).copied().unwrap_or(0),
+            ack,
+            queue: EventQueue::new(self.event_cap, self.bitten(RoomBite::Unbounded)),
+            input: InputBuffer::default(),
+            encoder: Encoder::new(self.tick_hz, self.quant),
+            priority: Priority::default(),
+            full_state_due: Some(false),
+            in_flight: None,
+            kicked: false,
+            server_bytes: 0,
+            max_datagram: Limits::SNAPSHOT,
+            datagrams: Vec::new(),
+            control: Vec::new(),
+            counters: ClientCounters::default(),
+            last_ack_hi: None,
+            pending_wraps: Vec::new(),
+            fire_presses: None,
+            t: TickValues::default(),
+            relevant: Vec::new(),
+            sees: Vec::new(),
+            nearest: Vec::new(),
+            cosmetic: Vec::new(),
+        };
+        c.control.push(Message::JoinAccepted(JoinAccepted {
+            slot: a.slot,
+            team: c.team,
+            squad: 0,
+            mode_id: self.mode_id,
+            mode_hash: self.mode_hash,
+            content_hash: self.content_hash,
+            tick: self.next_tick,
+            tick_hz: u16::try_from(self.tick_hz).unwrap_or(u16::MAX),
+            snapshot_hz: u16::try_from(self.tick_hz).unwrap_or(u16::MAX),
+        }));
+        self.clients[i] = Some(c);
+        Ok((a.slot, a.replaced))
+    }
+
+    /// The slot's session ended (a `Leave`, a closed link): the slot is free, its token and bind value kept.
+    pub fn leave(&mut self, slot: u8) {
+        self.roster.leave(slot);
+        if let Some(c) = self.clients.get_mut(usize::from(slot)) {
+            *c = None;
+        }
+    }
+
+    /// The pump's report for `slot` before a tick (u4-build-plan.md §4.5; plan.md §4.4): the bytes the server side
+    /// sent it since the last tick (pings and pongs, payload plus 28 B each), which the window counts, and the link's
+    /// largest datagram now, which bounds each group.
+    pub fn pump_report(&mut self, slot: u8, sent_bytes: u32, max_datagram: usize) {
+        if let Some(c) = self
+            .clients
+            .get_mut(usize::from(slot))
+            .and_then(Option::as_mut)
+        {
+            c.server_bytes = c.server_bytes.saturating_add(sent_bytes);
+            c.max_datagram = max_datagram;
+        }
+    }
+
+    /// One decoded message from `slot`'s session (the pump decoded it; npt-room sees only decoded messages): `Input`'s
+    /// acknowledgement and commands, `Resync`, `Leave`; the other client types change nothing here at U4e.
+    pub fn receive(&mut self, slot: u8, msg: &Message) {
+        match msg {
+            Message::Input(input) => self.receive_input(slot, input),
+            Message::Resync(_) => {
+                if let Some(c) = self
+                    .clients
+                    .get_mut(usize::from(slot))
+                    .and_then(Option::as_mut)
+                    && !c.kicked
+                {
+                    if c.in_flight.is_some() || c.full_state_due.is_some() {
+                        c.counters.resyncs_dropped += 1;
+                    } else {
+                        c.full_state_due = Some(false);
+                        c.counters.resyncs += 1;
+                    }
+                }
+            }
+            Message::Leave(_) => self.leave(slot),
+            _ => {}
+        }
+    }
+
+    fn receive_input(&mut self, slot: u8, input: &Input) {
+        let w = self.wrap();
+        let Some(c) = self
+            .clients
+            .get_mut(usize::from(slot))
+            .and_then(Option::as_mut)
+        else {
+            return;
+        };
+        if c.kicked {
+            return;
+        }
+        if input.ack_groups != 0 {
+            let mut acked = Vec::new();
+            let answer = match c.ack.newest() {
+                Some(near) => {
+                    let tick = w.widen16(ACK, near, input.ack_tick);
+                    let a = c.ack.ack(tick, input.ack_groups, &mut acked);
+                    if a != Acked::NeverSent {
+                        let hi = (tick >> 16) as u16;
+                        if c.last_ack_hi.is_some_and(|h| h != hi) {
+                            c.pending_wraps.push(ACK);
+                        }
+                        c.last_ack_hi = Some(hi);
+                    }
+                    a
+                }
+                None => Acked::NeverSent,
+            };
+            match answer {
+                Acked::Held => {
+                    c.queue.ack(&acked);
+                    if let Some((f, _)) = c.in_flight
+                        && c.ack.acked(f)
+                    {
+                        c.in_flight = None;
+                    }
+                }
+                Acked::Old => c.counters.acks_old += 1,
+                Acked::NeverSent => {
+                    // The acknowledgement names a snapshot never sent: the datagram's commands are refused (§4.14 :471).
+                    c.counters.ack_refused += 1;
+                    c.counters.ack_refused_commands += input.commands.len() as u64;
+                    return;
+                }
+            }
+        }
+        for cmd in input.commands.iter().rev() {
+            let cmd = Command { slot, ..*cmd };
+            match c.input.offer(cmd, w, &mut c.pending_wraps) {
+                Offered::Buffered => c.counters.commands_buffered += 1,
+                Offered::Repeat => c.counters.commands_repeat += 1,
+                Offered::PressRefused => c.counters.press_refused += 1,
+                Offered::Late => c.counters.late += 1,
+                Offered::Full => c.counters.buffer_full += 1,
+            }
+        }
+    }
+
+    /// Runs one tick: the commands, the step, the history, the flood, the reports, interest, the events and every
+    /// client's snapshot (the module text).
+    pub fn tick(&mut self) {
+        let t = self.next_tick;
+        for c in self.clients.iter_mut().flatten() {
+            c.begin_tick();
+        }
+        self.fill_input(t);
+        // Carry 26: the step's events are copied (stamped with the tick and their 32-bit numbers: carry 23) before
+        // anything else is read from the sim.
+        self.stamped.clear();
+        let events = self.sim.step(&self.input, &self.history);
+        for e in events {
+            self.stamped.push((self.next_event, t, *e));
+            self.next_event = self.next_event.wrapping_add(1);
+        }
+        self.input.spawns.clear();
+        self.input.teleports.clear();
+        self.input.attributes.clear();
+        self.history.push(self.sim.hit_volumes());
+        self.flood.update(&self.sim.portal_states().open);
+        self.reports.clone_from(self.sim.reports());
+        self.index_records();
+        self.find_relevant();
+        self.route_events(t);
+        self.encode_all(t);
+        self.next_tick = t.wrapping_add(1);
+    }
+
+    /// The tick's `TickInput`: per joined client the command option B runs (its slot the session's), with its Rewind
+    /// when it holds or presses fire, in the canonical order.
+    fn fill_input(&mut self, t: u32) {
+        self.input.tick = t;
+        self.input.commands.clear();
+        let fire = Command::FIRE;
+        for c in self.clients.iter_mut().flatten() {
+            if c.kicked {
+                continue;
+            }
+            let taken = c.input.take(t);
+            c.counters.late += u64::from(taken.late);
+            let Some(cmd) = taken.command else {
+                continue;
+            };
+            if taken.repeated {
+                c.counters.commands_rerun += 1;
+            } else {
+                c.counters.commands_run += 1;
+            }
+            let press = cmd.presses[Command::PRESS_FIRE];
+            let pressed = c.fire_presses.is_some_and(|p| p != press)
+                || (c.fire_presses.is_none() && press != 0);
+            c.fire_presses = Some(press);
+            let rewind = if cmd.buttons & fire != 0 || pressed {
+                let r = rewind_at(&cmd, &self.history, self.cover);
+                if r.is_none() {
+                    c.counters.rewinds_refused += 1;
+                    c.t.rewinds_refused.push(cmd.seq);
+                }
+                r
+            } else {
+                None
+            };
+            self.input.commands.push((
+                Command {
+                    slot: c.slot,
+                    ..cmd
+                },
+                rewind,
+            ));
+        }
+        canonical(&mut self.input.commands);
+    }
+
+    fn index_records(&mut self) {
+        let records = self.sim.entity_records();
+        let ix = &mut self.index;
+        ix.by_key.clear();
+        ix.parts.clear();
+        ix.pieces.clear();
+        self.grid.clear();
+        for (i, r) in records.iter().enumerate() {
+            ix.by_key.push((key(r), i));
+            match r.kind {
+                EntityKind::VehiclePart => ix.parts.push(i),
+                EntityKind::Door | EntityKind::Wall => ix.pieces.push(i),
+                _ => {}
+            }
+            if let Some(p) = point(r) {
+                self.grid.insert(p, i as u32);
+            }
+        }
+        ix.by_key.sort_by_key(|e| e.0);
+    }
+
+    /// Each client's relevant records (catalogue §3's spatial rows): soldiers, hulls and trajectories within the
+    /// radius whose area connects to the client's; a part with its hull; a door or the wall in a connected area (a
+    /// portal counts as inside both areas it joins). The nearest 8 enemy soldiers among them, for the priority.
+    fn find_relevant(&mut self) {
+        let records = self.sim.entity_records();
+        let r2 = i64::from(RADIUS_CM) * i64::from(RADIUS_CM);
+        let mut near = Vec::new();
+        let slots = self.teams.len();
+        for c in self.clients.iter_mut().flatten() {
+            c.relevant.clear();
+            c.nearest.clear();
+            c.sees.clear();
+            c.sees.resize(slots, false);
+            if c.kicked {
+                continue;
+            }
+            let Some(me) = self.index.get((EntityKind::Soldier, u16::from(c.slot))) else {
+                continue;
+            };
+            let me = &records[me];
+            let at = [me.grid[0], me.grid[1]];
+            let mut hulls = Vec::new();
+            let mut enemies = Vec::new();
+            self.grid.near(at, RADIUS_CM, &mut near);
+            for &i in &near {
+                let r = &records[i as usize];
+                if r.kind == EntityKind::Soldier && r.index == u16::from(c.slot) {
+                    continue;
+                }
+                let Some(p) = point(r) else { continue };
+                let d = dist2(at, p);
+                if d > r2 || !self.flood.reaches(me.area, r.area) {
+                    continue;
+                }
+                c.relevant.push(i as usize);
+                match r.kind {
+                    EntityKind::Soldier => {
+                        if let Some(s) = c.sees.get_mut(usize::from(r.index)) {
+                            *s = true;
+                        }
+                        if self
+                            .teams
+                            .get(usize::from(r.index))
+                            .is_some_and(|t| *t != c.team)
+                        {
+                            enemies.push((d, r.index));
+                        }
+                    }
+                    EntityKind::Vehicle => hulls.push(r.index),
+                    _ => {}
+                }
+            }
+            for &i in &self.index.parts {
+                if u16::try_from(records[i].grid[5]).is_ok_and(|h| hulls.contains(&h)) {
+                    c.relevant.push(i);
+                }
+            }
+            for &i in &self.index.pieces {
+                let r = &records[i];
+                let seen = match r.kind {
+                    EntityKind::Door => self.flood.sees_portal(me.area, usize::from(r.index)),
+                    _ => self.flood.reaches(me.area, r.area),
+                };
+                if seen {
+                    c.relevant.push(i);
+                }
+            }
+            c.relevant.sort_by_key(|&i| key(&records[i]));
+            enemies.sort_unstable();
+            c.nearest
+                .extend(enemies.iter().take(Priority::NEAREST_ENEMIES).map(|e| e.1));
+        }
+    }
+
+    /// The tick's events to each client (catalogue §3): a gameplay event (hit, kill) into the queue of every client it
+    /// concerns (shooter or victim) or whose interest holds a soldier it names; a cosmetic shot to every client whose
+    /// interest holds the shooter, never the shooter's own (but under PM-0001-62/server-echo). An overflow forces one
+    /// resync, or, inside an unacknowledged forced one, kicks (§4.7).
+    fn route_events(&mut self, t: u32) {
+        let echo = self.bitten(RoomBite::ServerEcho);
+        for c in self.clients.iter_mut().flatten() {
+            if c.kicked {
+                continue;
+            }
+            c.counters.events_expired += u64::from(c.queue.expire(t));
+            let me = c.slot;
+            let sees = |s: u8| c.sees.get(usize::from(s)).copied().unwrap_or(false);
+            let mut gameplay = Vec::new();
+            for &(number, tick, event) in &self.stamped {
+                match event {
+                    SimEvent::Shot { slot, .. } => {
+                        if (slot != me || echo) && (sees(slot) || slot == me) {
+                            c.cosmetic.push((number, event));
+                        }
+                    }
+                    SimEvent::Hit {
+                        shooter, victim, ..
+                    }
+                    | SimEvent::Kill {
+                        killer: shooter,
+                        victim,
+                    } => {
+                        if shooter == me || victim == me || sees(shooter) || sees(victim) {
+                            gameplay.push(Queued {
+                                number,
+                                tick,
+                                event,
+                                sent: 0,
+                            });
+                        }
+                    }
+                }
+            }
+            c.t.headroom = c.queue.headroom();
+            for q in gameplay {
+                if c.queue.push(q).is_ok() {
+                    c.counters.events_queued += 1;
+                    c.t.headroom = c.t.headroom.min(c.queue.headroom());
+                    continue;
+                }
+                // At the cap: the headroom is 0 at the overflow.
+                c.t.headroom = c.t.headroom.min(0);
+                if c.full_state_due == Some(true) || c.in_flight.is_some_and(|f| f.1) {
+                    // A second overflow inside an unacknowledged forced resync: Kick(overflow), the client dropped.
+                    c.kick(&mut self.roster);
+                } else {
+                    // One resync, sent this tick: the full state holds this tick's results and clears the queue
+                    // (catalogue Q8), so the rest of this tick's events are not queued.
+                    c.queue.clear();
+                    c.full_state_due = Some(true);
+                    c.in_flight = None;
+                    c.counters.overflow_resyncs += 1;
+                    c.t.overflow_resync = true;
+                }
+                break;
+            }
+        }
+    }
+
+    fn encode_all(&mut self, t: u32) {
+        let owner_to_all = self.bitten(RoomBite::OwnerToAll);
+        let no_fallback = self.bitten(RoomBite::NoFallback);
+        let records = self.sim.entity_records();
+        let slots = self.teams.len();
+        let full_ack = npt_sim::FULL_STATE_ACK_TICKS;
+        for c in self.clients.iter_mut().flatten() {
+            if c.kicked {
+                continue;
+            }
+            // The window moves on every tick, and the pump's bytes count against this tick's whether or not a
+            // snapshot goes out (u4-build-plan.md §4.5).
+            let (budget, left) = c.encoder.remainder(c.server_bytes);
+            c.server_bytes = 0;
+            c.t.budget = budget;
+            c.t.remainder = left;
+            c.t.burst_eviction = c.ack.evicted(t);
+            if let Some((f, true)) = c.in_flight
+                && t.wrapping_sub(f) > full_ack
+                && !c.ack.acked(f)
+            {
+                c.kick(&mut self.roster);
+                continue;
+            }
+            // The own-state section's slot: the client's own (carry 53), or under the owner-to-all bite the next one's.
+            let owner = if owner_to_all && slots > 1 {
+                ((usize::from(c.slot) + 1) % slots) as u16
+            } else {
+                u16::from(c.slot)
+            };
+            let mut own = Vec::new();
+            let mut push = |k: (EntityKind, u16)| {
+                if let Some(i) = self.index.get(k) {
+                    own.push(records[i]);
+                }
+            };
+            push((EntityKind::Soldier, owner));
+            // Carry 51: effect row i belongs to slot i / Scene::effect_rows().
+            let rows = self.effect_rows;
+            for i in owner.saturating_mul(rows)..owner.saturating_add(1).saturating_mul(rows) {
+                push((EntityKind::Effect, i));
+            }
+            push((EntityKind::OwnState, owner));
+            if let Some(forced) = c.full_state_due {
+                full_state(c, t, forced, records, &own);
+                continue;
+            }
+            if c.in_flight.is_some() {
+                continue;
+            }
+            snapshot(c, t, records, &own, no_fallback);
+        }
+    }
+}
+
+/// Sends `c` a full state at tick `t`: its own-state records and its relevant records whole (catalogue §3's
+/// `FullState` rows), recorded in the ring as one acknowledged-group tick, the queue cleared.
+fn full_state(
+    c: &mut ClientSlot,
+    t: u32,
+    forced: bool,
+    records: &[EntityRecord],
+    own: &[EntityRecord],
+) {
+    let mut frame = Frame {
+        own: own.iter().map(wire).collect(),
+        entities: c.relevant.iter().map(|&i| wire(&records[i])).collect(),
+    };
+    frame.own.sort_by_key(key);
+    frame.entities.sort_by_key(key);
+    let fs = FullState {
+        tick: t,
+        last_run: c.input.last_run().map(|cmd| cmd.seq),
+        own: frame.own.clone(),
+        entities: frame.entities.clone(),
+    };
+    let msg = Message::FullState(fs);
+    let mut probe = Vec::new();
+    if let Err(e) = c.encoder.quant.encode(&msg, &mut probe) {
+        count_refusal(&mut c.counters, e);
+        return;
+    }
+    c.full_state_due = None;
+    c.in_flight = Some((t, forced));
+    c.queue.clear();
+    c.counters.full_states += 1;
+    c.t.server_hash = Some(client_hash(t, &frame.entities, &frame.own, &[]));
+    c.ack.record(t, 1, frame, Vec::new());
+    c.control.push(msg);
+}
+
+fn count_refusal(counters: &mut ClientCounters, e: EncodeError) {
+    match e {
+        EncodeError::OutOfRange => counters.refused_out_of_range += 1,
+        EncodeError::TooMany => counters.refused_too_many += 1,
+        EncodeError::TooLarge => counters.refused_too_large += 1,
+        EncodeError::NotCanonical => counters.refused_not_canonical += 1,
+    }
+}
+
+/// One entity change waiting for room in the snapshot, with its priority.
+struct Candidate {
+    delta: EntityDelta,
+    id: (EntityKind, u16),
+    nearest: bool,
+    skipped: u32,
+    dist: i64,
+}
+
+/// Codes `c`'s snapshot of tick `t` against its baseline (u4-build-plan.md §4.5), within the window's remainder, and
+/// keeps the mirror's hash of what it means the client to hold (§4.6). A refusal by the encoder skips this tick's
+/// snapshot for this client, counted by kind (carry 54), and the next tick codes afresh.
+fn snapshot(
+    c: &mut ClientSlot,
+    t: u32,
+    records: &[EntityRecord],
+    own: &[EntityRecord],
+    no_fallback: bool,
+) {
+    let left = c.t.remainder;
+    let empty = Frame::default();
+    let (base, age) = match c.ack.baseline(t) {
+        Some(b) => (&b.frame, t.wrapping_sub(b.tick) as u16),
+        None => match c.ack.stale() {
+            // PM-0001-18/no-fallback: never fall back to the defaults, coding against the evicted baseline.
+            Some(s) if no_fallback => (&s.frame, t.wrapping_sub(s.tick).max(1) as u16),
+            _ => (&empty, 0),
+        },
+    };
+    let from_defaults = age == 0;
+    let limit = c.max_datagram.min(Limits::SNAPSHOT) as u32;
+    let Some(mut p) = Packer::new(limit, left) else {
+        c.counters.budget_short += 1;
+        return;
+    };
+    let mut groups_own = Vec::new();
+    let mut own_bits = 0;
+    for r in own {
+        if let Some(d) = EntityDelta::between(r, find(&base.own, key(r))) {
+            let bits = delta_bits(&d);
+            own_bits += bits;
+            if r.kind == EntityKind::OwnState {
+                c.t.own_state_bits = bits;
+                c.t.own_state_changed = true;
+            }
+            groups_own.push((d, bits));
+        }
+    }
+    for b in &base.own {
+        if !own.iter().any(|r| key(r) == key(b)) {
+            let d = EntityDelta::Remove {
+                kind: b.kind,
+                index: b.index,
+            };
+            own_bits += delta_bits(&d);
+            groups_own.push((d, delta_bits(&d)));
+        }
+    }
+    for (d, bits) in groups_own {
+        if p.place(bits, Section::Own).is_none() {
+            c.counters.budget_short += 1;
+            return;
+        }
+        p.groups[0].own.push(d);
+    }
+    c.t.own_bits = own_bits;
+    // Gameplay events, oldest first, resent until acknowledged.
+    let (mut sent, mut resent) = (0, 0);
+    for q in c.queue.iter_mut() {
+        let Some(g) = p.place(event_bits(&q.event), Section::Events) else {
+            break;
+        };
+        let age = t.wrapping_sub(q.tick) as u8;
+        p.groups[g].events.push((q.number as u16, age, q.event));
+        p.groups[g].numbers.push(q.number);
+        p.groups[g].gameplay.push(q.number);
+        q.sent += 1;
+        if q.sent == 1 {
+            sent += 1;
+        } else {
+            resent += 1;
+        }
+    }
+    // Entity changes: the nearest 8 enemies first, then by snapshots skipped and distance; then the removals.
+    let me = own.first().map_or([0, 0], |r| [r.grid[0], r.grid[1]]);
+    let mut candidates = Vec::new();
+    let mut held = Vec::new();
+    for &i in &c.relevant {
+        let r = &records[i];
+        let id = key(r);
+        match EntityDelta::between(r, find(&base.entities, id)) {
+            Some(delta) => candidates.push(Candidate {
+                delta,
+                id,
+                nearest: r.kind == EntityKind::Soldier && c.nearest.contains(&r.index),
+                skipped: c.priority.skipped(id),
+                dist: point(r).map_or(0, |q| dist2(me, q)),
+            }),
+            None => held.push(id),
+        }
+    }
+    for b in &base.entities {
+        let id = key(b);
+        if c.relevant
+            .binary_search_by_key(&id, |&i| key(&records[i]))
+            .is_err()
+        {
+            candidates.push(Candidate {
+                delta: EntityDelta::Remove {
+                    kind: b.kind,
+                    index: b.index,
+                },
+                id,
+                nearest: false,
+                skipped: Priority::EVERY,
+                dist: 0,
+            });
+        }
+    }
+    candidates.sort_by(|a, b| {
+        b.nearest
+            .cmp(&a.nearest)
+            .then((b.skipped >= Priority::EVERY - 1).cmp(&(a.skipped >= Priority::EVERY - 1)))
+            .then(b.skipped.cmp(&a.skipped))
+            .then(a.dist.cmp(&b.dist))
+            .then(a.id.cmp(&b.id))
+    });
+    let mut skipped = 0;
+    for cand in candidates {
+        match p.place(delta_bits(&cand.delta), Section::Entities) {
+            Some(g) => {
+                p.groups[g].entities.push(cand.delta);
+                c.priority.sent(cand.id);
+            }
+            None => {
+                c.priority.skip(cand.id);
+                skipped += 1;
+            }
+        }
+    }
+    for id in held {
+        c.priority.sent(id);
+    }
+    // The accumulator keeps no entity the client no longer has in view (bounded by the sim's entities).
+    c.priority.retain(|id| {
+        c.relevant
+            .binary_search_by_key(&id, |&i| key(&records[i]))
+            .is_ok()
+    });
+    // Cosmetic events, sent once if there is room.
+    for &(number, event) in &c.cosmetic {
+        match p.place(event_bits(&event), Section::Events) {
+            Some(g) => {
+                p.groups[g].events.push((number as u16, 0, event));
+                p.groups[g].numbers.push(number);
+                c.counters.cosmetic_sent += 1;
+            }
+            None => c.counters.cosmetic_skipped += 1,
+        }
+    }
+    // Code each group; any refusal skips the whole tick for this client (carry 54).
+    let n = p.groups.len() as u8;
+    let mut datagrams = Vec::with_capacity(p.groups.len());
+    let mut bytes = 0;
+    let last_run = c.input.last_run().map(|cmd| cmd.seq);
+    for (i, g) in p.groups.iter().enumerate() {
+        let msg = Message::Snapshot(Snapshot {
+            tick: t,
+            baseline_age: age,
+            newest_received: c.input.newest_received(),
+            last_run,
+            group: i as u8,
+            groups: n,
+            own: g.own.clone(),
+            entities: g.entities.clone(),
+            events: g.events.clone(),
+        });
+        let mut out = Vec::new();
+        let coded = c.encoder.quant.encode(&msg, &mut out).and_then(|()| {
+            debug_assert_eq!(
+                out.len() as u32,
+                group_bytes(g.bits),
+                "the size arithmetic is the encoding's"
+            );
+            if out.len() as u32 > limit {
+                Err(EncodeError::TooLarge)
+            } else {
+                Ok(())
+            }
+        });
+        if let Err(e) = coded {
+            count_refusal(&mut c.counters, e);
+            return;
+        }
+        bytes += out.len() as u32 + DATAGRAM_OVERHEAD;
+        datagrams.push(out);
+    }
+    // The mirror (§4.6's server side): the baseline plus every group's delta, the events as sent.
+    let frame = view(base, &p.groups);
+    let events: Vec<(u32, SimEvent)> = p
+        .groups
+        .iter()
+        .flat_map(|g| g.numbers.iter().zip(&g.events).map(|(n, e)| (*n, e.2)))
+        .collect();
+    c.t.server_hash = Some(client_hash(t, &frame.entities, &frame.own, &events));
+    let acks: Vec<(u8, u32)> = p
+        .groups
+        .iter()
+        .enumerate()
+        .flat_map(|(i, g)| g.gameplay.iter().map(move |n| (i as u8, *n)))
+        .collect();
+    if from_defaults {
+        c.counters.defaults_coded += 1;
+    } else if c.ack.baseline(t).is_none() {
+        c.counters.stale_coded += 1;
+    }
+    c.ack.record(t, n, frame, acks);
+    c.counters.snapshots += 1;
+    c.counters.groups += u64::from(n);
+    c.counters.split_ticks += u64::from(n > 1);
+    c.counters.skipped += u64::from(skipped);
+    c.counters.events_sent += sent;
+    c.counters.events_resent += resent;
+    c.t.skipped = skipped;
+    c.t.groups = n;
+    c.t.bytes = bytes;
+    c.t.baseline_age = Some(age);
+    c.datagrams = datagrams;
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use npt_sim::World;
+
+    fn cmd(slot: u8, tick: u32) -> (Command, Option<Rewind>) {
+        (
+            Command {
+                slot,
+                tick,
+                ..Command::default()
+            },
+            None,
+        )
+    }
+
+    /// Carry 21: commands received in two orders give one canonical list, two of one slot whose ticks straddle the
+    /// 32-bit tick wrap among them (seam.rs `TickInput::commands`: wrap-aware within a slot).
+    #[test]
+    fn carry_21_two_arrival_orders_sort_to_one_canonical_list_across_the_32_bit_wrap() {
+        let mut a = vec![cmd(3, 1), cmd(0, 7), cmd(3, u32::MAX), cmd(1, 5), cmd(3, 0)];
+        let mut b = a.clone();
+        b.reverse();
+        canonical(&mut a);
+        canonical(&mut b);
+        assert_eq!(a, b);
+        let order: Vec<(u8, u32)> = a.iter().map(|(c, _)| (c.slot, c.tick)).collect();
+        assert_eq!(order, [(0, 7), (1, 5), (3, u32::MAX), (3, 0), (3, 1)]);
+    }
+
+    /// Carry 23: the room stamps each event with the `TickInput::tick` it ran and consecutive 32-bit numbers, across a
+    /// 16-bit wrap from an injectable start a few below it; each client's queue holds them with those stamps.
+    #[test]
+    fn carry_23_events_carry_their_tick_and_consecutive_32_bit_numbers_across_a_16_bit_wrap() {
+        let scene = Scene::stress();
+        let world = World::new(&scene, 7, 900);
+        let greybox = world.greybox().clone();
+        let start = u32::from(u16::MAX) - 20;
+        let tokens: Vec<[u8; 16]> = (0..48u8).map(|s| [s; 16]).collect();
+        let mut room = Room::new(world, &scene, &greybox, tokens, 48, 30).with_event_start(start);
+        for s in 0..48u8 {
+            assert!(
+                room.join(&Join {
+                    token: [s; 16],
+                    ..Join::default()
+                })
+                .is_ok()
+            );
+        }
+        let rec = npt_sim::replay::Recording::synthetic(&scene, 7, 900, 100, 2_000);
+        let mut next = start;
+        let mut stamps: Vec<(u32, u32)> = Vec::new();
+        for _ in 0..60 {
+            let t = room.last_tick().wrapping_add(1);
+            for input in rec.ticks.iter().filter(|i| i.tick == t) {
+                for (c, _) in &input.commands {
+                    let msg = Message::Input(Input {
+                        commands: vec![*c],
+                        ..Input::default()
+                    });
+                    room.receive(c.slot, &msg);
+                }
+            }
+            room.tick();
+            let t = room.last_tick();
+            for &(number, tick, _) in &room.stamped {
+                assert_eq!(
+                    (number, tick),
+                    (next, t),
+                    "consecutive numbers, the tick run"
+                );
+                next = next.wrapping_add(1);
+                stamps.push((number, tick));
+            }
+            if let Some(c) = room.clients[0].as_mut() {
+                for q in c.queue.iter_mut() {
+                    assert!(stamps.contains(&(q.number, q.tick)), "{q:?}");
+                }
+            }
+        }
+        assert!(
+            next > u32::from(u16::MAX) + 1,
+            "the numbers crossed the 16-bit wrap: {next}"
+        );
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/roster.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/roster.rs
new file mode 100644
index 0000000..fbcd89a
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/roster.rs
@@ -0,0 +1,187 @@
+//! The roster (catalogue §2, `Join`'s row and its notes on `bind`; plan §10.3): per slot its test token, the bind value
+//! it took at its first accepted `Join`, and whether a session holds it. The caller passes the tokens in (the lab at
+//! U4r; minting is U6's).
+
+use npt_proto::JoinRejected;
+
+/// One slot of the roster. No `Debug`: it holds the slot's secrets (catalogue: nothing is echoed back; plan E22).
+#[derive(Clone)]
+struct Entry {
+    token: [u8; 16],
+    bind: Option<[u8; 16]>,
+    live: bool,
+}
+
+/// 1 when the two 16-byte values are equal, else 0, in constant time: every byte is read, the differences are folded
+/// into one byte, and that byte becomes the answer by arithmetic, never by a branch on the bytes (catalogue :93: "the
+/// test token compared in constant time").
+fn same(a: &[u8; 16], b: &[u8; 16]) -> usize {
+    let mut d = 0u8;
+    for (x, y) in a.iter().zip(b) {
+        d |= x ^ y;
+    }
+    let d = core::hint::black_box(d);
+    usize::from((u16::from(d).wrapping_sub(1) >> 8) as u8 & 1)
+}
+
+/// The roster: a token per slot, at most `n` sessions at once (no join beyond the roster's N: catalogue :93). Its
+/// `Debug` shows how many slots are bound and live, never a token or a bind value.
+#[derive(Clone)]
+pub struct Roster {
+    slots: Vec<Entry>,
+    n: usize,
+}
+
+impl core::fmt::Debug for Roster {
+    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
+        f.debug_struct("Roster")
+            .field("slots", &self.slots.len())
+            .field(
+                "bound",
+                &self.slots.iter().filter(|s| s.bind.is_some()).count(),
+            )
+            .field("live", &self.live())
+            .field("n", &self.n)
+            .finish()
+    }
+}
+
+/// What [`Roster`] made of an accepted `Join`: the slot, and whether a live session held it (a replacement: the old
+/// session gets `Kick(replaced)`, catalogue :93).
+pub(crate) struct Admitted {
+    pub(crate) slot: u8,
+    pub(crate) replaced: bool,
+}
+
+impl Roster {
+    /// A roster of one slot per token, slot `i` holding `tokens[i]`, admitting at most `n` sessions at once. At most 256
+    /// slots are kept (a slot is a `u8`).
+    pub fn new(mut tokens: Vec<[u8; 16]>, n: usize) -> Roster {
+        tokens.truncate(usize::from(u8::MAX) + 1);
+        Roster {
+            slots: tokens
+                .into_iter()
+                .map(|token| Entry {
+                    token,
+                    bind: None,
+                    live: false,
+                })
+                .collect(),
+            n,
+        }
+    }
+
+    /// The sessions live now.
+    pub fn live(&self) -> usize {
+        self.slots.iter().filter(|s| s.live).count()
+    }
+
+    /// Whether a session holds `slot` now.
+    pub fn is_live(&self, slot: u8) -> bool {
+        self.slots.get(usize::from(slot)).is_some_and(|s| s.live)
+    }
+
+    /// Whether `slot`'s token is bound to a client's bind value (at its first accepted `Join`).
+    pub fn is_bound(&self, slot: u8) -> bool {
+        self.slots
+            .get(usize::from(slot))
+            .is_some_and(|s| s.bind.is_some())
+    }
+
+    /// The room's checks of a `Join` (catalogue :93): the token against every slot's, each compared in constant time
+    /// and the whole roster read whatever matches; the bind value against the one the slot took at its first accepted
+    /// `Join`; no join beyond `n` live sessions (a replacement of a live session adds none). The version and the content
+    /// hash are npt-server's (checked before the room sees the `Join`).
+    pub(crate) fn admit(
+        &mut self,
+        token: &[u8; 16],
+        bind: &[u8; 16],
+    ) -> Result<Admitted, JoinRejected> {
+        let mut hit = 0usize;
+        for (i, s) in self.slots.iter().enumerate() {
+            let take = same(&s.token, token) & usize::from(hit == 0);
+            hit |= take.wrapping_neg() & (i + 1);
+        }
+        let Some(i) = hit.checked_sub(1) else {
+            return Err(JoinRejected::BadToken);
+        };
+        let live = self.live();
+        let s = &mut self.slots[i];
+        if let Some(bound) = &s.bind
+            && same(bound, bind) == 0
+        {
+            return Err(JoinRejected::BindMismatch);
+        }
+        let replaced = s.live;
+        if !replaced && live >= self.n {
+            return Err(JoinRejected::RosterFull);
+        }
+        s.bind = Some(*bind);
+        s.live = true;
+        Ok(Admitted {
+            slot: i as u8,
+            replaced,
+        })
+    }
+
+    /// The slot's session ended (a `Leave`, a closed link, a kick); the slot keeps its token and bind value for a
+    /// rejoin (catalogue `Leave`'s row).
+    pub(crate) fn leave(&mut self, slot: u8) {
+        if let Some(s) = self.slots.get_mut(usize::from(slot)) {
+            s.live = false;
+        }
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    fn token(i: u8) -> [u8; 16] {
+        [i; 16]
+    }
+
+    #[test]
+    fn same_is_one_for_equal_values_and_zero_for_any_one_bit_differing() {
+        let a = [7u8; 16];
+        assert_eq!(same(&a, &a), 1);
+        for i in 0..16 {
+            for bit in 0..8 {
+                let mut b = a;
+                b[i] ^= 1 << bit;
+                assert_eq!(same(&a, &b), 0, "byte {i} bit {bit}");
+            }
+        }
+    }
+
+    /// e9's room-side JoinRejected reasons (the version and content hash are the pump's: server/tests).
+    #[test]
+    fn e9_the_room_side_join_reasons_bad_token_bind_mismatch_and_roster_full() {
+        let mut r = Roster::new(vec![token(1), token(2), token(3)], 2);
+        assert_eq!(
+            r.admit(&token(9), &[0; 16]).err(),
+            Some(JoinRejected::BadToken)
+        );
+        let a = r.admit(&token(1), &[5; 16]).expect("slot 0");
+        assert_eq!((a.slot, a.replaced), (0, false));
+        assert!(r.is_bound(0) && r.is_live(0));
+        r.leave(0);
+        // The slot keeps its bind value: another bind is refused, the same one admitted again.
+        assert_eq!(
+            r.admit(&token(1), &[6; 16]).err(),
+            Some(JoinRejected::BindMismatch)
+        );
+        assert_eq!(r.admit(&token(1), &[5; 16]).map(|a| a.slot).ok(), Some(0));
+        assert_eq!(r.admit(&token(2), &[1; 16]).map(|a| a.slot).ok(), Some(1));
+        // Two live of n = 2: a third slot's valid join is refused; a replacement of a live one is not.
+        assert_eq!(
+            r.admit(&token(3), &[1; 16]).err(),
+            Some(JoinRejected::RosterFull)
+        );
+        let again = r.admit(&token(2), &[1; 16]).expect("a replacement");
+        assert_eq!((again.slot, again.replaced), (1, true));
+        assert_eq!(r.live(), 2);
+        let shown = format!("{r:?}");
+        assert_eq!(shown, "Roster { slots: 3, bound: 2, live: 2, n: 2 }");
+    }
+}
````

### npt-room's clippy.toml
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/room/clippy.toml .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/clippy.toml`:  1 file changed, 5 insertions(+), 1 deletion(-)._

````diff
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/room/clippy.toml b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/clippy.toml
index deb9534..e288057 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/room/clippy.toml
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/clippy.toml
@@ -1,7 +1,11 @@
-# The clock ban (plan §4.5): npt-room's timers count ticks; wall time reaches it only from npt-server, passed in.
+# The clock ban (plan §4.5): npt-room's timers count ticks; wall time reaches it only from npt-server, passed in. The
+# hash-map type bans (u4-build-plan.md §4.11; U4E-Q8 (a)): the room's maps and sets are ordered (BTreeMap) or indexed,
+# so no iteration order depends on a hasher's seed.
 disallowed-types = [
     { path = "std::time::Instant", reason = "the clock ban: time is passed in" },
     { path = "std::time::SystemTime", reason = "the clock ban: time is passed in" },
+    { path = "std::collections::HashMap", reason = "the hash-map ban: iteration order must not depend on a seed" },
+    { path = "std::collections::HashSet", reason = "the hash-map ban: iteration order must not depend on a seed" },
 ]
 disallowed-methods = [
     { path = "std::time::Instant::now", reason = "the clock ban" },
````

### npt-server's library (crates/server/src)
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/server/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src`:  2 files changed, 436 insertions(+), 3 deletions(-)._

````diff
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/server/src/lib.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/lib.rs
index b394380..1e0d231 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/server/src/lib.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/lib.rs
@@ -4,11 +4,14 @@
 //! is decoded), the wall-time limits from an injected `now`, the per-slot local control sockets, metrics and logs, and
 //! the panic catch. npt-lab imports this library so L1 runs the real server code on simulated time. Built in U4 and U6.
 //!
-//! Built so far: the loop policy (U4b, [`schedule`]): [`Schedule`] and [`due_ticks`], tested on a fake clock with
-//! PM-0004-67's three bites.
+//! Built so far: the loop policy (U4b): [`Schedule`] and [`due_ticks`], tested on a fake clock with PM-0004-67's three
+//! bites; and the I/O step (U4e): [`Decoder::pump`] with its [`ConnLimits`] and [`Counters`]. Each item has one path,
+//! at the crate root (W1-PUB's nit: the modules are private).
 
-pub mod schedule;
+mod pump;
+mod schedule;
 
+pub use pump::{ConnLimits, Counters, Decoder};
 #[cfg(feature = "bites")]
 pub use schedule::BITES;
 pub use schedule::{Dropped, Due, LoopCounters, Schedule, due_ticks};
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/pump.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/pump.rs
new file mode 100644
index 0000000..9cb3e5a
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/pump.rs
@@ -0,0 +1,430 @@
+//! The I/O step (u4-build-plan.md §4.1, §4.4 A26, §4.5, §4.9; catalogue §1 and §2): [`Decoder::pump`] decodes every
+//! inbound message of one session through npt-proto's `Quant::decode` with the `Quant` it was given, counts each
+//! refusal by kind, applies the size and channel checks at the socket ([`DecodeError`]'s socket kinds), the Input
+//! bucket, the Ping limit and the Join deadline on the injected `now`, sends the room's pings and answers the client's
+//! with their hold time, and keeps the session's RTT and the bytes the server side sent it since the last tick. It
+//! holds no `Link` (§4.11: so it can be `Send`); the caller lends one per call.
+
+use std::collections::VecDeque;
+
+use npt_link::{Channel, Link};
+use npt_proto::{DecodeError, Direction, Join, JoinRejected, Kick, Message, Ping, Pong, Quant};
+
+/// The Join deadline: 2 s from the connection's accept (catalogue :93), in ns.
+const JOIN_DEADLINE_NS: u64 = 2_000_000_000;
+/// The room's ping interval: a few a second (catalogue :91), in ns.
+const PING_INTERVAL_NS: u64 = 250_000_000;
+/// One second, in ns.
+const SECOND_NS: u64 = 1_000_000_000;
+/// The Input bucket's size (catalogue :87: "a bucket of 8 datagrams").
+const INPUT_BUCKET: u64 = 8;
+/// The Ping limit (catalogue :91: "10 a second per session"), as a bucket of 10 refilled at 10 a second.
+const PINGS_A_SECOND: u64 = 10;
+/// Outstanding pings kept, at most; an older one's `Pong` is then not outstanding.
+const OUTSTANDING: usize = 8;
+/// RTT samples kept for the lowest-recent estimate.
+const SAMPLES: usize = 8;
+/// What each datagram costs the window beyond its payload (catalogue :88).
+const DATAGRAM_OVERHEAD: u32 = 28;
+
+/// A token bucket on the injected clock, exact in integers: its level counts tokens in units of 10^-9 token, so a
+/// refill of `rate` tokens a second adds `rate` units per ns.
+#[derive(Clone, Copy, Debug)]
+struct Bucket {
+    cap: u64,
+    rate: u64,
+    level: u64,
+    at_ns: u64,
+}
+
+impl Bucket {
+    fn full(cap: u64, rate: u64, now_ns: u64) -> Bucket {
+        Bucket {
+            cap,
+            rate,
+            level: cap.saturating_mul(SECOND_NS),
+            at_ns: now_ns,
+        }
+    }
+
+    /// Takes one token at `now_ns` if the bucket holds one.
+    fn take(&mut self, now_ns: u64) -> bool {
+        let elapsed = now_ns.saturating_sub(self.at_ns);
+        self.at_ns = self.at_ns.max(now_ns);
+        self.level = self
+            .level
+            .saturating_add(elapsed.saturating_mul(self.rate))
+            .min(self.cap.saturating_mul(SECOND_NS));
+        if self.level >= SECOND_NS {
+            self.level -= SECOND_NS;
+            true
+        } else {
+            false
+        }
+    }
+}
+
+/// One connection's wall-time limits on the injected clock (catalogue :87, :91, :93): the Input bucket (8 datagrams,
+/// refilled at 2 × the tick rate a second), the Ping limit (10 a second) and the Join deadline (2 s from the accept).
+/// The tick rate is an argument (the F2 ruling): production callers pass `npt_sim::TICK_HZ`.
+#[derive(Clone, Copy, Debug)]
+pub struct ConnLimits {
+    tick_hz: u32,
+    input: Bucket,
+    ping: Bucket,
+    deadline_ns: u64,
+}
+
+impl ConnLimits {
+    /// The limits of a connection accepted at `accept_ns`, at `tick_hz` ticks a second.
+    pub fn new(tick_hz: u32, accept_ns: u64) -> ConnLimits {
+        let tick_hz = tick_hz.max(1);
+        ConnLimits {
+            tick_hz,
+            input: Bucket::full(INPUT_BUCKET, 2 * u64::from(tick_hz), accept_ns),
+            ping: Bucket::full(PINGS_A_SECOND, PINGS_A_SECOND, accept_ns),
+            deadline_ns: accept_ns.saturating_add(JOIN_DEADLINE_NS),
+        }
+    }
+
+    /// The Input bucket's refill, datagrams a second: 2 × the tick rate (60 at 30 Hz, 120 at 60 Hz).
+    pub fn input_refill_per_second(&self) -> u64 {
+        self.input.rate
+    }
+
+    /// The most of a `Pong`'s hold time the RTT subtracts: one tick period (catalogue :92: "33.3 ms at 30 Hz"), ns.
+    pub fn hold_cap_ns(&self) -> u64 {
+        SECOND_NS / u64::from(self.tick_hz)
+    }
+
+    /// When a connection with no `Join` is closed, ns.
+    pub fn join_deadline_ns(&self) -> u64 {
+        self.deadline_ns
+    }
+}
+
+/// One session's refusal and traffic counts (PM-0001-21's counters; U4f adds their -21 series: U4E-Q11).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub struct Counters {
+    /// `decode` refusals by kind: above the size the receiving end takes ...
+    pub too_large: u64,
+    /// ... ending inside a field ...
+    pub truncated: u64,
+    /// ... a stream message whose length prefix does not count its bytes ...
+    pub bad_length: u64,
+    /// ... a type byte no message has ...
+    pub unknown_type: u64,
+    /// ... a room → client type from a client (forged) ...
+    pub wrong_direction: u64,
+    /// ... a known type on a channel it does not travel on ...
+    pub wrong_channel: u64,
+    /// ... bytes after the last field ...
+    pub trailing_bytes: u64,
+    /// ... a reserved or pad bit set ...
+    pub unknown_field: u64,
+    /// ... a code outside its field ...
+    pub out_of_range: u64,
+    /// ... and well framed but not a message.
+    pub malformed: u64,
+    /// Datagrams before the session's `Join` was accepted: dropped (ADR 0001:446).
+    pub before_join: u64,
+    /// `Input` datagrams over the bucket: dropped.
+    pub input_limited: u64,
+    /// `Ping`s over the limit: dropped.
+    pub ping_limited: u64,
+    /// `Pong`s naming no outstanding ping of this session: dropped.
+    pub pong_unknown: u64,
+    /// Sessions closed as malformed because their first stream message was not a `Join` (never INVALID: the run
+    /// verdict is the judge's).
+    pub first_not_join: u64,
+    /// Sessions closed as malformed for a stream message that failed its framing, size or type checks.
+    pub stream_malformed: u64,
+    /// Sessions closed for no `Join` within 2 s of the accept.
+    pub join_deadline: u64,
+    /// `Join`s refused for the protocol version ...
+    pub wrong_version: u64,
+    /// ... or the content hash.
+    pub wrong_content_hash: u64,
+    /// A second `Join` on a session already joining or joined: dropped.
+    pub join_again: u64,
+    /// Pings the room sent.
+    pub pings_sent: u64,
+    /// Pongs the room sent.
+    pub pongs_sent: u64,
+}
+
+impl Counters {
+    fn refused(&mut self, e: DecodeError) {
+        match e {
+            DecodeError::TooLarge => self.too_large += 1,
+            DecodeError::Truncated => self.truncated += 1,
+            DecodeError::BadLength => self.bad_length += 1,
+            DecodeError::UnknownType => self.unknown_type += 1,
+            DecodeError::WrongDirection => self.wrong_direction += 1,
+            DecodeError::WrongChannel => self.wrong_channel += 1,
+            DecodeError::TrailingBytes => self.trailing_bytes += 1,
+            DecodeError::UnknownField => self.unknown_field += 1,
+            DecodeError::OutOfRange => self.out_of_range += 1,
+            DecodeError::Malformed => self.malformed += 1,
+        }
+    }
+}
+
+/// Where a session is in its life.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum State {
+    AwaitJoin,
+    /// The `Join` went to the room; its answer (`Decoder::accept` or `Decoder::reject`) is due.
+    Joining,
+    Joined,
+    Closed,
+}
+
+/// One session's decoder and I/O step (the module text).
+#[derive(Clone, Debug)]
+pub struct Decoder {
+    quant: Quant,
+    limits: ConnLimits,
+    counters: Counters,
+    version: u16,
+    content_hash: [u8; 32],
+    state: State,
+    next_ping_ns: u64,
+    ping_id: u8,
+    outstanding: VecDeque<(u8, u64)>,
+    samples: VecDeque<u64>,
+    sent_bytes: u32,
+}
+
+impl Decoder {
+    /// The decoder of a session accepted at `accept_ns`, decoding with `quant` (the wire's, `Quant::default()`; in a
+    /// bites build one of npt-proto's planted defects through `Quant::with_bite`), at `tick_hz` ticks a second
+    /// (production callers pass `npt_sim::TICK_HZ`), taking `Join`s of protocol `version` and `content_hash` only.
+    pub fn new(
+        quant: Quant,
+        tick_hz: u32,
+        accept_ns: u64,
+        version: u16,
+        content_hash: [u8; 32],
+    ) -> Decoder {
+        Decoder {
+            quant,
+            limits: ConnLimits::new(tick_hz, accept_ns),
+            counters: Counters::default(),
+            version,
+            content_hash,
+            state: State::AwaitJoin,
+            next_ping_ns: accept_ns,
+            ping_id: 0,
+            outstanding: VecDeque::new(),
+            samples: VecDeque::new(),
+            sent_bytes: 0,
+        }
+    }
+
+    /// The session's counts.
+    pub fn counters(&self) -> &Counters {
+        &self.counters
+    }
+
+    /// The session's limits.
+    pub fn limits(&self) -> &ConnLimits {
+        &self.limits
+    }
+
+    /// Whether the pump closed the session (the caller drops its link).
+    pub fn is_closed(&self) -> bool {
+        self.state == State::Closed
+    }
+
+    /// Whether the room accepted the session's `Join`.
+    pub fn is_joined(&self) -> bool {
+        self.state == State::Joined
+    }
+
+    /// The room accepted the `Join` the pump passed on: datagrams are taken from now on.
+    pub fn accept(&mut self) {
+        if self.state == State::Joining {
+            self.state = State::Joined;
+        }
+    }
+
+    /// The room refused the `Join`: `JoinRejected` with its reason, then the session closes (catalogue :96).
+    pub fn reject(&mut self, link: &mut dyn Link, reason: JoinRejected, now_ns: u64) {
+        self.send_control(link, &Message::JoinRejected(reason), now_ns);
+        self.state = State::Closed;
+    }
+
+    /// The bytes the server side sent this session since the last call (pings and pongs, payload plus 28 B each: the
+    /// room's window counts them, u4-build-plan.md §4.5).
+    pub fn take_sent_bytes(&mut self) -> u32 {
+        core::mem::take(&mut self.sent_bytes)
+    }
+
+    /// The session's RTT (u4-build-plan.md §4.9 :352-353): the link's smoothed estimate for an L1 or `wt` session
+    /// (Loopback's is 0), else the lowest recent sample of the room's own pings, each less the client's hold time up
+    /// to one tick period.
+    pub fn rtt_ns(&self, link: &dyn Link) -> Option<u64> {
+        link.rtt_ns().or_else(|| self.samples.iter().copied().min())
+    }
+
+    fn send_control(&mut self, link: &mut dyn Link, msg: &Message, now_ns: u64) {
+        let mut out = Vec::new();
+        if npt_proto::encode(msg, &mut out).is_ok() {
+            link.send_reliable(Channel::Control, &out, now_ns);
+        }
+    }
+
+    fn send_datagram(&mut self, link: &mut dyn Link, msg: &Message, now_ns: u64) {
+        let mut out = Vec::new();
+        if npt_proto::encode(msg, &mut out).is_ok() {
+            link.send_unreliable(&out, now_ns);
+            self.sent_bytes = self
+                .sent_bytes
+                .saturating_add(out.len() as u32 + DATAGRAM_OVERHEAD);
+        }
+    }
+
+    /// Closes the session as malformed, with `Kick(malformed)` while the stream still works (catalogue §1).
+    fn close_malformed(&mut self, link: &mut dyn Link, now_ns: u64) {
+        self.send_control(link, &Message::Kick(Kick::Malformed), now_ns);
+        self.state = State::Closed;
+    }
+
+    /// The I/O step at `now_ns` (the room at `tick`): the deadline, the ping sender, then every message that has
+    /// arrived, decoded and checked; what the room takes goes onto `out` with its arrival time. After a `Join` the
+    /// step returns, so the room answers it before the session's next message is read.
+    pub fn pump(
+        &mut self,
+        link: &mut dyn Link,
+        now_ns: u64,
+        tick: u32,
+        out: &mut Vec<(Message, u64)>,
+    ) {
+        if self.state == State::Closed {
+            return;
+        }
+        if self.state == State::AwaitJoin && now_ns >= self.limits.deadline_ns {
+            self.counters.join_deadline += 1;
+            self.state = State::Closed;
+            return;
+        }
+        if self.state == State::Joined && now_ns >= self.next_ping_ns {
+            let id = self.ping_id;
+            self.ping_id = id.wrapping_add(1);
+            self.outstanding.push_back((id, now_ns));
+            if self.outstanding.len() > OUTSTANDING {
+                self.outstanding.pop_front();
+            }
+            self.send_datagram(
+                link,
+                &Message::Ping(Ping {
+                    id,
+                    sent_ns: now_ns,
+                }),
+                now_ns,
+            );
+            self.counters.pings_sent += 1;
+            self.next_ping_ns = now_ns.saturating_add(PING_INTERVAL_NS);
+        }
+        while let Some(r) = link.poll_recv(now_ns) {
+            let stream = r.channel != Channel::Datagram;
+            let msg = match self
+                .quant
+                .decode(Direction::ClientToRoom, r.channel, &r.bytes)
+            {
+                Ok(m) => m,
+                Err(e) => {
+                    self.counters.refused(e);
+                    // A forged kind of a known type within its limits is refused and counted, never kicked
+                    // (u4-build-plan.md §4.10); any other bad stream message closes the session.
+                    if stream
+                        && !matches!(e, DecodeError::WrongChannel | DecodeError::WrongDirection)
+                    {
+                        self.counters.stream_malformed += 1;
+                        self.close_malformed(link, now_ns);
+                        return;
+                    }
+                    continue;
+                }
+            };
+            match (self.state, msg) {
+                (State::AwaitJoin, Message::Join(j)) => {
+                    if !self.join_ok(link, &j, now_ns) {
+                        return;
+                    }
+                    self.state = State::Joining;
+                    out.push((Message::Join(j), r.arrival_ns));
+                    return;
+                }
+                (State::AwaitJoin, _) if stream => {
+                    self.counters.first_not_join += 1;
+                    self.close_malformed(link, now_ns);
+                    return;
+                }
+                (State::AwaitJoin | State::Joining, _) if !stream => self.counters.before_join += 1,
+                (_, Message::Join(_)) => self.counters.join_again += 1,
+                (State::Joined, Message::Input(i)) => {
+                    if self.limits.input.take(now_ns) {
+                        out.push((Message::Input(i), r.arrival_ns));
+                    } else {
+                        self.counters.input_limited += 1;
+                    }
+                }
+                (State::Joined, Message::Ping(p)) => {
+                    if self.limits.ping.take(now_ns) {
+                        let hold = now_ns.saturating_sub(r.arrival_ns) / 1_000;
+                        let pong = Pong {
+                            id: p.id,
+                            echo_ns: p.sent_ns,
+                            tick,
+                            hold_us: u16::try_from(hold).unwrap_or(u16::MAX),
+                        };
+                        self.send_datagram(link, &Message::Pong(pong), now_ns);
+                        self.counters.pongs_sent += 1;
+                    } else {
+                        self.counters.ping_limited += 1;
+                    }
+                }
+                (State::Joined, Message::Pong(p)) => self.pong(&p, now_ns),
+                (State::Joined, m) => out.push((m, r.arrival_ns)),
+                _ => {}
+            }
+        }
+    }
+
+    /// The server's half of the `Join` checks (catalogue :93): the protocol version and the content hash.
+    fn join_ok(&mut self, link: &mut dyn Link, j: &Join, now_ns: u64) -> bool {
+        let reason = if j.version != self.version {
+            self.counters.wrong_version += 1;
+            JoinRejected::WrongVersion
+        } else if j.content_hash != self.content_hash {
+            self.counters.wrong_content_hash += 1;
+            JoinRejected::WrongContentHash
+        } else {
+            return true;
+        };
+        self.reject(link, reason, now_ns);
+        false
+    }
+
+    /// A `Pong` of one of the room's outstanding pings: one RTT sample, the hold time subtracted up to one tick
+    /// period (catalogue :92); any other id is dropped and counted.
+    fn pong(&mut self, p: &Pong, now_ns: u64) {
+        let Some(at) = self
+            .outstanding
+            .iter()
+            .position(|&(id, sent)| id == p.id && sent == p.echo_ns)
+        else {
+            self.counters.pong_unknown += 1;
+            return;
+        };
+        self.outstanding.remove(at);
+        let hold = (u64::from(p.hold_us) * 1_000).min(self.limits.hold_cap_ns());
+        let sample = now_ns.saturating_sub(p.echo_ns).saturating_sub(hold);
+        self.samples.push_back(sample);
+        if self.samples.len() > SAMPLES {
+            self.samples.pop_front();
+        }
+    }
+}
````

### npt-client's sources (crates/client/src)
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/client/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src`:  1 file changed, 785 insertions(+), 6 deletions(-)._

````diff
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/src/lib.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src/lib.rs
index e59d87d..cc67c02 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/src/lib.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src/lib.rs
@@ -4,19 +4,798 @@
 //! stamp, and `step` returns the datagrams and reliable messages to send, the block for the page, and when the next
 //! tick is due. Prediction and reconciliation, interpolation, clock sync, the aim cast and the block writer live here.
 //! No bindings, no transport, no clock; native for the bots and wasm32 for the Worker. Built in units U4 and U7.
+//!
+//! U4e (u4-build-plan.md §5 U4e) builds a minimal [`ClientEndpoint`]: decoding into integer frames and held floats,
+//! the complete-tick rule and the sentinel (§4.6), the client side of `client_hash`, the acknowledgement with its
+//! group mask, the rebuilds of the header's 16-bit sequences and of the event numbers, and the answers to the room's
+//! pings. Prediction, interpolation and the clock are U4f's and U4g's.
+
+use std::collections::VecDeque;
+
+use npt_link::Channel;
+use npt_proto::{
+    DecodeError, Direction, EntityDelta, FullState, Input, Join, Limits, Message, Pong, Quant,
+    Snapshot, Wrap, client_hash,
+};
+use npt_sim::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
+use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};
 
 /// The bite ids this crate plants (bites builds only). u4-build-plan.md §4.14 plants each bite where its code lives:
 /// PM-0001-17/client-rounding's site is the client's call into the wire grid, so its id lives here and not in npt-sim's
-/// grid. U4e's endpoint takes the grid for its every call into it from [`grid_with_bite`].
+/// grid; the endpoint takes the grid for its every call into it from [`grid_with_bite`]. PM-0001-58/lt-tick-offset and
+/// PM-0001-58/lt-event-seq are the endpoint's two rebuilds (carry 24).
 #[cfg(feature = "bites")]
-pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];
+pub const BITES: [&str; 3] = [
+    "PM-0001-17/client-rounding",
+    "PM-0001-58/lt-tick-offset",
+    "PM-0001-58/lt-event-seq",
+];
 
 /// The grid a client with the planted defect `bite` uses for its every call into it (bites builds only): for
 /// PM-0001-17/client-rounding ("a client quantiser that rounds differently from the server's") the grid's one defect,
-/// `npt_sim::grid::Config::floor_rounding`; `None` for an id this crate does not plant.
+/// `npt_sim::grid::Config::floor_rounding`; `None` for an id this crate does not plant there.
 #[cfg(feature = "bites")]
 pub fn grid_with_bite(bite: &str) -> Option<npt_sim::grid::Config> {
-    BITES
-        .contains(&bite)
-        .then(npt_sim::grid::Config::floor_rounding)
+    (bite == BITES[0]).then(npt_sim::grid::Config::floor_rounding)
+}
+
+/// The last-run rebuild's counter name in `Wrap` (PM-0001-58/lt-tick-offset: u4-build-plan.md §4.14 :469).
+const TICK_OFFSET: &str = "tick_offset";
+/// The event-number rebuild's counter name (PM-0001-58/lt-event-seq: §4.14 :470).
+const EVENT_SEQ: &str = "event_seq";
+/// The newest-received rebuild's counter name (no bite of its own).
+const CMD_SEQ: &str = "cmd_seq";
+
+/// The fields that hold a float on the client (the grid's five float kinds); every other field is an integer.
+fn is_float(f: Field) -> bool {
+    [POSITION_XY, POSITION_Z, YAW, PITCH, VELOCITY].contains(&f)
+}
+
+/// One record as the client holds it: its integers as decoded (the delta baseline), and, for each float field, the
+/// float it holds for interpolation and prediction (`from_grid` of the integer; 0 for an integer field).
+#[derive(Clone, Copy, Debug, PartialEq)]
+struct Held {
+    rec: EntityRecord,
+    held: [f32; EntityRecord::FIELDS],
+}
+
+impl Held {
+    fn of(rec: EntityRecord, grid: Config) -> Held {
+        let mut held = [0.0; EntityRecord::FIELDS];
+        for ((h, f), &k) in held.iter_mut().zip(rec.kind.layout()).zip(&rec.grid) {
+            if is_float(*f) {
+                *h = grid.from_grid(*f, k);
+            }
+        }
+        Held { rec, held }
+    }
+
+    /// The record the hash reads: each float field's held value turned back into a grid integer through `grid`
+    /// (u4-build-plan.md §4.6: "the floats it holds ... turned back into grid integers through grid").
+    fn hashed(&self, grid: Config) -> EntityRecord {
+        let mut r = self.rec;
+        for ((g, f), &h) in r
+            .grid
+            .iter_mut()
+            .zip(self.rec.kind.layout())
+            .zip(&self.held)
+        {
+            if is_float(*f) {
+                *g = grid.to_grid(*f, h);
+            }
+        }
+        r
+    }
+}
+
+fn key(h: &Held) -> (EntityKind, u16) {
+    (h.rec.kind, h.rec.index)
+}
+
+/// A view at one tick, each list sorted by (kind, index).
+#[derive(Clone, Debug, Default, PartialEq)]
+struct Frame {
+    entities: Vec<Held>,
+    own: Vec<Held>,
+}
+
+/// `list` after one delta, an update read against `base`'s record (each group decodes alone against one baseline).
+fn apply(
+    list: &mut Vec<Held>,
+    base: &[Held],
+    d: &EntityDelta,
+    grid: Config,
+) -> Result<(), DecodeError> {
+    let at = |list: &[Held], k: (EntityKind, u16)| list.binary_search_by_key(&k, key);
+    match d {
+        EntityDelta::Remove { kind, index } => {
+            if let Ok(i) = at(list, (*kind, *index)) {
+                list.remove(i);
+            }
+            Ok(())
+        }
+        EntityDelta::Add(r) => {
+            let h = Held::of(d.apply(None)?.unwrap_or(*r), grid);
+            match at(list, key(&h)) {
+                Ok(i) => list[i] = h,
+                Err(i) => list.insert(i, h),
+            }
+            Ok(())
+        }
+        EntityDelta::Update { kind, index, .. } => {
+            let b = at(base, (*kind, *index)).ok().map(|i| &base[i].rec);
+            let r = d.apply(b)?.ok_or(DecodeError::Malformed)?;
+            let h = Held::of(r, grid);
+            match at(list, key(&h)) {
+                Ok(i) => list[i] = h,
+                Err(i) => list.insert(i, h),
+            }
+            Ok(())
+        }
+    }
+}
+
+/// What the client has made of one tick.
+#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
+enum Filed {
+    #[default]
+    Pending,
+    Hash(u64),
+    Sentinel,
+}
+
+/// One tick in the client's frame ring.
+#[derive(Clone, Debug, Default)]
+struct Tick {
+    tick: u32,
+    used: bool,
+    groups: u8,
+    got: u8,
+    sigs: [u64; Limits::GROUPS],
+    base_age: Option<u16>,
+    frame: Frame,
+    events: Vec<(u32, SimEvent)>,
+    filed: Filed,
+}
+
+impl Tick {
+    fn complete(&self) -> bool {
+        self.groups > 0 && u16::from(self.got) == (1u16 << self.groups) - 1
+    }
+}
+
+/// FNV-1a 64 of a datagram's bytes: a duplicated datagram is the same bytes (carry 61 (b): filed once).
+fn signature(bytes: &[u8]) -> u64 {
+    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
+    for &b in bytes {
+        h ^= u64::from(b);
+        h = h.wrapping_mul(0x0000_0100_0000_01b3);
+    }
+    h
+}
+
+/// What one `step` gives the host: the datagrams and reliable messages to send, when the next step is due, and what
+/// U4r's logs write for this client (U4E-Q7): each hash filed under the tick it rebuilt (the sentinel 0 for a tick with
+/// a received group it could not decode), the ticks resolved lost (`tick.s.snap_incomplete` 1) as (first tick,
+/// count), and the counters that wrapped by their `Wrap` names (-58's `event.counter_wrap`).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub struct StepOut {
+    /// The datagrams to send, in order (the `Input`, then the `Pong`s).
+    pub datagrams: Vec<Vec<u8>>,
+    /// The reliable messages, each with its channel (the `Join`).
+    pub reliable: Vec<(Channel, Vec<u8>)>,
+    /// When the next step is due, in ns on the host's clock.
+    pub next_due_ns: u64,
+    /// (tick, hash) for each tick filed since the last step; hash 0 is the sentinel.
+    pub hashes: Vec<(u32, u64)>,
+    /// (first tick, count) for each run of ticks resolved lost since the last step.
+    pub lost: Vec<(u32, u32)>,
+    /// The counters that wrapped since the last step.
+    pub wraps: Vec<&'static str>,
+}
+
+/// The client endpoint (plan.md:486-489): `input` with the page's command and every message received since the last
+/// tick as (channel, bytes, arrival ns), then `step`. Every time is integer ns on the host's clock; it reads no clock.
+/// npt-client names nothing from npt-link but `Channel` (check.mjs; catalogue item 2).
+#[derive(Clone, Debug)]
+pub struct ClientEndpoint {
+    join: Join,
+    join_sent: bool,
+    slot: Option<u8>,
+    closed: bool,
+    tick_hz: u32,
+    lead: u32,
+    quant: Quant,
+    grid: Config,
+    wrap: Wrap,
+    ring: Vec<Tick>,
+    newest: Option<u32>,
+    low: u32,
+    ack: Option<(u32, u8)>,
+    next_cmd: u32,
+    cmd_sent: bool,
+    unacked: VecDeque<(u32, Command)>,
+    newest_received: Option<u32>,
+    last_run: Option<u32>,
+    event_near: u32,
+    pings: Vec<(u8, u64, u64)>,
+    now_ns: u64,
+    base_ns: Option<u64>,
+    steps: u64,
+    wrong_channel: u64,
+    refused: u64,
+    stale: u64,
+    duplicates: u64,
+    unknown_pongs: u64,
+    out: StepOut,
+}
+
+/// Pings the endpoint holds to answer at its next step, at most (the room sends a few a second).
+const PINGS_HELD: usize = 8;
+
+impl ClientEndpoint {
+    /// An endpoint that joins with `join` (its first step sends it), stepping at `tick_hz` (production callers pass
+    /// `npt_sim::TICK_HZ`), on the wire's `Quant`, grid and `Wrap`.
+    pub fn new(join: Join, tick_hz: u32) -> ClientEndpoint {
+        ClientEndpoint {
+            join,
+            join_sent: false,
+            slot: None,
+            closed: false,
+            tick_hz: tick_hz.max(1),
+            lead: 1,
+            quant: Quant::default(),
+            grid: Config::default(),
+            wrap: Wrap::default(),
+            ring: vec![Tick::default(); npt_sim::ACK_RING_TICKS as usize],
+            newest: None,
+            low: 0,
+            ack: None,
+            next_cmd: 0,
+            cmd_sent: false,
+            unacked: VecDeque::new(),
+            newest_received: None,
+            last_run: None,
+            event_near: 0,
+            pings: Vec::new(),
+            now_ns: 0,
+            base_ns: None,
+            steps: 0,
+            wrong_channel: 0,
+            refused: 0,
+            stale: 0,
+            duplicates: 0,
+            unknown_pongs: 0,
+            out: StepOut::default(),
+        }
+    }
+
+    /// The same endpoint stamping each command `lead` ticks after the newest tick it holds (the option-B buffer's
+    /// lead; U4f's controller replaces it).
+    pub fn with_lead(mut self, lead: u32) -> ClientEndpoint {
+        self.lead = lead;
+        self
+    }
+
+    /// The same endpoint with its command counter started at `start` (its low 16 bits the first sequence) and its
+    /// event rebuild started near `event_start`, the room's event counter's start (-58's injectable starts).
+    pub fn with_starts(mut self, start: u32, event_start: u32) -> ClientEndpoint {
+        self.next_cmd = start;
+        self.event_near = event_start;
+        self
+    }
+
+    /// The same endpoint with one planted defect by its id (bites builds only; per instance): one of [`BITES`]
+    /// (the grid of [`grid_with_bite`], or a plain `<` on the rebuild it names), or one of npt-proto's, whose `Quant`
+    /// the endpoint decodes with (U4E-Q4); `None` for an id neither plants.
+    #[cfg(feature = "bites")]
+    pub fn with_bite(mut self, bite: &str) -> Option<ClientEndpoint> {
+        if let Some(grid) = grid_with_bite(bite) {
+            self.grid = grid;
+        } else if bite == BITES[1] {
+            self.wrap = Wrap::plain_lt(TICK_OFFSET);
+        } else if bite == BITES[2] {
+            self.wrap = Wrap::plain_lt(EVENT_SEQ);
+        } else {
+            self.quant = Quant::with_bite(bite)?;
+        }
+        Some(self)
+    }
+
+    /// The slot `JoinAccepted` gave, once joined.
+    pub fn slot(&self) -> Option<u8> {
+        self.slot
+    }
+
+    /// Whether the room ended the session (`JoinRejected`, `Kick` or `RoomEnded`).
+    pub fn closed(&self) -> bool {
+        self.closed
+    }
+
+    /// The newest snapshot tick the client has read in a header.
+    pub fn newest_tick(&self) -> Option<u32> {
+        self.newest
+    }
+
+    /// The command the header's last-run sequence names, rebuilt to the client's own 32-bit command counter (U4E-Q2
+    /// (c): where U4g's replay starts).
+    pub fn last_run_command(&self) -> Option<u32> {
+        self.last_run
+    }
+
+    /// Messages dropped for arriving on a channel their type does not travel on (catalogue item 2: "on both sides").
+    pub fn wrong_channel(&self) -> u64 {
+        self.wrong_channel
+    }
+
+    /// Received messages refused by the decoder or by the baseline rule (a snapshot's tick then gets the sentinel).
+    pub fn refused(&self) -> u64 {
+        self.refused
+    }
+
+    /// Snapshot groups too old for the frame ring, dropped.
+    pub fn stale(&self) -> u64 {
+        self.stale
+    }
+
+    /// Duplicated datagrams (a group already decoded, the same bytes), filed once.
+    pub fn duplicates(&self) -> u64 {
+        self.duplicates
+    }
+
+    /// `Pong`s whose ping id is not one of the client's own (it sends none at U4e), dropped.
+    pub fn unknown_pongs(&self) -> u64 {
+        self.unknown_pongs
+    }
+
+    /// The page's command for the next step (its slot, tick and sequence are the endpoint's to stamp; `None`: no new
+    /// command), and every message received since the last call, each as (channel, bytes, arrival ns), at `now_ns`.
+    pub fn input(
+        &mut self,
+        command: Option<Command>,
+        received: &[(Channel, Vec<u8>, u64)],
+        now_ns: u64,
+    ) {
+        self.now_ns = now_ns;
+        self.base_ns.get_or_insert(now_ns);
+        for (channel, bytes, arrival_ns) in received {
+            self.receive(*channel, bytes, *arrival_ns);
+        }
+        if let (Some(c), Some(slot)) = (command, self.slot) {
+            let index = self.next_cmd;
+            self.next_cmd = index.wrapping_add(1);
+            self.cmd_sent = true;
+            let tick = self.newest.map_or(0, |n| n.wrapping_add(self.lead));
+            let mut c = Command {
+                slot,
+                tick,
+                seq: index as u16,
+                ..c
+            };
+            // A view time the wire cannot carry (after the command's tick, or more than 65,535 ticks before it: the
+            // `Input` codec's view offset) is taken as the command's own tick, so every `Input` encodes.
+            if tick.wrapping_sub(c.view_tick) > u32::from(u16::MAX) {
+                c.view_tick = tick;
+                c.view_fraction = 0;
+            }
+            self.unacked.push_front((index, c));
+            self.unacked.truncate(Limits::REDUNDANCY_CAP);
+        }
+    }
+
+    /// The step: the `Join` first, then each tick the `Input` (the newest command and every unacknowledged one, and the
+    /// acknowledgement of the newest snapshot tick a group of which decoded, with its group mask), the `Pong`s, and the
+    /// next due time.
+    pub fn step(&mut self) -> StepOut {
+        if !self.join_sent {
+            let mut out = Vec::new();
+            if npt_proto::encode(&Message::Join(self.join), &mut out).is_ok() {
+                self.out.reliable.push((Channel::Control, out));
+            }
+            self.join_sent = true;
+        }
+        if self.slot.is_some() && !self.closed {
+            let (ack_tick, ack_groups) = self.ack.map_or((0, 0), |(t, m)| (t as u16, m));
+            if ack_groups != 0 || !self.unacked.is_empty() {
+                let input = Input {
+                    ack_tick,
+                    ack_groups,
+                    commands: self.unacked.iter().map(|(_, c)| *c).collect(),
+                };
+                let mut out = Vec::new();
+                if npt_proto::encode(&Message::Input(input), &mut out).is_ok() {
+                    self.out.datagrams.push(out);
+                }
+            }
+            for (id, echo_ns, arrival_ns) in self.pings.drain(..) {
+                let hold = self.now_ns.saturating_sub(arrival_ns) / 1_000;
+                let pong = Pong {
+                    id,
+                    echo_ns,
+                    tick: self.newest.unwrap_or(0),
+                    hold_us: u16::try_from(hold).unwrap_or(u16::MAX),
+                };
+                let mut out = Vec::new();
+                if npt_proto::encode(&Message::Pong(pong), &mut out).is_ok() {
+                    self.out.datagrams.push(out);
+                }
+            }
+        }
+        self.steps += 1;
+        let base = self.base_ns.unwrap_or(self.now_ns);
+        let since = u128::from(self.steps) * 1_000_000_000 / u128::from(self.tick_hz);
+        self.out.next_due_ns = base.saturating_add(u64::try_from(since).unwrap_or(u64::MAX));
+        core::mem::take(&mut self.out)
+    }
+
+    fn receive(&mut self, channel: Channel, bytes: &[u8], arrival_ns: u64) {
+        match self.quant.decode(Direction::RoomToClient, channel, bytes) {
+            Ok(Message::Snapshot(s)) => self.group(s, bytes),
+            Ok(Message::FullState(f)) => self.full_state(f),
+            Ok(Message::JoinAccepted(a)) => self.slot = Some(a.slot),
+            Ok(Message::JoinRejected(_) | Message::Kick(_) | Message::RoomEnded(_)) => {
+                self.closed = true
+            }
+            Ok(Message::Ping(p)) => {
+                if self.pings.len() < PINGS_HELD {
+                    self.pings.push((p.id, p.sent_ns, arrival_ns));
+                }
+            }
+            Ok(Message::Pong(_)) => self.unknown_pongs += 1,
+            Ok(_) => {}
+            Err(DecodeError::WrongChannel) => self.wrong_channel += 1,
+            Err(_) => {
+                self.refused += 1;
+                // A snapshot datagram the client could not decode (a size or decode error): the sentinel under the
+                // tick its header names, read whole from its first four bytes after the type (snapshot.rs's wire text).
+                let snapshot = Limits::TYPES
+                    .iter()
+                    .position(|r| r.0 == "Snapshot")
+                    .map(|i| i as u8 + 1);
+                if channel == Channel::Datagram
+                    && bytes.first().copied() == snapshot
+                    && let Some(t) = bytes.get(1..5)
+                {
+                    let t = u32::from_le_bytes([t[0], t[1], t[2], t[3]]);
+                    self.sentinel(t);
+                }
+            }
+        }
+    }
+
+    /// Whether `t` is too old for the ring: at least its length older than the newest tick held.
+    fn too_old(&self, t: u32) -> bool {
+        self.newest.is_some_and(|n| {
+            !self.wrap.after32("tick", n, t) && n.wrapping_sub(t) >= npt_sim::ACK_RING_TICKS
+        })
+    }
+
+    /// The ring slot for `t`, reset for it if it held another tick.
+    fn slot_for(&mut self, t: u32) -> usize {
+        let i = (t % npt_sim::ACK_RING_TICKS) as usize;
+        if !self.ring[i].used || self.ring[i].tick != t {
+            self.ring[i] = Tick {
+                tick: t,
+                used: true,
+                ..Tick::default()
+            };
+        }
+        i
+    }
+
+    /// A header with tick `t`: if it is the newest, every tick it pushes out of the ring is resolved, lost unless filed
+    /// (carry 61 (c): only once none of its groups can still decode), and every tick between the old newest and it
+    /// that no group named, too (a tick none of whose groups arrived counts, known from a later header's tick).
+    fn see(&mut self, t: u32) {
+        let len = npt_sim::ACK_RING_TICKS;
+        let w = Wrap::default();
+        let Some(n) = self.newest else {
+            self.newest = Some(t);
+            self.low = t;
+            return;
+        };
+        if !w.after32("tick", n, t) {
+            return;
+        }
+        self.newest = Some(t);
+        let cutoff = t.wrapping_sub(len - 1);
+        let seen_end = n.wrapping_add(1);
+        let ring_end = if w.after32("tick", seen_end, cutoff) {
+            seen_end
+        } else {
+            cutoff
+        };
+        while w.after32("tick", self.low, ring_end) {
+            let k = self.low;
+            let e = &self.ring[(k % len) as usize];
+            if !(e.used && e.tick == k && e.filed != Filed::Pending) {
+                self.lost(k, 1);
+            }
+            self.low = k.wrapping_add(1);
+        }
+        if w.after32("tick", self.low, cutoff) {
+            self.lost(self.low, cutoff.wrapping_sub(self.low));
+            self.low = cutoff;
+        }
+    }
+
+    fn lost(&mut self, first: u32, count: u32) {
+        if let Some(last) = self.out.lost.last_mut()
+            && last.0.wrapping_add(last.1) == first
+        {
+            last.1 += count;
+            return;
+        }
+        self.out.lost.push((first, count));
+    }
+
+    /// The sentinel for tick `t` (a received group it could not decode: §4.6), once.
+    fn sentinel(&mut self, t: u32) {
+        if self.too_old(t) {
+            self.stale += 1;
+            return;
+        }
+        self.see(t);
+        let i = self.slot_for(t);
+        if self.ring[i].filed != Filed::Sentinel {
+            self.ring[i].filed = Filed::Sentinel;
+            self.out.hashes.push((t, 0));
+        }
+    }
+
+    /// Files `hash` for the tick in slot `i`: once for one hash; a second, different hash makes it the sentinel
+    /// (carry 61 (b)).
+    fn file(&mut self, i: usize, hash: u64) {
+        let t = self.ring[i].tick;
+        match self.ring[i].filed {
+            Filed::Pending => {
+                self.ring[i].filed = Filed::Hash(hash);
+                self.out.hashes.push((t, hash));
+            }
+            Filed::Hash(h) if h != hash => {
+                self.ring[i].filed = Filed::Sentinel;
+                self.out.hashes.push((t, 0));
+            }
+            _ => {}
+        }
+    }
+
+    fn acknowledge(&mut self, t: u32, group: u8) {
+        let w = Wrap::default();
+        match self.ack {
+            Some((a, m)) if a == t => self.ack = Some((a, m | 1 << group)),
+            Some((a, _)) if !w.after32("tick", a, t) => {}
+            _ => self.ack = Some((t, 1 << group)),
+        }
+    }
+
+    /// The header's two command sequences rebuilt to the client's own 32-bit command counter: the newest received
+    /// ends the repeats; the last run names the command U4g's replay starts from (U4E-Q2 (c); W1-N2).
+    fn rebuild_sequences(&mut self, newest_received: Option<u16>, last_run: Option<u16>) {
+        if !self.cmd_sent {
+            return;
+        }
+        // Nearest the newest command the client has stamped.
+        let near = self.next_cmd.wrapping_sub(1);
+        let w = self.wrap;
+        if let Some(low) = newest_received {
+            let n = w.widen16(CMD_SEQ, near, low);
+            if self
+                .newest_received
+                .is_none_or(|p| Wrap::default().after32("tick", p, n))
+            {
+                self.newest_received = Some(n);
+                self.unacked
+                    .retain(|(i, _)| Wrap::default().after32("tick", n, *i));
+            }
+        }
+        if let Some(low) = last_run {
+            let r = w.widen16(TICK_OFFSET, near, low);
+            if self
+                .last_run
+                .is_none_or(|p| Wrap::default().after32("tick", p, r))
+            {
+                if self.last_run.is_some_and(|p| p >> 16 != r >> 16) {
+                    self.out.wraps.push(TICK_OFFSET);
+                }
+                self.last_run = Some(r);
+            }
+        }
+    }
+
+    /// An event's 32-bit number from its 16 bits, nearest the newest number the client knows (`Wrap::widen16`).
+    fn event_number(&mut self, low: u16) -> u32 {
+        let n = self.wrap.widen16(EVENT_SEQ, self.event_near, low);
+        if Wrap::default().after32("tick", self.event_near, n) {
+            if n >> 16 != self.event_near >> 16 {
+                self.out.wraps.push(EVENT_SEQ);
+            }
+            self.event_near = n;
+        }
+        n
+    }
+
+    /// One snapshot group (u4-build-plan.md §4.6): decoded alone against its baseline (the defaults at age 0), its
+    /// tick complete once every group has; a group it cannot decode gives the tick the sentinel.
+    fn group(&mut self, s: Snapshot, bytes: &[u8]) {
+        let t = s.tick;
+        if self.too_old(t) {
+            self.stale += 1;
+            return;
+        }
+        self.see(t);
+        let i = self.slot_for(t);
+        if self.ring[i].filed == Filed::Sentinel {
+            return;
+        }
+        let g = usize::from(s.group);
+        let sig = signature(bytes);
+        if self.ring[i].got & (1 << g) != 0 {
+            if self.ring[i].sigs[g] == sig {
+                self.duplicates += 1;
+            } else {
+                self.refused += 1;
+                self.sentinel(t);
+            }
+            return;
+        }
+        let e = &self.ring[i];
+        if (e.groups != 0 && e.groups != s.groups)
+            || e.base_age.is_some_and(|a| a != s.baseline_age)
+        {
+            self.refused += 1;
+            self.sentinel(t);
+            return;
+        }
+        // The baseline: the defaults at age 0, else the complete frame `baseline_age` ticks before, if the ring holds
+        // it; else the group is refused (no baseline) and the tick gets the sentinel.
+        let base = if s.baseline_age == 0 {
+            Some(Frame::default())
+        } else {
+            let b = t.wrapping_sub(u32::from(s.baseline_age));
+            let j = (b % npt_sim::ACK_RING_TICKS) as usize;
+            let held = &self.ring[j];
+            (u32::from(s.baseline_age) < npt_sim::ACK_RING_TICKS
+                && held.used
+                && held.tick == b
+                && held.complete()
+                && matches!(held.filed, Filed::Hash(_)))
+            .then(|| held.frame.clone())
+        };
+        let Some(base) = base else {
+            self.refused += 1;
+            self.sentinel(t);
+            return;
+        };
+        let mut tick = core::mem::take(&mut self.ring[i]);
+        if tick.got == 0 {
+            tick.frame = base.clone();
+        }
+        let grid = self.grid;
+        let decoded = s
+            .own
+            .iter()
+            .try_for_each(|d| apply(&mut tick.frame.own, &base.own, d, grid))
+            .and_then(|()| {
+                s.entities
+                    .iter()
+                    .try_for_each(|d| apply(&mut tick.frame.entities, &base.entities, d, grid))
+            });
+        if decoded.is_err() {
+            self.ring[i] = tick;
+            self.refused += 1;
+            self.sentinel(t);
+            return;
+        }
+        for &(low, _age, event) in &s.events {
+            // The event's tick is the header's less its age (snapshot.rs's wire text); the hash reads its number.
+            let number = self.event_number(low);
+            tick.events.push((number, event));
+        }
+        tick.got |= 1 << g;
+        tick.groups = s.groups;
+        tick.sigs[g] = sig;
+        tick.base_age = Some(s.baseline_age);
+        self.ring[i] = tick;
+        self.acknowledge(t, s.group);
+        self.rebuild_sequences(s.newest_received, s.last_run);
+        if self.ring[i].complete() {
+            let e = &self.ring[i];
+            let grid = self.grid;
+            let entities: Vec<EntityRecord> =
+                e.frame.entities.iter().map(|h| h.hashed(grid)).collect();
+            let own: Vec<EntityRecord> = e.frame.own.iter().map(|h| h.hashed(grid)).collect();
+            let hash = client_hash(t, &entities, &own, &e.events);
+            self.file(i, hash);
+        }
+    }
+
+    /// A full state: the client's whole view at its tick, a complete frame, acknowledged as group 0 of that tick.
+    fn full_state(&mut self, f: FullState) {
+        let t = f.tick;
+        if self.too_old(t) {
+            self.stale += 1;
+            return;
+        }
+        self.see(t);
+        let i = self.slot_for(t);
+        if self.ring[i].got != 0 {
+            self.duplicates += 1;
+            return;
+        }
+        let grid = self.grid;
+        let mut frame = Frame {
+            entities: f.entities.iter().map(|r| Held::of(*r, grid)).collect(),
+            own: f.own.iter().map(|r| Held::of(*r, grid)).collect(),
+        };
+        frame.entities.sort_by_key(key);
+        frame.own.sort_by_key(key);
+        let entities: Vec<EntityRecord> = frame.entities.iter().map(|h| h.hashed(grid)).collect();
+        let own: Vec<EntityRecord> = frame.own.iter().map(|h| h.hashed(grid)).collect();
+        let hash = client_hash(t, &entities, &own, &[]);
+        let e = &mut self.ring[i];
+        e.frame = frame;
+        e.groups = 1;
+        e.got = 1;
+        e.base_age = Some(0);
+        self.file(i, hash);
+        self.acknowledge(t, 0);
+        self.rebuild_sequences(None, f.last_run);
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    /// The endpoint's call site into the grid (u4-build-plan.md §4.14 :459): its held floats turned back into grid
+    /// integers. A sweep of position values with half-centimetre ties and negatives, against the server's wire grid.
+    fn differ_share(grid: Config) -> (u32, u32) {
+        let wire = Config::default();
+        let (mut differ, mut all) = (0, 0);
+        for k in -2_000i32..2_000 {
+            for half in [0.0f64, 0.25, 0.5, 0.75] {
+                let x = ((f64::from(k) + half) / 100.0) as f32;
+                let r = EntityRecord {
+                    kind: EntityKind::Soldier,
+                    index: 0,
+                    generation: 0,
+                    area: 0,
+                    grid: [0; EntityRecord::FIELDS],
+                    raw: [0; EntityRecord::FIELDS],
+                };
+                let mut h = Held::of(r, grid);
+                h.held[0] = x;
+                differ += u32::from(h.hashed(grid).grid[0] != wire.to_grid(POSITION_XY, x));
+                all += 1;
+            }
+        }
+        (differ, all)
+    }
+
+    #[test]
+    fn the_endpoints_grid_integers_equal_the_servers_over_a_sweep_with_ties_and_negatives() {
+        assert_eq!(differ_share(Config::default()).0, 0);
+    }
+
+    /// PM-0001-17/client-rounding at the endpoint's call site (part (i)): the floor grid breaks a share of the sweep.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn pm_0001_17_client_rounding_breaks_a_share_of_the_endpoints_grid_integers() {
+        let bitten = grid_with_bite(BITES[0]).expect("planted here");
+        let (differ, all) = differ_share(bitten);
+        println!("client-rounding at the endpoint: {differ} of {all} differ");
+        assert!(differ > 0 && differ < all, "{differ} of {all}");
+        // The share over values decoded from the wire (every position integer from -20 m to 20 m held as its float
+        // and turned back), which the brief's 2.6 asks to state beside the sweep's.
+        let (mut wire_differ, mut wire_all) = (0u32, 0u32);
+        for k in -2_000..2_000 {
+            let held = Config::default().from_grid(POSITION_XY, k);
+            wire_differ += u32::from(bitten.to_grid(POSITION_XY, held) != k);
+            wire_all += 1;
+        }
+        println!(
+            "client-rounding over decoded values: {wire_differ} of {wire_all} differ (reported)"
+        );
+    }
 }
````

### npt-client's clippy.toml
_`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/client/clippy.toml .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/clippy.toml`:  1 file changed, 5 insertions(+), 1 deletion(-)._

````diff
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/clippy.toml b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/clippy.toml
index 5f2b674..520ce6d 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/clippy.toml
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/clippy.toml
@@ -1,7 +1,11 @@
-# The clock ban (plan §4.5): the client endpoint reads no clock; the Worker host passes arrival stamps and `now` in.
+# The clock ban (plan §4.5): the client endpoint reads no clock; the Worker host passes arrival stamps and `now` in. The
+# hash-map type bans (u4-build-plan.md §4.11; U4E-Q8 (a)): the endpoint's maps and sets are ordered or indexed, so no
+# iteration order depends on a hasher's seed.
 disallowed-types = [
     { path = "std::time::Instant", reason = "the clock ban: time is passed in (and std's clock traps on wasm32)" },
     { path = "std::time::SystemTime", reason = "the clock ban: time is passed in (and std's clock traps on wasm32)" },
+    { path = "std::collections::HashMap", reason = "the hash-map ban: iteration order must not depend on a seed" },
+    { path = "std::collections::HashSet", reason = "the hash-map ban: iteration order must not depend on a seed" },
 ]
 disallowed-methods = [
     { path = "std::time::Instant::now", reason = "the clock ban" },
````

### npt-room's tests (crates/room/tests, new)
_ 3 files changed, 1266 insertions(+); the diff itself is in `.checkpoints/netcode-proof-test/u4e-packet-tests.md` (`git diff --no-index .gate/u4e-empty .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests`)._

### npt-server's tests (crates/server/tests)
_ 4 files changed, 1273 insertions(+); the diff itself is in `.checkpoints/netcode-proof-test/u4e-packet-tests.md` (`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/server/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests`)._

### npt-client's tests (crates/client/tests)
_ 2 files changed, 319 insertions(+), 1 deletion(-); the diff itself is in `.checkpoints/netcode-proof-test/u4e-packet-tests.md` (`git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/client/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests`)._

## What it is judged against

### The approved plan: §5 U4e (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 704-738 (sha256 686a583c37e3), as they are:_

````text
### U4e, the room and the codec path (depends on U4a, U4b, U4d; size M-L; its own brief)
- **Builds:**
  - npt-room: `Room<S>`; `Roster` and the slot token compare (lab passes tokens in; minting is U6's); `ClientSlot`;
    `AckRing` (§4.5); `Encoder` (the window's remainder, groups with index and count, the layout table, the class
    filter, the mirror hash of §4.6); `EventQueue` (§4.7); `InterestGrid` (integer cm), `AreaFlood` (from
    `portal_states`), `Priority`; `InputBuffer` option B with `buffer_depth_ticks`; `rewind_at` (the integer half; the
    tolerance and cap rules land in U4h); the room-side checks -58's counters pass through; Resync one in flight.
    room's and client's clippy.toml gain the hash-map type bans (§4.11; U4E-Q8).
  - npt-server's library: `Decoder::pump` (decode, socket checks, the ping sender and answers with their hold time,
    per-client RTT and bytes sent: §4.5, §4.9), `ConnLimits`, `Counters`, the rate buckets and the Join deadline on the
    injected now. No AddrTable and no handshake cap (U6).
  - npt-client: decoding into integer frames and held floats, the complete-tick rule and the sentinel (§4.6), acks with
    the group mask, a minimal `ClientEndpoint` (input, step, the next due time in ns).
- **Tests:** catalogue item 4 (the layout table equals the Field-group rows; the authority-only groups absent); a forced
  group split, each group decoding alone; a lost group leaves its tick unlogged and counted, a refused one logs the
  sentinel; the baseline rule and the ring's age edges; a gap evicting B (on Loopback `npt_sim::ACK_RING_TICKS` tick
  periods: U4E-Q14); the window counting the pump's bytes; the mirror equal to the client's hash over Loopback at P0 for
  1,000 ticks; the Send asserts; **negative cases (T17, A15):** each JoinRejected reason (bad token, bind mismatch,
  roster full, wrong version, wrong content hash); a datagram before Join dropped and counted; a control message before
  Join closing the session as malformed, counted, never INVALID; the Input bucket (8, refilled at 60 a second) and the
  Ping limit (10 a second) on a fake clock; the 2 s Join deadline on a fake clock; a second Resync before the
  FullState's ack dropped and counted; npt-client drops and counts a message on the wrong channel (catalogue item 2's
  "on both sides", plan E7; nit d).
- **-18's bite end to end (the advisor's condition 6):** §4.14's unit proof is the server half only. U4e's proof runs
  from the encoder to the client, through a gap in delivery (on Loopback `npt_sim::ACK_RING_TICKS` tick periods:
  U4E-Q14): under no-fallback the client refuses every delta after the gap, so no fresh delta decodes within 500 ms; the
  clean build decodes one. If the bite's baseline turns out to be one the client still holds, the bite is not a defect
  on L1, and -18's L1 part comes to the advisor before it counts.
- **Bites (unit-shown):** no-fallback (-18, shown end to end as above), tick16, client-rounding (the client's call
  site), lt-ack, lt-tick-offset and lt-event-seq (§4.14), lt-cmd-seq and lt-press (the room's compares), server-echo
  (-62, the room's routing), unbounded (-22, the event queue's cap; U4f runs it judged: U4E-Q12), owner-to-all
  (catalogue item 5's), no-trailing and no-channel through the pump.
- **Marks moved:** none judged (U4r runs them).
- **Reviewers:** netcode-architect, security-auditor (the decode path, roster and token compare), engine-architect
  (crate boundaries), rust-reviewer, test-engineer.
````

### The approved plan: §4.1, the public items (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 178-203 (sha256 686a583c37e3), as they are:_

````text
### 4.1 Public items: plan §4.7 plus these, each with its reason
| Crate | Added or changed | Why |
|---|---|---|
| npt-sim | `SimCore` (moved from npt-room) | U4-SEAM |
| npt-sim | `TICK_HZ`, `LOOP_CATCHUP`, the tick-constant functions and their derived constants (§4.3) | plan.md:544-546; A4, A5: the Worker reaches npt-sim, never npt-server |
| npt-sim | `grid` (the wire grid, each way, per field kind) | U4-OWNSNAP: the sim's own-copy snap and both codec ends need one function |
| npt-sim | seam data: `Command`, `Rewind`, `TickInput`, `EntityRecord`, `HitVolumes`, `HitHistory` (moved from npt-room), `PortalStates`, `SimEvent`; `InputReports` and `SimCore::reports` (the clamp reports and the refusal counters, outside the hash: the advisor's condition 1); the owner-only own-state kind (W1-S1); the records-to-volumes function (W1-U4D A-M1); `InputReports`' refusal records (W1-U4D N-M2); the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`, `RayHit`, `replay::Step`, `Weapon` (TUNE-1); `Scene`'s and `World`'s accessors (`World::new`, `scene`, `greybox`, `next_tick`, `dump`; `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins`, `with_stand_ins`, `weapon`, `health`); W1-PUB's other approved items: `StandIns::OFF` and `s1`, `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`, carry 17's button and press constants, `Greybox::dm_tri`, and, bites builds only, `World::BITES`, `World::with_bite` and `World::bite_order_digest`; `Scene::effect_rows` (an effect row's owner, the one public home: carry 51) | §4.2; A1, A2; the advisor's condition 1; the advisor's W1-S1, W1-U4D and W1-PUB rulings; TUNE-1: `Weapon`, `Scene::weapon` and `Scene::health` are the facts each run's header records (U4r's npt-lab headers and U6's npt-server header: carry 60); carry 51 (the advisor's U4d close ruling, B2) |
| npt-sim | `Recording` (with `GENERATOR`, `encode`, `decode` and `synthetic`), `StateDump`, `replay::Replayer` and its methods | RQ5's field diff; U4-REPLAY; A12's one driver; W1-PUB: `synthetic` is in the library because U4d2 regenerates the recordings (§4.15) |
| npt-sim | `World::predict`, `World::load_movers`, `World::ray_cast`; `Scene::estimate`, `Scene::course`, `Scene::targets` beside `Scene::stress` | ADR 0004:166-178; ADR 0001:258-260; ADR 0001:1470-1482 |
| npt-sim | `EntityKind::ALL` and `EntityKind::layout`; `grid::Field::integer`; `full_state_ack_ticks_at`; `HitHistory::for_ms` in place of `HitHistory::new`; bites builds only: `grid::Config::floor_rounding`, `OWN_COPY_SNAP_BITES` and `own_copy_snap_with_bite`. Removed: bites-only `grid::Config::BITES` and `grid::Config::with_bite` (the advisor's U4s ruling, condition 1 (a)) | u4s-carries.md carry 30: arch F2 (one home for the per-kind field order; one type for a layout's fields); rust 1 (each derived constant is `<name>_at(TICK_HZ)`); netcode F6, A5; arch F1 |
| npt-proto | `RoomControl` (unfeatured; N5) | catalogue item 6 (:253); U5's fuzz target (catalogue :280) |
| npt-proto | `Wrap` (one compare per width, called with its counter's name) | -58's per-counter bites (§4.14) |
| npt-proto | `client_hash`; the `EntityRecord` ↔ wire mapping | -17 (§4.6); A18 |
| npt-proto | dependencies on npt-sim (grid, seam types) and npt-link (`Channel`) | U4-OWNSNAP, A18; catalogue item 2 (arch F37); leaf to leaf |
| npt-proto | `Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` | the signatures and fields of plan §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's W1-PUB ruling) |
| npt-client | a dependency on npt-link, for `Channel`; bites builds only, `BITES` and `grid_with_bite` (the advisor's U4s ruling, condition 1 (a)) | as above; u4s-carries.md carry 30: arch F1 |
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP`; bites builds only: `BITES` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling) |
| npt-server | `Decoder::pump` (the I/O step over `&mut dyn Link`, injected `now`, the ping sender, per-client RTT and bytes sent); `Schedule`, `Due`; `Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling |
| npt-l1 | `L1Net::next_event_ns` | C1' makes L1 event-driven |
| npt-bots (library) | `Behaviour` (the brawl, the open-loop scripts, the target-course shooter, the cheats, the never-acker), clock-injected | E12; U4-SERVER |
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |
| npt-log | `TickRecord.hs`, with `TickRecord::set_hashes` and `TickRecord::hashes` | marks/README.md:147-148 (since U4j, README §1's `hs`) |
| npt-judge | `marks::Mark::ws_bound_line`, `marks::Check::ws_bound` and `marks::BoundNumber` (PM-0001-18's WebSocket bound, its line held verbatim); `marks::Rule::WsPractice` (the `ws-practice` rule); `eval::Outcome::Empty`, `eval::Outcome::fails` and `figures::Lack::Empty` (0 compared fails, never passes) | Q20 and the advisor's ws-docs R3-R12 (-18's split, `ws-practice`); the advisor's U4j FX-6 (g) (A19); §4.6's "0 compared fails" |
| npt-judge | nothing for -30's pre-check: the `PRE-` id, marks/prechecks.json and the `"kind": "precheck"` rule are dropped (the advisor's condition 2) | Q10 (T2); the advisor's condition 2 |
| a new crate | `npt-hash-wasm` (cdylib; integer-only exports) | U4-WASM (B) |
| not built at U4 | `BlockWriter` (U7); `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, `AddrTable`, the handshake cap (U6) | §6 |
````

### The approved plan: §4.2, the seam (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 205-249 (sha256 686a583c37e3), as they are:_

````text
### 4.2 The seam
- `SimCore`:
  - `step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent]`;
  - `state_hash(&self) -> u64`;
  - `entity_records(&self) -> &[EntityRecord]`: per entity its id, kind, area id, grid integers, and the pre-grid
    values as `u32` bits (for -57's log; the room copies bits and never computes with them);
  - `hit_volumes(&self) -> &HitVolumes`; `portal_states(&self) -> &PortalStates` (each portal open or closed: A2);
  - `reports(&self) -> &InputReports`: the tick's clamp reports and the running refusal counters per kind, outside
    `state_hash` and outside the hashed event list; the room reads them through this accessor alone and logs them
    (the advisor's condition 1);
  - spawn, teleport and scripted attribute changes go in through `TickInput`.
  - One implementation: `World`. The static graph is npt-sim's plain-data `Areas` and `Portals` (plan.md:567), so the
    room's `AreaFlood` re-floods when `portal_states` changes, with no float and no sim internals.
- **`Rewind`** rides with each command that has fire held or a fire edge: `tick_a: u32`, `fraction: u8` (the blend
  from tick_a toward tick_a + 1; U4a's field table fixes the bits, net F116 proposes 8), `cover: CoverVariant`
  (R-10's two variants). Integers only. A Rewind naming a tick the lent ring does not hold is refused and counted by
  the sim, in `reports()` and never in the hash (the advisor's condition 1); the debug assert that a correct room never
  sends one sits in the room's `rewind_at`, not in the sim, so that condition 1's unit test can feed the sim one (the
  advisor's condition 1).
- **`HitHistory`** is npt-sim's fixed ring of `HitVolumes` per tick (capacity set at construction from U4h's test cap,
  in ms, through §4.3's functions). The room owns its instance and pushes `world.hit_volumes()` after each `step`; the
  replay driver does the same, so a recording needs no volumes. The client's predicting World carries none.
- **The clamp** (Q25): inside the shot phase, on the aim point in integer cm, from the in-step fire origin: a point
  beyond the weapon's reach + 0.87 cm (one grid cell's half-diagonal) moves along its own ray to exactly the reach and
  is snapped to the grid; a clamp report `{ slot, seq, point }` in `reports()` records it, never a `SimEvent` and never
  in the hash (the advisor's condition 1). A point within reach + 0.87 cm is never clamped again, so a clamped point
  sent back is stable (the twin's rule, §4.10), for every fire origin below the bound World's doc states (TUNE-2).
- **Two entry points** over the same phase functions (ADR 0004:166-178): the authority `step` and `World::predict` (one
  slot and its vehicle, against the movers `World::load_movers` loaded from the last decoded snapshot, drawing from no
  random stream). Both end each tick with the own-copy snap through `grid` (Q23).
- `World::ray_cast(&self, ray, movers: &HitVolumes) -> RayHit`: the static world from self, the movers supplied (the
  client's interpolated set for AimCast, the blended rewound set for a server shot: ADR 0001:258-260, :358-362; A11).
- Pools have fixed capacity; a full pool refuses and counts. The tick's event list is state and hashed
  (ADR 0004:544-561).
- **Refused input leaves no trace in the hash (the advisor's condition 1).** ADR 0004:544-556 hashes the tick's event
  list, so the clamp reports and every refusal counter (the cooldown, ammunition and reload edges, Rewinds outside the
  ring, a full pool) stay outside `state_hash` and outside that list (the advisor's U4s ruling, condition 1 (b)): they
  live in `reports()`, and the room logs them. The sim's one piece of per-slot input bookkeeping is the press counters
  of the last command it ran for that slot: a press edge is a counter that differs from them, and a refused press's
  edge is consumed there and changes nothing hashed. They stay outside `state_hash` and `StateDump` (a twin's counters
  differ by design); they are a function of the command stream alone, and the predicting World restores them from the
  client's own command at the snapshot's last-run sequence (U4g). `reports()` also lists each refusal per command
  (`{ slot, seq, kind }`, beside the clamp reports), so a twin removes exactly the refused parts. D-10 runs past the
  next accepted press. (The advisor's W1-U4D ruling: it replaces condition 1's 'what the server accepted, not what
  the client claimed' for the press counters.)
````

### The approved plan: §4.10, hostile input (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 360-390 (sha256 686a583c37e3), as they are:_

````text
### 4.10 Hostile input (Q16)
- Every check in plan §4.4's table gets its home and a -21 counter; the clamp is "(in the sim)", as cooldown, ammo and
  reload already are (Q25).
- **The clamp twin:** the forged run's room logs each clamp report's point (read through `reports()`, never from the
  hashed event list: the advisor's condition 1); the twin run sends exactly those points at the same instants. Since a
  point within reach + 0.87 cm is never clamped again, the twin's shots use the same points, and a unit test shows
  clamp-then-grid is idempotent, for every fire origin below the bound World's doc states (TUNE-2).
- **The twin's command for a kind the sim checks** (the cooldown, ammunition and reload edges; the clamp, whose twin
  sends the clamped point) is the forged command with only the refused part removed, everything else equal (the
  advisor's condition 1). For a refused press the refused part is the press itself: the twin's counter for that button
  is one lower from the refused command on, every other field equal, and the twin removes exactly the presses the
  forged run's reports list (the advisor's W1-U4D ruling).
- **`event.clamp_kind.counted_count` counts refusals only, never clamp reports** (SERIES.md:144; marks.json -21's
  "clamp: never refused or counted"): npt-lab writes it from the refusal counters, and a clamp report adds nothing to
  it (the advisor's condition 1).
- **-21's twins are fate-aligned (marks F27, M22) and ack-aligned (T7):**
  - forged content rides only inside datagrams the honest script sends anyway, at the same instants, on the same flows;
  - a forgery that voids a whole datagram (trailing bytes, an unknown field) has, in its twin, a datagram in that slot
    with the header only, no commands, and an ack equal to the previous datagram's (no new information), so the
    server's acks, baselines, snapshot sizes and group counts match the forged run's;
  - every client of the forged and clamp suites runs an open-loop seeded script (commands a function of seed, slot and
    tick only), so nothing it decodes feeds back into what it sends;
  - a wrong-channel forgery on the control stream has, in its twin, a valid no-effect message in that slot (a
    ClientSettings repeating the current delay); a wrong-channel message of a known type within its limits is refused
    and counted, never kicked, so no forged kind closes the session (catalogue Input row :87 against Kick row :99);
  - **the catch-up kind (T8):** one datagram holds at most 8 commands, the cap itself, so this kind sends extra
    datagrams in one tick (16 new commands in two), inside the per-connection bucket (8 datagrams, refilled at 2 x
    TICK_HZ a second), so the catch-up check refuses the excess, not the bucket. The twin sends the same number of
    datagrams at the same instants, carrying repeats only;
  - a test shows the forged run's and its twin's fate logs equal on every flow, and their snapshot group counts equal on
    every tick.
````

### The advisor's wave-1 ruling W1-S1, the soldier's layout (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 45-111 (sha256 a45575aeb3b4), as they are:_

````text
### W1-S1: the soldier's layout (S1; SEC-U4a-1, with SEC-U4a-2; U4d's engine-architect M2)
**Ruling:** I take the reviewers' fix, add the codec's enforcement of the GROUPS tables (SEC-U4a-2), and **lift the
freeze for exactly this change: it goes into U4d's fix pass now** (the npt-sim half). U4a's half (two GROUPS lines,
the enforcement and a test) is the main session's, in U4d's merge commit, on the tree that already holds U4a. U4a may
merge first without it.

**Why:**
- The leak holds. All four U4a reviewers agree, and so do the test-engineer's probe (velocity: 48 bits in every
  remote soldier record today) and U4d's worktree (seam.rs:208-221 adds WEAPON_STATE: 0 ready, 1 cooling down,
  2 reloading, 3 empty, to SOLDIER). Against them stand catalogue :136-137 (the weapon state is owner only; the remote
  row has neither) and ADR 0001:310-313 (remote soldiers: position, yaw, pitch, health, flag bits, weapon id).
- The freeze (my U4s ruling, condition 3) protected U4a while it built its tables against the same types. U4a's
  build is done, and its code reads `layout()` and `ALL` at run time (snapshot.rs:4-7, :86, :131; common/mod.rs:82,
  :98), so the freeze's reason no longer covers this change. The one U4a test it breaks (snapshot.rs:638-652, every
  kind needs a GROUPS home) is fixed in the same merge commit.
- U4d's fix pass already rewrites the same paths: the soldier's record writer (FLAG_FIRED, the magazine) and the
  records-to-volumes path (H1, A-M1). One pass and one round 2 cover all of it. A main-session commit after U4d's
  merge would need its own review, and its own pin and test churn.
- The fields that leave SOLDIER are exactly what U4g's Predictor needs to reset the own soldier, so
  engine-architect M2's "FIELDS is full" goes away with it.
- Nothing that sends one snapshot to several clients (U4e) exists before U4d merges, so SEC-U4a-1 closes before any
  client could read another's weapon state. That is the security-auditor's own bar ("Nothing sending to >1 client
  (U4e) merges before it").

**The change, exactly:**
- **npt-sim (U4d's fix pass):**
  - Add one new `EntityKind` variant, appended last to `ALL` (the builder names it; I suggest `OwnState`). The seven
    kinds keep their 3-bit codes: KIND_BITS stays 3 with 8 kinds.
  - Its layout holds every per-soldier value that decides that soldier's next ticks and is not in SOLDIER:
    velocity x, y, z (`grid::VELOCITY`; `raw` before the snap and `grid` of it, as carry 9 says), the ammunition, the
    fire timer (`wait`), the reload left, and the move-speed and fire-interval attributes (PM-0001-60's two rows).
    Each is a `Field::integer` whose range holds every value the World can reach, asserted. There is no
    active-reload field: the stand-in has none.
  - The World writes one such record per soldier: index = the slot, generation = the soldier's.
  - SOLDIER becomes position x, y, z, yaw, pitch, flags, health and weapon id (9 fields), the catalogue's remote row
    (:137). "Reloading" stays a flag bit (`FLAG_RELOADING`; ADR 0001:310-311 lists it among the remote flags).
  - `EntityRecord::FIELDS` stays 12.
  - Tests: s6's and carry 12's pins are updated; D-3 compares the new record too; carry 9's per-field off-grid
    counts cover the velocity fields in the new record.
- **npt-proto (the main session, in U4d's merge commit, after U4a has merged):**
  - The new kind joins the "own state" rows of `Snapshot::GROUPS` and `FullState::GROUPS` (2 lines).
  - The encoder and the decoder refuse a record whose kind its section's GROUPS rows do not list (SEC-U4a-2), with
    the existing error kinds.
  - The GROUPS test becomes three checks:
    - every kind has a home in both tables;
    - every owner-only kind (Effect and the new one) sits only in Owner rows;
    - a remote soldier's Add carries exactly the catalogue's remote row: position 16/16/12, yaw 16, pitch 10,
      flags 8, health, weapon id. This is the security-auditor's proof form, and it fails on today's tree by 48 bits.
  - tests/bites.rs's -57 test also sends the owner-only record, so "velocity untouched" still has a velocity field
    to check.
- **The review:**
  - U4d's round 2 reviews the combined diff (U4d's fix pass plus the proto half) in a scratch worktree, before the
    main-folder merge.
  - The security-auditor is added for this part only, so SEC-U4a-1 and SEC-U4a-2 close in writing.
- **u4-build-plan.md, Lanes' freeze bullet (:824-831):** add after its last sentence: "**Lifted for one change (the
  advisor's W1-S1 ruling, 2026-11-10):** U4d's fix pass appends one owner-only kind to `EntityKind` (last in `ALL`),
  holding velocity, the ammunition, the fire timer, the reload left and the move-speed and fire-interval attributes,
  and takes velocity and the weapon state out of SOLDIER's layout. `EntityRecord::FIELDS`, `Command`, `Rewind`,
  `CoverVariant` and the five grid fields stay frozen. U4a's half (the kind in both GROUPS tables' own-state rows, the
  codec's refusal of a kind its section does not list, and its test) is the main session's, in U4d's merge commit,
  after U4a has merged."
- **§4.1's npt-sim row** (:184) gains "the owner-only own-state kind (W1-S1)".
- **The catalogue, at U4d's merge** (with N8's edits): the own-state section carries the owner's soldier record, its
  owner-only record and its effect rows.
- **U4e's brief (rev 3)** states the kind and that the room sends it only in its owner's own-state section.

W1-S1 written 2026-11-10 (the first ruling made, since W1-U4D builds on it).
````

### The advisor's wave-1 ruling W1-N2, the header's command sequences (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 305-351 (sha256 a45575aeb3b4), as they are:_

````text
### W1-N2: U4a's snapshot header carries command sequences, not tick offsets
**Ruling: APPROVE the plan text edit (my condition 12), §4.14's -58/lt-tick-offset row included.** U4a's header
(snapshot.rs:9-16, :392-395) carries the full 32-bit tick, the baseline as its 16-bit age, the newest command received
and the last command run as 16-bit command sequences (each with a known flag), and the group index and count.

**Why:**
- ADR 0001:228-230 has each snapshot "acknowledge two commands": the newest received, which ends its repeats, and the
  last run, where the replay starts. A sequence names a command exactly. The tick a command ran at need not be its
  stamped tick: the input buffer delays it, and OQ-0001-6 (b) repeats the last command on a miss.
- The scout's own estimate already used sequences (net F112: "newest-received seq 2, last-run seq 2"). §4.4's
  "16-bit offsets (baseline, newest command received, last command run)" was a drafting slip.
- ADR 0001 part 6's "other ticks are 16-bit offsets" still holds for the ticks the header does carry: the baseline's
  age, and the event age in the body.
- Nothing gets looser. -58 keeps its six bites and its six wrapping counters.

**Text changes (u4-build-plan.md):**
- §4.4 (:273-274): replace the bullet with: "The snapshot header: the full 32-bit server tick; the baseline as its
  16-bit age in ticks (0: coded against the per-type defaults); the newest command received and the last command run
  as 16-bit command sequences, each with a known flag (ADR 0001:228-230 acknowledges two commands; a sequence names
  the command, while the tick it ran at can differ from its stamp; net F112 proposed them; the advisor's W1-N2
  ruling); the group index **and the group count** (so the client knows when a tick is complete: §4.6)."
- §4.14, the -58/lt-tick-offset row (:454): replace with "| -58/lt-tick-offset | the client's rebuild of the
  header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware
  compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) |
  0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the
  rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |".
- §5 U4r (:737): delete "-58 for 10 s at P0 and P3, every counter a few steps below its wrap; its six bites (§4.14)."
- §5 U4g, Marks moved (:791-795): add "-58 (10 s at P0 and P3, every counter a few steps below its wrap, 3 seeds per
  part; its six bites, §4.14), judged after U4g merges: its untraced-corrections check and lt-tick-offset's failing
  check read the Predictor (the advisor's W1-N2 ruling and U4E-Q2)."
- §10's readings preamble (:927-928) and §11 (:1028-1033): the readings list gains "PM-0001-58's 'the snapshot's
  tick offsets' (ADR 0001:1607) read as the snapshot header's two command acknowledgements (16-bit sequences); the
  remaining tick offsets, the baseline's age (at most 31: §4.5's ring) and the event age (8 bits), never cross a
  wrap, so they get no bite (W1-N2)".

**Elsewhere:**
- marks.json, `PM-0001-58/lt-tick-offset`: its "what" becomes "the snapshot header's command acknowledgements
  (16-bit sequences) compared with a plain <", and the id stays. The wrap check's name ("... tick offsets ...") says
  "the header's command acknowledgements" in place of "tick offsets". The client logs that counter's wrap as its own
  `event.counter_wrap` (U4e), so the check's count of 6 stays honest. U4j's brief (u4j-brief.md) carries both lines;
  U4j owns marks/ in wave 1.
- The catalogue :135: in N8's edits at U4a's merge (the header's fields as built).
- New carry (U4e): "An event older than 255 ticks cannot be coded: the event section's age is 8 bits (snapshot.rs's
  wire text). The room's EventQueue never queues one past that age: the overflow resync (§4.7) ends it first, or the
  room drops it from the resend and counts it. A unit test."

W1-N2 written 2026-11-10.
````

### The advisor's wave-1 ruling W1-PUB, the public items beyond §4.1 (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 353-435 (sha256 a45575aeb3b4), as they are:_

````text
### W1-PUB: the public items beyond §4.1's lists (A19)
**Ruling: APPROVE all three lists, and remove nothing.** Two nits are below, and §4.1's table is to be brought up to
date. Every item is one of three things:
- a type in the signature or a field of an item §4.1 or plan.md §4.7 names;
- the field-group layout table that §5 U4a itself lists;
- an item a planned test needs (the catalogue audit, carry 6's read-back).

None is a "for later" item.

- **U4a (npt-proto; its lib.rs:32-45, checked against the rust-reviewer's A19 list):**
  - `Message`, `Direction`, `DecodeError` and `EncodeError`: in `encode`'s and `decode`'s signatures.
  - The message structs, `EndReason`, `ProfileId`, `StepResult` and `TokenKind`: fields of the listed messages and
    controls.
  - `Limits`' constants and `Limits::TYPES`: `Limits` is listed, and catalogue items 2 and 3 test against `TYPES`.
  - `Quant::encode`, `Quant::decode` and the bites-only `Quant::with_bite` and `BITES`: the per-instance bites
    (§4.11).
  - `Wrap::after8`, `after16`, `after32` and `widen16`: §4.4's one compare per width, and the rebuild. `plain_lt` is
    bites-only.
  - `EntityDelta`'s `between` and `apply`: the `EntityRecord` mapping (§4.4).
  - `Class`, `Snapshot::GROUPS` and `FullState::GROUPS`: §5 U4a's "field-group layout table". W1-S1 makes the codec
    enforce them, so they become operative.
  - `Message::name`, `SessionControl::VARIANTS` and `variant`, `RoomControl::NAMES` and `name`: catalogue item 6's
    audit (approved, Q7).
  - Nit (rust F-A8, a follow-up): `Limits::GROUPS` and `Snapshot::GROUPS` share a name with two meanings. Rename the
    limit at the next edit of limits.rs.
- **U4b (npt-server's schedule.rs):**
  - `Dropped` and `LoopCounters`: types inside `Due` and `counters`.
  - `next_due_ns`: U6's timer, and the tests' "no tick owed" check.
  - `counters`: ADR 0004 part 5's monitoring.
  - `tick_hz` and `catchup`: carry 6's read-back.
  - Bites builds only: `Schedule::with_bite` and `BITES` (PM-0004-67's three, schedule.rs:251; lib.rs:13).
  - These match u4b-build.md:31-42 and schedule.rs:22-119 and :251-257.
  - Nit (a follow-up): each item has two paths (`npt_server::schedule::X` and `npt_server::X`: lib.rs:10, :13-14).
    Keep one at U4e's next edit of server/src/lib.rs.
- **U4d (npt-sim).** u4d-build.md holds no "For the advisor" section, so I ruled on the surface its lib.rs exports
  (worktree crates/sim/src/lib.rs:25-55, and each module's `pub` items):
  - Plain-data types of named items (plan.md §4.7: "with its plain-data input and output types"):
    - `Placement`, `AttributeChange` and `Attribute`: `TickInput`'s fields;
    - `ClampReport`: `InputReports`' field;
    - `VolumeBox`: `HitVolumes`' field;
    - `FieldDiff`: `StateDump::first_difference`'s result (RQ5's field diff);
    - `Ray` and `RayHit`: `ray_cast`'s argument and result;
    - `replay::Step`: `Replayer::step`'s result.
  - Named in plan.md §4.7: `Greybox`, `Areas`, `Portals`, `StandIns` and `trajectory_at`. Engine-architect F is
    right that the builder's gap list overstated this.
  - The constructors and accessors the room, lab and Wasm wrapper need:
    - `World::new`, `scene`, `greybox`, `next_tick` and `dump`;
    - `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins` and `with_stand_ins` (the header's scene id; the
      stand-ins' off switch on L1: §4.13);
    - `StandIns::OFF` and `s1`;
    - `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`;
    - `Recording::GENERATOR`, `encode`, `decode` and `synthetic`;
    - `Replayer`'s methods;
    - carry 17's button and press constants.
  - Bites-only: `World::BITES`, `with_bite` and `bite_order_digest`.
  - `Recording::synthetic` belongs in the library: U4d2 regenerates the recordings from the manifest (§4.15's text
    changes: W1-U4D).
  - `Greybox::dm_tri` stays public: the static graph without a World, for the room's AreaFlood. U4e's report says
    which route the room takes.
  - Nit (rides U4d's fix pass): `Recording` has two paths (lib.rs:17's `pub mod replay` and :28's re-export). Keep
    one.
  - `SimRng` stays crate-private, narrower than plan.md §4.7: no other crate draws from the sim's streams (§4.5; the
    spread stream is authority-only). Record it in U4d's report, and plan.md §4.7's npt-sim line at its next edit:
    "`SimRng` (crate-private: no other crate draws from the sim's streams; the advisor's W1-PUB ruling)".
- **§4.1's table (main session; text):**
  - npt-proto's rows gain one row: "`Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control
    types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s
    `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the
    catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` |
    the signatures and fields of §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's
    W1-PUB ruling)".
  - npt-server's row gains "`Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; bites builds
    only: `Schedule::with_bite`, `BITES`".
  - npt-sim's row gains:
    - "the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`,
      `RayHit`, `replay::Step`";
    - the owner-only own-state kind (W1-S1);
    - the records-to-volumes function (W1-U4D A-M1);
    - `InputReports`' refusal records (W1-U4D N-M2);
    - `Scene`'s and `World`'s accessors as listed above.
  - npt-room's row gains "bites builds only: `BITES`" (U4E-Q3).

W1-PUB written 2026-11-10.
````

### The advisor's wave-1 ruling W1-U4E, U4e's questions U4E-Q1 to Q11 (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 437-497 (sha256 a45575aeb3b4), as they are:_

````text
### W1-U4E: the U4e brief's questions (draft rev 2 at 0dccbf0, §8)
The brief can be finished after wave 1 merges on these rulings. Its rev 3 also carries W1-S1 (the owner-only kind,
sent only in its owner's own-state section), W1-N2 (the header, and U4e's event-age carry) and W1-U4D's carries
that name U4e.
- **U4E-Q1, confirmed: (a).** U4e plants and unit-shows PM-0001-58/lt-cmd-seq and lt-press: post-wrap commands
  refused, and an edge lost.
  - Why: §4.14:438 plants each bite where its code lives, and U4e builds the room's sequence-forward and press-edge
    compares (:691). The Bites line (:711-712) left them out by a slip.
  - Text: §5 U4e's Bites line gains "lt-cmd-seq and lt-press (the room's compares)".
- **U4E-Q2: (c), widened.** U4e builds the client's rebuild of the header's 16-bit last-run sequence (W1-N2) and
  unit-shows the bite as "the rebuilt sequence names the wrong command". U4g's Predictor starts its replay there and
  shows "the replay starts at the wrong command".
  - The widening: all of -58 is judged after U4g, not only this bite. Its check "0 corrections larger than 1 mm not
    traced" reads the Predictor's count, which does not exist before U4g. Judged at U4r, -58 would show a gap, or
    worse, a vacuous 0.
  - Text: W1-N2's §5 U4r and §5 U4g edits.
- **U4E-Q3: (a).** npt-room gains a bites-only `BITES` list, as npt-client has. npt-server already has one: U4b added
  PM-0004-67's three ids (schedule.rs:251, re-exported at lib.rs:13), and U4e adds its pump's ids to that list rather
  than making a second. U4e's report names them as beyond §4.1 under this ruling, and §4.1's rows gain them (W1-PUB).
  - Why: U4r's `--bite` dispatch and check.mjs's bite-home grep need one findable home for each id, and a bites-only
    item never reaches a measured build.
- **U4E-Q4: (a).** PM-0001-17/tick16 and PM-0001-58/tick16 stay in `npt_proto::BITES`, which is where U4a put them
  (quant.rs:34-42). The client's endpoint takes its `Quant` from `npt_proto::Quant::with_bite`, as it takes its grid
  from `grid_with_bite`, so each literal lives once.
  - Why not (c): it would add ids marks.json does not spell, for nothing a run needs.
  - Carry 24's text changes: "... `npt_client::BITES` grows with U4e's other client-site ids (-58/lt-tick-offset
    and -58/lt-event-seq, the client's rebuilds); -17/tick16 and -58/tick16 stay in `npt_proto::BITES`, the endpoint taking its `Quant` from
    `npt_proto::Quant::with_bite` (U4E-Q4); -45's ids and -62/cues-from-replay join npt-client in U4g, whose code
    holds their sites (U4E-Q10); no id appears in two crates' lists."
  - U4e then reports carry 24 as met under these words.
- **U4E-Q5: (a), plus one condition.** With a claimed view time whose tick the ring does not hold, `rewind_at` gives
  the command no Rewind and the room counts it. A refused Rewind runs as none in the sim, so this is one rule. U4h
  then replaces it with the tolerance and the cap.
  - The condition: that counter counts as a replacement for -30's setup condition. A pre-check leg with any is void
    (my condition 3).
  - Text: U4r's brief and carry 40 name the counter.
- **U4E-Q6: (a), plus two conditions.** U4e builds the cap as data, with a provisional value from net F121's
  arithmetic, recorded as provisional (1,024 is F121's example, above the about 400 a B1 burst adds). U4f measures
  the rate and records §4.7's value before -22's runs.
  - Condition 1: U4r's contract test logs each client's relevant gameplay-event rate and its queue's high-water mark
    per burst.
  - Condition 2: an overflow resync in any judged -17 or -18 run is reported, and those runs come to me before they
    count. F121 warns that a cap set too low resyncs every client on every burst and changes -18's heal path.
- **U4E-Q7: answered, confirmed.** The plan gives the series' writer (§4.12:397-398; U4r :719-720; `hs` is U4j's,
  :627). No ruling needed.
- **U4E-Q8: (a).** U4e adds the HashMap and HashSet type bans to crates/client/clippy.toml. It is §4.11's text and
  U4e's lane, and the client hash depends on iteration order.
  - Text: §5 U4e (:692) "room/clippy.toml gains the hash-map type bans" → "room's and client's clippy.toml gain the
    hash-map type bans (§4.11; U4E-Q8)".
- **U4E-Q9: answered, confirmed.** The F2 ruling covers it: the rate is passed as an argument, with a pin at 30 and
  60 Hz. No ruling needed.
- **U4E-Q10: (a).** U4e plants PM-0001-62/server-echo in npt-room, its event routing, with its unit proof ("one event
  sent"). -45's ids and -62/cues-from-replay join npt-client in U4g, which builds `Interp` and `CueLog`.
  - Why: an id with no site is a bite that cannot bite (risk 3).
  - Text: carry 24 as under Q4; §5 U4e's Bites line gains "server-echo (-62, the room's routing)".
- **U4E-Q11: (a).** U4e builds the server-side checks (size, trailing bytes, wrong channel, the bucket) and their
  unit tests, as its §5 says. U4f adds their -21 counters and the judged runs on U4e's pump.
  - Text: §5 U4f's Builds (:775) "the server-side checks (size, trailing bytes, wrong channel, the bucket)" → "the
    server-side checks' -21 counters and judged runs on U4e's pump (U4e builds and unit-tests the checks: U4E-Q11)".

W1-U4E written 2026-11-10.
````

### The advisor's wave-1 note W1-TARGET, the shared Cargo target (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 558-586 (sha256 a45575aeb3b4), as they are:_

````text
### W1-TARGET: the shared Cargo target (a note; one rule for wave 2's briefs)
- **What wave 1 showed.** Cargo keys a workspace member's artifacts without its folder and keeps dep-info paths
  package-relative (u4d-build.md, step 7). So a copy that builds after another copy's touch makes the other copy's
  stale sources look fresh, and the lanes ran each other's crates.
  - This happened at least five times: U4a 22:13, 22:14 and 22:55; U4d 06:35; and the test-engineers' runs A in
    u4ab-review-test.md and u4d-review-test.md.
  - Every time, the count check or a compile error caught it, and the run was not reported.
  - The 2 s touch loop narrows the window but does not close it: another lane can still build between a touch and
    this lane's fingerprint check.
- **The gap wave 2 opens.** Fix passes and re-runs often leave the test count unchanged (U4d's round 2 will). There
  the count check cannot tell this copy's code from another copy's.
- **What wave 2's briefs add** (agent briefs and notes, no kit edit, within Mustafa's one-folder rule):
  1. **A lock.** Every cargo, rustc or rustdoc call in every lane runs under `flock` on one file,
     `.gate/shared-target/.lane.lock`: shared (`flock -s`) for ordinary builds, exclusive (`flock -x`) for a counted
     run (the touch, check.mjs, the `--list` and clippy confirmations) and for each bite run. No other lane can then
     build inside a counted run. flock 2.39.3 is present.
  2. **Provenance.** A counted run's log must show "Compiling <crate> (<this copy's path>)" for every crate the lane
     changed, in each cargo invocation that builds it. A missing line means "not counted", whatever the count says.
  3. **Disk.** 12 GB is free, with the shared target at 4.7 GB. U4d2's wasm32 release build adds a target triple.
     Each lane checks for at least 4 GB free before a wasm32 or release build (the 2 GB floor stays for the rest).
     Between waves, with no lane running, the main session may empty the shared folder by `cargo clean --target-dir
     .gate/shared-target`, an approved method (LEFTOVER-DELETE). §7.1 item 7 ("No copy cleans the shared folder")
     binds the copies, not the main session.
  4. **After the merges.** The main session removes `.gate/u4a-prefix` and `.gate/u4b-prefix` (the resume card
     already says so). Wave 1's worktrees go by the standing procedure (history.md, 2026-11-08: leftovers on a local
     branch, then a plain `git worktree remove`).
- Nothing here needs Mustafa: it keeps his one-folder rule as written.

W1-TARGET written 2026-11-10.
````

### The advisor's check of the U4e brief: A1 to A7, U4E-Q12 to Q14, F2-ROUTE, conditions L1 to L4 (advisor-u4e-brief.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4e-brief.md` lines 1-368 (sha256 3485a6c65b85), as they are:_

````text
# The advisor's check of the U4e builder brief (rev 3.2), 2026-11-11

Seat: a day session's advisor agent (general-purpose, opus, xhigh), on main's `.claude/agents/advisor.md`. Read only:
no build, no run, no commit. Branch `claude/netcode-proof-test-plan-hcky5g` at c5190b0.

## Plan
1. Read the brief whole (N/u4e-brief.md rev 3.2) and the check brief (N/advisor-u4e-brief-brief.md).
2. Read the plan's §5 U4e and the parts of §4 it leans on (N/u4-build-plan.md), the Lanes, §6 and §7.
3. Read my rulings: W1-U4E, W1-S1, W1-N2, W1-TARGET (N/advisor-u4-wave1.md), N/advisor-u4d-close.md,
   N/advisor-u4j-r2.md B2.1 (carry 61), N/advisor-u4.md where the brief cites it.
4. Read N/u4s-carries.md §U4e (carries 21-27, 39, 50, 51, 53-57, 61) and compare with the brief's section 3.1 copy.
5. Read the check (N/u4e-brief-r3-check.md): F1 to F3 and the Mediums and Lows; confirm each fix in rev 3.1/3.2 text.
6. Check rev 3.2's fold (N/u4e_fold2.py): carry 61's note, R27, R28, R38, section 0's note on line numbers, the
   "U4j unmerged" lines.
7. Spot-check the brief's claims about merged code at the branch head (crates sim, proto, server, room, client, log,
   judge; marks/) and the message catalogue; .claude/rules/netcode.md.
8. Coverage table: every test, negative case, bite and carry of §5 U4e against the brief.
9. Rule: verdict (APPROVE or REVISE with exact words), U4E-Q12, Q13, Q14, and the F2 route.
10. End: Decisions needed (Mustafa), For Mustafa.

## Blocks (written as each finishes)

### Block 1. What I read, and the state it rests on (done)
- The brief whole (930 lines, rev 3.2); the check brief; plan §5 U4e (:704-735 as the file stands), §4.1-§4.15, Lanes,
  §6, §7; W1-U4E, W1-S1, W1-N2, W1-PUB, W1-TARGET (advisor-u4-wave1.md); advisor-u4.md conditions 6, 10, 12 and the F2
  ruling (:436-466); advisor-u4d-close.md B2 and its U4d-F part; advisor-u4j-r2.md B2.1 and B5; u4s-carries.md §U4e
  and §U4r (58, 62) and 52, 65; the check (u4e-brief-r3-check.md); u4c-review-netcode.md; u4c-lc8-rerun.md;
  u4c-round2-netcode-architect.md; the catalogue §1-§3 and §6; .claude/rules/netcode.md.
- Code at the branch head c5190b0. `git diff --stat 39f4290 d50396a` touches none of crates/{room, server, client, sim,
  proto, link}, check.mjs or docs/; d50396a..HEAD adds only results/c-u2-pm-0001-3/65869918809f/. So every rev-3 cite
  into U4e's crates, npt-sim, npt-proto, npt-link and check.mjs still lands on the same text.
- `Scene::effect_rows()` is not at the head yet (scene.rs:119 is still `pub(crate) effect_rows: u8`): expected, the main
  session adds it at launch (7.2 item 2).

### Block 2. Coverage of §5 U4e, testable as worded (done)
| Plan item (u4-build-plan.md :704-735) | Brief | Verdict |
|---|---|---|
| Builds, npt-room (all 13 items and the clippy bans) | 2.1 | present, testable |
| Builds, npt-server (pump, ConnLimits, Counters, buckets, deadline; no AddrTable) | 2.2 | present, testable |
| Builds, npt-client (frames, complete-tick rule, sentinel, acks, ClientEndpoint) | 2.3 | present, testable |
| catalogue item 4 | e1 | present, testable (12 and 6 rows: snapshot.rs:488, :644; catalogue :132-157) |
| forced split, each group alone | e2 | present, testable |
| lost group unlogged and counted; refused gives the sentinel | e3 | present; its resolution timing is carry 61 (c)'s and is not tied to e3 (finding A1) |
| baseline rule; ring age edges | e4 | present, testable |
| 1 s gap evicting B | e5a, e5b | present; as worded it stops on Loopback (U4E-Q14, ruled below) |
| window counting the pump's bytes | e6 | present, testable |
| mirror = client hash, Loopback, 1,000 ticks | e7 | present; its split clause may be unreachable for a legitimate encoder (finding A2) |
| Send asserts | e8 | present, testable |
| the nine negative cases | e9 | all nine present, each testable as worded |
| -18 end to end | 2.5 | present; the gap as U4E-Q14 |
| Bites line (12 names) | 2.6 | all 12 present with a site and a proof; owner-to-all's route needs one limit (finding A3) |
| -22/unbounded (§4.14 :475, not on the Bites line) | 2.6's last bullet | open: U4E-Q12, ruled below |
| Marks moved: none | 2 | present |
| carries 21-27, 39, 50, 51, 53-57, 61 (sixteen) | 3.1 word for word; e12; return item 6 | 3.1 equal to carries :147-247 (my own `diff` of the two ranges: no output); e12 has a line for each but carry 61 (finding A1) |
- Beyond the plan: nothing built that the plan, a carry or a ruling does not name. e10 is the F2 ruling's pins; e11 is
  §4.11's own test; e7's non-vacuity and e5's alignments are test strength, not builds.
- W1-TARGET in the build rules: item 1 (lock) is rule 2, item 2 (provenance) rule 3, item 3 (disk) rule 6; item 4 is
  the main session's. Carried as ruled.

### Block 3. The check's three Highs, and rev 3.2's fold (done)
- **F1 (the fold carried into the body): fixed.** e12 has lines for carries 53, 54, 55, 56 and 57; carry 53's test
  asserts a generation change happened (e12; 3.1's note); the count reads "Sixteen carries"; return item 6 names 21-27,
  39, 50, 51, 53-57 and 61; no "pending" or "launch as is" is left outside the change history (grep).
- **F2 (bite proof against check.mjs green): fixed.** 2.6 states wave 1's two parts (a bites-only twin that passes; an
  in-place scratch switch shown failing by name, restored and sha256-checked), and e5b, 2.5 and return item 4 use
  them. check.mjs's `testBothBuilds` (check.mjs:1493-1501) then stays green in both builds.
- **F3 (owner-to-all's proof): fixed.** The proof is carry 53's per-record identity test; the group-level class check
  and the entity-section route are named as not counting.
- **Rev 3.2's fold:**
  - 3.1 equals u4s-carries.md :147-247 (checked above), carry 61 included, and 3.1's note on carry 61 matches B2.1
    (the decode lean, the drop count with a SERIES.md row) and marks/README.md §1 at d50396a (:50-71: the
    `server_hash` and `decoded_hash` items, with `replay_hash` at :70-71).
  - R27 holds at d50396a: marks.json spells every id U4e plants but owner-to-all's; U4j's one new id,
    `PM-0001-18/ws-budget-x2` (marks.json:4431), is U6's.
  - R28 holds: `TickRecord.hs` (log/src/lib.rs:121), `set_hashes` (:158), `hashes` (:168); SERIES.md :319 and :320.
  - R38 holds: 396, 443, 52 of 52, 10 steps (u4j-build.md:354-355, :388-389).
  - Section 0's line-number note is right: the diff of 30528c7's plan against today's shows one row added after :199,
    §10's preamble three lines longer and §11 two; the brief cites nothing in the preamble (old :954-:963), so every
    cite from :200 to :953 is one lower and from :964 on four lower. I spot-checked 14 plan cites against the file:
    each lands.
  - The "U4j unmerged" lines are updated (carry 54's note, section 4's lane list, U4E-Q13 (c)).
  - Two stale lines the fold missed (finding A6): section 1.4 still cites carries "§U4e (:147-228)" and carry 40 at
    ":231-239" (now :147-247 and :250-258), and the provenance paragraph says "rev 3.1, this file".

### Block 4. My findings on rev 3.2, each with its exact words (done)
Severity on CLAUDE.md's scale. None is High; none is a security finding. Every "old" phrase below is the brief's text
read with its line breaks as spaces; the fold matches it that way, once, and rewraps.

**A1, Medium: carry 61's unit test has no line in e12.** 3.1's note on carry 61 says "its unit test ... is one of your
tests (2.4)", but e12 lists every carry that names a behaviour except 61, the one the fold added. Carry 61 also states
two rules no test reaches: (a) no server hash for a snapshot the room did not send, and (c) a tick is resolved lost only
once it has left the frame ring. That is the check's F1 again, for one carry; the carry's own words keep it from a
High.
- In e12, end carry 57's bullet with ";" in place of its final ".", and add after it:
  "  - carry 61: `ClientEndpoint` fed tick t+1's snapshot before tick t's (as L1's reorder step delivers them) files t
    under t (its hash if the client decodes the stale group, or lost if it drops it, the drop counted; never the
    sentinel), and one datagram received twice files one hash (the carry's unit test); a tick with a group missing is
    not resolved lost while it is younger than `npt_sim::ACK_RING_TICKS` ticks (by the newest tick the client holds)
    and is resolved lost (`snap_incomplete` 1, no hash) once it is that old (carry 61 (c)); and in carry 54's test the
    room exposes no server hash for client k at the skipped tick (carry 61 (a))."

**A2, Medium: e7's split clause may be out of reach for a legitimate encoder.** One group carries at most 1,000 B of
payload (`Limits::SNAPSHOT`, limits.rs:25), and one client's datagrams in one interval stay within about 1,067 B,
counting 28 B each (catalogue :88). Two groups therefore carry at most about 1,010 B, 10 B more than one. An encoder
that never splits for that gain is correct, and then no tick of e7's run splits: e7 fails as worded, and the builder
stops or bends the encoder to pass a test. The mirror's split case still needs a comparison.
- In e7, replace "at least one tick was sent as 2 or more groups," with: "at least one tick was sent as 2 or more groups
  (if the encoder never splits a tick at the window's budget, which is likely: one group carries up to 1,000 B of
  payload and two at most about 1,010 B of the about 1,067 B a tick allows, counting 28 B each: catalogue :88,
  limits.rs:25; then e2's forced split tick has the mirror compared with the client's hash the same way, and your
  report says which),".

**A3, Medium: owner-to-all's route needs the section's limits, and carry 53's test needs a snapshot on every tick.**
Writing every other slot's own-state records into each client's section (48 slots × 10 records) passes the own-state
section's 8-bit count (snapshot.rs:26; `TooMany` at :526-530) and the 1,000 B datagram. The encoder then refuses, carry
54 skips the snapshot, and carry 53's check ("in every `Snapshot` ... sent to it") reads nothing and passes: the bite
cannot show its defect. Catalogue item 5 asks for "one `owner` group to every client" (catalogue :254), so one slot is
enough.
- In 2.6's owner-to-all row, replace "which writes other slots' own-state records into every client's own-state section
  (carry 53's route)" with: "which writes one other slot's own-state records (its Soldier, OwnState and Effect rows)
  into every client's own-state section (carry 53's route; catalogue :254's "one `owner` group to every client"),
  within the section's 8-bit count (snapshot.rs:26, :526-530) and each group's 1,000 B, so that every bitten snapshot
  still encodes: a refused one is skipped under carry 54 and would leave carry 53's check nothing to fail on".
- In e12's carry 53 bullet, after "the rows whose owner is that slot (`Scene::effect_rows()`);" insert: " each client
  receives a `Snapshot` on each compared tick, and the test fails if one has none (a snapshot skipped under carry 54
  would make the check vacuous);".

**A4, Medium: the room must handle every encoder error, not only `OutOfRange`.** Carry 54 names `OutOfRange`; the
encoder also returns `TooMany`, `TooLarge` and `NotCanonical` (error.rs:42-58). The nearest trap is the event section's
8-bit count: after a 1 s gap the stress scene queues about 350 events (§4.7, u4-build-plan.md :336 as the file stands),
more than one snapshot can carry by count or by bytes. An `unwrap` there panics the room on the first burst. Carry
54's "without a panic, `unwrap` or `expect`" is the rule for the encoder's refusals; this extends it to the other
kinds, and narrows nothing.
- In 2.1's Encoder bullet, after "and counted per client; the next tick encodes afresh." add: "The other `EncodeError` kinds (`TooMany`,
  `TooLarge`, `NotCanonical`: error.rs:42-58) take the same path, counted under their kind. A correct encoder meets
  none: it keeps each section within its count (own-state 8 bits, entities 16, events 8: snapshot.rs:26, :526-530)
  and each group within `Limits::SNAPSHOT` (1,000 B: limits.rs:25), so an event backlog (after a gap, up to the
  queue's cap) goes out over as many snapshots as the remainder needs, carry 50 still bounding its age."

**A5, Low: 2.8 does not name the bites-only constructors.** §4.11 names `Room::with_bite` (:400 as the file stands;
the brief's numbering :398-400); the pump and the endpoint need a per-instance bite choice too (2.2, 2.3). The words
below use the brief's numbering.
- In 2.8, after "and, bites builds only, `BITES` (:194; U4E-Q3)" add: "; the bites-only constructors that choose a bite
  per instance (`Room::with_bite`, which §4.11 :398-400 names, and the pump's and the endpoint's, where they take the id
  rather than a `Quant` or grid config passed in) are expected: list them as beyond §4.1's rows, for
  engine-architect".

**A6, Low: three stale cites.**
- The event's wire text is snapshot.rs:31-32 (at 39f4290 and at the head), not :29-30: replace "snapshot.rs:29-30"
  with "snapshot.rs:31-32" in all three places (2.1 twice, 2.3 once).
- Section 1.4: replace "carries §U4e (:147-228), quoted in section 3; carry 40 (§U4r, :231-239)" with "carries §U4e
  (:147-247), quoted in section 3; carry 40 (§U4r, :250-258)".
- The provenance paragraph: replace "and revised once for that check (rev 3.1, this file)" with "and revised once for
  that check (rev 3.1); rev 3.2 is the main session's fold at U4j's merge, and rev 3.3, this file, its fold of the
  advisor's check (advisor-u4e-brief.md)".

**A7, Low: the base counts' "otherwise" branch is dead.** `<base>` holds U4j's merge and the carry-51 commit, so it
differs from 39f4290 under spikes/ whatever happens.
- Section 4 rule 5: replace "If `<base>` differs from 39f4290 under spikes/, the main session gives `<base>`'s counts at
  launch (7.2 item 1)." with "At d50396a (U4j's merge): measured 396, bites 443, self 52 of 52 (R38). `<base>` also
  holds the carry-51 commit, so it differs from both under spikes/, and the main session gives `<base>`'s counts at
  launch (7.2 item 1)."
- 7.2 item 1: replace "Otherwise the counts are 365, 412 and 52 of 52, with 10 steps." with "(`<base>` holds U4j's
  merge and the carry-51 commit, so this run is always needed; the last counts known are d50396a's: 396, 443, 52 of
  52, 10 steps.)"

### Block 5. The brief's open questions (done)
Plan cites inside the quoted brief words use the brief's numbering (the plan at 7375cdcf…, section 0).

**U4E-Q14 (the 1 s gap on Loopback): (b), with the gap tied to the ring, not to 30 Hz.**
- Why the 1 s gap cannot work on Loopback. Call L the last snapshot delivered before the gap and F the first tick
  after it. B is never newer than L, so B's age at F is at least F − L, which is the number of snapshots lost plus 1.
  The server's sends are ⌊k × 10⁹ / 30⌋ ns apart (U4b's `due_ticks`), so s(k + 30) − s(k) is exactly 10⁹ ns, and a
  half-open 1 s gap holds exactly 30 sends: F − L = 31. If the client's first input after the gap (which acknowledges
  L) arrives before F's encode, B = L at age 31, inside the ring (ages 0 to 31 kept: §4.5): no eviction. So e5a would
  fail at one alignment or both, and the builder would stop. The brief's own reading (31 or 32) is right.
- The fix. A gap of `npt_sim::ACK_RING_TICKS` tick periods holds 31 or 32 sends at 30 Hz, whatever the alignment and
  whichever way the builder rounds the ns (32 × 10⁹ / 30 is not whole), so F − L ≥ 32 and B leaves the ring. Tying it
  to the constant, not "1 s plus two tick periods", keeps it right at 60 Hz too (the ring is 64 ticks there, and
  1 s + 2 periods would be 62 sends, B at 63, kept).
- Not looser: -18's judged runs are L1 B1, whose 1 s bursts add the RTT to B's age (the brief's U4E-Q14 text); this
  changes only the Loopback unit proof, which exists to show the bite can bite (§4.5 :298-299). No ADR or mark word
  changes. The gap is a test's input, not a constant the build needs, so the F2 ruling's point (3) does not apply; it
  stays a `let` computed from the constant by path and the rate the test passes in (carry 39: no const or static).
- **Words, the brief:**
  - e5's lead: replace "e5 a 1 s gap evicting B (:718; §4.14 :460's unit proof, the server half), as two tests," with
    "e5 a gap evicting B (:718's "a 1 s gap evicting B": on Loopback the gap is `npt_sim::ACK_RING_TICKS` tick periods,
    U4E-Q14 (b); §4.14 :460's unit proof, the server half), as two tests,".
  - e5a: replace "The ring holds 32 ticks = 1.067 s at 30 Hz, so a 1 s gap (30 ticks) evicts B only with B's age at
    the gap's start counted in." with "The gap lasts `npt_sim::ACK_RING_TICKS` tick periods (1.067 s at 30 Hz: 1 s plus
    two tick periods; U4E-Q14 (b)), its length in ns on the injected clock computed in the test from that constant by
    path and the rate the test passes in (a test's input, never a const or static: carry 39). Why: every snapshot whose
    send falls in the gap is lost, at least 31 at 30 Hz, so B's age at the first tick after the gap is at least 32 at
    either alignment, while a 1 s gap loses exactly 30 and leaves B at 31 or 32 on Loopback, at the ring's edge."
  - e5a: replace "the precondition holds only if both give at least 32. If either leaves B at age 31 or less, report
    both (§4.14 :460's "a 1 s gap evicts B" then does not hold on Loopback), and do not lengthen the gap or choose the
    alignment on your own (U4E-Q14)." with "the precondition holds only if both give at least 32
    (`npt_sim::ACK_RING_TICKS`); if either gives less, stop that part and report both, and change neither the gap nor
    the alignment on your own. e5a also runs a 1 s gap at both alignments and reports B's tick and age there (reported,
    not asserted)."
  - 2.5: replace "through a 1 s gap in delivery:" with "through a gap in delivery (e5a's: `npt_sim::ACK_RING_TICKS`
    tick periods, U4E-Q14 (b)):".
  - 2.5's gap bullet: replace "that drops every datagram both ways for 1 s on the injected clock is inside the lane;
    its length in ns, never a `*_TICKS` constant (carry 27)." with "that drops every datagram both ways for e5a's gap
    on the injected clock is inside the lane; its length in ns, from `npt_sim::ACK_RING_TICKS` by path and the rate
    passed in, never a const or static (carries 27 and 39)."
- **Words, the plan** (the main session's commit; condition 12's list of allowed edits gains them by this ruling):
  - §4.14's -18/no-fallback row: replace "a 1 s gap evicts B; `burst_eviction` = 1" with "a gap evicts B (on Loopback
    `npt_sim::ACK_RING_TICKS` tick periods, 1 s plus two tick periods at 30 Hz, since there a 1 s gap leaves B at the
    ring's edge: the advisor's U4E-Q14 ruling); `burst_eviction` = 1".
  - §5 U4e, Tests: replace "a 1 s gap evicting B;" with "a gap evicting B (on Loopback `npt_sim::ACK_RING_TICKS` tick
    periods: U4E-Q14);".
  - §5 U4e, -18's bite end to end: replace "through a 1 s gap in delivery:" with "through a gap in delivery (on
    Loopback `npt_sim::ACK_RING_TICKS` tick periods: U4E-Q14):".
  Each phrase occurs once in the plan (checked).

**U4E-Q12 (-22/unbounded's planting unit): (c).** U4e plants and unit-shows it; U4f runs it judged; the plan's Bites line
gains it.
- Why: §4.14's own rule is "planted where its code lives, shown to change state by a unit test in that sub-unit, and
  run judged in the sub-unit that moves its mark" (:453-454 in the brief's numbering). U4e builds the queue and its cap
  (:707; U4E-Q6); U4f builds no queue (:792-795) and moves -22 (:801). The Bites line missed it, as it missed lt-cmd-seq
  and lt-press (U4E-Q1, the same slip). (b) would have U4f edit a site U4e wrote and unit-shows nothing of its own.
- What the bite needs from the room: -22's first check reads `tick.v.ack_starved_headroom_count` (marks.json,
  PM-0001-22: "it reaches the cap and never passes it", min == 0), whose writer SERIES.md:21 names as npt-room ("cap
  minus queue length ... negative = over the cap"). So the room exposes each client's queue length beside the cap.
- **Words, the brief:**
  - 2.1's EventQueue bullet, after its last sentence (carry 50's), add: "By U4E-Q12 (c), the cap is also
    PM-0001-22/unbounded's site (§4.14 :474; 2.6), so the room exposes, per client and tick, its queue's length beside
    the cap (the headroom, cap less length, signed: below 0 only under the bite), which U4f's -22 runs write as
    `tick.v.ack_starved_headroom_count` for the never-acking client (SERIES.md:21)."
  - 2.1's `npt_room::BITES` bullet: replace "-62/server-echo, and owner-to-all's: U4E-Q13)" with "-62/server-echo,
    -22/unbounded (U4E-Q12), and `catalogue-5/owner-to-all` (U4E-Q13))".
  - 2.6's table: after the PM-0001-62/server-echo row, add: "| PM-0001-22/unbounded | the event queue's cap: under the
    bite the queue keeps every event past its cap, with no overflow resync and no `Kick::Overflow` (§4.14 :474;
    U4E-Q12 (c)) | a never-acking client: in the clean build its headroom (cap less queue length) falls to 0, never
    below, and one resync follows (carry 57's test); under the bite it goes below 0 with no resync and no kick (§4.14
    :474's "headroom below 0"); the test may give the room a smaller cap, as data, to stay short. U4f runs it judged
    over a full S30 (plan :801) |".
  - 2.6's bullet: replace "-22/unbounded's site is U4e's `EventQueue` cap, and the plan's U4e Bites line omits it:
    U4E-Q12." with "-22/unbounded's site is U4e's `EventQueue` cap: U4e plants and unit-shows it, and U4f runs it judged
    (U4E-Q12 (c); the plan's Bites line gains it)."
- **Words, the plan** (the main session's commit): §5 U4e's Bites line: replace "server-echo (-62, the room's routing),
  owner-to-all (catalogue item 5's)," with "server-echo (-62, the room's routing), unbounded (-22, the event queue's
  cap; U4f runs it judged: U4E-Q12), owner-to-all (catalogue item 5's),".

**U4E-Q13 (the owner-to-all bite's id): (a), the id `catalogue-5/owner-to-all`.**
- Why: no mark holds catalogue item 5 (the visibility classes are proven at slice 3: ADR 0001:1687-1688, the brief's
  cite), so (b) would invent a PM row that judges nothing and (c) would put a bite under a mark that does not judge
  it; both misstate what a run proves. A name outside `PM-<ADR>-<row>/<name>` cannot be mistaken for a mark's bite:
  check.mjs's `BITE_ID` (check.mjs:689) does not match it, and npt-log takes a header's bite as any string
  (log/src/lib.rs:71-72). The cost is that check.mjs's one-home grep cannot see it, so a test holds its one home.
- **Words, the brief:**
  - 2.1: as under Q12 above (the `npt_room::BITES` bullet).
  - 2.6's owner-to-all row: replace "Its id: U4E-Q13 |" with "Its id: `catalogue-5/owner-to-all` (U4E-Q13 (a)) |".
  - 2.6's bullet: replace "Every id U4e plants is marks.json's spelling, but owner-to-all's, which marks.json does
    not have (R27; U4E-Q13)." with "Every id U4e plants is marks.json's spelling but one: `catalogue-5/owner-to-all`
    (U4E-Q13 (a)), which marks.json does not have and check.mjs's bite-home grep does not read (it reads only
    `PM-<ADR>-<row>/<name>`: check.mjs:689). So npt-room's bites test pins `npt_room::BITES` and checks each of its
    PM-form ids against marks.json, as server/tests/schedule.rs:345-353 does, exempting that one id by name; and a
    bites-only test in npt-server's tests (which reach every crate's list under `--workspace --all-features`) asserts
    that no other list holds it (`npt_proto::BITES`, npt-sim's `World::BITES` and `OWN_COPY_SNAP_BITES`,
    `npt_server::BITES`, `npt_client::BITES`). U4r's `--bite` dispatch selects it by that string."
- For U4r's brief, later: the class audit's runs (plan :750) select that id, and their headers list no mark, so the
  judge never reads it.

**Section 8 and 7.2, for all three:**
- Section 8's ruled table: after the U4E-Q11 row add "| U4E-Q12 | (c): U4e plants and unit-shows -22/unbounded at the
  event queue's cap; U4f runs it judged; the plan's Bites line gains it | 2.1, 2.6 |", "| U4E-Q13 | (a): the id
  `catalogue-5/owner-to-all`, in `npt_room::BITES` only, its home held by tests | 2.1, 2.6 |" and "| U4E-Q14 | (b), the
  gap tied to the ring: `npt_sim::ACK_RING_TICKS` tick periods on Loopback; the 1 s ages reported | 2.4 (e5), 2.5 |".
- Section 8's "**Open**" heading: replace "**Open** (new in rev 3, and U4E-Q14 in rev 3.1; only where no ruling gives
  the answer; each self-contained):" with "**Ruled at the advisor's check** (advisor-u4e-brief.md; the questions as
  asked, kept for the record; the rulings are in the table above):".
- 7.2 item 4: replace "the advisor's answers, written into 2.6, 2.1 and 2.4 (e5a)." with "answered by the advisor's
  check (advisor-u4e-brief.md) and written into 2.1, 2.4 (e5), 2.5 and 2.6 (done)."

### Block 6. F2-ROUTE: U4c's netcode F2 (Medium, open) (done)
**Ruling: a carry for U4d2, the next free entry (66 at this writing), in u4s-carries.md's "U4d2 or the main session"
section; U4r's brief takes it if U4d2's is final first. F2 keeps its Medium label and stays open until the carry is
met. L-c8's judged half PASS stands, and its hash half holds today on the evidence below.**
- What the evidence shows (u4c-lc8-rerun.md; u4c-build.md:79-84; u4c-review-test.md:95-99): the merged build's npt-lab
  runs of P2, B1 and P4 seed 1 print whole-v2-log hashes (43745683ab36d67b, 722f20713e18352e, 432693efb9698c05) equal
  to those of the builder's scratch logs of the same runs; on those logs' bytes the builder's scratch harness gave
  SHA-256(v1 tag + datagram entries) equal to U2's 913db8e9…, feabf2f0… and b8a04f2f…, in the builder's build and
  again in the test-engineer's own build. Equal whole-log hashes mean equal logs, so the merged build's datagram
  entries are U2's.
- What it lacks: the datagram-entry hash comes from an uncommitted scratch harness, so no later run of -3's set can show
  it without rebuilding that scratch. That is F2's point, and it still holds. It is a reproducibility gap, not a doubt
  about U4c: no Critical or High, no security finding.
- Why U4d2: the change is in crates/lab (probe.rs and main.rs), which is U4d2's alone in wave 2 (u4-build-plan.md :842,
  :862 as the file stands); U4d2 already edits npt-lab (its `hash` subcommand), so no new lane is opened. It needs no
  npt-l1 change: `FateLog::for_each_canonical` is public (l1/src/lib.rs:351), and U4c's committed test reads the
  same bytes through it (l1/tests/l1.rs:481-532). Not U4e: crates/lab is never U4e's (condition 10 (b)).
- Not a "for later" feature: it is the evidence form an approved test (L-c8) names; -3's L1 set was re-run because U4c
  changed npt-l1's model, and the next model change would need the same re-run.
- **The carry's words** (copy as they are):
```
66. **U4c's netcode F2, Medium (u4c-review-netcode.md:53-54; L-c8's judged re-run, u4c-lc8-rerun.md; the advisor's U4e
    brief ruling, F2-ROUTE):** "npt-lab's probe also writes the SHA-256 of (v1's tag + the v2 fate log's datagram
    entries), the hash §5 U4c's L-c8 compares with U2's committed 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4
    seed 1, beside the whole log's SHA-256: in the run log's header and in its summary line, so that -3's script
    (ops/runs/c-u2-pm-0001-3.mjs) records it unchanged. It reads the log through npt-l1's public
    `FateLog::for_each_canonical` and edits no crates/l1 file; v1's tag and the datagram entry's form are those U4c's
    test `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` reads (l1/tests/l1.rs:481-532). U4d2's
    report shows the three values from probe runs of those three runs (the -3 set's own arguments), each equal to
    U2's, with the commands it ran. Until then L-c8's hash half rests on U4c's scratch harness through equal
    whole-log hashes (u4c-lc8-rerun.md). If U4d2's brief is final without this entry, U4r's brief takes it word for
    word, U4r in place of U4d2."
```
- PR #40's report lists F2 as open (Medium, netcode-architect's, U4c), with carry 66 as its route.

### Block 7. The verdict (done)
**APPROVE, on conditions L1 to L4.** The build may start once `<base>` is filled and L1 to L3 are done.
- Why not REVISE: rev 3.1 fixed the check's three Highs as claimed, rev 3.2's fold is right, every test, negative case,
  bite and carry of §5 U4e is in the brief and testable, W1-TARGET's rule is in the build rules, and nothing goes
  beyond the plan. My findings are four Mediums and three Lows, each with exact words a script can fold, and the three
  open questions are ruled with exact words. Nothing needs a second check unless the fold goes beyond those words.
- Risk tier 1 (spike code inside PR #40, when U4e builds); this ruling is a notes-folder text. Reviewers for U4e's build
  stay the plan's (netcode-architect, security-auditor, engine-architect, rust-reviewer, test-engineer: plan :734-735
  as the file stands), at xhigh, counted toward the three-agent limit (condition 10 (c)).

**Conditions (the main session's):**
- **L1. Fold this ruling into the brief by script** (rev 3.3): A1 to A7 (block 4) and the U4E-Q12, Q13 and Q14 words
  (block 5), each old phrase matched with its line breaks read as spaces. I checked each against rev 3.2: each occurs
  once, but "snapshot.rs:29-30" (three times, all replaced). 3.1 stays untouched (its `diff` against carries :147-247
  still empty), and the fold is recorded under the brief's title. If a phrase fails to match, or the fold must change
  any other word, it comes back to me; otherwise there is no second check.
- **L2. The plan and carries text, in the main session's own commit:** u4-build-plan.md's four edits (U4E-Q12's Bites
  line; U4E-Q14's §4.14 row, §5 U4e Tests and -18 lines; each phrase occurs once), allowed by condition 12 through this
  ruling; u4s-carries.md gains entry 66 (block 6) in its "U4d2 or the main session" section, and U4d2's brief names it.
- **L3. The carry-51 commit before launch** (7.2 item 2): `Scene::effect_rows()` and its §4.1 line, then 7.2 item 1's
  counted run gives `<base>`'s counts, every check.mjs step passing. U4e's review round reads its diff from the commit
  before the carry-51 commit, so engine-architect (a new pub item: A19) and rust-reviewer see the accessor with the
  code that uses it.
- **L4. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in a worktree, under W1-TARGET's lock,
  beside U4d2's lane, with at most three agents at a time counting reviewers.

**For later briefs (no action now):**
- U4r: the class audit selects `catalogue-5/owner-to-all` by that string, and its runs' headers list no mark (U4E-Q13).
- U4f: -22's judged runs write the room's per-client queue length and cap as `tick.v.ack_starved_headroom_count` for
  the never-acker, and select -22/unbounded (U4E-Q12).
- The main session, at U4e's report: if U4e's client drops stale groups, the readings list gains B5's entry
  (advisor-u4j-r2.md:264-268), and carry 62 (g)'s drop count applies.

## What I checked, and what I did not
- **Checked, by reading:** the files in block 1; 3.1 against u4s-carries.md :147-247 by my own `diff` of the two line
  ranges (no output); every phrase this ruling replaces, counted in the flattened brief or plan (each once, but the
  three planned repeats); 14 plan cites (old numbering against today's file, by section 0's note), about 45 code cites
  at the head (seam.rs, snapshot.rs, quant.rs, msg.rs, wrap.rs, hash.rs, input.rs, limits.rs, error.rs, ticks.rs,
  history.rs, world.rs, scene.rs, greybox.rs, link/src/lib.rs, server and client manifests and lib.rs, check.mjs's
  bite and test steps), and R27, R28 and R38 at d50396a; marks.json's -18 and -22 entries; SERIES.md :21, :128, :150,
  :319-320; the catalogue's rows and §6 item 5; the ADR row the brief cites for Q13 (0001:1687-1688).
- **Arithmetic I did:** Q14's send counts (s(k + 30) − s(k) = 10⁹ ns exactly; 32 × 10⁹ / 30 not whole, so 31 or 32
  sends); A2's split gain (1,066.67 − 56 = about 1,010 B against 1,000 B); A3's own-state count (48 × 10 records
  against 255).
- **Not checked:** I built, ran and planted nothing (the main session holds the shared target's exclusive lock now).
  Q14's arithmetic assumes the server sends at U4b's schedule on the injected clock and Loopback delivers at the send
  instant (link/src/lib.rs:128-131), both as merged; the builder's e5a is what shows it. A2 rests on the catalogue's
  budget and `Limits::SNAPSHOT`, not on a built encoder. Whether L1 B1's bursts evict B is U4r's (`burst_eviction`
  logged). I did not re-derive U2's datagram hashes for F2: I read the builder's and the test-engineer's records.

## Decisions needed (Mustafa)
None. Every point here is a technical or test choice inside the documented vision, or a route between lanes: mine to
decide. No word or decision of Mustafa's changes, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)

````

### The advisor's U4j round-2 ruling B2.1: when a tick is written lost, and a stale group (advisor-u4j-r2.md)
_`.checkpoints/netcode-proof-test/advisor-u4j-r2.md` lines 88-129 (sha256 7efa9410323f), as they are:_

````text
### B2.1 NC-U4J-R2-N1, when a tick is written lost under L1's reorder: a CARRY (U4e and U4r), and README §1's words
- **The finding holds.** L1 reorders datagrams on every profile where the jitter or the reorder step can
  (l1/src/lib.rs:15-23: 1 % of a flow's datagrams overtaken by the next; P0 finds nothing to swap). README §1's
  `decoded_hash` item (worktree README.md:55-65) says no hash is dropped and a tick none of whose groups arrived is
  lost, but not when a writer may decide a tick is lost, nor what a late group of an older tick is. Either wrong
  reading fails a correct build (every P3 run INVALID through `crash`, or a FAIL through the sentinel); neither gives a
  false pass.
- **The rule (the reviewer's sentence, with three additions: the frame ring, "never acknowledged", and the drop's
  count below):**
  - a tick is written lost only once none of its groups can still decode: once it has left the client's frame ring
    (at `npt_sim::ACK_RING_TICKS` ticks old: plan §4.5, the client keeps its frames for the AckRing's ages), or at
    the stop (on L1 the runner may decide it there). The reviewer's "none of its groups can still decode", made a
    point a writer can test;
  - a group of a tick older than one already decoded is decoded and hashed under its own tick like any other, or, if
    the client drops it as stale, it is never acknowledged and that tick is lost (`null`, `tick.s.snap_incomplete` 1),
    never the sentinel. The addition "never acknowledged": plan §4.5 lets the server take as baseline only a tick
    whose every group the client acknowledged, and the client must hold every baseline the server can pick; a group
    dropped but acknowledged would end in the sentinel ("no baseline") later, a correct build failing;
  - the sentinel stays for §4.6's causes (no baseline, a bad header, a size or decode error).
- **Why this does not loosen -17, and what keeps -58's P3 honest.**
  - ADR 0001's -17 row (:1605) asks that "100 % of decoded snapshots rebuild the server's quantised state"; a group
    the client drops undecoded is not a decoded snapshot, and the tick stays resolved (lost, not compared) under
    FX-10's coverage rule. On L1 P0 nothing reorders, so R2.4's setup check (snap_incomplete sums to 0 on L1 P0)
    still catches a client that marks ticks lost without cause.
  - But -58 runs "at P3 (so reordering crosses the wrap)" (ADR 0001:1607). A client that drops every late group never
    decodes the late side of a reordered wrap, so those snapshots go uncompared. Both behaviours are legitimate client
    designs, and forcing a decode could demand baselines older than the frame ring keeps (a late group right after
    a B1 burst), so I do not forbid the drop. My lean is the decode: it keeps -58's P3 comparing the late snapshots
    its row exists for.
  - So the choice is made visible: U4e's report says which its client does; a client that drops counts each drop
    per client under a name with its SERIES.md row (carry 54's pattern), and U4r's -58 P3 contract run reports that
    count beside `decoded_hash_compared_count`.
  - If U4e's client drops, the readings list (B5) gains one entry; if it decodes, none is needed.
- **N6's two writer rules fold in** (the reviewer's request): the head (every client admitted, and its first snapshot
  resolved, in the warm-up) and the stop (the room sends no snapshot of a tick it does not log).
- **Where the words go:**
  - README §1's `server_hash` and `decoded_hash` items gain them in the text commit (B7.1, T1 and T2), so the
    contract both sides read holds one rule. This changes no judge code and no pass bar.
  - u4s-carries.md, §U4e: entry 61 (B7.2). §U4r: entry 62 (B7.2), with the proof the reviewer names (U4r's contract
    test on a short L1 P3 run with the reorder step on: no `crash` footer, `decoded_hash_mismatch_count` 0, no gap).
  - The U4e brief (rev 3, in its check now) re-copies §U4e by script before the advisor's check of the brief, since
    3.1 is word for word from u4s-carries.md.
````

### The advisor's U4j round-2 ruling B5: the readings, including the one for a client that drops stale groups (advisor-u4j-r2.md)
_`.checkpoints/netcode-proof-test/advisor-u4j-r2.md` lines 248-281 (sha256 7efa9410323f), as they are:_

````text
## B5. At the merge: the readings, the notes and PR #40's report (confirmed, with what round 2 adds)
- **R3's readings (a) to (f) go into u4j-build.md, all six, as ruled.** A new section "## Readings, as ruled", in the
  merge commit, copied by script by line range: advisor-u4j.md :472-506 (R3, whole), then this file's B4.2, B4.3
  and B3's NC-U4J-R2-N2 line. As ruled:
  - (a) C4's "at least one log" form: ACCEPTED (FU-1 tightens its bite direction);
  - (b) a bite run takes its set's measured runs' practice run: CHANGED by FX-3 (c), and now with B4.2's step (two
    practice runs in force: INVALID);
  - (c) `params.config_hz` for the ruling's `machine.config_hz`: ACCEPTED;
  - (d) a bite run that compared nothing is INVALID: ACCEPTED;
  - (e) the Arm rule's key: CHANGED by FX-5 (LG on aarch64, the room on x86_64, each name from its own process, an
    orphan counts everywhere, a missing replay entry a gap);
  - (f) the decoded-hash figures read every phase: ACCEPTED.
  None of these reads Mustafa's words or an ADR row, except (e)'s orphan part, below.
- **R5's orphan reading goes into the readings list** (u4-build-plan.md §10's preamble and §11), its words copied by
  script from advisor-u4j.md :538-540, in the merge commit (B7.5 gives the edits). It reaches Mustafa with the
  readings PR before R5, for his go then; it is stricter.
- **One more entry, only if U4e's client drops stale groups** (B2.1), added when U4e's report says so, in these words:
  "PM-0001-17's '100 % of decoded snapshots' (ADR 0001:1605; PM-0001-58 at :1607 by its check) read as leaving out a
  group the client drops undecoded as stale (a group of a tick older than one it already decoded, never
  acknowledged, each drop counted): that tick is lost, as a tick none of whose groups arrived, and is not compared
  (the advisor's U4j round-2 ruling, B2.1)."
- **PR #40's report** (its U4j part, when U4's report is folded; until then the card's closing checklist under Next
  steps holds the line) names, by id, severity and owner:
  - the open security findings, which keep PR #40 needs-mustafa: FU-1 (SEC-F7, Low), FU-2 (SEC-F9's later half,
    Medium), FU-3 (SEC-F10, Low), FU-4 (SEC-F12, Low), FU-5 (C3's flag, Low), and the new FU-7 (SEC-R2-N2, Low);
  - FU-6 (mine, A-U4J-1, Medium: the message-size condition), open;
  - round 2's follow-ups and carries: entries 61 to 65 (B7.2) and C-U6-9 to C-U6-11 (B7.4);
  - the review count: two rounds, F1 (High) closed in round 2, no round 3.
- **§4.1's U4j public items** (u4-build-plan.md, the merge commit; B7.3): found by diff against 411eeee, not from
  memory. The fixer's "only `Mark::ws_bound_line`" covers its fix pass alone; U4j as a whole adds more.
- **Round 1's carries are not yet in u4s-carries.md** (advisor-u4j.md "Carries": U4e, U4r, U6; nothing numbered
  them, and the U4e brief rev 3 says it "relies on nothing of U4j's"). They are numbered in the merge commit, merged
  with round 2's: entries 61 and 62 (B7.2) and C-U6-9 to C-U6-11 in plan.md's U6 row (B7.4). Then the U4e brief's
  3.1 is re-copied from §U4e, and its R28 row ("U4j has not merged") is redone at launch.
````

### Mustafa's words on the effect row's owner, 2026-11-11 (his words)
_`.checkpoints/netcode-proof-test/history.md` lines 340-340 (sha256 c62ac831ff2e), as they are:_

````text
  - The Low about the effect row's owner: defer it to U4e in writing. A Low never triggers a push.
````

### §4.5 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 292-304 (sha256 686a583c37e3), as they are:_

````text
### 4.5 Baselines, groups and the window (Q12, Q13)
- A snapshot that splits goes out as entity groups that each decode alone, all against one baseline B: the newest sent
  snapshot whose every group the client acknowledged (the input's ack carries the low 16 bits of the tick plus a group
  mask). A partly acknowledged tick is never a baseline.
- **The AckRing keeps ages 0 to 31** (age = this tick minus the snapshot's tick; 32 entries at 30 Hz), pinned by a unit
  test at both edges (age 31 kept, age 32 evicted: T15). A B older than that means coding against the per-type
  defaults. The client keeps its complete frames for the same ages, so every B the server can pick, the client holds.
- **Each burst logs `event.burst_eviction`** (1 if the last acknowledged baseline had left the ring at the first tick
  after the burst, else 0), so -18's bite shows whether it could bite.
- **The window (A7):** the room owns each client's 256 kbps window (plan.md:503's room column). The pump reports per
  client the bytes it sent since the last tick (pings, pongs: payload + 28 B each); the window counts them, and the
  encoder's budget is what remains. plan.md §4.4's list of per-tick inputs gains "bytes sent by the server side".
- The encoder exposes one tick's whole snapshot as a unit, for WsServer's gate at U6 (plan.md:1225).
````

### §4.6 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 306-328 (sha256 686a583c37e3), as they are:_

````text
### 4.6 The client-held hash (Q11)
- **What both sides hash** with one function, `client_hash`: the full 32-bit tick; the client's entity set for that
  tick in entity-id order, each entity's id, kind and grid integers; the own-state section; the event section's events
  as (32-bit event number, kind). The authority-only streams stay out (plan.md:531). The output is never 0.
- **The server side** hashes what the encoder intends the client to hold after decoding that tick: baseline B plus this
  tick's delta over the client's set (an entity skipped by the priority accumulator at B's value), the own-state
  section and the events as sent, with the server's own 32-bit tick and event numbers.
- **The client side** hashes its own held state for that tick: the frame it built, taken as the floats it holds for
  interpolation and prediction and turned back into grid integers through `grid`, with the 32-bit tick and event
  numbers as it rebuilt them. A correct client gives the same integers (an exhaustive round-trip test, U4s s2); a
  client that places an entity a cell off, rebuilds the wrong tick or numbers an event wrongly does not.
- **When the client logs:**
  - a tick whose every group decoded: its hash, under the tick it rebuilt;
  - a tick with a received group the client could not decode (no baseline, a bad header, a size or decode error): the
    sentinel 0, which always mismatches. In an honest run this never happens (the client holds every baseline the
    server can pick, and L1 neither corrupts nor duplicates: l1/src/lib.rs:42-43);
  - a tick with a group lost and none refused: nothing, and `tick.s.snap_incomplete` = 1 for that client.
- **The figure** (`decoded_hash_mismatch_count`, U4j) counts (client, tick) pairs whose hashes differ, plus every client
  hash at a (client, tick) the server never logged (a client that labels a frame with the wrong tick); server-only
  pairs are loss, not mismatches; 0 compared fails. `decoded_hash_compared_count` and the incomplete count are reported
  beside it.
- **-57's pairing (T20):** npt-lab pairs (client, tick, entity) only for fields that tick's snapshot carried and the
  client decoded, against the server's pre-grid value at that tick.
````

### §4.7 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 330-336 (sha256 686a583c37e3), as they are:_

````text
### 4.7 Events (Q14)
- The per-client event queue is bounded (cap as data). On overflow the room forces one resync (the FullState clears the
  client's unacknowledged events: catalogue Q8, :282). If that FullState is not acknowledged within
  `ticks_for_ms(2_000)` (60 ticks at 30 Hz), or the queue overflows again inside it, the room sends Kick(overflow) and
  drops the client (net F119). Event numbers are 16-bit on the wire, rebuilt to 32 bits by `Wrap`.
- The cap: the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), recorded. At
  about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every burst.
````

### §4.9 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 343-358 (sha256 686a583c37e3), as they are:_

````text
### 4.9 Lag compensation
- `HitHistory` (§4.2) stores `HitVolumes` per tick: soldiers, vehicle parts, and R-10's cover items (doors, the wall
  stand-in, vehicle part transforms: net F105). Its capacity comes from the measured test cap plus a margin (U4h),
  recorded, at most 1 MB (10^6 B, as PM-0001-7 judges it) (proposed 64 ticks, about 192 KB at about 3 KB a tick:
  net F106).
- `rewind_at` (npt-room) turns a command's accepted view time into a `Rewind` (tick_a, fraction, cover), or replaces the
  view time by the estimate and counts it, or clips to the cap and counts it. The sim blends linearly, as the client's
  Interp drew it (net F107).
- **The tolerance** (ADR 0001:358-395): a view time is accepted within one snapshot interval of the server's estimate
  (RTT + announced interpolation delay + input-buffer wait + half a tick). **The RTT comes from the pump (A6):** for a
  `wt` or L1 session the link's `rtt_ns`, for a `ws` session the lowest recent sample of the server's own pings
  (ADR 0001:367-368). The pump knows each session's kind from the listener that accepted it, so the room never reads a
  run parameter. The pump runs the server's ping sender (a few a second: catalogue Ping row), which -50's WebSocket leg
  and its ≥ 145 ms gap check read.
- The cap is switchable: the test cap, then after R3 the rule cap (plan.md:818-820). Cover variants: rewound with the
  victim, and present; both recorded (plan.md:1382).
````

### §4.11 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 392-407 (sha256 686a583c37e3), as they are:_

````text
### 4.11 Determinism and hygiene outside npt-sim (Q9)
- The HashMap and HashSet type bans join room's and client's clippy.toml; new clippy.toml files for npt-server and
  npt-lab, and for npt-bots (all three with the clock ban, naming `std::time::Instant`, `std::time::SystemTime`,
  `tokio::time::Instant` and their `now`: A14). npt-bots' file is crate-wide; U6's runtime half and its npt-net glue
  then carry reviewed per-item allows, listed in U6's plan.
- **npt-room is float-free (A3):** interest, priority and the window work on grid integers; a check.mjs grep refuses
  `f32` and `f64` in crates/room/src (U4s, with its self-test bite).
- **Bites per instance (T10):** every bite is chosen by a constructor argument or a config value the caller passes
  (`World::with_bite`, `Room::with_bite`, the grid config), never a static, as `L1Net::with_bite` does
  (l1/src/lib.rs:856-868). A test runs a bite instance and a clean instance on parallel threads in one process.
- **Send (A25):** compile-time asserts that `Room<World>`, `Decoder` and `ClientEndpoint` are `Send`, so U6 can move
  them to the tick thread; none holds a `Link`.
- **Paths no L1 run reaches (T17):** a U4-built path whose only marks are L2, RS or ML parts gets a named unit test in
  U4, or moves to the unit that first runs it (AddrTable and the handshake cap move to U6).
- The runner's own twin test (two processes, same seed and scenario: equal fate-log hashes, equal per-tick server and
  client hashes); bite: a HashMap-ordered client order (marks F29).
````

### §4.14 of u4-build-plan.md (cited by §5 U4e)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 453-476 (sha256 686a583c37e3), as they are:_

````text
### 4.14 Bites that need a site or a run setting (T4, T6, risk 3)
Each is planted where its code lives, shown to change state by a unit test in that sub-unit, and run judged in the
sub-unit that moves its mark.

| Bite | Its site | The check it fails | Run setting | Unit proof |
|---|---|---|---|---|
| -17/tick16, -58/tick16 | proto's header writer and the client's tick rebuild (the 16-bit value taken as the tick) | decoded hash: post-wrap frames logged under the wrong tick (an orphan pair) or as the sentinel | -17: `tick_start` = 63,736; -58: `ticks_below_wrap` = 30 | a frame at tick 65,537 hashes differently or is refused |
| -17/client-rounding | npt-client's call into `grid` (floor in place of nearest); the id lives in npt-client (`npt_client::BITES`, `grid_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | decoded hash | any -17 profile | the floor variant differs on about half of all position values |
| -18/no-fallback | the encoder's baseline choice, never falling back to the defaults | `burst_heal` (the fresh delta) | B1, ring ages 0-31 | a gap evicts B (on Loopback `npt_sim::ACK_RING_TICKS` tick periods, 1 s plus two tick periods at 30 Hz, since there a 1 s gap leaves B at the ring's edge: the advisor's U4E-Q14 ruling); `burst_eviction` = 1 |
| -21/not-older-than | the room's tick check (≥ in place of >) | fast clock's excess commands | the fast-clock suite, option A (marks.json -21 notes) | an equal-tick command runs |
| -21/no-catchup-cap | the room's per-tick command cap | forged: unrejected | the forged suite, option A, two datagrams in one tick | 16 commands run in one tick |
| -41/server-speed-1pct | the authority step's run speed x 1.01, not `predict`'s (Q24) | untraced corrections | CO, every profile | one moving tick differs |
| -41/server-quant-differs | the authority step's own-copy snap with another rounding than `grid`'s (Q23); the id lives at npt-sim's own-copy snap side (`OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | untraced corrections | CO | the snapped copy differs from the sent one |
| -45/buffer-one-short | Interp's slot count | L1 P2: extrapolated share < 1 % | P2; size = the clean P2 runs' logged largest delay in intervals + 2 - 1; the delay never clamped to the buffer | extrapolation on a fixed P2 arrival trace |
| -48/no-buffer-wait | the estimate without the input-buffer wait | view-hit share | where the buffer settles at 2 ticks or more (P3, option B, read from -20's logged depth; below 2, the advisor is asked before a fixed minimum is set) | honest view times replaced |
| -58/lt-cmd-seq | the room's sequence-forward compare | 0 commands refused | sequence 2 below its wrap | post-wrap commands refused |
| -58/lt-press | the room's press-edge compare | 0 presses lost or doubled | presses scripted through the wrap | an edge lost |
| -58/lt-tick-offset | the client's rebuild of the header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) | 0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |
| -58/lt-event-seq | the client's event-number rebuild | decoded hash (event numbers are hashed: §4.6) | event numbers 3 below the wrap; SS's hits flow | a post-wrap event rebuilt wrong |
| -58/lt-ack | the room's rebuild of the 16-bit ack: a wrong rebuild names a snapshot never sent, and the check "the acknowledgement names a sent snapshot" refuses that datagram's commands | 0 commands refused | acks crossing the wrap | a post-wrap ack refused |
| -62/cues-from-replay | CueLog written from replays | plays at most once | suite `cues`: the replay skip off, a replay on every snapshot (as PM-0001-44: ADR 0001:1650) | a replayed shot plays twice |
| -62/server-echo | the room sending the owner its own cosmetic shot events | own cosmetic sent = 0 | `cues` | one event sent |
| -22/unbounded | the event queue's cap | headroom min = 0 | a full S30 of simulated time (marks.json -22) | headroom below 0 |
| PM-0004-21 hash-map | a bites-only HashMap of at least 32 keys whose whole iteration order folds into a state value | D-2 and U4d2: first mismatching tick = T | two processes; each logs its order digest | the two digests differ; equal digests mean "not exercised", never caught or missed |
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `^(21|22|23|24|25|26|27|39|50|51|53|54|55|56|57|61|68)\. \*\*|U4e`._

### Carry 21
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 148-149 (sha256 cdab02404b30), as they are:_

````text
21. **netcode F2 (the room's half):** "The room sorts each tick's accepted commands into the canonical order
    (seam.rs `TickInput::commands`) before `step`; -30's comparer relies on it."
````

### Carry 22
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 150-151 (sha256 cdab02404b30), as they are:_

````text
22. **netcode F5 (the room's half):** "`rewind_at` never names `tick_a + 1` beyond the newest tick pushed into the ring
    when `fraction > 0`."
````

### Carry 23
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 152-153 (sha256 cdab02404b30), as they are:_

````text
23. **netcode F12:** "The room stamps each `SimEvent` with the tick it ran (`TickInput::tick`) and its 32-bit event
    number when it queues it; the sim's events carry neither."
````

### Carry 24
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 154-159 (sha256 cdab02404b30), as they are:_

````text
24. **arch F1 (-17's half; reworded by the advisor's W1-U4E ruling, U4E-Q4 and U4E-Q10):** "`ClientEndpoint` takes
    the grid for every call it makes into `npt_sim::grid` from `npt_client::grid_with_bite` (U4s) under
    `PM-0001-17/client-rounding`; `npt_client::BITES` grows with U4e's other client-site ids (-58/lt-tick-offset and
    -58/lt-event-seq, the client's rebuilds); -17/tick16 and -58/tick16 stay in `npt_proto::BITES`, the endpoint taking
    its `Quant` from `npt_proto::Quant::with_bite` (U4E-Q4); -45's ids and -62/cues-from-replay join npt-client in U4g,
    whose code holds their sites (U4E-Q10); no id appears in two crates' lists."
````

### Carry 25
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 160-162 (sha256 cdab02404b30), as they are:_

````text
25. **arch F8:** "npt-room adds `#![deny(clippy::float_arithmetic)]` at the top of crates/room/src/lib.rs beside
    check.mjs's float grep (the grep sees types and literals, not arithmetic on a value whose type it never names), or
    says why not."
````

### Carry 26
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 163-165 (sha256 cdab02404b30), as they are:_

````text
26. **arch N3:** "`step`'s `&[SimEvent]` holds the `&mut` borrow of the sim: the room consumes or copies the events
    before it reads `hit_volumes`, `reports` or `entity_records`; if that costs a copy each tick, U4e raises a seam
    change with the advisor rather than working around it."
````

### Carry 27
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 166-168 (sha256 cdab02404b30), as they are:_

````text
27. **this pass (CMD_CATCHUP):** "`InputBuffer::CMD_CATCHUP = npt_proto::Limits::REDUNDANCY_CAP` with its const assert
    (§4.3). The tick grep exempts the name `CMD_CATCHUP` alone (check.mjs `NOT_TICK_COUNTS`); every other `*_TICKS` or
    `*CATCHUP*` const or static under crates/ but crates/net/ is never set from a literal, tests included."
````

### Carry 39
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 169-179 (sha256 cdab02404b30), as they are:_

````text
39. **rust fix review nit (u4s-fix-review-rust.md:114-115; the advisor's U4s ruling, condition 2):** "A tick count
    needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it
    by path. Why: check.mjs:627 lets an initializer that merely names `TICK_HZ` through, and nothing pins such a value
    at 60 Hz." (Route, the advisor's F2 ruling, 2026-11-09: the plan names no new tick count for U4e; its counts are
    `FULL_STATE_ACK_TICKS` and `ACK_RING_TICKS`, in npt-sim since U4s, and its deadlines and limits run in ns on the
    injected now. The Input bucket's refill (2 × TICK_HZ a second) and the window's per-tick budget (256 kbps) follow
    carry 6's pattern: the type takes the rate as an argument, production callers pass `npt_sim::TICK_HZ`, a test pins
    each at 30 and 60 Hz, and no const or static outside npt-sim is derived from TICK_HZ. If U4e finds it needs a new
    tick count or a named constant derived from the rate, it writes no local form and records the need in its report;
    the main session adds it by carry 18's route in its own commit once U4d2 has merged, and the site and U4e's merge
    follow it.)
````

### Carry 50
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 180-183 (sha256 cdab02404b30), as they are:_

````text
50. **the event age (advisor-u4-wave1.md:347-349; the advisor's W1-N2 ruling):** "An event older than 255 ticks cannot
    be coded: the event section's age is 8 bits (snapshot.rs's wire text). The room's EventQueue never queues one past
    that age: the overflow resync (§4.7) ends it first, or the room drops it from the resend and counts it. A unit
    test."
````

### Carry 51
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 184-194 (sha256 cdab02404b30), as they are:_

````text
51. **SEC-U4d-R2-1, Low (u4d-round2-security.md; deferred to U4e by Mustafa, 2026-11-11: "The Low about the effect
    row's owner: defer it to U4e in writing. A Low never triggers a push."):** "An effect row's owner has no public
    home in npt-sim: the World finds it as the row's index / the scene's effect rows per soldier (world.rs:1879-1881),
    that count is pub(crate) (scene.rs:119), and the public doc only says 'its owner is its index / 8' (seam.rs:283).
    U4e's own-state filter never copies the 8. It reads the mapping from one public home (a pub accessor such as
    `Scene::effect_rows()`, or a pub rows-per-soldier constant with a const assert on every scene); npt-sim is not U4e's
    lane, and no such home exists today: the main session adds `Scene::effect_rows()` (pub, the rows per soldier, 8 or 0
    in today's scenes; approved as that home by the advisor's U4d close ruling) in its own commit when U4e's build
    starts, with its line in u4-build-plan.md §4.1's npt-sim row ("`Scene::effect_rows` (an effect row's owner, the one
    public home: carry 51)"). Any other home U4e finds it needs is a seam change for the advisor (carry 26's route),
    never a local copy of the 8. A test shows that slot k's client receives exactly slot k's effect rows."
````

### Carry 53
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 195-206 (sha256 cdab02404b30), as they are:_

````text
53. **N2-L1 and the security-auditor's own-state carry (u4d-round2-netcode.md, N2-L1; u4d-round2-security.md,
    'Carries for U4e', first bullet; the advisor's U4d close ruling):** "The codec checks a record's kind against its
    section, never its slot against the client: any slot's Soldier, OwnState or Effect record passes in the own-state
    section. So the room's encoder fills client c's own-state section with c's slot's records only. In every
    `Snapshot` and `FullState` sent to c, the own-state section's Soldier and OwnState records have index = c's slot
    and that soldier's generation, and its Effect rows are exactly the rows whose owner is c's slot (carry 51's
    mapping). A test shows it for every client of an SS room (48 clients) on each of at least 300 ticks, the ticks
    around a respawn (a generation change) included. Catalogue item 5's L1 check reads each own-section record's
    identity (kind, index, generation, an Effect row's owner) against the receiving client's slot, not only the
    section the record sits in. Its owner-to-all bite takes the route the codec leaves open: other slots' own-state
    records in every client's own-state section. Moving them into the entity section ends in `Err(OutOfRange)` at the
    encoder; that proves the codec, not the class filter, so it does not count as that bite."
````

### Carry 54
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 207-211 (sha256 cdab02404b30), as they are:_

````text
54. **The encoder's refusals in the room (u4d-round2-security.md, 'Carries for U4e', second bullet; the advisor's U4d
    close ruling):** "The room handles `Err(OutOfRange)` from the encoder (a section mix-up, or an integer outside its
    field's range) without a panic, `unwrap` or `expect`: that client's snapshot for that tick is skipped, the error
    is counted per client and logged (under a name with its SERIES.md row), and the next tick encodes afresh. A test
    plants one out-of-range record for one client and shows every other client's snapshot for that tick unchanged."
````

### Carry 55
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 212-216 (sha256 cdab02404b30), as they are:_

````text
55. **Team and Public rows (u4d-round2-security.md, 'Carries for U4e', third bullet; the advisor's U4d close
    ruling):** "npt-proto's `carries()` (snapshot.rs:173-179) counts Team and Public rows as the entity section; both
    are empty today. A kind placed in a Team row needs the room's class filter (that team only), since the codec
    cannot tell teams apart. Any Team or Public row added before U4's end comes to the advisor with its filter and a
    test."
````

### Carry 56
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 217-222 (sha256 cdab02404b30), as they are:_

````text
56. **N2-L3 (c), the own-state section's size (u4d-round2-netcode.md; probe X1, u4d-round2-test.md; the advisor's U4d
    close ruling):** "U4e's report states the measured own-state section per snapshot: bytes on the wire (mean, p99
    and max per client), in SS and in ES, with the OwnState record's share of it and how often that record changes.
    Over an S30 of SS, 47.49 of 48 OwnState records changed each tick (X1): the fire timer and the reload left count
    down every tick. R2's estimate allows 16 B per snapshot for the local player's own state (r2-netcode.md:1756). A
    mean above it is reported beside that figure, not judged: PM-0001-12 judges the downstream total."
````

### Carry 57
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 223-228 (sha256 cdab02404b30), as they are:_

````text
57. **U4E-Q6, the room's half (advisor-u4-wave1.md:473-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "The event queue's cap is data, with a provisional value from net F121's arithmetic (1,024 is
    F121's example, above the about 400 a B1 burst adds), recorded as provisional wherever the run's header records
    the room's settings; U4f measures the rate and records §4.7's value before -22's runs. The room counts each
    overflow resync per client, with its tick, in its log, so that every judged -17 and -18 run can report them (carry
    58)."
````

### Carry 61
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 229-247 (sha256 cdab02404b30), as they are:_

````text
61. **U4j's hash rules, the room and the client (the advisor's U4j ruling, "Carries", U4e, in FX-12's words; its
    round-2 ruling, B2.1: NC-U4J-R2-N1 and N6):** "U4e builds the room's and the client's side of marks/README.md
    §1's `hs` rules as U4j merged them (its `server_hash` and `decoded_hash` items, with the round-2 words), and
    exposes per client and tick what U4r's logs write (U4E-Q7): (a) the room's `server_hash` for each connected client
    from its admission on, in every phase the clients log; none for a client not connected or for a tick whose
    snapshot the room did not send that client; the room sends no snapshot of a tick it does not log, so its last
    tick record covers every snapshot it sent; (b) the client's hash filed under the tick it rebuilt, at the room's
    slot from `JoinAccepted`; two different hashes for one tick give the sentinel, one hash met twice (a duplicated
    datagram) is filed once, and no hash is ever dropped; (c) a tick resolved lost (`tick.s.snap_incomplete` 1, no
    hash) only once none of its groups can still decode: once it has left the client's frame ring (at
    `npt_sim::ACK_RING_TICKS` ticks old), or at the stop; a tick none of whose groups arrived counts, known from a
    later header's tick; (d) a group of a tick older than one the client has already decoded (L1's reorder step
    delivers such groups) is decoded and hashed under its own tick like any other, or, if the client drops it as
    stale, it is never acknowledged and that tick is lost, never the sentinel; the sentinel stays for §4.6's causes
    (no baseline, a bad header, a size or decode error). The advisor's lean is the decode: it keeps -58's P3
    comparing the late side of a reordered wrap (ADR 0001:1607). U4e's report says which its client does; a client
    that drops counts each drop per client under a name with its SERIES.md row. A unit test feeds `ClientEndpoint`
    tick t+1's snapshot before tick t's (as L1's reorder step delivers them) and shows t filed under t (its hash, or
    lost if the client drops it; never the sentinel), and that one datagram received twice files one hash."
````

### Carry 68
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 248-253 (sha256 cdab02404b30), as they are:_

````text
68. **NF-L1, ES's bytes and the synthetic recording (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4e
    takes no ES snapshot byte or bandwidth figure from `Recording::synthetic`'s ES recording as a measure of ES's load.
    Since U4d-F that recording's alive soldiers change yaw on about 48 % of soldier-ticks and pitch on about 18 % (its
    sky aims sit nearly still while the soldier moves), against R2's moving-delta assumption of yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751), so its bytes read lighter than R2's estimate.
    Any ES byte figure U4e reports from that recording states those two shares beside R2's."
````

### Carry 40
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 256-264 (sha256 cdab02404b30), as they are:_

````text
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1; its last sentence the main
    session's words for the advisor's W1-U4E ruling, U4E-Q5, approved with the clause 'for the tolerance and the cap' by
    the advisor's U4d close ruling):** "U4r's brief, which comes to the advisor, states that at U4r the room has neither
    the view-time tolerance nor the cap (both land in U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30
    pre-check's setup condition (0 view-time replacements and 0 clipped shots on both legs: the advisor's condition 3)
    holds by construction at U4r for the tolerance and the cap, and its RTT argument cannot be tested there; and it
    either re-runs the pre-check pair after U4h merges (6 runs) or says why not (the advisor's lean is the re-run). The
    room's count of commands that `rewind_at` gives no Rewind, because the ring does not hold their claimed view time's
    tick, counts as a replacement in that setup condition: a pre-check leg with any is VOID (U4E-Q5)."
````

### Carry 62
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 289-310 (sha256 cdab02404b30), as they are:_

````text
62. **U4j's log rules, U4r's runner and logs (the advisor's U4j ruling, "Carries", U4r, with FX-5 (a), FX-10, FX-12
    and R2.4; its round-2 ruling, B2.1: NC-U4J-R2-N1 and N6):** "U4r's logs meet marks/README.md §1's `hs` rules as
    U4j merged them, and the judge's coverage rule (SERIES.md's `decoded_hash_mismatch_count` row): (a) each name
    only in its own process's log: `server_hash` in the room's (process `room`), `decoded_hash` in the clients'
    (`client` on L1, `bots` on L2), `replay_hash` only in a `replay` log on x86_64; (b) every `hs` list and every
    `tick.s.snap_incomplete` list holds exactly the room header's `clients` entries, at the room's slot; (c) the
    writer buffers each client's ticks until every earlier tick is resolved, and writes a tick lost only once none of
    its groups can still decode (once it has left the client's frame ring, or at the stop: on L1 the runner may
    decide it there); a hash is never dropped, else the log ends `crash`; (d) the head: every client is admitted,
    and its first snapshot resolved, in the warm-up, before the room's first measured hash for its slot (a client
    admitted later is -19's join: C-U6-11); (e) the tail: after the measured phase the clients keep logging until
    every measured tick is resolved, a tick still in flight at the stop is not written, and the room's log covers
    every snapshot the room sent (it sends none of a tick it does not log); (f) a run and its twin log `h` over the
    same ticks; (g) the contract test shows `decoded_hash_mismatch_count` resolving with no gap on every -17 suite, a
    short L1 P3 suite with the reorder step on among them (no `crash` footer, 0 mismatches, no gap), and, if U4e's
    client drops stale groups (entry 61 (d)), reports its drop count beside `decoded_hash_compared_count` on -58's
    P3 suite; (h) NC-10 (the advisor's U4j ruling, R2.4): the contract test reports `tick.s.snap_incomplete`'s sum on
    each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0 part, with a bite (a client that writes
    null and incomplete 1 for a refused group), before -17's judged runs: `{"name": "setup: L1 P0 loses no group (no
    client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "all", "op": "==",
    "value": 0, "parts": ["L1 P0"], "setup": true}`. If it is not 0, the cause goes to the advisor first. marks/ is
    not U4r's lane unless its brief gives it this one check; else the main session adds it in its own commit."
````

### Carry 66
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 436-446 (sha256 cdab02404b30), as they are:_

````text
66. **U4c's netcode F2, Medium (u4c-review-netcode.md:53-54; L-c8's judged re-run, u4c-lc8-rerun.md; the advisor's U4e
    brief ruling, F2-ROUTE):** "npt-lab's probe also writes the SHA-256 of (v1's tag + the v2 fate log's datagram
    entries), the hash §5 U4c's L-c8 compares with U2's committed 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4
    seed 1, beside the whole log's SHA-256: in the run log's header and in its summary line, so that -3's script
    (ops/runs/c-u2-pm-0001-3.mjs) records it unchanged. It reads the log through npt-l1's public
    `FateLog::for_each_canonical` and edits no crates/l1 file; v1's tag and the datagram entry's form are those U4c's
    test `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` reads (l1/tests/l1.rs:481-532). U4d2's
    report shows the three values from probe runs of those three runs (the -3 set's own arguments), each equal to
    U2's, with the commands it ran. Until then L-c8's hash half rests on U4c's scratch harness through equal
    whole-log hashes (u4c-lc8-rerun.md). If U4d2's brief is final without this entry, U4r's brief takes it word for
    word, U4r in place of U4d2."
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48, 49, 52, 58, 59, 60, 63, 64, 65, 67, 69, 70, 71, 72.
