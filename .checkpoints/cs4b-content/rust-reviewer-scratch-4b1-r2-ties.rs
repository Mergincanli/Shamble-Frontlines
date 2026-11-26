// rust-reviewer's round-2 scratch (not part of any build): how many lines of a number corpus are real ties, where
// the canonical text's digits differ from the standard library's shortest digits (the even rule changed them).
// Usage: ties <corpus file>...
fn std_digits(x: f64) -> String {
    let s = format!("{:e}", x.abs());
    let m = s.split('e').next().unwrap_or("");
    m.chars().filter(char::is_ascii_digit).collect()
}
fn main() {
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).expect("a corpus file");
        let (mut lines, mut ties, mut small, mut negative_ties) = (0, 0, 0, 0);
        let mut examples = Vec::new();
        for line in text.lines().skip(1) {
            let mut parts = line.split(' ');
            let (Some(bits), Some(_), Some(canonical)) = (parts.next(), parts.next(), parts.next()) else { continue };
            let x = f64::from_bits(u64::from_str_radix(bits, 16).expect("hex"));
            lines += 1;
            if !x.is_finite() || x == 0.0 { continue; }
            let mantissa = canonical.trim_start_matches('-').split('e').next().unwrap_or("");
            let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect::<String>();
            let digits = digits.trim_start_matches('0').trim_end_matches('0');
            if digits != std_digits(x) {
                ties += 1;
                if x.abs() < 1.0 { small += 1; }
                if x < 0.0 { negative_ties += 1; }
                if examples.len() < 3 && x.abs() < 1.0 { examples.push(format!("{bits} {canonical}")); }
            }
        }
        println!("{path}: {lines} numbers, {ties} ties (the even rule changed std's digits), of which |x| < 1: {small}, negative: {negative_ties}; small examples {examples:?}");
    }
}
