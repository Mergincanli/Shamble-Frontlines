//! Recordings and their one replay driver (u4-build-plan.md §4.15; the advisor's Q4 ruling; A12).
//!
//! A recording is a header and each tick's [`TickInput`], fixed-width little-endian and never a `usize` (plan.md:527-
//! 529). The sim replays it alone: [`Replayer`] takes it as 32-bit words (npt-hash-wasm's `feed`), steps a [`World`]
//! per tick against the ring of past hit volumes it rebuilds itself, and gives each tick's hash and dump. npt-lab's
//! `hash` subcommand runs it natively and npt-hash-wasm's exports wrap it (U4d2); D-1 and D-2 run it here.
//!
//! **The byte layout.** The header, 15 words: magic `NPTR`, version 1, scene id, seed (low word, high word), `TICK_HZ`,
//! first tick, tick count, the ring's span in ms (carry 14; nit b), the generator's version (0 for a runner's
//! recording), and the scene's pools (soldiers, vehicles, parts, projectile capacity, effect rows per soldier). Then per
//! tick: the tick (4 B), the counts of commands, spawns, teleports and attribute changes (2 B each), then the entries:
//! a command is 39 B (slot 1, tick 4, sequence 2, buttons 2, press counters 4, move 2, aim 12, view tick 4, view
//! fraction 1, and its Rewind: present 1, tick_a 4, fraction 1, cover 1); a spawn or teleport 13 B (slot 1, position
//! 12); an attribute change 6 B (slot 1, attribute 1, value 4). Words are the bytes little-endian; the last word is
//! padded with zeros.

use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};

use crate::scene::{LARGE_PARTS, SMALL_PARTS, Scene};
use crate::seam::{
    Attribute, AttributeChange, Command, CoverVariant, Placement, Rewind, SimCore, TickInput,
};
use crate::{HitHistory, StateDump, TICK_HZ, World, ticks_for_ms};

const MAGIC: u32 = u32::from_le_bytes(*b"NPTR");
const VERSION: u32 = 1;
const HEADER_WORDS: usize = 15;
const HEADER_BYTES: usize = HEADER_WORDS * 4;
/// A tick's own header: the tick and four counts.
const TICK_HEADER_BYTES: usize = 12;
/// One command with its Rewind, as recorded.
pub(crate) const COMMAND_BYTES: usize = 39;
const PLACEMENT_BYTES: usize = 13;
const ATTRIBUTE_BYTES: usize = 6;

/// A recording (u4-build-plan.md §4.15): what one run's sim took from outside, tick by tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recording {
    /// The scene's id ([`Scene::id`]).
    pub scene: u8,
    /// The host's seed.
    pub seed: u64,
    /// The tick of its first step.
    pub first_tick: u32,
    /// The span of the hit-volume ring the run lent each step, ms ([`HitHistory::for_ms`]).
    pub span_ms: u32,
    /// The synthetic generator's version that wrote it ([`Recording::GENERATOR`]), or 0 for a runner's.
    pub generator: u32,
    /// Each tick's input, from the first tick on.
    pub ticks: Vec<TickInput>,
}

/// The pools a scene's header states, checked against the build's scene on decode.
fn pools(scene: &Scene) -> [u32; 5] {
    let parts = usize::from(scene.large_vehicles) * LARGE_PARTS
        + usize::from(scene.vehicles - scene.large_vehicles) * SMALL_PARTS;
    [
        u32::from(scene.slots),
        u32::from(scene.vehicles),
        parts as u32,
        u32::from(scene.projectile_capacity),
        u32::from(scene.effect_rows),
    ]
}

