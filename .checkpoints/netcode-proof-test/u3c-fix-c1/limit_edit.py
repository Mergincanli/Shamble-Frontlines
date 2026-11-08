"""U3c fix C1: each_limit_closes_before_buffering_and_is_counted gains S2's heads and F-F's pin; uses `Limit`."""
import pathlib
import sys

p = pathlib.Path(sys.argv[1])
s = p.read_text()

reps = [
    (
        """/// - the request cap is also the scanner's read limit, so the request bite lifts both.
///
/// Plus the limit raised in L2Config (one source) letting the 514 B frame in.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn each_limit_closes_before_buffering_and_is_counted() {""",
        """/// - the request cap is also the scanner's read limit, so the request bite lifts both.
///
/// S2: the request's end is where tungstenite's parser (httparse) puts it, so the limits apply to the same bytes it
/// reads as frames: a head ending in bare LFs, one with mixed line ends and one after leading blank lines each end
/// where httparse ends them, and the over-limit frame after them closes the connection, counted, with only the head
/// entered. The reproduced input among them: a head of exactly 4,096 B (tungstenite's read chunk) ending in "\\n\\n",
/// then a 604 B masked frame (before the fix it passed uncounted and entered the model).
///
/// F-F: after each close, tungstenite's side gets exactly the up records the model released, then the error; with a
/// 10 s one-way delay the request entered and is never released, so it is never handed (discarded by the close).
///
/// Plus the limit raised in L2Config (one source) letting the 514 B frame in.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn each_limit_closes_before_buffering_and_is_counted() {""",
    ),
    (
        """    let fragment = frame(false, 2, true, &[1; 300], false);
    let cases: Vec<Case> = vec![""",
        """    let fragment = frame(false, 2, true, &[1; 300], false);
    // S2's heads (httparse 1.10.1 ends a head at its first empty line, an LF or CR LF line, after a non-empty one,
    // leading empty lines skipped: lib.rs:558-581, :1105-1116).
    let mut lf = b"GET / HTTP/1.1\\nHost: localhost\\nUpgrade: websocket\\nConnection: Upgrade\\n\\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\\nSec-WebSocket-Version: 13\\nX-Pad: "
        .to_vec();
    lf.resize(4_096 - 2, b'a');
    lf.extend(b"\\n\\n");
    assert_eq!(lf.len(), 4_096, "the reproduced head: exactly one tungstenite read");
    let mixed = b"GET / HTTP/1.1\\r\\nHost: localhost\\nUpgrade: websocket\\r\\nConnection: Upgrade\\n\\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\\r\\nSec-WebSocket-Version: 13\\n\\r\\n"
        .to_vec();
    let leading = [&b"\\r\\n\\n\\r\\n"[..], REQUEST].concat();
    let cases: Vec<Case> = vec![
        (
            lf.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            lf.len() as u64 + OVERHEAD,
            Vec::new(),
        ),
        (
            mixed.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            mixed.len() as u64 + OVERHEAD,
            Vec::new(),
        ),
        (
            leading.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            leading.len() as u64 + OVERHEAD,
            Vec::new(),
        ),""",
    ),
    (
        """    for (head, bad, why, before, bites) in &cases {
        let (closed, entered, eof) = limit_case(&c, head, bad, &[], *before).await;
        assert_eq!(closed, Some(*why), "{why:?}");
        assert_eq!(
            entered, *before,
            "{why:?}: nothing of the bad frame entered"
        );
        assert!(eof, "{why:?}: S was dropped");
        for (off, then) in bites {
            let (closed, entered, _) = limit_case(&c, head, bad, off, *before).await;""",
        """    for (head, bad, why, before, bites) in &cases {
        let l = limit_case(&c, head, bad, &[], *before).await;
        assert_eq!(l.closed, Some(*why), "{why:?}");
        assert_eq!(
            l.entered, *before,
            "{why:?}: nothing of the bad frame entered"
        );
        assert!(l.eof, "{why:?}: S was dropped");
        println!(
            "{why:?} after a {} B head: closed before buffering ({before} B up); {} B handed, {} B released",
            head.len(),
            l.handed,
            l.released
        );
        for (off, then) in bites {
            let Limit {
                closed, entered, ..
            } = limit_case(&c, head, bad, off, *before).await;""",
    ),
    (
        """    // One source: the limit raised to 16,000 B in L2Config lets the 514 B frame's payload into the model.
    let mut open = c.clone();
    open.ws_max_message_bytes = 16_000;
    let (closed, entered, _) = limit_case(
        &open,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(closed, None);
    assert!(entered > request, "the payload entered: {entered} B");
}""",
        """    // F-F, pinned: with a 10 s one-way delay the request entered and was not released when the frame closed the
    // connection, so tungstenite's side is handed nothing before the error.
    let slow = cfg(fixed(20_000), 4);
    let l = limit_case(
        &slow,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(l.closed, Some(Frame));
    assert_eq!((l.entered, l.released, l.handed), (request, 0, 0));
    // One source: the limit raised to 16,000 B in L2Config lets the 514 B frame's payload into the model.
    let mut open = c.clone();
    open.ws_max_message_bytes = 16_000;
    let l = limit_case(
        &open,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(l.closed, None);
    assert!(l.entered > request, "the payload entered: {} B", l.entered);
}""",
    ),
]
for old, new in reps:
    if s.count(old) != 1:
        sys.exit(f"not exactly once: {old[:90]}")
    s = s.replace(old, new)
p.write_text(s)
print("each_limit updated")
