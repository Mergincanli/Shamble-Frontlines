//! Block 4's unit tests (u6c-brief.md section 3): the settings, the token file, the per-second rows, the calibration line,
//! LG's header and the glue's host order against F-JOIN-ONCE on a fake link.

use super::*;
use npt_link::{Channel, Recv, SendResult};
use std::sync::Mutex;

/// U8's load_settings shape (wip/u8-build.patch:2180-2186), with `edit` applied.
fn settings(edit: impl FnOnce(&mut serde_json::Value)) -> Vec<u8> {
    let mut v = serde_json::json!({
        "room": "02", "server": "192.0.2.10", "wt_port": 30003, "ws_port": 31003, "mode": "run",
        "transport": "wt", "clients": 48, "duration_s": 330, "token_path": "/t/room-02.tok",
        "log_path": "/l/load-02.jsonl", "run": "R1-headers-02", "set": "R1-headers", "marks": ["PM-0001-1"],
        "date": "2026-12-03", "commit": "0123456789abcdef0123456789abcdef01234567", "role": "measured",
        "layer": "L2", "profile": "P2", "scene": "SS", "tick_hz": 30,
        "machine": {"role": "LG", "name": "LG1", "plan": "CAX21", "location": "fsn1", "address": "192.0.2.20"},
        "behaviour": "brawl", "seed": 1, "pin_hex": "ab".repeat(32)
    });
    edit(&mut v);
    serde_json::to_vec(&v).expect("serializes")
}

