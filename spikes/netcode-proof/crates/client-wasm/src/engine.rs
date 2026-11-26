//! The browser client's plain-Rust core (u7-brief.md rev 1.1 Block R3; the advisor's C2, C5), tested natively; lib.rs
//! only converts for wasm-bindgen. The slot token, C5's inbox and outbox (every integer little-endian), the steps due
//! (each the native hosts' order: `input` with what arrived, then each command due, then the endpoint's `step`; at most
//! `LOOP_CATCHUP` a call, the rest left due), the counters by name, the limits.

use npt_client::wire::{
    CONTENT_HASH, Channel, Direction, Join, LOOP_CATCHUP, Limits, PROTOCOL_VERSION, TICK_HZ,
};
use npt_client::{ClientEndpoint, HONEST_MAX_LEAD};

use crate::script;

/// The slot token's form (U6a's token file line, `{slot} nptj_<base64url of 16 B>`): the prefix, then 22 characters.
const TOKEN_PREFIX: &str = "nptj_";
const TOKEN_CHARS: usize = 22;
/// Records one inbox may hold, and received messages waiting for a due step (C5; every queue bounded).
pub(crate) const INBOX_MAX: usize = 512;
/// An inbox record's head: channel u8, arrival ns u64, length u32 (C5; an outbox record's is channel u8, length u32).
const INBOX_HEAD: usize = 13;
/// The largest control-stream message the client takes, its 2 B length counted: the largest room-to-client control
/// row of `Limits::TYPES` (the catalogue's :68-71), `JoinAccepted`'s 256 B.
pub(crate) const CONTROL_MAX: usize = largest_control_to_client();
/// The largest full-state message the client takes, its 4 B length counted, as U6b's WtClient caps it (C5;
/// crates/net/src/wt_server.rs:49, 65,536 + 5 B): one above what npt-proto's decoder takes, which still refuses it.
pub(crate) const FULL_STATE_MAX: usize = 65_541;

const fn largest_control_to_client() -> usize {
    let mut i = 0;
    let mut max = 0;
    while i < Limits::TYPES.len() {
        let (_, channel, direction, limit) = Limits::TYPES[i];
        if matches!(channel, Channel::Control)
            && matches!(direction, Direction::RoomToClient)
            && limit > max
        {
            max = limit;
        }
        i += 1;
    }
    max
}

/// The counters' names, by index: the one home of what `Core::counters` holds (the page reads names, never
/// positions; the page needs the first three: u7-t-build.md F2). `last_run_command` is the room's last command run as
/// the snapshot headers name it, plus 1 (0: none yet), which shows the room running the page's commands.
pub const COUNTER_NAMES: [&str; 17] = [
    "joined",
    "clock_samples",
    "own_soldier",
    "steps",
    "commands",
    "inbox_records",
    "inbox_refused",
    "pending_dropped",
    "wrong_channel",
    "refused",
    "stale",
    "duplicates",
    "unknown_pongs",
    "clock_resets",
    "stamps_skipped",
    "last_run_command",
    "unencodable_commands",
];

/// `limits()` in C5's order: the largest control message, the largest full state (each with its length), the largest
/// datagram, then the WebSocket tags of `Datagram`, `Control` and `FullState` (npt-link's one home: C2).
pub fn limits() -> [u32; 6] {
    [
        CONTROL_MAX as u32,
        FULL_STATE_MAX as u32,
        Limits::SNAPSHOT_RECEIVE as u32,
        u32::from(Channel::Datagram.ws_tag()),
        u32::from(Channel::Control.ws_tag()),
        u32::from(Channel::FullState.ws_tag()),
    ]
}

/// Why [`Core::new`] refused its arguments (lib.rs turns it into a JsError).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Token,
    Bind,
}

impl Refused {
    pub fn text(self) -> &'static str {
        match self {
            Refused::Token => "the slot token is not nptj_ and 22 base64url characters",
            Refused::Bind => "the bind is not 16 bytes",
        }
    }
}

/// `nptj_` and 22 base64url characters (RFC 4648 §5, no padding) to the token's 16 bytes, in the canonical form U6a
/// writes (the last character's 4 spare bits zero); `None` for any other text.
pub fn decode_token(text: &str) -> Option<[u8; 16]> {
    let body = text.strip_prefix(TOKEN_PREFIX)?.as_bytes();
    if body.len() != TOKEN_CHARS {
        return None;
    }
    let mut out = [0u8; 16];
    let (mut acc, mut bits, mut n) = (0u32, 0u32, 0usize);
    for &c in body {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            if let Some(o) = out.get_mut(n) {
                *o = (acc >> bits) as u8;
            }
            n += 1;
            acc &= (1 << bits) - 1;
        }
    }
    (n == 16 && acc == 0).then_some(out)
}

/// A little-endian integer from `bytes` (at most 8).
fn le(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .rev()
        .fold(0u64, |n, &b| (n << 8) | u64::from(b))
}

