//! The crate's tests (u3c-plan.md 1.8 (e)), on real loopback sockets. Each is judged on identities between two kernel
//! readings of the same quantity (SIOCOUTQ against /proc/net/tcp's tx_queue, both `write_seq - snd_una`: tcp.c:630,
//! tcp_ipv4.c:2899), never on a wall-clock threshold; a time limit only stops a wait from hanging. The root-only
//! check (the ACK direction dropped by iptables) is the probe example's, run by
//! results/u3c-kernel-check/siocoutq-root-check.py.

use std::io::{ErrorKind, Read, Write};
use std::mem::offset_of;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::os::fd::AsFd;
use std::os::unix::net::{UnixDatagram, UnixStream};
use std::time::{Duration, Instant};

use socket2::{Domain, Protocol, SockRef, Socket, Type};

use crate::linux::{SIOCOUTQ, rtt_from};
use crate::{srtt_us, unacked_and_unsent};

/// A connected loopback pair: (listener, the accepted server end S, the client end C). The client's SO_RCVBUF is set
/// before connect when given, so the window it advertises is small from the handshake on.
fn pair(client_rcvbuf: Option<usize>) -> (TcpListener, TcpStream, TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let to: SocketAddr = listener.local_addr().expect("listener address");
    let client = Socket::new(Domain::IPV4, Type::STREAM, None).expect("socket");
    if let Some(bytes) = client_rcvbuf {
        client.set_recv_buffer_size(bytes).expect("SO_RCVBUF");
    }
    client.connect(&to.into()).expect("connect");
    let (server, _) = listener.accept().expect("accept");
    server.set_nodelay(true).expect("TCP_NODELAY");
    (listener, server, client.into())
}

fn port(s: &TcpStream) -> u16 {
    s.local_addr().expect("local address").port()
}

fn outq(s: &TcpStream) -> u32 {
    unacked_and_unsent(s.as_fd()).expect("SIOCOUTQ")
}

/// One read of /proc/net/tcp: (tx_queue, rx_queue) of the IPv4 socket bound to `local` and connected to `remote`, and
/// of its peer, the socket bound to `remote` and connected to `local`. tx_queue is `write_seq - snd_una`, rx_queue
/// `rcv_nxt - copied_seq` (tcp_ipv4.c:2893-2900).
fn proc_queues(local: u16, remote: u16) -> ((u32, u32), (u32, u32)) {
    let table = std::fs::read_to_string("/proc/net/tcp").expect("/proc/net/tcp");
    let mut mine = None;
    let mut peer = None;
    for row in table.lines().skip(1) {
        let f: Vec<&str> = row.split_whitespace().collect();
        let hex_port = |field: &str| {
            field
                .rsplit(':')
                .next()
                .and_then(|p| u16::from_str_radix(p, 16).ok())
        };
        let (Some(l), Some(r), Some(q)) = (f.get(1), f.get(2), f.get(4)) else {
            continue;
        };
        let Some((tx, rx)) = q.split_once(':') else {
            continue;
        };
        let queues = (
            u32::from_str_radix(tx, 16).expect("tx_queue"),
            u32::from_str_radix(rx, 16).expect("rx_queue"),
        );
        match (hex_port(l), hex_port(r)) {
            (Some(a), Some(b)) if a == local && b == remote => mine = Some(queues),
            (Some(a), Some(b)) if a == remote && b == local => peer = Some(queues),
            _ => {}
        }
    }
    (
        mine.expect("the socket's row"),
        peer.expect("its peer's row"),
    )
}

/// Polls `done` every 0.5 ms until it holds or `limit` passes; whether it held. The limit only stops a hang.
fn wait_until(limit: Duration, mut done: impl FnMut() -> bool) -> bool {
    let end = Instant::now() + limit;
    loop {
        if done() {
            return true;
        }
        if Instant::now() >= end {
            return false;
        }
        std::thread::sleep(Duration::from_micros(500));
    }
}

/// Ping-pongs `rounds` 100 B messages each way, so both ends have RTT samples and every byte is acknowledged.
fn exchange(s: &mut TcpStream, c: &mut TcpStream, rounds: usize) {
    let mut buf = [0u8; 100];
    for _ in 0..rounds {
        s.write_all(&[7; 100]).expect("server write");
        c.read_exact(&mut buf).expect("client read");
        c.write_all(&[8; 100]).expect("client write");
        s.read_exact(&mut buf).expect("server read");
    }
}

