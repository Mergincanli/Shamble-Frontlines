//! The room's /proc and /sys reads (u6a-brief.md Block 4, S4; plan E18: "steal and affinity reads", per-thread CPU and
//! RSS each second): pure parsers over the files' texts, which the tests feed fixed texts, and the few reads and the
//! pinning that use them. Linux only: on another system every read is empty and every value absent.
//!
//! **Pinning** (Q-U6A-5 (a), with the advisor's terms): `/usr/bin/taskset` at that fixed path, its environment
//! cleared, integer arguments only and no shell, on this process's own thread ids; every pin read back from
//! `/proc/self/task/<tid>/status` and refused unless it holds exactly the asked CPUs.
//!
//! **`alone`** (README §1; the advisor's ADV-U6A-4): the tick thread is alone on its vCPU when no task of any process
//! may run there, but the tick thread and kernel threads bound to that vCPU only (`PF_KTHREAD` in the task's `stat`
//! flags, its allowed list exactly the tick vCPU); an unbound kernel thread counts.

use std::io;
use std::process::{Command, Stdio};

/// `PF_KTHREAD` (include/linux/sched.h): the task is a kernel thread.
const PF_KTHREAD: u64 = 0x0020_0000;
/// The fixed path the room runs taskset from.
const TASKSET: &str = "/usr/bin/taskset";
/// Tasks [`tasks`] reads at most (a bound on a machine's /proc walk).
const TASKS_MAX: usize = 65_536;

/// A Linux CPU list (`0-3,8`) as its CPUs; a part that does not parse is left out, a range at most 4,096 CPUs.
pub(crate) fn cpu_list(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    for part in text.trim().split(',').filter(|p| !p.is_empty()) {
        match part.split_once('-') {
            Some((a, b)) => {
                if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                    out.extend((a..=b).take(4_096));
                }
            }
            None => out.extend(part.parse::<u32>().ok()),
        }
    }
    out
}

/// `second.v.steal_permille`'s reader (C-U6-4: npt-server's half; pub(crate) until U6c's probe needs it elsewhere): the
/// steal time of one vCPU (`/proc/stat`'s `cpu<N>` line, never the all-CPU `cpu` line), as a share of that vCPU's time
/// since the last read, in permille.
pub(crate) struct StealReader {
    cpu: u32,
    last: Option<(u64, u64)>,
}

impl StealReader {
    pub(crate) fn new(cpu: u32) -> StealReader {
        StealReader { cpu, last: None }
    }

    /// The vCPU's (steal, total) ticks in a `/proc/stat` text: its `cpu<N>` line's 8th value and the sum of its first
    /// 8 (user to steal; guest time is inside user already).
    fn ticks(&self, stat: &str) -> Option<(u64, u64)> {
        let name = format!("cpu{}", self.cpu);
        let line = stat
            .lines()
            .find(|l| l.split_whitespace().next() == Some(name.as_str()))?;
        let v: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .take(8)
            .map(|x| x.parse().ok())
            .collect::<Option<_>>()?;
        (v.len() == 8).then(|| (v[7], v.iter().sum()))
    }

    /// Whether `stat` holds the vCPU's line: a read of such a text that gives no share (the first, or no time since the
    /// last) did not fail (FP-13).
    pub(crate) fn has_line(&self, stat: &str) -> bool {
        self.ticks(stat).is_some()
    }

    /// The steal share since the last read, permille: `None` at the first read, for a text without the vCPU's line,
    /// or when no time passed.
    pub(crate) fn read(&mut self, stat: &str) -> Option<u32> {
        let now = self.ticks(stat)?;
        let last = self.last.replace(now)?;
        let total = now.1.checked_sub(last.1).filter(|t| *t > 0)?;
        let steal = now.0.saturating_sub(last.0);
        u32::try_from(steal.saturating_mul(1_000) / total).ok()
    }
}

/// `second.v.rss_bytes`: the resident memory in a `/proc/<pid>/status` text (`VmRSS`, kB), in bytes.
pub(crate) fn rss_bytes(status: &str) -> Option<u64> {
    let kb = status
        .lines()
        .find_map(|l| l.strip_prefix("VmRSS:"))?
        .trim()
        .trim_end_matches("kB")
        .trim()
        .parse::<u64>()
        .ok()?;
    Some(kb.saturating_mul(1_024))
}

