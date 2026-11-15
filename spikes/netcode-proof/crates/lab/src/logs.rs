//! The scenario's three logs (u4-build-plan.md §4.12 :410-415; marks/README.md §1 and §4; u4r-brief.md 2.1): on L1
//! one npt-lab process writes one log per process value the checks select on, each with its own header: `room` (the
//! room's per-tick World hash `h`, `hs.server_hash`, PM-0001-2's series, BRAWL-1's `in_building_slot_area`, the
//! `admit` events, `burst_eviction`), `client` (every client's `hs.decoded_hash` and `tick.s.snap_incomplete`, one
//! entry per slot) and `lab` (-57's errors, `burst_heal`, `counter_wrap`, the run's own records). Every tick and
//! second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on
//! ticks with one: the advisor's plan review, condition 10); every log ends with its footer.
//!
//! **The client writer** (carry 62 (a) to (f); carry 73; README §1's `hs` rules): each client's hashes are filed under
//! the tick it rebuilt (a 32-bit tick, unwrapped forward from the room's first tick, so a wrong rebuild past the 16-bit
//! wrap lands after every tick the room ran and is written there, in tick order, never dropped and never `crash`); two
//! different hashes for one tick make it the sentinel; the ticks the client resolves lost are `null` with
//! `snap_incomplete` 1. A tick is written once every client admitted at or before it has resolved it (or it lies
//! before that client's first resolved tick); at the stop, inside each client's span, a tick the room logged a hash
//! for and the client left unresolved is decided lost (on L1 the runner decides it there: nothing more can arrive), and
//! the ticks after its last resolved one (in flight) are not written. -57's errors of a (client, tick) go into the lab
//! log's record of that tick only when that client filed a hash for it that is not the sentinel.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;

use npt_log::{Ended, EventRecord, Header, Phase, SCHEMA, SecondRecord, TickRecord, Writer};
use npt_room::Room;
use npt_sim::EntityKind;

use crate::mirror::{Errors, Records, SetStats};
use crate::scenario::Sim;

/// The fields every log of a run shares, and its params.
#[derive(Clone, Debug)]
pub(crate) struct Meta {
    pub(crate) run: String,
    pub(crate) set: String,
    pub(crate) role: String,
    pub(crate) marks: Vec<String>,
    pub(crate) date: String,
    pub(crate) commit: String,
    pub(crate) layer: String,
    pub(crate) profile: String,
    pub(crate) seed: u64,
    pub(crate) scene: String,
    pub(crate) clients: u32,
    pub(crate) bite: Option<String>,
    pub(crate) params: BTreeMap<String, serde_json::Value>,
}

impl Meta {
    fn header(&self, process: &str) -> Result<Header, String> {
        Ok(Header {
            schema: SCHEMA,
            run: self.run.clone(),
            set: self.set.clone(),
            role: self.role.clone(),
            marks: self.marks.clone(),
            machine: crate::machine::container(),
            build: crate::machine::build(&self.commit)?,
            bite: self.bite.clone(),
            layer: self.layer.clone(),
            profile: self.profile.clone(),
            seed: self.seed,
            tick_hz: npt_sim::TICK_HZ,
            clients: self.clients,
            scene: self.scene.clone(),
            process: process.to_owned(),
            slot: 0,
            date: self.date.clone(),
            params: self.params.clone(),
        })
    }
}

type Out = Writer<Box<dyn std::io::Write>>;

/// One log in `dir` (never overwritten), or with no folder a sink (a runner recording regenerated inside `npt-lab
/// hash` keeps no log: carry 82).
fn open(dir: Option<&std::path::Path>, process: &str, meta: &Meta) -> Result<Out, String> {
    let out: Box<dyn std::io::Write> = match dir {
        None => Box::new(std::io::sink()),
        Some(dir) => {
            let path = dir.join(format!("{process}.jsonl"));
            let file = File::create_new(&path)
                .map_err(|e| format!("{}: {e} (a log is never overwritten)", path.display()))?;
            Box::new(BufWriter::new(file))
        }
    };
    Writer::new(out, &meta.header(process)?).map_err(|e| format!("{process} log: {e}"))
}

fn io(e: std::io::Error) -> String {
    format!("writing a log: {e}")
}

