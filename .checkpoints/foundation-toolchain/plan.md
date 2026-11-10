# Plan: foundation step 2.2, the toolchain (revision 2)

Status: revision 2. The advisor agent reviewed revision 1 (f5357520329a1ce76b7bf3ec3a873ea86f378cbc) at effort xhigh:
**APPROVE**, with conditions 1-6 (Part 9 records the verdict, the rulings and where each condition landed). Its words:
"No new plan review is needed if you fold conditions 1 to 5 in as written. Condition 1 must go into the plan before
FDN-NOCODE is sent to Mustafa." This revision folds conditions 1-5 as written and the Low items of condition 6. This
session plans only: no build, no code, no file outside `.checkpoints/` changed. Branch: `claude/laughing-pasteur-fwya24`
(this cloud session's branch, standing in for `cs/foundation-toolchain`). Base: main 2d22b69 (PR #46 merged;
STATUS-DIET, LANE-RULES, 2.1a and 2.1b not merged). Every line number was re-found on 2d22b69 (Appendix A); the build
re-finds them after 2.1b and LANE-RULES merge.
Sources: ADR 0002 (Proposed; parts 1-4 and 7-9, OQ-0002-1, -3, -4, -7, spike items 4 and 8, PM-0002-37 to -39,
migration plan 2.2), SPIKE_PLAN's foundation entry, the restructure plan at 828a3e0 (Part 2's sketch, Q2.2-a to -d,
Part 5, Appendix B) and at d389d91 (the SCOPE options), the advisor's rulings (`advisor-plan-rev2.md` at 828a3e0;
`advisor-plan-rev1.md`, this folder), Mustafa's words (1.2), PR #40's head 411eeee, and `research-network.md` (this
folder).

## In one screen
- **Mustafa's four asks** (his follow-up of 2026-11-10, item 3: "what it pins now, what stays an empty slot until the
  boundary spike, the CI job that runs the Rust checks, and its own plan review by the advisor agent"): Part 2, Part 3,
  Part 4, Part 9 (the advisor agent: APPROVE of revision 1, with conditions, folded here).
- **Pins now** (Part 2): Rust 1.99.1 (the netcode spike's pin, the same four keys), a virtual Cargo workspace
  (`crates/*`, resolver 3, edition 2024, `rust-version` 1.99.1, an empty `[workspace.dependencies]`, the workspace lint
  table, overflow checks for `sim` and `protocol`), `Cargo.lock`; npm workspaces (`packages/*`, one lockfile, exact
  versions, no root `"type"`), Node 22, TypeScript 6.0.3, Vitest 5.1.0, @playwright/test 1.65.0 (versions as read
  2026-11-10); six empty crates and five empty packages, one trivial test each; two `.gitignore` lines.
- **Empty slots** (Part 3), each named: OQ-0002-7 and the wasm-bindgen pin; OQ-0002-3 (one Wasm instance or two);
  OQ-0002-4 (isolation); OQ-0002-1 (the editor from `file://`); spike item 4 (generated types); spike item 8 (sizes,
  wasm-opt, `skipLibCheck`, the three.js casts, Playwright's clock).
- **The CI job** (Part 4): a new workflow file, `gate.yml` byte-identical. It installs Rust from `rust-toolchain.toml`
  with `rustup toolchain install` before any cargo call, checks `rustc --version` against the pin and that the active
  toolchain comes from that file, fetches, then runs `cargo test --workspace --locked`, a `wasm32-unknown-unknown`
  build, and `npm ci && npm test --workspaces` inside a network namespace: `sudo unshare --net`, loopback brought up as
  root, then `setpriv` back to the runner's user with no docker group and `no_new_privs`. Researched (GitHub's docs:
  passwordless sudo on Linux VMs; job containers refuse `--network`), so PM-0002-37 is promised; its proof is the job's
  own control and probe (judged on a TCP connection) plus four planted failures, each left red before its revert.
- **Kit, guard and CI edits** (Part 5): K1 the new workflow and K2a CLAUDE.md's hard-constraints heading (both
  protected, both covered by his words); K2b, K3, K4, the three "no engine code" lines (protected: asked, FDN-NOCODE,
  needed before step 5 only). Not in 2.2: the rules' globs (2.5, which brings the first v2 page), CLAUDE.md's Layout
  lines (REPO_MAP carries the new paths), guard checks for the new pins (2.3, his answer), the per-edit hook (its kit
  change-set).
- **The advisor's rulings:** G0's exit-2 mark moves to 2.3 (final); Node 22 (final, with a dated follow-up to move
  `tools/` and v2 to Node 26 together before 2027-03-31); the lint tools enter with 2.3; `.gitignore` takes two lines;
  the job has its own file; the planted failures are commits on the change-set's own branch.
