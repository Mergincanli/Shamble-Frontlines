# Advisor: the plan check of Stop B's preparation (stopb-plan.md, DRAFT, pushed in 3ce8c63), 2026-11-27, effort xhigh

Plan: main's advisor.md (unchanged since c9bd268; main b8f02b01): plan review list, risk tiers, escalation categories.
Read stopb-plan.md whole. Then its why: history.md:8163-8186 (NO WAITING FOR WINDOWS) and the last entry (his 21:35
UTC word); R1.md:94-95, :308-322, :342-343, section 2 step 8 and section 8; U8-FILLS (a) at history.md:8137.
Item 1: A1-A4 against advisor-u8-f1-head.md:57-79, PR #40's closing list, u8-stopb-security.md:134, advisor-u8-stopb.md:324,
u6c-report.md, u6c-build.md:11; then a Grep for "before Stop B", "at Stop B", "before R1" over the card,
u8-report.md, u6c-report.md, advisor-u8-head.md and advisor-u8-stopb.md. Item 2: B1-B2 against u8-f1-build.md:369-382
and advisor-u8-f1-merge.md:58-73. Item 3: C1-C3 against rs.yaml:174-221 (and lg.yaml's build block) and R1.md:313-318.
Item 4: D1-D4 against R1.md sections 8 and 2. Item 5: E against u8-brief.md 6.2-6.4, answer-1914.md and the card's
LEAN-MODE Stop B line. Item 6: Q1-Q6. Then the verdict, findings on the scale, Decisions needed, For Mustafa, status.
Reading with Read, Grep and Glob; Bash only for single git commands; nothing run, built or fetched.

## Notes (one line per check)
- stopb-plan.md (Read whole, 89 lines): steps 0, A1-A4, B1-B2, C1-C3, D1-D4, E1-E2, Size, Q1-Q6.
- history.md:8163-8186 is his NO WAITING FOR WINDOWS, which keeps "only R1's path, at most three agents, each finished
  block saved at once". His 21:34 and 21:35 words ("Continue with your work"; "There is plenty of usage, just continue
  after the planning too. No breaks, chop chop") are at :8241-8257. The plan's "Why now" quotes them rightly.
- R1.md:16-20, the commit pair: "rs.yaml and lg.yaml name the run's code commit Y (`/etc/npt/commit`); they live at X =
  Y plus that line"; `npt-userdata head` must show Y for both clones. R1.md:57-95 is his "Once" list; step 8 at :94-95.
  Section 8 at :308-322 has the fills, SEC-SB-5's list "committed at X" (:313) and the probe hours (:319-322, "(no
  reading exists yet)"). R1.md:351-353 is "Stop B's round": security-auditor and infra-engineer at xhigh, netcode at
  xhigh on sections 4-5, S14 and the probe block; three at once, one round.
- **KEY, rs.yaml:121-148 and r1.sh:30-33 (Read):** RS's root clone /opt/npt/clone is fetched at COMMIT =
  /etc/npt/commit = Y. r1.sh runs from that clone (`SELF=/opt/npt/clone/...`) with
  `EXPECTED_DIST=$SPIKE/ops/runs/r1-dist.sha256`, `SPIKE=/opt/npt/clone/spikes/netcode-proof`. So on RS the expected
  list is Y's copy, never X's.
  - dist_check (r1.sh:136-157) writes "(the expected list is empty: Stop B fills it)" into DIST-DIFFERS when the list is
    empty.
  - host_active (:127-135) then refuses every page.
  - dist-accept (:875-879) refuses an empty list ("the expected list is empty: Stop B's fill is missing").
  - r1sh.test.mjs:448-452 pins exactly that for the committed, unfilled list.
  - So a list filled only at X (stopb-plan D2; R1.md:313; r1.sh:136's comment; SEC-SB-5's fix text at
    u8-stopb-security.md:105 and my own advisor-u8-stopb.md:192) leaves R1's browser legs with no page on the day.
- web/build.mjs (Grep): no `git`, HEAD or commit read and no ops/ path. :164 refuses a wasm that holds the cargo home,
  SPIKE, ROOT or home paths. So ops/runs/r1-dist.sha256 does not feed the dist.
- PROMPT-R1.md:10, :22: the laptop's `<run>` worktree is at X ("`git -C <run> rev-parse HEAD` is X").
- A1 source, advisor-u8-f1-head.md:57-79: the plan's A1 matches it word for word in substance (pageUrl, parseLink from
  .gate/netcode-proof-test/web-check, ok with room, transport and token, one line per link and a count, a mismatch to
  me with the lines before Stop B).
- A2 sources:
  - PR #40's closing list (read for B5): "Also open, owned by Stop B's host read before R1 (U6c, the page host): no
    handshake or read deadline and no connection cap ... With no LimitNOFILE in U8's unit, about 1,020 idle
    connections ... can lock the host out".
  - u8-stopb-security.md:134-139, SEC-SB-9: the eleven sandbox options.
  - npt-host.service (Read): has NoNewPrivileges, ProtectHome, PrivateTmp, CapabilityBoundingSet=, RestrictNamespaces,
    LockPersonality and SystemCallArchitectures. Its :6-8 say the rest (ProtectSystem=strict, PrivateDevices,
    MemoryDenyWriteExecute, SystemCallFilter, RestrictAddressFamilies) "comes from the host's needs at U6c's merge".
  - u8-report.md:65-66: else "carried to R2 with R1's `systemd-analyze security` reading".
- **PR #40's closing list (body, read at B5) lacks three entries.** It names none of SEC-U8F1-3, SEC-U8F1-4 or SEC-SB-9's
  rest, which u8-report.md:131 sends "for PR #40's closing list". The plan's B2 has the list "lose" -3 and -4.
- A3 sources:
  - The reading the plan says is missing EXISTS. probe_reading.py (.checkpoints/netcode-proof-test/) was the main
    session's F-READINGS (1) script, advisor-u6c-r1.md:73-81.
  - u6c-r2-netcode.md:90-93, "Reading 3 (U8 timing)": "measured legs ≈ 112 s ...; six legs ≈ 675-680 s against U8's
    732 s and LG's 792 s. Nothing moves 122 s or 0.20 h". :162 repeats it, weighed by the netcode seat.
  - NC-SB-16's count was read there too (:45-50): the series counts records outside the burst in measured seconds.
  - The probe and load code since U6c's merge 17964a3d: only load.rs:44-45 changed (U7's MAX_LEAD constant, bots only;
    `git diff 17964a3d 69682158`). So the reading stands for today's probe.
  - R1.md:321's "(no reading exists yet)" is stale.
