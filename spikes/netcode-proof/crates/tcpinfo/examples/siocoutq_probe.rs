//! The probe behind u3c-plan.md 1.8 (e)'s root run, which tells SIOCOUTQ (`write_seq - snd_una`, tcp.c:630) from
//! SIOCOUTQNSD (`write_seq - snd_nxt`, tcp.c:632-640): it needs bytes that are sent but not acknowledged, so the ACK
//! direction is dropped for about 1 s while the server end writes. The script
//! `spikes/netcode-proof/results/u3c-kernel-check/siocoutq-root-check.py` adds and removes the iptables rule (this
//! program never touches the network's configuration) and holds the pass rule, which this program evaluates.
//!
//! Modes:
//! - `siocoutq_probe lockstep --port P`: the script's protocol. stdout: `INFO`, then `READY`; stdin: `GO` (the rule is
//!   in); stdout: one `SAMPLE` per write and per hold step, then `HOLD-END`; stdin: `RELEASED` (the rule is out);
//!   stdout: the release sample, one `RULE` line per rule and `VERDICT PASS|FAIL|VOID`. Exit 0 pass, 1 fail, 2 void.
//! - `siocoutq_probe zero-window`: a check without root of the sock_diag reader below: a client that never reads,
//!   with a small receive buffer, closes its window; then nothing is in flight, so tcpi_notsent_bytes must equal
//!   SIOCOUTQ, /proc's tx_queue and sock_diag's wqueue, all non-zero. Exit 0 pass, 1 fail.
//!
//! tcpi_notsent_bytes (`write_seq - snd_nxt`, tcp.c:3854) is read independently of npt-tcpinfo: a NETLINK_SOCK_DIAG
//! dump with INET_DIAG_INFO, the kernel's own tcp_info for the socket, through socket2's safe calls (no unsafe code).

use std::process::ExitCode;

fn main() -> ExitCode {
    #[cfg(target_os = "linux")]
    {
        probe::main()
    }
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("siocoutq_probe: Linux only");
        ExitCode::from(2)
    }
}

#[cfg(target_os = "linux")]
mod probe {
    use std::io::{self, BufRead, Read, Write};
    use std::mem::offset_of;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::os::fd::AsFd;
    use std::process::ExitCode;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;
    use std::time::{Duration, Instant};

    use npt_tcpinfo::{srtt_us, unacked_and_unsent};
    use socket2::{Domain, Protocol, SockRef, Socket, Type};

    /// The run's fixed shape (the script's header states the same numbers).
    const MSG: usize = 200;
    /// The warm-up's messages, and the most the write phase makes.
    const WRITES: u64 = 20;
    /// Fewer writes than this (a cwnd under 5) leave too little to separate SIOCOUTQ from SIOCOUTQNSD: VOID.
    const MIN_WRITES: u64 = 4;
    /// The cwnd room the write phase leaves free: one packet for a tail loss probe (a retransmission, so in flight).
    const LOSS_PROBE_ROOM: i64 = 1;
    const GAP: Duration = Duration::from_millis(5);
    const HOLD_FIRST: Duration = Duration::from_millis(200);
    const HOLD_STEP: Duration = Duration::from_millis(100);
    const HOLD_END: Duration = Duration::from_millis(1_000);
    /// "Near 0": at most one message's bytes not yet sent.
    const NOTSENT_NEAR: u32 = MSG as u32;
    /// Waits that only stop a hang (never a pass criterion).
    const DELIVERY_WAIT: Duration = Duration::from_millis(100);
    const SETTLE_WAIT: Duration = Duration::from_secs(2);
    /// R4's wait after RELEASED, in RTOs read at RELEASED: the pending retransmission is due within one (see the
    /// script's header); the second is slack.
    const RELEASE_RTOS: u32 = 2;

