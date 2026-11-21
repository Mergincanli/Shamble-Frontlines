# Review packet: U4e round 2 (on what changed: the fix pass FX-1 to FX-13 after the advisor's round-1 ruling)

_Cut by `review_packet.py` from `u4e-r2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Base: round 1's reviewed tree, the read copy `.gate/u4e-r1` (97bb9d3 with round 1's snapshot patch applied: wip/u4e.patch at db49108, kept as .gate/u4e/u4e-r1-snapshot.patch).
- U4e's copy after the fix pass: `.claude/worktrees/wf_0fa6595a-3e2-1` (branch worktree-wf_0fa6595a-3e2-1 at 97bb9d3; uncommitted). The same change as a patch on 97bb9d3: `.checkpoints/netcode-proof-test/wip/u4e.patch` (committed at 4300944). Its 23 files' sha256: `.gate/u4e/fx-hashes-end.sha256`.
- The diffs below compare round 1's tree with the fixed copy, per crate (sources, then tests). The fix pass's runs are under `.gate/u4e/` as fx-*.log (fx-counted1.log and fx-counted2.log: its two counted check.mjs runs; fx-t5print.log: its printed lines).
- Plan citations: the plan u4-build-plan.md changed at 85c08df (the advisor's U4d2 brief check, L2), which moved every line after §4.11. Plan cites in u4e-brief.md, advisor-u4e-r1.md and earlier rulings read at 686a583c: `git show 1e8c188:.checkpoints/netcode-proof-test/u4-build-plan.md`.

## What changed (the diffs)

### npt-room's sources (crates/room/src)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/room/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src`:  3 files changed, 93 insertions(+), 27 deletions(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/ack.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/ack.rs
index b93e400..e10b18b 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/ack.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/ack.rs
@@ -45,6 +45,13 @@ impl Sent {
     }
 }
 
