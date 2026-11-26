//! Bytes to a tree: the one grammar the content files are written in, and nothing else of JSON.
//!
//! Mustafa's words for this reader (2026-11-26): "Keep the reader to what the content files need and refuse
//! everything else, duplicate keys included." A document holds line feeds and printable ASCII only; its values are
//! objects, arrays, strings and numbers; its top value is an object. Refused, each by its own name: `null`, `true`,
//! `false`, `NaN`, `Infinity`; a byte-order mark; a comment; a trailing comma; a tab or a carriage return; every
//! string escape but `\"` and `\n` (the two the bundle's own text needs); a key that is not a lower-case name; a
//! key that is there twice; a number that is not written the one way the canonical writer writes it; more than
//! [`MAX_DEPTH`] levels or [`MAX_NODES`] values. Every node keeps the line and column it starts at.
//!
//! Nothing here panics on any bytes, and the work is bounded by the input's length: no value is read before its
//! depth and the node count are checked.

use crate::error::{Echo, Error, Kind};
use crate::number::{NumberError, number_text, parse_number};

/// The deepest nesting a document may have (the top object is level 1).
pub const MAX_DEPTH: u32 = 32;
/// The most values one document may hold, the top object included.
pub const MAX_NODES: u32 = 16_384;
/// The longest text a number has in canonical form (`-0.0000012345678901234567`): a longer one is refused unread.
pub const MAX_NUMBER_BYTES: usize = 25;
/// The longest key and the longest id.
pub const MAX_NAME_BYTES: usize = 64;

/// One value of a document, with where its text starts (line and column in bytes, both from 1).
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub line: u32,
    pub col: u32,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// The members in the order the text has them (a canonical text has them sorted by key).
    Object(Vec<Member>),
    Array(Vec<Node>),
    Text(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub key: String,
    /// Where the key's text starts.
    pub line: u32,
    pub col: u32,
    pub node: Node,
}

impl Value {
    /// The value's kind as a word for a message.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Object(_) => "an object",
            Self::Array(_) => "an array",
            Self::Text(_) => "a string",
            Self::Number(_) => "a number",
        }
    }
}

/// Whether `key` is a key's grammar: a lower-case letter, then lower-case letters, digits and underscores, at most
/// [`MAX_NAME_BYTES`] in all. (It leaves no room for an empty key or for `__proto__`.)
pub fn is_key(key: &str) -> bool {
    let mut bytes = key.bytes();
    key.len() <= MAX_NAME_BYTES
        && bytes.next().is_some_and(|first| first.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

/// Whether `id` is an id's grammar: a lower-case letter or a digit, then those, `-` and `_`, at most
/// [`MAX_NAME_BYTES`] in all. An id is also a file's name, so it starts with no `-` and is none of the names Windows
/// keeps for its devices (`con`, `prn`, `aux`, `nul`, `com0` to `com9`, `lpt0` to `lpt9`): no file of such a name can
/// be made there, whatever its ending.
pub fn is_id(id: &str) -> bool {
    let port = |name: &str| {
        id.strip_prefix(name)
            .is_some_and(|rest| rest.len() == 1 && rest.as_bytes().iter().all(u8::is_ascii_digit))
    };
    let mut bytes = id.bytes();
    id.len() <= MAX_NAME_BYTES
        && !["con", "prn", "aux", "nul"].contains(&id)
        && !port("com")
        && !port("lpt")
        && bytes
            .next()
            .is_some_and(|first| first.is_ascii_lowercase() || first.is_ascii_digit())
        && bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'_'
        })
}