- Owed-items Grep ("before Stop B", "at Stop B", "before R1", "Stop B's fill", with -o context) over the card,
  u8-report.md, u6c-report.md, advisor-u8-head.md, advisor-u8-stopb.md, then the sources they point to. Owed at Stop
  B's fill and NOT in the plan:
  - (i) **ADV-U8H-3:** "the main session reads the delay clock thread's vCPU before Stop B's fill" (u8-f1-brief.md:83;
    card :161). R1.md:243-245 still reads "DelayStream's self-test may join the run list at U6b's merge, if its pump
    runs on the io threads". That read decides whether ds-selftest stays in the alone-0 skip list (R1.md:243;
    r1.sh:38 TIMING_BLOCKS).
  - (ii) **TE-5 (T5i-1):** "one WebSocket leg without ignoreHTTPSErrors before R1, to record why every WebSocket load
    shows handshake_failed 1 beside accepted 1: Stop B's fill, by the main session" (u7-report.md:190-191;
    advisor-u7-round.md:27).
  - (iii) **NC-3's Stop B read:** `npt-ctl quic off` must DROP, never REJECT (u7-report.md:157-160, :195). I read it now:
    - npt-lib.sh:10 adds ports to the set `quic_off`;
    - rs.yaml:45-46 drop them (`udp dport @quic_off ip saddr != @metrics_src drop`; the IPv6 line drop);
    - a case-blind Grep for "reject" over ops/ finds no nft rule.
    - SATISFIED by this read.
  - (iv) **ADV-U7S-6's guard** (u7-report.md:198-203): "at Stop B's fill, if `git diff --stat 9c2fcf4 <then> -- r1.sh
    load.rs` is not empty, the advisor reads the pairing again".
    - It is not empty: r1.sh 17/6, by U8-F1.
    - My read of the hunks (`git diff -U0`): logq's `-` mode, probe_presence's P1 integers and pol(), and
      ds_selftest's ws_split. No hunk touches load_settings, and load.rs is unchanged.
    - So the pairing read at U7 (24 keys, machine's 5) stands. SATISFIED by this read, recorded here.
  - Already done: the Q-A/Q-B net-zero rewording (PROMPT-R1.md:127 reads "… checked, 0 finding(s)` ... (an empty file
    included)"); u8-report.md's owed record line (:145).
  - Day-of items, not the fill's: his laptop reads, `sudo -l` on RS, fsn1 stock (u8-report.md:68-69).
  - Not before R1: U6c's slot 41 (after R1); ADV-U7S-5 (config.mjs:60 comment, U8's next change); NC-R3-1 (follow-up).
- B sources:
  - u8-f1-build.md:369-382: SEC-U8F1-3's text is "Then, in the same PowerShell, push to `claude/npt-results` after the
    pre-push hook passes (a prompt)", which makes the step's 7th line, +1. SEC-U8F1-4's second part, R1.md:86-88's
    start-check line ending "its printed path and `git --version` are recorded", is a new line because :88 is at 120
    columns, +1.
  - u8-f1-build.md:383-385: the last re-wrap broke r1plan's pattern once; it was fixed by `\s+` in place.
  - advisor-u8-f1-merge.md:58-63, C2: "both go onto PR #40's closing list now, as security findings left open ... If
    they are built later, the line that builds them takes them off the list, naming its commit."
  - PR #40's body does not hold them (Notes above). The plan's B matches the texts and lines.
- C sources:
  - rs.yaml:174-221 (Read): npt-build under `env -i HOME PATH COMMIT REPO`, rustup 1.29.3 (x86_64, its sha256), node
    v22.23.4 (its sha256); then the clone at COMMIT; `rustup toolchain install`; `CARGO_TARGET_DIR=$HOME/.gate/wbg-target
    cargo install wasm-bindgen-cli --version 0.2.130 --locked --root $HOME/wbg`; `cargo fetch --locked`; the workspace
    `--bins` build and the bites build (:196-197); then in web/ `npm ci --ignore-scripts` and
    `CARGO_NET_OFFLINE=true node build.mjs` (:198-200). :221's list form runs before :224's ssh-host-keys.txt is added.
  - web/build.mjs:3-7, :141-172 (Grep): it checks wasm-bindgen 0.2.130 itself, then builds npt-client-wasm for
    wasm32 with `--locked --offline` in its own `--target-dir` under .gate/. It remaps CARGO_HOME, the spike folder
    and that target, scans the Wasm for home paths, and enforces the seven DIST_NAMES. So :196-197 do not feed
    web/dist.
  - Build inputs since U7's merge 69682158 (`git diff --stat` over crates, web, Cargo.lock, Cargo.toml,
    rust-toolchain.toml, .cargo): only crates/judge (eval.rs, marks.rs, tests/engine.rs).
  - u7-report.md:132-141 lists the seven measured-dist SHA-256s of U7's run 4. hooks.js differs in the bites dist; RS
    builds the measured one (no --bites).
- The repository is public (`gh api repos/{owner}/{repo}`: private false), so the raw URL and the VMs' anonymous
  `git fetch` of X and Y work by SHA.
- D sources:
  - R1.md section 2 step 7-8 and section 8 (above).
  - cloudinit.test.mjs:126 pins `/etc/npt/commit` to 40 hex digits.
  - r1plan.test.mjs:149-150 pin r1-dist.sha256's first line and R1.md's "committed at X, filled at Stop B from two
    equal\n  builds".
  - **r1sh.test.mjs:448-449 run dist_check on the COMMITTED list and assert DIST-DIFFERS says "the expected list is
    empty". Filling the list makes :449 fail, wherever it is filled.** The plan's D2 bar ("the ops tests ... pass") cannot
    hold unless that test changes.
  - r1-dist.sha256:1-4 (Read): a 4-line comment that itself says "committed at X".
- E sources:
  - u8-brief.md:408-473 (6.2-6.4, 7).
  - LEAN-MODE B (history.md:7868-7871): "In your Stop B message (R1's run plan, about 1 December), one screen", with
    items 1-3: each piece's estimate and likely size at about 3.3 times; "whether R2, R3, R5 and the close of 18 December
    hold"; for (a)-(c) "What each of these would save in lines and days. I have decided none of them"; "Your own lean on
    each, one line".
  - His 19:14 item 2 (history.md:8033): "a short list of the pieces after R1 that Netcode Builder 2 could build alone on
    its own branch. For each: its folder fence, its planned size in added lines, and what it must wait for."
  - answer-1914.md item 2: the draft rows.
  - u8-brief.md section 7 (:465-473): "Stop B's read is U8's one round", after U6c's merge, reading "U6c's host" among
    the rest. The round ran before U6c merged (advisor-u8-stopb.md, 2026-11-23), so U6c's host is the part still owed:
    that is A2.
- u8-brief.md:5-6: "R1.md's hours and slot table filled after U6c's merge (about Sat 28)", U8's own brief naming those
  hours a fill.
- **browser.test.mjs:205-208 (Read):** `const rs = text("cloud-init/rs.yaml")`, then
  `assert.throws(() => config.codeCommit(rs), /not filled/)`, and :208 replaces the zeros. With Y filled into rs.yaml
  (X), :207 no longer throws and :208's replace finds no zeros, so both fail at X. That is a second pin the fill breaks.
  config.mjs:101 codeCommit is what lib.mjs:129 reads on the laptop at X. No test pins R1.md:92-93's URL placeholder
  (Grep `termsOfService|production ACME` over ops/).
