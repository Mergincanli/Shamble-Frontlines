//! r2, the contract test's checker (u4r-brief.md 2.4 r2 (b), (c) and (e); U4R-Q11 (a)): one run's three logs, read
//! back, against marks/SERIES.md and the judge's validity inputs. The judge's own half, (a) every check of the parts
//! the run's header lists resolving with no gap and (d) PM-0001-2 passing, is `npt-judge --only` on the same folder:
//! npt-lab does not depend on npt-judge (its manifest is not U4r's lane), so the two halves run side by side, by hand
//! and from the run scripts.
//! - **(b) names:** every name the logs write (each header param as `header.params.<name>`, every `tick.v`, `tick.s`
//!   and `hs` name, every `second.v` and `second.s` name, every event field as `event.<event>.<field>`) has its
//!   SERIES.md row; a header param also counts by its bare name in the header params table, and an event's
//!   identifiers (`slot`, `burst`, `seq`: SERIES.md:313) are not measured values.
//! - **(c) validity inputs:** a footer on every log (the `complete` rule: README §4); one `admit` event per client,
//!   each `in_roster` true (the `roster` rule); `tick.v.late_count` on every room tick record. `late-zero` applies to
//!   PM-0001-30 alone, which judges no U4r run, so this asks it of every run (stronger than "where it applies").
//! - **(e) the shot band** (§4.13 :446-447): the measured seconds' mean of the room's `second.v.shots_count`, SS within
//!   [665, 735], ES within [68.4, 75.6].
//! - **The class audit** (the advisor's U4r build ruling 3, point 6 (1)): the lab log's `class_audit` event, its
//!   `breaches_count` printed on every run; a run with no such event, or with a count above 0, fails, unless the run's
//!   bite (its header's) is npt-room's class-filter bite, r4's, which must breach (catalogue item 5: the one id of
//!   `npt_room::BITES` outside the `PM-` form, room/src/lib.rs:32-46, looked up at run time in a bites build and never
//!   spelled here). The mirror's figures (everyone in view, -57's pairing, the audit) hold only on a run it followed.
//! - **Carry 60:** the weapon and health params in every log's header; a missing one is a failure.
//!
//! The other carries' figures the logs give (reported, never a failure): 62 (h) (the measured phase's sum of
//! `tick.s.snap_incomplete`), 58, 74, 77 (c), the rewinds refused, and -18's heal records.
//!
//! Exit codes (`main`): 0 PASS, 1 FAIL (every failure by name; a log that npt-log's reader refuses is one), 2 a usage
//! error, 3 an input that cannot be read (the `--series` file or a log in the `--logs` folder), never reported as the
//! contract's FAIL (point 6 (3)).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use npt_log::{Log, Phase};

/// `contract`'s usage line.
pub(crate) const USAGE: &str = "npt-lab contract --logs <run folder> --series <SERIES.md> (exit 0 PASS, 1 FAIL, \
2 a usage error, 3 an input that cannot be read: the --series file or a log in the --logs folder)";
/// The exit code of an input that cannot be read (the module text).
pub(crate) const UNREADABLE: u8 = 3;

/// The event fields that name an event's subject and are no measured value (SERIES.md:313).
const IDENTIFIERS: [&str; 3] = ["slot", "burst", "seq"];
/// Carry 60's header params: the weapon and the health.
const CARRY_60: [&str; 7] = [
    "weapon_interval_us",
    "weapon_damage_count",
    "weapon_reach_cm",
    "weapon_magazine_count",
    "weapon_reload_ms",
    "weapon_spread_urad",
    "soldier_health_count",
];
/// The clients of a full SS or ES run, the shot band's (PM-0001-2's parts: `"clients": 48`).
const FULL_CLIENTS: u64 = 48;
/// -18's heal bound on L1, ns (marks.json -18: `event.burst_heal.after_link_ns` <= 500,000,000).
const HEAL_BOUND_NS: u64 = 500_000_000;

/// What one run's logs gave: the report's lines, and every failure by name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub(crate) lines: Vec<String>,
    pub(crate) failures: Vec<String>,
}

