//! The judge's engine on logs built here, one test per verdict path of `marks/README.md` §3 to §6. The committed
//! fixtures (tests/fixtures.rs) prove the same paths on files and on the real marks.json.

use npt_judge::eval::{BiteStatus, RunOut};
use npt_judge::{VerdictKind, judge, parse_marks};
use npt_log::Phase::{After, Measured, Warmup};
use npt_log::{
    Build, Ended, EventRecord, Header, Log, Machine, Phase, SCHEMA, SecondRecord, TickRecord,
    Writer, read_log,
};
use serde_json::{Value, json};

/// One second record: its phase, its values and its lists.
type SecondSpec = (
    Phase,
    Vec<(&'static str, i64)>,
    Vec<(&'static str, Vec<i64>)>,
);

struct LogSpec {
    run: &'static str,
    set: &'static str,
    role: &'static str,
    marks: Vec<&'static str>,
    machine_role: &'static str,
    process: &'static str,
    bite: Option<&'static str>,
    layer: &'static str,
    profile: &'static str,
    seed: u64,
    params: Vec<(&'static str, Value)>,
    ticks: Vec<(Phase, Vec<(&'static str, i64)>)>,
    seconds: Vec<SecondSpec>,
    events: Vec<EventRecord>,
    footer: bool,
    /// The build's features; `None` follows the bite (C4): the bites build exactly when the log names a bite.
    features: Option<Vec<&'static str>>,
    arch: &'static str,
    binary: &'static str,
    /// The header's `clients` (every `hs` list holds this many entries: README §1).
    clients: u32,
}

impl LogSpec {
    fn room(run: &'static str, set: &'static str, mark: &'static str) -> Self {
        LogSpec {
            run,
            set,
            role: "measured",
            marks: vec![mark],
            machine_role: "RS",
            process: "room",
            bite: None,
            layer: "L2",
            profile: "P2",
            seed: 1,
            params: vec![("sim", json!("stress"))],
            ticks: vec![],
            seconds: vec![],
            events: vec![],
            footer: true,
            features: None,
            arch: "x86_64",
            binary: "0000000000000000000000000000000000000000000000000000000000000000",
            clients: 48,
        }
    }

    fn ticks(mut self, name: &'static str, values: &[i64]) -> Self {
        self.ticks = values
            .iter()
            .map(|v| (Phase::Measured, vec![(name, *v)]))
            .collect();
        self
    }

    fn build(self) -> Log {
        let header = Header {
            schema: SCHEMA,
            run: self.run.to_owned(),
            set: self.set.to_owned(),
            role: self.role.to_owned(),
            marks: self.marks.iter().map(|m| (*m).to_owned()).collect(),
            machine: Machine {
                role: self.machine_role.to_owned(),
                name: format!("{}1", self.machine_role),
                plan: "test".to_owned(),
                arch: self.arch.to_owned(),
                cpu: "test".to_owned(),
                vcpus: 4,
                mem_mb: 16_384,
                location: "fsn1".to_owned(),
                kernel: "6".to_owned(),
                address: String::new(),
                threads_per_core: 1,
                smt: vec![vec![0], vec![1], vec![2], vec![3]],
                clock_res_ns: 1,
            },
            build: Build {
                commit: "0".repeat(40),
                binary_sha256: self.binary.to_owned(),
                features: match &self.features {
                    Some(f) => f.iter().map(|s| (*s).to_owned()).collect(),
                    None if self.bite.is_some() => vec!["bites".to_owned()],
                    None => vec![],
                },
                profile: "release".to_owned(),
            },
            bite: self.bite.map(str::to_owned),
            layer: self.layer.to_owned(),
            profile: self.profile.to_owned(),
            seed: self.seed,
            tick_hz: 30,
            clients: self.clients,
            scene: "SS".to_owned(),
            process: self.process.to_owned(),
            slot: 0,
            date: "2026-11-06".to_owned(),
            params: self
                .params
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v))
                .collect(),
        };
        let mut w = Writer::new(Vec::new(), &header).expect("header");
        for (i, (ph, vals)) in self.ticks.into_iter().enumerate() {
            let mut t = TickRecord::new(i as u64 + 1, ph);
            for (k, v) in vals {
                t.v.insert(k.to_owned(), v);
            }
            w.tick(&t).expect("tick");
        }
        for (i, (ph, vals, lists)) in self.seconds.into_iter().enumerate() {
            let mut s = SecondRecord::new(i as u64, ph);
            for (k, v) in vals {
                s.v.insert(k.to_owned(), v);
            }
            for (k, v) in lists {
                s.s.insert(k.to_owned(), v);
            }
            w.second(&s).expect("second");
        }
        for e in &self.events {
            w.event(e).expect("event");
        }
        let bytes = if self.footer {
            w.finish(Ended::Clean).expect("footer")
        } else {
            // A cut log: everything but the footer.
            let mut text =
                String::from_utf8(w.finish(Ended::Clean).expect("footer")).expect("utf-8");
            text.truncate(text.trim_end().rfind('\n').expect("more than one line") + 1);
            text.into_bytes()
        };
        read_log(bytes.as_slice()).expect("reads back")
    }
}

fn marks_file(marks: &[Value]) -> npt_judge::Marks {
    let text = json!({"schema": 1, "drafts_commit": "a".repeat(40), "marks": marks}).to_string();
    parse_marks(&text).expect("the test's marks compile")
}

fn draft() -> Value {
    json!({"file": "docs/adr/0001-multiplayer-netcode.md", "row_sha256": "0".repeat(64)})
}

/// A tick-budget mark like PM-0001-4: p99 of work_ns at most 100, three runs, one bite.
fn tick_mark(id: &str, extra: Value) -> Value {
    let mut m = json!({
        "id": id, "title": "tick budget", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "S30", "select": {"layer": "L2"}, "min": 3}],
        "checks": [{"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100}],
        "bites": [{"id": format!("{id}/B-S"), "what": "a busy-wait"}]
    });
    if let (Some(m), Some(e)) = (m.as_object_mut(), extra.as_object()) {
        for (k, v) in e {
            m.insert(k.clone(), v.clone());
        }
    }
    m
}

fn bite_run(mark: &'static str, bite: &'static str, set: &'static str, values: &[i64]) -> Log {
    let mut s = LogSpec::room("bite-1", set, mark).ticks("work_ns", values);
    s.role = "bite";
    s.bite = Some(bite);
    s.build()
}

fn three_runs(mark: &'static str, worst: [i64; 3]) -> Vec<Log> {
    let ids = ["run-1", "run-2", "run-3"];
    ids.iter()
        .zip(worst)
        .map(|(id, w)| {
            LogSpec::room(id, "set-a", mark)
                .ticks("work_ns", &[10, 20, w])
                .build()
        })
        .collect()
}

fn verdict_of(marks: &[Value], logs: &[Log]) -> (VerdictKind, Vec<String>) {
    let v = judge(&marks_file(marks), logs);
    (v[0].verdict, v[0].why.clone())
}

const M: &str = "PM-0001-4";

#[test]
fn three_passing_runs_and_a_shown_bite_pass() {
    let mut logs = three_runs(M, [90, 95, 100]);
    logs.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[10, 150]));
    let (k, why) = verdict_of(&[tick_mark(M, json!({}))], &logs);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
}

#[test]
fn one_run_over_the_mark_fails() {
    let mut logs = three_runs(M, [90, 101, 100]);
    logs.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[150]));
    let (k, why) = verdict_of(&[tick_mark(M, json!({}))], &logs);
    assert_eq!(k, VerdictKind::Fail);
    assert!(
        why[0].contains("run run-2: tick p99 = 101, needs <= 100"),
        "{why:?}"
    );
}

#[test]
fn a_bite_that_passes_fails_the_mark_and_a_missing_bite_is_not_run() {
    let mut logs = three_runs(M, [90, 95, 100]);
    logs.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[10, 20]));
    let (k, why) = verdict_of(&[tick_mark(M, json!({}))], &logs);
    assert_eq!(k, VerdictKind::Fail);
    assert!(why.iter().any(|w| w.contains("did not fail")), "{why:?}");
    let (k, why) = verdict_of(&[tick_mark(M, json!({}))], &three_runs(M, [90, 95, 100]));
    assert_eq!(k, VerdictKind::NotRun);
    assert!(
        why.iter()
            .any(|w| w.contains("bite PM-0001-4/B-S: no run yet")),
        "{why:?}"
    );
}

#[test]
fn too_few_runs_are_not_run() {
    let logs: Vec<Log> = three_runs(M, [90, 95, 100]).into_iter().take(2).collect();
    let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::NotRun);
    assert!(why[0].contains("2 of 3 valid runs"), "{why:?}");
}

#[test]
fn a_session_outside_the_roster_makes_the_run_invalid() {
    let mut logs = three_runs(M, [90, 95, 100]);
    let mut s = LogSpec::room("run-4", "set-a", M).ticks("work_ns", &[10]);
    s.events = vec![
        EventRecord::new("admit", Phase::Measured)
            .with("session", 9)
            .with("in_roster", false),
    ];
    logs.push(s.build());
    let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::Invalid);
    assert!(why[0].contains("roster: session 9"), "{why:?}");
}

#[test]
fn a_control_without_its_capability_makes_the_run_invalid() {
    let cases = [
        (
            json!({"source": "session", "applied": true, "cap_valid": false, "rostered": true}),
            "applied without",
        ),
        (
            json!({"source": "session", "applied": false, "cap_valid": false, "rostered": true}),
            "a bug in our code",
        ),
        (
            json!({"source": "socket", "applied": true, "peer_ok": false}),
            "unchecked peer",
        ),
    ];
    for (fields, expect) in cases {
        let mut logs = three_runs(M, [90, 95, 100]);
        let mut e = EventRecord::new("control", Phase::Measured);
        for (k, v) in fields.as_object().expect("object") {
            e.f.insert(k.clone(), v.clone());
        }
        logs[0] = {
            let mut s = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10]);
            s.events = vec![e];
            s.build()
        };
        let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
        assert_eq!(k, VerdictKind::Invalid, "{expect}");
        assert!(why[0].contains(expect), "{why:?}");
    }
    // An outsider's refused control before Join is counted, never INVALID: it is not rostered and not applied.
    let mut logs = three_runs(M, [90, 95, 100]);
    logs[0] = {
        let mut s = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10]);
        s.events = vec![
            EventRecord::new("control", Phase::Measured)
                .with("source", "session")
                .with("applied", false)
                .with("rostered", false)
                .with("cap_valid", false),
        ];
        s.build()
    };
    let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
}

#[test]
fn a_cut_log_is_invalid_unless_its_room_was_killed() {
    let mut logs = three_runs(M, [90, 95, 100]);
    let mut cut = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10, 20]);
    cut.footer = false;
    logs[0] = cut.build();
    let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::Invalid);
    assert!(why[0].contains("no footer"), "{why:?}");
    let mut killed = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10, 20]);
    killed.footer = false;
    killed.params.push(("killed", json!(true)));
    logs[0] = killed.build();
    let (k, _) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::Pass);
}

#[test]
fn an_empty_series_is_a_gap_never_a_pass() {
    let mut logs = three_runs(M, [90, 95, 100]);
    logs[2] = LogSpec::room("run-3", "set-a", M)
        .ticks("other_ns", &[1])
        .build();
    let (k, why) = verdict_of(&[tick_mark(M, json!({"bites": []}))], &logs);
    assert_eq!(k, VerdictKind::Invalid);
    assert!(why[0].contains("no values for tick.v.work_ns"), "{why:?}");
}

fn vs_run(id: &'static str, steal: i64, lg_cpu: i64, inputs: i64, alone: bool) -> Vec<Log> {
    let mut room = LogSpec::room(id, "set-v", M).ticks("work_ns", &[10, 20, 30]);
    room.seconds = vec![
        (Phase::Measured, vec![("steal_permille", steal)], vec![]),
        (Phase::Measured, vec![("steal_permille", steal)], vec![]),
    ];
    room.events = ["start", "end"]
        .iter()
        .map(|w| {
            EventRecord::new("affinity", Phase::Measured)
                .with("when", *w)
                .with("cpu", 3)
                .with("alone", alone)
                .with("sibling_free", true)
        })
        .collect();
    let mut lg = LogSpec::room(id, "set-v", M);
    lg.machine_role = "LG";
    lg.process = "bots";
    lg.seconds = vec![
        (
            Phase::Measured,
            vec![("cpu_permille", lg_cpu)],
            vec![("inputs_count", vec![30, inputs])],
        ),
        (
            Phase::Measured,
            vec![("cpu_permille", 500)],
            vec![("inputs_count", vec![30, inputs])],
        ),
    ];
    vec![room.build(), lg.build()]
}

#[test]
fn v_s_holds_or_the_run_is_invalid() {
    let mark = tick_mark(M, json!({"bites": [], "validity": ["V-S"]}));
    let good: Vec<Log> = ["a", "b", "c"]
        .iter()
        .flat_map(|id| vs_run(id, 5, 700, 30, true))
        .collect();
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &good);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
    let cases: [(Vec<Log>, &str); 4] = [
        (vs_run("a", 11, 700, 30, true), "steal averaged 11"),
        (vs_run("a", 5, 800, 30, true), "800 permille CPU"),
        (vs_run("a", 5, 700, 29, true), "client 1"),
        (vs_run("a", 5, 700, 30, false), "not alone"),
    ];
    for (bad, expect) in cases {
        let mut logs = bad;
        logs.extend(
            ["b", "c"]
                .iter()
                .flat_map(|id| vs_run(id, 5, 700, 30, true)),
        );
        let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
        assert_eq!(k, VerdictKind::Invalid, "{expect}");
        assert!(why.iter().any(|w| w.contains(expect)), "{expect}: {why:?}");
    }
}

#[test]
fn a_stand_in_sim_labels_the_pass() {
    let mut logs = three_runs(M, [90, 95, 100]);
    let mut s = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10]);
    s.params = vec![("sim", json!("stand-in"))];
    logs[0] = s.build();
    let (k, _) = verdict_of(
        &[tick_mark(M, json!({"bites": [], "stand_in": true}))],
        &logs,
    );
    assert_eq!(k, VerdictKind::StandIn);
}

fn density_mark() -> Value {
    json!({
        "id": "PM-0006-14", "title": "rooms per VM", "verdict": "reported",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "N48", "select": {"clients": 48}, "min": 1}],
        "validity": ["padding"],
        "report": [{"name": "padded step p99", "series": "tick.v.padded_step_ns", "stat": "p99"}]
    })
}

fn density_run(short_tick: bool) -> Log {
    let mut s = LogSpec::room("d-1", "set-d", "PM-0006-14");
    s.params.push(("padding_target_ns", json!(8_330_000)));
    s.ticks = (0..5)
        .map(|i| {
            let padded = if short_tick && i == 3 {
                8_329_999
            } else {
                8_330_000 + i
            };
            (
                Phase::Measured,
                vec![("padded_step_ns", padded), ("work_ns", 9_000_000)],
            )
        })
        .collect();
    s.build()
}

#[test]
fn short_padding_makes_a_density_run_invalid() {
    let (k, why) = verdict_of(&[density_mark()], &[density_run(true)]);
    assert_eq!(k, VerdictKind::Invalid);
    assert!(
        why[0].contains("1 measured ticks fell short of 8330000 ns (first: tick 4 at 8329999 ns)"),
        "{why:?}"
    );
    let v = judge(&marks_file(&[density_mark()]), &[density_run(false)]);
    assert_eq!(v[0].verdict, VerdictKind::Reported, "{:?}", v[0].why);
    assert_eq!(v[0].report[0].value, "8330004");
}

#[test]
fn reports_keep_to_their_parts_and_pool_when_asked() {
    let mark = json!({
        "id": "PM-0006-19", "title": "logging cost", "verdict": "reported",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "on", "select": {"params.logging": "on"}, "min": 2},
                  {"name": "off", "select": {"params.logging": "off"}, "min": 1}],
        "report": [{"name": "p99 per run", "series": "tick.v.work_ns", "stat": "p99", "parts": ["on"]},
                   {"name": "pooled p99", "series": "tick.v.work_ns", "stat": "p99", "parts": ["on"], "pooled": true},
                   {"name": "max everywhere", "series": "tick.v.work_ns", "stat": "max"}]
    });
    let run = |id: &'static str, logging: &'static str, v: &[i64]| {
        let mut s = LogSpec::room(id, "set-r", "PM-0006-19").ticks("work_ns", v);
        s.params.push(("logging", json!(logging)));
        s.build()
    };
    let logs = vec![
        run("on-1", "on", &[100, 300]),
        run("on-2", "on", &[200, 200]),
        run("off-1", "off", &[50]),
    ];
    let v = judge(&marks_file(&[mark]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Reported, "{:?}", v[0].why);
    let got: Vec<(String, String, String)> = v[0]
        .report
        .iter()
        .map(|r| (r.name.clone(), r.run.clone(), r.value.clone()))
        .collect();
    let want = [
        ("p99 per run", "on-1", "300"),
        ("p99 per run", "on-2", "200"),
        ("pooled p99", "pooled over 2 runs", "300"),
        ("max everywhere", "on-1", "300"),
        ("max everywhere", "on-2", "200"),
        ("max everywhere", "off-1", "50"),
    ];
    let want: Vec<(String, String, String)> = want
        .iter()
        .map(|(a, b, c)| ((*a).to_owned(), (*b).to_owned(), (*c).to_owned()))
        .collect();
    assert_eq!(got, want);
}

#[test]
fn a_clean_reference_compares_within_its_factor() {
    let mark = json!({
        "id": "PM-0001-24", "title": "bottleneck", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "P2", "select": {"layer": "L2"}, "min": 1}],
        "validity": ["clean-ref"],
        "checks": [{"name": "within 5 % of clean", "series": "tick.v.work_ns", "stat": "p99", "op": "<=",
                    "factor": "1.05", "against": {"roles": ["clean"], "combine": "median"}}]
    });
    let clean = |id: &'static str, v: i64| {
        let mut s = LogSpec::room(id, "set-c", "PM-0001-24").ticks("work_ns", &[v]);
        s.role = "clean";
        s.build()
    };
    let measured = |v: i64| {
        LogSpec::room("m-1", "set-c", "PM-0001-24")
            .ticks("work_ns", &[v])
            .build()
    };
    let base = vec![clean("c-1", 100), clean("c-2", 101), clean("c-3", 102)];
    let mut logs = base.clone();
    logs.push(measured(106));
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &logs).0,
        VerdictKind::Pass,
        "106 <= 1.05 x 101"
    );
    let mut logs = base.clone();
    logs.push(measured(107));
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &logs).0,
        VerdictKind::Fail,
        "107 > 106.05"
    );
    let mut logs = vec![clean("c-1", 100), clean("c-2", 101), clean("c-3", 110)];
    logs.push(measured(100));
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(
        k,
        VerdictKind::Invalid,
        "the clean runs spread more than 2.5 %"
    );
    assert!(why[0].contains("clean-ref"), "{why:?}");
    let logs = vec![measured(100)];
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(
        k,
        VerdictKind::Invalid,
        "no clean runs at all: clean-ref fails first"
    );
    assert!(why[0].contains("0 clean runs"), "{why:?}");
}

#[test]
fn a_pooled_check_reads_every_run_at_once() {
    let mark = json!({
        "id": "PM-0006-19", "title": "logging cost", "verdict": "pass",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "on", "select": {"params.logging": "on"}, "min": 2}],
        "checks": [{"name": "pooled p99 vs off", "series": "tick.v.work_ns", "stat": "p99", "op": "<=",
                    "factor": "1.02", "against": {"roles": ["logging-off"], "combine": "pooled"}, "pooled": true}]
    });
    let run = |id: &'static str, role: &'static str, logging: &'static str, v: &[i64]| {
        let mut s = LogSpec::room(id, "set-l", "PM-0006-19").ticks("work_ns", v);
        s.role = role;
        s.params.push(("logging", json!(logging)));
        s.build()
    };
    let logs = vec![
        run("off-1", "logging-off", "off", &[100, 100]),
        run("off-2", "logging-off", "off", &[100, 100]),
        run("on-1", "measured", "on", &[100, 102]),
        run("on-2", "measured", "on", &[100, 100]),
    ];
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &logs).0,
        VerdictKind::Pass,
        "102 <= 1.02 x 100"
    );
    let mut logs = logs;
    logs[3] = run("on-2", "measured", "on", &[103, 103]);
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(k, VerdictKind::Fail, "103 > 102");
    assert!(why[0].contains("pooled"), "{why:?}");
}

#[test]
fn person_and_outside_marks() {
    let person = json!({"id": "PM-0001-53", "title": "looks correct", "draft": draft(), "verdict": "person",
                        "parts": [{"name": "MP", "select": {"machine.role": "MP"}, "min": 1}]});
    let outside = json!({"id": "PM-0001-39", "title": "60 Hz", "draft": draft(), "verdict": "outside",
                         "notes": "the 60 Hz run's own change-set"});
    let (k, _) = verdict_of(std::slice::from_ref(&person), &[]);
    assert_eq!(k, VerdictKind::NotRun);
    let checklist = |verdict: &str| {
        let mut s = LogSpec::room("mp-1", "set-mp", "PM-0001-53");
        s.machine_role = "MP";
        s.process = "browser";
        s.events = vec![
            EventRecord::new("person", Phase::Measured)
                .with("who", "Mustafa")
                .with("mark", "PM-0001-53")
                .with("verdict", verdict),
        ];
        s.build()
    };
    assert_eq!(
        verdict_of(std::slice::from_ref(&person), &[checklist("pass")]).0,
        VerdictKind::Pass
    );
    assert_eq!(
        verdict_of(std::slice::from_ref(&person), &[checklist("fail")]).0,
        VerdictKind::Fail
    );
    assert_eq!(
        verdict_of(&[person], &[checklist("maybe")]).0,
        VerdictKind::Invalid
    );
    // Outside this test from the start: it left before any trim, and reads MOVED (README §2.6), never NOT-RUN.
    let (k, why) = verdict_of(&[outside], &[]);
    assert_eq!(k, VerdictKind::Moved);
    assert!(why[0].starts_with("outside this change-set"));
}

#[test]
fn a_share_with_its_condition() {
    let mark = json!({
        "id": "PM-0001-48", "title": "hits register", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "P2", "select": {"profile": "P2"}, "min": 1}],
        "checks": [{"name": "registered", "series": "event.shot.view_hit", "stat": "share",
                    "where": {"op": "==", "value": 1}, "op": ">=", "value": "0.99"}]
    });
    let shots = |hits: usize, misses: usize| {
        let mut s = LogSpec::room("s-1", "set-s", "PM-0001-48");
        s.events = (0..hits)
            .map(|_| EventRecord::new("shot", Phase::Measured).with("view_hit", true))
            .chain(
                (0..misses)
                    .map(|_| EventRecord::new("shot", Phase::Measured).with("view_hit", false)),
            )
            .chain([EventRecord::new("shot", Phase::Warmup).with("view_hit", false)])
            .collect();
        s.build()
    };
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &[shots(99, 1)]).0,
        VerdictKind::Pass,
        "99 of 100; the warm-up shot is outside the window"
    );
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &[shots(98, 2)]).0,
        VerdictKind::Fail
    );
}

// ---- The advisor's U1 rulings (advisor-u1.md): C2, B7, B8, B9, B13, Q10, Q22, Q27 ----

/// A mark with two checks: the tick budget, and a count of forged commands that must be 0.
fn two_check_mark(extra: Value) -> Value {
    let mut m = tick_mark(M, json!({"bites": []}));
    m["checks"].as_array_mut().expect("checks").push(
        json!({"name": "no forged command applied", "series": "event.forged_applied.cmd",
                     "stat": "count", "op": "==", "value": 0}),
    );
    m["checks"]
        .as_array_mut()
        .expect("checks")
        .push(json!({"name": "queue depth p99", "series": "tick.v.queue_count", "stat": "p99", "op": "<=", "value": 8}));
    if let (Some(m), Some(e)) = (m.as_object_mut(), extra.as_object()) {
        for (k, v) in e {
            m.insert(k.clone(), v.clone());
        }
    }
    m
}

fn run_with(id: &'static str, work: &[i64], queue: Option<i64>) -> Log {
    let mut s = LogSpec::room(id, "set-a", M);
    s.ticks = work
        .iter()
        .map(|w| {
            let mut v = vec![("work_ns", *w)];
            if let Some(q) = queue {
                v.push(("queue_count", q));
            }
            (Phase::Measured, v)
        })
        .collect();
    s.build()
}

#[test]
fn c2_a_failing_check_wins_over_a_gap_and_a_gap_alone_is_invalid() {
    let mark = two_check_mark(json!({}));
    // An event that never happened counts 0, a real value: three clean runs pass.
    let clean: Vec<Log> = ["r1", "r2", "r3"]
        .iter()
        .map(|id| run_with(id, &[50], Some(3)))
        .collect();
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &clean);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
    // Run 2 fails the tick budget and has no queue counter: FAIL, the gap listed beside it.
    let mut logs = clean.clone();
    logs[1] = run_with("r2", &[150], None);
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(
        why.iter().any(|w| w.contains("run r2: tick p99 = 150")),
        "{why:?}"
    );
    assert!(
        why.iter()
            .any(|w| w.contains("run r2: also a gap in the record")
                && w.contains("tick.v.queue_count")),
        "{why:?}"
    );
    // A gap with no failure: INVALID.
    let mut logs = clean;
    logs[1] = run_with("r2", &[50], None);
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        why[0].contains("no values for tick.v.queue_count"),
        "{why:?}"
    );
}

