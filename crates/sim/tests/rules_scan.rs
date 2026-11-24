//! The interim scan of the simulation crate's rules (ADR 0004 parts 4, 7 and 9; PM-0004-23). It stands in for the
//! lints of the quality gate step under the advisor agent's three conditions (the build plan,
//! `docs/reports/2026-11-17-build-plan-slice-1.md`, "The sim's rules, until the lints exist"): it checks exactly
//! part 7's allow-list, it runs in the plain `cargo test`, and it is interim until those lints merge.
//!
//! It reads `crates/sim/src` as text and fails on: a float method outside part 7's list, a std hash map or hash set
//! without part 9's escape, and a clock. A scan of text sees no types, so a banned name counts wherever it is called
//! as a method (`.min(`) or through `f32::` or `f64::`; integer code writes the path form on its own type
//! (`u32::min(a, b)`, `Ord::min(a, b)`), which the scan lets through. Not caught, as part 7 says: the float `%`.

use std::fs;
use std::path::{Path, PathBuf};

/// Part 7's float methods, word for word; its operators, comparisons and `as` casts are syntax. The list of the
/// "exact non-arithmetic methods" is read as closed at the four the draft names (Mustafa's DEPART-3 (a)).
#[rustfmt::skip]
const ALLOWED: [&str; 14] = [
    "total_cmp", "sqrt", "floor", "ceil", "round", "trunc", "fract", "mul_add", "abs", "copysign", "is_finite", "is_nan",
    "to_bits", "from_bits",
];

/// Every inherent method of `f32` and of `f64` in std 1.99.1 (the two pages hold the same 79 names): read from
/// doc.rust-lang.org/1.99.1/std/primitive.f32.html and primitive.f64.html on 2026-11-24. Banned is what is here
/// and not in `ALLOWED`. A toolchain bump reads the pages again.
#[rustfmt::skip]
const STD: [&str; 79] = [
    "abs", "abs_sub", "acos", "acosh", "algebraic_add", "algebraic_div", "algebraic_mul", "algebraic_rem", "algebraic_sub",
    "asin", "asinh", "atan", "atan2", "atanh", "cbrt", "ceil", "clamp", "clamp_magnitude", "classify", "copysign", "cos",
    "cosh", "div_euclid", "erf", "erfc", "exp", "exp2", "exp_m1", "floor", "fract", "from_be_bytes", "from_bits",
    "from_le_bytes", "from_ne_bytes", "gamma", "hypot", "is_finite", "is_infinite", "is_nan", "is_normal",
    "is_sign_negative", "is_sign_positive", "is_subnormal", "ln", "ln_1p", "ln_gamma", "log", "log10", "log2", "max",
    "maximum", "midpoint", "min", "minimum", "mul_add", "next_down", "next_up", "powf", "powi", "recip", "rem_euclid",
    "round", "round_ties_even", "signum", "sin", "sin_cos", "sinh", "sqrt", "tan", "tanh", "to_be_bytes", "to_bits",
    "to_degrees", "to_int_unchecked", "to_le_bytes", "to_ne_bytes", "to_radians", "total_cmp", "trunc",
];

/// Part 9's ban: std's hashed collections and what builds them. Their iteration order is not the same on every run.
#[rustfmt::skip]
const HASHED: [&str; 6] = ["HashMap", "HashSet", "hash_map", "hash_set", "RandomState", "DefaultHasher"];

/// Part 4: the tick is the simulation's only clock.
const CLOCKS: [&str; 3] = ["Instant", "SystemTime", "UNIX_EPOCH"];

/// The words an item starts with: part 9's escape counts on an item, not on a field or a statement.
#[rustfmt::skip]
const ITEMS: [&str; 10] = ["type", "struct", "enum", "static", "const", "fn", "use", "impl", "mod", "trait"];

/// Part 9's escape as the scan reads it, token by token; `"…"` is a string that is not empty: the written reason.
#[rustfmt::skip]
const ESCAPE: [&str; 13] = ["#", "[", "allow", "(", "clippy", "::", "disallowed_types", ",", "reason", "=", "\"…\"", ")", "]"];

/// The banned methods that part 7 and PM-0004-23 name.
#[rustfmt::skip]
const NAMED: [&str; 15] = [
    "powi", "hypot", "cbrt", "to_degrees", "to_radians", "min", "max", "clamp", "rem_euclid", "div_euclid", "sin", "cos",
    "atan2", "powf", "exp",
];