#[test]
fn the_request_is_siocoutq_0x5411_not_siocoutqnsd() {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    assert_eq!(
        SIOCOUTQ, 0x5411,
        "SIOCOUTQ = TIOCOUTQ = 0x5411 (asm-generic/ioctls.h)"
    );
    assert_ne!(
        SIOCOUTQ, 0x894B,
        "SIOCOUTQNSD counts only the unsent bytes (tcp.c:632-640)"
    );
}

#[test]
fn siocoutq_on_a_listening_socket_is_einval() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let err = unacked_and_unsent(listener.as_fd()).expect_err("a listener has no send queue");
    assert_eq!(
        err.raw_os_error(),
        Some(libc::EINVAL),
        "tcp.c:624-625: {err}"
    );
}

/// Any descriptor may be passed (the functions are safe): on anything but a TCP socket each answers with an error, and
/// SIOCOUTQ's number (TIOCOUTQ) never reaches a descriptor that is not a TCP socket. A pipe is not a socket (ENOTSOCK
/// from getsockopt, net/socket.c:553-567); a UDP socket and Unix stream and datagram sockets are sockets that are not
/// TCP, which both functions refuse after reading SO_DOMAIN, SO_TYPE and SO_PROTOCOL (InvalidInput; MPTCP has its own
/// test below). TCP sockets still answer: a connected one reads, an unconnected one reads 0, and a listening one is
/// EINVAL from tcp_ioctl itself (siocoutq_on_a_listening_socket_is_einval: the check let it through). IPv6 is off in
/// the U3c container (socket(AF_INET6) is EAFNOSUPPORT), so the AF_INET6 branch is not exercised here.
#[test]
fn every_descriptor_that_is_not_a_tcp_socket_is_an_error() {
    let (pipe_r, pipe_w) = std::io::pipe().expect("pipe");
    for fd in [pipe_r.as_fd(), pipe_w.as_fd()] {
        let outq = unacked_and_unsent(fd).expect_err("a pipe is not a socket");
        assert_eq!(
            outq.raw_os_error(),
            Some(libc::ENOTSOCK),
            "SIOCOUTQ on a pipe: {outq}"
        );
        let rtt = srtt_us(fd).expect_err("a pipe is not a socket");
        assert_eq!(
            rtt.raw_os_error(),
            Some(libc::ENOTSOCK),
            "TCP_INFO on a pipe: {rtt}"
        );
    }
    let udp = UdpSocket::bind("127.0.0.1:0").expect("UDP");
    udp.connect(udp.local_addr().expect("address"))
        .expect("connect");
    udp.send(&[1; 100]).expect("send");
    let (unix_a, _unix_b) = UnixStream::pair().expect("Unix stream pair");
    let (unix_d, _unix_e) = UnixDatagram::pair().expect("Unix datagram pair");
    unix_d.send(&[1; 100]).expect("send");
    for (what, fd) in [
        ("UDP", udp.as_fd()),
        ("a Unix stream", unix_a.as_fd()),
        ("a Unix datagram", unix_d.as_fd()),
    ] {
        let outq = unacked_and_unsent(fd).expect_err(what);
        assert_eq!(
            outq.kind(),
            ErrorKind::InvalidInput,
            "SIOCOUTQ on {what}: {outq}"
        );
        let rtt = srtt_us(fd).expect_err(what);
        assert_eq!(
            rtt.kind(),
            ErrorKind::InvalidInput,
            "TCP_INFO on {what}: {rtt}"
        );
        eprintln!("{what}: SIOCOUTQ {outq}; TCP_INFO {rtt}");
    }
    let (_listener, mut s, mut c) = pair(None);
    exchange(&mut s, &mut c, 1);
    assert!(
        unacked_and_unsent(s.as_fd()).is_ok(),
        "a connected TCP socket reads"
    );
    let fresh = Socket::new(Domain::IPV4, Type::STREAM, None).expect("socket");
    assert_eq!(
        unacked_and_unsent(fresh.as_fd()).expect("an unconnected TCP socket reads"),
        0
    );
}

