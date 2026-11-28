# U8 Stop B round 2: infra-engineer seat (xhigh)

Plan: (1) check hashes; read the ruling advisor-u8-stopb.md :128-160, :227-285, :302-303 and u8-build.md "## Fix pass 2"; (2) interdiff old vs new ops/ for items 2, 3, 5, 7, 11, 16, 17 and INF-SB-1..-5; (3) recompute R1's hours, timeline, both delete-bys, sittings, slot table and cap with S14 0.80 h beside the browser legs and the LG blocks' new durations; (4) the builder's choices (LG as T0 - 15 min, the 20-minute margin and LG's deadline check, npt-host.service's start limit and sandbox); (5) verdict, INF-R2-n findings.

Inputs (sha256 checked): advisor-u8-stopb.md daf55033…9192; wip/u8-build.patch bca52b2c…6eb7; u8-build.md 6a67751d… (its "## Fix pass 2" read). W = M/.claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof; old = M/.gate/u8-r2-old/spikes/netcode-proof. I read only the interdiff of ops/; cites are W's. Status: all parts done, 2026-11-23.

## VERDICT: REVISE (one new High in a changed line, INF-R2-1: a one-line fix); otherwise APPROVE
INF-SB-1 is closed, and so are all four of my Mediums. New findings: 1 High, 5 Low.

## Closed or not
- **INF-SB-1 (High): closed on U8's side.** room_settings names the production pair only for browser rooms. Every other kind, `alone` included, writes the null pair, the room's own certificate (r1.sh:222, :233-236, :261). load_settings pins the production ledger only on WebSocket to a room that names a chain; otherwise it pins the room's hash file (r1.sh:326-329). Bring-up's room 00 therefore opens before any certificate exists, and the server chain depends on no production file. Test "2.2 INF-SB-1" checks every kind and ds's pin from the hash file (r1sh.test.mjs:295-308), and plant P11 failed it. Still open by the ruling: U6b's field form for the null pair and the WebSocket pin's hash file, both checked at U6b's merge (advisor-u8-stopb.md section 6).
- **INF-SB-2 (Medium): closed.**
  - The fetches are bounded: curl `--max-time 900 --retry 2` (rs.yaml:177, :182; lg.yaml:135) and git's low-speed limit on every fetch in both files.
  - The rule is in R1.md:128-135: the six providers, build.failed or no build.done by 10:15 means both VMs are deleted at once, one same-day retry by 10:45 (EUR 0.16 + 0.62 = 0.78, inside the cap), and no console fingerprint means no SSH.
  - PROMPT-R1.md §2 step 3 carries the 10:15 rule.
  - Residue: INF-R2-4 (Low). INF-R2-1 is a defect in the new fingerprint step, not in this rule.
- **INF-SB-3 (Medium): closed.** Stop B step 3 checks the create form in fsn1 first, with nbg1 as the fallback (R1.md:61-63). LOCATION is a fill (r1.sh:34) and feeds the records (r1.sh:257, load_settings :319).
- **INF-SB-4 (Medium): closed.** R1.md:66-67 reads "exactly the list's /32s: no Any IPv4 or Any IPv6, no default rule left". The 09:10 step reads the source counts aloud: fw-rs 2, 2, 1, 1 and fw-lg 1, 1, which match firewall/rs.txt:10-13 and lg.txt:20-21.
- **INF-SB-5 (Medium): closed.** R1.md:56-58 now asks for six Primary IPv4s ("the two kept, four made with the extra servers"), and test 5 checks 1 + 5 = 2 + 4.
- **My Lows on these items:**
  - Closed: INF-SB-6 (npt-host.service:4-8, :10-11; r1.sh:124), INF-SB-8 (rs.yaml:134), INF-SB-10 (R1.md:136-140), INF-SB-11 (PROMPT-R1.md §3 step 1), INF-SB-13 (R1.md:249-251).
  - Closed in the main text, with one stale line each: INF-SB-7 (R1.md:47-50) and INF-SB-12 (r1.sh:71-81). Both stale lines are INF-R2-2.
  - INF-SB-9 is item 18, the security seat's.

