//! DelayStream's release clock (u3c-plan.md 1.2, C3; M14's pattern): one process-wide dedicated thread with 1 ns
//! timer slack, written by the thread itself and read back (l2::thread, no unsafe code, no libc), that wakes each
//! connection's pump at its model's next event. No tokio timer is on the release path (tokio's resolution is 1 ms).
//!
//! - The clock's origin is the first use in the process; `now_ns` is DelayStream's clock, the one every stamp
//!   DelayStream takes or reports is on (C5).
//! - The schedule is a set of (due, sequence, key) ordered by due time, beside a table key -> (due, sequence, waker):
//!   a connection holds at most one entry, replaced when its due time changes and removed when it fires, when it is
//!   cancelled, or when the connection's pump ends (C3: both hold at most one entry per live connection).
//! - The thread pops every entry due, wakes the pumps outside the lock, and otherwise sleeps on a Condvar until the
//!   earliest due time (a nearer new entry wakes it).
//! - The per-release split (the advisor's "condition 10 again", 2026-11-08): when it fires a connection's entry, the
//!   thread keeps (the entry's due, its own `now` then) in `fired`, the split's stamp (1): `now` is read at the top
//!   of its loop, the first stamp after its Condvar wait returns (and again after each batch it wakes). The pump takes
//!   it at its next pass (`take_fired`). At most one entry per live connection, overwritten by a later fire unless the
//!   one there is untaken and for the same due (its first wake is kept), removed at `cancel` and `release`.

use super::thread;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::sync::{Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::task::Waker;
use std::time::{Duration, Instant};

/// Wakers fired per lock.
const BATCH: usize = 64;
/// How long a header waits for the clock thread's own slack reading.
const SLACK_ANSWER: Duration = Duration::from_secs(1);

#[derive(Debug, Default)]
struct State {
    schedule: BTreeSet<(u64, u64, u64)>,
    table: BTreeMap<u64, (u64, u64, Waker)>,
    /// key -> (the fired entry's due, the thread's `now` when it fired it, taken by the pump).
    fired: BTreeMap<u64, (u64, u64, bool)>,
    seq: u64,
    next_key: u64,
    /// Keys handed out and not yet released: the live connections.
    live: u64,
    /// The thread sleeps until this (u64::MAX: no deadline; 0: awake).
    wait_until: u64,
    started: bool,
    slack_ask: u64,
}

/// What the thread found about itself.
#[derive(Debug, Default, Clone)]
pub(crate) struct Info {
    pub(crate) tid: Option<u32>,
    pub(crate) slack: thread::Slack,
    /// (the request answered, the slack read then).
    slack_now: (u64, Option<u64>),
}

#[derive(Debug)]
pub(crate) struct Clock {
    epoch: Instant,
    state: Mutex<State>,
    cv: Condvar,
    info: Mutex<Info>,
    info_cv: Condvar,
}

static CLOCK: OnceLock<Clock> = OnceLock::new();

/// The process's clock (its origin set at the first call; the thread starts at [`Clock::start`]).
pub(crate) fn clock() -> &'static Clock {
    CLOCK.get_or_init(|| Clock {
        epoch: Instant::now(),
        state: Mutex::new(State {
            wait_until: u64::MAX,
            ..State::default()
        }),
        cv: Condvar::new(),
        info: Mutex::new(Info::default()),
        info_cv: Condvar::new(),
    })
}

