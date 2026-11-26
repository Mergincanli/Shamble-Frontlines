//! Numbers: how one is read from text and how one is written.
//!
//! Reading takes the whole number grammar of RFC 8259 and gives the closest double, as a browser's `JSON.parse`
//! does; the content crate's proof compares the two bit for bit (PM-0005-2). Writing gives the one text ECMAScript's
//! `Number::toString` gives, so `JSON.stringify` and this writer agree on every finite number. The loader itself is
//! stricter than this reader: it takes a number only when its text is already that one text (`json.rs`).

/// Why a text is not a number this crate reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberError {
    /// Not the number grammar of RFC 8259 (`-? int frac? exp?`): a leading zero, a lone point, `+1`, `NaN`.
    Grammar,
    /// A valid text whose value no double holds: it would read as infinity.
    NotFinite,
}

/// Whether `text` is RFC 8259's number grammar, whole.
fn is_number_grammar(text: &[u8]) -> bool {
    let mut rest = text;
    if let [b'-', tail @ ..] = rest {
        rest = tail;
    }
    // The whole part: a lone 0, or a digit from 1 to 9 and then any digits.
    rest = match rest {
        [b'0', tail @ ..] => tail,
        [b'1'..=b'9', tail @ ..] => digits(tail).1,
        _ => return false,
    };
    if let [b'.', tail @ ..] = rest {
        let (count, tail) = digits(tail);
        if count == 0 {
            return false;
        }
        rest = tail;
    }
    if let [b'e' | b'E', tail @ ..] = rest {
        let tail = match tail {
            [b'+' | b'-', tail @ ..] => tail,
            tail => tail,
        };
        let (count, tail) = digits(tail);
        if count == 0 {
            return false;
        }
        rest = tail;
    }
    rest.is_empty()
}

/// How many digits `text` starts with, and what follows them.
fn digits(text: &[u8]) -> (usize, &[u8]) {
    let count = text.iter().take_while(|byte| byte.is_ascii_digit()).count();
    (count, text.get(count..).unwrap_or(&[]))
}

/// The double a number's text names: the closest one, ties to even, as `JSON.parse` gives it.
pub fn parse_number(text: &[u8]) -> Result<f64, NumberError> {
    if !is_number_grammar(text) {
        return Err(NumberError::Grammar);
    }
    // The grammar holds ASCII only, and every text of it is one the standard library's reader takes; that reader
    // also takes texts outside it ("inf", "nan", ".5", "5."), which is why the grammar is checked first.
    let value: f64 = core::str::from_utf8(text)
        .ok()
        .and_then(|text| text.parse().ok())
        .ok_or(NumberError::Grammar)?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(NumberError::NotFinite)
    }
}

