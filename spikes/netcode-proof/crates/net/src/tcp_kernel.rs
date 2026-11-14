//! The plain WebSocket path's two kernel readings for fix 1's send gate (u3c-plan.md 1.3, 1.8 and 1.10). P0 and ML
//! run without DelayStream, so WsServer (U6) reads the srtt and the outstanding bytes from the connection's own TCP
//! socket, the two quantities DelayStream's `srtt_ns()` and `outstanding_bytes()` give from the model on L2:
//!
//! - srtt: `tcpi_rtt` from TCP_INFO, which Linux fills as `tp->srtt_us >> 3` (v6.8 net/ipv4/tcp.c:3842): the smoothed
//!   RTT in whole microseconds;
//! - outstanding: SIOCOUTQ, which Linux answers as `write_seq - snd_una` (v6.8 net/ipv4/tcp.c:630): every byte
//!   written to the socket and not yet acknowledged, sent or not. SIOCOUTQNSD (`write_seq - snd_nxt`) would count
//!   only the unsent bytes and miss those in flight.
//!
//! Both come through npt-tcpinfo, the spike's one crate with Linux calls (R20 (a)); npt-net itself stays free of
//! unsafe code. Off Linux both readings are `ErrorKind::Unsupported`. What WsServer does with an error, and where the
//! per-tick calls run, are U6's (1.10).

use std::io;

use tokio::net::TcpStream;

/// The connection's smoothed RTT in nanoseconds, or `None` before its first RTT sample, where fix 1's gate takes its
/// 100 ms default (`snapshot_budget(None, ..)`). tcpi_rtt is in whole microseconds, so an RTT under 1 us also reads
/// `None` (loopback on the U3c container reads 2 us).
pub(crate) fn srtt_ns(sock: &TcpStream) -> io::Result<Option<u64>> {
    Ok(npt_tcpinfo::srtt_us(handle(sock))?.map(|us| u64::from(us) * 1_000))
}

/// The bytes written to the connection's socket and not yet acknowledged, sent or not (SIOCOUTQ): fix 1's
/// "outstanding" on the plain path.
pub(crate) fn outstanding_bytes(sock: &TcpStream) -> io::Result<u64> {
    npt_tcpinfo::unacked_and_unsent(handle(sock)).map(u64::from)
}

/// The socket's descriptor, borrowed for one call. Off Unix npt-tcpinfo's stub takes any handle.
#[cfg(unix)]
fn handle(sock: &TcpStream) -> std::os::fd::BorrowedFd<'_> {
    use std::os::fd::AsFd;
    sock.as_fd()
}

#[cfg(not(unix))]
fn handle(sock: &TcpStream) -> &TcpStream {
    sock
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::os::fd::AsFd;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpSocket};

    use super::{outstanding_bytes, srtt_ns};

    /// On a tokio TcpStream, as WsServer holds it: the readings are npt-tcpinfo's, in nanoseconds and u64. Each pair is
    /// compared between two agreeing readings of npt-tcpinfo's own value, so an ACK arriving in between cannot make
    /// them differ; the comparisons that ran are counted and a minimum is asserted, so none can be skipped silently:
    /// the srtt once (retried until two readings agree), the outstanding bytes at least once while the client still
    /// takes data and once at the quiescent point after the writes stop.
    #[tokio::test]
    async fn the_plain_path_reads_the_kernel_through_npt_tcpinfo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let to = listener.local_addr().expect("address");
        // A small receive buffer, so the window closes soon once the client stops reading.
        let socket = TcpSocket::new_v4().expect("socket");
        socket.set_recv_buffer_size(4_096).expect("SO_RCVBUF");
        let mut client = socket.connect(to).await.expect("connect");
        let (mut server, _) = listener.accept().await.expect("accept");
        server.set_nodelay(true).expect("TCP_NODELAY");
        let mut buf = [0u8; 100];
        for _ in 0..20 {
            server.write_all(&[7; 100]).await.expect("write");
            client.read_exact(&mut buf).await.expect("read");
            client.write_all(&[8; 100]).await.expect("write");
            server.read_exact(&mut buf).await.expect("read");
        }
        // Loopback's RTT is a few us, near tcpi_rtt's 1 us floor: three samples through the client's delayed ACK (tens
        // of ms) lift the srtt clear of it.
        for _ in 0..3 {
            client.set_quickack(false).expect("TCP_QUICKACK");
            server.write_all(&[9; 200]).await.expect("write");
            let mut acked = false;
            for _ in 0..2_000 {
                if outstanding_bytes(&server).expect("outstanding_bytes") == 0 {
                    acked = true;
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
            assert!(acked, "the delayed ACK came");
            client.read_exact(&mut [0; 200]).await.expect("read");
        }
        // The connection is idle now (every byte acknowledged), so the srtt cannot move between the reads; the retry
        // only guards against a late ACK.
        let mut srtt_compared = 0;
        let mut ns = None;
        for _ in 0..100 {
            let fd = server.as_fd();
            let before = npt_tcpinfo::srtt_us(fd).expect("tcpi_rtt");
            let got = srtt_ns(&server).expect("srtt_ns");
            let after = npt_tcpinfo::srtt_us(fd).expect("tcpi_rtt");
            ns = got;
            if before == after {
                assert_eq!(got, before.map(|us| u64::from(us) * 1_000));
                srtt_compared += 1;
                break;
            }
        }
        assert_eq!(
            srtt_compared, 1,
            "srtt_ns compared with npt-tcpinfo's srtt_us"
        );
        assert!(
            ns.is_some_and(|ns| ns > 0),
            "an RTT sample after traffic: {ns:?}"
        );

        // A client that stops reading: the server's writes stay outstanding and grow.
        let compare = |server: &tokio::net::TcpStream| -> Option<u64> {
            let fd = server.as_fd();
            let a = npt_tcpinfo::unacked_and_unsent(fd).expect("SIOCOUTQ");
            let q = outstanding_bytes(server).expect("outstanding_bytes");
            let b = npt_tcpinfo::unacked_and_unsent(fd).expect("SIOCOUTQ");
            (a == b).then(|| {
                assert_eq!(q, u64::from(a));
                q
            })
        };
        let mut written = 0u64;
        let mut grew = false;
        let mut last = 0u64;
        let mut writes = 0;
        let mut compared = 0;
        for _ in 0..1_000 {
            match server.try_write(&[1; 1_000]) {
                Ok(n) => written += n as u64,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("write: {e}"),
            }
            writes += 1;
            compared += usize::from(compare(&server).is_some());
            let q = outstanding_bytes(&server).expect("outstanding_bytes");
            assert!(q <= written, "outstanding {q} > {written} written");
            grew |= q > last && last > 0;
            last = q;
            tokio::task::yield_now().await;
        }
        // The quiescent point: the window is closed or the writes stopped, so no ACK moves snd_una any more and two
        // readings agree; retried a bounded number of times.
        let mut quiet = None;
        for _ in 0..200 {
            quiet = compare(&server);
            if quiet.is_some() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
        eprintln!(
            "outstanding_bytes compared in {compared} of {writes} writes, then at the quiescent point: {quiet:?} of \
             {written} B written"
        );
        assert!(
            compared >= 1,
            "no write's readings agreed: {compared} of {writes}"
        );
        assert!(
            quiet.is_some_and(|q| q > 0 && q <= written),
            "the quiescent comparison ran: {quiet:?}"
        );
        assert!(
            grew && last > 0,
            "outstanding grew while the client did not read: last {last}"
        );
        drop(client);
    }
}
