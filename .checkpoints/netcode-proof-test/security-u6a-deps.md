# security-auditor: U6a Block 0 (b) dependency read (DEP-HOLD), 2026-11-22

Branch claude/netcode-proof-test-plan-hcky5g at d7dc0ce; Cargo.lock last changed at 4e91830 (2026-11-13).
Subject: u6a-brief.md rev 1, Block 0 (b) (section 5): npt-server gains tokio, sha2, serde, serde_json (workspace
pins); npt-super gains sha2. Read only: no install, no build, no cargo run. (No Edit tool in this agent: the file is
rewritten whole with Write after each block.)

## Plan
1. Each crate's requested features vs what the lock and the workspace already resolve; any optional dependency a
   newly enabled feature would add to the lock.
2. Versions to pin, matching the lock.
3. Advisories on those exact versions (sources).
4. What Block 0 (b)'s uses widen at the network or filesystem edge (settings JSON, token file, stdin, child spawn).
5. Verdict per crate; the binding lock-diff check for U6a's save.

## Notes (saved as I go)

### 1. Features asked vs resolved (DONE)
Sources: spikes/netcode-proof/Cargo.toml [workspace.dependencies]; crates/*/Cargo.toml; Cargo.lock; the crates'
own manifests in the local cargo registry cache (read only: ~/.cargo/registry/src/index.crates.io-*/).
- The brief asks for the workspace entries (u6a-brief.md:196-198, "its features as set"), no `features = [...]`, no
  `default-features` key. So each gets exactly the workspace entry:
  - tokio `=1.54.1`, features rt-multi-thread, net, time, sync, io-util, macros (tokio's default = []). npt-net
    already takes `tokio.workspace = true` (crates/net/Cargo.toml:25) and npt-server already depends on npt-net, so the
    npt-server build's tokio feature set is unchanged: zero new features, zero new optional deps. Lock entry deps
    (bytes, libc, mio, pin-project-lite, socket2, tokio-macros, windows-sys 0.61.2) = what those six features pull.
  - sha2 `=0.11.1`, default features (alloc, oid -> digest/alloc, digest/oid). npt-lab already takes
    `sha2.workspace = true` (crates/lab/Cargo.toml:35), same set; digest 0.11.3's lock entry already has const-oid.
  - serde `=1.0.229` with derive (+ default std): npt-log and npt-judge already take it; serde_derive is in the lock.
  - serde_json `=1.0.154`, default (std): npt-log, npt-lab, npt-judge already take it; lock deps itoa, memchr, serde,
    serde_core, zmij. No manifest in the workspace turns on unbounded_depth, arbitrary_precision or preserve_order.
- Feature trap checked (the caller's example), tokio 1.54.1's [features]: `signal` and `process` add
  signal-hook-registry (NOT in the lock: tokio's lock entry lists none; grep of the lock for signal-hook-registry,
  io-uring, digest-io: 0); `io-uring` adds io-uring and slab; `taskdump` backtrace; `parking_lot` parking_lot. None is
  asked. Watch item: Block 3 ends on "stdin closing (no tokio `signal` feature)" and Block 6 spawns and kills
  children. tokio's stdin would need `io-std` (= [], no package, but a manifest change) and tokio's child processes
  `process` (adds signal-hook-registry). Neither is in the brief, and a manifest change by the builder stops it
  (u6a-brief.md:29-32), so std::io::stdin on a plain thread and std::process are the only routes. SEC-U6AD-1.
- Second trap: digest 0.11.3 has no `std` feature and no `std::io::Write` impl (its lib.rs:32 points to the separate
  `digest-io` crate). Hashing the binary with `io::copy` into the hasher would need digest-io, a NEW crate. SEC-U6AD-5.
- Packages the real cargo change adds: NONE. Each of the four has a single version in the lock (no "name version"
  form needed in the dependency arrays; syn has two versions, but it is not a direct edge here).
- Cargo.lock still changes: it lists each workspace member's own `dependencies` array (dev ones too: npt-server's holds
  npt-client today). Expected: npt-server + "serde", "serde_json", "sha2", "tokio" (sorted after "npt-sim");
  npt-super + "sha2" (after "npt-log"). 5 lines added, 0 removed. The brief's "no package, version or checksum" and
  "only npt-server's and npt-super's dependency lists" hold for the lock too.
- check.mjs effects (read, not run): npt-super's rule (check.mjs:98-100) bans npt-sim/room/proto in its closure; sha2's
  closure (cfg-if, cpufeatures, digest, block-buffer, const-oid, crypto-common, hybrid-array, typenum) holds none.
  npt-server is not a LEAF (check.mjs:39), so tokio there breaks no transport rule. SIM_BUILDS (check.mjs:1677-1681)
  compares npt-sim's subtree features across npt-server, npt-lab, npt-hash-wasm: none of the four sits in npt-sim's
  subtree (libm, rand_chacha, rand_core), and npt-lab already pulls npt-server's closure plus sha2 and serde_json, so
  no new difference is expected; the deps step proves it at the build.

### 2. Versions to pin (DONE)
All through the workspace entries, which equal the lock: tokio 1.54.1 (checksum b52fd325...), sha2 0.11.1
(47d7069b...), serde 1.0.229 (4148590a...), serde_json 1.0.154 (e7e9cc8b...). In the two crate manifests no version
string of their own: `tokio.workspace = true`, `sha2.workspace = true`, `serde.workspace = true`,
`serde_json.workspace = true`. A local `version =`, `features =` or `default-features =` is a deviation (SEC-U6AD-1).

### 3. Advisories (DONE)
Source: OSV (api.osv.dev /v1/querybatch, ecosystem crates.io; OSV imports RustSec and GHSA), queried 2026-11-22
13:45 UTC, on the four and their locked closures (26 package-versions: tokio 1.54.1, sha2 0.11.1, serde 1.0.229,
serde_json 1.0.154, cfg-if 1.0.5, cpufeatures 0.3.1, digest 0.11.3, block-buffer 0.12.1, const-oid 0.10.2,
crypto-common 0.2.2, hybrid-array 0.4.15, typenum 1.20.1, itoa 1.0.18, memchr 2.8.3, serde_core 1.0.229, zmij 1.0.23,
serde_derive 1.0.229, proc-macro2 1.0.107, quote 1.0.47, syn 3.0.9, bytes 1.12.1, libc 0.2.191, mio 1.2.4,
pin-project-lite 0.2.17, socket2 0.6.5, tokio-macros 2.7.2). Result: none affecting any of them.
Control query (an empty answer is not a failure): OSV's tokio list returns 13 entries; the newest,
RUSTSEC-2026-0384 / GHSA-5wjr-9356-4wc8 / CVE-2026-115567 ("Race condition in tokio::time::Interval on Tokio v1.54.0"),
introduced 1.54.0, fixed 1.54.1: our pin is the fixed version. RUSTSEC-2026-0386 (AsyncFd I/O safety): ranges end at
1.51.6 and 1.53.3, not 1.54.x. Agrees with security-r1-deps.md's whole-lock OSV sweep today (2 hits: url 2.5.8 and
pin-utils 0.1.1, neither among these). Not checked: the RustSec repo directly (GitHub API 403 earlier today); cargo
deny and cargo audit are the main session's Block 0 evidence. Confidence: high.

