//! Figures: the computations a mark needs that README §2.3's statistics cannot express (ratios across scale steps,
//! slopes, arithmetic against a formula). Every name `marks.json` uses must be listed here; one that is listed but
//! not built yet says which unit builds it, and its check comes out NOT-RUN rather than guessed.

use crate::eval::{Missing, Run, always_rules};
use crate::ratio::Ratio;
use npt_log::Phase;
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
    let broken = always_rules(twin);
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
        return Err(Lack::Gap(format!(
            "run {}: {} has role {}, not twin",
            run.id, twin.id, twin.role
        )));
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
/// hash (compared) or by `snap_incomplete` 1 (lost, not compared), never both; per slot, every tick from its first
/// resolved tick to its last at which the room logged that slot's hash is resolved, and that span covers the room's
/// measured ticks for the slot. Ticks outside a slot's span are neither compared nor counted.
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

/// README §1's `null` rule on one run (the advisor's FX-10 (b) to (d)): the first (slot, tick) the client side leaves
/// unresolved, or resolves twice. `incomplete` holds the (slot, tick) pairs a client log marks `snap_incomplete` 1.
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
    let span = |slot: usize| {
        let ticks = resolved.get(&slot)?;
        Some((*ticks.first()?, *ticks.last()?))
    };
    // (c): inside a slot's span, every tick the room logged a hash for it is resolved.
    for (t, list) in server {
        for (slot, e) in list.iter().enumerate() {
            let inside = span(slot).is_some_and(|(a, b)| a <= *t && *t <= b);
            let done = resolved.get(&slot).is_some_and(|r| r.contains(t));
            if e.is_some() && inside && !done {
                return Some(format!(
                    "run {}: slot {slot}, tick {t}: the room logged a hash and no client log resolves it",
                    run.id
                ));
            }
        }
    }
    // (d): every slot the room logged at a measured tick has a span covering the room's first and last measured hash.
    let mut measured: BTreeMap<usize, (u64, u64)> = BTreeMap::new();
    for log in run.logs.iter().filter(|l| l.header.process == "room") {
        for t in log.ticks.iter().filter(|t| t.ph == Phase::Measured) {
            let Some(list) = t.hashes("server_hash") else {
                continue;
            };
            for (slot, e) in list.iter().enumerate() {
                if e.is_some() {
                    let r = measured.entry(slot).or_insert((t.t, t.t));
                    r.0 = r.0.min(t.t);
                    r.1 = r.1.max(t.t);
                }
            }
        }
    }
    for (slot, (a, b)) in measured {
        match span(slot) {
            None => {
                return Some(format!(
                    "run {}: slot {slot}: the room logged its hash at measured ticks {a} to {b}, and no client log \
                     resolves any tick of it",
                    run.id
                ));
            }
            Some((c, e)) if c > a || e < b => {
                return Some(format!(
                    "run {}: slot {slot}: the client side resolves ticks {c} to {e} only, not the room's measured \
                     ticks {a} to {b}",
                    run.id
                ));
            }
            Some(_) => {}
        }
    }
    None
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

