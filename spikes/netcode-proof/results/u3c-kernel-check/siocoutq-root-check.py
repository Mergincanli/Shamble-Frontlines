#!/usr/bin/env python3
# u3c-plan.md 1.8 (e): the root run that tells SIOCOUTQ from SIOCOUTQNSD on this kernel, for npt-tcpinfo's
# `unacked_and_unsent` (the plain WebSocket path's "outstanding" in fix 1's gate). Written 2026-11-07 by U3c build B;
# the pass rule below was fixed then and revised the same day by build B's fix (THE CWND RULE, R4's RTO bound), still
# before any root run. The main session runs it after the merge, as root, one evidence run at a time with nothing
# else on loopback (u3c-plan.md 1.6's safety and 2 (6)).
#
# WHY A ROOT RUN. SIOCOUTQ is `write_seq - snd_una` (Linux v6.8 net/ipv4/tcp.c:630): every byte written and not yet
# acknowledged, sent or not. SIOCOUTQNSD is `write_seq - snd_nxt` (tcp.c:632-640): the bytes not yet sent, which is
# also tcpi_notsent_bytes (tcp.c:3854). A peer that stops reading cannot tell them apart: with its window closed
# nothing is in flight, so both read the same (crates/tcpinfo's tests show it: SIOCOUTQ = tx_queue = notsent there).
# They differ only while bytes are sent and unacknowledged, which this run makes last about 1 s by dropping the ACKs.
#
# WHAT IT DOES. It builds and starts the probe (crates/tcpinfo/examples/siocoutq_probe.rs, mode `lockstep`): a
# loopback connection on 127.0.0.1:PORT whose server end S writes and whose client end C reads everything. After a
# warm-up (20 x 200 B, all acknowledged, an RTT sample taken) the probe says READY. This script then inserts ONE rule:
#     iptables -I INPUT 1 -i lo -p tcp --dport PORT -j DROP
# which drops every packet to S's port, that is C's ACKs (the ACK direction); S's data to C's ephemeral port passes.
# It sends GO. The probe writes up to 20 messages of 200 B, 5 ms apart, under THE CWND RULE below (all inside the
# first RTO, which is at least 200 ms here: tcp_rto_min_us), sampling after each write; then it holds, sampling every
# 100 ms from 200 ms to 1,000 ms after GO; then says HOLD-END. The script deletes the rule (the drop lasted about 1 s)
# and sends RELEASED. The probe waits for SIOCOUTQ to reach 0 (R4's bound), evaluates the rule and prints one RULE
# line per rule and its VERDICT.
# Each sample reads SIOCOUTQ through npt-tcpinfo, then the kernel's tcp_info for S through a NETLINK_SOCK_DIAG dump
# (independent of npt-tcpinfo: tcpi_notsent_bytes, tcpi_snd_cwnd, tcpi_snd_mss, the in-flight counts,
# tcpi_total_retrans, tcpi_rto, sock_diag's wqueue), then SIOCOUTQ again; the two SIOCOUTQ readings must agree, since
# no ACK can arrive while the rule is in.
#
# THE CWND RULE (fixed 2026-11-07 before any root run; it replaces "20 writes", which could never pass). With every
# ACK dropped, S's packets in flight never fall, and tcp_write_xmit sends a segment only while they are below snd_cwnd
# (tcp_cwnd_test: Linux v6.8 net/ipv4/tcp_output.c:2073-2094, used at :2743-2749). After this warm-up snd_cwnd reads
# 10 (TCP_INIT_CWND, include/net/tcp.h:241; once 16 in 10 mirrored warm-ups, critic B's cwnd.py), so writes 11..20
# would stay unsent: the peer could not read them and tcpi_notsent_bytes would grow (R1 fails), and each would wait
# out the probe's delivery limit, pushing the last write past the first RTO (VOID). So message k is written only if,
# read through sock_diag just before it, S's packets in flight (tcpi_unacked - tcpi_sacked - tcpi_lost + tcpi_retrans,
# tcp_packets_in_flight, include/net/tcp.h:1253-1256) plus one are below tcpi_snd_cwnd; the one packet of room is
# for a tail loss probe (tcp_output.c:2829-2875, :2900), a retransmission that may go out between the reading and the
# write and counts in flight. The first message that fails the test ends the write phase, since no ACK can bring the
# count down. With TCP_NODELAY and 200 B <= tcpi_snd_mss each message is one segment, so the bytes written stay within
# (cwnd - 1) x 200 B <= cwnd x MSS: all in flight and unacknowledged, none unsent. N is the number of writes made
# (expected 9 at cwnd 10; one fewer if a loss probe already went out mid-phase).
#
# PASS RULE (fixed before any root run; k counts the writes after GO, 200 B each, N writes and 200 N bytes in all):
#   R0 (setup, else VOID): before GO, SIOCOUTQ = 0, tcpi_notsent_bytes = 0, wqueue = 0, and npt-tcpinfo's srtt_us
#      equals sock_diag's tcpi_rtt (0 read as None; not required above 0: loopback's RTT is a few us, tcpi_rtt whole us).
#   R1 (it grows): after write k (k = 1..N), once C has read all 200 k bytes: SIOCOUTQ = 200 k exactly, in both
#      readings, and tcpi_notsent_bytes <= 200 (near 0: at most one message not yet sent; expected 0).
#   R2 (it holds): at every hold sample (200 .. 1,000 ms after GO): SIOCOUTQ = 200 N in both readings, C has read all
#      200 N B, and tcpi_notsent_bytes <= 200.
#   R3 (the drop was real): tcpi_total_retrans at the hold's last sample > at GO (S retransmitted for want of ACKs).
#   R4 (release, an identity, no fixed time): once SIOCOUTQ reads 0, or 2 x tcpi_rto (read through sock_diag at
#      RELEASED) has passed, SIOCOUTQ = 0 in both readings, sock_diag's wqueue = 0 and C has read all 200 N B; and
#      SIOCOUTQ reached 0 within that bound. Why the bound cannot flake: no ACK reached S while the rule was in, so S's
#      retransmission timer is pending at RELEASED and due within one tcpi_rto as read then (tcp_retransmit_timer
#      re-arms it with the backed-off icsk_rto after each retransmission, net/ipv4/tcp_timer.c:654-658; the first
#      arming, tcp_rearm_rto's (tcp_input.c:3200-3225) and a loss probe's (tcp_output.c:2829-2875) are at most
#      icsk_rto; no ACK changed icsk_rto meanwhile). With the rule out, that retransmission reaches C, which answers at
#      once with a duplicate ACK and a D-SACK for data it already holds (tcp_input.c:5228-5236), and the ACK moves
#      snd_una to write_seq one loopback RTT (a few us) later. The second RTO is slack for the timer wheel (HZ 250,
#      4 ms ticks) and scheduling. (It replaces a fixed 5 s wall-clock limit.)
#   VOID (rerun, neither pass nor fail): R0 fails; a sample has no sock_diag reading; the last write came at or after
#      tcpi_rto (read at READY) after GO, when an RTO could have collapsed the window mid-phase; tcpi_snd_mss < 200 at
#      READY (a message would not be one segment); N < 4 (too few writes to separate the two, a cwnd under 5); no
#      sock_diag reading at RELEASED (no R4 bound).
#   PASS = R1 to R4 all pass and nothing is void. Why it separates the two: once C holds every byte, S has sent them
#   all, so write_seq - snd_nxt = 0 (and tcpi_notsent_bytes reads it directly); a function on SIOCOUTQNSD would read
#   at most 200 where R1 demands 200 k (400 .. 200 N for k >= 2) and R2 demands 200 N. A function on SIOCOUTQ reads
#   exactly the bytes written since GO, because no ACK reaches S while the rule is in.
#
# SAFETY. The rule matches only TCP to PORT on lo. It is removed in a finally block that also runs on SIGINT, SIGTERM
# and SIGHUP, and at most about 1 s after it went in on the normal path (each probe line has a 5 s limit, so a stuck
# probe ends the run; after RELEASED, with the rule already out, the verdict may take up to 30 s, above R4's
# 2 x tcpi_rto). `iptables -S` is read before and after and must be identical (exit 3 otherwise). The run
# refuses to start if the rule already exists. Nothing else may use loopback during the run (the operator's promise).
#
# RECORDED. Kernel release, CONFIG_HZ, the TCP sysctls (congestion control, tcp_recovery, tcp_early_retrans, tcp_sack,
# tcp_frto, tcp_rto_min_us, tcp_thin_linear_timeouts), the probe socket's congestion control (set to cubic, the
# model's; this container defaults to bbr, u3-scout-timing.md:388), the spike's commit with any uncommitted change in
# its folder (this script's own new logs aside: OWN_OUTPUTS), every probe line.
#
# RUN (from the repository root, as root):
#   CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-model-target \
#     python3 -I spikes/netcode-proof/results/u3c-kernel-check/siocoutq-root-check.py [--port 47613]
# A dry run (`--dry-run`) makes no iptables call at all and needs no root: it checks the plumbing, and the rule must
# then fail R1 and R3 (no drop, so the ACKs arrive and every one of the 20 writes passes the cwnd rule): the dry run
# exits 0 only if it does, showing that the rule bites.
# OUTPUT: siocoutq-root-check-<UTC stamp>.txt (dry runs: siocoutq-root-check-dry-<stamp>.txt) next to this script;
# .txt, not .log, because the repository ignores *.log and the output is committed beside the script.
# EXIT: 0 pass, 1 fail, 2 void or setup error, 3 iptables -S differs after the run.

