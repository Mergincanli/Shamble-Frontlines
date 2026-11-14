fn main() {
    let shift: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(8);
    let (h, n) = u4s_xtarget::run(shift);
    println!("native shift {shift}: hash {h:016x} over {n} values");
}
