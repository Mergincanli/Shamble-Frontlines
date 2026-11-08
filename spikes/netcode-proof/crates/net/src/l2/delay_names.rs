//! The names DelayStream reports under (marks/SERIES.md's "DelayStream" section; README §1's rule: lowercase, ending
//! in the unit, or a 0/1 flag; the advisor's M15 and u3c-plan.md 1.7). Directions are the model's ends: `down` is what
//! RS sends (model end 0), `up` what the client sends (end 1). Per second they are `second.v.<name>`, at a
//! connection's end the `ws_summary` event's fields, per burst the `burst_return` event's DelayStream fields, and the
//! header's `header.params.<name>`.

use super::stats::Summary;
use super::ws_frame::REASONS;
use crate::tcp_model::Counters;

/// `[direction][field]`: the TCP model's counts (S12) per direction, in `tcp_counts`' order. The three `_max_` ones
/// are the connection's largest so far (the model keeps maxima, not windows).
pub(crate) const TCP: [[&str; 20]; 2] = [
    [
        "ws_down_segments_count",
        "ws_down_transmissions_count",
        "ws_down_lost_profile_count",
        "ws_down_lost_burst_count",
        "ws_down_first_lost_count",
        "ws_down_pure_acks_count",
        "ws_down_pure_acks_lost_count",
        "ws_down_repairs_rack_count",
        "ws_down_repairs_probe_count",
        "ws_down_repairs_timeout_count",
        "ws_down_backoff_max_count",
        "ws_down_stalls_count",
        "ws_down_stall_max_ns",
        "ws_down_stall_sum_ns",
        "ws_down_ooo_held_max_bytes",
        "ws_down_discards_old_count",
        "ws_down_dsacks_count",
        "ws_down_undos_loss_count",
        "ws_down_undos_frto_count",
        "ws_down_undos_recovery_count",
    ],
    [
        "ws_up_segments_count",
        "ws_up_transmissions_count",
        "ws_up_lost_profile_count",
        "ws_up_lost_burst_count",
        "ws_up_first_lost_count",
        "ws_up_pure_acks_count",
        "ws_up_pure_acks_lost_count",
        "ws_up_repairs_rack_count",
        "ws_up_repairs_probe_count",
        "ws_up_repairs_timeout_count",
        "ws_up_backoff_max_count",
        "ws_up_stalls_count",
        "ws_up_stall_max_ns",
        "ws_up_stall_sum_ns",
        "ws_up_ooo_held_max_bytes",
        "ws_up_discards_old_count",
        "ws_up_dsacks_count",
        "ws_up_undos_loss_count",
        "ws_up_undos_frto_count",
        "ws_up_undos_recovery_count",
    ],
];

/// Which of TCP's fields are maxima (reported as they stand), not counts (reported per window).
pub(crate) const TCP_IS_MAX: [bool; 20] = [
    false, false, false, false, false, false, false, false, false, false, true, false, true, false,
    true, false, false, false, false, false,
];

/// One direction's model counts in TCP's order.
pub(crate) fn tcp_counts(c: &Counters) -> [u64; 20] {
    [
        c.segments,
        c.transmissions,
        c.lost_profile,
        c.lost_burst,
        c.first_lost,
        c.pure_acks,
        c.pure_acks_lost,
        c.repairs_rack,
        c.repairs_probe,
        c.repairs_timeout,
        u64::from(c.max_backoff),
        c.stalls,
        c.stall_ns_max,
        c.stall_ns_sum,
        c.held_bytes_max,
        c.discards_old,
        c.dsacks,
        c.undos_loss,
        c.undos_frto,
        c.undos_recovery,
    ]
}

/// `[direction]`: the bytes DelayStream holds (current, and the window's or connection's largest).
pub(crate) const HELD: [[&str; 2]; 2] = [
    ["ws_down_held_bytes", "ws_down_held_max_bytes"],
    ["ws_up_held_bytes", "ws_up_held_max_bytes"],
];

