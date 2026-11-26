# security-auditor: U8-F1's one round (what leaves the R1 server), 2026-11-26

Status: IN PROGRESS (plan written first; findings appended as they land).

## Plan
- Inputs: wip/u8-f1-build.patch (sha256 26e0ec05...1755811478, checked: equal), base 77ffc95 files by git show, the
  built worktree files and HAND-BACK by Read only, u8-f1-brief.md section 5, advisor-u8-f1-brief.md C1-C7,
  advisor-u8-head.md sections 3-4, history.md:8068-8106.
- Q1: every printed value of 1a and 1b an integer (policy lines, `-1 -1`, `non_integer_fields`).
- Q2: the pin test's writers and address patterns; host name, routed IPv4/IPv6, hardware address each still caught
  between the pin test and redact-check.
- Q3: `attempts` and R1.md's wider sentence against S14-PUSH (a).
- Q4: the re-read line against SR-1/SR-2's words.
- Q5: Stop B's laptop read for Git for Windows and a real UTF-16 save.

## Notes (one line per finding as it lands)
- Q1 read (r1.sh:606-682, :718-722 in the worktree): every new value is a Python int (`type(v) is int`: no bool, float,
  text), names [a-z0-9_]+ only; pol() gives "<int> <int>" or "-1 -1"; the count line is an int; no new line can hold
  INVALID. P1's log missing: no P1 line at all (the leg's INVALID line says why): no exposure. l2_summary's fields
  (server/src/probe.rs:639-653, net/src/l2/names.rs) are counts and ns: none is an address, port or inode.
- Q2 read: redact-check.mjs:133-136, :219-249: a bare host name is never a finding (HOSTNAME fires only on an embedded
  non-VM quad); a MAC never (V6 candidate fails isIPv6); `--vms` (pre-push:5, :32-33: "the kept VM addresses") lets
  RS's own routed addresses through. So for RS's own host name, addresses and MAC the pin test is the only guard.
- Pin gaps found: r1.sh block filter (s14.test.mjs:224) misses heredoc/other-verb writers (r1.sh's own dist_check
  form, :139-157); probe.py values and prints unpinned (socket, subprocess imported; bursts runs as root in the root
  namespace, npt-s14:72); stamps.txt writers pinned by shape only (:230); `${OUT}` / unquoted `$OUT/` outside the
  kernel block; second `for s in` loop; no IPv6 literal form (repo redact test catches it, not by name); two checks
  lack the Mustafa message (:222-223, :213).
- Q3: `attempts` (r1.sh:105-107) = `attempt <n> <UTC>`; every item R1.md's sentence adds was in the folder at 77ffc95;
  U8-F1 adds no writer into s14/. Within "whole"; names no server.
- Q4: line = advisor-u8-head.md:141-143 word for word; residual: the push's shell is not named.
- Q5: Stop B read covers the Windows parts (matcher, baseline path); UTF-16 fails closed at the hook, but the hook
  scans every pushed commit (redact-check.mjs:321-341), so a UTF-16 commit needs a local rewrite before any push.
