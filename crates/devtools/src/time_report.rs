//! The time unit's rounding report (ADR 0004 part 4; PM-0004-11): every duration of the old game's tuning in whole
//! units of a 3,000th of a second, with what the rounding costs, and the same count for other units a second, so
//! that the figure chosen stands beside the ones it was chosen over. The durations come from
//! `vectors/legacy-durations.txt` (`packages/client/scripts/legacy-extract.mjs` writes it from the tuning file and
//! the reviewed list of duration fields); the report is `vectors/time-report.txt`, and a test holds the committed
//! file to what this module makes.
//!
//! Each duration is converted twice: by the content crate's own conversion, on the double its text reads as, and by
//! whole-number arithmetic on the text's own digits. The two must agree, or the report fails. It fails as well when
//! a rule's interval is shorter than one tick at 30 Hz: the simulation could not tell such an interval from a tick.

use std::fmt::Write as _;
use std::path::Path;

use content::number::parse_number;
use content::time::{UNITS_PER_SECOND, units_from_seconds};

use crate::files::{read_plain, write_plain};

/// The extract the report reads and the file it is.
pub const DURATIONS: &str = "legacy-durations.txt";
pub const REPORT: &str = "time-report.txt";
/// The units a second the report compares; the one chosen is `UNITS_PER_SECOND`.
pub const CANDIDATES: [u32; 5] = [60, 600, 3_000, 6_000, 60_000];

/// A duration's text as a whole number over a power of ten, in seconds: `0.072` is 72 over 10^3, and `280` in
/// milliseconds is 280 over 10^3.
fn exact(text: &str, unit: &str) -> Result<(u128, u32), String> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    let digits = format!("{whole}{fraction}");
    let plain =
        !whole.is_empty() && digits.len() <= 30 && digits.bytes().all(|byte| byte.is_ascii_digit());
    let number = digits
        .parse()
        .ok()
        .filter(|_| plain)
        .ok_or(format!("{text:?} is not a plain decimal"))?;
    Ok((
        number,
        fraction.len() as u32 + if unit == "ms" { 3 } else { 0 },
    ))
}

/// `number / 10^places` seconds in units of a `per_second`th: the nearest whole count (half goes up), whether that
/// count is the duration with nothing rounded, and how far the rounding moved it, in millionths of the duration.
fn units_exact(number: u128, places: u32, per_second: u32) -> (u128, bool, u128) {
    let (scaled, scale) = (number * u128::from(per_second), 10_u128.pow(places));
    let units = (scaled + scale / 2) / scale;
    let off = (units * scale).abs_diff(scaled) * 1_000_000 / scaled.max(1);
    (units, scaled.is_multiple_of(scale), off)
}

/// A count of hundredths as a decimal: `216` is `2.16`.
fn hundredths(count: u128) -> String {
    format!("{}.{:02}", count / 100, count % 100)
}