/// The inbox record at `at` (the `records`-th): its channel, arrival, bytes and end; `None` when it is refused (past
/// the buffer's end, an unknown channel, a length above its channel's cap, or past `INBOX_MAX` records).
fn record(inbox: &[u8], at: usize, records: usize) -> Option<(Channel, u64, &[u8], usize)> {
    if records >= INBOX_MAX {
        return None;
    }
    let head = inbox.get(at..at.checked_add(INBOX_HEAD)?)?;
    let (channel, cap) = match head[0] {
        0 => (Channel::Datagram, Limits::SNAPSHOT_RECEIVE),
        1 => (Channel::Control, CONTROL_MAX),
        2 => (Channel::FullState, FULL_STATE_MAX),
        _ => return None,
    };
    let arrival = le(&head[1..9]);
    let len = usize::try_from(le(&head[9..13])).ok()?;
    if len > cap {
        return None;
    }
    let end = at.checked_add(INBOX_HEAD)?.checked_add(len)?;
    let bytes = inbox.get(at + INBOX_HEAD..end)?;
    Some((channel, arrival, bytes, end))
}

/// One outbox record onto `out`: channel, length, bytes (C5).
pub(crate) fn push_record(out: &mut Vec<u8>, channel: u8, bytes: &[u8]) {
    out.push(channel);
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
}

/// The browser client (the module text).
pub struct Core {
    endpoint: ClientEndpoint,
    /// Received messages waiting for the next due step, as the endpoint takes them: (channel, bytes, arrival ns).
    pending: Vec<(Channel, Vec<u8>, u64)>,
    next_due_ns: Option<u64>,
    steps: u64,
    commands: u64,
    inbox_records: u64,
    inbox_refused: u64,
    pending_dropped: u64,
}

impl Core {
    /// The client of the slot `token` names, with the 16 B `bind` the page made: its `Join` carries npt-proto's
    /// protocol version and content hash (C2), its endpoint the controller at npt-client's honest cap (C3).
    pub fn new(token: &str, bind: &[u8]) -> Result<Core, Refused> {
        let token = decode_token(token).ok_or(Refused::Token)?;
        let bind: [u8; 16] = bind.try_into().map_err(|_| Refused::Bind)?;
        let join = Join {
            token,
            bind,
            version: PROTOCOL_VERSION,
            content_hash: CONTENT_HASH,
        };
        Ok(Core {
            endpoint: ClientEndpoint::new(join, TICK_HZ).with_clock(HONEST_MAX_LEAD),
            pending: Vec::new(),
            next_due_ns: None,
            steps: 0,
            commands: 0,
            inbox_records: 0,
            inbox_refused: 0,
            pending_dropped: 0,
        })
    }

    /// One wake at `now_ns`: the inbox taken, `prev_send_ns` (the host's send time of the last call's output; 0 before
    /// anything was sent) given to the endpoint, then every due step up to `LOOP_CATCHUP` (the rest left due); returns
    /// the outbox (datagrams on channel 0, control messages on 1).
    pub fn step(&mut self, now_ns: u64, prev_send_ns: u64, inbox: &[u8]) -> Vec<u8> {
        self.take(inbox);
        if prev_send_ns != 0 {
            self.endpoint.sent(prev_send_ns);
        }
        let mut out = Vec::new();
        for _ in 0..LOOP_CATCHUP {
            if self.next_due_ns.is_some_and(|due| due > now_ns) {
                break;
            }
            let received = core::mem::take(&mut self.pending);
            self.endpoint.input(None, &received, now_ns);
            if let (Some(newest), Some(_)) = (self.endpoint.newest_tick(), self.endpoint.slot()) {
                for _ in 0..self.endpoint.commands_due() {
                    // The presses from the commands issued, not the steps (U7R-NC-2): a catch-up burst runs many steps
                    // a wake but issues about one command a tick, so at most one edge goes in each command.
                    let c = script::command(self.commands, newest);
                    self.endpoint.input(Some(c), &[], now_ns);
                    self.commands += 1;
                }
            }
            let s = self.endpoint.step();
            for d in &s.datagrams {
                push_record(&mut out, 0, d);
            }
            for (channel, bytes) in &s.reliable {
                if *channel == Channel::Control {
                    push_record(&mut out, 1, bytes);
                }
            }
            self.next_due_ns = Some(s.next_due_ns);
            self.steps += 1;
        }
        out
    }

    /// The inbox's records into `pending`: at most `INBOX_MAX`; a record past the buffer's end, of an unknown channel
    /// or above its channel's cap is refused and counted, and the rest of the inbox skipped (C5). A record taken while
    /// `pending` is full is dropped and counted.
    fn take(&mut self, inbox: &[u8]) {
        let (mut at, mut records) = (0usize, 0usize);
        while at < inbox.len() {
            let Some(rec) = record(inbox, at, records) else {
                self.inbox_refused += 1;
                return;
            };
            let (channel, arrival, bytes, end) = rec;
            if self.pending.len() >= INBOX_MAX {
                self.pending_dropped += 1;
            } else {
                self.pending.push((channel, bytes.to_vec(), arrival));
            }
            self.inbox_records += 1;
            records += 1;
            at = end;
        }
    }

