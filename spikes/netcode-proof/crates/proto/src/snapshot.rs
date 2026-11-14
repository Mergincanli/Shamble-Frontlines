//! `Snapshot` and `FullState`: the room's view of the world for one client, the field-group layout table, and the
//! `EntityRecord` ↔ wire mapping (u4-build-plan.md §4.4, §4.5; catalogue §3).
//!
//! **One layout, from npt-sim:** every entity kind's fields are `EntityKind::layout()`'s, in its order, read each time
//! and never copied (u4s-carries.md carry 1), for every kind in `EntityKind::ALL`; each field is packed by [`Quant`]
//! in its own width, so every size here follows from the layout. U4d completes the lists in wave 1 and nothing here
//! changes.
//!
//! **A snapshot** goes out as one or more groups that each decode alone, all against one baseline: the header carries
//! the full 32-bit tick, the baseline as its age in ticks (0: coded against the per-type defaults, so every entity
//! comes whole), the newest command received and the last command run (16-bit sequences), and the group's index and
//! the group count, so the client knows when a tick is complete (§4.5, §4.6). The body holds the group's own-state
//! records, its spatial records and its events.
//!
//! **Each section carries only the kinds its field groups list** (the advisor's W1-S1; SEC-U4a-2): the own-state
//! section the kinds of its message's owner rows in [`Snapshot::GROUPS`] or [`FullState::GROUPS`] (the client's own
//! soldier, its owner-only record `EntityKind::OwnState` and its effect rows), the entity section the kinds of every
//! other row. The encoder refuses a record in a section that does not list its kind ([`EncodeError::OutOfRange`]),
//! and the decoder refuses one as soon as its kind is read ([`DecodeError::Malformed`]), so an owner-only record never
//! travels among the spatial ones, either way.
//!
//! **The wire, in bits** (least significant first; every pad and reserved bit zero):
//! - header (bytes, after the type): tick 32, baseline age 16, newest command received 16, last command run 16,
//!   flags 8 (bit 0: the newest command is known; bit 1: the last command run is known), group 8 (bits 0-2 the
//!   index, bits 3-5 the count less one);
//! - body: own-state count 8, entity count 16, event count 8, then each own-state delta, each entity delta and each
//!   event;
//! - an entity delta: its kind (3 bits: its position in `EntityKind::ALL`), its index 16, its operation 2 (0 add,
//!   1 update, 2 remove), then for an add its generation 16 and every layout field, for an update a mask of one bit
//!   per layout field and each changed field, for a removal nothing;
//! - an event: its number's low 16 bits, its age in ticks 8 (how long before the header's tick it happened), its kind
//!   2 (0 shot, 1 hit, 2 kill), then its fields (slots 8, sequences 16, damage 16).
//!
//! `FullState`'s header (after the type): tick 32, last command run 16, flags 8 (bit 0: it is known); its body:
//! own-state count 8, entity count 16, then each record whole (kind, index, generation, every layout field). It has
//! no baseline and no event section (catalogue §3; Q8).

use npt_sim::{EntityKind, EntityRecord, SimEvent};

use crate::bits::{BitReader, BitWriter};
use crate::error::{DecodeError, EncodeError};
use crate::limits::Limits;
use crate::quant::Quant;

/// A field group's visibility class (the catalogue's §1 vocabulary, less `authority-only`: such a group is never
/// written, so it has no layout entry).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// Only the slot's own client.
    Owner,
    /// Only clients on that team.
    Team,
    /// Only clients whose relevance region holds it and whose area connects to it, or whose event test it passes.
    Spatial,
    /// Every client.
    Public,
}

/// One entity's change against the baseline the snapshot is coded against: the `EntityRecord` ↔ wire mapping
/// (u4-build-plan.md §4.4, A18). A record's `area` and `raw` are the room's and never sent: a record from the wire has
/// them 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityDelta {
    /// The entity whole: it enters the client's set, comes back with a new generation (a removal and an addition: ADR
    /// 0001 part 6), or the snapshot is coded against the defaults.
    Add(EntityRecord),
    /// The fields of an entity the baseline holds that changed: bit `i` of `changed` for the kind's layout field `i`,
    /// whose new integer is `grid[i]`; every other entry is 0.
    Update {
        kind: EntityKind,
        index: u16,
        changed: u16,
        grid: [i32; EntityRecord::FIELDS],
    },
    /// The entity left the client's set.
    Remove { kind: EntityKind, index: u16 },
}

