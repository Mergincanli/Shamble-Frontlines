# security-auditor: U7 Block 0b amendment (npt-link, npt-proto as dev-deps of npt-client-wasm), DEP-HOLD answer

Date: 2026-11-25. Effort xhigh. Read only; this file is my only write. No Edit tool in this agent: this file is
rewritten whole with Write after each check.

## Plan
1. Read DEP-HOLD rule (.claude/CHECKPOINTS.md :60-69), my earlier answer (security-u7-0b-answer.md), dependency-check.md,
   security-u7-deps.md items (7)-(10) and the binding check.
2. Hash both amended files in .gate/u7-amend; compare to the stated sha256 values.
3. Diff the amended Cargo.toml and Cargo.lock against the branch's copies (content, not just numstat).
4. Read workspace Cargo.toml (:34, :37): confirm npt-link and npt-proto are path entries, no registry/git source.
5. Read npt-link and npt-proto manifests and lock blocks: confirm they add nothing third-party to the lock.
6. Read check.mjs :56-78 (deps step): confirm it reads shipped (normal) edges only and the R4 rule for npt-client-wasm.
7. Read pump.rs :372-378 and tests/common/mod.rs to confirm the stated need.
8. Re-check my earlier conditions (Q1 condition 5: --locked) against the amendment.
9. Answer Q1-Q3, findings on CLAUDE.md scale, DONE.

