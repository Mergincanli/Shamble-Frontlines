# security-auditor: answers to U7's Block 0b questions (u7-0b-security-request.md)

Reader: security-auditor (Opus 5.5), 2026-11-25. Branch claude/netcode-proof-test-plan-hcky5g (main folder, not
switched). Inputs read: u7-0b-security-request.md, u7-block0b.md, dep-check-0b-draft.md, every log under .gate/u7-0b/
(before, step2-metadata, step2-metadata-2, step3b-deps, step4-trees, step5-after, step6-deny-audit, step7-cli-offline),
my read security-u7-deps.md (items (7)-(10), binding check :281-316), security-r1-deps.md B4 (:104-123) and its pins
(:234-237, :276-277), security-u5-deps.md, u5-deps-facts.md and u6c-block0.md (precedents), .claude/CHECKPOINTS.md §3
(:60-69), .claude/scripts/checkpoint.mjs (:17-24, :120-140), main-run.sh, u7-run.sh, cargo 1.99.1's man pages
(cargo-install.1, cargo-info.1), .claude/hooks/routine-guard.mjs (equal to origin/main's: last change 08423ce).
Nothing installed, fetched or built; only reads, `git diff`/`git log`, `sha256sum`, `ls` and `grep`. Web pages read:
the cargo book's registry index page and its configuration page. No Edit tool in this agent: this file is rewritten
whole with Write at each block.

## Q2. May step 8's commit go before step 7? Verdict: (a), with four conditions
Amended after the first write; the verdict is unchanged. Checked against the step 4, 5, 6 and before logs and against
today's `git diff HEAD -- spikes/netcode-proof | sha256sum` = 0b5b2289...7c, the hash in the step 3b, 4, 5, 6 and 7 log
headers: the reasons stand. The conditions, reworded once Q1 was answered:
- Condition 1 now names Q1 (b) and gives each `<answer>`. The first version said only "under ... Q1".
- Condition 2's addendum list now includes Q1's checks.
- Condition 3 now asks for all of Q1's checks to have passed, and sends any failure to Q1's stop rule. The first
  version asked only for the `--version` line and the .crate sha256.
- Condition 4 is unchanged.

Why it is safe:
- The hold binds content, not order. checkpoint.mjs:120-140 releases a held file only when its sha256 in the pushed
  commit equals a `<sha256>  <path>` line of that commit's dependency-check.md (CHECKPOINTS.md §3, :60-69). A later
  change to any of the three files (for example a new wasm-bindgen version, should the CLI fail) gets a new hash and is
  held again until a new answer. So releasing now cannot let a later, different manifest through.
- Today's files equal the draft's three lines (`sha256sum`, read now): Cargo.toml bd1e2335...f59a, Cargo.lock
  55604a5f...9f5d, crates/client-wasm/Cargo.toml d8af72f7...000e. `git diff` shows exactly item (7)'s lines (+3; +6 -1)
  and item (8)'s four lock lines in npt-client-wasm's block, and nothing else under spikes/.
- Step 7 changes no repository file: the CLI is a binary under .gate/ (`--root`), and its download lands in
  ~/.cargo/registry. Nothing the hold guards depends on it. R1-R4 build the wasm-bindgen crate 0.2.130, which is already
  locked with its checksum (Cargo.lock:2045 a0181c14...3d1c, equal to the cached .crate's sha256, read now, and to the
  index per security-u7-deps.md's step 5 notes) and was cleared by items (7)-(10).
- Steps 0-6 and the redact step passed as written (u7-block0b.md; the logs read: step 4's five calls exit 0 with
  wasm-bindgen's features `default` and `std` only and the 20 names; step 5's `cargo check` 0 warning lines, 861/958
  before and after; step 6 "advisories ok, bans ok, sources ok", the seven duplicate names, 252 crate dependencies,
  "1 allowed warning found" RUSTSEC-2026-0441).
- The order 7-then-8 in my binding check (:313-316) was a sequence, not a dependency: I did not foresee that the CLI's
  .crate would not be cached. Moving 8 before 7 is a difference from (7)-(10) in order only; C10 brings it here, so this
  answer is the ruling.

Conditions:
1. The section says plainly that step 7 is open: in the draft's last header line, "The CLI (step 7): not yet
   installed; under security-u7-0b-answer.md Q1 (b); recorded in an addendum below after its install". It names this
   file with each `<answer>` filled: Q1 (b) the .crate fetched by `cargo info` and its sha256 checked before the
   install; Q2 (a) this commit before the CLI; Q3 (b) fetch times only.
2. The addendum after the install is `#` lines only (the CLI's `--version` line, the .crate's sha256 and path, the
   log's name, the count of crates downloaded, Q1's checks). Its sha256 goes in a `#` line, never in a bare
   `<sha256>  <path>` line, so the record's only bare lines stay the repository files the hold reads (the parser
   ignores other lines, checkpoint.mjs:126, but the PR review reads the record).
3. Nothing runs the CLI (build.mjs, T5 (ii), any `wasm-bindgen` call) until that addendum is committed with Q1's
   checks passed. If the install fails or a check differs, Q1's stop rule applies and it comes here before any use;
   the manifests and lock stay as committed unless a new answer says otherwise.
4. The draft's order otherwise stands: re-hash equal to the three lines and `git status --short -- spikes/` those three
   only, the gate PASS, the commit, the section by Edit, then the save, which must say "saved and pushed" (never a
   push by hand).

Findings for Q2: none.

## Q1. Step 7's install with the network on. Verdict: (b), with the fetch done by cargo itself (`cargo info`)

The request's scope, corrected: the install is not one download. With the network on, cargo fetches the CLI's index
entry (none is cached: ~/.cargo/registry/index/index.crates.io-1949cf8c6b5b557f/.cache/wa/sm/ holds no
wasm-bindgen-cli), its .crate, and every crate of its packaged Cargo.lock (254 packages, crates.io and path only:
security-r1-deps.md:116-117) not already among the 550 cached .crates. With `--bin wasm-bindgen` only that binary is
installed, but cargo still compiles the package's dependency graph (dependencies belong to the package, not to one
binary), so the build scripts and proc macros of that set run on this container, as root, in a session that can push to
the lane. R1 cleared that set (OSV over the CLI's lock: 6 hits, all in wasm-bindgen-test-runner's stack: buf_redux,
multipart, safemem, twoway, chrono RUSTSEC-2026-0543, url 2.5.8 RUSTSEC-2026-0592; security-r1-deps.md:120-122).

