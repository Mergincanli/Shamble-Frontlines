//! PM-0001-18's WebSocket part (`marks/README.md` §2.5 and §4, the `ws-practice` rule): the run-structured bound's
//! machine-readable line, the practice run's outcome record, and the factor the judge applies to the bound. The rules
//! are the advisor's on Mustafa's WS-BAR (2026-11-07): one unjudged practice run per build, its slowdown s = M_p / M_m,
//! the numbers moved by f = max(1, s) only while s is within the tolerance, never set by hand.

use crate::eval::{Run, always_rules, nearest_rank};
use crate::marks::{BoundNumber, Check, Mark, Measure, Threshold};
use crate::ratio::Ratio;
use npt_log::{EventRecord, Phase};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeSet;

/// The tag the bound's machine-readable line starts with; a JSON object follows it.
pub(crate) const BOUND_TAG: &str = "npt-ws-bound v1 ";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBound {
    model: String,
    commit: String,
    runs_count: u64,
    client_bursts_count: u64,
    x_ns: i64,
    y_ns: i64,
    k_count: i64,
    z_ns: i64,
    m_m_ns: i64,
    band_low: String,
    band_high: String,
    tolerance: String,
    guard_share: String,
}

/// The run-structured bound, as its machine-readable line gives it (README §2.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Bound {
    /// The TCP model the bound simulated: the practice run's and the run's `params.ws_tcp_model` (FX-13).
    pub model: String,
    /// Client-bursts per simulated run (480 for 48 clients x 10 bursts): a practice run holds exactly this many heals.
    pub client_bursts_count: u64,
    pub x_ns: i64,
    pub y_ns: i64,
    pub k_count: i64,
    pub z_ns: i64,
    /// The median, over the bound's simulated runs of a correct build, of each run's median.
    pub m_m_ns: i64,
    pub band_low: Ratio,
    pub band_high: Ratio,
    pub tolerance: Ratio,
    /// The band and the tolerance as the line writes them (`"0.965"`, `"1.10"`), for the verdict's detail.
    pub band_text: (String, String),
    pub tolerance_text: String,
}

/// Reads the bound's machine-readable line: [`BOUND_TAG`], then one JSON object with exactly README §2.5's fields.
pub(crate) fn parse_bound(line: &str) -> Result<Bound, String> {
    let json = line
        .strip_prefix(BOUND_TAG)
        .ok_or_else(|| format!("the bound line does not start with {BOUND_TAG:?}"))?;
    let raw: RawBound =
        serde_json::from_str(json).map_err(|e| format!("the bound line's object: {e}"))?;
    // At most 6 digits after the point (the advisor's FX-8), so no comparison of these fractions can overflow.
    let decimal = |name: &str, s: &str| {
        if s.split_once('.').is_some_and(|(_, f)| f.len() > 6) {
            return Err(format!(
                "the bound line's {name}: {s:?} has more than 6 digits after the point"
            ));
        }
        Ratio::parse(s).map_err(|e| format!("the bound line's {name}: {e}"))
    };
    let b = Bound {
        model: raw.model.clone(),
        client_bursts_count: raw.client_bursts_count,
        x_ns: raw.x_ns,
        y_ns: raw.y_ns,
        k_count: raw.k_count,
        z_ns: raw.z_ns,
        m_m_ns: raw.m_m_ns,
        band_low: decimal("band_low", &raw.band_low)?,
        band_high: decimal("band_high", &raw.band_high)?,
        tolerance: decimal("tolerance", &raw.tolerance)?,
        band_text: (raw.band_low.clone(), raw.band_high.clone()),
        tolerance_text: raw.tolerance.clone(),
    };
    let guard = decimal("guard_share", &raw.guard_share)?;
    let hex = |s: &str| {
        s.len() == 40
            && s.bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    };
    let problems = [
        (raw.model.is_empty(), "no model"),
        (
            !hex(&raw.commit),
            "a commit that is not 40 lowercase hex digits",
        ),
        (raw.runs_count < 2_000, "fewer than 2,000 simulated runs"),
        (raw.client_bursts_count == 0, "no client-bursts per run"),
        (
            b.x_ns <= 0 || b.y_ns <= 0 || b.z_ns <= 0 || b.m_m_ns <= 0,
            "a time that is not above 0",
        ),
        (b.k_count < 0, "a k below 0"),
        (
            // M_m is the median of the run medians, so a correct build's band holds 1 (the advisor's FX-1).
            b.band_low <= Ratio::int(0)
                || b.band_low > Ratio::int(1)
                || b.band_high < Ratio::int(1),
            "a band that is not 0 < low <= 1 <= high",
        ),
        (
            b.tolerance < Ratio::int(1) || b.tolerance > Ratio::new(11, 10),
            "a tolerance outside 1 to 1.10 (Mustafa's 10 %, never raised)",
        ),
        (
            guard < Ratio::int(0) || guard > Ratio::new(1, 100),
            "a guard share outside 0 to 0.01 (budget x2 within the tolerance in more than 1 % of runs)",
        ),
    ];
    match problems.iter().find(|(bad, _)| *bad) {
        Some((_, what)) => Err(format!("the bound line has {what}")),
        None => Ok(b),
    }
}