/// A task's time on a CPU from its `schedstat` text (the first value, ns).
pub(crate) fn schedstat_ns(text: &str) -> Option<u64> {
    text.split_whitespace().next()?.parse().ok()
}

/// CPU time `cpu_ns` over a wall time `wall_ns`, in permille of one vCPU.
pub(crate) fn permille(cpu_ns: u64, wall_ns: u64) -> u64 {
    if wall_ns == 0 {
        0
    } else {
        u64::try_from(u128::from(cpu_ns) * 1_000 / u128::from(wall_ns)).unwrap_or(u64::MAX)
    }
}

/// One task as the affinity read sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Task {
    pub(crate) tid: u32,
    pub(crate) kernel: bool,
    pub(crate) allowed: Vec<u32>,
}

impl Task {
    /// The task `tid` from its `status` text (`Cpus_allowed_list`) and its `stat` text (the flags, the 9th field: the
    /// 7th after the command's closing parenthesis, which the command itself may hold).
    pub(crate) fn parse(tid: u32, status: &str, stat: &str) -> Option<Task> {
        let allowed = cpu_list(
            status
                .lines()
                .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))?,
        );
        let flags: u64 = stat
            .rsplit_once(')')?
            .1
            .split_whitespace()
            .nth(6)?
            .parse()
            .ok()?;
        Some(Task {
            tid,
            kernel: flags & PF_KTHREAD != 0,
            allowed,
        })
    }
}

/// The tasks that count against the tick thread being alone on `vcpu` (the module text): every task whose allowed
/// list holds it, but the tick thread `tick_tid` and kernel threads bound to `vcpu` alone.
pub(crate) fn not_alone(tasks: &[Task], tick_tid: u32, vcpu: u32) -> Vec<u32> {
    tasks
        .iter()
        .filter(|t| t.tid != tick_tid && t.allowed.contains(&vcpu))
        .filter(|t| !(t.kernel && t.allowed == [vcpu]))
        .map(|t| t.tid)
        .collect()
}

/// `event.affinity.sibling_free`: no I/O runtime thread may run on an SMT sibling of the tick's `vcpu` (`smt`, the
/// header's sibling lists; 1 also on a 1-thread core, which has no sibling).
pub(crate) fn sibling_free(smt: &[Vec<u32>], vcpu: u32, io_allowed: &[Vec<u32>]) -> bool {
    let siblings: Vec<u32> = smt
        .iter()
        .filter(|l| l.contains(&vcpu))
        .flatten()
        .copied()
        .filter(|&c| c != vcpu)
        .collect();
    io_allowed
        .iter()
        .all(|a| !a.iter().any(|c| siblings.contains(c)))
}

/// `event.affinity.alone` from one /proc walk (NR-U6A-3: a partial read is never the best value): the reason when the
/// walk lacks the tick thread or tid 1 or reached [`TASKS_MAX`], else whether [`not_alone`] is empty.
pub(crate) fn alone(tasks: &[Task], tick_tid: u32, vcpu: u32) -> Result<bool, &'static str> {
    if tasks.len() >= TASKS_MAX
        || ![tick_tid, 1]
            .iter()
            .all(|t| tasks.iter().any(|x| x.tid == *t))
    {
        return Err("the /proc walk lacks the tick thread or tid 1, or reached its cap");
    }
    Ok(not_alone(tasks, tick_tid, vcpu).is_empty())
}

/// `event.affinity.sibling_free` from its reads (NR-U6A-3): the reason when the SMT map is empty or an I/O thread's
/// allowed list was not read (`None`), else [`sibling_free`].
pub(crate) fn sibling_free_read(
    smt: &[Vec<u32>],
    vcpu: u32,
    io: &[Option<Vec<u32>>],
) -> Result<bool, &'static str> {
    if smt.is_empty() {
        return Err("no SMT map");
    }
    let io: Vec<Vec<u32>> = io
        .iter()
        .cloned()
        .collect::<Option<_>>()
        .ok_or("an I/O thread's allowed list was not read")?;
    Ok(sibling_free(smt, vcpu, &io))
}

