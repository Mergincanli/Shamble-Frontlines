//! The press script (u7-brief.md rev 1.1 Block R3): what the browser client's soldier does, from its step count only
//! (the view tick it claims is the newest it holds): a slow circle, one fire press every 2 s, one reload press every
//! 20 s, no throw and no melee, aimed on a small circle round the map's centre. It fires and reloads honestly and never
//! throws (C-U6-19's line for U7: no hostile client).

use npt_client::wire::{Command, TICK_HZ};

/// One turn of the circle, a fire press and a reload press, each in steps.
const TURN_STEPS: u64 = 20 * TICK_HZ as u64;
const FIRE_EVERY: u64 = 2 * TICK_HZ as u64;
const RELOAD_EVERY: u64 = 20 * TICK_HZ as u64;
/// The circle's 16 directions at half speed (64 of 127): each x² + y² ≤ 127² (npt-room's kind 4).
const DIRS: [[i8; 2]; 16] = [
    [64, 0],
    [59, 24],
    [45, 45],
    [24, 59],
    [0, 64],
    [-24, 59],
    [-45, 45],
    [-59, 24],
    [-64, 0],
    [-59, -24],
    [-45, -45],
    [-24, -59],
    [0, -64],
    [24, -59],
    [45, -45],
    [59, -24],
];
/// The aim: on a circle of this radius round the map's centre (the stress scene's extent is ±12,800 cm: npt-room's
/// kind 6), at this height, cm.
const AIM_RADIUS_CM: i32 = 500;
const AIM_Z_CM: i32 = 100;

/// The command for the client's step `step`, claiming `view_tick` as its view (its slot, tick and sequence are the
/// endpoint's to stamp).
pub(crate) fn command(step: u64, view_tick: u32) -> Command {
    let d = DIRS[((step % TURN_STEPS) * DIRS.len() as u64 / TURN_STEPS) as usize];
    let mut presses = [0u8; 4];
    presses[Command::PRESS_FIRE] = (step / FIRE_EVERY) as u8;
    presses[Command::PRESS_RELOAD] = (step / RELOAD_EVERY) as u8;
    let aim = |v: i8| i32::from(v) * AIM_RADIUS_CM / 64;
    Command {
        move_xy: d,
        presses,
        aim: [aim(d[0]), aim(d[1]), AIM_Z_CM],
        view_tick,
        ..Command::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// R3: over 3,000 steps the script's commands keep the room's command rules: the move within 127², the aim inside
    /// the stress scene's ±12,800 cm, fire and reload counters each advancing by one at their own period, the throw and
    /// melee counters still, no held button; and at most one edge a step on each counter.
    #[test]
    fn r3_the_press_script_moves_slowly_fires_every_2_s_and_reloads_every_20_s() {
        let (mut fires, mut reloads) = (0u32, 0u32);
        let mut last = command(0, 0);
        for step in 0..3_000u64 {
            let c = command(step, 7);
            let (x, y) = (i32::from(c.move_xy[0]), i32::from(c.move_xy[1]));
            assert!(x * x + y * y <= 127 * 127, "step {step}");
            assert!(c.aim[..2].iter().all(|a| a.abs() <= 12_800), "step {step}");
            assert_eq!((c.buttons, c.view_tick), (0, 7));
            assert_eq!(c.presses[Command::PRESS_THROW], 0);
            assert_eq!(c.presses[Command::PRESS_MELEE], 0);
            for (i, n) in [
                (Command::PRESS_FIRE, &mut fires),
                (Command::PRESS_RELOAD, &mut reloads),
            ] {
                let edge = c.presses[i].wrapping_sub(last.presses[i]);
                assert!(edge <= 1, "step {step}: counter {i} moved {edge}");
                *n += u32::from(edge);
            }
            last = c;
        }
        assert_eq!((fires, reloads), (49, 4), "3,000 steps: 100 s at 30 Hz");
    }
}
