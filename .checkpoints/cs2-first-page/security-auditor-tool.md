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