- After the advisor tool's check, these reads:
  - The owed-items Grep (-o) over u8-report.md, u6c-report.md, advisor-u8-head.md and advisor-u8-stopb.md. It adds
    nothing beyond the items above; u6c-report.md:133 is the page-host item (A2, "with LimitNOFILE in U8's unit"), and
    :152 is NPT-EDGE-SLIVER, answered (a) by him (card :95).
  - PR #40's body, re-read by `gh api repos/{owner}/{repo}/pulls/40 --jq .body` at 21:5x. Its "Closing list: security
    findings left open" still names only SEC-U6B-3, SEC-U6C-R2-1, SEC-U6C-R2-2 and the page-host item: no
    SEC-U8F1-3, SEC-U8F1-4 or SEC-SB-9's rest.
  - config.mjs:100-105: codeCommit takes the 40 hex digits after `- path: /etc/npt/commit` and throws "not filled" on
    zeros.
  - PROMPT-R1.md:18-23, start checks 2 and 4. On R1's day `<run>` must be X, and `git -C <run> diff --name-only
    origin/main -- CLAUDE.md .claude` must print nothing, "On a mismatch, stop". So a kit move on main after X is made
    means a new X before R1.
  - main-run.sh:17-18 export CARGO_TARGET_DIR and `CARGO_NET_OFFLINE=true`, so rs.yaml's `cargo fetch` and `cargo
    install` cannot run through it.
  - The existing probe reading's limits, read:
    - It ran before H1 moved the start-edge read (u6c-report.md:34-36).
    - It read P0 and P1 only; "P2, P3, P4 and B1's profiles ... were not read; reading 3's bound holds for any
      profile with one-way plus half jitter under about 9 s" (u6c-r2-netcode.md:237-238).
    - No line there names NC-SB-16. My link is an inference from :40-41 ("keep until `outside` reaches `packets`").
      lab/src/probe.rs:124-131 counts `outside` only for sends not `in_burst`; u6c-build.md:11 says leg_series builds
      NC-SB-16's count, with tests.
- The coordinator's ADV-U8H-3 read (adv-u8h-3-read.md, present): the delay clock thread never runs on the tick vCPU
  (delay_clock.rs:92-99; main.rs:737, :841, :244, :655; npt-ctl:71; tick_loop.rs:1317-1323). Not settled: whether
  ds-selftest's judged numbers depend on the tick thread's send stamps. Moving ds-selftest out of TIMING_BLOCKS would
  be a U8 line.
- After the advisor tool's second check:
  - The owed-items Grep widened to u6a-report.md, u6b-report.md, um4-report.md, um4-plan.md, advisor-u7-head.md,
    advisor-u6c-m4b.md, advisor-u6c-r2.md and msg-2026-11-28.md. Nothing owed that moves Y:
    - advisor-u6c-m4b.md:253-255 is NPT-EDGE-SLIVER's (b), answered (a);
    - advisor-u6c-r2.md's items are H1's, merged;
    - advisor-u7-head.md:69-73 is ADV-U7S-6's guard, settled above.
  - **msg-2026-11-28.md:45-46** tells him the two Lows "are on PR #40's closing list". They are not (STOPB-2).
  - **NC-SB-16 is pinned by a test on a burst profile:**
    - server/src/probe_tests.rs:59-78 (`u6c_leg_logs_and_the_windows_count`) asserts
      `window_total(1_000_000, 100_000, Some((50 * SEC, SEC)))` = 1,010,000, "B1: the burst's 10,000 on top".
    - server/src/probe.rs:82-92's `window_total` is L1's loop (lab/src/probe.rs:124-131) word for word in its logic.
    - So no B1 leg is owed.
  - redact-check.mjs:133-145, :239-248: the host-name reading flags a host name only when its labels hold four numeric
    labels in a row (an address); the npm tarball path is excused. A URL's file name with a date in it parses as a host
    name (for example `name-v1.5-Month-24-2025.pdf`), so the real URL must be scanned before Y0, not first at X.
: STOPB-1 (High), the expected dist list must be in Y, not X (DONE)
- **What fails:**
  - RS clones Y, never X: rs.yaml:121, :143-148 fetch COMMIT = /etc/npt/commit = Y into /opt/npt/clone.
  - r1.sh runs from that clone and reads `EXPECTED_DIST=/opt/npt/clone/spikes/netcode-proof/ops/runs/r1-dist.sha256`
    (r1.sh:30-33).
  - A list filled only at X (the plan's D2, following R1.md:313) is therefore absent on RS. Bring-up writes
    "(the expected list is empty: Stop B fills it)" into DIST-DIFFERS (r1.sh:150), `host_active` refuses every page
    (:130-131), and `dist-accept` refuses an empty list (:878).
  - On R1's day no browser leg could run: his 80-minute sitting is lost, or a new commit pair and new VMs are needed.
  - Every bar in the plan would pass first, except two: r1sh.test.mjs:449 and browser.test.mjs:207-208 (STOPB-4).
- **Whose defect:** not the plan's. It is merged U8, in R1.md:313, r1.sh:136's comment and r1-dist.sha256:3. It comes
  from SEC-SB-5's fix text (u8-stopb-security.md:105, "commit the expected list at X ... root-owned in the clone") and
  from my own term at advisor-u8-stopb.md:192 ("The expected list is committed at X"). **Ignore my "committed at X".**
  The list must be in the commit the root clone holds: Y.
- **The fix form, ruled:**
  1. **Y0** = B2's commit: the two notes plus the two test stand-ins (STOPB-4), all under the gate and the counted
     run. Pushed, with CI green.
  2. **Build 1** of U7's dist at Y0, in worktree A with CARGO_HOME A.
  3. **Y** = Y0 plus ops/runs/r1-dist.sha256's seven lines from build 1, and nothing else. Bar:
     `git diff --name-only Y0 Y -- spikes/netcode-proof` prints exactly spikes/netcode-proof/ops/runs/r1-dist.sha256.
     web/build.mjs reads no ops/ path and no git state (Grep), so Y's dist inputs are Y0's.
  4. **Build 2** of U7's dist at Y itself, in worktree B with CARGO_HOME B. **Bar:** build 2's list equals the
     non-comment lines of the list committed at Y, byte for byte. The committed file keeps its 4-line comment. That proves both the paths' independence and that Y's own dist is the expected one.
     A difference comes to me before X, with both lists.
  5. **X** = Y plus the two cloud-init `/etc/npt/commit` lines (Y) and R1.md step 7's URL. Bar:
     `git diff --name-only Y X -- spikes/netcode-proof` prints exactly ops/cloud-init/rs.yaml, ops/cloud-init/lg.yaml
     and ops/runs/R1.md.
- **The words "committed at X"** (R1.md:313, r1.sh:136, r1-dist.sha256:3, and the pin at r1plan.test.mjs:150) stay as
  they are, because X descends from Y and so holds the list too. Stop B's message says in one line that the list sits
  at Y, which X carries. Fewer U8 lines are touched that way (item 7 of the count below).

## 1. The owed items before Y (A1-A4): A1, A2 and A4 APPROVE; A3 REVISE to two reads; four items added (DONE)
- **A1, the static link check: APPROVE.** It is my ruling's form (advisor-u8-f1-head.md:65-77). The run is the main
  session's, with no agent, before Y. A mismatch still comes to me with the lines; any config.mjs fix is a U8 line at
  his stop.
- **A2, Stop B's host read: APPROVE, with its brief named** (Q1):
  - One security-auditor seat (opus, xhigh), now.
  - Its reading list:
    - U6c's merged page host (crates/host serve.rs and its config), npt-host.service, ops/systemd/host.json and
      npt-host.path;
    - PR #40's page-host item (no handshake or read deadline, no connection cap, no LimitNOFILE);
    - SEC-SB-9's rest (u8-stopb-security.md:137-139, against npt-host.service:6-8's list);
    - B's two notes, read against SEC-U8F1-3 and -4 (Q4).
  - Its answer for each item: fixed where, which file's owner and which stop, or carried with his word.
  - A fix in npt-host.service (LimitNOFILE, sandbox lines) adds lines to a U8 file past 6,284. A fix in crates/host is
    a change to merged U6c code, so a new piece with its own plan. Either goes to him as a decision with the seat's
    rating, before X.
  - A finding cannot move Y unless it changes a file the servers build. If it does, Y comes after the fix.
- **A3, the probe block's hours: REVISE. No run; two reads** (Q2):
  - The container reading exists (STOPB-3):
    - u6c-r2-netcode.md:90-93 and :162: "measured legs ≈ 112 s ... six legs ≈ 675-680 s against U8's 732 s and LG's
      792 s. Nothing moves 122 s or 0.20 h". From the main session's probe_reading.py (F-READINGS (1)).
  - Read 1 names its limits:
    - P0 and P1 only. The bound holds for any profile with one-way delay plus half the jitter under about 9 s, so read
      P2-P4's and B1's delays from their profile file and confirm each is under it.
    - It ran before H1 moved the start-edge read, which is "normally under a millisecond" (u6c-report.md:34-37).
    - The probe and load code has not changed since U6c's merge, except U7's MAX_LEAD constant for the bots
      (`git diff 17964a3d 69682158`).
  - Read 2, NC-SB-16: the pinning test is server/src/probe_tests.rs:59-78. It asserts
    `window_total(1_000_000, 100_000, Some((50 * SEC, SEC)))` = 1,010,000 ("B1: the burst's 10,000 on top").
    probe.rs:82-92 is L1's loop (lab/src/probe.rs:124-131). Quote them both. No B1 leg is owed.
  - If either read moves the row, Q6 applies.
- **A4, main's tip at Y: APPROVE**, with the re-fill triggers made exact (Q5).
- **Owed and missing from A (STOPB-5):**
  - (i) **ADV-U8H-3**, the delay clock's vCPU:
    - DONE by the coordinator's read (adv-u8h-3-read.md): the clock thread never runs on the tick vCPU.
    - Ruled: the skip stands for R1. ds-selftest stays in TIMING_BLOCKS, the conservative side, as advisor-u8-head.md
      ruled ("until then the skip stands").
    - Two reasons:
      - Whether its judged numbers lean on the tick thread's send stamps is a netcode reading not made.
      - Moving it is a U8 line at the stop.
    - The read cannot move Y. It is carried after R1 as a netcode follow-up, and named in Stop B's message.
  - (ii) **TE-5 (T5i-1):** "one WebSocket leg without ignoreHTTPSErrors before R1 ... Stop B's fill, by the main
    session" (u7-report.md:190-191).
    - It is a record; no mark reads that series, so it cannot move Y.
    - Run it before X (U7's e2e harness, the container's Chromium) and record its handshake_failed and accepted
      counts.
    - If it cannot run in the container, the report says why, and the first R1 WebSocket leg reads it.
  - (iii) **NC-3's Stop B read**, DROP never REJECT: SATISFIED by my read (Notes: npt-lib.sh:10, rs.yaml:45-46, no
    reject rule in ops/).
  - (iv) **ADV-U7S-6's guard:** SATISFIED at today's HEAD by my read. r1.sh's hunks since 9c2fcf4 touch neither
    load_settings nor load.rs. It is repeated at X: if `git diff --stat 9c2fcf4 X -- spikes/netcode-proof/ops/runs/r1.sh
    spikes/netcode-proof/crates/bots/src/load.rs` shows more than today's 17/6 on r1.sh, I read the pairing again.

## 2. The two notes (B1-B2): APPROVE the texts and lines; the bar and the closing list amended (DONE)
- **The texts and places: APPROVE.** They match u8-f1-build.md:378-381 word for word: SEC-U8F1-3's sentence as the
  push step's 7th line, and SEC-U8F1-4's clause as a new line after R1.md:88.
- **The bar, exact:**
  - `git diff --numstat` reads net +1 in PROMPT-R1.md and net +1 in R1.md, so U8 is at 6,284.
  - The only other U8 lines touched in Y0 are STOPB-4's two test stand-ins, each rewritten in place (`1 1`).
  - A re-wrap may change a line in place only if a pin then fails on the re-wrap alone, as at u8-f1-build.md:383-385.
    That pin then takes its `\s+` form in place, the pin's own plant is re-proved by name, and the line is counted in
    the report.
  - Anything that adds a net line stops before the edit and goes to him (his stop).
  - The ops tests, wrappers ("315 ok, 0 failed") and redact (0 findings) pass in the counted run.
- **The closing list (STOPB-2):** PR #40's list holds neither SEC-U8F1-3 nor -4, though my C2
  (advisor-u8-f1-merge.md:59-63) put them there "now". u8-report.md:131 also sends SEC-SB-9's rest there.
  - Before B2: add all three to the PR body's list, each with its source.
  - At B2: SEC-U8F1-3 and -4 come off it, "closed by Y0 <sha>".
  - SEC-SB-9's rest stays until A2 settles it.
  - The report says the list missed them from U8-F1's merge until now.
  - The day message's item 27 (msg-2026-11-28.md:45-46: "They are on PR #40's closing list") is made true by the edit
    before the message goes, or reworded.
- **Its name:** B2's commit is Y0, not Y (STOPB-1). Y is Y0 plus the list.

## 3. Y, the tools and the dist hashes (C1-C3): REVISE to STOPB-1's order; the recipe and its environment set (DONE)
- **C1:** Y0, then Y, each pushed with CI green, in the order of STOPB-1. Build 1 runs at Y0, build 2 at Y.
- **C2, the tools: APPROVE.**
  - Printed beside rs.yaml's pins:
    - rustup (rs.yaml fetches the 1.29.3 archive; the container's own rustup prints its version);
    - the toolchain `rustup toolchain install` resolves from the spike's toolchain file;
    - `wasm-bindgen --version`, which must read 0.2.130 (build.mjs refuses otherwise);
    - `node --version` v22.23.4;
    - npm's version.
  - A mismatch comes to me before any build, as the plan says.
- **C3, the recipe: only the dist's own lines of rs.yaml, word for word.**
  - That is :193 (`rustup toolchain install`), :194 (`cargo install wasm-bindgen-cli --version 0.2.130 --locked`, with
    its CARGO_TARGET_DIR prefix and `--root` inside that worktree's scratch home), :195 (`cargo fetch --locked`) and
    :198-200 (in web/, `npm ci --ignore-scripts` and `CARGO_NET_OFFLINE=true node build.mjs`, both with
    npm_config_ignore_scripts=true).
  - Not :196-197, the workspace `--bins` and bites builds: they make RS's binaries, not web/dist. build.mjs builds
    npt-client-wasm itself in its own target (build.mjs:5-6, :152-158). That saves two release workspace builds per
    tree.
- **Where they run:** not through main-run.sh, whose :17-18 export `CARGO_NET_OFFLINE=true` and CARGO_TARGET_DIR. That
  would refuse :195's fetch and move :194's target.
  - Each build runs as its own background command, under the lane lock taken as main-run.sh takes it
    (.gate/shared-target/.lane.lock), with nothing else building.
  - CARGO_HOME is set per build, to two different paths.
  - Before each build it prints the NAMES (never the values) of every set variable among RUSTFLAGS,
    CARGO_ENCODED_RUSTFLAGS, RUSTC_WRAPPER, CARGO_BUILD_*, CARGO_PROFILE_*, CARGO_TARGET_DIR, CARGO_NET_OFFLINE and
    RUSTUP_TOOLCHAIN. All must be unset except rs.yaml's own per-command prefixes. Every written path is literal
    (FULL-PATHS).
- **The two worktrees:** A at Y0 and B at Y, each made by `git worktree add` under .gate/ with a literal path, removed
  after X (CHECKPOINTS §10). Free disk is printed before build 1 and after it.
- **The bars:**
  - Build 1: seven files, listed in rs.yaml:221's form (`find . -type f -print0 | LC_ALL=C sort -z | xargs -0
    sha256sum` in web/dist), so each line reads `<sha256>  ./<name>`. That list is r1-dist.sha256's body, under its
    4-line comment (r1plan.test.mjs:149 pins line 1).
  - Build 2's list equals Y's committed list byte for byte (STOPB-1, step 4).
  - A supporting line for the report, not a bar: compare with U7's seven (u7-report.md:134-141, the measured
    hooks.js). Only crates/judge changed in the build inputs since U7's merge, but U7's list was its snapshot tree's,
    never compared with the merge.
- **Q3:** `cargo fetch --locked` into the second CARGO_HOME through the proxy.
  - It is rs.yaml's own command, and it shows the lock fetches clean, as RS will on the day.
  - A copy of the first registry only if the proxy refuses, said so in the log.

## 4. The fill X (D1-D4): REVISE to STOPB-1's split; two pins named (DONE)
- **D1, the pins: APPROVE, and two are named now (STOPB-4).** Each breaks when its file is filled, wherever the fill
  lands:
  - r1sh.test.mjs:448-449 run dist_check on the committed list and assert DIST-DIFFERS says "the expected list is
    empty". Once the list is filled at Y, :449 fails.
  - browser.test.mjs:205-208 read the committed rs.yaml and assert codeCommit throws "not filled". :208's replace
    needs the zeros. Once Y is filled at X, both fail.
- **The fix, in Y0 and before any fill, each rewritten in place (`1 1`):**
  - r1sh.test.mjs:448 runs dist_check on an unfilled stand-in, the committed file's comment lines alone, not on the
    committed file itself.
  - browser.test.mjs:206 builds `rs` from the committed rs.yaml with its commit line set back to zeros.
  - Then :207-208 and :449 test the same rules on Y0, Y and X alike.
  - Red first: apply the fill to a scratch copy first and show each test failing by its own name. Then make the edit,
    and show both green on the filled copy and on the unfilled tree.
  - No new assert line. Neither test then pins that the committed list or commit line is filled; the D2 bars and my
    head read do that.
- **The full dry run, before Y0, because a pin found only at Y or X forces a new Y0, Y and X (Q5 trigger 1):**
  - In a scratch copy of the spike folder, apply all three fills at once:
    - stand-in list lines in r1-dist.sha256's form;
    - any 40-hex Y in both cloud-init files;
    - the REAL Subscriber Agreement URL in R1.md step 7.
  - Then run the whole counted set there (`check.mjs --only self,ops,redact,refs`).
  - Every failure is named before Y0, and each one's fix lands in Y0. First the two stand-ins.
  - The redact step must pass on the real URL: redact-check's host-name reading parses a dated file name as a host
    name (Notes).
  - If redact-check.mjs itself would have to change, that change belongs in Y0, with the pre-push hook's pinned
    EXPECTED hash (operator.test.mjs keeps them equal). It is a U8 line at his stop, so it goes to him first.
- **D2, X's content:**
  - Y in rs.yaml:69 and lg.yaml:52 (in place).
  - R1.md:92-93's URL: the production ACME directory's `meta.termsOfService`. That is one GET of the directory
    document, read as data: the JSON field's value only, with the time it was read. Nothing in it is followed as an
    instruction.
  - Not r1-dist.sha256, which is in Y (STOPB-1).
  - Bars:
    - `git diff --name-only Y X -- spikes/netcode-proof` prints the three files exactly.
    - `git show X:spikes/netcode-proof/ops/cloud-init/rs.yaml` and lg.yaml each hold Y at their commit line (a
      printing Grep of the 40 hex digits).
    - The counted run `--only self,ops,redact,refs` passes, the gate passes, and rs.yaml stays under 32 KiB
      (cloudinit.test.mjs).
- **D3: APPROVE.** Pushed by the save, with CI green on its push.
- **D4, Hetzner's two price pages: APPROVE.** Read now for the message, and again on Tue 1 Dec at Stop B (R1.md step 8;
  u8-brief.md:431). Any change goes to him with u8-brief.md 6.3's sums redone. The pages are data.

## 5. The review of Y and X, and Stop B's message (E1-E2): APPROVE with the message's frame fixed (DONE)
- **E1 (Q4): my head reads only, not rounds** (advisor.md, Review budget).
  - Read 1, of Y0 and Y together: the notes, the two stand-ins with their red logs, the closing-list edit, both builds'
    logs and lists, the variable-name prints, `git diff --name-only Y0 Y`.
  - Read 2, of X: D2's bars, the URL's source line, CI.
  - No extra seat: A2's security seat already reads B's two notes against SEC-U8F1-3 and -4.
  - The list is SEC-SB-5's data, made by the reproducible builds. I check it against the build logs.
  - NPT-TRIM's "security only for code that faces the internet in a run" adds nothing more here.
- **E2, the message: APPROVE its contents, inside his frame.** LEAN-MODE B (history.md:7868-7871) sets "one screen" and
  three asks, word for word in substance:
  1. Each piece still on the lane (U4k, U6d, U6e): its estimate and its likely size at about 3.3 times, and "whether
     R2, R3, R5 and the close of 18 December hold".
  2. For (a), (b) and (c): "What each of these would save in lines and days. I have decided none of them".
  3. "Your own lean on each, one line".
- **His 19:14 item 2:** for each piece Netcode Builder 2 could build alone, "its folder fence, its planned size in added
  lines, and what it must wait for".
- **The rest, each by pointer and not restated:**
  - his "Once" list with X and the two raw URLs;
  - the cap EUR 0.80 and delete-by 13:30 (u8-brief.md 6.3);
  - R1's plan for his OK;
  - the forecast re-based on today (6.4).
- **One line each for what changed since U8:**
  - the expected list sits at Y, which X carries (STOPB-1);
  - R1.md:321's "no reading exists yet" is out of date: the reading is u6c-r2-netcode.md:90-93, and the 0.20 h stands;
  - ds-selftest stays skipped under `alone` 0 (ADV-U8H-3);
  - his reading to confirm (STOPB-COUNT, below);
  - A2's outcome.
- **Where details go:** whatever does not fit one screen goes to a pointer.

## 6. Q1-Q6, one line each (DONE)
- **Q1:** Yes. One security-auditor seat (opus, xhigh) now, with section 1's reading list, beside A1 and A3. Both of
  those are the main session's own reads and runs, with no agent, so the session stays well under three agents.
- **Q2:** No new run is owed. The container reading exists (u6c-r2-netcode.md:90-93, :162: "Nothing moves 122 s or
  0.20 h"). A3 is two reads with their limits named: P2-P4's and B1's delays against the 9 s bound, and NC-SB-16's
  pinning test on a burst profile (server/src/probe_tests.rs:59-78, beside lab/src/probe.rs:124-131). Bar: each read
  quoted with file:line.
- **Q3:** A fresh `cargo fetch --locked` into the second CARGO_HOME through the proxy, which is rs.yaml's own command.
  Copy the first registry only if the proxy refuses, and say so.
- **Q4:** My head reads of Y0 with Y, then of X, only (not rounds). A2's security seat also reads B's two notes against
  SEC-U8F1-3 and -4. No infra seat.
- **Q5:** No reason to wait: fill now, so a defect has days to be fixed, not hours. Two exact re-fill triggers until R1
  starts:
  - (1) Any commit that changes a path under spikes/netcode-proof means a new Y0, Y and X. The dist is rebuilt only if
    its inputs moved (crates/, web/, Cargo.*, the toolchain file); otherwise the list carries over and build 2 is redone
    at the new Y.
  - (2) A move of main's CLAUDE.md or .claude, taken in, means a new X on the branch's tip, still naming Y, with D2's
    bar against Y. PROMPT-R1.md:22 stops the run if `<run>` at X differs from origin/main there.
  - A new X made after his Stop B OK goes to him before R1, with its two new raw URLs (R1.md:17-19), since he pastes
    them at 09:15.
  - On Stop B's day, print `git diff --name-only X origin/claude/netcode-proof-test-plan-hcky5g -- spikes/netcode-proof`
    (notes only, or empty) and `git diff --name-only X origin/main -- CLAUDE.md .claude` (empty). Re-read the two
    price pages.
- **Q6:** This does not arise: the reading does not move the row. If a read did move it, changing R1.md's hours and
  slot row would be a fill: U8's own brief names "R1.md's hours and slot table filled after U6c's merge"
  (u8-brief.md:5-6), and his U8-FILLS (a) puts Stop B's fills outside the stop. It would be named in his message, his
  to reverse. A change to r1.sh's timing values would be a U8 line at his stop, and would go to him.

## 7. The plan review checklist (advisor.md): what the plan leaves out (DONE)
- **Risk tier:** not named. It is 1: spikes/ code and tests, notes, a PR body edit. No kit, CI, gate, secret or money
  (reading the price pages and the ACME directory costs nothing).
- **Risks:** not named. They are:
  - STOPB-1's commit pair;
  - a move on main before R1 (Q5);
  - disk for two dist builds (printed);
  - the proxy for fetch and npm;
  - unequal builds (to me before X);
  - A2 finding a fix past his stop (a decision for him);
  - TE-5 not runnable in the container.
- **A2's agent:** opus, xhigh (R1.md:351 sets security at xhigh for Stop B's reads). It saves each finished block at
  once; at most this one agent.
- **Checkpoint:** the card and stopb-plan.md, each step's SHA added there as it lands.
- **Human check:** nothing a player sees in this pass. His Stop B steps are his.

## Verdict
**REVISE**, for one High (STOPB-1). It covers B2 through D only; the rest may start now.
- **Start now, as approved here:** A1; A2's seat with section 1's brief; A3 as two reads; A4; B1's dry run with
  section 2's bar; C2's tool prints; D4's first read; E2's drafting. Also the closing-list edit (STOPB-2) and TE-5
  (STOPB-5).
- **Waits for the revised plan:** B2 onward, that is Y0, the builds, Y and X. The plan gains conditions C1-C7 below.
- **The re-check:** I re-check only the changed lines. NPT-TRIM allows a re-check when a check "finds a Critical or
  High". It is a plan check, not a round.

### Findings, ranked (CLAUDE.md's scale)
- **STOPB-1 (High):** the expected dist list filled at X never reaches RS, which clones Y (r1.sh:30-33, rs.yaml:121-148).
  R1's browser legs would be refused on the day. The source is merged U8 and my own term (advisor-u8-stopb.md:192),
  not this plan.
- **STOPB-2 (Medium):** PR #40's closing list lacks SEC-U8F1-3, SEC-U8F1-4 (my C2, advisor-u8-f1-merge.md:59-63) and
  SEC-SB-9's rest (u8-report.md:131). B2 cannot take off the list what is not on it.
- **STOPB-3 (Medium):** A3's premise is wrong. The container probe reading exists (u6c-r2-netcode.md:90-93, :162), so
  the planned loopback run would repeat it, and R1.md:321's "no reading exists yet" would reach him as fact.
- **STOPB-4 (Medium):** two pins break when their files are filled, and the plan does not name them:
  r1sh.test.mjs:448-449 for the list and browser.test.mjs:205-208 for rs.yaml's commit line. D2's "ops tests pass" bar
  cannot hold as written.
- **STOPB-5 (Medium):** owed items missing from A.
  - ADV-U8H-3: now read by the coordinator; the skip stands.
  - TE-5 (T5i-1): owed at Stop B's fill.
  - NC-3's DROP read and ADV-U7S-6's guard: both satisfied by my reads here; ADV-U7S-6's is repeated at X.
- **STOPB-6 (Low):** the plan names no risk tier, no risks and no model or effort for A2's seat. It also misses that
  main-run.sh's CARGO_NET_OFFLINE and CARGO_TARGET_DIR exports would refuse rs.yaml's fetch.
- **STOPB-7 (Low):** E2 does not carry his "one screen" or his three asks word for word in substance (LEAN-MODE B,
  history.md:7868-7871).

### Conditions for the revised plan (each one line in stopb-plan.md)
- **C1 (STOPB-1):** the order Y0 → build 1 at Y0 → Y = Y0 + r1-dist.sha256 only → build 2 at Y, equal to Y's list → X
  = Y + the two commit lines + the URL. Bars: `git diff --name-only Y0 Y -- spikes/netcode-proof` is the list's file
  alone; `git diff --name-only Y X -- spikes/netcode-proof` is exactly the three files.
- **C2 (STOPB-4):** before Y0, a full dry run applies all three fills to a scratch copy (stand-in list lines, any 40-hex
  Y, the real URL) and runs `--only self,ops,redact,refs` there. Every failure it names is fixed in Y0. Among them, the
  two test stand-ins are rewritten in place (`1 1`), each red first on the filled copy and green on filled and
  unfilled trees, with no new assert line.
- **C3 (STOPB-2):** before B2, the three findings go onto PR #40's list. At B2, -3 and -4 come off, naming Y0. The
  report says the list missed them since U8-F1's merge, and the day message's item 27 is made true.
- **C4 (STOPB-3):** A3 is two reads (section 1), with no run: the reading's limits, and NC-SB-16's pinning test
  (probe_tests.rs:59-78). R1.md:321's stale words are named in his message, not edited.
- **C5 (STOPB-5):** A gains ADV-U8H-3 (done; the skip stands; carried after R1), TE-5 (run before X, or the reason it
  cannot run), NC-3 (satisfied) and ADV-U7S-6 (satisfied; repeated at X).
- **C6 (STOPB-6):** the builds run outside main-run.sh under the lane lock, with the build-affecting variable names
  printed and unset; only the dist's own rs.yaml lines run (section 3). The plan names the risk tier, the risks and
  A2's seat (opus, xhigh).
- **C7 (STOPB-7):** E2 is framed by LEAN-MODE B's one screen and its three asks, and by his 19:14 item 2's three
  fields. The rest goes by pointer.

## Decisions needed
- None from this check.
- Two may come later, each self-contained when it does:
  - A2's seat, if a fix needs lines in npt-host.service past his 6,284 stop (LimitNOFILE, the sandbox's rest).
  - A1, if config.mjs needs a fix.

## For Mustafa
1. Nothing to act on now.
2. For the day message, one line with the count, for his reading. In this pass, U8 grows by +2 net (the two notes:
   6,284, his stop), plus the fills (seven list lines; Y, X and the URL in place). Two more U8 test lines are rewritten
   in place at 0 net (the stand-ins C2 needs), after UM-4's three. "Net" is how his stop has been counted since U8-F1's
   brief (u8-f1-brief.md:13-15). If he counts a line rewritten in place, the stand-ins need +2 from him, since the fill
   cannot pass its own tests without them.

Status: COMPLETE 2026-11-27 22:00 UTC

## Re-check of rev 0.2
Plan: read stopb-plan.md rev 0.2 whole (132 lines). Check only the lines tagged [C1]-[C7] and [T], plus the coordinator's
items 1-5 against their evidence. Rule the rs.yaml:199 question. Same rules as above: read only; Bash for single git
commands only.

### Notes (one line per check)
- rev 0.2 (Read whole): the tags sit on :13-15 [C6], :26-29 [C6], :30-34 [C4], :35-39 [Q5], :40-45 [C5][T],
  :46-49 [T], :52-57 [C2][T], :62-64 [C2], :71-73 [C3], :76-81 [C1][T], :85-93 [C6], :94-96 [T], :101-102 [C2],
  :115-121 [C7].
- A1's logs (Grep):
  - a1-links.log:134 "links: 133, mismatches: 0";
  - a1-plant.log:2-8 four "MISMATCH smoke <browser>-ws" lines and :134 "mismatches: 4";
  - a1-web.log:7 the web step at exit 0, 22:06:32 UTC.
  - The claims hold. The plant's exit code is the main session's word; the log prints the count only.
- A6 (spot-reads):
  - PROMPT-R1.md:57-60: user-data compared with `<run>`'s rs.yaml (X), then `npt-userdata head` shows Y.
  - rs.yaml:148 and :191 compare each clone's HEAD with COMMIT (read earlier).
  - PROMPT-R1.md:22 is scoped to CLAUDE.md and .claude.
  - No check compares X with Y, so notes-only save commits between them break nothing. The claim holds.
- The URL's source chain:
  - issue.rs:162-164: "production" maps to `LetsEncrypt::Production.url()`;
  - Cargo.lock:688-690: instant-acme 0.8.5 from crates.io.
  - The URL itself I did not fetch (no web in this seat). B1's X shape scans it with redact-check.
- redact-check on the URL (reasoned): `LE-SA-v1.8-July-06-2026.pdf` parses as a host name. Its numeric labels (8, 06,
  2026) never make four in a row, so I expect no finding. B1's X shape is the proof.
- **R1.md:90-93 (the URL's place):** the placeholder spans :92-93. :92 runs to about 107 columns before it, so the
  63-character URL cannot join :92 under the 120-column wrap. The fill changes both lines: numstat `2 2`, net 0, not
  `1 1`. Rev 0.2's C1 bar (:80, "`1 1` each") would fail on R1.md.
- **B1's worktree:** a fresh `git worktree add` has no ops/browser/node_modules (gitignored). opsStep then fails at
  once with OPS_MISSING (check.mjs:2694, UM-4 (d)), so the dry run's ops step cannot show 104 of 104. Rev 0.2 :52-57
  names no install or link for it. U8's merge run used a link to the cleared install (u8-report.md:60).
- The npm deviation line (:94) names "rs.yaml:198"; the npm line is :199 (Grep: rs.yaml:199 is the only npm line in
  cloud-init/).
- **The rs.yaml:199 question:**
  - cloudinit.test.mjs:142 pins `/npm_config_ignore_scripts=true npm ci --ignore-scripts/`, and browser.test.mjs:128
    pins `/npm ci --ignore-scripts/`. Both would still match with `--no-audit --no-fund` appended, so option 1 needs no
    test line.
  - SEC-U7D-3 (security-u7-deps.md:223-226; item (3), :139-148) is about "C10's two commands" in this container: "the
    refused `npm audit` by another door", a session permission routed around.
  - Item (4) (:150-153) read rs.yaml's line for ignore-scripts only.
  - On RS no session refusal exists to route around. The audit call would send web/'s lock (TypeScript 6.0.3 alone) to
    the public registry, it changes no installed byte, and an audit failure does not fail `npm ci`.
- The other tagged lines match my conditions word for word in substance:
  - C1's order and bars (`7 0` for the list; build 2 against the non-comment lines);
  - C2's full dry run in three shapes, with red first;
  - C3's closing-list edit before B2;
  - C4's two reads;
  - C5's four items, with TE-5 in a disposable worktree, its tracked file never edited, time-boxed to 45 minutes;
  - C6's recipe, environment and risks;
  - C7's frame;
  - the Q5 triggers with "a new X after his OK goes to him".

### Rulings
- **The rs.yaml:199 question: option 2.** Keep rs.yaml as merged, and record the deviation as rev 0.2 :94-96 words it.
  Three reasons:
  - The audit call on RS is not SEC-U7D-3's case: no refusal is routed around there, and it sends one public package's
    name.
  - The flags change no installed byte, so the dist's equality bar does not lean on them.
  - Option 1 would touch a third U8 line in this pass, on an in-place reading he has not yet confirmed, for no change
    in evidence.
  - One exception: if A2's security seat rates RS's audit call as a finding, it rides Y0 in option 1's form (`1 1`, net
    0, before Y0, as a file the servers build from).
  - Neither option is his call. Option 2 changes nothing. Option 1 is a technical choice at net 0, by the count he
    has been told (the day message's line), and would move no stop, money or run day.
- **A1: DONE**, as claimed. **A6: holds**: nothing on R1's day compares X with Y.
- **The URL:** its source chain holds. It is filled only at X, after B1's X shape has scanned it.

### Verdict on rev 0.2
**APPROVE WITH CONDITIONS.** STOPB-1 (High) is resolved by C1's order, and no Critical or High is open. B2 onward may
start once these three lines are in rev 0.2:
- **R1 (Medium):** B1's worktree gets ops/browser/node_modules before its first run. Either a link to the main
  folder's cleared install, as U8's merge run did (u8-report.md:60), or the cleared form (`npm ci --ignore-scripts
  --no-audit --no-fund --prefix <the worktree's absolute ops/browser>`, with npm_config_ignore_scripts=true). The log
  says which. Without it the ops step fails on OPS_MISSING in every shape. The worktree is removed after B1.