/// The bound's X, k, Y and Z against a mark's `ws_bound` checks (README §2.3): each check holds the bound's number.
/// Checked when marks.json loads (marks.rs, the advisor's FX-6 (b)) and again by `find`.
pub(crate) fn checks_agree(bound: &Bound, checks: &[Check]) -> Result<(), String> {
    for c in checks {
        let (Some(n), Threshold::Value(value)) = (c.ws_bound, &c.threshold) else {
            continue;
        };
        let (got, want, letter) = match n {
            BoundNumber::X => (*value, bound.x_ns, "X"),
            BoundNumber::Z => (*value, bound.z_ns, "Z"),
            BoundNumber::Y => {
                // marks.rs gives every ws_bound y check a where: its value is Y, the check's value k.
                let Measure::Series { cond: Some(w), .. } = &c.measure else {
                    continue;
                };
                if *value != Ratio::int(i128::from(bound.k_count)) {
                    return Err(format!(
                        "its k is {}, check {:?} allows {value} over Y",
                        bound.k_count, c.name
                    ));
                }
                (w.value, bound.y_ns, "Y")
            }
        };
        if got != Ratio::int(i128::from(want)) {
            return Err(format!(
                "its {letter} is {want}, check {:?} holds {got}",
                c.name
            ));
        }
    }
    Ok(())
}

/// `v` rounded up to the next whole millisecond (the advisor's R6: each moved number "rounded up to the next
/// millisecond").
pub(crate) fn ceil_ms(v: Ratio) -> Ratio {
    let unit: i128 = 1_000_000;
    let den = v.den() * unit;
    let q = if v.num() >= 0 {
        (v.num() + den - 1) / den
    } else {
        v.num() / den
    };
    Ratio::int(q * unit)
}

/// What the ws-practice rule found for a run: its practice run, the bound, the factor applied, and the record's class
/// and rulings (printed whole beside the numbers in force: the advisor's FX-11).
pub(crate) struct Practice {
    pub run: String,
    pub bound: Bound,
    pub m_p_ns: i64,
    pub s: Ratio,
    pub f: Ratio,
    pub class: String,
    pub chance_ruling: Option<String>,
    pub second_ruling: Option<String>,
}

impl Practice {
    /// The bound's number, unmoved.
    pub fn bound_of(&self, n: BoundNumber) -> i64 {
        match n {
            BoundNumber::X => self.bound.x_ns,
            BoundNumber::Y => self.bound.y_ns,
            BoundNumber::Z => self.bound.z_ns,
        }
    }

    /// The number in force: the bound's times f, rounded up to the next millisecond (k never moves).
    pub fn moved(&self, n: BoundNumber) -> Ratio {
        ceil_ms(Ratio::int(i128::from(self.bound_of(n))) * self.f)
    }