    /// When the next step is due, ns on the host's clock; 0 before the first step (due at once).
    pub fn next_due_ns(&self) -> u64 {
        self.next_due_ns.unwrap_or(0)
    }

    /// The counters, by [`COUNTER_NAMES`]' index (each saturating at u32::MAX).
    pub fn counters(&self) -> [u32; COUNTER_NAMES.len()] {
        let e = &self.endpoint;
        let sat = |v: u64| u32::try_from(v).unwrap_or(u32::MAX);
        [
            u32::from(e.slot().is_some()),
            sat(e.clock().samples().len() as u64),
            u32::from(e.own_soldier().is_some()),
            sat(self.steps),
            sat(self.commands),
            sat(self.inbox_records),
            sat(self.inbox_refused),
            sat(self.pending_dropped),
            sat(e.wrong_channel()),
            sat(e.refused()),
            sat(e.stale()),
            sat(e.duplicates()),
            sat(e.unknown_pongs()),
            sat(e.clock().resets()),
            sat(e.stamps_skipped()),
            e.last_run_command().map_or(0, |c| sat(u64::from(c) + 1)),
            sat(e.unencodable_commands()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 6-bit value's base64url letter, computed (a 64-letter literal reads as a secret to redact-check's long-token
    /// rule: U6a's encoder does the same, wip/u6a-build.patch:756).
    fn letter(v: u32) -> char {
        let v = v as u8;
        char::from(match v {
            0..=25 => b'A' + v,
            26..=51 => b'a' + (v - 26),
            52..=61 => b'0' + (v - 52),
            62 => b'-',
            _ => b'_',
        })
    }

    /// `bytes` in base64url without padding (the test's own encoder, as U6a's token file writes it).
    fn b64url(bytes: &[u8]) -> String {
        let mut out = String::new();
        for c in bytes.chunks(3) {
            let n = c
                .iter()
                .enumerate()
                .fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
            for k in 0..=c.len() {
                out.push(letter((n >> (18 - 6 * k)) & 63));
            }
        }
        out
    }

    fn token(bytes: [u8; 16]) -> String {
        [TOKEN_PREFIX, &b64url(&bytes)].concat()
    }

    /// The inbox as the page packs it (the test's own packer, from C5's text).
    fn inbox(records: &[(u8, u64, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        for &(channel, arrival, bytes) in records {
            out.push(channel);
            out.extend_from_slice(&arrival.to_le_bytes());
            out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(bytes);
        }
        out
    }

    /// The outbox's records: (channel, bytes).
    fn outbox(mut b: &[u8]) -> Vec<(u8, Vec<u8>)> {
        let mut out = Vec::new();
        while !b.is_empty() {
            let len = le(&b[1..5]) as usize;
            out.push((b[0], b[5..5 + len].to_vec()));
            b = &b[5 + len..];
        }
        out
    }

    fn core() -> Core {
        Core::new(&token([0x5a; 16]), &[0xa5; 16]).expect("a client")
    }

    /// R3: U6a's token form decodes to its 16 bytes; no prefix, 21 or 23 characters, a character outside base64url
    /// or a last character with spare bits set is refused; a bind of other than 16 bytes too.
    #[test]
    fn r3_the_slot_token_decodes_to_its_16_bytes_and_anything_else_is_refused() {
        let raw: [u8; 16] = core::array::from_fn(|i| (i as u8).wrapping_mul(37).wrapping_add(11));
        let good = token(raw);
        assert_eq!(good.len(), 5 + 22);
        assert_eq!(decode_token(&good), Some(raw));
        let body = &good[5..];
        for bad in [
            body.to_owned(),
            ["nptx_", body].concat(),
            ["nptj_", &body[..21]].concat(),
            [good.as_str(), "A"].concat(),
            ["nptj_", &body[..21], "!"].concat(),
            ["nptj_", &body[..20], "=", &body[21..]].concat(),
            ["nptj_", &body[..21], "B"].concat(),
        ] {
            assert_eq!(decode_token(&bad), None, "{bad}");
        }
        assert_eq!(Core::new(body, &[0; 16]).err(), Some(Refused::Token));
        for n in [0, 15, 17] {
            assert_eq!(
                Core::new(&good, &vec![1; n]).err(),
                Some(Refused::Bind),
                "{n}"
            );
        }
    }

    /// R3 (C2): the first step sends one control message, the `Join`: framed (2 B length, then its type, 7: its row of
    /// `Limits::TYPES`), 69 B in all, carrying the token, the bind, npt-proto's version and content hash.
    #[test]
    fn r3_the_first_step_sends_the_join_with_the_shared_version_and_content_hash() {
        let mut c = core();
        let out = outbox(&c.step(1_000, 0, &[]));
        let controls: Vec<&Vec<u8>> = out.iter().filter(|r| r.0 == 1).map(|r| &r.1).collect();
        assert_eq!(controls.len(), 1);
        let j = controls[0];
        assert_eq!((j.len(), le(&j[0..2]), j[2]), (69, 67, 7));
        let find = |needle: &[u8]| j.windows(needle.len()).any(|w| w == needle);
        assert!(find(&[0x5a; 16]) && find(&[0xa5; 16]) && find(&CONTENT_HASH));
        assert!(find(&PROTOCOL_VERSION.to_le_bytes()));
        assert!(
            outbox(&c.step(2_000, 1_500, &[])).is_empty(),
            "nothing due before the next tick"
        );
    }

    /// R3: a wake 20 ticks late runs `LOOP_CATCHUP` steps and leaves the rest due; the next wake at the same time runs
    /// the next ones.
    #[test]
    fn r3_a_late_wake_runs_loop_catchup_steps_and_leaves_the_rest_due() {
        let mut c = core();
        c.step(0, 0, &[]);
        assert_eq!(c.counters()[3], 1);
        let late = 20 * 1_000_000_000 / u64::from(TICK_HZ);
        c.step(late, 0, &[]);
        assert_eq!(c.counters()[3], 1 + LOOP_CATCHUP);
        assert!(c.next_due_ns() <= late, "the rest is still due");
        c.step(late, 0, &[]);
        assert_eq!(c.counters()[3], 1 + 2 * LOOP_CATCHUP);
    }

    /// R3 (C5): an inbox the page packs is taken record by record, channel, arrival and bytes as packed, waiting in
    /// order for the next due step; a step with none due leaves them waiting.
    #[test]
    fn r3_the_inbox_and_outbox_round_trip() {
        let mut c = core();
        c.step(0, 0, &[]);
        let packed = inbox(&[
            (0, 1 << 40, &[2, 9]),
            (1, 77, &[3, 0, 8, 1, 2]),
            (2, 5, &[1, 0, 0, 0, 3]),
        ]);
        c.step(1, 0, &packed);
        assert_eq!(
            c.pending,
            vec![
                (Channel::Datagram, vec![2, 9], 1 << 40),
                (Channel::Control, vec![3, 0, 8, 1, 2], 77),
                (Channel::FullState, vec![1, 0, 0, 0, 3], 5),
            ]
        );
        assert_eq!((c.counters()[5], c.counters()[6]), (3, 0));
        let mut out = Vec::new();
        push_record(&mut out, 0, &[1, 2, 3]);
        push_record(&mut out, 1, &[]);
        assert_eq!(out, [0, 3, 0, 0, 0, 1, 2, 3, 1, 0, 0, 0, 0]);
        assert_eq!(outbox(&out), vec![(0, vec![1, 2, 3]), (1, vec![])]);
    }

    /// R3 (C5): a record cut short, of an unknown channel, or above its channel's cap is refused and counted, and the
    /// rest of that inbox skipped; the records before it are kept.
    #[test]
    fn r3_a_truncated_inbox_record_is_refused_counted_and_the_rest_skipped() {
        let good: (u8, u64, &[u8]) = (0, 1, &[2, 9]);
        let over = vec![0u8; Limits::SNAPSHOT_RECEIVE + 1];
        for (what, bad) in [
            ("cut short", {
                let b = inbox(&[good, (0, 2, &[1, 2, 3, 4])]);
                b[..b.len() - 1].to_vec()
            }),
            (
                "a head cut short",
                inbox(&[good, (0, 2, &[1])])[..INBOX_HEAD + 2 + 5].to_vec(),
            ),
            ("an unknown channel", inbox(&[good, (3, 2, &[1]), good])),
            ("above its cap", inbox(&[good, (0, 2, &over), good])),
        ] {
            let mut c = core();
            c.step(0, 0, &[]);
            c.step(1, 0, &bad);
            assert_eq!((c.counters()[5], c.counters()[6]), (1, 1), "{what}");
            assert_eq!(c.pending.len(), 1, "{what}");
        }
        let mut c = core();
        let many: Vec<(u8, u64, &[u8])> = (0..INBOX_MAX + 3)
            .map(|i| (0, i as u64, &[1u8][..]))
            .collect();
        c.step(0, 0, &inbox(&many));
        assert_eq!(
            (c.counters()[5], c.counters()[6]),
            (INBOX_MAX as u32, 1),
            "512 records a call"
        );
    }

    /// R3 (C5): the counters pair with their names by index, the page's three first.
    #[test]
    fn r3_counter_names_and_counters_pair_by_index() {
        let c = core();
        assert_eq!(c.counters().len(), COUNTER_NAMES.len());
        assert_eq!(
            COUNTER_NAMES[..3],
            ["joined", "clock_samples", "own_soldier"]
        );
        let mut names = COUNTER_NAMES.to_vec();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), COUNTER_NAMES.len(), "each name once");
    }

    /// R3 (C5): the limits are the wire's: `JoinAccepted`'s 256 B, U6b's full-state cap, the datagram receive limit,
    /// npt-link's tags.
    #[test]
    fn r3_the_limits_are_the_wires() {
        assert_eq!(CONTROL_MAX, Limits::JOIN_ACCEPTED);
        assert_eq!(FULL_STATE_MAX, Limits::FULL_STATE + 5);
        assert_eq!(limits(), [256, 65_541, 1_150, 0, 1, 2]);
    }

    /// The values of `key` in the golden file, in order: each a string's contents or a number's digits.
    fn values<'a>(text: &'a str, key: &str) -> Vec<&'a str> {
        let pat = format!("\"{key}\": ");
        text.match_indices(&pat)
            .map(|(i, _)| {
                let v = &text[i + pat.len()..];
                match v.strip_prefix('"') {
                    Some(s) => &s[..s.find('"').unwrap_or(0)],
                    None => &v[..v.find([',', ' ', '}']).unwrap_or(0)],
                }
            })
            .collect()
    }

    fn unhex(h: &str) -> Vec<u8> {
        (0..h.len() / 2)
            .map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap_or(0))
            .collect()
    }

