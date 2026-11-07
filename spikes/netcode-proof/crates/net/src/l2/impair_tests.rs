//! The impairment core on simulated time: one test per rule (M2, M3, M4, M10, M11's pool drop, M19's placement), with
//! the bites where they are cheap.

use super::super::config::L2Config;
use super::super::stats::Kind;
use super::*;
use npt_l1::{Profile, ReorderStep};
use std::net::SocketAddr;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

fn addr(port: u16) -> SocketAddr {
    SocketAddr::from(([192, 0, 2, 1], port))
}

fn cfg(profile: Profile) -> L2Config {
    L2Config {
        pool_buffers: 64,
        ..L2Config::new(42, profile)
    }
}

/// A profile with no jitter and no loss, `one_way_ms` each way, the reorder step off.
fn fixed(one_way_ms: u64) -> Profile {
    Profile {
        name: "T",
        rtt_ns: 2 * one_way_ms * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    }
}

/// Takes a datagram in, with its index as its buffer.
fn send(im: &mut Impairer<u64>, dir: usize, a: SocketAddr, t: u64, tag: u64) -> Intake<u64> {
    im.intake(dir, a, t, 100, || Some(tag))
}

fn due(i: Intake<u64>) -> u64 {
    match i {
        Intake::Queued { due_ns } => due_ns,
        Intake::Dropped { kind, .. } => panic!("dropped: {kind:?}"),
    }
}

fn kind(i: Intake<u64>) -> Kind {
    match i {
        Intake::Queued { .. } => Kind::Delivered,
        Intake::Dropped { kind, .. } => kind,
    }
}

/// Pops everything due by `now`, finishing each as delivered at its due time; returns (due, tag) in release order.
fn drain(im: &mut Impairer<u64>, now: u64) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    while let Some(mut e) = im.pop_due(now) {
        im.finish(&e, Kind::Delivered, e.due_ns, 0);
        out.push((e.due_ns, e.take_item().expect("a datagram")));
    }
    out
}

#[test]
fn ipv4_mapped_addresses_are_one_peer() {
    let mapped: SocketAddr = "[::ffff:192.0.2.1]:5000".parse().unwrap();
    assert_eq!(normalise(mapped), addr(5000));
    let v6: SocketAddr = "[2001:db8::1]:5000".parse().unwrap();
    assert_eq!(normalise(v6), v6, "a real IPv6 address stays");
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    send(&mut im, UP, addr(5000), 0, 0);
    send(&mut im, UP, mapped, 1, 1);
    assert_eq!(im.peers(), 1, "the mapped form found the same peer");
}

#[test]
fn the_pass_through_delays_nothing_and_drops_nothing() {
    let mut c = L2Config::clean(1);
    c.pool_buffers = 64;
    // Even a lossy profile and a block are ignored in the pass-through (M12: every impairment off).
    c.profile = Profile::p4();
    c.udp_block = true;
    let mut im: Impairer<u64> = Impairer::new(&c);
    for i in 0..50 {
        assert_eq!(due(send(&mut im, DOWN, addr(1), i * MS, i)), i * MS);
    }
    let out = drain(&mut im, SEC);
    assert_eq!(out.len(), 50);
    assert!(out.windows(2).all(|w| w[0].1 < w[1].1), "in order");
}

#[test]
fn the_injected_delay_is_net_of_the_base_rtt() {
    let mut c = cfg(fixed(50));
    c.base_rtt_ns = 10 * MS;
    let mut im: Impairer<u64> = Impairer::new(&c);
    // (100 ms - 10 ms) / 2 = 45 ms each way.
    assert_eq!(due(send(&mut im, DOWN, addr(1), 0, 0)), 45 * MS);
    assert_eq!(due(send(&mut im, UP, addr(1), 0, 1)), 45 * MS);
}

