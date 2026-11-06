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
                arch: "x86_64".to_owned(),
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
                binary_sha256: "0".repeat(64),
                features: vec![],
                profile: "release".to_owned(),
            },
            bite: self.bite.map(str::to_owned),
            layer: self.layer.to_owned(),
            profile: self.profile.to_owned(),
            seed: self.seed,
            tick_hz: 30,
            clients: 48,
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