### 4. What the uses widen (DONE)
Read: u6a-brief.md:36-84 (Blocks 2, 3, 4, 6); server/src/main.rs:1-6 and super/src/main.rs (both stubs today);
room/src/roster.rs:1-17, :92-103 (tokens compared in constant time already; minting is U6's).
- Network: U6a adds no transport (brief section 2: WtServer/WsServer are U6b's). tokio enters npt-server for the I/O
  runtime over in-process links only. Nothing network-facing is widened IF U6a binds no socket. serde_json's only
  input is the operator's settings file; client bytes keep going through npt-proto's binary Decoder. SEC-U6AD-4.
- Filesystem, settings (operator input, not a client): serde_json's default recursion limit (128) stays on; the brief's
  size cap must bound the READ, not the parsed length (read_to_string first allocates whatever the path holds: a FIFO
  or /dev/zero never ends). deny_unknown_fields is silently void under #[serde(flatten)]. SEC-U6AD-2 (Low if missed).
- Filesystem, token file (a secret): 0600 by chmod after write leaves a window, and a plain create follows a planted
  symlink or reuses an existing file. SEC-U6AD-3 (Medium if missed: the tokens are the Join secrets U6b will check).
- Filesystem, binary hash: /proc/self/exe (or the given path) hashed in a fixed buffer, never read whole. SEC-U6AD-5.
- Processes, npt-super: spawns the room binary, reads its stdout to `OPEN`, kills at 10 s. A child line read with
  read_line has no length bound; Command must take an argument vector, no shell; a killed child must be waited for.
  SEC-U6AD-6 (Low: the child is our own binary).
- DEP-HOLD scope note: any Cargo.toml change is held, a [profile] key too. The tick-thread panic catch needs
  `panic = "unwind"`, which is the default (no `panic =` key in any manifest today), so Block 2 needs no profile edit;
  npt-super's fake child (Block 6 tests) must also come without a manifest change (e.g. the test binary re-running
  itself), else it is a later held save.

### 5. Verdicts and conditions (DONE)
- **npt-server (tokio, sha2, serde, serde_json): CLEAR WITH CONDITIONS** (SEC-U6AD-1 to -5).
- **npt-super (sha2): CLEAR WITH CONDITIONS** (SEC-U6AD-1, -5, -6).

Conditions (each a finding at the build review if broken; severity is the one it would carry then):
- SEC-U6AD-1 (Medium): exactly `tokio.workspace = true`, `sha2.workspace = true`, `serde.workspace = true`,
  `serde_json.workspace = true` in crates/server/Cargo.toml [dependencies] and `sha2.workspace = true` in
  crates/super/Cargo.toml [dependencies]; no `features`, `default-features`, `optional` or version key; the workspace
  Cargo.toml unchanged. No tokio io-std, process, signal, fs, io-uring, parking_lot or taskdump: stdin by std on its
  own thread, children by std::process.
- SEC-U6AD-2 (Low): settings read through `File::take(CAP + 1)` (or equal), refused above CAP before parsing; the
  settings types `#[serde(deny_unknown_fields)]` with no `flatten`, `untagged` or `serde_json::Value` field; every
  number range-checked before it sizes anything (the client count before any token or slot vector).
- SEC-U6AD-3 (Medium): token file by `OpenOptions::new().write(true).create_new(true).mode(0o600)` (std's
  OpenOptionsExt, no new crate), never a chmod afterwards; an existing path is refused (exit 2); a short or failed
  /dev/urandom read is an exit, never a fallback; no token in any log, stdout line, panic message or Debug output (the
  settings and token types carry no Debug, as roster.rs's Entry); the run script reads it, nothing else.
- SEC-U6AD-4 (Medium): U6a binds and listens on no socket; no serde_json parse of bytes from a link, session or
  socket in npt-server or npt-super.
- SEC-U6AD-5 (Low): SHA-256 of a binary by a loop of `Read::read` into a fixed buffer and `Digest::update`; no
  digest-io or any other new crate; the whole file never read into memory.
- SEC-U6AD-6 (Low): npt-super's `Command` takes the program path and arguments as separate values, no `sh -c`; the
  child's stdout read with a byte bound per line (e.g. `take`); a child past the deadline killed and then waited for,
  its exit recorded; the child's stderr not copied into npt-super's log unbounded.

## Binding check for U6a's save (the main session)
Before (this read, at d7dc0ce): sha256 Cargo.toml 5ebfbedf...0d36a, Cargo.lock 7c551e2b...eb579,
crates/server/Cargo.toml 4a9d75c1...a136, crates/super/Cargo.toml 2f0c76bd...6b64; lock 242 [[package]] entries.
1. Lock diff (a), on its own first: exactly url 2.5.8 -> 2.5.9 and form_urlencoded 1.2.2 -> 1.2.3 (their version and
   checksum lines), nothing else (security-r1-deps.md:249-253's check; anything more goes back to me).
2. Then the manifest edits of SEC-U6AD-1 and the lock refreshed OFFLINE (e.g. `cargo metadata --offline
   --format-version 1`, or `cargo check --offline` without --locked; never `cargo update` without `-p`, never
   `generate-lockfile`). Offline makes a needed new package fail loudly instead of downloading it.
3. Lock diff (b), against the lock after (a), must be exactly these 5 added lines and nothing else:
   in npt-server's entry, after ` "npt-sim",`: ` "serde",` ` "serde_json",` ` "sha2",` ` "tokio",`;
   in npt-super's entry, after ` "npt-log",`: ` "sha2",`. 0 lines removed; still 242 [[package]] entries; no
   version, checksum or source line changed; grep of the lock for signal-hook-registry, io-uring, digest-io: 0.
4. `cargo tree --locked --offline -e features -p npt-server -i tokio` shows tokio's features as exactly
   rt-multi-thread, rt, net, time, sync, io-util, macros (and `default` absent or empty), the same as for
   `-p npt-net`; `cargo tree --locked --offline -e normal -p npt-super` holds no npt-sim, npt-room, npt-proto.
5. `cargo deny --locked check advisories bans sources` and `cargo audit` on the final lock, clean, with the advisory
   database's date; dependency-check.md lists the final hashes of Cargo.toml, Cargo.lock, crates/server/Cargo.toml and
   crates/super/Cargo.toml, naming security-r1-deps.md for (a) and this file for (b).
Anything outside 1-4 (another package, a feature change, the workspace Cargo.toml touched) comes back to me first.

DONE
