# security-auditor: U6c Block 0 lock refresh ruling (the two returns under condition 1)

Reader: security-auditor (Opus 5.5), 2026-11-24. Main folder spikes/netcode-proof, uncommitted, HEAD after U6b's merge
8efb9b2. Inputs: security-u6c-deps.md (my clearance; Conditions, Item 7, Binding check), security-r1-deps.md:228-274,
.gate/u6c-block0/lock.diff (sha256 efb07c46c8d96fc022273e5c224ed9d3a0fcc12c8bb1f718ee5675ba9bf4a070),
step2-metadata.log, step2b-pin.log. Read-only: no cargo that writes the lock or target. (No Edit tool here: this file
is rewritten whole with Write at each block.)

## Plan
1. Confirm the inputs: lock and diff hashes, numstat, package count, manifests' diff.
2. The 17 blocks: names, versions, source, checksums against security-r1-deps.md:258-274.
3. Existing lines changed: the 3 expected re-qualifications, and identify the 3 extra (windows-sys edges).
4. Why the pin moved them; whether the requirement ranges allow it; reach (targets); whether acceptable.
5. Arrays: npt-host +11, npt-bots +4, rcgen + pem 4.0.0, new arrays against the model.
6. The base fact (profile.dev.package.sha2) and the manifest diff against Item 1.
7. Remedy if needed; whether binding-check steps 3-5 run as written (and what to amend).
8. Verdict, findings, DONE.

## Notes
- Step 1: Cargo.lock now sha256 3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873; numstat 229/6;
  252 [[package]]; lock.diff hash equal to the main session's. Modified: Cargo.lock, Cargo.toml, crates/bots,
  crates/host Cargo.toml only. HEAD's lock (`git show HEAD:...Cargo.lock | sha256sum`) = 210bf2df...a219, my
  before-model.
- Step 2: 17 `+checksum` lines, each found in security-r1-deps.md:258-274; 17 `+version`; 17 `+source` all
  `registry+https://github.com/rust-lang/crates.io-index`; `aws-lc|jobserver|cmake|webpki-roots` 0; no "1.13.0" in the
  lock. hyper 1.12.0's cached .crate (~/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/hyper-1.12.0.crate)
  sha256 2c3e324d...7c9c = the cleared checksum.
- Step 3: the six `-` lines: ` "base64",` x2 (pem 3.0.6, sfv 0.15.0), ` "pem",` x1 (wtransport 0.7.2), the three I
  modelled; plus ` "windows-sys 0.61.2",` x3 -> ` "windows-sys 0.52.0",` in quinn-udp 0.5.16 (lock :1266),
  rustls-platform-verifier 0.7.1 (:1464), winapi-util 0.1.11 (:2150). Before, only ring 0.17.14 named 0.52.0; now four
  do, six still name 0.61.2 (generator, mio, nu-ansi-term, schannel, socket2, tokio); both windows-sys blocks remain.
