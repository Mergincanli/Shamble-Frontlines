//! npt-bots: the netcode proof test's headless clients (plan E12-E14): 30 inputs per second, the brawl, the
//! target-course shooter with a page-like frame loop, the cheats, the hostile clients, and the load orchestrator's
//! calibration and roll-out across the CAX21s. Built in units U4 and U6.
//!
//! U4r (u4-build-plan.md §5 U4r; u4r-brief.md 2.2) builds the library's [`Behaviour`], clock-injected and
//! transport-free (U4-SERVER): what one client's page would give its endpoint, from a seed, the client's slot and what
//! the host tells it, never from a clock or a socket. Two kinds: the brawl (the stress and estimate scenes' 48 soldiers
//! fighting in 3 x 16, every one holding fire, so the scene's weapon interval sets the shot rate PM-0001-2 checks:
//! §4.13) and the open-loop scripts (commands from the slot, the seed and the count of commands alone, standing in for
//! the cheats' command patterns in U4r's contract test: U4R-Q4 (a)). The target-course shooter is U4h's, the cheats
//! and the never-acker U4f's. The brawl's walk is on a leash (BRAWL-1): past 14 m from the cluster centre, by its own
//! soldier's position as the endpoint decoded it (`ClientEndpoint::own_soldier`), it steers back toward the centre.
//!
//! **When a command is given** (u4r-brief.md 2.2; carry 74 as U4R-Q14 (b1) amended it): at most one per client step,
//! and none until the endpoint holds a snapshot (before it the stamp would be tick 0), nor at a step whose newest
//! snapshot tick did not move since the last command (a second command would be stamped for the tick the first holds,
//! and the room counts it late: room/src/input.rs:107-113). A skipped tick runs option B's repeat. Each step first hands
//! the endpoint what arrived (`input(None, received, now)`), then, only when the newest tick moved,
//! `input(Some(command), &[], now)` (a call with nothing received only stamps). **The press rule** (U4R-Q14, A4): each
//! press counter moves by at most one edge per command given, never per step (the room refuses more edges than the ticks
//! a command covers: room/src/input.rs:91-99); a press wanted at a step without a command waits for the next command.
//! **The view time** (2.2): a client without `Interp` (U4g's) shows its newest snapshot, so each command claims
//! `view_tick` = the endpoint's newest tick and `view_fraction` = 0, which the room's ring holds at every lead of 1 or
//! more.

use std::collections::BTreeMap;

use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_sim::{Command, Scene};

/// Move vectors 22.5 degrees apart round the circle, each inside the room's x² + y² ≤ 127² check: a soldier walks
/// through them, one step every [`TURN_EVERY`] commands.
const MOVES: [[i8; 2]; 16] = [
    [127, 0],
    [117, 49],
    [89, 89],
    [49, 117],
    [0, 127],
    [-49, 117],
    [-89, 89],
    [-117, 49],
    [-127, 0],
    [-117, -49],
    [-89, -89],
    [-49, -117],
    [0, -127],
    [49, -117],
    [89, -89],
    [117, -49],
];
/// Commands between two turns of the move vector: a count of commands given, with no time or speed in it (a cadence in
/// time would need a rate-derived constant from npt-sim: carry 39; advisor-u4r-r1, FP-7 item 1).
const TURN_EVERY: u64 = 4;
/// Commands a brawler keeps its target before it picks again: a count of commands given, with no time in it.
const RETARGET_EVERY: u64 = 30;
/// The brawl's leash, outer radius, cm from the cluster centre (the origin; BRAWL-1, the advisor's plan review
/// condition 1): past it the walk vector becomes [`toward_centre`] until the soldier is back inside [`LEASH_IN_CM`]. A
/// distance in cm against positions in cm, with no rate in it (carry 39). Its margin is the door-entry margin: a
/// soldier's centre enters a building at 1,800 cm at the nearest door (building 0's, x 1,800 with |y| < 60: npt-sim
/// greybox.rs:83-136), 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a
/// decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only
/// test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance
/// per pattern; BRAWL-1 round 1, NA-B1-2).
const LEASH_OUT_CM: i64 = 1_400;
/// The leash's inner radius, cm: back inside it, the open-loop walk again (the hysteresis between the two radii keeps
/// it from flipping at each command). Above the wall stand-in's contact reach (its north face at 1,220 cm plus a
/// soldier's 40 cm radius: 1,260 cm; greybox.rs:156, world.rs:75-77), so no fixed box lies between the two radii (the
/// crates reach at most 990 cm). The spawn grid's corners lie at 1,414 cm (world.rs:703-722; scene.rs:153), just past
/// the outer radius, so a soldier spawned there steers in at once.
const LEASH_IN_CM: i64 = 1_300;
/// How far above a target's feet the brawl aims, cm (inside a soldier's box).
const AIM_ABOVE_FEET_CM: i32 = 100;
/// Where an aim with no target in view goes: a seeded point within this many cm of the map's centre, in x and y.
const BLIND_SPREAD_CM: u64 = 1_000;
/// The clamp script's points lie this far out in x and y, cm: beyond the weapon's 6,000 cm reach from any soldier near
/// the fight (the spawn grid within 1,414 cm of the centre, the script standing still: at least 14,142 cm away) yet
/// inside the map's ±12,800 cm, so the room's kind 6 never refuses them (u4f-brief.md 2.6; U4r's stand-in at
/// 32,000 cm lay outside the map: NA-8). f18 checks them against the extent the room reads (EA-U4F-4).
const FAR_CM: i32 = 11_000;
/// The fast clock's lane (U4F-Q11 (iii) (c)), cm: its line in y and its two ends in x, east of the fight past every
/// fight vehicle's sweep (hulls 5 to 14 m out, parts within 3.71 m: npt-sim world.rs:801-812), south of building 0's
/// wall (y above -460) and inside the outer vehicles' loops (from 48.3 m), so no box, door, wall or vehicle part lies
/// within its band; npt-bots' lane test shows every live tick moving 20 cm there over a judged run's window.
const LANE_Y_CM: i32 = -1_050;
const LANE_X_CM: [i32; 2] = [2_600, 3_600];
/// The lane's band, cm: inside it (in y, and in x within twice it of the ends) the walk runs along the lane, else it
/// heads for the west end.
const LANE_BAND_CM: i32 = 300;
/// The fast clock's sequences whose stamps it keeps (its copies keep their stamp): twice the redundancy cap.
const STAMPS_KEPT: usize = 16;

/// What a behaviour does with each command it gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Fire held, a walk round a small circle on the leash (BRAWL-1: toward the centre past [`LEASH_OUT_CM`], by the
    /// position the endpoint decoded), aimed at an enemy in view (picked by the seed), else at a seeded point.
    Brawl,
    /// Open loop: fire held and the fire press counter moved by one on every second command (scripted press edges).
    Presses,
    /// Open loop: fire held, aimed at a seeded point far beyond the weapon's reach (the sim clamps it: §4.2).
    Clamp,
    /// Open loop: fire held, the walk, aimed at a seeded point near the centre.
    Steady,
    /// Open loop: the walk with fire never pressed or held (-47's clock suite, U4F-Q4 (b): no event is stamped).
    NoFire,
    /// -21's fast clock: fire held at a seeded point, the lane's walk at full length ([`Behaviour::lane`]).
    Lane,
}

/// One client's behaviour (the module text): the brawl or an open-loop script, seeded per slot.
#[derive(Clone, Debug)]
pub struct Behaviour {
    kind: Kind,
    team: Option<u8>,
    teams: Vec<Option<u8>>,
    rng: u64,
    phase: u64,
    /// The seeded point an aim with no target goes to.
    blind: [i32; 3],
    target: Option<u8>,
    /// A command at every step after the first snapshot (D13's test-only setting): never a judged one.
    every_step: bool,
    presses: [u8; 4],
    last_newest: Option<u32>,
    given: u64,
    skipped: u64,
    /// The leash's state (the brawl only): steering toward the centre, from past [`LEASH_OUT_CM`] until inside
    /// [`LEASH_IN_CM`].
    steering: bool,
    /// U4f's controller rule (U4F-Q7 (iii) (c)): each step gives the commands the endpoint says are due.
    controlled: bool,
    /// A fire press on every `press_every`-th command given (`with_press_every`); 0: the kind's own rule.
    press_every: u64,
    /// -22's never-acker (`never_acks`): every `Input` it sends acknowledges nothing.
    never_acks: bool,
    /// -21's clamp twin (`with_points`): each sequence's aim point, from the clamp run's reports.
    points: BTreeMap<u16, [i32; 3]>,
    /// -21's lag switch (`lag_switch`).
    lag: Option<Lag>,
    /// -21's fast clock (`fast_clock`).
    fast: Option<Fast>,
    /// -21's forger (`forger`).
    forge: Option<Forge>,
    /// Kind 7's rest for the scene's weapon at the build's rate ([`fire_rest_at`]).
    fire_rest: Option<u64>,
    /// The lane walk's way (west), the fast clock's and the lag switch's (FP-4).
    lane_west: bool,
}

/// Kind 7's items, every one a press edge where the sim refuses it (SA-15, both kinds): item i (the i-th due
/// command) is a reload item when i % [`RELOAD_ITEMS`] is 2 (an honest reload edge on the command before, with fire
/// released there, starts a reload, which refuses the forged reload edge; on a soldier revived just before, the full
/// magazine refuses both, and nothing fires to leave it short: FP-1), else a fire item: fire released for the rest
/// ([`fire_rest_at`]) commands before it (so the weapon is ready), an honest fire press on the command before (it
/// fires), and the forged press on the next, in the weapon's cooldown (its interval is at least two ticks). The
/// interval is above the rest (F-G), so the forged command itself is no rest and holds fire: a soldier revived just
/// before it fires by the held trigger, as the twin does. A count of items, no time.
const RELOAD_ITEMS: u64 = 40;

/// Kind 7's rest at `tick_hz` for a weapon of `interval_us` (FP-3; carry 39: the rate an argument, pinned at 30 and
/// 60 Hz): the commands a fire item's weapon rests before its honest press, so a shot's cost (`interval_us` x
/// `tick_hz` units, falling 10^6 a command: npt-sim world.rs:1415-1416) has run out, `ceil(cost / 10^6)`. `None`
/// under two ticks (a cost below 2 x 10^6): the clock carries at most one tick's remainder, so the press one command
/// after the honest shot could find the weapon ready. SS's 68,571 µs: 3 at 30 Hz, 5 at 60 Hz.
pub const fn fire_rest_at(tick_hz: u32, interval_us: u32) -> Option<u64> {
    let cost = interval_us as u64 * tick_hz as u64;
    if cost >= 2_000_000 {
        Some(cost.div_ceil(1_000_000))
    } else {
        None
    }
}

/// Kind 7's dead window (FP-2; F-B-COUNT (a), Mustafa 2026-11-21: "Leave kind 7's presses out of that count when the
/// server's own state shows its soldier dead as that command or the one before it ran"): whether an item is left out of
/// -21's count, from its soldier's state before the command phase of the tick that ran its command d (`dead_at_d`) and
/// of the tick that ran d - 1 (`dead_before`, `None` when d - 1 never ran: then d alone decides). npt-lab's attribution
/// applies it; this crate's World test (T-K7) reads the same rule.
pub const fn kind_7_left_out(dead_at_d: bool, dead_before: Option<bool>) -> bool {
    dead_at_d || matches!(dead_before, Some(true))
}

/// Kind 10's hold, in steps without a command: the burst's 8 accepted commands less its own two steps, so the
/// forger's lead is the same after an episode as before it (a count of steps, with no time in it).
const HOLD_STEPS: u32 = npt_proto::Limits::REDUNDANCY_CAP as u32 - 2;

/// A forger's state (u4f-brief.md 2.4, 2.6, 2.12): its kind, interval, the bound (ticks) and kind 7's rest; whether
/// it is the twin (the same script with only the forged part made honest); kind 10's option (A: hold, then burst;
/// B: burst, then hold); the forged count, the newest command seen (honest), the forged sequences and kind 3's
/// forged copies not yet taken; whether this step's command is the forged one; the last `Input`'s acknowledgement;
/// kind 10's episode step, held datagram, kept stamps and next stamp; the twin's skips owed; kind 11's message; the
/// forged `Input`s that would not encode (0 expected).
#[derive(Clone, Debug, Default)]
struct Forge {
    kind: u8,
    every: u64,
    bound: u32,
    rest: u64,
    twin: bool,
    option_a: bool,
    sent: u64,
    last: Option<Command>,
    forged: Vec<u16>,
    copies: Vec<Command>,
    due: bool,
    ack: (u16, u8),
    episode: u32,
    held: Vec<Vec<u8>>,
    stamps: std::collections::VecDeque<(u16, u32)>,
    next_stamp: Option<u32>,
    skips: u32,
    message: Option<Vec<u8>>,
    unencodable: u64,
    burst: u8,
    resume: Option<u32>,
}

/// How kind 9 leaves an `Input` undecodable: a trailing byte, or a set pad bit (the newest command's last byte's top
/// bit: its 180 bits end 4 bits into its 23rd byte, the header's 5 B before it: proto/src/input.rs:9-10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Undecodable {
    Trailing,
    Pad,
}

/// The byte holding the newest command's pad bits in an `Input` datagram (5 B header, 23 B whole command).
const PAD_BYTE: usize = 5 + 22;

/// Kind 10's commands accepted in a burst (the per-tick cap and B's buffer: 8) and the burst's commands under each
/// option (A 16, B 12: the buffer's 8 and 4 more inside the bound).
const BURST_KEPT: u32 = npt_proto::Limits::REDUNDANCY_CAP as u32;

impl Forge {
    /// Kind 10's forged commands per episode (A 8, B 4), so an episode comes every `every` times that many commands.
    fn burst_refused(&self) -> u32 {
        if self.option_a {
            BURST_KEPT
        } else {
            BURST_KEPT / 2
        }
    }

    /// Each command of `i` (oldest first) takes its kept stamp, or, as a new sequence, the next one (contiguous from
    /// the forger's first stamp, the endpoint's own); a view tick after the stamp is the stamp (the endpoint's rule).
    /// Returns the new sequences.
    fn restamp(&mut self, i: &mut npt_proto::Input) -> Vec<u16> {
        let mut new = Vec::new();
        for c in i.commands.iter_mut().rev() {
            let kept = self.stamps.iter().find(|(s, _)| *s == c.seq).map(|x| x.1);
            c.tick = kept.unwrap_or_else(|| {
                let t = *self.next_stamp.get_or_insert(c.tick);
                self.next_stamp = Some(t.wrapping_add(1));
                self.stamps.push_back((c.seq, t));
                if self.stamps.len() > STAMPS_KEPT {
                    self.stamps.pop_front();
                }
                new.push(c.seq);
                t
            });
            if later(c.view_tick, c.tick) != c.tick {
                (c.view_tick, c.view_fraction) = (c.tick, 0);
            }
        }
        new
    }