#[derive(Debug)]
struct Token {
    /// An identifier, one punctuation mark (`::` is one), or `""` and `"…"` for an empty and a filled string.
    text: String,
    line: usize,
}

/// Splits Rust source into identifiers and punctuation. Comments (nested ones too), the content of every kind of
/// string, character literals, lifetimes and numbers are left out: a banned name in prose is not a finding.
fn tokens(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let at_or = |index: usize| chars.get(index).copied().unwrap_or('\0');
    let (mut at, mut line, mut out) = (0, 1, Vec::new());
    // Skips a string's content from just after its opening quote to just after its closing one.
    let string = |mut at: usize, hashes: usize, raw: bool, line: &mut usize| {
        let start = at;
        loop {
            match at_or(at) {
                '\0' => return (at, at > start),
                '\\' if !raw => at += 1,
                '"' if (1..=hashes).all(|k| at_or(at + k) == '#') => {
                    return (at + 1 + hashes, at > start);
                }
                '\n' => *line += 1,
                _ => {}
            }
            at += 1;
        }
    };
    while at < chars.len() {
        let (c, next) = (chars[at], at_or(at + 1));
        if c == '\n' {
            line += 1;
            at += 1;
        } else if c == '/' && next == '/' {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
        } else if c == '/' && next == '*' {
            let mut depth = 0;
            while at < chars.len() {
                match (chars[at], at_or(at + 1)) {
                    ('/', '*') => (depth, at) = (depth + 1, at + 1),
                    ('*', '/') => (depth, at) = (depth - 1, at + 1),
                    ('\n', _) => line += 1,
                    _ => {}
                }
                at += 1;
                if depth == 0 {
                    break;
                }
            }
        } else if c == '\'' {
            // A character literal ('a', '\n', '\'') or a lifetime ('a, 'static): the literal has its closing quote.
            if next == '\\' {
                at += 3;
                while at < chars.len() && chars[at] != '\'' {
                    at += 1;
                }
                at += 1;
            } else if at_or(at + 2) == '\'' {
                at += 3;
            } else {
                at += 1;
                while at_or(at).is_alphanumeric() || at_or(at) == '_' {
                    at += 1;
                }
            }
        } else if c.is_alphabetic() || c == '_' {
            let start = at;
            while at_or(at).is_alphanumeric() || at_or(at) == '_' {
                at += 1;
            }
            let word: String = chars[start..at].iter().collect();
            let hashes = (at..).take_while(|&index| at_or(index) == '#').count();
            let raw = matches!(word.as_str(), "r" | "br" | "cr");
            if (raw || (hashes == 0 && matches!(word.as_str(), "b" | "c")))
                && at_or(at + hashes) == '"'
            {
                let (end, filled) = string(at + hashes + 1, hashes, raw, &mut line);
                at = end;
                out.push(Token {
                    text: if filled { "\"…\"" } else { "\"\"" }.to_owned(),
                    line,
                });
            } else if word == "b" && at_or(at) == '\'' {
                // A byte literal: the next round reads it as a character literal.
            } else if !(word == "r" && hashes == 1) {
                // (`r#name` is a raw identifier: the next round reads the name.)
                out.push(Token { text: word, line });
            } else {
                at += 1;
            }
        } else if c.is_ascii_digit() {
            while at_or(at).is_alphanumeric()
                || at_or(at) == '_'
                || (at_or(at) == '.' && at_or(at + 1).is_ascii_digit())
            {
                at += 1;
            }
        } else if c == '"' {
            let (end, filled) = string(at + 1, 0, false, &mut line);
            at = end;
            out.push(Token {
                text: if filled { "\"…\"" } else { "\"\"" }.to_owned(),
                line,
            });
        } else {
            let pair = c == ':' && next == ':';
            if !c.is_whitespace() {
                out.push(Token {
                    text: if pair { "::".to_owned() } else { c.to_string() },
                    line,
                });
            }
            at += if pair { 2 } else { 1 };
        }
    }
    out
}

