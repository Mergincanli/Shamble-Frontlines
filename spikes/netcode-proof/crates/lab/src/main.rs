//! The npt-lab command (plan §4.7, §6.4): runs on this container's simulated time and writes npt-log results logs for
//! npt-judge. Two subcommands: `probe` (PM-0001-3's L1 half, unit U2) and `hash` (U4d2: the recordings manifest
//! regenerated and checked, the two native legs of the determinism check and its comparisons, in the release build;
//! its own usage is in hash.rs); the scenario runner and the x86-64 replay join it in U4.
//!
//! ```text
//! npt-lab probe --run <id> --set <set> --profile <P1..P4|B1> --seed <n> --date <YYYY-MM-DD> --commit <sha>
//!               --out <file.jsonl> [--role measured|twin|bite] [--twin <run>] [--bite <id>]
//!               [--packets <n>] [--rate <per second>] [--burst-at-s <s>]
//! npt-lab hash ...
//! ```
//! A log is never overwritten: `--out` must not exist. Exit 0 on a written log, 1 when the run failed, 2 on a usage
//! error. `probe`'s log header and summary line carry the whole fate log's SHA-256 and, since U4d2 (carry 66), the
//! SHA-256 of v1's tag and the log's datagram entries, the hash U2's v1 logs had.

mod contract;
mod hash;
mod logs;
mod machine;
mod mirror;
mod precheck;
mod probe;
mod replay;
mod scenario;

use npt_l1::Profile;
use npt_log::{Ended, Header, SCHEMA, Writer};
use std::collections::BTreeMap;
use std::io::BufWriter;
use std::process::ExitCode;

const USAGE: &str = "npt-lab probe --run <id> --set <set> --profile <P1..P4|B1> --seed <n> --date <YYYY-MM-DD> \
--commit <sha> --out <file.jsonl> [--role measured|twin|bite] [--twin <run>] [--bite <id>] [--packets <n>] \
[--rate <per second>] [--burst-at-s <s>]";

struct Args {
    run: String,
    set: String,
    role: String,
    profile: String,
    seed: u64,
    date: String,
    commit: String,
    out: String,
    twin: Option<String>,
    bite: Option<String>,
    packets: u64,
    rate: u64,
    burst_at_s: u64,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    let mut it = argv.iter();
    while let Some(flag) = it.next() {
        let name = flag
            .strip_prefix("--")
            .ok_or(format!("unexpected argument {flag:?}"))?;
        let value = it.next().ok_or(format!("{flag} needs a value"))?;
        if map.insert(name, value).is_some() {
            return Err(format!("{flag} given twice"));
        }
    }
    let mut take = |k: &str| map.remove(k).map(str::to_owned);
    let need = |v: Option<String>, k: &str| v.ok_or(format!("--{k} is required"));
    let num = |v: Option<String>, k: &str, default: u64| match v {
        None => Ok(default),
        Some(s) => s
            .parse::<u64>()
            .map_err(|_| format!("--{k} {s:?} is not a whole number")),
    };
    let args = Args {
        run: need(take("run"), "run")?,
        set: need(take("set"), "set")?,
        role: take("role").unwrap_or_else(|| "measured".to_owned()),
        profile: need(take("profile"), "profile")?,
        seed: num(Some(need(take("seed"), "seed")?), "seed", 0)?,
        date: need(take("date"), "date")?,
        commit: need(take("commit"), "commit")?,
        out: need(take("out"), "out")?,
        twin: take("twin"),
        bite: take("bite"),
        packets: num(take("packets"), "packets", probe::PACKETS)?,
        rate: num(take("rate"), "rate", probe::RATE_PPS)?,
        burst_at_s: num(take("burst-at-s"), "burst-at-s", 50)?,
    };
    if let Some(k) = map.keys().next() {
        return Err(format!("unknown option --{k}"));
    }
    if !["measured", "twin", "bite"].contains(&args.role.as_str()) {
        return Err(format!("--role {:?}: measured, twin or bite", args.role));
    }
    if (args.role == "bite") != args.bite.is_some() && args.role != "twin" {
        return Err("--role bite and --bite go together".to_owned());
    }
    Ok(args)
}

fn profile(name: &str, burst_at_s: u64) -> Result<Profile, String> {
    match name {
        "B1" => Ok(Profile::b1(&[burst_at_s * probe::SEC])),
        "P1" | "P2" | "P3" | "P4" => Ok(Profile::by_name(name).expect("a named profile")),
        _ => Err(format!(
            "--profile {name:?}: P1 to P4 or B1 (PM-0001-3's L1 parts; L1 has no P0 part)"
        )),
    }
}

