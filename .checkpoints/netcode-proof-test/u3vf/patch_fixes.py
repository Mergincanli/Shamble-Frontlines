# Throwaway diagnostics for u3-verify-fixes-model: patches the worktree copy of fixes.rs only.
import sys
p = sys.argv[1]
s = open(p).read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:70], s.count(old))
    s = s.replace(old, new)

rep("""    scaled_trigger: bool,
}
""", """    scaled_trigger: bool,
    /// DIAG: 0 parallel (as built), 1 browser endpoint lock (one attempt CONNECTING at a time), 2 restart (close the
    /// pending attempt, new SYN 10 ms later).
    lock: u8,
    /// DIAG: added to the client's srtt in the scaled trigger (an application-level RTT estimate).
    rtt_extra: u64,
}
""")
rep("""        gap.max(2 * m.srtt_ns(1).unwrap_or(100 * MS) + 100 * MS)""",
    """        gap.max(2 * (m.srtt_ns(1).unwrap_or(100 * MS) + o.rtt_extra) + 100 * MS)""")
rep("""    fresh_end: Option<u64>,
    delivered_seen: u64,
}

impl Conn {""", """    fresh_end: Option<u64>,
    delivered_seen: u64,
    dead: bool,
}

fn connecting(st: Step) -> bool {
    matches!(st, Step::Connecting | Step::Hello | Step::Flight | Step::Upgrade | Step::Switching)
}

impl Conn {""")
rep("""fn phase(seed: u64, burst_ns: u64, o: Opts) -> (u64, u32) {""",
    """fn phase(seed: u64, burst_ns: u64, o: Opts) -> (u64, u32) {
    let (a, b, _, _, _) = phase_d(seed, burst_ns, o);
    (a, b)
}

/// DIAG: also returns the winning connection's index, whether a takeover happened before the return, and conn 0's
/// [max backoff, max icsk_retransmits, TSecr backoff resets].
fn phase_d(seed: u64, burst_ns: u64, o: Opts) -> (u64, u32, usize, bool, [u64; 3]) {""")
rep("""        fresh_end: None,
        delivered_seen: 0,
    }];""", """        fresh_end: None,
        delivered_seen: 0,
        dead: false,
    }];
    let mut pre_takeover = false;""")
rep("""        for c in &conns {
            if let Some(e) = c.m.next_event() {""", """        for c in conns.iter().filter(|c| !c.dead) {
            if let Some(e) = c.m.next_event() {""")
rep("""        for c in conns.iter_mut() {
            c.m.advance(t);
        }""", """        for c in conns.iter_mut().filter(|c| !c.dead) {
            c.m.advance(t);
        }""")
rep("""        for (i, c) in conns.iter_mut().enumerate() {
            let d = c.m.delivered(1);""", """        for (i, c) in conns.iter_mut().enumerate() {
            if c.dead {
                continue;
            }
            let d = c.m.delivered(1);""")
rep("""        for (i, c) in conns.iter_mut().enumerate().skip(1) {
            if c.step == Step::Joined""", """        for (i, c) in conns.iter_mut().enumerate().skip(1) {
            if c.dead {
                continue;
            }
            if c.step == Step::Joined""")
rep("""                current = i;
                pending = false;""", """                current = i;
                if t < back {
                    pre_takeover = true;
                }
                pending = false;""")
rep("""        {
            attempts += 1;
            conns.push(Conn {
                m: TcpModel::connect(&p, seed, attempts, o.cfg, t),
                step: Step::Connecting,
                owed: [0, 0],
                fresh_end: None,
                delivered_seen: 0,
            });
            next_attempt = t + gap;
        }""", """        {
            let blocked = o.lock == 1 && conns.iter().skip(1).any(|c| !c.dead && connecting(c.step));
            if blocked {
                next_attempt = t + MS;
            } else {
                let mut syn_at = t;
                if o.lock == 2 {
                    for c in conns.iter_mut().skip(1) {
                        if !c.dead && connecting(c.step) {
                            c.dead = true;
                            syn_at = t + 10 * MS;
                        }
                    }
                }
                attempts += 1;
                conns.push(Conn {
                    m: TcpModel::connect(&p, seed, attempts, o.cfg, syn_at),
                    step: Step::Connecting,
                    owed: [0, 0],
                    fresh_end: None,
                    delivered_seen: 0,
                    dead: false,
                });
                next_attempt = t + gap;
            }
        }""")
rep("""        if conns
            .iter()
            .any(|c| c.fresh_end.is_some_and(|e| c.m.delivered(1) >= e))
        {
            return (t - back, attempts);
        }
    }""", """        if let Some(w) = conns
            .iter()
            .position(|c| !c.dead && c.fresh_end.is_some_and(|e| c.m.delivered(1) >= e))
        {
            let s0 = &conns[0].m.ends[0].snd;
            let d = [
                u64::from(conns[0].m.counters(0).max_backoff),
                u64::from(s0.diag_max_retransmits),
                s0.diag_ts_resets,
            ];
            return (t - back, attempts, w, pre_takeover, d);
        }
    }""")
# Opts constructions: add the two fields
rep("""    let o = |cfg: TcpConfig, reconnect: Option<u64>, adaptive: bool, scaled_trigger: bool| Opts {
        cfg,
        reconnect,
        adaptive,
        scaled_trigger,
    };""", """    let o = |cfg: TcpConfig, reconnect: Option<u64>, adaptive: bool, scaled_trigger: bool| Opts {
        cfg,
        reconnect,
        adaptive,
        scaled_trigger,
        lock: 0,
        rtt_extra: 0,
    };""")
rep("""        adaptive: true,
        scaled_trigger: true,
    };
    let (r, _) = sweep(o);""", """        adaptive: true,
        scaled_trigger: true,
        lock: 0,
        rtt_extra: 0,
    };
    let (r, _) = sweep(o);""")
rep("""        adaptive,
        scaled_trigger: scaled,
    };""", """        adaptive,
        scaled_trigger: scaled,
        lock: 0,
        rtt_extra: 0,
    };""")
s += open(sys.argv[2]).read()
open(p, 'w').write(s)
print("ok")
