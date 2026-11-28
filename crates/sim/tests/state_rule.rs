//! The state rule, read from the source (rust-reviewer's 11.7 row 27, T-A; engine-architect's block 7, B1): every
//! field of `struct World` is named in both of `World::visit`'s and `World::visit_shared`'s patterns and bound in
//! exactly one of them; the shared one binds all of what it names, with no `_` in it but `seed: _` and no `..`; and
//! `visit_row`'s pattern binds every field of `struct Soldier`, with no `_` and no `..`. So nothing can join the state
//! without being hashed, nor be hashed into the wrong part: a field left out of the shared bytes (a `counts: _`, the
//! newest command's move written `move_x: _`) is red here even where the compiler is content. It reads world.rs and
//! soldiers.rs as text, with the tokenizer the layering test and the content panic scan share.

use std::fs;
use std::path::Path;

// Its tokenizer only: the marking of test-only code is the layering test's and the panic scan's.
#[path = "support/marking.rs"]
#[allow(dead_code)]
mod marking;

use marking::{Token, tokens};

fn read(file: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(file);
    fs::read_to_string(&path).expect("the simulation's source reads")
}

fn is(t: &[Token], at: usize, text: &str) -> bool {
    t.get(at)
        .is_some_and(|token| token.text == text && !token.raw)
}

/// The tokens inside the brackets that open at `open`, up to their match.
fn inside(t: &[Token], open: usize) -> &[Token] {
    let mut depth = 0_usize;
    for (at, token) in t.iter().enumerate().skip(open) {
        match token.text.as_str() {
            "{" | "[" | "(" => depth += 1,
            "}" | "]" | ")" => {
                depth -= 1;
                if depth == 0 {
                    return &t[open + 1..at];
                }
            }
            _ => {}
        }
    }
    &t[open + 1..]
}

/// The tokens at the top level of `tokens` (no bracket open), split at each comma.
fn items(tokens: &[Token]) -> Vec<&[Token]> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0_usize, 0);
    for (at, token) in tokens.iter().enumerate() {
        match token.text.as_str() {
            "{" | "[" | "(" | "<" => depth += 1,
            "}" | "]" | ")" | ">" => depth = depth.saturating_sub(1),
            "," if depth == 0 => {
                out.push(&tokens[start..at]);
                start = at + 1;
            }
            _ => {}
        }
    }
    out.push(&tokens[start..]);
    out.into_iter().filter(|item| !item.is_empty()).collect()
}

/// The field names of `struct NAME { .. }`, in order.
fn fields(t: &[Token], name: &str) -> Vec<String> {
    let Some(at) = (0..t.len()).find(|&at| is(t, at, "struct") && is(t, at + 1, name)) else {
        return Vec::new();
    };
    let Some(open) = (at..t.len()).find(|&at| is(t, at, "{")) else {
        return Vec::new();
    };
    items(inside(t, open))
        .into_iter()
        .filter_map(|item| {
            // Attributes and a visibility come before the name; the name is the word before the first `:`.
            let colon = item.iter().position(|token| token.text == ":")?;
            item.get(colon.checked_sub(1)?)
                .map(|token| token.text.clone())
        })
        .collect()
}

/// The pattern `let HEAD { .. }` in the body of `fn FUNCTION`: its tokens between the braces.
fn pattern<'t>(t: &'t [Token], function: &str, head: &str) -> Option<&'t [Token]> {
    let start = (0..t.len())
        .find(|&at| is(t, at, "fn") && is(t, at + 1, function) && is(t, at + 2, "("))?;
    let at = (start..t.len())
        .find(|&at| is(t, at, "let") && is(t, at + 1, head) && is(t, at + 2, "{"))?;
    Some(inside(t, at + 2))
}

/// A pattern's fields: each one's name, and the tokens of its sub-pattern (a shorthand field binds its own name).
fn bindings(pattern: &[Token]) -> Vec<(String, Vec<String>)> {
    items(pattern)
        .into_iter()
        .map(|item| {
            let name = item[0].text.clone();
            let sub = if item.len() > 2 && item[1].text == ":" {
                &item[2..]
            } else {
                item
            };
            (name, sub.iter().map(|token| token.text.clone()).collect())
        })
        .collect()
}

/// Whether a sub-pattern drops something: a `_` anywhere in it, or a `..`.
fn drops(sub: &[String]) -> bool {
    sub.iter().any(|text| text == "_")
        || sub.windows(2).any(|pair| pair[0] == "." && pair[1] == ".")
}

