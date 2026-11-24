//! Figures: the computations a mark needs that README §2.3's statistics cannot express (ratios across scale steps,
//! slopes, arithmetic against a formula). Every name `marks.json` uses must be listed here; one that is listed but
//! not built yet says which unit builds it, and its check comes out NOT-RUN rather than guessed.

use crate::eval::{Missing, Run, every_run_rules, stat_of};
use crate::marks::Stat;
use crate::ratio::Ratio;
use npt_log::{Log, Phase, SecondRecord};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Why a figure has no value.
pub enum Lack {
    /// A gap in the record: the run is INVALID (README §2.3).
    Gap(String),
    /// A run the figure compares with is not in the set yet: the part is NOT-RUN, as for a missing `against`
    /// reference (README §2.3).
    Waiting(String),
    /// A comparison that compared nothing: its check fails, never passes, and shows no bite (README §2.3).
    Empty(String),
    /// The figure fails with no number to show (a client that never got its close): its check fails as a value over
    /// its limit does, and its reason is printed in the value's place.
    Fail(String),
}

/// A mark's kept parts by name, each with its `min` (the valid runs the part needs) and its valid runs: those that
/// held their validity rules and setup checks.
pub type Parts<'r, 'a> = BTreeMap<String, (usize, Vec<&'r Run<'a>>)>;

/// What a figure reads: the run under judgement, every run of its set, and the valid runs of the mark's kept parts
/// (for a figure that compares across parts or pools a part's runs; empty where the caller has no mark's parts).
pub struct Inputs<'r, 'a> {
    pub run: &'r Run<'a>,
    pub set: &'r [&'r Run<'a>],
    pub parts: &'r Parts<'r, 'a>,
}

/// PM-0001-2: the least share, over client slots, of measured ticks on which that slot's snapshot held every other
/// soldier (`tick.s.everyone_in_view`, one 0/1 per slot, in slot order).
fn everyone_in_view_min_share(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let mut seen: Vec<i128> = Vec::new();
    let mut ticks: i128 = 0;
    for log in &run.logs {
        for t in log.ticks.iter().filter(|t| t.ph == Phase::Measured) {
            let Some(flags) = t.s.get("everyone_in_view") else {
                continue;
            };
            if seen.is_empty() {
                seen = vec![0; flags.len()];
            }
            if flags.len() != seen.len() {
                return Err(Lack::Gap(format!(
                    "run {}: tick {} changes the slot count of everyone_in_view",
                    run.id, t.t
                )));
            }
            for (s, f) in seen.iter_mut().zip(flags) {
                *s += i128::from(*f == 1);
            }
            ticks += 1;
        }
    }
    if ticks == 0 || seen.is_empty() {
        return Err(Lack::Gap(format!(
            "run {}: no values for tick.s.everyone_in_view",
            run.id
        )));
    }
    let least = seen.iter().copied().min().unwrap_or(0);
    Ok(Ratio::new(least, ticks))
}

/// One header param of a run: the single value its logs give it (`None` if no log has it), or a gap if two logs
/// disagree.
fn run_param<'r>(run: &'r Run<'_>, name: &str) -> Result<Option<&'r Value>, Lack> {
    let mut found: Option<&Value> = None;
    for log in &run.logs {
        if let Some(v) = log.header.params.get(name) {
            match found {
                Some(f) if f != v => {
                    return Err(Lack::Gap(format!(
                        "run {}: its logs disagree on params.{name}",
                        run.id
                    )));
                }
                _ => found = Some(v),
            }
        }
    }
    Ok(found)
}

/// A run's `params.fate_log_sha256`: 64 lowercase hex digits, or a gap.
fn fate_log_hash<'r>(run: &'r Run<'_>) -> Result<&'r str, Lack> {
    let hash = run_param(run, "fate_log_sha256")?.and_then(Value::as_str);
    match hash {
        Some(h)
            if h.len() == 64
                && h.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
        {
            Ok(h)
        }
        _ => Err(Lack::Gap(format!(
            "run {}: params.fate_log_sha256 is missing or not 64 lowercase hex digits",
            run.id
        ))),
    }
}

/// The run a run names in `params.twin`, from its set: a gap if it names none, names itself, or names a run that
/// breaks a rule every run holds (complete logs, the roster, capabilities, and C4's build rule: a twin of a measured
/// run built with the bites, or of a bite run without them); NOT-RUN while the twin is not in the set yet. `fields`
/// are the header fields the twin must share (`bite` among them: a bite run's twin runs the same bite).
fn twin_of<'r, 'a>(
    run: &Run<'_>,
    set: &[&'r Run<'a>],
    fields: &[&str],
) -> Result<&'r Run<'a>, Lack> {
    let Some(name) = run_param(run, "twin")?.and_then(Value::as_str) else {
        return Err(Lack::Gap(format!(
            "run {}: params.twin names no twin run",
            run.id
        )));
    };
    if name == run.id {
        return Err(Lack::Gap(format!(
            "run {}: params.twin names the run itself",
            run.id
        )));
    }
    let Some(twin) = set.iter().copied().find(|r| r.id == name) else {
        return Err(Lack::Waiting(format!(
            "run {}: its twin {name} is not in set {} yet",
            run.id, run.set
        )));
    };
    // A bite run's logging-off twin runs the same bite on the same bites build (PM-0006-20's planted-format bite):
    // the one pair the build rule's role list refuses and a twin comparison needs. Every other rule holds for it.
    let roles: &[&str] = if run.role == "bite" && twin.role == "logging-off" {
        &["bite", "twin", "logging-off"]
    } else {
        &["bite", "twin"]
    };
    let broken = every_run_rules(twin, roles);
    if !broken.is_empty() {
        return Err(Lack::Gap(format!(
            "run {}: its twin {name} is invalid: {}",
            run.id,
            broken.join("; ")
        )));
    }
    let (Some(a), Some(b)) = (run.logs.first(), twin.logs.first()) else {
        return Err(Lack::Gap(format!("run {}: a run without a log", run.id)));
    };
    let (a, b) = (&a.header, &b.header);
    for field in fields {
        let same = match *field {
            "layer" => a.layer == b.layer,
            "profile" => a.profile == b.profile,
            "seed" => a.seed == b.seed,
            _ => run.bite == twin.bite,
        };
        if !same {
            return Err(Lack::Gap(format!(
                "run {}: its twin {name} has another {field}",
                run.id
            )));
        }
    }
    Ok(twin)
}

/// PM-0001-3 ("on L1, the same seed and profile give an identical packet-fate log on two runs"): 1 when the run's
/// `params.fate_log_sha256` equals its twin's, 0 when they differ. The twin is the run of the same set named in
/// `params.twin`, with role `twin` and the same layer, profile, seed and bite (a bite run's twin runs the same bite).
/// A run that names no twin, a twin that is not one, differs in those fields or is invalid (`twin_of`), and a missing
/// hash are gaps; a twin not in the set yet leaves the check NOT-RUN.
fn fate_log_twin_equal(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let own = fate_log_hash(run)?;
    let twin = twin_of(run, set, &["layer", "profile", "seed", "bite"])?;
    if twin.role != "twin" {
        return Err(Lack::Gap(format!(
            "run {}: {} has role {}, not twin",
            run.id, twin.id, twin.role
        )));
    }
    Ok(Ratio::int(i128::from(own == fate_log_hash(twin)?)))
}

/// A run's state hash per tick (`h`), over all its logs and every phase: a gap if a hash is not 16 lowercase hex
/// digits, or if two logs give one tick two hashes.
fn state_hashes(run: &Run<'_>) -> Result<BTreeMap<u64, u64>, Lack> {
    let mut out = BTreeMap::new();
    for log in &run.logs {
        for t in log.ticks.iter().filter(|t| t.h.is_some()) {
            let Some(h) = t.hash() else {
                return Err(Lack::Gap(format!(
                    "run {}: tick {}'s h is not 16 lowercase hex digits",
                    run.id, t.t
                )));
            };
            if out.insert(t.t, h).is_some_and(|old| old != h) {
                return Err(Lack::Gap(format!(
                    "run {}: two logs give tick {} two state hashes",
                    run.id, t.t
                )));
            }
        }
    }
    Ok(out)
}

/// PM-0001-21 and -30 ("identical server state hashes on every tick"): the tick numbers whose state hash (`h`) differs
/// between the run and the run it names in `params.twin`, a tick hashed in one only counting; every phase. The twin
/// (`twin_of`) has the same profile, seed and bite, and role `twin`, else a gap, as for `fate_log_twin_equal` (the
/// advisor's FX-4; -30's form comes with U6). It fails, never passes, when neither run hashed a tick.
fn twin_hash_mismatch_count(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let own = state_hashes(run)?;
    let twin = twin_of(run, set, &["profile", "seed", "bite"])?;
    if twin.role != "twin" {
        other_transport_twin(run, twin)?;
    }
    let other = state_hashes(twin)?;
    let ticks: BTreeSet<u64> = own.keys().chain(other.keys()).copied().collect();
    if ticks.is_empty() {
        return Err(Lack::Empty(format!(
            "run {} and its twin {} hashed no tick",
            run.id, twin.id
        )));
    }
    let differ = ticks.iter().filter(|t| own.get(t) != other.get(t)).count();
    Ok(Ratio::int(differ as i128))
}

