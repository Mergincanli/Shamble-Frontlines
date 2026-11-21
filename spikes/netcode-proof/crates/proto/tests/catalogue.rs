//! The class audit test, U1 part (plan §4.1; `docs/design/netcode-proof-messages.md` §6): the message catalogue read
//! as data. Every message type has a visibility class ("one with none fails the audit test", ADR 0001's catalogue),
//! every plan §4.7 type is there exactly once, and nothing authority-only is ever sent. Each assertion has a broken
//! copy of the doc that must fail it (the plan's "bites first", §6.1). At U4 the test also reads npt-proto's types,
//! limits and the encoder's layout table against these rows.

use std::collections::BTreeSet;
use std::net::{IpAddr, Ipv4Addr};

use npt_link::Channel;
use npt_proto::{
    ClientSettings, DecodeError, Direction, EndReason, FullState, Input, Join, JoinAccepted,
    JoinRejected, Kick, Leave, Limits, Message, Ping, Pong, Resync, RoomControl, RoomEnded,
    SessionControl, SessionStatus, Snapshot, decode, encode,
};
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};

const DOC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../docs/design/netcode-proof-messages.md"
);

const WIRE: &str =
    "| Type | Direction | Channel | Class | Authority | Size limit | Spike status | Draft |";
const FIELDS: &str = "| Message | Field group | Class | Sent to | Notes |";
const SESSION: &str = "| Variant | Fields | Who may send | Check | Effect | Draft |";
const ROOM: &str = "| Control | Fields | Who | Check | Effect |";

/// Plan §4.7's npt-proto messages and controls (`SessionStatus` by the advisor's A1 ruling, 2026-11-06).
const BUILT: [&str; 15] = [
    "Input",
    "Snapshot",
    "FullState",
    "Resync",
    "Ping",
    "Pong",
    "Join",
    "JoinAccepted",
    "JoinRejected",
    "ClientSettings",
    "RoomEnded",
    "Leave",
    "Kick",
    "SessionControl",
    "SessionStatus",
];
/// The draft's types the spike leaves out, each with its reason.
const OMITTED: [&str; 8] = [
    "RoomState",
    "HostCommand",
    "PickUpgrade",
    "Buy",
    "ContentReload",
    "Lobby calls (HTTPS)",
    "Supervisor calls",
    "Lobby ↔ supervisor",
];
const VARIANTS: [&str; 4] = ["SetProfile", "TriggerBurst", "BlockUdp", "RecordStep"];
const MESSAGE_CLASSES: [&str; 4] = ["n/a", "owner", "public", "per field"];
const GROUP_CLASSES: [&str; 5] = ["owner", "team", "spatial", "public", "authority-only"];

struct Table {
    header: String,
    rows: Vec<Vec<String>>,
}

fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
    inner.split('|').map(|c| c.trim().to_owned()).collect()
}

/// Every table in the doc: a run of lines starting with `|`, the second being the separator.
fn tables(text: &str) -> Vec<Table> {
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if !line.starts_with('|') {
            continue;
        }
        let header = line.trim_end().to_owned();
        let mut rows = Vec::new();
        let mut first = true;
        while let Some(next) = lines.peek() {
            if !next.starts_with('|') {
                break;
            }
            let row = lines.next().unwrap_or_default();
            if first && row.starts_with("|---") {
                first = false;
                continue;
            }
            first = false;
            rows.push(cells(row));
        }
        out.push(Table { header, rows });
    }
    out
}

/// The first text in backticks, or the whole cell.
fn name(cell: &str) -> String {
    let mut parts = cell.split('`');
    match (parts.next(), parts.next()) {
        (Some(_), Some(inner)) => inner.to_owned(),
        _ => cell.to_owned(),
    }
}

fn whole_bytes(size: &str) -> bool {
    // "166 B", "1,000 B …", "65,536 B …": a whole number, commas between thousands, then " B".
    let Some((number, rest)) = size.split_once(" B") else {
        return false;
    };
    let groups: Vec<&str> = number.split(',').collect();
    let digits = |g: &str| !g.is_empty() && g.bytes().all(|b| b.is_ascii_digit());
    let first_ok = digits(groups[0]) && groups[0].len() <= 3 && !groups[0].starts_with('0');
    let rest_ok = groups[1..].iter().all(|g| g.len() == 3 && digits(g));
    let after_ok = rest.is_empty() || rest.starts_with(' ');
    first_ok && rest_ok && after_ok && (groups.len() > 1 || groups[0].len() <= 3)
}

