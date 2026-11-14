//! The WebSocket request and frame scanner DelayStream runs on both directions' plaintext (u3c-plan.md 1.2, C6).
//!
//! It finds the boundaries DelayStream cuts records at (the upgrade request or response, then each frame, RFC 6455
//! §5.2) and, on the client's direction, applies the size limits before any of a frame's payload is buffered (plan
//! §4.4: limits before any delay buffering). A client's stream closes, with a counted [`Reason`], on:
//! - a frame whose payload, or a fragmented message's running payload total, is above the limit (`ws_max_message_bytes`,
//!   the catalogue's 513 B);
//! - an unmasked frame (§5.1: a client masks every frame);
//! - a control frame above 125 B, or a fragmented one (§5.5);
//! - a 64-bit length with its most significant bit set (§5.2);
//! - an upgrade request longer than its cap (`ws_max_request_bytes`, 8,192 B) without its blank line.
//!
//! The request ends exactly where tungstenite's parser ends it (S2), so the limits apply to the very bytes it then
//! reads as frames: tungstenite 0.30 parses the request with httparse 1.10.1 (handshake/server.rs:133-140), which
//! skips leading empty lines, ends every line at an LF (a CR is only allowed right before one; any other place errs)
//! and ends the head at the first empty line, an LF or a CR LF, after a non-empty line (httparse lib.rs:558-581,
//! :1105-1116, macros.rs:55-67). For every request httparse completes, the scanner's head ends at the same byte;
//! where httparse errs, tungstenite refuses the handshake, whatever the scanner decided. The module tests check it
//! against tungstenite's own `TryParse` on seeded inputs.
//!
//! Anything else (reserved bits or opcodes, a continuation with no message open, UTF-8) is left to tungstenite, which
//! closes on it. The server's direction is scanned the same way for its message ends (the per-return record's "first
//! whole message", S12), with no limit applied: tungstenite writes it. The scanner keeps a fixed state (at most a
//! 14 B header) and never allocates; it is a fuzz target at U5 (-34).

/// Why a client's stream was closed (each its own counter: `ws_close_<reason>_count`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Reason {
    /// The upgrade request reached its cap without its blank line.
    Request = 0,
    /// A frame's payload above the limit.
    Frame = 1,
    /// A fragmented message's running payload total above the limit.
    Message = 2,
    /// A client frame without a mask (§5.1).
    Unmasked = 3,
    /// A control frame above 125 B, or fragmented (§5.5).
    Control = 4,
    /// A 64-bit payload length with its most significant bit set (§5.2).
    Length = 5,
}

pub(crate) const REASONS: usize = 6;

impl Reason {
    pub(crate) fn text(self) -> &'static str {
        match self {
            Reason::Request => "the upgrade request is above its cap",
            Reason::Frame => "a frame is above the message limit",
            Reason::Message => "a fragmented message is above the message limit",
            Reason::Unmasked => "an unmasked client frame",
            Reason::Control => "a control frame above 125 B or fragmented",
            Reason::Length => "a 64-bit length with its top bit set",
        }
    }
}

/// A boundary the scanner found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Boundary {
    /// The end of the upgrade request (client) or response (server): its blank line.
    Head,
    /// The end of a frame; `message_end` when it ends a data message (FIN on a text, binary or continuation frame).
    Frame { message_end: bool },
}

/// Where the HTTP head's scan stands, by httparse's line rules (S2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Line {
    /// Before any non-empty line (httparse skips these empty lines).
    Leading,
    /// A CR there.
    LeadingCr,
    /// Inside a non-empty line.
    Content,
    /// At the start of a line after a non-empty one: an empty line here ends the head.
    Start,
    /// A CR there: an LF next ends the head.
    StartCr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// In the HTTP head: its length so far and where its lines stand.
    Head { len: u64, line: Line },
    /// In a frame header: the bytes so far.
    Header { buf: [u8; 14], have: usize },
    /// In a frame's payload: bytes left, and whether the frame ends a data message.
    Payload { left: u64, message_end: bool },
    /// After a close: nothing more is scanned.
    Closed,
}