/// PM-0001-30's twin form (C-U6-9), for a twin that is not role `twin`: the run and its twin are both measured runs,
/// each names the other in `params.twin`, and their `params.transport` are `wt` and `ws`, one each. Any other twin
/// that is not role `twin` is a gap (PM-0001-21's runs carry no transport, so its twins must be role `twin`).
fn other_transport_twin(run: &Run<'_>, twin: &Run<'_>) -> Result<(), Lack> {
    let gap = |why: &str| {
        Err(Lack::Gap(format!(
            "run {}: {} has role {}, not twin, and {why}",
            run.id, twin.id, twin.role
        )))
    };
    if run.role != "measured" || twin.role != "measured" {
        return gap("the two are not both measured runs");
    }
    if run_param(twin, "twin")?.and_then(Value::as_str) != Some(run.id) {
        return gap("it does not name this run back");
    }
    let transport = |r| Ok(run_param(r, "transport")?.and_then(Value::as_str));
    match (transport(run)?, transport(twin)?) {
        (Some("wt"), Some("ws")) | (Some("ws"), Some("wt")) => Ok(()),
        _ => gap("their params.transport are not wt and ws, one each"),
    }
}

/// A run's `tick.v.tick_allocs_count` per measured tick number, from its room logs (the tick thread is the room's):
/// a gap with no room log, a measured tick record without the count (counters are on every record), or two logs
/// giving one tick a count.
fn alloc_counts(run: &Run<'_>) -> Result<BTreeMap<u64, i64>, Lack> {
    let gap = |why: String| Err(Lack::Gap(format!("run {}: {why}", run.id)));
    let rooms: Vec<_> = run
        .logs
        .iter()
        .filter(|l| l.header.process == "room")
        .collect();
    if rooms.is_empty() {
        return gap("no room log (tick.v.tick_allocs_count's writer)".to_owned());
    }
    let mut out = BTreeMap::new();
    for t in rooms.iter().flat_map(|l| &l.ticks) {
        if t.ph != Phase::Measured {
            continue;
        }
        let Some(n) = t.v.get("tick_allocs_count") else {
            return gap(format!(
                "measured tick {} has no tick.v.tick_allocs_count",
                t.t
            ));
        };
        if out.insert(t.t, *n).is_some() {
            return gap(format!(
                "two room logs give tick {} an allocation count",
                t.t
            ));
        }
    }
    Ok(out)
}

/// PM-0006-20 ("on every tick after the warm-up ... equals the count with logging off", "compared tick by tick"): the
/// measured tick numbers whose `tick.v.tick_allocs_count` differs between the run and the run it names in
/// `params.twin`, a tick counted in one only counting. The twin (`twin_of`) has role `logging-off` and the same
/// profile, seed and bite, else a gap; a twin not in the set yet is NOT-RUN. It fails, never passes, when neither
/// run has a measured tick.
fn alloc_twin_diff_count(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let twin = twin_of(run, set, &["profile", "seed", "bite"])?;
    if twin.role != "logging-off" {
        return Err(Lack::Gap(format!(
            "run {}: {} has role {}, not logging-off",
            run.id, twin.id, twin.role
        )));
    }
    let (own, other) = (alloc_counts(run)?, alloc_counts(twin)?);
    let ticks: BTreeSet<u64> = own.keys().chain(other.keys()).copied().collect();
    if ticks.is_empty() {
        return Err(Lack::Empty(format!(
            "run {} and its twin {} have no measured tick",
            run.id, twin.id
        )));
    }
    let differ = ticks.iter().filter(|t| own.get(t) != other.get(t)).count();
    Ok(Ratio::int(differ as i128))
}

/// One `hs` name's hashes in a run, merged over its logs and every phase: per tick one entry per client slot, each
/// with whether its log was written on LG on aarch64 (`machine.role` LG, `machine.arch` aarch64: the Arm rule's
/// clients). `None` when no log has the name. Gaps: the name in a log of another process than the one that writes it
/// (`source`), lists of different lengths, a malformed entry, or two logs giving one (slot, tick) a hash.
type SlotHashes = BTreeMap<u64, Vec<Option<(u64, bool)>>>;

/// Which logs write each `hs` name (the advisor's FX-5 (a)): `server_hash` only the room's, `decoded_hash` only the
/// clients' (process `client` on L1, `bots` on L2), `replay_hash` only a `replay` log on x86_64.
fn source(name: &str) -> (&'static str, fn(&npt_log::Header) -> bool) {
    match name {
        "server_hash" => ("process room", |h| h.process == "room"),
        "decoded_hash" => ("process client (L1) or bots (L2)", |h| {
            h.process == "client" || h.process == "bots"
        }),
        _ => ("process replay on x86_64", |h| {
            h.process == "replay" && h.machine.arch == "x86_64"
        }),
    }
}

fn slot_hashes(run: &Run<'_>, name: &str) -> Result<Option<(usize, SlotHashes)>, Lack> {
    let mut out: SlotHashes = BTreeMap::new();
    let mut slots: Option<usize> = None;
    let (writer, writes) = source(name);
    for log in &run.logs {
        let h = &log.header;
        if !writes(h) && log.ticks.iter().any(|t| t.hs.contains_key(name)) {
            return Err(Lack::Gap(format!(
                "run {}: hs.{name} in the {} log on {} ({}): only {writer} writes it",
                run.id, h.process, h.machine.name, h.machine.arch
            )));
        }
        let arm = h.machine.role == "LG" && h.machine.arch == "aarch64";
        for t in log.ticks.iter().filter(|t| t.hs.contains_key(name)) {
            let Some(list) = t.hashes(name) else {
                return Err(Lack::Gap(format!(
                    "run {}: tick {}'s hs.{name} is malformed",
                    run.id, t.t
                )));
            };
            if *slots.get_or_insert(list.len()) != list.len() {
                return Err(Lack::Gap(format!(
                    "run {}: tick {}'s hs.{name} has {} slots, not {}",
                    run.id,
                    t.t,
                    list.len(),
                    slots.unwrap_or(0)
                )));
            }
            let merged = out.entry(t.t).or_insert_with(|| vec![None; list.len()]);
            for (slot, h) in list.into_iter().enumerate() {
                let Some(h) = h else { continue };
                if merged[slot].is_some() {
                    return Err(Lack::Gap(format!(
                        "run {}: two logs give slot {slot} at tick {} an hs.{name}",
                        run.id, t.t
                    )));
                }
                merged[slot] = Some((h, arm));
            }
        }
    }
    Ok(slots.map(|n| (n, out)))
}

/// The decoded-hash comparison of one run (u4-build-plan.md §4.6, the advisor's Q11 ruling, FX-5 and FX-10).
struct Decoded {
    /// Client hashes looked at: a (client, tick) pair with the server's hash, or a client hash at a (client, tick) the
    /// server never logged.
    compared: i128,
    /// Of those, the ones that count: a different hash, the sentinel 0, or no server hash there (on every machine);
    /// under the Arm rule (an LG client on aarch64, the room on x86_64) a different hash only when the x86-64
    /// re-decode misses the server's hash too.
    mismatched: i128,
    /// Mismatches under the Arm rule that the x86-64 re-decode does not reproduce: a cross-architecture finding,
    /// deciding nothing.
    cross_arch: i128,
    /// The first coverage gap (README §1's `null` rule: FX-10 (b) to (d)) or incomplete re-decode (FX-5 (d)). It
    /// decides the run only after the counted mismatches and the empty comparison (FX-10 (f)), so it never hides one.
    gap: Option<String>,
    /// An Arm-rule mismatch in a run with no replay log: NOT-RUN, after everything above.
    waiting: Option<String>,
}

