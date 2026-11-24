//! The interim scan of the simulation crate's rules (ADR 0004 parts 4, 7 and 9; PM-0004-23). It stands in for the
//! lints of the quality gate step under the advisor agent's three conditions (the build plan,
//! `docs/reports/2026-11-17-build-plan-slice-1.md`, "The sim's rules, until the lints exist"): it checks exactly
//! part 7's allow-list, it runs in the plain `cargo test`, and it is interim until those lints merge.
//!
//! It reads `crates/sim/src` as text and fails on: a float method outside part 7's list, a hash map or hash set
//! without part 9's escape, a clock, per-target code (`target_feature`; `core::arch` however it is brought in; the
//! relaxed SIMD calls by their names: PM-0004-23 names relaxed SIMD), and source brought in from outside `src/`
//! (`#[path]`, also inside `cfg_attr`; `include!`), which it would not read. A branch on the target itself
//! (`cfg(target_arch = "..")`) is not this scan's to judge: that is for review and the later lint.
//!
//! A scan of text sees no types. So a banned name counts wherever it is called as a method (`.min(`) and after any
//! path (`f32::min`, `Self::min`, a type alias, a generic, `<f32 as T>::min`), unless that path is an integer type
//! or one of `Ord`, `PartialOrd`, `Iterator` and `cmp`: integer code writes the path form on its own type
//! (`u32::min(a, b)`, `Ord::min(a, b)`), and the simulation's own helpers are called by their bare names or take
//! other names. Not caught: a macro that is handed a method's name and builds the call itself, and, as part 7
//! says, the float `%`.

use std::fs;
use std::path::{Path, PathBuf};

/// Part 7's float methods, word for word; its operators, comparisons and `as` casts are syntax. The list of the
/// "exact non-arithmetic methods" is read as closed at the four the draft names (Mustafa's DEPART-3 (a)).
#[rustfmt::skip]
const ALLOWED: [&str; 14] = [
    "total_cmp", "sqrt", "floor", "ceil", "round", "trunc", "fract", "mul_add", "abs", "copysign", "is_finite", "is_nan",
    "to_bits", "from_bits",
];

/// On part 7's list and refused here all the same: Mustafa's answer MUL-ADD (a) of 2026-11-24, 15:58:25 UTC, "The
/// fused multiply-add stays banned in the simulation; it writes a * b + c." `mul_add` is not the same on every
/// target on the pinned toolchain: exact natively on a processor with FMA, and not in WebAssembly, where the
/// toolchain's software `fmaf` misses results below the smallest normal `f32` (the `pin` lines of
/// `crates/devtools/vectors/f32-expected.txt` hold the measured cases). Part 7's list above stays word for word: the
/// draft gets a dated pointer at its next revision, not a changed list here.
const HELD: [(&str, &str); 1] = [(
    "mul_add",
    "MUL-ADD: its bits differ between the native build and WebAssembly on the pinned toolchain",
)];

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

/// Part 9's ban: hashed collections and what builds them (their iteration order is not the same on every run). A
/// name that holds `HashMap` or `HashSet` counts too: many hashed maps of other crates are named after std's. Not
/// all are (`DashMap`, hashbrown's `HashTable`): the lint of the quality gate step names types, this scan reads names.
#[rustfmt::skip]
const HASHED: [&str; 6] = ["HashMap", "HashSet", "hash_map", "hash_set", "RandomState", "DefaultHasher"];

/// The paths before a banned name that are not a float's: the integers, and what orders things.
#[rustfmt::skip]
const NOT_FLOATS: [&str; 16] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "Ord", "PartialOrd", "Iterator", "cmp",
];

/// Part 4: the tick is the simulation's only clock.
const CLOCKS: [&str; 3] = ["Instant", "SystemTime", "UNIX_EPOCH"];

/// The words an item that names a type or a value starts with: part 9's escape counts on such an item, not on a
/// field or a statement, and not on a whole `fn`, `impl`, `mod` or `trait` with everything in it.
#[rustfmt::skip]
const ITEMS: [&str; 6] = ["type", "struct", "enum", "static", "const", "use"];

