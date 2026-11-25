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
- F2 evidence (run): sa_fnv_unwind.py (beside these notes, python3 -I): 4 hidden bytes between 22 and 40 known bytes
  recovered uniquely from the FNV-1a 64 hash in 0.06 s (2 x 65,536 steps); 2 hidden bytes at once; control passes.
- F3 Low (run, copy .gate/sa-4a at e742a24 = eb14739's source, own target .gate/sa-target, offline): with
  recordings/first-loop.vector.txt replaced by a symlink to a scratch file OUTSIDE the repository, `make_recordings`
  (write mode, files.rs:67-73 std::fs::write follows links) overwrote that outside file with the vector text; `--check`
  said only "first-loop.vector.txt differs" before, and PASSED after (the link is invisible to check, files.rs:80-109).
  Read, not run: a link to an endless device (/dev/zero) makes check (files.rs:84), tests/recordings.rs:18,
  two_processes.rs:98, two_instances.rs:42, replay.test.mjs:15 and the e2e spec:68 read until memory runs out.
  Names are constants and the folder fixed: no traversal from file contents; only a link in the tree. CI never writes.
  Fix: files.rs refuses a path whose symlink_metadata is not a plain file (and a linked folder), ~8 + test ~10.
  Also files.rs:101 prints a directory entry's name raw (eprintln in make_recordings): `{name:?}`, 1 line (read).
- F4 Low (run, same copy): `SHAMBLE_TWO_PROCESSES_CHILD=1 cargo test -p devtools --test two_processes` passes in
  0.32 s: the run takes the child role (two_processes.rs:48-52), starts no child, compares nothing. Reach: the outer
  environment only (CI's workflow, a repo .cargo/config [env]: both reviewed files). Test integrity, not a leak.
  Process start itself is safe: current_exe, constant args, no shell, the child returns at once (no recursion).
- Copy removed with `git worktree remove` (clean first); `.gate/sa-target` left (rm refused by permissions).
- Q3 read: make_recordings.rs:12 fixed folder (CARGO_MANIFEST_DIR/recordings), names from RECORDINGS (scripts.rs:131) +
  constant suffixes (files.rs:51-59); bin/replay.rs reads the developer's own paths, writes nothing, never echoes file
  content (map id never printed; parse errors give line numbers); tests/recordings.rs scratch = CARGO_TARGET_TMPDIR +
  constant case names, removed after (remove_dir_all does not follow a link at the top since Rust 1.58.1). No
  traversal. CI sets no SHAMBLE_TWO_PROCESSES_CHILD and the repo has no .cargo config.
- Q4 read: determinism.spec.ts:14 `http://replay.invalid` (reserved name, never resolves: a missed route fails closed);
  :44-51 exact names only -> two files in sim-worker/pkg, else 404; :17-28 constant Worker text, no interpolation; data
  in = a test seed (readRecording checks TESTSEED) and numbers; writes nothing; logs names, counts, user agent;
  :39-42 checks the file under test equals the built page's byte for byte. node-names.d.ts: type declarations only.
- Q5 read: state_hash.rs:2 "it is not a security check"; sha256.rs:1-4 "for the tools' own files ... Tools and tests
  only"; hash-report.md and PR #70's body claim no security property. What guards the committed files is check(),
  which makes every file again and compares byte for byte, not the manifest's digests. Nothing mistaken.
- Not findings, conditions for later (no 4a line): C1 before Sim::shared_hash ships in a build whose authority is
  elsewhere (decide at cs7 where the predicted world first exists apart from the room; binding at Slice 3): reads the
  client's own world only; no message either way carries a hash over state outside the recipient's interest set, and
  no match/mismatch answer over such state (ADR 0001:323-324); pinned by a protocol message-set pin + an interest test.
  C2 cs8 (streams join the authority part): ChaCha8 key/buffers only through `secret`; test that state_lines of two
  worlds differing only in seed are equal at tick 0 and after equal draws (~10 lines). C3 Slice 3: the server refuses
  a seed starting with TESTSEED for a live match (~2 + test). C4 paced replay mode (4c/4d, = N5): dev marker refused by
  the production guard, its own Sim from the recording's test seed, replayOn keeps checkReplayable (test-engineer F-d).
- Coverage (4a touches): page input (post, held, p0..p3) -> messages.ts:123-134 + host.ts:104-118 + room checks (3b),
  move/aim/view not taken yet (replay refuses them, replay.rs:50-66) -> room clamps them in 4c (cs3 SA L1); start ->
  messages.ts:121-122 + Sim::create; shared hash -> not reachable from the page; a recording (tests/tools only) ->
  decode/readRecording, gap N2/F1; committed files -> check(), gap F3 (links).
- Verdict: APPROVE (all Low: F1 = N2 + Debug, F2 wording 2 lines, F3 hardening ~18, F4 test integrity ~2). Any not
  fixed in the PR counts as a security finding left open (advisor.md:103, :180-181, :267-268): name it to Mustafa.
DONE
