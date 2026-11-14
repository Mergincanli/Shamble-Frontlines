//! Exact fractions: thresholds, means, shares and factors are compared without rounding (plan §6.1). Display alone
//! goes through a float.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

/// A reduced fraction with a positive denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio {
    num: i128,
    den: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn checked(v: Option<i128>) -> i128 {
    // Values are i64 samples, counts and decimal thresholds: i128 leaves room for every product the judge forms.
    v.expect("npt-judge: a fraction overflowed i128; the inputs are far outside any mark's range")
}

impl Ratio {
    /// `num / den`, reduced. Panics on a zero denominator, which no caller forms.
    pub fn new(num: i128, den: i128) -> Ratio {
        assert!(den != 0, "npt-judge: a fraction with denominator 0");
        let (n, d) = if den < 0 {
            (checked(num.checked_neg()), checked(den.checked_neg()))
        } else {
            (num, den)
        };
        let g = gcd(n, d);
        if g > 1 {
            Ratio {
                num: n / g,
                den: d / g,
            }
        } else {
            Ratio { num: n, den: d }
        }
    }

    pub fn int(n: i128) -> Ratio {
        Ratio { num: n, den: 1 }
    }

    pub fn num(&self) -> i128 {
        self.num
    }

    pub fn den(&self) -> i128 {
        self.den
    }

    /// Reads an integer or a decimal: `12`, `-3`, `0.999`, `1.10`. A point needs a digit on each side; no exponent.
    pub fn parse(s: &str) -> Result<Ratio, String> {
        let (neg, body) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };
        let (int_part, frac_part) = match body.split_once('.') {
            Some((i, f)) => (i, f),
            None => (body, ""),
        };
        let digits_ok = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
        if !digits_ok(int_part) || (body.contains('.') && !digits_ok(frac_part)) {
            return Err(format!("{s:?} is not a decimal number"));
        }
        if int_part.len() + frac_part.len() > 30 {
            return Err(format!("{s:?} has more than 30 digits"));
        }
        let mut num: i128 = 0;
        for b in int_part.bytes().chain(frac_part.bytes()) {
            num = num * 10 + i128::from(b - b'0');
        }
        let den = 10_i128.pow(frac_part.len() as u32);
        Ok(Ratio::new(if neg { -num } else { num }, den))
    }

    /// Divides by a positive integer.
    pub fn div_int(self, n: i128) -> Ratio {
        Ratio::new(self.num, checked(self.den.checked_mul(n)))
    }

    pub fn abs(self) -> Ratio {
        Ratio::new(self.num.abs(), self.den)
    }

    /// For display only.
    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, o: Ratio) -> Ratio {
        Ratio::new(
            checked(self.num.checked_mul(o.num)),
            checked(self.den.checked_mul(o.den)),
        )
    }
}

impl Add for Ratio {
    type Output = Ratio;

    fn add(self, o: Ratio) -> Ratio {
        let num = checked(
            checked(self.num.checked_mul(o.den)).checked_add(checked(o.num.checked_mul(self.den))),
        );
        Ratio::new(num, checked(self.den.checked_mul(o.den)))
    }
}

impl Sub for Ratio {
    type Output = Ratio;

    fn sub(self, o: Ratio) -> Ratio {
        let num = checked(
            checked(self.num.checked_mul(o.den)).checked_sub(checked(o.num.checked_mul(self.den))),
        );
        Ratio::new(num, checked(self.den.checked_mul(o.den)))
    }
}

impl Ord for Ratio {
    fn cmp(&self, o: &Ratio) -> Ordering {
        let l = checked(self.num.checked_mul(o.den));
        let r = checked(o.num.checked_mul(self.den));
        l.cmp(&r)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, o: &Ratio) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            let v = self.to_f64();
            let text = format!("{v:.6}");
            let text = text.trim_end_matches('0').trim_end_matches('.');
            write!(f, "{text}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_decimals_exactly() {
        assert_eq!(Ratio::parse("0.999").unwrap(), Ratio::new(999, 1000));
        assert_eq!(Ratio::parse("1.10").unwrap(), Ratio::new(11, 10));
        assert_eq!(Ratio::parse("-3").unwrap(), Ratio::int(-3));
        assert_eq!(Ratio::parse("16670000").unwrap(), Ratio::int(16_670_000));
        for bad in ["", ".5", "5.", "1e3", "+1", "1.2.3", "0x10", " 1"] {
            assert!(Ratio::parse(bad).is_err(), "{bad:?} must be refused");
        }
    }

    #[test]
    fn compares_without_rounding() {
        let share = Ratio::new(999, 1000);
        assert!(Ratio::new(9_990, 10_000) >= share);
        assert!(Ratio::new(9_989, 10_000) < share);
        assert_eq!(Ratio::new(2, -4), Ratio::new(-1, 2));
        assert_eq!(
            Ratio::parse("1.10").unwrap() * Ratio::int(10),
            Ratio::int(11)
        );
        assert_eq!(Ratio::new(1, 3) + Ratio::new(1, 6), Ratio::new(1, 2));
        assert_eq!(Ratio::new(1, 2) - Ratio::new(3, 4), Ratio::new(-1, 4));
        assert_eq!(Ratio::int(7).div_int(2), Ratio::new(7, 2));
    }

    #[test]
    fn displays_short() {
        assert_eq!(Ratio::int(12).to_string(), "12");
        assert_eq!(Ratio::new(1, 4).to_string(), "0.25");
        assert_eq!(Ratio::new(2, 3).to_string(), "0.666667");
    }
}
