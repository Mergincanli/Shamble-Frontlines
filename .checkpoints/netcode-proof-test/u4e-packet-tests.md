# npt-room's tests (crates/room/tests, new): git diff --no-index .gate/u4e-empty .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/bites.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/bites.rs
new file mode 100644
index 0000000..4bdfe08
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/bites.rs
@@ -0,0 +1,33 @@
+//! npt-room's bite list (bites builds only; U4E-Q3 (a), U4E-Q13 (a)): pinned, and each PM-form id checked against
+//! marks.json as server/tests/schedule.rs does, the one id outside that form exempt by name (no mark judges catalogue
+//! item 5; check.mjs's bite-home grep reads only `PM-<ADR>-<row>/<name>`, so npt-server's tests hold its one home).
+#![cfg(feature = "bites")]
+
+const OWNER_TO_ALL: &str = "catalogue-5/owner-to-all";
+
+#[test]
+fn the_rooms_bites_are_pinned_and_every_pm_id_is_marks_jsons() {
+    assert_eq!(
+        npt_room::BITES,
+        [
+            "PM-0001-18/no-fallback",
+            "PM-0001-58/lt-ack",
+            "PM-0001-58/lt-cmd-seq",
+            "PM-0001-58/lt-press",
+            "PM-0001-62/server-echo",
+            "PM-0001-22/unbounded",
+            OWNER_TO_ALL,
+        ]
+    );
+    let marks = include_str!("../../../marks/marks.json");
+    for id in npt_room::BITES {
+        if id == OWNER_TO_ALL {
+            assert!(!marks.contains(id), "{id} is no mark's");
+            continue;
+        }
+        assert!(
+            marks.contains(&format!("\"{id}\"")),
+            "{id} is not in marks.json"
+        );
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/catalogue.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/catalogue.rs
new file mode 100644
index 0000000..3b5898f
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/catalogue.rs
@@ -0,0 +1,80 @@
+//! e1, catalogue item 4 (docs/design/netcode-proof-messages.md §6; u4-build-plan.md :717): the encoder's layout table
+//! (npt-proto's `Snapshot::GROUPS` and `FullState::GROUPS`, the one table the room's encoder codes with) names exactly
+//! the "Field groups" rows for `Snapshot` and `FullState`, each with the same class; the `authority-only` groups have no
+//! entry. The doc is read as proto/tests/catalogue.rs reads it: its path from `CARGO_MANIFEST_DIR`, tables by header.
+
+use npt_proto::{Class, FullState, Snapshot};
+
+const DOC: &str = concat!(
+    env!("CARGO_MANIFEST_DIR"),
+    "/../../../../docs/design/netcode-proof-messages.md"
+);
+const FIELD_GROUPS: &str = "| Message | Field group | Class | Sent to | Notes |";
+
+fn cells(line: &str) -> Vec<String> {
+    let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
+    inner.split('|').map(|c| c.trim().to_owned()).collect()
+}
+
+/// The rows of the table under `header`, the separator skipped.
+fn rows(text: &str, header: &str) -> Vec<Vec<String>> {
+    let mut lines = text.lines().skip_while(|l| l.trim_end() != header);
+    assert!(lines.next().is_some(), "no table headed {header}");
+    lines
+        .take_while(|l| l.starts_with('|'))
+        .filter(|l| !l.starts_with("|---"))
+        .map(cells)
+        .collect()
+}
+
+fn class(c: Class) -> &'static str {
+    match c {
+        Class::Owner => "owner",
+        Class::Team => "team",
+        Class::Spatial => "spatial",
+        Class::Public => "public",
+    }
+}
+
+#[test]
+fn e1_the_layout_tables_equal_the_field_group_rows_and_hold_no_authority_only_group() {
+    let text = std::fs::read_to_string(DOC).unwrap_or_else(|e| panic!("{DOC}: {e}"));
+    let table = rows(&text, FIELD_GROUPS);
+    let tables: [(&str, Vec<(&str, &str)>); 2] = [
+        (
+            "`Snapshot`",
+            Snapshot::GROUPS.iter().map(|g| (g.0, class(g.1))).collect(),
+        ),
+        (
+            "`FullState`",
+            FullState::GROUPS
+                .iter()
+                .map(|g| (g.0, class(g.1)))
+                .collect(),
+        ),
+    ];
+    let mut authority_only = 0;
+    for (message, code) in tables {
+        let doc: Vec<(String, String)> = table
+            .iter()
+            .filter(|r| r[0] == message)
+            .map(|r| (r[1].clone(), r[2].clone()))
+            .collect();
+        assert!(!doc.is_empty(), "no rows for {message}");
+        authority_only += doc.iter().filter(|r| r.1 == "authority-only").count();
+        let doc: Vec<(&str, &str)> = doc
+            .iter()
+            .filter(|r| r.1 != "authority-only")
+            .map(|r| (r.0.as_str(), r.1.as_str()))
+            .collect();
+        assert_eq!(
+            code, doc,
+            "{message}: the layout table against the catalogue's rows"
+        );
+        assert!(code.iter().all(|g| g.1 != "authority-only"));
+    }
+    assert_eq!(
+        authority_only, 6,
+        "three authority-only groups under each message, none in the tables"
+    );
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/room.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/room.rs
new file mode 100644
index 0000000..da62ecb
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/room/tests/room.rs
@@ -0,0 +1,1153 @@
+//! U4e's room tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.4): `Room<World>` driven through its own API by fake
+//! clients that acknowledge what they are sent (each tick's snapshot decoded with npt-proto, as a client would), with
+//! npt-sim's synthetic recording's commands. The Loopback tests with the real client endpoint are npt-server's.
+
+use std::collections::BTreeMap;
+
+use npt_proto::{
+    Direction, EntityDelta, FullState, Input, Join, Limits, Message, Resync, Snapshot,
+};
+use npt_room::{Room, rewind_at};
+use npt_sim::replay::Recording;
+use npt_sim::{
+    Command, CoverVariant, EntityKind, EntityRecord, HitHistory, HitVolumes, InputReports,
+    PortalStates, Scene, SimCore, SimEvent, TickInput, World,
+};
+
+fn token(slot: u8) -> [u8; 16] {
+    [slot.wrapping_add(1); 16]
+}
+
+fn join(slot: u8) -> Join {
+    Join {
+        token: token(slot),
+        bind: [slot ^ 0x3c; 16],
+        version: 1,
+        content_hash: [0; 32],
+    }
+}
+
+/// A room over `sim` on `scene` (its settings and bite from `prep`, before anyone joins), `n` slots joined.
+fn room_with<S: SimCore>(
+    sim: S,
+    scene: &Scene,
+    n: u8,
+    prep: impl FnOnce(Room<S>) -> Room<S>,
+) -> Room<S> {
+    let greybox = World::new(scene, 0, 0).greybox().clone();
+    let room = Room::new(
+        sim,
+        scene,
+        &greybox,
+        (0..n).map(token).collect(),
+        usize::from(n),
+        30,
+    );
+    let mut room = prep(room);
+    for s in 0..n {
+        assert_eq!(room.join(&join(s)).map(|j| j.0), Ok(s));
+    }
+    room
+}
+
+fn room_of<S: SimCore>(sim: S, scene: &Scene, n: u8) -> Room<S> {
+    room_with(sim, scene, n, |r| r)
+}
+
+fn world_room(scene: &Scene, seed: u64, first: u32, n: u8) -> Room<World> {
+    room_of(World::new(scene, seed, first), scene, n)
+}
+
+/// The commands of npt-sim's synthetic recording, by (slot, command tick).
+fn recorded(scene: &Scene, seed: u64, first: u32, ticks: u32) -> BTreeMap<(u8, u32), Command> {
+    let rec = Recording::synthetic(scene, seed, first, ticks, 2_000);
+    let mut by = BTreeMap::new();
+    for t in &rec.ticks {
+        for (c, _) in &t.commands {
+            by.entry((c.slot, c.tick)).or_insert(*c);
+        }
+    }
+    by
+}
+
+/// What one tick sent one client: its snapshot groups (decoded), their bytes, and its control messages.
+#[derive(Default)]
+struct Out {
+    tick: u32,
+    snapshots: Vec<Snapshot>,
+    bytes: Vec<Vec<u8>>,
+    full: Option<FullState>,
+    control: Vec<Message>,
+}
+
+/// The datagram channel, read from `Snapshot`'s row (npt-room's tests do not depend on npt-link).
+fn snapshot_of(bytes: &[u8]) -> Snapshot {
+    let (name, channel, _, _) = Limits::TYPES[1];
+    assert_eq!(name, "Snapshot");
+    match npt_proto::decode(Direction::RoomToClient, channel, bytes) {
+        Ok(Message::Snapshot(s)) => s,
+        other => panic!("the room sent a datagram that is no snapshot: {other:?}"),
+    }
+}
+
+fn collect<S: SimCore>(room: &mut Room<S>, slot: u8) -> Out {
+    let mut out = Out {
+        tick: room.last_tick(),
+        ..Out::default()
+    };
+    for d in room.take_datagrams(slot) {
+        out.snapshots.push(snapshot_of(&d));
+        out.bytes.push(d);
+    }
+    for m in room.take_control(slot) {
+        if let Message::FullState(f) = &m {
+            out.full = Some(f.clone());
+        }
+        out.control.push(m);
+    }
+    out
+}
+
+/// A fake client's acknowledgement state: the newest tick it got and the groups of it.
+#[derive(Clone, Copy, Default)]
+struct Ack(Option<(u32, u8)>);
+
+impl Ack {
+    fn take(&mut self, out: &Out) {
+        if out.full.is_some() {
+            self.0 = Some((out.tick, 1));
+        }
+        for s in &out.snapshots {
+            let m = match self.0 {
+                Some((t, m)) if t == s.tick => m,
+                _ => 0,
+            };
+            self.0 = Some((s.tick, m | 1 << s.group));
+        }
+    }
+    fn input(&self, commands: Vec<Command>) -> Input {
+        let (ack_tick, ack_groups) = self.0.map_or((0, 0), |(t, m)| (t as u16, m));
+        Input {
+            ack_tick,
+            ack_groups,
+            commands,
+        }
+    }
+}
+
+/// Runs `ticks` ticks of `room`'s `n` clients: before each tick every client whose `acks` says so acknowledges the
+/// newest tick it got, with its command from `command` for the tick about to run; after each, `seen` gets each client's
+/// output.
+fn drive<S: SimCore>(
+    room: &mut Room<S>,
+    n: u8,
+    ticks: u32,
+    acks: impl Fn(u8, u32) -> bool,
+    command: impl FnMut(u8, u32) -> Option<Command>,
+    seen: impl FnMut(&Room<S>, u8, &Out),
+) {
+    Clients::new(n).run(room, ticks, acks, command, seen);
+}
+
+/// The fake clients' acknowledgement states, kept across runs.
+struct Clients {
+    ack: Vec<Ack>,
+}
+
+impl Clients {
+    fn new(n: u8) -> Clients {
+        Clients {
+            ack: vec![Ack::default(); usize::from(n)],
+        }
+    }
+
+    fn run<S: SimCore>(
+        &mut self,
+        room: &mut Room<S>,
+        ticks: u32,
+        acks: impl Fn(u8, u32) -> bool,
+        mut command: impl FnMut(u8, u32) -> Option<Command>,
+        mut seen: impl FnMut(&Room<S>, u8, &Out),
+    ) {
+        let n = self.ack.len() as u8;
+        for _ in 0..ticks {
+            let next = room.last_tick().wrapping_add(1);
+            for s in 0..n {
+                let cmds: Vec<Command> = command(s, next).into_iter().collect();
+                let input = if acks(s, next) {
+                    self.ack[usize::from(s)].input(cmds)
+                } else {
+                    Input {
+                        commands: cmds,
+                        ..Input::default()
+                    }
+                };
+                room.receive(s, &Message::Input(input));
+            }
+            room.tick();
+            for s in 0..n {
+                let out = collect(room, s);
+                self.ack[usize::from(s)].take(&out);
+                seen(room, s, &out);
+            }
+        }
+    }
+}
+
+/// Plan silent (built and tested here): a `Join` on a slot with a live session replaces it (catalogue :93): the room
+/// reports the replacement, so the caller sends the old connection `Kick(replaced)`, and gives the slot a fresh
+/// session with its own `JoinAccepted` and full state.
+#[test]
+fn a_join_on_a_slot_with_a_live_session_replaces_it() {
+    let scene = Scene::course();
+    let mut room = world_room(&scene, 1, 100, 1);
+    room.tick();
+    assert!(collect(&mut room, 0).full.is_some());
+    assert_eq!(room.join(&join(0)), Ok((0, true)));
+    let out = collect(&mut room, 0);
+    assert!(matches!(out.control.as_slice(), [Message::JoinAccepted(_)]));
+    assert_eq!(room.roster().live(), 1);
+    room.tick();
+    assert!(collect(&mut room, 0).full.is_some(), "a fresh full state");
+}
+
+/// e8 (§4.11 :401-402): `Room<World>` is `Send`, so U6 can move it to the tick thread.
+#[test]
+fn e8_a_room_of_world_is_send() {
+    fn send<T: Send>() {}
+    send::<Room<World>>();
+}
+
+/// Carry 21: commands received in two orders give one `TickInput` (the canonical order) and one state.
+#[test]
+fn carry_21_commands_received_in_two_orders_give_one_tick_input() {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 3, 500, 80);
+    let mut a = world_room(&scene, 3, 500, 48);
+    let mut b = world_room(&scene, 3, 500, 48);
+    for _ in 0..60 {
+        let next = a.last_tick().wrapping_add(1);
+        let inputs: Vec<(u8, Input)> = (0..48u8)
+            .map(|s| {
+                let now = script.get(&(s, next)).copied();
+                let after = script.get(&(s, next.wrapping_add(1))).copied();
+                // Each client's newest command first, then the one before: two of one slot in one datagram.
+                let commands = after.into_iter().chain(now).collect();
+                (
+                    s,
+                    Input {
+                        commands,
+                        ..Input::default()
+                    },
+                )
+            })
+            .collect();
+        for (s, i) in &inputs {
+            a.receive(*s, &Message::Input(i.clone()));
+        }
+        for (s, i) in inputs.iter().rev() {
+            // The other room gets them in reverse, and each datagram twice.
+            b.receive(*s, &Message::Input(i.clone()));
+            b.receive(*s, &Message::Input(i.clone()));
+        }
+        a.tick();
+        b.tick();
+        assert_eq!(a.tick_input(), b.tick_input(), "tick {next}");
+        assert_eq!(a.sim().state_hash(), b.sim().state_hash(), "tick {next}");
+        for s in 0..48 {
+            collect(&mut a, s);
+            collect(&mut b, s);
+        }
+    }
+    let ran: u64 = (0..48u8)
+        .filter_map(|s| a.client(s))
+        .map(|c| c.counters().commands_run)
+        .sum();
+    assert!(ran > 1_000, "{ran} commands ran");
+}
+
+fn volumes(tick: u32) -> HitVolumes {
+    HitVolumes {
+        tick,
+        ..HitVolumes::default()
+    }
+}
+
+/// Carry 22 and U4E-Q5 (a): with N the newest tick pushed, a view time at N with a fraction above 0 gives no Rewind
+/// naming N + 1; the accepting edges give Rewinds equal to the command's view time with the room's cover.
+#[test]
+fn carry_22_rewind_at_never_names_tick_n_plus_1_and_accepts_every_held_view_time() {
+    let mut h = HitHistory::for_ms(200).expect("a small ring");
+    let n0 = 1_000;
+    for t in n0..n0 + 20 {
+        h.push(&volumes(t));
+    }
+    let n = n0 + 19;
+    let oldest = n + 1 - h.capacity();
+    let at = |view_tick: u32, view_fraction: u8| Command {
+        view_tick,
+        view_fraction,
+        ..Command::default()
+    };
+    let cover = CoverVariant::Present;
+    assert_eq!(rewind_at(&at(n, 1), &h, cover), None);
+    assert_eq!(rewind_at(&at(n, 255), &h, cover), None);
+    for (tick, fraction) in [(n, 0), (n - 1, 1), (n - 1, 255), (oldest, 0), (oldest, 7)] {
+        let r = rewind_at(&at(tick, fraction), &h, cover).expect("held");
+        assert_eq!((r.tick_a, r.fraction, r.cover), (tick, fraction, cover));
+    }
+    assert_eq!(rewind_at(&at(oldest - 1, 0), &h, cover), None);
+    assert_eq!(rewind_at(&at(n + 1, 0), &h, cover), None);
+}
+
+/// Carry 22 in the room (U4E-Q5 (a)): a firing command whose view time the ring does not hold runs with no Rewind and
+/// the room counts it, with its sequence for the per-shot log; a held one changes no count.
+#[test]
+fn carry_22_the_room_counts_each_firing_command_its_ring_cannot_rewind() {
+    let scene = Scene::course();
+    let mut room = world_room(&scene, 1, 100, 1);
+    let fire = |tick: u32, seq: u16, view_tick: u32, view_fraction: u8| Command {
+        tick,
+        seq,
+        buttons: Command::FIRE,
+        view_tick,
+        view_fraction,
+        ..Command::default()
+    };
+    for _ in 0..5 {
+        room.tick();
+    }
+    // The newest tick in the ring is the last tick run; the next tick's command views it.
+    let mut seq = 0;
+    let mut send = |room: &mut Room<World>, f: &dyn Fn(u32, u16) -> Command| -> u64 {
+        let next = room.last_tick() + 1;
+        seq += 1;
+        room.receive(
+            0,
+            &Message::Input(Input {
+                commands: vec![f(next, seq)],
+                ..Input::default()
+            }),
+        );
+        room.tick();
+        room.client(0).map_or(0, |c| c.counters().rewinds_refused)
+    };
+    let held = send(&mut room, &|t, s| fire(t, s, t - 1, 0));
+    assert_eq!(held, 0);
+    let refused = send(&mut room, &|t, s| fire(t, s, t - 1, 9));
+    assert_eq!(refused, 1);
+    assert_eq!(
+        room.client(0).map(|c| c.rewinds_refused().to_vec()),
+        Some(vec![2])
+    );
+    let held_again = send(&mut room, &|t, s| fire(t, s, t - 2, 9));
+    assert_eq!(held_again, 1);
+}
+
+/// e9: a second `Resync` before the full state's acknowledgement is dropped and counted (catalogue :90).
+#[test]
+fn e9_a_second_resync_before_the_full_states_ack_is_dropped_and_counted() {
+    let scene = Scene::course();
+    let mut room = world_room(&scene, 1, 100, 1);
+    let mut ack = Ack::default();
+    for _ in 0..3 {
+        room.receive(0, &Message::Input(ack.input(Vec::new())));
+        room.tick();
+        ack.take(&collect(&mut room, 0));
+    }
+    let c = |room: &Room<World>| {
+        room.client(0)
+            .map(|c| c.counters().clone())
+            .unwrap_or_default()
+    };
+    assert_eq!((c(&room).resyncs, c(&room).resyncs_dropped), (0, 0));
+    room.receive(0, &Message::Resync(Resync));
+    room.receive(0, &Message::Resync(Resync));
+    assert_eq!((c(&room).resyncs, c(&room).resyncs_dropped), (1, 1));
+    room.tick();
+    let out = collect(&mut room, 0);
+    assert!(out.full.is_some() && out.snapshots.is_empty());
+    room.receive(0, &Message::Resync(Resync));
+    assert_eq!(c(&room).resyncs_dropped, 2, "still in flight");
+    ack.take(&out);
+    room.receive(0, &Message::Input(ack.input(Vec::new())));
+    room.receive(0, &Message::Resync(Resync));
+    assert_eq!(
+        (c(&room).resyncs, c(&room).resyncs_dropped),
+        (2, 2),
+        "acknowledged: a new one is taken"
+    );
+}
+
+/// e9: an acknowledgement naming a tick never sent refuses its datagram's commands (§4.14 :471).
+#[test]
+fn e9_an_ack_naming_a_never_sent_tick_refuses_its_datagrams_commands() {
+    let scene = Scene::course();
+    let mut room = world_room(&scene, 1, 100, 1);
+    let mut ack = Ack::default();
+    for _ in 0..4 {
+        room.receive(0, &Message::Input(ack.input(Vec::new())));
+        room.tick();
+        ack.take(&collect(&mut room, 0));
+    }
+    let next = room.last_tick() + 1;
+    let cmd = Command {
+        tick: next,
+        seq: 1,
+        ..Command::default()
+    };
+    let ahead = Input {
+        ack_tick: (next + 5) as u16,
+        ack_groups: 1,
+        commands: vec![cmd],
+    };
+    room.receive(0, &Message::Input(ahead));
+    let k = room
+        .client(0)
+        .map(|c| c.counters().clone())
+        .unwrap_or_default();
+    assert_eq!(
+        (k.ack_refused, k.ack_refused_commands, k.commands_buffered),
+        (1, 1, 0)
+    );
+    room.receive(0, &Message::Input(ack.input(vec![cmd])));
+    let k = room
+        .client(0)
+        .map(|c| c.counters().clone())
+        .unwrap_or_default();
+    assert_eq!((k.ack_refused, k.commands_buffered), (1, 1));
+}
+
+/// The own-state section's identity check (carry 53): every own-section record of every snapshot and full state a
+/// client gets is its own slot's, with its soldier's generation, and the section it holds after each is exactly its
+/// soldier, its owner-only record and its `Scene::effect_rows()` effect rows (carry 51: row i belongs to slot
+/// i / effect_rows).
+struct OwnCheck {
+    rows: u16,
+    /// Per client, its own-state view by tick.
+    views: Vec<BTreeMap<u32, Vec<EntityRecord>>>,
+}
+
+fn key(r: &EntityRecord) -> (EntityKind, u16) {
+    (r.kind, r.index)
+}
+
+impl OwnCheck {
+    fn new(scene: &Scene, n: u8) -> OwnCheck {
+        OwnCheck {
+            rows: u16::from(scene.effect_rows()),
+            views: vec![BTreeMap::new(); usize::from(n)],
+        }
+    }
+
+    fn owns(&self, slot: u8, r: &EntityRecord) -> bool {
+        match r.kind {
+            EntityKind::Soldier | EntityKind::OwnState => r.index == u16::from(slot),
+            EntityKind::Effect => self.rows > 0 && r.index / self.rows == u16::from(slot),
+            _ => false,
+        }
+    }
+
+    /// Checks one client's output at its tick against the generation of its soldier.
+    fn check(&mut self, slot: u8, out: &Out, generation: u16) -> Result<(), String> {
+        let t = out.tick;
+        let view = if let Some(f) = &out.full {
+            for r in &f.own {
+                if !self.owns(slot, r) || r.generation != generation {
+                    return Err(format!(
+                        "tick {t}: client {slot}'s full state holds {:?} {} gen {}",
+                        r.kind, r.index, r.generation
+                    ));
+                }
+            }
+            let mut v = f.own.clone();
+            v.sort_by_key(key);
+            v
+        } else if let Some(first) = out.snapshots.first() {
+            let base: Vec<EntityRecord> = if first.baseline_age == 0 {
+                Vec::new()
+            } else {
+                let b = t.wrapping_sub(u32::from(first.baseline_age));
+                self.views[usize::from(slot)]
+                    .get(&b)
+                    .cloned()
+                    .ok_or_else(|| format!("tick {t}: client {slot}'s baseline {b} unknown"))?
+            };
+            let mut v = base.clone();
+            for s in &out.snapshots {
+                for d in &s.own {
+                    let (kind, index, added) = match d {
+                        EntityDelta::Add(r) => (r.kind, r.index, Some(r.generation)),
+                        EntityDelta::Update { kind, index, .. }
+                        | EntityDelta::Remove { kind, index } => (*kind, *index, None),
+                    };
+                    let probe = EntityRecord {
+                        kind,
+                        index,
+                        generation: added.unwrap_or(generation),
+                        area: 0,
+                        grid: [0; EntityRecord::FIELDS],
+                        raw: [0; EntityRecord::FIELDS],
+                    };
+                    if !self.owns(slot, &probe) || added.is_some_and(|g| g != generation) {
+                        return Err(format!(
+                            "tick {t}: client {slot}'s snapshot holds {kind:?} {index} generation {added:?}"
+                        ));
+                    }
+                    let b = base.iter().find(|r| key(r) == (kind, index));
+                    let at = v.binary_search_by_key(&(kind, index), key);
+                    match d.apply(b) {
+                        Ok(Some(r)) => match at {
+                            Ok(i) => v[i] = r,
+                            Err(i) => v.insert(i, r),
+                        },
+                        Ok(None) => {
+                            if let Ok(i) = at {
+                                v.remove(i);
+                            }
+                        }
+                        Err(e) => return Err(format!("tick {t}: client {slot}: {e}")),
+                    }
+                }
+            }
+            v
+        } else {
+            return Err(format!("tick {t}: client {slot} got no snapshot"));
+        };
+        let want: Vec<(EntityKind, u16)> = [(EntityKind::Soldier, u16::from(slot))]
+            .into_iter()
+            .chain((0..self.rows).map(|i| (EntityKind::Effect, u16::from(slot) * self.rows + i)))
+            .chain([(EntityKind::OwnState, u16::from(slot))])
+            .collect();
+        let got: Vec<(EntityKind, u16)> = view.iter().map(key).collect();
+        if got != want || view.iter().any(|r| r.generation != generation) {
+            return Err(format!(
+                "tick {t}: client {slot} holds {got:?}, wants {want:?} at gen {generation}"
+            ));
+        }
+        let views = &mut self.views[usize::from(slot)];
+        views.insert(t, view);
+        while views.len() > 40 {
+            views.pop_first();
+        }
+        Ok(())
+    }
+}
+
+fn generations(room: &Room<World>) -> Vec<u16> {
+    let mut g = vec![0; 64];
+    for r in room.sim().entity_records() {
+        if r.kind == EntityKind::Soldier {
+            g[usize::from(r.index)] = r.generation;
+        }
+    }
+    g
+}
+
+/// Runs carry 53's check on an SS room of 48 clients (driven by the recording of `seed`) for at least `min` ticks and
+/// until a generation change has been compared, at most `max`; the ticks run, the generation changes, and per
+/// client the own-state section's bits per snapshot and the `OwnState` record's bits and changes (carry 56).
+struct OwnRun {
+    ticks: u32,
+    changes: u32,
+    own_bits: Vec<u32>,
+    own_state_bits: Vec<u32>,
+    own_state_changed: u64,
+    snapshots: u64,
+}
+
+fn own_run(
+    mut room: Room<World>,
+    scene: &Scene,
+    seed: u64,
+    first: u32,
+    min: u32,
+    max: u32,
+) -> Result<OwnRun, String> {
+    let script = recorded(scene, seed, first, max + 10);
+    let mut check = OwnCheck::new(scene, 48);
+    let mut last = generations(&room);
+    let mut run = OwnRun {
+        ticks: 0,
+        changes: 0,
+        own_bits: Vec::new(),
+        own_state_bits: Vec::new(),
+        own_state_changed: 0,
+        snapshots: 0,
+    };
+    let mut err = None;
+    let mut clients = Clients::new(48);
+    while run.ticks < max && (run.ticks < min || run.changes == 0) && err.is_none() {
+        let gens = generations(&room);
+        clients.run(
+            &mut room,
+            1,
+            |_, _| true,
+            |s, t| script.get(&(s, t)).copied(),
+            |room, s, out| {
+                if err.is_some() {
+                    return;
+                }
+                let g = generations(room)[usize::from(s)];
+                if let Err(e) = check.check(s, out, g) {
+                    err = Some(e);
+                }
+                if let Some(c) = room.client(s)
+                    && !out.snapshots.is_empty()
+                {
+                    let (own, own_state, changed) = c.own_section();
+                    run.own_bits.push(own);
+                    run.own_state_bits.push(own_state);
+                    run.own_state_changed += u64::from(changed);
+                    run.snapshots += 1;
+                }
+            },
+        );
+        let now = generations(&room);
+        run.changes += now.iter().zip(&gens).filter(|(a, b)| a != b).count() as u32;
+        last = now;
+        run.ticks += 1;
+    }
+    let _ = last;
+    match err {
+        Some(e) => Err(e),
+        None => Ok(run),
+    }
+}
+
+fn stats(bits: &[u32]) -> (f64, u32, u32) {
+    let mut v: Vec<u32> = bits.iter().map(|b| b.div_ceil(8)).collect();
+    v.sort_unstable();
+    let mean = v.iter().map(|b| f64::from(*b)).sum::<f64>() / v.len().max(1) as f64;
+    let p99 = v
+        .get((v.len() * 99 / 100).min(v.len().saturating_sub(1)))
+        .copied()
+        .unwrap_or(0);
+    (mean, p99, v.last().copied().unwrap_or(0))
+}
+
+/// Carries 51, 53 and 56: for every client of an SS room (48 clients), on each of at least 300 ticks and through a
+/// generation change, every own-section record is its own slot's with its soldier's generation and its effect rows
+/// exactly its slot's (`Scene::effect_rows()`); each client gets a snapshot on every tick. The own-state section's
+/// size is measured over the same run (carry 56's SS half).
+/// A world of `scene` from `seed`, stepped through `warm` ticks of its synthetic recording from `first` (with its own
+/// hit history), so a room built round it starts where a stress-scene soldier is about to die: a soldier dies only
+/// after about 1,000 hits (a hit takes 1 of 1,000 health), about 36 s into SS for seed 1 (tick 1,287 from 200).
+fn warmed(scene: &Scene, seed: u64, first: u32, warm: u32) -> World {
+    let rec = Recording::synthetic(scene, seed, first, warm, 2_000);
+    let mut w = World::new(scene, seed, first);
+    let mut h = HitHistory::for_ms(2_000).expect("a 2 s ring");
+    for t in &rec.ticks {
+        w.step(t, &h);
+        h.push(w.hit_volumes());
+    }
+    w
+}
+
+#[test]
+fn carries_51_53_and_56_ss_each_client_gets_exactly_its_own_state_rows() {
+    let scene = Scene::stress();
+    let (seed, first, warm) = (1, 200, 850);
+    let world = warmed(&scene, seed, first, warm);
+    let start = world.next_tick();
+    let run = own_run(room_of(world, &scene, 48), &scene, seed, first, 300, 1_000)
+        .unwrap_or_else(|e| panic!("{e}"));
+    let (mean, p99, max) = stats(&run.own_bits);
+    let (smean, _, _) = stats(&run.own_state_bits);
+    let first = start;
+    println!(
+        "carry 53: SS seed {seed}, {} ticks from tick {first}, 48 clients, {} generation changes; carry 56 (SS): \
+         own-state section {mean:.2} B mean, {p99} B p99, {max} B max per snapshot ({} snapshots); OwnState record \
+         {smean:.2} B mean, changed in {:.4} of snapshots",
+        run.ticks,
+        run.changes,
+        run.snapshots,
+        run.own_state_changed as f64 / run.snapshots.max(1) as f64
+    );
+    assert!(
+        run.ticks >= 300 && run.changes >= 1,
+        "{} ticks, {} changes",
+        run.ticks,
+        run.changes
+    );
+}
+
+/// Carry 56's ES half (after U4d-F, merged at `<base>`): the own-state section's size in ES, 48 clients, 300 ticks.
+#[test]
+fn carry_56_es_the_own_state_sections_size_is_measured() {
+    let scene = Scene::estimate();
+    let (seed, first) = (1, 200);
+    let run = own_run(
+        world_room(&scene, seed, first, 48),
+        &scene,
+        seed,
+        first,
+        300,
+        300,
+    )
+    .unwrap_or_else(|e| panic!("{e}"));
+    let (mean, p99, max) = stats(&run.own_bits);
+    let (smean, _, _) = stats(&run.own_state_bits);
+    println!(
+        "carry 56 (ES): seed {seed}, {} ticks from tick {first}, 48 clients: own-state section {mean:.2} B mean, \
+         {p99} B p99, {max} B max per snapshot ({} snapshots); OwnState record {smean:.2} B mean, changed in {:.4} \
+         of snapshots",
+        run.ticks,
+        run.snapshots,
+        run.own_state_changed as f64 / run.snapshots.max(1) as f64
+    );
+    assert!(run.snapshots > 0);
+}
+
+/// A `SimCore` over `World` that plants one out-of-range field in one slot's `OwnState` record on one tick (carry
+/// 54): the ammunition above its 16-bit field.
+struct Planted {
+    world: World,
+    at: u32,
+    slot: u16,
+    events: Vec<SimEvent>,
+    records: Vec<EntityRecord>,
+    planted: bool,
+}
+
+impl SimCore for Planted {
+    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
+        let tick = self.world.next_tick();
+        self.events = self.world.step(input, history).to_vec();
+        self.planted = tick == self.at;
+        if self.planted {
+            self.records = self.world.entity_records().to_vec();
+            for r in &mut self.records {
+                if r.kind == EntityKind::OwnState && r.index == self.slot {
+                    r.grid[3] = 70_000;
+                }
+            }
+        }
+        &self.events
+    }
+    fn state_hash(&self) -> u64 {
+        self.world.state_hash()
+    }
+    fn entity_records(&self) -> &[EntityRecord] {
+        if self.planted {
+            &self.records
+        } else {
+            self.world.entity_records()
+        }
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
+/// Carry 54 (and carry 61 (a)): one out-of-range record planted for one client on one tick: that client's snapshot is
+/// skipped and counted, with no server hash, no panic; every other client's snapshot that tick is byte-equal to the
+/// run without the plant; the client's next tick is coded afresh.
+#[test]
+fn carry_54_an_out_of_range_record_skips_that_clients_snapshot_alone() {
+    let scene = Scene::stress();
+    let (seed, first, n) = (2, 300, 8u8);
+    let (at, k) = (first + 12, 5u8);
+    let script = recorded(&scene, seed, first, 40);
+    let planted = Planted {
+        world: World::new(&scene, seed, first),
+        at,
+        slot: u16::from(k),
+        events: Vec::new(),
+        records: Vec::new(),
+        planted: false,
+    };
+    let mut a = room_of(planted, &scene, n);
+    let mut b = world_room(&scene, seed, first, n);
+    let mut outs: BTreeMap<(char, u8, u32), Vec<Vec<u8>>> = BTreeMap::new();
+    let mut hashes: BTreeMap<(char, u8, u32), Option<u64>> = BTreeMap::new();
+    drive(
+        &mut a,
+        n,
+        20,
+        |_, _| true,
+        |s, t| script.get(&(s, t)).copied(),
+        |room, s, out| {
+            outs.insert(('a', s, out.tick), out.bytes.clone());
+            hashes.insert(
+                ('a', s, out.tick),
+                room.client(s).and_then(|c| c.server_hash()),
+            );
+        },
+    );
+    drive(
+        &mut b,
+        n,
+        20,
+        |_, _| true,
+        |s, t| script.get(&(s, t)).copied(),
+        |room, s, out| {
+            outs.insert(('b', s, out.tick), out.bytes.clone());
+            hashes.insert(
+                ('b', s, out.tick),
+                room.client(s).and_then(|c| c.server_hash()),
+            );
+        },
+    );
+    let ka = a
+        .client(k)
+        .map(|c| c.counters().clone())
+        .unwrap_or_default();
+    assert_eq!(ka.refused_out_of_range, 1);
+    assert!(
+        outs[&('a', k, at)].is_empty(),
+        "client {k}'s snapshot at the planted tick is skipped"
+    );
+    assert_eq!(
+        hashes[&('a', k, at)],
+        None,
+        "carry 61 (a): no server hash for a snapshot not sent"
+    );
+    assert!(
+        !outs[&('a', k, at + 1)].is_empty(),
+        "the next tick codes afresh"
+    );
+    assert!(hashes[&('a', k, at + 1)].is_some());
+    for s in (0..n).filter(|s| *s != k) {
+        assert_eq!(
+            outs[&('a', s, at)],
+            outs[&('b', s, at)],
+            "client {s} at the planted tick"
+        );
+        assert!(!outs[&('a', s, at)].is_empty());
+        assert_eq!(
+            a.client(s).map(|c| c.counters().refused_out_of_range),
+            Some(0)
+        );
+    }
+    assert_eq!(a.sim().state_hash(), b.sim().state_hash());
+}
+
+/// Carry 57, first part (and -22's clean half): a never-acking client overflows its queue: one resync, counted with
+/// its tick; its headroom falls to 0 and never below; the cap is the room's data, its provisional mark exposed.
+/// Then a second overflow before the full state's acknowledgement sends `Kick(overflow)` and drops the client.
+fn never_acker(
+    room: &mut Room<World>,
+    script: &BTreeMap<(u8, u32), Command>,
+    ticks: u32,
+) -> (Vec<u32>, i64, bool) {
+    let mut resyncs = Vec::new();
+    let mut min_headroom = i64::MAX;
+    let mut kicked = false;
+    drive(
+        room,
+        8,
+        ticks,
+        |s, _| s != 0,
+        |s, t| script.get(&(s, t)).copied(),
+        |room, s, out| {
+            if s != 0 {
+                return;
+            }
+            if let Some(c) = room.client(0) {
+                if c.overflow_resync() {
+                    resyncs.push(out.tick);
+                }
+                min_headroom = min_headroom.min(c.headroom());
+            }
+            kicked |= out
+                .control
+                .iter()
+                .any(|m| matches!(m, Message::Kick(npt_proto::Kick::Overflow)));
+        },
+    );
+    (resyncs, min_headroom, kicked)
+}
+
+#[test]
+fn carry_57_a_never_acker_overflows_once_then_a_second_overflow_kicks() {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 4, 400, 400);
+    assert_eq!(
+        world_room(&scene, 4, 400, 1).event_cap(),
+        (1_024, true),
+        "the provisional cap"
+    );
+    let mut room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
+        r.with_event_cap(48)
+    });
+    assert_eq!(room.event_cap(), (48, false));
+    let (resyncs, min_headroom, kicked) = never_acker(&mut room, &script, 300);
+    let c = room.client(0).expect("slot 0");
+    assert_eq!(resyncs.len(), 1, "one resync: {resyncs:?}");
+    assert_eq!(c.counters().overflow_resyncs, 1);
+    assert_eq!(
+        min_headroom, 0,
+        "the headroom reaches 0 and never goes below"
+    );
+    assert!(kicked && c.kicked() && c.counters().kicked == 1);
+    assert!(!room.roster().is_live(0), "the client is dropped");
+    assert!(
+        room.client(1).is_some_and(|c| !c.kicked()),
+        "an acking client stays"
+    );
+}
+
+/// Carry 57's other edge: a forced full state not acknowledged within `npt_sim::FULL_STATE_ACK_TICKS` kicks; one
+/// acknowledged on the last tick of that window does not.
+#[test]
+fn carry_57_a_forced_full_state_not_acked_within_full_state_ack_ticks_kicks() {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 4, 400, 600);
+    let window = npt_sim::FULL_STATE_ACK_TICKS;
+    // A cap of 256: the queue refills far slower than the window, so the window decides.
+    let run = |ack_at: Option<u32>| {
+        let mut room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
+            r.with_event_cap(256)
+        });
+        let mut forced = None;
+        let mut kicked_at = None;
+        drive(
+            &mut room,
+            8,
+            500,
+            |s, next| s != 0 || Some(next) == ack_at,
+            |s, t| script.get(&(s, t)).copied(),
+            |room, s, out| {
+                if s != 0 || kicked_at.is_some() {
+                    return;
+                }
+                if forced.is_none() && room.client(0).is_some_and(|c| c.overflow_resync()) {
+                    forced = Some(out.tick);
+                }
+                if out.control.iter().any(|m| matches!(m, Message::Kick(_))) {
+                    kicked_at = Some(out.tick);
+                }
+            },
+        );
+        (forced, kicked_at)
+    };
+    let (forced, kicked_at) = run(None);
+    let f = forced.expect("an overflow resync");
+    assert_eq!(
+        kicked_at,
+        Some(f + window + 1),
+        "kicked on the first tick past the window"
+    );
+    // The same run, the client acknowledging the forced full state just before tick f + window + 1 runs.
+    let (again, kicked_at) = run(Some(f + window + 1));
+    assert_eq!(again, Some(f));
+    assert!(
+        kicked_at.is_none_or(|k| k > f + window + 1),
+        "acknowledged within the window: kept ({kicked_at:?})"
+    );
+}
+
+/// How many of the shots a room's clients were sent were their own (catalogue §3's cosmetic-events row: "never the
+/// client whose own shot it is"), and how many were other clients'.
+fn echo_case(mut room: Room<World>, scene: &Scene, seed: u64, first: u32) -> (u64, u64) {
+    let script = recorded(scene, seed, first, 80);
+    let (mut own, mut others) = (0, 0);
+    drive(
+        &mut room,
+        8,
+        60,
+        |_, _| true,
+        |s, t| script.get(&(s, t)).copied(),
+        |_, s, out| {
+            for snap in &out.snapshots {
+                for (_, _, e) in &snap.events {
+                    if let SimEvent::Shot { slot, .. } = e {
+                        if *slot == s {
+                            own += 1;
+                        } else {
+                            others += 1;
+                        }
+                    }
+                }
+            }
+        },
+    );
+    (own, others)
+}
+
+/// -62/server-echo's site, clean (U4E-Q10 (a)): the room's routing sends a cosmetic shot to the clients that see it,
+/// never to the shooter's own client.
+#[test]
+fn pm_0001_62_the_room_never_sends_a_shooter_its_own_cosmetic_shot() {
+    let scene = Scene::stress();
+    let (own, others) = echo_case(world_room(&scene, 5, 600, 8), &scene, 5, 600);
+    assert!(others > 0, "shots were routed");
+    assert_eq!(own, 0, "{own} own shots sent");
+}
+
+/// The wrap runs (-58's room counters, e9's "each counter accepted across its wrap in the clean build"): one client of
+/// CO whose room's tick, command sequence and melee press counter each start a few steps below their wraps. Returns
+/// the client's counters and every counter wrap the room exposed, by name.
+fn wrap_case(mut room: Room<World>) -> (npt_room::ClientCounters, Vec<&'static str>) {
+    let mut ack = Ack::default();
+    // The client acknowledges two ticks late, so an acknowledgement from before the wrap reaches a room past it.
+    let mut late = [Ack::default(); 2];
+    let mut wraps = Vec::new();
+    for k in 0..40u16 {
+        let next = room.last_tick().wrapping_add(1);
+        let mut presses = [0; 4];
+        presses[Command::PRESS_MELEE] = 250u8.wrapping_add(k as u8);
+        let cmd = Command {
+            tick: next,
+            seq: 65_520u16.wrapping_add(k),
+            presses,
+            view_tick: next.wrapping_sub(1),
+            ..Command::default()
+        };
+        room.receive(0, &Message::Input(late[0].input(vec![cmd])));
+        room.tick();
+        ack.take(&collect(&mut room, 0));
+        late = [late[1], ack];
+        if let Some(c) = room.client(0) {
+            wraps.extend_from_slice(c.wraps());
+        }
+    }
+    let counters = room
+        .client(0)
+        .map(|c| c.counters().clone())
+        .unwrap_or_default();
+    (counters, wraps)
+}
+
+/// -58's three room-side compares in the clean build (u4-build-plan.md §4.14 :467, :468, :471): the acknowledgement,
+/// the command sequence and a press counter each cross their wraps and are accepted, each wrap exposed once by name.
+#[test]
+fn pm_0001_58_the_rooms_ack_sequence_and_press_compares_accept_across_their_wraps() {
+    let scene = Scene::course();
+    let (k, wraps) = wrap_case(world_room(&scene, 1, 65_520, 1));
+    assert_eq!(
+        (k.ack_refused, k.commands_repeat, k.press_refused),
+        (0, 0, 0),
+        "{k:?}"
+    );
+    assert_eq!(k.commands_buffered, 40);
+    for name in ["ack", "cmd_seq", "press"] {
+        assert_eq!(
+            wraps.iter().filter(|w| **w == name).count(),
+            1,
+            "{name}: {wraps:?}"
+        );
+    }
+}
+
+/// PM-0001-22/unbounded's clean half and its headroom (U4E-Q12 (c)): the never-acker's headroom never goes below 0.
+fn unbounded_case(mut room: Room<World>, scene: &Scene) -> (Vec<u32>, i64, bool) {
+    let script = recorded(scene, 4, 400, 400);
+    never_acker(&mut room, &script, 150)
+}
+
+/// The room's planted defects (bites builds only), each in wave 1's two parts (u4e-brief.md 2.6): a bites-only twin
+/// that asserts the clean test's check fails on the bitten instance; part (ii) is a scratch switch of the clean test.
+#[cfg(feature = "bites")]
+mod bites {
+    use super::*;
+
+    fn bitten(scene: &Scene, seed: u64, first: u32, n: u8, id: &str) -> Room<World> {
+        room_with(World::new(scene, seed, first), scene, n, |r| {
+            r.with_bite(id).expect("npt-room plants it")
+        })
+    }
+
+    #[test]
+    fn pm_0001_62_server_echo_sends_the_owner_its_own_cosmetic_shots() {
+        let scene = Scene::stress();
+        let (own, _) = echo_case(
+            bitten(&scene, 5, 600, 8, "PM-0001-62/server-echo"),
+            &scene,
+            5,
+            600,
+        );
+        assert!(own >= 1, "one event sent: {own}");
+    }
+
+    #[test]
+    fn pm_0001_58_lt_ack_refuses_a_post_wrap_ack() {
+        let (k, _) = wrap_case(bitten(&Scene::course(), 1, 65_520, 1, "PM-0001-58/lt-ack"));
+        assert!(k.ack_refused >= 1, "{k:?}");
+    }
+
+    #[test]
+    fn pm_0001_58_lt_cmd_seq_refuses_post_wrap_commands() {
+        let (k, _) = wrap_case(bitten(
+            &Scene::course(),
+            1,
+            65_520,
+            1,
+            "PM-0001-58/lt-cmd-seq",
+        ));
+        assert!(k.commands_repeat >= 1 && k.commands_buffered < 40, "{k:?}");
+    }
+
+    #[test]
+    fn pm_0001_58_lt_press_loses_an_edge() {
+        let (k, _) = wrap_case(bitten(
+            &Scene::course(),
+            1,
+            65_520,
+            1,
+            "PM-0001-58/lt-press",
+        ));
+        assert!(k.press_refused >= 1, "{k:?}");
+    }
+
+    #[test]
+    fn pm_0001_22_unbounded_keeps_the_never_acker_past_its_cap() {
+        let scene = Scene::stress();
+        let room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
+            r.with_event_cap(48)
+                .with_bite("PM-0001-22/unbounded")
+                .expect("planted")
+        });
+        let (resyncs, min_headroom, kicked) = unbounded_case(room, &scene);
+        assert!(min_headroom < 0, "headroom below 0: {min_headroom}");
+        assert!(resyncs.is_empty() && !kicked, "no resync and no kick");
+    }
+
+    #[test]
+    fn catalogue_5_owner_to_all_fails_carry_53s_check() {
+        let scene = Scene::stress();
+        let room = bitten(&scene, 1, 200, 48, "catalogue-5/owner-to-all");
+        let err = own_run(room, &scene, 1, 200, 3, 3).err();
+        assert!(
+            err.as_deref().is_some_and(|e| e.contains("client 0")),
+            "{err:?}"
+        );
+    }
+
+    #[test]
+    fn the_room_takes_npt_protos_ids_through_quant_and_no_other() {
+        let scene = Scene::course();
+        let room = || {
+            Room::new(
+                World::new(&scene, 1, 0),
+                &scene,
+                World::new(&scene, 1, 0).greybox(),
+                vec![],
+                1,
+                30,
+            )
+        };
+        assert!(room().with_bite("PM-0001-17/tick16").is_some());
+        assert!(room().with_bite("PM-0001-17/client-rounding").is_none());
+        assert!(room().with_bite("PM-0001-18/no-such-bite").is_none());
+    }
+}
+
+/// -22's clean half with the same cap as its twin: the headroom reaches 0 and never goes below, and one resync follows.
+#[test]
+fn pm_0001_22_the_never_ackers_headroom_never_goes_below_0() {
+    let scene = Scene::stress();
+    let room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
+        r.with_event_cap(48)
+    });
+    let (resyncs, min_headroom, _) = unbounded_case(room, &scene);
+    assert_eq!(min_headroom, 0);
+    assert!(!resyncs.is_empty());
+}