/// Which tokens part 9's one escape covers: `#[allow(clippy::disallowed_types, reason = "…")]` with a reason that is
/// written, standing on an item, up to that item's end (its `;`, or the brace that closes its body).
fn escaped(t: &[Token]) -> Vec<bool> {
    let text = |index: usize| t.get(index).map_or("", |token| token.text.as_str());
    let mut covered = vec![false; t.len()];
    for start in 0..t.len() {
        if !(0..ESCAPE.len()).all(|k| text(start + k) == ESCAPE[k]) {
            continue;
        }
        let mut at = start + ESCAPE.len();
        if text(at) == "pub" {
            at += if text(at + 1) == "(" { 4 } else { 1 };
        }
        if !ITEMS.contains(&text(at)) {
            continue;
        }
        let mut depth = 0;
        while at < t.len() {
            covered[at] = true;
            match text(at) {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" => depth -= 1,
                "}" => depth -= 1,
                _ => {}
            }
            if depth < 0 || (depth == 0 && matches!(text(at), ";" | "}")) {
                break;
            }
            at += 1;
        }
    }
    covered
}

/// Every breach of the rules in one file's source, each as "file:line: what".
fn breaches(file: &str, source: &str) -> Vec<String> {
    let t = tokens(source);
    let text = |index: Option<usize>| {
        index
            .and_then(|index| t.get(index))
            .map_or("", |token| token.text.as_str())
    };
    let covered = escaped(&t);
    let mut found = Vec::new();
    for (i, token) in t.iter().enumerate() {
        let name = token.text.as_str();
        let (before, before_2, before_3) = (
            text(i.checked_sub(1)),
            text(i.checked_sub(2)),
            text(i.checked_sub(3)),
        );
        let on_float = before == "::"
            && (matches!(before_2, "f32" | "f64")
                || (before_2 == ">" && matches!(before_3, "f32" | "f64")));
        if STD.contains(&name)
            && !ALLOWED.contains(&name)
            && ((before == "." && text(Some(i + 1)) == "(") || on_float)
        {
            found.push(format!(
                "{file}:{}: `{name}` is a float method outside ADR 0004 part 7's list (on an integer, write the path form on its own type, as in `u32::{name}(..)`)",
                token.line
            ));
        }
        if HASHED.contains(&name) && !covered[i] {
            found.push(format!(
                "{file}:{}: `{name}`: no std hash map or hash set in the simulation (ADR 0004 part 9); a `BTreeMap` or an index-ordered `Vec` instead, or the escape with its reason on the item",
                token.line
            ));
        }
        if CLOCKS.contains(&name)
            || (name == "time" && before == "::" && matches!(before_2, "std" | "core"))
        {
            found.push(format!(
                "{file}:{}: `{name}`: the tick is the simulation's only clock (ADR 0004 part 4)",
                token.line
            ));
        }
    }
    found
}

fn rust_files(folder: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(folder)
        .expect("a readable folder")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_simulation_crate_keeps_its_rules() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    let names: Vec<String> = files
        .iter()
        .map(|file| {
            file.strip_prefix(&src)
                .expect("under src")
                .display()
                .to_string()
        })
        .collect();
    // A scan of the wrong folder would find nothing and pass.
    assert!(
        names.iter().any(|name| name == "lib.rs") && names.iter().any(|name| name == "world.rs"),
        "not the crate's sources: {names:?}"
    );
    let found: Vec<String> = files
        .iter()
        .zip(&names)
        .flat_map(|(file, name)| {
            breaches(name, &fs::read_to_string(file).expect("a readable file"))
        })
        .collect();
    assert!(
        found.is_empty(),
        "the simulation crate breaks its rules:\n{}",
        found.join("\n")
    );
}

#[test]
fn the_lists_are_the_draft_s() {
    assert!(
        ALLOWED.iter().all(|name| STD.contains(name)),
        "an allowed name is not a std float method"
    );
    assert!(STD.is_sorted() && STD.windows(2).all(|pair| pair[0] != pair[1]));
    let banned = STD.iter().filter(|name| !ALLOWED.contains(name)).count();
    assert_eq!((STD.len(), ALLOWED.len(), banned), (79, 14, 65));
    for name in NAMED {
        assert!(STD.contains(&name) && !ALLOWED.contains(&name), "{name}");
    }
    let algebraic = STD.iter().filter(|name| name.starts_with("algebraic_"));
    assert_eq!(algebraic.count(), 5);
}