// One mask bit per layout field.
const _: () = assert!(EntityRecord::FIELDS <= u16::BITS as usize);

impl EntityDelta {
    /// What the wire sends for `now` against `baseline`, the client's record of the same entity (by kind and index)
    /// in the baseline frame: `None` when nothing changed. A missing baseline record, or one of another generation, is
    /// an [`EntityDelta::Add`].
    pub fn between(now: &EntityRecord, baseline: Option<&EntityRecord>) -> Option<EntityDelta> {
        match baseline {
            Some(b)
                if b.kind == now.kind && b.index == now.index && b.generation == now.generation =>
            {
                let mut changed = 0u16;
                let mut grid = [0; EntityRecord::FIELDS];
                let n = now.kind.layout().len();
                for (i, ((g, &new), &old)) in grid
                    .iter_mut()
                    .zip(&now.grid)
                    .zip(&b.grid)
                    .take(n)
                    .enumerate()
                {
                    if new != old {
                        changed |= 1 << i;
                        *g = new;
                    }
                }
                (changed != 0).then_some(EntityDelta::Update {
                    kind: now.kind,
                    index: now.index,
                    changed,
                    grid,
                })
            }
            _ => Some(EntityDelta::Add(wire_record(now))),
        }
    }

    /// The entity's record after this delta, from `baseline`, the client's record of it in the baseline frame:
    /// `None` after a removal.
    ///
    /// # Errors
    /// [`DecodeError::Malformed`] for an update whose baseline record is missing or another entity's.
    pub fn apply(
        &self,
        baseline: Option<&EntityRecord>,
    ) -> Result<Option<EntityRecord>, DecodeError> {
        match *self {
            EntityDelta::Add(r) => Ok(Some(wire_record(&r))),
            EntityDelta::Update {
                kind,
                index,
                changed,
                grid,
            } => {
                let b = baseline
                    .filter(|b| b.kind == kind && b.index == index)
                    .ok_or(DecodeError::Malformed)?;
                let mut r = wire_record(b);
                for (i, g) in grid.iter().enumerate().take(kind.layout().len()) {
                    if changed & (1 << i) != 0 {
                        r.grid[i] = *g;
                    }
                }
                Ok(Some(r))
            }
            EntityDelta::Remove { .. } => Ok(None),
        }
    }
}

/// `r` as the wire carries it: `area` and `raw` 0, and every `grid` entry past its kind's layout 0.
fn wire_record(r: &EntityRecord) -> EntityRecord {
    let mut out = EntityRecord {
        area: 0,
        raw: [0; EntityRecord::FIELDS],
        grid: [0; EntityRecord::FIELDS],
        ..*r
    };
    let n = r.kind.layout().len();
    out.grid[..n].copy_from_slice(&r.grid[..n]);
    out
}

/// A message's section: the own-state section, or the entity section, which holds every other record.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Own,
    Entities,
}

/// A message's field-group table ([`Snapshot::GROUPS`], [`FullState::GROUPS`]).
type Groups = [(&'static str, Class, &'static [EntityKind])];

/// Whether `section` carries records of `kind`: one of the message's rows for that section lists it, its owner rows
/// for the own-state section and every other row for the entity section (the module text).
fn carries(groups: &Groups, section: Section, kind: EntityKind) -> bool {
    groups.iter().any(|&(_, class, kinds)| {
        (class == Class::Owner) == (section == Section::Own) && kinds.contains(&kind)
    })
}

/// Bits of an entity's kind on the wire: enough for every position in `EntityKind::ALL`.
const KIND_BITS: u32 = usize::BITS - (EntityKind::ALL.len() - 1).leading_zeros();

