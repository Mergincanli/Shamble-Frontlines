# security-auditor: U7 Block 0b amendment (npt-link, npt-proto as dev-deps of npt-client-wasm), DEP-HOLD answer

Date: 2026-11-25. Effort xhigh. Read only; this file is my only write. No Edit tool in this agent: this file is
rewritten whole with Write after each check. Request: u7-0b-amend-security-request.md (this folder).
Nothing installed, fetched, built or run through cargo by me; only reads, `sha256sum`, `git diff --no-index`,
`git rev-parse`, `git ls-tree`, `git status`, `git log`, `git worktree list`, `ls`, `cat`, `sed -n`, `grep`, `find`.
No web.

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
  crates are already in npt-client-wasm's shipped closure and in its dev closure through npt-server and npt-room. The
  amendment adds edges, not crates.
- N6 (the base): `git rev-parse`: HEAD, the remote branch (@{u} = cab139e = HEAD) and c21582d hold the same blobs for
  both files (Cargo.lock e4d5ff35, client-wasm/Cargo.toml f6e0fefc), so N3's diff is against exactly what
  checkpoint.mjs compares with (heldDependencyFiles, checkpoint.mjs:120-140: changed against the remote branch, then
  released by a `<sha256>  <path>` line of the committed blob). `git ls-tree origin/main spikes/` prints nothing
  (origin/main c1da1bd as last fetched): main has no spikes/ folder, so the main-equality exception (CHECKPOINTS.md
  :68-69, checkpoint.mjs:135) cannot release either file. `git status --short -- spikes/netcode-proof`: clean.
- N7 (plan step 6): check.mjs (branch = amend tree: `git diff --no-index --stat` empty). closures() :56-75 follows
  only edges where `shipped(d)` (:59: some dep_kind is null, i.e. normal, or "build"); isShipped :78 the same test;
  dependencyProblems() :88+ builds on closures(). No npt-client-wasm deps rule exists yet: u7-brief.md:126-127 puts it
  in Block R4 ("it ships npt-client and wasm-bindgen only"), not built. advisor-u7-brief.md C6 :167-173 states the same
  reliance ("check.mjs reads shipped edges only (check.mjs:56-78), so R4's deps rule ... holds"). The deps step reads
  `cargo metadata --locked --format-version 1 --all-features` (:2135).
- N8 (check.mjs, every edge read): `grep dep_kinds|.deps` lists :59, :65, :78, :107-110, :121, :128, :136, :141, :153,
  :164, :177. The rules that read edges "in any kind" (dev included) are three: only npt-net may depend on npt-tcpinfo
  (:125-129), nothing depends on npt-hash-wasm (:140-144), nothing depends on npt-lab (:176-180). The new edges point at
  npt-link and npt-proto: none of the three. The rename rule (:169-174) reads npt-proto's and npt-client's manifests
  only; the amendment renames nothing anyway. linkUseProblems (:1007-1039) scans crates/proto/src/ and
  crates/client/src/ only (CHANNEL_ONLY :1011), so the room test's npt_link::{Loopback, Link, ...} in client-wasm's
  tests is outside it by design.
