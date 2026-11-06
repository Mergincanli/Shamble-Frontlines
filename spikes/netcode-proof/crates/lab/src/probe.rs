//! The L1 probe of PM-0001-3 (ADR 0001's row; plan §6.2 and R-6): datagrams through L1 at 10,000 per second each
//! way until each direction has sent 1,000,000 outside any burst, then each direction's loss and one-way delays and
//! the round trips, per second of send time, under `marks/SERIES.md`'s probe names.
//!
//! The probe measures only what its two ends receive, through npt-link's interface, as a real probe would; it never
//! reads L1's internals. The client sends up datagram i at 50 µs + i × 100 µs and the server sends down datagram i at
//! the same instant. Each down datagram also echoes the newest up datagram the server has received and not yet echoed,
//! with the time it held it, so the client reads a round trip as the echo's arrival minus the echoed datagram's send
//! time minus the hold (E10's ping with hold time). On L1 both ends share the simulated clock, so a one-way delay is
//! the arrival stamp minus the send time the datagram carries. L2's probe (U3, U6) keeps this design and takes its
//! one-way delays from `ImpairSocket`'s stamps on RS's clock instead.

use npt_l1::{FateLog, L1Net, Profile};
use npt_link::{Link, SendResult};
use npt_log::{Phase, SecondRecord};
use sha2::{Digest, Sha256};

pub const SEC: u64 = 1_000_000_000;
/// L2's probe rate (the advisor's reading, 2026-11-06), kept on L1 so both layers count the same way.
pub const RATE_PPS: u64 = 10_000;
/// Per direction, outside any burst (R-6: both layers, so PM-0001-3 and PM-0002-49 read one count).
pub const PACKETS: u64 = 1_000_000;
/// The probe's datagrams fit any link; L1 refuses nothing at this size.
const MAX_DATAGRAM: usize = 1200;
const MAGIC: &[u8; 4] = b"NPTP";
const LEN: usize = 40;
const NONE: u64 = u64::MAX;
/// After the last send, how long the probe keeps polling: beyond any datagram's delay on any profile, a bite's
/// doubled one included, plus a held datagram's wait.
const DRAIN_NS: u64 = 2 * SEC;

/// One run's settings.
pub struct Setup {
    pub profile: Profile,
    pub seed: u64,
    /// Datagrams per direction outside any burst.
    pub packets: u64,
    pub rate_pps: u64,
    /// A planted defect's id (bites builds only).
    pub bite: Option<String>,
}

/// What one run measured.
#[derive(Debug)]
pub struct Outcome {
    pub seconds: Vec<SecondRecord>,
    pub fate_log_sha256: String,
    /// Per direction (up, down): (datagrams delivered, of them reordered), from the fate log.
    pub reorder: [(u64, u64); 2],
    /// Datagrams sent per direction, bursts included.
    pub sent: u64,
    pub summary: Summary,
}

/// The run's headline numbers, for the one-line report (the judge reads the log, never these).
#[derive(Debug)]
pub struct Summary {
    pub lost: [u64; 2],
    pub mean_delay_ns: [u64; 2],
    pub mean_rtt_ns: u64,
    pub rtt_samples: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Datagram {
    /// 0 up, 1 down.
    dir: u8,
    index: u64,
    sent_ns: u64,
    /// The echoed up datagram's index, or NONE.
    echo: u64,
    hold_ns: u64,
}

fn encode(d: &Datagram) -> [u8; LEN] {
    let mut b = [0u8; LEN];
    b[0..4].copy_from_slice(MAGIC);
    b[4] = d.dir;
    b[8..16].copy_from_slice(&d.index.to_le_bytes());
    b[16..24].copy_from_slice(&d.sent_ns.to_le_bytes());
    b[24..32].copy_from_slice(&d.echo.to_le_bytes());
    b[32..40].copy_from_slice(&d.hold_ns.to_le_bytes());
    b
}

fn decode(b: &[u8]) -> Result<Datagram, String> {
    if b.len() != LEN || &b[0..4] != MAGIC || b[4] > 1 {
        return Err(format!("not a probe datagram ({} bytes)", b.len()));
    }
    let word = |at: usize| {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[at..at + 8]);
        u64::from_le_bytes(w)
    };
    Ok(Datagram {
        dir: b[4],
        index: word(8),
        sent_ns: word(16),
        echo: word(24),
        hold_ns: word(32),
    })
}

