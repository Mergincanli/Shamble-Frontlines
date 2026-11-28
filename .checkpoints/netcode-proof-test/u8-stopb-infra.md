# U8 Stop B round: infra-engineer seat (xhigh)

Plan: (1) read brief 0.1, 6, 7, :19-23, ruling C1-C10/L1-L5, u6c section 5, build notes; (2) read W/ops cloud-init, firewall, systemd; settle npt-host.path restart question from systemd docs; (3) R1.md ops plan, r1.sh, PROMPT-R1.md; (4) builder items F13, F14, F27, F11, ADV-U8-11, ADV-U8-14, C4, C6, C7; (5) two ruling questions; (6) cost, exposure, provider-down, rollback; (7) verdict.

Inputs checked (sha256): u8-brief.md 3d102be9…f8f2, advisor-u8-brief.md 595521d9…410, u8-build.md cf07e435…5673, wip/u8-build.patch 60205020…d662 (42 new files, all mode 100644). W = M/.claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof. Every cite below is W's file:line unless it names another file. Judged against the re-plan (u8-brief.md:19-23, L3, R1.md:256-262), not the stale plan.md lines. Status: all parts done (2026-11-23; resumed after the 07:53-11:14 UTC cut with nothing lost).

## VERDICT: REVISE
1 High, 4 Medium, 8 Low. The High is a few lines in r1.sh's room_settings; it shares its fix with netcode's NC-SB-11. The rest of the package is sound for operations. C5's two halves, the chain form, per-VM delete-bys, the fixed operator allows (F23), and the explicit install modes are all good work. The sums check out: the hours (3.55), the cap (0.6248 and 0.781, so EUR 0.80), the sittings (22 + 33 + 106 = 161 min), the slot table (133) and C6's lines.

## Findings (most severe first)