- Decisive check: the current lock with those three lines put back to 0.61.2 (in memory, `node -e`, no write) hashes to
  ea6efb003a59f06946c5ba48f3ba73ec1c92d79757e07f6a7ab5b79610a79111, my model exactly. So every other line (the 17
  blocks, their arrays, the re-qualified arrays, npt-host +11, npt-bots +4, rcgen + "pem 4.0.0", pem 4.0.0's
  ["base64 0.23.1", "serde_core"], instant-acme's "base64 0.22.1") is as modelled, and the three edges are the only
  difference.
- Step 4: requirements (REG manifests): quinn-udp `[target."cfg(windows)"] windows-sys ">=0.52, <=0.61"` (:95-96);
  rustls-platform-verifier `cfg(windows)` ">=0.52.0, <0.62.0" (:141-142); winapi-util `cfg(windows)` ">=0.48.0,
  <=0.61.*" (:47-48). Both versions satisfy each; all three are Windows-target-only edges, never compiled for our Linux
  server or wasm32 client. Cause: `cargo update -p hyper --precise` re-resolved and picked the other already-locked
  version for these ranged requirements (the pin log prints only the hyper downgrade: edges are not versions). Cause:
  medium confidence; fact and reach: high. OSV 2026-11-24T06:28:04Z: hyper 1.12.0, 1.13.0, windows-sys 0.52.0, 0.61.2,
  quinn-udp 0.5.16, rustls-platform-verifier 0.7.1, winapi-util 0.1.11: none.
- hyper 1.13.0: a release after my 2026-11-22 index read, cleared by no read; the pin-back to 1.12.0 is the route my
  binding check step 2 names. The pin lives in the lock only, which holds because every cargo call in check.mjs runs
  `--locked` (check.mjs:1686, :2015, :2017, :2033-2034, :2053, :2057, :2135, :2205-2206).
- Step 5: arrays read in lock.diff: npt-host +bytes, http, http-body-util, httparse, instant-acme, rcgen, serde,
  serde_json, sha2, tokio, tokio-rustls (+11); npt-bots +npt-server, serde, serde_json, tokio (+4); rcgen + "pem 4.0.0"
  before "ring". As Item 7.
- Step 6: manifest diff = Item 1 (a)-(c) exactly, plus SEC-U6CD-2's comment (worded as I proposed) and a comment block
  over the five entries; no features/optional/profile/patch key added. The base fact `[profile.dev.package.sha2]
  opt-level = 3` (HEAD Cargo.toml:122-123) is U6b's merged base, not Block 0's diff: optimisation level only, no lock
  line, no feature, no security reach. Accepted; condition 1 (d) concerns Block 0's edits and holds.

## Ruling 1: every difference from my model
- (a) hyper 1.13.0 at the first refresh: not acceptable as is (unread); remedied already by the pin-back, now 1.12.0
  with the cleared checksum. Acceptable.
- (b) three `cfg(windows)` edges moved windows-sys 0.61.2 -> 0.52.0 (quinn-udp, rustls-platform-verifier,
  winapi-util): no package, version, checksum, source or feature added or removed; within each crate's stated range;
  never compiled on our targets. Acceptable.
- Nothing else differs (the in-memory revert reproduces my model's hash).

## Ruling 2: remedy
- None needed. No further `cargo update`, no manifest pin, no other hyper version. Do not try to move the three edges
  back (another `update -p` re-resolves edges again and proves nothing more). The after-lock is
  3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873.

## Ruling 3: steps 3-5 of the binding check, amended where the lock moved
- Step 3 runs with three amendments, the rest as written: numstat `229	6` (not 226/3); `git diff -U0 -- Cargo.lock |
  grep '^-[^-]'` prints exactly six lines, ` "base64",` x2, ` "pem",` x1 and ` "windows-sys 0.61.2",` x3; `sha256sum
  Cargo.lock` = 3394b451...4873. Add: `grep -c '"windows-sys 0.52.0"' Cargo.lock` = 4 and `grep -c '"windows-sys
  0.61.2"' Cargo.lock` = 6; `grep -A1 '^name = "hyper"$' Cargo.lock` shows version 1.12.0.
- Step 4 runs as written (Linux-target trees do not see the edges; the `--target all -d --depth 0` list keeps the same
  seven names: windows-sys still has two versions).
- Steps 5 and 6 run as written (cargo audit still 252 crate dependencies; cargo-deny's seven multiple-versions names
  unchanged; only windows-sys's parent listing differs).
- Step 7: dependency-check.md's section names this file too, and records the hyper pin-back (`cargo update -p hyper
  --precise 1.12.0`, step2b-pin.log) and the three windows-sys edges.

## Findings
- **SEC-U6CL-1 (Low, evidence note):** the after-lock differs from my modelled hash by three Windows-only dependency
  edges introduced by the hyper pin-back. Exploit sketch: none (no package, version or checksum change; never compiled
  on our targets); the risk is only a reviewer finding a binding check whose numbers do not hold. Evidence:
  lock.diff:272-278, :289-295, :363-369; REG quinn-udp-0.5.16/Cargo.toml:95-96, rustls-platform-verifier-0.7.1/
  Cargo.toml:141-142, winapi-util-0.1.11/Cargo.toml:47-48. Fix: step 3 amended above; dependency-check.md records it.
- **SEC-U6CL-2 (Low, note):** hyper 1.13.0 is newer than the cleared 1.12.0 and is held back by the lock alone. Exploit
  sketch: none while every call is `--locked`; a later unlocked `cargo update` would take an unread version. Evidence:
  step2-metadata.log ("Adding hyper v1.13.0"); check.mjs `--locked` lines above. Fix: none now; any move to 1.13.x
  comes back for a read (with its checksum against the index).
- No Critical, High or Medium. Earlier SEC-U6CD-1 to -4 and Conditions 2-5 of security-u6c-deps.md stand unchanged.

## Verdict: CLEAR WITH CONDITIONS
The lock 3394b451...4873 is cleared for U6c's Block 0 save on: binding check steps 3 (as amended in Ruling 3) to 7
passing; security-u6c-deps.md Conditions 2-5 unchanged. Not verified: the exact cargo resolver rule that flipped the
three edges (medium confidence), and steps 4-6's outputs (the main session runs them).

DONE

## Addendum (2026-11-24, after step 4): three feature lines outside my lists
Input: .gate/u6c-block0/step4-tree.log :11-35 (06:30:49 UTC, `--locked --offline`, lock 3394b451...4873).
Plan: read each crate's [features] table and its dependents' entries in REG; rule; amend step 4's lists; step 7.

- (1) hyper `default`: REG hyper-1.12.0/Cargo.toml:76 `default = []`. Turned on because instant-acme 0.8.5's
  `[dependencies.hyper]` (its Cargo.toml:136-143) has no `default-features = false` (hyper-rustls's has: :90-92).
  An empty feature: enables no code and no dependency. Still no `server`. Acceptable.
- (2) hyper-util `default`: REG hyper-util-0.1.21/Cargo.toml:81 `default = []`; same cause, instant-acme's
  `[dependencies.hyper-util]` (:157-166) keeps defaults (hyper-rustls's turns them off: :94-100). Empty; no server,
  server-auto, server-graceful, client-proxy or client-pool. Acceptable.
- (3) hyper-rustls `rustls-native-certs`: REG hyper-rustls-0.27.10/Cargo.toml:75 `native-tokio =
  ["rustls-native-certs"]`, and :110-112 an optional dependency of that name: cargo lists the implicit feature an
  optional dependency makes. instant-acme asks `native-tokio` (:145-150), which I had listed; my list missed its
  implied line (my error, not a change). Reach: no new package (rustls-native-certs 0.8.5 is locked and already
  compiled on Linux under rustls-platform-verifier, which uses it for its roots: security-u6c-deps.md item 5). It
  compiles hyper-rustls's `with_native_roots` builder; our route stays `Account::builder()` -> the platform verifier
  (Condition 2), which reads the same native roots, so the trust store and SEC-U6CD-4's environment condition are
  unchanged. Not `webpki-tokio` / webpki-roots (grep 0). Acceptable.
- Ruling: all three acceptable as is; no remedy, nothing reruns because of them.
- Step 4 amended lists: hyper client, default, http1, http2; hyper-util client, client-legacy, default, http1, http2,
  tokio; hyper-rustls http1, http2, native-tokio, ring, rustls-native-certs, rustls-platform-verifier, tls12. The rest
  of step 4 as written (and matched, per the main session).
- Step 7: yes, dependency-check.md names them: the three amended lists as the recorded expectation, with this
  addendum as the source, so the record matches step4-tree.log.
- Finding SEC-U6CL-3 (Low, evidence note): my item 6 / step 4 lists omitted two empty `default` features and one
  implied optional-dependency feature. Exploit sketch: none (no code or package added); only a binding check whose
  lists do not hold. Evidence: the REG lines above. Fix: the amended lists.
- Verdict unchanged: CLEAR WITH CONDITIONS (no Critical, High or Medium).

DONE