    // include/uapi/linux/netlink.h, sock_diag.h, inet_diag.h, tcp_states.h (libc's Linux module lacks these).
    const NETLINK_SOCK_DIAG: i32 = 4;
    const NLM_F_REQUEST: u16 = 0x1;
    const NLM_F_DUMP: u16 = 0x300;
    const NLMSG_ERROR: u16 = 2;
    const NLMSG_DONE: u16 = 3;
    const SOCK_DIAG_BY_FAMILY: u16 = 20;
    const INET_DIAG_INFO: u16 = 2;
    const TCP_ESTABLISHED: u32 = 1;
    /// struct nlmsghdr (16 B), then struct inet_diag_msg (72 B), then the attributes.
    const NLMSG_HDR: usize = 16;
    const DIAG_MSG: usize = 72;

    /// One socket's sock_diag reading: inet_diag_msg's queues and the kernel's tcp_info fields this run needs.
    #[derive(Debug, Clone, Copy)]
    struct Diag {
        /// inet_diag_msg.idiag_wqueue: `write_seq - snd_una` for an established TCP socket (tcp_diag_get_info).
        wqueue: u32,
        notsent: u32,
        unacked_pkts: u32,
        total_retrans: u32,
        rto_us: u32,
        rtt_us: u32,
        backoff: u8,
        ca_state: u8,
        snd_cwnd: u32,
        snd_mss: u32,
        sacked: u32,
        lost: u32,
        retrans: u32,
    }

    impl Diag {
        /// The kernel's packets in flight, as tcp_cwnd_test counts them (v6.8 include/net/tcp.h:1253-1256,
        /// tcp_output.c:2083-2086): packets_out - (sacked_out + lost_out) + retrans_out, which tcp_info reports as
        /// tcpi_unacked, tcpi_sacked, tcpi_lost and tcpi_retrans (net/ipv4/tcp.c:3829-3833).
        fn in_flight(&self) -> i64 {
            i64::from(self.unacked_pkts) - i64::from(self.sacked) - i64::from(self.lost)
                + i64::from(self.retrans)
        }
    }

    fn u32_at(b: &[u8], at: usize) -> Option<u32> {
        b.get(at..at + 4)
            .and_then(|s| s.try_into().ok())
            .map(u32::from_ne_bytes)
    }

    fn u16_at(b: &[u8], at: usize) -> Option<u16> {
        b.get(at..at + 2)
            .and_then(|s| s.try_into().ok())
            .map(u16::from_ne_bytes)
    }

    /// The tcp_info fields at their offset_of! in the attribute's bytes; None if the attribute is too short.
    fn tcp_info_fields(info: &[u8], wqueue: u32) -> Option<Diag> {
        Some(Diag {
            wqueue,
            notsent: u32_at(info, offset_of!(libc::tcp_info, tcpi_notsent_bytes))?,
            unacked_pkts: u32_at(info, offset_of!(libc::tcp_info, tcpi_unacked))?,
            total_retrans: u32_at(info, offset_of!(libc::tcp_info, tcpi_total_retrans))?,
            rto_us: u32_at(info, offset_of!(libc::tcp_info, tcpi_rto))?,
            rtt_us: u32_at(info, offset_of!(libc::tcp_info, tcpi_rtt))?,
            backoff: *info.get(offset_of!(libc::tcp_info, tcpi_backoff))?,
            ca_state: *info.get(offset_of!(libc::tcp_info, tcpi_ca_state))?,
            snd_cwnd: u32_at(info, offset_of!(libc::tcp_info, tcpi_snd_cwnd))?,
            snd_mss: u32_at(info, offset_of!(libc::tcp_info, tcpi_snd_mss))?,
            sacked: u32_at(info, offset_of!(libc::tcp_info, tcpi_sacked))?,
            lost: u32_at(info, offset_of!(libc::tcp_info, tcpi_lost))?,
            retrans: u32_at(info, offset_of!(libc::tcp_info, tcpi_retrans))?,
        })
    }

