//! npt-super: the supervisor stand-in (plan E2, E16; §10.2): unprivileged, it starts, stops and kills room processes
//! through systemd template units by npt's fixed sudo wrappers, times each start to Open, and logs every exit.
//! Built in unit U6: U6a builds its start timer (u6a-brief.md Block 6, Q-U6A-1 (a); the advisor's ADV-U6A-3), the
//! systemd units and the 8-room density harness are U6e's.
//!
//! - `npt-super start <flags>`: the hold form. Starts one room (`<room> --settings <settings>`), times spawn to its
//!   `OPEN` line, writes `event.room_start.spawn_to_open_ns`, keeps the room until npt-super's own end (its stdin held
//!   open; closed when npt-super's stdin closes), and records its exit (`event.room_exit.nonzero`). U8's run plan
//!   starts every room through it (marks.json -16's notes: "every room start in every run").
//! - `npt-super time-starts <flags> --count <N>`: N of those one at a time with no hold (each room's stdin closed once
//!   it is open), each start's settings at `--settings` with `{n}` replaced by its number from 1.
//!
//! A start not open by the deadline (10 s, or `--deadline-ms`) is killed, waited for, and its elapsed time written as
//! its `spawn_to_open_ns`. The room's command takes its path and arguments as separate values, no shell (SEC-U6AD-6);
//! its stdout is read a bounded line at a time; its stderr goes to npt-super's stderr, never into the log. npt-super
//! reaches no sim, room or proto code (check.mjs's deps step); its log is npt-log's, process `super`, with this
//! machine from `Machine::read_local`, and its header carries the run's `--scene`, `--tick-hz` and `--bite` as the
//! room's does, so the judge reads both logs as one run (eval.rs's same-run check; NR-U6A-1).

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

use npt_log::{Build, Ended, EventRecord, Header, Machine, Phase, Writer};
use sha2::{Digest, Sha256};

/// The Open deadline by default, ms.
const DEADLINE_MS: u64 = 10_000;
/// The longest stdout line read from a room at once, bytes.
const LINE_MAX: u64 = 256;
/// A flag's value's length limit, bytes, and the most starts one call times.
const VALUE_MAX: usize = 4_096;
const COUNT_MAX: u32 = 1_000;

/// What the flags say (the module text); every flag is required but `--deadline-ms`, `--count`, `--marks` and
/// `--bite`.
struct Flags {
    room: PathBuf,
    settings: String,
    log: PathBuf,
    count: u32,
    deadline: Duration,
    header: [String; 12],
    marks: Vec<String>,
    bite: Option<String>,
    scene: String,
    tick_hz: u32,
}

/// The header flags, in [`Flags::header`]'s order.
const HEADER_FLAGS: [&str; 12] = [
    "--run",
    "--set",
    "--role",
    "--date",
    "--commit",
    "--layer",
    "--profile",
    "--machine-role",
    "--machine-name",
    "--machine-plan",
    "--machine-location",
    "--machine-address",
];

fn parse(args: &[String]) -> Result<Flags, String> {
    let mut room = None;
    let mut settings = None;
    let mut log = None;
    let mut count = 1;
    let mut deadline = DEADLINE_MS;
    let mut header: [Option<String>; 12] = Default::default();
    let mut marks = Vec::new();
    let (mut bite, mut scene, mut tick_hz) = (None, None, None);
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or_else(|| format!("{flag} has no value"))?;
        if value.len() > VALUE_MAX || value.chars().any(char::is_control) {
            return Err(format!(
                "{flag}'s value is above {VALUE_MAX} bytes or holds a control character"
            ));
        }
        match flag.as_str() {
            "--room" => room = Some(PathBuf::from(value)),
            "--settings" => settings = Some(value.clone()),
            "--log" => log = Some(PathBuf::from(value)),
            "--count" => count = value.parse().map_err(|_| format!("--count {value:?}"))?,
            "--deadline-ms" => {
                deadline = value
                    .parse()
                    .map_err(|_| format!("--deadline-ms {value:?}"))?
            }
            "--marks" => marks = value.split(',').map(str::to_owned).collect(),
            "--bite" => bite = Some(value.clone()),
            "--scene" => scene = Some(value.clone()),
            "--tick-hz" => {
                tick_hz = Some(
                    value
                        .parse::<u32>()
                        .ok()
                        .filter(|hz| (1..=1_000).contains(hz))
                        .ok_or_else(|| format!("--tick-hz {value:?} (1 to 1,000)"))?,
                )
            }
            f => {
                let i = HEADER_FLAGS
                    .iter()
                    .position(|h| *h == f)
                    .ok_or_else(|| format!("unknown flag {f}"))?;
                header[i] = Some(value.clone());
            }
        }
    }
    if count == 0 || count > COUNT_MAX || deadline == 0 || deadline > 600_000 {
        return Err(format!(
            "--count is 1 to {COUNT_MAX} and --deadline-ms 1 to 600,000"
        ));
    }
    let header = header
        .iter()
        .zip(HEADER_FLAGS)
        .map(|(h, f)| h.clone().ok_or_else(|| format!("{f} is missing")))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Flags {
        room: room.ok_or("--room is missing")?,
        settings: settings.ok_or("--settings is missing")?,
        log: log.ok_or("--log is missing")?,
        count,
        deadline: Duration::from_millis(deadline),
        header: header.try_into().map_err(|_| "the header flags")?,
        marks,
        bite,
        scene: scene.ok_or("--scene is missing")?,
        tick_hz: tick_hz.ok_or("--tick-hz is missing")?,
    })
}

