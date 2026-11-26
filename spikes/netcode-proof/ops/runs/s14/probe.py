# S14's kernel side on RS (u8-brief.md rev 1.1 section 1 Block 4; plan.md:1256; advisor-u3q.md:139-143), a copy of
# results/u3c-kernel-check/probe.py adapted to a veth pair between two network namespaces (never imported from
# results/). ops/sbin/npt-s14 runs it: `server` and `client` as npt inside the namespaces, `bursts` as root.
#   python3 -I probe.py messages                    the run's message count (the schedule's length)
#   python3 -I probe.py schedule                    the burst schedule as NPT_KC_BURSTS reads it
#   python3 -I probe.py server <port> <n>           accepts one connection; prints every message's send stamp and delay
#   python3 -I probe.py client <host> <port> <n>    prints `START <first due ns>`, sends, then one JSON object
#   python3 -I probe.py bursts <first due ns>       root: netem to 100 % loss on both directions at each burst
# ---- Fixed in writing before any run (S14; the advisor's section 2 ruling on the burst knob) ---------------------
# PATTERN: one TCP connection client -> server over the veth pair, 200 B messages every 33,333,333 ns (30 Hz),
#   TCP_NODELAY, cubic on the listener and both sockets (read back and recorded), as U3c's probe. netem (npt-s14):
#   50 ms each way, 2 % loss on the client's egress (the data direction), no jitter (netem's jitter reorders, which
#   the model leaves out).
# SCHEDULE (advisor-u8-stopb.md section 4, NC-SB-4): 600 s steady, then 34 one-second total-loss bursts, one every
#   10 s (starts at 600, 610, ..., 930 s), then a 20 s tail: 950 s, 28,500 messages a run, three runs. About 1,000
#   steady episodes pooled at 2 % loss and 102 burst recoveries, so each p99 reads a rank, not a maximum. Each burst
#   is (start_ns, len_ns) from the run's first message, which is the form `kernel_cross_check_model`'s NPT_KC_BURSTS
#   knob reads (MOVED's rider): total loss on both directions during [start, start + len), applied when a packet is
#   sent (netem drops at enqueue).
# CLOCK: CLOCK_MONOTONIC is one clock for every namespace of the host, so the server's receive stamp minus the send
#   stamp carried in the message is the delay, and the bursts mode times its netem changes on the same clock.
import json
import socket
import struct
import subprocess
import sys
import time

SIZE = 200
PERIOD_NS = 33_333_333
STEADY_S = 600
STEADY_END_NS = (STEADY_S - 1) * 1_000_000_000  # compare.py's steady window; the client reads total_retrans there too
BURSTS = [((STEADY_S + 10 * k) * 1_000_000_000, 1_000_000_000) for k in range(34)]
TOTAL_S = 950
MESSAGES = TOTAL_S * 1_000_000_000 // PERIOD_NS
TCP_CONGESTION = getattr(socket, "TCP_CONGESTION", 13)
COUNTERS = {
    "/proc/net/snmp": ("Tcp:", ["RetransSegs"]),
    "/proc/net/netstat": ("TcpExt:", ["TCPFastRetrans", "TCPLossProbes", "TCPTimeouts", "TCPLostRetransmit"]),
}
TC = "/usr/sbin/tc"
LINKS = [("npt-s14c", "s14c", "loss 2%"), ("npt-s14s", "s14s", "")]


def kernel_counters():
    """The named counters now, in this network namespace (None where a file or name is missing)."""
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


def server(port, n):
    errors = []
    ls = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    ls.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    set_cc(ls, errors, "listener")
    ls.bind(("", port))
    ls.listen(1)
    c, _ = ls.accept()
    set_cc(c, errors, "accepted")
    c.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    sent, delay = [None] * n, [None] * n
    buf, got = b"", 0
    while got < n:
        d = c.recv(65536)
        if not d:
            break
        now = time.monotonic_ns()
        buf += d
        while len(buf) >= SIZE:
            seq, stamp = struct.unpack_from("<QQ", buf, 0)
            buf = buf[SIZE:]
            if seq < n and sent[seq] is None:
                sent[seq], delay[seq] = stamp, now - stamp
                got += 1
    cc = get_cc(c)
    c.close()
    ls.close()
    print(json.dumps({"role": "server", "port": port, "messages": n, "delivered": got, "cc_server": cc,
                      "cc_errors": errors, "sent_ns": sent, "delay_ns": delay}))