fn in_burst(p: &Profile, t: u64) -> bool {
    p.bursts_ns
        .iter()
        .any(|&(start, len)| t >= start && t - start < len)
}

/// Datagram i's send time, both directions.
fn send_time(i: u64, interval: u64) -> u64 {
    interval / 2 + i * interval
}

/// Runs one probe on L1 and returns its per-second records and the fate log's hash.
pub fn run(setup: &Setup) -> Result<Outcome, String> {
    if setup.rate_pps == 0 || !SEC.is_multiple_of(setup.rate_pps) {
        return Err(format!("rate {} does not divide a second", setup.rate_pps));
    }
    let interval = SEC / setup.rate_pps;
    let profile = &setup.profile;
    // Enough datagrams that `packets` of them are sent outside the bursts.
    let mut total = 0u64;
    let mut outside = 0u64;
    while outside < setup.packets {
        if !in_burst(profile, send_time(total, interval)) {
            outside += 1;
        }
        total += 1;
    }
    let net = match &setup.bite {
        None => L1Net::new(setup.seed),
        Some(id) => with_bite(setup.seed, id)?,
    };
    let mut client = net.connect(profile.clone(), MAX_DATAGRAM, 0);
    let mut listener = net.listener();
    let mut server: Option<Box<dyn Link>> = None;
    let n = usize::try_from(total).map_err(|_| "too many datagrams".to_owned())?;
    let mut up_delay = vec![NONE; n];
    let mut down_delay = vec![NONE; n];
    let mut rtt = vec![NONE; n];
    // The newest up datagram the server received and has not echoed: (index, arrival).
    let mut newest: Option<(u64, u64)> = None;
    let end = send_time(total - 1, interval) + DRAIN_NS;
    let mut t = send_time(0, interval);
    let mut i = 0u64;
    while t <= end {
        if server.is_none() {
            server = listener.poll_accept(t).map(|(l, _)| l);
        }
        let s = server
            .as_mut()
            .ok_or("the probe's server end was not accepted")?;
        while let Some(r) = s.poll_recv(t) {
            let d = decode(&r.bytes)?;
            let slot = up_delay
                .get_mut(d.index as usize)
                .filter(|v| d.dir == 0 && **v == NONE)
                .ok_or(format!(
                    "up datagram {} unexpected or delivered twice",
                    d.index
                ))?;
            *slot = r.arrival_ns - d.sent_ns;
            newest = Some((d.index, r.arrival_ns));
        }
        while let Some(r) = client.poll_recv(t) {
            let d = decode(&r.bytes)?;
            let slot = down_delay
                .get_mut(d.index as usize)
                .filter(|v| d.dir == 1 && **v == NONE)
                .ok_or(format!(
                    "down datagram {} unexpected or delivered twice",
                    d.index
                ))?;
            *slot = r.arrival_ns - d.sent_ns;
            if d.echo != NONE {
                let echoed_sent = send_time(d.echo, interval);
                rtt[d.index as usize] = r.arrival_ns - echoed_sent - d.hold_ns;
            }
        }
        if i < total {
            let up = Datagram {
                dir: 0,
                index: i,
                sent_ns: t,
                echo: NONE,
                hold_ns: 0,
            };
            let (echo, hold_ns) = match newest.take() {
                Some((k, arrived)) => (k, t - arrived),
                None => (NONE, 0),
            };
            let down = Datagram {
                dir: 1,
                index: i,
                sent_ns: t,
                echo,
                hold_ns,
            };
            if client.send_unreliable(&encode(&up), t) != SendResult::Sent
                || s.send_unreliable(&encode(&down), t) != SendResult::Sent
            {
                return Err(format!("datagram {i} was refused"));
            }
            i += 1;
        }
        t += interval;
    }
    drop(server);
    let (fate_log_sha256, reorder) =
        net.with_fate_log(|log| (sha256_hex(log), log.reorder_counts()));
    let seconds = per_second(profile, interval, &up_delay, &down_delay, &rtt);
    let summary = summarize(&up_delay, &down_delay, &rtt);
    Ok(Outcome {
        seconds,
        fate_log_sha256,
        reorder,
        sent: total,
        summary,
    })
}

#[cfg(feature = "bites")]
fn with_bite(seed: u64, id: &str) -> Result<L1Net, String> {
    L1Net::with_bite(seed, id).ok_or(format!(
        "npt-l1 plants no bite {id:?} (it plants {:?})",
        L1Net::BITES
    ))
}