    /// R3 (C5): the T-half's golden file, read here against the real packing: its inbox taken gives its records
    /// (an arrival above 2^53 among them), and its outbox records packed give its hex.
    #[test]
    fn r3_the_golden_boundary_file_reads_and_packs_as_the_page_does() {
        let g = include_str!("../../../web/test/fixtures/boundary.json");
        let (hex, channels) = (values(g, "hex"), values(g, "channel"));
        let (arrivals, bytes) = (values(g, "arrival_ns"), values(g, "bytes_hex"));
        assert_eq!(
            (hex.len(), channels.len(), arrivals.len(), bytes.len()),
            (2, 5, 3, 5)
        );
        let mut c = core();
        c.step(0, 0, &[]);
        c.step(1, 0, &unhex(hex[0]));
        let want: Vec<(Channel, Vec<u8>, u64)> = (0..3)
            .map(|i| {
                let ch = [Channel::Datagram, Channel::Control, Channel::FullState][i];
                assert_eq!(channels[i], i.to_string());
                (ch, unhex(bytes[i]), arrivals[i].parse().unwrap_or(0))
            })
            .collect();
        assert_eq!(c.pending, want);
        assert!(want.iter().any(|r| r.2 > 1 << 53));
        let mut out = Vec::new();
        for i in 3..5 {
            push_record(&mut out, channels[i].parse().unwrap_or(9), &unhex(bytes[i]));
        }
        assert_eq!(out, unhex(hex[1]));
    }