/// Compares `hs.decoded_hash` (each client's hash of the state it holds, under the tick it rebuilt; the sentinel 0 for
/// a tick with a group it refused; `null` for none) with `hs.server_hash` (the server's hash of what it meant that
/// client to hold), over every phase. A server-only (client, tick) is loss, never compared; a client hash at a
/// (slot, tick) the server never logged counts on every machine. The Arm rule (ADR 0001, -17's key; the advisor's
/// FX-5): on an LG client's log on aarch64, in a run whose room log is on x86_64, a mismatch against a server hash is
/// checked against `hs.replay_hash` (its logged bytes decoded again by a `replay` log on x86_64): it counts when the
/// replay's entry differs from the server's too; a replay log with no entry there is a gap.
///
/// Structural gaps, returned at once: either name missing from every log, a name in a log of another process
/// (`source`), a list that does not hold the run's `clients` entries (the room log's header; every `hs` list and every
/// client log's `tick.s.snap_incomplete` list), a `snap_incomplete` entry other than 0 or 1, two logs at one
/// (slot, tick), or a server hash of 0 (the hash is never 0, so 0 is the client's sentinel alone). The coverage rule
/// (README §1's `null` rule, the advisor's FX-10) is kept for after the count: a (slot, tick) is resolved by a client
/// hash (compared) or by `snap_incomplete` 1 (lost, not compared), never both; per session of a slot (`coverage`:
/// cut at the room log's `admit` records), every tick from its first resolved tick to its last at which the room
/// logged that slot's hash is resolved, and that span covers the room's measured hashes of the session.
fn decoded(run: &Run<'_>) -> Result<Decoded, Lack> {
    let missing = |name: &str| {
        Lack::Gap(format!(
            "run {}: no log gives hs.{name} (decoded_hash_mismatch_count's input)",
            run.id
        ))
    };
    let (n_server, server) =
        slot_hashes(run, "server_hash")?.ok_or_else(|| missing("server_hash"))?;
    let (n_client, client) =
        slot_hashes(run, "decoded_hash")?.ok_or_else(|| missing("decoded_hash"))?;
    let replay = slot_hashes(run, "replay_hash")?;
    let rooms: Vec<&npt_log::Header> = run
        .logs
        .iter()
        .map(|l| &l.header)
        .filter(|h| h.process == "room")
        .collect();
    let counts: BTreeSet<u32> = rooms.iter().map(|h| h.clients).collect();
    let clients = match counts.iter().collect::<Vec<_>>().as_slice() {
        [c] => usize::try_from(**c).unwrap_or(usize::MAX),
        _ => {
            return Err(Lack::Gap(format!(
                "run {}: its room logs disagree on clients",
                run.id
            )));
        }
    };
    // Every list holds the run's clients entries (FX-10 (a)).
    let lengths = [
        ("server_hash", Some(n_server)),
        ("decoded_hash", Some(n_client)),
        ("replay_hash", replay.as_ref().map(|(n, _)| *n)),
    ];
    for (name, n) in lengths {
        if let Some(n) = n.filter(|n| *n != clients) {
            return Err(Lack::Gap(format!(
                "run {}: hs.{name} lists hold {n} entries, not the run's {clients} clients (the room log's header)",
                run.id
            )));
        }
    }
    let (_, client_writes) = source("decoded_hash");
    let mut incomplete: BTreeSet<(usize, u64)> = BTreeSet::new();
    for log in run.logs.iter().filter(|l| client_writes(&l.header)) {
        for t in &log.ticks {
            let Some(list) = t.s.get("snap_incomplete") else {
                continue;
            };
            if list.len() != clients {
                return Err(Lack::Gap(format!(
                    "run {}: tick {}'s tick.s.snap_incomplete in the {} log on {} holds {} entries, not the run's \
                     {clients} clients",
                    run.id,
                    t.t,
                    log.header.process,
                    log.header.machine.name,
                    list.len()
                )));
            }
            for (slot, v) in list.iter().enumerate() {
                match v {
                    0 => {}
                    1 => {
                        incomplete.insert((slot, t.t));
                    }
                    _ => {
                        return Err(Lack::Gap(format!(
                            "run {}: tick {}'s tick.s.snap_incomplete holds {v} for slot {slot}, not 0 or 1",
                            run.id, t.t
                        )));
                    }
                }
            }
        }
    }
    for (t, list) in &server {
        if let Some(slot) = list.iter().position(|e| matches!(e, Some((0, _)))) {
            return Err(Lack::Gap(format!(
                "run {}: the server logged 0 for slot {slot} at tick {t}; its hash is never 0",
                run.id
            )));
        }
    }
    let at = |map: &SlotHashes, t: u64, slot: usize| {
        map.get(&t)
            .and_then(|l| l.get(slot).copied().flatten())
            .map(|(h, _)| h)
    };
    // The Arm rule needs the run's room on x86_64 (the server's hash is the x86-64 reference).
    let room_x86 = rooms.iter().all(|h| h.machine.arch == "x86_64");
    let mut d = Decoded {
        compared: 0,
        mismatched: 0,
        cross_arch: 0,
        gap: None,
        waiting: None,
    };
    for (t, list) in &client {
        for (slot, entry) in list.iter().enumerate() {
            let Some((h, lg_arm)) = *entry else { continue };
            d.compared += 1;
            let theirs = at(&server, *t, slot);
            if theirs == Some(h) {
                continue;
            }
            // An orphan (no server hash there) counts on every machine: the replay has nothing to reproduce.
            if theirs.is_none() || !(lg_arm && room_x86) {
                d.mismatched += 1;
                continue;
            }
            let Some((_, rep)) = &replay else {
                d.waiting.get_or_insert_with(|| {
                    format!(
                        "run {}: slot {slot}'s LG aarch64 hash at tick {t} differs, and the run has no x86-64 \
                         re-decode (hs.replay_hash, process replay) yet",
                        run.id
                    )
                });
                continue;
            };
            match at(rep, *t, slot) {
                None => {
                    d.gap.get_or_insert_with(|| {
                        format!(
                            "run {}: slot {slot}'s LG aarch64 hash at tick {t} differs from the server's, and the \
                             x86-64 re-decode has no entry there (an incomplete re-decode)",
                            run.id
                        )
                    });
                }
                Some(r) if Some(r) == theirs => d.cross_arch += 1,
                Some(_) => d.mismatched += 1,
            }
        }
    }
    if let Some(g) = coverage(run, &server, &client, &incomplete) {
        d.gap.get_or_insert(g);
    }
    Ok(d)
}

/// Per slot, the ticks of one room-side session event (C-U6-11): `admit` in the room's log, `session_end` in the
/// room's log (L2) or the lab's (L1). A record without `slot` or `t` is not read, which only makes the coverage rule
/// stricter (a slot with one session fewer, or an end not recorded); a client-side log's record never counts.
fn session_ticks(run: &Run<'_>, ev: &str, writers: &[&str]) -> BTreeMap<usize, BTreeSet<u64>> {
    let mut out: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
    let logs = run.logs.iter();
    let logs = logs.filter(|l| writers.contains(&l.header.process.as_str()));
    for e in logs.flat_map(|l| &l.events).filter(|e| e.ev == ev) {
        let slot = e.f.get("slot").and_then(Value::as_u64);
        if let (Some(slot), Some(t)) = (slot, e.t) {
            let slot = usize::try_from(slot).unwrap_or(usize::MAX);
            out.entry(slot).or_default().insert(t);
        }
    }
    out
}

/// One session of a slot under the coverage rule: its ticks `from..to` (from its `admit` to the slot's next; the last
/// to the end), and the first and last tick in them the client side resolves. A slot with no `admit` record has one
/// session over every tick, `cut` false: the rule as it was before C-U6-11.
struct Session {
    from: u64,
    to: u64,
    span: Option<(u64, u64)>,
    cut: bool,
}

fn sessions(admits: Option<&BTreeSet<u64>>, resolved: Option<&BTreeSet<u64>>) -> Vec<Session> {
    let starts: Vec<u64> = admits.map_or_else(|| vec![0], |a| a.iter().copied().collect());
    let ends = starts.iter().skip(1).copied().chain([u64::MAX]);
    let session = |(from, to): (&u64, u64)| {
        let mut ticks = resolved.into_iter().flat_map(|r| r.range(*from..to));
        let first = ticks.next().copied();
        Session {
            from: *from,
            to,
            span: first.map(|f| (f, ticks.next_back().copied().unwrap_or(f))),
            cut: admits.is_some(),
        }
    };
    starts.iter().zip(ends).map(session).collect()
}