fn put_kind(w: &mut BitWriter<'_>, kind: EntityKind) {
    let code = EntityKind::ALL.iter().position(|k| *k == kind).unwrap_or(0);
    w.put(code as u64, KIND_BITS);
}

fn get_kind(r: &mut BitReader<'_>) -> Result<EntityKind, DecodeError> {
    let code = r.get(KIND_BITS)?;
    EntityKind::ALL
        .get(code as usize)
        .copied()
        .ok_or(DecodeError::OutOfRange)
}

/// A record's every layout field, each in its width; an entry past the layout must be 0.
fn put_fields(q: Quant, w: &mut BitWriter<'_>, r: &EntityRecord) -> Result<(), EncodeError> {
    let layout = r.kind.layout();
    if r.grid[layout.len()..].iter().any(|&g| g != 0) {
        return Err(EncodeError::NotCanonical);
    }
    for (f, &k) in layout.iter().zip(&r.grid) {
        q.put(w, *f, k)?;
    }
    Ok(())
}

fn get_fields(
    q: Quant,
    r: &mut BitReader<'_>,
    kind: EntityKind,
) -> Result<[i32; EntityRecord::FIELDS], DecodeError> {
    let mut grid = [0; EntityRecord::FIELDS];
    for (f, g) in kind.layout().iter().zip(&mut grid) {
        *g = q.get(r, *f)?;
    }
    Ok(grid)
}

/// A whole record: kind, index, generation, every layout field; refused in a section that does not carry its kind.
fn put_record(
    q: Quant,
    w: &mut BitWriter<'_>,
    r: &EntityRecord,
    groups: &Groups,
    section: Section,
) -> Result<(), EncodeError> {
    if !carries(groups, section, r.kind) {
        return Err(EncodeError::OutOfRange);
    }
    put_kind(w, r.kind);
    w.put(u64::from(r.index), 16);
    w.put(u64::from(r.generation), 16);
    put_fields(q, w, r)
}

fn get_record(
    q: Quant,
    r: &mut BitReader<'_>,
    groups: &Groups,
    section: Section,
) -> Result<EntityRecord, DecodeError> {
    let kind = get_kind(r)?;
    if !carries(groups, section, kind) {
        return Err(DecodeError::Malformed);
    }
    let index = r.u16()?;
    let generation = r.u16()?;
    let grid = get_fields(q, r, kind)?;
    Ok(EntityRecord {
        kind,
        index,
        generation,
        area: 0,
        grid,
        raw: [0; EntityRecord::FIELDS],
    })
}

const ADD: u64 = 0;
const UPDATE: u64 = 1;
const REMOVE: u64 = 2;

/// One delta; refused in a section that does not carry its kind.
fn put_delta(
    q: Quant,
    w: &mut BitWriter<'_>,
    d: &EntityDelta,
    coded_from_defaults: bool,
    groups: &Groups,
    section: Section,
) -> Result<(), EncodeError> {
    let kind = match d {
        EntityDelta::Add(rec) => rec.kind,
        EntityDelta::Update { kind, .. } | EntityDelta::Remove { kind, .. } => *kind,
    };
    if !carries(groups, section, kind) {
        return Err(EncodeError::OutOfRange);
    }
    match d {
        EntityDelta::Add(rec) => {
            put_kind(w, rec.kind);
            w.put(u64::from(rec.index), 16);
            w.put(ADD, 2);
            w.put(u64::from(rec.generation), 16);
            put_fields(q, w, rec)
        }
        EntityDelta::Update {
            kind,
            index,
            changed,
            grid,
        } => {
            let n = kind.layout().len();
            let all = (1u32 << n) - 1;
            if coded_from_defaults || *changed == 0 || u32::from(*changed) & !all != 0 {
                return Err(EncodeError::NotCanonical);
            }
            if grid
                .iter()
                .enumerate()
                .any(|(i, &g)| changed & (1 << i) == 0 && g != 0)
            {
                return Err(EncodeError::NotCanonical);
            }
            put_kind(w, *kind);
            w.put(u64::from(*index), 16);
            w.put(UPDATE, 2);
            w.put(u64::from(*changed), n as u32);
            for (i, f) in kind.layout().iter().enumerate() {
                if changed & (1 << i) != 0 {
                    q.put(w, *f, grid[i])?;
                }
            }
            Ok(())
        }
        EntityDelta::Remove { kind, index } => {
            if coded_from_defaults {
                return Err(EncodeError::NotCanonical);
            }
            put_kind(w, *kind);
            w.put(u64::from(*index), 16);
            w.put(REMOVE, 2);
            Ok(())
        }
    }
}