/// A figure's computation: the run under judgement and every run of its set.
pub type Compute = fn(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack>;

/// One listed figure.
pub struct Figure {
    pub name: &'static str,
    /// The computation, or `None` while it is not built.
    pub compute: Option<Compute>,
    /// The unit that builds it, for a figure not built yet.
    pub unit: &'static str,
    /// What it computes, in plain words.
    pub what: &'static str,
}

const fn planned(name: &'static str, unit: &'static str, what: &'static str) -> Figure {
    Figure {
        name,
        compute: None,
        unit,
        what,
    }
}

/// The list: every figure `marks.json` names (`marks/SERIES.md` gives each one's formula, inputs and draft words).
/// Each is built in the unit that first produces its inputs; until then its check or report is NOT-RUN.
pub const FIGURES: &[Figure] = &[
    Figure {
        name: "fate_log_twin_equal",
        compute: Some(fate_log_twin_equal),
        unit: "U2",
        what: "the same seed gives the same packet-fate log (PM-0001-3)",
    },
    Figure {
        name: "everyone_in_view_min_share",
        compute: Some(everyone_in_view_min_share),
        unit: "U1",
        what: "the least share of a slot's snapshots with all 47 others in its set (-2)",
    },
    Figure {
        name: "decoded_hash_mismatch_count",
        compute: Some(decoded_hash_mismatch_count),
        unit: "U4j",
        what: "decoded snapshots whose hash differs from the server's (-17, -19, -58)",
    },
    Figure {
        name: "decoded_hash_compared_count",
        compute: Some(decoded_hash_compared_count),
        unit: "U4j",
        what: "the (client, tick) pairs compared (-17)",
    },
    Figure {
        name: "twin_hash_mismatch_count",
        compute: Some(twin_hash_mismatch_count),
        unit: "U4j",
        what: "ticks whose hash differs from the twin run's (-21, -30)",
    },
    Figure {
        name: "cross_arch_mismatch_count",
        compute: Some(cross_arch_mismatch_count),
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
        compute: Some(machine_named),
        unit: "U4j",
        what: "every header names its machine in full (-1, PM-0006-11)",
    },
    planned(
        "sync_unit_cost",
        "U6",
        "mean server-only work per tick / (clients x mean relevant entities) (-10)",
    ),
    planned(
        "sync_cost_ratio",
        "U6",
        "N = 48's unit cost against N = 24's and 36's (-10)",
    ),
    planned(
        "down_client_mean_max_bits",
        "U6",
        "the largest client mean of the downstream (-12)",
    ),
    planned(
        "down_mean_ipv6_bits",
        "U6",
        "the downstream mean with IPv6's 20 B more per datagram (-12)",
    ),
    planned(
        "r2_down_formula_bits",
        "U6",
        "R2's downstream formula at the run's own counts (-12)",
    ),
    planned(
        "up_client_mean_max_bits",
        "U6",
        "the largest client mean of the upstream (-13)",
    ),
    planned(
        "rss_slope_bytes_per_10min",
        "U6",
        "the least-squares slope of RSS over the measured seconds (-15)",
    ),
    planned(
        "queue_growth_count",
        "U6",
        "per-client queues whose last-10-minute max exceeds the first's (-15)",
    ),
    planned(
        "session_memory_growth_count",
        "U6",
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
    planned(
        "bulk_control_p99_rise_ns",
        "U6",
        "the control stream's p99 delay with a bulk transfer minus without (-38)",
    ),
    planned(
        "one_shot_dropped_count",
        "U6",
        "one-shot actions sent minus received (-38)",
    ),
    planned(
        "crash_close_max_ns",
        "U6",
        "each client's RoomEnded or close after the tick panic, the slowest (-59)",
    ),
    planned(
        "egress_interface_bytes",
        "U6",
        "interface bytes sent in the room-hour (PM-0006-17)",
    ),
    planned(
        "egress_metric_vs_interface_ratio",
        "U6",
        "the room's bytes-sent metric against the interface counter (PM-0006-17)",
    ),
    planned(
        "r2_egress_formula_bytes",
        "U6",
        "R2's egress formula at the run's counts (PM-0006-17)",
    ),
    planned(
        "egress_vs_r2_ratio",
        "U6",
        "interface bytes against R2's formula (PM-0006-17)",
    ),
    planned(
        "logging_cost_ratio",
        "U6",
        "the pooled p99 with logging on against off (PM-0006-19)",
    ),
    planned(
        "alloc_twin_diff_count",
        "U6",
        "ticks whose allocation count differs from the logging-off twin's (PM-0006-20)",
    ),
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

/// Computes a figure: a gap if its inputs are missing, not-built if its code is not there yet.
pub(crate) fn compute(name: &str, run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Missing> {
    match FIGURES.iter().find(|f| f.name == name) {
        Some(Figure {
            compute: Some(f), ..
        }) => f(run, set).map_err(|lack| match lack {
            Lack::Gap(why) => Missing::Gap(why),
            Lack::Waiting(why) => Missing::NotBuilt(why),
            Lack::Empty(why) => Missing::Empty(why),
        }),
        Some(Figure { unit, .. }) => Err(Missing::NotBuilt(format!(
            "figure {name} is not built yet ({unit})"
        ))),
        None => Err(Missing::NotBuilt(format!(
            "figure {name} is not in the judge's list"
        ))),
    }
}
