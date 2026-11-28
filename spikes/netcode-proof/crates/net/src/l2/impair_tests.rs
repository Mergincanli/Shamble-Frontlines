//! The impairment core on simulated time: one test per rule (M2, M3, M4, M10, M11's pool drop, M19's placement), with
//! the bites where they are cheap.

use super::super::config::L2Config;
use super::super::shaper::Shaper;
use super::super::stats::Kind;
use super::*;
use npt_l1::{Profile, ReorderStep};
use std::cell::Cell;
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

    // The successor taken in after the held one was due but before its token popped (the release thread running
    // late): not an overtake, the held one keeps its own time (L1's `now < held.arrival`, lib.rs:621). The bite: without
    // the `t < due` guard it would be due 1 ns after the successor. At exactly the due time it is not one either.
    for late in [15 * MS, 10 * MS] {
        let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(10)));
        hold_next(&mut im, a, 1000, 0);
        send(&mut im, DOWN, a, 0, 0);
        hold_next(&mut im, a, 0, 0);
        assert_eq!(due(send(&mut im, DOWN, a, late, 1)), late + 10 * MS);
        assert_eq!(
            drain(&mut im, SEC),
            vec![(10 * MS, 0), (late + 10 * MS, 1)],
            "successor at {late} ns"
        );
        assert_eq!(im.run.overtaken[DOWN], 0);
        assert_eq!(im.run.reordered[DOWN], 0);
    }

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
    // Each queued datagram is charged its whole 1,500 B buffer, whatever its payload: 3 per peer, 5 in total.
    c.peer_queue_bytes = 3 * CHARGE_BYTES;
    c.total_queue_bytes = 5 * CHARGE_BYTES;
    let mut im: Impairer<u64> = Impairer::new(&c);
    let mut took = false;
    let r = im.intake(UP, addr(1), 0, 1_201, || {
        took = true;
        Some(0)
    });
    assert_eq!(kind(r), Kind::Oversize);
    assert!(!took, "no buffer taken for an oversize datagram");
    // Three of 100 B fill the peer's three buffers; the fourth is a tail drop.
    for i in 0..3 {
        assert_eq!(kind(send(&mut im, UP, addr(1), 0, i)), Kind::Delivered);
    }
    assert_eq!(kind(send(&mut im, UP, addr(1), 0, 3)), Kind::QueueDrop);
    // Another peer has room of its own, up to the five in total.
    for i in 0..2 {
        assert_eq!(kind(send(&mut im, UP, addr(2), 0, i)), Kind::Delivered);
    }
    assert_eq!(
        kind(send(&mut im, UP, addr(2), 0, 9)),
        Kind::QueueDrop,
        "the total bound"
    );
    assert_eq!(im.queued_bytes()[UP], 5 * CHARGE_BYTES);
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
    // A host-dropped first packet (oversize here) creates no peer, so it stamps nothing: the stamp is the first packet
    // that passes the host checks (a flood from new addresses cannot churn the table).
    let mut c = cfg(fixed(200));
    c.max_datagram_bytes = 100;
    let mut im: Impairer<u64> = Impairer::new(&c);
    assert_eq!(
        kind(im.intake(UP, addr(2), 3 * MS, 101, || Some(0))),
        Kind::Oversize
    );
    assert_eq!(im.first_seen(addr(2)), None);
    send(&mut im, UP, addr(2), 4 * MS, 1);
    assert_eq!(im.first_seen(addr(2)), Some(4 * MS));
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