/// The fixed-width little-endian reader over a byte slice.
struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let out = self.b.get(self.at..self.at + N)?.try_into().ok()?;
        self.at += N;
        Some(out)
    }

    fn u8(&mut self) -> Option<u8> {
        self.take::<1>().map(|b| b[0])
    }

    fn u16(&mut self) -> Option<u16> {
        self.take().map(u16::from_le_bytes)
    }

    fn u32(&mut self) -> Option<u32> {
        self.take().map(u32::from_le_bytes)
    }

    fn i32(&mut self) -> Option<i32> {
        self.take().map(i32::from_le_bytes)
    }

    fn i8(&mut self) -> Option<i8> {
        self.take().map(i8::from_le_bytes)
    }
}

/// A recording's header, as read.
#[derive(Clone, Copy, Debug)]
struct Header {
    scene: u8,
    seed: u64,
    first_tick: u32,
    ticks: u32,
    span_ms: u32,
    generator: u32,
}

fn read_header(r: &mut Reader) -> Result<Header, &'static str> {
    let mut w = [0u32; HEADER_WORDS];
    for x in &mut w {
        *x = r.u32().ok_or("a recording's header is 60 bytes")?;
    }
    if w[0] != MAGIC || w[1] != VERSION {
        return Err("not a version 1 npt-sim recording");
    }
    let scene = u8::try_from(w[2])
        .ok()
        .and_then(Scene::by_id)
        .ok_or("the header names no scene")?;
    if w[5] != TICK_HZ {
        return Err("the recording's TICK_HZ is not this build's");
    }
    if w[10..15] != pools(&scene) {
        return Err("the recording's pools are not its scene's");
    }
    Ok(Header {
        scene: scene.id(),
        seed: u64::from(w[3]) | u64::from(w[4]) << 32,
        first_tick: w[6],
        ticks: w[7],
        span_ms: w[8],
        generator: w[9],
    })
}

/// The bytes a tick needs past its own header, from its four counts.
fn tick_body_len(counts: [u16; 4]) -> usize {
    usize::from(counts[0]) * COMMAND_BYTES
        + (usize::from(counts[1]) + usize::from(counts[2])) * PLACEMENT_BYTES
        + usize::from(counts[3]) * ATTRIBUTE_BYTES
}

/// The four counts of the tick header at the reader's position, without moving it.
fn peek_counts(b: &[u8]) -> Option<[u16; 4]> {
    let mut r = Reader { b, at: 4 };
    Some([r.u16()?, r.u16()?, r.u16()?, r.u16()?])
}

fn read_placement(r: &mut Reader) -> Option<Placement> {
    Some(Placement {
        slot: r.u8()?,
        position: [r.i32()?, r.i32()?, r.i32()?],
    })
}

/// One tick into `into` (its vectors reused).
fn read_tick(r: &mut Reader, into: &mut TickInput) -> Result<(), &'static str> {
    const CUT: &str = "a recording cut inside a tick";
    into.tick = r.u32().ok_or(CUT)?;
    let counts = [
        r.u16().ok_or(CUT)?,
        r.u16().ok_or(CUT)?,
        r.u16().ok_or(CUT)?,
        r.u16().ok_or(CUT)?,
    ];
    into.commands.clear();
    for _ in 0..counts[0] {
        let c = Command {
            slot: r.u8().ok_or(CUT)?,
            tick: r.u32().ok_or(CUT)?,
            seq: r.u16().ok_or(CUT)?,
            buttons: r.u16().ok_or(CUT)?,
            presses: r.take().ok_or(CUT)?,
            move_xy: [r.i8().ok_or(CUT)?, r.i8().ok_or(CUT)?],
            aim: [
                r.i32().ok_or(CUT)?,
                r.i32().ok_or(CUT)?,
                r.i32().ok_or(CUT)?,
            ],
            view_tick: r.u32().ok_or(CUT)?,
            view_fraction: r.u8().ok_or(CUT)?,
        };
        let present = r.u8().ok_or(CUT)?;
        let rewind = Rewind {
            tick_a: r.u32().ok_or(CUT)?,
            fraction: r.u8().ok_or(CUT)?,
            cover: match r.u8().ok_or(CUT)? {
                0 => CoverVariant::Rewound,
                1 => CoverVariant::Present,
                _ => return Err("a Rewind's cover is 0 or 1"),
            },
        };
        let rewind = match present {
            0 => None,
            1 => Some(rewind),
            _ => return Err("a Rewind's presence is 0 or 1"),
        };
        into.commands.push((c, rewind));
    }
    into.spawns.clear();
    for _ in 0..counts[1] {
        into.spawns.push(read_placement(r).ok_or(CUT)?);
    }
    into.teleports.clear();
    for _ in 0..counts[2] {
        into.teleports.push(read_placement(r).ok_or(CUT)?);
    }
    into.attributes.clear();
    for _ in 0..counts[3] {
        let slot = r.u8().ok_or(CUT)?;
        let attribute = match r.u8().ok_or(CUT)? {
            0 => Attribute::MoveSpeed,
            1 => Attribute::FireInterval,
            _ => return Err("an attribute is 0 or 1"),
        };
        let value = r.u32().ok_or(CUT)?;
        into.attributes.push(AttributeChange {
            slot,
            attribute,
            value,
        });
    }
    Ok(())
}