#[test]
fn c2_a_bite_that_fails_one_check_and_empties_another_is_shown() {
    let mark = two_check_mark(json!({"bites": [{"id": "PM-0001-4/B-S", "what": "a busy-wait"}]}));
    let mut logs: Vec<Log> = ["r1", "r2", "r3"]
        .iter()
        .map(|id| run_with(id, &[50], Some(3)))
        .collect();
    let mut bite = LogSpec::room("bite-1", "set-a", M);
    bite.role = "bite";
    bite.bite = Some("PM-0001-4/B-S");
    bite.ticks = vec![(Phase::Measured, vec![("work_ns", 150)])];
    logs.push(bite.build());
    let v = judge(&marks_file(&[mark]), &logs);
    assert_eq!(v[0].bites[0].status, BiteStatus::Shown, "{:?}", v[0].bites);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
}

#[test]
fn c2_an_empty_tick_series_is_a_gap_even_for_a_count() {
    let mark = json!({
        "id": M, "title": "t", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "S30", "select": {"layer": "L2"}, "min": 1}],
        "checks": [{"name": "0 oversize", "series": "tick.s.snapshot_bytes", "stat": "count",
                    "where": {"op": ">", "value": 1067}, "op": "==", "value": 0}]
    });
    let (k, why) = verdict_of(&[mark], &[run_with("r1", &[50], None)]);
    assert_eq!(
        k,
        VerdictKind::Invalid,
        "no snapshot sizes written is a gap, never a vacuous 0: {why:?}"
    );
}

#[test]
fn b7_a_part_adds_its_own_rules_and_a_bite_run_takes_its_parts_rules() {
    let mark = json!({
        "id": "PM-0001-25", "title": "hostile input", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "flood", "select": {"params.suite": "flood"}, "min": 1, "validity": ["late-zero"]},
                  {"name": "origin", "select": {"params.suite": "origin"}, "min": 1}],
        "checks": [{"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100}],
        "bites": [{"id": "PM-0001-25/bite", "what": "x"}]
    });
    let run = |id: &'static str,
               suite: &'static str,
               role: &'static str,
               late: Option<i64>,
               work: i64| {
        let mut s = LogSpec::room(id, "set-h", "PM-0001-25");
        s.role = role;
        if role == "bite" {
            s.bite = Some("PM-0001-25/bite");
        }
        s.params.push(("suite", json!(suite)));
        let mut v = vec![("work_ns", work)];
        if let Some(l) = late {
            v.push(("late_count", l));
        }
        s.ticks = vec![(Phase::Measured, v)];
        s.build()
    };
    // The origin part has no late counter and needs none; the flood part's rule holds.
    let logs = vec![
        run("f-1", "flood", "measured", Some(0), 50),
        run("o-1", "origin", "measured", None, 50),
        run("b-1", "flood", "bite", Some(0), 150),
    ];
    let v = judge(&marks_file(std::slice::from_ref(&mark)), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
    // A bite run in the flood part is held to that part's rule too: a late command makes it INVALID.
    let mut logs = logs;
    logs[2] = run("b-1", "flood", "bite", Some(2), 150);
    let v = judge(&marks_file(&[mark]), &logs);
    assert_eq!(
        v[0].bites[0].status,
        BiteStatus::Invalid,
        "{:?}",
        v[0].bites
    );
}

fn load_mark() -> Value {
    json!({
        "id": "PM-0001-2", "title": "the load is the stress scene", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "SS N=48", "select": {"scene": "SS", "clients": 48}, "min": 1}],
        "checks": [{"name": "48 soldiers", "series": "tick.v.soldiers_count", "stat": "min", "op": ">=",
                    "value": 48, "parts": ["SS N=48"]}]
    })
}

#[test]
fn b8_a_run_that_did_not_carry_the_load_is_invalid() {
    let budget = tick_mark(
        M,
        json!({"bites": [], "parts": [{"name": "S30", "select": {"scene": "SS", "clients": 48}, "min": 1,
                                       "validity": ["pre:PM-0001-2"]}]}),
    );
    let run = |soldiers: Option<i64>| {
        let mut s = LogSpec::room("r1", "set-p", M);
        s.marks = vec![M, "PM-0001-2"];
        let mut v = vec![("work_ns", 50)];
        if let Some(n) = soldiers {
            v.push(("soldiers_count", n));
        }
        s.ticks = vec![(Phase::Measured, v)];
        s.build()
    };
    let marks = [budget.clone(), load_mark()];
    let v = judge(&marks_file(&marks), &[run(Some(48))]);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
    let v = judge(&marks_file(&marks), &[run(Some(47))]);
    assert_eq!(v[0].verdict, VerdictKind::Invalid, "{:?}", v[0].why);
    assert!(
        v[0].why[0].contains("pre:PM-0001-2: 48 soldiers = 47, needs >= 48"),
        "{:?}",
        v[0].why
    );
    let v = judge(&marks_file(&marks), &[run(None)]);
    assert_eq!(
        v[0].verdict,
        VerdictKind::Invalid,
        "a precondition with a gap: {:?}",
        v[0].why
    );
    // A precondition that does not exist, or names itself, is refused when marks.json is read.
    let bad = tick_mark(
        M,
        json!({"bites": [], "parts": [{"name": "S30", "select": {"scene": "SS"}, "min": 1,
                                       "validity": ["pre:PM-0001-99"]}]}),
    );
    let text = json!({"schema": 1, "drafts_commit": "a".repeat(40), "marks": [bad]}).to_string();
    assert!(npt_judge::parse_marks(&text).is_err());
}

#[test]
fn everyone_in_view_takes_the_worst_client() {
    let mark = json!({
        "id": "PM-0001-2", "title": "the load", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "SS", "select": {"scene": "SS"}, "min": 1}],
        "checks": [{"name": "everyone in view", "figure": "everyone_in_view_min_share", "op": ">=", "value": "0.99"}]
    });
    let run = |second_client_misses: usize| {
        let mut s = LogSpec::room("r1", "set-v", "PM-0001-2");
        s.marks = vec!["PM-0001-2"];
        let mut t: Vec<(Phase, Vec<(&'static str, i64)>)> = Vec::new();
        for _ in 0..100 {
            t.push((Phase::Measured, vec![("work_ns", 1)]));
        }
        s.ticks = t;
        let mut log = s.build();
        for (i, tick) in log.ticks.iter_mut().enumerate() {
            let second = i64::from(i >= second_client_misses);
            tick.s
                .insert("everyone_in_view".to_owned(), vec![1, second]);
        }
        log
    };
    // Client 2 sees everyone on 99 of 100 snapshots: at 0.99, it holds; on 98, it fails.
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &[run(1)]).0,
        VerdictKind::Pass
    );
    let (k, why) = verdict_of(&[mark], &[run(2)]);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(why[0].contains("everyone in view = 0.98"), "{why:?}");
}

#[test]
fn b9_a_failed_setup_makes_the_run_invalid_never_failed() {
    let mut mark = tick_mark(
        M,
        json!({"bites": [{"id": "PM-0001-4/setup-only", "what": "x"}]}),
    );
    mark["checks"].as_array_mut().expect("checks").push(json!({
        "name": "setup: the S30 starts below the wrap", "series": "header.params.start_tick",
        "stat": "max", "op": "<=", "value": 63736, "setup": true}));
    let run = |id: &'static str, start: i64, role: &'static str, work: i64| {
        let mut s = LogSpec::room(id, "set-s", M);
        s.role = role;
        if role == "bite" {
            s.bite = Some("PM-0001-4/setup-only");
        }
        s.params.push(("start_tick", json!(start)));
        s.ticks = vec![(Phase::Measured, vec![("work_ns", work)])];
        s.build()
    };
    let good = vec![
        run("r1", 63736, "measured", 50),
        run("r2", 63736, "measured", 50),
        run("r3", 63736, "measured", 50),
        run("b1", 63736, "bite", 150),
    ];
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &good).0,
        VerdictKind::Pass
    );
    let mut logs = good.clone();
    logs[1] = run("r2", 70000, "measured", 50);
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &logs);
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        why[0].contains("setup: setup: the S30 starts below the wrap = 70000"),
        "{why:?}"
    );
    // A bite whose run breaks only its setup is not shown: no bite may target a setup check.
    let mut logs = good;
    logs[3] = run("b1", 70000, "bite", 50);
    let v = judge(&marks_file(&[mark]), &logs);
    assert_eq!(
        v[0].bites[0].status,
        BiteStatus::Invalid,
        "{:?}",
        v[0].bites
    );
}

#[test]
fn b13_a_validity_bite_is_shown_only_by_its_own_rule() {
    let mark = json!({
        "id": "PM-0001-30", "title": "both transports", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "RS", "select": {"params.suite": "both"}, "min": 1}],
        "validity": ["late-zero"],
        "checks": [{"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100}],
        "bites": [{"id": "PM-0001-30/late-command", "what": "one command delayed past its tick",
                   "expect": "invalid", "rule": "late-zero"}]
    });
    let run = |id: &'static str, role: &'static str, late: i64, cut: bool| {
        let mut s = LogSpec::room(id, "set-t", "PM-0001-30");
        s.role = role;
        if role == "bite" {
            s.bite = Some("PM-0001-30/late-command");
        }
        s.params.push(("suite", json!("both")));
        s.ticks = vec![(Phase::Measured, vec![("work_ns", 50), ("late_count", late)])];
        s.footer = !cut;
        s.build()
    };
    let measured = run("m1", "measured", 0, false);
    let v = judge(
        &marks_file(std::slice::from_ref(&mark)),
        &[measured.clone(), run("b1", "bite", 1, false)],
    );
    assert_eq!(v[0].bites[0].status, BiteStatus::Shown, "{:?}", v[0].bites);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
    let v = judge(
        &marks_file(std::slice::from_ref(&mark)),
        &[measured.clone(), run("b1", "bite", 0, false)],
    );
    assert_eq!(v[0].bites[0].status, BiteStatus::Missed);
    assert_eq!(
        v[0].verdict,
        VerdictKind::Fail,
        "the validity check let a late command through"
    );
    let v = judge(
        &marks_file(&[mark]),
        &[measured, run("b1", "bite", 1, true)],
    );
    assert_eq!(
        v[0].bites[0].status,
        BiteStatus::Invalid,
        "invalid by another rule (a cut log) shows nothing"
    );
}

#[test]
fn q10_q22_the_0_1st_percentile_and_the_first_n_records() {
    let mark = json!({
        "id": "PM-0004-67", "title": "the loop keeps time", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "S30", "select": {"layer": "L2"}, "min": 1}],
        "checks": [{"name": "jitter p99 over the first 4 measured ticks", "series": "tick.v.start_jitter_ns",
                    "stat": "p99", "first": 4, "op": "<=", "value": 2000000},
                   {"name": "lower end of the delay", "series": "tick.v.delay_ns", "stat": "p001",
                    "op": ">=", "value": 10}]
    });
    let mut s = LogSpec::room("r1", "set-j", "PM-0004-67");
    s.marks = vec!["PM-0004-67"];
    // Ticks 5 and 6 carry a stall's jitter: outside the first 4 measured ticks, so they do not count.
    let jitter = [100, 200, 300, 400, 9_000_000, 9_000_000];
    let delay = [10, 11, 12, 13, 14, 15];
    s.ticks = jitter
        .iter()
        .zip(delay)
        .map(|(j, d)| {
            (
                Phase::Measured,
                vec![("start_jitter_ns", *j), ("delay_ns", d)],
            )
        })
        .collect();
    s.ticks.insert(
        0,
        (
            Phase::Warmup,
            vec![("start_jitter_ns", 9_000_000), ("delay_ns", 1)],
        ),
    );
    let (k, why) = verdict_of(&[mark], &[s.build()]);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
}

#[test]
fn q27_pm_0006_19s_off_runs_must_agree_within_1_percent() {
    let mark = json!({
        "id": "PM-0006-19", "title": "logging cost", "verdict": "pass",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "on", "select": {"params.logging": "on"}, "min": 1}],
        "validity": ["PM-0006-19"],
        "checks": [{"name": "on p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 1000}]
    });
    let run = |id: &'static str, role: &'static str, logging: &'static str, v: &[i64]| {
        let mut s = LogSpec::room(id, "set-l", "PM-0006-19").ticks("work_ns", v);
        s.role = role;
        s.params.push(("logging", json!(logging)));
        s.build()
    };
    let on = run("on-1", "measured", "on", &[100]);
    let agree = vec![
        on.clone(),
        run("off-1", "logging-off", "off", &[100]),
        run("off-2", "logging-off", "off", &[100]),
    ];
    assert_eq!(
        verdict_of(std::slice::from_ref(&mark), &agree).0,
        VerdictKind::Pass
    );
    let apart = vec![
        on,
        run("off-1", "logging-off", "off", &[100]),
        run("off-2", "logging-off", "off", &[103]),
    ];
    let (k, why) = verdict_of(&[mark], &apart);
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        why[0].contains("PM-0006-19: logging-off run off-1's p99 (100 ns) is more than 1 %"),
        "{why:?}"
    );
}

/// PM-0001-3's identity check alone: one L1 part, the twin figure, and the clock-seed bite.
fn twin_mark() -> Value {
    json!({
        "id": "PM-0001-3", "title": "profiles", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "L1 B1", "select": {"layer": "L1", "profile": "B1"}, "min": 1}],
        "checks": [{"name": "identical fate log", "figure": "fate_log_twin_equal", "op": "==", "value": 1,
                    "parts": ["L1 B1"]}],
        "bites": [{"id": "PM-0001-3/clock-seed", "what": "the RNG seeded from the clock"}]
    })
}

/// A probe log at L1 B1, seed 7, with its fate log's hash (one hex digit repeated) and the twin it names.
fn probe(run: &'static str, role: &'static str, hash: char, twin: Option<&'static str>) -> LogSpec {
    let mut s = LogSpec::room(run, "set-t", "PM-0001-3");
    s.role = role;
    s.layer = "L1";
    s.profile = "B1";
    s.seed = 7;
    s.process = "probe";
    s.params = vec![("fate_log_sha256", json!(hash.to_string().repeat(64)))];
    if let Some(t) = twin {
        s.params.push(("twin", json!(t)));
    }
    s
}

/// The clock-seed bite run and its twin, both in the bites build: their fate logs differ, so the bite is shown.
fn shown_clock_bite() -> Vec<Log> {
    let mut bite = probe("bite", "bite", 'b', Some("bite-twin"));
    bite.bite = Some("PM-0001-3/clock-seed");
    let mut twin = probe("bite-twin", "twin", 'c', None);
    twin.bite = Some("PM-0001-3/clock-seed");
    vec![bite.build(), twin.build()]
}

#[test]
fn u2_the_twin_figure_passes_on_one_fate_log_and_shows_the_clock_seeded_bite() {
    let mut logs = vec![
        probe("b1", "measured", 'a', Some("b1-twin")).build(),
        probe("b1-twin", "twin", 'a', None).build(),
    ];
    logs.extend(shown_clock_bite());
    let (k, why) = verdict_of(&[twin_mark()], &logs);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
    // The same bite run against a twin with its own hash is missed: the check cannot be shown to work.
    let mut missed = logs[..2].to_vec();
    let mut bite = probe("bite", "bite", 'b', Some("bite-twin"));
    bite.bite = Some("PM-0001-3/clock-seed");
    let mut twin = probe("bite-twin", "twin", 'b', None);
    twin.bite = Some("PM-0001-3/clock-seed");
    missed.extend([bite.build(), twin.build()]);
    let (k, why) = verdict_of(&[twin_mark()], &missed);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(why.iter().any(|w| w.contains("did not fail")), "{why:?}");
}

#[test]
fn u2_two_fate_logs_fail_and_a_missing_twin_is_not_run() {
    let mut logs = vec![
        probe("b1", "measured", 'a', Some("b1-twin")).build(),
        probe("b1-twin", "twin", 'd', None).build(),
    ];
    logs.extend(shown_clock_bite());
    let (k, why) = verdict_of(&[twin_mark()], &logs);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(
        why[0].contains("identical fate log = 0, needs == 1"),
        "{why:?}"
    );
    let mut alone = vec![probe("b1", "measured", 'a', Some("b1-twin")).build()];
    alone.extend(shown_clock_bite());
    let (k, why) = verdict_of(&[twin_mark()], &alone);
    assert_eq!(k, VerdictKind::NotRun, "{why:?}");
    assert!(
        why[0].contains("its twin b1-twin is not in set set-t yet"),
        "{why:?}"
    );
}

#[test]
fn u2_a_twin_record_that_cannot_be_compared_is_a_gap() {
    let unnamed = probe("b1", "measured", 'a', None);
    let mut not_a_twin = probe("b1-twin", "measured", 'a', Some("b1"));
    not_a_twin.run = "b1-twin";
    let mut other_seed = probe("b1-twin", "twin", 'a', None);
    other_seed.seed = 8;
    let mut bad_hash = probe("b1-twin", "twin", 'a', None);
    bad_hash.params = vec![("fate_log_sha256", json!("A".repeat(64)))];
    let mut measured_twin_of_a_bite = probe("bite-twin", "twin", 'c', None);
    measured_twin_of_a_bite.bite = None;
    let mut bite = probe("bite", "bite", 'b', Some("bite-twin"));
    bite.bite = Some("PM-0001-3/clock-seed");
    let cases: Vec<(Vec<Log>, &str)> = vec![
        (vec![unnamed.build()], "params.twin names no twin run"),
        (
            vec![
                probe("b1", "measured", 'a', Some("b1-twin")).build(),
                not_a_twin.build(),
            ],
            "b1-twin has role measured, not twin",
        ),
        (
            vec![
                probe("b1", "measured", 'a', Some("b1-twin")).build(),
                other_seed.build(),
            ],
            "its twin b1-twin has another seed",
        ),
        (
            vec![
                probe("b1", "measured", 'a', Some("b1-twin")).build(),
                bad_hash.build(),
            ],
            "run b1-twin: params.fate_log_sha256 is missing or not 64 lowercase hex digits",
        ),
        (
            vec![
                probe("b1", "measured", 'a', Some("b1-twin")).build(),
                probe("b1-twin", "twin", 'a', None).build(),
                bite.build(),
                measured_twin_of_a_bite.build(),
            ],
            // A bite run's twin from the measured build: the bite run is invalid (its reason is in the bite's detail).
            "bite PM-0001-3/clock-seed: its run is invalid",
        ),
    ];
    for (logs, expected) in cases {
        let (k, why) = verdict_of(&[twin_mark()], &logs);
        assert_eq!(k, VerdictKind::Invalid, "{expected}: {why:?}");
        assert!(
            why.iter().any(|w| w.contains(expected)),
            "{expected}: {why:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------------------------
// U4j: C4's build rule, the decoded and twin hash figures, machine_named, and -18's ws-practice rule.

fn says(why: &[String], text: &str) -> bool {
    why.iter().any(|w| w.contains(text))
}

/// C4 (the advisor's U2 ruling): a measured run built with the bites, a measured run naming a bite, and a bite run
/// built without them are INVALID; so is a measured run's twin built with them (checked where the twin is read).
#[test]
fn c4_the_build_matches_the_bite() {
    let bite_run = |features: Option<Vec<&'static str>>| {
        let mut s = LogSpec::room("bite-1", "set-a", M).ticks("work_ns", &[150]);
        s.role = "bite";
        s.bite = Some("PM-0001-4/B-S");
        s.features = features;
        s.build()
    };
    let with = |i: usize, change: &dyn Fn(&mut LogSpec)| {
        let mut logs = three_runs(M, [90, 95, 100]);
        let mut s =
            LogSpec::room(["run-1", "run-2", "run-3"][i], "set-a", M).ticks("work_ns", &[10, 95]);
        change(&mut s);
        logs[i] = s.build();
        logs.push(bite_run(None));
        verdict_of(&[tick_mark(M, json!({}))], &logs)
    };
    let (k, why) = with(1, &|_| {});
    assert_eq!(k, VerdictKind::Pass, "the base: {why:?}");
    let (k, why) = with(1, &|s| s.features = Some(vec!["bites"]));
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        says(
            &why,
            "run run-2: build: a measured run names no bite, but a log of it comes from a bites build"
        ),
        "{why:?}"
    );
    let (k, why) = with(2, &|s| s.bite = Some("PM-0001-4/B-S"));
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        says(
            &why,
            "run run-3: build: a measured run names the bite PM-0001-4/B-S"
        ),
        "{why:?}"
    );
    let mut logs = three_runs(M, [90, 95, 100]);
    logs.push(bite_run(Some(vec![])));
    let v = judge(&marks_file(&[tick_mark(M, json!({}))]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Invalid, "{:?}", v[0].why);
    assert_eq!(v[0].bites[0].status, BiteStatus::Invalid);
    assert!(
        says(
            &v[0].bites[0].runs[0].invalid_why,
            "build: the run names the bite PM-0001-4/B-S, but none of its logs comes from a bites build"
        ),
        "{:?}",
        v[0].bites[0].runs
    );
    // On L2 the bite lives in one binary: RS's room from the bites build, LG's bots from the measured one, is right.
    let mut lg = LogSpec::room("bite-1", "set-a", M);
    lg.role = "bite";
    lg.bite = Some("PM-0001-4/B-S");
    lg.machine_role = "LG";
    lg.process = "bots";
    lg.features = Some(vec![]);
    let mut logs = three_runs(M, [90, 95, 100]);
    logs.extend([bite_run(None), lg.build()]);
    assert_eq!(
        verdict_of(&[tick_mark(M, json!({}))], &logs).0,
        VerdictKind::Pass
    );
    // A measured run's twin from the bites build: the twin figure reads it, so the measured run is INVALID.
    let mut twin = probe("b1-twin", "twin", 'a', None);
    twin.features = Some(vec!["bites"]);
    let mut logs = vec![
        probe("b1", "measured", 'a', Some("b1-twin")).build(),
        twin.build(),
    ];
    logs.extend(shown_clock_bite());
    let (k, why) = verdict_of(&[twin_mark()], &logs);
    assert_eq!(k, VerdictKind::Invalid, "{why:?}");
    assert!(
        says(
            &why,
            "run b1: its twin b1-twin is invalid: build: a twin run names no bite, but a log of it comes from a bites build"
        ),
        "{why:?}"
    );
}

/// A decoded-hash mark like PM-0001-17: one L1 part, the figure, the compared count and the Arm rule's report.
fn hash_mark() -> Value {
    json!({
        "id": "PM-0001-17", "title": "decoded", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "L1", "select": {"layer": "L1"}, "min": 1}],
        "checks": [{"name": "0 mismatches", "figure": "decoded_hash_mismatch_count", "op": "==", "value": 0}],
        "report": [{"name": "compared", "figure": "decoded_hash_compared_count"},
                   {"name": "aarch64 only", "figure": "cross_arch_mismatch_count"}],
        "bites": [{"id": "PM-0001-17/tick16", "what": "a 16-bit tick"}]
    })
}

/// One log of a hashed run: `name`'s list on each tick 1, 2, ... (an empty list leaves that tick without it).
fn hash_log(
    run: &'static str,
    process: &'static str,
    name: &str,
    per_tick: &[&[Option<u64>]],
    edit: &dyn Fn(&mut LogSpec),
) -> Log {
    let mut s = LogSpec::room(run, "set-h", "PM-0001-17");
    s.layer = "L1";
    s.process = process;
    s.clients = 2;
    s.ticks = per_tick.iter().map(|_| (Phase::Measured, vec![])).collect();
    edit(&mut s);
    let mut log = s.build();
    for (t, list) in log.ticks.iter_mut().zip(per_tick) {
        if !list.is_empty() {
            t.set_hashes(name, list);
        }
    }
    log
}

const SERVER: [&[Option<u64>]; 3] = [
    &[Some(11), Some(21)],
    &[Some(12), Some(22)],
    &[Some(13), Some(23)],
];

fn report<'v>(v: &'v npt_judge::Verdict, name: &str) -> Vec<&'v str> {
    v.report
        .iter()
        .filter(|r| r.name == name)
        .map(|r| r.value.as_str())
        .collect()
}

#[test]
fn u4j_the_decoded_hash_gaps_and_its_every_phase() {
    let no = &|_: &mut LogSpec| {};
    let room = hash_log("h1", "room", "server_hash", &SERVER, no);
    let client = |lists: &[&[Option<u64>]]| hash_log("h1", "client", "decoded_hash", lists, no);
    let equal = client(&SERVER);
    let (k, why) = verdict_of(&[hash_mark()], &[room.clone(), equal.clone()]);
    assert_eq!(k, VerdictKind::NotRun, "the bite has no run: {why:?}");
    let incomplete = |list: Vec<i64>| {
        let mut log = equal.clone();
        log.ticks[0].s.insert("snap_incomplete".to_owned(), list);
        log
    };
    let cases: Vec<(Vec<Log>, &str)> = vec![
        (vec![equal.clone()], "no log gives hs.server_hash"),
        (vec![room.clone()], "no log gives hs.decoded_hash"),
        (
            vec![
                room.clone(),
                client(&[&[Some(11), Some(21), None], &[], &[]]),
            ],
            "hs.decoded_hash lists hold 3 entries, not the run's 2 clients (the room log's header)",
        ),
        (
            vec![
                room.clone(),
                equal.clone(),
                hash_log("h1", "client", "decoded_hash", &[&[Some(11), None]], no),
            ],
            "two logs give slot 0 at tick 1 an hs.decoded_hash",
        ),
        (
            vec![
                room.clone(),
                hash_log("h1", "lab", "decoded_hash", &SERVER, no),
            ],
            "hs.decoded_hash in the lab log on RS1 (x86_64): only process client (L1) or bots (L2) writes it",
        ),
        (
            vec![
                room.clone(),
                equal.clone(),
                hash_log("h1", "client", "server_hash", &SERVER, no),
            ],
            "hs.server_hash in the client log on RS1 (x86_64): only process room writes it",
        ),
        // FX-10 (a) and its malformed entries: a snap_incomplete list of the run's clients entries, each 0 or 1.
        (
            vec![room.clone(), incomplete(vec![0, 0, 0])],
            "tick 1's tick.s.snap_incomplete in the client log on RS1 holds 3 entries, not the run's 2 clients",
        ),
        (
            vec![room.clone(), incomplete(vec![2, 0])],
            "tick 1's tick.s.snap_incomplete holds 2 for slot 0, not 0 or 1",
        ),
        (
            vec![
                hash_log("h1", "room", "server_hash", &[&[Some(0), Some(21)]], no),
                client(&[&[Some(0), Some(21)]]),
            ],
            "the server logged 0 for slot 0 at tick 1; its hash is never 0",
        ),
    ];
    for (logs, expected) in cases {
        let v = judge(&marks_file(&[hash_mark()]), &logs);
        assert_eq!(
            v[0].verdict,
            VerdictKind::Invalid,
            "{expected}: {:?}",
            v[0].why
        );
        assert!(says(&v[0].why, expected), "{expected}: {:?}", v[0].why);
    }
    // A warm-up tick is read too (-58's wraps fall in its first seconds): a mismatch there fails the run.
    let warm = hash_log(
        "h1",
        "client",
        "decoded_hash",
        &[&[Some(11), Some(99)]],
        &|s| {
            s.ticks = vec![(Phase::Warmup, vec![])];
        },
    );
    let (k, why) = verdict_of(&[hash_mark()], &[room, warm]);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(says(&why, "0 mismatches = 1, needs == 0"), "{why:?}");
}

