//! The judging itself: runs from logs, values from series, validity, checks, parts, bites and the verdict
//! (`marks/README.md` §3 to §6).

use crate::figures;
use crate::marks::{
    Against, BoundNumber, Combine, Cond, Kind, Mark, Marks, Measure, Part, Rule, Source, Stat,
    Threshold, Window,
};
use crate::practice;
use crate::ratio::Ratio;
use npt_log::{Header, Log, Phase};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;

/// A run: every log whose header has the same `run`.
pub struct Run<'a> {
    pub id: &'a str,
    pub set: &'a str,
    pub role: &'a str,
    pub bite: Option<&'a str>,
    pub logs: Vec<&'a Log>,
    /// The run-level header fields its logs disagree on.
    pub inconsistent: Vec<&'static str>,
}

impl<'a> Run<'a> {
    pub fn serves(&self, mark: &str) -> bool {
        self.logs
            .iter()
            .any(|l| l.header.marks.iter().any(|m| m == mark))
    }
}

/// Groups logs into runs, in run-id order.
pub fn runs(logs: &[Log]) -> Vec<Run<'_>> {
    let mut by_run: BTreeMap<&str, Vec<&Log>> = BTreeMap::new();
    for log in logs {
        by_run.entry(log.header.run.as_str()).or_default().push(log);
    }
    by_run
        .into_iter()
        .map(|(id, logs)| {
            let h = &logs[0].header;
            let mut inconsistent = Vec::new();
            for l in &logs[1..] {
                let o = &l.header;
                let fields = [
                    ("set", o.set == h.set),
                    ("role", o.role == h.role),
                    ("bite", o.bite == h.bite),
                    ("layer", o.layer == h.layer),
                    ("profile", o.profile == h.profile),
                    ("tick_hz", o.tick_hz == h.tick_hz),
                    ("scene", o.scene == h.scene),
                    ("date", o.date == h.date),
                ];
                for (name, same) in fields {
                    if !same && !inconsistent.contains(&name) {
                        inconsistent.push(name);
                    }
                }
            }
            Run {
                id,
                set: &h.set,
                role: &h.role,
                bite: h.bite.as_deref(),
                logs,
                inconsistent,
            }
        })
        .collect()
}

/// Why a value could not be had: a gap in the record (the run is invalid), code not built yet (NOT-RUN), or a
/// comparison that compared nothing (its check fails: README §2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Missing {
    Gap(String),
    NotBuilt(String),
    Empty(String),
}

fn header_value(h: &Header, key: &str) -> Option<Value> {
    Some(match key {
        "layer" => Value::from(h.layer.as_str()),
        "profile" => Value::from(h.profile.as_str()),
        "role" => Value::from(h.role.as_str()),
        "scene" => Value::from(h.scene.as_str()),
        "process" => Value::from(h.process.as_str()),
        "tick_hz" => Value::from(h.tick_hz),
        "clients" => Value::from(h.clients),
        "machine.name" => Value::from(h.machine.name.as_str()),
        "machine.role" => Value::from(h.machine.role.as_str()),
        "machine.arch" => Value::from(h.machine.arch.as_str()),
        other => {
            return other
                .strip_prefix("params.")
                .and_then(|p| h.params.get(p).cloned());
        }
    })
}

fn log_matches(log: &Log, sel: &BTreeMap<String, Value>) -> bool {
    sel.iter()
        .all(|(k, v)| header_value(&log.header, k).as_ref() == Some(v))
}

/// A run matches if, for every key, one of its logs has that value.
fn run_matches(run: &Run<'_>, sel: &BTreeMap<String, Value>) -> bool {
    sel.iter().all(|(k, v)| {
        run.logs
            .iter()
            .any(|l| header_value(&l.header, k).as_ref() == Some(v))
    })
}

fn in_window(ph: Phase, w: Window) -> bool {
    match w {
        Window::All => true,
        Window::Measured => ph == Phase::Measured,
        Window::Warmup => ph == Phase::Warmup,
    }
}

fn source_text(s: &Source) -> String {
    match s {
        Source::TickV(n) => format!("tick.v.{n}"),
        Source::TickS(n) => format!("tick.s.{n}"),
        Source::SecondV(n) => format!("second.v.{n}"),
        Source::SecondS(n) => format!("second.s.{n}"),
        Source::Event { ev, field } => format!("event.{ev}.{field}"),
        Source::HeaderParam(n) => format!("header.params.{n}"),
    }
}

fn json_int(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64(),
        Value::Bool(b) => Some(i64::from(*b)),
        _ => None,
    }
}