    use npt_link::{Link, Loopback};
    use npt_proto::{MODE_HASH, Message, Quant};
    use npt_room::{RefusalRecord, Room};
    use npt_server::{Decoder, RttSource};
    use npt_sim::{Scene, World};

    /// The room's clock at its tick 0, ns (as npt-server's tests' lab).
    const BASE_NS: u64 = 1_000_000_000;
    /// The Loopback's datagram limit (npt-server's tests' lab: the room's groups stay within 1,000 B).
    const MAX_DATAGRAM: usize = 1_200;
    const MS: u64 = 1_000_000;
    /// The browser's clamp on a nested zero-delay timer (U7R-NC-2's 4 ms): the Worker's wakes are one chain of timers,
    /// so a wake that leaves steps due re-arms at 0 and runs 4 ms later.
    const TIMER_CLAMP_NS: u64 = 4 * MS;

    /// A test-only `Link` over a Loopback end, as npt-server's tests' `Gate`: every send stamped `by_ns` later (the
    /// path, and on the client's side the host's send lag too), so the other end polls it that much later; every
    /// other call is the Loopback's. The Decoder's own sends (the `Pong`) take the path as the room's do.
    struct Delayed {
        inner: Loopback,
        by_ns: u64,
    }

    impl Link for Delayed {
        fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> npt_link::SendResult {
            self.inner.send_unreliable(bytes, now_ns + self.by_ns)
        }
        fn send_reliable(
            &mut self,
            channel: Channel,
            bytes: &[u8],
            now_ns: u64,
        ) -> npt_link::SendResult {
            self.inner
                .send_reliable(channel, bytes, now_ns + self.by_ns)
        }
        fn poll_recv(&mut self, now_ns: u64) -> Option<npt_link::Recv> {
            self.inner.poll_recv(now_ns)
        }
        fn state(&self) -> npt_link::LinkState {
            self.inner.state()
        }
        fn rtt_ns(&self) -> Option<u64> {
            self.inner.rtt_ns()
        }
        fn max_datagram_size(&self) -> usize {
            self.inner.max_datagram_size()
        }
        fn unsent_unreliable(&self) -> usize {
            self.inner.unsent_unreliable()
        }
    }

