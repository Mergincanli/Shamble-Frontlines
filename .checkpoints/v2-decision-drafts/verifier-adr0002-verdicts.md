# Verifier verdicts: ADR 0002 (code structure, build and toolchain), plan step 7

Document: `docs/adr/0002-code-structure-build-toolchain.md` (1422 lines at a2b3874). Assembly checked: doc 1–937 =
engine-architect draft 1–937 (cmp); doc 938–1171 = `test-engineer-passmarks-0002.md` 11–244 (cmp; includes the end
marker); doc 1173–1422 = draft 940–1189 (cmp). Research files, UNREAD_SOURCES and LEGACY_INVENTORY unchanged since
9e7db28 (`git diff --quiet` exit 0); GLOSSARY unchanged (742 lines, §8 not yet appended: its text is the decision map
J1). Ledgers: engine-architect rows 1–261, test-engineer TE1–TE271; added rows A1… for claims with no ledger row.

Verdicts: confirmed · wrong · unsupported · citation off · unverified · pending (outside fetch hours: the source check
needs a web page and it is before 08:30 Amsterdam). "confirmed (R)" = the research says it at the cited place; where the
claim rests on a finding outside the verified set, "; source pending" follows. Proposed marks: "confirmed (proposed,
labelled)" when the doc labels them proposed with a reason. Cross-draft and inventory rows are judged against the
drafts and `docs/LEGACY_INVENTORY.md` rows they cite.

