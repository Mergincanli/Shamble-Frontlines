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
- 21:35 S4 release attestation href (from expanded_assets): https://github.com/wasm-bindgen/wasm-bindgen/attestations/61631161/download
  curl through the proxy -> CONNECT ok, then 403 Forbidden, application/json, 249 B, Connection: close; proxy status shows
  no relay failure (refusal is upstream: the egress policy or GitHub). Proxy README: do not retry 403 policy denials -> not
  retried with curl. So the attestation cannot be fetched as a file here by curl.
- 21:37 S5 GitHub docs, immutable releases https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/immutable-releases :
  "Immutable releases are releases where the assets and associated Git tag cannot be changed after publication."; tag
  "locked to a specific commit"; "creating an immutable release automatically generates a release attestation" "which is a
  cryptographically verifiable record of a release containing the release tag, commit SHA, and release assets." Page says
  nothing on format, signer, trust root or offline verification.
- 21:37 S6 GitHub docs, verify release integrity https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/verify-release-integrity :
  only `gh release verify RELEASE-TAG` and `gh release verify-asset RELEASE-TAG ARTIFACT-PATH` ("verify a local artifact is
  an exact match for a release asset"); nothing on Sigstore, trusted root, offline, or without the GitHub CLI.
- 21:37 S7 the attestation itself, read through WebFetch (a different route from the container's curl; fetch-model
  answers, so structure only): mediaType "application/vnd.dev.sigstore.bundle.v0.3+json"; verificationMaterial has
  `certificate` + `timestampVerificationData` (1 RFC 3161 timestamp), NO tlogEntries, no chain; dsseEnvelope payloadType
  "application/vnd.in-toto+json", signature 96 b64 chars (~72 B: ECDSA P-256 DER size); payload starts
  "eyJfdHlwZSI6Imh0dHBzOi8vaW4tdG90by5pby9TdGF0ZW1lbnQvdjEiLCAic3ViamVjdCI6W3sidXJpIjoicGtnOmdpdGh1Yi93YXNtLWJpbmRnZW4vd2"
  = in-toto Statement v1, first subject uri "pkg:github/wasm-bindgen/w...". No Rekor entry + TSA timestamp = the shape of
  GitHub's own Sigstore instance (as for private-repo attestations), not public-good Sigstore. To confirm from the cert.
- 21:40 S8 attestation payload, transcribed through WebFetch into .gate/cs2-first-page/tool-check/attestation-payload.webfetch.b64
  (3,560 b64 chars -> 2,670 B, sha256 53314ce3...e536; decodes to valid JSON; decoder kept in the session scratchpad
  sec-tool/decode_payload.py, run with python3 -I): in-toto Statement v1, predicateType
  "https://in-toto.io/attestation/release/v0.2", predicate {purl pkg:github/wasm-bindgen/wasm-bindgen@0.2.130, repository
  wasm-bindgen/wasm-bindgen, repositoryId 114685405, ownerId 220058222, databaseId 428706067, tag 0.2.130}; 15 subjects:
  the purl with sha1 c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2 (the tag's commit) + the 14 binary/.sha256sum assets with
  sha256. musl tarball subject 9687f3bb...227f = .sha256sum content = GitHub-shown digest; .sha256sum subject e7ae0bd6...93f8
  = local sha256 of the curl'd file. Source-code archives are not subjects (GitHub docs S6: made on request).
  So the attestation BINDS: tag -> commit -> asset name -> sha256. It says NOTHING about how the binary was built (no
  workflow, runner or source-to-binary link). Signature not yet checked (needs exact bytes + trust root).
- 21:42 S9 signing certificate (bundle's verificationMaterial.certificate.rawBytes, 748 b64 chars, transcribed through
  WebFetch in 8 chunks into tool-check/attestation-cert.webfetch.b64; DSSE sig 96 chars, no keyid, into
  attestation-sig.webfetch.b64). openssl x509 -text: Issuer "O=GitHub, Inc., CN=Fulcio Intermediate l1"; Subject
  "O=GitHub, Inc., CN=Attester"; serial 14:4b:1b:8a:...:d3:43; valid 2026-06-12 to 2027-06-12 (one-year cert, not a
  10-minute Fulcio identity cert); P-256 key; KU digitalSignature (critical); EKU codeSigning; CA:FALSE; AKI
  C0:E1:B8:52:A4:4B:4F:3E:DE:7A:38:11:06:CF:49:8E:A1:45:9B:33; SAN URI https://dotcom.releases.github.com; signed
  ecdsa-with-SHA384. = GitHub's own Sigstore instance (GitHub's CA), signer identity "GitHub releases", not a workflow.
- 21:43 S10 DSSE signature CHECKED here (python3 -I + openssl only): PAE = "DSSEv1 28 application/vnd.in-toto+json 2670 "
  + payload (2,714 B); `openssl dgst -sha256 -verify attester-pub.pem -signature sig.der pae.bin` -> "Verified OK" (sig =
  DER ECDSA, 70 B). Negative control: musl digest's first digit 9->8 in the PAE -> "Verification failure", exit 1.
  Proves: payload + signature + cert public key were transcribed exactly, and the statement (tag 0.2.130 -> commit
  c93b8ae... -> musl tarball sha256 9687f3bb...227f) was signed by the "Attester" key of the cert. NOT proved here: that
  the cert chains to GitHub's root (needs "Fulcio Intermediate l1" + root from GitHub's trusted_root.json, TUF repo
  tuf-repo.github.com, root embedded in gh) and the RFC 3161 timestamp (needs the TSA chain from the same trusted root).
  Cert bytes outside the key (names, SAN, extensions) are only checked by that chain step.
