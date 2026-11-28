//! The interim scan of the simulation crate's rules (ADR 0004 parts 4, 7 and 9; PM-0004-23). It stands in for the
//! lints of the quality gate step under the advisor agent's three conditions (the build plan,
//! `docs/reports/2026-11-17-build-plan-slice-1.md`, "The sim's rules, until the lints exist"): it checks exactly
//! part 7's allow-list, it runs in the plain `cargo test`, and it is interim until those lints merge.
//!
//! Since change-set 4b-1 it also reads the content crate's source (`crates/content/src`; the last tests of this
//! file): the same rules without the simulation's two allowances, and every call that can panic outside tests.
//!
//! It reads `crates/sim/src` as text and fails on: a float method outside part 7's list, a hash map or hash set
//! without part 9's escape, a clock, per-target code (`target_feature`; `core::arch` however it is brought in; the
//! relaxed SIMD calls by their names: PM-0004-23 names relaxed SIMD), and source brought in from outside `src/`
//! (`#[path]`, also inside `cfg_attr`; `include!`), which it would not read. A branch on the target itself
//! (`cfg(target_arch = "..")`) is not this scan's to judge: that is for review and the later lint.
//!
//! Since change-set 4b-2 (the collision library's arrival) it also holds: the library's name in one file only,
//! `collision.rs`, and there neither handed on (`pub use`) nor renamed (`as`); and a fused multiply-add by its other
//! names (`fma`, `fmaf` and the like), beside `mul_add` above. One `target_feature` is let through: a `#[cfg(..)]`
//! whose item is `compile_error!`, which can stop a build and can pick no code.
//!
//! A scan of text sees no types. So a banned name counts wherever it is called as a method (`.min(`) and after any
//! path (`f32::min`, `Self::min`, a type alias, a generic, `<f32 as T>::min`), unless that path is an integer type
//! or one of `Ord`, `PartialOrd`, `Iterator` and `cmp`: integer code writes the path form on its own type
//! (`u32::min(a, b)`, `Ord::min(a, b)`), and the simulation's own helpers are called by their bare names or take
//! other names. Not caught: a macro that is handed a method's name and builds the call itself, and, as part 7
//! says, the float `%`.

use std::fs;
use std::path::{Path, PathBuf};

#[path = "support/marking.rs"]
mod marking;

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

/// The collision library's crate, and the one file of `src/` that may name it (ADR 0004 part 10: "parry3d's
/// queries with our own kinematic movement", and "the surface used is small"; change-set 4's plan keeps that
/// surface in one module, so that the rest of the simulation holds our own types only). The path is exact: a
/// `collision.rs` in a folder below is another file.
const LIBRARY: &str = "parry3d";
const LIBRARY_FILE: &str = "collision.rs";

/// The maths routines' crate (ADR 0004 part 7: "transcendental maths only via pinned libm"), the one file of `src/`
/// that may name it, and, in that file, the one routine each named function may call (change-set 4c; rust-reviewer's
/// B7-1, the collision library's rule applied to `libm`). A routine is called by its full path, `libm::<routine>(`:
/// never imported, renamed or handed on. Every other `libm` path is refused, whatever its name, so `expm1`, `expf`,
/// `sincosf` and `atan2f` are refused by construction and not by a list. Each allowance is one routine for one
/// function, run while a match's constants are worked out (the built-file check's reviewed list says why each may be
/// in the players' file: packages/client/scripts/maths-allowed.mjs).
const MATHS: &str = "libm";
const MATHS_FILE: &str = "fmath.rs";
const MATHS_CALLS: [(&str, &str); 2] = [("exp_of", "exp"), ("sincos_of", "sincos")];

/// libm's two routines as the simulation reaches them, by their own names, each with its one caller (rust-reviewer's
/// block 17, Q1). Each name may stand only in `fmath.rs`, and there only in its own `fn` and in the body of its one
/// caller. A name counts wherever it stands as a whole token (a call, a path, `use`, `as`, `pub use`, a function
/// pointer, a macro's argument), test code included, so no file but `fmath.rs` reaches them and no other function
/// there does. Outside `src/` the vectors call them (crates/devtools), and nothing else may: devtools'
/// `tests/layering.rs` holds that for the room, the client endpoint and the players' module.
const RAW: [(&str, &str); 2] = [("exp_of", "share_per_tick"), ("sincos_of", "facing_of")];

