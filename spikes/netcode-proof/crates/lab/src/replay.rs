//! `replay`, the x86-64 re-decode ([`ReplayX86`]; u4r-brief.md 2.1 and 2.4 r3; plan :753-755, :761-764; R-8, the
//! advisor's condition 9): a run's logged client bytes (`scenario --bytes`: each client's received messages, step by
//! step, with each step's host instant) decoded again natively by a fresh `ClientEndpoint` per client, with the run's
//! join, lead and starts (read from the run's client log header), and the hashes it files written as `hs.replay_hash`
//! in a `replay` log (process `replay`, this machine's arch: only an x86_64 one is read, judge/src/figures.rs:222-233),
//! never in another log. Commands never reach the decoder (`ClientEndpoint::input` decodes what it is handed before it
//! stamps a command: client/src/lib.rs:379-410), so the re-decode hands each step its bytes with no command and calls
//! `step` as the run did; the hashes come back the run's. The re-decoder's client takes the run's bite when the run had
//! one (the same client), or the one `--bite` names: r3's is npt-client's own PM-0001-17/client-rounding (its grid
//! floored, `npt_client::grid_with_bite`, client/src/lib.rs:37-40), so npt-lab plants no new id.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_log::{Ended, Header, Log, Phase, TickRecord, Writer};
use serde_json::Value;

use crate::hash::Failure;
use crate::scenario::{Bites, join_of};

/// `replay`'s usage line.
pub(crate) const USAGE: &str =
    "npt-lab replay --logs <run folder> --bytes <folder> --out <folder> [--bite <id>]";

/// One `replay` call's settings.
#[derive(Clone, Debug)]
pub(crate) struct Settings {
    /// The run's folder: its `client.jsonl` gives the header and the hashes compared.
    pub(crate) logs: PathBuf,
    /// The run's `--bytes` folder: one `client-<slot>.bytes` per client.
    pub(crate) bytes: PathBuf,
    /// Where `replay.jsonl` goes (never overwritten).
    pub(crate) out: PathBuf,
    pub(crate) bite: Option<String>,
}

/// `replay`'s options (after `replay`), as `--name value` pairs, each once.
pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
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
    let mut take = |k: &str| map.remove(k).map(PathBuf::from);
    let logs = take("logs").ok_or("--logs is required")?;
    let bytes = take("bytes").ok_or("--bytes is required")?;
    let out = take("out").ok_or("--out is required")?;
    let bite = map.remove("bite").map(str::to_owned);
    if let Some(k) = map.keys().next() {
        return Err(format!("unknown option --{k}"));
    }
    Ok(Settings {
        logs,
        bytes,
        out,
        bite,
    })
}

/// One logged client step: its host instant and what it was handed, as (channel, bytes, arrival ns).
pub(crate) type Step = (u64, Vec<(Channel, Vec<u8>, u64)>);

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self
            .at
            .checked_add(n)
            .filter(|e| *e <= self.b.len())
            .ok_or_else(|| format!("cut short at byte {}", self.at))?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }

    fn u64(&mut self) -> Result<u64, String> {
        let mut w = [0u8; 8];
        w.copy_from_slice(self.take(8)?);
        Ok(u64::from_le_bytes(w))
    }

    fn u32(&mut self) -> Result<u32, String> {
        let mut w = [0u8; 4];
        w.copy_from_slice(self.take(4)?);
        Ok(u32::from_le_bytes(w))
    }
}

/// One client's bytes file, step by step (scenario.rs `write_step`'s format: host instant u64, count u32, then per
/// message channel u8 (0 datagram, 1 control, 2 full state), arrival u64, length u32 and the bytes, little-endian).
pub(crate) fn read_steps(bytes: &[u8]) -> Result<Vec<Step>, String> {
    let mut c = Cursor { b: bytes, at: 0 };
    let mut out = Vec::new();
    while c.at < bytes.len() {
        let host = c.u64()?;
        let count = c.u32()?;
        let mut got = Vec::new();
        for _ in 0..count {
            let channel = match c.take(1)?[0] {
                0 => Channel::Datagram,
                1 => Channel::Control,
                2 => Channel::FullState,
                x => return Err(format!("channel code {x} at byte {}", c.at - 1)),
            };
            let arrival = c.u64()?;
            let len = c.u32()? as usize;
            got.push((channel, c.take(len)?.to_vec(), arrival));
        }
        out.push((host, got));
    }
    Ok(out)
}

/// A header param as a whole number (the run's client log header).
fn param(h: &Header, name: &str) -> Result<u64, String> {
    h.params
        .get(name)
        .and_then(Value::as_u64)
        .ok_or(format!("the run's client log header has no param {name}"))
}