- 21:45 S11 release pipeline at the tag commit (WebFetch of raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/c93b8ae.../.github/workflows/{release-tag,main}.yml;
  fetch-model quotes, data only): release-tag.yml (on CI completed on main; permissions {} / contents: read; creates the tag
  with a GitHub App token "permission-contents: write" when Cargo.toml's version changed). main.yml on the tag push: 7 dist_*
  jobs on GitHub-hosted runners, e.g. dist_linux_x86_64_musl: `cargo build --manifest-path crates/cli/Cargo.toml --target
  x86_64-unknown-linux-musl --features vendored-openssl --release` (no --locked shown); uses actions/checkout@v7,
  dtolnay/rust-toolchain@stable, actions/upload-artifact@v7 (tags, not SHAs). deploy job (contents: write): `tar czvf
  gh-release/$name.tar.gz -C tmp $name` + `sha256sum gh-release/$name.tar.gz > gh-release/$name.tar.gz.sha256sum` (explains
  the "gh-release/" prefix), draft release via softprops/action-gh-release@v3. publish job: environment "publish" (approval
  per the release-tag comment), id-token: write, rust-lang/crates-io-auth-action@v1 = crates.io TRUSTED PUBLISHING,
  `./publish publish`. release job: `diff built uploaded` then `gh release edit ... --draft=false --latest`.
  "No attestation step exists" (no attest-build-provenance / cosign): NO build provenance for the binaries. The only signed
  statement is GitHub's release attestation (integrity vs the release, not how it was built).
- 21:46 S12 crates.io API https://crates.io/api/v1/crates/wasm-bindgen-cli/0.2.130 (WebFetch): created_at
  2026-10-30T06:15:16.195867Z; checksum c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a; crate_size 56,655;
  not yanked; MIT OR Apache-2.0; rust_version 1.86; published_by null; trustpub_data {provider github, repository
  wasm-bindgen/wasm-bindgen, run_id 41011884567, sha c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2} = SAME commit as the release
  tag; bins wasm-bindgen, wasm-bindgen-test-runner, wasm2es6js; features default ["rustls-tls"] (ureq/rustls), native-tls,
  openssl (dep:native-tls), vendored-openssl ["openssl","native-tls/vendored"]. Timeline: assets uploaded 06:11:06-07Z,
  crate published 06:15:16Z, release published 06:16: one tag run. The prebuilt Linux binaries are built with
  --features vendored-openssl ON TOP of default rustls (OpenSSL C sources compiled in); `cargo install` default = rustls only.
- 21:48 S13 crates.io API https://crates.io/api/v1/crates/wasm-bindgen/0.2.130 (WebFetch): created_at 2026-10-30T06:15:04.504224Z,
  checksum a0181c14ddd6c7b8b0b0132a5f1ea2f32be0d893cd3ea97b6e3b28886b8a3d1c (= plan :836's a0181c14...3d1c), crate_size
  81,643, published_by null, trustpub_data {github, wasm-bindgen/wasm-bindgen, run_id 41011884567, sha c93b8ae...} = the
  SAME run and commit as the CLI crate and the release tag. So library, CLI crate and release binary share one origin
  (one workflow run on one commit); only the crates have a trusted-publishing record, the binary has none.
- 21:48 S14 docs.rs source view https://docs.rs/crate/wasm-bindgen-cli/0.2.130/source/Cargo.lock (WebFetch list, fetch-model
  transcription; my own count below): the CLI crate's published Cargo.lock.
- 21:50 S15 my count of the S14 list (scratchpad sec-tool/cli-lock-list.txt): 254 [[package]] entries, 254 unique
  name+version. By name: 17 proc-macro crates (clap_derive, defmt-macros, displaydoc, jiff-static, openssl-macros,
  rstest_macros, rustversion, serde_derive, thiserror-impl, walrus-macro, wasm-bindgen-macro, windows-implement,
  windows-interface, yoke-derive, zerocopy-derive, zerofrom-derive, zerovec-derive); 31 entries are Windows/macOS/other-OS
  only; dev-only entries too (assert_cmd, predicates*, rstest*, pretty_assertions ...); vendored-openssl-only entries
  (openssl*, native-tls, foreign-types*, openssl-src = OpenSSL C sources). Already in our Cargo.lock: 5 (proc-macro2,
  quote, syn x2 (syn 3.0.9 in the CLI lock vs our syn 3.0.10), thiserror, thiserror-impl, unicode-ident).
  Build scripts: not countable from published metadata without fetching each crate (estimate from known crates: ~30-40,
  incl. ring's C/assembly build, proc-macro2, quote, serde, serde_json, libc, rustix, anyhow, thiserror, zerocopy,
  rustversion, httparse, mime_guess, semver, num-traits, crossbeam-utils, rayon-core, portable-atomic, wasm-bindgen,
  wasm-bindgen-shared, windows_* x8). Exact numbers need `cargo metadata` after a fetch-only step (dependency answer).
- 21:52 S16 crates/cli/Cargo.toml at c93b8ae (WebFetch restatement, not verbatim): bins wasm-bindgen,
  wasm-bindgen-test-runner, wasm2es6js; features default = rustls-tls (ureq/rustls), native-tls, openssl, vendored-openssl
  (both "legacy"); deps anyhow, clap 4 (derive), env_logger 0.11.5, log, native-tls (optional), rouille 3 (no default
  features), serde (derive), serde_derive, serde_json, shlex, tempfile, ureq 3 (default-features = false; brotli, gzip),
  walrus 0.27.2, wasm-bindgen-cli-support =0.2.130, wasm-bindgen-test-shared =0.2.130, wasmparser 0.245; no
  target-specific tables. So `--no-default-features` drops TLS (rustls, ring and its C/assembly build) and the
  wasm-bindgen bin still works (TLS serves only the test runner's remote webdriver traffic).
- 21:52 S17 main.yml deploy/release scripts at c93b8ae (WebFetch, quoted): artifacts via actions/download-artifact@v8;
  `mk()`: `mkdir -p tmp/$name`, removes *.d/*.pdb, `cp README.md LICENSE-MIT LICENSE-APACHE artifacts/$src/wasm* tmp/$name/`,
  `chmod +x tmp/$name/wasm*`, `tar czvf gh-release/$name.tar.gz -C tmp $name`, sha256sum line. Expected members: folder
  wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/ + README.md, LICENSE-MIT, LICENSE-APACHE, wasm-bindgen,
  wasm-bindgen-test-runner, wasm2es6js (whatever matched wasm*). Release job's check: `gh release view ... --jq
  '.assets[].name' | sort > uploaded; ls gh-release | sort > built; diff built uploaded` = compares NAMES only, not content.
- 21:53 S18 https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/c93b8ae.../Cargo.lock -> 404 (per WebFetch): the repo
  commits no Cargo.lock and the dist jobs build without --locked, so the release binary's dependency versions were resolved
  at CI time and are recorded nowhere we can read; the crate's published Cargo.lock (S14) is the only recorded resolution.
- 21:54 S19 container inventory (no tool run beyond --version of openssl/python/tar): gh at /usr/local/bin/gh (session rule:
  code search only, so no `gh release verify-asset` / `gh attestation`), openssl 3.0.13, python 3.13.18 (tarfile
  filter='data' available), GNU tar 1.35, readelf, file, strings; no cosign/sigstore CLI. npm's bundled @sigstore/{bundle,
  core,protobuf-specs,sign,tuf,verify} exist, but @sigstore/tuf/seeds.json seeds ONLY https://tuf-repo-cdn.sigstore.dev
  (public-good): GitHub's instance (tuf-repo.github.com) is not seeded, so nothing here can anchor "Fulcio Intermediate l1".