/// Every name SERIES.md gives a row: each backticked name in a table row's first cell, a `.x` shorthand joined to the
/// cell's first name's record (`event.ws_practice.class`, `.m_p_ns`: SERIES.md:316).
pub(crate) fn rows(series: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in series.lines() {
        let Some(rest) = line.strip_prefix('|') else {
            continue;
        };
        let cell = rest.split('|').next().unwrap_or("");
        let names: Vec<&str> = cell.split('`').skip(1).step_by(2).collect();
        let Some(first) = names.first() else {
            continue;
        };
        let stem = first.rsplit_once('.').map_or("", |(s, _)| s);
        for n in &names {
            if n.starts_with('.') {
                out.insert(format!("{stem}{n}"));
            } else {
                out.insert((*n).to_owned());
            }
        }
    }
    out
}

/// Every name the logs write (the module text's (b)).
pub(crate) fn written(logs: &[Log]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for l in logs {
        for k in l.header.params.keys() {
            out.insert(format!("header.params.{k}"));
        }
        for t in &l.ticks {
            out.extend(t.v.keys().map(|k| format!("tick.v.{k}")));
            out.extend(t.s.keys().map(|k| format!("tick.s.{k}")));
            out.extend(t.hs.keys().map(|k| format!("hs.{k}")));
        }
        for s in &l.seconds {
            out.extend(s.v.keys().map(|k| format!("second.v.{k}")));
            out.extend(s.s.keys().map(|k| format!("second.s.{k}")));
        }
        for e in &l.events {
            for f in e.f.keys().filter(|f| !IDENTIFIERS.contains(&f.as_str())) {
                out.insert(format!("event.{}.{f}", e.ev));
            }
        }
    }
    out
}

fn by_process<'a>(logs: &'a [Log], process: &str) -> Option<&'a Log> {
    logs.iter().find(|l| l.header.process == process)
}

fn field_u64(e: &npt_log::EventRecord, name: &str) -> Option<u64> {
    e.f.get(name).and_then(serde_json::Value::as_u64)
}

/// r2's checker over one run's logs (the module text).
pub(crate) fn check(logs: &[Log], series: &str) -> Outcome {
    let mut o = Outcome::default();
    // (b) every written name has its row.
    let have = rows(series);
    let names = written(logs);
    let missing: Vec<&String> = names
        .iter()
        .filter(|n| {
            !have.contains(n.as_str())
                && !n
                    .strip_prefix("header.params.")
                    .is_some_and(|k| have.contains(k))
        })
        .collect();
    o.lines.push(format!(
        "(b) names: {} written, {} without a SERIES.md row",
        names.len(),
        missing.len()
    ));
    for n in missing {
        o.failures.push(format!("(b) {n} has no SERIES.md row"));
    }
    // (c) the validity inputs.
    for l in logs {
        if l.footer.is_none() {
            o.failures.push(format!(
                "(c) the {} log has no footer (complete)",
                l.header.process
            ));
        }
        for k in CARRY_60 {
            if !l.header.params.contains_key(k) {
                o.failures.push(format!(
                    "carry 60: the {} log's header has no params.{k}",
                    l.header.process
                ));
            }
        }
    }
    for p in ["room", "client", "lab"] {
        if by_process(logs, p).is_none() {
            o.failures.push(format!("(c) no {p} log"));
        }
    }
    audit(logs, &mut o);
    let Some(room) = by_process(logs, "room") else {
        return o;
    };
    let clients = u64::from(room.header.clients);
    let mut admits: BTreeMap<u64, u64> = BTreeMap::new();
    for e in room.events.iter().filter(|e| e.ev == "admit") {
        let slot = field_u64(e, "slot").unwrap_or(u64::MAX);
        *admits.entry(slot).or_default() += 1;
        if e.f.get("in_roster") != Some(&serde_json::Value::Bool(true)) {
            o.failures
                .push(format!("(c) slot {slot}'s admit is not in_roster true"));
        }
    }
    for slot in 0..clients {
        match admits.get(&slot) {
            Some(1) => {}
            n => o.failures.push(format!(
                "(c) slot {slot}: {} admit events, not 1",
                n.copied().unwrap_or(0)
            )),
        }
    }
    let lacking = room
        .ticks
        .iter()
        .filter(|t| !t.v.contains_key("late_count"))
        .count();
    if lacking > 0 || room.ticks.is_empty() {
        o.failures.push(format!(
            "(c) {lacking} of {} room tick records have no tick.v.late_count",
            room.ticks.len()
        ));
    }
    let late: i64 = room
        .ticks
        .iter()
        .filter_map(|t| t.v.get("late_count"))
        .sum();
    o.lines.push(format!(
        "(c) footers on {} logs; admit events {} for {clients} clients; tick.v.late_count on {} room ticks, sum {late}",
        logs.iter().filter(|l| l.footer.is_some()).count(),
        admits.values().sum::<u64>(),
        room.ticks.len() - lacking
    ));
    // (e) the shot band.
    let measured: Vec<i64> = room
        .seconds
        .iter()
        .filter(|s| s.ph == Phase::Measured)
        .filter_map(|s| s.v.get("shots_count").copied())
        .collect();
    let n = measured.len() as i64;
    let sum: i64 = measured.iter().sum();
    let (lo, hi) = match room.header.scene.as_str() {
        "ES" => (684, 756),
        _ => (6_650, 7_350),
    };
    let in_band = n > 0 && sum * 10 >= lo * n && sum * 10 <= hi * n;
    // The band is the scene's 48 soldiers' (PM-0001-2's parts select 48 clients): a smaller test run is not held to it.
    let applies = clients == FULL_CLIENTS;
    o.lines.push(format!(
        "(e) shots per measured second: mean {:.2} over {n} s, band [{}, {}]: {}",
        if n > 0 { sum as f64 / n as f64 } else { 0.0 },
        lo as f64 / 10.0,
        hi as f64 / 10.0,
        match (applies, in_band) {
            (false, _) =>
                format!("not applied ({clients} clients; the band is for {FULL_CLIENTS})"),
            (true, true) => "in".to_owned(),
            (true, false) => "out".to_owned(),
        }
    ));
    if applies && !in_band {
        o.failures
            .push("(e) the shot band: the measured mean is outside it".to_owned());
    }
    carries(logs, &mut o);
    o
}

