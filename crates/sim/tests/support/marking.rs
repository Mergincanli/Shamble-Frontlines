//! Which code is test-only, read narrowly (security-auditor's third answer and its amendment, change-set 4c): the one
//! marking that devtools' `tests/layering.rs` and the content crate's panic scan in `tests/rules_scan.rs` both use, so
//! the two cannot drift (rust-reviewer's block 15, F5). A support file, not a test target (Cargo makes test targets
//! only of the files directly in `tests/`); each test takes it by `#[path]`.
//!
//! Test-only is only (i) a whole file whose first tokens are `#![cfg(test)]`, or (ii) `#[cfg(test)]`, then any further
//! `#[..]` attributes, then the keyword `mod` (after a `pub` with or without its reach, if one is written), a name and
//! `{`, standing at the file's top level (no brace, bracket or
//! parenthesis open: so never inside a macro's delimiters, a struct, a parameter list or a generic list), marked to
//! that module's closing `}`. Anything else under `#[cfg(test)]` (a `fn`, a `use`, an `impl`, a `const`) is read as
//! code the game runs: compiled out in truth, refused here all the same (a false red on the safe side; the author
//! moves it into the tests module). A raw name stays distinct from a keyword: `r#mod` is no `mod`.

/// One token: a name (a raw one without its `r#`, marked `raw`), one punctuation mark (`::` is one), or `""` and
/// `"…"` for an empty and a filled string or character. Comments, the content of strings and characters, lifetimes
/// and numbers are left out, so a delimiter in a literal or a comment opens nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub raw: bool,
    pub line: usize,
}

/// Splits Rust source into tokens.
pub fn tokens(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let at_or = |index: usize| chars.get(index).copied().unwrap_or('\0');
    let is_start = |c: char| c.is_alphabetic() || c == '_';
    let is_part = |c: char| c.is_alphanumeric() || c == '_';
    let (mut at, mut line, mut out) = (0, 1, Vec::new());
    let mut push = |text: String, raw: bool, line: usize| out.push(Token { text, raw, line });
    while at < chars.len() {
        let (c, next) = (chars[at], at_or(at + 1));
        if c == '\n' {
            line += 1;
            at += 1;
        } else if c.is_whitespace() {
            at += 1;
        } else if c == '/' && next == '/' {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
        } else if c == '/' && next == '*' {
            let mut depth = 0_usize;
            while at < chars.len() {
                match (chars[at], at_or(at + 1)) {
                    ('/', '*') => (depth, at) = (depth + 1, at + 1),
                    ('*', '/') => (depth, at) = (depth.saturating_sub(1), at + 1),
                    ('\n', _) => line += 1,
                    _ => {}
                }
                at += 1;
                if depth == 0 {
                    break;
                }
            }
        } else if let Some((hashes, open)) = string_start(&chars, at) {
            // A string of any kind (plain, byte, C, raw with its hashes): to just after its closing quote.
            let (raw, start) = (hashes.is_some(), open + 1);
            let hashes = hashes.unwrap_or(0);
            let mut end = start;
            loop {
                match at_or(end) {
                    '\0' => break,
                    '\\' if !raw => {
                        end += 1;
                        if at_or(end) == '\n' {
                            line += 1;
                        }
                    }
                    '"' if (1..=hashes).all(|k| at_or(end + k) == '#') => {
                        let text = if end > start { "\"…\"" } else { "\"\"" };
                        push(text.to_owned(), false, line);
                        end += 1 + hashes;
                        break;
                    }
                    '\n' => line += 1,
                    _ => {}
                }
                end += 1;
            }
            at = end;
        } else if c == '\'' || (c == 'b' && next == '\'') {
            // A character (`'x'`, `'\n'`, `'\''`, `b'x'`) to its closing quote; a lifetime (`'a`) is left out. An escape's
            // closing quote is looked for after the escaped character, so `'\''` ends at its own last quote
            // (security-auditor's M1).
            let open = if c == 'b' { at + 1 } else { at };
            let escaped = at_or(open + 1) == '\\';
            if escaped || at_or(open + 2) == '\'' {
                let mut end = if escaped { open + 3 } else { open + 2 };
                while at_or(end) != '\'' && at_or(end) != '\0' {
                    end += 1;
                }
                push("\"…\"".to_owned(), false, line);
                at = end + 1;
            } else {
                at = open + 1;
                while is_part(at_or(at)) {
                    at += 1;
                }
            }
        } else if c == 'r' && next == '#' && is_start(at_or(at + 2)) {
            let start = at + 2;
            at = start;
            while is_part(at_or(at)) {
                at += 1;
            }
            push(chars[start..at].iter().collect(), true, line);
        } else if is_start(c) {
            let start = at;
            while is_part(at_or(at)) {
                at += 1;
            }
            push(chars[start..at].iter().collect(), false, line);
        } else if c.is_ascii_digit() {
            // A number with its suffix; a `.` belongs to it only before a digit (`1..2` is a range).
            while is_part(at_or(at)) || (at_or(at) == '.' && at_or(at + 1).is_ascii_digit()) {
                at += 1;
            }
        } else if c == ':' && next == ':' {
            push("::".to_owned(), false, line);
            at += 2;
        } else {
            push(c.to_string(), false, line);
            at += 1;
        }
    }
    out
}