/// The World's events this tick, read from its dump's `events` section (npt-sim's canonical state: `World::dump`; the
/// section is cleared at each step and written per event, kind 1 a shot, 2 a hit, 3 a kill): (events, shots, hits
/// and kills). `at` caches the section's index (the sections before it are fixed-size pools). The advisor's U4r build
/// ruling, point 6, pins it against `SimCore::step`'s returned events (the test below).
pub(crate) fn dump_events(
    world: &Sim,
    at: &mut Option<usize>,
) -> Result<(usize, i64, i64), String> {
    let dump = world.dump();
    let words = dump.words();
    let a = match *at {
        Some(a) => a,
        None => {
            let a = (0..words.len())
                .find(|&i| dump.field(i).as_deref() == Some("events[0].count"))
                .ok_or("the World's dump has no events count")?;
            *at = Some(a);
            a
        }
    };
    let events = words.get(a).copied().unwrap_or(0) as usize;
    let kind = |i: usize| words.get(a + 1 + 4 * i).copied().unwrap_or(0);
    let shots = (0..events).filter(|&i| kind(i) == 1).count() as i64;
    let reliable = (0..events)
        .filter(|&i| kind(i) == 2 || kind(i) == 3)
        .count() as i64;
    Ok((events, shots, reliable))
}

/// One (client, tick)'s resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Res {
    Open,
    Hash(u64),
    Lost,
}

/// What the room's series need from the tick before.
#[derive(Default)]
struct RoomTrack {
    doors: BTreeMap<u16, i32>,
    projectiles: BTreeMap<u16, (u16, [i32; 12])>,
    late: Vec<u64>,
    refused: Vec<[u64; 4]>,
    events_at: Option<usize>,
    second: Option<(u64, Phase, [i64; 6])>,
}

/// The run's logs (the module text).
pub(crate) struct Logs {
    room: Out,
    client: Out,
    lab: Out,
    /// Whether the logs have a folder (`open`'s `dir`): a run with none keeps no client resolution (advisor-u4r-r1,
    /// RR3-L2 (a): `filed` then keeps only what `started` and the stop read).
    logging: bool,
    n: usize,
    first_tick: u32,
    phases: BTreeMap<u64, Phase>,
    res: BTreeMap<u64, Vec<Res>>,
    /// Per slot: the tick it was admitted at, its first and last resolved ticks, and whether it left the run.
    admitted: Vec<Option<u64>>,
    first: Vec<Option<u64>>,
    last: Vec<Option<u64>>,
    gone: Vec<bool>,
    errors: BTreeMap<(u64, u8), Errors>,
    /// Per slot, the ticks the room logged a hash for it, beyond the written mark (for the stop's decision).
    logged: BTreeMap<u64, Vec<bool>>,
    written: Option<u64>,
    track: RoomTrack,
    wraps: BTreeMap<&'static str, u64>,
    /// carry 62 (h)'s bite, PM-0001-17/whole-as-lost (the advisor's U4r build ruling, point 4): on every client, its
    /// first tick resolved whole in the measured phase filed as lost (null, `snap_incomplete` 1); per slot, that tick.
    /// Bites builds only, so the measured binary holds no bite code (advisor-u4r-r1, FP-6 item 3: RR3-L4).
    #[cfg(feature = "bites")]
    whole_as_lost: bool,
    #[cfg(feature = "bites")]
    bitten_at: Vec<Option<u64>>,
    pub(crate) counts: Counts,
}

/// The run's running totals, for its summary line and the lab log's last records.
#[derive(Clone, Debug, Default)]
pub(crate) struct Counts {
    pub(crate) incomplete_measured: u64,
    pub(crate) hashes: u64,
    pub(crate) sentinels: u64,
    pub(crate) lost_decided_at_stop: u64,
    pub(crate) conflicts: u64,
    pub(crate) shots_measured: i64,
    pub(crate) measured_seconds: i64,
    pub(crate) reliable_measured: i64,
    pub(crate) everyone_measured: u64,
    pub(crate) large_measured: u64,
    pub(crate) projectiles_measured: u64,
    pub(crate) sets_measured: u64,
    /// BRAWL-1's figure: measured slot-ticks with feet in a building area, and the slots that had any.
    pub(crate) in_building_measured: u64,
    pub(crate) in_building_slots: std::collections::BTreeSet<u8>,
}

impl Logs {
    /// Opens the three logs in `dir` (each `<process>.jsonl`, never overwritten), or three sinks with no folder.
    pub(crate) fn open(
        dir: Option<&std::path::Path>,
        meta: &Meta,
        first_tick: u32,
        whole_as_lost: bool,
    ) -> Result<Logs, String> {
        let n = meta.clients as usize;
        #[cfg(not(feature = "bites"))]
        let _ = whole_as_lost;
        Ok(Logs {
            room: open(dir, "room", meta)?,
            client: open(dir, "client", meta)?,
            lab: open(dir, "lab", meta)?,
            logging: dir.is_some(),
            n,
            first_tick,
            phases: BTreeMap::new(),
            res: BTreeMap::new(),
            admitted: vec![None; n],
            first: vec![None; n],
            last: vec![None; n],
            gone: vec![false; n],
            errors: BTreeMap::new(),
            logged: BTreeMap::new(),
            written: None,
            track: RoomTrack::default(),
            wraps: BTreeMap::new(),
            #[cfg(feature = "bites")]
            whole_as_lost,
            #[cfg(feature = "bites")]
            bitten_at: vec![None; n],
            counts: Counts::default(),
        })
    }