/// The largest client frame header: 2 B, an 8 B extended length, a 4 B mask key.
pub(crate) const MAX_HEADER: usize = 14;
/// RFC 6455 §5.5: a control frame's payload is at most 125 B.
const MAX_CONTROL: u64 = 125;

/// One direction's scanner.
#[derive(Debug, Clone)]
pub(crate) struct Scanner {
    /// The client's direction: limits applied, masks required.
    client: bool,
    max_payload: u64,
    max_head: u64,
    state: State,
    /// The open data message's payload so far (a fragmented message's running total).
    message: u64,
    /// The tests' bites (M2): the checks turned off, by reason.
    #[cfg(test)]
    off: [bool; REASONS],
}

impl Scanner {
    /// The client's direction: `max_payload` per frame and per message, `max_head` for the request.
    pub(crate) fn client(max_payload: u64, max_head: u64) -> Scanner {
        Scanner {
            client: true,
            max_payload,
            max_head,
            state: State::Head {
                len: 0,
                line: Line::Leading,
            },
            message: 0,
            #[cfg(test)]
            off: [false; REASONS],
        }
    }

    /// The server's direction: boundaries only, no limit.
    pub(crate) fn server() -> Scanner {
        Scanner {
            client: false,
            max_payload: u64::MAX,
            max_head: u64::MAX,
            state: State::Head {
                len: 0,
                line: Line::Leading,
            },
            message: 0,
            #[cfg(test)]
            off: [false; REASONS],
        }
    }

    /// The tests' bite (M2): the check for `r` off from now on.
    #[cfg(test)]
    pub(crate) fn set_off(&mut self, r: Reason) {
        self.off[r as usize] = true;
    }

    /// Whether the client direction's check for `r` runs: always, but for a test's bite.
    fn check(&self, r: Reason) -> bool {
        #[cfg(test)]
        if self.off[r as usize] {
            return false;
        }
        let _ = r;
        self.client
    }

    /// Whether the scanner is still in the HTTP head.
    pub(crate) fn in_head(&self) -> bool {
        matches!(self.state, State::Head { .. })
    }

    /// Scans `input` up to the first boundary or close: returns the bytes consumed and what ended the scan (None: all
    /// of `input` consumed with no boundary). After a close it consumes nothing.
    pub(crate) fn scan(&mut self, input: &[u8]) -> (usize, Option<Result<Boundary, Reason>>) {
        let request_cap = self.check(Reason::Request);
        let mut i = 0;
        while i < input.len() {
            match &mut self.state {
                State::Closed => return (i, None),
                State::Head { len, line } => {
                    let b = input[i];
                    i += 1;
                    *len += 1;
                    let (next, end) = match (*line, b) {
                        (Line::Leading | Line::LeadingCr, b'\n') => (Line::Leading, false),
                        (Line::Leading, b'\r') => (Line::LeadingCr, false),
                        (Line::Start | Line::StartCr, b'\n') => (Line::Start, true),
                        (Line::Start, b'\r') => (Line::StartCr, false),
                        (_, b'\n') => (Line::Start, false),
                        // A CR inside a line, or one not followed by an LF where httparse errs: content either way.
                        _ => (Line::Content, false),
                    };
                    *line = next;
                    if end {
                        self.state = State::Header {
                            buf: [0; MAX_HEADER],
                            have: 0,
                        };
                        return (i, Some(Ok(Boundary::Head)));
                    }
                    if *len >= self.max_head && request_cap {
                        self.state = State::Closed;
                        return (i, Some(Err(Reason::Request)));
                    }
                }
                State::Header { buf, have } => {
                    buf[*have] = input[i];
                    *have += 1;
                    i += 1;
                    if *have < 2 {
                        continue;
                    }
                    let masked = buf[1] & 0x80 != 0;
                    let len7 = buf[1] & 0x7f;
                    let need =
                        2 + match len7 {
                            126 => 2,
                            127 => 8,
                            _ => 0,
                        } + if masked { 4 } else { 0 };
                    if *have < need {
                        continue;
                    }
                    let (b0, buf) = (buf[0], *buf);
                    match self.header(b0, masked, len7, &buf) {
                        Ok(Some(b)) => return (i, Some(Ok(b))),
                        Ok(None) => {}
                        Err(r) => {
                            self.state = State::Closed;
                            return (i, Some(Err(r)));
                        }
                    }
                }
                State::Payload { left, message_end } => {
                    let take = (*left).min((input.len() - i) as u64);
                    i += take as usize;
                    *left -= take;
                    if *left == 0 {
                        let message_end = *message_end;
                        self.state = State::Header {
                            buf: [0; MAX_HEADER],
                            have: 0,
                        };
                        return (i, Some(Ok(Boundary::Frame { message_end })));
                    }
                }
            }
        }
        (i, None)
    }

