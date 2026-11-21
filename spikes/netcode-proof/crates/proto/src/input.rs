//! `Input`: a client's commands, the newest whole and each older one delta-coded against the newer one beside it
//! (ADR 0001 part 5: option U2), with the newest snapshot the client decoded (u4-build-plan.md §4.4, §4.5).
//!
//! **The field table** (u4-build-plan.md §4.4's "stated by U4a and read by the limit test"; net F116's proposal), in
//! bits, least significant bit first, each command starting at a byte and ending with zero pad bits:
//!
//! | Part | Fields | Bits |
//! |---|---|---|
//! | header | type 8, the acknowledged tick's low 16 bits 16, its group mask 8, command count 8 | 40 (5 B) |
//! | whole command (the newest) | tick 32, sequence 16, buttons 16, press counters 4 × 8, move 2 × 8, aim (`POSITION_XY`, `POSITION_XY`, `POSITION_Z`: 16 + 16 + 12), view offset 16, view fraction 8 | 180 (23 B) |
//! | repeat (each older one) | change mask 8, tick step 8, then the fields its mask names: buttons 16, press counters 32, move 16, aim 44, view offset and fraction 24 | at most 148 (19 B) |
//!
//! At the redundancy cap: 5 + 23 + 7 × 19 = 161 B, inside `Input`'s 166 B; a repeat is never larger than a whole
//! command (19 B against 23 B); the aim is 44 bits, under ADR 0001:226-227's 80. Every width is read from the grid's
//! fields (the aim's) or the `Command` field's own type.
//!
//! - **The view time** goes as its offset back from the command's own tick (`tick - view_tick`, at most 65,535) and
//!   its fraction; a repeat's mask names it when either differs from the newer command's.
//! - **A repeat's tick** is the newer command's tick less its step (0 to 255: ticks may repeat or skip under
//!   OQ-0001-6 (b)'s re-align); **its sequence** is the newer command's less one: the unacknowledged commands are
//!   consecutive, so it is not sent.
//! - **The mask:** bit 0 buttons, 1 the press counters, 2 the move, 3 the aim, 4 the view time; bits 5 to 7 are
//!   reserved and must be zero (an unknown field: ADR 0001:238).
//! - **The acknowledgement:** a group mask of 0 means the client has decoded no snapshot yet, and its tick is then 0:
//!   one encoding, so a tick with no group is an unknown field (as `Snapshot`'s flags rule), and the encoder refuses
//!   to write one.
//! - **`Command::slot`** is not on the wire: the session gives it. The decoder writes 0, and the room sets it.
//!
//! Values are checked here for what the wire can carry, not for what the room accepts: the move vector's length, the
//! aim's map bounds, the ticks and sequences are the room's checks (catalogue §2, `Input`'s row).

use npt_sim::Command;
use npt_sim::grid::{Field, POSITION_XY, POSITION_Z};

use crate::bits::{BitReader, BitWriter};
use crate::error::{DecodeError, EncodeError};
use crate::limits::Limits;
use crate::quant::Quant;

/// `Input`: the client's newest commands and the newest snapshot it decoded (the module text).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Input {
    /// The low 16 bits of the newest snapshot tick the client decoded a group of; 0 when it has decoded none.
    pub ack_tick: u16,
    /// Which groups of that tick it decoded, one bit per group index; 0 when it has decoded none.
    pub ack_groups: u8,
    /// The newest command first, then every unacknowledged one before it, newest to oldest, at most
    /// [`Limits::REDUNDANCY_CAP`]. Each older one's sequence is the one before its newer neighbour's.
    pub commands: Vec<Command>,
}

/// The aim point's three fields.
const AIM: [Field; 3] = [POSITION_XY, POSITION_XY, POSITION_Z];
const AIM_BITS: u32 = AIM[0].bits() + AIM[1].bits() + AIM[2].bits();
/// Bits of each `Command` field on the wire: from its own type, the press counters 4 of them.
const TICK_BITS: u32 = u32::BITS;
const SEQ_BITS: u32 = u16::BITS;
const BUTTONS_BITS: u32 = u16::BITS;
const PRESSES_BITS: u32 = 4 * u8::BITS;
const MOVE_BITS: u32 = 2 * i8::BITS;
const VIEW_BITS: u32 = u16::BITS + u8::BITS;
const MASK_BITS: u32 = u8::BITS;
const STEP_BITS: u32 = u8::BITS;