/// `[direction][statistic]`: the release lateness (M1's pair; C10): samples, p50, p99, p99.9, max, U3-Q1's
/// E[(L − L0 − 0.25 ms)+] with L0 the window's p50, and the samples beyond L0 + 0.25 ms.
pub(crate) const LATE: [[&str; 7]; 2] = [
    [
        "ws_late_down_count",
        "ws_late_down_p50_ns",
        "ws_late_down_p99_ns",
        "ws_late_down_p999_ns",
        "ws_late_down_max_ns",
        "ws_late_down_excess_ns",
        "ws_late_down_over_count",
    ],
    [
        "ws_late_up_count",
        "ws_late_up_p50_ns",
        "ws_late_up_p99_ns",
        "ws_late_up_p999_ns",
        "ws_late_up_max_ns",
        "ws_late_up_excess_ns",
        "ws_late_up_over_count",
    ],
];

/// The per-release split's parts (the advisor's "condition 10 again", 2026-11-08; delay.rs's module documentation), in
/// `Split::parts`' order: the woken class's clock, hop and work, then the other class's wait and work.
pub(crate) const PARTS: usize = 5;
/// The other class's releases by the step that released them: a pump pass's opening step that no clock wake covered,
/// the pump's step inside its pass (its up loop), an application task's step.
pub(crate) const KINDS: usize = 3;

/// One direction's split: each part's summary (its `n` is its class's count) and the other class's counts by kind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Split {
    pub(crate) parts: [Summary; PARTS],
    pub(crate) kinds: [u64; KINDS],
}

/// `[direction]`: the split's class counts, woken then other.
pub(crate) const SPLIT_COUNT: [[&str; 2]; 2] = [
    ["ws_late_down_woken_count", "ws_late_down_other_count"],
    ["ws_late_up_woken_count", "ws_late_up_other_count"],
];

/// `[direction][part][statistic]`: the part's p50, p99, p99.9, max, E[(x − x0 − 0.25 ms)+] with x0 its own p50, and
/// the samples beyond x0 + 0.25 ms (the lateness's statistics, through the same `stats::Running`).
pub(crate) const SPLIT: [[[&str; 6]; PARTS]; 2] = [
    [
        [
            "ws_late_down_clock_p50_ns",
            "ws_late_down_clock_p99_ns",
            "ws_late_down_clock_p999_ns",
            "ws_late_down_clock_max_ns",
            "ws_late_down_clock_excess_ns",
            "ws_late_down_clock_over_count",
        ],
        [
            "ws_late_down_hop_p50_ns",
            "ws_late_down_hop_p99_ns",
            "ws_late_down_hop_p999_ns",
            "ws_late_down_hop_max_ns",
            "ws_late_down_hop_excess_ns",
            "ws_late_down_hop_over_count",
        ],
        [
            "ws_late_down_work_p50_ns",
            "ws_late_down_work_p99_ns",
            "ws_late_down_work_p999_ns",
            "ws_late_down_work_max_ns",
            "ws_late_down_work_excess_ns",
            "ws_late_down_work_over_count",
        ],
        [
            "ws_late_down_other_wait_p50_ns",
            "ws_late_down_other_wait_p99_ns",
            "ws_late_down_other_wait_p999_ns",
            "ws_late_down_other_wait_max_ns",
            "ws_late_down_other_wait_excess_ns",
            "ws_late_down_other_wait_over_count",
        ],
        [
            "ws_late_down_other_work_p50_ns",
            "ws_late_down_other_work_p99_ns",
            "ws_late_down_other_work_p999_ns",
            "ws_late_down_other_work_max_ns",
            "ws_late_down_other_work_excess_ns",
            "ws_late_down_other_work_over_count",
        ],
    ],
    [
        [
            "ws_late_up_clock_p50_ns",
            "ws_late_up_clock_p99_ns",
            "ws_late_up_clock_p999_ns",
            "ws_late_up_clock_max_ns",
            "ws_late_up_clock_excess_ns",
            "ws_late_up_clock_over_count",
        ],
        [
            "ws_late_up_hop_p50_ns",
            "ws_late_up_hop_p99_ns",
            "ws_late_up_hop_p999_ns",
            "ws_late_up_hop_max_ns",
            "ws_late_up_hop_excess_ns",
            "ws_late_up_hop_over_count",
        ],
        [
            "ws_late_up_work_p50_ns",
            "ws_late_up_work_p99_ns",
            "ws_late_up_work_p999_ns",
            "ws_late_up_work_max_ns",
            "ws_late_up_work_excess_ns",
            "ws_late_up_work_over_count",
        ],
        [
            "ws_late_up_other_wait_p50_ns",
            "ws_late_up_other_wait_p99_ns",
            "ws_late_up_other_wait_p999_ns",
            "ws_late_up_other_wait_max_ns",
            "ws_late_up_other_wait_excess_ns",
            "ws_late_up_other_wait_over_count",
        ],
        [
            "ws_late_up_other_work_p50_ns",
            "ws_late_up_other_work_p99_ns",
            "ws_late_up_other_work_p999_ns",
            "ws_late_up_other_work_max_ns",
            "ws_late_up_other_work_excess_ns",
            "ws_late_up_other_work_over_count",
        ],
    ],
];

