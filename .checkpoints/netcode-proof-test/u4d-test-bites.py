# U4d build proof (test-engineer): the bites of u4-build-plan.md §5 U4d (the hash-map, the std sin site, -41's two
# sites, D-4's planted 0.0 / 0.0, the bites-only clippy allows), the bites the plan missed (D-1, D-3's streams, D-5's
# trace, D-6's thresholds, D-7's pins, D-8's slack, D-9's reference, D-10's ammunition, reload, cooldown-event, clamp
# and rewind traces), the bites builds' own unit proofs disarmed, and one probe file that settles reviewer findings by
# measurement. Written independently of the builder's plants P1-P16. Each plant is made in place in U4d's copy, alone;
# the one test (or clippy) it targets runs; the failure (or its absence) is recorded by name and message; every file is
# restored from memory and its sha256 checked against the reference (u4d-review-test.md). The probe file is created,
# run once, and deleted. Build rules (u4-build-plan.md §7.1): CARGO_TARGET_DIR is the shared folder, CARGO_NET_OFFLINE
# is true; every file under crates/ is touched before each build and every 2 s while it runs; df before each build
# (stop under 2 GB); a run must show "Compiling/Checking npt-sim (this copy's path)" and exactly one test.
# Run: python3 -I <this> <spike dir> [id ...]
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
import threading
import time

SPIKE = pathlib.Path(sys.argv[1]).resolve()
ONLY = set(sys.argv[2:])
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
SIM = SPIKE / "crates/sim"
WORLD, SCENE, HASH = SIM / "src/world.rs", SIM / "src/scene.rs", SIM / "src/hash.rs"
PROBE = SIM / "tests/te_probe.rs"
REF = {
    "src/greybox.rs": "2d75cb0b6949680c98463817046b15610c48454968d845e1ce65c414504f35e5",
    "src/grid.rs": "a0e24c08224e5dd412ad78e8caca870d92c6f2e03e36765bea5261ffcc999484",
    "src/hash.rs": "4663b2a8d7c771d05ac33d9af8a9c1bcb951f7d35ce34440717d8699cce45f06",
    "src/history.rs": "0c9012ace74fa3bf449d7aab6de363ab76071e0e4457ac26d2d901f29fee9cc7",
    "src/lib.rs": "70eb8262e18f6461c81db8a7a9eb17710fbe209ab8e800ea18ede9448f01ba6f",
    "src/replay.rs": "8c566b8ae73dd9b946de0b3bad73ce12c7348f25d8fe5bba1655efbe25e2c3ff",
    "src/rng.rs": "d6b61bbe47569943af598ce34b549cfc7297cefc27652cdd91414557bfc2cb7d",
    "src/scene.rs": "4d8a04b5769cbaff76dac41c60be0364a78d1a797167886ea78a73e8362b2d56",
    "src/seam.rs": "a176cd0d8868e070a2591483efe1d3de661adba173d1e4bd32bf6e2f731a410a",
    "src/ticks.rs": "5d852f1a81f72e958a89f5489bce5d556da92d096e86aebc4bd130e447dece5b",
    "src/trajectory.rs": "2c0968b3ce8fef64991c2d2962193fdb821b8d117b52c7761c31dc5cd2c602ef",
    "src/world.rs": "57b0cf9c212c8033b810dd724d4b61c017fdc9d06fa9801171326a6b605b476d",
    "tests/replay.rs": "a547314c1ea9593ee62bb079fb8d4f351b06ceb347e715224a30df0322b6b636",
    "tests/seam.rs": "d91b8c0c9fe872a0322126ecc6d6a72ba218bb63e3c24995abae177955da163b",
    "tests/world.rs": "41ffc20ff978ad9a1fbdfc2c6f9c427f3e45bd9a0e4629b227ae6158431409ca",
}

D2 = "d2_two_processes_give_equal_per_tick_hashes_over_the_three_synthetic_recordings"
D2HM = "d2_pm_0004_21_hash_map_is_first_seen_at_its_arming_tick"
SPEED = "bites::pm_0001_41_server_speed_1pct_moves_the_authority_step_and_not_predict"
QUANT = "bites::pm_0001_41_server_quant_differs_snaps_the_authoritys_own_copy_off_the_records_grid"
SIN = "bites::pm_0004_21_std_sin_is_planted_at_the_vehicles_loops_from_its_arming_tick"

QUANT_OLD = """        let pos_snapped = [
            snap.snap(POSITION_XY, pos[0]),
            snap.snap(POSITION_XY, pos[1]),
            snap.snap(POSITION_Z, pos[2]),
        ];"""
