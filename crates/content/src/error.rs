//! What the loader says when it refuses: which file, where in it, and why, as one record that is the same on every
//! target (the native and the WebAssembly verdicts are compared record for record), and as one plain sentence for a
//! person. Neither ever holds more than a short, escaped piece of the input: an error is shown on a page, printed in
//! a terminal and pasted into reports.

use core::fmt::{self, Write as _};

/// The most bytes of input one echo holds.
pub const ECHO_BYTES: usize = 64;
/// The most bytes [`Error::message`] returns, the cut's mark included.
pub const MESSAGE_BYTES: usize = 512;

/// A short piece of input, made safe to show: at most [`ECHO_BYTES`] bytes of it; every byte outside the printable
/// ASCII range, and the backslash, written as `\xNN`; a cut marked with three dots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Echo(String);

impl Echo {
    pub fn of(input: &[u8]) -> Self {
        let mut text = String::new();
        for &byte in input.iter().take(ECHO_BYTES) {
            if (0x20..=0x7e).contains(&byte) && byte != b'\\' {
                text.push(char::from(byte));
            } else {
                // Writing to a `String` cannot fail.
                let _ = write!(text, "\\x{byte:02X}");
            }
        }
        if input.len() > ECHO_BYTES {
            text.push_str("...");
        }
        Self(text)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Echo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Why a document, a file or a bundle is refused. One variant per reason, so that a test can name the refusal it
/// expects and "refused for a neighbouring reason" cannot pass for the right one.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    // The bytes, before any value is read.
    /// More bytes than the cap for this kind of input.
    TooLarge {
        bytes: u64,
        cap: u64,
    },
    Empty,
    ByteOrderMark,
    // The text: the one grammar the content files are written in.
    /// A byte that no content file holds: anything but a line feed and the printable ASCII range.
    ForbiddenByte {
        byte: u8,
    },
    /// A tab or a carriage return between values.
    Whitespace {
        byte: u8,
    },
    Comment,
    TrailingComma,
    TopLevelNotObject,
    TrailingData,
    Unterminated,
    UnexpectedByte {
        byte: u8,
        expected: &'static str,
    },
    /// `null`, `true`, `false`, `NaN`, `Infinity`: no content field holds one.
    RefusedLiteral {
        word: &'static str,
    },
    /// Any escape but `\"` and `\n`, named by the byte after the backslash.
    RefusedEscape {
        byte: u8,
    },
    BadKey {
        key: Echo,
    },
    DuplicateKey {
        key: Echo,
    },
    TooDeep {
        cap: u32,
    },
    TooManyNodes {
        cap: u32,
    },
    /// Longer than any canonical number: refused before it is read.
    NumberTooLong {
        len: u32,
        cap: u32,
    },
    NumberGrammar {
        text: Echo,
    },
    NumberNotFinite {
        text: Echo,
    },
    /// The number is valid, but not written the one way the canonical writer writes it.
    NumberNotCanonical {
        text: Echo,
        canonical: Echo,
    },
    /// The document is valid, but not byte for byte what the canonical writer makes of it.
    NotCanonical {
        expected: Echo,
        found: Echo,
    },
    // The typed read.
    UnknownKey {
        key: Echo,
    },
    MissingKey {
        key: &'static str,
    },
    WrongType {
        expected: &'static str,
        found: &'static str,
    },
    NotAWholeNumber {
        bits: u64,
    },
    OutOfRange {
        bits: u64,
        min_bits: u64,
        max_bits: u64,
        min_inclusive: bool,
        max_inclusive: bool,
    },
    TooMany {
        len: u32,
        max: u32,
    },
    TooFew {
        len: u32,
        min: u32,
    },
    BadId {
        id: Echo,
    },
    /// A value that must be one of a fixed list.
    NotOneOf {
        value: Echo,
        allowed: &'static str,
    },
    WrongFormat {
        found: Echo,
        expected: &'static str,
    },
    WrongVersion {
        bits: u64,
        newest: u32,
    },
    // The bundle and the whole set.
    BadPath {
        path: Echo,
    },
    /// The files of a bundle are in path order, each path once.
    PathOrder {
        path: Echo,
    },
    TooManyFiles {
        len: u32,
        cap: u32,
    },
    /// A file's `id` is its file name without `.json`.
    IdNotFileName {
        id: Echo,
    },
}

/// A refusal: the file (a bundle's path, when it is known), the place in its text (line and column in bytes, both
/// from 1; 0 and 0 for the whole document), the field (a path such as `shapes[3].min[0]`; empty when no field is
/// meant), and the reason.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub file: Option<String>,
    pub line: u32,
    pub col: u32,
    pub path: String,
    pub kind: Kind,
}

impl Error {
    /// A refusal that names no place: the whole document.
    pub fn whole(kind: Kind) -> Self {
        Self {
            file: None,
            line: 0,
            col: 0,
            path: String::new(),
            kind,
        }
    }