/// `[direction][kind]`: the other class's releases by the step that released them (`KINDS`' order).
pub(crate) const SPLIT_KIND: [[&str; KINDS]; 2] = [
    [
        "ws_late_down_other_open_count",
        "ws_late_down_other_mid_count",
        "ws_late_down_other_app_count",
    ],
    [
        "ws_late_up_other_open_count",
        "ws_late_up_other_mid_count",
        "ws_late_up_other_app_count",
    ],
];

/// The connection's own counts: closes by the scanner's reason (ws_frame::Reason's order), then the rest.
pub(crate) const CLOSE: [&str; REASONS] = [
    "ws_close_request_count",
    "ws_close_frame_count",
    "ws_close_message_count",
    "ws_close_unmasked_count",
    "ws_close_control_count",
    "ws_close_length_count",
];
pub(crate) const SHUTDOWN_TIMEOUT: &str = "ws_shutdown_timeout_count";
pub(crate) const SPLIT_WRITES: &str = "ws_split_writes_count";
pub(crate) const RETURNS_DROPPED: &str = "ws_returns_dropped_count";

/// The `ws_summary` event's own fields beside the per-second names.
pub(crate) const CONNS_BEFORE: &str = "ws_conns_before_count";
pub(crate) const ACCEPTED: &str = "ws_accepted_ns";
pub(crate) const WRITTEN: [&str; 2] = ["ws_down_written_bytes", "ws_up_written_bytes"];

/// The header's numbers.
pub(crate) const HEADER_NUMBERS: [&str; 22] = [
    "ws_wrapper",
    "ws_one_way_ns",
    "ws_half_jitter_ns",
    "ws_tcp_write_budget_bytes",
    "ws_tcp_mss_bytes",
    "ws_tcp_rto_min_ns",
    "ws_tcp_thin_linear_timeouts",
    "ws_tcp_timestamps",
    "ws_tcp_syn_rto_ns",
    "ws_tcp_syn_linear_timeouts_count",
    "ws_clock_timer_slack_ns",
    "ws_clock_timer_slack_start_ns",
    "ws_clock_timer_slack_set",
    "ws_max_message_bytes",
    "ws_max_request_bytes",
    "ws_record_max_bytes",
    "ws_record_overhead_bytes",
    "ws_up_read_floor_bytes",
    "ws_shutdown_wait_ns",
    "ws_bursts_max_count",
    "ws_returns_max_count",
    "ws_conn_fixed_bytes",
];

/// The header's strings (the slack error only when the write failed).
pub(crate) const HEADER_TEXTS: [&str; 7] = [
    "ws_tcp_model",
    "ws_tcp_not_modelled",
    "ws_clock_sched_policy",
    "ws_conn_index_rule",
    "ws_records",
    "ws_handshake",
    "ws_clock_timer_slack_error",
];

/// The `burst_return` event's DelayStream fields (u3c-plan.md C8), in `Return::row`'s order.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const RETURN: [&str; 10] = [
    "ws_return_ns",
    "outstanding_bytes",
    "srtt_ns",
    "srtt_sampled",
    "ws_untaken_bytes",
    "ws_owed_bytes",
    "ws_down_first_byte_ns",
    "ws_down_first_message_ns",
    "ws_up_first_byte_ns",
    "ws_up_first_message_ns",
];