type Case = (&'static str, Box<dyn FnOnce(&mut serde_json::Value)>);

/// C1 (c): each mode's settings taken (probe's pin may be 64 zeros); refused for an unknown field or a value out of
/// range (SEC-U6AD-2's rules).
#[test]
fn u6c_load_settings_are_taken_per_mode_and_refused_out_of_range() {
    for mode in ["run", "calibrate", "probe"] {
        let s = LoadSettings::parse(&settings(|v| {
            v["mode"] = mode.into();
            v["pin_hex"] = "0".repeat(64).into();
        }))
        .expect(mode);
        assert_eq!(s.mode, mode);
    }
    let cases: Vec<Case> = vec![
        ("unknown", Box::new(|v| v["extra"] = 1.into())),
        (
            "machine's unknown",
            Box::new(|v| v["machine"]["x"] = 1.into()),
        ),
        ("room", Box::new(|v| v["room"] = "32".into())),
        ("room width", Box::new(|v| v["room"] = "2".into())),
        ("room sign", Box::new(|v| v["room"] = "+5".into())),
        ("server", Box::new(|v| v["server"] = "rs.example".into())),
        ("port", Box::new(|v| v["wt_port"] = 0.into())),
        ("mode", Box::new(|v| v["mode"] = "flood".into())),
        ("transport", Box::new(|v| v["transport"] = "tcp".into())),
        ("clients 0", Box::new(|v| v["clients"] = 0.into())),
        ("clients", Box::new(|v| v["clients"] = 65.into())),
        ("duration", Box::new(|v| v["duration_s"] = 0.into())),
        (
            "paths",
            Box::new(|v| v["log_path"] = "/t/room-02.tok".into()),
        ),
        ("commit", Box::new(|v| v["commit"] = "ABC".into())),
        ("pin", Box::new(|v| v["pin_hex"] = "AB".repeat(32).into())),
        ("tick_hz", Box::new(|v| v["tick_hz"] = 60.into())),
        ("scene", Box::new(|v| v["scene"] = "XX".into())),
        ("behaviour", Box::new(|v| v["behaviour"] = "thrower".into())),
        ("control", Box::new(|v| v["run"] = "R\u{7}".into())),
        (
            "missing",
            Box::new(|v| drop(v.as_object_mut().expect("an object").remove("seed"))),
        ),
    ];
    for (what, edit) in cases {
        assert!(LoadSettings::parse(&settings(edit)).is_err(), "{what}");
    }
}

/// The token file (U6a's form), one token a slot: slots 0 to n − 1 read; a missing slot or a malformed line refused, the
/// refusal naming no token; base64url on RFC 4648's vectors.
#[test]
fn u6c_the_token_file_is_read_per_slot_and_a_refusal_names_no_token() {
    assert_eq!(base64url("Zm9vYg").as_deref(), Some(&b"foob"[..]));
    assert_eq!(base64url("-_8").as_deref(), Some(&[0xfb, 0xff][..]));
    assert_eq!(base64url("Zm9vY"), None, "not whole bytes");
    let a = "AAECAwQFBgcICQoLDA0ODw";
    let text = format!("1 nptj_{a}\n0 nptj_{a}\n2 nptj_{a}\n");
    let t = tokens(text.as_bytes(), 2).expect("read");
    assert_eq!(t[1], core::array::from_fn::<u8, 16, _>(|i| i as u8));
    assert!(tokens(text.as_bytes(), 4).is_err(), "no slot 3");
    for bad in [
        format!("0 nptj_{a}x\n"),
        format!("0 {a}\n"),
        format!("x nptj_{a}\n"),
    ] {
        let e = tokens(bad.as_bytes(), 1).expect_err("refused");
        assert!(!e.contains(a), "the token in the refusal: {e}");
    }
}

/// `second.v.cpu_permille` on fixed /proc/stat texts: busy (all but idle and iowait) over all, between two reads.
#[test]
fn u6c_cpu_permille_reads_the_machines_cpu_line() {
    let a =
        cpu_ticks("cpu  100 0 100 700 100 0 0 0 0 0\ncpu0 1 0 0 0 0 0 0 0 0 0\n").expect("a line");
    let b = cpu_ticks("cpu  400 0 200 1300 100 0 0 0 0 0\n").expect("a line");
    assert_eq!((a, b), ((200, 1_000), (600, 2_000)));
    assert_eq!(cpu_permille(a, b), Some(400));
    assert_eq!(cpu_permille(b, b), None, "no time");
    assert_eq!(cpu_ticks("cpu0 1 2 3\n"), None);
}

/// P11's test: `second.s.inputs_count` lists the playing clients in slot order, never in connection order.
#[test]
fn u6c_inputs_count_lists_the_playing_clients_in_slot_order() {
    assert_eq!(inputs_record(vec![(2, 30), (0, 29), (1, 31)]), [29, 31, 30]);
    assert!(inputs_record(Vec::new()).is_empty());
}

/// `calibrate`'s one line on fixed seconds: the largest CPU, the seconds at or over 800, the inputs' extremes.
#[test]
fn u6c_the_calibration_line_reads_its_seconds() {
    let s = vec![
        (Some(790), vec![30, 29]),
        (Some(800), vec![31, 30]),
        (None, vec![30, 28]),
        (Some(812), vec![30, 30]),
    ];
    assert_eq!(
        calibration(&s),
        "CALIBRATION cpu_max_permille=812 cpu_at_800_count=2 inputs_min_count=28 inputs_max_count=31"
    );
}

/// NC-SB-7: LG's header carries the room's role, layer, profile, scene and tick rate beside its run, set, marks, date,
/// commit and seed, all from the settings; the machine is LG's, process `load`; npt-log takes it.
#[test]
fn u6c_lgs_header_carries_the_rooms_fields_and_names_lg() {
    let s = LoadSettings::parse(&settings(|_| {})).expect("taken");
    let h = header(&s, 1);
    assert_eq!(
        (
            h.run.as_str(),
            h.set.as_str(),
            h.role.as_str(),
            h.layer.as_str(),
            h.profile.as_str(),
            h.scene.as_str()
        ),
        ("R1-headers-02", "R1-headers", "measured", "L2", "P2", "SS")
    );
    assert_eq!(
        (h.tick_hz, h.seed, h.clients, h.date.as_str()),
        (30, 1, 48, "2026-12-03")
    );
    assert_eq!(
        (
            h.machine.role.as_str(),
            h.machine.name.as_str(),
            h.process.as_str()
        ),
        ("LG", "LG1", "load")
    );
    assert_eq!(
        (h.marks.clone(), h.build.commit.len()),
        (vec!["PM-0001-1".to_owned()], 40)
    );
    assert_eq!(h.params["mode"], "run");
    Writer::new(Vec::new(), &h).expect("npt-log takes the header");
}

/// A link that reads Connecting for its first `open_at` polls, then Open (or Closed, `closed`), recording each send
/// with the poll count it came at.
struct Fake {
    polls: u32,
    open_at: u32,
    closed: bool,
    sent: Arc<Mutex<Vec<(bool, u32)>>>,
}

impl Link for Fake {
    fn send_unreliable(&mut self, _: &[u8], _: u64) -> SendResult {
        self.sent.lock().unwrap().push((false, self.polls));
        SendResult::Sent
    }
    fn send_reliable(&mut self, _: Channel, _: &[u8], _: u64) -> SendResult {
        self.sent.lock().unwrap().push((true, self.polls));
        SendResult::Sent
    }
    fn poll_recv(&mut self, _: u64) -> Option<Recv> {
        self.polls += 1;
        None
    }
    fn state(&self) -> LinkState {
        match (self.polls >= self.open_at, self.closed) {
            (false, _) => LinkState::Connecting,
            (true, false) => LinkState::Open,
            (true, true) => LinkState::Closed,
        }
    }
    fn rtt_ns(&self) -> Option<u64> {
        None
    }
    fn max_datagram_size(&self) -> usize {
        1_200
    }
    fn unsent_unreliable(&self) -> usize {
        0
    }
}

/// F-JOIN-ONCE (1)-(3): the glue never steps a Connecting link and steps at once after Open, so its first send is the
/// one `Join` (reliable) at the poll that read Open or the next; a link Closed first is counted failed and never
/// stepped.
#[test]
fn u6c_the_glue_steps_only_an_open_link_and_counts_a_failed_handshake() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("a runtime");
    let scene = npt_sim::Scene::stress();
    for closed in [false, true] {
        let sent = Arc::new(Mutex::new(Vec::new()));
        let link = Fake {
            polls: 0,
            open_at: 5,
            closed,
            sent: Arc::clone(&sent),
        };
        let e =
            ClientEndpoint::new(npt_proto::Join::default(), npt_sim::TICK_HZ).with_clock(MAX_LEAD);
        let h = Hosted::new(
            Behaviour::brawl(&scene, 0, 7).controlled(),
            e,
            Box::new(link),
        );
        let st = h.state();
        rt.block_on(h.run(now_ns() + 50_000_000, now_ns));
        let sent = sent.lock().unwrap();
        if closed {
            assert!(st.failed() && sent.is_empty(), "never stepped: {sent:?}");
        } else {
            assert!(!st.failed() && st.slot().is_none());
            assert_eq!(
                sent.first().map(|s| s.0),
                Some(true),
                "the Join first: {sent:?}"
            );
            assert!((5..=6).contains(&sent[0].1), "at once after Open: {sent:?}");
            assert_eq!(sent.iter().filter(|s| s.0).count(), 1, "one Join");
        }
    }
}
