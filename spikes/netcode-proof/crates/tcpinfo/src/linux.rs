//! The two Linux calls. Each unsafe block holds one unsafe operation (the foreign call) and states the invariant that
//! makes it sound; everything around the calls is safe code.

use std::io;
use std::mem::offset_of;
use std::os::fd::{AsRawFd, BorrowedFd};

/// The ioctl request: Linux's SIOCOUTQ, which include/uapi/linux/sockios.h defines as TIOCOUTQ: 0x5411 on x86-64 and
/// aarch64 (include/uapi/asm-generic/ioctls.h), with other values on mips, powerpc and sparc, which libc carries. Not
/// SIOCOUTQNSD (0x894B).
pub(crate) const SIOCOUTQ: libc::Ioctl = libc::TIOCOUTQ;

/// The size of the int a SOL_SOCKET getsockopt answers with.
const INT_LEN: usize = size_of::<libc::c_int>();

/// Where `tcpi_rtt` sits in the bytes TCP_INFO writes, and the length the kernel's answer must reach to hold it.
const RTT_AT: usize = offset_of!(libc::tcp_info, tcpi_rtt);
const RTT_END: usize = RTT_AT + size_of::<u32>();

/// The socket's smoothed round-trip time in microseconds: `tcpi_rtt`, which Linux fills as `tp->srtt_us >> 3` (v6.8
/// net/ipv4/tcp.c:3842). `None` when it is 0, before the first RTT sample (a new socket's `srtt_us` is 0): fix 1's
/// gate then takes its 100 ms default.
///
/// The kernel writes into a zeroed buffer the size of `libc::tcp_info`; the field is read at its `offset_of!` from the
/// bytes written, so no further unsafe operation is needed.
///
/// Like [`unacked_and_unsent`], it answers for a TCP socket only: SO_DOMAIN AF_INET or AF_INET6, SO_TYPE SOCK_STREAM
/// and SO_PROTOCOL IPPROTO_TCP are read first, each with a bounded getsockopt that fails with ENOTSOCK on anything but
/// a socket. Not only for symmetry: an MPTCP socket (SO_PROTOCOL IPPROTO_MPTCP, 262) answers SOL_TCP's TCP_INFO from
/// its first subflow (v6.8 net/mptcp/sockopt.c:1337, :1344), not as the connection. Any descriptor is sound to pass.
///
/// **Cost** (review REG-3): four getsockopt system calls per call, where the TCP_INFO reading alone would be one: the
/// three SOL_SOCKET ones of the TCP check each copy one int, then TCP_INFO copies the `tcp_info` struct. Fix 1's
/// per-tick gate calls this once per connection, so the check costs three small system calls per connection per tick.
/// Nothing is cached: each call checks the descriptor it is given.
///
/// # Errors
/// ENOTSOCK on a descriptor that is not a socket; `ErrorKind::InvalidInput` on a socket that is not TCP (UDP, Unix,
/// MPTCP); the getsockopt error; `ErrorKind::Unsupported` when the kernel's answer is too short to hold `tcpi_rtt`.
#[allow(
    unsafe_code,
    reason = "getsockopt(SOL_SOCKET) and getsockopt(TCP_INFO) have no safe wrapper in std or in the pinned crates \
              (socket2 0.6.5, rustix 1.1.6, nix 0.31.3): foreign calls each writing into a buffer this function owns, \
              TCP_INFO only on a TCP socket (ADR 0002:264-265; R20 (a))"
)]
pub fn srtt_us(fd: BorrowedFd<'_>) -> io::Result<Option<u32>> {
    let int_option = |name: libc::c_int| -> io::Result<libc::c_int> {
        let mut value: libc::c_int = 0;
        let Ok(mut len) = libc::socklen_t::try_from(INT_LEN) else {
            return Err(io::Error::from(io::ErrorKind::Unsupported));
        };
        // SAFETY: `fd` is an open descriptor for the whole call (BorrowedFd's guarantee), of any kind: getsockopt
        // fails with ENOTSOCK before touching anything on a descriptor that is not a socket (v6.8
        // net/socket.c:2397-2399, sockfd_lookup_light :553-567). At level SOL_SOCKET every socket is answered by
        // sk_getsockopt (:2364-2365), which writes `min(len, sizeof(int))` bytes (net/core/sock.c:1609, :1999-2001)
        // and stores that count in `len`. `value` is a live, aligned, writable c_int owned by this closure and `len` a
        // live socklen_t holding its size, 4, so the kernel writes at most those 4 bytes; any 4 bytes are a valid c_int.
        let rc = unsafe {
            libc::getsockopt(
                fd.as_raw_fd(),
                libc::SOL_SOCKET,
                name,
                (&raw mut value).cast(),
                &raw mut len,
            )
        };
        int_answer(name, rc, len, value)
    };
    require_tcp(
        int_option(libc::SO_DOMAIN)?,
        int_option(libc::SO_TYPE)?,
        int_option(libc::SO_PROTOCOL)?,
        "TCP_INFO",
    )?;
    let mut info = [0u8; size_of::<libc::tcp_info>()];
    let Ok(mut len) = libc::socklen_t::try_from(info.len()) else {
        return Err(io::Error::from(io::ErrorKind::Unsupported));
    };
    // SAFETY: `fd` is an open descriptor for the whole call (BorrowedFd's guarantee), and the three readings above show
    // it is a TCP socket. Soundness does not rest on that: getsockopt is the socket layer's own call, which fails with
    // ENOTSOCK before touching anything on a descriptor that is not a socket (v6.8 net/socket.c:2397-2399,
    // sockfd_lookup_light :553-567), so no device driver ever sees it, and on a socket a level the family does not
    // handle is an error. `info` is a live, writable buffer of exactly `len` bytes owned by this function, and `len` is
    // a live socklen_t holding that size; getsockopt's contract is that the kernel writes at most `len` bytes there and
    // stores how many it wrote in `len` (TCP's TCP_INFO: `len = min(len, sizeof(info))`, net/ipv4/tcp.c:4067). Any
    // bytes are a valid `[u8]`, and u8 needs no alignment, so the buffer stays valid whatever the kernel writes.
    let rc = unsafe {
        libc::getsockopt(
            fd.as_raw_fd(),
            libc::IPPROTO_TCP,
            libc::TCP_INFO,
            info.as_mut_ptr().cast(),
            &raw mut len,
        )
    };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    rtt_from(&info, usize::try_from(len).unwrap_or(0))
}

