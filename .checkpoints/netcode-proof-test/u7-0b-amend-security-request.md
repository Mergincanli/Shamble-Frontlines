# U7 Block 0b amendment: two dev-dependencies for C6's room test (security-auditor under DEP-HOLD; 2026-11-25 19:1x UTC)
You are security-auditor for one small read. Read only, effort xhigh. Your one output file:
/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/security-u7-0b-amend-answer.md.
Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.
git and gh always start their own command, with no other program in front of them.
Write your plan first and a notes line after each check, so a usage cut loses nothing. Never `cd` into a worktree; no
`git -C`; one simple command per Bash call; run nothing that installs, fetches, builds or writes outside your file (no
cargo build, test, update or fetch); read nothing from the web. Read the files below by absolute path.

What it is (plain words): DEP-HOLD (.claude/CHECKPOINTS.md section 3, :60-69): a manifest or lockfile change is pushed
only after your answer on exactly that content, recorded in dependency-check.md. U7's T-builder (brief 3, the Rust half)
reports that C6's in-process room test (advisor-u7-brief.md:167-173) needs npt-link and npt-proto as direct
dev-dependencies of npt-client-wasm: npt-server's `Decoder::pump` takes `&mut dyn npt_link::Link`
(spikes/netcode-proof/crates/server/src/pump.rs:372-378), and the harness C6 points to
(spikes/netcode-proof/crates/server/tests/common/mod.rs) names `npt_link::{Loopback, Link, ...}` and
`npt_proto::{Message, Quant, Join, ...}`. Block 0b gave npt-room, npt-server and npt-sim only (your
security-u7-0b-answer.md; dependency-check.md, both in .checkpoints/netcode-proof-test/).

The amendment, prepared in the scratch worktree /home/user/Shamble-Frontlines/.gate/u7-amend (detached at c21582d;
nothing committed; the branch's own copies are in /home/user/Shamble-Frontlines):
- spikes/netcode-proof/crates/client-wasm/Cargo.toml: `[dev-dependencies]` gains `npt-link.workspace = true` and
  `npt-proto.workspace = true` (both are path entries in the workspace's `[workspace.dependencies]`:
  spikes/netcode-proof/Cargo.toml:34 and :37). sha256 f949f2bfbffa972d4cc0e9beae5a1ecbc4430cd4f70a9566e4f4306b427dc25a.
- spikes/netcode-proof/Cargo.lock: npt-client-wasm's dependency array gains ` "npt-link",` and ` "npt-proto",`
  (`git diff --no-index --numstat` against the branch's copy: 2 0; no package, version, checksum or source line).
  sha256 6e6529a86d8c7577ad5905a75c5ff8a157f46eed61ae68b1c1fa530eff5500bc.
- `cargo tree --locked --offline --manifest-path /home/user/Shamble-Frontlines/.gate/u7-amend/spikes/netcode-proof/Cargo.toml
  -p npt-client-wasm -e normal,dev --depth 1` passed (the lock needs no change) and lists npt-client and wasm-bindgen
  as normal edges, npt-link, npt-proto, npt-room, npt-server and npt-sim as dev edges.
It lands on the branch after MAIN-66's push (that take-in binds the spike's tree), applied byte for byte; the
dependency-check.md lines then name these two sha256 values and your answer file.

Questions (each with its evidence):
Q1. May exactly these two files' content be recorded under DEP-HOLD? Any condition?
Q2. Does anything new ship? check.mjs's deps step reads shipped edges only (check.mjs:56-78): confirm from the code that
    R4's deps rule for npt-client-wasm (npt-client and wasm-bindgen only) still holds with these dev edges.
Q3. Is any condition of your security-u7-0b-answer.md affected (its Q1 condition 5 above all: every workspace cargo
    call stays `--locked`)?
Answer each by number, findings on CLAUDE.md's scale, and end the file with DONE on its own line. Your final reply:
one line per question (its answer and any condition).
