//! Figures: the computations a mark needs that README §2.3's statistics cannot express (ratios across scale steps,
//! slopes, arithmetic against a formula). Every name `marks.json` uses must be listed here; one that is listed but
//! not built yet says which unit builds it, and its check comes out NOT-RUN rather than guessed.

use crate::eval::{Missing, Run};
use crate::ratio::Ratio;
use npt_log::Phase;
use serde_json::Value;

/// Why a figure has no value.
pub enum Lack {
    /// A gap in the record: the run is INVALID (README §2.3).
    Gap(String),
    /// A run the figure compares with is not in the set yet: the part is NOT-RUN, as for a missing `against`
    /// reference (README §2.3).
    Waiting(String),
}

/// PM-0001-2: the least share, over client slots, of measured ticks on which that slot's snapshot held every other
/// soldier (`tick.s.everyone_in_view`, one 0/1 per slot, in slot order).
fn everyone_in_view_min_share(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let mut seen: Vec<i128> = Vec::new();
    let mut ticks: i128 = 0;
    for log in &run.logs {
        for t in log.ticks.iter().filter(|t| t.ph == Phase::Measured) {
            let Some(flags) = t.s.get("everyone_in_view") else {
                continue;
            };
            if seen.is_empty() {
                seen = vec![0; flags.len()];
            }
            if flags.len() != seen.len() {
                return Err(Lack::Gap(format!(
                    "run {}: tick {} changes the slot count of everyone_in_view",
                    run.id, t.t
                )));
            }
            for (s, f) in seen.iter_mut().zip(flags) {
                *s += i128::from(*f == 1);
            }
            ticks += 1;
        }
    }
    if ticks == 0 || seen.is_empty() {
        return Err(Lack::Gap(format!(
            "run {}: no values for tick.s.everyone_in_view",
            run.id
        )));
    }
    let least = seen.iter().copied().min().unwrap_or(0);
    Ok(Ratio::new(least, ticks))
}

/// One header param of a run: the single value its logs give it (`None` if no log has it), or a gap if two logs
/// disagree.
fn run_param<'r>(run: &'r Run<'_>, name: &str) -> Result<Option<&'r Value>, Lack> {
    let mut found: Option<&Value> = None;
    for log in &run.logs {
        if let Some(v) = log.header.params.get(name) {
            match found {
                Some(f) if f != v => {
                    return Err(Lack::Gap(format!(
                        "run {}: its logs disagree on params.{name}",
                        run.id
                    )));
                }
                _ => found = Some(v),
            }
        }
    }
    Ok(found)
}

/// A run's `params.fate_log_sha256`: 64 lowercase hex digits, or a gap.
fn fate_log_hash<'r>(run: &'r Run<'_>) -> Result<&'r str, Lack> {
    let hash = run_param(run, "fate_log_sha256")?.and_then(Value::as_str);
    match hash {
        Some(h)
            if h.len() == 64
                && h.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
        {
            Ok(h)
        }
        _ => Err(Lack::Gap(format!(
            "run {}: params.fate_log_sha256 is missing or not 64 lowercase hex digits",
            run.id
        ))),
    }
}

