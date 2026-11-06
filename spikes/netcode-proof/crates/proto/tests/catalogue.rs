//! The class audit test, U1 part (plan §4.1; `docs/design/netcode-proof-messages.md` §6): the message catalogue read
//! as data. Every message type has a visibility class ("one with none fails the audit test", ADR 0001's catalogue),
//! every plan §4.7 type is there exactly once, and nothing authority-only is ever sent. Each assertion has a broken
//! copy of the doc that must fail it (the plan's "bites first", §6.1). At U4 the test also reads npt-proto's types,
//! limits and the encoder's layout table against these rows.

use std::collections::BTreeSet;

const DOC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../docs/design/netcode-proof-messages.md"
);

const WIRE: &str =
    "| Type | Direction | Channel | Class | Authority | Size limit | Spike status | Draft |";
const FIELDS: &str = "| Message | Field group | Class | Sent to | Notes |";
const SESSION: &str = "| Variant | Fields | Who may send | Check | Effect | Draft |";
const ROOM: &str = "| Control | Fields | Who | Check | Effect |";

/// Plan §4.7's npt-proto messages and controls.
const BUILT: [&str; 14] = [
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
