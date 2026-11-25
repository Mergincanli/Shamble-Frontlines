# security-auditor, review round 1 of change-set 4a "The recording"

Head: worktree `.gate/lane1-takeover`, branch cs/4-walk, source as at eb14739 (notes head a874d71). Diff:
`git diff c1da1bd...HEAD -- crates packages`. Memory: no agent-memory folder exists for this agent (checked
`.claude/agent-memory*` in the main folder and this worktree): nothing earlier to carry.

## Threat model (five lines)
1. Every client is hostile: the page, the Worker and the players' WebAssembly file can be read and changed; any
   export on that file is callable by the player with any argument, any number of times.
2. The match seed (ADR 0004 part 8) is a secret of the authority: anything derived from it (whole-state hash, random
   draws, a state dump) is as secret as the seed while the match is live; a leak lets a client predict draws.
3. Repository files are attacker-reachable input for developers and CI (a public repo: a pull request can change a
   committed recording or hashes file); the readers must not panic, allocate unbounded, or steer file writes.
4. Test-only code (the devtools Wasm module, recording.ts, the e2e spec) must not reach a build players get; if it
   does, its readers become player-reachable.
5. The tools run with the developer's rights: paths from the command line are trusted (the developer's own), paths
   or names from file contents are not.

## Plan
1. Diff stat: manifests, lockfiles, CI, kit, gate files (question 5).
2. Seed paths: world.rs (StateSink::secret, visit, visit_shared, write_shared_state), room lib.rs (shared_hash),
   sim-wasm lib.rs (Sim::shared_hash), replay.rs Lines, recording.rs check/encode/decode, recording.ts; every place
   that prints, logs, writes or returns a value computed from the seed (question 1).
3. Can the page reach shared_hash: worker.ts, host.ts, messages.ts (question 1).
4. Readers: Recording::decode, parse_hashes, wasm.rs exports, readRecording/readHashes; where the test module is
   built and lands (build.mjs); who imports recording.ts (question 2).
5. Files and processes: files.rs, make_recordings.rs, replay.rs bin, tests/recordings.rs, tests/two_processes.rs
   (question 3).
6. Browser spec: determinism.spec.ts, node-names.d.ts (question 4).
7. FNV-1a / SHA-256 named or used as a security check anywhere (question 5).
8. Findings with severity, lines, follow-up change-set; coverage table; not verified.

## Findings (a line each, as found)
- Q5 checked: `git diff --name-status c1da1bd...HEAD` outside .checkpoints = crates/ and packages/ source + 4 files in
  crates/devtools/recordings/; no manifest, lockfile, CI, kit, gate or build-script file; devtools/Cargo.toml unchanged;
  new imports are Node built-ins only (node:fs, node:path, node:url, node:crypto).
- Q1 page reach checked: HostSim (host.ts:9-15) has no shared_hash; the page's 4 message types (messages.ts:10-34,
  checked at :118-142) never call it; worker.ts keeps Sim in module scope; the built Worker chunk (.gate/dist-production
  and dist-development, worker-Cw_wzaMy.js) assigns no globals; it holds the glue wrapper `shared_hash(){...
  sim_shared_hash...}` (class methods are not tree-shaken), reachable only by code holding a Sim. NOT reachable.
- Q1 seed paths checked: outside tests only devtools computes the whole-state hash (replay.rs:125); room/sim-wasm call
  write_shared_state only (room lib.rs:132, world.rs:189-192); players' file raw exports (node, WebAssembly.Module.exports)
  = one new `sim_shared_hash`, no state_hash/fnv/devtools strings (grep -a on pkg wasm, sha256 e419e4cc..., 55,607 B,
  equal in both built pages). No path for a real seed or a value computed from it in 4a.
- Q1 leak today: the shared hash covers tick + own buttons + own presses: all in the block the page already gets. Nothing.
- Q2 checked: test module built only in non-production builds into .gate/rules-module (build.mjs:11-16, 71-72, 122-136),
  outside the dev server's allow list (vite.config.mjs:37) and refused in a built page (build-checks.mjs:189-197); both
  built pages hold no SHMBLREC/TESTSEED/readRecording text. recording.ts: out by import graph only (= N5).
- F1 Low (= N2/R2, plus): replay.rs:87-99 and recording.ts:131-139/149-154 do not re-check TESTSEED; also recording.rs:38
  `Header` derives Debug with the seed (World and MatchSetup deliberately have none). Fix in Replay::new (one entry for
  tools, files.rs, test module) + checkReplayable; optional hand-written Debug. ~10 (+8 optional).
- F2 Low (doc now, tests later): world.rs:187-188 (pre-existing line, now on the players' file) calls the shared hash "the
  one a client's state can be compared with"; ADR 0001:323-324 says no state hash is exchanged with clients. From 4c the
  shared part holds every soldier. FNV-1a steps are invertible (xor, then multiply by an odd constant mod 2^64): a hash over
  mostly-known bytes gives the few unknown back (meet in the middle: 2^(4n) work for n unknown bytes). Conditions for
  cs7 / Slice 3 written in the report. ~2 lines now.