/// Whether `bite` is npt-room's class-filter bite, r4's (the module text): the one id of npt-room's own list outside the
/// `PM-` form, read from that list at run time. A measured build holds no list, so it knows no bite as that one.
fn room_class_filter_bite(bite: &str) -> bool {
    #[cfg(feature = "bites")]
    let room_s = npt_room::BITES.contains(&bite);
    #[cfg(not(feature = "bites"))]
    let room_s = false;
    room_s && !bite.starts_with("PM-")
}

/// The class audit (the module text): the lab log's `class_audit` event, its `breaches_count` on every run.
fn audit(logs: &[Log], o: &mut Outcome) {
    let Some(lab) = by_process(logs, "lab") else {
        return;
    };
    let Some(e) = lab.events.iter().find(|e| e.ev == "class_audit") else {
        o.failures
            .push("class audit: the lab log has no class_audit event".to_owned());
        return;
    };
    let Some(n) = field_u64(e, "breaches_count") else {
        o.failures
            .push("class audit: the class_audit event has no breaches_count".to_owned());
        return;
    };
    let first =
        e.f.get("first_breach")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
    let bite = lab.header.bite.as_deref();
    let exempt = bite.is_some_and(room_class_filter_bite);
    o.lines.push(format!(
        "class audit: breaches_count {n}{}{}",
        if n > 0 {
            format!(" (the first: {first:?})")
        } else {
            String::new()
        },
        if exempt {
            "; the run's bite is npt-room's class filter (r4's), which must breach: not a failure"
        } else {
            ""
        }
    ));
    if n > 0 && !exempt {
        o.failures.push(format!(
            "class audit: {n} breaches of the room-side mirror (the first: {first:?}), so its figures (everyone in \
             view, -57's pairing, the audit) do not hold{}",
            bite.map_or(String::new(), |b| format!(" (the run's bite {b})"))
        ));
    }
}

