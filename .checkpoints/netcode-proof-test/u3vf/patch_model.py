# Throwaway diagnostics for u3-verify-fixes-model: patches the worktree copy of tcp_model.rs only.
import sys
p = sys.argv[1]
s = open(p).read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:60], s.count(old))
    s = s.replace(old, new)

rep("""    pub rto_min_us: u64,
}
""", """    pub rto_min_us: u64,
    /// DIAG: Linux's Karn rule (no seq sample if any cum-acked skb was resent; sample from the oldest, first_ackt).
    pub linux_karn: bool,
    /// DIAG: TSecr RTT samples when Karn removes the seq sample and the ACK newly acks data (tcp_ack_update_rtt).
    pub ts_samples: bool,
    /// DIAG: tcp_timeout_mark_lost's RACK exception and cwnd = in flight + 1 on an RTO.
    pub rto_mark_rack: bool,
}
""")
rep("""            rto_min_us: RTO_MIN_US.unsigned_abs(),
        }""", """            rto_min_us: RTO_MIN_US.unsigned_abs(),
            linux_karn: false,
            ts_samples: false,
            rto_mark_rack: false,
        }""")
rep("""    retransmits: u32,
    /// The constant-repair bite's fixed timeout (tests only).""", """    retransmits: u32,
    linux_karn: bool,
    ts_samples: bool,
    rto_mark_rack: bool,
    /// DIAG counters.
    pub diag_ts_resets: u64,
    pub diag_max_retransmits: u32,
    /// The constant-repair bite's fixed timeout (tests only).""")
rep("""            retransmits: 0,
            write_seq: 0,""", """            retransmits: 0,
            linux_karn: cfg.linux_karn,
            ts_samples: cfg.ts_samples,
            rto_mark_rack: cfg.rto_mark_rack,
            diag_ts_resets: 0,
            diag_max_retransmits: 0,
            write_seq: 0,""")
# on_ack: record cum-acked separately
rep("""        let mut delivered: Vec<(u64, bool, u64)> = Vec::new();
        while s.segs.front().is_some_and(|g| g.end <= ack.cum) {
            if let Some(g) = s.segs.pop_front()
                && !g.sacked
            {
                delivered.push((g.xmit_ns, g.retrans, g.end));
            }
        }""", """        let mut delivered: Vec<(u64, bool, u64)> = Vec::new();
        let mut cum_n = 0usize;
        while s.segs.front().is_some_and(|g| g.end <= ack.cum) {
            if let Some(g) = s.segs.pop_front()
                && !g.sacked
            {
                delivered.push((g.xmit_ns, g.retrans, g.end));
                cum_n += 1;
            }
        }""")
rep("""        // RTT: the newest newly delivered segment that was never resent (Karn).
        if let Some(&(x, _, _)) = delivered.iter().filter(|d| !d.1).max_by_key(|d| (d.0, d.2)) {
            let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
            s.rtt.sample(m, s.snd_una, s.snd_nxt, s.rto_min_us);
            s.backoff = 0;
        }""", """        // RTT: the newest newly delivered segment that was never resent (Karn).
        if !s.linux_karn {
            if let Some(&(x, _, _)) = delivered.iter().filter(|d| !d.1).max_by_key(|d| (d.0, d.2)) {
                let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
                s.rtt.sample(m, s.snd_una, s.snd_nxt, s.rto_min_us);
                s.backoff = 0;
            } else if s.ts_samples && cum_n > 0 {
                let x = delivered[0].0;
                let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
                s.rtt.sample(m, s.snd_una, s.snd_nxt, s.rto_min_us);
                if s.backoff > 0 { s.diag_ts_resets += 1; }
                s.backoff = 0;
            }
        } else {
            let cum = &delivered[..cum_n];
            let sacked = &delivered[cum_n..];
            let any_retrans = cum.iter().any(|d| d.1);
            let seq = if !any_retrans { cum.iter().filter(|d| !d.1).map(|d| d.0).next() } else { None };
            let sack = sacked.iter().filter(|d| !d.1).map(|d| d.0).min();
            let pick = seq.or(sack);
            if let Some(x) = pick {
                let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
                s.rtt.sample(m, s.snd_una, s.snd_nxt, s.rto_min_us);
                s.backoff = 0;
            } else if s.ts_samples && cum_n > 0 {
                let x = cum[0].0;
                let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
                s.rtt.sample(m, s.snd_una, s.snd_nxt, s.rto_min_us);
                if s.backoff > 0 { s.diag_ts_resets += 1; }
                s.backoff = 0;
            }
        }""")
rep("""        for g in s.segs.iter_mut().filter(|g| !g.sacked) {
            g.lost = true;
            g.outstanding = false;
            g.cause = Some(Repair::Timeout);
        }
        s.cwnd = 1;""", """        let rack_rtt = if s.rack.set { s.rack.rtt_us } else { 0 };
        let mark_rack = s.rto_mark_rack;
        for (k, g) in s.segs.iter_mut().enumerate().filter(|(_, g)| !g.sacked) {
            if mark_rack && k != 0 && rack_rtt > (now - g.xmit_ns) / US {
                continue;
            }
            g.lost = true;
            g.outstanding = false;
            g.cause = Some(Repair::Timeout);
        }
        s.cwnd = if mark_rack { s.segs.iter().filter(|g| g.outstanding).count() as u64 + 1 } else { 1 };""")
rep("""        s.retransmits += 1;
        // tcp_retransmit_timer's""", """        s.retransmits += 1;
        s.diag_max_retransmits = s.diag_max_retransmits.max(s.retransmits);
        // tcp_retransmit_timer's""")
open(p, 'w').write(s)
print("ok")