    /// How the number in force was reached, for the verdict's detail: the practice outcome whole, so a run judged
    /// under a ruling never reads as IN TOLERANCE (the advisor's FX-11).
    pub fn describe(&self, n: BoundNumber) -> String {
        let why_f = if self.class == "FAR BETTER" {
            "FAR BETTER: at the bound"
        } else {
            "max(1, s)"
        };
        let mut text = format!(
            "{n:?} {} x f {} = {}, rounded up to the ms; practice run {}: {}, s = M_p {} / M_m {} = {}, band {} to {}, \
             tolerance {}, f = {} ({why_f})",
            self.bound_of(n),
            self.f,
            self.moved(n),
            self.run,
            self.class,
            self.m_p_ns,
            self.bound.m_m_ns,
            self.s,
            self.bound.band_text.0,
            self.bound.band_text.1,
            self.bound.tolerance_text,
            self.f,
        );
        if let Some(r) = &self.chance_ruling {
            text.push_str(&format!("; chance ruling: {r:?}"));
        }
        if let Some(r) = &self.second_ruling {
            text.push_str(&format!("; second ruling: {r:?}"));
        }
        text
    }
}

/// One header param of a run, the same in every log that has it.
fn param<'r>(run: &'r Run<'_>, name: &str) -> Result<Option<&'r Value>, String> {
    let mut found: Option<&Value> = None;
    for log in &run.logs {
        if let Some(v) = log.header.params.get(name) {
            if found.is_some_and(|f| f != v) {
                return Err(format!(
                    "run {}: its logs disagree on params.{name}",
                    run.id
                ));
            }
            found = Some(v);
        }
    }
    Ok(found)
}

/// A run's `params.started_utc`, in README §2.5's form (`2026-11-20T18:05:00Z`), which sorts as time does.
fn started(run: &Run<'_>) -> Result<String, String> {
    let s = param(run, "started_utc")?
        .and_then(Value::as_str)
        .ok_or_else(|| format!("run {} has no params.started_utc", run.id))?;
    let b = s.as_bytes();
    let form = b.len() == 20
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            10 => *c == b'T',
            13 | 16 => *c == b':',
            19 => *c == b'Z',
            _ => c.is_ascii_digit(),
        });
    if form {
        Ok(s.to_owned())
    } else {
        Err(format!(
            "run {}: params.started_utc {s:?} is not YYYY-MM-DDTHH:MM:SSZ",
            run.id
        ))
    }
}

fn spacing(run: &Run<'_>) -> Result<i64, String> {
    param(run, "burst_spacing_ns")?
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("run {} has no integer params.burst_spacing_ns", run.id))
}

/// RS's `machine.kernel` in a run (its logs with `machine.role` RS, all agreeing), `None` with no RS log or an empty
/// kernel.
fn rs_kernel<'r>(run: &'r Run<'_>) -> Result<Option<&'r str>, String> {
    let kernels: BTreeSet<&str> = run
        .logs
        .iter()
        .filter(|l| l.header.machine.role == "RS")
        .map(|l| l.header.machine.kernel.as_str())
        .collect();
    match kernels.iter().copied().collect::<Vec<_>>().as_slice() {
        [] | [""] => Ok(None),
        [k] => Ok(Some(k)),
        _ => Err(format!(
            "run {}: its RS logs disagree on machine.kernel",
            run.id
        )),
    }
}

/// The binaries a run's logs came from.
fn builds<'r>(run: &'r Run<'_>) -> BTreeSet<&'r str> {
    run.logs
        .iter()
        .map(|l| l.header.build.binary_sha256.as_str())
        .collect()
}

/// A run's measured heal times (`event.burst_heal.after_link_ns`), sorted.
fn heals(run: &Run<'_>) -> Result<Vec<i64>, String> {
    let mut v = Vec::new();
    for e in run
        .logs
        .iter()
        .flat_map(|l| &l.events)
        .filter(|e| e.ev == "burst_heal" && e.ph == Phase::Measured)
    {
        match e.f.get("after_link_ns").and_then(Value::as_i64) {
            Some(x) => v.push(x),
            None => {
                return Err(format!(
                    "run {}: a burst_heal event has no integer after_link_ns",
                    run.id
                ));
            }
        }
    }
    v.sort_unstable();
    Ok(v)
}

fn text<'e>(e: &'e EventRecord, name: &str) -> Option<&'e str> {
    e.f.get(name)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
}

fn int(e: &EventRecord, name: &str) -> Result<i64, String> {
    e.f.get(name)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("the ws_practice record has no integer {name}"))
}