/// The monotonic clock's smallest non-zero step over 1,000 steps, ns (the header's `machine.clock_res_ns`).
fn clock_res_ns() -> u64 {
    let mut least = u128::MAX;
    let mut last = Instant::now();
    let mut seen = 0;
    for _ in 0..10_000_000 {
        let now = Instant::now();
        let step = now.duration_since(last).as_nanos();
        if step > 0 {
            least = least.min(step);
            last = now;
            seen += 1;
            if seen == 1_000 {
                break;
            }
        }
    }
    u64::try_from(least).unwrap_or(1).max(1)
}

/// The running binary's SHA-256, by a loop of reads into a fixed buffer (SEC-U6AD-5).
fn binary_sha256() -> io::Result<String> {
    let mut f = File::open(std::env::current_exe()?)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// npt-super's log (made new): process `super`, this machine, this build.
fn log(f: &Flags) -> io::Result<Writer<io::LineWriter<File>>> {
    let h = &f.header;
    let header = Header {
        schema: npt_log::SCHEMA,
        run: h[0].clone(),
        set: h[1].clone(),
        role: h[2].clone(),
        marks: f.marks.clone(),
        machine: Machine::read_local(&h[7], &h[8], &h[9], &h[10], &h[11], clock_res_ns()),
        build: Build {
            commit: h[4].clone(),
            binary_sha256: binary_sha256()?,
            features: vec![],
            profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .to_owned(),
        },
        // The run's bite, rate and scene, as the room's header has them (NR-U6A-1); npt-super itself runs no ticks.
        bite: f.bite.clone(),
        layer: h[5].clone(),
        profile: h[6].clone(),
        seed: 0,
        tick_hz: f.tick_hz,
        clients: 0,
        scene: f.scene.clone(),
        process: "super".to_owned(),
        slot: 0,
        date: h[3].clone(),
        params: Default::default(),
    };
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&f.log)?;
    // F28 (U8's fix pass): line-flushed, so each record reaches the file as it is written (npt-log's Writer ends each
    // record with its newline): r1.sh reads `room_start` and `room_exit` while npt-super still runs.
    Writer::new(io::LineWriter::new(file), &header)
}

/// One room started: its process, and the time from spawn to its `OPEN` line (or to the deadline).
struct Started {
    child: Child,
    spawn_to_open_ns: u64,
    open: bool,
}

/// Starts `room --settings <settings>` and times spawn to its `OPEN` line; a room not open by `deadline` is killed
/// and waited for, its elapsed time kept.
fn start(room: &Path, settings: &str, deadline: Duration) -> io::Result<Started> {
    let t0 = Instant::now();
    let mut child = Command::new(room)
        .arg("--settings")
        .arg(settings)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let out = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("no stdout"))?;
    let (tx, rx) = channel();
    // The room's stdout, a bounded line at a time, until it closes; the first `OPEN` line's time is sent.
    std::thread::spawn(move || {
        let mut r = BufReader::new(out);
        let mut sent = false;
        loop {
            let mut line = Vec::new();
            match (&mut r).take(LINE_MAX).read_until(b'\n', &mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) if !sent && line == b"OPEN\n" => {
                    sent = true;
                    let _ = tx.send(t0.elapsed());
                }
                Ok(_) => {}
            }
        }
    });
    let ns = |d: Duration| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX);
    match rx.recv_timeout(deadline) {
        Ok(at) => Ok(Started {
            child,
            spawn_to_open_ns: ns(at),
            open: true,
        }),
        Err(_) => {
            let elapsed = t0.elapsed();
            let _ = child.kill();
            Ok(Started {
                child,
                spawn_to_open_ns: ns(elapsed),
                open: false,
            })
        }
    }
}