/// The carries' figures (reported).
fn carries(logs: &[Log], o: &mut Outcome) {
    if let Some(c) = by_process(logs, "client") {
        let sum: i64 = c
            .ticks
            .iter()
            .filter(|t| t.ph == Phase::Measured)
            .filter_map(|t| t.s.get("snap_incomplete"))
            .flatten()
            .sum();
        o.lines.push(format!(
            "carry 62 (h): the measured phase's sum of tick.s.snap_incomplete {sum}"
        ));
    }
    let Some(lab) = by_process(logs, "lab") else {
        return;
    };
    let keys = [
        "commands_count",
        "late_count",
        "steps_without_command_count",
        "stale_count",
        "rewinds_refused_count",
        "events_queued_count",
        "overflow_resyncs_count",
    ];
    let mut t: BTreeMap<&str, u64> = keys.iter().map(|k| (*k, 0)).collect();
    for e in lab.events.iter().filter(|e| e.ev == "client_counts") {
        for k in keys {
            *t.entry(k).or_default() += field_u64(e, k).unwrap_or(0);
        }
    }
    let lead = lab
        .header
        .params
        .get("lead_ticks_count")
        .cloned()
        .unwrap_or_default();
    o.lines.push(format!(
        "carry 74: lead {lead}; commands {}, late {}, steps without a command {}; carry 77 (c): stale {}; rewinds \
         refused {}; carry 58: events queued {}, overflow resyncs {}",
        t["commands_count"],
        t["late_count"],
        t["steps_without_command_count"],
        t["stale_count"],
        t["rewinds_refused_count"],
        t["events_queued_count"],
        t["overflow_resyncs_count"]
    ));
    let heals: Vec<&npt_log::EventRecord> =
        lab.events.iter().filter(|e| e.ev == "burst_heal").collect();
    if !heals.is_empty() || lab.header.profile == "B1" {
        let within = heals
            .iter()
            .filter(|e| field_u64(e, "after_link_ns").is_some_and(|n| n <= HEAL_BOUND_NS))
            .count();
        let pairs: BTreeSet<(u64, u64)> = heals
            .iter()
            .filter_map(|e| Some((field_u64(e, "slot")?, field_u64(e, "burst")?)))
            .collect();
        let queue_max = heals
            .iter()
            .filter_map(|e| field_u64(e, "queue_max_count"))
            .max()
            .unwrap_or(0);
        let full = heals
            .iter()
            .filter(|e| field_u64(e, "full_state") == Some(1))
            .count();
        let bursts = lab
            .header
            .params
            .get("burst_starts_ns")
            .and_then(serde_json::Value::as_array)
            .map_or(0, Vec::len);
        o.lines.push(format!(
            "-18: {} heal records ({} within 500 ms; {} (client, burst) pairs of {} clients x {bursts} bursts; a full \
             state during {full}); carry 58: queue high-water mark {queue_max}",
            heals.len(),
            within,
            pairs.len(),
            lab.header.clients
        ));
    }
}

/// The run folder's three logs, read back: `Err((UNREADABLE, why))` for a log that cannot be opened (the module text's
/// exit 3), `Err((1, the FAIL lines))` for one that npt-log's reader refuses (a contract FAIL: the log is there and is
/// not a log by README §1's discipline).
pub(crate) fn read(dir: &Path) -> Result<Vec<Log>, (u8, String)> {
    ["room", "client", "lab"]
        .iter()
        .map(|p| {
            let path = dir.join(format!("{p}.jsonl"));
            let f = std::fs::File::open(&path)
                .map_err(|e| (UNREADABLE, format!("cannot read {}: {e}", path.display())))?;
            npt_log::read_log(std::io::BufReader::new(f)).map_err(|e| {
                (
                    1,
                    format!(
                        "FAIL (c) {} does not read back as a log: {e}\ncontract: FAIL (1 failure)",
                        path.display()
                    ),
                )
            })
        })
        .collect()
}

