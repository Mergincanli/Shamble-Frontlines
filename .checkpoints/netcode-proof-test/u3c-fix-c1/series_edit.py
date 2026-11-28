"""U3c fix C1: SERIES.md's DelayStream rows (NC-U3c-2, -4, -9, -11, F7). Each replacement must match exactly once."""
import pathlib
import sys

p = pathlib.Path(sys.argv[1])
s = p.read_text()

held_old = (
    "bytes DelayStream holds at the second's end (down: plaintext not yet taken by S, at most the 64 KB budget; up: "
    "records in the model or released and not yet read, plus the scanner's buffer, at most 64 KB plus 8,192 B) |"
)
held_new = (
    "bytes DelayStream holds at the second's end (down: plaintext not yet taken by S, at most the 64 KB budget plus "
    "the acknowledged prefix of the front record not yet released, so below 64 KB + 16,406 B (a loss inside a record "
    "lets the acknowledgement of its first segments free that prefix in the budget while the record waits whole); up: "
    "records in the model or released and not yet read, plus the scanner's buffer: at most 64 KB plus the acknowledged "
    "prefix of the front unreleased record (below 549 B, or the request's 8,214 B) plus one frame (527 B) or the "
    "8,192 B request cap) |"
)
reps = [
    (held_old, held_new, 2),
    (
        "| shutdowns that gave up at their 10 s deadline (delivery or S's taking unfinished) and dropped S |",
        "| shutdowns that gave up at their 10 s deadline (delivery, S's taking the bytes, or S's own shutdown "
        "unfinished) and dropped S |",
        1,
    ),
    (
        "| from the return to the first in-order byte released down (S12); u64::MAX when none came before the "
        "connection ended |",
        "| from the return to the first in-order byte the model delivers down (its receiver's rcv_nxt advances; S12), "
        "which may sit inside a record not yet released whole; u64::MAX when none came before the connection ended |",
        1,
    ),
    (
        "| from the return to the first in-order byte released up; u64::MAX: none |",
        "| from the return to the first in-order byte the model delivers up (rcv_nxt advances); u64::MAX: none |",
        1,
    ),
    (
        "in the model's bytes (+ 22 B per TLS record); mechanism test (c) checks it against N x the snapshot |",
        "in the model's bytes (+ 22 B per TLS record); mechanism test (c) checks it against N x the snapshot, N from "
        "the srtt at the writer's last write before the return (NC-U3c-1). Narrower than advisor-ws-docs-r2.md:379 "
        "(3)'s outstanding_bytes, which adds what is held above the socket: that is this plus `ws_owed_bytes` plus "
        "what WsServer holds itself (U6 writes the sum; NC-U3c-4) |",
        1,
    ),
    (
        "| `event.burst_return.ws_untaken_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server "
        "on RS) | down bytes (model bytes) released by the model and not yet taken by S at the return |",
        "| `event.burst_return.ws_untaken_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server "
        "on RS) | down bytes (model bytes) released by the model and not yet taken by S at the return |\n"
        "| `event.burst_return.ws_owed_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on "
        "RS) | bytes tungstenite held above DelayStream at the return (offered in its latest write and not taken); "
        "r2:379 (3)'s outstanding_bytes = `outstanding_bytes` + this + what WsServer holds (NC-U3c-4) |",
        1,
    ),
    (
        "| `header.params.ws_returns_max_count` | count | npt-net (DelayStream) | the per-return records a connection "
        "keeps (1,024) |",
        "| `header.params.ws_returns_max_count` | count | npt-net (DelayStream) | the per-return records a connection "
        "keeps (1,024) |\n"
        "| `header.params.ws_conn_fixed_bytes` | bytes | npt-net (DelayStream) | the heap a connection allocates at "
        "wrap, before any WebSocket byte (F7): one lateness histogram per direction with its window's sparse mark "
        "(142,240 B each), the scanner's buffer (8,192 B at the defaults), the down staging buffer (16,384 B) and the "
        "pump's read buffer (527 B): 309,583 B at the defaults. On top: the held bytes (below about 82 KB each way) "
        "and, once a second's lateness touches more than 256 histogram buckets, a dense mark of 135,040 B per "
        "direction. U6 sizes WsServer's accept cap from it (and from tungstenite's own buffers) |",
        1,
    ),
    (
        "| that the TCP and TLS handshakes run undelayed below the wrapper and the model starts without an RTT sample |",
        "| that the TCP and TLS handshakes run undelayed below the wrapper and the model starts without an RTT sample "
        "(RTO 1 s and fix 1's 100 ms srtt until the first data acknowledgement, where Linux has the handshake's "
        "sample; NC-U3c-11) |",
        1,
    ),
]
for old, new, n in reps:
    got = s.count(old)
    if got != n:
        sys.exit(f"expected {n}, found {got}: {old[:80]}")
    s = s.replace(old, new)
p.write_text(s)
print("SERIES.md updated")
