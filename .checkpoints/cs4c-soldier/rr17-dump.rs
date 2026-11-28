// rust-reviewer block 17 scratch: libm 0.2.16's exp and sincos on the input files, outputs as f64 LE. Not product code.
fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let read = |name: &str| -> Vec<f64> {
        std::fs::read(format!("{dir}/{name}")).unwrap().chunks(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect()
    };
    let mut out = Vec::new();
    for x in read("exp-in.bin") {
        out.extend_from_slice(&libm::exp(x).to_le_bytes());
    }
    std::fs::write(format!("{dir}/exp-native.bin"), &out).unwrap();
    out.clear();
    for x in read("sincos-in.bin") {
        let (s, c) = libm::sincos(x);
        out.extend_from_slice(&s.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
    }
    std::fs::write(format!("{dir}/sincos-native.bin"), &out).unwrap();
}
