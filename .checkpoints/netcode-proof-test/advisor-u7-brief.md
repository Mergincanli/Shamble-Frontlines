# Advisor check: U7's build brief, rev 1 (2026-11-23)
Brief: u7-brief.md, 358 lines, sha256 c9e1fb33d078438bbe79a3cfd412defb3dbbfcb11b721d2583e80c00a13ebfab (confirmed).
Drafter: netcode-architect (notes netcode-u7-brief.md). Main's advisor.md read (git show origin/main:.claude/agents/advisor.md).

## Plan
1. Read the rules in force word for word: history.md:7561-7593 (NPT-TRIM), :7595-7607 (2026-11-22 answers),
   :2234-2282 (TEST-FIRST); the resume card's Rules, Running work and U7 line.
2. Read U7's rows: replan.md (:18, :45-46, :63-64, :77-79, :89-94, :101-102), replan-units.md §4 (:123-149) and §7,
   replan-marks.md (:31, :48, :49, :56, :75, :96); plan.md where cited (:485-506, :1123-1126, :1214-1223, :1255).
3. Read the sibling briefs' sections the brief leans on and their checks (u6a/u6b/u6c/u8 rev 1.1, advisor-u6a/u6b/u6c/u8),
   security-r1-deps.md (:232-255), security-u6c-deps.md, u6a-fix-pass.md's last section (O2 carry).
4. Grep the spike's code for each fact the brief states in section 0 and section 9 (client lib.rs, clock.rs, bots, lab
   scenario.rs, log lib.rs, client-wasm, Cargo.lock), plus U8's patch lines and U6a's patch lines.
5. Rule Q-U7-1..4, contradictions (a)-(i), size and R1's date, Block 0's security reads, and the rest (carries,
   tests, plants, bans, files, seats, not-verified, marks).
6. Write verdict, paste-ready conditions, answers.

## Notes (one line per finding, as I go)
- N1 Rules read word for word: NPT-TRIM history.md:7561-7593 (item 3 :7572; review seats :7590: "security only for
  code that faces the internet in a run"; one advisor check :7591); answers :7595-7607 (R1-DAY :7604, dates :7605);
  TEST-FIRST :2234-2282 (rule 2 :2261-2262). Card read (Rules, Now, Running work, U7 line :167-176).
- N2 O2 (u6a-review-netcode.md:155-158; u6a-fix-pass.md:79): "one home both the binary and the clients or page reach,
  the `mode_hash` too, with a test that they are equal". Code: `Join` carries version + content_hash only
  (proto/src/msg.rs:74-83), checked by the server's pump (server/src/pump.rs:484-497); `mode_hash` is JoinAccepted's
  (msg.rs:104), set [0;32] by U6a's binary (wip/u6a-build.patch:992), read by no client (grep: no hit in crates/client).
  Q-U7-2's lean names version and content hash only: it must take mode_hash and the equality test too. Medium (a carry
  met short; caught here, so no wire risk).