/// README §1's `null` rule on one run (the advisor's FX-10 (b) to (d); per session since C-U6-11): the first
/// (slot, tick) the client side leaves unresolved, or resolves twice. `incomplete` holds the (slot, tick) pairs a
/// client log marks `snap_incomplete` 1. A slot's sessions are cut at its `admit` records' ticks; in each, (c) every
/// tick from its first resolved tick to its last at which the room logged the slot's hash is resolved, and (d) that
/// span covers the room's measured hashes of the session. One tail is allowed: a session before the slot's last may
/// leave the room's hashes after its last resolved tick unresolved (in flight when it left), up to its recorded
/// `session_end` and never past it. A slot with `admit` records has no measured room hash before the first.
fn coverage(
    run: &Run<'_>,
    server: &SlotHashes,
    client: &SlotHashes,
    incomplete: &BTreeSet<(usize, u64)>,
) -> Option<String> {
    let mut resolved: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
    for (t, list) in client {
        for (slot, e) in list.iter().enumerate() {
            if e.is_some() {
                if incomplete.contains(&(slot, *t)) {
                    return Some(format!(
                        "run {}: slot {slot}, tick {t}: a client hash and tick.s.snap_incomplete 1 at once",
                        run.id
                    ));
                }
                resolved.entry(slot).or_default().insert(*t);
            }
        }
    }
    for (slot, t) in incomplete {
        resolved.entry(*slot).or_default().insert(*t);
    }
    let admits = session_ticks(run, "admit", &["room"]);
    let ended = session_ticks(run, "session_end", &["room", "lab"]);
    let mut measured: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
    for log in run.logs.iter().filter(|l| l.header.process == "room") {
        for t in log.ticks.iter().filter(|t| t.ph == Phase::Measured) {
            let Some(list) = t.hashes("server_hash") else {
                continue;
            };
            for (slot, _) in list.iter().enumerate().filter(|(_, e)| e.is_some()) {
                measured.entry(slot).or_default().insert(t.t);
            }
        }
    }
    let slots: BTreeSet<usize> = resolved.keys().chain(measured.keys()).copied().collect();
    let of: BTreeMap<usize, Vec<Session>> = slots
        .iter()
        .map(|s| (*s, sessions(admits.get(s), resolved.get(s))))
        .collect();
    // (c): inside a session's span, every tick the room logged a hash for the slot is resolved.
    for (t, list) in server {
        for (slot, e) in list.iter().enumerate() {
            let session = of
                .get(&slot)
                .and_then(|v| v.iter().find(|x| x.from <= *t && *t < x.to));
            let inside = session
                .and_then(|x| x.span)
                .is_some_and(|(a, b)| a <= *t && *t <= b);
            let done = resolved.get(&slot).is_some_and(|r| r.contains(t));
            if e.is_some() && inside && !done {
                return Some(format!(
                    "run {}: slot {slot}, tick {t}: the room logged a hash and no client log resolves it",
                    run.id
                ));
            }
        }
    }
    // (d): every session the room logged at a measured tick has a span covering its first and last measured hash.
    for (slot, ticks) in &measured {
        let list = of.get(slot).map_or(&[][..], Vec::as_slice);
        let who = |x: &Session| match x.cut {
            true => format!(
                "run {}: slot {slot}, its session admitted at tick {}",
                run.id, x.from
            ),
            false => format!("run {}: slot {slot}", run.id),
        };
        let early = list.first().filter(|x| x.cut).zip(ticks.first());
        if let Some((first, t)) = early.filter(|(first, t)| **t < first.from) {
            return Some(format!(
                "run {}: slot {slot}: the room logged its hash at measured tick {t}, before the slot's first admit \
                 (tick {})",
                run.id, first.from
            ));
        }
        for (i, x) in list.iter().enumerate() {
            let mut of_session = ticks.range(x.from..x.to);
            let Some(a) = of_session.next().copied() else {
                continue;
            };
            let b = of_session.next_back().copied().unwrap_or(a);
            let Some((c, e)) = x.span else {
                return Some(format!(
                    "{}: the room logged its hash at measured ticks {a} to {b}, and no client log resolves any tick \
                     of it",
                    who(x)
                ));
            };
            // The tail a session that left may leave: only before the slot's next admit, only up to its recorded end.
            let last = i + 1 == list.len();
            let end = ended.get(slot).and_then(|v| v.range(x.from..x.to).next());
            let left = !last && end.is_some_and(|end| b <= *end);
            if c > a || (e < b && !left) {
                let tail = match end {
                    _ if !x.cut || e >= b => String::new(),
                    _ if last => " (the slot's last session: no tail is allowed)".to_owned(),
                    None => " (no session_end records that it left)".to_owned(),
                    Some(end) => format!(" (its session_end is at tick {end})"),
                };
                return Some(format!(
                    "{}: the client side resolves ticks {c} to {e} only, not the room's measured ticks {a} to {b}{tail}",
                    who(x)
                ));
            }
        }
    }
    None
}

/// C62H-BOUND's one room-side reader, shared by its two figures (SERIES.md's `snap_lost_unexplained_count` and
/// `snap_whole_despite_link_loss_count` rows): the room's record per tick, and the clients' measured tick records.
struct Bound {
    /// Each room tick's `tick.s.snap_link_lost_count`, one entry per client slot.
    link: BTreeMap<u64, Vec<i64>>,
    /// The room's `hs.server_hash`: a non-null entry is a snapshot or a full state it sent that slot.
    server: SlotHashes,
    /// The measured (slot, tick) pairs filed lost: `tick.s.snap_incomplete` 1.
    lost: BTreeSet<(usize, u64)>,
    /// The measured (slot, tick) pairs filed whole: an `hs.decoded_hash` neither null nor the sentinel 0.
    whole: Vec<(usize, u64)>,
}

impl Bound {
    /// Whether the room sent `slot` a snapshot or a full state at tick `t`: a non-null `hs.server_hash` there.
    fn sent(&self, slot: usize, t: u64) -> bool {
        let entry = self.server.get(&t).and_then(|l| l.get(slot));
        entry.is_some_and(Option::is_some)
    }

    /// `slot`'s `tick.s.snap_link_lost_count` at tick `t`; `None` where the room has no record of `t`.
    fn link(&self, slot: usize, t: u64) -> Option<i64> {
        self.link.get(&t)?.get(slot).copied()
    }
}

/// [`Bound`]'s gaps, in the frozen row's order and words (SERIES.md's `snap_lost_unexplained_count` row).
const LINK_GAPS: [&str; 8] = [
    "no room log gives tick.s.snap_link_lost_count",
    "a room tick record without tick.s.snap_link_lost_count",
    "a room tick record without hs.server_hash",
    "a list that does not hold the run's clients entries",
    "an entry below 0",
    "a measured client tick record without tick.s.snap_incomplete",
    "a snap_incomplete entry other than 0 or 1",
    "a measured incomplete tick the room log has no record of",
];

/// Reads [`Bound`] from every room tick record and the clients' (`source`) measured ones, or a gap: `slot_hashes`'
/// own refusals and room logs that disagree on `clients` (as `decoded`'s), then the first of [`LINK_GAPS`] in the
/// row's order, at its first tick and slot. A list's length is the room logs' header `clients`.
fn bound(run: &Run<'_>) -> Result<Bound, Lack> {
    const LINK: &str = "snap_link_lost_count";
    let server = slot_hashes(run, "server_hash")?;
    let decoded = slot_hashes(run, "decoded_hash")?;
    let ((_, room), (_, client)) = (source("server_hash"), source("decoded_hash"));
    let rooms: Vec<_> = run.logs.iter().filter(|l| room(&l.header)).collect();
    let counts: BTreeSet<u32> = rooms.iter().map(|l| l.header.clients).collect();
    if counts.len() > 1 {
        let why = format!("run {}: its room logs disagree on clients", run.id);
        return Err(Lack::Gap(why));
    }
    let clients = counts.first().map_or(0, |c| *c as usize);
    let mut gaps: [Option<String>; 8] = Default::default();
    let mut gap = |i: usize, at: String| {
        gaps[i].get_or_insert(format!("{at}{}", LINK_GAPS[i]));
    };
    for (name, h) in [("hs.server_hash", &server), ("hs.decoded_hash", &decoded)] {
        if let Some((n, _)) = h.as_ref().filter(|(n, _)| *n != clients) {
            gap(3, format!("{name} lists hold {n} entries, not {clients}: "));
        }
    }
    let mut link = BTreeMap::new();
    for t in rooms.iter().flat_map(|l| &l.ticks) {
        if !t.hs.contains_key("server_hash") {
            gap(2, format!("tick {}: ", t.t));
        }
        let Some(l) = t.s.get(LINK) else {
            gap(1, format!("tick {}: ", t.t));
            continue;
        };
        if l.len() != clients {
            let at = format!("tick {}'s tick.s.{LINK} holds {} entries: ", t.t, l.len());
            gap(3, at);
        }
        if let Some((slot, v)) = l.iter().enumerate().find(|(_, v)| **v < 0) {
            let at = format!("tick {}'s tick.s.{LINK} gives slot {slot} {v}: ", t.t);
            gap(4, at);
        }
        link.insert(t.t, l.clone());
    }
    if link.is_empty() {
        gap(0, String::new());
    }
    let (mut lost, mut ticks) = (BTreeSet::new(), BTreeSet::new());
    let client_logs = run.logs.iter().filter(|l| client(&l.header));
    for t in client_logs.flat_map(|l| &l.ticks) {
        if t.ph != Phase::Measured {
            continue;
        }
        ticks.insert(t.t);
        let Some(f) = t.s.get("snap_incomplete") else {
            gap(5, format!("tick {}: ", t.t));
            continue;
        };
        if f.len() != clients {
            let at = format!(
                "tick {}'s tick.s.snap_incomplete holds {} entries: ",
                t.t,
                f.len()
            );
            gap(3, at);
        }
        for (slot, v) in f.iter().enumerate() {
            match v {
                0 => {}
                1 => {
                    lost.insert((slot, t.t));
                }
                _ => gap(6, format!("tick {}, slot {slot} holds {v}: ", t.t)),
            }
        }
    }
    if let Some((slot, t)) = lost.iter().find(|(_, t)| !link.contains_key(t)) {
        gap(7, format!("slot {slot}, tick {t}: "));
    }
    if let Some(g) = gaps.into_iter().flatten().next() {
        return Err(Lack::Gap(format!("run {}: {g}", run.id)));
    }
    let mut whole = Vec::new();
    for (t, list) in decoded.map(|(_, m)| m).unwrap_or_default() {
        for (slot, h) in list.into_iter().enumerate() {
            if ticks.contains(&t) && h.is_some_and(|(h, _)| h != 0) {
                whole.push((slot, t));
            }
        }
    }
    let server = server.map(|(_, m)| m).unwrap_or_default();
    Ok(Bound {
        link,
        server,
        lost,
        whole,
    })
}

/// PM-0001-17 (C62H-BOUND (a)): the measured client-ticks filed lost that neither the room's record explains (a null
/// `hs.server_hash`: it sent that slot nothing) nor the link's (`tick.s.snap_link_lost_count` 1 or more: npt-l1 lost
/// a snapshot datagram of it). Never empty: 0 incomplete is a real 0.
fn snap_lost_unexplained_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let b = bound(run)?;
    let mut n = 0;
    for &(slot, t) in &b.lost {
        n += i128::from(b.sent(slot, t) && b.link(slot, t) == Some(0));
    }
    Ok(Ratio::int(n))
}

