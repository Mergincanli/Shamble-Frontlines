//! `marks.json`, read and checked against `marks/README.md` §2, then compiled into typed checks. Every problem is
//! collected and reported at once; a judge with an unreadable mark refuses to judge.

use crate::figures;
use crate::ratio::Ratio;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// The format version of `marks.json` this judge reads.
pub const SCHEMA: u32 = 1;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    schema: u32,
    drafts_commit: String,
    marks: Vec<RawMark>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMark {
    id: String,
    title: String,
    draft: Draft,
    verdict: String,
    #[serde(default, rename = "where")]
    where_: Vec<String>,
    #[serde(default)]
    parts: Vec<RawPart>,
    #[serde(default)]
    validity: Vec<String>,
    #[serde(default)]
    checks: Vec<RawCheck>,
    #[serde(default)]
    report: Vec<RawCheck>,
    #[serde(default)]
    bites: Vec<Bite>,
    #[serde(default)]
    stand_in: bool,
    #[serde(default)]
    arm_rule: bool,
    /// PM-0001-18's WebSocket bound line, verbatim (README §2.5): only on a mark with `ws_bound` checks, which must
    /// hold it.
    #[serde(default)]
    ws_bound_line: Option<String>,
    #[serde(default)]
    readings: Vec<String>,
    #[serde(default)]
    notes: String,
}

/// Where a mark's row lives, and the hash the drift check compares.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Draft {
    pub file: String,
    pub row_sha256: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPart {
    name: String,
    #[serde(default)]
    select: BTreeMap<String, Value>,
    min: u32,
    /// Rules for this part's runs, added to the mark's (never removing any).
    #[serde(default)]
    validity: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCond {
    op: String,
    value: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAgainst {
    roles: Vec<String>,
    combine: String,
}

/// A check (`checks`) or a reported statistic (`report`): the same shape, with no comparison in a report.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCheck {
    name: String,
    #[serde(default)]
    series: Option<String>,
    #[serde(default)]
    figure: Option<String>,
    #[serde(default)]
    stat: Option<String>,
    #[serde(default)]
    window: Option<String>,
    #[serde(default, rename = "where")]
    where_: Option<RawCond>,
    #[serde(default)]
    op: Option<String>,
    #[serde(default)]
    value: Option<Value>,
    #[serde(default)]
    factor: Option<String>,
    #[serde(default)]
    against: Option<RawAgainst>,
    #[serde(default)]
    from: BTreeMap<String, Value>,
    #[serde(default)]
    parts: Vec<String>,
    #[serde(default)]
    pooled: bool,
    #[serde(default)]
    setup: bool,
    #[serde(default)]
    first: Option<u64>,
    /// `x`, `y` or `z`: the check's number is that number of the WebSocket bound, applied times the practice run's
    /// factor (README §2.3, the `ws-practice` rule).
    #[serde(default)]
    ws_bound: Option<String>,
}

/// A planted defect that must fail the mark first (plan §6.1, "Bites first"); or, with `expect: "invalid"`, one that
/// must make its run INVALID by the named rule alone (PM-0001-30's "the validity check itself").
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bite {
    pub id: String,
    pub what: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

/// The mark's kind (`verdict` in the file).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Pass,
    Reported,
    Person,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Le,
    Lt,
    Eq,
    Ge,
    Gt,
}

impl Op {
    fn parse(s: &str) -> Result<Op, String> {
        Ok(match s {
            "<=" => Op::Le,
            "<" => Op::Lt,
            "==" => Op::Eq,
            ">=" => Op::Ge,
            ">" => Op::Gt,
            _ => return Err(format!("op {s:?} is not one of <=, <, ==, >=, >")),
        })
    }

    pub fn holds(self, a: Ratio, b: Ratio) -> bool {
        match self {
            Op::Le => a <= b,
            Op::Lt => a < b,
            Op::Eq => a == b,
            Op::Ge => a >= b,
            Op::Gt => a > b,
        }
    }

    pub fn text(self) -> &'static str {
        match self {
            Op::Le => "<=",
            Op::Lt => "<",
            Op::Eq => "==",
            Op::Ge => ">=",
            Op::Gt => ">",
        }
    }
}

/// A condition on single values (`where`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cond {
    pub op: Op,
    pub value: Ratio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    Measured,
    All,
    Warmup,
}

