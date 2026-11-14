//! The release thread's own settings and readings, through /proc only: no unsafe code and no libc (M14; Mustafa's
//! U3-Q2 (b)).
//!
//! - **Timer slack:** a thread lowers its own slack by writing `/proc/<tid>/timerslack_ns`, the tid read from the
//!   `/proc/thread-self` link (`/proc/thread-self/timerslack_ns` does not exist: Linux registers the file for
//!   processes only, fs/proc/base.c tgid_base_stuff; the capability check runs only when p != current). It is read
//!   back and recorded, and read again by the thread itself whenever the header is built (another thread would need
//!   CAP_SYS_NICE to read it). A thread under a real-time policy reads 0: since kernel commit ed4fb6d7ef68 (mainline
//!   6.12, backported to 6.6.84) the kernel forces an RT task's slack to 0, and a return to SCHED_OTHER restores the
//!   default (50 µs), not the value written before. Whether RS's Ubuntu 6.8 kernel carries it is read at R1 (a kernel
//!   without it keeps the written value under SCHED_FIFO).
//! - **Real-time priority** is a recorded switch applied from outside (`chrt -f -p <prio> <tid>`, or the whole
//!   process under `chrt`): the thread's policy and priority are read from `/proc/self/task/<tid>/stat` (fields 41
//!   and 40) whenever the header is built. No spin lead (U3-Q2).
//! - **CPU:** `/proc/self/task/<tid>/schedstat`'s first field, the ns the thread ran (M13).

use std::io;

/// This thread's id, from the `/proc/thread-self` link (`<pid>/task/<tid>`).
pub(crate) fn own_tid() -> io::Result<u32> {
    let link = std::fs::read_link("/proc/thread-self")?;
    link.file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no tid in /proc/thread-self"))
}

/// What the thread found when it set its slack.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Slack {
    /// The write's error, if it failed.
    pub(crate) write_error: Option<String>,
    /// The value read back, if readable.
    pub(crate) read_ns: Option<u64>,
}

/// Writes `ns` to this thread's own timer slack (0: leaves the default, only reads), then reads it back. Called by
/// the thread itself.
pub(crate) fn set_own_slack(tid: u32, ns: u64) -> Slack {
    let path = format!("/proc/{tid}/timerslack_ns");
    let write_error = if ns == 0 {
        None
    } else {
        std::fs::write(&path, ns.to_string())
            .err()
            .map(|e| e.to_string())
    };
    let read_ns = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse().ok());
    Slack {
        write_error,
        read_ns,
    }
}

/// The thread's scheduling policy and real-time priority, from its stat line.
pub(crate) fn sched(tid: u32) -> Option<(u32, u32)> {
    let stat = std::fs::read_to_string(format!("/proc/self/task/{tid}/stat")).ok()?;
    parse_sched(&stat)
}

/// Fields after the command's closing parenthesis start at field 3: rt_priority is field 40, policy field 41.
pub(crate) fn parse_sched(stat: &str) -> Option<(u32, u32)> {
    let rest = &stat[stat.rfind(')')? + 1..];
    let fields: Vec<&str> = rest.split_whitespace().collect();
    let rt = fields.get(37)?.parse().ok()?;
    let policy = fields.get(38)?.parse().ok()?;
    Some((policy, rt))
}

/// The policy's name (include/uapi/linux/sched.h).
pub(crate) fn policy_name(policy: u32) -> &'static str {
    match policy {
        0 => "SCHED_OTHER",
        1 => "SCHED_FIFO",
        2 => "SCHED_RR",
        3 => "SCHED_BATCH",
        5 => "SCHED_IDLE",
        6 => "SCHED_DEADLINE",
        _ => "SCHED_UNKNOWN",
    }
}

/// The ns the thread has run, from its schedstat.
pub(crate) fn cpu_ns(tid: u32) -> Option<u64> {
    let s = std::fs::read_to_string(format!("/proc/self/task/{tid}/schedstat")).ok()?;
    s.split_whitespace().next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// M14: the thread writes 1 ns to its own slack and reads 1 back, with no capability and no unsafe code. Under a
    /// real-time policy (the whole test run under `chrt -f`) both the default and the value read back are 0: kernel
    /// commit ed4fb6d7ef68 (mainline 6.12, backported to 6.6.84) forces an RT task's slack to 0. Whether RS's Ubuntu
    /// 6.8 kernel carries it is read at R1: a kernel without it fails this test under `chrt` (it keeps 1 and 50 µs).
    /// The bite: `/proc/thread-self/timerslack_ns`, the scout's path, does not exist.
    #[test]
    fn a_thread_sets_its_own_slack_to_1_ns() {
        let (slack, sched_now, default) = std::thread::spawn(|| {
            let tid = own_tid().expect("a tid");
            let default = set_own_slack(tid, 0).read_ns;
            (set_own_slack(tid, 1), sched(tid), default)
        })
        .join()
        .unwrap();
        assert_eq!(slack.write_error, None);
        let (policy, _) = sched_now.expect("the stat line parses");
        let rt = policy == 1 || policy == 2;
        assert_eq!(
            slack.read_ns,
            Some(if rt { 0 } else { 1 }),
            "policy {}",
            policy_name(policy)
        );
        if rt {
            assert_eq!(default, Some(0), "an RT thread's default slack");
        } else {
            assert!(
                default.is_some_and(|d| d > 1),
                "the default slack ({default:?}) is above 1 ns"
            );
        }
        assert!(
            std::fs::metadata("/proc/thread-self/timerslack_ns").is_err(),
            "the scout's path exists after all"
        );
        // The main thread's slack is untouched: only the writing thread changed.
        let main = set_own_slack(own_tid().unwrap(), 0).read_ns;
        assert_ne!(main, Some(1));
    }

    #[test]
    fn the_stat_line_gives_policy_and_priority() {
        // A real line's shape, with a command holding spaces and a parenthesis.
        let mut line = String::from("4242 (npt l2) rel) S");
        for i in 4..=52 {
            let v = match i {
                40 => "50".to_owned(),
                41 => "1".to_owned(),
                _ => i.to_string(),
            };
            line.push(' ');
            line.push_str(&v);
        }
        assert_eq!(parse_sched(&line), Some((1, 50)));
        assert_eq!(parse_sched("1 (x) S 2 3"), None);
        assert_eq!(policy_name(1), "SCHED_FIFO");
        let tid = own_tid().unwrap();
        assert!(sched(tid).is_some());
        assert!(cpu_ns(tid).is_some());
    }
}