fn count(n: usize) -> [u8; 2] {
    u16::try_from(n)
        .expect("a tick holds at most 65,535 entries of each kind")
        .to_le_bytes()
}

fn write_tick(out: &mut Vec<u8>, t: &TickInput) {
    out.extend_from_slice(&t.tick.to_le_bytes());
    out.extend_from_slice(&count(t.commands.len()));
    out.extend_from_slice(&count(t.spawns.len()));
    out.extend_from_slice(&count(t.teleports.len()));
    out.extend_from_slice(&count(t.attributes.len()));
    for (c, rewind) in &t.commands {
        out.push(c.slot);
        out.extend_from_slice(&c.tick.to_le_bytes());
        out.extend_from_slice(&c.seq.to_le_bytes());
        out.extend_from_slice(&c.buttons.to_le_bytes());
        out.extend_from_slice(&c.presses);
        out.extend_from_slice(&c.move_xy.map(|m| m.to_le_bytes()[0]));
        for a in c.aim {
            out.extend_from_slice(&a.to_le_bytes());
        }
        out.extend_from_slice(&c.view_tick.to_le_bytes());
        out.push(c.view_fraction);
        let r = rewind.unwrap_or(Rewind {
            tick_a: 0,
            fraction: 0,
            cover: CoverVariant::Rewound,
        });
        out.push(u8::from(rewind.is_some()));
        out.extend_from_slice(&r.tick_a.to_le_bytes());
        out.push(r.fraction);
        out.push(match r.cover {
            CoverVariant::Rewound => 0,
            CoverVariant::Present => 1,
        });
    }
    for p in t.spawns.iter().chain(&t.teleports) {
        out.push(p.slot);
        for k in p.position {
            out.extend_from_slice(&k.to_le_bytes());
        }
    }
    for a in &t.attributes {
        out.push(a.slot);
        out.push(match a.attribute {
            Attribute::MoveSpeed => 0,
            Attribute::FireInterval => 1,
        });
        out.extend_from_slice(&a.value.to_le_bytes());
    }
}

impl Recording {
    /// The synthetic generator's version: a recordings manifest names it beside each recording's seed (§4.15).
    pub const GENERATOR: u32 = 1;

    /// The recording's bytes (the module's layout).
    ///
    /// # Panics
    /// If the scene id is unknown, or a tick holds more than 65,535 entries of one kind.
    pub fn encode(&self) -> Vec<u8> {
        let scene = Scene::by_id(self.scene).expect("a recording names a known scene");
        let mut out = Vec::with_capacity(HEADER_BYTES + self.ticks.len() * 1_900);
        let head: [u32; 10] = [
            MAGIC,
            VERSION,
            u32::from(self.scene),
            self.seed as u32,
            (self.seed >> 32) as u32,
            TICK_HZ,
            self.first_tick,
            u32::try_from(self.ticks.len()).expect("at most u32::MAX ticks"),
            self.span_ms,
            self.generator,
        ];
        for w in head.iter().chain(&pools(&scene)) {
            out.extend_from_slice(&w.to_le_bytes());
        }
        for t in &self.ticks {
            write_tick(&mut out, t);
        }
        out
    }