/// Where a series' values come from (README §2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    TickV(String),
    TickS(String),
    SecondV(String),
    SecondS(String),
    Event { ev: String, field: String },
    HeaderParam(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stat {
    Count,
    Sum,
    Min,
    Max,
    Mean,
    /// A nearest-rank percentile, as a fraction (p99 = 99/100).
    Pct(Ratio),
    Share,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combine {
    Mean,
    Median,
    Pooled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Against {
    pub roles: Vec<String>,
    pub combine: Combine,
}

/// What a check or a reported statistic measures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Measure {
    Series {
        source: Source,
        stat: Stat,
        window: Window,
        cond: Option<Cond>,
        /// Only the first N records of each log in the window (PM-0004-67's first 36,000 measured ticks).
        first: Option<u64>,
    },
    Figure(String),
}

/// The comparison a check makes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Threshold {
    Value(Ratio),
    Against { factor: Ratio, against: Against },
}

/// Which number of PM-0001-18's WebSocket bound a check holds (README §2.3, `ws_bound`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundNumber {
    /// The median's limit: the check's `value`.
    X,
    /// The count's limit: the check's `where` value is Y and its `value` is k.
    Y,
    /// The maximum's limit: the check's `value`.
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub name: String,
    pub measure: Measure,
    pub op: Op,
    pub threshold: Threshold,
    pub from: BTreeMap<String, Value>,
    pub parts: Vec<String>,
    pub pooled: bool,
    /// A setup condition: when it fails, the run did not test the mark, so it is INVALID, never FAIL.
    pub setup: bool,
    /// The check holds a number of the WebSocket bound, which the judge applies times the practice run's factor.
    pub ws_bound: Option<BoundNumber>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub name: String,
    pub measure: Measure,
    pub from: BTreeMap<String, Value>,
    /// The parts it is computed in; empty for every part.
    pub parts: Vec<String>,
    /// Computed once over the part's valid runs together, not run by run.
    pub pooled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub name: String,
    pub select: BTreeMap<String, Value>,
    pub min: u32,
    /// Rules for this part's runs, beside the mark's.
    pub validity: Vec<Rule>,
}

/// A validity rule a mark or a part may name (README §4); the four that hold for every run are not listed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    VS,
    Density,
    Lg80,
    Padding,
    LateZero,
    Reorder,
    CleanRef,
    /// PM-0006-19's own rule: each logging-off run's p99 within 1 % of the off runs' pooled p99.
    LoggingOff,
    /// The run must pass another mark's checks (`pre:PM-0001-2`: it carried the stress scene's load).
    Pre(String),
    /// PM-0001-18's WebSocket part (`ws-practice:<tolerance>`): the run's set holds its practice run, whose record is
    /// in tolerance, and the numbers in force are the bound's times its factor (README §4).
    WsPractice {
        /// The largest slowdown the numbers may move by (Mustafa's 10 %: 1.10, never raised).
        tolerance: Ratio,
        /// The tolerance as `marks.json` writes it.
        text: String,
    },
}

impl Rule {
    pub(crate) fn parse(s: &str) -> Result<Rule, String> {
        Ok(match s {
            "V-S" => Rule::VS,
            "PM-0006-13" => Rule::Density,
            "PM-0006-19" => Rule::LoggingOff,
            "lg-80" => Rule::Lg80,
            "padding" => Rule::Padding,
            "late-zero" => Rule::LateZero,
            "reorder" => Rule::Reorder,
            "clean-ref" => Rule::CleanRef,
            other => {
                if let Some(id) = other.strip_prefix("pre:") {
                    if id.starts_with("PM-000") {
                        return Ok(Rule::Pre(id.to_owned()));
                    }
                } else if let Some(text) = other.strip_prefix("ws-practice:") {
                    let tolerance = Ratio::parse(text)?;
                    if tolerance < Ratio::int(1) || tolerance > Ratio::new(11, 10) {
                        return Err(format!(
                            "{s:?}: the tolerance is at least 1 and at most 1.10 (Mustafa's 10 %, never raised)"
                        ));
                    }
                    return Ok(Rule::WsPractice {
                        tolerance,
                        text: text.to_owned(),
                    });
                }
                return Err(format!("validity rule {s:?} is not one of README §4's"));
            }
        })
    }