/// M4's "per direction": a peer's up and down flows are separate streams, so the same index draws different fates in
/// the two directions. The bite: one stream for both (the key without its direction) gives equal fates at every index.
#[test]
fn a_peers_two_directions_draw_apart() {
    let mut c = cfg(Profile::p2());
    c.profile.reorder = ReorderStep::OFF;
    let mut im: Impairer<u64> = Impairer::new(&c);
    let fate = |i: Intake<u64>, t: u64| match i {
        Intake::Queued { due_ns } => Some(due_ns - t),
        Intake::Dropped { .. } => None,
    };
    let n = 2_000u64;
    let mut same = 0u64;
    for i in 0..n {
        let t = i * MS;
        let up = fate(send(&mut im, UP, addr(1), t, i), t);
        let down = fate(send(&mut im, DOWN, addr(1), t, i), t);
        same += u64::from(up == down);
        drain(&mut im, t);
    }
    // P2's ±10 ms jitter in whole ns: two independent draws agree by chance about never (both lost: 0.04 %).
    assert!(
        same < 10,
        "{same} of {n} indices drew the same fate up and down"
    );
}

/// M19's placement with jitter: the shaper serialises in intake order, then each datagram takes its own delay, so a
/// datagram's due is (a FIFO's departure over the intake times) + (that index's delay, read from the same seed's run
/// without the shaper), and the jitter after the bottleneck may reorder. The bite: a shaper after the delay would
/// serialise in arrival order, and every due would move.
#[test]
fn the_shaper_serialises_in_intake_order_before_the_jitter() {
    let p = Profile {
        name: "T",
        rtt_ns: 100 * MS,
        jitter_ns: 40 * MS,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    };
    let mut plain: Impairer<u64> = Impairer::new(&cfg(p.clone()));
    // 1 Mbps with 200 ms of queue (25,000 B): a 1,000 B payload is 1,028 B, 8.224 ms; one every 5 ms builds a queue.
    let mut shaped: Impairer<u64> = Impairer::new(&cfg(p).with_bottleneck(1_000_000));
    let mut fifo = Shaper::new(1_000_000, 25_000);
    let mut dues = Vec::new();
    for i in 0..30u64 {
        let t = i * 5 * MS;
        let delay = due(plain.intake(DOWN, addr(1), t, 1_000, || Some(i))) - t;
        let depart = fifo.offer(t, 1_000).expect("the queue holds them");
        let d = due(shaped.intake(DOWN, addr(1), t, 1_000, || Some(i)));
        assert_eq!(
            d,
            depart + delay,
            "datagram {i}: its departure in intake order, then its own delay"
        );
        dues.push(d);
    }
    assert!(
        dues.windows(2).any(|w| w[1] < w[0]),
        "the ±20 ms jitter after the shaper reorders some: {dues:?}"
    );
}

/// M10's eviction rule with two unpinned candidates: the least recently seen goes, whichever slot it holds. The bite:
/// evicting the most recently seen (or always one slot) fails one of the two rounds.
#[test]
fn eviction_takes_the_least_recently_seen() {
    let mut c = cfg(fixed(10));
    c.peer_capacity = 2;
    let mut im: Impairer<u64> = Impairer::new(&c);
    send(&mut im, UP, addr(1), 0, 0);
    send(&mut im, UP, addr(2), MS, 1);
    // Peer 1 (slot 0) is seen again, so peer 2 (slot 1) is the least recently seen.
    send(&mut im, DOWN, addr(1), 2 * MS, 2);
    send(&mut im, UP, addr(3), 3 * MS, 3);
    assert_eq!(im.ordinal_of(addr(2)), None, "peer 2 went");
    assert_eq!(
        (im.ordinal_of(addr(1)), im.ordinal_of(addr(3))),
        (Some(0), Some(2))
    );
    // Now peer 3 (slot 1) is seen again: peer 1 (slot 0) is the least recently seen.
    send(&mut im, UP, addr(3), 4 * MS, 4);
    send(&mut im, UP, addr(4), 5 * MS, 5);
    assert_eq!(im.ordinal_of(addr(1)), None, "peer 1 went");
    assert_eq!(
        (im.ordinal_of(addr(3)), im.ordinal_of(addr(4))),
        (Some(2), Some(3))
    );
    assert_eq!(im.run.peers_evicted, 2);
}

/// A pool of buffers for the tests: `take` gives one while any is left (never given back here).
struct Pool(Cell<usize>);

