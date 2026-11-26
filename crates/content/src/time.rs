//! Durations: written in seconds, counted in whole units of a 3,000th of a second (ADR 0004 part 4).
//!
//! 3,000 is the least number that a millisecond and a 60th of a second both divide. Every duration of the old game's
//! tuning but its own step is a whole number of milliseconds and converts exactly, and a tick is a whole number of
//! units at 30 Hz (100) and at 60 Hz (50), so a duration in units stays the same when the tick rate changes. The
//! rounding report lists every old duration with its units (`devtools`'s `time_report`, PM-0004-11). A content file
//! keeps a duration as it is written, in seconds; whoever starts a match converts it here, once, when it works out
//! the match's own numbers from the content. Nothing converts while a match runs: timers count units.

/// Units in a second.
pub const UNITS_PER_SECOND: u32 = 3_000;

/// Why a number of seconds is no duration the game counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationError {
    NotFinite,
    Negative,
    /// More units than 32 bits hold: about 16.5 days.
    TooLong,
}

/// A duration in seconds as whole units, to the nearest unit (half a unit goes up).
///
/// # Errors
/// A number that is not finite, is below zero, or is too long to count.
pub fn units_from_seconds(seconds: f64) -> Result<u32, DurationError> {
    if !seconds.is_finite() {
        return Err(DurationError::NotFinite);
    }
    if seconds < 0.0 {
        return Err(DurationError::Negative);
    }
    let units = (seconds * f64::from(UNITS_PER_SECOND)).round();
    if units > f64::from(u32::MAX) {
        return Err(DurationError::TooLong);
    }
    // In range and whole by the lines above: the cast loses nothing.
    Ok(units as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_duration_is_rounded_to_the_nearest_unit_never_cut() {
        // 0.072 s times 3,000 is 215.99999999999997 as a double: 216 units, where cutting would give 215.
        assert!(std::hint::black_box(0.072_f64) * 3_000.0 < 216.0);
        #[rustfmt::skip]
        let cases = [
            (0.072, 216), (0.06, 180), (0.02, 60), (1.0 / 60.0, 50), (1.0 / 30.0, 100), (0.0, 0), (-0.0, 0),
            (7.0, 21_000), (0.28, 840), (0.5 / 3_000.0, 1), (0.49 / 3_000.0, 0), (1.5 / 3_000.0, 2),
        ];
        for (seconds, units) in cases {
            assert_eq!(units_from_seconds(seconds), Ok(units), "{seconds}");
        }
    }

    #[test]
    fn a_duration_that_cannot_be_counted_is_refused() {
        let longest = f64::from(u32::MAX) / 3_000.0;
        assert_eq!(units_from_seconds(longest), Ok(u32::MAX));
        assert_eq!(
            units_from_seconds(f64::from(u32::MAX - 1) / 3_000.0),
            Ok(u32::MAX - 1)
        );
        assert_eq!(
            units_from_seconds(4_294_967_296.0 / 3_000.0),
            Err(DurationError::TooLong)
        );
        assert_eq!(units_from_seconds(1e300), Err(DurationError::TooLong));
        assert_eq!(units_from_seconds(-0.001), Err(DurationError::Negative));
        for seconds in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(units_from_seconds(seconds), Err(DurationError::NotFinite));
        }
    }
}