    /// Kind 11's message on the control stream: the forger's a `Ping` (a datagram type) framed for that stream, which
    /// the pump refuses as the wrong channel; the twin's a `ClientSettings` repeating the current delay (0: no `Interp`
    /// yet), which changes nothing (2.12).
    fn message(&self, now_ns: u64) -> Option<Vec<u8>> {
        use npt_proto::{ClientSettings, Message, Ping, encode};
        let mut out = Vec::new();
        if self.twin {
            encode(
                &Message::ClientSettings(ClientSettings { delay: 0 }),
                &mut out,
            )
            .ok()?;
            return Some(out);
        }
        let ping = Message::Ping(Ping {
            id: self.sent as u8,
            sent_ns: now_ns,
        });
        encode(&ping, &mut out).ok()?;
        let mut framed = u16::try_from(out.len()).ok()?.to_le_bytes().to_vec();
        framed.append(&mut out);
        Some(framed)
    }

    /// One item forged: counted, and its sequence kept for the lab.
    fn record(&mut self, seq: u16) {
        self.sent += 1;
        self.forged.push(seq);
    }

    /// The forged part of one `Input` whose newest command `c` is new and due, `prev` the command before it, `ack` the
    /// previous `Input`'s acknowledgement (2.6's per-kind table; the twin's rows, 2.12). Returns kind 9's way.
    fn forge_input(
        &mut self,
        i: &mut npt_proto::Input,
        c: Command,
        prev: Option<Command>,
        newest: u32,
        ack: (u16, u8),
    ) -> Option<Undecodable> {
        let (n, twin) = (self.sent + 1, self.twin);
        match self.kind {
            // The twin never forges these (FP-5): its refused commands are never made (`forge_step`'s skip).
            1 | 2 | 4 | 5 | 6 if !twin => {
                i.commands[0] = forged(self.kind, c, prev, newest, self.bound, n)?;
                self.record(c.seq);
            }
            3 => {
                let p = prev.filter(|p| p.seq == c.seq.wrapping_sub(1))?;
                let mut copy = p;
                if !twin {
                    if n % 2 == 1 {
                        copy.move_xy[0] = p.move_xy[0].wrapping_add(1);
                    } else {
                        copy.aim[0] = p.aim[0].saturating_add(100);
                    }
                    self.record(p.seq);
                    self.copies.push(copy);
                }
                match i.commands.get_mut(1) {
                    Some(x) => *x = copy,
                    None => i.commands.push(copy),
                }
            }
            8 | 9 if twin && (self.kind == 9 || i.ack_groups != 0) => {
                i.commands.clear();
                (i.ack_tick, i.ack_groups) = ack;
            }
            8 if i.ack_groups != 0 => {
                i.ack_tick = i.ack_tick.wrapping_add(1_000);
                self.record(c.seq);
            }
            9 => {
                self.record(c.seq);
                return Some(if n % 2 == 1 {
                    Undecodable::Trailing
                } else {
                    Undecodable::Pad
                });
            }
            7 | 11 if !twin => self.record(c.seq),
            _ => {}
        }
        None
    }

    /// The forger's hook on what its endpoint sends at its step (2.6: the honest datagrams, decoded and re-encoded with
    /// only the kind's change; kind 9's left undecodable on purpose; kind 10's restamped, its burst's first `Input`
    /// held and sent ahead of the second's). An `Input` whose forged form would not encode goes honest, counted.
    fn outgoing(&mut self, newest: u32, datagrams: &mut Vec<Vec<u8>>) {
        use npt_proto::{Direction, Message, decode, encode};
        let (due, burst) = (
            core::mem::take(&mut self.due),
            core::mem::take(&mut self.burst),
        );
        let input = |d: &Vec<u8>| {
            matches!(
                decode(Direction::ClientToRoom, Channel::Datagram, d),
                Ok(Message::Input(_))
            )
        };
        for d in datagrams.iter_mut() {
            let Ok(Message::Input(mut i)) = decode(Direction::ClientToRoom, Channel::Datagram, d)
            else {
                continue;
            };
            let honest = i.clone();
            let ack = core::mem::replace(&mut self.ack, (i.ack_tick, i.ack_groups));
            let fresh = i.commands.first().copied().filter(|c| {
                self.last
                    .is_none_or(|l| (c.seq.wrapping_sub(l.seq) as i16) > 0)
            });
            if self.kind == 10 {
                let new = self.restamp(&mut i);
                if burst == 1 {
                    self.resume = self.next_stamp;
                } else if burst == 2 {
                    self.next_stamp = self.resume;
                    let twin = self.twin;
                    for s in new.into_iter().filter(|_| !twin) {
                        self.record(s);
                    }
                }
            }
            let mut way = None;
            if let Some(c) = fresh {
                let prev = i.commands.get(1).copied().or(self.last);
                self.last = Some(c);
                if due {
                    way = self.forge_input(&mut i, c, prev, newest, ack);
                }
            }
            if i == honest && way.is_none() {
                continue;
            }
            let mut out = Vec::new();
            if encode(&Message::Input(i), &mut out).is_err() {
                self.unencodable += 1;
                continue;
            }
            match way {
                Some(Undecodable::Trailing) => out.push(0),
                Some(Undecodable::Pad) => out[PAD_BYTE] |= 0x80,
                None => {}
            }
            *d = out;
        }
        if burst == 1 {
            self.held.extend(datagrams.extract_if(.., |d| input(d)));
        } else if burst == 2 {
            let mut both = core::mem::take(&mut self.held);
            both.append(datagrams);
            *datagrams = both;
        }
    }
}

/// `c`, the newest command of a new sequence, forged in `kind` (2.6's per-kind table), the `n`-th (from 1), against
/// `prev`, the honest command of the sequence before it, at the endpoint's newest tick `newest` under the lead bound
/// `bound`: kind 1 its tick newest + 2 + the bound + k, k from 1 to 100 in turn; kind 2 its tick the previous command's;
/// kind 4 its move [127, 127] or [-128, -128] in turn; kind 5 its fire press counter moved one edge more than the ticks
/// it covers; kind 6 its aim's x or y in turn at 20,000 cm (outside ±12,800, inside the grid's ±32,767). A view tick
/// after the stamp becomes the stamp (the endpoint's own rule). `None` where the kind needs a previous command and has
/// none, or for a kind built elsewhere.
fn forged(
    kind: u8,
    mut c: Command,
    prev: Option<Command>,
    newest: u32,
    bound: u32,
    n: u64,
) -> Option<Command> {
    let (k, even) = (((n - 1) % 100) as u32 + 1, n.is_multiple_of(2));
    match (kind, prev) {
        (1, _) => c.tick = newest.wrapping_add(2 + bound + k),
        (2, Some(p)) => c.tick = p.tick,
        (4, _) => c.move_xy = if even { [127, 127] } else { [-128, -128] },
        (5, Some(p)) => {
            let covered = c.tick.wrapping_sub(p.tick).min(100) as u8;
            let fire = &mut c.presses[Command::PRESS_FIRE];
            *fire = p.presses[Command::PRESS_FIRE].wrapping_add(covered + 1);
        }
        (6, _) => c.aim[usize::from(even)] = if even { -20_000 } else { 20_000 },
        _ => return None,
    }
    if later(c.view_tick, c.tick) != c.tick {
        (c.view_tick, c.view_fraction) = (c.tick, 0);
    }
    Some(c)
}

/// The fast clock's state: the lead bound in ticks, the commands stamped, the last stamp, each recent sequence's stamp
/// (every copy keeps it, so no copy differs: kind 3), and the copies left out of an `Input` that would not encode.
#[derive(Clone, Debug, Default)]
struct Fast {
    bound: u32,
    stamped: u64,
    last: Option<u32>,
    stamps: std::collections::VecDeque<(u16, u32)>,
    left_out: u64,
}

/// The later of two ticks, wrap-aware.
fn later(a: u32, b: u32) -> u32 {
    if (a.wrapping_sub(b) as i32) > 0 { a } else { b }
}

/// The fast clock's stamp for its `k`-th command (from 1) after `last`, at the endpoint's newest tick `newest`, under the
/// lead bound `bound` (U4F-Q11 (v) (b) as the advisor's F1 amends it): every 10th the later of `last` and newest + 3 +
/// the bound (past the bound on arrival at P0, where the room's next tick is at most newest + 2); every other the later
/// of `last` and the earlier of last + 1 and newest + the bound; the first after none newest + 1. Never below `last`.
fn fast_stamp(k: u64, last: Option<u32>, newest: u32, bound: u32) -> u32 {
    let last = last.unwrap_or(newest);
    if k.is_multiple_of(10) {
        return later(newest.wrapping_add(3).wrapping_add(bound), last);
    }
    let (next, cap) = (last.wrapping_add(1), newest.wrapping_add(bound));
    later(last, if later(next, cap) == next { cap } else { next })
}

/// The lag switch's state (u4f-brief.md 2.6): its `Input` datagrams held for `hold_ns` at the start of every `every_ns`
/// of host time from its first send (the first hold one period in), then sent in one burst; each burst's hold.
#[derive(Clone, Debug, Default)]
struct Lag {
    hold_ns: u64,
    every_ns: u64,
    first_ns: Option<u64>,
    held: Vec<Vec<u8>>,
    held_since: Option<u64>,
    bursts: Vec<u64>,
}

/// splitmix64's step: the bots' one generator, seeded per slot (no clock, no hash map).
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A uniform integer in `[0, n)` from one draw, by multiply and shift.
fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

/// A move vector from (x, y) toward the origin at about full command length, inside the room's x² + y² ≤ 127² check
/// (so the sim never scales it: world.rs:494-498): each part is 127 times its share of ⌈|(x, y)|⌉, truncated toward 0,
/// so the vector is never longer than 127 and, at the leash's radii, longer than 125. Integers only. Its inputs are
/// decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), or the fast
/// clock's lane's offsets from them (`lane`: x - x0 and y - LANE_Y_CM, each at most 32,768 + 3,600 in size); in i64,
/// x² + y² of either stays far below i64's range, so the arithmetic cannot overflow.
fn toward_centre(x: i32, y: i32) -> [i8; 2] {
    let (x, y) = (i64::from(x), i64::from(y));
    let d2 = x * x + y * y;
    let r = d2.isqrt();
    let len = if r * r < d2 { r + 1 } else { r }.max(1);
    let part = |v: i64| i8::try_from(-v * 127 / len).unwrap_or(0);
    [part(x), part(y)]
}

impl Behaviour {
    fn new(kind: Kind, scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        let mut rng = seed ^ (u64::from(slot) << 56) ^ 0x6e70_742d_626f_7473;
        let phase = below(next(&mut rng), MOVES.len() as u64);
        let spread =
            |r: &mut u64| below(next(r), 2 * BLIND_SPREAD_CM + 1) as i32 - BLIND_SPREAD_CM as i32;
        let blind = match kind {
            Kind::Clamp => {
                let sx = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
                let sy = if next(&mut rng) & 1 == 0 { 1 } else { -1 };
                [sx * FAR_CM, sy * FAR_CM, AIM_ABOVE_FEET_CM]
            }
            _ => [spread(&mut rng), spread(&mut rng), AIM_ABOVE_FEET_CM],
        };
        Behaviour {
            kind,
            team: scene.team(slot),
            teams: (0..scene.slots()).map(|s| scene.team(s)).collect(),
            rng,
            phase,
            blind,
            target: None,
            every_step: false,
            presses: [0; 4],
            last_newest: None,
            given: 0,
            skipped: 0,
            steering: false,
            controlled: false,
            press_every: 0,
            never_acks: false,
            points: BTreeMap::new(),
            lag: None,
            fast: None,
            forge: None,
            fire_rest: fire_rest_at(npt_sim::TICK_HZ, scene.weapon().interval_us),
            lane_west: false,
        }
    }

    /// -21's forger of `kind` (2.4's 11 kinds; `None` for another) under the lead bound `bound` (ticks), on a
    /// fixed-lead endpoint at one command a step (2.6's per-kind table): the same behaviour with one forged item per
    /// `every` commands given, in 2.4's unit. Kinds 1, 2, 4, 5 and 6: the `every`-th command's newest copy forged in
    /// the first `Input` that carries it (the endpoint keeps the honest command, so every later copy is a repeat of a
    /// refused sequence: 2.4 item 2). Kind 3: that `Input` carries the previous command's copy with its move or aim
    /// changed. Kind 7: the command itself carries a fire edge in the weapon's cooldown, or (every 40th item, the
    /// crate's `RELOAD_ITEMS`) a reload edge inside a reload an honest edge started, where the sim refuses it. Kind 8:
    /// that `Input`'s acknowledgement names a tick never sent; kind 9: it takes a trailing byte or a set pad bit (in
    /// turn); kind 11: a `Ping` on the control stream beside it. Kind 10: an episode every `every` x 8 (A) or x 4 (B)
    /// commands, 16 (A) or 12 (B) commands in two datagrams sent in one step, stamped on from its last accepted stamp +
    /// 1, so all but the first 8 are refused by the catch-up cap (A) or the full buffer (B), 6 steps without a command
    /// (the crate's `HOLD_STEPS`) before (A) or after (B) the burst keeping its lead. `None` for `every` below 2:
    /// kinds 2 and 5 forge against the previous command, so it must have been accepted (SEC-U4F-1), which every
    /// interval of 2 or more gives on a lossless profile (P0); for kind 7 when the scene's weapon has no rest at the
    /// build's rate or `every` is not above it (`fire_rest_at`: SS's 3 at 30 Hz, so from 4; FP-3, F-G: the forged
    /// command itself then holds fire, so a soldier revived just before it fires as the twin's held trigger does); and
    /// for an `every` whose 8 times overflows (kind 10's episode: FP-8).
    pub fn forger(mut self, kind: u8, every: u64, bound: u32) -> Option<Behaviour> {
        ((1..=11).contains(&kind) && every >= 2).then_some(())?;
        every.checked_mul(u64::from(BURST_KEPT))?;
        let rest = match self.fire_rest {
            Some(r) if kind == 7 && every > r => r,
            _ if kind == 7 => return None,
            _ => 0,
        };
        self.forge = Some(Forge {
            kind,
            every,
            bound,
            rest,
            ..Forge::default()
        });
        Some(self)
    }