QUANT_NEW = """        let pos_snapped = if authority {
            [
                snap.from_grid(POSITION_XY, (f64::from(pos[0]) * 100.0).floor() as i32),
                snap.from_grid(POSITION_XY, (f64::from(pos[1]) * 100.0).floor() as i32),
                snap.snap(POSITION_Z, pos[2]),
            ]
        } else {
            [
                snap.snap(POSITION_XY, pos[0]),
                snap.snap(POSITION_XY, pos[1]),
                snap.snap(POSITION_Z, pos[2]),
            ]
        };"""
HASHMAP_FOLD = """        if self.elapsed == 1_234 {
            let map: std::collections::HashMap<u32, u32> = (0..64).map(|k| (k, k)).collect();
            for k in map.keys() {
                self.spawn_acc = self.spawn_acc.wrapping_mul(31).wrapping_add(u64::from(*k));
            }
        }
        self.events.clear();
        self.reports.clamps.clear();
"""
EV = "        self.events.clear();\n        self.reports.clamps.clear();\n"
PLANTED = "/// The planted defects `World` carries (bites builds only), each chosen per instance (u4-build-plan.md §4.11).\n"

PROBE_SRC = r'''//! U4d test-engineer probes (temporary: created, run once and deleted by u4d-test-bites.py; never committed). Each
//! prints its measurement on a line that starts with PROBE-; none asserts.

use npt_sim::replay::{Replayer, Step};
use npt_sim::{
    Command, EntityKind, EntityRecord, HitHistory, HitVolumes, Placement, Recording, Scene, SimCore, SimEvent,
    TICK_HZ, TickInput, World,
};
use std::collections::BTreeMap;

const SPAN_MS: u32 = 2_000;

fn ring() -> HitHistory {
    HitHistory::for_ms(SPAN_MS).expect("2 s")
}

fn room_tick(w: &mut World, h: &mut HitHistory, t: &TickInput) -> Vec<SimEvent> {
    let e = w.step(t, h).to_vec();
    h.push(w.hit_volumes());
    e
}

fn soldier(w: &World, slot: u8) -> EntityRecord {
    *w.entity_records()
        .iter()
        .find(|r| r.kind == EntityKind::Soldier && r.index == u16::from(slot))
        .expect("a record")
}

fn le_words(bytes: &[u8]) -> Vec<u32> {
    let mut b = bytes.to_vec();
    b.resize(b.len().div_ceil(4) * 4, 0);
    b.chunks(4).map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]])).collect()
}

fn fire_at(w: &mut World, h: &mut HitHistory, seq: u16, aim: [i32; 3]) -> Vec<[i32; 3]> {
    let c = Command { slot: 0, tick: w.next_tick(), seq, buttons: Command::FIRE, aim, ..Command::default() };
    let t = TickInput { tick: w.next_tick(), commands: vec![(c, None)], ..TickInput::default() };
    room_tick(w, h, &t);
    w.reports().clamps.iter().map(|r| r.point).collect()
}

#[test]
fn probe_a_clamp_idempotence_near_the_grid_top() {
    for feet_z in [0, 3_900, 3_955, 4_000, 4_095] {
        let mut w = World::new(&Scene::course(), 9, 0);
        let mut h = ring();
        let t = TickInput {
            tick: 0,
            spawns: vec![Placement { slot: 0, position: [0, -1_500, feet_z] }],
            ..TickInput::default()
        };
        room_tick(&mut w, &mut h, &t);
        let z = soldier(&w, 0).grid[2];
        let first = fire_at(&mut w, &mut h, 1, [300_000, -1_500, z + 140]);
        let again = first.first().map(|p| fire_at(&mut w, &mut h, 2, *p));
        println!("PROBE-A feet z {feet_z} cm: record z {z}; first clamp {first:?}; the clamped point sent again: {again:?}");
    }
}

#[test]
fn probe_b_flag_fired_on_a_catch_up_tick() {
    let scene = Scene::course();
    let mut authority = World::new(&scene, 7, 100);
    let mut h = ring();
    for k in 0..5u16 {
        let c = Command { slot: 0, tick: authority.next_tick(), seq: k, aim: [0, -500, 140], ..Command::default() };
        let t = TickInput { tick: authority.next_tick(), commands: vec![(c, None)], ..TickInput::default() };
        room_tick(&mut authority, &mut h, &t);
    }
    let before = authority.clone();
    let mut h2 = ring();
    let t0 = authority.next_tick();
    let c1 = Command { slot: 0, tick: t0, seq: 10, presses: [1, 0, 0, 0], aim: [0, -500, 140], ..Command::default() };
    let c2 = Command { slot: 0, tick: t0.wrapping_add(1), seq: 11, presses: [1, 0, 0, 0], aim: [0, -500, 140], ..Command::default() };
    let t = TickInput { tick: t0, commands: vec![(c1, None), (c2, None)], ..TickInput::default() };
    let events = room_tick(&mut authority, &mut h, &t);
    let mut client = before.clone();
    client.load_movers(authority.entity_records());
    let fired = (client.predict(&c1), client.predict(&c2));
    let (a, p) = (soldier(&authority, 0), soldier(&client, 0));
    println!("PROBE-B catch-up tick (two commands, the first fires): authority events {events:?}; predicted fired {fired:?}; records equal: {}", a == p);
    for i in 0..EntityRecord::FIELDS {
        if a.grid[i] != p.grid[i] || a.raw[i] != p.raw[i] {
            println!("PROBE-B field {i}: grid {} (authority) vs {} (predicted), raw {:#x} vs {:#x}", a.grid[i], p.grid[i], a.raw[i], p.raw[i]);
        }
    }
    let mut a2 = before.clone();
    let mut c = before.clone();
    for cmd in [c1, c2] {
        let t = TickInput { tick: a2.next_tick(), commands: vec![(cmd, None)], ..TickInput::default() };
        room_tick(&mut a2, &mut h2, &t);
        c.load_movers(a2.entity_records());
        c.predict(&cmd);
    }
    println!("PROBE-B control (the same two commands, one a tick): records equal: {}", soldier(&a2, 0) == soldier(&c, 0));
}

#[test]
fn probe_c_load_movers_with_a_record_this_world_has_no_entity_for() {
    let ss = World::new(&Scene::stress(), 1, 0);
    let mut part = *ss.entity_records().iter().find(|r| r.kind == EntityKind::VehiclePart).expect("a part");
    part.index = 400;
    let mut w = World::new(&Scene::course(), 1, 0);
    let mut h = ring();
    let before = w.hit_volumes().cover.len();
    w.load_movers(&[part]);
    let t = TickInput { tick: w.next_tick(), ..TickInput::default() };
    room_tick(&mut w, &mut h, &t);
    println!(
        "PROBE-C CO cover boxes: {before} before; after load_movers of one part record of index 400 and one step: {} (HitVolumes::MAX_COVER {})",
        w.hit_volumes().cover.len(),
        HitVolumes::MAX_COVER
    );
}

#[test]
fn probe_d_decode_preallocation_from_the_header_alone() {
    let mut b = Recording::synthetic(&Scene::course(), 4, 0, 1, SPAN_MS).encode();
    b.truncate(60);
    b[28..32].copy_from_slice(&(1u32 << 20).to_le_bytes());
    let peak = || {
        std::fs::read_to_string("/proc/self/status")
            .expect("status")
            .lines()
            .find(|l| l.starts_with("VmPeak"))
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let before = peak();
    let r = Recording::decode(&b).map(|_| ());
    let after = peak();
    println!(
        "PROBE-D size_of::<TickInput>() = {} B; decode of a {} B file whose header claims 2^20 ticks: {r:?}; {before} -> {after}",
        std::mem::size_of::<TickInput>(),
        b.len()
    );
}

#[test]
fn probe_e_a_non_canonical_recording_through_the_replayer() {
    let mut rec = Recording::synthetic(&Scene::stress(), 1, 0, 3, SPAN_MS);
    rec.ticks[1].commands.swap(0, 1);
    let bytes = rec.encode();
    let decoded = Recording::decode(&bytes).map(|_| ());
    let words = le_words(&bytes);
    let out = std::panic::catch_unwind(move || {
        let mut rp = Replayer::new();
        for w in words {
            rp.feed(w);
        }
        let mut steps = Vec::new();
        loop {
            let s = rp.step();
            let more = matches!(s, Step::Stepped(_));
            steps.push(format!("{s:?}"));
            if !more {
                break;
            }
        }
        steps
    });
    let out = out.map_err(|e| {
        e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
    });
    println!("PROBE-E decode: {decoded:?}; replayer: {out:?}");
}

#[test]
fn probe_f_stress_scene_shots_by_minute_over_thirteen_minutes() {
    let scene = Scene::stress();
    let minute = 60 * TICK_HZ;
    let n = 13 * minute;
    let rec = Recording::synthetic(&scene, 1, 0, n, SPAN_MS);
    let mut w = World::new(&scene, 1, 0);
    let mut h = ring();
    let mut shots = vec![0u64; 13];
    let mut kills = 0u64;
    for (k, t) in rec.ticks.iter().enumerate() {
        let e = room_tick(&mut w, &mut h, t);
        shots[k / minute as usize] += e.iter().filter(|e| matches!(e, SimEvent::Shot { .. })).count() as u64;
        kills += e.iter().filter(|e| matches!(e, SimEvent::Kill { .. })).count() as u64;
    }
    let per_s: Vec<String> = shots.iter().map(|s| format!("{}.{:02}", s / 60, s * 100 / 60 % 100)).collect();
    let empty = w.dump();
    let mut dry = 0;
    for i in 0..empty.words().len() {
        if empty.field(i).is_some_and(|f| f.starts_with("soldier[") && f.ends_with(".ammo")) && empty.words()[i] == 0 {
            dry += 1;
        }
    }
    println!("PROBE-F SS shots a second, minute by minute over 13 min: {per_s:?}; kills {kills}; soldiers with ammo 0 at the end: {dry}");
}

#[test]
fn probe_g_estimate_scene_against_pm_0001_2_es_bands() {
    let scene = Scene::estimate();
    let secs = 100u32;
    let first = 1_000_000;
    let rec = Recording::synthetic(&scene, 2, first, secs * TICK_HZ, SPAN_MS);
    let mut w = World::new(&scene, 2, first);
    let mut h = ring();
    let (mut shots, mut proj_sum, mut spawns, mut kills) = (0u64, 0u64, 0u64, 0u64);
    let (mut proj_min, mut veh_min, mut sold_min) = (usize::MAX, usize::MAX, usize::MAX);
    let live = |w: &World| -> Vec<(u16, u16)> {
        w.entity_records().iter().filter(|r| r.kind == EntityKind::Projectile).map(|r| (r.index, r.generation)).collect()
    };
    let mut prev = live(&w);
    for t in &rec.ticks {
        let e = room_tick(&mut w, &mut h, t);
        shots += e.iter().filter(|e| matches!(e, SimEvent::Shot { .. })).count() as u64;
        kills += e.iter().filter(|e| matches!(e, SimEvent::Kill { .. })).count() as u64;
        let proj = live(&w);
        spawns += proj.iter().filter(|p| !prev.contains(p)).count() as u64;
        proj_sum += proj.len() as u64;
        proj_min = proj_min.min(proj.len());
        let r = w.entity_records();
        veh_min = veh_min.min(r.iter().filter(|r| r.kind == EntityKind::Vehicle).count());
        sold_min = sold_min.min(r.iter().filter(|r| r.kind == EntityKind::Soldier).count());
        prev = proj;
    }
    let ticks = u64::from(secs * TICK_HZ);
    let s = u64::from(secs);
    println!(
        "PROBE-G ES over {secs} s: shots {}.{:02}/s (band 68.4-75.6); live projectiles mean {}.{:02} (band 30.4-33.6), min {proj_min}; spawns {}.{:02}/s (band 20.064-22.176); vehicles min {veh_min} (>= 15); soldiers min {sold_min} (>= 48); kills {kills}",
        shots / s, shots * 100 / s % 100, proj_sum / ticks, proj_sum * 100 / ticks % 100, spawns / s, spawns * 100 / s % 100
    );
}

#[test]
fn probe_h_the_d10_clamp_stream_shoots_at_step_10() {
    let mut w = World::new(&Scene::course(), 5, 0);
    let mut h = ring();
    let mut presses = [0u8; 4];
    let mut fired = Vec::new();
    for k in 0..60u32 {
        if k == 30 {
            presses[Command::PRESS_FIRE] = presses[Command::PRESS_FIRE].wrapping_add(1);
        }
        let c = Command {
            slot: 0,
            tick: k,
            seq: k as u16,
            buttons: if k == 10 { Command::FIRE } else { 0 },
            presses,
            aim: if k == 10 { [300_000, -500, 140] } else { [0, -500, 140] },
            ..Command::default()
        };
        let t = TickInput { tick: k, commands: vec![(c, None)], ..TickInput::default() };
        if room_tick(&mut w, &mut h, &t).iter().any(|e| matches!(e, SimEvent::Shot { slot: 0, .. })) {
            fired.push(k);
        }
    }
    println!("PROBE-H d10's clamp stream (fire held at 10 aimed 3 km off, a press at 30): shots at steps {fired:?}");
}

#[test]
fn probe_k_stress_records_that_change_each_tick_by_kind() {
    let scene = Scene::stress();
    let rec = Recording::synthetic(&scene, 1, 0, 300, SPAN_MS);
    let mut w = World::new(&scene, 1, 0);
    let mut h = ring();
    let kinds = [
        EntityKind::Soldier,
        EntityKind::Vehicle,
        EntityKind::VehiclePart,
        EntityKind::Projectile,
        EntityKind::Door,
        EntityKind::Wall,
        EntityKind::Effect,
    ];
    let key = |r: &EntityRecord| (kinds.iter().position(|k| *k == r.kind).expect("a kind"), r.index);
    let snap = |w: &World| -> BTreeMap<(usize, u16), (u16, [i32; EntityRecord::FIELDS])> {
        w.entity_records().iter().map(|r| (key(r), (r.generation, r.grid))).collect()
    };
    let mut prev = snap(&w);
    let (mut changed, mut total) = ([0u64; 7], [0u64; 7]);
    for t in &rec.ticks {
        room_tick(&mut w, &mut h, t);
        let now = snap(&w);
        for (k, v) in &now {
            total[k.0] += 1;
            changed[k.0] += u64::from(prev.get(k) != Some(v));
        }
        prev = now;
    }
    let n = rec.ticks.len() as u64;
    let rows: Vec<String> = kinds
        .iter()
        .enumerate()
        .map(|(i, k)| format!("{k:?} {}.{:01} of {}.{:01}", changed[i] / n, changed[i] * 10 / n % 10, total[i] / n, total[i] * 10 / n % 10))
        .collect();
    println!("PROBE-K SS records whose grid or generation changed from the previous tick, a tick on average over 300 ticks: {rows:?}");
}
'''