/// PM-0001-17, the link record's own check (C62H-BOUND (a)): the measured client-ticks filed whole at which the room's
/// record gives that slot `tick.s.snap_link_lost_count` 1 or more. A tick the room has no record of is not read here
/// (`decoded_hash_mismatch_count` counts it). Never empty.
fn snap_whole_despite_link_loss_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let b = bound(run)?;
    let mut n = 0;
    for &(slot, t) in &b.whole {
        n += i128::from(b.link(slot, t).is_some_and(|l| l >= 1));
    }
    Ok(Ratio::int(n))
}

/// PM-0001-17, -19 and -58: the (client, tick) mismatches `decoded` counts, in the advisor's FX-10 (f) order: the
/// structural gaps first (`decoded` returns them), then any counted mismatch gives its count (a FAIL), then 0 compared
/// is `empty` (fails, never passes), then a coverage gap or an incomplete re-decode, then an Arm-rule mismatch waiting
/// for its re-decode (NOT-RUN), then 0.
fn decoded_hash_mismatch_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let d = decoded(run)?;
    if d.mismatched > 0 {
        return Ok(Ratio::int(d.mismatched));
    }
    if d.compared == 0 {
        return Err(Lack::Empty(format!(
            "run {}: no client logged a hash (0 compared)",
            run.id
        )));
    }
    if let Some(g) = d.gap {
        return Err(Lack::Gap(g));
    }
    if let Some(w) = d.waiting {
        return Err(Lack::Waiting(w));
    }
    Ok(Ratio::int(0))
}

/// PM-0001-17, reported: the client hashes `decoded` looked at.
fn decoded_hash_compared_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    Ok(Ratio::int(decoded(run)?.compared))
}

/// The Arm rule's finding, reported: LG aarch64 client hashes that differ from the server's while their x86-64
/// re-decode agrees with it. The hash kind only: the shot view test's and the throw landing's kinds join with the
/// figures that read them (U4h's view_hit_registered_share, false_hit_share and throw_desync_count), so a run without
/// decoded hashes is a gap here, never a 0; NOT-RUN while an Arm-rule mismatch waits for its re-decode.
fn cross_arch_mismatch_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let d = decoded(run)?;
    match d.waiting {
        Some(w) => Err(Lack::Waiting(w)),
        None => Ok(Ratio::int(d.cross_arch)),
    }
}

/// PM-0001-1 and PM-0006-11: 1 when every log's header names its machine in full (role, name, plan, arch, cpu,
/// location and kernel not empty; vcpus, mem_mb, threads_per_core and clock_res_ns above 0; the SMT map with every
/// sibling list filled) and every VM (RS, LG, MV) gives its day address; else 0.
fn machine_named(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let named = run.logs.iter().all(|log| {
        let m = &log.header.machine;
        let texts = [
            &m.role,
            &m.name,
            &m.plan,
            &m.arch,
            &m.cpu,
            &m.location,
            &m.kernel,
        ];
        let vm = matches!(m.role.as_str(), "RS" | "LG" | "MV");
        texts.iter().all(|t| !t.is_empty())
            && m.vcpus > 0
            && m.mem_mb > 0
            && m.threads_per_core > 0
            && m.clock_res_ns > 0
            && !m.smt.is_empty()
            && m.smt.iter().all(|s| !s.is_empty())
            && (!vm || !m.address.is_empty())
    });
    Ok(Ratio::int(i128::from(named)))
}

// U6j's figures over second records (`marks/SERIES.md`'s "Figures" rows give each one's formula and draft words).

/// The run's one room log: a gap if it has none, or more than one (these figures read one room's counters).
fn room_log<'r>(run: &'r Run<'_>) -> Result<&'r Log, Lack> {
    let mut rooms = run.logs.iter().filter(|l| l.header.process == "room");
    match (rooms.next(), rooms.next()) {
        (Some(log), None) => Ok(log),
        (None, _) => Err(Lack::Gap(format!("run {}: no room log", run.id))),
        _ => Err(Lack::Gap(format!("run {}: more than one room log", run.id))),
    }
}

/// A name only one process writes (`kind`: `second.v`, `second.s`, `tick.v`, `tick.s`, or `event` for an event's
/// name): in a log of another process it is a gap, as for the `hs` names (the advisor's FX-5 (a)).
fn only_from(run: &Run<'_>, process: &str, kind: &str, name: &str) -> Result<(), Lack> {
    let has = |l: &Log| match kind {
        "second.v" => l.seconds.iter().any(|s| s.v.contains_key(name)),
        "second.s" => l.seconds.iter().any(|s| s.s.contains_key(name)),
        "tick.v" => l.ticks.iter().any(|t| t.v.contains_key(name)),
        "event" => l.events.iter().any(|e| e.ev == name),
        _ => l.ticks.iter().any(|t| t.s.contains_key(name)),
    };
    let other = run
        .logs
        .iter()
        .find(|l| l.header.process != process && has(l));
    other.map_or(Ok(()), |l| {
        Err(Lack::Gap(format!(
            "run {}: {kind}.{name} in the {} log on {}: only process {process} writes it",
            run.id, l.header.process, l.header.machine.name
        )))
    })
}

/// One series of the room log's measured second records, by `sec` (`get` reads it off a record). Gaps: the name in
/// another process's log, a measured second without it (every second record carries its counters, 0 included:
/// README §2.3), a second logged twice, no measured second.
fn room_seconds<'r, T>(
    run: &'r Run<'_>,
    kind: &str,
    name: &str,
    get: impl Fn(&'r SecondRecord) -> Option<T>,
) -> Result<BTreeMap<u64, T>, Lack> {
    only_from(run, "room", kind, name)?;
    let mut out = BTreeMap::new();
    let seconds = room_log(run)?.seconds.iter();
    for s in seconds.filter(|s| s.ph == Phase::Measured) {
        let Some(v) = get(s) else {
            return Err(Lack::Gap(format!(
                "run {}: measured second {} of the room log has no {kind}.{name}",
                run.id, s.sec
            )));
        };
        if out.insert(s.sec, v).is_some() {
            return Err(Lack::Gap(format!(
                "run {}: the room log has measured second {} twice",
                run.id, s.sec
            )));
        }
    }
    match out.is_empty() {
        true => Err(Lack::Gap(format!(
            "run {}: no measured second in the room log ({kind}.{name})",
            run.id
        ))),
        false => Ok(out),
    }
}

/// A `second.s` series of the room log's measured seconds. Every list holds as many entries as the first, and for
/// a series with one entry per client (`per_client`) the run's `clients` (the room log's header): else a gap.
fn room_lists<'r>(
    run: &'r Run<'_>,
    name: &str,
    per_client: bool,
) -> Result<BTreeMap<u64, &'r [i64]>, Lack> {
    let lists = room_seconds(run, "second.s", name, |s| s.s.get(name).map(Vec::as_slice))?;
    let clients = usize::try_from(room_log(run)?.header.clients).unwrap_or(usize::MAX);
    let first = lists.values().next().map_or(0, |l| l.len());
    let (want, of) = match per_client {
        true => (clients, "the run's clients (the room log's header)"),
        false => (first, "as its first measured second"),
    };
    match lists.iter().find(|(_, l)| l.len() != want) {
        Some((sec, l)) => Err(Lack::Gap(format!(
            "run {}: measured second {sec}'s second.s.{name} holds {} entries, not {want}, {of}",
            run.id,
            l.len()
        ))),
        None => Ok(lists),
    }
}

/// -12's and -13's reading of "per client": the largest, over client slots, of the slot's mean over the measured
/// seconds of a per-client series.
fn slot_mean_max(run: &Run<'_>, name: &str) -> Result<Ratio, Lack> {
    let lists = room_lists(run, name, true)?;
    let clients = lists.values().next().map_or(0, |l| l.len());
    let sum = |slot: usize| lists.values().map(|l| i128::from(l[slot])).sum::<i128>();
    match (0..clients).map(sum).max() {
        Some(most) => Ok(Ratio::new(most, lists.len() as i128)),
        None => Err(Lack::Gap(format!(
            "run {}: second.s.{name} lists hold no client",
            run.id
        ))),
    }
}

/// PM-0001-12's IPv6 report: the mean over client-seconds of the downstream with IPv6's 20 B more per datagram.
fn down_mean_ipv6_bits(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let total = |name: &str| -> Result<(i128, i128), Lack> {
        let lists = room_lists(i.run, name, true)?;
        let all = lists.values().flat_map(|l| l.iter());
        let sum = all.map(|v| i128::from(*v)).sum::<i128>();
        Ok((sum, lists.values().map(|l| l.len() as i128).sum()))
    };
    let (bits, client_seconds) = total("down_bits")?;
    let (datagrams, _) = total("down_datagrams_count")?;
    match client_seconds {
        0 => Err(Lack::Gap(format!(
            "run {}: second.s.down_bits lists hold no client",
            i.run.id
        ))),
        n => Ok(Ratio::new(bits + 160 * datagrams, n)),
    }
}