/// PM-0001-3 ("on L1, the same seed and profile give an identical packet-fate log on two runs"): 1 when the run's
/// `params.fate_log_sha256` equals its twin's, 0 when they differ. The twin is the run of the same set named in
/// `params.twin`, with role `twin` and the same layer, profile, seed and bite (a bite run's twin runs the same bite).
/// A run that names no twin, a twin that is not one or differs in those fields, and a missing hash are gaps; a twin not
/// in the set yet leaves the check NOT-RUN.
fn fate_log_twin_equal(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack> {
    let Some(name) = run_param(run, "twin")?.and_then(Value::as_str) else {
        return Err(Lack::Gap(format!(
            "run {}: params.twin names no twin run",
            run.id
        )));
    };
    let own = fate_log_hash(run)?;
    let Some(twin) = set.iter().find(|r| r.id == name) else {
        return Err(Lack::Waiting(format!(
            "run {}: its twin {name} is not in set {} yet",
            run.id, run.set
        )));
    };
    if twin.role != "twin" {
        return Err(Lack::Gap(format!(
            "run {}: {name} has role {}, not twin",
            run.id, twin.role
        )));
    }
    let (Some(a), Some(b)) = (run.logs.first(), twin.logs.first()) else {
        return Err(Lack::Gap(format!("run {}: a run without a log", run.id)));
    };
    let (a, b) = (&a.header, &b.header);
    for (field, same) in [
        ("layer", a.layer == b.layer),
        ("profile", a.profile == b.profile),
        ("seed", a.seed == b.seed),
        ("bite", run.bite == twin.bite),
    ] {
        if !same {
            return Err(Lack::Gap(format!(
                "run {}: its twin {name} has another {field}",
                run.id
            )));
        }
    }
    Ok(Ratio::int(i128::from(own == fate_log_hash(twin)?)))
}

/// A figure's computation: the run under judgement and every run of its set.
pub type Compute = fn(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack>;

/// One listed figure.
pub struct Figure {
    pub name: &'static str,
    /// The computation, or `None` while it is not built.
    pub compute: Option<Compute>,
    /// The unit that builds it, for a figure not built yet.
    pub unit: &'static str,
    /// What it computes, in plain words.
    pub what: &'static str,
}

const fn planned(name: &'static str, unit: &'static str, what: &'static str) -> Figure {
    Figure {
        name,
        compute: None,
        unit,
        what,
    }
}

/// The list: every figure `marks.json` names (`marks/SERIES.md` gives each one's formula, inputs and draft words).
/// Each is built in the unit that first produces its inputs; until then its check or report is NOT-RUN.
pub const FIGURES: &[Figure] = &[
    Figure {
        name: "fate_log_twin_equal",
        compute: Some(fate_log_twin_equal),
        unit: "U2",
        what: "the same seed gives the same packet-fate log (PM-0001-3)",
    },
    Figure {
        name: "everyone_in_view_min_share",
        compute: Some(everyone_in_view_min_share),
        unit: "U1",
        what: "the least share of a slot's snapshots with all 47 others in its set (-2)",
    },
    planned(
        "decoded_hash_mismatch_count",
        "U4",
        "decoded snapshots whose hash differs from the server's (-17, -19, -58)",
    ),
    planned(
        "decoded_hash_compared_count",
        "U4",
        "the (client, tick) pairs compared (-17)",
    ),
    planned(
        "twin_hash_mismatch_count",
        "U4",
        "ticks whose hash differs from the twin run's (-21, -30)",
    ),
    planned(
        "cross_arch_mismatch_count",
        "U4",
        "L2 mismatches the x86-64 replay does not reproduce (the Arm rule)",
    ),
    planned(
        "view_hit_registered_share",
        "U4",
        "shots hitting in the shooter's view that the server registered (-48, -49)",
    ),
    planned(
        "false_hit_share",
        "U4",
        "view misses registered as hits (-48)",
    ),
    planned(
        "rule_cap_set_by_rule",
        "U4",
        "the rule cap equals the picked option's P2 honest-rewind p99 in ticks (-49)",
    ),
    planned(
        "throw_desync_count",
        "U4",
        "throws that desync, the x86-64 replay agreeing (-54)",
    ),
    planned(
        "machine_named",
        "U4",
        "every header names its machine in full (-1, PM-0006-11)",
    ),
    planned(
        "sync_unit_cost",
        "U6",
        "mean server-only work per tick / (clients x mean relevant entities) (-10)",
    ),
    planned(
        "sync_cost_ratio",
        "U6",
        "N = 48's unit cost against N = 24's and 36's (-10)",
    ),
    planned(
        "down_client_mean_max_bits",
        "U6",
        "the largest client mean of the downstream (-12)",
    ),
    planned(
        "down_mean_ipv6_bits",
        "U6",
        "the downstream mean with IPv6's 20 B more per datagram (-12)",
    ),
    planned(
        "r2_down_formula_bits",
        "U6",
        "R2's downstream formula at the run's own counts (-12)",
    ),
    planned(
        "up_client_mean_max_bits",
        "U6",
        "the largest client mean of the upstream (-13)",
    ),
    planned(
        "rss_slope_bytes_per_10min",
        "U6",
        "the least-squares slope of RSS over the measured seconds (-15)",
    ),
    planned(
        "queue_growth_count",
        "U6",
        "per-client queues whose last-10-minute max exceeds the first's (-15)",
    ),
    planned(
        "session_memory_growth_count",
        "U6",
        "the same rule on per-session memory, over halves (-24)",
    ),
    planned(
        "kill_p99_ratio",
        "U6",
        "the other rooms' p99 after the SIGKILL against before (-27)",
    ),
    planned(
        "rtt_agreement_p99_ns",
        "U6",
        "p99 of |app RTT - quinn's rtt()| per second (-33)",
    ),
    planned(
        "bulk_control_p99_rise_ns",
        "U6",
        "the control stream's p99 delay with a bulk transfer minus without (-38)",
    ),
    planned(
        "one_shot_dropped_count",
        "U6",
        "one-shot actions sent minus received (-38)",
    ),
    planned(
        "crash_close_max_ns",
        "U6",
        "each client's RoomEnded or close after the tick panic, the slowest (-59)",
    ),
    planned(
        "egress_interface_bytes",
        "U6",
        "interface bytes sent in the room-hour (PM-0006-17)",
    ),
    planned(
        "egress_metric_vs_interface_ratio",
        "U6",
        "the room's bytes-sent metric against the interface counter (PM-0006-17)",
    ),
    planned(
        "r2_egress_formula_bytes",
        "U6",
        "R2's egress formula at the run's counts (PM-0006-17)",
    ),
    planned(
        "egress_vs_r2_ratio",
        "U6",
        "interface bytes against R2's formula (PM-0006-17)",
    ),
    planned(
        "logging_cost_ratio",
        "U6",
        "the pooled p99 with logging on against off (PM-0006-19)",
    ),
    planned(
        "alloc_twin_diff_count",
        "U6",
        "ticks whose allocation count differs from the logging-off twin's (PM-0006-20)",
    ),
    planned(
        "one_build_hash",
        "U7",
        "every browser run used the same client build (-32)",
    ),
    planned(
        "untimed_trap_count",
        "U7",
        "Worker traps with no timed rejoin after them (-19, the advisor's Q12)",
    ),
    planned(
        "input_latency_p99_rise_ns",
        "U7",
        "input-to-screen p99 at P4 minus at P0, the worst kind (-43)",
    ),
    planned(
        "input_latency_p50_rise_ns",
        "U7",
        "the same with medians (-43)",
    ),
    planned(
        "rooms_per_vm_count",
        "U8",
        "per N, the lowest of the sessions' largest k whose every room meets the marks (PM-0006-14)",
    ),
    planned(
        "times_of_day_rule",
        "U8",
        "the sessions start 6 h apart, one between 18:00 and 23:00 Amsterdam (-27, PM-0006-13, -14)",
    ),
];

pub fn known(name: &str) -> bool {
    FIGURES.iter().any(|f| f.name == name)
}

/// Computes a figure: a gap if its inputs are missing, not-built if its code is not there yet.
pub(crate) fn compute(name: &str, run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Missing> {
    match FIGURES.iter().find(|f| f.name == name) {
        Some(Figure {
            compute: Some(f), ..
        }) => f(run, set).map_err(|lack| match lack {
            Lack::Gap(why) => Missing::Gap(why),
            Lack::Waiting(why) => Missing::NotBuilt(why),
        }),
        Some(Figure { unit, .. }) => Err(Missing::NotBuilt(format!(
            "figure {name} is not built yet ({unit})"
        ))),
        None => Err(Missing::NotBuilt(format!(
            "figure {name} is not in the judge's list"
        ))),
    }
}
