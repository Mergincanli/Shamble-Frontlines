//! The names ImpairSocket reports under (marks/SERIES.md's rows; README §1's rule: lowercase, ending in the unit, or
//! a 0/1 flag). Per second they are `second.v.<name>`; over the whole run the same names except
//! `l2_release_cpu_permille` (the run carries `l2_release_cpu_ns` instead), plus the reorder shares, are the
//! `l2_summary` event's fields. Up is inbound (client to server: ImpairSocket sits on RS), down outbound.

use super::stats::{Counters, ERR_CLASSES, KINDS, Kind, Summary};

/// `[direction][kind]`: `l2_<up|down>_<kind>_count` (up has no send errors: those rows are never written).
pub(crate) const FATE: [[&str; KINDS]; 2] = [
    [
        "l2_up_delivered_count",
        "l2_up_lost_count",
        "l2_up_burst_count",
        "l2_up_blocked_count",
        "l2_up_shaper_drop_count",
        "l2_up_queue_drop_count",
        "l2_up_pool_drop_count",
        "l2_up_oversize_count",
        "l2_up_send_err_count",
        "l2_up_would_block_count",
    ],
    [
        "l2_down_delivered_count",
        "l2_down_lost_count",
        "l2_down_burst_count",
        "l2_down_blocked_count",
        "l2_down_shaper_drop_count",
        "l2_down_queue_drop_count",
        "l2_down_pool_drop_count",
        "l2_down_oversize_count",
        "l2_down_send_err_count",
        "l2_down_would_block_count",
    ],
];

/// `[direction][statistic]`: the release-lateness summary (M1): samples, p50, p99, p99.9, max, U3-Q1's
/// E[(L − L0 − 0.25 ms)+] with L0 the window's p50, and the samples beyond L0 + 0.25 ms.
pub(crate) const LATE: [[&str; 7]; 2] = [
    [
        "l2_late_up_count",
        "l2_late_up_p50_ns",
        "l2_late_up_p99_ns",
        "l2_late_up_p999_ns",
        "l2_late_up_max_ns",
        "l2_late_up_excess_ns",
        "l2_late_up_over_count",
    ],
    [
        "l2_late_down_count",
        "l2_late_down_p50_ns",
        "l2_late_down_p99_ns",
        "l2_late_down_p999_ns",
        "l2_late_down_max_ns",
        "l2_late_down_excess_ns",
        "l2_late_down_over_count",
    ],
];

pub(crate) const OVERTAKEN: [&str; 2] = ["l2_up_overtaken_count", "l2_down_overtaken_count"];
pub(crate) const REORDERED: [&str; 2] = ["l2_up_reordered_count", "l2_down_reordered_count"];
/// The send errors by class (M5), down only.
pub(crate) const SEND_ERR: [&str; ERR_CLASSES.len()] = [
    "l2_down_err_msgsize_count",
    "l2_down_err_nobufs_count",
    "l2_down_err_perm_count",
    "l2_down_err_unreach_count",
    "l2_down_err_refused_count",
    "l2_down_err_other_count",
];
pub(crate) const PEERS: &str = "l2_peers_count";
pub(crate) const PEER_EVICTED: &str = "l2_peer_evicted_count";
pub(crate) const PEER_OVERFLOW: &str = "l2_peer_overflow_count";
pub(crate) const FATE_LOG_FULL: &str = "l2_fate_log_full_count";
pub(crate) const RECV_ERR: &str = "l2_recv_err_count";
pub(crate) const POOL_FREE: &str = "l2_pool_free_count";
pub(crate) const QUEUED: [&str; 2] = ["l2_up_queued_bytes", "l2_down_queued_bytes"];
pub(crate) const CPU_PERMILLE: &str = "l2_release_cpu_permille";
pub(crate) const CPU_NS: &str = "l2_release_cpu_ns";

/// One window's counters and lateness as (name, value) pairs, in a fixed order.
pub(crate) fn window(c: &Counters, late: [Summary; 2], out: &mut Vec<(&'static str, u64)>) {
    for dir in 0..2 {
        for k in Kind::ALL {
            if dir == 0 && matches!(k, Kind::SendError | Kind::WouldBlock) {
                continue;
            }
            out.push((FATE[dir][k as usize], c.fates[dir][k as usize]));
        }
        out.push((OVERTAKEN[dir], c.overtaken[dir]));
        out.push((REORDERED[dir], c.reordered[dir]));
        let s = late[dir];
        let vals = [s.n, s.p50, s.p99, s.p999, s.max, s.excess, s.over];
        for (name, v) in LATE[dir].iter().zip(vals) {
            out.push((name, v));
        }
    }
    for (name, v) in SEND_ERR.iter().zip(c.send_err) {
        out.push((name, v));
    }
    out.push((PEER_EVICTED, c.peers_evicted));
    out.push((PEER_OVERFLOW, c.peer_overflow));
    out.push((FATE_LOG_FULL, c.fate_log_full));
    out.push((RECV_ERR, c.recv_err));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// README §1's naming rule, and one name per quantity.
    #[test]
    fn every_name_follows_the_rule_once() {
        let mut out = Vec::new();
        window(&Counters::default(), [Summary::default(); 2], &mut out);
        let mut names: Vec<&str> = out.iter().map(|x| x.0).collect();
        names.extend([PEERS, POOL_FREE, QUEUED[0], QUEUED[1], CPU_PERMILLE, CPU_NS]);
        for n in &names {
            assert!(n.starts_with("l2_"), "{n}");
            assert!(
                n.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
                "{n}"
            );
            assert!(
                ["_ns", "_bytes", "_permille", "_count"]
                    .iter()
                    .any(|u| n.ends_with(u)),
                "{n} ends in no unit"
            );
        }
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len(), "a name twice");
        // 8 kinds up + 10 down, 2 x (overtaken, reordered, 7 lateness), 6 error classes, 4 table counters.
        assert_eq!(out.len(), 8 + 10 + 2 * 9 + 6 + 4);
    }

    /// M15: every per-second name has its SERIES.md row (the `l2_summary` event reuses them but the CPU permille; its
    /// `l2_release_cpu_ns` is named in the section's text, and socket_tests checks every field it writes).
    #[test]
    fn every_per_second_name_has_a_series_row() {
        let series = include_str!("../../../../marks/SERIES.md");
        let mut out = Vec::new();
        window(&Counters::default(), [Summary::default(); 2], &mut out);
        let mut names: Vec<&str> = out.iter().map(|x| x.0).collect();
        names.extend([PEERS, POOL_FREE, QUEUED[0], QUEUED[1], CPU_PERMILLE]);
        for n in names {
            assert!(
                series.contains(&format!("| `second.v.{n}` |")),
                "no SERIES.md row for second.v.{n}"
            );
        }
        assert!(series.contains(&format!("`{CPU_NS}`")));
    }
}