- **Order:** builds when Mustafa starts it; the PR stays a draft until 2.1b has merged (his answer: "2.2 merges after
  2.1b"); risk tier 2; no Brave check (nothing a player sees, hears or feels).

## Part 1. Goal, scope, rules, risk tier

### 1.1 Goal and change type
ADR 0002 migration plan 2.2 (`0002:1432-1438`): "The two workspaces and the pins (part 2), the first crates (`sim`,
`protocol`, `room`, `client-endpoint`, `sim-wasm`, `devtools`) and packages (`client`, `sim-worker`, `net`, `types`,
`e2e`)", done when "from a fresh clone on the CI runner, `cargo test --workspace --locked` and `npm ci && npm test
--workspaces` pass with no network access after the install steps; `rustc --version` in CI equals the pin; deleting the
toolchain file fails G0 with exit 2 ...; a pull request that edits `Cargo.toml` without `Cargo.lock` fails at the first
cargo step". Change type: toolchain and CI (logic: tests on the real files; CI: proof runs with planted failures;
security: CI hygiene, the cut-off and the dependency review). Legacy symbols +0 −0 (the gate's check 3).

### 1.2 Mustafa's words this plan rests on (word for word)
- His decision of 2026-11-10 (`mustafa-2026-11-10.md` at 828a3e0): "The second account may plan and build foundation
  steps 2.1 and 2.2 now. Steps 2.3 to 2.6 keep their order and wait for their results."
- SCOPE (a) (`mustafa-2026-11-10-answers.md` at 828a3e0): "I accept ADR 0002's part 1 alone: the repository after the
  restructure, with the legacy hard constraints applying to legacy/ only once the files have moved. Record it in the
  ADR's Status line in 2.1b's docs pass, on my go. Nothing else in ADR 0002 is accepted by this."
- FREEZE-FDN, his words on main next to KIT-FREEZE-2 (`docs/STATUS.md:686-690`): "The freeze stops new rules, new guard
  checks and kit redesign. It does not stop the smallest edits the foundation needs to keep the kit true after its own
  moves (paths in the gate and in CLAUDE.md, the toolchain's CI job, the lines that scope the legacy constraints). Each
  such edit still asks me and merges on my go."
- His answers in this session's prompt (`mustafa-2026-11-10-session.md`, this folder): "FREEZE-FDN and the CI job: yes,
  it is covered. ... Guard checks for the new pins are not covered; they wait for the gate step (2.3)." and "Merge
  order: 2.2 merges after 2.1b."
- LANE-RULES' Models line with his addition (`docs/STATUS.md:700-710`): Fable 5.2 may advise through the advisor tool;
  "it is an extra. The recorded plan review by the advisor agent stays required." (The advisor tool is not among this
  session's tools; the advisor agent's review is recorded in Part 9.)

### 1.3 Does any rule forbid 2.2 that his decisions do not cover? No (the advisor: "holds").
| Rule (main 2d22b69) | What it says | What covers 2.2 |
|---|---|---|
| `CLAUDE.md:20-22`, `:52` | "No engine code: throwaway spikes only ..."; "Phase 0 writes no v2 code." | his decision: "may plan and build foundation steps 2.1 and 2.2 now" |
| `CLAUDE.md:45-50` | the hard constraints (classic scripts, r128, ...) | SCOPE (a): they apply to `legacy/` only once the files have moved; 2.2 merges after 2.1b |
| `advisor.md:107` | "No engine code. Throwaway spikes only ..." | his decision, as above |
| `advisor.md:323` | Never: "Merge a hard-constraint change before its accepted ADR is on main." | ADR 0002 part 1 accepted (SCOPE (a)), recorded in its Status line by 2.1b's docs pass; step 5 checks it is on main before K2a and K2b are written (condition 5) |
| KIT-FREEZE-2 (`STATUS:682-683`) | the kit freezes until the gate step | FREEZE-FDN (1.2): the CI job and the constraint-scope lines; FDN-NOCODE asks about three more lines |
| `engine-architect.md:52`, `graphics-engineer.md:41` | Never: "write engine code in phase 0" | they bind only those agents' own writing (the main session builds); engine-architect reviews 2.2 and is briefed on his decision (step 7); graphics-engineer when it next reviews v2 work |
| `V2_PHASE0_BRIEF.md:20-21`, `:144` | "No engine code" in phase 0 | his decision. The brief's own `:22` names "Phase 1: the repository restructure, toolchain, CI gate v2". It is the claude.ai Advisor's file: no session edits it, so these lines stay stale, and the report says so |
| `NIGHT_SHIFT.md:54-55` | "No v2 spike runs until ..." | not night work: 2.2 needs cargo, which the guard refuses to routines (`CHECKPOINTS.md` §10) |

### 1.4 Not in scope
2.3's gate v2 (G0-G13, `--prove`, the budget file, the aggregate required check, caches, Dependabot, the scheduled
jobs including part 2's weekly latest-dependencies job); the lint tools, their configs and `crates/sim/clippy.toml`
(G1-G3); cargo-deny (G5); `content`, `navbake`, `server`, `supervisor`, `editor`, `fuzz/` (each enters with its slice
or job: part 1, part 3); guard checks for the new pins (his answer); the per-edit hook (part 7; PM-0002-62: "lands in
its kit change-set, not in the foundation"); the rules' globs (Part 6); `spikes/` (never touched).

### 1.5 Risk tier and route
Risk tier 2 (a new `.github/` workflow, CLAUDE.md, possibly `advisor.md`): the advisor plus security-auditor; reviewers
on the final diff (Part 8); merges on Mustafa's go from a day session, after 2.1b. The PR Advisor cannot merge it:
`mergeAllowed` refuses every new path but the docs (Part 5, E9).

## Part 2. What 2.2 pins now (ask 1)
| Pin | Value | File | Source | Proved by |
|---|---|---|---|---|
| Rust | `channel = "1.99.1"`, `profile = "minimal"`, `components = ["clippy", "rustfmt"]`, `targets = ["wasm32-unknown-unknown"]` | `rust-toolchain.toml` | part 2; R5-CI-REC2; the same four keys and values as `spikes/netcode-proof/rust-toolchain.toml` at 411eeee | E2: the job's pin check and its active-toolchain check; M2 |
| Cargo workspace | virtual; `members = ["crates/*"]` and no `exclude`; `resolver = "3"`; `[workspace.package]` `edition = "2024"`, `rust-version = "1.99.1"`, `publish = false`; an empty `[workspace.dependencies]` | root `Cargo.toml` | part 2; R5-LAYOUT-REC1; Appendix B | E1, E2; M3 |
| Lockfile | `Cargo.lock` committed; every CI cargo command takes `--locked` | root `Cargo.lock` | part 2; R5-CI-REC2 | M3 |
| Lint table | `[workspace.lints.rust] unsafe_code = "deny"`; `[workspace.lints.clippy] all = { level = "warn", priority = -1 }`; `[lints] workspace = true` in every crate | root `Cargo.toml`, each crate | parts 4 and 7 (rustc enforces the deny now; clippy's levels act from 2.3, G2) | L1 |
| Overflow | `[profile.release.package.sim] overflow-checks = true`, the same for `protocol` | root `Cargo.toml` | part 4 | E1: the release run of `sim` and `protocol` |
| npm workspaces | `workspaces: ["packages/*"]`; one root `package-lock.json`; exact versions; `private: true`; no `"type"` key | root `package.json` | part 2; Appendix B | E1, E2, E6, E7 |
| Node | 22: `engines.node: "22.x"`, CI `node-version: 22` | root `package.json`, the workflow | part 2 ("the way the legacy toolkit pins 22", TL17); Q2.2-c (a), the advisor's ruling | E2 prints it |
| TypeScript | `6.0.3` (the 6.0 line's last patch; npm's `latest` is 7.0.2, which part 2 leaves to its own change-set) | root `devDependencies` | part 2 | E2 prints `tsc --version`; E7 |
| Vitest | `5.1.0` (engines `^22.12.0\|\|^24.0.0\|\|>=26.0.0`) | root `devDependencies` | part 8 (TypeScript unit) | runs four packages' tests |
| Playwright Test | `@playwright/test` `1.65.0`; no browser installed in 2.2 | root `devDependencies` | part 8 (`e2e`) | runs `e2e`'s test (A2) |
| `.gitignore` | `/target/` (anchored) and `test-results/` (unanchored) | `.gitignore` | part 1 (only what 2.2's tools write; the rest of part 1's list enters with the tools that write it) | E1: `git status` clean after the tests |

Versions are the registry's on 2026-11-10 (`npm view`, `research-network.md` B4; the advisor re-read them); the build
takes the newest patch of each named line on its day and records it, and E7 checks that the major lines stay as above.

**The crates** (part 3; each a `Cargo.toml` with `version = "0.0.0"`, the workspace's edition, `rust-version` and
`publish`, `[lints] workspace = true`, no dependencies; a `src/lib.rs` with one doc line naming its part-3 role and
one test): `sim` and `protocol`: `overflow_checks_are_on`, a `#[should_panic]` test that adds 1 to
`std::hint::black_box(u8::MAX)` (it passes only with overflow checks on, so the release run proves part 4); `room`,
`client-endpoint`, `sim-wasm`, `devtools`: `stub_builds`. Each crate's dependencies enter with the code that needs them
(part 3's table is the rule G5 checks from 2.3). `sim-wasm` is a plain library: no wasm-bindgen, no `crate-type`
(Part 3).

**The packages** (part 3; folder names as part 3, package names `@shamble/<folder>`, each `private: true`,
`"type": "module"`, one `test` script): `client`, `sim-worker`, `net`, `types`: `src/index.ts` exporting
`PACKAGE = "@shamble/<folder>"` and `src/index.test.ts` (Vitest) checking it; `e2e`: `tests/stub.spec.ts`
(Playwright Test, no browser fixture). No tsconfig (part 7, G3: 2.3), no Vite config (G6), no `index.html` (2.5). The
internal dependencies between packages enter with their code; when they do, they name the sibling's exact version so
npm links it and never looks it up in the registry (security-auditor's point to check then).

**Symbols added** (v2 only; legacy +0 −0): six Rust test functions (`overflow_checks_are_on` ×2, `stub_builds` ×4);
four TypeScript constants `PACKAGE` and four Vitest tests; one Playwright test. Nothing removed.

**Enters later, by step** (not an empty slot of the boundary spike): ESLint 10, typescript-eslint, Prettier, the base
tsconfig, rustfmt's and clippy's configs (2.3, G1-G3); cargo-deny (2.3, G5); rust-cache and npm caching (2.3); the weekly
latest-dependencies job (2.3, part 9's scheduled list); the fuzz jobs' dated nightly (`fuzz/`, with the fuzz jobs).

## Part 3. What stays an empty slot until the boundary spike (ask 2)
| Slot left empty in 2.2 | Open question or spike item | Why 2.2 leaves it |
|---|---|---|
| wasm-bindgen: the library in `sim-wasm`, the CLI pinned to `Cargo.lock`'s version (taiki-e/install-action by SHA, `fallback: none`) and the CLI-equals-lock check | OQ-0002-7 (`0002:1063-1071`: is its new home healthy?; decider: "a by-day reading of its maintenance issue (#4533) and the guide's governance page") and R5-CI-REC3 | `sim-wasm` has no bindings until the boundary spike fixes its exports; the by-day reading is done before the step that adds wasm-bindgen |
| `sim-wasm`'s exports, `crate-type`, and `sim-worker`'s shape | OQ-0002-3 (`0002:1008-1021`: one Wasm instance or two in a local match) | both stay stubs |
| the isolation flag and the Vite headers | OQ-0002-4 (`0002:1022-1034`) | no Vite config in 2.2 |
| the `editor` package | OQ-0002-1 (`0002:986-997`: the editor from `file://`) | `editor` enters at slice 5 |
| `packages/types`' three generators, the layout hash, `tsc --strict` on the output, byte-identical regeneration | spike item 4 (`0002:1144-1148`; R5-LAYOUT-OQ4, R5-TSLINT-OQ3) | `types` is a stub until 2.4 |
| `opt-level` "s" or "z", wasm-opt, `skipLibCheck`, the three.js type casts, Playwright's clock in the Worker | spike item 8 (`0002:1165-1171`; R1-RR-OQ5, R5-CI-OQ4, R5-TSLINT-OQ6, R5-TSLINT-OQ4, R5-TEST-OQ1) | no release `opt-level`, no wasm-opt, no tsconfig flag in 2.2 |

Also left empty, by other results: the sim's maths and geometry dependencies (the sim core spike, ADR 0004); the
protocol codec and the transport crates (the netcode proof test, ADR 0001); the three.js version (ADR 0003; `client`
at 2.5).

## Part 4. The CI job that runs the Rust checks (ask 3)

### 4.1 Shape
- A new file `.github/workflows/toolchain.yml`, one job `toolchain`. `gate.yml` stays byte-identical (E5), so the gate
  and smoke jobs are unchanged by construction; 2.3 replaces this file with the gate v2's workflow (part 9, option G1)
  in one move.
- `on: push` and `pull_request`, no path filters (part 9: "No path filters"); `concurrency: group:
  toolchain-${{ github.workflow }}-${{ github.ref }}`, `cancel-in-progress: true` (as `gate.yml`; so a planted failure
  is left red before its revert is pushed: Part 7); `runs-on: ubuntu-24.04` (the runner image page: "`ubuntu-latest`
  label will use Ubuntu 26.04 in November 2026"); `timeout-minutes: 15`; `permissions: contents: read`;
  `persist-credentials: false`; the same action SHAs as `gate.yml` (checkout v7.0.1, setup-node v7.0.0); no caches
  (each run is the cold, fresh runner PM-0002-37 names); no secrets; no `pull_request_target`; `defaults.run.shell:
  bash` (`-eo pipefail`).
- Everything it runs is written in the workflow itself, never in a script under `tools/`: `mergeAllowed` lets the PR
  Advisor merge a non-gate file under `tools/` alone, while `.github/` is protected and refused (Part 5).
- Workflow env `RUSTUP_AUTO_INSTALL: "0"`: since rustup 1.28.1 "Automatic install is enabled by default" (the image has
  rustup 1.29.2), so without it "before any cargo call" is only an order; with it the explicit install is the only way
  a toolchain arrives (N4).
- The control and the probe use `https://github.com/` (the advisor's F-4: the runner must reach GitHub to run at all,
  so the control adds no dependency the job does not already have).

### 4.2 Steps (draft; the build may only tighten it)
```yaml
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - name: Install the pinned Rust toolchain from rust-toolchain.toml (before any cargo call)
        run: rustup toolchain install
      - name: Check rustc against the pin
        run: |
          pin=$(sed -n 's/^channel = "\([0-9]\{1,\}\.[0-9]\{1,\}\.[0-9]\{1,\}\)"$/\1/p' rust-toolchain.toml)
          [ -n "$pin" ] && [ "$(printf '%s\n' "$pin" | wc -l)" -eq 1 ] || { echo "rust-toolchain.toml: no single exact x.y.z channel"; exit 1; }
          [ -z "${RUSTUP_TOOLCHAIN-}" ] || { echo "RUSTUP_TOOLCHAIN is set: it would beat rust-toolchain.toml"; exit 1; }
          active=$(rustup show active-toolchain); echo "$active"
          case "$active" in *rust-toolchain.toml*) ;; *) echo "the active toolchain does not come from rust-toolchain.toml"; exit 1 ;; esac
          v=$(rustc --version); echo "$v"
          case "$v" in "rustc $pin "*) echo "rustc equals the pin $pin" ;; *) echo "rustc is not the pin $pin"; exit 1 ;; esac
          cargo --version; node --version; npm --version
      - name: Fetch every dependency (the last step with the network)
        run: |
          cargo fetch --locked
          npm ci --ignore-scripts --no-audit --no-fund
      - name: Build and test with the network cut off
        run: |
          command -v unshare setpriv ip curl getent
          curl -sS --max-time 20 -o /dev/null https://github.com/ \
            || { echo "control failed: no network before the cut, so the probe below would prove nothing"; exit 1; }
          sudo -n unshare --net -- bash -ec 'ip link set lo up; exec "$@"' lo-up \
            setpriv --reuid="$(id -u)" --regid=nogroup --clear-groups --no-new-privs -- \
            env -i HOME="$HOME" PATH="$PATH" USER="$USER" CI=true LANG=C.UTF-8 \
              CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}" \
              RUSTUP_AUTO_INSTALL=0 npm_config_update_notifier=false \
            bash -euo pipefail -c '
              id; ip -o link; rustc --version
              getent hosts github.com || echo "getent: no answer (printed, not judged)"
              if curl -sS --max-time 10 -o /dev/null https://github.com/; then
                echo "the network is still reachable inside the cut"; exit 1
              fi
              echo "the network is cut"
              cargo test --workspace --locked
              cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm
              npm ci --offline --ignore-scripts --no-audit --no-fund
              npm test --workspaces
              npx --no -- tsc --version
            '
```
- The wasm32 build covers the crates part 3 builds for Wasm (`sim`, `protocol`, `room`, `client-endpoint`, `sim-wasm`;
  `devtools` is native); it is PM-0002-37's "a fresh runner builds native and `wasm32-unknown-unknown` from
  `rust-toolchain.toml` alone" (also R5-CI-REC2 mark 1).
- `npm ci` runs twice: online, it fills npm's cache; inside the cut, `npm ci --offline` ("Force offline mode: no
  network requests will be done during install") rebuilds `node_modules` from the cache alone, which is PM-0002-37's
  literal "`npm ci && npm test --workspaces` pass with no network access after the install steps".
- `npm test --workspaces` without `--if-present`, so a package without a test script fails (N3, proved by L3).
- Inside the cut the job prints `id` (no docker group), `ip -o link` (only `lo`, up), `rustc --version` (the pin, read
  through the same toolchain file) and the DNS answer, if any; it judges only the TCP probe.

### 4.3 How a runner cuts the network: the research (`research-network.md`, B1-B3)
- GitHub's runner docs: "The Linux and macOS virtual machines both run using passwordless `sudo`." and "each
  GitHub-hosted runner is a new virtual machine (VM)". Job containers cannot do it: "The `--network` and
  `--entrypoint` options are not supported." A firewall rule for the whole VM or the runner's user would cut the runner
  itself: "A GitHub-hosted runner must establish connections to GitHub-owned endpoints ...".
- So: a network namespace for one child process. util-linux `unshare`: "-n, --net: Create a new network namespace."
  Under sudo (the page does not say what an unprivileged `unshare --net` needs, so nothing rests on user namespaces).
  util-linux ships `unshare` and `setpriv` on Ubuntu 24.04 (the advisor checked in an Ubuntu 24.04.5 container:
  util-linux 2.39.3, a package every install has).
- Loopback: the namespace's `lo` is brought up by root before the drop (the advisor's F-10), so a runner that needs
  127.0.0.1 works, and 2.5's local server will need it; loopback reaches only processes in the same namespace, and the
  host's services listen in the root namespace (reasoning).
- Back to the runner's user: util-linux `setpriv` `--reuid`, `--regid` ("you must specify one of --clear-groups,
  --groups, --keep-groups, or --init-groups if you set any primary gid"; it takes a group name, `<gid|group>`, so
  `nogroup` works), `--clear-groups` ("Clear supplementary groups."), and `--no-new-privs` ("execve(2) will not grant
  new privileges." "This bit is inherited by child processes and cannot be unset."); the kernel's no_new_privs page:
  "the setuid and setgid bits will no longer change the uid or gid", so code inside cannot `sudo` its way out. With the
  group set to `nogroup` and supplementary groups cleared, no docker group remains (the Docker daemon's socket is a
  file, reachable from any network namespace by a member of its group: reasoning, N2; `id` inside the cut shows it).
- DNS is printed, not judged (the advisor's F-3): a local resolver can answer through a socket file, which a network
  namespace does not cut, so a name may resolve inside the cut while no connection can leave it (N5). The probe judges
  a TCP connection to github.com:443, after the same connection succeeded outside (the control).
- What a new namespace contains is not stated on the pages read (network_namespaces(7), ip-netns(8)); the job does not
  rest on it: it prints `ip -o link` inside, and its probe must fail inside after the control passed outside.
- The image (runner-images, Ubuntu 24.04, image 20261101.360.1) lists "Rustup 1.29.2", "Rust 1.99.1", iproute2; it does
  not list util-linux, so the job's `command -v` line fails loudly if a tool were missing. Its default Rust equals the
  pin today (N1), so the version check alone cannot show the toolchain came from the file: the job also checks that
  `rustup show active-toolchain` names `rust-toolchain.toml` (the advisor's F-5); rustup names the file's toolchain
  `1.99.1-x86_64-unknown-linux-gnu`, apart from the image's `stable`.
- Not researched and not relied on: unprivileged user namespaces (Ubuntu 24.04's AppArmor restriction), bubblewrap,
  firejail, `systemd-run --property=PrivateNetwork=yes`, third-party egress actions.

### 4.4 What the cut proves, and what it does not
It proves honest code builds and tests with no network: a test or build step that reaches out fails (M1a, M1b). It is
not a sandbox against hostile pull-request code (the job holds no secret and `contents: read` only); the cleared groups
and `no_new_privs` close the two easy ways out (docker, sudo). If the runner refuses any part of it (E2 shows), the
build stops and the advisor rules; there is no silent fallback to `--offline` flags alone, which would not cover the
tests themselves.

### 4.5 The marks
- PM-0002-37: promised (E2, M1a, M1b).
- PM-0002-38: 2.2's part only, "`rustc --version` in CI equals the pin" (E2); every tool's version against its pin in
  every job is 2.3's aggregate job (its method).
- PM-0002-39: "a pull request that edits `Cargo.toml` without `Cargo.lock` fails at the first cargo step" is 2.2's (M3);
  "deleting `rust-toolchain.toml` fails G0 with exit 2" moves to 2.3 with G0 (Q2.2-a (a), final: the advisor's ruling),
  recorded as a done-when reading in the register (a decision doc: his go); 2.2's job still fails closed before any
  cargo call when the file is gone (M2). The Playwright-browser item and the control are 2.3's.

## Part 5. Kit, guard and CI edits, with the guard's marks, and FREEZE-FDN
P = `protectedRel` true (routines may not write it; a day session's write asks Mustafa). M = `mergeAllowed` true (the
PR Advisor may merge it). Run on main 2d22b69 with the guard's own functions (`marks.mjs` in the scratchpad; the
advisor reproduced every mark with its own script, `advisor-marks.mjs`, this folder).
| # | File and lines (2d22b69) | Edit | Marks | His words on main name it? | Status |
|---|---|---|---|---|---|
| K1 | `.github/workflows/toolchain.yml` (new) | the CI job (Part 4) | P, not M | "the toolchain's CI job" | covered (his answer in this session's prompt); one ask at the write; merges on his go |
| K2a | `CLAUDE.md:45` (the heading) | scope the hard constraints to `legacy/` (ADR 0002 part 1, SCOPE (a)) | P, not M | "the lines that scope the legacy constraints" | covered; one ask at the write, after condition 5's check |
| K2b | `CLAUDE.md:52` ("Phase 0 writes no v2 code.") | his 2026-11-10 exception for steps 2.1 and 2.2, one clause | P, not M | not in the parenthesis | asked: FDN-NOCODE |
| K3 | `CLAUDE.md:20-22` ("No engine code: throwaway spikes only, and only after Mustafa and the Advisor have approved the drafts; until then, stop.") | the same exception, one clause | P, not M | not in the parenthesis | asked: FDN-NOCODE |
| K4 | `.claude/agents/advisor.md:107` ("No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts.") | the same exception, one clause (else the advisor's own rule reads 2.3's crates as a REVISE) | P, not M | not in the parenthesis | asked: FDN-NOCODE |
| — | `CLAUDE.md:41-43` (the Layout lines) | none: they list the legacy layering and a few top-level items, not every folder (no `.github/`, no `README.md`), and stay true; REPO_MAP carries the new paths (E8) | P, not M | "paths in ... CLAUDE.md" would allow it | not needed (the advisor's F-6) |
| — | `.claude/rules/pages.md:3-4`, `engine-core.md:3`, `engine-game.md:3`, `engine-editor.md:3` (globs) | none in 2.2 (Part 6) | P, not M | option (a) named "the rules' globs"; his words on main do not | 2.5's plan |
| — | `.claude/rules/netcode.md:3-5` (`**/net/**`) | none: it starts loading for `packages/net/**`, as its own heading says ("active once these folders exist"); its bullets fit transport code | P, not M | — | observation |
| — | the guard (`routine-guard.mjs:28-29` `GATE_FILES`, `:35-48` `protectedRel`) | none: protecting the pins is a new guard check | P, not M | his answer: "not covered; they wait for the gate step (2.3)" | 2.3 |
| — | the per-edit hook and his note "fast checks on save" (SPIKE_PLAN :438-442) | none: a new hook is new kit | P | not covered (KIT-FREEZE-2 stops new rules and kit redesign) | its kit change-set (PM-0002-62) |
| — | `.github/workflows/gate.yml` | none (E5) | P, not M | — | unchanged |
| — | `.gitignore` | two lines | neither | — | no ask (not kit) |

Not kit, for completeness: root `rust-toolchain.toml`, `Cargo.toml`, `Cargo.lock`, `package.json`,
`package-lock.json`, everything under `crates/` and `packages/`: neither mark (so routines can write them on a branch
and day sessions get no ask until 2.3; the PR Advisor cannot merge them). Docs: `docs/REPO_MAP.md`, STATUS, ROADMAP
and the report are M; the register (`docs/adr/README.md`) is not M (a decision doc: his go).

Draft wording (the build re-finds the lines after 2.1b and LANE-RULES; each write asks him):
- K2a: "## Hard constraints of the legacy game, `legacy/` only (change ONLY through an accepted ADR signed off by
  Mustafa, never on your own)".
- K2b: after "Phase 0 writes no v2 code": ", except foundation steps 2.1 and 2.2 (Mustafa, 2026-11-10)", with a
  pointer to his words (DECISIONS.md once STATUS-DIET has merged). The rest of the paragraph stays.
- K3 and K4, after the first sentence: "Exception: foundation steps 2.1 and 2.2 (Mustafa, 2026-11-10)."

## Part 6. Coexistence with PR #40's spike (Appendix B kept) and the rules' globs
- **No text `spikes/` in a tracked non-Markdown file outside `spikes/`, `docs/`, `.checkpoints/`** (the spike's
  `referenceProblems`, 411eeee `check.mjs:1250-1264`): the root `Cargo.toml` takes `members = ["crates/*"]` and no
  `exclude`; `package.json` takes `packages/*`; `.gitignore` and the workflow name what they cover. E4a.
- **No root `.cargo/config.toml`** (`cargoConfigFiles`, `check.mjs:1226-1245`, reads `.cargo/config.toml` in every folder
  above the spike): the workflow puts nothing in cargo config; flags and folders stay on command lines.
- **No `"type"` in the root `package.json`** (the syntax hook runs `node --check` in place, `syntax-gate.mjs:3` and
  `:21-23`; after 2.1b the nearest `package.json` of `legacy/*.js` is the root one): each package sets its own. E6.
- **The rules' globs: left to 2.5** (the advisor: "justified"). They match `**/index.html`, `**/editor.html` and
  `**/shamble-*.js`; after 2.1b they match only `legacy/` files, and 2.2 adds no such file (no page until 2.5, no
  editor until slice 5), so they stay true through 2.2. Narrowing them now would be a kit edit his words on main do not
  name and no move of 2.2 needs; 2.5's plan narrows them, with its ask, in the change-set that adds the first v2
  `index.html` (the FREEZE-FDN option (a) he picked listed the globs, so 2.5 can still ask).
- **Checked in the build, not assumed:** (E4b) `cargo metadata --format-version 1 --no-deps` run in
  `spikes/netcode-proof/` still names the spike's own workspace root; (E4c) the gate job's and the smoke job's
  installs (`npm ci --prefix tools --omit=optional --ignore-scripts --no-audit --no-fund`, and the same without
  `--omit=optional`; the night shift's is the latter) still install from `tools/package-lock.json` with the root
  `package.json` present; (E4d) the spike's own `node spikes/netcode-proof/check.mjs --only deps,refs` passes on 2.2's
  tree merged with PR #40's head (in a worktree under `.gate/`, never by switching the main folder), or on main if #40
  has merged by then.

## Part 7. Evidence
Local cargo runs build into `.gate/foundation-toolchain/target` (`--target-dir`), cleaned afterwards with `cargo clean
--target-dir .gate/foundation-toolchain/target` (`CHECKPOINTS.md` §10; the advisor's F-11); CI keeps `/target/`.
| Id | What | How | Pass |
|---|---|---|---|
| E1 | the workspaces build and test locally | `cargo test --workspace --locked`; the wasm32 build of Part 4; `cargo test --release -p sim -p protocol --locked` (each with the target folder above); `npm ci --ignore-scripts && npm test --workspaces`; `git status --porcelain` | all pass; the release run's `should_panic` tests pass (part 4's overflow pin); the tree stays clean |
| L1 | the lint table bites | an `unsafe {}` block in one stub, `cargo test` | compile error from `unsafe_code`; reverted |
| L2 | a stale npm lock fails | a devDependency added to `package.json` without the lock, `npm ci` | npm's mismatch error; reverted |
| L3 | a package without a test script fails | its `test` script removed, `npm test --workspaces` | non-zero exit (N3); reverted |
| E2 | the job on the PR's head (1 run, PM-0002-37's count) | CI log | green; rustup installs `1.99.1` from the file; the active toolchain names `rust-toolchain.toml`; `rustc 1.99.1 (...)` outside and inside the cut; the control passes outside; inside, `id` shows no docker group, `ip -o link` shows only `lo` (up), the DNS answer is printed, the TCP probe fails, every test passes; the job's wall time reported |
| M1a | a Rust test that opens a TCP connection to `github.com:443` | Part 7's procedure below | red at `cargo test` inside the cut |
| M1b | a Vitest test that fetches `https://github.com/` | the same | red at `npm test --workspaces` |
| M2 | `rust-toolchain.toml` deleted | the same | red at the install or the pin check (its `sed`); no cargo step runs |
| M3 | a path dependency added (`protocol` on `sim`) without `Cargo.lock` | the same | red at `cargo fetch --locked`, the first step that reads the lock |
| E4a-d | coexistence (Part 6) | `git grep -n 'spikes/' -- ':!spikes' ':!docs' ':!.checkpoints' ':!*.md'`; `test ! -e .cargo/config.toml`; E4b-d as Part 6 | no output; absent; as Part 6 |
| E5 | the gate and smoke jobs unchanged | `git diff --exit-code origin/main -- .github/workflows/gate.yml`; both jobs green on the PR | no diff; green |
| E6 | the legacy side unaffected | `node tools/gate.mjs` (symbols +0 −0); `node -e` asserting the root `package.json` has no `type` key | PASS; holds |
| E7 | exact versions | a `node -e` check over every `package.json`: no `^`, `~`, `*`, `x`, range or tag; TypeScript's major is 6 | holds |
| E8 | the repository map | `node .claude/scripts/repo-map-check.mjs` | exit 0 (new top-level entries listed) |
| E9 | the kit-edit list is complete | `marks.mjs` on the final file list | every P path is in Part 5 and was asked; no new path but the docs is M |
| E10 | the kit's own tests still pass | `node --test .claude/hooks/routine-guard.test.mjs .claude/hooks/settings.test.mjs .claude/scripts/checkpoint.test.mjs` | main's counts |
| E11 | the dependency review (condition 4) | from `package-lock.json`: the package count, every package with `hasInstallScript`, and every `resolved` address (each must point at `https://registry.npmjs.org/`) | listed in security-auditor's brief and the report |

**The planted failures** (M1a, M1b, M2, M3; condition 3): each is one commit on the change-set's own branch while its PR
is a draft. Push it alone, then wait until its run is red at the expected step; only then push its `git revert` (with
`cancel-in-progress`, a revert pushed early can cancel the red run and lose the evidence). Reverts are new commits,
never a force-push or a reset; none carries `[skip ci]`; none touches a protected file, so none needs an ask; the final
diff against main contains none of them; the report lists each run's id and its failing step. The control and probe
inside E2's one green run already show that the probe bites.

## Part 8. Steps, reviewers, models
Steps (each one commit; the gate after each; the checkpoint `foundation-toolchain` updated, committed and pushed after
every step; the saver while agents run):
1. Root pins: `rust-toolchain.toml`, `Cargo.toml`, `package.json`, `.gitignore`; `Cargo.lock` (`cargo
   generate-lockfile`) and `package-lock.json` (`npm install --ignore-scripts`), the versions recorded.
2. The six crates and five packages. E1, L1-L3, E6, E7, E11.
3. K1, the workflow (one ask). Push (draft PR); E2.
4. M1a, M1b, M2, M3, each by Part 7's procedure; run ids recorded.
5. After 2.1b and LANE-RULES have merged: bring main in. If the build started from main after both merged, there is
   nothing to merge. Otherwise merge in a worktree under `.gate/`, never in the main folder (CLAUDE.md's Never list
   counts `merge` as a switch; only `git merge --ff-only origin/main` is allowed there): `git worktree add --detach
   .gate/foundation-toolchain-merge HEAD`, `git merge origin/main` there, push `HEAD:<branch>` from there (a
   fast-forward of the remote branch), and carry on in that worktree (`CHECKPOINTS.md` §8). Then check that ADR 0002's
   Status line on main records part 1 as accepted (2.1b's docs pass), since `advisor.md:323` forbids merging a
   hard-constraint change before its accepted ADR is on main (condition 5); only then K2a (one ask), and K2b, K3 and
   K4 if FDN-NOCODE is (a) (one ask each). Re-run E1-E11.
6. Docs: REPO_MAP (E8); the register (the G0 done-when reading, Q2.2-a; his go); STATUS (or DECISIONS) and ROADMAP by
   docs-keeper, with the dated follow-up for Node (Q2.2-c: one change-set moving `tools/` and v2 to Node 26 together,
   after 2.3, merged before 2027-03-31).
7. The review round on the final diff, 3 at a time: security-auditor (tier 2: the workflow, sudo, the cut, CI hygiene,
   and the dependency review of E11: CLAUDE.md's Never line on running unreviewed code from the internet is met the way
   `tools/` meets it today, by a pinned lockfile plus this review), infra-engineer (CI), rust-reviewer (manifests, the
   toolchain file); then test-engineer (is it proven) and engine-architect (the crate and package boundaries; briefed
   that his 2026-11-10 decision is the exception to its file's "write engine code in phase 0" line). One fix pass
   within the review budget; the verifier on the final diff; the report (`/advisor-report`, which also says that
   `V2_PHASE0_BRIEF.md:20-21` and `:144` stay stale, the claude.ai Advisor's file); the checkpoint folded in and
   deleted; the PR ready; the advisor's review of that head; his go.
Every agent runs Opus 5.5 (`model: opus`) at its file's effort: `high` for all but engine-architect, whose file says
`xhigh` (the advisor's F-7); this plan's review ran at `xhigh` at his instruction. At most 3 agents at a time. Not
night-queue work.

## Part 9. Its own plan review (ask 4)
**The advisor agent's verdict on revision 1** (f5357520329a1ce76b7bf3ec3a873ea86f378cbc; effort xhigh; from main's
`.claude/agents/advisor.md`; notes `advisor-plan-rev1.md`, this folder, ending DONE): **APPROVE**, with conditions:
1. FDN-NOCODE's text: three lines, not two (K2 split into K2a, covered, and K2b, asked); quote the SCOPE option (a)
   text he picked and say his reply does not repeat that clause; open in plain words; needed before step 5 only.
   → Landed: Part 5 (K2a, K2b), Decisions needed.
2. Step 5's "merge main" in a worktree under `.gate/`, pushed from there. → Landed: Part 8 step 5.
3. `cancel-in-progress` stated; each planted failure pushed alone and left red before its revert. → Landed: 4.1,
   Part 7's procedure.
4. security-auditor's brief names the dependency review (package count, install scripts, `resolved` addresses).
   → Landed: E11, Part 8 step 7.
5. Before K2 is written, check that ADR 0002's Status line on main records part 1 as accepted. → Landed: Part 8 step
   5, 1.3.
6. Low items, folded in: F-3 (the probe judged on TCP, DNS printed: 4.2, 4.3, N5); F-5 (the active-toolchain check,
   `rustc --version` inside the cut: 4.2, 4.3); F-10 (`lo` up as root before the drop: 4.2, 4.3); F-4 (the control
   and probe on github.com: 4.1); F-6 (the Layout row: Part 5); F-7 (each agent at its file's effort: Part 8); F-11
   (local cargo runs under `.gate/`: Part 7); engine-architect and graphics-engineer briefed (1.3, step 7);
   V2_PHASE0_BRIEF stays stale, said in the report (1.3, step 7).
Its rulings: Q2.2-a (a) final ("G0 is a stage of the gate v2, and PM-0002-39 already runs it under `--prove`"; the
register entry needs his go); Q2.2-c (a) Node 22 final, with the dated follow-up (step 6); the three changes from the
sketch approved (the lint tools to 2.3, `.gitignore` to `/target/` and `test-results/`, the job in its own file); the
planted-failure commits approved with condition 3; plan 1.3 holds; Part 5 complete on the guard's marks (reproduced);
FDN-NOCODE needed; the globs left to 2.5 justified; the workflow draft holds (permissions, pins, no secrets, no caches,
no `pull_request_target`, the order sudo, unshare, setpriv, a real control), with conditions 3 and 6.
Its own checks: the marks on 35 paths; every cited line; the npm versions; `rustup toolchain install --help` ("by
default the active toolchain"); util-linux 2.39.3 in an Ubuntu 24.04.5 container; the research's claims quoted or, for
two inferences, proved by L3 and E2.
The advisor tool (Fable 5.2) is not among this session's tools, so it was not consulted before the plan was committed
nor before it was called done; his words make it an extra, and this recorded review is the required one.

## Part 10. Risks
1. The runner refuses part of the cut (`sudo -n`, `ip link set lo up`, `setpriv`'s options, `nogroup`): E2 shows it on
   the first push; the build stops for the advisor's ruling (4.4).
2. A red `toolchain` job on main is not a rollback trigger: CLAUDE.md's Rollback names `gate` and `smoke`. Until 2.3's
   one required check, a red `toolchain` on main is fixed forward, plan first; advisor.md:322 already forbids merging
   with "failing or pending checks", so the job blocks merges by the rules, though not by branch protection or the
   guard (which reads `gate` and `smoke`, `routine-guard.mjs:1486-1490`).
3. The job adds a few minutes of CI to every push and pull request (no path filters, no caches); E2 reports the time.
4. The pins have no guard protection until 2.3 (his answer); the PR Advisor's allow-list still refuses them.
5. npm's `latest` TypeScript is 7: a bare `npm install typescript` would take it; E7 checks the major.
6. Built before 2.1b merges, the branch brings main in by step 5's worktree path and re-runs everything; the kit lines
   are written last, on the then-current text (2.1b edits `CLAUDE.md:19`, next to K3).
7. `@playwright/test` running one test with no browser installed is assumed (A2); if it needs a browser, `e2e`'s stub
   test runs under Vitest until 2.3 brings G12's browsers.
8. `npm ci --offline` from a cache filled by the online `npm ci` is assumed to need nothing more (A3); E2 proves it.
9. The new workspace changes nothing the legacy gate reads (`gate.mjs:34` lists the five files); E5 and E6 check it.
10. The npm tree is new code from the internet: the pinned lockfile, `--ignore-scripts` everywhere and E11's review
    (condition 4) are its guard until 2.3's checks.

## Part 11. What Mustafa checks
No Brave check: 2.2 adds no page and nothing a player sees, hears or feels. His actions: answer FDN-NOCODE (before step
5); start the build when he chooses; approve the kit asks as they come (K1, K2a, and K2b, K3 and K4 if he answers
FDN-NOCODE (a)); his go on the final head after 2.1b has merged.

## Decisions needed (Mustafa)
- **FDN-NOCODE: does your freeze exception for the foundation (FREEZE-FDN) also cover adding your 2026-11-10
  exception to the three "no engine code" lines, `CLAUDE.md:20-22`, `CLAUDE.md:52` and
  `.claude/agents/advisor.md:107`?**
  - Why it matters: once 2.2 merges, main holds v2 crates while those three lines still say "no engine code"; the
    advisor works from its own file and would read 2.3's plan against it.
  - The texts: the FREEZE-FDN option (a) you picked listed, for 2.2, "its CI job, the constraint-scope lines and the
    rules' globs"; your words on main say the freeze "does not stop the smallest edits the foundation needs to keep the
    kit true after its own moves (paths in the gate and in CLAUDE.md, the toolchain's CI job, the lines that scope the
    legacy constraints)". The SCOPE option (a) you picked (`d389d91:.checkpoints/foundation/plan.md:524-536`) said
    "... and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2
    code" lines"; your reply to it ("I accept ADR 0002's part 1 alone ...") does not repeat that clause.
  - Options: (a) yes: each line gets one clause, each edit asks you first and merges on your go; (b) no: the three lines
    stay until the kit unfreezes, and every plan and review until then cites your decision as the exception.
  - Recommendation: (a).
  - Needed before step 5 only, not for the build.
- Already answered in this session's prompt: the CI job is covered; guard checks for the new pins wait for 2.3; 2.2
  merges after 2.1b. Not asked now: the rules' globs (2.5's plan asks).

## Appendix A. Cited lines, re-found on main 2d22b69
- ADR 0002: part 1 `:137-156`; part 2 `:157-181`; part 3 `:182-246`; part 4 `:247-278`; part 7 `:416-484` (the
  per-edit hook `:472-474`); part 8 `:485-561`; part 9 `:562-654` (G0 `:569`, G9 `:578`); OQ-0002-1 `:986-997`;
  OQ-0002-3 `:1008-1021`; OQ-0002-4 `:1022-1034`; OQ-0002-7 `:1063-1071`; spike item 4 `:1144-1148`, item 8
  `:1165-1171`; PM-0002-37 `:1289`, -38 `:1290`, -39 `:1291`, -62 `:1314`; migration plan 2.2 `:1432-1438`.
- SPIKE_PLAN foundation entry `:420-442`. CLAUDE.md `:19`, `:20-22`, `:41-43`, `:45-53`. advisor.md `:107` (was `:98`
  at 44446a5), `:322`, `:323` (was `:313`), the revert steps `:280-290`. `engine-architect.md:52`,
  `graphics-engineer.md:41`. STATUS: KIT-FREEZE-2 and FREEZE-FDN `:680-690`, LANE-RULES `:700-710`, SPIKE-SCOPE (a)
  `:918-919` (was `:739-746`), SLICE-ORDER condition 3 `:827` (was `:654`). V2_PHASE0_BRIEF `:20-22`, `:144`.
  NIGHT_SHIFT `:54-55`. `syntax-gate.mjs:3`, `:21-23`. `gate.mjs:34`, `:115`. `routine-guard.mjs:28-29`, `:35-48`,
  `:80-100` (`mergeAllowed` and its lists), `:1483-1490` (the merge check reads `gate` and `smoke`). Rules' globs:
  `pages.md:3-4`, `engine-core.md:3`, `engine-game.md:3`, `engine-editor.md:3`, `netcode.md:3-5`.
- The SCOPE options at d389d91: `.checkpoints/foundation/plan.md:524-536`.
- PR #40 at 411eeee: `spikes/netcode-proof/rust-toolchain.toml` (1.99.1, minimal, clippy and rustfmt, wasm32);
  `check.mjs:1226-1245` (`cargoConfigFiles`), `:1250-1264` (`referenceProblems`); `.cargo/config.toml` sets only
  `target-dir`; the PR changes nothing outside `spikes/` (no `.github/`, `.claude/`, `tools/` or root file).

## Appendix B. Findings from this plan's research and review
- N1: the runner image's default Rust is 1.99.1, equal to the pin; the job checks the active toolchain's source (4.3).
- N2: the runner's docker group would reach the Docker socket from inside the cut; the group is dropped (4.3).
- N3: npm's docs do not state that a workspace without a `test` script fails `npm test --workspaces` (L3 proves it).
- N4: rustup auto-installs by default since 1.28.1; the job sets `RUSTUP_AUTO_INSTALL=0` (4.1).
- N5: a name may resolve inside the cut through a local resolver's socket; the probe judges a TCP connection (4.3;
  the advisor's F-3).
- A1-A3 (assumed, proved in the build): `sudo -n unshare` with `ip link set lo up` and `setpriv` works on the runner
  (E2); Playwright Test runs a browserless test without browsers (E1); `npm ci --offline` needs only the cache the
  online `npm ci` filled (E2).
- The local rehearsal of `unshare` in this session's container was refused by the session's permission check and not
  retried; nothing in the plan rests on it.