def client(host, port, n):
    errors = []
    counters_before = kernel_counters()
    cs = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    set_cc(cs, errors, "client")
    cs.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    for _ in range(50):
        try:
            cs.connect((host, port))
            break
        except OSError:
            time.sleep(0.1)
    else:
        sys.exit("probe.py client: could not connect")
    cc = get_cc(cs)
    first = time.monotonic_ns() + 500_000_000
    print(f"START {first}", flush=True)
    late, steady_retrans, steady_end = [], None, -(-STEADY_END_NS // PERIOD_NS)
    for i in range(n):
        if i == steady_end:
            # NC-SB-12: the steady window's retransmissions, before any burst's, for compare.py's noise guard.
            steady_retrans = struct.unpack_from("<24I", cs.getsockopt(socket.IPPROTO_TCP, socket.TCP_INFO, 256), 8)[23]
        due = first + i * PERIOD_NS
        while True:
            now = time.monotonic_ns()
            if now >= due:
                break
            time.sleep(min(0.005, (due - now) / 1e9))
        stamp = time.monotonic_ns()
        late.append(stamp - due)
        cs.sendall(struct.pack("<QQ", i, stamp).ljust(SIZE, b"\0"))
    time.sleep(3)
    info = cs.getsockopt(socket.IPPROTO_TCP, socket.TCP_INFO, 256)
    cs.close()
    # struct tcp_info (include/uapi/linux/tcp.h): 8 u8 fields, then u32 rto, ato, snd_mss, rcv_mss, unacked, sacked,
    # lost, retrans, fackets, last_data_sent, last_ack_sent, last_data_recv, last_ack_recv, pmtu, rcv_ssthresh, rtt,
    # rttvar, snd_ssthresh, snd_cwnd, advmss, reordering, rcv_rtt, rcv_space, total_retrans.
    u32 = struct.unpack_from("<24I", info, 8)
    print(json.dumps({"role": "client", "messages": n, "first_due_ns": first, "period_ns": PERIOD_NS,
                      "cc_client": cc, "cc_errors": errors, "send_late_ns": late,
                      "tcp_info_client": {"rto_us": u32[0], "rtt_us": u32[15], "rttvar_us": u32[16],
                                          "snd_cwnd": u32[18], "total_retrans": u32[23],
                                          "total_retrans_steady": steady_retrans},
                      "kernel_counters_before": counters_before, "kernel_counters_after": kernel_counters()}))


def netem(loss_all):
    """Both links to total loss (a burst) or back to their steady settings; each link's (call start, call end), the
    data direction (the client's egress) first (NC-SB-17: each direction's switch is recorded apart)."""
    out = []
    for ns, dev, steady in LINKS:
        loss = "loss 100%" if loss_all else steady
        t0 = time.monotonic_ns()
        subprocess.run([TC, "-n", ns, "qdisc", "change", "dev", dev, "root", "netem", "delay", "50ms", *loss.split()],
                       check=True, env={"PATH": "/usr/sbin:/usr/bin"})
        out.append((t0, time.monotonic_ns()))
    return out


def bursts(first):
    now = time.monotonic_ns()
    if abs(first - now) > 10_000_000_000:
        print("probe.py bursts: the first-message stamp is not within 10 s of now: refused", file=sys.stderr)
        sys.exit(2)
    actual, reverse, calls = [], [], []
    for start, length in BURSTS:
        time.sleep(max(0, first + start - time.monotonic_ns()) / 1e9)
        on = netem(True)
        time.sleep(max(0, first + start + length - time.monotonic_ns()) / 1e9)
        off = netem(False)
        # `actual` is the data direction's window, which compare.py's validity rule and recoveries read; the other
        # direction's is recorded beside it (NC-SB-17).
        actual.append([on[0][1] - first, off[0][1] - on[0][1]])
        reverse.append([on[1][1] - first, off[1][1] - on[1][1]])
        calls.append([[e - s for s, e in on], [e - s for s, e in off]])
    print(json.dumps({"schedule": [list(b) for b in BURSTS], "actual": actual, "actual_reverse": reverse,
                      "tc_call_ns": calls}))


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "messages" and len(sys.argv) == 2:
        print(MESSAGES)
    elif cmd == "schedule" and len(sys.argv) == 2:
        print(",".join(f"{s}:{l}" for s, l in BURSTS))
    elif cmd == "server" and len(sys.argv) == 4:
        server(int(sys.argv[2]), int(sys.argv[3]))
    elif cmd == "client" and len(sys.argv) == 5:
        client(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]))
    elif cmd == "bursts" and len(sys.argv) == 3 and sys.argv[2].isdigit():
        bursts(int(sys.argv[2]))
    else:
        print("usage: probe.py messages|schedule|server <port> <n>|client <host> <port> <n>|bursts <ns>", file=sys.stderr)
        sys.exit(2)


main()