impl Pool {
    fn take(&self) -> Option<u64> {
        let n = self.0.get();
        if n == 0 {
            return None;
        }
        self.0.set(n - 1);
        Some(0)
    }
}

/// M10's intent at the defaults: every queued datagram is charged its whole 1,500 B buffer against both byte bounds,
/// so one peer sending small datagrams holds at most its own bound (2,048 buffers a direction): it can neither empty
/// the shared pool nor fill a direction's total, and every other peer still gets through. Two flooders fill a total,
/// and still the pool is never empty (both totals together are the pool). The bite: charging the payload (50 B here)
/// lets one flooder take every pool buffer.
#[test]
fn one_flooding_peer_cannot_starve_another() {
    let c = L2Config::new(42, fixed(50));
    let buffers = |bytes: u64| bytes / CHARGE_BYTES;
    assert_eq!(
        2 * buffers(c.total_queue_bytes),
        c.pool_buffers as u64,
        "both directions' totals are the pool"
    );
    assert_eq!(
        2 * buffers(c.peer_queue_bytes),
        buffers(c.total_queue_bytes),
        "one peer holds half a direction's total"
    );
    let pool = Pool(Cell::new(c.pool_buffers));
    let mut im: Impairer<u64> = Impairer::new(&c);
    let n = 10_000u64;
    for i in 0..n {
        for dir in [UP, DOWN] {
            im.intake(dir, addr(1), i, 50, || pool.take());
        }
    }
    for dir in [UP, DOWN] {
        let f = im.run.fates[dir];
        assert_eq!(
            (f[Kind::QueueDrop as usize], f[Kind::PoolDrop as usize]),
            (n - buffers(c.peer_queue_bytes), 0),
            "the flooder meets its own bound, never the pool's end"
        );
        assert_eq!(im.queued_bytes()[dir], c.peer_queue_bytes);
    }
    for dir in [UP, DOWN] {
        assert_eq!(
            kind(im.intake(dir, addr(2), n, 50, || pool.take())),
            Kind::Delivered,
            "another peer gets through"
        );
    }
    // A second flooder fills the up direction's total: a third peer's up datagram is then a tail drop at the total
    // bound, never a pool drop, and its down datagram gets through.
    for i in 0..n {
        im.intake(UP, addr(3), n + i, 50, || pool.take());
    }
    assert_eq!(im.queued_bytes()[UP], c.total_queue_bytes);
    assert_eq!(
        kind(im.intake(UP, addr(4), 3 * n, 50, || pool.take())),
        Kind::QueueDrop
    );
    assert_eq!(
        kind(im.intake(DOWN, addr(4), 3 * n, 50, || pool.take())),
        Kind::Delivered
    );
    assert_eq!(
        im.run.fates[UP][Kind::PoolDrop as usize] + im.run.fates[DOWN][Kind::PoolDrop as usize],
        0
    );
    assert!(pool.0.get() > 0, "the pool never ran empty");
}