/// Every breach of the state rule in this world.rs and this soldiers.rs.
fn breaches(world_rs: &str, soldiers_rs: &str) -> Vec<String> {
    let (world, soldiers) = (tokens(world_rs), tokens(soldiers_rs));
    let mut found = Vec::new();
    let state = fields(&world, "World");
    let (Some(whole), Some(shared), Some(row)) = (
        pattern(&world, "visit", "Self"),
        pattern(&world, "visit_shared", "Self"),
        pattern(&world, "visit_row", "Soldier"),
    ) else {
        return vec!["a pattern of visit, visit_shared or visit_row is missing".to_owned()];
    };
    let (whole, shared, row) = (bindings(whole), bindings(shared), bindings(row));
    let bound = |list: &[(String, Vec<String>)], field: &str| {
        list.iter()
            .find(|(name, _)| name == field)
            .map(|(_, sub)| sub != &["_"])
    };
    for field in &state {
        match (bound(&whole, field), bound(&shared, field)) {
            (Some(a), Some(b)) if a != b => {}
            (Some(_), Some(_)) => found.push(format!(
                "World.{field} is bound in both or in neither of visit and visit_shared"
            )),
            _ => found.push(format!(
                "World.{field} is not named in both of visit and visit_shared"
            )),
        }
    }
    for (name, sub) in &shared {
        let seed = name == "seed" && sub == &["_"];
        if !seed && drops(sub) {
            found.push(format!(
                "visit_shared drops part of World.{name}: {}",
                sub.join(" ")
            ));
        }
    }
    for (list, what) in [(&whole, "visit"), (&shared, "visit_shared")] {
        for (name, _) in list.iter().filter(|(name, _)| !state.contains(name)) {
            found.push(format!("{what} names {name}, which is no field of World"));
        }
    }
    let soldier = fields(&soldiers, "Soldier");
    for field in &soldier {
        match row.iter().find(|(name, _)| name == field) {
            Some((_, sub)) if !drops(sub) => {}
            _ => found.push(format!("visit_row does not bind Soldier.{field}")),
        }
    }
    for (name, sub) in row
        .iter()
        .filter(|(name, sub)| !soldier.contains(name) || drops(sub))
    {
        found.push(format!(
            "visit_row's {name} is no bound field of Soldier: {}",
            sub.join(" ")
        ));
    }
    found
}

#[test]
fn every_field_of_the_state_is_hashed_once_in_its_own_part() {
    let (world, soldiers) = (read("world.rs"), read("soldiers.rs"));
    assert_eq!(breaches(&world, &soldiers), Vec::<String>::new());
    // What the reading itself sees: the world's four fields, the row's ten.
    let world_tokens = tokens(&world);
    assert_eq!(
        fields(&world_tokens, "World"),
        ["tick", "player", "soldiers", "seed"]
    );
    assert_eq!(fields(&tokens(&soldiers), "Soldier").len(), 10);
}

/// Each planted copy breaks the rule, and is named.
#[test]
fn each_planted_breach_of_the_state_rule_is_red() {
    let (world, soldiers) = (read("world.rs"), read("soldiers.rs"));
    let planted = |from: &[(&str, &str)]| {
        let mut text = world.clone();
        for (old, new) in from {
            assert!(
                text.contains(old),
                "the plant's anchor {old:?} is in world.rs"
            );
            text = text.replacen(old, new, 1);
        }
        breaches(&text, &soldiers)
    };
    // A count of queries joins the world and is left out of both patterns.
    let counts = planted(&[
        (
            "    seed: [u8; SEED_BYTES],\n}",
            "    seed: [u8; SEED_BYTES],\n    counts: QueryCounts,\n}",
        ),
        (
            "            seed,\n        } = self;",
            "            seed,\n            counts: _,\n        } = self;",
        ),
        (
            "            seed: _,\n        } = self;",
            "            seed: _,\n            counts: _,\n        } = self;",
        ),
    ]);
    assert!(
        counts
            .iter()
            .any(|line| line.contains("World.counts is bound in both or in neither")),
        "{counts:?}"
    );
    assert!(
        counts
            .iter()
            .any(|line| line.contains("visit_shared drops part of World.counts")),
        "{counts:?}"
    );
    // The record as the whole command, its move and aim dropped from the shared bytes.
    let record = planted(&[(
        "player: PlayerRecord { buttons, presses },",
        "player: PlayerCommand { buttons, presses, move_x: _, move_y: _, aim: _ },",
    )]);
    assert!(
        record
            .iter()
            .any(|line| line.contains("visit_shared drops part of World.player")),
        "{record:?}"
    );
    // A field bound in both parts, and the rest of a pattern dropped by `..`.
    let both = planted(&[(
        "            tick: _,\n            player: _,",
        "            tick,\n            player: _,",
    )]);
    assert!(
        both.iter()
            .any(|line| line.contains("World.tick is bound in both")),
        "{both:?}"
    );
    let rest = planted(&[(
        "            seed: _,\n        } = self;",
        "            seed: _,\n            ..\n        } = self;",
    )]);
    assert!(!rest.is_empty(), "{rest:?}");
    // A row's field written `_`, and one left to `..`.
    for (old, new) in [
        ("        held_move,\n", "        held_move: _,\n"),
        ("        pull,\n    } = row;", "        ..\n    } = row;"),
    ] {
        let row = planted(&[(old, new)]);
        assert!(
            row.iter()
                .any(|line| line.contains("visit_row does not bind Soldier.")),
            "{row:?}"
        );
    }
}
