Wed Nov 18 21:05:18 UTC 2026
advisor plan review of cs2-first-page plan.md: read plan, draft diff, resume card, Mustafa's words, build plan parts, CHECKPOINTS §3, toolchain.yml, crates/packages, ADR lines; rule on Q1-9; findings appended below as made.

## Findings as made (time of this block below)
Wed Nov 18 21:21:30 UTC 2026
Read: plan.md (all 915 lines), diff vs plan-draft.md, resume card, his 20:20:09 message, DECISIONS 2026-11-17/18 entries,
build plan :1-320, :440-610, :740-790, :920-1030, Part 15 :57-186, ADR 0004 parts 3-9, ADR 0002 parts 2-6 and
:1078-1097, :1287-1337 marks, :1468-1512, ADR 0003 parts 1-4, CHECKPOINTS §3, checkpoint.mjs hold, toolchain.yml,
Cargo.toml, crates (sim, sim-wasm, client-endpoint layout.rs), package.json files, package-lock (vite entries), routine-guard :28-90.
Verified: TS 6.0.3 getAutomaticTypeDirectiveNames returns `options.types ?? []` (node_modules/typescript/lib/typescript.js:44672-44675),
so @types/three cannot enter CI's two tsc lines on packages/types (Q4). gate.mjs/smoke.mjs read only legacy/ (Q4).
Cargo.toml has [profile.release.package."*"] overflow-checks=false (OVERFLOW-3P real). vite 8.3.7 in lock dev+peer (37 peer entries).

F1 Medium (plan :704-716, :781-782): OVERFLOW-3P misstates the register row OVERFLOW-ALL (docs/adr/README.md:234): the row is
  "the advisor agent's rulings D1 and D1-R2 ... never his decision; his go for this row: PR #48's merge"; its trigger is "when the
  first third-party crate enters any workspace crate's dependency tree" (ts-rs did, cs1, dev-only; cs1's report is silent), not
  "the player's Wasm"; its choice is "with its cost measured ... a named override for each such crate, or dropping "*"". The
  recommended (a) keep "*" is a third option, unmeasured. Must say (a) leaves the row's text, why ts-rs did not trigger it, quote
  the row; add to section 17.
F2 Medium (plan :319-331, :372-375, :593-595): the wasm-bindgen CLI (code from the internet, in no manifest): the answer is to
  cover "its release attestation", but no step verifies the attestation against the downloaded file, before the first run
  (build.mjs's checkCli runs `--version`: a run), nor says what happens if it cannot be read (proxy refuses github.com pages of
  other repos: risk 14). The .sha256sum is the same origin as the tarball. Unpacking an untrusted tarball: no listing check, no
  fresh empty folder named. Fix: sha256 vs recorded value before unpacking; list members (no absolute path, no "..", no link);
  unpack into a new empty folder; verify the attestation (gh attestation verify --repo wasm-bindgen/wasm-bindgen, or the
  attestations API by digest) before any run; if not verifiable, stop and ask him.
F3 Medium (plan :98, 4.1 table): "the muzzle's point and the plain-line hook (8)" and "warm-up and quality (lane 2's need for 6)"
  contradict Part 15.2 (part-15 :117-121): "Lane 1 builds all of these in change-sets 2 to 5, on the plain stand-in, so that 7, 8
  and 9 need nothing new from lane 2" -- after 5 merges render/ is lane 2's, so lane 1 cannot build them at 8. Not in DEPART-2 or
  section 17. Fix the table to "by 5" (or list it as a departure for his answer).
F4 Medium (plan section 12, :604-616): netcode-architect missing. CLAUDE.md step 5: "netcode-architect (sim, server, protocol)";
  this change-set gives `sim` its first code (world, tick counter's overflow rule, authority step, canonical bytes, state hash) and
  the tick pacing/schedule whose contract ADR 0004 part 5 shares with the online path. Precedent: my cs1 APPROVE condition added
  netcode-architect (cs1 report :49-51). Add it (3+3+1 is still one round).
F5 Medium (plan :534-538 vs section 13): the hand-off readings are claimed "in the container's Chromium and in Brave from his
  reports"; the build plan wants "a reading on your laptop" (build plan :540), but the Brave checklist collects only the default
  (8-byte) size. Add padded runs to section 13 (with the URL flag, copy report each) or drop the Brave claim.
F6 Medium (CLAUDE.md step 4, research): RNG-CRATE's and the OQ-0002-7 reading's load-bearing claims (rand 0.10.0 changelog line,
  PR #1658 merged 2026-01-23 + "Source moved", dhardy's sentence and date, chacha20 0.10.2 `rng`; #4533 maintainers, no written
  release policy) were read by a fetch agent / the main session only; no verifier spot-check before they go to him. I could not
  check them (no fetch by my brief).
Wed Nov 18 21:24:52 UTC 2026
F7 Medium (plan :143-146, :400-404): e2e layout departs from Part 15.2 (part-15 :106, :108, :110): lane 2's picture, asset and
  feel tests are named `packages/e2e/tests/look/`, `tests/assets/`, `tests/feel/`; the plan makes `tests/` the browser-free `node`
  project that CI's `npm test` runs and puts browser specs in `browser/`. Lane 2's browser tests would land in the node project.
  Not in DEPART-2/section 17. Fix: keep Part 15.2's paths (chromium project = `tests/` minus a browser-free subfolder such as
  `tests/node/`, which `--project=node` runs), or list it for his answer.