#[test]
fn u4j_the_arm_rule_counts_an_aarch64_mismatch_only_when_the_x86_64_re_decode_misses_too() {
    let no = &|_: &mut LogSpec| {};
    let arm = &|s: &mut LogSpec| {
        s.arch = "aarch64";
        s.machine_role = "LG";
    };
    let room = hash_log("h1", "room", "server_hash", &SERVER, no);
    let wrong: [&[Option<u64>]; 3] = [
        &[Some(11), Some(21)],
        &[Some(12), Some(99)],
        &[Some(13), Some(23)],
    ];
    let client = hash_log("h1", "bots", "decoded_hash", &wrong, arm);
    let replay = |lists: &[&[Option<u64>]]| hash_log("h1", "replay", "replay_hash", lists, no);
    // The x86-64 re-decode agrees with the server: an aarch64-only finding, reported, deciding nothing.
    let v = judge(
        &marks_file(&[hash_mark()]),
        &[room.clone(), client.clone(), replay(&SERVER)],
    );
    assert_eq!(
        v[0].parts[0].verdict,
        VerdictKind::Pass,
        "{:?}",
        v[0].parts[0].why
    );
    assert_eq!(report(&v[0], "aarch64 only"), ["1"]);
    assert_eq!(report(&v[0], "compared"), ["6"]);
    // It misses the server's hash too: a real mismatch.
    let (k, why) = verdict_of(
        &[hash_mark()],
        &[room.clone(), client.clone(), replay(&wrong)],
    );
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    // No re-decode yet: NOT-RUN, never a guess.
    let v = judge(&marks_file(&[hash_mark()]), &[room.clone(), client]);
    assert_eq!(
        v[0].parts[0].verdict,
        VerdictKind::NotRun,
        "{:?}",
        v[0].parts[0].why
    );
    assert!(
        says(&v[0].parts[0].why, "the run has no x86-64 re-decode"),
        "{:?}",
        v[0].parts[0].why
    );
    // A coverage gap beside a waiting Arm mismatch (TE2-4): slot 0's tick 2 is left unresolved while slot 1's LG
    // mismatch waits for its re-decode. The gap decides: INVALID, never NOT-RUN.
    let holed: [&[Option<u64>]; 3] = [
        &[Some(11), Some(21)],
        &[None, Some(99)],
        &[Some(13), Some(23)],
    ];
    let lg = hash_log("h1", "bots", "decoded_hash", &holed, arm);
    let v = judge(&marks_file(&[hash_mark()]), &[room.clone(), lg]);
    let part = &v[0].parts[0];
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    assert!(
        says(&part.why, "no client log resolves it"),
        "{:?}",
        part.why
    );
    // On x86-64 a mismatch counts as it stands, with or without a replay.
    let x86 = hash_log("h1", "client", "decoded_hash", &wrong, no);
    let (k, _) = verdict_of(&[hash_mark()], &[room, x86, replay(&SERVER)]);
    assert_eq!(k, VerdictKind::Fail);
    // The Arm rule needs the room on x86_64 (the advisor's FX-5 (b)): with an aarch64 room, the LG mismatch counts.
    let arm_room = hash_log("h1", "room", "server_hash", &SERVER, &|s| {
        s.arch = "aarch64"
    });
    let lg = hash_log("h1", "bots", "decoded_hash", &wrong, arm);
    let (k, why) = verdict_of(&[hash_mark()], &[arm_room, lg, replay(&SERVER)]);
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    // The rule's key is LG (TE2-1): an aarch64 `client` log on role C or ML, the room on x86_64 and the re-decode
    // agreeing with the server, is a mismatch that counts (1), and nothing is reported as aarch64 only.
    for role in ["C", "ML"] {
        let room = hash_log("h1", "room", "server_hash", &SERVER, no);
        let client = hash_log("h1", "client", "decoded_hash", &wrong, &|s| {
            s.arch = "aarch64";
            s.machine_role = role;
        });
        let v = judge(
            &marks_file(&[hash_mark()]),
            &[room, client, replay(&SERVER)],
        );
        let part = &v[0].parts[0];
        assert_eq!(part.verdict, VerdictKind::Fail, "{role}: {:?}", part.why);
        assert!(says(&part.why, "= 1, needs"), "{role}: {:?}", part.why);
        assert_eq!(report(&v[0], "aarch64 only"), ["0"], "{role}");
    }
}

/// A reason carries log text (run ids, a person's name). `summary_line` escapes every control character, so the line
/// stays one line and no forged verdict line starts on stdout (SEC-R2-N2). The reason is set here by hand: npt-log's
/// writer refuses a control character in a run id, and the judge does not rest on that.
#[test]
fn the_summary_line_escapes_control_characters() {
    let mut v = judge(
        &marks_file(&[tick_mark(M, json!({"bites": []}))]),
        &three_runs(M, [90, 101, 100]),
    );
    assert_eq!(v[0].verdict, VerdictKind::Fail);
    v[0].why[0] = "S30: run run-2\nPM-0001-9    PASS\ttick p99 = 101\u{1b}[2K".to_owned();
    let line = npt_judge::summary_line(&v[0]);
    assert_eq!(
        line,
        "PM-0001-4    FAIL      S30: run run-2\\nPM-0001-9    PASS\\ttick p99 = 101\\u{1b}[2K"
    );
}

#[test]
fn u4j_an_empty_comparison_fails_the_run_and_shows_no_bite() {
    let no = &|_: &mut LogSpec| {};
    let room = hash_log("h1", "room", "server_hash", &SERVER, no);
    let none: [&[Option<u64>]; 3] = [&[None, None], &[None, None], &[None, None]];
    let v = judge(
        &marks_file(&[hash_mark()]),
        &[
            room.clone(),
            hash_log("h1", "client", "decoded_hash", &none, no),
        ],
    );
    assert_eq!(v[0].verdict, VerdictKind::Fail, "{:?}", v[0].why);
    assert!(
        says(
            &v[0].why,
            "L1: run h1: 0 mismatches: run h1: no client logged a hash (0 compared); an empty comparison fails"
        ),
        "{:?}",
        v[0].why
    );
    assert_eq!(report(&v[0], "compared"), ["0"]);
    let bite = |id: &'static str, lists: &[&[Option<u64>]]| {
        let b = &|s: &mut LogSpec| {
            s.role = "bite";
            s.bite = Some("PM-0001-17/tick16");
        };
        let mut logs = vec![
            hash_log(id, "room", "server_hash", &SERVER, b),
            hash_log(id, "client", "decoded_hash", lists, b),
        ];
        for l in &mut logs {
            l.header.run = id.to_owned();
        }
        logs
    };
    let equal = hash_log("h1", "client", "decoded_hash", &SERVER, no);
    // A bite run that compared nothing did not test the bite: INVALID, never shown.
    let mut logs = vec![room.clone(), equal.clone()];
    logs.extend(bite("b1", &none));
    let v = judge(&marks_file(&[hash_mark()]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Invalid, "{:?}", v[0].why);
    assert_eq!(v[0].bites[0].status, BiteStatus::Invalid);
    assert!(
        says(
            &v[0].bites[0].runs[0].invalid_why,
            "an empty comparison shows no bite"
        ),
        "{:?}",
        v[0].bites[0].runs
    );
    // Post-wrap frames under the wrong tick: hashes at ticks the server never logged count, and show the bite.
    let mut logs = vec![room, equal];
    let mut wrapped = bite("b1", &[&[Some(11), Some(21)]]);
    wrapped[1].ticks[0].t = 9;
    logs.extend(wrapped);
    let v = judge(&marks_file(&[hash_mark()]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
    assert_eq!(v[0].bites[0].status, BiteStatus::Shown);
}

/// C-U6-11's run h1 over measured ticks 1 to 12: slot 0's room hash at the ticks in `room` and its client hash at
/// those in `client`; slot 1 has both at every tick and no session record (one span, as before). `events` are slot
/// 0's room-side session records (name, tick, the process whose log carries it).
fn session_run(room: &[u64], client: &[u64], events: &[(&str, u64, &'static str)]) -> Vec<Log> {
    let lists = |own: &[u64]| -> Vec<Vec<Option<u64>>> {
        let slot0 = |t: u64| own.contains(&t).then_some(100 + t);
        (1..=12).map(|t| vec![slot0(t), Some(200 + t)]).collect()
    };
    let log = |process: &'static str, name: &str, own: &[u64]| {
        let lists = lists(own);
        let lists: Vec<&[Option<u64>]> = lists.iter().map(Vec::as_slice).collect();
        let mut log = hash_log("h1", process, name, &lists, &|_| {});
        let of_process = events.iter().filter(|e| e.2 == process).map(|(ev, t, _)| {
            let mut e = EventRecord::new(ev, Measured).with("slot", 0);
            if *ev == "admit" {
                e = e.with("session", *t).with("in_roster", true);
            }
            e.t = Some(*t);
            e
        });
        log.events = of_process.collect();
        log
    };
    let mut lab = log("lab", "server_hash", &[]);
    lab.ticks
        .iter_mut()
        .for_each(|t| drop(t.hs.remove("server_hash")));
    vec![
        log("room", "server_hash", room),
        log("client", "decoded_hash", client),
        lab,
    ]
}

/// The L1 part's run of `session_run`'s logs under `hash_mark`: whether it counts, and its coverage gap.
fn covered(logs: &[Log]) -> (bool, String) {
    let o = &judge(&marks_file(&[hash_mark()]), logs)[0].parts[0].runs[0];
    (o.valid, o.invalid_why.join("; "))
}

const FIRST: [u64; 6] = [1, 2, 3, 4, 5, 6];
const SECOND: [u64; 4] = [9, 10, 11, 12];

/// C-U6-11 (-19's joins): the coverage rule runs per session, cut at the room log's `admit` records. A session that
/// left may leave the room's last hashes unresolved, up to its recorded `session_end` and never past it.
#[test]
fn c_u6_11_coverage_runs_per_session_cut_at_the_admits() {
    let room = [&FIRST[..], &SECOND[..]].concat();
    let client = [&FIRST[..4], &SECOND[..]].concat();
    // Two sessions: the first leaves with ticks 5 and 6 in flight, its end recorded at tick 6; re-admitted at 9.
    let left = [
        ("admit", 1, "room"),
        ("session_end", 6, "lab"),
        ("admit", 9, "room"),
    ];
    assert_eq!(
        covered(&session_run(&room, &client, &left)),
        (true, String::new())
    );
    // The end recorded in the room's log (L2's home) counts the same.
    let l2 = [
        ("admit", 1, "room"),
        ("session_end", 6, "room"),
        ("admit", 9, "room"),
    ];
    assert!(covered(&session_run(&room, &client, &l2)).0);
    // No session_end: a silent stop never passes.
    let silent = [("admit", 1, "room"), ("admit", 9, "room")];
    let (valid, why) = covered(&session_run(&room, &client, &silent));
    assert!(!valid, "{why}");
    assert!(
        why.contains(
            "run h1: slot 0, its session admitted at tick 1: the client side resolves ticks 1 to 4 only, not the \
             room's measured ticks 1 to 6 (no session_end records that it left)"
        ),
        "{why}"
    );
    // An end only a client-side log records is not the room's word.
    let claimed = [
        ("admit", 1, "room"),
        ("session_end", 6, "client"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&room, &client, &claimed));
    assert!(
        !valid && why.contains("(no session_end records that it left)"),
        "{why}"
    );
    // The room logged a hash after the recorded end.
    let early = [
        ("admit", 1, "room"),
        ("session_end", 5, "lab"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&room, &client, &early));
    assert!(
        !valid && why.contains("measured ticks 1 to 6 (its session_end is at tick 5)"),
        "{why}"
    );
    // The slot's last session may leave for good the same way: its end recorded at tick 6, ticks 5 and 6 in flight.
    let gone = [("admit", 1, "room"), ("session_end", 6, "lab")];
    assert_eq!(
        covered(&session_run(&FIRST, &FIRST[..4], &gone)),
        (true, String::new())
    );
    // Still connected at the stop (no session_end): its span reaches the room's last measured hash, or it is a gap.
    let (valid, why) = covered(&session_run(&FIRST, &FIRST[..4], &[("admit", 1, "room")]));
    assert!(
        !valid && why.contains("measured ticks 1 to 6 (no session_end records that it left)"),
        "{why}"
    );
    // And never past its end: the room logged its hash at tick 6, after the end at tick 5.
    let past = [("admit", 1, "room"), ("session_end", 5, "lab")];
    let (valid, why) = covered(&session_run(&FIRST, &FIRST[..4], &past));
    assert!(
        !valid && why.contains("measured ticks 1 to 6 (its session_end is at tick 5)"),
        "{why}"
    );
    // A session the client never resolved a tick of.
    let (valid, why) = covered(&session_run(&room, &SECOND, &left));
    assert!(
        !valid
            && why.contains(
                "slot 0, its session admitted at tick 1: the room logged its hash at measured ticks 1 to 6, and \
                 no client log resolves any tick of it"
            ),
        "{why}"
    );
}

/// C-U6-11's edges (the test-engineer's review, M2): whose `session_end` counts, which one, and whose `admit`.
#[test]
fn c_u6_11_each_session_has_its_own_end_and_only_the_room_side_admits() {
    // Three sessions (admitted at 1, 5 and 9). The first stops silently after tick 2; the second's end at tick 8
    // is not the first's.
    let every: Vec<u64> = (1..=12).collect();
    let ends = [
        ("admit", 1, "room"),
        ("admit", 5, "room"),
        ("session_end", 8, "room"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&every, &[1, 2, 5, 6, 9, 10, 11, 12], &ends));
    assert!(
        !valid
            && why.contains(
                "its session admitted at tick 1: the client side resolves ticks 1 to 2 only"
            )
            && why.contains("(no session_end records that it left)"),
        "{why}"
    );
    // Two ends recorded in one session: the first is its end, so the room's hash at tick 6 is past it.
    let room = [&FIRST[..], &SECOND[..]].concat();
    let client = [&FIRST[..4], &SECOND[..]].concat();
    let two = [
        ("admit", 1, "room"),
        ("session_end", 5, "room"),
        ("session_end", 6, "lab"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&room, &client, &two));
    assert!(
        !valid && why.contains("(its session_end is at tick 5)"),
        "{why}"
    );
    // An admit only a client-side log records cuts nothing: with it, ticks 5 and 6 would be an allowed tail.
    let claimed = [
        ("admit", 1, "room"),
        ("session_end", 6, "lab"),
        ("admit", 7, "client"),
    ];
    let (valid, why) = covered(&session_run(
        &every,
        &[1, 2, 3, 4, 7, 8, 9, 10, 11, 12],
        &claimed,
    ));
    assert!(
        !valid
            && why.contains("slot 0, tick 5: the room logged a hash and no client log resolves it"),
        "{why}"
    );
}

/// C-U6-11's head and the two cases the advisor's plan review fixed (condition 5).
#[test]
fn c_u6_11_a_mid_run_join_and_the_slots_with_no_admit_record() {
    let joined = [("admit", 5, "room")];
    let from = |a: u64| (a..=12).collect::<Vec<u64>>();
    // Admitted in the measured phase at tick 5, its first snapshot resolved at 5.
    assert_eq!(
        covered(&session_run(&from(5), &from(5), &joined)),
        (true, String::new())
    );
    // Its first resolved tick is 7, after the room's first hash of the session.
    let (valid, why) = covered(&session_run(&from(5), &from(7), &joined));
    assert!(!valid, "{why}");
    assert!(
        why.contains(
            "slot 0, its session admitted at tick 5: the client side resolves ticks 7 to 12 only, not the room's \
             measured ticks 5 to 12"
        ) && !why.contains("12 ("),
        "{why}"
    );
    // A measured room hash before the slot's first admit belongs to no session.
    let (valid, why) = covered(&session_run(&from(3), &from(5), &joined));
    assert!(
        !valid && why.contains("slot 0: the room logged its hash at measured tick 3, before the slot's first admit (tick 5)"),
        "{why}"
    );
    // A slot with no admit record keeps its single span: a join nobody recorded still fails, in the words as before.
    let (valid, why) = covered(&session_run(&from(5), &from(7), &[]));
    assert!(
        !valid
            && why.contains(
                "run h1: slot 0: the client side resolves ticks 7 to 12 only, not the room's measured ticks 5 to 12"
            ),
        "{why}"
    );
    // The same for a leave and a rejoin with no admit record, even with the end recorded.
    let room = [&FIRST[..], &SECOND[..]].concat();
    let client = [&FIRST[..4], &SECOND[..]].concat();
    let (valid, why) = covered(&session_run(&room, &client, &[("session_end", 6, "lab")]));
    assert!(
        !valid
            && why.contains("slot 0, tick 5: the room logged a hash and no client log resolves it"),
        "{why}"
    );
    // And a slot with no admit record gets no tail, whatever end is recorded: the rule as it was.
    let (valid, why) = covered(&session_run(
        &FIRST,
        &FIRST[..4],
        &[("session_end", 6, "lab")],
    ));
    let old = "run h1: slot 0: the client side resolves ticks 1 to 4 only, not the room's measured ticks 1 to 6";
    assert!(!valid && why.ends_with(old), "{why}");
    // An admit record with no tick cuts nothing: the two sessions read as one, and the ticks in flight are a gap.
    let both = [
        ("admit", 1, "room"),
        ("session_end", 6, "lab"),
        ("admit", 9, "room"),
    ];
    let mut logs = session_run(&room, &client, &both);
    logs[0].events[1].t = None;
    let (valid, why) = covered(&logs);
    assert!(
        !valid
            && why.contains("slot 0, tick 5: the room logged a hash and no client log resolves it"),
        "{why}"
    );
}

// C62H-BOUND's two figures (SERIES.md's frozen rows): A, snap_lost_unexplained_count; B, the link record's converse.

/// A -17 mark with the two C62H-BOUND checks (== 0) on one L1 part, and the whole-as-lost bite.
fn bound_mark() -> Value {
    json!({
        "id": "PM-0001-17", "title": "bound", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "L1", "select": {"layer": "L1"}, "min": 1}],
        "checks": [{"name": "A", "figure": "snap_lost_unexplained_count", "op": "==", "value": 0},
                   {"name": "B", "figure": "snap_whole_despite_link_loss_count", "op": "==", "value": 0}],
        "bites": [{"id": "PM-0001-17/whole-as-lost", "what": "a whole tick filed lost"}]
    })
}

/// One tick of both logs: its phase, then the room's `hs.server_hash` and `tick.s.snap_link_lost_count` and the
/// client's `hs.decoded_hash` and `tick.s.snap_incomplete`, 2 slots each (a hash of -1: `null`).
type BoundTick = (Phase, [[i64; 2]; 4]);

/// Run c1's room and client logs (`hash_log`'s), one record per tick on each.
fn bound_run(ticks: &[BoundTick]) -> Vec<Log> {
    let none: Vec<&[Option<u64>]> = vec![&[][..]; ticks.len()];
    let phases = |s: &mut LogSpec| s.ticks = ticks.iter().map(|t| (t.0, vec![])).collect();
    let mut room = hash_log("c1", "room", "server_hash", &none, &phases);
    let mut client = hash_log("c1", "client", "decoded_hash", &none, &phases);
    let hashes = |l: [i64; 2]| l.map(|h| u64::try_from(h).ok());
    for (i, (_, [server, link, decoded, lost])) in ticks.iter().enumerate() {
        let (r, c) = (&mut room.ticks[i], &mut client.ticks[i]);
        r.set_hashes("server_hash", &hashes(*server));
        r.s.insert("snap_link_lost_count".into(), link.to_vec());
        c.set_hashes("decoded_hash", &hashes(*decoded));
        c.s.insert("snap_incomplete".into(), lost.to_vec());
    }
    vec![room, client]
}

/// The L1 part's run, judged on `logs` with `bound_mark`.
fn bound_out(logs: &[Log]) -> RunOut {
    judge(&marks_file(&[bound_mark()]), logs)[0].parts[0].runs[0].clone()
}

/// The checks' outcomes and values, A then B (`-`: no value).
fn ab(o: &RunOut) -> Vec<String> {
    o.checks
        .iter()
        .map(|c| format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-")))
        .collect()
}

/// A measured tick at which slot 0 is filed lost though the room sent it a snapshot and the link lost none of it.
const UNEXPLAINED: BoundTick = (Measured, [[1, 2], [0, 0], [-1, 2], [1, 0]]);

#[test]
fn c62hb_each_explanation_alone_reads_0() {
    // Slot 0: the room sent it nothing, the link lost none (E1 alone); slot 1: sent, the link lost one (E2 alone).
    let t = (Measured, [[-1, 2], [0, 1], [-1, -1], [1, 1]]);
    let o = bound_out(&bound_run(&[t]));
    assert_eq!(ab(&o), ["Pass 0", "Pass 0"], "{o:?}");
}

#[test]
fn c62hb_an_unexplained_tick_reads_1() {
    let o = bound_out(&bound_run(&[UNEXPLAINED]));
    assert_eq!(ab(&o), ["Fail 1", "Pass 0"], "{o:?}");
}

#[test]
fn c62hb_warmup_and_after_phase_ticks_are_not_read() {
    // In each phase: slot 0 an unexplained tick; slot 1 a whole hash at a tick the link lost a datagram of.
    let odd = |ph| (ph, [[1, 2], [0, 1], [-1, 2], [1, 0]]);
    let o = bound_out(&bound_run(&[odd(Warmup), odd(After)]));
    assert_eq!(ab(&o), ["Pass 0", "Pass 0"], "(A, B): {o:?}");
}

#[test]
fn c62hb_the_converse_and_the_sentinel() {
    let whole = |h| bound_run(&[(Measured, [[1, 2], [1, 0], [h, 2], [0, 0]])]);
    assert_eq!(ab(&bound_out(&whole(1))), ["Pass 0", "Fail 1"]);
    // The sentinel 0 (a refused group) is not a whole hash.
    assert_eq!(ab(&bound_out(&whole(0))), ["Pass 0", "Pass 0"]);
    // The only such tick is at a tick the room has no record of: not read here (the frozen row).
    let mut logs = whole(1);
    logs[1].ticks[0].t = 2;
    let o = bound_out(&logs);
    assert!(o.valid && o.gaps.is_empty(), "{o:?}");
    assert_eq!(ab(&o), ["Pass 0", "Pass 0"], "{o:?}");
}

#[test]
fn c62hb_eight_gaps_read_on_both_figures() {
    const LINK: &str = "snap_link_lost_count";
    const FLAGS: &str = "snap_incomplete";
    // Tick 1: slot 1 filed lost, the room sent it nothing (E1); tick 2: both whole.
    let base = [
        (Measured, [[1, -1], [0, 0], [1, -1], [0, 1]]),
        (Measured, [[3, 4], [0, 0], [3, 4], [0, 0]]),
    ];
    // The cases: the series in no room log; one room tick without it; one without hs.server_hash, where slot 1 is
    // filed lost; its list one entry short; a negative entry; a measured client record without snap_incomplete; a
    // measured incomplete tick with no room record; a snap_incomplete entry of 2.
    let edits: [fn(&mut [Log]); 8] = [
        |l| l[0].ticks.iter_mut().for_each(|t| drop(t.s.remove(LINK))),
        |l| drop(l[0].ticks[0].s.remove(LINK)),
        |l| drop(l[0].ticks[0].hs.remove("server_hash")),
        |l| drop(l[0].ticks[1].s.insert(LINK.into(), vec![0])),
        |l| drop(l[0].ticks[1].s.insert(LINK.into(), vec![0, -1])),
        |l| drop(l[1].ticks[1].s.remove(FLAGS)),
        |l| drop(l[0].ticks.remove(0)),
        |l| drop(l[1].ticks[1].s.insert(FLAGS.into(), vec![0, 2])),
    ];
    let phrases = [
        "no room log gives tick.s.snap_link_lost_count",
        "a room tick record without tick.s.snap_link_lost_count",
        "a room tick record without hs.server_hash",
        "a list that does not hold the run's clients entries",
        "an entry below 0",
        "a measured client tick record without tick.s.snap_incomplete",
        "a measured incomplete tick the room log has no record of",
        "a snap_incomplete entry other than 0 or 1",
    ];
    for (i, (edit, phrase)) in edits.iter().zip(phrases).enumerate() {
        let mut logs = bound_run(&base);
        edit(&mut logs);
        let o = bound_out(&logs);
        assert!(!o.valid && ab(&o) == ["Gap -", "Gap -"], "{phrase}: {o:?}");
        for c in &o.checks {
            let note = c.note.as_deref().unwrap_or("");
            assert!(note.contains(phrase), "{phrase}: {note}");
            // The shadowing pair split: one room tick without the series is not "no room log gives" it.
            assert!(i != 1 || !note.contains(phrases[0]), "{note}");
        }
    }
}

#[test]
fn c62hb_a_bite_with_an_unexplained_tick_is_shown() {
    let mut logs = bound_run(&[UNEXPLAINED]);
    for l in &mut logs {
        l.header.role = "bite".to_owned();
        l.header.bite = Some("PM-0001-17/whole-as-lost".to_owned());
        l.header.build.features = vec!["bites".to_owned()];
    }
    let v = judge(&marks_file(&[bound_mark()]), &logs);
    let b = &v[0].bites[0];
    assert_eq!(b.status, BiteStatus::Shown, "{:?}", b.runs);
    assert!(b.runs[0].valid && b.runs[0].gaps.is_empty(), "{:?}", b.runs);
    assert_eq!(ab(&b.runs[0]), ["Fail 1", "Pass 0"], "{:?}", b.runs);
}

/// A twin-hash mark like PM-0001-21's forged part.
fn twin_hash_mark() -> Value {
    json!({
        "id": "PM-0001-21", "title": "forged", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "forged", "select": {"params.suite": "forged"}, "min": 1}],
        "checks": [{"name": "state hash equals the twin's", "figure": "twin_hash_mismatch_count", "op": "==", "value": 0}]
    })
}