/// What a test that cannot run on this host reports (review C2R-9): one line, `SKIPPED <test>: <why>`, in one write to
/// `out`. libtest has no skip, so the test still passes; the caller hands it `io::stderr()` itself, whose writes get
/// past libtest's capture (it takes only the print macros' output, and shows that only for a failing test), so the
/// line reaches check.mjs's test step, which counts and names each one in its summary.
fn report_skip(out: &mut impl Write, test: &str, why: &str) {
    out.write_all(format!("SKIPPED {test}: {why}\n").as_bytes())
        .expect("write the skip line");
}

/// An MPTCP socket (SO_PROTOCOL IPPROTO_MPTCP, 262) is not TCP for either reading (review F8): Linux would answer its
/// SOL_TCP TCP_INFO from the first subflow (v6.8 net/mptcp/sockopt.c:1337, :1344), so srtt_us refuses it as
/// unacked_and_unsent does. A kernel without MPTCP (socket() refused, or net.mptcp.enabled = 0) reports a skip that
/// check.mjs's test step shows (mptcp_case); the U3c container has MPTCP.
#[test]
fn an_mptcp_socket_is_not_tcp_for_either_reading() {
    mptcp_case(
        || Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::MPTCP)),
        &mut std::io::stderr(),
    );
}

/// A kernel without MPTCP gives a skip line, not a silent pass (review C2R-9): mptcp_case with a socket() that fails
/// as such a kernel's does (ENOPROTOOPT when net.mptcp.enabled = 0) writes exactly one line, the one check.mjs counts.
#[test]
fn a_kernel_without_mptcp_reports_the_mptcp_case_skipped() {
    let mut out = Vec::new();
    let ran = mptcp_case(
        || Err(std::io::Error::from_raw_os_error(libc::ENOPROTOOPT)),
        &mut out,
    );
    assert!(!ran, "no socket, so the case did not run");
    let line = String::from_utf8(out).expect("UTF-8");
    let why = std::io::Error::from_raw_os_error(libc::ENOPROTOOPT);
    assert_eq!(
        line,
        format!(
            "SKIPPED an_mptcp_socket_is_not_tcp_for_either_reading: no MPTCP socket on this kernel ({why})\n"
        )
    );
}

/// The MPTCP case on the socket `make` gives: `false`, with a skip line written to `out`, when it gives none; else the
/// socket is connected to a loopback TCP listener (the connection falls back to plain TCP underneath, and the socket
/// stays MPTCP), the state in which TCP_INFO would answer, both readings must refuse it, and the result is `true`.
fn mptcp_case(make: impl FnOnce() -> std::io::Result<Socket>, out: &mut impl Write) -> bool {
    let mptcp = match make() {
        Ok(s) => s,
        Err(e) => {
            report_skip(
                out,
                "an_mptcp_socket_is_not_tcp_for_either_reading",
                &format!("no MPTCP socket on this kernel ({e})"),
            );
            return false;
        }
    };
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let to: SocketAddr = listener.local_addr().expect("listener address");
    mptcp.connect(&to.into()).expect("connect");
    let (_server, _) = listener.accept().expect("accept");
    for (what, err) in [
        (
            "TCP_INFO",
            srtt_us(mptcp.as_fd()).expect_err("MPTCP is not TCP"),
        ),
        (
            "SIOCOUTQ",
            unacked_and_unsent(mptcp.as_fd()).expect_err("MPTCP is not TCP"),
        ),
    ] {
        assert_eq!(
            err.kind(),
            ErrorKind::InvalidInput,
            "{what} on MPTCP: {err}"
        );
        assert!(
            err.to_string().contains("protocol 262"),
            "{what} on MPTCP: {err}"
        );
    }
    true
}