    /// A 32-bit tick as the log's never-wrapping tick: forward from the room's first tick.
    pub(crate) fn wide(&self, t: u32) -> u64 {
        u64::from(self.first_tick) + u64::from(t.wrapping_sub(self.first_tick))
    }

    /// An event in the lab log.
    pub(crate) fn lab_event(&mut self, e: &EventRecord) -> Result<(), String> {
        self.lab.event(e).map_err(io)
    }

    /// An event in the room log.
    pub(crate) fn room_event(&mut self, e: &EventRecord) -> Result<(), String> {
        self.room.event(e).map_err(io)
    }

    /// The room admitted `slot` (its session `session`) before tick `t` ran.
    pub(crate) fn admit(
        &mut self,
        slot: u8,
        session: u64,
        t: u32,
        ph: Phase,
    ) -> Result<(), String> {
        let w = self.wide(t);
        if let Some(a) = self.admitted.get_mut(usize::from(slot)) {
            a.get_or_insert(w);
        }
        let mut e = EventRecord::new("admit", ph)
            .with("session", session)
            .with("in_roster", true)
            .with("slot", slot);
        e.t = Some(w);
        self.room.event(&e).map_err(io)
    }

    /// Whether `slot` was admitted and has resolved a tick (carry 62 (d)'s head).
    pub(crate) fn started(&self, slot: u8) -> bool {
        let s = usize::from(slot);
        self.admitted.get(s).is_some_and(Option::is_some)
            && self.first.get(s).is_some_and(Option::is_some)
    }

    /// Whether every tick up to `w` is written (every client resolved it).
    pub(crate) fn written_through(&self, w: u64) -> bool {
        self.written.is_some_and(|m| m >= w)
    }

    /// `slot`'s session ended (replaced, kicked or left): its later ticks are not waited for.
    pub(crate) fn gone(&mut self, slot: u8) {
        if let Some(g) = self.gone.get_mut(usize::from(slot)) {
            *g = true;
        }
    }

    /// A counter that wrapped (`ClientSlot::wraps`, `StepOut::wraps`): one record per counter name, at its first wrap
    /// (carry 77 (a)), in the lab log.
    pub(crate) fn wrapped(&mut self, name: &'static str, t: u32, ph: Phase) -> Result<(), String> {
        if self.wraps.contains_key(name) {
            return Ok(());
        }
        let w = self.wide(t);
        self.wraps.insert(name, w);
        let mut e = EventRecord::new("counter_wrap", ph)
            .with("counter", name)
            .with("tick_count", w);
        e.t = Some(w);
        self.lab.event(&e).map_err(io)
    }

    fn entry(&mut self, w: u64) -> &mut Vec<Res> {
        let n = self.n;
        self.res.entry(w).or_insert_with(|| vec![Res::Open; n])
    }

    fn resolved(&mut self, slot: usize, w: u64) {
        let f = &mut self.first[slot];
        if f.is_none_or(|x| w < x) {
            *f = Some(w);
        }
        let l = &mut self.last[slot];
        if l.is_none_or(|x| w > x) {
            *l = Some(w);
        }
    }

    /// The whole-as-lost bite's site (bites builds only): a whole decode (not the sentinel) of slot `s`'s first measured
    /// tick, and every later filing of that tick, goes in as lost; `w` the filing's tick, `h` its hash.
    #[cfg(feature = "bites")]
    fn bitten(&mut self, s: usize, w: u64, h: u64) -> bool {
        let as_lost = self.whole_as_lost
            && h != 0
            && match self.bitten_at[s] {
                Some(b) => b == w,
                None => self.phases.get(&w) == Some(&Phase::Measured),
            };
        if as_lost {
            self.bitten_at[s] = Some(w);
        }
        as_lost
    }