/// A room log with one state hash per tick (`None`: no `h` on that tick), naming its twin.
fn hashed(
    run: &'static str,
    role: &'static str,
    twin: Option<&'static str>,
    hashes: &[Option<u64>],
) -> LogSpec {
    let mut s = LogSpec::room(run, "set-w", "PM-0001-21");
    s.role = role;
    s.layer = "L1";
    s.params = vec![("suite", json!("forged"))];
    if let Some(t) = twin {
        s.params.push(("twin", json!(t)));
    }
    s.ticks = hashes.iter().map(|_| (Phase::Measured, vec![])).collect();
    s
}

fn with_hashes(s: LogSpec, hashes: &[Option<u64>]) -> Log {
    let mut log = s.build();
    for (t, h) in log.ticks.iter_mut().zip(hashes) {
        if let Some(h) = h {
            t.set_hash(*h);
        }
    }
    log
}

#[test]
fn u4j_the_twin_hash_compares_every_tick_and_its_twin_whole() {
    let pair = |own: &[Option<u64>], other: &[Option<u64>]| {
        vec![
            with_hashes(hashed("f1", "measured", Some("f1-twin"), own), own),
            with_hashes(hashed("f1-twin", "twin", None, other), other),
        ]
    };
    let h = [Some(1), Some(2), Some(3)];
    assert_eq!(
        verdict_of(&[twin_hash_mark()], &pair(&h, &h)).0,
        VerdictKind::Pass
    );
    let (k, why) = verdict_of(&[twin_hash_mark()], &pair(&h, &[Some(1), Some(9), Some(3)]));
    assert_eq!(k, VerdictKind::Fail);
    assert!(
        says(&why, "state hash equals the twin's = 1, needs == 0"),
        "{why:?}"
    );
    let (k, why) = verdict_of(&[twin_hash_mark()], &pair(&h, &[Some(1), Some(2), None]));
    assert_eq!(
        k,
        VerdictKind::Fail,
        "a tick hashed in one only counts: {why:?}"
    );
    let (k, why) = verdict_of(&[twin_hash_mark()], &pair(&[None], &[None]));
    assert_eq!(k, VerdictKind::Fail, "{why:?}");
    assert!(
        says(&why, "hashed no tick; an empty comparison fails"),
        "{why:?}"
    );
    let (k, why) = verdict_of(&[twin_hash_mark()], &pair(&h, &h)[..1]);
    assert_eq!(k, VerdictKind::NotRun, "{why:?}");
    assert!(
        says(&why, "its twin f1-twin is not in set set-w yet"),
        "{why:?}"
    );
    let mut seed = hashed("f1-twin", "twin", None, &h);
    seed.seed = 2;
    let mut bites = hashed("f1-twin", "twin", None, &h);
    bites.features = Some(vec!["bites"]);
    let mut cut = hashed("f1-twin", "twin", None, &h);
    cut.footer = false;
    let mut profile = hashed("f1-twin", "twin", None, &h);
    profile.profile = "P3";
    let mut other_bite = hashed("f1-twin", "twin", None, &h);
    other_bite.bite = Some("PM-0001-21/other");
    let measured = hashed("f1-twin", "measured", Some("f1"), &h);
    let own = || with_hashes(hashed("f1", "measured", Some("f1-twin"), &h), &h);
    let mut bad_h = own();
    bad_h.ticks[1].h = Some("not-a-hash".to_owned());
    let mut second = LogSpec::room("f1", "set-w", "PM-0001-21");
    second.layer = "L1";
    second.process = "lab";
    second.params = vec![("suite", json!("forged"))];
    second.ticks = vec![(Phase::Measured, vec![])];
    let cases: Vec<(Vec<Log>, &str)> = vec![
        (
            vec![with_hashes(seed, &h), own()],
            "its twin f1-twin has another seed",
        ),
        (
            vec![with_hashes(bites, &h), own()],
            "its twin f1-twin is invalid: build:",
        ),
        (
            vec![with_hashes(cut, &h), own()],
            "its twin f1-twin is invalid: complete:",
        ),
        (
            vec![with_hashes(profile, &h), own()],
            "its twin f1-twin has another profile",
        ),
        (
            vec![with_hashes(other_bite, &h), own()],
            "its twin f1-twin has another bite",
        ),
        (
            vec![with_hashes(measured, &h), own()],
            "run f1: f1-twin has role measured, not twin",
        ),
        (
            vec![with_hashes(hashed("f1", "measured", Some("f1"), &h), &h)],
            "params.twin names the run itself",
        ),
        (
            vec![with_hashes(hashed("f1", "measured", None, &h), &h)],
            "params.twin names no twin run",
        ),
        (
            pair(&h, &h).into_iter().skip(1).chain([bad_h]).collect(),
            "tick 2's h is not 16 lowercase hex digits",
        ),
        (
            pair(&h, &h)
                .into_iter()
                .chain([with_hashes(second, &[Some(7)])])
                .collect(),
            "two logs give tick 1 two state hashes",
        ),
    ];
    for (logs, expected) in cases {
        let (k, why) = verdict_of(&[twin_hash_mark()], &logs);
        assert_eq!(k, VerdictKind::Invalid, "{expected}: {why:?}");
        assert!(says(&why, expected), "{expected}: {why:?}");
    }
}

#[test]
fn u4j_machine_named_needs_every_field_and_a_vm_its_address() {
    let mark = json!({
        "id": "PM-0006-11", "title": "RS named", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "RS", "select": {"process": "room"}, "min": 1}],
        "checks": [{"name": "named", "figure": "machine_named", "op": "==", "value": 1}]
    });
    let run = |role: &'static str, edit: &dyn Fn(&mut npt_log::Machine)| {
        let mut s = LogSpec::room("m1", "set-m", "PM-0006-11");
        s.machine_role = role;
        let mut log = s.build();
        log.header.machine.address = "203.0.113.10".to_owned();
        edit(&mut log.header.machine);
        verdict_of(std::slice::from_ref(&mark), &[log]).0
    };
    assert_eq!(run("RS", &|_| {}), VerdictKind::Pass);
    assert_eq!(
        run("C", &|m| m.address.clear()),
        VerdictKind::Pass,
        "not a VM: no address"
    );
    type Edit<'a> = &'a dyn Fn(&mut npt_log::Machine);
    let missing: [(&str, Edit<'_>); 14] = [
        ("a VM with no address", &|m| m.address.clear()),
        ("no kernel", &|m| m.kernel.clear()),
        ("no CPU model", &|m| m.cpu.clear()),
        ("no location", &|m| m.location.clear()),
        ("no vCPU count", &|m| m.vcpus = 0),
        ("no SMT map", &|m| m.smt.clear()),
        ("an empty sibling list", &|m| m.smt.push(vec![])),
        ("no clock resolution", &|m| m.clock_res_ns = 0),
        // The advisor's FX-14 (TE-F13): every other condition, each alone.
        ("no role", &|m| m.role.clear()),
        ("no name", &|m| m.name.clear()),
        ("no plan", &|m| m.plan.clear()),
        ("no arch", &|m| m.arch.clear()),
        ("no memory", &|m| m.mem_mb = 0),
        ("no threads per core", &|m| m.threads_per_core = 0),
    ];
    for (why, edit) in missing {
        assert_eq!(run("RS", edit), VerdictKind::Fail, "{why}");
    }
}

// PM-0001-18's WebSocket part, judged with the real marks.json: the ws-practice rule and the numbers in force.

fn real_marks() -> npt_judge::Marks {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../marks/marks.json"
    ))
    .expect("marks");
    parse_marks(&text).expect("marks.json compiles")
}

/// marks.json's PM-0001-18 `ws_bound_line`, the provisional line (the advisor's FX-6 (d)): a record carries it byte for
/// byte.
const WS_BOUND: &str = r#"npt-ws-bound v1 {"model":"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound","commit":"bf9d24807106c4524d1aa0849409d219259c3943","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;

/// One WebSocket B1 run of 480 client-bursts: `low` at 600 ms, the 240th (the median) at `median`, then `high` at
/// 700 ms, and `slow` at `slow_ns`.
fn heals(median: i64, slow: usize, slow_ns: i64) -> Vec<i64> {
    let mut v = vec![600_000_000; 239];
    v.push(median);
    v.extend(std::iter::repeat_n(700_000_000, 240 - slow));
    v.extend(std::iter::repeat_n(slow_ns, slow));
    v
}

/// The bound's model, as marks.json's provisional `ws_bound_line` names it (FX-6 (d)).
const WS_MODEL: &str = "provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound";

/// A run's setup the ws-practice rule compares (the advisor's FX-13): RS's `machine.kernel` (`None`: no RS log in the
/// run), `params.config_hz` and `params.ws_tcp_model` (`None`: left out).
#[derive(Clone)]
struct Setup {
    kernel: Option<&'static str>,
    config_hz: Option<i64>,
    model: Option<&'static str>,
}

const SETUP: Setup = Setup {
    kernel: Some("6.8.0-48-generic"),
    config_hz: Some(250),
    model: Some(WS_MODEL),
};

/// Everything a ws-practice scenario changes.
#[derive(Clone)]
struct Ws {
    counted: Vec<i64>,
    practice: Vec<i64>,
    record: Vec<(&'static str, Value)>,
    practice_started: &'static str,
    practice_binary: &'static str,
    practice_scene: &'static str,
    practice_spacing: i64,
    practice_footer: bool,
    records: usize,
    counted_started: Option<&'static str>,
    second_practice: bool,
    /// The counted run's setup (the bite run's too) and the practice run's.
    counted_setup: Setup,
    practice_setup: Setup,
}

/// The base: M_p 680 ms against M_m 654 ms, so s = f = 680/654; the numbers in force are 697 ms, 2.6 s and 7.175 s
/// (the bound's 670 ms, 2.5 s and 6.9 s times f, rounded up to the ms); the counted run's median, 690 ms, and its 6
/// heals at 2.55 s pass only at those numbers.
fn ws_base() -> Ws {
    Ws {
        counted: heals(690_000_000, 6, 2_550_000_000),
        practice: heals(680_000_000, 5, 2_590_000_000),
        record: vec![
            ("class", json!("IN TOLERANCE")),
            ("m_p_ns", json!(680_000_000)),
            ("bound", json!(WS_BOUND)),
            (
                "bound_file",
                json!("spikes/netcode-proof/results/u3-ws-fallback/bound.txt"),
            ),
            ("bound_sha256", json!("0".repeat(64))),
            ("x_moved_ns", json!(697_000_000)),
            ("y_moved_ns", json!(2_600_000_000_i64)),
            ("z_moved_ns", json!(7_175_000_000_i64)),
            ("k_count", json!(5)),
            ("s", json!("1.039755")),
            ("f", json!("1.039755")),
        ],
        practice_started: "2026-11-20T18:00:00Z",
        practice_binary: "a",
        practice_scene: "SS",
        practice_spacing: 30_000_000_000,
        practice_footer: true,
        records: 1,
        counted_started: Some("2026-11-20T21:00:00Z"),
        second_practice: false,
        counted_setup: SETUP,
        practice_setup: SETUP,
    }
}

fn ws_log(
    run: &'static str,
    role: &'static str,
    binary: &'static str,
    started: Option<&'static str>,
    heals: &[i64],
) -> LogSpec {
    let mut s = LogSpec::room(run, "set-ws", "PM-0001-18");
    s.role = role;
    s.machine_role = "LG";
    s.process = "bots";
    s.profile = "B1";
    s.binary = binary;
    s.params = vec![
        ("transport", json!("ws")),
        ("burst_spacing_ns", json!(30_000_000_000_i64)),
    ];
    if let Some(t) = started {
        s.params.push(("started_utc", json!(t)));
    }
    s.events = heals
        .iter()
        .enumerate()
        .map(|(i, h)| {
            EventRecord::new("burst_heal", Phase::Measured)
                .with("after_link_ns", *h)
                .with("slot", i as i64 % 48)
                .with("burst", i as i64 / 48)
        })
        .collect();
    s
}

/// One WebSocket B1 run: LG's bots log (`edit` applied to it) with the setup's params, and RS's room log carrying the
/// setup's kernel when it names one.
fn ws_run(
    run: &'static str,
    role: &'static str,
    binary: &'static str,
    started: Option<&'static str>,
    heals: &[i64],
    setup: &Setup,
    edit: &dyn Fn(&mut LogSpec),
) -> Vec<Log> {
    let mut bots = ws_log(run, role, binary, started, heals);
    if let Some(m) = setup.model {
        bots.params.push(("ws_tcp_model", json!(m)));
    }
    if let Some(hz) = setup.config_hz {
        bots.params.push(("config_hz", json!(hz)));
    }
    edit(&mut bots);
    let bite = bots.bite;
    let mut logs = vec![bots.build()];
    if let Some(k) = setup.kernel {
        let mut rs = LogSpec::room(run, "set-ws", "PM-0001-18");
        rs.role = role;
        rs.profile = "B1";
        rs.binary = binary;
        rs.bite = bite;
        rs.params = vec![("transport", json!("ws"))];
        let mut log = rs.build();
        log.header.machine.kernel = k.to_owned();
        logs.push(log);
    }
    logs
}

/// Applies `edit` to every log of run `id`.
fn edit_run(logs: &mut [Log], id: &str, edit: &dyn Fn(&mut Log)) {
    for l in logs.iter_mut().filter(|l| l.header.run == id) {
        edit(l);
    }
}

fn ws_logs(w: &Ws) -> Vec<Log> {
    let binary = |c: &str| -> &'static str { if c == "a" { "aa" } else { "bb" } };
    let mut practice = ws_run(
        "ws-practice",
        "practice",
        binary(w.practice_binary),
        Some(w.practice_started),
        &w.practice,
        &w.practice_setup,
        &|s| {
            s.footer = w.practice_footer;
            s.params[1] = ("burst_spacing_ns", json!(w.practice_spacing));
            for _ in 0..w.records {
                let mut e = EventRecord::new("ws_practice", Phase::After);
                for (k, v) in &w.record {
                    e = e.with(k, v.clone());
                }
                s.events.push(e);
            }
        },
    );
    edit_run(&mut practice, "ws-practice", &|l| {
        l.header.scene = w.practice_scene.to_owned();
    });
    let mut logs = ws_run(
        "ws-1",
        "measured",
        "aa",
        w.counted_started,
        &w.counted,
        &w.counted_setup,
        &|_| {},
    );
    logs.extend(practice.clone());
    if w.second_practice {
        logs.extend(another_practice(
            &practice,
            "ws-practice-0",
            "2026-11-20T17:00:00Z",
            &[],
        ));
    }
    logs.extend(ws_run(
        "ws-bite",
        "bite",
        "cc",
        Some("2026-11-20T22:00:00Z"),
        &heals(776_000_000, 0, 0),
        &w.counted_setup,
        &|s| s.bite = Some("PM-0001-18/ws-budget-x2"),
    ));
    logs
}

fn ws_part(w: &Ws) -> (npt_judge::eval::PartOut, npt_judge::Verdict) {
    ws_judge(&ws_logs(w))
}

/// -18 judged on these logs with the real marks.json: the WebSocket part and the mark's verdict.
fn ws_judge(logs: &[Log]) -> (npt_judge::eval::PartOut, npt_judge::Verdict) {
    let marks = real_marks();
    let i = marks
        .marks
        .iter()
        .position(|m| m.id == "PM-0001-18")
        .expect("-18");
    let v = judge(&marks, logs).remove(i);
    let part = v
        .parts
        .iter()
        .find(|p| p.name == "L2 B1 ws")
        .expect("the WebSocket part")
        .clone();
    (part, v)
}

#[test]
fn u4j_ws_practice_puts_the_practice_runs_numbers_in_force() {
    let (part, v) = ws_part(&ws_base());
    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
    let checks = &part.runs[0].checks;
    let median = checks
        .iter()
        .find(|c| c.name.starts_with("WebSocket: half"))
        .expect("check c");
    assert_eq!(median.value.as_deref(), Some("690000000"));
    assert!(
        median
            .needs
            .starts_with("<= 697000000 (X 670000000 x f 1.039755 = 697000000"),
        "{}",
        median.needs
    );
    assert!(
        median.needs.contains("M_p 680000000 / M_m 654000000"),
        "{}",
        median.needs
    );
    // The practice outcome printed whole (the advisor's FX-11, R8 (v): "the ruling is in the run's report").
    for text in [
        "practice run ws-practice: IN TOLERANCE",
        "band 0.965 to 1.035",
        "tolerance 1.10",
        "f = 1.039755 (max(1, s))",
    ] {
        assert!(median.needs.contains(text), "{text}: {}", median.needs);
    }
    let over = checks
        .iter()
        .find(|c| c.name.starts_with("WebSocket: at most"))
        .expect("check d");
    assert_eq!(
        over.value.as_deref(),
        Some("0"),
        "none over Y' = 2.6 s (6 over the unmoved 2.5 s)"
    );
    let bite = v
        .bites
        .iter()
        .find(|b| b.id == "PM-0001-18/ws-budget-x2")
        .expect("the WebSocket bite");
    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
    // FAR BETTER, with the advisor's chance ruling: f = 1, so the bound itself is in force and 690 ms fails it.
    let mut w = ws_base();
    w.practice = heals(600_000_000, 5, 2_590_000_000);
    w.record[0].1 = json!("FAR BETTER");
    w.record[1].1 = json!(600_000_000);
    w.record
        .extend([("chance_ruling", json!("advisor-r3.md:12"))]);
    w.record[5].1 = json!(670_000_000);
    w.record[6].1 = json!(2_500_000_000_i64);
    w.record[7].1 = json!(6_900_000_000_i64);
    w.practice = heals(600_000_000, 0, 0);
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Fail, "{:?}", part.why);
    assert!(
        says(
            &part.why,
            "WebSocket: half the client-bursts heal within X x f"
        ),
        "{:?}",
        part.why
    );
    // A second practice run of one build counts only with its ruling; the latest is the one in force.
    let mut w = ws_base();
    w.second_practice = true;
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    assert!(
        says(&part.why, "2 practice runs of one build"),
        "{:?}",
        part.why
    );
    w.record.push((
        "second_ruling",
        json!("advisor-r3.md:40: the cause found and fixed"),
    ));
    assert_eq!(ws_part(&w).0.verdict, VerdictKind::Pass);
    // BEYOND under a chance ruling never reads as IN TOLERANCE: the class and the ruling's text are printed.
    let mut w = ws_base();
    w.record[0].1 = json!("BEYOND");
    w.record
        .push(("chance_ruling", json!("advisor-r3.md:20: a lone count")));
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
    let median = part.runs[0]
        .checks
        .iter()
        .find(|c| c.name.starts_with("WebSocket: half"))
        .expect("check c");
    for text in [
        "practice run ws-practice: BEYOND",
        "chance ruling: \"advisor-r3.md:20: a lone count\"",
    ] {
        assert!(median.needs.contains(text), "{text}: {}", median.needs);
    }
}