- **R2 (Low):** C1's X bar for R1.md reads "net 0, its hunk inside :92-93" (`2 2` expected after the re-wrap), not
  `1 1`. rs.yaml and lg.yaml stay `1 1`.
- **R3 (Low):** :94's "rs.yaml:198" becomes ":199", and the line adds "kept as merged (B6's re-check, option 2)".

### Decisions needed
- None.

### For Mustafa
- Nothing to act on.
- The day message's count line stands as this file's first "For Mustafa" and the plan's Size give it: two notes, +2 net (6,284);
  two test lines rewritten in place at 0 net; the fills. Option 2 adds no third in-place line.

Status: COMPLETE 2026-11-27 22:12 UTC

## Ruling on the dist difference
Plan: read stopb-dist-diff.md whole. Read wasm-bindgen-shared 0.2.130's build.rs and lib.rs in the registry source
myself. Check that the two hashes in `producers` are Y0's and Y's SHA prefixes. Then answer Q1-Q4. Read only; Bash for
single git commands.

### Notes (one line per check)
- stopb-dist-diff.md (Read whole, 63 lines):
  - five builds;
  - the deciding build at Y0 on a third path (.gate/stopb-c) gave 2a97394b, the same as build 1;
  - Y gave a94698198f73 on two paths, and again after a clean;
  - the 9-byte difference is all in `producers`;
  - the earlier path theory is withdrawn in the file's own words ("superseded").
