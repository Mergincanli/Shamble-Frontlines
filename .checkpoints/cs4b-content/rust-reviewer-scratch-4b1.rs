// rust-reviewer's scratch for change-set 4b-1 (not part of any build of the repository).
// Built by hand with rustc against crates/content (see rust-reviewer-review-4b1.md, "Runs").
// `scratch numbers <out>`: writes "<bits> <number_text>" lines for the Node oracle, and checks two things here:
//   (a) parse_number(number_text(x)) == x for every finite x written (round trip);
//   (b) in the tie window (x = M * 2^-q, q = 1..=25, M odd), number_text's digits against ECMAScript's rule worked
//       out apart from the crate: candidates s-1, s, s+1 at the std text's length, valid when std's own parser reads
//       them back as x, closest by exact u128 arithmetic (x = M*5^q / 10^q), a tie to the even one.
// `scratch loader`: the loader spot-checks (0.1 + 0.2 as a corner, deep nesting, a bundle holding its own closing lines).
use content::number::{number_text, parse_number};
use std::io::Write;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}

/// std's shortest digits and decimal places for a positive finite x: ("d1d2..dk", decimals = k - n).
fn std_digits(x: f64) -> (String, i32) {
    let s = format!("{:e}", x);
    let (m, e) = s.split_once('e').unwrap();
    let digits: String = m.chars().filter(|c| c.is_ascii_digit()).collect();
    let e: i32 = e.parse().unwrap();
    let decimals = digits.len() as i32 - (e + 1);
    (digits, decimals)
}

/// ECMAScript's digits for x = m * 2^-q (m odd, q >= 1), worked out from exact arithmetic and std's parser only.
fn es_digits(x: f64, m: u64, q: u32) -> Option<(u128, i32)> {
    let (digits, d) = std_digits(x);
    if d < 0 || d as u32 > q {
        return None; // outside what this oracle handles exactly (no tie possible there, by the reviewer's algebra)
    }
    let s_r: u128 = digits.parse().unwrap();
    let exact: u128 = (m as u128).checked_mul(5u128.checked_pow(q)?)?; // x = exact / 10^q
    let scale = 10u128.checked_pow(q - d as u32)?;
    let mut best: Option<(u128, u128)> = None; // (distance, s)
    for s in [s_r.checked_sub(1)?, s_r, s_r + 1] {
        if s == 0 {
            continue;
        }
        let text = format!("{s}e-{d}");
        let back: f64 = text.parse().unwrap();
        if back != x {
            continue;
        }
        let at = s.checked_mul(scale)?;
        let dist = at.abs_diff(exact);
        best = match best {
            None => Some((dist, s)),
            Some((bd, bs)) if dist < bd || (dist == bd && s % 2 == 0 && bs % 2 == 1) => Some((dist, s)),
            keep => keep,
        };
    }
    best.map(|(_, s)| (s, d))
}

