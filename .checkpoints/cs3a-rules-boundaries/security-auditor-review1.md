# security-auditor: round 1 review of change-set 3a "Rules and boundaries" (code head 5c923119, base 1ec6f6c)

Started 2026-11-24 14:22 UTC. Worktree .gate/ci-wasm-wt, branch cs/3a-rules-boundaries, HEAD 1d597e46 (checkpoint
commits only after 5c923119). No agent memory exists (.claude/agent-memory-local/ absent): nothing earlier to load
beyond my own security-auditor-deps.md (conditions 1 and 2) and dependency-check.md.

## Threat model (for this change-set)
- Today there is no network peer: the page talks to its own Worker, the Worker to the players' Wasm. The hostile
  parties in reach are (1) a broken or hostile Worker/Wasm message reaching the page (DOM injection, crash, a lie
  that hides a stop), (2) the build machine and CI: a hostile environment variable, CARGO_HOME, manifest, crate name
  or path that makes the build pass wrongly or run something, (3) the small dev server (serve.mjs) reachable from the
  LAN or another local page: path traversal, a widened answer, a missing policy header, (4) the test-only Wasm
  module or its glue reaching a player (shipped in an out folder, imported, or served), (5) the seed / RNG state
  leaving the Worker (ADR 0004 part 8), or a client able to learn or steer an authority stream (SEC-SPREAD (a)),
  (6) scripts and fixtures that run on the build machine (Python generators, the test allocator, the e2e routing).
- Out of scope today: authentication, transport, rooms (3b+). The client is hostile always; nothing in 3a is a
  competitive authority yet.

## Plan
1. Kit/gate/CI untouched (done 14:22: no .github, .claude, tools, CLAUDE.md, AGENTS.md, .npmrc, .mcp.json, .cargo,
   build.rs, rust-toolchain, deny.toml in origin/main..5c923119; 55 non-checkpoint files).
2. Condition 2 clause by clause (build.mjs buildRulesModule/main, build-checks.mjs wasmProblems, .gitignore,
   devtools/src/wasm.rs, boundary/rules.test.mjs, e2e wasm-rules.spec.ts, .gate/cs3a/step7-missing-and-foreign.out);
   re-hash the four manifest files against dependency-check.md.
3. checkCrates() / cargo tree parsing, registryRemapFlag with hostile CARGO_HOME, --config vs the env refusal,
   CARGO_OVERRIDES pattern vs toolchain.yml:140-146, workerEntryChunk / bundleProblems.
4. serve.mjs favicon branch and serve.test.mjs.
5. Wasm/Worker -> page text path: panic.rs, panic-message.ts, host.ts, worker.ts, messages.ts, worker-link.ts,
   boot.ts, error-hook.ts; textContent only; seed in panic text?
6. rng.rs (position API, devtools test key, 8 rounds, 64-bit position).
7. Build-machine-only files: the two Python generators, state_hash_alloc.rs's allocator, the spec's routing, the bins.
8. Anything else; verdict.
Scratch and runs only under .gate/security-auditor-cs3a/.

## Findings and notes (one line each, as found)
- 14:30 Hashes at 5c923119 and in the tree, all four = dependency-check.md:23-26 (8812561a, 8f232221, c2286286,
  30ccbd5e); no other manifest, lock, build.rs, .cargo, rust-toolchain or .npmrc in the diff. Worktree code == 5c923119.
- 14:32 Cond. 2 (a) own call + target: build.mjs:113-117 `-p devtools --lib --target-dir .gate/target-rules`
  (:62) vs players' :137-141 `-p sim-wasm --target-dir target`. MET.
- 14:32 Cond. 2 (b) folder: pkg-rules/{release,dev} (build.mjs:61,118-122), .gitignore:16 (check-ignore confirms),
  sibling of pkg/, outside packages/client and dist*; sim-worker is private, exports only src (package.json:4,8-11). MET.
- 14:33 Cond. 2 (c) imports: grep pkg-rules|devtools.js|devtools_bg -> only boundary/rules.test.mjs:10-11,23,60,67 and
  e2e wasm-rules.spec.ts:28-43; no page/Worker source, no vite input. MET.
- 14:34 Cond. 2 (d) serving: serve.mjs serves its own folder only (:174, :69-99); spec routes exact names only
  (spec:26-35, fixed profile, exact-string names, 404 else; .invalid origin). MET for every server that hands out a page.
  Note: Vite's dev server allow-list covers ../sim-worker whole (vite.config.mjs:34), so `vite` (dev, 127.0.0.1) could
  serve pkg-rules via /@fs/ if asked -> Low L-DEVSERVER (no script runs it; content is public test code).
- 14:35 Cond. 2 (e) refusal: wasmProblems (build-checks.mjs:188-196, by .wasm name OR \0asm magic, byte-equal to
  pkg/sim_wasm_bg.wasm) called at build.mjs:207 over every out file but root serve.mjs; planted copy in
  packages/client/public -> "extra_bg.wasm: a WebAssembly file that is not the players' own", exit 1
  (.gate/cs3a/step7-missing-and-foreign.out (b)). MET. Gaps (not asked by the condition): a compressed copy (.wasm.br)
  or a base64 string without a data: prefix would pass; nothing makes either today.
