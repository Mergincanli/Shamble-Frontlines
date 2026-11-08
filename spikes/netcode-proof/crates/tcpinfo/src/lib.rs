//! npt-tcpinfo: two readings of a Linux TCP socket's send side, for npt-net's WebSocket server on the plain path (P0
//! and ML run without DelayStream), where fix 1's send gate reads the kernel instead of the TCP model (u3c-plan.md 1.3
//! and 1.8; R20 (a)).
//!
//! - [`srtt_us`]: the smoothed round-trip time, `tcpi_rtt` from getsockopt(IPPROTO_TCP, TCP_INFO), which Linux fills
//!   as `tp->srtt_us >> 3` (v6.8 net/ipv4/tcp.c:3842).
//! - [`unacked_and_unsent`]: ioctl SIOCOUTQ, `write_seq - snd_una` (tcp.c:630): every byte written to the socket and
//!   not yet acknowledged, sent or not. Not SIOCOUTQNSD (`write_seq - snd_nxt`, tcp.c:632-640), which counts only the
//!   bytes not yet sent; /proc/net/tcp's tx_queue is the same `write_seq - snd_una` (tcp_ipv4.c:2899).
//!
//! **The spike's second `unsafe` exception** (the first is npt-client-wasm's), granted by the advisor under ADR
//! 0002:264-265's rule: "an exception needs a written invariant, a test and an `#[allow]` with its reason". The
//! `#[allow(unsafe_code, reason = ...)]` sits directly on the two functions in linux.rs and nowhere else in the
//! crate. Each unsafe block holds one unsafe operation and a SAFETY comment stating its invariant (the two clippy
//! lints below are errors). The `tcp_info` field is read from the bytes the kernel wrote at `offset_of!`, with no
//! further unsafe code. check.mjs's deps step checks that libc is the only shipped dependency, that only npt-net
//! depends on the crate, that each of the two attributes sits on its function and no other relaxing attribute exists
//! here, that every `unsafe` in the crate lies inside those two functions, that the two clippy lints below stay denied
//! here and relaxed nowhere, and that no crate but npt-client-wasm and this one has `unsafe` code.
//!
//! **Any descriptor is sound to pass.** Both functions first read SO_DOMAIN, SO_TYPE and SO_PROTOCOL (ENOTSOCK on a
//! descriptor that is not a socket) and make their call only on a TCP socket; any other answers with an error.
//! SIOCOUTQ shares its number with TIOCOUTQ, whose meaning on a descriptor that is not a socket is the driver's, so
//! for [`unacked_and_unsent`] the check is what makes the ioctl sound. For [`srtt_us`], whose getsockopt is the socket
//! layer's own call, it keeps an MPTCP socket out: that one answers TCP_INFO from its first subflow (v6.8
//! net/mptcp/sockopt.c:1337, :1344). npt-net passes a `tokio::net::TcpStream`'s descriptor.
//!
//! **Linux only.** On every other target the two functions are stubs that return `ErrorKind::Unsupported`, so the
//! workspace still builds there: check.mjs's clippy-wasm32 step lints this crate for wasm32, a target that is not
//! Linux.

#![deny(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block
)]

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{srtt_us, unacked_and_unsent};

// The stub is the other targets' implementation; on Linux it is compiled for its own test only.
#[cfg(any(not(target_os = "linux"), test))]
mod stub;
#[cfg(not(target_os = "linux"))]
pub use stub::{srtt_us, unacked_and_unsent};

#[cfg(all(test, target_os = "linux"))]
mod tests;
