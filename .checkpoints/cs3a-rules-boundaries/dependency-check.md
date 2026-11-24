# The release record of change-set 3a's dependency change (.claude/CHECKPOINTS.md, section 3)

Written by the main session on 2026-11-24 13:35:41 UTC (from the clock), after security-auditor's answer and before the manifests
were committed. The answer: `.checkpoints/cs3a-rules-boundaries/security-auditor-deps.md` (security-auditor, Opus 5.5
at effort max, 2026-11-24, 13:12 to 13:25 UTC; the four files hashed to these values when it started and when it
ended, by its own lines): "The dependency change may be committed and built: yes with conditions (1 and 2 above;
neither changes one of the four files)." What it read: `deps-manifests.diff`, `deps-lockfiles.diff` and
`deps-sha256.txt` beside this file. No package from outside enters or changes (Mustafa's RNG-SOURCE (c): our own
ChaCha8, no random-number crate).

Its condition 1 (show on the batch alone that the players' Wasm does not change) is met, 13:27 to 13:29 UTC, in a clean
worktree `.gate/cs3a-batch` of the head (code = 50656d6): the head's code with a clean target folder gives
`pkg/sim_wasm_bg.wasm` 28,987 bytes, sha256 c4935acaa142ce1046cf30fa1f6f3edd7697834f32c7536159ce58972ed32c31; the
same code plus only the four files, after `npm ci --offline --ignore-scripts` and again with a clean target folder,
gives the same bytes and the same sha256; the four files hashed to the values below after the install and after the
build. Its condition 2 is step 7's (the test module's own cargo call and target folder, an ignored folder outside the
page's and the Worker's, no page or Worker import, and the build's refusal of any other .wasm in an out folder, shown
on a planted copy). Its Low follow-up: the root `Cargo.toml`'s comment "in sim-wasm only" (line 14), for the next
manifest batch.

One line per file, as `sha256sum` prints it from the repository's root:

8812561a8beed25cae2e80f85b14ed7530ff3564058e63278ca64725faa1874e  crates/devtools/Cargo.toml
8f23222148e355b4775be8408c64dd37af6399d30a7e2f5559df3664da6c5c3b  Cargo.lock
c22862869340e0cbc0e41dd2ade215ee454382841eb5b604a9b917b78b76cb06  packages/sim-worker/package.json
30ccbd5e32fc6d22e88b15392bee46211b4c8f3529bcd35188958d365f85e474  package-lock.json