    /// The rule's name as `marks.json` writes it.
    pub fn name(&self) -> String {
        match self {
            Rule::VS => "V-S".to_owned(),
            Rule::Density => "PM-0006-13".to_owned(),
            Rule::LoggingOff => "PM-0006-19".to_owned(),
            Rule::Lg80 => "lg-80".to_owned(),
            Rule::Padding => "padding".to_owned(),
            Rule::LateZero => "late-zero".to_owned(),
            Rule::Reorder => "reorder".to_owned(),
            Rule::CleanRef => "clean-ref".to_owned(),
            Rule::Pre(id) => format!("pre:{id}"),
            Rule::WsPractice { text, .. } => format!("ws-practice:{text}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    pub id: String,
    pub title: String,
    pub draft: Draft,
    pub kind: Kind,
    pub where_: Vec<String>,
    pub parts: Vec<Part>,
    pub validity: Vec<Rule>,
    pub checks: Vec<Check>,
    pub report: Vec<Report>,
    pub bites: Vec<Bite>,
    pub stand_in: bool,
    pub arm_rule: bool,
    /// The WebSocket bound's machine-readable line, verbatim (README §2.5; PM-0001-18): checked at load against the
    /// mark's `ws_bound` checks and its `ws-practice` tolerance; a practice record's `bound` must equal it byte for
    /// byte.
    pub ws_bound_line: Option<String>,
    pub readings: Vec<String>,
    pub notes: String,
}

/// Every mark of one `marks.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marks {
    pub drafts_commit: String,
    pub marks: Vec<Mark>,
}

/// The header fields a part's `select` or a check's `from` may name (README §2.2).
pub const SELECT_KEYS: [&str; 10] = [
    "layer",
    "profile",
    "role",
    "scene",
    "tick_hz",
    "clients",
    "process",
    "machine.name",
    "machine.role",
    "machine.arch",
];

fn valid_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z'))
        && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

fn check_keys(keys: &BTreeMap<String, Value>, what: &str) -> Result<(), String> {
    for key in keys.keys() {
        let ok = SELECT_KEYS.contains(&key.as_str())
            || key.strip_prefix("params.").is_some_and(valid_name);
        if !ok {
            return Err(format!(
                "{what} names {key:?}, which is not a header field it may select on"
            ));
        }
    }
    Ok(())
}

fn parse_source(s: &str) -> Result<Source, String> {
    let parts: Vec<&str> = s.split('.').collect();
    let bad = || format!("series {s:?} is not one of README §2.3's forms");
    let named = |n: &str| {
        if valid_name(n) {
            Ok(n.to_owned())
        } else {
            Err(bad())
        }
    };
    match parts.as_slice() {
        ["tick", "v", n] => Ok(Source::TickV(named(n)?)),
        ["tick", "s", n] => Ok(Source::TickS(named(n)?)),
        ["second", "v", n] => Ok(Source::SecondV(named(n)?)),
        ["second", "s", n] => Ok(Source::SecondS(named(n)?)),
        ["event", ev, field] => Ok(Source::Event {
            ev: named(ev)?,
            field: named(field)?,
        }),
        ["header", "params", n] => Ok(Source::HeaderParam(named(n)?)),
        _ => Err(bad()),
    }
}

fn parse_stat(s: &str) -> Result<Stat, String> {
    Ok(match s {
        "count" => Stat::Count,
        "sum" => Stat::Sum,
        "min" => Stat::Min,
        "max" => Stat::Max,
        "mean" => Stat::Mean,
        "share" => Stat::Share,
        "p50" => Stat::Pct(Ratio::new(1, 2)),
        "p90" => Stat::Pct(Ratio::new(9, 10)),
        "p99" => Stat::Pct(Ratio::new(99, 100)),
        "p999" => Stat::Pct(Ratio::new(999, 1000)),
        "p001" => Stat::Pct(Ratio::new(1, 1000)),
        _ => return Err(format!("stat {s:?} is not one of README §2.3's")),
    })
}

/// An integer, or a decimal written as a string.
fn parse_number(v: &Value) -> Result<Ratio, String> {
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Ratio::int(i128::from(i)))
            } else if let Some(u) = n.as_u64() {
                Ok(Ratio::int(i128::from(u)))
            } else {
                Err(format!(
                    "{n} is a float; write a fraction as a decimal string, such as \"0.999\""
                ))
            }
        }
        Value::String(s) => Ratio::parse(s),
        Value::Bool(b) => Ok(Ratio::int(i128::from(*b))),
        _ => Err(format!("{v} is not a number")),
    }
}

fn compile_measure(c: &RawCheck) -> Result<Measure, String> {
    match (&c.series, &c.figure) {
        (Some(series), None) => {
            let source = parse_source(series)?;
            let stat = parse_stat(c.stat.as_deref().ok_or("a series needs a stat")?)?;
            let window = match c.window.as_deref().unwrap_or("measured") {
                "measured" => Window::Measured,
                "all" => Window::All,
                "warmup" => Window::Warmup,
                w => return Err(format!("window {w:?} is not measured, all or warmup")),
            };
            let cond = match &c.where_ {
                Some(w) => Some(Cond {
                    op: Op::parse(&w.op)?,
                    value: parse_number(&w.value)?,
                }),
                None => None,
            };
            match (&stat, &cond) {
                (Stat::Share, None) => return Err("a share needs a where".to_owned()),
                (Stat::Share | Stat::Count, _) | (_, None) => {}
                (_, Some(_)) => return Err("only count and share take a where".to_owned()),
            }
            if matches!(source, Source::HeaderParam(_)) && (c.window.is_some() || c.first.is_some())
            {
                return Err("a header value has no window".to_owned());
            }
            if c.first == Some(0) {
                return Err("first needs at least 1 record".to_owned());
            }
            Ok(Measure::Series {
                source,
                stat,
                window,
                cond,
                first: c.first,
            })
        }
        (None, Some(figure)) => {
            if !figures::known(figure) {
                return Err(format!("figure {figure:?} is not in the judge's list"));
            }
            if c.stat.is_some() || c.window.is_some() || c.where_.is_some() || c.first.is_some() {
                return Err("a figure takes no stat, window, where or first".to_owned());
            }
            Ok(Measure::Figure(figure.clone()))
        }
        _ => Err("exactly one of series and figure".to_owned()),
    }
}