    /// A recording from its bytes; trailing bytes past the last tick are refused unless they are a final word's
    /// zero padding (up to 3 zero bytes).
    pub fn decode(bytes: &[u8]) -> Result<Recording, &'static str> {
        let mut r = Reader { b: bytes, at: 0 };
        let h = read_header(&mut r)?;
        // At most as many ticks as the bytes present can hold (each is at least its own 12-byte header), so a short
        // file's header cannot reserve more than its own size suggests (the rust review's L1).
        let room = bytes.len().saturating_sub(HEADER_BYTES) / TICK_HEADER_BYTES;
        let mut ticks = Vec::with_capacity((h.ticks as usize).min(room));
        for _ in 0..h.ticks {
            let mut t = TickInput::default();
            read_tick(&mut r, &mut t)?;
            ticks.push(t);
        }
        let rest = &bytes[r.at..];
        if rest.len() > 3 || rest.iter().any(|b| *b != 0) {
            return Err("bytes past the recording's last tick");
        }
        Ok(Recording {
            scene: h.scene,
            seed: h.seed,
            first_tick: h.first_tick,
            span_ms: h.span_ms,
            generator: h.generator,
            ticks,
        })
    }

    /// The synthetic recording of `scene` from `seed` (u4-build-plan.md §4.15: "a seeded generator ... open-loop
    /// commands, Rewinds drawn inside the ring", needing no room): `ticks` ticks from `first_tick`, its ring
    /// `span_ms` long. Every non-plate soldier sends one command a tick (a function of the seed, the slot and the tick
    /// alone), now and then holding one back and sending it beside the next (the catch-up case) or losing one; it moves
    /// in a random walk steered toward its team's home by its own dead reckoning, aims at a point among the other
    /// teams, holds fire (the stress and estimate scenes always; the courses in bursts) and presses fire and throw now
    /// and then, and reload on the courses; a rare aim point lies beyond the weapon's reach (the clamp) and a rare
    /// Rewind outside the
    /// ring (the refusal). Each firing command's Rewind names ticks the ring holds, and its view time is that Rewind's.
    /// Every few hundred ticks a soldier is spawned at its home, teleported there, or its move speed set. The commands
    /// are written in the canonical order (carry 8).
    pub fn synthetic(
        scene: &Scene,
        seed: u64,
        first_tick: u32,
        ticks: u32,
        span_ms: u32,
    ) -> Recording {
        let mut key = *b"npt-sim synthetic recordings....";
        key[..8].copy_from_slice(&seed.to_le_bytes());
        let mut rng = ChaCha8Rng::from_seed(key);
        rng.set_stream(u64::from(scene.id()));
        let mut draw = |n: u32| ((u64::from(rng.next_u32()) * u64::from(n)) >> 32) as u32;
        let capacity = ticks_for_ms(span_ms);
        let slots = scene.slots;
        let always_fire = scene.live_projectiles > 0 || scene.spawn_millihertz > 0;
        #[derive(Clone, Copy, Default)]
        struct Client {
            seq: u16,
            presses: [u8; 4],
            mv: [i8; 2],
            buttons: u16,
            turn_at: u32,
            est: [f32; 2],
            held_back: Option<(Command, Option<Rewind>)>,
        }
        let homes = scene.homes;
        let home_of =
            |slot: u8| -> [i32; 2] { homes[usize::from(scene.team(slot).unwrap_or(0)).min(2)] };
        let mut clients: Vec<Client> = (0..slots)
            .map(|s| Client {
                est: home_of(s).map(|k| k as f32 / 100.0),
                ..Client::default()
            })
            .collect();
        let speed = scene.move_speed as f32 / 100.0;
        let reach = scene.weapon.reach_cm as i32;
        let mut out = Vec::with_capacity(ticks as usize);
        for e in 0..ticks {
            let tick = first_tick.wrapping_add(e);
            let mut input = TickInput {
                tick,
                ..TickInput::default()
            };
            for slot in 0..slots {
                if scene.is_plate(slot) {
                    continue;
                }
                let c = &mut clients[usize::from(slot)];
                // A new heading every 0.5 to 2 s: home when it has wandered more than 8 m, else at random.
                if e >= c.turn_at {
                    c.turn_at = e + ticks_for_ms(500 + draw(1_500));
                    let home = home_of(slot).map(|k| k as f32 / 100.0);
                    let to = [home[0] - c.est[0], home[1] - c.est[1]];
                    let far = to[0] * to[0] + to[1] * to[1] > 64.0;
                    let heading = if far {
                        to
                    } else if draw(5) == 0 {
                        [0.0, 0.0]
                    } else {
                        [draw(255) as f32 - 127.0, draw(255) as f32 - 127.0]
                    };
                    // At most full speed: the length the room accepts.
                    let n = (heading[0] * heading[0] + heading[1] * heading[1]).sqrt();
                    c.mv = if n > 0.0 {
                        let k = 127.0 / n;
                        [(heading[0] * k) as i8, (heading[1] * k) as i8]
                    } else {
                        [0, 0]
                    };
                    let fire = always_fire || draw(3) > 0;
                    c.buttons = if fire { Command::FIRE } else { 0 };
                    match draw(10) {
                        0 => c.buttons |= Command::CROUCH,
                        1 | 2 => c.buttons |= Command::SPRINT,
                        _ => {}
                    }
                }
                c.est[0] += f32::from(c.mv[0]) / 127.0 * speed / TICK_HZ as f32;
                c.est[1] += f32::from(c.mv[1]) / 127.0 * speed / TICK_HZ as f32;
                if draw(40) == 0 {
                    c.presses[Command::PRESS_FIRE] = c.presses[Command::PRESS_FIRE].wrapping_add(1);
                }
                // Reloads: the courses only (a stress or estimate soldier's magazine outlasts the run, and a reload
                // would pause its fire for 2 s).
                if !always_fire && draw(300) == 0 {
                    c.presses[Command::PRESS_RELOAD] =
                        c.presses[Command::PRESS_RELOAD].wrapping_add(1);
                }
                let throws = draw(400) == 0;
                if throws {
                    c.presses[Command::PRESS_THROW] =
                        c.presses[Command::PRESS_THROW].wrapping_add(1);
                }
                // The aim: a point among another team's soldiers, or rarely one beyond the weapon's reach.
                let target = home_of((slot + 1 + draw(u32::from(slots.max(2) - 1)) as u8) % slots);
                let aim = if draw(500) == 0 {
                    [target[0] + 2 * reach, target[1], 100]
                } else {
                    [
                        target[0] + draw(1_000) as i32 - 500,
                        target[1] + draw(1_000) as i32 - 500,
                        50 + draw(150) as i32,
                    ]
                };
                // ES only (its `aim_one_in` above 1; the other scenes draw nothing more, so their recordings are as
                // before): on all but one command in `aim_one_in`, none that throws, the aim is 40 m above the shooter's
                // own reckoning, so the shot rises almost straight up and meets only a soldier whose box holds the
                // shooter's chest (ES's 10 gameplay events a second: T-M1).
                let aim = if scene.aim_one_in > 1 && !throws && draw(scene.aim_one_in) != 0 {
                    [(c.est[0] * 100.0) as i32, (c.est[1] * 100.0) as i32, 4_000]
                } else {
                    aim
                };
                c.seq = c.seq.wrapping_add(1);
                let firing = c.buttons & Command::FIRE != 0;
                // The Rewind: ticks the ring holds after e pushes (the newest is tick - 1), or rarely one it does not.
                let held = e.min(capacity);
                let rewind = if !firing || held < 2 {
                    None
                } else if draw(1_000) == 0 {
                    Some(Rewind {
                        tick_a: tick.wrapping_add(5),
                        fraction: 0,
                        cover: CoverVariant::Rewound,
                    })
                } else {
                    Some(Rewind {
                        tick_a: tick.wrapping_sub(2 + draw(held - 1)),
                        fraction: draw(256) as u8,
                        cover: if draw(2) == 0 {
                            CoverVariant::Rewound
                        } else {
                            CoverVariant::Present
                        },
                    })
                };
                let cmd = Command {
                    slot,
                    tick,
                    seq: c.seq,
                    buttons: c.buttons,
                    presses: c.presses,
                    move_xy: c.mv,
                    aim,
                    view_tick: rewind.map_or(tick.wrapping_sub(3), |r| r.tick_a),
                    view_fraction: rewind.map_or(0, |r| r.fraction),
                };
                if let Some(back) = c.held_back.take() {
                    input.commands.push(back);
                    input.commands.push((cmd, rewind));
                } else {
                    match draw(64) {
                        0 => c.held_back = Some((cmd, rewind)),
                        1 => {}
                        _ => input.commands.push((cmd, rewind)),
                    }
                }
            }
            if e % 311 == 157 {
                let slot = draw(u32::from(slots)) as u8;
                if !scene.is_plate(slot) {
                    input.attributes.push(AttributeChange {
                        slot,
                        attribute: Attribute::MoveSpeed,
                        value: 400 + draw(400),
                    });
                }
            }
            if e % 457 == 300 || e % 997 == 500 {
                let slot = draw(u32::from(slots)) as u8;
                if !scene.is_plate(slot) {
                    let home = home_of(slot);
                    let p = Placement {
                        slot,
                        position: [home[0], home[1], 0],
                    };
                    clients[usize::from(slot)].est = home.map(|k| k as f32 / 100.0);
                    if e % 457 == 300 {
                        input.teleports.push(p);
                    } else {
                        input.spawns.push(p);
                    }
                }
            }
            out.push(input);
        }
        Recording {
            scene: scene.id(),
            seed,
            first_tick,
            span_ms,
            generator: Recording::GENERATOR,
            ticks: out,
        }
    }
}

