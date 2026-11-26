// U8-F1 scratch: item 1e's pin test as drafted for ops/tests/s14.test.mjs (it would sit there between the two marker
// lines below; `text` joins that file's helpers import). Not in the spike's tree: it prices the never-carried part.
import assert from "node:assert/strict";
import { test } from "node:test";
import { text } from "../../../spikes/netcode-proof/ops/tests/helpers.mjs";

// ---- draft start ----
// S14-PUSH (a)'s second condition: R1's push carries the s14 folder whole, so what RS writes into it is pinned: its
// files, kernel.txt's sources, probe.py's JSON keys, its two addresses (compare.py's judge runs here, after the push).
test("S14-PUSH (a): the s14 folder's writers, kernel.txt's sources, probe.py's keys and its two addresses, pinned", () => {
  const only = (what, got, want) => assert.deepEqual([...new Set(got)].filter((x) => !want.includes(x)), [],
    `${what}: a change that puts anything more into the pushed s14 folder stops and goes to Mustafa first`);
  const npt = text("sbin/npt-s14"), probe = text("runs/s14/probe.py"), r1 = text("runs/r1.sh"), cmp = text("runs/s14/compare.py");
  const kernel = npt.slice(npt.indexOf("\n{\n"), npt.indexOf("\nfor run in")) + npt.split("\n").filter((l) => l.includes('>> "$OUT/kernel.txt"')).join("\n");
  const block = r1.slice(r1.indexOf("\ns14_run() {"), r1.indexOf("\nbites() {")) + r1.match(/\nblock_start\(\) \{[^]*?\n\}\n/)[0];
  only("npt-s14's files", [...npt.matchAll(/"\$OUT\/([^"]*)"/g)].map((m) => m[1]), ["kernel.txt", "server-run$run.json", "client-run$run.json", "bursts-run$run.json"]);
  only("npt-s14's lines writing them", npt.split("\n").map((l) => l.trim()).filter((l) => /"\$OUT\/|"\$PROBE" (?:server|client|bursts)/.test(l)), ['} > "$OUT/kernel.txt"',
    `printf 'loadavg_before_run%s %s\\n' "$run" "$(cat /proc/loadavg)" >> "$OUT/kernel.txt"`, `printf 'loadavg_after_run%s %s\\n' "$run" "$(cat /proc/loadavg)" >> "$OUT/kernel.txt"`,
    'ip netns exec "$NSS" "${SETPRIV_NPT[@]}" /usr/bin/python3 -I "$PROBE" server "$PORT" "$N" \\', '| head -c "$CAP" > "$OUT/server-run$run.json" &',
    'coproc CLIENT { exec ip netns exec "$NSC" "${SETPRIV_NPT[@]}" /usr/bin/python3 -I "$PROBE" client 198.51.100.2 "$PORT" "$N"; }',
    'python3 -I "$PROBE" bursts "${BASH_REMATCH[1]}" > "$OUT/bursts-run$run.json"', 'head -c "$CAP" <&"${CLIENT[0]}" > "$OUT/client-run$run.json"']);
  const summary = cmp.slice(cmp.indexOf('elif cmd == "summary"'), cmp.indexOf('elif cmd == "judge"')), standing = cmp.slice(cmp.indexOf("def standing("), cmp.indexOf("def model_lines("));
  assert.deepEqual([summary.match(/\bprint\(/g).length, cmp.match(/^import \w+$/gm)], [1, ["import glob", "import json", "import math", "import os", "import sys"]], "compare.py summary's one print");
  only("compare.py summary's values (summary.txt on RS)", [...(summary + standing).matchAll(/\{([^{}]+)\}/g)].map((m) => m[1]), ["name", "s['delivered']", "s['messages']",
    "base", "len(eps)", "len(rec)", "lat99", "c['tcp_info_client']", "kind or 'judged'", "why", "rtt", "len(BURSTS)", "s1", "l1", "s0", "l0", "steady"]);
  only("npt-s14's other $OUT lines", npt.split("\n").filter((l) => /\$OUT\b(?!\/)/.test(l)).map((l) => l.trim()),
    ['[ ! -e "$OUT" ] || die "$OUT exists already"', 'install -d -o root -g root -m 0755 "$OUT"', `printf 'npt-s14: %s runs done; results in %s\\n' "$RUNS" "$OUT"`]);
  only("kernel.txt's sources", [...kernel.matchAll(/\$\(([^)]*)\)/g)].map((m) => m[1]).concat(kernel.split("\n").map((l) => l.trim())
    .filter((l) => l && !/^(printf |for s in |tcp_dsack |done$|\{$|\} > )/.test(l))), ["uname -r", "date -u +%Y-%m-%dT%H:%M:%SZ", "cat /proc/loadavg",
    'ip netns exec "$NSC" cat "/proc/sys/net/ipv4/$s" 2> /dev/null || echo absent', 'tc -n "$NSC" qdisc show dev s14c', 'tc -n "$NSS" qdisc show dev s14s']);
  only("kernel.txt's variables", [...kernel.matchAll(/\$\{?([A-Za-z_]\w*)/g)].map((m) => m[1]), ["N", "RUNS", "s", "NSC", "NSS", "run", "OUT"]);
  assert.deepEqual(npt.match(/for s in ([^;]+); do/)[1].split(/[\s\\]+/), ["tcp_recovery", "tcp_early_retrans", "tcp_sack", "tcp_frto", "tcp_rto_min_us",
    "tcp_thin_linear_timeouts", "tcp_timestamps", "tcp_dsack", "tcp_congestion_control", "tcp_available_congestion_control"], "the listed net.ipv4 settings");
  only("r1.sh's writes into the folder", block.split("\n").map((l) => l.trim()).filter((l) => /^[^#]*(?:>|\btee |\bcp |s14_run ")/.test(l)), [
    'python3 -I "$SPIKE/ops/runs/s14/compare.py" summary "$1" | tee "$1/summary.txt"', 'cp -rp "$dir"/. "$1/"', 's14_run "$d"', 's14_run "$d/rerun"',
    'cp "$d/summary.txt" "$d/INVALID-s14.txt"', `printf 'no time for a re-run before the bites: on R2'"'"'s RS, a new folder (R1'"'"'s kept)\\n' | tee "$d/rerun-R2.txt"`,
    'cp -p "$R/browsers/stamps.txt" "$d/browser-stamps.txt" 2> /dev/null || true', `printf 'attempt %s %s\\n' "$n" "$(stamp)" >> "$R/$b/attempts"`,
    'if [ -f "$R/chain/$b.done" ] && ! compgen -G "$R/$b/INVALID*" > /dev/null; then',
    `dir=$(sudo -n /usr/local/sbin/npt-s14 run | sed -n 's/^npt-s14: .* results in \\(\\/var\\/lib\\/npt\\/s14\\/[0-9TZ]*\\)$/\\1/p')`]);
  only("the stamps' writers", r1.match(/^stamp\(\) .*|^ *printf '%s (?:up|down) %s\\n' .*/gm), ["stamp() { date -u +%Y-%m-%dT%H:%M:%SZ; }",
    `  printf '%s up %s\\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"`, `  printf '%s down %s\\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"`]);
  only("probe.py's JSON keys", [...probe.matchAll(/(?<!env=\{)"(\w+)": /g), ...probe.match(/^COUNTERS = \{[^]*?^\}/m)[0].matchAll(/"([A-Z]\w+)"/g)].map((m) => m[1]), [
    "role", "port", "messages", "delivered", "cc_server", "cc_errors", "sent_ns", "delay_ns", "first_due_ns", "period_ns", "cc_client", "send_late_ns",
    "tcp_info_client", "rto_us", "rtt_us", "rttvar_us", "snd_cwnd", "total_retrans", "total_retrans_steady", "kernel_counters_before", "kernel_counters_after",
    "schedule", "actual", "actual_reverse", "tc_call_ns", "RetransSegs", "TCPFastRetrans", "TCPLossProbes", "TCPTimeouts", "TCPLostRetransmit"]);
  for (const [name, t] of Object.entries({ "npt-s14": npt, "probe.py": probe, "compare.py": cmp, "r1.sh's s14 block": block })) {
    only(`${name}'s addresses`, t.match(/\b\d{1,3}(?:\.\d{1,3}){3}\b|\b[0-9a-f]{2}(?::[0-9a-f]{2}){5}\b/gi) ?? [], ["198.51.100.1", "198.51.100.2"]);
  }
});
// ---- draft end ----
