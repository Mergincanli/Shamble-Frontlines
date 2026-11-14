//! The one trajectory function (plan E1: "300 live projectiles as trajectory records through one shared function";
//! ADR 0001 part 6): a projectile is its record (start tick, start position, velocity), and where it is at any tick is
//! this closed form, the same on the server, the client and both targets (PM-0004-73).

use crate::TICK_HZ;

/// Gravity, m/s².
pub(crate) const GRAVITY: f32 = 9.81;

/// Where a projectile started at `start` (metres) with `velocity` (m/s) is `ticks` ticks later: the ballistic arc
/// under gravity, stopped where it meets the ground (z = 0), where it stays. Closed form, `+ - * /` and `sqrt` only (ADR
/// 0004's float allow-list), so it needs no step and no transcendental; a start below the ground is taken as on it.
pub fn trajectory_at(start: [f32; 3], velocity: [f32; 3], ticks: u32) -> [f32; 3] {
    let t = ticks as f32 / TICK_HZ as f32;
    let z0 = if start[2] > 0.0 { start[2] } else { 0.0 };
    let vz = velocity[2];
    // The time it lands: the larger root of z0 + vz t - g t² / 2 = 0.
    let land = (vz + (vz * vz + 2.0 * GRAVITY * z0).sqrt()) / GRAVITY;
    let t = if t < land { t } else { land };
    let z = z0 + vz * t - 0.5 * GRAVITY * t * t;
    [
        start[0] + velocity[0] * t,
        start[1] + velocity[1] * t,
        if z > 0.0 { z } else { 0.0 },
    ]
}