Process note (Low): step 0's OQ-0002-7 reading ran 21:04-21:08 UTC, before this APPROVE (commits dd8077f, 1950f82). Notes are not
  edits (CLAUDE.md Never line), so no breach; but plan :30-31 says steps 0 and 1 start on the APPROVE: the report must say so with
  its times. Its result fired the plan's own stop rule (:349-350): no written release policy -> new question WB-HEALTH, not yet in
  plan.md: it must enter section 16 (reporting rule) and "what holds which step" (step 2 waits for it too).
L1 Low (:30, :72): step 1 is not "the same under every answer": under RNG-CRATE (a)/(b) World and write_state gain the stream state
  after step 3. Say "its code stands under every answer; (a) or (b) adds to it".
L2 Low (:132, :147): `.gitignore` `pkg/` matches any folder named pkg at any depth: use `packages/sim-worker/pkg/`.
L3 Low: OVERFLOW-XCRATE-CI's anchor "(or the first crate to depend on sim)" (STATUS.md:95; archive :637-651) is reached here
  (sim-wasm depends on sim); PM-0002-53's counter test is cs3's (build plan :475): name the deferral in section 17 (point 8 names
  only WASM-ARTIFACT and OVERFLOW-PER-CRATE).
L4 Low (:317-318): the lockfile rule's first sentence ("may only add entries and change the lines of our own crates and packages")
  will stop the step on a predictable change: vite 8.3.7 and 36 other entries carry "peer": true today (package-lock); a direct
  vite declaration likely drops that flag on them. Allow and list flag-only changes (peer, dev, optional, devOptional); keep the
  stop for version, resolved, integrity, dependency lists.
L5 Low (:381, Part 0 :223-226): build.mjs "copies serve.mjs into dist/" reads unconditional; Part 0 keeps the zip's server out
  of the production build: copy it only in the development flavour; the production check shows it absent, and names its method
  (a scan of dist-prod for named dev-only markers, with the dev build as control), with `padBytes` and `?backend=webgl` named
  in that list or justified.
L6 Low: ADR 0002 part 2/7 CSP-ready (no inline script or style; no blob:/data: Worker or Wasm, :436-437; G8 :582) is claimed
  (:392) but not checked on the built output; add a scan of dist/ to build.mjs's checks or the evidence.
L7 Low: ADR 0002 part 5's API text ("returns where they lie", :338; API list :330) differs from the plan's step (due time only;
  block_ptr/len read once) and the added `rebase` export: technical and inside PM-0002-3's intent; list it in section 17 for the
  draft's next revision.
L8 Low (section 13 :634-637): the hidden-tab steps are not measurable as written ("the number it had when you left"): ask him to
  note the count before leaving; expected within about 30 of it on return (no jump of ~1,800 per minute away).
L9 Low (section 12): art-director (CLAUDE.md step 5: visuals, UI) not called and not explicitly deferred: state why (placeholder
  scene that decides nothing, dev-only readout, his Brave check is the gate) or call it.
L10 Low: keep the state hash off the page, readout, copy report and logs (it covers the seed; FNV-1a is not one-way; ADR 0004
  part 8 forbids a client-derivable seed); a note for PM-0004-21's pick at cs3.
L11 Low: PM-0002-20's own bite (COOP without COEP must read false, ADR 0002 :1292) is not among the planted defects/cases.
L12 Low: SIZE-2 (a) is the status quo and 2,900 is inside the build plan's own tolerance (:198-200: "half as much again" = 3,000):
  it can go to him as information, not a question; harmless if kept.
Wed Nov 18 21:26:53 UTC 2026
Corrections to refs above: ADR 0002 CSP-ready is :434-436 (not :436-437); G8 :582; Part 15.2's sentence :120-121.
Spot-checks of the plan's own citations (all hold): build plan :215-218, :198-200, :27, :472, :925, :939, :941, :977-979,
:1008; ADR 0002 :1086-1093, :1495-1500; toolchain.yml :93; routine-guard :38 (.npmrc) and :84 (MERGE_PLACES);
archive :807-812, :817-831, :850-856; kit-hold-ci/plan.md :40-41 and M2 :112 at f167f061; PM-0004-21's text (ADR 0004
:1142) matches the plan's "function picked and its cost per tick". Not checkable here (no fetch by my brief): the RNG-CRATE
and OQ-0002-7 web claims, RFC 9923's 64-bit vectors, registry sizes and dates.
Saver note: checkpoint.mjs:226 `if (background && codeAhead)` -> watch never pushes code commits; the plan's reason for the
record-before-commit order is moot (the order is still right); pushes go through a foreground `save`.

## Verdict: APPROVE, with conditions (see the hand-back). Step 1 may start now; step 2 waits for his answers (RNG-CRATE,
OVERFLOW-3P, DEPART-2, WB-HEALTH) and for conditions 1-5; condition 6 before the download; 7 before the round; 8 before
the Brave check.

## Changed lines, read once
Wed Nov 18 22:00:59 UTC 2026
Plan: read git diff 29da1ca HEAD -- plan.md, the verifier's table, security-auditor's tool notes, sections 16-17 whole; rule on (a) step 2's three manifest-free parts and (b) TOOL-SOURCE (c).