+// The group masks (`Sent::acked`, an acknowledgement's, `full_mask`) are a u8: they rely on at most 8 groups a tick
+// (RR-3; the wire's 3-bit group).
+const _: () = assert!(
+    npt_proto::Limits::GROUPS <= 8,
+    "a u8 group mask holds at most 8 groups"
+);
+
 /// What an acknowledgement named (the room's check "the acknowledgement names a sent snapshot": catalogue :87).
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
 pub(crate) enum Acked {
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/room.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/room.rs
index 3075ceb..83118b2 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/room.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/room.rs
@@ -105,6 +105,9 @@ pub struct ClientCounters {
     pub resyncs: u64,
     /// A second `Resync` before the full state's acknowledgement: dropped (catalogue :90).
     pub resyncs_dropped: u64,
+    /// A `Resync` whose answer would come sooner than `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state
+    /// sent the client: dropped (SEC-U4E-4: an acknowledgement, the client's own claim, never gates one by itself).
+    pub resyncs_limited: u64,
     /// `Kick(overflow)`s sent: the client was dropped (§4.7).
     pub kicked: u64,
     /// Commands accepted into the input buffer.
@@ -167,11 +170,14 @@ pub struct ClientSlot {
     full_state_due: Option<bool>,
     /// The full state sent and not yet acknowledged: its tick, and whether an overflow forced it.
     in_flight: Option<(u32, bool)>,
+    /// The tick of the last full state sent (the join's, a forced one or an answered one): the `Resync` limit's.
+    last_full_state: Option<u32>,
     kicked: bool,
     server_bytes: u32,
     max_datagram: usize,
     datagrams: Vec<Vec<u8>>,
-    control: Vec<Message>,
+    /// The reliable messages, each encoded once with the room's `Quant` (plan.md:508-509).
+    control: Vec<Vec<u8>>,
     counters: ClientCounters,
     last_ack_hi: Option<u16>,
     pending_wraps: Vec<&'static str>,
@@ -260,7 +266,11 @@ impl ClientSlot {
     }
 
     /// The changed entities the last tick's snapshot held at the baseline's value.
-    pub fn skipped(&self) -> u32 {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn skipped(&self) -> u32 {
         self.t.skipped
     }
 
@@ -306,11 +316,20 @@ impl ClientSlot {
     }
 
     fn kick(&mut self, roster: &mut Roster) {
-        self.control.push(Message::Kick(Kick::Overflow));
+        self.send_control(&Message::Kick(Kick::Overflow));
         self.kicked = true;
         self.counters.kicked += 1;
         roster.leave(self.slot);
     }
+
+    /// Queues `msg` for the control outbox, encoded here, once, with the room's `Quant` (plan.md:508-509: "Encoding
+    /// stays on the tick"; the advisor-u4e-r1 R2, A1).
+    fn send_control(&mut self, msg: &Message) {
+        let mut out = Vec::new();
+        if self.encoder.quant.encode(msg, &mut out).is_ok() {
+            self.control.push(out);
+        }
+    }
 }
 
 /// `r` as the wire carries it: `area` and `raw` 0.
@@ -391,6 +410,10 @@ pub struct Room<S: SimCore> {
     #[cfg(feature = "bites")]
     bite: Option<RoomBite>,
     input: TickInput,
+    /// The next tick's scripted spawns, teleports and attribute changes, until `fill_input` moves them into `input`.
+    scripted_spawns: Vec<Placement>,
+    scripted_teleports: Vec<Placement>,
+    scripted_attributes: Vec<AttributeChange>,
     stamped: Vec<(u32, u32, SimEvent)>,
     reports: InputReports,
     index: Index,
@@ -443,6 +466,9 @@ impl<S: SimCore> Room<S> {
             #[cfg(feature = "bites")]
             bite: None,
             input: TickInput::default(),
+            scripted_spawns: Vec::new(),
+            scripted_teleports: Vec::new(),
+            scripted_attributes: Vec::new(),
             stamped: Vec::new(),
             reports: InputReports::default(),
             index: Index::default(),
@@ -548,7 +574,8 @@ impl<S: SimCore> Room<S> {
         self.next_tick.wrapping_sub(1)
     }
 
-    /// The last tick's `TickInput` (what a recording holds: the commands in the canonical order with their Rewinds).
+    /// The last tick's `TickInput`, whole (what a recording holds: the commands in the canonical order with their
+    /// Rewinds, and the tick's scripted spawns, teleports and attribute changes).
     pub fn tick_input(&self) -> &TickInput {
         &self.input
     }
@@ -573,9 +600,11 @@ impl<S: SimCore> Room<S> {
             .unwrap_or_default()
     }
 
-    /// The reliable messages for `slot` (`JoinAccepted`, `FullState`, `Kick`), each for the channel its row in
-    /// `Limits::TYPES` names.
-    pub fn take_control(&mut self, slot: u8) -> Vec<Message> {
+    /// The reliable messages for `slot` (`JoinAccepted`, `FullState`, `Kick`), each encoded once, when the room queued
+    /// it, with the room's `Quant` (plan.md:508-509: "Encoding stays on the tick"). The caller sends each on the
+    /// channel its type byte's row of `Limits::TYPES` names (npt-proto's rule; the type byte follows the stream's
+    /// length prefix).
+    pub fn take_control(&mut self, slot: u8) -> Vec<Vec<u8>> {
         self.clients
             .get_mut(usize::from(slot))
             .and_then(Option::as_mut)
@@ -583,19 +612,20 @@ impl<S: SimCore> Room<S> {
             .unwrap_or_default()
     }
 
-    /// The tick's scripted spawns, teleports and attribute changes (integers only: seam.rs), for the next step.
+    /// The next tick's scripted spawns, teleports and attribute changes (integers only: seam.rs): they wait here and
+    /// go into that tick's `TickInput` (so [`Room::tick_input`] after the tick holds them).
     pub fn script(
         &mut self,
         spawns: &[Placement],
         teleports: &[Placement],
         attributes: &[AttributeChange],
     ) {
-        self.input.spawns.clear();
-        self.input.spawns.extend_from_slice(spawns);
-        self.input.teleports.clear();
-        self.input.teleports.extend_from_slice(teleports);
-        self.input.attributes.clear();
-        self.input.attributes.extend_from_slice(attributes);
+        self.scripted_spawns.clear();
+        self.scripted_spawns.extend_from_slice(spawns);
+        self.scripted_teleports.clear();
+        self.scripted_teleports.extend_from_slice(teleports);
+        self.scripted_attributes.clear();
+        self.scripted_attributes.extend_from_slice(attributes);
     }
 
     /// A `Join` the pump passed (its version and content hash already checked there): the roster's checks, then a
@@ -625,6 +655,7 @@ impl<S: SimCore> Room<S> {
             priority: Priority::default(),
             full_state_due: Some(false),
             in_flight: None,
+            last_full_state: None,
             kicked: false,
             server_bytes: 0,
             max_datagram: Limits::SNAPSHOT,
@@ -640,7 +671,7 @@ impl<S: SimCore> Room<S> {
             nearest: Vec::new(),
             cosmetic: Vec::new(),
         };
-        c.control.push(Message::JoinAccepted(JoinAccepted {
+        c.send_control(&Message::JoinAccepted(JoinAccepted {
             slot: a.slot,
             team: c.team,
             squad: 0,
@@ -683,6 +714,7 @@ impl<S: SimCore> Room<S> {
         match msg {
             Message::Input(input) => self.receive_input(slot, input),
             Message::Resync(_) => {
+                let next = self.next_tick;
                 if let Some(c) = self
                     .clients
                     .get_mut(usize::from(slot))
@@ -691,6 +723,13 @@ impl<S: SimCore> Room<S> {
                 {
                     if c.in_flight.is_some() || c.full_state_due.is_some() {
                         c.counters.resyncs_dropped += 1;
+                    } else if c
+                        .last_full_state
+                        .is_some_and(|f| next.wrapping_sub(f) < npt_sim::FULL_STATE_ACK_TICKS)
+                    {
+                        // SEC-U4E-4's room half (advisor-u4e-r1 R3): the answer, at the next tick, would come sooner
+                        // than FULL_STATE_ACK_TICKS ticks after the last full state: at most one in that span.
+                        c.counters.resyncs_limited += 1;
                     } else {
                         c.full_state_due = Some(false);
                         c.counters.resyncs += 1;
@@ -777,9 +816,6 @@ impl<S: SimCore> Room<S> {
             self.stamped.push((self.next_event, t, *e));
             self.next_event = self.next_event.wrapping_add(1);
         }
-        self.input.spawns.clear();
-        self.input.teleports.clear();
-        self.input.attributes.clear();
         self.history.push(self.sim.hit_volumes());
         self.flood.update(&self.sim.portal_states().open);
         self.reports.clone_from(self.sim.reports());
@@ -790,10 +826,16 @@ impl<S: SimCore> Room<S> {
         self.next_tick = t.wrapping_add(1);
     }
 
-    /// The tick's `TickInput`: per joined client the command option B runs (its slot the session's), with its Rewind
-    /// when it holds or presses fire, in the canonical order.
+    /// The tick's `TickInput`: the script waiting for it, and per joined client the command option B runs (its slot
+    /// the session's), with its Rewind when it holds or presses fire, in the canonical order.
     fn fill_input(&mut self, t: u32) {
         self.input.tick = t;
+        self.input.spawns.clear();
+        self.input.spawns.append(&mut self.scripted_spawns);
+        self.input.teleports.clear();
+        self.input.teleports.append(&mut self.scripted_teleports);
+        self.input.attributes.clear();
+        self.input.attributes.append(&mut self.scripted_attributes);
         self.input.commands.clear();
         let fire = Command::FIRE;
         for c in self.clients.iter_mut().flatten() {
@@ -1072,18 +1114,20 @@ fn full_state(
         entities: frame.entities.clone(),
     };
     let msg = Message::FullState(fs);
-    let mut probe = Vec::new();
-    if let Err(e) = c.encoder.quant.encode(&msg, &mut probe) {
+    // Encoded once, here, with the room's `Quant`: the bytes the client gets (A1), and a refusal counted (carry 54).
+    let mut bytes = Vec::new();
+    if let Err(e) = c.encoder.quant.encode(&msg, &mut bytes) {
         count_refusal(&mut c.counters, e);
         return;
     }
     c.full_state_due = None;
     c.in_flight = Some((t, forced));
+    c.last_full_state = Some(t);
     c.queue.clear();
     c.counters.full_states += 1;
     c.t.server_hash = Some(client_hash(t, &frame.entities, &frame.own, &[]));
     c.ack.record(t, 1, frame, Vec::new());
-    c.control.push(msg);
+    c.control.push(bytes);
 }
 
 fn count_refusal(counters: &mut ClientCounters, e: EncodeError) {
@@ -1164,11 +1208,22 @@ fn snapshot(
     // Gameplay events, oldest first, resent until acknowledged.
     let (mut sent, mut resent) = (0, 0);
     for q in c.queue.iter_mut() {
+        // Carry 50 (RR-2): `route_events` dropped every event older than 255 ticks this tick, so the age fits the
+        // wire's 8 bits; one that does not is never coded.
+        let event_age = u8::try_from(t.wrapping_sub(q.tick));
+        debug_assert!(
+            event_age.is_ok(),
+            "an event older than 255 ticks reached the encoder (carry 50)"
+        );
+        let Ok(event_age) = event_age else {
+            continue;
+        };
         let Some(g) = p.place(event_bits(&q.event), Section::Events) else {
             break;
         };
-        let age = t.wrapping_sub(q.tick) as u8;
-        p.groups[g].events.push((q.number as u16, age, q.event));
+        p.groups[g]
+            .events
+            .push((q.number as u16, event_age, q.event));
         p.groups[g].numbers.push(q.number);
         p.groups[g].gameplay.push(q.number);
         q.sent += 1;
@@ -1377,6 +1432,7 @@ mod tests {
         let rec = npt_sim::replay::Recording::synthetic(&scene, 7, 900, 100, 2_000);
         let mut next = start;
         let mut stamps: Vec<(u32, u32)> = Vec::new();
+        let mut checked = 0u32;
         for _ in 0..60 {
             let t = room.last_tick().wrapping_add(1);
             for input in rec.ticks.iter().filter(|i| i.tick == t) {
@@ -1402,6 +1458,7 @@ mod tests {
             if let Some(c) = room.clients[0].as_mut() {
                 for q in c.queue.iter_mut() {
                     assert!(stamps.contains(&(q.number, q.tick)), "{q:?}");
+                    checked += 1;
                 }
             }
         }
@@ -1409,5 +1466,6 @@ mod tests {
             next > u32::from(u16::MAX) + 1,
             "the numbers crossed the 16-bit wrap: {next}"
         );
+        assert!(checked > 0, "the queue half checked a queued event (RR-10)");
     }
 }
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/roster.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/roster.rs
index fbcd89a..197c1d9 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/room/src/roster.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/src/roster.rs
@@ -56,7 +56,7 @@ pub(crate) struct Admitted {
 impl Roster {
     /// A roster of one slot per token, slot `i` holding `tokens[i]`, admitting at most `n` sessions at once. At most 256
     /// slots are kept (a slot is a `u8`).
-    pub fn new(mut tokens: Vec<[u8; 16]>, n: usize) -> Roster {
+    pub(crate) fn new(mut tokens: Vec<[u8; 16]>, n: usize) -> Roster {
         tokens.truncate(usize::from(u8::MAX) + 1);
         Roster {
             slots: tokens
@@ -82,7 +82,8 @@ impl Roster {
     }
 
     /// Whether `slot`'s token is bound to a client's bind value (at its first accepted `Join`).
-    pub fn is_bound(&self, slot: u8) -> bool {
+    #[cfg_attr(not(test), allow(dead_code))]
+    pub(crate) fn is_bound(&self, slot: u8) -> bool {
         self.slots
             .get(usize::from(slot))
             .is_some_and(|s| s.bind.is_some())
````

### npt-server's library (crates/server/src)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/server/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src`:  2 files changed, 63 insertions(+), 19 deletions(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/server/src/lib.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/lib.rs
index 1e0d231..eef3695 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/server/src/lib.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/lib.rs
@@ -11,7 +11,7 @@
 mod pump;
 mod schedule;
 
-pub use pump::{ConnLimits, Counters, Decoder};
+pub use pump::{ConnLimits, Counters, Decoder, RttSource};
 #[cfg(feature = "bites")]
 pub use schedule::BITES;
 pub use schedule::{Dropped, Due, LoopCounters, Schedule, due_ticks};
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/server/src/pump.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/pump.rs
index 9cb3e5a..f558d7a 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/server/src/pump.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/src/pump.rs
@@ -98,7 +98,11 @@ impl ConnLimits {
     }
 
     /// When a connection with no `Join` is closed, ns.
-    pub fn join_deadline_ns(&self) -> u64 {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn join_deadline_ns(&self) -> u64 {
         self.deadline_ns
     }
 }
@@ -134,9 +138,12 @@ pub struct Counters {
     pub ping_limited: u64,
     /// `Pong`s naming no outstanding ping of this session: dropped.
     pub pong_unknown: u64,
-    /// Sessions closed as malformed because their first stream message was not a `Join` (never INVALID: the run
-    /// verdict is the judge's).
+    /// Sessions closed as malformed because their first stream message was not a `Join` (a decodable other type, or
+    /// a known type on the wrong channel or from the wrong direction: catalogue :93), never INVALID (the run verdict
+    /// is the judge's).
     pub first_not_join: u64,
+    /// Stream messages received while the session's `Join` waited for the room's answer: dropped (SEC-U4E-3).
+    pub joining_dropped: u64,
     /// Sessions closed as malformed for a stream message that failed its framing, size or type checks.
     pub stream_malformed: u64,
     /// Sessions closed for no `Join` within 2 s of the accept.
@@ -170,6 +177,18 @@ impl Counters {
     }
 }
 
+/// Where a session's RTT comes from (u4-build-plan.md §4.9 :352-354): by the session's kind, which the pump's caller
+/// learns from the listener that accepted it.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub enum RttSource {
+    /// A `wt` or L1 session: the link's smoothed estimate (`Link::rtt_ns`), `None` until it has one; never the room's
+    /// ping samples.
+    Link,
+    /// A `ws` session: the lowest of the last 8 samples of the room's own pings, each less the client's hold time up
+    /// to one tick period; never the link's estimate.
+    Pings,
+}
+
 /// Where a session is in its life.
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
 enum State {
@@ -188,6 +207,7 @@ pub struct Decoder {
     counters: Counters,
     version: u16,
     content_hash: [u8; 32],
+    rtt: RttSource,
     state: State,
     next_ping_ns: u64,
     ping_id: u8,
@@ -199,13 +219,15 @@ pub struct Decoder {
 impl Decoder {
     /// The decoder of a session accepted at `accept_ns`, decoding with `quant` (the wire's, `Quant::default()`; in a
     /// bites build one of npt-proto's planted defects through `Quant::with_bite`), at `tick_hz` ticks a second
-    /// (production callers pass `npt_sim::TICK_HZ`), taking `Join`s of protocol `version` and `content_hash` only.
+    /// (production callers pass `npt_sim::TICK_HZ`), taking `Join`s of protocol `version` and `content_hash` only, its
+    /// RTT from `rtt` (the session's kind).
     pub fn new(
         quant: Quant,
         tick_hz: u32,
         accept_ns: u64,
         version: u16,
         content_hash: [u8; 32],
+        rtt: RttSource,
     ) -> Decoder {
         Decoder {
             quant,
@@ -213,6 +235,7 @@ impl Decoder {
             counters: Counters::default(),
             version,
             content_hash,
+            rtt,
             state: State::AwaitJoin,
             next_ping_ns: accept_ns,
             ping_id: 0,
@@ -228,7 +251,11 @@ impl Decoder {
     }
 
     /// The session's limits.
-    pub fn limits(&self) -> &ConnLimits {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn limits(&self) -> &ConnLimits {
         &self.limits
     }
 
@@ -238,7 +265,11 @@ impl Decoder {
     }
 
     /// Whether the room accepted the session's `Join`.
-    pub fn is_joined(&self) -> bool {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn is_joined(&self) -> bool {
         self.state == State::Joined
     }
 
@@ -261,11 +292,14 @@ impl Decoder {
         core::mem::take(&mut self.sent_bytes)
     }
 
-    /// The session's RTT (u4-build-plan.md §4.9 :352-353): the link's smoothed estimate for an L1 or `wt` session
-    /// (Loopback's is 0), else the lowest recent sample of the room's own pings, each less the client's hold time up
-    /// to one tick period.
+    /// The session's RTT (u4-build-plan.md §4.9 :352-354), from its [`RttSource`]: for an L1 or `wt` session the link's
+    /// smoothed estimate (Loopback's is 0), for a `ws` session the lowest recent sample of the room's own pings, each
+    /// less the client's hold time up to one tick period. One session never mixes the two.
     pub fn rtt_ns(&self, link: &dyn Link) -> Option<u64> {
-        link.rtt_ns().or_else(|| self.samples.iter().copied().min())
+        match self.rtt {
+            RttSource::Link => link.rtt_ns(),
+            RttSource::Pings => self.samples.iter().copied().min(),
+        }
     }
 
     fn send_control(&mut self, link: &mut dyn Link, msg: &Message, now_ns: u64) {
@@ -336,8 +370,8 @@ impl Decoder {
                 Ok(m) => m,
                 Err(e) => {
                     self.counters.refused(e);
-                    // A forged kind of a known type within its limits is refused and counted, never kicked
-                    // (u4-build-plan.md §4.10); any other bad stream message closes the session.
+                    // A forged kind of a known type within its limits is refused and counted, never kicked, on a
+                    // joined session (u4-build-plan.md §4.10); any other bad stream message closes the session.
                     if stream
                         && !matches!(e, DecodeError::WrongChannel | DecodeError::WrongDirection)
                     {
@@ -345,6 +379,13 @@ impl Decoder {
                         self.close_malformed(link, now_ns);
                         return;
                     }
+                    // Before the `Join`, a forged kind first on the stream is "any other type first on the stream"
+                    // (catalogue :93; advisor-u4e-r1 R5, SEC-U4E-1): the session closes as malformed.
+                    if stream && self.state == State::AwaitJoin {
+                        self.counters.first_not_join += 1;
+                        self.close_malformed(link, now_ns);
+                        return;
+                    }
                     continue;
                 }
             };
@@ -386,9 +427,11 @@ impl Decoder {
                         self.counters.ping_limited += 1;
                     }
                 }
-                (State::Joined, Message::Pong(p)) => self.pong(&p, now_ns),
+                (State::Joined, Message::Pong(p)) => self.pong(&p, r.arrival_ns),
                 (State::Joined, m) => out.push((m, r.arrival_ns)),
-                _ => {}
+                // Only a stream message other than `Join` in the Joining state is left (a closed session returns
+                // before the loop, and every close returns): dropped and counted (SEC-U4E-3).
+                _ => self.counters.joining_dropped += 1,
             }
         }
     }
@@ -408,9 +451,10 @@ impl Decoder {
         false
     }
 
-    /// A `Pong` of one of the room's outstanding pings: one RTT sample, the hold time subtracted up to one tick
-    /// period (catalogue :92); any other id is dropped and counted.
-    fn pong(&mut self, p: &Pong, now_ns: u64) {
+    /// A `Pong` of one of the room's outstanding pings, arrived at `arrival_ns`: one RTT sample, from its arrival (not
+    /// the pump's later run: NC-M3), the hold time subtracted up to one tick period (catalogue :92); any other id is
+    /// dropped and counted.
+    fn pong(&mut self, p: &Pong, arrival_ns: u64) {
         let Some(at) = self
             .outstanding
             .iter()
@@ -421,7 +465,7 @@ impl Decoder {
         };
         self.outstanding.remove(at);
         let hold = (u64::from(p.hold_us) * 1_000).min(self.limits.hold_cap_ns());
-        let sample = now_ns.saturating_sub(p.echo_ns).saturating_sub(hold);
+        let sample = arrival_ns.saturating_sub(p.echo_ns).saturating_sub(hold);
         self.samples.push_back(sample);
         if self.samples.len() > SAMPLES {
             self.samples.pop_front();
````

### npt-client's sources (crates/client/src)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/client/src .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src`:  1 file changed, 71 insertions(+), 11 deletions(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/client/src/lib.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src/lib.rs
index cc67c02..882f93d 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/client/src/lib.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/src/lib.rs
@@ -164,6 +164,13 @@ impl Tick {
     }
 }
 
+// The group masks (`Tick::got`, the acknowledgement's) are a u8 and `Tick::sigs` holds one entry per group: they rely
+// on at most 8 groups a tick (RR-3; the wire's 3-bit group).
+const _: () = assert!(
+    Limits::GROUPS <= 8,
+    "a u8 group mask holds at most 8 groups"
+);
+
 /// FNV-1a 64 of a datagram's bytes: a duplicated datagram is the same bytes (carry 61 (b): filed once).
 fn signature(bytes: &[u8]) -> u64 {
     let mut h: u64 = 0xcbf2_9ce4_8422_2325;
@@ -210,6 +217,11 @@ pub struct ClientEndpoint {
     wrap: Wrap,
     ring: Vec<Tick>,
     newest: Option<u32>,
+    /// The tick of the first header taken (its first group's, full state's or refused datagram's): a header before it
+    /// and at least the ring behind the newest names a tick the client was never sent (advisor-u4e-r1 R1, case 3).
+    first: Option<u32>,
+    /// The bad headers' ticks already given the sentinel (case 3), at most `BAD_HEADERS_HELD`, oldest first.
+    bad_headers: VecDeque<u32>,
     low: u32,
     ack: Option<(u32, u8)>,
     next_cmd: u32,
@@ -232,6 +244,8 @@ pub struct ClientEndpoint {
 
 /// Pings the endpoint holds to answer at its next step, at most (the room sends a few a second).
 const PINGS_HELD: usize = 8;
+/// Bad headers' ticks the endpoint remembers, so each gets the sentinel once (the groups of one tick arrive together).
+const BAD_HEADERS_HELD: usize = npt_sim::ACK_RING_TICKS as usize;
 
 impl ClientEndpoint {
     /// An endpoint that joins with `join` (its first step sends it), stepping at `tick_hz` (production callers pass
@@ -249,6 +263,8 @@ impl ClientEndpoint {
             wrap: Wrap::default(),
             ring: vec![Tick::default(); npt_sim::ACK_RING_TICKS as usize],
             newest: None,
+            first: None,
+            bad_headers: VecDeque::new(),
             low: 0,
             ack: None,
             next_cmd: 0,
@@ -308,7 +324,11 @@ impl ClientEndpoint {
     }
 
     /// Whether the room ended the session (`JoinRejected`, `Kick` or `RoomEnded`).
-    pub fn closed(&self) -> bool {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn closed(&self) -> bool {
         self.closed
     }
 
@@ -328,12 +348,14 @@ impl ClientEndpoint {
         self.wrong_channel
     }
 
-    /// Received messages refused by the decoder or by the baseline rule (a snapshot's tick then gets the sentinel).
+    /// Received messages refused by the decoder, by the baseline rule, or as a bad header (a tick before the first
+    /// header taken, at least the ring behind the newest); a snapshot's tick then gets the sentinel.
     pub fn refused(&self) -> u64 {
         self.refused
     }
 
-    /// Snapshot groups too old for the frame ring, dropped.
+    /// Snapshot groups and full states of a tick at least the ring behind the newest, from the first header taken on
+    /// (a tick resolved when it left the ring), dropped.
     pub fn stale(&self) -> u64 {
         self.stale
     }
@@ -344,7 +366,11 @@ impl ClientEndpoint {
     }
 
     /// `Pong`s whose ping id is not one of the client's own (it sends none at U4e), dropped.
-    pub fn unknown_pongs(&self) -> u64 {
+    #[allow(
+        dead_code,
+        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
+    )]
+    pub(crate) fn unknown_pongs(&self) -> u64 {
         self.unknown_pongs
     }
 
@@ -470,6 +496,39 @@ impl ClientEndpoint {
         })
     }
 
+    /// Whether a header naming tick `t` is too old for the ring, so nothing is decoded from it (advisor-u4e-r1 R1). A
+    /// tick from the first header taken on was resolved when it left the ring: dropped and counted `stale` (case 2).
+    /// A tick before the first header taken was never sent to the client: a bad header (§4.6), the sentinel under the
+    /// tick it names (case 3), counted in `refused` when `count` (a decoder refusal is counted already).
+    fn old_header(&mut self, t: u32, count: bool) -> bool {
+        if !self.too_old(t) {
+            return false;
+        }
+        if self
+            .first
+            .is_some_and(|f| Wrap::default().after32("tick", t, f))
+        {
+            self.refused += u64::from(count);
+            self.bad_header(t);
+        } else {
+            self.stale += 1;
+        }
+        true
+    }
+
+    /// The sentinel for a bad header's tick `t` (case 3), once while `BAD_HEADERS_HELD` such ticks are remembered: no
+    /// frame and no ring slot (slot `t % 32` may hold a live tick), never acknowledged, the newest tick unmoved.
+    fn bad_header(&mut self, t: u32) {
+        if self.bad_headers.contains(&t) {
+            return;
+        }
+        if self.bad_headers.len() >= BAD_HEADERS_HELD {
+            self.bad_headers.pop_front();
+        }
+        self.bad_headers.push_back(t);
+        self.out.hashes.push((t, 0));
+    }
+
     /// The ring slot for `t`, reset for it if it held another tick.
     fn slot_for(&mut self, t: u32) -> usize {
         let i = (t % npt_sim::ACK_RING_TICKS) as usize;
@@ -491,6 +550,7 @@ impl ClientEndpoint {
         let w = Wrap::default();
         let Some(n) = self.newest else {
             self.newest = Some(t);
+            self.first = Some(t);
             self.low = t;
             return;
         };
@@ -519,11 +579,14 @@ impl ClientEndpoint {
         }
     }
 
+    /// A run of `count` ticks resolved lost from `first`: merged into the last run when it continues it and the sum
+    /// fits a u32, else a new run (SEC-U4E-7: never an overflow, whatever the header ticks).
     fn lost(&mut self, first: u32, count: u32) {
         if let Some(last) = self.out.lost.last_mut()
             && last.0.wrapping_add(last.1) == first
+            && let Some(sum) = last.1.checked_add(count)
         {
-            last.1 += count;
+            last.1 = sum;
             return;
         }
         self.out.lost.push((first, count));
@@ -531,8 +594,7 @@ impl ClientEndpoint {
 
     /// The sentinel for tick `t` (a received group it could not decode: §4.6), once.
     fn sentinel(&mut self, t: u32) {
-        if self.too_old(t) {
-            self.stale += 1;
+        if self.old_header(t, false) {
             return;
         }
         self.see(t);
@@ -619,8 +681,7 @@ impl ClientEndpoint {
     /// tick complete once every group has; a group it cannot decode gives the tick the sentinel.
     fn group(&mut self, s: Snapshot, bytes: &[u8]) {
         let t = s.tick;
-        if self.too_old(t) {
-            self.stale += 1;
+        if self.old_header(t, true) {
             return;
         }
         self.see(t);
@@ -713,8 +774,7 @@ impl ClientEndpoint {
     /// A full state: the client's whole view at its tick, a complete frame, acknowledged as group 0 of that tick.
     fn full_state(&mut self, f: FullState) {
         let t = f.tick;
-        if self.too_old(t) {
-            self.stale += 1;
+        if self.old_header(t, true) {
             return;
         }
         self.see(t);
````

### npt-room's tests (crates/room/tests)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/room/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests`:  1 file changed, 316 insertions(+), 17 deletions(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/room/tests/room.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/room.rs
index da62ecb..2161be2 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/room/tests/room.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/room.rs
@@ -11,7 +11,7 @@ use npt_room::{Room, rewind_at};
 use npt_sim::replay::Recording;
 use npt_sim::{
     Command, CoverVariant, EntityKind, EntityRecord, HitHistory, HitVolumes, InputReports,
-    PortalStates, Scene, SimCore, SimEvent, TickInput, World,
+    Placement, PortalStates, Scene, SimCore, SimEvent, TickInput, World,
 };
 
 fn token(slot: u8) -> [u8; 16] {
@@ -90,6 +90,17 @@ fn snapshot_of(bytes: &[u8]) -> Snapshot {
     }
 }
 
+/// A reliable message the room encoded, decoded on the stream its type travels on (the control stream or the
+/// full-state stream, each read from a row of `Limits::TYPES`: npt-room's tests name no npt-link type); `decode`
+/// refuses it on the other (its length prefix and its type byte's row disagree).
+fn control_of(bytes: &[u8]) -> Message {
+    ["JoinAccepted", "FullState"]
+        .iter()
+        .filter_map(|name| Limits::TYPES.iter().find(|r| r.0 == *name))
+        .find_map(|r| npt_proto::decode(Direction::RoomToClient, r.1, bytes).ok())
+        .unwrap_or_else(|| panic!("a control message no stream decodes: {bytes:?}"))
+}
+
 fn collect<S: SimCore>(room: &mut Room<S>, slot: u8) -> Out {
     let mut out = Out {
         tick: room.last_tick(),
@@ -99,7 +110,8 @@ fn collect<S: SimCore>(room: &mut Room<S>, slot: u8) -> Out {
         out.snapshots.push(snapshot_of(&d));
         out.bytes.push(d);
     }
-    for m in room.take_control(slot) {
+    for b in room.take_control(slot) {
+        let m = control_of(&b);
         if let Message::FullState(f) = &m {
             out.full = Some(f.clone());
         }
@@ -344,13 +356,15 @@ fn carry_22_the_room_counts_each_firing_command_its_ring_cannot_rewind() {
     assert_eq!(held_again, 1);
 }
 
-/// e9: a second `Resync` before the full state's acknowledgement is dropped and counted (catalogue :90).
+/// e9: a second `Resync` before the full state's acknowledgement is dropped and counted (catalogue :90); once it is
+/// acknowledged, a new one is limited until `npt_sim::FULL_STATE_ACK_TICKS` ticks after that full state, then taken
+/// (SEC-U4E-4). The first `Resync`s come that many ticks after the join's full state.
 #[test]
 fn e9_a_second_resync_before_the_full_states_ack_is_dropped_and_counted() {
     let scene = Scene::course();
     let mut room = world_room(&scene, 1, 100, 1);
     let mut ack = Ack::default();
-    for _ in 0..3 {
+    for _ in 0..npt_sim::FULL_STATE_ACK_TICKS {
         room.receive(0, &Message::Input(ack.input(Vec::new())));
         room.tick();
         ack.take(&collect(&mut room, 0));
@@ -372,13 +386,248 @@ fn e9_a_second_resync_before_the_full_states_ack_is_dropped_and_counted() {
     ack.take(&out);
     room.receive(0, &Message::Input(ack.input(Vec::new())));
     room.receive(0, &Message::Resync(Resync));
+    let counts = |room: &Room<World>| {
+        let k = c(room);
+        (k.resyncs, k.resyncs_dropped, k.resyncs_limited)
+    };
+    assert_eq!(
+        counts(&room),
+        (1, 2, 1),
+        "acknowledged, but sooner than the window after that full state: limited"
+    );
+    let f = out.tick;
+    while room.last_tick().wrapping_add(1).wrapping_sub(f) < npt_sim::FULL_STATE_ACK_TICKS {
+        room.tick();
+        ack.take(&collect(&mut room, 0));
+        room.receive(0, &Message::Input(ack.input(Vec::new())));
+    }
+    room.receive(0, &Message::Resync(Resync));
     assert_eq!(
-        (c(&room).resyncs, c(&room).resyncs_dropped),
-        (2, 2),
-        "acknowledged: a new one is taken"
+        counts(&room),
+        (2, 2, 1),
+        "acknowledged and the window past: a new one is taken"
     );
 }
 
+/// SEC-U4E-4's room half (advisor-u4e-r1 R3; the test seat's Q-6): a client that acknowledges each full state and asks
+/// again gets no `Resync` answered sooner than `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state it was
+/// sent (here its join's): each such `Resync` is dropped and counted `resyncs_limited`; the first whose answer falls
+/// that many ticks after it is answered.
+#[test]
+fn sec_u4e_4_a_resync_sooner_than_full_state_ack_ticks_after_the_last_full_state_is_limited() {
+    let scene = Scene::stress();
+    let mut room = world_room(&scene, 1, 100, 1);
+    let mut ack = Ack::default();
+    room.tick();
+    let out = collect(&mut room, 0);
+    assert!(out.full.is_some(), "the join's full state");
+    let joined_at = out.tick;
+    ack.take(&out);
+    room.receive(0, &Message::Input(ack.input(Vec::new())));
+    let counters = |room: &Room<World>| {
+        room.client(0)
+            .map(|c| c.counters().clone())
+            .unwrap_or_default()
+    };
+    let window = npt_sim::FULL_STATE_ACK_TICKS;
+    let (mut asked, mut answered) = (0, 0);
+    while room.last_tick().wrapping_add(1).wrapping_sub(joined_at) < window {
+        room.receive(0, &Message::Resync(Resync));
+        asked += 1;
+        room.tick();
+        let out = collect(&mut room, 0);
+        answered += u64::from(out.full.is_some());
+        ack.take(&out);
+        room.receive(0, &Message::Input(ack.input(Vec::new())));
+    }
+    let k = counters(&room);
+    println!(
+        "SEC-U4E-4: {asked} Resyncs within {window} ticks of the join's full state, {answered} answered; resyncs {}, \
+         limited {}, dropped {}, full states {}",
+        k.resyncs, k.resyncs_limited, k.resyncs_dropped, k.full_states
+    );
+    assert!(asked >= 30, "{asked}");
+    assert_eq!(
+        (
+            answered,
+            k.resyncs,
+            k.resyncs_limited,
+            k.resyncs_dropped,
+            k.full_states
+        ),
+        (0, 0, asked, 0, 1)
+    );
+    room.receive(0, &Message::Resync(Resync));
+    room.tick();
+    let out = collect(&mut room, 0);
+    assert_eq!(out.tick.wrapping_sub(joined_at), window);
+    assert!(out.full.is_some(), "answered at the window's end");
+    let k = counters(&room);
+    assert_eq!((k.resyncs, k.resyncs_limited, k.full_states), (1, asked, 2));
+}
+
+/// A4 (advisor-u4e-r1 R2): `tick_input()` after `tick()` is that tick's whole input ("what a recording holds"): a
+/// teleport scripted for the next tick is in it, and the tick after, with nothing scripted, holds none.
+#[test]
+fn a4_tick_input_after_a_tick_is_that_ticks_whole_input() {
+    let scene = Scene::course();
+    let mut room = world_room(&scene, 1, 100, 1);
+    room.tick();
+    let teleport = Placement {
+        slot: 0,
+        position: [100, 200, 0],
+    };
+    room.script(&[], &[teleport], &[]);
+    room.tick();
+    assert_eq!(room.tick_input().tick, room.last_tick());
+    assert_eq!(room.tick_input().teleports, [teleport]);
+    room.tick();
+    assert!(room.tick_input().teleports.is_empty(), "nothing scripted");
+}
+
+/// A `SimCore` over `World` that keeps the cover of every Rewind it is handed (A5's `with_cover` test).
+struct Covers {
+    world: World,
+    seen: Vec<CoverVariant>,
+}
+
+impl SimCore for Covers {
+    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
+        self.seen.extend(
+            input
+                .commands
+                .iter()
+                .filter_map(|(_, r)| r.map(|r| r.cover)),
+        );
+        self.world.step(input, history)
+    }
+    fn state_hash(&self) -> u64 {
+        self.world.state_hash()
+    }
+    fn entity_records(&self) -> &[EntityRecord] {
+        self.world.entity_records()
+    }
+    fn hit_volumes(&self) -> &HitVolumes {
+        self.world.hit_volumes()
+    }
+    fn portal_states(&self) -> &PortalStates {
+        self.world.portal_states()
+    }
+    fn reports(&self) -> &InputReports {
+        self.world.reports()
+    }
+}
+
+/// A5 (advisor-u4e-r1 R2; u4-build-plan.md §4.9's two cover variants, §4.11's rule): a room built
+/// `with_cover(CoverVariant::Present)` hands the sim each Rewind with `cover` Present; a room left at its default,
+/// Rewound.
+#[test]
+fn a5_a_room_with_cover_present_hands_the_sim_each_rewind_with_cover_present() {
+    let scene = Scene::course();
+    let covers = |prep: fn(Room<Covers>) -> Room<Covers>| {
+        let sim = Covers {
+            world: World::new(&scene, 1, 100),
+            seen: Vec::new(),
+        };
+        let mut room = room_with(sim, &scene, 1, prep);
+        for _ in 0..5 {
+            room.tick();
+        }
+        for seq in 1..=3u16 {
+            let next = room.last_tick() + 1;
+            let fire = Command {
+                tick: next,
+                seq,
+                buttons: Command::FIRE,
+                view_tick: next - 1,
+                ..Command::default()
+            };
+            room.receive(
+                0,
+                &Message::Input(Input {
+                    commands: vec![fire],
+                    ..Input::default()
+                }),
+            );
+            room.tick();
+        }
+        room.sim().seen.clone()
+    };
+    let present = covers(|r| r.with_cover(CoverVariant::Present));
+    assert!(
+        present.len() >= 3 && present.iter().all(|c| *c == CoverVariant::Present),
+        "{present:?}"
+    );
+    let default = covers(|r| r);
+    assert!(
+        !default.is_empty() && default.iter().all(|c| *c == CoverVariant::Rewound),
+        "{default:?}"
+    );
+}
+
+/// TE-8 with RR-2 (advisor-u4e-r1 R5; carry 50, brief e12's "an event at age 255 is still sent, none at 256", for
+/// what the room sends): a client that acknowledges its join's full state and nothing after keeps its gameplay events
+/// queued and resent; the first is carried at age 255 and by no snapshot from age 256 on, when the room drops it from
+/// the resend and counts it expired.
+#[test]
+fn carry_50_the_room_sends_an_event_at_age_255_and_none_at_256() {
+    let scene = Scene::stress();
+    let (seed, first) = (4, 400);
+    let script = recorded(&scene, seed, first, 320);
+    let mut room = room_with(World::new(&scene, seed, first), &scene, 8, |r| {
+        r.with_event_cap(1 << 16)
+    });
+    let mut oldest: Option<(u16, u32)> = None;
+    let mut ages: Vec<u32> = Vec::new();
+    let mut last = 0;
+    drive(
+        &mut room,
+        8,
+        300,
+        |s, next| s != 0 || next == first + 1,
+        |s, t| script.get(&(s, t)).copied(),
+        |_, s, out| {
+            if s != 0 {
+                return;
+            }
+            last = out.tick;
+            for snap in &out.snapshots {
+                for &(n, age, e) in &snap.events {
+                    if !matches!(e, SimEvent::Hit { .. } | SimEvent::Kill { .. }) {
+                        continue;
+                    }
+                    let at = out.tick.wrapping_sub(u32::from(age));
+                    if oldest.is_none() {
+                        oldest = Some((n, at));
+                    }
+                    if oldest == Some((n, at)) {
+                        ages.push(u32::from(age));
+                    }
+                }
+            }
+        },
+    );
+    let (n, at) = oldest.expect("a gameplay event for slot 0");
+    let k = room
+        .client(0)
+        .map(|c| c.counters().clone())
+        .unwrap_or_default();
+    println!(
+        "carry 50: event {n} of tick {at} carried {} times, at ages {:?} to {:?}; last tick {last}; events expired {}, \
+         overflow resyncs {}",
+        ages.len(),
+        ages.first(),
+        ages.last(),
+        k.events_expired,
+        k.overflow_resyncs
+    );
+    assert!(last >= at + 260, "the run reaches past age 256: {last}");
+    assert_eq!(k.overflow_resyncs, 0, "no resync cleared the queue");
+    assert!(ages.contains(&255), "carried at age 255: {ages:?}");
+    assert_eq!(ages.iter().max(), Some(&255), "none at 256 or older");
+    assert!(k.events_expired >= 1);
+}
+
 /// e9: an acknowledgement naming a tick never sent refuses its datagram's commands (§4.14 :471).
 #[test]
 fn e9_an_ack_naming_a_never_sent_tick_refuses_its_datagrams_commands() {
@@ -831,15 +1080,16 @@ fn carry_54_an_out_of_range_record_skips_that_clients_snapshot_alone() {
 
 /// Carry 57, first part (and -22's clean half): a never-acking client overflows its queue: one resync, counted with
 /// its tick; its headroom falls to 0 and never below; the cap is the room's data, its provisional mark exposed.
-/// Then a second overflow before the full state's acknowledgement sends `Kick(overflow)` and drops the client.
+/// Then a second overflow before the full state's acknowledgement sends `Kick(overflow)` and drops the client. Returns
+/// the overflow resyncs' ticks, the lowest headroom, and the tick of the first `Kick(overflow)`.
 fn never_acker(
     room: &mut Room<World>,
     script: &BTreeMap<(u8, u32), Command>,
     ticks: u32,
-) -> (Vec<u32>, i64, bool) {
+) -> (Vec<u32>, i64, Option<u32>) {
     let mut resyncs = Vec::new();
     let mut min_headroom = i64::MAX;
-    let mut kicked = false;
+    let mut kicked = None;
     drive(
         room,
         8,
@@ -856,10 +1106,14 @@ fn never_acker(
                 }
                 min_headroom = min_headroom.min(c.headroom());
             }
-            kicked |= out
-                .control
-                .iter()
-                .any(|m| matches!(m, Message::Kick(npt_proto::Kick::Overflow)));
+            if kicked.is_none()
+                && out
+                    .control
+                    .iter()
+                    .any(|m| matches!(m, Message::Kick(npt_proto::Kick::Overflow)))
+            {
+                kicked = Some(out.tick);
+            }
         },
     );
     (resyncs, min_headroom, kicked)
@@ -886,7 +1140,15 @@ fn carry_57_a_never_acker_overflows_once_then_a_second_overflow_kicks() {
         min_headroom, 0,
         "the headroom reaches 0 and never goes below"
     );
-    assert!(kicked && c.kicked() && c.counters().kicked == 1);
+    // TE-6 (advisor-u4e-r1 R6): the second overflow kicks, before the full state's timeout would (its first tick past
+    // the window), so the timeout cannot stand in for this edge.
+    let timeout = resyncs[0] + npt_sim::FULL_STATE_ACK_TICKS + 1;
+    println!(
+        "carry 57: first overflow resync at {}, Kick(overflow) at {kicked:?}, the timeout's tick {timeout}",
+        resyncs[0]
+    );
+    assert!(kicked.is_some_and(|k| k < timeout), "{kicked:?}");
+    assert!(c.kicked() && c.counters().kicked == 1);
     assert!(!room.roster().is_live(0), "the client is dropped");
     assert!(
         room.client(1).is_some_and(|c| !c.kicked()),
@@ -1038,7 +1300,7 @@ fn pm_0001_58_the_rooms_ack_sequence_and_press_compares_accept_across_their_wrap
 }
 
 /// PM-0001-22/unbounded's clean half and its headroom (U4E-Q12 (c)): the never-acker's headroom never goes below 0.
-fn unbounded_case(mut room: Room<World>, scene: &Scene) -> (Vec<u32>, i64, bool) {
+fn unbounded_case(mut room: Room<World>, scene: &Scene) -> (Vec<u32>, i64, Option<u32>) {
     let script = recorded(scene, 4, 400, 400);
     never_acker(&mut room, &script, 150)
 }
@@ -1107,7 +1369,44 @@ mod bites {
         });
         let (resyncs, min_headroom, kicked) = unbounded_case(room, &scene);
         assert!(min_headroom < 0, "headroom below 0: {min_headroom}");
-        assert!(resyncs.is_empty() && !kicked, "no resync and no kick");
+        assert!(
+            resyncs.is_empty() && kicked.is_none(),
+            "no resync and no kick"
+        );
+    }
+
+    /// A1 (advisor-u4e-r1 R2; plan.md:508-509, "Encoding stays on the tick"): `take_control` hands out the full state
+    /// the room encoded with its own `Quant`. Under PM-0001-57/pos-4cm (npt-proto's, forwarded by `Room::with_bite`)
+    /// its bytes are that `Quant`'s encoding of the message they decode to, and differ from the clean twin room's,
+    /// which are `npt_proto::encode`'s (a re-encoding of the decoded bitten message with `npt_proto::encode` would equal
+    /// them: -57 coarsens the integers before packing, and coarse integers pack unchanged).
+    #[test]
+    fn a1_take_control_hands_out_the_full_state_encoded_with_the_rooms_quant() {
+        let id = "PM-0001-57/pos-4cm";
+        let scene = Scene::stress();
+        let full = |prep: &dyn Fn(Room<World>) -> Room<World>| {
+            let mut room = room_with(warmed(&scene, 1, 200, 30), &scene, 1, prep);
+            room.tick();
+            room.take_control(0)
+                .into_iter()
+                .find(|b| matches!(control_of(b), Message::FullState(_)))
+                .expect("the join's full state")
+        };
+        let bitten = full(&|r| r.with_bite(id).expect("npt-proto plants it"));
+        let clean = full(&|r| r);
+        let mut again = Vec::new();
+        npt_proto::Quant::with_bite(id)
+            .expect("npt-proto plants it")
+            .encode(&control_of(&bitten), &mut again)
+            .expect("it encodes");
+        assert_eq!(again, bitten, "the room's Quant's encoding");
+        let mut wire = Vec::new();
+        npt_proto::encode(&control_of(&clean), &mut wire).expect("it encodes");
+        assert_eq!(
+            wire, clean,
+            "the clean room's bytes are npt_proto::encode's"
+        );
+        assert_ne!(bitten, clean, "the bite reached the full state");
     }
 
     #[test]
````

### npt-server's tests (crates/server/tests)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/server/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests`:  3 files changed, 273 insertions(+), 22 deletions(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/common/mod.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/common/mod.rs
index 59820ca..5bfda1b 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/common/mod.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/common/mod.rs
@@ -13,7 +13,7 @@ use npt_client::ClientEndpoint;
 use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
 use npt_proto::{Join, Limits, Message, Quant};
 use npt_room::Room;
-use npt_server::Decoder;
+use npt_server::{Decoder, RttSource};
 use npt_sim::replay::Recording;
 use npt_sim::{Command, Scene, World};
 
@@ -41,6 +41,22 @@ pub fn join(slot: u8) -> Join {
     }
 }
 
+/// The stream a reliable message the room encoded travels on: its type byte's row of `Limits::TYPES` (npt-proto's rule,
+/// limits.rs:55), the type byte read after the stream's length prefix (2 B on the control stream, 4 B on the
+/// full-state stream: the catalogue's §1).
+pub fn stream_of(bytes: &[u8]) -> Channel {
+    [(Channel::Control, 2), (Channel::FullState, 4)]
+        .into_iter()
+        .find(|&(channel, at)| {
+            bytes
+                .get(at)
+                .and_then(|&t| usize::from(t).checked_sub(1))
+                .and_then(|i| Limits::TYPES.get(i))
+                .is_some_and(|r| r.1 == channel)
+        })
+        .map_or(Channel::Control, |(channel, _)| channel)
+}
+
 /// A window of ns in which a [`Gate`] drops every datagram it is asked to send.
 #[derive(Clone, Default)]
 pub struct Window(Rc<Cell<(u64, u64)>>);
@@ -187,7 +203,14 @@ impl Lab {
                         window: window.clone(),
                         dropped: Rc::default(),
                     },
-                    decoder: Decoder::new(Quant::default(), hz, base_ns, VERSION, CONTENT),
+                    decoder: Decoder::new(
+                        Quant::default(),
+                        hz,
+                        base_ns,
+                        VERSION,
+                        CONTENT,
+                        RttSource::Link,
+                    ),
                     endpoint,
                     slot: None,
                     offset_ns: offsets.get(i).copied().unwrap_or(1_000_000),
@@ -217,7 +240,14 @@ impl Lab {
 
     /// The same lab with session `i`'s pump decoding with `quant`.
     pub fn with_decoder(mut self, i: usize, quant: Quant) -> Lab {
-        self.conns[i].decoder = Decoder::new(quant, self.hz, self.base_ns, VERSION, CONTENT);
+        self.conns[i].decoder = Decoder::new(
+            quant,
+            self.hz,
+            self.base_ns,
+            VERSION,
+            CONTENT,
+            RttSource::Link,
+        );
         self
     }
 
@@ -271,14 +301,9 @@ impl Lab {
                 }
                 conn.server.send_unreliable(&d, now);
             }
-            for m in self.room.take_control(slot) {
-                let channel = Limits::TYPES
-                    .iter()
-                    .find(|r| r.0 == m.name())
-                    .map_or(Channel::Control, |r| r.1);
-                let mut out = Vec::new();
-                npt_proto::encode(&m, &mut out).expect("the room's control messages encode");
-                conn.server.send_reliable(channel, &out, now);
+            // The room encoded each once (A1); the lab sends the bytes as they are, on their type's stream.
+            for b in self.room.take_control(slot) {
+                conn.server.send_reliable(stream_of(&b), &b, now);
             }
             if let Some(h) = self.room.client(slot).and_then(|c| c.server_hash()) {
                 self.server.insert((slot, t), h);
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/loopback.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/loopback.rs
index ef6267a..e0ab2fd 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/loopback.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/loopback.rs
@@ -198,6 +198,44 @@ fn e6_the_window_counts_the_pumps_bytes() {
     );
 }
 
+/// TE-2 (advisor-u4e-r1 R6; e6's binding case, §4.5 :300-302): on one tick, the same room state with the pump's report
+/// raised by `P` bytes sends at most the remainder (the per-tick budget less every pump byte), while its twin with
+/// nothing added sends more than that remainder: the remainder, not the whole budget, bounds what the encoder packs.
+#[test]
+fn e6_the_pumps_bytes_shrink_the_snapshot_to_the_remainder() {
+    const P: u32 = 600;
+    let scene = Scene::stress();
+    let run = |p: u32| {
+        let script = recorded(&scene, 3, 300, 40);
+        let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
+        let mut lab = Lab::new(room(&scene, 3, 300, 48), endpoints(48), &offsets, script);
+        lab.run(20, |_| {});
+        lab.room.pump_report(0, p, common::MAX_DATAGRAM);
+        let mut at = (0, 0, 0, 0);
+        lab.run(1, |lab| {
+            if let Some(c) = lab.room.client(0) {
+                let (budget, left) = c.budget();
+                at = (budget, left, c.bytes(), c.groups());
+            }
+        });
+        (at, lab.compare())
+    };
+    let ((budget, left, bytes, groups), (compared, bad)) = run(P);
+    let ((twin_budget, _, twin_bytes, _), _) = run(0);
+    println!(
+        "e6 (binding): budget {budget} B, remainder {left} B after {P} B more of the pump's; sent {bytes} B in \
+         {groups} groups; the twin with none added sent {twin_bytes} B; {compared} pairs compared, {bad} bad"
+    );
+    assert_eq!(budget, twin_budget, "the same tick of the window");
+    assert!(left + P <= budget, "the remainder counts the added bytes");
+    assert!(groups >= 1 && bytes <= left, "{bytes} B sent of {left}");
+    assert!(twin_bytes > left, "the twin's snapshot would not fit it");
+    assert_eq!(bad, 0, "the shrunk snapshot decodes to the mirror's hash");
+}
+
+/// The first tick `tick16_lab`'s room runs.
+const TICK16_FIRST: u32 = 65_520;
+
 /// -17/tick16 and -58/tick16's unit proof through the room's encoder and the endpoint (U4E-Q4 (a); §4.14 :459): over
 /// Loopback with the room's tick past 65,535, the client files tick 65,537 under itself with the mirror's hash.
 fn tick16_case(lab: &mut Lab) -> (Option<u64>, Option<u64>) {
@@ -208,8 +246,8 @@ fn tick16_case(lab: &mut Lab) -> (Option<u64>, Option<u64>) {
 
 fn tick16_lab(bite: Option<&str>) -> Lab {
     let scene = Scene::stress();
-    let script = recorded(&scene, 1, 65_520, 40);
-    let mut r = room(&scene, 1, 65_520, 2);
+    let script = recorded(&scene, 1, TICK16_FIRST, 40);
+    let mut r = room(&scene, 1, TICK16_FIRST, 2);
     let mut e = endpoints(2);
     #[cfg(feature = "bites")]
     if let Some(id) = bite {
@@ -223,13 +261,30 @@ fn tick16_lab(bite: Option<&str>) -> Lab {
     Lab::new(r, e, &[3_000_000; 2], script)
 }
 
+/// The clean half (advisor-u4e-r1 R1, what the judge counts): every pair compared equal (at least 30), tick 65,537's
+/// pair among them, and no client hash outside the ticks the room ran (65,520 on).
 #[test]
 fn pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself() {
-    let (server, client) = tick16_case(&mut tick16_lab(None));
+    let mut lab = tick16_lab(None);
+    let (server, client) = tick16_case(&mut lab);
+    let (compared, bad) = lab.compare();
+    let below: Vec<(u8, u32)> = lab
+        .client
+        .keys()
+        .filter(|k| k.1 < TICK16_FIRST)
+        .copied()
+        .collect();
+    println!(
+        "tick16 clean: the server's hash at 65,537 {server:?}, the client's {client:?}; {compared} pairs compared, \
+         {bad} bad; client keys below 65,520 {below:?}; client 0 stale {}",
+        lab.conns[0].endpoint.stale()
+    );
     assert!(
         server.is_some() && server == client,
         "{server:?} {client:?}"
     );
+    assert!(compared >= 30 && bad == 0, "{compared} compared, {bad} bad");
+    assert!(below.is_empty(), "{below:?}");
 }
 
 /// The bites that need every crate's list (bites builds only, under `--workspace --all-features`: npt-server's
@@ -238,15 +293,40 @@ fn pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself() {
 mod bites {
     use super::*;
 
+    /// Part (i) (advisor-u4e-r1 R1: what the judge counts): under either bite the header names the tick's low 16 bits,
+    /// so after 65,535 the client reads 0, 1, 2 ..., ticks before its first header and at least the ring behind its
+    /// newest: bad headers, each filed as the sentinel under the tick it names, a client hash the server never logged
+    /// (an orphan). The figure fails (a bad pair), slot 0 holds an orphan below 65,520, and nothing is filed at or
+    /// past 65,536.
     #[test]
-    fn pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_client_hash() {
+    fn pm_0001_17_and_58_tick16_post_wrap_frames_are_filed_as_orphan_sentinels() {
         for id in ["PM-0001-17/tick16", "PM-0001-58/tick16"] {
             let mut lab = tick16_lab(Some(id));
             let (server, client) = tick16_case(&mut lab);
+            let (compared, bad) = lab.compare();
+            let slot0: Vec<(u32, u64)> = lab
+                .client
+                .iter()
+                .filter(|(k, _)| k.0 == 0)
+                .map(|(k, h)| (k.1, *h))
+                .collect();
+            let orphans: Vec<u32> = slot0
+                .iter()
+                .filter(|(t, _)| *t < TICK16_FIRST && !lab.server.contains_key(&(0, *t)))
+                .map(|(t, _)| *t)
+                .collect();
+            let post: Vec<u32> = slot0
+                .iter()
+                .filter(|(t, _)| *t >= 65_536)
+                .map(|(t, _)| *t)
+                .collect();
             println!(
-                "{id}: the server's hash at 65,537 {server:?}, the client's {client:?}; client 0 dropped {} stale",
+                "{id}: the server's hash at 65,537 {server:?}, the client's {client:?}; {compared} pairs compared, \
+                 {bad} bad; client 0's orphans {orphans:?}; at or past 65,536 {post:?}; client 0 stale {}",
                 lab.conns[0].endpoint.stale()
             );
+            assert!(bad >= 1, "{id}: the figure fails");
+            assert!(!orphans.is_empty() && post.is_empty(), "{id}");
             assert!(server.is_some() && server != client, "{id}");
         }
     }
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/pump.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/pump.rs
index 295ad2b..d0804ea 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/server/tests/pump.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/pump.rs
@@ -6,9 +6,10 @@ mod common;
 use common::{CONTENT, VERSION, bind, join, token};
 use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
 use npt_proto::{
-    Direction, Input, Join, JoinRejected, Kick, Message, Ping, Pong, Quant, Resync, decode, encode,
+    Direction, Input, Join, JoinRejected, Kick, Limits, Message, Ping, Pong, Quant, Resync, decode,
+    encode,
 };
-use npt_server::{ConnLimits, Decoder};
+use npt_server::{ConnLimits, Decoder, RttSource};
 use npt_sim::{Scene, World};
 
 fn bytes(m: &Message) -> Vec<u8> {
@@ -17,8 +18,9 @@ fn bytes(m: &Message) -> Vec<u8> {
     out
 }
 
+/// A `wt` or L1 session's decoder (its RTT the link's), accepted at 0 at 30 Hz.
 fn decoder(quant: Quant) -> Decoder {
-    Decoder::new(quant, 30, 0, VERSION, CONTENT)
+    Decoder::new(quant, 30, 0, VERSION, CONTENT, RttSource::Link)
 }
 
 /// Everything the client end has received by `now`, decoded.
@@ -251,14 +253,14 @@ impl Link for NoRtt {
 
 /// e10 and 2.2: the `Pong`'s hold time counts up to one tick period (pinned at 30 and 60 Hz), the pump sends a few
 /// pings a second and counts their bytes, a `Pong` naming no outstanding ping is dropped and counted (plan silent:
-/// built and tested here), and the RTT is the link's, else the lowest ping sample.
+/// built and tested here), and a `ws` session's RTT is the lowest ping sample; a `wt` or L1 session's is the link's.
 #[test]
 fn e10_the_hold_time_counts_one_tick_period_and_unknown_pongs_are_dropped() {
     assert_eq!(ConnLimits::new(30, 0).hold_cap_ns(), 33_333_333);
     assert_eq!(ConnLimits::new(60, 0).hold_cap_ns(), 16_666_666);
     let (s, mut c) = Loopback::pair(1_200);
     let mut s = NoRtt(s);
-    let mut d = decoder(Quant::default());
+    let mut d = Decoder::new(Quant::default(), 30, 0, VERSION, CONTENT, RttSource::Pings);
     c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
     let mut out = Vec::new();
     d.pump(&mut s, 0, 0, &mut out);
@@ -286,12 +288,156 @@ fn e10_the_hold_time_counts_one_tick_period_and_unknown_pongs_are_dropped() {
     assert_eq!(d.rtt_ns(&s), Some(100_000_000 - 33_333_333));
     let (s2, _c2) = Loopback::pair(1_200);
     assert_eq!(
-        d.rtt_ns(&s2),
+        decoder(Quant::default()).rtt_ns(&s2),
         Some(0),
         "Loopback's own RTT for an L1 or wt session"
     );
 }
 
+/// One RTT sample on a session of kind `rtt` over `s` (its client end `c`): joined at 0, the room's ping sent at
+/// 1,000 ns, the client's `Pong` (hold 0) sent at 1,000 ns + 100 ms, the pump run `wait_ns` after it arrived.
+fn one_sample<L: Link>(mut s: L, c: &mut Loopback, rtt: RttSource, wait_ns: u64) -> (Decoder, L) {
+    let mut d = Decoder::new(Quant::default(), 30, 0, VERSION, CONTENT, rtt);
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    d.accept();
+    d.pump(&mut s, 1_000, 7, &mut out);
+    let Some(Message::Ping(p)) = received(c, 1_000).pop() else {
+        panic!("a ping");
+    };
+    let at = 1_000 + 100_000_000;
+    let pong = Pong {
+        id: p.id,
+        echo_ns: p.sent_ns,
+        tick: 0,
+        hold_us: 0,
+    };
+    c.send_unreliable(&bytes(&Message::Pong(pong)), at);
+    d.pump(&mut s, at + wait_ns, 7, &mut out);
+    assert_eq!(
+        (d.counters().pings_sent, d.counters().pong_unknown),
+        (1, 0),
+        "the Pong is the ping's: one sample"
+    );
+    (d, s)
+}
+
+/// NC-M3 (advisor-u4e-r1 R2): the RTT sample is taken at the `Pong`'s arrival, not when the pump runs: a `Pong` sent
+/// 100 ms after its ping (hold 0) and pumped 20 ms after it arrived gives 100 ms, not 120 ms.
+#[test]
+fn e10_the_rtt_sample_is_taken_at_the_pongs_arrival_not_at_the_pump() {
+    let (s, mut c) = Loopback::pair(1_200);
+    let (d, s) = one_sample(NoRtt(s), &mut c, RttSource::Pings, 20_000_000);
+    assert_eq!(d.rtt_ns(&s), Some(100_000_000));
+}
+
+/// A2 (advisor-u4e-r1 R2; u4-build-plan.md §4.9 :352-354): a `ws` session's RTT is its ping samples', never the
+/// link's, even over a link that answers one (Loopback's 0).
+#[test]
+fn a2_a_pings_session_takes_its_ping_sample_never_the_links_rtt() {
+    let (s, mut c) = Loopback::pair(1_200);
+    assert_eq!(s.rtt_ns(), Some(0), "the link answers");
+    let (d, s) = one_sample(s, &mut c, RttSource::Pings, 0);
+    assert_eq!(d.rtt_ns(&s), Some(100_000_000));
+}
+
+/// A2: a `wt` or L1 session's RTT is the link's, `None` while the link has none though a ping sample is held, never
+/// the sample.
+#[test]
+fn a2_a_link_session_takes_the_links_rtt_never_a_ping_sample() {
+    let (s, mut c) = Loopback::pair(1_200);
+    let (d, s) = one_sample(NoRtt(s), &mut c, RttSource::Link, 0);
+    assert_eq!(d.rtt_ns(&s), None);
+    let (s2, _c2) = Loopback::pair(1_200);
+    assert_eq!(d.rtt_ns(&s2), Some(0), "a link with an estimate: its own");
+}
+
+/// TE-5 (advisor-u4e-r1 R6; U4E-Q11 (a)'s size check): an `Input` datagram one byte over `Limits::INPUT` is refused
+/// and counted as too large; the room gets nothing and the session stays open.
+#[test]
+fn e9_an_input_one_byte_over_its_limit_is_counted_too_large() {
+    let (mut s, mut c, mut d) = joined(Quant::default());
+    let mut big = input();
+    big.resize(Limits::INPUT + 1, 0);
+    c.send_unreliable(&big, 1);
+    let mut out = Vec::new();
+    d.pump(&mut s, 1, 0, &mut out);
+    assert_eq!(
+        (d.counters().too_large, out.len(), d.is_closed()),
+        (1, 0, false)
+    );
+}
+
+/// TE-5: a control-stream message one byte over the stream's cap (the largest client-to-room control type's limit,
+/// its 2 B length included) is refused as too large and closes the session as malformed, with `Kick(malformed)`.
+#[test]
+fn e9_a_control_message_one_byte_over_the_streams_cap_is_too_large_and_closes_the_session() {
+    let cap = Limits::TYPES
+        .iter()
+        .filter(|r| r.1 == Channel::Control && r.2 != Direction::RoomToClient)
+        .map(|r| r.3)
+        .max()
+        .expect("client-to-room control types");
+    let (mut s, mut c, mut d) = joined(Quant::default());
+    let mut big = bytes(&Message::Join(join(0)));
+    big.resize(cap + 1, 0);
+    let length = u16::try_from(big.len() - 2).expect("a 2 B length");
+    big[..2].copy_from_slice(&length.to_le_bytes());
+    c.send_reliable(Channel::Control, &big, 1);
+    let mut out = Vec::new();
+    d.pump(&mut s, 1, 0, &mut out);
+    assert!(out.is_empty() && d.is_closed());
+    assert_eq!(
+        (d.counters().too_large, d.counters().stream_malformed),
+        (1, 1)
+    );
+    // The joined session's first pump sent its first ping before it read; the close's Kick comes last.
+    assert_eq!(
+        received(&mut c, 1).last(),
+        Some(&Message::Kick(Kick::Malformed))
+    );
+}
+
+/// SEC-U4E-1 (advisor-u4e-r1 R5; catalogue :93, "any other type first on the stream closes the session as
+/// malformed"): before the `Join`, a known type on the wrong channel first on the stream (an `Input` framed on the
+/// control stream) closes the session as malformed; the `Join` after it is never taken.
+#[test]
+fn sec_u4e_1_a_forged_kind_first_on_the_stream_closes_the_session() {
+    let (mut s, mut c) = Loopback::pair(1_200);
+    let mut d = decoder(Quant::default());
+    let body = input();
+    let mut framed = (body.len() as u16).to_le_bytes().to_vec();
+    framed.extend_from_slice(&body);
+    c.send_reliable(Channel::Control, &framed, 0);
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    assert!(out.is_empty() && d.is_closed());
+    assert_eq!(
+        (d.counters().first_not_join, d.counters().wrong_channel),
+        (1, 1)
+    );
+    assert_eq!(received(&mut c, 0), [Message::Kick(Kick::Malformed)]);
+}
+
+/// SEC-U4E-3 (advisor-u4e-r1 R5): a stream message while the session's `Join` waits for the room's answer is dropped
+/// and counted under its own name; the session stays open.
+#[test]
+fn sec_u4e_3_a_stream_message_while_joining_is_counted() {
+    let (mut s, mut c) = Loopback::pair(1_200);
+    let mut d = decoder(Quant::default());
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    assert!(matches!(out.as_slice(), [(Message::Join(_), 0)]));
+    out.clear();
+    c.send_reliable(Channel::Control, &bytes(&Message::Resync(Resync)), 1);
+    d.pump(&mut s, 1, 0, &mut out);
+    assert!(out.is_empty() && !d.is_closed());
+    assert_eq!(d.counters().joining_dropped, 1);
+}
+
 /// The socket checks through the pump (A26; -21's two decode sites): an `Input` with a byte after its last field is
 /// refused and counted as trailing bytes, one on the control stream as the wrong channel (refused and counted, never
 /// kicked: §4.10); the session stays open and the room gets neither.
````

### npt-client's tests (crates/client/tests)
_`git diff --no-index .gate/u4e-r1/spikes/netcode-proof/crates/client/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests`:  1 file changed, 130 insertions(+), 1 deletion(-)._

````diff
diff --git a/.gate/u4e-r1/spikes/netcode-proof/crates/client/tests/endpoint.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/endpoint.rs
index 60b5b21..137d655 100644
--- a/.gate/u4e-r1/spikes/netcode-proof/crates/client/tests/endpoint.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/endpoint.rs
@@ -4,7 +4,8 @@
 use npt_client::{ClientEndpoint, StepOut};
 use npt_link::Channel;
 use npt_proto::{
-    EntityDelta, FullState, Join, JoinAccepted, Limits, Message, Snapshot, client_hash, encode,
+    Direction, EntityDelta, FullState, Join, JoinAccepted, Limits, Message, Snapshot, client_hash,
+    decode, encode,
 };
 use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};
 
@@ -149,6 +150,105 @@ fn e3_a_lost_group_leaves_its_tick_unlogged_and_a_refused_one_logs_the_sentinel(
     assert!(!out.hashes.iter().any(|h| h.0 == 101));
 }
 
+/// The acknowledgements (tick, group mask) in the datagrams one step sent.
+fn acks(out: &StepOut) -> Vec<(u16, u8)> {
+    out.datagrams
+        .iter()
+        .filter_map(
+            |d| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
+                Ok(Message::Input(i)) => Some((i.ack_tick, i.ack_groups)),
+                _ => None,
+            },
+        )
+        .collect()
+}
+
+/// TE-4 (advisor-u4e-r1 R6; plan :716-717's "each group decoding alone"): group 1 of a split tick arrives and group 0
+/// never does. Group 1 decodes alone against the baseline and is acknowledged (the mask without bit 0), nothing is
+/// filed for the tick, and it is resolved lost once it has left the frame ring, never the sentinel.
+#[test]
+fn e3_a_split_tick_without_group_0_acknowledges_its_other_group_and_is_resolved_lost() {
+    let mut e = joined(endpoint(), 100);
+    let out = feed(&mut e, vec![datagram(moved(101, 1, 1, 2, 150))]);
+    assert!(out.hashes.is_empty(), "one group of two: nothing filed");
+    assert_eq!(acks(&out), [(101, 0b10)], "group 1 acknowledged alone");
+    assert_eq!(e.refused(), 0);
+    let fresh = Snapshot {
+        tick: 101 + npt_sim::ACK_RING_TICKS,
+        entities: vec![EntityDelta::Add(soldier(1, 100))],
+        ..Snapshot::default()
+    };
+    let out = feed(&mut e, vec![datagram(fresh)]);
+    assert!(
+        out.lost.iter().any(|&(a, n)| a <= 101 && 101 < a + n),
+        "{:?}",
+        out.lost
+    );
+    assert!(
+        !out.hashes.iter().any(|h| h.0 == 101),
+        "lost, never the sentinel"
+    );
+}
+
+/// TE-7 (carry 61 (b): "two different hashes for one tick give the sentinel"): one group received again with other
+/// bytes, and a tick completed by its full state and then sent again as a snapshot of other content: each files the
+/// sentinel after the tick's first hash.
+#[test]
+fn carry_61_b_a_second_different_hash_for_one_tick_gives_the_sentinel() {
+    let mut e = joined(endpoint(), 100);
+    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 1, 150))]);
+    assert_eq!(
+        out.hashes,
+        [(101, hash_of(101, &[soldier(1, 150), soldier(2, 200)]))]
+    );
+    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 1, 151))]);
+    assert_eq!(out.hashes, [(101, 0)], "the same group with other bytes");
+    let full = bytes(&Message::FullState(FullState {
+        tick: 102,
+        last_run: None,
+        own: vec![soldier(0, 0)],
+        entities: vec![soldier(1, 100)],
+    }));
+    let out = feed(&mut e, vec![(Channel::FullState, full, 0)]);
+    assert_eq!(out.hashes, [(102, hash_of(102, &[soldier(1, 100)]))]);
+    let other = Snapshot {
+        tick: 102,
+        entities: vec![EntityDelta::Add(soldier(1, 300))],
+        ..Snapshot::default()
+    };
+    let out = feed(&mut e, vec![datagram(other)]);
+    assert_eq!(out.hashes, [(102, 0)], "tick 102 again, with another hash");
+}
+
+/// SEC-U4E-7 (advisor-u4e-r1 R5): four incomplete groups 2^31 - 1 ticks apart in one input (a hostile server's) never
+/// overflow a lost run's count: a sum past `u32::MAX` starts a new run, and the runs, each continuing the one before,
+/// cover every tick resolved (from 101, the tick after the full state, to the last header's ring cutoff).
+#[test]
+fn sec_u4e_7_lost_runs_2_pow_31_apart_never_overflow_and_cover_the_ticks() {
+    let mut e = joined(endpoint(), 100);
+    let apart = (1u32 << 31) - 1;
+    let groups = (0..4u32)
+        .map(|k| {
+            datagram(Snapshot {
+                tick: 101u32.wrapping_add(k.wrapping_mul(apart)),
+                groups: 2,
+                ..Snapshot::default()
+            })
+        })
+        .collect();
+    let out = feed(&mut e, groups);
+    let runs = &out.lost;
+    println!("SEC-U4E-7: lost runs {runs:?}");
+    assert!(runs.len() >= 2, "{runs:?}");
+    assert_eq!(runs.first().map(|r| r.0), Some(101));
+    for w in runs.windows(2) {
+        assert_eq!(w[0].0.wrapping_add(w[0].1), w[1].0, "{runs:?}");
+    }
+    let covered: u64 = runs.iter().map(|r| u64::from(r.1)).sum();
+    let want = 3 * u64::from(apart) - u64::from(npt_sim::ACK_RING_TICKS - 1);
+    assert_eq!(covered, want);
+}
+
 /// Carry 61: tick t + 1's snapshot fed before tick t's (as L1's reorder step delivers them) files t under t with its
 /// hash (the client decodes a stale group: the advisor's lean), never the sentinel; one datagram received twice files
 /// one hash; a tick with a group missing is not resolved lost while younger than `npt_sim::ACK_RING_TICKS` ticks (by
@@ -306,6 +406,35 @@ mod bites {
         assert_ne!(got, want);
     }
 
+    /// TE-3 (advisor-u4e-r1 R6; §5 U4e's "client-rounding (the client's call site)", carry 24's first clause): one
+    /// decoded snapshot fed to a clean endpoint and to one built with PM-0001-17/client-rounding: the clean one files
+    /// the mirror's hash, the bitten one another hash.
+    #[test]
+    fn pm_0001_17_client_rounding_at_the_endpoints_call_site_files_another_hash() {
+        let entities: Vec<EntityRecord> = (1..=16u16)
+            .map(|i| soldier(i, i32::from(i) * 37 + 11))
+            .collect();
+        let snap = Snapshot {
+            tick: 101,
+            entities: entities.iter().map(|r| EntityDelta::Add(*r)).collect(),
+            ..Snapshot::default()
+        };
+        let mirror = client_hash(101, &entities, &[], &[]);
+        let filed = |e: ClientEndpoint| {
+            let mut e = joined(e, 100);
+            feed(&mut e, vec![datagram(snap.clone())]).hashes
+        };
+        assert_eq!(filed(endpoint()), [(101, mirror)]);
+        let got = filed(bitten("PM-0001-17/client-rounding"));
+        println!(
+            "client-rounding at the endpoint: the mirror's {mirror}, the bitten endpoint's {got:?}"
+        );
+        assert!(
+            got.len() == 1 && got[0].0 == 101 && got[0].1 != mirror && got[0].1 != 0,
+            "{got:?}"
+        );
+    }
+
     #[test]
     fn the_endpoint_takes_npt_protos_ids_through_quant_and_no_other() {
         assert!(endpoint().with_bite("PM-0001-17/tick16").is_some());
````

## What it is judged against

### The advisor's round-1 ruling, R1 to R8: the findings' routes, the fix pass's scope and bounds, round 2's seats (advisor-u4e-r1.md)
_`.checkpoints/netcode-proof-test/advisor-u4e-r1.md` lines 57-437 (sha256 7c1293097039), as they are:_

````text
### R1. NC-H1 (High) with TE-1 (High): the finding holds; the reading, the fix's form and the unit proof. REVISE (before the merge).
**The finding holds, by reading and by run.** Under either tick16 bite the header carries the tick's low 16 bits, so
after 65,535 the client reads 0, 1, 2 …; each is at least `ACK_RING_TICKS` behind its newest (65,535), so `too_old`
drops it before `see`, a hash, the sentinel or a lost record. The test seat's Q-1 shows it: "client 0 post-wrap keys
[]; server post-wrap hashes 14; compared 30, bad 0; sentinels 0; … stale 14; lost []" (r1-te-q1-tick16.log, for both
ids). The twin's `server != client` holds with the client's `None`, which is the silent drop. §4.14's row asks for "a
frame at tick 65,537 [that] hashes differently or is refused", and its failing check is "post-wrap frames logged under
the wrong tick (an orphan pair) or as the sentinel"; the figure counts neither a dropped group nor a server-only pair
(SERIES.md:266: "a server hash with no client hash is loss, not compared"). So U4r's judged bite runs would read 0
mismatches: a bite never caught. Evidence that does not hold: High, as both seats rate it.

**The reading (mine to rule: it reads my own text, B2.1 and carry 61, not an ADR row or a word of Mustafa's).** Carry
61 (d) ("a group of a tick older than one the client has already decoded … is decoded … or, if the client drops it as
stale, it is never acknowledged and that tick is lost, never the sentinel") was written for L1's reorder step: a tick
the client was sent and can still place. A header tick falls in one of three cases:
1. **Inside the ring** (less than `npt_sim::ACK_RING_TICKS` ticks behind the newest tick the client holds): decoded and
   hashed under its own tick, as built. This is (d)'s decode, my B2.1 lean; NC-L4 and A's "decodes" are right.
2. **From the first snapshot on, at least the ring behind the newest:** that tick was resolved when it left the ring
   (lost, or filed), so nothing can be filed for it now; a group of it is dropped, never acknowledged, and counted
   (`stale`), as built. This is (d)'s drop, for a tick already lost; it creates no new lost tick.
3. **Before the client's first snapshot and at least the ring behind the newest:** a tick the client was never sent and
   never resolved. Its header is §4.6's "bad header" (the sentinel causes B2.1 keeps: "no baseline, a bad header, a
   size or decode error"): the client files the sentinel under the tick the header names. That is a client hash at a
   (slot, tick) the server never logged, which the figure counts (SERIES.md:266; §4.6 :323-325's orphan).
- Why case 3 is safe for a correct build: in an honest run no group names such a tick. L1 builds no duplication
  (l1/src/lib.rs:75) and changes no byte; a datagram arrives late by at most its profile's jitter spread plus one
  spacing (:15-23; P4's ±40 ms round trip is the widest, :195), far under the ring's 1.067 s; the room sends no
  snapshot while a client's full state is unacknowledged (room.rs:1041-1047), and that full state is the client's
  first tick. So it changes nothing in an honest run and catches the bite: stricter, never looser.
- Why not the netcode-architect's simpler form (every header at least the ring behind the newest gets the sentinel):
  for a tick of case 2 that would file a sentinel at a tick already written lost, which README §1's coverage rule
  forbids ("never both") and a writer cannot place, so a correct build meeting one very late group (possible only
  off L1, on a real network at U6) would end INVALID or `crash`. Case 3 carries the same proof without that risk.
- No readings-list entry: no ADR row or word of Mustafa's is read; -17's "100 % of decoded snapshots" is untouched (the
  sentinel is not a decoded snapshot, and the figure already counts it).

**The fix's form (npt-client; the fix pass):**
- The endpoint keeps the tick of the first header it took (its first group's or full state's).
- Wherever it meets a header tick (`group`, `full_state`, and the refused snapshot datagram's tick read in `receive`):
  case 1 as built; case 2 as built (`stale`); case 3 files `(t, 0)` in `StepOut::hashes` once per tick, with bounded
  memory, without `see`, without `slot_for` (the ring slot `t % 32` may hold a live tick: 65,537's slot is 1's), never
  acknowledged, and counted in `refused` with the other sentinel causes.
- The newest tick never moves for a case-3 header (it is behind by definition). NC-L7's far-ahead header stays a
  follow-up (R5).

**What the unit proof asserts (what the judge counts),** in `pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_
client_hash` (its name may change to say what it now shows) and its clean twin, over `tick16_lab`'s 30 ticks:
- clean build: `lab.compare()` gives (n, 0) with n ≥ 30, the client's hash at (0, 65,537) equal to the server's, and no
  client key outside the ticks the room ran (65,520 on);
- under each of PM-0001-17/tick16 and PM-0001-58/tick16: `lab.compare().1` ≥ 1 (the figure fails); slot 0 has at least
  one client key below 65,520 whose server entry is absent (an orphan), and none at or past 65,536; the test prints
  compared, bad, the orphan keys and `stale()`. (Q-1's run predicts 14 orphans, one per post-wrap tick reached.)
- **Plants** (by Edit in the copy, restored, sha256-checked; each named in the record): (a) case 3 put back to case 2's
  silent drop: the bitten twin FAILS by name; (b) s03/s04's scratch switch (the bite's defect in place in the clean
  build) re-run once: the clean test FAILS by name. Part (i) in the counted run: the twin passes and no clean test
  fails.

**The judged half is U4r's** (carry 73, T2): its client writer must place hashes filed at ticks before the run's first
tick (README §1 already asks it to buffer and sort, since "a client's rebuilt ticks can go back"), and its contract
test runs one short tick16 bite run past the wrap and shows the figure FAIL with mismatches ≥ 1, never INVALID through
a `crash` footer.

**The contract text** (marks/README.md §1, in the merge commit: T1) gains case 3 in one sentence, so the rule both
sides read holds it. No judge code changes. Severity of NC-H1 and TE-1 stays High until round 2 sees the fix.

R1 written 2026-11-12.

### R2. The six Mediums marked before the merge: all six in the fix pass. APPROVE as findings, with these forms.
Each label stands. Each code fix comes with a test that fails under a named plant (by Edit in the copy, restored,
sha256-checked, recorded), as the test seat proved its findings.
- **NC-M2 (carry 68 not met): the report's words, no code.** Carry 56's ES figure is "an ES byte figure U4e reports
  from that recording", so carry 68 applies as worded ("states those two shares beside R2's"); the two shares are the
  recording's, which carry 68 gives (about 48 % and 18 %). The report carries T5's line beside carry 56's figures, with
  the numbers of the merged tree's run (M5). A print of the own soldier's own shares would be better evidence but is
  not what the carry asks; it is not required. Carry 68 is then listed as met.
- **NC-M3 (the Pong's RTT sample from the pump's now): fix.** `pong` takes the datagram's `r.arrival_ns`, as the Ping
  answer beside it does (pump.rs:376). Test: Q-2's case committed (the Pong sent at 1,000 ns + 100 ms, the pump run 20
  ms later): `rtt_ns` gives 100 ms less the hold, where today it gives 120 ms (r1-te-q2-q5-pump.log). Plant: `now_ns`
  passed again; the test fails by name.
- **A1 (control messages handed out typed, encoded twice): fix.** `Room::take_control` hands out bytes: each reliable
  message encoded once, on the tick, with the room's own `Quant` (the encode `full_state` already does to count a
  refusal is the one kept; `JoinAccepted` and `Kick` likewise), so plan.md:508-509's "Encoding stays on the tick" holds
  in the library, not in each caller. The caller sends each on the channel its type byte's row of `Limits::TYPES`
  names (npt-proto's documented rule, limits.rs:55); the lab (server/tests/common/mod.rs) does exactly that and no
  longer encodes. Test (bites builds): under `Room::with_bite("PM-0001-57/pos-4cm")`, one join; the FullState bytes from
  `take_control` equal `Quant::with_bite("PM-0001-57/pos-4cm")`'s encoding of the message they decode to and differ
  from `npt_proto::encode`'s. Plant: `take_control`'s bytes encoded with `npt_proto::encode`; the test fails by name.
- **A2 (the RTT source chosen by what the link answers): fix.** §4.9 (u4-build-plan.md:352-354) names the source by the
  session's kind, which the pump learns from its listener. `Decoder::new` takes it as a new npt-server type,
  `RttSource` with two variants, `Link` (a wt or L1 session: the link's `rtt_ns`, `None` until it has one, never the
  ping samples) and `Pings` (a ws session: the lowest of the last 8 samples, never the link's). One new public item,
  its reason §4.9, in §4.1's npt-server row (T6). Tests: a link answering `Some(x)` with a `Pings` session and one
  sample y ≠ x gives y; a `Link` session over `NoRtt` gives `None` with a sample held. e10's ping-path test passes
  `Pings`. Plant: `rtt_ns` back to `link.rtt_ns().or_else(..)`; the first test fails by name. The report does not repeat
  the record's line that the pump's report to the room carries the RTT (u4e-build.md:22-23): `pump_report` has none,
  which is right at U4e (U4h's tolerance is its consumer).
- **A4 (`tick_input()` holding two ticks' state): fix.** The scripted spawns, teleports and attributes wait in their own
  field; `fill_input` moves them into `input` for the tick they are for; nothing clears `input` after the step. Then
  `tick_input()` after `tick()` is that tick's whole input (its doc: "what a recording holds"). Test: script one
  teleport, `tick()`, and `tick_input().teleports` holds it; a second `tick()` with no script gives none. Plant: the old
  clear after the step; the test fails by name.
- **A5 (public items beyond §4.1 and plan §4.7): fix, with one correction.** Eight of the nine items with no consumer
  become pub(crate): `Roster::new`, `Roster::is_bound` (both used only by roster.rs's own unit tests), `Decoder::limits`,
  `Decoder::is_joined`, `ConnLimits::join_deadline_ns`, `ClientEndpoint::closed`, `ClientEndpoint::unknown_pongs`,
  `ClientSlot::skipped`. **`ClientEndpoint::with_lead` stays public:** its consumer is U4r's lab, which must set the
  lead from the profile (NC-M6's carry 74, R3); the report gives that reason. `Room::with_cover` stays public (§4.9's
  two cover variants, recorded per run) and gains a test (§4.11's rule): a room built `with_cover(CoverVariant::Present)`
  hands the sim each Rewind with `cover` Present. A later unit that needs one of the eight makes it public with its
  reason. The report lists every public item beyond §4.1's rows with its reason (the engine-architect's list,
  u4e-review-arch.md:57-66, updated for A2's `RttSource`, A1's `take_control` and this item), and the merge commit
  brings §4.1's npt-room, npt-server and npt-client rows up to date (T6).

R2 written 2026-11-12.

### R3. The Mediums marked not before the merge: each one's route (labels stand)
New carries start at 73 (u4s-carries.md ends at 72) and at C-U6-12 (plan.md's U6 row ends at C-U6-11). Their words are
in T2 and T3.
- **SEC-U4E-4 (Resync, ack, Resync): the room's half moves INTO the fix pass; the transport's half is C-U6-12.** The
  test seat measured it (Q-6: 29 full states in 30 ticks, 256,418 B, about 8 times the 32,000 B a second the window
  allows; r1-te-pg-q6-q7.log). The room meets the catalogue's words (a second Resync before the ack is dropped), but the
  ack is the client's own claim, and the security-auditor's durable note is the rule: an acknowledgement must never gate
  server work by itself. The room is the rule's home and U4e owns it now; a later unit would reopen npt-room for it. Of
  the two fixes offered I take the rate limit, not counting full states against the window: it is local, it leaves the
  window's arithmetic (e6, -12, -18) as reviewed, and an honest client (a Worker restart, a long hidden tab: catalogue
  :90) never resyncs twice in 2 s. **The rule:** a client's `Resync` is answered only when at least
  `npt_sim::FULL_STATE_ACK_TICKS` ticks (by path) have passed since the tick of the last full state the room sent that
  client (its join's, a forced one or an answered one); a sooner one is dropped and counted under its own name
  (`resyncs_limited`), beside the one-in-flight rule's `resyncs_dropped`. The overflow's forced resync is not limited by
  it (its own bound is carry 57's kick). Test: Q-6's case committed: over the 30 ticks after the join's full state no
  Resync is answered and each is counted; after `FULL_STATE_ACK_TICKS` ticks one is. Plant: the limit removed; the test
  fails by name. This bounds the extra bytes to one full state per 2 s per client. The transport's half (at most one
  open full-state stream per session, the catalogue FullState row's proposal) is U6's (C-U6-12), so SEC-U4E-4 stays
  open in PR #40's report for that half.
- **NC-M4 (events packed before the nearest 8 enemies): carry 78, U4g.** The order holds by reading (room.rs:1164-1180
  before :1181-1237) and breaks the brief's Priority rule ("the nearest 8 enemies in every snapshot": u4e-brief.md:251)
  under a backlog larger than a snapshot's remainder. The test seat's Q-11 shows the gap test never meets it (about 23
  events resent a snapshot, entity changes in every snapshot from F on: r1-te-q11-nearest8.log), and no U4r mark reads
  entity freshness (-17 compares hashes; -18 reads the fresh delta; -57 reads only carried fields). U4g builds
  interpolation and event release on the room's and the client's files, where stale entities would show; it fixes the
  order first, with a test.
- **NC-M5 with A3 (no event base on the wire): carry 75 (U4r) and C-U6-13 (U6).** Q-10 shows it (an endpoint starting at
  0 rebuilds events numbered 200,000 wrong, and the hash reads them: r1-te-p23-q10.log). U4r's lab can avoid it today
  by giving each endpoint the room's numbers at its admission (`with_starts`); the wire field (JoinAccepted or FullState
  carries the room's next 32-bit event number) is npt-proto's and the catalogue's, not U4e's lane, and its first judged
  need is -19's join hash (C-U6-11), so it is U6's.
- **NC-M6 (the endpoint's default lead of 1): carry 74, U4r.** U4r runs -17 at P2, P3 and B1 and -18 at B1 with this
  endpoint before U4f's controller exists; with lead 1 every command is late above about half a tick of one-way delay,
  and -17 would compare a world of soldiers standing still. U4r sets the lead per profile with `with_lead` (which is why
  it stays public: R2, A5) and its contract test shows the commands arrive in time.
- **A6 (no library path to end a session), with SEC-U4E-5: carry 76 (U4r) and C-U6-14 (U6).** The lab ignores
  `replaced` and never closes a kicked session (server/tests/common/mod.rs:238-249), so a replaced session's Inputs keep
  reaching the slot. U4r's lab and U6's TickLoop each write this glue; each carry names the rule and its test. No
  `Decoder::close` now: U4r's lab holds the link and can send `Kick(replaced)` itself and drop the session; U6 adds a
  library item if its TickLoop needs one, with its reason.
- **RR-5 (the room's tick at 48 clients against PM-0001-4's budget): C-U6-15 (U6), and a card follow-up.** PM-0001-4 is
  judged at L2, SS, P2, 48 clients over an S30 (marks.json), which is U6's. The rust-reviewer's bench (three runs:
  room.tick 5.61 / 5.89 / 6.07 ms mean, p99 10.5 / 11.5 / 9.7 ms, the twin's step about 0.2 ms; 2,838 allocations and
  6.20 MB requested a tick, identical across runs; linear in clients: r1-rr-bench.log) puts the room's own work at about
  60-70 % of -4's p99 on this 4-vCPU VM, before the pump and the logs. Not judged at U4e, no regression against a base
  (the room is new), so a carry, with the bench method and the cheap wins named; any unit that edits the encode path
  before U6 (U4f, U4g or U4h) may take them, with the bench before and after.

R3 written 2026-11-12.

### R4. The five Lows marked before the merge: each rides the push the High needs anyway, as report words (T5)
Mustafa's rule holds ("A Low never triggers a push"): none of these would push on its own; the fix pass is needed for
R1 and R2, so they ride it as the report's words and no code.
- **NC-L1, condition 6 (§5 U4e): CONFIRMED, not triggered.** The bite's baseline is not "one the client still holds".
  §4.5 defines holding by age: "The client keeps its complete frames for the same ages [0 to 31], so every B the server
  can pick, the client holds"; the client refuses any `baseline_age` of `ACK_RING_TICKS` or more before it looks at the
  frame (client/src/lib.rs:655-663). That the bytes of B = 329 still sit in ring slot 9 (329 mod 32), because F = 363
  or 362 uses slot 11 or 10 (F − B = 34 or 33, not a multiple of 32), does not make B held: a client that decoded
  against a frame older than the ring would keep baselines the server can never pick, which no rule asks for, and it
  would be the lenient client, not the bitten room, that differs from the contract. So the bite is a defect on L1, and
  -18's L1 part counts at U4r as planned, with no further check of mine. The report states B, F, both ages, both slots,
  and this reading in one line (T5).
- **NC-L3, e7's "at least one resent" carried by gap.rs: ACCEPTED.** On zero-delay Loopback every ack lands before the
  next tick, so a legitimate build resends nothing in e7 (b11-e7print.log: resent 0), the same reason my brief check
  gave for the split clause (its A2). The mirror is compared over resent events in gap.rs's clean run (236 pairs, 0
  differ, 439 and 462 resent). The report says which test carries each clause (T5).
- **NC-L4, B2.1's answer: "decode". CONFIRMED** (R1's case 1). B5's extra reading is not needed (no readings-list entry),
  carry 62 (g)'s drop count does not apply, and `stale()` counts only R1's case 2 (a group of a tick already resolved
  when it left the ring), which is not carry 61 (d)'s drop, so it needs no SERIES.md row; U4r's contract test reports
  its sum (carry 77), 0 in an honest L1 run. The builder's proposed `tick.s.snap_too_old_count` is not added.
- **A7, carry 26: raised now, and the seam stays. ACCEPTED.** The per-tick copy is carry 23's own stamp list (each
  event with its tick and 32-bit number) into a reused `Vec` of `Copy` events (room.rs:772-779); a seam change would
  remove no copy that carry 23 does not need anyway. Carry 26 is met by this raise; the report says so.
- **A14, carry 54's series: the builder's name ACCEPTED, the row in the merge commit.** `event.snap_refused.tick_count`
  follows SERIES.md's form for a per-event tick (`event.counter_wrap.tick_count`, SERIES.md:150). Its row (T4) goes into
  SERIES.md in the merge commit (marks/ is not U4e's lane; check.mjs's marks step then reads it on the merged tree:
  M5), before U4r writes it.

R4 written 2026-11-12.

### R5. The other Lows: each one's route (labels stand; none pushes on its own)
**Ride the fix pass** (each small, in a file the fix pass edits anyway, each with its proof; the security ones close
open security findings in PR #40's report):
- **SEC-U4E-1: the catalogue's rule wins before Join.** Catalogue :93 ("any other type first on the stream closes the
  session as malformed") governs a session in AwaitJoin; §4.10's "no forged kind closes the session" is written for
  joined sessions (the forged suite's twins) and stays so. In AwaitJoin a stream message the decoder refuses as wrong
  channel or wrong direction closes the session as `first_not_join` with `Kick(malformed)`, as a decodable non-Join
  does. Test: Q-3's case committed, asserting the session closed and `first_not_join` 1. Plant: today's count-and-go-on.
- **SEC-U4E-3:** a stream message in the Joining state is counted under its own name (pump.rs:391's `_ => {}`). Test:
  Q-4's case, asserting that counter 1. Plant: `_ => {}` back.
- **SEC-U4E-7:** the lost run's add (client/src/lib.rs:526) never overflows: a sum that would pass `u32::MAX` starts a
  new run. Test: Q-7's case committed, asserting no panic and lost runs that cover the ticks resolved. Plant:
  `last.1 += count` back; the test panics by name (debug build).
- **TE-7** (carry 61 (b)'s "two different hashes for one tick give the sentinel"): a test feeding one group twice with
  different bytes, and one tick completed twice with different hashes, each giving the sentinel. Plant: P-H.
- **TE-8 with RR-2** (carry 50's room half): a room test where a never-acking client's queued event reaches age 256:
  the room drops it from the resend and counts `events_expired`, and no snapshot carries it (brief e12's "an event at
  age 255 is still sent, none at 256", shown for what the room sends); the age cast at room.rs:1170 becomes
  `u8::try_from` with a debug assert. Plant: P-G.
- **RR-3:** a const assert `Limits::GROUPS <= 8` beside the u8 group masks (room/src/ack.rs, client/src/lib.rs).
  Compile-time; no plant.
- **RR-10:** carry 23's unit test asserts that it checked at least one queued event (P-23 showed the half is not
  vacuous today; the assert keeps it so).

**Carries** (T2, T3):
- **SEC-U4E-2** (no rate limit on the control stream): C-U6-12, with SEC-U4E-4's transport half.
- **SEC-U4E-5:** with A6 (carry 76, C-U6-14).
- **SEC-U4E-8** (option B reruns a firing command with its old view time): carry 81, U4h.
- **SEC-U4E-9 with RR-7 and TE-9** (the canonical sort under option A): carry 80, U4f.
- **NC-L7** (a refused datagram whose tick reads far ahead moves `newest`) and **NC-L8** (an `Input` dropped whole when
  one command fails to encode): carry 80, U4f, the next unit to edit npt-client.
- **NC-L9, NC-L10, NC-L2** (U4r's readings of the room: one wrap record per counter name; carry 62 (h)'s setup window;
  `burst_eviction` 1 at age exactly 32 while the bitten room heals there): carry 77, U4r. NC-L10 changes my own words
  in carry 62 (h): its window "all" becomes the measured phase (each join's ticks between the full state and its ack
  are unsent, so the client resolves them lost by design); a non-zero sum in the measured phase still comes to me.
- **NC-L12** (-62's own cosmetic count per client): carry 79, U4g.
- **RR-4, A10, A12:** into RR-5's list of cheap wins (C-U6-15).

**The report's words** (T5):
- **NC-L5 = RR-1:** §4.2's "the debug assert that a correct room never sends one" is met by construction: `rewind_at`
  returns `None` before it builds a Rewind naming an unheld tick (rewind.rs:18-33), so the assert at :28-31 can never
  fire; it may stay as documentation. (RR-1 cites rewind.rs:73-77; the file has 33 lines.)
- **NC-L11 with A11:** the three values the plan leaves open are module constants: the interest radius 64 m and its
  cell (the plan is silent on both), and the hit history's 2 s (provisional until U4h: carry 28). The report names them
  as U4e's choices. A setter or getter comes with the unit that first needs one (U4h for the history; the radius per
  mode at slice 3, ADR 0001 part 9 (a)).
- **SEC-U4E-6 = A13 = NC-L11's second half:** a Hit or Kill reaches a client that sees only the victim with the
  shooter's slot (room.rs:954-968), against ADR 0001 part 9 (f) ("no shooter id when the shooter is not relevant to the
  viewer"); the wire has no form without it (U4a's), and part 9 is proven at slice 3 (ADR 0001:1687-1688). A known
  deviation until then; PR #40's report lists it as an open security finding (Low).
- **RR-8:** the six types public in name only are plan §4.7's list (u4-build-plan.md §4.1 base); A5's report list says
  so.
- **TE-10:** the endpoint's 1,150 B case is refused by its trailing bytes too; the size check's own proof is npt-proto's
  `a3_a_snapshot_is_taken_up_to_1_150_b_and_sent_up_to_1_000_b` (proto/src/msg.rs:587 on: 1,001 B and 1,150 B taken,
  1,151 B refused `TooLarge` by its size, :607-622). No code.

**Follow-ups on the card** (open findings, each with its file:line; no carry, no push): **NC-L6 = RR-9** (a carry-54
skip has already moved `sent`, the accumulator and `cosmetic_sent`: room.rs:1166-1180, :1226-1240, :1288-1291; the
next encoder edit moves them after the encode succeeds); **RR-6** (the room's full-state deadline and ring use the
30 Hz constants while `Room::new` takes `tick_hz`: room.rs:1003, :1015, ack.rs:76-77; the first unit to run a room at
another rate takes their `_at` forms by carry 39's route); **A8** (`Room::new` takes the scene and greybox beside the
sim unchecked: room.rs:408-415); **A9** (wire knowledge outside npt-proto: the client's raw tick read and the two 28 B
overheads, unpinned; the next npt-proto change gains a header-tick read and the overhead in `Limits`).

R5 written 2026-11-12.

### R6. The test-engineer's findings (REVISE): accepted at their labels; the five Mediums in the fix pass
The ids are its Block 5's, which supersede the ids written inline in its Block 4 (it says so). I read its logs for the
results each finding rests on: Q-1, Q-2, Q-6, Q-7, Q-9 and Q-10's printed lines, P-A's, P-B's and P-D''s exit 0 with
every binary ok, P-C's single failure (its own probe), P-21/22/57's two carry-22 failures, and the counted run's "check:
all 10 steps pass" with 458 and 524 (r1-te-*.log under .gate/u4e/). Each plant was restored and the copy's 23 files
equal the start list (I re-ran `sha256sum -c` in both copies: 23 of 23 OK).
- **TE-1 (High):** R1. Its probe Q-1 is what settles NC-H1.
- **TE-2 (e6's remainder never binds; P-A passes): fix.** An e6 case where the pump's bytes change what is sent: on one
  tick, the same room state with pump bytes p large enough that the snapshot must shrink (48 SS clients, or a large
  `pump_report`) sends at most budget − p, while with p = 0 it sends more than budget − p. Plant: P-A
  (`left = c.t.budget`); the case fails by name. The plan's "the window counting the pump's bytes" is then proven, not
  only read back.
- **TE-3 (client-rounding's call site in the endpoint untested; P-B passes): fix.** A bites-only twin feeding one
  decoded snapshot to a clean endpoint and to one built `with_bite("PM-0001-17/client-rounding")`: the clean one files
  the mirror's hash, the bitten one a different hash. Plant: P-B; the twin fails by name. That meets §5 U4e's
  "client-rounding (the client's call site)" and carry 24's first clause.
- **TE-4 (group 1 without group 0 not shown; P-D' passes): fix.** e2 (or e3) also drops group 0 of one split tick:
  every other group of it decodes and is acknowledged (the ack's group mask without bit 0), and the tick is resolved
  lost, never the sentinel. Plant: P-D'; the case fails by name (P-D, the in-order form, already fails e2).
- **TE-5 (the pump's size check untested; P-C passes): fix.** Q-5's case committed (an `Input` datagram of
  `Limits::INPUT + 1` B: `too_large` 1, nothing passed on), and one control-stream message one byte over its channel's
  cap: `too_large` 1 and the session closed as malformed (pump.rs:341-347). Plant: P-C; both fail by name. That is
  U4E-Q11 (a)'s "size ... and their unit tests".
- **TE-6 (carry 57's second-overflow kick cannot fail; P-57 passes): fix.** The test asserts that the kick's tick is
  below the first overflow resync's tick + `FULL_STATE_ACK_TICKS` + 1 (Q-9: 459 against 495), so the timeout cannot
  stand in for it. Plant: P-57; the test fails by name.
- **TE-7, TE-8:** ride the fix pass (R5). **TE-9:** carry 80 (U4f). **TE-10:** the report's words (R5).
- **Its settling runs hold** as it reports: NC-H1 (Q-1), NC-M3 (Q-2), SEC-U4E-1 (Q-3), SEC-U4E-3 (Q-4), SEC-U4E-4 (Q-6),
  SEC-U4E-7 (Q-7), NC-M5 (Q-10), NC-M4 in part (Q-11: the order yes, the figures no). Each becomes, or is the model
  for, the committed test named in R2, R3 and R5.
- **Its counted run** (r1-te-check-counted.log): all 10 steps PASS, measured 458, bites 524, self 52 of 52, equal to the
  builder's two (b11-counted1.log, b11-counted2.log). Its empty provenance block is a fact of check.mjs, which keeps
  cargo's Compiling lines out of its output; for a review seat's run the pwd line (the copy) and the touch before it are
  enough. The merge's counted run keeps the builder's form: separate `--no-run` builds whose Compiling lines name the
  main folder (M5).

R6 written 2026-11-12.

### R7. The main session's points (the card, "Running")
- **Carry 68 missing from the builder's carries met, though carry 56's ES figure is reported: CONFIRMED** (NC-M2 is the
  same point). Route: R2, T5's own-state bullet; carry 68 then counts as met.
- **Carry 61's stale-group choice ("decode within the ring, drop and count beyond it"): CONFIRMED, with R1's
  correction.** Within the ring: decode (NC-L4). Beyond it, two cases: a tick from the client's first snapshot on was
  already resolved when it left the ring, so its group is dropped and counted (`stale`); a tick before the first
  snapshot is a bad header and gets the sentinel (R1, case 3). B5's extra reading does not apply (R4).
- **The builder's own points:**
  - e7's split and resend clauses: CONFIRMED as carried by e2 (my brief check's A2) and by gap.rs's clean run (R4,
    NC-L3).
  - -18's baseline still in memory: condition 6 is not triggered (R4, NC-L1).
  - Owner-to-all writes the next slot's records into each client's own-state section: CONFIRMED. That is my brief
    check's A3 ("one other slot's own-state records ... within the section's 8-bit count ... so that every bitten
    snapshot still encodes"), and s15 fails carry 53's per-record test by name ("tick 1050: client 0's full state holds
    Soldier 1 gen 0").
  - Carry 26's copy: raised now and accepted (R4, A7).
  - Its proposed series: `event.snap_refused.tick_count` for carry 54 ACCEPTED (R4, A14; row T4);
    `tick.s.snap_too_old_count` for carry 61 NOT added (R4, NC-L4).
  - `Kick(replaced)` left to the caller: ACCEPTED for U4e (the room's doc says so: room.rs:601-606), with the caller's
    rule and its tests carried to U4r and U6 (R3: carries 76 and C-U6-14).
- **Whether the resumed test seat counts with the other four as one round: YES.** A round is one review of one head by
  all the change-set's reviewers, in batches of at most three (CLAUDE.md's review budget; my role file's "Review
  budget"). The head did not change across the cut: the main session restored plant P-D by Edit (client/src/lib.rs
  sha256 = start; the copy's crates/ equal .gate/u4e-r1's; the re-cut patch equal to wip/u4e.patch), the resumed seat
  found 23 of 23 files equal to its start list, recorded its two earlier plants' diffs afresh and re-ran them in its own
  form, and ended at 23 of 23; I re-checked both copies now (23 of 23 each). The cut is a process event, not a second
  review. Round 1 is complete; round 2 on what changed is the last ordinary round, and a third runs only for a Critical
  or High still open after it.

R7 written 2026-11-12.

### R8. The fix pass: its scope, its builder, and round 2's seats. APPROVE on these terms.
**Scope (each item with its test and its named plant; nothing else):**
- FX-1 R1 (NC-H1, TE-1): the client's case 3 and the tick16 twin's assertions; plants (a) and (b).
- FX-2 NC-M3; FX-3 A1 (with the lab's send in server/tests/common/mod.rs); FX-4 A2 (`RttSource`); FX-5 A4; FX-6 A5 (the
  eight items pub(crate), `with_cover`'s test); FX-7 SEC-U4E-4's room half (the Resync limit). All as R2 and R3 say.
- FX-8 TE-2, FX-9 TE-3, FX-10 TE-4, FX-11 TE-5, FX-12 TE-6, as R6 says.
- FX-13 the riders: SEC-U4E-1, SEC-U4E-3, SEC-U4E-7, TE-7, TE-8 with RR-2, RR-3, RR-10, as R5 says.
- Not the builder's: the report, marks/, the plan files and the carries (the main session's, from T1-T6).
- **Bounds:** files only under crates/room, crates/server (not schedule.rs) and crates/client; no manifest, no
  Cargo.lock, no npt-proto, npt-sim, npt-link or npt-lab file. One new public item, `RttSource`, and `take_control`'s
  changed type; any other public item, any change to the window's arithmetic, or a test that cannot be made to fail
  under its plant stops that item: the builder reports it, and it comes to me.

**Who builds:** one builder (general-purpose, Opus, effort xhigh), in the same worktree
(`.claude/worktrees/wf_0fa6595a-3e2-1`, uncommitted on 97bb9d3), reading its record (u4e-build.md), this ruling's R1-R6
and the copy's diff first, as U4d-F's fix pass did; under W1-TARGET's lock (the lane lock held for each cargo run, the
shared target, offline), counted in the three-agent limit. Its brief carries, word for word, my U4d-F ruling's T4:
item 1 (the write rule: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes;
never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole
with Write.") and item 2 with the crates named (npt-room, npt-server and npt-client: "a failure in npt-room,
npt-server or npt-client, or in a test that reads their files, stops your step group (report it), unless your step's
plan names it as expected …"). Its plan is written first into the record's new section "Fix pass, round 1", then a
notes line after each FX.

**Its evidence** (in the record): for each FX the plant's diff, the run (`-x`, by name), the test FAILED by name, the
restore by Edit and the file's sha256 equal to its fixed form; s03/s04 re-run once (R1 plant (b)); fmt (rustfmt's diff
applied by Edit), clippy in both builds and for wasm32, doc; check.mjs twice counted in the copy, all 10 steps, with
the counts as deltas on 458 / 524 / 52 and each new test named; the printed lines of e7, the tick16 twin and its clean
test, and carry 56's test (for T5); and at the end the sha256 of every changed or new file under crates/, in
`sha256sum` form with paths from the copy's root, saved as .gate/u4e/fx-hashes-end.sha256.

**Then the main session:** re-cuts the snapshot by worktree_patch.py (wip/u4e.patch replaced, committed), checks that
the copy's files equal fx-hashes-end.sha256, and cuts round 2's packet by review_packet.py on what changed (round 1's
snapshot at db49108 against the new one), with this ruling as the "judged against" half.

**Round 2's seats, on what changed** (Opus at xhigh; condition 10 (c)'s limit: two batches; each brief carries T4
item 1):
- Batch 1, read only: **netcode-architect** (R1's cases and the twin; NC-M3; A2; the Resync limit; SEC-U4E-1),
  **security-auditor** (the pump's changes: NC-M3, A2, SEC-U4E-1 and -3, TE-5; the client's decode path: R1's case 3
  and SEC-U4E-7; the Resync limit; A1's bytes), **engine-architect** (A1, A2, A4, A5, and T6's §4.1 rows against the
  final diff).
- Batch 2: **test-engineer** (with Edit, in the copy: each FX's test fails under its plant, its own form where it doubts
  the builder's; its start hashes first, as .gate/u4e/r2-hashes-start.sha256, then one counted check.mjs), beside the
  **rust-reviewer** (read only, no bench: panics, casts and overflow in the changed code, R1's bounded memory, RR-2 and
  RR-3).
- **Round 2's outcome:** APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead on M1-M9 (its Lows
  become follow-ups the main session records). A Medium comes to me first and becomes a follow-up or a carry, never a
  third round. A High or a Critical comes to me for a third round on it alone (the review budget's one exception); one
  still open after that stops U4e for Mustafa.

R8 written 2026-11-12.

````

### The advisor's round-1 ruling, T1: marks/README.md §1's case-3 sentence, which the client's rule must match (advisor-u4e-r1.md)
_`.checkpoints/netcode-proof-test/advisor-u4e-r1.md` lines 497-502 (sha256 7c1293097039), as they are:_

````text
### T1. marks/README.md §1, the `decoded_hash` item (the merge commit)
The old text `that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel.` (found once, line breaks
read as spaces; checked at the branch head) becomes the text below, rewrapped to the item's indent within 120:
```
that tick is lost (`null`, `tick.s.snap_incomplete` 1), never the sentinel. That covers the ticks from the client's first snapshot on: a snapshot group or full state whose header names a tick before the client's first snapshot and at least `npt_sim::ACK_RING_TICKS` ticks behind the newest tick it holds names a tick it was never sent (a bad header), so the client files the sentinel under that tick, once, never acknowledged, and `decoded_hash_mismatch_count` counts it as a client hash at a tick the server never logged (the advisor's U4e round-1 ruling, R1).
```
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