import datetime
import fnmatch
import gzip
import os
import pathlib
import queue
import shutil
import signal
import subprocess
import sys
import threading

HERE = pathlib.Path(__file__).resolve().parent
SPIKE = HERE.parent.parent
LINE_LIMIT_S = 5.0
# After RELEASED (the rule is out): R4 waits up to 2 x tcpi_rto, which a late release can push past 5 s.
VERDICT_LIMIT_S = 30.0
SYSCTLS = [
    "tcp_congestion_control",
    "tcp_recovery",
    "tcp_early_retrans",
    "tcp_sack",
    "tcp_frto",
    "tcp_rto_min_us",
    "tcp_thin_linear_timeouts",
]


class Log:
    def __init__(self, path):
        self.f = open(path, "w", encoding="utf-8")
        self.path = path

    def __call__(self, line):
        print(line, flush=True)
        self.f.write(line + "\n")
        self.f.flush()


def hz():
    try:
        with gzip.open("/proc/config.gz", "rt") as f:
            text = f.read()
    except OSError:
        try:
            text = pathlib.Path(f"/boot/config-{os.uname().release}").read_text()
        except OSError:
            return "unknown"
    for line in text.splitlines():
        if line.startswith("CONFIG_HZ="):
            return line.split("=", 1)[1]
    return "unknown"


