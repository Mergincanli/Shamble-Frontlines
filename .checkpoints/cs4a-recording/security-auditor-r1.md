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