/// What [`Replayer::step`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// It ran this tick.
    Stepped(u32),
    /// The words fed so far hold no whole next tick (or no whole header yet).
    NeedMore,
    /// Every tick the header counts has run.
    Done,
    /// The recording is not one this build can replay; nothing more runs.
    Failed(&'static str),
}

/// The one replay driver (A12; u4-build-plan.md §4.15): fed a recording as 32-bit words, it builds the [`World`] and
/// the ring of past hit volumes from the header ([`HitHistory::for_ms`] of the header's span: carry 14), and steps one
/// tick per [`Replayer::step`], pushing the world's volumes into its ring after each, as the room does. The commands
/// go to the world as recorded, in the order recorded (carry 8).
#[derive(Debug)]
pub struct Replayer {
    bytes: Vec<u8>,
    read: usize,
    header: Option<Header>,
    world: Option<World>,
    history: Option<HitHistory>,
    stepped: u32,
    input: TickInput,
    failed: Option<&'static str>,
    #[cfg(feature = "bites")]
    bite: Option<(&'static str, u32)>,
}

impl Default for Replayer {
    fn default() -> Replayer {
        Replayer::new()
    }
}

impl Replayer {
    /// An empty driver, waiting for its first words.
    pub fn new() -> Replayer {
        Replayer {
            bytes: Vec::new(),
            read: 0,
            header: None,
            world: None,
            history: None,
            stepped: 0,
            input: TickInput::default(),
            failed: None,
            #[cfg(feature = "bites")]
            bite: None,
        }
    }