    /// The forger's twin (2.12): the same script and the same commands given, with only the forged part made honest:
    /// kinds 1, 2, 4, 5, 6 and 10's refused commands never made (their sequence index and stamp used by
    /// `ClientEndpoint::skip_command`: kinds 1, 2, 4, 5 and 6's at the due step itself, in the command's place; kind
    /// 10's burst's at the next step, once the acknowledgement emptied the endpoint's copies: FP-8, NC-U4F2-6); kind
    /// 3's copy with its honest content; kind 7's command without the forged edge; kinds 8's and 9's `Input` the header
    /// only, its acknowledgement the previous `Input`'s; kind 11's message a `ClientSettings` repeating the delay.
    pub fn twin(mut self) -> Behaviour {
        if let Some(f) = self.forge.as_mut() {
            f.twin = true;
        }
        self
    }

    /// Kind 10's option A shape (hold, then burst; a burst of 16): the room's option, which the lab gives it.
    pub fn catchup_under_a(mut self) -> Behaviour {
        if let Some(f) = self.forge.as_mut() {
            f.option_a = true;
        }
        self
    }

    /// The sequences the forger forged since the last take (the lab's attribution: 2.4 item 6): kind 3's the copy's,
    /// kind 10's the refused burst's, every other kind's the forged command's (kinds 8, 9 and 11: its step's newest).
    pub fn take_forged(&mut self) -> Vec<u16> {
        self.forge
            .as_mut()
            .map(|f| core::mem::take(&mut f.forged))
            .unwrap_or_default()
    }

    /// Kind 3's forged copies since the last take, each as sent (the attribution's "no command with the forged copy's
    /// content runs").
    pub fn take_copies(&mut self) -> Vec<Command> {
        self.forge
            .as_mut()
            .map(|f| core::mem::take(&mut f.copies))
            .unwrap_or_default()
    }

    /// The forger's `Input`s whose forged form would not encode, sent honest instead (0 expected: TE3-4, SA3-2).
    pub fn forge_unencodable(&self) -> u64 {
        self.forge.as_ref().map_or(0, |f| f.unencodable)
    }

    /// Kind 11's hook on what the endpoint sends reliably at its step: the step's message onto the control stream.
    pub fn reliable(&mut self, out: &mut Vec<(Channel, Vec<u8>)>) {
        if let Some(m) = self.forge.as_mut().and_then(|f| f.message.take()) {
            out.push((Channel::Control, m));
        }
    }

    /// -21's 1.5x clock (u4f-brief.md 2.6; U4F-Q11 (iii) (c) and (v) (b) as F1 amends it): fire held at a seeded point,
    /// the lane's walk at full length, a command at every step once the endpoint holds a snapshot (its host clock's rate
    /// is the lab's), each stamped by the crate's `fast_stamp` under the lead bound `bound` (ticks) in
    /// [`Behaviour::outgoing`].
    pub fn fast_clock(scene: &Scene, slot: u8, seed: u64, bound: u32) -> Behaviour {
        let mut b = Behaviour::new(Kind::Lane, scene, slot, seed).every_step();
        b.fast = Some(Fast {
            bound,
            ..Fast::default()
        });
        b
    }

    /// The fast clock's copies left out of an `Input` whose rewritten stamps would not encode (0 expected at P0).
    pub fn fast_left_out(&self) -> u64 {
        self.fast.as_ref().map_or(0, |f| f.left_out)
    }

    /// The lane walk (U4F-Q11 (iii) (c); the fast clock's, and since FP-4 the lag switch's): by its decoded position,
    /// toward the lane's west end until inside the lane's band, then along the lane at full length (20 cm a tick),
    /// turning at each end; no complete tick yet, or a dead soldier, stands (the sim moves no dead soldier).
    fn lane(&mut self, own: Option<([i32; 3], bool)>) -> [i8; 2] {
        let Some(([x, y, _], false)) = own else {
            return [0, 0];
        };
        let [x0, x1] = LANE_X_CM;
        let reach = 2 * LANE_BAND_CM;
        if (y - LANE_Y_CM).abs() > LANE_BAND_CM || x < x0 - reach || x > x1 + reach {
            return toward_centre(x - x0, y - LANE_Y_CM);
        }
        if x >= x1 {
            self.lane_west = true;
        } else if x <= x0 {
            self.lane_west = false;
        }
        if self.lane_west { [-127, 0] } else { [127, 0] }
    }

    /// -21's lag-switch client's script (FP-4, NC-U4F2-3): fire held at a seeded point and the fast clock's lane walk
    /// on the fixed-lead rule, so its movement can show a burst's commands run (the standing steady script moved
    /// nothing); closed loop, as the suite has no twin. [`Behaviour::lag_switch`] goes on it.
    pub fn lane_walk(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        Behaviour::new(Kind::Lane, scene, slot, seed)
    }

    /// -21's lag switch (2.6; U4F-Q15 (c)): the same behaviour, its `Input` datagrams held `hold_ns` (200 ms in the
    /// judged runs: about 6 datagrams, inside the bucket's 8) once every `every_ns` (6 s: 100 bursts in 600 s), then
    /// sent in one burst at the first step after the hold, ahead of that step's own; on simulated host time only.
    pub fn lag_switch(mut self, hold_ns: u64, every_ns: u64) -> Behaviour {
        self.lag = Some(Lag {
            hold_ns,
            every_ns: every_ns.max(1),
            ..Lag::default()
        });
        self
    }

    /// The lag switch's bursts sent since the last take, each its hold in ns (`event.lag_burst.held_ns`).
    pub fn take_bursts(&mut self) -> Vec<u64> {
        self.lag
            .as_mut()
            .map(|l| core::mem::take(&mut l.bursts))
            .unwrap_or_default()
    }

    /// -21's clamp twin (2.12): the same behaviour with each listed sequence's aim replaced, in every `Input` that
    /// carries it, by the point the clamp run's sim clamped that command's shot to (`event.clamp_report`): a point at
    /// the reach, so the twin's shots are the clamp run's.
    pub fn with_points(mut self, points: BTreeMap<u16, [i32; 3]>) -> Behaviour {
        self.points = points;
        self
    }

    /// -22's never-acker (u4f-brief.md 2.6): the same behaviour, its every `Input` re-encoded with no acknowledgement
    /// (ack groups 0) by [`Behaviour::outgoing`], so the room never learns a snapshot or a full state arrived.
    pub fn never_acks(mut self) -> Behaviour {
        self.never_acks = true;
        self
    }

    /// The cheats' hook on what the endpoint sends at its step at `now_ns` (host time; 2.6: the honest datagram, decoded
    /// and re-encoded with only the cheat's change): the never-acker's `Input`s lose their acknowledgement, the clamp
    /// twin's commands take their listed points, the lag switch holds its `Input`s and sends them in a burst; an honest
    /// behaviour changes nothing. Every rewritten `Input` encodes (only the ack fields and aims inside the map change).
    pub fn outgoing(&mut self, now_ns: u64, datagrams: &mut Vec<Vec<u8>>) {
        use npt_proto::{Direction, Message, decode, encode};
        if let Some(l) = self.lag.as_mut() {
            let first = *l.first_ns.get_or_insert(now_ns);
            let since = now_ns.saturating_sub(first);
            if since >= l.every_ns && since % l.every_ns < l.hold_ns {
                let input = |d: &Vec<u8>| {
                    matches!(
                        decode(Direction::ClientToRoom, Channel::Datagram, d),
                        Ok(Message::Input(_))
                    )
                };
                l.held_since.get_or_insert(now_ns);
                l.held.extend(datagrams.extract_if(.., |d| input(d)));
            } else if let Some(at) = l.held_since.take() {
                let mut burst = core::mem::take(&mut l.held);
                burst.append(datagrams);
                *datagrams = burst;
                l.bursts.push(now_ns.saturating_sub(at));
            }
        }
        let newest = self.last_newest.unwrap_or(0);
        if let Some(f) = self.forge.as_mut() {
            f.outgoing(newest, datagrams);
            return;
        }
        if !self.never_acks && self.points.is_empty() && self.fast.is_none() {
            return;
        }
        for d in datagrams.iter_mut() {
            if let Ok(Message::Input(mut i)) = decode(Direction::ClientToRoom, Channel::Datagram, d)
            {
                if self.never_acks {
                    (i.ack_tick, i.ack_groups) = (0, 0);
                }
                for c in &mut i.commands {
                    if let Some(p) = self.points.get(&c.seq) {
                        c.aim = *p;
                    }
                }
                // The fast clock: each new sequence, oldest first, takes its stamp; a copy keeps its sequence's; a view
                // tick after the stamp is the stamp (the endpoint's rule, so the view offset encodes).
                if let Some(f) = self.fast.as_mut() {
                    for c in i.commands.iter_mut().rev() {
                        let kept = f.stamps.iter().find(|(s, _)| *s == c.seq).map(|x| x.1);
                        c.tick = kept.unwrap_or_else(|| {
                            f.stamped += 1;
                            let t = fast_stamp(f.stamped, f.last, newest, f.bound);
                            f.last = Some(t);
                            f.stamps.push_back((c.seq, t));
                            if f.stamps.len() > STAMPS_KEPT {
                                f.stamps.pop_front();
                            }
                            t
                        });
                        if later(c.view_tick, c.tick) != c.tick {
                            (c.view_tick, c.view_fraction) = (c.tick, 0);
                        }
                    }
                }
                // Every rewritten `Input` encodes: an older copy a past-bound stamp leaves more than 255 ticks below
                // (after an idle) is left out, oldest first, so the rest stay consecutive (the codec's two rules).
                let mut out = Vec::new();
                let mut ok = encode(&Message::Input(i.clone()), &mut out).is_ok();
                while !ok && i.commands.len() > 1 {
                    i.commands.pop();
                    out.clear();
                    if let Some(f) = self.fast.as_mut() {
                        f.left_out += 1;
                    }
                    ok = encode(&Message::Input(i.clone()), &mut out).is_ok();
                }
                if ok {
                    *d = out;
                }
            }
        }
    }

    /// The same behaviour under U4f's tick-lead controller (`ClientEndpoint::with_clock`; u4f-brief.md 2.2): each step
    /// gives exactly the commands the endpoint says are due (none, one or two; none before its first stamp is due), so
    /// no tick goes without a stamped command but by loss. Each command still moves each press counter by at most one
    /// edge (U4R-Q14's (b1) and A4). The fixed-lead rule (one command only when the newest tick moved) is the default.
    pub fn controlled(mut self) -> Behaviour {
        self.controlled = true;
        self
    }

    /// The same behaviour with a fire press on every `n`-th command it gives (U4F-Q11 (i) (a): -20's runs give the brawl
    /// `n` = 2, npt-lab's `--brawl-press-every`, for its 10,000 scripted presses on the load r2's P3 suite showed); a
    /// press waits for a command, never per step; 0 keeps the kind's own rule.
    pub fn with_press_every(mut self, n: u64) -> Behaviour {
        self.press_every = n;
        self
    }

    /// The brawl of `slot` in `scene`, from the run's `seed`.
    pub fn brawl(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        Behaviour::new(Kind::Brawl, scene, slot, seed)
    }

    /// The clock suite's open-loop script (-47; U4F-Q4 (b)): the walk, fire never held or pressed, so the room stamps
    /// no gameplay event and carry 75's admission bound is never met.
    pub fn never_fires(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        Behaviour::new(Kind::NoFire, scene, slot, seed)
    }

    /// The open-loop script named `script` (`presses`, `clamp` or `steady`) of `slot` in `scene`, from `seed`;
    /// `None` for another name.
    pub fn script(scene: &Scene, slot: u8, seed: u64, script: &str) -> Option<Behaviour> {
        let kind = match script {
            "presses" => Kind::Presses,
            "clamp" => Kind::Clamp,
            "steady" => Kind::Steady,
            _ => return None,
        };
        Some(Behaviour::new(kind, scene, slot, seed))
    }

    /// The same behaviour giving a command at every step once the endpoint holds a snapshot, whether or not its newest
    /// tick moved: D13's test-only setting (u4r-brief.md section 11), which shows the duplicate stamps U4R-Q14 (b)
    /// counts; never a judged run's.
    pub fn every_step(mut self) -> Behaviour {
        self.every_step = true;
        self
    }

    /// The commands given so far.
    pub fn given(&self) -> u64 {
        self.given
    }

    /// The steps since the endpoint's first snapshot at which no command was given (its newest tick had not moved).
    pub fn skipped(&self) -> u64 {
        self.skipped
    }

    /// One client step at `now_ns` on the host's clock (u4r-brief.md 2.2; the module text): `received` goes to the
    /// endpoint first, then a command if one is due. `view` is what the host says this client sees at its newest tick:
    /// (slot, position in the position grid's cm) of other soldiers; the open-loop scripts never read it. Returns
    /// whether a command was given. A host that computes a view must feed the endpoint itself and pass `&[]` (the view
    /// is of the newest tick the endpoint holds after what it was fed: advisor-u4r-r1, FP-7 item 4).
    pub fn step(
        &mut self,
        endpoint: &mut ClientEndpoint,
        received: &[(Channel, Vec<u8>, u64)],
        now_ns: u64,
        view: &[(u8, [i32; 3])],
    ) -> bool {
        endpoint.input(None, received, now_ns);
        let (Some(newest), Some(_)) = (endpoint.newest_tick(), endpoint.slot()) else {
            return false;
        };
        if self.controlled {
            let due = endpoint.commands_due();
            if due == 0 {
                self.skipped += 1;
            }
            for _ in 0..due {
                let command = self.next_command(newest, view, endpoint.own_soldier());
                endpoint.input(Some(command), &[], now_ns);
            }
            return due > 0;
        }
        if !self.every_step && self.last_newest == Some(newest) {
            self.skipped += 1;
            return false;
        }
        if self.forge.is_some() {
            return self.forge_step(endpoint, newest, view, now_ns);
        }
        let command = self.next_command(newest, view, endpoint.own_soldier());
        endpoint.input(Some(command), &[], now_ns);
        true
    }