/// Every value of a series in a run's logs (those `from` picks), or a gap.
/// Every value of a series in a run's logs (those `from` picks), from the first `first` records of each log in the
/// window if given. An empty list is not an error here: `measure` decides what an empty series means.
fn collect(
    run: &Run<'_>,
    source: &Source,
    window: Window,
    first: Option<u64>,
    from: &BTreeMap<String, Value>,
) -> Result<Vec<i64>, String> {
    let logs: Vec<&Log> = run
        .logs
        .iter()
        .copied()
        .filter(|l| log_matches(l, from))
        .collect();
    if logs.is_empty() {
        return Err(format!(
            "run {}: no log matches from {}",
            run.id,
            serde_json::to_string(from).unwrap_or_default()
        ));
    }
    let take = first.map_or(usize::MAX, |n| usize::try_from(n).unwrap_or(usize::MAX));
    let mut out = Vec::new();
    for log in logs {
        match source {
            Source::TickV(n) => out.extend(
                log.ticks
                    .iter()
                    .filter(|t| in_window(t.ph, window))
                    .take(take)
                    .filter_map(|t| t.v.get(n).copied()),
            ),
            Source::TickS(n) => {
                for t in log
                    .ticks
                    .iter()
                    .filter(|t| in_window(t.ph, window))
                    .take(take)
                {
                    if let Some(s) = t.s.get(n) {
                        out.extend_from_slice(s);
                    }
                }
            }
            Source::SecondV(n) => out.extend(
                log.seconds
                    .iter()
                    .filter(|s| in_window(s.ph, window))
                    .take(take)
                    .filter_map(|s| s.v.get(n).copied()),
            ),
            Source::SecondS(n) => {
                for s in log
                    .seconds
                    .iter()
                    .filter(|s| in_window(s.ph, window))
                    .take(take)
                {
                    if let Some(v) = s.s.get(n) {
                        out.extend_from_slice(v);
                    }
                }
            }
            Source::Event { ev, field } => {
                for e in log
                    .events
                    .iter()
                    .filter(|e| &e.ev == ev && in_window(e.ph, window))
                    .take(take)
                {
                    match e.f.get(field).and_then(json_int) {
                        Some(v) => out.push(v),
                        None => {
                            return Err(format!(
                                "run {}: an {ev} event has no integer {field}",
                                run.id
                            ));
                        }
                    }
                }
            }
            Source::HeaderParam(n) => {
                if let Some(v) = log.header.params.get(n) {
                    match json_int(v) {
                        Some(i) => out.push(i),
                        None => {
                            return Err(format!("run {}: params.{n} is not an integer", run.id));
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}

/// The same, for the validity rules: an empty series is a gap there.
fn collect_some(
    run: &Run<'_>,
    source: &Source,
    window: Window,
    from: &BTreeMap<String, Value>,
) -> Result<Vec<i64>, String> {
    let v = collect(run, source, window, None, from)?;
    if v.is_empty() {
        Err(format!(
            "run {}: no values for {}",
            run.id,
            source_text(source)
        ))
    } else {
        Ok(v)
    }
}

/// The 1-based nearest rank of quantile `q` among `n` values: ⌈q × n⌉ (plan §6.1).
pub fn nearest_rank(n: usize, q: Ratio) -> usize {
    let n = n as i128;
    let rank = (q.num() * n + q.den() - 1) / q.den();
    rank.clamp(1, n.max(1)) as usize
}

fn meets(v: i64, c: Cond) -> bool {
    c.op.holds(Ratio::int(i128::from(v)), c.value)
}

/// A statistic of a non-empty list of values.
pub(crate) fn stat_of(stat: &Stat, cond: Option<Cond>, values: &[i64]) -> Ratio {
    let n = values.len() as i128;
    let sum = || values.iter().map(|v| i128::from(*v)).sum::<i128>();
    let count_meeting = |c: Cond| values.iter().filter(|v| meets(**v, c)).count() as i128;
    match stat {
        Stat::Count => Ratio::int(cond.map_or(n, count_meeting)),
        Stat::Sum => Ratio::int(sum()),
        Stat::Min => Ratio::int(values.iter().min().map_or(0, |v| i128::from(*v))),
        Stat::Max => Ratio::int(values.iter().max().map_or(0, |v| i128::from(*v))),
        Stat::Mean => Ratio::new(sum(), n.max(1)),
        Stat::Pct(q) => {
            let mut sorted = values.to_vec();
            sorted.sort_unstable();
            Ratio::int(i128::from(sorted[nearest_rank(sorted.len(), *q) - 1]))
        }
        Stat::Share => Ratio::new(cond.map_or(0, count_meeting), n.max(1)),
    }
}

/// A measure over one or more runs (more than one for a pooled check or a pooled reference).
fn measure(
    m: &Measure,
    runs: &[&Run<'_>],
    set: &[&Run<'_>],
    from: &BTreeMap<String, Value>,
) -> Result<Ratio, Missing> {
    match m {
        Measure::Series {
            source,
            stat,
            window,
            cond,
            first,
        } => {
            let mut values = Vec::new();
            for run in runs {
                values.extend(collect(run, source, *window, *first, from).map_err(Missing::Gap)?);
            }
            if values.is_empty() {
                // An event that never happened is a real 0 (the advisor's C2 ruling). Every tick and second record
                // carries its counters and flags, 0 included (SERIES.md), so an empty tick or second series, or any
                // other statistic of an empty series, is a gap in the record.
                return if matches!(stat, Stat::Count) && matches!(source, Source::Event { .. }) {
                    Ok(Ratio::int(0))
                } else {
                    let ids: Vec<&str> = runs.iter().map(|r| r.id).collect();
                    Err(Missing::Gap(format!(
                        "run {}: no values for {}",
                        ids.join("+"),
                        source_text(source)
                    )))
                };
            }
            Ok(stat_of(stat, *cond, &values))
        }
        Measure::Figure(name) => match runs {
            [run] => figures::compute(name, run, set),
            _ => Err(Missing::NotBuilt(format!(
                "figure {name} cannot be pooled over runs"
            ))),
        },
    }
}

/// How one check came out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Pass,
    Fail,
    /// A gap in the record: the run is invalid.
    Gap,
    /// Not decidable yet (code not built, a reference run missing): NOT-RUN.
    Cannot,
    /// A comparison that compared nothing (a decoded or twin hash figure with 0 pairs): the run fails, as for `Fail`,
    /// but no bite is shown by it (a bite run that compared nothing did not test its bite: README §5).
    Empty,
}

impl Outcome {
    /// The check failed the run: a plain failure, or an empty comparison.
    pub fn fails(self) -> bool {
        matches!(self, Outcome::Fail | Outcome::Empty)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckOut {
    pub name: String,
    pub outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub needs: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// A setup check: its failure makes the run INVALID, and no bite is shown by it.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub setup: bool,
}

fn reference(
    m: &Measure,
    against: &Against,
    set: &[&Run<'_>],
    from: &BTreeMap<String, Value>,
) -> Result<Ratio, String> {
    let refs: Vec<&Run<'_>> = set
        .iter()
        .copied()
        .filter(|r| against.roles.iter().any(|role| role == r.role) && always_rules(r).is_empty())
        .collect();
    if refs.is_empty() {
        return Err(format!(
            "no valid run with role {} in the set",
            against.roles.join(" or ")
        ));
    }
    let as_text = |e: Missing| match e {
        Missing::Gap(s) | Missing::NotBuilt(s) | Missing::Empty(s) => s,
    };
    match against.combine {
        Combine::Pooled => measure(m, &refs, set, from).map_err(as_text),
        Combine::Mean | Combine::Median => {
            let mut values = Vec::new();
            for r in &refs {
                values.push(measure(m, &[*r], set, from).map_err(as_text)?);
            }
            values.sort();
            let n = values.len();
            Ok(if against.combine == Combine::Mean {
                values
                    .iter()
                    .fold(Ratio::int(0), |a, v| a + *v)
                    .div_int(n as i128)
            } else if n % 2 == 1 {
                values[n / 2]
            } else {
                (values[n / 2 - 1] + values[n / 2]).div_int(2)
            })
        }
    }
}

/// The ws-practice rule's tolerance for a check's parts (the mark's rule, else its first part's).
fn practice_tolerance(mark: &Mark, check: &crate::marks::Check) -> Option<Ratio> {
    let tolerance = |r: &Rule| match r {
        Rule::WsPractice { tolerance, .. } => Some(*tolerance),
        _ => None,
    };
    mark.validity.iter().find_map(tolerance).or_else(|| {
        mark.parts
            .iter()
            .filter(|p| check.parts.contains(&p.name))
            .flat_map(|p| &p.validity)
            .find_map(tolerance)
    })
}

/// A `ws_bound` check with the numbers in force: the bound's number times the practice run's factor, rounded up to
/// the millisecond (README §2.3); k stays. The ws-practice rule has already held, so a lookup failing here is a gap.
fn in_force(
    check: &crate::marks::Check,
    n: BoundNumber,
    run: &Run<'_>,
    ctx: &Ctx<'_>,
) -> Result<(crate::marks::Check, String), String> {
    let tolerance = practice_tolerance(ctx.mark, check)
        .ok_or_else(|| format!("{} holds no ws-practice rule", ctx.mark.id))?;
    let p = practice::find(run, ctx.all, ctx.mark, tolerance)?;
    let mut moved = check.clone();
    match (n, &mut moved.measure) {
        (BoundNumber::Y, Measure::Series { cond: Some(c), .. }) => c.value = p.moved(n),
        _ => moved.threshold = Threshold::Value(p.moved(n)),
    }
    moved.ws_bound = None;
    Ok((moved, p.describe(n)))
}

fn eval_check(
    check: &crate::marks::Check,
    runs: &[&Run<'_>],
    set: &[&Run<'_>],
    ctx: &Ctx<'_>,
) -> CheckOut {
    if let (Some(n), [run]) = (check.ws_bound, runs) {
        return match in_force(check, n, run, ctx) {
            Ok((moved, how)) => {
                let mut o = eval_check(&moved, runs, set, ctx);
                o.name = check.name.clone();
                o.needs = format!("{} ({how})", o.needs);
                o
            }
            Err(why) => CheckOut {
                name: check.name.clone(),
                outcome: Outcome::Gap,
                value: None,
                needs: String::new(),
                note: Some(format!("ws-practice: {why}")),
                setup: check.setup,
            },
        };
    }
    let (op, needs_text) = (check.op, |t: &str| format!("{} {t}", check.op.text()));
    let out = |outcome, value: Option<Ratio>, needs: String, note: Option<String>| CheckOut {
        name: check.name.clone(),
        outcome,
        value: value.map(|v| v.to_string()),
        needs,
        note,
        setup: check.setup,
    };
    let value = match measure(&check.measure, runs, set, &check.from) {
        Ok(v) => v,
        Err(Missing::Gap(why)) => return out(Outcome::Gap, None, String::new(), Some(why)),
        Err(Missing::NotBuilt(why)) => return out(Outcome::Cannot, None, String::new(), Some(why)),
        Err(Missing::Empty(why)) => {
            let needs = match &check.threshold {
                Threshold::Value(t) => needs_text(&format!("{t} over at least 1 compared")),
                Threshold::Against { .. } => String::new(),
            };
            return out(Outcome::Empty, None, needs, Some(why));
        }
    };
    match &check.threshold {
        Threshold::Value(t) => {
            let pass = op.holds(value, *t);
            out(
                if pass { Outcome::Pass } else { Outcome::Fail },
                Some(value),
                needs_text(&t.to_string()),
                None,
            )
        }
        Threshold::Against { factor, against } => {
            match reference(&check.measure, against, set, &check.from) {
                Ok(r) => {
                    let t = *factor * r;
                    let pass = op.holds(value, t);
                    out(
                        if pass { Outcome::Pass } else { Outcome::Fail },
                        Some(value),
                        needs_text(&format!(
                            "{factor} x {r} ({}, {:?})",
                            against.roles.join("+"),
                            against.combine
                        )),
                        None,
                    )
                }
                Err(why) => out(Outcome::Cannot, Some(value), String::new(), Some(why)),
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Validity (README §4).

fn field_bool(f: &BTreeMap<String, Value>, name: &str) -> Option<bool> {
    f.get(name).and_then(Value::as_bool)
}

/// The rules that hold for every run: consistent headers, complete logs, the roster, capabilities, and the build
/// (the advisor's C4): a measured build never names a bite, and a bite never runs without the bites build.
pub(crate) fn always_rules(run: &Run<'_>) -> Vec<String> {
    let mut why = Vec::new();
    if !run.inconsistent.is_empty() {
        why.push(format!(
            "the run's logs disagree on {}",
            run.inconsistent.join(", ")
        ));
    }
    let bites_build = run
        .logs
        .iter()
        .any(|l| l.header.build.features.iter().any(|f| f == "bites"));
    match run.bite {
        Some(id) => {
            if run.role != "bite" && run.role != "twin" {
                why.push(format!(
                    "build: a {} run names the bite {id}; only a bite run or a bite run's twin may",
                    run.role
                ));
            }
            if !bites_build {
                why.push(format!(
                    "build: the run names the bite {id}, but none of its logs comes from a bites build"
                ));
            }
        }
        None => {
            if bites_build {
                why.push(format!(
                    "build: a {} run names no bite, but a log of it comes from a bites build (measured builds never \
                     contain the bites)",
                    run.role
                ));
            }
        }
    }
    for log in &run.logs {
        let h = &log.header;
        let killed = h.params.get("killed") == Some(&Value::Bool(true));
        if log.footer.is_none() && !killed {
            why.push(format!(
                "complete: the {} log on {} has no footer (cut short)",
                h.process, h.machine.name
            ));
        }
        for e in &log.events {
            match e.ev.as_str() {
                "admit" => match field_bool(&e.f, "in_roster") {
                    Some(true) => {}
                    Some(false) => why.push(format!(
                        "roster: session {} was admitted outside the roster",
                        e.f.get("session").map(Value::to_string).unwrap_or_default()
                    )),
                    None => why.push("roster: an admit event without in_roster".to_owned()),
                },
                "control" => {
                    let applied = field_bool(&e.f, "applied").unwrap_or(false);
                    match e.f.get("source").and_then(Value::as_str) {
                        Some("session") => {
                            let cap = field_bool(&e.f, "cap_valid").unwrap_or(false);
                            let rostered = field_bool(&e.f, "rostered").unwrap_or(false);
                            if applied && !cap {
                                why.push("capability: a session control was applied without a valid capability".to_owned());
                            } else if rostered && !cap {
                                why.push("capability: a rostered session sent a control without a valid capability (a bug in our code)".to_owned());
                            }
                        }
                        Some("socket") => {
                            if applied && !field_bool(&e.f, "peer_ok").unwrap_or(false) {
                                why.push("capability: a socket control was applied from an unchecked peer".to_owned());
                            }
                        }
                        _ => {
                            why.push("capability: a control event with no known source".to_owned())
                        }
                    }
                }
                _ => {}
            }
        }
    }
    why
}

fn sel(pairs: &[(&str, &str)]) -> BTreeMap<String, Value> {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_owned(), Value::from(*v)))
        .collect()
}

fn lg_cpu(run: &Run<'_>, why: &mut Vec<String>) {
    let lg: Vec<&Log> = run
        .logs
        .iter()
        .copied()
        .filter(|l| l.header.machine.role == "LG")
        .collect();
    if lg.is_empty() {
        why.push("lg-80: no LG log in the run".to_owned());
        return;
    }
    for log in lg {
        let seconds: Vec<_> = log
            .seconds
            .iter()
            .filter(|s| s.ph == Phase::Measured)
            .collect();
        if seconds.is_empty() {
            why.push(format!(
                "lg-80: {} has no measured seconds",
                log.header.machine.name
            ));
        }
        for s in seconds {
            match s.v.get("cpu_permille") {
                Some(c) if *c < 800 => {}
                Some(c) => {
                    why.push(format!(
                        "lg-80: {} at {c} permille CPU in second {}",
                        log.header.machine.name, s.sec
                    ));
                    break;
                }
                None => {
                    why.push(format!(
                        "lg-80: {} second {} has no cpu_permille",
                        log.header.machine.name, s.sec
                    ));
                    break;
                }
            }
        }
    }
}

fn p99_work(run: &Run<'_>) -> Result<Ratio, String> {
    let v = collect_some(
        run,
        &Source::TickV("work_ns".to_owned()),
        Window::Measured,
        &BTreeMap::new(),
    )?;
    Ok(stat_of(&Stat::Pct(Ratio::new(99, 100)), None, &v))
}

fn rule(
    rule: &Rule,
    run: &Run<'_>,
    set: &[&Run<'_>],
    ctx: &Ctx<'_>,
    why: &mut Vec<String>,
    undecided: &mut Vec<String>,
) {
    match rule {
        Rule::VS => {
            let mut starts = 0;
            let mut ends = 0;
            for e in run
                .logs
                .iter()
                .flat_map(|l| &l.events)
                .filter(|e| e.ev == "affinity")
            {
                match e.f.get("when").and_then(Value::as_str) {
                    Some("start") => starts += 1,
                    Some("end") => ends += 1,
                    _ => {}
                }
                let alone = field_bool(&e.f, "alone") == Some(true);
                let sibling = field_bool(&e.f, "sibling_free") == Some(true);
                if !alone || !sibling {
                    why.push("V-S: the tick thread was not alone on its vCPU, or its sibling was not free (PM-0001-1)".to_owned());
                }
            }
            if starts == 0 || ends == 0 {
                why.push("V-S: no affinity record at the run's start and end".to_owned());
            }
            match collect_some(
                run,
                &Source::SecondV("steal_permille".to_owned()),
                Window::Measured,
                &sel(&[("machine.role", "RS")]),
            ) {
                Ok(v) => {
                    let mean = stat_of(&Stat::Mean, None, &v);
                    if mean > Ratio::int(10) {
                        why.push(format!(
                            "V-S: RS's steal averaged {mean} permille, over 10 (1 %)"
                        ));
                    }
                }
                Err(e) => why.push(format!("V-S: no steal record ({e})")),
            }
            let hz = i128::from(run.logs[0].header.tick_hz);
            let lg: Vec<&Log> = run
                .logs
                .iter()
                .copied()
                .filter(|l| l.header.machine.role == "LG")
                .collect();
            if lg.is_empty() {
                why.push("V-S: no LG log in the run".to_owned());
            }
            for log in lg {
                let mut totals: Vec<i128> = Vec::new();
                let mut seconds: i128 = 0;
                for s in log.seconds.iter().filter(|s| s.ph == Phase::Measured) {
                    let Some(counts) = s.s.get("inputs_count") else {
                        why.push(format!(
                            "V-S: {} second {} has no inputs_count",
                            log.header.machine.name, s.sec
                        ));
                        break;
                    };
                    if totals.is_empty() {
                        totals = vec![0; counts.len()];
                    }
                    if counts.len() != totals.len() {
                        why.push(format!(
                            "V-S: {} changes its client count in second {}",
                            log.header.machine.name, s.sec
                        ));
                        break;
                    }
                    for (t, c) in totals.iter_mut().zip(counts) {
                        *t += i128::from(*c);
                    }
                    seconds += 1;
                }
                let want = seconds * hz;
                for (i, t) in totals.iter().enumerate() {
                    if (t - want).abs() * 100 > want {
                        why.push(format!(
                            "V-S: client {i} on {} sent {t} inputs in {seconds} s, not {want} within 1 %",
                            log.header.machine.name
                        ));
                    }
                }
            }
            lg_cpu(run, why);
        }
        Rule::Density => {
            let mut p99s = Vec::new();
            for role in ["bracket-first", "bracket-last"] {
                match set.iter().find(|r| r.role == role) {
                    Some(r) => match p99_work(r) {
                        Ok(p) => p99s.push(p),
                        Err(e) => why.push(format!(
                            "PM-0006-13: the {role} run has no tick record ({e})"
                        )),
                    },
                    None => why.push(format!("PM-0006-13: no {role} run in the set")),
                }
            }
            if let [a, b] = p99s.as_slice() {
                let (lo, hi) = if a <= b { (*a, *b) } else { (*b, *a) };
                if hi > lo * Ratio::new(105, 100) {
                    why.push(format!("PM-0006-13: the bracket runs' p99s ({lo} and {hi} ns) differ by more than 5 %"));
                }
            }
            lg_cpu(run, why);
        }
        Rule::Lg80 => lg_cpu(run, why),
        Rule::Padding => {
            let mut any = false;
            for log in &run.logs {
                let measured: Vec<_> = log
                    .ticks
                    .iter()
                    .filter(|t| t.ph == Phase::Measured)
                    .collect();
                if !measured.iter().any(|t| t.v.contains_key("padded_step_ns")) {
                    continue;
                }
                any = true;
                let Some(target) = log
                    .header
                    .params
                    .get("padding_target_ns")
                    .and_then(json_int)
                else {
                    why.push("padding: no params.padding_target_ns in the header".to_owned());
                    continue;
                };
                let mut short = 0_u64;
                let mut first = None;
                for t in &measured {
                    match t.v.get("padded_step_ns") {
                        Some(p) if *p >= target => {}
                        Some(p) => {
                            short += 1;
                            first.get_or_insert((t.t, *p));
                        }
                        None => {
                            why.push(format!("padding: tick {} has no padded step", t.t));
                            break;
                        }
                    }
                }
                if let Some((tick, p)) = first {
                    why.push(format!(
                        "padding: {short} measured ticks fell short of {target} ns (first: tick {tick} at {p} ns)"
                    ));
                }
            }
            if !any {
                why.push("padding: no padded step recorded".to_owned());
            }
        }
        Rule::LateZero => match collect_some(
            run,
            &Source::TickV("late_count".to_owned()),
            Window::Measured,
            &BTreeMap::new(),
        ) {
            Ok(v) => {
                let late: i128 = v.iter().map(|x| i128::from(*x)).sum();
                if late != 0 {
                    why.push(format!("late-zero: {late} commands ran late"));
                }
            }
            Err(e) => why.push(format!("late-zero: no late counter ({e})")),
        },
        Rule::Reorder => {
            let from_header = run.logs.iter().any(|l| {
                l.header
                    .params
                    .get("reorder_permille")
                    .and_then(json_int)
                    .is_some_and(|v| v > 0)
            });
            let from_events = run.logs.iter().flat_map(|l| &l.events).any(|e| {
                e.ev == "reorder"
                    && e.f
                        .get("permille")
                        .and_then(json_int)
                        .is_some_and(|v| v > 0)
            });
            if !from_header && !from_events {
                why.push("reorder: no reorder share above 0 recorded".to_owned());
            }
        }
        Rule::CleanRef => {
            let clean: Vec<&Run<'_>> = set.iter().copied().filter(|r| r.role == "clean").collect();
            if clean.len() < 3 {
                why.push(format!(
                    "clean-ref: {} clean runs in the set, not 3",
                    clean.len()
                ));
                return;
            }
            let mut p99s = Vec::new();
            for r in &clean {
                match p99_work(r) {
                    Ok(p) => p99s.push(p),
                    Err(e) => {
                        why.push(format!(
                            "clean-ref: clean run {} has no tick record ({e})",
                            r.id
                        ));
                        return;
                    }
                }
            }
            p99s.sort();
            let n = p99s.len();
            let median = if n % 2 == 1 {
                p99s[n / 2]
            } else {
                (p99s[n / 2 - 1] + p99s[n / 2]).div_int(2)
            };
            for p in &p99s {
                if (*p - median).abs() > median * Ratio::new(25, 1000) {
                    why.push(format!("clean-ref: a clean run's p99 ({p} ns) is more than 2.5 % from their median ({median} ns)"));
                }
            }
        }
        Rule::LoggingOff => {
            // PM-0006-19's own rule: each "off" run's p99 within 1 % of the "off" runs' pooled p99.
            let off: Vec<&Run<'_>> = set
                .iter()
                .copied()
                .filter(|r| r.role == "logging-off")
                .collect();
            if off.is_empty() {
                why.push("PM-0006-19: no logging-off run in the set".to_owned());
                return;
            }
            let p99 = Stat::Pct(Ratio::new(99, 100));
            let mut pooled = Vec::new();
            let mut each = Vec::new();
            for r in &off {
                match collect_some(
                    r,
                    &Source::TickV("work_ns".to_owned()),
                    Window::Measured,
                    &BTreeMap::new(),
                ) {
                    Ok(v) => {
                        each.push((r.id, stat_of(&p99, None, &v)));
                        pooled.extend(v);
                    }
                    Err(e) => {
                        why.push(format!("PM-0006-19: {e}"));
                        return;
                    }
                }
            }
            let p = stat_of(&p99, None, &pooled);
            for (id, q) in each {
                if (q - p).abs() > p * Ratio::new(1, 100) {
                    why.push(format!(
                        "PM-0006-19: logging-off run {id}'s p99 ({q} ns) is more than 1 % from the pooled off p99 ({p} ns)"
                    ));
                }
            }
        }
        Rule::Pre(id) => {
            // The run must pass the precondition mark's checks that apply to it (the advisor's B8 ruling).
            let Some(pre) = ctx.marks.marks.iter().find(|m| &m.id == id) else {
                why.push(format!("pre:{id}: not in marks.json"));
                return;
            };
            let matched = matched_parts(pre, run);
            let checks: Vec<&crate::marks::Check> = pre
                .checks
                .iter()
                .filter(|c| {
                    if matched.is_empty() {
                        c.parts.is_empty()
                    } else {
                        matched.iter().any(|p| applies(c, &p.name))
                    }
                })
                .collect();
            if checks.is_empty() {
                undecided.push(format!("pre:{id}: none of its checks applies to this run"));
                return;
            }
            for c in checks {
                let o = eval_check(c, &[run], set, ctx);
                let note = o.note.clone().unwrap_or_default();
                match o.outcome {
                    Outcome::Pass => {}
                    Outcome::Fail => why.push(format!(
                        "pre:{id}: {} = {}, needs {}",
                        c.name,
                        o.value.as_deref().unwrap_or("?"),
                        o.needs
                    )),
                    Outcome::Empty => why.push(format!("pre:{id}: {}: {note}", c.name)),
                    Outcome::Gap => why.push(format!("pre:{id}: a gap in the record: {note}")),
                    Outcome::Cannot => undecided.push(format!("pre:{id}: {note}")),
                }
            }
        }
        Rule::WsPractice { tolerance, .. } => {
            if let Err(e) = practice::find(run, ctx.all, ctx.mark, *tolerance) {
                why.push(format!("ws-practice: {e}"));
            }
        }
    }
}

/// What the validity rules found: why the run is invalid, and what cannot be decided yet (NOT-RUN).
#[derive(Debug, Default)]
struct Validity {
    invalid: Vec<String>,
    undecided: Vec<String>,
}

/// The four rules that hold for every run, then each named rule.
fn validity<'r>(
    run: &Run<'_>,
    rules: impl IntoIterator<Item = &'r Rule>,
    set: &[&Run<'_>],
    ctx: &Ctx<'_>,
) -> Validity {
    let mut v = Validity {
        invalid: always_rules(run),
        undecided: Vec::new(),
    };
    for r in rules {
        rule(r, run, set, ctx, &mut v.invalid, &mut v.undecided);
    }
    v
}

/// What every judgement may consult: the whole marks file (for preconditions), the mark being judged (whose practice
/// run the ws-practice rule looks for), and every run the judge reads (where that practice run may be, in any set).
pub(crate) struct Ctx<'m> {
    marks: &'m Marks,
    mark: &'m Mark,
    all: &'m [Run<'m>],
}

/// The parts of a mark whose `select` a run matches, its role aside.
fn matched_parts<'m>(mark: &'m Mark, run: &Run<'_>) -> Vec<&'m Part> {
    mark.parts
        .iter()
        .filter(|p| {
            let mut s = p.select.clone();
            s.remove("role");
            run_matches(run, &s)
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------------------------
// Parts, bites and the verdict.

/// The verdicts of plan §6.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum VerdictKind {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "STAND-IN")]
    StandIn,
    #[serde(rename = "REPORTED")]
    Reported,
    #[serde(rename = "NOT-RUN")]
    NotRun,
}

impl fmt::Display for VerdictKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            VerdictKind::Pass => "PASS",
            VerdictKind::Fail => "FAIL",
            VerdictKind::Invalid => "INVALID",
            VerdictKind::StandIn => "STAND-IN",
            VerdictKind::Reported => "REPORTED",
            VerdictKind::NotRun => "NOT-RUN",
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunOut {
    pub run: String,
    /// The run counts: it held its validity rules and setup checks, and has no gap unless a check failed.
    pub valid: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub invalid_why: Vec<String>,
    /// Gaps in the record of a run that failed anyway: listed beside the failure, never hiding it.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gaps: Vec<String>,
    /// What cannot be decided yet (a precondition's figure not built, ...): the part is NOT-RUN.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub undecided: Vec<String>,
    pub checks: Vec<CheckOut>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PartOut {
    pub name: String,
    pub verdict: VerdictKind,
    pub why: Vec<String>,
    pub runs: Vec<RunOut>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pooled: Vec<CheckOut>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BiteStatus {
    /// It failed the mark, as it must.
    Shown,
    /// It passed every check: the check cannot be shown to work.
    Missed,
    Invalid,
    NotRun,
    Undecided,
}

#[derive(Debug, Clone, Serialize)]
pub struct BiteOut {
    pub id: String,
    pub status: BiteStatus,
    pub runs: Vec<RunOut>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportOut {
    pub name: String,
    pub part: String,
    pub run: String,
    pub value: String,
}

/// One mark's verdict, with everything that led to it.
#[derive(Debug, Clone, Serialize)]
pub struct Verdict {
    pub mark: String,
    pub title: String,
    pub verdict: VerdictKind,
    pub why: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<PartOut>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bites: Vec<BiteOut>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub report: Vec<ReportOut>,
}

fn applies(check: &crate::marks::Check, part: &str) -> bool {
    check.parts.is_empty() || check.parts.iter().any(|p| p == part)
}

fn part_select(part: &Part) -> BTreeMap<String, Value> {
    let mut s = part.select.clone();
    s.entry("role".to_owned())
        .or_insert_with(|| Value::from("measured"));
    s
}

fn set_of<'r, 'a>(all: &'r [Run<'a>], set: &str) -> Vec<&'r Run<'a>> {
    all.iter().filter(|r| r.set == set).collect()
}

/// Judges one run against a list of checks; gaps make it invalid.
/// Judges one run (README §3, the advisor's C2 ruling): its validity rules first; then its setup checks, whose failure
/// or gap makes it INVALID; then its checks. Any failing check fails the run, with its gaps listed beside the failure;
/// with no failure, any gap makes it INVALID.
fn judge_run<'c>(
    run: &Run<'_>,
    all: &[Run<'_>],
    rules: &[&Rule],
    checks: impl Iterator<Item = &'c crate::marks::Check>,
    ctx: &Ctx<'_>,
) -> RunOut {
    let set = set_of(all, run.set);
    let v = validity(run, rules.iter().copied(), &set, ctx);
    let mut out = RunOut {
        run: run.id.to_owned(),
        valid: v.invalid.is_empty(),
        invalid_why: v.invalid,
        gaps: Vec::new(),
        undecided: v.undecided,
        checks: Vec::new(),
    };
    if !out.valid {
        return out;
    }
    let (setup, plain): (Vec<_>, Vec<_>) = checks.partition(|c| c.setup);
    for c in setup {
        let o = eval_check(c, &[run], &set, ctx);
        let note = o.note.clone().unwrap_or_default();
        match o.outcome {
            Outcome::Pass => {}
            Outcome::Fail => out.invalid_why.push(format!(
                "setup: {} = {}, needs {}",
                c.name,
                o.value.as_deref().unwrap_or("?"),
                o.needs
            )),
            Outcome::Gap | Outcome::Empty => {
                out.invalid_why.push(format!("setup: {}: {note}", c.name))
            }
            Outcome::Cannot => out.undecided.push(note),
        }
        out.checks.push(o);
    }
    if !out.invalid_why.is_empty() {
        out.valid = false;
        return out;
    }
    for c in plain {
        out.checks.push(eval_check(c, &[run], &set, ctx));
    }
    let gaps: Vec<String> = out
        .checks
        .iter()
        .filter(|c| c.outcome == Outcome::Gap)
        .map(|c| {
            format!(
                "a gap in the record: {}",
                c.note.clone().unwrap_or_default()
            )
        })
        .collect();
    if out.checks.iter().any(|c| c.outcome.fails()) {
        out.gaps = gaps;
    } else if !gaps.is_empty() {
        out.invalid_why = gaps;
        out.valid = false;
    }
    out
}

/// The rules a run of these parts is judged by: the mark's, then each part's (a part adds, never removes).
fn rules_of<'m>(mark: &'m Mark, parts: &[&'m Part]) -> Vec<&'m Rule> {
    let mut rules: Vec<&Rule> = mark.validity.iter().collect();
    for p in parts {
        for r in &p.validity {
            if !rules.contains(&r) {
                rules.push(r);
            }
        }
    }
    rules
}

fn fail_lines(part: &str, r: &RunOut, out: &mut Vec<String>) {
    for c in r.checks.iter().filter(|c| c.outcome.fails()) {
        if c.outcome == Outcome::Empty {
            out.push(format!(
                "{part}: run {}: {}: {}; an empty comparison fails (needs {})",
                r.run,
                c.name,
                c.note.as_deref().unwrap_or(""),
                c.needs
            ));
            continue;
        }
        out.push(format!(
            "{part}: run {}: {} = {}, needs {}",
            r.run,
            c.name,
            c.value.as_deref().unwrap_or("?"),
            c.needs
        ));
    }
}

fn eval_part(
    mark: &Mark,
    part: &Part,
    members: &[&Run<'_>],
    all: &[Run<'_>],
    ctx: &Ctx<'_>,
) -> PartOut {
    let rules = rules_of(mark, &[part]);
    let runs: Vec<RunOut> = members
        .iter()
        .map(|r| {
            judge_run(
                r,
                all,
                &rules,
                mark.checks
                    .iter()
                    .filter(|c| !c.pooled && applies(c, &part.name)),
                ctx,
            )
        })
        .collect();
    let valid_runs: Vec<&Run<'_>> = members
        .iter()
        .zip(&runs)
        .filter(|(_, o)| o.valid)
        .map(|(r, _)| *r)
        .collect();
    let mut pooled = Vec::new();
    if !valid_runs.is_empty() {
        let set = set_of(all, valid_runs[0].set);
        for c in mark
            .checks
            .iter()
            .filter(|c| c.pooled && applies(c, &part.name))
        {
            pooled.push(eval_check(c, &valid_runs, &set, ctx));
        }
    }
    let mut why = Vec::new();
    let failed = runs
        .iter()
        .any(|r| r.valid && r.checks.iter().any(|c| c.outcome.fails()))
        || pooled.iter().any(|c| c.outcome.fails());
    let invalid = runs.iter().any(|r| !r.valid) || pooled.iter().any(|c| c.outcome == Outcome::Gap);
    let cannot = runs.iter().any(|r| {
        r.valid
            && (!r.undecided.is_empty() || r.checks.iter().any(|c| c.outcome == Outcome::Cannot))
    }) || pooled.iter().any(|c| c.outcome == Outcome::Cannot);
    let verdict = if failed {
        for r in runs.iter().filter(|r| r.valid) {
            fail_lines(&part.name, r, &mut why);
            for g in &r.gaps {
                why.push(format!("{}: run {}: also {g}", part.name, r.run));
            }
        }
        for c in pooled.iter().filter(|c| c.outcome.fails()) {
            why.push(format!(
                "{}: pooled: {} = {}, needs {}",
                part.name,
                c.name,
                c.value.as_deref().unwrap_or("?"),
                c.needs
            ));
        }
        VerdictKind::Fail
    } else if invalid {
        for r in runs.iter().filter(|r| !r.valid) {
            for w in &r.invalid_why {
                why.push(format!("{}: run {}: {w}", part.name, r.run));
            }
        }
        for c in pooled.iter().filter(|c| c.outcome == Outcome::Gap) {
            why.push(format!(
                "{}: pooled {}: {}",
                part.name,
                c.name,
                c.note.clone().unwrap_or_default()
            ));
        }
        VerdictKind::Invalid
    } else if cannot {
        let notes = runs
            .iter()
            .flat_map(|r| &r.checks)
            .chain(&pooled)
            .filter(|c| c.outcome == Outcome::Cannot)
            .filter_map(|c| c.note.clone())
            .chain(runs.iter().flat_map(|r| r.undecided.iter().cloned()));
        for n in notes {
            let line = format!("{}: {n}", part.name);
            if !why.contains(&line) {
                why.push(line);
            }
        }
        VerdictKind::NotRun
    } else if (valid_runs.len() as u32) < part.min {
        why.push(format!(
            "{}: {} of {} valid runs",
            part.name,
            valid_runs.len(),
            part.min
        ));
        VerdictKind::NotRun
    } else {
        VerdictKind::Pass
    };
    PartOut {
        name: part.name.clone(),
        verdict,
        why,
        runs,
        pooled,
    }
}

fn eval_bite(
    mark: &Mark,
    bite: &crate::marks::Bite,
    serving: &[&Run<'_>],
    all: &[Run<'_>],
    ctx: &Ctx<'_>,
) -> BiteOut {
    let bite_runs: Vec<&Run<'_>> = serving
        .iter()
        .copied()
        .filter(|r| r.role == "bite" && r.bite == Some(bite.id.as_str()))
        .collect();
    if bite_runs.is_empty() {
        return BiteOut {
            id: bite.id.clone(),
            status: BiteStatus::NotRun,
            runs: vec![],
        };
    }
    let mut outs = Vec::new();
    let mut statuses = Vec::new();
    for run in bite_runs {
        // A bite run takes the validity of the parts it matches (the advisor's B7 ruling).
        let matched = matched_parts(mark, run);
        let rules = rules_of(mark, &matched);
        if let Some(expected) = bite.rule.as_deref() {
            // An expect-invalid bite: shown only when its run is INVALID by that one rule and holds every other.
            let set = set_of(all, run.set);
            let mut others = Validity {
                invalid: always_rules(run),
                undecided: Vec::new(),
            };
            let mut by_rule = Vec::new();
            for r in &rules {
                if r.name() == expected {
                    rule(r, run, &set, ctx, &mut by_rule, &mut others.undecided);
                } else {
                    rule(
                        r,
                        run,
                        &set,
                        ctx,
                        &mut others.invalid,
                        &mut others.undecided,
                    );
                }
            }
            statuses.push(if !others.invalid.is_empty() {
                BiteStatus::Invalid
            } else if !others.undecided.is_empty() {
                BiteStatus::Undecided
            } else if by_rule.is_empty() {
                BiteStatus::Missed
            } else {
                BiteStatus::Shown
            });
            outs.push(RunOut {
                run: run.id.to_owned(),
                valid: by_rule.is_empty() && others.invalid.is_empty(),
                invalid_why: others.invalid.into_iter().chain(by_rule).collect(),
                gaps: Vec::new(),
                undecided: others.undecided,
                checks: Vec::new(),
            });
            continue;
        }
        let checks = mark.checks.iter().filter(|c| {
            if matched.is_empty() {
                c.parts.is_empty()
            } else {
                matched.iter().any(|p| applies(c, &p.name))
            }
        });
        let mut o = judge_run(run, all, &rules, checks, ctx);
        // Shown when a check fails on a run that held its validity rules and setup checks (C2: gaps aside). A bite
        // run whose only failure is an empty comparison compared nothing, so it did not test its bite: INVALID.
        if o.valid
            && !o.checks.iter().any(|c| c.outcome == Outcome::Fail)
            && o.checks.iter().any(|c| c.outcome == Outcome::Empty)
        {
            o.valid = false;
            for c in o.checks.iter().filter(|c| c.outcome == Outcome::Empty) {
                o.invalid_why.push(format!(
                    "{}: {}; an empty comparison shows no bite",
                    c.name,
                    c.note.as_deref().unwrap_or("")
                ));
            }
        }
        statuses.push(if !o.valid {
            BiteStatus::Invalid
        } else if o.checks.iter().any(|c| c.outcome == Outcome::Fail) {
            BiteStatus::Shown
        } else if !o.undecided.is_empty()
            || o.checks.iter().any(|c| c.outcome == Outcome::Cannot)
            || o.checks.iter().all(|c| c.setup)
        {
            BiteStatus::Undecided
        } else {
            BiteStatus::Missed
        });
        outs.push(o);
    }
    let status = [
        BiteStatus::Missed,
        BiteStatus::Invalid,
        BiteStatus::Undecided,
    ]
    .into_iter()
    .find(|s| statuses.contains(s))
    .unwrap_or(BiteStatus::Shown);
    BiteOut {
        id: bite.id.clone(),
        status,
        runs: outs,
    }
}

fn report_values(
    mark: &Mark,
    parts: &[PartOut],
    members: &BTreeMap<String, Vec<&Run<'_>>>,
    all: &[Run<'_>],
) -> Vec<ReportOut> {
    let text = |v: Result<Ratio, Missing>| match v {
        Ok(v) => v.to_string(),
        Err(Missing::Gap(why) | Missing::NotBuilt(why) | Missing::Empty(why)) => {
            format!("not available: {why}")
        }
    };
    let mut out = Vec::new();
    for part in parts {
        let Some(runs) = members.get(&part.name) else {
            continue;
        };
        let valid: Vec<&Run<'_>> = runs
            .iter()
            .zip(&part.runs)
            .filter(|(_, ro)| ro.valid)
            .map(|(r, _)| *r)
            .collect();
        if valid.is_empty() {
            continue;
        }
        let set = set_of(all, valid[0].set);
        for r in mark
            .report
            .iter()
            .filter(|r| r.parts.is_empty() || r.parts.contains(&part.name))
        {
            if r.pooled {
                out.push(ReportOut {
                    name: r.name.clone(),
                    part: part.name.clone(),
                    run: format!("pooled over {} runs", valid.len()),
                    value: text(measure(&r.measure, &valid, &set, &r.from)),
                });
                continue;
            }
            for run in &valid {
                out.push(ReportOut {
                    name: r.name.clone(),
                    part: part.name.clone(),
                    run: run.id.to_owned(),
                    value: text(measure(&r.measure, &[*run], &set, &r.from)),
                });
            }
        }
    }
    out
}

fn judge_person(mark: &Mark, serving: &[&Run<'_>]) -> (VerdictKind, Vec<String>) {
    let mut why = Vec::new();
    let mut seen = 0;
    let mut failed = false;
    let mut invalid = false;
    for run in serving {
        let events: Vec<_> = run
            .logs
            .iter()
            .flat_map(|l| &l.events)
            .filter(|e| {
                e.ev == "person"
                    && e.f.get("mark").and_then(Value::as_str) == Some(mark.id.as_str())
            })
            .collect();
        if events.is_empty() {
            continue;
        }
        let rules = always_rules(run);
        if !rules.is_empty() {
            invalid = true;
            why.extend(rules.into_iter().map(|w| format!("run {}: {w}", run.id)));
            continue;
        }
        for e in events {
            seen += 1;
            let who = e.f.get("who").and_then(Value::as_str).unwrap_or("?");
            match e.f.get("verdict").and_then(Value::as_str) {
                Some("pass") => why.push(format!("run {}: {who}: pass", run.id)),
                Some("fail") => {
                    failed = true;
                    why.push(format!("run {}: {who}: fail", run.id));
                }
                other => {
                    invalid = true;
                    why.push(format!(
                        "run {}: {who}'s verdict {other:?} is not pass or fail",
                        run.id
                    ));
                }
            }
        }
    }
    let kind = if failed {
        VerdictKind::Fail
    } else if invalid {
        VerdictKind::Invalid
    } else if seen == 0 {
        why.push("no person record yet".to_owned());
        VerdictKind::NotRun
    } else {
        VerdictKind::Pass
    };
    (kind, why)
}

fn judge_mark(mark: &Mark, all: &[Run<'_>], ctx: &Ctx<'_>) -> Verdict {
    let serving: Vec<&Run<'_>> = all.iter().filter(|r| r.serves(&mark.id)).collect();
    let mut v = Verdict {
        mark: mark.id.clone(),
        title: mark.title.clone(),
        verdict: VerdictKind::NotRun,
        why: vec![],
        parts: vec![],
        bites: vec![],
        report: vec![],
    };
    match mark.kind {
        Kind::Outside => {
            v.why
                .push(format!("outside this change-set: {}", mark.notes));
            return v;
        }
        Kind::Person => {
            let (kind, why) = judge_person(mark, &serving);
            v.verdict = kind;
            v.why = why;
            return v;
        }
        Kind::Pass | Kind::Reported => {}
    }
    let mut members: BTreeMap<String, Vec<&Run<'_>>> = BTreeMap::new();
    for part in &mark.parts {
        let s = part_select(part);
        let m: Vec<&Run<'_>> = serving
            .iter()
            .copied()
            .filter(|r| run_matches(r, &s))
            .collect();
        v.parts.push(eval_part(mark, part, &m, all, ctx));
        members.insert(part.name.clone(), m);
    }
    v.bites = mark
        .bites
        .iter()
        .map(|b| eval_bite(mark, b, &serving, all, ctx))
        .collect();
    let part_is = |k: VerdictKind| v.parts.iter().any(|p| p.verdict == k);
    let bite_is = |s: BiteStatus| v.bites.iter().any(|b| b.status == s);
    let mut why = Vec::new();
    let kind = if part_is(VerdictKind::Fail) || bite_is(BiteStatus::Missed) {
        VerdictKind::Fail
    } else if part_is(VerdictKind::Invalid) || bite_is(BiteStatus::Invalid) {
        VerdictKind::Invalid
    } else if part_is(VerdictKind::NotRun)
        || bite_is(BiteStatus::NotRun)
        || bite_is(BiteStatus::Undecided)
    {
        VerdictKind::NotRun
    } else if mark.kind == Kind::Reported {
        VerdictKind::Reported
    } else {
        let lighter = members.values().flatten().any(|r| {
            r.logs
                .iter()
                .any(|l| l.header.params.get("sim") == Some(&Value::from("stand-in")))
        });
        if mark.stand_in && lighter {
            why.push(
                "passed on a sim lighter than the stress scene (ADR 0001's stand-in label)"
                    .to_owned(),
            );
            VerdictKind::StandIn
        } else {
            VerdictKind::Pass
        }
    };
    for p in &v.parts {
        if kind != VerdictKind::Pass && kind != VerdictKind::Reported && p.verdict == kind {
            why.extend(p.why.iter().cloned());
        }
    }
    for b in &v.bites {
        let line = match b.status {
            BiteStatus::Missed => Some(format!(
                "bite {} did not fail: the check cannot be shown to work",
                b.id
            )),
            BiteStatus::NotRun => Some(format!("bite {}: no run yet", b.id)),
            BiteStatus::Invalid => Some(format!("bite {}: its run is invalid", b.id)),
            BiteStatus::Undecided => Some(format!(
                "bite {}: undecided (a check cannot be computed yet)",
                b.id
            )),
            BiteStatus::Shown => None,
        };
        let wanted = match kind {
            VerdictKind::Fail => b.status == BiteStatus::Missed,
            VerdictKind::Invalid => b.status == BiteStatus::Invalid,
            VerdictKind::NotRun => matches!(b.status, BiteStatus::NotRun | BiteStatus::Undecided),
            _ => false,
        };
        if wanted {
            why.extend(line);
        }
    }
    if mark.arm_rule && kind == VerdictKind::Fail {
        why.push("the Arm rule (ADR 0001): an aarch64-only mismatch is a cross-architecture finding; this judge applies it to the decoded hashes (decoded_hash_mismatch_count, against the x86-64 re-decode), and the shot view test's and throw landing's kinds with their figures (U4h)".to_owned());
    }
    v.report = report_values(mark, &v.parts, &members, all);
    v.verdict = kind;
    v.why = why;
    v
}

/// One verdict per mark, in `marks.json`'s order.
pub fn judge(marks: &Marks, logs: &[Log]) -> Vec<Verdict> {
    let all = runs(logs);
    marks
        .marks
        .iter()
        .map(|m| {
            judge_mark(
                m,
                &all,
                &Ctx {
                    marks,
                    mark: m,
                    all: &all,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marks::Op;

    #[test]
    fn nearest_rank_matches_the_plan() {
        let p99 = Ratio::new(99, 100);
        assert_eq!(nearest_rank(36_000, p99), 35_640);
        assert_eq!(nearest_rank(18_000, p99), 17_820);
        assert_eq!(nearest_rank(72_000, p99), 71_280);
        assert_eq!(nearest_rank(54_000, p99), 53_460);
        assert_eq!(nearest_rank(1, p99), 1);
        assert_eq!(nearest_rank(100, p99), 99);
        assert_eq!(nearest_rank(101, p99), 100);
        assert_eq!(nearest_rank(1_000, Ratio::new(999, 1000)), 999);
        assert_eq!(nearest_rank(2, Ratio::new(1, 2)), 1);
        assert_eq!(nearest_rank(3, Ratio::new(1, 2)), 2);
    }

    #[test]
    fn statistics_are_exact() {
        let v: Vec<i64> = (1..=100).collect();
        assert_eq!(
            stat_of(&Stat::Pct(Ratio::new(99, 100)), None, &v),
            Ratio::int(99)
        );
        assert_eq!(stat_of(&Stat::Mean, None, &[1, 2]), Ratio::new(3, 2));
        assert_eq!(
            stat_of(&Stat::Sum, None, &[i64::MAX, i64::MAX]),
            Ratio::int(2 * i128::from(i64::MAX))
        );
        let over = Cond {
            op: Op::Gt,
            value: Ratio::int(90),
        };
        assert_eq!(stat_of(&Stat::Share, Some(over), &v), Ratio::new(1, 10));
        assert_eq!(stat_of(&Stat::Count, Some(over), &v), Ratio::int(10));
        assert_eq!(stat_of(&Stat::Max, None, &[3, -7, 5]), Ratio::int(5));
        assert_eq!(stat_of(&Stat::Min, None, &[3, -7, 5]), Ratio::int(-7));
    }
}