M, B, CM, CB, P = "measured", "bites", "clippy-measured", "clippy-bites", "probe"
# (id, what, kind, (test file, test name) or None, [(file, old, new)], must it fail?, what the failure must say)
BITES = [
    # The plan's bites, in the clean (measured) build: the defect planted in the clean path, caught by the clean tests.
    ("B1", "hash-map: a 64-key std HashMap's whole order folded into spawner.acc at tick 1234 (SS, recording 0)", M,
     ("replay", D2), [(WORLD, EV, HASHMAP_FOLD)], True, 'first: Some((0, 1234, "spawner[0].acc"))'),
    ("B1-clippy", "the same HashMap outside the bites cfg: sim/clippy.toml's ban", CM, None,
     [(WORLD, EV, HASHMAP_FOLD)], True, "disallowed"),
    ("B2", "std sin in place of libm::sinf in World::sinf (every vehicle loop)", M, ("replay", "d7_the_hash_and_dump_vectors_are_pinned"),
     [(WORLD, "        libm::sinf(x)\n    }", "        x.sin()\n    }")], True, "assertion"),
    ("B2-clippy", "the same std sin outside the bites cfg: sim/clippy.toml's ban", CM, None,
     [(WORLD, "        libm::sinf(x)\n    }", "        x.sin()\n    }")], True, "disallowed method"),
    ("B3", "-41 speed: the authority step's run speed x 1.01 (predict's not)", M, ("world", "d3_prediction_equals_authority_for_one_slot_bit_for_bit"),
     [(WORLD, "        let mut speed = speed_cm as f32 / 100.0;\n", "        let mut speed = speed_cm as f32 / 100.0;\n        if authority {\n            speed *= 1.01;\n        }\n")],
     True, "record"),
    ("B4", "-41 quant: the authority's own-copy snap of x and y by floor, not the wire grid's nearest", M,
     ("world", "d3_prediction_equals_authority_for_one_slot_bit_for_bit"), [(WORLD, QUANT_OLD, QUANT_NEW)], True, "record"),
    ("B5", "D-4: soldier[0].pitch = 0.0 / 0.0 after the effects phase (phase 6) at elapsed 150", M,
     ("world", "d4_the_state_holds_no_non_finite_value_on_any_tick"),
     [(WORLD, "        self.run_effects();\n", "        self.run_effects();\n        if self.elapsed == 150 {\n            self.soldiers[0].pitch = 0.0 / 0.0;\n        }\n")],
     True, "non-finite value in the state after tick 1150: soldier[0].pitch"),
    # Missed by the plan, measured build.
    ("E1", "D-1: a process-wide counter folded into each new World (two instances share state)", M,
     ("world", "d1_two_worlds_on_one_recording_agree_every_tick"),
     [(WORLD, PLANTED, "static TE_WORLDS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);\n\n" + PLANTED),
      (WORLD, "            spawn_acc: 0,\n            rngs: STREAMS", "            spawn_acc: TE_WORLDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed),\n            rngs: STREAMS")],
     True, "assertion"),
    ("E2", "D-3: predict draws once from the authority-only spread stream", M, ("world", "d3_prediction_equals_authority_for_one_slot_bit_for_bit"),
     [(WORLD, "        let fired = self.run_command(command, None, false);\n",
       "        let _ = self.rngs[Stream::Spread as usize - 1].below(7);\n        let fired = self.run_command(command, None, false);\n")],
     True, "rng["),
    ("E3", "D-5: a refused throw draws from the projectiles' stream (a trace)", M, ("world", "d5_a_full_pool_refuses_and_counts_and_leaves_no_trace"),
     [(WORLD, "            self.reports.pool_full += 1;\n            return;", "            self.reports.pool_full += 1;\n            let _ = self.rngs[Stream::Projectiles as usize - 1].below(2);\n            return;")],
     True, "step 16"),
    ("E4a", "D-6: the stress scene with 14 big vehicles", M, ("world", "d6_the_stress_scene_meets_pm_0001_2_on_every_tick"),
     [(SCENE, "            vehicles: 15,\n            large_vehicles: 4,", "            vehicles: 14,\n            large_vehicles: 4,")], True, "big vehicles: 14"),
    ("E4b", "D-6: the stress scene with 3 very large units", M, ("world", "d6_the_stress_scene_meets_pm_0001_2_on_every_tick"),
     [(SCENE, "            vehicles: 15,\n            large_vehicles: 4,", "            vehicles: 15,\n            large_vehicles: 3,")], True, "units of 20 parts: 3"),
    ("E4c", "D-6: the stress scene with 299 live stand-in projectiles", M, ("world", "d6_the_stress_scene_meets_pm_0001_2_on_every_tick"),
     [(SCENE, "            live_projectiles: 300,", "            live_projectiles: 299,")], True, "projectiles:"),
    ("E4d", "D-6: the stress weapon's interval 75,000 us (about 640 shots a second)", M, ("world", "d6_the_stress_scene_meets_pm_0001_2_on_every_tick"),
     [(SCENE, "                interval_us: 68_571,", "                interval_us: 75_000,")], True, "shots a second"),
    ("E5", "D-7: FNV-1a-64's prime off by 2", M, ("replay", "d7_the_hash_and_dump_vectors_are_pinned"),
     [(HASH, "const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;", "const FNV_PRIME: u64 = 0x0000_0100_0000_01b5;")], True, "assertion"),
    ("E6", "D-9: the blend by fraction / 255 in place of / 256 (1 cm off at fraction 255)", M,
     ("world", "d9_a_rewind_blends_tick_a_and_tick_a_plus_one_by_its_fraction"),
     [(WORLD, "        let f = f32::from(r.fraction) / 256.0;", "        let f = f32::from(r.fraction) / 255.0;")], None, ""),
    ("E7", "D-10: a fire press refused for no ammunition adds 1 unit to the weapon clock (a trace)", M,
     ("world", "d10_a_fire_press_with_no_ammunition_leaves_no_trace"),
     [(WORLD, "                self.reports.ammunition += 1;\n", "                self.reports.ammunition += 1;\n                s.wait += 1;\n")], True, "step 100"),
    ("E8", "D-10: a refused reload press adds 1 unit to the weapon clock (a trace)", M,
     ("world", "d10_a_reload_press_the_weapon_refuses_leaves_no_trace"),
     [(WORLD, "                    self.reports.reload += 1;\n", "                    self.reports.reload += 1;\n                    s.wait += 1;\n")], True, "step 5"),
    ("E9", "D-10: a cooldown refusal writes a Shot event into the hashed event list", M,
     ("world", "d10_a_fire_press_in_the_cooldown_leaves_no_trace"),
     [(WORLD, "                self.reports.cooldown += 1;\n", "                self.reports.cooldown += 1;\n                self.events.push(SimEvent::Shot { slot: c.slot, seq: c.seq });\n")],
     True, "step 11: the event lists"),
    ("E10", "D-10: a clamp folds into spawner.acc (the clamp in the hash)", M, ("world", "d10_a_clamped_aim_point_leaves_no_trace"),
     [(WORLD, "        if authority {\n            self.reports.clamps.push(ClampReport {",
       "        self.spawn_acc = self.spawn_acc.wrapping_add(1);\n        if authority {\n            self.reports.clamps.push(ClampReport {")],
     True, "step 10: the state hashes"),
    ("E11", "D-10: a Rewind outside the ring folds into spawner.acc (a trace)", M, ("world", "d10_a_rewind_outside_the_ring_leaves_no_trace"),
     [(WORLD, "                    self.reports.rewind_outside += 1;\n                    None",
       "                    self.reports.rewind_outside += 1;\n                    self.spawn_acc ^= 1;\n                    None")], True, "step 15"),
    ("E12", "D-8: the clamp's slack 2 cm in place of 0.87 cm", M, ("world", "d8_the_clamp_moves_a_far_point_to_the_reach_and_never_again"),
     [(WORLD, "const CLAMP_SLACK: f64 = 0.0087;", "const CLAMP_SLACK: f64 = 0.02;")], True, "not clamped"),
    ("P", "probes A-K (reviewer findings settled by measurement)", P, ("te_probe", None), [(PROBE, None, PROBE_SRC)], None, ""),
    # Bites builds: the plan's own unit proofs, as built and with their sites disarmed.
    ("B1b", "the bites build's hash-map D-2 as built (its digests and first mismatch)", B, ("replay", D2HM), [], False, ""),
    ("B1c", "hash-map bite: the folded word left out of the dump and hash (exercised, never seen)", B, ("replay", D2HM),
     [(WORLD, '            s.wide("bite", 0, "hash_map_order", fold);', "            let _ = fold;")], True, "D-2 reports the arming tick"),
    ("B2b", "std-sin bite: the site disarmed (std sin only for a NaN argument)", B, ("world", SIN),
     [(WORLD, "        if self.bitten(Bite::StdSin) {\n            return std_sin(x);", "        if self.bitten(Bite::StdSin) && x.is_nan() {\n            return std_sin(x);")],
     None, ""),
    ("B3b", "-41 speed bite: disarmed (x 1.0)", B, ("world", SPEED),
     [(WORLD, "            speed *= 1.01;", "            speed *= 1.0;")], True, "moving ticks differ"),
    ("B4b", "-41 quant bite: disarmed (the wire grid returned)", B, ("world", QUANT),
     [(WORLD, "            return b.snap;", "            return Config::default();")], True, "ticks off the record's grid"),
    ("B6a", "the hash-map site's allow without clippy::disallowed_types", CB, None,
     [(WORLD, "    clippy::disallowed_types,\n    clippy::iter_over_hash_type,\n", "    clippy::iter_over_hash_type,\n")], True, "disallowed type"),
    ("B6b", "the std-sin site's allow without clippy::disallowed_methods", CB, None,
     [(WORLD, "    clippy::disallowed_methods,\n    reason = \"PM-0004-21/std-sin's", "    clippy::needless_return,\n    reason = \"PM-0004-21/std-sin's")],
     True, "disallowed method"),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def check_all(when):
    bad = [f for f, h in REF.items() if sha(SIM / f) != h]
    extra = PROBE.exists()
    print(f"{when}: every crates/sim file at its reference sha256: {not bad and not extra} {bad if bad else ''}"
          f"{' (probe file present!)' if extra else ''}")
    assert not bad and not extra


def df_ok():
    gb = shutil.disk_usage("/home/user").free / 1e9
    print(f"       df /home/user: {gb:.1f} GB free")
    if gb < 2:
        sys.exit("under 2 GB free: stop")


def touch():
    for p in (SPIKE / "crates").rglob("*"):
        if p.is_file():
            os.utime(p)


def busy():
    for args in (["pgrep", "-x", "cargo"], ["pgrep", "-x", "rustc"], ["pgrep", "-x", "rustdoc"],
                 ["pgrep", "-f", "clippy-driver"], ["pgrep", "-f", "cargo-clippy"]):
        if subprocess.run(args, stdout=subprocess.DEVNULL).returncode == 0:
            return True
    return False


def wait_idle(need=4, limit=600):
    idle, waited = 0, 0
    while idle < need and waited < limit:
        idle = 0 if busy() else idle + 1
        time.sleep(1)
        waited += 1
    return waited, idle >= need


def toucher(stop):
    while not stop.wait(2.0):
        touch()


def run(kind, target):
    df_ok()
    waited, idle = wait_idle()
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET, CARGO_NET_OFFLINE="true")
    feats = ["--all-features"] if kind in (B, CB) else []
    if kind in (CM, CB):
        args = ["cargo", "clippy", "--locked", "-p", "npt-sim", "--all-targets"] + feats + ["--", "-D", "warnings"]
    elif kind == P:
        args = ["cargo", "test", "--locked", "-p", "npt-sim", "--test", "te_probe", "--", "--nocapture", "--test-threads", "1"]
    else:
        args = ["cargo", "test", "--locked", "-p", "npt-sim"] + feats + ["--test", target[0], "--", target[1], "--exact", "--nocapture"]
    touch()
    stop = threading.Event()
    th = threading.Thread(target=toucher, args=(stop,))
    th.start()
    t0 = time.time()
    try:
        r = subprocess.run(args, cwd=SPIKE, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    finally:
        stop.set()
        th.join()
    others = busy()
    print(f"       waited {waited} s for idle ({'idle' if idle else 'NOT idle: ran beside another cargo'}); "
          f"ran {time.time() - t0:.0f} s; another cargo running at the end: {others}")
    return r.returncode, r.stdout


check_all("start")
originals = {p: p.read_bytes() for p in (WORLD, SCENE, HASH)}
results = []
for bid, what, kind, target, edits, must_fail, says in BITES:
    if ONLY and bid not in ONLY:
        continue
    planted = {}
    for path, old, new in edits:
        if old is None:
            planted[path] = new
            continue
        src = planted.get(path, originals[path].decode())
        assert src.count(old) == 1, f"{bid}: anchor found {src.count(old)} times in {path.name}"
        planted[path] = src.replace(old, new, 1)
    print(f"{bid}: {what}")
    try:
        for path, src in planted.items():
            path.write_text(src)
        code, out = run(kind, target)
    finally:
        for path in planted:
            if path == PROBE:
                path.unlink()
            else:
                path.write_bytes(originals[path])
    restored = {p.name: (not p.exists()) if p == PROBE else sha(p) == REF[str(p.relative_to(SIM))] for p in planted}
    clippy = kind in (CM, CB)
    mine = (f"Checking npt-sim v0.0.0 ({SIM})" if clippy else f"Compiling npt-sim v0.0.0 ({SIM})") in out
    if clippy:
        errs = [l for l in out.splitlines() if l.startswith("error")]
        verdict = "BITES" if (mine and code != 0 and says in out) else "MISSED"
        print(f"{verdict} {bid}: exit {code}; this copy's npt-sim checked: {mine}; says {says!r}: {says in out}")
        for l in errs[:6]:
            print("       " + l[:300])
        at = re.findall(r"--> (crates/sim/[^\n]+)", out)
        print(f"       at {at[:3]}")
    else:
        compiled = "error[" not in out and "could not compile" not in out
        ran = re.findall(r"running (\d+) tests?", out)
        result = re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
        name = target[1]
        failed = name is not None and f"test {name} ... FAILED" in out
        passed = name is not None and f"test {name} ... ok" in out
        panic = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,6})", out)
        msg = " | ".join(l.strip() for l in panic.group(2).splitlines() if l.strip())[:700] if panic else "(no panic)"
        if kind == P:
            verdict = "PROBE RAN" if (compiled and mine and result and result[-1][1] == "0") else "PROBE FAILED"
        elif must_fail is True:
            verdict = "BITES" if (compiled and mine and failed and says in out) else "MISSED"
        elif must_fail is False:
            verdict = "PASSES" if (compiled and mine and passed) else "FAILED"
        else:
            verdict = "FAILED (informative)" if failed else ("PASSED (not caught)" if passed else "UNCLEAR")
        print(f"{verdict} {bid}: {name}; this copy's npt-sim compiled: {mine}; {'compiled' if compiled else 'DID NOT COMPILE'}; "
              f"running {ran}; results {result}; says {says!r}: {(says in out) if says else '-'}")
        print(f"       at {panic.group(1) if panic else '-'}: {msg}")
        for l in out.splitlines():
            if l.startswith("PROBE-") or "order digests" in l or "first differing tick" in l or l.startswith("D-6:"):
                print("       " + l[:900])
        if not compiled:
            print("       " + "\n       ".join(l for l in out.splitlines() if l.startswith("error"))[:1500])
    print(f"       restored: {restored}")
    results.append((bid, verdict))
    assert all(restored.values()), f"{bid}: not restored"
    sys.stdout.flush()
check_all("end")
print("summary: " + "; ".join(f"{b} {v}" for b, v in results))
