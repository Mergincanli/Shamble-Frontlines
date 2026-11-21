# Review packet: U4r round 1 (the L1 runner and its judged runs' scripts: npt-lab's `scenario` with `--record`, `replay`, logs, mirror, contract checker and -30 pre-check, and its `hash` changes; npt-bots' `Behaviour`, clippy.toml and tests; npt-log, npt-server's pump and schedule; the recordings manifests; check.mjs's wasm-step options; ops/runs' c-u4r-r2.mjs, c-u4r-runs.mjs and det-u4.mjs; marks/SERIES.md's new rows; the §U4r carries and the carries the four build rulings name)

_Cut by `review_packet.py` from `u4r-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- The build's `<base>`: 614823c (P6) on the spike branch claude/netcode-proof-test-plan-hcky5g. Its spikes/ equals 4e91830's (L3, U4R-Q1 (b): `git diff --stat 4e91830 614823c -- spikes/` is empty).
- Counts at `<base>`, from L3's counted run at 4e91830 (`.gate/l3-manifests/counted.log`, HEAD 4e91830): check.mjs all 11 steps pass; measured build 491 tests passed, bites build 561, self 71 of 71.
- U4r's builder worktree: `.claude/worktrees/agent-a11e2354169f39acf` (branch worktree-agent-a11e2354169f39acf, HEAD 2fbec77). 2fbec77 is later than `<base>` by .checkpoints/ commits only (`git diff --stat 614823c 2fbec77` names two files, both under .checkpoints/). The change is uncommitted there: 26 entries under spikes/netcode-proof (13 modified, 13 new).
- The same change as a patch: `.checkpoints/netcode-proof-test/wip/u4r.patch` (26 files; sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2; committed at b084998).
- The read copy `.gate/u4r-snap1`: a detached worktree at 2fbec77 with that patch applied. `git status --porcelain --untracked-files=all` there lists the 26 files, and each of them equals the builder worktree's copy by cmp.
- The diffs below compare the main folder's copy of each file (main folder HEAD b084998; its spikes/ equals 2fbec77's: `git diff --stat 2fbec77 HEAD -- spikes/` and `git status --porcelain -- spikes/` are both empty) with the read copy's; a new file is compared with /dev/null. The tests' diffs (check.test.mjs, crates/bots/tests/behaviour.rs, crates/server/tests/schedule.rs) are in the side file `.checkpoints/netcode-proof-test/u4r-packet-tests.md`.
- The build's two end checks: `.gate/u4r/c5-end1.log` and `.gate/u4r/c5-end2.log` (`node check.mjs` in the builder worktree at HEAD 2fbec77, through the lane's runner `.checkpoints/netcode-proof-test/u4r-run.sh`; exit 0 each): all 11 steps pass; measured build 526 tests passed, bites build 603, self 76 of 76.
- The build's other logs and run folders are under `.gate/u4r/`; among them det-u4.log, d13.log, rec.log, regen-all.log, regen-precheck.log, b11-r4r8r9.log and the folders smoke1/ and whole-as-lost/, which the rulings below cite.
- The main folder's marks/marks.json, marks/SERIES.md and docs/design/netcode-proof-messages.md, copied below by line range, equal `<base>`'s.

## What changed (the diffs)

### npt-lab: scenario.rs (new: `SimClock`, the L1 scenario and the `scenario` subcommand's settings, `--bite` and `--record` among them)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs`:  1 file changed, 2304 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs
new file mode 100644
index 0000000..460a3c9
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs
@@ -0,0 +1,2304 @@
+//! `L1Scenario` (u4-build-plan.md §5 U4r; plan.md §4.7, RQ8; u4r-brief.md 2.1): one server and its clients on one
+//! simulated clock, [`SimClock`] (integer ns), in RQ8's order at each instant:
+//! 1. packets due: L1 applies every event due at or before the instant at the instant's first call (here the
+//!    listener's `poll_accept`, which also hands out each new link's server end), and never delivers a packet in the
+//!    instant it was sent, P0 included (l1/src/lib.rs:65-70);
+//! 2. the server's I/O step: per session `Decoder::pump`, then the room's `join` (after `Ok`, `Decoder::accept`; on
+//!    `Err`, `Decoder::reject`), the pump called again after a `Join` (it returns after one), `receive` for every other
+//!    message of a joined session, and `pump_report` with the bytes the pump sent;
+//! 3. the server tick if due, through U4b's `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)` (carry 6), then
+//!    each session's datagrams and reliable messages onto its link, each reliable message on the channel its type's
+//!    row of `Limits::TYPES` names;
+//! 4. the clients' 60 Hz frames: no work until U4g's `Interp`, so the runner adds no instant for them;
+//! 5. each client whose next step is due, in slot order: what its link delivered since its last step (polled into its
+//!    inbox at every instant: the socket's buffer), its `Behaviour::step`, then `ClientEndpoint::step`, whose datagrams
+//!    and reliable messages go onto its link.
+//!
+//! The runner wakes at the next of: a tick due, a client's step due, the net's next internal event
+//! (`L1Net::next_event_ns`, L-c5). Each client's host clock is the SimClock plus a seeded offset (rate 1: the 1.5 rate
+//! is U4f's cheat), and each arrival is stamped on it. L1's RTT is its own RFC 9002 estimate.
+//!
+//! **Session handling** (carry 76): when `Room::join` reports a replacement, the lab sends the old session
+//! `Kick(replaced)` and drops its `Decoder` and link before it routes anything else; when the room kicks a client or
+//! takes its `Leave`, the lab drops that session. **Reliable messages' channel:** each is decoded on both stream
+//! channels with the `Quant` it was coded with, and exactly one must take it (never a silent fall-back to Control).
+//! **Tokens** are seeded per slot and never logged.
+
+use std::collections::BTreeMap;
+use std::io::Write;
+use std::path::PathBuf;
+
+use npt_bots::Behaviour;
+use npt_client::ClientEndpoint;
+use npt_l1::{L1Net, Profile};
+use npt_link::{Channel, Link};
+use npt_log::{Ended, EventRecord, Phase};
+use npt_proto::{Direction, Join, Kick, Message, Quant};
+use npt_room::Room;
+use npt_server::{Decoder, RttSource, Schedule, due_ticks};
+use npt_sim::replay::Recording;
+use npt_sim::{Placement, Scene, StandIns, World};
+use serde_json::Value;
+
+use crate::hash::Failure;
+use crate::logs::{Logs, Meta};
+use crate::mirror::{Mirror, Records, SetStats};
+
+/// The protocol version and content hash the lab's room and clients share.
+pub(crate) const VERSION: u16 = 1;
+pub(crate) const CONTENT: [u8; 32] = *b"npt-lab scenario content hash v1";
+/// Each L1 link's datagram limit: QUIC's smallest maximum datagram size (RFC 9000 §14; the room's groups fit it).
+pub(crate) const MAX_DATAGRAM: usize = 1_200;
+/// One second, ns.
+pub(crate) const SEC: u64 = 1_000_000_000;
+/// The span of the room's hit-volume ring, ms: a copy of npt-room's private `HISTORY_MS` (room/src/room.rs:33,
+/// provisional until U4h), which a runner recording's header records (carry 14's U4r half).
+pub(crate) const ROOM_HISTORY_MS: u32 = 2_000;
+/// Carry 75's bound: every client is admitted before the room has stamped this many events past its start.
+pub(crate) const ADMIT_EVENTS_BOUND: u64 = 32_768;
+/// The highest feet a scripted spawn or teleport may set, cm: World's stated clamp bound (TUNE-2; world.rs:653-660),
+/// the position grid's top less the fire origin's 140 cm above the feet, read from npt-sim's grid.
+pub(crate) const PLACEMENT_Z_BOUND_CM: i32 = npt_sim::grid::POSITION_Z.max() - 140;
+
+/// The simulated clock (plan.md:617): integer ns, which only moves forward.
+#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
+pub(crate) struct SimClock {
+    now_ns: u64,
+}
+
+impl SimClock {
+    pub(crate) fn now(self) -> u64 {
+        self.now_ns
+    }
+
+    /// Moves to `t`, which is never earlier.
+    pub(crate) fn advance_to(&mut self, t: u64) {
+        assert!(t >= self.now_ns, "the SimClock never goes back");
+        self.now_ns = t;
+    }
+}
+
+/// splitmix64's step: the lab's seeded draws (offsets, tokens, burst offsets).
+pub(crate) fn mix(state: &mut u64) -> u64 {
+    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
+    let mut z = *state;
+    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
+    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
+    z ^ (z >> 31)
+}
+
+/// A uniform integer in `[0, n)`.
+pub(crate) fn below(word: u64, n: u64) -> u64 {
+    ((u128::from(word) * u128::from(n)) >> 64) as u64
+}
+
+fn sixteen(state: &mut u64) -> [u8; 16] {
+    let mut out = [0u8; 16];
+    out[..8].copy_from_slice(&mix(state).to_le_bytes());
+    out[8..].copy_from_slice(&mix(state).to_le_bytes());
+    out
+}
+
+/// Slot `slot`'s token and bind value, seeded (minting is U6's: plan :716).
+pub(crate) fn credentials(seed: u64, slot: u8) -> ([u8; 16], [u8; 16]) {
+    let mut s = seed ^ 0x746f_6b65_6e73_0000 ^ (u64::from(slot) << 40);
+    (sixteen(&mut s), sixteen(&mut s))
+}
+
+/// The join a client of slot `slot` sends.
+pub(crate) fn join_of(seed: u64, slot: u8) -> Join {
+    let (token, bind) = credentials(seed, slot);
+    Join {
+        token,
+        bind,
+        version: VERSION,
+        content_hash: CONTENT,
+    }
+}
+
+/// The lead of a client on `profile` (carry 74 as U4R-Q14 amended it): the round trip plus the jitter in ticks,
+/// rounded up, at least 1, plus 1: P0 2, P1 3, P2 and B1 5, P3 7.
+pub(crate) fn lead_for(profile: &Profile) -> u32 {
+    let ms = (profile.rtt_ns + profile.jitter_ns) / 1_000_000;
+    npt_sim::ticks_for_ms(u32::try_from(ms).unwrap_or(u32::MAX)).max(1) + 1
+}
+
+/// When the room's tick `k` ticks after its first is due on the SimClock (U4b's schedule, base 0).
+pub(crate) fn due_ns(k: u64) -> u64 {
+    (u128::from(k) * u128::from(SEC) / u128::from(npt_sim::TICK_HZ)) as u64
+}
+
+/// The channel a reliable message the room encoded travels on: decoded on both stream channels with `quant`, the
+/// one that takes it (the module text); an error when none does or both do.
+pub(crate) fn stream_of(bytes: &[u8], quant: Quant) -> Result<Channel, String> {
+    let ok = |ch| quant.decode(Direction::RoomToClient, ch, bytes).is_ok();
+    match (ok(Channel::Control), ok(Channel::FullState)) {
+        (true, false) => Ok(Channel::Control),
+        (false, true) => Ok(Channel::FullState),
+        (a, b) => Err(format!(
+            "a reliable message of {} B decodes on Control {a} and on FullState {b}: its channel is not known",
+            bytes.len()
+        )),
+    }
+}
+
+/// Carry 49 (TUNE-2): a scripted spawn or teleport above World's clamp bound is refused as a scenario error.
+pub(crate) fn check_placements(
+    spawns: &[Placement],
+    teleports: &[Placement],
+) -> Result<(), String> {
+    for (what, p) in spawns
+        .iter()
+        .map(|p| ("spawn", p))
+        .chain(teleports.iter().map(|p| ("teleport", p)))
+    {
+        if p.position[2] > PLACEMENT_Z_BOUND_CM {
+            return Err(format!(
+                "a scripted {what} of slot {} to z {} cm is above World's clamp bound {} cm (TUNE-2): refused",
+                p.slot, p.position[2], PLACEMENT_Z_BOUND_CM
+            ));
+        }
+    }
+    Ok(())
+}
+
+/// The planted defects of one run, by the instance each goes to (u4r-brief.md 2.1: the `--bite` dispatch).
+#[derive(Clone, Debug, Default)]
+#[cfg_attr(
+    not(feature = "bites"),
+    allow(dead_code, reason = "a measured build reads no bite")
+)]
+pub(crate) struct Bites {
+    pub(crate) id: Option<String>,
+    pub(crate) world: bool,
+    pub(crate) room: bool,
+    pub(crate) client: bool,
+    pub(crate) decoder: bool,
+    pub(crate) l1: bool,
+    pub(crate) schedule: bool,
+    /// npt-server's pump (each session's `Decoder::with_bite`: PM-0001-30/wallclock, U4R-Q6 (a)).
+    pub(crate) pump: bool,
+    /// npt-lab's own: carry 62 (h)'s client writer, PM-0001-17/whole-as-lost (the advisor's U4r build ruling, point 4).
+    pub(crate) whole_as_lost: bool,
+    /// npt-lab's own: the -30 pre-check's late-command replay client (one leg's, one client's: precheck.rs).
+    pub(crate) late_command: bool,
+    /// npt-lab's model bite, chosen by its unit tests only (r1): the clients connect in a hash map's order.
+    #[cfg_attr(
+        not(all(test, feature = "bites")),
+        allow(dead_code, reason = "only the bites build's unit tests choose it")
+    )]
+    pub(crate) client_order: bool,
+}
+
+/// npt-lab's own bite ids (bites builds only): carry 62 (h)'s client writer (U4R-Q2 (c); its form and id the advisor's
+/// U4r build ruling, point 4: "a client writer that files a tick it decoded whole as lost: null and incomplete 1"), and
+/// the -30 pre-check's late-command replay client (u4r-brief.md 2.5, 7.2; marks.json:6567).
+#[cfg(feature = "bites")]
+pub(crate) const LAB_BITES: [&str; 2] = ["PM-0001-17/whole-as-lost", "PM-0001-30/late-command"];
+/// npt-lab's model bite (r1's), chosen only by its unit tests (l1's `with_model_bite` form).
+#[cfg(all(test, feature = "bites"))]
+pub(crate) const CLIENT_ORDER_BITE: &str = "npt-lab/client-order";
+
+impl Bites {
+    /// Each instance whose crate's list holds `id` (an id no instance knows is a usage error).
+    #[cfg(feature = "bites")]
+    pub(crate) fn of(id: &str) -> Result<Bites, String> {
+        let proto = npt_proto::BITES.contains(&id);
+        let b = Bites {
+            id: Some(id.to_owned()),
+            world: World::BITES.contains(&id),
+            room: npt_room::BITES.contains(&id) || proto,
+            client: npt_client::BITES.contains(&id) || proto,
+            decoder: proto,
+            l1: L1Net::BITES.contains(&id),
+            schedule: npt_server::BITES.contains(&id),
+            pump: Decoder::new(
+                Quant::default(),
+                npt_sim::TICK_HZ,
+                0,
+                VERSION,
+                CONTENT,
+                RttSource::Link,
+            )
+            .with_bite(id)
+            .is_some(),
+            whole_as_lost: LAB_BITES[0] == id,
+            late_command: LAB_BITES[1] == id,
+            client_order: false,
+        };
+        let any = b.world
+            || b.room
+            || b.client
+            || b.l1
+            || b.schedule
+            || b.pump
+            || b.whole_as_lost
+            || b.late_command;
+        if !any {
+            return Err(format!(
+                "--bite {id:?}: no instance of this build plants it"
+            ));
+        }
+        Ok(b)
+    }
+
+    #[cfg(not(feature = "bites"))]
+    pub(crate) fn of(id: &str) -> Result<Bites, String> {
+        Err(format!(
+            "--bite {id}: this is a measured build, which holds no bites (build npt-lab with --features bites)"
+        ))
+    }
+
+    /// The `Quant` the decoders and the clients code the wire with.
+    pub(crate) fn quant(&self) -> Quant {
+        #[cfg(feature = "bites")]
+        if self.decoder
+            && let Some(q) = self.id.as_deref().and_then(Quant::with_bite)
+        {
+            return q;
+        }
+        Quant::default()
+    }
+
+    fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> World {
+        #[cfg(feature = "bites")]
+        if self.world
+            && let Some(w) = self
+                .id
+                .as_deref()
+                .and_then(|id| World::with_bite(scene, seed, first_tick, id, arm_tick))
+        {
+            return w;
+        }
+        let _ = arm_tick;
+        World::new(scene, seed, first_tick)
+    }
+
+    fn room(&self, room: Room<World>) -> Room<World> {
+        #[cfg(feature = "bites")]
+        if self.room
+            && let Some(id) = self.id.as_deref()
+        {
+            return room
+                .with_bite(id)
+                .expect("the room plants its own and npt-proto's ids");
+        }
+        room
+    }
+
+    pub(crate) fn client(&self, e: ClientEndpoint) -> ClientEndpoint {
+        #[cfg(feature = "bites")]
+        if self.client
+            && let Some(id) = self.id.as_deref()
+        {
+            return e
+                .with_bite(id)
+                .expect("the client plants its own and npt-proto's ids");
+        }
+        e
+    }
+
+    fn net(&self, seed: u64) -> L1Net {
+        #[cfg(feature = "bites")]
+        if self.l1
+            && let Some(net) = self.id.as_deref().and_then(|id| L1Net::with_bite(seed, id))
+        {
+            return net;
+        }
+        L1Net::new(seed)
+    }
+
+    fn schedule(&self) -> Schedule {
+        #[cfg(feature = "bites")]
+        if self.schedule
+            && let Some(s) = self
+                .id
+                .as_deref()
+                .and_then(|id| Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id))
+        {
+            return s;
+        }
+        Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)
+    }
+}
+
+/// One server-side session: its link, its decoder and, once joined, its slot.
+pub(crate) struct Session {
+    pub(crate) id: u64,
+    pub(crate) link: Box<dyn Link>,
+    pub(crate) decoder: Decoder,
+    pub(crate) slot: Option<u8>,
+    /// Messages this session's pump handed the room (r8 and r9 read them).
+    pub(crate) routed: u64,
+    /// The RTT rule's records (u4r-brief.md 7.2): the first pump instant after which `rtt_ns` is `Some`, and the
+    /// first pump instant that decoded an `Input` with a command.
+    pub(crate) first_sample_ns: Option<u64>,
+    pub(crate) first_input_ns: Option<u64>,
+}
+
+/// The server half (the room behind one decoder per session), the same on L1 and on Loopback.
+pub(crate) struct Server {
+    pub(crate) room: Room<World>,
+    pub(crate) sessions: Vec<Option<Session>>,
+    quant: Quant,
+    rtt: RttSource,
+    next_id: u64,
+    /// Each admission since the last take: (slot, session, tick it runs before).
+    pub(crate) admitted: Vec<(u8, u64, u32)>,
+    /// Each session ended since the last take: (slot, why).
+    pub(crate) ended: Vec<(u8, &'static str)>,
+    /// The sessions the room's slots are held by (r8 reads which).
+    pub(crate) owner: BTreeMap<u8, u64>,
+    /// The pump's planted defect every session's decoder takes (`Bites::pump`; bites builds only).
+    #[cfg_attr(
+        not(feature = "bites"),
+        allow(dead_code, reason = "a measured build plants no bite")
+    )]
+    pub(crate) decoder_bite: Option<String>,
+}
+
+impl Server {
+    pub(crate) fn new(room: Room<World>, quant: Quant, rtt: RttSource) -> Server {
+        Server {
+            room,
+            sessions: Vec::new(),
+            quant,
+            rtt,
+            next_id: 0,
+            admitted: Vec::new(),
+            ended: Vec::new(),
+            owner: BTreeMap::new(),
+            decoder_bite: None,
+        }
+    }
+
+    /// A new link's server end, accepted at `now`.
+    pub(crate) fn accept(&mut self, link: Box<dyn Link>, now: u64) -> u64 {
+        let id = self.next_id;
+        self.next_id += 1;
+        let decoder = Decoder::new(
+            self.quant,
+            npt_sim::TICK_HZ,
+            now,
+            VERSION,
+            CONTENT,
+            self.rtt,
+        );
+        #[cfg(feature = "bites")]
+        let decoder = match self.decoder_bite.as_deref() {
+            Some(id) => decoder
+                .with_bite(id)
+                .expect("Bites::pump checked that the pump plants it"),
+            None => decoder,
+        };
+        self.sessions.push(Some(Session {
+            id,
+            link,
+            decoder,
+            slot: None,
+            routed: 0,
+            first_sample_ns: None,
+            first_input_ns: None,
+        }));
+        id
+    }
+
+    fn drop_slot(&mut self, slot: u8, keep: u64, why: &'static str, now: u64) {
+        for s in &mut self.sessions {
+            let Some(x) = s else { continue };
+            if x.slot == Some(slot) && x.id != keep {
+                if why == "replaced" {
+                    let mut out = Vec::new();
+                    if npt_proto::encode(&Message::Kick(Kick::Replaced), &mut out).is_ok() {
+                        x.link.send_reliable(Channel::Control, &out, now);
+                    }
+                }
+                *s = None;
+                self.ended.push((slot, why));
+            }
+        }
+    }
+
+    /// RQ8's step 2 at `now` (the module text).
+    pub(crate) fn io(&mut self, now: u64) {
+        let tick = self.room.last_tick().wrapping_add(1);
+        for i in 0..self.sessions.len() {
+            while let Some(x) = self.sessions[i].as_mut() {
+                let mut msgs = Vec::new();
+                x.decoder.pump(x.link.as_mut(), now, tick, &mut msgs);
+                if x.first_sample_ns.is_none() && x.decoder.rtt_ns(x.link.as_ref()).is_some() {
+                    x.first_sample_ns = Some(now);
+                }
+                let mut joined = false;
+                let mut replaced = None;
+                let mut left = false;
+                for (m, _) in msgs {
+                    let x = self.sessions[i].as_mut().expect("present");
+                    match (&m, x.slot) {
+                        (Message::Join(j), _) => {
+                            joined = true;
+                            match self.room.join(j) {
+                                Ok((slot, r)) => {
+                                    x.decoder.accept();
+                                    x.slot = Some(slot);
+                                    self.admitted.push((slot, x.id, tick));
+                                    self.owner.insert(slot, x.id);
+                                    if r {
+                                        replaced = Some((slot, x.id));
+                                    }
+                                }
+                                Err(r) => x.decoder.reject(x.link.as_mut(), r, now),
+                            }
+                        }
+                        (_, Some(slot)) => {
+                            if let Message::Input(inp) = &m
+                                && !inp.commands.is_empty()
+                                && x.first_input_ns.is_none()
+                            {
+                                x.first_input_ns = Some(now);
+                            }
+                            x.routed += 1;
+                            left = matches!(m, Message::Leave(_));
+                            self.room.receive(slot, &m);
+                            if left {
+                                break;
+                            }
+                        }
+                        _ => {}
+                    }
+                }
+                if let Some((slot, keep)) = replaced {
+                    // Carry 76: the old session is told and dropped before anything else is routed.
+                    self.drop_slot(slot, keep, "replaced", now);
+                }
+                if left {
+                    let slot = self.sessions[i].as_ref().and_then(|x| x.slot);
+                    self.sessions[i] = None;
+                    if let Some(slot) = slot {
+                        self.owner.remove(&slot);
+                        self.ended.push((slot, "left"));
+                    }
+                    break;
+                }
+                if !joined {
+                    break;
+                }
+            }
+            let Some(x) = self.sessions[i].as_mut() else {
+                continue;
+            };
+            if x.decoder.is_closed() {
+                let slot = x.slot;
+                self.sessions[i] = None;
+                if let Some(slot) = slot {
+                    self.ended.push((slot, "closed"));
+                }
+                continue;
+            }
+            if let Some(slot) = x.slot {
+                let sent = x.decoder.take_sent_bytes();
+                self.room
+                    .pump_report(slot, sent, x.link.max_datagram_size());
+            }
+        }
+    }
+
+    /// The session holding `slot`, if any.
+    pub(crate) fn session(&mut self, slot: u8) -> Option<&mut Session> {
+        self.sessions
+            .iter_mut()
+            .flatten()
+            .find(|x| x.slot == Some(slot))
+    }
+
+    /// Sends `slot`'s tick output (RQ8's step 3, after the tick) on its session's link, then drops the session if the
+    /// room kicked it (carry 76).
+    pub(crate) fn send(
+        &mut self,
+        slot: u8,
+        datagrams: &[Vec<u8>],
+        control: &[Vec<u8>],
+        now: u64,
+    ) -> Result<(), String> {
+        let quant = self.quant;
+        let kicked = self.room.client(slot).is_some_and(|c| c.kicked());
+        let Some(x) = self.session(slot) else {
+            return Ok(());
+        };
+        for d in datagrams {
+            x.link.send_unreliable(d, now);
+        }
+        for b in control {
+            x.link.send_reliable(stream_of(b, quant)?, b, now);
+        }
+        if kicked {
+            self.sessions
+                .iter_mut()
+                .filter(|s| s.as_ref().is_some_and(|x| x.slot == Some(slot)))
+                .for_each(|s| *s = None);
+            self.owner.remove(&slot);
+            self.ended.push((slot, "kicked"));
+        }
+        Ok(())
+    }
+}
+
+/// One run's settings (the `scenario` subcommand's options).
+#[derive(Clone, Debug)]
+pub(crate) struct Settings {
+    pub(crate) meta: Meta,
+    pub(crate) out: PathBuf,
+    pub(crate) scene: Scene,
+    pub(crate) profile: Profile,
+    pub(crate) clients: u8,
+    pub(crate) seed: u64,
+    pub(crate) tick_start: u32,
+    pub(crate) event_start: u32,
+    pub(crate) cmd_start: u32,
+    pub(crate) warmup_ticks: u64,
+    pub(crate) measured_ticks: u64,
+    pub(crate) after_max_ticks: u64,
+    /// B1's scheduled interval between burst starts, ns, before each burst's offset (`--burst-every-s`); 0 on another
+    /// profile.
+    pub(crate) burst_spacing_ns: u64,
+    pub(crate) behaviour: String,
+    pub(crate) every_step: bool,
+    pub(crate) bites: Bites,
+    pub(crate) arm_tick: u32,
+    pub(crate) lead: u32,
+    pub(crate) record: Option<PathBuf>,
+    pub(crate) bytes: Option<PathBuf>,
+    /// Whether the run writes its logs (`false`: a runner recording regenerated inside `npt-lab hash`).
+    pub(crate) logs: bool,
+    /// Whether the run hands its recording back in memory (carry 82's regeneration).
+    pub(crate) record_mem: bool,
+}
+
+/// A runner recording made again (carry 82; u4r-brief.md 2.1 (ii)): `npt-lab scenario --record` at these settings, the
+/// brawl, no log kept, its bytes handed back.
+pub(crate) fn record(
+    scene: &Scene,
+    profile: &str,
+    seed: u64,
+    clients: u8,
+    first_tick: u32,
+    warmup_s: u64,
+    measured_s: u64,
+) -> Result<Vec<u8>, String> {
+    let line = format!(
+        "--run regenerate --set regenerate --date 2000-01-01 --commit {} --out - --scene {} --profile {profile} \
+         --seed {seed} --clients {clients} --tick-start {first_tick} --warmup-s {warmup_s} --measured-s {measured_s}",
+        "0".repeat(40),
+        scene.name()
+    );
+    let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+    let mut s = parse(&argv)?;
+    s.logs = false;
+    s.record_mem = true;
+    let o = run(&s).map_err(|f| match f {
+        Failure::Usage(m) | Failure::Failed(m) => m,
+    })?;
+    o.recording_bytes
+        .ok_or_else(|| "the scenario kept no recording".to_owned())
+}
+
+/// One client: its endpoint and link, its behaviour, its host clock's offset and its next step.
+struct Client {
+    slot: u8,
+    endpoint: ClientEndpoint,
+    link: Box<dyn Link>,
+    behaviour: Behaviour,
+    offset_ns: u64,
+    next_step_ns: u64,
+    inbox: Vec<(Channel, Vec<u8>, u64)>,
+    last_stamp: Option<u32>,
+    duplicates: u64,
+    bytes_log: Option<std::io::BufWriter<std::fs::File>>,
+}
+
+/// One burst of B1 for one client: (start, return), its heal, and what -18's record carries beside it.
+#[derive(Clone, Debug, Default)]
+struct Burst {
+    start_ns: u64,
+    end_ns: u64,
+    healed: bool,
+    full_state: bool,
+    queue_max: u32,
+    eviction_logged: bool,
+}
+
+/// What one run gives its caller: the summary line's numbers.
+#[derive(Clone, Debug, Default)]
+pub(crate) struct Summary {
+    pub(crate) ticks: u64,
+    pub(crate) after_ticks: u64,
+    pub(crate) fate_log_sha256: String,
+    pub(crate) world_hashes: Vec<(u32, u64)>,
+    pub(crate) commands: u64,
+    pub(crate) skipped: u64,
+    pub(crate) duplicates: u64,
+    pub(crate) late: u64,
+    pub(crate) press_refused: u64,
+    pub(crate) buffer_full: u64,
+    pub(crate) rewinds_refused: u64,
+    pub(crate) stale: u64,
+    pub(crate) overflow_resyncs: u64,
+    pub(crate) heals: u64,
+    pub(crate) audit: crate::mirror::Audit,
+    pub(crate) counts: crate::logs::Counts,
+    pub(crate) recording: Option<(usize, String)>,
+    pub(crate) recording_bytes: Option<Vec<u8>>,
+    pub(crate) order_digest: u64,
+    /// A bite run whose after phase reached its cap with measured ticks unresolved (carry 73).
+    pub(crate) after_capped: bool,
+}
+
+fn sha256_hex(bytes: &[u8]) -> String {
+    use sha2::{Digest, Sha256};
+    Sha256::digest(bytes)
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect()
+}
+
+fn fate_sha256(net: &L1Net) -> String {
+    use sha2::{Digest, Sha256};
+    net.with_fate_log(|log| {
+        let mut h = Sha256::new();
+        log.for_each_canonical(|b| h.update(b));
+        h.finalize().iter().map(|b| format!("{b:02x}")).collect()
+    })
+}
+
+/// The order the clients connect in: slot order, or under r1's model bite a hash map's (bites test builds only).
+fn connect_order(n: u8, b: &Bites) -> Vec<u8> {
+    #[cfg(all(test, feature = "bites"))]
+    if b.client_order {
+        return client_order_bitten(n);
+    }
+    let _ = b;
+    (0..n).collect()
+}
+
+/// r1's bite (marks F29): the clients connect in a `HashMap`'s iteration order, which a process's random state seeds.
+#[cfg(all(test, feature = "bites"))]
+#[allow(clippy::disallowed_types, clippy::iter_over_hash_type)]
+fn client_order_bitten(n: u8) -> Vec<u8> {
+    let slots: std::collections::HashMap<u8, ()> = (0..n).map(|s| (s, ())).collect();
+    slots.keys().copied().collect()
+}
+
+/// The digest of a connect order (each process logs it: equal digests mean the bite was not exercised).
+fn order_digest(order: &[u8]) -> u64 {
+    let mut s = 0x6f72_6465_7200_0000u64;
+    for o in order {
+        s ^= u64::from(*o);
+        s = mix(&mut s);
+    }
+    s
+}
+
+/// Runs one scenario (the module text) and writes its three logs into `s.out`.
+pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
+    let fail = Failure::Failed;
+    let scene = s.scene.clone().with_stand_ins(StandIns::OFF);
+    let n = s.clients.min(scene.slots());
+    let world = s.bites.world(&scene, s.seed, s.tick_start, s.arm_tick);
+    let greybox = world.greybox().clone();
+    let tokens = (0..scene.slots())
+        .map(|slot| credentials(s.seed, slot).0)
+        .collect();
+    let room = Room::new(
+        world,
+        &scene,
+        &greybox,
+        tokens,
+        usize::from(n),
+        npt_sim::TICK_HZ,
+    )
+    .with_hashes(CONTENT, [0; 32])
+    .with_event_start(s.event_start);
+    let room = s.bites.room(room);
+    let quant = s.bites.quant();
+    let mut server = Server::new(room, quant, RttSource::Link);
+    if s.bites.pump {
+        server.decoder_bite = s.bites.id.clone();
+    }
+    let mut schedule = s.bites.schedule();
+    let net = s.bites.net(s.seed);
+    let mut listener = net.listener();
+    if s.logs {
+        std::fs::create_dir_all(&s.out).map_err(|e| fail(format!("{}: {e}", s.out.display())))?;
+    }
+    let mut meta = s.meta.clone();
+    let (cap, provisional) = server.room.event_cap();
+    meta.params
+        .insert("event_queue_cap_count".into(), cap.into());
+    meta.params
+        .insert("event_queue_cap_provisional".into(), provisional.into());
+    let mut logs = Logs::open(
+        s.logs.then_some(s.out.as_path()),
+        &meta,
+        s.tick_start,
+        s.bites.whole_as_lost,
+    )
+    .map_err(fail)?;
+    let mut rng = s.seed ^ 0x6c61_622d_6f66_6673;
+    let order = connect_order(n, &s.bites);
+    let mut clients: Vec<Client> = Vec::new();
+    for &slot in &order {
+        let link = net.connect(s.profile.clone(), MAX_DATAGRAM, 0);
+        let endpoint = ClientEndpoint::new(join_of(s.seed, slot), npt_sim::TICK_HZ)
+            .with_lead(s.lead)
+            .with_starts(s.cmd_start, s.event_start);
+        let endpoint = s.bites.client(endpoint);
+        let behaviour = if s.behaviour == "brawl" {
+            Behaviour::brawl(&scene, slot, s.seed)
+        } else {
+            Behaviour::script(&scene, slot, s.seed, &s.behaviour)
+                .ok_or_else(|| Failure::Usage(format!("--behaviour {:?}", s.behaviour)))?
+        };
+        let behaviour = if s.every_step {
+            behaviour.every_step()
+        } else {
+            behaviour
+        };
+        let bytes_log = match &s.bytes {
+            None => None,
+            Some(dir) => {
+                std::fs::create_dir_all(dir)
+                    .map_err(|e| fail(format!("{}: {e}", dir.display())))?;
+                let p = dir.join(format!("client-{slot}.bytes"));
+                let f = std::fs::File::create_new(&p)
+                    .map_err(|e| fail(format!("{}: {e}", p.display())))?;
+                Some(std::io::BufWriter::new(f))
+            }
+        };
+        clients.push(Client {
+            slot,
+            endpoint,
+            link,
+            behaviour,
+            offset_ns: below(mix(&mut rng), 1_000 * SEC),
+            next_step_ns: 1 + below(mix(&mut rng), due_ns(1) - 1),
+            inbox: Vec::new(),
+            last_stamp: None,
+            duplicates: 0,
+            bytes_log,
+        });
+    }
+    clients.sort_by_key(|c| c.slot);
+    let digest = order_digest(&order);
+    let mut mirror = Mirror::new(scene.slots(), scene.effect_rows(), scene.name() == "ES");
+    // -57's three series only on a run whose header lists PM-0001-57 (the advisor's U4r build ruling, point 3): the
+    // judge reads a run for a mark only when a header lists it, so no other run needs them.
+    let with_57 = s.meta.marks.iter().any(|m| m == "PM-0001-57");
+    let mut bursts: Vec<Vec<Burst>> = (0..n)
+        .map(|_| {
+            s.profile
+                .bursts_ns
+                .iter()
+                .map(|&(start, len)| Burst {
+                    start_ns: start,
+                    end_ns: start + len,
+                    ..Burst::default()
+                })
+                .collect()
+        })
+        .collect();
+    let mut full_states: Vec<Vec<(u32, u64)>> = vec![Vec::new(); usize::from(n)];
+    let recording_on = s.record.is_some() || s.record_mem;
+    let mut recording = recording_on.then(|| Recording {
+        scene: scene.id(),
+        seed: s.seed,
+        first_tick: s.tick_start,
+        span_ms: ROOM_HISTORY_MS,
+        generator: 0,
+        ticks: Vec::new(),
+    });
+    let mut clock = SimClock::default();
+    let mut k: u64 = 0;
+    let measured_end = s.warmup_ticks + s.measured_ticks;
+    let last_measured = logs.wide(s.tick_start.wrapping_add((measured_end.max(1) - 1) as u32));
+    let mut events_stamped: u64 = 0;
+    let mut inputs: Vec<i64> = Vec::new();
+    let mut view = Vec::new();
+    let mut summary = Summary {
+        order_digest: digest,
+        ..Summary::default()
+    };
+    let phase = |k: u64| {
+        if k < s.warmup_ticks {
+            Phase::Warmup
+        } else if k < measured_end {
+            Phase::Measured
+        } else {
+            Phase::After
+        }
+    };
+    let mut stop = false;
+    while !stop {
+        let now = clock.now();
+        // (1) packets due, and each new link's server end.
+        while let Some((link, _)) = listener.poll_accept(now) {
+            server.accept(link, now);
+        }
+        // (2) the server's I/O step.
+        server.io(now);
+        for (slot, session, tick) in std::mem::take(&mut server.admitted) {
+            if events_stamped >= ADMIT_EVENTS_BOUND {
+                return Err(fail(format!(
+                    "slot {slot} admitted after {events_stamped} events past the event start (carry 75's bound \
+                     {ADMIT_EVENTS_BOUND}): comes to the advisor first"
+                )));
+            }
+            logs.admit(slot, session, tick, phase(k)).map_err(fail)?;
+        }
+        for (slot, _) in std::mem::take(&mut server.ended) {
+            logs.gone(slot);
+        }
+        // (3) the server tick, if due.
+        let due = due_ticks(now, &schedule);
+        schedule = due.next;
+        for _ in 0..due.to_run {
+            let ph = phase(k);
+            let t = server.room.last_tick().wrapping_add(1);
+            server.room.tick();
+            if let Some(r) = recording.as_mut() {
+                r.ticks.push(server.room.tick_input().clone());
+            }
+            let records = npt_sim::SimCore::entity_records(server.room.sim()).to_vec();
+            let index = Records::new(&records);
+            let mut sizes = vec![Vec::new(); usize::from(n)];
+            let mut sets = vec![SetStats::default(); usize::from(n)];
+            for slot in 0..n {
+                let datagrams = server.room.take_datagrams(slot);
+                let control = server.room.take_control(slot);
+                let acked = server.room.client(slot).and_then(|c| c.last_acked());
+                let errs = mirror.tick(slot, t, &datagrams, &control, quant, &index, acked);
+                if s.logs && with_57 {
+                    logs.errors(slot, t, errs);
+                }
+                for b in &control {
+                    if let Ok(Message::FullState(f)) =
+                        quant.decode(Direction::RoomToClient, Channel::FullState, b)
+                    {
+                        full_states[usize::from(slot)].push((f.tick, now));
+                        for burst in &mut bursts[usize::from(slot)] {
+                            if now >= burst.start_ns && !burst.healed {
+                                burst.full_state = true;
+                            }
+                        }
+                    }
+                }
+                sizes[usize::from(slot)] = datagrams.iter().map(Vec::len).collect();
+                sets[usize::from(slot)] = mirror.latest(slot);
+                server.send(slot, &datagrams, &control, now).map_err(fail)?;
+                if let Some(c) = server.room.client(slot) {
+                    for w in c.wraps() {
+                        logs.wrapped(w, t, ph).map_err(fail)?;
+                    }
+                    if c.overflow_resync() {
+                        summary.overflow_resyncs += 1;
+                        let mut e = EventRecord::new("overflow_resync", ph).with("slot", slot);
+                        e.t = Some(logs.wide(t));
+                        logs.lab_event(&e).map_err(fail)?;
+                    }
+                    for b in &mut bursts[usize::from(slot)] {
+                        if now >= b.start_ns && !b.healed {
+                            b.queue_max = b.queue_max.max(c.queue().0);
+                        }
+                        if now >= b.end_ns && !b.eviction_logged {
+                            b.eviction_logged = true;
+                            let i = bursts_index(&s.profile, b.start_ns);
+                            let mut e = EventRecord::new("burst_eviction", ph)
+                                .with("slot", slot)
+                                .with("burst", i)
+                                .with("evicted", i64::from(c.burst_eviction()))
+                                .with("baseline_age", c.baseline_age().map_or(-1, i64::from));
+                            e.t = Some(logs.wide(t));
+                            logs.room_event(&e).map_err(fail)?;
+                        }
+                    }
+                }
+            }
+            for (slot, _) in std::mem::take(&mut server.ended) {
+                logs.gone(slot);
+            }
+            if s.logs {
+                events_stamped += logs
+                    .room_tick(&server.room, t, ph, &sets, &sizes, &inputs)
+                    .map_err(fail)?;
+            }
+            inputs.clear();
+            summary
+                .world_hashes
+                .push((t, npt_sim::SimCore::state_hash(server.room.sim())));
+            k += 1;
+            if k == s.warmup_ticks {
+                for c in &clients {
+                    if !logs.started(c.slot) {
+                        return Err(fail(format!(
+                            "slot {} was not admitted, or resolved no tick, in the warm-up (carry 62 (d))",
+                            c.slot
+                        )));
+                    }
+                }
+            }
+        }
+        // (5) the clients whose step is due, in slot order.
+        for c in &mut clients {
+            if c.next_step_ns > now {
+                continue;
+            }
+            let host = now + c.offset_ns;
+            let mut received = std::mem::take(&mut c.inbox);
+            while let Some(r) = c.link.poll_recv(now) {
+                received.push((r.channel, r.bytes, r.arrival_ns + c.offset_ns));
+            }
+            if let Some(w) = c.bytes_log.as_mut() {
+                write_step(w, host, &received).map_err(fail)?;
+            }
+            c.endpoint.input(None, &received, host);
+            if let Some(newest) = c.endpoint.newest_tick() {
+                mirror.view(c.slot, newest, &mut view);
+            } else {
+                view.clear();
+            }
+            if c.behaviour.step(&mut c.endpoint, &[], host, &view) {
+                let stamp = c.endpoint.newest_tick().map(|t| t.wrapping_add(s.lead));
+                if stamp.is_some() && stamp == c.last_stamp {
+                    c.duplicates += 1;
+                }
+                c.last_stamp = stamp;
+            }
+            let out = c.endpoint.step();
+            for d in &out.datagrams {
+                if matches!(
+                    npt_proto::decode(Direction::ClientToRoom, Channel::Datagram, d),
+                    Ok(Message::Input(_))
+                ) {
+                    inputs.push(d.len() as i64);
+                }
+                c.link.send_unreliable(d, now);
+            }
+            for (ch, b) in &out.reliable {
+                c.link.send_reliable(*ch, b, now);
+            }
+            logs.filed(c.slot, &out.hashes, &out.lost);
+            for w in &out.wraps {
+                let t = c.endpoint.newest_tick().unwrap_or(s.tick_start);
+                logs.wrapped(w, t, phase(k)).map_err(fail)?;
+            }
+            for b in &mut bursts[usize::from(c.slot)] {
+                if b.healed || now < b.end_ns {
+                    continue;
+                }
+                let fs = &full_states[usize::from(c.slot)];
+                let fresh = out.hashes.iter().any(|&(tick, h)| {
+                    let k_of = u64::from(tick.wrapping_sub(s.tick_start));
+                    h != 0
+                        && due_ns(k_of) >= b.end_ns
+                        && !fs.iter().any(|&(f, at)| f == tick && at >= b.start_ns)
+                });
+                if fresh {
+                    b.healed = true;
+                    summary.heals += 1;
+                    let i = bursts_index(&s.profile, b.start_ns);
+                    let mut e = EventRecord::new("burst_heal", phase(k))
+                        .with("after_link_ns", now - b.end_ns)
+                        .with("slot", c.slot)
+                        .with("burst", i)
+                        .with("full_state", i64::from(b.full_state))
+                        .with("queue_max_count", b.queue_max);
+                    e.ns = Some(now);
+                    logs.lab_event(&e).map_err(fail)?;
+                }
+            }
+            c.next_step_ns = out.next_due_ns.saturating_sub(c.offset_ns).max(now + 1);
+        }
+        // Every client link's arrivals into its inbox (the socket's buffer), stamped on its host clock.
+        for c in &mut clients {
+            while let Some(r) = c.link.poll_recv(now) {
+                c.inbox
+                    .push((r.channel, r.bytes, r.arrival_ns + c.offset_ns));
+            }
+        }
+        logs.flush().map_err(fail)?;
+        // A recording run holds exactly the warm-up and the measured ticks: it stops at the measured phase's end.
+        if recording_on && k >= measured_end {
+            stop = true;
+            continue;
+        }
+        if k >= measured_end && logs.written_through(last_measured) {
+            summary.after_ticks = k - measured_end;
+            stop = true;
+            continue;
+        }
+        if k >= measured_end + s.after_max_ticks {
+            if s.bites.id.is_none() {
+                return Err(fail(format!(
+                    "the measured ticks were not all resolved within the after phase's {} ticks (carry 62 (e))",
+                    s.after_max_ticks
+                )));
+            }
+            // A bite run ends at the cap (carry 73: tick16 past the wrap files its rebuilds under ticks the room never
+            // ran, so the real ticks never resolve): the stop decides each client's unresolved ticks inside its span
+            // lost, the wrong rebuilds are written after every tick the room ran, and the logs end clean, so the judge
+            // reads the mismatch, never a cut log. A measured run still fails here.
+            summary.after_ticks = k - measured_end;
+            summary.after_capped = true;
+            stop = true;
+            continue;
+        }
+        // The next instant: a tick due, a client's step due, or the net's next internal event.
+        let mut next = schedule.next_due_ns();
+        for c in &clients {
+            next = next.min(c.next_step_ns);
+        }
+        if let Some(e) = net.next_event_ns() {
+            next = next.min(e.max(now + 1));
+        }
+        clock.advance_to(next.max(now + 1));
+    }
+    summary.ticks = k;
+    summary.fate_log_sha256 = fate_sha256(&net);
+    let mut last = Vec::new();
+    for c in &mut clients {
+        if let Some(w) = c.bytes_log.as_mut() {
+            w.flush()
+                .map_err(|e| fail(format!("a client's bytes log: {e}")))?;
+        }
+        let counters = server.room.client(c.slot).map(|x| x.counters().clone());
+        let k = counters.unwrap_or_default();
+        summary.commands += c.behaviour.given();
+        summary.skipped += c.behaviour.skipped();
+        summary.duplicates += c.duplicates;
+        summary.late += k.late;
+        summary.press_refused += k.press_refused;
+        summary.buffer_full += k.buffer_full;
+        summary.rewinds_refused += k.rewinds_refused;
+        summary.stale += c.endpoint.stale();
+        let mut e = EventRecord::new("client_counts", Phase::After)
+            .with("slot", c.slot)
+            .with("commands_count", c.behaviour.given())
+            .with("steps_without_command_count", c.behaviour.skipped())
+            .with("duplicate_stamps_count", c.duplicates)
+            .with("late_count", k.late)
+            .with("press_refused_count", k.press_refused)
+            .with("buffer_full_count", k.buffer_full)
+            .with("rewinds_refused_count", k.rewinds_refused)
+            .with("stale_count", c.endpoint.stale())
+            .with("events_queued_count", k.events_queued)
+            .with("overflow_resyncs_count", k.overflow_resyncs);
+        e.t = Some(logs.wide(server.room.last_tick()));
+        last.push(e);
+    }
+    summary.audit = mirror.audit.clone();
+    last.push(
+        EventRecord::new("fate_log", Phase::After).with("sha256", summary.fate_log_sha256.clone()),
+    );
+    last.push(
+        EventRecord::new("run_lengths", Phase::After)
+            .with("warmup_ticks_count", s.warmup_ticks)
+            .with("measured_ticks_count", s.measured_ticks)
+            .with("after_ticks_count", summary.after_ticks)
+            .with("after_capped", summary.after_capped)
+            .with("order_digest", format!("{digest:016x}")),
+    );
+    if let Some(r) = recording {
+        let bytes = r.encode();
+        let sha = sha256_hex(&bytes);
+        if let Some(path) = &s.record {
+            if let Some(dir) = path.parent() {
+                std::fs::create_dir_all(dir)
+                    .map_err(|e| fail(format!("{}: {e}", dir.display())))?;
+            }
+            let mut f = std::fs::File::create_new(path).map_err(|e| {
+                fail(format!(
+                    "{}: {e} (a recording is never overwritten)",
+                    path.display()
+                ))
+            })?;
+            f.write_all(&bytes)
+                .map_err(|e| fail(format!("{}: {e}", path.display())))?;
+        }
+        last.push(
+            EventRecord::new("recording", Phase::After)
+                .with("ticks_count", r.ticks.len())
+                .with("bytes", bytes.len())
+                .with("sha256", sha.clone())
+                .with("span_ms", r.span_ms),
+        );
+        summary.recording = Some((bytes.len(), sha));
+        if s.record_mem {
+            summary.recording_bytes = Some(bytes);
+        }
+    }
+    let a = &summary.audit;
+    last.push(
+        EventRecord::new("class_audit", Phase::After)
+            .with("owner_count", a.owner)
+            .with("spatial_count", a.spatial)
+            .with("spatial_unchecked_count", a.spatial_unchecked)
+            .with("public_count", a.public)
+            .with("events_count", a.events)
+            .with("team_count", 0)
+            .with("breaches_count", a.breaches_count)
+            .with(
+                "first_breach",
+                a.breaches.first().cloned().unwrap_or_default(),
+            ),
+    );
+    let counts = logs.finish(Ended::Clean, &last).map_err(fail)?;
+    summary.counts = counts;
+    Ok(summary)
+}
+
+/// Which burst of the profile starts at `start` (its index, 0 to 9).
+fn bursts_index(p: &Profile, start: u64) -> usize {
+    p.bursts_ns.iter().position(|b| b.0 == start).unwrap_or(0)
+}
+
+/// One client step's received bytes, for `replay` (r3): host instant u64, count u32, then per message channel u8,
+/// arrival u64, length u32 and the bytes, little-endian (replay.rs `read_steps` reads it back).
+pub(crate) fn write_step(
+    w: &mut impl Write,
+    host_ns: u64,
+    received: &[(Channel, Vec<u8>, u64)],
+) -> Result<(), String> {
+    let e = |e: std::io::Error| format!("a client's bytes log: {e}");
+    w.write_all(&host_ns.to_le_bytes()).map_err(e)?;
+    w.write_all(&(received.len() as u32).to_le_bytes())
+        .map_err(e)?;
+    for (ch, b, at) in received {
+        let code: u8 = match ch {
+            Channel::Datagram => 0,
+            Channel::Control => 1,
+            Channel::FullState => 2,
+        };
+        w.write_all(&[code]).map_err(e)?;
+        w.write_all(&at.to_le_bytes()).map_err(e)?;
+        w.write_all(&(b.len() as u32).to_le_bytes()).map_err(e)?;
+        w.write_all(b).map_err(e)?;
+    }
+    Ok(())
+}
+
+/// The header params of a run (u4r-brief.md 2.1): the plan's, the carries' and the lead.
+pub(crate) fn params(s: &Settings, suite: &str, twin: Option<&str>) -> BTreeMap<String, Value> {
+    let mut p: BTreeMap<String, Value> = BTreeMap::new();
+    let w = s.scene.weapon();
+    p.insert("suite".into(), suite.into());
+    p.insert("cmd_timing".into(), "B".into());
+    p.insert("lead_ticks_count".into(), s.lead.into());
+    p.insert("tick_start".into(), s.tick_start.into());
+    p.insert(
+        "ticks_below_wrap".into(),
+        ((65_536 - s.tick_start % 65_536) % 65_536).into(),
+    );
+    p.insert("event_start_count".into(), s.event_start.into());
+    p.insert("cmd_start_count".into(), s.cmd_start.into());
+    p.insert("stand_ins".into(), Value::Array(Vec::new()));
+    p.insert("stand_in_burn_ns".into(), 0.into());
+    p.insert("stand_in_top_up_ns".into(), 0.into());
+    p.insert("weapon_interval_us".into(), w.interval_us.into());
+    p.insert("weapon_damage_count".into(), w.damage.into());
+    p.insert("weapon_reach_cm".into(), w.reach_cm.into());
+    p.insert("weapon_magazine_count".into(), w.magazine.into());
+    p.insert("weapon_reload_ms".into(), w.reload_ms.into());
+    p.insert("weapon_spread_urad".into(), w.spread_urad.into());
+    p.insert("soldier_health_count".into(), s.scene.health().into());
+    p.insert("warmup_ticks_count".into(), s.warmup_ticks.into());
+    p.insert("measured_ticks_count".into(), s.measured_ticks.into());
+    p.insert("behaviour".into(), s.behaviour.clone().into());
+    p.insert(
+        "features".into(),
+        Value::Array(if cfg!(feature = "bites") {
+            vec!["bites".into()]
+        } else {
+            Vec::new()
+        }),
+    );
+    p.insert(
+        "reorder_step_permille".into(),
+        s.profile.reorder.permille.into(),
+    );
+    if !s.profile.bursts_ns.is_empty() {
+        let starts: Vec<u64> = s.profile.bursts_ns.iter().map(|b| b.0).collect();
+        p.insert("burst_starts_ns".into(), starts.into());
+        // SERIES.md's row names npt-lab (L1) as its writer (the advisor's U4r build ruling 3, point 8 (D)).
+        p.insert("burst_spacing_ns".into(), s.burst_spacing_ns.into());
+    }
+    if let Some(t) = twin {
+        p.insert("twin".into(), t.into());
+    }
+    if s.every_step {
+        p.insert("every_step".into(), true.into());
+    }
+    p
+}
+
+/// `scenario`'s usage line.
+pub(crate) const USAGE: &str = "npt-lab scenario --run <id> --set <set> --date <YYYY-MM-DD> --commit <sha> \
+--out <folder> --scene SS|ES --profile P0..P4|B1 --seed <n> [--role measured|twin|bite] [--marks <id,id...>] \
+[--twin <run>] [--suite <name>] [--clients <n>] [--tick-start <n>] [--event-start <n>] [--cmd-start <n>] \
+[--warmup-s <s>] [--measured-s <s>] [--after-max-s <s>] [--behaviour brawl|presses|clamp|steady] [--bursts <n>] \
+[--burst-first-s <s>] [--burst-every-s <s>] [--bite <id> [--arm-tick <n>]] [--record <file>] [--bytes <folder>] \
+[--every-step 1] [--lead <1..8>] [--logs 0 (with --record)]";
+
+/// B1's burst starts (u4r-brief.md 7.1, -18's row): `count` bursts, the first `first_s` after the measured phase's
+/// first tick and one every `every_s`, each starting a seeded offset in [0, 1 tick) after its tick boundary.
+pub(crate) fn burst_starts(
+    seed: u64,
+    warmup_ticks: u64,
+    count: u64,
+    first_s: u64,
+    every_s: u64,
+) -> Vec<u64> {
+    let hz = u64::from(npt_sim::TICK_HZ);
+    let mut rng = seed ^ 0x6275_7273_7473_0000;
+    (0..count)
+        .map(|i| {
+            let boundary = due_ns(warmup_ticks + first_s * hz + i * every_s * hz);
+            boundary + below(mix(&mut rng), due_ns(1))
+        })
+        .collect()
+}
+
+/// The `scenario` subcommand's options (after `scenario`), as `--name value` pairs, each once.
+pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
+    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
+    let mut it = argv.iter();
+    while let Some(flag) = it.next() {
+        let name = flag
+            .strip_prefix("--")
+            .ok_or(format!("unexpected argument {flag:?}"))?;
+        let value = it.next().ok_or(format!("{flag} needs a value"))?;
+        if map.insert(name, value).is_some() {
+            return Err(format!("{flag} given twice"));
+        }
+    }
+    let mut take = |k: &str| map.remove(k).map(str::to_owned);
+    let need = |v: Option<String>, k: &str| v.ok_or(format!("--{k} is required"));
+    let num = |v: Option<String>, k: &str, default: u64| match v {
+        None => Ok(default),
+        Some(s) => s
+            .parse::<u64>()
+            .map_err(|_| format!("--{k} {s:?} is not a whole number")),
+    };
+    let small =
+        |v: u64, k: &str| u32::try_from(v).map_err(|_| format!("--{k} {v} is above 32 bits"));
+    let run = need(take("run"), "run")?;
+    let set = need(take("set"), "set")?;
+    let date = need(take("date"), "date")?;
+    let commit = need(take("commit"), "commit")?;
+    let out = PathBuf::from(need(take("out"), "out")?);
+    let scene_name = need(take("scene"), "scene")?;
+    let scene = match scene_name.as_str() {
+        "SS" => Scene::stress(),
+        "ES" => Scene::estimate(),
+        "CO" => Scene::course(),
+        "TC" => Scene::targets(),
+        other => return Err(format!("--scene {other:?}: SS, ES, CO or TC")),
+    };
+    let profile_name = need(take("profile"), "profile")?;
+    let seed = num(Some(need(take("seed"), "seed")?), "seed", 0)?;
+    let role = take("role").unwrap_or_else(|| "measured".to_owned());
+    let marks: Vec<String> = take("marks")
+        .map(|m| m.split(',').map(str::to_owned).collect())
+        .unwrap_or_default();
+    let twin = take("twin");
+    let suite = take("suite").unwrap_or_else(|| "brawl".to_owned());
+    let clients = num(take("clients"), "clients", u64::from(scene.slots()))?;
+    let clients = u8::try_from(clients)
+        .ok()
+        .filter(|c| (1..=scene.slots()).contains(c))
+        .ok_or(format!("--clients {clients}: 1 to {}", scene.slots()))?;
+    let tick_start = small(num(take("tick-start"), "tick-start", 0)?, "tick-start")?;
+    let event_start = small(num(take("event-start"), "event-start", 0)?, "event-start")?;
+    let cmd_start = small(num(take("cmd-start"), "cmd-start", 0)?, "cmd-start")?;
+    let hz = u64::from(npt_sim::TICK_HZ);
+    let warmup_ticks = num(take("warmup-s"), "warmup-s", 10)? * hz;
+    let measured_ticks = num(take("measured-s"), "measured-s", 60)? * hz;
+    let after_max_ticks = num(take("after-max-s"), "after-max-s", 30)? * hz;
+    let behaviour = take("behaviour").unwrap_or_else(|| "brawl".to_owned());
+    if !["brawl", "presses", "clamp", "steady"].contains(&behaviour.as_str()) {
+        return Err(format!(
+            "--behaviour {behaviour:?}: brawl, presses, clamp or steady"
+        ));
+    }
+    let bursts = num(take("bursts"), "bursts", 10)?;
+    let burst_first_s = num(take("burst-first-s"), "burst-first-s", 30)?;
+    let burst_every_s = num(take("burst-every-s"), "burst-every-s", 60)?;
+    let bite = take("bite");
+    let arm_tick = small(
+        num(take("arm-tick"), "arm-tick", u64::from(tick_start))?,
+        "arm-tick",
+    )?;
+    let record = take("record").map(PathBuf::from);
+    let bytes = take("bytes").map(PathBuf::from);
+    let every_step = match take("every-step").as_deref() {
+        None => false,
+        Some("1") => true,
+        Some(v) => return Err(format!("--every-step {v:?}: 1 or absent")),
+    };
+    let logs = match take("logs").as_deref() {
+        None | Some("1") => true,
+        Some("0") => false,
+        Some(v) => return Err(format!("--logs {v:?}: 0 or 1")),
+    };
+    let lead_override = take("lead")
+        .map(|v| {
+            v.parse::<u32>()
+                .ok()
+                .filter(|l| (1..=8).contains(l))
+                .ok_or(format!("--lead {v:?}: 1 to 8"))
+        })
+        .transpose()?;
+    if let Some(k) = map.keys().next() {
+        return Err(format!("unknown option --{k}"));
+    }
+    if !["measured", "twin", "bite"].contains(&role.as_str()) {
+        return Err(format!("--role {role:?}: measured, twin or bite"));
+    }
+    if (role == "bite") != bite.is_some() && role != "twin" {
+        return Err("--role bite and --bite go together".to_owned());
+    }
+    if !logs && record.is_none() {
+        return Err("--logs 0 is for a recording run (--record): a run keeps its logs".to_owned());
+    }
+    if (every_step || lead_override.is_some()) && !marks.is_empty() {
+        return Err(
+            "--every-step and --lead are D13's test-only settings: a run with them lists no mark"
+                .to_owned(),
+        );
+    }
+    if (every_step || lead_override.is_some()) && record.is_some() {
+        // A recording's settings are its manifest entry's, which has neither key (hash.rs `Kind::Runner`; the
+        // advisor's U4r build ruling, point 9 (a)).
+        return Err(
+            "--every-step and --lead are D13's test-only settings: a recording run (--record) takes neither"
+                .to_owned(),
+        );
+    }
+    let profile = match profile_name.as_str() {
+        "B1" => Profile::b1(&burst_starts(
+            seed,
+            warmup_ticks,
+            bursts,
+            burst_first_s,
+            burst_every_s,
+        )),
+        p => Profile::by_name(p).ok_or(format!("--profile {p:?}: P0 to P4 or B1"))?,
+    };
+    let bites = match &bite {
+        None => Bites::default(),
+        Some(id) => Bites::of(id)?,
+    };
+    let lead = lead_override.unwrap_or_else(|| lead_for(&profile));
+    let burst_spacing_ns = if profile.bursts_ns.is_empty() {
+        0
+    } else {
+        burst_every_s.saturating_mul(SEC)
+    };
+    let mut s = Settings {
+        meta: Meta {
+            run,
+            set,
+            role,
+            marks,
+            date,
+            commit,
+            layer: "L1".to_owned(),
+            profile: profile.name.to_owned(),
+            seed,
+            scene: scene.name().to_owned(),
+            clients: u32::from(clients),
+            bite,
+            params: BTreeMap::new(),
+        },
+        out,
+        scene,
+        profile,
+        clients,
+        seed,
+        tick_start,
+        event_start,
+        cmd_start,
+        warmup_ticks,
+        measured_ticks,
+        after_max_ticks,
+        burst_spacing_ns,
+        behaviour,
+        every_step,
+        bites,
+        arm_tick,
+        lead,
+        record,
+        bytes,
+        logs,
+        record_mem: false,
+    };
+    s.meta.params = params(&s, &suite, twin.as_deref());
+    Ok(s)
+}
+
+/// `part / whole` as a decimal for a report line (0 when `whole` is 0): carry 49's figures, reported, never judged.
+fn share(part: u64, whole: u64) -> f64 {
+    if whole == 0 {
+        0.0
+    } else {
+        part as f64 / whole as f64
+    }
+}
+
+/// The summary line of a run. A `--logs 0` run prints "not counted (--logs 0)" for every figure taken from the logs'
+/// counts or from `room_tick`, never 0 (the advisor's U4r build ruling, point 9 (b)); the counts that need no log stay.
+pub(crate) fn summary_line(s: &Settings, o: &Summary) -> String {
+    let c = &o.counts;
+    let logged = |v: String| {
+        if s.logs {
+            v
+        } else {
+            "not counted (--logs 0)".to_owned()
+        }
+    };
+    format!(
+        "{} {} {} seed {}: {} ticks ({} after{}); lead {}; commands {} (steps without one {}, duplicate stamps {}); \
+         late {}; press refused {}; buffer full {}; rewinds refused {}; stale {}; overflow resyncs {}; heals {}; \
+         client hashes {} (sentinels {}); measured incomplete {}; decided lost at the stop {}; shots per measured \
+         second {}; carry 49: measured client sets {}, holding every very large unit {}, projectiles per set {}; \
+         audit {} owner, {} spatial checked against the radius, {} spatial not checked, {} breaches; fate log {}; \
+         connect order digest {:016x}",
+        s.meta.run,
+        s.scene.name(),
+        s.profile.name,
+        s.seed,
+        o.ticks,
+        o.after_ticks,
+        if o.after_capped {
+            ", the after phase capped with measured ticks unresolved: a bite run, carry 73"
+        } else {
+            ""
+        },
+        s.lead,
+        o.commands,
+        o.skipped,
+        o.duplicates,
+        o.late,
+        o.press_refused,
+        o.buffer_full,
+        o.rewinds_refused,
+        o.stale,
+        o.overflow_resyncs,
+        o.heals,
+        logged(c.hashes.to_string()),
+        logged(c.sentinels.to_string()),
+        logged(c.incomplete_measured.to_string()),
+        logged(c.lost_decided_at_stop.to_string()),
+        logged(
+            c.shots_measured
+                .checked_div(c.measured_seconds)
+                .unwrap_or(0)
+                .to_string()
+        ),
+        logged(c.sets_measured.to_string()),
+        logged(format!(
+            "{} ({:.4})",
+            c.large_measured,
+            share(c.large_measured, c.sets_measured)
+        )),
+        logged(format!(
+            "{:.2}",
+            share(c.projectiles_measured, c.sets_measured)
+        )),
+        o.audit.owner,
+        o.audit.spatial,
+        o.audit.spatial_unchecked,
+        o.audit.breaches_count,
+        &o.fate_log_sha256[..16.min(o.fate_log_sha256.len())],
+        o.order_digest,
+    )
+}
+
+#[cfg(test)]
+pub(crate) mod tests {
+    use super::*;
+    use std::path::Path;
+
+    pub(crate) const COMMIT: &str = "0000000000000000000000000000000000000000";
+
+    /// A test run's settings, through `scenario`'s own parser.
+    pub(crate) fn settings(out: &Path, args: &str) -> Settings {
+        let line = format!(
+            "--run t --set t --date 2026-11-13 --commit {COMMIT} --out {} --scene SS {args}",
+            out.display()
+        );
+        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+        parse(&argv).unwrap_or_else(|e| panic!("{line}: {e}"))
+    }
+
+    /// One test's scratch folder; removed when the test passes, kept when it fails (the advisor's U4r build ruling 2,
+    /// point 3 (ii)).
+    pub(crate) struct Scratch(PathBuf);
+
+    impl std::ops::Deref for Scratch {
+        type Target = PathBuf;
+
+        fn deref(&self) -> &PathBuf {
+            &self.0
+        }
+    }
+
+    impl Drop for Scratch {
+        fn drop(&mut self) {
+            if !std::thread::panicking() {
+                // The test's own teardown of the folder it made (absent if the test wrote nothing).
+                let _ = std::fs::remove_dir_all(&self.0);
+            }
+        }
+    }
+
+    /// A fresh folder for one test's logs, named by the test and the process, inside the cargo target dir the test
+    /// binary runs from: target/<profile>/npt-lab-tests/, beside deps/ (so under .gate/, plan :762, and removable by
+    /// `cargo clean --target-dir`), never the system's temporary folder.
+    pub(crate) fn scratch(name: &str) -> Scratch {
+        let exe = std::env::current_exe().expect("the test binary");
+        let profile = exe
+            .parent()
+            .and_then(Path::parent)
+            .expect("the test binary lies in target/<profile>/deps/");
+        let d = profile
+            .join("npt-lab-tests")
+            .join(format!("{name}-{}", std::process::id()));
+        if d.exists() {
+            std::fs::remove_dir_all(&d).expect("an old scratch folder of this process");
+        }
+        Scratch(d)
+    }
+
+    pub(crate) fn read(dir: &Path, process: &str) -> npt_log::Log {
+        let f = std::fs::File::open(dir.join(format!("{process}.jsonl"))).expect("the log");
+        npt_log::read_log(std::io::BufReader::new(f)).expect("the log reads")
+    }
+
+    /// r1's leg settings: 16 clients on P2 (its loss and jitter make each link's fates its own), 6 s of warm-up, 2 s
+    /// measured.
+    const R1_ARGS: &str =
+        "--seed 1 --profile P2 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10";
+
+    /// One leg of r1, run as its own process (D-2's form: this test binary re-run through `current_exe`): a no-op
+    /// unless `NPT_LAB_R1_LEG` names its folder; `NPT_LAB_R1_BITE` set selects the client-order bite.
+    #[test]
+    fn r1_leg() {
+        let Ok(dir) = std::env::var("NPT_LAB_R1_LEG") else {
+            return;
+        };
+        #[allow(unused_mut, reason = "the bite is set in bites builds only")]
+        let mut s = settings(Path::new(&dir), R1_ARGS);
+        #[cfg(feature = "bites")]
+        if std::env::var("NPT_LAB_R1_BITE").is_ok() {
+            s.bites.client_order = true;
+            s.bites.id = Some(CLIENT_ORDER_BITE.to_owned());
+        }
+        run(&s).unwrap_or_else(|e| panic!("the leg: {e:?}"));
+    }
+
+    /// Runs two legs at once, each its own process, and returns their folders.
+    fn two_legs(name: &str, bite: bool) -> [Scratch; 2] {
+        let exe = std::env::current_exe().expect("the test binary");
+        let dirs = [scratch(&format!("{name}-1")), scratch(&format!("{name}-2"))];
+        let children: Vec<_> = dirs
+            .iter()
+            .map(|d| {
+                let mut c = std::process::Command::new(&exe);
+                c.args(["--exact", "scenario::tests::r1_leg", "--test-threads", "1"])
+                    .env("NPT_LAB_R1_LEG", d.as_os_str());
+                if bite {
+                    c.env("NPT_LAB_R1_BITE", "1");
+                }
+                c.stdout(std::process::Stdio::null())
+                    .spawn()
+                    .expect("a leg starts")
+            })
+            .collect();
+        for mut c in children {
+            assert!(c.wait().expect("a leg ends").success(), "a leg failed");
+        }
+        dirs
+    }
+
+    /// r1's comparison: the fate logs' SHA-256s, the room's per-tick World hash `h` and `hs.server_hash`, and the
+    /// clients' `hs.decoded_hash` and `tick.s.snap_incomplete`, over the same ticks. `Ok` with the ticks compared, or
+    /// the first difference.
+    fn twin_differences(a: &Path, b: &Path) -> Result<usize, String> {
+        if let Some(e) = fate_difference(a, b) {
+            return Err(e);
+        }
+        tick_differences(a, b)
+    }
+
+    fn fate_difference(a: &Path, b: &Path) -> Option<String> {
+        let fate = |d: &Path| {
+            read(d, "lab")
+                .events
+                .iter()
+                .find(|e| e.ev == "fate_log")
+                .map(|e| e.f["sha256"].clone())
+        };
+        (fate(a) != fate(b)).then(|| format!("fate logs {:?} and {:?}", fate(a), fate(b)))
+    }
+
+    /// The per-tick half of r1's comparison.
+    fn tick_differences(a: &Path, b: &Path) -> Result<usize, String> {
+        let mut compared = 0;
+        for process in ["room", "client"] {
+            let (x, y) = (read(a, process), read(b, process));
+            if x.ticks.len() != y.ticks.len() {
+                return Err(format!(
+                    "{process}: {} and {} tick records",
+                    x.ticks.len(),
+                    y.ticks.len()
+                ));
+            }
+            for (p, q) in x.ticks.iter().zip(&y.ticks) {
+                if p.t != q.t
+                    || p.h != q.h
+                    || p.hs != q.hs
+                    || p.s.get("snap_incomplete") != q.s.get("snap_incomplete")
+                {
+                    return Err(format!("{process}: tick {} (and {}) differs", p.t, q.t));
+                }
+                compared += 1;
+            }
+        }
+        Ok(compared)
+    }
+
+    fn digest(d: &Path) -> String {
+        read(d, "lab")
+            .events
+            .iter()
+            .find(|e| e.ev == "run_lengths")
+            .map(|e| e.f["order_digest"].to_string())
+            .unwrap_or_default()
+    }
+
+    /// r1, the runner's twin test (§4.11 :406-407; carry 62 (f)): two processes, one seed and scenario, give equal
+    /// fate logs and equal per-tick server and client hashes over the same ticks.
+    #[test]
+    fn r1_two_processes_of_one_seed_give_equal_fate_logs_and_per_tick_hashes() {
+        let [a, b] = two_legs("r1", false);
+        let compared = twin_differences(&a, &b).unwrap_or_else(|e| panic!("r1: {e}"));
+        assert!(compared > 200, "ticks compared: {compared}");
+        assert_eq!(digest(&a), digest(&b), "slot order in both");
+        println!(
+            "r1: {compared} tick records compared, equal; connect order digest {}",
+            digest(&a)
+        );
+    }
+
+    /// r1's bite (marks F29; bites builds only): the clients connect in a `HashMap`'s order, which each process's
+    /// random state seeds, so the links' fates and then the per-tick hashes differ; equal order digests mean not
+    /// exercised, never caught or missed (§4.14 :476).
+    #[cfg(feature = "bites")]
+    #[test]
+    fn r1_bite_a_hash_map_client_order_makes_the_two_processes_differ() {
+        let [a, b] = two_legs("r1-bite", true);
+        let (da, db) = (digest(&a), digest(&b));
+        println!("r1 bite: order digests {da} and {db}");
+        assert_ne!(da, db, "not exercised: both processes drew one order");
+        let fate = fate_difference(&a, &b).expect("r1 must catch the bite in the fate logs");
+        let ticks =
+            tick_differences(&a, &b).expect_err("r1 must catch the bite in the per-tick hashes");
+        println!("r1 bite: caught: {fate}; {ticks}");
+    }
+
+    /// Carry 49 (TUNE-2), this brief's unit test: a scripted teleport above World's clamp bound is refused by name, one
+    /// at the bound is accepted, and so are spawns at it.
+    #[test]
+    fn s1_a_scripted_placement_above_the_clamp_bound_is_refused_one_at_it_accepted() {
+        assert_eq!(
+            PLACEMENT_Z_BOUND_CM, 3_955,
+            "World's stated bound: feet at 39.55 m"
+        );
+        let at = |z| Placement {
+            slot: 3,
+            position: [100, -200, z],
+        };
+        let e =
+            check_placements(&[], &[at(PLACEMENT_Z_BOUND_CM + 1)]).expect_err("above the bound");
+        assert!(e.contains("teleport") && e.contains("TUNE-2"), "{e}");
+        assert!(check_placements(&[at(PLACEMENT_Z_BOUND_CM)], &[at(PLACEMENT_Z_BOUND_CM)]).is_ok());
+        assert!(check_placements(&[at(4_000)], &[]).is_err(), "a spawn too");
+    }
+
+    /// Carry 74 as U4R-Q14 ruled: the round trip plus the jitter in ticks, rounded up, at least 1, plus 1.
+    #[test]
+    fn s2_each_profile_s_lead() {
+        let lead = |p: Option<Profile>| lead_for(&p.expect("a profile"));
+        assert_eq!(lead(Profile::by_name("P0")), 2);
+        assert_eq!(lead(Profile::by_name("P1")), 3);
+        assert_eq!(lead(Profile::by_name("P2")), 5);
+        assert_eq!(lead(Some(Profile::b1(&[0]))), 5);
+        assert_eq!(lead(Profile::by_name("P3")), 7);
+        assert!(lead(Profile::by_name("P3")) <= 8, "within CMD_CATCHUP");
+    }
+
+    /// -18's ten bursts (u4r-brief.md 7.1): each a seeded offset in [0, 1 tick) after a tick boundary, 60 s apart from
+    /// 30 s into the measured phase, and the same for the same seed.
+    #[test]
+    fn s3_burst_starts_lie_within_a_tick_of_their_boundaries() {
+        let b = burst_starts(3, 300, 10, 30, 60);
+        assert_eq!(b.len(), 10);
+        for (i, s) in b.iter().enumerate() {
+            let boundary = due_ns(300 + 900 + 1_800 * i as u64);
+            assert!(*s >= boundary && *s < boundary + due_ns(1), "burst {i}");
+        }
+        assert_eq!(b, burst_starts(3, 300, 10, 30, 60));
+        assert_ne!(b, burst_starts(4, 300, 10, 30, 60));
+    }
+
+    /// Carry 82 and §4.15: a runner recording (the room's `tick_input()` after each tick, generator 0, the room's
+    /// span) regenerates byte for byte, holds exactly the warm-up and measured ticks, and replays through npt-sim's
+    /// `Replayer` to the room's own per-tick World hashes.
+    #[test]
+    fn s5_a_runner_recording_regenerates_byte_for_byte_and_replays_to_the_room_s_hashes() {
+        let scene = Scene::stress();
+        let a = record(&scene, "P0", 1, 8, 100, 4, 2).expect("a recording");
+        let b = record(&scene, "P0", 1, 8, 100, 4, 2).expect("again");
+        assert_eq!(a, b, "regenerated byte for byte");
+        let r = Recording::decode(&a).expect("it decodes");
+        assert_eq!(
+            (r.ticks.len(), r.first_tick, r.generator, r.span_ms),
+            (180, 100, 0, ROOM_HISTORY_MS)
+        );
+        let dir = scratch("s5");
+        let mut s = settings(
+            &dir,
+            "--seed 1 --profile P0 --clients 8 --tick-start 100 --warmup-s 4 --measured-s 2",
+        );
+        s.record_mem = true;
+        let o = run(&s).expect("the run");
+        assert_eq!(
+            o.recording_bytes.as_deref(),
+            Some(a.as_slice()),
+            "the logged run records the same bytes"
+        );
+        let words: Vec<u32> = a
+            .chunks(4)
+            .map(|c| {
+                let mut w = [0u8; 4];
+                w[..c.len()].copy_from_slice(c);
+                u32::from_le_bytes(w)
+            })
+            .collect();
+        let mut rep = npt_sim::replay::Replayer::new();
+        for w in words {
+            rep.feed(w);
+        }
+        let mut replayed = Vec::new();
+        while let npt_sim::replay::Step::Stepped(tick) = rep.step() {
+            replayed.push((tick, rep.state_hash().expect("a hash after a step")));
+        }
+        assert_eq!(
+            replayed, o.world_hashes,
+            "the replay's hash at every tick is the room's"
+        );
+    }
+
+    /// r4's run: SS, 48 clients in 3 x 16, P2, 6 s of warm-up and 2 s measured.
+    fn r4_run(seed: u64, bite: &str) -> Summary {
+        let dir = scratch(&format!(
+            "r4-{seed}{}",
+            if bite.is_empty() { "" } else { "-bite" }
+        ));
+        let s = settings(
+            &dir,
+            &format!("--seed {seed} --profile P2 --warmup-s 6 --measured-s 2 {bite}"),
+        );
+        run(&s).unwrap_or_else(|e| panic!("r4 seed {seed}: {e:?}"))
+    }
+
+    /// r4, the class audit (catalogue §6 at U4, item 5): every group the encoder writes into each client's snapshots
+    /// and full states is one its class lets that client see, on 3 seeds; `team` and `public` sections do not exist in
+    /// the spike (not exercised).
+    #[test]
+    fn r4_every_group_is_one_its_class_lets_that_client_see_on_three_seeds() {
+        for seed in 1..=3 {
+            let o = r4_run(seed, "");
+            let a = &o.audit;
+            println!(
+                "r4 seed {seed}: {} ticks; audited owner {} (own-state records), spatial {} checked against the \
+                 radius and {} not checked (entity records), public {} (headers), events {}; team 0 and public \
+                 sections 0: not exercised; breaches {}",
+                o.ticks,
+                a.owner,
+                a.spatial,
+                a.spatial_unchecked,
+                a.public,
+                a.events,
+                a.breaches_count
+            );
+            assert!(
+                a.owner > 0 && a.spatial > 0 && a.public > 0,
+                "seed {seed}: audited"
+            );
+            assert!(a.breaches.is_empty(), "seed {seed}: {:?}", a.breaches);
+        }
+    }
+
+    /// r4's bite (U4E-Q13 (a)): `catalogue-5/owner-to-all`, its run's headers listing no mark, fails the audit by name.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn r4_bite_owner_to_all_fails_the_audit_by_name() {
+        let o = r4_run(1, "--role bite --bite catalogue-5/owner-to-all");
+        let first = o.audit.breaches.first().cloned().unwrap_or_default();
+        println!(
+            "r4 bite: {} breaches (16 texts kept); the first: {first}",
+            o.audit.breaches_count
+        );
+        assert!(
+            first.contains("own-state section holds"),
+            "the owner rule fires: {first:?}"
+        );
+    }
+
+    /// A joined Loopback session to `server` for slot `slot`'s credentials: its client link and endpoint, the room's
+    /// answer delivered.
+    fn joined(
+        server: &mut Server,
+        slot: u8,
+        now: &mut u64,
+    ) -> (npt_link::Loopback, ClientEndpoint, u64) {
+        let (s, mut c) = npt_link::Loopback::pair(MAX_DATAGRAM);
+        let id = server.accept(Box::new(s), *now);
+        let mut e = ClientEndpoint::new(join_of(1, slot), npt_sim::TICK_HZ);
+        e.input(None, &[], *now);
+        for (ch, b) in e.step().reliable {
+            c.send_reliable(ch, &b, *now);
+        }
+        *now += 1;
+        server.io(*now);
+        deliver(server, &mut c, &mut e, *now);
+        (c, e, id)
+    }
+
+    /// One tick of the room, its output sent, and what reached `c` handed to `e`.
+    fn deliver(server: &mut Server, c: &mut npt_link::Loopback, e: &mut ClientEndpoint, now: u64) {
+        server.room.tick();
+        for slot in 0..server.room.sim().scene().slots() {
+            let d = server.room.take_datagrams(slot);
+            let k = server.room.take_control(slot);
+            server.send(slot, &d, &k, now).expect("sent");
+        }
+        let mut got = Vec::new();
+        while let Some(r) = c.poll_recv(now + 1) {
+            got.push((r.channel, r.bytes, r.arrival_ns));
+        }
+        e.input(None, &got, now + 1);
+    }
+
+    fn room_counts(server: &Server, slot: u8) -> u64 {
+        server.room.client(slot).map_or(0, |c| {
+            let k = c.counters();
+            k.commands_buffered + k.commands_repeat + k.late + k.buffer_full + k.press_refused
+        })
+    }
+
+    fn send_input(c: &mut npt_link::Loopback, e: &mut ClientEndpoint, now: u64) {
+        e.input(Some(npt_sim::Command::default()), &[], now);
+        for d in e.step().datagrams {
+            c.send_unreliable(&d, now);
+        }
+    }
+
+    fn send_leave(c: &mut npt_link::Loopback, now: u64) {
+        let mut out = Vec::new();
+        npt_proto::encode(&Message::Leave(npt_proto::Leave::Quit), &mut out).expect("a Leave");
+        c.send_reliable(Channel::Control, &out, now);
+    }
+
+    fn lab_room(n: u8) -> Server {
+        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
+        let world = World::new(&scene, 1, 0);
+        let greybox = world.greybox().clone();
+        let tokens = (0..scene.slots()).map(|s| credentials(1, s).0).collect();
+        let room = Room::new(
+            world,
+            &scene,
+            &greybox,
+            tokens,
+            usize::from(n),
+            npt_sim::TICK_HZ,
+        )
+        .with_hashes(CONTENT, [0; 32]);
+        Server::new(room, Quant::default(), RttSource::Link)
+    }
+
+    /// r8 (carry 76): a second endpoint joins with slot 0's token and bind while the first is live; after it the
+    /// first's Inputs and `Leave` reach no room call, and the second keeps the slot.
+    #[test]
+    fn r8_a_replaced_session_s_inputs_and_leave_reach_no_room_call() {
+        let mut server = lab_room(4);
+        let mut now = 1_000;
+        let (mut c1, mut e1, first) = joined(&mut server, 0, &mut now);
+        assert_eq!(server.owner.get(&0), Some(&first));
+        let (_c2, _e2, second) = joined(&mut server, 0, &mut now);
+        assert_eq!(
+            server.owner.get(&0),
+            Some(&second),
+            "the second holds slot 0"
+        );
+        assert!(
+            server.ended.contains(&(0, "replaced")),
+            "the first was told and dropped"
+        );
+        let before = room_counts(&server, 0);
+        for _ in 0..5 {
+            now += 33_000_000;
+            send_input(&mut c1, &mut e1, now);
+            server.io(now + 1);
+        }
+        assert_eq!(
+            room_counts(&server, 0),
+            before,
+            "the first's Inputs reach no room call"
+        );
+        send_leave(&mut c1, now);
+        server.io(now + 2);
+        assert!(
+            server.room.client(0).is_some(),
+            "the first's Leave freed nothing"
+        );
+        assert_eq!(server.owner.get(&0), Some(&second));
+    }
+
+    /// r9 (carry 76): a session the room kicks (`Kick(overflow)`: a client that never acknowledges, under fire, with an
+    /// event cap of 2) is dropped, and its later datagrams and its `Leave` reach no room call.
+    #[test]
+    fn r9_a_kicked_session_s_later_datagrams_reach_no_room_call() {
+        let mut server = lab_room(48);
+        server.room = server.room.clone().with_event_cap(2);
+        let mut now = 1_000;
+        let (mut c0, mut e0, _) = joined(&mut server, 0, &mut now);
+        let (mut c1, mut e1, _) = joined(&mut server, 16, &mut now);
+        let mut kicked = false;
+        for _ in 0..600 {
+            now += 33_333_333;
+            // Slot 16 fires at slot 0 (another team); slot 0 never acknowledges.
+            let target = npt_sim::SimCore::entity_records(server.room.sim())
+                .iter()
+                .find(|r| r.kind == npt_sim::EntityKind::Soldier && r.index == 0)
+                .map(|r| [r.grid[0], r.grid[1], r.grid[2] + 100])
+                .expect("slot 0's soldier");
+            e1.input(
+                Some(npt_sim::Command {
+                    buttons: npt_sim::Command::FIRE,
+                    aim: target,
+                    ..npt_sim::Command::default()
+                }),
+                &[],
+                now,
+            );
+            for d in e1.step().datagrams {
+                c1.send_unreliable(&d, now);
+            }
+            server.io(now + 1);
+            deliver(&mut server, &mut c1, &mut e1, now + 2);
+            let mut drain = Vec::new();
+            while let Some(r) = c0.poll_recv(now + 3) {
+                drain.push((r.channel, r.bytes, r.arrival_ns));
+            }
+            // The kick read from the room itself, never from the glue under test.
+            if server.room.client(0).is_some_and(|c| c.kicked()) {
+                kicked = true;
+                break;
+            }
+        }
+        assert!(kicked, "the room kicked the client that never acknowledged");
+        assert!(
+            server.session(0).is_none(),
+            "the lab dropped the kicked session"
+        );
+        send_input(&mut c0, &mut e0, now + 10);
+        send_leave(&mut c0, now + 10);
+        server.io(now + 11);
+        assert!(
+            server.room.client(0).is_some_and(|c| c.kicked()),
+            "its Leave reached no room call: the kicked slot is not left"
+        );
+    }
+
+    /// The reliable messages' channel (the module text): exactly one stream channel takes each, never a fall-back.
+    #[test]
+    fn s4_a_reliable_message_goes_on_the_one_channel_that_takes_it() {
+        let q = Quant::default();
+        let mut ja = Vec::new();
+        npt_proto::encode(
+            &Message::JoinAccepted(npt_proto::JoinAccepted::default()),
+            &mut ja,
+        )
+        .expect("encodes");
+        assert_eq!(stream_of(&ja, q), Ok(Channel::Control));
+        let mut fs = Vec::new();
+        npt_proto::encode(
+            &Message::FullState(npt_proto::FullState {
+                tick: 7,
+                last_run: None,
+                own: Vec::new(),
+                entities: Vec::new(),
+            }),
+            &mut fs,
+        )
+        .expect("encodes");
+        assert_eq!(stream_of(&fs, q), Ok(Channel::FullState));
+        assert!(stream_of(&[0, 1, 99], q).is_err(), "no channel takes it");
+    }
+
+    /// The advisor's U4r build ruling, point 5: `ROOM_HISTORY_MS` (a copy of npt-room's private `HISTORY_MS`) pinned to
+    /// the room by behaviour. With the ring full, a firing command run at tick T claiming view tick T - C, C being
+    /// `HitHistory::for_ms(ROOM_HISTORY_MS)`'s capacity, gets a Rewind (the ring then holds T - C to T - 1), and one
+    /// claiming T - C - 1 counts in `rewinds_refused`.
+    #[test]
+    fn p5_the_room_s_ring_holds_room_history_ms_s_capacity_and_no_more() {
+        let c = npt_sim::HitHistory::for_ms(ROOM_HISTORY_MS)
+            .expect("a ring")
+            .capacity();
+        let mut server = lab_room(4);
+        let mut now = 1_000;
+        let (mut link, mut e, _) = joined(&mut server, 0, &mut now);
+        for _ in 0..c + 10 {
+            now += 33_333_333;
+            deliver(&mut server, &mut link, &mut e, now);
+        }
+        let mut seen = Vec::new();
+        for (seq, d) in [(0u16, c), (1, c + 1)] {
+            let t = server.room.last_tick().wrapping_add(1);
+            let cmd = npt_sim::Command {
+                slot: 0,
+                tick: t,
+                seq,
+                buttons: npt_sim::Command::FIRE,
+                aim: [0, 0, 100],
+                view_tick: t.wrapping_sub(d),
+                view_fraction: 0,
+                ..npt_sim::Command::default()
+            };
+            let mut b = Vec::new();
+            npt_proto::encode(
+                &Message::Input(npt_proto::Input {
+                    commands: vec![cmd],
+                    ..npt_proto::Input::default()
+                }),
+                &mut b,
+            )
+            .expect("an Input");
+            now += 33_333_333;
+            link.send_unreliable(&b, now);
+            server.io(now + 1);
+            let refused = |s: &Server| s.room.client(0).map_or(0, |x| x.counters().rewinds_refused);
+            let before = refused(&server);
+            deliver(&mut server, &mut link, &mut e, now + 2);
+            let rewind = server
+                .room
+                .tick_input()
+                .commands
+                .iter()
+                .find(|(x, _)| x.slot == 0 && x.seq == seq)
+                .map(|(_, r)| r.is_some());
+            seen.push((d, rewind, refused(&server) - before));
+        }
+        println!("p5: capacity {c} ticks; (view age, Rewind given, refused) {seen:?}");
+        assert_eq!(seen, [(c, Some(true), 0), (c + 1, Some(false), 1)]);
+    }
+
+    /// The advisor's U4r build ruling, point 2: mirror.rs's `INTEREST_RADIUS_CM` (a copy of npt-room's private
+    /// `RADIUS_CM`, room.rs:29) pinned to the room by placement. Two soldiers teleported (through `check_placements`)
+    /// to exactly the copy's radius and one cm beyond it from slot 0's soldier, in slot 0's own area; slot 0's join
+    /// FullState, decoded by the mirror, holds the first and not the second. `None` when no direction lands both
+    /// placements exactly in that area (the copy then unpinned).
+    fn radius_pin() -> Option<(bool, bool)> {
+        use crate::mirror::INTEREST_RADIUS_CM;
+        let r = i32::try_from(INTEREST_RADIUS_CM).expect("a radius in i32");
+        let mut server = lab_room(4);
+        server.room.tick();
+        let records = npt_sim::SimCore::entity_records(server.room.sim()).to_vec();
+        let at = |rs: &[npt_sim::EntityRecord], s: u16| {
+            rs.iter()
+                .find(|x| x.kind == npt_sim::EntityKind::Soldier && x.index == s)
+                .copied()
+                .expect("a soldier")
+        };
+        let me = at(&records, 0);
+        let p = [me.grid[0], me.grid[1], me.grid[2]];
+        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
+        for &(ax, ay) in &dirs {
+            for &(bx, by) in &dirs {
+                let mut s = lab_room(4);
+                s.room.tick();
+                let a = Placement {
+                    slot: 20,
+                    position: [p[0] + ax * r, p[1] + ay * r, p[2]],
+                };
+                let b = Placement {
+                    slot: 21,
+                    position: [p[0] + bx * (r + 1), p[1] + by * (r + 1), p[2]],
+                };
+                check_placements(&[], &[a, b]).expect("inside carry 49's bound");
+                s.room.script(&[], &[a, b], &[]);
+                s.room.tick();
+                let recs = npt_sim::SimCore::entity_records(s.room.sim()).to_vec();
+                let (ra, rb, rm) = (at(&recs, 20), at(&recs, 21), at(&recs, 0));
+                let landed =
+                    |x: &npt_sim::EntityRecord, q: [i32; 3]| x.grid[0] == q[0] && x.grid[1] == q[1];
+                if !(landed(&ra, a.position)
+                    && landed(&rb, b.position)
+                    && ra.area == rm.area
+                    && rb.area == rm.area)
+                {
+                    continue;
+                }
+                let (srv, mut cl) = npt_link::Loopback::pair(MAX_DATAGRAM);
+                s.accept(Box::new(srv), 1);
+                let mut e = ClientEndpoint::new(join_of(1, 0), npt_sim::TICK_HZ);
+                e.input(None, &[], 1);
+                for (ch, bytes) in e.step().reliable {
+                    cl.send_reliable(ch, &bytes, 1);
+                }
+                s.io(2);
+                let t = s.room.last_tick().wrapping_add(1);
+                s.room.tick();
+                let recs = npt_sim::SimCore::entity_records(s.room.sim()).to_vec();
+                let control = s.room.take_control(0);
+                let datagrams = s.room.take_datagrams(0);
+                let mut mirror = Mirror::new(48, Scene::stress().effect_rows(), false);
+                mirror.tick(
+                    0,
+                    t,
+                    &datagrams,
+                    &control,
+                    Quant::default(),
+                    &Records::new(&recs),
+                    s.room.client(0).and_then(|c| c.last_acked()),
+                );
+                let mut view = Vec::new();
+                mirror.view(0, t, &mut view);
+                let holds = |slot: u8| view.iter().any(|(x, _)| *x == slot);
+                println!(
+                    "p2: slot 0 at {p:?} (area {}); slot 20 at {:?}, slot 21 at {:?}; the join FullState of tick {t} \
+                     holds slot 20 {}, slot 21 {} ({} soldiers in view)",
+                    rm.area,
+                    a.position,
+                    b.position,
+                    holds(20),
+                    holds(21),
+                    view.len()
+                );
+                return Some((holds(20), holds(21)));
+            }
+        }
+        None
+    }
+
+    #[test]
+    fn p2_the_mirror_s_radius_copy_is_the_room_s_by_placement() {
+        match radius_pin() {
+            Some(held) => assert_eq!(held, (true, false), "at the radius held, one cm beyond not"),
+            // The placement exists at the head, so a None is a failure (the advisor's U4r build ruling 2, point 4 (a)).
+            None => panic!(
+                "no direction lands both placements in slot 0's area: the copy is no longer pinned"
+            ),
+        }
+    }
+
+    /// The advisor's U4r build ruling, point 9 (a): `--lead` and `--every-step` are refused with `--record` too.
+    #[test]
+    fn p9_lead_and_every_step_are_refused_with_record() {
+        let dir = scratch("p9");
+        let line = |extra: &str| {
+            format!(
+                "--run t --set t --date 2026-11-13 --commit {COMMIT} --out {} --scene SS --seed 1 --profile P0 \
+                 --record {}/r.rec {extra}",
+                dir.display(),
+                dir.display()
+            )
+        };
+        let argv = |l: String| l.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
+        assert!(parse(&argv(line(""))).is_ok(), "a recording run");
+        for extra in ["--lead 3", "--every-step 1"] {
+            let e = parse(&argv(line(extra))).expect_err(extra);
+            assert!(
+                e.contains("a recording run (--record) takes neither"),
+                "{extra}: {e}"
+            );
+        }
+    }
+
+    /// The measured phase's sum of `tick.s.snap_incomplete` in a run's client log, and whether its lab log holds -57's
+    /// series.
+    fn incomplete_and_57(dir: &Path) -> (i64, bool) {
+        let client = read(dir, "client");
+        let sum = client
+            .ticks
+            .iter()
+            .filter(|t| t.ph == Phase::Measured)
+            .filter_map(|t| t.s.get("snap_incomplete"))
+            .flatten()
+            .sum();
+        let lab = read(dir, "lab");
+        (
+            sum,
+            lab.ticks.iter().any(|t| t.s.contains_key("pos_err_um")),
+        )
+    }
+
+    /// A short L1 P0 run: SS, 16 clients, 6 s of warm-up, 2 s measured.
+    const P0_SHORT: &str =
+        "--seed 2 --profile P0 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10";
+
+    /// The advisor's U4r build ruling, points 3 and 4 (the clean half): a short L1 P0 run whose header lists
+    /// PM-0001-57 writes -57's series and files no tick lost in the measured phase (sum 0); one whose header does not
+    /// list it writes none of them.
+    #[test]
+    fn p3_p4_a_short_p0_run_writes_57_s_series_only_when_listed_and_files_no_tick_lost() {
+        let with = scratch("p3-with");
+        run(&settings(
+            &with,
+            &format!("{P0_SHORT} --marks PM-0001-17,PM-0001-57"),
+        ))
+        .expect("the run");
+        let without = scratch("p3-without");
+        run(&settings(
+            &without,
+            &format!("{P0_SHORT} --marks PM-0001-17"),
+        ))
+        .expect("the run");
+        let (sum_with, series_with) = incomplete_and_57(&with);
+        let (sum_without, series_without) = incomplete_and_57(&without);
+        println!(
+            "p3/p4 clean: listing -57: measured incomplete {sum_with}, -57 series {series_with}; not listing it: \
+             {sum_without}, {series_without}"
+        );
+        assert_eq!((sum_with, sum_without), (0, 0));
+        assert!(series_with && !series_without);
+    }
+
+    /// The advisor's U4r build ruling, point 4 (carry 62 (h)'s bite, PM-0001-17/whole-as-lost; bites builds only): the
+    /// bitten writer files each client's first tick resolved whole in the measured phase as lost, so the measured
+    /// phase's sum of `tick.s.snap_incomplete` is the client count (the clean run's is 0: the test above).
+    #[cfg(feature = "bites")]
+    #[test]
+    fn p4_bite_whole_as_lost_makes_the_measured_incomplete_sum_the_client_count() {
+        let dir = scratch("p4-bite");
+        let s = settings(
+            &dir,
+            &format!("{P0_SHORT} --marks PM-0001-17 --role bite --bite PM-0001-17/whole-as-lost"),
+        );
+        run(&s).expect("the run");
+        let (sum, _) = incomplete_and_57(&dir);
+        println!("p4 bite: measured incomplete {sum} of 16 clients");
+        assert_eq!(sum, 16);
+    }
+}
````

### npt-lab: logs.rs (new: the scenario's three logs)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs`:  1 file changed, 786 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs
new file mode 100644
index 0000000..66761d6
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs
@@ -0,0 +1,786 @@
+//! The scenario's three logs (u4-build-plan.md §4.12 :410-415; marks/README.md §1 and §4; u4r-brief.md 2.1): on L1
+//! one npt-lab process writes one log per process value the checks select on, each with its own header: `room` (the
+//! room's per-tick World hash `h`, `hs.server_hash`, PM-0001-2's series, the `admit` events, `burst_eviction`), `client`
+//! (every client's `hs.decoded_hash` and `tick.s.snap_incomplete`, one entry per slot) and `lab` (-57's errors,
+//! `burst_heal`, `counter_wrap`, the run's own records). Every tick and second record carries all its counters, 0
+//! included; every log ends with its footer.
+//!
+//! **The client writer** (carry 62 (a) to (f); carry 73; README §1's `hs` rules): each client's hashes are filed under
+//! the tick it rebuilt (a 32-bit tick, unwrapped forward from the room's first tick, so a wrong rebuild past the 16-bit
+//! wrap lands after every tick the room ran and is written there, in tick order, never dropped and never `crash`); two
+//! different hashes for one tick make it the sentinel; the ticks the client resolves lost are `null` with
+//! `snap_incomplete` 1. A tick is written once every client admitted at or before it has resolved it (or it lies
+//! before that client's first resolved tick); at the stop, inside each client's span, a tick the room logged a hash
+//! for and the client left unresolved is decided lost (on L1 the runner decides it there: nothing more can arrive), and
+//! the ticks after its last resolved one (in flight) are not written. -57's errors of a (client, tick) go into the lab
+//! log's record of that tick only when that client filed a hash for it that is not the sentinel.
+
+use std::collections::BTreeMap;
+use std::fs::File;
+use std::io::BufWriter;
+
+use npt_log::{Ended, EventRecord, Header, Phase, SCHEMA, SecondRecord, TickRecord, Writer};
+use npt_room::Room;
+use npt_sim::{EntityKind, World};
+
+use crate::mirror::{Errors, Records, SetStats};
+
+/// The fields every log of a run shares, and its params.
+#[derive(Clone, Debug)]
+pub(crate) struct Meta {
+    pub(crate) run: String,
+    pub(crate) set: String,
+    pub(crate) role: String,
+    pub(crate) marks: Vec<String>,
+    pub(crate) date: String,
+    pub(crate) commit: String,
+    pub(crate) layer: String,
+    pub(crate) profile: String,
+    pub(crate) seed: u64,
+    pub(crate) scene: String,
+    pub(crate) clients: u32,
+    pub(crate) bite: Option<String>,
+    pub(crate) params: BTreeMap<String, serde_json::Value>,
+}
+
+impl Meta {
+    fn header(&self, process: &str) -> Result<Header, String> {
+        Ok(Header {
+            schema: SCHEMA,
+            run: self.run.clone(),
+            set: self.set.clone(),
+            role: self.role.clone(),
+            marks: self.marks.clone(),
+            machine: crate::machine::container(),
+            build: crate::machine::build(&self.commit)?,
+            bite: self.bite.clone(),
+            layer: self.layer.clone(),
+            profile: self.profile.clone(),
+            seed: self.seed,
+            tick_hz: npt_sim::TICK_HZ,
+            clients: self.clients,
+            scene: self.scene.clone(),
+            process: process.to_owned(),
+            slot: 0,
+            date: self.date.clone(),
+            params: self.params.clone(),
+        })
+    }
+}
+
+type Out = Writer<Box<dyn std::io::Write>>;
+
+/// One log in `dir` (never overwritten), or with no folder a sink (a runner recording regenerated inside `npt-lab
+/// hash` keeps no log: carry 82).
+fn open(dir: Option<&std::path::Path>, process: &str, meta: &Meta) -> Result<Out, String> {
+    let out: Box<dyn std::io::Write> = match dir {
+        None => Box::new(std::io::sink()),
+        Some(dir) => {
+            let path = dir.join(format!("{process}.jsonl"));
+            let file = File::create_new(&path)
+                .map_err(|e| format!("{}: {e} (a log is never overwritten)", path.display()))?;
+            Box::new(BufWriter::new(file))
+        }
+    };
+    Writer::new(out, &meta.header(process)?).map_err(|e| format!("{process} log: {e}"))
+}
+
+fn io(e: std::io::Error) -> String {
+    format!("writing a log: {e}")
+}
+
+/// The World's events this tick, read from its dump's `events` section (npt-sim's canonical state: `World::dump`; the
+/// section is cleared at each step and written per event, kind 1 a shot, 2 a hit, 3 a kill): (events, shots, hits
+/// and kills). `at` caches the section's index (the sections before it are fixed-size pools). The advisor's U4r build
+/// ruling, point 6, pins it against `SimCore::step`'s returned events (the test below).
+pub(crate) fn dump_events(
+    world: &World,
+    at: &mut Option<usize>,
+) -> Result<(usize, i64, i64), String> {
+    let dump = world.dump();
+    let words = dump.words();
+    let a = match *at {
+        Some(a) => a,
+        None => {
+            let a = (0..words.len())
+                .find(|&i| dump.field(i).as_deref() == Some("events[0].count"))
+                .ok_or("the World's dump has no events count")?;
+            *at = Some(a);
+            a
+        }
+    };
+    let events = words.get(a).copied().unwrap_or(0) as usize;
+    let kind = |i: usize| words.get(a + 1 + 4 * i).copied().unwrap_or(0);
+    let shots = (0..events).filter(|&i| kind(i) == 1).count() as i64;
+    let reliable = (0..events)
+        .filter(|&i| kind(i) == 2 || kind(i) == 3)
+        .count() as i64;
+    Ok((events, shots, reliable))
+}
+
+/// One (client, tick)'s resolution.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum Res {
+    Open,
+    Hash(u64),
+    Lost,
+}
+
+/// What the room's series need from the tick before.
+#[derive(Default)]
+struct RoomTrack {
+    doors: BTreeMap<u16, i32>,
+    projectiles: BTreeMap<u16, (u16, [i32; 12])>,
+    late: Vec<u64>,
+    refused: Vec<[u64; 4]>,
+    events_at: Option<usize>,
+    second: Option<(u64, Phase, [i64; 6])>,
+}
+
+/// The run's logs (the module text).
+pub(crate) struct Logs {
+    room: Out,
+    client: Out,
+    lab: Out,
+    n: usize,
+    first_tick: u32,
+    phases: BTreeMap<u64, Phase>,
+    res: BTreeMap<u64, Vec<Res>>,
+    /// Per slot: the tick it was admitted at, its first and last resolved ticks, and whether it left the run.
+    admitted: Vec<Option<u64>>,
+    first: Vec<Option<u64>>,
+    last: Vec<Option<u64>>,
+    gone: Vec<bool>,
+    errors: BTreeMap<(u64, u8), Errors>,
+    /// Per slot, the ticks the room logged a hash for it, beyond the written mark (for the stop's decision).
+    logged: BTreeMap<u64, Vec<bool>>,
+    written: Option<u64>,
+    track: RoomTrack,
+    wraps: BTreeMap<&'static str, u64>,
+    /// carry 62 (h)'s bite, PM-0001-17/whole-as-lost (the advisor's U4r build ruling, point 4): on every client, its
+    /// first tick resolved whole in the measured phase filed as lost (null, `snap_incomplete` 1); per slot, that tick.
+    whole_as_lost: bool,
+    bitten_at: Vec<Option<u64>>,
+    pub(crate) counts: Counts,
+}
+
+/// The run's running totals, for its summary line and the lab log's last records.
+#[derive(Clone, Debug, Default)]
+pub(crate) struct Counts {
+    pub(crate) incomplete_measured: u64,
+    pub(crate) hashes: u64,
+    pub(crate) sentinels: u64,
+    pub(crate) lost_decided_at_stop: u64,
+    pub(crate) conflicts: u64,
+    pub(crate) shots_measured: i64,
+    pub(crate) measured_seconds: i64,
+    pub(crate) reliable_measured: i64,
+    pub(crate) everyone_measured: u64,
+    pub(crate) large_measured: u64,
+    pub(crate) projectiles_measured: u64,
+    pub(crate) sets_measured: u64,
+}
+
+impl Logs {
+    /// Opens the three logs in `dir` (each `<process>.jsonl`, never overwritten), or three sinks with no folder.
+    pub(crate) fn open(
+        dir: Option<&std::path::Path>,
+        meta: &Meta,
+        first_tick: u32,
+        whole_as_lost: bool,
+    ) -> Result<Logs, String> {
+        let n = meta.clients as usize;
+        Ok(Logs {
+            room: open(dir, "room", meta)?,
+            client: open(dir, "client", meta)?,
+            lab: open(dir, "lab", meta)?,
+            n,
+            first_tick,
+            phases: BTreeMap::new(),
+            res: BTreeMap::new(),
+            admitted: vec![None; n],
+            first: vec![None; n],
+            last: vec![None; n],
+            gone: vec![false; n],
+            errors: BTreeMap::new(),
+            logged: BTreeMap::new(),
+            written: None,
+            track: RoomTrack::default(),
+            wraps: BTreeMap::new(),
+            whole_as_lost,
+            bitten_at: vec![None; n],
+            counts: Counts::default(),
+        })
+    }
+
+    /// A 32-bit tick as the log's never-wrapping tick: forward from the room's first tick.
+    pub(crate) fn wide(&self, t: u32) -> u64 {
+        u64::from(self.first_tick) + u64::from(t.wrapping_sub(self.first_tick))
+    }
+
+    /// An event in the lab log.
+    pub(crate) fn lab_event(&mut self, e: &EventRecord) -> Result<(), String> {
+        self.lab.event(e).map_err(io)
+    }
+
+    /// An event in the room log.
+    pub(crate) fn room_event(&mut self, e: &EventRecord) -> Result<(), String> {
+        self.room.event(e).map_err(io)
+    }
+
+    /// The room admitted `slot` (its session `session`) before tick `t` ran.
+    pub(crate) fn admit(
+        &mut self,
+        slot: u8,
+        session: u64,
+        t: u32,
+        ph: Phase,
+    ) -> Result<(), String> {
+        let w = self.wide(t);
+        if let Some(a) = self.admitted.get_mut(usize::from(slot)) {
+            a.get_or_insert(w);
+        }
+        let mut e = EventRecord::new("admit", ph)
+            .with("session", session)
+            .with("in_roster", true)
+            .with("slot", slot);
+        e.t = Some(w);
+        self.room.event(&e).map_err(io)
+    }
+
+    /// Whether `slot` was admitted and has resolved a tick (carry 62 (d)'s head).
+    pub(crate) fn started(&self, slot: u8) -> bool {
+        let s = usize::from(slot);
+        self.admitted.get(s).is_some_and(Option::is_some)
+            && self.first.get(s).is_some_and(Option::is_some)
+    }
+
+    /// Whether every tick up to `w` is written (every client resolved it).
+    pub(crate) fn written_through(&self, w: u64) -> bool {
+        self.written.is_some_and(|m| m >= w)
+    }
+
+    /// `slot`'s session ended (replaced, kicked or left): its later ticks are not waited for.
+    pub(crate) fn gone(&mut self, slot: u8) {
+        if let Some(g) = self.gone.get_mut(usize::from(slot)) {
+            *g = true;
+        }
+    }
+
+    /// A counter that wrapped (`ClientSlot::wraps`, `StepOut::wraps`): one record per counter name, at its first wrap
+    /// (carry 77 (a)), in the lab log.
+    pub(crate) fn wrapped(&mut self, name: &'static str, t: u32, ph: Phase) -> Result<(), String> {
+        if self.wraps.contains_key(name) {
+            return Ok(());
+        }
+        let w = self.wide(t);
+        self.wraps.insert(name, w);
+        let mut e = EventRecord::new("counter_wrap", ph)
+            .with("counter", name)
+            .with("tick_count", w);
+        e.t = Some(w);
+        self.lab.event(&e).map_err(io)
+    }
+
+    fn entry(&mut self, w: u64) -> &mut Vec<Res> {
+        let n = self.n;
+        self.res.entry(w).or_insert_with(|| vec![Res::Open; n])
+    }
+
+    fn resolved(&mut self, slot: usize, w: u64) {
+        let f = &mut self.first[slot];
+        if f.is_none_or(|x| w < x) {
+            *f = Some(w);
+        }
+        let l = &mut self.last[slot];
+        if l.is_none_or(|x| w > x) {
+            *l = Some(w);
+        }
+    }
+
+    /// What one client step filed: (tick, hash) pairs (0 the sentinel) and the runs of ticks it resolved lost.
+    pub(crate) fn filed(&mut self, slot: u8, hashes: &[(u32, u64)], lost: &[(u32, u32)]) {
+        let s = usize::from(slot);
+        if s >= self.n {
+            return;
+        }
+        for &(t, h) in hashes {
+            let w = self.wide(t);
+            if self.written.is_some_and(|m| w <= m) {
+                // Never dropped silently: counted, and the log ends `crash` (README §1).
+                self.counts.conflicts += 1;
+                continue;
+            }
+            // The bite: a whole decode (not the sentinel) of the client's first measured tick, and every later filing
+            // of that tick, goes in as lost.
+            let as_lost = self.whole_as_lost
+                && h != 0
+                && match self.bitten_at[s] {
+                    Some(b) => b == w,
+                    None => self.phases.get(&w) == Some(&Phase::Measured),
+                };
+            if as_lost {
+                self.bitten_at[s] = Some(w);
+            }
+            let e = &mut self.entry(w)[s];
+            *e = match (*e, as_lost) {
+                (_, true) => Res::Lost,
+                (Res::Open | Res::Lost, false) => Res::Hash(h),
+                (Res::Hash(x), false) if x == h => Res::Hash(x),
+                (Res::Hash(_), false) => Res::Hash(0),
+            };
+            self.resolved(s, w);
+        }
+        for &(first, count) in lost {
+            for k in 0..count {
+                let w = self.wide(first.wrapping_add(k));
+                if self.written.is_some_and(|m| w <= m) {
+                    continue;
+                }
+                let e = &mut self.entry(w)[s];
+                if *e == Res::Open {
+                    *e = Res::Lost;
+                }
+                self.resolved(s, w);
+            }
+        }
+    }
+
+    /// -57's candidate errors of (slot, tick), kept until that client's resolution of the tick is written.
+    pub(crate) fn errors(&mut self, slot: u8, t: u32, e: Errors) {
+        if e.pos_um.is_empty() && e.yaw_microdeg.is_empty() && e.pitch_microdeg.is_empty() {
+            return;
+        }
+        let w = self.wide(t);
+        self.errors.insert((w, slot), e);
+    }
+
+    fn writable(&self, w: u64) -> bool {
+        (0..self.n).all(|s| {
+            let started = self.admitted[s].is_some_and(|a| a <= w);
+            if !started || self.gone[s] {
+                return true;
+            }
+            if self.first[s].is_none_or(|f| w < f) {
+                // Admitted, no tick resolved yet: wait (its first resolved tick may be this one).
+                return self.first[s].is_some();
+            }
+            self.res.get(&w).is_some_and(|r| r[s] != Res::Open)
+        })
+    }
+
+    fn write_client_tick(&mut self, w: u64) -> Result<(), String> {
+        let ph = self.phase(w);
+        let list = self
+            .res
+            .remove(&w)
+            .unwrap_or_else(|| vec![Res::Open; self.n]);
+        self.logged.remove(&w);
+        let mut errs = Errors::default();
+        for (s, r) in list.iter().enumerate() {
+            if let Res::Hash(h) = r
+                && *h != 0
+                && let Some(e) = self.errors.remove(&(w, s as u8))
+            {
+                errs.pos_um.extend(e.pos_um);
+                errs.yaw_microdeg.extend(e.yaw_microdeg);
+                errs.pitch_microdeg.extend(e.pitch_microdeg);
+            }
+        }
+        self.errors.retain(|(t, _), _| *t > w);
+        if list.iter().any(|r| *r != Res::Open) {
+            let mut rec = TickRecord::new(w, ph);
+            let hashes: Vec<Option<u64>> = list
+                .iter()
+                .map(|r| match r {
+                    Res::Hash(h) => Some(*h),
+                    _ => None,
+                })
+                .collect();
+            let incomplete: Vec<i64> = list.iter().map(|r| i64::from(*r == Res::Lost)).collect();
+            self.counts.hashes += hashes.iter().flatten().count() as u64;
+            self.counts.sentinels += hashes.iter().flatten().filter(|h| **h == 0).count() as u64;
+            if ph == Phase::Measured {
+                self.counts.incomplete_measured += incomplete.iter().sum::<i64>() as u64;
+            }
+            rec.set_hashes("decoded_hash", &hashes);
+            rec.s.insert("snap_incomplete".to_owned(), incomplete);
+            self.client.tick(&rec).map_err(io)?;
+        }
+        if !errs.pos_um.is_empty()
+            || !errs.yaw_microdeg.is_empty()
+            || !errs.pitch_microdeg.is_empty()
+        {
+            let mut rec = TickRecord::new(w, ph);
+            rec.s.insert("pos_err_um".to_owned(), errs.pos_um);
+            rec.s
+                .insert("yaw_err_microdeg".to_owned(), errs.yaw_microdeg);
+            rec.s
+                .insert("pitch_err_microdeg".to_owned(), errs.pitch_microdeg);
+            self.lab.tick(&rec).map_err(io)?;
+        }
+        self.written = Some(w);
+        Ok(())
+    }
+
+    fn phase(&self, w: u64) -> Phase {
+        self.phases
+            .get(&w)
+            .copied()
+            .or_else(|| self.phases.values().next_back().copied())
+            .unwrap_or(Phase::After)
+    }
+
+    /// Writes every tick that is complete, in order.
+    pub(crate) fn flush(&mut self) -> Result<(), String> {
+        loop {
+            let next = match self.written {
+                None => self.phases.keys().next().copied(),
+                Some(m) => self.phases.range(m + 1..).next().map(|(k, _)| *k),
+            };
+            let Some(w) = next else { return Ok(()) };
+            if !self.writable(w) {
+                return Ok(());
+            }
+            self.write_client_tick(w)?;
+        }
+    }
+
+    /// The room ran tick `t` in phase `ph`: its record (the World hash, `hs.server_hash`, PM-0001-2's series, the
+    /// sizes), from the room after the tick; `sets` is each slot's set as the mirror rebuilt it; `datagrams` each
+    /// slot's snapshot datagrams' payload sizes; `inputs` the `Input` datagrams' sizes the clients sent since the last
+    /// tick. Returns the events the World stamped at the tick.
+    pub(crate) fn room_tick(
+        &mut self,
+        room: &Room<World>,
+        t: u32,
+        ph: Phase,
+        sets: &[SetStats],
+        datagrams: &[Vec<usize>],
+        inputs: &[i64],
+    ) -> Result<u64, String> {
+        let w = self.wide(t);
+        self.phases.insert(w, ph);
+        let world = room.sim();
+        let records = npt_sim::SimCore::entity_records(world);
+        let scene = world.scene();
+        let index = Records::new(records);
+        let mut rec = TickRecord::new(w, ph);
+        rec.set_hash(npt_sim::SimCore::state_hash(world));
+        // PM-0001-2's room series.
+        let count = |k: EntityKind| records.iter().filter(|r| r.kind == k).count() as i64;
+        let mut teams: BTreeMap<u8, i64> = BTreeMap::new();
+        for r in records.iter().filter(|r| r.kind == EntityKind::Soldier) {
+            if let Some(team) = u8::try_from(r.index).ok().and_then(|s| scene.team(s)) {
+                *teams.entry(team).or_default() += 1;
+            }
+        }
+        let rows = u16::from(scene.effect_rows().max(1));
+        let mut effects: BTreeMap<u16, i64> = BTreeMap::new();
+        for r in records.iter().filter(|r| r.kind == EntityKind::Effect) {
+            if (r.grid[1] as u32).wrapping_sub(t) as i32 > 0 {
+                *effects.entry(r.index / rows).or_default() += 1;
+            }
+        }
+        let n = self.n;
+        let mut server = vec![None; n];
+        let mut clients_count = 0i64;
+        let mut late = 0u64;
+        self.track.late.resize(n, 0);
+        self.track.refused.resize(n, [0; 4]);
+        let mut refused_events = Vec::new();
+        for (s, h) in server.iter_mut().enumerate() {
+            let Some(c) = room.client(s as u8) else {
+                continue;
+            };
+            *h = c.server_hash();
+            if h.is_some() {
+                self.logged.entry(w).or_insert_with(|| vec![false; n])[s] = true;
+            }
+            let k = c.counters();
+            if !c.kicked() && k.snapshots > 0 {
+                clients_count += 1;
+            }
+            late += k.late.saturating_sub(self.track.late[s]);
+            self.track.late[s] = k.late;
+            let now = [
+                k.refused_out_of_range,
+                k.refused_too_many,
+                k.refused_too_large,
+                k.refused_not_canonical,
+            ];
+            for (i, kind) in ["out_of_range", "too_many", "too_large", "not_canonical"]
+                .iter()
+                .enumerate()
+            {
+                if now[i] > self.track.refused[s][i] {
+                    refused_events.push((s, *kind));
+                }
+            }
+            self.track.refused[s] = now;
+        }
+        rec.set_hashes("server_hash", &server);
+        let v = &mut rec.v;
+        v.insert("soldiers_count".into(), count(EntityKind::Soldier));
+        v.insert(
+            "smallest_team_count".into(),
+            teams.values().copied().min().unwrap_or(0),
+        );
+        v.insert("snapshot_clients_count".into(), clients_count);
+        v.insert("big_vehicles_count".into(), count(EntityKind::Vehicle));
+        v.insert("very_large_units_count".into(), index.large_units() as i64);
+        v.insert("projectiles_count".into(), count(EntityKind::Projectile));
+        v.insert(
+            "effects_per_soldier_max_count".into(),
+            effects.values().copied().max().unwrap_or(0),
+        );
+        v.insert("late_count".into(), late as i64);
+        let flags: Vec<i64> = (0..n)
+            .map(|s| i64::from(sets.get(s).is_some_and(|x| x.everyone)))
+            .collect();
+        rec.s.insert("everyone_in_view".into(), flags);
+        rec.s.insert(
+            "snap_datagram_bytes".into(),
+            datagrams.iter().flatten().map(|b| *b as i64).collect(),
+        );
+        rec.s.insert(
+            "snap_wire_bytes".into(),
+            datagrams
+                .iter()
+                .map(|d| d.iter().map(|b| *b as i64 + 28).sum())
+                .collect(),
+        );
+        rec.s.insert("input_payload_bytes".into(), inputs.to_vec());
+        if ph == Phase::Measured {
+            for s in sets.iter().take(n) {
+                self.counts.sets_measured += 1;
+                self.counts.everyone_measured += u64::from(s.everyone);
+                self.counts.large_measured += u64::from(s.large_units);
+                self.counts.projectiles_measured += u64::from(s.projectiles);
+            }
+        }
+        self.room.tick(&rec).map_err(io)?;
+        for (s, kind) in refused_events {
+            let mut e = EventRecord::new("snap_refused", ph)
+                .with("slot", s)
+                .with("kind", kind)
+                .with("tick_count", w);
+            e.t = Some(w);
+            self.room.event(&e).map_err(io)?;
+        }
+        self.room_second(world, t, ph, records)
+    }
+
+    /// The per-second counts: shots, gameplay events (hits and kills) and projectile spawns from the World's own event
+    /// list and records (the dump's `events` section: npt-sim's canonical state), portal changes from the doors.
+    fn room_second(
+        &mut self,
+        world: &World,
+        t: u32,
+        ph: Phase,
+        records: &[npt_sim::EntityRecord],
+    ) -> Result<u64, String> {
+        let (events, shots, reliable) = dump_events(world, &mut self.track.events_at)?;
+        let mut portal = 0i64;
+        let mut spawns = 0i64;
+        let mut stand_in_spawns = 0i64;
+        let mut stand_in_changes = 0i64;
+        let mut live = BTreeMap::new();
+        for r in records {
+            match r.kind {
+                EntityKind::Door => {
+                    if self
+                        .track
+                        .doors
+                        .insert(r.index, r.grid[0])
+                        .is_some_and(|o| o != r.grid[0])
+                    {
+                        portal += 1;
+                    }
+                }
+                EntityKind::Projectile => {
+                    let stand_in = r.grid[7] == 0;
+                    match self.track.projectiles.get(&r.index) {
+                        Some((g, grid)) if *g == r.generation => {
+                            if stand_in && *grid != r.grid {
+                                stand_in_changes += 1;
+                            }
+                        }
+                        _ => {
+                            spawns += 1;
+                            stand_in_spawns += i64::from(stand_in);
+                        }
+                    }
+                    live.insert(r.index, (r.generation, r.grid));
+                }
+                _ => {}
+            }
+        }
+        self.track.projectiles = live;
+        let sec = u64::from(t.wrapping_sub(self.first_tick)) / u64::from(npt_sim::TICK_HZ);
+        let add = [
+            shots,
+            portal,
+            reliable,
+            spawns,
+            stand_in_changes,
+            stand_in_spawns,
+        ];
+        match &mut self.track.second {
+            Some((s, p, acc)) if *s == sec => {
+                *p = ph;
+                for (a, x) in acc.iter_mut().zip(add) {
+                    *a += x;
+                }
+            }
+            _ => {
+                if let Some((s, p, acc)) = self.track.second.take() {
+                    self.write_second(s, p, acc)?;
+                }
+                self.track.second = Some((sec, ph, add));
+            }
+        }
+        Ok(events as u64)
+    }
+
+    fn write_second(&mut self, sec: u64, ph: Phase, acc: [i64; 6]) -> Result<(), String> {
+        let mut r = SecondRecord::new(sec, ph);
+        for (name, v) in [
+            "shots_count",
+            "portal_changes_count",
+            "reliable_events_count",
+            "projectile_spawns_count",
+            "stand_in_changes_count",
+            "stand_in_spawns_count",
+        ]
+        .into_iter()
+        .zip(acc)
+        {
+            r.v.insert(name.to_owned(), v);
+        }
+        if ph == Phase::Measured {
+            self.counts.shots_measured += acc[0];
+            self.counts.reliable_measured += acc[2];
+            self.counts.measured_seconds += 1;
+        }
+        self.room.second(&r).map_err(io)
+    }
+
+    /// The stop: inside each client's span the ticks the room logged a hash for and the client left unresolved are
+    /// decided lost (the module text), every complete tick written, then the footers. `ended` is `Clean`, or `Crash`
+    /// when a hash came after its tick was written (never silently dropped).
+    pub(crate) fn finish(mut self, ended: Ended, last: &[EventRecord]) -> Result<Counts, String> {
+        if let Some((s, p, acc)) = self.track.second.take() {
+            self.write_second(s, p, acc)?;
+        }
+        let logged: Vec<(u64, Vec<bool>)> =
+            self.logged.iter().map(|(w, l)| (*w, l.clone())).collect();
+        for (w, list) in logged {
+            for (s, room_had) in list.iter().enumerate() {
+                let inside =
+                    self.first[s].is_some_and(|f| f <= w) && self.last[s].is_some_and(|l| w <= l);
+                if *room_had && inside && !self.gone[s] {
+                    let e = &mut self.entry(w)[s];
+                    if *e == Res::Open {
+                        *e = Res::Lost;
+                        self.counts.lost_decided_at_stop += 1;
+                    }
+                }
+            }
+        }
+        // Every slot's later ticks are in flight: not waited for.
+        self.gone.fill(true);
+        self.flush()?;
+        let rest: Vec<u64> = self
+            .res
+            .keys()
+            .copied()
+            .filter(|w| self.written.is_none_or(|m| *w > m))
+            .collect();
+        for w in rest {
+            self.write_client_tick(w)?;
+        }
+        for e in last {
+            self.lab.event(e).map_err(io)?;
+        }
+        let ended = if self.counts.conflicts > 0 {
+            Ended::Crash
+        } else {
+            ended
+        };
+        self.room.finish(ended).map_err(io)?;
+        self.client.finish(ended).map_err(io)?;
+        self.lab.finish(ended).map_err(io)?;
+        Ok(self.counts)
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use npt_sim::{Command, HitHistory, Scene, SimCore, SimEvent, StandIns, TickInput};
+
+    /// The advisor's U4r build ruling, point 6: the dump reader's counts at each tick equal the shots, hits and kills
+    /// in `SimCore::step`'s returned events, over a short seeded run of the stress scene in which every soldier fires
+    /// at one soldier of another team (team 0 at slot 16, the others at slot 0), so there are kills.
+    #[test]
+    fn p6_the_dump_reader_counts_the_shots_hits_and_kills_simcore_step_returns() {
+        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
+        let mut world = World::new(&scene, 3, 0);
+        let history = HitHistory::for_ms(2_000).expect("a ring");
+        let mut at = None;
+        let (mut shots, mut hits, mut kills) = (0i64, 0i64, 0i64);
+        for k in 0..600u32 {
+            let soldiers: BTreeMap<u16, [i32; 3]> = SimCore::entity_records(&world)
+                .iter()
+                .filter(|r| r.kind == EntityKind::Soldier)
+                .map(|r| (r.index, [r.grid[0], r.grid[1], r.grid[2]]))
+                .collect();
+            let commands = (0..48u8)
+                .filter_map(|s| {
+                    let p = soldiers.get(&if s < 16 { 16 } else { 0 })?;
+                    let c = Command {
+                        slot: s,
+                        tick: k,
+                        seq: k as u16,
+                        buttons: Command::FIRE,
+                        aim: [p[0], p[1], p[2] + 100],
+                        view_tick: k,
+                        ..Command::default()
+                    };
+                    Some((c, None))
+                })
+                .collect();
+            let input = TickInput {
+                tick: k,
+                commands,
+                ..TickInput::default()
+            };
+            let (mut s, mut r) = (0i64, 0i64);
+            for e in world.step(&input, &history) {
+                match e {
+                    SimEvent::Shot { .. } => s += 1,
+                    SimEvent::Hit { .. } => {
+                        r += 1;
+                        hits += 1;
+                    }
+                    SimEvent::Kill { .. } => {
+                        r += 1;
+                        kills += 1;
+                    }
+                }
+            }
+            shots += s;
+            let (_, rs, rr) = dump_events(&world, &mut at).expect("the reader");
+            assert_eq!(
+                (rs, rr),
+                (s, r),
+                "tick {k}: the reader's shots and hits plus kills"
+            );
+        }
+        println!(
+            "p6: 600 ticks; shots {shots}, hits {hits}, kills {kills}, each tick's counts equal"
+        );
+        assert!(shots > 0 && hits > 0 && kills > 0, "{shots} {hits} {kills}");
+    }
+}
````

### npt-lab: mirror.rs (new: the room-side frame mirror; r4's class audit)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/mirror.rs`:  1 file changed, 490 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/mirror.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/mirror.rs
new file mode 100644
index 0000000..8e542eb
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/mirror.rs
@@ -0,0 +1,490 @@
+//! The room-side frame mirror (u4r-brief.md 2.1, 2.4 r4; catalogue §6 item 5): each client's snapshots and full
+//! states as the room handed them out (`take_datagrams`, `take_control`), decoded with npt-proto and the `Quant` that
+//! client decodes with, and rebuilt into the frame the room meant that client to hold at each tick (its baseline's
+//! frame plus the tick's deltas, every group of it, before the link). npt-client exposes no decoded record (R16), so
+//! the lab reads what it needs here:
+//! - **the client's set** at each tick: whether it holds every other soldier (ES: and every vehicle), PM-0001-2's
+//!   `tick.s.everyone_in_view`; whether it holds all of the very large units (every part of each); how many
+//!   projectiles (carry 49);
+//! - **-57's pairing** (§4.6 :327-328; u4r-brief.md 2.1): per (client, tick, entity), only the fields that tick's
+//!   snapshot carried (an `Add`'s whole record, an `Update`'s changed fields), each grid integer's value against the
+//!   room's pre-grid value at that tick (`EntityRecord::raw`, carry 9); the scenario keeps a tick's errors only once
+//!   that client filed a hash for the tick that is not the sentinel, so only decoded fields count;
+//! - **r4's class audit**: every record of the own-state section is the client's own (its soldier, its `OwnState`, its
+//!   effect rows); no other record names it; every spatial record lies inside that client's interest radius at that
+//!   tick (the room's rule, its radius only: the flood is npt-room's, crate-private); no cosmetic shot of the client's
+//!   own reaches it. Snapshots and full states have no team or public section in the spike (catalogue :144-145,
+//!   :153-154), so those two rules cannot fire: reported as not exercised.
+
+use std::collections::{BTreeMap, VecDeque};
+
+use npt_link::Channel;
+use npt_proto::{Direction, EntityDelta, Message, Quant};
+use npt_sim::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, YAW};
+use npt_sim::{EntityKind, EntityRecord, SimEvent};
+
+/// The room's interest radius, cm: a copy of npt-room's private `RADIUS_CM` (room/src/room.rs:29), which r4's spatial
+/// rule checks (the report names the copy).
+pub(crate) const INTEREST_RADIUS_CM: i64 = 6_400;
+/// A very large unit's parts, at least (PM-0001-2: "4 very large units of at least 20 parts").
+pub(crate) const LARGE_UNIT_PARTS: usize = 20;
+/// Frames kept per client: the ack ring's ticks and two more (a baseline is at most the ring old). Beyond them each
+/// client also keeps the frame of the room's `ClientSlot::last_acked()` (room.rs:238), as the room's own stale copy
+/// does: under PM-0001-18/no-fallback the room codes against that tick's frame after it left the ring (ack.rs `stale`,
+/// taken only while its tick is `last_acked`), at ages 33 and more (the advisor's U4r build ruling 3, point 8 (A)).
+const FRAMES_KEPT: usize = npt_sim::ACK_RING_TICKS as usize + 2;
+
+type Frame = BTreeMap<(EntityKind, u16), EntityRecord>;
+
+/// One (client, tick)'s -57 errors: positions in µm, yaw and pitch in µdeg, one per carried field group.
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct Errors {
+    pub(crate) pos_um: Vec<i64>,
+    pub(crate) yaw_microdeg: Vec<i64>,
+    pub(crate) pitch_microdeg: Vec<i64>,
+}
+
+/// What one client's set held at one tick (from the frame the room meant it to hold).
+#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
+pub(crate) struct SetStats {
+    /// Every other soldier (ES: and every vehicle) in the set.
+    pub(crate) everyone: bool,
+    /// Every very large unit's hull and parts in the set.
+    pub(crate) large_units: bool,
+    pub(crate) projectiles: u32,
+}
+
+/// r4's counts: records audited per class, and the first breaches (the advisor's U4r build ruling, point 2).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct Audit {
+    pub(crate) owner: u64,
+    /// Entity-section records checked against the room's radius (soldiers, hulls, parts, projectiles).
+    pub(crate) spatial: u64,
+    /// Entity-section records not checked against it: doors, the wall, and any record that tick's room records lack.
+    pub(crate) spatial_unchecked: u64,
+    pub(crate) public: u64,
+    pub(crate) events: u64,
+    /// The first 16 breaches' texts, and every breach counted.
+    pub(crate) breaches: Vec<String>,
+    pub(crate) breaches_count: u64,
+}
+
+#[derive(Clone, Debug, Default)]
+struct ClientMirror {
+    frames: VecDeque<(u32, Frame)>,
+    latest: SetStats,
+}
+
+impl ClientMirror {
+    fn frame(&self, tick: u32) -> Option<&Frame> {
+        self.frames.iter().find(|(t, _)| *t == tick).map(|(_, f)| f)
+    }
+
+    /// Keeps `frame` as the room's frame of `tick`: the newest [`FRAMES_KEPT`] frames, and beyond them the frame of
+    /// `pin`, the room's `last_acked()` for this client (the constant's text). In a clean run the baseline is always
+    /// inside the ring, so the frames looked up are the same as with no pin.
+    fn keep(&mut self, tick: u32, frame: Frame, pin: Option<u32>) {
+        self.frames.retain(|(t, _)| *t != tick);
+        self.frames.push_back((tick, frame));
+        let unpinned = |t: u32| Some(t) != pin;
+        while self.frames.iter().filter(|(t, _)| unpinned(*t)).count() > FRAMES_KEPT {
+            let Some(i) = self.frames.iter().position(|(t, _)| unpinned(*t)) else {
+                break;
+            };
+            self.frames.remove(i);
+        }
+    }
+}
+
+/// The room-side records of one tick, indexed for the mirror.
+pub(crate) struct Records<'a> {
+    by_key: BTreeMap<(EntityKind, u16), &'a EntityRecord>,
+    /// Each very large unit's hull index and its part indices.
+    large: Vec<(u16, Vec<u16>)>,
+    /// Soldier slots and vehicle hulls, for the everyone-in-view flag.
+    soldiers: Vec<u16>,
+    vehicles: Vec<u16>,
+}
+
+impl<'a> Records<'a> {
+    pub(crate) fn new(records: &'a [EntityRecord]) -> Records<'a> {
+        let mut by_key = BTreeMap::new();
+        let mut parts: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
+        let mut soldiers = Vec::new();
+        let mut vehicles = Vec::new();
+        for r in records {
+            by_key.insert((r.kind, r.index), r);
+            match r.kind {
+                EntityKind::Soldier => soldiers.push(r.index),
+                EntityKind::Vehicle => vehicles.push(r.index),
+                EntityKind::VehiclePart => {
+                    if let Ok(hull) = u16::try_from(r.grid[5]) {
+                        parts.entry(hull).or_default().push(r.index);
+                    }
+                }
+                _ => {}
+            }
+        }
+        let large = parts
+            .into_iter()
+            .filter(|(_, p)| p.len() >= LARGE_UNIT_PARTS)
+            .collect();
+        Records {
+            by_key,
+            large,
+            soldiers,
+            vehicles,
+        }
+    }
+
+    /// The very large units (hulls of at least 20 parts) this tick.
+    pub(crate) fn large_units(&self) -> usize {
+        self.large.len()
+    }
+
+    fn get(&self, kind: EntityKind, index: u16) -> Option<&EntityRecord> {
+        self.by_key.get(&(kind, index)).copied()
+    }
+}
+
+/// The value grid integer `k` of `field` stands for (the wire grid's, as the client holds it).
+fn value(field: Field, k: i32) -> f64 {
+    f64::from(Config::default().from_grid(field, k))
+}
+
+/// The room's pre-grid value of field `i` of `r` (`raw` holds it by its f32 bits).
+fn raw(r: &EntityRecord, i: usize) -> f64 {
+    f64::from(f32::from_bits(r.raw[i]))
+}
+
+/// An angle's difference brought into [-π, π].
+fn turn(d: f64) -> f64 {
+    let t = std::f64::consts::TAU;
+    let d = d.rem_euclid(t);
+    if d > t / 2.0 { d - t } else { d }
+}
+
+/// -57's errors for one carried record `sent` (the grid integers the client decodes), `changed` its carried fields'
+/// mask, against the room's record `room` of the same entity at that tick.
+fn errors(sent: &EntityRecord, changed: u16, room: &EntityRecord, out: &mut Errors) {
+    let layout = sent.kind.layout();
+    let carried = |i: usize| i < layout.len() && changed & (1 << i) != 0;
+    let mut pos2 = 0.0f64;
+    let mut any_pos = false;
+    for (i, f) in layout.iter().enumerate() {
+        if !carried(i) {
+            continue;
+        }
+        if *f == POSITION_XY || *f == POSITION_Z {
+            let e = value(*f, sent.grid[i]) - raw(room, i);
+            pos2 += e * e;
+            any_pos = true;
+        } else if *f == YAW || *f == PITCH {
+            let e = turn(value(*f, sent.grid[i]) - raw(room, i)).abs();
+            let microdeg = (e.to_degrees() * 1e6).ceil() as i64;
+            if *f == YAW {
+                out.yaw_microdeg.push(microdeg);
+            } else {
+                out.pitch_microdeg.push(microdeg);
+            }
+        }
+    }
+    if any_pos {
+        out.pos_um.push((pos2.sqrt() * 1e6).ceil() as i64);
+    }
+}
+
+/// Every client's mirror (the module text).
+pub(crate) struct Mirror {
+    clients: Vec<ClientMirror>,
+    slots: u8,
+    effect_rows: u8,
+    /// ES's flag also needs every vehicle (PM-0001-2's ES row, carry 59).
+    with_vehicles: bool,
+    pub(crate) audit: Audit,
+}
+
+impl Mirror {
+    /// The mirror of `slots` clients of a scene with `effect_rows` per soldier; `with_vehicles` for ES.
+    pub(crate) fn new(slots: u8, effect_rows: u8, with_vehicles: bool) -> Mirror {
+        Mirror {
+            clients: vec![ClientMirror::default(); usize::from(slots)],
+            slots,
+            effect_rows,
+            with_vehicles,
+            audit: Audit::default(),
+        }
+    }
+
+    /// The latest set stats of `slot` (from its newest frame at or before the last tick seen).
+    pub(crate) fn latest(&self, slot: u8) -> SetStats {
+        self.clients
+            .get(usize::from(slot))
+            .map(|c| c.latest)
+            .unwrap_or_default()
+    }
+
+    /// The soldiers `slot`'s frame of `tick` holds, with their positions (the bots' view: npt-bots' `Behaviour`).
+    pub(crate) fn view(&self, slot: u8, tick: u32, into: &mut Vec<(u8, [i32; 3])>) {
+        into.clear();
+        let Some(f) = self
+            .clients
+            .get(usize::from(slot))
+            .and_then(|c| c.frame(tick))
+        else {
+            return;
+        };
+        for ((kind, index), r) in f {
+            if *kind == EntityKind::Soldier
+                && let Ok(s) = u8::try_from(*index)
+            {
+                into.push((s, [r.grid[0], r.grid[1], r.grid[2]]));
+            }
+        }
+    }
+
+    fn own(&self, slot: u8, r: &EntityRecord) -> bool {
+        let rows = u16::from(self.effect_rows.max(1));
+        match r.kind {
+            EntityKind::Soldier | EntityKind::OwnState => r.index == u16::from(slot),
+            EntityKind::Effect => self.effect_rows > 0 && r.index / rows == u16::from(slot),
+            _ => false,
+        }
+    }
+
+    fn breach(&mut self, text: String) {
+        self.audit.breaches_count += 1;
+        if self.audit.breaches.len() < 16 {
+            self.audit.breaches.push(text);
+        }
+    }
+
+    /// The own-state section's rule (owner: only the client's own records) and the entity section's (no record of
+    /// the client's own; every spatial record inside the interest radius of the client's soldier at that tick).
+    fn audit_records(
+        &mut self,
+        slot: u8,
+        tick: u32,
+        own: &[EntityRecord],
+        entities: &[EntityRecord],
+        room: &Records<'_>,
+    ) {
+        for r in own {
+            self.audit.owner += 1;
+            if !self.own(slot, r) {
+                self.breach(format!(
+                    "tick {tick}: client {slot}'s own-state section holds {:?} {} (owner: only its own slot's)",
+                    r.kind, r.index
+                ));
+            }
+        }
+        let me = room.get(EntityKind::Soldier, u16::from(slot));
+        for r in entities {
+            if self.own(slot, r) || matches!(r.kind, EntityKind::OwnState | EntityKind::Effect) {
+                self.audit.spatial_unchecked += 1;
+                self.breach(format!(
+                    "tick {tick}: client {slot}'s entity section holds {:?} {} (an owner-only record)",
+                    r.kind, r.index
+                ));
+                continue;
+            }
+            let at = match r.kind {
+                EntityKind::Soldier | EntityKind::Vehicle => {
+                    room.get(r.kind, r.index).map(|x| [x.grid[0], x.grid[1]])
+                }
+                EntityKind::Projectile => room.get(r.kind, r.index).map(|x| [x.grid[1], x.grid[2]]),
+                EntityKind::VehiclePart => room
+                    .get(r.kind, r.index)
+                    .and_then(|p| u16::try_from(p.grid[5]).ok())
+                    .and_then(|h| room.get(EntityKind::Vehicle, h))
+                    .map(|x| [x.grid[0], x.grid[1]]),
+                _ => None,
+            };
+            let (Some(me), Some(at)) = (me, at) else {
+                // Doors, the wall, or a record this tick's room records lack: no radius test.
+                self.audit.spatial_unchecked += 1;
+                continue;
+            };
+            self.audit.spatial += 1;
+            {
+                let (dx, dy) = (
+                    i64::from(at[0]) - i64::from(me.grid[0]),
+                    i64::from(at[1]) - i64::from(me.grid[1]),
+                );
+                if dx * dx + dy * dy > INTEREST_RADIUS_CM * INTEREST_RADIUS_CM {
+                    self.breach(format!(
+                        "tick {tick}: client {slot}'s entity section holds {:?} {} beyond the interest radius",
+                        r.kind, r.index
+                    ));
+                }
+            }
+        }
+    }
+
+    fn stats(&self, frame: &Frame, room: &Records<'_>, slot: u8) -> SetStats {
+        let has = |k: EntityKind, i: u16| frame.contains_key(&(k, i));
+        let others = room
+            .soldiers
+            .iter()
+            .filter(|s| **s != u16::from(slot))
+            .all(|s| has(EntityKind::Soldier, *s));
+        let vehicles =
+            !self.with_vehicles || room.vehicles.iter().all(|v| has(EntityKind::Vehicle, *v));
+        let large = !room.large.is_empty()
+            && room.large.iter().all(|(h, parts)| {
+                has(EntityKind::Vehicle, *h)
+                    && parts.iter().all(|p| has(EntityKind::VehiclePart, *p))
+            });
+        let projectiles = frame
+            .keys()
+            .filter(|(k, _)| *k == EntityKind::Projectile)
+            .count() as u32;
+        SetStats {
+            everyone: others && vehicles,
+            large_units: large,
+            projectiles,
+        }
+    }
+
+    /// One tick's messages for `slot` (the room's datagrams and reliable messages, as it handed them out), decoded with
+    /// `quant`: the frame rebuilt, the audit run, and -57's errors of what the snapshot carried returned. `last_acked`
+    /// is the room's `ClientSlot::last_acked()` for `slot` after the tick, whose frame is kept beyond the ring's
+    /// ([`FRAMES_KEPT`]'s text).
+    #[allow(
+        clippy::too_many_arguments,
+        reason = "the room's tick output for one client, and the one room value the frames' keep needs"
+    )]
+    pub(crate) fn tick(
+        &mut self,
+        slot: u8,
+        tick: u32,
+        datagrams: &[Vec<u8>],
+        control: &[Vec<u8>],
+        quant: Quant,
+        room: &Records<'_>,
+        last_acked: Option<u32>,
+    ) -> Errors {
+        let mut errs = Errors::default();
+        if slot >= self.slots {
+            return errs;
+        }
+        for b in control {
+            let Ok(channel) = crate::scenario::stream_of(b, quant) else {
+                self.breach(format!(
+                    "tick {tick}: client {slot}: a reliable message on no known channel"
+                ));
+                continue;
+            };
+            let Ok(Message::FullState(f)) = quant.decode(Direction::RoomToClient, channel, b)
+            else {
+                continue;
+            };
+            self.audit.public += 1;
+            self.audit_records(slot, f.tick, &f.own, &f.entities, room);
+            let frame: Frame = f
+                .entities
+                .iter()
+                .chain(&f.own)
+                .map(|r| ((r.kind, r.index), *r))
+                .collect();
+            if f.tick == tick {
+                for r in f.entities.iter().chain(&f.own) {
+                    if let Some(x) = room.get(r.kind, r.index) {
+                        errors(r, u16::MAX, x, &mut errs);
+                    }
+                }
+            }
+            let stats = self.stats(&frame, room, slot);
+            let c = &mut self.clients[usize::from(slot)];
+            c.latest = stats;
+            // Kept under the room's tick: the room builds a full state at the tick it takes it (the wire's tick is
+            // the same in a clean run, and a bitten header's under tick16).
+            c.keep(tick, frame, last_acked);
+        }
+        let mut frame: Option<Frame> = None;
+        for d in datagrams {
+            let Ok(Message::Snapshot(s)) =
+                quant.decode(Direction::RoomToClient, Channel::Datagram, d)
+            else {
+                self.breach(format!(
+                    "tick {tick}: client {slot}: a datagram the mirror cannot decode"
+                ));
+                continue;
+            };
+            self.audit.public += 1;
+            if frame.is_none() {
+                let c = &self.clients[usize::from(slot)];
+                // The baseline by the room's tick, never the wire's: the datagrams are the ones the room took at
+                // `tick` (equal in a clean run; under PM-0001-17/tick16 the wire's is 16 bits, carry 73).
+                frame = if s.baseline_age == 0 {
+                    Some(Frame::new())
+                } else {
+                    c.frame(tick.wrapping_sub(u32::from(s.baseline_age)))
+                        .cloned()
+                };
+                if frame.is_none() {
+                    self.breach(format!(
+                        "tick {tick}: client {slot}: its baseline frame (age {}) is not mirrored",
+                        s.baseline_age
+                    ));
+                    return errs;
+                }
+            }
+            let f = frame.as_mut().expect("set above");
+            let mut own = Vec::new();
+            let mut entities = Vec::new();
+            for (section, deltas) in [(true, &s.own), (false, &s.entities)] {
+                for delta in deltas {
+                    let (kind, index, changed) = match *delta {
+                        EntityDelta::Add(r) => (r.kind, r.index, u16::MAX),
+                        EntityDelta::Update {
+                            kind,
+                            index,
+                            changed,
+                            ..
+                        } => (kind, index, changed),
+                        EntityDelta::Remove { kind, index } => {
+                            f.remove(&(kind, index));
+                            continue;
+                        }
+                    };
+                    // Both sections are coded against the one baseline frame, which holds both.
+                    let base = f.get(&(kind, index)).copied();
+                    let Ok(Some(rebuilt)) = delta.apply(base.as_ref()) else {
+                        self.breach(format!(
+                            "tick {tick}: client {slot}: {kind:?} {index}'s update has no baseline record"
+                        ));
+                        continue;
+                    };
+                    if let Some(x) = room.get(kind, index) {
+                        errors(&rebuilt, changed, x, &mut errs);
+                    }
+                    f.insert((kind, index), rebuilt);
+                    if section {
+                        own.push(rebuilt);
+                    } else {
+                        entities.push(rebuilt);
+                    }
+                }
+            }
+            for (_, _, e) in &s.events {
+                self.audit.events += 1;
+                if let SimEvent::Shot { slot: shooter, .. } = e
+                    && *shooter == slot
+                {
+                    self.breach(format!(
+                        "tick {tick}: client {slot} was sent its own shot (cosmetic)"
+                    ));
+                }
+            }
+            self.audit_records(slot, tick, &own, &entities, room);
+        }
+        if let Some(f) = frame {
+            let stats = self.stats(&f, room, slot);
+            let c = &mut self.clients[usize::from(slot)];
+            c.latest = stats;
+            c.keep(tick, f, last_acked);
+        }
+        errs
+    }
+}
````

### npt-lab: contract.rs (new: r2, the contract test's checker)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs`:  1 file changed, 632 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs
new file mode 100644
index 0000000..4221f73
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs
@@ -0,0 +1,632 @@
+//! r2, the contract test's checker (u4r-brief.md 2.4 r2 (b), (c) and (e); U4R-Q11 (a)): one run's three logs, read
+//! back, against marks/SERIES.md and the judge's validity inputs. The judge's own half, (a) every check of the parts
+//! the run's header lists resolving with no gap and (d) PM-0001-2 passing, is `npt-judge --only` on the same folder:
+//! npt-lab does not depend on npt-judge (its manifest is not U4r's lane), so the two halves run side by side, by hand
+//! and from the run scripts.
+//! - **(b) names:** every name the logs write (each header param as `header.params.<name>`, every `tick.v`, `tick.s`
+//!   and `hs` name, every `second.v` and `second.s` name, every event field as `event.<event>.<field>`) has its
+//!   SERIES.md row; a header param also counts by its bare name in the header params table, and an event's
+//!   identifiers (`slot`, `burst`, `seq`: SERIES.md:313) are not measured values.
+//! - **(c) validity inputs:** a footer on every log (the `complete` rule: README §4); one `admit` event per client,
+//!   each `in_roster` true (the `roster` rule); `tick.v.late_count` on every room tick record. `late-zero` applies to
+//!   PM-0001-30 alone, which judges no U4r run, so this asks it of every run (stronger than "where it applies").
+//! - **(e) the shot band** (§4.13 :446-447): the measured seconds' mean of the room's `second.v.shots_count`, SS within
+//!   [665, 735], ES within [68.4, 75.6].
+//! - **The class audit** (the advisor's U4r build ruling 3, point 6 (1)): the lab log's `class_audit` event, its
+//!   `breaches_count` printed on every run; a run with no such event, or with a count above 0, fails, unless the run's
+//!   bite (its header's) is npt-room's class-filter bite, r4's, which must breach (catalogue item 5: the one id of
+//!   `npt_room::BITES` outside the `PM-` form, room/src/lib.rs:32-46, looked up at run time in a bites build and never
+//!   spelled here). The mirror's figures (everyone in view, -57's pairing, the audit) hold only on a run it followed.
+//! - **Carry 60:** the weapon and health params in every log's header; a missing one is a failure.
+//!
+//! The other carries' figures the logs give (reported, never a failure): 62 (h) (the measured phase's sum of
+//! `tick.s.snap_incomplete`), 58, 74, 77 (c), the rewinds refused, and -18's heal records.
+//!
+//! Exit codes (`main`): 0 PASS, 1 FAIL (every failure by name; a log that npt-log's reader refuses is one), 2 a usage
+//! error, 3 an input that cannot be read (the `--series` file or a log in the `--logs` folder), never reported as the
+//! contract's FAIL (point 6 (3)).
+
+use std::collections::{BTreeMap, BTreeSet};
+use std::path::Path;
+
+use npt_log::{Log, Phase};
+
+/// `contract`'s usage line.
+pub(crate) const USAGE: &str = "npt-lab contract --logs <run folder> --series <SERIES.md> (exit 0 PASS, 1 FAIL, \
+2 a usage error, 3 an input that cannot be read: the --series file or a log in the --logs folder)";
+/// The exit code of an input that cannot be read (the module text).
+pub(crate) const UNREADABLE: u8 = 3;
+
+/// The event fields that name an event's subject and are no measured value (SERIES.md:313).
+const IDENTIFIERS: [&str; 3] = ["slot", "burst", "seq"];
+/// Carry 60's header params: the weapon and the health.
+const CARRY_60: [&str; 7] = [
+    "weapon_interval_us",
+    "weapon_damage_count",
+    "weapon_reach_cm",
+    "weapon_magazine_count",
+    "weapon_reload_ms",
+    "weapon_spread_urad",
+    "soldier_health_count",
+];
+/// The clients of a full SS or ES run, the shot band's (PM-0001-2's parts: `"clients": 48`).
+const FULL_CLIENTS: u64 = 48;
+/// -18's heal bound on L1, ns (marks.json -18: `event.burst_heal.after_link_ns` <= 500,000,000).
+const HEAL_BOUND_NS: u64 = 500_000_000;
+
+/// What one run's logs gave: the report's lines, and every failure by name.
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct Outcome {
+    pub(crate) lines: Vec<String>,
+    pub(crate) failures: Vec<String>,
+}
+
+/// Every name SERIES.md gives a row: each backticked name in a table row's first cell, a `.x` shorthand joined to the
+/// cell's first name's record (`event.ws_practice.class`, `.m_p_ns`: SERIES.md:316).
+pub(crate) fn rows(series: &str) -> BTreeSet<String> {
+    let mut out = BTreeSet::new();
+    for line in series.lines() {
+        let Some(rest) = line.strip_prefix('|') else {
+            continue;
+        };
+        let cell = rest.split('|').next().unwrap_or("");
+        let names: Vec<&str> = cell.split('`').skip(1).step_by(2).collect();
+        let Some(first) = names.first() else {
+            continue;
+        };
+        let stem = first.rsplit_once('.').map_or("", |(s, _)| s);
+        for n in &names {
+            if n.starts_with('.') {
+                out.insert(format!("{stem}{n}"));
+            } else {
+                out.insert((*n).to_owned());
+            }
+        }
+    }
+    out
+}
+
+/// Every name the logs write (the module text's (b)).
+pub(crate) fn written(logs: &[Log]) -> BTreeSet<String> {
+    let mut out = BTreeSet::new();
+    for l in logs {
+        for k in l.header.params.keys() {
+            out.insert(format!("header.params.{k}"));
+        }
+        for t in &l.ticks {
+            out.extend(t.v.keys().map(|k| format!("tick.v.{k}")));
+            out.extend(t.s.keys().map(|k| format!("tick.s.{k}")));
+            out.extend(t.hs.keys().map(|k| format!("hs.{k}")));
+        }
+        for s in &l.seconds {
+            out.extend(s.v.keys().map(|k| format!("second.v.{k}")));
+            out.extend(s.s.keys().map(|k| format!("second.s.{k}")));
+        }
+        for e in &l.events {
+            for f in e.f.keys().filter(|f| !IDENTIFIERS.contains(&f.as_str())) {
+                out.insert(format!("event.{}.{f}", e.ev));
+            }
+        }
+    }
+    out
+}
+
+fn by_process<'a>(logs: &'a [Log], process: &str) -> Option<&'a Log> {
+    logs.iter().find(|l| l.header.process == process)
+}
+
+fn field_u64(e: &npt_log::EventRecord, name: &str) -> Option<u64> {
+    e.f.get(name).and_then(serde_json::Value::as_u64)
+}
+
+/// r2's checker over one run's logs (the module text).
+pub(crate) fn check(logs: &[Log], series: &str) -> Outcome {
+    let mut o = Outcome::default();
+    // (b) every written name has its row.
+    let have = rows(series);
+    let names = written(logs);
+    let missing: Vec<&String> = names
+        .iter()
+        .filter(|n| {
+            !have.contains(n.as_str())
+                && !n
+                    .strip_prefix("header.params.")
+                    .is_some_and(|k| have.contains(k))
+        })
+        .collect();
+    o.lines.push(format!(
+        "(b) names: {} written, {} without a SERIES.md row",
+        names.len(),
+        missing.len()
+    ));
+    for n in missing {
+        o.failures.push(format!("(b) {n} has no SERIES.md row"));
+    }
+    // (c) the validity inputs.
+    for l in logs {
+        if l.footer.is_none() {
+            o.failures.push(format!(
+                "(c) the {} log has no footer (complete)",
+                l.header.process
+            ));
+        }
+        for k in CARRY_60 {
+            if !l.header.params.contains_key(k) {
+                o.failures.push(format!(
+                    "carry 60: the {} log's header has no params.{k}",
+                    l.header.process
+                ));
+            }
+        }
+    }
+    for p in ["room", "client", "lab"] {
+        if by_process(logs, p).is_none() {
+            o.failures.push(format!("(c) no {p} log"));
+        }
+    }
+    audit(logs, &mut o);
+    let Some(room) = by_process(logs, "room") else {
+        return o;
+    };
+    let clients = u64::from(room.header.clients);
+    let mut admits: BTreeMap<u64, u64> = BTreeMap::new();
+    for e in room.events.iter().filter(|e| e.ev == "admit") {
+        let slot = field_u64(e, "slot").unwrap_or(u64::MAX);
+        *admits.entry(slot).or_default() += 1;
+        if e.f.get("in_roster") != Some(&serde_json::Value::Bool(true)) {
+            o.failures
+                .push(format!("(c) slot {slot}'s admit is not in_roster true"));
+        }
+    }
+    for slot in 0..clients {
+        match admits.get(&slot) {
+            Some(1) => {}
+            n => o.failures.push(format!(
+                "(c) slot {slot}: {} admit events, not 1",
+                n.copied().unwrap_or(0)
+            )),
+        }
+    }
+    let lacking = room
+        .ticks
+        .iter()
+        .filter(|t| !t.v.contains_key("late_count"))
+        .count();
+    if lacking > 0 || room.ticks.is_empty() {
+        o.failures.push(format!(
+            "(c) {lacking} of {} room tick records have no tick.v.late_count",
+            room.ticks.len()
+        ));
+    }
+    let late: i64 = room
+        .ticks
+        .iter()
+        .filter_map(|t| t.v.get("late_count"))
+        .sum();
+    o.lines.push(format!(
+        "(c) footers on {} logs; admit events {} for {clients} clients; tick.v.late_count on {} room ticks, sum {late}",
+        logs.iter().filter(|l| l.footer.is_some()).count(),
+        admits.values().sum::<u64>(),
+        room.ticks.len() - lacking
+    ));
+    // (e) the shot band.
+    let measured: Vec<i64> = room
+        .seconds
+        .iter()
+        .filter(|s| s.ph == Phase::Measured)
+        .filter_map(|s| s.v.get("shots_count").copied())
+        .collect();
+    let n = measured.len() as i64;
+    let sum: i64 = measured.iter().sum();
+    let (lo, hi) = match room.header.scene.as_str() {
+        "ES" => (684, 756),
+        _ => (6_650, 7_350),
+    };
+    let in_band = n > 0 && sum * 10 >= lo * n && sum * 10 <= hi * n;
+    // The band is the scene's 48 soldiers' (PM-0001-2's parts select 48 clients): a smaller test run is not held to it.
+    let applies = clients == FULL_CLIENTS;
+    o.lines.push(format!(
+        "(e) shots per measured second: mean {:.2} over {n} s, band [{}, {}]: {}",
+        if n > 0 { sum as f64 / n as f64 } else { 0.0 },
+        lo as f64 / 10.0,
+        hi as f64 / 10.0,
+        match (applies, in_band) {
+            (false, _) =>
+                format!("not applied ({clients} clients; the band is for {FULL_CLIENTS})"),
+            (true, true) => "in".to_owned(),
+            (true, false) => "out".to_owned(),
+        }
+    ));
+    if applies && !in_band {
+        o.failures
+            .push("(e) the shot band: the measured mean is outside it".to_owned());
+    }
+    carries(logs, &mut o);
+    o
+}
+
+/// Whether `bite` is npt-room's class-filter bite, r4's (the module text): the one id of npt-room's own list outside the
+/// `PM-` form, read from that list at run time. A measured build holds no list, so it knows no bite as that one.
+fn room_class_filter_bite(bite: &str) -> bool {
+    #[cfg(feature = "bites")]
+    let room_s = npt_room::BITES.contains(&bite);
+    #[cfg(not(feature = "bites"))]
+    let room_s = false;
+    room_s && !bite.starts_with("PM-")
+}
+
+/// The class audit (the module text): the lab log's `class_audit` event, its `breaches_count` on every run.
+fn audit(logs: &[Log], o: &mut Outcome) {
+    let Some(lab) = by_process(logs, "lab") else {
+        return;
+    };
+    let Some(e) = lab.events.iter().find(|e| e.ev == "class_audit") else {
+        o.failures
+            .push("class audit: the lab log has no class_audit event".to_owned());
+        return;
+    };
+    let Some(n) = field_u64(e, "breaches_count") else {
+        o.failures
+            .push("class audit: the class_audit event has no breaches_count".to_owned());
+        return;
+    };
+    let first =
+        e.f.get("first_breach")
+            .and_then(serde_json::Value::as_str)
+            .unwrap_or("");
+    let bite = lab.header.bite.as_deref();
+    let exempt = bite.is_some_and(room_class_filter_bite);
+    o.lines.push(format!(
+        "class audit: breaches_count {n}{}{}",
+        if n > 0 {
+            format!(" (the first: {first:?})")
+        } else {
+            String::new()
+        },
+        if exempt {
+            "; the run's bite is npt-room's class filter (r4's), which must breach: not a failure"
+        } else {
+            ""
+        }
+    ));
+    if n > 0 && !exempt {
+        o.failures.push(format!(
+            "class audit: {n} breaches of the room-side mirror (the first: {first:?}), so its figures (everyone in \
+             view, -57's pairing, the audit) do not hold{}",
+            bite.map_or(String::new(), |b| format!(" (the run's bite {b})"))
+        ));
+    }
+}
+
+/// The carries' figures (reported).
+fn carries(logs: &[Log], o: &mut Outcome) {
+    if let Some(c) = by_process(logs, "client") {
+        let sum: i64 = c
+            .ticks
+            .iter()
+            .filter(|t| t.ph == Phase::Measured)
+            .filter_map(|t| t.s.get("snap_incomplete"))
+            .flatten()
+            .sum();
+        o.lines.push(format!(
+            "carry 62 (h): the measured phase's sum of tick.s.snap_incomplete {sum}"
+        ));
+    }
+    let Some(lab) = by_process(logs, "lab") else {
+        return;
+    };
+    let keys = [
+        "commands_count",
+        "late_count",
+        "steps_without_command_count",
+        "stale_count",
+        "rewinds_refused_count",
+        "events_queued_count",
+        "overflow_resyncs_count",
+    ];
+    let mut t: BTreeMap<&str, u64> = keys.iter().map(|k| (*k, 0)).collect();
+    for e in lab.events.iter().filter(|e| e.ev == "client_counts") {
+        for k in keys {
+            *t.entry(k).or_default() += field_u64(e, k).unwrap_or(0);
+        }
+    }
+    let lead = lab
+        .header
+        .params
+        .get("lead_ticks_count")
+        .cloned()
+        .unwrap_or_default();
+    o.lines.push(format!(
+        "carry 74: lead {lead}; commands {}, late {}, steps without a command {}; carry 77 (c): stale {}; rewinds \
+         refused {}; carry 58: events queued {}, overflow resyncs {}",
+        t["commands_count"],
+        t["late_count"],
+        t["steps_without_command_count"],
+        t["stale_count"],
+        t["rewinds_refused_count"],
+        t["events_queued_count"],
+        t["overflow_resyncs_count"]
+    ));
+    let heals: Vec<&npt_log::EventRecord> =
+        lab.events.iter().filter(|e| e.ev == "burst_heal").collect();
+    if !heals.is_empty() || lab.header.profile == "B1" {
+        let within = heals
+            .iter()
+            .filter(|e| field_u64(e, "after_link_ns").is_some_and(|n| n <= HEAL_BOUND_NS))
+            .count();
+        let pairs: BTreeSet<(u64, u64)> = heals
+            .iter()
+            .filter_map(|e| Some((field_u64(e, "slot")?, field_u64(e, "burst")?)))
+            .collect();
+        let queue_max = heals
+            .iter()
+            .filter_map(|e| field_u64(e, "queue_max_count"))
+            .max()
+            .unwrap_or(0);
+        let full = heals
+            .iter()
+            .filter(|e| field_u64(e, "full_state") == Some(1))
+            .count();
+        let bursts = lab
+            .header
+            .params
+            .get("burst_starts_ns")
+            .and_then(serde_json::Value::as_array)
+            .map_or(0, Vec::len);
+        o.lines.push(format!(
+            "-18: {} heal records ({} within 500 ms; {} (client, burst) pairs of {} clients x {bursts} bursts; a full \
+             state during {full}); carry 58: queue high-water mark {queue_max}",
+            heals.len(),
+            within,
+            pairs.len(),
+            lab.header.clients
+        ));
+    }
+}
+
+/// The run folder's three logs, read back: `Err((UNREADABLE, why))` for a log that cannot be opened (the module text's
+/// exit 3), `Err((1, the FAIL lines))` for one that npt-log's reader refuses (a contract FAIL: the log is there and is
+/// not a log by README §1's discipline).
+pub(crate) fn read(dir: &Path) -> Result<Vec<Log>, (u8, String)> {
+    ["room", "client", "lab"]
+        .iter()
+        .map(|p| {
+            let path = dir.join(format!("{p}.jsonl"));
+            let f = std::fs::File::open(&path)
+                .map_err(|e| (UNREADABLE, format!("cannot read {}: {e}", path.display())))?;
+            npt_log::read_log(std::io::BufReader::new(f)).map_err(|e| {
+                (
+                    1,
+                    format!(
+                        "FAIL (c) {} does not read back as a log: {e}\ncontract: FAIL (1 failure)",
+                        path.display()
+                    ),
+                )
+            })
+        })
+        .collect()
+}
+
+/// The `contract` subcommand: `Ok` with the lines when nothing failed, else `Err((1, lines and failures))`; a usage
+/// error is `Err((2, ...))`, an input that cannot be read `Err((UNREADABLE, ...))` (the module text).
+pub(crate) fn main(argv: &[String]) -> Result<Vec<String>, (u8, String)> {
+    let usage = |m: &str| (2u8, m.to_owned());
+    let (mut logs, mut series) = (None, None);
+    let mut it = argv.iter();
+    while let Some(flag) = it.next() {
+        let v = it.next().ok_or(usage("each option needs a value"))?;
+        match flag.as_str() {
+            "--logs" if logs.is_none() => logs = Some(v),
+            "--series" if series.is_none() => series = Some(v),
+            _ => return Err(usage(&format!("unexpected {flag:?}"))),
+        }
+    }
+    let (Some(logs), Some(series)) = (logs, series) else {
+        return Err(usage("--logs and --series are required"));
+    };
+    let text = std::fs::read_to_string(series)
+        .map_err(|e| (UNREADABLE, format!("cannot read --series {series}: {e}")))?;
+    let read = read(Path::new(logs))?;
+    let o = check(&read, &text);
+    let mut lines = vec![format!("contract of {logs}:")];
+    lines.extend(o.lines.iter().cloned());
+    if o.failures.is_empty() {
+        lines.push("contract: PASS".to_owned());
+        Ok(lines)
+    } else {
+        lines.extend(o.failures.iter().map(|f| format!("FAIL {f}")));
+        lines.push(format!("contract: FAIL ({} failures)", o.failures.len()));
+        Err((1, lines.join("\n")))
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::scenario::tests::{scratch, settings};
+
+    const SERIES: &str = include_str!("../../../marks/SERIES.md");
+
+    /// One short L1 P0 run's logs (SS, 48 clients so the shot band applies, 6 s of warm-up, 2 s measured, its header
+    /// listing -17).
+    fn short_run() -> Vec<Log> {
+        let dir = scratch("r2");
+        let s = settings(
+            &dir,
+            "--seed 2 --profile P0 --warmup-s 6 --measured-s 2 --after-max-s 10 --marks PM-0001-17",
+        );
+        crate::scenario::run(&s).expect("the run");
+        read(&dir).expect("its logs")
+    }
+
+    /// r2's checker passes the clean run, and each of U4R-Q11's four doctored copies fails it by name, and so do two
+    /// more: the shot band, and a mirror breach (the advisor's U4r build ruling 3, point 6 (1)).
+    #[test]
+    fn r2_the_checker_passes_a_clean_run_and_fails_each_doctored_copy_by_name() {
+        let logs = short_run();
+        let clean = check(&logs, SERIES);
+        for l in &clean.lines {
+            println!("r2 clean: {l}");
+        }
+        assert_eq!(clean.failures, Vec::<String>::new(), "the clean run");
+        let room = logs
+            .iter()
+            .position(|l| l.header.process == "room")
+            .expect("a room log");
+        // (1) one admit event gone.
+        let mut d = logs.clone();
+        let i = d[room]
+            .events
+            .iter()
+            .position(|e| e.ev == "admit")
+            .expect("an admit");
+        let slot = field_u64(&d[room].events[i], "slot").expect("its slot");
+        d[room].events.remove(i);
+        let f = check(&d, SERIES).failures;
+        assert!(
+            f.contains(&format!("(c) slot {slot}: 0 admit events, not 1")),
+            "{f:?}"
+        );
+        // (2) one footer gone.
+        let mut d = logs.clone();
+        d[1].footer = None;
+        let f = check(&d, SERIES).failures;
+        assert!(
+            f.contains(&format!(
+                "(c) the {} log has no footer (complete)",
+                d[1].header.process
+            )),
+            "{f:?}"
+        );
+        // (3) one written name's SERIES.md row gone.
+        let row = "| `tick.s.snap_incomplete` |";
+        assert!(SERIES.contains(row), "the row is there");
+        let without: String = SERIES
+            .lines()
+            .filter(|l| !l.starts_with(row))
+            .collect::<Vec<_>>()
+            .join("\n");
+        let f = check(&logs, &without).failures;
+        assert_eq!(
+            f,
+            ["(b) tick.s.snap_incomplete has no SERIES.md row".to_owned()]
+        );
+        // (4) one late_count series gone (a room tick record without it).
+        let mut d = logs.clone();
+        d[room].ticks[3].v.remove("late_count");
+        let f = check(&d, SERIES).failures;
+        assert!(
+            f.contains(&format!(
+                "(c) 1 of {} room tick records have no tick.v.late_count",
+                d[room].ticks.len()
+            )),
+            "{f:?}"
+        );
+        // (5) beyond U4R-Q11's four: the shot band, with every measured second's shots halved.
+        let mut d = logs.clone();
+        for s in d[room]
+            .seconds
+            .iter_mut()
+            .filter(|s| s.ph == Phase::Measured)
+        {
+            if let Some(v) = s.v.get_mut("shots_count") {
+                *v /= 2;
+            }
+        }
+        let f = check(&d, SERIES).failures;
+        assert_eq!(
+            f,
+            ["(e) the shot band: the measured mean is outside it".to_owned()]
+        );
+        // (6) the advisor's U4r build ruling 3, point 6 (1): one breach of the mirror added to the lab log's audit.
+        let lab = logs
+            .iter()
+            .position(|l| l.header.process == "lab")
+            .expect("a lab log");
+        let mut d = logs.clone();
+        let a = d[lab]
+            .events
+            .iter_mut()
+            .find(|e| e.ev == "class_audit")
+            .expect("the class audit");
+        a.f.insert("breaches_count".to_owned(), serde_json::json!(1));
+        a.f.insert(
+            "first_breach".to_owned(),
+            serde_json::json!("tick 2: client 0: planted"),
+        );
+        let f = check(&d, SERIES).failures;
+        assert_eq!(
+            f,
+            ["class audit: 1 breaches of the room-side mirror (the first: \"tick 2: client 0: planted\"), so its \
+              figures (everyone in view, -57's pairing, the audit) do not hold"
+                .to_owned()]
+        );
+        // Its one exception, r4's bite (bites builds only: npt-room's list is read at run time); another bite's
+        // breaches still fail.
+        #[cfg(feature = "bites")]
+        {
+            let mut x = d.clone();
+            x[lab].header.bite = Some("catalogue-5/owner-to-all".to_owned());
+            assert_eq!(
+                check(&x, SERIES).failures,
+                Vec::<String>::new(),
+                "r4's bite"
+            );
+            x[lab].header.bite = Some("PM-0001-18/no-fallback".to_owned());
+            assert_eq!(check(&x, SERIES).failures.len(), 1, "another bite");
+            // The advisor's U4r build ruling 4, point 2: exactly one id of npt-room's list lies outside the `PM-` form,
+            // so a second such id cannot widen the exemption unseen.
+            let outside: Vec<&str> = npt_room::BITES
+                .iter()
+                .copied()
+                .filter(|b| !b.starts_with("PM-"))
+                .collect();
+            assert_eq!(
+                outside,
+                ["catalogue-5/owner-to-all"],
+                "npt-room's ids outside the PM- form"
+            );
+        }
+        println!("r2: the six doctored copies each fail by name");
+    }
+
+    /// The advisor's U4r build ruling 3, point 6 (3): a `--series` file or a log that cannot be read is exit 3 with its
+    /// own message, never the contract's FAIL (exit 1).
+    #[test]
+    fn r2_an_input_that_cannot_be_read_is_exit_3_never_the_contract_s_fail() {
+        let dir = scratch("r2-unreadable");
+        std::fs::create_dir_all(&*dir).expect("the folder");
+        let d = dir.to_str().expect("a UTF-8 path");
+        let argv = |logs: &str, series: &str| -> Vec<String> {
+            ["--logs", logs, "--series", series]
+                .iter()
+                .map(|s| (*s).to_owned())
+                .collect()
+        };
+        let series = format!("{}/../../marks/SERIES.md", env!("CARGO_MANIFEST_DIR"));
+        let e = main(&argv(d, &series)).expect_err("an empty folder: no log");
+        assert_eq!(e.0, UNREADABLE, "{}", e.1);
+        assert!(
+            e.1.starts_with("cannot read ") && !e.1.contains("FAIL"),
+            "{}",
+            e.1
+        );
+        let e = main(&argv(d, &format!("{d}/no-such-SERIES.md"))).expect_err("no SERIES file");
+        assert_eq!(e.0, UNREADABLE, "{}", e.1);
+        assert!(e.1.starts_with("cannot read --series "), "{}", e.1);
+        assert!(USAGE.contains("3 an input that cannot be read"));
+    }
+
+    /// The row reader takes every backticked name of a first cell and joins `.x` shorthands to its record.
+    #[test]
+    fn r2_rows_reads_each_first_cell_s_names() {
+        let r = rows(
+            "| `a.b.c`, `.d_ns` | x | y |\n| `soak` | S30 | z |\nnot a row `e.f`\n| Name | Unit |\n",
+        );
+        let want: BTreeSet<String> = ["a.b.c", "a.b.d_ns", "soak"]
+            .iter()
+            .map(|s| (*s).to_owned())
+            .collect();
+        assert_eq!(r, want);
+    }
+}
````

### npt-lab: precheck.rs (new: the -30 pre-check)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs`:  1 file changed, 977 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs
new file mode 100644
index 0000000..1eef179
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs
@@ -0,0 +1,977 @@
+//! The -30 pre-check (u4r-brief.md 7.2; plan :772-796; the advisor's conditions 2 and 3; carry 40): a recorded npt-lab
+//! test, never a judged id. Replay clients send the SS P0 recording's commands to one room over Loopback and to another
+//! over L1 P0, in RQ8's order on one SimClock each, and `HashCompare::of` compares the two rooms' per-tick World hashes.
+//!
+//! - **The tick base:** both legs' `World::new(scene, recording.seed, recording.first_tick)`, so each leg's first tick
+//!   is the recording's first (`run_leg` refuses a tick that is not the recording's), run at SimClock instant 0 (U4b's
+//!   schedule, base 0); the k-th tick is due at `due_ns(k)`.
+//! - **The encoder** (R16): `ClientEndpoint::input` stamps its own ticks and sequences, so it cannot send the
+//!   recording's. Each replay client runs a `ClientEndpoint` only to decode (`input(None, received, now)`) and to send
+//!   its `Join` and `Pong`s; the endpoint's own `Input` (the acknowledgement only: it is given no command) is read back
+//!   for its `ack_tick` and `ack_groups` and replaced by the replay client's own `Input`, encoded with npt-proto: that
+//!   acknowledgement and the recorded commands (their slot, tick, sequence, presses and view time as the room ran them),
+//!   newest first. So the acknowledgement names only snapshots the room sent it. No npt-client change.
+//! - **Pacing** (plan silent): the command stamped T is sent at the instant tick T − L runs (step 5, after the tick), L
+//!   the lead (`buffer_depth_ticks_count`, 1 to 7). On Loopback it is received at the next instant's step 2, the instant
+//!   tick T − L + 1 is due; on L1 P0 at the next net instant after the send (never the same instant: l1/src/lib.rs:65-70);
+//!   either way before tick T − L + 1 runs, so it waits at most L ticks, within `CMD_CATCHUP` (8). Each `Input` carries
+//!   every sent command whose tick has not run (at most L, newest first, consecutive sequences), so a lost datagram
+//!   would be covered by the next; at P0 and on Loopback none is lost. A command the recording repeats (option B's rerun:
+//!   its sequence again) is sent once: the room reruns it as the recording shows.
+//! - **Admission** does not enter the World's hashed state: only a tick's `TickInput` does (room.rs's join adds nothing
+//!   to it; scenario.rs s5 replays a runner recording's `TickInput`s alone to the room's every World hash), so the two
+//!   legs may admit a client at different ticks; the records give each client's admit tick per leg.
+//! - **A replay client sends its first `Input` only after it decodes `JoinAccepted`** (`ClientEndpoint::slot`).
+//! - **Scripts:** each recorded tick's spawns, teleports and attribute changes go to the leg's room by `Room::script`
+//!   before that tick, through carry 49's `check_placements`.
+//! - **The comparer:** `HashCompare::of` over each leg's `TickHash { recording, tick, hash }` list, the recording's
+//!   manifest name on both, from the recording's first tick, each hash npt-sim's `SimCore::state_hash` of the room's
+//!   World after the tick (the value logs.rs writes as the room log's `h`), never the mirror hash.
+//! - **The record** (the advisor's U4r build ruling 2, points 1 and 2): no per-leg npt-log log; the result lines
+//!   (runs.txt) carry what a header would: the build (`--commit`, required), the recording checked, each leg's room
+//!   configuration, `buffer_depth_ticks_count` L beside the room's measured depth (L - 1 by definition after each
+//!   client's first take), and the refused Rewinds per command.
+//! - **The setup condition and the RTT rule:** per leg, `late_count` (the room's `late`), `rewinds_refused`, the commands
+//!   run off their stamp, and [`rtt_rule`]; the tolerance's replacements and the cap's clips are not built at U4r (0 by
+//!   construction: rewind.rs:1-2). Any of the first three above 0, or the RTT rule on the L1 leg, makes the pair VOID.
+
+use std::collections::BTreeMap;
+
+use npt_client::ClientEndpoint;
+use npt_l1::{L1Net, Profile};
+use npt_lab::{HashCompare, TickHash};
+use npt_link::{Channel, Link};
+use npt_proto::{Direction, Input, Limits, Message, Quant};
+use npt_room::Room;
+use npt_server::{RttSource, Schedule, due_ticks};
+use npt_sim::replay::Recording;
+use npt_sim::{Command, Scene, StandIns, World};
+
+use crate::scenario::{
+    Bites, CONTENT, MAX_DATAGRAM, Server, check_placements, credentials, join_of,
+};
+
+/// The pre-check's suite name (7.2).
+pub(crate) const SUITE: &str = "precheck-30";
+
+/// A pre-check leg.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub(crate) enum Leg {
+    Loopback,
+    L1,
+}
+
+impl Leg {
+    pub(crate) fn name(self) -> &'static str {
+        match self {
+            Leg::Loopback => "loopback",
+            Leg::L1 => "L1",
+        }
+    }
+}
+
+/// One client's records on one leg (7.2: the RTT rule's three, and the admit tick).
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct ClientRecord {
+    pub(crate) slot: u8,
+    pub(crate) admit_tick: Option<u32>,
+    /// The SimClock instant of the first pump call after which the session's `Decoder::rtt_ns(link)` is `Some`.
+    pub(crate) first_sample_ns: Option<u64>,
+    /// The instant of the pump call that decoded its first `Input` with a command.
+    pub(crate) first_input_ns: Option<u64>,
+    /// The tick its first command ran at, and that tick's run instant.
+    pub(crate) first_run_tick: Option<u32>,
+    pub(crate) first_run_ns: Option<u64>,
+    /// That tick's due instant (base 0 + floor(k x 10^9 / TICK_HZ), k its index from the leg's first tick: 7.2 (3)),
+    /// beside the run instant (the advisor's U4r build ruling 2, its Low finding).
+    pub(crate) first_run_due_ns: Option<u64>,
+    /// The room's measured depth at this client's first take (the take that ran its first command).
+    pub(crate) first_take_depth: Option<u32>,
+}
+
+/// The RTT rule (7.2; plan :781-790; U4R-Q10 (b')): on the L1 leg, a client whose first command runs strictly before
+/// its first-sample instant, or that has no first-sample instant when its first command runs, makes the leg VOID; equal
+/// instants are not VOID (at one instant the pump comes before the tick). The Loopback leg's `rtt_ns` is Some(0) from
+/// the start (link/src/lib.rs:203-205), so the rule binds the L1 leg only. `Some(reason)` for VOID.
+pub(crate) fn rtt_rule(leg: Leg, records: &[ClientRecord]) -> Option<String> {
+    if leg == Leg::Loopback {
+        return None;
+    }
+    for r in records {
+        let Some(run) = r.first_run_ns else { continue };
+        match r.first_sample_ns {
+            None => {
+                return Some(format!(
+                    "client {}: no first-sample instant on the L1 leg when its first command runs ({run} ns)",
+                    r.slot
+                ));
+            }
+            Some(sample) if run < sample => {
+                return Some(format!(
+                    "client {}: its first command runs at {run} ns, before its first sample at {sample} ns",
+                    r.slot
+                ));
+            }
+            Some(_) => {}
+        }
+    }
+    None
+}
+
+/// One leg's outcome (the module text).
+#[derive(Clone, Debug, Default)]
+pub(crate) struct LegOutcome {
+    /// The lead L this leg's replay clients paced by (its configured lead: `run_leg`'s `lead`).
+    pub(crate) lead: u32,
+    pub(crate) hashes: Vec<TickHash>,
+    pub(crate) late: u64,
+    pub(crate) rewinds_refused: u64,
+    /// New commands the room ran at another tick than their stamp.
+    pub(crate) off_stamp: u64,
+    /// The room's measured buffer depth (`ClientSlot::buffer_depth_ticks`: "how many ticks ahead the newest buffered
+    /// command was at the last take", room/src/input.rs:28-29), read after each tick, over each client's takes after
+    /// its first take only (that one is the client record's): depth to the number of (client, tick) reads. Under 7.2's
+    /// pacing it is L - 1 by definition (the advisor's U4r build ruling 2, point 1).
+    pub(crate) depth: BTreeMap<u32, u64>,
+    /// The takes after each client's first at which the measured depth is not L - 1, and the first 64 as (client,
+    /// tick, measured): reported, never a VOID cause.
+    pub(crate) depth_deviations: u64,
+    pub(crate) deviations: Vec<(u8, u32, u32)>,
+    /// The refused Rewinds per command (`ClientSlot::rewinds_refused`, read after each tick: 7.2 "per leg and per
+    /// command"): the total and the first 64 as (client, tick, sequence).
+    pub(crate) refused_logged: u64,
+    pub(crate) refused: Vec<(u8, u32, u16)>,
+    pub(crate) records: Vec<ClientRecord>,
+    /// Sessions ended on this leg: (slot, why).
+    pub(crate) ended: Vec<(u8, &'static str)>,
+    /// Scripted placements and attribute changes put into the room.
+    pub(crate) scripted: usize,
+    /// The late-command bite's delayed command's tick, on its leg.
+    pub(crate) delayed: Option<u32>,
+    pub(crate) rtt_void: Option<String>,
+}
+
+/// One replay client (the module text).
+struct ReplayClient {
+    slot: u8,
+    endpoint: ClientEndpoint,
+    link: Box<dyn Link>,
+    inbox: Vec<(Channel, Vec<u8>, u64)>,
+    /// The recorded commands of this slot, in sequence order (each once).
+    cmds: Vec<Command>,
+    /// Commands sent so far: `cmds[..next]`; `cmds[unrun..next]` sent and not yet run.
+    next: usize,
+    unrun: usize,
+    /// The late-command bite: the tick T whose command and every later one are held until T has run.
+    hold: Option<u32>,
+    released: bool,
+}
+
+impl ReplayClient {
+    /// RQ8's step 5 at `now`, after tick `ran` ran: the endpoint decodes what arrived and sends its `Join` and `Pong`s;
+    /// once joined, the commands due (stamped at most `ran + lead`) go in this client's own `Input`.
+    fn step(&mut self, now: u64, ran: u32, lead: u32) -> Option<u32> {
+        let mut got = std::mem::take(&mut self.inbox);
+        while let Some(r) = self.link.poll_recv(now) {
+            got.push((r.channel, r.bytes, r.arrival_ns));
+        }
+        self.endpoint.input(None, &got, now);
+        let out = self.endpoint.step();
+        for (ch, b) in &out.reliable {
+            self.link.send_reliable(*ch, b, now);
+        }
+        let mut ack = (0u16, 0u8);
+        for d in &out.datagrams {
+            match npt_proto::decode(Direction::ClientToRoom, Channel::Datagram, d) {
+                Ok(Message::Input(i)) => ack = (i.ack_tick, i.ack_groups),
+                _ => {
+                    self.link.send_unreliable(d, now);
+                }
+            }
+        }
+        self.endpoint.slot()?;
+        let held = |c: &Command, hold: Option<u32>| hold.is_some_and(|t| ran < t && c.tick >= t);
+        while self.next < self.cmds.len()
+            && self.cmds[self.next].tick <= ran.wrapping_add(lead)
+            && !held(&self.cmds[self.next], self.hold)
+        {
+            self.next += 1;
+        }
+        while self.unrun < self.next && self.cmds[self.unrun].tick <= ran {
+            self.unrun += 1;
+        }
+        let mut from = self.unrun;
+        let mut delayed = None;
+        if let Some(t) = self.hold
+            && ran >= t
+            && !self.released
+        {
+            // The bite's release: T first (it ran already: option B reran the last command there), the later ones after.
+            self.released = true;
+            if let Some(i) = self.cmds.iter().position(|c| c.tick >= t) {
+                from = from.min(i);
+                delayed = Some(self.cmds[i].tick);
+            }
+        }
+        let mut commands: Vec<Command> = Vec::new();
+        for c in self.cmds[from..self.next].iter().rev() {
+            if commands.len() == Limits::REDUNDANCY_CAP
+                || commands
+                    .last()
+                    .is_some_and(|n: &Command| n.seq != c.seq.wrapping_add(1))
+            {
+                break;
+            }
+            commands.push(*c);
+        }
+        if commands.is_empty() && ack == (0, 0) {
+            return delayed;
+        }
+        let input = Input {
+            ack_tick: ack.0,
+            ack_groups: ack.1,
+            commands,
+        };
+        let mut b = Vec::new();
+        if npt_proto::encode(&Message::Input(input), &mut b).is_ok() {
+            self.link.send_unreliable(&b, now);
+        }
+        delayed
+    }
+}
+
+/// Each slot's recorded commands, each once in sequence order (a rerun, its sequence again, skipped), checked to have
+/// run at their stamped tick in the recording.
+pub(crate) fn commands_of(rec: &Recording) -> Result<BTreeMap<u8, Vec<Command>>, String> {
+    let mut out: BTreeMap<u8, Vec<Command>> = BTreeMap::new();
+    for ti in &rec.ticks {
+        for (c, _) in &ti.commands {
+            let list = out.entry(c.slot).or_default();
+            if list.last().is_some_and(|l| l.seq == c.seq) {
+                continue;
+            }
+            if c.tick != ti.tick {
+                return Err(format!(
+                    "the recording's command {} of slot {} is stamped {} but ran at {}: not a clean recording",
+                    c.seq, c.slot, c.tick, ti.tick
+                ));
+            }
+            list.push(*c);
+        }
+    }
+    Ok(out)
+}
+
+/// Runs one leg (the module text) on the recording `rec` named `name`, at lead `lead`, with `bite` (an id) on it.
+pub(crate) fn run_leg(
+    rec: &Recording,
+    name: &str,
+    leg: Leg,
+    lead: u32,
+    bite: Option<&str>,
+) -> Result<LegOutcome, String> {
+    let scene = Scene::by_id(rec.scene)
+        .ok_or(format!("no scene {}", rec.scene))?
+        .with_stand_ins(StandIns::OFF);
+    let cmds = commands_of(rec)?;
+    let world = World::new(&scene, rec.seed, rec.first_tick);
+    let greybox = world.greybox().clone();
+    let tokens = (0..scene.slots())
+        .map(|slot| credentials(rec.seed, slot).0)
+        .collect();
+    let room = Room::new(
+        world,
+        &scene,
+        &greybox,
+        tokens,
+        cmds.len(),
+        npt_sim::TICK_HZ,
+    )
+    .with_hashes(CONTENT, [0; 32]);
+    let mut server = Server::new(room, Quant::default(), RttSource::Link);
+    let mut late_bite = false;
+    if let Some(id) = bite {
+        let b = Bites::of(id)?;
+        if b.pump {
+            server.decoder_bite = Some(id.to_owned());
+        } else if b.late_command {
+            late_bite = true;
+        } else {
+            return Err(format!("--bite {id}: not a -30 pre-check bite"));
+        }
+    }
+    let mut schedule = Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP);
+    let net = L1Net::new(rec.seed);
+    let mut listener = net.listener();
+    let p0 = Profile::by_name("P0").ok_or("no profile P0")?;
+    let mut clients: Vec<ReplayClient> = Vec::new();
+    for (&slot, list) in &cmds {
+        let link: Box<dyn Link> = match leg {
+            Leg::L1 => net.connect(p0.clone(), MAX_DATAGRAM, 0),
+            Leg::Loopback => {
+                let (s, c) = npt_link::Loopback::pair(MAX_DATAGRAM);
+                server.accept(Box::new(s), 0);
+                Box::new(c)
+            }
+        };
+        let hold = if late_bite && clients.is_empty() {
+            let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
+            Some(
+                list.iter()
+                    .map(|c| c.tick)
+                    .find(|t| t.wrapping_sub(t0) >= 60 && t.wrapping_sub(t0) < 1 << 31)
+                    .ok_or("the late-command bite: slot's commands end before 60 ticks")?,
+            )
+        } else {
+            None
+        };
+        clients.push(ReplayClient {
+            slot,
+            endpoint: ClientEndpoint::new(join_of(rec.seed, slot), npt_sim::TICK_HZ)
+                .with_lead(lead),
+            link,
+            inbox: Vec::new(),
+            cmds: list.clone(),
+            next: 0,
+            unrun: 0,
+            hold,
+            released: false,
+        });
+    }
+    let mut o = LegOutcome {
+        lead,
+        ..LegOutcome::default()
+    };
+    let mut admitted: BTreeMap<u8, u32> = BTreeMap::new();
+    let mut first_run: BTreeMap<u8, (u32, u64)> = BTreeMap::new();
+    let mut first_take: BTreeMap<u8, u32> = BTreeMap::new();
+    let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
+    let mut last_seq: BTreeMap<u8, u16> = BTreeMap::new();
+    let mut now = 0u64;
+    let mut idx = 0usize;
+    while idx < rec.ticks.len() {
+        // (1) packets due, and each new link's server end (L1).
+        while let Some((link, _)) = listener.poll_accept(now) {
+            server.accept(link, now);
+        }
+        // (2) the server's I/O step.
+        server.io(now);
+        for (slot, _, tick) in std::mem::take(&mut server.admitted) {
+            admitted.entry(slot).or_insert(tick);
+        }
+        o.ended.append(&mut server.ended);
+        // (3) the server tick, if due.
+        let due = due_ticks(now, &schedule);
+        schedule = due.next;
+        let mut ran = None;
+        for _ in 0..due.to_run {
+            let Some(ti) = rec.ticks.get(idx) else { break };
+            check_placements(&ti.spawns, &ti.teleports)?;
+            o.scripted += ti.spawns.len() + ti.teleports.len() + ti.attributes.len();
+            server
+                .room
+                .script(&ti.spawns, &ti.teleports, &ti.attributes);
+            let t = server.room.last_tick().wrapping_add(1);
+            if t != ti.tick {
+                return Err(format!(
+                    "{} leg: the room's tick {t} is not the recording's {} (the tick base)",
+                    leg.name(),
+                    ti.tick
+                ));
+            }
+            server.room.tick();
+            o.hashes.push(TickHash {
+                recording: name.to_owned(),
+                tick: t,
+                hash: npt_sim::SimCore::state_hash(server.room.sim()),
+            });
+            let mut first_now = Vec::new();
+            for (c, _) in &server.room.tick_input().commands {
+                if last_seq.insert(c.slot, c.seq) != Some(c.seq) {
+                    o.off_stamp += u64::from(c.tick != t);
+                    if let std::collections::btree_map::Entry::Vacant(v) = first_run.entry(c.slot) {
+                        v.insert((t, now));
+                        first_now.push(c.slot);
+                    }
+                }
+            }
+            for &slot in first_run.keys() {
+                let Some(c) = server.room.client(slot) else {
+                    continue;
+                };
+                let d = c.buffer_depth_ticks();
+                if first_now.contains(&slot) {
+                    first_take.insert(slot, d);
+                    continue;
+                }
+                *o.depth.entry(d).or_default() += 1;
+                if d != lead - 1 {
+                    o.depth_deviations += 1;
+                    if o.deviations.len() < 64 {
+                        o.deviations.push((slot, t, d));
+                    }
+                }
+            }
+            for c in &clients {
+                for &seq in server
+                    .room
+                    .client(c.slot)
+                    .map_or(&[][..], |x| x.rewinds_refused())
+                {
+                    o.refused_logged += 1;
+                    if o.refused.len() < 64 {
+                        o.refused.push((c.slot, t, seq));
+                    }
+                }
+            }
+            for c in &clients {
+                let d = server.room.take_datagrams(c.slot);
+                let k = server.room.take_control(c.slot);
+                server.send(c.slot, &d, &k, now)?;
+            }
+            o.ended.append(&mut server.ended);
+            idx += 1;
+            ran = Some(t);
+        }
+        // (5) each replay client, after a tick ran.
+        if let Some(t) = ran {
+            for c in &mut clients {
+                if let Some(d) = c.step(now, t, lead) {
+                    o.delayed = Some(d);
+                }
+            }
+        }
+        for c in &mut clients {
+            while let Some(r) = c.link.poll_recv(now) {
+                c.inbox.push((r.channel, r.bytes, r.arrival_ns));
+            }
+        }
+        let mut next = schedule.next_due_ns();
+        if let Some(e) = net.next_event_ns() {
+            next = next.min(e.max(now + 1));
+        }
+        now = next.max(now + 1);
+    }
+    for c in &clients {
+        let k = server
+            .room
+            .client(c.slot)
+            .map(|x| x.counters().clone())
+            .unwrap_or_default();
+        o.late += k.late;
+        o.rewinds_refused += k.rewinds_refused;
+        let session = server.session(c.slot);
+        let (sample, input) =
+            session.map_or((None, None), |s| (s.first_sample_ns, s.first_input_ns));
+        o.records.push(ClientRecord {
+            slot: c.slot,
+            admit_tick: admitted.get(&c.slot).copied(),
+            first_sample_ns: sample,
+            first_input_ns: input,
+            first_run_tick: first_run.get(&c.slot).map(|r| r.0),
+            first_run_ns: first_run.get(&c.slot).map(|r| r.1),
+            first_run_due_ns: first_run
+                .get(&c.slot)
+                .map(|r| crate::scenario::due_ns(u64::from(r.0.wrapping_sub(t0)))),
+            first_take_depth: first_take.get(&c.slot).copied(),
+        });
+    }
+    o.rtt_void = rtt_rule(leg, &o.records);
+    Ok(o)
+}
+
+/// The pair's result (7.2): VOID if either leg breaks the setup condition (late, rewinds refused, a command off its
+/// stamp) or the RTT rule, or nothing was compared; else PASS when the comparer names no differing tick, FAIL when it
+/// names one. Returns (result, reason).
+pub(crate) fn verdict(a: &LegOutcome, b: &LegOutcome, cmp: &HashCompare) -> (&'static str, String) {
+    let mut void = Vec::new();
+    for (leg, o) in [("loopback", a), ("L1", b)] {
+        if o.late > 0 {
+            void.push(format!("{leg} late_count {}", o.late));
+        }
+        if o.rewinds_refused > 0 {
+            void.push(format!("{leg} rewinds_refused {}", o.rewinds_refused));
+        }
+        if o.off_stamp > 0 {
+            void.push(format!("{leg} commands off their stamp {}", o.off_stamp));
+        }
+        if let Some(r) = &o.rtt_void {
+            void.push(format!("{leg} RTT rule: {r}"));
+        }
+    }
+    if !void.is_empty() {
+        return ("VOID", void.join("; "));
+    }
+    match cmp {
+        HashCompare::Equal { entries } => {
+            ("PASS", format!("{entries} ticks, every World hash equal"))
+        }
+        HashCompare::Empty => (
+            "VOID",
+            "the comparer compared nothing (Empty is never equal)".to_owned(),
+        ),
+        HashCompare::Mismatch {
+            index,
+            ours,
+            theirs,
+            compared,
+            mismatching,
+        } => (
+            "FAIL",
+            format!(
+                "first differing index {index} (loopback {:?}, L1 {:?}); {mismatching} of {compared} differ",
+                ours.as_ref().map(|t| (t.tick, format!("{:016x}", t.hash))),
+                theirs
+                    .as_ref()
+                    .map(|t| (t.tick, format!("{:016x}", t.hash)))
+            ),
+        ),
+    }
+}
+
+/// `precheck`'s usage line.
+pub(crate) const USAGE: &str = "npt-lab precheck --manifest <precheck-recordings.json> --recording <file> \
+--commit <sha> [--lead <1..7>] [--bite <id> --bite-leg loopback|L1]";
+
+/// The `precheck` subcommand: the recording checked against its manifest entry (its byte length and SHA-256), both legs
+/// run, and the result lines printed (the run script writes them to runs.txt). `Err` for a usage error (2) or a failure
+/// to run (1).
+pub(crate) fn main(argv: &[String]) -> Result<Vec<String>, (u8, String)> {
+    let usage = |m: String| (2u8, m);
+    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
+    let mut it = argv.iter();
+    while let Some(flag) = it.next() {
+        let k = flag
+            .strip_prefix("--")
+            .ok_or(usage(format!("unexpected argument {flag:?}")))?;
+        let v = it.next().ok_or(usage(format!("{flag} needs a value")))?;
+        if map.insert(k, v).is_some() {
+            return Err(usage(format!("{flag} given twice")));
+        }
+    }
+    let manifest = map
+        .remove("manifest")
+        .ok_or(usage("--manifest is required".into()))?;
+    let file = map
+        .remove("recording")
+        .ok_or(usage("--recording is required".into()))?;
+    // No per-leg log is written (the advisor's U4r build ruling 2, point 2): the result lines carry what a header
+    // would, the build first.
+    let commit = map
+        .remove("commit")
+        .ok_or(usage("--commit <40 hex> is required".into()))?;
+    let lead = match map.remove("lead") {
+        None => crate::scenario::lead_for(&Profile::by_name("P0").ok_or((1, "no P0".to_owned()))?),
+        Some(v) => v
+            .parse::<u32>()
+            .ok()
+            .filter(|l| (1..=7).contains(l))
+            .ok_or(usage(format!("--lead {v:?}: 1 to 7")))?,
+    };
+    let bite = map.remove("bite");
+    let bite_leg = match (bite, map.remove("bite-leg")) {
+        (None, None) => None,
+        (Some(_), Some("loopback")) => Some(Leg::Loopback),
+        (Some(_), Some("L1")) => Some(Leg::L1),
+        _ => {
+            return Err(usage(
+                "--bite and --bite-leg loopback|L1 go together".into(),
+            ));
+        }
+    };
+    if let Some(k) = map.keys().next() {
+        return Err(usage(format!("unknown option --{k}")));
+    }
+    let build = crate::machine::build(commit).map_err(usage)?;
+    let m = crate::machine::container();
+    let build_line = format!(
+        "build: commit {}, binary sha256 {}, features {:?}, profile {}; machine: {} {}, {} vCPUs, {} MB, kernel {}",
+        build.commit,
+        build.binary_sha256,
+        build.features,
+        build.profile,
+        m.arch,
+        m.cpu,
+        m.vcpus,
+        m.mem_mb,
+        m.kernel
+    );
+    let text = std::fs::read_to_string(manifest).map_err(|e| (1, format!("{manifest}: {e}")))?;
+    let entries = crate::hash::manifest(&text).map_err(|e| (1, format!("{manifest}: {e}")))?;
+    let [entry] = entries.as_slice() else {
+        return Err((
+            1,
+            format!("{manifest}: the pre-check's manifest holds one entry"),
+        ));
+    };
+    let bytes = std::fs::read(file).map_err(|e| (1, format!("{file}: {e}")))?;
+    let sha = {
+        use sha2::{Digest, Sha256};
+        Sha256::digest(&bytes)
+            .iter()
+            .map(|b| format!("{b:02x}"))
+            .collect::<String>()
+    };
+    if bytes.len() as u64 != entry.bytes || sha != entry.sha256 {
+        return Err((
+            1,
+            format!(
+                "{file}: {} bytes, sha256 {sha}: not the manifest's {} ({} bytes, {})",
+                bytes.len(),
+                entry.name,
+                entry.bytes,
+                entry.sha256
+            ),
+        ));
+    }
+    let rec = Recording::decode(&bytes).map_err(|e| (1, format!("{file}: {e}")))?;
+    let mut lines = vec![
+        build_line,
+        format!(
+            "recording {}: {} bytes, sha256 {sha}, as the manifest's; generator {}, first_tick {}, span_ms {}, ticks {}",
+            entry.name,
+            bytes.len(),
+            rec.generator,
+            rec.first_tick,
+            rec.span_ms,
+            rec.ticks.len()
+        ),
+    ];
+    lines.extend(pair(&rec, &entry.name, lead, bite.zip(bite_leg)).map_err(|e| (1, e))?);
+    Ok(lines)
+}
+
+/// Both legs and the result lines (the module text).
+pub(crate) fn pair(
+    rec: &Recording,
+    name: &str,
+    lead: u32,
+    bite: Option<(&str, Leg)>,
+) -> Result<Vec<String>, String> {
+    let on = |leg: Leg| bite.filter(|b| b.1 == leg).map(|b| b.0);
+    let a = run_leg(rec, name, Leg::Loopback, lead, on(Leg::Loopback))?;
+    let b = run_leg(rec, name, Leg::L1, lead, on(Leg::L1))?;
+    let cmp = HashCompare::of(&a.hashes, &b.hashes);
+    let (result, why) = verdict(&a, &b, &cmp);
+    let ring =
+        npt_sim::HitHistory::for_ms(crate::scenario::ROOM_HISTORY_MS).map_or(0, |h| h.capacity());
+    let mut lines = vec![format!(
+        "{SUITE} {name}: {} ticks from tick {} (the tick base, both legs: World::new(scene, {}, {})); lead {lead}; \
+         buffer_depth_ticks_count {lead} (the pacing lead, the value set), both legs; cmd_timing B; bite {}",
+        rec.ticks.len(),
+        rec.ticks.first().map_or(rec.first_tick, |t| t.tick),
+        rec.seed,
+        rec.first_tick,
+        bite.map_or("none".to_owned(), |(id, leg)| format!(
+            "{id} on the {} leg",
+            leg.name()
+        ))
+    )];
+    for (leg, o) in [(Leg::Loopback, &a), (Leg::L1, &b)] {
+        let admits: Vec<u32> = o.records.iter().filter_map(|r| r.admit_tick).collect();
+        lines.push(format!(
+            "leg {}: room configuration: layer {}, profile {}, cmd_timing B, lead {}, tick base World::new(scene, \
+             {}, {}), seed {}, ring {} ms = {ring} ticks (HitHistory::for_ms)",
+            leg.name(),
+            leg.name(),
+            if leg == Leg::L1 { "P0" } else { "none" },
+            o.lead,
+            rec.seed,
+            rec.first_tick,
+            rec.seed,
+            crate::scenario::ROOM_HISTORY_MS
+        ));
+        lines.push(format!(
+            "leg {}: buffer_depth_ticks_count {} (the pacing lead, the value set); the room's measured depth at its \
+             take (input.rs:28-29: the newest buffered command's tick minus the tick taken) is L - 1 by definition under \
+             this pacing: over the takes after each client's first (depth: takes) {:?}; takes where it is not L - 1: {}, \
+             the first 64 (client, tick, measured) {:?}",
+            leg.name(),
+            o.lead,
+            o.depth,
+            o.depth_deviations,
+            o.deviations
+        ));
+        lines.push(format!(
+            "leg {}: refused Rewinds per command: {}, the first 64 (client, tick, sequence) {:?}",
+            leg.name(),
+            o.refused_logged,
+            o.refused
+        ));
+        lines.push(format!(
+            "leg {}: clients admitted {} (admit ticks {}..{}); ended {:?}; late_count {}; rewinds_refused {}; commands \
+             off their stamp {}; scripted {}; view-time replacements and clipped shots: not built, 0 by construction; \
+             delayed command {}; RTT rule: {}",
+            leg.name(),
+            admits.len(),
+            admits.iter().min().copied().unwrap_or(0),
+            admits.iter().max().copied().unwrap_or(0),
+            o.ended,
+            o.late,
+            o.rewinds_refused,
+            o.off_stamp,
+            o.scripted,
+            o.delayed.map_or("none".to_owned(), |t| format!("tick {t}")),
+            o.rtt_void.as_deref().unwrap_or(if leg == Leg::Loopback {
+                "Some(0) from the start"
+            } else {
+                "holds"
+            })
+        ));
+        for r in &o.records {
+            lines.push(format!(
+                "  {} client {}: admit tick {:?}; first sample {:?} ns; first input {:?} ns; first run tick {:?} at {:?} \
+                 ns (its due instant {:?} ns); measured depth at its first take {:?}",
+                leg.name(),
+                r.slot,
+                r.admit_tick,
+                r.first_sample_ns,
+                r.first_input_ns,
+                r.first_run_tick,
+                r.first_run_ns,
+                r.first_run_due_ns,
+                r.first_take_depth
+            ));
+        }
+    }
+    lines.push(format!(
+        "comparer (HashCompare::of, loopback against L1): {cmp:?}"
+    ));
+    lines.push(format!("result: {result} ({why})"));
+    Ok(lines)
+}
+
+/// Each leg's first run tick check (tests and the bites' records): the result line of `pair`'s output.
+#[cfg(test)]
+fn result_of(lines: &[String]) -> &str {
+    lines.last().map_or("", String::as_str)
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    /// The SS P0 seed 4 recording's first 120 ticks (its runner entry's settings, precheck-recordings.json, with a 4 s
+    /// warm-up and no measured phase: the run is the same, cut sooner), enough for every client's first commands.
+    fn short_recording() -> Recording {
+        let b =
+            crate::scenario::record(&Scene::stress(), "P0", 4, 48, 0, 4, 0).expect("a recording");
+        Recording::decode(&b).expect("it decodes")
+    }
+
+    fn rec(slot: u8, sample: Option<u64>, run: Option<u64>) -> ClientRecord {
+        ClientRecord {
+            slot,
+            first_sample_ns: sample,
+            first_run_ns: run,
+            ..ClientRecord::default()
+        }
+    }
+
+    /// r7 (U4R-Q10 (b')), case 1: a first-sample instant after the first command's run instant gives VOID.
+    #[test]
+    fn r7_a_first_sample_after_the_first_command_s_run_is_void() {
+        let v = rtt_rule(
+            Leg::L1,
+            &[rec(0, Some(10), Some(10)), rec(1, Some(101), Some(100))],
+        );
+        assert!(v.is_some_and(|r| r.contains("client 1") && r.contains("before its first sample")));
+    }
+
+    /// r7, case 2: equal instants are not VOID (the pump comes before the tick).
+    #[test]
+    fn r7_equal_instants_are_not_void() {
+        assert_eq!(rtt_rule(Leg::L1, &[rec(0, Some(100), Some(100))]), None);
+        assert_eq!(rtt_rule(Leg::L1, &[rec(0, Some(99), Some(100))]), None);
+    }
+
+    /// r7, case 3: no first-sample instant on the L1 leg when the first command runs gives VOID.
+    #[test]
+    fn r7_no_first_sample_on_the_l1_leg_is_void() {
+        let v = rtt_rule(
+            Leg::L1,
+            &[rec(0, Some(1), Some(5)), rec(7, None, Some(100))],
+        );
+        assert!(v.is_some_and(|r| r.contains("client 7") && r.contains("no first-sample")));
+        assert_eq!(
+            rtt_rule(Leg::L1, &[rec(7, None, None)]),
+            None,
+            "no command ran: nothing to judge"
+        );
+    }
+
+    /// r7, case 4: the Loopback leg ("Some(0) from the start") never gives VOID, whatever its records say.
+    #[test]
+    fn r7_the_loopback_leg_never_gives_void() {
+        assert_eq!(
+            rtt_rule(
+                Leg::Loopback,
+                &[rec(0, None, Some(100)), rec(1, Some(200), Some(100))]
+            ),
+            None
+        );
+    }
+
+    /// r6, the pre-check's pacing test (7.2's pacing rule): a short clean replay of the SS P0 recording over both legs
+    /// gives `late_count` 0 and `rewinds_refused` 0 on both, every command run at its stamped tick, the room's measured
+    /// depth L - 1 at every take after each client's first (input.rs:28-29: the newest buffered command's tick minus
+    /// the tick taken; the advisor's U4r build ruling 2, point 1), each first command run at its tick's due instant,
+    /// every client admitted, the RTT rule holding, and equal per-tick World hashes (PASS).
+    #[test]
+    fn r6_a_clean_replay_runs_every_command_at_its_stamp_on_both_legs() {
+        let r = short_recording();
+        // `precheck`'s default lead, P0's (main: `lead_for`).
+        let p0 = crate::scenario::lead_for(&Profile::by_name("P0").expect("P0"));
+        let lines = pair(&r, "SS-P0-s4", p0, None).expect("both legs run");
+        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
+            println!("r6: {l}");
+        }
+        let a = run_leg(&r, "SS-P0-s4", Leg::Loopback, p0, None).expect("loopback");
+        let b = run_leg(&r, "SS-P0-s4", Leg::L1, p0, None).expect("L1");
+        for (leg, o) in [("loopback", &a), ("L1", &b)] {
+            assert_eq!((o.late, o.rewinds_refused, o.off_stamp), (0, 0, 0), "{leg}");
+            assert_eq!(o.records.len(), 48, "{leg}");
+            assert!(
+                o.records
+                    .iter()
+                    .all(|c| c.admit_tick.is_some() && c.first_run_tick.is_some()),
+                "{leg}"
+            );
+            println!(
+                "r6 {leg}: the room's measured depth at its take {:?}; {} scripted; ticks {}",
+                o.depth,
+                o.scripted,
+                o.hashes.len()
+            );
+            assert!(o.ended.is_empty(), "{leg}: {:?}", o.ended);
+            assert_eq!(o.rtt_void, None, "{leg}");
+            // Point 1 (4): L - 1 at every take after each client's first, but at the recording's last L - 1 ticks,
+            // where the recording holds no command stamped j + L - 1, so the newest held is its last tick: there the
+            // measured depth is exactly (last tick - j), one take per client and tick, and nothing else deviates. L is
+            // the leg's configured lead, and the list compared is whole (at most 64 kept): the advisor's U4r build
+            // ruling 3, point 5.
+            let lead = o.lead;
+            assert_eq!(lead, p0, "{leg}: the leg's configured lead");
+            assert!(
+                o.depth_deviations <= 64 && o.deviations.len() as u64 == o.depth_deviations,
+                "{leg}: {} deviations, {} kept: the list compared must be whole",
+                o.depth_deviations,
+                o.deviations.len()
+            );
+            let last = r.ticks.last().map(|t| t.tick).expect("ticks");
+            let slots: Vec<u8> = o.records.iter().map(|c| c.slot).collect();
+            let end: Vec<(u8, u32, u32)> = (1..lead)
+                .rev()
+                .flat_map(|back| {
+                    let j = last - (back - 1);
+                    slots.iter().map(move |&s| (s, j, last - j))
+                })
+                .collect();
+            let mut seen = o.deviations.clone();
+            seen.sort_by_key(|&(s, j, _)| (j, s));
+            assert_eq!(
+                (o.depth_deviations, seen),
+                (end.len() as u64, end),
+                "{leg}: the deviations are the recording's end alone"
+            );
+            assert_eq!(
+                o.depth.get(&(lead - 1)).copied().unwrap_or(0) + o.depth_deviations,
+                o.depth.values().sum::<u64>(),
+                "{leg}: {:?}",
+                o.depth
+            );
+            let first_takes: BTreeMap<Option<u32>, usize> =
+                o.records.iter().fold(BTreeMap::new(), |mut m, r| {
+                    *m.entry(r.first_take_depth).or_default() += 1;
+                    m
+                });
+            println!(
+                "r6 {leg}: the measured depth at each client's first take (depth: clients) {first_takes:?}; after it \
+                 {:?}; not L - 1 only at the recording's last tick {last} ({} takes)",
+                o.depth, o.depth_deviations
+            );
+            // The Low finding: each first command ran at its tick's due instant (no catch-up at P0).
+            assert!(
+                o.records
+                    .iter()
+                    .all(|r| r.first_run_ns == r.first_run_due_ns),
+                "{leg}"
+            );
+            assert_eq!(o.refused_logged, 0, "{leg}: {:?}", o.refused);
+        }
+        assert_eq!(
+            HashCompare::of(&a.hashes, &b.hashes),
+            HashCompare::Equal {
+                entries: r.ticks.len()
+            }
+        );
+        assert_eq!(result_of(&lines).split(' ').nth(1), Some("PASS"));
+    }
+
+    /// The late-command bite (2.5, 7.2; bites builds only): one replay client on the L1 leg holds T and every later
+    /// command until T has run, then sends T first: that leg's `late_count` is exactly 1, and the pair is VOID by it alone.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn precheck_bite_late_command_makes_the_pair_void_by_late_count_alone() {
+        let r = short_recording();
+        let lines = pair(
+            &r,
+            "SS-P0-s4",
+            2,
+            Some(("PM-0001-30/late-command", Leg::L1)),
+        )
+        .expect("both legs run");
+        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
+            println!("late-command bite: {l}");
+        }
+        let b = run_leg(&r, "SS-P0-s4", Leg::L1, 2, Some("PM-0001-30/late-command")).expect("L1");
+        assert_eq!(b.late, 1, "exactly the one delayed command");
+        assert!(b.delayed.is_some());
+        assert_eq!((b.rewinds_refused, b.rtt_void.clone()), (0, None));
+        // The advisor's U4r build ruling 2, point 4 (b): VOID by late_count alone, exactly.
+        assert_eq!(result_of(&lines), "result: VOID (L1 late_count 1)");
+        assert_eq!(b.off_stamp, 0, "the L1 leg runs no command off its stamp");
+        let a = run_leg(&r, "SS-P0-s4", Leg::Loopback, 2, None).expect("loopback");
+        assert_eq!(
+            (a.late, a.rewinds_refused, a.off_stamp, a.rtt_void.clone()),
+            (0, 0, 0, None),
+            "the Loopback leg is clean"
+        );
+        // Point 1 (5): the bitten client's depth deviations around T are the bite's.
+        println!(
+            "late-command bite: L1 depth deviations {} (the bite's, around tick {:?}): {:?}",
+            b.depth_deviations, b.delayed, b.deviations
+        );
+    }
+
+    /// The advisor's U4r build ruling 2, point 2 (1): `--commit` is required; without it `precheck` is a usage error
+    /// (exit 2) before any file is read.
+    #[test]
+    fn precheck_without_commit_is_a_usage_error() {
+        let argv: Vec<String> = ["--manifest", "m.json", "--recording", "r.rec"]
+            .iter()
+            .map(|s| (*s).to_owned())
+            .collect();
+        let e = main(&argv).expect_err("no --commit");
+        assert_eq!(e.0, 2, "{}", e.1);
+        assert!(e.1.contains("--commit"), "{}", e.1);
+    }
+
+    /// The wallclock bite (2.5, 7.2; bites builds only): the L1 leg's pump changes each command's aim by the wall clock,
+    /// so the pair is valid and the comparer names a differing tick (FAIL); a PASS would be a missed bite.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn precheck_bite_wallclock_gives_a_valid_pair_whose_comparer_names_a_tick() {
+        let r = short_recording();
+        let lines = pair(&r, "SS-P0-s4", 2, Some(("PM-0001-30/wallclock", Leg::L1)))
+            .expect("both legs run");
+        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
+            println!("wallclock bite: {l}");
+        }
+        let result = result_of(&lines);
+        assert!(
+            result.starts_with("result: FAIL (first differing index"),
+            "{result}"
+        );
+    }
+}
````

### npt-lab: replay.rs (new: `replay`, the x86-64 re-decode)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs`:  1 file changed, 429 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs
new file mode 100644
index 0000000..241ef0d
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs
@@ -0,0 +1,429 @@
+//! `replay`, the x86-64 re-decode ([`ReplayX86`]; u4r-brief.md 2.1 and 2.4 r3; plan :753-755, :761-764; R-8, the
+//! advisor's condition 9): a run's logged client bytes (`scenario --bytes`: each client's received messages, step by
+//! step, with each step's host instant) decoded again natively by a fresh `ClientEndpoint` per client, with the run's
+//! join, lead and starts (read from the run's client log header), and the hashes it files written as `hs.replay_hash`
+//! in a `replay` log (process `replay`, this machine's arch: only an x86_64 one is read, judge/src/figures.rs:222-233),
+//! never in another log. Commands never reach the decoder (`ClientEndpoint::input` decodes what it is handed before it
+//! stamps a command: client/src/lib.rs:379-410), so the re-decode hands each step its bytes with no command and calls
+//! `step` as the run did; the hashes come back the run's. The re-decoder's client takes the run's bite when the run had
+//! one (the same client), or the one `--bite` names: r3's is npt-client's own PM-0001-17/client-rounding (its grid
+//! floored, `npt_client::grid_with_bite`, client/src/lib.rs:37-40), so npt-lab plants no new id.
+
+use std::collections::BTreeMap;
+use std::path::{Path, PathBuf};
+
+use npt_client::ClientEndpoint;
+use npt_link::Channel;
+use npt_log::{Ended, Header, Log, Phase, TickRecord, Writer};
+use serde_json::Value;
+
+use crate::hash::Failure;
+use crate::scenario::{Bites, join_of};
+
+/// `replay`'s usage line.
+pub(crate) const USAGE: &str =
+    "npt-lab replay --logs <run folder> --bytes <folder> --out <folder> [--bite <id>]";
+
+/// One `replay` call's settings.
+#[derive(Clone, Debug)]
+pub(crate) struct Settings {
+    /// The run's folder: its `client.jsonl` gives the header and the hashes compared.
+    pub(crate) logs: PathBuf,
+    /// The run's `--bytes` folder: one `client-<slot>.bytes` per client.
+    pub(crate) bytes: PathBuf,
+    /// Where `replay.jsonl` goes (never overwritten).
+    pub(crate) out: PathBuf,
+    pub(crate) bite: Option<String>,
+}
+
+/// `replay`'s options (after `replay`), as `--name value` pairs, each once.
+pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
+    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
+    let mut it = argv.iter();
+    while let Some(flag) = it.next() {
+        let name = flag
+            .strip_prefix("--")
+            .ok_or(format!("unexpected argument {flag:?}"))?;
+        let value = it.next().ok_or(format!("{flag} needs a value"))?;
+        if map.insert(name, value).is_some() {
+            return Err(format!("{flag} given twice"));
+        }
+    }
+    let mut take = |k: &str| map.remove(k).map(PathBuf::from);
+    let logs = take("logs").ok_or("--logs is required")?;
+    let bytes = take("bytes").ok_or("--bytes is required")?;
+    let out = take("out").ok_or("--out is required")?;
+    let bite = map.remove("bite").map(str::to_owned);
+    if let Some(k) = map.keys().next() {
+        return Err(format!("unknown option --{k}"));
+    }
+    Ok(Settings {
+        logs,
+        bytes,
+        out,
+        bite,
+    })
+}
+
+/// One logged client step: its host instant and what it was handed, as (channel, bytes, arrival ns).
+pub(crate) type Step = (u64, Vec<(Channel, Vec<u8>, u64)>);
+
+struct Cursor<'a> {
+    b: &'a [u8],
+    at: usize,
+}
+
+impl<'a> Cursor<'a> {
+    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
+        let end = self
+            .at
+            .checked_add(n)
+            .filter(|e| *e <= self.b.len())
+            .ok_or_else(|| format!("cut short at byte {}", self.at))?;
+        let s = &self.b[self.at..end];
+        self.at = end;
+        Ok(s)
+    }
+
+    fn u64(&mut self) -> Result<u64, String> {
+        let mut w = [0u8; 8];
+        w.copy_from_slice(self.take(8)?);
+        Ok(u64::from_le_bytes(w))
+    }
+
+    fn u32(&mut self) -> Result<u32, String> {
+        let mut w = [0u8; 4];
+        w.copy_from_slice(self.take(4)?);
+        Ok(u32::from_le_bytes(w))
+    }
+}
+
+/// One client's bytes file, step by step (scenario.rs `write_step`'s format: host instant u64, count u32, then per
+/// message channel u8 (0 datagram, 1 control, 2 full state), arrival u64, length u32 and the bytes, little-endian).
+pub(crate) fn read_steps(bytes: &[u8]) -> Result<Vec<Step>, String> {
+    let mut c = Cursor { b: bytes, at: 0 };
+    let mut out = Vec::new();
+    while c.at < bytes.len() {
+        let host = c.u64()?;
+        let count = c.u32()?;
+        let mut got = Vec::new();
+        for _ in 0..count {
+            let channel = match c.take(1)?[0] {
+                0 => Channel::Datagram,
+                1 => Channel::Control,
+                2 => Channel::FullState,
+                x => return Err(format!("channel code {x} at byte {}", c.at - 1)),
+            };
+            let arrival = c.u64()?;
+            let len = c.u32()? as usize;
+            got.push((channel, c.take(len)?.to_vec(), arrival));
+        }
+        out.push((host, got));
+    }
+    Ok(out)
+}
+
+/// A header param as a whole number (the run's client log header).
+fn param(h: &Header, name: &str) -> Result<u64, String> {
+    h.params
+        .get(name)
+        .and_then(Value::as_u64)
+        .ok_or(format!("the run's client log header has no param {name}"))
+}
+
+/// The x86-64 re-decoder of one run (plan.md:617's `ReplayX86`): its clients' join, lead and starts, and the bite its
+/// clients take.
+pub(crate) struct ReplayX86 {
+    seed: u64,
+    lead: u32,
+    cmd_start: u32,
+    event_start: u32,
+    first_tick: u32,
+    bites: Bites,
+}
+
+impl ReplayX86 {
+    /// The re-decoder of the run whose client log header is `h`; `bite` the one `--bite` names (else the run's).
+    pub(crate) fn of(h: &Header, bite: Option<&str>) -> Result<ReplayX86, String> {
+        let small = |name: &str| {
+            u32::try_from(param(h, name)?).map_err(|_| format!("param {name} is above 32 bits"))
+        };
+        let bites = match bite.or(h.bite.as_deref()) {
+            None => Bites::default(),
+            Some(id) => Bites::of(id)?,
+        };
+        Ok(ReplayX86 {
+            seed: h.seed,
+            lead: small("lead_ticks_count")?,
+            cmd_start: small("cmd_start_count")?,
+            event_start: small("event_start_count")?,
+            first_tick: small("tick_start")?,
+            bites,
+        })
+    }
+
+    /// A 32-bit tick as the logs' never-wrapping tick: forward from the run's first tick (logs.rs `wide`).
+    fn wide(&self, t: u32) -> u64 {
+        u64::from(self.first_tick) + u64::from(t.wrapping_sub(self.first_tick))
+    }
+
+    /// Slot `slot`'s steps decoded again: the hashes its endpoint files, by wide tick (two different hashes for one
+    /// tick give the sentinel 0, as the client writer's rule: logs.rs `filed`).
+    pub(crate) fn decode(&self, slot: u8, steps: &[Step]) -> BTreeMap<u64, u64> {
+        let e = ClientEndpoint::new(join_of(self.seed, slot), npt_sim::TICK_HZ)
+            .with_lead(self.lead)
+            .with_starts(self.cmd_start, self.event_start);
+        let mut e = self.bites.client(e);
+        let mut out: BTreeMap<u64, u64> = BTreeMap::new();
+        for (host, got) in steps {
+            e.input(None, got, *host);
+            for (t, h) in e.step().hashes {
+                let w = self.wide(t);
+                let v = out.entry(w).or_insert(h);
+                if *v != h {
+                    *v = 0;
+                }
+            }
+        }
+        out
+    }
+}
+
+/// What one `replay` call found against the run's client log.
+#[derive(Clone, Debug, Default, PartialEq, Eq)]
+pub(crate) struct Outcome {
+    pub(crate) clients: usize,
+    pub(crate) steps: u64,
+    pub(crate) replay_hashes: u64,
+    /// (client, tick) pairs at which the client log holds a hash.
+    pub(crate) compared: u64,
+    /// Of those, the ones the replay gives no hash or another hash for, plus every replay hash at a (client, tick)
+    /// where the client log holds none.
+    pub(crate) mismatches: u64,
+    pub(crate) first: Option<String>,
+}
+
+/// Compares the client log's `hs.decoded_hash` with the replay's hashes `by_tick` (wide tick to one entry per slot).
+pub(crate) fn compare(client: &Log, by_tick: &BTreeMap<u64, Vec<Option<u64>>>) -> Outcome {
+    let mut o = Outcome::default();
+    let miss = |o: &mut Outcome, text: String| {
+        o.mismatches += 1;
+        o.first.get_or_insert(text);
+    };
+    let mut seen: BTreeMap<(u64, usize), u64> = BTreeMap::new();
+    for rec in &client.ticks {
+        let Some(list) = rec.hashes("decoded_hash") else {
+            continue;
+        };
+        for (slot, h) in list.iter().enumerate() {
+            let Some(h) = h else { continue };
+            o.compared += 1;
+            seen.insert((rec.t, slot), *h);
+            let r = by_tick
+                .get(&rec.t)
+                .and_then(|l| l.get(slot).copied().flatten());
+            if r != Some(*h) {
+                miss(
+                    &mut o,
+                    format!(
+                        "client {slot} tick {}: logged {h:016x}, replay {r:x?}",
+                        rec.t
+                    ),
+                );
+            }
+        }
+    }
+    for (t, list) in by_tick {
+        for (slot, r) in list.iter().enumerate() {
+            if let Some(r) = r
+                && !seen.contains_key(&(*t, slot))
+            {
+                miss(
+                    &mut o,
+                    format!(
+                        "client {slot} tick {t}: the replay gives {r:016x}, the client log nothing"
+                    ),
+                );
+            }
+        }
+    }
+    o
+}
+
+fn read_log(path: &Path) -> Result<Log, String> {
+    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
+    npt_log::read_log(std::io::BufReader::new(f)).map_err(|e| format!("{}: {e}", path.display()))
+}
+
+/// One `replay` call (the module text): every client's bytes decoded again, `replay.jsonl` written into `s.out`, and
+/// the outcome against the run's client log.
+pub(crate) fn run(s: &Settings) -> Result<Outcome, Failure> {
+    let fail = Failure::Failed;
+    let client = read_log(&s.logs.join("client.jsonl")).map_err(fail)?;
+    let h = &client.header;
+    let rx = ReplayX86::of(h, s.bite.as_deref()).map_err(Failure::Usage)?;
+    let n = usize::try_from(h.clients).map_err(|_| fail("clients above usize".to_owned()))?;
+    let mut by_tick: BTreeMap<u64, Vec<Option<u64>>> = BTreeMap::new();
+    let mut o = Outcome {
+        clients: n,
+        ..Outcome::default()
+    };
+    for slot in 0..n {
+        let s8 = u8::try_from(slot).map_err(|_| fail(format!("slot {slot} above 255")))?;
+        let p = s.bytes.join(format!("client-{slot}.bytes"));
+        let bytes = std::fs::read(&p).map_err(|e| fail(format!("{}: {e}", p.display())))?;
+        let steps = read_steps(&bytes).map_err(|e| fail(format!("{}: {e}", p.display())))?;
+        o.steps += steps.len() as u64;
+        for (w, hash) in rx.decode(s8, &steps) {
+            by_tick.entry(w).or_insert_with(|| vec![None; n])[slot] = Some(hash);
+            o.replay_hashes += 1;
+        }
+    }
+    // The replay log: the run's header as the client log holds it, but its process, machine, build and bite.
+    let mut header = h.clone();
+    header.process = "replay".to_owned();
+    header.machine = crate::machine::container();
+    header.build = crate::machine::build(&h.build.commit).map_err(fail)?;
+    if s.bite.is_some() {
+        header.bite = s.bite.clone();
+    }
+    std::fs::create_dir_all(&s.out).map_err(|e| fail(format!("{}: {e}", s.out.display())))?;
+    let path = s.out.join("replay.jsonl");
+    let file = std::fs::File::create_new(&path).map_err(|e| {
+        fail(format!(
+            "{}: {e} (a log is never overwritten)",
+            path.display()
+        ))
+    })?;
+    let io = |e: std::io::Error| fail(format!("{}: {e}", path.display()));
+    let mut w = Writer::new(std::io::BufWriter::new(file), &header).map_err(io)?;
+    let phases: BTreeMap<u64, Phase> = client.ticks.iter().map(|r| (r.t, r.ph)).collect();
+    for (t, list) in &by_tick {
+        let ph = phases
+            .range(..=*t)
+            .next_back()
+            .map_or(Phase::Warmup, |(_, p)| *p);
+        let mut rec = TickRecord::new(*t, ph);
+        rec.set_hashes("replay_hash", list);
+        w.tick(&rec).map_err(io)?;
+    }
+    w.finish(Ended::Clean).map_err(io)?;
+    let c = compare(&client, &by_tick);
+    Ok(Outcome {
+        compared: c.compared,
+        mismatches: c.mismatches,
+        first: c.first,
+        ..o
+    })
+}
+
+/// The summary line of a `replay` call.
+pub(crate) fn summary_line(s: &Settings, o: &Outcome) -> String {
+    format!(
+        "replay of {}: {} clients, {} steps, {} replay hashes; against the client log: {} (client, tick) pairs \
+         compared, {} mismatches{}",
+        s.logs.display(),
+        o.clients,
+        o.steps,
+        o.replay_hashes,
+        o.compared,
+        o.mismatches,
+        o.first
+            .as_ref()
+            .map(|f| format!("; the first: {f}"))
+            .unwrap_or_default()
+    )
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::scenario::tests::{Scratch, read, scratch, settings};
+
+    /// r3's run: SS, 16 clients, P2, 6 s of warm-up and 2 s measured, each client's received bytes logged (under the
+    /// test binary's target dir: `scratch`).
+    fn r3_run(name: &str) -> (Scratch, Scratch) {
+        let dir = scratch(name);
+        let bytes = scratch(&format!("{name}-bytes"));
+        let s = settings(
+            &dir,
+            &format!(
+                "--seed 1 --profile P2 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10 --bytes {}",
+                bytes.display()
+            ),
+        );
+        crate::scenario::run(&s).unwrap_or_else(|e| panic!("r3's run: {e:?}"));
+        (dir, bytes)
+    }
+
+    /// The bytes file's reader takes back what `write_step` wrote, and refuses a cut-short file and an unknown channel.
+    #[test]
+    fn r3_the_bytes_file_reads_back_step_by_step() {
+        let steps: Vec<Step> = vec![
+            (5, Vec::new()),
+            (
+                9,
+                vec![
+                    (Channel::Control, vec![1, 2], 7),
+                    (Channel::Datagram, vec![], 8),
+                ],
+            ),
+            (12, vec![(Channel::FullState, vec![3; 300], 11)]),
+        ];
+        let mut b = Vec::new();
+        for (host, got) in &steps {
+            crate::scenario::write_step(&mut b, *host, got).expect("writes");
+        }
+        assert_eq!(read_steps(&b).expect("reads"), steps);
+        assert!(read_steps(&b[..b.len() - 1]).is_err(), "cut short");
+        let mut bad = b.clone();
+        bad[20] = 9;
+        assert!(read_steps(&bad).is_err(), "channel code 9");
+    }
+
+    /// r3, `replay`'s U4 test (plan :761-764): one short L1 run (SS P2) logs each client's received bytes; `replay`,
+    /// decoding them again, gives each client's logged hash at every (client, tick) that has one, with more than 0
+    /// compared, and its log is a `replay` log with one entry per client.
+    #[test]
+    fn r3_the_re_decode_gives_every_client_s_logged_hash() {
+        let (dir, bytes) = r3_run("r3");
+        let out = scratch("r3-replay");
+        let s = Settings {
+            logs: dir.to_path_buf(),
+            bytes: bytes.to_path_buf(),
+            out: out.to_path_buf(),
+            bite: None,
+        };
+        let o = run(&s).unwrap_or_else(|e| panic!("replay: {e:?}"));
+        println!("r3: {}", summary_line(&s, &o));
+        assert!(o.compared > 0, "a replay that compares nothing fails");
+        assert_eq!(o.mismatches, 0, "{:?}", o.first);
+        let log = read(&out, "replay");
+        assert_eq!(log.header.process, "replay");
+        assert!(
+            log.ticks
+                .iter()
+                .all(|t| t.hashes("replay_hash").is_some_and(|l| l.len() == 16)
+                    && !t.hs.contains_key("decoded_hash"))
+        );
+    }
+
+    /// r3's bite (plan :763-764): the re-decoder's grid floored (npt-client's PM-0001-17/client-rounding, given to
+    /// `--bite`: no new id) gives at least one (client, tick) mismatch.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn r3_bite_a_floored_re_decoder_grid_gives_a_mismatch() {
+        let (dir, bytes) = r3_run("r3-bite");
+        let out = scratch("r3-bite-replay");
+        let s = Settings {
+            logs: dir.to_path_buf(),
+            bytes: bytes.to_path_buf(),
+            out: out.to_path_buf(),
+            bite: Some("PM-0001-17/client-rounding".to_owned()),
+        };
+        let o = run(&s).unwrap_or_else(|e| panic!("replay: {e:?}"));
+        println!("r3 bite: {}", summary_line(&s, &o));
+        assert!(o.compared > 0);
+        assert!(o.mismatches >= 1, "the floored grid must give a mismatch");
+    }
+}
````

### npt-lab: hash.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/hash.rs .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs`:  1 file changed, 316 insertions(+), 7 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/hash.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs
index 6bb355d..7a4a92f 100644
--- a/spikes/netcode-proof/crates/lab/src/hash.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -195,10 +195,27 @@ pub(crate) fn parse(argv: &[String]) -> Result<Mode, String> {
     Ok(parsed)
 }
 
+/// How an entry's recording is made (carry 82: the schema's `kind` key and the dispatch `manifest` and
+/// `regenerate_all` need).
+#[derive(Clone, Debug, PartialEq, Eq)]
+pub(crate) enum Kind {
+    /// `Recording::synthetic` of the entry's scene, seed, first tick, ticks and span (generator 2).
+    Synthetic,
+    /// A runner recording: `npt-lab scenario --record` at these settings (generator 0), the brawl on L1 with the
+    /// entry's scene, seed and first tick, re-run whole to regenerate it (u4r-brief.md 2.1 (ii)).
+    Runner {
+        profile: String,
+        clients: u8,
+        warmup_s: u64,
+        measured_s: u64,
+    },
+}
+
 /// One recordings manifest entry (u4-build-plan.md §4.15; crates/lab/recordings.json).
 #[derive(Clone, Debug, PartialEq, Eq)]
 pub(crate) struct Entry {
     pub(crate) name: String,
+    pub(crate) kind: Kind,
     pub(crate) scene: u8,
     pub(crate) seed: u64,
     pub(crate) first_tick: u32,
@@ -209,7 +226,18 @@ pub(crate) struct Entry {
     pub(crate) sha256: String,
 }
 
-/// The manifest's entries, in its order.
+/// The fewest bytes one tick of a recording takes (its tick and four counts: replay.rs's tick header), so no
+/// recording of `bytes` holds more than `bytes / 12` ticks (carry 82).
+const TICK_HEADER_BYTES: u64 = 12;
+
+/// Whether `name` is a manifest name: one or more of `A-Z`, `a-z`, `0-9` and `-` (carry 82: it reaches a file path and
+/// the `h <recording> <tick> <hash>` line).
+fn name_ok(name: &str) -> bool {
+    !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
+}
+
+/// The manifest's entries, in its order. Each entry's name and its `ticks` against its `bytes` are checked here,
+/// before anything is generated (carry 82).
 pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
     let v: Value =
         serde_json::from_str(text).map_err(|e| format!("the manifest is not JSON: {e}"))?;
@@ -235,7 +263,34 @@ pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
                     .map_err(|_| format!("manifest entry {name}: \"{k}\" {n} is above 32 bits"))
             })
         };
+        if !name_ok(&name) {
+            return Err(format!(
+                "manifest entry {i}: name {name:?} is not one or more of A-Z, a-z, 0-9 and -"
+            ));
+        }
+        let kind = match e.get("kind").and_then(Value::as_str) {
+            Some("synthetic") => Kind::Synthetic,
+            Some("runner") => Kind::Runner {
+                profile: e
+                    .get("profile")
+                    .and_then(Value::as_str)
+                    .ok_or(format!(
+                        "manifest entry {name}: a runner entry with no \"profile\""
+                    ))?
+                    .to_owned(),
+                clients: u8::try_from(num("clients")?)
+                    .map_err(|_| format!("manifest entry {name}: \"clients\" above 255"))?,
+                warmup_s: num("warmup_s")?,
+                measured_s: num("measured_s")?,
+            },
+            other => {
+                return Err(format!(
+                    "manifest entry {name}: \"kind\" {other:?}, not \"synthetic\" or \"runner\""
+                ));
+            }
+        };
         let entry = Entry {
+            kind,
             scene: u8::try_from(num("scene")?)
                 .map_err(|_| format!("manifest entry {name}: no such scene"))?,
             seed: num("seed")?,
@@ -254,6 +309,12 @@ pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
         if out.iter().any(|o: &Entry| o.name == entry.name) {
             return Err(format!("manifest entry {} is named twice", entry.name));
         }
+        if u64::from(entry.ticks) > entry.bytes / TICK_HEADER_BYTES {
+            return Err(format!(
+                "manifest entry {}: {} ticks cannot fit {} bytes (at most bytes / {TICK_HEADER_BYTES})",
+                entry.name, entry.ticks, entry.bytes
+            ));
+        }
         out.push(entry);
     }
     Ok(out)
@@ -266,10 +327,38 @@ fn sha256_hex(bytes: &[u8]) -> String {
         .collect()
 }
 
-/// The entry's recording, generated (after its generator is checked: `regenerate_all` checks every entry first).
+/// The entry's recording, generated (after its generator is checked: `regenerate_all` checks every entry first): the
+/// synthetic generator's, or a runner recording's whole scenario re-run (carry 82).
 fn generate(e: &Entry) -> Result<Vec<u8>, String> {
     let scene = Scene::by_id(e.scene).ok_or(format!("{}: no scene {}", e.name, e.scene))?;
-    Ok(Recording::synthetic(&scene, e.seed, e.first_tick, e.ticks, e.span_ms).encode())
+    match &e.kind {
+        Kind::Synthetic => {
+            Ok(Recording::synthetic(&scene, e.seed, e.first_tick, e.ticks, e.span_ms).encode())
+        }
+        Kind::Runner {
+            profile,
+            clients,
+            warmup_s,
+            measured_s,
+        } => crate::scenario::record(
+            &scene,
+            profile,
+            e.seed,
+            *clients,
+            e.first_tick,
+            *warmup_s,
+            *measured_s,
+        )
+        .map_err(|m| format!("{}: {m}", e.name)),
+    }
+}
+
+/// The generator an entry's kind must name: `Recording::GENERATOR` for a synthetic one, 0 for a runner's.
+fn generator_of(k: &Kind) -> u32 {
+    match k {
+        Kind::Synthetic => Recording::GENERATOR,
+        Kind::Runner { .. } => 0,
+    }
 }
 
 /// Whether `bytes` are the entry's: its length and SHA-256.
@@ -284,6 +373,27 @@ pub(crate) fn check(e: &Entry, bytes: &[u8]) -> Result<(), String> {
             e.sha256
         ));
     }
+    // The bytes' own header against the entry (the advisor's U4r build ruling, point 5): a manifest field that
+    // disagrees with the bytes it pins fails by name.
+    let r = Recording::decode(bytes)
+        .map_err(|m| format!("{}: the bytes do not decode: {m}", e.name))?;
+    let held = [
+        (
+            "first_tick",
+            u64::from(r.first_tick),
+            u64::from(e.first_tick),
+        ),
+        ("span_ms", u64::from(r.span_ms), u64::from(e.span_ms)),
+        ("ticks", r.ticks.len() as u64, u64::from(e.ticks)),
+    ];
+    for (field, bytes_say, entry_says) in held {
+        if bytes_say != entry_says {
+            return Err(format!(
+                "{}: the recording's {field} is {bytes_say}, the manifest's {entry_says}",
+                e.name
+            ));
+        }
+    }
     Ok(())
 }
 
@@ -294,7 +404,7 @@ pub(crate) fn regenerate_all(
     mut keep: impl FnMut(&Entry, &[u8]) -> Result<(), String>,
 ) -> Result<Vec<String>, String> {
     for e in entries {
-        if e.generator != Recording::GENERATOR {
+        if e.kind == Kind::Synthetic && e.generator != Recording::GENERATOR {
             return Err(format!(
                 "{}: generator {}, not this build's Recording::GENERATOR {} (only its recordings are pinned)",
                 e.name,
@@ -302,6 +412,12 @@ pub(crate) fn regenerate_all(
                 Recording::GENERATOR
             ));
         }
+        if e.generator != generator_of(&e.kind) {
+            return Err(format!(
+                "{}: generator {}, not a runner recording's 0",
+                e.name, e.generator
+            ));
+        }
     }
     let mut lines = Vec::new();
     for e in entries {
@@ -1253,6 +1369,7 @@ mod tests {
         let bytes = Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, 2_000).encode();
         let e = Entry {
             name: "TC-s8".to_owned(),
+            kind: Kind::Synthetic,
             scene: Scene::targets().id(),
             seed: 8,
             first_tick: u32::MAX - 20,
@@ -1298,6 +1415,23 @@ mod tests {
             "{err}"
         );
         assert_eq!(none, 0);
+        // Point 5 (the advisor's U4r build ruling): the bytes' header against the entry, field by field.
+        for (field, set) in [
+            (
+                "first_tick",
+                (|e: &mut Entry| e.first_tick += 1) as fn(&mut Entry),
+            ),
+            ("span_ms", |e: &mut Entry| e.span_ms += 1),
+            ("ticks", |e: &mut Entry| e.ticks -= 1),
+        ] {
+            let mut off = e.clone();
+            set(&mut off);
+            let err = check(&off, &bytes).expect_err(field);
+            assert!(
+                err.starts_with(&format!("TC-s8: the recording's {field} is ")),
+                "{err}"
+            );
+        }
     }
 
     #[test]
@@ -1324,16 +1458,87 @@ mod tests {
         let text = include_str!("../recordings.json");
         let entries = manifest(text).unwrap();
         let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
+        // U4d's three synthetic recordings; U4r's two runner recordings are det-recordings.json's (the advisor's U4r
+        // build ruling, point 1).
         assert_eq!(names, ["SS-s1", "ES-s2", "TC-s3"]);
+        assert!(entries.iter().all(|e| e.ticks == 9_000
+            && e.kind == Kind::Synthetic
+            && e.generator == Recording::GENERATOR));
+        let det = manifest(include_str!("../det-recordings.json")).unwrap();
+        let names: Vec<&str> = det.iter().map(|e| e.name.as_str()).collect();
+        assert_eq!(names, ["SS-P2-s1", "ES-P3-s2"]);
         assert!(
-            entries
-                .iter()
-                .all(|e| e.generator == Recording::GENERATOR && e.ticks == 9_000)
+            det.iter()
+                .all(|e| matches!(e.kind, Kind::Runner { .. }) && e.generator == 0)
         );
+        let pre = manifest(include_str!("../precheck-recordings.json")).unwrap();
+        assert!(matches!(pre[0].kind, Kind::Runner { .. }) && pre[0].generator == 0);
+        assert_eq!(pre.len(), 1);
+        assert_eq!(pre[0].name, "SS-P0-s4");
         assert!(manifest("{\"recordings\": [{\"name\": \"x\"}]}").is_err());
         assert!(manifest("[]").is_err());
     }
 
+    /// One manifest entry's text, `name` and `ticks` given (a synthetic TC entry of 799,693 bytes otherwise).
+    fn one(name: &str, ticks: u64, kind: &str) -> String {
+        format!(
+            "{{\"recordings\": [{{\"name\": {name:?}, {kind} \"scene\": 4, \"seed\": 3, \"first_tick\": 0, \
+             \"ticks\": {ticks}, \"span_ms\": 2000, \"generator\": 2, \"bytes\": 799693, \"sha256\": \"00\"}}]}}"
+        )
+    }
+
+    /// r10, carry 82: the reader refuses a name outside `[A-Za-z0-9-]+` and a `ticks` above `bytes / 12` before it
+    /// generates anything (`manifest` runs before `regenerate_all` generates), and an entry with no known kind.
+    #[test]
+    fn r10_carry_82_the_manifest_refuses_a_bad_name_and_ticks_above_bytes_over_12() {
+        let k = "\"kind\": \"synthetic\",";
+        assert!(manifest(&one("TC-s3", 9_000, k)).is_ok(), "the clean entry");
+        for bad in ["../TC", "TC s3", "TC\nh x 1 2", "", "TC_s3", "TC.rec"] {
+            let e = manifest(&one(bad, 9_000, k)).expect_err(bad);
+            assert!(e.contains("is not one or more of"), "{bad:?}: {e}");
+        }
+        let at = 799_693 / 12;
+        assert!(
+            manifest(&one("TC-s3", at, k)).is_ok(),
+            "ticks at bytes / 12"
+        );
+        let e = manifest(&one("TC-s3", at + 1, k)).expect_err("ticks above bytes / 12");
+        assert!(e.contains("cannot fit"), "{e}");
+        assert!(manifest(&one("TC-s3", 9_000, "")).is_err(), "no kind");
+        assert!(
+            manifest(&one("TC-s3", 9_000, "\"kind\": \"script\",")).is_err(),
+            "an unknown kind"
+        );
+    }
+
+    /// Carry 82's dispatch: a runner entry names its settings and generator 0; a synthetic one generator 2.
+    #[test]
+    fn r10_carry_82_a_runner_entry_reads_its_settings_and_its_generator_is_0() {
+        let text = "{\"recordings\": [{\"name\": \"SS-P2-s1\", \"kind\": \"runner\", \"profile\": \"P2\", \
+                    \"clients\": 48, \"warmup_s\": 10, \"measured_s\": 290, \"scene\": 1, \"seed\": 1, \
+                    \"first_tick\": 0, \"ticks\": 9000, \"span_ms\": 2000, \"generator\": 0, \"bytes\": 999999, \
+                    \"sha256\": \"00\"}]}";
+        let e = manifest(text).expect("a runner entry");
+        assert_eq!(
+            e[0].kind,
+            Kind::Runner {
+                profile: "P2".to_owned(),
+                clients: 48,
+                warmup_s: 10,
+                measured_s: 290
+            }
+        );
+        let wrong = text.replace("\"generator\": 0", "\"generator\": 2");
+        let mut kept = 0;
+        let err = regenerate_all(&manifest(&wrong).expect("reads"), |_, _| {
+            kept += 1;
+            Ok(())
+        })
+        .expect_err("a runner entry of generator 2");
+        assert!(err.contains("not a runner recording's 0"), "{err}");
+        assert_eq!(kept, 0);
+    }
+
     #[test]
     fn h6_usage_errors_are_refused() {
         let base = "--manifest m --dir d --out o";
@@ -1439,6 +1644,110 @@ mod tests {
         );
     }
 
+    /// Carry 83 (d) (EA-9's one default, the legs'): `hash compare` with a bite and no `--on`, over two recordings,
+    /// arms only the first its lists hold. The hash-map bite's field is in a World's dump only once the bite is armed
+    /// and its tick reached (npt-sim's world.rs `write_state`), so `--digest-field`, which replays the first given
+    /// dump's recording to that dump's tick as `compare` arms it, finds the field on the first recording and not on the
+    /// second. A `compare` that armed every recording would find it on both, and read the second's clean dump as
+    /// another layout.
+    #[cfg(feature = "bites")]
+    #[test]
+    fn r10_carry_83_d_compare_with_a_bite_and_no_on_arms_only_the_first_recording_listed() {
+        const FIELD: &str = "bite[0].hash_map_order";
+        let dir = crate::scenario::tests::scratch("hash-83d");
+        std::fs::create_dir_all(&*dir).expect("the folder");
+        let d = dir.to_str().expect("a UTF-8 path").to_owned();
+        let path = |f: &str| format!("{d}/{f}");
+        let (first, first_bytes) = short();
+        let second_bytes =
+            Recording::synthetic(&Scene::targets(), 9, u32::MAX - 20, 60, 2_000).encode();
+        let second = Entry {
+            name: "TC-s9".to_owned(),
+            seed: 9,
+            bytes: second_bytes.len() as u64,
+            sha256: sha256_hex(&second_bytes),
+            ..first.clone()
+        };
+        // Both recordings in the folder, their manifest, and one clean list of both (ours and theirs alike).
+        let mut lines = String::new();
+        let mut entries = Vec::new();
+        for (e, bytes) in [(&first, &first_bytes), (&second, &second_bytes)] {
+            std::fs::write(rec_path(&d, &e.name), bytes).expect("the recording");
+            let mut rp = Replayer::new();
+            for w in le_words(bytes) {
+                rp.feed(w);
+            }
+            while let Step::Stepped(tick) = rp.step() {
+                let hash = rp.state_hash().expect("a hash after a step");
+                let h = TickHash {
+                    recording: e.name.clone(),
+                    tick,
+                    hash,
+                };
+                lines.push_str(&format!("{}\n", hash_line(&h)));
+            }
+            entries.push(format!(
+                "{{\"name\": \"{}\", \"kind\": \"synthetic\", \"scene\": {}, \"seed\": {}, \"first_tick\": {}, \
+                 \"ticks\": {}, \"span_ms\": {}, \"generator\": {}, \"bytes\": {}, \"sha256\": \"{}\"}}",
+                e.name, e.scene, e.seed, e.first_tick, e.ticks, e.span_ms, e.generator, e.bytes, e.sha256
+            ));
+        }
+        std::fs::write(
+            path("m.json"),
+            format!("{{\"recordings\": [{}]}}", entries.join(", ")),
+        )
+        .expect("the manifest");
+        std::fs::write(path("ours.lines"), &lines).expect("ours");
+        std::fs::write(path("theirs.lines"), &lines).expect("theirs");
+        let arm = Arm {
+            id: "PM-0004-21/hash-map".to_owned(),
+            tick: first.first_tick.wrapping_add(5),
+            on: None,
+        };
+        let at = first.first_tick.wrapping_add(10);
+        // The first recording's dump with the bite armed, the second's clean, each at that tick.
+        for (e, a, f) in [
+            (&first, Some(&arm), "first.dump"),
+            (&second, None, "second.dump"),
+        ] {
+            let own = replay_to(&d, e, at, a).expect("a replay");
+            std::fs::write(path(f), dump_text(&e.name, at, own.hash(), own.words()))
+                .expect("a dump");
+        }
+        let digest = |dump: &str| {
+            let (out, equal) = run(&argv(&format!(
+                "compare --ours {} --theirs {} --manifest {} --dir {d} --ours-dump {} --digest-field {FIELD} \
+                 --ours-name ours --theirs-name theirs --bite {} --arm {}",
+                path("ours.lines"),
+                path("theirs.lines"),
+                path("m.json"),
+                path(dump),
+                arm.id,
+                arm.tick
+            )))
+            .expect("compare runs");
+            assert!(equal, "the two lists are equal: {out:?}");
+            out.into_iter()
+                .find(|l| l.starts_with("ours digest "))
+                .expect("a digest line")
+        };
+        let armed = digest("first.dump");
+        let clean = digest("second.dump");
+        println!("83 (d): {armed}\n83 (d): {clean}");
+        let word = armed.split(' ').nth(2).unwrap_or("");
+        assert!(
+            word.len() == 16
+                && word.bytes().all(|b| b.is_ascii_hexdigit())
+                && armed.contains(&format!("of its dump at TC-s8 tick {at})")),
+            "the first recording listed is armed: {armed}"
+        );
+        assert_eq!(
+            clean,
+            format!("ours digest none ({FIELD} at words [] of its dump at TC-s9 tick {at})"),
+            "the second is not"
+        );
+    }
+
     #[test]
     fn a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ() {
         // RR-L3: the field's indexes are this build's own dump's, so a dump of another layout (P6's word-size plant
````

### npt-lab: main.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/main.rs .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/main.rs`:  1 file changed, 93 insertions(+), 2 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/main.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/main.rs
index aa02208..dcd2e4e 100644
--- a/spikes/netcode-proof/crates/lab/src/main.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/main.rs
@@ -13,9 +13,15 @@
 //! error. `probe`'s log header and summary line carry the whole fate log's SHA-256 and, since U4d2 (carry 66), the
 //! SHA-256 of v1's tag and the log's datagram entries, the hash U2's v1 logs had.
 
+mod contract;
 mod hash;
+mod logs;
 mod machine;
+mod mirror;
+mod precheck;
 mod probe;
+mod replay;
+mod scenario;
 
 use npt_l1::Profile;
 use npt_log::{Ended, Header, SCHEMA, Writer};
@@ -216,10 +222,95 @@ fn main() -> ExitCode {
             }
         };
     }
+    if cmd == "scenario" {
+        let s = match scenario::parse(rest) {
+            Ok(s) => s,
+            Err(e) => {
+                eprintln!("npt-lab scenario: {e}\nusage: {}", scenario::USAGE);
+                return ExitCode::from(2);
+            }
+        };
+        return match scenario::run(&s) {
+            Ok(o) => {
+                println!("{}", scenario::summary_line(&s, &o));
+                ExitCode::SUCCESS
+            }
+            Err(hash::Failure::Usage(e)) => {
+                eprintln!("npt-lab scenario: {e}\nusage: {}", scenario::USAGE);
+                ExitCode::from(2)
+            }
+            Err(hash::Failure::Failed(e)) => {
+                eprintln!("npt-lab scenario: {e}");
+                ExitCode::from(1)
+            }
+        };
+    }
+    if cmd == "replay" {
+        let s = match replay::parse(rest) {
+            Ok(s) => s,
+            Err(e) => {
+                eprintln!("npt-lab replay: {e}\nusage: {}", replay::USAGE);
+                return ExitCode::from(2);
+            }
+        };
+        return match replay::run(&s) {
+            Ok(o) => {
+                println!("{}", replay::summary_line(&s, &o));
+                ExitCode::SUCCESS
+            }
+            Err(hash::Failure::Usage(e)) => {
+                eprintln!("npt-lab replay: {e}\nusage: {}", replay::USAGE);
+                ExitCode::from(2)
+            }
+            Err(hash::Failure::Failed(e)) => {
+                eprintln!("npt-lab replay: {e}");
+                ExitCode::from(1)
+            }
+        };
+    }
+    if cmd == "precheck" {
+        return match precheck::main(rest) {
+            Ok(lines) => {
+                for l in lines {
+                    println!("{l}");
+                }
+                ExitCode::SUCCESS
+            }
+            Err((code, e)) => {
+                eprintln!("npt-lab precheck: {e}\nusage: {}", precheck::USAGE);
+                ExitCode::from(code)
+            }
+        };
+    }
+    if cmd == "contract" {
+        return match contract::main(rest) {
+            Ok(lines) => {
+                for l in lines {
+                    println!("{l}");
+                }
+                ExitCode::SUCCESS
+            }
+            Err((code, e)) => {
+                if code == 2 {
+                    eprintln!("npt-lab contract: {e}\nusage: {}", contract::USAGE);
+                } else if code == contract::UNREADABLE {
+                    // Not the contract's FAIL (the advisor's U4r build ruling 3, point 6 (3)).
+                    eprintln!("npt-lab contract: {e} (exit {code}: an input that cannot be read)");
+                } else {
+                    println!("{e}");
+                }
+                ExitCode::from(code)
+            }
+        };
+    }
     if cmd != "probe" {
         eprintln!(
-            "npt-lab: unknown command {cmd:?}\nusage: {USAGE}\n       {}",
-            hash::USAGE
+            "npt-lab: unknown command {cmd:?}\nusage: {USAGE}\n       {}\n       {}\n       {}\n       {}\n       {}",
+            hash::USAGE,
+            scenario::USAGE,
+            replay::USAGE,
+            precheck::USAGE,
+            contract::USAGE
         );
         return ExitCode::from(2);
     }
````

### npt-lab: lib.rs
_`git diff --no-index spikes/netcode-proof/crates/lab/src/lib.rs .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs`:  1 file changed, 2 insertions(+), 1 deletion(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/src/lib.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs
index dbfd0d5..8bc3479 100644
--- a/spikes/netcode-proof/crates/lab/src/lib.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs
@@ -5,7 +5,8 @@
 //! ticks); the x86-64 re-decode and replay tool for the Arm rule (`ReplayX86`); the native-against-Wasm hash comparer
 //! ([`HashCompare`], U4d2). Nothing depends on this crate. Built in units U2 and U4.
 
-/// One tick's entry in a leg's per-tick hash list: the recording (or run) it replays, the tick and the state hash.
+/// One tick's entry in a leg's per-tick hash list: the recording both legs replay (its manifest name), the tick and the
+/// state hash.
 #[derive(Clone, Debug, PartialEq, Eq)]
 pub struct TickHash {
     /// The recording both legs replay (the same name on both legs), as its manifest names it.
````

### npt-lab's synthetic recordings manifest (crates/lab/recordings.json)
_`git diff --no-index spikes/netcode-proof/crates/lab/recordings.json .gate/u4r-snap1/spikes/netcode-proof/crates/lab/recordings.json`:  1 file changed, 4 insertions(+), 4 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/lab/recordings.json b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/recordings.json
index 2ff8533..278db8c 100644
--- a/spikes/netcode-proof/crates/lab/recordings.json
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/recordings.json
@@ -1,8 +1,8 @@
 {
-  "about": "The recordings manifest (u4-build-plan.md §4.15; u4d2-brief.md 2.3; carry 52): the three synthetic recordings of D-2 (sim/tests/replay.rs:21-29), each regenerated by `npt-lab hash` with Recording::synthetic(scene, seed, first_tick, ticks, span_ms) under Recording::GENERATOR 2 (U4d-F, adb1bab) into the checkout's .gate/ (never committed) and checked against its byte length and SHA-256 here before any leg runs. An entry whose generator is not Recording::GENERATOR is refused before anything is generated. Scene ids: 1 SS, 2 ES, 3 CO, 4 TC (Scene::by_id).",
+  "about": "The recordings manifest (u4-build-plan.md §4.15; u4d2-brief.md 2.3; carries 52 and 82): each entry regenerated by `npt-lab hash` into the checkout's .gate/ (never committed) and checked against its byte length and SHA-256 here before any leg runs. Its `kind` says how: `synthetic`, Recording::synthetic(scene, seed, first_tick, ticks, span_ms) under Recording::GENERATOR 2 (U4d-F, adb1bab), the three of D-2 (sim/tests/replay.rs:21-29); `runner`, `npt-lab scenario --record` re-run at the entry's scene, seed, first tick, profile, clients, warmup_s and measured_s (the brawl on L1, generator 0: u4r-brief.md 2.1 (ii)). Before anything is generated, every entry is refused whose name is not one or more of A-Z, a-z, 0-9 and -, whose ticks exceed bytes / 12, or whose generator is not its kind's (synthetic: Recording::GENERATOR; runner: 0). Scene ids: 1 SS, 2 ES, 3 CO, 4 TC (Scene::by_id).",
   "recordings": [
-    {"name": "SS-s1", "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16694921, "sha256": "ab3cc8423ed5f373c2539a31d900c83dad30ffe909508dbff2f3032424001458"},
-    {"name": "ES-s2", "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16700849, "sha256": "e3257dbd2972c1caed67f53f5697985de24d7c77f7c74c0568f563963899e0c8"},
-    {"name": "TC-s3", "scene": 4, "seed": 3, "first_tick": 4294962796, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 799693, "sha256": "c018b6503adf5836767b498177aa0537532bbafdce2acbb8f7dccd9675adbfc5"}
+    {"name": "SS-s1", "kind": "synthetic", "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16694921, "sha256": "ab3cc8423ed5f373c2539a31d900c83dad30ffe909508dbff2f3032424001458"},
+    {"name": "ES-s2", "kind": "synthetic", "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16700849, "sha256": "e3257dbd2972c1caed67f53f5697985de24d7c77f7c74c0568f563963899e0c8"},
+    {"name": "TC-s3", "kind": "synthetic", "scene": 4, "seed": 3, "first_tick": 4294962796, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 799693, "sha256": "c018b6503adf5836767b498177aa0537532bbafdce2acbb8f7dccd9675adbfc5"}
   ]
 }
````

### npt-lab's runner recordings manifest for det-u4.mjs (crates/lab/det-recordings.json, new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/det-recordings.json`:  1 file changed, 7 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/det-recordings.json b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/det-recordings.json
new file mode 100644
index 0000000..e6cd904
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/det-recordings.json
@@ -0,0 +1,7 @@
+{
+  "about": "U4r's two runner recordings (u4-build-plan.md §4.15 :494-501; u4r-brief.md 2.1 (ii); carry 82's schema, recordings.json's), in a manifest of their own by the advisor's U4r build ruling, point 1 (advisor-u4r-build1, route (c)): ops/runs/det-u4.mjs runs check.mjs's `wasm` step's clean half on this file (regenerate and SHA-check, native leg 1 against leg 2, Wasm leg 1 against native leg 1), while check.mjs's step keeps recordings.json's three synthetic recordings. Each entry is regenerated by `npt-lab scenario --record` at its scene, seed, first tick, profile, clients, warmup_s and measured_s (the brawl on L1, generator 0), read by `npt-lab hash`'s same reader. Scene ids: 1 SS, 2 ES, 3 CO, 4 TC (Scene::by_id).",
+  "recordings": [
+    {"name": "SS-P2-s1", "kind": "runner", "profile": "P2", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16937652, "sha256": "5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3"},
+    {"name": "ES-P3-s2", "kind": "runner", "profile": "P3", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16929501, "sha256": "fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b"}
+  ]
+}
````

### npt-lab's pre-check recordings manifest (crates/lab/precheck-recordings.json, new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/lab/precheck-recordings.json`:  1 file changed, 6 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/precheck-recordings.json b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/precheck-recordings.json
new file mode 100644
index 0000000..9c4b6c5
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/precheck-recordings.json
@@ -0,0 +1,6 @@
+{
+  "about": "The -30 pre-check's recording (u4-build-plan.md §4.15 :500-501: SS P0 seed 4 serves the pre-check only; u4r-brief.md 2.1 (i)): a runner recording in recordings.json's schema (carry 82), read by `npt-lab hash`'s same reader and regenerated by the pre-check's script before it replays it: `npt-lab scenario --record` at these settings (the brawl on L1, 48 clients, 10 s warm-up and 290 s measured: 9,000 ticks), its byte length and SHA-256 checked here. check.mjs's `wasm` step never reads this file.",
+  "recordings": [
+    {"name": "SS-P0-s4", "kind": "runner", "profile": "P0", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 1, "seed": 4, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16950444, "sha256": "6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3"}
+  ]
+}
````

### npt-bots' library (crates/bots/src/lib.rs: `Behaviour`)
_`git diff --no-index spikes/netcode-proof/crates/bots/src/lib.rs .gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs`:  1 file changed, 239 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/crates/bots/src/lib.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs
index 9477f5f..0519f46 100644
--- a/spikes/netcode-proof/crates/bots/src/lib.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs
@@ -1,3 +1,242 @@
 //! npt-bots: the netcode proof test's headless clients (plan E12-E14): 30 inputs per second, the brawl, the
 //! target-course shooter with a page-like frame loop, the cheats, the hostile clients, and the load orchestrator's
 //! calibration and roll-out across the CAX21s. Built in units U4 and U6.
+//!
+//! U4r (u4-build-plan.md §5 U4r; u4r-brief.md 2.2) builds the library's [`Behaviour`], clock-injected and
+//! transport-free (U4-SERVER): what one client's page would give its endpoint, from a seed, the client's slot and what
+//! the host tells it, never from a clock or a socket. Two kinds: the brawl (the stress and estimate scenes' 48 soldiers
+//! fighting in 3 x 16, every one holding fire, so the scene's weapon interval sets the shot rate PM-0001-2 checks:
+//! §4.13) and the open-loop scripts (commands from the slot, the seed and the count of commands alone, standing in for
+//! the cheats' command patterns in U4r's contract test: U4R-Q4 (a)). The target-course shooter is U4h's, the cheats
+//! and the never-acker U4f's.
+//!
+//! **When a command is given** (u4r-brief.md 2.2; carry 74 as U4R-Q14 (b1) amended it): at most one per client step,
+//! and none until the endpoint holds a snapshot (before it the stamp would be tick 0), nor at a step whose newest
+//! snapshot tick did not move since the last command (a second command would be stamped for the tick the first holds,
+//! and the room counts it late: room/src/input.rs:107-113). A skipped tick runs option B's repeat. Each step first hands
+//! the endpoint what arrived (`input(None, received, now)`), then, only when the newest tick moved,
+//! `input(Some(command), &[], now)` (a call with nothing received only stamps). **The press rule** (U4R-Q14, A4): each
+//! press counter moves by at most one edge per command given, never per step (the room refuses more edges than the ticks
+//! a command covers: room/src/input.rs:91-99); a press wanted at a step without a command waits for the next command.
+//! **The view time** (2.2): a client without `Interp` (U4g's) shows its newest snapshot, so each command claims
+//! `view_tick` = the endpoint's newest tick and `view_fraction` = 0, which the room's ring holds at every lead of 1 or
+//! more.
+
+use npt_client::ClientEndpoint;
+use npt_link::Channel;
+use npt_sim::{Command, Scene};
+
+/// Move vectors 22.5 degrees apart round the circle, each inside the room's x² + y² ≤ 127² check: a soldier walking
+/// through them, one step every [`TURN_EVERY`] commands, walks a circle about 2 m across at the stress scene's speed.
+const MOVES: [[i8; 2]; 16] = [
+    [127, 0],
+    [117, 49],
+    [89, 89],
+    [49, 117],
+    [0, 127],
+    [-49, 117],
+    [-89, 89],
+    [-117, 49],
+    [-127, 0],
+    [-117, -49],
+    [-89, -89],
+    [-49, -117],
+    [0, -127],
+    [49, -117],
+    [89, -89],
+    [117, -49],
+];
+/// Commands between two turns of the move vector.
+const TURN_EVERY: u64 = 4;
+/// Commands a brawler keeps its target before it picks again (about 1 s at 30 commands a second).
+const RETARGET_EVERY: u64 = 30;
+/// How far above a target's feet the brawl aims, cm (inside a soldier's box).
+const AIM_ABOVE_FEET_CM: i32 = 100;
+/// Where an aim with no target in view goes: a seeded point within this many cm of the map's centre, in x and y.
+const BLIND_SPREAD_CM: u64 = 1_000;
+/// The clamp script's points lie this far out in x and y, cm: beyond any weapon's reach from the fight, inside the
+/// position grid's range (±32,767 cm).
+const FAR_CM: i32 = 32_000;
+
+/// What a behaviour does with each command it gives.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum Kind {
+    /// Fire held, a walk round a small circle, aimed at an enemy in view (picked by the seed), else at a seeded point.
+    Brawl,
+    /// Open loop: fire held and the fire press counter moved by one on every second command (scripted press edges).
+    Presses,
+    /// Open loop: fire held, aimed at a seeded point far beyond the weapon's reach (the sim clamps it: §4.2).
+    Clamp,
+    /// Open loop: fire held, the walk, aimed at a seeded point near the centre.
+    Steady,
+}
+
+/// One client's behaviour (the module text): the brawl or an open-loop script, seeded per slot.
+#[derive(Clone, Debug)]
+pub struct Behaviour {
+    kind: Kind,
+    team: Option<u8>,
+    teams: Vec<Option<u8>>,
+    rng: u64,
+    phase: u64,
+    /// The seeded point an aim with no target goes to.
+    blind: [i32; 3],
+    target: Option<u8>,
+    /// A command at every step after the first snapshot (D13's test-only setting): never a judged one.
+    every_step: bool,
+    presses: [u8; 4],
+    last_newest: Option<u32>,
+    given: u64,
+    skipped: u64,
+}
+
+/// splitmix64's step: the bots' one generator, seeded per slot (no clock, no hash map).
+fn next(state: &mut u64) -> u64 {
+    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
+    let mut z = *state;
+    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
+    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
+    z ^ (z >> 31)
+}
+
+/// A uniform integer in `[0, n)` from one draw, by multiply and shift.
+fn below(word: u64, n: u64) -> u64 {
+    ((u128::from(word) * u128::from(n)) >> 64) as u64
+}
+
+impl Behaviour {
+    fn new(kind: Kind, scene: &Scene, slot: u8, seed: u64) -> Behaviour {
+        let mut rng = seed ^ (u64::from(slot) << 56) ^ 0x6e70_742d_626f_7473;
+        let phase = below(next(&mut rng), MOVES.len() as u64);
+        let spread =
+            |r: &mut u64| below(next(r), 2 * BLIND_SPREAD_CM + 1) as i32 - BLIND_SPREAD_CM as i32;
+        let blind = match kind {
+            Kind::Clamp => {
+                let sx = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
+                let sy = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
+                [sx * FAR_CM, sy * FAR_CM, AIM_ABOVE_FEET_CM]
+            }
+            _ => [spread(&mut rng), spread(&mut rng), AIM_ABOVE_FEET_CM],
+        };
+        Behaviour {
+            kind,
+            team: scene.team(slot),
+            teams: (0..scene.slots()).map(|s| scene.team(s)).collect(),
+            rng,
+            phase,
+            blind,
+            target: None,
+            every_step: false,
+            presses: [0; 4],
+            last_newest: None,
+            given: 0,
+            skipped: 0,
+        }
+    }
+
+    /// The brawl of `slot` in `scene`, from the run's `seed`.
+    pub fn brawl(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
+        Behaviour::new(Kind::Brawl, scene, slot, seed)
+    }
+
+    /// The open-loop script named `script` (`presses`, `clamp` or `steady`) of `slot` in `scene`, from `seed`;
+    /// `None` for another name.
+    pub fn script(scene: &Scene, slot: u8, seed: u64, script: &str) -> Option<Behaviour> {
+        let kind = match script {
+            "presses" => Kind::Presses,
+            "clamp" => Kind::Clamp,
+            "steady" => Kind::Steady,
+            _ => return None,
+        };
+        Some(Behaviour::new(kind, scene, slot, seed))
+    }
+
+    /// The same behaviour giving a command at every step once the endpoint holds a snapshot, whether or not its newest
+    /// tick moved: D13's test-only setting (u4r-brief.md section 11), which shows the duplicate stamps U4R-Q14 (b)
+    /// counts; never a judged run's.
+    pub fn every_step(mut self) -> Behaviour {
+        self.every_step = true;
+        self
+    }
+
+    /// The commands given so far.
+    pub fn given(&self) -> u64 {
+        self.given
+    }
+
+    /// The steps since the endpoint's first snapshot at which no command was given (its newest tick had not moved).
+    pub fn skipped(&self) -> u64 {
+        self.skipped
+    }
+
+    /// One client step at `now_ns` on the host's clock (u4r-brief.md 2.2; the module text): `received` goes to the
+    /// endpoint first, then a command if one is due. `view` is what the host says this client sees at its newest tick:
+    /// (slot, position in the position grid's cm) of other soldiers; the open-loop scripts never read it. Returns
+    /// whether a command was given.
+    pub fn step(
+        &mut self,
+        endpoint: &mut ClientEndpoint,
+        received: &[(Channel, Vec<u8>, u64)],
+        now_ns: u64,
+        view: &[(u8, [i32; 3])],
+    ) -> bool {
+        endpoint.input(None, received, now_ns);
+        let (Some(newest), Some(_)) = (endpoint.newest_tick(), endpoint.slot()) else {
+            return false;
+        };
+        if !self.every_step && self.last_newest == Some(newest) {
+            self.skipped += 1;
+            return false;
+        }
+        let command = self.command(newest, view);
+        endpoint.input(Some(command), &[], now_ns);
+        self.last_newest = Some(newest);
+        self.given += 1;
+        true
+    }
+
+    /// The next command (its slot, tick and sequence are the endpoint's to stamp), claiming `newest` as its view.
+    fn command(&mut self, newest: u32, view: &[(u8, [i32; 3])]) -> Command {
+        let k = self.given;
+        if self.kind == Kind::Presses && k % 2 == 1 {
+            self.presses[Command::PRESS_FIRE] = self.presses[Command::PRESS_FIRE].wrapping_add(1);
+        }
+        let aim = match self.kind {
+            Kind::Brawl => self.brawl_aim(k, view),
+            _ => self.blind,
+        };
+        let walk = MOVES[((k / TURN_EVERY + self.phase) % MOVES.len() as u64) as usize];
+        Command {
+            buttons: Command::FIRE,
+            presses: self.presses,
+            move_xy: if self.kind == Kind::Clamp {
+                [0, 0]
+            } else {
+                walk
+            },
+            aim,
+            view_tick: newest,
+            view_fraction: 0,
+            ..Command::default()
+        }
+    }
+
+    /// The brawl's aim: its target's position in view, raised into its box, re-picked by the seed every
+    /// [`RETARGET_EVERY`] commands or when the target is out of view; else the seeded blind point.
+    fn brawl_aim(&mut self, k: u64, view: &[(u8, [i32; 3])]) -> [i32; 3] {
+        let enemy = |s: &u8| {
+            let team = self.teams.get(usize::from(*s)).copied().flatten();
+            team.is_some() && team != self.team
+        };
+        let in_view = |t: u8| view.iter().any(|(s, _)| *s == t);
+        if k.is_multiple_of(RETARGET_EVERY) || !self.target.is_some_and(in_view) {
+            let enemies: Vec<u8> = view.iter().map(|(s, _)| *s).filter(enemy).collect();
+            self.target = (!enemies.is_empty())
+                .then(|| enemies[below(next(&mut self.rng), enemies.len() as u64) as usize]);
+        }
+        match self.target.and_then(|t| view.iter().find(|(s, _)| *s == t)) {
+            Some((_, p)) => [p[0], p[1], p[2].saturating_add(AIM_ABOVE_FEET_CM)],
+            None => self.blind,
+        }
+    }
+}
````

### npt-bots' clippy.toml (crates/bots/clippy.toml, new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/bots/clippy.toml`:  1 file changed, 17 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/clippy.toml b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/clippy.toml
new file mode 100644
index 0000000..30e689a
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/clippy.toml
@@ -0,0 +1,17 @@
+# npt-bots' clock and hash-map bans (u4-build-plan.md §4.11; the advisor's U4d2 brief check, A13): the bots are
+# clock-injected and transport-free (U4-SERVER), so their commands follow from what they are given and a seed alone,
+# and they take no order from a hash map. The clock ban is npt-server's, as its clippy.toml words it; the two type bans
+# are npt-sim's. Crate-wide; U6's runtime half adds its reviewed allows (§4.11). The workspace sets disallowed_types and
+# disallowed_methods to deny.
+disallowed-types = [
+    { path = "std::collections::HashMap", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::collections::HashSet", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::time::Instant", reason = "the clock ban: time is passed in" },
+    { path = "std::time::SystemTime", reason = "the clock ban: time is passed in" },
+    { path = "tokio::time::Instant", reason = "the clock ban: time is passed in", allow-invalid = true },
+]
+disallowed-methods = [
+    { path = "std::time::Instant::now", reason = "the clock ban" },
+    { path = "std::time::SystemTime::now", reason = "the clock ban" },
+    { path = "tokio::time::Instant::now", reason = "the clock ban", allow-invalid = true },
+]
````

### npt-log (crates/log/src/lib.rs)
_`git diff --no-index spikes/netcode-proof/crates/log/src/lib.rs .gate/u4r-snap1/spikes/netcode-proof/crates/log/src/lib.rs`:  1 file changed, 179 insertions(+), 3 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/log/src/lib.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/log/src/lib.rs
index 11d395d..c4565b9 100644
--- a/spikes/netcode-proof/crates/log/src/lib.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/log/src/lib.rs
@@ -302,7 +302,114 @@ fn check_names<'a>(names: impl Iterator<Item = &'a String>) -> io::Result<()> {
     Ok(())
 }
 
+/// The first text in `v` (a string or an object key, at any depth) holding a character for which `char::is_control`
+/// holds, as (its path, the character): carry 63, FU-7.
+fn control_char(v: &serde_json::Value, path: &str) -> Option<(String, char)> {
+    match v {
+        serde_json::Value::String(s) => s
+            .chars()
+            .find(|c| c.is_control())
+            .map(|c| (path.to_owned(), c)),
+        serde_json::Value::Array(a) => a
+            .iter()
+            .enumerate()
+            .find_map(|(i, x)| control_char(x, &format!("{path}[{i}]"))),
+        serde_json::Value::Object(o) => o.iter().find_map(|(k, x)| {
+            let p = if path.is_empty() {
+                k.clone()
+            } else {
+                format!("{path}.{k}")
+            };
+            k.chars()
+                .find(|c| c.is_control())
+                .map(|c| (p.clone(), c))
+                .or_else(|| control_char(x, &p))
+        }),
+        _ => None,
+    }
+}
+
+/// A JSON value walked only to find an object that holds one key twice (carry 63, FU-3): serde_json's own reading
+/// keeps the last one silently.
+struct NoDuplicateKey;
+
+impl<'de> serde::de::DeserializeSeed<'de> for NoDuplicateKey {
+    type Value = ();
+
+    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
+        d.deserialize_any(self)
+    }
+}
+
+impl<'de> serde::de::Visitor<'de> for NoDuplicateKey {
+    type Value = ();
+
+    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        f.write_str("a JSON value")
+    }
+
+    fn visit_bool<E>(self, _: bool) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_i64<E>(self, _: i64) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_u64<E>(self, _: u64) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_f64<E>(self, _: f64) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_str<E>(self, _: &str) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_unit<E>(self) -> Result<(), E> {
+        Ok(())
+    }
+
+    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
+        while seq.next_element_seed(NoDuplicateKey)?.is_some() {}
+        Ok(())
+    }
+
+    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
+        let mut seen = std::collections::BTreeSet::new();
+        while let Some(key) = map.next_key::<String>()? {
+            if seen.contains(&key) {
+                return Err(serde::de::Error::custom(format!(
+                    "an object holds the key {key:?} twice"
+                )));
+            }
+            map.next_value_seed(NoDuplicateKey)?;
+            seen.insert(key);
+        }
+        Ok(())
+    }
+}
+
+/// Refuses a JSON text in which any object holds one key twice (carry 63, FU-3).
+fn no_duplicate_key(text: &str) -> Result<(), String> {
+    use serde::de::DeserializeSeed;
+    let mut de = serde_json::Deserializer::from_str(text);
+    NoDuplicateKey
+        .deserialize(&mut de)
+        .and_then(|()| de.end())
+        .map_err(|e| format!("not one JSON record: {e}"))
+}
+
 fn check_header(h: &Header) -> Result<(), String> {
+    let v = serde_json::to_value(h).map_err(|e| format!("header: {e}"))?;
+    if let Some((field, c)) = control_char(&v, "") {
+        return Err(format!(
+            "header field {field:?} holds the control character U+{:04X}",
+            u32::from(c)
+        ));
+    }
     if h.schema != SCHEMA {
         return Err(format!(
             "header schema {} (this build reads {SCHEMA})",
@@ -448,9 +555,10 @@ impl fmt::Display for ReadError {
 
 impl std::error::Error for ReadError {}
 
-/// Reads one log and checks its discipline: the header first and only once, known record kinds and fields only,
-/// ticks strictly increasing, seconds never going back, the footer last with the right record count. A file with no
-/// footer reads as cut short (`footer` is `None`); the judge's `complete` rule decides what that means.
+/// Reads one log and checks its discipline: the header first and only once, known record kinds and fields only, no
+/// JSON object holding one key twice (carry 63), no control character in the header's text (carry 63), ticks strictly
+/// increasing, seconds never going back, the footer last with the right record count. A file with no footer reads as
+/// cut short (`footer` is `None`); the judge's `complete` rule decides what that means.
 pub fn read_log<R: BufRead>(input: R) -> Result<Log, ReadError> {
     let mut header: Option<Header> = None;
     let mut log_ticks = Vec::new();
@@ -467,6 +575,7 @@ pub fn read_log<R: BufRead>(input: R) -> Result<Log, ReadError> {
         if footer.is_some() {
             return Err(err("a record after the footer".to_owned()));
         }
+        no_duplicate_key(&text).map_err(err)?;
         let mut value: serde_json::Value =
             serde_json::from_str(&text).map_err(|e| err(format!("not one JSON record: {e}")))?;
         let kind = match value.as_object_mut().and_then(|o| o.remove("k")) {
@@ -860,4 +969,71 @@ mod tests {
         let e = read_log(back.as_bytes()).expect_err("must fail");
         assert_eq!(e.line, 3);
     }
+
+    /// Carry 63, FU-3: a JSON object holding one key twice is refused on read, at any depth (serde_json alone keeps
+    /// the last one, silently).
+    #[test]
+    fn fu3_a_json_object_with_a_duplicate_key_is_refused_on_read() {
+        let text = String::from_utf8(sample_log()).expect("utf-8");
+        let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
+        assert!(read_log(text.as_bytes()).is_ok(), "the clean log reads");
+        // A tick record whose `v` holds one name twice: the first value would be lost.
+        lines[1] = lines[1].replacen("\"work_ns\":", "\"work_ns\":1,\"work_ns\":", 1);
+        let doubled = lines.join("\n") + "\n";
+        let e = read_log(doubled.as_bytes()).expect_err("a duplicate key in a tick's v");
+        assert_eq!(e.line, 2);
+        assert!(e.msg.contains("work_ns"), "{}", e.msg);
+        // A duplicate at the record's top level, and one inside the header's params.
+        let top = format!("{}\n", text.lines().next().expect("header")).replacen(
+            "\"run\":",
+            "\"run\":\"R0-other\",\"run\":",
+            1,
+        );
+        let e = read_log(top.as_bytes()).expect_err("a duplicate key in the header");
+        assert_eq!(e.line, 1);
+        let nested = format!("{}\n", text.lines().next().expect("header")).replacen(
+            "\"sim\":",
+            "\"sim\":\"x\",\"sim\":",
+            1,
+        );
+        assert!(
+            read_log(nested.as_bytes()).is_err(),
+            "a duplicate key in params"
+        );
+    }
+
+    /// Carry 63, FU-7: a header text field holding a character for which `char::is_control` holds is refused, on write
+    /// and on read (today only an empty `run`, `set`, `role` or `process` is).
+    #[test]
+    fn fu7_a_header_text_field_with_a_control_character_is_refused() {
+        let cases: [fn(&mut Header); 6] = [
+            |h| h.run = "R0\u{7}".to_owned(),
+            |h| h.scene = "S\nS".to_owned(),
+            |h| h.marks = vec!["PM-0001-4\u{1b}[2J".to_owned()],
+            |h| h.machine.cpu = "cpu\r".to_owned(),
+            |h| h.build.features = vec!["bites\u{0}".to_owned()],
+            |h| {
+                h.params
+                    .insert("note".to_owned(), serde_json::json!(["a", "b\u{85}"]));
+            },
+        ];
+        for (i, case) in cases.iter().enumerate() {
+            let mut h = header();
+            case(&mut h);
+            assert!(
+                Writer::new(Vec::new(), &h).is_err(),
+                "case {i}: the writer refuses the header"
+            );
+            let line = format!(
+                "{}\n",
+                serde_json::to_string(&Line::Header(&h)).expect("serializes")
+            );
+            let e = read_log(line.as_bytes()).expect_err("the reader refuses the header");
+            assert!(e.msg.contains("control"), "case {i}: {}", e.msg);
+        }
+        assert!(
+            Writer::new(Vec::new(), &header()).is_ok(),
+            "the clean header writes"
+        );
+    }
 }
````

### npt-server: pump.rs
_`git diff --no-index spikes/netcode-proof/crates/server/src/pump.rs .gate/u4r-snap1/spikes/netcode-proof/crates/server/src/pump.rs`:  1 file changed, 43 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/crates/server/src/pump.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/src/pump.rs
index f558d7a..04fb86f 100644
--- a/spikes/netcode-proof/crates/server/src/pump.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/src/pump.rs
@@ -214,6 +214,35 @@ pub struct Decoder {
     outstanding: VecDeque<(u8, u64)>,
     samples: VecDeque<u64>,
     sent_bytes: u32,
+    /// PM-0001-30/wallclock planted on this session (bites builds only; U4R-Q6 (a)).
+    #[cfg(feature = "bites")]
+    wallclock: bool,
+}
+
+/// PM-0001-30/wallclock (marks.json:6563: "a wall-clock read on one transport's command path"; U4R-Q6 (a)): the decoder
+/// of one -30 pre-check leg's sessions reads the wall clock and changes each passed-on command's aim by it, so that
+/// leg's room runs other commands than the other leg's on every run, whatever the machine's ticks per second.
+#[cfg(feature = "bites")]
+pub(crate) const WALLCLOCK_BITE: &str = "PM-0001-30/wallclock";
+
+/// The wallclock bite's site (bites builds only; U4R-Q6 (a)'s field rule): every command of `i` gets its aim's x XORed
+/// with `1 | (wall-clock ns & 0xfe)`, never 0, so the value is never the received one and moves at most 255 cm (inside
+/// the aim's range); a field the World takes at that tick, never the tick, sequence, slot, buttons, presses or view time.
+#[cfg(feature = "bites")]
+#[allow(
+    clippy::disallowed_methods,
+    clippy::disallowed_types,
+    reason = "the planted defect is a wall-clock read (PM-0001-30/wallclock): bites builds only"
+)]
+fn wallclock_aims(mut i: npt_proto::Input) -> npt_proto::Input {
+    let ns = std::time::SystemTime::now()
+        .duration_since(std::time::UNIX_EPOCH)
+        .map_or(0, |d| d.subsec_nanos());
+    let k = 1 | (ns & 0xfe) as i32;
+    for c in &mut i.commands {
+        c.aim[0] ^= k;
+    }
+    i
 }
 
 impl Decoder {
@@ -242,9 +271,21 @@ impl Decoder {
             outstanding: VecDeque::new(),
             samples: VecDeque::new(),
             sent_bytes: 0,
+            #[cfg(feature = "bites")]
+            wallclock: false,
         }
     }
 
+    /// The same decoder with one planted defect by its id (bites builds only; per instance, U4R-Q6 (a)):
+    /// PM-0001-30/wallclock; `None` for another id (the schedule's are `Schedule::with_bite`'s).
+    #[cfg(feature = "bites")]
+    pub fn with_bite(mut self, bite: &str) -> Option<Decoder> {
+        (bite == WALLCLOCK_BITE).then(|| {
+            self.wallclock = true;
+            self
+        })
+    }
+
     /// The session's counts.
     pub fn counters(&self) -> &Counters {
         &self.counters
@@ -407,6 +448,8 @@ impl Decoder {
                 (_, Message::Join(_)) => self.counters.join_again += 1,
                 (State::Joined, Message::Input(i)) => {
                     if self.limits.input.take(now_ns) {
+                        #[cfg(feature = "bites")]
+                        let i = if self.wallclock { wallclock_aims(i) } else { i };
                         out.push((Message::Input(i), r.arrival_ns));
                     } else {
                         self.counters.input_limited += 1;
````

### npt-server: schedule.rs
_`git diff --no-index spikes/netcode-proof/crates/server/src/schedule.rs .gate/u4r-snap1/spikes/netcode-proof/crates/server/src/schedule.rs`:  1 file changed, 9 insertions(+), 3 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/server/src/schedule.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/src/schedule.rs
index e5587d3..04424bb 100644
--- a/spikes/netcode-proof/crates/server/src/schedule.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/src/schedule.rs
@@ -245,10 +245,16 @@ const BITE_IDS: [(&str, Bite); 3] = [
     ("PM-0004-67/uncounted-drop", Bite::UncountedDrop),
 ];
 
-/// The bite ids this module plants (bites builds only): PM-0004-67's three, whose site is `due_ticks` (u4-build-plan.md
-/// §5 U4b; §4.14 plants each bite where its code lives).
+/// The bite ids npt-server plants (bites builds only): PM-0004-67's three, whose site is `due_ticks` (u4-build-plan.md
+/// §5 U4b; §4.14 plants each bite where its code lives), and PM-0001-30/wallclock, whose site is the pump's command
+/// path (`Decoder::with_bite`; U4R-Q6 (a)).
 #[cfg(feature = "bites")]
-pub const BITES: [&str; 3] = [BITE_IDS[0].0, BITE_IDS[1].0, BITE_IDS[2].0];
+pub const BITES: [&str; 4] = [
+    BITE_IDS[0].0,
+    BITE_IDS[1].0,
+    BITE_IDS[2].0,
+    crate::pump::WALLCLOCK_BITE,
+];
 
 #[cfg(feature = "bites")]
 impl Schedule {
````

### npt-hash-wasm's loader (crates/hash-wasm/loader.mjs)
_`git diff --no-index spikes/netcode-proof/crates/hash-wasm/loader.mjs .gate/u4r-snap1/spikes/netcode-proof/crates/hash-wasm/loader.mjs`:  1 file changed, 4 insertions(+), 2 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/crates/hash-wasm/loader.mjs b/.gate/u4r-snap1/spikes/netcode-proof/crates/hash-wasm/loader.mjs
index 972fd86..3e388c8 100644
--- a/spikes/netcode-proof/crates/hash-wasm/loader.mjs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/hash-wasm/loader.mjs
@@ -10,8 +10,10 @@
 //   so no host clock or random source can reach the Wasm leg (the advisor's U4d2 round-1 ruling, FX-4: an import
 //   object `{}` would have given a module importing `constructor.constructor` JS's Function); and a target_features
 //   section that lists simd128 or relaxed-simd as used or required, or none, or more than one, or one that does not
-//   parse (FX-2: plan.md §4.5's "simd128 and relaxed-simd off", read from the module itself, so a source-level
-//   `#[target_feature]` or a flag reaching rustc by a route check.mjs does not read is refused too). It instantiates
+//   parse (FX-2: plan.md §4.5's "simd128 and relaxed-simd off", read from the module itself: a feature enabled at
+//   compile time, by a source-level `#[target_feature]` or a flag from a route check.mjs does not read, shows in that
+//   section and is refused; but the section is the linker's record, so link arguments from a RUSTC_WRAPPER, allowed for
+//   sccache, can rewrite it, and the x86-64 legs have no such check: carry 85 (a)). It instantiates
 //   with `Object.create(null)`, an import object with nothing in it, not even Object.prototype's names.
 // - It streams the recording in: the file's bytes as little-endian 32-bit words, the last zero-padded, fed in chunks
 //   of 64 KiB with `step` called until NEED_MORE between them, never the whole file before the first step.
````

### check.mjs
_`git diff --no-index spikes/netcode-proof/check.mjs .gate/u4r-snap1/spikes/netcode-proof/check.mjs`:  1 file changed, 154 insertions(+), 38 deletions(-)._

````diff
diff --git a/spikes/netcode-proof/check.mjs b/.gate/u4r-snap1/spikes/netcode-proof/check.mjs
index c015186..ecbec86 100644
--- a/spikes/netcode-proof/check.mjs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/check.mjs
@@ -680,14 +680,22 @@ export function unsafeProblems(files, read) {
       if (EXCEPTION_LEVELS[crate] === "deny") {
         const name = UNSAFE_EXCEPTIONS[crate];
         if (f === `${crate}build.rs`) problems.push(`${f} is a build script in ${name}, which may have none: ${DENY_EXCEPTION_WHY}`);
-        for (const m of mask.matchAll(/\binclude\b(?!\s*!)/g)) {
+        // Carry 85 (c): `include` only directly before `!` and an opening bracket, so neither a rename nor a macro
+        // given the bare name (`m!(include!)`) can build a call the include! rule does not see.
+        for (const m of mask.matchAll(/\binclude\b(?!![([{])/g)) {
           problems.push(
-            `${f}:${lineAt(mask, m.index)} names \`include\` other than as an include!, include_str! or include_bytes! call ` +
-              `(a renamed include!, as \`use std::include as inc;\`, hides the call from the include! rule), which ${name} may not: ` +
+            `${f}:${lineAt(mask, m.index)} names \`include\` other than directly before \`!\` and an opening bracket, as an ` +
+              `include!, include_str! or include_bytes! call is written (a renamed include!, as \`use std::include as inc;\`, or ` +
+              `one a macro builds, as \`m!(include!)\`, hides the call from the include! rule), which ${name} may not: ` +
               DENY_EXCEPTION_WHY,
           );
         }
       }
+      // Carry 85 (c): no attribute in any crate whose text starts with a metavariable, so a #[path] (or an allow) built
+      // from a macro's `meta` fragment cannot bring an unread file under a listed export's allow.
+      for (const m of mask.matchAll(/#\s*!?\s*\[\s*\$/g)) {
+        problems.push(`${f}:${lineAt(mask, m.index)} has an attribute whose text starts with a metavariable (\`#[$\` or \`#![$\`): a macro could build a #[path] from a \`meta\` fragment that this scan does not read`);
+      }
       problems.push(...pathProblems(f, crate, code, mask, attrs));
     } else if (f === `${crate}Cargo.toml` && EXCEPTION_LEVELS[crate] === "deny") {
       if (tomlValues(tomlScan(text), "build").length) {
@@ -1425,6 +1433,13 @@ const UNSTABLE_ENV = {
  *   the ones cargoConfigFiles finds); a rustflags value this check cannot read;
  * - an alias that replaces clippy or fmt (CARGO_ALIAS_CLIPPY or _FMT, or such a key in a config file);
  * - another compiler or a workspace wrapper (COMPILER_ENV, COMPILER_KEYS);
+ * - a linker set for any target (carry 85 (a)), as the compiler is: an environment variable whose name ends in _LINKER
+ *   (CARGO_TARGET_<triple>_LINKER, CARGO_HOST_LINKER) or a `linker` key in any table of a config file: a linker's own
+ *   arguments can rewrite the Wasm module's target_features section, the linker's record that the loader reads;
+ * - a profile set outside the manifests (carry 85 (b)): an environment variable whose name starts with CARGO_PROFILE_,
+ *   or a `profile` key, dotted key or table in a config file (any table or form, which fails closed): a config's
+ *   `[profile.release.package.npt-sim] overflow-checks = false` drops npt-sim's `-C overflow-checks=on`, which plan §4.5
+ *   asks for in every profile;
  * - a config `include` key or `[[include]]` table, its name quoted or not, which loads a config file cargoConfigFiles
  *   does not read (cargo 1.99.1 honours it);
  * - a target runner (CARGO_TARGET_<triple>_RUNNER, or a `runner` key in a config file): it runs the test binaries in
@@ -1452,6 +1467,8 @@ export function cargoConfigProblems(env, configs) {
     const alias = k.match(/^CARGO_ALIAS_(\w+)$/)?.[1].toLowerCase();
     if (SHADOWABLE.includes(alias)) problems.push(`${k} in the environment replaces \`cargo ${alias}\` with an alias`);
     if (COMPILER_ENV.includes(k)) problems.push(`${k} in the environment replaces rustc or wraps the workspace's rustc calls (only RUSTC_WRAPPER may wrap them)`);
+    if (/_LINKER$/.test(k)) problems.push(`${k} in the environment sets a linker (${JSON.stringify(v)}): its arguments can rewrite the target_features section the Wasm loader reads`);
+    if (/^CARGO_PROFILE_/.test(k)) problems.push(`${k} in the environment sets a profile key (${JSON.stringify(v)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
     if (/^CARGO_TARGET_\w+_RUNNER$/.test(k)) problems.push(`${k} in the environment runs the test binaries in cargo's place`);
     if (Object.hasOwn(UNSTABLE_ENV, k)) problems.push(`${k} in the environment ${UNSTABLE_ENV[k]}`);
   }
@@ -1467,6 +1484,15 @@ export function cargoConfigProblems(env, configs) {
     for (const name of COMPILER_KEYS) {
       if (tomlValues(scan, name).length) problems.push(`${path} has a key \`${name}\`: it replaces rustc or wraps the workspace's rustc calls`);
     }
+    for (const v of tomlValues(scan, "linker")) {
+      problems.push(`${path} has a key \`linker\` (${v === null ? "a value this check cannot read" : JSON.stringify(v.join(" "))}): it sets a linker, whose arguments can rewrite the target_features section the Wasm loader reads`);
+    }
+    // A key, dotted key or table header named `profile`, bare or quoted (as tomlValues reads keys, and `.` too).
+    const profile =
+      tomlHeaderHeads(scan).includes("profile") ||
+      /(?:^|[\s.{,])profile[ \t]*[.=]/m.test(scan.mask) ||
+      scan.strings.some((s) => s.value === "profile" && /^[ \t]*[.=]/.test(scan.mask.slice(s.end)) && /(?:^|[\s.{,[])$/.test(scan.mask.slice(0, s.start)));
+    if (profile) problems.push(`${path} has a \`profile\` key or table: it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
     if (tomlValues(scan, "include").length || tomlHeaderHeads(scan).includes("include")) {
       problems.push(`${path} has a key \`include\`: it loads a config file this check does not read`);
     }
@@ -1545,9 +1571,10 @@ export function buildFlagProblems(sources) {
  * `--target` (the advisor's U4d2 round-1 ruling, FX-1: RR-M2 with TE-M1, SEC-U4D2-5, SEC-U4D2-6), as sentences naming
  * the source and its value:
  * - each source rustflagSources reads (every environment variable whose name ends in RUSTFLAGS, and every `rustflags`
- *   key of the cargo configs read) with a flag in it. Cargo gives rustc the first non-empty one of
- *   CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, the target tables and build.rustflags and drops the others
- *   (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags), so a read-back given all of them could
+ *   key of the cargo configs read) with a flag in it. Cargo gives rustc one source and drops the others
+ *   (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags; carry 85 (e)): CARGO_ENCODED_RUSTFLAGS if
+ *   set, even empty; else RUSTFLAGS if set, even empty; else the matching target tables' flags, if any; else
+ *   build.rustflags. So a read-back given all of them could
  *   take a flag's negation from a source cargo drops (`-Ctarget-feature=-simd128,-relaxed-simd` in RUSTFLAGS beside
  *   CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge passed while the module cargo built held both: RR-M2), and a
  *   flag no name rule knows (`--cfg feature="bites"`, `-C overflow-checks=off`, `-C llvm-args=...`) reached rustc unseen
@@ -1759,16 +1786,26 @@ export function manifestKeyProblems(manifests) {
 }
 
 /**
- * The cargo config files cargo reads when it runs in `start`: .cargo/config.toml and .cargo/config in `start` and in
- * every folder above it, then in CARGO_HOME (`env`'s, default ~/.cargo); each that exists, as { path, text }.
+ * The folders cargo reads its config files from when it runs in `start`: .cargo in `start` and in every folder above
+ * it, then CARGO_HOME (`env`'s) as cargo reads it (carry 85 (d)): an empty one as unset (~/.cargo), a relative one
+ * against `start`, the folder cargo runs in (cargo's home crate: CARGO_HOME non-empty, joined to the working folder).
  */
-function cargoConfigFiles(start, env) {
+export function cargoConfigDirs(start, env) {
   const dirs = [];
   for (let d = resolve(start); ; d = dirname(d)) {
     dirs.push(join(d, ".cargo"));
     if (dirname(d) === d) break;
   }
-  dirs.push(env.CARGO_HOME ?? join(homedir(), ".cargo"));
+  dirs.push(env.CARGO_HOME ? resolve(start, env.CARGO_HOME) : join(homedir(), ".cargo"));
+  return dirs;
+}
+
+/**
+ * The cargo config files cargo reads when it runs in `start`: .cargo/config.toml and .cargo/config in each of
+ * cargoConfigDirs; each that exists, as { path, text }.
+ */
+function cargoConfigFiles(start, env) {
+  const dirs = cargoConfigDirs(start, env);
   const found = [];
   for (const dir of new Set(dirs)) {
     for (const name of ["config.toml", "config"]) {
@@ -1846,13 +1883,27 @@ export function marksProblems(marks, readFile) {
 
 /**
  * One command, waited for: `timeout` (ms; 0, none) stops it with SIGTERM once passed, and the result says so
- * (`timedOut`: spawnSync's ETIMEDOUT), so a hung call is a named failure, not a lock held forever (INF-4).
- */
-function run(cmd, args, cwd, env = process.env, timeout = 0) {
-  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout } : {}) });
+ * (`timedOut`: spawnSync's ETIMEDOUT), so a hung call is a named failure, not a lock held forever (INF-4). A call with a
+ * timeout runs in its own process group (spawnSync's `detached`: the child calls setsid), and once it is stopped that
+ * whole group is killed (SIGKILL to the group), so every process it started goes with it (carry 84 (b): spawnSync's
+ * timeout signals the direct child alone, so `npt-lab hash`'s two leg processes kept running). Limits: a process that
+ * leaves the group by its own setsid is not reached; and since the group is its own session, a signal that stops
+ * check.mjs while such a call runs (Ctrl-C) does not reach the call: end it by hand (`pgrep -a npt-lab`; `pgrep -af
+ * loader.mjs`). check.test.mjs runs it on real processes (carry 84 (c)).
+ */
+export function run(cmd, args, cwd, env = process.env, timeout = 0) {
+  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout, detached: true } : {}) });
+  const timedOut = r.error?.code === "ETIMEDOUT";
+  if (timedOut && r.pid) {
+    try {
+      process.kill(-r.pid, "SIGKILL");
+    } catch {
+      // ESRCH: no process of the group is left.
+    }
+  }
   // `stdout` alone is kept for the steps that parse it: cargo writes progress lines ("Blocking waiting for file
   // lock", "Downloaded …") to stderr, and they must never reach a JSON parser.
-  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status, timedOut: r.error?.code === "ETIMEDOUT" };
+  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status, timedOut };
 }
 
 /**
@@ -1998,8 +2049,11 @@ export function testBothBuilds(runner = run) {
  * - The build-flag scan and rustc's read-back read the rustflags sources above, not a build script's
  *   `cargo::rustc-flags` or a RUSTC_WRAPPER's own flags. No source may hold a flag (buildInputProblems: the advisor's
  *   U4d2 round-1 ruling, FX-1), so on a step that passes the read-back is given no flag, which is what cargo gives
- *   rustc from those sources; a flag reaching rustc another way, or a source-level `#[target_feature]`, is the Wasm
- *   leg's loader's to refuse from the module itself (its target_features section, FX-2).
+ *   rustc from those sources. The Wasm leg's loader reads the module's target_features section (FX-2): a feature
+ *   enabled at compile time, by a flag from another route or a source-level `#[target_feature]`, shows there and is
+ *   refused; but the section is the linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can
+ *   rewrite it (built: sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check at all
+ *   (carry 85 (a)).
  * - Inputs that can lower lints and are not read here: a cargo config's [env] table (CLIPPY_ARGS, C2R-4 G, or any
  *   other variable it sets for rustc), what a member's build script prints (`cargo::rustc-env`, C2R-4 H), a
  *   RUSTC_WRAPPER program (allowed, for sccache) and a rust-toolchain.toml `path`. Lint capping by any of them
@@ -2029,9 +2083,11 @@ const DEPS_CHECKED =
   "npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
   "them, and keeps lib.rs's deny of the two clippy unsafe lints, and npt-hash-wasm relaxes it only by one #[allow] on " +
   "each of its seven listed exports, whose #[unsafe(no_mangle)] is its only unsafe code, each taking and giving u32 " +
-  "or i32 only; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but as a macro call; " +
+  "or i32 only; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but directly before `!` " +
+  "and an opening bracket; no attribute's text starts with a metavariable; " +
   "the environment and the cargo configs read pass no " +
-  "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no config include and " +
+  "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no linker, no profile, " +
+  "no config include and " +
   "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; no rustflags source holds a " +
   "flag (so none enables fma, simd128 or relaxed-simd or sets target-cpu=native) and neither the environment nor a " +
   "cargo config sets the build target, and rustc's read-back, given what cargo gives rustc, lists sse2 and " +
@@ -2208,7 +2264,10 @@ export function readScan(text) {
   return { catches, notCatchable, record };
 }
 
-/** The wasm step's time limit on each of its calls but its cargo builds: the step's whole 5-minute budget. */
+/**
+ * The wasm step's one deadline, shared by every call but its cargo builds from the first of them (carry 84 (a)): the
+ * step's whole 5-minute budget.
+ */
 const WASM_CALL_LIMIT_MS = 300_000;
 /** The real file operations of the wasm step (check.test.mjs passes its own), paths under the spike for `read`. */
 const WASM_IO = {
@@ -2245,28 +2304,55 @@ const WASM_IO = {
  *    ruling): a catch (recording R, tick T', field F) arms it at T' on R, native leg 1 and Wasm leg 1, whose first
  *    mismatch must be (R, T', F); "not catchable at this site on x86-64 and wasm32" is printed with the scan's record,
  *    and no std-sin leg runs.
+ *    The re-show (carry 83 (c)): the bitten native leg against a clean bites-build native leg (the hash-map half's
+ *    native leg 1, clean on the manifest's first recording before HASH_MAP_ARM: that pair's equal prefix) must first
+ *    differ at the scan's (recording, tick, field); a catch elsewhere fails the step, as no clean leg covers it.
  * It passes only with every manifest recording's ticks compared (its count each, their sum per pair) and 0 mismatching
- * for leg 1 against leg 2 and leg 1 against Wasm, h2's vectors equal and each bite's verdict as above. On a failure it
- * still runs every comparison it has the legs for; its last lines are one line per pair, h2 and each bite, then each
- * problem, then the summary, which main prints: the ticks per pair, the mismatches, each bite's outcome and each timed
- * call's seconds.
+ * for leg 1 against leg 2 and leg 1 against Wasm, `npt-lab hash` and its clean `compare` exiting 0 (carry 83 (b)), h2's
+ * vectors equal and each bite's verdict as above. On a failure it still runs every comparison it has the legs for; its
+ * last lines are one line per pair, h2 and each bite, then each problem, then the summary, which main prints: the
+ * ticks per pair, the mismatches, each bite's outcome and each timed call's seconds.
+ *
+ * Its options (the advisor's U4r build ruling, point 1 (4)): `manifest`, the recordings manifest every `npt-lab hash`
+ * call and the Wasm legs read (RECORDINGS_MANIFEST by default); `bitesHalf`, whether step 6 and its two bites builds
+ * run (by default they do). ops/runs/det-u4.mjs runs the clean half on crates/lab/det-recordings.json with the bites
+ * half off; check.mjs runs the defaults.
  */
-export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_STEP_DIR) {
+export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_STEP_DIR, { manifest = RECORDINGS_MANIFEST, bitesHalf = true } = {}) {
   const problems = [];
   const lines = [];
   const timed = [];
-  // Every call but the four cargo builds has WASM_CALL_LIMIT_MS (the advisor's U4d2 round-1 ruling, FX-7: INF-4): a
-  // hung leg would hold the lane lock for every other lane's cargo. The builds have none: under the exclusive lock no
-  // other cargo holds theirs, and a cold build's time is INF-1's to measure, not to cut.
+  // Every call but the four cargo builds shares one deadline, WASM_CALL_LIMIT_MS from the first of them (carry 84 (a);
+  // before it, each call had its own: the advisor's U4d2 round-1 ruling, FX-7: INF-4): each is given what is left, and
+  // once one is stopped, or the time is spent, the rest are not started, each named as skipped. A hung leg would hold
+  // the lane lock for every other lane's cargo. The builds have none: under the exclusive lock no other cargo holds
+  // theirs, and a cold build's time is INF-1's to measure, not to cut.
+  const share = `${WASM_CALL_LIMIT_MS / 1000} s shared by every call but its cargo builds, from the first of them`;
+  let deadline = null;
+  let stopped = null;
   const call = (label, cmd, args) => {
     const t0 = now();
-    const limit = cmd === "cargo" ? 0 : WASM_CALL_LIMIT_MS;
+    if (cmd !== "cargo") {
+      deadline ??= t0 + WASM_CALL_LIMIT_MS;
+      if (stopped || t0 >= deadline) {
+        problems.push(`${label}: not started (skipped): ${stopped ? `${stopped} was stopped at the step's deadline` : "the step's deadline had passed"} (${share})`);
+        return { ok: false, out: "", stdout: "", status: null, timedOut: false, skipped: true };
+      }
+    }
+    const limit = cmd === "cargo" ? 0 : deadline - t0;
     const r = runner(cmd, args, HERE, undefined, limit);
     timed.push(`${label} ${((now() - t0) / 1000).toFixed(1)}`);
-    if (r.timedOut) problems.push(`${label}: still running after ${limit / 1000} s, so it was stopped (the wasm step's limit on each call but its four cargo builds)`);
+    if (r.timedOut) {
+      stopped ??= label;
+      problems.push(`${label}: still running at the step's deadline (${share}; ${(limit / 1000).toFixed(1)} s were left for it), so it was stopped with every process it started`);
+    }
     return r;
   };
-  const failure = (r) => lastLines(r.out, 1).trim() || `exit ${r.status}`;
+  const failure = (r) => (r.skipped ? "not started (skipped: the step's deadline)" : lastLines(r.out, 1).trim() || `exit ${r.status}`);
+  // Carry 83 (b): a clean-half call must also exit 0; its lines alone are not its verdict.
+  const exited = (what, r) => {
+    if (r && !r.skipped && !r.timedOut && r.status !== 0) problems.push(`${what}: exit ${r.status ?? "by a signal"}, not 0 (the clean half requires exit 0 as well as its lines)`);
+  };
   const bin = join(dir, "bin");
   const rec = join(dir, "recordings");
   const meas = join(dir, "measured");
@@ -2274,9 +2360,9 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
   const sin = join(dir, "std-sin");
   io.reset(dir);
   for (const d of [bin, rec, meas, hm, sin]) io.mkdir(d);
-  // 1. The builds.
+  // 1. The builds (the two bites builds only with the bites half).
   const art = {};
-  for (const b of WASM_BUILDS) {
+  for (const b of WASM_BUILDS.filter((x) => bitesHalf || !["labBites", "wasmBites"].includes(x.key))) {
     const r = call(`build ${b.copy}`, "cargo", b.args);
     const path = r.ok ? builtArtifact(r.stdout ?? "", b) : null;
     if (!r.ok) problems.push(`the release build of ${b.what} failed: ${failure(r)}`);
@@ -2289,13 +2375,13 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
   // The manifest's recordings, as the loader needs them (npt-lab checks the manifest's own rules).
   let entries = [];
   try {
-    const m = JSON.parse(io.read(RECORDINGS_MANIFEST));
+    const m = JSON.parse(io.read(manifest));
     entries = m.recordings.map((e) => ({ name: e.name, first: e.first_tick, ticks: e.ticks }));
   } catch (e) {
-    problems.push(`${RECORDINGS_MANIFEST} cannot be read: ${e.message}`);
+    problems.push(`${manifest} cannot be read: ${e.message}`);
   }
   const total = entries.reduce((s, e) => s + e.ticks, 0);
-  const manifestArgs = ["--manifest", RECORDINGS_MANIFEST, "--dir", rec];
+  const manifestArgs = ["--manifest", manifest, "--dir", rec];
   const loader = (wasm, e, out, extra = []) => [LOADER, "--wasm", wasm, "--rec", join(rec, `${e.name}.rec`), "--name", e.name, "--first-tick", String(e.first), "--out", out, ...extra];
   const stepped = (r) => Number(r.out.match(/: (\d+) ticks stepped/)?.[1] ?? Number.NaN);
   // A recording regenerated in this run (the folder is new each run, and npt-lab writes each file once it is checked).
@@ -2317,6 +2403,7 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
   let vectors = [];
   if (art.lab && entries.length) {
     const legs = call("npt-lab hash", art.lab, ["hash", ...manifestArgs, "--out", meas]);
+    exited("npt-lab hash", legs);
     const regen = legs.out.match(/^npt-lab hash: (.*(?:the manifest has|not this build's Recording::GENERATOR).*)$/m)?.[1];
     if (regen) problems.push(`the recordings manifest: ${regen}`);
     if (/order digest/.test(legs.out)) problems.push("a clean-half native leg wrote an order digest: a bites build's output given to the measured half");
@@ -2336,6 +2423,7 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
     const compareArgs = ["hash", "compare", "--ours", join(meas, "native-1.lines"), "--theirs", theirs.join(","), ...manifestArgs];
     const names = ["--ours-name", "native leg 1", "--theirs-name", "Wasm leg 1"];
     const cr = theirs.length ? call("compare", art.lab, [...compareArgs, ...names]) : null;
+    exited("npt-lab hash compare (native leg 1 against Wasm leg 1)", cr);
     let c = cr ? readComparison(cr.out, "native leg 1", "Wasm leg 1") : null;
     const e = c?.first && !c.field ? entries.find((x) => x.name === c.first.recording) : null;
     if (e) {
@@ -2361,17 +2449,24 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
   // 6. The bites half.
   const bites = { hashMap: "not run", stdSin: "not run" };
   const index = {};
-  if (art.labBites && art.wasmBites && entries.some(has)) {
+  if (!bitesHalf) {
+    // Off by the option: no bites build was made and no bites-build call starts (ops/runs/det-u4.mjs).
+    bites.hashMap = "not run: the bites half is off (wasmStep's bitesHalf option)";
+    bites.stdSin = bites.hashMap;
+  } else if (art.labBites && art.wasmBites && entries.some(has)) {
     const listed = call("npt-lab bites", art.labBites, ["hash", "bites"]);
     for (const m of listed.out.matchAll(/^(\d+) (\S+)$/gm)) index[m[2]] = Number(m[1]);
     for (const id of Object.values(WASM_BITES)) if (!index[id]) problems.push(`npt-lab hash bites lists no ${id} (World::BITES)`);
   } else problems.push(`the bites half did not run: ${art.labBites && art.wasmBites ? "no recording was regenerated" : "no bites builds"}`);
   const rec0 = entries[0];
+  // Whether the hash-map half's native legs ran: its leg 1 is the std-sin re-show's clean bites-build leg.
+  let cleanLeg = false;
   if (index[WASM_BITES.hashMap] && rec0 && !has(rec0)) problems.push(`hash-map: ${rec0.name} was not regenerated, so it did not run`);
   if (index[WASM_BITES.hashMap] && rec0 && has(rec0)) {
     const arm = String(HASH_MAP_ARM);
     const bite = ["--bite", WASM_BITES.hashMap, "--arm", arm, "--on", rec0.name];
     const legs = call("hash-map native legs", art.labBites, ["hash", ...manifestArgs, "--out", hm, "--only", rec0.name, "--bite", WASM_BITES.hashMap, "--arm", arm, "--dump-at", arm]);
+    cleanLeg = !legs.skipped && !legs.timedOut;
     const digests = { "native leg 1": legs.out.match(/^leg 1 order digest (\S+)/m)?.[1], "native leg 2": legs.out.match(/^leg 2 order digest (\S+)/m)?.[1] };
     for (const k of [1, 2]) {
       const r = call(`hash-map Wasm ${k}`, process.execPath, loader(art.wasmBites, rec0, join(hm, `wasm-${k}.lines`), ["--bite", String(index[WASM_BITES.hashMap]), "--arm", arm, "--dump-at", arm, "--dump-out", join(hm, `wasm-${k}.dump`)]));
@@ -2438,8 +2533,29 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
         "Wasm leg 1",
       );
       const caught = c?.first?.recording === recording && c.first.tick === tick && c.field === field;
+      // The re-show (carry 83 (c)): the bite, not another native-Wasm difference, causes the mismatch judged. The bitten
+      // native leg against a clean bites-build native leg, the hash-map half's leg 1 (clean on the manifest's first
+      // recording before HASH_MAP_ARM: its pair's equal prefix), replayed clean in `compare`'s own process to name the
+      // field against the bitten leg's dump.
+      let reshown = false;
+      if (recording !== rec0.name || tick >= HASH_MAP_ARM) {
+        problems.push(
+          `std-sin: the scan's catch ${recording} tick ${tick} is not on the manifest's first recording (${rec0.name}) before the hash-map bite's arming tick ${HASH_MAP_ARM}, so no clean bites-build native leg covers it and the bite's cause is not re-shown`,
+        );
+      } else if (!cleanLeg) {
+        problems.push("std-sin: the re-show did not run: the hash-map half's native legs, its clean leg, did not run");
+      } else {
+        const s = call("std-sin re-show", art.labBites, ["hash", "compare", "--ours", join(hm, "native-1.lines"), "--theirs", join(sin, "native-1.lines"), ...manifestArgs, "--theirs-dump", join(sin, "native-1.dump"), "--ours-name", "clean native leg 1", "--theirs-name", "std-sin native leg 1"]);
+        const rs = readComparison(s.out, "clean native leg 1", "std-sin native leg 1");
+        reshown = rs?.first?.recording === recording && rs.first.tick === tick && rs.field === field;
+        if (!reshown) {
+          problems.push(
+            `std-sin: the re-show (the bitten native leg against a clean bites-build native leg) ${rs ? `first differs at ${rs.first ? `${rs.first.recording} tick ${rs.first.tick}, field ${rs.field ?? "none"}` : (rs.at ?? "no entry")}` : `wrote no comparison: ${failure(s)}`}, not at the scan's ${recording} tick ${tick}, ${field}`,
+          );
+        }
+      }
       bites.stdSin = caught
-        ? `caught at ${recording} tick ${tick}, ${field}, as the scan found`
+        ? `caught at ${recording} tick ${tick}, ${field}, as the scan found${reshown ? "; re-shown against a clean bites-build native leg" : ""}`
         : `NOT caught at the scan's ${recording} tick ${tick}, ${field}: ${c ? `first mismatch ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}` : "no comparison was written"}`;
       if (!caught) {
         problems.push(
````

### ops/runs/c-u4r-r2.mjs (new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs`:  1 file changed, 146 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
new file mode 100644
index 0000000..b8df242
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
@@ -0,0 +1,146 @@
+#!/usr/bin/env node
+// U4r's contract test r2 (u4r-brief.md 2.4 r2), re-made from a commit: the advisor's U4r build ruling 3, point 6 (4).
+// The builder's own r2 runs are builder evidence only (ruling 2, point 33); this script's run from the merge commit is
+// the record, before -17's judged runs and before carry 62 (h)'s setup check is decided (7.1).
+//   node spikes/netcode-proof/ops/runs/c-u4r-r2.mjs --date YYYY-MM-DD
+// - Each suite's three lines exactly as the builder ran them (.gate/u4r/c3-r2-p0.log, c3-r2-suites.log and
+//   c3-r2-bites.log, U4r's copy 3; then copy 5's runs of ruling 3), but for --date, --commit and --out: `npt-lab
+//   scenario` (SS or ES, 48 clients, seed 1, a 10 s warm-up and 20 s measured unless the suite names other lengths),
+//   then `npt-lab contract --logs <its folder> --series marks/SERIES.md`, then `npt-judge --marks marks/marks.json
+//   --logs <its folder> --only <the run's marks> --out <suite>-verdict.json`.
+// - The suites: the ten measured ones of copy 3 (p0, p2, p3, b1; -21's option-B stand-ins forged, clamp, fast-clock and
+//   lag-switch; es, carry 59's; evstart, carry 75's); ruling 3's point 3 pair (the tick16 bite at tick_start 64,651, so
+//   the 16-bit wrap falls at measured tick 585, 15 before the measured end; and its clean partner at 64,936, the wrap at
+//   measured tick 300); and point 4's bitten B1 run (PM-0001-18/no-fallback; since point 8 (A) the mirror keeps the
+//   frame the room's stale copy codes against). Copy 3's three earlier tick16 tries (tick_start 65,506) stay builder
+//   evidence and are not re-made.
+// - Every command runs whatever the one before gave, each exit code recorded: a FAIL is a result here, never a stop and
+//   never re-run (condition 5). Expected, by ruling 3: b1 fails PM-0001-2's everyone-in-view share, so its -17 and -18
+//   are INVALID and not counted (point 1: npt-room's, fixed in U4g); es fails ES's reliable-events band (point 2: U4h's);
+//   b1-nofallback fails -18's max check and is INVALID by pre:PM-0001-2 while the room is unfixed (points 4 and 8 (A)).
+//   Each suite's line in r2.txt states what is expected beside what came; the main session reads both.
+// - Run it under the lane lock, exclusive (u4r-brief.md section 4 rule 2: it holds bite runs, and its outputs are a
+//   record): `bash <runner> -x <copy> <log> node ops/runs/c-u4r-r2.mjs --date ...` or the main folder's own runner.
+// - It refuses uncommitted changes to tracked files (every log names its commit). Its outputs go under
+//   .gate/netcode-proof-test/r2-<commit12>/ (gitignored): one folder per suite (its three logs), <suite>-verdict.json
+//   beside it, the binaries under bin/, and r2.txt (the commit, the binaries' SHA-256s, then per suite each command, its
+//   exit code, its wall seconds and its output). Never overwritten. It stops before a run when less than 4 GB is free
+//   (section 4 rule 6's floor). The binaries: npt-lab's measured and bites builds and npt-judge, built --release
+//   --locked, each artifact's path read from cargo's --message-format=json messages (check.mjs's builtArtifact) and
+//   copied out before the next build replaces it.
+
+import { spawnSync } from "node:child_process";
+import { createHash } from "node:crypto";
+import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
+import { dirname, join } from "node:path";
+import { fileURLToPath } from "node:url";
+import { builtArtifact } from "../../check.mjs";
+
+const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
+const ROOT = join(SPIKE, "..", "..");
+const GATE = join(ROOT, ".gate", "netcode-proof-test");
+/** The free space below which no run starts, bytes (section 4 rule 6's 4 GB floor). */
+const FLOOR = 4 * 1024 ** 3;
+const MARKS_17 = "PM-0001-2,PM-0001-17,PM-0001-57";
+const B1 = "--profile B1 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 30 --bursts 2 --burst-first-s 5 --burst-every-s 15";
+
+/**
+ * The suites, in the order the builder ran them: `build` names the npt-lab build (`lab` measured, `bites`), `run` the
+ * run id, `args` the scenario's settings after --out, `marks` its header's marks (the judge's --only), `expect` ruling
+ * 3's or copy 3's outcome.
+ */
+const SUITES = [
+  { key: "p0", build: "lab", run: "u4r-r2-p0-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p0", expect: "contract PASS; PM-0001-2's SS checks, -17 (0 mismatches) and -57 pass on the run" },
+  { key: "p2", build: "lab", run: "u4r-r2-p2-1", args: "--scene SS --profile P2 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p2", expect: "contract PASS; PM-0001-2, -17 and -57 pass" },
+  { key: "p3", build: "lab", run: "u4r-r2-p3-1", args: "--scene SS --profile P3 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p3", expect: "contract PASS; PM-0001-2, -17 and -57 pass (carry 62 (g): P3, the reorder step on)" },
+  { key: "b1", build: "lab", run: "u4r-r2-b1-1", args: `--scene SS ${B1}`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", expect: "contract PASS; PM-0001-2 FAIL (everyone in view 0.273333, needs >= 0.99), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2, recorded and not counted; -57 passes (ruling 3, point 1: npt-room's, fixed in U4g)" },
+  { key: "forged", build: "lab", run: "u4r-r2-forged-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour presses", marks: "PM-0001-2", suite: "forged", expect: "contract PASS; PM-0001-2 passes" },
+  { key: "clamp", build: "lab", run: "u4r-r2-clamp-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour clamp", marks: "PM-0001-2", suite: "clamp", expect: "contract PASS; PM-0001-2 passes" },
+  { key: "fast-clock", build: "lab", run: "u4r-r2-fast-clock-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "fast-clock", expect: "contract PASS; PM-0001-2 passes" },
+  { key: "lag-switch", build: "lab", run: "u4r-r2-lag-switch-1", args: "--scene SS --profile P1 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "lag-switch", expect: "contract PASS; PM-0001-2 passes" },
+  { key: "es", build: "lab", run: "u4r-r2-es-1", args: "--scene ES --profile P0 --seed 1 --warmup-s 10 --measured-s 20", marks: "PM-0001-2", suite: "r2-es", expect: "contract PASS; PM-0001-2 FAIL by ES's reliable events (50.05 a second against 9.5-10.5), the other 11 ES checks pass; no ES run counts at U4r (ruling 3, point 2: U4h's)" },
+  { key: "evstart", build: "lab", run: "u4r-r2-evstart-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --event-start 65533 --warmup-s 10 --measured-s 20", marks: "PM-0001-2,PM-0001-17", suite: "r2-evstart", expect: "contract PASS; PM-0001-2 and -17 (0 mismatches) pass (carry 75)" },
+  { key: "tick16", build: "bites", run: "u4r-r2-tick16-4", args: "--scene SS --profile P0 --seed 1 --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16", marks: "PM-0001-2,PM-0001-17", suite: "r2-tick16", expect: "contract PASS (0 mirror breaches); PM-0001-2 passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped as it comes (false when the room's overflow resync resolves the measured ticks before the cap, true when the cap path of ruling 3, point 7 (a) ends the run); carry 62 (h)'s measured sum is the bite's own (tick16d: 720 = 48 x 15 post-wrap measured ticks) (ruling 3, point 3, as ruling 4, point 1 reads it; carry 73)" },
+  { key: "tick16-clean", build: "lab", run: "u4r-r2-tick16-clean-1", args: "--scene SS --profile P0 --seed 1 --tick-start 64936 --warmup-s 10 --measured-s 20 --after-max-s 5", marks: MARKS_17, suite: "r2-tick16-clean", expect: "contract PASS (0 mirror breaches); PM-0001-2, -17 (0 mismatches) and -57 pass; the after phase resolves without the cap (ruling 3, point 3)" },
+  { key: "b1-nofallback", build: "bites", run: "u4r-r2-b1-nofallback-1", args: `--scene SS ${B1} --role bite --bite PM-0001-18/no-fallback`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", expect: "contract PASS (0 mirror breaches, ruling 3, point 8 (A)); -18's max check FAILs (a heal later than 500 ms); INVALID by pre:PM-0001-2 is expected while npt-room is unfixed (ruling 3, points 4 and 8 (A))" },
+];
+
+function sh(cmd, args, cwd = SPIKE) {
+  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
+  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
+  return r.stdout.trim();
+}
+
+const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");
+
+/** One release build, its artifact's path read from cargo's JSON messages and copied to `to`. */
+function build(args, target, to) {
+  const out = sh("cargo", ["build", "--release", "--locked", ...args, "--message-format=json"]);
+  const path = builtArtifact(out, { target, kind: "bin" });
+  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
+  copyFileSync(path, to);
+  return to;
+}
+
+/** Stops before a run when less than FLOOR bytes are free where the logs go. */
+function floor(what) {
+  const s = statfsSync(ROOT);
+  const free = s.bavail * s.bsize;
+  if (free < FLOOR) throw new Error(`${what}: ${(free / 1024 ** 3).toFixed(1)} GB free, below the 4 GB floor: stopped before it`);
+}
+
+/** One command, run from the spike whatever it gives: its line, exit code, wall seconds and output. */
+function step(exe, args) {
+  const t0 = Date.now();
+  const r = spawnSync(exe, args, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
+  const secs = ((Date.now() - t0) / 1000).toFixed(1);
+  const code = r.status ?? `signal ${r.signal}`;
+  return [`$ ${[exe, ...args].join(" ")}`, `exit ${code} (wall ${secs} s)`, `${r.stdout ?? ""}${r.stderr ?? ""}`.trimEnd()];
+}
+
+function main(argv) {
+  const date = argv.includes("--date") ? argv[argv.indexOf("--date") + 1] : undefined;
+  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || argv.length !== 2) throw new Error("usage: c-u4r-r2.mjs --date YYYY-MM-DD");
+  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) {
+    throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
+  }
+  const commit = sh("git", ["rev-parse", "HEAD"]);
+  const out = join(GATE, `r2-${commit.slice(0, 12)}`);
+  if (existsSync(out)) throw new Error(`${out} exists: a run set is never overwritten`);
+  const bin = join(out, "bin");
+  mkdirSync(bin, { recursive: true });
+  const exe = {
+    lab: build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab")),
+    bites: build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites")),
+  };
+  const judge = build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge"));
+  const lines = [
+    `# U4r's r2, re-made from commit ${commit} on ${date} (the advisor's U4r build ruling 3, point 6 (4))`,
+    `npt-lab (measured build) sha256 ${sha256(exe.lab)}`,
+    `npt-lab (bites build) sha256 ${sha256(exe.bites)}`,
+    `npt-judge sha256 ${sha256(judge)}`,
+    "",
+  ];
+  for (const s of SUITES) {
+    floor(`suite ${s.key}`);
+    const logs = join(out, s.key);
+    const said = [
+      `## suite ${s.key} (the ${s.build === "lab" ? "measured" : "bites"} build); expected: ${s.expect}`,
+      ...step(exe[s.build], ["scenario", "--run", s.run, "--set", "u4r-r2", "--date", date, "--commit", commit, "--out", logs, ...s.args.split(" "), "--marks", s.marks, "--suite", s.suite]),
+      ...step(exe[s.build], ["contract", "--logs", logs, "--series", "marks/SERIES.md"]),
+      ...step(judge, ["--marks", "marks/marks.json", "--logs", logs, "--only", s.marks, "--out", join(out, `${s.key}-verdict.json`)]),
+      "",
+    ];
+    console.log(said.join("\n"));
+    lines.push(...said);
+  }
+  writeFileSync(join(out, "r2.txt"), `${lines.join("\n")}\n`);
+  console.log(`written: ${join(".gate", "netcode-proof-test", `r2-${commit.slice(0, 12)}`, "r2.txt")}`);
+}
+
+try {
+  main(process.argv.slice(2));
+} catch (e) {
+  console.error(`c-u4r-r2: ${e.message}`);
+  process.exit(1);
+}
````

### ops/runs/c-u4r-runs.mjs (new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs`:  1 file changed, 216 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
new file mode 100644
index 0000000..ca29c1a
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
@@ -0,0 +1,216 @@
+#!/usr/bin/env node
+// U4r's judged runs and the -30 pre-check, in this container (u4r-brief.md 2.3 and section 7; c-u2-pm-0001-3.mjs's
+// model). One script for every set, the set named first:
+//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date YYYY-MM-DD
+//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs judged --profile P0|P2|P3|B1 --date YYYY-MM-DD
+// - precheck (set u4r-precheck-30; 7.2): SS P0 seed 4's recording made again by `npt-lab scenario --record` at its
+//   crates/lab/precheck-recordings.json entry's settings (the set stops if that run's rewinds refused are above 0: 7.1),
+//   then three pairs by `npt-lab precheck`, each checking the file against that entry first: clean (the measured
+//   build), PM-0001-30/wallclock and PM-0001-30/late-command (the bites build), each bite on the L1 leg. runs.txt holds
+//   every pair's lines whole (both legs' records, the comparer's output and the result) and no verdict: the judge
+//   never reads these runs.
+// - judged (set u4r-<profile>; 7.1): the profile's three measured runs (seeds 1 to 3; SS, 48 clients, L1, a 10 s
+//   warm-up, 600 s measured, tick_start 63,736; B1 with 10 bursts, every 60 s from 30 s into the measured phase), their
+//   headers listing PM-0001-2 (every L1 part's pre:PM-0001-2), PM-0001-17 and PM-0001-57 (-57's parts are on -17's
+//   runs), and PM-0001-18 on B1; then, in the bites build at seed 1 with the same settings, one run per bite of those
+//   marks (marks.json's lists, read at run time) whose part has this profile: -17's and -57's on P0, each header
+//   listing PM-0001-2 and the bite's own mark; then `npt-judge --only` on the set's folder. One profile per call keeps
+//   each set's logs and the judge's memory to one profile's (the advisor's U4r build ruling, point 3: the main session
+//   plans the disk and judges each set's folder apart).
+// - The tick16 bite run (the advisor's U4r build ruling 3, point 3): its tick_start is computed from its own lengths,
+//   65,551 - (warm-up ticks + measured ticks) (47,251 for 300 + 18,000), so the 16-bit wrap falls 15 ticks before the
+//   measured phase's end and the measured window ends before the bitten room evicts; every other -17 run, clean and the
+//   client-rounding bite run, starts at 63,736 (U4R-Q7 (a) as that ruling reads it).
+// - No -18 bite run at U4r (the advisor's U4r build ruling 3, point 1): -18's bitten judged run (PM-0001-18/no-fallback)
+//   moves to U4g with the room's fix; ws-budget-x2 is L2's. The three B1 measured runs stay in this set for -57's L1 B1
+//   part, which has no precondition; their -17 and -18 verdicts are recorded INVALID (pre:PM-0001-2's everyone-in-view
+//   share, npt-room's recovery after each burst) and never counted.
+// - It refuses uncommitted changes to tracked files (every log names its commit); raw logs go under
+//   .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run), results under results/u4r/<commit12>/<profile>/
+//   or results/u4r-precheck-30/<commit12>/; neither is ever overwritten. It stops before a run when less than 4 GB is
+//   free (u4r-brief.md section 4 rule 6's floor).
+// - Both npt-lab builds and npt-judge are built --release --locked, each artifact's path read from cargo's
+//   --message-format=json messages (check.mjs's builtArtifact, as the wasm step reads its builds) and copied out before
+//   the next build replaces it: so it is right under the main folder's own cache (CARGO_TARGET_DIR unset, as plan §7.1
+//   item 6 has judged runs) and under the shared target alike.
+// - Each npt-lab process is timed (the wall time around the call) and its seconds go on its runs.txt line beside its
+//   ticks (npt-lab's summary line), for the ticks-per-second figure (2.4 r5, 7.3).
+// - No run logs its clients' bytes (`scenario --bytes`): no part these sets judge reads a replay log (only an x86_64
+//   replay log of an L2 Arm run is read: judge/src/figures.rs:222-233).
+
+import { spawnSync } from "node:child_process";
+import { createHash } from "node:crypto";
+import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
+import { dirname, join } from "node:path";
+import { fileURLToPath } from "node:url";
+import { builtArtifact } from "../../check.mjs";
+
+const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
+const ROOT = join(SPIKE, "..", "..");
+const GATE = join(ROOT, ".gate", "netcode-proof-test");
+const SEEDS = [1, 2, 3];
+const PROFILES = ["P0", "P2", "P3", "B1"];
+/** -17's L1 runs start here, clean and the client-rounding bite run (U4R-Q7 (a), as ruling 3, point 3 reads it). */
+const TICK_START = 63_736;
+/** Each judged run's warm-up and measured lengths, s (plan :767: 10 minutes measured). */
+const WARMUP_S = 10;
+const MEASURED_S = 600;
+/** npt_sim::TICK_HZ: ticks per second. */
+const TICK_HZ = 30;
+/**
+ * The tick16 bite run's start (the advisor's U4r build ruling 3, point 3): from its own lengths, so the wrap (65,536)
+ * falls 15 ticks before its measured phase's end: 65,551 - (warm-up ticks + measured ticks).
+ */
+const TICK16 = "PM-0001-17/tick16";
+const TICK16_START = 65_536 + 15 - (WARMUP_S + MEASURED_S) * TICK_HZ;
+/** The bites of these marks that only L2 plants (marks.json: -18's ws-budget-x2 is the L2 B1 ws part's). */
+const L2_ONLY = ["PM-0001-18/ws-budget-x2"];
+/** The bites whose judged run is not U4r's (the advisor's U4r build ruling 3, point 1: -18's no-fallback, to U4g). */
+const NOT_AT_U4R = ["PM-0001-18/no-fallback"];
+/** The free space below which no run starts, bytes (section 4 rule 6's 4 GB floor). */
+const FLOOR = 4 * 1024 ** 3;
+/** precheck-recordings.json's scene ids (Scene::by_id). */
+const SCENES = { 1: "SS", 2: "ES", 3: "CO", 4: "TC" };
+
+function sh(cmd, args, cwd = ROOT) {
+  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
+  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
+  return r.stdout.trim();
+}
+
+const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");
+
+/** One release build, its artifact's path read from cargo's JSON messages and copied to `to`. */
+function build(args, target, to) {
+  const out = sh("cargo", ["build", "--release", "--locked", ...args, "--message-format=json"], SPIKE);
+  const path = builtArtifact(out, { target, kind: "bin" });
+  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
+  copyFileSync(path, to);
+  return to;
+}
+
+/** Stops before a run when less than FLOOR bytes are free where the logs go. */
+function floor(what) {
+  const s = statfsSync(ROOT);
+  const free = s.bavail * s.bsize;
+  if (free < FLOOR) throw new Error(`${what}: ${(free / 1024 ** 3).toFixed(1)} GB free, below the 4 GB floor: stopped before it`);
+}
+
+/** One npt-lab process, timed: its summary line with the wall seconds. */
+function timed(exe, args) {
+  const t0 = Date.now();
+  const out = sh(exe, args);
+  const secs = ((Date.now() - t0) / 1000).toFixed(1);
+  return { out, secs };
+}
+
+function main(argv) {
+  const set = argv[0];
+  const opt = (k) => (argv.includes(k) ? argv[argv.indexOf(k) + 1] : undefined);
+  const date = opt("--date");
+  const profile = opt("--profile");
+  const usage = "usage: c-u4r-runs.mjs precheck --date YYYY-MM-DD | judged --profile P0|P2|P3|B1 --date YYYY-MM-DD";
+  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || !["precheck", "judged"].includes(set)) throw new Error(usage);
+  if (set === "judged" && !PROFILES.includes(profile)) throw new Error(usage);
+  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) {
+    throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
+  }
+  const commit = sh("git", ["rev-parse", "HEAD"]);
+  const short = commit.slice(0, 12);
+  const name = set === "precheck" ? "u4r-precheck-30" : `u4r-${profile}`;
+  const logs = join(GATE, "runs", `${name}-${short}`);
+  const results = set === "precheck" ? join(SPIKE, "results", "u4r-precheck-30", short) : join(SPIKE, "results", "u4r", short, profile);
+  for (const d of [logs, results]) if (existsSync(d)) throw new Error(`${d} exists: a run set is never overwritten`);
+
+  // Both builds of npt-lab, each copied out before the next build replaces it, and the judge.
+  const bin = join(GATE, "bin", `${name}-${short}`);
+  mkdirSync(bin, { recursive: true });
+  const lab = build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab"));
+  const labBites = build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites"));
+  const judge = set === "judged" ? build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge")) : null;
+  mkdirSync(logs, { recursive: true });
+  const head = [
+    `commit ${commit}`,
+    `npt-lab (measured build) sha256 ${sha256(lab)}`,
+    `npt-lab (bites build) sha256 ${sha256(labBites)}`,
+    ...(judge ? [`npt-judge sha256 ${sha256(judge)}`] : []),
+    `raw logs: .gate/netcode-proof-test/runs/${name}-${short}/ (not committed: plan §7.2)`,
+  ];
+  const common = ["--set", name, "--date", date, "--commit", commit];
+  const lines = [];
+
+  if (set === "precheck") {
+    const manifest = join(SPIKE, "crates", "lab", "precheck-recordings.json");
+    const [e] = JSON.parse(readFileSync(manifest, "utf8")).recordings;
+    const rec = join(logs, `${e.name}.rec`);
+    floor(`the recording ${e.name}`);
+    const made = timed(lab, [
+      "scenario", "--run", `record-${e.name}`, ...common, "--out", join(logs, `record-${e.name}`), "--scene", SCENES[e.scene],
+      "--profile", e.profile, "--seed", String(e.seed), "--clients", String(e.clients), "--tick-start", String(e.first_tick),
+      "--warmup-s", String(e.warmup_s), "--measured-s", String(e.measured_s), "--logs", "0", "--suite", "record", "--record", rec,
+    ]);
+    console.log(made.out);
+    lines.push(`# the recording (npt-lab scenario --record at the manifest entry's settings; wall ${made.secs} s)`, made.out, `${e.name} sha256 ${sha256(rec)}`);
+    const refused = Number(made.out.match(/rewinds refused (\d+)/)?.[1] ?? Number.NaN);
+    if (refused !== 0) throw new Error(`${e.name}'s run refused ${refused} rewinds: the set stops before the pre-check (7.1; carry 40)`);
+    const pairs = [
+      { pair: "clean", exe: lab, extra: [], expected: "PASS" },
+      { pair: "wallclock", exe: labBites, extra: ["--bite", "PM-0001-30/wallclock", "--bite-leg", "L1"], expected: "FAIL" },
+      { pair: "late-command", exe: labBites, extra: ["--bite", "PM-0001-30/late-command", "--bite-leg", "L1"], expected: "VOID" },
+    ];
+    for (const p of pairs) {
+      floor(`the ${p.pair} pair`);
+      const r = timed(p.exe, ["precheck", "--manifest", manifest, "--recording", rec, "--commit", commit, ...p.extra]);
+      const result = r.out.match(/^result: (\S+)/m)?.[1] ?? "none";
+      const said = `## pair ${p.pair} (wall ${r.secs} s): result ${result}, expected ${p.expected}${result === p.expected ? "" : " (NOT as expected: to the advisor)"}`;
+      console.log(said);
+      lines.push(said, r.out);
+    }
+    mkdirSync(results, { recursive: true });
+    const top = [`# u4r-precheck-30: the -30 pre-check (u4r-brief.md 7.2), run in this container on ${date}; no verdict (no judged id)`];
+    writeFileSync(join(results, "runs.txt"), `${[...top, ...head, "", ...lines].join("\n")}\n`);
+    console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r-precheck-30", short)}/runs.txt`);
+    return;
+  }
+
+  const marks = JSON.parse(readFileSync(join(SPIKE, "marks", "marks.json"), "utf8"));
+  const listed = ["PM-0001-2", "PM-0001-17", "PM-0001-57", ...(profile === "B1" ? ["PM-0001-18"] : [])];
+  const settings = (tickStart) => ["--scene", "SS", "--profile", profile, "--tick-start", String(tickStart), "--warmup-s", String(WARMUP_S), "--measured-s", String(MEASURED_S), "--suite", name];
+  const one = (exe, run, seed, extra, tickStart = TICK_START) => {
+    floor(run);
+    const r = timed(exe, ["scenario", "--run", run, ...common, "--out", join(logs, run), "--seed", String(seed), ...settings(tickStart), ...extra]);
+    const said = `${r.out} (wall ${r.secs} s)`;
+    console.log(said);
+    lines.push(said);
+  };
+  for (const s of SEEDS) one(lab, `l1-${profile.toLowerCase()}-s${s}`, s, ["--role", "measured", "--marks", listed.join(",")]);
+  // The bites: one run each at seed 1 on P0, the profile of a part that reads it (-17's and -57's); -18's none at U4r
+  // (ruling 3, point 1: to U4g), so a B1 set runs no bite. The tick16 run at TICK16_START (ruling 3, point 3).
+  const bitesOn = (mark) => (mark === "PM-0001-18" ? null : "P0");
+  for (const m of marks.marks.filter((x) => listed.includes(x.id) && x.id !== "PM-0001-2" && bitesOn(x.id) === profile)) {
+    for (const b of (m.bites ?? []).filter((x) => !L2_ONLY.includes(x.id) && !NOT_AT_U4R.includes(x.id))) {
+      one(labBites, `bite-${b.id.split("/")[1]}`, 1, ["--role", "bite", "--bite", b.id, "--marks", `PM-0001-2,${m.id}`], b.id === TICK16 ? TICK16_START : TICK_START);
+    }
+  }
+  const verdict = join(logs, "verdict.json");
+  const judged = sh(judge, ["--marks", join(SPIKE, "marks", "marks.json"), "--logs", logs, "--only", listed.join(","), "--out", verdict]);
+  console.log(judged);
+  mkdirSync(results, { recursive: true });
+  copyFileSync(verdict, join(results, "verdict.json"));
+  const top = [
+    `# ${name}: -17's, -57's${profile === "B1" ? " and -18's" : ""} L1 ${profile} part(s), run in this container on ${date} (u4r-brief.md 7.1)`,
+    ...head,
+    `judge: ${judged}`,
+    "",
+    "# one line per run (npt-lab's own summary line and its wall time; the judge reads the logs, never these lines)",
+  ];
+  writeFileSync(join(results, "runs.txt"), `${[...top, ...lines].join("\n")}\n`);
+  console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r", short, profile)}/verdict.json and runs.txt`);
+}
+
+try {
+  main(process.argv.slice(2));
+} catch (e) {
+  console.error(`c-u4r-runs: ${e.message}`);
+  process.exit(1);
+}
````

### ops/runs/det-u4.mjs (new)
_`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs`:  1 file changed, 36 insertions(+)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs
new file mode 100644
index 0000000..a0ba0f4
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs
@@ -0,0 +1,36 @@
+#!/usr/bin/env node
+// U4r's determinism run on its two runner recordings (the advisor's U4r build ruling, point 1, route (c);
+// u4-build-plan.md :800-802): check.mjs's `wasm` step, imported (never a copy), on crates/lab/det-recordings.json with
+// the bites half off, in its own folder under .gate/ (never the step's WASM_STEP_DIR).
+//   node spikes/netcode-proof/ops/runs/det-u4.mjs
+// - Its clean half: each runner entry regenerated inside `npt-lab hash` by its `npt-lab scenario --record` settings and
+//   checked by length and SHA-256 (carry 82), native leg 1 against native leg 2, Wasm leg 1 against native leg 1, and
+//   h2's vectors. The bites half proves the comparer catches a planted divergence; check.mjs shows that on SS-s1 at
+//   every run, on the same code, so it is off here (the ruling's point 1).
+// - The step's rules hold here as they are wasmStep's own: `npt-lab hash` and its compare must exit 0 (carry 83 (b));
+//   every call but the cargo builds shares one 300 s deadline from the first of them, and a call stopped by it takes
+//   every process it started (carry 84).
+// - It prints as check.mjs's main prints a step (PASS or FAIL, the step's seconds and its summary: each pair, h2 and
+//   each call's seconds; on a FAIL its last 40 lines and the file under .gate/ that keeps its whole output) and exits 0
+//   only on PASS. A FAIL is never re-run into a PASS (u4r-brief.md section 4's no-retry rule for the wasm step).
+// - Run it as check.mjs is run: under the lane lock, with the session's CARGO_TARGET_DIR (the step's cargo builds read
+//   their artifacts' paths from cargo's JSON messages, so either cache serves).
+
+import { dirname, join } from "node:path";
+import { fileURLToPath } from "node:url";
+import { FAILED_STEP_DIR, main, wasmStep } from "../../check.mjs";
+
+const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
+const ROOT = join(SPIKE, "..", "..");
+/** The manifest it reads: U4r's two runner recordings, in recordings.json's schema (the ruling's point 1 (2)). */
+const DET_MANIFEST = "crates/lab/det-recordings.json";
+/** Its own folder under .gate/ (gitignored), emptied at each run as the step empties its own. */
+const DET_DIR = join(ROOT, ".gate", "netcode-proof-test", "det-u4");
+
+const steps = { "det-u4": () => wasmStep(undefined, undefined, undefined, DET_DIR, { manifest: DET_MANIFEST, bitesHalf: false }) };
+try {
+  process.exitCode = main(process.argv.slice(2), steps, console.log, FAILED_STEP_DIR);
+} catch (e) {
+  console.error(`det-u4: ${e.message}`);
+  process.exitCode = 2;
+}
````

### marks/SERIES.md
_`git diff --no-index spikes/netcode-proof/marks/SERIES.md .gate/u4r-snap1/spikes/netcode-proof/marks/SERIES.md`:  1 file changed, 31 insertions(+)._

````diff
diff --git a/spikes/netcode-proof/marks/SERIES.md b/.gate/u4r-snap1/spikes/netcode-proof/marks/SERIES.md
index 4d08c19..9672bd2 100644
--- a/spikes/netcode-proof/marks/SERIES.md
+++ b/.gate/u4r-snap1/spikes/netcode-proof/marks/SERIES.md
@@ -330,6 +330,37 @@ row; the script fails otherwise). Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-6
 | `header.params.burst_spacing_ns` | ns | npt-lab (L1), the run script (L2) | B1's scheduled interval between burst starts, before each burst's offset; a practice run and its counted run share it | -18 (the rule) |
 | `header.params.config_hz` | Hz | the run script (L2) | RS's kernel CONFIG_HZ (the ruling's `machine.config_hz`, as a header param: README §2.5) | -18 (the practice record's header checks) |
 
+### Names added with U4r (the L1 runner, npt-lab `scenario`; u4r-brief.md 2.1; U4R-Q2 (c): new rows only, 2026-11-13)
+
+Every name U4r's runner writes that had no row. The header params record the run's settings (README §1's `params`:
+name to JSON value); the events in the lab log (`client_counts`, `class_audit`, `fate_log`, `run_lengths`, `recording`)
+are written once at the run's end, after the measured phase. No mark reads them yet but -18's two `burst_heal` fields
+(reported).
+
+| Name | Unit | Written by (crate) | Meaning | Marks |
+|---|---|---|---|---|
+| `header.params.lead_ticks_count` | count (ticks) | npt-lab | the command lead every client of the run stamps with: the profile's round trip plus its jitter in ticks, rounded up, at least 1, plus 1 (carry 74 as U4R-Q14 ruled: P0 2, P1 3, P2 and B1 5, P3 7) | none (recorded beside `cmd_timing`) |
+| `header.params.event_start_count`, `.cmd_start_count` | count (a sequence) | npt-lab | the room's first event sequence (`Room::with_event_start`) and each client's first command sequence (`ClientEndpoint::with_starts`): 0, or a run's start near the 16-bit wrap (carry 75) | none (recorded) |
+| `header.params.event_queue_cap_provisional` | flag (true or false) | npt-lab (from `Room::event_cap`) | true while the room's event-queue cap is provisional (carry 57) | none (recorded beside `event_queue_cap_count`) |
+| `header.params.warmup_ticks_count`, `.measured_ticks_count` | count (ticks) | npt-lab | the run's warm-up and measured lengths in ticks (the after phase's: `event.run_lengths.after_ticks_count`) | none (recorded) |
+| `header.params.behaviour` | string | npt-lab | the bots' behaviour (npt-bots' `Behaviour`): `brawl`, or an open-loop script `presses`, `clamp` or `steady` | none (recorded) |
+| `header.params.features` | list of strings | npt-lab | the npt-lab build's features: `["bites"]` or `[]` (the header's `build.features` is the judge's) | none (recorded) |
+| `header.params.every_step` | flag (true) | npt-lab | present only on D13's test-only runs, which give a command at every client step (never a judged run: refused with marks) | none |
+| `header.params.stand_in_burn_ns`, `.stand_in_top_up_ns` | ns | npt-lab | S1's stand-in burn and top-up as run: 0 on L1 (stand-ins off: §4.13) | none (recorded) |
+| `header.params.weapon_interval_us`, `.weapon_damage_count`, `.weapon_reach_cm`, `.weapon_magazine_count`, `.weapon_reload_ms`, `.weapon_spread_urad` | as each name's suffix (µs, count, cm, count, ms, µrad) | npt-lab (from `Scene::weapon`) | the scene's weapon: shot interval, damage, reach, magazine, reload and spread (carry 60) | none (recorded) |
+| `header.params.soldier_health_count` | count | npt-lab (from `Scene::health`) | the scene's soldiers' health (carry 60) | none (recorded) |
+| `tick.s.input_payload_bytes` | bytes | npt-lab | the room log: each `Input` datagram's payload the clients sent since the last tick, in send order (not one per slot) | none (recorded) |
+| `event.admit.session`, `.in_roster` | identifier; flag (true or false) | npt-lab (the room log) | per admission (field `slot`): the session's id in the run, and whether its join was in the room's roster (README §4's `roster` rule reads `in_roster`) | the `roster` rule |
+| `event.burst_heal.full_state`, `.queue_max_count` | flag; count | npt-lab (L1) | per heal record (fields `slot`, `burst`): 1 when a `FullState` reached that client during the burst (a full state never counts as the heal); the client's event queue's high-water mark from the burst's start to its heal (carry 58) | -18 (reported) |
+| `event.burst_eviction.baseline_age` | count (ticks), signed | npt-lab (from npt-room) | per client per burst (fields `slot`, `burst`): the age of the client's baseline at the first tick after the burst; 0: coded against the per-type defaults (no baseline); -1: no snapshot sent that tick | -18 (recorded) |
+| `event.counter_wrap.counter` | name | npt-lab | the counter that wrapped (beside `tick_count`) | -58 (recorded) |
+| `event.snap_refused.kind` | name | npt-lab (from npt-room's refusal counters) | `out_of_range`, `too_many`, `too_large` or `not_canonical` (beside `tick_count`) | none (recorded) |
+| `event.client_counts.commands_count`, `.steps_without_command_count`, `.duplicate_stamps_count`, `.late_count`, `.press_refused_count`, `.buffer_full_count`, `.rewinds_refused_count`, `.stale_count`, `.events_queued_count`, `.overflow_resyncs_count` | count | npt-lab (from npt-bots, npt-room and npt-client's counters) | per client at the run's end (field `slot`): commands given, client steps without one, two commands stamped for one tick (D13 only), the room's late, press-refused, buffer-full and refused-rewind counts, `ClientEndpoint::stale()`, events queued, overflow resyncs (carries 74 and 77 (c)) | none (r2 reports them) |
+| `event.class_audit.owner_count`, `.spatial_count`, `.spatial_unchecked_count`, `.public_count`, `.events_count`, `.team_count`, `.breaches_count`, `.first_breach` | count; the last a text | npt-lab (the room-side mirror) | r4's class audit over the run: own-state records, entity records checked against the interest radius and not checked, headers, events, team records (0: no team section in the spike), every breach, and the first breach's text | none (r4) |
+| `event.fate_log.sha256` | hex string | npt-lab (from npt-l1's fate log) | the SHA-256 of the run's L1 fate log (r1, the twin test) | none (r1) |
+| `event.run_lengths.warmup_ticks_count`, `.measured_ticks_count`, `.after_ticks_count`, `.order_digest`, `.after_capped` | count (ticks); 16 hex digits; flag (true or false) | npt-lab | the run's three phases in ticks; the digest of the order its clients connected in (r1's bite); true when a bite run's after phase reached its cap with measured ticks unresolved, so the stop decided them lost (carry 73's tick16 past the wrap; a measured run fails there instead) | none (recorded) |
+| `event.recording.ticks_count`, `.bytes`, `.sha256`, `.span_ms` | count; bytes; hex string; ms | npt-lab | a `--record` run's recording: its ticks, size, SHA-256 and the room's hit-volume span (carry 82) | none (recorded) |
+
 ## Series read only by figures (18)
 
 | Name | Unit | Written by (crate) | Meaning | Figures |
````

### check.test.mjs (check.mjs's self-tests)
_ 1 file changed, 200 insertions(+), 34 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4r-packet-tests.md` (`git diff --no-index spikes/netcode-proof/check.test.mjs .gate/u4r-snap1/spikes/netcode-proof/check.test.mjs`)._

### npt-bots' tests (crates/bots/tests/behaviour.rs, new)
_ 1 file changed, 236 insertions(+); the diff itself is in `.checkpoints/netcode-proof-test/u4r-packet-tests.md` (`git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/bots/tests/behaviour.rs`)._

### npt-server's schedule tests (crates/server/tests/schedule.rs)
_ 1 file changed, 12 insertions(+), 2 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4r-packet-tests.md` (`git diff --no-index spikes/netcode-proof/crates/server/tests/schedule.rs .gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs`)._

## What it is judged against

### The approved plan: §5 U4r, the L1 runner and its judged runs (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 750-803 (sha256 1e631c55f4c4), as they are:_

````text
### U4r, the L1 runner and its judged runs (depends on U4c, U4d2, U4e, U4j; size M; its own brief)
- **Builds:** npt-lab: `SimClock`; `L1Scenario` (RQ8's order); the header params (scene, clients, profile, seed,
  `cmd_timing`, `tick_start`, `ticks_below_wrap`, suite, `stand_ins`); the `--bite` dispatch to each instance; the
  per-process logs with `admit` events and footers (§4.12); `scenario` (with `--record`) and `replay` (the x86-64
  re-decode, R-8: it re-decodes a run's logged client bytes natively and writes the clients' hashes; the advisor's
  condition 9); three runner recordings (§4.15: SS P2 seed 1, ES P3 seed 2, and SS P0 seed 4 for the -30 pre-check: the
  advisor's condition 3; TC B1 seed 3 is U4h's: U4R-Q9); -57's pairing (§4.6). npt-bots' library: the brawl `Behaviour`
  and the open-loop scripts. Run scripts ops/runs/c-u4r-*.mjs, as U2's c-u2-pm-0001-3.mjs.
- **Tests:** the runner's twin test with its bite (§4.11); the contract test (§4.12), which also shows every PM-0001-2
  check passing, not only resolving, on the short runs of each precondition suite: -17's four profiles, -18's B1,
  -20's P3 and each of -21's eight suites with its open-loop scripts; a suite that fails comes to the advisor before its
  marks count, as §4.13 says for the shot band (the advisor's condition 5); `replay`'s U4 test (the advisor's
  condition 9): one short L1 run (SS P2) logs each client's received bytes under .gate/, and `replay`, re-decoding
  them, gives the clients' logged hashes at every (client, tick); bite: the re-decoder's grid config floored, which must
  give mismatches; the class audit, catalogue item 5, at SS P2 over 3 seeds with the owner-to-all bite; the runner's
  measured ticks per second, and the total wall time of the run set stated before it starts.
- **Then the judged runs** (after the merge, in the main folder, by `npt-judge --only`, 3 seeds per part):
  - -17 at L1 P0, P2, P3 and B1 (10 min, 48 clients, SS); bites tick16 (runs starting at `tick_start` = 63,736) and
    client-rounding.
  - -57 on -17's runs; bites pos-4cm, yaw-12bit, pitch-8bit.
  - -18 at L1 B1 (10 bursts, each offset 0 to 1 tick; the fresh-delta reading: advisor-u3q.md:180-186); bite
    no-fallback, with `burst_eviction` logged.
  - **-30's pre-check, a recorded npt-lab test, not a judged id (Q10; the advisor's conditions 2 and 3):**
    - suite `precheck-30`; one leg over Loopback (layer `loopback`), one over L1 P0 (layer `L1`); replay clients send
      the SS P0 recording's commands (§4.15) to the room on both legs, under option B with the input buffer deep enough
      for P0; both legs' input-buffer depth (`buffer_depth_ticks`) recorded (marks.json -30's notes; condition 3);
    - the comparer: npt-lab's `HashCompare` (built in U4d2) compares the two legs' per-tick server hashes and names the
      first tick that differs; `tick.v.late_count` must sum to 0 on both legs (condition 2);
    - the setup condition (condition 3): 0 view-time replacements and 0 clipped shots on both legs, each replacement
      and clip logged per shot; a leg with any, or with `late_count` above 0, makes the pair VOID, reported, never a
      pass or a fail;
    - why the L1 leg's RTT before its first sample cannot make the legs decide differently (condition 3): Loopback's
      `rtt_ns` is always Some(0) (link/src/lib.rs:203-205), while L1 starts at U4c's 333 ms. The RTT reaches the room
      only through the tolerance's accept-or-replace and the cap's clip (§4.9), and an accepted view time's Rewind
      depends on the view time alone, so the legs can differ through the RTT only by a replacement or a clip, which the
      setup condition makes VOID. In a correct build it does not arise: each replay client sends its first `Input` only
      after it decodes `JoinAccepted`, whose packet it acknowledges at once (U4c's immediate acks; at one instant
      internal events come before application sends: L-c6), so the server's L1 link holds its first sample (about 0 at
      P0) before the room reads that client's RTT for any command. The runner logs per client the instant of that first
      sample and the tick of its first command run; a client whose first command ran before its first sample makes the
      leg VOID;
    - the bites: wallclock (a wall-clock read in the pump for one leg's sessions, bites build only) must give a
      mismatch; late-command (one replayed command delayed past its tick) must make the pair VOID, never a pass;
    - the headers list no mark, so the judge never reads these runs; results/u4r-precheck-30/<commit>/runs.txt and the
      comparer's output are committed, and the advisor reads them in U4r's review, as for U3d's self-test
      (advisor-u3d.md, U3D-JUDGE); nothing goes into marks.json;
    - -30 stays NOT-RUN until U6: never reported as -30's part, never in U6's folder.
  - The sizes for fixes.rs (§2), as series with SERIES.md rows; the WebSocket framing they assume, stated (a 1 B channel
    tag, catalogue :73; a 2 B header under 126 B and 4 B up to 65,535 B, RFC 6455 §5.2; +4 B for the client's mask;
    TLS 1.3's +22 B, RFC 8446 §5.2).
- **Evidence:** results/u4r/<commit>/verdict.json and runs.txt (the commit, the binaries' SHA-256s, one line per run);
  the recordings' SHA-256s and U4d2's step green on them (in check.mjs if the step stays within 5 minutes, else
  through ops/runs/det-u4.mjs at every merge that touches sim, proto, room or hash-wasm, on the advisor's ok).
- **Reviewers:** netcode-architect, test-engineer, engine-architect, rust-reviewer.
````

### The approved plan: §4.1, the public items (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 178-203 (sha256 1e631c55f4c4), as they are:_

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
| npt-client | a dependency on npt-link, for `Channel`; `StepOut`'s fields (`datagrams`, `reliable`, `next_due_ns`, `hashes`, `lost`, `wraps`); `ClientEndpoint`'s methods (`new`, `input`, `step`, `with_starts`, `with_lead`, `slot`, `newest_tick`, `last_run_command`, `wrong_channel`, `refused`, `stale`, `duplicates`); bites builds only, `BITES` (3 ids), `grid_with_bite` and `ClientEndpoint::with_bite` | as above; u4s-carries.md carry 30: arch F1; plan.md:486 (`StepOut`); carries 24, 61 (b) and (c); U4E-Q2, U4E-Q7; -58's counter wraps; carry 74 (`with_lead`, set by U4r's lab); the advisor's U4e brief ruling, A5 (the bites-only constructor); the advisor's U4e round-1 ruling, R2 (A5) |
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP` and `buffer_depth_ticks`; `ClientCounters` (the type `ClientSlot::counters` returns, its fields the per-client counts); `Room`'s methods (`new`, `join`, `leave`, `receive`, `pump_report`, `tick`, `take_datagrams`, `take_control` handing out each reliable message's bytes, `client`, `last_tick`, `sim`, `roster`, `script`, `tick_input`, `reports`, `event_cap`, `with_event_cap`, `with_event_start`, `with_hashes`, `with_cover`); `ClientSlot`'s getters (`slot`, `counters`, `server_hash`, `burst_eviction`, `baseline_age`, `groups`, `bytes`, `budget`, `last_acked`, `own_section`, `overflow_resync`, `wraps`, `rewinds_refused`, `queue`, `headroom`, `buffer_depth_ticks`, `kicked`); `Roster::live` and `is_live`; bites builds only: `BITES` and `Room::with_bite` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling); §4.2 (`script`, `reports`), §4.5 (`burst_eviction`, the window's pair), §4.9 (`with_cover`), §4.11 (`with_bite`); U4E-Q6, U4E-Q7, U4E-Q11, U4E-Q12; carries 23, 40, 54, 56, 57 and 61 (a); plan.md:508-509 (`take_control`: encoding stays on the tick); the advisor's U4e round-1 ruling, R2 (A1, A4, A5) |
| npt-server | `Decoder::pump` (the I/O step over `&mut dyn Link`, injected `now`, the ping sender, per-client RTT and bytes sent); `Schedule`, `Due`; `Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; `Decoder`'s methods (`new` taking an `RttSource`, `counters`, `is_closed`, `accept`, `reject`, `take_sent_bytes`, `rtt_ns`); `RttSource` (`Link`, `Pings`); `ConnLimits::new`, `input_refill_per_second`, `hold_cap_ns`; `Counters`' fields; the modules private, each item at the crate root; bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling; §4.9 (`RttSource`: the RTT by the session's kind), §4.10 (`Counters`, U4f's -21 series); carry 39's 30 and 60 Hz pins; the advisor's U4e round-1 ruling, R2 (A2, A5) |
| npt-l1 | `L1Net::next_event_ns` | C1' makes L1 event-driven |
| npt-bots (library) | `Behaviour` (the brawl, the open-loop scripts, the target-course shooter, the cheats, the never-acker), clock-injected | E12; U4-SERVER |
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` (`hash` with its modes: the two native legs, `compare`, `vectors` and `bites`); in the library, `HashCompare` (its constructor `of` and its variants `Equal`, `Empty` and `Mismatch`) and `TickHash` (the per-tick entry `of` takes); in the run header, the param `fate_log_v1_datagram_sha256` | plan.md:590-591; the advisor's condition 2 (`HashCompare` is U4r's -30 comparer, so it lives in the library and reads no file); §4.6's "0 compared fails" (`Empty`, as npt-judge's `eval::Outcome::Empty`); carry 66 (the v1 datagram hash, for §5 U4c's L-c8); the advisor's U4d2 round-1 ruling (EA-7) |
| npt-log | `TickRecord.hs`, with `TickRecord::set_hashes` and `TickRecord::hashes` | marks/README.md:147-148 (since U4j, README §1's `hs`) |
| npt-judge | `marks::Mark::ws_bound_line`, `marks::Check::ws_bound` and `marks::BoundNumber` (PM-0001-18's WebSocket bound, its line held verbatim); `marks::Rule::WsPractice` (the `ws-practice` rule); `eval::Outcome::Empty`, `eval::Outcome::fails` and `figures::Lack::Empty` (0 compared fails, never passes) | Q20 and the advisor's ws-docs R3-R12 (-18's split, `ws-practice`); the advisor's U4j FX-6 (g) (A19); §4.6's "0 compared fails" |
| npt-judge | nothing for -30's pre-check: the `PRE-` id, marks/prechecks.json and the `"kind": "precheck"` rule are dropped (the advisor's condition 2) | Q10 (T2); the advisor's condition 2 |
| a new crate | `npt-hash-wasm` (cdylib): its seven `#[unsafe(no_mangle)] pub extern "C"` exports `begin`, `feed`, `step`, `hash_lo`, `hash_hi`, `dump_len` and `dump_word`, integer-only (u32), every const private, beside wasm-ld's `memory`; its Node loader crates/hash-wasm/loader.mjs, exporting `HASH_WASM_EXPORTS` (the export list's one home, which check.mjs imports), `LINKER_EXPORTS`, `LOADER_EXPORTS`, `BEGIN`, `STEP`, `moduleProblems`, `exportProblems`, `instantiate`, `forEachChunk`, `replay`, `parse` and `main` | U4-WASM (B); plan.md §4.2's third `unsafe` exception (ADR 0002:264-265); U7 splits the loader for the browser (C-U7-2); the advisor's U4d2 round-1 ruling (EA-7) |
| not built at U4 | `BlockWriter` (U7); `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, `AddrTable`, the handshake cap (U6) | §6 |
````

### The approved plan: §4.4, the wire (the counters' starts) (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 267-290 (sha256 1e631c55f4c4), as they are:_

````text
### 4.4 The wire
- Framing, channels and limits as the catalogue gives them (:61-78; rows :87-101). Every decoder checks the size first,
  returns an error and never panics (ADR 0001:446).
- **The grid** (`npt_sim::grid`): positions at 1 cm, yaw in 16 bits, pitch in 10 bits over ±90° (net F37, F55),
  velocity as U4a's table states; round to nearest with one stated tie rule, in integers after one scale multiply. The
  sim's own-copy snap, the encoder and the client all call it. Bites on it are chosen through a grid config value the
  caller passes, never a static (§4.11).
- **npt-proto's `Quant`** packs grid integers into their widths (position 16/16/12 bits: ±327.68 m, 0 to 40.95 m). The
  `EntityRecord` ↔ wire-field mapping lives here, one home for the room's encode and the client's decode, with a round
  trip test (A18).
- The `Input` field table is stated by U4a and read by the limit test (net F116's proposal: about 23 B whole; a repeat
  as change mask + tick delta + changed fields, never larger than the whole command; 5 + 23 + 7 x 19 = 161 B at the
  redundancy cap, under 166 B; the aim at 1 cm in 44 bits, under ADR 0001:226-227's 80).
- The snapshot header: the full 32-bit server tick; the baseline as its 16-bit age in ticks (0: coded against the
  per-type defaults); the newest command received and the last command run as 16-bit command sequences, each with a
  known flag (ADR 0001:228-230 acknowledges two commands; a sequence names the command, while the tick it ran at can
  differ from its stamp; net F112 proposed them; the advisor's W1-N2 ruling); the group index **and the group count**
  (so the client knows when a tick is complete: §4.6).
- `Wrap`: one wrap-aware compare per width, called with its counter's name; every counter takes an injectable start
  value (-58).
- **Reading (A26):** plan §4.4's server column lists "trailing bytes and unknown fields; the wrong channel". Those
  checks run in npt-server's `Decoder::pump`, which counts them; they are implemented as npt-proto decode errors
  (`TrailingBytes`, `WrongChannel`), and only the pump calls the decoder on client input. Their bite sites are in
  proto's decode.
````

### The approved plan: §4.5, baselines, groups and the window (-18's `burst_eviction`) (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 292-304 (sha256 1e631c55f4c4), as they are:_

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

### The approved plan: §4.6, the client-held hash (-57's pairing) (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 306-328 (sha256 1e631c55f4c4), as they are:_

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

### The approved plan: §4.7, events (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 330-336 (sha256 1e631c55f4c4), as they are:_

````text
### 4.7 Events (Q14)
- The per-client event queue is bounded (cap as data). On overflow the room forces one resync (the FullState clears the
  client's unacknowledged events: catalogue Q8, :282). If that FullState is not acknowledged within
  `ticks_for_ms(2_000)` (60 ticks at 30 Hz), or the queue overflows again inside it, the room sends Kick(overflow) and
  drops the client (net F119). Event numbers are 16-bit on the wire, rebuilt to 32 bits by `Wrap`.
- The cap: the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), recorded. At
  about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every burst.
````

### The approved plan: §4.9, lag compensation (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 343-358 (sha256 1e631c55f4c4), as they are:_

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

### The approved plan: §4.11, determinism and hygiene outside npt-sim (the twin test, the bite dispatch, the clippy.toml bans) (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 392-407 (sha256 1e631c55f4c4), as they are:_

````text
### 4.11 Determinism and hygiene outside npt-sim (Q9)
- The HashMap and HashSet type bans join room's and client's clippy.toml; new clippy.toml files for npt-server and
  npt-lab, and for npt-bots (all three with the same HashMap and HashSet type bans, by Q9 (a), and with the clock ban,
  naming `std::time::Instant`, `std::time::SystemTime`, `tokio::time::Instant` and their `now`: A14). npt-bots' file is
  crate-wide; U6's runtime half and its npt-net glue then carry reviewed per-item allows, listed in U6's plan.
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

### The approved plan: §4.12, logs, the contract test and the stress-scene precondition (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 409-425 (sha256 1e631c55f4c4), as they are:_

````text
### 4.12 Logs, the contract test and the stress-scene precondition
- On L1 one npt-lab process writes one log per process value the checks select on (`room`, `client`, `lab`), each with
  its own header (marks/README.md:9; marks F28).
- **The contract test** (a short scenario per suite) shows that every check of every U4 L1 part resolves (no gap),
  every written name has its SERIES.md row, and **the validity inputs exist (T18):** one `admit` event per client (so
  the roster rule, README:171, is not vacuous), a footer per log, and `tick.v.late_count` wherever `late-zero` applies.
  It also resolves PM-0001-2's checks on every SS suite (T1).
- **Which marks gain the scene, the clients and `pre:PM-0001-2` (Q22, T1):**

| Mark | U4 L1 parts | Select gains | `pre:PM-0001-2` | Why |
|---|---|---|---|---|
| -17 | P0, P2, P3, B1 | scene SS, clients 48 | yes | the codec under the real load; shots hold (§4.13) |
| -18 | B1 | scene SS, clients 48 | yes | B1 under option A loses about 1.2 % of shots, inside the band (§4.13) |
| -20 | P3, A and B | scene SS, clients 48 | yes | redundancy repairs P3's losses |
| -21 | 8 suites | scene SS, clients 48 | yes | at most about 9 shots per second lost to refusals (§4.13) |
| -22 | never-ack | scene SS, clients 48 | **no** | the drop leaves 47 clients by design, which PM-0001-2's "a client on every soldier" refuses (the slot count of `everyone_in_view` changes too: figures.rs:26-36). README §4 (:193-197) puts the precondition on -22, so U4j writes this exception and its reason into README §4's sentence; -22's runs still log PM-0001-2's series (shots, projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition 4; a reading of ADR 0001:1589, for the readings PR: §10) |
| -14, -16, -19, -23 to -25, -27 | none at U4 | unchanged | unchanged | no U4 run feeds them; their scenes belong to the R run plans |
````

### The approved plan: §4.13, scenes (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 427-451 (sha256 1e631c55f4c4), as they are:_

````text
### 4.13 Scenes (data, recorded in each header)
- **SS, the stress scene,** with net F113's numbers: 48 soldiers in 3 x 16 in one cluster in the centre field; 15 big
  vehicles (the 4 of at least 20 parts in the fight); 300 live projectiles as trajectory records; 384 effect rows (8 per
  soldier) in the state and the hash; doors giving one portal change per second near the fight; one destructible-wall
  stand-in; the stand-in weapon's interval in integer sub-tick units for 700 shots per second with deaths counted;
  DM_Tri's extent (LEGACY_INVENTORY.md:153); the 4 very large units loop inside the fight and the other 11 elsewhere,
  and the stand-in projectiles start inside it; parts are recorded relative to their hull; effect rows record their
  end tick; a hit takes 1 of 1,000 health and SS and ES script no reloads, with a magazine that covers an S30 plus 5
  minutes at one soldier's rate (the advisor's W1-U4D ruling). A soldier counts in `soldiers_count` whether alive or
  awaiting respawn (one per slot), so deaths never drop it below 48.
- **The shot band under each precondition suite (arithmetic, before any run):** 700 / (48 x 30) = 0.486 shots per
  command-tick.
  - B1 under option A (nothing runs during a burst: OQ-0001-6 (a), ADR 0001:1156-1162): each 1 s burst loses about 22
    of its 30 ticks of commands (the redundancy cap recovers 8), about 0.73 s of shots; 10 bursts in 600 s give a mean of
    about 700 x (1 - 7.3 / 600) = 691.5 ≥ 665. Under option B the server repeats the last command on a miss, so about 700.
  - -21's forged suite: 11 forging clients, one kind each, 1,000 forged commands each; if every refusal costs that
    tick's shot, about 11 x 1,000 x 0.486 = 5,350 shots over 600 s, about 8.9 a second: a mean of about 691.
  - Any one client silent for a whole run: 700 x 47 / 48 = 685.4 ≥ 665. No suite can raise shots above 735: the
    command cap and the sim's cooldown bound each soldier's rate.
  U4r's contract test shows the band per suite on its short runs; a suite outside it goes to the advisor before its
  marks count.
- **S1's stand-ins on L1 (A17, Q26):** the 1 ms burn and the 2.80 ms top-up are off on L1 (no L1 mark reads time: ADR
  0001:1466-1467), recorded in `params.stand_ins`; the 384 effect rows stay, since they are state.
- ES, CO and TC as ADR 0001:1470-1482 gives them; TC's static targets are thin plates (at most 2 cm thick) on the
  ground, at 2.5 m and at 4 m (Q17; net F109).
````

### The approved plan: §4.14, bites that need a site or a run setting (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 453-476 (sha256 1e631c55f4c4), as they are:_

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

### The approved plan: §4.15, recordings (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 478-504 (sha256 1e631c55f4c4), as they are:_

````text
### 4.15 Recordings (A1, A12, T9; Q4)
- **Fields:** a header (scene, seed, TICK_HZ, pool sizes, HitHistory's capacity (nit b)); then per tick a
  `TickInput`: per soldier one `Command` (slot 1 B, tick 4, sequence 2, buttons 2, press counters 4, move 2 (the advisor's U4s ruling, condition 1 (a)), aim in cm 12, view tick 4 and fraction 1) and its
  `Rewind` (present 1, tick_a 4, fraction 1, cover 1); spawns, teleports and scripted attribute changes when they occur.
  Fixed-width little-endian, no `usize` (plan.md:527-529).
- **Size:** 39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers before spawns and the
  tick's own header (the advisor's U4s ruling, condition 1 (a)); about 16.8 MB per 9,000 ticks and 50.5 MB for three,
  commands only (my arithmetic; U4d states the measured size). Fed to Wasm as 32-bit words: about 4.2 million calls
  each (the advisor's F1 ruling).
- **Both uses:** the sim-only replay steps npt-sim with each `TickInput` and the ring the driver rebuilds; -30's
  pre-check (a recorded npt-lab test, never judged: the advisor's condition 2) takes the SS P0 recording below (the
  advisor's condition 3) and re-encodes its commands as `Input` (stamped ticks, view times, sequences and press
  counters from the recording; the ack from what that leg's replay client decodes live) and lets the room compute its
  own Rewinds.
- **One driver:** `npt_sim::replay::Replayer` (feed words, step, per-tick hash and dump). npt-lab's `hash` subcommand
  runs it natively; npt-hash-wasm's exports wrap it; D-2 calls it in a test.
- **Sources:** U4d's three synthetic recordings (open-loop commands, Rewinds drawn inside the ring) from a seeded
  generator in npt-sim's library (`Recording::synthetic`), since U4d2's step regenerates them from the manifest,
  needing no room; U4r's two runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2) and U4h's TC B1 seed 3,
  made with its TC shooter (the advisor's U4r brief check, U4R-Q9), which hold the room's real Rewinds; and a fourth
  runner recording, **SS P0 seed 4** (`scenario --record`), the -30 pre-check's source (the advisor's condition 3).
  Replayed at P0, the other three's view times sit about one RTT older than P0's estimate, so the room would replace
  them on both legs, each from its own RTT; a recording made at P0 keeps them inside the tolerance. The determinism step
  keeps the runner recordings (two at U4r, the third at U4h); SS P0 seed 4 serves the pre-check only.
- **Where they live:** regenerated at check time under .gate/ from a committed recordings manifest (scene, seed,
  script, generator version) with each file's SHA-256; a different hash fails the step and names the recording.
  Never committed as files.
````

### The approved plan: §7.1, Mustafa's shared-target rule (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 910-922 (sha256 1e631c55f4c4), as they are:_

````text
### 7.1 Mustafa's shared-target rule (2026-11-09; checkpoint :404-413; as u3d-plan.md §7.1)
1. Every cargo call in each builder's worktree and each bite's scratch copy runs with
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`; checked once after the first build.
2. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +`.
3. Each run's test count matches the brief's delta; `--list` names the new tests; a bite run shows its own test
   failing by name; a mismatch means another copy's artifact ran: touch, rebuild, never report that run.
4. After the last bite, touch and rebuild before any further claim.
5. Cargo's own lock serializes two copies' builds: expected.
6. The main folder keeps its own cache (.gate/netcode-proof-test/target): the merge checks, the gate and every judged
   L1 run happen there, with `CARGO_TARGET_DIR` unset.
7. No copy cleans the shared folder.
8. Every brief names the folder, items 2 and 3, and .claude/rules/netcode.md as binding on proto, room, client and
   server code.
````

### The approved plan: §7.2, who builds, and how (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 924-932 (sha256 1e631c55f4c4), as they are:_

````text
### 7.2 Who builds, and how
- Each sub-unit: one builder (general-purpose, opus, effort xhigh) in an isolated worktree, or, for U4s, U4b and U4j,
  the main session or one builder in a free lane (the advisor's U4s ruling, condition 1 (c)); the bites planted by a
  scratch script under §7.1; merged by the main session, one at a time.
- Judged L1 runs: the main session, in the main folder, on release builds, one set at a time, in the background;
  raw logs under .gate/, verdicts and runs.txt committed under results/.
- **Public items (A19):** each report lists the pub items it adds, by crate, against §4.1; engine-architect checks them
  in review. check.mjs does not check narrow surfaces (check.mjs:1134-1135), so review is the check.
- One review round per sub-unit on its final diff, then a short report to the advisor; U4's report folds them.
````

### The proof test's plan: §4.7, npt-lab's public items, RQ8's order and the bites line (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 617-631 (sha256 3ca4e12a39c9), as they are:_

````text
- **npt-lab:** `SimClock` (integer ns), `L1Scenario`, `ReplayX86`, `HashCompare` (netcode NC2-F9).
  - **Each simulated instant, in this order:**
    1. packets due, ordered by arrival time and then fate-log index, never delivered in the instant they were sent
       (P0 included);
    2. the server's I/O step: decoding, socket checks, ping answers;
    3. the server tick if due: tick k is due at base + ⌊k × 10⁹ / TICK_HZ⌋ ns;
    4. the client frames that are due (60 Hz, seeded phase);
    5. each client whose `next_due_ns` has come, in slot order.
  - **Each client's own host clock:** offset + rate × SimClock, both seeded; rate 1.5 for -21's cheat.
  - **L1's RTT** is an RFC 9002-style smoothed estimate, not the simulator's true value.
  - **Packet fates** are drawn per (seed, link, direction, packet index), so both command-timing options see the same
    losses.
- **web:** the files in E7; `wdio/*.spec.ts`.
- **Bites:** compiled only into the `bites` feature build, and chosen at run time by `--bite <id>`. The measured
  builds never contain them. Each run's header records the bite id, the enabled features and the binary's SHA-256.
````

### The proof test's plan: §4.2, narrow public surfaces (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 454-454 (sha256 3ca4e12a39c9), as they are:_

````text
- **Narrow public surfaces:** each crate's public items are those in §4.7; everything else is `pub(crate)`.
````

### marks.json: PM-0001-2, the load is the stress scene (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 151-624 (sha256 9952bcda3cb8), as they are:_

````text
    {
      "id": "PM-0001-2",
      "title": "The load is the stress scene",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "4665ed574038843c9ebb25488b8dfdd2aace823d6a0c55d48b2e11c0e66a86a3"
      },
      "verdict": "pass",
      "where": [
        "R1",
        "R2",
        "R3",
        "R4",
        "R5",
        "R6",
        "R7"
      ],
      "parts": [
        {
          "name": "SS runs",
          "select": {
            "scene": "SS",
            "clients": 48
          },
          "min": 3
        },
        {
          "name": "ES runs",
          "select": {
            "scene": "ES",
            "clients": 48
          },
          "min": 3
        },
        {
          "name": "SS N=24 (PM-0001-10's scale step)",
          "select": {
            "scene": "SS",
            "clients": 24
          },
          "min": 3
        },
        {
          "name": "SS N=36 (PM-0001-10's scale step)",
          "select": {
            "scene": "SS",
            "clients": 36
          },
          "min": 3
        }
      ],
      "validity": [],
      "checks": [
        {
          "name": "48 soldiers on every measured tick",
          "series": "tick.v.soldiers_count",
          "stat": "min",
          "op": ">=",
          "value": 48,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs",
            "ES runs"
          ]
        },
        {
          "name": "three teams of 16",
          "series": "tick.v.smallest_team_count",
          "stat": "min",
          "op": ">=",
          "value": 16,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "a connected snapshot-receiving client on every soldier",
          "series": "tick.v.snapshot_clients_count",
          "stat": "min",
          "op": ">=",
          "value": 48,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs",
            "ES runs"
          ]
        },
        {
          "name": "15 big vehicles",
          "series": "tick.v.big_vehicles_count",
          "stat": "min",
          "op": ">=",
          "value": 15,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs",
            "ES runs"
          ]
        },
        {
          "name": "4 very large units of at least 20 parts",
          "series": "tick.v.very_large_units_count",
          "stat": "min",
          "op": ">=",
          "value": 4,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "300 live projectiles",
          "series": "tick.v.projectiles_count",
          "stat": "min",
          "op": ">=",
          "value": 300,
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "700 shots per second, run mean at least -5 %",
          "series": "second.v.shots_count",
          "stat": "mean",
          "op": ">=",
          "value": "665",
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "700 shots per second, run mean at most +5 %",
          "series": "second.v.shots_count",
          "stat": "mean",
          "op": "<=",
          "value": "735",
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "portal changes 1 per second, mean at least -5 % (proposed band)",
          "series": "second.v.portal_changes_count",
          "stat": "mean",
          "op": ">=",
          "value": "0.95",
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "portal changes 1 per second, mean at most +5 % (proposed band)",
          "series": "second.v.portal_changes_count",
          "stat": "mean",
          "op": "<=",
          "value": "1.05",
          "from": {
            "process": "room"
          },
          "parts": [
            "SS runs"
          ]
        },
        {
          "name": "everyone in view on at least 99 % of each client's snapshots",
          "figure": "everyone_in_view_min_share",
          "op": ">=",
          "value": "0.99",
          "parts": [
            "SS runs",
            "ES runs"
          ]
        },
        {
          "name": "ES: 72 shots per second, mean at least -5 %",
          "series": "second.v.shots_count",
          "stat": "mean",
          "op": ">=",
          "value": "68.4",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 72 shots per second, mean at most +5 %",
          "series": "second.v.shots_count",
          "stat": "mean",
          "op": "<=",
          "value": "75.6",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 10 reliable events per second, mean at least -5 %",
          "series": "second.v.reliable_events_count",
          "stat": "mean",
          "op": ">=",
          "value": "9.5",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 10 reliable events per second, mean at most +5 %",
          "series": "second.v.reliable_events_count",
          "stat": "mean",
          "op": "<=",
          "value": "10.5",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 32 live projectiles, mean at least -5 %",
          "series": "tick.v.projectiles_count",
          "stat": "mean",
          "op": ">=",
          "value": "30.4",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 32 live projectiles, mean at most +5 %",
          "series": "tick.v.projectiles_count",
          "stat": "mean",
          "op": "<=",
          "value": "33.6",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 0.44 spawns per player per second (21.12/s), mean at least -5 %",
          "series": "second.v.projectile_spawns_count",
          "stat": "mean",
          "op": ">=",
          "value": "20.064",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "ES: 0.44 spawns per player per second (21.12/s), mean at most +5 %",
          "series": "second.v.projectile_spawns_count",
          "stat": "mean",
          "op": "<=",
          "value": "22.176",
          "from": {
            "process": "room"
          },
          "parts": [
            "ES runs"
          ]
        },
        {
          "name": "N=24: 24 soldiers on every measured tick",
          "series": "tick.v.soldiers_count",
          "stat": "min",
          "op": ">=",
          "value": 24,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=24: three teams of 8",
          "series": "tick.v.smallest_team_count",
          "stat": "min",
          "op": ">=",
          "value": 8,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=24: a connected snapshot-receiving client on every soldier",
          "series": "tick.v.snapshot_clients_count",
          "stat": "min",
          "op": ">=",
          "value": 24,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=24: 15 big vehicles (fixed)",
          "series": "tick.v.big_vehicles_count",
          "stat": "min",
          "op": ">=",
          "value": 15,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=24: 4 very large units of at least 20 parts (fixed)",
          "series": "tick.v.very_large_units_count",
          "stat": "min",
          "op": ">=",
          "value": 4,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=24: 300 live projectiles (fixed)",
          "series": "tick.v.projectiles_count",
          "stat": "min",
          "op": ">=",
          "value": 300,
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: 36 soldiers on every measured tick",
          "series": "tick.v.soldiers_count",
          "stat": "min",
          "op": ">=",
          "value": 36,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: three teams of 12",
          "series": "tick.v.smallest_team_count",
          "stat": "min",
          "op": ">=",
          "value": 12,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: a connected snapshot-receiving client on every soldier",
          "series": "tick.v.snapshot_clients_count",
          "stat": "min",
          "op": ">=",
          "value": 36,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: 15 big vehicles (fixed)",
          "series": "tick.v.big_vehicles_count",
          "stat": "min",
          "op": ">=",
          "value": 15,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: 4 very large units of at least 20 parts (fixed)",
          "series": "tick.v.very_large_units_count",
          "stat": "min",
          "op": ">=",
          "value": 4,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: 300 live projectiles (fixed)",
          "series": "tick.v.projectiles_count",
          "stat": "min",
          "op": ">=",
          "value": 300,
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        }
      ],
      "report": [
        {
          "name": "effects per soldier, max",
          "series": "tick.v.effects_per_soldier_max_count",
          "stat": "max",
          "from": {
            "process": "room"
          }
        },
        {
          "name": "stand-in entity changes per second, mean",
          "series": "second.v.stand_in_changes_count",
          "stat": "mean",
          "from": {
            "process": "room"
          }
        },
        {
          "name": "stand-in entity spawns per second, mean",
          "series": "second.v.stand_in_spawns_count",
          "stat": "mean",
          "from": {
            "process": "room"
          }
        },
        {
          "name": "N=24: shots per second, mean (reported, not checked)",
          "series": "second.v.shots_count",
          "stat": "mean",
          "parts": [
            "SS N=24 (PM-0001-10's scale step)"
          ]
        },
        {
          "name": "N=36: shots per second, mean (reported, not checked)",
          "series": "second.v.shots_count",
          "stat": "mean",
          "parts": [
            "SS N=36 (PM-0001-10's scale step)"
          ]
        }
      ],
      "bites": [
        {
          "id": "PM-0001-2/fire-stops",
          "what": "a load script that stops firing after 10 minutes fails the 700 shots per second"
        }
      ],
      "stand_in": false,
      "arm_rule": false,
      "readings": [],
      "notes": "Runs: every run of -4 to -27, -39 and -63 lists it (the 60 Hz ones are outside). The stand-ins (ADR 0004's S1: 384 effect rows, the 1 ms burn, the top-up to 2.80 ms) are named in `header.params.stand_ins`; a list cannot be checked by a stat, so the judge reports it. The portal band of +-5 % is proposed (the row names no tolerance; the shots and ES rates have +-5 %). 'Everyone in view' is per client, so it is a figure, not a pooled share. The bite run must run past 10 minutes (an S30): in a 5-minute run the defect never shows. ES's 'all 15 vehicles in view' is in the same flag (`tick.s.everyone_in_view` reads soldiers in SS, soldiers and vehicles in ES)."
    },
````

### marks.json: PM-0001-17 (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 3844-4006 (sha256 9952bcda3cb8), as they are:_

````text
    {
      "id": "PM-0001-17",
      "title": "Every decoded snapshot is the server's state",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "3075f6384860068622268fc3efff47db2b8a29624d063882d72beab37eedffea"
      },
      "verdict": "pass",
      "where": [
        "C",
        "R3"
      ],
      "parts": [
        {
          "name": "L1 P0",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P0",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 P2",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P2",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 P3",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "P3",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L1 B1",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "B1",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L2 P0",
          "select": {
            "layer": "L2",
            "profile": "P0",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 P2",
          "select": {
            "layer": "L2",
            "profile": "P2",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 P3",
          "select": {
            "layer": "L2",
            "profile": "P3",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "L2 B1",
          "select": {
            "layer": "L2",
            "profile": "B1",
            "machine.role": "LG"
          },
          "min": 1
        },
        {
          "name": "every S30",
          "select": {
            "params.soak": "S30"
          },
          "min": 3
        }
      ],
      "validity": [],
      "checks": [
        {
          "name": "100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule)",
          "figure": "decoded_hash_mismatch_count",
          "op": "==",
          "value": 0
        },
        {
          "name": "setup: the S30 starts 1,800 ticks below 65,536, so it crosses the 16-bit wrap",
          "series": "header.params.tick_start",
          "stat": "max",
          "op": "==",
          "value": 63736,
          "parts": [
            "every S30"
          ],
          "setup": true
        }
      ],
      "report": [
        {
          "name": "aarch64-only mismatches (a cross-architecture finding; decides nothing)",
          "figure": "cross_arch_mismatch_count"
        },
        {
          "name": "decoded snapshots compared",
          "figure": "decoded_hash_compared_count"
        },
        {
          "name": "client-ticks left incomplete (a group lost, none refused: not compared, u4-build-plan.md §4.6)",
          "series": "tick.s.snap_incomplete",
          "stat": "sum",
          "window": "all"
        }
      ],
      "bites": [
        {
          "id": "PM-0001-17/tick16",
          "what": "a 16-bit tick in the snapshot header (fails at the wrap: the bite run must cross it)"
        },
        {
          "id": "PM-0001-17/client-rounding",
          "what": "a client quantiser that rounds differently from the server's"
        }
      ],
      "stand_in": false,
      "arm_rule": true,
      "readings": [
        "R-5",
        "R-8"
      ],
      "notes": "L1: 48 headless clients, 3 seeds per profile, 10 minutes each under P0, P2, P3 and B1, in the stress scene (its L1 parts select scene SS and 48 clients and carry pre:PM-0001-2: u4-build-plan.md §4.12, the advisor's Q22 ruling); L2: LG's 48 clients, 1 run per profile (R3, VS4; their scene is the R run plans'). The server logs each client's quantised hash per tick (`hs.server_hash`), the client its decoded hash under the tick it rebuilt (`hs.decoded_hash`: the sentinel 0 for a tick with a group it refused, null for a tick it could not complete, counted in `tick.s.snap_incomplete`: §4.6, the advisor's Q11 ruling); each LG client logs every datagram and stream byte it receives (kept on LG). Arm rule: an L2 mismatch counts only once those bytes, decoded again on x86-64 (npt-lab, in this container or on RS: R-8; `hs.replay_hash`, process replay), miss the server's hash too; a mismatch only on aarch64 is reported. The figure fails, never passes, on 0 compared snapshots; a client hash at a tick the server never logged counts as a mismatch."
    },
````

### marks.json: PM-0001-57 (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 4007-4105 (sha256 9952bcda3cb8), as they are:_

````text
    {
      "id": "PM-0001-57",
      "title": "Decoded values keep the protocol's precision",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "c4245c8ba4c4747e33b8f7294322062c5175944dc8abdbc3b4da4a33e2e39ea0"
      },
      "verdict": "pass",
      "where": [
        "C"
      ],
      "parts": [
        {
          "name": "L1 P0",
          "select": {
            "layer": "L1",
            "profile": "P0"
          },
          "min": 3
        },
        {
          "name": "L1 P2",
          "select": {
            "layer": "L1",
            "profile": "P2"
          },
          "min": 3
        },
        {
          "name": "L1 P3",
          "select": {
            "layer": "L1",
            "profile": "P3"
          },
          "min": 3
        },
        {
          "name": "L1 B1",
          "select": {
            "layer": "L1",
            "profile": "B1"
          },
          "min": 3
        }
      ],
      "validity": [],
      "checks": [
        {
          "name": "positions: max error at most 1 cm",
          "series": "tick.s.pos_err_um",
          "stat": "max",
          "window": "all",
          "op": "<=",
          "value": 10000
        },
        {
          "name": "yaw: max error at most 0.01 degree",
          "series": "tick.s.yaw_err_microdeg",
          "stat": "max",
          "window": "all",
          "op": "<=",
          "value": 10000
        },
        {
          "name": "pitch: max error at most one 0.18 degree step",
          "series": "tick.s.pitch_err_microdeg",
          "stat": "max",
          "window": "all",
          "op": "<=",
          "value": 180000
        }
      ],
      "report": [
        {
          "name": "positions: error p99",
          "series": "tick.s.pos_err_um",
          "stat": "p99",
          "window": "all"
        }
      ],
      "bites": [
        {
          "id": "PM-0001-57/pos-4cm",
          "what": "positions quantised to 4 cm (errors up to 2 cm)"
        },
        {
          "id": "PM-0001-57/yaw-12bit",
          "what": "yaw in 12 bits (0.088 degree steps)"
        },
        {
          "id": "PM-0001-57/pitch-8bit",
          "what": "pitch in 8 bits (0.70 degree steps)"
        }
      ],
      "stand_in": false,
      "arm_rule": false,
      "readings": [],
      "notes": "PM-0001-17's L1 runs: the server logs each sent entity's position, yaw and pitch before quantising, the client the values it decodes; npt-lab writes each value's error against the server's. Units `_um` and `_microdeg` are not in README §1's list (question). 0.18 degree is the row's literal; the 10-bit field's exact step depends on the pitch range (180 / 1024 = 0.176 degree if it spans 180 degrees)."
    },
````

### marks.json: PM-0001-18 (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 4261-4443 (sha256 9952bcda3cb8), as they are:_

````text
    {
      "id": "PM-0001-18",
      "title": "A loss burst heals",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "cdef35068e22dea771d6dfec13dfab0697b1953805c3f773ed276266c77e18f7"
      },
      "verdict": "pass",
      "where": [
        "C",
        "R3"
      ],
      "parts": [
        {
          "name": "L1 B1",
          "select": {
            "layer": "L1",
            "scene": "SS",
            "profile": "B1",
            "clients": 48
          },
          "min": 3,
          "validity": [
            "pre:PM-0001-2"
          ]
        },
        {
          "name": "L2 B1 wt",
          "select": {
            "layer": "L2",
            "profile": "B1",
            "machine.role": "LG",
            "params.transport": "wt"
          },
          "min": 1
        },
        {
          "name": "L2 B1 ws",
          "select": {
            "layer": "L2",
            "profile": "B1",
            "machine.role": "LG",
            "params.transport": "ws"
          },
          "min": 1,
          "validity": [
            "ws-practice:1.10"
          ]
        }
      ],
      "validity": [],
      "checks": [
        {
          "name": "WebTransport: every client decodes a fresh delta within 500 ms of the link coming back",
          "series": "event.burst_heal.after_link_ns",
          "stat": "max",
          "op": "<=",
          "value": 500000000,
          "parts": [
            "L1 B1",
            "L2 B1 wt"
          ]
        },
        {
          "name": "every client after every burst: 48 x 10 heal records",
          "series": "event.burst_heal.after_link_ns",
          "stat": "count",
          "op": ">=",
          "value": 480
        },
        {
          "name": "WebSocket: half the client-bursts heal within X x f (X provisional: bf9d248's first bound, 0.67 s)",
          "series": "event.burst_heal.after_link_ns",
          "stat": "p50",
          "op": "<=",
          "value": 670000000,
          "parts": [
            "L2 B1 ws"
          ],
          "ws_bound": "x"
        },
        {
          "name": "WebSocket: at most k client-bursts over Y x f (k and Y provisional: bf9d248's first bound, 5 over 2.5 s)",
          "series": "event.burst_heal.after_link_ns",
          "stat": "count",
          "where": {
            "op": ">",
            "value": 2500000000
          },
          "op": "<=",
          "value": 5,
          "parts": [
            "L2 B1 ws"
          ],
          "ws_bound": "y"
        },
        {
          "name": "WebSocket: no client-burst over Z x f (Z provisional: bf9d248's first bound, 6.9 s)",
          "series": "event.burst_heal.after_link_ns",
          "stat": "max",
          "op": "<=",
          "value": 6900000000,
          "parts": [
            "L2 B1 ws"
          ],
          "ws_bound": "z"
        }
      ],
      "report": [
        {
          "name": "heal time p50",
          "series": "event.burst_heal.after_link_ns",
          "stat": "p50"
        },
        {
          "name": "heal time p99",
          "series": "event.burst_heal.after_link_ns",
          "stat": "p99"
        },
        {
          "name": "WebSocket: heal time p99.9",
          "series": "event.burst_heal.after_link_ns",
          "stat": "p999",
          "parts": [
            "L2 B1 ws"
          ]
        },
        {
          "name": "WebSocket: client-bursts over 1 s",
          "series": "event.burst_heal.after_link_ns",
          "stat": "count",
          "where": {
            "op": ">",
            "value": 1000000000
          },
          "parts": [
            "L2 B1 ws"
          ]
        },
        {
          "name": "WebSocket: unsent plus unacknowledged bytes at a return, max (the server's per-return series)",
          "series": "event.burst_return.outstanding_bytes",
          "stat": "max",
          "parts": [
            "L2 B1 ws"
          ]
        },
        {
          "name": "WebSocket: the send budget at a return, max",
          "series": "event.burst_return.budget_bytes",
          "stat": "max",
          "parts": [
            "L2 B1 ws"
          ]
        },
        {
          "name": "WebSocket: smoothed RTT at a return, p50",
          "series": "event.burst_return.srtt_ns",
          "stat": "p50",
          "parts": [
            "L2 B1 ws"
          ]
        }
      ],
      "bites": [
        {
          "id": "PM-0001-18/no-fallback",
          "what": "a baseline that never falls back (deltas only against the last acknowledged snapshot); WebTransport's parts only (L1 B1, L2 B1 wt)"
        },
        {
          "id": "PM-0001-18/ws-budget-x2",
          "what": "a send budget twice the formula's (budget_scale 2): fails the WebSocket median check on the L2 B1 ws part, at the measured build's numbers in force"
        }
      ],
      "stand_in": false,
      "arm_rule": false,
      "ws_bound_line": "npt-ws-bound v1 {\"model\":\"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound\",\"commit\":\"bf9d24807106c4524d1aa0849409d219259c3943\",\"runs_count\":2000,\"client_bursts_count\":480,\"x_ns\":670000000,\"y_ns\":2500000000,\"k_count\":5,\"z_ns\":6900000000,\"m_m_ns\":654000000,\"band_low\":\"0.965\",\"band_high\":\"1.035\",\"tolerance\":\"1.10\",\"guard_share\":\"0.0000\"}",
      "readings": [
        "R-5",
        "R-12"
      ],
      "notes": "B1 with 48 clients, 10 bursts per run (proposed), each burst's start moved by a random offset of 0 to one tick period against the room's tick (the advisor's reading, 2026-11-07); L1 3 seeds in the stress scene (scene SS, 48 clients, pre:PM-0001-2: u4-build-plan.md §4.12, the advisor's Q22 ruling); L2 1 run per transport (R3, VS4), the WebSocket one through the accept path's delay wrapper, whose clients run Linux's TCP as the model's do. Each client writes one `burst_heal` event per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant (a fresh delta; deltas queued during the burst do not count); a client that never heals writes none, so the count catches it. WebTransport keeps the plain 500 ms limit. WebSocket (Mustafa's WS-BAR, 2026-11-07): the median at most X, at most k over Y, none over Z, from the run-structured bound for a correct build with fix 1 and one unjudged practice run of the same build (role practice, listing this mark, earlier, the same scene, profile and burst spacing). The judge reads that run's `ws_practice` record (README §2.5), recomputes s = M_p / M_m and f (max(1, s), or 1 for FAR BETTER: at the bound), and applies X, Y and Z times f, each rounded up to the next millisecond, k unmoved; checks (c) to (e) print the numbers in force with M_p, M_m, s and f beside them. The `ws-practice` rule makes the counted run INVALID unless the record is IN TOLERANCE, or names the advisor's chance ruling (R8 (v)), and its numbers are the judge's; one practice run counts per build (a second needs a ruling naming a found and fixed cause). The tolerance is 1.10 (Mustafa's 10 %), lowered only before the practice run by the bound's guard, never raised. PROVISIONAL: X 0.67 s, k 5 over Y 2.5 s and Z 6.9 s are bf9d248's first, single-burst bound (spikes/netcode-proof/results/u3-ws-fallback/bar-fix1-bf9d248.txt); the run-structured bound (fixes.rs, its own unit after U4r) replaces them, and the tolerance if its guard lowers it, with its committed machine-readable line, named here by path and commit, before R3's run plan. The bound's line is held verbatim in `ws_bound_line`; until fixes.rs's run-structured bound replaces it, its model says provisional, so no counted run can match it. Bites: no-fallback on WebTransport's parts only; ws-budget-x2 on the WebSocket part, judged at the measured build's numbers in force (a bites build has no practice run of its own). On L2 the time uses the client's server-clock estimate, whose error bound is PM-0001-47's. The row's last reported item, the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms, gets its series with its writer at U6; the client's fast reconnect over WebSocket is not measured here (OQ-0001-19)."
    },
````

### marks.json: PM-0001-30 (spikes/netcode-proof/marks/marks.json)
_`spikes/netcode-proof/marks/marks.json` lines 6511-6577 (sha256 9952bcda3cb8), as they are:_

````text
    {
      "id": "PM-0001-30",
      "title": "Both transports give the same server",
      "draft": {
        "file": "docs/adr/0001-multiplayer-netcode.md",
        "row_sha256": "b48098d8ad8839bbecfb599d391d6d974e4f4e1fe39b7717f1c84e1fa29f0dfe"
      },
      "verdict": "pass",
      "where": [
        "R3",
        "C"
      ],
      "parts": [
        {
          "name": "RS: 3 recordings x 2 transports",
          "select": {
            "params.suite": "both-transports",
            "profile": "P0",
            "machine.name": "RS"
          },
          "min": 6
        },
        {
          "name": "C: loopback, 3 recordings x 2 transports",
          "select": {
            "params.suite": "both-transports",
            "profile": "P0",
            "machine.role": "C"
          },
          "min": 6
        }
      ],
      "validity": [
        "late-zero"
      ],
      "checks": [
        {
          "name": "identical server state hashes on every tick (each run against its other-transport twin)",
          "figure": "twin_hash_mismatch_count",
          "op": "==",
          "value": 0
        }
      ],
      "report": [
        {
          "name": "input buffer depth set for P0 (ticks)",
          "series": "header.params.buffer_depth_ticks_count",
          "stat": "max"
        }
      ],
      "bites": [
        {
          "id": "PM-0001-30/wallclock",
          "what": "a wall-clock read on one transport's command path"
        },
        {
          "id": "PM-0001-30/late-command",
          "what": "the validity check itself: one command of a replay delayed past its tick must make that run invalid (counter above 0), never a pass",
          "expect": "invalid",
          "rule": "late-zero"
        }
      ],
      "stand_in": false,
      "arm_rule": false,
      "readings": [],
      "notes": "3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket at P0, the input buffer deep enough for P0 on both, recorded; each run names its other-transport run in `params.twin`, so all 6 runs are measured and all 6 must keep `late-zero`. R3 (VS5), and in this container on loopback (plan §6.2). The row's second bite, 'one command of a replay delayed past its tick must make that run invalid, never a pass', expects INVALID: under the contract an invalid bite run makes the mark INVALID, so it is not in `bites`; it belongs in npt-judge's fixtures (a run with `tick.v.late_count` above 0 must come out INVALID) (question)."
    },
````

### SERIES.md at `<base>`: the Series table's header, then `tick.v.ack_starved_headroom_count` and `tick.v.big_vehicles_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 19-22 (sha256 3a9673698897), as they are:_

````text
| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `tick.v.ack_starved_headroom_count` | count, signed | npt-room | event-queue cap minus queue length of the never-acking client (negative = over the cap) | -22 |
| `tick.v.big_vehicles_count` | count | npt-room | big vehicles live (stand-ins included) | -2 |
````

### SERIES.md at `<base>`: `tick.v.effects_per_soldier_max_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 31-31 (sha256 3a9673698897), as they are:_

````text
| `tick.v.effects_per_soldier_max_count` | count | npt-room | the most active effects any soldier has this tick | -2 |
````

### SERIES.md at `<base>`: `tick.v.projectiles_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 43-43 (sha256 3a9673698897), as they are:_

````text
| `tick.v.projectiles_count` | count | npt-room | live projectiles (trajectory records) | -2 |
````

### SERIES.md at `<base>`: `tick.v.smallest_team_count` to `tick.v.very_large_units_count` (lines 51-58)
_`spikes/netcode-proof/marks/SERIES.md` lines 51-58 (sha256 3a9673698897), as they are:_

````text
| `tick.v.smallest_team_count` | count | npt-room | soldiers in the smallest team this tick | -2 |
| `tick.v.snapshot_clients_count` | count | npt-room | soldiers with a connected, snapshot-receiving client | -2 |
| `tick.v.soldiers_count` | count | npt-room | soldiers in the sim this tick | -2 |
| `tick.v.start_jitter_ns` | ns | npt-server | actual minus scheduled start of the tick (S17), from the loop's monotonic clock | 0004-67 |
| `tick.v.tick_allocs_count` | count | npt-server | allocations on the tick thread this tick (counting GlobalAlloc) | 0006-20 |
| `tick.v.unsent_snaps_max_count` | count | npt-server | the most snapshots any client has waiting above the socket (not yet written to it, held by the room or its WebSocket library), sampled each tick | -31 |
| `tick.v.untraced_corrections_count` | count | npt-client | client: corrections above 1 mm this tick NOT traced to a late or lost command (PM-0001-41's count; read by -58 too) | -58, -41 |
| `tick.v.very_large_units_count` | count | npt-room | very large units of at least 20 parts live | -2 |
````

### SERIES.md at `<base>`: `tick.s.pitch_err_microdeg` to `tick.s.yaw_err_microdeg` (lines 66-73: -57's error series and the snapshot byte series among them)
_`spikes/netcode-proof/marks/SERIES.md` lines 66-73 (sha256 3a9673698897), as they are:_

````text
| `tick.s.pitch_err_microdeg` | µdeg (proposed, Q3) | npt-lab | L1: one per decoded pitch: error against the server's value before quantising | -57 |
| `tick.s.pos_err_um` | µm (proposed, Q3) | npt-lab | L1: one per decoded entity position: error against the server's value before quantising | -57 |
| `tick.s.relevant_starved_count` | count | npt-room | one per client per snapshot: relevant entities sent in none of this and the previous 3 snapshots | -8 |
| `tick.s.rewind_ns` | ns | npt-server | one per rewound shot: its query time against the candidates | -7 |
| `tick.s.snap_datagram_bytes` | bytes | npt-server | one per snapshot datagram sent: its payload as handed to send_datagram (the cap rule) | -12, -14 |
| `tick.s.snap_payload_bytes` | bytes | npt-room | one per client per snapshot: the encoded payload | -6 |
| `tick.s.snap_wire_bytes` | bytes | npt-server | one per client per snapshot interval: UDP payload + 28 B per datagram | -8 |
| `tick.s.yaw_err_microdeg` | µdeg (proposed, Q3) | npt-lab | L1: one per decoded yaw: error against the server's value before quantising | -57 |
````

### SERIES.md at `<base>`: `second.v.portal_changes_count` to `second.v.stand_in_spawns_count` (lines 87-97)
_`spikes/netcode-proof/marks/SERIES.md` lines 87-97 (sha256 3a9673698897), as they are:_

````text
| `second.v.portal_changes_count` | count | npt-room | portal changes (doors) this second | -2 |
| `second.v.proc_cpu_permille` | permille of one vCPU | npt-server | a room process's whole CPU time this second (density rows); not LG's `cpu_permille` | -27, 0006-13 |
| `second.v.projectile_spawns_count` | count | npt-room | projectiles spawned this second | -2 |
| `second.v.recv_over_max_count` | count | web | browser: datagrams received this second above its current maxDatagramSize | -14 |
| `second.v.reliable_events_count` | count | npt-room | reliable gameplay events emitted this second | -2 |
| `second.v.rewinds_count` | count | npt-room | rewound shots this second | -7 |
| `second.v.rss_bytes` | bytes | npt-server | the room process's resident memory, read each second | -15, -22, -25 |
| `second.v.shots_count` | count | npt-room | hitscan shots fired this second | -2 |
| `second.v.snap_datagrams_sent_count` | count | npt-server | snapshot datagrams sent this second (all clients) | -11 |
| `second.v.stand_in_changes_count` | count | npt-room | stand-in entity changes this second | -2 |
| `second.v.stand_in_spawns_count` | count | npt-room | stand-in entity spawns this second | -2 |
````

### SERIES.md at `<base>`: `event.burst_heal.after_link_ns`
_`spikes/netcode-proof/marks/SERIES.md` lines 134-134 (sha256 3a9673698897), as they are:_

````text
| `event.burst_heal.after_link_ns` | ns | npt-bots (L2), npt-lab (L1) | per client per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant (a fresh delta; V29, fixes.rs:786-787); fields `slot` and `burst` (below) | -18 |
````

### SERIES.md at `<base>`: `event.counter_wrap.tick_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 150-150 (sha256 3a9673698897), as they are:_

````text
| `event.counter_wrap.tick_count` | count (a tick number) | npt-lab | per counter that wrapped (`counter` names it): the tick of its wrap | -58 |
````

### SERIES.md at `<base>`: `event.snap_refused.tick_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 218-218 (sha256 3a9673698897), as they are:_

````text
| `event.snap_refused.tick_count` | count (a tick number) | npt-lab (from npt-room's per-client refusal counters) | per snapshot the room's encoder refused for a client (carry 54: that client's snapshot for that tick is skipped; `slot` names the client, `kind` the refusal: `out_of_range`, `too_many`, `too_large` or `not_canonical`): the tick it skipped | -17, -18 (reported) |
````

### SERIES.md at `<base>`: `header.params.buffer_depth_ticks_count`
_`spikes/netcode-proof/marks/SERIES.md` lines 240-240 (sha256 3a9673698897), as they are:_

````text
| `header.params.buffer_depth_ticks_count` | count | npt-room | -30's input buffer depth set for P0 | -30 |
````

### SERIES.md at `<base>`: `header.params.tick_start` and `.ticks_below_wrap`
_`spikes/netcode-proof/marks/SERIES.md` lines 252-253 (sha256 3a9673698897), as they are:_

````text
| `header.params.tick_start` | count (a tick number) | npt-server | the run's first tick (63,736 for an S30) | -17 |
| `header.params.ticks_below_wrap` | count | npt-lab | ticks the run starts below a multiple of 65,536 (-58: 30) | -58 |
````

### SERIES.md at `<base>`: U4j's table header, then `hs.server_hash` to `event.burst_heal.burst` (lines 315-323)
_`spikes/netcode-proof/marks/SERIES.md` lines 315-323 (sha256 3a9673698897), as they are:_

````text
| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `hs.server_hash` | 16 hex digits or null, per client slot | npt-room (the encoder's mirror, §4.6) | at tick t: the server's hash (`npt_proto::client_hash`) of what it meant that client to hold after decoding t; never 0; written only in the room's log (process `room`); null where it sent that client nothing for t | -17, -19, -58 (figures) |
| `hs.decoded_hash` | 16 hex digits or null, per client slot | npt-client (L1, through npt-lab's client log), npt-bots (L2), web (browser) | only in the clients' logs, process `client` on L1 and `bots` on L2, at the tick t the client rebuilt: its hash of the state it holds for t (its held floats re-gridded); the sentinel 0 for a tick with a received group it could not decode; null for none (a group lost, none refused) | -17, -19, -58 (figures) |
| `hs.replay_hash` | 16 hex digits or null, per client slot | npt-lab (`replay`, process replay: the x86-64 re-decode of an L2 client's logged bytes, R-8) | only in a `replay` log on x86_64: the same as `decoded_hash`, decoded again natively; read for the Arm rule | -17, -19 (figures) |
| `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot (the run's `clients` entries, each 0 or 1), at the tick it resolves: 1 when a group of that tick was lost and none refused, or the client dropped one as stale (README §1's `decoded_hash`), so the client logs no hash for it (§4.6), a tick none of whose groups arrived included (known from a later header's tick); with `hs.decoded_hash` it decides README §1's coverage rule (`decoded_hash_mismatch_count`) | -17, -58, -19 (the coverage rule; -17 reports it) |
| `event.burst_eviction.evicted` | flag | npt-room | per client per burst (fields `slot`, `burst`): 1 when the client's last acknowledged baseline had left the AckRing (ages 0 to 31) at the first tick after the burst, else 0; shows whether -18's no-fallback bite could bite (§4.5) | -18 (recorded) |
| `event.clamp_report.x_um`, `.y_um`, `.z_um` | µm | npt-room (from npt-sim's `reports()`) | per clamp report (fields `slot`, `seq`): the aim point the sim clamped to the weapon's reach, its grid cm x 10,000; read from `reports()`, never from the hashed event list; the clamp twin sends exactly these points (§4.10, the advisor's condition 1) | -21 (the clamp twin's input) |
| `event.burst_heal.burst` | identifier (0 to 9) | npt-bots (L2), npt-lab (L1) | the burst's index in the run, beside `slot` on each heal record | -18 |
````

### SERIES.md at `<base>`: `header.params.burst_spacing_ns`
_`spikes/netcode-proof/marks/SERIES.md` lines 330-330 (sha256 3a9673698897), as they are:_

````text
| `header.params.burst_spacing_ns` | ns | npt-lab (L1), the run script (L2) | B1's scheduled interval between burst starts, before each burst's offset; a practice run and its counted run share it | -18 (the rule) |
````

### SERIES.md at `<base>`: `tick.s.everyone_in_view` (series read only by figures)
_`spikes/netcode-proof/marks/SERIES.md` lines 337-337 (sha256 3a9673698897), as they are:_

````text
| `tick.s.everyone_in_view` | flag | npt-room | one per client per snapshot: 1 when its set holds all 47 other soldiers (ES: and all 15 vehicles) | everyone_in_view_min_share |
````

### SERIES.md at `<base>`: `tick.v.late_count` (names the judge's validity rules read)
_`spikes/netcode-proof/marks/SERIES.md` lines 365-365 (sha256 3a9673698897), as they are:_

````text
| `tick.v.late_count` | count | npt-room | late-zero: sums to 0 | not read by any check (-30 names it in its notes) |
````

### SERIES.md at `<base>`: the header-param table's header, then `soak` to `suite` (lines 370-376)
_`spikes/netcode-proof/marks/SERIES.md` lines 370-376 (sha256 3a9673698897), as they are:_

````text
| Param | Values | Used by |
|---|---|---|
| `soak` | S30, S60 or none | -4 to -16, -17, PM-0004-67 (selects) |
| `sim` | stress or stand-in | -4's STAND-IN |
| `stand_ins` | list of names | -2 (reported in the detail: Q16) |
| `cmd_timing` | A or B (OQ-0001-6) | -20, -21, -42, -48, -49 |
| `suite` | the run's test suite (wrap, joins, forged, clamp, fast-clock, lag-switch, never-ack, throttle, bottleneck, flood, origin, conn-panic, tick-panic, wt-attempts, fallback, fallback-443, both-transports, ws-stall, one-build, rtt, rtt-fixed-100, fuzz, deps, mds, datagram-timing, one-shots, brawl, own-change, input-latency, replay, extrapolation, delay-adapt, events, clock, rewind-cheat, aim-height, feedback, cues, checklist, throws, beside-vehicle, record, stalls, ping, firewall, fw-probe, rule-list, metrics-probe, egress, bill, logging-cost, logging-alloc, soak-8, cert-14d, cert-15d, rotation; and precheck-30, the -30 pre-check's recorded npt-lab test, whose headers list no mark, so no part selects it) | selects throughout |
````

### SERIES.md at `<base>`: the header param `twin`
_`spikes/netcode-proof/marks/SERIES.md` lines 389-389 (sha256 3a9673698897), as they are:_

````text
| `twin` | the run name of its twin | -3, -21, -30, PM-0006-20 |
````

### SERIES.md at `<base>`: the header params `reorder_step_permille` to `event_queue_cap_count` (lines 400-405)
_`spikes/netcode-proof/marks/SERIES.md` lines 400-405 (sha256 3a9673698897), as they are:_

````text
| `reorder_step_permille` | the reorder step's draw share (part 13's 10, on in every run), recorded beside the profile | -3 (recorded) |
| `probe_rate_pps` | the probe's datagrams per second per direction (10,000) | -3 (recorded) |
| `probe_sent_count` | the probe's datagrams sent per direction, bursts included (1,000,000, or 1,010,000 at B1) | -3 (recorded) |
| `burst_starts_ns` | B1's burst starts on the run's clock (a list; each burst lasts 1 s) | -3 (recorded) |
| `padding_target_ns` | ns | padding rule |
| `event_queue_cap_count` | count | -22 (reported) |
````

### The message catalogue: the class vocabulary (docs/design/netcode-proof-messages.md)
_`docs/design/netcode-proof-messages.md` lines 35-50 (sha256 0de1e5902bc8), as they are:_

````text
### The class vocabulary
- **A message's class** (the "Wire messages" table; the Class column of ADR 0001's catalogue: "Every message type has
  a visibility class; one with none fails the audit test", row 661):
  - `n/a`: the message tells no client anything about the world or another player. This covers everything a client
    sends, and the link's own `Ping` and `Pong`;
  - `owner`: only the client it is about receives it;
  - `public`: any client in the room may receive it;
  - `per field`: each field group has its own class (the "Field groups" table).
- **A field group's class** (the "Field groups" table; ADR 0001's state table, 634-658, and part 9 (e), 406-408):
  - `owner`: only the slot's own client;
  - `team`: only clients on that team;
  - `spatial`: only clients whose relevance region holds it and whose area connects to it (part 9 (a), (b)), or, for
    an event, clients its radius and area test pass (part 9 (f));
  - `public`: every client;
  - `authority-only`: never sent to any client. The state table writes this class as "none", with "Sent to: nobody"
    (638): see Q4.
````

### The message catalogue: §3, field groups, with each group's class (docs/design/netcode-proof-messages.md)
_`docs/design/netcode-proof-messages.md` lines 125-157 (sha256 0de1e5902bc8), as they are:_

````text
## 3. Field groups

The groups of the two `per field` messages, with their classes from ADR 0001's state table (634-658). The
`authority-only` groups are listed under both messages, because a "whole state" encoder is where such data would most
easily leak. The spatial groups are packed by the priority accumulator under the per-client cap: the client's own
state always, the nearest 8 enemies in every snapshot, every relevant entity at least every 4 snapshots (part 9 (d)).

| Message | Field group | Class | Sent to | Notes |
|---|---|---|---|---|
| `Snapshot` | header: server tick | public | every client | the full 32-bit server tick, in every datagram (318-321); the state table's "server tick and clock" (650) |
| `Snapshot` | header: baseline and acknowledgements | owner | that slot's client | the baseline as its 16-bit age in ticks from the header tick, 0 meaning coded against the per-type defaults (319); the group index and the group count, so the client knows when a tick is complete; the newest command received, which ends its repeats, and the last command run, where the replay starts, each as a 16-bit command sequence with a known flag (228-230: a sequence names the command, while the tick it ran at can differ from its stamp; U4a as built, the advisor's W1-N2 ruling) |
| `Snapshot` | own state | owner | that slot's client | position and velocity, unquantised or the room's own copy quantised the same way each tick; weapon state (ammo, fire timers, reload, active reload); the stand-in attribute rows own movement and firing read, for PM-0001-60; sent when changed (203-209; state table 636-637). As built (U4d, the advisor's W1-S1): the section carries the owner's soldier record (the remote row's fields), its owner-only record `OwnState` (velocity, ammunition, fire timer, reload left, move speed, fire interval; the stand-in has no active reload) and its effect rows, and the codec refuses a record whose kind the section's rows do not list |
| `Snapshot` | remote soldiers | spatial | clients whose region holds the soldier and whose area connects (part 9 (a), (b)) | E3: position to 1 cm in 16, 16 and 12 bits, yaw in 16 bits, pitch in 10 bits; health; 8 flag bits for stance and actions; weapon id; named by kind and index, with the generation in the record that brings it into the set; a respawn at a held index is a removal plus an addition (279-281, 309-318) |
| `Snapshot` | vehicles | spatial | clients whose region holds the vehicle's bounds plus its visual radius (part 9 (f)) | hull and turret transforms and health per part; parts travel with their parent (313-314); the stand-ins: 15 big vehicles, 4 of at least 20 parts (plan E1) |
| `Snapshot` | projectiles | spatial | clients the projectile is relevant to, from the tick it becomes relevant | trajectory records (type, start tick, start position, velocity, fuse), re-based to that tick and re-sent only when the trajectory changes; the blast that ends one is its last change (281-289, 304-306); the thrower gets it too (state table 641) |
| `Snapshot` | world pieces | spatial | clients whose region holds the piece | the doors that open and close once a second and the destructible-wall stand-in (plan E1); part of the draft's "spatial records" (row 668); proposed: a portal counts as inside both areas it joins |
| `Snapshot` | gameplay events | spatial | clients the event's radius and area test pass (part 9 (f)); shots at or hitting the viewer always | hits, deaths and kills, and blasts that end no projectile; stamped with their tick and resent until acknowledged, from a bounded queue (297-302); no shooter id when the shooter is not relevant to the viewer |
| `Snapshot` | hit confirmation | owner | the shooter's client | a gameplay event, resent until acknowledged (308-309; state table 639) |
| `Snapshot` | cosmetic events | spatial | clients the event's radius and area test pass, never the client whose own shot it is | shots, muzzle flashes, sounds, and action starts (fire, reload, throw, melee); sent once, never resent (303, 309-311) |
| `Snapshot` | team section | team | that team's clients | empty in the spike: the stand-in sim has no team-private state (no economy: 1642-1643), so the encoder writes none |
| `Snapshot` | public section | public | every client | empty in the spike: no uplinks, countdown or steals (state table 645), so the encoder writes none |
| `Snapshot` | weapon spread stream | authority-only | nobody | SEC-SPREAD (a): the spread's draws and state stay on the room (212-215; the state table's class "none", 638) |
| `Snapshot` | RNG state | authority-only | nobody | every random stream's state (`ChaCha8Rng`, seeded by the host: plan §4.5); the client's prediction draws from no stream (193) |
| `Snapshot` | room-only records | authority-only | nobody | the per-tick state hash (no hash is exchanged with clients: 322-326); the hit-volume history (part 8); other clients' commands, view times and RTTs, and the view-time and aim side file (377-385); the roster's tokens and capabilities |
| `FullState` | header: server tick | public | the client it is for | the full 32-bit tick the whole state is taken at |
| `FullState` | header: baseline and acknowledgements | owner | the client it is for | no baseline (a full state is coded from the per-type defaults: 276-278); the last command run, where the client's prediction starts |
| `FullState` | own state | owner | the client it is for | as in `Snapshot`, whole |
| `FullState` | spatial world | spatial | the client's interest set at that tick | every remote soldier, vehicle, projectile (re-based to that tick) and world piece the client may see, whole. Proposed: no event section; a resync clears the client's queue of unacknowledged events, since the state already holds their results (Q8) |
| `FullState` | team section | team | that team's clients | empty in the spike (as in `Snapshot`) |
| `FullState` | public section | public | the client it is for | empty in the spike (as in `Snapshot`) |
| `FullState` | weapon spread stream | authority-only | nobody | as in `Snapshot` |
| `FullState` | RNG state | authority-only | nobody | as in `Snapshot`; a full state is the client's view, never a sim save |
| `FullState` | room-only records | authority-only | nobody | as in `Snapshot` |
````

### The message catalogue: §6, what the class audit test asserts (r4 is its U4 item 5) (docs/design/netcode-proof-messages.md)
_`docs/design/netcode-proof-messages.md` lines 213-255 (sha256 0de1e5902bc8), as they are:_

````text
## 6. What the class audit test asserts

The main session writes the test. This is what it checks.

**At U1, on this doc alone:**
1. The doc holds exactly the four tables, found by their exact header rows (section 1), and no other table.
2. "Wire messages" holds plan §4.7's 15 names (`Input`, `Snapshot`, `FullState`, `Resync`, `Ping`, `Pong`, `Join`,
   `JoinAccepted`, `JoinRejected`, `ClientSettings`, `RoomEnded`, `Leave`, `Kick`, `SessionControl`, and
   `SessionStatus` by the advisor's A1 ruling), each exactly once with Spike status `built in U4`, and no other row is
   `built in U4`.
3. Each built row has a Direction of `client → room`, `room → client` or `both ways`; a Channel of `datagram`,
   `control stream` or `full-state stream`; an Authority that starts with `Sender:` and names at least one of
   npt-server, npt-room and npt-client; a Size limit that starts with a whole number of bytes; and a Draft that is
   not empty.
4. Every row's Class, omitted rows included, is one of `n/a`, `owner`, `public` and `per field`: a type without a
   class fails (ADR 0001:661).
5. `RoomState`, `HostCommand`, `PickUpgrade`, `Buy`, `ContentReload`, `Lobby calls (HTTPS)`, `Supervisor calls` and
   `Lobby ↔ supervisor` are each present, with a Spike status of `omitted: ` and a reason that is not empty.
6. In "Field groups", every Message is a built `per field` type, and each `per field` type has at least one row. Each
   Class is one of `owner`, `team`, `spatial`, `public` and `authority-only`. Every `authority-only` row is sent to
   `nobody`, and no other row is. Both messages list the weapon spread stream and the RNG state as `authority-only`.
7. "Session controls" holds exactly `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep` (plan §4.7), and each
   row's Fields start with `cap`.
8. Every "Room-wide controls" row's Who names `npt`.
9. Each assertion has a broken copy of the doc that must fail it (one class removed, `RoomState`'s reason deleted, one
   §4.7 type missing, and so on): the plan's "bites first" rule (§6.1).

**At U4, against npt-proto and the encoder:**
1. npt-proto's message types equal the built rows' Type names, and `SessionControl`'s variants equal the "Session
   controls" rows, both ways round: a type in the code with no row fails, and so does a row with no type.
2. Each type's channel and direction in the code equal its row's. A message received on another channel is refused and
   counted, on both sides (plan E7).
3. npt-proto's `Limits` equal the Size limit column. For each type, a buffer one byte over its limit is refused before
   anything is allocated (checked with PM-0001-34's counting allocator). (The allocation half, refused before anything
   is allocated, is checked with PM-0001-34's counting allocator in U5: the advisor's Q8 ruling; U4 proves the
   size-first refusal.)
4. The encoder's layout table names exactly the "Field groups" rows for `Snapshot` and `FullState`, each with the same
   class. The `authority-only` groups have no layout entry at all.
5. In an L1 run (the stress scene, 48 clients in 3 × 16, P2, 3 seeds), every group the encoder writes into each
   client's snapshots and full states is one its class lets that client see: `owner` only to its own slot, `team`
   only to that team, `spatial` only inside that client's interest set or event test, `public` to anyone. A bite that
   sends one `owner` group to every client must fail.
6. The control-socket messages decode with the 1,024 B limit and never panic (plan §10.2; Q6).
````

### The advisor's check of the U4r brief: conditions L1 to L6 and the U4R-Q rulings (advisor-u4r-brief.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4r-brief.md` lines 1-463 (sha256 35f281bfc11c), as they are:_

````text
# The advisor's check of the U4r builder brief (rev 3.1), 2026-11-13

Seat: a day session's advisor (workflow agent, opus, effort xhigh), on main's `.claude/agents/advisor.md` (read with
`git show origin/main:.claude/agents/advisor.md`). Read only: no build, no edit of a source or of the brief, no commit.
Branch `claude/netcode-proof-test-plan-hcky5g`, head ad248a2 when this was written (checkpoint commits only since
c49773f). The brief: u4r-brief.md rev 3.1, sha256 bcaef38322375a74c902f998eef079b630e9525ae98550d84f7e318596e36e2e
(1,625 lines). Notes: advisor-u4r-brief-notes.md (this run was cut by the spend limit at 13:06 after B9 and resumed at
14:12; the notes hold every block).

## Verdict
**APPROVE WITH CONDITIONS.** The build may launch once conditions L1 to L5 are met; L6 is the launch.
- Every item of plan §5 U4r (its Builds, Tests, judged runs, the -30 pre-check, the sizes, the Evidence and the
  reviewers) and every carry of §U4r is in the brief, testable as worded (check 1). The placeholders P1 to P15 each find
  their anchor once, with the first and last words named (check 2). My rulings since the night's rev 2 are applied, with
  two gaps (check 3: A1, A8). netcode-architect's High (F-j) and four Mediums are answered, and the revision's own
  addition (U4R-Q14 (b), the duplicate stamps) holds under my arithmetic.
- My findings: **two Medium** (A1: the TE-M3 line and P12 miss my TE-M3 rulings; A3: U4R-Q14 (a)'s lead has no margin
  at P0) and **seven Low** (A2, A4 to A9). None High; no security defect.
- All twelve open questions are ruled below, none escalated: each is a technical or test choice inside the plan, or
  the text of my own conditions and carries. No rule or pass bar is loosened, and no word or decision of Mustafa's
  changes.
- The lane runner u4r-run.sh: **approved**. INF2-4's runner follow-up (my M9 Block 2): **met**.
- Risk tier 1 (spike code on PR #40's branch; check.mjs is the spike's own check, not one of tools/' gate files). This
  ruling is a notes-folder text.

## What I read, and the state it rests on
- The brief whole; its history (rev 2 at ed973c9; the rev 3 check u4r-brief-r3-check.md: REVISE, F-j High, F-a, F-k,
  F-l, F-m Medium, 13 Lows; the drafter's and reviser's notes u4r-brief-r3-notes.md, read as their account, not as
  evidence).
- The sources: u4-build-plan.md §5 U4r (:750-803) and the parts the brief cites; plan.md; u4s-carries.md §U4r
  (:255-432) and entries 6, 14, 20, 49, 63, 74; my rulings advisor-u4.md, -u4-wave1, -u4d-close, -u4j, -u4j-r2,
  -u4df, -u4e-r1, -u4e-r2, -u4d2-r1, -u4d2-r2, -u4d2-m9, -te-m3 and -te-m3-check; the code at the head under
  spikes/netcode-proof/ (crates client, room, server, l1, link, sim, lab, bots, log, net; check.mjs).
- **Stamps at the head** (equal to section 0's): plan 01ede63d…, plan.md 3ca4e12a…, carries 8b7f165e…, history
  88bc195f… (2,011 lines), u4r-run.sh 349b51ad…; all 13 rulings' sha256 equal section 0. New since: advisor-te-m3.md
  5ecc3e23… and advisor-te-m3-check.md 1ffd0294… (A1).
- **spikes/ moved once since the brief's head read:** `git diff --stat cc8595d HEAD -- spikes/` names only
  crates/net/src/l2/delay_tests.rs (+10 −3, c49773f, TE-M3's fix); `git diff --quiet c49773f HEAD --
  spikes/netcode-proof` exits 0 (A2).

## Check 1: coverage of §5 U4r, testable as worded
Every Builds item (`SimClock`, `L1Scenario` in RQ8's order, the header params, the `--bite` dispatch, the logs with
`admit` events and footers, `scenario --record` and `replay`, the runner recordings, -57's pairing, the brawl and the
open-loop scripts, the c-u4r scripts), every Test (r1 twin with its bite; r2 contract with PM-0001-2 passing and
condition 5's suites by U4R-Q4; r3 replay with the floored-grid bite; r4 class audit with owner-to-all; r5 ticks per
second and the set's total), every judged run (-17 ×4 with tick16 at 63,736 and client-rounding; -57 with its three
bites; -18 B1 with 10 bursts, no-fallback and `burst_eviction`), the -30 pre-check whole (suite, legs, depth, comparer,
`late_count`, the setup condition, the RTT rule with its logs, both bites, no mark, the records, NOT-RUN), the sizes for
fixes.rs with their framing, the Evidence (7.4) and the reviewers (section 12 item 7) are in the brief, each ending in
an exit code, a test's own summary line or a judge's line. The plan cites I spot-checked hold (:750, :756, :758,
:766-772, :791-792, :796-803, :806, :835, :837, :881-888, :909-930). The additions beyond the plan are marked as the
brief's (r6, r7, 7.2's receipt field, 8.4's switch, precheck-recordings.json, the `rewinds_refused` reports, -18's heal
bite check, 2.2's step rule, D13), but one (A7).

## Check 2: the placeholders
Print-only, against the files now: P1 carries "## U4r (the L1 runner and its judged runs)" once (:255; span :255-432;
entry numbers exactly the 17); P2 history :292 once and its continuation :293 ("before it changes anything."), the line
before it "  USAGE-SAVINGS: (a), all three, with these limits." (two leading spaces: the brief's whitespace rule reads
it as one, so its assert holds); P3 df-ruling :462-464; P4 df-ruling :465-468 (two `<crate>`: the first replaced,
"<crate>'s files" by "their files"); P5 j-r2 :168-171; P8 j-r2 :298; P9 m9 :252-256; P10 m9 :259-264; P11 d2-r2
:304-308; P12 d2-r1 :308-310; P13 to P15 carries :72-75, :116-119, :135-136. Each once, first and last words as named;
every closed file's sha256 equals section 0. P12's words now need my TE-Q2 beside them (A1), and TE-Q3's words need a
placeholder of their own (P16, A1).

## Check 3: my rulings since rev 2
Applied as ruled: W1-N2 and U4E-Q2 (-58 out: C2), U4E-Q3 to Q6 and Q13 (C6, carry 58, owner-to-all), W1-TARGET item 3
as amended (P9), the close ruling's F3 and carries 58-60, the U4j ruling's carry 62 with R2.4, FU-3 and FU-6, j-r2's
B2.1, B2.3, B6 and B3, e-ruling's U4E-Q13 and F2-ROUTE (carry 66 met by U4d2), df-ruling R3 (69), R4.1 (P4), R6 (the
slip line) and T4 items 1 to 3, e-r1's R1 (73), A5, R3 (74 to 76), R4 with NC-L1 (C7) and NC-L4, R5 (77) and M9,
e-r2's merge note, d2-ruling A13 (crates/bots/clippy.toml), d2-r1 Blocks 4 and 5 (82, 83, TE-M3), INF-1's det-u4.mjs
question, d2-r2 Blocks 1, 2, 5 (the interim rule) and 6 (P11), m9 Blocks 1 and 2. Two gaps: my TE-M3 rulings of today
(A1) and d2-r1's RR-L4 (A8).

## Findings, each with its exact fix
Old phrases are matched with each line break and the next line's indent read as one space. I counted each one in the
flattened brief: once each (50 phrases, print-only `python3 -I` from stdin).

**A1, Medium: the TE-M3 line and P12 do not carry my TE-M3 rulings (TE-Q2, TE-Q3), so TE2-5's end reads wrong.** The
line still says "its cause is the main session's planned change before U4r's merge", and P12 ends the rule "until
TE-M3's cause is found and fixed", which c49773f now meets. My TE-Q2 (advisor-te-m3.md:119-121) moved the end to U4r's
merge, and only if no check.mjs test-step failure happens in P5 or in U4r's counted and judged runs. My check of the
commit (advisor-te-m3-check.md, "What the main session does next", item 2) asks for exactly these words. The test's
lines also moved (:2551-2603 is now its doc :2544-2553 and the test :2554-2614). Fix (made only now that this check has
returned, as that item says):
- Replace the span from "and TE-M3's npt-net test," to "(TE2-5 (c), d2-r1 :303-304)." with: "and TE-M3's npt-net test,
  `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` (delay_tests.rs:2544-2614,
  its doc :2544-2553), which failed under a counted run's load (d2-r1 :287-293). Its cause is found and fixed at
  c49773f (TE2-5 (c); the advisor's TE-M3 ruling, advisor-te-m3.md, and its check of the commit,
  advisor-te-m3-check.md: APPROVE): a pump run already under way when the shutdown began calls S's `poll_shutdown`
  once but started before the test's count, so the assertion is `s_calls <= polls + 1`. "<<P16>>" (the advisor's TE-M3
  ruling, TE-Q3, word for word). The test is in no crate U4r changes and reads no U4r file, so a failure of it (polls =
  4 included) is recorded under TE2-5 with its name and panic (FX-10's file of the failed step kept beside the log
  under .gate/u4r/), does not stop your step group, and comes to the advisor whole; never a retry."
- Replace "(the advisor's U4d2 round-1 ruling, Block 5). **Expected failures, named in advance:**" with "(the
  advisor's U4d2 round-1 ruling, Block 5); since TE-M3's fix (c49773f) the rule ends at U4r's merge, and only if no
  failure of check.mjs's test step (any test) happens in U4r's counted and judged runs; if one does, the rule continues
  and that failure comes to the advisor (TE-Q2, advisor-te-m3.md:119-121; TE-M3's P5 had none). **Expected failures,
  named in advance:**".
- Section 12 item 5: replace the span from "5. **TE2-5 (c), TE-M3's planned change**" to "so the builder's TE2-5
  entries read it." with "5. **TE2-5 (c), TE-M3's change: made at c49773f** (advisor-te-m3.md; its check
  advisor-te-m3-check.md, APPROVE; CI green on c49773f). Section 4's TE-M3 line carries it, with P16 and TE-Q2's end."
- Section 12 item 3 gains: "`<<P16>>` advisor-te-m3.md :136-139: the words inside TE-Q3's quotation marks, from
  \"U3c's (u3c-fix2-c2b.md:74)\" to \"run watch every assertion.\" (each found once; sha256 5ecc3e23… asserted
  first)." Section 0's stamps gain `te-m3` advisor-te-m3.md (5ecc3e23…), `te-m3-check` advisor-te-m3-check.md
  (1ffd0294…) and `u4r-check`, this file (its sha256 at the fold).

**A2, Low: the "spikes/ is cc8595d's" tests no longer hold as worded.** Since c49773f, `git diff --quiet cc8595d <base>
-- spikes/netcode-proof` exits 1, so section 12 item 1 would send the launch to a counted run it does not need and item
9 to a re-walk of section 9; and the ancestry checks would accept a base without TE-M3's fix, which the TE-M3 line now
assumes. The counts are unchanged at c49773f (TE-M3's P5: 11 of 11 steps, 491 / 561, self 71 of 71, marks 81, redact
0: advisor-te-m3-check.md, item 6). Fix, each old phrase by its new one:
- "Its spikes/ is cc8595d's (`git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exits 0), U4d2's merge, which
  holds" → "Its spikes/ was cc8595d's, U4d2's merge, and is now c49773f's (TE-M3's fix, delay_tests.rs alone: `git diff
  --stat cc8595d c49773f -- spikes/` names that one file; `git diff --quiet c49773f HEAD -- spikes/netcode-proof`
  exits 0); cc8595d holds".
- "`git merge-base --is-ancestor cc8595d HEAD` exits 0. Record both" → "`git merge-base --is-ancestor c49773f HEAD`
  exits 0 (TE-M3's fix, on U4d2's merge cc8595d). Record both".
- "at cc8595d, U4d2's merge, the same: D1)" → "at cc8595d, U4d2's merge, and at c49773f, TE-M3's fix, the same: D1)".
- "the `git merge-base --is-ancestor cc8595d HEAD` result" → "the `git merge-base --is-ancestor c49773f HEAD` result".
- "read at the head fa8e335 (spikes/ = cc8595d)" → "read at the head fa8e335 (spikes/ = cc8595d; c49773f since then
  changes one npt-net test, which no row names)".
- "spikes/ at `<base>` differs from cc8595d (section 12 item 8)." → "spikes/ at `<base>` differs from c49773f's
  (section 12 item 9)." (this also fixes A5's third cite).
- D1: "They are `<base>`'s counts if `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> --
  spikes/netcode-proof` exits 0 (it exits 0 at fa8e335)" → "The same at c49773f, TE-M3's fix (its P5: 11 of 11 steps,
  491 / 561, self 71 of 71, marks 81, redact 0: advisor-te-m3-check.md). They are `<base>`'s counts if `cd
  /home/user/Shamble-Frontlines && git diff --quiet c49773f <base> -- spikes/netcode-proof` exits 0 (it exits 0 at
  ad248a2)".
- Section 12 item 1: "If `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> -- spikes/netcode-proof`
  exits 0, the counts are D1's" → the same with c49773f for cc8595d.
- Section 12 item 9: "if spikes/ at `<base>` differs from cc8595d," → "if spikes/ at `<base>` differs from c49773f's,".

**A3, Medium: U4R-Q14 (a)'s lead has no margin at P0.** The round-trip form gives P0 a lead of 1. With no delay, a
client step's newest n means only s ≤ t_{n+1}: the snapshot of n + 1 leaves at t_{n+1} (RQ8 step 3) and is never
delivered in its send instant (l1/src/lib.rs:1672, `gate_ns < now_ns`), while the client steps come after the tick
(step 5). So a step at a tick's own due instant reads n, stamps n + 1, the tick just run, and its `Input` is delivered
only after that instant: late (room/src/input.rs:107-113). A client's step instants are fixed from its first `input`
instant at the tick period (client/src/lib.rs:386, :452-453), so a client whose first input falls on a tick instant
keeps falling on them (wholly when it is a whole 100 ms from the base, every third tick), and every such command is
late. Lead 2 removes it. Fix: U4R-Q14's ruling below (the lead `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) +
1`, so P0 2), with its words in 2.1, carry 74 and D13.

**A4, Low: U4R-Q14 (b1) needs a press rule.** The room refuses a command whose press counter moved by more edges than
the ticks it covers since the last accepted command (input.rs:91-99, `PressRefused`, counted apart from `late`:
room.rs:796). Under (b1) a bot that counts its press edges per step can give, after a skipped step, a command covering
one tick with two edges: refused, silently, outside carry 74's late count. Fix: in U4R-Q14's ruling below (press edges
counted per command given), with 2.2's sentence and D13's `press_refused` count.

**A5, Low: three cross-references send the launch re-check to section 12 item 8; it is item 9.** Fix: "the re-check at
launch compares (section 12 item 8)." → "the re-check at launch compares (section 12 item 9)."; "(section 12 items 3
and 8)" → "(section 12 items 3 and 9)"; the third is A2's section 9 line.

**A6, Low: one wrong cite.** 2.1 step (2)'s `Decoder::accept` and `reject` are pump.rs:276-287 (:263-274 are
`is_closed` and `is_joined`). Fix: "(pump.rs:265-275; without `accept`" → "(pump.rs:276-287; without `accept`".

**A7, Low: carry 49's unit test is an addition the brief does not mark.** Carry 49 asks the refusal, not a test of it.
I keep the test (a refusal never shown to fire is not shown to work). Fix: "A unit test shows the refusal can fire:" →
"A unit test (this brief's: carry 49 asks none) shows the refusal can fire:"; and in section 2's preamble, "it adds
nothing it does not mark as its own (2.4 r6 and r7, 7.2's added records, 8.4's switch)," → "it adds nothing it does
not mark as its own (2.1's test of carry 49's refusal, 2.4 r6 and r7, 7.2's added records, 8.4's switch),".

**A8, Low: d2-r1's RR-L4 is not named where the `wasm` step's time is judged.** RR-L4 (d2-r1 :209-212) says that if
U4r's Evidence decision brings the step's time into question, the first remedy is to feed `replay_to` by chunks and
cache the hash per `step` in hash-wasm's State, before the step's runs move out of check.mjs. Fix: "Over 300 s, it comes
to the advisor as U4r's det-u4.mjs question" → "Over 300 s, it comes to the advisor as U4r's det-u4.mjs question, with
RR-L4's remedy (d2-r1 :209-212: `replay_to` fed by chunks and the hash cached per `step`, crates/hash-wasm, not your
lane) weighed first".

**A9, Low: the drafter's scratch copy by shell redirect (task item 8).** The drafter wrote plan-df.md into the
session's scratchpad by a shell redirect (its notes, B3a). I checked: it is outside the repository, its sha256 equals
`git show df8975b:.checkpoints/netcode-proof-test/u4-build-plan.md`'s (0d69332c…), it was only read, and the notes say
so at their top (df-ruling R6's form). It broke the write rule's letter (a `>` into a file, in any folder), changed
nothing in the repository and nothing the brief rests on. Recorded; nothing redone, no push. The brief's section 4
already carries the lesson ("in any folder, the session scratchpad included"); the builder's limits stand.

## Rulings on the open questions
Each ruling's words go into section 10's "Answered" table (a row each, its question kept as asked below it, for the
record) and into the places it names; where the brief says "by U4R-Qn", the ruling's words replace it.

**U4R-Q1 (npt-lab to npt-bots, npt-bots to npt-sim): (b).** The main session adds both edges in its own commit on
committed state before the launch (condition 10 (d)): `npt-bots.workspace = true` in crates/lab/Cargo.toml and
`npt-sim.workspace = true` (no features) in crates/bots/Cargo.toml, and nothing else. Its own counted run (main-run.sh,
`-x`) shows: Cargo.lock's diff holds only those two edges (no new package); check.mjs's 11 steps pass with the base's
counts; the deps step's npt-sim feature sets stay one set in SIM_BUILDS (check.mjs:1636-1640); and the `wasm` step's
seconds beside D3's 33.1 s warm, since npt-lab's two release builds (WASM_BUILDS) now carry npt-net's tree
(bots/Cargo.toml:24). If that tree puts the step's time in question, it comes to me first (one route then: npt-net
optional in npt-bots behind a feature its two binaries require). The builder's lane holds no manifest (section 4).

**U4R-Q2 (what is left): (c).** U4r's lane gains marks/SERIES.md for new rows only, one per name U4r writes that has
none (R26's list), each naming its writer; no existing row changes. Carry 62 (h)'s check in marks.json is the main
session's (the carry's "else" branch), after r2's evidence: if r2's measured-phase sum of `tick.s.snap_incomplete` is 0
on every short L1 P0 run, the main session adds the check and its bite entry to -17's L1 P0 part in its own commit on
the spike branch before the counted run of U4r's merge step, and shows the check can fail by running the judge (`--only
PM-0001-17`) on the builder's bitten short P0 logs (setup check FAIL by name); not 0 sends the cause to me first. The
bite's site is U4r's (npt-lab's client writer, bites build only), its id in the form `PM-0001-17/<name>`, named in the
report; marks.json's entry uses that exact spelling.

**U4R-Q4 (r2's scope): (a).** r2 covers -17's four profiles, -18's B1, -20's P3 under option B and -21's option-B
suites with PM-0001-2 passing and the shot band, the open-loop scripts standing in for the cheats' command patterns
(those suites' own checks not resolved), plus the carries' suites. This narrows my condition 5 at U4r to what can run
there and moves its option-A half, not the bar: -21's option-A suites and -20's option A must pass PM-0001-2 in U4f's
contract test before their marks count. The main session adds a carry line to u4s-carries.md §U4f (:491): "**U4R-Q4,
condition 5's option-A half (the advisor's U4r brief check):** U4f extends U4r's contract test (r2) to -21's option-A
suites and -20's P3 option A, with every PM-0001-2 check passing, not only resolving, and the shot band, before -20's
and -21's marks count; a suite that fails comes to the advisor first."

**U4R-Q6 (the wallclock bite's site): (a),** with one rule for the field. U4r's lane gains in npt-server, bites builds
only: `Decoder::with_bite(self, &str) -> Option<Decoder>` (a pub item beyond §4.1, named in the report under this
ruling); the site in `pump`, on that instance's decoded `Input`s; `#[allow(clippy::disallowed_methods,
clippy::disallowed_types)]` on the site alone (server/clippy.toml:6-17); `npt_server::BITES` widened to four, with
crates/server/tests/schedule.rs:347's pin updated. **The field rule:** the wall-clock read changes, on every command
the pump passes on for that leg, a field the World's hashed state takes at that tick (the command's move or aim), to a
value never equal to the received one (for example XOR with 1 | the clock's low bits), and never its tick, sequence,
slot, buttons, press counters or view time, which would give a VOID, a repeat or a refusal and show nothing. The bite
is shown on a pair that is not VOID, the comparer naming a tick (7.2). P4's replacement gains ", or in npt-server".
.claude/rules/netcode.md binds these edits. rust-reviewer and engine-architect read them.

**U4R-Q7 (-17's clean runs' `tick_start`): (a).** Every -17 L1 run, clean and bitten, starts at 63,736; -57's and
-18's shared runs with them. Replace "the clean runs' `tick_start`: U4R-Q7;" with "every -17 L1 run, clean and bitten,
at `tick_start` = 63,736 (U4R-Q7 (a));".

**U4R-Q8 (-18's runs): (a), shared.** -18's measured runs are -17's B1 runs, with -18's 10 bursts (offsets 0 to 1
tick) and headers listing -17, -57 and -18; each bite run lists its own mark only. Replace "its bursts: U4R-Q8" with
"its bursts are -18's (U4R-Q8 (a): -18's measured runs are these, their headers listing -17, -57 and -18)"; 7.3's row
"| -18 measured (B1 × seeds 1-3), if not shared with -17's B1 runs (U4R-Q8) | 3 × 600 s | 1,800 |" is removed (the
totals: U4R-Q9 below).

**U4R-Q9 (the TC B1 recording): (b).** U4r records SS P2 seed 1, ES P3 seed 2 and SS P0 seed 4; TC B1 seed 3 is U4h's,
made with its TC shooter. The real reason is the Rewinds: a TC recording without TC's shooter holds Rewinds the
shooter would not make, and U4h builds both the shooter and the rules they meet (plan :835-837). It also takes one
`scenario --record` out of the `wasm` step. Brief words:
- "**The four runner recordings**" → "**The three runner recordings at U4r**"; "`scenario --record` at SS P2 seed 1,
  ES P3 seed 2, TC B1 seed 3 (U4R-Q9), and SS P0 seed 4" → "`scenario --record` at SS P2 seed 1 and ES P3 seed 2, and
  SS P0 seed 4 (TC B1 seed 3 is U4h's, with its TC shooter: U4R-Q9 (b))".
- "(SS P2 seed 1, ES P3 seed 2, TC B1 seed 3, or two by U4R-Q9)" → "(SS P2 seed 1 and ES P3 seed 2 at U4r; U4h adds
  TC B1 seed 3)".
- "Its cost is about 3 × 9,000 / R s on top of the step's 33.1 s warm (R the ticks per second of 7.3: 300 s at R = 90,
  the whole 5-minute budget; 90 s at R = 300)" → "Its cost is about 2 × 9,000 / R s on top of the step's 33.1 s warm
  (R the ticks per second of 7.3: 200 s at R = 90; 60 s at R = 300)".
- "so the three about 3 × 9,000 / R s (300 s at R = 90; 90 s at R = 300), plus their replays;" → "so the two about 2 ×
  9,000 / R s (200 s at R = 90; 60 s at R = 300), plus their replays;".
- "the four recordings first (the pre-check needs SS P0 seed 4)" → "the three recordings first (the pre-check needs SS
  P0 seed 4)"; "The four recordings (or three, by U4R-Q9):" → "The three recordings:".
- 7.3: "| the four runner recordings | 4 × 300 s | 1,200 |" → "| the three runner recordings | 3 × 300 s | 900 |";
  "**15,600 s = 4.33 h = 468,000 ticks** (13,800 s = 414,000 ticks if -18 shares -17's B1 runs)" → "**13,500 s = 3.75 h
  = 405,000 ticks** (-18 sharing -17's B1 runs: U4R-Q8 (a))"; "**Wall time** = 468,000 ticks ÷ R" → "**Wall time** =
  405,000 ticks ÷ R"; the R table's rows → "| 300 | 10× | 1,350 s ≈ 23 min |", "| 90 | 3× | 4,500 s ≈ 1.3 h |", "| 30 |
  1× | 13,500 s ≈ 3.8 h |", "| 10 | 0.33× | 40,500 s ≈ 11.3 h |"; "the warm-ups add at most about 31 runs × 30 s = 930
  s" → "the warm-ups add at most about 27 runs × 30 s = 810 s".
- The plan text (the main session's commit, as my earlier rulings' plan edits; each old phrase found once): §4.15's
  "U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2, TC B1 seed 3)" → "U4r's two
  runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2) and U4h's TC B1 seed 3, made with its TC shooter
  (the advisor's U4r brief check, U4R-Q9)"; §4.15's "The determinism step keeps its three; the fourth serves the
  pre-check only." → "The determinism step keeps the runner recordings (two at U4r, the third at U4h); SS P0 seed 4
  serves the pre-check only."; §5 U4r's "the four runner recordings (§4.15; the fourth, SS P0, for the -30
  pre-check: the advisor's condition 3)" → "three runner recordings (§4.15: SS P2 seed 1, ES P3 seed 2, and SS P0 seed
  4 for the -30 pre-check: the advisor's condition 3; TC B1 seed 3 is U4h's: U4R-Q9)"; §5 U4h's Builds: after "the tolerance-edge
  stamper and the 150 ms pong delayer;" add " the TC B1 seed 3 runner recording with the TC shooter, joining the
  recordings manifest (U4R-Q9);". Carry line for §U4h
  (:548): "**U4R-Q9, the TC B1 seed 3 recording (the advisor's U4r brief check):** U4h records TC B1 seed 3 with
  `scenario --record` and its TC shooter, adds it to crates/lab/recordings.json under carry 82's schema, and states
  the `wasm` step's time with it against the 5-minute budget."

**U4R-Q10 (showing the RTT rule can fire): (b').** r7 at U4r; (a') only in the re-run after U4h, and only if U4h's
brief states a timing in which it can fire.

**U4R-Q11 (negative cases for r2): (a).** Unit tests on doctored copies of a short run's logs, each failing r2 by name:
one `admit` event gone, one footer gone, one written name's SERIES.md row gone, one `late_count` series gone where
`late-zero` applies.

**U4R-Q12 (the re-run after U4h): (a), reading (i).** My "6 runs" (advisor :346) meant one recording and three pairs:
reading (i), with 8.3's replay-first rule for the recording, results in results/u4h-precheck-30/<commit>/, read in
U4h's review. -30's three-recording shape is U6's C part. Carry line for §U4h (:548): "**O1's re-run (carry 40; the
advisor's U4r brief check, U4R-Q12 (a)):** after U4h merges, the main session replays U4r's SS P0 seed 4 recording on
the U4h build and counts both legs' replacements; at 0 that is the clean pair, else SS P0 seed 4 is re-recorded on the
U4h build (its SHA-256 beside the old one's); then the three pairs (clean, wallclock, late-command), six runs, by U4r's
pre-check script, results in results/u4h-precheck-30/<commit>/, read by the advisor in U4h's review; -30 stays NOT-RUN
until U6. U4h's brief states what its estimate does with an L1 session whose `rtt_ns` is still `None`; the re-run
compares each client's first-sample instant with the instant U4h's estimate first reads that client's RTT (its receipt
or its run, both logged at U4r); the Loopback leg's sessions keep `RttSource::Link`."

**U4R-Q13 (carry 63): (a), bounded.** U4r's lane gains crates/log/src/lib.rs for carry 63's two refusals and their
tests in its test module (log/src/lib.rs:549) only: `read_log` refusing a JSON object with a duplicate key, and
`check_header` refusing a header text field holding a character for which `char::is_control` holds; each test written
to fail before its refusal and shown failing by name then (the builder's notes give the failing line), nothing else in
crates/log changed, and check.mjs's test step's judge fixtures still passing. security-auditor reads them (the seat
that raised FU-3 and FU-7). P4's replacement gains ", or in npt-log".

**U4R-Q14 (carry 74's lead and the duplicate stamps): (a1) amended by A3, and (b1) with A4's rule.** The arithmetic,
from the merged code: the endpoint stamps each command newest + lead when the page gives it (client/src/lib.rs:394) and
sends it in that step's `Input` (:429); the room counts late a command whose tick it has run or already buffered
(room/src/input.rs:107-113), after the forward compare (:81-89) and the press compare (:91-104); L1's delay each way
is d − a + U[0, 2a], with d half the round trip and a half the jitter (l1/src/lib.rs:215-221, :789-814); the pump
(RQ8 step 2) runs before the tick (step 3) at one instant. Let tick k run at t_k (P = 33.3 ms at 30 Hz; its snapshot
leaves then):
- **The lead.** With a delay above 0, a step at s whose newest is n has not received n + 1, so s < t_{n+1} + d + a;
  its command, stamped n + L, arrives before t_{n+1} + rtt + jitter, and is in time if that is at or before t_{n+L}
  (an arrival at t_{n+L} itself is pumped before the tick). So (L − 1)·P ≥ rtt + jitter holds every command at every
  phase and every draw: L = ⌈(rtt + jitter) / P⌉ + 1, the brief's `ticks_for_ms(rtt_ms + jitter_ms) + 1`: P1 3, P2
  and B1 5 (120 / 33.3 = 3.6), P3 7 (180 / 33.3 = 5.4). It is the worst case, not the least lead the check's model
  found clean (P3 at 6); I keep it because it has no tail. At P0, A3: lead 2. **The ruled lead:**
  `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`: P0 2, P1 3, P2 and B1 5, P3 7, each within `CMD_CATCHUP` = 8.
- **Carry 74's own formula contradicts its premise.** At lead 1 a command is in time only if 2d plus the step's wait
  fits one tick period, so "every command late" sets in when the round trip passes a tick period, which is the carry's
  own words ("one-way delay above about half a tick period"). The arrival offset 2d + the step's wait ± the jitter
  spans about 80 to 153 ms at P2: lead 3 (100 ms) leaves most commands late, lead 4 (133 ms) a tail, lead 5 (167 ms)
  none, as the check's model found (97.7 %, 2.3 %, 0).
- **Losses and bursts at the ruled lead** (arithmetic; r2 reports them, nothing judged here): a command whose first
  `Input` is lost goes again in the next step's `Input` (the endpoint keeps the unacknowledged ones up to the
  redundancy cap: client/src/lib.rs:407-408, :429), one period later, and is late only in the tail where its arrival
  offset passes (L − 1)·P: about 2 % of resends, so about 0.05 % of P2's commands (2 % loss) and 0.1 % of P3's (5 %).
  A B1 burst (1 s of total loss both ways) under (b1) loses only the commands given while the snapshots sent before it
  still arrive, about (d + a) / P ≈ 2 per client, resent after it and late: about 20 a client over 10 bursts, about
  0.1 % of a 10-minute run. So carry 74's 1 % rule can hold on every U4r profile, and D13 and r2 show whether it does.
- **The duplicates (b) hold as the revision says.** A step inside a snapshot's arrival window (as wide as `jitter_ns`)
  sees that snapshot arrived with probability p, by a fresh draw each step, so two steps in a row read one newest with
  probability p(1 − p); over phases spread evenly, `jitter_ns` / P × 1/6: about 10 % at P2 and B1, 15 % at P3, 5 % at
  P1; at P0 only through a seeded clock rate other than 1. Such a second command is late (input.rs:110), or, if a press
  counter moved, refused (the press compare runs first and covers 0 ticks: :91-99), so (b2) would mix two counts, and
  under a B1 burst every step's command would carry one frozen stamp. (b1) leaves the stamps strictly rising, so a
  command is late only by its arrival, and the late count stays the link-timing signal carry 74 means.
- **A4's rule with (b1):** the bots count press edges per command given, never per step; a press wanted at a step
  without a command waits for the next command, at most one new edge per counter per command (input.rs:91-99).
- **Carry 74, amended by this ruling** (the main session's commit to u4s-carries.md; the carry is my R3's, and P1 then
  copies the amended words): "(its largest one-way delay, half the round trip plus half the jitter, in ticks rounded
  up, plus 1)" → "(the round trip plus the jitter, in ticks rounded up, at least 1, plus 1:
  `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`, so P0 2, P1 3, P2 and B1 5, P3 7, within `CMD_CATCHUP` = 8;
  its bots give a command only at a client step whose newest snapshot tick moved since their last command, and move
  each press counter by at most one edge per command; the advisor's U4r brief check, U4R-Q14)". Its 1 % rule and its
  report keep their words. This tightens nothing and loosens nothing: the bar stays 1 % of the commands sent.
- **Brief words.** 2.1: replace the span from "- **The lead** (carry 74, by U4R-Q14):" to "Either way r2 reports each
  short run's late count beside the commands sent (carry 74)." with "- **The lead** (carry 74 as U4R-Q14 amended it):
  each endpoint's `with_lead` (client/src/lib.rs:291-295; 1 by default, :262) from the run's profile,
  `npt_sim::ticks_for_ms((rtt_ns + jitter_ns) / 1,000,000).max(1) + 1` (sim/src/ticks.rs:35, rounding up: carry 39;
  every profile is in whole ms, l1/src/lib.rs:166-175): P0 2, P2 and B1 5, P3 7, each within `CMD_CATCHUP` = 8
  (room/src/input.rs:59-61), recorded beside `cmd_timing` in the header. It covers the round trip, the jitter and one
  client step at every phase, and at P0, where a packet is never delivered in its send instant, a step on a tick's own
  instant (U4R-Q14). The bots give no command at a client step whose newest tick did not move (2.2). r2 reports each
  short run's late count beside the commands sent and the steps without a command (carry 74)." 2.2: after "and r2
  reports per run the steps without a command beside the commands sent." add " A command's press counters move by at
  most one edge each since the bot's last command (the room refuses more edges than the ticks a command covers:
  room/src/input.rs:91-99), so a press wanted at a step without a command waits for the next command (U4R-Q14)." 3.2's
  row 74: "U4R-Q14 asks the advisor before launch, and the brief rewords no rule" → "U4R-Q14 ruled: the carry's lead
  amended to the round trip (P0 2), with the bots' step and press rules; its 1 % rule unchanged".
- **D13, widened:** replace the span from "- **D13, the lead and the duplicate stamps on a short P2 run" to "1 % at
  the ruled lead stops the build's recordings and comes to the advisor (carry 74)." with "- **D13, the lead and the
  duplicate stamps on short runs (stays; the builder's, before any `scenario --record`):** the brawl (SS, 48 clients,
  seed 1; one length for all, in the notes), each run under the runner (`-x`): (1) P2 at lead 5 under 2.2's rules;
  (2) P2 at lead 5 with a command at every step (a test-only setting, never a judged one: the duplicates' share, about
  10 % by U4R-Q14's arithmetic); (3) P2 at lead 3 (carry 74's old formula) under 2.2's rules; (4) P3 at lead 7 under
  2.2's rules; (5) P0 at lead 2 under 2.2's rules. The notes give per run the commands sent, the room's `late`,
  `press_refused` and `buffer_full` counts (`ClientCounters`, room.rs:118-122) with the late share, the commands stamped
  for a tick already stamped (`newest_tick()` + lead after each call), the steps without a command and
  `rewinds_refused`. Expected: (1), (4) and (5) at most 1 % late with 0 refused; (3) most commands late; (2) reported
  against the arithmetic. Its exact command lines cannot be written before the build (as D4). A late share above 1 %,
  or any press refusal, in (1), (4) or (5) stops the build's recordings and comes to the advisor (carry 74)."
- Wherever else the brief says "P0 1" for the lead (2.1's lead bullet, U4R-Q14's text, kept as asked), the fold reads
  the ruled numbers.

**Section 4's lane, as these rulings leave it.** Replace the bullet from "- **Waiting on a ruling, so not yours until it
comes:**" to "A part that needs one stops, goes in your report, and the rest is built." with "- **Opened by the
advisor's rulings (advisor-u4r-brief.md):** marks/SERIES.md, new rows only, for the names U4r writes (U4R-Q2 (c));
crates/log/src/lib.rs for carry 63's two refusals and their tests only (U4R-Q13 (a)); in npt-server, the wallclock
bite's items only (U4R-Q6 (a): the site in pump.rs, `Decoder::with_bite`, `npt_server::BITES` widened to four, and its
pin in crates/server/tests/schedule.rs:347; loopback.rs:347's one-home test reads the list whole). **Not yours:**
crates/lab/Cargo.toml, crates/bots/Cargo.toml and Cargo.lock (the main session's commit before launch: U4R-Q1 (b));
marks.json (carry 62 (h)'s check is the main session's: U4R-Q2 (c)). A part that needs anything more stops, goes in
your report, and the rest is built." In "Never": "crates/server (but U4R-Q6 (a)'s items, if ruled)," → "crates/server
(but U4R-Q6 (a)'s items),"; "crates/log (but U4R-Q13's), crates/judge," → "crates/log (but carry 63's two refusals:
U4R-Q13 (a)), crates/judge,"; "crates/tcpinfo; marks/ (but U4R-Q2's); docs/;" → "crates/tcpinfo; marks/ (but
SERIES.md's new rows: U4R-Q2 (c)); docs/;". P4's replacement: "npt-lab or npt-bots, or in npt-log, or in npt-server, in
check.mjs's deps, wasm or marks step, or in check.test.mjs" (both of section 12 item 3's optional clauses ruled in; the
marks step because SERIES.md's new rows are U4r's: U4R-Q2 (c)); section 12 item 3's P4 line is reworded to that one
string, and its assert checks it. Section 6
item 12's out-of-lane needs: "(U4R-Q1, Q2, Q6, Q13)" stays as the report's list of what it met through those rulings.

**The review panel (section 12 item 7): confirmed.** netcode-architect, test-engineer, engine-architect (the public
items, `Decoder::with_bite` among them), rust-reviewer (the npt-server and npt-log edits included), security-auditor
(carry 76's session boundary, carry 85's refusals, carry 63's two refusals, the wall-clock read in server code) and
infra-engineer (carry 84 and the `wasm` step's time against its budget), at xhigh, at most three agents at a time with
the builder counted, so two batches of three, one round. **The judged runs' target (item 8): confirmed** as written.

## The lane runner u4r-run.sh: approved
My own `diff u4d2-run.sh u4r-run.sh`: 6 lines, only the lane's names (the header :2-3, the log folder :9, the usage
line :16, the log case :41-42). The CACHEDIR.TAG check (:46-49), `exec 9>` and `flock "$mode" 9` (:52-53) and the
provenance block are byte-equal to u4d2-run.sh's. u4r-run.sh.diff equals my diff; u4r_runner.py asserts exactly that
(its keep list, one-line finds, lines of at most 120). sha256 349b51ad…, `bash -n` exit 0. The brief's cites
u4r-run.sh:12-15 and :18-22 hold. No finding. At launch it needs its sha256 and `bash -n` only (section 12 item 2).

## INF2-4's runner follow-up (my M9 Block 2): met
- **The two diffs against Block 2's lines.** main-run.sh.inf2-4.diff is Block 2 :157-171 exactly (S, T, LOCK=$T, the
  exports, the log, the shift, the tag check with its `fi`, mkdir, cd, `exec`, `flock -x 9`), the header sentence of
  :184-186 after "...exits with that command's code.", and MR9-L1's provenance form (:179-182).
  u4d2-run.sh.inf2-4.diff is :188-198 (T, LOCK, the export; the tag check between `[ -d ]` and mkdir; `exec` and
  `flock "$mode" 9`) and the header sentence after "...first non-zero code."; its provenance untouched (already
  anchored). No other line. The blobs (d9ce0e1, 0a5a587) are the diffs' post-images, committed at 90f5f61.
- **P1:** I re-ran `bash -n` on all nine scripts: exit 0 each.
- **P3:** .gate/u4d2-merge/inf2-4-p3a.log (`-x`) runs 11:01:18 to 11:01:26, exit 0, its provenance block empty
  (MR9-L1 fixed); .gate/u4d2/inf2-4-p3b.log (shared) was held until 11:01:26 (its mtime 0.45 s after p3a's), exit 0.
  The criterion holds. Not shown by the logs: p3b's start instant (the 6 s wait is the card's account).
- **P4:** the six one-line copies and their .diff files read: one line each, as Block 2's "small script" allows
  (inf2_4_proof.py asserts one line). I re-ran main-run.sh's (a) and (b) with their logs under .gate/u4r-brief-check/:
  both exit 5 with the STOP line, no marker and no log, the missing folders still absent ((b) made the empty
  .gate/u4r-brief-check/ by its `mkdir -p`). (c) not re-run: it opens the live lock file, which a check may not touch
  while other lanes could hold it.
- **The card's record** ("INF2-4's runner follow-up: DONE") matches the files and logs.

## TE2-5 (c): what the brief's TE-M3 line must carry
A1's words: the commit c49773f; the cause in one line; TE-Q3's quoted words (P16, advisor-te-m3.md:136-139) word for
word; TE-Q2's end (at U4r's merge, and only if no check.mjs test-step failure in U4r's counted and judged runs; else
the rule continues and the failure comes to me); every failure of the test, polls = 4 included, whole to me, never a
retry; the test's lines at the head. The brief is edited only now that this check has returned
(advisor-te-m3-check.md, item 2's last sentence).

## Conditions before the launch (the main session's)
- **L1. Fold this ruling into the brief by script (rev 3.2):** A1, A2 and A5 to A8 as written; the twelve rulings with
  their brief words (Q7, Q8, Q9, Q14 and section 4's lane as written; the others as section 10's table rows); section
  10's "Answered" table gains the twelve; the fold recorded under the brief's title. Each old phrase is asserted found
  once with its line breaks read as one space (I counted all of them: once each). If a phrase fails to match, or the
  fold must change any other word, it comes back to me; otherwise there is no second check.
- **L2. Carries and plan text, in the main session's own commit, before the launch:** carry 74's amended words
  (U4R-Q14); the §U4f line (U4R-Q4); the two §U4h lines (U4R-Q9, U4R-Q12); the plan's §4.15, §5 U4r and §5 U4h edits
  (U4R-Q9). Each old phrase found once. Then section 12 item 9 runs: section 0's stamps re-set for the plan and
  carries, and every cite of the brief into them whose line moved re-pointed by script, each asserted.
- **L3. The manifests (U4R-Q1 (b)),** on committed state, with its counted run's results in the card: Cargo.lock's two
  edges only, 11 steps with the base's counts, SIM_BUILDS one set, the `wasm` step's seconds. Then `<base>` is that
  commit or later, and section 12 item 1 takes its counts from that counted run.
- **L4. The placeholders and values (section 12 items 1 to 3, with P16):** each anchor found once, every closed file's
  sha256 asserted first, and P4 with both clauses.
- **L5. The card:** "Rules in force" keeps TE2-5's rule until U4r's merge (TE-Q2), and the U4r brief's entry names this
  ruling and rev 3.2.
- **L6. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in its own worktree from `<base>`, under
  W1-TARGET's lock and u4r-run.sh; at most three agents at a time, reviewers and the builder counted.

## What the main session does next, in order
1. Read this ruling; record it on the card (L5).
2. L3: the manifest commit and its counted run.
3. L2: the carries and plan text commit; re-stamp section 0's plan and carries.
4. L1: fold this ruling into the brief (rev 3.2), then section 12 items 1 to 3 and 9 (L4) on that text.
5. L6: launch the builder. D13 runs first in the build, before any `scenario --record`.
6. After the build: the panel above in two batches; my merge conditions follow U4d2's M1 to M9 form; then Q2 (c)'s
   marks.json commit if r2's (h) sums are 0, before the merge step's counted run.

## What I checked, and what I did not
- **Checked, by reading:** the brief whole and its change lists; plan §5 U4r and the cited lines; carries §U4r, 6, 14,
  20, 49, 63 and 74; my rulings named above; the code at the head for every cite in check 1, the -57 route, the -18
  record, the view time, the RTT rule and Q14's arithmetic (client :379-456, input.rs :20-150, room.rs :60-140 and
  :785-800, pump.rs :224-303, l1 :55-70, :160-225, :789-814, :1667-1681, sim ticks.rs, check.mjs :879-945, :1625-1660,
  :2015-2045); the placeholders' anchors and every fold phrase (print-only scripts from stdin).
- **Checked, by running (read only):** sha256 of every stamped file; git ancestry and diffs; `bash -n` on the nine
  scripts; main-run.sh's INF2-4 copies (a) and (b), logs under .gate/u4r-brief-check/.
- **Not checked:** I built, tested and timed nothing. Q14's numbers are arithmetic from the code under the model's
  assumptions (one datagram per snapshot, steps at a fixed phase, uniform draws), not a run: D13 measures them before
  any recording. The cost of npt-net's tree in npt-lab's builds is L3's counted run's to show. I did not re-run INF2-4's
  (c) (the live lock file).

## Decisions needed (Mustafa)
None. Every point is a technical or test choice inside the plan, a lane between units, or the text of my own conditions
and carries (condition 5's option-A half moved to U4f with its bar; carry 74's lead and its bots' rules). No rule or
pass bar is loosened beyond his words, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)
````

### The advisor's first ruling on U4r's build: copy 1's points 1 to 10 and further findings, then the fact check (advisor-u4r-build1-full.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4r-build1-full.md` lines 1-169 (sha256 476121ab65e6), as they are:_

````text
# Advisor: the full rulings on U4r build copy 1's points (workflow wf_21a67b58-19b), 2026-11-13

Written by script, word for word, from the workflow's journal (the session folder's
subagents/workflows/wf_21a67b58-19b/journal.jsonl): first the advisor's StructuredOutput result (advisor-u4r-build1),
then the fact check's (u4r-facts). The advisor's own notes: advisor-u4r-build1.md; the fact check's:
u4r-build1-facts.md; the questions: u4r-build1-questions.md. Nothing below is the main session's.

## The rulings

### Point 1 (severity: Medium; to Mustafa: no)

- **Decision:** Route (c), the plan's own (u4-build-plan.md :800-802: "in check.mjs if the step stays within 5 minutes, else through ops/runs/det-u4.mjs ... on the advisor's ok"), with my ok. The two runner entries go into a manifest of their own, crates/lab/det-recordings.json, in recordings.json's schema. ops/runs/det-u4.mjs runs the wasm step's clean half on it: regenerate and SHA-check, native leg 1 against leg 2, Wasm leg 1 against native leg 1, plus h2's vectors. It reuses check.mjs's wasmStep by import, never a copy. check.mjs's step keeps the three synthetic recordings. (a) RR-L4 was weighed first and does not fix this: it acts on the Wasm leg only and saves at most about 11.6 s + W, while regeneration takes about 195 s per legs call. Not taken. (b) is not taken: under (c) it saves nothing, and hash.rs `legs` stays as merged (brief 2.6). (d) is taken only inside (c): on its own it drops the plan's evidence. Machine and margin: the 300 s is judged on this container, the only machine that runs check.mjs (CI's gate.yml runs tools/gate.mjs and toolchain.yml runs the root cargo tests; neither runs spikes/; 4 vCPU Xeon @ 2.30 GHz, 16 GB). The figure is the step's printed seconds with builds included (M9's basis), labelled warm or cold with uptime, and cold binds. A recording set stays in check.mjs only while that cold figure is at most 200 s (two-thirds of 300 s). det-u4.mjs's non-cargo calls must stay at most 240 s inside carry 84's shared 300 s deadline; above 240 s it comes to me, with RR-L4 weighed first then. Words this changes: carry 82's "U4r's runner recordings join crates/lab/recordings.json" now reads "join crates/lab/det-recordings.json, in recordings.json's schema". Those are the reviewers' EA-8 and RR-L5 words as ruled in the advisor's U4d2 round-1 ruling; the advisor amends them here, and the rule (kind key, dispatch, name and ticks checks) is unchanged. Carry 87 (my U4r brief check's words) now reads: "adds it to crates/lab/det-recordings.json under carry 82's schema, and states det-u4.mjs's time with it, its non-cargo calls against carry 84's shared 300 s deadline (over 240 s it comes to the advisor, RR-L4's remedy weighed first)". Plan :800-802's trigger list widens to every crate on the scenario's path. The brief's 2.1 (i), section 5 item 4 and return list item 9 read as this ruling says.
- **Builder:** Point 1, the determinism route: (c), with the advisor's ok. (1) crates/lab/recordings.json stays as you have it: the three synthetic entries, each with "kind": "synthetic", and nothing else. (2) New file crates/lab/det-recordings.json (this ruling opens it). It uses recordings.json's schema and is read by the same `manifest`. It holds the two runner entries exactly as your notes :225-230 give them, SS-P2-s1 then ES-P3-s2, with an `about` text naming this ruling. hash.rs's h5 also asserts it: two entries, both kind runner, generator 0. (3) hash.rs `legs` stays as merged: no regeneration limited to `--only`'s entries. (4) check.mjs (this ruling opens this much beyond carries 83-85): `wasmStep` takes two options, the manifest path (default RECORDINGS_MANIFEST) and whether its bites half runs (default true). With the defaults it runs and prints exactly what it does now. With the bites half off it starts no bites-build call and says the half did not run by that option. Add one check.test.mjs case with the fake runner: a non-default manifest reaches every `npt-lab hash` call's `--manifest` and the loader's recordings, and the bites half off starts no bites-build call. Carries 83 (b) to (d) and 84 stay yours in `wasmStep`, as section 4's lane says, so det-u4.mjs inherits the exit-code rule, the one 300 s deadline shared from the first non-cargo call, and a stopped call taking every process it started. (5) New ops/runs/det-u4.mjs: it imports `wasmStep` and calls it with crates/lab/det-recordings.json, the bites half off, and its own folder under .gate/ (never the step's). It prints its lines and summary as check.mjs's main prints the step's, and exits 0 only on PASS. Section 4's no-retry rule for the wasm step binds it. (6) Run it once through the runner with -x (df first; the 4 GB floor), logging to .gate/u4r/det-u4.log. Its PASS line, every call's seconds and the two Wasm legs' seconds (W) go into your notes and report. If its calls other than cargo builds sum above 240 s, stop and report before going on. (7) Section 5 item 4 and return list item 9 now read: check.mjs's wasm step seconds on the three synthetic recordings (warm or cold, with uptime), det-u4.mjs's time beside them, and this ruling as the route.
- **Main session:** Sends the builder text and records the ruling on the card. In its own commit it amends carry 82's location words and carry 87's words in u4s-carries.md, each with a pointer to this ruling. At its next plan edit it amends u4-build-plan.md :800-802: det-u4.mjs runs at every merge that touches npt-sim, npt-proto, npt-room or npt-hash-wasm (the plan's list) or any crate on the scenario's path: npt-server, npt-client, npt-l1, npt-link, npt-bots, npt-lab (brief 2.1 (iii)). It adds det-u4.mjs to U4r's merge step (main-run.sh -x, after check.mjs) and to every later merge on that list. A SHA-256 failure there means that unit re-pins, never a retry. The review panel (section 12 item 7): infra-engineer reads det-u4.mjs and wasmStep's two options (carry 84's seat), and engine-architect counts the check.mjs changes (2.7).
- **Why:** As merged, the three legs calls (measured, hash-map, std-sin: check.mjs:2319, :2374, :2432) each regenerate every manifest entry (hash.rs:836-857). The file times show about 195.2 s per call for the two runner entries (142.6 + 52.6). So the step would take about 621 s + W warm and 650 s + W cold (fact check). The builder's "~170 s" was not a measured figure. Today's per-call limit would stop nothing (the largest call is 201.0 s), but carry 84 (a)'s shared deadline would stop the step at the hash-map legs call. (b) would bring it to about 259 s + W cold. That leaves about 40 - W s, and under section 4's no-retry rule a stopped call stops the step group. Every check.mjs run would also grow by about 200 s under the exclusive lock. U4h's TC B1 recording (carry 87) would push it over anyway. And every change on the scenario path would fail check.mjs's SHA until re-pinned. (d) alone drops "U4d2's step green on them" (plan :800-802; §4.15 :500-501). The clean half is enough: the bites half proves the comparer catches a planted divergence, and check.mjs shows that on SS-s1 every run, on the same code. Not for Mustafa: the plan gives this route "on the advisor's ok". No word or decision of his changes. Risk tier 1 (spike code and checkpoint text; check.mjs is the spike's own check, not a gate file).

### Point 2 (severity: Medium; to Mustafa: no)

- **Decision:** (a), accepted, and pinned by behaviour. The mirror keeps its radius copy. A test shows the copy equals the room's radius by placement, so a later change to the provisional RADIUS_CM cannot silently weaken r4. r4 is reported as what it checks: the room's radius for soldiers, hulls, parts and projectiles; the area flood, doors, the wall and events' radius-and-area test unchecked; events checked only for no own cosmetic shot. It is never reported as "spatial passed". (b) is not taken now: it is a seam change to U4e's merged crate for an audit that can wait. (c) is rejected: a second copy of the room's logic that would drift. The full spatial half becomes a follow-up.
- **Builder:** Point 2 (F1): (a). Keep mirror.rs's INTEREST_RADIUS_CM copy, cited to room.rs:29. Add one test that pins it to the room by behaviour. With scripted placements through check_placements, put one soldier exactly INTEREST_RADIUS_CM from client 0's soldier and one at INTEREST_RADIUS_CM + 1, both in an area connected to client 0's. Client 0's join FullState, decoded by the mirror, must hold the first and not the second. Show it failing by name under a plant of the copy (6,300), restored by Edit with the sha256 checked. If no placement within carry 49's bound puts both in a connected area, write that in your notes and skip the test; the report then says the copy is unpinned. In r4, count two things apart: the spatial records checked against the radius (soldiers, hulls, parts, projectiles), and those not checked (doors, the wall, and any record that tick's room records lack: mirror.rs:281 and :283 pass them today). Also keep a counter of every breach beside the 16 kept (mirror.rs:236-240), and report that count. The report's words: "spatial: the room's radius only, through a copy [pinned by test | unpinned]; the area flood, doors, the wall and the events' radius-and-area test not checked, since npt-room exposes neither its radius nor its flood (room.rs:29; interest.rs:86-147); events checked only for no own cosmetic shot." Never "spatial passed".
- **Main session:** Adds a card follow-up: r4's full spatial half (the area flood, doors, the wall, and events' radius-and-area test), checked against the room's own interest set through one read-only npt-room item, for the next unit whose lane opens crates/room, named in that unit's brief. In U4r's review it checks the report's r4 wording and counts.
- **Why:** The room's interest set is the radius AND the area flood (room.rs:904-966: soldiers, hulls and projectiles; parts by their hull; doors by sees_portal, the wall by reaches). The mirror checks the radius only, for four kinds (mirror.rs:244-295), skips doors and the wall, and tests events only for an own cosmetic shot (:421-430). For the four kinds, the code means it cannot flag a legitimate send. But it passes an in-radius entity in an unreachable area, any door or the wall, and any wrong event routing, so it is weaker than catalogue §6 item 5 ("spatial only inside that client's interest set or event test", netcode-proof-messages.md:251-254). Brief 2.6 says a needed npt-room change stops and goes to the advisor. A read-only accessor is cheap, but r4's bite (owner-to-all) and its sound radius half meet U4r's need. Reporting the gap honestly and deferring it is "address or explicitly defer", not a loosened bar. The catalogue holds no word of Mustafa's and is not a decision doc.

### Point 3 (severity: Medium; to Mustafa: no)

- **Decision:** U4r changes nothing in the series' form: SERIES.md:66-73's rows, marks.json -57 and the judge stay. One economy in U4r's own writer: -57's three series are computed and written only on runs whose header lists PM-0001-57. Those are -17's measured runs, which -57 shares, and -57's three bite runs. -17's and -18's bite runs, the pre-check's legs and the short test runs then carry none, which saves about 1.5-3 GB of the judged set. The judge reads a run for a mark only when a header lists it (eval.rs:28-34), so no check loses a value. The judged runs' disk is the main session's plan, met by approved methods only. It goes to Mustafa only if the set cannot fit that way.
- **Builder:** Point 3 (F3): the series' form does not change (SERIES.md:66-73, marks.json and the judge are not your lane). One change in your writer: npt-lab computes and writes -57's three series (pos_err_um, yaw_err_microdeg, pitch_err_microdeg) only on a run whose header's marks list PM-0001-57. That means -17's measured runs, which -57 shares, and -57's three bite runs. Every other run writes none of them; the mirror and the bots' view are unchanged. r2's -17 L1 P0 suite, whose marks list -57, shows them present, and one short run without -57 in its marks shows none. In your report, per profile, give the measured bytes per measured tick of each of the three logs from your runs (P0 d13-5, P2 d13-1, P3 d13-4, B1 from r2's short B1 run), and the size of a 10-minute run (18,300 ticks). The judged set's total is the main session's.
- **Main session:** Before the first judged set, it plans the disk in writing. (1) It projects the set's logs from the builder's per-profile rates times 7.1's runs (12 measured, the bite runs, the pre-check's six) against df at that time, keeping the 4 GB floor before every release build. (2) It frees space only by approved methods: the part-cleans of .gate/shared-target and of the main folder's cache by W1-TARGET item 3 as amended, between waves with no lane running. No log is deleted, moved off or replaced by a compressed copy. (3) It runs one set at a time, with df before each run, and stops if the next run's projected logs would take free space below 4 GB. (4) It measures npt-judge's peak memory and wall time (/usr/bin/time -v) on one P0 measured run's logs before judging a whole set, and judges each set's folder apart. The judge reads every log under --logs into memory (judge/src/lib.rs:37-46), and p99 copies and sorts the values (eval.rs:287-288). (5) If the set cannot fit, it stops and brings it to Mustafa under Decisions needed (his LEFTOVER-DELETE route: a narrow kit rule), never a one-off delete.
- **Why:** From the fact check's figures: P0 27,810, P2 12,005 and P3 6,868 bytes per measured tick across the three logs, so 508.9, 219.7 and 125.7 MB per 10-minute run. The 12 measured runs come to about 3.2 GB with B1 (B1 not measured), plus up to about 3 GB for 7.3's six bite runs at P0 rates, plus the pre-check. Free space now is 5,720,961,024 B (df -B1). Of that, .gate/shared-target (8.3G) and the main folder's cache (8.8G) can be cleaned only by the approved part-cleans. Run logs cannot be deleted by any session. Mustafa's LEFTOVER-DELETE (history.md:404-406): "leave them. 14 GB free is enough, and a one-off delete past the deny list isn't a habit we start; if it's ever needed, it becomes a narrow kit rule instead." His premise was 14 GB free. Changing the series' form would touch marks.json, SERIES.md's existing rows and the judge. That is outside U4r's lane (U4R-Q2 (c): new rows only), and his USAGE-SAVINGS keeps "everything that touches the judge, the marks or a pass bar" in the main session's full read (history.md:294-295). -57's p99 report needs every value. Section 4 rule 6's floors gate builds, not run logs. Not to Mustafa now, because approved cleans can likely make room; to him only if they cannot.

### Point 4 (severity: Medium; to Mustafa: no)

- **Decision:** New bite form: a client writer that files a tick it decoded whole as lost (decoded_hash null, snap_incomplete 1). On every client it is that client's first tick resolved whole in the measured phase. Id PM-0001-17/whole-as-lost, replacing PM-0001-17/refused-as-lost. The bite stays built. If r2's measured-phase sum is not 0 on every short L1 P0 run, no script arms it and the cause comes to me first (carry 62 (h)'s else branch). Whose words: the bite's wording "(a client that writes null and incomplete 1 for a refused group)" is the advisor's (U4j ruling R2.4, advisor-u4j.md:438-449), carried as carry 62 (h) and quoted at brief 2.5 (:643). The advisor amends it here to "(a client writer that files a tick it decoded whole as lost: null and incomplete 1)". The check's JSON stays word for word (value 0, window measured, setup true), and so does the main session's role (U4R-Q2 (c)).
- **Builder:** Point 4 (F4): the bite becomes a client writer that files a tick it decoded whole as lost (decoded_hash null, snap_incomplete 1). On every client it acts on that client's first tick resolved whole in the measured phase; every other tick is filed as the clean writer files it. Its id is PM-0001-17/whole-as-lost, in place of PM-0001-17/refused-as-lost (scenario.rs:200; `refused_as_lost` in logs.rs:132-179 and :275). Bites build only, with its literal in npt-lab's sources only. Its unit proof, in the bites build: a short L1 P0 run with the bite gives a measured-phase sum of tick.s.snap_incomplete equal to its client count, and the clean run gives 0; name the test in your report. Keep one bitten short L1 P0 run's logs (bites build, header marks listing PM-0001-17) under .gate/u4r/whole-as-lost/ and name the folder in your report: the main session runs the judge on them. If r2's measured-phase sum on any short L1 P0 run is above 0, no run script arms the bite, and the cause goes to the main session first.
- **Main session:** Notes the amended bite wording under carry 62 (h) in u4s-carries.md, with a pointer to this ruling, and records that brief 2.5's row and :643 read this way. After r2's evidence (a sum of 0 on every short L1 P0 run), it adds the check (its JSON unchanged) and the bite entry "PM-0001-17/whole-as-lost" to -17's L1 P0 part in marks.json in its own commit, before the counted run of U4r's merge step. It then runs `npt-judge --only PM-0001-17` on the builder's .gate/u4r/whole-as-lost/ logs and shows the setup check FAIL by name (U4R-Q2 (c)). If r2's sum is not 0, it brings the cause to me first.
- **Why:** The client files a sentinel only on a decode error, a duplicate with other bytes, a group-count or baseline-age disagreement, no complete baseline, an apply error, a bad header, or a second hash for the tick (client/src/lib.rs:473-749). L1 P0 has 0 delay, 0 jitter, 0 loss and only the 1 % reorder step, so no link fate there causes any of them. The runs had 0 sentinels in smoke1 (21,552 hashes), d13-5 (86,352) and SS-P0-s4's run (431,904). So `refused_as_lost && h == 0` (logs.rs:275) changes no record, the setup check stays 0 on the bite run, and U4R-Q2 (c)'s demonstration ("setup check FAIL by name") could not be made. Wired into -17, that bite would be evidence that does not hold. The check exists because "a refused group can hide only as 'lost and counted'. On L1 P0 (no loss) that count must be 0" (R2.4). Filing a tick decoded whole as lost is exactly that outcome, made on purpose. Not Mustafa's: the words amended are the advisor's, and no pass bar or check value moves.

### Point 5 (severity: Low; to Mustafa: no)

- **Decision:** Accepted with the cite, and pinned where the room's pub items allow. scenario.rs's ROOM_HISTORY_MS stays a copy of room.rs:33. A behaviour test pins it at the ring's edge against npt-sim's pub HitHistory::for_ms(...).capacity(), if npt-room's pub items can place a command there; if not, the copy is reported as unpinned. hash.rs `check` (carry 82's reader) also compares each regenerated recording's header (first_tick, span_ms, tick count) with its entry, so a manifest field that disagrees with the bytes fails by name. No npt-room pub item now. A §U4h carry: when U4h sizes the ring and changes HISTORY_MS, it makes the span readable outside npt-room, the copy gives way to it, and the runner entries are re-pinned.
- **Builder:** Point 5 (F5): keep ROOM_HISTORY_MS (scenario.rs:54-56, cited to room.rs:33). Pin it by behaviour if npt-room's pub items reach the ring's edge. A test fills a Room's ring, then gives one client a firing command whose view tick is the oldest the ring should hold by HitHistory::for_ms(ROOM_HISTORY_MS)'s capacity (npt-sim, pub: sim/src/history.rs:29, :42), and another one tick older. The first must get a Rewind and the second must count in rewinds_refused. Show it failing by name under a plant of the copy (2,100), restored by Edit with the sha256 checked. If no pub item lets a command carry those view ticks, write that in your notes and skip the test; the report then says the copy is unpinned. Also, in hash.rs `check`: decode the regenerated bytes' header (`Recording::decode`, sim/src/replay.rs:320) and refuse, by name, a first_tick, span_ms or tick count that differs from the entry's, for every entry, with one test case.
- **Main session:** Adds a §U4h carry line to u4s-carries.md: "When U4h changes npt-room's HISTORY_MS (it sizes the ring: §4.9; carry 28), it makes the span readable outside npt-room (a pub constant or getter), npt-lab's ROOM_HISTORY_MS gives way to it, and the runner entries are re-pinned (the advisor's U4r build ruling, point 5)."
- **Why:** HISTORY_MS is private (room.rs:33, used at :446), and the room exposes no ring capacity. s5 compares the copy with itself (scenario.rs:1590-1592). A runner entry's manifest span_ms is read (hash.rs:297) but never used or compared (generate :330-344; check :355-365 compares length and SHA-256 only). A copy that is too small fails closed in replay only if a Rewind is that old, which the brawl's view times (newest tick, fraction 0) never are. A copy that is too large passes unseen, against carry 14's U4r half ("the ring the room used"). The drift risk lands at U4h, which changes room.rs:33. Low: a cited copy of a provisional constant, correct today.

### Point 6 (severity: Low; to Mustafa: no)

- **Decision:** Accepted: the World's dump is a sound source for the shot band's figure and D13's shot rates. It is the authoritative state: World::dump is pub (world.rs:1018), its events section is cleared at each step (world.rs:2100) and written per event (:1995-2017), and the canonical hash includes the event list (seam.rs:549-550). One unit test pins the reader against SimCore::step's own returned events, which also catches a later change to the dump's layout. No band, floor or check moves. SERIES.md:91 and :94 name npt-room as the writer while npt-lab computes these figures; that is a follow-up, and no row changes in U4r.
- **Builder:** Point 6 (F2): the World's dump stays the source. Add one unit test that pins logs.rs's dump reader: step a World directly over a short seeded run with shots, hits and kills, and check that the reader's counts at each tick equal the shots, hits and kills in SimCore::step's returned events (sim/src/seam.rs:546-548). Show it failing by name under a plant of the reader (kind read one word off), restored by Edit with the sha256 checked. The report says these figures are counted only in logged runs (a --logs 0 run counts none). It also says that SERIES.md:91 (reliable_events_count) and :94 (shots_count) name npt-room as the writer, while npt-lab computes them from the World's dump into the room process's log; no row is changed.
- **Main session:** Adds a card follow-up: the writer column of SERIES.md:91 and :94, for the next marks/ edit that is the main session's. Reads the reader's test in U4r's review.
- **Why:** Room::tick keeps the stepped events in a private field (room.rs:813-818, :417), and no pub getter exists. The dump is the World's own canonical serialization. Each Shot, Hit and Kill is pushed once (world.rs:1550-1553, :1620, :1638), the World is stepped once per room tick (room.rs:814), and room_second sums once per second (logs.rs:584-601). The sections before events are fixed-size pools, so the cached index holds today, and the test guards it from now on. Mustafa's words (history.md:338-339): "The 9.5 floor (PM-0001-2) does not move." Nothing here moves it, or the shot band (marks.json:285-305, 665-735).

### Point 7 (severity: Low; to Mustafa: no)

- **Decision:** Accepted as built. The brawl aims from the room-side frame of that client's newest tick: the mirror, fed with the room's datagrams before the link. The report states this. npt-client exposes no decoded record (R16), so a client-side view would need a seam change. The mirror view is deterministic and independent of the link's fates, so the twin test and the recordings hold. No U4r mark judges the brawl's aim, and the open-loop scripts read no view. A follow-up goes to U6's brief, where bots run apart from the room and so need the client's own view.
- **Builder:** Point 7: accepted as built; no change. The report's words: "the brawl aims from the room-side frame of that client's newest tick (the mirror, fed before the link: scenario.rs:916-921, mirror.rs:213-225), which may hold entities from datagrams the client lost or refused, or a baseline it never completed; no U4r mark judges the brawl's aim; the open-loop scripts read no view."
- **Main session:** Adds a card follow-up for U6's brief (npt-bots' npt-net glue, plan :897-898). There the bots run beside no room-side mirror, so the brawl's view must come from the client's own decoded state: an npt-client read item (R16) or U4g's Interp.
- **Why:** ClientEndpoint's pub API returns no frame or records (client/src/lib.rs:253-415). The mirror is fed before server.send, so before L1's fates (scenario.rs:830-852; mirror.rs:1-4). Only Kind::Brawl reads the view (bots lib.rs:204-207). The judged figures (-17, -57, -18, PM-0001-2's bands) do not judge aim fidelity. The shot rate comes from fire held on every command, and D13 shows about 698 shots per measured second at P2 lead 5, inside the 665-735 band.

### Point 8 (severity: none; to Mustafa: no)

- **Decision:** A fact for the report; nothing follows. (b1) is in force, so no judged run has a duplicate stamp, and D13's stop rule did not fire. The measured excess over the arithmetic only makes (b1) more needed. The arithmetic was a stated approximation: jitter only, one datagram per snapshot assumed. The notes' line on the run length is corrected.
- **Builder:** Point 8: a fact for the report; nothing follows. Report D13's table as your notes :178-184 give it, and add the following. Run (1)'s 12,290 steps without a command equal run (2)'s 12,290 duplicate stamps. Run (2)'s 12,489 late are those 12,290 plus 199 late by arrival (run (1): 213). Against U4R-Q14 (b)'s about 10 % at P2 and 15 % at P3, the measured figures are 14.0 % at P2 (run 2) and 20.4 % steps without a command at P3 (run 4: 17,889 of 87,541). The cause of the extra points was not checked. Correct your notes :175: the length was a 10 s warm-up (300 ticks) and 50 s measured (1,500 ticks), plus 0 to 35 ticks after (d13.log: 1,833, 1,833, 1,833, 1,835 and 1,800 ticks).
- **Main session:** Nothing beyond reading D13 in U4r's review as a fact.
- **Why:** d13.log:19-31 match the notes field for field. (b1) holds by construction: the bots give no command while the newest tick has not moved (bots lib.rs:187-190), and --every-step is refused with marks (scenario.rs:1290-1292). Runs (1), (4) and (5) were 0.28 %, 0.19 % and 0 % late with no press refusal, so the stop rule (brief :1840-1841) did not fire. The arithmetic (brief :1751-1757: integral of p(1 - p) = 1/6, times jitter over the tick period) covers jitter only. Run (3) at lead 3 (87.3 % late, 479 shots per second) confirms U4R-Q14 (a1)'s lead. The notes' "1,800 ticks measured" was wrong: scenario.rs:1246-1247 multiplies seconds by TICK_HZ (30). No defect.

### Point 9 (severity: Medium; to Mustafa: no)

- **Decision:** Approved with three fixes and one statement. (a) --lead and --every-step are also refused with --record: a recording's settings are its manifest entry's, and Kind::Runner has neither key. (b) A --logs 0 run's summary prints "not counted (--logs 0)" for every figure taken from the logs, never 0. Today SS-P0-s4's no-log L1 P0 run prints "measured incomplete 0", which reads like carry 62 (h)'s trigger figure. (c) The recordings' wall times are reported labelled as taken before the no-log memory fix. det-u4.mjs's per-call seconds give the time after it. The recordings themselves stand, since they regenerated byte for byte after the fix. (d) The report states that a no-log run does not check carry 75's admission bound. No code change for (d): the recordings' bytes do not depend on client-side event decoding.
- **Builder:** Point 9: (a) refuse --lead and --every-step with --record too (a recording's settings are its manifest entry's, and Kind::Runner has neither key: hash.rs:205-212), with one parse-test case each. (b) In a --logs 0 run, the summary line prints "not counted (--logs 0)" in place of every figure it takes from the logs' counts or from room_tick: shots per measured second, measured incomplete, decided lost at the stop, client hashes and sentinels, and the -57 and PM-0001-2 figures. Never print 0 for them. The counts that need no logs (commands, steps without one, duplicates, late, press refused, buffer full, rewinds refused, stale) stay. (c) Report 200.1, 104.2 and 167.2 s labelled "scenario --record, before the no-log memory fix (which build first held it: not kept)", and det-u4.mjs's per-call seconds (point 1) as the regeneration time after it. No re-recording: the regenerated bytes equal the recorded ones (regen-all.log:23-24; regen-precheck.log). (d) The report states that a --logs 0 run does not check carry 75's admission bound (scenario.rs:804-808, :884-888); no change.
- **Main session:** Nothing beyond reading the labels in U4r's review. For 7.1's stop rule it reads SS-P0-s4's rewinds_refused, which is counted with or without logs (scenario.rs:1022). It never reads a no-log run's incomplete or shot figures.
- **Why:** From the fact check: the flags are refused only with marks (scenario.rs:1290-1292). A recording made with them would fail closed at the SHA-256 check, so (a) is a Low guard. A no-log run skips room_tick (scenario.rs:884-888) and prints 0 for shots and measured incomplete (rec.log:7, :14, :21). Carry 62 (h)'s evidence is r2's logged short L1 P0 runs (brief :1176-1178), and a P0 recording run printing "measured incomplete 0" could be read as one: that misreading is the Medium. The fix (scenario.rs:833-835) changed memory, not bytes (s5: the bytes do not depend on the logs). Later regeneration took about 142.6, 52.6 and 108.1 s, from file times, which is not a measured wall time.

### Point 10 (severity: Low; to Mustafa: no)

- **Decision:** Four earlier Lows recorded as follow-ups; none triggers a push on its own (Mustafa, history.md:340: "A Low never triggers a push."). (1) The fold's rewrap split three code spans across lines: D1's main-run.sh command, `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)` and F6-3's new span (u4r-fold-check2.md:48-50). The words are unchanged. Goes to the card's follow-ups, fixed only if the brief is re-stamped for another reason. (2) My own premise on npt-net's tree was wrong (advisor-u4r-fold.md FQ6 (b): npt-net was already in npt-lab's builds through npt-server, server/Cargo.toml:29). Goes to U4r's report, return list item 15: U4R-Q1's line names the correction, as the brief's Q1 third cell already does. My closed rulings stay as written. (3) Two stale l1/src/lib.rs cites in the brief's R6 row (:1488): BITE_IDS ":752-758" is now :746-751, and ":764-773" (the model's own bites) is now the block :753-766 (doc :753-755, cfg :756, array :757-766; the task's :753-757 names its head). Goes to the message to copy 2, as data, and to the card's follow-ups. (4) Section 0's "How rev 2's df8975b cites were re-pointed" bullet lists crates/lab/Cargo.toml and crates/bots/Cargo.toml among files unchanged since df8975b. In rev 3.2 that bullet is :217-218 (the task's :210-211). But :184-190 says spikes/ is 4e91830's, and L3 changed both manifests (lab/Cargo.toml:32; bots/Cargo.toml:22). In effect, npt-net is now bots/Cargo.toml:25, not the :24 cited at C26 (:173) and in U4R-Q1's record (:1565). Goes to the message to copy 2, as data (read cites into those two files at the head, as 2.2's F6-1 words do), and to the card's follow-ups.
- **Builder:** For your reading of the brief (data, no change of yours). R6's cites are l1/src/lib.rs BITE_IDS :746-751 and the model bites' block :753-766. Section 0's list of files unchanged since df8975b is stale for crates/lab/Cargo.toml and crates/bots/Cargo.toml: L3 (4e91830) changed both, so read their lines at the head (npt-sim at bots/Cargo.toml:22, npt-net at :25; npt-bots at lab/Cargo.toml:32).
- **Main session:** Writes the four follow-ups on the card with these homes, includes (3) and (4) as data in the message to copy 2, and lists (2) for U4r's report's item 15. No push for any of them.
- **Why:** Each is wording or a cite with no effect on a rule, a pass bar or a word of Mustafa's. His rule (history.md:340) and the review budget make each a follow-up. Line numbers were checked in rev 3.2 (sha256 59d570b8, 1,909 lines) and in the snapshot's l1/src/lib.rs and bots/Cargo.toml.

### Further findings

- **Medium:** r4's reported spatial count overstates what was checked. mirror.rs:261 counts every entity record (`audit.spatial += 1`), but doors and the wall fall to `_ => None` (:281), and any record that tick's room records lack passes `if let (Some(me), Some(at))` (:283) unchecked. So the notes' "spatial 374,694/371,513/347,972 entity records" (u4r-build.md:236-237) include records that got no radius test. Also the breach list stops at 16 (mirror.rs:236-240), so the bite's "16 breaches" (b11-r4r8r9.log:69) is the cap, not a count. **Fix:** Folded into point 2's builder text: count checked and unchecked spatial records apart, keep a full breach counter beside the 16 kept, and use the report's stated words.
- **Medium:** The judge's memory and time on the judged set are not measured. npt-judge reads every log under --logs into memory (judge/src/lib.rs:37-46; npt-log's Log holds every TickRecord with its BTreeMap<String, Vec<i64>> lists: log/src/lib.rs:104-121, :534-536), and p99 copies and sorts a part's values (eval.rs:287-288). At P0, -57's series hold about 5,700 values per tick (fact check, smoke1 tick 300), about 103 M per 10-minute run. As i64 that is about 0.8 GB per P0 run, several GB for a set's folder, on a 16 GB machine. **Fix:** Folded into point 3's main-session item (4): measure peak memory and wall time on one P0 run's logs first, and judge each set's folder apart. Point 3's writer change keeps -57's series out of runs that do not list -57.
- **Low:** Line numbers in the task's point 10 text differ from rev 3.2 (59d570b8). Section 0's bullet is :217-218 (not :210-211), and "is now 4e91830's" is in :184-190 (not :182-186). The R6 row's model-bite block runs :753-766, of which :753-757 is the head. A side effect of point 10 (4): npt-net is at bots/Cargo.toml:25, while C26 (:173) and U4R-Q1's record (:1565) say :24. **Fix:** Point 10's homes use the corrected lines. No change to the brief, a closed record at its stamp; the card's follow-up names them.

## The fact check

### Point 1

- **held:** The `npt-lab hash --manifest crates/lab/recordings.json` call with the two runner entries appended took 201.0 s, regenerated all five entries as the manifest's, and its native legs compared 45,000 ticks as equal. Evidence: .gate/u4r/regen-all.log: 'regenerated' lines for SS-s1, ES-s2, TC-s3, SS-P2-s1 and ES-P3-s2; 'native leg 1 against native leg 2: 45000 ticks compared, 0 mismatching'; 'real 3m20.992s'. Release measured build (cargo build --release -p npt-lab, same log). File times in .gate/u4r/regen-all/ give the split: the three synthetic files at about 0.1 s, SS-P2-s1.rec 142.6 s after the start, ES-P3-s2.rec 52.6 s after that, native-*.lines 5.7 s later.
- **held:** SS-P0-s4 was regenerated byte for byte through `npt-lab hash` on precheck-recordings.json in 110.1 s. Evidence: .gate/u4r/regen-precheck.log: 'regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a..., as the manifest's', '9000 ticks compared, 0 mismatching', 'real 1m50.068s'. File times: about 108.1 s of regeneration, about 1.9 s of legs.
- **held:** The `wasm` step makes three legs calls: the measured one, then hash-map and std-sin in the bites build. Evidence: check.mjs (unchanged in the snapshot): wasmStep at :2254; 'npt-lab hash' at :2319 (art.lab, no --only); 'hash-map native legs' at :2374 (art.labBites, --only rec0 = entries[0] = SS-s1); 'std-sin native legs' at :2432 (art.labBites, --only the scan's first catch). The std-sin call runs because crates/lab/std-sin-scan.txt holds 'catch SS-s1 2 vehicle[8].pos.y', a manifest recording (check.mjs:2420-2432).
- **held:** hash.rs `legs` regenerates every manifest entry whatever `--only` selects; `--only` selects only what is replayed. This is U4d2's merged behaviour. Evidence: Snapshot hash.rs:836 `let replayed = selected(&entries, only...)` and :855 `regenerate_all(&entries, ...)` over the whole manifest. selected() at :484-496. The leg processes only read the files and never regenerate (serve, :965-975). At the base commit 2fbec77 it is the same: hash.rs:751 selected and :770 regenerate_all(&entries). The brief's lane keeps `hash` 'as merged but for carries 82 and 83' (brief :993-997).
- **not held:** Builder's estimate: about 33 s + 3 × ~170 s for the step with the runner entries. Evidence: Arithmetic from the measured figures (computed field). What is true: The ~170 s per call is not a measured figure. Measured: the hash call adds 197.9 s (201.0 − 3.1), and each bites legs call would add the runner regeneration, 195.2 s (142.6 + 52.6). The bites build's regeneration speed was not measured. That gives about 621 s + W warm. Both the estimate and the measured figure are well over 300 s.
- **not held:** The 300 s is the step's budget (WASM_CALL_LIMIT_MS, carry 84). Evidence: check.mjs:2211-2212 `WASM_CALL_LIMIT_MS = 300_000`, commented 'The wasm step's time limit on each of its calls but its cargo builds'. :2258-2266 gives each non-cargo call its own 300 s and cargo calls none. No step-level deadline exists. Carry 84 (a) (u4s-carries.md:396-401) makes it one deadline shared by the non-cargo calls, counted from the first of them, at U4r's change to the step. That change is not yet made: check.mjs is not among the snapshot's 11 files. The '5-minute budget' itself comes from u4-build-plan.md:69 (T9) and :801. M9 judged the step's printed seconds, builds included (advisor-u4d2-m9.md:117-118, :127). What is true: Today the limit is per call, so no call with the runner entries would be stopped (the largest measured call is 201.0 s). Only the step's total would exceed 5 minutes. Under carry 84 (a)'s shared deadline the step would be stopped at the hash-map legs call. That deadline counts non-cargo calls only, while the printed step seconds include the builds: two different bases.
- **held:** The base is 33.1 s warm and 61.6 s cold. Evidence: .gate/u4d2-merge/m5-counted.log:170 'PASS wasm 33.1 s': builds 5.4+5.2+1.2+1.2 = 13.0 s; non-cargo calls sum to 19.9 s. .gate/u4d2-merge/cold-run2.log:23 'PASS wasm 61.6 s': builds 41.8 s; non-cargo 19.6 s. Also, the most recent warm figure on the spike's current code (4e91830, which has the same spikes/ as 2fbec77) is 35.8 s (.gate/l3-manifests/counted.log:172: builds 15.0 s, non-cargo 20.8 s). All three are on the three synthetic recordings.
- **held:** Not measured: the step itself with the runner entries, and r5's R. Evidence: No log of a `check.mjs --only wasm` run is among the logs in .gate/u4r/. r5 is not built (notes :277). D13's rate of about 61 ticks/s (notes :190) is rough: about 1,833 ticks in about 29 s per run (d13.log 20:13:16-20:15:42 for five runs).
- **held:** The builder held the two runner entries out of recordings.json; the three synthetic entries carry `"kind": "synthetic"`; SS-P0-s4 is a runner entry in precheck-recordings.json; the held entries are verbatim at notes :225-230. Evidence: Snapshot crates/lab/recordings.json holds 3 entries, all of kind synthetic. precheck-recordings.json holds SS-P0-s4, kind runner, generator 0. The bytes and SHA-256s of the notes' held entries equal regen-all.log's (SS-P2-s1 16937652 5b79abb2...; ES-P3-s2 16929501 fad8f67d...). hash.rs:1409-1423 (h5) asserts the three synthetic entries and the pre-check runner entry.
- **held:** Option (a), RR-L4's remedy, is crates/hash-wasm and outside U4r's lane. Evidence: advisor-u4d2-r1.md:209-212: 'replay_to feeds the whole recording, so the drain is quadratic; the Wasm leg hashes every tick twice ... feed replay_to by chunks ... cache the hash per step in hash-wasm's State'. It acts only on the Wasm loader calls, not on hash.rs regenerate_all. The lane allows only loader.mjs's header sentence (brief :999-1000, :1010).
- **not held:** Option (b)'s estimate: about 33 + 170 s plus the two runner entries' Wasm replays. Evidence: Computed field. What is true: From the measured figures: about 230.8 s + W for the whole step warm (M5 basis), 233.5 s + W on the L3 basis, 259.3 s + W cold. The measured hash call is 201.0 s by itself, and the rest of the step is about 30 s.
- **held:** Option (c) cites plan :800-802 for det-u4.mjs. Evidence: 'plan' in the brief is u4-build-plan.md (brief :193). u4-build-plan.md:800-802: 'U4d2's step green on them (in check.mjs if the step stays within 5 minutes, else through ops/runs/det-u4.mjs at every merge that touches sim, proto, room or hash-wasm, on the advisor's ok)'. plan.md:800-802 (the file the task names) is an unrelated cost table. The brief at :1543 (U4R-Q2 (ii) and (iii)) says the same route.
- **could not check:** The machine the 300 s is judged on (this container or CI). Evidence: No document names a machine. CI runs no check.mjs: .github/workflows/gate.yml runs tools/gate.mjs, and toolchain.yml runs `cargo test -p sim -p protocol ...`; neither has a spikes/ or check.mjs line. So every measured figure of the step is this container's: 4 vCPU Intel Xeon @ 2.30 GHz, 16 GB (the machine block in smoke1/lab.jsonl's header). The 61.6 s cold figure is INF-1's from this container (brief :1789-1791).
- **Computed:** Base (M5 warm, m5-counted.log:170): builds B = 13.0; non-cargo N0 = 19.9 (hash 3.1; Wasm 2.8+0.9+0.1; hash-map legs 2.2; hash-map Wasm 2.6+2.6; compares 0.5+0.3; std-sin legs 2.1; std-sin Wasm 2.6; std-sin compare 0.1). Runner regeneration G = 142.6 + 52.6 = 195.2 s (file times). W = Wasm leg 1 on SS-P2-s1 and ES-P3-s2: NOT measured (the synthetic SS-s1 and ES-s2 of similar size took 2.8 s and 0.9 s). The bites build's regeneration time is not measured; G is assumed. Today's code with the runner entries appended: N = 19.9 + (201.0−3.1) + 195.2 + 195.2 + W = 608.2 + W; step warm = 13.0 + 608.2 = 621.2 + W (L3 basis: 20.8 + 197.7 + 390.4 + 15.0 = 623.9 + W); cold = 41.8 + 19.6 + 197.9 + 390.4 = 649.7 + W. Under today's per-call limit nothing stops (max call 201.0 < 300). Under carry 84 (a)'s shared deadline: 201.0 + 3.8 + W before hash-map legs, which needs about 197.4 s, gives about 402 + W > 300, so the step stops. (a) RR-L4 alone saves at most the Wasm calls (3.8 + 2.6 + 2.6 + 2.6 = 11.6 s, + W): N ≥ 596.6 s, step ≥ 609.6 s warm. Regeneration stays 3 × about 195 s. (b) Regenerate only --only's entries: the bites calls stay at 2.2 and 2.1 s; N = 19.9 + 197.9 + W = 217.8 + W; step warm 230.8 + W (L3 233.5 + W), cold 41.8 + 217.5 + W = 259.3 + W. Margin against 300: as carry 84's non-cargo deadline 82.2 − W; as the whole printed step 69.2 − W warm (66.5 − W L3), 40.7 − W cold. (c) The check.mjs step stays at 33.1 warm / 35.8 warm (L3) / 61.6 cold. det-u4.mjs's time is undefined by any file; if it runs today's three legs calls over five entries it is about 608 + W non-cargo, and at least one regeneration (195.2) plus legs. (d) The step stays at 33.1 / 35.8 / 61.6 s; no check.mjs step then regenerates or replays the runner entries (the wasm step never reads precheck-recordings.json: its 'about' text; brief :415-418).

### Point 2

- **held:** npt-room's RADIUS_CM is private. Evidence: room/src/room.rs:29 `const RADIUS_CM: i32 = 6_400;` (no pub).
- **held:** AreaFlood's methods are pub(crate). Evidence: interest.rs:77 `pub struct AreaFlood` (re-exported at room/src/lib.rs:27). Its methods are all pub(crate): new :86, update :99, reaches :140, sees_portal :147. The type is public; nothing on it can be called from outside. ClientSlot's pub methods (room.rs:195-295) expose no relevant set.
- **held:** The mirror's spatial rule checks a copy of the radius (mirror.rs INTEREST_RADIUS_CM, 6,400 cm), not the flood. Evidence: mirror.rs:28 `INTEREST_RADIUS_CM: i64 = 6_400`. audit_records at mirror.rs:244-295: Soldier and Vehicle by grid[0..2], Projectile by grid[1..3], VehiclePart through its hull (grid[5]), all taken from the room's records of that tick, against the client's own soldier; breach at :288. There is no flood test.
- **held:** What the room's interest set is. Evidence: find_relevant, room.rs:904-970. Soldiers, hulls and projectiles are relevant when point() is within RADIUS_CM² AND `flood.reaches(me.area, r.area)` (:924-935; point() at :345-351 uses the same grid fields as the mirror). Parts are relevant whenever their hull is (:953-957). Doors are relevant by `flood.sees_portal` and the wall by `flood.reaches`, with no radius (:958-966). Snapshot Add and Update deltas come only from c.relevant (room.rs:1240-1252); other baseline entities are sent as Remove (:1254-1269). FullState is built at the same tick t from c.relevant (room.rs:1106-1115).
- **held:** Can the radius-only rule flag a legitimate send? Evidence: By the code it cannot, for the four kinds it checks. Every Add or Update of a soldier, hull or projectile passed `dist2 <= r2` on the same fields of the same tick's records. A part is relevant only with a relevant hull, and the mirror checks the hull. The mirror runs each tick with that tick's records (scenario.rs:826-832), and full states carry tick t. Runs: r4 had 0 breaches on 3 seeds (b11-r4r8r9.log:31-34), and every smoke, D13 and recording run had 0 breaches.
- **held:** Can it pass a wrong send? Evidence: mirror.rs:270-282 does not check doors or walls at all (`_ => None` at :281), and there is no flood test. What is true: Yes. It passes: (1) a soldier, hull, projectile or part inside the radius but in an area the flood does not reach; (2) any door or wall, which are not radius-bound in the room and not checked by the mirror; (3) any event, since the event test checks only 'no own cosmetic shot' (mirror.rs:421-430), not the room's sees() routing (room.rs:975-1005). Also, the breach list is capped at 16 (mirror.rs:236-240), so the r4 bite's '16 breaches' (b11-r4r8r9.log:69) is the cap.

### Point 3

- **held:** On the debug smoke run (SS P0 seed 1, 48 clients, 450 ticks): lab.jsonl 10,961,471 B, room.jsonl 881,985 B, client.jsonl 492,198 B. Evidence: ls .gate/u4r/smoke1; b5-smoke1.log:72 '450 ticks (0 after)'; header profile debug.
- **held:** -57's values cost about 24 KB a tick. Evidence: For P0 only. The lab.jsonl tick lines hold only s.pos_err_um, yaw_err_microdeg and pitch_err_microdeg (one parsed line, smoke1). Measured-phase bytes per tick: smoke1 P0 24,510; d13-5 P0 24,768. What is true: This holds for P0 only. P2: d13-1 8,936 and relp2 8,590. P3: d13-4 3,818. P2 at lead 3 (d13-3): 15,000 whole-run. B1: no run.
- **held:** About 440 MB per 10-minute run (18,300 ticks). Evidence: For lab.jsonl at P0: 24,510 × 18,300 = 448.5 MB (smoke1 rate); 24,768 × 18,300 = 453.3 MB (d13-5). What is true: P0 only. All three logs at P0 come to about 509 MB; P2 about 220 MB; P3 about 126 MB (computed).
- **not held:** The notes' 'about 60 positions per client per tick, about 50 M values per 10-minute run'. Evidence: smoke1 measured tick 300: 2,887 positions, 2,645 yaws and 144 pitches. What is true: About 60 positions per client holds. 50 M is the positions alone (60 × 48 × 18,300 = 52.7 M). With yaw and pitch there are about 5,700 values per tick at P0, about 103 M per run.
- **held:** SERIES.md:66-73's rows are -57's series. Evidence: Rows 66 (pitch_err_microdeg), 67 (pos_err_um) and 73 (yaw_err_microdeg) are -57's. The range also holds 68-72: relevant_starved_count, rewind_ns, snap_datagram_bytes, snap_payload_bytes and snap_wire_bytes, other ids' rows. marks.json:4008-4100: -57's parts L1 P0, P2, P3 and B1 (min 3 each), max checks (window all) and a p99 report.
- **held:** The judged runs that carry -57's series: count and length (brief 7.1). Evidence: Brief :1153-1167: -57's four parts run 'on -17's P0/P2/P3/B1 runs', seeds 1-3 at 10 minutes each, so 12 measured runs; -18's B1 measured runs are those same B1 runs. Bite runs: 'one run per bite at seed 1 ... with the part's own settings' (:1147-1150). For -17: tick16 and client-rounding; for -57: pos-4cm, yaw-12bit and pitch-8bit, each its own bite run; for -18: no-fallback. Table rows read 'as above' per part, so the bite-run count depends on reading: up to 2×4 + 3×4 + 1 = 21 if per part. Every logged run writes the series (logs.rs:358-364).
- **held:** The judged runs are the main session's, after the merge (brief :328). Evidence: Brief :327-328 'the run scripts for the judged runs (section 7), which the main session runs after the merge'; :1140-1144.
- **could not check:** Free disk now 5.7 GB. Evidence: That was a reading at the time of writing. Now df -h /home/user gives 5.6G available (5,998,694,400 B); builder copy 2 is still writing. du -sh .gate/shared-target gives 8.1G; .gate as a whole 22G; .gate/u4r 291M.
- **held:** Section 4 rule 6's floors: 4 GB before wasm, doc and clippy-wasm32 builds and `scenario --record`, 2 GB otherwise. Evidence: Brief :968-971 says 'At least 4 GB free before any wasm32 or release build, which every check.mjs run is (clippy-wasm32, doc, the wasm step) and every scenario --record or r5 timing build; at least 2 GB before every other build ... never clean anything'. What is true: Held in part. The 4 GB floor covers every release build and r5 timing build too. The floors gate builds by the builder; they say nothing about the size of the main session's run logs.
- **held:** Mustafa's LEFTOVER-DELETE: 'leave them'. Evidence: history.md:404-410, verbatim: "leave them. 14 GB free is enough, and a one-off delete past the deny list isn't a habit we start; if it's ever needed, it becomes a narrow kit rule instead." His stated premise was 14 GB free; 5.6G is free now.
- **Computed:** Measured-phase bytes per tick for all three logs: P0 24,768 + 1,946 + 1,096 = 27,810 (d13-5); P2 8,936 + 1,987 + 1,082 = 12,005 (d13-1); P3 3,818 + 1,988 + 1,062 = 6,868 (d13-4). Per 10-minute run of 18,300 ticks (warm-up 300 + measured 18,000; after phase 0-35): P0 508.9 MB (lab alone 453.3), P2 219.7 MB (lab 163.5), P3 125.7 MB (lab 69.9), B1 not measured. The 12 measured runs: 3 × (508.9 + 219.7 + 125.7) = 2,562.9 MB plus 3 × B1 (if B1 ≈ P2: +659 MB, about 3.2 GB in all). Each bite run adds its profile's per-run size (up to 21 bite runs under the per-part reading; their count is not fixed by 7.1). Against 5,998,694,400 B free now (5.6 GiB) and 8.1 GB in the shared target.

### Point 4

- **held:** In a correct L1 P0 run no group is refused. Evidence: The client gives the sentinel only on: a decode error (client/src/lib.rs:473-487), a duplicate group with other bytes (:690-701), a group count or baseline age that disagrees with the tick's other groups (:703-710), no complete baseline held (:711-730), an apply error (:741-749), a bad header (a tick before the first header and at least the ring behind, :501-530), or a second, different hash for the tick (:608-622). L1 P0 has 0 ms delay, 0 jitter and 0 loss, only the 1 % reorder step (l1/src/lib.rs:166-180; smoke header reorder_step_permille 10), and duplication is not built (l1 :75). None of these causes follows from a P0 link fate alone. Runs: 0 sentinels in smoke1 (21,552 hashes), d13-5 (86,352) and SS-P0-s4's recording (431,904): b5-smoke1.log:72, d13.log:31, rec.log:21. This was not proven impossible.
- **held:** As worded, the bite has nothing to act on and cannot fail the P0 setup check on its own; it is built as `refused_as_lost` in logs.rs. Evidence: logs.rs:275-282 `let as_lost = self.refused_as_lost && h == 0;` turns a filed sentinel into Res::Lost, written as decoded_hash null and snap_incomplete 1 (logs.rs:339-355). With 0 sentinels no record changes, so the check's sum stays 0 (short P0 runs: 'measured incomplete 0' in b5-smoke1.log:72 and d13.log:31). The id PM-0001-17/refused-as-lost is in scenario.rs:200 (bites build) and wired at :218 and :699. Nuances: it acts on every sentinel, including bad headers and second-hash conflicts, not only refused groups. It is already built, although brief :643 says it is built 'only if its check goes into -17', and r2 does not exist yet.
- **held:** The check in marks.json is the main session's (U4R-Q2 (c)). Evidence: Brief :1545 ('Carry 62 (h)'s check in marks.json is the main session's ... The bite's site is U4r's ... its id in the form PM-0001-17/<name>'); :662-663; :1006.
- **held:** Carry 62 (h)'s own words, and whose words they are. Evidence: u4s-carries.md:289-312, (h): "NC-10 (the advisor's U4j ruling, R2.4): the contract test reports tick.s.snap_incomplete's sum on each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0 part, with a bite (a client that writes null and incomplete 1 for a refused group), before -17's judged runs: {... "window": "measured" ... "setup": true}. If it is not 0, the cause goes to the advisor first." The bite's wording is the advisor's, from advisor-u4j.md:438-449 (R2.4), whose rationale is: 'a refused group can hide only as "lost and counted". On L1 P0 (no loss) that count must be 0'. R2.4 had 'window': 'all'; the carry has 'measured', per e-r1 R5 and NC-L10 as the carry states. Brief :643 quotes it.

### Point 5

- **held:** npt-room's HISTORY_MS is private (room.rs:33). Evidence: room/src/room.rs:33 `const HISTORY_MS: u32 = 2_000;`, used at :446 `HitHistory::for_ms(HISTORY_MS)`. Room's pub methods (room.rs:431-805) expose no history or ring capacity. HitHistory::capacity() is pub in npt-sim (sim/src/history.rs:42), but the room's HitHistory is a private field.
- **held:** The recordings' span_ms is a copy: scenario.rs ROOM_HISTORY_MS = 2,000. Evidence: scenario.rs:54-56 (doc: 'a copy of npt-room's private HISTORY_MS (room/src/room.rs:33 ...)'). span_ms is written at scenario.rs:770 (the Recording header) and :1068 (the lab log's `recording` event). Test s5 asserts `r.span_ms == ROOM_HISTORY_MS` (scenario.rs:1590-1592), which compares the copy with itself; no test compares it with the room's. A runner entry's manifest span_ms is read (hash.rs:297) but never compared: generate (hash.rs:330-344) ignores it for runner entries, and check (:353-365) compares bytes and SHA-256 only.
- **held:** The brief wants the ring the room used (:407, :930; :1254 'The ring holds 2 s at U4r'). Evidence: Brief :407 "span_ms the ring the room used (HISTORY_MS 2,000, provisional until U4h: room.rs:33, :446)"; :929-930; :1254.

### Point 6

- **held:** No item gives a tick's events, so shots and gameplay events per second are read from the World's dump (`events` section). Evidence: SimCore::step returns the tick's events (sim/src/seam.rs:546-548), but Room::tick calls it and copies them into the private field `stamped` (room.rs:813-818, :417). SimCore, World and Room have no pub events getter (grep). logs.rs room_second (:518-603) reads World::dump() (pub, world.rs:1018): it caches the index of 'events[0].count' and reads kind at at+1+4i (1 shot; 2 and 3 hit and kill).
- **held:** The World's dump's events section holds that tick's events. Evidence: world.rs:2100 `self.events.clear()` at the start of each step. A Shot is pushed once per fired entry (world.rs:1550-1553), a Hit at :1620 and a Kill at :1638. The dump writes the count and then 4 words per event (world.rs:1995-2017). Every section before it is a fixed-size pool (effects vec at :846, projectiles at the scene's capacity :855, soldiers, parts and vehicles built once), so the cached index stays valid.
- **held:** Every shot is counted exactly once per tick. Evidence: Room::tick steps the World once (room.rs:814). logs.room_tick, and through it room_second, runs once per room tick (scenario.rs:819-888). Shots are summed per second (logs.rs:584-601), and the measured ones go into shots_measured (:617-623). What is true: Only in logged runs: room_tick runs only `if s.logs` (scenario.rs:884-888). A --logs 0 or regeneration run counts none, and prints 'shots per measured second 0' (rec.log:7, :14, :21).
- **held:** They feed r2's PM-0001-2 shot band and D13's shot rates. Evidence: marks.json:285-305: second.v.shots_count from process room, mean 665 to 735. logs.rs:606-614 writes shots_count into the room log. Summary line at scenario.rs:1372. d13.log shows 698, 698, 479, 698 and 696 per measured second. SERIES.md:91 and :94 name 'npt-room' as these rows' writer; the snapshot's writer is npt-lab's logs.rs, into the room-process log.

### Point 7

- **held:** npt-client exposes no decoded record (R16). Evidence: ClientEndpoint's pub API (client/src/lib.rs:253-415): new, with_lead, with_starts, with_bite, slot, newest_tick, last_run_command, wrong_channel, refused, stale, duplicates, input, step. Nothing returns a frame or records.
- **held:** In npt-lab the view is the room-side frame of that client's newest tick (the mirror); the open-loop scripts never read it. Evidence: scenario.rs:916-921 `mirror.view(c.slot, newest, &mut view)` with newest = endpoint.newest_tick(). mirror.view (mirror.rs:213-225) gives the soldiers of the mirror's frame of that tick, the client's own included. bots lib.rs:204-207: only Kind::Brawl calls brawl_aim(view); the other kinds aim at self.blind.
- **held:** The view may hold entities from datagrams the client lost. Evidence: The mirror is fed the room's datagrams before server.send, so before L1's fates (scenario.rs:830-852; mirror.rs:1-4 'every group of it, before the link'). Its frame is its own baseline frame plus every group. newest_tick() is 'the newest snapshot tick the client has read in a header' (client/src/lib.rs:335-338), so one decoded group of a tick is enough. What is true: Yes, and more broadly: the view can include groups the client lost, groups it refused, and a baseline it never completed, since the mirror's baseline is its own frame (mirror.rs:369-380).

### Point 8

- **held:** D13 (2), P2 at lead 5 with a command at every step: duplicate stamps 12,290 of 87,709 (14.01 %), late 12,489 (14.24 %). Evidence: d13.log:22: commands 87709, duplicate stamps 12290, late 12489, steps without one 0.
- **held:** The notes' other table rows: (1) 75,419 / 213 / 12,290; (3) 65,870 late, rewinds refused 2,873, 479 shots; (4) 69,652 / 129 / 17,889; (5) 86,352 / 0. Evidence: d13.log:19, :25, :28, :31 match field for field. Press refused, buffer full and rewinds refused are 0 except (3)'s 2,873.
- **held:** Runs (1), (4) and (5) were at most 1 % late with 0 press refusals (0.28 %, 0.19 %, 0 %), so D13's stop rule did not fire. Evidence: 213/75,419 = 0.28 %; 129/69,652 = 0.19 %; 0/86,352. Brief :1840-1841: 'A late share above 1 %, or any press refusal, in (1), (4) or (5) stops the build's recordings'.
- **not held:** Notes :175: 'one length for all: 1,800 ticks measured plus warm-up and after'. Evidence: d13.log command lines: --warmup-s 10 --measured-s 50. scenario.rs:1251-1252 multiplies seconds by TICK_HZ (30: smoke header tick_hz). Summaries: 1833, 1833, 1833, 1835 and 1800 ticks. What is true: Warm-up 300 + measured 1,500 = 1,800 ticks, plus 0-35 after.
- **held:** (b1) is in force, so no judged run has duplicates. Evidence: By construction: bots lib.rs:187-190 gives no command while `last_newest == Some(newest)` unless every_step. The stamp is newest + lead, and newest only moves forward. --every-step is refused with marks (scenario.rs:1290-1292). Judged runs are not run yet.
- **held:** U4R-Q14 (b)'s arithmetic: about 10 % at P2 and B1, 15 % at P3. Evidence: Brief :1751-1757: p(1−p) ≤ 25 %; averaged over phases ≈ jitter/T × 1/6, with one datagram per snapshot assumed. l1/src/lib.rs:186-192: P2 jitter 20 ms with 2 % loss, P3 jitter 30 ms with 5 % loss. The arithmetic covers jitter only, not loss.
- **Computed:** ∫0..1 p(1−p) dp = 1/2 − 1/3 = 1/6. P2/B1: (20/33.33) × 1/6 = 0.600/6 = 10.0 %. P3: (30/33.33) × 1/6 = 0.900/6 = 15.0 %. Measured: P2 duplicates (2) 12,290/87,709 = 14.01 %; P2 steps without a command (1) 12,290/(75,419 + 12,290) = 14.01 %; P3 steps without a command (4) 17,889/(69,652 + 17,889 = 87,541) = 20.44 %. Both are about 4-5 points above the arithmetic; attribution was not checked. Late shares: (1) 0.28 %, (2) 14.24 %, (3) 87.34 %, (4) 0.19 %, (5) 0 %.

### Point 9

- **held:** --lead <1..8> is test-only and refused on a run with marks, like --every-step. Evidence: scenario.rs:1270-1277 accepts 1 to 8, anything else is an error. :1290-1292 refuses `(every_step || lead_override.is_some()) && !marks.is_empty()`. Not refused: with --record, with --role twin or bite, or on any run without --marks. The lead is always in the header (lead_ticks_count, scenario.rs:1127); every_step appears only when true (:1167-1169). The runner manifest's Kind::Runner has no lead or every-step key (hash.rs:205-212), and record() passes neither (scenario.rs:540-563). So a recording made with them would not regenerate: the SHA-256 check fails closed (hash.rs:353-365).
- **held:** --every-step takes 1 only. Evidence: scenario.rs:1260-1264: '1' or absent; any other value is an error.
- **held:** --logs 0 only with --record. Evidence: scenario.rs:1265-1269 accepts 0 or 1; :1287-1289 refuses '--logs 0 is for a recording run (--record)'. The regeneration path sets logs = false directly after parse (scenario.rs:556-558). What is true: Its limits beyond that: a no-log run skips room_tick (scenario.rs:884-888), so shots, PM-0001-2's series, measured incomplete (phases stay empty, and phase() falls back to After, logs.rs:369-375) and carry 75's admission bound (events_stamped stays 0, scenario.rs:804) are not counted. rec.log:7, :14 and :21 print 'shots per measured second 0' and 'measured incomplete 0' at P2 and P3. Recording runs also stop at the measured end (scenario.rs:982-985). s5 shows the bytes do not depend on the logs (scenario.rs:1583-1596).
- **held:** The recordings' wall times (200.1, 104.2, 167.2 s) were taken before the no-log memory fix (notes :208-209). Evidence: rec.log real 3m20.101s, 1m44.248s and 2m47.190s, run 20:17-20:25. The fix is in the snapshot: scenario.rs:833-835 keeps -57's errors only `if s.logs`. Regeneration was later and faster: SS-P0-s4 about 108.1 s (regen file times) against 167.2 s recorded; SS-P2-s1 142.6 s against 200.1 s; ES-P3-s2 52.6 s against 104.2 s. Which build first held the fix could not be checked (the binaries are not kept).
````

### The advisor's second ruling on U4r's build: copy 2's points (advisor-u4r-build2-full.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4r-build2-full.md` lines 1-62 (sha256 f4f5ac925633), as they are:_

````text
# Advisor: the full second ruling on U4r's build, copy 2's points (workflow wf_083df555-757), 2026-11-14

Written by script, word for word, from the workflow's journal (the session folder's
subagents/workflows/wf_083df555-757/journal.jsonl): the advisor's StructuredOutput result (advisor-u4r-build2; cut by the
spend limit at 22:02:57 UTC and resumed by resumeFromRunId at 00:16). Its own notes: advisor-u4r-build2.md. Point
numbers 31 to 33 are point 3's parts (i) to (iii). Nothing below is the main session's.

## The rulings

### Point 1 (severity: Medium; to Mustafa: no)

- **Decision:** (a), made exact. The header value stays the pacing lead L, the value set (marks.json:6557 'input buffer depth set for P0'; SERIES.md:240), printed as `buffer_depth_ticks_count L` on both legs. Beside it goes the room's measured depth (`ClientSlot::buffer_depth_ticks`), with its expected relation: measured = L - 1 at every take after each client's first. Two things fix that relation. The measure is defined that way (input.rs:28-29 'at the last take'; :142-145 the newest buffered command's tick minus the tick just taken). And 7.2's pacing sends T at step 5 of the instant tick T - L runs, so at take j the newest command held is stamped j + L - 1. The brief's 7.2 words 'report any tick after each client's first commands where the two differ' now read 'report any take after each client's first where the room's measured depth is not L - 1'. A deviation is reported, never a VOID cause on its own; 7.2's VOID causes are unchanged. (b) is rejected: L - 1 is an observation, not what was set. Writing it would make -30's 'depth set for P0' read 1 while the replay sends 2 ahead. Whose words change: the brief's 7.2 :1197-1200, the main session's text, which I approved. The plan's 'both legs' input-buffer depth recorded' (:775), my condition 3, the -30 report's series and SERIES.md stay.
- **Builder:** Point 1, the depth: keep L. (1) The pair line keeps 'buffer_depth_ticks_count L (the pacing lead, the value set), both legs'. (2) On each leg line, beside it, write: 'the room's measured depth at its take (input.rs:28-29: the newest buffered command's tick minus the tick taken) is L - 1 by definition under this pacing'. Then give the histogram over the takes after each client's first take only. Then give the count of later takes where the measured depth is not L - 1, listing the first 64 as (client, tick, measured). (3) On each client line, give that client's measured depth at its first take. Today the 48 first takes sit in the histogram as its 0s: split them out. (4) r6 asserts on both legs that every reading after each client's first take is L - 1, with no deviation. Its doc says that in place of 'the room's measured depth the lead' (precheck.rs:694-696). (5) A deviation is reported, never a VOID cause: `verdict` is unchanged. In the late-command pair, the bitten client's deviations around T are the bite's, and the report says so. (6) The report states the relation in one line, citing input.rs:28-29 and :142-145, with r6's measured figures.
- **Main session:** Sends the builder text. On the card (or at the brief's next edit, if it is re-stamped), records that 7.2 :1197-1200 now reads 'report any take after each client's first where the room's measured depth is not L - 1', with a pointer to this ruling. Adds a card line for U6's brief (-30's marked C part) and U4f's (option A): header.params.buffer_depth_ticks_count is the depth set (under option B, the pacing lead), and the room's measure at its take reads L - 1 under this pacing. In U4r's review it reads the depth lines in runs.txt.
- **Why:** As built, the pair line prints 'buffer_depth_ticks_count 2' beside a histogram {0: 48, 1: 5,568} that mixes the first takes with the later ones (c2-b10c-4.log:337-338). So 7.2's comparison as worded would flag every take (5,568 of 5,568) or say nothing. r6's doc claims the depth equals the lead, while its body prints the depth and asserts nothing about it (precheck.rs:715-720). Not for Mustafa: no word or decision of his changes, the -30 report's series and the plan's words stay, and it is risk tier 1 (spike code and checkpoint text).

### Point 2 (severity: Medium; to Mustafa: no)

- **Decision:** No per-leg npt-log log is required at U4r. The pre-check's result lines are the record, made complete. The plan's committed records are runs.txt and the comparer's output (u4-build-plan.md :793-795). My own U4 condition 2 made a header optional: 'If the Loopback leg writes an npt-log header, README §1 gains the layer value `loopback`' (advisor-u4.md:100-101). 'The headers list no mark, so the judge never reads these runs' (plan :793; marks/README.md:36) stays true: no header is written, and any header a later unit writes lists no mark. Where 7.2 says 'the header', read 'the pair's result lines in runs.txt'. That covers :1197-1198 (buffer_depth_ticks_count), :1201-1202 (the room configuration 'recorded in both headers') and :1221 (the tick base 'in both legs' headers'). With no header, the lines must hold what a header would: the build (commit, binary SHA-256, features: plan.md:630-631 via brief :361-363), the recording's SHA-256 (7.4 :1364), each leg's room configuration including the ring, and 7.2's per-command log of refused Rewinds (:1253). Whose words change: the brief's (the main session's text, my approval). Not the plan's, not Mustafa's.
- **Builder:** Point 2, the records: write no per-leg npt-log log. The record is runs.txt (precheck's result lines) plus the comparer's line (plan :793-795). Add to the lines what a header would hold. (1) `npt-lab precheck` takes `--commit <40 hex>`, required, as `scenario` does. It prints one build line from `crate::machine::build(&commit)` and `crate::machine::container()`: the commit, the binary's SHA-256, its features and the machine. Add one parse-test case: no --commit is a usage error (exit 2). (2) Print the recording line it checked: the manifest name, bytes and SHA-256 'as the manifest's', plus generator, first_tick, span_ms and ticks. (3) Each leg line states that leg's room configuration: layer (loopback or L1), profile (none or P0), cmd_timing B, the lead, the tick base, the seed, and the ring (ROOM_HISTORY_MS and `HitHistory::for_ms(ROOM_HISTORY_MS)`'s capacity in ticks). (4) Per leg, log the refused Rewinds per command. After each tick, read `ClientSlot::rewinds_refused()` (room.rs:262-265) for every client, and print the total with the first 64 as (client, tick, sequence). 7.2 says 'Log it per leg and per command'. (5) Write the depth as point 1 says. The report says three things: the hashes compared are `SimCore::state_hash` of the room's World after each tick, the value a room log writes as `h` (logs.rs:466); the comparer's line is `HashCompare`'s Debug printed by precheck.rs `pair`, since `compare_text` stays private in hash.rs; and no per-leg log is written, by this ruling.
- **Main session:** Sends the builder text. On the card it records that brief 7.2's 'header' words (:1197-1202, :1221) read as the pair's result lines in runs.txt, with a pointer to this ruling; plan :793 and README:36 stay. Its B12 run script for the pre-check passes --commit (the merged commit) and writes the lines to results/u4r-precheck-30/<commit12>/runs.txt. U6's judged C part writes real headers under its own brief. In U4r's review it reads runs.txt's build and recording lines.
- **Why:** precheck.rs `main` takes no --commit and prints no build identity. It checks the recording's bytes and SHA-256 (:531-550) but prints only its name. It does not print the ring span. It sums rewinds_refused per client at the end (:402-409) and does not list them per command. With no header, runs.txt would not say which binary or which recording produced a PASS or a FAIL. The hashes already are what 7.2 names: `SimCore::state_hash` (:358-362) is the value logs.rs writes as a room log's `h`. A full per-leg log (about 432,000 depth values per leg) would add nothing the judge reads (it never reads these runs), and nothing U4r's review needs once the lines are complete. Not for Mustafa: no word or decision of his changes; risk tier 1.

### Point 31 (severity: Low; to Mustafa: no)

- **Decision:** 3 (i): not U4r's fix. The failure predates U4r (it is already there at 2fbec77), and the files are outside U4r's lane. npt-server's `bites` feature turns on npt-sim's, npt-proto's and npt-room's only, and npt-client is a dev-dependency (server/Cargo.toml). Meanwhile tests/loopback.rs:292-348 names `npt_client::BITES` under cfg(feature = "bites"). Both files are U4e's and unchanged in the snapshot. U4r's npt-server lane covers only U4R-Q6 (a)'s items: pump.rs, schedule.rs and tests/schedule.rs. check.mjs's `--workspace --all-features` unifies the features and runs the test (c2-b10a-2.log: schedule 17 passed, loopback 7 passed), so no check loses a test. It becomes a follow-up for the next unit whose lane opens crates/server.
- **Builder:** Point 3 (i): no change. Put it in the report's 'known, not fixed' list: `cargo test -p npt-server --all-features` stops at E0425 in tests/loopback.rs (`npt_client::BITES`). The cause: `-p` turns on npt-server's own `bites`, which does not turn on npt-client's (a dev-dependency). It predates U4r: server/Cargo.toml and tests/loopback.rs are unchanged since 2fbec77. The measure is check.mjs's `cargo test --workspace --all-features`, which runs it (c2-b10a-2.log).
- **Main session:** Adds a card follow-up for the next unit whose lane opens npt-server's manifest or tests (U4h's server items, or U4f if sooner), named in that unit's brief: make `cargo test -p npt-server --all-features` compile, with that command's run as the evidence. No push for it.
- **Why:** It is a cargo feature-selection artefact that fails before any test runs, and the workspace command check.mjs uses works. Mustafa's 'A Low never triggers a push' (history.md:340) and the review budget make it a follow-up. Not for Mustafa: none of his words or decisions change; risk tier 1.

### Point 32 (severity: Low; to Mustafa: no)

- **Decision:** 3 (ii): the system temp folder is replaced by the cargo target dir, and each test removes its own folder when it passes. Plan :762 puts r3's received bytes 'under .gate/'. The shared helper `scenario::tests::scratch` (scenario.rs:1492-1501) roots every npt-lab test folder in std::env::temp_dir() and never removes it after the test. /tmp is on the same disk as .gate/ (/dev/vda, 5.2G free), and it holds 93 such folders, 126 MB, today. No approved method removes folders in that form. That runs against Mustafa's LEFTOVER-DELETE (history.md:404-406) and its applied line: scratch is kept in forms the approved methods can remove, a cargo target dir or a worktree (:406-410). The test binary's own cargo target dir (.gate/shared-target here) meets both plan :762 and that rule. A test removing the folder it made in that run is its own teardown, not a 'clean' under section 4 rule 6. The 93 existing folders stay: no one-off delete.
- **Builder:** Point 3 (ii): change `scenario::tests::scratch` (scenario.rs:1492-1501), the one helper every npt-lab test uses. (1) Root it inside the cargo target dir the test binary runs from: take `std::env::current_exe()`'s folder (target/<profile>/deps/) and put `npt-lab-tests/<name>-<pid>` beside deps/, in target/<profile>/. Here that is inside .gate/shared-target, so `cargo clean --target-dir` removes it. (2) Have each test remove its folder when it passes, and keep it when it fails. For example, `scratch` can return a guard that removes the folder on drop unless `std::thread::panicking()`. r3's bytes then lie under .gate/ (plan :762). Never delete the 93 folders already in /tmp/npt-lab-tests (section 4 rule 6; Mustafa's LEFTOVER-DELETE). Re-run the npt-lab suites in both builds afterwards. The report says where test scratch goes, and that the run scripts' --bytes go under .gate/ (B12).
- **Main session:** Counts /tmp/npt-lab-tests (126 MB now) in its disk plan (ruling 1's point 3). It makes no one-off delete. If those folders ever must go, that goes to Mustafa as a narrow kit rule (LEFTOVER-DELETE), not a delete.
- **Why:** Low: the folders are never committed and small. But they sit on the same disk as the 4 GB floor, no approved method removes them, and r3's use goes against plan :762's '.gate/'. Not for Mustafa: this applies his LEFTOVER-DELETE and changes none of his words; risk tier 1.

### Point 33 (severity: Medium; to Mustafa: no)

- **Decision:** 3 (iii): the kept whole-as-lost logs are builder evidence, not the record. The bitten run is re-made from the committed code, and that run is the one judged. npt-lab writes whatever --commit it is given (machine.rs:77-101). The builder passed 2fbec77 for a tree holding the uncommitted U4r changes, so header.build.commit names a commit whose code is not the code that ran. header.build.binary_sha256 pins the binary, not its source. The judge reads no log's commit (judge/src reads only marks.json's drafts_commit), so the demonstration would still fire. But a record citing those logs would cite the wrong source. This changes my ruling 1, point 4's main-session step: the main session judges a bitten run made from the commit that holds U4r's code, not the builder's logs. The same holds for every log a builder makes before its code is committed (D13, the recordings' runs, r2's short runs and their verdicts): builder evidence only, labelled, never a committed record. The judged runs and the pre-check already run after the merge (brief section 7).
- **Builder:** Point 3 (iii): keep .gate/u4r/whole-as-lost/ as it is; never edit or delete a log. In your notes and report, label every log you made before your code is committed as: 'tree: 2fbec77 plus the uncommitted U4r changes; header.build.commit names 2fbec77; the binary is header.build.binary_sha256 <value>'. That covers D13, the recordings' runs, r2's short runs and their verdicts, and whole-as-lost. Draw no committed record from them. For whole-as-lost, give the header's binary SHA-256 and the exact command lines (the build line and the scenario line from whole-as-lost.log), so the main session can re-make the run. No other change.
- **Main session:** Ruling 1, point 4's step now reads as follows. After U4r's code is committed (the merge step's commit), the main session re-makes one bitten short L1 P0 run from that commit, with the builder's command line (bites build, --commit that commit, marks PM-0001-17, --bite PM-0001-17/whole-as-lost). It writes it into a new folder under .gate/ named with the commit's first 12 hex digits. After its own marks.json commit adds the setup check and the bite entry, it runs `npt-judge --only PM-0001-17` on that run and shows the setup check FAIL by name (U4R-Q2 (c)). That run and its verdict are the record, made before -17's judged runs. It may judge the builder's kept logs first as a dry check, labelled pre-commit. It records the change on the card with a pointer to this ruling, and never cites a builder-made log as a run of a commit.
- **Why:** No wrong record exists yet, but my own ruling-1 step would have produced one: a demonstration citing commit 2fbec77 for code that was not 2fbec77, which is evidence that does not hold. Not for Mustafa: the words amended are mine (ruling 1, point 4), and no check, value or pass bar moves; risk tier 1.

### Point 4 (severity: Low; to Mustafa: no)

- **Decision:** Copy 2's code conforms to every Builder line it took on: points 1 (1)-(3), 2, 3's writer, 4, 5, 6 and 9 (a)-(b), and the wallclock site's field rule. Two tests are weaker than what they claim (Low). (a) p2's radius pin passes when `radius_pin()` finds no placement: it prints 'unpinned' and passes. Now that a placement exists, a later change could unpin the copy silently. My ruling allowed the skip only for a placement that does not exist at build time. (b) The late-command bite test asserts `starts_with("result: VOID (L1 late_count 1)")`, which a VOID by two causes would also pass. 7.2 shows the bite 'only when the pair comes out VOID by late_count alone' (:1301-1303; eval.rs:1412). The printed line is exact today (c2-b10c-3.log: 'result: VOID (L1 late_count 1)', commands off their stamp 0). Still open by design, now copy 3's: point 1 (4)-(7) and point 3's bytes per tick.
- **Builder:** Point 4, two test fixes; no other change to the rulings' work. (a) In p2_the_mirror_s_radius_copy_is_the_room_s_by_placement, a `None` from `radius_pin()` now fails by name ('no direction lands both placements in slot 0's area: the copy is no longer pinned'), because the placement exists at the head. The plant run need not be repeated, since the Some branch is unchanged. (b) In precheck_bite_late_command_makes_the_pair_void_by_late_count_alone, assert three things: the result line equals 'result: VOID (L1 late_count 1)' exactly; the L1 leg's off_stamp is 0; and the Loopback leg's late, rewinds_refused and off_stamp are 0, with rtt_void None. Re-run both tests in the bites build.
- **Main session:** Reads both tests in U4r's review. Nothing else.
- **Why:** Checked in .gate/u4r-snap1 against each Builder line. Point 9 (a): scenario.rs:1356-1363, test :2136-2160. Point 9 (b): summary_line :1422-1470. Point 3: :787-789 and :872-874, test :2183-2204. Point 4: logs.rs:302-347, LAB_BITES cfg(bites) scenario.rs:196-197, test :2209-2221. Point 2's counters: mirror.rs:254-307 and class_audit :1125-1129; pin test :2039-2134. Point 5: :1981-2037, and hash.rs check :365-395 with h5 :1386-1436. Point 1: det-recordings.json as notes :225-230, and h5 :1457-1470. Wallclock: pump.rs:228-246. Both gaps are tests weaker than their stated claims while the claims hold today: Low, riding the builder's ongoing work. Not for Mustafa: none of his words or decisions change; risk tier 1.

### Point 5 (severity: Low; to Mustafa: no)

- **Decision:** Two more items before copy 3 goes on, both Low and both in U4r's lane. (a) Carry 83 (a) is not done: TickHash's struct doc (lab/src/lib.rs:8) still reads 'the recording (or run) it replays', and the item fell off copy 2's next list (brief :654, :1243). (b) `Bites::of`'s `any` (scenario.rs:229-236) leaves out `b.pump`, so PM-0001-30/wallclock is accepted only because npt_server::BITES also lists it. The rest of copy 2's part matches 7.2: tick base, encoder route, pacing, JoinAccepted first, scripts, comparer, setup counts, RTT rule, admit ticks, ended sessions and both bites. Not ruled here: copy 3's r2 design (contract.rs, a subcommand beyond 2.7's list), which is copy 3's part. Data: wip/u4r.patch was replaced at 2026-11-14 00:14 (sha256 585a8639..., no longer 48fa2959...). I read code only in .gate/u4r-snap1, which is unchanged since 21:52:58.
- **Builder:** Point 5: (a) For carry 83 (a), change TickHash's struct doc (lab/src/lib.rs:8) to its field doc's words, for example: 'One tick's entry in a leg's per-tick hash list: the recording both legs replay (its manifest name), the tick and the state hash.' Change nothing else in lib.rs. (b) In `Bites::of`, add `b.pump` to `any`, so the pump's id is accepted on its own, not through npt_server::BITES. Both ride your next scenario.rs and lib.rs edits. Run fmt and clippy in both builds afterwards.
- **Main session:** Sends the builder text. Notes that wip/u4r.patch now holds a later snapshot (585a8639...) than the one this ruling read (48fa2959...), so any review of copy 3's work names its own snapshot. Reads copy 3's contract.rs design in U4r's review, or asks me for a ruling before r2's runs count.
- **Why:** (a) is a named carry edit in U4r's lane (brief section 4, :654) that was left open. (b) is harmless today, but it makes the dispatch depend on a list npt-lab does not own. Lows ride the builder's ongoing work (history.md:340: 'A Low never triggers a push'). Not for Mustafa: none of his words or decisions change; risk tier 1.

### Further findings

- **Low:** precheck.rs's client lines print the instant the first command's tick ran ('first run tick X at Y ns', `first_run_ns`). 7.2 (3) (:1266) asks for that tick's due instant (base + floor(k x 10^9 / TICK_HZ)). In `run_leg` the two are equal only by construction: the instants step to `schedule.next_due_ns()`, and there is no catch-up at P0. No record says so. **Fix:** Either print the due instant beside the run instant on each client line (the module's `due_ns(k)`), or have the report state in one line that they are equal in run_leg and why. The RTT rule keeps comparing the run instant, as 7.2 words it.
- **Medium:** My ruling 1, point 4's main-session step told the main session to judge the builder's kept .gate/u4r/whole-as-lost/ logs as U4R-Q2 (c)'s demonstration. Those logs' header names commit 2fbec77 for uncommitted code (point 33). **Fix:** Amended in point 33: the record is a bitten run re-made from the commit that holds U4r's code and judged after the marks.json commit. The main session records the amendment on the card.
````

### The advisor's third ruling on U4r's build: copy 3's points 1 to 8 and further findings, then the netcode architect's read and the fact check (advisor-u4r-build3-full.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4r-build3-full.md` lines 1-235 (sha256 78ebb5af7d48), as they are:_

````text
# Advisor: the full third ruling on U4r's build, copy 3's points (workflow wf_345cc3c9-ca1), 2026-11-14

Written by script, word for word, from the workflow's journal (the session folder's
subagents/workflows/wf_345cc3c9-ca1/journal.jsonl): the advisor's StructuredOutput result (advisor-u4r-build3; its run
stopped by the container restart at about 01:18 UTC and resumed by resumeFromRunId at 01:22), then the netcode
architect's read (u4r-arch-3) and the fact check (u4r-facts-3). Notes: advisor-u4r-build3.md, u4r-build3-arch.md,
u4r-build3-facts.md; the questions: u4r-build3-questions.md. Nothing below is the main session's.

## The rulings

### Point 1 (severity: High; blocks the merge: no; to Mustafa: no)

- **Decision:** The room is partly working as designed and partly defective, and the plan has a gap. None of it is U4r's code. (i) Eviction at every B1 burst is designed: the ring keeps ages 0-31 (plan §4.5 :296-300), -18's bite rests on 'a gap evicts B' (§4.14 :461), and coding from the per-type defaults is ADR 0001 part 6's rule. (ii) Packing the event backlog before any entity change (room.rs:1208-1235, before the nearest-8 loop at :1236 on) is carry 78's known defect against ADR part 9 (d)'s 'the nearest 8 enemies in every snapshot'. Carry 78's premise, 'no U4e or U4r run meets it' (u4s-carries.md:546-553), no longer holds: U4r's B1 suite meets it. (iii) Rebuilding each client's set from the defaults under the 1,000 B cap leaves every client without the other soldiers for about 4 to 11 s per burst. No plan text says when the set must be whole again, and my own Q22 put pre:PM-0001-2 on -17's and -18's B1 parts after checking only the shot band (plan §4.12 :416-424, §4.13). Verified myself: the worst slot holds everyone on 246 of 900 measured ticks (0.273333); the 30-tick window means are 47.9, 0.5, 0.0, 0.0, 0.0, 0.6, 4.1, 12.4, 23.1, 31.9, 40.1, 44.2, 45.8, 46.3; 96 burst_eviction records, each evicted 1 at age 0. The FAIL stands as r2's B1 result (condition 5) and is never re-run into a pass. No bar moves: Q22's pre:PM-0001-2 stays on -17's and -18's L1 B1 parts, the 0.99 share stays, and the flag's input (the mirror frame) stays. Rejected: exempting or windowing B1 (it loosens a validity bar); reading the flag from the room's interest set (it would hide a real 4-11 s blindness and let -18 pass on it); a longer ring (npt-sim's, and it breaks -18's bite design); and a full state on eviction now (it changes ADR part 6's words). The fix goes to U4g, which already owns carry 78 in the same function (`snapshot`). Carry 78 widens to the room's recovery after an eviction. Target: on B1 with 10 bursts in 600 s, PM-0001-2's everyone-in-view share is at least 0.99, which allows at most 18 flag-0 ticks per client per burst on average. It is shown first by a short B1 suite of U4g's build, then by -17's and -18's L1 B1 judged runs. The form is U4g's plan, for example the relevant soldiers the baseline lacks first, with carry 78's reorder; a form that changes ADR words reaches Mustafa through U4g's plan and the readings PR. At U4r, -17's and -18's L1 B1 parts do not count. The three B1 judged runs stay in U4r's set because they serve -57's L1 B1 part, which has no precondition; their -17 and -18 verdicts are recorded INVALID and never counted. -18's bitten judged run moves to U4g. For this build, r2 (d)'s 'pass' on -18's B1 reads 'resolves; its FAIL reported under this ruling'.
- **Builder:** Point 1, B1: change no code, and do not re-run the b1 suite. Keep .gate/u4r/r2/b1/ and b1-verdict.json as they are. In the report's r2 section, state the B1 suite as a FAIL under condition 5: PM-0001-2's everyone in view 0.273333 (the worst slot 246 of 900 measured ticks), so -17 and -18 L1 B1 are INVALID by pre:PM-0001-2, while -57 L1 B1's three checks pass (6,974; 2,750; 88,060). Give the cause in three lines with their evidence. (1) Every client's baseline is evicted at each burst: 96 burst_eviction records, evicted 1, baseline_age 0. (2) The event backlog is packed before any entity change (room.rs:1208-1235; queue high-water 631). (3) Each client's set is rebuilt from the defaults under the 1,000 B cap (snap_datagram_bytes 996-1,000 B per client per tick before, during and after). Give the 30-tick window figures as means with one decimal (47.9, 0.5, 0.0, 0.0, 0.0, 0.6, 4.1, 12.4, 23.1, 31.9, 40.1, 44.2, 45.8, 46.3), and the 10-burst arithmetic: if every burst costs what burst 0 did, the least share at 10 bursts in 600 s is at most 0.919. Say it is npt-room's, not U4r's, and that by the advisor's ruling 3, point 1, the room's fix and the passing evidence of -17's and -18's L1 B1 parts go to U4g.
- **Main session:** 1. Send the builder text. 2. On the card, carry 78 gains this line: 'its premise "no U4e or U4r run meets it" does not hold: U4r's short B1 suite meets it (the advisor's U4r build ruling 3, point 1)'. It widens to the room's recovery after an eviction, with the target (share at least 0.99 on B1 at 10 bursts in 600 s, at most 18 flag-0 ticks per client per burst on average, shown first on a short B1 suite of U4g's build), and it names -17's and -18's L1 B1 parts and -18's no-fallback bite run as moved to U4g. U4g's brief names all of this. 3. U4r's brief: 2.4 r2 (d) for -18's B1, and 7.1's -17 L1 B1 and -18 L1 B1 rows, read as this ruling says, with a pointer. Keep the B1 measured runs in U4r's set for -57, and record -17's and -18's B1 verdicts as INVALID, not counted. The plan's §5 U4r judged-runs line and U4g's 'Marks moved' line change at the next plan edit (RQ1 :114 is U4-wide, so U4 as a whole still moves -18). 4. The readings-PR list (plan §11) gains: 'pre:PM-0001-2's everyone-in-view share applied to every -17 and -18 L1 run (Q22), where ADR 0001:1589 names it "for the downstream rows"; stricter'. 5. U4r's report tells Mustafa the story in plain words: after each 1 s loss burst, every client lost sight of the other soldiers for about 4-11 s; the cause is the room's; it is fixed in U4g, and -18's L1 evidence moves there. 6. When U4g's brief is sized, weigh his SIZE-CHECK (history.md:398-399).
- **Why:** The logs and the code agree: room.rs:1163-1170 and :1208-1292, mirror.rs:314-337, and my own recount of b1/room.jsonl. The precondition did its job and caught a real defect. Reading the flag from the interest set, or exempting B1, would make -18 pass while every client is blind for seconds, which is evidence that does not hold. U4r's runner, mirror and judge call are right, so the merge is not blocked; -57's B1 part is unaffected. Not for Mustafa now. His only recorded word on PM-0001-2 is ES's events floor (history.md:338-339, verbatim: 'The 9.5 floor (PM-0001-2) does not move.'). That floor is point 2's check, not the 0.99 share, which ADR 0001:1589 calls a 'proposed share' (ADR status: Proposed). No bar or word of his moves. Moving parts between U4's sub-units is the order of approved work, which is mine; RQ1 is U4-wide. Any ADR word the fix changes, and the new readings-PR line, reach his go through U4g's plan and the readings PR before R5 (plan §11). Risk tier 1.

### Point 2 (severity: Medium; blocks the merge: no; to Mustafa: no)

- **Decision:** The band is right and the brawl is mis-set for ES. It aims at its target's exact position (bots/src/lib.rs:226-245), so 68.5 % to 69.5 % of its 72 shots a second hit: 987-1,001 reliable events against 1,440 shots in the 20 measured seconds. ES's key needs about 14 % (10 of 72). The band does not move: 9.5 is Mustafa's floor (history.md:338-339), and 10.5 is marks.json's +5 %. SS's brawl stays as it is: SS has no event band, and changing it would change the B1 evidence of point 1. The brawl gains an ES-only, seeded hit rule, for example aiming at the target one command in N and otherwise at a seeded point outside its box. N is set by the U4d close ruling's step-4 measurement form (advisor-u4d-close.md:90-105), at P2, PM-0001-12's ES profile (ADR 0001:1600). That means seeds 1-3; a 10-minute measured window after a warm-up; ES's four bands over the window (shots 68.4-75.6, gameplay events 9.5-10.5, projectiles 30.4-33.6, spawns 20.064-22.176 a second); each 300 s half's event rate in its band; and the share holding all 47 soldiers and all 15 vehicles at least 0.99. Once set, the value is frozen in the code with its measurement cited, and SS's suites stay equal figure for figure. The owner is U4h, not U4r. U4h opens npt-bots (the TC shooter's Behaviour, plan :834-845), and its lag compensation changes which shots hit, so a value set before it would not hold. No ES run counts before PM-0001-12's ES runs (carry 59). At U4r: no change; the report gives the pair carry 59 asks for.
- **Builder:** Point 2, ES: change nothing in npt-bots, and do not re-run the es suite. In the report's r2 section, give the ES suite's FAIL under condition 5, with carry 59's pair side by side: reliable events 50.05 a second (1,001 in the 20 measured seconds) against the band 9.5-10.5, beside U4d-F's 7.250 (the synthetic recording at its old reckoning). Give the cause: the brawl aims at its target's exact position (bots/src/lib.rs:226-245), so between 68.5 % and 69.5 % of its 1,440 measured shots hit (reliable events are Hit and Kill; at most 14 kills at damage 10 and health 1,000). Say that the other 11 ES checks pass, that no ES run counts at U4r, and that by the advisor's ruling 3, point 2, the brawl's ES hit rule and its measurement are U4h's.
- **Main session:** 1. Send the builder text. 2. On the card, add a carry for U4h: 'the brawl's ES-only seeded hit rule; its value set by advisor-u4d-close.md:90-105's step-4 form at P2 (seeds 1-3, a 10-minute window, the four ES bands, each 300 s half in band, the everyone-in-view share at least 0.99), on U4h's build with lag compensation; frozen in the code with its measurement cited; SS's brawl unchanged and its suites equal figure for figure; before any ES run of PM-0001-12 counts'. Carry 59 gains a pointer to this ruling. 3. U4h's brief names the carry and opens crates/bots/src for it.
- **Why:** Carry 59 asked me to see both figures before a missed band is read as a bot defect. 50.05 against 7.250, with a hit share bounded at 68.5-69.5 % from the logs, is a load script that misses the scene's key, not a band to widen. Calibrating a load script to the ADR's ES key is a technical choice inside the documented targets. Mustafa's words fix the 9.5 floor and it stays ('The 9.5 floor (PM-0001-2) does not move.', history.md:338-339). The ceiling is marks.json's and stays too. No ES run counts at U4r, so nothing judged is lost. Risk tier 1.

### Point 3 (severity: Medium; blocks the merge: yes; to Mustafa: no)

- **Decision:** Carry 73's outcome (the mismatch figure FAILs, never INVALID) stands; the run's settings change. Past the wrap the bitten clients stop acknowledging and their bots stop commanding. The room evicts at about wrap + 31 and the flag falls to 0: in tick16c the flag holds 48 from 65,507 to 65,566 and is 0 from 65,567, and every flag is 0 from 65,715 to the end (65,715, not 65,746). That is the bite's own effect. With tick_start 65,506 and a 300-tick warm-up, the wrap fell at warm-up tick 30, so the whole measured phase lay inside the collapse. The run never crossed the wrap inside its measured phase, which -17/tick16's own text asks for: 'the bite run must cross it' (marks.json). The fix is to put the wrap inside the measured phase, 15 ticks before its end, so the measured window ends before the room evicts. pre:PM-0001-2 then holds. The judge's decoded() reads every phase (figures.rs:236-266) and counts each orphan as a mismatch (:422-425), and the mismatch count comes before any gap (:542-554), so the post-wrap sentinels make -17 FAIL. This needs no code, no judge or marks.json change, and moves no bar. Rejected: a per-bite validity exception (it loosens a validity bar and narrows the B7 ruling), and accepting INVALID (it breaks carry 73's words). The judged bite run in 7.1 takes the same placement: tick_start = 65,551 - (warm-up ticks + measured ticks), which is 47,251 for 300 + 18,000, with the wrap 15 ticks before the measured end. The clean -17 runs and the client-rounding bite run stay at 63,736 (U4R-Q7 (a)). A clean run that crosses the wrap is added, because no clean run in copy 3 reaches 65,536. The three earlier tries stay on record. This is a setting fixed by ruling before the run, not a re-run of a failed suite into a pass.
- **Builder:** Point 3, tick16: run settings only, each run once (-x). (1) In the bites build: SS P0 seed 1, `--role bite --bite PM-0001-17/tick16 --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5`, marks PM-0001-2,PM-0001-17. The wrap falls at measured tick 585, 15 before the measured end, and the header's ticks_below_wrap is 885. Then run `contract`, then `npt-judge --only PM-0001-2,PM-0001-17`. Expected: every PM-0001-2 check passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped is true; the mirror has 0 breaches; contract PASS. (2) In the measured build, its clean partner across the wrap: the same scene, profile, seed and lengths with `--tick-start 64936` (the wrap at measured tick 300), marks PM-0001-2,PM-0001-17,PM-0001-57, and no --bite. Expected: PM-0001-2 passes; -17 passes with 0 mismatches; -57 passes; 0 mirror breaches; the after phase resolves without the cap. If either outcome differs, stop and send it to the advisor (condition 5); never re-run it. Keep r2/tick16, tick16b and tick16c as they are. The report gives carry 73's run at this setting and why: the bitten room keeps its pre-wrap baselines for about 31 ticks at P0 (tick16c: 48 through 65,566). It also gives the three earlier tries as evidence, with the collapse's true first ticks (65,567, then every flag 0 from 65,715).
- **Main session:** 1. Send the builder text. 2. On the card, carry 73's '(-58/tick16's ticks_below_wrap = 30 keeps it short)' now reads: 'the short run's wrap lies inside the measured phase, 15 ticks before its end (the advisor's U4r build ruling 3, point 3)'. 'Never INVALID' stays. 3. In U4r's brief, 2.4 r2's carry 73 line and 7.1's -17 L1 P0 row change: the tick16 bite run's tick_start = 65,551 - (warm-up + measured ticks), the wrap 15 ticks before the measured end, recorded in its header. U4R-Q7 (a)'s 'clean and bitten' now reads 'clean, and the client-rounding bite run'. Plan :767-768 is noted for the next plan edit. 4. B12's tick16 script computes tick_start from its own lengths. 5. In U4r's review, read the two runs' verdicts.
- **Why:** Checked myself: tick16c's flag segments; figures.rs:236-266 and :316-440 (every phase read, an orphan counted on every machine); eval.rs's B7 rule (a bite run takes its parts' validity); and the header arithmetic (65,551 - 900 = 64,651; 65,536 - 64,651 = 885). At the brief's settings the bite could never be shown, since an INVALID bite run shows nothing, so -17's L1 P0 pass could never count (bites first). The run setting fixes that with no bar moved. It blocks merge only in that r2's carry 73 item must be shown at the new setting before U4r's report. The words that change are mine (carry 73, U4R-Q7 (a)) and the main session's brief and plan text, which I approved. None is Mustafa's. Risk tier 1.

### Point 4 (severity: Medium; blocks the merge: no; to Mustafa: no)

- **Decision:** Yes, the brief's expectation is wrong. 'No client holds a heal within 500 ms' contradicts carry 77 (b), which the brief itself cites (:109, :459, :1169): under the bite, a tick coded from the defaults escapes it and heals. For the bitten run, r2 now reads as follows. On the short B1 run with PM-0001-18/no-fallback, -18's max check fails: some client's heal comes later than 500 ms (here 1,688,286,323 ns). The heals within 500 ms are counted apart by cause. 14 are in burst 0, each at burst_eviction baseline_age 0 (carry 77 (b)'s case). 11 are in burst 1, at ages 33 or 34: a second case of the same cause. There the stale copy is taken only when the ring slot is overwritten (ack.rs:109-114), after the base is chosen (room.rs:1163-1170). This is derived from timing, not shown client by client. All 71 slow heals follow a full state (71 overflow resyncs; full_state 1 on each). The clean run holds one heal per client and burst within 500 ms (96 of 96, 48-130 ms, none after a full state). The heal record is honest: scenario.rs:989-1012 takes neither the sentinel (h != 0) nor a full state's tick. A record that took the sentinel would have shown the 71 slow clients healing within about 50 ms. The hole is in npt-room, outside U4r's lane, so U4r changes no code for it. U4g, which opens npt-room for point 1, closes it: the stale copy is taken when a tick becomes last_acked (ack.rs:156-158). That is shown on a short bitten B1 run with no heal within 500 ms; the brief's original sentence then holds and carry 77 (b)'s counting apart ends. This run's everyone-in-view 0.48 is not a figure: see the mirror item under point 8.
- **Builder:** Point 4, -18's bite: no code change for the bite. In the report's r2 section, replace the expectation 'no client holds a heal within 500 ms' with the following. The bitten run fails -18's max check (max 1,688,286,323 ns > 500,000,000), and the clean run holds 96 of 96 heals within 500 ms. Count the 25 fast heals apart. 14 are in burst 0, each at burst_eviction baseline_age 0 (carry 77 (b)'s case). 11 are in burst 1, at baseline_age 33 or 34, named as a second case of the same cause: the stale copy is taken only when the ring slot is overwritten (ack.rs:109-114), after the base is chosen (room.rs:1163-1170). Say it is derived from timing, not shown client by client. State that all 71 slow heals followed a full state (71 overflow resyncs, full_state 1 on each), and that the heal record takes neither the sentinel nor a full state's tick (scenario.rs:994-999). State the mirror's 2,314 breaches on this run (the first: 'tick 64218: client 23: its baseline frame (age 35) is not mirrored'). Its PM-0001-2 value is 0.48, not the clean b1's 0.273333, and it is not a figure until the mirror item of point 8 is done.
- **Main session:** 1. Send the builder text. 2. Correct U4r's brief 2.4 r2's last bullet to this ruling's words, with a pointer. 3. Carry 77 (b) gains the second case (a late acknowledgement of a held tick, after the stale copy was taken). 4. On the card, add a Low carry for U4g: close the no-fallback bite's hole in ack.rs (take the stale copy when a tick becomes last_acked), with one ring unit test and a short bitten B1 run showing no heal within 500 ms. It rides U4g's npt-room work, with no push of its own.
- **Why:** Checked myself: scenario.rs:989-1012 (the heal rule), room.rs:1163-1170 (the base chosen before the tick is recorded), and the b1-nofallback lab log (2,314 breaches). The bite is still shown by -18's own max check, so no bar and no mark changes. Only the brief's test expectation changes, and that sentence is the main session's text, which I approved. Not Mustafa's: none of his words or decisions change. Risk tier 1.

### Point 5 (severity: Low; blocks the merge: no; to Mustafa: no)

- **Decision:** Ruling 2's point 1 (4) was wrong at the recording's end, and copy 3's reading is right. It now reads: 'every reading after each client's first take is L - 1, except at the recording's last L - 1 ticks, where the newest held command is the last tick itself and the measured depth is exactly (last tick - tick taken), one take per client and tick; no other deviation.' 7.2's reporting rule (ruling 2's words) stands. The report separates the 48 end-of-recording deviations, which are expected, from any other, and the verdict is unchanged. Two Lows on r6's assertion: precheck.rs:846 hard-codes `let lead = 2u32`, and only the first 64 of the sorted deviation list are compared. A change of P0's lead, or more than 64 deviations, would leave the assertion stale or partial. Both ride the next precheck.rs edit; neither triggers a push.
- **Builder:** Point 5, r6: keep copy 3's assertion as the rule (L - 1 at every take but the recording's last L - 1 ticks, exactly last tick minus the tick taken there, one take per client and tick, no other deviation). With your next precheck.rs edit, make two small changes. Take L from the leg's configured lead, not the literal `let lead = 2u32` (precheck.rs:846). Compare the whole deviation list against the computed end set, or assert that its count is at most 64 before comparing the first 64. The report's point 1 (6) line states the relation and its exception at the recording's last L - 1 ticks, as copy 3 wrote it (r6, L = 2, both legs: first take 1, then {1: 5,520} and 48 takes of 0 at tick 119).
- **Main session:** Send the builder text. On the card, record that ruling 2's point 1 (4) now reads as above, with a pointer to this ruling. In U4r's review, read r6's assertion.
- **Why:** c3-rul2b.log:155-168 shows the 48 zeros all at tick 119, one per client, on both legs, and copy 2's reading of them as first takes was wrong. With no command stamped after the recording's last tick, the relation cannot hold there under 7.2's pacing. The words amended are mine. Mustafa's 'A Low never triggers a push' (history.md:340) governs the two test Lows. Not for Mustafa; risk tier 1.

### Point 6 (severity: Medium; blocks the merge: yes; to Mustafa: no)

- **Decision:** The design is accepted, with conditions. The split into two halves is right: npt-lab cannot depend on npt-judge, because crates/lab/Cargo.toml is not U4r's lane (brief :1005), and ten 48-client suites cannot go into check.mjs's test step, where npt-lab's bin tests already take 90.7 s (c2-rul-reg.log:185). `contract` and `precheck` are subcommands beyond 2.7's list, accepted by this ruling, and the report's public-items line names them. Conditions: (1) The checker reads the lab log's class audit. On every run it prints breaches_count, and it fails a run whose count is above 0 unless the run's bite is catalogue-5/owner-to-all (r4's, which must breach). Today a run with 2,314 mirror breaches printed contract: PASS. A sixth doctored case (a breach added) fails by name. (2) The module doc (contract.rs:16) says what the code does at :140-147: missing carry-60 header params are a failure. (3) An unreadable SERIES file or log folder is not reported as the contract's FAIL (today exit 1, :350-351). It gets its own message and exit code, named in USAGE. (4) r2's suites must be re-makeable from the merge commit. The builder's runs are builder evidence only (ruling 2, point 33), and 7.1 decides carry 62 (h)'s setup check and -17's start on r2. So a run script holds each suite's lines, and the main session's run of it from the merge commit is the record. A suite that failed here (B1, ES) is expected to fail there too, and is recorded under this ruling, never re-run into a pass.
- **Builder:** Point 6, r2's design: accepted, with four changes. (1) contract.rs reads the lab log's class_audit: print breaches_count on every run, and fail a run whose count is above 0 unless its header's bite is catalogue-5/owner-to-all. Add a sixth doctored case to r2's unit test (a breach added: fails by name). (2) The module doc (contract.rs:16) says that missing carry-60 header params are a failure, as the code does (:140-147). (3) An unreadable SERIES file or log folder gets its own message and exit code (not 1), named in USAGE, so exit 1 means a contract FAIL only. (4) In B12, add ops/runs/c-u4r-r2.mjs. It holds each r2 suite's scenario, contract and npt-judge --only lines exactly as you ran them (c3-r2-suites.log, c3-r2-bites.log), plus point 3's two runs and point 4's bitten run. Each output goes under its own .gate/ folder named by the commit's first 12 hex digits, with -x. Run its npt-lab lines by hand on your tree, as B12 says. Afterwards run fmt and clippy, and npt-lab's tests in both builds. The report's public-items line names `contract` and `precheck` as subcommands beyond 2.7's list, accepted by the advisor's ruling 3, point 6.
- **Main session:** 1. Send the builder text. 2. After U4r's merge, run c-u4r-r2.mjs from the merge commit (labelled by that commit) before -17's judged runs and before deciding carry 62 (h)'s setup check (7.1). That run and its verdicts are the record. B1 and ES are expected to fail as they did here, and are recorded under this ruling. 3. In U4r's review, read contract.rs against these four points.
- **Why:** Checked myself: contract.rs:1-30, :136-150 and :334-368; ruling 2's points 5 and 33; brief 2.7 and section 4's lanes. The design meets r2 (b), (c) and (e) and keeps the judge's half real (npt-judge --only). But a checker that passes a run whose mirror failed 2,314 times lets mirror-derived figures (everyone in view, -57's pairing, the audit) through unchecked. And suites made only before the commit would leave 7.1's gate with no committed record. Not Mustafa's: no word or decision of his changes. Risk tier 1.

### Point 7 (severity: none; blocks the merge: no; to Mustafa: no)

- **Decision:** Both fixes are accepted. (a) The after-phase cap exception holds for every run that carries a bite id (scenario.rs:1034-1049). A run with no bite id still fails at the cap. C4 makes a measured run that names a bite INVALID (eval.rs:539-570), so carry 62 (e) is unchanged for every run the judge counts as measured. Deciding the unresolved ticks lost at the stop is carry 62 (c)'s 'at the stop: on L1 the runner may decide it there', which clean runs already do (b1 27, p2 32). Carry 62 (e) gains one reading: 'a bite run that reaches the after phase's cap stops there; its unresolved ticks are decided lost at the stop (carry 62 (c)); the stop is recorded as after_capped in the summary and run_lengths'. (b) Looking the mirror's frames up by the room's tick is right. tick16b's 5,828 breaches (the first 'tick 65536: client 0: its baseline frame (age 1) is not mirrored') became 0 in tick16c. On the clean P0 re-run every record is equal, except the run_lengths record's added after_capped field. That clean evidence lies below the wrap, so point 3's clean partner run shows it across the wrap.
- **Builder:** Point 7: both fixes accepted; change nothing. The report states (a) as carry 62 (e)'s reading for bite runs, in the advisor's ruling 3's words. It states (b) with tick16b's 5,828 breaches against tick16c's 0, and the clean P0 re-run equal record for record except the added after_capped field. Point 3's clean partner run shows (b) across the wrap.
- **Main session:** On the card, record carry 62 (e)'s reading for bite runs, with a pointer to this ruling. In U4r's review, read scenario.rs:1034-1049 and mirror.rs:396-414.
- **Why:** Checked myself: scenario.rs:1034-1049 (the exception is keyed on s.bites.id) and mirror.rs:396-414 (the lookup by the room's tick). The fact check matched the clean re-run record for record. No measured run's rule changes, and the reading is mine (carry 62 is the advisor's U4j ruling's text). Not for Mustafa; risk tier 1.

### Point 8 (severity: Medium; blocks the merge: yes; to Mustafa: no)

- **Decision:** Three things in copy 3's part depart from the brief or the rulings, plus seven Lows. (A) Medium: the mirror cannot follow the no-fallback bite's stale baseline. FRAMES_KEPT = ACK_RING_TICKS + 2 = 34 (mirror.rs:32), while the bitten room codes against a stale copy aged 35 and more. On a missing frame, mirror.rs:409-414 returns early and leaves that tick's stats stale. Result: 2,314 breaches on r2/b1-nofallback, which copy 3's notes left out. A bite run takes its parts' validity (B7), so -18's bitten judged run would be ruled valid or INVALID on a figure the mirror cannot compute. The fix is in U4r's lane: keep each client's frame at the room's `ClientSlot::last_acked()` (room.rs:238) beyond FRAMES_KEPT, as the room's stale copy does. Clean runs are unchanged, since there the baseline is always inside the ring. (B) -18's L1 B1 runs, the clean ones and the bitten one, are 7.1's, and now move to U4g under point 1. (C) Point 33's labels miss .gate/u4r/regen/ and regen-all/. The Lows: (1) SERIES.md's new baseline_age row says '-1 when it had none', but 0 means coded against the defaults (no baseline) and -1 means no snapshot sent that tick (room.rs:216-219; scenario.rs:914). (2) The notes say 22 rows in 'Names added with U4r'; there are 21 (git diff). (3) tick16's flag is 0 from 65,715, not 65,746. (4) The bitten run's 0.48 is not 'as the clean b1' (0.273333). (5) header.params.burst_spacing_ns has a row naming npt-lab (L1) as its writer, but U4r's runner does not write it. (6) The P0 bytes sum 27,817 is a sum of rounded parts (27,816.x). (7) SERIES.md:368 names npt-room as everyone_in_view's writer, while npt-lab writes it from its mirror. That row is the main session's (U4r adds rows only). The suite table, point 33's binary SHA-256 labels, the per-profile bytes and the 10-minute sizes all hold (the fact check recomputed them).
- **Builder:** Point 8: one fix and seven small items, all riding your current work. (A) mirror.rs: keep each client's frame at the room's `ClientSlot::last_acked()` (room.rs:238) beyond FRAMES_KEPT, so a snapshot coded against the no-fallback bite's stale copy finds its baseline. Show it once (-x) on b1's settings in the bites build with `--role bite --bite PM-0001-18/no-fallback`: 0 breaches, its heal and everyone-in-view figures reported as they come, and the verdict sent to the advisor if anything but the expected max FAIL (an INVALID by pre:PM-0001-2 is expected while the room is unfixed: report it). Also show that the clean P0 run (r2/p0-s1's settings) stays equal record for record. Keep r2/b1-nofallback as it is. (B) Fix the baseline_age row's words in your SERIES.md subsection: '0: coded against the per-type defaults (no baseline); -1: no snapshot sent that tick'. (C) Label .gate/u4r/regen/ and regen-all/ 'tree: 2fbec77 plus the uncommitted U4r changes; no header'. (D) Write header.params.burst_spacing_ns (the spacing the run used, in ns) in every B1 header, so its row holds on L1. (E) In the notes and the report: 21 rows, not 22; tick16's flags 0 from 65,715 (first collapse 65,567); the bitten run's PM-0001-2 value 0.48; the P0 sum 27,816 (27,817 is the sum of rounded parts); and the window figures as means. (F) The report states that npt-lab writes tick.s.everyone_in_view from its mirror of the room's datagrams (SERIES.md:368 names npt-room).
- **Main session:** 1. Send the builder text. 2. At its next SERIES.md edit, the main session corrects row :368's writer (npt-lab, from the room's datagrams), or states the reading. 3. In U4r's review, read mirror.rs's last_acked keep and the bitten run's 0 breaches.
- **Why:** Checked myself: b1-nofallback/lab.jsonl class_audit (2,314 breaches, the first at age 35); mirror.rs:32 and :396-414; room.rs:238 (last_acked is public). The fact check covered the row count, the baseline_age semantics, the labels and the bytes. The mirror fix is U4r's own code, in its lane, and -18's bitten run cannot be judged on a figure the mirror cannot compute. The Lows ride that push ('A Low never triggers a push', history.md:340). Not for Mustafa; risk tier 1.

### Further findings

- **Medium:** PM-0001-18's heal reading (a fresh delta decoded within 500 ms of the link's return; the advisor's reading of 2026-11-06 in ADR 0001's row) passes a room whose clients hold none of the other soldiers for 4-11 s after each burst. On r2/b1, 96 of 96 heals came within 48-130 ms, next to an everyone-in-view share of 0.273333. Only pre:PM-0001-2's share, applied to -18 by Q22, catches it, and ADR 0001:1589 names that share 'for the downstream rows'. **Fix:** No change now: keep pre:PM-0001-2 on -18's L1 parts (point 1). Add a line to the readings PR for Mustafa's go before R5: whether 'a loss burst heals' should also bound the time until each client's set holds every relevant soldier again. Recommendation: keep the precondition as the check that catches it, and decide on a new check only once U4g's fix shows the recovered time. It is his because ADR mark words are his to accept (ADR 0001 is Proposed).
- **Medium:** My own Q22 (plan §4.12 :416-424) put pre:PM-0001-2 on -17's and -18's B1 parts after checking only the shot band (§4.13 :438-440). Its application of the everyone-in-view share to every -17 and -18 L1 run is not on the readings-PR list (plan §11 :1080-1093 lists only Q22's -22 exception), although ADR 0001:1589 scopes the share 'for the downstream rows'. **Fix:** The main session adds the reading to plan §11's readings-PR list at its next plan edit, in point 1's words, so Mustafa's go on the readings PR covers it.
- **Low:** A clean run past the 16-bit wrap was never made in copy 3 (every suite ends by tick 64,968). So U4R-Q7 (a)'s premise, that the clean -17 runs cross the wrap 1,800 ticks in with no effect, and the mirror's room-tick lookup past 65,536 rest on code reading only. **Fix:** Point 3's clean partner run (measured build, --tick-start 64936, the wrap at measured tick 300) shows it before U4r's report; if it fails, it goes to the advisor (condition 5).

## The netcode architect's read

### Point 1 (design gap)

- **Mechanism:** Code paths are under /home/user/Shamble-Frontlines/.gate/u4r-snap1/spikes/netcode-proof/crates/. Logs are under /home/user/Shamble-Frontlines/.gate/u4r/r2/b1. Recovery goes through three stages, each read from the code and checked against the logs.

(1) Every client's baseline is evicted at every burst. This is designed. The ring keeps ages 0-31 (sim/src/ticks.rs:58-70: 32 ticks, 1.067 s at 30 Hz). baseline() takes the newest fully acknowledged tick at age 1-31, or none (room/src/ack.rs:181-193). B1 is P2 plus 1 s of total loss in both directions, decided at send time (l1/src/lib.rs:197-201, :1097, :1271, :1426). The last ack the room gets before a burst names a tick 2-4 ticks before the burst starts. Add 1 s and the 100 ms RTT and that tick is past age 31 at every burst, for every client (burst_eviction evicted 1 in 96 of 96). The room then codes against the per-type defaults (room.rs:1163-1170, the `_ => (&empty, 0)` arm), as ADR 0001 part 6 (:276-279) and u4-build-plan.md §4.5 (:296-300) say. Nothing sends a full state on eviction: full_state_due is set only at join (room.rs:656), on a Resync (:734) and on an overflow (:1030). The mirror reads the room's intent before the link (lab/src/mirror.rs:1-4, :314-337), so the flag stays 1 through the burst (48 at 64186-64214) and drops at the eviction tick (46 at 64215, 14 at 64216, 0 from 64217).

(2) The event backlog takes the whole window first. This is the known defect of carry 78. The window is 1,066-1,067 B a tick (room/src/encoder.rs:14, :26-31) and is already full in steady state: client 0's snap_wire_bytes is about 1,028 B every tick before, during and after the burst. snapshot() packs the own section first, then every queued gameplay event oldest first, resent until acknowledged (room.rs:1208-1235), and only then entity changes, nearest 8 first (:1236-1292). After the burst each client's queue holds 546-631 events (burst_heal queue_max_count), about 5 KB at 74 bits a hit. They drain at about one window per ack round trip, against an inflow of about 16 a tick (events_queued 20,176 for slot 0 over 1,233 ticks, about 490 a second; §4.7 sized the cap on net F121's about 350). lab.jsonl's carried-field counts show about one positional record per client per tick (its own soldier) at 64218-64248. So for about 1 s no snapshot carried any entity change, the nearest 8 included. Carry 78 (u4s-carries.md:546; advisor-u4e-r1.md:577-584) deferred the fix to U4g on the premise that "no U4e or U4r run meets it"; this suite meets it. Point 2 touches this stage: a brawl that hits less in SS would shrink the backlog, but cannot fix stage 3.

(3) The set is rebuilt from nothing through the ack pipeline. This is the bulk of the time, about 7-11 s. Each tick's frame is its baseline plus that tick's deltas only (room.rs:1351, :1369; encoder.rs:191-202). The baseline lags one ack round trip (B1: 100 ms ± 20 ms, about 3-5 ticks). So the acknowledged set grows by one tick's spare room per round trip, while the priority accumulator rotates which Adds each in-flight tick carries (room/src/interest.rs:157-187; room.rs:1272-1295). SS's whole relevant set is about 8.8 KB of Adds by seam.rs:249-282's layouts (soldier 129 bits, projectile 179, part 113, hull 97; 47 + 300 + at least 80 + 15 records). That has to fit into about 1,000 B a tick, which updates of the held entities and the events already share. Per 30-tick window: 47, 0, 0, 0, 0, 0, 4, 12, 23, 31, 40, 44, 45, 46 slots holding everyone. Flag-0 ticks per client per burst: median 225-245, max 370 (12.3 s).

The arithmetic that bounds the options: 0.99 of a 600 s run (18,000 measured ticks) allows 180 flag-0 ticks per client, which is 18 per burst at 10 bursts. The worst client's single burst (370 ticks) is already over a whole run's allowance, so no B1 run with even one burst in 10 minutes can meet PM-0001-2 with this room. Even with no events and no updates, 8.8 KB through windows of about 990 B at a 3-5 tick round trip costs about 27-45 ticks per burst. Neither point 2's tuning nor carry 78's reorder alone can get to 18.

What U4e's plan and rulings meant:
- Eviction at every B1 burst is intended. §4.5 logs burst_eviction "so -18's bite shows whether it could bite", and §4.14 :461 builds -18/no-fallback on "a gap evicts B". A 1.067 s ring against a 1 s burst plus RTT guarantees it.
- The queue cap was sized so that a burst does not resync clients (§4.7 :335-336; carries 57, 58).
- -18's heal is a fresh delta within 500 ms. The defaults-coded snapshot gives it: 96 of 96, 48-130 ms, no full state.
- No text says when the client's set must be whole again.
- §4.12's table (Q22, :416-424) put pre:PM-0001-2 on -17's and -18's B1 parts after checking only the shot band (§4.13 :438-440).
- ADR 0001's encoding-options table, row E (:815), counts "a spike when a baseline expires" as a cost. That assumes a large catch-up snapshot, which the per-client hard cap (part 6) turns into this slow rebuild.

So: designed eviction and defaults rule, a known packing-order defect worth about 1 s, and a precondition whose fit with B1 was never checked.

Mustafa: his recorded word on PM-0001-2 is about ES's events floor (history.md:338, 2026-11-11: "The 9.5 floor (PM-0001-2) does not move."), not the 0.99 share, which ADR 0001:1589 calls a "proposed share". Every option below except deferral changes words in ADR 0001 or in the catalogue.
- **Option:** Missing soldiers first, in npt-room, with carry 78's reorder: in snapshot(), place the Adds of relevant soldiers the baseline lacks (ES: and hulls) and the nearest 8's changes before the event loop (room.rs:1208-1292). **Cost:** About 20-40 lines in room.rs plus two npt-room tests: an evicted baseline under SS load gives a first defaults-coded frame holding all 47 soldiers, and carry 78's backlog case. npt-room is U4e's crate, outside U4r's lane, so this needs a lane exception, a small npt-room change set before U4r's judged B1 runs, or U4g. Events lose about one round trip of room per burst; queue high-water rises by about 60-80, still under the 1,024 cap, and carry 58 reports it. Projectiles and parts still take seconds to rebuild (carry 49's B1 share, reported only). ES at B1 (47 soldiers + 15 hulls, about 940 B) would sit at the datagram limit. **Changes:** Catalogue :129-130 and ADR part 9 (d)'s priority words gain "relevant soldiers the baseline lacks, first". By arithmetic, 47 × 129 bits ≈ 758 B plus the own section fits one 1,066 B window, so every post-eviction frame holds everyone: about 0-1 flag-0 ticks per burst. That must be shown by a new build's B1 suite, never by re-running this build. -18's heal and its bite are unchanged. No pass bar moves.
- **Option:** A full state on eviction, in npt-room: set full_state_due when snapshot() would code from the defaults for an admitted client, or, to avoid the burst, on the first Acked::Old after an eviction (room.rs:782). **Cost:** About 10 lines plus 2 tests. The no-fallback bite must skip it, or the bite cannot bite. About 9 KB per client per burst goes on the reliable stream outside the 256 kbps window (room.rs:1097-1131 never charges it; C-U6-16 weighs charging it): 48 × ~9 KB at every burst, ADR row E's spike back. Snapshots stop while it is in flight (room.rs:1087-1089), and a FullState never counts as a heal (marks.json -18 notes; brief :1169). Triggered at the eviction tick, the full state falls inside the burst for 34 of 48 clients at burst 0 (A+32 ≤ k 480 < 480.88) and all 48 at burst 1, and waits for the stream's backed-off PTO (L-c3). That risks -18's 500 ms maximum. Triggering on the first Old ack risks less but leaves a few flag-0 ticks. Same lane question as above. **Changes:** ADR 0001 part 6's "with no usable baseline the server codes against the per-entity defaults" (:277-278). -18's heal path (carry 58's concern) and the heal record's full_state field, which becomes 1 on every burst. No pass bar moves.
- **Option:** A longer ring: ACK_RING_TICKS from 32 to, for example, 64 (sim/src/ticks.rs:65-70; ADR part 6 calls it "data, proposed 32"), for the room and the client together. **Cost:** One constant plus every pin to 32. The mirror's FRAMES_KEPT and the client ring derive from it; ack.rs e4 uses len(). Room memory roughly doubles (about 55 MB to about 110 MB for 48 clients, my arithmetic at about 450 records of about 80 B per frame). npt-sim is U4s and U4d's crate. B1 never evicts any more, so -18/no-fallback cannot bite on B1 (burst_eviction 0; plan §4.14 :461), and -18's L1 part stops exercising the from-defaults path at all. **Changes:** ADR part 6's number. -18's bite needs its own longer-gap setting (marks.json, the plan). With no eviction, skipped entities stay at the baseline's values, so the flag holds and heals are normal deltas. No pass bar moves, but -18's bite design does.
- **Option:** Defer -17's and -18's L1 B1 parts out of U4r: they stay NOT-RUN at U4r, with a carry to the unit that changes the room's recovery. U4g already owns carry 78's reorder in npt-room. **Cost:** -18 has no judged L1 evidence at U4r (L1 B1 is its only L1 part); -17 keeps 3 of its 4 L1 parts. The U4 plan's RQ1 "bursts (-18)" moves to that unit. **Changes:** No code, no ADR word, no bar. The carry names this mechanism and the target: at most 18 flag-0 ticks per client per burst at 10 bursts in 600 s. U4r's report states the B1 suite's FAIL (condition 5: never re-run into a pass) and corrects carry 78's premise.
- **Option:** Read the everyone-in-view flag from the room's relevant set (interest) instead of the mirror frame: an npt-room getter over ClientSlot's `sees` (room.rs:188, filled :913-940; for ES also the relevant hulls), and logs.rs:538-541. **Cost:** About 5-15 lines plus a test, and an npt-room API addition (the plan's getter list, u4-build-plan.md:194). It makes the precondition blind to the encoder's post-burst collapse, which is real: each client's set is empty for about 8 s after each burst. It does not help point 3, where the interest sets themselves lose soldiers. **Changes:** Arguably ADR 0001:1589's own words ("every client's relevant set holds all 47 other soldiers"), and SERIES.md:368 names npt-room as the writer, so it is a reading the advisor can make. But it changes the figure's input and turns B1's 0.273 into about 1. NAMED: in effect this moves the precondition's bar on B1.
- **Option:** Exempt B1 from the precondition, or window it: (a) drop pre:PM-0001-2 from -17's and -18's L1 B1 parts, like -22's exception; (b) give PM-0001-2's share a window on B1 that excludes each burst's start to its return + W. **Cost:** (a): marks.json, README §4's sentence and plan §4.12's table. ADR 0001:1589 names every run of PM-0001-4 to PM-0001-27 as downstream, so this is an ADR reading that needs Mustafa's go through the readings PR. (b): judge and marks code, with W ≥ 12.5 s for the worst client, which leaves about 22 % of a 10-burst run uncovered. **Changes:** Both loosen a validity bar. NAMED: each moves a bar or its coverage.

### Point 3 (design gap)

- **Mechanism:** Code paths are under .gate/u4r-snap1/spikes/netcode-proof/crates/; logs are .gate/u4r/r2/tick16c. Two effects of the bite, both past the wrap and both by the bite's design.

(1) Decoding stops and the room evicts. The tick16 bite cuts only the snapshot header's tick to 16 bits: proto/src/snapshot.rs:532 is header_tick's one use (proto/src/quant.rs:133-139). FullState keeps its 32-bit tick. Past the wrap each header names 0, 1, 2 ..., which the client finds too old for its ring and earlier than its first header. It files the sentinel under that tick, decodes nothing, and moves neither its newest tick nor its acknowledgement (client/src/lib.rs:493-529: too_old, old_header, bad_header). The room gets no new acknowledgement, evicts at about wrap + 32 and codes from the defaults (flag 48 to 65566, 0 at 65581-65596). The unacknowledged events overflow the 1,024 cap and force full states (140 overflow resyncs, the first at 65605). The client takes those (32-bit tick) and acknowledges them, giving about 30 ticks of flag 1 (48 at 65611-65626), then the room evicts again.

(2) The bots stop commanding and the fight disperses. The bots give a command only when the endpoint's newest tick moves (bots/src/lib.rs:183-189), so after the wrap no client commands: 1,532 commands in all (about 48 × 32), steps without one 48,772. Option B re-runs each soldier's last command every tick (room/src/input.rs:127-140): fixed walk, frozen aim, fire held. Reliable events fall to about 150-270 a second, from p0-s1's 475-600, while shots hold at 672-720. The second overflow cycle restores only 30 of 48 (65686-65701). None of the 44 measured-phase overflow full states holds every other soldier (measured flag sum 0), and each of them is the room's relevant set whole (room.rs:1104-1107). So past the wrap the room's interest sets themselves lose soldiers, not only the mirror frames.

With tick_start 65506 and a 300-tick warm-up, the wrap falls at warm-up tick 30 and the whole measured phase lies in the collapse. The judge gives a bite run the validity of the parts it matches (judge/src/eval.rs:1408-1410, the advisor's B7 ruling), and -17's L1 P0 part carries pre:PM-0001-2, hence INVALID. The figure itself would fail: the client's 48,772 sentinels lie under ticks the room never logged (widened past 2^32 in client.jsonl), and decoded() counts every orphan in every phase (judge/src/figures.rs:422-425).

Carry 73 (u4s-carries.md:328-337) foresaw the coverage gap ("the judge returns a mismatch count before it reads a coverage gap"), not the precondition. Point 7's two fixes remove the cap failure and the mirror's breaches; they do not touch this. The judged -17/tick16 bite run at the part's own settings (wrap 1,800 ticks in) would be INVALID the same way.
- **Option:** Put the wrap inside the measured phase, before the collapse (run settings only). Short run: --tick-start 65191 (65536 − 300 − 45), --warmup-s 10, --measured-s 2, so the measured span is 65491-65550 and the wrap falls at measured tick 45. Judged bite run: a tick_start that puts the wrap in the last 40-150 measured ticks. **Cost:** Carry 73's "ticks_below_wrap = 30" is re-read as counted from the measured start (the header would say 345), and brief 7.1's "the part's own settings" gains a tick_start of its own for the judged bite run. PM-0001-2's per-second bands are judged over 2 s (shots 672 and 720 per second; portal changes 1 a second on p0-s1, to be confirmed on the run). It spends PM-0001-2's 1 % slack but moves no bar. **Changes:** No code: the scenario's options exist. The room keeps the pre-wrap baseline until about 65567 (last ack 65535 at P0, ring 32), so the mirror's flag stays 1 on every measured tick (16 ticks of margin), and 15 post-wrap ticks leave the soldiers no time to disperse. The client files sentinels under 0, 1, 2 ... from the first post-wrap group, so the figure FAILs with at least 48 orphans. The run also crosses the wrap inside the measured phase, which -17's bite text ("the bite run must cross it") asks for and today's run does not do.
- **Option:** Exempt the -17/tick16 bite run from pre:PM-0001-2: a per-bite validity exception in marks.json and in eval.rs:1408-1410's rules_of, with its reason in README §4 beside -22's (the bite stops every client's decoding and commands, so the load disperses by design). **Cost:** The main session's code: judge, marks.json schema, a fixture and a test. It narrows the B7 ruling for one bite. **Changes:** NAMED: it loosens a validity bar for a bite run (not a measured run's bar). The figure's FAIL (orphans) then shows on the run as it is.
- **Option:** Carry 73 accepts INVALID, with the figure shown beside it: the contract checker (lab contract.rs) or npt-judge prints the bite run's mismatch count without the validity gate. **Cost:** Weaker evidence: the judge's verdict on the bite stays "invalid", so check.mjs and the R run plans would read "bite invalid", not "shown". **Changes:** Carry 73's words ("never INVALID"). No bar moves.
- **Option:** Keep the load alive under the bite from the lab (bots commanding at every step). Not viable. **Cost:** Every command would be stamped from the frozen newest tick plus the lead (client/src/lib.rs:394), arrive late, and still leave option B running the old command. **Changes:** Nothing useful. Listed so it is ruled out.

### Point 4 (defect)

- **Mechanism:** Code paths are under .gate/u4r-snap1/spikes/netcode-proof/crates/; logs are .gate/u4r/r2/b1-nofallback. The fast heals come from a hole in the bite (bites build only), and part of it is already known as carry 77 (b).

How the hole works. Under PM-0001-18/no-fallback the ring keeps a stale copy of the last acknowledged view only inside record(), when the slot holding last_acked is overwritten (room/src/ack.rs:109-114). stale() returns it only while its tick is last_acked (ack.rs:207-212). But snapshot() chooses its base before it records the tick (room.rs:1163-1170 against :1369). So on the first tick with no baseline after any new last_acked, stale() is still empty and the bitten encoder falls through to the defaults arm. That one snapshot is coded from the defaults: it decodes alone, the client acknowledges it, and the client has a fresh baseline, so the bite has nothing left to bite. Whether it counts as a heal depends on its due instant against the link's return. The heal record (lab/src/scenario.rs:989-1012) needs a non-zero hash, due_ns(tick) at or after the return, and not a FullState's tick.

Burst 0 (start k 450.88, return k 480.88), 14 fast heals. Clients whose last pre-burst ack reached the room for k 449 get the defaults tick at k 481 (due 16.033 s, after the 16.029 s return) and heal. All 14 fast heals had baseline_age 0 at the eviction event; the 34 slow ones had 33 or 34. This is carry 77 (b)'s case (u4s-carries.md:371-374).

Burst 1 (return k 930.33), 11 fast heals by a second case of the same cause. Every A+32 falls inside this burst (no age 0 among the 48). Some clients had decoded k 900 before the downlink burst, and their first post-burst Input (acking k 900) reaches the room before tick 932. k 900 is still held then (newest sent 931; ack.rs:138-142), so last_acked moves to 900 after the stale copy of A was taken. Tick 932 then has neither a baseline (age 32) nor a stale copy of 900: defaults again, decoded, heal at 103-136 ms. Their eviction events show 33 or 34, so carry 77 (b)'s words do not catch them. This case rests on timing derivation: no per-client baseline age per tick is logged, so it is not verified client by client.

The other 71 never decode. The client refuses a baseline age of 32 or more before it looks at its frame (advisor-u4e-r1.md:638-642; 2,451 sentinels). Their queues reach the cap (high-water 1,024), and the overflow resync's FullState heals them at the first snapshot after its acknowledgement: 1.10-1.69 s. (71 = overflow resyncs 71 = full_state 71; carry 58 sends overflow resyncs in judged -17 and -18 runs to the advisor.) So the bite does fail -18's max check (1.688 s against 0.5 s).

The brief's 2.4 r2 sentence ("no client holds a heal within 500 ms") does not hold for the bite as built, and it contradicts carry 77 (b), which the brief itself cites (:109, :459, :1169). The heal record is honest: the 71 slow heals show it takes neither the sentinel nor the FullState's tick (scenario.rs:994-999), or they would have healed at about 50 ms. So the defect is in the bite, with no measured-build effect, and the brief's expectation is the wrong half of the pair.
- **Option:** Report it, counted apart, with no code: r2 says the bite is shown by -18's max check (1.688 s) and counts the 25 fast heals apart. 14 are carry 77 (b)'s case (eviction age 0); 11 are the late-acknowledgement case, named as a second case of 77 (b)'s cause. The advisor corrects the brief's r2 sentence. **Cost:** The burst-1 cause stays derived, not shown client by client. **Changes:** The brief's test expectation, not a pass bar: -18's 500 ms check is unchanged. Carry 77 (b) gains its second case in the report.
- **Option:** Turn the derivation into evidence, in U4r's lane: the heal record gains the room's baseline age for the snapshot that healed (scenario.rs keeps ClientSlot::baseline_age() per client and tick for the ticks after each return), and r2 asserts on the bitten run that every heal within 500 ms came from a defaults-coded tick (age 0) and every other one came after a FullState. **Cost:** About 10-20 lines in scenario.rs, one SERIES.md row, one assertion. **Changes:** The brief's wording as in the first option. The cause of every fast heal is then checked, not inferred. No bar moves.
- **Option:** Close the hole in the bite (npt-room, bites build only): take the stale copy when a tick becomes last_acked (ack.rs:156-158: under keep_stale, clone that entry into stale), so stale() always holds the last acknowledged view and the bitten encoder never codes from the defaults once a client has acknowledged anything. **Cost:** About 5 lines plus one ring unit test: at the eviction tick, and after a late acknowledgement of a held tick, stale() is the last acknowledged frame. One Frame clone per new acknowledgement, under the bite only. npt-room is U4e's crate (a lane question). A new bitten run is needed to show the result. **Changes:** Expected: 0 of 96 heals within 500 ms, so the brief's sentence holds as written. Carry 77 (b)'s case disappears and its words change. No measured-build change; no bar moves.

## The fact check

### Point 1

- **held:** B1 suite: 48 clients, seed 1, 10 s + 30 s, two 1 s bursts Evidence: .gate/u4r/r2/b1/room.jsonl:1 header: clients 48, seed 1, profile B1, warmup_ticks_count 300, measured_ticks_count 900, burst_starts_ns [15029325961, 30011005135]; l1/src/lib.rs:197-201 B1 = P2 plus a 1 s burst at each start
- **held:** PM-0001-2 FAIL, everyone in view 0.273333 (needs >= 0.99) Evidence: .gate/u4r/r2/b1-verdict.json [0] PM-0001-2 verdict FAIL, part 'SS runs', check 'everyone in view...' value 0.273333 needs >= 0.99; recomputed from room.jsonl with judge/src/figures.rs:24-54 (least per-slot share of measured ticks): 246/900. Note: PM-0001-2's own verdict is FAIL, because its 'SS runs' part selects scene SS and 48 clients at any profile (marks.json PM-0001-2 parts), so the B1 run counts in -2's own part too
- **held:** -17 and -18 L1 B1 INVALID by pre:PM-0001-2 Evidence: b1-verdict.json [1] -17 part 'L1 B1' INVALID and [3] -18 part 'L1 B1' INVALID, invalid_why 'pre:PM-0001-2: everyone in view ... = 0.273333'; marks.json -17 and -18 parts 'L1 B1' select {layer L1, scene SS, profile B1, clients 48}, min 3, validity ['pre:PM-0001-2']; judge/src/eval.rs:958-996 (the run must pass the pre mark's checks for its matched parts)
- **held:** What marks.json's PM-0001-2 check requires Evidence: marks.json PM-0001-2 check {name 'everyone in view on at least 99 % of each client's snapshots', figure everyone_in_view_min_share, op >=, value '0.99', parts [SS runs, ES runs]}. The 99 % is 'proposed share' in ADR 0001's row (docs/adr/0001-multiplayer-netcode.md:1589)
- **held:** -57 passes Evidence: b1-verdict.json [2] -57 'L1 B1' run valid, checks pass: 6974 <= 10000, 2750 <= 10000, 88060 <= 180000; the part itself is NOT-RUN '1 of 3 valid runs'
- **held:** The mirror logged 0 breaches Evidence: r2/b1/lab.jsonl:1315 class_audit breaches_count 0; c3-r2-suites.log:54 '0 breaches'
- **held:** After each burst every client's baseline is evicted (burst_eviction, baseline_age 0, all 96) Evidence: r2/b1/room.jsonl: 96 burst_eviction events, all {evicted 1, baseline_age 0}, 48 at tick 64217 (burst 0) and 48 at 64667 (burst 1); code: room/src/ack.rs:66-69, :180-205 (32-tick ring, age 32 evicted, then the per-type defaults), room/src/room.rs:1160-1170
- **held:** The room then sends about 1,000 B a tick per client (snap_datagram_bytes 47.9 KB for 48 per tick before, during and after) Evidence: room.jsonl tick.s.snap_datagram_bytes: 48 datagrams every tick, each 996-1,000 B; per-tick sum mean 47,954 B before burst 0 (64036-64186), 47,966 during, 47,953 after, 47,939 after burst 1. The cap is Limits::SNAPSHOT = 1,000 (proto/src/limits.rs:25; room.rs:1172). Context: this is the steady state at P2/B1 (r2/p2 measured mean 999.2 B per datagram), not something the burst causes
- **held:** Each client's set is rebuilt from nothing over about 10 s Evidence: Approximate. After eviction at 64217, each slot's first tick holding everyone is 119-320 ticks later (4.0-10.7 s); its last 0 before 64630 is 150-414 ticks later (median 256); the all-48 state recurs on only 2 of the 450 ticks before burst 1 (47 at 64660). The window means reach 46.3 at window 14 (about 12.9 s after eviction)
- **held:** Per 30-tick window from 64,186: 47, 0, 0, 0, 0, 0, 4, 12, 23, 31, 40, 44, 45, 46 slots holding everyone Evidence: Holds only as the window's mean number of slots, truncated to an integer (the notes don't say so): means 47.9, 0.5, 0.0, 0.0, 0.0, 0.6, 4.1, 12.4, 23.1, 31.9, 40.1, 44.2, 45.8, 46.3. Rounded they would read 48, 1, 0, 0, 0, 1, 4, 12, 23, 32, 40, 44, 46, 46. Flags drop at 64215-64216 (46, 14), at the link's return, before the eviction tick 64217
- **held:** At 10 bursts in 600 s every judged L1 B1 run would be INVALID for -17 and -18 Evidence: Extrapolation from this run's per-burst cost (see computed): least share <= 0.919 at 10 bursts, assuming each burst costs what burst 0 did. Brief :1169 sets the judged run at 10 min with 10 bursts; -18's notes in marks.json say 10 bursts (proposed)
- **held:** npt-room is outside U4r's lane Evidence: u4r-brief.md:1009 'Never: crates/sim, crates/proto, crates/l1, crates/link, crates/room, ...'
- **not held:** PM-0001-2 holds his floor (history.md) Evidence: The only Mustafa words on PM-0001-2 in history.md are at :338-339 (2026-11-11 about 11:50 UTC, verbatim): 'The event-rate drift in the estimate scene: fix the scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does not move.' What is true: The mark does hold his floor, but that floor is ES's 9.5 reliable-events bound, which is point 2's check, not the everyone-in-view check that fails here. history.md holds no Mustafa decision on the 0.99 share or on pre:PM-0001-2: ADR 0001:1589 calls the share 'proposed', and marks.json's -17 and -18 notes credit pre:PM-0001-2 to 'the advisor's Q22 ruling'. docs/DECISIONS.md has no PM-0001-2 entry
- **Computed:** Least slot 246 of 900 measured ticks = 0.273333 (max slot 569). Per-window figures from 64186 are window MEANS of slots holding, truncated: 47.9, 0.5, 0, 0, 0, 0.6, 4.1, 12.4, 23.1, 31.9, 40.1, 44.2, 45.8, 46.3. Burst 0 window 64187-64666 (480 ticks): zero ticks per slot min 145, mean 237.1, max 397. 10-burst extrapolation: 18,000 measured ticks, at least 10 x 145 = 1,450 zero ticks per slot -> share <= 1 - 1450/18000 = 0.919 < 0.99; even one burst: worst slot 397 > 180 (1 % of 18,000) -> 0.978.

### Point 2

- **held:** ES suite (carry 59): ES, P0, 48 clients Evidence: .gate/u4r/r2/es/room.jsonl:1 header: scene ES, profile P0, clients 48, behaviour brawl, marks [PM-0001-2], tick_start 0, weapon_interval_us 666667, weapon_damage_count 10, soldier_health_count 1000
- **held:** PM-0001-2 FAIL on ES reliable events, 50.05 a second (needs at most 10.5) Evidence: es-verdict.json [0] FAIL, why 'ES runs: ... ES: 10 reliable events per second, mean at most +5 % = 50.05, needs <= 10.5'; room.jsonl second records (measured): 1,001 / 20 = 50.05
- **held:** Its other 11 ES checks pass (shots 72, projectiles 32.38, spawns 21.1, everyone in view 1) Evidence: es-verdict.json ES runs part: 12 checks, 11 pass: 48 soldiers, 48 clients, 15 vehicles, everyone in view 1, shots 72 in [68.4, 75.6], reliable >= 9.5 passes at 50.05, projectiles 32.383333 in [30.4, 33.6], spawns 21.1 in [20.064, 22.176]
- **held:** The band in marks.json is 9.5 to 10.5 Evidence: marks.json PM-0001-2 checks 'ES: 10 reliable events per second, mean at least -5 %' (>= 9.5) and 'at most +5 %' (<= 10.5), series second.v.reliable_events_count, parts [ES runs]. Mustafa's words fix the 9.5 floor only (history.md:338-339); 50.05 is above the ceiling
- **held:** The brawl aims at its target's exact position (mirror.rs view; bots brawl_aim) Evidence: bots/src/lib.rs:226-245: brawl_aim returns the target's position from view, z + AIM_ABOVE_FEET_CM (100, :54), re-targeting every 30 commands (:52); view = mirror.view(c.slot, newest) (lab/src/scenario.rs:959-960; mirror.rs:212-230: the soldiers' grid positions in the frame the room means that client to hold at its newest tick)
- **held:** About 70 % of its 72 shots a second hit Evidence: Computed 68.5-69.5 % (see computed); reliable count = kinds 2 or 3 (lab/src/logs.rs:115-117; sim/src/world.rs:2005-2011: 2 Hit, 3 Kill). The logs hold no separate hit count, so the share is bounded, not exact
- **held:** The brawl is U4r's (npt-bots); tuning to a band is a design choice (carry 59; U4d-F's 7.250 figure) Evidence: u4r-brief.md:997 crates/bots/src/ in U4r's files; carry 59 (u4s-carries.md:274-283): 'U4r's report gives that figure [7.250] beside its ES suite's event rate, and a missed event band is not read as a bot defect before the advisor has seen both'
- **Computed:** Measured 20 s: reliable events 1,001 and shots 1,440, so 50.05/s and 72/s. Reliable = Hit (kind 2) + Kill (kind 3). Kills <= 1,426 reliable over the whole run / 101 = 14 (damage 10, health 1,000), so measured hits are 987 to 1,001, a hit share of 987/1440 = 68.5 % to 1001/1440 = 69.5 %.

### Point 3

- **held:** Run: SS P0 seed 1 from 30 ticks below the wrap, --bite PM-0001-17/tick16 Evidence: .gate/u4r/r2/tick16c/room.jsonl:1 header: bite PM-0001-17/tick16, role bite, tick_start 65506, ticks_below_wrap 30, build features [bites], binary a95d331b...
- **held:** After the two fixes the run ends clean and the mirror has 0 breaches Evidence: tick16c footers 'ended':'clean' on room, client and lab; lab.jsonl class_audit breaches_count 0; run_lengths after_capped true, after_ticks_count 150
- **held:** The judge gives INVALID by pre:PM-0001-2, everyone in view 0 Evidence: tick16c-verdict.json -17: bite PM-0001-17/tick16 status 'invalid', invalid_why only 'pre:PM-0001-2: everyone in view ... = 0', checks []. Same in tick16b-verdict.json. The first try's verdict (tick16-verdict.json) was INVALID for the 3 missing footers AND pre:PM-0001-2 = 0; notes :712 name only the cut logs
- **held:** Everyone in view is 0 on every client and measured tick Evidence: tick16c room.jsonl: no flag is 1 on any of the 600 measured ticks (65806-66405)
- **not held:** ... from tick 65,746 on Evidence: tick16c room.jsonl run-length segments (see computed); the last tick with any flag 1 is 65714. tick16 and tick16b give the same segments What is true: Every flag is 0 from tick 65,715 (warm-up) to the run's end. The first collapse is at 65,567, 31 ticks past the wrap, followed by two partial recoveries after overflow resyncs
- **held:** The room's own sets collapse past the wrap once the bitten clients' acks and commands stop matching Evidence: Consistent with the logs, not proven by them: the mirror (0 breaches) rebuilds the frames the room hands out. c3-r2-tick16c.log:19: commands 1,532 (vs 43,152 clean), steps without one 48,772, rewinds refused 37,820, overflow resyncs 140; lab.jsonl counter_wrap 'ack' at 65606; after 65715 nearly every datagram is exactly 998 B. The logs hold no baseline age per tick, so 'acks stop matching' is inferred
- **held:** Carry 73 expects 'the judge's outcome the mismatch figure FAIL with at least one mismatch, never INVALID' Evidence: u4s-carries.md:328-337 (carry 73); u4r-brief.md:581-582
- **held:** A judged tick16 bite run would be INVALID the same way while -17's L1 parts carry pre:PM-0001-2 Evidence: judge/src/eval.rs:1408-1409 'A bite run takes the validity of the parts it matches (the advisor's B7 ruling)'; marks.json -17 'L1 P0' validity [pre:PM-0001-2]; brief :1161: the judged tick16 run starts at 63,736 and crosses the wrap 1,800 ticks in. INVALID follows if the same collapse occurs (see computed)
- **held:** marks.json and the judge are the main session's (not U4r's) Evidence: u4r-brief.md:1006-1007 marks.json 'Not yours' (carry 62 (h)'s check is the main session's); :1010 'Never: ... crates/judge'
- **could not check:** Would the mismatch figure show mismatches if the run were valid Evidence: No judge run was allowed. Inputs exist: client.jsonl holds 1,018 tick records at ticks the room never logged (phase after, up to t = 4294968314), with 48,772 non-null hashes (= the summary's sentinels); at the room's ticks, 1,532 compared and 0 mismatched. Whether judge/src/figures.rs's `decoded` counts after-phase records was not traced
- **Computed:** tick16c: everyone_in_view summed over 48 slots: 48 through 65566; 0 for 65567-65604 (the wrap + 31); 48 for 65605-65636 (after 48 overflow resyncs at 65605); 0 for 65637-65682; 26-31 for 65683-65714 (after 46 + 2 resyncs at 65683/65685); 0 from 65715 to 66555 (the end). Measured 65806-66405: 0 on all 600 x 48. A 10-minute run that crosses the wrap 1,800 ticks in, with the same collapse, holds at most 1,500 of 18,000 measured ticks: share <= 0.083.

### Point 4

- **held:** b1's settings with --bite PM-0001-18/no-fallback: 96 heal records, 25 within 500 ms, median 1.197 s, max 1.688 s Evidence: c3-r2-bites.log:38-48 command line (B1, seed 1, 10 s + 30 s, 2 bursts, 5 s first, 15 s apart) and contract line '-18: 96 heal records (25 within 500 ms; ...; a full state during 71)'; r2/b1-nofallback/lab.jsonl burst_heal: recomputed (see computed)
- **held:** Full state during 71 bursts; overflow resyncs 71; sentinels 2,451; queue high-water 1,024; evictions 96 of 96 (baseline ages 0, 33, 34) Evidence: lab.jsonl burst_heal full_state 1 x71, 0 x25; queue_max_count max 1024 (= event_queue_cap_count); c3-r2-bites.log:38 overflow resyncs 71, sentinels 2451; room.jsonl burst_eviction 96, all evicted 1, ages 0 x14, 33 x67, 34 x15
- **held:** The clean b1 has 96 of 96 within 500 ms Evidence: r2/b1/lab.jsonl burst_heal: 96 records, 48 per burst, 48 distinct slots each, all <= 500 ms (min 48.3 ms, median 79.4 ms, max 129.6 ms), full_state 0 on all
- **held:** The bite fails -18's max check Evidence: Holds from the records only: marks.json -18 check 'every client decodes a fresh delta within 500 ms' = series event.burst_heal.after_link_ns, stat max, <= 500000000, parts [L1 B1, L2 B1 wt]; the bite's max is 1,688,286,323. The judge did not evaluate it (run INVALID, checks []). In a short run, -18's count check (>= 480 heal records) would also fail, on the clean b1 too (96)
- **held:** Brief 2.4 r2 expects 'no client holds a heal within 500 ms' on the bite Evidence: u4r-brief.md:589-592: '-18's heal record shown to bite: on a short B1 run with PM-0001-18/no-fallback no client holds a heal within 500 ms of a burst's link return, while the clean run holds one per client and burst (a record that took the sentinel 0 or a FullState's tick as a decode would pass the bitten run)'; also C29 :52-55
- **held:** The judge: -18 INVALID by pre:PM-0001-2 (as the clean b1) Evidence: b1-nofallback-verdict.json -18: bite PM-0001-18/no-fallback status 'invalid', invalid_why 'pre:PM-0001-2: everyone in view ... = 0.48', checks []. The value is 0.48, not the clean run's 0.273333
- **not held:** (Not in the notes :701-707) the bite run's mirror state Evidence: c3-r2-bites.log:38 summary '2314 breaches'; b1-nofallback/lab.jsonl class_audit breaches_count 2314, first_breach 'tick 64218: client 23: its baseline frame (age 35) is not mirrored'; mirror.rs:34 FRAMES_KEPT = ACK_RING_TICKS + 2 = 34, and mirror.rs:409-414 returns early on that breach What is true: The notes leave out that the mirror breached 2,314 times on this run. The everyone-in-view 0.48 behind the INVALID verdict comes from a mirror that could not rebuild every client's frame. The contract still printed PASS: contract.rs has no breach check
- **Computed:** No-fallback heal records: 96; 25 at or under 500 ms (48.3-135.8 ms); sorted values 48 and 49 (1-based) are 1,196,837,738 and 1,197,359,484, so the notes' median is the 49th (the upper median); max 1,688,286,323. Cross-tab: all 25 fast heals had full_state 0; all 71 with full_state 1 took over 500 ms. Burst 0: the 14 fast heals are exactly the 14 clients evicted with baseline_age 0. Burst 1: 11 fast, all at baseline_age 33 or 34 without a full state.

### Point 5

- **held:** In r6 the recording's last L - 1 ticks hold 48 takes of depth 0 at tick 119 (no later command exists) Evidence: .gate/u4r/c3-rul2b.log:155 and :159 (both legs: 'takes where it is not L - 1: 48, the first 64 (client, tick, measured) [(0, 119, 0), (1, 119, 0), ...]'), :166 and :168 ('first take {Some(1): 48}; after it {0: 48, 1: 5520}; not L - 1 only at the recording's last tick 119 (48 takes)'); :153 120 ticks from tick 0, lead 2
- **held:** Copy 2 had read these 0s as the first takes Evidence: u4r-build.md:406-407 (copy 2, c2-b10c-4.log): '{0: 48, 1: 5,568}: 1 = L - 1 at every take after each client's first command, 0 at each client's first take'; ruling 2 point 1, Builder (3) (advisor-u4r-build2-full.md:13) took the same reading
- **held:** Point 1 (4)'s words Evidence: advisor-u4r-build2-full.md:13 Builder (4): 'r6 asserts on both legs that every reading after each client's first take is L - 1, with no deviation.'
- **held:** Copy 3 reports every deviation as point 1 (2) says Evidence: c3-rul2b.log:155, :159, :289 (each leg line prints the count and the first 64 (client, tick, measured)); the late-command pair's L1 leg lists 50: (0, 59, 0), (0, 60, 0) and the 48 at 119 (:289, :329)
- **held:** r6 asserts the relation wherever it can hold Evidence: lab/src/precheck.rs:814-890: asserts the deviation count and the sorted first-64 list equal the computed end set (client, last - (back - 1), back - 1) for back in 1..lead, and that every other take is L - 1 (:860-865). The lead is hard-coded `let lead = 2u32` (:846), and the list compared is the first 64 only, which is enough at 48 clients. It is not point 1 (4)'s literal 'no deviation'
- **Computed:** r6, L = 2, 120 ticks (0-119): first takes {1: 48}; later takes 5,568 = {1: 5,520, 0: 48}; the 48 zeros are (client, 119, 0) for clients 0-47; 48 + 5,568 = 5,616 = 48 x 117 takes.

### Point 6

- **held:** npt-lab does not depend on npt-judge, and crates/lab/Cargo.toml is not U4r's lane Evidence: crates/lab/Cargo.toml [dependencies]: no npt-judge; u4r-brief.md:1005 'Not yours: crates/lab/Cargo.toml'
- **held:** `npt-lab contract` is a subcommand beyond 2.7's list Evidence: u4r-brief.md:679-683 allows the subcommands scenario and replay beside hash; lab/src/main.rs:285 dispatches contract (and :271 precheck)
- **held:** The checker covers r2 (b), (c) and (e), with carry 60 as a failure and the carries' figures as report lines Evidence: lab/src/contract.rs:111-232: (b) every written name has a SERIES.md row; (c) footers, one admit per slot with in_roster true, late_count on every room tick; carry 60's header params missing = failure (:140-147); (e) shot band SS [665, 735], ES [68.4, 75.6], applied only at 48 clients (:214); carries() :235-320 prints lines only. The module doc :16 says carry 60 is 'reported, never a failure ... a failure if missing': the wording contradicts itself, the code fails
- **held:** Exit 0 PASS, 1 FAIL with every failure by name, 2 usage Evidence: contract.rs:336-366. Also exit 1 when the SERIES file or the logs cannot be read (:350-351), not only on a FAIL
- **held:** U4R-Q11's four negative cases and a fifth in its unit tests, in check.mjs's count Evidence: contract.rs:384-460 test r2_the_checker_passes_a_clean_run_and_fails_each_doctored_copy_by_name: (1) an admit gone, (2) a footer gone, (3) the tick.s.snap_incomplete row gone, (4) one room tick's late_count gone, (5) shots halved; its run is SS P0 seed 2, 6 s + 2 s, 48 clients by default (:378; scenario.rs:1307); c3-r2-contract3.log '2 passed'; c3-reg.log:145 npt-lab bin 45 passed (43 + 2)
- **held:** The suites are run by hand through the release binary (scenario, contract, the real npt-judge --only) Evidence: c3-r2-suites.log and c3-r2-bites.log command lines: /home/user/Shamble-Frontlines/.gate/shared-target/release/npt-lab scenario ..., npt-lab contract --logs ... --series marks/SERIES.md, npt-judge --marks marks/marks.json --logs ... --only ... --out ...-verdict.json
- **held:** Why not one Rust test: npt-lab's bin tests already take 90.7 s in debug Evidence: .gate/u4r/c2-rul-reg.log:185 '43 passed; ... finished in 90.72s'
- **held:** Ruling 2's point 5 left this design to copy 3 Evidence: advisor-u4r-build2-full.md:54 'Not ruled here: copy 3's r2 design (contract.rs, a subcommand beyond 2.7's list), which is copy 3's part'; :56 Main session: 'Reads copy 3's contract.rs design in U4r's review, or asks me for a ruling before r2's runs count'

### Point 7

- **held:** (a) A bite run that reaches the after phase's cap ends there; a measured run still fails at the cap (carry 62 (e)) Evidence: lab/src/scenario.rs:1034-1049: at k >= measured_end + after_max_ticks, error 'the measured ticks were not all resolved within the after phase's N ticks (carry 62 (e))' if s.bites.id is None, else after_capped = true and stop. The condition is 'any run with --bite': role bite, or a twin given --bite (scenario.rs:1359 lets a twin take --bite), not tick16 only. Default after-max-s 30 (scenario.rs:1318)
- **held:** Summary.after_capped and run_lengths.after_capped written, summary line says so, SERIES.md row widened Evidence: scenario.rs:650 field; :1097-1103 run_lengths .with("after_capped"); :1468-1472 summary text; tick16b and tick16c lab.jsonl run_lengths {after_capped: true, after_ticks_count 150}; the SERIES.md row is in copy 3's own new subsection (git diff: 31 insertions, 0 deletions)
- **held:** The stop decides each client's unresolved ticks lost and the logs end clean Evidence: c3-r2-tick16b.log:46 and c3-r2-tick16c.log:19 'decided lost at the stop 34415'; tick16b and tick16c footers ended 'clean' on all three logs; first try (r2/tick16) logs end without footers and the client log stops at warm-up tick 65651
- **held:** (a) touches carry 62 (e) for bite runs Evidence: u4s-carries.md:300-301, (e): 'after the measured phase the clients keep logging until every measured tick is resolved, a tick still in flight at the stop is not written'; (c) :297-299 allows 'at the stop: on L1 the runner may decide it there'. Deciding lost at the stop already happens on clean runs (b1: 27, p2: 32, c3-r2-suites.log:20, :54)
- **held:** (b) The mirror keeps frames and looks baselines up by the room's tick, not the wire's 16-bit tick Evidence: mirror.rs:383-386 full states kept by `tick` (the room's); :400-407 baseline = c.frame(tick - baseline_age); the call passes the room's tick (scenario.rs:874); the tick16 header writer is tick & 0xffff (proto/src/quant.rs:133-140). The first breach before the fix: tick16b lab.jsonl class_audit first_breach 'tick 65536: client 0: its baseline frame (age 1) is not mirrored', 5,828 breaches; 0 after the fix (tick16c)
- **held:** Equal in a clean run; a clean P0 re-run equal figure for figure (notes :730-731) Evidence: See computed. Fate log be408a87... and audit 92,920 / 2,623,643 / 12,720 / 0 in both. Caveat: no clean run in copy 3 crosses the wrap (every suite ends by 64,968), so equality past 65,536 rests on the code (the clean header tick is the full tick: quant.rs:133-140), not on a run
- **held:** Second vs third try: every count as the second try Evidence: c3-r2-tick16b.log:46 vs c3-r2-tick16c.log:19: scenario counts equal. Carry 49's projectiles per set differs (12.65 -> 12.33), as do the audit counts (434,460/187,272/31,646/5,828 -> 459,038/264,878/33,677/0). The notes give 12.33 for the third try
- **Computed:** p0-s1 (21:59, before the fixes) vs p0-s1b (00:28, after): with the header's run id and build blanked, room 980/980, client 901/901 and lab 952/952 records are equal, except the one lab run_lengths record, which gains "after_capped": false. The summary text after the run name is identical (c3-r2-p0.log:20 vs c3-r2-tick16c.log:50). Both runs cover ticks 63,736-64,635 only: below the 16-bit wrap.

### Point 8

- **held:** SERIES.md: a subsection 'Names added with U4r', 21 rows (notes :636); no existing row changed Evidence: .gate/u4r-snap1 git diff marks/SERIES.md: 31 insertions, 0 deletions, 21 table rows
- **not held:** SERIES.md holds 22 rows in 'Names added with U4r' (notes :879) Evidence: git diff: 21 rows beginning '+| `' What is true: 21 rows
- **not held:** The baseline_age row: 'the age of the client's baseline at the first tick after the burst, -1 when it had none' Evidence: scenario.rs:914 writes c.baseline_age().map_or(-1, ...); room.rs:216-219 'The last tick's snapshot's baseline age (0: the per-type defaults); None when none was sent' What is true: 0 means coded against the defaults (no baseline: every clean b1 eviction logs 0); -1 means no snapshot was sent that tick. The row's '-1 when it had none' reads as 'no baseline'
- **held:** burst_spacing_ns has its row and U4r's runner does not write it; overflow_resync writes no name Evidence: SERIES.md:330 row present; b1 header params hold burst_starts_ns but no burst_spacing_ns; scenario.rs:899 overflow_resync .with("slot") only
- **held:** Suite table (:667-677) figures Evidence: c3-r2-suites.log:20, :37, :54, :73, :88, :103, :118, :133, :148 summaries and the r2/*-verdict.json checks match every cell checked (commands, late, steps without one, measured incomplete, shots, -17's 0 of 43,488 / 41,851 / 43,152 with 1,066 / 2,565 / 0 left incomplete, -57's maxima, -2's 11 pass on p2, p3, forged, clamp, fast-clock, lag-switch, evstart)
- **held:** Point 33's labels: each log folder's binary sha256 as listed (:838-846), the whole-as-lost hash and command lines (:850-857) Evidence: Header build.binary_sha256 checked in full for d13-1, d13-5, smoke1, relp2, r2/p0-s1, r2/p2, r2/es, r2/p0-s1b, r2/tick16, r2/b1-nofallback, r2/tick16b, r2/tick16c and whole-as-lost (all equal), by prefix for the rest; whole-as-lost.log:3 and :18 match the quoted lines. The label text follows advisor-u4r-build2-full.md:40. Not covered by the list: .gate/u4r/regen/ and regen-all/ (recordings and native .lines from the uncommitted tree; no headers)
- **held:** Per-profile bytes table (:738-746) and the 10-minute sizes Evidence: Recomputed from each folder's three logs (see computed); 18,300 = 300 + 18,000 ticks, as ruling 1 point 3 asks (advisor-u4r-build1-full.md:27)
- **held:** A run without -57 writes a lab log of events only: forged 16,877 B, evstart 16,748 B; room and client about 1,760,000 and 985,000 B for 900 ticks; about 3,050 B per measured tick, about 56 MB per 10-minute run Evidence: ls -l: r2/forged lab 16,877, room 1,762,761, client 986,907; r2/evstart lab 16,748, room 1,759,548, client 984,974 (r2/es lab 16,594, also events only); (1,760,000 + 985,000) / 900 = 3,050; x 18,300 = 55.8 MB
- **held:** .gate/u4r/r2/ holds 119 MB Evidence: du -sh .gate/u4r/r2: 119M
- **Computed:** Per-profile bytes recomputed (measured-phase line bytes / measured ticks): d13-5 1,953/1,096/24,768, sum 27,816.x (the notes' 27,817 is the sum of the rounded parts), 509.0-509.1 MB; d13-1 1,993/1,082/8,936 = 12,011, 219.8 MB; d13-4 1,995/1,062/3,818 = 6,875, 125.8 MB; r2/b1 2,006/1,039/7,371 = 10,416, 190.6 MB; r2/p0-s1 1,954/1,097/24,630 = 27,681, 506.6 MB; r2/p2 1,996/1,083/8,592 = 11,671, 213.6 MB; r2/p3 1,997/1,062/3,610 = 6,669, 122.0 MB. Every 10-minute figure = per-tick sum x 18,300.
````

### The advisor's fourth ruling on U4r's build: copy 5's points 1 to 3 and further findings (advisor-u4r-build4-full.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4r-build4-full.md` lines 1-32 (sha256 9d49b484cd3f), as they are:_

````text
# Advisor: the full fourth ruling on U4r's build, copy 5's points (workflow wf_f276e5ae-878), 2026-11-14

Written by script, word for word, from the workflow's journal (the session folder's
subagents/workflows/wf_f276e5ae-878/journal.jsonl): the advisor's StructuredOutput result (advisor-u4r-build4). Its own
notes: advisor-u4r-build4.md; the questions: u4r-build4-questions.md. Nothing below is the main session's.

## The rulings

### Point 1 (severity: Low; blocks the merge: no; to Mustafa: no)

- **Decision:** Option (a). The flag after_capped records how the run ended. It is not an expected result, and my ruling 3, point 3 was wrong to expect it true. I predicted that the post-wrap measured ticks stay unresolved until the cap, as they did in tick16b and tick16c. At the new setting the room's overflow resync comes first. I checked this myself in r2/tick16d. The flag is 48 from 64,652 to 65,566, 0 from 65,567 to 65,595, and 48 again at 65,596. All 48 overflow_resync events fall at 65,596 (wrap + 60). In client.jsonl, 65,536 to 65,595 are null on all 48 clients, 65,596 decodes to the server's hash, and 60 orphans per client are filed after every tick the room ran (2,880 = 48 x 60). Measured incomplete is 720 (48 x 15, the post-wrap measured ticks 65,536 to 65,550), and 1,488 (48 x 31) after-phase client-ticks were decided lost at the stop. The split fits the full states pushing the post-wrap ticks out of each client's ring. So written_through(last measured) held, and the run took the normal stop (scenario.rs:1033-1037) 46 ticks into the after phase, before the cap branch (:1038-1052). Nothing downstream reads after_capped: only SERIES.md's row, the scenario summary and c-u4r-r2.mjs's text name it (grep); neither the judge nor contract.rs reads it. Every load-bearing item holds. The run is valid (pre:PM-0001-2's share is 1.0 on 600 of 600 measured ticks). The PM-0001-17/tick16 bite is shown by decoded_hash_mismatch_count 2,880 (needs == 0; at least 48 expected). The mirror has 0 breaches and contract PASSes. So carry 73's outcome ('the mismatch figure FAIL with at least one mismatch, never INVALID') is met at ruling 3's setting. The cap path of ruling 3, point 7 (a) is already shown by tick16b and tick16c (after_capped true, 150 ticks, clean footers). The clean partner matches every expected item (I verified its verdict file: -2 passes 11 of 11, -17 0 mismatches, -57 6,981 / 2,750 / 88,044, 0 after-phase ticks), so it shows point 7 (b) across the wrap. Option (b) is rejected: picking a setting so that a path flag comes out as predicted would tune a run to my guess, not to the evidence. Nothing is re-run, and condition 5 holds. Word changes, all the advisor's: ruling 3, point 3's expected 'after_capped is true' now reads 'after_capped as it comes: false when the room's overflow resync resolves the measured ticks before the cap (carry 62 (e)'s normal path), true when the cap path of point 7 (a) ends the run; either way the run is valid and -17 FAILs by the mismatch figure'. Carry 73 gains a reading (below). No bar, mark, brief word or plan line moves. Ruling 3, point 3's merge block (carry 73 shown at the new setting before U4r's report) is met as builder evidence. The record is still the merge-commit run of c-u4r-r2.mjs.
- **Builder:** Point 1 (P3-CAP), the advisor's ruling 4: option (a). Keep .gate/u4r/r2/tick16d/, tick16d-verdict.json, tick16-clean/ and its verdict as they are. Do not re-run anything and do not try a new setting. (1) In ops/runs/c-u4r-r2.mjs, suite tick16's expect text: replace 'after_capped true (ruling 3, point 3; carry 73)' with 'after_capped as it comes (false when the room's overflow resync resolves the measured ticks before the cap, true when the cap path of ruling 3, point 7 (a) ends the run); carry 62 (h)'s measured sum is the bite's own (tick16d: 720 = 48 x 15 post-wrap measured ticks) (ruling 3, point 3, as ruling 4, point 1 reads it; carry 73)'. (2) In the report's r2 section, give the tick16 bite run at ruling 3's setting: ticks_below_wrap 885, the wrap at measured tick 585; run valid (PM-0001-2's everyone-in-view share 1.0, 600 of 600 measured ticks); -17's bite shown by decoded_hash_mismatch_count 2,880 (60 orphans per client); 0 mirror breaches; contract PASS. Then give after_capped false with its cause in one sentence: at 65,596 (wrap + 60) the room's 48 overflow resyncs sent full states, after which every measured tick was resolved (the 15 post-wrap measured ticks lost: measured incomplete 720), so the run ended by carry 62 (e)'s normal path 46 ticks into the after phase, below the 150-tick cap; 1,488 after-phase client-ticks (48 x 31) were decided lost at the stop. Give the flag segments: 48 from 64,652 to 65,566, 0 from 65,567 to 65,595, 48 at 65,596. Give the clean partner: 900 ticks, 0 after; PM-0001-2 11 of 11 pass; -17 0 mismatches; -57 6,981, 2,750, 88,044; 0 breaches; it shows ruling 3, point 7 (b) across the wrap. Say that the cap path is shown by tick16b and tick16c (after_capped true, 150 after-phase ticks). (3) Then go on with r5 and section 5's end checks, after point 2's one assertion.
- **Main session:** 1. Send the builder text. 2. On the card, carry 73 gains this reading: 'its "the post-wrap ticks the room did log stay unresolved for that client" holds until the room's overflow resync sends that client a full state; the client then resolves them lost (r2/tick16d: the 48 resyncs at wrap + 60, measured incomplete 720 = 48 x 15). A bite run's after_capped records its path, true or false (the advisor's U4r build ruling 4, point 1).' 3. Ruling 3, point 3's expected outcome reads as this ruling says. Note it beside the ruling's pointer in U4r's brief 2.4 r2 (the carry 73 line); no brief word changes. 4. For 7.1's judged tick16 bite run (tick_start 47,251): the same reading applies, and carry 58 still holds. Its overflow resyncs come to the advisor with the P0 set's verdict before -17's L1 P0 counts. Expected: about 48, at about wrap + 60, all in the after phase, which are the bite's. A resync before the wrap or inside the measured phase is a deviation. 5. In U4r's review, read tick16d's logs and c-u4r-r2.mjs's amended expect text. 6. The tick16 run's 720 is the bite's own loss, not the cause carry 62 (h)'s else branch asks about. The decision on the check follows the further finding below.
- **Why:** Read myself: c5-p3.log; tick16d room.jsonl (flag segments, phases 300/600/46), lab.jsonl (48 resyncs at 65,596; run_lengths after_capped false, 46) and client.jsonl (65,536-65,595 null; the orphans); tick16d-verdict.json (shown, valid, 2,880); tick16-clean-verdict.json; tick16b and tick16c's resyncs (65,605 on) and run_lengths (true, 150); scenario.rs:1033-1052; logs.rs:8-16; a grep for every reader of after_capped. A mis-predicted path flag that no check reads, while every judged item matches, is my wording error. It is not a defect, and it is not a result to chase with another setting. The words that change are mine (ruling 3, point 3; the reading on carry 73, whose text is the advisor's U4e round-1 ruling, R1). None is Mustafa's, and no bar moves. Risk tier 1.

### Point 2 (severity: none; blocks the merge: no; to Mustafa: no)

- **Decision:** Keep copy 5's run-time lookup. Do not spell the id. Brief section 4 (:1023-1026) says 'npt-lab's sources never spell another crate's bite id as a string literal'. That covers every id, not only the PM- form that check.mjs's scan reads (BITE_ID, check.mjs:887). A literal `catalogue-5/owner-to-all` outside a test module would break the rule even though the scan would not flag it. The lookup (contract.rs:248-256: in npt_room::BITES and outside the PM- form) matches room/src/lib.rs:32-46, which documents that one id as the only one outside the form. The spelled ids at contract.rs:569 and scenario.rs:1852-1856 are in test modules, which may name ids. A measured-build contract never applies the exemption, and that is harmless and fails closed. A measured build cannot make an owner-to-all run (C4; --bite there is a usage error). Both run scripts run each suite's contract with that suite's own build (c-u4r-r2.mjs:129-130). r4's owner-to-all runs are unit tests that no script runs contract over (scenario.rs:1811-1860). A measured-build contract handed such logs fails them and names the bite. One weakness: a second id outside the PM- form added to npt_room::BITES would widen the exemption without anyone seeing. That is a Low guard, riding the current work. My ruling 3, point 6 (1) words now read: 'unless its header's bite is npt-room's class-filter bite (catalogue-5/owner-to-all), exempted by a bites-build contract'.
- **Builder:** Point 2 (CR-ID), the advisor's ruling 4: keep the run-time lookup (contract.rs:248-256), and spell no other crate's bite id in npt-lab's non-test sources (brief section 4's rule covers every id, not only the PM- form). One Low, riding your current work: in contract.rs's bites-build test block (:563-575), add one assertion that exactly one id of npt_room::BITES lies outside the PM- form, so a second such id cannot widen the exemption unseen. Section 5's end checks cover it (fmt, clippy in both builds, npt-lab's tests in both builds; the bites bin count rises only if you add a test rather than an assertion: say which). In the report, state the choice in two lines. (1) The exemption is applied only by a bites-build contract, the one build that can make that run; both run scripts run each suite's contract with its own build, and a measured-build contract fails an owner-to-all run's breaches and names the bite. (2) r4's owner-to-all runs are unit tests, and no script runs contract over them.
- **Main session:** 1. Send the builder text. 2. On the card, ruling 3, point 6 (1)'s exception reads as this ruling words it, with a pointer. 3. In U4r's review, read contract.rs:248-301 and the new assertion.
- **Why:** Read myself: contract.rs:1-30, :248-301 and :540-575; room/src/lib.rs:32-46; brief :370-380 and :1023-1026; check.mjs:887-956 (the scan matches only PM- literals outside test modules); scenario.rs:1811-1860 and :1525 (the test module). Spelling the id would satisfy the scan but break the brief's rule. The lookup meets the rule, and its only gap, the measured build, cannot occur on any scripted path and fails closed. 'A Low never triggers a push' (Mustafa, history.md:340), so the assertion rides the current work. Not Mustafa's: the words amended are mine. Risk tier 1.

### Point 3 (severity: none; blocks the merge: no; to Mustafa: no)

- **Decision:** Both items are ruling 3's own, as the main session read them. (1) Ruling 3, point 3 (advisor-u4r-build3-full.md:27, main-session item 4 at :29): the judged tick16 bite run's tick_start = 65,551 - (warm-up ticks + measured ticks), which is 47,251 for 300 + 18,000. That puts the wrap at measured tick 17,985, 15 before the measured end (ticks_below_wrap 18,285). Every other -17 run, clean and the client-rounding bite run, stays at 63,736 (U4R-Q7 (a) as ruling 3 reads it). (2) Ruling 3, point 1 (:13; main-session item 2 at :15): -18's bitten judged run (PM-0001-18/no-fallback) moves to U4g. The three B1 measured runs stay in U4r's set for -57's L1 B1 part, and their -17 and -18 verdicts are recorded INVALID and not counted. The snapshot (wip/u4r.patch, 707cc3df...) still had 63,736 for every run and the no-fallback run on B1. Copy 5's live c-u4r-runs.mjs now matches both items: :20-27 (header), :60-65 (TICK16_START = 65,536 + 15 - (10 + 600) x 30 = 47,251), :68-69 (NOT_AT_U4R), :187-193 (no bite on B1; the tick16 run at TICK16_START, the rest at 63,736). Only the B1 set's -18 bite now reads NOT-RUN ('no run yet'), which is what moving it to U4g means. c-u4r-r2.mjs's short b1-nofallback suite rightly stays: it is r2's evidence (ruling 3, points 4, 6 (4) and 8 (A)), not the judged run. No word of the brief, plan, marks or carries changes beyond ruling 3's own main-session items.
- **Builder:** Point 3, the advisor's ruling 4: both script items are ruling 3's own, as sent. (1) The judged tick16 bite run's tick_start is 65,551 - (warm-up + measured ticks), 47,251 for 300 + 18,000 (the wrap at measured tick 17,985, 15 before the measured end), and every other -17 run, clean and client-rounding, stays at 63,736. (2) No -18 bite run in the B1 set (no-fallback's judged run is U4g's). The three B1 measured runs stay for -57's L1 B1 part, with their -17 and -18 verdicts recorded INVALID and not counted. Your current c-u4r-runs.mjs lines (:20-27, :60-69, :187-193) match both: change nothing more there. Keep c-u4r-r2.mjs's short b1-nofallback suite (r2's evidence, ruling 3, point 8 (A)). The report names both changes with their source (ruling 3, points 3 and 1).
- **Main session:** 1. Send the builder text (it confirms what was sent). 2. In U4r's review, read c-u4r-runs.mjs's live lines against ruling 3, points 1 and 3, not the snapshot's. 3. Ruling 3's own main-session items for the brief (7.1's -17 L1 P0 row; 2.4 r2 (d) and 7.1's -17 and -18 L1 B1 rows) and the plan's next edit (§5's U4r judged-runs line, plan :767-768, U4g's 'Marks moved' line) stay as ruling 3 set them, with nothing added.
- **Why:** Read myself: ruling 3's points 1 and 3 (Decision and main-session items); the snapshot's c-u4r-runs.mjs hunk; the live file whole (216 lines); the arithmetic (65,551 - 18,300 = 47,251; 65,536 - 47,251 = 18,285; 18,285 - 300 = 17,985). The marks.json setup check on tick_start (63,736) covers only the 'every S30' part, so the new tick16 start meets no setup check. These are my own ruling's items, carried out as written. Risk tier 1. Not Mustafa's.

### Further findings

- **Medium:** Carry 62 (h)'s planned setup check would make -17's L1 P0 part INVALID twice over. When the main session decides it at 7.1, it is to add {"name": "setup: L1 P0 loses no group (no client-tick left incomplete)", ..., "parts": ["L1 P0"], "setup": true} with the bite entry PM-0001-17/whole-as-lost (U4R-Q2 (c); ruling 1, point 4: 'the check's JSON stays word for word'). The judge makes a run INVALID when a setup check fails (eval.rs:1189-1208), and it marks the mark INVALID when a bite is INVALID (eval.rs:1476, :1660). (1) Any tick16 bite run that crosses the wrap inside its measured phase leaves post-wrap measured ticks lost (tick16d: measured incomplete 720 = 48 x 15). So the setup check would make the tick16 bite run INVALID, breaking carry 73's 'never INVALID', and -17's L1 P0 could never count, since bites come first. (2) whole-as-lost as a bite entry could never be Shown either: its run would be INVALID by the same setup check, and an expect-invalid bite may name only a validity rule of the mark or its parts (marks.rs:862-876), never a setup check. c-u4r-runs.mjs runs every listed -17 bite on P0 from marks.json at run time, so the judged P0 set would read INVALID. The cause the carry's else branch asks for, for tick16d's 720, is the bite's own: given here. The clean short L1 P0 sums are 0 (p0, forged, clamp, fast-clock, evstart, tick16-clean, p0-s1c). **Fix:** My decision, applied by the main session at 7.1. The check goes into marks.json as a plain check of -17's L1 P0 part: the same series, stat, window, op, value and parts, without "setup": true and without the 'setup: ' prefix, for example 'L1 P0 loses no group (no client-tick left incomplete)'. It is stricter on clean runs (a lost group on P0 FAILs -17 rather than making the run INVALID). Both bites are then Shown: tick16 by the mismatch figure and by this check (720), whole-as-lost by this check (48). It is still added only after the record run of c-u4r-r2.mjs shows 0 on every clean short L1 P0 run. A bite run's sum is reported with its cause and is never that condition. Before -17's judged runs, the main session runs `npt-judge --only PM-0001-17` on .gate/u4r/whole-as-lost/ and on the record run's tick16 folder, and shows both bites Shown, never INVALID. The words that change are the advisor's: carry 62 (h)'s JSON (the U4j ruling, R2.4) and ruling 1, point 4's 'JSON word for word' and 'setup check FAIL by name'. Neither is Mustafa's. The check is a stricter reading of -17, so the main session adds it to plan §11's readings-PR list (u4-build-plan.md:1081-1093) for his go before R5. It does not block U4r's merge (marks.json is the main session's lane, after the merge). Not for Mustafa now.
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `^(14|28|40|49|57|58|59|60|62|63|69|73|74|75|76|77|78|82|83|84|85|87|89)\. \*\*|U4r`._

### Carry 14
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 116-119 (sha256 c74e52553bd8), as they are:_

````text
14. **rust 3 (the 1 MB half):** "`HitHistory::for_ms` also returns `None` for a span whose ring would exceed 1 MB
    (capacity × one tick's `HitVolumes` bytes: u4-build-plan.md §4.9), tested at the edge, once `HitVolumes` holds its
    volumes. The recording header carries the ring's span in ms beside `TICK_HZ` (nit b's capacity), and the replayer
    rebuilds the ring with `HitHistory::for_ms(span_ms)`."
````

### Carry 57
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 223-228 (sha256 c74e52553bd8), as they are:_

````text
57. **U4E-Q6, the room's half (advisor-u4-wave1.md:473-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "The event queue's cap is data, with a provisional value from net F121's arithmetic (1,024 is
    F121's example, above the about 400 a B1 burst adds), recorded as provisional wherever the run's header records
    the room's settings; U4f measures the rate and records §4.7's value before -22's runs. The room counts each
    overflow resync per client, with its tick, in its log, so that every judged -17 and -18 run can report them (carry
    58)."
````

### Carry 61
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 229-247 (sha256 c74e52553bd8), as they are:_

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

### Carry 40
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 256-264 (sha256 c74e52553bd8), as they are:_

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

### Carry 49
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 265-268 (sha256 c74e52553bd8), as they are:_

````text
49. **N-M4 and TUNE-2 (advisor-u4-wave1.md:291-294; the advisor's W1-U4D ruling):** "The contract test's report gives,
    for each SS suite, the share of snapshots whose client set holds all 4 very large units, and the mean number of
    projectiles per client set (reported, not judged: W1-U4D N-M4). The runner refuses a scripted spawn or teleport
    above World's stated clamp bound (W1-U4D TUNE-2)."
````

### Carry 58
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 269-273 (sha256 c74e52553bd8), as they are:_

````text
58. **U4E-Q6's two conditions (advisor-u4-wave1.md:476-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "U4r's contract test logs each client's relevant gameplay-event rate and its event queue's
    high-water mark per burst. An overflow resync in any judged -17 or -18 run (U4r's, or a later unit's) is reported,
    and those runs come to the advisor before they count: net F121 warns that a cap set too low resyncs every client
    on every burst and changes -18's heal path."
````

### Carry 59
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 274-283 (sha256 c74e52553bd8), as they are:_

````text
59. **A2-L3 and T2-M1, ES under the bots (u4d-round2-arch.md, u4d-round2-test.md; the advisor's U4d close ruling):**
    "U4r's contract test also runs a short ES suite of the brawl `Behaviour` (48 clients) and resolves PM-0001-2's ES
    checks on it: shots, gameplay events, live projectiles and spawns in their bands (W1-U4D T-M1's numbers) and the
    share of snapshots whose client set holds all 47 other soldiers and all 15 vehicles. A band or share missed goes
    to the advisor before any ES run of PM-0001-12 counts. The ES runs themselves stay under PM-0001-2's counter
    script, run by run (ADR 0001:1480). Since U4d-F all 15 of ES's hulls loop inside the fight (the advisor's U4d close
    ruling, B1.3), where they are cover and block moves: on the synthetic recording, at its old reckoning and share,
    they cut ES's gameplay events from 9.853 to 7.250 a second over 300 s (U4d-F's step 2). The brawl meets the same
    hulls. U4r's report gives that figure beside its ES suite's event rate, and a missed event band is not read as a
    bot defect before the advisor has seen both."
````

### Carry 60
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 284-288 (sha256 c74e52553bd8), as they are:_

````text
60. **TUNE-1's header (advisor-u4-wave1.md:252-253; the advisor's W1-U4D ruling; homed by the advisor's U4d close
    ruling):** "Each run's header records the scene's weapon (interval, damage, reach, magazine, reload, spread) and
    its soldiers' health, read from `Scene::weapon()` and `Scene::health()`. U4r's runner writes them into npt-lab's
    L1 headers, and the contract test shows them in every header. U6's npt-server header does the same (plan.md's U6
    row)."
````

### Carry 62
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 289-316 (sha256 c74e52553bd8), as they are:_

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
    client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "measured", "op": "==",
    "value": 0, "parts": ["L1 P0"], "setup": true}`. If it is not 0, the cause goes to the advisor first. (Both the
    reported sum and the check read the measured phase: each join's ticks between its full state and that full state's
    acknowledgement carry no snapshot, so the client resolves them lost by design in the warm-up: the advisor's U4e
    round-1 ruling, R5, NC-L10.) marks/ is not U4r's lane unless its brief gives it this one check; else the main
    session adds it in its own commit."
    **(h) amended (the advisor's U4r build ruling, point 4; advisor-u4r-build1-full.md):** the bite's wording now reads
    "(a client writer that files a tick it decoded whole as lost: null and incomplete 1)", its id
    PM-0001-17/whole-as-lost; the check's JSON stays word for word. U4r's brief 2.5 row (:643) reads this way.
````

### Carry 63
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 317-322 (sha256 c74e52553bd8), as they are:_

````text
63. **FU-3 and FU-7's log half, the next change to npt-log (the advisor's U4j ruling, FU-3: SEC-F10; its round-2
    ruling, B3: SEC-R2-N2):** "The next change to npt-log (U4r's, if its brief takes crates/log; else the unit that
    next edits it) adds two refusals, each with a test that fails before it: `read_log` refuses a JSON object with a
    duplicate key (today the last one wins, silently: FU-3); `check_header` refuses any header text field holding a
    character for which Rust's `char::is_control` holds (today it refuses only an empty `run`, `set`, `role` or
    `process`: log/src/lib.rs:305-321; FU-7). Until then both stay open security findings in PR #40's report."
````

### Carry 69
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 323-327 (sha256 c74e52553bd8), as they are:_

````text
69. **NF-L1's U4r half (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "U4r takes no ES byte or bandwidth
    figure from `Recording::synthetic`'s ES recording as ES's load: its judged ES runs are the bots' (carry 59). Any ES
    byte figure it reports from that recording states the recording's churn beside R2's moving-delta assumption: yaw
    changing on about 48 % of alive soldier-ticks and pitch on about 18 % (U4d-F's scene), against yaw in every moving
    update and pitch half the time (docs/research/r2-netcode.md:1751)."
````

### Carry 73
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 328-337 (sha256 c74e52553bd8), as they are:_

````text
73. **NC-H1 and TE-1, tick16's judged half (u4e-review-netcode.md, Block h; u4e-review-test.md, TE-1 and probe Q-1;
    the advisor's U4e round-1 ruling, R1):** "Since U4e, under PM-0001-17/tick16 and PM-0001-58/tick16 the client
    files the sentinel for each post-wrap group under the tick its 16-bit header names (0, 1, 2 …): ticks before the
    client's first snapshot, which the room never logged for that slot (marks/README.md §1's bad-header sentence), so
    `decoded_hash_mismatch_count` counts each one, while the post-wrap ticks the room did log stay unresolved for that
    client. U4r's client writer places those hashes in its log (README §1: it files every hash under its rebuilt tick,
    buffering and sorting before it writes, so that records stay in tick order) and never ends the log `crash` for
    them. Its contract test runs one short tick16 bite run past the wrap (-58/tick16's `ticks_below_wrap` = 30 keeps it
    short) and shows the judge's outcome: the mismatch figure FAIL with at least one mismatch, never INVALID (the judge
    returns a mismatch count before it reads a coverage gap: judge/src/figures.rs:542-554)."
````

### Carry 74
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 338-349 (sha256 c74e52553bd8), as they are:_

````text
74. **NC-M6, the endpoint's lead (u4e-review-netcode.md, Block h; the advisor's U4e round-1 ruling, R3):** "U4e's
    `ClientEndpoint` stamps each command `newest + lead`, with a lead of 1 by default (crates/client/src/lib.rs), and
    U4f's lead controller comes after U4r's runs. On a link whose one-way delay is above about half a tick period every
    command would reach the room after its tick and be dropped as late, `last_run` would stay `None`, and option B
    would have nothing to repeat: -17 would compare a world of soldiers standing still. U4r sets each endpoint's lead
    with `ClientEndpoint::with_lead` from the run's profile (the round trip plus the jitter, in ticks rounded up, at
    least 1, plus 1: `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`, so P0 2, P1 3, P2 and B1 5, P3 7, within
    `CMD_CATCHUP` = 8; its bots give a command only at a client step whose newest snapshot tick moved since their last
    command, and move each press counter by at most one edge per command; the advisor's U4r brief check, U4R-Q14),
    records it with `cmd_timing` in the header, and its contract test reports on each short run the commands that
    arrived late (the room's `late` count) beside the commands sent; a run with more than 1 % of its commands late comes
    to the advisor before its marks count."
````

### Carry 75
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 350-358 (sha256 c74e52553bd8), as they are:_

````text
75. **NC-M5 and A3, the client's event-number base (u4e-review-netcode.md, Block h; u4e-review-arch.md, A3; the test
    seat's probe Q-10; the advisor's U4e round-1 ruling, R3):** "U4e's client rebuilds each event's 16-bit number near
    its own start (`ClientEndpoint::with_starts`, 0 by default), and the wire carries no event base (`JoinAccepted` and
    `FullState` have none), so a client whose start is 32,768 or more from the room's counter rebuilds every event
    number wrong, and the hash reads them (Q-10). U4r's lab gives each endpoint, through `with_starts`, the event start
    it gave the room (`Room::with_event_start`) and its command start, and admits every client before the room has
    stamped 32,768 events past that start (about 30 s of SS's events); a later admission comes to the advisor first
    (the room exposes no event counter). The contract test shows `decoded_hash_mismatch_count` 0 on a run whose event
    start is far from 0 (-58/lt-event-seq's, 3 below the wrap, among them). The wire field is U6's (C-U6-13)."
````

### Carry 76
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 359-366 (sha256 c74e52553bd8), as they are:_

````text
76. **A6 and SEC-U4E-5, ending a session (u4e-review-arch.md, A6; u4e-review-security.md, SEC-U4E-5; the advisor's U4e
    round-1 ruling, R3):** "npt-room decides a replacement and a `Kick(overflow)` but sends the old connection nothing,
    npt-server's `Decoder` has no call that ends a session, and `Room::receive` takes any message by slot alone, so the
    caller's session handling is a security boundary. U4r's lab, when `Room::join` reports a replacement, sends the old
    session `Kick(replaced)` and drops its `Decoder` and link before it routes anything else; when the room kicks a
    client or takes its `Leave`, it drops that session. Two tests: a second endpoint joining with slot 0's token and
    bind while the first is live, after which the first's Inputs and `Leave` reach no room call and the second keeps
    the slot; and a kicked session whose later datagrams reach no room call."
````

### Carry 77
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 367-376 (sha256 c74e52553bd8), as they are:_

````text
77. **U4r's readings of U4e's room and client (u4e-review-netcode.md, NC-L2, NC-L4, NC-L9 and NC-L10; the advisor's U4e
    round-1 ruling, R4 and R5):** "(a) -58's wrap check counts `event.counter_wrap` records (marks.json -58), and the
    room and the client report wraps per client slot (`ClientSlot::wraps`, `StepOut::wraps`): U4r writes one record per
    counter name that wrapped (SERIES.md's row: 'per counter that wrapped'), never one per client, so one counter's 48
    per-client wraps cannot pass the check. (b) Under PM-0001-18/no-fallback, a burst whose last acknowledged baseline
    is exactly 32 ticks old at the first tick after it logs `event.burst_eviction` 1, yet the bitten room codes that
    tick from the defaults (its stale copy is taken only once B's ring slot is overwritten) and heals: U4r's report
    counts the bitten runs' bursts at that age apart. (c) The contract test reports each client's
    `ClientEndpoint::stale()` sum on each short L1 run (groups of ticks already resolved when they left the frame ring;
    no SERIES.md row); a sum above 0 on L1 comes to the advisor."
````

### Carry 82
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 377-386 (sha256 c74e52553bd8), as they are:_

````text
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** "U4r's runner recordings join
    crates/lab/recordings.json" now reads "join crates/lab/det-recordings.json, in recordings.json's schema"; the rule
    (kind key, dispatch, name and ticks checks) is unchanged.
````

### Carry 83
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 387-401 (sha256 c74e52553bd8), as they are:_

````text
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5; with EA2-3, RR2-L3 and TE2-R2-2: the
    advisor's U4d2 round-2 ruling, Block 2):** "(a) For -30's pre-check, U4r gives `HashCompare::of` two lists that
    name the same recording on both legs and line up tick for tick (it compares by position, names included), and says
    where its printed comparison line comes from (`compare_text` is private to npt-lab's binary); TickHash's struct doc
    then says what its field doc says, 'the recording both legs replay' (today it still says 'the recording (or run) it
    replays'). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also requires
    exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines alone,
    which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that the bite
    causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg is the
    committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's first
    recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside the
    hash-map pair's equal prefix). (d) One npt-lab bin test runs `hash compare` with a bite and no `--on` over two
    recordings and asserts that only the first listed is armed (EA-9's one default, the legs'; today no test pins it:
    compare arming every recording again leaves every test and the step green, r2-te-rr2-l3-compare-arm.log)."
````

### Carry 84
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 402-416 (sha256 c74e52553bd8), as they are:_

````text
84. **INF2-1 with INF2-2 and TE2-R2-1, the wasm step's time bound and a stopped call's processes
    (u4d2-r2-review-infra.md, INF2-1 and INF2-2; u4d2-r2-review-test.md, TE2-R2-1; the advisor's U4d2 round-2 ruling,
    Block 1):** "At U4r's change to check.mjs's `wasm` step (entries 82 and 83), or in ops/runs/det-u4.mjs if U4r's
    Evidence decision moves the step's runs there: (a) the step's calls other than its cargo builds share one deadline,
    WASM_CALL_LIMIT_MS (300 s) from the first of them: each is given what is left, and once one is stopped the rest are
    not started, each named as skipped (today each call has its own 300 s, so a hang in npt-sim's replay makes 12 calls
    each wait out 300 s, 60 min under the exclusive lane lock: r2-inf-hang.log). (b) A call stopped by the deadline
    takes every process it started with it (today spawnSync's timeout signals only the direct child, so `npt-lab
    hash`'s two leg processes can keep running after their parent is stopped, unseen by the runners' idle wait, which
    looks for cargo and rustc only), by a documented means (its own process group, killed whole, or another) or by one
    whose effect (c)'s second test proves. (c) Two self-tests: the shared deadline, with a fake runner and clock (a
    first call that takes the whole limit; the next calls skipped and named); and run() itself on real processes (a
    child that starts a grandchild and sleeps, with a limit of about 200 ms: run() returns `timedOut`, and neither
    process is left), which also pins run()'s timeout wiring (today dropping it leaves 71 of 71 green:
    r2-te-inf2-2-run-timeout.log)."
````

### Carry 85
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 417-440 (sha256 c74e52553bd8), as they are:_

````text
85. **The deps step's unread inputs and two overclaims (u4d2-r2-review-security.md, SEC-U4D2-R2-1 to -3;
    u4d2-r2-review-rust.md, RR2-L1, RR2-L2 and RR2-L4 (a); u4d2-r2-review-test.md, TE2-R2-3; the advisor's U4d2 round-2
    ruling, Block 2):** "At U4r's change to check.mjs (entries 82 to 84), the deps step, each refusal naming its source
    and value, with one self-test case each, and the container's environment and the spike's own config still passing:
    (a) refuses a linker set for any target, by an environment variable whose name ends in _LINKER or by a `linker` key
    in any table of a cargo config read, as COMPILER_ENV and COMPILER_KEYS refuse a compiler (today
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER passes: r2-rr-superset.log run 3); and DEPS_CHECKED's limits and
    crates/hash-wasm/loader.mjs's header (the one edit U4r makes in crates/hash-wasm) say what the target_features read
    gives: a feature enabled at compile time shows in the section, which the loader refuses, but the section is the
    linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can rewrite it (built:
    sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check (today both say a flag from a
    route check.mjs does not read is refused too). (b) It refuses any `profile` key in a cargo config read and any
    environment variable whose name starts with CARGO_PROFILE_ (today a config's `[profile.release.package.npt-sim]
    overflow-checks = false` passes and drops npt-sim's `-C overflow-checks=on`, which plan §4.5 asks for in every
    profile: r2-sec-fx1-config.log, r2-sec-routes.log). (c) In npt-tcpinfo and npt-hash-wasm it refuses `include` other
    than directly before `!` and an opening bracket, and in every crate an attribute whose text starts with a
    metavariable (`#[$` or `#![$`; none today), so that neither `m!(include!)` nor a #[path] built from a `meta`
    fragment brings an unread file under a listed export's allow (today both pass, and deps' held line says every
    include! literal names a .rs file: r2-te-sec-r2-3-macro-include.log, r2-sec-pathmeta.log). (d) It reads an empty
    CARGO_HOME as unset and a relative one against the folder cargo runs in, as cargo does (today an empty one hides
    ~/.cargo/config.toml from FX-1's refusal and from the read-back: r2-rr-cargohome.log). (e) Its text beside
    buildInputProblems says which source cargo uses: CARGO_ENCODED_RUSTFLAGS if set, even empty; else RUSTFLAGS if set,
    even empty; else the matching target tables' flags, if any; else build.rustflags (today 'the first non-empty
    one')."
````

### Carry 86
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 530-532 (sha256 c74e52553bd8), as they are:_

````text
86. **U4R-Q4, condition 5's option-A half (the advisor's U4r brief check):** U4f extends U4r's contract test (r2) to
    -21's option-A suites and -20's P3 option A, with every PM-0001-2 check passing, not only resolving, and the shot
    band, before -20's and -21's marks count; a suite that fails comes to the advisor first.
````

### Carry 78
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 546-553 (sha256 c74e52553bd8), as they are:_

````text
78. **NC-M4, the nearest 8 before the events (u4e-review-netcode.md, Block b; the test seat's probe Q-11; the advisor's
    U4e round-1 ruling, R3):** "U4e's encoder packs a snapshot's events before any entity change (crates/room/src/
    room.rs, `snapshot`), so an event backlog larger than a snapshot's remainder sends snapshots with no entity change,
    against `Priority`'s rule 'the nearest 8 enemies in every snapshot' (catalogue :129-130; ADR 0001 part 9 (d); the
    U4e brief, 2.1). After U4e's gap of `ACK_RING_TICKS` tick periods the backlog went out at about 23 events a
    snapshot and every snapshot still carried entity changes (Q-11), so no U4e or U4r run meets it; a backlog near the
    event cap would. U4g packs the nearest 8 enemies' changes first, then the events, then the rest, with a test in
    which a backlog larger than one snapshot's remainder still sends the nearest 8's changes in every snapshot."
````

### Carry 28
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 561-562 (sha256 c74e52553bd8), as they are:_

````text
28. **netcode F6 / rust 3:** "The ring's span is U4h's test cap plus its margin in ms, recorded; the room builds it with
    `HitHistory::for_ms(span_ms)` (no tick count is ever written for it), and the recording header carries the span."
````

### Carry 87
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 572-578 (sha256 c74e52553bd8), as they are:_

````text
87. **U4R-Q9, the TC B1 seed 3 recording (the advisor's U4r brief check):** U4h records TC B1 seed 3 with `scenario
    --record` and its TC shooter, adds it to crates/lab/recordings.json under carry 82's schema, and states the `wasm`
    step's time with it against the 5-minute budget.
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** from "adds it to" on, it now
    reads "adds it to crates/lab/det-recordings.json under carry 82's schema, and states det-u4.mjs's time with it, its
    non-cargo calls against carry 84's shared 300 s deadline (over 240 s it comes to the advisor, RR-L4's remedy
    weighed first)".
````

### Carry 88
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 579-586 (sha256 c74e52553bd8), as they are:_

````text
88. **O1's re-run (carry 40; the advisor's U4r brief check, U4R-Q12 (a)):** after U4h merges, the main session replays
    U4r's SS P0 seed 4 recording on the U4h build and counts both legs' replacements; at 0 that is the clean pair, else
    SS P0 seed 4 is re-recorded on the U4h build (its SHA-256 beside the old one's); then the three pairs (clean,
    wallclock, late-command), six runs, by U4r's pre-check script, results in results/u4h-precheck-30/<commit>/, read by
    the advisor in U4h's review; -30 stays NOT-RUN until U6. U4h's brief states what its estimate does with an L1
    session whose `rtt_ns` is still `None`; the re-run compares each client's first-sample instant with the instant
    U4h's estimate first reads that client's RTT (its receipt or its run, both logged at U4r); the Loopback leg's
    sessions keep `RttSource::Link`.
````

### Carry 89
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 587-590 (sha256 c74e52553bd8), as they are:_

````text
89. **F5, the recordings' span copy (the advisor's U4r build ruling, point 5; advisor-u4r-build1-full.md; added by the
    main session):** "When U4h changes npt-room's HISTORY_MS (it sizes the ring: §4.9; carry 28), it makes the span
    readable outside npt-room (a pub constant or getter), npt-lab's ROOM_HISTORY_MS gives way to it, and the runner
    entries are re-pinned (the advisor's U4r build ruling, point 5)."
````

### Carry 66
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 621-631 (sha256 c74e52553bd8), as they are:_

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

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 41, 42, 43, 44, 45, 46, 47, 48, 50, 51, 52, 53, 54, 55, 56, 64, 65, 67, 68, 70, 71, 72, 79, 80, 81.