/// A fused multiply-add under the names it has outside std's `mul_add` (C's, which the `libm` crate and the
/// toolchain's own routines share): Mustafa's MUL-ADD (a) again. The built-file check refuses the same names in
/// what was built (`packages/client/scripts/build-checks.mjs`, `FUSED_STEPS`); this refuses them where they are
/// written.
const FUSED: [&str; 7] = [
    "fma",
    "fmaf",
    "fmaf16",
    "fmaf128",
    "fmal",
    "fma_round",
    "fma_wide_round",
];

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

/// Which tokens stand in a `#[cfg(..)]` whose item is `compile_error!`: a condition that can only stop a build picks
/// no code, so it may name a target feature (the simulation refuses a native build with `fma` or `avx2` on).
fn build_guards(t: &[Token]) -> Vec<bool> {
    let text = |index: usize| t.get(index).map_or("", |token| token.text.as_str());
    let mut covered = vec![false; t.len()];
    for start in 0..t.len() {
        if [
            text(start),
            text(start + 1),
            text(start + 2),
            text(start + 3),
        ] != ["#", "[", "cfg", "("]
        {
            continue;
        }
        // To the bracket that closes the attribute.
        let (mut end, mut depth) = (start + 1, 0);
        while end < t.len() {
            match text(end) {
                "[" | "(" => depth += 1,
                "]" | ")" => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                break;
            }
            end += 1;
        }
        if end < t.len() && text(end + 1) == "compile_error" && text(end + 2) == "!" {
            covered[start..=end].fill(true);
        }
    }
    covered
}

/// What is wrong with one naming of the collision library, if anything: it stands outside the library's one file,
/// or in a `use` that hands it on (`pub`) or renames it (`as`).
fn library_breach(file: &str, t: &[Token], at: usize) -> Option<&'static str> {
    if file != LIBRARY_FILE {
        return Some(
            "is named outside `collision.rs`: the rest of the simulation holds our own types only",
        );
    }
    let text = |index: usize| t.get(index).map_or("", |token| token.text.as_str());
    // The statement this naming stands in: back to what ends the one before it, on to its own `;`.
    let start = (0..at)
        .rev()
        .find(|&index| matches!(text(index), ";" | "}"))
        .map_or(0, |index| index + 1);
    let end = (at..t.len())
        .find(|&index| text(index) == ";")
        .unwrap_or(t.len());
    let words: Vec<&str> = (start..end).map(text).collect();
    let item = words
        .iter()
        .position(|word| matches!(*word, "use" | "extern"));
    match item {
        Some(item) if words[..item].contains(&"pub") => {
            Some("is handed on with `pub`: other files would name the library through this one")
        }
        Some(_) if words.contains(&"as") => {
            Some("is renamed with `as`: the library's names stay the ones a search finds")
        }
        _ => None,
    }
}

/// Whether a token is a name: a letter or `_` first, then letters, digits and `_`.
fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_')
        && chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// The name of the innermost named function whose body holds token `at`, if any: a function's body is the first
/// `{` after its name outside parentheses and brackets, to the `}` that closes it.
fn enclosing_fn(t: &[Token], at: usize) -> Option<&str> {
    let text = |index: usize| t.get(index).map_or("", |token| token.text.as_str());
    let mut found = None;
    for k in 0..at {
        if text(k) != "fn" || !is_name(text(k + 1)) {
            continue;
        }
        let mut depth = 0_usize;
        let mut open = None;
        for index in k + 2..at {
            match text(index) {
                "(" | "[" => depth += 1,
                ")" | "]" => depth = depth.saturating_sub(1),
                "{" if depth == 0 => {
                    open = Some(index);
                    break;
                }
                ";" if depth == 0 => break,
                _ => {}
            }
        }
        let Some(open) = open else { continue };
        let mut braces = 0_usize;
        let mut close = t.len();
        for (index, token) in t.iter().enumerate().skip(open) {
            match token.text.as_str() {
                "{" => braces += 1,
                "}" => {
                    braces = braces.saturating_sub(1);
                    if braces == 0 {
                        close = index;
                        break;
                    }
                }
                _ => {}
            }
        }
        if at < close {
            found = Some(text(k + 1));
        }
    }
    found
}