/// Where a string starts at `at`, if one does: its raw hashes (`Some(n)` for a raw string, `None` otherwise) and the
/// place of its opening quote. Prefixes: `b`, `c`, `r`, `br`, `cr`.
fn string_start(chars: &[char], at: usize) -> Option<(Option<usize>, usize)> {
    let at_or = |index: usize| chars.get(index).copied().unwrap_or('\0');
    // A prefix only counts at the start of a word: `abr"x"` is no raw string.
    let starts_word = at == 0 || !(at_or(at - 1).is_alphanumeric() || at_or(at - 1) == '_');
    let mut index = at;
    if starts_word && matches!(at_or(index), 'b' | 'c') {
        index += 1;
    }
    if starts_word && at_or(index) == 'r' {
        let mut hashes = 0;
        while at_or(index + 1 + hashes) == '#' {
            hashes += 1;
        }
        if at_or(index + 1 + hashes) == '"' {
            return Some((Some(hashes), index + 1 + hashes));
        }
        return None;
    }
    if at_or(index) == '"' && (index == at || starts_word) {
        return Some((None, index));
    }
    None
}

/// For each token, whether it stands in test-only code, read narrowly (this file's first words).
pub fn test_only(t: &[Token]) -> Vec<bool> {
    let text = |at: usize| t.get(at).map_or("", |token| token.text.as_str());
    let plain = |at: usize, word: &str| {
        t.get(at)
            .is_some_and(|token| !token.raw && token.text == word)
    };
    const FILE_MARK: [&str; 8] = ["#", "!", "[", "cfg", "(", "test", ")", "]"];
    const MARK: [&str; 7] = ["#", "[", "cfg", "(", "test", ")", "]"];
    if (0..FILE_MARK.len()).all(|k| plain(k, FILE_MARK[k])) {
        return vec![true; t.len()];
    }
    // The place after the bracket that opens at `at`, with what it holds.
    let past = |mut at: usize| {
        let mut depth = 0_usize;
        while at < t.len() {
            match text(at) {
                "[" | "(" | "{" => depth += 1,
                "]" | ")" | "}" => depth = depth.saturating_sub(1),
                _ => {}
            }
            at += 1;
            if depth == 0 {
                break;
            }
        }
        at
    };
    let mut inside = vec![false; t.len()];
    let (mut at, mut depth) = (0, 0_usize);
    while at < t.len() {
        let marked = depth == 0 && (0..MARK.len()).all(|k| plain(at + k, MARK[k]));
        if marked {
            let mut item = at + MARK.len();
            while plain(item, "#") && plain(item + 1, "[") {
                item = past(item + 1);
            }
            // A visibility, with or without its reach, changes nothing of a test-only module (content's map.rs has
            // `pub(crate) mod tests`).
            if plain(item, "pub") {
                item = if plain(item + 1, "(") {
                    past(item + 1)
                } else {
                    item + 1
                };
            }
            let named = t.get(item + 1).is_some_and(|token| {
                token
                    .text
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphabetic() || c == '_')
            });
            if plain(item, "mod") && named && plain(item + 2, "{") {
                // To the module's own closing `}`: a closer it did not open ends the mark there too.
                let mut open = 0_usize;
                let mut end = item + 2;
                while end < t.len() {
                    match text(end) {
                        "{" | "[" | "(" => open += 1,
                        "}" | "]" | ")" => {
                            open = open.saturating_sub(1);
                            if open == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    end += 1;
                }
                for slot in inside
                    .iter_mut()
                    .take(end.saturating_add(1).min(t.len()))
                    .skip(at)
                {
                    *slot = true;
                }
                at = end.saturating_add(1);
                continue;
            }
        }
        match text(at) {
            "{" | "[" | "(" => depth += 1,
            "}" | "]" | ")" => depth = depth.saturating_sub(1),
            _ => {}
        }
        at += 1;
    }
    inside
}