fn compile_check(c: &RawCheck, part_names: &BTreeSet<&str>) -> Result<Check, String> {
    let measure = compile_measure(c)?;
    let op = Op::parse(c.op.as_deref().ok_or("a check needs an op")?)?;
    let threshold = match (&c.value, &c.factor, &c.against) {
        (Some(v), None, None) => Threshold::Value(parse_number(v)?),
        (None, Some(f), Some(a)) => {
            if a.roles.is_empty() {
                return Err("against needs at least one role".to_owned());
            }
            let combine = match a.combine.as_str() {
                "mean" => Combine::Mean,
                "median" => Combine::Median,
                "pooled" => Combine::Pooled,
                other => return Err(format!("combine {other:?} is not mean, median or pooled")),
            };
            if matches!(measure, Measure::Figure(_)) {
                return Err("against compares a series' statistic, not a figure".to_owned());
            }
            Threshold::Against {
                factor: Ratio::parse(f)?,
                against: Against {
                    roles: a.roles.clone(),
                    combine,
                },
            }
        }
        _ => return Err("a check needs either value, or factor with against".to_owned()),
    };
    check_keys(&c.from, "from")?;
    for p in &c.parts {
        if !part_names.contains(p.as_str()) {
            return Err(format!(
                "check names part {p:?}, which the mark does not have"
            ));
        }
    }
    if c.setup && (c.pooled || matches!(threshold, Threshold::Against { .. })) {
        return Err("a setup check reads its own run only: no pooled or against".to_owned());
    }
    let ws_bound = match c.ws_bound.as_deref() {
        None => None,
        Some(letter) => {
            let n = match letter {
                "x" => BoundNumber::X,
                "y" => BoundNumber::Y,
                "z" => BoundNumber::Z,
                other => return Err(format!("ws_bound {other:?} is not x, y or z")),
            };
            let Measure::Series { stat, cond, .. } = &measure else {
                return Err("ws_bound holds a series check's number, not a figure's".to_owned());
            };
            if c.pooled || c.setup || !matches!(threshold, Threshold::Value(_)) {
                return Err(
                    "a ws_bound check is a plain check: no pooled, setup or against".to_owned(),
                );
            }
            if c.parts.is_empty() {
                return Err("a ws_bound check names its parts (each with ws-practice)".to_owned());
            }
            // Each letter's shape is pinned (SEC-F11 on U4j): the judge moves exactly these limits by f.
            let (shape, want) = match n {
                BoundNumber::X => (
                    matches!(stat, Stat::Pct(q) if *q == Ratio::new(1, 2))
                        && cond.is_none()
                        && op == Op::Le,
                    "stat p50, op <= and no where (the median at most X)",
                ),
                BoundNumber::Y => (
                    matches!(stat, Stat::Count)
                        && matches!(cond, Some(w) if w.op == Op::Gt)
                        && op == Op::Le,
                    "stat count, a where with op > and op <= (at most k over Y)",
                ),
                BoundNumber::Z => (
                    matches!(stat, Stat::Max) && cond.is_none() && op == Op::Le,
                    "stat max, op <= and no where (none over Z)",
                ),
            };
            if !shape {
                return Err(format!("ws_bound {letter} is {want}; its shape is another"));
            }
            Some(n)
        }
    };
    Ok(Check {
        name: c.name.clone(),
        measure,
        op,
        threshold,
        from: c.from.clone(),
        parts: c.parts.clone(),
        pooled: c.pooled,
        setup: c.setup,
        ws_bound,
    })
}

fn compile_report(c: &RawCheck, part_names: &BTreeSet<&str>) -> Result<Report, String> {
    if c.setup || c.ws_bound.is_some() {
        return Err(
            "a reported statistic decides nothing, so it is no setup or ws_bound check".to_owned(),
        );
    }
    if c.op.is_some() || c.value.is_some() || c.factor.is_some() || c.against.is_some() {
        return Err(
            "a reported statistic decides nothing: no op, value, factor or against".to_owned(),
        );
    }
    for p in &c.parts {
        if !part_names.contains(p.as_str()) {
            return Err(format!(
                "report names part {p:?}, which the mark does not have"
            ));
        }
    }
    check_keys(&c.from, "from")?;
    let measure = compile_measure(c)?;
    if c.pooled && matches!(measure, Measure::Figure(_)) {
        return Err("a figure cannot be pooled".to_owned());
    }
    Ok(Report {
        name: c.name.clone(),
        measure,
        from: c.from.clone(),
        parts: c.parts.clone(),
        pooled: c.pooled,
    })
}

