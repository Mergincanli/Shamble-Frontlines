# Change-set 3b: the one batch of manifest changes, and the answer it was held for

- **What it is:** path dependencies between our own crates, and one comment. No package from outside the repository
  enters; no version, source or checksum of a registry package changes. `room` and `client-endpoint` each take
  `protocol` and `sim`; `sim-wasm` takes `protocol` and `room`; `devtools` takes `client-endpoint`, `protocol`
  and `room`; the root manifest's comment on wasm-bindgen now names both crates that use it (the follow-up
  3A-MANIFEST-COMMENT). The lockfile was made by `cargo update --workspace --offline`, which printed "Locking 0
  packages": only our own crates' dependency lists change.
- **Held** uncommitted and unbuilt until security-auditor (Opus 5.5, effort max) had answered on exactly this content.
  Its first line: "The dependency change may be committed and built: yes". No security finding, no condition; three
  Lows (wording of two comments; a pointer for the later cargo-deny file). Its notes: `security-auditor-deps.md`.
- **The six files, sha256, as answered on and as committed** (measured again right before the commit, 2026-11-24, 23:41 UTC):
  - `Cargo.toml` 593297a811501a67fc87273b795da4d423fa1bb0086d30d988d96418958489a8
  - `Cargo.lock` a73653ea4f8a14f0849d23cf43ecae308240d1fb96b48bb5656609e6157562b8
  - `crates/room/Cargo.toml` 83aeccc17e2b120af4814a3e589c71b0c213ad5bd1d57185fb8bcc566cbe14ae
  - `crates/client-endpoint/Cargo.toml` ccf21ea7c637b78fa3ee4277dc52fd69bf9830d2c58677ca262f7ebf78601c49
  - `crates/sim-wasm/Cargo.toml` 5105930087245dff087bffdbbe68ee53cf10582169a8470e41e6c14e7f0f50f5
  - `crates/devtools/Cargo.toml` 006a3b7ae0a60cc65b1bc6b7dff2b529309f21bb27612f5fa903c76fc4a25b8e
- **Its Low L2, kept in sight:** `devtools`' three new edges and its comment serve the replay runner and the link
  simulator (the plan's steps 7 and 8). If the size ladder moves both out of 3b, that manifest is looked at again,
  and a change to it is a new batch under the hold.