/// The bytes written to the socket and not yet acknowledged by the peer, sent or not: ioctl SIOCOUTQ, which Linux
/// answers as `write_seq - snd_una` (v6.8 net/ipv4/tcp.c:630), 0 while the handshake runs (:627-628).
///
/// The request number 0x5411 is also TIOCOUTQ, and on a descriptor that is not a socket its meaning is the driver's,
/// so the ioctl is made only once the descriptor is shown to be a TCP socket: SO_DOMAIN AF_INET or AF_INET6, SO_TYPE
/// SOCK_STREAM and SO_PROTOCOL IPPROTO_TCP, each read with a bounded getsockopt that fails with ENOTSOCK on anything
/// but a socket. Any descriptor is therefore sound to pass.
///
/// # Errors
/// ENOTSOCK on a descriptor that is not a socket; `ErrorKind::InvalidInput` on a socket that is not TCP; the ioctl
/// error, EINVAL on a listening socket (tcp.c:624-625); `ErrorKind::InvalidData` if the kernel's int were negative.
#[allow(
    unsafe_code,
    reason = "ioctl(SIOCOUTQ) and getsockopt(SOL_SOCKET) have no safe wrapper in std or in the pinned crates (socket2 \
              0.6.5, rustix 1.1.6, nix 0.31.3): foreign calls each writing one int this function owns, the ioctl \
              only on a TCP socket (ADR 0002:264-265; R20 (a))"
)]
pub fn unacked_and_unsent(fd: BorrowedFd<'_>) -> io::Result<u32> {
    let int_option = |name: libc::c_int| -> io::Result<libc::c_int> {
        let mut value: libc::c_int = 0;
        let Ok(mut len) = libc::socklen_t::try_from(INT_LEN) else {
            return Err(io::Error::from(io::ErrorKind::Unsupported));
        };
        // SAFETY: `fd` is an open descriptor for the whole call (BorrowedFd's guarantee), of any kind: getsockopt
        // fails with ENOTSOCK before touching anything on a descriptor that is not a socket (v6.8
        // net/socket.c:2397-2399, sockfd_lookup_light :553-567). At level SOL_SOCKET every socket is answered by
        // sk_getsockopt (:2364-2365), which writes `min(len, sizeof(int))` bytes (net/core/sock.c:1609, :1999-2001)
        // and stores that count in `len`. `value` is a live, aligned, writable c_int owned by this closure and `len` a
        // live socklen_t holding its size, 4, so the kernel writes at most those 4 bytes; any 4 bytes are a valid c_int.
        let rc = unsafe {
            libc::getsockopt(
                fd.as_raw_fd(),
                libc::SOL_SOCKET,
                name,
                (&raw mut value).cast(),
                &raw mut len,
            )
        };
        int_answer(name, rc, len, value)
    };
    require_tcp(
        int_option(libc::SO_DOMAIN)?,
        int_option(libc::SO_TYPE)?,
        int_option(libc::SO_PROTOCOL)?,
        "SIOCOUTQ",
    )?;
    let mut answ: libc::c_int = 0;
    // SAFETY: `fd` is an open descriptor for the whole call (BorrowedFd's guarantee), and the three readings above
    // show it is an AF_INET or AF_INET6 socket of type SOCK_STREAM and protocol IPPROTO_TCP: a TCP socket (a socket's
    // family, type and protocol never change, but for IPV6_ADDRFORM, which turns an IPv6 TCP socket into an IPv4 TCP
    // one). On a socket the ioctl goes to sock_ioctl (fs/ioctl.c:869-871; net/socket.c:160, :1341), never to a device
    // driver; then sock_do_ioctl (:1222), inet_ioctl or inet6_ioctl (net/ipv4/af_inet.c:1003-1004,
    // net/ipv6/af_inet6.c:591), sk_ioctl and sock_ioctl_out (net/core/sock.c:4191-4199, :4207-4224): tcp_ioctl stores
    // one int (tcp.c:646, `*karg = answ`) and put_user copies exactly that int to the pointer. `answ` is a live,
    // aligned, writable c_int owned by this function, and the variadic argument is passed as a pointer to it.
    let rc = unsafe { libc::ioctl(fd.as_raw_fd(), SIOCOUTQ, &raw mut answ) };
    if rc < 0 {
        return Err(io::Error::last_os_error());
    }
    u32::try_from(answ)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("SIOCOUTQ gave {answ}")))
}

