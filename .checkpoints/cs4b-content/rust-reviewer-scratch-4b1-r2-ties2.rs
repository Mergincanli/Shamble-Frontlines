// rust-reviewer's round-2 scratch (not part of any build): real ties per section of the number corpus, and for the
// "ties" section the binary places of each line (after the trailing zeros) against whether it is a real tie.
// Usage: ties2 <corpus file> <first line of the ties section> <its line count> (line numbers from 1, header = 1)
fn std_digits(x: f64) -> String {
    let s = format!("{:e}", x.abs());
    s.split('e').next().unwrap_or("").chars().filter(char::is_ascii_digit).collect()
}
fn places(x: f64) -> i32 {
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & 0x000f_ffff_ffff_ffff;
    let (mantissa, power) = if exponent == 0 { (fraction, -1074) } else { (fraction | (1 << 52), exponent - 1075) };
    -(power + mantissa.trailing_zeros() as i32)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("a corpus file");
    let (first, count): (usize, usize) = (args[2].parse().unwrap(), args[3].parse().unwrap());
    let mut by_places = std::collections::BTreeMap::<i32, (u32, u32)>::new();
    let (mut inside, mut outside) = (0, 0);
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if number == 1 { continue; }
        let mut parts = line.split(' ');
        let (Some(bits), Some(_), Some(canonical)) = (parts.next(), parts.next(), parts.next()) else { continue };
        let x = f64::from_bits(u64::from_str_radix(bits, 16).expect("hex"));
        if !x.is_finite() || x == 0.0 { continue; }
        let mantissa = canonical.trim_start_matches('-').split('e').next().unwrap_or("");
        let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect::<String>();
        let tie = digits.trim_start_matches('0').trim_end_matches('0') != std_digits(x);
        let in_section = number >= first && number < first + count;
        if in_section {
            let entry = by_places.entry(places(x)).or_default();
            entry.0 += 1;
            if tie { entry.1 += 1; inside += 1; }
        } else if tie {
            outside += 1;
        }
    }
    println!("{}: ties section lines {first}..{}: {inside} real ties; elsewhere {outside}", args[1], first + count - 1);
    println!("  binary places -> (lines, real ties): {by_places:?}");
}