#[test]
fn u4j_ws_practice_refuses_every_number_not_set_by_its_rules() {
    let field = |name: &'static str, value: Value| {
        move |w: &mut Ws| {
            for f in &mut w.record {
                if f.0 == name {
                    f.1 = value.clone();
                }
            }
        }
    };
    let bound = |from: &'static str, to: &'static str| {
        field("bound", json!(WS_BOUND.replacen(from, to, 1)))
    };
    type Edit = Box<dyn Fn(&mut Ws)>;
    let cases: Vec<(Edit, &str)> = vec![
        (
            Box::new(|w| w.practice_binary = "b"),
            "run ws-1: no practice run of its build started before it",
        ),
        (
            Box::new(|w| w.practice_started = "2026-11-20T22:00:00Z"),
            "run ws-1: no practice run of its build started before it",
        ),
        (
            Box::new(|w| w.counted_started = None),
            "run ws-1 has no params.started_utc",
        ),
        (
            Box::new(|w| w.counted_started = Some("20 Nov 2026 21:00")),
            "is not YYYY-MM-DDTHH:MM:SSZ",
        ),
        (
            Box::new(|w| w.practice_scene = "ES"),
            "ran another scene or profile",
        ),
        (
            Box::new(|w| w.practice_spacing = 20_000_000_000),
            "ran another burst spacing",
        ),
        (
            Box::new(|w| w.practice_footer = false),
            "practice run ws-practice is invalid: complete:",
        ),
        (
            Box::new(|w| w.records = 0),
            "holds 0 ws_practice records, not 1",
        ),
        (
            Box::new(|w| w.records = 2),
            "holds 2 ws_practice records, not 1",
        ),
        (
            Box::new(field("class", json!("BEYOND"))),
            "is \"BEYOND\" at s = 1.039755",
        ),
        (Box::new(field("class", json!("INVALID"))), "is \"INVALID\""),
        (
            Box::new(field("m_p_ns", json!(681_000_000))),
            "the record's m_p_ns is not practice run ws-practice's median",
        ),
        (
            Box::new(field("x_moved_ns", json!(700_000_000))),
            "the record's x_moved_ns is not the judge's X 670000000",
        ),
        (
            Box::new(field("k_count", json!(6))),
            "the record's k_count is not the bound's",
        ),
        // A record's bound line other than marks.json's, in any field (FX-6: these numbers' agreement with the
        // checks and the rule is checked at load, marks.rs ws_bound_line_agrees_with_its_checks).
        (
            Box::new(bound(r#""tolerance":"1.10""#, r#""tolerance":"1.05""#)),
            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
        ),
        (
            Box::new(bound(r#""x_ns":670000000"#, r#""x_ns":680000000"#)),
            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
        ),
        (
            Box::new(bound(r#""k_count":5"#, r#""k_count":4"#)),
            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
        ),
        (
            Box::new(bound("npt-ws-bound v1 ", "npt-ws-bound ")),
            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
        ),
        (
            Box::new(|w| w.practice = heals(680_000_000, 6, 2_610_000_000)),
            "but practice run ws-practice has 6 heals over 2600000000 ns",
        ),
        // FX-7 (the advisor's R7: "Valid only with 480 heal records"; probe G): one heal short or over, M_p unchanged.
        (
            Box::new(|w| {
                let i = w
                    .practice
                    .iter()
                    .position(|h| *h == 700_000_000)
                    .expect("a 700 ms heal");
                w.practice.remove(i);
            }),
            "practice run ws-practice has 479 heal records, not the bound's 480",
        ),
        (
            Box::new(|w| w.practice.insert(0, 600_000_000)),
            "practice run ws-practice has 481 heal records, not the bound's 480",
        ),
        // FX-13 (the advisor's R7: "a header naming the model, RS's kernel and CONFIG_HZ"): the practice run ran on
        // the counted run's setup, its model the bound's.
        (
            Box::new(|w| w.practice_setup.kernel = Some("6.8.0-49-generic")),
            "practice run ws-practice ran another RS's machine.kernel than run ws-1",
        ),
        (
            Box::new(|w| w.practice_setup.config_hz = Some(1000)),
            "practice run ws-practice ran another params.config_hz than run ws-1",
        ),
        (
            Box::new(|w| w.practice_setup.model = Some("linux-6.8-tcp v3")),
            "practice run ws-practice ran another params.ws_tcp_model than run ws-1",
        ),
        (
            Box::new(|w| {
                w.practice_setup.model = Some("linux-6.8-tcp v3");
                w.counted_setup.model = Some("linux-6.8-tcp v3");
            }),
            "run ws-1's params.ws_tcp_model is not the bound's model",
        ),
        (
            Box::new(|w| {
                w.practice_setup.kernel = None;
                w.counted_setup.kernel = None;
            }),
            "run ws-1 has no RS's machine.kernel",
        ),
        (
            Box::new(|w| {
                w.practice_setup.config_hz = None;
                w.counted_setup.config_hz = None;
            }),
            "run ws-1 has no params.config_hz",
        ),
        (
            Box::new(|w| {
                w.practice_setup.model = None;
                w.counted_setup.model = None;
            }),
            "run ws-1 has no params.ws_tcp_model",
        ),
        (
            Box::new(|w| {
                // M_p 740 ms: s = 740/654 = 1.1315, beyond the tolerance (the 240th of 480 heals, 750 ms above it).
                let mut v = vec![600_000_000; 239];
                v.push(740_000_000);
                v.extend(std::iter::repeat_n(750_000_000, 235));
                v.extend(std::iter::repeat_n(2_590_000_000, 5));
                w.practice = v;
                w.record[1].1 = json!(740_000_000);
            }),
            "is \"IN TOLERANCE\" at s = 1.131498",
        ),
    ];
    for (edit, expected) in cases {
        let mut w = ws_base();
        edit(&mut w);
        let (part, v) = ws_part(&w);
        assert_eq!(
            part.verdict,
            VerdictKind::Invalid,
            "{expected}: {:?}",
            part.why
        );
        assert!(says(&part.why, expected), "{expected}: {:?}", part.why);
        assert_eq!(v.verdict, VerdictKind::Invalid, "{expected}");
    }
    // BEYOND with the advisor's chance ruling (a lone count or maximum failure, s within the tolerance) counts.
    let mut w = ws_base();
    field("class", json!("BEYOND"))(&mut w);
    w.record.push(("chance_ruling", json!("advisor-r3.md:20")));
    assert_eq!(ws_part(&w).0.verdict, VerdictKind::Pass);
}

/// The record's field `name` set to `value`.
fn set_field(w: &mut Ws, name: &str, value: Value) {
    for f in &mut w.record {
        if f.0 == name {
            f.1 = value.clone();
        }
    }
}

/// The test-engineer's probe A (the advisor's FX-1, the High): a bound line whose band is 2.0 to 2.0 and a FAR BETTER
/// record with a chance ruling at s = 1.9 would move X, Y and Z by 90 % (X' 1.273 s), past Mustafa's 10 %. The
/// numbers never move past the tolerance: the run is INVALID, and its 1.2 s median never passes.
#[test]
fn u4j_ws_far_better_past_the_cap_is_invalid() {
    let mut w = ws_base();
    let mut counted = vec![600_000_000; 239];
    counted.push(1_200_000_000);
    counted.extend(std::iter::repeat_n(1_250_000_000, 240));
    w.counted = counted;
    let mut practice = vec![600_000_000; 239];
    practice.push(1_242_600_000);
    practice.extend(std::iter::repeat_n(1_300_000_000, 240));
    w.practice = practice;
    let line = WS_BOUND
        .replacen(r#""band_low":"0.965""#, r#""band_low":"2.0""#, 1)
        .replacen(r#""band_high":"1.035""#, r#""band_high":"2.0""#, 1);
    for (name, value) in [
        ("class", json!("FAR BETTER")),
        ("m_p_ns", json!(1_242_600_000)),
        ("bound", json!(line)),
        ("x_moved_ns", json!(1_273_000_000)),
        ("y_moved_ns", json!(4_750_000_000_i64)),
        ("z_moved_ns", json!(13_110_000_000_i64)),
    ] {
        set_field(&mut w, name, value);
    }
    w.record.push(("chance_ruling", json!("x")));
    // Probe A: the record's line widens the band, so it is not the mark's line: the pin refuses it by its own reason.
    let (part, v) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    let pin = "its bound line is not PM-0001-18's ws_bound_line, byte for byte";
    assert!(says(&part.why, pin), "{:?}", part.why);
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    // Probe A pinned (TE2-2): the record carries marks.json's own line, and FAR BETTER at s = 1.9 with a chance
    // ruling is refused by the class rule, since 1.9 is not below the band.
    set_field(&mut w, "bound", json!(WS_BOUND));
    let (part, v) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    assert!(
        says(&part.why, r#"is "FAR BETTER" at s = 1.9"#),
        "{:?}",
        part.why
    );
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
}

/// 480 practice heals with the median (the 240th) at `median`: 239 at 600 ms below it, then `rest` above it and
/// `slow` at `slow_ns`.
fn practice_heals(median: i64, rest: i64, slow: usize, slow_ns: i64) -> Vec<i64> {
    let mut v = vec![600_000_000; 239];
    v.push(median);
    v.extend(std::iter::repeat_n(rest, 240 - slow));
    v.extend(std::iter::repeat_n(slow_ns, slow));
    v
}

/// The advisor's boundaries on the outcome record (advisor-ws-docs-r2.md:378: "s = 1.00, 1.10 and 1.1001"; FX-9 and
/// the test-engineer's T1a, T1b, T2, T3): Mustafa's 10 % cap, band_low and the Z' recheck, each pinned.
#[test]
fn u4j_ws_the_tolerance_and_band_boundaries() {
    let needs = |part: &npt_judge::eval::PartOut, prefix: &str| {
        part.runs[0]
            .checks
            .iter()
            .find(|c| c.name.starts_with(prefix))
            .map(|c| c.needs.clone())
            .unwrap_or_default()
    };
    let record = |w: &mut Ws, m_p: i64, moved: [i64; 3]| {
        set_field(w, "m_p_ns", json!(m_p));
        set_field(w, "x_moved_ns", json!(moved[0]));
        set_field(w, "y_moved_ns", json!(moved[1]));
        set_field(w, "z_moved_ns", json!(moved[2]));
    };
    // s = 1.00: the bound itself, f = 1.
    let mut w = ws_base();
    w.practice = practice_heals(654_000_000, 700_000_000, 5, 2_590_000_000);
    w.counted = heals(660_000_000, 5, 2_550_000_000);
    record(
        &mut w,
        654_000_000,
        [670_000_000, 2_500_000_000, 6_900_000_000],
    );
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Pass, "s = 1.00: {:?}", part.why);
    let c = needs(&part, "WebSocket: half");
    assert!(
        c.starts_with("<= 670000000 (X 670000000 x f 1 = 670000000"),
        "s = 1.00: {c}"
    );
    // s = 1.10 (T1a): at the cap, 0.737 s, 2.75 s and 7.59 s.
    let mut w = ws_base();
    w.practice = practice_heals(719_400_000, 750_000_000, 5, 2_590_000_000);
    record(
        &mut w,
        719_400_000,
        [737_000_000, 2_750_000_000, 7_590_000_000],
    );
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Pass, "s = 1.10: {:?}", part.why);
    for (prefix, start) in [
        (
            "WebSocket: half",
            "<= 737000000 (X 670000000 x f 1.1 = 737000000",
        ),
        (
            "WebSocket: at most",
            "<= 5 (Y 2500000000 x f 1.1 = 2750000000",
        ),
        (
            "WebSocket: no client-burst",
            "<= 7590000000 (Z 6900000000 x f 1.1 = 7590000000",
        ),
    ] {
        let c = needs(&part, prefix);
        assert!(c.starts_with(start), "s = 1.10: {c}");
    }
    // The three refusals, each judged and collected, so a plant that loosens all three shows each by name.
    // s = 1.1001 (T1b): just past the cap, its record's moved numbers the judge's at that f: INVALID.
    let mut t1b = ws_base();
    t1b.practice = practice_heals(719_465_400, 750_000_000, 5, 2_590_000_000);
    record(
        &mut t1b,
        719_465_400,
        [738_000_000, 2_751_000_000, 7_591_000_000],
    );
    // IN TOLERANCE at s = 0.95, below band_low 0.965 (T2): INVALID, though the counted run passes at the bound.
    let mut t2 = ws_base();
    t2.practice = practice_heals(621_300_000, 650_000_000, 0, 0);
    t2.counted = heals(660_000_000, 5, 2_550_000_000);
    record(
        &mut t2,
        621_300_000,
        [670_000_000, 2_500_000_000, 6_900_000_000],
    );
    // One practice heal over Z' = 7.175 s only (T3; 1 over Y', within k): IN TOLERANCE does not hold.
    let mut t3 = ws_base();
    t3.practice = heals(680_000_000, 1, 7_200_000_000);
    let mut wrong = Vec::new();
    for (case, w, expected) in [
        ("T1b", &t1b, "is \"IN TOLERANCE\" at s = 1.1001"),
        ("T2", &t2, "is \"IN TOLERANCE\" at s = 0.95"),
        (
            "T3",
            &t3,
            "has 1 heals over 2600000000 ns and its slowest is 7200000000 ns",
        ),
    ] {
        let (part, _) = ws_part(w);
        if part.verdict != VerdictKind::Invalid || !says(&part.why, expected) {
            wrong.push(format!("{case}: {:?} {:?}", part.verdict, part.why));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A copy of the scenario's practice run (every log of run ws-practice) under another id, started at `started`, its
/// record given `extra`.
fn another_practice(logs: &[Log], id: &str, started: &str, extra: &[(&str, Value)]) -> Vec<Log> {
    let mut out: Vec<Log> = logs
        .iter()
        .filter(|l| l.header.run == "ws-practice")
        .cloned()
        .collect();
    for p in &mut out {
        p.header.run = id.to_owned();
        if p.header.params.contains_key("started_utc") {
            p.header
                .params
                .insert("started_utc".to_owned(), json!(started));
        }
        for e in p.events.iter_mut().filter(|e| e.ev == "ws_practice") {
            for (k, v) in extra {
                e.f.insert((*k).to_owned(), v.clone());
            }
        }
    }
    out
}

/// The advisor's R9 ("a counted WebSocket run that fails stands as FAIL") and FX-2: a practice run started after the
/// counted run (the test-engineer's probes E1 and E2) is no candidate, so it never turns the FAIL into INVALID.
#[test]
fn u4j_ws_a_practice_run_after_the_counted_run_changes_nothing() {
    let mut w = ws_base();
    w.counted = heals(700_000_000, 6, 2_550_000_000);
    let logs = ws_logs(&w);
    let (part, _) = ws_judge(&logs);
    assert_eq!(part.verdict, VerdictKind::Fail, "E0: {:?}", part.why);
    assert!(
        says(
            &part.why,
            "WebSocket: half the client-bursts heal within X x f"
        ),
        "E0: {:?}",
        part.why
    );
    for (case, extra) in [
        (
            "E1",
            vec![(
                "second_ruling",
                json!("advisor-r3.md:40: the cause found and fixed"),
            )],
        ),
        ("E2", vec![]),
    ] {
        let mut later = logs.clone();
        later.extend(another_practice(
            &logs,
            "ws-practice-2",
            "2026-11-20T23:00:00Z",
            &extra,
        ));
        let (part, _) = ws_judge(&later);
        assert_eq!(part.verdict, VerdictKind::Fail, "{case}: {:?}", part.why);
        assert!(
            says(
                &part.why,
                "WebSocket: half the client-bursts heal within X x f"
            ),
            "{case}: {:?}",
            part.why
        );
    }
}

/// The advisor's R7 ("one practice run per build") over every run the judge reads (FX-3 (a), (b); the test-engineer's
/// probe F): a second practice run of the build in another set counts, listing the mark or not, so with no
/// second_ruling the run is INVALID; the latest must list the mark; a practice run in another set alone is in force.
#[test]
fn u4j_ws_one_practice_run_per_build_over_every_set() {
    let logs = ws_logs(&ws_base());
    let elsewhere = |listed: bool| {
        let mut p = another_practice(
            &logs,
            "ws-practice-0",
            "2026-11-20T17:00:00Z",
            &[("class", json!("BEYOND"))],
        );
        edit_run(&mut p, "ws-practice-0", &|l| {
            l.header.set = "set-ws-0".to_owned();
            if !listed {
                l.header.marks.clear();
            }
        });
        p
    };
    for listed in [true, false] {
        let mut with = logs.clone();
        with.extend(elsewhere(listed));
        let (part, _) = ws_judge(&with);
        assert_eq!(
            part.verdict,
            VerdictKind::Invalid,
            "listed {listed}: {:?}",
            part.why
        );
        assert!(
            says(&part.why, "2 practice runs of one build"),
            "listed {listed}: {:?}",
            part.why
        );
    }
    // The practice run, in another set, is the only one of the build: it is in force.
    let mut moved = logs.clone();
    edit_run(&mut moved, "ws-practice", &|l| {
        l.header.set = "set-ws-0".to_owned();
    });
    let (part, _) = ws_judge(&moved);
    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
    // The latest practice run of the build does not list the mark: INVALID.
    let mut unlisted = logs.clone();
    edit_run(&mut unlisted, "ws-practice", &|l| l.header.marks.clear());
    let (part, _) = ws_judge(&unlisted);
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    assert!(
        says(&part.why, "does not list PM-0001-18"),
        "{:?}",
        part.why
    );
}

/// The advisor's FX-6 (the test-engineer's probe H): M_m, the band, the guard, the model and the commit come from
/// marks.json's `ws_bound_line`, never from a log. A record whose bound line says M_m 620 ms (its moved numbers
/// recomputed to match) would let a 730 ms median pass at 735 ms; a record whose line differs only in its commit is
/// refused too.
#[test]
fn u4j_ws_the_record_carries_the_marks_bound_line() {
    let mut w = ws_base();
    let mut counted = vec![600_000_000; 239];
    counted.push(730_000_000);
    counted.extend(std::iter::repeat_n(750_000_000, 234));
    counted.extend(std::iter::repeat_n(2_550_000_000, 6));
    w.counted = counted;
    let (part, _) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Fail, "H0: {:?}", part.why);
    let mut h1 = w.clone();
    for (name, value) in [
        (
            "bound",
            json!(WS_BOUND.replacen(r#""m_m_ns":654000000"#, r#""m_m_ns":620000000"#, 1)),
        ),
        ("x_moved_ns", json!(735_000_000)),
        ("y_moved_ns", json!(2_742_000_000_i64)),
        ("z_moved_ns", json!(7_568_000_000_i64)),
    ] {
        set_field(&mut h1, name, value);
    }
    let (part, _) = ws_part(&h1);
    assert_eq!(part.verdict, VerdictKind::Invalid, "H1: {:?}", part.why);
    assert!(
        says(&part.why, "is not PM-0001-18's ws_bound_line"),
        "H1: {:?}",
        part.why
    );
    let mut commit = ws_base();
    set_field(
        &mut commit,
        "bound",
        json!(WS_BOUND.replacen(r#""commit":"bf9d248"#, r#""commit":"ff9d248"#, 1)),
    );
    let (part, _) = ws_part(&commit);
    assert_eq!(part.verdict, VerdictKind::Invalid, "commit: {:?}", part.why);
    assert!(
        says(&part.why, "is not PM-0001-18's ws_bound_line"),
        "commit: {:?}",
        part.why
    );
}

/// The ws-budget-x2 bite run of -18's WebSocket part: its run outcome.
fn ws_bite(v: &npt_judge::Verdict) -> &npt_judge::eval::BiteOut {
    v.bites
        .iter()
        .find(|b| b.id == "PM-0001-18/ws-budget-x2")
        .expect("the WebSocket bite")
}

/// FX-3 (c) and TE-F15: a bite run takes the practice run in force for its set's measured runs that serve the mark
/// with its layer, profile and transport (one build among them), so it is judged at its counted run's f.
#[test]
fn u4j_ws_a_bite_run_takes_its_counted_runs_practice_run() {
    let logs = ws_logs(&ws_base());
    let check_c = |r: &npt_judge::eval::RunOut| {
        r.checks
            .iter()
            .find(|c| c.name.starts_with("WebSocket: half"))
            .map(|c| c.needs.clone())
            .expect("check c")
    };
    // (1) The same f as the measured run's.
    let (part, v) = ws_judge(&logs);
    let bite = ws_bite(&v);
    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
    assert!(check_c(&bite.runs[0]).contains("x f 1.039755 = 697000000"));
    // (2) The set's measured runs come from two builds: the bite cannot tell whose numbers are in force.
    let mut two = logs.clone();
    let mut other: Vec<Log> = logs
        .iter()
        .filter(|l| l.header.run == "ws-1")
        .cloned()
        .collect();
    edit_run(&mut other, "ws-1", &|l| {
        l.header.run = "ws-2".to_owned();
        l.header.build.binary_sha256 = "dd".to_owned();
    });
    two.extend(other);
    let (_, v) = ws_judge(&two);
    let bite = ws_bite(&v);
    assert_eq!(bite.status, BiteStatus::Invalid, "{:?}", bite.runs);
    assert!(
        says(&bite.runs[0].invalid_why, "come from 2 builds"),
        "{:?}",
        bite.runs[0].invalid_why
    );
    // (3) Practice runs of two builds in the set, measured runs of one: the measured build's practice run.
    let mut practices = logs.clone();
    let mut p = another_practice(&logs, "ws-practice-dd", "2026-11-20T17:00:00Z", &[]);
    edit_run(&mut p, "ws-practice-dd", &|l| {
        l.header.build.binary_sha256 = "dd".to_owned();
    });
    practices.extend(p);
    let (_, v) = ws_judge(&practices);
    let bite = ws_bite(&v);
    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
    // (4) A practice run of an older build in another set changes nothing.
    let mut older = logs.clone();
    let mut p = another_practice(&logs, "ws-practice-ee", "2026-11-19T17:00:00Z", &[]);
    edit_run(&mut p, "ws-practice-ee", &|l| {
        l.header.build.binary_sha256 = "ee".to_owned();
        l.header.set = "set-old".to_owned();
    });
    older.extend(p);
    let (_, v) = ws_judge(&older);
    let bite = ws_bite(&v);
    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
    // (5) Two measured runs of one build with different practice runs in force (TE2-3): the second started after a
    // second practice run of the build, which holds a second ruling. The bite cannot tell whose numbers hold.
    let ruling = json!("advisor-r3.md:40: the cause found and fixed");
    let mut split = logs.clone();
    split.extend(another_practice(
        &logs,
        "ws-practice-2",
        "2026-11-20T23:00:00Z",
        &[("second_ruling", ruling)],
    ));
    let mut second: Vec<Log> = logs
        .iter()
        .filter(|l| l.header.run == "ws-1")
        .cloned()
        .collect();
    edit_run(&mut second, "ws-1", &|l| {
        l.header.run = "ws-2".to_owned();
        if let Some(started) = l.header.params.get_mut("started_utc") {
            *started = json!("2026-11-20T23:30:00Z");
        }
    });
    split.extend(second);
    let (_, v) = ws_judge(&split);
    let bite = ws_bite(&v);
    assert_eq!(bite.status, BiteStatus::Invalid, "{:?}", bite.runs);
    assert!(
        says(
            &bite.runs[0].invalid_why,
            "have practice runs ws-practice and ws-practice-2 in force"
        ),
        "{:?}",
        bite.runs[0].invalid_why
    );
}

// ---------------------------------------------------------------------------------------------------------------
// MOVED (README §2.6): a mark, part, check or bite that left this test is never judged and never reads PASS.

fn moved() -> Value {
    json!({"to": "Slice 3", "why": "the human run left", "by": "NPT-TRIM"})
}

/// The tick-budget mark with two parts: "S30" (profile P2, three runs) and "P3" (profile P3, one run), given here.
fn two_parts(p3_extra: Value, extra: Value) -> Value {
    let mut p3 = json!({"name": "P3", "select": {"profile": "P3"}, "min": 1});
    if let (Some(p), Some(e)) = (p3.as_object_mut(), p3_extra.as_object()) {
        p.extend(e.clone());
    }
    let mut m = tick_mark(
        M,
        json!({"bites": [], "parts": [{"name": "S30", "select": {"profile": "P2"}, "min": 3}, p3]}),
    );
    if let (Some(m), Some(e)) = (m.as_object_mut(), extra.as_object()) {
        m.extend(e.clone());
    }
    m
}

/// One measured run at this profile with these work times; an empty list gives a run with no tick record (a gap).
fn profile_run(id: &'static str, profile: &'static str, work: &[i64]) -> Log {
    let mut s = LogSpec::room(id, "set-a", M).ticks("work_ns", work);
    s.profile = profile;
    s.build()
}

/// A bite run of the B-S bite at this profile.
fn profile_bite(profile: &'static str, work: &[i64]) -> Log {
    let mut s = LogSpec::room("bite-1", "set-a", M).ticks("work_ns", work);
    s.role = "bite";
    s.bite = Some("PM-0001-4/B-S");
    s.profile = profile;
    s.build()
}

#[test]
fn a_mark_moved_whole_is_never_judged() {
    let mark = tick_mark(M, json!({"moved": moved()}));
    // Passing runs with a shown bite, failing runs, and no run at all: MOVED each time, with no part or bite judged.
    let mut passing = three_runs(M, [90, 95, 100]);
    passing.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[10, 150]));
    for (logs, listed) in [
        (passing, 4),
        (three_runs(M, [90, 101, 100]), 3),
        (vec![], 0),
    ] {
        let v = judge(&marks_file(std::slice::from_ref(&mark)), &logs);
        assert_eq!(v[0].verdict, VerdictKind::Moved, "{:?}", v[0].why);
        assert_eq!(
            v[0].why,
            [format!(
                "moved to Slice 3: the human run left (NPT-TRIM); {listed} run(s) here list it: counted, not judged"
            )]
        );
        assert!(v[0].parts.is_empty() && v[0].bites.is_empty());
        assert_eq!(v[0].moved.as_ref().map(|m| m.to.as_str()), Some("Slice 3"));
    }
    // The same mark kept: the failing runs fail it, so the label is what stops the judging.
    let (k, _) = verdict_of(
        &[tick_mark(M, json!({"bites": []}))],
        &three_runs(M, [90, 101, 100]),
    );
    assert_eq!(k, VerdictKind::Fail);
    // A person's mark moved whole: a recorded "fail" is not read.
    let person = json!({"id": "PM-0001-53", "title": "looks correct", "draft": draft(), "verdict": "person",
                        "moved": moved()});
    let mut s = LogSpec::room("mp-1", "set-mp", "PM-0001-53");
    s.events = vec![
        EventRecord::new("person", Phase::Measured)
            .with("who", "Mustafa")
            .with("mark", "PM-0001-53")
            .with("verdict", "fail"),
    ];
    assert_eq!(verdict_of(&[person], &[s.build()]).0, VerdictKind::Moved);
}

#[test]
fn a_moved_part_is_never_judged_and_the_kept_parts_still_decide() {
    let mark = two_parts(json!({"moved": moved()}), json!({}));
    let s30 = |worst: [i64; 3]| -> Vec<Log> {
        vec![
            profile_run("run-1", "P2", &[10, 20, worst[0]]),
            profile_run("run-2", "P2", &[10, 20, worst[1]]),
            profile_run("run-3", "P2", &[10, 20, worst[2]]),
        ]
    };
    // The kept part passes: MOVED, whatever the moved part's runs hold (none, a failure, a gap).
    for p3 in [None, Some(vec![500]), Some(vec![])] {
        let mut logs = s30([90, 95, 100]);
        logs.extend(p3.map(|work| profile_run("p3-1", "P3", &work)));
        let v = judge(&marks_file(std::slice::from_ref(&mark)), &logs);
        assert_eq!(v[0].verdict, VerdictKind::Moved, "{:?}", v[0].why);
        assert_eq!(
            v[0].why[0],
            "the kept parts hold; moved, never judged here: part \"P3\" to Slice 3"
        );
        assert_eq!(v[0].parts[0].verdict, VerdictKind::Pass);
        let p3 = &v[0].parts[1];
        assert_eq!(p3.verdict, VerdictKind::Moved);
        assert!(p3.runs.is_empty(), "a moved part's runs are never judged");
        assert_eq!(
            p3.why,
            ["P3: moved to Slice 3: the human run left (NPT-TRIM)"]
        );
    }
    // The kept part fails, has a gap, or lacks a run: FAIL, INVALID and NOT-RUN still show.
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &s30([90, 101, 100]));
    assert_eq!(k, VerdictKind::Fail);
    assert_eq!(why, ["S30: run run-2: tick p99 = 101, needs <= 100"]);
    let mut gap = s30([90, 95, 100]);
    gap[1] = profile_run("run-2", "P2", &[]);
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &gap);
    assert_eq!(k, VerdictKind::Invalid);
    assert!(why[0].contains("run run-2: a gap in the record"), "{why:?}");
    let two: Vec<Log> = s30([90, 95, 100]).into_iter().take(2).collect();
    let (k, why) = verdict_of(std::slice::from_ref(&mark), &two);
    assert_eq!(k, VerdictKind::NotRun);
    assert_eq!(why, ["S30: 2 of 3 valid runs"]);
    // The same mark with "P3" kept and unrun: NOT-RUN, so the label is what makes it MOVED.
    let (k, why) = verdict_of(&[two_parts(json!({}), json!({}))], &s30([90, 95, 100]));
    assert_eq!(k, VerdictKind::NotRun);
    assert_eq!(why, ["P3: 0 of 1 valid runs"]);
}

/// A moved check is skipped in each place checks are chosen (a part's plain checks, its pooled checks, its setup
/// checks, a bite run's checks): beside each, the same check kept, which decides the mark.
#[test]
fn a_moved_check_is_skipped_wherever_checks_are_chosen() {
    let kept = json!({"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100});
    let with = |check: Value, bites: Value| {
        tick_mark(M, json!({"checks": [kept.clone(), check], "bites": bites}))
    };
    let label = |mut check: Value| {
        check["moved"] = moved();
        check
    };
    let runs = three_runs(M, [90, 95, 100]);
    let cases = [
        (
            "a plain check that fails",
            json!({"name": "max", "series": "tick.v.work_ns", "stat": "max", "op": "<=", "value": 50}),
            VerdictKind::Fail,
        ),
        (
            "a plain check with a gap",
            json!({"name": "absent", "series": "tick.v.absent_ns", "stat": "max", "op": "<=", "value": 50}),
            VerdictKind::Invalid,
        ),
        (
            "a pooled check that fails",
            json!({"name": "pooled max", "series": "tick.v.work_ns", "stat": "max", "op": "<=", "value": 50,
                   "pooled": true}),
            VerdictKind::Fail,
        ),
        (
            "a setup check that fails",
            json!({"name": "setup", "series": "tick.v.work_ns", "stat": "min", "op": ">=", "value": 1000,
                   "setup": true}),
            VerdictKind::Invalid,
        ),
    ];
    for (what, check, kept_verdict) in cases {
        let (k, why) = verdict_of(&[with(check.clone(), json!([]))], &runs);
        assert_eq!(k, kept_verdict, "{what}, kept: {why:?}");
        let name = check["name"].as_str().expect("a name").to_owned();
        let v = judge(&marks_file(&[with(label(check), json!([]))]), &runs);
        assert_eq!(v[0].verdict, VerdictKind::Moved, "{what}");
        let line =
            format!("the kept parts hold; moved, never judged here: check {name:?} to Slice 3");
        assert_eq!(v[0].why, [line], "{what}");
        let json = serde_json::to_value(&v[0]).expect("json");
        assert_eq!(
            json["moved_checks"],
            json!([{"name": name, "moved": moved()}])
        );
        let judged = v[0].parts[0]
            .runs
            .iter()
            .flat_map(|r| &r.checks)
            .chain(&v[0].parts[0].pooled);
        assert!(
            judged.clone().count() == 3 && judged.clone().all(|c| c.name == "tick p99"),
            "{what}"
        );
    }
    // A bite run: it passes the kept check and fails only the moved one, so it did not show the bite. Kept, the
    // same check is what the bite fails.
    let bites = json!([{"id": "PM-0001-4/B-S", "what": "a busy-wait"}]);
    let max =
        json!({"name": "max", "series": "tick.v.work_ns", "stat": "max", "op": "<=", "value": 50});
    let mut logs = three_runs(M, [40, 45, 50]);
    logs.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[10, 60]));
    let (k, why) = verdict_of(&[with(max.clone(), bites.clone())], &logs);
    assert_eq!(k, VerdictKind::Pass, "{why:?}");
    let (k, why) = verdict_of(&[with(label(max.clone()), bites.clone())], &logs);
    assert_eq!(k, VerdictKind::Fail);
    assert_eq!(
        why,
        ["bite PM-0001-4/B-S did not fail: the check cannot be shown to work"]
    );
    // The same on a bite run that matches no part (profile P4), which is judged on the mark-wide checks alone.
    let mark = two_parts(
        json!({}),
        json!({"checks": [kept, label(max)], "bites": bites}),
    );
    let logs = [
        profile_run("run-1", "P2", &[10, 20, 40]),
        profile_run("run-2", "P2", &[10, 20, 45]),
        profile_run("run-3", "P2", &[10, 20, 50]),
        profile_run("p3-1", "P3", &[10]),
        profile_bite("P4", &[10, 60]),
    ];
    let v = judge(&marks_file(&[mark]), &logs);
    assert_eq!(v[0].bites[0].status, BiteStatus::Missed);
    assert_eq!(v[0].verdict, VerdictKind::Fail);
}

#[test]
fn a_moved_bite_is_never_evaluated() {
    let mark = tick_mark(
        M,
        json!({"bites": [{"id": "PM-0001-4/B-S", "moved": moved(), "what": "a busy-wait"}]}),
    );
    // With no run of it (NOT-RUN if kept) and with a run that passes every check (FAIL if kept): MOVED.
    let mut missed = three_runs(M, [90, 95, 100]);
    missed.push(bite_run(M, "PM-0001-4/B-S", "set-a", &[10, 20]));
    for logs in [three_runs(M, [90, 95, 100]), missed] {
        let v = judge(&marks_file(std::slice::from_ref(&mark)), &logs);
        assert_eq!(v[0].verdict, VerdictKind::Moved, "{:?}", v[0].why);
        assert_eq!(
            v[0].why,
            ["the kept parts hold; moved, never judged here: bite PM-0001-4/B-S to Slice 3"]
        );
        assert_eq!(v[0].bites[0].status, BiteStatus::Moved);
        assert!(v[0].bites[0].runs.is_empty());
        let json = serde_json::to_value(&v[0].bites[0]).expect("json");
        assert_eq!(
            json,
            json!({"id": "PM-0001-4/B-S", "status": "moved", "runs": [], "moved": moved()})
        );
    }
    // A kept failure still shows past a moved bite.
    let (k, _) = verdict_of(&[mark], &three_runs(M, [90, 101, 100]));
    assert_eq!(k, VerdictKind::Fail);
}

/// A bite run that matches only moved parts is left out (it is not a run of this test); one that matches no part
/// keeps today's rule and is judged on the mark-wide checks.
#[test]
fn a_bite_run_on_moved_parts_only_is_left_out() {
    let mark = two_parts(
        json!({"moved": moved()}),
        json!({"bites": [{"id": "PM-0001-4/B-S", "what": "a busy-wait"}]}),
    );
    let s30 = || {
        vec![
            profile_run("run-1", "P2", &[10, 20, 90]),
            profile_run("run-2", "P2", &[10, 20, 95]),
            profile_run("run-3", "P2", &[10, 20, 100]),
        ]
    };
    let with_bite = |profile: &'static str, work: &[i64]| {
        let mut logs = s30();
        logs.push(profile_bite(profile, work));
        judge(&marks_file(std::slice::from_ref(&mark)), &logs).remove(0)
    };
    // On the moved part only: whether it would have shown the bite (150) or missed it (20), it is no run of the bite.
    for work in [[10, 150], [10, 20]] {
        let v = with_bite("P3", &work);
        assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
        assert_eq!(v.why, ["bite PM-0001-4/B-S: no run yet"]);
        assert_eq!(v.bites[0].status, BiteStatus::NotRun);
        assert!(v.bites[0].runs.is_empty());
    }
    // Matching no part (profile P4): judged on the mark-wide check, shown or missed as before.
    let v = with_bite("P4", &[10, 150]);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "{:?}",
        v.bites[0].runs
    );
    assert_eq!(v.verdict, VerdictKind::Moved, "{:?}", v.why);
    let v = with_bite("P4", &[10, 20]);
    assert_eq!(v.bites[0].status, BiteStatus::Missed);
    assert_eq!(v.verdict, VerdictKind::Fail);
    // On the kept part: judged with that part's rules, as before.
    let v = with_bite("P2", &[10, 150]);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "{:?}",
        v.bites[0].runs
    );
    // On a kept part and a moved one at once: only the kept part's checks apply. The moved part's own check
    // (max at most 50) would have shown the bite; the kept check (p99 at most 100) lets it through.
    let l2 = json!({"name": "L2", "moved": moved(), "select": {"layer": "L2"}, "min": 1});
    let max = json!({"name": "max", "series": "tick.v.work_ns", "stat": "max", "op": "<=", "value": 50,
                     "parts": ["L2"]});
    let kept = json!({"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100});
    let s30_part = json!({"name": "S30", "select": {"profile": "P2"}, "min": 3});
    let mixed = tick_mark(M, json!({"parts": [s30_part, l2], "checks": [kept, max]}));
    let mut logs = s30();
    logs.push(profile_bite("P2", &[10, 60]));
    let v = judge(&marks_file(&[mixed]), &logs).remove(0);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Missed,
        "{:?}",
        v.bites[0].runs
    );
    assert_eq!(
        v.bites[0].runs[0].checks.len(),
        1,
        "the moved part's check is not read"
    );
}

/// MOVED comes before REPORTED and STAND-IN: neither a reported mark nor a stand-in pass hides a moved part.
#[test]
fn moved_outranks_reported_and_stand_in() {
    let stand_in = |p3: Value| two_parts(p3, json!({"stand_in": true}));
    let mut logs = vec![
        profile_run("run-1", "P2", &[10, 20, 90]),
        profile_run("run-2", "P2", &[10, 20, 95]),
        profile_run("run-3", "P2", &[10, 20, 100]),
        profile_run("p3-1", "P3", &[10]),
    ];
    let mut s = LogSpec::room("run-1", "set-a", M).ticks("work_ns", &[10]);
    s.params = vec![("sim", json!("stand-in"))];
    logs[0] = s.build();
    assert_eq!(
        verdict_of(&[stand_in(json!({}))], &logs).0,
        VerdictKind::StandIn
    );
    assert_eq!(
        verdict_of(&[stand_in(json!({"moved": moved()}))], &logs).0,
        VerdictKind::Moved
    );
    let mut density = density_mark();
    density["parts"] = json!([{"name": "N48", "select": {"clients": 48}, "min": 1},
                              {"name": "N36", "moved": moved(), "select": {"clients": 36}, "min": 1}]);
    let v = judge(&marks_file(&[density]), &[density_run(false)]);
    assert_eq!(v[0].verdict, VerdictKind::Moved, "{:?}", v[0].why);
    assert_eq!(
        v[0].report[0].value, "8330004",
        "the kept part is still reported"
    );
}

// ---------------------------------------------------------------------------------------------------------------
// Part b (U6j): the figures of the network runs, on made-up logs. Most of their input series have no writer yet.

/// A mark like PM-0006-20: the tick thread's allocation count with logging on equals its logging-off twin's.
fn alloc_mark() -> Value {
    json!({
        "id": "PM-0006-20", "title": "logging allocates nothing", "verdict": "pass",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "logging on", "select": {"params.suite": "logging-alloc"}, "min": 1}],
        "checks": [{"name": "allocs equal", "figure": "alloc_twin_diff_count", "op": "==", "value": 0}],
        "bites": [{"id": "PM-0006-20/planted-format", "what": "a format! on the tick thread"}]
    })
}

/// One room log of an allocation run: its role, the twin it names, whether it runs the planted-format bite, and its
/// ticks in order, each with its phase and its allocation count (`None`: the record has no count).
fn alloc_run(
    id: &'static str,
    role: &'static str,
    twin: Option<&'static str>,
    bite: bool,
    ticks: &[(Phase, Option<i64>)],
) -> Log {
    let mut s = LogSpec::room(id, "set-alloc", "PM-0006-20");
    s.role = role;
    s.bite = bite.then_some("PM-0006-20/planted-format");
    s.params.push(("suite", json!("logging-alloc")));
    s.params.extend(twin.map(|t| ("twin", json!(t))));
    s.ticks = ticks
        .iter()
        .map(|(ph, n)| {
            (
                *ph,
                n.map(|n| ("tick_allocs_count", n)).into_iter().collect(),
            )
        })
        .collect();
    s.build()
}

/// Measured ticks 1, 2, … with these allocation counts.
fn measured(counts: &[i64]) -> Vec<(Phase, Option<i64>)> {
    counts.iter().map(|n| (Measured, Some(*n))).collect()
}

#[test]
fn alloc_twin_diff_count_compares_the_two_runs_tick_by_tick() {
    let on =
        |counts: &[i64]| alloc_run("on-1", "measured", Some("off-1"), false, &measured(counts));
    let off = |role: &'static str, counts: &[i64]| {
        alloc_run("off-1", role, None, false, &measured(counts))
    };
    // The bite's own pair is in every case: its count differs from its logging-off twin's on every tick (shown).
    let bite = alloc_run(
        "bite-1",
        "bite",
        Some("bite-off"),
        true,
        &measured(&[4, 4, 4]),
    );
    let bite_off = alloc_run("bite-off", "logging-off", None, true, &measured(&[3, 3, 3]));
    let judged = |on_log: Log, off_log: Option<Log>| {
        let mut logs = vec![on_log, bite.clone(), bite_off.clone()];
        logs.extend(off_log);
        judge(&marks_file(&[alloc_mark()]), &logs).remove(0)
    };
    // Pass: equal at every measured tick.
    let v = judged(on(&[3, 3, 3]), Some(off("logging-off", &[3, 3, 3])));
    assert_eq!(v.verdict, VerdictKind::Pass, "{:?}", v.why);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "{:?}",
        v.bites[0].runs
    );
    // Fail: one tick differs; a tick that only one of the two runs has counts too.
    let fail = ["logging on: run on-1: allocs equal = 1, needs == 0"];
    let v = judged(on(&[3, 4, 3]), Some(off("logging-off", &[3, 3, 3])));
    assert_eq!(
        (v.verdict, v.why.as_slice()),
        (VerdictKind::Fail, &fail.map(String::from)[..])
    );
    let v = judged(on(&[3, 3, 3]), Some(off("logging-off", &[3, 3])));
    assert_eq!(
        (v.verdict, v.why.as_slice()),
        (VerdictKind::Fail, &fail.map(String::from)[..])
    );
    // Not run yet: the twin is not in the set.
    let v = judged(on(&[3, 3, 3]), None);
    assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
    assert!(
        says(&v.why, "its twin off-1 is not in set set-alloc yet"),
        "{:?}",
        v.why
    );
    // Gaps: a twin of another role; a measured tick record without the count.
    let v = judged(on(&[3, 3, 3]), Some(off("twin", &[3, 3, 3])));
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(&v.why, "off-1 has role twin, not logging-off"),
        "{:?}",
        v.why
    );
    let holed = [(Measured, Some(3)), (Measured, None)];
    let v = judged(
        alloc_run("on-1", "measured", Some("off-1"), false, &holed),
        Some(off("logging-off", &[3, 3])),
    );
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(&v.why, "measured tick 2 has no tick.v.tick_allocs_count"),
        "{:?}",
        v.why
    );
    // An empty comparison fails: neither run has a measured tick.
    let warm = [(Warmup, Some(3))];
    let v = judged(
        alloc_run("on-1", "measured", Some("off-1"), false, &warm),
        Some(alloc_run("off-1", "logging-off", None, false, &warm)),
    );
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    assert!(
        says(&v.why, "have no measured tick; an empty comparison fails"),
        "{:?}",
        v.why
    );
}

