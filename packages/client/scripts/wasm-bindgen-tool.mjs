// The one wasm-bindgen command-line tool this repository's build may run, by its exact values. The tool is a
// ready-built program from the internet that no lockfile covers, so these values are its lock: the build compares
// the program's size and sha256 with them before every run, `--version` included (build-checks.mjs, checkCliFile),
// and the CI edit that brings the tool to CI (a pull request of its own) checks the archive's.
//
// Mustafa's words of 2026-11-18 (docs/DECISIONS.md, TOOL-SOURCE): "(a), for this one file only:
// wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz, 11860397 bytes, sha256
// 9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f [...] My rule on unreviewed code from the
// internet stands as it is: every other file, version or checksum comes back to me." And: "Inside it the program
// wasm-bindgen has 8170784 bytes and sha256 ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931."
// So a change to any value below is a question for him first, and a version bump is its own change-set.

export const WASM_BINDGEN_TOOL = Object.freeze({
  /** Equal to the `wasm-bindgen` library in Cargo.lock; the build refuses any other (PM-0002-18). */
  version: "0.2.130",
  archive: Object.freeze({
    url: "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/0.2.130/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz",
    bytes: 11860397,
    sha256: "9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f",
  }),
  /** The one member of the archive that is taken out and run. */
  program: Object.freeze({
    member: "wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm-bindgen",
    bytes: 8170784,
    sha256: "ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931",
  }),
});