/// The practice run in force for a counted run, with its one record: of the practice runs built from the counted
/// run's binaries and started before it, over every run the judge reads (in any set, listing the mark or not), the
/// latest; with more than one, only if its record has a `second_ruling`; and it must list the mark (README §4,
/// `ws-practice`; the advisor's R7, R9, FX-2 and FX-3).
fn practice_of<'r, 'a>(
    counted: &Run<'_>,
    all: &'r [Run<'a>],
    mark: &Mark,
) -> Result<(&'r Run<'a>, &'r EventRecord), String> {
    let when = started(counted)?;
    let mine = builds(counted);
    let mut candidates: Vec<(&'r Run<'a>, String)> = Vec::new();
    for p in all
        .iter()
        .filter(|p| p.role == "practice" && builds(p) == mine)
    {
        // Only a practice run started before the run is a candidate: one started after never changes its verdict
        // (the advisor's R9 and FX-2).
        let p_when = started(p)?;
        if p_when < when {
            candidates.push((p, p_when));
        }
    }
    candidates.sort_by(|a, b| a.1.cmp(&b.1));
    let Some(&(p, _)) = candidates.last() else {
        return Err(format!(
            "run {}: no practice run of its build started before it (role practice, listing {})",
            counted.id, mark.id
        ));
    };
    if !p.serves(&mark.id) {
        return Err(format!(
            "practice run {}, the latest of run {}'s build, does not list {}",
            p.id, counted.id, mark.id
        ));
    }
    let records: Vec<&'r EventRecord> = p
        .logs
        .iter()
        .flat_map(|l| &l.events)
        .filter(|e| e.ev == "ws_practice")
        .collect();
    let [record] = records.as_slice() else {
        return Err(format!(
            "practice run {} holds {} ws_practice records, not 1",
            p.id,
            records.len()
        ));
    };
    if candidates.len() > 1 && text(record, "second_ruling").is_none() {
        return Err(format!(
            "{} practice runs of one build: a second counts only with a ruling naming a found and fixed cause \
             (second_ruling)",
            candidates.len()
        ));
    }
    Ok((p, *record))
}

/// The measured runs whose practice run a bite run takes (a bites build has none of its own): those of its set that
/// serve the mark with its layer, profile and `params.transport`, all of one build (the advisor's FX-3 (c)).
fn measured_of<'r, 'a>(
    bite: &Run<'_>,
    all: &'r [Run<'a>],
    mark: &Mark,
) -> Result<Vec<&'r Run<'a>>, String> {
    let (layer, profile) = (&bite.logs[0].header.layer, &bite.logs[0].header.profile);
    let transport = param(bite, "transport")?;
    let mut measured = Vec::new();
    for m in all
        .iter()
        .filter(|m| m.set == bite.set && m.role == "measured" && m.serves(&mark.id))
    {
        let h = &m.logs[0].header;
        if &h.layer == layer && &h.profile == profile && param(m, "transport")? == transport {
            measured.push(m);
        }
    }
    let kinds = measured
        .iter()
        .map(|m| builds(m))
        .collect::<BTreeSet<_>>()
        .len();
    match kinds {
        0 => Err(format!(
            "set {} holds no measured run serving {} with bite run {}'s layer, profile and transport",
            bite.set, mark.id, bite.id
        )),
        1 => Ok(measured),
        n => Err(format!(
            "set {}'s measured runs of bite run {}'s layer, profile and transport come from {n} builds, so it \
             cannot tell whose numbers are in force",
            bite.set, bite.id
        )),
    }
}