    pub fn at(line: u32, col: u32, path: &str, kind: Kind) -> Self {
        Self {
            file: None,
            line,
            col,
            path: path.to_owned(),
            kind,
        }
    }

    /// The same refusal, with the file it was found in.
    #[must_use]
    pub fn in_file(mut self, file: &str) -> Self {
        self.file = Some(file.to_owned());
        self
    }

    /// The refusal as one line in a fixed field order, with every number as digits and every double as its 16 hex
    /// digits: what two targets' verdicts are compared on.
    pub fn record(&self) -> String {
        let file = self.file.as_deref().unwrap_or("-");
        let path = if self.path.is_empty() {
            "-"
        } else {
            &self.path
        };
        format!(
            "{file}|{}:{}|{path}|{}",
            self.line,
            self.col,
            self.kind.record()
        )
    }

    /// One plain sentence for a person, cut to [`MESSAGE_BYTES`] bytes.
    pub fn message(&self) -> String {
        let mut text = String::new();
        let _ = write!(text, "the content is refused: {}", self.kind);
        if let Some(file) = &self.file {
            let _ = write!(text, ", in {}", Echo::of(file.as_bytes()));
        }
        if self.line != 0 {
            let _ = write!(text, ", line {} column {}", self.line, self.col);
        }
        if !self.path.is_empty() {
            let _ = write!(text, ", at {}", Echo::of(self.path.as_bytes()));
        }
        cut(text)
    }
}

/// Cuts a message to [`MESSAGE_BYTES`] bytes, marking the cut. Every byte of a message is ASCII (an echo escapes the
/// rest), so any byte position is a character boundary; a message that is not is replaced, never sliced.
fn cut(text: String) -> String {
    const MARK: &str = "...";
    if text.len() <= MESSAGE_BYTES {
        return text;
    }
    let keep = MESSAGE_BYTES.saturating_sub(MARK.len());
    match text.get(..keep) {
        Some(head) => format!("{head}{MARK}"),
        None => "the content is refused (the reason could not be shown)".to_owned(),
    }
}

impl Kind {
    /// The reason as one piece of a record: the variant's name and its payload, doubles as hex digits.
    pub fn record(&self) -> String {
        match self {
            Self::OutOfRange {
                bits,
                min_bits,
                max_bits,
                min_inclusive,
                max_inclusive,
            } => format!(
                "OutOfRange {bits:016x} {} {min_bits:016x} {max_bits:016x} {}",
                if *min_inclusive { '[' } else { '(' },
                if *max_inclusive { ']' } else { ')' }
            ),
            Self::NotAWholeNumber { bits } => format!("NotAWholeNumber {bits:016x}"),
            Self::WrongVersion { bits, newest } => format!("WrongVersion {bits:016x} {newest}"),
            // Every other payload is whole numbers, fixed words and echoes: the derived form names each field.
            other => format!("{other:?}"),
        }
    }
}

