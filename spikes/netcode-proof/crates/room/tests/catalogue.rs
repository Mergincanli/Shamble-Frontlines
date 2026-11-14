//! e1, catalogue item 4 (docs/design/netcode-proof-messages.md §6; u4-build-plan.md :717): the encoder's layout table
//! (npt-proto's `Snapshot::GROUPS` and `FullState::GROUPS`, the one table the room's encoder codes with) names exactly
//! the "Field groups" rows for `Snapshot` and `FullState`, each with the same class; the `authority-only` groups have no
//! entry. The doc is read as proto/tests/catalogue.rs reads it: its path from `CARGO_MANIFEST_DIR`, tables by header.

use npt_proto::{Class, FullState, Snapshot};

const DOC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../docs/design/netcode-proof-messages.md"
);
const FIELD_GROUPS: &str = "| Message | Field group | Class | Sent to | Notes |";

fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
    inner.split('|').map(|c| c.trim().to_owned()).collect()
}

/// The rows of the table under `header`, the separator skipped.
fn rows(text: &str, header: &str) -> Vec<Vec<String>> {
    let mut lines = text.lines().skip_while(|l| l.trim_end() != header);
    assert!(lines.next().is_some(), "no table headed {header}");
    lines
        .take_while(|l| l.starts_with('|'))
        .filter(|l| !l.starts_with("|---"))
        .map(cells)
        .collect()
}

fn class(c: Class) -> &'static str {
    match c {
        Class::Owner => "owner",
        Class::Team => "team",
        Class::Spatial => "spatial",
        Class::Public => "public",
    }
}

#[test]
fn e1_the_layout_tables_equal_the_field_group_rows_and_hold_no_authority_only_group() {
    let text = std::fs::read_to_string(DOC).unwrap_or_else(|e| panic!("{DOC}: {e}"));
    let table = rows(&text, FIELD_GROUPS);
    let tables: [(&str, Vec<(&str, &str)>); 2] = [
        (
            "`Snapshot`",
            Snapshot::GROUPS.iter().map(|g| (g.0, class(g.1))).collect(),
        ),
        (
            "`FullState`",
            FullState::GROUPS
                .iter()
                .map(|g| (g.0, class(g.1)))
                .collect(),
        ),
    ];
    let mut authority_only = 0;
    for (message, code) in tables {
        let doc: Vec<(String, String)> = table
            .iter()
            .filter(|r| r[0] == message)
            .map(|r| (r[1].clone(), r[2].clone()))
            .collect();
        assert!(!doc.is_empty(), "no rows for {message}");
        authority_only += doc.iter().filter(|r| r.1 == "authority-only").count();
        let doc: Vec<(&str, &str)> = doc
            .iter()
            .filter(|r| r.1 != "authority-only")
            .map(|r| (r.0.as_str(), r.1.as_str()))
            .collect();
        assert_eq!(
            code, doc,
            "{message}: the layout table against the catalogue's rows"
        );
        assert!(code.iter().all(|g| g.1 != "authority-only"));
    }
    assert_eq!(
        authority_only, 6,
        "three authority-only groups under each message, none in the tables"
    );
}
