# U3c's kernel cross-check, the kernel side (run.sh's header holds the pattern and the pass rule). Adapted from the
# scout's u3-tcp-stall-probe.py: loopback TCP, client -> server, one 200 B message every 33,333,333 ns with
# TCP_NODELAY, TCP_CONGESTION set to cubic on the listener and both sockets (read back and recorded). Loss comes from
# run.sh's iptables rule, never from here. Prints one JSON object: every message's send lateness and delay (ns), the
# congestion control each socket ended with, and the client's tcp_info at the end.
# Also recorded: the system-wide retransmission counters before and after the run (/proc/net/snmp Tcp RetransSegs;
# /proc/net/netstat TcpExt TCPFastRetrans, TCPLossProbes, TCPTimeouts, TCPLostRetransmit), as evidence beside the
# client's tcpi_total_retrans, which compare.py's noise guard reads (run.sh's header).
# Run: python3 -I probe.py <port> <messages>
import json
import socket
import struct
import sys
import threading
import time

PORT = int(sys.argv[1])
N = int(sys.argv[2])
SIZE = 200
PERIOD_NS = 33_333_333
TCP_CONGESTION = getattr(socket, "TCP_CONGESTION", 13)
COUNTERS = {
    "/proc/net/snmp": ("Tcp:", ["RetransSegs"]),
    "/proc/net/netstat": ("TcpExt:", ["TCPFastRetrans", "TCPLossProbes", "TCPTimeouts", "TCPLostRetransmit"]),
}


def kernel_counters():
    """The named counters now (None where a file or name is missing)."""
    out = {}
    for path, (prefix, names) in COUNTERS.items():
        try:
            lines = [line.split() for line in open(path) if line.startswith(prefix)]
        except OSError:
            lines = []
        table = dict(zip(lines[0][1:], (int(x) for x in lines[1][1:]))) if len(lines) >= 2 else {}
        for name in names:
            out[name] = table.get(name)
    return out


def set_cc(s, errors, who):
    try:
        s.setsockopt(socket.IPPROTO_TCP, TCP_CONGESTION, b"cubic")
    except OSError as e:
        errors.append(f"{who}: {e}")


def get_cc(s):
    return s.getsockopt(socket.IPPROTO_TCP, TCP_CONGESTION, 16).split(b"\0", 1)[0].decode()


errors = []
recv_at = [None] * N
sent_at = [None] * N
server_cc = []


def server(ls):
    c, _ = ls.accept()
    set_cc(c, errors, "accepted")
    server_cc.append(get_cc(c))
    c.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    buf = b""
    got = 0
    while got < N:
        d = c.recv(65536)
        if not d:
            break
        now = time.monotonic_ns()
        buf += d
        while len(buf) >= SIZE:
            seq, sent = struct.unpack_from("<QQ", buf, 0)
            buf = buf[SIZE:]
            recv_at[seq] = now
            got += 1
    c.close()


counters_before = kernel_counters()
ls = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
ls.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
set_cc(ls, errors, "listener")
ls.bind(("127.0.0.1", PORT))
ls.listen(1)
t = threading.Thread(target=server, args=(ls,))
t.start()
cs = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
set_cc(cs, errors, "client")
cs.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
cs.connect(("127.0.0.1", PORT))
client_cc = get_cc(cs)
start = time.monotonic_ns()
late = []
for i in range(N):
    due = start + i * PERIOD_NS
    while True:
        now = time.monotonic_ns()
        if now >= due:
            break
        time.sleep(min(0.005, (due - now) / 1e9))
    stamp = time.monotonic_ns()
    sent_at[i] = stamp
    late.append(stamp - due)
    cs.sendall(struct.pack("<QQ", i, stamp).ljust(SIZE, b"\0"))
info = cs.getsockopt(socket.IPPROTO_TCP, socket.TCP_INFO, 256)
cs.close()
t.join()
ls.close()
counters_after = kernel_counters()
# struct tcp_info (include/uapi/linux/tcp.h): 8 u8 fields, then u32 rto, ato, snd_mss, rcv_mss, unacked, sacked, lost,
# retrans, fackets, last_data_sent, last_ack_sent, last_data_recv, last_ack_recv, pmtu, rcv_ssthresh, rtt, rttvar,
# snd_ssthresh, snd_cwnd, advmss, reordering, rcv_rtt, rcv_space, total_retrans.
u32 = struct.unpack_from("<24I", info, 8)
delays = [None if r is None else r - s for r, s in zip(recv_at, sent_at)]
print(
    json.dumps(
        {
            "port": PORT,
            "messages": N,
            "size_bytes": SIZE,
            "period_ns": PERIOD_NS,
            "cc_client": client_cc,
            "cc_server": server_cc[0] if server_cc else None,
            "cc_errors": errors,
            "delivered": sum(1 for d in delays if d is not None),
            "send_late_ns": late,
            "delay_ns": delays,
            "tcp_info_client": {
                "rto_us": u32[0],
                "rtt_us": u32[15],
                "rttvar_us": u32[16],
                "snd_cwnd": u32[18],
                "total_retrans": u32[23],
            },
            "kernel_counters_before": counters_before,
            "kernel_counters_after": counters_after,
        }
    )
)
