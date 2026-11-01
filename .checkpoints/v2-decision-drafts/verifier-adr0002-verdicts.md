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
