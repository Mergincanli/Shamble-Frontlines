//! The typed read: a tree to the fields a kind of content has, each read by its name, checked against its range, and
//! named by its path in any refusal. A key no field took is refused when the object is finished, so a content file
//! holds exactly its kind's fields: nothing missing, nothing extra, no fallback.

use crate::error::{Echo, Error, Kind};
use crate::json::{Member, Node, Value, is_id};

/// The range a number must lie in, each end counted in or left out.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub min: f64,
    pub max: f64,
    pub min_inclusive: bool,
    pub max_inclusive: bool,
}

impl Range {
    /// From `min` to `max`, both counted in.
    pub const fn closed(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            min_inclusive: true,
            max_inclusive: true,
        }
    }

    /// Above `min`, up to `max` counted in.
    pub const fn above(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            min_inclusive: false,
            max_inclusive: true,
        }
    }

    pub fn holds(&self, value: f64) -> bool {
        let above = if self.min_inclusive {
            value >= self.min
        } else {
            value > self.min
        };
        let below = if self.max_inclusive {
            value <= self.max
        } else {
            value < self.max
        };
        above && below
    }
}

/// One number field of a kind: its key, its unit (for the tables people read) and its range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub unit: &'static str,
    pub range: Range,
}

/// An object being read field by field.
pub struct Object<'a> {
    members: &'a [Member],
    taken: Vec<bool>,
    path: String,
}

/// `outer.key`, or the key alone at the top.
fn join(outer: &str, key: &str) -> String {
    if outer.is_empty() {
        key.to_owned()
    } else {
        format!("{outer}.{key}")
    }
}

fn wrong_type(node: &Node, path: &str, expected: &'static str) -> Error {
    Error::at(
        node.line,
        node.col,
        path,
        Kind::WrongType {
            expected,
            found: node.value.kind_name(),
        },
    )
}

/// A number inside its range, or the refusal that names both.
pub fn number_in(node: &Node, path: &str, range: Range) -> Result<f64, Error> {
    let Value::Number(value) = node.value else {
        return Err(wrong_type(node, path, "a number"));
    };
    if range.holds(value) {
        return Ok(value);
    }
    let kind = Kind::OutOfRange {
        bits: value.to_bits(),
        min_bits: range.min.to_bits(),
        max_bits: range.max.to_bits(),
        min_inclusive: range.min_inclusive,
        max_inclusive: range.max_inclusive,
    };
    Err(Error::at(node.line, node.col, path, kind))
}

/// A whole number from `min` to `max`: 1.5, -1 and anything above `max` are refused, never cut to fit.
pub fn whole_in(node: &Node, path: &str, min: u32, max: u32) -> Result<u32, Error> {
    let Value::Number(value) = node.value else {
        return Err(wrong_type(node, path, "a whole number"));
    };
    // Every `u32` is a double exactly, so the comparison is exact, and the cast below then loses nothing.
    if value.fract() == 0.0 && value >= f64::from(min) && value <= f64::from(max) {
        return Ok(value as u32);
    }
    Err(Error::at(
        node.line,
        node.col,
        path,
        Kind::NotAWholeNumber {
            bits: value.to_bits(),
        },
    ))
}

impl<'a> Object<'a> {
    /// `node` as an object at `path` (empty for a document's top).
    pub fn of(node: &'a Node, path: &str) -> Result<Self, Error> {
        let Value::Object(members) = &node.value else {
            return Err(wrong_type(node, path, "an object"));
        };
        Ok(Self {
            members,
            taken: vec![false; members.len()],
            path: path.to_owned(),
        })
    }

    /// The member named `key`, marked as read. The reader refused a key that is there twice, so there is one.
    fn take(&mut self, key: &'static str) -> Result<(&'a Node, String), Error> {
        let at = self.members.iter().position(|member| member.key == key);
        let found = at.and_then(|at| Some((self.members.get(at)?, self.taken.get_mut(at)?)));
        let Some((member, taken)) = found else {
            return Err(Error::at(0, 0, &self.path, Kind::MissingKey { key }));
        };
        *taken = true;
        Ok((&member.node, join(&self.path, key)))
    }