/// Every broken assertion of the catalogue's §6 (U1 part), as a sentence; empty when the catalogue holds.
fn audit(text: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let all = tables(text);
    let find = |header: &str| {
        all.iter()
            .filter(|t| t.header == header)
            .collect::<Vec<_>>()
    };
    // 1. Exactly the four tables, each once.
    for h in [WIRE, FIELDS, SESSION, ROOM] {
        let n = find(h).len();
        if n != 1 {
            problems.push(format!("1: the table {h} appears {n} times, not once"));
        }
    }
    for t in &all {
        if ![WIRE, FIELDS, SESSION, ROOM].contains(&t.header.as_str()) {
            problems.push(format!("1: an unknown table {}", t.header));
        }
    }
    if let [wire] = find(WIRE).as_slice() {
        let mut built = Vec::new();
        for (i, r) in wire.rows.iter().enumerate() {
            if r.len() != 8 {
                problems.push(format!("wire row {}: {} cells, not 8", i + 1, r.len()));
                continue;
            }
            let ty = name(&r[0]);
            let (dir, chan, class, auth, size, status, draft) =
                (&r[1], &r[2], &r[3], &r[4], &r[5], &r[6], &r[7]);
            // 4. Every row has a class from the vocabulary.
            if !MESSAGE_CLASSES.contains(&class.as_str()) {
                problems.push(format!(
                    "4: {ty} has class {class:?}, not one of {MESSAGE_CLASSES:?}"
                ));
            }
            if status == "built in U4" {
                built.push(ty.clone());
                // 3. A built row's direction, channel, authority, size and draft.
                if !["client → room", "room → client", "both ways"].contains(&dir.as_str()) {
                    problems.push(format!(
                        "3: {ty}'s direction {dir:?} is not in the vocabulary"
                    ));
                }
                if !["datagram", "control stream", "full-state stream"].contains(&chan.as_str()) {
                    problems.push(format!(
                        "3: {ty}'s channel {chan:?} is not in the vocabulary"
                    ));
                }
                let names_a_side = ["npt-server", "npt-room", "npt-client"]
                    .iter()
                    .any(|s| auth.contains(s));
                if !auth.starts_with("Sender:") || !names_a_side {
                    problems.push(format!(
                        "3: {ty}'s authority does not start with Sender: and name a side"
                    ));
                }
                if !whole_bytes(size) {
                    problems.push(format!(
                        "3: {ty}'s size limit {size:?} does not start with whole bytes"
                    ));
                }
                if draft.is_empty() {
                    problems.push(format!("3: {ty} names no draft"));
                }
            } else if let Some(reason) = status.strip_prefix("omitted:") {
                if reason.trim().is_empty() {
                    problems.push(format!("5: {ty} is omitted with no reason"));
                }
                if !auth.starts_with("Draft:") || size != "n/a (not built)" {
                    problems.push(format!(
                        "5: {ty}'s omitted row does not quote the draft or says it is sized"
                    ));
                }
                if !OMITTED.contains(&ty.as_str()) {
                    problems.push(format!(
                        "5: {ty} is omitted but not one of the draft's types the spike leaves out"
                    ));
                }
            } else {
                problems.push(format!(
                    "2: {ty}'s status {status:?} is neither built in U4 nor omitted"
                ));
            }
        }
        // 2. Plan §4.7's types, each exactly once and built; nothing else built.
        for b in BUILT {
            let n = built.iter().filter(|t| *t == b).count();
            if n != 1 {
                problems.push(format!("2: {b} is built {n} times, not once"));
            }
        }
        for t in &built {
            if !BUILT.contains(&t.as_str()) {
                problems.push(format!("2: {t} is built but not in plan §4.7"));
            }
        }
        // 5. Each omitted type present.
        let names: BTreeSet<String> = wire
            .rows
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| name(&r[0]))
            .collect();
        for o in OMITTED {
            if !names.contains(o) {
                problems.push(format!("5: the omitted type {o} is missing"));
            }
        }
        // 6. Field groups: of built per-field types, every such type covered, the classes, and authority-only = nobody.
        let per_field: BTreeSet<String> = wire
            .rows
            .iter()
            .filter(|r| r.len() == 8 && r[3] == "per field" && r[6] == "built in U4")
            .map(|r| name(&r[0]))
            .collect();
        if let [fields] = find(FIELDS).as_slice() {
            let mut covered = BTreeSet::new();
            for r in &fields.rows {
                if r.len() != 5 {
                    problems.push(format!(
                        "6: a field-group row with {} cells, not 5",
                        r.len()
                    ));
                    continue;
                }
                let msg = name(&r[0]);
                let (group, class, to) = (&r[1], &r[2], &r[3]);
                if !per_field.contains(&msg) {
                    problems.push(format!(
                        "6: {msg} has field groups but is not a built per-field type"
                    ));
                }
                covered.insert(msg.clone());
                if !GROUP_CLASSES.contains(&class.as_str()) {
                    problems.push(format!(
                        "6: {msg} / {group} has class {class:?}, not one of {GROUP_CLASSES:?}"
                    ));
                }
                if (class == "authority-only") != (to == "nobody") {
                    problems.push(format!(
                        "6: {msg} / {group}: authority-only and sent to nobody must go together"
                    ));
                }
            }
            for m in &per_field {
                if !covered.contains(m) {
                    problems.push(format!("6: {m} is per field but has no field groups"));
                }
                for g in ["weapon spread stream", "RNG state"] {
                    let ok = fields.rows.iter().any(|r| {
                        r.len() == 5 && name(&r[0]) == *m && r[1] == g && r[2] == "authority-only"
                    });
                    if !ok {
                        problems.push(format!("6: {m} does not list the {g} as authority-only"));
                    }
                }
            }
        }
    }
    // 7. The session controls: exactly plan §4.7's variants, each carrying the capability.
    if let [session] = find(SESSION).as_slice() {
        let got: Vec<String> = session
            .rows
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| name(&r[0]))
            .collect();
        if got.iter().map(String::as_str).collect::<Vec<_>>() != VARIANTS {
            problems.push(format!(
                "7: the session controls are {got:?}, not {VARIANTS:?}"
            ));
        }
        for r in &session.rows {
            if r.len() != 6 || !r[1].starts_with("`cap`") {
                problems.push(format!(
                    "7: {} does not carry the capability first",
                    name(&r[0])
                ));
            }
        }
    }
    // 8. Every room-wide control is driven by `npt` alone.
    if let [room] = find(ROOM).as_slice() {
        for r in &room.rows {
            if r.len() != 5 || !r[2].contains("`npt`") {
                problems.push(format!(
                    "8: room-wide control {} does not name `npt` as who drives it",
                    name(&r[0])
                ));
            }
        }
    }
    problems
}

