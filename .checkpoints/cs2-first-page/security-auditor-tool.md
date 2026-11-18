# security-auditor: wasm-bindgen CLI 0.2.130 origin check (cs2-first-page, started 2026-11-18 21:31 UTC)

Plan: read plan.md s6-7 + advisor F2; read the 0.2.130 release page/assets via WebFetch (data only); fetch only the small text proof files with curl into .gate/cs2-first-page/tool-check/; check what each binds and whether it can be verified here (openssl/python/node, no new tools); count wasm-bindgen-cli 0.2.130's locked crates (build scripts, proc macros) from published metadata only; answer Q1-Q4; each finding saved below with its source.

## Findings (saved as made)
- 21:3x S1 release page https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.130 (WebFetch, data only): tag 0.2.130,
  published by github-actions 30 Oct 06:16, commit c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2 (GitHub-signed commit, GPG
  B5690EEEBB952194); badge "Immutable release. Only release title and notes can be modified."; "Assets 17" (lazy list
  failed to load in fetch); no attestation/provenance text on the page itself; notes say nothing on binaries/checksums.
- 21:34 S2 expanded assets https://github.com/wasm-bindgen/wasm-bindgen/releases/expanded_assets/0.2.130 (WebFetch; digests
  as transcribed by the fetch model, UNVERIFIED until compared with curl'd files): 17 entries = 7 targets x (tar.gz +
  .tar.gz.sha256sum) + "Source code (zip)" + "Source code (tar.gz)" + "Release attestation (json)". Targets: aarch64-apple-darwin,
  aarch64-unknown-linux-gnu, aarch64-unknown-linux-musl, riscv64gc-unknown-linux-gnu, x86_64-apple-darwin,
  x86_64-pc-windows-msvc, x86_64-unknown-linux-musl. GitHub-shown sha256 (fetch model): x86_64-unknown-linux-musl.tar.gz
  9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f (11.3 MB, 2026-10-30T06:11:06Z); its .sha256sum
  e7ae0bd6eb9706637bab3db6991d4023617d0f4b213a0515134b5f42266993f8 (131 B). Source code + attestation dated 05:45:20Z.
  So "release attestation" = GitHub immutable-release attestation (not a project-made sigstore asset).
- 21:34 S3 curl of the musl .sha256sum (release download URL -> 302 release-assets.githubusercontent.com, Azure blob,
  Last-Modified 30 Oct 2026 06:11:07 GMT, 131 B): content = "9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f  gh-release/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz\n";
  local sha256 of the file = e7ae0bd6...93f8 = GitHub-shown digest of that asset; the tarball digest inside = GitHub-shown
  tarball digest. So the fetch model's transcription holds for these two, and the project's .sha256sum agrees with GitHub's
  server-computed asset digest. Note: name field carries a "gh-release/" prefix (sha256sum -c on the file as-is fails;
  the plan's printf form is right). Both values are SAME ORIGIN as the tarball (the release), proof of integrity, not origin.
  Saved: .gate/cs2-first-page/tool-check/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz.sha256sum (+ headers).