    pub fn number(&mut self, field: &Field) -> Result<f64, Error> {
        let (node, path) = self.take(field.name)?;
        number_in(node, &path, field.range)
    }

    pub fn whole(&mut self, key: &'static str, min: u32, max: u32) -> Result<u32, Error> {
        let (node, path) = self.take(key)?;
        whole_in(node, &path, min, max)
    }

    pub fn text(&mut self, key: &'static str) -> Result<&'a str, Error> {
        let (node, path) = self.take(key)?;
        match &node.value {
            Value::Text(text) => Ok(text),
            _ => Err(wrong_type(node, &path, "a string")),
        }
    }

    /// A string that is an id.
    pub fn id(&mut self, key: &'static str) -> Result<&'a str, Error> {
        let (node, path) = self.take(key)?;
        match &node.value {
            Value::Text(text) if is_id(text) => Ok(text),
            Value::Text(text) => Err(Error::at(
                node.line,
                node.col,
                &path,
                Kind::BadId {
                    id: Echo::of(text.as_bytes()),
                },
            )),
            _ => Err(wrong_type(node, &path, "a string")),
        }
    }

    /// A string that is one of `allowed`: its place in that list. `listed` is the list as a message shows it.
    pub fn one_of(
        &mut self,
        key: &'static str,
        allowed: &[&str],
        listed: &'static str,
    ) -> Result<usize, Error> {
        let (node, path) = self.take(key)?;
        let Value::Text(text) = &node.value else {
            return Err(wrong_type(node, &path, "a string"));
        };
        allowed
            .iter()
            .position(|allowed| allowed == text)
            .ok_or_else(|| {
                let kind = Kind::NotOneOf {
                    value: Echo::of(text.as_bytes()),
                    allowed: listed,
                };
                Error::at(node.line, node.col, &path, kind)
            })
    }

    /// An array of at most `max` entries, with its path.
    pub fn array(&mut self, key: &'static str, max: u32) -> Result<(&'a [Node], String), Error> {
        let (node, path) = self.take(key)?;
        let Value::Array(items) = &node.value else {
            return Err(wrong_type(node, &path, "an array"));
        };
        let len = u32::try_from(items.len()).unwrap_or(u32::MAX);
        if len > max {
            return Err(Error::at(
                node.line,
                node.col,
                &path,
                Kind::TooMany { len, max },
            ));
        }
        Ok((items, path))
    }

    /// An array of exactly `N` numbers, each inside `range`: a corner, a place.
    pub fn numbers<const N: usize>(
        &mut self,
        key: &'static str,
        range: Range,
    ) -> Result<[f64; N], Error> {
        let (node, path) = self.take(key)?;
        let Value::Array(items) = &node.value else {
            return Err(wrong_type(node, &path, "an array"));
        };
        let count = |count: usize| u32::try_from(count).unwrap_or(u32::MAX);
        let (len, exactly) = (count(items.len()), count(N));
        if len != exactly {
            let kind = if len > exactly {
                Kind::TooMany { len, max: exactly }
            } else {
                Kind::TooFew { len, min: exactly }
            };
            return Err(Error::at(node.line, node.col, &path, kind));
        }
        let mut numbers = [0.0; N];
        for (at, (number, item)) in numbers.iter_mut().zip(items).enumerate() {
            *number = number_in(item, &format!("{path}[{at}]"), range)?;
        }
        Ok(numbers)
    }

    pub fn object(&mut self, key: &'static str) -> Result<Object<'a>, Error> {
        let (node, path) = self.take(key)?;
        Object::of(node, &path)
    }

    /// Every file starts the same way: its kind's `format`, its `version`, and its `id`. The typed read takes the
    /// kind's newest version only: a file of an older version is migrated to the newest before it is read (ADR 0005's
    /// CD-4), so a version under the newest is refused here as a version, never read as the newest.
    pub fn header(&mut self, format: &'static str, newest: u32) -> Result<&'a str, Error> {
        let (node, path) = self.take("format")?;
        match &node.value {
            Value::Text(found) if found == format => {}
            Value::Text(found) => {
                let kind = Kind::WrongFormat {
                    found: Echo::of(found.as_bytes()),
                    expected: format,
                };
                return Err(Error::at(node.line, node.col, &path, kind));
            }
            _ => return Err(wrong_type(node, &path, "a string")),
        }
        let (node, path) = self.take("version")?;
        whole_in(node, &path, newest, newest).map_err(|error| match error.kind {
            // A version is refused as a version: the file's own number, and the newest this build reads.
            Kind::NotAWholeNumber { bits } => Error::at(
                error.line,
                error.col,
                &error.path,
                Kind::WrongVersion { bits, newest },
            ),
            _ => error,
        })?;
        self.id("id")
    }

    /// Ends the read: a key that no field took is refused, the first one in the text's order.
    pub fn finish(self) -> Result<(), Error> {
        let unread = self
            .members
            .iter()
            .zip(&self.taken)
            .find(|(_, taken)| !**taken);
        match unread {
            None => Ok(()),
            Some((member, _)) => {
                let kind = Kind::UnknownKey {
                    key: Echo::of(member.key.as_bytes()),
                };
                Err(Error::at(member.line, member.col, &self.path, kind))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::parse;

    const SPEED: Field = Field {
        name: "speed",
        unit: "m/s",
        range: Range::above(0.0, 20.0),
    };

    #[test]
    fn fields_are_read_by_name_and_an_unread_key_is_refused() {
        let top = parse(
            b"{\"format\": \"shamble-test\", \"id\": \"soldier\", \"speed\": 6.3, \"version\": 1}",
        )
        .expect("valid");
        let mut object = Object::of(&top, "").expect("an object");
        assert_eq!(object.header("shamble-test", 1), Ok("soldier"));
        assert_eq!(object.number(&SPEED), Ok(6.3));
        assert_eq!(object.finish(), Ok(()));
        // The same file, one field never asked for: refused by its name, at its place.
        let mut object = Object::of(&top, "").expect("an object");
        assert_eq!(object.header("shamble-test", 1), Ok("soldier"));
        let error = object.finish().expect_err("speed was not read");
        assert_eq!(
            (error.kind, error.col),
            (
                Kind::UnknownKey {
                    key: Echo::of(b"speed")
                },
                45
            )
        );
        // A field that is not there.
        let mut object = Object::of(&top, "movement").expect("an object");
        let error = object.text("missing").expect_err("no such key");
        assert_eq!(
            (error.kind, error.path.as_str()),
            (Kind::MissingKey { key: "missing" }, "movement")
        );
    }

    #[test]
    fn a_range_is_held_at_its_ends_to_the_last_bit() {
        let read = |text: &str| {
            let top = parse(format!("{{\"speed\": {text}}}").as_bytes()).expect(text);
            Object::of(&top, "").expect("an object").number(&SPEED)
        };
        // Above 0, up to 20 counted in: the doubles next to each end, on both sides.
        assert_eq!(read("20"), Ok(20.0));
        assert_eq!(read("5e-324"), Ok(f64::from_bits(1)));
        assert!(read("19.999999999999996").is_ok());
        for outside in ["0", "20.000000000000004", "-1", "1e+300"] {
            let error = read(outside).expect_err(outside);
            assert!(
                matches!(
                    error.kind,
                    Kind::OutOfRange {
                        min_inclusive: false,
                        max_inclusive: true,
                        ..
                    }
                ),
                "{outside}"
            );
            assert_eq!(error.path, "speed");
        }
        assert!(Range::closed(0.0, 1.0).holds(0.0) && Range::closed(0.0, 1.0).holds(1.0));
        assert!(!Range::closed(0.0, 1.0).holds(f64::NAN));
        assert_eq!(
            read("\"fast\"").expect_err("a string").kind,
            Kind::WrongType {
                expected: "a number",
                found: "a string"
            }
        );
    }

    #[test]
    fn a_whole_number_is_never_cut_to_fit() {
        let read = |text: &str| {
            let top = parse(format!("{{\"count\": {text}}}").as_bytes()).expect(text);
            Object::of(&top, "")
                .expect("an object")
                .whole("count", 0, u32::MAX)
        };
        assert_eq!(read("0"), Ok(0));
        assert_eq!(read("4294967295"), Ok(u32::MAX));
        for refused in ["1.5", "-1", "4294967296", "1e+300", "0.5"] {
            assert!(
                matches!(
                    read(refused).expect_err(refused).kind,
                    Kind::NotAWholeNumber { .. }
                ),
                "{refused}"
            );
        }
    }

    #[test]
    fn the_header_refuses_another_format_another_version_and_a_bad_id() {
        let header = |text: &str| {
            let top = parse(text.as_bytes()).expect(text);
            Object::of(&top, "")
                .expect("an object")
                .header("shamble-test", 1)
                .map(str::to_owned)
                .map_err(|error| error.kind)
        };
        assert_eq!(
            header("{\"format\": \"shamble-test\", \"id\": \"a\", \"version\": 1}"),
            Ok("a".to_owned())
        );
        assert_eq!(
            header("{\"format\": \"shamble-content-bundle\", \"id\": \"a\", \"version\": 1}"),
            Err(Kind::WrongFormat {
                found: Echo::of(b"shamble-content-bundle"),
                expected: "shamble-test"
            })
        );
        for version in ["2", "0", "1.5", "-1"] {
            let text =
                format!("{{\"format\": \"shamble-test\", \"id\": \"a\", \"version\": {version}}}");
            assert!(
                matches!(header(&text), Err(Kind::WrongVersion { newest: 1, .. })),
                "{version}"
            );
        }
        for id in ["A", "a b", "<img src=x>", "a\\nb", ""] {
            let text =
                format!("{{\"format\": \"shamble-test\", \"id\": \"{id}\", \"version\": 1}}");
            assert!(matches!(header(&text), Err(Kind::BadId { .. })), "{id}");
        }
        assert_eq!(
            header("{\"id\": \"a\", \"version\": 1}"),
            Err(Kind::MissingKey { key: "format" })
        );
    }

    /// The typed read takes a kind's newest version only (the advisor agent's VERSION-RULE ruling, 2026-11-28): an
    /// older version is refused as a version, as a newer one is, and is never read as the newest.
    #[test]
    fn the_header_takes_the_newest_version_only() {
        let header = |version: &str| {
            let text =
                format!("{{\"format\": \"shamble-test\", \"id\": \"a\", \"version\": {version}}}");
            let top = parse(text.as_bytes()).expect("valid text");
            Object::of(&top, "")
                .expect("an object")
                .header("shamble-test", 2)
                .map(str::to_owned)
                .map_err(|error| error.kind)
        };
        assert_eq!(header("2"), Ok("a".to_owned()));
        for version in ["1", "3", "0", "1.5", "-1"] {
            assert!(
                matches!(header(version), Err(Kind::WrongVersion { newest: 2, .. })),
                "{version}"
            );
        }
    }

    #[test]
    fn lists_and_fixed_words() {
        let top = parse(b"{\"kind\": \"grass\", \"items\": [1, 2, 3]}").expect("valid");
        let mut object = Object::of(&top, "regions[0]").expect("an object");
        assert_eq!(
            object.one_of("kind", &["street", "grass"], "street, grass"),
            Ok(1)
        );
        let (items, path) = object.array("items", 3).expect("three fit");
        assert_eq!((items.len(), path.as_str()), (3, "regions[0].items"));
        let mut object = Object::of(&top, "").expect("an object");
        assert_eq!(
            object
                .one_of("kind", &["box", "ramp"], "box, ramp")
                .expect_err("not listed")
                .kind,
            Kind::NotOneOf {
                value: Echo::of(b"grass"),
                allowed: "box, ramp"
            }
        );
        assert_eq!(
            object.array("items", 2).expect_err("one too many").kind,
            Kind::TooMany { len: 3, max: 2 }
        );
    }
}