/// Each planted breach, with the name its one finding must hold.
#[rustfmt::skip]
const BITES: [(&str, &str); 22] = [
    ("fn f(x: f32) -> f32 { x.sin() }", "`sin`"),
    ("fn f(deg: f32) -> f32 { deg.to_radians() }", "`to_radians`"),
    ("fn f(a: f32, b: f32) -> f32 { a.min(b) }", "`min`"),
    ("fn f(x: f32) -> f32 { f32::powi(x, 2) }", "`powi`"),
    ("fn f(x: f64) -> f64 { <f64>::hypot(x, x) }", "`hypot`"),
    ("fn f(v: &[f32]) -> Vec<f32> { v.iter().copied().map(f32::cbrt).collect() }", "`cbrt`"),
    ("fn f(x: f32) -> f32 { x\n    .algebraic_add(1.0) }", "2: `algebraic_add`"),
    ("fn f(x: f32) -> [u8; 4] { x.to_le_bytes() }", "`to_le_bytes`"),
    ("struct S { seen: std::collections::HashMap<u32, u32> }", "`HashMap`"),
    ("use std::collections::hash_map::Entry;", "`hash_map`"),
    ("fn f() { let _ = std::hash::RandomState::new(); }", "`RandomState`"),
    ("fn f() { let _ = Instant::now(); }", "`Instant`"),
    ("fn f(t: SystemTime) {}", "`SystemTime`"),
    ("use std::time::Duration;", "`time`"),
    ("fn f(d: core::time::Duration) {}", "`time`"),
    // The escape: not on a field, not without a written reason, and no further than its own item.
    ("struct S { #[allow(clippy::disallowed_types, reason = \"lookup only\")] seen: HashSet<u32> }", "`HashSet`"),
    ("#[allow(clippy::disallowed_types, reason = \"\")] type Seen = HashSet<u32>;", "`HashSet`"),
    ("#[allow(clippy::disallowed_types)] type Seen = HashSet<u32>;", "`HashSet`"),
    ("#[allow(clippy::disallowed_types, reason = \"lookup only\")] type A = BTreeSet<u32>;\ntype B = HashSet<u32>;", "2: `HashSet`"),
    // The scan comes back out of a literal, a lifetime and a comment: what follows them is still read.
    ("fn f() -> (char, char, u8) { ('\"', '\\'', b'\"') } fn g(x: f32) -> f32 { x.cos() }", "`cos`"),
    ("fn f<'a>(x: &'a f32) -> &'static str { r#\"a \"quote\" \"# } fn g(x: f32) -> f32 { x.tan() }", "`tan`"),
    ("/* a /* nested */ comment */ fn g(x: f32) -> f32 { x.exp() } // x.ln()", "`exp`"),
];

/// What must pass: the allowed methods, the path forms on integers, the escape on an item, and names in prose.
#[rustfmt::skip]
const CONTROLS: [&str; 8] = [
    "fn f(x: f32) -> f32 { x.abs().fract().sqrt().mul_add(2.0, 1.5e3).copysign(-1.0).total_cmp(&x) as i32 as f32 }",
    "fn f(a: u32, b: u32) -> u32 { core::cmp::min(a, b) + u32::max(a, b) + Ord::clamp(a, 0, 9) + min(a, b) }",
    "fn f(tick: u32) -> [u8; 4] { u32::to_le_bytes(tick) }",
    "#[allow(clippy::disallowed_types, reason = \"lookup only, outside the serialized state\")]\npub(crate) struct Lookup { by_name: std::collections::HashMap<u32, [u8; 4]> }",
    "#[allow(clippy::disallowed_types, reason = \"lookup only\")] use std::collections::{HashMap, HashSet};",
    "// a HashMap, x.sin() and Instant::now()\n/* std::time /* x.cos() */ x.tan() */ fn f() {}",
    "fn f() -> (&'static str, &'static [u8]) { (\"a HashMap, x.sin(), \\\" std::time\", br#\"x.cos() \"Instant\" \"#) }",
    "fn f(r#type: u32, time: u32) -> u32 { r#type + time }",
];

#[test]
fn each_planted_breach_is_found_once_and_by_name() {
    for (source, must) in BITES {
        let found = breaches("planted.rs", source);
        assert!(
            found.len() == 1 && found[0].contains(must),
            "{source}\nfound: {found:#?}\nwanted one finding with {must}"
        );
    }
}

#[test]
fn what_the_rules_allow_passes() {
    for source in CONTROLS {
        assert_eq!(
            breaches("control.rs", source),
            Vec::<String>::new(),
            "{source}"
        );
    }
}