- N3 Section 0's code facts checked, all hold: client-wasm stub (Cargo.toml:26-28, lib.rs 2 lines); `with_clock`
  :363 (`max_lead.max(1)`), `input` :533-559, `step` :683-755 (next_due from the step count), Ping's `sent_ns` =
  step time :740-743; bots `with_clock(9)` :1305; lab :60-66 assert; clock.rs :65, :122-124, :163, :209; lock :663,
  :1879 (wasm-bindgen's four crates present, default = ["std"] in its 0.2.130 manifest, read from ~/.cargo/registry).
  Token `nptj_` + 22 (u6a patch :789, :1191); constants patch :548-551 (cite exact). plan.md:488 has `prev_send_ns`.
  C-U7-3/-4 quotes equal plan.md:1255 word for word.
- N4 Q-U7-3: the client's cap (9) can never equal the room's bound (12 ticks): it is the bound less a margin by design
  (client lib.rs:354-359; scenario.rs:56-60). "Equals" in advisor-u6a-brief.md:96-97 is read as the binding; the room's
  bound does not depend on cmd_timing (room.rs:594-595, 539). Confirm the lean, with the assert naming the one constant.
- N5 Medium: R3's in-process test cites "as npt-server's tests host it", but that harness is a Room behind npt-server's
  `Decoder` (server/tests/common/mod.rs:2-3, :12-16); Block 0b gives only npt-room and npt-sim, and npt-room takes
  decoded messages only (catalogue :56-57), so the builder must stop for a manifest or hand-write a second pump. Fix:
  npt-server as a third dev-dependency (no package; check.mjs's closures count shipped edges only, :59-66, :77-78).
- N6 Medium: the two halves build days apart from different bases, but the inbox/outbox packing's byte widths, channel
  values and `limits()` order are not fixed in the brief (only "channel, arrival ns, length, bytes"); the T-half's fake
  core would set them, the R-half could differ; first contact T5 (ii) on Sun 29, the lane's last day. Fix: fixed layout
  plus one shared golden vector both sides test.
- N7 Medium: R4's `web` step runs `tsc --noEmit` yet section 1 says the Node tests run "over tsc's output"; and a fresh
  worktree has no web/node_modules (gitignored: .gitignore:1), so after U7 merges every later lane unit's check.mjs
  fails its `web` step. Fix: emit to .gate/, install by `npm ci --ignore-scripts` when absent, fail never skip, a
  check.test.mjs pin.
- N8 -28 (contradiction (a)): replan.md:63-65 (his RE-PLAN-OK, history.md:7600, "the table's other leans": replan.md:105)
  keeps the carried crate's parts; replan-units.md:82-83, :138 is the drafter's fact sheet. marks.json -28 (:6194-6408):
  control part selects header `role: control` (:6248-6255, :6305-6313), `params.crate`; so Q-U7-4's `hash=wrong`
  needs save()'s header (Q-U7-1). The page's attempt.ok fits "datagrams + one bidirectional stream" (SERIES.md:132).
- N9 -56 item 2 (contradiction (g)): marks.json -56 notes (:10797) "each browser's negotiated draft"; -28's notes
  (:6408) name `event.attempt.draft` (a string, SERIES.md:831 Q16 open). No writer in u6b/u6c/u8 briefs (grep). The page
  cannot read it; the server (WtServer) sees the CONNECT and SETTINGS. Medium, outside U7: U6b's launch and Stop B.
- N10 U8 snapshot: save() (patch :398-404) writes page lines only; read_log needs the header first (log lib.rs:585-587)
  and a footer counting header + lines (:630-641; a file with none reads as cut short, :558-561). Header fields
  :57-90 (13 machine fields :24-42; date YYYY-MM-DD, run/set/role/process non-empty :405-440). -36 selects
  params.suite/browser/path, machine.name (marks.json:7337-7424). Q-U7-1 (a) holds; U8's fix pass item 9.
- N11 U8's /npt/cert-hash.json: only comments (patch :9 cert14d.mjs header, :1387 R1.md) = fix pass item 3, already.
- N12 U8's rs.yaml (patch :865-910): RS builds with `npm ci --ignore-scripts` + `node build.mjs`, CLI under $HOME/wbg,
  dist copied, dist.sha256 written; nothing compares it with R1.md's expected values on the day (only `keep`, :1686):
  the brief's "which Stop B compares" is wrong (R1's report compares). Low.
- N13 rust-toolchain.toml pins 1.99.1 with the wasm32 target; no rustflags in .cargo/config.toml; release has no debug:
  the remap-based byte equality is plausible, unverified until RS.
- N14 web/dist is gitignored (.gitignore:3 `dist/`), web/dist-bites is not: a bites build could be committed. Low.
  redact-check skips only .git, node_modules, target, .gate (:277): it walks web/dist when present. Low.
- N15 brief cites u6c-brief.md:126-128 for the allow-list: rev 1.1 has it at :241-246 (C9 sits at :124-127). Low.
- N16 Seats: U6b's and U6c's rounds take three seats (netcode, test, security: replan.md:93-94), so a builder cannot run
  beside them (3 agents at most); the T-half's Mon 23-Thu 26 window must avoid Wed 25 and Fri 27. Assumption, R1 ruling.
- N17 Medium: T5 (ii) runs the page from a Node test server, never from U6c's npt-host; U6c's own reading serves
  "fixture files" when no dist exists (u6c-brief.md:357-358), and the T-half cannot make the Wasm before R3. So the real
  host (its compiled allow-list, MIME types, the CSP word for word with the trusted name: u6c-brief.md:241-250) first
  serves the real page at R1, with him present. U6c merges before U7's R-half, so T5 (ii) can add one leg through it.
- N18 Block 0 coverage: security-r1-deps.md:238 (typescript CLEAR), :245-247 (npm rule), :218-220 (SEC-R1D-6),
  :232-233 (wasm-bindgen), :234-237 and :276-277 (the CLI, "U7 here, U8 on RS": this container is covered),
  :254-255 (U7's binding line); security-u6c-deps.md:186-190 (tools YES again). The brief's section 4 says security
  confirms the .npmrc substitute "at Stop B" while 0a installs now (section 5 lists it in the short read): Low.
- N19 Bans: .claude/rules/netcode.md read; the page's clock use is transport stamping, not shared sim code; queues
  bounded; npm-only typescript; nothing in the brief breaks them. -28/-36/-29/-14/PM-0006-25 series names exist
  (SERIES.md:90, :132, :154, :171); bite ids exist (marks.json:6541, :7493); windows "all"; no mark, judge or
  marks.json change anywhere in the brief or in my conditions.
- Notes DONE; the ruling follows.

---

# Ruling

## 1. Verdict: APPROVE WITH CONDITIONS (C1-C14 below)
No Critical and no High, so no re-check (NPT-TRIM, history.md:7591). Six Mediums, each read against its source in full
(N2, N5, N6, N7, N9, N17); C1-C14 close or route them, paste-ready. Nothing in the brief or in these conditions changes
a pass mark, the judge or marks.json: the series and bite names already exist, and the header fields are npt-log's own.
Risk tier of what U7 builds: 1 (spike code and two npm files under DEP-HOLD; no `.npmrc`, so no gate file).

| Id | Sev | Finding | Where | Fix |
|---|---|---|---|---|
| ADV-U7-1 | Medium | Q-U7-2 meets U6a's O2 carry short: O2 asks one home for the protocol version, the content hash "and the `mode_hash` too, with a test that they are equal" (u6a-review-netcode.md:155-158); the lean names two values and no equality test | u7-brief.md:323-327 | C2 |
| ADV-U7-2 | Medium | The Wasm boundary's byte layout (inbox and outbox field widths, channel values, `limits()` order) is not fixed, while the two halves build days apart on different bases; the first meeting is T5 (ii) on the lane's last day | u7-brief.md:108-117, :61-71 | C5 |
| ADV-U7-3 | Medium | R3's in-process test is "as npt-server's tests host it", which is a `Room` behind npt-server's `Decoder` (server/tests/common/mod.rs:2-3); Block 0b gives npt-room and npt-sim only, and npt-room takes decoded messages only, so the builder stops for a manifest or writes a second pump | u7-brief.md:153-155, :237-243 | C6 |
| ADV-U7-4 | Medium | R4's `web` step runs `tsc --noEmit` though the Node tests run "over tsc's output", and a fresh worktree has no web/node_modules (gitignored), so after U7 merges every later lane unit's check.mjs fails that step | u7-brief.md:118-120, :127 | C7 |
| ADV-U7-5 | Medium | -56 item 2 keeps "each browser's draft" (MARK-NOTES, his yes: history.md:7601; replan.md:109) but no brief writes it; the page cannot read it, the server can | marks.json:10797; u7-brief.md:142-143 | section 4 (g), C13 |
| ADV-U7-6 | Medium | U6c's npt-host never serves U7's real dist before R1: T5 (ii) uses a Node test server and U6c's reading uses fixtures (u6c-brief.md:357-358), so the real allow-list, MIME types and word-for-word CSP meet the real page first in front of him | u7-brief.md:181-191 | C8 |
| ADV-U7-7 | Low | A WebTransport attempt abandoned at 1.5 s could still become ready; nothing says it never joins, and T5 does not state a slot per load (C-U6-19's gate line) | u7-brief.md:72-76, :217-222 | C9 |
| ADV-U7-8 | Low | Section 4 has security confirm the `.npmrc` substitute "at Stop B", but Block 0a installs now; the short read lists the CLI, already cleared for this container (security-r1-deps.md:234) | u7-brief.md:229-230, :256-260 | C10 |
| ADV-U7-9 | Low | No release valve though the last units grew 1.9-2.3 times; R1 needs neither the page bites nor `wt-attempts` | u7-brief.md:297-303 | C11 |
| ADV-U7-10 | Low | Cites and small traps: the allow-list is u6c-brief.md:241-246 (not :126-128, now C9); "the build's byte equality on RS, which Stop B compares" (nothing compares it on the day: rs.yaml writes dist.sha256, `keep` copies it); web/dist-bites is not gitignored (only `dist/` is: .gitignore:3); one tsconfig with lib `dom` and `webworker` likely reports duplicate globals | u7-brief.md:58, :96-98, :197-198, :50 | C13; C7 (two configs) |

Count: 0 Critical, 0 High, 6 Medium (ADV-U7-5 is outside U7's code), 4 Low.

## 2. Conditions (paste-ready; the main session pastes them word for word into the brief's section 0.1 as rev 1.1)
**C1 (Q-U7-1 ruled; contradiction (e)), sections 1 (T2's records), 2 and 9.** "Q-U7-1 reads "Ruled (a)". The page
writes no header and no footer; U8's `save()` writes both around the page's lines (U8's fix pass item 9,
advisor-u7-brief.md section 5). Each page line is exactly npt-log's `event` form (`k`, `ev`, `ph`, `ns`, `f`) or
`second` form (`k`, `sec`, `ph`, `v`), with no other key (both structs deny unknown fields: crates/log/src/lib.rs:195-233);
`ph` is `measured`; `sec` and `ns` count from the Worker's start. Beside `window.nptRecords` the page sets two read-only
values for `save()`: `window.nptClockResNs` (the Worker clock's step: the smallest non-zero difference over 1,000
consecutive `performance.now()` reads at start, integer ns) and `window.nptDropped` (records dropped past the 4,096
cap). Before U8 merges, the main session writes one T5 (ii) leg's records through U8's `save()` and runs npt-judge over
that folder with -36's mark: the log must read (a verdict, never a read error)."

**C2 (Q-U7-2 ruled with U6a's O2 carry; ADV-U7-1), sections 1 (R3, R4), 6 and 9.** "Q-U7-2 reads "Ruled, the lean,
with U6a's O2 carry (u6a-review-netcode.md:155-158)". One home in npt-proto for the three values U6a's binary holds
today (wip/u6a-build.patch:548-551, :990-992), unchanged: `npt_proto::PROTOCOL_VERSION: u16 = 1`, `CONTENT_HASH:
[u8; 32] = *b"npt-server room content hash v1 "` and `MODE_HASH: [u8; 32] = [0; 32]`; npt-server's binary takes all
three from there and its private copies go. WsLink's tags in npt-link: `Channel::ws_tag(self) -> u8` and
`Channel::from_ws_tag(u8) -> Option<Channel>`, with a round-trip test and an unknown-tag test; the catalogue's WebSocket
line (docs/design/netcode-proof-messages.md:73-76) names the three tag values (.claude/rules/netcode.md: a protocol
change updates the catalogue). U6b builds them (the main session names this in U6b's launch prompt); if U6b's merged
code lacks them, R4 moves them, and crates/proto, crates/link, crates/net, crates/server/src/main.rs and that catalogue
line join the R-half's files for that only. The equality test, U7's half: R3's in-process test builds its room as the
binary does (npt-server's `Decoder` with `PROTOCOL_VERSION` and `CONTENT_HASH`; `Room::with_hashes(CONTENT_HASH,
MODE_HASH)`), the Wasm `Client`'s `Join` is accepted, and the `JoinAccepted` the room sends carries `CONTENT_HASH` and
`MODE_HASH`; T5 (ii)'s joins against the merged release binary are the end-to-end. No value changes, so no wire change."

**C3 (Q-U7-3 ruled), sections 1 (R2), 3 and 9.** "Q-U7-3 reads "Ruled, the lean". One pub constant,
`npt_client::HONEST_MAX_LEAD: u32 = 9`, read by npt-client-wasm, npt-bots (bots/src/lib.rs:1305), npt-lab (its
`CONTROLLER_MAX_LEAD`, scenario.rs:60, becomes that constant) and client/tests/trace.rs (:18); no literal 9 is left as a
cap. The binding is npt-lab's compile-time assert (scenario.rs:62-66), rewritten to name `npt_client::HONEST_MAX_LEAD`:
at most `InputBuffer::CMD_CATCHUP + 1` and below `ticks_for_ms_at(TICK_HZ, Room::LEAD_BOUND_MS)`. The ruling's
"equals" (advisor-u6a-brief.md:96-97) reads as this binding: the cap is the bound less a margin by design
(client/src/lib.rs:354-359), and the bound does not depend on `cmd_timing` (room.rs:594-595), so the page names no
option. Plant P17: the constant set to 12 must fail `cargo check` on that assert."

**C4 (Q-U7-4 ruled; contradiction (a)), sections 1 (T2), 2, 3 and 9.** "Q-U7-4 reads "Ruled, the lean". -28's
carried-crate parts stay kept, as replan.md:63-65 (his RE-PLAN-OK, history.md:7600) and replan-marks.md:48 say;
replan-units.md:138's "connect.ts (-28 moved)" reads as superseded, and no paper is edited. The page takes suite
`wt-attempts` with `cert-14d`'s attempt rules (WebTransport only, no fallback, one `attempt.ok` per load), and the
`hash=wrong` control: accepted only with `suite=wt-attempts` (a `hash` with any other suite is a malformed link), it
pins the room's 32 bytes each XOR 0xff, so the session must fail; the header's `role` `control` and `params.crate` are
`save()`'s (C1). The 100-attempt laptop script waits for R8's plan and stays off R1's path. Plant P19: `hash=wrong`
accepted with `cert-14d` must fail the URL test."

**C5 (the Wasm boundary; ADV-U7-2), sections 1 (T2, T3, R3) and 3.** "The boundary is fixed here, every integer
little-endian. The inbox (`step`'s third argument) is records of: channel u8 (0 `Datagram`, 1 `Control`, 2
`FullState`), arrival ns u64, length u32, then that many bytes; at most 512 records; a record past the buffer's end, an
unknown channel or a length above that channel's cap is refused and counted, and the rest of that inbox is skipped. The
outbox is records of channel u8 (0 or 1), length u32, bytes. `limits()` returns, in this order: the largest
control-stream message the client accepts (the largest room-to-client control row of `npt_proto::Limits::TYPES`; the
limit counts the length and type, catalogue :68-71), the largest full-state message with its length (as U6b's WtClient
caps it, 65,541 B: u6b-brief.md:159), the largest datagram the client accepts (`Limits::SNAPSHOT_RECEIVE`), then the
WebSocket tags of `Datagram`, `Control` and `FullState` (C2). `counter_names()` and `counters()` pair by index; the page reads names, never positions. One golden file,
`web/test/fixtures/boundary.json` (the T-half's: hex of an inbox with one record per channel and of an outbox with two
records, each with its decoded fields), is read by a Node test against the fake core and by an R3 Rust test
(`include_str!`) against the real packing; plant P18 (the arrival ns packed as u32 on the Rust side) must fail the Rust
test."

**C6 (R3's room; ADV-U7-3), sections 1 (R3), 3 and 5.** "Block 0b's `[dev-dependencies]` in crates/client-wasm are
`npt-room.workspace = true`, `npt-server.workspace = true` and `npt-sim.workspace = true`, so R3's in-process test hosts
the `Room` behind npt-server's real `Decoder`, as npt-server's tests do (crates/server/tests/common/mod.rs:2-3), never a
hand-written pump. Expected lock diff: npt-client-wasm's dependency array gains ` "npt-room",`, ` "npt-server",`,
` "npt-sim",` and ` "wasm-bindgen",` and nothing else (no package, version, checksum or source line). The dev edge ships
nothing: check.mjs reads shipped edges only (check.mjs:56-78), so R4's deps rule (npt-client and wasm-bindgen only)
holds; npt-bots already has the same dev edge to npt-server."

**C7 (the `web` step; ADV-U7-4), sections 1 (R4) and 3.** "R4's `web` step: (1) when
web/node_modules/typescript/package.json is absent or not version 6.0.3, `npm ci --ignore-scripts` in web/ with
`npm_config_ignore_scripts=true` in its environment (never `npm install`); (2) `tsc` over web's config(s), emitting to
`<checkout>/.gate/netcode-proof-test/web-check/` (never web/dist), with `tsc -b` if the page and the Worker take two
configs; (3) `node --test 'web/test/*.test.mjs'` over that emitted output (its path passed in the environment). The step
fails, never skips, when any of the three fails. check.test.mjs pins the step's commands as it pins the cargo steps'
(check.mjs:2612-2613), and the self count changes with it, stated."

**C8 (the real host before R1; ADV-U7-6), section 3 (Block T5 (ii)) and section 10.** "T5 (ii) adds one leg through U6c's
merged npt-host (its release build, its compiled allow-list and CSP unchanged) serving the measured dist: Chromium
started with `--host-resolver-rules` mapping rs.nt.shamblefrontlines.com to 127.0.0.1, the host and the room's WebSocket
listener on a test certificate for that name admitted only by the test context's `ignoreHTTPSErrors`, the room's
WebTransport hash pinned as on RS. Pass: `smoke` on `wt` shows "WebTransport · Playing" and on `ws` "WebSocket ·
Playing", every dist file loads with its listed type, and the console shows no CSP violation. If the mapping does not
reach QUIC in this Chromium, the leg records what loaded and says so; any mismatch with U6c's host is a one-line fix on
one side before U7 merges, named in the report."

**C9 (ADV-U7-7), sections 1 (T2), 3 and 4.** "A WebTransport attempt abandoned at the 1.5 s timeout is closed and never
joins, even if it becomes ready later: the `Join` goes on exactly one transport per load (a Node test: wt ready at 1.6 s,
after the fallback began, is closed with no `Join` on it). T5 gives each page load its own slot, or restarts the room
between loads, as C-U6-19's gate line asks."

**C10 (Block 0; ADV-U7-8), sections 4 and 5.** "Section 4's "(security confirms the substitute at Stop B)" reads
"(security-auditor's short read confirms the substitute for web/ before Block 0a; Stop B confirms U8's and RS's)".
Covered already, no new read: typescript 6.0.3 exact (security-r1-deps.md:238); the npm rule and SEC-R1D-6
(:245-247, :218-220) with C2's substitute; wasm-bindgen =0.2.130 with its conditions (:232-233; its 0.2.130 manifest has
`default = ["std"]`); wasm-bindgen-cli 0.2.130 in this container and on RS (:234-237, checksum :276-277); cargo-deny
0.22.0 and cargo-audit 0.23.0 (security-u6c-deps.md:186-190). security-auditor's one short read, now, for both halves,
covers: (1) web/package.json exactly: `name`, `"private": true`, `"type": "module"`, `"devDependencies": {"typescript":
"6.0.3"}`, and no `scripts`, `overrides` or other dependency key; (2) web/package-lock.json as made: lockfileVersion 3,
only the root and `node_modules/typescript` entries, version 6.0.3, `resolved`
https://registry.npmjs.org/typescript/-/typescript-6.0.3.tgz with an sha512 `integrity`, no `hasInstallScript`; (3) how
it is made: `npm_config_ignore_scripts=true npm install --package-lock-only --ignore-scripts --before=<7 days back>`,
then `npm ci --ignore-scripts` in the same environment, `npm config get registry` read back as
https://registry.npmjs.org/ (user and global npmrc read, never changed), npm's and Node's versions recorded; (4) the
`.npmrc` substitute for web/ (advisor-u8-brief.md C2), now, since web/'s install precedes Stop B; (5) `npm audit`'s
output recorded; (6) check.mjs's redact step clean on the new lock (F26's fix); (7) Block 0b's manifest lines exactly:
the workspace `wasm-bindgen = "=0.2.130"` with no `features` or `default-features` key, crates/client-wasm's
`wasm-bindgen.workspace = true` in place of its comment line, and C6's three dev-dependencies; (8) the lock diff model:
`git diff --numstat -- Cargo.lock` prints `4	0`, the four lines of C6 only, the package count equal to U6c's merged
lock; (9) the feature tree: `cargo tree --locked --offline -p npt-client-wasm -e features --target
wasm32-unknown-unknown -i wasm-bindgen` shows default and std only, and `-e normal` names no js-sys, web-sys,
wasm-bindgen-futures or serde; (10) `cargo deny --locked check advisories bans sources` and `cargo audit` as U6c's
binding check step 6 words them, database dates recorded, `cargo check --workspace --all-targets --locked` 0 warnings,
check.mjs's test and deps counts unchanged by 0b. Block 0a runs after that answer; Block 0b's binding check runs the hour
U6c merges, and only a difference from (7)-(10) goes back to security."

**C11 (the valve; ADV-U7-9), section 8.** "After the T-half, the T-builder writes its added lines in its notes; if they
pass 3,800, the page bites (T4's `--bites` half, its tests, P15 and T5's pass mark 8) and the `wt-attempts` suite (C4,
P19) are not built in U7 and ride before R8 (R1 runs no page bite and no -28 leg), named in that day's message. Past
6,000 SIZE-CHECK applies as written (history.md:2048-2051)."

**C12 (review), section 7.** "Security does not sit in U7's round: U7's code listens on nothing and runs only in the
browsers RS's firewall admits (NPT-TRIM, history.md:7590; replan.md:93-94). Stop B's security seat adds one item: the
page's handling of the slot token and the bind value (fragment, Worker, Wasm; never in a record, status, console line
or log; the dist scan), read on U7's merged code beside U6c's host. The netcode seat also reads C2's homes and C5's
boundary; the test seat C5's golden file, C8's leg and P17-P19."

**C13 (ADV-U7-5, ADV-U7-10), sections 0, 1, 2, 3 and 10.** "Section 0's allow-list cite reads u6c-brief.md:241-246.
Section 3's "(the build's byte equality on RS, which Stop B compares)" reads "(the build's byte equality on RS: Stop B
fills R1.md's expected values from the merged commit; R1's report compares them with RS's dist.sha256, which `keep`
copies; a mismatch is recorded and stops nothing)". `node build.mjs --bites` writes to
`<checkout>/.gate/netcode-proof-test/dist-bites/` (gitignored), never under web/. Section 2's and section 10's lines on
-56 item 2 read: "the page cannot read the negotiated draft; the server can: U6b's launch prompt asks its builder to
record, if web-transport-quinn 0.13 exposes it at accept, each WebTransport session's draft in the room's log (its name
to the advisor with U6b's new names, a string per SERIES.md:831's Q16), else to say so in its notes; R1.md names the
source at Stop B's round; with neither, R1's report records item 2 NOT-RUN and it comes to the advisor before -56 is
judged". The T-builder re-reads U8's config.mjs and lib.mjs from U8's newest snapshot at its start and before T5, with
file:line in its notes (the brief's U8 cites are patch lines)."

**C14 (counts), section 3.** "With C2, C5 and C6, at least +20 measured and +20 bites in Rust (+2 more if R4 moves the
tag functions); about +34 Node tests (C4's refused `hash`, C5's golden, C9's late-ready attempt, C7's check.test.mjs
pin); plants P1-P19 (P17 from C3, P18 from C5, P19 from C4); T5's pass marks 1-9 plus C8's leg."

## 3. Q-U7-1 to -4
- **Q-U7-1 (the browser logs' header): (a), the lean.** Only the laptop knows the run, the set, the machine and the date;
  the page keeps to what it measured. npt-log needs the header first (crates/log/src/lib.rs:585-587) and a footer for a
  complete log (:558-561, :630-641); -36's parts select `params.suite`, `params.browser`, `params.path` and
  `machine.name` (marks.json:7337-7424), -28's control part the header's `role` (:6248-6255): all header fields, so
  `save()`'s. The page adds only the clock step and its drop count (C1). Into U8's fix pass now (section 5's item 9),
  so Stop B's round reads an aligned `save()`.
- **Q-U7-2 (the shared constants): the lean, widened by O2** (C2): npt-proto's three constants with `MODE_HASH`,
  npt-link's tag functions, the catalogue line, built by U6b (named in its launch prompt), else R4; the equality is
  R3's test against a room built as the binary builds it, plus T5 (ii) against the real binary.
- **Q-U7-3 (C-U7-3's cap): the lean, confirmed** (C3). "Equals" cannot hold (9 against 12 ticks, by design), so the
  binding is the compile-time assert naming the one constant; it holds under both `cmd_timing` options.
- **Q-U7-4 (-28's carried-crate parts): the lean** (C4): `wt-attempts` and a bounded `hash=wrong` in the page now
  (about 30 lines), the 100-attempt script with R8's plan; first in line for the valve (C11).

## 4. The contradictions (a)-(i)
- **(a) -28.** Real. replan.md:63-65 is his approved plan (RE-PLAN-OK yes, history.md:7600; "the table's other leans",
  replan.md:105) and replan-marks.md:48 agrees; replan-units.md:82-83, :138 is the drafter's fact sheet. No document
  changes (older papers are not rewritten: advisor-u6b-brief.md:143-145); the brief per C4. U8: nothing now (the
  100-attempt script is R8's plan, after R1).
- **(b) `web/.npmrc`.** advisor-u8-brief.md C2 rules (any `.npmrc` is a gate file: main's advisor.md, escalation 3).
  security-r1-deps.md:245 is not edited; the brief's section 4 per C10; security's short read confirms the substitute
  for web/ before 0a. U8: nothing (C2 is built; Stop B confirms U8's and RS's).
- **(c) C-U7-1 to -4 at plan.md:1255.** Checked word for word. No document changes; U7's report meets C-U7-1 and -2
  (GO, replan.md:78-79), -3 and -4 by number from plan.md:1255, and u4s-carries.md:831 (cut with the Origin checks,
  replan-units.md:19) and :870-871 (U6c's now) by number. U8: nothing.
- **(d) `/npt/cert-hash.json` in U8.** Only comments: cert14d.mjs's header (patch :9) and R1.md (patch :1387). Already
  U8's fix pass item 3 (advisor-u6c-brief.md:339-340); nothing new. The main session's interface check at U6c's merge
  confirms it.
- **(e) `save()` writes no header.** Q-U7-1 (a): U8's fix pass item 9 (section 5), now, not Stop B's round.
- **(f) No library holds the content hash or the tags.** Q-U7-2 with O2 (C2): U6b's launch prompt, else R4. Not U8's.
- **(g) -56 item 2's draft.** Real (ADV-U7-5), not U7's code. Waits for two homes, worded in C13: U6b's launch prompt
  (the server side records it if web-transport-quinn exposes it) and R1.md at Stop B's round (the source). Not U8's
  fix pass: R1.md's record step is read in Stop B's round.
- **(h) replan-units.md's plan.md cites past :379.** Explained by 4d71571 (advisor-u6c-brief.md N0). No document
  changes; the brief cites live lines. U8: nothing.
- **(i) U8's cites are patch lines.** No document changes; they are labelled as patch lines (wip/u8-build.patch:...);
  the T-builder works from U8's newest snapshot with file:line in its notes (C13). U8: nothing.

## 5. U8's fix pass, item 9 (paste-ready, appended to advisor-u6c-brief.md section 5's message before "Report:")
"9. The browser logs' header and footer (Q-U7-1 ruled (a), advisor-u7-brief.md). `save()` writes each leg's `.npt.log`
as one npt-log log that crates/log's `read_log` accepts (crates/log/src/lib.rs:558-656): line 1 a header
`{"k":"header",…}` with exactly `Header`'s fields (:57-90, deny_unknown_fields): `schema` npt-log's `SCHEMA` (a test
reads it from lib.rs); `run`, `set` and `date` (YYYY-MM-DD) from the run; `role` `measured`, or `control` on a
`wt-attempts` leg with `hash=wrong`, or `bite` on a bites-dist leg; `marks` the suite's marks from R1.md's table;
`machine`'s 13 fields (:24-42): role `ML`, name `ML*` until he names the laptop, `address` empty, `location` `home`,
plan, arch, cpu, vcpus, mem_mb, kernel, threads_per_core and smt from Node's `os` where it gives them (else empty, 0 or
[], never invented), `clock_res_ns` the page's `window.nptClockResNs`; `build` with `commit` the run's commit Y,
`binary_sha256` the SHA-256 of `/npt_client_wasm_bg.wasm` fetched once per run from the page host, `features` [] (or
["bites"]), `profile` `release`; `bite` the armed id or null; `layer` `real`; `profile` `real-path`; `seed` 0; `tick_hz`
30; `clients` 1; `scene` the room's; `process` `browser`; `slot` the leg's slot; `params` `suite`, `browser`, `path`
(`eu-path` in R1), `transport`, `room`, `attempt`, `version` (the page's `#version` text), `records_dropped_count`
(`window.nptDropped`), and `crate` on `wt-attempts` legs. Then the page's lines unchanged; then
`{"k":"footer","records":<1 + the page's lines>,"ended":"clean"}`, or `"stopped"` when the leg ended on its time
limit. `read()` also returns `nptClockResNs` and `nptDropped` (config.mjs's PAGE gains both names). The header builder
is a pure function in config.mjs (no WebDriver import), tested on fixture page lines: the field set equal to `Header`'s,
the footer's count, every page line unchanged, no token or URL in the file. Plant: the footer's count without the
header must fail that test."

## 6. Size and R1's date
- **Size:** the brief's 2,200 (Rust 660 on the lane) at 1.9-2.3 is 4,200-5,050, under the 6,000 stop; C2-C9 add about
  150-250. C11 names the valve, so no SIZE-CHECK stop is likely.
- **R1 on Thursday 3 December holds for U7, with about one day of slack (Wed 2), on these assumptions:** (1) U6a's fix
  pass and its second round on the one High close by the end of Tue 24 (the lane already planned U6a's round, fixes and
  merge Mon 23-Tue 24: advisor-u6c-brief.md:271); a later merge shows first in U6b's trigger (end Thu 26); (2) U6b and
  U6c each close in one round, U6c merged Sat 28 (trigger end Sun 29); (3) agent seats: U6b's and U6c's rounds take all
  three seats (Wed 25, Fri 27), so the T-builder and U8's fix-pass builder run around them, and the T-half and T5 (i)
  finish by the end of Thu 26; (4) security's one short read (C10) is in before Block 0a, so 0b is only its binding
  check the hour U6c merges; (5) U7's R-half Sun 29, its one round and merge by the end of Mon 30, R1.md's dist SHA-256s
  from the merged commit before Tue 1; (6) no SIZE-CHECK stop and no cut longer than half a day. **Trigger** as the
  brief has it: U7 not merged by the end of Mon 30 sends him three lines that day (closer: the split R1 of
  replan.md:45-46). Not a decision today: one line in the day's message.
- **Levers to take now:** security's short read for both halves today (C10); Block 0a and the T-builder today, ahead of
  U6b's round; Q-U7-1 into U8's fix pass (section 5) and Q-U7-2's homes and the draft line into U6b's launch prompt
  (C2, C13); the boundary fixed now (C5), so the R-half reads no T-half code to start; the valve named (C11).

## 7. The rest
- **Carries met by number:** C-U7-3 (Block R2, C3's form), C-U7-4 (Block R1); C-U7-1 and -2 GO (replan.md:78-79), the
  tool half of C-U7-2 recorded as the container's (Chromium 1243, playwright-core 1.56.1, both checked here); C-U6-19's
  line (written; C9 adds the slot per load); u6b-brief.md condition 4 (the hash file's bytes); advisor-u6c-brief.md C1 (e)
  and C9 (per-room URL, host ignores queries); advisor-u8-brief.md C2 (no `.npmrc`); U6a's O2 (C2); u8-build.md F14
  (rs.yaml's build line, taken).
- **Tests, bites, plants:** each plant tied to a named test; P17-P19 added. The two page bites exist in marks.json
  (:6541, :7493) and R1 runs neither (u8 R1.md's bites list, patch :1491-1493). Plants run once each, restored and
  sha256-checked.
- **Bans:** the brief's list stands with .claude/rules/netcode.md; the clock is transport stamping in the Worker only;
  every Worker queue and the framing buffers bounded by C5's caps.
- **Files:** as section 6, plus C2's fallback files (only if R4 moves the constants) and C7's check.test.mjs pin; U8's
  files stay U8's (C1's header is U8's fix pass); every manifest and lock Block 0's.
- **Not verified here:** no build or browser run; Chromium's `--host-resolver-rules` reaching QUIC (C8 says what
  happens if not); the tsconfig `dom` with `webworker` clash (from TypeScript's known lib overlap, not run); whether
  web-transport-quinn 0.13 exposes the negotiated draft; Firefox, Edge, Brave; the byte equality on RS; typescript
  6.0.3's publish date against `--before` (security's read checks it).
- **Pass marks, the judge, marks.json:** untouched (N19).

## Decisions needed
None. Every ruling here is inside the plan, undoable, and touches no pass mark, money or main (TEST-FIRST rule 2,
history.md:2261). R1's three lines go to him only on a trigger in section 6.

## For Mustafa
Nothing from this check. The day's message names: U7's brief rev 1.1 (no High; six Mediums closed by its conditions);
R1 on Thursday 3 December holds with about a day of slack, and its trigger.

DONE