fn doc() -> String {
    std::fs::read_to_string(DOC).expect("the catalogue is at docs/design/netcode-proof-messages.md")
}

#[test]
fn the_catalogue_holds() {
    let problems = audit(&doc());
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// One broken copy per assertion: each must fail, naming the right assertion.
#[test]
fn every_assertion_bites() {
    let text = doc();
    let row = |ty: &str| {
        text.lines()
            .find(|l| l.starts_with(&format!("| `{ty}` |")))
            .unwrap_or_else(|| panic!("the {ty} row"))
            .to_owned()
    };
    let field_row = |msg: &str, group: &str| {
        text.lines()
            .find(|l| l.starts_with(&format!("| `{msg}` | {group} |")))
            .unwrap_or_else(|| panic!("the {msg} / {group} row"))
            .to_owned()
    };
    let replace_cell = |line: &str, index: usize, value: &str| {
        let mut c = cells(line);
        c[index] = value.to_owned();
        format!("| {} |", c.join(" | "))
    };
    let input = row("Input");
    let room_state = row("RoomState");
    let kick = row("Kick");
    let status = row("SessionStatus");
    let spread = field_row("Snapshot", "weapon spread stream");
    let own = field_row("Snapshot", "own state");
    let set_profile = row("SetProfile");
    let mint = row("MintToken");
    let cases: Vec<(&str, String)> = vec![
        ("4:", text.replace(&input, &replace_cell(&input, 3, ""))),
        (
            "5:",
            text.replace(&room_state, &replace_cell(&room_state, 6, "omitted: ")),
        ),
        ("2:", text.replace(&format!("{kick}\n"), "")),
        ("2:", text.replace(&format!("{status}\n"), "")),
        (
            "2:",
            text.replace(&format!("{kick}\n"), &format!("{kick}\n{kick}\n")),
        ),
        (
            "3:",
            text.replace(&input, &replace_cell(&input, 5, "about 166 B")),
        ),
        (
            "3:",
            text.replace(&input, &replace_cell(&input, 2, "carrier pigeon")),
        ),
        (
            "3:",
            text.replace(&input, &replace_cell(&input, 4, "anyone may send it")),
        ),
        (
            "1:",
            text.replace(FIELDS, "| Message | Group | Class | Sent to | Notes |"),
        ),
        (
            "1:",
            format!("{text}\n| Extra | table |\n|---|---|\n| a | b |\n"),
        ),
        ("6:", text.replace(&own, &replace_cell(&own, 2, "private"))),
        (
            "6:",
            text.replace(&spread, &replace_cell(&spread, 2, "spatial")),
        ),
        ("6:", text.replace(&own, &replace_cell(&own, 3, "nobody"))),
        (
            "7:",
            text.replace(
                &set_profile,
                &replace_cell(&set_profile, 1, "`profile` (1 B)"),
            ),
        ),
        ("7:", text.replace(&format!("{set_profile}\n"), "")),
        (
            "8:",
            text.replace(&mint, &replace_cell(&mint, 2, "anyone on the internet")),
        ),
    ];
    for (expect, broken) in &cases {
        assert_ne!(
            broken, &text,
            "the broken copy for {expect} changed nothing"
        );
        let problems = audit(broken);
        assert!(
            problems.iter().any(|p| p.starts_with(expect)),
            "a copy broken for assertion {expect} passed: {problems:?}"
        );
    }
}

/// A size cell's leading whole bytes (`1,000 B …` is 1,000).
fn leading_bytes(size: &str) -> Option<usize> {
    let (number, _) = size.split_once(" B")?;
    number.replace(',', "").parse().ok()
}

/// The doc's words for npt-proto's channel and direction values.
fn channel_word(c: Channel) -> &'static str {
    match c {
        Channel::Datagram => "datagram",
        Channel::Control => "control stream",
        Channel::FullState => "full-state stream",
    }
}