/// The `contract` subcommand: `Ok` with the lines when nothing failed, else `Err((1, lines and failures))`; a usage
/// error is `Err((2, ...))`, an input that cannot be read `Err((UNREADABLE, ...))` (the module text).
pub(crate) fn main(argv: &[String]) -> Result<Vec<String>, (u8, String)> {
    let usage = |m: &str| (2u8, m.to_owned());
    let (mut logs, mut series) = (None, None);
    let mut it = argv.iter();
    while let Some(flag) = it.next() {
        let v = it.next().ok_or(usage("each option needs a value"))?;
        match flag.as_str() {
            "--logs" if logs.is_none() => logs = Some(v),
            "--series" if series.is_none() => series = Some(v),
            _ => return Err(usage(&format!("unexpected {flag:?}"))),
        }
    }
    let (Some(logs), Some(series)) = (logs, series) else {
        return Err(usage("--logs and --series are required"));
    };
    let text = std::fs::read_to_string(series)
        .map_err(|e| (UNREADABLE, format!("cannot read --series {series}: {e}")))?;
    let read = read(Path::new(logs))?;
    let o = check(&read, &text);
    let mut lines = vec![format!("contract of {logs}:")];
    lines.extend(o.lines.iter().cloned());
    if o.failures.is_empty() {
        lines.push("contract: PASS".to_owned());
        Ok(lines)
    } else {
        lines.extend(o.failures.iter().map(|f| format!("FAIL {f}")));
        lines.push(format!("contract: FAIL ({} failures)", o.failures.len()));
        Err((1, lines.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::tests::{Scratch, scratch, settings};

    const SERIES: &str = include_str!("../../../marks/SERIES.md");

    /// One short L1 P0 run's logs (SS, 48 clients so the shot band applies, 6 s of warm-up, 2 s measured, its header
    /// listing -17), with its scratch guard, so a failing caller keeps them (advisor-u4r-r1, FP-6 item 6: T-5).
    fn short_run() -> (Scratch, Vec<Log>) {
        let dir = scratch("r2");
        let s = settings(
            &dir,
            "--seed 2 --profile P0 --warmup-s 6 --measured-s 2 --after-max-s 10 --marks PM-0001-17",
        );
        crate::scenario::run(&s).expect("the run");
        let logs = read(&dir).expect("its logs");
        (dir, logs)
    }

    /// r2's checker passes the clean run, and each of U4R-Q11's four doctored copies fails it by name, and so do two
    /// more: the shot band, and a mirror breach (the advisor's U4r build ruling 3, point 6 (1)).
    #[test]
    fn r2_the_checker_passes_a_clean_run_and_fails_each_doctored_copy_by_name() {
        let (_guard, logs) = short_run();
        let clean = check(&logs, SERIES);
        for l in &clean.lines {
            println!("r2 clean: {l}");
        }
        assert_eq!(clean.failures, Vec::<String>::new(), "the clean run");
        let room = logs
            .iter()
            .position(|l| l.header.process == "room")
            .expect("a room log");
        // (1) one admit event gone.
        let mut d = logs.clone();
        let i = d[room]
            .events
            .iter()
            .position(|e| e.ev == "admit")
            .expect("an admit");
        let slot = field_u64(&d[room].events[i], "slot").expect("its slot");
        d[room].events.remove(i);
        let f = check(&d, SERIES).failures;
        assert!(
            f.contains(&format!("(c) slot {slot}: 0 admit events, not 1")),
            "{f:?}"
        );
        // (2) one footer gone.
        let mut d = logs.clone();
        d[1].footer = None;
        let f = check(&d, SERIES).failures;
        assert!(
            f.contains(&format!(
                "(c) the {} log has no footer (complete)",
                d[1].header.process
            )),
            "{f:?}"
        );
        // (3) one written name's SERIES.md row gone.
        let row = "| `tick.s.snap_incomplete` |";
        assert!(SERIES.contains(row), "the row is there");
        let without: String = SERIES
            .lines()
            .filter(|l| !l.starts_with(row))
            .collect::<Vec<_>>()
            .join("\n");
        let f = check(&logs, &without).failures;
        assert_eq!(
            f,
            ["(b) tick.s.snap_incomplete has no SERIES.md row".to_owned()]
        );
        // (4) one late_count series gone (a room tick record without it).
        let mut d = logs.clone();
        d[room].ticks[3].v.remove("late_count");
        let f = check(&d, SERIES).failures;
        assert!(
            f.contains(&format!(
                "(c) 1 of {} room tick records have no tick.v.late_count",
                d[room].ticks.len()
            )),
            "{f:?}"
        );
        // (5) beyond U4R-Q11's four: the shot band, with every measured second's shots halved.
        let mut d = logs.clone();
        for s in d[room]
            .seconds
            .iter_mut()
            .filter(|s| s.ph == Phase::Measured)
        {
            if let Some(v) = s.v.get_mut("shots_count") {
                *v /= 2;
            }
        }
        let f = check(&d, SERIES).failures;
        assert_eq!(
            f,
            ["(e) the shot band: the measured mean is outside it".to_owned()]
        );
        // (6) the advisor's U4r build ruling 3, point 6 (1): one breach of the mirror added to the lab log's audit.
        let lab = logs
            .iter()
            .position(|l| l.header.process == "lab")
            .expect("a lab log");
        let mut d = logs.clone();
        let a = d[lab]
            .events
            .iter_mut()
            .find(|e| e.ev == "class_audit")
            .expect("the class audit");
        a.f.insert("breaches_count".to_owned(), serde_json::json!(1));
        a.f.insert(
            "first_breach".to_owned(),
            serde_json::json!("tick 2: client 0: planted"),
        );
        let f = check(&d, SERIES).failures;
        assert_eq!(
            f,
            ["class audit: 1 breaches of the room-side mirror (the first: \"tick 2: client 0: planted\"), so its \
              figures (everyone in view, -57's pairing, the audit) do not hold"
                .to_owned()]
        );
        // Its one exception, r4's bite (bites builds only: npt-room's list is read at run time); another bite's
        // breaches still fail.
        #[cfg(feature = "bites")]
        {
            let mut x = d.clone();
            x[lab].header.bite = Some("catalogue-5/owner-to-all".to_owned());
            assert_eq!(
                check(&x, SERIES).failures,
                Vec::<String>::new(),
                "r4's bite"
            );
            x[lab].header.bite = Some("PM-0001-18/no-fallback".to_owned());
            assert_eq!(check(&x, SERIES).failures.len(), 1, "another bite");
            // The advisor's U4r build ruling 4, point 2: exactly one id of npt-room's list lies outside the `PM-` form,
            // so a second such id cannot widen the exemption unseen.
            let outside: Vec<&str> = npt_room::BITES
                .iter()
                .copied()
                .filter(|b| !b.starts_with("PM-"))
                .collect();
            assert_eq!(
                outside,
                ["catalogue-5/owner-to-all"],
                "npt-room's ids outside the PM- form"
            );
        }
        println!("r2: the six doctored copies each fail by name");
    }

    /// The advisor's U4r build ruling 3, point 6 (3): a `--series` file or a log that cannot be read is exit 3 with its
    /// own message, never the contract's FAIL (exit 1).
    #[test]
    fn r2_an_input_that_cannot_be_read_is_exit_3_never_the_contract_s_fail() {
        let dir = scratch("r2-unreadable");
        std::fs::create_dir_all(&*dir).expect("the folder");
        let d = dir.to_str().expect("a UTF-8 path");
        let argv = |logs: &str, series: &str| -> Vec<String> {
            ["--logs", logs, "--series", series]
                .iter()
                .map(|s| (*s).to_owned())
                .collect()
        };
        let series = format!("{}/../../marks/SERIES.md", env!("CARGO_MANIFEST_DIR"));
        let e = main(&argv(d, &series)).expect_err("an empty folder: no log");
        assert_eq!(e.0, UNREADABLE, "{}", e.1);
        assert!(
            e.1.starts_with("cannot read ") && !e.1.contains("FAIL"),
            "{}",
            e.1
        );
        let e = main(&argv(d, &format!("{d}/no-such-SERIES.md"))).expect_err("no SERIES file");
        assert_eq!(e.0, UNREADABLE, "{}", e.1);
        assert!(e.1.starts_with("cannot read --series "), "{}", e.1);
        assert!(USAGE.contains("3 an input that cannot be read"));
    }

    /// The row reader takes every backticked name of a first cell and joins `.x` shorthands to its record.
    #[test]
    fn r2_rows_reads_each_first_cell_s_names() {
        let r = rows(
            "| `a.b.c`, `.d_ns` | x | y |\n| `soak` | S30 | z |\nnot a row `e.f`\n| Name | Unit |\n",
        );
        let want: BTreeSet<String> = ["a.b.c", "a.b.d_ns", "soak"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        assert_eq!(r, want);
    }
}