    /// What one client step filed: (tick, hash) pairs (0 the sentinel) and the runs of ticks it resolved lost. A run
    /// with no log folder only resolves each tick (its first and last resolved ticks, which `started` and the stop
    /// read) and keeps no per-tick resolution (advisor-u4r-r1, RR3-L2 (a)): nothing writes there, and with no room
    /// tick logged no tick is ever written, so neither the conflict count nor the whole-as-lost bite can act.
    pub(crate) fn filed(&mut self, slot: u8, hashes: &[(u32, u64)], lost: &[(u32, u32)]) {
        let s = usize::from(slot);
        if s >= self.n {
            return;
        }
        if !self.logging {
            for &(t, _) in hashes {
                let w = self.wide(t);
                self.resolved(s, w);
            }
            for &(first, count) in lost {
                for k in 0..count {
                    let w = self.wide(first.wrapping_add(k));
                    self.resolved(s, w);
                }
            }
            return;
        }
        for &(t, h) in hashes {
            let w = self.wide(t);
            if self.written.is_some_and(|m| w <= m) {
                // Never dropped silently: counted, and the log ends `crash` (README §1).
                self.counts.conflicts += 1;
                continue;
            }
            #[cfg(feature = "bites")]
            if self.bitten(s, w, h) {
                self.entry(w)[s] = Res::Lost;
                self.resolved(s, w);
                continue;
            }
            let e = &mut self.entry(w)[s];
            *e = match *e {
                Res::Open | Res::Lost => Res::Hash(h),
                Res::Hash(x) if x == h => Res::Hash(x),
                Res::Hash(_) => Res::Hash(0),
            };
            self.resolved(s, w);
        }
        for &(first, count) in lost {
            for k in 0..count {
                let w = self.wide(first.wrapping_add(k));
                if self.written.is_some_and(|m| w <= m) {
                    continue;
                }
                let e = &mut self.entry(w)[s];
                if *e == Res::Open {
                    *e = Res::Lost;
                }
                self.resolved(s, w);
            }
        }
    }

    /// -57's candidate errors of (slot, tick), kept until that client's resolution of the tick is written.
    pub(crate) fn errors(&mut self, slot: u8, t: u32, e: Errors) {
        if e.pos_um.is_empty() && e.yaw_microdeg.is_empty() && e.pitch_microdeg.is_empty() {
            return;
        }
        let w = self.wide(t);
        self.errors.insert((w, slot), e);
    }

    fn writable(&self, w: u64) -> bool {
        (0..self.n).all(|s| {
            let started = self.admitted[s].is_some_and(|a| a <= w);
            if !started || self.gone[s] {
                return true;
            }
            if self.first[s].is_none_or(|f| w < f) {
                // Admitted, no tick resolved yet: wait (its first resolved tick may be this one).
                return self.first[s].is_some();
            }
            self.res.get(&w).is_some_and(|r| r[s] != Res::Open)
        })
    }

    fn write_client_tick(&mut self, w: u64) -> Result<(), String> {
        let ph = self.phase(w);
        let list = self
            .res
            .remove(&w)
            .unwrap_or_else(|| vec![Res::Open; self.n]);
        self.logged.remove(&w);
        let mut errs = Errors::default();
        for (s, r) in list.iter().enumerate() {
            if let Res::Hash(h) = r
                && *h != 0
                && let Some(e) = self.errors.remove(&(w, s as u8))
            {
                errs.pos_um.extend(e.pos_um);
                errs.yaw_microdeg.extend(e.yaw_microdeg);
                errs.pitch_microdeg.extend(e.pitch_microdeg);
            }
        }
        self.errors.retain(|(t, _), _| *t > w);
        if list.iter().any(|r| *r != Res::Open) {
            let mut rec = TickRecord::new(w, ph);
            let hashes: Vec<Option<u64>> = list
                .iter()
                .map(|r| match r {
                    Res::Hash(h) => Some(*h),
                    _ => None,
                })
                .collect();
            let incomplete: Vec<i64> = list.iter().map(|r| i64::from(*r == Res::Lost)).collect();
            self.counts.hashes += hashes.iter().flatten().count() as u64;
            self.counts.sentinels += hashes.iter().flatten().filter(|h| **h == 0).count() as u64;
            if ph == Phase::Measured {
                self.counts.incomplete_measured += incomplete.iter().sum::<i64>() as u64;
            }
            rec.set_hashes("decoded_hash", &hashes);
            rec.s.insert("snap_incomplete".to_owned(), incomplete);
            self.client.tick(&rec).map_err(io)?;
        }
        if !errs.pos_um.is_empty()
            || !errs.yaw_microdeg.is_empty()
            || !errs.pitch_microdeg.is_empty()
        {
            let mut rec = TickRecord::new(w, ph);
            rec.s.insert("pos_err_um".to_owned(), errs.pos_um);
            rec.s
                .insert("yaw_err_microdeg".to_owned(), errs.yaw_microdeg);
            rec.s
                .insert("pitch_err_microdeg".to_owned(), errs.pitch_microdeg);
            self.lab.tick(&rec).map_err(io)?;
        }
        self.written = Some(w);
        Ok(())
    }