Why the order matters: cargo checks a downloaded .crate against the checksum in the index entry it fetched in the same
call, over the same proxy (the cargo book's index format: `cksum` is "A SHA256 checksum of the `.crate` file"; cargo
1.99.1's binary carries "failed to verify the checksum of" and "checksum for `...` changed between lock files"; the
code path itself not read: medium-high confidence). The only anchor that does not come from that same fetch is R1's pin
c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a (read 2026-11-22). Under (a) the pin is checked only
after third-party code has run, so a mismatch would be detection after the fact, in a container that can push. Under
(b) the root .crate is checked first; its packaged Cargo.lock then pins every dependency by checksum (with `--locked`
cargo refuses a lock that would change, and a lock checksum that differs from the index's), so the whole tree is
anchored on the pin before anything builds. The cost is one extra cargo call.

The fetch without a build, by cargo's own network path (not another one): `cargo info <name>@<version>` downloads the
.crate into the registry cache to read its manifest and builds nothing (cargo-info.1: "It fetches data from the
package's Cargo.toml file"). This lane did exactly that for cargo-fuzz 0.13.2, cargo-deny 0.22.0 and cargo-audit 0.23.0
on 2026-11-21 (u5-deps-facts.md:35-38: "each downloaded its .crate into the registry cache, nothing built or
installed"; their cached .crates, timed 15:09:17-18 UTC that day, still hash to security-u5-deps.md:153-154's values,
read now). Not `cargo install --dry-run`: unstable (cargo-install.1:155-158) and its download behaviour not verified.
The precedent's own installs (U5) checked only `--version` after a plain `cargo install --locked`; this rule is stronger.

Conditions:
1. Before the call, the main session writes with the Write tool the check file
   /home/user/Shamble-Frontlines/.gate/u7-0b/cli-crate.sha256 holding exactly one line (two spaces, final newline):
   `c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a  /root/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-cli-0.2.130.crate`
2. One main-run.sh call (log /home/user/Shamble-Frontlines/.gate/u7-0b/step7-cli.log), three commands joined by `::`:
   - `env CARGO_NET_OFFLINE=false cargo info wasm-bindgen-cli@0.2.130 --locked`
   - `sha256sum -c /home/user/Shamble-Frontlines/.gate/u7-0b/cli-crate.sha256`
   - `env CARGO_NET_OFFLINE=false CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u7-0b/cli-target cargo install
     wasm-bindgen-cli --version 0.2.130 --locked --root /home/user/Shamble-Frontlines/.gate/u7-0b/cli-root --bin
     wasm-bindgen`
   main-run.sh stops at the first non-zero exit (:55-71), so a missing or different .crate (`sha256sum -c` exits 1 on
   either) stops the line before any build. `CARGO_NET_OFFLINE=false` overrides the runner's export (main-run.sh:18) for
   that command only (environment beats config files: the cargo book's configuration page, net.offline). This lane has
   used it: U6c's `env CARGO_NET_OFFLINE=false cargo metadata` went online, "Adding hyper v1.13.0", and its `cargo
   fetch` printed "Downloaded hyper v1.12.0" (u6c-block0.md:28-32). It needs no runner edit and no `env` option word.
   `env -u CARGO_NET_OFFLINE` is an allowed fallback: it sits in front of cargo, not git or gh, so SEC-U7B-1's interim
   rule does not touch it. If cargo rejected the value, the line would stop at its first command, before any fetch.
   Two separate calls (info and check, then the install) are equally fine.
3. Nothing else changes: CARGO_HTTP_CAINFO stays as the container sets it (the proxy's CA bundle); no `--config`, no
   other `CARGO_NET_*` or `CARGO_HTTP_*` value, no TLS setting; no `--force`, `--features`, `--no-default-features` or
   `--all-features`; never `cargo fetch`, `update` or `generate-lockfile`. cargo install ignores the spike's
   .cargo/config.toml (cargo-install.1:104-109: discovery starts at $CARGO_HOME/config.toml, which does not exist).
4. Checks after the call, all before the CLI's first use and recorded in Q2's addendum:
   - (a) `/home/user/Shamble-Frontlines/.gate/u7-0b/cli-root/bin/wasm-bindgen --version` prints exactly the line
     `wasm-bindgen 0.2.130` (a whole-line compare, not a substring), and `ls` of that bin folder shows `wasm-bindgen`
     only;
   - (b) the packaged lock ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-cli-0.2.130/Cargo.lock
     has 254 `[[package]]` blocks and no `source = "git+` line; the log has no "Updating git repository" line; the
     count of the log's `Downloaded <name> v<version>` lines recorded, and each such name and version is in that lock;
   - (c) `cargo audit --file` on that packaged lock, as its own runner call (main-run.sh stops a line at a non-zero
     exit). Its expected exit is non-zero, by design: url 2.5.8's RUSTSEC-2026-0592 is a vulnerability entry. Record it:
     hits only on the crates R1 named above;
   - (d) `sha256sum spikes/netcode-proof/Cargo.lock` still 55604a5f...9f5d (cargo info `--locked` writes no workspace
     lock).
5. After the install every workspace cargo call stays `--locked` (it is the rule already; the cache now holds crates an
   unlocked offline resolve could pick up). The CLI's target folder may be removed as this unit's own folder.
6. The stop rule, by what happened:
   - `cargo info` fails (a usage or network error) or leaves no .crate (`sha256sum -c` reports the file missing): no
     install; stop and bring it here. Nothing else.
   - The .crate is present but its sha256 differs: no install. Nothing from it has run (cargo info parses the manifest
     only). Move it into .gate/u7-0b/quarantine/ so no later cargo call reuses it, record its sha256, make no further
     download in this container, and bring it here. Notes may still be saved.
   - The install stops on a checksum error ("failed to verify the checksum", "checksum for ... changed"): if the log
     shows no `Compiling` line before the error, as the branch above; if it shows one, as the branch below.
   - Check 4 (a) differs after the install (third-party code has run and the binary is not what was cleared): remove
     the root, nothing uses the CLI, and no commit, save or push of anything made after that call until Mustafa and I
     have read it.
   - Check 4 (b) or (c) differs (a count, a git source, a downloaded name, or an advisory beyond R1's set): with the
     hash already matched, this means a counting difference or a new advisory, not a swapped file. The CLI is not used
     until I have read it; no freeze on commits or saves.
   - The CLI is not on R1-R4's path (first use T5 (ii), after R3), so no branch here is urgent.

## Q3. A command form for the database dates. Verdict: (b) record the fetch times only, as U6c did

Why not (a): `git -C` is refused because the guard checks git by the folder it runs in (routine-guard.mjs:1046: "run
git from the folder, without -C, -c or --git-dir"). `env -C <folder> git ...` reaches the same place by another door,
and the guard does not read it at all (SEC-U7B-1). The two calls you made were read-only and on folders outside this
repository, so they did no harm, but the form is not one to keep. My own step 6 wording (security-u7-deps.md:310-312,
"run inside" each folder, with no form the guard reads) invited it (SEC-U7B-3).

Conditions:
1. dependency-check.md's database line gives the fetch times only (2026-11-25 05:44:45 and 05:44:47 UTC, as U6c's
   section gives its own). Drop the draft's "(newest entry 2026-11-24T22:47:44+01:00)" from the record; u7-block0b.md
   keeps it as written, since it states the method. Item (10) asks only that both database dates be recorded.
2. No git command on those two folders is needed. If a newest-entry date is wanted later, the form comes here first:
   I have not read how the guard treats git run in a folder outside the project, so I name none.

## Findings (CLAUDE.md's severity scale)
- **SEC-U7B-1 (Critical by the scale: a way around the guard; day sessions and their agents only; open):**
  - What: `unwrap()` (routine-guard.mjs:295-316) drops every word after `env` that starts with `-` (:302), but not the
    value word of env's value-taking options (`-C DIR`, `-u NAME`, `-S STRING`, and `--chdir`, `--unset`,
    `--split-string` with a separate value). That value becomes the "program", so bash()'s per-command loop
    (:716-764) never reaches git() or gh(), and dayShellWrites() (:869-898) sees no git writer. The same holds for
    `stdbuf -o MODE` (:310) and `nice --adjustment N` (:309). The one-word forms (`--chdir=DIR`, `-CDIR`) are dropped
    too, yet they move git's folder, so git() would check the wrong folder.
  - No ask catches it: parseShell's `unreadable` (:168-174) is set only by a comment, a line continuation, `$'...'`, a
    here-document delimiter that is not a plain word, or `>&`/`<&` before a file name. `env -C /path git switch x`
    holds none of these and names no GIT_DIR-like variable (:693-698).
  - Exploit sketch: `env -C /home/user/Shamble-Frontlines git switch <branch>` (the Never rule's main-folder switch) or
    `env -u X git push --force origin <branch>` runs with no ask and no deny, in a day session or from a subagent.
  - Who is not affected: routines. unwrapRole() refuses any env option (:485-496).
  - Context: L1 (kit-v3-3's stated limits, docs/reports/2026-11-05-kit-v3-3.md:127-131) already leaves `bash -c` unread
    in day sessions, and the permission layer still applies. But `env` is a wrapper the guard says it reads
    (routine-guard.test.mjs:132, "env wrapper hides force push"), and this form was used innocently today.
  - Evidence: found by reading the code; not run (the guard was not exercised).
  - Fix: a kit change-set (tier 2: plan, the advisor, security, Mustafa asked first, his go).
    - In unwrap(), consume the value of each value-taking option of env, stdbuf and nice.
    - Treat any env chdir option as a folder the guard cannot follow, so the main-folder check reads the project folder
      (as dayFolder's unknown folder does).
    - In a day session, ask on `-S`/`--split-string` (re-split text the guard does not read); refuse an agent.
    - Add test rows for each form beside :132, plus mutation rows.
  - Until then: no session or agent puts env, stdbuf or nice options in front of git or gh.
  - Status: it does not block U7's 0b. While open, it keeps that kit PR needs-mustafa.
- **SEC-U7B-2 (Low; my read; closed by this answer):** the binding check's header (security-u7-deps.md:281-282, "every
  cargo call through the U7 runner with CARGO_NET_OFFLINE=true") contradicted item (10): the CLI's install needs the
  network, and step 6's database fetches went to it too (step6-deny-audit.log:721-723, "Fetching advisory database",
  "Updating crates.io index"), as item (10) cleared. Exploit sketch: none against the game; a cleared step that cannot
  run as written invites an improvised form. Fix: Q1 names the networked calls. My later binding checks list each
  networked call by its command.
- **SEC-U7B-3 (Low; my read; closed by Q3):** step 6's "run inside" each database folder (:310-312) gave no command form
  the guard reads, and invited `env -C`. Fix: Q3's ruling.
No High or Medium. Anti-cheat coverage table: not applicable (no player action is in scope).

## Not verified
- That `cargo info` 1.99.1 leaves the .crate in ~/.cargo/registry/cache in this call (the U5 precedent observed it on
  this toolchain; condition 2's `sha256sum -c` catches it if not), and that it accepts `--locked` in this workspace
  (cargo-info.1:78-89 lists the flag; not run).
- That `--bin wasm-bindgen` still compiles the whole dependency graph. That is cargo's per-package model; I did not
  read wasm-bindgen-cli's manifest for `required-features` or optional dependencies, which would mean less compiles,
  never more.
- That cargo fetches every package before compiling (stop rule 6 reads the log's `Compiling` lines instead).
- Cargo's checksum code path (inferred from the cargo book and the strings in cargo's binary, not read in source; two
  fetches of cargo's source on GitHub returned 404).
- SEC-U7B-1 by a run (code read only, as asked).

## Memory candidates (not written: this task allows only this file)
- Download-only for a tool crate: `cargo info <name>@<ver> --locked`, then `sha256sum -c` against the read's pin,
  before `cargo install --locked` (U5 2026-11-21, U7 2026-11-25). Under that rule, a post-install check alone is
  detection only.
- The day guard's unwrap() misreads value-taking options of env, stdbuf and nice (SEC-U7B-1, 2026-11-25): never use
  them before git or gh. To go online for one cargo call: `env CARGO_NET_OFFLINE=false` (U6c used it).
- A binding check names each networked call by its command; never write "every call offline" above a step that fetches.

DONE
