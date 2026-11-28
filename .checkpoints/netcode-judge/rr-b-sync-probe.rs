// rust-reviewer part b: does sync_cost_ratio's exact arithmetic (figures.rs:1282-1294) fit i128 at real sizes?
// Uses the crate's own ratio.rs unchanged. Build: rustc -O --edition 2021 -o <out> this-file
#[allow(dead_code)]
#[path = "../../spikes/netcode-proof/crates/judge/src/ratio.rs"]
mod ratio;
use ratio::Ratio;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, lo: i64, hi: i64) -> i64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        lo + ((self.0 >> 33) as i64) % (hi - lo + 1)
    }
}

// sync_unit: sum of server_only_ns over measured ticks / sum of every relevant_count entry.
fn unit(rng: &mut Lcg, ticks: usize, clients: usize, rel: (i64, i64), ns: (i64, i64)) -> Ratio {
    let (mut n, mut e) = (0i128, 0i128);
    for _ in 0..ticks {
        n += rng.next(ns.0, ns.1) as i128;
        for _ in 0..clients {
            e += rng.next(rel.0, rel.1) as i128;
        }
    }
    Ratio::new(n, e)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ticks: usize = args.get(1).map_or(54_000, |s| s.parse().unwrap());
    std::panic::set_hook(Box::new(|_| {}));
    for k in 3..=6 {
        let r = std::panic::catch_unwind(|| {
            let mut rng = Lcg(42 + k as u64);
            let own = unit(&mut rng, 54_000, 48, (40, 50), (1_000_000, 3_000_000));
            let mut worst: Option<Ratio> = None;
            for (clients, rel) in [(24usize, (20i64, 26i64)), (36, (30, 38))] {
                let mut sum = Ratio::int(0);
                for _ in 0..k {
                    sum = sum + unit(&mut rng, ticks, clients, rel, (500_000, 2_000_000));
                }
                let ratio = own * Ratio::new(sum.den(), sum.num()) * Ratio::int(k as i128);
                worst = Some(worst.map_or(ratio, |w| w.max(ratio)));
            }
            let w = worst.unwrap();
            (w.to_f64(), w.num().unsigned_abs().ilog10(), w.den().ilog10())
        });
        match r {
            Ok((v, n, d)) => println!("ticks {ticks}, {k} runs per part: ok, ratio {v:.4} (num ~1e{n}, den ~1e{d})"),
            Err(_) => println!("ticks {ticks}, {k} runs per part: PANIC (a fraction overflowed i128)"),
        }
    }
}