fn direction_word(d: Direction) -> &'static str {
    match d {
        Direction::ClientToRoom => "client → room",
        Direction::RoomToClient => "room → client",
        Direction::BothWays => "both ways",
    }
}

/// The U4 part of the audit (catalogue §6, "At U4": items 1, 2, 3's size half and 6), the doc read against npt-proto's
/// own tables: every broken assertion as a sentence starting with its item, empty when they hold.
fn audit_u4(text: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let all = tables(text);
    let find = |header: &str| all.iter().find(|t| t.header == header);
    let built: Vec<&Vec<String>> = find(WIRE)
        .map(|t| {
            t.rows
                .iter()
                .filter(|r| r.len() == 8 && r[6] == "built in U4")
                .collect()
        })
        .unwrap_or_default();
    let doc_names: BTreeSet<String> = built.iter().map(|r| name(&r[0])).collect();
    let code_names: BTreeSet<String> = Limits::TYPES.iter().map(|t| t.0.to_owned()).collect();
    // 1. The message types, both ways round; and the session controls' variants.
    for n in doc_names.difference(&code_names) {
        problems.push(format!("u4-1: the built row {n} has no npt-proto type"));
    }
    for n in code_names.difference(&doc_names) {
        problems.push(format!("u4-1: npt-proto's type {n} has no built row"));
    }
    let doc_variants: Vec<String> = find(SESSION)
        .map(|t| {
            t.rows
                .iter()
                .filter(|r| !r.is_empty())
                .map(|r| name(&r[0]))
                .collect()
        })
        .unwrap_or_default();
    if doc_variants.iter().map(String::as_str).collect::<Vec<_>>() != SessionControl::VARIANTS {
        problems.push(format!(
            "u4-1: the session controls are {doc_variants:?}, npt-proto's SessionControl {:?}",
            SessionControl::VARIANTS
        ));
    }
    for (ty, channel, direction, limit) in Limits::TYPES {
        let Some(r) = built.iter().find(|r| name(&r[0]) == ty) else {
            continue;
        };
        // 2. Each type's channel and direction.
        if r[2] != channel_word(channel) {
            problems.push(format!(
                "u4-2: {ty} travels on {:?} in npt-proto, on {:?} in the doc",
                channel_word(channel),
                r[2]
            ));
        }
        if r[1] != direction_word(direction) {
            problems.push(format!(
                "u4-2: {ty} goes {:?} in npt-proto, {:?} in the doc",
                direction_word(direction),
                r[1]
            ));
        }
        // 3. Each type's limit: the Size limit column's leading number; for `Snapshot` also the receive limit its
        // cell names.
        if leading_bytes(&r[5]) != Some(limit) {
            problems.push(format!(
                "u4-3: {ty}'s limit is {limit} B in npt-proto, {:?} in the doc",
                r[5]
            ));
        }
        if ty == "Snapshot"
            && !r[5].contains(&format!(
                "refuses a datagram above {} B",
                thousands(Limits::SNAPSHOT_RECEIVE)
            ))
        {
            problems.push(format!(
                "u4-3: the doc's Snapshot cell does not name npt-client's {} B",
                Limits::SNAPSHOT_RECEIVE
            ));
        }
    }
    // 6. The room-wide controls: the doc's rows are npt-proto's RoomControl, npt-load-ctl apart (its row says it is not
    // a room socket), and their size limit is RoomControl's.
    let doc_controls: Vec<String> = find(ROOM)
        .map(|t| {
            t.rows
                .iter()
                .filter(|r| !r.is_empty())
                .map(|r| name(&r[0]))
                .filter(|n| n != "npt-load-ctl")
                .collect()
        })
        .unwrap_or_default();
    if doc_controls.iter().map(String::as_str).collect::<Vec<_>>() != RoomControl::NAMES {
        problems.push(format!(
            "u4-6: the room-wide controls are {doc_controls:?}, npt-proto's {:?}",
            RoomControl::NAMES
        ));
    }
    let size_line = format!(
        "**Size:** at most {} B a message",
        thousands(Limits::ROOM_CONTROL)
    );
    if !text.contains(&size_line) {
        problems.push(format!(
            "u4-6: the doc's room-wide controls do not say {size_line:?}"
        ));
    }
    problems
}