/// The header's bytes: type, acknowledged tick, group mask, count.
pub(crate) const HEADER_BYTES: usize = 5;
/// A whole command's bytes.
pub(crate) const WHOLE_BYTES: usize =
    ((TICK_BITS + SEQ_BITS + BUTTONS_BITS + PRESSES_BITS + MOVE_BITS + AIM_BITS + VIEW_BITS)
        as usize)
        .div_ceil(8);
/// The largest repeat's bytes: every field changed.
pub(crate) const REPEAT_BYTES: usize =
    ((MASK_BITS + STEP_BITS + BUTTONS_BITS + PRESSES_BITS + MOVE_BITS + AIM_BITS + VIEW_BITS)
        as usize)
        .div_ceil(8);

// The catalogue's rule: a repeat is never larger than a whole command, and a full datagram fits Input's limit.
const _: () = assert!(REPEAT_BYTES <= WHOLE_BYTES);
const _: () = assert!(
    HEADER_BYTES + WHOLE_BYTES + (Limits::REDUNDANCY_CAP - 1) * REPEAT_BYTES <= Limits::INPUT
);

const BUTTONS: u8 = 1;
const PRESSES: u8 = 1 << 1;
const MOVE: u8 = 1 << 2;
const AIM_CHANGED: u8 = 1 << 3;
const VIEW: u8 = 1 << 4;
const KNOWN: u8 = BUTTONS | PRESSES | MOVE | AIM_CHANGED | VIEW;

/// A command's view time as the wire carries it: the offset back from its own tick, and the fraction.
fn view(c: &Command) -> Result<(u16, u8), EncodeError> {
    let offset =
        u16::try_from(c.tick.wrapping_sub(c.view_tick)).map_err(|_| EncodeError::OutOfRange)?;
    Ok((offset, c.view_fraction))
}

fn put_presses(w: &mut BitWriter<'_>, p: [u8; 4]) {
    for x in p {
        w.put(u64::from(x), 8);
    }
}

fn put_move(w: &mut BitWriter<'_>, m: [i8; 2]) {
    for x in m {
        w.put(u64::from(x as u8), 8);
    }
}

fn put_aim(q: Quant, w: &mut BitWriter<'_>, aim: [i32; 3]) -> Result<(), EncodeError> {
    for (f, k) in AIM.into_iter().zip(aim) {
        q.put(w, f, k)?;
    }
    Ok(())
}

fn get_presses(r: &mut BitReader<'_>) -> Result<[u8; 4], DecodeError> {
    Ok([r.u8()?, r.u8()?, r.u8()?, r.u8()?])
}

fn get_move(r: &mut BitReader<'_>) -> Result<[i8; 2], DecodeError> {
    Ok([r.u8()? as i8, r.u8()? as i8])
}

fn get_aim(q: Quant, r: &mut BitReader<'_>) -> Result<[i32; 3], DecodeError> {
    Ok([q.get(r, AIM[0])?, q.get(r, AIM[1])?, q.get(r, AIM[2])?])
}

impl Input {
    pub(crate) fn put(&self, q: Quant, w: &mut BitWriter<'_>) -> Result<(), EncodeError> {
        if self.commands.len() > Limits::REDUNDANCY_CAP {
            return Err(EncodeError::TooMany);
        }
        if self.ack_groups == 0 && self.ack_tick != 0 {
            return Err(EncodeError::NotCanonical);
        }
        w.put(u64::from(self.ack_tick), 16);
        w.put(u64::from(self.ack_groups), 8);
        w.put(self.commands.len() as u64, 8);
        let mut newer: Option<&Command> = None;
        for c in &self.commands {
            let (offset, fraction) = view(c)?;
            match newer {
                None => {
                    w.put(u64::from(c.tick), TICK_BITS);
                    w.put(u64::from(c.seq), SEQ_BITS);
                    w.put(u64::from(c.buttons), BUTTONS_BITS);
                    put_presses(w, c.presses);
                    put_move(w, c.move_xy);
                    put_aim(q, w, c.aim)?;
                    w.put(u64::from(offset), 16);
                    w.put(u64::from(fraction), 8);
                }
                Some(n) => {
                    if c.seq != n.seq.wrapping_sub(1) {
                        return Err(EncodeError::NotCanonical);
                    }
                    let step = u8::try_from(n.tick.wrapping_sub(c.tick))
                        .map_err(|_| EncodeError::OutOfRange)?;
                    let mut mask = 0u8;
                    if c.buttons != n.buttons {
                        mask |= BUTTONS;
                    }
                    if c.presses != n.presses {
                        mask |= PRESSES;
                    }
                    if c.move_xy != n.move_xy {
                        mask |= MOVE;
                    }
                    if c.aim != n.aim {
                        mask |= AIM_CHANGED;
                    }
                    if (offset, fraction) != view(n)? {
                        mask |= VIEW;
                    }
                    w.put(u64::from(mask), MASK_BITS);
                    w.put(u64::from(step), STEP_BITS);
                    if mask & BUTTONS != 0 {
                        w.put(u64::from(c.buttons), BUTTONS_BITS);
                    }
                    if mask & PRESSES != 0 {
                        put_presses(w, c.presses);
                    }
                    if mask & MOVE != 0 {
                        put_move(w, c.move_xy);
                    }
                    if mask & AIM_CHANGED != 0 {
                        put_aim(q, w, c.aim)?;
                    }
                    if mask & VIEW != 0 {
                        w.put(u64::from(offset), 16);
                        w.put(u64::from(fraction), 8);
                    }
                }
            }
            w.align();
            newer = Some(c);
        }
        Ok(())
    }