/// C-U6-10: the planted-format bite run's logging-off twin runs the same bite on the bites build. The build rule
/// refuses a logging-off run that names a bite everywhere else; `twin_of` lets it through for a bite run alone.
#[test]
fn a_bite_runs_logging_off_twin_may_name_its_bite() {
    let on = alloc_run(
        "on-1",
        "measured",
        Some("off-1"),
        false,
        &measured(&[3, 3, 3]),
    );
    let off = alloc_run("off-1", "logging-off", None, false, &measured(&[3, 3, 3]));
    let bite = alloc_run(
        "bite-1",
        "bite",
        Some("bite-off"),
        true,
        &measured(&[4, 4, 4]),
    );
    let judged = |mark: Value, logs: Vec<Log>| judge(&marks_file(&[mark]), &logs).remove(0);
    // The pair as it runs: the bite is shown.
    let bite_off = alloc_run("bite-off", "logging-off", None, true, &measured(&[3, 3, 3]));
    let v = judged(
        alloc_mark(),
        vec![on.clone(), off.clone(), bite.clone(), bite_off],
    );
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "{:?}",
        v.bites[0].runs
    );
    // The same twin as a measured run: a measured run never names a bite, so the bite run's comparison has a gap.
    let as_measured = alloc_run("bite-off", "measured", None, true, &measured(&[3, 3, 3]));
    let v = judged(alloc_mark(), vec![on.clone(), off, bite, as_measured]);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Invalid,
        "{:?}",
        v.bites[0].runs
    );
    let why = &v.bites[0].runs[0].invalid_why;
    assert!(
        says(
            why,
            "its twin bite-off is invalid: build: a measured run names the bite"
        ),
        "{why:?}"
    );
    // A measured run whose logging-off twin names a bite: refused as before.
    let mut no_bites = alloc_mark();
    no_bites["bites"] = json!([]);
    let off_bitten = alloc_run("off-1", "logging-off", None, true, &measured(&[3, 3, 3]));
    let v = judged(no_bites, vec![on, off_bitten]);
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(
            &v.why,
            "its twin off-1 is invalid: build: a logging-off run names the bite"
        ),
        "{:?}",
        v.why
    );
}

// U6j's figures (part b): each on made-up logs, with a pass, a fail and a gap case. No writer logs these series yet.

/// Run f1 (L2) under a mark with one check of `figure` (`op` `value`): the check as "Outcome value" (`-`: no
/// value), and its note.
fn figure_check(figure: &str, op: &str, value: Value, logs: &[Log]) -> (String, String) {
    let mark = json!({
        "id": M, "title": "a figure", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "P", "select": {"layer": "L2"}, "min": 1}],
        "checks": [{"name": "c", "figure": figure, "op": op, "value": value}],
        "bites": []
    });
    let v = judge(&marks_file(&[mark]), logs);
    let c = &v[0].parts[0].runs[0].checks[0];
    let value = c.value.as_deref().unwrap_or("-");
    (
        format!("{:?} {value}", c.outcome),
        c.note.clone().unwrap_or_default(),
    )
}

/// One log of run f1 with 2 clients: `process` on machine role `machine`, its second records in order from 0.
fn seconds_log(process: &'static str, machine: &'static str, seconds: Vec<SecondSpec>) -> Log {
    let mut s = LogSpec::room("f1", "set-f", M);
    (s.process, s.machine_role, s.clients, s.seconds) = (process, machine, 2, seconds);
    s.build()
}