/// What is wrong with one naming of the maths routines' crate, if anything: it stands outside its one file; it is
/// no full-path call of a routine (an import, a rename, a path that goes on, a function's address); or the routine is
/// not the one the function it stands in may call. A fused multiply-add by its libm name is the fused rule's to name.
fn maths_breach(file: &str, t: &[Token], at: usize) -> Option<String> {
    let text = |index: usize| t.get(index).map_or("", |token| token.text.as_str());
    let routine = text(at + 2);
    if text(at + 1) == "::" && FUSED.contains(&routine) {
        return None;
    }
    if file != MATHS_FILE {
        return Some(
            "are named outside `fmath.rs`: the simulation calls them from one file".to_owned(),
        );
    }
    if text(at + 1) != "::" || !is_name(routine) || text(at + 3) != "(" {
        return Some("are named but not called by a full path, `libm::<routine>(`: never imported, renamed or handed on".to_owned());
    }
    let function = enclosing_fn(t, at).unwrap_or("no function");
    if MATHS_CALLS.contains(&(function, routine)) {
        return None;
    }
    let allowed: Vec<String> = MATHS_CALLS
        .iter()
        .map(|(function, routine)| format!("`libm::{routine}` in `{function}`"))
        .collect();
    Some(format!(
        "are called as `libm::{routine}` in `{function}`, where the rule allows only {}",
        allowed.join(" and ")
    ))
}

/// Every breach of the rules in one file's source, each as "file:line: what".
fn breaches(file: &str, source: &str) -> Vec<String> {
    breaches_of(file, source, true)
}