/// One window's (or the connection's) rows in a fixed order: per direction the model's counts, the held bytes, the
/// lateness and its per-release split (the woken class's count and parts, then the other class's, then its kinds);
/// then the connection's own counts.
pub(crate) fn window(
    tcp: [[u64; 20]; 2],
    held: [[u64; 2]; 2],
    late: [Summary; 2],
    split: [Split; 2],
    own: &[u64; REASONS + 3],
    out: &mut Vec<(&'static str, u64)>,
) {
    for d in 0..2 {
        for (name, v) in TCP[d].iter().zip(tcp[d]) {
            out.push((name, v));
        }
        for (name, v) in HELD[d].iter().zip(held[d]) {
            out.push((name, v));
        }
        let s = late[d];
        for (name, v) in LATE[d]
            .iter()
            .zip([s.n, s.p50, s.p99, s.p999, s.max, s.excess, s.over])
        {
            out.push((name, v));
        }
        let sp = split[d];
        for (part, s) in sp.parts.iter().enumerate() {
            match part {
                0 => out.push((SPLIT_COUNT[d][0], s.n)),
                3 => out.push((SPLIT_COUNT[d][1], s.n)),
                _ => {}
            }
            for (name, v) in SPLIT[d][part]
                .iter()
                .zip([s.p50, s.p99, s.p999, s.max, s.excess, s.over])
            {
                out.push((name, v));
            }
        }
        for (name, v) in SPLIT_KIND[d].iter().zip(sp.kinds) {
            out.push((name, v));
        }
    }
    let names = CLOSE
        .iter()
        .chain([SHUTDOWN_TIMEOUT, SPLIT_WRITES, RETURNS_DROPPED].iter());
    for (name, v) in names.zip(own.iter()) {
        out.push((name, *v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_v() -> Vec<&'static str> {
        let mut out = Vec::new();
        window(
            [[0; 20]; 2],
            [[0; 2]; 2],
            [Summary::default(); 2],
            [Split::default(); 2],
            &[0; REASONS + 3],
            &mut out,
        );
        out.iter().map(|x| x.0).collect()
    }

    /// README §1's naming rule, and one name per quantity across every row DelayStream writes.
    #[test]
    fn every_name_follows_the_rule_once() {
        let mut names = all_v();
        names.extend([CONNS_BEFORE, ACCEPTED, WRITTEN[0], WRITTEN[1]]);
        let flags = [
            "ws_wrapper",
            "ws_tcp_thin_linear_timeouts",
            "ws_tcp_timestamps",
            "srtt_sampled",
        ];
        let mut every: Vec<&str> = names.clone();
        every.extend(HEADER_NUMBERS);
        every.extend(RETURN);
        for n in &every {
            assert!(
                n.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
                "{n}"
            );
            assert!(
                flags.contains(n)
                    || n.ends_with("_set")
                    || ["_ns", "_bytes", "_permille", "_count"]
                        .iter()
                        .any(|u| n.ends_with(u)),
                "{n} ends in no unit"
            );
        }
        let mut sorted = every.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), every.len(), "a name twice");
        // Per direction 20 model counts, 2 held, 7 lateness, and the split's 2 class counts, 5 parts of 6 statistics
        // and 3 kinds; then 6 close reasons and 3 more.
        assert_eq!(
            all_v().len(),
            2 * (20 + 2 + 7 + 2 + PARTS * 6 + KINDS) + REASONS + 3
        );
    }

    /// M15 and u3c-plan.md 1.7: every name DelayStream writes has its SERIES.md row (U3b's pattern in names.rs); a
    /// missing row fails here.
    #[test]
    fn every_name_has_a_series_row() {
        let series = include_str!("../../../../marks/SERIES.md");
        for n in all_v() {
            assert!(
                series.contains(&format!("| `second.v.{n}` |")),
                "no SERIES.md row for second.v.{n}"
            );
        }
        for n in [CONNS_BEFORE, ACCEPTED, WRITTEN[0], WRITTEN[1]] {
            assert!(
                series.contains(&format!("| `event.ws_summary.{n}` |")),
                "no SERIES.md row for event.ws_summary.{n}"
            );
        }
        for n in HEADER_NUMBERS.iter().chain(HEADER_TEXTS.iter()) {
            assert!(
                series.contains(&format!("| `header.params.{n}` |")),
                "no SERIES.md row for header.params.{n}"
            );
        }
        for n in RETURN {
            assert!(
                series.contains(&format!("| `event.burst_return.{n}` |")),
                "no SERIES.md row for event.burst_return.{n}"
            );
        }
        // The summary reuses every per-second name: the section's text says so.
        assert!(series.contains("**The `ws_summary` event**"));
    }
}
