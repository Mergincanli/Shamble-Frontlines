//! Every target but Linux: the same two names, which report `ErrorKind::Unsupported` and nothing else. They take any
//! handle (a `BorrowedFd` on another Unix, a `BorrowedSocket` on Windows, anything on a target with neither), so a
//! caller compiles everywhere. No unsafe code here.

use std::io;

/// The stub of the Linux `srtt_us`: always `ErrorKind::Unsupported`.
///
/// # Errors
/// Always `ErrorKind::Unsupported`.
pub fn srtt_us<H>(_fd: H) -> io::Result<Option<u32>> {
    Err(unsupported())
}

/// The stub of the Linux `unacked_and_unsent`: always `ErrorKind::Unsupported`.
///
/// # Errors
/// Always `ErrorKind::Unsupported`.
pub fn unacked_and_unsent<H>(_fd: H) -> io::Result<u32> {
    Err(unsupported())
}

fn unsupported() -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        "npt-tcpinfo reads Linux TCP sockets only",
    )
}
