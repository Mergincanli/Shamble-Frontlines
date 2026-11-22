# infra-engineer: U8 build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22): draft U8's build brief rev 1 to `.checkpoints/netcode-proof-test/u8-brief.md`.
U8 = the run package without Safari: cloud-init, firewall, R1's run plan, operator files, wrappers, browser scripts.
No Rust; builds off the cargo lane beside U6a from Mon 23 Nov; needed by Stop B (~Tue 1 Dec) and R1 (Thu 3 Dec).
(No Edit tool in this agent: this file is rewritten whole with Write at each block.)

## Plan
1. Read memory (none for infra-engineer under .claude/agent-memory-local/), infra-engineer-digest.md. DONE (no memory dir)
2. Read replan.md, replan-units.md §3 §5 §7, plan.md:1256 + §7.2-7.3 + §12.2-12.3, security-r1-deps.md,
   advisor-u6a-brief.md, u6a-brief.md, history.md:7561-7607. DONE
3. Layout facts (plan.md §4.1 ops/ paths, §4.8 npm pins, ROUTE). 
4. Work out planned dates R2, R3, R5, close with arithmetic.
5. Write u8-brief.md (sections 1-8 of the task).
6. Hand back <=15-line summary.

## Notes (one line per finding)
- N1 replan.md:19 U8 1,360 (1,850-2,050); :27-39 runs R1 4/4 h 0.62/0.80, R2 11/11 1.72/1.90, R3 10/10 1.56/1.80, R5 17/56 3.34/3.60; his part: VMs, home address in firewall, one TXT per order (two in R1, ~2 min each), ~2 h present in R1.
- N2 replan.md:89-94 order: after R1 MOVED first, then R1 report, then U4k, U6d, U6e, U6j; R2, R3, R5. Stop B = U8's one round from U6c's merge.
- N3 replan-units.md §5 :151-171 U8 stays/goes list, size split (cloud-init 370, fw 40, R1.md 200, operator 180, hook 20, wrappers 150, specs+config 250, checklist 40, checks+tests 110). §7 :194-205 day table had R1 Wed 2 Dec; he chose Thu 3 Dec (history.md:7604).
- N4 replan-units.md §3 :107-121 DNS-01 by hand: npt-cert issue staging|production, TXT `_acme-challenge.rs.nt`, TTL 1 min, poll authoritative NS <=30 min, shortlived required in prod, no revoke, fw-rs loses TCP 80.
- N5 CONFLICT to flag: plan.md §7.3 step 9.1 (:898) and §8.4 (:961-964, :983) still say `npt-cert revoke` + TCP 80 http-01; superseded by replan-units.md:107-118 (approved RE-PLAN-OK (4), history.md:7600). U8 follows the re-plan; plan.md:914/:923 TCP 80 rows dropped. Not a contradiction of the re-plan; plan.md text is stale.
- N6 plan.md:1256 U8 row: S14 veth+netem (TCP-FREEZE, NETEM-BAR (a)), DelayStream loaded self-test (cond-10: 48 conn at P2, >=10,000 samples/dir, E<~10us, p99<=1ms), ImpairSocket R1 self-test (E L0=0 <=10,000 ns, >=1,000,000/dir), C-U8-1..-6; row also lists netcode checklist and W-PR, both cut by re-plan (replan-units.md:163-165).
- N7 plan.md:819 R1 row hours sum 2.95 to 4 incl. Safari 0.3 (goes); re-plan adds QUIC-blocked fallback + dev cert +0.45 h (replan.md:31, :98-99). 2.95-0.3+0.45 = 3.10 + C-U8-3 sizing, still under 4.
- N8 plan.md §12.3 laptop session: operator settings.json at ~/npt-operator, ask on every git push, denies gh/WebFetch/WebSearch/node .gate/*/ssh key read; start `claude --model opus --effort xhigh --settings ...`; Manual mode; redact-check pre-push from fixed copy with Stop B SHA-256.
- N9 plan.md:1268-1281 Stop B: review run package + four-attempt guard check + tree hash + his OK of settings.json and PROMPT-R1.md + Limits tab + R1 plan/cap/delete-by. W-PR part gone (Safari cut).
- N10 security-r1-deps.md:241-247 webdriverio 10.2.0 exact standalone, plain Node scripts, no @wdio/cli; SEC-R1D-2 allowedIps 127.0.0.1 for chromedriver/msedgedriver; -3 drivers by hand with SHA-256 via wdio:*Options.binary, browser binaries given; -5 minimatch accepted risk or override 11.0.9; -6 lock made with --before 7 days; npm ci --ignore-scripts, .npmrc ignore-scripts=true, integrity sha512, npm audit recorded. Binding: U8 lock = webdriverio 10.2.0 tree; extra install script or OSV hit returns to security (:256-257).
- N11 advisor-u6a-brief.md:233-250 conditions 2, 3, 4 (word for word); Q-U6A-5 (:82-92): U8's brief confirms util-linux taskset on the image; settings with no tick vCPU pin nothing.
- N12 u6a-brief.md:134-139 token file: per-slot tokens 16 bytes from /dev/urandom into a 0600 file for the run script (create_new, SEC-U6AD-3), never logged; room ends by duration or stdin close; exit 0/2/non-zero. :156-160 + cond 3: npt-super hold form keeps room until its own end (stdin held open).
- N13 u6a-brief.md:189-190: U8 sets system slices' allowed CPUs so tick vCPU free; U6a only reads.