#[cfg(not(feature = "bites"))]
fn with_bite(_seed: u64, id: &str) -> Result<L1Net, String> {
    Err(format!(
        "--bite {id}: this is a measured build, which holds no bites (build npt-lab with --features bites)"
    ))
}

fn sha256_hex(log: &FateLog) -> String {
    let mut h = Sha256::new();
    log.for_each_canonical(|b| h.update(b));
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn as_i64(v: u64) -> i64 {
    i64::try_from(v).unwrap_or(i64::MAX)
}

/// The records: one per second of send time, every name on every record (SERIES.md's rule), lists possibly empty.
fn per_second(
    profile: &Profile,
    interval: u64,
    up: &[u64],
    down: &[u64],
    rtt: &[u64],
) -> Vec<SecondRecord> {
    let mut out: Vec<SecondRecord> = Vec::new();
    let names = [
        "probe_up_lost",
        "probe_up_delay_ns",
        "probe_burst_up_lost",
        "probe_down_lost",
        "probe_down_delay_ns",
        "probe_burst_down_lost",
        "probe_rtt_ns",
    ];
    for i in 0..up.len() {
        let t = send_time(i as u64, interval);
        let sec = t / SEC;
        if out.last().is_none_or(|r| r.sec != sec) {
            let mut r = SecondRecord::new(sec, Phase::Measured);
            for name in names {
                r.s.insert(name.to_owned(), Vec::new());
            }
            out.push(r);
        }
        let r = out.last_mut().expect("pushed above");
        let burst = in_burst(profile, t);
        for (lost, delay, burst_lost, d) in [
            (
                "probe_up_lost",
                "probe_up_delay_ns",
                "probe_burst_up_lost",
                up[i],
            ),
            (
                "probe_down_lost",
                "probe_down_delay_ns",
                "probe_burst_down_lost",
                down[i],
            ),
        ] {
            let flag = i64::from(d == NONE);
            let list = if burst { burst_lost } else { lost };
            r.s.get_mut(list)
                .expect("every name is on the record")
                .push(flag);
            if d != NONE {
                r.s.get_mut(delay)
                    .expect("every name is on the record")
                    .push(as_i64(d));
            }
        }
        if rtt[i] != NONE {
            r.s.get_mut("probe_rtt_ns")
                .expect("every name is on the record")
                .push(as_i64(rtt[i]));
        }
    }
    out
}

fn summarize(up: &[u64], down: &[u64], rtt: &[u64]) -> Summary {
    let stats = |xs: &[u64]| {
        let got: Vec<u64> = xs.iter().copied().filter(|&v| v != NONE).collect();
        let lost = (xs.len() - got.len()) as u64;
        let sum: u128 = got.iter().map(|&v| u128::from(v)).sum();
        let mean = if got.is_empty() {
            0
        } else {
            (sum / got.len() as u128) as u64
        };
        (lost, mean, got.len() as u64)
    };
    let (up_lost, up_mean, _) = stats(up);
    let (down_lost, down_mean, _) = stats(down);
    let (_, rtt_mean, rtt_n) = stats(rtt);
    Summary {
        lost: [up_lost, down_lost],
        mean_delay_ns: [up_mean, down_mean],
        mean_rtt_ns: rtt_mean,
        rtt_samples: rtt_n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(profile: Profile, seed: u64, packets: u64) -> Setup {
        Setup {
            profile,
            seed,
            packets,
            rate_pps: RATE_PPS,
            bite: None,
        }
    }

    fn series(o: &Outcome, name: &str) -> Vec<i64> {
        o.seconds
            .iter()
            .flat_map(|r| {
                r.s.get(name)
                    .expect("every record has every name")
                    .iter()
                    .copied()
            })
            .collect()
    }

    #[test]
    fn a_probe_datagram_round_trips_and_nothing_else_decodes() {
        let d = Datagram {
            dir: 1,
            index: 7,
            sent_ns: 8,
            echo: 9,
            hold_ns: 10,
        };
        assert_eq!(decode(&encode(&d)), Ok(d));
        assert!(decode(&encode(&d)[..39]).is_err());
        let mut bad = encode(&d);
        bad[0] = b'X';
        assert!(decode(&bad).is_err());
        bad = encode(&d);
        bad[4] = 2;
        assert!(decode(&bad).is_err());
    }

    #[test]
    fn a_short_p2_probe_counts_every_datagram_and_measures_the_profile() {
        let o = run(&setup(Profile::p2(), 1, 30_000)).unwrap();
        assert_eq!(o.sent, 30_000);
        assert_eq!(
            o.seconds.len(),
            3,
            "three seconds of send time at 10,000 a second"
        );
        for r in &o.seconds {
            assert_eq!(r.s.len(), 7, "every name on every record");
            assert_eq!(r.s["probe_up_lost"].len(), 10_000);
        }
        let up_lost = series(&o, "probe_up_lost");
        let up_delay = series(&o, "probe_up_delay_ns");
        let lost = up_lost.iter().filter(|&&f| f == 1).count();
        assert_eq!(up_lost.len(), 30_000);
        assert_eq!(
            up_delay.len(),
            30_000 - lost,
            "one delay per delivered datagram"
        );
        assert_eq!(lost as u64, o.summary.lost[0]);
        assert!((450..=750).contains(&lost), "{lost} of 30,000 lost at 2 %");
        // Delays stay near 50 +- 10 ms; at 0.1 ms spacing the reorder step adds at most a spacing to a held datagram.
        assert!(
            up_delay
                .iter()
                .all(|&d| (40_000_000..=60_100_000).contains(&d))
        );
        // A down datagram echoes only an up datagram that arrived since the last echo: with arrivals jittered at 0.98
        // per 0.1 ms, about e^-0.98 = 38 % of intervals have none, so about 61 % of down datagrams carry a round trip.
        let rtt = series(&o, "probe_rtt_ns");
        assert!(
            (15_000..=21_000).contains(&rtt.len()),
            "{} round trips",
            rtt.len()
        );
        let mean = rtt.iter().sum::<i64>() / rtt.len() as i64;
        assert!(
            (99_000_000..=101_000_000).contains(&mean),
            "mean round trip {mean} ns"
        );
        assert!(
            series(&o, "probe_burst_up_lost").is_empty(),
            "no burst at P2"
        );
        assert_eq!(o.fate_log_sha256.len(), 64);
    }

    #[test]
    fn the_same_seed_gives_the_same_hash_and_records_and_another_seed_does_not() {
        let a = run(&setup(Profile::p1(), 5, 10_000)).unwrap();
        let b = run(&setup(Profile::p1(), 5, 10_000)).unwrap();
        let c = run(&setup(Profile::p1(), 6, 10_000)).unwrap();
        assert_eq!(a.fate_log_sha256, b.fate_log_sha256);
        assert_eq!(a.seconds, b.seconds);
        assert_ne!(a.fate_log_sha256, c.fate_log_sha256);
    }

    #[test]
    fn b1_sends_its_burst_on_top_and_loses_all_of_it() {
        let o = run(&setup(Profile::b1(&[SEC]), 2, 20_000)).unwrap();
        assert_eq!(
            o.sent, 30_000,
            "20,000 outside the burst and the 10,000 inside it"
        );
        let burst_up = series(&o, "probe_burst_up_lost");
        let burst_down = series(&o, "probe_burst_down_lost");
        assert_eq!((burst_up.len(), burst_down.len()), (10_000, 10_000));
        assert!(burst_up.iter().chain(&burst_down).all(|&f| f == 1));
        assert_eq!(series(&o, "probe_up_lost").len(), 20_000);
        assert_eq!(series(&o, "probe_down_lost").len(), 20_000);
    }

    #[test]
    fn the_fate_logs_reorder_counts_are_reported_per_direction() {
        let o = run(&setup(Profile::p3(), 3, 10_000)).unwrap();
        for (delivered, reordered) in o.reorder {
            assert!(delivered > 9_000 && reordered > 0 && reordered <= delivered);
        }
    }

    #[cfg(not(feature = "bites"))]
    #[test]
    fn a_measured_build_refuses_a_bite() {
        let mut s = setup(Profile::p2(), 1, 10);
        s.bite = Some("PM-0001-3/rtt-each-way".to_owned());
        assert!(run(&s).unwrap_err().contains("measured build"));
    }

    #[cfg(feature = "bites")]
    #[test]
    fn a_bites_build_runs_a_bite_and_refuses_an_unknown_one() {
        let mut s = setup(Profile::p2(), 1, 10_000);
        s.bite = Some("PM-0001-3/rtt-each-way".to_owned());
        let o = run(&s).unwrap();
        assert!(
            o.summary.mean_delay_ns[0] > 95_000_000,
            "the full round trip each way"
        );
        s.bite = Some("PM-0001-3/nothing".to_owned());
        assert!(run(&s).is_err());
    }
}