    /// A forger's step once its newest tick moved ([`Behaviour::forger`], [`Behaviour::twin`]): the twin's skips owed
    /// first, then this step's command, each forged part marked for [`Behaviour::outgoing`] (kind 7's edges put on
    /// the command itself, kind 11's message for [`Behaviour::reliable`]). Returns whether a command was given.
    fn forge_step(
        &mut self,
        e: &mut ClientEndpoint,
        newest: u32,
        view: &[(u8, [i32; 3])],
        now_ns: u64,
    ) -> bool {
        let own = e.own_soldier();
        let n = self.given + 1;
        let Some(f) = self.forge.as_mut() else {
            return false;
        };
        for _ in 0..core::mem::take(&mut f.skips) {
            e.skip_command();
        }
        if f.kind == 10 {
            return self.catchup_step(e, newest, view, own, now_ns);
        }
        let due = n.is_multiple_of(f.every);
        // Kind 7 ([`RELOAD_ITEMS`]): the due command `d`'s item kind, by its index d / every (reload item i % 40 == 2:
        // d is at least 4, so a shot has left the magazine short of full by its honest edge at d - 1, unless a revive
        // filled it, and then the full magazine refuses both edges).
        let (every, kind) = (f.every, f.kind);
        let reload = |d: u64| (d / every) % RELOAD_ITEMS == 2;
        let ahead = |k: u64| kind == 7 && (n + k).is_multiple_of(every);
        // Fire released on the command before every item, and for the rest before a fire item (FP-1, FP-3).
        let rest = (1..=f.rest).any(|k| ahead(k) && (k == 1 || !reload(n + k)));
        if ahead(1) {
            let b = if reload(n + 1) {
                Command::PRESS_RELOAD
            } else {
                Command::PRESS_FIRE
            };
            self.presses[b] = self.presses[b].wrapping_add(1);
        }
        if kind == 7 && due && !f.twin {
            let b = if reload(n) {
                Command::PRESS_RELOAD
            } else {
                Command::PRESS_FIRE
            };
            self.presses[b] = self.presses[b].wrapping_add(1);
        }
        let skip = due && f.twin && [1, 2, 4, 5, 6].contains(&kind);
        f.due = due && !(f.twin && kind == 7) && !skip;
        if due && kind == 11 {
            f.message = f.message(now_ns);
        }
        let mut command = self.next_command(newest, view, own);
        if rest {
            // Kind 7: before a fire item the weapon rests (fire released) so the honest press before the forged one
            // fires; before a reload item nothing fires, so a magazine a revive filled stays full (FP-1).
            command.buttons &= !Command::FIRE;
        }
        if skip {
            // The twin of a room-whole kind: the refused command is never made (U4F-Q13 (b)).
            e.skip_command();
            return false;
        }
        e.input(Some(command), &[], now_ns);
        true
    }

    /// Kind 10's step ([`Behaviour::forger`]): an episode once every `every` x its refused count commands (A: 8,
    /// B: 4): under A, [`HOLD_STEPS`] steps with no command, then 8 commands (their `Input` held), then 8 more (both
    /// `Input`s sent in this step); under B the 8, then 4 more, then the hold. The twin gives none of the burst's
    /// refused commands (generated, never made: their indices skipped at its next step).
    fn catchup_step(
        &mut self,
        e: &mut ClientEndpoint,
        newest: u32,
        view: &[(u8, [i32; 3])],
        own: Option<([i32; 3], bool)>,
        now_ns: u64,
    ) -> bool {
        let given = self.given;
        let Some(f) = self.forge.as_mut() else {
            return false;
        };
        let period = f.every * u64::from(f.burst_refused());
        if f.episode == 0 && given > 0 && given.is_multiple_of(period) {
            f.episode = 1;
        }
        let (first, second) = if f.option_a {
            (HOLD_STEPS + 1, HOLD_STEPS + 2)
        } else {
            (1, 2)
        };
        let step = f.episode;
        let give = match step {
            0 => 1,
            s if s == first => BURST_KEPT,
            s if s == second => f.burst_refused(),
            _ => 0,
        };
        if step > 0 {
            f.episode = if step == HOLD_STEPS + 2 { 0 } else { step + 1 };
            f.burst = if step == first {
                1
            } else {
                2 * u8::from(step == second)
            };
        }
        let unmade = step == second && f.twin;
        if unmade {
            f.skips = give;
        }
        for _ in 0..give {
            let c = self.next_command(newest, view, own);
            if !unmade {
                e.input(Some(c), &[], now_ns);
            }
        }
        give > 0 && !unmade
    }

    /// The decision function (crate-private; `step` calls it, and so do this crate's tests): the next command from
    /// `newest`, `view` and `own`, the own soldier as the endpoint decoded it (`ClientEndpoint::own_soldier`: its grid
    /// position and dead bit), counted as given.
    fn next_command(
        &mut self,
        newest: u32,
        view: &[(u8, [i32; 3])],
        own: Option<([i32; 3], bool)>,
    ) -> Command {
        let command = self.command(newest, view, own);
        self.last_newest = Some(newest);
        self.given += 1;
        command
    }

    /// The brawl's walk on the leash (BRAWL-1, Fix (a)): `walk`, the open-loop vector, unless the own soldier is past
    /// [`LEASH_OUT_CM`] from the centre, where it becomes [`toward_centre`] until the soldier is back inside
    /// [`LEASH_IN_CM`]. It reads only the position the client decoded (x and y in toward_centre's domain), never the
    /// server's state, so a re-run command or a step cut short is corrected once the decoded position shows it
    /// (lead - 1 steps later, plus the ticks between the newest and the newest complete tick). `None` (no complete
    /// tick yet) or a dead soldier (a respawn puts it at home) keeps the walk and the leash's state.
    fn leash(&mut self, own: Option<([i32; 3], bool)>, walk: [i8; 2]) -> [i8; 2] {
        let Some(([x, y, _], false)) = own else {
            return walk;
        };
        let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
        if d2 > LEASH_OUT_CM.pow(2) {
            self.steering = true;
        } else if d2 < LEASH_IN_CM.pow(2) {
            self.steering = false;
        }
        if self.steering {
            toward_centre(x, y)
        } else {
            walk
        }
    }

    /// The next command (its slot, tick and sequence are the endpoint's to stamp), claiming `newest` as its view.
    fn command(
        &mut self,
        newest: u32,
        view: &[(u8, [i32; 3])],
        own: Option<([i32; 3], bool)>,
    ) -> Command {
        let k = self.given;
        let every = if self.press_every > 0 {
            self.press_every
        } else {
            2 * u64::from(self.kind == Kind::Presses)
        };
        if every > 0 && k % every == every - 1 {
            self.presses[Command::PRESS_FIRE] = self.presses[Command::PRESS_FIRE].wrapping_add(1);
        }
        let aim = match self.kind {
            Kind::Brawl => self.brawl_aim(k, view),
            _ => self.blind,
        };
        let walk = MOVES[((k / TURN_EVERY + self.phase) % MOVES.len() as u64) as usize];
        let walk = match self.kind {
            Kind::Brawl => self.leash(own, walk),
            Kind::Lane => self.lane(own),
            // C-U4F2-5 (U4F2-Q1 (c), (b) not shown): the steady script stands, its fire, aim and presses kept. Its
            // circle walk entered buildings in the sim-only test at full length and at 1/32 of it (a walking soldier
            // that meets a passing vehicle part is set at the part's face), while a still soldier never moves.
            Kind::Steady => [0, 0],
            _ => walk,
        };
        Command {
            buttons: if self.kind == Kind::NoFire {
                0
            } else {
                Command::FIRE
            },
            presses: self.presses,
            move_xy: if self.kind == Kind::Clamp {
                [0, 0]
            } else {
                walk
            },
            aim,
            view_tick: newest,
            view_fraction: 0,
            ..Command::default()
        }
    }

    /// The brawl's aim: its target's position in view, raised into its box, re-picked by the seed every
    /// [`RETARGET_EVERY`] commands or when the target is out of view; else the seeded blind point.
    fn brawl_aim(&mut self, k: u64, view: &[(u8, [i32; 3])]) -> [i32; 3] {
        let enemy = |s: &u8| {
            let team = self.teams.get(usize::from(*s)).copied().flatten();
            team.is_some() && team != self.team
        };
        let in_view = |t: u8| view.iter().any(|(s, _)| *s == t);
        if k.is_multiple_of(RETARGET_EVERY) || !self.target.is_some_and(in_view) {
            let enemies: Vec<u8> = view.iter().map(|(s, _)| *s).filter(enemy).collect();
            self.target = (!enemies.is_empty())
                .then(|| enemies[below(next(&mut self.rng), enemies.len() as u64) as usize]);
        }
        match self.target.and_then(|t| view.iter().find(|(s, _)| *s == t)) {
            Some((_, p)) => [p[0], p[1], p[2].saturating_add(AIM_ABOVE_FEET_CM)],
            None => self.blind,
        }
    }
}

