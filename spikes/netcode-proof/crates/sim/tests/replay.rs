//! U4d's tests of the recordings and their one driver (u4-build-plan.md §4.15, §5 U4d): D-2 two processes over the
//! three synthetic recordings, D-7 the pinned hash and dump vectors, the recording's byte layout and size (carry 19),
//! its canonical order (carry 8), and the replayer's refusals. In bites builds, PM-0004-21/hash-map through D-2.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command as Process, Stdio};

use npt_sim::replay::{Recording, Replayer, Step};
use npt_sim::{
    Attribute, AttributeChange, Command, CoverVariant, HitHistory, Placement, Rewind, Scene,
    SimCore, TICK_HZ, TickInput, World,
};

/// The ring span the recordings carry: 2 s.
const SPAN_MS: u32 = 2_000;
/// Set in a D-2 child's environment: `clean`, or a bite id and its arming tick as `<id>@<tick>`.
const CHILD: &str = "NPT_SIM_D2_CHILD";
/// The prefix of every line a D-2 child writes for its parent.
const MARK: &str = "npt-d2 ";

/// The three synthetic recordings (u4-build-plan.md §4.15): SS seed 1, ES seed 2 and TC seed 3, 9,000 ticks each (5
/// minutes at 30 Hz), their rings 2 s long; ES's ticks start high and TC's cross the u32 wrap.
fn recordings() -> [Recording; 3] {
    [
        Recording::synthetic(&Scene::stress(), 1, 0, 9_000, SPAN_MS),
        Recording::synthetic(&Scene::estimate(), 2, 1_000_000, 9_000, SPAN_MS),
        Recording::synthetic(&Scene::targets(), 3, u32::MAX - 4_499, 9_000, SPAN_MS),
    ]
}

/// A recording as the 32-bit words npt-hash-wasm's `feed` takes: its bytes, little-endian, the last word zero-padded.
fn words(rec: &Recording) -> Vec<u32> {
    le_words(&rec.encode())
}

/// Bytes as little-endian 32-bit words, the last zero-padded.
fn le_words(bytes: &[u8]) -> Vec<u32> {
    let mut b = bytes.to_vec();
    b.resize(b.len().div_ceil(4) * 4, 0);
    let (words, rest) = b.as_chunks::<4>();
    assert!(rest.is_empty());
    words.iter().map(|w| u32::from_le_bytes(*w)).collect()
}