# npt-server's tests (crates/server/tests): git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/server/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/common/mod.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/common/mod.rs
new file mode 100644
index 0000000..59820ca
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/common/mod.rs
@@ -0,0 +1,378 @@
+//! The Loopback lab for U4e's tests (u4-build-plan.md §5 Lanes: in npt-server's own tests, through its npt-link and its
+//! dev dependency on npt-client; never in crates/lab): a `Room<World>` behind one `Decoder` per session, each session a
+//! Loopback pair to a real `ClientEndpoint`, on an injected clock. The server ticks at U4b's schedule
+//! (base + ⌊k × 10^9 / tick_hz⌋ ns); each client steps at its own offset after each server tick. A test-only `Link`
+//! wrapper, [`Gate`], drops every datagram in a window both ways (2.5's gap; Loopback itself neither delays nor loses).
+#![allow(dead_code, reason = "each test binary uses part of the lab")]
+
+use std::cell::Cell;
+use std::collections::BTreeMap;
+use std::rc::Rc;
+
+use npt_client::ClientEndpoint;
+use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
+use npt_proto::{Join, Limits, Message, Quant};
+use npt_room::Room;
+use npt_server::Decoder;
+use npt_sim::replay::Recording;
+use npt_sim::{Command, Scene, World};
+
+pub const VERSION: u16 = 1;
+pub const CONTENT: [u8; 32] = [7; 32];
+/// Loopback's datagram limit for the lab's sessions (the link's own; the room's groups stay within 1,000 B).
+pub const MAX_DATAGRAM: usize = 1_200;
+
+pub fn token(slot: u8) -> [u8; 16] {
+    let mut t = [0x5a; 16];
+    t[0] = slot;
+    t
+}
+
+pub fn bind(slot: u8) -> [u8; 16] {
+    [slot ^ 0xa5; 16]
+}
+
+pub fn join(slot: u8) -> Join {
+    Join {
+        token: token(slot),
+        bind: bind(slot),
+        version: VERSION,
+        content_hash: CONTENT,
+    }
+}
+
+/// A window of ns in which a [`Gate`] drops every datagram it is asked to send.
+#[derive(Clone, Default)]
+pub struct Window(Rc<Cell<(u64, u64)>>);
+
+impl Window {
+    pub fn set(&self, from_ns: u64, to_ns: u64) {
+        self.0.set((from_ns, to_ns));
+    }
+    fn holds(&self, now_ns: u64) -> bool {
+        let (a, b) = self.0.get();
+        now_ns >= a && now_ns < b
+    }
+}
+
+/// A test-only `Link` over a Loopback end: datagrams sent inside its window are dropped (counted), every other call is
+/// the Loopback's.
+pub struct Gate {
+    inner: Loopback,
+    window: Window,
+    pub dropped: Rc<Cell<u64>>,
+}
+
+impl Link for Gate {
+    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
+        if self.window.holds(now_ns) {
+            self.dropped.set(self.dropped.get() + 1);
+            return SendResult::Sent;
+        }
+        self.inner.send_unreliable(bytes, now_ns)
+    }
+    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
+        self.inner.send_reliable(channel, bytes, now_ns)
+    }
+    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
+        self.inner.poll_recv(now_ns)
+    }
+    fn state(&self) -> LinkState {
+        self.inner.state()
+    }
+    fn rtt_ns(&self) -> Option<u64> {
+        self.inner.rtt_ns()
+    }
+    fn max_datagram_size(&self) -> usize {
+        self.inner.max_datagram_size()
+    }
+    fn unsent_unreliable(&self) -> usize {
+        self.inner.unsent_unreliable()
+    }
+}
+
+/// What a client's command is, from its slot and the tick it will stamp.
+pub type Script = Box<dyn FnMut(u8, u32) -> Option<Command>>;
+
+/// The commands of a synthetic recording (npt-sim's seeded generator), by (slot, tick): each slot's first command of
+/// each tick, as the page's command for the tick the client stamps it with.
+pub fn recorded(scene: &Scene, seed: u64, first_tick: u32, ticks: u32) -> Script {
+    let rec = Recording::synthetic(scene, seed, first_tick, ticks, 2_000);
+    let mut by: BTreeMap<(u8, u32), Command> = BTreeMap::new();
+    for t in &rec.ticks {
+        for (c, _) in &t.commands {
+            by.entry((c.slot, c.tick)).or_insert(*c);
+        }
+    }
+    Box::new(move |slot, tick| by.get(&(slot, tick)).copied())
+}
+
+/// One session: the server's end with its decoder, the client's end with its endpoint, and what each side filed.
+pub struct Conn {
+    pub server: Gate,
+    pub client: Gate,
+    pub decoder: Decoder,
+    pub endpoint: ClientEndpoint,
+    pub slot: Option<u8>,
+    pub offset_ns: u64,
+    pub step: u64,
+    /// Every (tick, hash) the client filed, in order (0 the sentinel).
+    pub filed: Vec<(u32, u64)>,
+    /// The runs of ticks the client resolved lost.
+    pub lost: Vec<(u32, u32)>,
+    pub wraps: Vec<&'static str>,
+}
+
+/// The lab (the module text).
+pub struct Lab {
+    pub room: Room<World>,
+    pub conns: Vec<Conn>,
+    pub hz: u32,
+    pub base_ns: u64,
+    pub k: u64,
+    pub now_ns: u64,
+    pub script: Script,
+    /// The room's hash per (slot, tick).
+    pub server: BTreeMap<(u8, u32), u64>,
+    /// The client hashes per (slot, tick): a second, different hash makes it the sentinel (marks/README.md §1).
+    pub client: BTreeMap<(u8, u32), u64>,
+    /// When each server tick ran, ns.
+    pub tick_at: BTreeMap<u32, u64>,
+    /// The window every session's gates share (both ways).
+    pub window: Window,
+    /// The bytes the pump reported to the room per (slot, tick) before that tick ran.
+    pub pumped: BTreeMap<(u8, u32), u32>,
+    /// Drops the room's datagram (slot, tick, group) when it answers true (e2's and e3's lost group).
+    pub tamper: Option<Tamper>,
+}
+
+/// Which of the room's datagrams the lab drops, by (slot, tick, group).
+pub type Tamper = Box<dyn FnMut(u8, u32, usize) -> bool>;
+
+impl Lab {
+    /// A lab of `endpoints.len()` sessions on `room`, each client stepping `offsets[i]` ns after each server tick.
+    pub fn new(
+        room: Room<World>,
+        endpoints: Vec<ClientEndpoint>,
+        offsets: &[u64],
+        script: Script,
+    ) -> Lab {
+        Lab::sized(room, endpoints, offsets, script, MAX_DATAGRAM)
+    }
+
+    /// The same, each session's Loopback taking datagrams up to `max_datagram` bytes.
+    pub fn sized(
+        room: Room<World>,
+        endpoints: Vec<ClientEndpoint>,
+        offsets: &[u64],
+        script: Script,
+        max_datagram: usize,
+    ) -> Lab {
+        let hz = npt_sim::TICK_HZ;
+        let base_ns = 1_000_000_000;
+        let window = Window::default();
+        let conns = endpoints
+            .into_iter()
+            .enumerate()
+            .map(|(i, endpoint)| {
+                let (s, c) = Loopback::pair(max_datagram);
+                Conn {
+                    server: Gate {
+                        inner: s,
+                        window: window.clone(),
+                        dropped: Rc::default(),
+                    },
+                    client: Gate {
+                        inner: c,
+                        window: window.clone(),
+                        dropped: Rc::default(),
+                    },
+                    decoder: Decoder::new(Quant::default(), hz, base_ns, VERSION, CONTENT),
+                    endpoint,
+                    slot: None,
+                    offset_ns: offsets.get(i).copied().unwrap_or(1_000_000),
+                    step: 0,
+                    filed: Vec::new(),
+                    lost: Vec::new(),
+                    wraps: Vec::new(),
+                }
+            })
+            .collect();
+        Lab {
+            room,
+            conns,
+            hz,
+            base_ns,
+            k: 0,
+            now_ns: base_ns,
+            script,
+            server: BTreeMap::new(),
+            client: BTreeMap::new(),
+            tick_at: BTreeMap::new(),
+            window,
+            pumped: BTreeMap::new(),
+            tamper: None,
+        }
+    }
+
+    /// The same lab with session `i`'s pump decoding with `quant`.
+    pub fn with_decoder(mut self, i: usize, quant: Quant) -> Lab {
+        self.conns[i].decoder = Decoder::new(quant, self.hz, self.base_ns, VERSION, CONTENT);
+        self
+    }
+
+    /// The time of server tick `k` (U4b's schedule).
+    pub fn tick_ns(&self, k: u64) -> u64 {
+        self.base_ns + (u128::from(k) * 1_000_000_000 / u128::from(self.hz)) as u64
+    }
+
+    fn server_tick(&mut self, now: u64) {
+        let tick = self.room.last_tick().wrapping_add(1);
+        for conn in &mut self.conns {
+            loop {
+                let mut msgs = Vec::new();
+                conn.decoder.pump(&mut conn.server, now, tick, &mut msgs);
+                let mut joined = false;
+                for (m, _) in msgs {
+                    match (&m, conn.slot) {
+                        (Message::Join(j), _) => {
+                            joined = true;
+                            match self.room.join(j) {
+                                Ok((slot, _)) => {
+                                    conn.decoder.accept();
+                                    conn.slot = Some(slot);
+                                }
+                                Err(r) => conn.decoder.reject(&mut conn.server, r, now),
+                            }
+                        }
+                        (_, Some(slot)) => self.room.receive(slot, &m),
+                        _ => {}
+                    }
+                }
+                if !joined {
+                    break;
+                }
+            }
+            if let Some(slot) = conn.slot {
+                let sent = conn.decoder.take_sent_bytes();
+                self.pumped.insert((slot, tick), sent);
+                self.room
+                    .pump_report(slot, sent, conn.server.max_datagram_size());
+            }
+        }
+        self.room.tick();
+        let t = self.room.last_tick();
+        self.tick_at.insert(t, now);
+        for conn in &mut self.conns {
+            let Some(slot) = conn.slot else { continue };
+            for (g, d) in self.room.take_datagrams(slot).into_iter().enumerate() {
+                if self.tamper.as_mut().is_some_and(|f| f(slot, t, g)) {
+                    continue;
+                }
+                conn.server.send_unreliable(&d, now);
+            }
+            for m in self.room.take_control(slot) {
+                let channel = Limits::TYPES
+                    .iter()
+                    .find(|r| r.0 == m.name())
+                    .map_or(Channel::Control, |r| r.1);
+                let mut out = Vec::new();
+                npt_proto::encode(&m, &mut out).expect("the room's control messages encode");
+                conn.server.send_reliable(channel, &out, now);
+            }
+            if let Some(h) = self.room.client(slot).and_then(|c| c.server_hash()) {
+                self.server.insert((slot, t), h);
+            }
+        }
+    }
+
+    fn client_step(&mut self, i: usize, now: u64) {
+        let conn = &mut self.conns[i];
+        let mut received = Vec::new();
+        while let Some(r) = conn.client.poll_recv(now) {
+            received.push((r.channel, r.bytes, r.arrival_ns));
+        }
+        conn.endpoint.input(None, &received, now);
+        // The page's command for the tick the endpoint stamps it with (its newest tick plus its lead of 1).
+        let stamp = conn.endpoint.newest_tick().map(|n| n.wrapping_add(1));
+        if let (Some(slot), Some(t)) = (conn.endpoint.slot(), stamp)
+            && let Some(c) = (self.script)(slot, t)
+        {
+            conn.endpoint.input(Some(c), &[], now);
+        }
+        let out = conn.endpoint.step();
+        for d in &out.datagrams {
+            conn.client.send_unreliable(d, now);
+        }
+        for (ch, b) in &out.reliable {
+            conn.client.send_reliable(*ch, b, now);
+        }
+        let slot = conn.endpoint.slot().unwrap_or(u8::MAX);
+        for &(t, h) in &out.hashes {
+            conn.filed.push((t, h));
+            let e = self.client.entry((slot, t)).or_insert(h);
+            if *e != h {
+                *e = 0;
+            }
+        }
+        conn.lost.extend_from_slice(&out.lost);
+        conn.wraps.extend_from_slice(&out.wraps);
+        conn.step += 1;
+    }
+
+    /// Runs `ticks` server ticks, each client stepping at its offset after each; `after` sees the lab after each
+    /// server tick.
+    pub fn run(&mut self, ticks: u64, mut after: impl FnMut(&Lab)) {
+        for _ in 0..ticks {
+            let at = self.tick_ns(self.k);
+            let next = self.tick_ns(self.k + 1);
+            self.now_ns = at;
+            self.server_tick(at);
+            after(self);
+            let mut order: Vec<(u64, usize)> = (0..self.conns.len())
+                .map(|i| (at + self.conns[i].offset_ns.min(next - at - 1), i))
+                .collect();
+            order.sort_unstable();
+            for (t, i) in order {
+                self.now_ns = t;
+                self.client_step(i, t);
+            }
+            self.k += 1;
+        }
+    }
+
+    /// Every (slot, tick) both sides hashed, and how many pairs differ or are the sentinel.
+    pub fn compare(&self) -> (usize, usize) {
+        let mut compared = 0;
+        let mut bad = 0;
+        for (k, h) in &self.client {
+            compared += 1;
+            if *h == 0 || self.server.get(k) != Some(h) {
+                bad += 1;
+            }
+        }
+        (compared, bad)
+    }
+}
+
+/// `n` endpoints that join as slots 0 to n-1.
+pub fn endpoints(n: u8) -> Vec<ClientEndpoint> {
+    (0..n)
+        .map(|s| ClientEndpoint::new(join(s), npt_sim::TICK_HZ))
+        .collect()
+}
+
+/// A room on `scene` from `seed`, its first tick `first_tick`, with a roster of `n` slots.
+pub fn room(scene: &Scene, seed: u64, first_tick: u32, n: u8) -> Room<World> {
+    let world = World::new(scene, seed, first_tick);
+    let greybox = world.greybox().clone();
+    Room::new(
+        world,
+        scene,
+        &greybox,
+        (0..n).map(token).collect(),
+        usize::from(n),
+        npt_sim::TICK_HZ,
+    )
+    .with_hashes(CONTENT, [0; 32])
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/gap.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/gap.rs
new file mode 100644
index 0000000..d5eecb4
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/gap.rs
@@ -0,0 +1,263 @@
+//! e5 and -18's bite end to end (u4e-brief.md 2.4 e5, 2.5; u4-build-plan.md §4.14 :461, §5 U4e): a gap in delivery,
+//! every datagram dropped both ways by the lab's `Gate` on the injected clock, of `npt_sim::ACK_RING_TICKS` tick
+//! periods (U4E-Q14 (b)), at two alignments against the server's send of tick T; the clients step 5 ms after each
+//! server tick. Its length in ns is computed here from that constant by path and the rate the test passes in.
+
+mod common;
+
+use std::collections::BTreeMap;
+
+use common::{Lab, endpoints, recorded, room};
+use npt_room::Room;
+use npt_sim::{Scene, World};
+
+const CLIENTS: u8 = 4;
+const OFFSET_NS: u64 = 5_000_000;
+
+/// Where the gap starts against the server's send of tick T.
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+enum Align {
+    /// 1 ns after it: T is delivered, the client's acknowledgement of T is the first datagram lost.
+    AfterSend,
+    /// 1 ns before it: T's snapshot is the first datagram lost.
+    BeforeSend,
+}
+
+/// What one gap run showed for client 0.
+#[derive(Clone, Debug, PartialEq, Eq)]
+struct Gap {
+    align: Align,
+    /// Tick T and the gap's start and end, ns.
+    t: u32,
+    from_ns: u64,
+    to_ns: u64,
+    /// The first tick the server runs at or after the gap's end, F.
+    f: u32,
+    /// B's tick (the last tick fully acknowledged) and B's age at F, and the room's per-tick eviction flag at F.
+    b_tick: Option<u32>,
+    b_age: Option<u32>,
+    evicted: bool,
+    /// The baseline age F's snapshot was coded at (0: the per-type defaults).
+    f_age: Option<u16>,
+    /// The first tick at or after F the client decoded with the server's hash, and when the server sent it.
+    fresh: Option<(u32, u64)>,
+    /// The sentinels the client filed from F on.
+    sentinels: usize,
+    /// Every pair compared over the run, and how many differ or are the sentinel; the events resent.
+    compared: usize,
+    bad: usize,
+    resent: u64,
+    /// The room's hashes, for e11.
+    server: BTreeMap<(u8, u32), u64>,
+}
+
+/// The gap's length: `npt_sim::ACK_RING_TICKS` tick periods at `tick_hz` (1.067 s at 30 Hz), in ns.
+fn ring_gap_ns(tick_hz: u32) -> u64 {
+    u64::from(npt_sim::ACK_RING_TICKS) * 1_000_000_000 / u64::from(tick_hz)
+}
+
+fn gap_case(room: Room<World>, align: Align, gap_ns: u64) -> Gap {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 1, 300, 200);
+    let offsets = vec![OFFSET_NS; usize::from(CLIENTS)];
+    let mut lab = Lab::new(room, endpoints(CLIENTS), &offsets, script);
+    let mut per_tick: BTreeMap<u32, (Option<u32>, bool, Option<u16>)> = BTreeMap::new();
+    let mut record = |lab: &Lab| {
+        if let Some(c) = lab.room.client(0) {
+            per_tick.insert(
+                lab.room.last_tick(),
+                (c.last_acked(), c.burst_eviction(), c.baseline_age()),
+            );
+        }
+    };
+    lab.run(30, &mut record);
+    let t = lab.room.last_tick() + 1;
+    let send = lab.tick_ns(lab.k);
+    let from_ns = match align {
+        Align::AfterSend => send + 1,
+        Align::BeforeSend => send - 1,
+    };
+    let to_ns = from_ns + gap_ns;
+    lab.window.set(from_ns, to_ns);
+    lab.run(32 + 30, &mut record);
+    let f = *lab
+        .tick_at
+        .iter()
+        .find(|(_, at)| **at >= to_ns)
+        .map(|(t, _)| t)
+        .expect("a tick after the gap");
+    let (b_tick, evicted, f_age) = per_tick[&f];
+    let fresh = lab
+        .client
+        .iter()
+        .filter(|((s, k), h)| {
+            *s == 0 && *k >= f && **h != 0 && lab.server.get(&(0, *k)) == Some(*h)
+        })
+        .map(|((_, k), _)| (*k, lab.tick_at[k]))
+        .next();
+    let sentinels = lab
+        .client
+        .iter()
+        .filter(|((s, k), h)| *s == 0 && *k >= f && **h == 0)
+        .count();
+    let (compared, bad) = lab.compare();
+    let resent = lab.room.client(0).map_or(0, |c| c.counters().events_resent);
+    Gap {
+        align,
+        t,
+        from_ns,
+        to_ns,
+        f,
+        b_tick,
+        b_age: b_tick.map(|b| f - b),
+        evicted,
+        f_age,
+        fresh,
+        sentinels,
+        compared,
+        bad,
+        resent,
+        server: lab.server.clone(),
+    }
+}
+
+fn clean_room() -> Room<World> {
+    room(&Scene::stress(), 1, 300, CLIENTS)
+}
+
+/// e5a (u4e-brief.md 2.4; U4E-Q14 (b)): at both alignments the gap leaves B at age ≥ `npt_sim::ACK_RING_TICKS` at the
+/// first tick after it, and `burst_eviction` is 1 there; a 1 s gap's B is reported beside it (not asserted).
+#[test]
+fn e5a_a_gap_of_ack_ring_ticks_periods_evicts_b_at_both_alignments() {
+    for align in [Align::AfterSend, Align::BeforeSend] {
+        let g = gap_case(clean_room(), align, ring_gap_ns(npt_sim::TICK_HZ));
+        println!(
+            "e5a {align:?}: T {} gap {}..{} ns, F {}, B {:?} at age {:?}, evicted {}",
+            g.t, g.from_ns, g.to_ns, g.f, g.b_tick, g.b_age, g.evicted
+        );
+        assert!(
+            g.b_age.is_some_and(|a| a >= npt_sim::ACK_RING_TICKS),
+            "{align:?}: {g:?}"
+        );
+        assert!(g.evicted, "{align:?}: burst_eviction = 1");
+        let one = gap_case(clean_room(), align, 1_000_000_000);
+        println!(
+            "e5a {align:?}, a 1 s gap: F {}, B {:?} at age {:?}, evicted {} (reported, not asserted)",
+            one.f, one.b_tick, one.b_age, one.evicted
+        );
+    }
+}
+
+/// e5b: the first snapshot after the gap is coded against the per-type defaults (`baseline_age` 0: §4.5).
+fn e5b_check(room: impl Fn() -> Room<World>) -> Result<(), String> {
+    for align in [Align::AfterSend, Align::BeforeSend] {
+        let g = gap_case(room(), align, ring_gap_ns(npt_sim::TICK_HZ));
+        if g.f_age != Some(0) {
+            return Err(format!("{align:?}: F {} coded at age {:?}", g.f, g.f_age));
+        }
+    }
+    Ok(())
+}
+
+#[test]
+fn e5b_the_first_snapshot_after_the_gap_is_coded_against_the_defaults() {
+    e5b_check(clean_room).unwrap_or_else(|e| panic!("{e}"));
+}
+
+/// 2.5: end to end, a fresh delta decodes within 500 ms of the gap's end on the injected clock, at both alignments;
+/// over the whole run every compared pair is equal (the mirror across events resent after the gap: e7's resent part).
+fn end_to_end_check(room: impl Fn() -> Room<World>) -> Result<Vec<Gap>, String> {
+    let mut out = Vec::new();
+    for align in [Align::AfterSend, Align::BeforeSend] {
+        let g = gap_case(room(), align, ring_gap_ns(npt_sim::TICK_HZ));
+        match g.fresh {
+            Some((_, at)) if at <= g.to_ns + 500_000_000 => {}
+            _ => {
+                return Err(format!(
+                    "{align:?}: no fresh delta within 500 ms of {} ns: {:?}",
+                    g.to_ns, g.fresh
+                ));
+            }
+        }
+        out.push(g);
+    }
+    Ok(out)
+}
+
+#[test]
+fn pm_0001_18_a_fresh_delta_decodes_within_500_ms_of_the_gaps_end() {
+    let runs = end_to_end_check(clean_room).unwrap_or_else(|e| panic!("{e}"));
+    for g in &runs {
+        let (tick, at) = g.fresh.unwrap_or_default();
+        println!(
+            "2.5 {:?}: gap ends {} ns; the first fresh delta is tick {tick}, sent at {at} ns ({} ms after); {} pairs, {} \
+             differ; {} events resent",
+            g.align,
+            g.to_ns,
+            (at - g.to_ns) / 1_000_000,
+            g.compared,
+            g.bad,
+            g.resent
+        );
+        assert_eq!(g.bad, 0, "{:?}", g.align);
+        assert!(g.resent >= 1, "{:?}: events resent after the gap", g.align);
+    }
+}
+
+/// -18's bite (bites builds only), in wave 1's two parts: the bites-only twins assert the defect on the bitten room.
+#[cfg(feature = "bites")]
+mod bites {
+    use super::*;
+
+    const NO_FALLBACK: &str = "PM-0001-18/no-fallback";
+
+    fn bitten() -> Room<World> {
+        clean_room()
+            .with_bite(NO_FALLBACK)
+            .expect("npt-room plants it")
+    }
+
+    #[test]
+    fn pm_0001_18_no_fallback_codes_the_first_snapshot_after_the_gap_against_an_evicted_baseline() {
+        assert!(e5b_check(bitten).is_err());
+    }
+
+    #[test]
+    fn pm_0001_18_no_fallback_leaves_no_fresh_delta_within_500_ms() {
+        assert!(end_to_end_check(bitten).is_err());
+        for align in [Align::AfterSend, Align::BeforeSend] {
+            let g = gap_case(bitten(), align, ring_gap_ns(npt_sim::TICK_HZ));
+            println!(
+                "2.5 bitten {align:?}: F {} coded at age {:?} (B {:?}, age {:?}); fresh {:?}; {} sentinels from F",
+                g.f, g.f_age, g.b_tick, g.b_age, g.fresh, g.sentinels
+            );
+            assert!(
+                g.fresh.is_none() && g.sentinels >= 1,
+                "the client refuses every delta after the gap"
+            );
+        }
+    }
+
+    /// e11 (§4.11 :398-400): a bitten instance and a clean one on parallel threads in one process: the bitten one shows
+    /// its defect, the clean one none, its result equal to the same run alone.
+    #[test]
+    fn e11_a_bitten_and_a_clean_instance_run_side_by_side_on_two_threads() {
+        let gap = ring_gap_ns(npt_sim::TICK_HZ);
+        let alone = gap_case(clean_room(), Align::AfterSend, gap);
+        let a = std::thread::spawn(move || gap_case(bitten(), Align::AfterSend, gap));
+        let b = std::thread::spawn(move || gap_case(clean_room(), Align::AfterSend, gap));
+        let (a, b) = (
+            a.join().expect("bitten thread"),
+            b.join().expect("clean thread"),
+        );
+        assert!(
+            a.fresh.is_none() && a.f_age != Some(0),
+            "the bitten instance shows its defect"
+        );
+        assert!(
+            b.fresh.is_some() && b.f_age == Some(0),
+            "the clean instance shows none"
+        );
+        assert_eq!(b, alone, "the clean result equals the same run alone");
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/loopback.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/loopback.rs
new file mode 100644
index 0000000..ef6267a
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/loopback.rs
@@ -0,0 +1,292 @@
+//! U4e's Loopback tests (u4-build-plan.md §5 U4e): the room, the pump and the real client endpoint end to end on an
+//! injected clock, in npt-server's own tests (the advisor's condition 10 (b)).
+
+mod common;
+
+use common::{Lab, endpoints, recorded, room};
+use npt_room::ClientCounters;
+use npt_sim::Scene;
+
+/// Every client's counters, summed field by field.
+fn totals(lab: &Lab) -> ClientCounters {
+    let mut t = ClientCounters::default();
+    for k in lab
+        .conns
+        .iter()
+        .filter_map(|c| c.slot.and_then(|s| lab.room.client(s)))
+        .map(|c| c.counters())
+    {
+        t.snapshots += k.snapshots;
+        t.groups += k.groups;
+        t.split_ticks += k.split_ticks;
+        t.defaults_coded += k.defaults_coded;
+        t.skipped += k.skipped;
+        t.budget_short += k.budget_short;
+        t.refused_out_of_range += k.refused_out_of_range;
+        t.refused_too_many += k.refused_too_many;
+        t.refused_too_large += k.refused_too_large;
+        t.refused_not_canonical += k.refused_not_canonical;
+        t.events_queued += k.events_queued;
+        t.events_sent += k.events_sent;
+        t.events_resent += k.events_resent;
+        t.cosmetic_sent += k.cosmetic_sent;
+        t.cosmetic_skipped += k.cosmetic_skipped;
+        t.commands_buffered += k.commands_buffered;
+        t.commands_run += k.commands_run;
+        t.commands_rerun += k.commands_rerun;
+        t.late += k.late;
+        t.rewinds_refused += k.rewinds_refused;
+    }
+    t
+}
+
+/// e7 (u4-build-plan.md :719; §4.6): the mirror equals the client's hash over Loopback at P0 for 1,000 ticks, every
+/// pair equal, none the sentinel, for all 48 clients of the stress scene (SS: the scene every judged -17 run carries,
+/// its 384 effect rows and 300 trajectories included), the clients' commands npt-sim's synthetic recording's (seed 1).
+/// The parts the mirror models, counted: entities the priority accumulator held at B's value, gameplay events carried,
+/// snapshots coded against a baseline. On zero-delay Loopback a client's acknowledgement of tick t always reaches the
+/// room before tick t + 1 is coded, so no event is resent and, at the window's budget, no tick splits (one group holds
+/// up to 1,000 B of the about 1,067 B: the advisor's A2); both counts are printed, and the gap test (gap.rs) compares the
+/// mirror over resent events, e2 over a forced split.
+#[test]
+fn e7_the_mirror_equals_the_clients_hash_over_loopback_at_p0_for_1_000_ticks() {
+    let scene = Scene::stress();
+    let (seed, first) = (1, 1_000);
+    let ticks = 1_010;
+    let script = recorded(&scene, seed, first, ticks + 10);
+    let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
+    let mut lab = Lab::new(
+        room(&scene, seed, first, 48),
+        endpoints(48),
+        &offsets,
+        script,
+    );
+    let mut nonzero_age = 0u64;
+    lab.run(u64::from(ticks), |lab| {
+        for c in &lab.conns {
+            if let Some(age) = c
+                .slot
+                .and_then(|s| lab.room.client(s))
+                .and_then(|c| c.baseline_age())
+                && age > 0
+            {
+                nonzero_age += 1;
+            }
+        }
+    });
+    let (compared, bad) = lab.compare();
+    let per_client: Vec<usize> = (0..48u8)
+        .map(|s| lab.client.keys().filter(|k| k.0 == s).count())
+        .collect();
+    let t = totals(&lab);
+    println!(
+        "e7: {compared} pairs compared, {bad} differ or are the sentinel; per client at least {}; split ticks {}, \
+         skipped {}, events carried {}, resent {}, non-default baselines {nonzero_age}; {t:?}",
+        per_client.iter().min().unwrap_or(&0),
+        t.split_ticks,
+        t.skipped,
+        t.events_sent,
+        t.events_resent,
+    );
+    assert_eq!(bad, 0);
+    assert!(per_client.iter().all(|n| *n >= 1_000), "{per_client:?}");
+    assert!(
+        t.skipped >= 1,
+        "an entity held at B's value by the priority accumulator"
+    );
+    assert!(t.events_sent >= 1, "a gameplay event carried");
+    assert!(nonzero_age >= 1, "a snapshot coded against a baseline");
+    assert!(t.commands_run >= 1, "the clients' commands ran");
+}
+
+/// e2 (u4-build-plan.md :716-717): a forced group split (each session's link takes datagrams of at most 300 B, so the
+/// room's groups are bounded by it): ticks go out as several groups, each decoding alone against the one baseline, and
+/// the mirror equals the client's hash on every split tick (the advisor's A2: e7's split part). One group of one split
+/// tick dropped: the other groups still decode and are acknowledged, the tick is never filed and is resolved lost (e3).
+#[test]
+fn e2_a_forced_split_decodes_group_by_group_and_the_mirror_equals_the_clients_hash() {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 2, 300, 80);
+    let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
+    let mut lab = common::Lab::sized(
+        room(&scene, 2, 300, 48),
+        endpoints(48),
+        &offsets,
+        script,
+        300,
+    );
+    let drop_at = 330u32;
+    lab.tamper = Some(Box::new(move |slot, tick, group| {
+        slot == 0 && tick == drop_at && group == 1
+    }));
+    let mut split: Vec<(u8, u32)> = Vec::new();
+    lab.run(65, |lab| {
+        for s in 0..48 {
+            if lab.room.client(s).is_some_and(|c| c.groups() > 1) {
+                split.push((s, lab.room.last_tick()));
+            }
+        }
+    });
+    let mut compared_splits = 0;
+    for k in split.iter().filter(|k| !(k.0 == 0 && k.1 == drop_at)) {
+        let c = lab.client.get(k);
+        assert!(
+            c.is_some_and(|h| *h != 0) && c == lab.server.get(k),
+            "{k:?}"
+        );
+        compared_splits += 1;
+    }
+    println!(
+        "e2: {} split ticks, {compared_splits} compared equal",
+        split.len()
+    );
+    assert!(compared_splits >= 10, "{split:?}");
+    assert!(split.contains(&(0, drop_at)), "the dropped tick was split");
+    assert!(
+        !lab.client.contains_key(&(0, drop_at)),
+        "a tick with a lost group is never filed"
+    );
+    let lost = &lab.conns[0].lost;
+    assert!(
+        lost.iter().any(|&(a, n)| a <= drop_at && drop_at < a + n),
+        "{lost:?}"
+    );
+    let (compared, bad) = lab.compare();
+    assert_eq!(bad, 0, "{compared} compared");
+}
+
+/// e6 (u4-build-plan.md :718-719; §4.5 :300-302): the window counts the pump's bytes: on a tick the pump reported k
+/// pings (payload plus 28 B each), the encoder's budget is the per-tick budget less them, and the snapshot sent that
+/// tick stays within that remainder.
+#[test]
+fn e6_the_window_counts_the_pumps_bytes() {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 3, 300, 120);
+    let mut lab = Lab::new(
+        room(&scene, 3, 300, 4),
+        endpoints(4),
+        &[2_000_000; 4],
+        script,
+    );
+    let mut seen: Vec<(u32, u32, u32, u32)> = Vec::new();
+    lab.run(100, |lab| {
+        let t = lab.room.last_tick();
+        for s in 0..4u8 {
+            if let (Some(c), Some(&p)) = (lab.room.client(s), lab.pumped.get(&(s, t)))
+                && c.groups() > 0
+            {
+                let (budget, left) = c.budget();
+                seen.push((p, budget, left, c.bytes()));
+            }
+        }
+    });
+    let pinged: Vec<_> = seen.iter().filter(|s| s.0 > 0).collect();
+    println!(
+        "e6: {} snapshots, {} on ticks with pump bytes, e.g. {:?}",
+        seen.len(),
+        pinged.len(),
+        pinged.first()
+    );
+    assert!(pinged.len() >= 4, "the pump pings a few times a second");
+    for &(p, budget, left, bytes) in &seen {
+        assert_eq!(left, budget - p);
+        assert!(bytes <= left, "{bytes} B sent of {left}");
+    }
+    assert!(
+        pinged.iter().all(|s| s.0 == 10 + 28),
+        "one 10 B ping plus 28 B"
+    );
+}
+
+/// -17/tick16 and -58/tick16's unit proof through the room's encoder and the endpoint (U4E-Q4 (a); §4.14 :459): over
+/// Loopback with the room's tick past 65,535, the client files tick 65,537 under itself with the mirror's hash.
+fn tick16_case(lab: &mut Lab) -> (Option<u64>, Option<u64>) {
+    lab.run(30, |_| {});
+    let k = (0u8, 65_537u32);
+    (lab.server.get(&k).copied(), lab.client.get(&k).copied())
+}
+
+fn tick16_lab(bite: Option<&str>) -> Lab {
+    let scene = Scene::stress();
+    let script = recorded(&scene, 1, 65_520, 40);
+    let mut r = room(&scene, 1, 65_520, 2);
+    let mut e = endpoints(2);
+    #[cfg(feature = "bites")]
+    if let Some(id) = bite {
+        r = r.with_bite(id).expect("npt-proto plants it");
+        e = e
+            .into_iter()
+            .map(|c| c.with_bite(id).expect("npt-proto plants it"))
+            .collect();
+    }
+    let _ = (&mut r, &mut e, bite);
+    Lab::new(r, e, &[3_000_000; 2], script)
+}
+
+#[test]
+fn pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself() {
+    let (server, client) = tick16_case(&mut tick16_lab(None));
+    assert!(
+        server.is_some() && server == client,
+        "{server:?} {client:?}"
+    );
+}
+
+/// The bites that need every crate's list (bites builds only, under `--workspace --all-features`: npt-server's
+/// `bites` feature does not turn on npt-client's).
+#[cfg(feature = "bites")]
+mod bites {
+    use super::*;
+
+    #[test]
+    fn pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_client_hash() {
+        for id in ["PM-0001-17/tick16", "PM-0001-58/tick16"] {
+            let mut lab = tick16_lab(Some(id));
+            let (server, client) = tick16_case(&mut lab);
+            println!(
+                "{id}: the server's hash at 65,537 {server:?}, the client's {client:?}; client 0 dropped {} stale",
+                lab.conns[0].endpoint.stale()
+            );
+            assert!(server.is_some() && server != client, "{id}");
+        }
+    }
+
+    /// U4E-Q13 (a): `catalogue-5/owner-to-all` is in npt-room's list and no other crate's (check.mjs's bite-home grep
+    /// reads only the `PM-<ADR>-<row>/<name>` form, so this test holds its one home); no id is in two lists.
+    #[test]
+    fn catalogue_5_owner_to_all_lives_in_npt_room_alone_and_no_id_is_in_two_lists() {
+        let id = "catalogue-5/owner-to-all";
+        let lists: [(&str, Vec<&str>); 6] = [
+            ("npt_room::BITES", npt_room::BITES.to_vec()),
+            ("npt_proto::BITES", npt_proto::BITES.to_vec()),
+            ("npt_sim::World::BITES", npt_sim::World::BITES.to_vec()),
+            (
+                "npt_sim::OWN_COPY_SNAP_BITES",
+                npt_sim::OWN_COPY_SNAP_BITES.to_vec(),
+            ),
+            ("npt_server::BITES", npt_server::BITES.to_vec()),
+            ("npt_client::BITES", npt_client::BITES.to_vec()),
+        ];
+        let homes: Vec<&str> = lists
+            .iter()
+            .filter(|l| l.1.contains(&id))
+            .map(|l| l.0)
+            .collect();
+        assert_eq!(homes, ["npt_room::BITES"]);
+        let mut all: Vec<&str> = lists
+            .iter()
+            .filter(|l| l.0 != "npt_sim::World::BITES")
+            .flat_map(|l| l.1.clone())
+            .collect();
+        // World::BITES lists the own-copy snap's id beside OWN_COPY_SNAP_BITES (world.rs:605): one site, two lists.
+        all.extend(
+            npt_sim::World::BITES
+                .iter()
+                .filter(|b| !npt_sim::OWN_COPY_SNAP_BITES.contains(b)),
+        );
+        let n = all.len();
+        all.sort_unstable();
+        all.dedup();
+        assert_eq!(all.len(), n, "an id in two lists");
+    }
+}
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/pump.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/pump.rs
new file mode 100644
index 0000000..295ad2b
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/server/tests/pump.rs
@@ -0,0 +1,340 @@
+//! U4e's pump tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.2, e9, e10): `Decoder::pump` over a Loopback session on a
+//! fake clock, every time in ns; the room joins where a test needs its half of the `Join` checks.
+
+mod common;
+
+use common::{CONTENT, VERSION, bind, join, token};
+use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
+use npt_proto::{
+    Direction, Input, Join, JoinRejected, Kick, Message, Ping, Pong, Quant, Resync, decode, encode,
+};
+use npt_server::{ConnLimits, Decoder};
+use npt_sim::{Scene, World};
+
+fn bytes(m: &Message) -> Vec<u8> {
+    let mut out = Vec::new();
+    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
+    out
+}
+
+fn decoder(quant: Quant) -> Decoder {
+    Decoder::new(quant, 30, 0, VERSION, CONTENT)
+}
+
+/// Everything the client end has received by `now`, decoded.
+fn received(c: &mut Loopback, now: u64) -> Vec<Message> {
+    let mut out = Vec::new();
+    while let Some(r) = c.poll_recv(now) {
+        out.push(
+            decode(Direction::RoomToClient, r.channel, &r.bytes)
+                .expect("the room's messages decode"),
+        );
+    }
+    out
+}
+
+/// A session whose `Join` the pump took and passed on (the room's answer stood in for by `accept`).
+fn joined(quant: Quant) -> (Loopback, Loopback, Decoder) {
+    let (mut s, mut c) = Loopback::pair(1_200);
+    let mut d = decoder(quant);
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    assert!(matches!(out.as_slice(), [(Message::Join(_), 0)]));
+    d.accept();
+    (s, c, d)
+}
+
+fn input() -> Vec<u8> {
+    bytes(&Message::Input(Input::default()))
+}
+
+/// e8 (§4.11 :401-402): the `Decoder` is `Send` (it holds no `Link`).
+#[test]
+fn e8_the_decoder_is_send() {
+    fn send<T: Send>() {}
+    send::<Decoder>();
+}
+
+/// e9: each `JoinRejected` reason, end to end on Loopback (one table test naming the failing reason): the pump's
+/// version and content hash, the room's token, bind and roster.
+#[test]
+fn e9_each_join_rejected_reason_reaches_the_client() {
+    let scene = Scene::course();
+    let world = World::new(&scene, 1, 0);
+    let greybox = world.greybox().clone();
+    let mut room = npt_room::Room::new(world, &scene, &greybox, vec![token(0), token(1)], 1, 30);
+    // Slot 0 joins and leaves, binding its token; then joins again and stays (the roster's N is 1).
+    assert_eq!(room.join(&join(0)).map(|j| j.0), Ok(0));
+    room.leave(0);
+    assert_eq!(room.join(&join(0)).map(|j| j.0), Ok(0));
+    let table = [
+        (
+            "wrong version",
+            Join {
+                version: VERSION + 1,
+                ..join(1)
+            },
+            JoinRejected::WrongVersion,
+        ),
+        (
+            "wrong content hash",
+            Join {
+                content_hash: [8; 32],
+                ..join(1)
+            },
+            JoinRejected::WrongContentHash,
+        ),
+        (
+            "bad token",
+            Join {
+                token: [0xee; 16],
+                ..join(1)
+            },
+            JoinRejected::BadToken,
+        ),
+        (
+            "bind mismatch",
+            Join {
+                bind: bind(9),
+                ..join(0)
+            },
+            JoinRejected::BindMismatch,
+        ),
+        ("roster full", join(1), JoinRejected::RosterFull),
+    ];
+    for (name, j, want) in table {
+        let (mut s, mut c) = Loopback::pair(1_200);
+        let mut d = decoder(Quant::default());
+        c.send_reliable(Channel::Control, &bytes(&Message::Join(j)), 5);
+        let mut out = Vec::new();
+        d.pump(&mut s, 5, 0, &mut out);
+        for (m, _) in out {
+            if let Message::Join(j) = m {
+                match room.join(&j) {
+                    Ok(_) => panic!("{name}: admitted"),
+                    Err(r) => d.reject(&mut s, r, 5),
+                }
+            }
+        }
+        assert!(d.is_closed(), "{name}: the session closes");
+        assert_eq!(received(&mut c, 5), [Message::JoinRejected(want)], "{name}");
+    }
+    assert_eq!(room.roster().live(), 1, "only slot 0's first session");
+}
+
+/// e9: a datagram before `Join` is dropped and counted (ADR 0001:446).
+#[test]
+fn e9_a_datagram_before_join_is_dropped_and_counted() {
+    let (mut s, mut c) = Loopback::pair(1_200);
+    let mut d = decoder(Quant::default());
+    c.send_unreliable(&input(), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    assert!(out.is_empty());
+    assert_eq!(d.counters().before_join, 1);
+    assert!(!d.is_closed());
+}
+
+/// e9: a control message before `Join` closes the session as malformed, counted under its reason, never INVALID (the
+/// run verdict is the judge's): the reason's count, `Kick(malformed)`, and a roster that admitted no session.
+#[test]
+fn e9_a_control_message_before_join_closes_the_session_as_malformed() {
+    let scene = Scene::course();
+    let world = World::new(&scene, 1, 0);
+    let greybox = world.greybox().clone();
+    let room = npt_room::Room::new(world, &scene, &greybox, vec![token(0)], 1, 30);
+    let (mut s, mut c) = Loopback::pair(1_200);
+    let mut d = decoder(Quant::default());
+    c.send_reliable(Channel::Control, &bytes(&Message::Resync(Resync)), 0);
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    assert!(out.is_empty() && d.is_closed());
+    assert_eq!(d.counters().first_not_join, 1);
+    assert_eq!(received(&mut c, 0), [Message::Kick(Kick::Malformed)]);
+    assert_eq!(room.roster().live(), 0, "the roster admitted no session");
+}
+
+/// e9 and e10: the Input bucket (8, refilled at 2 × the tick rate a second) on a fake clock, either side of a refill
+/// instant (10^9 / 60 ns is not a whole number), and its rate pinned at 30 and 60 Hz.
+#[test]
+fn e9_e10_the_input_bucket_holds_8_and_refills_at_2_times_the_tick_rate() {
+    for (hz, per_second) in [(30, 60), (60, 120)] {
+        assert_eq!(ConnLimits::new(hz, 0).input_refill_per_second(), per_second);
+    }
+    assert_eq!(
+        ConnLimits::new(npt_sim::TICK_HZ, 0).input_refill_per_second(),
+        2 * u64::from(npt_sim::TICK_HZ)
+    );
+    let (mut s, mut c, mut d) = joined(Quant::default());
+    let mut out = Vec::new();
+    let mut at = |now: u64, n: usize, out: &mut Vec<(Message, u64)>| {
+        out.clear();
+        for _ in 0..n {
+            c.send_unreliable(&input(), now);
+        }
+        d.pump(&mut s, now, 0, out);
+        (out.len(), d.counters().input_limited)
+    };
+    assert_eq!(at(0, 9, &mut out), (8, 1), "8 taken, the ninth refused");
+    // The next token comes at 10^9 / 60 = 16,666,666.67 ns.
+    assert_eq!(
+        at(16_666_666, 1, &mut out),
+        (0, 2),
+        "1 ns before the refill"
+    );
+    assert_eq!(at(16_666_667, 1, &mut out), (1, 2), "1 ns after it");
+}
+
+/// e9: the Ping limit (10 a second per session) on a fake clock, either side of a refill instant (10^8 ns).
+#[test]
+fn e9_the_ping_limit_answers_10_a_second() {
+    let (mut s, mut c, mut d) = joined(Quant::default());
+    let ping = |id| bytes(&Message::Ping(Ping { id, sent_ns: 0 }));
+    let mut out = Vec::new();
+    let mut pongs = |now: u64, n: u8| {
+        for id in 0..n {
+            c.send_unreliable(&ping(id), now);
+        }
+        d.pump(&mut s, now, 0, &mut out);
+        let got = received(&mut c, now)
+            .into_iter()
+            .filter(|m| matches!(m, Message::Pong(_)))
+            .count();
+        (got, d.counters().ping_limited)
+    };
+    assert_eq!(pongs(0, 11), (10, 1));
+    assert_eq!(pongs(100_000_000 - 1, 1), (0, 2), "1 ns before the refill");
+    assert_eq!(pongs(100_000_000 + 1, 1), (1, 2), "1 ns after it");
+}
+
+/// e9: the 2 s Join deadline on a fake clock.
+#[test]
+fn e9_a_session_with_no_join_within_2_s_is_closed() {
+    let (mut s, _c) = Loopback::pair(1_200);
+    let mut d = decoder(Quant::default());
+    let mut out = Vec::new();
+    d.pump(&mut s, 1_999_999_999, 0, &mut out);
+    assert!(!d.is_closed());
+    d.pump(&mut s, 2_000_000_000, 0, &mut out);
+    assert!(d.is_closed());
+    assert_eq!(d.counters().join_deadline, 1);
+}
+
+/// A Loopback end whose link reports no RTT, so the pump's own ping samples decide (a `ws` session's case).
+struct NoRtt(Loopback);
+
+impl Link for NoRtt {
+    fn send_unreliable(&mut self, b: &[u8], now: u64) -> SendResult {
+        self.0.send_unreliable(b, now)
+    }
+    fn send_reliable(&mut self, ch: Channel, b: &[u8], now: u64) -> SendResult {
+        self.0.send_reliable(ch, b, now)
+    }
+    fn poll_recv(&mut self, now: u64) -> Option<Recv> {
+        self.0.poll_recv(now)
+    }
+    fn state(&self) -> LinkState {
+        self.0.state()
+    }
+    fn rtt_ns(&self) -> Option<u64> {
+        None
+    }
+    fn max_datagram_size(&self) -> usize {
+        self.0.max_datagram_size()
+    }
+    fn unsent_unreliable(&self) -> usize {
+        0
+    }
+}
+
+/// e10 and 2.2: the `Pong`'s hold time counts up to one tick period (pinned at 30 and 60 Hz), the pump sends a few
+/// pings a second and counts their bytes, a `Pong` naming no outstanding ping is dropped and counted (plan silent:
+/// built and tested here), and the RTT is the link's, else the lowest ping sample.
+#[test]
+fn e10_the_hold_time_counts_one_tick_period_and_unknown_pongs_are_dropped() {
+    assert_eq!(ConnLimits::new(30, 0).hold_cap_ns(), 33_333_333);
+    assert_eq!(ConnLimits::new(60, 0).hold_cap_ns(), 16_666_666);
+    let (s, mut c) = Loopback::pair(1_200);
+    let mut s = NoRtt(s);
+    let mut d = decoder(Quant::default());
+    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
+    let mut out = Vec::new();
+    d.pump(&mut s, 0, 0, &mut out);
+    d.accept();
+    d.pump(&mut s, 1_000, 7, &mut out);
+    assert_eq!(d.counters().pings_sent, 1);
+    assert_eq!(d.take_sent_bytes(), 10 + 28, "a ping's 10 B plus 28");
+    let Some(Message::Ping(p)) = received(&mut c, 1_000).pop() else {
+        panic!("a ping");
+    };
+    let at = 1_000 + 100_000_000;
+    let pong = |id, hold_us| {
+        bytes(&Message::Pong(Pong {
+            id,
+            echo_ns: p.sent_ns,
+            tick: 0,
+            hold_us,
+        }))
+    };
+    c.send_unreliable(&pong(p.id.wrapping_add(77), 0), at);
+    c.send_unreliable(&pong(p.id, 50_000), at);
+    d.pump(&mut s, at, 7, &mut out);
+    assert_eq!(d.counters().pong_unknown, 1);
+    // 100 ms less the hold time counted up to 33.33 ms (it claimed 50 ms).
+    assert_eq!(d.rtt_ns(&s), Some(100_000_000 - 33_333_333));
+    let (s2, _c2) = Loopback::pair(1_200);
+    assert_eq!(
+        d.rtt_ns(&s2),
+        Some(0),
+        "Loopback's own RTT for an L1 or wt session"
+    );
+}
+
+/// The socket checks through the pump (A26; -21's two decode sites): an `Input` with a byte after its last field is
+/// refused and counted as trailing bytes, one on the control stream as the wrong channel (refused and counted, never
+/// kicked: §4.10); the session stays open and the room gets neither.
+fn socket_case(quant: Quant) -> (u64, u64, usize) {
+    let (mut s, mut c, mut d) = joined(quant);
+    let mut trailing = input();
+    trailing.push(0);
+    c.send_unreliable(&trailing, 1);
+    // An `Input` framed as a control-stream message: its 2 B length, then its type and body.
+    let body = input();
+    let mut framed = (body.len() as u16).to_le_bytes().to_vec();
+    framed.extend_from_slice(&body);
+    c.send_reliable(Channel::Control, &framed, 1);
+    let mut out = Vec::new();
+    d.pump(&mut s, 1, 0, &mut out);
+    assert!(!d.is_closed());
+    (
+        d.counters().trailing_bytes,
+        d.counters().wrong_channel,
+        out.len(),
+    )
+}
+
+#[test]
+fn pm_0001_21_the_pump_counts_trailing_bytes_and_the_wrong_channel() {
+    assert_eq!(socket_case(Quant::default()), (1, 1, 0));
+}
+
+/// The pump's two npt-proto bites (bites builds only; their ids are npt-proto's, chosen through `Quant::with_bite`):
+/// the clean test's counts stay 0 under each.
+#[cfg(feature = "bites")]
+mod bites {
+    use super::*;
+
+    #[test]
+    fn pm_0001_21_no_trailing_leaves_the_pumps_trailing_count_at_0() {
+        let q = Quant::with_bite("PM-0001-21/no-trailing").expect("npt-proto plants it");
+        assert_eq!(socket_case(q).0, 0);
+    }
+
+    #[test]
+    fn pm_0001_21_no_channel_leaves_the_pumps_wrong_channel_count_at_0() {
+        let q = Quant::with_bite("PM-0001-21/no-channel").expect("npt-proto plants it");
+        assert_eq!(socket_case(q).1, 0);
+    }
+}

# npt-client's tests (crates/client/tests): git diff --no-index .gate/u4e-prefix/spikes/netcode-proof/crates/client/tests .claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests
diff --git a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/tests/bites.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/bites.rs
index cb19fdd..3540ec8 100644
--- a/.gate/u4e-prefix/spikes/netcode-proof/crates/client/tests/bites.rs
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/bites.rs
@@ -7,7 +7,11 @@ use npt_sim::grid::{Config, POSITION_XY};
 #[test]
 fn pm_0001_17_client_rounding_is_planted_here_alone_and_moves_about_half_of_all_positions() {
     let id = "PM-0001-17/client-rounding";
-    assert_eq!(npt_client::BITES, [id]);
+    // U4e's endpoint adds its two rebuilds' ids (carry 24); the grid's defect stays -17's alone.
+    assert_eq!(
+        npt_client::BITES,
+        [id, "PM-0001-58/lt-tick-offset", "PM-0001-58/lt-event-seq"]
+    );
     let bitten = npt_client::grid_with_bite(id).expect("the client plants -17's");
     assert_eq!(bitten, Config::floor_rounding());
     // No id of another site: the own-copy snap's (-41), the encoder's (-57), an unknown one.
diff --git a/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/endpoint.rs b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/endpoint.rs
new file mode 100644
index 0000000..60b5b21
--- /dev/null
+++ b/.claude/worktrees/wf_0fa6595a-3e2-1/spikes/netcode-proof/crates/client/tests/endpoint.rs
@@ -0,0 +1,314 @@
+//! U4e's endpoint tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.3, 2.4): crafted room messages fed to a real
+//! `ClientEndpoint`, each encoded with npt-proto as the room sends it, and what the endpoint files read back.
+
+use npt_client::{ClientEndpoint, StepOut};
+use npt_link::Channel;
+use npt_proto::{
+    EntityDelta, FullState, Join, JoinAccepted, Limits, Message, Snapshot, client_hash, encode,
+};
+use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};
+
+fn bytes(m: &Message) -> Vec<u8> {
+    let mut out = Vec::new();
+    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
+    out
+}
+
+fn soldier(index: u16, x: i32) -> EntityRecord {
+    let mut grid = [0; EntityRecord::FIELDS];
+    grid[0] = x;
+    grid[1] = -x;
+    grid[2] = 150;
+    grid[6] = 1_000;
+    EntityRecord {
+        kind: EntityKind::Soldier,
+        index,
+        generation: 0,
+        area: 0,
+        grid,
+        raw: [0; EntityRecord::FIELDS],
+    }
+}
+
+/// A joined endpoint (slot 0) holding the full state of tick `t`: soldiers 1 and 2 at x = 100 and 200, its own soldier.
+fn joined(e: ClientEndpoint, t: u32) -> ClientEndpoint {
+    let mut e = e;
+    let accepted = bytes(&Message::JoinAccepted(JoinAccepted {
+        slot: 0,
+        tick: t,
+        ..JoinAccepted::default()
+    }));
+    let full = bytes(&Message::FullState(FullState {
+        tick: t,
+        last_run: None,
+        own: vec![soldier(0, 0)],
+        entities: vec![soldier(1, 100), soldier(2, 200)],
+    }));
+    e.input(
+        None,
+        &[
+            (Channel::Control, accepted, 0),
+            (Channel::FullState, full, 0),
+        ],
+        0,
+    );
+    let out = e.step();
+    assert_eq!(out.hashes.len(), 1, "the full state's tick is filed");
+    e
+}
+
+fn endpoint() -> ClientEndpoint {
+    ClientEndpoint::new(Join::default(), 30)
+}
+
+/// A snapshot group of tick `t` against the baseline `age` ticks back: soldier 1 moved to `x`.
+fn moved(t: u32, age: u16, group: u8, groups: u8, x: i32) -> Snapshot {
+    let base = soldier(1, 100);
+    Snapshot {
+        tick: t,
+        baseline_age: age,
+        group,
+        groups,
+        entities: vec![EntityDelta::between(&soldier(1, x), Some(&base)).expect("a change")],
+        ..Snapshot::default()
+    }
+}
+
+fn datagram(s: Snapshot) -> (Channel, Vec<u8>, u64) {
+    (Channel::Datagram, bytes(&Message::Snapshot(s)), 0)
+}
+
+fn feed(e: &mut ClientEndpoint, msgs: Vec<(Channel, Vec<u8>, u64)>) -> StepOut {
+    e.input(None, &msgs, 0);
+    e.step()
+}
+
+/// What a client holding `entities` and its own soldier hashes at `t` with no events.
+fn hash_of(t: u32, entities: &[EntityRecord]) -> u64 {
+    client_hash(t, entities, &[soldier(0, 0)], &[])
+}
+
+/// e8 (§4.11 :401-402): `ClientEndpoint` is `Send`.
+#[test]
+fn e8_the_client_endpoint_is_send() {
+    fn send<T: Send>() {}
+    send::<ClientEndpoint>();
+}
+
+/// e9 (catalogue item 2's "on both sides"; nit d): a message on a channel its type does not travel on is dropped and
+/// counted, never filed; a datagram above 1,150 B is refused by its size (the sentinel under its header's tick).
+#[test]
+fn e9_the_endpoint_drops_a_wrong_channel_message_and_refuses_a_datagram_above_1_150_b() {
+    let mut e = joined(endpoint(), 100);
+    let s = bytes(&Message::Snapshot(moved(101, 1, 0, 1, 150)));
+    // The snapshot framed as a control-stream message: its 2 B length, then its type and body.
+    let mut framed = (s.len() as u16).to_le_bytes().to_vec();
+    framed.extend_from_slice(&s);
+    let out = feed(&mut e, vec![(Channel::Control, framed, 0)]);
+    assert_eq!((e.wrong_channel(), out.hashes.len()), (1, 0));
+    let mut big = s;
+    big.resize(Limits::SNAPSHOT_RECEIVE + 1, 0);
+    big[1..5].copy_from_slice(&102u32.to_le_bytes());
+    let out = feed(&mut e, vec![(Channel::Datagram, big, 0)]);
+    assert_eq!(out.hashes, [(102, 0)], "refused by its size: the sentinel");
+    assert_eq!(e.refused(), 1);
+}
+
+/// e3 (u4-build-plan.md :717-718): a lost group leaves its tick unlogged (and resolved lost, counted, once it has left
+/// the frame ring); a refused group logs the sentinel.
+#[test]
+fn e3_a_lost_group_leaves_its_tick_unlogged_and_a_refused_one_logs_the_sentinel() {
+    let mut e = joined(endpoint(), 100);
+    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 2, 150))]);
+    assert!(
+        out.hashes.is_empty(),
+        "one group of two: not complete, nothing filed"
+    );
+    // A group naming a baseline the client does not hold (tick 90): refused, the sentinel.
+    let out = feed(&mut e, vec![datagram(moved(102, 12, 0, 1, 150))]);
+    assert_eq!(out.hashes, [(102, 0)]);
+    // Corrupt bytes after the last field: refused by the decoder, the sentinel under the header's tick.
+    let mut b = bytes(&Message::Snapshot(moved(103, 3, 0, 1, 150)));
+    b.push(0);
+    let out = feed(&mut e, vec![(Channel::Datagram, b, 0)]);
+    assert_eq!(out.hashes, [(103, 0)]);
+    // Tick 101 is resolved lost once it has left the ring, never filed.
+    let len = npt_sim::ACK_RING_TICKS;
+    let last = 101 + len;
+    let fresh = Snapshot {
+        tick: last,
+        entities: vec![EntityDelta::Add(soldier(1, 100))],
+        ..Snapshot::default()
+    };
+    let out = feed(&mut e, vec![datagram(fresh)]);
+    assert!(
+        out.lost.iter().any(|&(a, n)| a <= 101 && 101 < a + n),
+        "{:?}",
+        out.lost
+    );
+    assert!(!out.hashes.iter().any(|h| h.0 == 101));
+}
+
+/// Carry 61: tick t + 1's snapshot fed before tick t's (as L1's reorder step delivers them) files t under t with its
+/// hash (the client decodes a stale group: the advisor's lean), never the sentinel; one datagram received twice files
+/// one hash; a tick with a group missing is not resolved lost while younger than `npt_sim::ACK_RING_TICKS` ticks (by
+/// the newest tick held), and is once it is that old.
+#[test]
+fn carry_61_reordered_and_duplicated_datagrams_file_one_hash_under_each_tick() {
+    let mut e = joined(endpoint(), 100);
+    let later = moved(102, 2, 0, 1, 170);
+    let earlier = moved(101, 1, 0, 1, 160);
+    let out = feed(
+        &mut e,
+        vec![datagram(later.clone()), datagram(earlier.clone())],
+    );
+    let want = [
+        (102, hash_of(102, &[soldier(1, 170), soldier(2, 200)])),
+        (101, hash_of(101, &[soldier(1, 160), soldier(2, 200)])),
+    ];
+    assert_eq!(
+        out.hashes, want,
+        "each under its own tick, neither the sentinel"
+    );
+    let out = feed(&mut e, vec![datagram(earlier)]);
+    assert!(out.hashes.is_empty(), "a duplicate is filed once");
+    assert_eq!(e.duplicates(), 1);
+    // Tick 103 of two groups, one lost: not resolved while younger than the ring, resolved once that old.
+    feed(&mut e, vec![datagram(moved(103, 3, 0, 2, 180))]);
+    let len = npt_sim::ACK_RING_TICKS;
+    let at = |t: u32| Snapshot {
+        tick: t,
+        entities: vec![EntityDelta::Add(soldier(1, 100))],
+        ..Snapshot::default()
+    };
+    let mut lost = Vec::new();
+    for t in 104..103 + len {
+        lost.extend(feed(&mut e, vec![datagram(at(t))]).lost);
+    }
+    assert!(
+        !lost.iter().any(|&(a, n)| a <= 103 && 103 < a + n),
+        "103 is {} old: {lost:?}",
+        len - 1
+    );
+    let out = feed(&mut e, vec![datagram(at(103 + len))]);
+    assert!(
+        out.lost.iter().any(|&(a, n)| a <= 103 && 103 < a + n),
+        "{:?}",
+        out.lost
+    );
+    assert!(
+        !out.hashes.iter().any(|h| h.0 == 103 && h.1 == 0),
+        "lost, never the sentinel"
+    );
+}
+
+/// The endpoint after stamping `n` commands from a counter started at `start`, at newest tick 100.
+fn commanding(e: ClientEndpoint, start: u32, n: u32) -> ClientEndpoint {
+    let mut e = joined(e.with_starts(start, 0), 100);
+    for _ in 0..n {
+        e.input(Some(Command::default()), &[], 0);
+    }
+    e.step();
+    e
+}
+
+/// The last-run rebuild the endpoint makes of a header naming command `seq` (16 bits), after 8 commands from 65,530.
+fn last_run_case(e: ClientEndpoint, seq: u16) -> (Option<u32>, Vec<&'static str>) {
+    let mut e = commanding(e, 65_530, 8);
+    let s = Snapshot {
+        last_run: Some(seq),
+        ..moved(101, 1, 0, 1, 150)
+    };
+    let out = feed(&mut e, vec![datagram(s)]);
+    let s = Snapshot {
+        last_run: Some(seq.wrapping_add(2)),
+        ..moved(102, 2, 0, 1, 150)
+    };
+    let mut wraps = out.wraps;
+    wraps.extend(feed(&mut e, vec![datagram(s)]).wraps);
+    (e.last_run_command(), wraps)
+}
+
+/// -58's last-run rebuild in the clean build (U4E-Q2 (c)): the header's 16-bit last-run sequence names the client's own
+/// command across the wrap, and that counter's wrap is exposed once by name (W1-N2).
+#[test]
+fn pm_0001_58_the_last_run_rebuild_names_the_right_command_across_the_wrap() {
+    let (last, wraps) = last_run_case(endpoint(), 65_535);
+    assert_eq!(last, Some(65_537), "65,535 then 65,537 (low bits 1)");
+    assert_eq!(wraps, ["tick_offset"]);
+}
+
+/// The hash the endpoint files at tick 102 for events numbered 65,533 to 65,537 (two snapshots), with its event
+/// counter's start at `event_start`, and the hash a client that numbered them right would file.
+fn event_case(e: ClientEndpoint) -> (u64, u64, Vec<&'static str>) {
+    let mut e = joined(e.with_starts(0, 65_533), 100);
+    let ev = SimEvent::Kill {
+        killer: 1,
+        victim: 2,
+    };
+    let s1 = Snapshot {
+        events: [65_533u32, 65_534, 65_535]
+            .iter()
+            .map(|n| (*n as u16, 0, ev))
+            .collect(),
+        ..moved(101, 1, 0, 1, 150)
+    };
+    let s2 = Snapshot {
+        events: [65_536u32, 65_537]
+            .iter()
+            .map(|n| (*n as u16, 0, ev))
+            .collect(),
+        ..moved(102, 2, 0, 1, 150)
+    };
+    let out = feed(&mut e, vec![datagram(s1), datagram(s2)]);
+    let got = out.hashes.iter().find(|h| h.0 == 102).map_or(0, |h| h.1);
+    let want = client_hash(
+        102,
+        &[soldier(1, 150), soldier(2, 200)],
+        &[soldier(0, 0)],
+        &[(65_536, ev), (65_537, ev)],
+    );
+    (got, want, out.wraps)
+}
+
+/// -58's event-number rebuild in the clean build (§4.14 :470): post-wrap event numbers are rebuilt to their 32 bits
+/// (the hash reads them), and the counter's wrap is exposed once by name.
+#[test]
+fn pm_0001_58_event_numbers_are_rebuilt_across_the_16_bit_wrap() {
+    let (got, want, wraps) = event_case(endpoint());
+    assert_eq!(got, want);
+    assert_eq!(wraps, ["event_seq"]);
+}
+
+/// npt-client's planted defects (bites builds only), each in wave 1's two parts: a bites-only twin asserting the clean
+/// test's check fails on the bitten endpoint.
+#[cfg(feature = "bites")]
+mod bites {
+    use super::*;
+
+    fn bitten(id: &str) -> ClientEndpoint {
+        endpoint().with_bite(id).expect("npt-client plants it")
+    }
+
+    #[test]
+    fn pm_0001_58_lt_tick_offset_names_the_wrong_command() {
+        let (last, _) = last_run_case(bitten("PM-0001-58/lt-tick-offset"), 65_535);
+        assert_ne!(
+            last,
+            Some(65_537),
+            "the rebuilt sequence names the wrong command: {last:?}"
+        );
+    }
+
+    #[test]
+    fn pm_0001_58_lt_event_seq_rebuilds_a_post_wrap_event_wrong() {
+        let (got, want, _) = event_case(bitten("PM-0001-58/lt-event-seq"));
+        assert_ne!(got, want);
+    }
+
+    #[test]
+    fn the_endpoint_takes_npt_protos_ids_through_quant_and_no_other() {
+        assert!(endpoint().with_bite("PM-0001-17/tick16").is_some());
+        assert!(endpoint().with_bite("PM-0001-18/no-fallback").is_none());
+    }
+}