/// The same, with or without the simulation's own two allowances: the rule on float methods (part 7's list) and part
/// 9's escape for a hashed map with its reason.
fn breaches_of(file: &str, source: &str, simulation: bool) -> Vec<String> {
    let t = tokens(source);
    let text = |index: Option<usize>| {
        index
            .and_then(|index| t.get(index))
            .map_or("", |token| token.text.as_str())
    };
    let covered = if simulation {
        escaped(&t)
    } else {
        vec![false; t.len()]
    };
    let guards = build_guards(&t);
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
        // A `libm` path is the maths rule's to judge, below: it refuses every one but the allowed calls.
        let on_a_path = before == "::" && !NOT_FLOATS.contains(&before_2) && before_2 != MATHS;
        let float_call = simulation && STD.contains(&name) && (called || on_a_path);
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
        if (name == "target_feature" && !guards[i]) || arch || name.contains("_relaxed_") {
            found.push(format!(
                "{file}:{}: `{name}`: no per-target code in the simulation, SIMD least of all (ADR 0004 part 7: one result on every target)",
                token.line
            ));
        }
        if simulation && FUSED.contains(&name) {
            found.push(format!(
                "{file}:{}: `{name}`: a fused multiply-add, whose bits differ between the native build and WebAssembly (MUL-ADD); write `a * b + c`",
                token.line
            ));
        }
        if let Some(why) = (simulation && name == LIBRARY)
            .then(|| library_breach(file, &t, i))
            .flatten()
        {
            found.push(format!(
                "{file}:{}: `{name}`: the collision library {why} (ADR 0004 part 10)",
                token.line
            ));
        }
        if let Some(why) = (simulation && name == MATHS)
            .then(|| maths_breach(file, &t, i))
            .flatten()
        {
            found.push(format!(
                "{file}:{}: `{name}`: the maths routines {why} (ADR 0004 part 7)",
                token.line
            ));
        }
        if let Some((_, caller)) = RAW.iter().find(|(raw, _)| simulation && *raw == name) {
            let own = before == "fn" || enclosing_fn(&t, i) == Some(*caller);
            if file != MATHS_FILE || !own {
                found.push(format!(
                    "{file}:{}: `{name}`: libm's routine as the simulation reaches it stands only in `fmath.rs`, in its own `fn` and in `{caller}` (ADR 0004 part 7)",
                    token.line
                ));
            }
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

/// The same for the rules of change-set 4b-2: a fused multiply-add by its other names, a target feature outside a
/// build's own refusal, and the collision library's name by the file it stands in; and change-set 4c's maths rule
/// (B7-1's bites): a routine in a function not allowed it, another routine of libm's, an import with a bare call,
/// libm in a second file, a path that goes on, a function's address.
#[rustfmt::skip]
const LATER_BITES: [(&str, &str, &str); 40] = [
    // security-auditor's L1 on C3: another routine of the same family, a path from the crate root, a renamed crate.
    ("fmath.rs", "fn exp_of(x: f64) -> f32 { libm::expf(x as f32) }", "are called as `libm::expf` in `exp_of`"),
    ("fmath.rs", "fn facing_of(x: f64) -> f64 { ::libm::exp(x) }", "are called as `libm::exp` in `facing_of`"),
    ("lib.rs", "extern crate libm as m;", "`libm`: the maths routines are named outside `fmath.rs`"),
    // The raw names (block 17's bites): in the other caller, in other files, imported, renamed, handed on, as a
    // function's address, through a glob import, in fmath.rs's own tests; and libm called back in a caller.
    ("fmath.rs", "fn facing_of(x: f64) -> f64 { exp_of(x) }", "`exp_of`: libm's routine as the simulation reaches it stands only in `fmath.rs`, in its own `fn` and in `share_per_tick`"),
    ("fmath.rs", "fn share_per_tick(x: f64) -> (f64, f64) { sincos_of(x) }", "`sincos_of`: libm's routine as the simulation reaches it"),
    ("world.rs", "fn f() -> f64 { crate::fmath::exp_of(1.0) }", "`exp_of`: libm's routine as the simulation reaches it"),
    ("rules.rs", "use crate::fmath::exp_of as e;", "`exp_of`: libm's routine as the simulation reaches it"),
    ("soldiers.rs", "use crate::fmath::{sincos_of};", "`sincos_of`: libm's routine as the simulation reaches it"),
    ("lib.rs", "pub use fmath::exp_of;", "`exp_of`: libm's routine as the simulation reaches it"),
    ("collision.rs", "fn g() { let f: fn(f64) -> f64 = crate::fmath::exp_of; }", "`exp_of`: libm's routine as the simulation reaches it"),
    ("world.rs", "use crate::fmath::*;\nfn g(x: f64) -> (f64, f64) { sincos_of(x) }", "2: `sincos_of`: libm's routine as the simulation reaches it"),
    ("fmath.rs", "#[cfg(test)]\nmod tests {\n    fn t() -> f64 { super::exp_of(1.0) }\n}", "3: `exp_of`: libm's routine as the simulation reaches it"),
    ("fmath.rs", "pub fn share_per_tick(r: f64, dt: f64) -> f32 { (1.0 - libm::exp(-(r * dt))) as f32 }", "are called as `libm::exp` in `share_per_tick`"),
    ("fmath.rs", "fn share_per_tick(x: f64) -> f64 { libm::expm1(x) }", "`libm`: the maths routines are called as `libm::expm1` in `share_per_tick`"),
    ("fmath.rs", "fn exp_of(x: f64) -> f64 { libm::exp(x) }\nfn other(x: f64) -> (f64, f64) { libm::sincos(x) }", "2: `libm`: the maths routines are called as `libm::sincos` in `other`"),
    ("fmath.rs", "fn facing_of(x: f32) -> (f32, f32) { libm::sincosf(x) }", "are called as `libm::sincosf` in `facing_of`"),
    ("fmath.rs", "fn share_per_tick(x: f64) -> f64 { libm::sin(x) }", "are called as `libm::sin` in `share_per_tick`"),
    ("fmath.rs", "const ONE: f64 = 1.0;\nconst E: f64 = libm::exp(ONE);", "2: `libm`: the maths routines are called as `libm::exp` in `no function`"),
    ("fmath.rs", "use libm::{exp};\nfn share_per_tick(x: f64) -> f64 { exp(x) }", "`libm`: the maths routines are named but not called by a full path"),
    ("fmath.rs", "use libm::*;\nfn share_per_tick(x: f64) -> f64 { exp(x) }", "`libm`: the maths routines are named but not called by a full path"),
    ("fmath.rs", "pub use libm::exp;", "`libm`: the maths routines are named but not called by a full path"),
    ("fmath.rs", "fn facing_of(x: f64) -> f64 { libm::math::sincos(x).0 }", "`libm`: the maths routines are named but not called by a full path"),
    ("fmath.rs", "fn share_per_tick() -> fn(f64) -> f64 { libm::exp }", "`libm`: the maths routines are named but not called by a full path"),
    ("movement.rs", "fn f(x: f64) -> f64 { libm::exp(x) }", "`libm`: the maths routines are named outside `fmath.rs`"),
    ("planted.rs", "fn f(a: f32, b: f32, c: f32) -> f32 { fmaf(a, b, c) }", "`fmaf`: a fused multiply-add"),
    ("planted.rs", "fn f(a: f64, b: f64, c: f64) -> f64 { libm::fma(a, b, c) }", "`fma`: a fused multiply-add"),
    ("collision.rs", "use parry3d::simba::scalar::ComplexField;\nfn f(a: f32) -> f32 { ComplexField::fma_round(a) }", "2: `fma_round`"),
    ("planted.rs", "extern \"C\" { fn fmaf128(a: f32, b: f32, c: f32) -> f32; }", "`fmaf128`"),
    // A target feature that picks code, on any item but a build's refusal; the attribute's other spellings.
    ("planted.rs", "#[cfg(target_feature = \"fma\")] fn f() {}", "`target_feature`"),
    ("planted.rs", "#[cfg(not(target_feature = \"avx2\"))] const WIDE: bool = true; compile_error!(\"x\");", "`target_feature`"),
    ("planted.rs", "#[cfg_attr(target_feature = \"fma\", inline)] compile_error!(\"x\");", "`target_feature`"),
    ("planted.rs", "fn f() -> bool { cfg!(target_feature = \"fma\") }", "`target_feature`"),
    // The library: outside its one file, in a file of the same name one folder down, handed on, renamed.
    ("world.rs", "use parry3d::query::Ray;", "`parry3d`: the collision library is named outside `collision.rs`"),
    ("world.rs", "fn f(ray: &parry3d::query::Ray) {}", "`parry3d`: the collision library is named outside `collision.rs`"),
    ("map/collision.rs", "use parry3d::query::Ray;", "`parry3d`: the collision library is named outside `collision.rs`"),
    ("lib.rs", "pub use collision::StaticWorld;\nextern crate parry3d;", "2: `parry3d`: the collision library is named outside"),
    ("collision.rs", "pub use parry3d::query::Ray;", "`parry3d`: the collision library is handed on with `pub`"),
    ("collision.rs", "fn f() {}\npub(crate) use parry3d::{bounding_volume::Aabb, query::Ray};", "2: `parry3d`: the collision library is handed on with `pub`"),
    ("collision.rs", "use parry3d as shapes;", "`parry3d`: the collision library is renamed with `as`"),
    ("collision.rs", "struct S;\nuse parry3d::query::{Ray as Line, RayCast};", "2: `parry3d`: the collision library is renamed with `as`"),
];

/// And what those rules let through: a build's refusal under a target feature, the library in its own file with
/// its own names, words that only look like the banned ones, and the maths rule's two allowed calls.
#[rustfmt::skip]
const LATER_CONTROLS: [(&str, &str); 9] = [
    ("fmath.rs", "pub fn exp_of(x: f64) -> f64 { libm::exp(x) }\npub fn share_per_tick(rate: f64, dt: f64) -> f32 { (1.0 - exp_of(-(rate * dt))) as f32 }"),
    ("fmath.rs", "pub fn sincos_of(x: f64) -> (f64, f64) { libm::sincos(x) }\npub fn facing_of(deg: f64) -> [f32; 2] { let (s, c) = sincos_of(deg * K); [s as f32, 0.0 - c as f32] }"),
    ("world.rs", "// libm, exp_of and sincos_of live in fmath.rs\nfn f(libmx: u32, exp_offset: u32, my_exp_of: u32) -> &'static str { \"libm::exp(x) and sincos_of\" }"),
    ("lib.rs", "#[cfg(any(target_feature = \"fma\", target_feature = \"avx2\"))]\ncompile_error!(\"the simulation is built without them\");"),
    ("lib.rs", "#[cfg(target_feature = \"fma\")] compile_error!(\"x\"); fn f(fmax: u32, format: u32) -> u32 { fmax + format }"),
    ("collision.rs", "use parry3d::bounding_volume::Aabb;\nuse parry3d::query::{Ray, RayCast};\nfn f(ray: &Ray, shape: &parry3d::shape::Cuboid) -> f32 { ray.dir.x as f32 }"),
    ("collision.rs", "pub struct StaticWorld { tree: parry3d::partitioning::Bvh }\npub fn f(n: usize) -> u32 { n as u32 }"),
    ("world.rs", "// parry3d is named in collision.rs only\nfn f() -> &'static str { \"parry3d and fmaf\" }"),
    ("world.rs", "use crate::collision::StaticWorld; fn f(world: &StaticWorld, parry: u32) -> u32 { parry }"),
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

#[test]
fn the_later_rules_find_each_planted_breach_once_and_pass_what_they_allow() {
    for (file, source, must) in LATER_BITES {
        let found = breaches(file, source);
        assert!(
            found.len() == 1 && found[0].starts_with(file) && found[0].contains(must),
            "{file}: {source}\nfound: {found:#?}\nwanted one finding with {must}"
        );
    }
    for (file, source) in LATER_CONTROLS {
        assert_eq!(
            breaches(file, source),
            Vec::<String>::new(),
            "{file}: {source}"
        );
    }
    // The library's one file is not there yet (the static collision world brings it): the rule holds from now on.
    assert!(FUSED.iter().all(|name| !STD.contains(name)) && STD.contains(&"mul_add"));
}

/// What the content crate may not call outside its tests: it refuses what is wrong and never panics on any input.
/// Its own lint guard says so (`crates/content/src/lib.rs`), but only clippy holds that guard and the toolchain job
/// runs no clippy; this scan holds, in the plain `cargo test`, the calls below wherever they are written as a
/// method or after a path, until a lint step does. It is narrower than the guard on one side (an index out of range
/// and an overflowing sum are not things a scan of text can see) and wider on another (the guard lets `assert!`
/// through).
const PANIC_CALLS: [&str; 4] = ["unwrap", "expect", "unwrap_err", "expect_err"];
#[rustfmt::skip]
const PANIC_MACROS: [&str; 10] = [
    "panic", "unreachable", "todo", "unimplemented", "assert", "assert_eq", "assert_ne", "debug_assert",
    "debug_assert_eq", "debug_assert_ne",
];
/// The content crate's breaches in one file: the simulation's rules without its two allowances (the float list:
/// the loader's arithmetic on doubles is judged where it enters the players' file; and part 9's escape: no hashed
/// map anywhere in this crate), and a call that can panic outside the tests.
fn content_breaches(file: &str, source: &str) -> Vec<String> {
    // The narrow marking, shared with devtools' layering test (F5: one home, so the two cannot drift).
    let t = marking::tokens(source);
    let tests = marking::test_only(&t);
    let mut found = breaches_of(file, source, false);
    for (i, token) in t.iter().enumerate() {
        let name = token.text.as_str();
        let before = i.checked_sub(1).map_or("", |at| t[at].text.as_str());
        let next = t.get(i + 1).map_or("", |token| token.text.as_str());
        let call = PANIC_CALLS.contains(&name) && matches!(before, "." | "::") && next == "(";
        if (call || (PANIC_MACROS.contains(&name) && next == "!")) && !tests[i] {
            found.push(format!(
                "{file}:{}: `{name}`: the content crate refuses what is wrong and never panics (its lint guard, ADR 0005 CD-2): return the refusal",
                token.line
            ));
        }
    }
    found
}

#[test]
fn the_content_crate_keeps_its_rules() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    let name = |file: &PathBuf| {
        file.strip_prefix(&src)
            .expect("under src")
            .display()
            .to_string()
    };
    let names: Vec<String> = files.iter().map(name).collect();
    // A scan of the wrong folder would find nothing and pass.
    assert!(
        ["lib.rs", "json.rs", "bundle.rs", "map.rs"]
            .iter()
            .all(|file| names.iter().any(|name| name == file)),
        "not the content crate's sources: {names:?}"
    );
    let read = |file: &PathBuf| fs::read_to_string(file).expect("a readable file");
    let found: Vec<String> = files
        .iter()
        .zip(&names)
        .flat_map(|(file, name)| content_breaches(name, &read(file)))
        .collect();
    assert!(
        found.is_empty(),
        "the content crate breaks its rules:\n{}",
        found.join("\n")
    );
    // The guard this scan stands beside is still the crate's first words: no `unsafe`, and every one of its seven
    // lints denied outside the tests (read as tokens: a guard turned to `allow`, or one lint gone, fails here).
    let words: Vec<String> = tokens(&read(&src.join("lib.rs")))
        .into_iter()
        .map(|token| token.text)
        .collect();
    let guard = words.join(" ");
    assert!(
        guard.contains("# ! [ forbid ( unsafe_code ) ]"),
        "no `#![forbid(unsafe_code)]` in crates/content/src/lib.rs"
    );
    let denied = guard
        .split_once("# ! [ cfg_attr ( not ( test ) , deny (")
        .map(|(_, rest)| rest);
    let denied: Vec<&str> = denied
        .and_then(|rest| rest.split_once(')'))
        .map_or(Vec::new(), |(lints, _)| lints.split(' ').collect());
    #[rustfmt::skip]
    let lints = ["unwrap_used", "expect_used", "indexing_slicing", "panic", "unreachable", "todo", "arithmetic_side_effects"];
    for lint in lints {
        assert!(
            denied.contains(&lint),
            "crates/content/src/lib.rs no longer denies clippy::{lint} outside its tests"
        );
    }
}

#[rustfmt::skip]
const CONTENT_BITES: [(&str, &str); 24] = [
    // The escaped quote as a character, written without a space (security-auditor's M1): the tests module ends at its
    // own brace, so the call after it is read as shipped.
    ("#[cfg(test)] mod tests { const Q: [char; 2] = ['\\'','{']; }\nfn f(x: Option<u8>) -> u8 { x.unwrap() }", "2: `unwrap`"),
    // The narrow marking (support/marking.rs, change-set 4c's F5): `#[cfg(test)]` marks a top-level `mod NAME { .. }`
    // and nothing else, so a test-only `const` or `fn` outside a tests module is read as shipped (a false red on the
    // safe side: it moves into the tests module); and a raw name is no keyword, so a parameter `r#fn` under
    // `#[cfg(test)]` hides no body.
    ("#[cfg(test)]\n#[allow(dead_code)]\npub(crate) const X: [u8; 2] = [Some(1_u8).unwrap(), 2];\nfn f() {}", "3: `unwrap`"),
    ("#[cfg(test)]\npub fn g(x: Option<u8>) -> u8 { x.unwrap() }\nfn f() {}", "2: `unwrap`"),
    ("fn f(#[cfg(test)] r#fn: u8) -> u8 { x.unwrap() }", "`unwrap`"),
    // A file is test-only as a whole only by `#![cfg(test)]` itself, at its start: another condition, or the
    // words further down, mark nothing.
    ("#![cfg(not(test))]\nfn f(x: Option<u8>) -> u8 { x.unwrap() }", "2: `unwrap`"),
    ("#![cfg(any(test, debug_assertions))]\nfn f(x: Option<u8>) -> u8 { x.unwrap() }", "2: `unwrap`"),
    ("fn g() {}\n#![cfg(test)]\nfn f(x: Option<u8>) -> u8 { x.unwrap() }", "3: `unwrap`"),
    ("fn f(x: Option<u8>) -> u8 { Option::unwrap(x) }", "`unwrap`"),
    ("fn f(x: Result<u8, u8>) -> u8 { Result::expect(x, \"it is there\") }", "`expect`"),
    ("fn f(x: Result<u8, u8>) -> u8 { x.unwrap_err() }", "`unwrap_err`"),
    ("fn f(x: Result<u8, u8>) -> u8 { x.expect_err(\"it is not\") }", "`expect_err`"),
    ("fn f(x: u8) { assert!(x > 0); }", "`assert`"),
    ("fn f(x: u8) { debug_assert_eq!(x, 0); }", "`debug_assert_eq`"),
    ("fn f(x: u8) -> u8 { match x { #[cfg(test)] 0 => 1, _ => 2 } }\nfn g(x: Option<u8>) -> u8 { x.unwrap() }", "2: `unwrap`"),
    ("#[allow(clippy::disallowed_types, reason = \"an escape the simulation has\")]\nstruct S(std::collections::HashSet<u8>);", "`HashSet`"),
    ("fn f(x: Option<u8>) -> u8 { x.unwrap() }", "`unwrap`"),
    ("fn f(x: Result<u8, u8>) -> u8 { x.expect(\"it is there\") }", "`expect`"),
    ("fn f() { panic!(\"no\") }", "`panic`"),
    ("fn f(x: u8) -> u8 { match x { 0 => 1, _ => unreachable!() } }", "`unreachable`"),
    ("fn f() { todo!() }", "`todo`"),
    ("fn f() { unimplemented!() }", "`unimplemented`"),
    ("#[cfg(test)] mod tests { fn g() { let _ = Some(1).unwrap(); } }\nfn f(x: Option<u8>) -> u8 { x.unwrap() }", "2: `unwrap`"),
    ("fn f() { let _ = std::collections::HashMap::<u8, u8>::new(); }", "`HashMap`"),
    ("fn f() { let _ = Instant::now(); }", "`Instant`"),
];

#[rustfmt::skip]
const CONTENT_CONTROLS: [&str; 8] = [
    "//! A test-only file, like the export test of the generated types.\n\n#![cfg(test)]\n\nuse std::fs;\nfn f(x: Option<u8>) -> u8 { assert!(x.is_some()); fs::read(\"x\").expect(\"a file\"); x.unwrap() }",
    "#[cfg(test)]\n#[allow(dead_code)]\nmod helpers {\n    pub(crate) const X: [u8; 2] = [Some(1_u8).unwrap(), 2];\n    pub fn g(x: Option<u8>) -> u8 { assert!(x.is_some()); x.unwrap() }\n}\nfn f() {}",
    "fn f(x: Option<u8>) -> u8 { x.unwrap_or(0) }",
    "fn f(x: Result<u8, u8>) -> u8 { x.unwrap_or_default() }",
    "#[cfg(test)]\nmod tests {\n    #[test]\n    fn g() { let x: Option<u8> = None; assert!(x.is_none()); panic!(\"a test may\"); }\n    fn h(x: Option<u8>) -> u8 { x.expect(\"in a test\") }\n}",
    "#[cfg(test)]\nuse std::fmt::Debug;\nfn f() {}",
    "fn f(x: f64) -> f64 { (x * 100.0).round().sqrt().sin() }",
    "// x.unwrap() and panic!(\"in prose\")\nfn expect(x: u8) -> u8 { x }\nfn f() -> u8 { expect(1) }",
];

#[test]
fn each_breach_planted_in_content_code_is_found_once_and_by_name() {
    for (source, must) in CONTENT_BITES {
        let found = content_breaches("planted.rs", source);
        assert!(
            found.len() == 1 && found[0].contains(must),
            "{source}\nfound: {found:#?}\nwanted one finding with {must}"
        );
    }
}

#[test]
fn what_the_content_crates_rules_allow_passes() {
    for source in CONTENT_CONTROLS {
        assert_eq!(
            content_breaches("control.rs", source),
            Vec::<String>::new(),
            "{source}"
        );
    }
}
