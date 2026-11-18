//! The layout table of the output block and its hash (ADR 0002 part 5). The block is the bytes the simulation writes
//! for the page each tick: a header, then sections of fixed-size rows. This table is its one description: the block
//! writer takes its offsets from it, a test prints it as TypeScript into `packages/types` (`layout_ts`), and the
//! page refuses a block whose header carries another `LAYOUT_HASH`.

/// The element type of a field. A variant enters with its first use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    /// One byte, unsigned.
    U8,
    /// Four bytes, unsigned, little-endian.
    U32,
}

impl ElementType {
    /// The name in the generated TypeScript and in the hash's byte stream.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U32 => "u32",
        }
    }

    /// The size in bytes. A field's offset is a multiple of it.
    #[must_use]
    pub const fn size(self) -> u32 {
        match self {
            Self::U8 => 1,
            Self::U32 => 4,
        }
    }
}

/// One value of a row: its name, its type and its offset in bytes from the row's start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    /// The field's name, unique in its section: lower-case letters, digits and `_`, a letter first.
    pub name: &'static str,
    /// Its element type.
    pub ty: ElementType,
    /// Its offset in bytes from the row's start, a multiple of its type's size.
    pub offset: u32,
}

/// One section of the block: rows of `stride` bytes, each holding `fields`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Section {
    /// The section's name, unique in the table, formed like a field's name.
    pub name: &'static str,
    /// The size of one row in bytes, a multiple of the section's largest field.
    pub stride: u32,
    /// The row's fields; none overlaps another.
    pub fields: &'static [Field],
}

/// The block's sections, in the order they are written. The header is first and has one row; `tick` at 0 and
/// `layout_hash` at 4 never move, because the page reads the hash before it trusts any other offset. Each entity
/// section adds its row count to the header when it enters.
#[rustfmt::skip]
pub const BLOCK_LAYOUT: &[Section] = &[
    Section { name: "header", stride: 8, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
    ] },
];

/// The hash of `BLOCK_LAYOUT`, written into every block's header and printed into the generated TypeScript.
pub const LAYOUT_HASH: u32 = layout_hash(BLOCK_LAYOUT);

// A block of zero bytes must never pass the page's check.
const _: () = assert!(LAYOUT_HASH != 0);

const FNV_OFFSET_BASIS: u32 = 0x811C_9DC5;
const FNV_PRIME: u32 = 0x0100_0193;
/// The byte in front of each section's bytes in the hashed stream. With it and `FIELD_TAG` the stream reads one way
/// only, so two tables that keep the layout rules (no 0x00 in a name) never give one stream.
const SECTION_TAG: u8 = 0x02;
/// The byte in front of each field's bytes in the hashed stream.
const FIELD_TAG: u8 = 0x01;

/// FNV-1a, 32 bits (RFC 9923), over one byte stream that states every value of the table. Per section: the section
/// tag, its name, 0x00, its stride; per field: the field tag, its name, 0x00, its type's name, 0x00, its offset.
/// Numbers are `u32`, little-endian. It detects drift between the writer and the page; it is not a security check.
#[must_use]
pub const fn layout_hash(sections: &[Section]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;
    let mut s = 0;
    while s < sections.len() {
        // Every member is named in these two patterns: one added to the table later does not compile until it is
        // hashed here, or left out here in plain sight.
        let Section {
            name,
            stride,
            fields,
        } = sections[s];
        hash = fnv1a(hash, &[SECTION_TAG]);
        hash = fnv1a(hash, name.as_bytes());
        hash = fnv1a(hash, &[0]);
        hash = fnv1a(hash, &stride.to_le_bytes());
        let mut f = 0;
        while f < fields.len() {
            let Field { name, ty, offset } = fields[f];
            hash = fnv1a(hash, &[FIELD_TAG]);
            hash = fnv1a(hash, name.as_bytes());
            hash = fnv1a(hash, &[0]);
            hash = fnv1a(hash, ty.name().as_bytes());
            hash = fnv1a(hash, &[0]);
            hash = fnv1a(hash, &offset.to_le_bytes());
            f += 1;
        }
        s += 1;
    }
    hash
}

