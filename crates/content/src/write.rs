//! The one canonical writer (ADR 0005 CD-1): a tree to the one text a content file may have.
//!
//! The text is `JSON.stringify(value, null, 2)` of the value with its keys sorted, and a final line feed: keys in
//! byte order at every level, arrays in their order, two spaces of indent, one entry a line, `{}` and `[]` for the
//! empty ones, numbers as `number_text` writes them. A browser makes the same bytes with its own functions, which is
//! how a script writes a content file that this crate then judges.
//!
//! The loader refuses a document whose bytes are not this text ([`first_difference`]), so every content file has
//! exactly one form, and the tool that checks files and the loader that reads them cannot disagree.

use crate::json::{Member, Node, Value};
use crate::number::number_text;

/// Where the writer's text goes: a string, or a comparison with a text that is already there.
pub trait Sink {
    fn put(&mut self, text: &str);
}

impl Sink for String {
    fn put(&mut self, text: &str) {
        self.push_str(text);
    }
}

/// The canonical text of a document.
pub fn canonical(top: &Node) -> String {
    let mut text = String::new();
    canonical_into(top, &mut text);
    text
}

/// Writes the canonical text of a document into `sink`.
pub fn canonical_into(top: &Node, sink: &mut impl Sink) {
    value(&top.value, 0, sink);
    sink.put("\n");
}

fn indent(level: usize, sink: &mut impl Sink) {
    for _ in 0..level {
        sink.put("  ");
    }
}

fn value(value_: &Value, level: usize, sink: &mut impl Sink) {
    let inner = level.saturating_add(1);
    match value_ {
        Value::Number(number) => sink.put(&number_text(*number)),
        Value::Text(text) => string(text, sink),
        Value::Array(items) if items.is_empty() => sink.put("[]"),
        Value::Array(items) => {
            sink.put("[\n");
            for (at, item) in items.iter().enumerate() {
                if at != 0 {
                    sink.put(",\n");
                }
                indent(inner, sink);
                value(&item.value, inner, sink);
            }
            sink.put("\n");
            indent(level, sink);
            sink.put("]");
        }
        Value::Object(members) if members.is_empty() => sink.put("{}"),
        Value::Object(members) => {
            // Keys in byte order, whatever order the tree has them in.
            let mut sorted: Vec<&Member> = members.iter().collect();
            sorted.sort_by(|a, b| a.key.as_bytes().cmp(b.key.as_bytes()));
            sink.put("{\n");
            for (at, member) in sorted.iter().enumerate() {
                if at != 0 {
                    sink.put(",\n");
                }
                indent(inner, sink);
                string(&member.key, sink);
                sink.put(": ");
                value(&member.node.value, inner, sink);
            }
            sink.put("\n");
            indent(level, sink);
            sink.put("}");
        }
    }
}

/// A string between quotes, as `JSON.stringify` writes it: a quote, a line feed and a backslash escaped. A string the
/// reader gave holds printable ASCII, quotes and line feeds, never a backslash (it takes no escape that could make
/// one); a text handed to the bundle's writer may hold one, and the reader then refuses that bundle by name.
fn string(text: &str, sink: &mut impl Sink) {
    sink.put("\"");
    let mut rest = text;
    while let Some(at) = rest.find(['"', '\n', '\\']) {
        let (plain, tail) = rest.split_at_checked(at).unwrap_or((rest, ""));
        sink.put(plain);
        sink.put(match tail.as_bytes().first() {
            Some(b'"') => "\\\"",
            Some(b'\n') => "\\n",
            _ => "\\\\",
        });
        rest = tail.get(1..).unwrap_or("");
    }
    sink.put(rest);
    sink.put("\"");
}

/// The most bytes of each text a difference shows.
const SHOWN: usize = 24;

/// Where a text stops being the canonical text of its own tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// The first byte that differs, counted from 0; the text's length when the text is only too short.
    pub offset: usize,
    /// What the canonical text has from there, and what the text has: a few bytes each.
    pub expected: Vec<u8>,
    pub found: Vec<u8>,
}

struct Compare<'a> {
    source: &'a [u8],
    at: usize,
    difference: Option<Difference>,
}