    /// A driver whose world carries the planted defect `bite`, armed at `arm_tick` ([`World::with_bite`]; bites builds
    /// only); `None` for an id `World` does not plant.
    #[cfg(feature = "bites")]
    pub fn with_bite(bite: &str, arm_tick: u32) -> Option<Replayer> {
        let id = World::BITES.into_iter().find(|id| *id == bite)?;
        let mut r = Replayer::new();
        r.bite = Some((id, arm_tick));
        Some(r)
    }

    /// The recording's next 4 bytes, little-endian.
    pub fn feed(&mut self, word: u32) {
        self.bytes.extend_from_slice(&word.to_le_bytes());
    }

    /// The world as of the last tick run (`None` before the header).
    pub fn world(&self) -> Option<&World> {
        self.world.as_ref()
    }

    /// The last tick's state hash (`None` before the header).
    pub fn state_hash(&self) -> Option<u64> {
        self.world.as_ref().map(SimCore::state_hash)
    }

    /// The last tick's dump (`None` before the header).
    pub fn dump(&self) -> Option<StateDump> {
        self.world.as_ref().map(World::dump)
    }

    /// Runs the next tick if the words fed hold it whole.
    pub fn step(&mut self) -> Step {
        if let Some(why) = self.failed {
            return Step::Failed(why);
        }
        let step = self.try_step();
        if let Step::Failed(why) = step {
            self.failed = Some(why);
        }
        if self.read > 1 << 16 {
            self.bytes.drain(..self.read);
            self.read = 0;
        }
        step
    }