fn compile_mark(m: &RawMark) -> Result<Mark, Vec<String>> {
    let mut errors = Vec::new();
    let kind = match m.verdict.as_str() {
        "pass" => Kind::Pass,
        "reported" => Kind::Reported,
        "person" => Kind::Person,
        "outside" => Kind::Outside,
        other => {
            errors.push(format!(
                "verdict {other:?} is not pass, reported, person or outside"
            ));
            Kind::Outside
        }
    };
    let id_ok = m.id.starts_with("PM-000") && m.id.len() > 8;
    if !id_ok {
        errors.push(format!("id {:?} is not a pass mark id", m.id));
    }
    if m.title.trim().is_empty() {
        errors.push("an empty title".to_owned());
    }
    let hash_ok = m.draft.row_sha256.len() == 64
        && m.draft
            .row_sha256
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if !hash_ok {
        errors.push("draft.row_sha256 is not 64 lowercase hex digits".to_owned());
    }
    if !m.draft.file.starts_with("docs/adr/") {
        errors.push(format!(
            "draft.file {:?} is not under docs/adr/",
            m.draft.file
        ));
    }
    let mut part_names = BTreeSet::new();
    let mut parts = Vec::new();
    for p in &m.parts {
        if !part_names.insert(p.name.as_str()) {
            errors.push(format!("part {:?} twice", p.name));
        }
        if let Err(e) = check_keys(&p.select, "select") {
            errors.push(format!("part {:?}: {e}", p.name));
        }
        if p.min == 0 {
            errors.push(format!("part {:?} needs min of at least 1", p.name));
        }
        let mut part_rules = Vec::new();
        for v in &p.validity {
            match Rule::parse(v) {
                Ok(r) => part_rules.push(r),
                Err(e) => errors.push(format!("part {:?}: {e}", p.name)),
            }
        }
        parts.push(Part {
            name: p.name.clone(),
            select: p.select.clone(),
            min: p.min,
            validity: part_rules,
        });
    }
    let mut checks = Vec::new();
    for c in &m.checks {
        match compile_check(c, &part_names) {
            Ok(c) => checks.push(c),
            Err(e) => errors.push(format!("check {:?}: {e}", c.name)),
        }
    }
    let mut report = Vec::new();
    for r in &m.report {
        match compile_report(r, &part_names) {
            Ok(r) => report.push(r),
            Err(e) => errors.push(format!("report {:?}: {e}", r.name)),
        }
    }
    let mut validity = Vec::new();
    for v in &m.validity {
        match Rule::parse(v) {
            Ok(r) => validity.push(r),
            Err(e) => errors.push(e),
        }
    }
    // A ws_bound check sits only on parts the ws-practice rule judges, and each bound number has one check: the rule
    // compares the bound's numbers with these checks'.
    let practice = |r: &Rule| matches!(r, Rule::WsPractice { .. });
    let mut letters = BTreeSet::new();
    for c in checks.iter().filter(|c| c.ws_bound.is_some()) {
        if !letters.insert(format!("{:?}", c.ws_bound)) {
            errors.push(format!(
                "check {:?}: a second check for ws_bound {:?}",
                c.name, c.ws_bound
            ));
        }
        for p in &c.parts {
            let ruled = validity.iter().any(practice)
                || parts
                    .iter()
                    .any(|q| &q.name == p && q.validity.iter().any(practice));
            if !ruled {
                errors.push(format!(
                    "check {:?}: its part {p:?} is not judged by ws-practice",
                    c.name
                ));
            }
        }
    }
    // The bound's line (the advisor's FX-6): held by a mark with ws_bound checks, and only by one; its X, k, Y and Z are
    // the checks' and its tolerance every ws-practice rule's, so M_m, the band, the guard, the model and the commit
    // come from reviewed marks.json, never from a log.
    let bound_checks = m.checks.iter().any(|c| c.ws_bound.is_some());
    match (&m.ws_bound_line, bound_checks) {
        (None, true) => errors.push(
            "ws_bound_line: a mark with ws_bound checks holds the bound's line (README §2.5)"
                .to_owned(),
        ),
        (Some(_), false) => errors
            .push("ws_bound_line: only a mark with ws_bound checks holds a bound line".to_owned()),
        (Some(line), true) => match crate::practice::parse_bound(line) {
            Err(e) => errors.push(format!("ws_bound_line: {e}")),
            Ok(bound) => {
                if let Err(e) = crate::practice::checks_agree(&bound, &checks) {
                    errors.push(format!("ws_bound_line: {e}"));
                }
                for r in validity
                    .iter()
                    .chain(parts.iter().flat_map(|p| &p.validity))
                {
                    if let Rule::WsPractice { tolerance, .. } = r
                        && *tolerance != bound.tolerance
                    {
                        errors.push(format!(
                            "ws_bound_line: its tolerance {} is not the ws-practice rule's {tolerance}",
                            bound.tolerance
                        ));
                    }
                }
            }
        },
        (None, false) => {}
    }
    // The shape is judged on the file's lists, so a check that failed to compile is reported once, not twice.
    match kind {
        Kind::Pass => {
            if m.parts.is_empty() || m.checks.is_empty() {
                errors.push("a pass mark needs at least one part and one check".to_owned());
            }
        }
        Kind::Reported => {
            if m.parts.is_empty() || !m.checks.is_empty() || !m.bites.is_empty() {
                errors.push("a reported mark needs parts, and has no checks or bites".to_owned());
            }
        }
        Kind::Person => {
            if !m.checks.is_empty() || !m.bites.is_empty() {
                errors.push("a person's mark has no checks or bites".to_owned());
            }
        }
        Kind::Outside => {
            // Its bites may stay as data for the change-set that runs it; this judge never evaluates them.
            if !m.parts.is_empty() || !m.checks.is_empty() || m.notes.trim().is_empty() {
                errors.push(
                    "an outside mark has no parts or checks, and says why in notes".to_owned(),
                );
            }
        }
    }
    if m.stand_in && m.id != "PM-0001-4" {
        errors.push("only PM-0001-4 has a stand-in verdict (plan §6.1)".to_owned());
    }
    let mut bite_ids = BTreeSet::new();
    for b in &m.bites {
        if !bite_ids.insert(b.id.as_str()) || !b.id.starts_with(&format!("{}/", m.id)) {
            errors.push(format!(
                "bite {:?} is twice, or does not start with \"{}/\"",
                b.id, m.id
            ));
        }
        // A bite either must FAIL the mark, or (expect "invalid") must make its run INVALID by one named rule.
        match (b.expect.as_deref(), b.rule.as_deref()) {
            (None, None) => {}
            (Some("invalid"), Some(rule)) => {
                let named = m.validity.iter().any(|v| v == rule)
                    || m.parts.iter().any(|p| p.validity.iter().any(|v| v == rule));
                if Rule::parse(rule).is_err() || !named {
                    errors.push(format!(
                        "bite {:?} expects INVALID by {rule:?}, which is not a rule of the mark or its parts",
                        b.id
                    ));
                }
            }
            _ => errors.push(format!(
                "bite {:?}: expect must be \"invalid\", given with its rule",
                b.id
            )),
        }
    }
    if m.validity
        .iter()
        .chain(m.parts.iter().flat_map(|p| &p.validity))
        .any(|v| v == &format!("pre:{}", m.id))
    {
        errors.push("a mark cannot be its own precondition".to_owned());
    }
    if errors.is_empty() {
        Ok(Mark {
            id: m.id.clone(),
            title: m.title.clone(),
            draft: m.draft.clone(),
            kind,
            where_: m.where_.clone(),
            parts,
            validity,
            checks,
            report,
            bites: m.bites.clone(),
            stand_in: m.stand_in,
            arm_rule: m.arm_rule,
            ws_bound_line: m.ws_bound_line.clone(),
            readings: m.readings.clone(),
            notes: m.notes.clone(),
        })
    } else {
        Err(errors)
    }
}