/// FNV-1a-64, written again here from its definition (the offset basis 0xcbf29ce484222325, the prime 0x100000001b3).
fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h = (h ^ u64::from(*b)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// FNV-1a-64 over 32-bit words' little-endian bytes.
fn fnv_words(words: &[u32]) -> u64 {
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    fnv(&bytes)
}

/// The D-2 child's mode, from its environment: `None` in the parent.
fn child_mode() -> Option<Option<(String, u32)>> {
    let mode = std::env::var(CHILD).ok()?;
    if mode == "clean" {
        return Some(None);
    }
    let (id, arm) = mode.rsplit_once('@').expect("<id>@<tick>");
    Some(Some((id.to_string(), arm.parse().expect("an arming tick"))))
}

/// A D-2 child: replays each recording (the first with `bite`, armed at its tick), and after each tick writes its
/// hash for the parent and waits for its word: `n` the next tick, `d` this tick's dump words, `c <words>` the first
/// field where this tick's dump differs from those words, `q` stop. A bitten world's order digest is written once
/// armed. Each process logs its own digest (u4-build-plan.md §4.14).
fn serve(bite: Option<(String, u32)>) {
    let mut out = std::io::stdout().lock();
    let mut lines = std::io::stdin().lock().lines();
    for (r, rec) in recordings().iter().enumerate() {
        #[cfg(feature = "bites")]
        let bitten = bite
            .as_ref()
            .filter(|_| r == 0)
            .map(|(id, arm)| Replayer::with_bite(id, *arm).expect("a bite World plants"));
        #[cfg(not(feature = "bites"))]
        let bitten: Option<Replayer> = {
            assert!(bite.is_none(), "a bite asked of a measured build");
            None
        };
        let mut rp = bitten.unwrap_or_default();
        for w in words(rec) {
            rp.feed(w);
        }
        #[cfg(feature = "bites")]
        let mut digest_logged = false;
        loop {
            let tick = match rp.step() {
                Step::Stepped(tick) => tick,
                Step::Done => break,
                other => panic!("recording {r}: {other:?}"),
            };
            #[cfg(feature = "bites")]
            if !digest_logged && let Some(d) = rp.world().and_then(World::bite_order_digest) {
                writeln!(out, "{MARK}o {d:016x}").expect("stdout");
                digest_logged = true;
            }
            let hash = rp.state_hash().expect("a stepped replayer has a world");
            writeln!(out, "{MARK}h {r} {tick} {hash:016x}").expect("stdout");
            out.flush().expect("stdout");
            loop {
                let line = lines.next().expect("the parent's word").expect("stdin");
                let dump = rp.dump().expect("a stepped replayer has a world");
                match line
                    .split_once(' ')
                    .map_or((line.as_str(), ""), |(a, b)| (a, b))
                {
                    ("n", _) => break,
                    ("q", _) => return,
                    ("d", _) => {
                        let ws: Vec<String> =
                            dump.words().iter().map(|w| format!("{w:x}")).collect();
                        writeln!(out, "{MARK}d {}", ws.join(" ")).expect("stdout");
                    }
                    ("c", theirs) => {
                        let theirs: Vec<u32> = theirs
                            .split(' ')
                            .map(|w| u32::from_str_radix(w, 16).expect("a hex word"))
                            .collect();
                        let diff = dump.first_difference(&theirs).expect("the dumps differ");
                        writeln!(
                            out,
                            "{MARK}f {} {} {:08x} {:08x}",
                            diff.word, diff.field, diff.ours, diff.theirs
                        )
                        .expect("stdout");
                    }
                    other => panic!("a word the child does not know: {other:?}"),
                }
                out.flush().expect("stdout");
            }
        }
    }
    writeln!(out, "{MARK}end").expect("stdout");
}

/// One D-2 leg: this test binary re-run through `std::env::current_exe()` as a child, its stdin and stdout piped.
struct Leg {
    child: Child,
    stdin: ChildStdin,
    out: BufReader<ChildStdout>,
    /// The order digest it logged, if its world was bitten.
    digest: Option<String>,
}

impl Leg {
    fn spawn(test: &str, mode: &str) -> Leg {
        let exe = std::env::current_exe().expect("the test binary's path");
        let mut child = Process::new(exe)
            .args([test, "--exact", "--nocapture", "--test-threads", "1", "-q"])
            .env(CHILD, mode)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("the child starts");
        let stdin = child.stdin.take().expect("piped");
        let out = BufReader::new(child.stdout.take().expect("piped"));
        Leg {
            child,
            stdin,
            out,
            digest: None,
        }
    }

    /// Its next line for the parent (the harness's own lines skipped, a digest line kept aside).
    fn next(&mut self) -> String {
        loop {
            let mut line = String::new();
            let n = self.out.read_line(&mut line).expect("the child's stdout");
            assert!(n > 0, "a D-2 child ended without its last line");
            if let Some(at) = line.find(MARK) {
                let rest = line[at + MARK.len()..].trim_end().to_string();
                if let Some(d) = rest.strip_prefix("o ") {
                    self.digest = Some(d.to_string());
                    continue;
                }
                return rest;
            }
        }
    }

    fn send(&mut self, word: &str) {
        writeln!(self.stdin, "{word}").expect("the child's stdin");
        self.stdin.flush().expect("the child's stdin");
    }

    fn finish(mut self) {
        drop(self.stdin);
        let status = self.child.wait().expect("the child ends");
        assert!(status.success(), "a D-2 child failed: {status}");
    }
}

/// What D-2 found: the ticks compared, the first mismatch (recording, tick, field) and each leg's order digest.
#[derive(Debug)]
struct Outcome {
    compared: u32,
    first: Option<(u32, u32, String)>,
    digests: [Option<String>; 2],
}

/// D-2's comparison: two child processes of `test` in `mode`, stepped in lock step; at the first tick whose hashes
/// differ, the second leg's dump words go to the first, which names the first field that differs.
fn compare(test: &str, mode: &str) -> Outcome {
    let (mut a, mut b) = (Leg::spawn(test, mode), Leg::spawn(test, mode));
    let mut compared = 0;
    let mut first = None;
    loop {
        let (x, y) = (a.next(), b.next());
        if x == "end" && y == "end" {
            break;
        }
        let (px, py) = (parse_hash(&x), parse_hash(&y));
        assert_eq!((px.0, px.1), (py.0, py.1), "the legs step the same ticks");
        compared += 1;
        if px.2 != py.2 {
            b.send("d");
            let theirs = b.next();
            let theirs = theirs.strip_prefix("d ").expect("a dump line");
            a.send(&format!("c {theirs}"));
            let diff = a.next();
            let field = diff
                .strip_prefix("f ")
                .and_then(|d| d.split(' ').nth(1))
                .expect("a field line")
                .to_string();
            first = Some((px.0, px.1, field));
            a.send("q");
            b.send("q");
            break;
        }
        a.send("n");
        b.send("n");
    }
    let digests = [a.digest.clone(), b.digest.clone()];
    a.finish();
    b.finish();
    Outcome {
        compared,
        first,
        digests,
    }
}

/// A hash line's recording, tick and hash.
fn parse_hash(line: &str) -> (u32, u32, String) {
    let mut p = line
        .strip_prefix("h ")
        .unwrap_or_else(|| panic!("a hash line: {line}"))
        .split(' ');
    let r = p.next().and_then(|x| x.parse().ok()).expect("a recording");
    let t = p.next().and_then(|x| x.parse().ok()).expect("a tick");
    (r, t, p.next().expect("a hash").to_string())
}

#[test]
fn d2_two_processes_give_equal_per_tick_hashes_over_the_three_synthetic_recordings() {
    if let Some(mode) = child_mode() {
        serve(mode);
        return;
    }
    // A quick test of the judged check (U4d2's release-build step): two processes, each its own std seeds and its own
    // allocator's addresses, replay the three recordings through the one driver; every tick's hashes are equal.
    let test = "d2_two_processes_give_equal_per_tick_hashes_over_the_three_synthetic_recordings";
    let outcome = compare(test, "clean");
    assert_eq!(outcome.first, None, "{outcome:?}");
    assert_eq!(
        outcome.compared, 27_000,
        "every tick of the three recordings compared"
    );
    assert_eq!(outcome.digests, [None, None]);
}

#[cfg(feature = "bites")]
#[test]
fn d2_pm_0004_21_hash_map_is_first_seen_at_its_arming_tick() {
    if let Some(mode) = child_mode() {
        serve(mode);
        return;
    }
    // u4-build-plan.md §4.14: a bites-only std HashMap of 64 keys, its whole iteration order folded into a state word
    // at the arming tick T. Each process logs its order digest; D-2 must report T as the first mismatching tick and
    // the folded word as the field. Two equal digests mean "not exercised" (never caught or missed): then no tick may
    // differ.
    let test = "d2_pm_0004_21_hash_map_is_first_seen_at_its_arming_tick";
    let arm = 1_234;
    let outcome = compare(test, &format!("PM-0004-21/hash-map@{arm}"));
    let [Some(a), Some(b)] = &outcome.digests else {
        panic!("each process logs its order digest: {outcome:?}");
    };
    println!("PM-0004-21/hash-map: order digests {a} and {b}; {outcome:?}");
    if a == b {
        assert_eq!(outcome.first, None, "not exercised, yet a tick differed");
    } else {
        assert_eq!(
            outcome.first,
            Some((0, arm, "bite[0].hash_map_order".to_string())),
            "D-2 reports the arming tick and the folded word"
        );
    }
}

#[test]
fn d7_the_hash_and_dump_vectors_are_pinned() {
    // The hash function: FNV-1a-64's published vectors, by the second implementation above.
    assert_eq!(fnv(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(fnv(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(fnv(b"foobar"), 0x8594_4171_f739_67e8);
    // Each scene's world before its first tick, and the stress scene's after 1, 30 and 300 ticks of its synthetic
    // recording: the dump's length and the state hash, which is FNV-1a-64 over the dump's words (carry 38). Pinned
    // here for this build; their cross-target half (x86-64 against wasm32) is U4d2's.
    let pinned: [(Scene, usize, u64); 4] = [
        (Scene::stress(), 6_488, 6_786_077_935_583_382_297),
        (Scene::estimate(), 1_944, 8_237_017_275_248_454_161),
        (Scene::course(), 260, 12_984_630_011_248_297_851),
        (Scene::targets(), 340, 5_080_305_515_375_423_141),
    ];
    for (scene, words, hash) in &pinned {
        let world = World::new(scene, 1, 0);
        let d = world.dump();
        assert_eq!(d.hash(), world.state_hash(), "{}", scene.name());
        assert_eq!(fnv_words(d.words()), world.state_hash(), "{}", scene.name());
        assert_eq!(
            (d.words().len(), world.state_hash()),
            (*words, *hash),
            "{}",
            scene.name()
        );
    }
    let rec = Recording::synthetic(&Scene::stress(), 1, 0, 300, SPAN_MS);
    let mut world = World::new(&Scene::stress(), 1, 0);
    let mut history = HitHistory::for_ms(SPAN_MS).expect("2 s");
    let mut after = Vec::new();
    for (k, t) in rec.ticks.iter().enumerate() {
        world.step(t, &history);
        history.push(world.hit_volumes());
        if [0, 29, 299].contains(&k) {
            let d = world.dump();
            assert_eq!(fnv_words(d.words()), world.state_hash());
            after.push((k + 1, d.words().len(), world.state_hash()));
        }
    }
    assert_eq!(
        after,
        [
            (1, 6_772, 10_412_709_864_748_183_085),
            (30, 6_628, 8_845_330_375_396_163_272),
            (300, 6_612, 3_337_416_615_378_533_976),
        ]
    );
    // The dump's first names: the world's tick, its 64-bit tick count (two words), then soldier 0's fields.
    let names: Vec<String> = (0..6).filter_map(|i| world.dump().field(i)).collect();
    assert_eq!(
        names,
        [
            "world[0].next_tick",
            "world[0].elapsed",
            "world[0].elapsed",
            "soldier[0].alive",
            "soldier[0].respawn_left",
            "soldier[0].generation"
        ]
    );
    // The three synthetic recordings' bytes (generator version 2): length and FNV-1a-64, for the recordings manifest
    // (u4-build-plan.md §4.15: regenerated at check time, each file's digest committed).
    let digests: Vec<(usize, u64)> = recordings()
        .iter()
        .map(|r| {
            let b = r.encode();
            (b.len(), fnv(&b))
        })
        .collect();
    assert_eq!(Recording::GENERATOR, 2);
    assert_eq!(
        digests,
        [
            (16_694_921, 35_974_458_070_191_737),
            (16_700_849, 15_077_317_756_489_230_056),
            (799_693, 9_126_256_229_328_065_459),
        ]
    );
}

#[test]
fn recordings_round_trip_and_a_command_is_39_bytes() {
    // u4-build-plan.md §4.15 (the netcode side nit; carry 19): a command with its Rewind is 39 B (slot 1, tick 4,
    // sequence 2, buttons 2, press counters 4, move 2, aim 12, view tick 4, view fraction 1; present 1, tick_a 4,
    // fraction 1, cover 1), a spawn or teleport 13 B, an attribute change 6 B; a tick's own header 12 B; the header
    // 60 B. Measured here from the encoder.
    let empty = Recording {
        scene: Scene::stress().id(),
        seed: 9,
        first_tick: 5,
        span_ms: SPAN_MS,
        generator: 0,
        ticks: vec![TickInput {
            tick: 5,
            ..TickInput::default()
        }],
    };
    let size = |rec: &Recording| rec.encode().len();
    assert_eq!(size(&empty), 60 + 12);
    let c = Command {
        slot: 3,
        tick: 5,
        seq: 7,
        buttons: Command::FIRE | Command::CROUCH,
        presses: [1, 2, 3, 4],
        move_xy: [-128, 127],
        aim: [i32::MIN, -1, i32::MAX],
        view_tick: 4,
        view_fraction: 200,
    };
    let r = Rewind {
        tick_a: u32::MAX,
        fraction: 255,
        cover: CoverVariant::Present,
    };
    let mut one = empty.clone();
    one.ticks[0].commands = vec![(c, Some(r))];
    assert_eq!(size(&one) - size(&empty), 39, "a command with its Rewind");
    let mut forty_eight = empty.clone();
    forty_eight.ticks[0].commands = (0..48)
        .map(|s| {
            (
                Command {
                    slot: s,
                    ..Command::default()
                },
                None,
            )
        })
        .collect();
    assert_eq!(size(&forty_eight) - 60, 12 + 1_872, "a tick of 48 soldiers");
    let mut placed = empty.clone();
    placed.ticks[0].spawns = vec![Placement {
        slot: 1,
        position: [-1, 2, i32::MAX],
    }];
    placed.ticks[0].teleports = vec![Placement::default()];
    placed.ticks[0].attributes = vec![AttributeChange {
        slot: 2,
        attribute: Attribute::FireInterval,
        value: u32::MAX,
    }];
    assert_eq!(size(&placed) - size(&empty), 13 + 13 + 6);
    for rec in [&empty, &one, &forty_eight, &placed] {
        assert_eq!(Recording::decode(&rec.encode()).as_ref(), Ok(rec));
    }
    // The three synthetic recordings: each round-trips; their measured sizes.
    for rec in recordings() {
        let bytes = rec.encode();
        assert_eq!(Recording::decode(&bytes).as_ref(), Ok(&rec));
        let commands: usize = rec.ticks.iter().map(|t| t.commands.len()).sum();
        println!(
            "recording scene {} seed {}: {} ticks, {commands} commands, {} bytes",
            rec.scene,
            rec.seed,
            rec.ticks.len(),
            bytes.len()
        );
    }
}

/// Whether `commands` is in the canonical order, by netcode F2's words: slot ascending; within a slot, command tick
/// strictly ascending, wrap-aware.
fn canonical(commands: &[(Command, Option<Rewind>)]) -> bool {
    commands.windows(2).all(|w| {
        let (a, b) = (&w[0].0, &w[1].0);
        a.slot < b.slot || (a.slot == b.slot && (b.tick.wrapping_sub(a.tick) as i32) > 0)
    })
}

#[test]
fn a_synthetic_recording_is_in_the_canonical_order_and_the_replayer_feeds_it_unchanged() {
    // Carry 8's other half: the generator writes the canonical order (with a slot's held-back command beside its next
    // one now and then), and the replayer feeds the world the recorded order unchanged, so its per-tick hashes are a
    // world stepped directly on the recording's ticks.
    let mut doubled = 0;
    for rec in recordings() {
        let scene = Scene::by_id(rec.scene).expect("a known scene");
        let mut direct = World::new(&scene, rec.seed, rec.first_tick);
        let mut history = HitHistory::for_ms(rec.span_ms).expect("the header's span");
        let mut rp = Replayer::new();
        for w in words(&rec) {
            rp.feed(w);
        }
        for (k, t) in rec.ticks.iter().take(1_500).enumerate() {
            assert!(canonical(&t.commands), "tick {k}: {:?}", t.commands);
            doubled += t
                .commands
                .windows(2)
                .filter(|w| w[0].0.slot == w[1].0.slot)
                .count();
            let tick = direct.next_tick();
            direct.step(t, &history);
            history.push(direct.hit_volumes());
            assert_eq!(rp.step(), Step::Stepped(tick), "tick {k}");
            assert_eq!(rp.state_hash(), Some(direct.state_hash()), "tick {k}");
        }
    }
    assert!(doubled > 100, "{doubled} ticks with a slot's two commands");
}

#[test]
fn the_replayer_steps_a_tick_only_when_its_words_are_whole() {
    let rec = Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, SPAN_MS);
    let all = words(&rec);
    let mut whole = Replayer::new();
    for w in &all {
        whole.feed(*w);
    }
    let mut hashes = Vec::new();
    while let Step::Stepped(_) = whole.step() {
        hashes.push(whole.state_hash());
    }
    assert_eq!(hashes.len(), 60);
    assert_eq!(whole.step(), Step::Done);
    // Word by word: NeedMore until a tick is whole, then that tick; the same hashes.
    let mut rp = Replayer::new();
    assert_eq!(rp.step(), Step::NeedMore, "no header yet");
    assert!(rp.world().is_none() && rp.state_hash().is_none() && rp.dump().is_none());
    let mut got = Vec::new();
    for w in &all {
        rp.feed(*w);
        while let Step::Stepped(_) = rp.step() {
            got.push(rp.state_hash());
        }
    }
    assert_eq!(got, hashes);
    assert_eq!(rp.step(), Step::Done);
}

#[test]
fn the_replayer_refuses_a_recording_it_cannot_replay() {
    // Hostile or foreign bytes fail with a reason and step nothing more (the failure sticks); decode refuses the same
    // and also a cut recording or trailing bytes.
    let rec = Recording::synthetic(&Scene::course(), 4, 0, 5, SPAN_MS);
    let good = rec.encode();
    let run = |bytes: &[u8]| {
        let mut rp = Replayer::new();
        for w in le_words(bytes) {
            rp.feed(w);
        }
        let mut last = rp.step();
        while let Step::Stepped(_) = last {
            last = rp.step();
        }
        last
    };
    assert_eq!(run(&good), Step::Done);
    let word_at = |i: usize, w: u32| {
        let mut b = good.clone();
        b[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
        b
    };
    // The header's words: magic, version, scene, seed, TICK_HZ, first tick, ticks, span, generator, then the pools.
    // The span is the replayer's to refuse (HitHistory::for_ms: carry 14); decode reads it as written.
    for (why, bytes, decodes) in [
        ("magic", word_at(0, 0), false),
        ("version", word_at(1, 2), false),
        ("scene", word_at(2, 9), false),
        ("TICK_HZ", word_at(5, TICK_HZ + 30), false),
        ("pools", word_at(10, 47), false),
        ("a zero span", word_at(8, 0), true),
        ("a span above 1 MB", word_at(8, 7_334), true),
    ] {
        let step = run(&bytes);
        assert!(matches!(step, Step::Failed(_)), "{why}: {step:?}");
        assert_eq!(Recording::decode(&bytes).is_ok(), decodes, "{why}");
    }
    // The span at the 1 MB edge (10^6 B: 220 ticks at 30 Hz) replays.
    assert_eq!(run(&word_at(8, 7_333)), Step::Done);
    // A command's presence, cover and an attribute byte out of range: the first tick's first command starts at byte
    // 60 + 12; its Rewind's presence byte is at offset 32 in it, the cover at 38, and the attribute change follows it.
    let one = Recording {
        ticks: vec![TickInput {
            tick: 0,
            commands: vec![(Command::default(), None)],
            attributes: vec![AttributeChange {
                slot: 0,
                attribute: Attribute::MoveSpeed,
                value: 1,
            }],
            ..TickInput::default()
        }],
        ..rec.clone()
    };
    let base = one.encode();
    for (why, at, value) in [
        ("presence", 72 + 32, 2u8),
        ("cover", 72 + 38, 2),
        ("attribute", 72 + 39 + 1, 2),
    ] {
        let mut b = base.clone();
        b[at] = value;
        assert!(matches!(run(&b), Step::Failed(_)), "{why}");
        assert!(Recording::decode(&b).is_err(), "{why}");
    }
    // Commands out of the canonical order (one slot's command ticks not ascending): decode reads them as written, and
    // the replayer refuses the tick, never stepping World into its order assert (the rust review's L2).
    let disordered = Recording {
        ticks: vec![TickInput {
            tick: 0,
            commands: vec![
                (
                    Command {
                        tick: 1,
                        seq: 2,
                        ..Command::default()
                    },
                    None,
                ),
                (
                    Command {
                        tick: 0,
                        seq: 1,
                        ..Command::default()
                    },
                    None,
                ),
            ],
            ..TickInput::default()
        }],
        ..rec.clone()
    };
    let b = disordered.encode();
    assert!(Recording::decode(&b).is_ok());
    assert!(matches!(run(&b), Step::Failed(_)), "the canonical order");
    // Cut inside a tick: decode refuses it; the replayer waits for the rest (NeedMore), never steps a part.
    let cut = &good[..good.len() - 5];
    assert!(Recording::decode(cut).is_err());
    assert_eq!(run(cut), Step::NeedMore);
    // Trailing bytes past the last tick: up to three zero bytes are a word's padding; anything else is refused.
    let mut padded = good.clone();
    padded.extend_from_slice(&[0, 0, 0]);
    assert!(Recording::decode(&padded).is_ok());
    padded.push(0);
    assert!(Recording::decode(&padded).is_err());
    let mut trailing = good.clone();
    trailing.push(1);
    assert!(Recording::decode(&trailing).is_err());
    // A failure sticks.
    let mut rp = Replayer::new();
    for w in le_words(&word_at(0, 0)) {
        rp.feed(w);
    }
    let failed = rp.step();
    assert!(matches!(failed, Step::Failed(_)));
    assert_eq!(rp.step(), failed);
}
