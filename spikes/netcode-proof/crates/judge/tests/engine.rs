//! The judge's engine on logs built here, one test per verdict path of `marks/README.md` §3 to §6. The committed
//! fixtures (tests/fixtures.rs) prove the same paths on files and on the real marks.json.

use npt_judge::eval::BiteStatus;
use npt_judge::{VerdictKind, judge, parse_marks};
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
    let (k, why) = verdict_of(&[outside], &[]);
    assert_eq!(k, VerdictKind::NotRun);
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
    let (part, v) = ws_part(&w);
    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
    assert!(says(&part.why, "ws-practice: "), "{:?}", part.why);
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
}