/// M2: one queue ordered by (due, sequence) across peers. The bite: a FIFO per direction would release peer A's
/// late datagram before peer B's early one.
#[test]
fn one_queue_releases_by_due_time_across_peers() {
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    let mut slow = cfg(fixed(80));
    slow.seed = 42;
    assert!(im.set_peer(addr(1), Settings::of(&slow), 0));
    assert_eq!(due(send(&mut im, DOWN, addr(1), 0, 100)), 80 * MS);
    assert_eq!(due(send(&mut im, DOWN, addr(2), MS, 200)), 11 * MS);
    assert_eq!(due(send(&mut im, UP, addr(2), 2 * MS, 300)), 12 * MS);
    assert_eq!(im.next_due(), Some(11 * MS));
    assert!(
        drain(&mut im, 10 * MS).is_empty(),
        "nothing is due before its time"
    );
    let order: Vec<u64> = drain(&mut im, SEC).iter().map(|x| x.1).collect();
    assert_eq!(order, vec![200, 300, 100]);
    // Equal due times keep their intake order.
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    for tag in 0..5 {
        send(&mut im, DOWN, addr(tag as u16 + 1), 0, tag);
    }
    let order: Vec<u64> = drain(&mut im, SEC).iter().map(|x| x.1).collect();
    assert_eq!(order, vec![0, 1, 2, 3, 4]);
}

/// Each datagram's own uniform jitter (M3's "jitter per datagram"), and the loss share.
#[test]
fn delays_are_uniform_per_datagram_and_loss_is_the_profiles() {
    let mut c = cfg(Profile::p2());
    c.pool_buffers = 1;
    c.profile.reorder = ReorderStep::OFF;
    let mut im: Impairer<u64> = Impairer::new(&c);
    let n = 100_000u64;
    let (mut lost, mut sum, mut lo, mut hi) = (0u64, 0u64, u64::MAX, 0u64);
    for i in 0..n {
        match send(&mut im, DOWN, addr(1), 0, i) {
            Intake::Queued { due_ns } => {
                sum += due_ns;
                lo = lo.min(due_ns);
                hi = hi.max(due_ns);
                let mut e = im.pop_due(SEC).expect("due");
                im.finish(&e, Kind::Delivered, e.due_ns, 0);
                e.take_item();
            }
            Intake::Dropped { kind, .. } => {
                assert_eq!(kind, Kind::Lost);
                lost += 1;
            }
        }
    }
    // P2: 50 ms +- 10 ms each way, 2 % loss.
    assert!((1_800..=2_200).contains(&lost), "lost {lost} of {n}");
    let mean = sum / (n - lost);
    assert!(mean.abs_diff(50 * MS) < MS / 10, "mean {mean}");
    assert_eq!(
        (lo / MS, hi.div_ceil(MS)),
        (40, 60),
        "the uniform's ends: {lo} {hi}"
    );
    assert!(lo >= 40 * MS && hi <= 60 * MS);
}

