//! `rewind_at`, the integer half (u4-build-plan.md §4.9 :348-350; §4.2 :219-223): a command's view time to the
//! [`Rewind`] its shot sees the past through. The tolerance and the cap are U4h's.

use npt_sim::{Command, CoverVariant, HitHistory, Rewind};

/// Whether `history` holds every tick `r` reads: `tick_a`, and `tick_a + 1` (wrapping) when `fraction` is above 0
/// (seam.rs's rule for a Rewind the lent ring holds).
fn holds(history: &HitHistory, r: &Rewind) -> bool {
    history.get(r.tick_a).is_some()
        && (r.fraction == 0 || history.get(r.tick_a.wrapping_add(1)).is_some())
}

/// The [`Rewind`] for `command`'s claimed view time (`view_tick`, `view_fraction`: seam.rs) against the room's ring,
/// with the room's cover variant; `None` when the ring does not hold that view time (it lacks `view_tick`, or the
/// fraction is above 0 and it lacks `view_tick + 1`). By U4E-Q5 (a) the command then runs with no Rewind and the room
/// counts it (a refused Rewind runs as none in the sim, so this is one rule); U4h replaces it with the tolerance and
/// the cap. So it never names `tick_a + 1` beyond the newest tick pushed when `fraction > 0` (carry 22).
pub fn rewind_at(command: &Command, history: &HitHistory, cover: CoverVariant) -> Option<Rewind> {
    let r = Rewind {
        tick_a: command.view_tick,
        fraction: command.view_fraction,
        cover,
    };
    if !holds(history, &r) {
        return None;
    }
    // The sim refuses and counts a Rewind its ring does not hold (seam.rs); a correct room never sends one (§4.2).
    debug_assert!(
        holds(history, &r),
        "rewind_at names a tick the ring does not hold"
    );
    Some(r)
}