def sysctl(name):
    try:
        return pathlib.Path("/proc/sys/net/ipv4", name).read_text().strip()
    except OSError:
        return "absent"


# This script's own outputs (the logs it writes next to itself), left out of the provenance check while they are new:
# a run's log is opened before `git status` runs, so with them in, every run would read "with uncommitted changes" (u3c
# review F-J, fixed 2026-11-08 after the committed root run, whose log says so for that reason). Only an untracked one
# is left out: an edit to a committed log is listed like any other change (review C2R-10).
OWN_OUTPUTS = "siocoutq-root-check-*.txt"


def uncommitted():
    """The uncommitted changes in the spike's folder, as paths relative to the repository root (git status
    --porcelain), or the error text when git status fails. An untracked file next to this script whose name matches
    OWN_OUTPUTS is left out; everything else is listed, a committed log's edit included. Tested by
    test_siocoutq_root_check.py (check.mjs's self step runs it)."""
    status = subprocess.run(
        ["git", "-C", str(SPIKE), "status", "--porcelain", "--untracked-files=all", "--", "."],
        capture_output=True,
        text=True,
    )
    top = subprocess.run(["git", "-C", str(SPIKE), "rev-parse", "--show-toplevel"], capture_output=True, text=True)
    if status.returncode != 0 or top.returncode != 0:
        return f"git status failed: {(status.stderr or top.stderr).strip()}"
    root = pathlib.Path(top.stdout.strip())

    def own_new_log(line):
        path = root / line[3:]
        return line.startswith("?? ") and path.parent.resolve() == HERE and fnmatch.fnmatch(path.name, OWN_OUTPUTS)

    return [line[3:] for line in status.stdout.splitlines() if line.strip() and not own_new_log(line)]


def provenance():
    """The spike's commit and its uncommitted changes (up to five paths, so a reader can tell a source change from a
    stray file), this script's own outputs aside."""
    head = subprocess.run(["git", "-C", str(SPIKE), "rev-parse", "HEAD"], capture_output=True, text=True)
    text = f"spike commit {head.stdout.strip() or 'unknown'}"
    paths = uncommitted()
    if isinstance(paths, str):
        return f"{text} ({paths})"
    if paths:
        more = f" and {len(paths) - 5} more" if len(paths) > 5 else ""
        text += f" (with uncommitted changes: {', '.join(paths[:5])}{more})"
    return text


def iptables(*args, check=True):
    return subprocess.run(["iptables", *args], capture_output=True, text=True, check=check)


class Interrupted(Exception):
    pass


def on_signal(signum, _frame):
    raise Interrupted(f"signal {signum}")