    /// A complete header: the checks, then the payload state (or the frame's end at once for an empty payload).
    fn header(
        &mut self,
        b0: u8,
        masked: bool,
        len7: u8,
        buf: &[u8; MAX_HEADER],
    ) -> Result<Option<Boundary>, Reason> {
        let fin = b0 & 0x80 != 0;
        let opcode = b0 & 0x0f;
        let len = match len7 {
            126 => u64::from(u16::from_be_bytes([buf[2], buf[3]])),
            127 => {
                let mut b = [0u8; 8];
                b.copy_from_slice(&buf[2..10]);
                let v = u64::from_be_bytes(b);
                if v >> 63 != 0 && self.check(Reason::Length) {
                    return Err(Reason::Length);
                }
                v
            }
            n => u64::from(n),
        };
        let control = opcode & 0x08 != 0;
        if !masked && self.check(Reason::Unmasked) {
            return Err(Reason::Unmasked);
        }
        if control && (len > MAX_CONTROL || !fin) && self.check(Reason::Control) {
            return Err(Reason::Control);
        }
        if len > self.max_payload && self.check(Reason::Frame) {
            return Err(Reason::Frame);
        }
        let mut message_end = false;
        if !control {
            // A text or binary frame opens a message; a continuation adds to it (tungstenite closes on one with no
            // message open).
            self.message = if opcode == 0 {
                self.message.saturating_add(len)
            } else {
                len
            };
            if self.message > self.max_payload && self.check(Reason::Message) {
                return Err(Reason::Message);
            }
            if fin {
                message_end = true;
                self.message = 0;
            }
        }
        if len == 0 {
            self.state = State::Header {
                buf: [0; MAX_HEADER],
                have: 0,
            };
            return Ok(Some(Boundary::Frame { message_end }));
        }
        self.state = State::Payload {
            left: len,
            message_end,
        };
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand_core::{Rng, SeedableRng};

    const LIMIT: u64 = 513;
    const HEAD: u64 = 8_192;

    /// A client frame: masked (or not), with the shortest length encoding unless `long` forces the 64-bit one.
    fn frame(fin: bool, opcode: u8, masked: bool, payload: usize, long: bool) -> Vec<u8> {
        let mut v = vec![(u8::from(fin) << 7) | opcode];
        let m = if masked { 0x80 } else { 0 };
        if long {
            v.push(m | 127);
            v.extend((payload as u64).to_be_bytes());
        } else if payload < 126 {
            v.push(m | payload as u8);
        } else if payload <= 0xffff {
            v.push(m | 126);
            v.extend((payload as u16).to_be_bytes());
        } else {
            v.push(m | 127);
            v.extend((payload as u64).to_be_bytes());
        }
        if masked {
            v.extend([1, 2, 3, 4]);
        }
        v.extend(std::iter::repeat_n(0x5a, payload));
        v
    }

    const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\n\r\n";

    /// Every boundary and close in `input`, fed in chunks of `chunk` bytes, with the offsets they end at.
    fn all(s: &mut Scanner, input: &[u8], chunk: usize) -> Vec<(usize, Result<Boundary, Reason>)> {
        let mut out = Vec::new();
        let mut at = 0;
        for c in input.chunks(chunk.max(1)) {
            let mut i = 0;
            while i < c.len() {
                let (n, ev) = s.scan(&c[i..]);
                i += n;
                if let Some(e) = ev {
                    out.push((at + i, e));
                    if e.is_err() {
                        return out;
                    }
                } else if n == 0 {
                    return out;
                }
            }
            at += c.len();
        }
        out
    }

    #[test]
    fn the_request_then_each_frame_is_a_boundary_in_any_chunking() {
        let mut input = REQUEST.to_vec();
        let frames = [
            frame(true, 2, true, 513, false),
            frame(false, 2, true, 200, false),
            frame(true, 9, true, 125, false),
            frame(true, 0, true, 313, false),
            frame(true, 1, true, 0, false),
        ];
        let mut ends = vec![REQUEST.len()];
        for f in &frames {
            input.extend(f);
            ends.push(input.len());
        }
        for chunk in [1, 2, 3, 7, 64, 10_000] {
            let mut s = Scanner::client(LIMIT, HEAD);
            let got = all(&mut s, &input, chunk);
            let at: Vec<usize> = got.iter().map(|g| g.0).collect();
            assert_eq!(at, ends, "chunk {chunk}");
            let kinds: Vec<Result<Boundary, Reason>> = got.iter().map(|g| g.1).collect();
            assert_eq!(
                kinds,
                vec![
                    Ok(Boundary::Head),
                    Ok(Boundary::Frame { message_end: true }),
                    Ok(Boundary::Frame { message_end: false }),
                    Ok(Boundary::Frame { message_end: false }),
                    Ok(Boundary::Frame { message_end: true }),
                    Ok(Boundary::Frame { message_end: true }),
                ]
            );
        }
    }

    /// Each close reason, found at the header's last byte, before any of the payload.
    #[test]
    fn each_limit_closes_at_the_header() {
        let cases: [(Vec<u8>, Reason); 7] = [
            (frame(true, 2, true, 514, false), Reason::Frame),
            (
                [
                    frame(false, 2, true, 300, false),
                    frame(true, 0, true, 214, false),
                ]
                .concat(),
                Reason::Message,
            ),
            (frame(true, 2, false, 10, false), Reason::Unmasked),
            (frame(true, 9, true, 126, false), Reason::Control),
            (frame(false, 9, true, 5, false), Reason::Control),
            (
                {
                    let mut f = frame(true, 2, true, 0, true);
                    f[2] = 0x80;
                    f
                },
                Reason::Length,
            ),
            (frame(true, 2, true, 70_000, false), Reason::Frame),
        ];
        for (f, why) in cases {
            let mut s = Scanner::client(LIMIT, HEAD);
            let input = [REQUEST, &f].concat();
            let got = all(&mut s, &input, 1);
            let (at, last) = *got.last().unwrap();
            assert_eq!(last, Err(why));
            // The close comes at a header's end: the payload (if any) is never consumed.
            let header_len = |b1: u8| {
                2 + match b1 & 0x7f {
                    126 => 2,
                    127 => 8,
                    _ => 0,
                } + if b1 & 0x80 != 0 { 4 } else { 0 }
            };
            let last_start = if why == Reason::Message {
                REQUEST.len() + frame(false, 2, true, 300, false).len()
            } else {
                REQUEST.len()
            };
            assert_eq!(
                at,
                last_start + header_len(input[last_start + 1]),
                "{why:?}"
            );
            // Nothing more is scanned after a close.
            assert_eq!(s.scan(&[0x82, 0x80, 0, 0, 0, 0]), (0, None));
        }
    }

    /// tungstenite's own head length for `input` (its server handshake's parser, httparse underneath): Some(size)
    /// when the head completes, None while incomplete, Err when refused.
    fn tungstenite_head(input: &[u8]) -> Result<Option<usize>, ()> {
        use tokio_tungstenite::tungstenite::handshake::machine::TryParse;
        use tokio_tungstenite::tungstenite::handshake::server::Request;
        match Request::try_parse(input) {
            Ok(Some((size, _))) => Ok(Some(size)),
            Ok(None) => Ok(None),
            Err(_) => Err(()),
        }
    }

    /// Where the client scanner ends the head in `input` (fed in `chunk`-byte pieces), if it does.
    fn head_end(input: &[u8], chunk: usize) -> Option<usize> {
        let mut s = Scanner::client(LIMIT, HEAD);
        all(&mut s, input, chunk)
            .first()
            .and_then(|&(at, e)| (e == Ok(Boundary::Head)).then_some(at))
    }

    /// S2: bare LF, mixed line ends and leading empty lines end the head where tungstenite ends it, in any chunking;
    /// and the reviewer's input (a 4,096 B head ending in "\n\n" then a frame) ends at 4,096, so the frame after it is
    /// scanned as a frame.
    #[test]
    fn the_head_ends_where_tungstenite_ends_it() {
        let mut lf = b"GET / HTTP/1.1\nHost: localhost\nUpgrade: websocket\nX-Pad: ".to_vec();
        lf.resize(4_094, b'a');
        lf.extend(b"\n\n");
        let heads: [&[u8]; 6] = [
            REQUEST,
            b"GET / HTTP/1.1\n\n",
            b"GET / HTTP/1.1\r\nHost: a\n\r\n",
            b"GET / HTTP/1.1\nHost: a\r\n\n",
            b"\r\n\r\n\nGET / HTTP/1.1\r\nHost: a\r\n\r\n",
            &lf,
        ];
        for head in heads {
            let input = [head, &frame(true, 2, true, 604, false)].concat();
            assert_eq!(
                tungstenite_head(&input),
                Ok(Some(head.len())),
                "{:?}",
                String::from_utf8_lossy(&head[..head.len().min(40)])
            );
            for chunk in [1, 2, 3, 7, 4_096, 10_000] {
                assert_eq!(head_end(&input, chunk), Some(head.len()), "chunk {chunk}");
            }
            let mut s = Scanner::client(LIMIT, HEAD);
            let got = all(&mut s, &input, 4_096);
            assert_eq!(
                got.get(1),
                Some(&(head.len() + 2 + 2 + 4, Err(Reason::Frame))),
                "the frame after the head is closed at its header"
            );
        }
    }

    /// S2's differential: seeded requests with random leading empty lines, line ends (LF or CR LF), headers and
    /// tails, some corrupted (a byte replaced, often a CR, LF or NUL): wherever tungstenite completes the head, the
    /// scanner ends it at the same byte; wherever the scanner ends it, tungstenite on those bytes completes at the same
    /// byte or refuses the request, never waits for more. 20,000 inputs, every case counted.
    #[test]
    fn the_head_matches_tungstenite_on_seeded_inputs() {
        let mut rng = ChaCha8Rng::seed_from_u64(0x6e70_745f_6865_6164);
        let mut below = |n: u64| ((u128::from(rng.next_u64()) * u128::from(n)) >> 64) as u64;
        let (mut both, mut refused, mut incomplete) = (0u64, 0u64, 0u64);
        for case in 0..20_000u64 {
            let eol = |r: u64| -> &'static [u8] { if r == 0 { b"\n" } else { b"\r\n" } };
            let mut input = Vec::new();
            for _ in 0..below(3) {
                input.extend(eol(below(2)));
            }
            input.extend(b"GET /p HTTP/1.1");
            input.extend(eol(below(2)));
            for h in 0..below(12) {
                input.extend(format!("X-H{h}: v").as_bytes());
                for _ in 0..below(20) {
                    input.push(b"abc 123:/=;,"[below(12) as usize]);
                }
                input.extend(eol(below(2)));
            }
            input.extend(eol(below(2)));
            for _ in 0..below(40) {
                input.push(below(256) as u8);
            }
            if below(3) == 0 {
                for _ in 0..1 + below(3) {
                    let at = below(input.len() as u64) as usize;
                    input[at] = [b'\r', b'\n', 0, b' ', b'x'][below(5) as usize];
                }
            }
            let chunk = 1 + below(64) as usize;
            let ours = head_end(&input, chunk);
            let theirs = tungstenite_head(&input);
            if let Ok(Some(size)) = theirs {
                assert_eq!(ours, Some(size), "case {case}: {input:?}");
                both += 1;
            }
            if let Some(s) = ours {
                match tungstenite_head(&input[..s]) {
                    Ok(Some(size)) => assert_eq!(size, s, "case {case}"),
                    Ok(None) => {
                        panic!("case {case}: tungstenite waits past the scanner's end: {input:?}")
                    }
                    Err(()) => refused += 1,
                }
            } else {
                incomplete += 1;
            }
        }
        println!(
            "heads: {both} completed by both at the same byte, {refused} ended by the scanner and refused by \
             tungstenite, {incomplete} not ended"
        );
        assert!(both > 10_000 && refused > 100 && incomplete > 100);
    }

    #[test]
    fn a_request_without_its_blank_line_closes_at_its_cap() {
        let mut s = Scanner::client(LIMIT, 64);
        let long = vec![b'a'; 100];
        let (n, ev) = s.scan(&long);
        assert_eq!((n, ev), (64, Some(Err(Reason::Request))));
        // A request of exactly the cap, ending with its blank line, passes.
        let mut s = Scanner::client(LIMIT, REQUEST.len() as u64);
        assert_eq!(s.scan(REQUEST), (REQUEST.len(), Some(Ok(Boundary::Head))));
    }

    #[test]
    fn the_server_direction_applies_no_limit() {
        let mut s = Scanner::server();
        let resp = b"HTTP/1.1 101 Switching Protocols\r\n\r\n";
        let input = [&resp[..], &frame(true, 2, false, 65_541, false)].concat();
        let got = all(&mut s, &input, 4_096);
        assert_eq!(
            got,
            vec![
                (resp.len(), Ok(Boundary::Head)),
                (input.len(), Ok(Boundary::Frame { message_end: true }))
            ]
        );
    }

    /// C6: the seeded random-input test. Random bytes, and valid streams with random corruption, in random chunks:
    /// the scanner never panics, never consumes more than it is given, never reports a frame (with its header) above
    /// the limit plus 14 B or a head above its cap, and makes progress (a call that consumes nothing has closed).
    #[test]
    fn random_input_never_panics_and_stays_within_its_bounds() {
        let mut rng = ChaCha8Rng::seed_from_u64(0x6e70_745f_6672_616d);
        let mut below = |n: u64| ((u128::from(rng.next_u64()) * u128::from(n)) >> 64) as u64;
        let (mut boundaries, mut closes) = (0u64, [0u64; REASONS]);
        for case in 0..20_000u64 {
            let mut input = Vec::new();
            if case % 2 == 0 {
                let n = below(3_000) as usize;
                input.extend((0..n).map(|_| below(256) as u8));
                if below(2) == 0 {
                    input.splice(0..0, REQUEST.iter().copied());
                }
            } else {
                input.extend(REQUEST);
                for _ in 0..below(20) {
                    let opcode = [0, 1, 2, 8, 9, 10, 3][below(7) as usize];
                    let len = [0, 1, 125, 126, 513, 514, 65_535, 70_000][below(8) as usize];
                    input.extend(frame(
                        below(4) != 0,
                        opcode,
                        below(10) != 0,
                        len.min(600),
                        below(9) == 0,
                    ));
                }
                for _ in 0..below(4) {
                    if !input.is_empty() {
                        let at = below(input.len() as u64) as usize;
                        input[at] = below(256) as u8;
                    }
                }
            }
            let mut s = Scanner::client(LIMIT, 512 + below(HEAD));
            let max_head = s.max_head;
            let mut i = 0;
            let mut since = 0usize;
            while i < input.len() {
                let chunk = 1 + below(700) as usize;
                let end = (i + chunk).min(input.len());
                let (n, ev) = s.scan(&input[i..end]);
                assert!(n <= end - i);
                i += n;
                since += n;
                match ev {
                    Some(Ok(b)) => {
                        boundaries += 1;
                        let cap = match b {
                            Boundary::Head => max_head,
                            Boundary::Frame { .. } => LIMIT + MAX_HEADER as u64,
                        };
                        assert!(
                            since as u64 <= cap,
                            "case {case}: a {b:?} record of {since} B"
                        );
                        since = 0;
                    }
                    Some(Err(r)) => {
                        closes[r as usize] += 1;
                        assert!(
                            since as u64 <= max_head.max(LIMIT + MAX_HEADER as u64),
                            "case {case}: {since} B held before a close"
                        );
                        break;
                    }
                    None => assert!(n > 0 || end == i, "no progress without a close"),
                }
            }
        }
        println!("random input: {boundaries} boundaries, closes by reason {closes:?}");
        assert!(boundaries > 10_000);
        assert!(
            closes.iter().all(|&c| c > 0),
            "every reason reached: {closes:?}"
        );
    }
}