- N9 (cargo's kinds, local primary sources): cargo 1.99.1's cargo-metadata.1 :296-302: dep_kinds "kind" is "dev",
  "build", or null for a normal dependency (what :59 and :78 test). cargo-tree.1 :62-64: "for a mostly equivalent
  overview of what `cargo build` does, `cargo tree -e normal,build` is pretty close". A local record of the same:
  .gate/u7-0b/step4-trees.log :198-230 (Block 0b, 05:43:53 UTC, `cargo tree --locked --offline -p npt-client-wasm -e
  normal --target wasm32-unknown-unknown --prefix none`, exit 0) lists npt-client, npt-link, npt-proto, npt-sim, libm,
  rand_chacha, ppv-lite86, zerocopy, rand_core and wasm-bindgen's stack, and neither npt-server nor npt-room, which were
  already dev edges then (:231-237). So npt-link and npt-proto were in the shipped set before this amendment, and dev
  edges were already absent from the shipped view. security-u7-deps.md:105-110 recorded that no wasm32 step compiles dev
  edges (clippy-wasm32 without --all-targets, the wasm step builds npt-hash-wasm only, build.mjs builds the cdylib).
- N10 (plan step 7): pump.rs :372-378: `pub fn pump(&mut self, link: &mut dyn Link, now_ns: u64, tick: u32, out: &mut
  Vec<(Message, u64)>)`. tests/common/mod.rs :12-18 imports npt_client, npt_link::{Channel, Link, LinkState, Loopback,
  Recv, SendResult}, npt_proto::{Join, Limits, Message, Quant}, npt_room::Room, npt_server::{Decoder, RttSource},
  npt_sim. `grep 'pub use npt_(link|proto)'` over crates/server, room, client src: no re-export. In Rust a crate names
  only its direct dependencies, so a test that builds a Loopback and a Message needs both as direct edges. The need is
  real; the form copies npt-server's own test harness.
- N11 (deny, inferred, not run): deny.toml :12-20: wildcards = "deny", allow-wildcard-paths = true, unknown-registry
  and unknown-git deny. The two new edges have the same form as the three Block 0b edges that passed `cargo deny
  --locked check advisories bans sources` (dependency-check.md:114-115) and as npt-client's normal edges to the same two
  crates: path, no version, in a `publish = false` workspace. No package and no source change, so advisories, sources
  and the duplicate list cannot change; bans' wildcard check reads manifest edges, so it is inferred, not shown: Q1
  condition 4 runs it.
- N12 (scratch tree): .gate/u7-amend/spikes/netcode-proof has no target/ folder (`ls -la`): no build there. Its
  .cargo/config.toml (= branch) sets only `[build] target-dir`.
- N13 (how the lock lines were made): no log for the amendment exists (`find` under .gate/ and this folder: only the
  request, this file and the worktree). The task card (.checkpoints/netcode-proof-test.md:89) says the main session
  made "the same two Edits" (by the Edit tool) and that `cargo tree --locked --offline` "passed, run outside the lane
  runner: read only, no target". So, on the card's account, no unlocked cargo call wrote the lock; the one cargo call
  was `--locked --offline`. That call's output is reported (request :27-29, card :89), not seen by me: no log of it
  exists. cargo-tree.1 :272-284: `--locked` exits with an error when "Cargo attempted to change the lock file due to a
  different dependency resolution", so a pass means cargo accepted the hand-edited lock as its own.
- N14 (dev-edge tests already in check.test.mjs): the metadata fixture builds dep_kinds from [name, kind] pairs
  (:89-104); "a dev dependency does not ship, so it does not count" (:155-161, a leaf with a dev edge to tokio: no
  problem; a build edge: a problem, :162-164); likewise :216, :238, :2050; the any-kind rules are pinned by :192
  (npt-lab), :231 (npt-tcpinfo), :2049 (npt-hash-wasm). So the shipped filter R4 will build on is already tested.
- N15 (who else built): `git worktree list`: the T-builder's worktree is not among this checkout's worktrees (it runs
  elsewhere: card :89, "u7-build-rs.md in its worktree"). Whether it compiled with the two edges before this answer is
  not something I can read; see Not verified.
- N16 (deny's form under the runner): .gate/u7-0b/step6-deny-audit.log: `cargo deny --locked check advisories bans
  sources` ran through the runner (:10) and exited 0 with "advisories ok, bans ok, sources ok" (:718-719). The log's
  only "Fetching advisory database" and "Updating crates.io index" lines (:721, :723) sit inside the cargo audit call
  (:720-735); deny printed no fetch line, though its own database was fetched in that call (05:44:45, per
  dependency-check.md:116-117). Without `advisories`, deny has no database to read; that it then fetches nothing is my
  inference, not seen. Condition 4 keeps its stop rule for that reason.

## Q1. May exactly these two files' content be recorded under DEP-HOLD? YES, with conditions
Why: the change is two featureless dev edges to two path crates of this workspace that npt-client-wasm already ships
through npt-client (N5, N9); the lock gains two lines in one array and nothing else (N3); no package, version, source or
checksum moves; 252 packages before and after (N5); nothing new can be downloaded, built from outside the workspace, or
shipped. The base is exactly what the hold compares with (N6). The need is real (N10).

Conditions:
1. Before applying (after MAIN-66's push), `sha256sum` from the repository root of spikes/netcode-proof/Cargo.toml,
   spikes/netcode-proof/Cargo.lock and spikes/netcode-proof/crates/client-wasm/Cargo.toml prints bd1e2335...f59a,
   55604a5f...9f5d and d8af72f7...000e. If any differs, the amendment is void as prepared: rebuild it on the new base
   and bring the new content here. Never apply the scratch files over a changed base.
2. Applied by the same two Edits (never by cargo, never by copying another file over): afterwards `sha256sum` prints
   f949f2bfbffa972d4cc0e9beae5a1ecbc4430cd4f70a9566e4f4306b427dc25a for crates/client-wasm/Cargo.toml and
   6e6529a86d8c7577ad5905a75c5ff8a157f46eed61ae68b1c1fa530eff5500bc for Cargo.lock; `git diff --numstat` on those two
   prints `2	0` for each; `git status --short -- spikes/` shows those two files only.
3. On the branch tree, through the lane runner (its log header records HEAD and the diff hash), every call `--locked
   --offline`: (a) `cargo tree -p npt-client-wasm -e normal,dev --depth 1`: normal edges npt-client and wasm-bindgen,
   dev edges npt-link, npt-proto, npt-room, npt-server and npt-sim, nothing else; (b) `cargo tree -p npt-client-wasm -e
   normal --target wasm32-unknown-unknown --prefix none`: the same lines as .gate/u7-0b/step4-trees.log :199-229 (the
   shipped set unchanged). If `--locked` refuses, stop and come here; never a refresh by `cargo metadata` without
   `--locked`, `update` or `generate-lockfile`.
4. `cargo deny --locked check bans sources` through the lane runner, as step 6 ran its three-check form (N16): exit 0,
   "bans ok, sources ok", the seven duplicate names of Block 0b's run (step6-deny-audit.log) and no new warning.
   Advisories and cargo audit need no re-run (no package changed). If it tries the network or fails, stop and come
   here; no `CARGO_NET_OFFLINE=false` for it.
5. `node check.mjs --only deps` PASS on the branch tree (its metadata call is `--locked`, check.mjs:2135).
6. dependency-check.md: a new section in the existing shape: `#` lines naming this file (with its sha256) and
   conditions 1-5 with their results; then exactly these two bare lines, each on one line, as `sha256sum` prints them
   from the repository root (full hash, two spaces, path):
   f949f2bfbffa972d4cc0e9beae5a1ecbc4430cd4f70a9566e4f4306b427dc25a  spikes/netcode-proof/crates/client-wasm/Cargo.toml
   6e6529a86d8c7577ad5905a75c5ff8a157f46eed61ae68b1c1fa530eff5500bc  spikes/netcode-proof/Cargo.lock
   No bare line for the workspace Cargo.toml (unchanged, not held). Then the save, which must say "saved and pushed";
   never a push by hand. Block 0b's section (:105-139) stays as written: it records what was true at d8af72f7.
7. Any difference from conditions 1-6 comes back here before the save.

## Q2. Does anything new ship? NO
- From the code: check.mjs's closures() and isShipped (:56-78) keep an edge only when some dep_kind is null (normal) or
  "build" (cargo-metadata.1 :296-302: a dev edge is "dev"). npt-link and npt-proto are dev-only direct edges of
  npt-client-wasm (N3), so they are filtered; they were already in its shipped closure through npt-client (N5, N9).
- The three any-kind rules (:125-129 npt-tcpinfo, :140-144 npt-hash-wasm, :176-180 npt-lab) do not name either crate
  (N8). The wasm32 builds compile no dev edge (N9). The featureless edges turn on no feature (N5).
- R4's rule: it does not exist in check.mjs yet (u7-brief.md:126-127; N7), so "still holds" is a condition on R4, not
  a fact about today's code. Condition (for R4's builder): R4 writes it as npt-hash-wasm's rule is written (:133-139):
  npt-client-wasm's direct edges filtered by isShipped equal exactly {npt-client, wasm-bindgen}; never as an any-kind
  loop (the :125-129, :140-144, :176-180 shape) and never over closures() (which already holds npt-link, npt-proto,
  npt-sim and wasm-bindgen's stack). Its test adds a dev edge that must not trip it, beside :155-161. Written that way,
  Block 0b's three dev edges and these two pass it alike: the amendment does not change R4's question.

## Q3. Is any condition of security-u7-0b-answer.md affected? NO
- Q1 condition 5 (every workspace cargo call stays `--locked`): kept, on the card's account (N13): the lock lines were
  made by the Edit tool and the one cargo call was `--locked --offline` (its output reported, not seen). On the branch,
  Q1 conditions 2-5 above keep every call `--locked`. The run outside the lane runner was read-only with `--offline` of
  its own, the same network stance as the runner's export; condition 3 puts the branch's calls back through the runner
  for the log.
- Q1 conditions 1-4 and 6 (the CLI's fetch, check and install) and Q2 conditions 1-4 (0b's commit and its addendum):
  done and recorded (dependency-check.md:105-139); this amendment touches neither the CLI nor that record.
- Q3 (database dates): untouched; no database is fetched.
- SEC-U7B-1 (Critical, the day guard's env unwrap): unchanged and still open; this amendment neither touches nor relies
  on it. No env, stdbuf or nice option goes in front of git or gh in any condition here.

## What this answer supersedes (security's side only)
- security-u7-deps.md item (7)(b): the dev-dependencies are now exactly npt-link, npt-proto, npt-room, npt-server and
  npt-sim (all `.workspace = true`, no other key); the rest of item (7) stands.
- Item (8): against U6c's merged lock the diff is now `6	0` (the six array lines npt-link, npt-proto, npt-room,
  npt-server, npt-sim, wasm-bindgen); against Block 0b's lock, `2	0`. Its no-package, no-version, no-checksum rule
  stands.
- The binding check's step 4 bullet `cargo tree -p npt-client-wasm -e dev --depth 1`: five names, not three.
- Items (9) and (10): unchanged (no shipped edge, no feature, no package).
- Not mine to re-rule: C6's wording (advisor-u7-brief.md:167-173, three dev-dependencies and the lock "and nothing
  else") is the advisor agent's; the main session takes the departure there (card :93 item 13 already names it).

## Findings (CLAUDE.md's severity scale)
None. No Critical, High, Medium or Low from this read. SEC-U7B-1 stays as security-u7-0b-answer.md left it (open,
Critical, the kit's; unchanged here). Anti-cheat coverage table: not applicable (no player action is in scope).

## Not verified
- How the scratch lock lines were made: by hand with the Edit tool, on the task card's account only (card :89); no log
  of the edits exists. Q1 conditions 1-3 make the branch's copy independent of it (base hash, after hash, `--locked`).
- The scratch `cargo tree --locked --offline` result (no log; reported in the request and the card). Q1 condition 3
  re-runs it on the branch with a log.
- That `cargo deny ... bans` gives the same result (inferred from the same edge form, N11), and that `check bans
  sources` without `advisories` makes no fetch under the runner (N16). Q1 condition 4 runs it and stops on either.
- That the T-builder compiled nothing holding the two edges before this answer (its worktree is not readable here;
  CHECKPOINTS.md :66-67, "Until then build nothing that holds the change"). Nothing from that worktree reaches the
  branch except through the main session's two Edits under conditions 1-2.
- The cargo reference's wording on development dependencies (no web in this task); the local man pages and
  step4-trees.log stand in (N9).

DONE