## Notes (one line per check)
- N1 (plan step 1): CHECKPOINTS.md :60-69 read: the hold binds content by sha256 per file, released by a
  dependency-check.md line naming the answer. dependency-check.md :105-139 read: Block 0b's three lines (:119-121,
  client-wasm/Cargo.toml d8af72f7...000e, lock 55604a5f...9f5d) and the CLI addendum (:122-139, "every workspace cargo
  call stays --locked (Q1 condition 5)"). security-u7-0b-answer.md read in full: Q1 conditions 1-6, Q2 conditions 1-4,
  Q3, SEC-U7B-1 (open, Critical, the guard's env unwrap). security-u7-deps.md item (7) :175-182 ("exactly" three
  dev-dependencies; "Nothing else in either file"), item (8) :184-190, item (9) :192-199, binding check :281-316 read.
- N2 (plan step 2): `sha256sum`, read now: .gate/u7-amend crates/client-wasm/Cargo.toml f949f2bf...dc25a and
  Cargo.lock 6e6529a8...00bc, both equal to the request. The branch's copies: client-wasm/Cargo.toml d8af72f7...000e and
  Cargo.lock 55604a5f...9f5d, equal to dependency-check.md:120-121 (Block 0b's cleared content). The workspace
  Cargo.toml is bd1e2335...f59a in both trees (equal to :119): untouched by the amendment.
- N3 (plan step 3): `git diff --no-index` branch -> amend. Manifest: one hunk, +2 -0, `npt-link.workspace = true` and
  `npt-proto.workspace = true` at the top of [dev-dependencies], nothing else. `cat -A`: LF endings, no stray bytes;
  [lints], [lib] crate-type ["cdylib","rlib"] and [dependencies] (npt-client, wasm-bindgen) unchanged. Lock: one hunk at
  :907-916, +2 -0, ` "npt-link",` and ` "npt-proto",` in npt-client-wasm's array (alphabetical, after ` "npt-client",`);
  no [[package]], name, version, source or checksum line.
- N4 (plan step 4): workspace Cargo.toml (amend tree, = branch by hash): :34 `npt-link = { path = "crates/link" }`,
  :37 `npt-proto = { path = "crates/proto" }`: path entries, no version, registry, git or features key. Both are members
  (:7, :10). No [patch] or [replace] in the workspace manifest (read :1-143).
- N5 (plan step 5): lock blocks (amend tree): npt-link :980-981, no dependencies; npt-proto :1010-1017, npt-link,
  npt-sim, rand_chacha, rand_core. `grep -c '^\[\[package\]\]'`: 252 in both trees. Manifests: crates/link/Cargo.toml
  no [features], [dependencies] empty by rule (:12-13); crates/proto/Cargo.toml [features] `bites = ["npt-sim/bites"]`
  only, no `default` (:12-15). So a featureless `.workspace = true` edge turns on no feature, even in test builds.
  crates/client/Cargo.toml :20-22: npt-link, npt-proto, npt-sim are already npt-client's normal dependencies, so both
  crates are already in npt-client-wasm's shipped closure (security-u7-deps.md:108 lists them) and in its dev closure
  through npt-server and npt-room. The amendment adds edges, not crates.
- N6 (the base): `git rev-parse`: HEAD, the remote branch (@{u} = cab139e = HEAD) and c21582d hold the same blobs for
  both files (Cargo.lock e4d5ff35, client-wasm/Cargo.toml f6e0fefc), so the diff in N3 is against exactly what
  checkpoint.mjs compares with. origin/main has no spikes/netcode-proof (`fatal: ... not in 'origin/main'`), so the
  hold's main-equality exception (CHECKPOINTS.md :68-69) cannot release either file, and main's take-in cannot carry a
  different copy of them. `git status --short -- spikes/netcode-proof`: clean.
- N7 (plan step 6): check.mjs (branch = amend tree: `git diff --no-index --stat` empty). closures() :56-75 follows
  only edges where `shipped(d)` (:59: some dep_kind is null, i.e. normal, or "build"); isShipped :78 the same test;
  dependencyProblems() :88+ builds on closures(). No npt-client-wasm deps rule exists yet: u7-brief.md:126-127 puts it
  in Block R4 ("it ships npt-client and wasm-bindgen only"), not built. advisor-u7-brief.md C6 :167-173 states the same
  reliance ("check.mjs reads shipped edges only (check.mjs:56-78), so R4's deps rule ... holds").
- N8 (check.mjs, every edge read): `grep dep_kinds|.deps` lists :59, :65, :78, :107-110, :121, :128, :136, :141, :153,
  :164, :177. The rules that read edges "in any kind" (dev included) are three: only npt-net may depend on npt-tcpinfo
  (:125-129), nothing depends on npt-hash-wasm (:140-144), nothing depends on npt-lab (:176-180). The new edges point at
  npt-link and npt-proto: none of the three. The rename rule (:169-174) reads npt-proto's and npt-client's manifests
  only; the amendment renames nothing anyway. linkUseProblems (:1007-1039) scans crates/proto/src/ and
  crates/client/src/ only (CHANNEL_ONLY :1011), so the room test's npt_link::{Loopback, Link, ...} in client-wasm's
  tests is outside it by design.
- N9 (cargo's kinds, local primary sources): cargo 1.99.1's cargo-metadata.1 :296-302: dep_kinds "kind" is "dev",
  "build", or null for a normal dependency (what :59 and :78 test). cargo-tree.1 :62-64: "for a mostly equivalent
  overview of what `cargo build` does, `cargo tree -e normal,build` is pretty close". The cargo reference's "Development
  dependencies" section (not re-read today, no web in this task: from memory) says they are used for tests, examples
  and benchmarks, not for building the package; security-u7-deps.md:105-110 recorded that no wasm32 step compiles dev
  edges (clippy-wasm32 without --all-targets, the wasm step builds npt-hash-wasm only, build.mjs builds the cdylib).
- N10 (plan step 7): pump.rs :372-378: `pub fn pump(&mut self, link: &mut dyn Link, now_ns: u64, tick: u32, out: &mut
  Vec<(Message, u64)>)`. tests/common/mod.rs :12-18 imports npt_client, npt_link::{Channel, Link, LinkState, Loopback,
  Recv, SendResult}, npt_proto::{Join, Limits, Message, Quant}, npt_room::Room, npt_server::{Decoder, RttSource},
  npt_sim. `grep 'pub use npt_(link|proto)'` over crates/server, room, client src: no re-export. Edition 2024: a crate
  names only its direct dependencies, so a test that builds a Loopback and a Message needs both as direct edges. The
  need is real; the form copies npt-server's own test harness.
- N11 (deny): deny.toml :12-20: wildcards = "deny", allow-wildcard-paths = true, unknown-registry and unknown-git deny.
  The two new edges have the same form as the three Block 0b edges that passed `cargo deny --locked check advisories
  bans sources` (dependency-check.md:114-115) and as npt-client's normal edges to the same two crates: path, no
  version, inside a `publish = false` workspace. No package and no source change, so advisories, sources and the
  duplicate list cannot change. Not re-run (task: no cargo).
- N12 (scratch tree): .gate/u7-amend/spikes/netcode-proof has no target/ folder (`ls -la`): no build there.
