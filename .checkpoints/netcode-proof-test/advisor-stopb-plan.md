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
  - PR #40's body, re-read by `gh api repos/{owner}/{repo}/pulls/40 --jq .body` at 22:0x. Its "Closing list: security
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

## The main finding first: STOPB-1 (High), the expected dist list must be in Y, not X (DONE)
- **What fails:**
  - RS clones Y, never X: rs.yaml:121, :143-148 fetch COMMIT = /etc/npt/commit = Y into /opt/npt/clone.
  - r1.sh runs from that clone and reads `EXPECTED_DIST=/opt/npt/clone/spikes/netcode-proof/ops/runs/r1-dist.sha256`
    (r1.sh:30-33).
  - A list filled only at X (the plan's D2, following R1.md:313) is therefore absent on RS. Bring-up writes
    "(the expected list is empty: Stop B fills it)" into DIST-DIFFERS (r1.sh:150), `host_active` refuses every page
    (:130-131), and `dist-accept` refuses an empty list (:878).
  - On R1's day no browser leg could run: his 80-minute sitting is lost, or a new commit pair and new VMs are needed.
  - Every bar in the plan would pass first, except one: r1sh.test.mjs:449 (STOPB-4).
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
  4. **Build 2** of U7's dist at Y itself, in worktree B with CARGO_HOME B. **Bar:** build 2's list equals the list
     committed at Y, byte for byte. That proves both the paths' independence and that Y's own dist is the expected one.
     A difference comes to me before X, with both lists.
  5. **X** = Y plus the two cloud-init `/etc/npt/commit` lines (Y) and R1.md step 7's URL. Bar:
     `git diff --name-only Y X -- spikes/netcode-proof` prints exactly ops/cloud-init/rs.yaml, ops/cloud-init/lg.yaml
     and ops/runs/R1.md.
- **The words "committed at X"** (R1.md:313, r1.sh:136, r1-dist.sha256:3, and the pin at r1plan.test.mjs:150) stay as
  they are, because X descends from Y and so holds the list too. Stop B's message says in one line that the list sits
  at Y, which X carries. Fewer U8 lines are touched that way (item 7 of the count below).