/// What a SOL_SOCKET getsockopt of one int answered, read right after the call: its error when `rc` is not 0, and
/// `ErrorKind::InvalidData` unless it wrote exactly one int.
fn int_answer(
    name: libc::c_int,
    rc: libc::c_int,
    len: libc::socklen_t,
    value: libc::c_int,
) -> io::Result<libc::c_int> {
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    if usize::try_from(len).ok() != Some(INT_LEN) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("getsockopt({name}) wrote {len} bytes, not {INT_LEN}"),
        ));
    }
    Ok(value)
}

/// `Ok` for a TCP socket only (SO_DOMAIN AF_INET or AF_INET6, SO_TYPE SOCK_STREAM, SO_PROTOCOL IPPROTO_TCP, so not
/// MPTCP's 262); otherwise `ErrorKind::InvalidInput`, naming the `call` that is not made.
fn require_tcp(
    domain: libc::c_int,
    kind: libc::c_int,
    protocol: libc::c_int,
    call: &str,
) -> io::Result<()> {
    if matches!(domain, libc::AF_INET | libc::AF_INET6)
        && kind == libc::SOCK_STREAM
        && protocol == libc::IPPROTO_TCP
    {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "not a TCP socket (domain {domain}, type {kind}, protocol {protocol}): {call} is not read"
        ),
    ))
}

/// `tcpi_rtt` from the bytes getsockopt wrote: `Unsupported` unless the `written` length covers the whole field
/// (a kernel or a sandbox with a shorter `tcp_info`); 0 maps to `None`.
pub(crate) fn rtt_from(info: &[u8], written: usize) -> io::Result<Option<u32>> {
    let field: Option<[u8; 4]> = if written >= RTT_END {
        info.get(RTT_AT..RTT_END).and_then(|b| b.try_into().ok())
    } else {
        None
    };
    let Some(bytes) = field else {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("TCP_INFO wrote {written} bytes; tcpi_rtt needs {RTT_END}"),
        ));
    };
    let rtt = u32::from_ne_bytes(bytes);
    Ok((rtt != 0).then_some(rtt))
}