impl Sink for Compare<'_> {
    fn put(&mut self, text: &str) {
        for &byte in text.as_bytes() {
            match &mut self.difference {
                // After the first difference only the canonical side is still collected, to be shown.
                Some(difference) if difference.expected.len() < SHOWN => {
                    difference.expected.push(byte)
                }
                Some(_) => return,
                None if self.source.get(self.at) == Some(&byte) => {
                    self.at = self.at.saturating_add(1)
                }
                None => {
                    let found = self.source.get(self.at..).unwrap_or(&[]);
                    let found = found.get(..SHOWN).unwrap_or(found).to_vec();
                    self.difference = Some(Difference {
                        offset: self.at,
                        expected: vec![byte],
                        found,
                    });
                }
            }
        }
    }
}

/// The first place where `source` is not the canonical text of `top` (the tree read from it), or `None` when it is
/// that text byte for byte. It streams: the canonical text is never held whole.
pub fn first_difference(top: &Node, source: &[u8]) -> Option<Difference> {
    let mut compare = Compare {
        source,
        at: 0,
        difference: None,
    };
    canonical_into(top, &mut compare);
    if compare.difference.is_none() && compare.at != source.len() {
        // Everything the writer wrote is there, and then more.
        let found = source.get(compare.at..).unwrap_or(&[]);
        let found = found.get(..SHOWN).unwrap_or(found).to_vec();
        compare.difference = Some(Difference {
            offset: compare.at,
            expected: Vec::new(),
            found,
        });
    }
    compare.difference
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::parse;

    #[test]
    fn the_text_is_json_stringify_with_sorted_keys_two_spaces_and_a_final_line_feed() {
        // What `JSON.stringify(sortedKeys(value), null, 2) + "\n"` gives for this value in a browser.
        let top = parse(
            b"{\"b\": [1, {\"z\": [], \"a\": {}}, \"q\\\"r\\ns\"], \"a\": -0.5, \"c\": [[2]]}",
        )
        .expect("valid");
        assert_eq!(
            canonical(&top),
            "{\n  \"a\": -0.5,\n  \"b\": [\n    1,\n    {\n      \"a\": {},\n      \"z\": []\n    },\n    \"q\\\"r\\ns\"\n  ],\n  \"c\": [\n    [\n      2\n    ]\n  ]\n}\n"
        );
        assert_eq!(canonical(&parse(b"{}").expect("valid")), "{}\n");
    }

    #[test]
    fn writing_then_reading_then_writing_gives_the_same_bytes() {
        let text = "{\n  \"a\": [\n    1e-7,\n    1e+21,\n    0.1\n  ],\n  \"b\": \"x\"\n}\n";
        let top = parse(text.as_bytes()).expect("valid");
        assert_eq!(canonical(&top), text);
        assert_eq!(first_difference(&top, text.as_bytes()), None);
        assert_eq!(parse(canonical(&top).as_bytes()).expect("valid"), top);
    }

    #[test]
    fn a_text_that_is_not_canonical_is_found_at_its_first_differing_byte() {
        let difference = |text: &str| {
            first_difference(&parse(text.as_bytes()).expect(text), text.as_bytes()).expect(text)
        };
        // Keys out of order: the canonical text has "a" where the text has "b".
        let found = difference("{\n  \"b\": 1,\n  \"a\": 2\n}\n");
        assert_eq!(
            (found.offset, found.expected.first(), found.found.first()),
            (5, Some(&b'a'), Some(&b'b'))
        );
        // One line, no final line feed, four spaces, a space before the colon, a blank line at the end.
        assert_eq!(difference("{\"a\": 1}\n").offset, 1);
        assert_eq!(difference("{\n  \"a\": 1\n}").offset, 12);
        assert_eq!(difference("{\n    \"a\": 1\n}\n").offset, 4);
        assert_eq!(difference("{\n  \"a\" : 1\n}\n").offset, 7);
        let extra = difference("{\n  \"a\": 1\n}\n\n");
        assert_eq!(
            (extra.offset, extra.expected.len(), extra.found.as_slice()),
            (13, 0, b"\n".as_slice())
        );
        assert_eq!(difference("{ }\n").offset, 1);
        assert_eq!(difference("{\n  \"a\": [ ]\n}\n").offset, 10);
        // What is shown of either side is a few bytes, however long the rest is.
        let long = format!("{{\"a\": \"{}\"}}\n", "x".repeat(10_000));
        let found = difference(&long);
        assert!(found.expected.len() <= SHOWN && found.found.len() <= SHOWN);
    }
}
