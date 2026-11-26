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