#[test]
fn tcpi_rtt_is_none_before_a_sample_and_some_after_traffic() {
    let fresh = Socket::new(Domain::IPV4, Type::STREAM, None).expect("socket");
    assert_eq!(
        srtt_us(fresh.as_fd()).expect("TCP_INFO on a new socket"),
        None,
        "a new socket's srtt_us is 0: no sample yet"
    );
    let (_listener, mut s, mut c) = pair(None);
    exchange(&mut s, &mut c, 20);
    let quick = (
        srtt_us(s.as_fd()).expect("TCP_INFO"),
        srtt_us(c.as_fd()).expect("TCP_INFO"),
    );
    // Loopback's RTT is about 2 us here (it read 1 us in one of 20 runs), and tcpi_rtt is in whole microseconds, so an
    // RTT under 1 us would read 0. Five samples each way through a peer in delayed-ACK mode (its ACK waits for the
    // delayed-ACK timer, tens of ms) put the srtt well above that floor, so the test does not depend on the CPU's speed.
    for _ in 0..5 {
        delayed_ack_sample(&mut s, &mut c);
        delayed_ack_sample(&mut c, &mut s);
    }
    let server = srtt_us(s.as_fd()).expect("TCP_INFO");
    let client = srtt_us(c.as_fd()).expect("TCP_INFO");
    eprintln!(
        "tcpi_rtt after 20 round trips: server {:?} us, client {:?} us; after 5 delayed-ACK samples each way: server \
         {server:?} us, client {client:?} us",
        quick.0, quick.1
    );
    assert!(server.is_some_and(|us| us > 0), "server {server:?}");
    assert!(client.is_some_and(|us| us > 0), "client {client:?}");
}

/// One RTT sample for `writer` that includes the peer's delayed-ACK wait: the reader leaves quick-ACK mode, the writer
/// sends 200 B, the reader does not read until the writer's SIOCOUTQ shows the ACK came, then reads.
fn delayed_ack_sample(writer: &mut TcpStream, reader: &mut TcpStream) {
    SockRef::from(&*reader)
        .set_tcp_quickack(false)
        .expect("TCP_QUICKACK");
    writer.write_all(&[9; 200]).expect("write");
    assert!(
        wait_until(Duration::from_secs(2), || outq(writer) == 0),
        "the delayed ACK came"
    );
    reader.read_exact(&mut [0; 200]).expect("read");
}

#[test]
fn tcpi_rtt_is_read_only_when_the_answer_covers_it() {
    let at = offset_of!(libc::tcp_info, tcpi_rtt);
    // Linux's uapi layout (include/uapi/linux/tcp.h): 8 bytes of u8 fields, then u32 tcpi_rto .. tcpi_rtt, the 16th.
    assert_eq!(
        at, 68,
        "libc's tcp_info places tcpi_rtt where the kernel writes it"
    );
    let _is_u32: fn(&libc::tcp_info) -> u32 = |i| i.tcpi_rtt;
    let mut info = [0u8; size_of::<libc::tcp_info>()];
    info[at..at + 4].copy_from_slice(&1_234u32.to_ne_bytes());
    assert_eq!(rtt_from(&info, at + 4).expect("covers"), Some(1_234));
    assert_eq!(rtt_from(&info, info.len()).expect("covers"), Some(1_234));
    for short in [0, at, at + 3] {
        let err = rtt_from(&info, short).expect_err("too short");
        assert_eq!(err.kind(), ErrorKind::Unsupported, "{short} bytes: {err}");
    }
    let err = rtt_from(&info[..at + 2], info.len()).expect_err("buffer too short");
    assert_eq!(err.kind(), ErrorKind::Unsupported);
    info[at..at + 4].fill(0);
    assert_eq!(
        rtt_from(&info, info.len()).expect("covers"),
        None,
        "0 is no sample"
    );
}

#[test]
fn the_stub_reports_unsupported() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let a = crate::stub::srtt_us(listener.as_fd()).expect_err("stub");
    let b = crate::stub::unacked_and_unsent(()).expect_err("stub");
    assert_eq!(a.kind(), ErrorKind::Unsupported);
    assert_eq!(b.kind(), ErrorKind::Unsupported);
}