/// A datagram the host checks drop from a new (possibly spoofed) address, whether oversize, over a byte bound or with
/// the pool empty, creates no peer, evicts nobody and stamps nothing: its slot is taken only after the checks. The
/// bite: placing the peer first evicts peer 1 (the least recently seen) at the first such datagram.
#[test]
fn host_drops_from_new_addresses_touch_no_peer() {
    let mut c = cfg(fixed(10));
    c.peer_capacity = 2;
    c.max_datagram_bytes = 1_200;
    c.total_queue_bytes = 2 * CHARGE_BYTES;
    c.fate_log_capacity = 8;
    let mut im: Impairer<u64> = Impairer::new(&c);
    send(&mut im, UP, addr(1), 0, 0);
    send(&mut im, UP, addr(2), MS, 1);
    // The table is full of unpinned peers, and the up direction's total is full.
    assert_eq!(
        kind(im.intake(UP, addr(3), 2 * MS, 1_201, || Some(9))),
        Kind::Oversize
    );
    assert_eq!(kind(send(&mut im, UP, addr(4), 3 * MS, 9)), Kind::QueueDrop);
    assert_eq!(
        kind(im.intake(DOWN, addr(5), 4 * MS, 100, || None)),
        Kind::PoolDrop
    );
    assert_eq!(im.peers(), 2);
    assert_eq!(
        (im.ordinal_of(addr(1)), im.ordinal_of(addr(2))),
        (Some(0), Some(1))
    );
    assert_eq!(im.run.peers_evicted, 0);
    for a in [addr(3), addr(4), addr(5)] {
        assert_eq!(im.first_seen(a), None);
    }
    // The fate log files them before the table: ordinal u64::MAX, no index.
    let f = im.fates.entries();
    assert_eq!(f.len(), 3);
    assert!(
        f.iter()
            .all(|e| e.ordinal == u64::MAX && e.index == NO_INDEX)
    );
    // A datagram that passes the checks still takes a slot: the least recently seen unpinned peer goes.
    assert_eq!(
        kind(send(&mut im, DOWN, addr(6), 5 * MS, 9)),
        Kind::Delivered
    );
    assert_eq!(im.run.peers_evicted, 1);
    assert_eq!(
        (im.ordinal_of(addr(1)), im.ordinal_of(addr(6))),
        (None, Some(2))
    );
}

/// The shaper across -23's throttle toggles (128 kbps, 5,920 B): the datagrams the old shaper accepted keep their
/// departures, and the next departure is never before the last one, at the throttle's end and at its start, so with a
/// fixed delay nothing overtakes a queued datagram (release order = intake order). The bite: a fresh shaper (or none)
/// sends the datagram after the toggle at once, ahead of the queue.
#[test]
fn a_throttle_toggle_keeps_fifo_order() {
    const TX: u64 = 92_500_000; // 1,452 B + 28 B at 128 kbps
    let a = addr(1);
    let delay = 50 * MS;
    let on = Settings::of(&cfg(fixed(50)).with_throttle());
    let off = Settings::of(&cfg(fixed(50)));
    let mut im: Impairer<u64> = Impairer::new(&cfg(fixed(50)));
    assert!(im.set_peer(a, on.clone(), 0));
    for k in 1..=4u64 {
        assert_eq!(
            due(im.intake(DOWN, a, 0, 1_452, || Some(k))),
            k * TX + delay
        );
    }
    // The throttle ends at 1 ms: the next datagram leaves with the last accepted one, not at once.
    assert!(im.set_peer(a, off.clone(), MS));
    assert_eq!(
        due(im.intake(DOWN, a, 2 * MS, 100, || Some(5))),
        4 * TX + delay
    );
    // Once that departure has passed, at once again.
    assert_eq!(due(im.intake(DOWN, a, 500 * MS, 100, || Some(6))), 550 * MS);
    // The throttle starts again at 600 ms and takes two; it ends at 601 ms, and a datagram follows them.
    assert!(im.set_peer(a, on.clone(), 600 * MS));
    for k in 1..=2u64 {
        assert_eq!(
            due(im.intake(DOWN, a, 600 * MS, 1_452, || Some(6 + k))),
            600 * MS + k * TX + delay
        );
    }
    assert!(im.set_peer(a, off, 601 * MS));
    assert_eq!(
        due(im.intake(DOWN, a, 602 * MS, 100, || Some(9))),
        600 * MS + 2 * TX + delay
    );
    // It starts again at 603 ms while that departure (785 ms) is still ahead: the new shaper is busy until then, so
    // its first departure follows it at the rate.
    assert!(im.set_peer(a, on, 603 * MS));
    assert_eq!(
        due(im.intake(DOWN, a, 603 * MS, 1_452, || Some(10))),
        600 * MS + 3 * TX + delay
    );
    let order: Vec<u64> = drain(&mut im, u64::MAX).iter().map(|x| x.1).collect();
    assert_eq!(order, (1..=10).collect::<Vec<u64>>());
    assert_eq!(
        (
            im.run.reordered[DOWN],
            im.run.fates[DOWN][Kind::ShaperDrop as usize]
        ),
        (0, 0)
    );
}