/// `n` with commas between thousands, as the doc writes sizes.
fn thousands(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[test]
fn u4_items_1_2_3_and_6_hold_against_npt_proto() {
    let problems = audit_u4(&doc());
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// One broken copy of the doc per U4 assertion: each must fail, naming its item (the plan's "bites first", §6.1; a7's
/// broken-copy bites).
#[test]
fn u4_every_assertion_bites() {
    let text = doc();
    let row = |ty: &str| {
        text.lines()
            .find(|l| l.starts_with(&format!("| `{ty}` |")))
            .unwrap_or_else(|| panic!("the {ty} row"))
            .to_owned()
    };
    let replace_cell = |line: &str, index: usize, value: &str| {
        let mut c = cells(line);
        c[index] = value.to_owned();
        format!("| {} |", c.join(" | "))
    };
    let (input, ping, pong, kick, burst, block) = (
        row("Input"),
        row("Ping"),
        row("Pong"),
        row("Kick"),
        row("PeerBurst"),
        row("BlockUdp"),
    );
    let cases: Vec<(&str, String)> = vec![
        ("u4-1:", text.replace(&format!("{kick}\n"), "")),
        ("u4-1:", text.replace("| `Kick` |", "| `Boot` |")),
        (
            "u4-1:",
            text.replace(&block, &block.replace("`BlockUdp`", "`BlockTcp`")),
        ),
        (
            "u4-2:",
            text.replace(&input, &replace_cell(&input, 2, "control stream")),
        ),
        (
            "u4-2:",
            text.replace(&pong, &replace_cell(&pong, 1, "room → client")),
        ),
        (
            "u4-3:",
            text.replace(&ping, &ping.replace("| 32 B (proposed", "| 33 B (proposed")),
        ),
        (
            "u4-3:",
            text.replace(
                "refuses a datagram above 1,150 B",
                "refuses a datagram above 1,151 B",
            ),
        ),
        (
            "u4-6:",
            text.replace(&burst, &burst.replace("`PeerBurst`", "`PeerBlast`")),
        ),
        (
            "u4-6:",
            text.replace("at most 1,024 B a message", "at most 1,025 B a message"),
        ),
    ];
    for (expect, broken) in &cases {
        assert_ne!(
            broken, &text,
            "the broken copy for {expect} changed nothing"
        );
        let problems = audit_u4(broken);
        assert!(
            problems.iter().any(|p| p.starts_with(expect)),
            "a copy broken for {expect} passed: {problems:?}"
        );
    }
}

/// A type's bytes after their framing (its type and body), framed for `channel` (catalogue §1).
fn reframed(bytes: &[u8], from: Channel, to: Channel) -> Vec<u8> {
    let frame = |c: Channel| match c {
        Channel::Datagram => 0,
        Channel::Control => 2,
        Channel::FullState => 4,
    };
    let content = &bytes[frame(from)..];
    let mut out = match frame(to) {
        2 => (content.len() as u16).to_le_bytes().to_vec(),
        4 => (content.len() as u32).to_le_bytes().to_vec(),
        _ => Vec::new(),
    };
    out.extend_from_slice(content);
    out
}

/// Item 2, the code's half: each type decodes at its receiver on its own channel and is refused there on every other
/// one (`WrongChannel`), and refused at the other end (`WrongDirection`) unless it travels both ways. Item 3's size
/// half: one byte over the limit the doc gives is the size error, whatever the buffer holds (a3 in tests/wire.rs runs
/// it per type on 1,000 cases).
#[test]
fn u4_items_2_and_3_each_type_on_its_channel_its_direction_and_its_limit() {
    let text = doc();
    let empty = [
        Message::Input(Input::default()),
        Message::Snapshot(Snapshot::default()),
        Message::FullState(FullState::default()),
        Message::Resync(Resync),
        Message::Ping(Ping::default()),
        Message::Pong(Pong::default()),
        Message::Join(Join::default()),
        Message::JoinAccepted(JoinAccepted::default()),
        Message::JoinRejected(JoinRejected::RosterFull),
        Message::ClientSettings(ClientSettings::default()),
        Message::RoomEnded(RoomEnded {
            reason: EndReason::Crash,
            final_tick: 7,
        }),
        Message::Leave(Leave::Quit),
        Message::Kick(Kick::Overflow),
        Message::SessionControl(SessionControl::TriggerBurst { cap: [1; 16] }),
        Message::SessionStatus(SessionStatus::default()),
    ];
    for m in &empty {
        let (ty, channel, direction, limit) = Limits::TYPES
            .iter()
            .copied()
            .find(|t| t.0 == m.name())
            .expect("a row");
        let mut bytes = Vec::new();
        encode(m, &mut bytes).expect("encodes");
        let receivers: &[Direction] = match direction {
            Direction::BothWays => &[Direction::ClientToRoom, Direction::RoomToClient],
            d => &[d],
        };
        for &at in receivers {
            assert_eq!(decode(at, channel, &bytes).as_ref(), Ok(m), "{ty}");
            for other in [Channel::Datagram, Channel::Control, Channel::FullState] {
                if other != channel {
                    let b = reframed(&bytes, channel, other);
                    assert_eq!(
                        decode(at, other, &b),
                        Err(DecodeError::WrongChannel),
                        "{ty} on {other:?}"
                    );
                }
            }
        }
        if direction != Direction::BothWays {
            let wrong = if direction == Direction::ClientToRoom {
                Direction::RoomToClient
            } else {
                Direction::ClientToRoom
            };
            assert_eq!(
                decode(wrong, channel, &bytes),
                Err(DecodeError::WrongDirection),
                "{ty}"
            );
        }
        // The size half: the doc's limit (for Snapshot, the receive limit its cell names), counted as the doc counts.
        let wire = text
            .lines()
            .find(|l| l.starts_with(&format!("| `{ty}` |")))
            .map(cells)
            .expect("its row");
        let doc_limit = if ty == "Snapshot" {
            Limits::SNAPSHOT_RECEIVE
        } else {
            leading_bytes(&wire[5]).expect("bytes")
        };
        assert_eq!(leading_bytes(&wire[5]), Some(limit));
        let over = doc_limit + 1 + if channel == Channel::FullState { 4 } else { 0 };
        let mut b = bytes.clone();
        b.resize(over, 0xa5);
        assert_eq!(
            decode(direction_or_room(direction), channel, &b),
            Err(DecodeError::TooLarge),
            "{ty}"
        );
    }
}

fn direction_or_room(d: Direction) -> Direction {
    if d == Direction::BothWays {
        Direction::ClientToRoom
    } else {
        d
    }
}

/// Item 6: the room-wide controls decode with the 1,024 B limit and never panic: every control at its largest decodes,
/// one byte over the limit is the size error, and 10,000 fixed-seed random buffers up to the limit give an error or
/// the control that encodes to them (tests/wire.rs's a1 and a2 run 10,000 and 100,000 more).
#[test]
fn u4_item_6_room_controls_decode_within_1_024_b_and_never_panic() {
    let sixteen: Vec<IpAddr> = (0..16u8)
        .map(|i| {
            if i % 2 == 0 {
                IpAddr::V4(Ipv4Addr::new(10, 0, 0, i))
            } else {
                "2001:db8:0:1::".parse().expect("an address")
            }
        })
        .collect();
    let largest = RoomControl::RaiseAddrCap {
        cap: 48,
        addrs: sixteen,
    };
    let mut b = Vec::new();
    largest.encode(&mut b).expect("encodes");
    assert!(b.len() <= Limits::ROOM_CONTROL, "{}", b.len());
    assert_eq!(RoomControl::decode(&b), Ok(largest));
    // The enum and its names, both ways round: one control of each kind names NAMES in order (an exhaustive match
    // gives each its code), as item 1 holds SessionControl's variants to the doc's rows.
    let peers = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let one_of_each = [
        RoomControl::MintToken {
            slot: 1,
            kind: npt_proto::TokenKind::Runner,
        },
        RoomControl::MintCapability { slot: 2 },
        RoomControl::SetRewindCap { ticks: 9 },
        RoomControl::RaiseAddrCap {
            cap: 3,
            addrs: peers.clone(),
        },
        RoomControl::InjectPanic {
            target: None,
            at: 4,
        },
        RoomControl::InjectStall { ms: 200, at: 5 },
        RoomControl::SetPeerProfile {
            peers: None,
            profile: npt_proto::ProfileId::P3,
        },
        RoomControl::SetPeerThrottle {
            peers: peers.clone(),
            kbps: 128,
            secs: 10,
        },
        RoomControl::SetPeerBottleneck {
            peers: peers.clone(),
            percent: 50,
            secs: 600,
        },
        RoomControl::PeerBurst {
            peers: peers.clone(),
            ms: 1_000,
        },
        RoomControl::BlockPeerUdp {
            peers: None,
            on: true,
        },
    ];
    assert_eq!(one_of_each.map(|c| c.name()), RoomControl::NAMES);
    let variants = [
        SessionControl::SetProfile {
            cap: [0; 16],
            profile: npt_proto::ProfileId::Default,
        },
        SessionControl::TriggerBurst { cap: [0; 16] },
        SessionControl::BlockUdp {
            cap: [0; 16],
            on: false,
        },
        SessionControl::RecordStep {
            cap: [0; 16],
            step: 14,
            result: npt_proto::StepResult::Observation,
            note: String::new(),
        },
    ];
    assert_eq!(variants.map(|v| v.variant()), SessionControl::VARIANTS);
    let mut over = vec![0u8; Limits::ROOM_CONTROL + 1];
    over[..2].copy_from_slice(&((Limits::ROOM_CONTROL - 1) as u16).to_le_bytes());
    over[2] = 4;
    assert_eq!(RoomControl::decode(&over), Err(DecodeError::TooLarge));
    let mut rng = ChaCha8Rng::seed_from_u64(0xc6);
    for _ in 0..10_000 {
        let len = rng.next_u32() as usize % (Limits::ROOM_CONTROL + 1);
        let mut buf = vec![0u8; len];
        rng.fill_bytes(&mut buf);
        if len >= 3 {
            buf[..2].copy_from_slice(&((len - 2) as u16).to_le_bytes());
            buf[2] = 1 + (rng.next_u32() % 11) as u8;
        }
        let r = std::panic::catch_unwind(|| RoomControl::decode(&buf)).expect("never a panic");
        if let Ok(c) = r {
            let mut again = Vec::new();
            c.encode(&mut again).expect("encodes");
            assert_eq!(again, buf);
        }
    }
}

#[test]
fn sizes_read_as_whole_bytes() {
    for ok in ["166 B (proposed)", "1,000 B of payload", "65,536 B", "8 B"] {
        assert!(whole_bytes(ok), "{ok}");
    }
    for bad in [
        "about 166 B",
        "166B",
        "1000,0 B",
        "0 B",
        "1,00 B",
        "n/a (not built)",
        "166 bytes",
    ] {
        assert!(!whole_bytes(bad), "{bad}");
    }
}
