# U3 scout probe (throwaway; rust-reviewer, 2026-11-06): how Linux repairs ONE lost segment in a steady 30 Hz stream.
# Loopback TCP, client -> server, one 200 B message every 33.3 ms with TCP_NODELAY. Loss is injected outside this
# script by one iptables rule on lo for the server port (random 2 %, client -> server direction only), added before
# and removed after the run by the caller. Each message carries (seq, send time); the server logs the arrival delay.
# A lost segment shows up as a stall: its delay jumps, and the next messages queue behind it (TCP keeps order).
# Prediction (RACK, net/ipv4/tcp_recovery.c): stall ~ the gap to the next message (33 ms) + RTT (~0 on lo);
# an RTO-driven repair would give >= 200 ms (TCP_RTO_MIN). Run: python3 -I <this> <port> <messages>
import socket, struct, sys, threading, time

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 47999
N = int(sys.argv[2]) if len(sys.argv) > 2 else 3000
SIZE = 200
PERIOD_NS = 33_333_333

delays = [None] * N

def server(ls):
    c, _ = ls.accept()
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
            delays[seq] = now - sent
            got += 1
    c.close()

ls = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
ls.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
ls.bind(("127.0.0.1", PORT))
ls.listen(1)
t = threading.Thread(target=server, args=(ls,))
t.start()
cs = socket.create_connection(("127.0.0.1", PORT))
cs.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
start = time.monotonic_ns()
for i in range(N):
    due = start + i * PERIOD_NS
    while True:
        now = time.monotonic_ns()
        if now >= due:
            break
        time.sleep(min(0.005, (due - now) / 1e9))
    msg = struct.pack("<QQ", i, time.monotonic_ns()).ljust(SIZE, b"\0")
    cs.sendall(msg)
info = cs.getsockopt(socket.IPPROTO_TCP, socket.TCP_INFO, 256)
cs.close()
t.join()
# struct tcp_info: u8 state, ca_state, retransmits, probes, backoff, options, wscale, flags; then u32 rto, ato,
# snd_mss, rcv_mss, unacked, sacked, lost, retrans, fackets, last_data_sent, last_ack_sent, last_data_recv,
# last_ack_recv, pmtu, rcv_ssthresh, rtt, rttvar, snd_ssthresh, snd_cwnd, advmss, reordering, rcv_rtt, rcv_space,
# total_retrans (include/uapi/linux/tcp.h)
u32 = struct.unpack_from("<24I", info, 8)
rto_us, rtt_us, rttvar_us, total_retrans = u32[0], u32[15], u32[16], u32[23]
d = [x for x in delays if x is not None]
ms = sorted(x / 1e6 for x in d)
def q(p):
    return ms[min(len(ms) - 1, max(0, int(p * len(ms) + 0.5) - 1))]
# a stall starts where a message's delay exceeds 5 ms and the previous message's did not
stalls = []
for i in range(1, len(delays)):
    a, b = delays[i - 1], delays[i]
    if b is not None and a is not None and b > 5_000_000 and a <= 5_000_000:
        stalls.append(b / 1e6)
stalls.sort()
print(f"messages={len(d)} of {N}; delay ms p50={q(0.5):.3f} p99={q(0.99):.3f} max={ms[-1]:.3f}")
print(f"client tcp_info at end: rto={rto_us/1e3:.1f} ms rtt={rtt_us/1e3:.3f} ms rttvar={rttvar_us/1e3:.3f} ms total_retrans={total_retrans}")
if stalls:
    sq = lambda p: stalls[min(len(stalls) - 1, max(0, int(p * len(stalls) + 0.5) - 1))]
    print(f"stalls={len(stalls)} (first delayed message per stall) ms: min={stalls[0]:.1f} p50={sq(0.5):.1f} p90={sq(0.9):.1f} max={stalls[-1]:.1f}")
    print("stall histogram (ms bins of 10):", {int(s // 10) * 10: sum(1 for x in stalls if int(x // 10) * 10 == int(s // 10) * 10) for s in stalls})
else:
    print("stalls=0")
