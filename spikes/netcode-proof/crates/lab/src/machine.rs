//! The header's machine and build for a run in this container (machine role `C`; plan §5, §6.1), read from Linux's
//! `/proc` and `/sys` and the binary itself. A value the machine does not offer stays empty or 0, and the judge's
//! machine figure (U4) names what is missing.

use npt_log::{Build, Machine};
use sha2::{Digest, Sha256};

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// `0-3,8` -> [0, 1, 2, 3, 8].
fn cpu_list(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    for part in text.trim().split(',').filter(|p| !p.is_empty()) {
        match part.split_once('-') {
            Some((a, b)) => {
                if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                    out.extend(a..=b);
                }
            }
            None => out.extend(part.parse::<u32>().ok()),
        }
    }
    out
}

/// This container, as the header's `machine`. L1 runs on simulated time, so its clock resolution is the simulated
/// clock's: 1 ns.
pub fn container() -> Machine {
    let cpuinfo = read("/proc/cpuinfo");
    let cpu = cpuinfo
        .lines()
        .find_map(|l| l.strip_prefix("model name").and_then(|r| r.split_once(':')))
        .map(|(_, v)| v.trim().to_owned())
        .unwrap_or_default();
    let mem_kb = read("/proc/meminfo")
        .lines()
        .find_map(|l| l.strip_prefix("MemTotal:"))
        .and_then(|v| v.trim().trim_end_matches("kB").trim().parse::<u64>().ok())
        .unwrap_or(0);
    let vcpus =
        std::thread::available_parallelism().map_or(0, |n| u32::try_from(n.get()).unwrap_or(0));
    let mut smt: Vec<Vec<u32>> = Vec::new();
    for cpu in 0..vcpus {
        let siblings = cpu_list(&read(&format!(
            "/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list"
        )));
        if !siblings.is_empty() && !smt.contains(&siblings) {
            smt.push(siblings);
        }
    }
    let threads_per_core = smt
        .iter()
        .map(|s| u32::try_from(s.len()).unwrap_or(0))
        .max()
        .unwrap_or(0);
    Machine {
        role: "C".to_owned(),
        name: "C".to_owned(),
        plan: String::new(),
        arch: std::env::consts::ARCH.to_owned(),
        cpu,
        vcpus,
        mem_mb: mem_kb / 1024,
        location: String::new(),
        kernel: read("/proc/sys/kernel/osrelease").trim().to_owned(),
        address: String::new(),
        threads_per_core,
        smt,
        clock_res_ns: 1,
    }
}

/// The running binary, as the header's `build`: the commit the caller names, the binary's SHA-256, its features and
/// its profile.
pub fn build(commit: &str) -> Result<Build, String> {
    if commit.len() != 40
        || !commit
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(format!(
            "--commit {commit:?} is not a 40-digit lowercase hex commit"
        ));
    }
    let exe = std::env::current_exe().map_err(|e| format!("the binary's path: {e}"))?;
    let bytes = std::fs::read(&exe).map_err(|e| format!("{}: {e}", exe.display()))?;
    let binary_sha256 = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let mut features = Vec::new();
    if cfg!(feature = "bites") {
        features.push("bites".to_owned());
    }
    Ok(Build {
        commit: commit.to_owned(),
        binary_sha256,
        features,
        profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
        .to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_lists_read_ranges_and_singles() {
        assert_eq!(cpu_list("0-3,8\n"), vec![0, 1, 2, 3, 8]);
        assert_eq!(cpu_list("5"), vec![5]);
        assert_eq!(cpu_list(""), Vec::<u32>::new());
    }

    #[test]
    fn a_build_needs_a_full_commit_and_hashes_its_own_binary() {
        assert!(build("abc").is_err());
        assert!(build(&"A".repeat(40)).is_err(), "lowercase only");
        let b = build(&"0".repeat(40)).unwrap();
        assert_eq!(b.binary_sha256.len(), 64);
        assert_eq!(b.features.is_empty(), !cfg!(feature = "bites"));
    }

    #[test]
    fn the_container_names_itself_c_with_a_simulated_clock() {
        let m = container();
        assert_eq!(
            (m.role.as_str(), m.name.as_str(), m.clock_res_ns),
            ("C", "C", 1)
        );
        assert!(
            m.address.is_empty(),
            "never an address outside a VM (plan §10.6)"
        );
        assert!(!m.arch.is_empty());
    }
}