fn numbers(out_path: &str) {
    let mut out = std::io::BufWriter::new(std::fs::File::create(out_path).unwrap());
    let mut written = 0u64;
    let mut not_round_trip: Vec<String> = Vec::new();
    let mut emit = |x: f64, out: &mut std::io::BufWriter<std::fs::File>| {
        if !x.is_finite() {
            return;
        }
        let text = number_text(x);
        writeln!(out, "{:016x} {}", x.to_bits(), text).unwrap();
        written += 1;
        let back = parse_number(text.as_bytes());
        if back != Ok(x) && not_round_trip.len() < 20 && !not_round_trip.iter().any(|w: &String| w.starts_with(&format!("{:016x}", x.to_bits()))) {
            not_round_trip.push(format!("{:016x} {} -> {:?}", x.to_bits(), text, back.map(f64::to_bits)));
        }
    };
    // 1. Every power of two, both signs, and its two neighbours.
    for e in 0..2047u64 {
        for sign in [0u64, 1 << 63] {
            let b = (e << 52) | sign;
            for bits in [b.wrapping_sub(1), b, b + 1] {
                if bits >> 52 & 0x7ff != 0x7ff {
                    emit(f64::from_bits(bits), &mut out);
                }
            }
        }
    }
    // 2. The tie window, checked here against ECMAScript's rule, and written for Node too.
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut checked = 0u64;
    let mut ties = 0u64;
    let mut wrong: Vec<String> = Vec::new();
    let mut wrong_total = 0u64;
    for q in 1..=25u32 {
        let five = 5u128.pow(q);
        let limit = ((10u128.pow(19) / five) as u64).clamp(1, (1u64 << 53) - 1);
        for i in 0..400_000u64 {
            let m = if i < 20_000 { 2 * i + 1 } else { (rng.next() % limit) | 1 };
            if m >= 1 << 53 || m > limit.max(1) && i < 20_000 {
                continue;
            }
            let x = m as f64 * 2f64.powi(-(q as i32)); // exact: m < 2^53, a power of two scale, normal range
            for (xx, mm, qq) in [(x, m, q)] {
                let Some((s, d)) = es_digits(xx, mm, qq) else { continue };
                checked += 1;
                let (sd, _) = std_digits(xx);
                if s.to_string() != sd {
                    ties += 1;
                }
                // number_text's digits and places, read back from its text.
                let t = number_text(xx);
                let back: f64 = t.parse().unwrap();
                let (td, tdec) = std_digits(back);
                let expect_back: f64 = format!("{s}e-{d}").parse().unwrap();
                let mantissa_part = t.split('e').next().unwrap_or("");
                let text_digits: String = mantissa_part.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                let text_digits = text_digits.trim_start_matches('0').trim_end_matches('0').to_owned();
                let s_trim = s.to_string().trim_end_matches('0').to_owned();
                if text_digits != s_trim || back != expect_back {
                    wrong_total += 1;
                    if wrong.len() < 20 && !wrong.iter().any(|w: &String| w.starts_with(&format!("{:016x}", xx.to_bits()))) {
                        wrong.push(format!(
                            "{:016x} m={mm} q={qq}: number_text {t} (reads as {:016x}, digits {td} dec {tdec}), ECMAScript {s}e-{d}",
                            xx.to_bits(),
                            back.to_bits()
                        ));
                    }
                }
                emit(xx, &mut out);
                emit(f64::from_bits(xx.to_bits() + 1), &mut out);
                emit(f64::from_bits(xx.to_bits() - 1), &mut out);
            }
        }
    }
    // 3. Random doubles over the whole range.
    for _ in 0..2_000_000 {
        emit(f64::from_bits(rng.next()), &mut out);
    }
    drop(emit);
    out.flush().unwrap();
    println!("written {written}; tie-window values checked {checked}, of which ECMAScript differs from std: {ties}");
    println!("tie-window disagreements with ECMAScript's rule: {wrong_total} (distinct shown: {})", wrong.len());
    for line in &wrong {
        println!("  {line}");
    }
    println!("not round-tripping: {}", not_round_trip.len());
    for line in &not_round_trip {
        println!("  {line}");
    }
}