/// The x86-64 re-decoder of one run (plan.md:617's `ReplayX86`): its clients' join, lead and starts, and the bite its
/// clients take.
pub(crate) struct ReplayX86 {
    seed: u64,
    lead: u32,
    cmd_start: u32,
    event_start: u32,
    first_tick: u32,
    bites: Bites,
}

impl ReplayX86 {
    /// The re-decoder of the run whose client log header is `h`; `bite` the one `--bite` names (else the run's).
    pub(crate) fn of(h: &Header, bite: Option<&str>) -> Result<ReplayX86, String> {
        let small = |name: &str| {
            u32::try_from(param(h, name)?).map_err(|_| format!("param {name} is above 32 bits"))
        };
        let bites = match bite.or(h.bite.as_deref()) {
            None => Bites::default(),
            Some(id) => Bites::of(id)?,
        };
        Ok(ReplayX86 {
            seed: h.seed,
            lead: small("lead_ticks_count")?,
            cmd_start: small("cmd_start_count")?,
            event_start: small("event_start_count")?,
            first_tick: small("tick_start")?,
            bites,
        })
    }

    /// A 32-bit tick as the logs' never-wrapping tick: forward from the run's first tick (logs.rs `wide`).
    fn wide(&self, t: u32) -> u64 {
        u64::from(self.first_tick) + u64::from(t.wrapping_sub(self.first_tick))
    }

    /// Slot `slot`'s steps decoded again: the hashes its endpoint files, by wide tick (two different hashes for one
    /// tick give the sentinel 0, as the client writer's rule: logs.rs `filed`).
    pub(crate) fn decode(&self, slot: u8, steps: &[Step]) -> BTreeMap<u64, u64> {
        let e = ClientEndpoint::new(join_of(self.seed, slot), npt_sim::TICK_HZ)
            .with_lead(self.lead)
            .with_starts(self.cmd_start, self.event_start);
        let mut e = self.bites.client(e);
        let mut out: BTreeMap<u64, u64> = BTreeMap::new();
        for (host, got) in steps {
            e.input(None, got, *host);
            for (t, h) in e.step().hashes {
                let w = self.wide(t);
                let v = out.entry(w).or_insert(h);
                if *v != h {
                    *v = 0;
                }
            }
        }
        out
    }
}

/// What one `replay` call found against the run's client log.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub(crate) clients: usize,
    pub(crate) steps: u64,
    pub(crate) replay_hashes: u64,
    /// (client, tick) pairs at which the client log holds a hash.
    pub(crate) compared: u64,
    /// Of those, the ones the replay gives no hash or another hash for, plus every replay hash at a (client, tick)
    /// where the client log holds none.
    pub(crate) mismatches: u64,
    pub(crate) first: Option<String>,
}

/// Compares the client log's `hs.decoded_hash` with the replay's hashes `by_tick` (wide tick to one entry per slot).
pub(crate) fn compare(client: &Log, by_tick: &BTreeMap<u64, Vec<Option<u64>>>) -> Outcome {
    let mut o = Outcome::default();
    let miss = |o: &mut Outcome, text: String| {
        o.mismatches += 1;
        o.first.get_or_insert(text);
    };
    let mut seen: BTreeMap<(u64, usize), u64> = BTreeMap::new();
    for rec in &client.ticks {
        let Some(list) = rec.hashes("decoded_hash") else {
            continue;
        };
        for (slot, h) in list.iter().enumerate() {
            let Some(h) = h else { continue };
            o.compared += 1;
            seen.insert((rec.t, slot), *h);
            let r = by_tick
                .get(&rec.t)
                .and_then(|l| l.get(slot).copied().flatten());
            if r != Some(*h) {
                miss(
                    &mut o,
                    format!(
                        "client {slot} tick {}: logged {h:016x}, replay {r:x?}",
                        rec.t
                    ),
                );
            }
        }
    }
    for (t, list) in by_tick {
        for (slot, r) in list.iter().enumerate() {
            if let Some(r) = r
                && !seen.contains_key(&(*t, slot))
            {
                miss(
                    &mut o,
                    format!(
                        "client {slot} tick {t}: the replay gives {r:016x}, the client log nothing"
                    ),
                );
            }
        }
    }
    o
}

fn read_log(path: &Path) -> Result<Log, String> {
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    npt_log::read_log(std::io::BufReader::new(f)).map_err(|e| format!("{}: {e}", path.display()))
}