/// BRAWL-1's tests (Fix (c); the advisor's plan review, conditions 2, 3, 4 and 13): in this crate, through the
/// crate-private decision function; only this module names npt-sim's World.
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use npt_sim::{
        EntityKind, HitHistory, Placement, SimCore, StandIns, TickInput, World, ticks_for_ms,
    };

    use super::*;

    const _: () = assert!(
        LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500,
        "the advisor's plan review, condition 1's bounds"
    );

    /// A live soldier at (x, y), as the endpoint would give it.
    fn at(x: i32, y: i32) -> Option<([i32; 3], bool)> {
        Some(([x, y, 150], false))
    }

    fn len2(m: [i8; 2]) -> i64 {
        i64::from(m[0]).pow(2) + i64::from(m[1]).pow(2)
    }

    /// The steering unit test: the decision function with set positions, beside a twin of the same seed fed no
    /// position (today's open-loop walk, command for command). Inside LEASH_IN_CM it walks; between the radii it keeps
    /// what it did (the hysteresis); past LEASH_OUT_CM it steers to the centre and keeps steering under blocked steps
    /// (the position does not move) and skipped ones (the room re-ran the last command: the next position is further
    /// out) until it is inside LEASH_IN_CM; None and dead keep the walk and the leash's state; on the outer radius it
    /// is not past it; the open-loop scripts never steer. No sideways part is kept (condition 1 lets it go: no fixed
    /// box lies between the radii). Every leash vector holds x² + y² ≤ 127², above 125² from LEASH_IN_CM out, and
    /// points toward the centre. The blocked, kept-state and outer-radius checks each fall on a command whose
    /// open-loop walk differs from the steer, so each can fail (BRAWL-1 round 1, TE-B1-3).
    #[test]
    fn brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one() {
        let scene = Scene::stress();
        let mut b = Behaviour::brawl(&scene, 5, 1);
        let mut twin = Behaviour::brawl(&scene, 5, 1);
        let mut give = |own: Option<([i32; 3], bool)>| {
            let walk = twin.next_command(100, &[], None).move_xy;
            (b.next_command(100, &[], own).move_xy, walk)
        };
        for (own, why) in [
            (at(0, 0), "the centre"),
            (at(500, -300), "inside"),
            (at(1_350, 0), "between the radii, never past the outer"),
        ] {
            let (got, walk) = give(own);
            assert_eq!(got, walk, "{why}: the walk");
        }
        let steer = [-127, 0];
        assert_eq!(give(at(1_450, 0)).0, steer, "past the outer radius");
        let blocked = toward_centre(1_450, 200);
        for _ in 0..3 {
            let (got, walk) = give(at(1_450, 200));
            assert_ne!(
                walk, blocked,
                "the twin's walk differs, so the check can fail"
            );
            assert_eq!(got, blocked, "blocked: the position does not move");
        }
        assert_eq!(
            give(at(1_480, 30)).0,
            toward_centre(1_480, 30),
            "skipped: further out"
        );
        assert_eq!(
            give(at(1_350, 0)).0,
            steer,
            "between the radii after it steered"
        );
        assert_eq!(
            give(at(1_301, 0)).0,
            steer,
            "still outside the inner radius"
        );
        let (got, walk) = give(Some(([0, 0, 150], true)));
        assert_eq!(got, walk, "dead: the walk");
        let (got, walk) = give(None);
        assert_eq!(got, walk, "no complete tick: the walk");
        let (got, walk) = give(at(1_350, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, steer, "None and dead keep the leash's state");
        let (got, walk) = give(at(1_299, 0));
        assert_eq!(got, walk, "inside the inner radius: the walk again");
        let (got, walk) = give(at(1_350, 0));
        assert_eq!(got, walk, "between the radii, not past the outer since");
        let (got, walk) = give(at(1_400, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, walk, "on the outer radius: not past it");
        let (got, walk) = give(at(1_401, 0));
        assert_ne!(
            walk, steer,
            "the twin's walk differs, so the check can fail"
        );
        assert_eq!(got, steer, "1 cm past the outer radius");
        assert_eq!(
            give(at(1_100, -1_100)).0,
            [-89, 89],
            "a diagonal past the outer radius"
        );
        let mut steady = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
        let mut open = Behaviour::script(&scene, 5, 1, "steady").expect("a script");
        assert_eq!(
            steady.next_command(100, &[], at(5_000, 0)).move_xy,
            open.next_command(100, &[], None).move_xy,
            "an open-loop script never steers"
        );
        let edges = [
            (32_767, 32_767),
            (-32_768, -32_768),
            (32_767, 0),
            (0, -32_768),
            (1_300, 0),
        ];
        let sweep = (-3_000..=3_000i32)
            .step_by(37)
            .flat_map(|x| (-3_000..=3_000i32).step_by(41).map(move |y| (x, y)));
        for (x, y) in sweep.chain(edges) {
            let d2 = i64::from(x).pow(2) + i64::from(y).pow(2);
            if d2 < LEASH_IN_CM.pow(2) {
                continue;
            }
            let m = toward_centre(x, y);
            assert!(
                len2(m) <= 127 * 127 && len2(m) > 125 * 125,
                "({x}, {y}): {m:?}"
            );
            assert!(
                i64::from(m[0]) * i64::from(x) + i64::from(m[1]) * i64::from(y) < 0,
                "({x}, {y}): {m:?} toward the centre"
            );
        }
    }

    /// f36 (ii) (TE3-1): over 300 steps of a lossless loop on the room's tick grid (a snapshot each step but every fifth,
    /// each ping answered at once with the server's time), a controller run's behaviour gives a command for every stamp
    /// the endpoint names (as many commands as stamps sent, the stamps contiguous, none at a step with none due: the
    /// endpoint's `not_due` 0, 2.2's bot rule); a fixed-lead run keeps
    /// lib.rs:14-21's rule: one command at each step whose newest tick moved, none at the others.
    #[test]
    fn f36_ii_a_controller_run_gives_every_stamp_and_a_fixed_lead_run_keeps_its_rule() {
        use npt_proto::{Direction, JoinAccepted, Message, Pong, Snapshot, decode, encode};
        let bytes = |m: &Message| {
            let mut o = Vec::new();
            encode(m, &mut o).expect("encodes");
            o
        };
        let scene = Scene::stress();
        let period = 1_000_000_000 / 30;
        for controlled in [true, false] {
            let (mut e, mut b) = (
                ClientEndpoint::new(npt_proto::Join::default(), 30),
                Behaviour::brawl(&scene, 0, 1),
            );
            if controlled {
                (e, b) = (e.with_clock(9), b.controlled());
            } else {
                e = e.with_lead(2);
            }
            let ja = Message::JoinAccepted(JoinAccepted {
                tick: 100,
                ..JoinAccepted::default()
            });
            let mut rx = vec![(Channel::Control, bytes(&ja), 0)];
            let (mut stamps, mut gave, mut moved) = (Vec::new(), 0u64, 0u64);
            for k in 0..300u32 {
                let now = u64::from(k) * period;
                if k % 5 != 4 {
                    let s = Snapshot {
                        tick: 100 + k,
                        groups: 1,
                        ..Snapshot::default()
                    };
                    rx.push((Channel::Datagram, bytes(&Message::Snapshot(s)), now));
                    moved += 1;
                }
                let given = b.step(&mut e, &rx, now, &[]);
                gave += u64::from(given);
                assert!(controlled || given == (k % 5 != 4), "step {k}");
                rx.clear();
                for d in e.step().datagrams {
                    match decode(Direction::ClientToRoom, Channel::Datagram, &d) {
                        Ok(Message::Input(i)) => {
                            for c in i.commands.iter().rev() {
                                if stamps.last().is_none_or(|&(s, _)| c.seq > s) {
                                    stamps.push((c.seq, c.tick));
                                }
                            }
                        }
                        Ok(Message::Ping(p)) => {
                            let pong = Pong {
                                id: p.id,
                                echo_ns: p.sent_ns,
                                tick: 101 + k,
                                hold_us: 0,
                                server_ns: now,
                            };
                            rx.push((Channel::Datagram, bytes(&Message::Pong(pong)), now));
                        }
                        _ => {}
                    }
                }
            }
            assert!(stamps.len() >= 240, "{controlled}: {} stamps", stamps.len());
            for w in stamps.windows(2) {
                assert_eq!(w[1].0, w[0].0 + 1, "{controlled}: every sequence sent");
                if controlled {
                    assert_eq!(w[1].1, w[0].1 + 1, "contiguous stamps: {w:?}");
                }
            }
            let want = if controlled {
                stamps.len() as u64
            } else {
                moved
            };
            assert_eq!(b.given(), want, "{controlled}: commands given");
            assert!(gave <= b.given(), "{controlled}");
            assert_eq!(e.not_due(), 0, "{controlled}: a command with none due");
        }
    }

    /// f18 (i) (2.6; NA-8): for every slot of the stress scene the clamp script holds fire with no press edge and stands
    /// still, its point inside the map's extent as the room reads it for kind 6 (the greybox's first area: EA-U4F-4)
    /// and beyond the weapon's reach from anywhere within 1,500 cm of the centre (the spawn grid's corners lie at
    /// 1,414 cm).
    #[test]
    fn f18_the_clamp_scripts_points_lie_beyond_reach_and_inside_the_map() {
        let scene = Scene::stress();
        let far = i64::from(scene.weapon().reach_cm) + 1_500;
        let [x0, y0, x1, y1] = World::new(&scene, 1, FIRST).greybox().areas().rects[0];
        for slot in 0..scene.slots() {
            let mut b = Behaviour::script(&scene, slot, 1, "clamp").expect("a script");
            for _ in 0..4 {
                let c = b.next_command(100, &[], None);
                let inside = (x0..=x1).contains(&c.aim[0]) && (y0..=y1).contains(&c.aim[1]);
                assert!(inside, "{slot}: {:?}", c.aim);
                let [x, y, _] = c.aim.map(i64::from);
                assert!(x * x + y * y > far * far, "{slot}: {:?}", c.aim);
                assert_eq!(
                    (c.buttons, c.presses, c.move_xy),
                    (Command::FIRE, [0; 4], [0, 0])
                );
            }
        }
    }

    /// f19's npt-bots half (F1; TE3-4): at 45 steps to 30 ticks, the fast clock's stamps never fall; every 10th lies 3 +
    /// the bound past the newest tick (past the room's bound on arrival at P0); every other at most the newest + the
    /// bound, one past the last while below it, held equal while the newest stands (kind 2's case).
    #[test]
    fn f19_the_fast_clocks_stamps_never_fall_and_every_tenth_lies_past_the_bound() {
        assert_eq!(fast_stamp(1, None, 1_000, 12), 1_001, "the first");
        assert_eq!(
            fast_stamp(2, Some(1_002), 1_000, 12),
            1_003,
            "below the cap"
        );
        assert_eq!(
            fast_stamp(3, Some(1_012), 1_000, 12),
            1_012,
            "held at the cap"
        );
        let (mut last, mut newest, mut held) = (None, 1_000u32, 0);
        for k in 1..=300u64 {
            newest += u32::from(k % 3 != 0);
            let t = fast_stamp(k, last, newest, 12);
            let floor = last.unwrap_or(0);
            assert!(t >= floor, "{k}: {t} after {last:?}");
            if k % 10 == 0 {
                assert_eq!(t, (newest + 15).max(floor), "{k}");
            } else {
                assert!(t <= (newest + 12).max(floor), "{k}");
            }
            held += u32::from(last == Some(t));
            last = Some(t);
        }
        assert!(held > 50, "equal stamps: {held}");
    }

    /// U4F-Q11 (iii) (c)'s test (+1; A1; advisor-u4f-lead-bound.md E4): the fast clock's lane is collision-free, so the
    /// arithmetic reference (its live ticks at full walking speed) equals an honest client's movement on its inputs.
    /// npt-sim's World from the judged runs' first tick, slot 0 alone driven by the fast clock's walk one command a tick
    /// (an honest client's rate) on its own position 16 steps old (the largest stamp lead plus a snapshot's way); over a
    /// judged fast-clock run's window (a 10 s warm-up to reach the lane, then 610 s) every tick after the warm-up moves it
    /// exactly 20 cm, walking (fire held: no sprint; no crouch), past the static boxes, the doors' and the wall's
    /// schedules and every vehicle part; the other soldiers stand (soldiers never collide with each other).
    #[test]
    fn q11_iii_c_the_fast_clocks_lane_moves_full_walking_speed_every_live_tick() {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = World::new(&scene, 1, FIRST);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let mut b = Behaviour::fast_clock(&scene, 0, 1, 12);
        let (warm, window) = (ticks_for_ms(10_000), ticks_for_ms(610_000));
        let (mut past, mut prev, mut moved) = (VecDeque::new(), None::<[i32; 3]>, 0u32);
        for k in 0..=warm + window {
            let t = FIRST.wrapping_add(k);
            let me = soldiers(&world).into_iter().find(|s| s.0 == 0);
            let (_, p, dead, _) = me.expect("slot 0's record");
            assert!(!dead, "tick {t}");
            if let Some(q) = prev.filter(|_| k > warm) {
                let d2 = |i: usize| i64::from(p[i] - q[i]).pow(2);
                assert_eq!(d2(0) + d2(1) + d2(2), 400, "tick {t}: {q:?} to {p:?}");
                moved += 1;
            }
            prev = Some(p);
            past.push_back(p);
            if past.len() > 16 {
                past.pop_front();
            }
            let mut c = b.next_command(t, &[], Some((past[0], false)));
            assert_eq!(c.buttons & (Command::SPRINT | Command::CROUCH), 0);
            (c.slot, c.tick, c.seq) = (0, t, k as u16);
            let input = TickInput {
                tick: t,
                commands: vec![(c, None)],
                ..TickInput::default()
            };
            world.step(&input, &history);
        }
        assert_eq!(moved, window);
    }

    /// f15's case for one of the room-whole kinds (2.6's per-kind table; TE3-4, SA3-2): the steady script on a fixed-lead
    /// endpoint, a snapshot each step and no acknowledgement back (so every `Input` re-sends its unacknowledged copies),
    /// every 3rd new sequence forged. Each forged `Input` decodes, its acknowledgement and every older copy equal to the
    /// honest datagram's (so each later copy of a forged sequence is honest), and its newest command differs from the
    /// honest one in the kind's field alone, breaking that check's rule (against the honest previous command and the
    /// newest tick); each sequence forged once, and at least 30 forged in 120 steps. Its datagram is the same under
    /// either option (the endpoint reads none).
    fn f15(kind: u8) {
        use npt_proto::{Direction, Input, JoinAccepted, Message, Snapshot, decode, encode};
        let bytes = |m: &Message| {
            let mut o = Vec::new();
            encode(m, &mut o).expect("encodes");
            o
        };
        let input = |d: &[u8]| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
            Ok(Message::Input(i)) => Some(i),
            _ => None,
        };
        let scene = Scene::stress();
        let [x0, y0, x1, y1] = World::new(&scene, 1, FIRST).greybox().areas().rects[0];
        let mut e = ClientEndpoint::new(npt_proto::Join::default(), 30).with_lead(2);
        let mut b = Behaviour::script(&scene, 0, 1, "steady")
            .and_then(|b| b.forger(kind, 3, 12))
            .expect("a room-whole forger");
        let ja = Message::JoinAccepted(JoinAccepted {
            tick: 100,
            ..JoinAccepted::default()
        });
        let (mut rx, mut seqs) = (vec![(Channel::Control, bytes(&ja), 0)], Vec::new());
        let mut last: Option<Command> = None;
        for k in 0..120u32 {
            let now = u64::from(k) * 33_333_333;
            let s = Snapshot {
                tick: 101 + k,
                groups: 1,
                ..Snapshot::default()
            };
            rx.push((Channel::Datagram, bytes(&Message::Snapshot(s)), now));
            b.step(&mut e, &rx, now, &[]);
            rx.clear();
            let honest = e.step().datagrams;
            let mut forged = honest.clone();
            b.outgoing(now, &mut forged);
            assert_eq!(honest.len(), forged.len());
            let newest = e.newest_tick().expect("a snapshot");
            for (h, f) in honest.iter().zip(&forged) {
                let (Some(h), Some(f)): (Option<Input>, Option<Input>) = (input(h), input(f))
                else {
                    assert_eq!(h, f, "only an `Input` changes");
                    continue;
                };
                assert_eq!((h.ack_tick, h.ack_groups), (f.ack_tick, f.ack_groups));
                assert_eq!(
                    h.commands[1..],
                    f.commands[1..],
                    "{kind}: older copies honest"
                );
                let (hc, fc) = (h.commands[0], f.commands[0]);
                let prev = h
                    .commands
                    .get(1)
                    .copied()
                    .or(last.filter(|l| l.seq != hc.seq));
                last = Some(hc);
                if hc == fc {
                    continue;
                }
                assert!(!seqs.contains(&fc.seq), "{kind}: forged once");
                seqs.push(fc.seq);
                let same = |a: Command| (a.slot, a.seq, a.buttons, a.view_tick);
                assert_eq!(same(hc), same(fc), "{kind}");
                let p = prev.expect("a previous command");
                assert_eq!(p.seq, hc.seq.wrapping_sub(1), "{kind}: the sequence before");
                let (x, y) = (i32::from(fc.move_xy[0]), i32::from(fc.move_xy[1]));
                let fire = Command::PRESS_FIRE;
                // Kind 6 against the extent the room reads (EA-U4F-4), inside the grid's ±32,767.
                let inside = (x0..=x1).contains(&fc.aim[0]) && (y0..=y1).contains(&fc.aim[1]);
                let broken = match kind {
                    1 => (15..=114).contains(&fc.tick.wrapping_sub(newest)),
                    2 => fc.tick == p.tick,
                    4 => x * x + y * y > 127 * 127,
                    5 => fc.presses[fire].wrapping_sub(p.presses[fire]) > (fc.tick - p.tick) as u8,
                    _ => !inside && fc.aim[..2].iter().all(|a| a.abs() <= 32_767),
                };
                assert!(broken, "{kind}: {fc:?} after {p:?}");
                let (mut back, mut presses) = (fc, hc.presses);
                presses[fire] = fc.presses[fire];
                assert_eq!(presses, fc.presses, "{kind}: only fire's counter");
                (back.tick, back.move_xy, back.presses, back.aim) =
                    (hc.tick, hc.move_xy, hc.presses, hc.aim);
                assert_eq!(back, hc, "{kind}");
                let changed = [
                    hc.tick != fc.tick,
                    hc.move_xy != fc.move_xy,
                    hc.presses != fc.presses,
                    hc.aim != fc.aim,
                ];
                let want = match kind {
                    1 | 2 => 0,
                    4 => 1,
                    5 => 2,
                    _ => 3,
                };
                assert_eq!(
                    changed.iter().filter(|c| **c).count(),
                    1,
                    "{kind}: one field"
                );
                assert!(changed[want], "{kind}: its field");
            }
        }
        assert_eq!(b.take_forged(), seqs);
        assert!(seqs.len() >= 30, "{kind}: {} forged", seqs.len());
    }

    #[test]
    fn f15_the_kind_1_forger_breaks_only_the_lead_bound() {
        f15(1);
    }

    #[test]
    fn f15_the_kind_2_forger_breaks_only_the_newer_tick_rule() {
        f15(2);
    }

    #[test]
    fn f15_the_kind_4_forger_breaks_only_the_move_length() {
        f15(4);
    }

    #[test]
    fn f15_the_kind_5_forger_breaks_only_the_press_edge_rule() {
        f15(5);
    }

    #[test]
    fn f15_the_kind_6_forger_breaks_only_the_aim_bounds() {
        f15(6);
    }

    /// SEC-U4F-1: each room-whole forger refuses an interval below 2 (kinds 2 and 5 forge against the previous command,
    /// which must have been accepted) and takes 2 (kind 7 its rest: the fp3 test).
    #[test]
    fn f15_a_forgers_interval_below_2_is_refused() {
        let scene = Scene::stress();
        let steady = || Behaviour::script(&scene, 0, 1, "steady").expect("a script");
        for k in 1..=11 {
            assert!(steady().forger(k, 1, 12).is_none(), "{k}");
            assert!(k == 7 || steady().forger(k, 2, 12).is_some(), "{k}");
            let over = steady().forger(k, u64::MAX / 8 + 1, 12);
            assert!(over.is_none(), "{k}: x 8 overflows");
        }
        assert!(steady().forger(12, 2, 12).is_none(), "no kind 12");
    }

    /// FP-3 (NC-U4F2-2 = RR-U4F2-1, SA-U4F2-2): kind 7's rest from the weapon and the rate, SS's pinned at 30 and 60
    /// Hz (3 and 5) and `None` under two ticks; kind 7's forger takes an interval only above its rest (F-G: 2 and 3
    /// refused at 30 Hz, 4 taken), and refuses any interval on a weapon with none.
    #[test]
    fn fp3_kind_7s_rest_comes_from_the_weapon_and_the_rate() {
        let scene = Scene::stress();
        let ss = scene.weapon().interval_us;
        assert_eq!([30, 60].map(|hz| fire_rest_at(hz, ss)), [Some(3), Some(5)]);
        let two_ticks = [66_666, 66_667].map(|i| fire_rest_at(30, i));
        assert_eq!(two_ticks, [None, Some(3)], "1,999,980 and 2,000,010 units");
        let rest = fire_rest_at(npt_sim::TICK_HZ, ss).expect("SS's rest");
        let steady = || Behaviour::script(&scene, 6, 1, "steady").expect("a script");
        let taken = [rest - 1, rest, rest + 1].map(|e| steady().forger(7, e, 12).is_some());
        assert_eq!(taken, [false, false, true], "2 and 3 refused at 30 Hz");
        let mut none = steady();
        none.fire_rest = None;
        assert!(none.forger(7, 1_000, 12).is_none(), "no rest");
    }

    /// One side of f15's run for kinds 3, 7, 8, 9, 10 and 11: a forger or its twin, its endpoint, the newest
    /// sequence it sent (the acknowledgement fed back) and, per step, its endpoint's datagrams before and after
    /// `outgoing` and its reliable messages.
    struct Side {
        b: Behaviour,
        e: ClientEndpoint,
        sent: Option<u16>,
        steps: Vec<[Vec<Vec<u8>>; 3]>,
    }

    /// The steady script on a fixed-lead endpoint, forging kind `kind` every 3rd command (kind 10 under option A when
    /// `a`), and its twin, fed the same snapshot ticks for 300 steps, each acknowledging the newest command its side
    /// sent (as at P0; for kind 3 every 5th acknowledges nothing, so some `Input`s carry the copy already).
    fn sides(kind: u8, a: bool) -> [Side; 2] {
        sides_at(kind, a, 3, 300)
    }

    /// [`sides`] at the interval `every` for `steps` steps.
    fn sides_at(kind: u8, a: bool, every: u64, steps: u32) -> [Side; 2] {
        use npt_proto::{JoinAccepted, Message, Snapshot, encode};
        let bytes = |m: &Message| {
            let mut o = Vec::new();
            encode(m, &mut o).expect("encodes");
            o
        };
        let scene = Scene::stress();
        let mut sides = [false, true].map(|twin| {
            let b = Behaviour::script(&scene, 0, 1, "steady")
                .and_then(|b| b.forger(kind, every, 12))
                .expect("a forger");
            let b = if a { b.catchup_under_a() } else { b };
            Side {
                b: if twin { b.twin() } else { b },
                e: ClientEndpoint::new(npt_proto::Join::default(), 30).with_lead(2),
                sent: None,
                steps: Vec::new(),
            }
        });
        let ja = bytes(&Message::JoinAccepted(JoinAccepted {
            tick: 100,
            ..JoinAccepted::default()
        }));
        for k in 0..steps {
            let now = u64::from(k) * 33_333_333;
            for s in &mut sides {
                let snap = Snapshot {
                    tick: 101 + k,
                    groups: 1,
                    newest_received: s.sent.filter(|_| kind != 3 || k % 5 != 4),
                    ..Snapshot::default()
                };
                let mut rx = vec![(Channel::Datagram, bytes(&Message::Snapshot(snap)), now)];
                if k == 0 {
                    rx.insert(0, (Channel::Control, ja.clone(), 0));
                }
                s.b.step(&mut s.e, &rx, now, &[]);
                let out = s.e.step();
                let mut sent = out.datagrams.clone();
                s.b.outgoing(now, &mut sent);
                let mut reliable = Vec::new();
                s.b.reliable(&mut reliable);
                for i in sent.iter().filter_map(|d| decoded(d).ok()) {
                    s.sent = i.commands.first().map(|c| c.seq).or(s.sent);
                }
                let reliable = reliable.into_iter().map(|r| r.1).collect();
                s.steps.push([out.datagrams, sent, reliable]);
            }
        }
        for s in &sides {
            assert_eq!(
                (s.b.forge_unencodable(), s.e.unencodable_commands()),
                (0, 0)
            );
            assert_eq!(s.e.twin_left_out(), 0, "the twin left out no command");
        }
        sides
    }

    /// One datagram as an `Input`.
    fn decoded(d: &[u8]) -> Result<npt_proto::Input, npt_proto::DecodeError> {
        use npt_proto::{Direction, Message, decode};
        match decode(Direction::ClientToRoom, Channel::Datagram, d)? {
            Message::Input(i) => Ok(i),
            _ => Err(npt_proto::DecodeError::UnknownType),
        }
    }

    /// The `Input`s of one step's datagrams.
    fn inputs(ds: &[Vec<u8>]) -> Vec<npt_proto::Input> {
        ds.iter().filter_map(|d| decoded(d).ok()).collect()
    }

    /// f15's case for kinds 3, 8, 9 and 11 (2.6's per-kind table; TE3-4, SA3-2): each forged `Input` differs from the
    /// honest one in the kind's part alone (kind 3: the previous command's copy, its move or aim changed, in place or
    /// added; kind 8: the acknowledgement's tick, never sent; kind 9: a trailing byte or a set pad bit in turn, so it
    /// does not decode; kind 11: its datagrams unchanged, a `Ping` on the control stream refused as the wrong channel),
    /// at least 30 forged in 300 steps, each sequence once, and the twin's datagram in its slot by 2.12's row (kind 3:
    /// the copy honest; kinds 8 and 9: the header only, the previous `Input`'s acknowledgement; kind 11: a
    /// `ClientSettings` repeating the delay).
    fn f15_datagram(kind: u8) {
        use npt_proto::{ClientSettings, DecodeError, Direction, Message, decode};
        let [mut f, t] = sides(kind, false);
        let (mut seen, mut ways, mut prev_ack) = (Vec::new(), [0u32; 2], (0u16, 0u8));
        for (k, ([honest, forged, msg], [_, twin, tmsg])) in
            f.steps.iter().zip(&t.steps).enumerate()
        {
            let (h, tw) = (inputs(honest), inputs(twin));
            if kind == 11 {
                assert_eq!(honest, forged, "{k}");
                assert_eq!(msg.len(), tmsg.len(), "{k}: the twin's message in its slot");
                for (m, tm) in msg.iter().zip(tmsg) {
                    let ctl = |b: &[u8]| decode(Direction::ClientToRoom, Channel::Control, b);
                    assert_eq!(ctl(m), Err(DecodeError::WrongChannel), "{k}");
                    let ping = decode(Direction::ClientToRoom, Channel::Datagram, &m[2..]);
                    assert!(matches!(ping, Ok(Message::Ping(_))), "{k}: {ping:?}");
                    let want = Ok(Message::ClientSettings(ClientSettings { delay: 0 }));
                    assert_eq!(ctl(tm), want, "{k}");
                    seen.push(h[0].commands[0].seq);
                }
                continue;
            }
            for (j, (hd, fd)) in honest.iter().zip(forged).enumerate() {
                let hi = decoded(hd).expect("the honest Input");
                let ack = core::mem::replace(&mut prev_ack, (hi.ack_tick, hi.ack_groups));
                if hd == fd {
                    continue;
                }
                let c = hi.commands[0];
                match kind {
                    3 => {
                        let fi = decoded(fd).expect("a forged copy encodes");
                        assert_eq!(fi.commands[0], c, "{k}: the new command honest");
                        let (x, y) = (fi.commands[1], tw[j].commands[1]);
                        assert_eq!((x.seq, x.tick), (y.seq, y.tick), "{k}");
                        assert_eq!(x.seq, c.seq.wrapping_sub(1), "{k}: the copy one below");
                        let moved = [x.move_xy != y.move_xy, x.aim != y.aim];
                        assert_eq!(moved.iter().filter(|m| **m).count(), 1, "{k}: {x:?} {y:?}");
                        let (mut back, mut honest) = (x, y);
                        (back.move_xy, back.aim, honest.move_xy, honest.aim) =
                            ([0; 2], [0; 3], [0; 2], [0; 3]);
                        assert_eq!(back, honest, "{k}: only its move or aim");
                        assert_eq!(fi.commands[2..], hi.commands[2.min(hi.commands.len())..]);
                        ways[usize::from(hi.commands.len() > 1)] += 1;
                        seen.push(x.seq);
                    }
                    8 => {
                        let fi = decoded(fd).expect("a forged ack encodes");
                        assert_eq!(fi.ack_tick, hi.ack_tick.wrapping_add(1_000), "{k}");
                        assert_eq!((fi.ack_groups, &fi.commands), (hi.ack_groups, &hi.commands));
                        seen.push(c.seq);
                    }
                    _ => {
                        let way = usize::from(decoded(fd) == Err(DecodeError::UnknownField));
                        ways[way] += 1;
                        if way == 0 {
                            assert_eq!(
                                (decoded(fd), &fd[..hd.len()]),
                                (Err(DecodeError::TrailingBytes), &hd[..])
                            );
                            assert_eq!(fd.len(), hd.len() + 1, "{k}: one trailing byte");
                        } else {
                            let diff: Vec<u8> = hd.iter().zip(fd).map(|(a, b)| a ^ b).collect();
                            let mut want = vec![0u8; hd.len()];
                            want[PAD_BYTE] = 0x80;
                            assert_eq!(diff, want, "{k}: the newest command's top pad bit");
                        }
                        assert!(ways[0].abs_diff(ways[1]) <= 1, "in turn");
                        seen.push(c.seq);
                    }
                }
                if kind != 3 {
                    let ti = &tw[j];
                    assert!(ti.commands.is_empty(), "{k}: the twin's header only");
                    assert_eq!((ti.ack_tick, ti.ack_groups), ack, "{k}: the previous ack");
                }
            }
        }
        assert_eq!(
            f.b.take_forged(),
            seen,
            "{kind}: each forged once, in order"
        );
        if kind == 3 {
            assert_eq!(f.b.take_copies().len(), seen.len());
            assert!(
                ways[0] > 0 && ways[1] > 0,
                "kind 3: added {} and in place {}",
                ways[0],
                ways[1]
            );
        }
        assert!(seen.len() >= 30, "{kind}: {} forged", seen.len());
    }

    #[test]
    fn f15_the_kind_3_forger_breaks_only_the_sequence_forward_rule() {
        f15_datagram(3);
    }

    #[test]
    fn f15_the_kind_8_forger_acknowledges_only_a_tick_never_sent() {
        f15_datagram(8);
    }

    #[test]
    fn f15_the_kind_9_forger_sends_trailing_bytes_and_a_set_pad_bit_in_turn() {
        f15_datagram(9);
    }

    #[test]
    fn f15_the_kind_11_forger_sends_a_known_type_on_the_wrong_channel() {
        f15_datagram(11);
    }

    /// f15's kind-7 case (SA-15; 2.6's row: the endpoint stores the forged command), at the rest + 1 (F-G: 4 at 30 Hz):
    /// its datagrams unchanged; each forged command carries one edge against the command before it (inside kind 5's
    /// rule), on the item's button (reload every [`RELOAD_ITEMS`]-th item, at index 2, else fire), right after an
    /// honest edge of the same button; a fire item's rest before it holds no fire (SS's: 3 at 30 Hz), nor a reload
    /// item's one command before it (FP-1); the twin gives the same commands with only the forged edges taken off (its
    /// counters one lower from each on: carry 46).
    #[test]
    fn f15_the_kind_7_forger_presses_where_the_sim_refuses() {
        let rest = fire_rest_at(npt_sim::TICK_HZ, Scene::stress().weapon().interval_us);
        let rest = rest.expect("SS's rest");
        let [mut f, t] = sides_at(7, false, rest + 1, 300);
        let forged = f.b.take_forged();
        let (mut cmds, mut tcmds) = (Vec::new(), Vec::new());
        for ([honest, sent, _], [_, tsent, _]) in f.steps.iter().zip(&t.steps) {
            assert_eq!(honest, sent, "the datagrams unchanged");
            let new = |ds: &[Vec<u8>], v: &mut Vec<Command>| {
                for c in inputs(ds).iter().flat_map(|i| i.commands.iter().rev()) {
                    if v.last().is_none_or(|l: &Command| c.seq > l.seq) {
                        v.push(*c);
                    }
                }
            };
            new(sent, &mut cmds);
            new(tsent, &mut tcmds);
        }
        assert_eq!(cmds.len(), tcmds.len(), "one command a step on both");
        let (fire, reload) = (Command::PRESS_FIRE, Command::PRESS_RELOAD);
        let (mut taken, mut items) = ([0u8; 2], 0u64);
        let edges = |a: &Command, b: &Command| {
            [fire, reload].map(|x| b.presses[x].wrapping_sub(a.presses[x]))
        };
        for (n, w) in cmds.windows(2).enumerate().map(|(i, w)| (i + 1, w)) {
            let e = edges(&w[0], &w[1]);
            assert!(e[0] + e[1] <= 1, "{n}: one edge at most");
            if forged.contains(&w[1].seq) {
                items += 1;
                let b = usize::from(items % RELOAD_ITEMS == 2);
                assert_eq!(e[b], 1, "{n}: item {items}'s button");
                assert_eq!(
                    edges(&cmds[n - 2], &w[0])[b],
                    1,
                    "{n}: an honest edge before"
                );
                let before = if b == 0 { rest as usize } else { 1 };
                let rested = cmds[n.saturating_sub(before)..n]
                    .iter()
                    .all(|c| c.buttons & Command::FIRE == 0);
                assert!(rested, "{n}: {before} rested before item {items}");
                taken[b] += 1;
            }
            let mut twin = w[1];
            twin.presses[fire] = twin.presses[fire].wrapping_sub(taken[0]);
            twin.presses[reload] = twin.presses[reload].wrapping_sub(taken[1]);
            assert_eq!(tcmds[n], twin, "{n}: the twin's command");
        }
        assert_eq!(items, forged.len() as u64);
        assert!(taken[0] >= 30 && taken[1] >= 2, "{taken:?}");
    }

    /// T-K7 (FP-1; TE-U4F2-1's proof of NC-U4F2-1): a kind-7 forger and its twin at the judged interval
    /// ([`sides_at`], every 12th command, 640 steps), each run in a World of SS, with the same killers and placements
    /// in both (at an interval of 3, the rest, every command is a rest, so an item d holds no fire and, after a revive
    /// at d, its forged edge fires a shot the twin's d does not). The forger's soldier (slot 6, the judged runs' kind-7
    /// slot) is teleported to an open point (35 m out: past the fight vehicles' parts, inside the outer loops' and 13
    /// m from the nearest building), the 47 others onto a ring 8 m round it, holding fire at its chest (radial rays,
    /// 107 cm apart at the ring). Kill 1: the forger stands idle until it dies, then its script starts so that item 2's
    /// command before (a reload item's d - 1) is the first it runs after the sim's own respawn (`ticks_for_ms(1_000)`
    /// later, SS's respawn: npt-sim scene.rs:186); its earlier commands run dead. It runs alive past item 42 (a reload
    /// item, alive); kill 2 teleports it back and kills it again, and a spawn the test picks revives it at a fire
    /// item's d, at least 10 ticks after the death, so that item's d - 1 ran dead. Asserts: every item whose d - 1 and
    /// d ran alive carries its own record (fire: Cooldown, reload: Reload) under its sequence; every tick's state hash
    /// equal between the two Worlds; the set-up reached each case. FP-2: the items `kind_7_left_out` leaves out (d or
    /// d - 1 run dead) are counted apart and printed, and every other item carries its record (unrejected 0). Its
    /// plants: fire held on a reload item's d - 1 (the shape before FP-1); the window ignored (FP-2).
    #[test]
    fn t_k7_kind_7_items_next_to_a_death_and_a_revive_through_the_world() {
        use npt_sim::RefusalKind;
        const SLOT: u8 = 6;
        const C: [i32; 3] = [3_000, -1_800, 0];
        let [mut f, t] = sides_at(7, false, 12, 640);
        let forged = f.b.take_forged();
        let commands = |s: &Side| {
            let mut v: Vec<Command> = Vec::new();
            let all = s.steps.iter().flat_map(|x| inputs(&x[1]));
            for c in all.flat_map(|i| i.commands.into_iter().rev()) {
                if v.last().is_none_or(|l| c.seq > l.seq) {
                    v.push(c);
                }
            }
            v
        };
        let (fc, tc) = (commands(&f), commands(&t));
        assert_eq!(fc.len(), tc.len(), "one command a step on both");
        // Each item (from 1): its command's index, and whether it is a reload item.
        let index: BTreeMap<u16, usize> = (0..).zip(&fc).map(|(j, c)| (c.seq, j)).collect();
        let items: Vec<(usize, bool)> = (1..)
            .zip(&forged)
            .map(|(i, s)| (index[s], i % RELOAD_ITEMS == 2))
            .collect();
        assert!(items[1].1 && items[41].1 && !items[43].1);
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut worlds = [World::new(&scene, 1, FIRST), World::new(&scene, 1, FIRST)];
        let history = HitHistory::for_ms(2_000).expect("a ring");
        // A soldier killed at tick D runs its next commands dead and the one at D + this alive.
        let respawn = ticks_for_ms(1_000) as usize;
        assert!(respawn > items[1].0);
        let others = f64::from(scene.slots() - 1);
        let ring: Vec<Placement> = (0..scene.slots())
            .filter(|s| *s != SLOT)
            .enumerate()
            .map(|(i, slot)| {
                let a = core::f64::consts::TAU * i as f64 / others;
                let at = |c: i32, v: f64| c + (800.0 * v).round() as i32;
                Placement {
                    slot,
                    position: [at(C[0], a.cos()), at(C[1], a.sin()), 0],
                }
            })
            .collect();
        let here = Placement {
            slot: SLOT,
            position: C,
        };
        let kill_2 = items[43].0 + 1;
        // The killers firing; the script's first tick; the spawn's tick; the last command run.
        let (mut firing, mut start, mut spawn, mut last) = (true, None, None, None);
        let (mut dead, mut kills, mut first_diff) = (false, Vec::new(), None);
        let mut ran_dead: Vec<Option<bool>> = vec![None; fc.len()];
        let mut records: BTreeMap<u16, Vec<RefusalKind>> = BTreeMap::new();
        for k in 0..fc.len() + 400 {
            let j = start.and_then(|s| k.checked_sub(s));
            if j.is_some_and(|j| j >= fc.len() || last.is_some_and(|l| j > l)) {
                break;
            }
            let tick = FIRST.wrapping_add(k as u32);
            let mut input = TickInput {
                tick,
                ..TickInput::default()
            };
            if k == 0 {
                input.teleports = ring.clone();
            }
            if k == 0 || j == Some(kill_2) {
                input.teleports.push(here);
                firing = true;
            }
            if spawn == Some(k) {
                input.spawns.push(here);
            }
            if let Some(j) = j {
                ran_dead[j] = Some(dead && spawn != Some(k));
            }
            let mut both = [Vec::new(), Vec::new()];
            for slot in 0..scene.slots() {
                for (out, side) in both.iter_mut().zip([&fc, &tc]) {
                    if let Some(j) = j.filter(|_| slot == SLOT) {
                        let mut c = side[j];
                        (c.slot, c.tick) = (slot, tick);
                        out.push((c, None));
                    } else if firing && slot != SLOT {
                        let c = Command {
                            slot,
                            tick,
                            seq: k as u16,
                            buttons: Command::FIRE,
                            aim: [C[0], C[1], 140],
                            view_tick: tick,
                            ..Command::default()
                        };
                        out.push((c, None));
                    }
                }
            }
            for (w, commands) in worlds.iter_mut().zip(both) {
                w.step(
                    &TickInput {
                        commands,
                        ..input.clone()
                    },
                    &history,
                );
            }
            let [w, tw] = &worlds;
            if first_diff.is_none() && w.state_hash() != tw.state_hash() {
                first_diff = Some(k);
            }
            for r in w.reports().refusals.iter().filter(|r| r.slot == SLOT) {
                records.entry(r.seq).or_default().push(r.kind);
            }
            dead = soldiers(w).iter().any(|x| x.0 == SLOT && x.2);
            if firing && dead {
                firing = false;
                kills.push(k);
                match start {
                    None => start = Some(k + respawn + 1 - items[1].0),
                    Some(s) => {
                        let (jd, _) = *items
                            .iter()
                            .find(|&&(jd, r)| !r && s + jd > k + 10)
                            .expect("a fire item left");
                        spawn = Some(s + jd);
                        last = Some(jd + 10);
                    }
                }
            }
        }
        assert_eq!(kills.len(), 2, "two kills: {kills:?}");
        // The cases: both runs alive; d dead; a fire item's d - 1 dead and d alive; a reload item's d - 1 the first
        // live command; a reload item with d - 2 to d alive.
        let (mut cases, mut missing, mut left_out) = ([0u32; 5], Vec::new(), Vec::new());
        for &(j, reload) in &items {
            let Some(at_d) = ran_dead[j] else {
                continue;
            };
            let before = ran_dead[j - 1];
            let got = records.get(&fc[j].seq).cloned().unwrap_or_default();
            let want = if reload {
                RefusalKind::Reload
            } else {
                RefusalKind::Cooldown
            };
            // FP-2: the attribution's rule; every item outside the dead window must carry its record.
            if !kind_7_left_out(at_d, before) {
                cases[0] += 1;
                if !got.contains(&want) {
                    missing.push((j + 1, fc[j].seq, reload, got.clone()));
                }
                if reload {
                    cases[3 + usize::from(ran_dead[j - 2] == Some(false))] += 1;
                }
            } else {
                cases[if at_d { 1 } else { 2 }] += u32::from(at_d || !reload);
                left_out.push((j + 1, fc[j].seq, reload, before, at_d, got));
            }
        }
        println!(
            "t-k7: kills at ticks {kills:?}, script from tick {start:?}, spawn at tick {spawn:?}; cases (alive, d \
             dead, fire d - 1 dead, reload after a revive, reload alive) {cases:?}; dead_left_out {} (FP-2: n, seq, \
             reload, d - 1 dead, d dead, records): {left_out:?}",
            left_out.len()
        );
        assert_eq!(
            (missing, first_diff),
            (Vec::new(), None),
            "kind-7 items outside the dead window (FP-2) without their record, so unrejected (n, seq, reload, \
             records), and the first tick whose state hash differs between the forger's World and the twin's"
        );
        assert!(
            cases[0] >= 40 && cases[1..].iter().all(|c| *c >= 1),
            "the set-up reached every case: {cases:?}"
        );
    }

    /// f15's kind-10 case under both options (2.6's row; SA3-8, SA31-4 (c)): each episode sends two `Input`s in one
    /// step, the first 8 new commands, the second 8 more new (A) or the first's last 4 again and 4 new (B), stamped
    /// one apart on from the forger's last accepted stamp, every view tick at most its own tick, each `Input` encoding;
    /// the forged sequences are the second's new ones; [`HOLD_STEPS`] steps with no new command come before (A) or
    /// after (B) the burst; the twin sends the first `Input` and then the same commands again (repeats only), and its
    /// later commands keep the forger's sequences.
    #[test]
    fn f15_the_kind_10_forger_sends_more_commands_in_one_tick_than_the_cap() {
        for a in [true, false] {
            let [mut f, t] = sides(10, a);
            let forged = f.b.take_forged();
            let (mut bursts, mut quiet, mut want) = (0, 0u32, Vec::new());
            let (mut quiet_runs, mut newest) = (Vec::new(), None::<u16>);
            for ([_, sent, _], [_, tsent, _]) in f.steps.iter().zip(&t.steps) {
                let (fi, ti) = (inputs(sent), inputs(tsent));
                let s: Vec<&Command> = fi.iter().flat_map(|i| i.commands.iter()).collect();
                assert!(s.iter().all(|c| later(c.view_tick, c.tick) == c.tick));
                let top = s.iter().map(|c| c.seq).max();
                if top.is_some_and(|x| newest.is_none_or(|n| x > n)) {
                    quiet_runs.push(core::mem::take(&mut quiet));
                    newest = top;
                } else {
                    quiet += 1;
                }
                if fi.len() < 2 {
                    continue;
                }
                bursts += 1;
                let (one, two) = (&fi[0].commands, &fi[1].commands);
                assert_eq!(one.len(), 8, "a: {a}");
                let new = if a { 8 } else { 4 };
                assert_eq!(
                    &two[two.len() - (8 - new)..],
                    &one[..8 - new],
                    "a: {a}: the repeats"
                );
                let all: Vec<&Command> = two[..new].iter().chain(one).collect();
                for p in all.windows(2) {
                    assert_eq!(
                        (p[0].seq.wrapping_sub(1), p[0].tick - 1),
                        (p[1].seq, p[1].tick)
                    );
                }
                want.extend(two[..new].iter().rev().map(|c| c.seq));
                assert_eq!(ti.len(), 2, "the twin's two in that step");
                assert_eq!(&ti[0].commands, one, "the twin's first: the accepted ones");
                assert_eq!(&ti[1].commands, one, "the twin's second: repeats only");
            }
            assert_eq!(forged, want, "a: {a}");
            assert!(bursts >= 2, "a: {a}: {bursts}");
            quiet_runs.push(quiet);
            let held = quiet_runs.iter().filter(|q| **q >= HOLD_STEPS).count();
            assert!(held >= bursts, "a: {a}: {quiet_runs:?}");
            let seqs = |s: &Side| -> BTreeMap<u16, ()> {
                let all = s.steps.iter().flat_map(|x| inputs(&x[1]));
                all.flat_map(|i| i.commands).map(|c| (c.seq, ())).collect()
            };
            let mut kept = seqs(&f);
            kept.retain(|s, _| !forged.contains(s));
            assert_eq!(
                kept,
                seqs(&t),
                "a: {a}: the twin sends the same sequences but the refused"
            );
        }
    }

    /// FP-5 (NC-U4F2-4, TE-U4F2-2): U4F-Q13 (b)'s twin of the room-whole kinds 1, 2, 4, 5 and 6 never makes a
    /// refused command: it sends no forged sequence, and its commands are the forger's honest ones less the forged
    /// (the kind-10 case's pattern), at least 30 forged in 300 steps.
    #[test]
    fn fp5_the_room_whole_kinds_twin_never_makes_the_refused_command() {
        for kind in [1, 2, 4, 5, 6] {
            let [mut f, t] = sides(kind, false);
            let forged = f.b.take_forged();
            let by_seq = |s: &Side, k: usize| -> BTreeMap<u16, Command> {
                let all = s.steps.iter().flat_map(|x| inputs(&x[k]));
                all.flat_map(|i| i.commands).map(|c| (c.seq, c)).collect()
            };
            let mut kept = by_seq(&f, 0);
            kept.retain(|s, _| !forged.contains(s));
            assert!(forged.len() >= 30, "{kind}: {} forged", forged.len());
            assert_eq!(kept, by_seq(&t, 1), "{kind}: the twin's commands");
        }
    }

    /// The judged runs' first tick (the run scripts' `--tick-start 63736`).
    const FIRST: u32 = 63_736;
    /// The feed's delay, in World steps: each slot decides on its soldier as it was this many steps before, the
    /// largest lead a run takes (`--lead`'s cap, 8), as a step count with no time in it.
    const DELAY: usize = 8;

    /// How the sim-only test feeds the World (each a slot's command per tick).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Pattern {
        /// (i), barred: a command every tick.
        EveryTick,
        /// (ii), barred: a seeded skip pattern; a skipped tick re-runs the slot's last command unchanged, as the room's
        /// `InputBuffer::take` does. Per slot, each tick that is not skipped or right after a streak starts a streak
        /// with chance 1/8, of 1 to 6 ticks (uniform): about 30 % of ticks skipped, the longest streak 6 (the run
        /// prints both as measured).
        Skips,
        /// (iii), printed only: B1's bursts. Every slot re-runs its last command for `ticks_for_ms(1_000)` ticks at
        /// ten burst starts, the first 30 s into the measured phase (after the 10 s warm-up) and then every 60 s (the
        /// run scripts' B1 settings).
        Bursts,
        /// (iv), barred (C-U4F2-5, U4F2-Q1 point 3): option A at P1. Each slot gives one command per client step (a
        /// seeded phase in the tick), stamped P1's lead ahead (npt-lab's `lead_for`: its 50 ms RTT and 10 ms jitter in
        /// ticks, plus 1); each arrives P1's one-way 25 ms ± 5 ms (uniform, seeded per command: l1/src/lib.rs:182-184)
        /// later and runs at the first tick at or after its arrival, so a slot runs two commands in one tick and none
        /// in another (no re-run on an empty tick).
        OptionA,
    }

    /// One run's record: soldier-ticks in a building and the first; the largest distance from the centre (its square in
    /// cm², slot, tick, x, y); skipped ticks, chances and the longest streak; the soldier-ticks seen dead; each soldier
    /// named as stuck (slot, tick, x, y).
    #[derive(Debug, Default)]
    struct Record {
        in_building: u64,
        first_in: Option<(u32, u8, u16)>,
        far: (i64, u8, u32, i32, i32),
        skipped: u64,
        chances: u64,
        streak: u32,
        dead: u64,
        stuck: Vec<(u8, u32, i32, i32)>,
    }

    /// One Soldier record as the test reads it: (slot, grid position, dead bit, area).
    type Seen = (u8, [i32; 3], bool, u16);

    /// Each Soldier record (the dead bit: flags, the layout's sixth field, bit 3: npt-sim world.rs:92, :120), with the
    /// dead bit tied to health (the seventh field) on every record: npt-sim sets both at a death and clears both at a
    /// revive (world.rs:1630-1634, :1176-1188). This pins the literals npt-client copies (client lib.rs:250-251,
    /// SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.
    fn soldiers(world: &World) -> Vec<Seen> {
        SimCore::entity_records(world)
            .iter()
            .filter(|r| r.kind == EntityKind::Soldier)
            .map(|r| {
                let dead = r.grid[5] & (1 << 3) != 0;
                assert_eq!(
                    dead,
                    r.grid[6] == 0,
                    "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0",
                    r.index
                );
                (
                    r.index as u8,
                    [r.grid[0], r.grid[1], r.grid[2]],
                    dead,
                    r.area,
                )
            })
            .collect()
    }

    /// The sim-only test's one run (no network): npt-sim's World from the judged runs' first tick, SS's 48 soldiers
    /// on `script` (BRAWL-1's brawl; C-U4F2-5's steady: U4F2-Q1 point 3, the script as its parameter) driven through
    /// the decision function for `ms` of ticks under `pattern`. The position fed in is the World's own record, `DELAY`
    /// steps old, standing in for the decoded position (with no network they are equal: this tests the steering, not
    /// the decoding). It records every Soldier record with `area != 0`, the largest distance from the centre, and
    /// names a soldier whose position stays inside a 100 cm square for more than 300 commands given.
    fn sim_script(
        seed: u64,
        ms: u32,
        pattern: Pattern,
        script: fn(&Scene, u8, u64) -> Behaviour,
    ) -> Record {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = World::new(&scene, seed, FIRST);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let n = usize::from(scene.slots());
        let mut bots: Vec<Behaviour> = (0..scene.slots())
            .map(|s| script(&scene, s, seed))
            .collect();
        // The option-A pattern's client steps and arrivals (ns from the run's first tick), and each slot's pending
        // commands with the tick index each runs at.
        let period = 1_000_000_000 / u64::from(npt_sim::TICK_HZ);
        let mut arrive = seed ^ 0x6172_7269_7665_0000;
        let phases: Vec<u64> = (0..n).map(|_| below(next(&mut arrive), period)).collect();
        let mut pending: Vec<VecDeque<(u64, Command)>> = vec![VecDeque::new(); n];
        let lead = ticks_for_ms(50 + 10).max(1) + 1;
        let mut last: Vec<Option<Command>> = vec![None; n];
        let mut seq = vec![0u16; n];
        let mut rng = seed ^ 0x736b_6970_7300_0000;
        let (mut left, mut rest, mut run) = (vec![0u32; n], vec![false; n], vec![0u32; n]);
        // Per slot: the square's corners, the commands given inside it, and whether it was named.
        let mut square = vec![([0i32; 2], [0i32; 2], 0u64, false); n];
        let bursts: Vec<(u32, u32)> = (0..10)
            .map(|i| {
                let s = ticks_for_ms(10_000 + 30_000 + 60_000 * i);
                (s, s + ticks_for_ms(1_000))
            })
            .collect();
        let mut past: VecDeque<Vec<Seen>> = VecDeque::new();
        let mut r = Record::default();
        for k in 0..ticks_for_ms(ms) {
            let t = FIRST.wrapping_add(k);
            let now = soldiers(&world);
            assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");
            for &(slot, p, dead, area) in &now {
                let s = usize::from(slot);
                if dead {
                    r.dead += 1;
                }
                if area != 0 {
                    r.in_building += 1;
                    r.first_in.get_or_insert((t, slot, area));
                }
                let d2 = i64::from(p[0]).pow(2) + i64::from(p[1]).pow(2);
                if d2 > r.far.0 {
                    r.far = (d2, slot, t, p[0], p[1]);
                }
                let (lo, hi, given, named) = &mut square[s];
                let (nlo, nhi) = (
                    [lo[0].min(p[0]), lo[1].min(p[1])],
                    [hi[0].max(p[0]), hi[1].max(p[1])],
                );
                if *given == 0 || nhi[0] - nlo[0] > 100 || nhi[1] - nlo[1] > 100 {
                    square[s] = ([p[0], p[1]], [p[0], p[1]], 0, false);
                } else {
                    (*lo, *hi) = (nlo, nhi);
                    if *given > 300 && !*named {
                        *named = true;
                        r.stuck.push((slot, t, p[0], p[1]));
                    }
                }
            }
            past.push_back(now);
            if past.len() > DELAY + 1 {
                past.pop_front();
            }
            let fed = &past[0];
            let view: Vec<(u8, [i32; 3])> = fed.iter().map(|x| (x.0, x.1)).collect();
            let burst = bursts.iter().any(|&(a, b)| (a..b).contains(&k));
            let mut commands = Vec::with_capacity(n);
            for s in 0..n {
                if pattern == Pattern::OptionA {
                    let own = fed
                        .iter()
                        .find(|x| usize::from(x.0) == s)
                        .map(|x| (x.1, x.2));
                    let mut c = bots[s].next_command(t.wrapping_sub(DELAY as u32), &view, own);
                    (c.slot, c.tick, c.seq) = (s as u8, t.wrapping_add(lead), seq[s]);
                    seq[s] = seq[s].wrapping_add(1);
                    square[s].2 += 1;
                    let at = u64::from(k) * period
                        + phases[s]
                        + 20_000_000
                        + below(next(&mut arrive), 10_000_001);
                    pending[s].push_back((at.div_ceil(period), c));
                    let before = commands.len();
                    while pending[s].front().is_some_and(|p| p.0 <= u64::from(k)) {
                        let (_, c) = pending[s].pop_front().expect("a pending command");
                        commands.push((c, None));
                    }
                    r.skipped += u64::from(commands.len() == before);
                    continue;
                }
                let skip = last[s].is_some()
                    && match pattern {
                        Pattern::EveryTick | Pattern::OptionA => false,
                        Pattern::Bursts => burst,
                        Pattern::Skips => {
                            r.chances += 1;
                            if left[s] > 0 {
                                left[s] -= 1;
                                rest[s] = left[s] == 0;
                                true
                            } else if rest[s] {
                                rest[s] = false;
                                false
                            } else if below(next(&mut rng), 8) == 0 {
                                left[s] = below(next(&mut rng), 6) as u32;
                                rest[s] = left[s] == 0;
                                true
                            } else {
                                false
                            }
                        }
                    };
                run[s] = if skip { run[s] + 1 } else { 0 };
                r.streak = r.streak.max(run[s]);
                let c = match last[s] {
                    Some(c) if skip => {
                        r.skipped += 1;
                        c
                    }
                    _ => {
                        let own = fed
                            .iter()
                            .find(|x| usize::from(x.0) == s)
                            .map(|x| (x.1, x.2));
                        let newest = t.wrapping_sub(DELAY as u32);
                        let mut c = bots[s].next_command(newest, &view, own);
                        (c.slot, c.tick, c.seq) = (s as u8, t, seq[s]);
                        seq[s] = seq[s].wrapping_add(1);
                        square[s].2 += 1;
                        last[s] = Some(c);
                        c
                    }
                };
                commands.push((c, None));
            }
            let input = TickInput {
                tick: t,
                commands,
                ..TickInput::default()
            };
            world.step(&input, &history);
        }
        r
    }

    /// One run, printed: its pattern, seed and figures (the stuck print names each soldier). The skip chances and the
    /// rate per 1000 are the skip pattern's alone (ADV-PT-L1); every pattern prints its skipped ticks and longest
    /// streak.
    fn printed(seed: u64, ms: u32, pattern: Pattern) -> Record {
        printed_script(seed, ms, pattern, Behaviour::brawl, "brawl1")
    }

    /// [`printed`] with every slot on `script`, its print named `name`.
    fn printed_script(
        seed: u64,
        ms: u32,
        pattern: Pattern,
        script: fn(&Scene, u8, u64) -> Behaviour,
        name: &str,
    ) -> Record {
        let r = sim_script(seed, ms, pattern, script);
        let skips = match pattern {
            Pattern::OptionA => format!("{} slot-ticks ran no command", r.skipped),
            Pattern::EveryTick => format!("skipped {}", r.skipped),
            Pattern::Skips => format!(
                "skipped {} of {} chances ({} per 1000)",
                r.skipped,
                r.chances,
                (r.skipped * 1_000).checked_div(r.chances).unwrap_or(0)
            ),
            Pattern::Bursts => format!("skipped {} in its bursts", r.skipped),
        };
        println!(
            "{name} sim seed {seed} {pattern:?} ({} ticks): in a building {} soldier-ticks (first {:?}); largest \
             distance from the centre {} cm (slot {}, tick {}, at {}, {}); {skips}, longest streak {}; dead {} \
             soldier-ticks; stuck (slot, tick, x, y): {:?}",
            ticks_for_ms(ms),
            r.in_building,
            r.first_in,
            r.far.0.isqrt(),
            r.far.1,
            r.far.2,
            r.far.3,
            r.far.4,
            r.streak,
            r.dead,
            r.stuck
        );
        r
    }

    /// The sim-only test's short copy (condition 13): seed 1, `ticks_for_ms(120_000)`, both barred patterns; no Soldier
    /// record with `area != 0` on any tick; and soldiers die in each pattern, so `soldiers()`' dead-bit tie runs on
    /// real deaths (BRAWL-1 round 1, TE-B1-6).
    #[test]
    fn brawl1_sim_short_no_soldier_enters_a_building() {
        for pattern in [Pattern::EveryTick, Pattern::Skips] {
            let r = printed(1, 120_000, pattern);
            assert_eq!(r.in_building, 0, "{pattern:?}: first {:?}", r.first_in);
            assert!(r.dead > 0, "the dead-bit tie is exercised");
        }
    }

    /// Every slot on the steady script (-21's honest clients and the forgers' base walk: U4F2-Q1 point 3).
    fn steady(scene: &Scene, slot: u8, seed: u64) -> Behaviour {
        Behaviour::script(scene, slot, seed, "steady").expect("a script")
    }

    /// C-U4F2-5's sim-only test, short copy (the debug test step's; U4F2-Q1 point 3): all 48 slots on the steady
    /// script, seed 1, `ticks_for_ms(120_000)`, the barred patterns (every tick, the skips, option A at P1): no
    /// Soldier record with `area != 0` on any tick.
    #[test]
    fn c_u4f2_5_sim_short_no_steady_soldier_enters_a_building() {
        for pattern in [Pattern::EveryTick, Pattern::Skips, Pattern::OptionA] {
            let r = printed_script(1, 120_000, pattern, steady, "c-u4f2-5");
            assert_eq!(r.in_building, 0, "{pattern:?}: first {:?}", r.first_in);
        }
    }

    /// C-U4F2-5's sim-only test, full (U4F2-Q1 point 3): seeds 1 to 3, a judged run's `ticks_for_ms(610_000)`, all 48
    /// slots on the steady script; the barred patterns (every tick, the skips, option A at P1) put no Soldier record
    /// in a building on any tick; B1's bursts are printed only. Its plant is today's walk (25d316e's `MOVES` circle),
    /// which must read in a building above 0 in at least one pattern and seed.
    #[test]
    #[ignore = "the full 610 s run, 3 seeds x 4 patterns: run by name in release for the record (C-U4F2-5)"]
    fn c_u4f2_5_sim_full_no_steady_soldier_enters_a_building_seeds_1_to_3() {
        let mut barred = Vec::new();
        for seed in 1..=3 {
            for pattern in [
                Pattern::EveryTick,
                Pattern::Skips,
                Pattern::OptionA,
                Pattern::Bursts,
            ] {
                let r = printed_script(seed, 610_000, pattern, steady, "c-u4f2-5");
                if pattern != Pattern::Bursts && r.in_building > 0 {
                    barred.push((seed, pattern, r.in_building, r.first_in));
                }
            }
        }
        assert!(barred.is_empty(), "in a building: {barred:?}");
    }

    /// TE-B1-2's npt-sim half (BRAWL-1 round 1): a soldier teleported to building 0's centre (2,200, 0: npt-sim
    /// greybox.rs:92; its footprint is half-open, x 1,800 to 2,600) reads area 1 in its Soldier record, and every other
    /// soldier area 0, so the area the sim-only test bars is one npt-sim sets. A teleport moves only a live soldier
    /// (world.rs:1147-1151), and every soldier starts alive (:745).
    #[test]
    fn brawl1_a_soldier_teleported_into_building_0_reads_area_1() {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        let mut world = World::new(&scene, 1, FIRST);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let input = TickInput {
            tick: FIRST,
            teleports: vec![Placement {
                slot: 0,
                position: [2_200, 0, 0],
            }],
            ..TickInput::default()
        };
        world.step(&input, &history);
        let now = soldiers(&world);
        assert_eq!(
            now.len(),
            usize::from(scene.slots()),
            "one Soldier record per slot"
        );
        for (slot, _, _, area) in now {
            let want = if slot == 0 { 1 } else { 0 };
            assert_eq!(area, want, "slot {slot}: area 1 in building 0, else 0");
        }
    }

    /// The sim-only test, full (conditions 3 and 13): seeds 1 to 3, a judged run's `ticks_for_ms(610_000)`, the two
    /// barred patterns (no Soldier record with `area != 0` on any tick) and B1's bursts (printed only, never barred).
    #[test]
    #[ignore = "the full 610 s run, 3 seeds x 3 patterns (about 165,000 World steps): run by name in release for the \
                record (the advisor's plan review, condition 13)"]
    fn brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3() {
        let mut barred = Vec::new();
        for seed in 1..=3 {
            for pattern in [Pattern::EveryTick, Pattern::Skips, Pattern::Bursts] {
                let r = printed(seed, 610_000, pattern);
                if pattern != Pattern::Bursts && r.in_building > 0 {
                    barred.push((seed, pattern, r.in_building, r.first_in));
                }
            }
        }
        assert!(barred.is_empty(), "in a building: {barred:?}");
    }
}