/// A second record of `phase` with list series only.
fn lists(phase: Phase, lists: &[(&'static str, [i64; 2])]) -> SecondSpec {
    let lists = lists.iter().map(|(k, v)| (*k, v.to_vec())).collect();
    (phase, vec![], lists)
}

#[test]
fn u6j_the_largest_client_mean_of_the_downstream_and_the_upstream() {
    // Per measured second: slot 0, slot 1. A warm-up second is not read. The other direction differs on purpose.
    let run = |name: &'static str, other: &'static str, slot1: [i64; 3]| {
        let mut seconds = vec![lists(
            Warmup,
            &[(name, [900_000, 900_000]), (other, [1, 1])],
        )];
        seconds
            .extend(slot1.map(|b| lists(Measured, &[(name, [159_999, b]), (other, [700_000, 1])])));
        vec![seconds_log("room", "RS", seconds)]
    };
    let down = |logs: &[Log]| figure_check("down_client_mean_max_bits", "<=", json!(160_000), logs);
    assert_eq!(
        down(&run("down_bits", "up_bits", [160_000; 3])).0,
        "Pass 160000"
    );
    // Slot 1's 480,003 bits over 3 s: 160,001, though the mean over both clients is 160,000.
    assert_eq!(
        down(&run("down_bits", "up_bits", [160_001; 3])).0,
        "Fail 160001"
    );
    assert_eq!(
        down(&run("down_bits", "up_bits", [160_000, 160_000, 160_001])).0,
        "Fail 160000.333333"
    );
    let up = |logs: &[Log]| figure_check("up_client_mean_max_bits", "<=", json!(160_000), logs);
    assert_eq!(
        up(&run("up_bits", "down_bits", [160_001; 3])).0,
        "Fail 160001"
    );
    assert_eq!(up(&run("up_bits", "down_bits", [1; 3])).0, "Pass 159999");
    // Gaps: a measured second without the series; a list that does not hold the run's clients; the name in a log
    // of another process; no room log; no measured second.
    let mut gaps: Vec<(Vec<Log>, &str)> = vec![];
    let mut logs = run("down_bits", "up_bits", [1; 3]);
    logs[0].seconds[2].s.remove("down_bits");
    gaps.push((
        logs,
        "run f1: measured second 2 of the room log has no second.s.down_bits",
    ));
    let mut logs = run("down_bits", "up_bits", [1; 3]);
    logs[0].seconds[3]
        .s
        .insert("down_bits".into(), vec![1, 2, 3]);
    gaps.push((
        logs,
        "run f1: measured second 3's second.s.down_bits holds 3 entries, not 2, the run's clients (the room log's header)",
    ));
    let mut logs = run("down_bits", "up_bits", [1; 3]);
    logs.push(seconds_log(
        "bots",
        "LG",
        vec![lists(Measured, &[("down_bits", [1, 1])])],
    ));
    gaps.push((
        logs,
        "run f1: second.s.down_bits in the bots log on LG1: only process room writes it",
    ));
    let bots = seconds_log("bots", "LG", vec![lists(Measured, &[("x_count", [1, 1])])]);
    gaps.push((vec![bots], "run f1: no room log"));
    let warm = seconds_log("room", "RS", vec![lists(Warmup, &[("down_bits", [1, 1])])]);
    gaps.push((
        vec![warm],
        "run f1: no measured second in the room log (second.s.down_bits)",
    ));
    for (logs, why) in &gaps {
        assert_eq!(down(logs), ("Gap -".to_owned(), (*why).to_owned()));
    }
}

#[test]
fn u6j_the_ipv6_downstream_mean_adds_160_bits_per_datagram() {
    let second = |bits, datagrams| {
        lists(
            Measured,
            &[("down_bits", bits), ("down_datagrams_count", datagrams)],
        )
    };
    let ipv6 = |logs: &[Log]| figure_check("down_mean_ipv6_bits", "==", json!(1_320), logs);
    // 3 seconds of 1,000 bits and 2 datagrams per client: 1,000 + 2 x 160.
    let logs = vec![seconds_log(
        "room",
        "RS",
        vec![second([1_000, 1_000], [2, 2]); 3],
    )];
    assert_eq!(ipv6(&logs).0, "Pass 1320");
    // The mean over client-seconds, not the largest client's: (1,000 + 3,000 + 160 x 4) / 2 over each second.
    let logs = vec![seconds_log(
        "room",
        "RS",
        vec![second([1_000, 3_000], [1, 3]); 3],
    )];
    assert_eq!(ipv6(&logs).0, "Fail 2320");
    let mut logs = vec![seconds_log(
        "room",
        "RS",
        vec![second([1_000, 1_000], [2, 2]); 3],
    )];
    logs[0].seconds[1].s.remove("down_datagrams_count");
    let gap = "run f1: measured second 1 of the room log has no second.s.down_datagrams_count";
    assert_eq!(ipv6(&logs), ("Gap -".to_owned(), gap.to_owned()));
}

#[test]
fn u6j_the_control_streams_p99_rise_under_a_bulk_transfer() {
    const BULK: &str = "control_delay_bulk_ns";
    const IDLE: &str = "control_delay_idle_ns";
    // Two bots logs, pooled: the samples with a transfer in flight in one, those without in the other.
    let run = |slowest: i64| {
        let warm = lists(Warmup, &[(BULK, [90_000_000, 90_000_000])]);
        let bulk = lists(Measured, &[(BULK, [1_000_000, slowest])]);
        let idle = lists(Measured, &[(IDLE, [2_000_000, 1_000_000])]);
        vec![
            seconds_log("room", "RS", vec![lists(Measured, &[])]),
            seconds_log("bots", "LG", vec![warm, bulk]),
            seconds_log("bots", "LG", vec![idle]),
        ]
    };
    let rise =
        |logs: &[Log]| figure_check("bulk_control_p99_rise_ns", "<=", json!(5_000_000), logs);
    assert_eq!(rise(&run(7_000_000)).0, "Pass 5000000");
    assert_eq!(rise(&run(7_000_001)).0, "Fail 5000001");
    // A faster stream under the transfer is a negative rise, a real value.
    assert_eq!(rise(&run(1_500_000)).0, "Pass -500000");
    // No sample with a transfer in flight in the measured seconds: a gap, never a rise of 0.
    let mut logs = run(7_000_000);
    logs[1].seconds[1].s.remove(BULK);
    let gap = "run f1: no second.s.control_delay_bulk_ns sample in the measured seconds of LG's bots logs";
    assert_eq!(rise(&logs), ("Gap -".to_owned(), gap.to_owned()));
    let mut logs = run(7_000_000);
    logs[0].seconds[0].s.insert(IDLE.into(), vec![1]);
    let gap = "run f1: second.s.control_delay_idle_ns in the room log on RS1: only process bots writes it";
    assert_eq!(rise(&logs), ("Gap -".to_owned(), gap.to_owned()));
    assert_eq!(rise(&logs[..1]).1, "run f1: no bots log");
}

#[test]
fn u6j_one_shots_sent_less_those_received_over_the_whole_run() {
    const SENT: &str = "one_shots_sent_count";
    const GOT: &str = "one_shots_received_count";
    let counts = |name: &'static str, per_second: [(Phase, i64); 3]| -> Vec<SecondSpec> {
        per_second
            .map(|(ph, n)| (ph, vec![(name, n)], vec![]))
            .to_vec()
    };
    // Sent in the warm-up and in the measured phase; received a second later, the last after the measured phase.
    let run = |received: i64| {
        let sent = counts(SENT, [(Warmup, 4_000), (Measured, 6_000), (After, 0)]);
        let got = counts(GOT, [(Warmup, 0), (Measured, 4_000), (After, received)]);
        vec![
            seconds_log("room", "RS", got),
            seconds_log("bots", "LG", sent),
        ]
    };
    let dropped = |logs: &[Log]| figure_check("one_shot_dropped_count", "==", json!(0), logs);
    assert_eq!(dropped(&run(6_000)).0, "Pass 0");
    assert_eq!(dropped(&run(5_999)).0, "Fail 1");
    // More received than sent is not 0 either.
    assert_eq!(dropped(&run(6_001)).0, "Fail -1");
    let mut logs = run(6_000);
    logs[1].seconds[2].v.remove(SENT);
    let gap = "run f1: second 2 of the bots log on LG1 has no second.v.one_shots_sent_count";
    assert_eq!(dropped(&logs), ("Gap -".to_owned(), gap.to_owned()));
    let mut logs = run(6_000);
    logs[0].seconds.clear();
    let gap = "run f1: no second record in its room log (second.v.one_shots_received_count)";
    assert_eq!(dropped(&logs), ("Gap -".to_owned(), gap.to_owned()));
    // Two room logs in one run: which room's count?
    let mut logs = run(6_000);
    logs.push(logs[0].clone());
    assert_eq!(
        dropped(&logs),
        (
            "Gap -".to_owned(),
            "run f1: more than one room log".to_owned()
        )
    );
    let mut logs = run(6_000);
    logs[0].seconds[0].v.insert(SENT.into(), 1);
    let gap =
        "run f1: second.v.one_shots_sent_count in the room log on RS1: only process bots writes it";
    assert_eq!(dropped(&logs), ("Gap -".to_owned(), gap.to_owned()));
}

/// Run f1's room log with `rss(sec)` as `second.v.rss_bytes` on each of `n` measured seconds.
fn rss_run(n: u64, rss: impl Fn(i64) -> i64) -> Vec<Log> {
    let second = |sec| (Measured, vec![("rss_bytes", rss(sec))], vec![]);
    vec![seconds_log(
        "room",
        "RS",
        (0..n as i64).map(second).collect(),
    )]
}

#[test]
fn u6j_the_resident_memory_slope_per_10_minutes() {
    let high =
        |logs: &[Log]| figure_check("rss_slope_bytes_per_10min", "<=", json!(1_000_000), logs).0;
    let low =
        |logs: &[Log]| figure_check("rss_slope_bytes_per_10min", ">=", json!(-1_000_000), logs).0;
    let flat = rss_run(20, |_| 50_000_000);
    assert_eq!(
        (high(&flat), low(&flat)),
        ("Pass 0".to_owned(), "Pass 0".to_owned())
    );
    let rising = rss_run(20, |sec| 50_000_000 + 1_000 * sec);
    assert_eq!(
        (high(&rising), low(&rising)),
        ("Pass 600000".to_owned(), "Pass 600000".to_owned())
    );
    let leak = rss_run(20, |sec| 50_000_000 + 2_000 * sec);
    assert_eq!(
        (high(&leak), low(&leak)),
        ("Fail 1200000".to_owned(), "Pass 1200000".to_owned())
    );
    let falling = rss_run(20, |sec| 50_000_000 - 2_000 * sec);
    assert_eq!(
        (high(&falling), low(&falling)),
        ("Pass -1200000".to_owned(), "Fail -1200000".to_owned())
    );
    // x is the record's second, not its place in the log: a missing second and a warm-up second move nothing.
    let mut holed = rss_run(20, |sec| 50_000_000 + 1_000 * sec);
    holed[0].seconds.remove(7);
    holed[0].seconds[0].ph = Warmup;
    assert_eq!(high(&holed), "Pass 600000");
    // Not on a line: (0, 0), (1, 0), (2, 3) fit a slope of 3/2 bytes per second.
    assert_eq!(high(&rss_run(3, |sec| [0, 0, 3][sec as usize])), "Pass 900");
    // 2 hours at 1 GB, 1 byte more each second: exact, and no overflow.
    assert_eq!(high(&rss_run(7_200, |sec| 1_000_000_000 + sec)), "Pass 600");
    // A malformed log (a second numbered at the end of u64) gives a gap, never a wrapped slope.
    let mut far = rss_run(3, |_| 1);
    far[0].seconds[2].sec = u64::MAX;
    let (line, note) = figure_check("rss_slope_bytes_per_10min", "<=", json!(1_000_000), &far);
    let wide = "run f1: second.v.rss_bytes: its seconds or values are too large for an exact slope";
    assert_eq!((line.as_str(), note.as_str()), ("Gap -", wide));
    let one = figure_check(
        "rss_slope_bytes_per_10min",
        "<=",
        json!(1_000_000),
        &rss_run(1, |_| 1),
    );
    let gap = "run f1: one measured second of second.v.rss_bytes gives no slope";
    assert_eq!(one, ("Gap -".to_owned(), gap.to_owned()));
    let mut logs = rss_run(20, |_| 1);
    logs[0].seconds[4].v.remove("rss_bytes");
    assert_eq!(high(&logs), "Gap -");
    let mut logs = rss_run(20, |_| 1);
    logs[0].seconds[4].sec = 3;
    let twice = figure_check("rss_slope_bytes_per_10min", "<=", json!(1_000_000), &logs);
    assert_eq!(twice.1, "run f1: the room log has measured second 3 twice");
}

/// Run f1's room log: `n` measured seconds, each with slot 0's and slot 1's length of both per-client queues from
/// `queue(sec)` (the same in both), and `buffered(sec)` as the 3 sessions' buffered bytes.
fn queue_run(
    n: i64,
    queue: impl Fn(i64) -> [i64; 2],
    buffered: impl Fn(i64) -> [i64; 3],
) -> Vec<Log> {
    let second = |sec| {
        let queues = ["out_queue_count", "event_queue_count"].map(|k| (k, queue(sec).to_vec()));
        let mut lists = queues.to_vec();
        lists.push(("session_buffered_bytes", buffered(sec).to_vec()));
        (Measured, vec![], lists)
    };
    vec![seconds_log("room", "RS", (0..n).map(second).collect())]
}

#[test]
fn u6j_a_queue_that_grows_from_the_first_10_minutes_to_the_last() {
    let grown = |logs: &[Log]| figure_check("queue_growth_count", "==", json!(0), logs);
    // 1,300 measured seconds: the first 10 minutes are seconds 0 to 599, the last 700 to 1,299.
    let flat = queue_run(1_300, |_| [5, 5], |_| [0; 3]);
    assert_eq!(grown(&flat).0, "Pass 0");
    // Slot 1's largest length is 5 in the first 10 minutes and 6 in the last: both of its queues count.
    let run = queue_run(1_300, |sec| [5, 5 + i64::from(sec == 1_000)], |_| [0; 3]);
    assert_eq!(grown(&run).0, "Fail 2");
    let mut one = queue_run(1_300, |_| [5, 5], |_| [0; 3]);
    one[0].seconds[699]
        .s
        .insert("event_queue_count".into(), vec![9, 5]);
    one[0].seconds[700]
        .s
        .insert("out_queue_count".into(), vec![6, 5]);
    assert_eq!(
        grown(&one).0,
        "Fail 1",
        "second 699 is in neither window, second 700 in the last"
    );
    // A fall, and a peak between the two windows, are not growth.
    let run = queue_run(
        1_300,
        |sec| [5 - i64::from(sec > 600), 5 + i64::from(sec == 650)],
        |_| [0; 3],
    );
    assert_eq!(grown(&run).0, "Pass 0");
    // The windows go by time: with second 100's record missing, second 600 is still outside the first 10 minutes,
    // so slot 0's peak there and in the last 10 minutes is growth (the first 600 records would hold second 600).
    let peaks = |sec| [5 + i64::from(sec == 600 || sec == 1_000), 5];
    let mut holed = queue_run(1_300, peaks, |_| [0; 3]);
    holed[0].seconds.remove(100);
    assert_eq!(grown(&holed).0, "Fail 2");
    let short = grown(&queue_run(1_199, |_| [5, 5], |_| [0; 3]));
    let gap = "run f1: measured seconds 0 to 1198 of second.s.out_queue_count: the first and the last 10 minutes \
               need 1,200 measured seconds";
    assert_eq!(short, ("Gap -".to_owned(), gap.to_owned()));
    assert_eq!(grown(&queue_run(1_200, |_| [5, 5], |_| [0; 3])).0, "Pass 0");
    let mut logs = queue_run(1_300, |_| [5, 5], |_| [0; 3]);
    logs[0].seconds[9].s.remove("event_queue_count");
    let gap = "run f1: measured second 9 of the room log has no second.s.event_queue_count";
    assert_eq!(grown(&logs), ("Gap -".to_owned(), gap.to_owned()));
}

#[test]
fn u6j_session_memory_that_grows_from_the_first_half_to_the_second() {
    let grown = |logs: &[Log]| figure_check("session_memory_growth_count", "==", json!(0), logs);
    // 5 measured seconds: seconds 0 to 2 are the first half, 3 and 4 the second.
    assert_eq!(
        grown(&queue_run(5, |_| [1, 1], |_| [10, 10, 10])).0,
        "Pass 0"
    );
    let run = queue_run(5, |_| [1, 1], |sec| [10 + i64::from(sec == 4), 10, 10]);
    assert_eq!(grown(&run).0, "Fail 1");
    // The middle second is the first half's: a peak there that the second half only equals is flat.
    let run = queue_run(5, |_| [1, 1], |sec| [10 + i64::from(sec >= 2), 10, 10]);
    assert_eq!(grown(&run).0, "Pass 0");
    // A fall is not growth; a queue that grows counts beside the buffers.
    let run = queue_run(
        5,
        |sec| [1, 1 + i64::from(sec == 3)],
        |sec| [10 - i64::from(sec > 2), 10, 10],
    );
    assert_eq!(grown(&run).0, "Fail 2");
    let mut logs = queue_run(5, |_| [1, 1], |_| [10, 10, 10]);
    logs[0].seconds[1].s.remove("session_buffered_bytes");
    let gap = "run f1: measured second 1 of the room log has no second.s.session_buffered_bytes";
    assert_eq!(grown(&logs), ("Gap -".to_owned(), gap.to_owned()));
    // The buffers are one per session, not one per client: their count only has to stay the same.
    let mut logs = queue_run(5, |_| [1, 1], |_| [10, 10, 10]);
    logs[0].seconds[3]
        .s
        .insert("session_buffered_bytes".into(), vec![10, 10]);
    let gap = "run f1: measured second 3's second.s.session_buffered_bytes holds 2 entries, not 3, as its first \
               measured second";
    assert_eq!(grown(&logs), ("Gap -".to_owned(), gap.to_owned()));
    let one = grown(&queue_run(1, |_| [1, 1], |_| [10, 10, 10]));
    let gap = "run f1: measured seconds 0 to 0 of second.s.session_buffered_bytes: two halves need 2 measured seconds";
    assert_eq!(one, ("Gap -".to_owned(), gap.to_owned()));
}

/// A mark like PM-0001-10: three parts by client count, the ratio checked on N=48's runs, the unit cost reported.
fn sync_mark() -> Value {
    let part =
        |n: u32, min: u32| json!({"name": format!("N={n}"), "select": {"clients": n}, "min": min});
    json!({
        "id": "PM-0001-10", "title": "sync cost", "draft": draft(), "verdict": "pass",
        "parts": [part(48, 1), part(24, 3), part(36, 3)],
        "checks": [{"name": "ratio", "figure": "sync_cost_ratio", "op": "<=", "value": "1.1", "parts": ["N=48"]}],
        "report": [{"name": "unit", "figure": "sync_unit_cost"}],
        "bites": []
    })
}

/// One room log of that mark: one measured tick with `work` ns of server-only work and `relevant` entities in
/// total over its `clients` clients.
fn sync_log(id: &'static str, clients: u32, work: i64, relevant: i64) -> Log {
    let mut s = LogSpec::room(id, "set-s", "PM-0001-10").ticks("server_only_ns", &[work]);
    s.clients = clients;
    let mut log = s.build();
    let mut list = vec![0; clients as usize];
    list[0] = relevant;
    log.ticks[0].s.insert("relevant_count".into(), list);
    log
}

/// Seven runs: the N=48 run at unit cost `own`, three at N=24 and three at N=36 at the given unit costs.
fn sync_runs(own: i64, n24: [i64; 3], n36: [i64; 3]) -> Vec<Log> {
    let ids = [["a24", "b24", "c24"], ["a36", "b36", "c36"]];
    let mut logs = vec![sync_log("a48", 48, own * 100, 100)];
    logs.extend((0..3).map(|k| sync_log(ids[0][k], 24, n24[k] * 100, 100)));
    logs.extend((0..3).map(|k| sync_log(ids[1][k], 36, n36[k] * 100, 100)));
    logs
}

/// N=48's run under `sync_mark`: its check as "Outcome value", and its note.
fn sync_out(logs: &[Log]) -> (String, String) {
    let v = judge(&marks_file(&[sync_mark()]), logs);
    let c = &v[0].parts[0].runs[0].checks[0];
    let line = format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-"));
    (line, c.note.clone().unwrap_or_default())
}

#[test]
fn u6j_the_sync_unit_cost_at_48_against_24_and_36() {
    // 110 against a mean of 100 at N=24 and of 105 at N=36: the larger ratio is 110/100.
    let logs = sync_runs(110, [100; 3], [105; 3]);
    assert_eq!(sync_out(&logs).0, "Pass 1.1");
    let v = judge(&marks_file(&[sync_mark()]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::Pass, "{:?}", v[0].why);
    assert_eq!(
        report(&v[0], "unit"),
        ["110", "100", "100", "100", "105", "105", "105"]
    );
    assert_eq!(sync_out(&sync_runs(111, [100; 3], [105; 3])).0, "Fail 1.11");
    // N=36 decides when it is the cheaper reference: 110/95.
    assert_eq!(
        sync_out(&sync_runs(110, [100; 3], [95; 3])).0,
        "Fail 1.157895"
    );
    // The mean of the three runs' unit costs: 90, 100 and 110 are 100.
    assert_eq!(
        sync_out(&sync_runs(110, [90, 100, 110], [105; 3])).0,
        "Pass 1.1"
    );
    // The unit cost is work over entities, whatever the client count: 550 ns over 5 entities is 110.
    let mut logs = sync_runs(110, [100; 3], [105; 3]);
    logs[0] = sync_log("a48", 48, 550, 5);
    assert_eq!(sync_out(&logs).0, "Pass 1.1");
    // A reference run that is not valid (a cut log) is not in the mean; with it gone, N=24 waits for its third run.
    let mut logs = sync_runs(110, [100; 3], [105; 3]);
    let mut cut = LogSpec::room("d24", "set-s", "PM-0001-10").ticks("server_only_ns", &[900]);
    (cut.clients, cut.footer) = (24, false);
    logs.push(cut.build());
    assert_eq!(sync_out(&logs).0, "Pass 1.1");
    logs.remove(3);
    let (line, note) = sync_out(&logs);
    let wait = "run a48: part N=24 has 2 of its 3 valid runs: its mean unit cost is not there yet";
    assert_eq!((line.as_str(), note.as_str()), ("Cannot -", wait));
    // Without the cut log (an INVALID run of its own), the mark waits: NOT-RUN.
    logs.pop();
    let v = judge(&marks_file(&[sync_mark()]), &logs);
    assert_eq!(v[0].verdict, VerdictKind::NotRun, "{:?}", v[0].why);
}

#[test]
fn u6j_the_sync_unit_costs_gaps_and_its_large_sums() {
    let gap = |edit: &dyn Fn(&mut Vec<Log>)| {
        let mut logs = sync_runs(110, [100; 3], [105; 3]);
        edit(&mut logs);
        let (line, note) = sync_out(&logs);
        assert_eq!(line, "Gap -", "{note}");
        note
    };
    let why = gap(&|l| drop(l[0].ticks[0].s.remove("relevant_count")));
    assert_eq!(
        why,
        "run a48: measured tick 1 of the room log lacks tick.v.server_only_ns or tick.s.relevant_count"
    );
    let why = gap(&|l| drop(l[0].ticks[0].s.insert("relevant_count".into(), vec![1; 47])));
    assert_eq!(
        why,
        "run a48: tick 1's tick.s.relevant_count holds 47 entries, not the run's 48 clients"
    );
    let why = gap(&|l| l[0].ticks[0].ph = Warmup);
    assert_eq!(
        why,
        "run a48: no measured tick in the room log (tick.v.server_only_ns)"
    );
    let why = gap(&|l| drop(l[0].ticks[0].s.insert("relevant_count".into(), vec![0; 48])));
    assert_eq!(
        why,
        "run a48: tick.s.relevant_count sums to 0 over the measured ticks: no unit cost"
    );
    // A reference whose work is all 0 gives nothing to divide by.
    let why =
        gap(&|l| (1..4).for_each(|k| *l[k].ticks[0].v.get_mut("server_only_ns").expect("set") = 0));
    assert_eq!(why, "run a48: part N=24 has a mean unit cost of 0");
    // A gap in a reference run's own record is this run's gap too.
    let why = gap(&|l| drop(l[5].ticks[0].s.remove("relevant_count")));
    assert_eq!(
        why,
        "run b36: measured tick 1 of the room log lacks tick.v.server_only_ns or tick.s.relevant_count"
    );
    // Sums as large as a 30-minute soak's (a 33 ms tick on each of 54,000 ticks, 62 entities for each of 48
    // clients), on denominators with no common factor: the exact fractions stay inside i128.
    let mut logs = vec![sync_log("a48", 48, 1_781_999_999_989, 160_704_011)];
    let refs = [
        (593_999_999_971, 26_783_993),
        (593_999_999_993, 26_784_011),
        (594_000_000_011, 26_784_017),
    ];
    let ids = [["a24", "b24", "c24"], ["a36", "b36", "c36"]];
    for (n, ids) in [24, 36].into_iter().zip(ids) {
        logs.extend((0..3).map(|k| sync_log(ids[k], n, refs[k].0, refs[k].1 + i64::from(n))));
    }
    // 11,088.6 ns per entity against a mean of 22,177.4 at N=24 and at N=36.
    assert_eq!(sync_out(&logs), ("Pass 0.500001".to_owned(), String::new()));
}

/// Run `id` of part N=24 with totals as large as a 10-minute run's (18,000 ticks x 24 clients x 38 entities).
fn ten_minute_24(id: &'static str, k: usize) -> Log {
    let totals = [
        (197_999_999_977, 16_416_007),
        (198_000_000_019, 16_416_011),
        (197_999_999_993, 16_416_013),
        (198_000_000_007, 16_416_029),
        (198_000_000_043, 16_416_031),
        (197_999_999_929, 16_416_037),
    ];
    sync_log(id, 24, totals[k].0, totals[k].1)
}

/// The rust-reviewer's and the test-engineer's High (H1): the exact mean of the reference runs' unit costs is over
/// the part's `min` runs, as -10's row has it ("the 3 runs"); a fourth valid run is a gap that says so, and sums
/// that would not fit the judge's arithmetic are a gap too, never a stop of the whole judge.
#[test]
fn u6j_the_sync_reference_is_the_parts_min_runs_and_never_stops_the_judge() {
    let ids = ["a24", "b24", "c24", "d24", "e24", "f24"];
    let with = |n24: usize, mark: &Value| {
        let mut logs = vec![sync_log("a48", 48, 1_781_999_999_989, 160_704_011)];
        logs.extend((0..n24).map(|k| ten_minute_24(ids[k], k)));
        let n36 = [
            (197_999_999_959, 32_400_011),
            (198_000_000_031, 32_400_017),
            (198_000_000_049, 32_400_023),
        ];
        logs.extend(
            ["a36", "b36", "c36"]
                .into_iter()
                .zip(n36)
                .map(|(id, (w, r))| sync_log(id, 36, w, r)),
        );
        let v = judge(&marks_file(std::slice::from_ref(mark)), &logs);
        let c = &v[0].parts[0].runs[0].checks[0];
        (
            format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-")),
            c.note.clone().unwrap_or_default(),
        )
    };
    // Three 10-minute runs per part against a 30-minute N=48 run: exact, a value (11,088.6 ns per entity against
    // a mean of 6,111.1 at N=36, the cheaper reference).
    let value = ("Fail 1.814517".to_owned(), String::new());
    assert_eq!(with(3, &sync_mark()), value);
    // A fourth valid run of N=24 (a rerun in the same folder), and six (two sessions' runs).
    let gap = |n: usize| {
        let folder = "the row's mean is over its 3: judge a folder that holds one session's runs";
        (
            "Gap -".to_owned(),
            format!("run a48: part N=24 has {n} valid runs; {folder}"),
        )
    };
    assert_eq!(with(4, &sync_mark()), gap(4));
    assert_eq!(with(6, &sync_mark()), gap(6));
    // A mark that asks for four: the exact mean of four such unit costs does not fit, and the judge says so.
    let mut four = sync_mark();
    four["parts"][1]["min"] = json!(4);
    let wide =
        "run a48: part N=24 has unit costs whose exact mean does not fit the judge's arithmetic";
    assert_eq!(with(4, &four), ("Gap -".to_owned(), wide.to_owned()));
    // And for six: there the sum itself passes i128, before any ratio is formed.
    four["parts"][1]["min"] = json!(6);
    assert_eq!(with(6, &four), ("Gap -".to_owned(), wide.to_owned()));
}

/// The plan review's condition 2 (the test-engineer's M3): a part's valid runs, as the figures read them, hold
/// their validity rules with nothing undecided and pass the part's setup checks; a moved part gives none.
#[test]
fn u6j_only_valid_runs_of_kept_parts_reach_the_figures() {
    let out = |mark: &Value, logs: &[Log]| {
        let v = judge(&marks_file(&[mark.clone(), unbuilt_mark()]), logs);
        let c = &v[0].parts[0].runs[0].checks[0];
        (
            format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-")),
            c.note.clone().unwrap_or_default(),
        )
    };
    // A fourth N=24 run that fails the part's setup check is not one of its valid runs: the mean is over the three.
    let mut mark = sync_mark();
    mark["checks"].as_array_mut().expect("checks").push(json!({
        "name": "setup: starts low", "series": "header.params.start_tick", "stat": "max", "op": "<=",
        "value": 10, "setup": true, "parts": ["N=24"]}));
    let mut logs = sync_runs(110, [100; 3], [105; 3]);
    logs.push(sync_log("d24", 24, 90_000, 100));
    for (k, log) in logs
        .iter_mut()
        .enumerate()
        .filter(|(_, l)| l.header.clients == 24)
    {
        log.header
            .params
            .insert("start_tick".into(), json!(if k == 7 { 99 } else { 5 }));
    }
    assert_eq!(out(&mark, &logs), ("Pass 1.1".to_owned(), String::new()));
    // N=36's runs wait on a precondition whose figure is not built: undecided, so none of them is in the mean yet.
    let mut mark = sync_mark();
    mark["parts"][2]["validity"] = json!(["pre:PM-0001-54"]);
    let mut logs = sync_runs(110, [100; 3], [105; 3]);
    for log in logs.iter_mut().filter(|l| l.header.clients == 36) {
        log.header.marks.push("PM-0001-54".to_owned());
    }
    let wait = |part: &str, min: u32| {
        let why = format!(
            "run a48: part {part} has 0 of its {min} valid runs: its mean unit cost is not there yet"
        );
        ("Cannot -".to_owned(), why)
    };
    assert_eq!(out(&mark, &logs), wait("N=36", 3));
    // A moved part's runs never reach a figure.
    let mut mark = sync_mark();
    mark["parts"][2]["moved"] = json!({"to": "Slice 6", "why": "a test", "by": "the test"});
    assert_eq!(
        out(&mark, &sync_runs(110, [100; 3], [105; 3])),
        wait("N=36", 1)
    );
}

/// A mark whose one check is a figure nobody has built: as a precondition it stays undecided.
fn unbuilt_mark() -> Value {
    json!({
        "id": "PM-0001-54", "title": "not built", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "any", "select": {"layer": "L2"}, "min": 1}],
        "checks": [{"name": "throws", "figure": "throw_desync_count", "op": "==", "value": 0}],
        "bites": []
    })
}

/// The test-engineer's M1: the unit cost is the work of every measured tick over the entities of every (tick,
/// slot) entry, and R2's E the mean over every such entry.
#[test]
fn u6j_the_unit_cost_and_r2s_entities_read_every_tick_and_every_slot() {
    // Tick 1: 3,000 ns, 1 entity per client (48); tick 2: 6,600 ns, 2 per client (96): 9,600 / 144.
    let mut s =
        LogSpec::room("a48", "set-s", "PM-0001-10").ticks("server_only_ns", &[3_000, 6_600]);
    s.clients = 48;
    let mut own = s.build();
    for (t, per_client) in own.ticks.iter_mut().zip([1, 2]) {
        t.s.insert("relevant_count".into(), vec![per_client; 48]);
    }
    let mut logs = sync_runs(110, [100; 3], [105; 3]);
    logs[0] = own;
    let v = judge(&marks_file(&[sync_mark()]), &logs);
    assert_eq!(report(&v[0], "unit")[0], "66.666667", "{:?}", v[0].report);
    // Slots alternating 63 and 61 entities on one tick, 62 each on the other: the mean entry is still 62.
    let mut logs = egress_run(R2_COUNTS, [1_000, 1_000 + R2_BYTES], [0, R2_BYTES]);
    let uneven: Vec<i64> = (0..48).map(|k| if k % 2 == 0 { 63 } else { 61 }).collect();
    logs[0].ticks[0].s.insert("relevant_count".into(), uneven);
    let (line, note) = figure_check("r2_down_formula_bits", "==", json!("146122.56"), &logs);
    assert_eq!(line, "Pass 146122.56", "{note}");
}

/// Run f1's room log for R2's formula, 48 clients at 30 Hz: two measured ticks with `relevant` entities in every
/// client's set and `vehicles` big vehicles; 25 measured seconds with `shots` and `reliable` each and `spawns` in
/// all; the interface counter (`iface`) and the room's metric read twice, an hour apart.
fn egress_run(counts: [i64; 5], iface: [i64; 2], metric: [i64; 2]) -> Vec<Log> {
    let [relevant, vehicles, shots, reliable, spawns] = counts;
    let mut s = LogSpec::room("f1", "set-f", M).ticks("big_vehicles_count", &[vehicles; 2]);
    let second = |k: i64| {
        let spawned = spawns / 25 + if k == 24 { spawns % 25 } else { 0 };
        let v = vec![
            ("shots_count", shots),
            ("reliable_events_count", reliable),
            ("projectile_spawns_count", spawned),
        ];
        (Measured, v, vec![])
    };
    s.seconds = (0..25).map(second).collect();
    let read = |ev: &str, field: &str, value: i64, ns: u64| {
        let mut e = EventRecord::new(ev, Measured).with(field, value);
        e.ns = Some(ns);
        e
    };
    let hour = 3_600_000_000_000;
    s.events = vec![
        read("iface", "tx_bytes", iface[0], 5),
        read("room_metric", "sent_bytes", metric[0], 5),
        read("iface", "tx_bytes", iface[1], 5 + hour),
        read("room_metric", "sent_bytes", metric[1], 5 + hour),
    ];
    let mut log = s.build();
    for t in &mut log.ticks {
        t.s.insert("relevant_count".into(), vec![relevant; 48]);
    }
    vec![log]
}

/// R2's own counts (r2-netcode.md's worked estimate): 47 other soldiers and 15 vehicles in every set, 72 shots and
/// 10 reliable events a second, 528 projectile spawns in 25 seconds.
const R2_COUNTS: [i64; 5] = [62, 15, 72, 10, 528];
const R2_BYTES: i64 = 3_156_247_296;