/// One `replay` call (the module text): every client's bytes decoded again, `replay.jsonl` written into `s.out`, and
/// the outcome against the run's client log.
pub(crate) fn run(s: &Settings) -> Result<Outcome, Failure> {
    let fail = Failure::Failed;
    let client = read_log(&s.logs.join("client.jsonl")).map_err(fail)?;
    let h = &client.header;
    // The header's client count as a slot count, 1 to 255, before any .bytes file is read or any list sized
    // (advisor-u4r-r1, FP-6 item 4: RR3-L7).
    let clients = u8::try_from(h.clients)
        .ok()
        .filter(|c| *c > 0)
        .ok_or_else(|| {
            fail(format!(
                "the run's client log header names {} clients: 1 to 255",
                h.clients
            ))
        })?;
    let rx = ReplayX86::of(h, s.bite.as_deref()).map_err(Failure::Usage)?;
    let n = usize::from(clients);
    let mut by_tick: BTreeMap<u64, Vec<Option<u64>>> = BTreeMap::new();
    let mut o = Outcome {
        clients: n,
        ..Outcome::default()
    };
    for s8 in 0..clients {
        let slot = usize::from(s8);
        let p = s.bytes.join(format!("client-{slot}.bytes"));
        let bytes = std::fs::read(&p).map_err(|e| fail(format!("{}: {e}", p.display())))?;
        let steps = read_steps(&bytes).map_err(|e| fail(format!("{}: {e}", p.display())))?;
        o.steps += steps.len() as u64;
        for (w, hash) in rx.decode(s8, &steps) {
            by_tick.entry(w).or_insert_with(|| vec![None; n])[slot] = Some(hash);
            o.replay_hashes += 1;
        }
    }
    // The replay log: the run's header as the client log holds it, but its process, machine, build and bite.
    let mut header = h.clone();
    header.process = "replay".to_owned();
    header.machine = crate::machine::container();
    header.build = crate::machine::build(&h.build.commit).map_err(fail)?;
    if s.bite.is_some() {
        header.bite = s.bite.clone();
    }
    std::fs::create_dir_all(&s.out).map_err(|e| fail(format!("{}: {e}", s.out.display())))?;
    let path = s.out.join("replay.jsonl");
    let file = std::fs::File::create_new(&path).map_err(|e| {
        fail(format!(
            "{}: {e} (a log is never overwritten)",
            path.display()
        ))
    })?;
    let io = |e: std::io::Error| fail(format!("{}: {e}", path.display()));
    let mut w = Writer::new(std::io::BufWriter::new(file), &header).map_err(io)?;
    let phases: BTreeMap<u64, Phase> = client.ticks.iter().map(|r| (r.t, r.ph)).collect();
    for (t, list) in &by_tick {
        let ph = phases
            .range(..=*t)
            .next_back()
            .map_or(Phase::Warmup, |(_, p)| *p);
        let mut rec = TickRecord::new(*t, ph);
        rec.set_hashes("replay_hash", list);
        w.tick(&rec).map_err(io)?;
    }
    w.finish(Ended::Clean).map_err(io)?;
    let c = compare(&client, &by_tick);
    Ok(Outcome {
        compared: c.compared,
        mismatches: c.mismatches,
        first: c.first,
        ..o
    })
}

