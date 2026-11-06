
// ---- DIAG (u3-verify-fixes-model; throwaway, worktree only) ----

fn sweep_d(o: Opts) -> Vec<(u64, u64, usize, bool, [u64; 3])> {
    (0..1_000u64)
        .map(|i| {
            let burst = 5 * SEC + ((i * 618_033_989) % SEC) * 2;
            let (r, a, w, pre, d) = phase_d(1_000 + i, burst, o);
            let _ = a;
            (r, 1_000 + i, w, pre, d)
        })
        .collect()
}

fn summary(name: &str, o: Opts) {
    let mut v = sweep_d(o);
    v.sort_unstable_by_key(|x| x.0);
    let r: Vec<u64> = v.iter().map(|x| x.0).collect();
    let over = |x: u64| r.iter().filter(|&&q| q > x).count();
    let by_new = v.iter().filter(|x| x.2 > 0).count();
    let pre = v.iter().filter(|x| x.3).count();
    let worst: Vec<String> = v
        .iter()
        .rev()
        .take(5)
        .map(|x| format!("{}ms s{} w{} b{} rx{} ts{}", x.0 / MS, x.1, x.2, x.4[0], x.4[1], x.4[2]))
        .collect();
    println!(
        "DIAG {name}: p50 {} p99 {} max {} ms; over 1 s {}, over 1.5 s {}; won by a new conn {}, pre-burst takeovers {}; worst {:?}",
        rank(&r, 500) / MS,
        rank(&r, 990) / MS,
        r.last().copied().unwrap_or(0) / MS,
        over(SEC),
        over(1_500 * MS),
        by_new,
        pre,
        worst
    );
}

#[test]
#[ignore]
fn diag_table() {
    let base = TcpConfig::default();
    let small = TcpConfig { write_budget: SMALL_BUDGET, ..base };
    let thin = TcpConfig { thin_linear_timeouts: true, ..small };
    let lk = |c: TcpConfig| TcpConfig { linux_karn: true, ..c };
    let lkts = |c: TcpConfig| TcpConfig { linux_karn: true, ts_samples: true, ..c };
    let ts = |c: TcpConfig| TcpConfig { ts_samples: true, ..c };
    let rr = |c: TcpConfig| TcpConfig { rto_mark_rack: true, ..c };
    let all = |c: TcpConfig| TcpConfig { linux_karn: true, ts_samples: true, rto_mark_rack: true, ..c };
    let r = Some(300 * MS);
    let mk = |cfg: TcpConfig, reconnect: Option<u64>, adaptive: bool, scaled: bool, lock: u8, extra: u64| Opts {
        cfg,
        reconnect,
        adaptive,
        scaled_trigger: scaled,
        lock,
        rtt_extra: extra,
    };
    let rec = |cfg: TcpConfig| mk(cfg, r, true, true, 0, 0);
    let rows: Vec<(&str, Opts)> = vec![
        ("REC as built", rec(base)),
        ("REC + browser endpoint lock (serialized)", mk(base, r, true, true, 1, 0)),
        ("REC + restart (close pending, every 300 ms)", mk(base, r, true, true, 2, 0)),
        ("REC + app RTT +17 ms", mk(base, r, true, true, 0, 17 * MS)),
        ("REC + app RTT +33 ms", mk(base, r, true, true, 0, 33 * MS)),
        ("REC linux_karn", rec(lk(base))),
        ("REC linux_karn+ts", rec(lkts(base))),
        ("REC rto_mark_rack", rec(rr(base))),
        ("REC all three model fixes", rec(all(base))),
        ("REC all three + lock", mk(all(base), r, true, true, 1, 0)),
        ("REC all three + lock + app RTT +17", mk(all(base), r, true, true, 1, 17 * MS)),
        ("adaptive, no reconnect, as built", mk(base, None, true, false, 0, 0)),
        ("adaptive, no reconnect, all three", mk(all(base), None, true, false, 0, 0)),
        ("defaults as built", mk(base, None, false, false, 0, 0)),
        ("defaults ts (model Karn + TSecr)", mk(ts(base), None, false, false, 0, 0)),
        ("defaults linux_karn", mk(lk(base), None, false, false, 0, 0)),
        ("defaults linux_karn+ts", mk(lkts(base), None, false, false, 0, 0)),
        ("defaults all three", mk(all(base), None, false, false, 0, 0)),
        ("fix1 alone as built", mk(small, None, false, false, 0, 0)),
        ("fix1 alone ts (model Karn + TSecr)", mk(ts(small), None, false, false, 0, 0)),
        ("fix1 alone linux_karn", mk(lk(small), None, false, false, 0, 0)),
        ("fix1 alone linux_karn+ts", mk(lkts(small), None, false, false, 0, 0)),
        ("fix1 alone all three", mk(all(small), None, false, false, 0, 0)),
        ("fix1+2 as built", mk(thin, None, false, false, 0, 0)),
        ("fix1+2 all three", mk(all(thin), None, false, false, 0, 0)),
        ("fix1+2 rto100 as built", mk(TcpConfig { rto_min_us: 100_000, ..thin }, None, false, false, 0, 0)),
        ("fix1+2 rto50 as built", mk(TcpConfig { rto_min_us: 50_000, ..thin }, None, false, false, 0, 0)),
        ("fix1+2 rto50 ts", mk(ts(TcpConfig { rto_min_us: 50_000, ..thin }), None, false, false, 0, 0)),
        ("fix3 alone as built", mk(base, r, false, false, 0, 0)),
        ("fix3 alone + lock", mk(base, r, false, false, 1, 0)),
        ("fix1+2+3 + lock", mk(thin, r, false, false, 1, 0)),
    ];
    for (name, o) in rows {
        summary(name, o);
    }
}