    /// R3's room (C6; C2's equality, U7's half): npt-room's `Room` behind npt-server's real `Decoder`, as npt-server's
    /// tests host it (crates/server/tests/common/mod.rs) and as the binary builds both (tick_loop.rs:455-467,
    /// :850-857): T5's scene and seed (stress, 7), first tick 0, npt-proto's version and content hash at the decoder,
    /// `with_hashes(CONTENT_HASH, MODE_HASH)` at the room. One Loopback pair, each end behind a `Delayed` (U7R-NC-1:
    /// the path each way, and the host's send lag on the client's side; 0 for the plain room); the `Core` woken at its
    /// own `next_due_ns` (plus the wake schedule's lateness, U7R-TE-3), never before the last wake's send (its lag), nor
    /// within the timer clamp after a wake that left steps due, with what arrived as C5's records, its outbox sent at
    /// once and that send's time passed as the next call's, as the Worker does (worker.ts:283-299).
    struct RoomHost {
        room: Room<World>,
        decoder: Decoder,
        server: Delayed,
        client: Delayed,
        core: Core,
        slot: Option<u8>,
        /// Server ticks run, and the time of the last event.
        k: u64,
        now_ns: u64,
        prev_send_ns: u64,
        /// The host's send lag, the wakes so far, the lateness of each wake by its index and the client's steps before
        /// it, the earliest next wake.
        lag_ns: u64,
        wakes: u64,
        late: fn(u64, u32) -> u64,
        floor_ns: u64,
        /// The wait after a wake that left steps due: the browser's clamp, or 0 for a host that re-wakes at once.
        clamp_ns: u64,
        /// Every reliable message the room sent the client: its stream and bytes.
        from_room: Vec<(Channel, Vec<u8>)>,
        /// Every refusal record the room kept (its kinds 1-6, 8, 10; the sim's own are not among them).
        refusals: Vec<RefusalRecord>,
    }

    impl RoomHost {
        fn new() -> RoomHost {
            RoomHost::with_path(0, 0, 0, |_, _| 0)
        }

        /// The room over `up_ns` and `down_ns` of path, the host sending `lag_ns` after each wake, wake `w` (after `s`
        /// client steps) running `late(w, s)` after its due time.
        fn with_path(up_ns: u64, down_ns: u64, lag_ns: u64, late: fn(u64, u32) -> u64) -> RoomHost {
            let raw = [0x5a; 16];
            let scene = Scene::stress();
            let world = World::new(&scene, 7, 0);
            let greybox = world.greybox().clone();
            let room = Room::new(world, &scene, &greybox, vec![raw], 1, TICK_HZ)
                .with_hashes(CONTENT_HASH, MODE_HASH);
            let decoder = Decoder::new(
                Quant::default(),
                TICK_HZ,
                BASE_NS,
                PROTOCOL_VERSION,
                CONTENT_HASH,
                RttSource::Link,
            );
            let (server, client) = Loopback::pair(MAX_DATAGRAM);
            let core = Core::new(&token(raw), &[0xa5; 16]).expect("a client");
            RoomHost {
                room,
                decoder,
                server: Delayed {
                    inner: server,
                    by_ns: down_ns,
                },
                client: Delayed {
                    inner: client,
                    by_ns: lag_ns + up_ns,
                },
                core,
                slot: None,
                k: 0,
                now_ns: BASE_NS,
                prev_send_ns: 0,
                lag_ns,
                wakes: 0,
                late,
                floor_ns: BASE_NS,
                clamp_ns: TIMER_CLAMP_NS,
                from_room: Vec::new(),
                refusals: Vec::new(),
            }
        }

        /// The counter `name` (by name, as the page reads them).
        fn counter(&self, name: &str) -> u32 {
            let i = COUNTER_NAMES.iter().position(|n| *n == name);
            i.map_or(u32::MAX, |i| self.core.counters()[i])
        }

        /// One server tick at `now`: the session's messages through the decoder into the room, the tick, then the
        /// room's datagrams and reliable messages (each on its own stream: npt-proto's `channel_of`) to the client.
        fn server_tick(&mut self, now: u64) {
            let tick = self.room.last_tick().wrapping_add(1);
            loop {
                let mut msgs = Vec::new();
                self.decoder.pump(&mut self.server, now, tick, &mut msgs);
                let mut joined = false;
                for (m, _) in msgs {
                    match (&m, self.slot) {
                        (Message::Join(j), _) => {
                            joined = true;
                            match self.room.join(j) {
                                Ok((slot, _)) => {
                                    self.decoder.accept();
                                    self.slot = Some(slot);
                                }
                                Err(r) => self.decoder.reject(&mut self.server, r, now),
                            }
                        }
                        (_, Some(slot)) => self.room.receive(slot, &m),
                        _ => {}
                    }
                }
                if !joined {
                    break;
                }
            }
            if let Some(slot) = self.slot {
                let sent = self.decoder.take_sent_bytes();
                self.room
                    .pump_report(slot, sent, self.server.max_datagram_size());
            }
            self.room.tick();
            self.refusals.extend_from_slice(self.room.refusals());
            if let Some(slot) = self.slot {
                for d in self.room.take_datagrams(slot) {
                    self.server.send_unreliable(&d, now);
                }
                for b in self.room.take_control(slot) {
                    let channel = npt_proto::channel_of(&b).expect("a framed message");
                    self.server.send_reliable(channel, &b, now);
                    self.from_room.push((channel, b));
                }
            }
        }