fn has_pre(m: &Mark) -> bool {
    m.validity
        .iter()
        .chain(m.parts.iter().flat_map(|p| &p.validity))
        .any(|r| matches!(r, Rule::Pre(_)))
}

/// Reads `marks.json` text. On any problem, every problem found, each naming its mark.
pub fn parse(text: &str) -> Result<Marks, Vec<String>> {
    let raw: RawFile = serde_json::from_str(text).map_err(|e| vec![format!("marks.json: {e}")])?;
    let mut errors = Vec::new();
    if raw.schema != SCHEMA {
        errors.push(format!(
            "marks.json schema {} (this judge reads {SCHEMA})",
            raw.schema
        ));
    }
    let commit_ok =
        raw.drafts_commit.len() == 40 && raw.drafts_commit.bytes().all(|b| b.is_ascii_hexdigit());
    if !commit_ok {
        errors.push("drafts_commit is not a full 40-digit commit".to_owned());
    }
    let mut ids = BTreeSet::new();
    let mut marks = Vec::new();
    for m in &raw.marks {
        if !ids.insert(m.id.clone()) {
            errors.push(format!("{}: listed twice", m.id));
        }
        match compile_mark(m) {
            Ok(mark) => marks.push(mark),
            Err(es) => errors.extend(es.into_iter().map(|e| format!("{}: {e}", m.id))),
        }
    }
    // A precondition names a mark of the same file that is judged by script, and is not itself conditional.
    for m in &marks {
        for r in m
            .validity
            .iter()
            .chain(m.parts.iter().flat_map(|p| &p.validity))
        {
            if let Rule::Pre(id) = r {
                match marks.iter().find(|x| &x.id == id) {
                    Some(pre) if pre.kind == Kind::Pass && !has_pre(pre) => {}
                    Some(_) => errors.push(format!(
                        "{}: precondition {id} must be a pass mark with no precondition of its own",
                        m.id
                    )),
                    None => {
                        errors.push(format!("{}: precondition {id} is not in marks.json", m.id))
                    }
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(Marks {
            drafts_commit: raw.drafts_commit,
            marks,
        })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(marks: &str) -> String {
        format!(
            r#"{{"schema":1,"drafts_commit":"{}","marks":[{marks}]}}"#,
            "a".repeat(40)
        )
    }

    fn mark(extra: &str) -> String {
        format!(
            r#"{{"id":"PM-0001-4","title":"t","draft":{{"file":"docs/adr/0001-multiplayer-netcode.md","row_sha256":"{}"}},{extra}}}"#,
            "0".repeat(64)
        )
    }

    const PART: &str = r#""parts":[{"name":"S30","select":{"layer":"L2"},"min":3}]"#;

    #[test]
    fn a_whole_pass_mark_compiles() {
        let text = file(&mark(&format!(
            r#""verdict":"pass",{PART},"validity":["V-S"],"stand_in":true,
            "checks":[{{"name":"p99","series":"tick.v.work_ns","stat":"p99","op":"<=","value":16670000}},
                      {{"name":"share","series":"event.shot.hit","stat":"share","where":{{"op":"==","value":1}},"op":">=","value":"0.99"}},
                      {{"name":"vs","series":"tick.v.work_ns","stat":"p99","op":"<=","factor":"1.10","against":{{"roles":["clean"],"combine":"median"}}}}],
            "report":[{{"name":"p50","series":"tick.v.work_ns","stat":"p50"}}],
            "bites":[{{"id":"PM-0001-4/B-S","what":"a busy-wait"}}]"#
        )));
        let marks = parse(&text).expect("compiles");
        let m = &marks.marks[0];
        assert_eq!(m.kind, Kind::Pass);
        assert_eq!(m.checks.len(), 3);
        assert_eq!(m.validity, vec![Rule::VS]);
        assert!(matches!(
            m.checks[1].measure,
            Measure::Series {
                stat: Stat::Share,
                ..
            }
        ));
    }

    #[test]
    fn problems_are_all_reported() {
        let text = file(&mark(&format!(
            r#""verdict":"pass",{PART},"validity":["V-X"],
            "checks":[{{"name":"a","series":"tick.v.Work","stat":"p99","op":"<=","value":1}},
                      {{"name":"b","series":"tick.v.w","stat":"p98","op":"<=","value":1}},
                      {{"name":"c","series":"tick.v.w","stat":"p99","op":"=<","value":1}},
                      {{"name":"d","series":"tick.v.w","stat":"p99","op":"<=","value":0.5}},
                      {{"name":"e","figure":"no_such_figure","op":"<=","value":1}},
                      {{"name":"f","series":"tick.v.w","stat":"p99","op":"<=","value":1,"parts":["nope"]}}]"#
        )));
        let errors = parse(&text).expect_err("must fail");
        assert_eq!(errors.len(), 7, "{errors:#?}");
        assert!(errors.iter().all(|e| e.starts_with("PM-0001-4: ")));
    }

    #[test]
    fn kinds_keep_their_shape() {
        let outside_no_notes = file(&mark(r#""verdict":"outside""#));
        assert!(parse(&outside_no_notes).is_err());
        let outside = file(&mark(r#""verdict":"outside","notes":"the 60 Hz run""#));
        assert!(parse(&outside).is_ok());
        let outside_with_bites = file(&mark(
            r#""verdict":"outside","notes":"the 60 Hz run","bites":[{"id":"PM-0001-4/B-S","what":"kept as data"}]"#,
        ));
        assert!(
            parse(&outside_with_bites).is_ok(),
            "an outside mark may keep its bites as data"
        );
        let outside_with_part = file(&mark(&format!(r#""verdict":"outside","notes":"x",{PART}"#)));
        assert!(parse(&outside_with_part).is_err());
        let report_bad_part = file(&mark(&format!(
            r#""verdict":"reported",{PART},"report":[{{"name":"a","series":"tick.v.w","stat":"max","parts":["nope"]}}]"#
        )));
        assert!(
            parse(&report_bad_part).is_err(),
            "a report names only the mark's parts"
        );
        let report_pooled_figure = file(&mark(&format!(
            r#""verdict":"reported",{PART},"report":[{{"name":"a","figure":"sync_cost_ratio","pooled":true}}]"#
        )));
        assert!(
            parse(&report_pooled_figure).is_err(),
            "a figure cannot be pooled"
        );
        let reported_with_check = file(&mark(&format!(
            r#""verdict":"reported",{PART},"checks":[{{"name":"a","series":"tick.v.w","stat":"max","op":"<=","value":1}}]"#
        )));
        assert!(parse(&reported_with_check).is_err());
        let pass_no_check = file(&mark(&format!(r#""verdict":"pass",{PART}"#)));
        assert!(parse(&pass_no_check).is_err());
        let unknown_field = file(&mark(r#""verdict":"outside","notes":"x","extra":1"#));
        assert!(parse(&unknown_field).is_err());
    }

    /// A WebSocket bound line (README §2.5) that agrees with the X, Y and Z checks below and the tolerance 1.10.
    const LINE: &str = r#"npt-ws-bound v1 {"model":"linux-6.8-tcp v3","commit":"bf9d248000000000000000000000000000000000","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;
    const X: &str = r#"{"name":"x","series":"event.burst_heal.after_link_ns","stat":"p50","op":"<=","value":670000000,"parts":["ws"],"ws_bound":"x"}"#;
    const Y: &str = r#"{"name":"y","series":"event.burst_heal.after_link_ns","stat":"count","where":{"op":">","value":2500000000},"op":"<=","value":5,"parts":["ws"],"ws_bound":"y"}"#;
    const Z: &str = r#"{"name":"z","series":"event.burst_heal.after_link_ns","stat":"max","op":"<=","value":6900000000,"parts":["ws"],"ws_bound":"z"}"#;

    /// A mark with a WebSocket part judged by `rule`, these checks and, if given, this bound line.
    fn ws_mark(rule: &str, line: Option<&str>, checks: &str) -> String {
        let line = line.map_or(String::new(), |l| {
            format!(
                r#","ws_bound_line":{}"#,
                serde_json::to_string(l).expect("a string")
            )
        });
        file(&mark(&format!(
            r#""verdict":"pass","parts":[{{"name":"ws","select":{{"layer":"L2"}},"min":1,"validity":["{rule}"]}},
               {{"name":"wt","select":{{"layer":"L1"}},"min":1}}],"checks":[{checks}]{line}"#
        )))
    }

    /// The bound's line held verbatim in `ws_bound_line` (the advisor's FX-6 on U4j): a mark with `ws_bound` checks
    /// holds it and no other mark does, and the mark is refused unless the line's X, k, Y and Z equal its checks' and
    /// its tolerance the `ws-practice` rule's.
    #[test]
    fn ws_bound_line_agrees_with_its_checks() {
        let all = format!("{X},{Y},{Z}");
        let good = parse(&ws_mark("ws-practice:1.10", Some(LINE), &all)).expect("compiles");
        assert_eq!(good.marks[0].ws_bound_line.as_deref(), Some(LINE));
        let other = |from: &str, to: &str| LINE.replacen(from, to, 1);
        let plain =
            r#"{"name":"p","series":"tick.v.w","stat":"max","op":"<=","value":1,"parts":["wt"]}"#;
        for (text, why) in [
            (
                ws_mark(
                    "ws-practice:1.10",
                    Some(&other(r#""x_ns":670000000"#, r#""x_ns":680000000"#)),
                    &all,
                ),
                "its X is 680000000, check \"x\" holds 670000000",
            ),
            (
                ws_mark(
                    "ws-practice:1.10",
                    Some(&other(r#""tolerance":"1.10""#, r#""tolerance":"1.05""#)),
                    &all,
                ),
                "its tolerance 1.05 is not the ws-practice rule's 1.1",
            ),
            (
                ws_mark("ws-practice:1.10", None, &all),
                "a mark with ws_bound checks holds the bound's line",
            ),
            (
                ws_mark("ws-practice:1.10", Some(LINE), plain),
                "only a mark with ws_bound checks holds a bound line",
            ),
            (
                ws_mark(
                    "ws-practice:1.10",
                    Some(
                        &other(r#""band_low":"0.965""#, r#""band_low":"2.0""#).replacen(
                            r#""band_high":"1.035""#,
                            r#""band_high":"2.0""#,
                            1,
                        ),
                    ),
                    &all,
                ),
                "a band that is not 0 < low <= 1 <= high",
            ),
        ] {
            let errors = parse(&text).expect_err(why);
            assert!(errors.iter().any(|e| e.contains(why)), "{why}: {errors:?}");
        }
    }

    /// PM-0001-18's WebSocket checks (README §2.3 `ws_bound`, §4 `ws-practice`): only on parts the rule judges, one
    /// per bound number, each in the shape its number needs (x a median at most X, y a count over Y at most k, z a
    /// maximum at most Z: SEC-F11); the tolerance between 1 and 1.10.
    #[test]
    fn ws_bound_checks_keep_their_shape() {
        let ws = |rule: &str, checks: &str| ws_mark(rule, Some(LINE), checks);
        let good = parse(&ws("ws-practice:1.10", &format!("{X},{Y},{Z}"))).expect("compiles");
        let m = &good.marks[0];
        assert_eq!(
            m.checks.iter().map(|c| c.ws_bound).collect::<Vec<_>>(),
            [
                Some(BoundNumber::X),
                Some(BoundNumber::Y),
                Some(BoundNumber::Z)
            ]
        );
        assert_eq!(
            m.parts[0].validity,
            [Rule::WsPractice {
                tolerance: Ratio::new(11, 10),
                text: "1.10".to_owned()
            }]
        );
        assert_eq!(m.parts[0].validity[0].name(), "ws-practice:1.10");
        let bad = [
            (ws("ws-practice:1.11", X), "a raised tolerance"),
            (ws("ws-practice:0.99", X), "a tolerance below 1"),
            (ws("ws-practice:ten", X), "a tolerance that is no number"),
            (
                ws("ws-practice:1.10", &X.replace(r#""x"}"#, r#""w"}"#)),
                "no such number",
            ),
            (
                ws("ws-practice:1.10", &X.replace(r#"["ws"]"#, r#"["wt"]"#)),
                "a part the rule does not judge",
            ),
            (
                ws("ws-practice:1.10", &X.replace(r#","parts":["ws"]"#, "")),
                "no parts",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    &format!("{X},{}", X.replace(r#""name":"x""#, r#""name":"x2""#)),
                ),
                "two x checks",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    &Y.replace(r#""where":{"op":">","value":2500000000},"#, ""),
                ),
                "y with no where",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    &X.replace(r#""stat":"p50""#, r#""stat":"count""#),
                ),
                "x on a count",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    &X.replace(r#""ws_bound":"x""#, r#""ws_bound":"x","pooled":true"#),
                ),
                "pooled",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    &X.replace(r#""ws_bound":"x""#, r#""ws_bound":"x","setup":true"#),
                ),
                "a setup check",
            ),
            (
                ws(
                    "ws-practice:1.10",
                    r#"{"name":"f","figure":"machine_named","op":"==","value":1,"parts":["ws"],"ws_bound":"x"}"#,
                ),
                "a figure",
            ),
        ];
        for (text, why) in bad {
            assert!(parse(&text).is_err(), "{why} must be refused");
        }
        // Each letter's shape (SEC-F11): another op or statistic is refused, by its own reason.
        for (check, why) in [
            (X.replace(r#""op":"<=""#, r#""op":">=""#), "x with op >="),
            (
                X.replace(r#""stat":"p50""#, r#""stat":"max""#),
                "x with stat max",
            ),
            (
                Z.replace(r#""stat":"max""#, r#""stat":"p50""#),
                "z with stat p50",
            ),
            (
                Y.replace(r#""where":{"op":">""#, r#""where":{"op":">=""#),
                "y with a where op >=",
            ),
        ] {
            let errors = parse(&ws("ws-practice:1.10", &check)).expect_err(why);
            assert!(
                errors.iter().any(|e| e.contains("its shape is")),
                "{why}: {errors:?}"
            );
        }
        let report = file(&mark(&format!(
            r#""verdict":"reported",{PART},"report":[{{"name":"r","series":"tick.v.w","stat":"max","ws_bound":"x"}}]"#
        )));
        assert!(parse(&report).is_err(), "a report holds no bound number");
    }
}