    pub(crate) fn get(q: Quant, r: &mut BitReader<'_>) -> Result<Input, DecodeError> {
        let ack_tick = r.u16()?;
        let ack_groups = r.u8()?;
        if ack_groups == 0 && ack_tick != 0 && q.strict() {
            return Err(DecodeError::UnknownField);
        }
        let count = usize::from(r.u8()?);
        if count > Limits::REDUNDANCY_CAP {
            return Err(DecodeError::OutOfRange);
        }
        let mut commands: Vec<Command> = Vec::with_capacity(count);
        for _ in 0..count {
            let c = match commands.last() {
                None => {
                    let tick = r.u32()?;
                    let seq = r.u16()?;
                    let buttons = r.u16()?;
                    let presses = get_presses(r)?;
                    let move_xy = get_move(r)?;
                    let aim = get_aim(q, r)?;
                    let offset = r.u16()?;
                    let fraction = r.u8()?;
                    Command {
                        slot: 0,
                        tick,
                        seq,
                        buttons,
                        presses,
                        move_xy,
                        aim,
                        view_tick: tick.wrapping_sub(u32::from(offset)),
                        view_fraction: fraction,
                    }
                }
                Some(n) => {
                    let mask = r.u8()?;
                    if mask & !KNOWN != 0 && q.strict() {
                        return Err(DecodeError::UnknownField);
                    }
                    let step = r.u8()?;
                    let tick = n.tick.wrapping_sub(u32::from(step));
                    let (n_offset, n_fraction) =
                        (n.tick.wrapping_sub(n.view_tick), n.view_fraction);
                    let buttons = if mask & BUTTONS != 0 {
                        r.u16()?
                    } else {
                        n.buttons
                    };
                    let presses = if mask & PRESSES != 0 {
                        get_presses(r)?
                    } else {
                        n.presses
                    };
                    let move_xy = if mask & MOVE != 0 {
                        get_move(r)?
                    } else {
                        n.move_xy
                    };
                    let aim = if mask & AIM_CHANGED != 0 {
                        get_aim(q, r)?
                    } else {
                        n.aim
                    };
                    let (offset, fraction) = if mask & VIEW != 0 {
                        (u32::from(r.u16()?), r.u8()?)
                    } else {
                        (n_offset, n_fraction)
                    };
                    // Canonical: a field its mask names differs from the newer command's (else the encoder leaves it
                    // out), so one command list has one encoding.
                    let same = (mask & BUTTONS != 0 && buttons == n.buttons)
                        || (mask & PRESSES != 0 && presses == n.presses)
                        || (mask & MOVE != 0 && move_xy == n.move_xy)
                        || (mask & AIM_CHANGED != 0 && aim == n.aim)
                        || (mask & VIEW != 0 && (offset, fraction) == (n_offset, n_fraction));
                    if same && q.strict() {
                        return Err(DecodeError::Malformed);
                    }
                    Command {
                        slot: 0,
                        tick,
                        seq: n.seq.wrapping_sub(1),
                        buttons,
                        presses,
                        move_xy,
                        aim,
                        view_tick: tick.wrapping_sub(offset),
                        view_fraction: fraction,
                    }
                }
            };
            r.align(q.strict())?;
            commands.push(c);
        }
        Ok(Input {
            ack_tick,
            ack_groups,
            commands,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_field_table_gives_23_b_whole_19_b_repeats_and_161_b_at_the_cap() {
        assert_eq!(AIM_BITS, 44);
        assert_eq!(WHOLE_BYTES, 23);
        assert_eq!(REPEAT_BYTES, 19);
        assert_eq!(HEADER_BYTES + WHOLE_BYTES + 7 * REPEAT_BYTES, 161);
    }
}
