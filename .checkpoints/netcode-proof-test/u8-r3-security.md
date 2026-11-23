# U8 last round (round 3): security seat, SEC-R2-1 and SEC-R2-2 fixes only, 2026-11-23

Plan: (1) check the snapshot's sha256 (0e7fbab2…) and that W equals it; read advisor-u8-r2.md section 3 items 2-3 and
:126-139; u8-build.md "## Pass 3" items 3.2, 3.3. (2) Interdiff rs.yaml, lg.yaml, cloudinit.test.mjs (2.7), r1.sh
(bringup only), r1sh.test.mjs (2.19) against .gate/u8-r3-old. (3) Read the container record u8-p3-sudo-l-file.txt.
(4) Verdicts SEC-R2-1, SEC-R2-2; new findings on these lines only; what I did not verify. Last line DONE.

W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof (read only).

## Verdicts
- **SEC-R2-1 = INF-R2-1 (High): CLOSED.** Both firstboots now make /etc/issue.d before writing into it (rs.yaml:117,
  lg.yaml:92). Test 1 asserts the order in both files, and the builder's plant fails it, as section 3 item 2 of the
  ruling words it.
- **SEC-R2-2 (High): CLOSED.**
  - bringup writes `sudo -n -l | cat > "$d/sudo-l.txt"` (r1.sh:484, under r1.sh:27's `set -euo pipefail`).
  - Test 7's 2.19 case asserts the pipe form. Its lbuf.c model, wrapped at 80 columns, turns the nine lines into 11,
    and that list must stop sudo_exact.
  - The container record shows the wrap into a regular file; the pipe form does not wrap (checked here).
  - The plant (the plain redirect back) fails the case. This is section 3 item 3 of the ruling, word for word.

## Findings table
| Id | Severity | Where | What | Smallest fix |
|---|---|---|---|---|
| (none) | | | Neither fix opens anything new on its changed lines | |

## Notes (saved as I go)
- N1 (resumed after the 17:33-21:10 UTC cut). wip/u8-build.patch sha256 0e7fbab2…5b15 OK (re-checked); W equals it
  (43 files, byte-equal to the patch's added text: scratchpad patchcmp.py); advisor-u8-r2.md d107cbe3…a784 OK.
  Ruling read: section 3 items 2 and 3, section 5 (:126-139). Builder's account: u8-build.md "## Pass 3" 3.2, 3.3.
- N2 SEC-R2-1, rs.yaml: the only hunk of the fix is rs.yaml:117 `install -d -o root -g root -m 0755 /etc/issue.d`,
  the fingerprint step's first command, before the write at :118-119. The other rs.yaml hunk (the curl
  `--speed-limit` lines at :178, :183) is pass-3 item 8, outside this round.
- N3 SEC-R2-1, lg.yaml: the same line at lg.yaml:92, before the write at :93-94 (the curl hunk at :136 is item 8).
- N4 Static check (PyYAML under `python3 -I`, read only): both files parse. In each firstboot the order is
  `set -euo pipefail`, then the ERR trap, then the mkdir, then the write (rs at lines 3, 9, 11, 13; lg at 2, 8, 10,
  12). rs.yaml is 12,489 bytes and lg.yaml 7,783, both under 32 KiB.
- N5 Security of the added line: root runs it on /etc/issue.d, whose parent /etc is root-only, so no user can plant
  there first. The file it guards is root-written. Nothing new is exposed. The trailing `# …` is a bash comment inside
  the YAML block scalar, so it does not change the YAML. agetty reads `*.issue` files there because /etc/issue exists
  (base-files).
- N6 Test 1's 2.7 case (cloudinit.test.mjs:163-164) asserts `\ninstall -d -o root -g root -m 0755 /etc/issue.d ` comes
  before `> /etc/issue.d/` in each role's firstboot (the parsed block, so lines start at column 0). The builder's plant
  (the line dropped from both) failed rs and lg 2.7 and was restored (u8-build.md Pass 3, 3.2).
- N7 SEC-R2-2, r1.sh: the only bringup hunk is r1.sh:482-484 (a two-line comment, then the pipe form). r1.sh's other
  hunks are pass-3 items 1 and 4-8, outside this round.
  - pipefail (r1.sh:27) keeps sudo's failure, and bringup runs with errexit active (the dispatch calls it bare).
  - Even a lost failure fails closed: an empty list makes sudo_exact exit 1, and bring-up dies.
  - `cat` comes from r1.sh's fixed PATH. Nothing else changed.
- N8 Container record, .gate/u8-p3-sudo-l-file.txt (246 bytes, 17:17). sudo 1.9.15p5, as root, no terminal, into a
  regular file: the Defaults entry is wrapped into three lines (`    env_reset, mail_badpass,` / `    secure_path=…,` /
  `    use_pty`). I ran the pipe form here myself (`sudo -n -l | cat`, read only): the same entry is one 126-character
  line. So on this sudo, wrapping is off through a FIFO and on into a file, as display.c reads.
- N9 Test 7's 2.19 case (r1sh.test.mjs, the SEC-R2-2 block):
  - Its `wrap` function follows lbuf.c's sudo_lbuf_println: break at the last space inside the width, else at the
    first space past it; continuations indented by the given indent, with width cols − indent; blanks skipped after a
    break.
  - Checked against the record's three Defaults lines (indent 4). With indent 8 it gives 11 lines for sudoers.d/npt's
    nine (peers split after `^peers(`, rooms before its NN group), matching my round-2 lengths (158 and 92), and that
    list must make sudo_exact fail.
  - The bringup regex asserts `  sudo -n -l | cat > "$d/sudo-l.txt"`. The plant (the plain redirect back) failed
    2.19 and was restored (u8-build.md 3.3).

## What I did not verify
- No VM: Hetzner's ubuntu-24.04 image's sudo build and /etc/issue.d state, agetty --reload, and whether the console
  shows the issue.d lines (the last is infra's display question, not this fix's).
- I re-ran no test and no plant (read only); the pass and fail counts are the builder's (u8-build.md Pass 3).
- The VM's privileges lines are read as the test models them. The merged-line and separate-line forms are both
  covered by 2.19; I did not see the VM's own `sudo -l` for npt.

DONE