Verified sets used (condition 16): R1 = every claim the R1 recommendations rest on (r1:3233) — R1-BND findings all
checked except F13, F14, F17 (partly: a/b/c confirmed in table), F23c (now OQ9), F24 (a/b confirmed in table), F31;
F2 confirmed for the version match, 0.2.123 and the dates (F2b partly). R5 = r5:2108–2222 + second pass: LAYOUT F1, F2,
F4, F5, F7, F12, F14, F15; TSLINT F1, F4, F5, F13, F15, F16, F21, F22; RUSTLINT F1, F2, F7, F8, F9, F11, F13, F14, F15;
TEST F2, F3, F5, F6, F8, F9, F12, F13, F14; CI F1, F2, F3, F5, F7, F9, F10, F13, F14, F15, F17, F18, F20, F22; PREV F1–F4
(F3's secure-use quotes not), F6, F12, F13, F14, F18, F20; GATE F1, F2, F4. R5 recommendation text is NOT research
verified (C16): a REC claim is confirmed (R) and its source check is the findings it rests on.

## Engine-architect ledger rows

| # | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|
| 1 | The foundation, quoted word for word | GLOSSARY §8 (map J1); brief §2 | confirmed | map J1 lines 1068–1078 and brief 52–59: identical text (diff, blockquote prefix aside) | §8 not yet in GLOSSARY (lands step 6); the anchor `#the-foundation` dangles until then |
| 2 | SLICE-ORDER slices 1–6 and three conditions | STATUS Decisions pending; brief §1 | confirmed | STATUS:94–101 lists slices 1–4 and the three conditions; slices 5 "Octopon-3 and Editor v2" and 6 "scale to 48" only in brief §1 (DRAFTS_BRIEF:31–37) | note N-1: slices 5–6 have no lasting source until docs-keeper records them (C12) |
| 3 | SPIKE-SCOPE (a) | STATUS | confirmed | STATUS:187–188 "once the drafts are approved, the legacy constraints apply to `legacy/` only and `spikes/` is throwaway" | |
| 4 | VISION_V2 §6 items served (§6.1–§6.4, §6.6–§6.9) | VISION_V2 §6 | confirmed | VISION_V2:62–81; each item's wording matches the doc's gloss (§6.2 aiming at height; §6.4 big multi-part vehicles; §6.8 moments) | |
| 5 | VISION_V2 §3/§4: design for 48, bots fill slots, four browsers, 30+ fps on Low, about 100 ms | VISION_V2 §3, §4 | confirmed | VISION:46–55 "Bots fill empty slots."; "Chrome, Edge, Firefox and Safari"; "30+ fps on Low"; "about 100 ms of ping"; 48 = the design point, GLOSSARY:356 "the design point is N = 48, the 24v24 goal (VISION_V2 §3)" | VISION_V2 has no "48"; the gloss rests on GLOSSARY's reading |
| 6 | MODES-3 (a): largest mode 24v24, N = 48 | STATUS; GLOSSARY 335–354 | confirmed | STATUS:104–105 "the biggest match is 24v24 = 48"; GLOSSARY:343 "Answered … the largest mode is the goal 24v24, N = 48" | ledger's STATUS lines 102–104 are off (104–105); doc gives no lines |
| 7 | TICK-30: 30 Hz baseline, 60 Hz only with the 60 Hz margin ≥ 25 %, ADR 0001 owns it | STATUS; GLOSSARY §8 | confirmed | STATUS:118–120; map J1 (N3 owner 0001) | "headroom" not used: rule 5 OK |
| 8 | 30 blocks a second at 30 Hz | arithmetic | confirmed | 1 block/tick × 30 ticks/s = 30 | |
| 9 | PREVIEW-WALL (a) | STATUS; GLOSSARY | confirmed | STATUS:106–107 "private from the first preview that reaches the test game server" | ledger STATUS lines off by one |
| 10 | BASELINE-WHEN (b) | STATUS; GLOSSARY | confirmed | STATUS:112–114 "List it in ADR 0006 as open, with temporary positions." | ledger lines 114–115 off (112–114) |
| 11 | EDITOR_V2 §5's two lines quoted | EDITOR_V2:51–52 | confirmed | EDITOR_V2:51 "It still runs by double-clicking editor.html. The player build (index.html) must never load editor code." :52 "Written so it ports cleanly if ADR-002 moves the project to modules." | |
| 12 | LOOK_V2 §8: no external assets ends; glTF/GLB allowed; build serves model files | LOOK_V2 §8 | confirmed | LOOK_V2:65 "The legacy rule 'no external assets' ends with the legacy engine."; :69 "a glTF/GLB export" | |
| 13 | wasm-bindgen ≥ 0.2.128 CLI = library; releases broke callers in 2026 | r1 R1-BND-F2 894–902; REC4 2297–2308 | confirmed | r1:895–897 "Requires matching `wasm-bindgen` and CLI versions (schema bump)."; 0.2.123 "a breaking change for callers that relied on widening `&mut` references."; year from asset timestamps (r1:899–900); verifier r1:3401–3402 | research verified (F2a; F2b partly confirmed, .122 dropped) |
| 14 | Module workers in all four browsers; both transports open in a worker | r1 F25 1073–1077; F29 1098–1103 | confirmed | r1:1073 "Module workers work in all four target browsers"; r1:1098 "Both transports the netcode plans can be opened inside a worker" | research verified (r1:3430, 3434) |
| 15 | Shared memory needs isolation; Safari isolates only with `require-corp` | r1 F12 964–970; F22 1047–1052 | confirmed | r1:966 "To use shared memory your document must be in a secure context and cross-origin isolated."; r1:1047 "Safari has no `credentialless`, so an isolated page on Safari must use `require-corp`" | research verified |
| 16 | One transferred buffer per tick works everywhere without isolation | r1 REC2 2265–2285 | confirmed | r1:2274 "one bounded copy, without isolation or nightly Rust" | research verified |
| 17 | `std::thread::spawn` panics; `-Cpanic=abort`; one panic ends the instance | r1 F19 1019–1025 | confirmed | r1:1020–1023 "`std::thread::spawn` will panic." … "compiled with `-Cpanic=abort`." (so a panic ends the sim's instance: R1-RR-OQ3) | research verified |
| 18 | Cargo builds a dependency with the union of features | r5 R5-LAYOUT-F4 83–91 | confirmed | r5:85–86 "Cargo will use the union of all features enabled on that dependency when building it." | research verified (r5:2120) |
| 19 | Only clippy's correctness group denies by default; without `-D warnings` the rest only prints | r5 R5-RUSTLINT-F1 306–311 | confirmed | r5:308–309 "The `clippy::correctness` group is the only lint group in Clippy whose lints are deny-by-default" | nit: allow-by-default groups (pedantic, restriction) print nothing at all; "every other lint only prints" holds for warn-level lints |
| 20 | `iter_over_hash_type` sees only `for` loops | r5 F9 348–354 | confirmed | r5:350–353 "in for loops" … "an iterator chain … outside a `for` is not flagged" | research verified |
| 21 | typescript-eslint supports `>=4.8.4 <6.1.0`, not TS 7 | r5 F4 192–195; F5 196–201 | confirmed | r5:194 "`>=4.8.4 <6.1.0`"; r5:199–200 "typescript-eslint isn't compatible with TS 7 at this time" | research verified |
| 22 | Playwright cannot drive branded Safari | r5 F8 454–462 | confirmed | r5:458 "Playwright doesn't work with the branded version of Safari since it relies on patches." | research verified |
| 23 | A dependent job "is skipped and may not block merging"; path filters leave required checks pending | r5 CI-F20 695–700 | confirmed | r5:697–698 "Associated checks stay in a "Pending" state and block merging"; "is skipped and may not block merging." | research verified |
| 24 | Forks get no secrets; `pull_request_target` and `workflow_run` hand secrets and write tokens | r5 PREV-F1–F3 765–782 | confirmed, with a nit | r5:765–766 "secrets are not passed to the runner when a workflow is triggered from a forked repository."; r5:776–777 workflow_run "is able to access secrets and write tokens"; F2 (r5:770–774) says `pull_request_target` runs "with its trust" and "Avoid using this event if you need to build or run code from the pull request." | F2 never says "secrets and write tokens"; that quote is F3's (workflow_run). Finding F-9 (low) |
| 25 | Vite's size limit only warns | r5 GATE-F2 950–953 | confirmed | r5:950–951 "`build.chunkSizeWarningLimit` is the "Limit for chunk size warnings (in kB)."" | research verified |
| 26 | Client host: no file larger than 25 MiB | r6 HOST-F3 72–79; N37 | confirmed | r6:75–76 "The maximum file size for a single Cloudflare Pages site asset is 25 MiB." | research verified (tsv yes) |
| 27 | WebTransport only in secure contexts | r2 TRN-F35 1268–1272 | confirmed | r2:1269 "This feature is available only in secure contexts (HTTPS)" | research verified |
| 28 | Takes and gives between drafts (I-2, I-3, I-5, I-13, I-14, I-17) | decision map a, d | confirmed (cross-draft) | map:66–98 (0002 decides 1–9; "Takes: the protocol (0001), the tick and determinism rules (0004), …"); map part d I-2, I-3, I-5, I-13, I-14, I-17 owners 0002 | the decision map is a checkpoint file (leaves the branch): note N-2 |
| 29 | Today's gate: base ref, exit 2, never silent skip, `--json`, mutation proofs | inventory TL1:386 | confirmed (inventory) | TL1 mark "The base-ref, exit-code and mutation-proof rules carry over"; r5:934–938 "never a silent skip … `--json` output" | |
| 30 | 22 of 22 planted defects failed exactly the expected checks | r5 534, 938 | confirmed (and at source) | r5:534–535 and r5:938 "22 of 22 planted defects failed exactly the expected checks"; source in repo: docs/reports/2026-10-14-gate-check-5.md:8 "22 of 22 mutation proofs fail exactly the expected checks" | source is a repository file, read locally |
| 31 | TL2, TL3, TL5, TL6, TL8 exist only for classic scripts: drop | inventory 387–393 | confirmed (inventory) | each row's mark is **drop** (lines 387, 388, 390, 391, 393) | |
| 32 | Player build never reaches editor code, enforced by heuristics; EDITOR_V2 §5 keeps it | TL4:389; EDITOR_V2:51 | confirmed (inventory) | TL4 mark "EDITOR_V2 §5 (:51-52) keeps the rule … The mode-guard, typeof and reachability heuristics are legacy-only" | |
| 33 | Smoke shape carries; network rule flips | TL9:394 | confirmed (inventory) | TL9 mark "The harness shape carries over (state waits, … console errors and blocked network as failures, renderer string …) … the network rule flips" | |
| 34 | Tuning dump byte-identical over two runs | TL11:396; r5 1444–1445 | confirmed (and at source) | r5:1444–1445 "the tuning dump was byte-identical over two runs"; source docs/legacy-tuning.json:8 "two runs per map gave byte-identical output" | TL11's row itself does not say it (the r5 line does) |
| 35 | Exact pins, lockfile, `npm ci`, CI hygiene proven | TL17:402; KT8:417 | confirmed (inventory) | TL17 "Pins the toolkit's exact dependencies … The lockfile is installed with npm ci"; r5:980–981 (KT8: "SHA pins, read-only token, `persist-credentials: false`, `--ignore-scripts`, concurrency cancel") | |
| 36 | The SIM fence "leaked through comments"; frame loop in render loop; command never wired | r1 1176–1178, 1159–1164, 1150–1154 | wrong (wording) for CS11; confirmed for GP13/GP14/CS1 | r1:1176–1177 "The comment fence leaked: shape data came from the visual recipe and `hasLOS` read a global."; CS11 (inventory:179) "it is not self-contained: BODY reads CHAR_DEFAULT …, hasLOS reads the global SIM" | the fence is made of comments; it leaked through code crossing it, not "through comments". Finding F-6 |
| 37 | Legacy combat step 0.80–1.00 ms at 48 soldiers | r1 GC2 1168–1171 | confirmed | r1:1168–1169 "perfbots at 48 soldiers: combat 0.80–1.00 ms per update step (legacy JavaScript, not Rust)" | research verified (r1:3445) |
| 38 | Two hand-synced pages only because of classic scripts and file://; r128 CDN tag has no SRI/CSP | PG1:359; PG19:377; STATUS; map J2 | confirmed (inventory) | PG1 mark "Two hand-duplicated pages exist only because of classic scripts and file://."; PG19 "with no integrity hash and no CSP" | "STATUS item CDN-SRI": STATUS:262 has the item but not the code CDN-SRI (the map's J2 name); note N-2 |
| 39 | One repository, both workspaces at the root (option A), `legacy/` beside | r5 REC1 1153–1169; options 997–1002 | confirmed (R) | r5:1153 "One repository with both workspaces at its root (option A)"; r5:1158 "`legacy/` sits beside `crates/` and `packages/`"; option A at r5:999 | rests on LAYOUT-F1 (verified) |
| 40 | Several repositories would break R1's version matching | r5 1002; R1-BND-F2 | confirmed | r5:1001 option C "version skew breaks the identical-results rule (R1-DET-REC6) and the CLI-library pin (R1-BND-F2)"; r5:1162–1163 | |
| 41 | Five files move together, keep relative paths, `legacy/index.html` runs by double-click | R5-LAYOUT-REC6; CLAUDE.md | confirmed (repo) | r5:1233 "The five game files move to `legacy/`"; index.html:1256–1257 and editor.html:1257–1259 load the engine files by bare relative name, three.js from the CDN | proven again by phase 2.1's Brave step |
| 42 | `fuzz/` outside default members: cargo-fuzz needs nightly | r5 1175–1176 (R5-TEST-F4) | confirmed (R and source) | r5:1175–1176 "Fuzz targets live in `fuzz/`, outside the workspace's default members, because they need a nightly compiler (R5-TEST-F4)" | R5-TEST-F4 source confirmed (S-4: raw README, github.com itself 403) |
| 43 | Nothing imports `legacy/` (TSLINT-REC2) or `spikes/` (the same rule) | r5 1265–1282; SPIKE-SCOPE | confirmed (R and source) | r5:1269 "nothing ↛ `legacy/`"; `spikes/` is the doc's extension (stated as such) | import rule's R5-TSLINT-F14 source confirmed (S-1) |
| 44 | Today's gate and the oracle keep working on `legacy/` until legacy retires | r5 REC6 1233–1245 | confirmed (R) | r5:1236–1237 "today's gate keeps guarding critical fixes to legacy and the legacy oracle can still produce golden values … until legacy retires" | rests on legacy facts only |
| 45 | `.gitignore` additions | r5 1237–1238 | confirmed (R) | r5:1237–1238 lists exactly `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`, `test-results/` and proptest's and insta's pending files | |
| 46 | Legacy gate's place is the restructure plan's call (OQ-0002-2) | r5 LAYOUT-OQ5 1761–1762; GATE-OQ3 1871–1872 | confirmed (R) | r5:1871–1872 "Both in `tools/`, or the legacy gate inside `legacy/`. Decides it: the phase 1 restructure plan" | |
| 47 | Exact stable Rust, 1.99.1 when read | r5 F5 92; REC1 | confirmed | r5:1156 "(1.99.1 today)"; verifier r5:2118 "Announcing Rust 1.99.1" | research verified |
| 48 | Toolchain file components, `profile = "minimal"`; `rustup toolchain install` first | r5 CI-REC2 1523–1535 | confirmed (R and source) | r5:1524–1526 "`targets = ["wasm32-unknown-unknown"]`, components `clippy` and `rustfmt`, and `profile = "minimal"`. CI installs it with `rustup toolchain install`" | F13 verified; R5-CI-F12 source confirmed (S-2) |
| 49 | Root `Cargo.toml`: resolver 3, edition 2024, rust-version = pin, `[workspace.dependencies]` | r5 REC1 1153–1158 | confirmed (R and source) | r5:1154–1156 verbatim | F1, F2 verified; F6 source confirmed (S-2); F3 partly (S-11: resolver page confirms resolver 3's rule; its "must be set explicitly" wording not found) |
| 50 | `Cargo.lock` committed; `--locked` everywhere | r5 CI-REC2 | confirmed (R and source) | r5:1526–1527 "Cargo.lock is committed, and every CI cargo command takes `--locked`" | R5-CI-F11 source confirmed (S-3) |
| 51 | npm workspaces, one root lockfile, exact versions; CI's npm cache keys on it | r5 REC1; XQ-OQ2 1895–1898; CI-REC4 | confirmed (R), answer is this record's | r5:1157–1158 "one `package-lock.json` and exact versions"; r5:1895–1898 XQ-OQ2 (decider: the phase 1 restructure plan) | the ADR answers XQ-OQ2 itself; R5 names the restructure plan as decider (note N-4). F16 (setup-node) not verified: pending |
| 52 | Node pinned as legacy pins 22 | r5 1158; TL17 | confirmed | r5:1158 "Node is pinned the way the legacy toolkit pins 22 (TL17)"; TL17 "on Node 22" | |
| 53 | TypeScript 6.0.x exact; TS 7 its own change-set | r5 TSLINT-REC1 | confirmed | r5:1250, 1255 "Move to TypeScript 7 in its own change-set once typescript-eslint supports it (F5)" | the decisive facts F4, F5 verified |
| 54 | CLI at the lock's version via install-action by SHA, `fallback: none`, compared before generating | r5 CI-REC3 1536–1546 | confirmed (R) | r5:1538–1540 verbatim | rests on F14, F15 (verified) |
| 55 | CLI directly, not wasm-pack; wasm-pack's repos to be archived or handed over; its pages not read | r1 F1 886–893; r5 LAYOUT-OQ2 | confirmed | r1:887–889 "every other rustwasm repository (wasm-pack, gloo, twiggy, walrus and the rest) was to be archived in place or handed to its existing maintainers" | R5-LAYOUT-OQ2's decider is "after reading wasm-pack's repository" (r5:1754–1755), not done: the doc says so openly; note N-4 |
| 56 | Each toolchain bump its own change-set reporting new warnings | r5 RUSTLINT-REC6 | confirmed (R) | r5:1374–1376 "a toolchain upgrade is its own change-set. The change-set reports the new warnings" | F15 verified |
| 57 | Weekly latest-deps job, `continue-on-error` | r5 CI-REC2 1526–1527 | confirmed (R and source) | r5:1527–1528 "A weekly scheduled job tests the latest dependency versions without blocking (`continue-on-error`)" | R5-CI-F19 source confirmed (S-7) |
| 58 | Why: one file pins all; one compiler for native = Wasm; typed lint needs TS < 6.1 | r5 CI-REC2 reason; TSLINT-F4 | confirmed | r5:1528–1531 "R1-DET-REC6 compares native and Wasm results, which only means something when both come from one compiler version"; r5:194 | F12 part pending as row 48 |
| 59 | Part 2 need: via R1-DET-REC6, §6.6 and §6.8 | r5 CI-REC2 Serves | citation off | r5:1532 "Serves: §6.6 and §3 through R1-DET-REC6"; r1:2236–2237 R1-DET-REC6 serves "§6.6 … §3 (the scale test at 48)" | should read §6.6 and §3; finding F-12 (low) |
| 60 | Crate list `sim`, `content`, `protocol`, `sim-wasm`, `server`, `devtools`, roles and §6 items | r5 REC2 1170–1178 | confirmed (R) | r5:1171–1176 crates and their "serves" lists match the table (sim §6.1–§6.5; content §6.3, §6.7; protocol §6.1; sim-wasm §6.2; server §6.1, §6.5, §6.6, §6.8; devtools §6.6) | doc adds §6.9 to `sim-wasm` and §6.8 to `devtools`: small extensions of R5's lists (R1-BND-REC1 serves §6.9) |
| 61 | `room` crate added (I/O-free, builds for Wasm), enters with the foundation | SLICE-ORDER cond. 1; R1-BND-F19; R2-TRN-REC4 | confirmed (basis); rule 3 issue | r1:1020 "`std::thread::spawn` will panic"; r2:2685 "The simulation must not depend on any I/O"; STATUS:97–98 condition 1 | not an option of any research Options section; the doc says "no research compared them with other placements" but proposes it. Finding F-3 (rule 3) |
| 62 | `navbake` crate above `sim`, never in `sim-wasm`, enters slice 2 | r4 XT-OQ5 1195–1200; NAV-REC1; NAV-REC4 | confirmed (basis); rule 3 issue | r4:1196–1198 "Where the bake code lives decides whether the generator ships in the player's Wasm … Side B: missing answer … Decided by: ADR-0002" | as row 61: F-3 |
| 63 | Bake runs in CI, on map save and in the editor's Test; never in prediction | r4 NAV-REC1 807–808; NAV-REC4 888–889 | confirmed (R); source n/a | r4:807–808 "The bake runs in the editor's Test, on map save and in CI"; r4:888–889 "Client prediction never runs it" | R4 recommendation text; the claim is R4's design, no web source behind these words |
| 64 | Each crate enters with the first slice that needs it | r5 1178; SLICE-ORDER | confirmed (proposed, labelled "no crate for later") | r5:1178 "No other crate or package until a §6 item asks for it" | entry slices are the doc's choice |
| 65 | `protocol` holds the Rust side of the thin interface with its loopback | ADR 0001 migration step 2 | confirmed (cross-draft) | 0001:1254–1255 "the protocol crate (message types, the codec, the thin transport interface with its loopback backend)" | |
| 66 | `protocol` builds for Wasm: no transport crate or async runtime | R1-BND-F19 | confirmed | r1:1020 (threads panic on the default Wasm target) | |
| 67 | Packages `client`, `editor`, `sim-worker`, `net`, `types`, `e2e` | r5 REC2 1176–1178 | confirmed (R) | r5:1176–1178 verbatim list | |
| 68 | `net` holds the browser's two backends, which R2-TRN-REC4 puts in TypeScript | r2 TRN-REC4 2673–2676 | confirmed | r2:2674 "Client (TypeScript): WebTransport and WebSocket backends." | research verified |
| 69 | Lobby and supervisor crate wait for ADR 0006 (OQ-0006-3) | r2 ROOM-OQ9 3104–3105; ADR 0006 | confirmed | r2:3104–3105 "Same binary as the supervisor or a separate service"; 0006:615–619 OQ-0006-3 options (a)/(b) | |
| 70 | `sim` → `content` only, no transport/async/clock; cargo-deny `deny` + `wrappers` and `cargo tree` | r5 REC2 1179–1180; F14; R2-TRN-REC4 m3 | confirmed | r5:1179–1180 verbatim; F14 verified (r5:2124 "`wrappers` … denies all transitive dependencies on it"); r2:2697 "checked with `cargo tree`" | |
| 71 | `room` builds for wasm32 with no async runtime, no transport crate | the doc's rule | confirmed (proposed, follows row 61) | — | covered by F-3 |
| 72 | `sim-wasm` never depends on `navbake` or `server` | the doc's rule; R4-XT-OQ5 | confirmed (proposed) | r4:1196 (the concern) | |
| 73 | `client` never reaches `editor`: ESLint rule and manifest walk | r5 REC2 1181–1182; F15 | confirmed (R and source) | r5:1181–1182 verbatim; F15 verified (manifest fields) | TSLINT-F14 source confirmed (S-1) |
| 74 | `sim-worker` and `net` never import rendering or DOM | r5 TSLINT-REC2 1268–1269 | confirmed (R and source) | r5:1268–1269 "`sim-worker` and `net` ↛ rendering and DOM packages" | TSLINT-F14 source confirmed (S-1) |
| 75 | One feature set for `sim`; `cargo tree -e features` compared | r5 REC4 1206–1218 | confirmed | r5:1207–1209, 1216–1218 | rests on F4 (verified) |
| 76 | The legacy fence leaked (CS11) | r1 1176–1178 | confirmed | r1:1176 "The comment fence leaked" | |
| 77 | R5-LAYOUT-REC2 predates SLICE-ORDER and puts rooms in `server` | r5 1175; SLICE-ORDER | confirmed | r5:1175 "`server` (the binary with transport and rooms" ; R5 dated 2026-10-28/29, SLICE-ORDER 2026-10-31 (STATUS:94) | |
| 78 | Cache question: no ECS crate, parry3d not Rapier (ADR 0004 part 1); quinn and the proof test's WebTransport crate (ADR 0001 part 10) | ADR 0004, 0001 | citation off (cross-draft) | 0004:131 part 1 "No ECS crate"; parry3d and Rapier are 0004 part 10 (0004:295–297, Rapier kept as a measured fallback); 0001:308 part 10 "quinn 0.11.x; the WebTransport layer is picked by the netcode proof test" | should cite ADR 0004 parts 1 and 10; finding F-11 (low) |
| 79 | Cache measured against R5-CI-REC1's 8 GB mark in the foundation | r5 CI-REC1 1518–1520 | confirmed (R; R5's proposed mark) | r5:1519–1520 "the repository's total cache stays at or below 8 GB … leaving room under 10 GB" | |
| 80 | Part 3 need per crate; one feature set: §4, §6.6, §6.8 | r5 1171–1176, 1188, 1213–1214 | confirmed | r5:1213–1215 "Serves: §4 … §6.6 and §6.8 (re-runnable matches)" | |
| 81 | Overflow checks on for `sim`, `protocol` in every profile; explicit wrapping/checked ops | r5 RUSTLINT-REC4; R1-RR-OQ4 | confirmed | r5:1353–1356 verbatim; r1:2941–2945 "Decides it: ADR-0002 (build profiles) with R5" | per-package override is "inferred" in F13 (r5:378–379), verified list supports it |
| 82 | Dev and release differ on overflow by default | r5 F13 374–379 | confirmed | r5:376–377 "`overflow-checks = true` under `[profile.dev]`, `false` under `[profile.release]`" | research verified |
| 83 | Server keeps `panic = "unwind"`, connection work isolated, decoders never panic | r2 ROOM-REC2 2762–2768 | confirmed | r2:2765–2767 "So the build keeps the default `panic = "unwind"` (F8), each connection's work is isolated from the room's tick, and decoders never panic" | research verified |
| 84 | Wasm keeps `-Cpanic=abort`; trap ends the instance; restart is ADR 0004 part 20 with ADR 0001's resync | r1 F19; ADR 0004 p20 | confirmed | r1:1022–1023; 0004:466–467 "the Worker's host starts a fresh instance and rejoins through ADR 0001's resync path" | |
| 85 | Baseline x86-64, never `+fma`, `target-cpu=native`, `relaxed-simd`; the rule is ADR 0004's part 9 | ADR 0004 p9; r5 1328 | citation off (cross-draft) | the rule is ADR 0004 part 7 (0004:237–238 "the server therefore builds for baseline x86-64, never with `+fma` or `-C target-cpu=native` … (the build flags are ADR 0002's)"; `relaxed-simd` at 0004:234); part 9 has no flag rule; r5:1327–1328 "The `relaxed-simd` target feature … the gate reads the sim's build flags" | finding F-10 (low) |
| 86 | `unsafe_code` denied; exception needs invariant, test, `#[allow]` with reason | r5 RUSTLINT-REC1 1307–1308 | confirmed (R) | r5:1307–1308 verbatim | |
| 87 | Wasm size budget has no research number; spikes measure raw, gzip, wasm-opt, "s"/"z", parry3d | r1 RR-OQ5 2946–2950 | confirmed | r1:2946 "R1 records the size but sets no limit"; r1:2949 "measure wasm-opt, `opt-level` "s" against "z" and the panic strategy" | R1-RR-OQ5's "the panic strategy" measurement is not in the doc's list (omission, nit) |
| 88 | Budget file takes the measured value as first limit | r5 GATE-REC4 | confirmed (R) | r5:1737–1738 "Starting values come from the first spike's measurements (R1-RR-OQ5), not from guesses." | |
| 89 | Part 4 need: §4 (REC4), §6.6, §6.8; panic §6.1 | r5 1360–1361; r2 2772 | confirmed | r5:1360–1361; r2:2771 "Serves VISION §6.1, §4" | |
| 90 | One module Worker, single-threaded, stable Rust (A2+E1); page makes zero Wasm calls | r1 BND-REC1 | confirmed | r1:2252–2254, 2262 | research verified |
| 91 | One flat versioned block, preallocated, copied once into a pooled buffer, transferred (B2); not B4 | r1 BND-REC2 | confirmed | r1:2265–2267 | research verified |
| 92 | Header and sections; event section | r1 2266–2268 | confirmed | r1:2267–2269 "header (tick, layout hash, counts per entity kind), aligned sections per entity kind (soldiers, vehicle parts, projectiles, structures) and an event section" | |
| 93 | Added sections: pickups (ADR 0004 part 18) and owner-only data (I-21) | UPGRADE-SCOPE; map I-21; ADR 0004 p18 | confirmed (settled; cross-draft) | 0004:426–432 part 18 (pickups as sim entities, the PlayerState record); map I-21 "sent to their owner" | |
| 94 | Event contents (… pick results, drops, moments): ADR 0004 part 19 emits them | ADR 0004 p19; ADR 0001 p6 | confirmed (cross-draft), nit | 0004:448–451 "hits, deaths and kills …, blasts …, cosmetic shot events, purchases, uplink cache events, … upgrade picks and pickups" | "drops" and "captures" are not ADR 0004's words ("pickups", "uplink cache events"); consistency pass |
| 95 | Remote entities at the interpolation delay from the Worker's buffer; local player predicted | ADR 0001 parts 4, 6, 7 | confirmed (cross-draft) | 0001:253–254 "snapshot buffer kept in the Worker: the Worker publishes already-delayed remote poses in each per-tick block" | |
| 96 | Page binds views, keeps two newest blocks, returns older ones | r1 2269–2270; TICK-REC5 | confirmed | r1:2270–2271 "the page binds typed-array views (no copy) and keeps the latest two blocks for interpolation, returning older ones to the pool" | |
| 97 | Per entity or per field measured in the spike; ADR 0003 reads typed arrays either way | r1 BND-OQ6; ADR 0003 p10 | confirmed | r1:2737–2740; 0003:1181 "the renderer reads typed arrays whatever the layout" | |
| 98 | Block writer serializes field by field as the state hash does; no padding, no `unsafe` | r1 RR-OQ6 2951–2954 | confirmed (basis); answer outside the OQ's options | r1:2952–2954 "a state hash must read fields … (R1-DET-REC6 already serializes field by field), and writing R1-BND-REC2's block safely needs a derive that rejects padding, or `unsafe` code with a written invariant and a test. Decides it: rust-reviewer's crate pick in the synthesis" | a third route R1-RR-OQ6 does not list, not labelled proposed; Appendix marks the OQ "answered". Finding F-8 (low) |
| 99 | R1-RR-OQ6's two alternatives | r1 2951–2954 | confirmed | as row 98 | |
| 100 | B3 is the measured alternative | r1 2271 | confirmed | r1:2271 "B3 (SharedArrayBuffer ring) is the spike's measured alternative" | |
| 101 | One command per tick (C1): seq, move, bitmask, edge counters; fields ADR 0001's | r1 BND-REC3 | confirmed | r1:2286–2289 | R1 also lists "aim (yaw and pitch)" in the command; the doc replaces aim with the cursor ray (row 102, Appendix "changed") |
| 102 | Command also carries the cursor ray and view time; Worker casts the ray (I-19) | ADR 0001 p5; map I-19 | confirmed (cross-draft) | 0001:202–204 "cursor ray from the camera and the view time it drew (how they cross the boundary is ADR 0002's); the Worker casts …" | |
| 103 | Worker stamps the tick it applies the command to | r1 2288–2289 | confirmed | r1:2289 "the worker stamps the tick it applies it to" | |
| 104 | Control API is wasm-bindgen's own with its `.d.ts` (D1), constant calls per tick | r1 D1 1766; REC4 | confirmed | r1:1766 "Calls only, a constant number per tick" | |
| 105 | Types from three sources into `packages/types`; ts-rs via `TS_RS_EXPORT_DIR` on `cargo test`; not tsify | r1 BND-REC4; r5 LAYOUT-REC3 | confirmed | r1:2297–2301; r5:1194–1198 "ts-rs exports into `packages/types/src/generated/` through `TS_RS_EXPORT_DIR` in `.cargo/config.toml`" | F7, F9 verified |
| 106 | Content's types from the source ADR 0005 picks (rank 31) | r5 XT-OQ9; map I-5 | confirmed (cross-draft) | r5:2012–2015 "Decided by: ADR-0005 (content data), with ADR-0002" | |
| 107 | Isolation off (G1); flag adds COOP/COEP only to profiling builds; Mustafa's preview built with it off | r1 BND-REC5; GLOSSARY 636–651 | confirmed | r1:2309–2310; r5:1906–1909 "the preview Mustafa checks is built with the flag off" | |
| 108 | Production isolates if B3 passes mark 5, or timer pacing fails and W3 passes | r1 REC5; XU-OQ4 | confirmed (B3 trigger); the W3 trigger is the doc's reading (proposed) | r1:2310–2311 "Adopt it in production only if the spike's B2-against-B3 test passes REC2's threshold"; r1:2864–2865 "What decides it: R1-TICK-REC5's jitter mark and R1-BND-REC2 mark 5, read together in the synthesis" | the second trigger changes R1-BND-REC5 (Appendix says "changed"); fine |
| 109 | Worker owns the connection (H2); page sees only the block | r1 BND-REC6 | confirmed | r1:2326–2328 | |
| 110 | Datagrams in and out in one call each, so calls per tick stay constant | R1-BND-REC1 m2 | confirmed (proposed); see F-2 | r1:2262–2263 "the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)" | with the command, step and block pointer this makes 5–6 calls: finding F-2 |
| 111 | Why (part 5) | r1 REC1–3 reasons; D4 | confirmed | r1:2253–2254, 2273–2274, 2290–2291; r1:1769 "The only option that covers the bulk buffer" | "the only thing tsify and ts-rs cannot type": R1 says D4 is the only option covering it (same meaning) |
| 112 | Part 5 needs | r1 2256–2258, 2275, 2302–2303 | confirmed | as cited | |
| 113 | Part 6: one build, two modes, same page code; local server = `room` + `sim`, the authority, bots | r1 REC6; SLICE-ORDER; ADR 0001 p10 | confirmed | r1:2328–2330 "offline, the same worker build runs the match itself, with bots filling the slots. The page code is the same in both."; 0001:441 | |
| 114 | Local server and client share one instance (proposed start; OQ-0002-3) | ADR 0001 migration 2 | confirmed (proposed, labelled in OQ-0002-3) | 0001:1254 | |
| 115 | Worker host paces, steps, posts; commands pass outside Wasm (OQ-0004-7) | ADR 0004 p5, OQ-0004-7 | confirmed (cross-draft) | 0004:187–189 (W2 pacing); 0004:792 OQ-0004-7 | |
| 116 | Offline play as a shipped feature is a design question (S11) | r1 BND-OQ1 | confirmed | r1:2721–2723 "Is offline (solo against bots) play a v2 need? … Decides: Mustafa" | |
| 117 | Part 6 need: §3 and §4 as REC6 traces them | r1 2330–2332 | confirmed | r1:2331–2332 "VISION_V2 §3 ("Bots fill empty slots"), §4 (… prediction is not delayed by a busy frame)" | |
| 118 | Base tsconfig flags; project references; `tsc -b` beside every Vite build (Vite does not type-check) | r5 TSLINT-REC1; LAYOUT-REC5 1223 | confirmed (R); source F10 checked below | r5:1250–1254 flags verbatim; r5:1223 "`tsc -b` runs beside every Vite build"; r5:1226 "does not type-check (F10)" | sources confirmed: LAYOUT-F10 (S-6), TSLINT-F2 (S-17), F6–F12 (S-18; F12's construct list unverified, see TE98); TSLINT-F3, F24, F25 pending (time) |
| 119 | ESLint 10 flat config, strict-type-checked + stylistic, projectService, `--max-warnings 0`, four project rules | r5 TSLINT-REC2 1265–1282 | confirmed (R); source partly checked | r5:1265–1271 verbatim (incl. `considerDefaultExhaustiveForUnions: false`, TL6's list, `no-undef` off) | F4, F13, F15, F16 verified; sources confirmed: F14 (S-1), F17 (S-8), F18 (S-25), F19 (S-26); F20 pending (time) |
| 120 | Prettier on `packages/**` and root configs, not `docs/`/`legacy/`, check mode | r5 TSLINT-REC3 | confirmed (R) | r5:1283–1286 | F21, F22 verified |
| 121 | `textContent` rule by review until the baseline names its lint (ADR 0006 p13 B6) | r5 TSLINT-OQ5 1776–1778; ADR 0006 | confirmed | r5:1776–1778 "No rule for it was read. Decides it: security-auditor with the security baseline"; 0006:369 B6 "holds by review" | |
| 122 | Workspace lint table, opt-in a gate check; `unsafe_code` deny; clippy `all` warn; `-D warnings` host and wasm32; pedantic on sim, protocol, content | r5 RUSTLINT-REC1 | confirmed (R) | r5:1306–1312; mark r5:1319 "a crate whose `Cargo.toml` lacks `[lints] workspace = true` fails a gate check of its own" | F1, F2 verified; sources confirmed: RUSTLINT-F3 (S-27), F4 (S-19), F6 (S-20) |
| 123 | Hash-map, clock and float bans in `crates/sim/clippy.toml`, deny, backstop, flops lints allowed, no `--fix`; same for content's sim parts | r5 RUSTLINT-REC2; GLOSSARY 729–742 | confirmed (R) | r5:1321–1327 verbatim incl. "(and for the parts of `content` the sim calls)" | F7, F8, F9 verified; sources confirmed: F5 (S-20), F10 (S-28) |
| 124 | Only escape: reviewed `#[allow]` for a lookup-only map outside the serialized state (resolution 29) | r5 RUSTLINT-OQ5; GLOSSARY | confirmed (settled) | r5:1796–1798 "an `#[allow]` with a written reason may admit a map that is only looked up and lies outside the serialized sim state" | |
| 125 | Dependency map types join `disallowed-types` from rust-reviewer's `cargo tree` list | r5 RUSTLINT-OQ3 | confirmed (R) | r5:1790–1792 "rust-reviewer lists the map types of the sim's dependency tree and adds them to `disallowed-types`" | |
| 126 | Fallback text scan if clippy cannot name float methods | r5 RUSTLINT-OQ1 | confirmed (R) | r5:1785–1787 "a reviewed text or syntax scan of the sim crate for those method names" | |
| 127 | `unwrap_used`, `expect_used`, `indexing_slicing` deny in decoders and loader, allowed in tests; not in `sim` | r5 RUSTLINT-REC3 | confirmed (R) | r5:1340–1345 | F1, F11 verified; F12 source confirmed (S-21) |
| 128 | rustfmt stable options (`edition`, `style_edition` 2024, `newline_style = "Unix"`), check mode | r5 RUSTLINT-REC5 | confirmed | r5:1365–1367; F14 verified | |
| 129 | `panic`, `arithmetic_side_effects` not added: sources not read | r5 RUSTLINT-OQ2 | confirmed | r5:1788–1789 "Their source files were not at the paths tried (HTTP 404)" | |
| 130 | Per-edit hook (`tsc -b` touched project, ESLint file, `cargo check -p`), exit 2; kit change-set | r5 TSLINT-REC4 | confirmed (R) | r5:1292–1295 | kit change-set: plan rule 11/C13 (nothing in `.claude/` changes) |
| 131 | Part 7 why: only correctness denies; type ban catches what `iter_over_hash_type` misses | r5 F1, F9; REC2 1329–1330 | confirmed | r5:1329–1330 "`iter_over_hash_type` alone misses iterator chains (F9), so the type ban is the main check" | |
| 132 | Part 7 needs | r5 1334–1335, 1350, 1277 | confirmed | as cited | |
| 133 | Test layers table | r5 TEST-REC1 1386–1411 | confirmed (R), one nit | r5:1387–1400 match row by row | soak row "up to the largest mode's N (48)" while the test-mode list (used by part 8 and PM-0002-76) includes the N = 64 headroom run: note N-5 |
| 134 | Benchmarks never pass or fail the gate | r5 1400; R3-MEASURE-REC3 | confirmed | r5:1400 "never a gate pass or fail (R3-MEASURE-REC3: CI gates counts, not timings)" | |
| 135 | Determinism tools: `devtools` runner built as the server builds the sim; twice in two processes and Wasm in Node; Playwright replay page in three engines; first mismatch and field diff | r5 TEST-REC2 1412–1430 | confirmed (R) | r5:1412–1418, 1420 | F5, F6, F8 verified |
| 136 | Gate: three short recordings; nightly: a 10-minute bot match at N = 48 and every later recording | r5 1419–1421; MODES-3 | confirmed | r5:1419–1421 "three short recordings (… two teams, three teams, bots only); nightly: R1-ECS-REC2 mark 1's 10-minute bot match and every recording added since"; r5:1428–1429 at the largest mode's N | |
| 137 | Goldens: capture, fixture contents, the rows kept, not random-AI or GW14; insta never rewritten | r5 TEST-REC3 1431–1453 | confirmed (R) | r5:1431–1443 verbatim | F3 verified |
| 138 | L1 seeded, simulated time, per direction half RTT (proposed by R5), impairments, fate log; L2 under quinn, TCP proxy; profiles by name | r5 TEST-REC4 | confirmed (R) | r5:1454–1459 | F12, F13 verified; F4 confirmed (S-4); F11 (turmoil, github.com) unverified: HTTP 403 |
| 139 | Soak and load: Rust protocol bots, every test mode over L1 at P2, 30 min, one L2 run with 4 browsers; measurements; 3-hour pre-playtest soak | r5 TEST-REC5; GLOSSARY 375–394 | confirmed (R), widened | r5:1475–1484; GLOSSARY:376–377 "the one list of modes the spikes run … it replaces the topics' separate lists" | R5 runs items 1 and 4 plus scale steps; the doc uses the whole list, as the GLOSSARY says. F14 verified |
| 140 | Flakes: retries 0, `failOnFlakyTests`, `forbidOnly`, fixed seed, fresh seeds nightly, simulated time, test hooks, Playwright clock, no shared state, artefacts and issue | r5 TEST-REC6 | confirmed (R) | r5:1490–1498 | F2, F9 verified; sources confirmed: TEST-F1 (S-22), F7 (S-23), F10 (S-10) |
| 141 | Bench: replay-runner mode; stress scene; monotonic clock outside the sim; mean, p50, p95, p99, max over 3 runs; one vCPU of the reference server and Wasm in the Worker on ML | r1 TICK-REC2 2044–2057 | confirmed (method R1's; tool is the doc's, labelled "none was read") | r1:2047–2051 "36,000 ticks per run, 3 runs … Report mean, p50, p95, p99 and max per N"; r5:1818–1820 "until then … benchmarks reported with method, never gated" | |
| 142 | No bench crate, coverage tool or nextest: none was read | r5 TEST-OQ8 | confirmed | r5:1816–1818 "cargo-nextest, code coverage and the bench harness were not researched" | |
| 143 | Part 8 needs | r5 1406, 1425–1426, 1486 | confirmed | as cited | |
| 144 | One Node gate: base ref, exit 0/1/2, `--json`, `--allow-removed`, `--quick` (never for a commit), `--prove` | r5 GATE-REC2 1683–1705 | confirmed (R; legacy basis verified in repo by R5's verifier) | r5:1684–1690 verbatim | |
| 145 | Checks G0–G12 | r5 GATE-REC1 1655–1682 | confirmed (R), with the doc's stated changes | r5:1658–1670; doc's additions in G2 (opt-in, build flags), G12 (network rule) are listed as "changed" in Appendix A | |
| 146 | G13 maps check added | R4-XT-OQ5; R4-NAV-REC1; ADR 0004 | confirmed (basis); proposed | r4:1198 "The gate does not include R4's checks … Decided by: ADR-0002 (crate list and gate checks)" | labelled "Changed from R5-GATE-REC1" with reason |
| 147 | G2 also checks the lints opt-in and build flags | r5 RUSTLINT-REC1 mark; ADR 0004 | confirmed | r5:1319; build-flag rule: ADR 0004 part 7 (see row 85) | |
| 148 | G12 fails every outbound request except the local test server | TL9; r5 1723 | confirmed | TL9 mark "the network rule flips to allow the local game server's WebTransport and WebSocket"; r5:1723 mutation "an outbound request to an external host" | |
| 149 | The gate v2 has 14 checks | arithmetic | confirmed | G0–G13 = 14 | |
| 150 | `--prove`: committed suite, throwaway worktree, exact checks, one mutation and one control per id, on gate changes and nightly; cargo-mutants report only | r5 GATE-REC3 | confirmed (R) | r5:1706–1710, 1724–1728 | GATE-F1 verified |
| 151 | G13's mutation and control | the doc | confirmed (proposed) | — | |
| 152 | Budgets as data; raising reviewed; first spikes' values | r5 GATE-REC4 | confirmed (R) | r5:1734–1738 | F2 verified |
| 153 | CI G1: one workflow, five parallel jobs, one required final job (`if: always()` inferred), skip = failure, id comparison; no path filters | r5 CI-REC5; GATE-REC2 1689–1690 | confirmed (R) | r5:1556–1564, 1575–1577; r5:1688–1690 | F17, F20 verified; sources confirmed: CI-F21 (S-24), F23 (S-33), F24 (S-13) |
| 154 | Deliberate skip only after GATE-OQ1 (first month's CI times) | r5 GATE-OQ1 | confirmed | r5:1867–1868 "Decides it: the first month's CI times … until then every check runs" | |
| 155 | Preview deploy and Safari run not among required jobs | r5 1564–1566 | confirmed | r5:1564–1566 | |
| 156 | Caching: rust-cache v2.9.3 by SHA, key per kind, save only from main; no sccache; setup-node `cache: npm`, `npm ci --ignore-scripts`; browsers not cached; no auto caching with secrets | r5 CI-REC1, CI-REC4 | confirmed (R); sources partly checked | r5:1510–1512, 1547–1550 | F1, F5, F9, F10, F18 verified; sources confirmed: CI-F4 (S-9), F6 (S-9), F16 (S-5) |
| 157 | 10 GB per repository | r5 CI-F5 | confirmed | verifier r5:2148 "By default, the limit is 10 GB per repository" | research verified |
| 158 | sccache cannot cache cdylib, bin, proc-macro, incremental | r5 CI-F9 | confirmed | r5:2149 "This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates." | research verified |
| 159 | Hygiene; Dependabot (SHA pins tried on one action); lockfiles wait for GA on Mustafa's plan | r5 CI-REC5; CI-OQ3, OQ6 | confirmed (R) | r5:1559–1560, 1567–1569; r5:1830–1832; r5:1838–1839 "Decided by a trial PR on one action" | F22 verified; F24 fetched (F13) |
| 160 | Scheduled set incl. advisories (also on `Cargo.lock` changes); blocking policy is the baseline's | r5 GATE-REC1 1671–1674; BASELINE-WHEN | confirmed | r5:1671–1674; 0006:364 B1 | GATE-F3 source confirmed (S-29, S-34) |
| 161 | Standard runners: no GPU; headless Chromium on SwiftShader | r5 CI-F17; r3 MEASURE-F8 | confirmed | r5:2155 "GPU only under larger runners"; r3:648–650 "SwiftShader Device (Subzero)" | research verified |
| 162 | CI gates counts on whatever backend; prints it; extra `forceWebGL` run | r3 MEASURE-REC3; WGPU-F5 | confirmed (R); the extra run proposed | r3:1203–1205; r3:92 `forceWebGL` "for testing purposes" | |
| 163 | No GPU runner: a paid larger runner | r3 MEASURE-F9 | confirmed (R); source: see fetch F15 | r3:655–656 "GitHub offers "GPU-powered runners" among its paid larger runners" | |
| 164 | No arm64 leg: first fleet x86-64 (ADR 0006 p5); AArch64 before Arm hosts (ADR 0004) | r5 CI-OQ7; ADR 0006, 0004 | confirmed | r5:1840–1841; 0006:184 "The server CPU is x86-64"; 0004:239–240 | |
| 165 | Test builds carry no crash endpoint; Sentry trap check optional in the spike | r5 XT-OQ12; R6-OBS-REC3 | confirmed (proposed answer) | r5:2026–2029 "Decided by: ADR-0002 (CI jobs, secrets and the smoke test's network rule)" | |
| 166 | Sentry upload job only on deployed builds, own environment, no PR checkout; scope ADR 0006 B9 | r6 OBS-REC3; r5 PREV-REC1; ADR 0006 | confirmed (proposed; cross-draft) | r6:755 "debug files uploaded by CI and never shipped"; 0006:372 B9 | |
| 167 | Part 9 why | r5 CI-REC5, CI-REC1 | confirmed | r5:1569–1576; r5:1515–1516 "Saving only from `main` also keeps fork-written caches out" | CI-F6 basis: fetch F09 |
| 168 | Part 9 needs (G11 §4 fairness r5 1679–1680; G13 §6.2 and EDITOR_V2 §3) | r5; EDITOR_V2:39 | confirmed | r5:1679–1680; EDITOR_V2:39 "Validation panel: bot reachability (every spawn to every goal), the 6 m gap rule" | |
| 169 | Preview shape P1 incl. permissions, caching off, URL, one comment; forks no preview; no `pull_request_target`/`workflow_run` | r5 PREV-REC1 | confirmed (R) | r5:1588–1594 verbatim | F1–F4, F6 verified; sources confirmed: PREV-F5 (S-33), F7 (S-30), F9 (S-16, its README half), F3's secure-use quotes (S-24); F8 pending (time) |
| 170 | Artifacts kept 7 days, ADR 0006's retention | r5 PREV-REC3; ADR 0006 p3 | confirmed | r5:1619 "Build artifacts keep 7 days (`retention-days`), not the default 90"; 0006:157 | |
| 171 | Generated `_headers`; production and preview sets; COOP/COEP only with flag; deploy job checks HTML and `.wasm` | r5 PREV-REC4; GLOSSARY 652–665 | confirmed | r5:1625–1634 | F12, F18, F20 verified |
| 172 | Close job and weekly sweep here; policy and PREV-DELETE ADR 0006's | r5 PREV-REC3; ADR 0006 p3 | confirmed | r5:1616–1619; 0006:155–157 | |
| 173 | Multiplayer previews: binary built by CI, delivery job, cert hash written into the preview's files | r5 XT-OQ6; r6 HOST-REC3; ADR 0001 p10 | confirmed (proposed, labelled "no research compared it") | r5:1996–2001 "Decided by: ADR-0002 (preview workflow) and ADR-0006 …" | rule 3: F-3 |
| 174 | Delivery token scope and PR-built code on staging wait for the baseline (B9) | ADR 0006 B9; r5 XT-OQ6 | confirmed | 0006:372; r5:1997–1998 "a second CI secret and PR-built native code on a VM" | |
| 175 | Part 10 needs | r5 1598–1599; r6 680 | confirmed | r5:1598–1599; r6:680 "Serves: §6.1 (rooms, tested per PR)" | |
| 176 | Editor: own package and entry in the same Vite config; player build never loads editor code | r5 LAYOUT-REC5; EDITOR_V2:51; TL4 | confirmed | r5:1219–1220 "`client` and `editor` are separate HTML entries in one Vite config (or one config each)" | |
| 177 | Editor runs bake and map checks in a Worker from `navbake` + `sim` in Wasm | R4-NAV-REC1; EDITOR_V2 §3 | confirmed (basis); proposed placement | r4:807–808 (bake in the editor's Test) | |
| 178 | Research did not test a Vite page with Worker and Wasm from `file://` | r1 DATA-OQ11 | confirmed | r1:2818–2819 "whether the Wasm validator loads from `file://` was not researched" | |
| 179 | "if ADR-002 moves the project to modules" overtaken; text left to owners (DOCS-ALIGN) | EDITOR_V2:52; map J3; STATUS | confirmed | EDITOR_V2:52; plan step 11 adds it to DOCS-ALIGN (STATUS:146–149 does not list it yet) | |
| 180 | Part 11 needs | EDITOR_V2 §5; r5 1188 | confirmed | r5:1188 "the editor split serves §6.7" | |
| 181 | Boundary spike with the proof test, before the foundation | map part g, J6 | confirmed (cross-draft; proposal for SPIKE_PLAN) | doc says "`docs/SPIKE_PLAN.md` gives the spike its final name and place" | |
| 182 | Bundler other than Vite / other languages settled by GLOSSARY §8, the foundation | GLOSSARY §8 | confirmed (settled; C12 form) | map J1:1071 "the toolchain: a Cargo workspace plus TypeScript with Vite" | |
| 183 | Q1 options A–C | r5 997–1002 | confirmed (R) | r5:999–1001 cell by cell | |
| 184 | Q2 npm read (F8), in use (TL17), "(Legacy)" for npm 11; pnpm/Yarn not read, not proposable | r5 1004–1008 | confirmed; source checked (F12 fetch) | r5:1006–1007; npmjs page carries "Legacy" (fetch F12) | rule 3 respected |
| 185 | Q3 committed vs build-time | r5 1010–1014 | confirmed (R) | r5:1013–1014 | |
| 186 | Q4 the two crate-list options | r5 1170–1193; R1-BND-F19; R4-XT-OQ5 | confirmed (facts); rule 3 issue on the chosen row | the chosen row is not in any research Options section | F-3 |
| 187 | Q5 options A–C | r5 1019–1024 | confirmed (R) | r5:1021–1023 | |
| 188 | Q6 lint configs and formatters | r5 1026–1038 | confirmed (R) | r5:1028–1038; Biome con from r5:1287 "a second linter beside the typed one without type-aware rules on record (F23)" | |
| 189 | Q7 options A–C | r5 1040–1050 | confirmed (R) | r5:1044–1046 | |
| 190 | Q8 A1–A3 | r1 1733–1739 | confirmed | r1:1737–1739; A3 "R3 decides"; 0003:383 "OffscreenCanvas was not researched" | |
| 191 | Q9 B1–B4 | r1 1741–1752 | confirmed | r1:1745–1748 | |
| 192 | Q10 C1–C2 | r1 1754–1760 | confirmed | r1:1755–1758 | |
| 193 | Q11 D1–D4 | r1 1762–1769 | confirmed | r1:1766–1769 | |
| 194 | Q12 E1–E2; no §6 item asks for split ticks | r1 1771–1774 | confirmed | r1:1774 "No VISION_V2 §6 item asks the client to split one tick across cores." | |
| 195 | Q13 G1–G3: 100 µs / 5 µs timers, CORP, frames, popups | r1 1776–1782 | confirmed | r1:1777–1782 | F30 verified |
| 196 | Q14 H1–H2 | r1 1784–1788 | confirmed | r1:1785–1788 | |
| 197 | Q15 test-tool options | r5 1055–1076 | confirmed (R) | r5:1057–1076 | unread options labelled "not proposable" |
| 198 | Q16 cache, toolchain and CLI options; wasm-pack not proposable | r5 1081–1097; LAYOUT-OQ2; R1-BND-F1 | confirmed (R) | r5:1083–1097 | wasm-pack row is the doc's addition, labelled not proposable |
| 199 | Q17 G1–G3; free on public repos (F17) | r5 1099–1105 | confirmed | r5:1103–1105; r5:2155 "free and unlimited on public repositories." | |
| 200 | Q18 gate options and long-check options | r5 1133–1146 | confirmed (R) | r5:1137–1146 | |
| 201 | Q19 P1–P5 | r5 1110–1118 | confirmed (R) | r5:1114–1118 | |
| 202 | Client host Cloudflare Pages, Workers static assets alternative; advisor 2026-10-30 (resolution 19) | GLOSSARY 587–603 | confirmed (settled) | GLOSSARY:588–592 "Decided by the advisor, 2026-10-30 (resolution 19)" | |
| 203 | Legacy fence leaked because nothing checked it; every rule a gate check with a planted defect | r1 CS11; part 9 | confirmed | r1:1176–1178 | |
| 204 | B2's delivery waits for the message queue, "to be measured" | r1 1746 | confirmed; nit | r1:1746 "Delivery waits for the page's message queue (inferred, to be measured)" | it is R1's option table, not "R1-BND-REC2's option B2" text; R1 marks it inferred |
| 205 | Local-match Worker load unmeasured; ADR 0001 trade-off 5 counts it in the client's sim share | ADR 0001 | confirmed (cross-draft) | 0001:802–803 "the Worker also encodes and decodes every message for the loopback path; that cost is not measured yet and counts in [the client's sim share]" | |
| 206 | Strict configs not semver-stable; three.js flag cost unmeasured | r5 TSLINT-F13; OQ4 | confirmed | r5:2128 "not considered "stable" under Semantic Versioning (semver)."; r5:1773–1775 | |
| 207 | `--prove` rebuilds per mutation; R5 proposes ≤ 30 min | r5 1731–1732; GATE-OQ5 | confirmed | r5:1731–1732 "`--prove` itself takes under 30 minutes (it rebuilds per mutation; proposed)" | |
| 208 | TS-only work needs the Rust toolchain for G0–G2 | r5 1694–1698; GATE-OQ1 | confirmed | r5:1695–1697 "A and B both need the Rust toolchain (G0 checks it, and `--quick` runs G1–G2's cargo fmt and clippy)" | |
| 209 | Every check enforces one recommendation of R1, R4 or R5 | r5 1675–1677 | confirmed | r5:1676 "each check enforces one wave 1 or R5 recommendation, so no check is "for later"" | G13 → R4-NAV-REC1 |
| 210 | Gate machines need pinned Rust, Wasm target, Node, browsers; Mustafa's Windows included | r5 1694–1695; TSLINT-REC4 mark | confirmed | r5:1300 "on Mustafa's Windows machine and on Linux" | |
| 211 | Hard constraints change for v2 only after acceptance; nothing in CLAUDE.md/.claude changes | SPIKE-SCOPE; plan rule 11 | confirmed (settled) | STATUS:187–188; plan rule 11 | |
| 212 | Classic scripts → modules and crates; TL2, TL3, TL6, TL8 drop | inventory | confirmed (inventory) | marks at lines 387, 388, 391, 393 | |
| 213 | Double-click player build ends; HTTPS and localhost are R3-WGPU-REC4's secure contexts; WebTransport needs HTTPS | r3 WGPU-REC4 941–951; r2 F35; map J2 | confirmed | r3:942–943 "the game is served from a secure context (HTTPS, or localhost in development)"; r2:1269 | R3-WGPU-F20 source confirmed (S-31) |
| 214 | Layering rule → part 3 rules, G5, G8 | CLAUDE.md; TL4, TL7 | confirmed (inventory) | TL4, TL7 marks redesign | |
| 215 | three.js is ADR 0003's; bundled from npm; no CDN tag | map J2; ADR 0003/0006 | confirmed (cross-draft) | map J2 CDN-SRI "v2 bundles three.js from npm with Vite"; 0003:137 part 2 pin | |
| 216 | No external assets ends (LOOK_V2 §8); model files under 25 MiB each | LOOK_V2:65; R6-HOST-F3 | confirmed | LOOK_V2:65; r6:75–76 | |
| 217 | Other constraints are ADR 0003's, 0001's, 0006's and 0004's | other drafts | confirmed (cross-draft, by mention) | `KVStore` in 0001/0004/0005/0006; `LinearEncoding`, `stylizeScene` in 0003/0004/0005; "Animation is cosmetic" in 0001/0003/0004/0005 | the sentence's mapping is loose (ADR 0005 also handles three of them): consistency pass |
| 218 | Gate step would run gate v2 for v2 code, today's for `legacy/`: a kit change-set | CLAUDE.md protocol step 4 | confirmed (proposed follow-up) | — | |
| 219 | Inventory: no mark changes; none look-brief-pending; table marks and lines | inventory | confirmed (inventory) | all 26 rows' marks and lines checked (CS11:179; PG1/18/19:359/376/377; TL1–TL17:386–402; KT7:416, KT8:417, KT9:418, KT12:421, KT19:428) | |
| 220 | `file://` is a secure context on MDN's table (R1-DATA-OQ11's second pass) | r1 2819–2823 | confirmed | r1:2820–2821 "lists "`file:///path/to/resource.html` ✅ Secure (`file` URL)"" | research verified (second verifier pass) |
| 221 | OQ-0002-1 options; (c) not researched; decider spike then Mustafa | r1 DATA-OQ11; XT-OQ10 | confirmed | r1:3077–3078 "Decided by: ADR-0002 (editor build), or Mustafa if EDITOR_V2 §5's double-click rule must change" | (c) labelled not researched and not proposed: rule 3 OK |
| 222 | OQ-0002-2 options, decider, what decides it; KT21 | r5 GATE-OQ3; LAYOUT-OQ5; LAYOUT-REC6; KT21:430 | confirmed | r5:1871–1872; r5:1243–1245 | |
| 223 | OQ-0002-3 options; proposed start (a) | ADR 0001; R1-BND-REC1 m2; F19 | confirmed (proposed start labelled) | 0001:1254 (loopback in protocol crate) | the options are the doc's own (no research option); acceptable for an OQ |
| 224 | OQ-0002-4 gains and costs; login outside the page unless `same-origin-allow-popups` isolates | r1 REC5; OQ9; OQ5 | confirmed | r1:2316–2320; r1:2747–2752; r1:2734–2736 | |
| 225 | OQ-0002-5 options; nightly interim per R5-XQ-OQ1 | r5 XQ-OQ1; TEST-REC1; TEST-OQ4 | confirmed | r5:1892–1893 "until then the macOS run can be nightly without breaking either part" | (c) WebdriverIO e2e labelled not researched |
| 226 | OQ-0002-6 jump 3.07 m vs 2.98 m; decider game-designer with Mustafa | r5 TEST-OQ3 | confirmed | r5:1806–1808 | |
| 227 | OQ-0002-7: moved in 2025, broke compatibility twice in 2026 (0.2.123, 0.2.128); decider reads #4533 and governance page | r1 BND-OQ7; F1, F2 | confirmed | r1:2741–2744 | note: github.com pages returned HTTP 403 to curl in this session (proxy: repository access not enabled), so the named reading may need another route (see fetch audit F04/F05/F16) |
| 228 | Spike throwaway in `spikes/`, shares proof test's harness and stand-in sim | SPIKE-SCOPE; map part g | confirmed (settled; cross-draft) | STATUS:187–188 | |
| 229 | Spike item 1 marks | r1 REC1 2258–2264 | confirmed | r1:2259–2263 | research verified |
| 230 | Page's cost of the sim: ADR 0003 owns the row, stated as a p99 (LE-B) | r3 1127; map LE-B | confirmed | r3:1127 "sim cost seen by the page … p99 ≤ 1.0 ms"; 0003:972 PM-0003-20 | |
| 231 | ≤ 4 calls in one instance, ≤ 8 in two (proposed) | arithmetic | confirmed (proposed, labelled) | 4 × 2 = 8 | see F-2 on the online count |
| 232 | Brave added | r5 PREV-REC4 m2 | confirmed | r5:1639–1640 "in Chrome, Edge, Firefox, Safari and Brave (R1-BND-REC5's pass mark, plus Brave for Mustafa)" | |
| 233 | Spike item 2 marks | r1 REC2 2276–2285; MODES-3 | confirmed | r1:2276–2284 verbatim, N = 48 per MODES-3 (a) | |
| 234 | Bytes and copy time per layout; writer's time reported | r1 BND-OQ6; RR-OQ6 | confirmed (report proposed) | r1:2739–2740 | |
| 235 | Spike item 3 marks | r1 REC3 2292–2296 | confirmed | r1:2292–2295 ("offline" read as local play) | |
| 236 | Ray and view time in the same message (proposed) | part 5 | confirmed (proposed, labelled) | — | |
| 237 | Spike item 4 marks incl. `erasableSyntaxOnly`, two runs byte-identical | r1 REC4; r5 LAYOUT-REC3; TSLINT-OQ3; LAYOUT-OQ4 | confirmed (R) | r1:2304–2307; r5:1203–1204; r5:1770–1772; r5:1759–1760 | |
| 238 | Spike item 5 marks | r5 LAYOUT-REC5 1229–1231; r1 REC5 2321–2324 | confirmed | as cited | |
| 239 | Spike item 6 marks + OQ-0002-3 measures | r1 REC6 2334–2338 | confirmed | r1:2334–2337 | |
| 240 | Spike item 7: ADR 0004's S18 and R1-TICK-REC2 P2 here; W3 if pacing fails | ADR 0004 S18; r1 2054–2057 | confirmed (cross-draft) | 0004:840 S18 "the boundary spike (ADR 0002)"; 0004:824 S2 "the sim core spike's sim in the boundary spike's Worker" | |
| 241 | Spike item 8 measurements; no pass mark; under 25 MiB | r1 RR-OQ5; r5 CI-OQ4, TSLINT-OQ4, OQ6, TEST-OQ1 | confirmed | r5:1833–1834 "The wasm-opt (binaryen) pin. It is not on install-action's list"; r5:1779–1780 | |
| 242 | Spike item 9: editor from `file://` in five browsers | OQ-0002-1 | confirmed (proposed) | — | |
| 243 | Spike item 10: optional Sentry trap, 4 browsers | r6 OBS-REC3 765–768 | confirmed (R; proposed mark of R6) | r6:765–766 "arrives in Sentry with Rust function names in Chrome, Edge, Firefox and Safari (4 of 4)" | R6-OBS-F7 verified; F10 source confirmed (S-35); F6, F8 pending (time) |
| 244 | Mustafa runs the spike in Brave; test-engineer adds the checklist | plan rule 12 | confirmed (settled) | plan rule 12 | the checklist is present (doc 1136–1157) |
| 245 | Foundation is not a spike | map part g | confirmed (cross-draft) | — | |
| 246 | Migration phase 1 order | SLICE-ORDER cond. 3; map J6 | confirmed | STATUS:101 "The netcode proof test … still runs before the foundation." | |
| 247 | Phase 2.1 done-when | r5 LAYOUT-REC6 mark | confirmed (R; Brave step proposed) | r5:1243–1245 | |
| 248 | Phase 2.2 done-when | r5 LAYOUT-REC1 mark; CI-REC2 | confirmed (R) | r5:1166–1169; r5:1533–1534 | |
| 249 | Phase 2.3 done-when incl. 10/20 min "proposed by R5; VISION_V2 sets no CI target", 8 GB | r5 CI-REC5, CI-REC1, GATE-REC3 | confirmed (R) | r5:1580–1581 "(proposed budget; VISION_V2 sets no CI target; legacy is about 1 min)" | |
| 250 | Phase 2.5 done-when (ADR 0001, 0003, 0004 milestones) | other drafts | confirmed (cross-draft) | 0001:1257–1258 `Join`, `JoinAccepted`, `Input`, `Snapshot` over loopback with L1 | |
| 251 | Phase 2.6 done-when | r5 PREV-REC1 m1; PREV-REC4 m1 | confirmed (R) | r5:1600–1601; r5:1636–1637 | |
| 252 | Foundation done when empty scene, CI green, preview link | GLOSSARY §8 | confirmed | map J1:1077 | |
| 253 | Slices 1–6 milestones | SLICE-ORDER; r5 TEST-REC5 | confirmed (settled; R; proposed per slice) | r5:1487–1489 | |
| 254 | Test plan | r5 GATE-REC3, TEST-REC1–6 | confirmed (R) | as rows 133–150 | |
| 255 | Rollback: B3, W3, C2 behind the flag; page code unchanged | r1 REC2, REC5; XU-OQ4; ADR 0003 p10 | confirmed | 0003:1181 | |
| 256 | Rollback: second Worker not researched | — | confirmed (labelled) | — | |
| 257 | Workers static assets take the same `_headers` | GLOSSARY 587–603 | confirmed (settled) | GLOSSARY:588–589 "the alternative that takes the same `_headers` file" | |
| 258 | After acceptance a superseding ADR (template 0000) | template | confirmed (design doc) | docs/adr/0000-template.md Status line | |
| 259 | Appendix A: 104 ids in the map's order; 32/7/0/40/11/14 | map ids tsv | confirmed (arithmetic and script) | 104 rows; `diff` against the map's owner-0002 ids: identical, no duplicates; counts adopted 32, changed 7, answered 40, open 11, spike 14 (script) | |
| 260 | Intro: settled stack; VISION_V2's browsers, laptop, 48; the foundation | brief §1; VISION_V2; GLOSSARY §8 | confirmed | DRAFTS_BRIEF:23–26; VISION:52–53 | "modest laptop" paraphrases "a laptop with integrated graphics" |
| 261 | Intro summary | parts 1–10 | confirmed (as the parts' rows) | — | |
| A1 | (no row) Doc line 1171 "---8<--- the subsection ends on the line above ---" | — | wrong (assembly artifact) | doc:1171 is passmarks file line 244, a cut marker; doc:1172–1173 two blank lines | finding F-1 |
| A2 | (no row) Note under the table: "the drafter confirms the reading, or the design batches its inputs" | NOTE PM-0002-3 | open instruction left in the ADR | doc:1084–1088 | F-2 |
| A3 | (no row) "R5's working name for the gate v2 is today's path" (OQ-0002-2) | r5 GATE-REC2 | confirmed | r5:1684 "working name `node tools/gate.mjs`, v2 edition" | |
| A4 | (no row) "Its results go into this record before Mustafa accepts it" (spike section) | plan | confirmed (process; matches migration step 1) | doc:1179–1180 | |
| A5 | (no row) Editor: "a phase 1 track, slice 5; its map document is ADR 0005's" | brief §1; map I-9 | confirmed | DRAFTS_BRIEF:35 "Octopon-3 and Editor v2" (slice 5) | |
| A6 | (no row) Test table soak row "up to the largest mode's N (48)" vs the test-mode list's N = 64 headroom (PM-0002-76) | GLOSSARY 375–387 | inconsistent wording | GLOSSARY:385 item 5 "the headroom run, N = 64" | note N-5 |

## Test-engineer ledger rows (the "Pass marks" subsection, doc 938–1170)

Doc line = TE ledger's "draft" line for draft lines ≤ 937; the TE ledger's draft lines after 939 map to doc +233.
"conf." = confirmed; "prop." = a proposed reading, method or number the doc labels proposed with a reason.

| # | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|
| TE1 | ML: ADR 0003's minimum laptop, Windows 11, Chrome, Edge, Firefox | r3 BUDGET-REC1 | conf. (R3's proposal) | r3:1096–1099 "Windows 11, run on Chrome and Firefox" | Edge added from R1-BND-REC1 mark 3 |
| TE2 | ML panel 60 Hz unless named | — | conf. (prop.) | — | |
| TE3 | MAC: latest Safari; model recorded | r1 REC1 m3 | conf. | r1:2263 | |
| TE4 | MP: RTX 5080 in Brave; never a budget result | r3 1102–1103 | conf. | r3:1101–1102 "Mustafa's RTX 5080 is a headroom and Brave check, never the recommended reference" | |
| TE5 | CI: ubuntu-24.04, 4 CPUs, 16 GB, no GPU | r5 CI-F17 673–676 | conf. | verifier r5:2155 "ubuntu-24.04 public "4" \| "16 GB""; GPU only under larger runners | ledger tier "source to check" is wrong: F17 is research verified |
| TE6 | CI gives counts, never hardware times | r3 MEASURE-REC3 | conf. | r3:1203–1206 | |
| TE7 | BX uses the proof test's throwaway sim | ADR 0001 spike | conf. (cross-draft) | 0001 spike section | |
| TE8 | Stress-scene counts: 16v16v16, 15 big vehicles, 4 very large, 300 projectiles | GL 397–420 | conf. | GLOSSARY:400–404 | |
| TE9 | Very large units of 20 parts | GL 438–442 | conf. | GLOSSARY:439–440 "at least 20 parts" | doc's "of 20 parts" = the minimum |
| TE10 | Scale steps 12/24/36 in three equal teams | GL 400–402 | conf. (prop.) | GLOSSARY:401–402 names the steps only | |
| TE11 | Scripted movers fill unsimulated sections | — | conf. (prop.) | doc:958–959 reason given | |
| TE12 | Plain renderer, not the look | plan rules 6, 10 | conf. (prop.) | — | rule 6 respected |
| TE13 | BX-P padded to 13.3 ms | GL 214–218 | conf. | GLOSSARY:215–216 "13.3 ms at 30 Hz" | |
| TE14 | 600 s × 30 = 18,000 ticks | arithmetic | conf. | 600 × 30 = 18,000 | |
| TE15 | Online via the proof test's room and impairment | ADR 0001 | conf. (cross-draft) | — | |
| TE16 | P0 and P2 values | r2 1987–1990 | conf. | r2:1988 "P0 = 0 ms, no jitter, 0 % loss · … P2 = 100 ms, ±20 ms, 2 %" | |
| TE17 | H: 3 × 600 after 300 | r3 MEASURE-REC1 | conf. | r3:1183–1184 | |
| TE18 | `performance.now()`, not rAF (1 ms) | r3 MEASURE-F7 | conf. | r3:641–643 | verified |
| TE19 | Nearest rank 570th, 17,100th, 17,820th | arithmetic | conf. | ⌈0.95×600⌉ = 570; ⌈0.95×18,000⌉ = 17,100; ⌈0.99×18,000⌉ = 17,820 | |
| TE20 | V: medians within 3 % | r3 1192–1193 | conf. | r3:1192–1193 "two back-to-back runs … medians within 3 %" | |
| TE21 | Count mark statistic = run max | — | conf. (prop.) | — | |
| TE22 | No statistic = p99; one-off = plain limit | GL 141–147 | conf. | GLOSSARY:142, 146–147 | |
| TE23 | U and P modes | ADR 0003 | conf. (prop., as 0003) | 0003:927 | per-browser switch "not researched" stated |
| TE24 | 100 µs / 5 µs | r3 F7 | conf. | r3:641–642 | |
| TE25 | Sub-ms marks on the isolation-flag build | ADR 0003 | conf. (prop.) | — | |
| TE26 | Shared clock; worker own time origin | r1 REC2 m3; F30 | conf. | r1:2279–2280 | |
| TE27 | Counting build | — | conf. (prop.) | — | |
| TE28 | 30 Hz, no 60 Hz mark | TICK-30; map J1 | conf. | STATUS:118–120 | |
| TE29 | Command shim has no profile (posted messages reliable) | r1 REC3 | conf. (prop. reading) | r1:2290–2291 (posted messages are structured clones, F34) | reliability is the reader's inference |
| TE30 | "a check without a failing case can't be shown to work" | r5 1727–1728 | conf. | r5:1727–1728 verbatim | |
| TE31 | Rule B busy-wait | ADR 0003 | conf. (prop.) | — | |
| TE32 | Mutation fails exactly its check; control passes | r5 1706–1709 | conf. | r5:1707–1709 | |
| TE33 | ML named with CPU, GPU, RAM, screen; `hardwareConcurrency` | r1 REC1 m1; BND-OQ3 (2727–2730); r3 1114–1116 | conf.; citation off | R1-BND-OQ3 is r1:2728–2731 (doc PM-0002-1 cites 2727–2730) | finding F-13 (low) |
| TE34 | Displays and commits recorded | — | conf. (prop.) | — | |
| TE35 | 0 page calls, 0 instantiations | r1 REC1 m2 | conf. | r1:2262 | |
| TE36 | Bite: page instantiates `sim-wasm` | — | conf. (prop.) | — | |
| TE37 | Same calls at N = 12 and 48, ≤ 4 | r1 REC1 m2 | conf. | r1:2262–2263 | |
| TE38 | ≤ 8 for two instances | arithmetic | conf. | 4 × 2 | |
| TE39 | Glue allocator calls counted, ≤ 4 on control-API calls | — | conf. (prop. reading) | doc:1084–1088 | the reading narrows R1's "Wasm calls"; F-2 |
| TE40 | 1,800 ticks | arithmetic | conf. | 60 × 30 | |
| TE41 | Bite: per-soldier accessor | — | conf. (prop.) | — | |
| TE42 | p95 difference ≤ 1.0 ms at 12/24/36/48 | r1 REC1 m1 | conf. | r1:2259–2261 | |
| TE43 | Same at N = 48 on BX-P | — | conf. (prop.) | — | |
| TE44 | Both statistics gate (LE-B) | map LE-B; PM-0003-20 | conf. (cross-draft) | 0003:972 "**Which statistic gates (decision map LE-B): both, each in its own spike**" | |
| TE45 | Page's cost ≤ 1.0 ms per frame | GL 214–218; r3 1127 | conf. | GLOSSARY:217; r3:1127 | |
| TE46 | 10 interleaved pairs; median = mean of 5th and 6th | ADR 0003 PM-0003-37 | conf. (prop.) | 0003:989 "10 pairs … the median of the 10 paired differences" | |
| TE47 | 3 % of 28 ms = 0.84 ms | arithmetic | conf. | 0.03 × 28 = 0.84; r3:1124 median ≤ 28 ms | |
| TE48 | 1,200 blocks, 40 s | arithmetic | conf. | 40 × 30 = 1,200 | |
| TE49 | Mode U; rule B | — | conf. (prop.) | — | |
| TE50 | Four browsers and Brave | r1 REC1 m3 | conf. | r1:2263 | |
| TE51 | 0 errors, 18,000 ± 1 ticks, one block per tick | — | conf. (prop.) | — | |
| TE52 | Bite: `console.error` at 2 s | r5 1723 | conf. | r5:1723 | |
| TE53 | ≤ 1 copy, 0 page copies and per-entity calls | r1 REC2 m1 | conf. | r1:2276–2277 | |
| TE54 | Exactly 1 message per tick | part 5 | conf. (prop.) | — | |
| TE55 | Detached-buffer and view-identity checks | r1 F15 | conf. | r1:2290–2291; F15a verified (r1:3413) | |
| TE56 | Bite: no transfer list | — | conf. (prop.) | — | |
| TE57 | 0 buffer/typed-array allocations per frame | r1 REC2 m2 | conf. | r1:2277–2278 | |
| TE58 | Warm-up = first 300 frames | r3 1183–1184 | conf. (prop.) | — | |
| TE59 | < 0.1 MB/s as allocation rate | r1 REC2 m2 | conf. (number); prop. reading | r1:2278 "heap growth under 0.1 MB/s … (allocation sampling)" | |
| TE60 | 0.1 × 600 = 60 MB | arithmetic | conf. | | |
| TE61 | Sampling incl. collected objects (TL15) | tools/perfbots.js 9–10 | conf. (repo) | perfbots.js:9–10 "inspector sampling profile with includeObjectsCollectedByMajorGC/MinorGC" | |
| TE62 | Chrome only | — | conf. (prop.) | — | |
| TE63 | CDP for GC and heap | ADR 0003 PM-0003-24 | conf. (prop.; "not researched" there) | 0003:976 | |
| TE64 | 3 matches; 64-byte bite | PM-0003-29 | conf. (prop.) | 0003:981 | |
| TE65 | Latency ≤ one frame at p95 | r1 REC2 m3 | conf. | r1:2278–2280 | |
| TE66 | 17,100th | arithmetic | conf. | | |
| TE67 | Endpoints defined | — | conf. (prop.) | — | |
| TE68 | Page loaded to 16 ms/frame | r3 1125 | conf. | r3:1125 "Main-thread CPU per frame … p99 ≤ 16 ms" | |
| TE69 | Mode P, 3 matches | — | conf. (prop.) | — | |
| TE70 | Bite: late post | — | conf. (prop.) | — | |
| TE71 | 0 lost/reordered events at N = 48 | r1 REC2 m4 | conf. | r1:2280–2281 | |
| TE72 | Event sequence number | — | conf. (prop.) | — | |
| TE73 | Bite: block returned unread | — | conf. (prop.) | — | |
| TE74 | Bytes and copy time per layout | r1 BND-OQ6 (2736–2739) | conf.; citation off | OQ6 is r1:2737–2740 | F-13 |
| TE75 | Writer time beside the copy | r1 RR-OQ6 | conf. | r1:2951–2954 | |
| TE76 | Critic rank 37 decided here | map | conf. (settled) | — | |
| TE77 | B3 weighed only if ≥ 0.5 ms or one frame | r1 REC2 m5 | conf. | r1:2282–2284 | |
| TE78 | "Page time" = handler and binding | — | conf. (prop.) | — | |
| TE79 | B2 and B3 both on isolated build | r1 REC5; r3 F7 | conf. (prop.) | — | |
| TE80 | 10 pairs | — | conf. (prop.) | — | |
| TE81 | 0 lost/doubled over 10,000 | r1 REC3 m1 | conf. | r1:2292–2294 | |
| TE82 | Releases counted; short presses | r1 REC3 | conf. (prop.) | r1:2287–2288 edge counters | |
| TE83 | Shim U(0,100 ms), 10 % doubled | — | conf. (prop.) | — | |
| TE84 | Network press mark is ADR 0001's (R2-SYNC-REC2 at P3) | ADR 0001 | conf. (cross-draft) | 0001:1113, 1357 | |
| TE85 | Bite: bitmask only | — | conf. (prop.) | — | |
| TE86 | Key to tick ≤ tick + frame at p95 | r1 REC3 m2 | conf. | r1:2294–2295 | |
| TE87 | 33.3 + 16.7 = 50.0 ms | arithmetic | conf. | 1000/30 = 33.3; 1000/60 = 16.7 | |
| TE88 | 9,500th of 10,000 | arithmetic | conf. | | |
| TE89 | Playwright headed; Safari via safaridriver | r5 F8; TEST-REC2 | conf. | r5:1416–1418 | |
| TE90 | Bite: frame-callback posting + extra tick | — | conf. (prop.) | — | |
| TE91 | Ray and view time in the same message | part 5 | conf. (prop.) | — | |
| TE92 | Zero diff | r1 REC4 m1 | conf. | r1:2304–2305 | |
| TE93 | Rename and hand edit fail | r5 LAYOUT-REC3 | conf. (R5 prop.) | r5:1203–1204 | |
| TE94 | Control: rename with regeneration | r5 1716 | conf. | r5:1716 | |
| TE95 | Layout change fails at build or startup | r1 REC4 m2 | conf. | r1:2305–2306 | |
| TE96 | Test design | — | conf. (prop.) | — | |
| TE97 | `tsc --strict` passes | r1 REC4 m3 | conf. | r1:2306 | |
| TE98 | `erasableSyntaxOnly`; bite: a generated `enum` (the flag rejects one) | r5 TSLINT-OQ3 | conf. (R); source unverified for `enum` | r5:1770–1771 "If ts-rs or wasm-bindgen emit `enum` or `namespace`, the flag rejects them (F12)"; source (fetch G02): the tsconfig page says the flag errors "on most TypeScript-specific constructs that have runtime behavior" and shows only a parameter-property example; R5-TSLINT-F12 itself calls the construct list "general knowledge … (unverified)" | finding F-14 (low) |
| TE99 | CI fails on CLI/library mismatch | r1 REC4 m4; r5 CI-REC3 | conf. | r1:2306–2307 | |
| TE100 | CLI one patch off | r5 1716 | conf. | | |
| TE101 | Two runs byte-identical | r5 1203–1204 | conf. | | |
| TE102 | 20 runs | r5 LAYOUT-OQ4; TEST-REC6 | conf. (prop.) | r5:1759–1760; r5:1503 | |
| TE103 | `crossOriginIsolated` with/without flag, three engines, vite and vite preview | r5 LAYOUT-REC5 | conf. | r5:1229–1231 | |
| TE104 | 12 cases | arithmetic | conf. | 3 × 2 × 2 | |
| TE105 | Read in the Worker; COOP-only bite | — | conf. (prop.) | — | |
| TE106 | Conditional: four browsers on a preview; 0 COEP violations | r1 REC5 m | conf. | r1:2321–2324 | |
| TE107 | `same-origin-allow-popups` recorded | r1 BND-OQ9 (2750–2755) | conf.; citation off | OQ9 is r1:2747–2752 | F-13 |
| TE108 | Full match = 10-minute match; ReportingObserver | — | conf. (prop.; tool "not researched") | — | |
| TE109 | 0 network messages on the page | r1 REC6 m1 | conf. | r1:2334–2335 | |
| TE110 | Online P2; CI at P0; bite | — | conf. (prop.) | — | |
| TE111 | 100 ms busy loop, no gap | r1 REC6 m2 | conf. | r1:2335–2336 | |
| TE112 | "No gap" = ≤ two intervals | — | conf. (prop.) | — | |
| TE113 | 2 × 33.3 = 66.7 ms | arithmetic | conf. | | |
| TE114 | Gated at P0, reported at P2 | r2 1989 | conf. (prop.) | | |
| TE115 | Same Wasm and page code | r1 REC6 m3 | conf. | r1:2336–2337 | |
| TE116 | sha256 from network log | — | conf. (prop.) | — | |
| TE117 | OQ-0002-3 measures | doc OQ-0002-3 | conf. | doc:833–834 | |
| TE118 | (b) gated on trap survival | OQ-0002-3; OQ-0004-7 | conf. (prop.) | 0004:792 | |
| TE119 | S18 numbers | r1 TICK-REC5; ADR 0004 S18 | conf. | r1:2109–2114; 0004:840 | |
| TE120 | ≤ 7 ticks per frame | ADR 0004 p5 | conf. | 0004:184–185, 192 "⌊0.25 × 30⌋ = 7" | |
| TE121 | ADR 0004's numbers stand where they differ | map part c | conf. | | |
| TE122 | "More than 5 minutes" read as 6; R1-BND-OQ2 (2723–2726) | r1 BND-OQ2; XU-OQ12 | conf.; citation off | OQ2 is r1:2724–2727; source of the 5-minute rule checked (fetch F14: "The page has been hidden for more than 5 minutes."; "once per minute") | F-13 |
| TE123 | "Back in step" = predicted tick's lead within ±1 tick (unless ADR 0004 defines it) | — | superseded (cross-draft) | ADR 0004's PM-0004-70 now defines it: "within 0.5 s of the tab's return the client has applied the server's state and its commands are accepted again" (0004:1012) | finding F-7: two definitions of one mark |
| TE124 | 120/144 Hz displays attached to ML | — | conf. (prop.); differs from ADR 0004 | PM-0004-69 measures smoothness on "RD: Chrome and Firefox on the displays on hand" (0004:1011) | F-7 |
| TE125 | 17,820th; 3 hidden trials | arithmetic; prop. | conf.; differs from ADR 0004 | PM-0004-70: "5 hides per duration and browser" | F-7 |
| TE126 | Bites: no accumulator; simulating the gap | r1 TICK-REC5 | conf. (prop.) | r1:2095–2101 | |
| TE127 | W3 only if W2 fails | r1 TICK-REC5; XU-OQ4 | conf. | r1:2096–2097 | |
| TE128 | P2: 13.3 ms p99 in the Worker while 30+ fps on Low | r1 TICK-REC2 P2; ADR 0004 S2 | conf. | r1:2054–2057 | |
| TE129 | 40 % of 33.3 = 13.3 ms | arithmetic | conf. | 0.4 × 33.33 = 13.33 | |
| TE130 | 30+ fps = every 1 s window ≥ 30 frames | VISION §4 | conf. (prop.) | — | |
| TE131 | N from R1-TICK-OQ7 | ADR 0004 S2 | conf. (cross-draft) | 0004:824 | |
| TE132 | Size settings | r1 RR-OQ5 | conf. | r1:2949 | |
| TE133 | wasm-opt version recorded | r5 CI-OQ4 | conf. (R) | r5:1833–1834 | |
| TE134 | Every served file < 25 MiB | r6 642; N37 | conf. | r6:641–642; F3 verified | |
| TE135 | 25 × 1,048,576 = 26,214,400 | arithmetic | conf. | | |
| TE136 | First value becomes the first limit | r5 1737–1738 | conf. | | |
| TE137 | gzip level 9 | — | conf. (prop.) | — | |
| TE138 | Errors and casts vs three.js types | r5 TSLINT-OQ4 | conf. (R) | r5:1773–1775 | |
| TE139 | `skipLibCheck` on/off, median of 3 | r5 TSLINT-OQ6 | conf. (R; 3 runs prop.) | r5:1779–1780; source (fetch G02) "Skip type checking of declaration files" | |
| TE140 | Playwright clock in the Worker | r5 TEST-OQ1 | conf. (R); source checked | r5:1802–1803; fetch F10: the clock page "overrides native global classes and functions related to time"; "worker" appears only in the site menu ("Service Workers") | R5-TEST-F10 confirmed |
| TE141 | R1-DATA-OQ8 belongs to this spike | map part g; ADR 0005 CD-7 | conf. (cross-draft) but gap | 0005 content section:203 "Vite plugin pushes changed files to the page, which hands them to the Worker … (ADR 0002 places the plugin)"; :519–520 "How files reach the Worker under Vite is settled in the Rust ↔ TypeScript boundary spike (R1-DATA-OQ8)" | ADR 0002's decision and spike items 1–10 never mention the plugin or R1-DATA-OQ8; only PM-0002-31 does. Finding F-5 |
| TE142 | Page relay route | r1 DATA-REC5 | conf. | r1:2545–2547 "a small Vite plugin pushes changed files (F22) to the page, which hands them to the Wasm worker" | |
| TE143 | ≤ 1 s, 20 of 20 saves (CD-7) | ADR 0005 CD-7 | conf. (cross-draft; 20 saves prop.) | 0005 content section:516–518 | |
| TE144 | Editor from `file://`, five browsers | doc OQ-0002-1 | conf. (prop.) | | |
| TE145 | Wasm trap in Sentry, 4 of 4 | r6 OBS-REC3 | conf. | r6:765–766 | |
| TE146 | Only if the EU org exists | doc item 10 | conf. | | |
| TE147 | Mustafa's Brave run with checklist | plan rule 12 | conf. | doc:1136–1157 | |
| TE148 | Gate passes on `legacy/`; four mutation proofs | r5 LAYOUT-REC6 m | conf. | r5:1243–1245 | |
| TE149 | Tuning dump byte for byte | r5 1245 | conf. | | |
| TE150 | 2 runs ("byte-identical over two runs") | r5 1444–1445 | conf. (and at source) | docs/legacy-tuning.json:8 | |
| TE151 | Fresh clone offline tests | r5 LAYOUT-REC1 m | conf. | r5:1166–1168 | |
| TE152 | Fresh runner builds from the toolchain file alone | r5 CI-REC2 m1 | conf.; source checked | r5:1532–1533; fetch F02 (rustup overrides): the toolchain file's `targets` "contains a list of platforms to install for" | R5-CI-F12 / LAYOUT-F6 confirmed |
| TE153 | Network namespace | — | conf. (prop.; "not researched") | — | |
| TE154 | Versions equal pins in every log | r5 CI-REC2 m3; RUSTLINT-REC6 m; LAYOUT-REC1 m | conf. | r5:1534–1535; r5:1380 | |
| TE155 | Node, TS and CLI checked too | doc part 2 | conf. (prop.) | | |
| TE156 | Same lint result a week apart | r5 1380–1381 | conf. | | |
| TE157 | Deleting the toolchain file fails G0 with exit 2 | r5 1168–1169; 1712 | conf. | | |
| TE158 | `Cargo.toml` without lock fails first cargo step | r5 CI-REC2 m2 | conf.; source checked | r5:1533–1534; fetch F03 (`--locked`): "Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated." | R5-CI-F11 confirmed |
| TE159 | Missing browser → exit 2 | r5 1704–1705 | conf. | | |
| TE160 | Control: consistent bump | r5 1712 | conf. | | |
| TE161 | G1 mutations and control | r5 1713 | conf. | | |
| TE162 | After `prettier --write`, ESLint 0 problems | r5 1290–1291 | conf. | | |
| TE163 | Byte-identical Windows/Linux | r5 1372–1373 | conf. | | |
| TE164 | G2 mutations and controls | r5 1714; RUSTLINT marks | conf. | r5:1714, 1318–1320, 1336–1339, 1351–1352 | |
| TE165 | Build-flag mutations | doc part 4 | conf. (prop.) | rule is ADR 0004 part 7 (row 85) | |
| TE166 | Text-scan fallback | doc part 7; OQ1 | conf. | | |
| TE167 | G3: one per flag and per rule | r5 1261–1263, 1279–1281, 1715 | conf. | | |
| TE168 | `sim-worker`/`net`, `legacy/`, `spikes/` imports | doc part 3; r5 1268–1269 | conf. | | |
| TE169 | G4 | r5 1716 | conf. | | |
| TE170 | G5: transport/tokio/quinn in sim; feature check | r5 1189–1192, 1216–1218, 1717 | conf. | | |
| TE171 | G5: tokio in room; navbake/server from sim-wasm | doc part 3 | conf. (prop.) | | |
| TE172 | Wildcard; duplicate outside skip list | r5 1663 | conf. | r5:1663; GATE-F4 verified | |
| TE173 | G7 and budget-file raise | r5 1718, 1743–1744 | conf. | | |
| TE174 | G8 dynamic import; control; no editor file | r5 1191–1192, 1231, 1719 | conf. | | |
| TE175 | G9; one defect per layer except bench | r5 1720; 1407–1409 | conf. | | |
| TE176 | Three divergences at first tick, field diff | r5 1427–1428 | conf. | | |
| TE177 | G10 < 3 min | r5 1428–1429 | conf. | | |
| TE178 | 5 runs | r5 1580 | conf. (prop.) | | |
| TE179 | L1 identity | r5 1469–1470 | conf. | | |
| TE180 | Within 5 % relative | r5 1470–1471 | conf. | | |
| TE181 | 1,000,000 packets | NOTE | conf. (prop., reason shown) | | departs from R5's 100,000 with stated arithmetic |
| TE182 | SE arithmetic: 0.031 pp; 1.6 SE; ~1 in 9; 0.010 pp; 5 SE | arithmetic | conf. | √(0.01×0.99/100,000) = 0.000315 = 0.031 pp; 0.05/0.0315 = 1.59; P(\|Z\|>1.59) = 0.112 ≈ 1/9; √(0.01×0.99/10⁶) = 0.0099 pp; 0.05/0.00995 = 5.0 | |
| TE183 | G11 < 2 min | r5 1472–1473 | conf. | | |
| TE184 | Jitter vs L1's documented distribution | — | conf. (prop.) | | |
| TE185 | G11 mutations | r5 1722 | conf. | | |
| TE186 | G12 mutations | r5 1723; 1670 | conf. | | |
| TE187 | `forceWebGL` run | doc part 9 | conf. (prop.) | | |
| TE188 | `--prove` rule | r5 1706–1709, 1731 | conf. | | |
| TE189 | `--prove` < 30 min | r5 1731–1732; GL 146–147 | conf. | | |
| TE190 | Overflow in four builds; wrapping agrees | r5 1362–1364 | conf. | | |
| TE191 | No separate overflow-cost limit (draft sets none) | r5 1364 | conf. (reading); gap | r5:1363–1364 "the checks' cost … is reported with method …, and the ADR sets the limit" | ADR 0002 marks R5-RUSTLINT-REC4 "adopted" but sets no limit (it defers to R5-RUSTLINT-OQ4's spike): note N-6 |
| TE192 | `tsc -b` < 30 s warm | r5 1263–1264 | conf. | | |
| TE193 | Typed lint < 60 s | r5 1281–1282 | conf. | | |
| TE194 | Clippy < 30 s warm, < 3 min cold | r5 1319–1320 | conf. | | |
| TE195 | `--quick` < 60 s; full < 10 min; Linux and Windows; 5 runs | r5 1691–1692, 1703–1704 | conf. | | |
| TE196 | Linux on the session's machine, Windows on MP | — | conf. (prop.) | | |
| TE197 | CI ≤ 10 min p50 warm, ≤ 20 cold | r5 1579–1581; N40 | conf. | | |
| TE198 | 3rd of 5 | arithmetic | conf. | | |
| TE199 | Cold = each of 3 runs | — | conf. (prop.) | | |
| TE200 | TS-only PR passes; failing/cancelled turn it red | r5 1579, 1582–1583 | conf. | | |
| TE201 | Skipped job turns it red | doc part 9; r5 1557–1559 | conf. | r5:1557–1559 | |
| TE202 | Removing a check fails the aggregate job | r5 1704 | conf. | | |
| TE203 | Fork PR: no secret, deploy skipped | r5 1582; 1601–1602 | conf. | | |
| TE204 | Test account for the fork | — | conf. (prop.) | | |
| TE205 | Warm cache: no dep recompiled; ≤ 8 GB; 3 runs | r5 1518–1521 | conf. | | |
| TE206 | Warm `npm ci` faster; Playwright vs 57–67 s | r5 1553–1555 | conf. | | |
| TE207 | CLI install < 30 s over 3 runs | r5 1545–1546 | conf. | | |
| TE208 | 20 identical verdicts; timing test; retry-only test | r5 1503–1505 | conf. | | |
| TE209 | Every layer 20 in a row | r5 1409–1410 | conf. | | |
| TE210 | Hook ≤ 10 s p95 over 20 edits | r5 1299–1301 | conf. | | |
| TE211 | 19th of 20 | arithmetic | conf. | | |
| TE212 | Hook in a kit change-set | doc part 7 | conf. | | |
| TE213 | Types hold in CI | doc phase 2.4 | conf. | | |
| TE214 | Skeleton counts and milestones | doc phase 2.5 | conf. (cross-draft) | | |
| TE215 | 1,800 ticks | arithmetic | conf. | | |
| TE216 | Link ≤ 5 min, median of 5 | r5 1600–1601; N39 | conf. | | |
| TE217 | Deploy job runs no PR code | r5 1602–1603 | conf. | | |
| TE218 | Marker in the log | — | conf. (prop.) | | |
| TE219 | Header check; no COOP/COEP without flag | r5 1636–1640 | conf. | | |
| TE220 | Isolation on previews in five browsers | r5 1638–1640 | conf. | | |
| TE221 | Foundation end | GLOSSARY §8 | conf. | | |
| TE222 | Brave without a login | r5 1648–1650; PREVIEW-WALL | conf. | r5:1649 "a preview link opens in Brave on Windows without a login"; the foundation's previews reach no game server | |
| TE223 | Boundary marks on slice 1 at N = 12 | doc migration 3 | conf. | | |
| TE224 | Goldens bite; CS6 20,000/20,000; regeneration; list | r5 1451–1453 | conf. | | |
| TE225 | G13 pair | doc part 9 | conf. (prop.) | | |
| TE226 | 30-min bot room at 48, P2, 0/0/0, slope ≤ 1 MB/10 min; planted leak | r5 1487–1489 | conf. | | |
| TE227 | Tick time only from the reference server | GL 167–172 | conf. | GLOSSARY:168 "on one vCPU of the reference server" | |
| TE228 | 16.7 ms at 30 Hz | GL 168–169 | conf. | | |
| TE229 | G12 both backends; multiplayer preview behind the wall | doc migration 5; PREVIEW-WALL | conf. | | |
| TE230 | Round-trip test covers every kind | doc migration 6; r5 1391 | conf. | | |
| TE231 | G8 and editor = G13 | doc migration 7 | conf. | | |
| TE232 | Test-mode list under (a) incl. N = 64 headroom | GL 375–387 | conf. | GLOSSARY:385–389 | |
| TE233 | 30 min each over L1 at P2; one L2 run; 3-hour soak | r5 1478–1486 | conf. | | |
| TE234 | Online per-tick calls are 5 or 6 unless batched | doc part 5 | conf. (the inconsistency is real) | doc:265–272, 286–289 | F-2 |
| TE235 | Glue allocator call when bytes are passed | — | unverified (labelled not researched) | | |
| TE236 | Failing run never re-run until green | r5 1490–1498 | conf. | | |
| TE237 | Failed mark returns via Rollback routes | doc Rollback; plan | conf. | | |
| TE238 | Numbers change only in the owning draft | map part c | conf. | | |
| TE239 | Failing foundation mark blocks its change-set; budget raise reviewed | CLAUDE.md step 4; r5 1734–1738 | conf. | | |
| TE240–TE250 | "No measurable mark" lines (OQ-0002-2, part 2, 3, 4, 5, 7, 8, 9, 10, OQ-0002-7, no 60 Hz mark) | doc parts; ADR 0001, 0003, 0004 | conf. | doc:1114–1133 match the parts cited; ADR 0001's R2-ROOM-REC2 mark 6 at 0001:548; PM-0003-36 at 0003:988 | |
| TE251 | No CI time is a hardware result | r3 F8; doc part 8 | conf. | r3:648–650 | |
| TE252 | Mustafa's PC far above the minimum: never the budget | r3 1102–1103 | conf. | | |
| TE253–TE261 | Checklist steps' numbers and expectations (18,000 ticks; 1 and 6 minutes away; ~0.5 s; file://; legacy double-click; required check; no login; isolated yes/no) | as each row | conf. | TE14, TE119, TE122, R1-TICK-REC5 (r1:2113–2114), R5-PREV-REC4 m2 | checklists present with expected results: rule 12 met |
| TE262–TE270 | Spike entries for SPIKE_PLAN (names, order, decides lists, sizes) | map part g | not in the document | these rows describe `test-engineer-passmarks-0002.md`'s "Spike entries" section, which stays in the notes for plan step 6; the ADR does not contain them | both-ways check: ledger rows outside the doc (expected by plan step 6) |
| TE271 | P2's "on Low" = ADR 0003's tier | r1 2054–2057 | conf. (prop.) | doc:1031 "(proposed: "on Low" names ADR 0003's tier)" | |
