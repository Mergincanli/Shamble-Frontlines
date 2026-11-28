
/// TEMPORARY (C1b finish; removed after its runs): the critic's probe 2, printed only. A spawned pump over SlowS (each
/// write takes `slow`), fixed(200) (100 ms one way), records A and B of 1,000 B written `gap` apart: B's lateness, the
/// time from its model delivery stamp to the return of S's write that took it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn zz_probe2_second_record_goes_out_soon_after_its_release() {
    for (slow, gap) in [(30u64, 20u64), (2, 1), (0, 0)] {
        let (_client, server) = tokio::io::duplex(1 << 20);
        let out = Arc::new(Mutex::new(Vec::new()));
        let s = SlowS {
            io: server,
            slow: Duration::from_millis(slow),
            out: Arc::clone(&out),
        };
        let mut ds = DelayStream::wrap(s, &cfg(fixed(200), 20), 0, now()).expect("wrapped");
        let rec = vec![0x82u8; 1_000];
        ds.write_all(&rec).await.expect("A");
        std::thread::sleep(Duration::from_millis(gap));
        ds.write_all(&rec).await.expect("B");
        assert!(
            until(HANG, || out.lock().expect("out").len() >= 2).await,
            "hang guard: A and B written"
        );
        let s = ds.snapshot();
        let down: Vec<&SplitLog> = s.logs.splits.iter().filter(|r| r.dir == DOWN).collect();
        assert_eq!(down.len(), 2, "A and B released and written");
        let us = |x: u64| (x - s.origin) / 1_000;
        println!(
            "PROBE2 slow {slow} ms gap {gap} ms: releases (model us) [{}, {}]; S write returns (model us) [{}, {}]; \
             B written {} us after its release (B's kind {}, woken {})",
            us(down[0].due),
            us(down[1].due),
            us(down[0].at),
            us(down[1].at),
            (down[1].at - down[1].due) / 1_000,
            down[1].kind,
            down[1].woke.is_some()
        );
    }
}