/// Reads one document. The text's form (sorted keys, the indent) is not judged here: `crate::document` compares the
/// whole text with the canonical writer's afterwards.
pub fn parse(bytes: &[u8]) -> Result<Node, Error> {
    let mut parser = Parser {
        bytes,
        at: 0,
        line: 1,
        col: 1,
        nodes: 0,
        path: String::new(),
    };
    if bytes.is_empty() {
        return Err(parser.error(Kind::Empty));
    }
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(parser.error(Kind::ByteOrderMark));
    }
    parser.space()?;
    match parser.peek() {
        None => return Err(parser.error(Kind::Empty)),
        Some(b'{') => {}
        Some(byte) => {
            // A value of another kind, a refused word or a stray byte: named for what it is where that says more.
            parser.refuse_start(byte)?;
            return Err(parser.error(Kind::TopLevelNotObject));
        }
    }
    let top = parser.value(1)?;
    parser.space()?;
    if parser.peek().is_some() {
        return Err(parser.error(Kind::TrailingData));
    }
    Ok(top)
}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
    line: u32,
    col: u32,
    nodes: u32,
    /// The field the parser stands in, as the typed reader would name it: `shapes[3].min`.
    path: String,
}

impl Parser<'_> {
    fn error(&self, kind: Kind) -> Error {
        Error::at(self.line, self.col, &self.path, kind)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    /// Steps over one byte.
    fn bump(&mut self) {
        if self.peek() == Some(b'\n') {
            self.line = self.line.saturating_add(1);
            self.col = 1;
        } else {
            self.col = self.col.saturating_add(1);
        }
        self.at = self.at.saturating_add(1);
    }

    /// Steps over spaces and line feeds, the only white space a content file has.
    fn space(&mut self) -> Result<(), Error> {
        loop {
            match self.peek() {
                Some(b' ' | b'\n') => self.bump(),
                Some(byte @ (b'\t' | b'\r')) => return Err(self.error(Kind::Whitespace { byte })),
                Some(b'/') => return Err(self.error(Kind::Comment)),
                // Named for what it is wherever it stands, not for what was expected in its place.
                Some(byte) if !(0x20..=0x7e).contains(&byte) => {
                    return Err(self.error(Kind::ForbiddenByte { byte }));
                }
                _ => return Ok(()),
            }
        }
    }

    /// The refusals a value's first byte can already name; `Ok` when the byte may start a value.
    fn refuse_start(&mut self, byte: u8) -> Result<(), Error> {
        if byte.is_ascii_alphabetic() {
            let word = self.bytes.get(self.at..).unwrap_or(&[]);
            let length = word
                .iter()
                .take_while(|byte| byte.is_ascii_alphabetic())
                .count();
            let word = word.get(..length).unwrap_or(&[]);
            let refused = ["null", "true", "false", "NaN", "Infinity"]
                .into_iter()
                .find(|refused| refused.as_bytes() == word);
            return Err(self.error(match refused {
                Some(word) => Kind::RefusedLiteral { word },
                None => Kind::UnexpectedByte {
                    byte,
                    expected: "a value",
                },
            }));
        }
        if byte != b'\n' && !(0x20..=0x7e).contains(&byte) {
            return Err(self.error(Kind::ForbiddenByte { byte }));
        }
        Ok(())
    }

    /// One value. `depth` is the level it would stand at; a nested value is refused before it is entered.
    fn value(&mut self, depth: u32) -> Result<Node, Error> {
        let (line, col) = (self.line, self.col);
        let Some(byte) = self.peek() else {
            return Err(self.error(Kind::Unterminated));
        };
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > MAX_NODES {
            return Err(self.error(Kind::TooManyNodes { cap: MAX_NODES }));
        }
        let value = match byte {
            b'{' | b'[' if depth > MAX_DEPTH => {
                return Err(self.error(Kind::TooDeep { cap: MAX_DEPTH }));
            }
            b'{' => self.object(depth)?,
            b'[' => self.array(depth)?,
            b'"' => Value::Text(self.string()?),
            b'-' | b'0'..=b'9' => Value::Number(self.number()?),
            other => {
                self.refuse_start(other)?;
                return Err(self.error(Kind::UnexpectedByte {
                    byte: other,
                    expected: "a value",
                }));
            }
        };
        Ok(Node { line, col, value })
    }

    /// After a comma: the next entry, never the closing bracket.
    fn after_comma(&mut self, close: u8) -> Result<(), Error> {
        self.space()?;
        if self.peek() == Some(close) {
            return Err(self.error(Kind::TrailingComma));
        }
        Ok(())
    }

    fn object(&mut self, depth: u32) -> Result<Value, Error> {
        self.bump();
        let mut members: Vec<Member> = Vec::new();
        let outer = self.path.len();
        self.space()?;
        if self.peek() == Some(b'}') {
            self.bump();
            return Ok(Value::Object(members));
        }
        loop {
            let (line, col) = (self.line, self.col);
            match self.peek() {
                Some(b'"') => {}
                Some(byte) => {
                    return Err(self.error(Kind::UnexpectedByte {
                        byte,
                        expected: "a key",
                    }));
                }
                None => return Err(self.error(Kind::Unterminated)),
            }
            let key = self.string()?;
            if !is_key(&key) {
                return Err(Error::at(
                    line,
                    col,
                    &self.path,
                    Kind::BadKey {
                        key: Echo::of(key.as_bytes()),
                    },
                ));
            }
            if outer != 0 {
                self.path.push('.');
            }
            self.path.push_str(&key);
            self.space()?;
            match self.peek() {
                Some(b':') => self.bump(),
                Some(byte) => {
                    return Err(self.error(Kind::UnexpectedByte {
                        byte,
                        expected: "a colon",
                    }));
                }
                None => return Err(self.error(Kind::Unterminated)),
            }
            self.space()?;
            let node = self.value(depth.saturating_add(1))?;
            self.path.truncate(outer);
            members.push(Member {
                key,
                line,
                col,
                node,
            });
            self.space()?;
            match self.peek() {
                Some(b',') => {
                    self.bump();
                    self.after_comma(b'}')?;
                }
                Some(b'}') => {
                    self.bump();
                    break;
                }
                Some(byte) => {
                    return Err(self.error(Kind::UnexpectedByte {
                        byte,
                        expected: "a comma or the object's end",
                    }));
                }
                None => return Err(self.error(Kind::Unterminated)),
            }
        }
        // The list is cut to its length: a list that grew keeps room for more (four members after the first), and a
        // tree of many small objects would hold several times what it needs.
        members.shrink_to_fit();
        // A key may be there once. Sorting the members' places by key (and by place, so the order is total and the
        // same on every target) puts equal keys side by side; the later one is named. No hashed map anywhere.
        let mut order: Vec<usize> = (0..members.len()).collect();
        order.sort_unstable_by(|&a, &b| {
            let key = |at: usize| members.get(at).map(|member| member.key.as_str());
            key(a).cmp(&key(b)).then(a.cmp(&b))
        });
        for pair in order.windows(2) {
            if let [first, second] = *pair
                && let (Some(first), Some(second)) = (members.get(first), members.get(second))
                && first.key == second.key
            {
                let kind = Kind::DuplicateKey {
                    key: Echo::of(second.key.as_bytes()),
                };
                return Err(Error::at(second.line, second.col, &self.path, kind));
            }
        }
        Ok(Value::Object(members))
    }

    fn array(&mut self, depth: u32) -> Result<Value, Error> {
        self.bump();
        let mut items = Vec::new();
        let outer = self.path.len();
        self.space()?;
        if self.peek() == Some(b']') {
            self.bump();
            return Ok(Value::Array(items));
        }
        loop {
            self.path.push('[');
            self.path.push_str(&items.len().to_string());
            self.path.push(']');
            items.push(self.value(depth.saturating_add(1))?);
            self.path.truncate(outer);
            self.space()?;
            match self.peek() {
                Some(b',') => {
                    self.bump();
                    self.after_comma(b']')?;
                }
                Some(b']') => {
                    self.bump();
                    // Cut to its length, as an object's list is.
                    items.shrink_to_fit();
                    return Ok(Value::Array(items));
                }
                Some(byte) => {
                    return Err(self.error(Kind::UnexpectedByte {
                        byte,
                        expected: "a comma or the array's end",
                    }));
                }
                None => return Err(self.error(Kind::Unterminated)),
            }
        }
    }

    /// A string, from its opening quote to its closing one: printable ASCII, and the two escapes `\"` and `\n`.
    fn string(&mut self) -> Result<String, Error> {
        self.bump();
        // The text's own length, counted before any of it is kept: a string that grows as it is read can hold twice
        // its size when it ends, and a bundle's files are strings of up to a megabyte. The count stops where the
        // read below refuses, so a string that is refused takes no room for what follows the refusal.
        let (mut length, mut escaped) = (0_usize, false);
        for &byte in self.bytes.get(self.at..).unwrap_or(&[]) {
            match (escaped, byte) {
                (false, b'\\') => escaped = true,
                (true, b'"' | b'n') | (false, b' ' | b'!' | b'#'..=b'~') => {
                    escaped = false;
                    length = length.saturating_add(1);
                }
                _ => break,
            }
        }
        let mut text = String::with_capacity(length);
        loop {
            let Some(byte) = self.peek() else {
                return Err(self.error(Kind::Unterminated));
            };
            match byte {
                b'"' => {
                    self.bump();
                    return Ok(text);
                }
                b'\\' => {
                    self.bump();
                    match self.peek() {
                        Some(b'"') => text.push('"'),
                        Some(b'n') => text.push('\n'),
                        Some(byte) => return Err(self.error(Kind::RefusedEscape { byte })),
                        None => return Err(self.error(Kind::Unterminated)),
                    }
                    self.bump();
                }
                0x20..=0x7e => {
                    text.push(char::from(byte));
                    self.bump();
                }
                _ => return Err(self.error(Kind::ForbiddenByte { byte })),
            }
        }
    }

    /// A number, taken only when its text is the canonical writer's own text for its value.
    fn number(&mut self) -> Result<f64, Error> {
        let start = self.at;
        let rest = self.bytes.get(start..).unwrap_or(&[]);
        let length = rest
            .iter()
            .take_while(|byte| matches!(byte, b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E'))
            .count();
        let text = rest.get(..length).unwrap_or(&[]);
        if length > MAX_NUMBER_BYTES {
            return Err(self.error(Kind::NumberTooLong {
                len: u32::try_from(length).unwrap_or(u32::MAX),
                cap: MAX_NUMBER_BYTES as u32,
            }));
        }
        let value = match parse_number(text) {
            Ok(value) => value,
            Err(NumberError::Grammar) => {
                return Err(self.error(Kind::NumberGrammar {
                    text: Echo::of(text),
                }));
            }
            Err(NumberError::NotFinite) => {
                return Err(self.error(Kind::NumberNotFinite {
                    text: Echo::of(text),
                }));
            }
        };
        let canonical = number_text(value);
        if canonical.as_bytes() != text {
            let kind = Kind::NumberNotCanonical {
                text: Echo::of(text),
                canonical: Echo::of(canonical.as_bytes()),
            };
            return Err(self.error(kind));
        }
        for _ in 0..length {
            self.bump();
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind_of(text: &str) -> Kind {
        parse(text.as_bytes()).expect_err(text).kind
    }
    fn echo(text: &str) -> Echo {
        Echo::of(text.as_bytes())
    }

    #[test]
    fn a_document_of_the_four_kinds_reads_with_its_places() {
        let top = parse(b"{\n  \"a\": [1, 2.5, \"x\\\"y\\nz\"],\n  \"b\": {\"c\": -3}\n}\n")
            .expect("valid");
        let Value::Object(members) = &top.value else {
            panic!("not an object")
        };
        assert_eq!((top.line, top.col), (1, 1));
        let keys: Vec<&str> = members.iter().map(|member| member.key.as_str()).collect();
        assert_eq!(keys, ["a", "b"]);
        let Value::Array(items) = &members[0].node.value else {
            panic!("not an array")
        };
        assert_eq!(
            (items[0].line, items[0].col, &items[0].value),
            (2, 9, &Value::Number(1.0))
        );
        assert_eq!(items[1].value, Value::Number(2.5));
        assert_eq!(items[2].value, Value::Text("x\"y\nz".to_owned()));
        assert_eq!((members[1].line, members[1].col), (3, 3));
        assert_eq!((members[1].node.line, members[1].node.col), (3, 8));
        assert_eq!(
            parse(b"{}").expect("the empty object").value,
            Value::Object(Vec::new())
        );
        assert_eq!(
            parse(b" \n{\"a\": []} \n")
                .expect("spaces and line feeds around it")
                .line,
            2
        );
    }

    #[test]
    fn everything_else_of_json_is_refused_by_its_own_name() {
        let unexpected = |byte: u8, expected: &'static str| Kind::UnexpectedByte { byte, expected };
        #[rustfmt::skip]
        let cases = [
            ("", Kind::Empty),
            (" \n ", Kind::Empty),
            ("\u{feff}{}", Kind::ByteOrderMark),
            ("[]", Kind::TopLevelNotObject),
            ("\"a\"", Kind::TopLevelNotObject),
            ("1", Kind::TopLevelNotObject),
            ("null", Kind::RefusedLiteral { word: "null" }),
            ("{} {}", Kind::TrailingData),
            ("{}x", Kind::TrailingData),
            ("{\"a\": null}", Kind::RefusedLiteral { word: "null" }),
            ("{\"a\": true}", Kind::RefusedLiteral { word: "true" }),
            ("{\"a\": false}", Kind::RefusedLiteral { word: "false" }),
            ("{\"a\": NaN}", Kind::RefusedLiteral { word: "NaN" }),
            ("{\"a\": Infinity}", Kind::RefusedLiteral { word: "Infinity" }),
            ("{\"a\": nul}", unexpected(b'n', "a value")),
            ("{\"a\": 1,}", Kind::TrailingComma),
            ("{\"a\": [1,]}", Kind::TrailingComma),
            ("{\"a\": 1 // x\n}", Kind::Comment),
            ("{/* x */}", Kind::Comment),
            ("{\t\"a\": 1}", Kind::Whitespace { byte: b'\t' }),
            ("{\"a\": 1}\r\n", Kind::Whitespace { byte: b'\r' }),
            ("{\"a\": 1", Kind::Unterminated),
            ("{\"a\": \"x", Kind::Unterminated),
            ("{\"a\": \"x\\", Kind::Unterminated),
            ("{\"a\"", Kind::Unterminated),
            ("{\"a\":", Kind::Unterminated),
            ("{", Kind::Unterminated),
            ("{\"a\": [", Kind::Unterminated),
            ("{\"a\" 1}", unexpected(b'1', "a colon")),
            ("{\"a\": 1 \"b\": 2}", unexpected(b'"', "a comma or the object's end")),
            ("{\"a\": [1 2]}", unexpected(b'2', "a comma or the array's end")),
            ("{a: 1}", unexpected(b'a', "a key")),
            ("{\"a\": }", unexpected(b'}', "a value")),
            ("{\"a\": 'x'}", unexpected(b'\'', "a value")),
            ("{\"a\": \"\\t\"}", Kind::RefusedEscape { byte: b't' }),
            ("{\"a\": \"\\\\\"}", Kind::RefusedEscape { byte: b'\\' }),
            ("{\"a\": \"\\/\"}", Kind::RefusedEscape { byte: b'/' }),
            ("{\"a\": \"\\u0041\"}", Kind::RefusedEscape { byte: b'u' }),
            ("{\"a\": \"\\r\"}", Kind::RefusedEscape { byte: b'r' }),
            ("{\"a\": \"\\b\"}", Kind::RefusedEscape { byte: b'b' }),
            ("{\"a\": \"\\f\"}", Kind::RefusedEscape { byte: b'f' }),
            ("{\"a\": \"x\ny\"}", Kind::ForbiddenByte { byte: b'\n' }),
            ("{\"a\": \"x\ty\"}", Kind::ForbiddenByte { byte: b'\t' }),
            ("{\"a\": \"\u{7f}\"}", Kind::ForbiddenByte { byte: 0x7f }),
            ("{\"a\": \"\u{e9}\"}", Kind::ForbiddenByte { byte: 0xc3 }),
            ("{\"a\": \u{e9}}", Kind::ForbiddenByte { byte: 0xc3 }),
            ("{\u{1}\"a\": 1}", Kind::ForbiddenByte { byte: 1 }),
            ("{\"a\"\u{0}: 1}", Kind::ForbiddenByte { byte: 0 }),
            ("{\"a\": 1\u{e9}}", Kind::ForbiddenByte { byte: 0xc3 }),
            ("{}\u{7f}", Kind::ForbiddenByte { byte: 0x7f }),
            ("{\"\": 1}", Kind::BadKey { key: echo("") }),
            ("{\"A\": 1}", Kind::BadKey { key: echo("A") }),
            ("{\"1a\": 1}", Kind::BadKey { key: echo("1a") }),
            ("{\"a-b\": 1}", Kind::BadKey { key: echo("a-b") }),
            ("{\"__proto__\": 1}", Kind::BadKey { key: echo("__proto__") }),
            ("{\"a\": 1, \"a\": 2}", Kind::DuplicateKey { key: echo("a") }),
            ("{\"b\": 1, \"a\": 2, \"b\": 3}", Kind::DuplicateKey { key: echo("b") }),
            ("{\"a\": {\"c\": 1, \"b\": 2, \"c\": 3}}", Kind::DuplicateKey { key: echo("c") }),
        ];
        for (text, kind) in cases {
            assert_eq!(kind_of(text), kind, "{text:?}");
        }
        let long_key = format!("{{\"{}\": 1}}", "k".repeat(MAX_NAME_BYTES + 1));
        assert!(matches!(kind_of(&long_key), Kind::BadKey { .. }));
        assert!(parse(format!("{{\"{}\": 1}}", "k".repeat(MAX_NAME_BYTES)).as_bytes()).is_ok());
    }

    #[test]
    fn a_number_is_taken_only_in_its_one_canonical_text() {
        #[rustfmt::skip]
        let taken = ["0", "1", "-1.5", "6.3", "0.000001", "1e-7", "1e+21", "100000000000000000000", "5e-324", "-327.67"];
        for text in taken {
            assert!(
                parse(format!("{{\"a\": {text}}}").as_bytes()).is_ok(),
                "{text}"
            );
        }
        let not = |text: &str, canonical: &str| Kind::NumberNotCanonical {
            text: echo(text),
            canonical: echo(canonical),
        };
        #[rustfmt::skip]
        let cases = [
            ("1.50", not("1.50", "1.5")),
            ("1.0", not("1.0", "1")),
            ("1E5", not("1E5", "100000")),
            ("1e5", not("1e5", "100000")),
            ("-0", not("-0", "0")),
            ("-0.0", not("-0.0", "0")),
            ("0.0000001", not("0.0000001", "1e-7")),
            ("1e21", not("1e21", "1e+21")),
            ("9007199254740993", not("9007199254740993", "9007199254740992")),
            ("0.30000000000000001", not("0.30000000000000001", "0.3")),
            ("1e400", Kind::NumberNotFinite { text: echo("1e400") }),
            ("-1e309", Kind::NumberNotFinite { text: echo("-1e309") }),
            ("01", Kind::NumberGrammar { text: echo("01") }),
            ("-01", Kind::NumberGrammar { text: echo("-01") }),
            ("1.", Kind::NumberGrammar { text: echo("1.") }),
            ("1e", Kind::NumberGrammar { text: echo("1e") }),
            ("--1", Kind::NumberGrammar { text: echo("--1") }),
            ("-", Kind::NumberGrammar { text: echo("-") }),
            ("-Infinity", Kind::NumberGrammar { text: echo("-") }),
            // "0" is a number; the "x" after it is where the object's next byte is refused.
            ("0x1F", Kind::UnexpectedByte { byte: b'x', expected: "a comma or the object's end" }),
            ("+1", Kind::UnexpectedByte { byte: b'+', expected: "a value" }),
            (".5", Kind::UnexpectedByte { byte: b'.', expected: "a value" }),
        ];
        for (text, kind) in cases {
            assert_eq!(kind_of(&format!("{{\"a\": {text}}}")), kind, "{text}");
        }
        // A number of 26 bytes or more is refused before it is read at all; 25 is the longest canonical text.
        assert!(parse(b"{\"a\": -0.0000012345678901234567}").is_ok());
        assert_eq!(
            kind_of("{\"a\": -0.00000123456789012345670}"),
            Kind::NumberTooLong { len: 26, cap: 25 }
        );
        let flood = format!("{{\"a\": 1{}}}", "0".repeat(100_000));
        assert_eq!(
            kind_of(&flood),
            Kind::NumberTooLong {
                len: 100_001,
                cap: 25
            }
        );
    }

    #[test]
    fn depth_and_size_are_capped_before_they_are_entered() {
        let nested = |depth: usize| format!("{}1{}", "{\"a\": ".repeat(depth), "}".repeat(depth));
        assert!(parse(nested(MAX_DEPTH as usize).as_bytes()).is_ok());
        assert_eq!(
            kind_of(&nested(MAX_DEPTH as usize + 1)),
            Kind::TooDeep { cap: MAX_DEPTH }
        );
        // Far deeper than any stack would take, in arrays and in objects: refused at level 33, never entered.
        assert_eq!(
            kind_of(&format!("{{\"a\": {}", "[".repeat(1_000_000))),
            Kind::TooDeep { cap: MAX_DEPTH }
        );
        assert_eq!(
            kind_of(&"{\"a\": ".repeat(1_000_000)),
            Kind::TooDeep { cap: MAX_DEPTH }
        );
        // The top object, the array and its items: MAX_NODES values pass, one more does not.
        let items = |count: usize| format!("{{\"a\": [{}]}}", vec!["0"; count].join(", "));
        assert!(parse(items(MAX_NODES as usize - 2).as_bytes()).is_ok());
        assert_eq!(
            kind_of(&items(MAX_NODES as usize - 1)),
            Kind::TooManyNodes { cap: MAX_NODES }
        );
    }

    #[test]
    fn an_error_names_its_line_its_column_and_its_field() {
        let place = |text: &str| {
            let error = parse(text.as_bytes()).expect_err(text);
            (error.line, error.col, error.path)
        };
        let refused_word = "{\n  \"shapes\": [\n    {\"min\": [0, 1, true]}\n  ]\n}\n";
        assert_eq!(place(refused_word), (3, 20, "shapes[0].min[2]".to_owned()));
        assert_eq!(
            place("{\n  \"a\": {\"b\": 1, \"b\": 2}\n}"),
            (2, 17, "a".to_owned())
        );
        assert_eq!(place("{\"a\": 1,\n\"b\": 01}"), (2, 6, "b".to_owned()));
    }

    #[test]
    fn keys_and_ids_have_their_grammars() {
        for key in ["a", "walk_speed_m_per_s", "x2", "a_"] {
            assert!(is_key(key), "{key}");
        }
        for key in ["", "A", "_a", "2x", "a-b", "a b", "__proto__", "a.b"] {
            assert!(!is_key(key), "{key}");
        }
        for id in ["soldier", "test-scene", "step_055", "0", "a-"] {
            assert!(is_id(id), "{id}");
        }
        for id in ["", "-a", "_a", "A", "a b", "a/b", "a.json", "a\n", "<img>"] {
            assert!(!is_id(id), "{id}");
        }
        assert!(is_id(&"i".repeat(MAX_NAME_BYTES)) && !is_id(&"i".repeat(MAX_NAME_BYTES + 1)));
        // An id is a file's name: none of the names Windows keeps for its devices, and every name beside them.
        for id in [
            "con", "prn", "aux", "nul", "com0", "com1", "com9", "lpt0", "lpt5", "lpt9",
        ] {
            assert!(!is_id(id), "{id}");
        }
        for id in [
            "co", "cons", "con-1", "nul_", "com", "com10", "coma", "lpt", "lpt-1", "lpt00", "1com",
            "com_1",
        ] {
            assert!(is_id(id), "{id}");
        }
    }
}