/// M4: fates per (peer, direction) with 16 words each. The other peer's traffic and the other direction's never move
/// them, and the key is the peer's slot id, not its ephemeral port.
#[test]
fn fates_do_not_move_with_interleaving_or_the_port() {
    let run = |interleave: bool, port_b: u16| -> (Vec<Kind>, Vec<u64>) {
        let mut c = cfg(Profile::p3());
        c.pool_buffers = 4_096;
        let mut im: Impairer<u64> = Impairer::new(&c);
        // A is seen first in both runs, so A is slot id 0 and B slot id 1.
        send(&mut im, UP, addr(1), 0, 0);
        send(&mut im, UP, addr(port_b), 0, 0);
        let mut a = (Vec::new(), Vec::new());
        for i in 0..2_000u64 {
            let t = i * MS;
            if interleave {
                // B's traffic and A's other direction between A's datagrams.
                send(&mut im, DOWN, addr(port_b), t, i);
                send(&mut im, UP, addr(1), t, i);
                send(&mut im, UP, addr(port_b), t, i);
            }
            let r = send(&mut im, DOWN, addr(1), t, i);
            if let Intake::Queued { due_ns } = r {
                a.1.push(due_ns - t);
            }
            a.0.push(kind(r));
            drain(&mut im, t);
        }
        a
    };
    let quiet = run(false, 2);
    assert!(quiet.0.contains(&Kind::Lost), "P3's 5 % loss shows");
    assert_eq!(
        run(true, 2),
        quiet,
        "B's traffic and A's up direction moved A's down fates"
    );
    assert_eq!(run(true, 40_000), quiet, "B's port moved A's fates");
    // And B's own fates follow its slot id, not its port.
    let b = |port: u16| {
        let mut im: Impairer<u64> = Impairer::new(&cfg(Profile::p3()));
        send(&mut im, UP, addr(1), 0, 0);
        (0..500u64)
            .map(|i| {
                let k = kind(send(&mut im, DOWN, addr(port), i * MS, i));
                drain(&mut im, i * MS);
                k
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(b(2), b(50_000));
    // The bite: a different seed moves them.
    let mut other = cfg(Profile::p3());
    other.seed = 43;
    let mut im: Impairer<u64> = Impairer::new(&other);
    let ks: Vec<Kind> = (0..2_000u64)
        .map(|i| {
            let k = kind(send(&mut im, DOWN, addr(1), i * MS, i));
            drain(&mut im, i * MS);
            k
        })
        .collect();
    assert_ne!(ks, quiet.0);
}

fn hold_next(im: &mut Impairer<u64>, a: SocketAddr, permille: u32, loss_ppm: u32) {
    let mut p = fixed(10);
    p.reorder = ReorderStep { permille };
    p.loss_ppm = loss_ppm;
    assert!(im.set_peer(a, Settings::of(&cfg(p)), 0));
}

/// M3: npt-l1's step as built. A held datagram is overtaken by its successor (due 1 ns after it) when the successor is
/// taken in before the held one is due and is not dropped.
#[test]
fn the_reorder_step_holds_until_1_ns_after_the_successor() {
    let a = addr(1);
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    hold_next(&mut im, a, 1000, 0);
    assert_eq!(
        due(send(&mut im, DOWN, a, 0, 0)),
        10 * MS,
        "held, due at its own time"
    );
    hold_next(&mut im, a, 0, 0);
    assert_eq!(due(send(&mut im, DOWN, a, 2 * MS, 1)), 12 * MS);
    let out = drain(&mut im, SEC);
    assert_eq!(out, vec![(12 * MS, 1), (12 * MS + 1, 0)], "overtaken");
    assert_eq!(im.run.overtaken[DOWN], 1);
    assert_eq!(im.run.reordered[DOWN], 1, "RFC 4737 counts it once");

    // The successor lost: the held one goes at its own time.
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    hold_next(&mut im, a, 1000, 0);
    send(&mut im, DOWN, a, 0, 0);
    hold_next(&mut im, a, 0, 1_000_000);
    assert_eq!(kind(send(&mut im, DOWN, a, 2 * MS, 1)), Kind::Lost);
    assert_eq!(drain(&mut im, SEC), vec![(10 * MS, 0)]);
    assert_eq!(im.run.overtaken[DOWN], 0);

    // The successor too late: the token releases the held one at its own time.
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    hold_next(&mut im, a, 1000, 0);
    send(&mut im, DOWN, a, 0, 0);
    hold_next(&mut im, a, 0, 0);
    assert_eq!(drain(&mut im, 10 * MS - 1), vec![]);
    assert_eq!(drain(&mut im, 10 * MS), vec![(10 * MS, 0)]);
    send(&mut im, DOWN, a, 20 * MS, 1);
    assert_eq!(drain(&mut im, SEC), vec![(30 * MS, 1)]);
    assert_eq!(im.run.overtaken[DOWN], 0);
    assert_eq!(im.run.reordered[DOWN], 0);

    // A successor in the other direction is not its successor.
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
    hold_next(&mut im, a, 1000, 0);
    send(&mut im, DOWN, a, 0, 0);
    hold_next(&mut im, a, 0, 0);
    send(&mut im, UP, a, 2 * MS, 1);
    assert_eq!(drain(&mut im, SEC), vec![(10 * MS, 0), (12 * MS, 1)]);
}

/// M3: the step's share where the jitter cannot reorder (P2 at 30 Hz: ±10 ms against 33 ms), about 1 % of delivered
/// datagrams, and the impairment layer's measured share (RFC 4737) equals the step's count. The bite: the step off
/// gives 0.
#[test]
fn the_reorder_share_is_the_steps_one_percent() {
    let share = |step: ReorderStep| {
        let mut c = cfg(Profile::p2());
        c.profile.reorder = step;
        let mut im: Impairer<u64> = Impairer::new(&c);
        for i in 0..30_000u64 {
            let t = i * 33_333_333;
            send(&mut im, DOWN, addr(1), t, i);
            drain(&mut im, t);
        }
        drain(&mut im, u64::MAX);
        (
            im.run.fates[DOWN][Kind::Delivered as usize],
            im.run.reordered[DOWN],
            im.run.overtaken[DOWN],
        )
    };
    let (delivered, reordered, overtaken) = share(ReorderStep::PART_13);
    let permille = reordered * 1000 / delivered;
    assert!((7..=13).contains(&permille), "{reordered} of {delivered}");
    assert_eq!(reordered, overtaken);
    assert_eq!(share(ReorderStep::OFF).1, 0);
}

#[test]
fn bursts_and_the_block_drop_both_ways() {
    let mut p = fixed(10);
    p.bursts_ns = vec![(SEC, SEC)];
    let mut im: Impairer<u64> = Impairer::new(&cfg(p));
    assert_eq!(
        kind(send(&mut im, DOWN, addr(1), SEC - 1, 0)),
        Kind::Delivered
    );
    assert_eq!(kind(send(&mut im, DOWN, addr(1), SEC, 1)), Kind::Burst);
    assert_eq!(
        kind(send(&mut im, UP, addr(1), 2 * SEC - 1, 2)),
        Kind::Burst
    );
    assert_eq!(
        kind(send(&mut im, UP, addr(1), 2 * SEC, 3)),
        Kind::Delivered
    );
    let mut blocked = cfg(fixed(10));
    blocked.udp_block = true;
    assert!(im.set_peer(addr(1), Settings::of(&blocked), 0));
    assert_eq!(kind(send(&mut im, UP, addr(1), 3 * SEC, 4)), Kind::Blocked);
    assert_eq!(
        kind(send(&mut im, DOWN, addr(1), 3 * SEC, 5)),
        Kind::Blocked
    );
    assert!(im.set_peer(addr(1), Settings::of(&cfg(fixed(10))), 0));
    assert_eq!(
        kind(send(&mut im, DOWN, addr(1), 4 * SEC, 6)),
        Kind::Delivered,
        "lifted"
    );
    assert_eq!(im.run.fates[UP][Kind::Burst as usize], 1);
    assert_eq!(im.run.fates[DOWN][Kind::Blocked as usize], 1);
}

/// M19's placement: the shaper's departure, then the profile's delay; its drops are its own count.
#[test]
fn the_shaper_sits_before_the_delay() {
    let c = cfg(fixed(50)).with_throttle();
    let mut im: Impairer<u64> = Impairer::new(&c);
    // 1,452 B = 1,480 B counted: 92.5 ms each at 128 kbps, then 50 ms of delay.
    for k in 1..=4u64 {
        let i = im.intake(DOWN, addr(1), 0, 1_452, || Some(k));
        assert_eq!(due(i), k * 92_500_000 + 50 * MS);
    }
    assert_eq!(
        kind(im.intake(DOWN, addr(1), 0, 1_452, || Some(5))),
        Kind::ShaperDrop
    );
    assert_eq!(im.run.fates[DOWN][Kind::ShaperDrop as usize], 1);
    assert_eq!(im.run.fates[DOWN][Kind::Lost as usize], 0);
    // The other direction has its own queue.
    assert_eq!(
        due(im.intake(UP, addr(1), 0, 1_452, || Some(6))),
        92_500_000 + 50 * MS
    );
}

/// M10 and M11: the size limit, the byte bounds and the pool are drops before the fate draws (no index, no buffer).
#[test]
fn host_drops_take_no_index_and_are_counted_apart() {
    let mut c = cfg(fixed(10));
    c.max_datagram_bytes = 1_200;
    c.peer_queue_bytes = 300;
    c.total_queue_bytes = 500;
    let mut im: Impairer<u64> = Impairer::new(&c);
    let mut took = false;
    let r = im.intake(UP, addr(1), 0, 1_201, || {
        took = true;
        Some(0)
    });
    assert_eq!(kind(r), Kind::Oversize);
    assert!(!took, "no buffer taken for an oversize datagram");
    // Three of 100 B fill the peer's 300 B; the fourth is a tail drop.
    for i in 0..3 {
        assert_eq!(kind(send(&mut im, UP, addr(1), 0, i)), Kind::Delivered);
    }
    assert_eq!(kind(send(&mut im, UP, addr(1), 0, 3)), Kind::QueueDrop);
    // Another peer has room of its own, up to the 500 B total.
    for i in 0..2 {
        assert_eq!(kind(send(&mut im, UP, addr(2), 0, i)), Kind::Delivered);
    }
    assert_eq!(
        kind(send(&mut im, UP, addr(2), 0, 9)),
        Kind::QueueDrop,
        "the total bound"
    );
    assert_eq!(im.queued_bytes()[UP], 500);
    // The other direction's bounds are its own.
    assert_eq!(kind(send(&mut im, DOWN, addr(1), 0, 10)), Kind::Delivered);
    // An empty pool.
    assert_eq!(
        kind(im.intake(DOWN, addr(1), 0, 100, || None)),
        Kind::PoolDrop
    );
    let up = im.run.fates[UP];
    assert_eq!(
        (
            up[Kind::Oversize as usize],
            up[Kind::QueueDrop as usize],
            up[Kind::Lost as usize]
        ),
        (1, 2, 0)
    );
    assert_eq!(im.run.fates[DOWN][Kind::PoolDrop as usize], 1);
    // Releasing frees the bytes.
    assert_eq!(drain(&mut im, SEC).len(), 6);
    assert_eq!(im.queued_bytes(), [0, 0]);
    assert_eq!(kind(send(&mut im, UP, addr(1), SEC, 11)), Kind::Delivered);
    // None of the host drops took an index: peer 1's up flow released indices 0, 1, 2 and now gives 3.
    let mut e = im.pop_due(u64::MAX).expect("queued");
    assert_eq!(e.index, 3);
    e.take_item();
}

/// M10: a table of fixed capacity, the least recently seen unpinned peer evicted (counted), pinned peers kept, the
/// overflow flow (counted) when every slot is pinned; an evicted peer's held datagram is never lost.
#[test]
fn the_peer_table_is_bounded() {
    let mut c = cfg(fixed(10));
    c.peer_capacity = 2;
    let mut im: Impairer<u64> = Impairer::new(&c);
    send(&mut im, UP, addr(1), 0, 0);
    send(&mut im, UP, addr(2), MS, 1);
    // Peer 1 holds a datagram when it is evicted.
    hold_next(&mut im, addr(2), 0, 0);
    let mut p = fixed(10);
    p.reorder = ReorderStep { permille: 1000 };
    im.set_default(Settings::of(&cfg(p)));
    send(&mut im, DOWN, addr(1), 2 * MS, 2);
    assert_eq!(im.ordinal_of(addr(1)), Some(0));
    send(&mut im, UP, addr(3), 3 * MS, 3);
    assert_eq!(im.peers(), 2);
    assert_eq!(
        im.ordinal_of(addr(1)),
        None,
        "the least recently seen unpinned peer went"
    );
    assert_eq!(
        im.ordinal_of(addr(3)),
        Some(2),
        "a new slot id, never reused"
    );
    assert_eq!(im.run.peers_evicted, 1);
    let tags: Vec<u64> = drain(&mut im, SEC).iter().map(|x| x.1).collect();
    assert!(
        tags.contains(&2),
        "the evicted peer's held datagram was released: {tags:?}"
    );
    // Pin both: a new address takes the overflow flow.
    assert!(im.set_peer(addr(3), Settings::of(&cfg(fixed(10))), 4 * MS));
    assert_eq!(kind(send(&mut im, UP, addr(4), 5 * MS, 4)), Kind::Delivered);
    assert_eq!(im.run.peer_overflow, 1);
    assert_eq!(im.peers(), 2);
    assert!(
        !im.set_peer(addr(5), Settings::of(&cfg(fixed(10))), 6 * MS),
        "no room to pin"
    );
    assert_eq!(drain(&mut im, SEC).len(), 1);
}

/// U3-Q8: the first raw packet's stamp is its arrival at the socket, never the end of its delay; sends set none.
#[test]
fn the_first_raw_packet_is_stamped_at_arrival() {
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(200)));
    send(&mut im, DOWN, addr(1), MS, 0);
    assert_eq!(
        im.first_seen(addr(1)),
        None,
        "an outbound datagram is not a raw packet from the peer"
    );
    send(&mut im, UP, addr(1), 7 * MS, 1);
    send(&mut im, UP, addr(1), 9 * MS, 2);
    assert_eq!(im.first_seen(addr(1)), Some(7 * MS));
    let mapped: SocketAddr = "[::ffff:192.0.2.1]:1".parse().unwrap();
    assert_eq!(im.first_seen(mapped), Some(7 * MS));
    // Even an oversize first packet is stamped (it is a raw packet), then dropped.
    let mut c = cfg(fixed(200));
    c.max_datagram_bytes = 10;
    let mut im: Impairer<u64> = Impairer::new(&c);
    assert_eq!(kind(send(&mut im, UP, addr(2), 3 * MS, 0)), Kind::Oversize);
    assert_eq!(im.first_seen(addr(2)), Some(3 * MS));
}

#[test]
fn finish_counts_errors_lateness_and_the_window() {
    let mut c = cfg(fixed(10));
    c.fate_log_capacity = 16;
    let mut im: Impairer<u64> = Impairer::new(&c);
    send(&mut im, DOWN, addr(1), 0, 0);
    send(&mut im, DOWN, addr(1), 0, 1);
    let mut e = im.pop_due(SEC).unwrap();
    im.finish(&e, Kind::SendError, 10 * MS + 40_000, 105);
    e.take_item();
    let mut e = im.pop_due(SEC).unwrap();
    im.finish(&e, Kind::WouldBlock, 10 * MS + 60_000, 11);
    e.take_item();
    assert_eq!(im.run.send_err[1], 1, "ENOBUFS");
    assert_eq!(im.run.fates[DOWN][Kind::WouldBlock as usize], 1);
    let s = im.late_run[DOWN].summary();
    assert_eq!((s.n, s.max), (2, 60_000));
    let f = im.fates.entries();
    assert_eq!(f.len(), 2);
    assert_eq!(
        (f[0].kind, f[0].errno, f[0].out_ns),
        (Kind::SendError, 105, 10 * MS + 40_000)
    );
    let second = im.take_second();
    assert_eq!(second.fates[DOWN][Kind::WouldBlock as usize], 1);
    assert_eq!(im.second, Counters::default());
    assert_eq!(im.late_second[DOWN].summary().n, 0);
    assert_eq!(im.late_run[DOWN].summary().n, 2, "the run keeps its own");
}