/// The non-reading peer with a small SO_RCVBUF (u3c-plan.md 1.8 (e)). S writes 1,000 B chunks without blocking until
/// its send buffer is full. SIOCOUTQ grows; at a quiescent point it equals /proc's tx_queue, read between two
/// SIOCOUTQ readings that agree, and the bytes written less the peer's rx_queue (snd_una = the peer's rcv_nxt).
/// It cannot tell SIOCOUTQ from SIOCOUTQNSD: with the window closed nothing is in flight, so both read the same. The
/// delayed-ACK test below and the root run do.
#[test]
fn siocoutq_grows_with_a_non_reading_peer_and_equals_proc_tx_queue() {
    const CHUNK: usize = 1_000;
    const CAP: usize = 8 << 20;
    let (_listener, mut s, c) = pair(Some(4_096));
    let (sp, cp) = (port(&s), port(&c));
    s.set_nonblocking(true).expect("non-blocking");
    let mut written = 0usize;
    let mut readings = Vec::new();
    loop {
        match s.write(&[1; CHUNK]) {
            Ok(n) => written += n,
            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
            Err(e) => panic!("write: {e}"),
        }
        let q = outq(&s);
        assert!(q as usize <= written, "SIOCOUTQ {q} > {written} written");
        readings.push(q);
        assert!(written < CAP, "the send buffer never filled");
    }
    let mut last = None;
    let quiet = wait_until(Duration::from_secs(5), || {
        let q1 = outq(&s);
        let ((tx, _), (_, rx)) = proc_queues(sp, cp);
        let q2 = outq(&s);
        last = Some((q1, q2, tx, rx));
        q1 == q2 && q1 == tx && q1 as usize + rx as usize == written
    });
    let (q, _, tx, rx) = last.expect("one reading");
    eprintln!(
        "non-reading peer: {} writes, {written} B written; SIOCOUTQ first {} final {q}; tx_queue {tx}; peer rx_queue {rx}",
        readings.len(),
        readings[0]
    );
    assert!(
        quiet,
        "no quiescent reading: last (SIOCOUTQ, again, tx_queue, peer rx_queue) = {last:?}, written {written}"
    );
    assert!(
        q > readings[0],
        "SIOCOUTQ grows: first {} final {q}",
        readings[0]
    );
    drop(c);
}

/// The receiver in delayed-ACK mode (TCP_QUICKACK 0): right after a 200 B write the peer already holds the bytes (its
/// rx_queue is 200) but has not acknowledged them yet, so they are sent and unacked: notsent = write_seq - snd_nxt =
/// 0, which SIOCOUTQNSD would read, while SIOCOUTQ reads 200. Judged on identities: in every attempt whose two
/// SIOCOUTQ readings agree they equal tx_queue, and at least one attempt shows the state (a Python prototype on this
/// kernel caught it in 200 of 200).
#[test]
fn siocoutq_counts_sent_but_unacked_bytes() {
    const MSG: usize = 200;
    const ATTEMPTS: usize = 50;
    let (_listener, mut s, mut c) = pair(None);
    let (sp, cp) = (port(&s), port(&c));
    exchange(&mut s, &mut c, 20);
    assert!(
        wait_until(Duration::from_secs(2), || outq(&s) == 0),
        "warm-up acknowledged"
    );
    let (mut stable, mut caught) = (0, 0);
    let mut buf = [0u8; MSG];
    for _ in 0..ATTEMPTS {
        SockRef::from(&c)
            .set_tcp_quickack(false)
            .expect("TCP_QUICKACK");
        s.write_all(&[3; MSG]).expect("write");
        let q1 = outq(&s);
        let ((tx, _), (_, rx)) = proc_queues(sp, cp);
        let q2 = outq(&s);
        if q1 == q2 {
            stable += 1;
            assert_eq!(q1, tx, "SIOCOUTQ = tx_queue = write_seq - snd_una");
            assert!(
                q1 == 0 || q1 as usize == MSG,
                "one 200 B segment, acked whole: {q1}"
            );
            if q1 as usize == MSG && rx as usize == MSG {
                caught += 1;
            }
        }
        c.read_exact(&mut buf).expect("read");
        assert!(
            wait_until(Duration::from_secs(2), || outq(&s) == 0),
            "acknowledged after the read"
        );
    }
    eprintln!(
        "sent-but-unacked state caught in {caught} of {ATTEMPTS} attempts ({stable} with agreeing readings)"
    );
    assert!(
        caught >= 1,
        "never caught the sent-but-unacked state ({stable} stable readings)"
    );
}