/// The run's logs of one process other than the room (`bots`: LG's clients): a gap if it has none.
fn logs_of<'r>(run: &'r Run<'_>, process: &str) -> Result<Vec<&'r Log>, Lack> {
    let logs = run.logs.iter().filter(|l| l.header.process == process);
    let logs: Vec<&Log> = logs.copied().collect();
    match logs.is_empty() {
        true => Err(Lack::Gap(format!("run {}: no {process} log", run.id))),
        false => Ok(logs),
    }
}

/// PM-0001-38: the p99 of the control stream's message delays with a bulk transfer in flight minus the p99 with
/// none, each over every sample of LG's bots logs' measured seconds. No sample of either kind is a gap, not a 0.
fn bulk_control_p99_rise_ns(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let p99 = |name: &str| -> Result<Ratio, Lack> {
        only_from(i.run, "bots", "second.s", name)?;
        let logs = logs_of(i.run, "bots")?;
        let seconds = logs.iter().flat_map(|l| &l.seconds);
        let lists = seconds.filter(|s| s.ph == Phase::Measured);
        let samples: Vec<i64> = lists
            .filter_map(|s| s.s.get(name))
            .flatten()
            .copied()
            .collect();
        match samples.is_empty() {
            true => Err(Lack::Gap(format!(
                "run {}: no second.s.{name} sample in the measured seconds of LG's bots logs",
                i.run.id
            ))),
            false => Ok(stat_of(&Stat::Pct(Ratio::new(99, 100)), None, &samples)),
        }
    };
    Ok(p99("control_delay_bulk_ns")? - p99("control_delay_idle_ns")?)
}

/// The sum of a `second.v` counter over every second record, any phase, of one process's logs. Gaps: the name in
/// another process's log, no such log, a second record without it, no second record.
fn second_total(run: &Run<'_>, process: &str, name: &str) -> Result<i128, Lack> {
    only_from(run, process, "second.v", name)?;
    let logs = logs_of(run, process)?;
    let mut values = logs
        .iter()
        .flat_map(|l| l.seconds.iter().map(move |s| (l, s)));
    let mut sum: Option<i128> = None;
    values.try_for_each(|(l, s)| match s.v.get(name) {
        Some(v) => {
            sum = Some(sum.unwrap_or(0) + i128::from(*v));
            Ok(())
        }
        None => Err(Lack::Gap(format!(
            "run {}: second {} of the {process} log on {} has no second.v.{name}",
            run.id, s.sec, l.header.machine.name
        ))),
    })?;
    sum.ok_or_else(|| {
        Lack::Gap(format!(
            "run {}: no second record in its {process} log (second.v.{name})",
            run.id
        ))
    })
}

/// PM-0001-38: the one-shot actions LG's clients sent minus those the room received, over the whole run.
fn one_shot_dropped_count(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let sent = second_total(i.run, "bots", "one_shots_sent_count")?;
    let received = second_total(i.run, "room", "one_shots_received_count")?;
    Ok(Ratio::int(sent - received))
}

/// PM-0001-15: the least-squares slope of the room's resident memory over its measured seconds, in bytes per 10
/// minutes (x is the record's `sec`, so a missing second moves nothing): 600 x (n Sxy - Sx Sy) / (n Sxx - Sx Sx),
/// exact. Fewer than two measured seconds give no slope: a gap.
fn rss_slope_bytes_per_10min(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let name = "rss_bytes";
    let rss = room_seconds(i.run, "second.v", name, |s| s.v.get(name).copied())?;
    let first = rss.keys().next().copied().unwrap_or(0);
    // Seconds counted from the first one keep the sums small: 2 hours at 1 GB stays near 1e23, far inside i128.
    let points = rss
        .iter()
        .map(|(sec, y)| (i128::from(sec - first), i128::from(*y)));
    let (mut n, mut sx, mut sy, mut sxx, mut sxy) = (0, 0, 0, 0, 0);
    for (x, y) in points {
        (n, sx, sy, sxx, sxy) = (n + 1, sx + x, sy + y, sxx + x * x, sxy + x * y);
    }
    match n * sxx - sx * sx {
        0 => Err(Lack::Gap(format!(
            "run {}: one measured second of second.v.rss_bytes gives no slope",
            i.run.id
        ))),
        den => Ok(Ratio::new(600 * (n * sxy - sx * sy), den)),
    }
}

/// PM-0001-15's "flat" rule over the room's measured seconds: the count of (series, index) pairs whose largest
/// value in the late window is above their largest in the early one. `windows` gives, from the first and last
/// measured `sec`, the early window's last second and the late window's first (by time, so a missing second record
/// moves no boundary), or `None` when the seconds are too few for two windows (`need`): a gap.
fn growth_count(
    run: &Run<'_>,
    series: &[(&str, bool)],
    need: &str,
    windows: fn(u64, u64) -> Option<(u64, u64)>,
) -> Result<Ratio, Lack> {
    let mut grown = 0;
    for (name, per_client) in series {
        let lists = room_lists(run, name, *per_client)?;
        let first = lists.keys().next().copied().unwrap_or(0);
        let last = lists.keys().next_back().copied().unwrap_or(0);
        let Some((early, late)) = windows(first, last) else {
            return Err(Lack::Gap(format!(
                "run {}: measured seconds {first} to {last} of second.s.{name}: {need}",
                run.id
            )));
        };
        let most =
            |index: usize, from: u64, to: u64| lists.range(from..=to).map(|(_, l)| l[index]).max();
        let width = lists.values().next().map_or(0, |l| l.len());
        let grew = |index: &usize| most(*index, late, last) > most(*index, first, early);
        grown += (0..width).filter(grew).count() as i128;
    }
    Ok(Ratio::int(grown))
}

/// PM-0001-15: the per-client queues (outbound, unacknowledged events) whose largest length in the last 10 measured
/// minutes is above their largest in the first 10.
fn queue_growth_count(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let queues = [("out_queue_count", true), ("event_queue_count", true)];
    let need = "the first and the last 10 minutes need 1,200 measured seconds";
    growth_count(i.run, &queues, need, |first, last| {
        (last - first >= 1_199).then(|| (first + 599, last - 599))
    })
}

/// PM-0001-24: the same rule on the two halves of the measured seconds (the middle second of an odd count is the
/// first half's), over each session's buffered bytes and each client's two queues.
fn session_memory_growth_count(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let series = [
        ("session_buffered_bytes", false),
        ("out_queue_count", true),
        ("event_queue_count", true),
    ];
    let need = "two halves need 2 measured seconds";
    growth_count(i.run, &series, need, |first, last| {
        let mid = first + (last - first) / 2;
        (last > first).then_some((mid, mid + 1))
    })
}

/// PM-0001-10's unit cost of one run: mean server-only work per tick / (clients x mean relevant entities), over
/// the room log's measured ticks. Every `tick.s.relevant_count` list holds the run's `clients` entries, so that is
/// the sum of `tick.v.server_only_ns` over the sum of every entry, exactly. Gaps: no room log or more than one, a
/// name in another process's log, a measured tick without either series, a list of another length, no measured
/// tick, no relevant entity at all (nothing to divide by).
fn sync_unit(run: &Run<'_>) -> Result<Ratio, Lack> {
    let (work, relevant) = ("server_only_ns", "relevant_count");
    only_from(run, "room", "tick.v", work)?;
    only_from(run, "room", "tick.s", relevant)?;
    let room = room_log(run)?;
    let clients = usize::try_from(room.header.clients).unwrap_or(usize::MAX);
    let gap = |why: String| Err(Lack::Gap(format!("run {}: {why}", run.id)));
    let (mut ns, mut entities, mut ticks) = (0_i128, 0_i128, 0);
    for t in room.ticks.iter().filter(|t| t.ph == Phase::Measured) {
        let (Some(w), Some(list)) = (t.v.get(work), t.s.get(relevant)) else {
            return gap(format!(
                "measured tick {} of the room log lacks tick.v.{work} or tick.s.{relevant}",
                t.t
            ));
        };
        if list.len() != clients {
            return gap(format!(
                "tick {}'s tick.s.{relevant} holds {} entries, not the run's {clients} clients",
                t.t,
                list.len()
            ));
        }
        ns += i128::from(*w);
        entities += list.iter().map(|e| i128::from(*e)).sum::<i128>();
        ticks += 1;
    }
    match (ticks, entities) {
        (0, _) => gap(format!("no measured tick in the room log (tick.v.{work})")),
        (_, 0) => gap(format!(
            "tick.s.{relevant} sums to 0 over the measured ticks: no unit cost"
        )),
        _ => Ok(Ratio::new(ns, entities)),
    }
}

/// PM-0001-10: this run's unit cost against the mean unit cost of the valid runs at N = 24 and, separately, at
/// N = 36 (the mark's parts of those names); the larger of the two ratios. NOT-RUN while either part has fewer
/// valid runs than its `min`; a gap if a reference's mean is 0.
fn sync_cost_ratio(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let own = sync_unit(i.run)?;
    let mut worst: Option<Ratio> = None;
    for part in ["N=24", "N=36"] {
        let (min, runs) = i
            .parts
            .get(part)
            .map_or((1, &[][..]), |(m, r)| (*m, &r[..]));
        if runs.len() < min.max(1) {
            return Err(Lack::Waiting(format!(
                "run {}: part {part} has {} of its {min} valid runs: its mean unit cost is not there yet",
                i.run.id,
                runs.len()
            )));
        }
        let mut sum = Ratio::int(0);
        for r in runs {
            sum = sum + sync_unit(r)?;
        }
        if sum.num() == 0 {
            return Err(Lack::Gap(format!(
                "run {}: the mean unit cost of part {part} is 0",
                i.run.id
            )));
        }
        // own / (sum / n): every product checked, and the two ratios compared exactly at any size.
        let ratio = own * Ratio::new(sum.den(), sum.num()) * Ratio::int(runs.len() as i128);
        worst = Some(worst.map_or(ratio, |w| w.max(ratio)));
    }
    Ok(worst.unwrap_or(own))
}

