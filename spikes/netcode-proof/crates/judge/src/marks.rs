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
}

/// A planted defect that must fail the mark first (plan §6.1, "Bites first").
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bite {
    pub id: String,
    pub what: String,
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
    },
    Figure(String),
}

/// The comparison a check makes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Threshold {
    Value(Ratio),
    Against { factor: Ratio, against: Against },
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
}

/// A validity rule a mark may name (README §4); the three that hold for every run are not listed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    VS,
    Density,
    Lg80,
    Padding,
    LateZero,
    Reorder,
    CleanRef,
}

impl Rule {
    fn parse(s: &str) -> Result<Rule, String> {
        Ok(match s {
            "V-S" => Rule::VS,
            "PM-0006-13" => Rule::Density,
            "lg-80" => Rule::Lg80,
            "padding" => Rule::Padding,
            "late-zero" => Rule::LateZero,
            "reorder" => Rule::Reorder,
            "clean-ref" => Rule::CleanRef,
            _ => return Err(format!("validity rule {s:?} is not one of README §4's")),
        })
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
            if matches!(source, Source::HeaderParam(_)) && c.window.is_some() {
                return Err("a header value has no window".to_owned());
            }
            Ok(Measure::Series {
                source,
                stat,
                window,
                cond,
            })
        }
        (None, Some(figure)) => {
            if !figures::known(figure) {
                return Err(format!("figure {figure:?} is not in the judge's list"));
            }
            if c.stat.is_some() || c.window.is_some() || c.where_.is_some() {
                return Err("a figure takes no stat, window or where".to_owned());
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
    Ok(Check {
        name: c.name.clone(),
        measure,
        op,
        threshold,
        from: c.from.clone(),
        parts: c.parts.clone(),
        pooled: c.pooled,
    })
}

fn compile_report(c: &RawCheck, part_names: &BTreeSet<&str>) -> Result<Report, String> {
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
        parts.push(Part {
            name: p.name.clone(),
            select: p.select.clone(),
            min: p.min,
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
            readings: m.readings.clone(),
            notes: m.notes.clone(),
        })
    } else {
        Err(errors)
    }
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
}