        /// One wake of the client at `now`: what arrived, packed as C5's inbox; the outbox sent `lag_ns` later (the
        /// client's `Delayed` adds it), which is the send time the next call gets.
        fn wake(&mut self, now: u64) {
            let mut got = Vec::new();
            while let Some(r) = self.client.poll_recv(now) {
                let tag = match r.channel {
                    Channel::Datagram => 0,
                    Channel::Control => 1,
                    Channel::FullState => 2,
                };
                got.push((tag, r.arrival_ns, r.bytes));
            }
            let packed: Vec<(u8, u64, &[u8])> =
                got.iter().map(|(t, a, b)| (*t, *a, b.as_slice())).collect();
            let out = self.core.step(now, self.prev_send_ns, &inbox(&packed));
            for (channel, bytes) in outbox(&out) {
                if channel == 0 {
                    self.client.send_unreliable(&bytes, now);
                } else {
                    self.client.send_reliable(Channel::Control, &bytes, now);
                }
            }
            self.prev_send_ns = now + self.lag_ns;
            self.wakes += 1;
            let left_due = self.core.next_due_ns() <= now + self.lag_ns;
            self.floor_ns = now + self.lag_ns + if left_due { self.clamp_ns } else { 0 };
        }

        /// Server ticks on U4b's schedule, the client woken at each of its due times between them, until `done` holds
        /// or `max_ticks` ticks have run; whether `done` held.
        fn run(&mut self, max_ticks: u64, done: impl Fn(&RoomHost) -> bool) -> bool {
            while self.k < max_ticks {
                if done(self) {
                    return true;
                }
                let at = BASE_NS + self.k * 1_000_000_000 / u64::from(TICK_HZ);
                loop {
                    let late = (self.late)(self.wakes, self.counter("steps"));
                    let due = (self.core.next_due_ns() + late)
                        .max(self.floor_ns)
                        .max(self.now_ns);
                    if due >= at {
                        break;
                    }
                    self.now_ns = due;
                    self.wake(due);
                }
                self.now_ns = at;
                self.server_tick(at);
                self.k += 1;
            }
            done(self)
        }
    }

    /// R3 (C6; C2's equality): the client's `Join` (npt-proto's version and content hash) is accepted by the room as
    /// the binary builds it; the `JoinAccepted` it sends carries `CONTENT_HASH` and `MODE_HASH`; the client takes a
    /// full state, holds its own soldier (the page's Playing), and goes on acknowledging the room's snapshots (it
    /// files their ticks) while the room runs its commands; nothing refused on either side.
    #[test]
    fn r3_the_client_joins_the_room_behind_the_servers_decoder_and_plays() {
        let mut h = RoomHost::new();
        assert!(
            h.run(300, |h| h.counter("own_soldier") == 1),
            "Playing within 10 s of room time"
        );
        assert_eq!((h.slot, h.counter("joined")), (Some(0), 1));
        let accepted: Vec<_> = (h.from_room.iter())
            .filter(|(ch, _)| *ch == Channel::Control)
            .filter_map(|(_, b)| {
                match npt_proto::decode(Direction::RoomToClient, Channel::Control, b) {
                    Ok(Message::JoinAccepted(a)) => Some(a),
                    _ => None,
                }
            })
            .collect();
        assert_eq!(accepted.len(), 1);
        assert_eq!(
            (
                accepted[0].slot,
                accepted[0].content_hash,
                accepted[0].mode_hash
            ),
            (0, CONTENT_HASH, MODE_HASH)
        );
        assert!(h.from_room.iter().any(|(ch, _)| *ch == Channel::FullState));
        let (playing_at, acked_then) = (h.k, h.room.client(0).and_then(|c| c.last_acked()));
        h.run(playing_at + 60, |_| false);
        assert_eq!(h.k, playing_at + 60);
        let acked = h.room.client(0).and_then(|c| c.last_acked());
        assert!(
            acked > acked_then && acked.is_some(),
            "{acked_then:?} -> {acked:?}"
        );
        assert!(h.counter("clock_samples") > 0 && h.counter("last_run_command") > 0);
        assert!(h.room.client(0).is_some_and(|c| c.commands_run() > 0));
        for name in [
            "inbox_refused",
            "pending_dropped",
            "wrong_channel",
            "refused",
        ] {
            assert_eq!(h.counter(name), 0, "{name}");
        }
        assert!(h.refusals.is_empty(), "{:?}", h.refusals);
    }