- /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-shared-0.2.130/build.rs:10-21 (Read):
  `Command::new("git").arg("rev-parse").arg("HEAD")`, with no directory set, so it runs in the build script's own
  folder, which cargo sets to the package's source folder. If 9 or more characters come back it prints
  `cargo:rustc-env=WBG_VERSION=<first 9>`.
- src/lib.rs:405-411 (Grep): `version()` = CARGO_PKG_VERSION, plus " (" + WBG_VERSION + ")" when that is set.
- The two hashes are our commits:
  - `git log -1 87cb8677` → 87cb8677d51535ea…, "Stop B Y0: the two security notes ...";
  - `git log -1 652e9513` → 652e9513ce2c1369…, "Stop B Y: R1's expected dist list, from build 1 at Y0".
  - The `producers` strings "(87cb8677d)" and "(652e9513c)" are exactly their first 9 characters.
  - `git diff --numstat 87cb8677 652e9513 -- spikes/netcode-proof` → `7 0` on r1-dist.sha256 alone.
  - So nothing but the commit's name separates the two builds.
- On RS: rs.yaml:175-176 runs the build as npt-build under `env -i HOME=/home/npt-build PATH=… COMMIT REPO`.
  - rustup-init's default CARGO_HOME there is /home/npt-build/.cargo, so cargo unpacks the registry source under it.
    No git repository encloses that folder; the clone is the sibling /home/npt-build/clone (rs.yaml:188-192).
  - `env -i` passes no GIT_* variable.
  - So WBG_VERSION stays unset on RS, and `producers` reads plain "wasm-bindgen 0.2.130". This is reasoned from the
    code and rs.yaml, not run on RS.
  - Every list made so far carried a hash: build 1, build 2, and U7's run 4. None of them would equal RS's dist.
    - **Correction (2026-11-28 03:0x UTC, at the Y' head read):** U7's run 4 carried no hash. Its Wasm is 901a6492…
      (u7-report.md:141), the plain value build 1′ and build 2′ produce. I repeated stopb-dist-diff.md:32's "differs
      the same way" without checking it. Only build 1 and build 2, plus build 2's re-run and the third build at Y0,
      carried our commit's name.

### Findings
- **STOPB-8 (High; caught before X by C1's bar, and resolved by the method below):**
  - The container builds put CARGO_HOME inside the worktree, so wasm-bindgen's build script wrote our commit's SHA
    prefix into the Wasm's `producers`.
  - Y's list (652e9513) would have refused every page on R1's day (DIST-DIFFERS; r1.sh:130-131), the same harm as
    STOPB-1.
  - It is no code defect, and the earlier builds were not wrong in themselves. It is an environment difference from RS
    that C3 did not name, and I did not name it either: my C6 listed cargo's variables but not where CARGO_HOME sits.
  - **SEC-SB-5's two-path build did its job:** this was its first real result (u8-report.md:76), and its bar caught it.

### Answers
- **Q1: the root cause is accepted, and the fix sits inside C3.**
  - The proof:
    - the commit-for-path swap (Y0 gives 2a97394b on paths A and C; Y gives a94698198f73 on paths B and A);
    - the 9 bytes that differ, all in `producers`;
    - build.rs read line by line;
    - the exact SHA prefixes.
  - C3 asked for rs.yaml's own dist lines. RS runs them with CARGO_HOME outside any repository, so putting CARGO_HOME,
    the wasm-bindgen `--root` and its CARGO_TARGET_DIR outside every repository reproduces RS's condition. It is not a
    third deviation.
  - The npm flags remain the one recorded deviation (option 2).
- **Q2: the bars, with three additions. Yes, one line in his message.** Each of the two builds:
  1. Runs with every `GIT_*` variable left out of its environment, as RS's `env -i` does. The names are printed, never
     the values, beside C6's cargo names. Without it, an inherited GIT_DIR would hand build.rs a repository whatever
     the folder.
  2. Prints `git rev-parse HEAD` failing ("not a git repository") from inside that CARGO_HOME's unpacked
     `registry/src/<index>/wasm-bindgen-shared-0.2.130` folder, which is build.rs's own folder, not just the
     CARGO_HOME root. Run it after the install and the fetch, when the folder exists.
  3. Prints its Wasm's `producers` entry: "wasm-bindgen 0.2.130" with no parenthesised suffix. A printing read of the
     custom section; a silent grep does not count (EXIT-SILENT).
  4. Equality, as before: two builds, two worktree paths (under .gate/, per the worktree rule) and two CARGO_HOMEs.
     Their lists are equal byte for byte.
  5. The new list's six other lines equal Y's six. Only the Wasm line moves, which shows the leak touched nothing else.
  - **His message:** one line. "The expected list's Wasm carries wasm-bindgen's plain version, as RS will build it. A
    first list carried our own commit's name through wasm-bindgen's build script and was voided before the fill. The
    day's dist compare stays the wall." It goes with the void Y's SHA in the report, not on his screen.
- **Q3: yes, a follow-up after R1, not now.**
  - web/build.mjs (U7's file) refuses a Wasm whose `producers` names wasm-bindgen with any parenthesised suffix,
    failing closed with its reason, beside its home-path scan (build.mjs:164). It does not strip the suffix: refusing
    keeps the dist exactly wasm-bindgen's output and points the builder at the environment.
  - It comes with a test and a plant.
  - Taker: the next change to web/build.mjs. Named in the report's follow-ups.
  - Now, it would be a code change to a build input, so a new Y and new builds, for a leak the method below already
    closes.
- **Q4: before the rebuild:**
  - (a) Redo **both** builds outside every repository. Build 1's earlier Wasm carried Y0's name, so it stands for
    nothing.
    - Build 1′ runs at Y0 or at Y (equal inputs, now that the commit no longer leaks). It gives the list's new Wasm
      line.
    - Y′ = Y plus that one line replaced: `1 1` against Y; against Y0 still the list's file alone, `7 0`.
    - Build 2′ runs at Y′ itself and must equal Y′'s non-comment lines.
  - (b) Y (652e9513) stays in history, recorded as void in the report and on the card. It is never named in a
    cloud-init file. X names Y′.
  - (c) The scratchpad folders (npt-dist-a, npt-dist-b) are outside the repository, so FULL-PATHS holds by literal
    paths. If the scratchpad is cleared mid-build, redo that build; nothing in it is evidence once the logs are under
    .gate/stopb/.
  - (d) The lane lock and C6's prints stay, now with the `GIT_*` names added.
  - (e) X comes after MAIN-70 and after his answers on A2's host findings, as proposed.
    - If MAIN-70 changes any path under spikes/netcode-proof, Q5's trigger (1) applies (a new Y0, Y′ and X).
    - If it changes CLAUDE.md or .claude, trigger (2) applies (X takes them in, still naming Y′).
  - (f) My head read covers Y′ with both new builds' logs, the three prints per build, and the `1 1` diff against Y.

### Verdict
**APPROVE the fix, with Q2's three added bars and Q4 (a)-(f).**
- STOPB-8 is resolved by the method once both builds pass those bars.
- Nothing here is Mustafa's to decide: it is a build method inside C3, with no code, stop, money or run-day change.

### Decisions needed
- None.

### For Mustafa
- Nothing to act on.
- Q2's one line goes into Stop B's message.

Status: COMPLETE 2026-11-28 02:14 UTC

## Y' head read
Plan: read launch-yp-head.md's brief. Then, by SHA only, one git call each:
- Y′ 1ceecc32: its subject and parents;
- `git diff --numstat` against Y (652e9513) and Y0 (87cb8677) over spikes/netcode-proof, with the hunk;
- `git show 1ceecc32:…/r1-dist.sha256`;
- a printing `git grep` for 2a97394b beside 901a6492;
- the name list of 652e9513..1ceecc32, for MAIN-70's paths, checking no kit or CI path and nothing under spikes/ but the
  list.

Then, with Read and Grep: build1p.log and build2p.log (Q2's three prints, exit codes, the trees), build1p-dist.sha256
and build2p-dist.sha256 against Y′'s non-comment lines and against u7-report.md:134-141, yp-counted.log, and the
gate's log. Then the verdict, with CI's state as read, the line-623 correction (done above), Decisions needed, For
Mustafa and the Status line. Nothing run or built; no GitHub writes.

### Notes (one line per check)