    fn phase(&self, w: u64) -> Phase {
        self.phases
            .get(&w)
            .copied()
            .or_else(|| self.phases.values().next_back().copied())
            .unwrap_or(Phase::After)
    }

    /// Writes every tick that is complete, in order.
    pub(crate) fn flush(&mut self) -> Result<(), String> {
        loop {
            let next = match self.written {
                None => self.phases.keys().next().copied(),
                Some(m) => self.phases.range(m + 1..).next().map(|(k, _)| *k),
            };
            let Some(w) = next else { return Ok(()) };
            if !self.writable(w) {
                return Ok(());
            }
            self.write_client_tick(w)?;
        }
    }

    /// The room ran tick `t` in phase `ph`: its record (the World hash, `hs.server_hash`, PM-0001-2's series, the
    /// sizes), from the room after the tick; `sets` is each slot's set as the mirror rebuilt it; `datagrams` each
    /// slot's snapshot datagrams' payload sizes; `inputs` the `Input` datagrams' sizes the clients sent since the last
    /// tick. Returns the events the World stamped at the tick.
    pub(crate) fn room_tick(
        &mut self,
        room: &Room<Sim>,
        t: u32,
        ph: Phase,
        sets: &[SetStats],
        datagrams: &[Vec<usize>],
        inputs: &[i64],
    ) -> Result<u64, String> {
        let w = self.wide(t);
        self.phases.insert(w, ph);
        let world = room.sim();
        let records = npt_sim::SimCore::entity_records(world);
        let scene = world.scene();
        let index = Records::new(records);
        let mut rec = TickRecord::new(w, ph);
        rec.set_hash(npt_sim::SimCore::state_hash(world));
        // PM-0001-2's room series.
        let count = |k: EntityKind| records.iter().filter(|r| r.kind == k).count() as i64;
        let mut teams: BTreeMap<u8, i64> = BTreeMap::new();
        for r in records.iter().filter(|r| r.kind == EntityKind::Soldier) {
            if let Some(team) = u8::try_from(r.index).ok().and_then(|s| scene.team(s)) {
                *teams.entry(team).or_default() += 1;
            }
        }
        let rows = u16::from(scene.effect_rows().max(1));
        let mut effects: BTreeMap<u16, i64> = BTreeMap::new();
        for r in records.iter().filter(|r| r.kind == EntityKind::Effect) {
            if (r.grid[1] as u32).wrapping_sub(t) as i32 > 0 {
                *effects.entry(r.index / rows).or_default() += 1;
            }
        }
        let n = self.n;
        let mut server = vec![None; n];
        let mut clients_count = 0i64;
        let mut late = 0u64;
        self.track.late.resize(n, 0);
        self.track.refused.resize(n, [0; 4]);
        let mut refused_events = Vec::new();
        for (s, h) in server.iter_mut().enumerate() {
            let Some(c) = room.client(s as u8) else {
                continue;
            };
            *h = c.server_hash();
            if h.is_some() {
                self.logged.entry(w).or_insert_with(|| vec![false; n])[s] = true;
            }
            let k = c.counters();
            if !c.kicked() && k.snapshots > 0 {
                clients_count += 1;
            }
            late += k.late.saturating_sub(self.track.late[s]);
            self.track.late[s] = k.late;
            let now = [
                k.refused_out_of_range,
                k.refused_too_many,
                k.refused_too_large,
                k.refused_not_canonical,
            ];
            for (i, kind) in ["out_of_range", "too_many", "too_large", "not_canonical"]
                .iter()
                .enumerate()
            {
                if now[i] > self.track.refused[s][i] {
                    refused_events.push((s, *kind));
                }
            }
            self.track.refused[s] = now;
        }
        rec.set_hashes("server_hash", &server);
        let v = &mut rec.v;
        v.insert("soldiers_count".into(), count(EntityKind::Soldier));
        v.insert(
            "smallest_team_count".into(),
            teams.values().copied().min().unwrap_or(0),
        );
        v.insert("snapshot_clients_count".into(), clients_count);
        v.insert("big_vehicles_count".into(), count(EntityKind::Vehicle));
        v.insert("very_large_units_count".into(), index.large_units() as i64);
        v.insert("projectiles_count".into(), count(EntityKind::Projectile));
        v.insert(
            "effects_per_soldier_max_count".into(),
            effects.values().copied().max().unwrap_or(0),
        );
        v.insert("late_count".into(), late as i64);
        let flags: Vec<i64> = (0..n)
            .map(|s| i64::from(sets.get(s).is_some_and(|x| x.everyone)))
            .collect();
        rec.s.insert("everyone_in_view".into(), flags);
        // BRAWL-1's figure (Fix (b)): each soldier whose feet are in a building area (its record's `area`, the sim's
        // own: building i is area i + 1), as (slot, area) pairs, on every logged tick with one, in any phase.
        let in_building: Vec<i64> = records
            .iter()
            .filter(|r| r.kind == EntityKind::Soldier && r.area != 0)
            .flat_map(|r| [i64::from(r.index), i64::from(r.area)])
            .collect();
        if !in_building.is_empty() {
            if ph == Phase::Measured {
                self.counts.in_building_measured += in_building.len() as u64 / 2;
                self.counts
                    .in_building_slots
                    .extend(in_building.iter().step_by(2).map(|&s| s as u8));
            }
            rec.s.insert("in_building_slot_area".into(), in_building);
        }
        rec.s.insert(
            "snap_datagram_bytes".into(),
            datagrams.iter().flatten().map(|b| *b as i64).collect(),
        );
        rec.s.insert(
            "snap_wire_bytes".into(),
            datagrams
                .iter()
                .map(|d| d.iter().map(|b| *b as i64 + 28).sum())
                .collect(),
        );
        rec.s.insert("input_payload_bytes".into(), inputs.to_vec());
        if ph == Phase::Measured {
            for s in sets.iter().take(n) {
                self.counts.sets_measured += 1;
                self.counts.everyone_measured += u64::from(s.everyone);
                self.counts.large_measured += u64::from(s.large_units);
                self.counts.projectiles_measured += u64::from(s.projectiles);
            }
        }
        self.room.tick(&rec).map_err(io)?;
        for (s, kind) in refused_events {
            let mut e = EventRecord::new("snap_refused", ph)
                .with("slot", s)
                .with("kind", kind)
                .with("tick_count", w);
            e.t = Some(w);
            self.room.event(&e).map_err(io)?;
        }
        self.room_second(world, t, ph, records)
    }