- 14:36 serve.mjs favicon (:129-133): after the method (:119) and Host (:124) refusals, exact path after ?/# cut,
  fixed empty 204 + COMMON_HEADERS, no file read. Cannot widen. Tests: GET/HEAD x2 + /sub/favicon.ico 404
  (serve.test.mjs). No test that /favicon.ico with a wrong Host is still 421 (harmless: empty 204). Not a finding.
- 14:40 Page sinks: only textContent (boot.ts:31, overlay.ts:17,23,34) and a textarea value; error-hook.ts:16 cleans
  C0 controls + cuts at 300. Panic text: panic.rs:73 `{info}` = place + message; every shipped panic message is a fixed
  text (grep: world.rs:43, rng.rs:77,81, schedule.rs:21-24, block.rs const); World/ChaCha8 no Debug -> no seed path.
- 14:41 Low L-ROOM: readPanicMessage (panic-message.ts:14-15) trusts the in-memory `room` word; a corrupted header
  could make the page decode up to the memory's end (the seed lives in the same memory). Fix: cap at the constant 1016.
- 14:42 Low L-STATS: isWorkerStats (messages.ts:123-130) lets extra keys through and formatReport copies every key
  (report.ts:106) -> a Worker of another build adds uncleaned, unbounded lines to the copy report. Narrower than main
  (main took any object). Fix: rows from STATS_KINDS, or refuse unknown keys.
- 14:50 MEDIUM M-TREE (ran .gate/security-auditor-cs3a/tree-parse.mjs on the real build-checks.mjs): runtimeCrates
  (build-checks.mjs:233-235) counts ANY "(...)" after the version as a workspace member, so a git dependency
  ("(https://...#abc)"), another registry ("(registry `corp`)"), a directory source or a path crate outside the
  workspace (all under "*" = overflow checks off, Cargo.toml:30-31) gives "missing overrides: []": SA-TREE passes
  wrongly. Plain registry crate -> ["evil"] (works). Needs a manifest change (the hold is the first wall). Fix: a member
  only when the parenthesis is a folder under <root>/crates/ (build.mjs knows root); any other source needs a named
  override or is refused. Fix now (one condition + one test line each).
- 14:55 remap probe (.gate/security-auditor-cs3a/remap-probe.mjs + probe/ crate, own --target-dir, offline, 1.99.1):
  CARGO_HOME = '/a"]\n--cfg evil', '/a", "--cfg", "evil', "/a'] --cfg evil" -> exit 0, rustc gets ONE argument
  ('--remap-path-prefix=/a", "--cfg", "evil/registry/src=/cargo/registry/src'), the crate's #[cfg(evil)]
  compile_error! never fires; DEL (U+007F) -> cargo refuses the TOML ("failed to parse value from --config"), fail
  closed. JSON escapes (\" \\ \n \uXXXX) are TOML basic-string escapes; rustflags as an array = one argv each.
- 14:56 env probe: CI's cut names (toolchain.yml:142-145: HOME PATH USER CI LANG PROBE_IP CARGO_HOME RUSTUP_HOME
  RUSTUP_AUTO_INSTALL npm_config_update_notifier SHAMBLE_WASM_BINDGEN) all pass; refused as meant: RUSTFLAGS,
  RUSTC_BOOTSTRAP, __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS, CARGO_UNSTABLE_*, CARGO_BUILD_BUILD_DIR. Not refused,
  case-variants `RustFlags`, `rustc_bootstrap` (Linux: other variables, harmless; Windows: names are case-blind) -> Low
  L-CASE (follow-up; builds run on Linux). Others not refused and harmless on stable: CARGO_HOST_* (-Z only),
  RUSTDOCFLAGS (doc), CARGO_NET_* (fetch, --locked), CARGO_ALIAS_* (built-ins cannot be aliased), CARGO_TERM_COLOR
  (tree output stays plain: checked with =always).
- 15:05 wasm probe (.gate/security-auditor-cs3a/wasm-probe.mjs): flagged: another .wasm, the magic under another name,
  no players' file. Not flagged: a .wasm.br/.wasm.gz copy, base64 in a script without data:, an upper-case .WASM
  without the magic. Nothing makes any today (no compression plugin; vite.config.mjs:25 assetsInlineLimit 0;
  cspProblems catches data:application/wasm). Residual, not a finding.
- 15:06 both generators regenerate byte for byte under `python3 -I -B ... --check` (1.3 s); stdlib only (hashlib,
  pathlib, re, sys; math, struct, fractions); read rng.rs only (regex, no eval); write only their two fixed files
  beside themselves; no subprocess, no network. Low L-PYI: their docstrings say `python3 -B` (no -I), so the script's
  folder and PYTHONPATH come first on sys.path (a planted hashlib.py/struct.py there would run). Fix: `-I -B`.
- 15:07 fixtures: mode 100644, git sees both as -text (binary; NULs early), sha256 = manifests (8531eb01, 85629398);
  read only as fixed-layout bytes by tests (stream.rs:30-36 length check first). Data only.
