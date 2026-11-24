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
    /// Exact at every size: the cross products where they fit i128, else `cmp_wide` (a figure that divides one sum
    /// of fractions by another, as -10's cost ratio does, can pass them).
    fn cmp(&self, o: &Ratio) -> Ordering {
        match (self.num.checked_mul(o.den), o.num.checked_mul(self.den)) {
            (Some(l), Some(r)) => l.cmp(&r),
            _ => cmp_wide((self.num, self.den), (o.num, o.den)),
        }
    }
}

/// `a.0 / a.1` against `b.0 / b.1`, both denominators above 0, with no product formed: the whole parts first, then
/// the remainders, compared through their reciprocals with the order turned round (Euclid's steps, so it ends).
fn cmp_wide(mut a: (i128, i128), mut b: (i128, i128)) -> Ordering {
    let mut turned = false;
    loop {
        let (whole_a, rest_a) = (a.0.div_euclid(a.1), a.0.rem_euclid(a.1));
        let (whole_b, rest_b) = (b.0.div_euclid(b.1), b.0.rem_euclid(b.1));
        let order = match (whole_a.cmp(&whole_b), rest_a, rest_b) {
            (Ordering::Equal, 0, 0) => Ordering::Equal,
            (Ordering::Equal, 0, _) => Ordering::Less,
            (Ordering::Equal, _, 0) => Ordering::Greater,
            (Ordering::Equal, _, _) => {
                (a, b, turned) = ((a.1, rest_a), (b.1, rest_b), !turned);
                continue;
            }
            (order, _, _) => order,
        };
        return if turned { order.reverse() } else { order };
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
    fn compares_fractions_whose_cross_products_pass_i128() {
        let big = 10_i128.pow(30);
        // 1 + 1/10^30 against 1 + 1/(10^30 + 1): the cross products are near 10^60.
        let (a, b) = (Ratio::new(big + 1, big), Ratio::new(big + 2, big + 1));
        assert!(a > b && b < a && a != b);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert!(Ratio::new(-(big + 1), big) < Ratio::new(-(big + 2), big + 1));
        assert!(Ratio::new(-(big + 1), big) < b && b > Ratio::int(1));
        assert!(
            Ratio::new(big * 7 + 3, big + 7) < Ratio::int(7)
                && Ratio::int(7) > Ratio::new(big * 7 + 3, big + 7)
        );
        // The wide comparison agrees with the cross products wherever those fit.
        let small: Vec<(i128, i128)> = (-6..=6)
            .flat_map(|n| (1..=6).map(move |d| (n, d)))
            .collect();
        for a in &small {
            for b in &small {
                assert_eq!(
                    cmp_wide(*a, *b),
                    (a.0 * b.1).cmp(&(b.0 * a.1)),
                    "{a:?} {b:?}"
                );
            }
        }
    }

    #[test]
    fn displays_short() {
        assert_eq!(Ratio::int(12).to_string(), "12");
        assert_eq!(Ratio::new(1, 4).to_string(), "0.25");
        assert_eq!(Ratio::new(2, 3).to_string(), "0.666667");
    }
}