def main(argv):
    port = 47613
    dry = False
    i = 0
    while i < len(argv):
        if argv[i] == "--dry-run":
            dry = True
        elif argv[i] == "--port" and i + 1 < len(argv):
            i += 1
            port = int(argv[i])
        else:
            print(f"unknown argument {argv[i]}", file=sys.stderr)
            return 2
        i += 1
    if not 1024 <= port <= 65535:
        print("the port must be 1024-65535", file=sys.stderr)
        return 2
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    log = Log(HERE / f"siocoutq-root-check-{'dry-' if dry else ''}{stamp}.txt")
    rule = ["INPUT", "-i", "lo", "-p", "tcp", "--dport", str(port), "-j", "DROP"]

    log(f"# siocoutq-root-check {'DRY RUN (no iptables call) ' if dry else ''}at {stamp}")
    log(f"# kernel {os.uname().release}, CONFIG_HZ {hz()}")
    log("# sysctl " + " ".join(f"{s}={sysctl(s)}" for s in SYSCTLS))
    log(f"# {provenance()}")

    if not dry:
        if os.geteuid() != 0:
            log("# not root: run it as root, or with --dry-run")
            return 2
        if shutil.which("iptables") is None:
            log("# no iptables")
            return 2
        if iptables("-C", *rule, check=False).returncode == 0:
            log(f"# the rule {' '.join(rule)} already exists: someone else's run? Not starting.")
            return 2

    build = subprocess.run(
        ["cargo", "+1.99.1", "build", "--locked", "-q", "-p", "npt-tcpinfo", "--example", "siocoutq_probe"],
        cwd=SPIKE,
        capture_output=True,
        text=True,
    )
    if build.returncode != 0:
        log("# the probe did not build:\n" + build.stderr)
        return 2
    target = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", SPIKE / "target"))
    probe = target / "debug" / "examples" / "siocoutq_probe"

    before = iptables("-S").stdout if not dry else None
    for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(s, on_signal)

    def remove_rule():
        if dry:
            return
        while iptables("-C", *rule, check=False).returncode == 0:
            iptables("-D", *rule)
            log(f"# rule out: iptables -D {' '.join(rule)}")

    proc = None
    lines = queue.Queue()
    verdict = None
    rules = {}
    code = 2
    try:
        proc = subprocess.Popen(
            [str(probe), "lockstep", "--port", str(port)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

        def pump():
            for line in proc.stdout:
                lines.put(line.rstrip("\n"))
            lines.put(None)

        threading.Thread(target=pump, daemon=True).start()

        def until(word, limit=LINE_LIMIT_S):
            nonlocal verdict
            while True:
                try:
                    line = lines.get(timeout=limit)
                except queue.Empty:
                    raise Interrupted(f"no probe line within {limit} s while waiting for {word}")
                if line is None:
                    raise Interrupted(f"the probe ended before {word}")
                log(line)
                if line.startswith("RULE "):
                    parts = line.split()
                    rules[parts[1]] = parts[2]
                if line.startswith("VERDICT "):
                    verdict = line.split()[1]
                    if word != "VERDICT":
                        raise Interrupted(f"the probe gave its verdict before {word}")
                if line == word or (word == "VERDICT" and verdict is not None):
                    return

        def send(word):
            proc.stdin.write(word + "\n")
            proc.stdin.flush()
            log(f"# sent {word}")

        until("READY")
        if not dry:
            iptables("-I", rule[0], "1", *rule[1:])
            log(f"# rule in: iptables -I {rule[0]} 1 {' '.join(rule[1:])}")
        send("GO")
        until("HOLD-END")
        remove_rule()
        send("RELEASED")
        until("VERDICT", VERDICT_LIMIT_S)
        code = proc.wait(timeout=LINE_LIMIT_S)
    except (Interrupted, subprocess.TimeoutExpired, BrokenPipeError) as e:
        log(f"# stopped: {e}")
        code = 2
    finally:
        remove_rule()
        if proc is not None and proc.poll() is None:
            proc.kill()
            proc.wait()
        if not dry:
            after = iptables("-S").stdout
            if after != before:
                log("# IPTABLES -S DIFFERS after the run:\n# before:\n" + before + "# after:\n" + after)
                code = 3
            else:
                log(f"# iptables -S identical before and after ({len(before.splitlines())} lines)")

    if dry:
        bites = verdict == "FAIL" and rules.get("R1") == "fail" and rules.get("R3") == "fail"
        log(
            f"# dry run: verdict {verdict}, R1 {rules.get('R1')}, R3 {rules.get('R3')}: "
            + ("the rule fails without the drop, as it must" if bites else "NOT the expected failure")
        )
        log(f"# log: {log.path}")
        return 0 if bites else 1
    log(f"# verdict {verdict}, exit {code}; log: {log.path}")
    return code


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