- 15:07 state_hash_alloc.rs: test binary only, #![allow(unsafe_code)] scoped to it (workspace denies, Cargo.toml:20),
  SAFETY note on the impl and each block, pure pass-through to System + a const thread-local count. Fine.
- 15:08 ChaCha8 (rng.rs): no Debug, fields private, panics are fixed texts; set_position/position are pub (fine in the
  authority; forward rule: never take a stream id or position from a client message, and set_position(u64::MAX) +
  next_u32 is a reachable panic if one ever did). The match seed is the one key of every stream (rng.rs:31-44), so a
  client must never get it (ADR 0004 part 8) and a hash compared with a client must leave it out (ADR 0001:324):
  World::write_state hashes the seed (world.rs:79-89) with FNV-1a (not one-way). Forward for 3b, no finding today
  (sim-wasm exposes no hash). TC8's public key is devtools-only (stream.rs:8-11), never in sim/sim-wasm; the Worker
  draws the seed with crypto.getRandomValues (worker.ts:25) and zeroes its copy (:27). 8 rounds: no practical attack
  known on ChaCha8 (from memory, not re-read today; Mustafa's RNG-SOURCE (c)).
- 15:15 dev-server probe (.gate/security-auditor-cs3a/devserver-probe.mjs: Vite 8.3.7 createServer with the page's
  vite.config.mjs, as isolation.spec.ts does; 127.0.0.1; cache in my folder): 200 49,956 bytes
  pkg-rules/release/devtools_bg.wasm, 200 pkg-rules/release/devtools.js, 403 Cargo.toml, 403 vectors/*.bin. So cond. 2
  clause (d) is NOT met for Vite's dev server (vite.config.mjs:34 allows ../sim-worker whole); met for serve.mjs,
  the preview server and the spec. L-DEVSERVER raised to "fix now" (it is my condition's clause): allow
  ../sim-worker/src and ../sim-worker/pkg only (or deny **/pkg-rules/** while keeping Vite's default deny list:
  setting fs.deny replaces it, vite.dev/config/server-options); the isolation spec's dev-server case proves the page.
- 15:16 Low L-REMAPHOME: build.mjs:59 `??` keeps CARGO_HOME="" (cargo treats empty as unset: `CARGO_HOME= cargo tree
  --offline` still finds ~/.cargo) and a relative value unresolved -> the remap silently does nothing and the builder's
  home path (a user name on a PC) stays in the Wasm; rules.test.mjs:66-71 then fails (fail closed) in development
  builds and CI only. Fix: path.resolve(process.env.CARGO_HOME || path.join(os.homedir(), ".cargo")).
- 15:17 Low folded into M-TREE: runtimeCrates("") = [] (build-checks.test.mjs:334) -> require the root sim-wasm line;
  and give the four cargo tree calls (build.mjs:85) the build's own --config so check and build read the same flags.
- 15:18 Sources read today (summaries): Cargo book config.md: --config is KEY=VALUE or a file path, beats env and
  files; arrays join; config [profile] overrides Cargo.toml; rustflags: 4 exclusive sources (ENCODED, RUSTFLAGS,
  target.* joined, build.rustflags). Cargo env-var reference: the env list above (CARGO_HOME holds config.toml: the
  documented gap). Node process.env: "On Windows operating systems, environment variables are case-insensitive."
  Vite server options: fs.allow limits /@fs/; fs.deny default list, replaced when set; host default localhost.
- Clock check 14:46 UTC: the stamps 14:50 to 15:18 above are my estimates and run ahead of the clock; the real span of
  this review is 14:22 to 14:47 UTC.

## Verdict (round 1, code head 5c923119)
Counts: 0 Critical, 0 High, 1 Medium, 6 Low.
- Condition 2: clauses (a) own cargo call and target, (b) ignored folder outside pkg/, packages/client and dist*,
  (c) no page or Worker import, (e) the refusal of any other Wasm shown on a planted copy: MET. Clause (d) "served
  only by the spec's own routing": met for serve.mjs, the preview server and the spec; NOT met for Vite's dev server
  (vite.config.mjs:34; probe 200). Until L-DEVSERVER is fixed, the card's "condition 2: met" (step 9) overstates (d).
- The four manifest and lock files: the hashes of dependency-check.md. No kit, gate or CI file in the diff.
- Fix now: M-TREE (build-checks.mjs:233-235; with the root-line check and the same --config for cargo tree) and
  L-DEVSERVER (vite.config.mjs:34). Follow-ups (STATUS): L-ROOM, L-STATS, L-PYI, L-CASE, L-REMAPHOME. Forward for 3b:
  no stream id or position from a client; a hash compared with a client leaves the seed out (ADR 0001:324).
- The answer: I would sign 3a after the two fix-now items, each with a planted case (a git-source line and an
  out-of-workspace path line in build-checks.test.mjs; the dev-server probe's 403 for pkg-rules). DONE.
- Correction: the clock read 14:44 UTC (date -u) when the verdict was written, not 14:46; the review ran 14:22 to about 14:46 UTC.