impl fmt::Display for Kind {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ends = |min: bool, max: bool| (if min { '[' } else { '(' }, if max { ']' } else { ')' });
        match self {
            Self::TooLarge { bytes, cap } => write!(f, "{bytes} bytes, more than the {cap} allowed"),
            Self::Empty => f.write_str("it is empty"),
            Self::ByteOrderMark => f.write_str("it starts with a byte-order mark"),
            Self::ForbiddenByte { byte } => write!(f, "the byte 0x{byte:02X}, which no content file holds"),
            Self::Whitespace { byte } => write!(f, "white space other than a space or a line feed (0x{byte:02X})"),
            Self::Comment => f.write_str("a comment"),
            Self::TrailingComma => f.write_str("a comma with nothing after it"),
            Self::TopLevelNotObject => f.write_str("the document is not an object"),
            Self::TrailingData => f.write_str("text after the document's end"),
            Self::Unterminated => f.write_str("the text ends inside a value"),
            Self::UnexpectedByte { byte, expected } => write!(f, "the byte 0x{byte:02X} where {expected} was expected"),
            Self::RefusedLiteral { word } => write!(f, "the word {word}, which no content field holds"),
            Self::RefusedEscape { byte } => write!(f, "an escape the content does not use (backslash, then 0x{byte:02X})"),
            Self::BadKey { key } => write!(f, "the key \"{key}\" is not a lower-case name of 1 to 64 characters"),
            Self::DuplicateKey { key } => write!(f, "the key \"{key}\" is there twice"),
            Self::TooDeep { cap } => write!(f, "values nested more than {cap} deep"),
            Self::TooManyNodes { cap } => write!(f, "more than {cap} values in one document"),
            Self::NumberTooLong { len, cap } => write!(f, "a number of {len} characters, more than the {cap} a number takes"),
            Self::NumberGrammar { text } => write!(f, "\"{text}\" is not a number"),
            Self::NumberNotFinite { text } => write!(f, "the number {text} is too large"),
            Self::NumberNotCanonical { text, canonical } => write!(f, "the number {text} must be written {canonical}"),
            Self::NotCanonical { expected, found } => write!(f, "the text is not in canonical form: \"{found}\" where \"{expected}\" belongs"),
            Self::UnknownKey { key } => write!(f, "the key \"{key}\" is not known"),
            Self::MissingKey { key } => write!(f, "the key \"{key}\" is missing"),
            Self::WrongType { expected, found } => write!(f, "{found} where {expected} belongs"),
            Self::NotAWholeNumber { bits } => write!(f, "{} is not a whole number in range", f64::from_bits(*bits)),
            Self::OutOfRange { bits, min_bits, max_bits, min_inclusive, max_inclusive } => {
                let (open, close) = ends(*min_inclusive, *max_inclusive);
                write!(f, "{} is outside {open}{}, {}{close}", f64::from_bits(*bits), f64::from_bits(*min_bits), f64::from_bits(*max_bits))
            }
            Self::TooMany { len, max } => write!(f, "{len} entries, more than the {max} allowed"),
            Self::TooFew { len, min } => write!(f, "{len} entries, fewer than the {min} needed"),
            Self::BadId { id } => write!(f, "\"{id}\" is not an id (lower-case letters, digits, - and _, 1 to 64)"),
            Self::NotOneOf { value, allowed } => write!(f, "\"{value}\" is not one of {allowed}"),
            Self::WrongFormat { found, expected } => write!(f, "the format is \"{found}\", not \"{expected}\""),
            Self::WrongVersion { bits, newest } => write!(f, "version {} is not one this build reads (newest: {newest})", f64::from_bits(*bits)),
            Self::BadPath { path } => write!(f, "\"{path}\" is not a content file's path"),
            Self::PathOrder { path } => write!(f, "\"{path}\" is out of order or there twice"),
            Self::TooManyFiles { len, cap } => write!(f, "{len} files, more than the {cap} allowed"),
            Self::IdNotFileName { id } => write!(f, "the id \"{id}\" is not the file's name"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_echo_is_short_and_shows_no_raw_byte_outside_printable_ascii() {
        assert_eq!(Echo::of(b"walk_speed").as_str(), "walk_speed");
        assert_eq!(
            Echo::of(b"a\x1b[2J\\b\x7f\x80\n").as_str(),
            "a\\x1B[2J\\x5Cb\\x7F\\x80\\x0A"
        );
        let long = Echo::of(&[b'k'; 1 << 20]);
        assert_eq!(long.as_str().len(), ECHO_BYTES + 3);
        assert!(long.as_str().ends_with("..."));
        // Exactly at the cap: whole, and no mark.
        assert_eq!(Echo::of(&[b'k'; ECHO_BYTES]).as_str().len(), ECHO_BYTES);
        // The widest echo: every byte escaped, four characters each.
        assert_eq!(Echo::of(&[0x1b; 100]).as_str().len(), ECHO_BYTES * 4 + 3);
    }

    #[test]
    fn a_message_is_capped_whatever_the_input_was() {
        let key = Echo::of(&[0x1b; 1 << 20]);
        let path = "p".repeat(4096);
        let error = Error::at(3, 9, &path, Kind::UnknownKey { key }).in_file(&"f".repeat(4096));
        let message = error.message();
        assert!(message.len() <= MESSAGE_BYTES, "{}", message.len());
        assert!(message.ends_with("..."));
        assert!(message.bytes().all(|byte| (0x20..=0x7e).contains(&byte)));
        // A short one is whole and says where.
        let short = Error::at(2, 5, "walk_speed_m_per_s", Kind::MissingKey { key: "id" })
            .in_file("movement/soldier.json");
        assert_eq!(
            short.message(),
            "the content is refused: the key \"id\" is missing, in movement/soldier.json, line 2 column 5, at walk_speed_m_per_s"
        );
    }

    #[test]
    fn a_record_is_one_line_with_doubles_as_hex_digits() {
        let error = Error::at(
            4,
            23,
            "gravity_m_per_s2",
            Kind::OutOfRange {
                bits: 150.0f64.to_bits(),
                min_bits: 0.0f64.to_bits(),
                max_bits: 100.0f64.to_bits(),
                min_inclusive: false,
                max_inclusive: true,
            },
        )
        .in_file("movement/soldier.json");
        assert_eq!(
            error.record(),
            "movement/soldier.json|4:23|gravity_m_per_s2|OutOfRange 4062c00000000000 ( 0000000000000000 4059000000000000 ]"
        );
        assert_eq!(Error::whole(Kind::Empty).record(), "-|0:0|-|Empty");
        assert_eq!(
            Error::whole(Kind::DuplicateKey {
                key: Echo::of(b"id")
            })
            .record(),
            "-|0:0|-|DuplicateKey { key: Echo(\"id\") }"
        );
        assert!(!error.record().contains('\n'));
    }
}