## R1's day after S14's 0.80 h and the LG blocks' new durations (R1.md's figures, checked against r1.sh)
- **Hours:** the rows sum to 4.05. S14's last 0.50 h runs beside the browser legs, so the day's span is 3.55, still 4 h billed per VM (R1.md:44-50; test 5's sum minus the overlap).
- **What r1.sh actually takes:** the LG blocks now run LG for the room's duration plus 30 s and wait for room_exit, then LG (r1.sh:362-385). That gives headers about 6 min against its 9-minute row, ds about 7.75 against 9, and calibrate about 6 against 6, at the edge of its 0.10 h row. The probe takes about 12.7 min (14.2 at its bound) against its 0.20 h, and starts about 1.5 min. So the five blocks take about 34-36 min of their 42-minute slot (10:39-11:21).
- **S14:** 3 × 951 s plus the summary, about 48 min (S14_S 2,880 s). It runs from about 11:13-11:21 to about 12:01-12:09, beside the browser legs at 11:39-12:57, overlapping them by about 22-30 min.
- **S14 re-run:** a VOID or INVALID S14 re-runs only if it would end 5 min before boot + 3 h 27 min, about 12:58 (r1.sh:654). So a re-run ends by about 12:53 and never pushes the bites (12:57). The bites chain refuses to start while the server chain runs (r1.sh:103-115).
- **Delete-bys:** unchanged, RS 13:30 and LG about 13:15-13:20. The chain's cutoff is RS's boot + 3 h 40 min, about 13:10, so the 12:57 bites keep 13 minutes. LG blocks are cut off at about 12:55-13:00, and they run 10:39-11:15.
- **His sittings:** 22 + 33 + 106 = 161 min, unchanged (R1.md:114). One more prompt falls inside the third sitting: the early collect (PROMPT-R1.md §4).
- **Slot table:** 133, unchanged.
- **Cap:** EUR 0.80, planned EUR 0.62, unchanged. The slack is 5 min at worst on RS and about 10 min on LG (R1.md:47-49).
- **R1 still fits one day.**
- **The builder's choices:**
  - LG taken as made 15 min before RS (r1.sh:78-79): sound. R1.md:89's 09:15 creation step puts LG within about ±2 min of it, well inside the 20-minute margin, and §2 step 9 records the real value.
  - The 20-minute margin, with LG blocks checked against LG's own delete-by (r1.sh:80-81): sound.
  - npt-host.service's limit of 3 starts in 300 s (:10-11): sound; INF-R2-3 is only its wording.
  - The sandbox subset (:21-26): ProtectHome, PrivateTmp, an empty bounding set (TCP 31000 needs no capability), RestrictNamespaces, LockPersonality and SystemCallArchitectures=native. All are safe for a read-only server on x86_64 Ubuntu 24.04; the rest waits for U6c's merge, as ruled.

## Findings (most severe first)

### INF-R2-1 (High): The new fingerprint step writes into /etc/issue.d, which the image likely lacks, so both builds fail at their first step
- **Where:** rs.yaml:116-119 and lg.yaml:91-94: `{ echo …; for k in …; do ssh-keygen -l -E sha256 -f "$k"; done; } > /etc/issue.d/50-npt-hostkeys.issue`, under `set -euo pipefail` with the ERR trap (rs.yaml:115, lg.yaml:90).
- **What fails:** nothing in firstboot creates /etc/issue.d. In the Ubuntu 24.04.5 container it does not exist and no installed package owns it; base-files owns /etc/issue only (`ls -ld /etc/issue.d`, `dpkg -S`). No noble package ships a file under /etc/issue.d/ (https://packages.ubuntu.com/search?searchon=contents&keywords=issue.d&mode=filename&suite=noble&arch=any, read 2026-11-23). If the directory is missing on Hetzner's image:
  - the redirection fails, the group never runs, and the command returns 1;
  - the ERR trap writes "FAILED at line 117" (RS) or "FAILED at line 92" (LG) and build.failed, and `set -e` ends firstboot;
  - both VMs fail about 2 minutes after boot, before the clone;
  - at 10:06 the session reads FAILED and R1.md:131-133 deletes both VMs. A same-day retry fails the same way, because the files come from commit X. R1's day is lost.
- **How sure:** medium-high that the directory is absent; certain that the write then fails.
- **What does work:** agetty does read /etc/issue.d once it exists, because /etc/issue exists ("If the file exists, then agetty also checks for /etc/issue.d directory"; files with `.issue` are printed in version-sort order). And getty runs during firstboot, because cloud-final.service has `After=multi-user.target` (https://raw.githubusercontent.com/canonical/cloud-init/main/systemd/cloud-final.service). So only the missing directory breaks the step. Source for agetty: agetty(8), util-linux 2.39.3-9ubuntu6.6, https://manpages.ubuntu.com/manpages/noble/man8/agetty.8.html.
- **Smallest fix:** `install -d -o root -g root -m 0755 /etc/issue.d` as the step's first command in both files. Test 1 asserts the directory is made before the write (plant: drop it).

### INF-R2-2 (Low): Two stale numbers from fixes made elsewhere in the same files
- **Where:** PROMPT-R1.md:92-93 still says the chain "stops itself 10 minutes before RS's delete-by". It is now 20 minutes, and LG blocks are also checked against LG's delete-by (r1.sh:71-81, R1.md:124-125). R1.md:290 still says "inside the 0.45 h left", which INF-SB-7 replaced at R1.md:47-50 with the per-VM slack.
- **Smallest fix:** "20 minutes before RS's delete-by (an LG block, LG's)"; and "inside RS's 5 minutes at worst: re-checked at U6c's merge".

### INF-R2-3 (Low): npt-host.service's header is off by one
- **Where:** npt-host.service:5 says "a third start inside 300 s fails this unit".
- **What is true:** with `StartLimitBurst=3`, three starts are permitted and the fourth is refused ("Units which are started more than burst times within an interval time span are not permitted to start any more": systemd.unit(5), noble). That means the path unit's start plus two restarts; the behaviour itself is right.
- **Smallest fix:** "a fourth start".

### INF-R2-4 (Low): curl's bound can outlast R1.md's own 45-minute rule
- **Where:** rs.yaml:177, :182; lg.yaml:135; R1.md:131 ("so a hang becomes build.failed").
- **What fails:** `--max-time 900 --retry 2` allows 15 min per attempt, and curl retries a timeout as transient, so one stalled file can take about 45 min before build.failed. Until then a stall reads as "no build.done by 10:15", which reaches the same teardown, so this is wording plus a slower signal. `rustup toolchain install` and `cargo install` keep their own tools' timeouts; the 10:15 rule covers them too.
- **Smallest fix:** add `--speed-limit 1000 --speed-time 60` (git's form), so a stall fails in about a minute.

### INF-R2-5 (Low): The fingerprint comes before firstboot's fetches, not before the VM's
- **Where:** rs.yaml:9-12, lg.yaml:8 (package_update and packages), against rs.yaml:116 and lg.yaml:91 ("before any fetch").
- **What fails:**
  - cloud-init's package_update_upgrade_install runs first in the final stage, before scripts_user (cloud.cfg.tmpl), so apt's update, and on RS its two installs, come before the step. A slow mirror delays the fingerprint toward the 15-minute teardown rule (PROMPT-R1.md §2 step 2).
  - cloud-final's output also goes to the console (StandardOutput=journal+console, the same unit file), so later output can scroll the lines away. Enter at the login prompt redraws the issue.
- **Smallest fix:** install the two packages inside firstboot after the fingerprint step (`apt-get update && apt-get install -y bind9-dnsutils tcpdump`, before the `command -v` check), and drop package_update and packages. §2 step 2 adds "press Enter on the console to redraw the lines".

### INF-R2-6 (Low): §2 step 9 gives no command for LG's boot time in epoch seconds
- **Where:** PROMPT-R1.md:68-69 (`r1.sh lg-boot <LG's boot in epoch seconds>`).
- **What fails:** the session must convert `uptime -s` (local text on the VM) by hand on Windows, which leaves room for a time-zone slip. The damage is bounded: r1.sh:783 checks 10 digits within 6 h, the margin is 20 minutes, and the LG blocks end about 2 h before LG's delete-by.
- **Smallest fix:** `ssh -F <ssh> npt-lg grep btime /proc/stat`, which prints the boot time in epoch seconds.

## Sources read this round (2026-11-23)
- agetty(8), noble, util-linux 2.39.3-9ubuntu6.6: https://manpages.ubuntu.com/manpages/noble/man8/agetty.8.html. /etc/issue.d is read when /etc/issue exists; `.issue` files in version-sort order; `--reload` updates the prompts of running agettys (needs inotify).
- cloud-init systemd/cloud-final.service, main: https://raw.githubusercontent.com/canonical/cloud-init/main/systemd/cloud-final.service. `After=multi-user.target`; `StandardOutput=journal+console`.
- packages.ubuntu.com contents search for "issue.d" in noble (URL above): no file under /etc/issue.d/.
- From round 1, still in force: systemd.unit(5), systemd.path(5), cloud.cfg.tmpl (u8-stopb-infra.md, "Sources read").

DONE