#[test]
fn u6j_r2s_formula_at_its_own_counts_and_the_egress_against_it() {
    let golden = egress_run(R2_COUNTS, [1_000, 1_000 + R2_BYTES], [0, R2_BYTES]);
    let check = |figure: &str, op: &str, value: Value, logs: &[Log]| {
        figure_check(figure, op, value, logs).0
    };
    // 3653064/25 bits a second per client (R2 prints 146.1 kbps); x 48 clients x 3,600 s / 8: R2's 3.156 GB.
    assert_eq!(
        check("r2_down_formula_bits", "==", json!("146122.56"), &golden),
        "Pass 146122.56"
    );
    assert_eq!(
        check("r2_egress_formula_bytes", "==", json!(R2_BYTES), &golden),
        "Pass 3156247296"
    );
    assert_eq!(
        check("egress_interface_bytes", "==", json!(R2_BYTES), &golden),
        "Pass 3156247296"
    );
    assert_eq!(
        check("egress_vs_r2_ratio", "<=", json!("1.2"), &golden),
        "Pass 1"
    );
    assert_eq!(
        check(
            "egress_metric_vs_interface_ratio",
            ">=",
            json!("0.95"),
            &golden
        ),
        "Pass 1"
    );
    // Everything sent twice: 2.0.
    let double = egress_run(R2_COUNTS, [1_000, 1_000 + 2 * R2_BYTES], [0, R2_BYTES]);
    assert_eq!(
        check("egress_vs_r2_ratio", "<=", json!("1.2"), &double),
        "Fail 2"
    );
    assert_eq!(
        check(
            "egress_metric_vs_interface_ratio",
            ">=",
            json!("0.95"),
            &double
        ),
        "Fail 0.5"
    );
    // The limits themselves pass. With no shots, events or spawns the formula gives 3,024,864,000 B.
    let quiet =
        |bytes: i64, metric: i64| egress_run([62, 15, 0, 0, 0], [7, 7 + bytes], [0, metric]);
    let at =
        |logs: &[Log], op: &str, limit: &str| check("egress_vs_r2_ratio", op, json!(limit), logs);
    assert_eq!(at(&quiet(2_419_891_200, 0), ">=", "0.8"), "Pass 0.8");
    assert_eq!(at(&quiet(2_419_891_199, 0), ">=", "0.8"), "Fail 0.8");
    assert_eq!(at(&quiet(3_629_836_800, 0), "<=", "1.2"), "Pass 1.2");
    assert_eq!(at(&quiet(3_629_836_801, 0), "<=", "1.2"), "Fail 1.2");
    let metric = |logs: &[Log], op: &str, limit: &str| {
        check("egress_metric_vs_interface_ratio", op, json!(limit), logs)
    };
    assert_eq!(
        metric(&quiet(2_000_000, 2_100_000), "<=", "1.05"),
        "Pass 1.05"
    );
    assert_eq!(
        metric(&quiet(2_000_000, 2_100_002), "<=", "1.05"),
        "Fail 1.050001"
    );
    assert_eq!(
        metric(&quiet(2_000_000, 1_900_000), ">=", "0.95"),
        "Pass 0.95"
    );
    // The reports show the arithmetic, and the egress one the time between the counter's two reads.
    let mark = json!({
        "id": M, "title": "egress", "draft": draft(), "verdict": "reported",
        "parts": [{"name": "P", "select": {"layer": "L2"}, "min": 1}],
        "report": [{"name": "bits", "figure": "r2_down_formula_bits"},
                   {"name": "bytes", "figure": "r2_egress_formula_bytes"},
                   {"name": "interface", "figure": "egress_interface_bytes"}]
    });
    let v = judge(&marks_file(&[mark]), &golden);
    let sum = "4/5 x 30 Hz x (900 + 75 x 62 + 19 x 15) + 32 x 72 + 192 x 10 + 88 x 21.12";
    assert_eq!(report(&v[0], "bits"), [format!("146122.56 ({sum})")]);
    let bytes = format!(
        "3156247296 (({sum}) x 48 clients x 3,600 s / 8; the interface counter's two reads are 3600 s apart)"
    );
    assert_eq!(report(&v[0], "bytes"), [bytes]);
    assert_eq!(report(&v[0], "interface"), ["3156247296"]);
}

#[test]
fn u6j_the_formulas_and_the_counters_gaps() {
    let gap = |figure: &str, edit: &dyn Fn(&mut Log)| {
        let mut logs = egress_run(R2_COUNTS, [1_000, 1_000 + R2_BYTES], [0, R2_BYTES]);
        edit(&mut logs[0]);
        let (line, note) = figure_check(figure, "<=", json!(1), &logs);
        assert_eq!(line, "Gap -", "{figure}: {note}");
        note
    };
    // The interface counter: read three times, read once, fallen, standing still, not a whole number.
    let third = |l: &mut Log| l.events.push(l.events[0].clone());
    let why = gap("egress_interface_bytes", &third);
    assert_eq!(
        why,
        "run f1: 3 event.iface records in the room log, not the two reads (start and end)"
    );
    let why = gap("egress_vs_r2_ratio", &|l| drop(l.events.remove(2)));
    assert_eq!(
        why,
        "run f1: 1 event.iface records in the room log, not the two reads (start and end)"
    );
    let fell = |l: &mut Log| l.events.swap(0, 2);
    let why = gap("egress_vs_r2_ratio", &fell);
    assert_eq!(
        why,
        "run f1: event.iface.tx_bytes fell from 3156248296 to 1000 between its two reads"
    );
    assert_eq!(gap("egress_metric_vs_interface_ratio", &fell), why);
    let still = |l: &mut Log| drop(l.events[2].f.insert("tx_bytes".into(), json!(1_000)));
    let why = gap("egress_interface_bytes", &still);
    assert_eq!(
        why,
        "run f1: event.iface.tx_bytes did not move between its two reads"
    );
    let why = gap("egress_interface_bytes", &|l| {
        drop(l.events[2].f.insert("tx_bytes".into(), json!("9")))
    });
    assert_eq!(
        why,
        "run f1: an event.iface record has no whole-number tx_bytes"
    );
    let why = gap("egress_metric_vs_interface_ratio", &|l| {
        drop(l.events.remove(3))
    });
    assert_eq!(
        why,
        "run f1: 1 event.room_metric records in the room log, not the two reads (start and end)"
    );
    // The formula's inputs: a measured tick or second without one of them, a list of another length.
    let why = gap("r2_down_formula_bits", &|l| {
        assert!(l.ticks[1].v.remove("big_vehicles_count").is_some())
    });
    assert_eq!(
        why,
        "run f1: measured tick 2 of the room log lacks tick.s.relevant_count or tick.v.big_vehicles_count"
    );
    let why = gap("r2_egress_formula_bytes", &|l| {
        assert!(l.seconds[3].v.remove("reliable_events_count").is_some())
    });
    assert_eq!(
        why,
        "run f1: measured second 3 of the room log has no second.v.reliable_events_count"
    );
    let why = gap("egress_vs_r2_ratio", &|l| {
        drop(l.ticks[0].s.insert("relevant_count".into(), vec![62; 47]))
    });
    assert_eq!(
        why,
        "run f1: tick 1's tick.s.relevant_count holds 47 entries, not the run's 48 clients"
    );
    let why = gap("r2_down_formula_bits", &|l| {
        l.ticks.iter_mut().for_each(|t| t.ph = Warmup)
    });
    assert_eq!(
        why,
        "run f1: no measured tick in the room log, or 0 clients (tick.s.relevant_count)"
    );
    // A counter read in another process's log is not the room's.
    let mut logs = egress_run(R2_COUNTS, [1_000, 1_000 + R2_BYTES], [0, R2_BYTES]);
    let mut bots = seconds_log("bots", "LG", vec![]);
    bots.events = vec![logs[0].events[0].clone()];
    logs.push(bots);
    let (line, note) = figure_check("egress_interface_bytes", "<=", json!(1), &logs);
    let why = "run f1: event.iface in the bots log on LG1: only process room writes it";
    assert_eq!((line.as_str(), note.as_str()), ("Gap -", why));
}

/// Run f1 with 3 clients: the room's tick panic at 1,000 ns on its clock, and LG's bots log with each listed
/// (slot, server-clock estimate) as a `room_end`.
fn crash_run(closes: &[(u64, i64)]) -> Vec<Log> {
    let mut room = seconds_log("room", "RS", vec![]);
    room.header.clients = 3;
    room.events = vec![EventRecord::new("tick_panic", After).with("server_ns", 1_000)];
    let mut bots = seconds_log("bots", "LG", vec![]);
    bots.header.clients = 3;
    let close = |(slot, at): &(u64, i64)| {
        EventRecord::new("room_end", After)
            .with("slot", *slot)
            .with("server_est_ns", *at)
    };
    bots.events = closes.iter().map(close).collect();
    vec![room, bots]
}

#[test]
fn u6j_every_client_is_closed_within_a_second_of_the_tick_panic() {
    let second = 1_000_000_000;
    let closed = |logs: &[Log]| figure_check("crash_close_max_ns", "<=", json!(second), logs);
    // The slowest of the three decides; a close the client's clock puts before the panic counts as it is.
    assert_eq!(
        closed(&crash_run(&[(0, 900), (1, 1_000 + second), (2, 5_000)])).0,
        "Pass 1000000000"
    );
    assert_eq!(
        closed(&crash_run(&[(0, 900), (1, 1_001 + second), (2, 5_000)])).0,
        "Fail 1000000001"
    );
    assert_eq!(
        closed(&crash_run(&[(0, 900), (1, 400), (2, 100)])).0,
        "Pass -100"
    );
    // A client with no room_end was left open: the check fails with no number, and the reason is the line.
    let open = crash_run(&[(0, 900), (2, 5_000)]);
    let why = "run f1: slot 1 has no event.room_end: 1 of 3 clients left open after the tick panic";
    assert_eq!(closed(&open), ("Fail -".to_owned(), why.to_owned()));
    let mark = json!({
        "id": M, "title": "crash", "draft": draft(), "verdict": "pass",
        "parts": [{"name": "P", "select": {"layer": "L2"}, "min": 1}],
        "checks": [{"name": "closed", "figure": "crash_close_max_ns", "op": "<=", "value": second}],
        "bites": []
    });
    let (kind, lines) = verdict_of(&[mark], &open);
    assert_eq!(kind, VerdictKind::Fail);
    assert_eq!(
        lines,
        [format!("P: run f1: closed: {why} (needs <= 1000000000)")]
    );
    // Gaps: two panics, a panic with no time, a room_end twice for a slot, for a slot the run lacks, with no slot,
    // with no time; either event in another process's log; no bots log.
    let full = || crash_run(&[(0, 900), (1, 950), (2, 5_000)]);
    // Copies event `from.1` of log `from.0` to the end of log `to`.
    let copy = |l: &mut Vec<Log>, from: (usize, usize), to: usize| {
        let event = l[from.0].events[from.1].clone();
        l[to].events.push(event);
    };
    let gap = |edit: &dyn Fn(&mut Vec<Log>)| {
        let mut logs = full();
        edit(&mut logs);
        let (line, note) = closed(&logs);
        assert_eq!(line, "Gap -", "{note}");
        note
    };
    let why = gap(&|l| copy(l, (0, 0), 0));
    assert_eq!(
        why,
        "run f1: 2 event.tick_panic records in the room log, not one with a whole-number server_ns"
    );
    let why = gap(&|l| drop(l[0].events[0].f.remove("server_ns")));
    assert_eq!(
        why,
        "run f1: 1 event.tick_panic records in the room log, not one with a whole-number server_ns"
    );
    let why = gap(&|l| copy(l, (1, 2), 1));
    assert_eq!(why, "run f1: two event.room_end records for slot 2");
    let why = gap(&|l| drop(l[1].events[2].f.insert("slot".into(), json!(3))));
    assert_eq!(
        why,
        "run f1: an event.room_end record names slot 3; the run has 3 clients"
    );
    let why = gap(&|l| drop(l[1].events[1].f.remove("slot")));
    assert_eq!(
        why,
        "run f1: an event.room_end record has no slot or no whole-number server_est_ns"
    );
    assert_eq!(
        gap(&|l| drop(l[1].events[1].f.remove("server_est_ns"))),
        why
    );
    let why = gap(&|l| copy(l, (1, 0), 0));
    assert_eq!(
        why,
        "run f1: event.room_end in the room log on RS1: only process bots writes it"
    );
    let why = gap(&|l| copy(l, (0, 0), 1));
    assert_eq!(
        why,
        "run f1: event.tick_panic in the bots log on LG1: only process room writes it"
    );
    assert_eq!(gap(&|l| drop(l.pop())), "run f1: no bots log");
    let why = gap(&|l| {
        l[0].header.clients = 0;
        l[1].events.clear();
    });
    assert_eq!(why, "run f1: a run of 0 clients has no close to time");
}

/// A mark like PM-0006-19: three logging-on runs, the pooled ratio, its own validity rule and the busy-wait bite.
fn logging_mark() -> Value {
    json!({
        "id": "PM-0006-19", "title": "logging cost", "verdict": "pass",
        "draft": {"file": "docs/adr/0006-infrastructure-operations.md", "row_sha256": "0".repeat(64)},
        "parts": [{"name": "on", "select": {"params.suite": "logging-cost"}, "min": 3}],
        "validity": ["PM-0006-19"],
        "checks": [{"name": "cost", "figure": "logging_cost_ratio", "op": "<=", "value": "1.02"}],
        "bites": [{"id": "PM-0006-19/busy-50us", "what": "a 50 us busy-wait in the log call"}]
    })
}

/// One run of that mark in set-l: role `measured` (logging on), `logging-off`, or `bite` (the busy-wait bite).
fn logging_run(id: &'static str, role: &'static str, work: &[i64]) -> Log {
    let mut s = LogSpec::room(id, "set-l", "PM-0006-19").ticks("work_ns", work);
    s.role = role;
    s.params.push(("suite", json!("logging-cost")));
    s.bite = (role == "bite").then_some("PM-0006-19/busy-50us");
    s.build()
}

/// Three on runs with the given slowest ticks, three off runs at 100 ns, and the bite run at 150 ns.
fn logging_runs(on: [i64; 3]) -> Vec<Log> {
    let ids = [["on-1", "on-2", "on-3"], ["off-1", "off-2", "off-3"]];
    let mut logs: Vec<Log> = (0..3)
        .map(|k| logging_run(ids[0][k], "measured", &[90, on[k]]))
        .collect();
    logs.extend(ids[1].map(|id| logging_run(id, "logging-off", &[100, 100])));
    logs.push(logging_run("busy", "bite", &[150, 150]));
    logs
}

/// The on runs' check under `logging_mark`, each as "Outcome value".
fn logging_checks(v: &npt_judge::Verdict) -> Vec<String> {
    let line = |c: &npt_judge::eval::CheckOut| {
        format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-"))
    };
    v.parts[0]
        .runs
        .iter()
        .map(|r| r.checks.first().map_or("none".to_owned(), line))
        .collect()
}

#[test]
fn u6j_the_logging_cost_pools_the_on_runs_against_the_off_runs() {
    let judged = |logs: &[Log]| judge(&marks_file(&[logging_mark()]), logs).remove(0);
    // Pooled p99 102 against 100: every on run reads the same 1.02. The bite run alone is 1.5: shown.
    let v = judged(&logging_runs([102, 101, 100]));
    assert_eq!(v.verdict, VerdictKind::Pass, "{:?}", v.why);
    assert_eq!(logging_checks(&v), ["Pass 1.02"; 3]);
    assert_eq!(v.bites[0].status, BiteStatus::Shown, "{:?}", v.bites[0]);
    // One slow run: the mean of the three runs' own p99s is 102, the pooled p99 is 106, and all three fail.
    let v = judged(&logging_runs([100, 100, 106]));
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    assert_eq!(logging_checks(&v), ["Fail 1.06"; 3]);
    assert_eq!(
        logging_checks(&judged(&logging_runs([103, 100, 100]))),
        ["Fail 1.03"; 3]
    );
    // Two off runs: not all there yet. Four: which three?
    let mut logs = logging_runs([102, 101, 100]);
    logs.remove(5);
    let v = judged(&logs);
    assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
    assert!(
        says(
            &v.why,
            "run on-1: 2 of the 3 logging-off runs are in set set-l"
        ),
        "{:?}",
        v.why
    );
    let mut logs = logging_runs([102, 101, 100]);
    logs.push(logging_run("off-4", "logging-off", &[100, 100]));
    let v = judged(&logs);
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(&v.why, "run on-1: 4 logging-off runs in set set-l, not 3"),
        "{:?}",
        v.why
    );
    // Two valid on runs (the third's log is cut): the part's three are not all there, and nothing fails early.
    let mut logs = logging_runs([100, 100, 106]);
    let mut cut = LogSpec::room("on-3", "set-l", "PM-0006-19").ticks("work_ns", &[90, 106]);
    cut.params.push(("suite", json!("logging-cost")));
    cut.footer = false;
    logs[2] = cut.build();
    let v = judged(&logs);
    assert_eq!(logging_checks(&v)[..2], ["Cannot -"; 2], "{:?}", v.why);
    let note = v.parts[0].runs[0].checks[0]
        .note
        .clone()
        .unwrap_or_default();
    assert_eq!(
        note,
        "run on-1: 2 of the 3 valid logging-on runs its part needs are in set set-l"
    );
}

/// The test-engineer's M3 and L6: each set pools its own on runs; an on run with no measured work and off runs
/// whose pooled p99 is 0 are gaps.
#[test]
fn u6j_the_logging_cost_pools_one_set_at_a_time() {
    let judged = |logs: &[Log]| judge(&marks_file(&[logging_mark()]), logs).remove(0);
    // A second set (set-m) whose on runs are twice as slow: neither set's pool holds the other's runs.
    let mut logs = logging_runs([102, 101, 100]);
    let other = [
        ("m-on-1", "measured", 200),
        ("m-on-2", "measured", 200),
        ("m-on-3", "measured", 200),
    ];
    let off = [
        ("m-off-1", "logging-off", 100),
        ("m-off-2", "logging-off", 100),
        ("m-off-3", "logging-off", 100),
    ];
    for (id, role, slowest) in other.into_iter().chain(off) {
        let mut log = logging_run(id, role, &[90, slowest]);
        log.header.set = "set-m".to_owned();
        logs.push(log);
    }
    let v = judged(&logs);
    for (run, line) in v.parts[0].runs.iter().zip(logging_checks(&v)) {
        let want = if run.run.starts_with("m-") {
            "Fail 2"
        } else {
            "Pass 1.02"
        };
        assert_eq!(line, want, "{}", run.run);
    }
    // Off runs that did no work at all: nothing to divide by.
    let mut logs = logging_runs([102, 101, 100]);
    for log in logs.iter_mut().filter(|l| l.header.role == "logging-off") {
        log.ticks
            .iter_mut()
            .for_each(|t| *t.v.get_mut("work_ns").expect("set") = 0);
    }
    let v = judged(&logs);
    let note = v.parts[0].runs[0].checks[0]
        .note
        .clone()
        .unwrap_or_default();
    assert_eq!(note, "run on-1: the logging-off runs' pooled p99 is 0");
    assert_eq!(logging_checks(&v), ["Gap -"; 3]);
    // An on run with no measured tick: a gap for every run of the pool (the mark has no validity rule here).
    let mut mark = logging_mark();
    mark["validity"] = json!([]);
    let mut logs = logging_runs([102, 101, 100]);
    logs[1].ticks.iter_mut().for_each(|t| t.ph = Warmup);
    let v = judge(&marks_file(&[mark]), &logs).remove(0);
    let note = v.parts[0].runs[0].checks[0]
        .note
        .clone()
        .unwrap_or_default();
    assert_eq!(note, "run on-2: no values for tick.v.work_ns");
    assert_eq!(logging_checks(&v), ["Gap -"; 3]);
}

/// PM-0006-19's rule and its figure both refuse a logging-off run that breaks a rule every run holds (the plan
/// review's condition 9: the rule skipped them before).
#[test]
fn u6j_a_logging_off_run_holds_the_rules_every_run_holds() {
    let mut logs = logging_runs([102, 101, 100]);
    let mut cut = LogSpec::room("off-2", "set-l", "PM-0006-19").ticks("work_ns", &[100, 100]);
    (cut.role, cut.footer) = ("logging-off", false);
    logs[4] = cut.build();
    let v = judge(&marks_file(&[logging_mark()]), &logs).remove(0);
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    let rule = "PM-0006-19: logging-off run off-2 is invalid: complete: ";
    assert!(
        v.parts[0]
            .runs
            .iter()
            .all(|r| !r.valid && says(&r.invalid_why, rule)),
        "{:?}",
        v.parts[0].runs
    );
    // The figure alone (no validity rule on the mark) says the same, as a gap.
    let mut mark = logging_mark();
    mark["validity"] = json!([]);
    let v = judge(&marks_file(&[mark]), &logs).remove(0);
    let note = v.parts[0].runs[0].checks[0]
        .note
        .clone()
        .unwrap_or_default();
    let cut = "run on-1: logging-off run off-2 is invalid: complete: the room log on RS1 has no footer (cut short)";
    assert_eq!(note, cut);
    assert_eq!(logging_checks(&v), ["Gap -"; 3]);
}

/// One room of an 8-room session (set set-k): its measured and after-phase ticks' work, and whether it was killed.
fn kill_room(id: &'static str, killed: bool, measured: &[i64], after: &[i64]) -> Log {
    let mut s = LogSpec::room(id, "set-k", M);
    let ticks = |ph: Phase, v: &[i64]| {
        v.iter()
            .map(|w| (ph, vec![("work_ns", *w)]))
            .collect::<Vec<_>>()
    };
    s.ticks = [ticks(Measured, measured), ticks(After, after)].concat();
    s.params.push(("killed", json!(killed)));
    s.build()
}

#[test]
fn u6j_a_surviving_rooms_p99_after_the_kill_against_before() {
    // The check on the survivor `room-1`; the killed room is the set's evidence of the kill.
    let ratio = |after: &[i64], with_killed: usize| {
        let mut logs = vec![kill_room("room-1", false, &[90, 100], after)];
        logs.extend(
            [
                kill_room("room-8", true, &[100], &[]),
                kill_room("room-7", true, &[100], &[]),
            ]
            .into_iter()
            .take(with_killed),
        );
        let mark = json!({
            "id": M, "title": "kill", "draft": draft(), "verdict": "pass",
            "parts": [{"name": "survivors", "select": {"params.killed": false}, "min": 1}],
            "checks": [{"name": "c", "figure": "kill_p99_ratio", "op": "<=", "value": "1.05"}],
            "bites": []
        });
        let v = judge(&marks_file(&[mark]), &logs).remove(0);
        let c = &v.parts[0].runs[0].checks[0];
        (
            format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-")),
            c.note.clone().unwrap_or_default(),
        )
    };
    assert_eq!(ratio(&[80, 105], 1).0, "Pass 1.05");
    // The before is the measured ticks alone: 106 after against 100, not against the run's 106 overall.
    assert_eq!(ratio(&[80, 106], 1).0, "Fail 1.06");
    assert_eq!(ratio(&[80, 95], 1).0, "Pass 0.95");
    let none = (
        "Gap -".to_owned(),
        "run room-1: no after-phase tick in the room log (tick.v.work_ns)".to_owned(),
    );
    assert_eq!(ratio(&[], 1), none);
    let wait = "run room-1: the killed room's run (params.killed true) is not in set set-k yet";
    assert_eq!(
        ratio(&[80, 105], 0),
        ("Cannot -".to_owned(), wait.to_owned())
    );
    let two = "run room-1: 2 runs of set set-k say params.killed true; one room is killed";
    assert_eq!(ratio(&[80, 105], 2), ("Gap -".to_owned(), two.to_owned()));
}

/// The gaps the review found untested (the test-engineer's L6): a measured p99 of 0 and a work series in another
/// process's log for the kill ratio; a formula of 0 bytes for the egress ratio.
#[test]
fn u6j_the_kill_ratios_and_the_egress_ratios_last_gaps() {
    let mark = |figure: &str| {
        json!({
            "id": M, "title": "gaps", "draft": draft(), "verdict": "pass",
            "parts": [{"name": "P", "select": {"params.killed": false}, "min": 1}],
            "checks": [{"name": "c", "figure": figure, "op": "<=", "value": 1}],
            "bites": []
        })
    };
    let note = |figure: &str, logs: &[Log]| {
        let v = judge(&marks_file(&[mark(figure)]), logs).remove(0);
        let c = &v.parts[0].runs[0].checks[0];
        assert_eq!(format!("{:?}", c.outcome), "Gap", "{c:?}");
        c.note.clone().unwrap_or_default()
    };
    let killed = kill_room("room-8", true, &[100], &[]);
    let idle = vec![kill_room("room-1", false, &[0, 0], &[5]), killed.clone()];
    assert_eq!(
        note("kill_p99_ratio", &idle),
        "run room-1: its measured p99 of tick.v.work_ns is 0"
    );
    let mut bots = kill_room("room-1", false, &[90, 100], &[105]);
    bots.header.process = "bots".to_owned();
    let why = "run room-1: tick.v.work_ns in the bots log on RS1: only process room writes it";
    assert_eq!(note("kill_p99_ratio", &[bots, killed]), why);
    // A room at 0 Hz that sent nothing countable: R2's formula gives 0 bytes, and the ratio has no bottom.
    let mut logs = egress_run([62, 15, 0, 0, 0], [7, 9], [0, 1]);
    logs[0].header.tick_hz = 0;
    logs[0].header.params.insert("killed".into(), json!(false));
    assert_eq!(
        note("egress_vs_r2_ratio", &logs),
        "run f1: R2's formula gives 0 bytes at the run's counts"
    );
}