    /// The per-second counts: shots, gameplay events (hits and kills) and projectile spawns from the World's own event
    /// list and records (the dump's `events` section: npt-sim's canonical state), portal changes from the doors.
    fn room_second(
        &mut self,
        world: &Sim,
        t: u32,
        ph: Phase,
        records: &[npt_sim::EntityRecord],
    ) -> Result<u64, String> {
        let (events, shots, reliable) = dump_events(world, &mut self.track.events_at)?;
        let mut portal = 0i64;
        let mut spawns = 0i64;
        let mut stand_in_spawns = 0i64;
        let mut stand_in_changes = 0i64;
        let mut live = BTreeMap::new();
        for r in records {
            match r.kind {
                EntityKind::Door => {
                    if self
                        .track
                        .doors
                        .insert(r.index, r.grid[0])
                        .is_some_and(|o| o != r.grid[0])
                    {
                        portal += 1;
                    }
                }
                EntityKind::Projectile => {
                    let stand_in = r.grid[7] == 0;
                    match self.track.projectiles.get(&r.index) {
                        Some((g, grid)) if *g == r.generation => {
                            if stand_in && *grid != r.grid {
                                stand_in_changes += 1;
                            }
                        }
                        _ => {
                            spawns += 1;
                            stand_in_spawns += i64::from(stand_in);
                        }
                    }
                    live.insert(r.index, (r.generation, r.grid));
                }
                _ => {}
            }
        }
        self.track.projectiles = live;
        let sec = u64::from(t.wrapping_sub(self.first_tick)) / u64::from(npt_sim::TICK_HZ);
        let add = [
            shots,
            portal,
            reliable,
            spawns,
            stand_in_changes,
            stand_in_spawns,
        ];
        match &mut self.track.second {
            Some((s, p, acc)) if *s == sec => {
                *p = ph;
                for (a, x) in acc.iter_mut().zip(add) {
                    *a += x;
                }
            }
            _ => {
                if let Some((s, p, acc)) = self.track.second.take() {
                    self.write_second(s, p, acc)?;
                }
                self.track.second = Some((sec, ph, add));
            }
        }
        Ok(events as u64)
    }

    fn write_second(&mut self, sec: u64, ph: Phase, acc: [i64; 6]) -> Result<(), String> {
        let mut r = SecondRecord::new(sec, ph);
        for (name, v) in [
            "shots_count",
            "portal_changes_count",
            "reliable_events_count",
            "projectile_spawns_count",
            "stand_in_changes_count",
            "stand_in_spawns_count",
        ]
        .into_iter()
        .zip(acc)
        {
            r.v.insert(name.to_owned(), v);
        }
        if ph == Phase::Measured {
            self.counts.shots_measured += acc[0];
            self.counts.reliable_measured += acc[2];
            self.counts.measured_seconds += 1;
        }
        self.room.second(&r).map_err(io)
    }