/// The text of a finite number: the shortest digits that read back as the same double, laid out as ECMAScript's
/// `Number::toString` lays them out (plain digits from 1e-6 up to 1e21, an exponent beyond). Both zeros are "0", as
/// `JSON.stringify` writes them; a number that is not finite has no text and gives "null", which no reader of this
/// crate takes.
pub fn number_text(value: f64) -> String {
    if !value.is_finite() {
        return "null".to_owned();
    }
    if value == 0.0 {
        return "0".to_owned();
    }
    let sign = if value < 0.0 { "-" } else { "" };
    // The standard library's exponent form holds the shortest digits that read back as this double: "d.ddde-7".
    let scientific = format!("{:e}", value.abs());
    let Some((mantissa, exponent)) = scientific.split_once('e') else {
        return scientific;
    };
    let Ok(exponent) = exponent.parse::<i32>() else {
        return scientific;
    };
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let count = i32::try_from(digits.len()).unwrap_or(i32::MAX);
    // ECMAScript's n: the value is 0.<digits> times ten to the n.
    let point = exponent.saturating_add(1);
    let zeros = |count: i32| "0".repeat(usize::try_from(count).unwrap_or(0));
    if (count..=21).contains(&point) {
        // A whole number: the digits, then zeros.
        format!("{sign}{digits}{}", zeros(point.saturating_sub(count)))
    } else if (1..=21).contains(&point) {
        // The point falls inside the digits.
        let (whole, fraction) = digits
            .split_at_checked(usize::try_from(point).unwrap_or(0))
            .unwrap_or((&digits, ""));
        format!("{sign}{whole}.{fraction}")
    } else if (-5..=0).contains(&point) {
        // A small fraction: "0.", zeros, the digits.
        format!("{sign}0.{}{digits}", zeros(point.saturating_neg()))
    } else {
        // An exponent: one digit, the rest behind a point, "e+" or "e-".
        let power = point.saturating_sub(1);
        let (first, rest) = digits.split_at_checked(1).unwrap_or((&digits, ""));
        let point = if rest.is_empty() { "" } else { "." };
        let direction = if power < 0 { '-' } else { '+' };
        format!(
            "{sign}{first}{point}{rest}e{direction}{}",
            power.unsigned_abs()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammar_is_rfc_8259s_and_nothing_else() {
        for text in [
            "0",
            "-0",
            "1",
            "12",
            "-7",
            "0.5",
            "1.50",
            "1e5",
            "1E5",
            "1e+5",
            "1e-5",
            "0e0",
            "0.0e-0",
            "123.456e789",
        ] {
            assert!(is_number_grammar(text.as_bytes()), "{text}");
        }
        for text in [
            "",
            "-",
            "+1",
            ".5",
            "5.",
            "01",
            "-01",
            "00",
            "1e",
            "1e+",
            "--1",
            "0x1F",
            "1.e3",
            "1 ",
            " 1",
            "NaN",
            "Infinity",
            "-Infinity",
            "inf",
            "nan",
            "1_000",
            "1,5",
            "1e5.5",
            "e5",
            "1ee5",
            "0.",
            "-.5",
        ] {
            assert!(!is_number_grammar(text.as_bytes()), "{text}");
            assert_eq!(
                parse_number(text.as_bytes()),
                Err(NumberError::Grammar),
                "{text}"
            );
        }
    }

    #[test]
    fn a_number_reads_as_the_closest_double_and_never_as_infinity() {
        assert_eq!(parse_number(b"6.3").map(f64::to_bits), Ok(6.3f64.to_bits()));
        assert_eq!(
            parse_number(b"-0").map(f64::to_bits),
            Ok((-0.0f64).to_bits())
        );
        assert_eq!(
            parse_number(b"9007199254740993"),
            Ok(9_007_199_254_740_992.0)
        );
        assert_eq!(parse_number(b"5e-324").map(f64::to_bits), Ok(1));
        assert_eq!(parse_number(b"1.7976931348623157e308"), Ok(f64::MAX));
        for text in ["1e309", "-1e309", "1e400", "2e308"] {
            assert_eq!(
                parse_number(text.as_bytes()),
                Err(NumberError::NotFinite),
                "{text}"
            );
        }
        // A long text is still a number: the reader, unlike the loader, sets no length.
        let long = format!("1{}", "0".repeat(10_000));
        assert_eq!(parse_number(long.as_bytes()), Err(NumberError::NotFinite));
        let long = format!("0.{}1", "0".repeat(10_000));
        assert_eq!(parse_number(long.as_bytes()), Ok(0.0));
    }

    #[test]
    fn a_number_is_written_as_ecmascripts_number_to_string_writes_it() {
        // Each pair: a double, and what `String(x)` gives in a browser (the corpus tests compare thousands more).
        for (value, text) in [
            (0.0, "0"),
            (-0.0, "0"),
            (1.0, "1"),
            (-1.5, "-1.5"),
            (6.3, "6.3"),
            (0.55, "0.55"),
            (100.0, "100"),
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (123456789012345680000.0, "123456789012345680000"),
            (1.5e21, "1.5e+21"),
            (0.000001, "0.000001"),
            (0.0000001, "1e-7"),
            (0.0000012345678901234567, "0.0000012345678901234567"),
            (1.2345e-7, "1.2345e-7"),
            (5e-324, "5e-324"),
            (f64::MAX, "1.7976931348623157e+308"),
            (9007199254740992.0, "9007199254740992"),
            (0.1 + 0.2, "0.30000000000000004"),
            (327.67, "327.67"),
            (-327.67, "-327.67"),
        ] {
            assert_eq!(number_text(value), text);
            // Reading the text back gives the same double, but for the sign of zero.
            assert_eq!(parse_number(text.as_bytes()), Ok(value), "{text}");
        }
        assert_eq!(number_text(f64::NAN), "null");
        assert_eq!(number_text(f64::INFINITY), "null");
        // The longest text there is: 25 bytes (the loader's cap on a number's length rests on this).
        assert_eq!(number_text(-0.0000012345678901234567).len(), 25);
    }
}