### INF-SB-1 (High): Bring-up's `alone` room names a production chain that does not exist yet, so the server chain cannot start
- **Where:** r1.sh:131 (`transport=both` by default), :163 (every room names `/var/lib/npt/tls/production.chain.pem` and `production.key.pem`), :305-306 (bring-up's `room_settings 00 alone`, then `room_up 00`), :186-191 (room_up dies when no `room_start` arrives in 15 s), :582 (`chain` refuses without `bringup/done`). R1.md:78-80 and PROMPT-R1.md:57 (§2 step 10) run bring-up at 10:06-10:24. The certificate comes after it, at 10:24-10:39 (PROMPT-R1.md §3).
- **What fails:** U6b's room always binds both listeners. Its WebSocket certificate is either "the public chain and key files from U6c's `npt-cert`, each read under a size cap, or the room's own certificate" (u6b-brief.md:209-212). r1.sh always picks the public files. At bring-up those files do not exist yet: the production order has not run. Room 00 fails to start, and `room_up` dies with "room 00 did not open within 15 s". So bring-up never writes `done`, `alone` is never read, and `r1.sh chain server` refuses at 10:39. Nothing in PROMPT-R1.md tells the session to re-run bring-up after the certificate, so the session stops and the server blocks (headers, probe, calibrate, starts, ds, S14) are lost for the day.
- **How sure:** high that the settings name a missing file at bring-up. Medium-high that U6b's room refuses to start on it: U6b is not merged, and its brief's wording reads that way.
- **Smallest fix:** room_settings names the production pair only for the browser rooms. Every other kind (alone, headers, probe, calibrate, start, ds, bite) takes U6b's own-certificate form. load_settings' `ws` pin then reads the room's hash file whenever the room serves its own certificate. Fix pass item 5's own words already allow this ("the production order's SHA256 line when the room names the production chain"). The same change delivers R1.md:110-112's outage path (NC-SB-11). Test 7 asserts that no non-browser room names `production.*`.

### INF-SB-2 (Medium): A failed or hung bring-up has no bounded, cheap exit, and R1.md's provider-down line gets its billing wrong
- **Where:** rs.yaml:101-226 and lg.yaml:79-157 (firstboot runs once, as root, from runcmd; `npt` cannot re-run it). The fetches have no time bound: rs.yaml:170-171 and :175 (curl, no `--max-time`), rs.yaml:138 and :180, lg.yaml:109 and :138 (git fetch, no low-speed limit). rs.yaml:167-192 holds the whole build heredoc; its binaries are installed only after it, at :195-203. Also PROMPT-R1.md:35-40 (§2 steps 2-3) and R1.md:112-113.
- **What fails:**
  - **(a) A hang:** a stalled fetch writes neither build.failed nor build.done. cloud-init prints the host-key fingerprints in its final stage after runcmd's script (cloud.cfg.tmpl: scripts_user, then keys_to_console), so the console never shows the fingerprint that §2 step 2 needs before any strict SSH. The session therefore cannot read steps.log, and §2 step 3's "wait (5 minutes between reads)" cannot be reached. Nothing bounds this.
  - **(b) A failure:** it is first seen at 10:06, because he is away and the session runs nothing. R1.md:113's "30 minutes' wait, then teardown (under an hour billed per VM)" then runs to about 10:40. By then RS has lived about 1 h 10 and LG about 1 h 25, so each is billed 2 hours (Hetzner rounds up every started hour), about EUR 0.31. The wait buys nothing, since nothing on the VM retries. A same-day retry after it costs 0.31 + 0.62 = EUR 0.94, over the EUR 0.80 cap. Deleting both VMs by about 10:15 keeps a retry at 0.16 + 0.62 = EUR 0.78.
  - **(c) The provider list is too short:** R1.md:112-113 names GitHub and crates.io only. Bring-up also fetches from static.rust-lang.org, nodejs.org, registry.npmjs.org and the Ubuntu archive. Any failure inside the heredoc (web's `npm ci` included) skips the binary install, so an npm or Node outage costs the server blocks too, not only the page.
- **Smallest fix:**
  - Bound each fetch: `curl … --max-time 900 --retry 2`, and `git -c http.lowSpeedLimit=1000 -c http.lowSpeedTime=60 fetch …`. A hang then becomes build.failed.
  - R1.md 3's rule: on build.failed, or no build.done by T0 + 45 min (10:15), he deletes LG then RS at once. He re-creates them once the same day if the provider is back by about 10:45 (cap EUR 0.78); otherwise R1 moves to another day.
  - §2 step 2 states that the fingerprint appears only once cloud-init's final stage ends.
  - The provider list names all six sources.

### INF-SB-3 (Medium): Stop B fixes the kept Primary IPs to fsn1 without checking that CCX23 and CAX21 can be created there
- **Where:** R1.md:56-58 (Primary IPs and firewalls in `fsn1` at Stop B), :89 (both VMs in `fsn1`), :110 ("R1 waits (the kept addresses are bound to fsn1)"). r1.sh:158 and :232 hard-code `fsn1` in every record.
- **What fails:** Hetzner's public pages, read 2026-11-23, list no FSN1 under any CCX model (only NBG1, HEL1, HIL1, ASH1, SIN1) and none under CAX21's details (NBG1, HEL1). Every Create button carries "This product is currently unavailable" (https://www.hetzner.com/cloud/general-purpose, https://www.hetzner.com/cloud/cost-optimized). The notice sits on every product, CX included, so it is probably a page-rendering default and not stock. Even so, nothing in the plan checks availability before two location-bound addresses are bought. If fsn1 offers no CCX23 on Thursday, "R1 waits" waits on nothing.
- **How sure:** low on real unavailability; high that no step checks it first.
- **Smallest fix:** a new Stop B step before FIXED-IP: in the console's create form, he confirms that CCX23 and CAX21 are both offered in fsn1 (else nbg1, at the same German prices), then makes the Primary IPs there. The location becomes a fill that r1.sh's records read.

### INF-SB-4 (Medium): Nothing says the firewall rules hold only the /32s, and C5 proves port 22 and ICMP only
- **Where:** firewall/rs.txt:10-13 and lg.txt:20-21 (the lists end at "nothing else" in the brief). R1.md:56-58 (Stop B step 3) and :88 (09:10: "type his home IPv4 into the SSH rule and the two game-port rules"). r1.sh:289-302 (C5 checks LG to RS port 22, and RS's ping to LG).
- **What fails:** the steps say what to type, not what to remove. Two gaps:
  - Suppose the console starts a new firewall with SSH and ICMP rules from Any IPv4 and Any IPv6, or pre-fills a new rule's sources that way. Then typing a /32 next to "Any" leaves the rule open. This is from memory of the console: the firewalls overview, last changed 2021-12-23, describes no defaults (https://docs.hetzner.com/cloud/firewalls/overview/).
  - C5 catches an open port 22 or an open ICMP rule. It cannot see a game-port rule or the 9100 rule, because LG, the only vantage point outside his home, is admitted there.
  
  A leftover "Any" on UDP 30001-30032 or TCP 31000-31032 would therefore pass every check and expose every room, which is the case Q-U8-2's ruling names.
- **Smallest fix:** R1.md's Stop B step 3 and the 09:10 step read "each rule's sources are exactly the list's /32s: no Any IPv4 or Any IPv6, no default rule left". At 09:10 he reads each rule's source count aloud to the session (fw-rs 2, 2, 1, 1; fw-lg 1, 1), so no address goes into a file.

### INF-SB-5 (Medium): C7's Limits read counts two Primary IPs, but R5's servers need about six Primary IPv4s
- **Where:** R1.md:54-55 (C7 word for word: "RS plus five CAX21s and two Primary IPs").
- **What fails:** every server's public IPv4 is a Primary IP ("Up to 1 Primary IPv4 per server"; servers are created "with or without" Primary IPs: https://docs.hetzner.com/cloud/servers/primary-ips/overview/, read 2026-11-23). The test runs on IPv4, so each extra LG needs one. RS plus five CAX21s means six Primary IPv4s: the two kept ones and four made with the servers. If the project's Primary IP limit sits between two and six, R5's extra LGs fail at creation on Mon 14 Dec, after the read said they fit.
- **How sure:** medium that the Limits tab counts the auto-made ones.
- **Smallest fix:** the read becomes "RS plus five CAX21s, and six Primary IPv4s (two kept, four made with the extra servers)". C7 is the advisor's text, so the advisor confirms the change.

### INF-SB-6 (Low): npt-host.path restarts the host on every exit; `Restart=no` is not what holds it
- **Where:** systemd/npt-host.path:9; systemd/npt-host.service:13; r1.sh:77-81 (host_active).
- **Settled from the docs** (the builder's open question): yes, it restarts, and yes, only the start rate limit bounds it.
  - systemd.path(5) (noble, 255.4): "When a service unit triggered by a path unit terminates (regardless whether it exited successfully or failed), monitored paths are checked immediately again, and the service accordingly restarted instantly". The loop ends only when the service hits its start rate limit, which "causes the path unit to fail as well".
  - The defaults are 5 starts in 10 s (systemd-system.conf(5)), and they cover every kind of start (systemd.unit(5)).
  - So a host that dies at once fails both units within a second, which is good. A host that dies slower than about every 2 s is restarted forever, and host_active can read `active` between restarts.
  - The path unit also fires as soon as the file appears (it watches with inotify), so the chain must appear whole. That means U6c's issue mode writes it under a temporary name and renames it into place, which is U6c's to confirm.
- **Smallest fix:** add `StartLimitIntervalSec=300` and `StartLimitBurst=3` to npt-host.service's [Unit], so a flapping host fails both units for good. host_active also refuses when `systemctl is-failed npt-host.path` reports failed. The unit's header states the restart rule.

### INF-SB-7 (Low): The "0.45 h left" in R1.md is not real slack
- **Where:** R1.md:47 and :241.
- **What fails:** the 3.55 block hours end at 13:03. Teardown runs 13:05-13:25 (R1.md:84, :96), and LG is made about 09:17 (R1.md:89, :105). So the slack before a fifth billed hour is about 5-25 min on RS, depending on when in the teardown RS is deleted, and about 10 min on LG. It is not 27 min.
- **Consequence:** it stays inside the cap, which covers 5 h. But it matters for any block that grows (see both questions).
- **Smallest fix:** state the slack per VM against its delete-by.

### INF-SB-8 (Low): Bring-up never checks that its two packages installed
- **Where:** rs.yaml:12; npt-cert:56-57.
- **What fails:** a failure in cloud-init's packages module does not stop runcmd. If the mirror is down, `dig` is missing, and the first sign is "the ACME host did not resolve" at 10:24, a misleading message.
- **Smallest fix:** firstboot runs `command -v dig tcpdump || false` after the upgrade step, so the failure becomes build.failed.

### INF-SB-9 (Low): L4's fallback runs only when systemd refuses the setting (ADV-U8-14)
- **Where:** npt-ctl:80-89.
- **What fails:** systemd may accept `AllowedCPUs=` on init.scope and still leave PID 1's allowed list holding the tick vCPU. Then neither path moves PID 1, the read-back at :104 only counts it, and `alone` reads 0, so every timing block is INVALID.
- **Smallest fix:** after set-property, read `Cpus_allowed_list` in /proc/1/status. If it still holds the tick vCPU, run the taskset fallback as well, and record both.

### INF-SB-10 (Low): The provider-down text misses three cases
- **Where:** R1.md:17-18 and :110-113; browser/config.mjs:60.
- **The three cases:**
  - **GitHub raw down at 09:15:** the run worktree at X on the laptop holds the same files, and the user-data check compares against it anyway, so that copy is the fallback.
  - **Cloudflare's authoritative DNS down after the order:** every browser leg loads `https://rs.nt.shamblefrontlines.com:31000` (config.mjs:60), so all the browser legs stop, not only the order. LG pins by hash and needs no DNS. A hosts entry on the laptop and the PC would be his act only.
  - **Hetzner's console down at teardown:** the VMs keep billing at EUR 0.156 an hour, and no API token exists by design. The cap is passed about an hour past the spare.
- **Smallest fix:** one line each.

### INF-SB-11 (Low): The ledger read can wait forever
- **Where:** PROMPT-R1.md:62-64 ("if neither source answers, wait").
- **What fails:** crt.sh is often slow or down. With no bound, the certificate block can run into the server chain's slot.
- **Smallest fix:** wait at most 10 minutes, then it comes to him.

### INF-SB-12 (Low): The chain's deadline margin is shorter than its longest block
- **Where:** r1.sh:67-74 and :547.
- **What fails:** a block may start up to 10 minutes before RS's creation plus 4 h, but the probe block alone can run 14.2 min (852 s, r1.sh:382). The check also reads RS's boot only, while LG's delete-by comes about 10-15 min earlier. This only bites under an overrun.
- **Smallest fix:** a 20-minute margin, measured against the earlier of the two delete-bys (LG's, from §2 step 9's `uptime -s`).

### INF-SB-13 (Low): R1.md §6's dates rest on a basis the advisor has since dropped (C6)
- **Where:** R1.md:194-224.
- **What fails:** the arithmetic matches C6 word for word: 8,110-13,010, middle 10,560; 6.9, 9.0 and 11.1 lane days; U6e Thu 10 (Fri 11 at the top); U6j Sat 12 (Tue 15 at the top). Both inputs, the 1.35-1.5 factor and the 1,175 lines a day, are ones advisor-u6c-brief.md:262-270 says "no longer" hold. The conclusion survives on the u6c ruling's cadence: about 1.5 days per lane unit from Block 0 to merge, so 5 units take about 7.5 lane days from Fri 4 and U6j lands about Fri 11-Sat 12.
- **Smallest fix:** one sentence saying so.

## Builder items and conditions (confirmed or not)
- **F13 (default user):** user data's `system_info.default_user` merges over the image's. On Ubuntu that brings groups [adm, cdrom, dip, lxd, sudo] (cloud.cfg.tmpl), while `sudo: null` writes no rule. users_groups runs in the init stage, and firstboot's removal (rs.yaml:121-123, lg.yaml:98-100) runs in the final stage. Between them, `npt` sits in `sudo` (password locked, so it cannot use it) and in `lxd`. Only his key logs in, from his /32. Bring-up's `id -nG` must read exactly `npt`, and `sudo -n -l` is grepped for a full rule (r1.sh:286-288); `listpw=any` lets `-l` run without a password because NOPASSWD entries exist. Sound. Not verified on Hetzner's image: its own cloud.cfg might differ, but the checks read the end state.
- **F14 (dist build):** taken by U7's brief (u7-brief.md:59-60). rs.yaml:189-191 matches it, and `$HOME/wbg/bin` is on PATH (rs.yaml:178) for wasm-bindgen. The wasm32 target comes from rust-toolchain.toml. Closed.
- **F27 (file modes):** sound. rs.yaml:145-149, :199-203, :219-220 and lg.yaml:116-120 install with explicit modes. r1.sh runs through `bash`, the S14 rig through `python3`. Git for Windows treats a file starting with `#!` as an executable hook, so the copied pre-push needs no exec bit. Nothing runs from git's mode bits.
- **F11 (`node --test`):** Node 22's docs make test arguments glob patterns, to "be enclosed in double quotes ... to prevent shell expansion" (https://nodejs.org/docs/latest-v22.x/api/test.html, read 2026-11-23). The main session's `ops` step in check.mjs passes `'ops/**/*.test.mjs'` as one argument (no shell), never `ops/`.
- **ADV-U8-11 (LG's hours):** closed by L1. LG gets its own delete-by and goes first at 13:05 (R1.md:96, :105), so on the plan's timeline LG is billed 4 h. A 5th hour (EUR 0.0176) is inside the cap.
- **ADV-U8-14:** the fallback is built (npt-ctl:80-89); the residue is INF-SB-9.
- **C4 (RS-to-LG path):** built as ruled. rs.yaml:162-164 makes the key (0600, stays on RS). PROMPT-R1.md:47-49 pipes only the `.pub`. npt-ctl:179-191 writes it, root-owned with `from=`, `restrict` and `command=`, under lg.yaml:72's AuthorizedKeysFile. npt-load-ctl reads fixed verbs from SSH_ORIGINAL_COMMAND. A detached `setsid` load outlives the SSH call because Ubuntu's logind does not kill user processes by default; not verified on a VM.
- **C6:** arithmetic confirmed (INF-SB-13).
- **C7:** word for word at R1.md:54-55; the residue is INF-SB-5.
- **C8, C2, npm:** the firewall-profile read is at R1.md:66-67 and PROMPT-R1.md:23-25 (C8). Both forms of the npm command are at R1.md:62-63 and rules.md:48-49 (C2). The 4 moderates are at R1.md:63-64.
- **Sizes:** rs.yaml 11,375 bytes, lg.yaml 6,958, both under 32,768.

## The two questions for the ruling (from R1.md's own figures; not judging the fixes)
- **(1) Each LG-loaded block ends its room 30 s before LG stops (headers and ds, +30 s each; NC-SB-2's fix).**
  - **Hours:** +60 s (0.017 h). 3.55 becomes 3.57 h, still 4 h billed per VM.
  - **Timeline:** the server chain runs 10:39-11:40 instead of 11:39. By r1.sh's own durations, the six blocks take about 44-48 min of their 60-min slot, so no row has to change.
  - **His sittings:** unchanged, 161 min in three.
  - **Slot table:** unchanged, 133 (LG's bots take no browser slot).
  - **Cap:** EUR 0.80 holds; planned stays EUR 0.62.
  - **One day:** yes. With calibrate included it is +90 s, with the same answers.
- **(2) S14 grown to about ten times the stall episodes (NC-SB-4).** Today's S14 row is 0.30 h: 3 runs of (1 + 170) s, about 8.6 min, plus about 9.4 min of margin. Three forms:
  - **(a) Thirty runs as today:** 30 × 171 s = 85.5 min, so the row is about 1.58 h (+1.28 h) and the plan about 4.83 h.
    - **As placed now** (S14 last in the server chain, him away): chain 10:39-12:56, browser legs 12:56-14:14, bites 14:14-14:20, teardown 14:22-14:42.
    - **What breaks:** RS lives about 5 h 12 and LG about 5 h 25, so each is billed 6 h, EUR 0.93, over the EUR 0.80 cap. R1.md's "every block stops at RS's delete-by" (13:30) would cut the browser legs after 34 of their 78 min. deadline_ok (boot + 3 h 50) would refuse the bites.
    - **Verdict as placed:** does not fit one day within the cap.
    - **Re-ordered** (server chain without S14 10:39-11:21; browser legs 11:21-12:39; bites 12:39-12:45; LG deleted about 12:47 at 4 h; S14 as its own chain 12:45-14:20; RS deleted about 14:25 at 5 h): EUR 0.693 + 0.0704 = 0.76, inside the cap with no spare hour left on RS.
    - **What the re-order needs:** RS's delete-by and deadline_ok move to creation + 5 h. By the cap rule (planned + one spare hour per VM) the cap becomes 6 × 0.1386 + 5 × 0.0176 = 0.92, rounded to EUR 1.00, which is his to OK.
    - **His sittings when re-ordered:** 22 + 33 + 86 + about 10 = about 151 min, in four sittings.
  - **(b) Three runs with a 10-minute steady window (NC-SB-4's own example):** 3 × 721 s = 36 min, so the row is about 0.76 h (+0.46 h) and the plan 4.01 h.
    - **Timeline:** chain 10:39-12:06, browser legs 12:06-13:24, bites 13:24.
    - **What breaks:** deadline_ok refuses the bites (cutoff about 13:21), and both VMs pass their 4-hour delete-bys, so 5 h are billed each: EUR 0.78, inside EUR 0.80 but using the spare hour. Deleting LG right after the ping (about 13:02, so 4 h) gives EUR 0.76.
    - **His sittings:** 161 min, with the third now 12:06-13:52.
    - **One day:** yes, if the bites move before the browser legs or the delete-bys move to 5 h (cap rule: EUR 1.00).
  - **(c) Several namespace pairs at once (NC-SB-4's other form), or S14 beside the browser sitting:** hours, cap, sittings and timeline are unchanged, with no slack. This works only if netcode rules that S14's numbers are unharmed by the browser rooms on the housekeeping vCPUs.
  - **Slot table:** unchanged in every form (133).

## Section 7's lines
- **Cost per run (prices read 2026-11-23, before VAT):**
  - **The rates:** CAX21 EUR 0.0176 an hour (cost-optimized page). CCX23 EUR 86.49 a month at NBG1, which is EUR 0.1386 an hour at the pages' 624-hour ratio; the page shows one hourly rate, EUR 0.1178, for HEL1. Both equal plan.md:787-789. Every started hour is billed (billing FAQ).
  - **R1:** planned EUR 0.62 (4 h × 0.1386 + 4 h × 0.0176); cap EUR 0.80 (5 h each = 0.781).
  - **Extras:** the kept Primary IPv4s cost EUR 0.50 a month each (EUR 0.0008 an hour), at most EUR 1.00 for December. Traffic is EUR 0: RS to LG over public IPs in one network zone is internal and free, and home to RS is outgoing traffic inside the 20 TB included. A re-made bring-up costs EUR 0.16 per started hour of both VMs.
  - **CI:** U8 adds no workflow and no Actions minutes. The later `ops` step in check.mjs adds seconds of `node --test` and bash, and CI does not run check.mjs (advisor-u6c-brief.md:372).
  - **R2-R5:** EUR 6.62 planned, EUR 7.30 in caps; unchanged at today's prices.
- **What it exposes:**
  - **RS:** TCP 22 (key-only, `AllowUsers npt`, forwarding only to LG:22), UDP 30001-30032, TCP 31000 (the page host) and TCP 31001-31032, all from his /32 and LG's /32 by fw-rs only; nftables accepts them from any source (Q-U8-2; INF-SB-4). TCP 9100 is open to LG only, by both walls.
  - **LG:** SSH from RS only (fw-lg), for `npt` and the forced-command `npt-load`; ICMP from his /32.
  - **Public:** the A record `rs.nt` pointing at the kept address, and the production certificate's name in CT logs.
  - **The public repo (F8):** the run's date, hours, ports and users. It holds no address or secret, which test 6 and the pre-push hook enforce.
  - **On the VMs and the laptop:** user data holds no secret, and the metadata service is blocked for npt-room and npt-host (rs.yaml:54-55). The ACME key stays in one process's memory. The TLS key (0640) and the RS-to-LG key (0600) die with RS. Room tokens travel in URL fragments, on the USB stick (deleted after the run) and in the laptop's `.gate/`. The operator key stays until the close.
- **What breaks if a provider is down:**
  - **Hetzner:** if fsn1 refuses the VMs, R1 waits or moves location (INF-SB-3). If the console is down at teardown, billing goes on with no API way out (INF-SB-10). If the firewalls are not applied, C5 stops the run before any room opens.
  - **Cloudflare:** if the dashboard or the authoritative servers are down during the order, npt-cert fails within 30 min and the browser legs are lost. Down after the order, every browser leg stops because the page loads by name. The server blocks pin by hash and run on once INF-SB-1 is fixed.
  - **GitHub:** raw down at 09:15 means pasting from the run worktree at X. The clone failing at bring-up means build.failed or a hang (INF-SB-2). The results push failing means it waits; the results are safe in the laptop's `.gate/`.
  - **npm registry (or nodejs.org, static.rust-lang.org, crates.io, the Ubuntu archive):** a failure at bring-up loses the whole build, not only the page (INF-SB-2). The laptop's install happened at Stop B, so R1's day does not depend on npm there.
  - **Let's Encrypt:** the order fails and the run stops, so the browser legs move to another day (one more bring-up, about EUR 0.16). The server blocks can run only after INF-SB-1's fix (with NC-SB-11).
  - **Independent of any provider:** npt-super's buffered log (F28) makes every room_up fail on RS until it is fixed, a stop-item for the main session.
- **Rollback:** revert U8's merge commit on claude/netcode-proof-test-plan-hcky5g. U8 adds 42 new files under spikes/netcode-proof/ops/ and changes no tracked file, so the revert is clean; the main session's `ops` step in check.mjs reverts with it or alone. On the run day: delete LG, then RS, in the console. The kept Primary IPs, the A record and the two firewall objects stay until the close: the IPs at EUR 0.50 a month each, the firewalls free. At the close he removes the operator key from the project and deletes the laptop's settings, hook, drivers and node_modules. Results already pushed to claude/npt-results revert by a commit there.

## Not verified here
- No VM. Not checked:
  - the Hetzner image's own cloud.cfg (F13), and whether the VNC console shows cloud-init's /dev/console output (§1.6's fingerprint route);
  - CCX23 and CAX21 stock in fsn1 (INF-SB-3);
  - the console's default firewall sources (INF-SB-4);
  - init.scope's `AllowedCPUs=` (INF-SB-9);
  - the cold build time behind R1.md's 0.60 h (unmeasured; firstboot's stamped steps.log measures it on the day).
- U6b's and U6c's code is not merged, so the room's refusal of a missing chain (INF-SB-1) and the issue mode's write order (INF-SB-6) are read from their briefs.

## Sources read (date read 2026-11-23)
- systemd.path(5), Ubuntu noble, 255.4-1ubuntu8.18: https://manpages.ubuntu.com/manpages/noble/man5/systemd.path.5.html. Re-check and instant restart on the service's end; the start limit fails the path unit too; TriggerLimitIntervalSec 2 s and TriggerLimitBurst 200 by default; inotify.
- systemd-system.conf(5), noble: https://manpages.ubuntu.com/manpages/noble/man5/systemd-system.conf.5.html. DefaultStartLimitIntervalSec 10 s, DefaultStartLimitBurst 5.
- systemd.unit(5), noble: https://manpages.ubuntu.com/manpages/noble/man5/systemd.unit.5.html. The limits apply to every kind of start.
- Hetzner billing FAQ: https://docs.hetzner.com/cloud/billing/faq/ (last change 2020-07-07). Every started hour billed; billed while the server exists; same-zone traffic over public IPs is internal and free; unassigned Primary IPs billed; IPv6 free.
- Hetzner Primary IPs: https://docs.hetzner.com/cloud/servers/primary-ips/overview/ (2022-06-28). EUR 0.50 a month, EUR 0.0008 an hour; at most one IPv4 per server.
- Hetzner Firewalls: https://docs.hetzner.com/cloud/firewalls/overview/ (2021-12-23). Unmatched traffic is blocked; label selectors apply firewalls; no defaults described.
- https://www.hetzner.com/cloud/general-purpose and https://www.hetzner.com/cloud/cost-optimized (prices and the location lists as quoted in INF-SB-3 and section 7).
- cloud-init modules reference: https://docs.cloud-init.io/en/latest/reference/modules.html. `user` overrides default_user; `sudo: null`; scripts_user runs in the final stage.
- cloud-init cloud.cfg.tmpl: https://raw.githubusercontent.com/canonical/cloud-init/main/config/cloud.cfg.tmpl. The module order (keys_to_console after scripts_user) and Ubuntu's default_user.
- Node.js 22 test runner: https://nodejs.org/docs/latest-v22.x/api/test.html. Test arguments are glob patterns, quoted.

DONE