    /// R3 (C6): the press script's commands over 3,000 client steps draw no refusal from the room (no kind 1-6, 8 or
    /// 10 record, every refusal counter of the slot 0) and are run; only the sim's own (cooldown) may refuse a press.
    #[test]
    fn r3_the_press_script_draws_no_refusal_but_the_sims_own_over_3_000_steps() {
        let mut h = RoomHost::new();
        assert!(h.run(3_400, |h| h.counter("steps") >= 3_000), "3,000 steps");
        no_room_refusal(&h);
        let c = h.room.client(0).expect("slot 0").counters();
        assert!(c.commands_run >= 2_900, "{}", c.commands_run);
        assert!(h.counter("commands") >= 2_900);
    }

    /// No refusal from the room (no kind 1-6, 8 or 10 record, and the slot's eight refusal counters 0), and none of
    /// the client's own (unencodable commands, decode refusals).
    fn no_room_refusal(h: &RoomHost) {
        let c = h.room.client(0).expect("slot 0").counters();
        assert!(
            h.refusals.is_empty(),
            "{:?}",
            &h.refusals[..h.refusals.len().min(8)]
        );
        assert_eq!(
            [
                c.lead_refused,
                c.not_newer,
                c.seq_refused,
                c.move_refused,
                c.press_refused,
                c.aim_refused,
                c.buffer_full,
                c.ack_refused,
            ],
            [0; 8]
        );
        assert_eq!(
            (h.counter("unencodable_commands"), h.counter("refused")),
            (0, 0)
        );
    }

    /// U7R-NC-1 (C-U7-3 through the Core): over a 40 ms path each way, with the host sending 5 ms after each wake, the
    /// client plays with no refusal and its newest clock sample's RTT is the path's 80 ms, within the room's 1 µs hold
    /// rounding: the Core gives the endpoint the host's send time (without it, the wire stamp: 85 ms). Jitter and loss
    /// stay with the endpoint's lab suites: the Core adds no logic that depends on them.
    #[test]
    fn r3_over_a_40_ms_path_with_a_5_ms_send_lag_the_clock_reads_80_ms() {
        let mut h = RoomHost::with_path(40 * MS, 40 * MS, 5 * MS, |_, _| 0);
        assert!(h.run(300, |h| h.counter("own_soldier") == 1), "Playing");
        h.run(h.k + 90, |_| false);
        no_room_refusal(&h);
        let rtt = h.core.endpoint.clock().samples().back().map(|s| s.rtt_ns);
        assert!(
            rtt.is_some_and(|r| r.abs_diff(80 * MS) <= 1_000),
            "{rtt:?} ns"
        );
    }

    /// U7R-TE-3 (with U7R-NC-2): wakes late as a busy page's are (every 10th 200 ms late, one 1 s stall at wake 400,
    /// one 3 s gap at wake 900), each catch-up wake at most `LOOP_CATCHUP` steps, under the browser's 4 ms timer clamp
    /// and with none (a host that re-wakes at once). The room refuses nothing (kinds 5 and 10 among them) and the
    /// client plays on. A gap of 2 s or more empties the clock's 2 s window, so the first command after it covers the
    /// whole gap; under the clamp a catch-up spans one fire period at most. NC-2's double edge needs the narrow case
    /// the last leg builds: 100 ms each way (a pong landing inside the stall keeps one sample), no clamp, a 2.1 s
    /// stall at step 659 (59 of a 60-step fire period): about 63 steps, two fire edges, between two commands one tick
    /// apart. Presses counted by steps draw kind 5 there; counted by commands, none.
    #[test]
    fn r3_late_wakes_a_1_s_stall_and_a_3_s_gap_draw_no_refusal() {
        let busy = |w: u64, _: u32| match w {
            400 => 1_000 * MS,
            900 => 3_000 * MS,
            w if w % 10 == 9 => 200 * MS,
            _ => 0,
        };
        let edge = |_: u64, s: u32| if s == 659 { 2_100 * MS } else { 0 };
        for (path_ns, late, clamp_ns) in [
            (0, busy as fn(u64, u32) -> u64, TIMER_CLAMP_NS),
            (0, busy, 0),
            (100 * MS, edge, 0),
        ] {
            let mut h = RoomHost::with_path(path_ns, path_ns, 0, late);
            h.clamp_ns = clamp_ns;
            assert!(h.run(3_000, |h| h.wakes > 1_200), "1,200 wakes");
            no_room_refusal(&h);
            assert_eq!(h.counter("own_soldier"), 1);
        }
    }

    /// U7R-TE-7: a datagram the endpoint cannot decode, in the inbox, counts in `refused`, read by its name.
    #[test]
    fn r3_a_malformed_datagram_counts_in_refused_by_name() {
        let mut c = core();
        c.step(0, 0, &[]);
        let next = 1_000_000_000 / u64::from(TICK_HZ);
        c.step(next, 0, &inbox(&[(0, 1, &[0xff; 9])]));
        let i = COUNTER_NAMES.iter().position(|n| *n == "refused");
        assert_eq!(i.map(|i| c.counters()[i]), Some(1));
    }
}