#[test]
#[ignore]
fn diag_side_effects_rec() {
    // The recommended configuration exactly (thin linear timeouts OFF), and with app RTT +17 ms.
    for extra in [0, 17 * MS] {
        for p in [Profile::p1(), Profile::p2(), Profile::p3(), Profile::p4()] {
            let (w, n, s) = (1..=3).fold((0, 0, 0), |(w, n, s), seed| {
                let (a, b, c) = snapshot_share_x(&p, TcpConfig::default(), seed, extra);
                (w + a, n + b, s + c)
            });
            println!(
                "DIAG side REC extra {} ms {}: writes {w} of {n} ({} permille skipped); silences {s} in 3 min",
                extra / MS,
                p.name,
                (n - w) * 1000 / n
            );
        }
    }
}

fn snapshot_share_x(p: &Profile, cfg: TcpConfig, seed: u64, extra: u64) -> (u64, u64, u64) {
    let o = Opts {
        cfg,
        reconnect: Some(300 * MS),
        adaptive: true,
        scaled_trigger: true,
        lock: 0,
        rtt_extra: extra,
    };
    let mut m = TcpModel::with_config(p, seed, 0, cfg);
    let (mut written, mut ticks, mut silences) = (0, 0, 0);
    let (mut last_seen, mut last_data, mut in_silence) = (0u64, TICK, false);
    let mut t = TICK;
    while t <= 60 * SEC {
        m.advance(t);
        let d = m.delivered(1);
        if d > last_seen {
            last_seen = d;
            last_data = t;
            in_silence = false;
        } else if t >= last_data + trigger(&o, 300 * MS, &m) && !in_silence {
            silences += 1;
            in_silence = true;
        }
        ticks += 1;
        if m.free_budget(0) >= SNAP && m.outstanding(0) + SNAP <= adaptive_limit(&m) {
            m.write(0, SNAP, t);
            written += 1;
        }
        m.write(1, INPUT, t + TICK / 2);
        t += TICK;
    }
    (written, ticks, silences)
}

#[test]
#[ignore]
fn diag_srtt() {
    // The ends' srtt after 5 s of normal play at each profile, as built vs Linux's Karn rule (oldest sample).
    for lkv in [false, true] {
        for p in [Profile::p1(), Profile::p2(), Profile::p3(), Profile::p4()] {
            let cfg = TcpConfig { linux_karn: lkv, ..TcpConfig::default() };
            let mut acc = (0u64, 0u64);
            for seed in 1..=20u64 {
                let mut m = TcpModel::with_config(&p, seed, 0, cfg);
                let mut t = TICK;
                while t <= 5 * SEC {
                    m.advance(t);
                    if m.free_budget(0) >= SNAP && m.outstanding(0) + SNAP <= adaptive_limit(&m) {
                        m.write(0, SNAP, t);
                    }
                    m.write(1, INPUT, t + TICK / 2);
                    t += TICK;
                }
                acc.0 += m.srtt_ns(0).unwrap_or(0);
                acc.1 += m.srtt_ns(1).unwrap_or(0);
            }
            println!(
                "DIAG srtt linux_karn {lkv} {} (rtt {} ms): server {} ms, client {} ms (mean of 20)",
                p.name,
                p.rtt_ns / MS,
                acc.0 / 20 / MS,
                acc.1 / 20 / MS
            );
        }
    }
}