/// A run's own counts for R2's worked bandwidth formula (docs/research/r2-netcode.md, E3: lines 1742-1766), all
/// from the room log's measured records, as the netcode-architect ruled their meaning (R2-INPUTS, 2026-11-24).
struct R2Counts {
    /// The header's `tick_hz`: the snapshot rate.
    hz: i128,
    clients: i128,
    /// E: the mean `tick.s.relevant_count` entry (the other soldiers and the big-vehicle hulls in a client's set).
    entities: Ratio,
    /// V: the mean `tick.v.big_vehicles_count`.
    vehicles: Ratio,
    /// S, R, J: the mean `second.v` `shots_count`, `reliable_events_count`, `projectile_spawns_count`.
    per_second: [Ratio; 3],
}

impl R2Counts {
    /// Gaps: no room log or more than one, a name in another process's log, a measured tick or second without one
    /// of its series, a `relevant_count` list that does not hold the run's `clients` entries, no measured tick or
    /// second, a run of 0 clients.
    fn of(run: &Run<'_>) -> Result<R2Counts, Lack> {
        let (relevant, big) = ("relevant_count", "big_vehicles_count");
        only_from(run, "room", "tick.s", relevant)?;
        only_from(run, "room", "tick.v", big)?;
        let room = room_log(run)?;
        let clients = i128::from(room.header.clients);
        let gap = |why: String| Err(Lack::Gap(format!("run {}: {why}", run.id)));
        let (mut entities, mut vehicles, mut ticks) = (0_i128, 0_i128, 0_i128);
        for t in room.ticks.iter().filter(|t| t.ph == Phase::Measured) {
            let (Some(list), Some(v)) = (t.s.get(relevant), t.v.get(big)) else {
                return gap(format!(
                    "measured tick {} of the room log lacks tick.s.{relevant} or tick.v.{big}",
                    t.t
                ));
            };
            if list.len() as i128 != clients {
                return gap(format!(
                    "tick {}'s tick.s.{relevant} holds {} entries, not the run's {clients} clients",
                    t.t,
                    list.len()
                ));
            }
            entities += list.iter().map(|e| i128::from(*e)).sum::<i128>();
            vehicles += i128::from(*v);
            ticks += 1;
        }
        if ticks == 0 || clients == 0 {
            return gap(format!(
                "no measured tick in the room log, or 0 clients (tick.s.{relevant})"
            ));
        }
        let mean = |name: &str| -> Result<Ratio, Lack> {
            let values = room_seconds(run, "second.v", name, |s| s.v.get(name).copied())?;
            let sum = values.values().map(|v| i128::from(*v)).sum::<i128>();
            Ok(Ratio::new(sum, values.len() as i128))
        };
        Ok(R2Counts {
            hz: i128::from(room.header.tick_hz),
            clients,
            entities: Ratio::new(entities, ticks * clients),
            vehicles: Ratio::new(vehicles, ticks),
            per_second: [
                mean("shots_count")?,
                mean("reliable_events_count")?,
                mean("projectile_spawns_count")?,
            ],
        })
    }

    /// Bits per second to one client: 8 x (hz x (90 + 7.5 x (E - V) + 9.4 x V) + 4 S + 24 R + 11 J) bytes, which is
    /// 4/5 x hz x (900 + 75 E + 19 V) + 32 S + 192 R + 88 J. Per snapshot 90 B fixed (57 transport, 8 header, 16
    /// own state, 7 structures, 2 economy), 7.5 B per other soldier, 9.4 B per vehicle; 4 B per shot, 8 B x 3 sends
    /// per reliable event, 11 B per projectile spawn.
    fn bits(&self) -> Ratio {
        let [shots, reliable, spawns] = self.per_second;
        let snapshot =
            Ratio::int(900) + Ratio::int(75) * self.entities + Ratio::int(19) * self.vehicles;
        Ratio::new(4 * self.hz, 5) * snapshot
            + Ratio::int(32) * shots
            + Ratio::int(192) * reliable
            + Ratio::int(88) * spawns
    }

    /// The arithmetic, for the reports that show it.
    fn text(&self) -> String {
        let [shots, reliable, spawns] = self.per_second;
        format!(
            "4/5 x {} Hz x (900 + 75 x {} + 19 x {}) + 32 x {shots} + 192 x {reliable} + 88 x {spawns}",
            self.hz, self.entities, self.vehicles
        )
    }
}

/// PM-0006-17: R2's formula for a room-hour, in bytes: the bits per second per client x clients x 3,600 / 8.
fn r2_egress_formula_bytes(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let counts = R2Counts::of(i.run)?;
    Ok(counts.bits() * Ratio::int(counts.clients * 450))
}

/// Its arithmetic, with the time between the interface counter's two reads (the formula takes 3,600 s as written).
fn r2_egress_detail(i: &Inputs<'_, '_>) -> Option<String> {
    let counts = R2Counts::of(i.run).ok()?;
    let reads = room_log(i.run)
        .ok()?
        .events
        .iter()
        .filter(|e| e.ev == "iface");
    let span = match reads.map(|e| e.ns).collect::<Vec<_>>()[..] {
        [Some(start), Some(end)] if end >= start => {
            format!(
                "{} s apart",
                Ratio::new(i128::from(end - start), 1_000_000_000)
            )
        }
        _ => "not timed (no ns on both)".to_owned(),
    };
    Some(format!(
        "({}) x {} clients x 3,600 s / 8; the interface counter's two reads are {span}",
        counts.text(),
        counts.clients
    ))
}

/// A counter the room logs twice, at the start and at the end of its window (`event.<ev>.<field>`, in record
/// order): how far it moved. Gaps: the event in another process's log, no room log or more than one, not exactly
/// two such events, a field that is not a whole number, a counter that fell.
fn counter_growth(run: &Run<'_>, ev: &str, field: &str) -> Result<i128, Lack> {
    only_from(run, "room", "event", ev)?;
    let events = room_log(run)?.events.iter().filter(|e| e.ev == ev);
    let reads: Vec<Option<i64>> = events
        .map(|e| e.f.get(field).and_then(Value::as_i64))
        .collect();
    let why = match reads[..] {
        [Some(start), Some(end)] if end >= start => return Ok(i128::from(end) - i128::from(start)),
        [Some(start), Some(end)] => {
            format!("event.{ev}.{field} fell from {start} to {end} between its two reads")
        }
        [_, _] => format!("an event.{ev} record has no whole-number {field}"),
        _ => format!(
            "{} event.{ev} records in the room log, not the two reads (start and end)",
            reads.len()
        ),
    };
    Err(Lack::Gap(format!("run {}: {why}", run.id)))
}

/// PM-0006-17: the bytes RS's interface sent between its counter's two reads. A counter that did not move is a gap.
fn egress_interface_bytes(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    match counter_growth(i.run, "iface", "tx_bytes")? {
        0 => Err(Lack::Gap(format!(
            "run {}: event.iface.tx_bytes did not move between its two reads",
            i.run.id
        ))),
        bytes => Ok(Ratio::int(bytes)),
    }
}

/// PM-0006-17: the room's own bytes-sent metric over the same two moments, against the interface's bytes.
fn egress_metric_vs_interface_ratio(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let interface = egress_interface_bytes(i)?;
    let metric = counter_growth(i.run, "room_metric", "sent_bytes")?;
    Ok(Ratio::new(metric, interface.num()))
}

/// PM-0006-17: the interface's bytes against R2's formula at the run's own counts.
fn egress_vs_r2_ratio(i: &Inputs<'_, '_>) -> Result<Ratio, Lack> {
    let formula = r2_egress_formula_bytes(i)?;
    match formula.num() {
        0 => Err(Lack::Gap(format!(
            "run {}: R2's formula gives 0 bytes at the run's counts",
            i.run.id
        ))),
        num => Ok(egress_interface_bytes(i)? * Ratio::new(formula.den(), num)),
    }
}

/// A figure's computation over its [`Inputs`].
pub type Compute = fn(&Inputs<'_, '_>) -> Result<Ratio, Lack>;

/// The arithmetic behind a figure's value, for a report that must show it; `None` when its inputs are missing.
pub type Detail = fn(&Inputs<'_, '_>) -> Option<String>;

/// One listed figure.
pub struct Figure {
    pub name: &'static str,
    /// The computation, or `None` while it is not built.
    pub compute: Option<Compute>,
    /// The unit that builds it, for a figure not built yet.
    pub unit: &'static str,
    /// What it computes, in plain words.
    pub what: &'static str,
    /// The arithmetic printed beside its reported value, for the figures whose row asks for it.
    pub detail: Option<Detail>,
}