/// Every task of every process under `proc` (`/proc`), at most [`TASKS_MAX`]; a task that ends mid-read is left out.
pub(crate) fn tasks(proc: &str) -> Vec<Task> {
    let mut out = Vec::new();
    let Ok(pids) = std::fs::read_dir(proc) else {
        return out;
    };
    for pid in pids.flatten() {
        let Some(pid) = pid.file_name().to_str().and_then(|p| p.parse::<u32>().ok()) else {
            continue;
        };
        let Ok(tids) = std::fs::read_dir(format!("{proc}/{pid}/task")) else {
            continue;
        };
        for tid in tids.flatten() {
            let Some(tid) = tid.file_name().to_str().and_then(|t| t.parse::<u32>().ok()) else {
                continue;
            };
            let dir = format!("{proc}/{pid}/task/{tid}");
            let (Ok(status), Ok(stat)) = (
                std::fs::read_to_string(format!("{dir}/status")),
                std::fs::read_to_string(format!("{dir}/stat")),
            ) else {
                continue;
            };
            if let Some(t) = Task::parse(tid, &status, &stat) {
                out.push(t);
            }
            if out.len() >= TASKS_MAX {
                return out;
            }
        }
    }
    out
}

/// The calling thread's id (`/proc/thread-self` names `<pid>/task/<tid>`).
pub(crate) fn thread_tid() -> Option<u32> {
    std::fs::read_link("/proc/thread-self")
        .ok()?
        .file_name()?
        .to_str()?
        .parse()
        .ok()
}

/// The allowed CPUs of this process's thread `tid`, read back from /proc.
pub(crate) fn allowed(tid: u32) -> Option<Vec<u32>> {
    allowed_in("/proc/self/task", tid)
}

/// The allowed CPUs of thread `tid` under `task_dir` (`/proc/self/task`).
fn allowed_in(task_dir: &str, tid: u32) -> Option<Vec<u32>> {
    let status = std::fs::read_to_string(format!("{task_dir}/{tid}/status")).ok()?;
    status
        .lines()
        .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))
        .map(cpu_list)
}

/// The I/O runtime's threads' allowed lists for `event.affinity.sibling_free` (NR2-U6A-2): a thread no longer under
/// `task_dir` (`/proc/self/task`) has exited (a blocking-pool thread after its idle time, U6b's transports' too), so
/// it is left out; a live one whose list cannot be read is `None`, a failed read.
pub(crate) fn io_allowed(task_dir: &str, tids: &[u32]) -> Vec<Option<Vec<u32>>> {
    let live = |t: u32| std::path::Path::new(&format!("{task_dir}/{t}")).exists();
    tids.iter()
        .filter_map(|&t| match allowed_in(task_dir, t) {
            Some(list) => Some(Some(list)),
            None if !live(t) => None,
            None => Some(None),
        })
        .collect()
}

/// Pins this process's thread `tid` to `cpus` by taskset (the module text), then reads the pin back.
///
/// # Errors
/// No CPU asked, taskset could not run or failed, or the read-back is not exactly `cpus`.
pub(crate) fn pin(tid: u32, cpus: &[u32]) -> io::Result<()> {
    let mut want: Vec<u32> = cpus.to_vec();
    want.sort_unstable();
    want.dedup();
    if want.is_empty() {
        return Err(io::Error::other("a pin to no CPU"));
    }
    let list = want
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let status = Command::new(TASKSET)
        .env_clear()
        .arg("-p")
        .arg("-c")
        .arg(&list)
        .arg(tid.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "taskset could not pin thread {tid} to {list} ({status})"
        )));
    }
    match allowed(tid) {
        Some(got) if got == want => Ok(()),
        got => Err(io::Error::other(format!(
            "thread {tid}'s pin reads back as {got:?}, not {want:?}"
        ))),
    }
}

