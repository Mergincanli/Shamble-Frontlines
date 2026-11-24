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