/// The summary line of a `replay` call.
pub(crate) fn summary_line(s: &Settings, o: &Outcome) -> String {
    format!(
        "replay of {}: {} clients, {} steps, {} replay hashes; against the client log: {} (client, tick) pairs \
         compared, {} mismatches{}",
        s.logs.display(),
        o.clients,
        o.steps,
        o.replay_hashes,
        o.compared,
        o.mismatches,
        o.first
            .as_ref()
            .map(|f| format!("; the first: {f}"))
            .unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::tests::{Scratch, read, scratch, settings};

    /// r3's run: SS, 16 clients, P2, 6 s of warm-up and 2 s measured, each client's received bytes logged (under the
    /// test binary's target dir: `scratch`).
    fn r3_run(name: &str) -> (Scratch, Scratch) {
        let dir = scratch(name);
        let bytes = scratch(&format!("{name}-bytes"));
        let s = settings(
            &dir,
            &format!(
                "--seed 1 --profile P2 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10 --bytes {}",
                bytes.display()
            ),
        );
        crate::scenario::run(&s).unwrap_or_else(|e| panic!("r3's run: {e:?}"));
        (dir, bytes)
    }

    /// advisor-u4r-r1, FP-6 item 4 (RR3-L7): a run's client log whose header names 256 clients is refused as a failure
    /// before any .bytes file is read (the bytes folder given does not exist).
    #[test]
    fn fp6_a_header_of_256_clients_is_refused_before_any_bytes_file_is_read() {
        let dir = scratch("replay-256");
        std::fs::create_dir_all(&*dir).expect("the folder");
        let header = Header {
            schema: npt_log::SCHEMA,
            run: "t".to_owned(),
            set: "t".to_owned(),
            role: "measured".to_owned(),
            marks: Vec::new(),
            machine: crate::machine::container(),
            build: crate::machine::build(crate::scenario::tests::COMMIT).expect("a build"),
            bite: None,
            layer: "L1".to_owned(),
            profile: "P0".to_owned(),
            seed: 1,
            tick_hz: npt_sim::TICK_HZ,
            clients: 256,
            scene: "SS".to_owned(),
            process: "client".to_owned(),
            slot: 0,
            date: "2026-11-14".to_owned(),
            params: BTreeMap::new(),
        };
        let f = std::fs::File::create(dir.join("client.jsonl")).expect("the log");
        Writer::new(f, &header)
            .expect("its header")
            .finish(Ended::Clean)
            .expect("its footer");
        let s = Settings {
            logs: dir.to_path_buf(),
            bytes: dir.join("no-such-bytes-folder"),
            out: dir.join("out"),
            bite: None,
        };
        match run(&s) {
            Err(Failure::Failed(m)) => {
                assert_eq!(m, "the run's client log header names 256 clients: 1 to 255")
            }
            Err(Failure::Usage(m)) => panic!("a usage error: {m}"),
            Ok(o) => panic!("replayed: {o:?}"),
        }
    }

    /// The bytes file's reader takes back what `write_step` wrote, and refuses a cut-short file and an unknown channel.
    #[test]
    fn r3_the_bytes_file_reads_back_step_by_step() {
        let steps: Vec<Step> = vec![
            (5, Vec::new()),
            (
                9,
                vec![
                    (Channel::Control, vec![1, 2], 7),
                    (Channel::Datagram, vec![], 8),
                ],
            ),
            (12, vec![(Channel::FullState, vec![3; 300], 11)]),
        ];
        let mut b = Vec::new();
        for (host, got) in &steps {
            crate::scenario::write_step(&mut b, *host, got).expect("writes");
        }
        assert_eq!(read_steps(&b).expect("reads"), steps);
        assert!(read_steps(&b[..b.len() - 1]).is_err(), "cut short");
        let mut bad = b.clone();
        bad[20] = 9;
        assert!(read_steps(&bad).is_err(), "channel code 9");
    }

    /// r3, `replay`'s U4 test (plan :761-764): one short L1 run (SS P2) logs each client's received bytes; `replay`,
    /// decoding them again, gives each client's logged hash at every (client, tick) that has one, with more than 0
    /// compared, and its log is a `replay` log with one entry per client.
    #[test]
    fn r3_the_re_decode_gives_every_client_s_logged_hash() {
        let (dir, bytes) = r3_run("r3");
        let out = scratch("r3-replay");
        let s = Settings {
            logs: dir.to_path_buf(),
            bytes: bytes.to_path_buf(),
            out: out.to_path_buf(),
            bite: None,
        };
        let o = run(&s).unwrap_or_else(|e| panic!("replay: {e:?}"));
        println!("r3: {}", summary_line(&s, &o));
        assert!(o.compared > 0, "a replay that compares nothing fails");
        assert_eq!(o.mismatches, 0, "{:?}", o.first);
        let log = read(&out, "replay");
        assert_eq!(log.header.process, "replay");
        assert!(
            log.ticks
                .iter()
                .all(|t| t.hashes("replay_hash").is_some_and(|l| l.len() == 16)
                    && !t.hs.contains_key("decoded_hash"))
        );
    }

    /// r3's bite (plan :763-764): the re-decoder's grid floored (npt-client's PM-0001-17/client-rounding, given to
    /// `--bite`: no new id) gives at least one (client, tick) mismatch.
    #[cfg(feature = "bites")]
    #[test]
    fn r3_bite_a_floored_re_decoder_grid_gives_a_mismatch() {
        let (dir, bytes) = r3_run("r3-bite");
        let out = scratch("r3-bite-replay");
        let s = Settings {
            logs: dir.to_path_buf(),
            bytes: bytes.to_path_buf(),
            out: out.to_path_buf(),
            bite: Some("PM-0001-17/client-rounding".to_owned()),
        };
        let o = run(&s).unwrap_or_else(|e| panic!("replay: {e:?}"));
        println!("r3 bite: {}", summary_line(&s, &o));
        assert!(o.compared > 0);
        assert!(o.mismatches >= 1, "the floored grid must give a mismatch");
    }
}