impl Clock {
    /// ns since the clock's origin.
    pub(crate) fn now_ns(&self) -> u64 {
        self.epoch.elapsed().as_nanos() as u64
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Starts the thread once per process, with `slack_ns` (the first connection's `timer_slack_ns`; 0 leaves the
    /// kernel's default), and hands out a key for one connection.
    pub(crate) fn start(&'static self, slack_ns: u64) -> io::Result<u64> {
        let mut st = self.lock();
        if !st.started {
            std::thread::Builder::new()
                .name("npt-delay-clock".into())
                .spawn(move || self.run(slack_ns))?;
            st.started = true;
        }
        st.next_key += 1;
        st.live += 1;
        Ok(st.next_key)
    }

    /// Sets `key`'s one entry to fire at `due` (clock ns) with `waker`, replacing any earlier entry.
    pub(crate) fn schedule(&self, key: u64, due: u64, waker: &Waker) {
        let mut st = self.lock();
        let w = match st.table.remove(&key) {
            Some((d, s, w)) => {
                st.schedule.remove(&(d, s, key));
                if w.will_wake(waker) { w } else { waker.clone() }
            }
            None => waker.clone(),
        };
        st.seq += 1;
        let seq = st.seq;
        st.schedule.insert((due, seq, key));
        st.table.insert(key, (due, seq, w));
        let wake = due < st.wait_until;
        drop(st);
        if wake {
            self.cv.notify_one();
        }
    }

    /// Removes `key`'s entry, if any, and its fire. A fire not yet taken covers nothing later: a connection cancels
    /// only with no event pending (a record released after it comes from an event added later, so is due after the
    /// fire's stamp) or when its DelayStream is dropped.
    pub(crate) fn cancel(&self, key: u64) {
        let mut st = self.lock();
        if let Some((d, s, _)) = st.table.remove(&key) {
            st.schedule.remove(&(d, s, key));
        }
        st.fired.remove(&key);
    }

    /// Removes `key`'s entry and its fire, and gives the key back (its connection's pump ended).
    pub(crate) fn release(&self, key: u64) {
        let mut st = self.lock();
        if let Some((d, s, _)) = st.table.remove(&key) {
            st.schedule.remove(&(d, s, key));
        }
        st.fired.remove(&key);
        st.live = st.live.saturating_sub(1);
    }

    /// `key`'s latest fire not yet taken, if the thread stamped it at or before `upto` (the pump pass's own stamp):
    /// (the entry's due, the thread's `now` then), marked taken. A fire stamped after `upto` is left for the next pass.
    pub(crate) fn take_fired(&self, key: u64, upto: u64) -> Option<(u64, u64)> {
        let mut st = self.lock();
        match st.fired.get_mut(&key) {
            Some(f) if !f.2 && f.1 <= upto => {
                f.2 = true;
                Some((f.0, f.1))
            }
            _ => None,
        }
    }

    /// (schedule entries, table entries, live connections), and whether `key` holds an entry in any of the schedule,
    /// the table and the fires (C3's test).
    #[cfg(test)]
    pub(crate) fn census(&self, key: u64) -> (usize, usize, u64, bool) {
        let st = self.lock();
        let held = st.table.contains_key(&key)
            || st.schedule.iter().any(|e| e.2 == key)
            || st.fired.contains_key(&key);
        (st.schedule.len(), st.table.len(), st.live, held)
    }

    /// The thread's id and its slack as set at its start.
    pub(crate) fn info(&self) -> Info {
        self.info
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Asks the thread to read its own timer slack now (only the thread itself may read it without CAP_SYS_NICE) and
    /// waits up to 1 s for the answer; None if unreadable or unanswered.
    pub(crate) fn slack_now(&self) -> Option<u64> {
        let ask = {
            let mut st = self.lock();
            if !st.started {
                return None;
            }
            st.slack_ask += 1;
            st.slack_ask
        };
        self.cv.notify_one();
        let end = Instant::now() + SLACK_ANSWER;
        let mut info = self.info.lock().unwrap_or_else(PoisonError::into_inner);
        while info.slack_now.0 < ask {
            let left = end.checked_duration_since(Instant::now())?;
            info = self
                .info_cv
                .wait_timeout(info, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        info.slack_now.1
    }

    fn run(&'static self, slack_ns: u64) {
        let tid = thread::own_tid().ok();
        let slack = tid.map_or_else(thread::Slack::default, |t| {
            thread::set_own_slack(t, slack_ns)
        });
        {
            let mut info = self.info.lock().unwrap_or_else(PoisonError::into_inner);
            info.tid = tid;
            info.slack = slack;
        }
        let mut fired: Vec<Waker> = Vec::with_capacity(BATCH);
        let mut answered = 0u64;
        let mut st = self.lock();
        loop {
            if st.slack_ask != answered {
                // The /proc read runs with the schedule's lock released (F6), so no pump's schedule() waits on it.
                answered = st.slack_ask;
                drop(st);
                let read = tid.and_then(|t| thread::set_own_slack(t, 0).read_ns);
                self.info
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .slack_now = (answered, read);
                self.info_cv.notify_all();
                st = self.lock();
                continue;
            }
            let now = self.now_ns();
            while fired.len() < BATCH {
                match st.schedule.first() {
                    Some(&(due, _, key)) if due <= now => {
                        st.schedule.pop_first();
                        if let Some((_, _, w)) = st.table.remove(&key) {
                            fired.push(w);
                            // The split's stamp (1); an untaken fire for the same due keeps its first wake.
                            let keep = st.fired.get(&key).is_some_and(|f| !f.2 && f.0 == due);
                            if !keep {
                                st.fired.insert(key, (due, now, false));
                            }
                        }
                    }
                    _ => break,
                }
            }
            if !fired.is_empty() {
                drop(st);
                for w in fired.drain(..) {
                    w.wake();
                }
                st = self.lock();
                continue;
            }
            match st.schedule.first() {
                Some(&(due, _, _)) => {
                    st.wait_until = due;
                    st = self
                        .cv
                        .wait_timeout(st, Duration::from_nanos(due - now))
                        .unwrap_or_else(PoisonError::into_inner)
                        .0;
                }
                None => {
                    st.wait_until = u64::MAX;
                    st = self.cv.wait(st).unwrap_or_else(PoisonError::into_inner);
                }
            }
            st.wait_until = 0;
        }
    }
}
