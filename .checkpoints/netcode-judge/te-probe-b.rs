
// ---- test-engineer's probes (scratch copy only; appended to the copy's tests/engine.rs) ----

/// PB1: a silent session (no session_end) is not saved by a later session's session_end.
#[test]
fn probe_pb1_a_later_sessions_end_does_not_cover_a_silent_one() {
    let room: Vec<u64> = (1..=12).collect();
    let client = [1, 2, 5, 6, 9, 10, 11, 12];
    let ev = [
        ("admit", 1, "room"),
        ("admit", 5, "room"),
        ("session_end", 8, "room"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&room, &client, &ev));
    assert!(
        !valid && why.contains("admitted at tick 1") && why.contains("(no session_end records that it left)"),
        "{why}"
    );
}

/// PB2: two ends in one window: the first decides (decision 1).
#[test]
fn probe_pb2_the_first_session_end_in_the_window_decides() {
    let room = [&FIRST[..], &SECOND[..]].concat();
    let client = [&FIRST[..4], &SECOND[..]].concat();
    let ev = [
        ("admit", 1, "room"),
        ("session_end", 5, "room"),
        ("session_end", 6, "lab"),
        ("admit", 9, "room"),
    ];
    let (valid, why) = covered(&session_run(&room, &client, &ev));
    assert!(!valid && why.contains("(its session_end is at tick 5)"), "{why}");
}

/// PB3: an admit only a client-side log records cuts nothing.
#[test]
fn probe_pb3_a_client_side_admit_cuts_nothing() {
    let room: Vec<u64> = (1..=12).collect();
    let client = [1, 2, 3, 4, 7, 8, 9, 10, 11, 12];
    // The room's admit at 1; a client-side admit at 7 and a lab end at 6 would make 5 and 6 an allowed tail.
    let ev = [("admit", 1, "room"), ("session_end", 6, "lab"), ("admit", 7, "client")];
    let (valid, why) = covered(&session_run(&room, &client, &ev));
    assert!(!valid && why.contains("slot 0, tick 5"), "{why}");
}

/// One room log for -10 with several measured ticks and per-slot lists.
fn probe_sync_log(id: &'static str, clients: u32, works: &[i64], lists: &[Vec<i64>]) -> Log {
    let mut s = LogSpec::room(id, "set-s", "PM-0001-10").ticks("server_only_ns", works);
    s.clients = clients;
    let mut log = s.build();
    for (t, l) in log.ticks.iter_mut().zip(lists) {
        t.s.insert("relevant_count".into(), l.clone());
    }
    log
}

/// PB4, PB5: U is the work of every measured tick over the entities of every (tick, slot) entry.
#[test]
fn probe_pb4_pb5_the_unit_cost_sums_every_tick_and_every_slot() {
    // a48: tick 1 work 3,000 with 1 entity per slot (48), tick 2 work 6,600 with 2 per slot (96): 9,600/144.
    let mut logs = vec![probe_sync_log("a48", 48, &[3_000, 6_600], &[vec![1; 48], vec![2; 48]])];
    logs.extend(sync_runs(110, [100; 3], [105; 3]).into_iter().skip(1));
    let v = judge(&marks_file(&[sync_mark()]), &logs);
    assert_eq!(report(&v[0], "unit")[0], "66.666667", "{:?}", v[0].report);
}

/// PB6: R2's E is the mean over every (tick, slot) entry, not over ticks of the largest slot.
#[test]
fn probe_pb6_r2s_e_is_the_mean_entry() {
    let mut logs = egress_run(R2_COUNTS, [1_000, 1_000 + R2_BYTES], [0, R2_BYTES]);
    // Slots alternate 63 and 61: the mean entry is still 62.
    let alt: Vec<i64> = (0..48).map(|k| if k % 2 == 0 { 63 } else { 61 }).collect();
    logs[0].ticks[0].s.insert("relevant_count".into(), alt);
    let (line, note) = figure_check("r2_down_formula_bits", "==", json!("146122.56"), &logs);
    assert_eq!(line, "Pass 146122.56", "{note}");
}

/// PB9: each set pools its own on runs (ON-POOL-SET): another set's slow on runs never enter this one's pool.
#[test]
fn probe_pb9_each_set_pools_its_own_on_runs() {
    let mut logs = logging_runs([102, 101, 100]);
    let other = [
        ("m-on-1", "measured", 200),
        ("m-on-2", "measured", 200),
        ("m-on-3", "measured", 200),
        ("m-off-1", "logging-off", 100),
        ("m-off-2", "logging-off", 100),
        ("m-off-3", "logging-off", 100),
    ];
    for (id, role, slow) in other {
        let mut l = logging_run(id, role, &[90, slow]);
        l.header.set = "set-m".to_owned();
        logs.push(l);
    }
    let v = judge(&marks_file(&[logging_mark()]), &logs).remove(0);
    let lines: Vec<(String, String)> = v.parts[0]
        .runs
        .iter()
        .map(|r| {
            let c = &r.checks[0];
            (r.run.clone(), format!("{:?} {}", c.outcome, c.value.as_deref().unwrap_or("-")))
        })
        .collect();
    for (run, line) in &lines {
        let want = if run.starts_with("m-") { "Fail 2" } else { "Pass 1.02" };
        assert_eq!(line, want, "{run}: {lines:?}");
    }
}