/// The room's end: its stdin closed, then its exit within `deadline`, else killed and waited for. Returns whether it
/// exited non-zero (a kill or a signal counts as non-zero).
fn end(mut s: Started, deadline: Duration) -> bool {
    drop(s.child.stdin.take());
    let step = Duration::from_millis(20);
    let mut waited = Duration::ZERO;
    loop {
        match s.child.try_wait() {
            Ok(Some(status)) => return status.code() != Some(0),
            Ok(None) if waited < deadline => {
                std::thread::sleep(step);
                waited += step;
            }
            _ => {
                let _ = s.child.kill();
                let status = s.child.wait();
                return !matches!(status, Ok(st) if st.code() == Some(0));
            }
        }
    }
}

fn started(w: &mut Writer<io::LineWriter<File>>, s: &Started) -> io::Result<()> {
    w.event(
        &EventRecord::new("room_start", Phase::Measured)
            .with("spawn_to_open_ns", s.spawn_to_open_ns),
    )
}

fn exited(w: &mut Writer<io::LineWriter<File>>, nonzero: bool) -> io::Result<()> {
    w.event(&EventRecord::new("room_exit", Phase::Measured).with("nonzero", i64::from(nonzero)))
}

/// `time-starts`: N starts one at a time, each closed once open (the module text).
fn time_starts(f: &Flags, w: &mut Writer<io::LineWriter<File>>) -> io::Result<()> {
    for n in 1..=f.count {
        let settings = f.settings.replace("{n}", &n.to_string());
        let s = start(&f.room, &settings, f.deadline)?;
        started(w, &s)?;
        let nonzero = end(s, f.deadline);
        exited(w, nonzero)?;
    }
    Ok(())
}

/// `start`, the hold form: one start, held until `stop` says npt-super's end came (or the room ends first).
fn hold(f: &Flags, w: &mut Writer<io::LineWriter<File>>, stop: &Receiver<()>) -> io::Result<()> {
    let settings = f.settings.replace("{n}", "1");
    let mut s = start(&f.room, &settings, f.deadline)?;
    started(w, &s)?;
    while s.open {
        match stop.recv_timeout(Duration::from_millis(50)) {
            Ok(()) | Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {
                if matches!(s.child.try_wait(), Ok(Some(_))) {
                    break;
                }
            }
        }
    }
    let nonzero = end(s, f.deadline);
    exited(w, nonzero)
}