const fn fnv1a(mut hash: u32, bytes: &[u8]) -> u32 {
    let mut i = 0;
    while i < bytes.len() {
        hash = (hash ^ bytes[i] as u32).wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::ElementType::{U8, U32};
    use super::*;

    const fn field(name: &'static str, ty: ElementType, offset: u32) -> Field {
        Field { name, ty, offset }
    }

    const fn section(name: &'static str, stride: u32, fields: &'static [Field]) -> Section {
        Section {
            name,
            stride,
            fields,
        }
    }

    fn name_is_plain(name: &str) -> bool {
        let mut chars = name.chars();
        chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    /// The rules every table keeps, or the first one broken. Names are plain (`[a-z][a-z0-9_]*`: no 0x00 for the
    /// hash's stream, and printable as TypeScript property names) and unique; a field sits at a multiple of its size,
    /// inside the row, on no other field; a row's size is a multiple of its largest field, so typed-array views bind.
    fn broken_rule(sections: &[Section]) -> Option<String> {
        for (i, section) in sections.iter().enumerate() {
            let name = section.name;
            if !name_is_plain(name) || sections[..i].iter().any(|other| other.name == name) {
                return Some(format!("section name {name:?} is not plain or not unique"));
            }
            let (stride, sizes) = (section.stride, section.fields.iter().map(|f| f.ty.size()));
            let align = sizes.max().unwrap_or(1);
            if stride == 0 || stride % align != 0 {
                return Some(format!("{name}: stride {stride} is no multiple of {align}"));
            }
            for (j, field) in section.fields.iter().enumerate() {
                let (own, at, size) = (field.name, field.offset, field.ty.size());
                let earlier = &section.fields[..j];
                if !name_is_plain(own) || earlier.iter().any(|other| other.name == own) {
                    return Some(format!(
                        "{name}: field name {own:?} is not plain or not unique"
                    ));
                }
                // An earlier field's end cannot overflow: it passed this check in its own turn.
                let Some(end) = at.checked_add(size).filter(|end| *end <= stride) else {
                    return Some(format!("{name}.{own}: it ends outside the row"));
                };
                if at % size != 0 {
                    return Some(format!(
                        "{name}.{own}: offset {at} is no multiple of {size}"
                    ));
                }
                let ends = |other: &Field| other.offset + other.ty.size();
                if earlier
                    .iter()
                    .any(|other| at < ends(other) && other.offset < end)
                {
                    return Some(format!("{name}.{own}: it overlaps another field"));
                }
            }
        }
        None
    }

    /// One good table with both element types.
    #[rustfmt::skip]
    const GOOD: &[Section] = &[section("row", 8, &[field("a", U32, 4), field("b", U8, 0), field("c", U8, 3)])];
    /// One table per rule, each breaking only that rule.
    #[rustfmt::skip]
    const BAD: &[(&str, &[Section])] = &[
        ("a name with a capital",         &[section("Row", 4, &[])]),
        ("an empty name",                 &[section("row", 4, &[field("", U8, 0)])]),
        ("a section name used twice",     &[section("row", 4, &[]), section("row", 4, &[])]),
        ("a field name used twice",       &[section("row", 4, &[field("a", U8, 0), field("a", U8, 1)])]),
        ("a stride of 0",                 &[section("row", 0, &[])]),
        ("a stride off its alignment",    &[section("row", 6, &[field("a", U32, 0)])]),
        ("an unaligned field",            &[section("row", 8, &[field("a", U32, 2)])]),
        ("a field past the row's end",    &[section("row", 4, &[field("a", U8, 4)])]),
        ("an offset past the u32 range",  &[section("row", 8, &[field("a", U32, 0xFFFF_FFFC)])]),
        ("a narrow field on a wide one",  &[section("row", 8, &[field("a", U32, 0), field("b", U8, 3)])]),
        ("a wide field on a narrow one",  &[section("row", 8, &[field("a", U8, 1), field("b", U32, 0)])]),
    ];

    #[test]
    fn layout_rules_hold() {
        assert_eq!(broken_rule(BLOCK_LAYOUT), None);
        let header = BLOCK_LAYOUT
            .first()
            .expect("the block starts with its header");
        assert_eq!(header.name, "header");
        assert_eq!(header.fields.first(), Some(&field("tick", U32, 0)));
        assert_eq!(header.fields.get(1), Some(&field("layout_hash", U32, 4)));

        // The check itself bites.
        assert_eq!(broken_rule(GOOD), None);
        for (what, table) in BAD {
            assert!(broken_rule(table).is_some(), "the rules must refuse {what}");
        }
    }

    const FIRST: Section = section("first", 8, &[field("a", U32, 0), field("b", U8, 4)]);
    const SECOND: Section = section("second", 8, &[field("c", U32, 0)]);
    const BASE: &[Section] = &[FIRST, SECOND];
    /// `BASE` with one thing changed in each table; each is still a table the rules accept.
    #[rustfmt::skip]
    const CHANGED: &[(&str, &[Section])] = &[
        ("section name", &[section("firsu", 8, &[field("a", U32, 0), field("b", U8, 4)]), SECOND]),
        ("stride",       &[section("first", 12, &[field("a", U32, 0), field("b", U8, 4)]), SECOND]),
        ("field name",   &[section("first", 8, &[field("a", U32, 0), field("d", U8, 4)]), SECOND]),
        ("field type",   &[section("first", 8, &[field("a", U32, 0), field("b", U32, 4)]), SECOND]),
        ("field offset", &[section("first", 8, &[field("a", U32, 0), field("b", U8, 5)]), SECOND]),
        ("field order",  &[section("first", 8, &[field("b", U8, 4), field("a", U32, 0)]), SECOND]),
        ("field count",  &[section("first", 8, &[field("a", U32, 0), field("b", U8, 4), field("e", U8, 5)]), SECOND]),
        ("field's section", &[
            section("first", 8, &[field("a", U32, 0)]),
            section("second", 8, &[field("b", U8, 4), field("c", U32, 0)]),
        ]),
        ("field in the second section", &[FIRST, section("second", 8, &[field("c", U8, 0)])]),
        ("section order",               &[SECOND, FIRST]),
        ("section count, one more",     &[FIRST, SECOND, section("third", 4, &[])]),
        ("section count, one less",     &[FIRST]),
    ];
    /// Two tables that keep the rules and would give one byte stream without the tag bytes.
    #[rustfmt::skip]
    const TAG_PAIR: [&[Section]; 2] = [
        &[section("a", 0xFFFF_FFFF, &[field("b", U8, 0x7A79_7800), field("c", U8, 0)])],
        &[section("a", 0xFFFF_FFFF, &[]), section("b", 0x0000_3875, &[field("xyzc", U8, 0)])],
    ];

    #[test]
    fn hash_covers_every_value() {
        assert_eq!(broken_rule(BASE), None);
        let mut seen = vec![layout_hash(BASE)];
        for (what, table) in CHANGED {
            assert_eq!(broken_rule(table), None, "the table with a changed {what}");
            let hash = layout_hash(table);
            assert!(
                !seen.contains(&hash),
                "a changed {what} must change the hash"
            );
            seen.push(hash);
        }

        // Known answers, from a second implementation of the byte stream this file's doc describes (a script, not
        // this code): they hold the function itself still, whatever the block's own table becomes.
        assert_eq!(layout_hash(BASE), 0x9adb_ecc7);
        assert_eq!(layout_hash(&[]), FNV_OFFSET_BASIS);
        for table in TAG_PAIR {
            assert_eq!(broken_rule(table), None);
        }
        assert_eq!(layout_hash(TAG_PAIR[0]), 0x4265_359c);
        assert_eq!(layout_hash(TAG_PAIR[1]), 0xbd51_cd2f);
    }

    /// The test values of RFC 9923 for FNV-1a, 32 bits.
    #[test]
    fn fnv1a_matches_rfc_9923() {
        assert_eq!(fnv1a(FNV_OFFSET_BASIS, b""), 0x811c_9dc5);
        assert_eq!(fnv1a(FNV_OFFSET_BASIS, b"a"), 0xe40c_292c);
        assert_eq!(fnv1a(FNV_OFFSET_BASIS, b"foobar"), 0xbf9c_f968);
    }
}
