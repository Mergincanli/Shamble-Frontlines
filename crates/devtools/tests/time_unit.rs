//! The time unit (ADR 0004 part 4; PM-0004-11): 3,000 units a second, the same number in the content crate, which
//! converts, and in the simulation, which counts; every duration of the old game's tuning as whole units, held
//! against the digits of its own text; and the rounding report, which fails when a rule's interval is shorter than a
//! tick. The same report is `cargo run -p devtools --bin time_report -- --check`.

use std::path::{Path, PathBuf};

use content::time::{UNITS_PER_SECOND, units_from_seconds};
use devtools::files::read_plain;
use devtools::time_report::{CANDIDATES, DURATIONS, report, run};

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors")
}

fn durations() -> String {
    String::from_utf8(read_plain(&folder(), DURATIONS).expect("the extract")).expect("it is text")
}

#[test]
fn the_content_crates_unit_is_the_simulations_and_a_tick_is_a_whole_number_of_it() {
    // One number since the simulation depends on the content crate: its unit is that crate's own, re-exported. The
    // check stays for the day someone gives the simulation a number of its own again.
    assert_eq!(UNITS_PER_SECOND, sim::time::UNITS_PER_SECOND);
    assert_eq!((sim::TICK_HZ, sim::time::UNITS_PER_TICK), (30, 100));
    assert!(CANDIDATES.contains(&UNITS_PER_SECOND));
}

#[test]
fn every_old_duration_is_three_units_a_millisecond_of_its_own_text() {
    // The oracle is the text's own digits, not the report: a text of at most three decimals is a whole number of
    // milliseconds, and its units are three times that. Only the old game's own step is no such text.
    let (mut whole, mut steps) = (0, 0);
    for line in durations().lines().skip(1) {
        let fields: Vec<&str> = line.split('\t').collect();
        let [path, unit, kind, text] = fields[..] else {
            panic!("{line}")
        };
        let read: f64 = text.parse().expect(text);
        let seconds = if unit == "ms" { read / 1_000.0 } else { read };
        if kind == "step" {
            assert_eq!(
                (text, units_from_seconds(seconds)),
                ("0.016666666666666666", Ok(50))
            );
            steps += 1;
            continue;
        }
        let (ones, decimals) = text.split_once('.').unwrap_or((text, ""));
        let places = if unit == "ms" { 0 } else { 3 };
        assert!(
            decimals.len() <= places,
            "{path}: {text} {unit} is no whole number of milliseconds"
        );
        let digits = format!("{ones}{decimals:0<places$}");
        let milliseconds: u32 = digits.parse().expect(text);
        assert_eq!(units_from_seconds(seconds), Ok(3 * milliseconds), "{path}");
        whole += 1;
    }
    assert_eq!((whole, steps), (68, 1));
}

#[test]
fn the_committed_report_is_what_this_build_makes_of_the_committed_durations() {
    let done = run(&folder(), true);
    assert_eq!(
        done,
        Ok("time-report.txt is what this build makes of legacy-durations.txt".to_owned())
    );
    let made = report(&durations()).expect("the report");
    assert!(made.contains("\ncandidate 3000 (the unit): 68 of 69 without rounding; worst under one millionth (tables.FIXED_DT.values 0.016666666666666666 s)\n"));
    assert!(made.contains("\ncandidate 600: 67 of 69 without rounding; worst 4629 millionths (tables.WEAPONS.values[2].fireInterval 0.072 s)\n"));
    assert!(made.contains(
        "\ntables.WEAPONS.values[2].fireInterval\ts\trules\t0.072\t216\t2.16\t4.32\texact\n"
    ));
    assert!(
        made.contains("\ntables.WR_DEPLOY_MS.values\tms\tlook\t280\t840\t8.40\t16.80\texact\n")
    );
}

#[test]
fn a_rules_interval_under_one_tick_fails_the_report_and_an_extract_that_is_none_is_refused() {
    let with = |line: &str| format!("{}{line}\n", durations().replacen(" 69\n", " 70\n", 1));
    // A planted interval of 0.02 s, 60 units, is under the tick's 100: the report fails by its path, where a look's
    // duration of the same length is only a row.
    let short = report(&with(
        "tables.WEAPONS.values[9].fireInterval\ts\trules\t0.02",
    ));
    let named = "rules' intervals shorter than one tick at 30 Hz (1): tables.WEAPONS.values[9].fireInterval (0.02 s, 60 units)";
    assert_eq!(short, Err(named.to_owned()));
    let look =
        report(&with("tables.T.values.sparkLife\ts\tlook\t0.02")).expect("a look's duration");
    assert!(look.ends_with("tables.T.values.sparkLife\ts\tlook\t0.02\t60\t0.60\t1.20\texact\n"));
    let rounded = report(&with("tables.T.values.sparkLife\ts\tlook\t0.0005"))
        .expect("half a millisecond and a half unit");
    assert!(
        rounded.ends_with("\t0.0005\t2\t0.02\t0.04\trounded by 333333 millionths\n"),
        "{rounded}"
    );
    // Four and a half units go up to five: to the even count they would go down to four.
    let up =
        report(&with("tables.T.values.sparkLife\ts\tlook\t0.0015")).expect("four and a half units");
    assert!(
        up.ends_with("\t0.0015\t5\t0.05\t0.10\trounded by 111111 millionths\n"),
        "{up}"
    );
    // An extract that miscounts, a line that is no four fields, a unit and a kind that are none, a text that is no
    // plain decimal: each refused by name.
    let refused = |text: String| report(&text).expect_err("refused");
    assert!(
        refused(durations().replacen(" 69\n", " 68\n", 1)).ends_with("and it holds 69 durations")
    );
    assert!(refused(with("a.b\ts\trules")).starts_with("not a path, a unit, a kind and a text"));
    assert!(
        refused(with("a.b\tmin\trules\t1")).starts_with("not a path, a unit, a kind and a text")
    );
    assert!(refused(with("a.b\ts\tplay\t1")).starts_with("not a path, a unit, a kind and a text"));
    for text in ["1e-3", "-1", ".5", "1.", ""] {
        let why = refused(with(&format!("a.b\ts\tlook\t{text}")));
        assert!(
            why.contains("is not a plain decimal") || why.contains("is refused"),
            "{text}: {why}"
        );
    }
}