fn run_probe(a: &Args) -> Result<String, String> {
    let profile = profile(&a.profile, a.burst_at_s)?;
    let machine = machine::container();
    let build = machine::build(&a.commit)?;
    let file = std::fs::File::create_new(&a.out)
        .map_err(|e| format!("{}: {e} (a log is never overwritten)", a.out))?;
    let setup = probe::Setup {
        profile: profile.clone(),
        seed: a.seed,
        packets: a.packets,
        rate_pps: a.rate,
        bite: a.bite.clone(),
    };
    let o = probe::run(&setup)?;
    let permille =
        |(delivered, reordered): (u64, u64)| (reordered * 1000).checked_div(delivered).unwrap_or(0);
    let mut params: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    params.insert("fate_log_sha256".into(), o.fate_log_sha256.clone().into());
    params.insert(
        "fate_log_v1_datagram_sha256".into(),
        o.fate_log_v1_datagram_sha256.clone().into(),
    );
    params.insert("reorder_up_permille".into(), permille(o.reorder[0]).into());
    params.insert(
        "reorder_down_permille".into(),
        permille(o.reorder[1]).into(),
    );
    params.insert(
        "reorder_permille".into(),
        permille(o.reorder[0]).min(permille(o.reorder[1])).into(),
    );
    params.insert(
        "reorder_step_permille".into(),
        profile.reorder.permille.into(),
    );
    params.insert("probe_rate_pps".into(), a.rate.into());
    params.insert("probe_sent_count".into(), o.sent.into());
    if !profile.bursts_ns.is_empty() {
        let starts: Vec<u64> = profile.bursts_ns.iter().map(|b| b.0).collect();
        params.insert("burst_starts_ns".into(), starts.into());
    }
    if let Some(t) = &a.twin {
        params.insert("twin".into(), t.clone().into());
    }
    let header = Header {
        schema: SCHEMA,
        run: a.run.clone(),
        set: a.set.clone(),
        role: a.role.clone(),
        marks: vec!["PM-0001-3".to_owned()],
        machine,
        build,
        bite: a.bite.clone(),
        layer: "L1".to_owned(),
        profile: profile.name.to_owned(),
        seed: a.seed,
        tick_hz: 30,
        clients: 1,
        scene: "none".to_owned(),
        process: "probe".to_owned(),
        slot: 0,
        date: a.date.clone(),
        params,
    };
    let mut w =
        Writer::new(BufWriter::new(file), &header).map_err(|e| format!("{}: {e}", a.out))?;
    for r in &o.seconds {
        w.second(r).map_err(|e| format!("{}: {e}", a.out))?;
    }
    w.finish(Ended::Clean)
        .map_err(|e| format!("{}: {e}", a.out))?;
    let s = &o.summary;
    Ok(format!(
        "{} {} seed {}: {} sent each way; lost up {} down {}; mean one-way up {} ns down {} ns; mean round trip {} ns \
         over {} samples; reorder up {} down {} of 1000; fate log {}; v1 datagram fate log {}",
        a.run,
        profile.name,
        a.seed,
        o.sent,
        s.lost[0],
        s.lost[1],
        s.mean_delay_ns[0],
        s.mean_delay_ns[1],
        s.mean_rtt_ns,
        s.rtt_samples,
        permille(o.reorder[0]),
        permille(o.reorder[1]),
        &o.fate_log_sha256[..16],
        &o.fate_log_v1_datagram_sha256[..16],
    ))
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let Some((cmd, rest)) = argv.split_first() else {
        eprintln!("usage: {USAGE}\n       {}", hash::USAGE);
        return ExitCode::from(2);
    };
    if cmd == "hash" {
        return match hash::run(rest) {
            Ok((lines, equal)) => {
                for l in lines {
                    println!("{l}");
                }
                ExitCode::from(if equal { 0 } else { 1 })
            }
            Err(hash::Failure::Usage(e)) => {
                eprintln!("npt-lab hash: {e}\nusage: {}", hash::USAGE);
                ExitCode::from(2)
            }
            Err(hash::Failure::Failed(e)) => {
                eprintln!("npt-lab hash: {e}");
                ExitCode::from(1)
            }
        };
    }
    if cmd == "scenario" {
        let s = match scenario::parse(rest) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("npt-lab scenario: {e}\nusage: {}", scenario::USAGE);
                return ExitCode::from(2);
            }
        };
        return match scenario::run(&s) {
            Ok(o) => {
                println!("{}", scenario::summary_line(&s, &o));
                ExitCode::SUCCESS
            }
            Err(hash::Failure::Usage(e)) => {
                eprintln!("npt-lab scenario: {e}\nusage: {}", scenario::USAGE);
                ExitCode::from(2)
            }
            Err(hash::Failure::Failed(e)) => {
                eprintln!("npt-lab scenario: {e}");
                ExitCode::from(1)
            }
        };
    }
    if cmd == "replay" {
        let s = match replay::parse(rest) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("npt-lab replay: {e}\nusage: {}", replay::USAGE);
                return ExitCode::from(2);
            }
        };
        return match replay::run(&s) {
            Ok(o) => {
                println!("{}", replay::summary_line(&s, &o));
                ExitCode::SUCCESS
            }
            Err(hash::Failure::Usage(e)) => {
                eprintln!("npt-lab replay: {e}\nusage: {}", replay::USAGE);
                ExitCode::from(2)
            }
            Err(hash::Failure::Failed(e)) => {
                eprintln!("npt-lab replay: {e}");
                ExitCode::from(1)
            }
        };
    }
    if cmd == "precheck" {
        return match precheck::main(rest) {
            Ok(lines) => {
                for l in lines {
                    println!("{l}");
                }
                ExitCode::SUCCESS
            }
            Err((code, e)) => {
                eprintln!("npt-lab precheck: {e}\nusage: {}", precheck::USAGE);
                ExitCode::from(code)
            }
        };
    }
    if cmd == "contract" {
        return match contract::main(rest) {
            Ok(lines) => {
                for l in lines {
                    println!("{l}");
                }
                ExitCode::SUCCESS
            }
            Err((code, e)) => {
                if code == 2 {
                    eprintln!("npt-lab contract: {e}\nusage: {}", contract::USAGE);
                } else if code == contract::UNREADABLE {
                    // Not the contract's FAIL (the advisor's U4r build ruling 3, point 6 (3)).
                    eprintln!("npt-lab contract: {e} (exit {code}: an input that cannot be read)");
                } else {
                    println!("{e}");
                }
                ExitCode::from(code)
            }
        };
    }
    if cmd != "probe" {
        eprintln!(
            "npt-lab: unknown command {cmd:?}\nusage: {USAGE}\n       {}\n       {}\n       {}\n       {}\n       {}",
            hash::USAGE,
            scenario::USAGE,
            replay::USAGE,
            precheck::USAGE,
            contract::USAGE
        );
        return ExitCode::from(2);
    }
    let args = match parse(rest) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("npt-lab probe: {e}\nusage: {USAGE}");
            return ExitCode::from(2);
        }
    };
    match run_probe(&args) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("npt-lab probe: {e}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    const BASE: &str = "--run r --set s --profile P2 --seed 3 --date 2026-11-06 --out x.jsonl \
                        --commit 0000000000000000000000000000000000000000";

    #[test]
    fn the_options_parse_with_their_defaults() {
        let a = parse(&argv(BASE)).unwrap();
        assert_eq!(
            (a.role.as_str(), a.packets, a.rate, a.burst_at_s),
            ("measured", 1_000_000, 10_000, 50)
        );
        assert_eq!(a.seed, 3);
        assert!(a.twin.is_none() && a.bite.is_none());
    }

    #[test]
    fn bad_options_are_refused() {
        assert!(parse(&argv("--run r")).is_err(), "missing options");
        assert!(
            parse(&argv(&format!("{BASE} --colour red"))).is_err(),
            "an unknown option"
        );
        assert!(
            parse(&argv(&format!("{BASE} --seed 4"))).is_err(),
            "an option twice"
        );
        assert!(
            parse(&argv(&format!("{BASE} --role bite"))).is_err(),
            "a bite role without its bite"
        );
        assert!(
            parse(&argv(&format!("{BASE} --bite PM-0001-3/clock-seed"))).is_err(),
            "a bite as measured"
        );
        assert!(
            parse(&argv(&format!(
                "{BASE} --role bite --bite PM-0001-3/clock-seed"
            )))
            .is_ok()
        );
        assert!(
            parse(&argv(&format!(
                "{BASE} --role twin --bite PM-0001-3/clock-seed"
            )))
            .is_ok(),
            "a bite's twin"
        );
        assert!(profile("P0", 50).is_err(), "L1 has no P0 part");
        assert_eq!(
            profile("B1", 50).unwrap().bursts_ns,
            vec![(50 * probe::SEC, probe::SEC)]
        );
    }
}
