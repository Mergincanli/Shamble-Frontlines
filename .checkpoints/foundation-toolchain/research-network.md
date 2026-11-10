# Research notes: how a GitHub-hosted runner can run steps with the network cut off (task foundation-toolchain)
Plan of blocks: B1 GitHub runner docs (sudo, VM, container network option, image contents) · B2 Linux tools (unshare,
setpriv, no_new_privs, a new network namespace's contents) · B3 npm, Cargo, rustup offline behaviour · B4 Node
release lines (Q2.2-c) · B5 the design and its proof runs.
All quotes were read on 2026-11-10 by web-fetch agents (pages loaded, HTTP 200); "not found" means the page was read
and does not say it.

## B1 GitHub runner docs (done)
- https://docs.github.com/en/actions/reference/runners/github-hosted-runners
  - "The Linux and macOS virtual machines both run using passwordless `sudo`." (found)
  - "each GitHub-hosted runner is a new virtual machine (VM) hosted by GitHub." Exception: "Single-CPU runners are
    hosted in a container on a shared VM" (`ubuntu-slim`). `ubuntu-24.04` is a listed standard label (found).
  - "A GitHub-hosted runner must establish connections to GitHub-owned endpoints to perform essential communication
    operations." (found) -> a firewall rule for the whole VM or the runner's user would cut the runner itself; a
    namespace for one child process does not.
- https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/run-jobs-in-a-container
  - "The `--network` and `--entrypoint` options are not supported." (found) -> no `container.options: --network none`.
- https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2404-Readme.md
  - "OS Version: 24.04.5 LTS", "Image Version: 20261101.360.1"; Rust tools: "Cargo 1.99.1", "Rust 1.99.1",
    "Rustdoc 1.99.1", "Rustup 1.29.2" (found). The image's default Rust equals the pin today, so a version check alone
    cannot show the toolchain came from rust-toolchain.toml (finding N1).
  - iproute2 "6.1.0-1ubuntu6.4" listed; util-linux, unshare, setpriv: not listed (the page lists added packages; the
    job checks `command -v unshare setpriv` and fails if absent).
  - "`ubuntu-latest` label will use Ubuntu 26.04 in November 2026" -> pin `ubuntu-24.04`, as gate.yml does.
DONE B1, next B2.

## B2 Linux tools (done)
- https://github.com/util-linux/util-linux/blob/master/sys-utils/unshare.1.adoc
  - `-n, --net`: "Create a new network namespace." (found)
  - `-S, --setuid`: "Set the user ID which will be used in the entered namespace."; `-G, --setgid`: "Set the group ID
    which will be used in the entered namespace and drop supplementary groups." (found)
  - The privileges unshare itself needs: not found on the page (only `-r, --map-root-user`: "... even when run
    unprivileged."). So: run it under sudo (B1's passwordless sudo); never rely on unprivileged user namespaces.
- https://github.com/util-linux/util-linux/blob/master/sys-utils/setpriv.1.adoc
  - `--nnp, --no-new-privs`: "Set the no_new_privs bit. With this bit set, execve(2) will not grant new privileges."
    "This bit is inherited by child processes and cannot be unset." (found)
  - `--reuid`: "Set the real, effective, or both UIDs."; `--regid`: "... For safety, you must specify one of
    --clear-groups, --groups, --keep-groups, or --init-groups if you set any primary gid."; `--clear-groups`: "Clear
    supplementary groups." (found). `--reset-env` exists; the examples say "To also reset the environment (as sudo(8)
    does by default), add --reset-env:" (setpriv keeps the environment unless told).
- https://github.com/torvalds/linux/blob/master/Documentation/userspace-api/no_new_privs.rst
  - "With no_new_privs set, execve() promises not to grant the privilege to do anything that could not have been done
    without the execve call." "For example, the setuid and setgid bits will no longer change the uid or gid; ..."
    "Once the bit is set, it is inherited across fork, clone, and execve and cannot be unset." (found) -> code inside
    the cut cannot regain root through sudo to leave the namespace.
- https://man7.org/linux/man-pages/man7/network_namespaces.7.html and .../man8/ip-netns.8.html
  - What a new network namespace contains (loopback only, down): not found on either page; ip-netns's example runs
    "ip netns exec vpn ip link set lo up" ("Bring up the loopback interface ..."), which implies lo starts down. The
    plan does not rest on it: the job prints `ip -o link` inside the cut and probes the network itself (B5).
- Finding N2: clearing supplementary groups matters: the runner user's docker group would let a process inside the
  cut reach the Docker daemon's Unix socket (not network-namespaced) and run a container with the host network.
  `--clear-groups` drops it. (Reasoning, not a quote: Unix sockets on the file system are reachable from any network
  namespace; the docker group on the image is an assumption to check at build: `id` printed before the cut.)
DONE B2, next B3.

## B3 npm, Cargo, rustup offline behaviour (done)
- https://docs.npmjs.com/cli/v11/using-npm/config
  - `offline`: "Force offline mode: no network requests will be done during install." "To allow the CLI to fill in
    missing cache data, see `--prefer-offline`." (found)
  - `ignore-scripts`: "If true, npm does not run scripts specified in package.json files." ... "will still run their
    intended script if `ignore-scripts` is set, but they will _not_ run any pre- or post-scripts." (found)
  - `update-notifier`: "Set to false to suppress the update notification ..." (found); `audit`, `fund` default true.
  - `if-present`: "If true, npm will not exit with an error code when `run` is invoked for a script that isn't
    defined in the `scripts` section of `package.json`." default false (found).
- https://docs.npmjs.com/cli/v11/commands/npm-ci
  - "If dependencies in the package lock do not match those in `package.json`, `npm ci` will exit with an error instead
    of updating the package lock." "If a `node_modules` is already present, it will be automatically removed before
    `npm ci` begins its install." (found)
- https://docs.npmjs.com/cli/v11/commands/npm-test and .../npm-run
  - That a workspace with no `test` script fails `npm test --workspaces`: not stated (inferred from `if-present`'s
    default false). Finding N3: the build proves it with a mutation (a package without a test script must fail).
- https://doc.rust-lang.org/cargo/commands/cargo-test.html
  - `--offline`: "Prevents Cargo from accessing the network for any reason." "Without this flag, Cargo will stop with
    an error if it needs to access the network and the network is not available." `--locked`: "Asserts that the exact
    same dependencies and versions are used as when the existing `Cargo.lock` file ..." (fails when "The lock file is
    missing." or "Cargo attempted to change the lock file ..."). `--frozen`: "Equivalent to specifying both `--locked`
    and `--offline`." (found)
- https://doc.rust-lang.org/cargo/commands/cargo-fetch.html
  - "Subsequent Cargo commands will be able to run offline after a `cargo fetch` unless the lock file changes."
    "If `--target` is not specified, then all target dependencies are fetched." (found)
- rustup CHANGELOG (https://raw.githubusercontent.com/rust-lang/rustup/refs/heads/main/CHANGELOG.md)
  - 1.28.0: "rustup will no longer automatically install the active toolchain if it is not installed." "To ensure its
    installation, run `rustup toolchain install` with no arguments." (found)
  - 1.28.1: "This is a patch release to restore the automatic install behavior by default." "Automatic install is
    enabled by default but can be opted out by setting `RUSTUP_AUTO_INSTALL` environment variable to `0`." (found)
  - 1.28.2: "`rustup set auto-install disable` can now be used to disable automatic installation of the toolchain."
- https://rust-lang.github.io/rustup/overrides.html
  - Precedence: "A toolchain override shorthand used on the command-line, such as `cargo +beta`." / "The
    `RUSTUP_TOOLCHAIN` environment variable." / "A directory override, set with the `rustup override` command." / "The
    `rust-toolchain.toml` file." / "The default toolchain." (found). "If both files are present in a directory, the
    latter is used for backwards compatibility." (`rust-toolchain` beats `rust-toolchain.toml`).
  - `rustup toolchain install` with no arguments: not on this page (the 1.28.0 changelog is its source).
- Finding N4: auto-install is on by default since 1.28.1 (the image has rustup 1.29.2), so "before any cargo call" is
  only an order unless the job sets `RUSTUP_AUTO_INSTALL=0`; with it, the explicit install is the only way the
  toolchain arrives, and a later toolchain switch fails instead of downloading.
DONE B3, next B4.

## B4 Node release lines, for Q2.2-c (done)
- https://github.com/nodejs/Release (README table; "Dates are subject to change."):
  22.x "Maintenance LTS", Jod, initial 2024-04-24, active LTS 2024-10-29, maintenance 2025-10-21, end-of-life
  2027-04-30; 24.x "Maintenance LTS", Krypton, 2025-05-06, 2025-10-28, 2026-10-20, 2028-04-30; 26.x "Active LTS",
  Lithium, 2026-05-05, 2026-10-28, 2027-10-20, 2029-04-30. On 2026-11-10 only 26.x is Active LTS.
- The npm registry, read 2026-11-10 with `npm view` (this container): typescript dist-tag latest "7.0.2"; the 6.0 line
  has 6.0.2 and 6.0.3; vitest 5.1.0, engines `^22.12.0||^24.0.0||>=26.0.0`; @playwright/test 1.65.0, engines `>=20`.
DONE B4, next B5 (the design, written into plan.md).