fn get_delta(
    q: Quant,
    r: &mut BitReader<'_>,
    coded_from_defaults: bool,
    groups: &Groups,
    section: Section,
) -> Result<EntityDelta, DecodeError> {
    let kind = get_kind(r)?;
    if !carries(groups, section, kind) {
        return Err(DecodeError::Malformed);
    }
    let index = r.u16()?;
    let op = r.get(2)?;
    let d = match op {
        ADD => {
            let generation = r.u16()?;
            let grid = get_fields(q, r, kind)?;
            EntityDelta::Add(EntityRecord {
                kind,
                index,
                generation,
                area: 0,
                grid,
                raw: [0; EntityRecord::FIELDS],
            })
        }
        UPDATE => {
            let layout = kind.layout();
            // A kind with no fields has no update: its mask would be empty.
            if layout.is_empty() {
                return Err(DecodeError::Malformed);
            }
            let changed = r.get(layout.len() as u32)? as u16;
            if changed == 0 {
                return Err(DecodeError::Malformed);
            }
            let mut grid = [0; EntityRecord::FIELDS];
            for (i, f) in layout.iter().enumerate() {
                if changed & (1 << i) != 0 {
                    grid[i] = q.get(r, *f)?;
                }
            }
            EntityDelta::Update {
                kind,
                index,
                changed,
                grid,
            }
        }
        REMOVE => EntityDelta::Remove { kind, index },
        _ => return Err(DecodeError::OutOfRange),
    };
    if coded_from_defaults && !matches!(d, EntityDelta::Add(_)) {
        return Err(DecodeError::Malformed);
    }
    Ok(d)
}

fn put_event(w: &mut BitWriter<'_>, (number, age, event): &(u16, u8, SimEvent)) {
    w.put(u64::from(*number), 16);
    w.put(u64::from(*age), 8);
    match *event {
        SimEvent::Shot { slot, seq } => {
            w.put(0, 2);
            w.put(u64::from(slot), 8);
            w.put(u64::from(seq), 16);
        }
        SimEvent::Hit {
            shooter,
            seq,
            victim,
            damage,
        } => {
            w.put(1, 2);
            w.put(u64::from(shooter), 8);
            w.put(u64::from(seq), 16);
            w.put(u64::from(victim), 8);
            w.put(u64::from(damage), 16);
        }
        SimEvent::Kill { killer, victim } => {
            w.put(2, 2);
            w.put(u64::from(killer), 8);
            w.put(u64::from(victim), 8);
        }
    }
}

fn get_event(r: &mut BitReader<'_>) -> Result<(u16, u8, SimEvent), DecodeError> {
    let number = r.u16()?;
    let age = r.u8()?;
    let event = match r.get(2)? {
        0 => SimEvent::Shot {
            slot: r.u8()?,
            seq: r.u16()?,
        },
        1 => SimEvent::Hit {
            shooter: r.u8()?,
            seq: r.u16()?,
            victim: r.u8()?,
            damage: r.u16()?,
        },
        2 => SimEvent::Kill {
            killer: r.u8()?,
            victim: r.u8()?,
        },
        _ => return Err(DecodeError::OutOfRange),
    };
    Ok((number, age, event))
}

