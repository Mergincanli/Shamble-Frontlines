//! The judge's engine on logs built here, one test per verdict path of `marks/README.md` §3 to §6. The committed
//! fixtures (tests/fixtures.rs) prove the same paths on files and on the real marks.json.

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
            layer: "L2".to_owned(),
            profile: "P2".to_owned(),
            seed: 1,
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