/// A figure U6j built, with no arithmetic to show.
const fn built(name: &'static str, compute: Compute, what: &'static str) -> Figure {
    Figure {
        name,
        compute: Some(compute),
        unit: "U6j",
        what,
        detail: None,
    }
}

const fn planned(name: &'static str, unit: &'static str, what: &'static str) -> Figure {
    Figure {
        name,
        compute: None,
        unit,
        what,
        detail: None,
    }
}

/// The list: every figure `marks.json` names (`marks/SERIES.md` gives each one's formula, inputs and draft words).
/// Each is built in the unit that first produces its inputs; until then its check or report is NOT-RUN.
pub const FIGURES: &[Figure] = &[
    Figure {
        name: "fate_log_twin_equal",
        compute: Some(|i| fate_log_twin_equal(i.run, i.set)),
        detail: None,
        unit: "U2",
        what: "the same seed gives the same packet-fate log (PM-0001-3)",
    },
    Figure {
        name: "everyone_in_view_min_share",
        compute: Some(|i| everyone_in_view_min_share(i.run, i.set)),
        detail: None,
        unit: "U1",
        what: "the least share of a slot's snapshots with all 47 others in its set (-2)",
    },
    Figure {
        name: "decoded_hash_mismatch_count",
        compute: Some(|i| decoded_hash_mismatch_count(i.run, i.set)),
        detail: None,
        unit: "U4j",
        what: "decoded snapshots whose hash differs from the server's (-17, -19, -58)",
    },
    Figure {
        name: "decoded_hash_compared_count",
        compute: Some(|i| decoded_hash_compared_count(i.run, i.set)),
        detail: None,
        unit: "U4j",
        what: "the (client, tick) pairs compared (-17)",
    },
    Figure {
        name: "snap_lost_unexplained_count",
        compute: Some(|i| snap_lost_unexplained_count(i.run, i.set)),
        detail: None,
        unit: "C62H-BOUND",
        what: "client-ticks filed lost that neither the room's record (no snapshot sent) nor the link's (a snapshot datagram lost) explains (-17)",
    },
    Figure {
        name: "snap_whole_despite_link_loss_count",
        compute: Some(|i| snap_whole_despite_link_loss_count(i.run, i.set)),
        detail: None,
        unit: "C62H-BOUND",
        what: "client-ticks filed whole at which the link's record lost a snapshot datagram: the record overstated (-17)",
    },
    Figure {
        name: "twin_hash_mismatch_count",
        compute: Some(|i| twin_hash_mismatch_count(i.run, i.set)),
        detail: None,
        unit: "U4j",
        what: "ticks whose hash differs from the twin run's (-21, -30)",
    },
    Figure {
        name: "cross_arch_mismatch_count",
        compute: Some(|i| cross_arch_mismatch_count(i.run, i.set)),
        detail: None,
        unit: "U4j",
        what: "aarch64 hash mismatches the x86-64 re-decode does not reproduce (the Arm rule; the hash kind)",
    },
    planned(
        "view_hit_registered_share",
        "U4",
        "shots hitting in the shooter's view that the server registered (-48, -49)",
    ),
    planned(
        "false_hit_share",
        "U4",
        "view misses registered as hits (-48)",
    ),
    planned(
        "rule_cap_set_by_rule",
        "U4",
        "the rule cap equals the picked option's P2 honest-rewind p99 in ticks (-49)",
    ),
    planned(
        "throw_desync_count",
        "U4",
        "throws that desync, the x86-64 replay agreeing (-54)",
    ),
    Figure {
        name: "machine_named",
        compute: Some(|i| machine_named(i.run, i.set)),
        detail: None,
        unit: "U4j",
        what: "every header names its machine in full (-1, PM-0006-11)",
    },
    built(
        "sync_unit_cost",
        |i| sync_unit(i.run),
        "mean server-only work per tick / (clients x mean relevant entities) (-10)",
    ),
    built(
        "sync_cost_ratio",
        sync_cost_ratio,
        "N = 48's unit cost against N = 24's and 36's (-10)",
    ),
    built(
        "down_client_mean_max_bits",
        |i| slot_mean_max(i.run, "down_bits"),
        "the largest client mean of the downstream (-12)",
    ),
    built(
        "down_mean_ipv6_bits",
        down_mean_ipv6_bits,
        "the downstream mean with IPv6's 20 B more per datagram (-12)",
    ),
    Figure {
        name: "r2_down_formula_bits",
        compute: Some(|i| Ok(R2Counts::of(i.run)?.bits())),
        unit: "U6j",
        what: "R2's downstream formula at the run's own counts (-12)",
        detail: Some(|i| R2Counts::of(i.run).ok().map(|c| c.text())),
    },
    built(
        "up_client_mean_max_bits",
        |i| slot_mean_max(i.run, "up_bits"),
        "the largest client mean of the upstream (-13)",
    ),
    built(
        "rss_slope_bytes_per_10min",
        rss_slope_bytes_per_10min,
        "the least-squares slope of RSS over the measured seconds (-15)",
    ),
    built(
        "queue_growth_count",
        queue_growth_count,
        "per-client queues whose last-10-minute max exceeds the first's (-15)",
    ),
    built(
        "session_memory_growth_count",
        session_memory_growth_count,
        "the same rule on per-session memory, over halves (-24)",
    ),
    planned(
        "kill_p99_ratio",
        "U6",
        "the other rooms' p99 after the SIGKILL against before (-27)",
    ),
    planned(
        "rtt_agreement_p99_ns",
        "U6",
        "p99 of |app RTT - quinn's rtt()| per second (-33)",
    ),
    built(
        "bulk_control_p99_rise_ns",
        bulk_control_p99_rise_ns,
        "the control stream's p99 delay with a bulk transfer minus without (-38)",
    ),
    built(
        "one_shot_dropped_count",
        one_shot_dropped_count,
        "one-shot actions sent minus received (-38)",
    ),
    planned(
        "crash_close_max_ns",
        "U6",
        "each client's RoomEnded or close after the tick panic, the slowest (-59)",
    ),
    built(
        "egress_interface_bytes",
        egress_interface_bytes,
        "interface bytes sent in the room-hour (PM-0006-17)",
    ),
    built(
        "egress_metric_vs_interface_ratio",
        egress_metric_vs_interface_ratio,
        "the room's bytes-sent metric against the interface counter (PM-0006-17)",
    ),
    Figure {
        name: "r2_egress_formula_bytes",
        compute: Some(r2_egress_formula_bytes),
        unit: "U6j",
        what: "R2's egress formula at the run's counts (PM-0006-17)",
        detail: Some(r2_egress_detail),
    },
    built(
        "egress_vs_r2_ratio",
        egress_vs_r2_ratio,
        "interface bytes against R2's formula (PM-0006-17)",
    ),
    planned(
        "logging_cost_ratio",
        "U6",
        "the pooled p99 with logging on against off (PM-0006-19)",
    ),
    Figure {
        name: "alloc_twin_diff_count",
        compute: Some(|i| alloc_twin_diff_count(i.run, i.set)),
        unit: "U6j",
        what: "ticks whose allocation count differs from the logging-off twin's (PM-0006-20)",
        detail: None,
    },
    planned(
        "one_build_hash",
        "U7",
        "every browser run used the same client build (-32)",
    ),
    planned(
        "untimed_trap_count",
        "U7",
        "Worker traps with no timed rejoin after them (-19, the advisor's Q12)",
    ),
    planned(
        "input_latency_p99_rise_ns",
        "U7",
        "input-to-screen p99 at P4 minus at P0, the worst kind (-43)",
    ),
    planned(
        "input_latency_p50_rise_ns",
        "U7",
        "the same with medians (-43)",
    ),
    planned(
        "rooms_per_vm_count",
        "U8",
        "per N, the lowest of the sessions' largest k whose every room meets the marks (PM-0006-14)",
    ),
    planned(
        "times_of_day_rule",
        "U8",
        "the sessions start 6 h apart, one between 18:00 and 23:00 Amsterdam (-27, PM-0006-13, -14)",
    ),
];

pub fn known(name: &str) -> bool {
    FIGURES.iter().any(|f| f.name == name)
}

/// The arithmetic behind a figure's value, where its row asks for it shown and its inputs are there.
pub(crate) fn detail(name: &str, inputs: &Inputs<'_, '_>) -> Option<String> {
    let figure = FIGURES.iter().find(|f| f.name == name)?;
    figure.detail.and_then(|d| d(inputs))
}

/// Computes a figure: a gap if its inputs are missing, not-built if its code is not there yet.
pub(crate) fn compute(name: &str, inputs: &Inputs<'_, '_>) -> Result<Ratio, Missing> {
    match FIGURES.iter().find(|f| f.name == name) {
        Some(Figure {
            compute: Some(f), ..
        }) => f(inputs).map_err(|lack| match lack {
            Lack::Gap(why) => Missing::Gap(why),
            Lack::Waiting(why) => Missing::NotBuilt(why),
            Lack::Empty(why) => Missing::Empty(why),
            Lack::Fail(why) => Missing::Fail(why),
        }),
        Some(Figure { unit, .. }) => Err(Missing::NotBuilt(format!(
            "figure {name} is not built yet ({unit})"
        ))),
        None => Err(Missing::NotBuilt(format!(
            "figure {name} is not in the judge's list"
        ))),
    }
}