/// One group of one tick's snapshot for one client (the module text). Its `Default` is an empty single group at
/// tick 0.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    /// The server tick, all 32 bits.
    pub tick: u32,
    /// The baseline's age, `tick` less the baseline's tick; 0: coded against the per-type defaults, every entity an
    /// [`EntityDelta::Add`].
    pub baseline_age: u16,
    /// The newest command received from this client, which ends its repeats (`None` before the first).
    pub newest_received: Option<u16>,
    /// The last command run, where the client's replay starts (`None` before the first).
    pub last_run: Option<u16>,
    /// This group's index, below `groups`.
    pub group: u8,
    /// How many groups this tick's snapshot has, 1 to [`Limits::GROUPS`].
    pub groups: u8,
    /// The own-state section's deltas: the client's own soldier, its owner-only record and its effect rows.
    pub own: Vec<EntityDelta>,
    /// The spatial records' deltas.
    pub entities: Vec<EntityDelta>,
    /// The event section: each event's number (low 16 bits), its age in ticks before `tick`, and the event.
    pub events: Vec<(u16, u8, SimEvent)>,
}

impl Default for Snapshot {
    fn default() -> Snapshot {
        Snapshot {
            tick: 0,
            baseline_age: 0,
            newest_received: None,
            last_run: None,
            group: 0,
            groups: 1,
            own: Vec::new(),
            entities: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl Snapshot {
    /// The field groups a snapshot's encoder writes, in the catalogue's order (§3's `Snapshot` rows less the
    /// authority-only three, which have no entry): each group's name as the catalogue gives it, its class, and the
    /// entity kinds whose records it carries (each kind's fields are its `EntityKind::layout()`). The own-state group
    /// carries the client's own soldier, its owner-only record (`EntityKind::OwnState`: its velocity and its weapon's
    /// state, the advisor's W1-S1) and its effect rows. Each section carries only the kinds its rows list (the module
    /// text).
    pub const GROUPS: [(&'static str, Class, &'static [EntityKind]); 12] = [
        ("header: server tick", Class::Public, &[]),
        ("header: baseline and acknowledgements", Class::Owner, &[]),
        (
            "own state",
            Class::Owner,
            &[
                EntityKind::Soldier,
                EntityKind::Effect,
                EntityKind::OwnState,
            ],
        ),
        ("remote soldiers", Class::Spatial, &[EntityKind::Soldier]),
        (
            "vehicles",
            Class::Spatial,
            &[EntityKind::Vehicle, EntityKind::VehiclePart],
        ),
        ("projectiles", Class::Spatial, &[EntityKind::Projectile]),
        (
            "world pieces",
            Class::Spatial,
            &[EntityKind::Door, EntityKind::Wall],
        ),
        ("gameplay events", Class::Spatial, &[]),
        ("hit confirmation", Class::Owner, &[]),
        ("cosmetic events", Class::Spatial, &[]),
        ("team section", Class::Team, &[]),
        ("public section", Class::Public, &[]),
    ];

    pub(crate) fn put(&self, q: Quant, w: &mut BitWriter<'_>) -> Result<(), EncodeError> {
        if usize::from(self.groups) > Limits::GROUPS {
            return Err(EncodeError::TooMany);
        }
        if self.group >= self.groups {
            return Err(EncodeError::OutOfRange);
        }
        if self.own.len() > usize::from(u8::MAX)
            || self.entities.len() > usize::from(u16::MAX)
            || self.events.len() > usize::from(u8::MAX)
        {
            return Err(EncodeError::TooMany);
        }
        w.put(u64::from(q.header_tick(self.tick)), 32);
        w.put(u64::from(self.baseline_age), 16);
        w.put(u64::from(self.newest_received.unwrap_or(0)), 16);
        w.put(u64::from(self.last_run.unwrap_or(0)), 16);
        let flags =
            u64::from(self.newest_received.is_some()) | u64::from(self.last_run.is_some()) << 1;
        w.put(flags, 8);
        w.put(u64::from(self.group) | u64::from(self.groups - 1) << 3, 8);
        w.put(self.own.len() as u64, 8);
        w.put(self.entities.len() as u64, 16);
        w.put(self.events.len() as u64, 8);
        let from_defaults = self.baseline_age == 0;
        for d in &self.own {
            put_delta(q, w, d, from_defaults, &Snapshot::GROUPS, Section::Own)?;
        }
        for d in &self.entities {
            put_delta(q, w, d, from_defaults, &Snapshot::GROUPS, Section::Entities)?;
        }
        for e in &self.events {
            put_event(w, e);
        }
        Ok(())
    }

    pub(crate) fn get(q: Quant, r: &mut BitReader<'_>) -> Result<Snapshot, DecodeError> {
        let strict = q.strict();
        let tick = r.u32()?;
        let baseline_age = r.u16()?;
        let newest = r.u16()?;
        let last = r.u16()?;
        let flags = r.u8()?;
        let group_byte = r.u8()?;
        if strict && (flags & !0b11 != 0 || group_byte & !0b11_1111 != 0) {
            return Err(DecodeError::UnknownField);
        }
        let known = |bit: u8, v: u16| {
            if flags & bit != 0 {
                Ok(Some(v))
            } else if v != 0 && strict {
                Err(DecodeError::UnknownField)
            } else {
                Ok(None)
            }
        };
        let newest_received = known(1, newest)?;
        let last_run = known(2, last)?;
        let group = group_byte & 0b111;
        let groups = ((group_byte >> 3) & 0b111) + 1;
        if group >= groups {
            return Err(DecodeError::Malformed);
        }
        let n_own = usize::from(r.u8()?);
        let n_entities = usize::from(r.u16()?);
        let n_events = usize::from(r.u8()?);
        let from_defaults = baseline_age == 0;
        // Each count is bounded by its width and every item is at least 21 bits, so the datagram's own size bounds
        // what is read; the vectors grow with what decodes.
        let mut own = Vec::new();
        for _ in 0..n_own {
            own.push(get_delta(
                q,
                r,
                from_defaults,
                &Snapshot::GROUPS,
                Section::Own,
            )?);
        }
        let mut entities = Vec::new();
        for _ in 0..n_entities {
            entities.push(get_delta(
                q,
                r,
                from_defaults,
                &Snapshot::GROUPS,
                Section::Entities,
            )?);
        }
        let mut events = Vec::new();
        for _ in 0..n_events {
            events.push(get_event(r)?);
        }
        Ok(Snapshot {
            tick,
            baseline_age,
            newest_received,
            last_run,
            group,
            groups,
            own,
            entities,
            events,
        })
    }
}

/// A client's whole view at one tick, on its own stream: at join, after a `Resync`, or when the room forces a resync
/// (the module text).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FullState {
    /// The tick the whole state is taken at, all 32 bits.
    pub tick: u32,
    /// The last command run, where the client's prediction starts (`None` before the first).
    pub last_run: Option<u16>,
    /// The own-state section: the client's own soldier, its owner-only record and its effect rows, whole.
    pub own: Vec<EntityRecord>,
    /// Every entity of the client's interest set at that tick, whole.
    pub entities: Vec<EntityRecord>,
}

impl FullState {
    /// The field groups a full state's encoder writes (catalogue §3's `FullState` rows less the authority-only
    /// three), as [`Snapshot::GROUPS`] gives them.
    pub const GROUPS: [(&'static str, Class, &'static [EntityKind]); 6] = [
        ("header: server tick", Class::Public, &[]),
        ("header: baseline and acknowledgements", Class::Owner, &[]),
        (
            "own state",
            Class::Owner,
            &[
                EntityKind::Soldier,
                EntityKind::Effect,
                EntityKind::OwnState,
            ],
        ),
        (
            "spatial world",
            Class::Spatial,
            &[
                EntityKind::Soldier,
                EntityKind::Vehicle,
                EntityKind::VehiclePart,
                EntityKind::Projectile,
                EntityKind::Door,
                EntityKind::Wall,
            ],
        ),
        ("team section", Class::Team, &[]),
        ("public section", Class::Public, &[]),
    ];

    pub(crate) fn put(&self, q: Quant, w: &mut BitWriter<'_>) -> Result<(), EncodeError> {
        if self.own.len() > usize::from(u8::MAX) || self.entities.len() > usize::from(u16::MAX) {
            return Err(EncodeError::TooMany);
        }
        w.put(u64::from(self.tick), 32);
        w.put(u64::from(self.last_run.unwrap_or(0)), 16);
        w.put(u64::from(self.last_run.is_some()), 8);
        w.put(self.own.len() as u64, 8);
        w.put(self.entities.len() as u64, 16);
        for rec in &self.own {
            put_record(q, w, rec, &FullState::GROUPS, Section::Own)?;
        }
        for rec in &self.entities {
            put_record(q, w, rec, &FullState::GROUPS, Section::Entities)?;
        }
        Ok(())
    }

    pub(crate) fn get(q: Quant, r: &mut BitReader<'_>) -> Result<FullState, DecodeError> {
        let strict = q.strict();
        let tick = r.u32()?;
        let last = r.u16()?;
        let flags = r.u8()?;
        if strict && (flags & !1 != 0 || (flags & 1 == 0 && last != 0)) {
            return Err(DecodeError::UnknownField);
        }
        let last_run = (flags & 1 != 0).then_some(last);
        let n_own = usize::from(r.u8()?);
        let n_entities = usize::from(r.u16()?);
        let mut own = Vec::new();
        for _ in 0..n_own {
            own.push(get_record(q, r, &FullState::GROUPS, Section::Own)?);
        }
        let mut entities = Vec::new();
        for _ in 0..n_entities {
            entities.push(get_record(q, r, &FullState::GROUPS, Section::Entities)?);
        }
        Ok(FullState {
            tick,
            last_run,
            own,
            entities,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The advisor's W1-S1, three checks: every kind has a home in both tables; the owner-only kinds (effect rows and
    /// the owner-only record) sit in owner rows only; and a remote soldier's add carries exactly the catalogue's remote
    /// row (docs/design/netcode-proof-messages.md §3), the security review's proof form for SEC-U4a-1.
    #[test]
    fn groups_home_every_kind_keep_owner_only_kinds_to_owner_rows_and_send_the_remote_row() {
        for table in [&Snapshot::GROUPS[..], &FullState::GROUPS[..]] {
            for kind in EntityKind::ALL {
                let homes: Vec<_> = table.iter().filter(|g| g.2.contains(&kind)).collect();
                assert!(!homes.is_empty(), "{kind:?} has no group");
                if matches!(kind, EntityKind::Effect | EntityKind::OwnState) {
                    assert!(
                        homes.iter().all(|g| g.1 == Class::Owner),
                        "{kind:?}: {homes:?}"
                    );
                }
            }
        }
        assert_eq!(KIND_BITS, 3);
        // The remote row: position in 16, 16 and 12 bits, yaw 16, pitch 10, 8 flag bits, then health (0 to 1,023)
        // and weapon id (0 to 15) in the sim's ranges; nothing of its owner's (velocity, the weapon's state).
        let widths: Vec<u32> = EntityKind::Soldier
            .layout()
            .iter()
            .map(|f| f.bits())
            .collect();
        assert_eq!(widths, [16, 16, 12, 16, 10, 8, 10, 4]);
        // On the wire, a remote soldier's add is its kind (3 bits), index 16, operation 2, generation 16 and the row's
        // 92 bits: a marker bit written right after it is the buffer's highest set bit.
        let soldier = EntityRecord {
            kind: EntityKind::Soldier,
            index: 4,
            generation: 1,
            area: 0,
            grid: [0; EntityRecord::FIELDS],
            raw: [0; EntityRecord::FIELDS],
        };
        let mut bytes = Vec::new();
        let mut w = BitWriter::new(&mut bytes);
        put_delta(
            Quant::default(),
            &mut w,
            &EntityDelta::Add(soldier),
            false,
            &Snapshot::GROUPS,
            Section::Entities,
        )
        .expect("a remote soldier");
        w.put(1, 1);
        let top = bytes.last().expect("bytes").leading_zeros() as usize;
        assert_eq!(bytes.len() * 8 - 1 - top, 3 + 16 + 2 + 16 + 92);
    }
}