/// The report's text for the durations extract's text.
///
/// # Errors
/// An extract that is not one, a duration the two conversions disagree on, or a rule's interval under one tick at
/// 30 Hz, each by its path.
pub fn report(durations: &str) -> Result<String, String> {
    let mut lines = durations.lines();
    let first = lines.next().unwrap_or("");
    let mut rows = String::new();
    // Per candidate: how many durations it holds without rounding, and the worst rounding with its path.
    let mut held = CANDIDATES.map(|_| (0_u32, 0_u128, String::new()));
    let (mut count, mut short) = (0_u32, Vec::new());
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let [
            path,
            unit @ ("s" | "ms"),
            kind @ ("rules" | "look" | "step"),
            text,
        ] = fields[..]
        else {
            return Err(format!("not a path, a unit, a kind and a text: {line:?}"));
        };
        count += 1;
        let (number, places) = exact(text, unit).map_err(|why| format!("{path}: {why}"))?;
        for (candidate, (whole, worst, at)) in CANDIDATES.iter().zip(&mut held) {
            let (_, exact, off) = units_exact(number, places, *candidate);
            *whole += u32::from(exact);
            if !exact && (off > *worst || at.is_empty()) {
                (*worst, *at) = (off, format!("{path} {text} {unit}"));
            }
        }
        // The content crate's conversion, on the double the content reader makes of the text.
        let read = parse_number(text.as_bytes())
            .map_err(|error| format!("{path}: {text} is refused: {error:?}"))?;
        let seconds = if unit == "ms" { read / 1_000.0 } else { read };
        let units = units_from_seconds(seconds)
            .map_err(|error| format!("{path}: {text} {unit}: {error:?}"))?;
        let (whole, exact, off) = units_exact(number, places, UNITS_PER_SECOND);
        if u128::from(units) != whole {
            return Err(format!(
                "{path}: the conversion gives {units} units where the text's digits give {whole}"
            ));
        }
        if kind == "rules" && units < UNITS_PER_SECOND / 30 {
            short.push(format!("{path} ({text} {unit}, {units} units)"));
        }
        let note = match (exact, kind) {
            (true, _) => "exact".to_owned(),
            (false, "step") => {
                "rounded: the old game's own step, which the tick replaces".to_owned()
            }
            (false, _) => format!("rounded by {off} millionths"),
        };
        let per_tick = |hz: u32| {
            hundredths(u128::from(units) * u128::from(hz) * 100 / u128::from(UNITS_PER_SECOND))
        };
        let _ = writeln!(
            rows,
            "{path}\t{unit}\t{kind}\t{text}\t{units}\t{}\t{}\t{note}",
            per_tick(30),
            per_tick(60)
        );
    }
    if first.rsplit(' ').next() != Some(count.to_string().as_str())
        || !first.starts_with("shamble-legacy-durations 1 ")
    {
        return Err(format!(
            "the extract's first line is {first:?}, and it holds {count} durations"
        ));
    }
    if !short.is_empty() {
        return Err(format!(
            "rules' intervals shorter than one tick at 30 Hz ({}): {}",
            short.len(),
            short.join(", ")
        ));
    }
    let mut text = format!(
        "shamble-time-report 1 {count}\n# The time unit is {UNITS_PER_SECOND} units a second: {} a tick at 30 Hz, {} at 60 Hz. The durations are those of\n\
         # {first}\n# Units a second, compared: how many of the {count} durations each holds without rounding, and the worst rounding.\n",
        UNITS_PER_SECOND / 30,
        UNITS_PER_SECOND / 60
    );
    for (candidate, (whole, worst, at)) in CANDIDATES.iter().zip(&held) {
        let chosen = if *candidate == UNITS_PER_SECOND {
            " (the unit)"
        } else {
            ""
        };
        let worst = match (at.is_empty(), *worst) {
            (true, _) => "no rounding".to_owned(),
            (false, 0) => format!("worst under one millionth ({at})"),
            (false, worst) => format!("worst {worst} millionths ({at})"),
        };
        let _ = writeln!(
            text,
            "candidate {candidate}{chosen}: {whole} of {count} without rounding; {worst}"
        );
    }
    text.push_str("# path, unit, kind, text as written, units, ticks at 30 Hz, ticks at 60 Hz, note (tab-separated)\n");
    Ok(text + &rows)
}

/// Makes the report from the extract in `folder`; with `check`, compares it with the committed report and writes
/// nothing, else writes it. The line to print comes back.
///
/// # Errors
/// What `report` gives, a file that is no plain file, or a committed report that is not what this build makes.
pub fn run(folder: &Path, check: bool) -> Result<String, String> {
    let durations = String::from_utf8(read_plain(folder, DURATIONS)?)
        .map_err(|_| format!("{DURATIONS} is not text"))?;
    let made = report(&durations)?;
    if !check {
        write_plain(folder, REPORT, made.as_bytes())?;
        return Ok(format!("{REPORT} written"));
    }
    if read_plain(folder, REPORT)? != made.as_bytes() {
        return Err(format!(
            "{REPORT} is not what this build makes of {DURATIONS}: run time_report without --check"
        ));
    }
    Ok(format!("{REPORT} is what this build makes of {DURATIONS}"))
}