    fn try_step(&mut self) -> Step {
        let Some(h) = self.header else {
            if self.bytes.len() - self.read < HEADER_BYTES {
                return Step::NeedMore;
            }
            let mut r = Reader {
                b: &self.bytes[self.read..],
                at: 0,
            };
            let h = match read_header(&mut r) {
                Ok(h) => h,
                Err(why) => return Step::Failed(why),
            };
            self.read += HEADER_BYTES;
            let Some(history) = HitHistory::for_ms(h.span_ms) else {
                return Step::Failed("the header's ring span is 0 ms or above 1 MB of volumes");
            };
            let scene = Scene::by_id(h.scene).unwrap_or_else(Scene::stress);
            #[cfg(feature = "bites")]
            if let Some((bite, arm)) = self.bite {
                self.world = World::with_bite(&scene, h.seed, h.first_tick, bite, arm);
            }
            if self.world.is_none() {
                self.world = Some(World::new(&scene, h.seed, h.first_tick));
            }
            self.history = Some(history);
            self.header = Some(h);
            return self.try_step();
        };
        if self.stepped == h.ticks {
            return Step::Done;
        }
        let rest = &self.bytes[self.read..];
        let Some(counts) = peek_counts(rest) else {
            return Step::NeedMore;
        };
        let len = TICK_HEADER_BYTES + tick_body_len(counts);
        if rest.len() < len {
            return Step::NeedMore;
        }
        let mut r = Reader {
            b: &rest[..len],
            at: 0,
        };
        if let Err(why) = read_tick(&mut r, &mut self.input) {
            return Step::Failed(why);
        }
        // A tick out of the canonical order is a recording this build cannot replay, never a panic of World's order
        // assert (the rust review's L2).
        if !crate::world::canonical(&self.input.commands) {
            return Step::Failed("a tick's commands are not in the canonical order");
        }
        self.read += len;
        let (Some(world), Some(history)) = (&mut self.world, &mut self.history) else {
            return Step::Failed("no world");
        };
        let tick = world.next_tick();
        world.step(&self.input, history);
        history.push(world.hit_volumes());
        self.stepped += 1;
        Step::Stepped(tick)
    }
}