    /// The stop: inside each client's span the ticks the room logged a hash for and the client left unresolved are
    /// decided lost (the module text), every complete tick written, then the footers. `ended` is `Clean`, or `Crash`
    /// when a hash came after its tick was written (never silently dropped).
    pub(crate) fn finish(mut self, ended: Ended, last: &[EventRecord]) -> Result<Counts, String> {
        if let Some((s, p, acc)) = self.track.second.take() {
            self.write_second(s, p, acc)?;
        }
        let logged: Vec<(u64, Vec<bool>)> =
            self.logged.iter().map(|(w, l)| (*w, l.clone())).collect();
        for (w, list) in logged {
            for (s, room_had) in list.iter().enumerate() {
                let inside =
                    self.first[s].is_some_and(|f| f <= w) && self.last[s].is_some_and(|l| w <= l);
                if *room_had && inside && !self.gone[s] {
                    let e = &mut self.entry(w)[s];
                    if *e == Res::Open {
                        *e = Res::Lost;
                        self.counts.lost_decided_at_stop += 1;
                    }
                }
            }
        }
        // Every slot's later ticks are in flight: not waited for.
        self.gone.fill(true);
        self.flush()?;
        let rest: Vec<u64> = self
            .res
            .keys()
            .copied()
            .filter(|w| self.written.is_none_or(|m| *w > m))
            .collect();
        for w in rest {
            self.write_client_tick(w)?;
        }
        for e in last {
            self.lab.event(e).map_err(io)?;
        }
        let ended = if self.counts.conflicts > 0 {
            Ended::Crash
        } else {
            ended
        };
        self.room.finish(ended).map_err(io)?;
        self.client.finish(ended).map_err(io)?;
        self.lab.finish(ended).map_err(io)?;
        Ok(self.counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use npt_sim::{Command, HitHistory, Placement, Scene, SimCore, SimEvent, StandIns, TickInput};

    /// The advisor's U4r build ruling, point 6: the dump reader's counts at each tick equal the shots, hits and kills
    /// in `SimCore::step`'s returned events, over a short seeded run of the stress scene in which every soldier fires
    /// at one soldier of another team (team 0 at slot 16, the others at slot 0), so there are kills.
    #[test]
    fn p6_the_dump_reader_counts_the_shots_hits_and_kills_simcore_step_returns() {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = Sim::new(&scene, 3, 0);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let mut at = None;
        let (mut shots, mut hits, mut kills) = (0i64, 0i64, 0i64);
        for k in 0..600u32 {
            let soldiers: BTreeMap<u16, [i32; 3]> = SimCore::entity_records(&world)
                .iter()
                .filter(|r| r.kind == EntityKind::Soldier)
                .map(|r| (r.index, [r.grid[0], r.grid[1], r.grid[2]]))
                .collect();
            let commands = (0..48u8)
                .filter_map(|s| {
                    let p = soldiers.get(&if s < 16 { 16 } else { 0 })?;
                    let c = Command {
                        slot: s,
                        tick: k,
                        seq: k as u16,
                        buttons: Command::FIRE,
                        aim: [p[0], p[1], p[2] + 100],
                        view_tick: k,
                        ..Command::default()
                    };
                    Some((c, None))
                })
                .collect();
            let input = TickInput {
                tick: k,
                commands,
                ..TickInput::default()
            };
            let (mut s, mut r) = (0i64, 0i64);
            for e in world.step(&input, &history) {
                match e {
                    SimEvent::Shot { .. } => s += 1,
                    SimEvent::Hit { .. } => {
                        r += 1;
                        hits += 1;
                    }
                    SimEvent::Kill { .. } => {
                        r += 1;
                        kills += 1;
                    }
                }
            }
            shots += s;
            let (_, rs, rr) = dump_events(&world, &mut at).expect("the reader");
            assert_eq!(
                (rs, rr),
                (s, r),
                "tick {k}: the reader's shots and hits plus kills"
            );
        }
        println!(
            "p6: 600 ticks; shots {shots}, hits {hits}, kills {kills}, each tick's counts equal"
        );
        assert!(shots > 0 && hits > 0 && kills > 0, "{shots} {hits} {kills}");
    }

    /// advisor-u4r-r1, RR3-L2 (a): a run with no log folder keeps no per-tick client resolution. After an admission
    /// and one client's filings over 9,000 ticks (hashes, sentinels and lost runs), `res` is empty, the slot has
    /// started, and its first and last resolved ticks are the filings' first and last.
    #[test]
    fn rr3_l2_a_a_run_with_no_log_folder_keeps_no_per_tick_resolution() {
        let meta =
            crate::scenario::tests::settings(std::path::Path::new("-"), "--profile P0 --seed 1")
                .meta;
        let first = 63_736u32;
        let mut logs = Logs::open(None, &meta, first, false).expect("three sinks");
        logs.admit(3, 0, first, Phase::Warmup)
            .expect("the admission");
        assert!(!logs.started(3), "admitted, nothing resolved yet");
        for k in 0..9_000u32 {
            let t = first.wrapping_add(k);
            match k % 10 {
                0 => logs.filed(3, &[], &[(t, 1)]),
                1 => logs.filed(3, &[(t, 0)], &[]),
                _ => logs.filed(3, &[(t, u64::from(k) + 1)], &[]),
            }
        }
        assert!(logs.res.is_empty(), "{} ticks kept", logs.res.len());
        assert!(logs.started(3), "admitted and resolved");
        let w = u64::from(first);
        assert_eq!((logs.first[3], logs.last[3]), (Some(w), Some(w + 8_999)));
    }

    /// BRAWL-1 round 1 (TE-B1-2 = RR-B1-1), the building figure's positive control: soldier 3, teleported to building
    /// 0's centre (area 1) and given no command, stays there. A warm-up room tick writes its (slot, area) pair and
    /// counts nothing; the next room tick, measured, counts one slot-tick for slot 3 (two ticks, because npt-log's
    /// writer refuses a record at or before the last one: log lib.rs:467-474); both tick records hold the pair; the
    /// summary line prints the count with its slot, and "not counted (--logs 0)" on a no-log run.
    #[test]
    fn brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only() {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let first = 63_736u32;
        let mut server = crate::scenario::Server::build(
            &scene,
            1,
            first,
            48,
            0,
            0,
            &crate::scenario::Bites::default(),
        );
        server.room.script(
            &[],
            &[Placement {
                slot: 3,
                position: [2_200, 0, 0],
            }],
            &[],
        );
        server.room.tick();
        let dir = crate::scenario::tests::scratch("brawl1-figure");
        std::fs::create_dir_all(dir.as_path()).expect("the scratch folder");
        let s = crate::scenario::tests::settings(&dir, "--profile P0 --seed 1");
        let mut logs = Logs::open(Some(dir.as_path()), &s.meta, first, false).expect("three logs");
        logs.room_tick(
            &server.room,
            server.room.last_tick(),
            Phase::Warmup,
            &[],
            &[],
            &[],
        )
        .expect("the warm-up tick");
        assert_eq!(logs.counts.in_building_measured, 0, "warm-up: not counted");
        assert!(logs.counts.in_building_slots.is_empty(), "warm-up: no slot");
        server.room.tick();
        logs.room_tick(
            &server.room,
            server.room.last_tick(),
            Phase::Measured,
            &[],
            &[],
            &[],
        )
        .expect("the measured tick");
        assert_eq!(
            logs.counts.in_building_measured, 1,
            "measured: one slot-tick"
        );
        assert_eq!(
            logs.counts.in_building_slots,
            std::collections::BTreeSet::from([3]),
            "measured: slot 3"
        );
        let counts = logs.finish(Ended::Clean, &[]).expect("the footers");
        let room = crate::scenario::tests::read(&dir, "room");
        assert_eq!(room.ticks.len(), 2, "the warm-up and the measured tick");
        for t in &room.ticks {
            assert_eq!(
                t.s.get("in_building_slot_area"),
                Some(&vec![3, 1]),
                "tick {} ({:?}): written in any phase",
                t.t,
                t.ph
            );
        }
        let o = crate::scenario::Summary {
            counts: counts.clone(),
            ..crate::scenario::Summary::default()
        };
        let line = crate::scenario::summary_line(&s, &o);
        assert!(
            line.ends_with("in a building: 1 slot-ticks (slots 3)"),
            "{line}"
        );
        // parse takes --logs 0 only with --record ("a run keeps its logs"); the path is only stored, never written.
        let quiet = crate::scenario::tests::settings(
            &dir,
            &format!(
                "--profile P0 --seed 1 --logs 0 --record {}",
                dir.join("brawl1.rec").display()
            ),
        );
        let line = crate::scenario::summary_line(&quiet, &o);
        assert!(
            line.ends_with("in a building: not counted (--logs 0)"),
            "{line}"
        );
    }
}