/// The practice run whose numbers are in force for `run`, checked whole (README §4, `ws-practice`), over every run
/// the judge reads. A bite run takes the practice run of its set's measured runs (`measured_of`), so it is judged at
/// its counted run's numbers.
pub(crate) fn find(
    run: &Run<'_>,
    all: &[Run<'_>],
    mark: &Mark,
    tolerance: Ratio,
) -> Result<Practice, String> {
    let gap = spacing(run)?;
    let counted = if run.role == "bite" {
        measured_of(run, all, mark)?
    } else {
        vec![run]
    };
    let mut chosen: Option<(&Run<'_>, &EventRecord)> = None;
    for c in &counted {
        let (p, record) = practice_of(c, all, mark)?;
        if let Some((q, _)) = chosen
            && q.id != p.id
        {
            return Err(format!(
                "set {}'s measured runs have practice runs {} and {} in force, so bite run {} cannot tell whose \
                 numbers are in force",
                run.set, q.id, p.id, run.id
            ));
        }
        chosen = Some((p, record));
    }
    let Some((p, record)) = chosen else {
        return Err(format!(
            "run {}: no counted run to take numbers from",
            run.id
        ));
    };
    let broken = always_rules(p);
    if !broken.is_empty() {
        return Err(format!(
            "practice run {} is invalid: {}",
            p.id,
            broken.join("; ")
        ));
    }
    let (a, b) = (&run.logs[0].header, &p.logs[0].header);
    if a.scene != b.scene || a.profile != b.profile {
        return Err(format!(
            "practice run {} ran another scene or profile",
            p.id
        ));
    }
    if spacing(p)? != gap {
        return Err(format!("practice run {} ran another burst spacing", p.id));
    }
    // The record carries the mark's bound line byte for byte (the advisor's FX-6): M_m, the band, the guard, the
    // model and the commit come from reviewed marks.json, never from a log.
    let mark_line = mark
        .ws_bound_line
        .as_deref()
        .ok_or_else(|| format!("{} holds no ws_bound_line", mark.id))?;
    let line = text(record, "bound").ok_or("the ws_practice record has no bound line")?;
    if line != mark_line {
        return Err(format!(
            "practice run {}'s record: its bound line is not {}'s ws_bound_line, byte for byte",
            p.id, mark.id
        ));
    }
    let bound = parse_bound(mark_line)?;
    if bound.tolerance != tolerance {
        return Err(format!(
            "the bound's tolerance {} is not marks.json's {tolerance}",
            bound.tolerance
        ));
    }
    checks_agree(&bound, &mark.checks).map_err(|e| format!("the bound line: {e}"))?;
    // The practice run ran on the run's setup (the advisor's R7 and FX-13): the same TCP model, the bound's; the
    // same CONFIG_HZ; the same RS kernel; each present in both runs.
    let present = |v: Option<&Value>| v.filter(|v| !v.is_null()).cloned();
    let setups = [
        (
            "params.ws_tcp_model",
            present(param(run, "ws_tcp_model")?),
            present(param(p, "ws_tcp_model")?),
        ),
        (
            "params.config_hz",
            present(param(run, "config_hz")?),
            present(param(p, "config_hz")?),
        ),
        (
            "RS's machine.kernel",
            rs_kernel(run)?.map(Value::from),
            rs_kernel(p)?.map(Value::from),
        ),
    ];
    for (what, mine, theirs) in setups {
        match (mine, theirs) {
            (None, _) => return Err(format!("run {} has no {what}", run.id)),
            (_, None) => return Err(format!("practice run {} has no {what}", p.id)),
            (Some(a), Some(b)) if a != b => {
                return Err(format!(
                    "practice run {} ran another {what} than run {}",
                    p.id, run.id
                ));
            }
            _ => {}
        }
    }
    if param(run, "ws_tcp_model")?.and_then(Value::as_str) != Some(bound.model.as_str()) {
        return Err(format!(
            "run {}'s params.ws_tcp_model is not the bound's model {:?}",
            run.id, bound.model
        ));
    }
    let heal = heals(p)?;
    // Exactly the bound's client-bursts (the advisor's R7, "Valid only with 480 heal records"; FX-7).
    if heal.len() as u64 != bound.client_bursts_count {
        return Err(format!(
            "practice run {} has {} heal records, not the bound's {}",
            p.id,
            heal.len(),
            bound.client_bursts_count
        ));
    }
    let m_p_ns = heal[nearest_rank(heal.len(), Ratio::new(1, 2)) - 1];
    if int(record, "m_p_ns")? != m_p_ns {
        return Err(format!(
            "the record's m_p_ns is not practice run {}'s median heal time {m_p_ns}",
            p.id
        ));
    }
    let s = Ratio::new(i128::from(m_p_ns), i128::from(bound.m_m_ns));
    let class = text(record, "class").unwrap_or("");
    let chance = text(record, "chance_ruling").is_some();
    let class_ok = match class {
        "IN TOLERANCE" => s >= bound.band_low && s <= tolerance,
        "BEYOND" => chance && s <= tolerance,
        "FAR BETTER" => chance && s < bound.band_low,
        _ => false,
    };
    if !class_ok {
        return Err(format!(
            "practice run {}'s record is {class:?} at s = {s} (band {} to {}, tolerance {tolerance}){}: its numbers \
             are not in force",
            p.id,
            bound.band_low,
            bound.band_high,
            if chance {
                ""
            } else {
                ", with no chance ruling"
            }
        ));
    }
    // FAR BETTER runs at the bound (the advisor's R8 (v)); IN TOLERANCE and BEYOND at max(1, s). The numbers never
    // move past the tolerance (Mustafa's WS-BAR cap), whatever the record says.
    let f = if class == "FAR BETTER" || s <= Ratio::int(1) {
        Ratio::int(1)
    } else {
        s
    };
    if f > tolerance {
        return Err(format!(
            "practice run {}: f = {f} is above the tolerance {tolerance}: the numbers never move past it",
            p.id
        ));
    }
    let practice = Practice {
        run: p.id.to_owned(),
        bound,
        m_p_ns,
        s,
        f,
        class: class.to_owned(),
        chance_ruling: text(record, "chance_ruling").map(str::to_owned),
        second_ruling: text(record, "second_ruling").map(str::to_owned),
    };
    for (n, field) in [
        (BoundNumber::X, "x_moved_ns"),
        (BoundNumber::Y, "y_moved_ns"),
        (BoundNumber::Z, "z_moved_ns"),
    ] {
        if Ratio::int(i128::from(int(record, field)?)) != practice.moved(n) {
            return Err(format!(
                "the record's {field} is not the judge's {}",
                practice.describe(n)
            ));
        }
    }
    if int(record, "k_count")? != practice.bound.k_count {
        return Err("the record's k_count is not the bound's (k never moves)".to_owned());
    }
    if class == "IN TOLERANCE" {
        let y = practice.moved(BoundNumber::Y);
        let over = heal
            .iter()
            .filter(|h| Ratio::int(i128::from(**h)) > y)
            .count();
        let worst = Ratio::int(i128::from(heal[heal.len() - 1]));
        if over as i64 > practice.bound.k_count || worst > practice.moved(BoundNumber::Z) {
            return Err(format!(
                "the record says IN TOLERANCE, but practice run {} has {over} heals over {y} ns and its slowest is \
                 {worst} ns",
                p.id
            ));
        }
    }
    Ok(practice)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = r#"npt-ws-bound v1 {"model":"linux-6.8-tcp v3","commit":"bf9d248000000000000000000000000000000000","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;

    #[test]
    fn the_bound_line_reads_as_readme_writes_it() {
        let b = parse_bound(LINE).expect("README §2.5's example");
        assert_eq!(
            (b.x_ns, b.y_ns, b.k_count, b.z_ns),
            (670_000_000, 2_500_000_000, 5, 6_900_000_000)
        );
        assert_eq!(b.m_m_ns, 654_000_000);
        assert_eq!(b.tolerance, Ratio::new(11, 10));
        for (from, to, why) in [
            ("npt-ws-bound v1 ", "npt-ws-bound v2 ", "another tag"),
            (
                r#""runs_count":2000"#,
                r#""runs_count":1999"#,
                "too few runs",
            ),
            (
                r#""tolerance":"1.10""#,
                r#""tolerance":"1.11""#,
                "a raised tolerance",
            ),
            (
                r#""tolerance":"1.10""#,
                r#""tolerance":"0.99""#,
                "a tolerance below 1",
            ),
            (
                r#""guard_share":"0.0000""#,
                r#""guard_share":"0.0101""#,
                "a failed guard",
            ),
            (
                r#""band_low":"0.965""#,
                r#""band_low":"1.04""#,
                "a band upside down",
            ),
            (r#""k_count":5"#, r#""k_count":-1"#, "a negative k"),
            (r#""x_ns":670000000"#, r#""x_ns":0"#, "a zero time"),
            (
                r#""commit":"bf9d248"#,
                r#""commit":"BF9D248"#,
                "an upper-case commit",
            ),
            (
                r#""model":"linux-6.8-tcp v3","#,
                r#""model":"","#,
                "no model",
            ),
            (r#","guard_share":"0.0000""#, "", "a missing field"),
            (
                r#""guard_share":"0.0000""#,
                r#""guard_share":"0.0000","x":1"#,
                "an unknown field",
            ),
            (r#""x_ns":670000000"#, r#""x_ns":670000000.5"#, "a float"),
        ] {
            assert!(LINE.contains(from), "{why}: {from}");
            assert!(parse_bound(&LINE.replacen(from, to, 1)).is_err(), "{why}");
        }
    }

    /// M_m is the median of a correct build's run medians, so the band of their 1st to 99th percentiles holds 1: a
    /// line whose band leaves 1 out is refused (the advisor's FX-1 on U4j).
    #[test]
    fn the_bound_line_refuses_a_band_that_leaves_out_1() {
        let band = |low: &str, high: &str| {
            LINE.replacen(
                r#""band_low":"0.965""#,
                &format!(r#""band_low":"{low}""#),
                1,
            )
            .replacen(
                r#""band_high":"1.035""#,
                &format!(r#""band_high":"{high}""#),
                1,
            )
        };
        for (low, high) in [("2.0", "2.0"), ("1.01", "1.2"), ("0.5", "0.9")] {
            assert!(
                parse_bound(&band(low, high)).is_err(),
                "band {low} to {high} must be refused"
            );
        }
        assert!(
            parse_bound(&band("1", "1")).is_ok(),
            "a band of 1 to 1 holds 1"
        );
    }

    /// The four fractions have at most 6 digits after the point, so no comparison of them overflows: the
    /// test-engineer's probe B (two 29-digit fractions) is an error, never a panic (the advisor's FX-8).
    #[test]
    fn the_bound_line_refuses_more_than_6_decimals() {
        let probe_b = LINE
            .replacen(
                r#""band_low":"0.965""#,
                r#""band_low":"0.12345678901234567890123456789""#,
                1,
            )
            .replacen(
                r#""band_high":"1.035""#,
                r#""band_high":"0.99999999999999999999999999999""#,
                1,
            );
        assert!(parse_bound(&probe_b).is_err(), "probe B");
        for (from, to) in [
            (r#""band_low":"0.965""#, r#""band_low":"0.9650001""#),
            (r#""band_high":"1.035""#, r#""band_high":"1.0350001""#),
            (r#""tolerance":"1.10""#, r#""tolerance":"1.1000000""#),
            (r#""guard_share":"0.0000""#, r#""guard_share":"0.0000000""#),
        ] {
            assert!(parse_bound(&LINE.replacen(from, to, 1)).is_err(), "{to}");
        }
        let six = LINE.replacen(r#""band_low":"0.965""#, r#""band_low":"0.965000""#, 1);
        assert!(parse_bound(&six).is_ok(), "6 digits after the point");
    }

    #[test]
    fn moved_numbers_round_up_to_the_millisecond() {
        assert_eq!(ceil_ms(Ratio::int(704_000_000)), Ratio::int(704_000_000));
        assert_eq!(ceil_ms(Ratio::int(704_000_001)), Ratio::int(705_000_000));
        assert_eq!(
            ceil_ms(Ratio::new(1_407_000_001, 2)),
            Ratio::int(704_000_000)
        );
        // The advisor's R6 example: the provisional bound at f = 1.10 gives 0.737 s, 2.75 s and 7.59 s.
        let f = Ratio::new(11, 10);
        assert_eq!(
            ceil_ms(Ratio::int(670_000_000) * f),
            Ratio::int(737_000_000)
        );
        assert_eq!(
            ceil_ms(Ratio::int(2_500_000_000) * f),
            Ratio::int(2_750_000_000)
        );
        assert_eq!(
            ceil_ms(Ratio::int(6_900_000_000) * f),
            Ratio::int(7_590_000_000)
        );
    }
}
