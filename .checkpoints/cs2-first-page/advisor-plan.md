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