/// This process's CPU time so far (every thread's `schedstat`), and the part of it the threads `tids` ran, ns; `None`
/// when /proc/self/task or no thread's `schedstat` could be read (NR-U6A-4: never a 0 for a failed read).
pub(crate) fn cpu_ns(tids: &[u32]) -> Option<(u64, u64)> {
    let (mut all, mut some, mut read) = (0u64, 0u64, false);
    let dir = std::fs::read_dir("/proc/self/task").ok()?;
    for t in dir.flatten() {
        let Some(tid) = t.file_name().to_str().and_then(|x| x.parse::<u32>().ok()) else {
            continue;
        };
        // A thread that ended mid-read is left out.
        let Some(ns) = std::fs::read_to_string(format!("/proc/self/task/{tid}/schedstat"))
            .ok()
            .and_then(|s| schedstat_ns(&s))
        else {
            continue;
        };
        read = true;
        all = all.saturating_add(ns);
        if tids.contains(&tid) {
            some = some.saturating_add(ns);
        }
    }
    read.then_some((all, some))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `/proc/stat` text where only `cpu3`'s steal moves: the all-CPU line's share would be a quarter of it.
    fn stat(steal3: u64, idle: u64) -> String {
        format!(
            "cpu  400 0 40 {} 0 0 0 {steal3} 0 0\ncpu0 100 0 10 {idle} 0 0 0 0 0 0\ncpu1 100 0 10 {idle} 0 0 0 0 0 0\n\
             cpu2 100 0 10 {idle} 0 0 0 0 0 0\ncpu3 100 0 10 {idle} 0 0 0 {steal3} 0 0\nintr 1 2 3\n",
            4 * idle
        )
    }

    #[test]
    fn s4_the_steal_reader_reads_the_tick_vcpus_line_only() {
        let mut r = StealReader::new(3);
        assert_eq!(
            r.read(&stat(0, 1_000)),
            None,
            "the first read sets the baseline"
        );
        // cpu3: 100 more steal ticks and 900 more idle ticks: 100 of 1,000.
        assert_eq!(r.read(&stat(100, 1_900)), Some(100));
        assert_eq!(r.read(&stat(100, 1_900)), None, "no time passed");
        assert_eq!(r.read(&stat(150, 2_850)), Some(50), "50 of the next 1,000");
        let mut other = StealReader::new(0);
        other.read(&stat(0, 1_000));
        assert_eq!(
            other.read(&stat(100, 1_900)),
            Some(0),
            "vCPU 0 had no steal"
        );
        let mut missing = StealReader::new(9);
        assert_eq!(missing.read(&stat(0, 1)), None, "no cpu9 line");
        assert_eq!(missing.read("cpu9 1 2 3\n"), None, "a short line");
    }

    #[test]
    fn s4_rss_and_schedstat_read_their_texts() {
        let status = "Name:\tnpt-server\nVmPeak:\t  9000 kB\nVmRSS:\t   51200 kB\nThreads:\t7\n";
        assert_eq!(rss_bytes(status), Some(51_200 * 1_024));
        assert_eq!(rss_bytes("Name:\tx\n"), None);
        assert_eq!(schedstat_ns("123456789 1000 5\n"), Some(123_456_789));
        assert_eq!(schedstat_ns(""), None);
        assert_eq!(
            permille(250_000_000, 1_000_000_000),
            250,
            "a quarter of one vCPU"
        );
        assert_eq!(
            permille(2_000_000_000, 1_000_000_000),
            2_000,
            "two vCPUs' worth"
        );
        assert_eq!(permille(5, 0), 0);
        assert_eq!(cpu_list("0-3,8\n"), [0, 1, 2, 3, 8]);
    }

    fn task(tid: u32, kernel: bool, allowed: &str) -> Task {
        let flags = if kernel { PF_KTHREAD | 0x40 } else { 0x40_0040 };
        let status = format!("Name:\tt\nCpus_allowed:\tf\nCpus_allowed_list:\t{allowed}\n");
        // The command holds a space and a parenthesis: the flags are read after its last ')'.
        let stat =
            format!("{tid} (odd (name) x) S 2 0 0 0 -1 {flags} 0 0 0 0 1 1 0 0 20 0 1 0 5 0 0");
        Task::parse(tid, &status, &stat).expect("parses")
    }

    /// ADV-U6A-4's fixtures: a per-CPU kernel thread on the tick vCPU is left out; an unbound kernel thread and a user
    /// thread that may run there count; a thread kept off it does not.
    #[test]
    fn s4_alone_leaves_out_only_the_tick_thread_and_kernel_threads_bound_to_its_vcpu() {
        let tick = task(10, false, "3");
        assert!(!tick.kernel && tick.allowed == [3]);
        let percpu = task(20, true, "3");
        let unbound = task(21, true, "0-3");
        let user = task(30, false, "0-3");
        let off = task(31, false, "0-2");
        assert!(percpu.kernel && unbound.kernel && !user.kernel);
        assert!(
            not_alone(&[tick.clone(), percpu.clone(), off.clone()], 10, 3).is_empty(),
            "alone"
        );
        assert_eq!(
            not_alone(&[tick.clone(), percpu.clone(), unbound, off.clone()], 10, 3),
            [21]
        );
        assert_eq!(not_alone(&[tick, percpu, user, off], 10, 3), [30]);
    }

    #[test]
    fn s4_sibling_free_reads_fixed_smt_maps() {
        // SMT: 2 threads a core, vCPU 3's sibling is 7.
        let smt = vec![vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]];
        assert!(sibling_free(&smt, 3, &[vec![0, 1, 2], vec![4, 5, 6]]));
        assert!(
            !sibling_free(&smt, 3, &[vec![0, 1, 2], vec![6, 7]]),
            "an I/O thread may run on 7"
        );
        let single = vec![vec![0], vec![1], vec![2], vec![3]];
        assert!(
            sibling_free(&single, 3, &[vec![0, 1, 2, 3]]),
            "1 on a 1-thread core"
        );
    }

    /// NR-U6A-3, TR-U6A-9: a failed or partial read is never the best value. `alone` needs a walk holding the tick
    /// thread and tid 1; `sibling_free` needs the SMT map and every I/O thread's allowed list.
    #[test]
    fn s4_a_partial_walk_or_a_failed_read_is_never_alone_or_sibling_free() {
        let (init, tick, off) = (
            task(1, false, "0-2"),
            task(10, false, "3"),
            task(31, false, "0-2"),
        );
        assert_eq!(
            alone(&[init.clone(), tick.clone(), off.clone()], 10, 3),
            Ok(true)
        );
        assert!(alone(&[], 10, 3).is_err(), "an empty walk");
        assert!(alone(&[tick, off], 10, 3).is_err(), "a walk without tid 1");
        assert!(
            alone(&[init], 10, 3).is_err(),
            "a walk without the tick thread"
        );
        let smt = vec![vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]];
        assert_eq!(sibling_free_read(&smt, 3, &[Some(vec![0, 1])]), Ok(true));
        assert!(
            sibling_free_read(&[], 3, &[Some(vec![0, 1])]).is_err(),
            "an empty SMT map"
        );
        assert!(
            sibling_free_read(&smt, 3, &[Some(vec![0, 1]), None]).is_err(),
            "an I/O thread's list not read"
        );
    }

    /// NR2-U6A-2: an I/O thread that has exited is left out of `sibling_free`'s read (no failed read, so no 0 for it);
    /// a live one is read; a live tid under a folder that cannot be read stays a failed read.
    #[test]
    fn u6b_an_exited_io_thread_is_left_out_of_the_sibling_read() {
        let dead = std::thread::spawn(|| thread_tid().expect("its tid"))
            .join()
            .expect("the thread");
        // `join` returns at the kernel's clear of the thread's tid futex, a little before its /proc entry goes
        // (check run 2 read it there once): wait for the entry to go, at most 5,000 waits of 1 ms (no clock read).
        let entry = format!("/proc/self/task/{dead}");
        let mut waits = 0;
        while std::path::Path::new(&entry).exists() {
            waits += 1;
            assert!(waits <= 5_000, "{entry} still there");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let me = thread_tid().expect("this thread's tid");
        let got = io_allowed("/proc/self/task", &[dead, me]);
        assert_eq!(got.len(), 1, "the exited thread is left out: {got:?}");
        assert!(got[0].as_ref().is_some_and(|l| !l.is_empty()));
        let smt = vec![vec![0, 4], vec![1, 5]];
        assert!(sibling_free_read(&smt, 0, &got).is_ok(), "no failed read");
        let unreadable = std::env::temp_dir().join(format!("npt-u6b-task-{}", std::process::id()));
        std::fs::create_dir_all(unreadable.join(me.to_string())).expect("a folder");
        let dir = unreadable.to_str().expect("utf-8");
        assert_eq!(io_allowed(dir, &[me]), vec![None], "live but unread");
        std::fs::remove_dir_all(&unreadable).expect("removed");
    }
}