fn loader() {
    use content::bundle::{load, load_file, write};
    use content::map;
    // A map whose corner is 0.1 + 0.2: canonical text of that double, as a shape's min y.
    let corner = number_text(0.1 + 0.2);
    let base = r#"{"bounds": {"max": [50, 20, 50], "min": [-50, 0, -50]}, "format": "shamble-map", "id": "yard", "regions": [], "shapes": [{"group": "yard", "id": "s", "max": [1, 1, 1], "min": [0, CORNER, 0], "type": "box"}], "spawns": [{"at": [0, 0, -5], "facing_deg": 90, "id": "a"}], "version": 1}"#;
    let one_line = base.replace("CORNER", &corner);
    let top = content::json::parse(one_line.as_bytes()).expect("one line parses");
    let text = content::write::canonical(&top);
    let r = load_file("maps/yard.json", text.as_bytes());
    println!("corner {corner}: {:?}", r.map(|_| "taken").map_err(|e| e.record()));
    // The same corner as 0.3 is taken.
    let ok = content::write::canonical(&content::json::parse(base.replace("CORNER", "0.3").as_bytes()).unwrap());
    println!("corner 0.3: {:?}", load_file("maps/yard.json", ok.as_bytes()).map(|_| "taken").map_err(|e| e.record()));
    // A bound at 0.1 + 0.2 on the centimetre grid, and 327.67 + 0.01 (the reach plus a centimetre, by addition).
    let m = map::read(&content::json::parse(ok.as_bytes()).unwrap()).unwrap();
    let mut m2 = m.clone();
    m2.bounds.max[1] = 0.1 + 0.2;
    m2.shapes[0].max[1] = 0.3;
    println!("bound y 0.1+0.2: {:?}", map::check(&m2).map_err(|e| e.record()));
    let mut m3 = m.clone();
    m3.bounds.max[0] = 327.67 + 0.01;
    println!("bound x 327.67+0.01 = {}: {:?}", number_text(327.67 + 0.01), map::check(&m3).map_err(|e| e.record()));
    // Deep nesting, in a file and in the bundle itself: a million brackets, refused, no stack overflow.
    let deep_file = format!("{{\n  \"a\": {}", "[".repeat(1_000_000));
    println!("deep file: {:?}", load_file("movement/a.json", deep_file.as_bytes()).map(|_| ()).map_err(|e| e.record()));
    let deep_bundle = format!("{{\n  \"files\": {}", "[".repeat(1_000_000));
    println!("deep bundle: {:?}", load(deep_bundle.as_bytes()).map(|_| ()).map_err(|e| e.record()));
    let deep_in_bundle = write(&[("movement/a.json", &deep_file)]);
    println!("deep file in a bundle: {:?}", load(deep_in_bundle.as_bytes()).map(|_| ()).map_err(|e| e.record()));
    // Objects nested to the cap, then one more, then a million: the parse and the drop of the tree.
    let nested = |n: usize| format!("{}1{}", "{\"a\": ".repeat(n), "}".repeat(n));
    println!("32 objects: {:?}", content::json::parse(nested(32).as_bytes()).map(|_| ()).map_err(|e| e.record()));
    println!("33 objects: {:?}", content::json::parse(nested(33).as_bytes()).map(|_| ()).map_err(|e| e.record()));
    // A bundle whose file text holds the bundle's own closing lines.
    let closing = "\n  ],\n  \"format\": \"shamble-content-bundle\",\n  \"version\": 1\n}\n";
    let tm = content::movement::TEST_TEXT;
    let tricky = format!("{}{}", tm, closing);
    let b = write(&[("movement/test-soldier.json", &tricky)]);
    println!("file text + closing lines: {:?}", load(b.as_bytes()).map(|s| s.movements().len()).map_err(|e| e.record()));
    // The closing lines inside a string field of the file (the id): refused as an id, inside the file.
    let tricky2 = tm.replace("\"id\": \"test-soldier\"", &format!("\"id\": \"{}\"", closing.replace('"', "\\\"").replace('\n', "\\n")));
    let b2 = write(&[("movement/test-soldier.json", &tricky2)]);
    println!("closing lines as the id: {:?}", load(b2.as_bytes()).map(|s| s.movements().len()).map_err(|e| e.record()));
    // A bundle that is itself the text of a file, which holds a file (backslashes): refused by the escape.
    let inner = write(&[("movement/test-soldier.json", tm)]);
    let b3 = write(&[("movement/test-soldier.json", &inner)]);
    println!("bundle in a bundle: {:?}", load(b3.as_bytes()).map(|s| s.movements().len()).map_err(|e| e.record()));
    // The valid bundle, for contrast.
    println!("valid: {:?}", load(write(&[("movement/test-soldier.json", tm)]).as_bytes()).map(|s| s.movements().len()).map_err(|e| e.record()));
    // Ties and near-ties through number_text: the crate's own three, three of the reviewer's, each with bits +-1.
    for bits in [0x42d4_61b1_91e0_dfc8_u64, 0x431d_6ecb_23d5_7117, 0x4215_553b_6f88_d800, 0x3e60_0000_0000_0000, 0x3e70_0000_0000_0000, 0xbe70_0000_0000_0000] {
        for b in [bits - 1, bits, bits + 1] {
            let x = f64::from_bits(b);
            println!("{b:016x} number_text {} std {:e} round-trips {}", number_text(x), x, parse_number(number_text(x).as_bytes()) == Ok(x));
        }
    }
}

