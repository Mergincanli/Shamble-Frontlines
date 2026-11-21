//! The tick constant and every tick-counted value derived from it (u4-build-plan.md §4.3; plan §4.6; RQ6).
//!
//! `TICK_HZ` is the one build-time tick constant. Every tick-counted value is a const fn of a tick rate, and each
//! constant below is its own function at `TICK_HZ`, written `<name>_at(TICK_HZ)` and never a literal: the tests pin
//! every function at 30 and at 60 Hz, a const assert holds each constant equal to its function at whatever `TICK_HZ`
//! is (so the 60 Hz run's build refuses to compile a constant that drifted from its function), and check.mjs's deps
//! step refuses these names set in any other form. The `_at` forms take the rate as an argument, for those tests and for
//! the 60 Hz run's change-set; everything else uses the constants.

/// The tick rate in ticks per second: the one tick constant (plan §4.6). The 60 Hz run's own change-set replaces
/// npt-sim through the seam and changes no other crate. A const assert below holds it in 1 to 1,000.
pub const TICK_HZ: u32 = 30;

/// The highest tick rate the build may set: at it, `ack_ring_ticks_at`'s `32 × tick_hz` is 32,000, far inside u32, and
/// `ticks_for_ms_at` works in u64, so no derived constant overflows.
const MAX_TICK_HZ: u32 = 1_000;

const _: () = assert!(
    TICK_HZ >= 1 && TICK_HZ <= MAX_TICK_HZ,
    "TICK_HZ is 1 to 1,000 ticks per second"
);

/// The ticks that cover `ms` milliseconds at `tick_hz`, rounded up: ⌈ms × tick_hz / 1,000⌉ (u4-build-plan.md §4.3).
/// Worked in u64 and saturated at `u32::MAX`, so it never overflows.
pub const fn ticks_for_ms_at(tick_hz: u32, ms: u32) -> u32 {
    let ticks = (ms as u64 * tick_hz as u64).div_ceil(1_000);
    if ticks > u32::MAX as u64 {
        u32::MAX
    } else {
        ticks as u32
    }
}

/// [`ticks_for_ms_at`] at [`TICK_HZ`]: how every millisecond bound becomes a tick count.
pub const fn ticks_for_ms(ms: u32) -> u32 {
    ticks_for_ms_at(TICK_HZ, ms)
}

/// The loop's catch-up cap at `tick_hz`: at most ⌊0.25 × tick_hz⌋ ticks run in one wake (plan §4.6: 7 at 30 Hz, 15
/// at 60 Hz; ADR 0004 part 5's S17). The other cap, commands per client per tick, is the redundancy cap and is not
/// derived from the tick rate (`CMD_CATCHUP`, npt-room).
pub const fn loop_catchup_at(tick_hz: u32) -> u32 {
    tick_hz / 4
}

/// [`loop_catchup_at`] at [`TICK_HZ`]: what npt-server's `Schedule` and the Worker's host pass as the catch-up cap.
pub const LOOP_CATCHUP: u32 = loop_catchup_at(TICK_HZ);

/// The FullState acknowledgement bound at `tick_hz`: a forced resync not acknowledged within 2 s ends in
/// Kick(overflow) (u4-build-plan.md §4.7): [`ticks_for_ms_at`] of 2,000 (60 ticks at 30 Hz, 120 at 60 Hz).
pub const fn full_state_ack_ticks_at(tick_hz: u32) -> u32 {
    ticks_for_ms_at(tick_hz, 2_000)
}

/// [`full_state_ack_ticks_at`] at [`TICK_HZ`], [`ticks_for_ms`] of 2,000.
pub const FULL_STATE_ACK_TICKS: u32 = full_state_ack_ticks_at(TICK_HZ);

/// The length in ticks of the room's AckRing and of the client's ring of complete frames at `tick_hz`: 32 × tick_hz
/// / 30, so the ring keeps ages 0 to 31 at 30 Hz, about 1.067 s (u4-build-plan.md §4.5; ADR 0001 part 6). At most
/// 1,066 within `TICK_HZ`'s asserted range.
///
/// # Panics
/// Above 134,217,727 ticks per second `32 × tick_hz` overflows u32 (a panic where overflow checks are on, a compile
/// error in a const); `TICK_HZ`'s const assert keeps the build's own rate far below that.
pub const fn ack_ring_ticks_at(tick_hz: u32) -> u32 {
    32 * tick_hz / 30
}

/// [`ack_ring_ticks_at`] at [`TICK_HZ`].
pub const ACK_RING_TICKS: u32 = ack_ring_ticks_at(TICK_HZ);

/// ClientSettings' step at `tick_hz`: the interpolation delay moves by at most one interval per second, so one step
/// every `tick_hz` ticks (u4-build-plan.md §4.3).
pub const fn settings_step_ticks_at(tick_hz: u32) -> u32 {
    tick_hz
}

/// [`settings_step_ticks_at`] at [`TICK_HZ`].
pub const SETTINGS_STEP_TICKS: u32 = settings_step_ticks_at(TICK_HZ);

// Each derived constant is its own function at TICK_HZ, at every TICK_HZ: written so, check.mjs refuses any other
// form, and these hold it should the form be evaded (the 60 Hz build then fails here, naming the constant).
const _: () = assert!(
    LOOP_CATCHUP == loop_catchup_at(TICK_HZ),
    "LOOP_CATCHUP is loop_catchup_at(TICK_HZ)"
);
const _: () = assert!(
    FULL_STATE_ACK_TICKS == full_state_ack_ticks_at(TICK_HZ),
    "FULL_STATE_ACK_TICKS is full_state_ack_ticks_at(TICK_HZ)"
);
const _: () = assert!(
    ACK_RING_TICKS == ack_ring_ticks_at(TICK_HZ),
    "ACK_RING_TICKS is ack_ring_ticks_at(TICK_HZ)"
);
const _: () = assert!(
    SETTINGS_STEP_TICKS == settings_step_ticks_at(TICK_HZ),
    "SETTINGS_STEP_TICKS is settings_step_ticks_at(TICK_HZ)"
);