/// npt-super's own stdin closing, as a channel (read into a fixed buffer and dropped).
fn stdin_closes() -> Receiver<()> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let mut buf = [0u8; 256];
        let mut stdin = io::stdin().lock();
        while matches!(stdin.read(&mut buf), Ok(n) if n > 0) {}
        let _ = tx.send(());
    });
    rx
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((cmd, rest)) = args.split_first() else {
        eprintln!("usage: npt-super start|time-starts <flags>");
        return ExitCode::from(2);
    };
    let flags = match parse(rest) {
        Ok(f) if cmd == "start" || cmd == "time-starts" => f,
        Ok(_) => {
            eprintln!("npt-super: {cmd:?} is not start or time-starts");
            return ExitCode::from(2);
        }
        Err(e) => {
            eprintln!("npt-super: {e}");
            return ExitCode::from(2);
        }
    };
    let mut w = match log(&flags) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("npt-super: the log: {e}");
            return ExitCode::from(1);
        }
    };
    let run = if cmd == "start" {
        hold(&flags, &mut w, &stdin_closes())
    } else {
        time_starts(&flags, &mut w)
    };
    let done = run.and_then(|()| w.finish(Ended::Clean).map(|_| ()));
    match done {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("npt-super: {e}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    /// A folder in the system's temporary folder, new for this test.
    fn folder(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("npt-u6a-super-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the test's folder");
        dir
    }

    /// One fake-room test at a time (TR-U6A-4 (c)), held across `fake()` and the room's spawns: a script written while
    /// another test forks can be held open for writing by that child, and its exec then fails ("Text file busy").
    static FAKE: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn one_fake() -> std::sync::MutexGuard<'static, ()> {
        FAKE.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// A fake room in `dir`: an executable script with `body` (it ignores its arguments; no shell runs it from npt-super,
    /// which execs the file by its path).
    fn fake(dir: &Path, body: &str) -> PathBuf {
        let path = dir.join("room");
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("written");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("executable");
        path
    }

    fn flags(dir: &Path, room: PathBuf, count: u32, deadline_ms: u64, extra: &[&str]) -> Flags {
        let mut args: Vec<String> = [
            "--room",
            room.to_str().expect("utf-8"),
            "--settings",
            "start-{n}.json",
            "--log",
            dir.join("super.jsonl").to_str().expect("utf-8"),
            "--count",
            &count.to_string(),
            "--deadline-ms",
            &deadline_ms.to_string(),
            "--marks",
            "PM-0001-16",
            "--scene",
            "SS",
            "--tick-hz",
            "30",
        ]
        .iter()
        .chain(extra)
        .map(|s| (*s).to_owned())
        .collect();
        for (i, f) in HEADER_FLAGS.iter().enumerate() {
            args.push((*f).to_owned());
            args.push(match i {
                3 => "2026-11-22".to_owned(),
                4 => "0".repeat(40),
                11 => String::new(),
                _ => "C".to_owned(),
            });
        }
        parse(&args).expect("the flags parse")
    }

    fn events(dir: &Path) -> (Vec<(String, i64)>, Option<Ended>) {
        let text = std::fs::read_to_string(dir.join("super.jsonl")).expect("the log");
        let log = npt_log::read_log(text.as_bytes()).expect("reads back");
        assert_eq!(log.header.process, "super");
        let ev = log
            .events
            .iter()
            .map(|e| {
                let k = if e.ev == "room_start" {
                    "spawn_to_open_ns"
                } else {
                    "nonzero"
                };
                (e.ev.clone(), e.f[k].as_i64().expect("an integer"))
            })
            .collect();
        (ev, log.footer.map(|f| f.ended))
    }

    /// Against a fake room that prints `OPEN` 200 ms after its start and exits 0 once its stdin closes: `time-starts`
    /// writes each start's spawn to Open (at least 200 ms, within the 5 s cap) and its exit (0), and the hold form
    /// keeps the room until its own end comes, then records the exit.
    #[test]
    fn h_time_starts_and_the_hold_form_time_a_fake_room_to_open_and_record_its_exit() {
        let _one = one_fake();
        let dir = folder("fake");
        let room = fake(&dir, "sleep 0.2\necho OPEN\ncat > /dev/null");
        let f = flags(&dir, room.clone(), 2, 5_000, &[]);
        let mut w = log(&f).expect("the log");
        time_starts(&f, &mut w).expect("two starts");
        let (tx, rx) = channel();
        // Taken before the ender starts (TR-U6A-4 (d)), so the hold is timed from no later than the ender's clock.
        let held = Instant::now();
        let ender = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            let _ = tx.send(());
        });
        hold(&f, &mut w, &rx).expect("the held start");
        let held = held.elapsed();
        ender.join().expect("the ender");
        w.finish(Ended::Clean).expect("the footer");
        let (ev, footer) = events(&dir);
        println!("fake room: {ev:?}; held {} ms", held.as_millis());
        let names: Vec<&str> = ev.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(
            names,
            [
                "room_start",
                "room_exit",
                "room_start",
                "room_exit",
                "room_start",
                "room_exit"
            ]
        );
        for (name, x) in &ev {
            match name.as_str() {
                "room_start" => assert!((200_000_000..5_000_000_000).contains(x), "{x} ns"),
                _ => assert_eq!(*x, 0, "a clean exit"),
            }
        }
        assert!(
            held >= Duration::from_millis(300),
            "held until its end came"
        );
        assert_eq!(footer, Some(Ended::Clean));
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    /// A silent room (never `OPEN`) is killed at the deadline, waited for, its elapsed time written as its spawn to
    /// Open, and its exit recorded as non-zero.
    #[test]
    fn h_a_silent_room_is_killed_at_the_deadline_with_its_elapsed_time_written() {
        let _one = one_fake();
        let dir = folder("silent");
        let room = fake(&dir, "exec sleep 30");
        let f = flags(&dir, room, 1, 500, &[]);
        let mut w = log(&f).expect("the log");
        let t0 = Instant::now();
        time_starts(&f, &mut w).expect("one start");
        let took = t0.elapsed();
        w.finish(Ended::Clean).expect("the footer");
        let (ev, _) = events(&dir);
        println!("silent room: {ev:?}; took {} ms", took.as_millis());
        assert_eq!(ev.len(), 2);
        assert!(
            ev[0].1 >= 500_000_000,
            "the elapsed time at the deadline: {} ns",
            ev[0].1
        );
        assert_eq!(ev[1], ("room_exit".to_owned(), 1), "killed: non-zero");
        assert!(took < Duration::from_secs(5), "killed, not waited out");
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    /// The header carries the run's bite, scene and tick rate (`--bite` here; `--scene SS --tick-hz 30` from
    /// `flags`), so npt-super's log sits in its run beside the room's (NR-U6A-1); against a fake room that opens after
    /// 2.1 s (the double-load bite's delay), this log's own `spawn_to_open_ns` fails PM-0001-16's check "every start,
    /// process spawn to Open, under 2 s" (max < 2,000,000,000) by name.
    #[test]
    fn h_the_log_header_carries_the_runs_bite_scene_and_tick_rate() {
        let _one = one_fake();
        let dir = folder("header");
        let room = fake(&dir, "sleep 2.1\necho OPEN\ncat > /dev/null");
        let f = flags(&dir, room, 1, 5_000, &["--bite", "PM-0001-16/double-load"]);
        let mut w = log(&f).expect("the log");
        time_starts(&f, &mut w).expect("one start");
        w.finish(Ended::Clean).expect("the footer");
        let text = std::fs::read_to_string(dir.join("super.jsonl")).expect("the log");
        let h = npt_log::read_log(text.as_bytes())
            .expect("reads back")
            .header;
        assert_eq!(
            (h.bite.as_deref(), h.scene.as_str(), h.tick_hz),
            (Some("PM-0001-16/double-load"), "SS", 30)
        );
        let (ev, _) = events(&dir);
        let max = ev
            .iter()
            .filter(|(n, _)| n == "room_start")
            .map(|(_, x)| *x)
            .max();
        println!(
            "npt-super's header: {:?}, {}, {} Hz; max spawn to OPEN {max:?} ns",
            h.bite, h.scene, h.tick_hz
        );
        let max = max.expect("one start");
        assert!(
            max >= 2_000_000_000,
            "PM-0001-16's check 'every start, process spawn to Open, under 2 s' fails on npt-super's log: max {max} ns"
        );
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    /// F28: npt-super's log reaches its file record by record: while the hold form still holds its room (npt-super
    /// running), `room_start` is read back from the file with no footer yet; `room_exit` follows once the room ends.
    #[test]
    fn u6b_each_record_reaches_the_file_while_npt_super_runs() {
        let _one = one_fake();
        let dir = folder("flush");
        let room = fake(&dir, "echo OPEN\ncat > /dev/null");
        let f = flags(&dir, room, 1, 5_000, &[]);
        let (tx, rx) = channel();
        let held = std::thread::spawn(move || {
            let mut w = log(&f).expect("the log");
            hold(&f, &mut w, &rx).expect("the held start");
            w
        });
        let read = || {
            let text = std::fs::read_to_string(dir.join("super.jsonl")).unwrap_or_default();
            npt_log::read_log(text.as_bytes()).ok()
        };
        let t0 = Instant::now();
        let mut live = None;
        while t0.elapsed() < Duration::from_secs(5) {
            if let Some(l) = read().filter(|l| !l.events.is_empty()) {
                live = Some(l);
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let live = live.expect("room_start on disk while npt-super runs");
        assert_eq!(live.events[0].ev, "room_start");
        assert!(live.footer.is_none(), "npt-super has not ended");
        tx.send(()).expect("the end");
        let w = held.join().expect("the holder");
        let after = read().expect("reads");
        assert_eq!(after.events.len(), 2, "room_exit on disk before the footer");
        assert!(after.footer.is_none());
        w.finish(Ended::Clean).expect("the footer");
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    #[test]
    fn h_the_flags_are_refused_unknown_missing_or_out_of_range() {
        assert!(parse(&["--nope".to_owned(), "x".to_owned()]).is_err());
        assert!(
            parse(&["--room".to_owned()]).is_err(),
            "a flag with no value"
        );
        assert!(
            parse(&["--room".to_owned(), "r".to_owned()]).is_err(),
            "the rest missing"
        );
        assert!(
            parse(&["--tick-hz".to_owned(), "0".to_owned()])
                .is_err_and(|e| e.contains("1 to 1,000")),
            "a rate out of range"
        );
    }
}