/// The loader's cost per byte near the 8 MiB cap, by the kind of bytes: each case 7 times, min and median wall time.
fn speed() {
    use content::bundle::{load, write};
    use std::time::Instant;
    let tm = content::movement::TEST_TEXT;
    // A movement file with one extra member `zz` holding `extra` (sorts last: canonical), refused as an unknown key
    // after the whole bundle and the file were read and compared.
    let with_extra = |extra: &str, id: &str| {
        let body = tm.trim_end_matches("\n}\n").replace("test-soldier", id);
        format!("{body},\n  \"zz\": {extra}\n}}\n")
    };
    let ids: Vec<String> = (0..16).map(|at| format!("f{at:02}")).collect();
    // 1. Strings: 8 files of one 1,000,000-byte string each (the crate's own limit case, about 8 MB).
    let string = format!("\"{}\"", "x".repeat(1_000_000));
    let strings: Vec<(String, String)> = ids.iter().take(8).map(|id| (format!("movement/{id}.json"), with_extra(&string, id))).collect();
    // 2. Numbers: 16 files of an array of 16,000 numbers of 17 significant digits each (number_text per number).
    let mut seed = 0x1234_5678_9abc_def1_u64;
    let mut numbers_of = |count: usize| {
        let items: Vec<String> = (0..count).map(|_| {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            let x = f64::from_bits((seed >> 12) | 0x3ff0_0000_0000_0000) * 1000.0;
            format!("    {}", number_text(x))
        }).collect();
        format!("[\n{}\n  ]", items.join(",\n"))
    };
    let numbers: Vec<(String, String)> = ids.iter().map(|id| (format!("movement/{id}.json"), with_extra(&numbers_of(16_000), id))).collect();
    // 3. Keys: 16 files of an object of 16,000 distinct keys (the duplicate search sorts 16,000 keys per file).
    let keys = format!("{{\n{}\n  }}", (0..16_000).map(|at| format!("    \"k{at:05}\": 0")).collect::<Vec<_>>().join(",\n"));
    let keyed: Vec<(String, String)> = ids.iter().map(|id| (format!("movement/{id}.json"), with_extra(&keys, id))).collect();
    // 4. Equal keys: one file, 16,383 times the same key (refused as a duplicate once the object is read).
    let same = format!("{{{}}}\n", vec!["\"a\": 0"; 16_383].join(", "));
    for (name, files) in [("strings", &strings), ("numbers", &numbers), ("keys", &keyed)] {
        let refs: Vec<(&str, &str)> = files.iter().map(|(p, t)| (p.as_str(), t.as_str())).collect();
        let bundle = write(&refs);
        let mut times: Vec<f64> = (0..7).map(|_| {
            let started = Instant::now();
            let verdict = load(bundle.as_bytes());
            let took = started.elapsed().as_secs_f64();
            assert!(verdict.is_err());
            took
        }).collect();
        times.sort_by(f64::total_cmp);
        let verdict = load(bundle.as_bytes()).map(|_| ()).map_err(|e| e.record());
        let bytes = bundle.len();
        println!("{name}: {bytes} bytes; min {:.1} ms, median {:.1} ms, {:.1} ns/byte (median); {verdict:?}", times[0] * 1e3, times[3] * 1e3, times[3] * 1e9 / bytes as f64);
    }
    let mut times: Vec<f64> = (0..7).map(|_| {
        let started = Instant::now();
        let verdict = content::bundle::load_file("movement/a.json", same.as_bytes());
        assert!(verdict.is_err());
        started.elapsed().as_secs_f64()
    }).collect();
    times.sort_by(f64::total_cmp);
    println!("equal keys: {} bytes; min {:.2} ms, median {:.2} ms", same.len(), times[0] * 1e3, times[3] * 1e3);
    // One file at a time, the file stage alone (parse, canonical compare, typed read), each about a megabyte or the
    // node cap: 16,000 numbers, 16,000 distinct keys, one string of a megabyte.
    let one = |name: &str, text: &str| {
        let mut times: Vec<f64> = (0..7).map(|_| {
            let started = Instant::now();
            let verdict = content::bundle::load_file("movement/f00.json", text.as_bytes());
            assert!(verdict.is_err());
            started.elapsed().as_secs_f64()
        }).collect();
        times.sort_by(f64::total_cmp);
        let verdict = content::bundle::load_file("movement/f00.json", text.as_bytes()).map(|_| ()).map_err(|e| e.record());
        println!("file, {name}: {} bytes; median {:.2} ms, {:.1} ns/byte; {verdict:?}", text.len(), times[3] * 1e3, times[3] * 1e9 / text.len() as f64);
    };
    one("16,000 numbers", &numbers[0].1);
    one("16,000 keys", &keyed[0].1);
    one("a 1 MB string", &strings[0].1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("numbers") => numbers(&args[2]),
        Some("loader") => loader(),
        Some("speed") => speed(),
        _ => eprintln!("usage: scratch numbers <out> | scratch loader | scratch speed"),
    }
}