    /// The established IPv4 TCP socket bound to `sport` and connected to `dport`, read through a NETLINK_SOCK_DIAG
    /// dump (every established IPv4 TCP socket in this network namespace, filtered here by its ports).
    fn diag(sport: u16, dport: u16) -> io::Result<Option<Diag>> {
        let nl = Socket::new(
            Domain::from(libc::AF_NETLINK),
            Type::RAW,
            Some(Protocol::from(NETLINK_SOCK_DIAG)),
        )?;
        let mut req = Vec::with_capacity(NLMSG_HDR + 56);
        req.extend_from_slice(&((NLMSG_HDR + 56) as u32).to_ne_bytes());
        req.extend_from_slice(&SOCK_DIAG_BY_FAMILY.to_ne_bytes());
        req.extend_from_slice(&(NLM_F_REQUEST | NLM_F_DUMP).to_ne_bytes());
        req.extend_from_slice(&1u32.to_ne_bytes()); // nlmsg_seq
        req.extend_from_slice(&0u32.to_ne_bytes()); // nlmsg_pid
        // struct inet_diag_req_v2: family, protocol, ext (the INET_DIAG_INFO bit), pad, states, then a 48 B sockid a
        // dump ignores.
        req.extend_from_slice(&[
            libc::AF_INET as u8,
            libc::IPPROTO_TCP as u8,
            1 << (INET_DIAG_INFO - 1),
            0,
        ]);
        req.extend_from_slice(&(1u32 << TCP_ESTABLISHED).to_ne_bytes());
        req.extend_from_slice(&[0u8; 48]);
        nl.send(&req)?;
        let mut buf = vec![0u8; 1 << 16];
        let mut found = None;
        loop {
            let n = (&nl).read(&mut buf)?;
            let mut at = 0;
            while at + NLMSG_HDR <= n {
                let len = u32_at(&buf, at).unwrap_or(0) as usize;
                let kind = u16_at(&buf, at + 4).unwrap_or(0);
                if len < NLMSG_HDR || at + len > n {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "a truncated netlink message",
                    ));
                }
                let msg = &buf[at + NLMSG_HDR..at + len];
                match kind {
                    NLMSG_DONE => return Ok(found),
                    NLMSG_ERROR => {
                        let errno = msg
                            .get(..4)
                            .and_then(|s| s.try_into().ok())
                            .map(i32::from_ne_bytes)
                            .unwrap_or(0);
                        return Err(io::Error::from_raw_os_error(-errno));
                    }
                    SOCK_DIAG_BY_FAMILY if msg.len() >= DIAG_MSG => {
                        // inet_diag_msg: family, state, timer, retrans; the sockid (sport, dport in network order);
                        // expires, rqueue, wqueue, uid, inode.
                        let sp = u16::from_be_bytes([msg[4], msg[5]]);
                        let dp = u16::from_be_bytes([msg[6], msg[7]]);
                        if sp == sport && dp == dport {
                            let wqueue = u32_at(msg, 60).unwrap_or(0);
                            let mut a = DIAG_MSG;
                            while a + 4 <= msg.len() {
                                let rlen = usize::from(u16_at(msg, a).unwrap_or(0));
                                let rtype = u16_at(msg, a + 2).unwrap_or(0);
                                if rlen < 4 || a + rlen > msg.len() {
                                    break;
                                }
                                if rtype == INET_DIAG_INFO {
                                    found = tcp_info_fields(&msg[a + 4..a + rlen], wqueue);
                                }
                                a += rlen.next_multiple_of(4);
                            }
                        }
                    }
                    _ => {}
                }
                at += len.next_multiple_of(4);
            }
        }
    }

    fn outq(s: &TcpStream) -> io::Result<u32> {
        unacked_and_unsent(s.as_fd())
    }

    /// /proc/net/tcp's tx_queue for the IPv4 socket bound to `local` and connected to `remote` (tcp_ipv4.c:2899).
    fn proc_tx_queue(local: u16, remote: u16) -> io::Result<Option<u32>> {
        let table = std::fs::read_to_string("/proc/net/tcp")?;
        for row in table.lines().skip(1) {
            let f: Vec<&str> = row.split_whitespace().collect();
            let port = |i: usize| {
                f.get(i)
                    .and_then(|a| a.rsplit(':').next())
                    .and_then(|p| u16::from_str_radix(p, 16).ok())
            };
            if port(1) == Some(local) && port(2) == Some(remote) {
                let tx = f
                    .get(4)
                    .and_then(|q| q.split_once(':'))
                    .and_then(|(tx, _)| u32::from_str_radix(tx, 16).ok());
                return Ok(tx);
            }
        }
        Ok(None)
    }

    fn wait_until(limit: Duration, mut done: impl FnMut() -> bool) -> bool {
        let end = Instant::now() + limit;
        loop {
            if done() {
                return true;
            }
            if Instant::now() >= end {
                return false;
            }
            thread::sleep(Duration::from_micros(200));
        }
    }

    fn kernel_release() -> String {
        std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_or_else(|e| format!("unknown ({e})"), |s| s.trim().to_owned())
    }

    pub(crate) fn main() -> ExitCode {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let result = match args.first().map(String::as_str) {
            Some("lockstep") => match (
                args.get(1).map(String::as_str),
                args.get(2).map(|p| p.parse::<u16>()),
            ) {
                (Some("--port"), Some(Ok(port))) if port != 0 => lockstep(port),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "lockstep needs --port <1-65535>",
                )),
            },
            Some("zero-window") => zero_window(),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: siocoutq_probe lockstep --port P | zero-window",
            )),
        };
        match result {
            Ok(code) => code,
            Err(e) => {
                println!("VERDICT VOID probe error: {e}");
                ExitCode::from(2)
            }
        }
    }

    /// One reading: SIOCOUTQ, sock_diag, SIOCOUTQ again (the two agree when no ACK arrived in between).
    struct Sample {
        phase: &'static str,
        k: u64,
        t_us: u128,
        written: u64,
        q1: u32,
        q2: u32,
        delivered: u64,
        diag: Option<Diag>,
    }

    impl Sample {
        fn line(&self) -> String {
            let d = self.diag.map_or_else(
                || "diag=none".to_owned(),
                |d| {
                    format!(
                        "notsent={} wqueue={} unacked_pkts={} in_flight={} snd_cwnd={} snd_mss={} total_retrans={} \
                         rto_us={} rtt_us={} backoff={} ca_state={}",
                        d.notsent,
                        d.wqueue,
                        d.unacked_pkts,
                        d.in_flight(),
                        d.snd_cwnd,
                        d.snd_mss,
                        d.total_retrans,
                        d.rto_us,
                        d.rtt_us,
                        d.backoff,
                        d.ca_state
                    )
                },
            );
            format!(
                "SAMPLE phase={} k={} t_us={} written={} siocoutq={} siocoutq_again={} delivered={} {d}",
                self.phase, self.k, self.t_us, self.written, self.q1, self.q2, self.delivered
            )
        }
    }

    fn say(line: &str) {
        let mut out = io::stdout().lock();
        // A closed pipe means the script is gone; the probe then ends at its next read of stdin.
        let _ = writeln!(out, "{line}");
        let _ = out.flush();
    }

    fn expect_command(
        lines: &mut impl Iterator<Item = io::Result<String>>,
        want: &str,
    ) -> io::Result<()> {
        match lines.next() {
            Some(Ok(l)) if l.trim() == want => Ok(()),
            Some(Ok(l)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("expected {want}, got {l:?}"),
            )),
            Some(Err(e)) => Err(e),
            None => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("stdin closed before {want}"),
            )),
        }
    }

    fn lockstep(port: u16) -> io::Result<ExitCode> {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)))?;
        let client = TcpStream::connect(listener.local_addr()?)?;
        let (server, _) = listener.accept()?;
        server.set_nodelay(true)?;
        let cport = client.local_addr()?.port();
        // The model the cross-checks compare with is Cubic; this container defaults to bbr (u3-scout-timing.md:388).
        let cc_set = SockRef::from(&server).set_tcp_congestion(b"cubic");
        let cc = SockRef::from(&server).tcp_congestion().map(|b| {
            String::from_utf8_lossy(&b)
                .trim_end_matches('\0')
                .to_owned()
        });

        // The client reads everything, counting the bytes, until the server closes.
        let delivered = Arc::new(AtomicU64::new(0));
        let reader = {
            let delivered = Arc::clone(&delivered);
            let mut client = client.try_clone()?;
            thread::spawn(move || {
                let mut buf = [0u8; 4_096];
                while let Ok(n) = client.read(&mut buf) {
                    if n == 0 {
                        break;
                    }
                    delivered.fetch_add(n as u64, Ordering::SeqCst);
                }
            })
        };

        // Warm-up: RTT samples, then everything acknowledged.
        let mut server_w = server.try_clone()?;
        for _ in 0..WRITES {
            server_w.write_all(&[5; MSG])?;
        }
        let warm = WRITES * MSG as u64;
        let warm_ok = wait_until(SETTLE_WAIT, || delivered.load(Ordering::SeqCst) == warm)
            && wait_until(SETTLE_WAIT, || outq(&server).is_ok_and(|q| q == 0));
        let srtt = srtt_us(server.as_fd())?;
        let d0 = diag(port, cport)?;
        say(&format!(
            "INFO kernel={} port={port} client_port={cport} congestion={} congestion_set={} srtt_us={srtt:?} \
             siocoutq={} notsent={:?} rto_us={:?} total_retrans={:?} snd_cwnd={:?} snd_mss={:?} notsent_offset={} \
             rtt_offset={}",
            kernel_release(),
            cc.as_deref().unwrap_or("unknown"),
            if cc_set.is_ok() { "ok" } else { "refused" },
            outq(&server)?,
            d0.map(|d| d.notsent),
            d0.map(|d| d.rto_us),
            d0.map(|d| d.total_retrans),
            d0.map(|d| d.snd_cwnd),
            d0.map(|d| d.snd_mss),
            offset_of!(libc::tcp_info, tcpi_notsent_bytes),
            offset_of!(libc::tcp_info, tcpi_rtt),
        ));
        // npt-tcpinfo's srtt_us must agree with the kernel's tcp_info read through sock_diag (0 reads as None). The
        // connection is idle, so tcpi_rtt cannot move between the two reads. It is not required to be above 0:
        // loopback's RTT is a few us and tcpi_rtt is in whole us.
        let r0 = warm_ok
            && d0.is_some_and(|d| {
                d.notsent == 0 && d.wqueue == 0 && srtt == Some(d.rtt_us).filter(|&us| us != 0)
            });
        let Some(d0) = d0.filter(|_| r0) else {
            say(
                "RULE R0 fail setup: warm-up not acknowledged, no sock_diag reading, or srtt_us disagrees with it",
            );
            say("VERDICT VOID setup");
            return Ok(ExitCode::from(2));
        };
        say("RULE R0 pass");
        say("READY");

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        expect_command(&mut lines, "GO")?;
        let t0 = Instant::now();
        let base = delivered.load(Ordering::SeqCst);
        let mut samples = Vec::new();
        let sample = |phase, k, written, samples: &mut Vec<Sample>| -> io::Result<()> {
            let q1 = outq(&server)?;
            let diag = diag(port, cport)?;
            let q2 = outq(&server)?;
            let s = Sample {
                phase,
                k,
                t_us: t0.elapsed().as_micros(),
                written,
                q1,
                q2,
                delivered: delivered.load(Ordering::SeqCst) - base,
                diag,
            };
            say(&s.line());
            samples.push(s);
            Ok(())
        };

        // The write phase: up to WRITES x MSG at GAP, all inside the first RTO (checked below). Message k is written
        // only while the kernel would send it at once: the packets in flight, read just before it, plus one stay
        // below the cwnd (tcp_cwnd_test's own condition, with one packet kept for a loss probe, which may go out
        // between the reading and the write and counts in flight). With no ACK coming back, the first message the
        // cwnd held back would stay unsent (tcpi_notsent_bytes 200) and never reach the peer, so the phase stops
        // there instead.
        let mut last_write_us = 0;
        let mut writes = 0;
        let mut stopped = format!("all {WRITES} writes made");
        for k in 1..=WRITES {
            let due = t0 + GAP * u32::try_from(k - 1).unwrap_or(u32::MAX);
            if let Some(wait) = due.checked_duration_since(Instant::now()) {
                thread::sleep(wait);
            }
            let Some(gate) = diag(port, cport)? else {
                stopped = format!("no sock_diag reading before write {k}");
                break;
            };
            if gate.in_flight() + LOSS_PROBE_ROOM >= i64::from(gate.snd_cwnd) {
                stopped = format!(
                    "write {k} not made: {} packets in flight + {LOSS_PROBE_ROOM} for a loss probe >= snd_cwnd {}",
                    gate.in_flight(),
                    gate.snd_cwnd
                );
                break;
            }
            server_w.write_all(&[6; MSG])?;
            writes = k;
            last_write_us = t0.elapsed().as_micros();
            let written = k * MSG as u64;
            wait_until(DELIVERY_WAIT, || {
                delivered.load(Ordering::SeqCst) - base >= written
            });
            sample("write", k, written, &mut samples)?;
        }
        say(&format!("INFO writes={writes} ({stopped})"));
        // The hold: the drop stays in until about HOLD_END after GO.
        let total = writes * MSG as u64;
        let mut next = HOLD_FIRST;
        let mut i = 0;
        while next <= HOLD_END {
            if let Some(wait) = (t0 + next).checked_duration_since(Instant::now()) {
                thread::sleep(wait);
            }
            i += 1;
            sample("hold", i, total, &mut samples)?;
            next += HOLD_STEP;
        }
        say("HOLD-END");
        expect_command(&mut lines, "RELEASED")?;
        let released = Instant::now();
        // R4's bound: RELEASE_RTOS x the RTO the kernel holds at RELEASED (tcpi_rto, after any backoff).
        let at_release = diag(port, cport)?;
        let bound = at_release.map(|d| Duration::from_micros(u64::from(d.rto_us)) * RELEASE_RTOS);
        let zero = wait_until(bound.unwrap_or(Duration::ZERO), || {
            outq(&server).is_ok_and(|q| q == 0)
        });
        let release_ms = released.elapsed().as_millis();
        sample("release", 1, total, &mut samples)?;

        // The rule, as the script's header states it.
        let mut fails = Vec::new();
        let mut void = Vec::new();
        if last_write_us >= u128::from(d0.rto_us) {
            void.push(format!(
                "the last write came {last_write_us} us after GO, not before the first RTO ({} us)",
                d0.rto_us
            ));
        }
        if (d0.snd_mss as usize) < MSG {
            void.push(format!(
                "snd_mss {} < {MSG} B: a message would not be one segment",
                d0.snd_mss
            ));
        }
        if writes < MIN_WRITES {
            void.push(format!(
                "{writes} writes, fewer than {MIN_WRITES} ({stopped})"
            ));
        }
        if at_release.is_none() {
            void.push("no sock_diag reading at RELEASED, so no R4 bound".to_owned());
        }
        for s in samples.iter().filter(|s| s.phase != "release") {
            let Some(d) = s.diag else {
                void.push(format!("{} {}: no sock_diag reading", s.phase, s.k));
                continue;
            };
            let want = u32::try_from(s.written).unwrap_or(u32::MAX);
            let rule = if s.phase == "write" { "R1" } else { "R2" };
            if s.q1 != want || s.q2 != want {
                fails.push(format!(
                    "{rule} {} {}: SIOCOUTQ {} then {}, not {want}",
                    s.phase, s.k, s.q1, s.q2
                ));
            }
            if s.delivered < s.written {
                fails.push(format!(
                    "{rule} {} {}: the peer read {} of {}",
                    s.phase, s.k, s.delivered, s.written
                ));
            }
            if d.notsent > NOTSENT_NEAR {
                fails.push(format!(
                    "{rule} {} {}: tcpi_notsent_bytes {} > {NOTSENT_NEAR}",
                    s.phase, s.k, d.notsent
                ));
            }
        }
        let retrans_end = samples
            .iter()
            .filter(|s| s.phase == "hold")
            .filter_map(|s| s.diag)
            .map(|d| d.total_retrans)
            .next_back();
        if !retrans_end.is_some_and(|r| r > d0.total_retrans) {
            fails.push(format!(
                "R3: tcpi_total_retrans {} at GO, {retrans_end:?} at the hold's end: no retransmission, so no ACK was dropped",
                d0.total_retrans
            ));
        }
        if !zero {
            fails.push(format!(
                "R4: SIOCOUTQ not 0 within {RELEASE_RTOS} x tcpi_rto at RELEASED ({bound:?})"
            ));
        }
        if let Some(s) = samples.iter().rev().find(|s| s.phase == "release") {
            let wqueue = s.diag.map(|d| d.wqueue);
            if s.q1 != 0 || s.q2 != 0 || wqueue != Some(0) || s.delivered != total {
                fails.push(format!(
                    "R4: at release SIOCOUTQ {} then {}, wqueue {wqueue:?}, the peer read {} of {total}",
                    s.q1, s.q2, s.delivered
                ));
            }
        }
        for r in ["R1", "R2", "R3", "R4"] {
            let mine: Vec<&String> = fails.iter().filter(|f| f.starts_with(r)).collect();
            if mine.is_empty() {
                say(&format!("RULE {r} pass"));
            } else {
                say(&format!(
                    "RULE {r} fail ({} findings; first: {})",
                    mine.len(),
                    mine[0]
                ));
            }
        }
        say(&format!(
            "INFO release_ms={release_ms} r4_bound={bound:?} rto_us_at_release={:?} backoff_at_release={:?} \
             last_write_us={last_write_us} writes={writes}",
            at_release.map(|d| d.rto_us),
            at_release.map(|d| d.backoff)
        ));

        drop(server_w);
        let _ = server.shutdown(std::net::Shutdown::Both);
        let _ = reader.join();
        Ok(if !void.is_empty() {
            say(&format!("VERDICT VOID {}", void.join("; ")));
            ExitCode::from(2)
        } else if fails.is_empty() {
            say("VERDICT PASS");
            ExitCode::SUCCESS
        } else {
            say(&format!("VERDICT FAIL {} findings", fails.len()));
            ExitCode::from(1)
        })
    }

    /// The sock_diag reader's own check, without root: a closed window leaves nothing in flight, so the unsent bytes
    /// equal the unacknowledged ones, and four readings must agree.
    fn zero_window() -> io::Result<ExitCode> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let to = listener.local_addr()?;
        let c = Socket::new(Domain::IPV4, Type::STREAM, None)?;
        c.set_recv_buffer_size(4_096)?;
        c.connect(&to.into())?;
        let client: TcpStream = c.into();
        let (mut server, _) = listener.accept()?;
        server.set_nodelay(true)?;
        server.set_nonblocking(true)?;
        let (sp, cp) = (to.port(), client.local_addr()?.port());
        let mut written = 0u64;
        loop {
            match server.write(&[1; 1_000]) {
                Ok(n) => written += n as u64,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        let mut last = None;
        let agree = wait_until(Duration::from_secs(5), || {
            let (Ok(q1), Ok(Some(d)), Ok(Some(tx)), Ok(q2)) = (
                outq(&server),
                diag(sp, cp),
                proc_tx_queue(sp, cp),
                outq(&server),
            ) else {
                return false;
            };
            last = Some((q1, q2, d.notsent, d.wqueue, tx));
            q1 == q2 && q1 > 0 && d.notsent == q1 && d.wqueue == q1 && tx == q1
        });
        say(&format!(
            "SELFCHECK written={written} (SIOCOUTQ, again, tcpi_notsent_bytes, wqueue, tx_queue)={last:?}"
        ));
        drop(client);
        Ok(if agree {
            say("SELFCHECK PASS");
            ExitCode::SUCCESS
        } else {
            say("SELFCHECK FAIL");
            ExitCode::from(1)
        })
    }
}
