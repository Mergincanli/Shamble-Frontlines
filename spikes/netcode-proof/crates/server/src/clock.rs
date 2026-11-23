//! The one reader of the monotonic clock (U6a; server/clippy.toml's clock ban and its reviewed per-item allow): every
//! other item in npt-server takes the time as an argument (`now_ns`), so the loop policy, the pump and the tick loop's
//! core run on an injected clock in their tests and on npt-lab's simulated clock on L1.

use std::sync::OnceLock;

/// The instant the process first read the clock: the zero of [`now_ns`].
#[allow(
    clippy::disallowed_types,
    reason = "U6a's one clock item (server/clippy.toml asks for a reviewed per-item allow): the monotonic clock's zero"
)]
static ORIGIN: OnceLock<std::time::Instant> = OnceLock::new();

/// Monotonic nanoseconds since this process's first read (saturating at `u64::MAX`, about 584 years).
#[allow(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "U6a's one clock item (server/clippy.toml asks for a reviewed per-item allow): TickLoop's thread, the I/O \
              runtime's sessions and the per-second reads take their time from here, and from nowhere else"
)]
pub(crate) fn now_ns() -> u64 {
    let origin = ORIGIN.get_or_init(std::time::Instant::now);
    u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

/// Reads [`resolution_ns`] makes at most, so a clock that never moves cannot hold the start.
const RESOLUTION_READS: u32 = 10_000_000;

/// The clock's resolution as this process sees it (E18: "clock resolution per machine"; the header's
/// `machine.clock_res_ns`): the smallest non-zero step between two consecutive reads of [`now_ns`] over `samples`
/// steps (at most [`RESOLUTION_READS`] reads); 1 when no step is seen, so the header never says 0.
pub(crate) fn resolution_ns(samples: u32) -> u64 {
    let mut least = u64::MAX;
    let mut last = now_ns();
    let mut seen = 0;
    for _ in 0..RESOLUTION_READS {
        if seen >= samples {
            break;
        }
        let now = now_ns();
        if now != last {
            least = least.min(now - last);
            last = now;
            seen += 1;
        }
    }
    if least == u64::MAX { 1 } else { least }
}