/// What follows `const` where it starts a function, not a constant.
const ITEMS_NOT: [&str; 4] = ["fn", "unsafe", "async", "extern"];

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
    /// An identifier, one punctuation mark (`::` is one, and so is `..`), or `""` and `"…"` for an empty and a filled
    /// string.
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
                '\\' if !raw => {
                    at += 1;
                    if at_or(at) == '\n' {
                        *line += 1;
                    }
                }
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
            // Two marks that stand side by side: a path's `::`, a range's `..` (a number between two dots, as in
            // `pair.0.sin()`, is left out above and leaves two single dots).
            let pair = (c == ':' || c == '.') && next == c;
            if !c.is_whitespace() {
                out.push(Token {
                    text: if pair {
                        format!("{c}{c}")
                    } else {
                        c.to_string()
                    },
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
        // `const fn` is a function, and the escape does not cover a function's body.
        if !ITEMS.contains(&text(at)) || (text(at) == "const" && ITEMS_NOT.contains(&text(at + 1)))
        {
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
        let (before, before_2, next) = (
            text(i.checked_sub(1)),
            text(i.checked_sub(2)),
            text(Some(i + 1)),
        );
        // A method call (a range's `..` is its own token, so `0..min(a, b)` is none), or any path that is not an
        // integer's.
        let called = before == "." && next == "(";
        let on_a_path = before == "::" && !NOT_FLOATS.contains(&before_2);
        let float_call = STD.contains(&name) && (called || on_a_path);
        if let Some((_, why)) = HELD.iter().find(|(held, _)| float_call && *held == name) {
            found.push(format!(
                "{file}:{}: `{name}` is on ADR 0004 part 7's list and held back all the same ({why}); write `a * b + c`",
                token.line
            ));
        } else if float_call && !ALLOWED.contains(&name) {
            found.push(format!(
                "{file}:{}: `{name}` is a float method outside ADR 0004 part 7's list (on an integer, write the path form on its own type, as in `u32::{name}(..)`)",
                token.line
            ));
        }
        let hashed = HASHED.contains(&name) || name.contains("HashMap") || name.contains("HashSet");
        if hashed && !covered[i] {
            found.push(format!(
                "{file}:{}: `{name}`: no hash map or hash set in the simulation (ADR 0004 part 9); a `BTreeMap` or an index-ordered `Vec` instead, or the escape with its reason on the type's own item",
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
        // `arch` as a step of a path (`core::arch`, `k::arch`, `arch::wasm32`) or as an entry of a `use` list.
        let arch = name == "arch"
            && (before == "::"
                || next == "::"
                || (matches!(before, "{" | ",") && matches!(next, "as" | "," | "}")));
        if name == "target_feature" || arch || name.contains("_relaxed_") {
            found.push(format!(
                "{file}:{}: `{name}`: no per-target code in the simulation, SIMD least of all (ADR 0004 part 7: one result on every target)",
                token.line
            ));
        }
        // `#[path = ".."]`, also as `#[cfg_attr(.., path = "..")]`: the name, `=`, then a text, inside brackets.
        let path_attribute = name == "path"
            && next == "="
            && matches!(before, "[" | "(" | ",")
            && text(Some(i + 2)).starts_with('"');
        if path_attribute || (name == "include" && next == "!") {
            found.push(format!(
                "{file}:{}: `{name}`: the simulation's source lies under `src/`, where this scan reads it; a file brought in from elsewhere would not be read",
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

#[test]
fn what_is_held_back_is_one_name_of_the_draft_s_list() {
    assert!(
        HELD.iter()
            .all(|(name, why)| ALLOWED.contains(name) && why.starts_with("MUL-ADD"))
    );
    assert_eq!(HELD.map(|(name, _)| name), ["mul_add"]);
}

/// Each planted breach, with the name its one finding must hold.
#[rustfmt::skip]
const BITES: [(&str, &str); 43] = [
    ("fn f(x: f32) -> f32 { x.sin() }", "`sin`"),
    ("fn f(deg: f32) -> f32 { deg.to_radians() }", "`to_radians`"),
    ("fn f(a: f32, b: f32) -> f32 { a.min(b) }", "`min`"),
    ("fn f(x: f32) -> f32 { f32::powi(x, 2) }", "`powi`"),
    ("fn f(x: f64) -> f64 { <f64>::hypot(x, x) }", "`hypot`"),
    ("fn f(v: &[f32]) -> Vec<f32> { v.iter().copied().map(f32::cbrt).collect() }", "`cbrt`"),
    // Held back (MUL-ADD), as a method and by its path.
    ("fn f(x: f32, y: f32, z: f32) -> f32 { x.mul_add(y, z) }", "`mul_add` is on ADR 0004 part 7's list and held back"),
    ("fn f(a: f32, b: f32, c: f32) -> f32 { f32::mul_add(a, b, c) }", "`mul_add` is on ADR 0004 part 7's list and held back"),
    // A float under another name: an alias, `Self`, a generic, a trait path.
    ("type Real = f32; fn f(x: Real) -> Real { Real::sin(x) }", "`sin`"),
    ("impl Trig for f32 { fn g(self) -> f32 { Self::tan(self) } }", "`tan`"),
    ("fn f<F: Float>(x: F) -> F { F::cbrt(x) }", "`cbrt`"),
    ("fn f(x: f32) -> f32 { <f32 as Float>::powi(x, 2) }", "`powi`"),
    ("fn f(x: f32) -> f32 { x\n    .algebraic_add(1.0) }", "2: `algebraic_add`"),
    ("fn f(x: f32) -> [u8; 4] { x.to_le_bytes() }", "`to_le_bytes`"),
    ("struct S { seen: std::collections::HashMap<u32, u32> }", "`HashMap`"),
    ("use std::collections::hash_map::Entry;", "`hash_map`"),
    ("fn f() { let _ = std::hash::RandomState::new(); }", "`RandomState`"),
    ("struct S { seen: rustc_hash::FxHashMap<u32, u32> }", "`FxHashMap`"),
    ("fn f() { let _ = Instant::now(); }", "`Instant`"),
    ("fn f(t: SystemTime) {}", "`SystemTime`"),
    ("use std::time::Duration;", "`time`"),
    ("fn f(d: core::time::Duration) {}", "`time`"),
    // Per-target code, and source from outside `src/`.
    ("#[target_feature(enable = \"relaxed-simd\")] fn f() {}", "`target_feature`"),
    ("use core::arch::wasm32::f32x4_add;", "`arch`"),
    ("#[cfg(target_arch = \"wasm32\")] use core::{arch::wasm32::v128, fmt};", "`arch`"),
    ("use core as k; fn f(x: k::arch::wasm32::v128) {}", "`arch`"),
    ("use core::{arch as simd, fmt};", "`arch`"),
    ("fn f(a: v128, b: v128, c: v128) -> v128 { f32x4_relaxed_madd(a, b, c) }", "`f32x4_relaxed_madd`"),
    ("#[cfg_attr(feature = \"x\", path = \"../elsewhere.rs\")] mod elsewhere;", "`path`"),
    // A number between two dots is no range: a call on a tuple's field is a call.
    ("fn f(p: (f32, f32)) -> f32 { p.0.sin() }", "`sin`"),
    ("fn f(m: (f32,)) -> f32 { m.0.mul_add(2.0, 1.0) }", "`mul_add` is on ADR 0004 part 7's list and held back"),
    ("#[allow(clippy::disallowed_types, reason = \"lookup only\")] const fn f() -> usize { size_of::<HashSet<u32>>() }", "`HashSet`"),
    ("#[path = \"../../elsewhere.rs\"] mod elsewhere;", "`path`"),
    ("include!(\"generated.rs\");", "`include`"),
    // The escape: not on a field, not without a written reason, and no further than its own item.
    ("struct S { #[allow(clippy::disallowed_types, reason = \"lookup only\")] seen: HashSet<u32> }", "`HashSet`"),
    ("#[allow(clippy::disallowed_types, reason = \"\")] type Seen = HashSet<u32>;", "`HashSet`"),
    ("#[allow(clippy::disallowed_types)] type Seen = HashSet<u32>;", "`HashSet`"),
    ("#[allow(clippy::disallowed_types, reason = \"lookup only\")] type A = BTreeSet<u32>;\ntype B = HashSet<u32>;", "2: `HashSet`"),
    ("#[allow(clippy::disallowed_types, reason = \"lookup only\")] fn f() { let _ = HashSet::<u32>::new(); }", "`HashSet`"),
    // The scan comes back out of a literal, a lifetime and a comment: what follows them is still read.
    ("fn f() -> (char, char, u8) { ('\"', '\\'', b'\"') } fn g(x: f32) -> f32 { x.cos() }", "`cos`"),
    ("fn f<'a>(x: &'a f32) -> &'static str { r#\"a \"quote\" \"# } fn g(x: f32) -> f32 { x.tan() }", "`tan`"),
    ("/* a /* nested */ comment */ fn g(x: f32) -> f32 { x.exp() } // x.ln()", "`exp`"),
    ("fn f() -> &'static str { \"a line break \\\n inside a text\" }\nfn g(x: f32) -> f32 { x.ln() }", "3: `ln`"),
];

/// What must pass: the allowed methods, the path forms on integers, the escape on an item, and names in prose.
#[rustfmt::skip]
const CONTROLS: [&str; 12] = [
    "fn f(x: f32) -> f32 { (x.abs().fract().sqrt() * 2.0 + 1.5e3).copysign(-1.0).total_cmp(&x) as i32 as f32 }",
    "fn f(a: u32, b: u32) -> u32 { core::cmp::min(a, b) + u32::max(a, b) + Ord::clamp(a, 0, 9) + min(a, b) }",
    "fn f(tick: u32) -> [u8; 4] { u32::to_le_bytes(tick) }",
    "fn f(a: u32, b: u32) -> u32 { (0..min(a, b)).sum::<u32>() + Iterator::max([a, b].into_iter()).unwrap_or(0) }",
    "fn f(include: u32, path: u32, arch: u32) -> u32 { include + path + arch }",
    "fn f(p: (u32, u32), path: u32) -> u32 { let path = path + p.0; (path..=u32::max(path, p.1)).count() as u32 }",
    "#[allow(clippy::disallowed_types, reason = \"lookup only\")] const SEEN: Option<HashSet<u32>> = None;",
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
