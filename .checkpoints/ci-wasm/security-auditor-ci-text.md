Task: security-auditor, part "ci-text": security reading of the proposed CI-WASM workflow text (toolchain.yml) before it goes to Mustafa (2026-11-19).

## Plan (blocks, in order)
- B1. Read brief-context.md (done at start), ci-edit.diff, the whole proposed toolchain.yml, today's toolchain.yml, ci-plant-p-gh.diff.
- B2. Read plan.md sections 5, 9, 10, 16.
- B3. Read infra-engineer-ci-plan.md blocks 2, 3, 4, 7 and "Not verified".
- B4. Read my earlier tool reading (.gate/ci-wasm/hist/security-auditor-tool.md) and tool-route.sh.
- B5. Read build.mjs, build-checks.mjs, wasm-bindgen-tool.mjs, playwright.config.ts.
- B6. Read docs/reports/2026-11-18-kit-hold-ci.md (guarantees not to loosen).
- B7. Local checks of shell lines on scratch files under .gate/ci-wasm/security/ (awk on tar -tv listing, sort -V -C, sed on curl -w, grep -qxF, pipefail behaviours).
- B8. Official docs reads (docs.github.com: GITHUB_TOKEN on forks, permissions, attestations; cli.github.com: gh attestation verify, env vars GH_HOST/GH_REPO/GH_ENTERPRISE_TOKEN/GH_CONFIG_DIR; the gh advisory). Summaries saved as read.
- B9. Threat model and per-question analysis (Q1..Q7), findings with line numbers and replacement text.
- B10. Answer drafted in notes; DONE.

## Notes (findings and block completions)
- B1 partly: brief-context.md read. Next: the diff and proposed file.
- B1 done: diff (90 added), proposed file (190 lines; tool steps at lines 74-118, cut additions 138, 164-189), today's file (100 lines), P-GH plant (2 lines before the real check). Next: plan.md.
- B2 done: plan.md sections 0, 5, 9, 10, 16 read. Key claims to test: s5 item 3 wording of what verify-asset proves ("byte for byte an asset GitHub recorded and signed for that immutable release; does not prove how the program was built"); gh floor 2.93.0 for CVE-2026-48501 (image lists 2.103.3); PERM-ATT recommends (a) no attestations:read; P-GH should fail with gh's own line and answer PERM-ATT; CHROME-SOURCE recommends (a) image Chrome 157.0.8103.50. Next: infra-engineer notes.
- B3 done: infra-engineer blocks 2-8 + end sections read. Its sources: S2.1-S2.15 (gh manual, verify-release-integrity, immutable-releases, runner readme: gh 2.103.3, Chrome 157.0.8103.50, image 20261111.370.1; advisory GHSA-8xvp-7hj6-mcj9 affected <=2.92.0 patched 2.93.0, hosts tuf-repo.github.com, tuf-repo-cdn.sigstore.dev, tmaproduction.blob.core.windows.net; verify_asset.go trunk flow tag->commit->attestations by digest(PredicateType release)->subject->VerifyAttestation; attestation.go SAN ^https://dotcom\.releases\.github\.com$, NoPublicGood; REST attestations: fine-grained needs attestations:read; permissions: unnamed -> none). Claims I must test: (i) P1 "gh reaches the subject comparison only after fetching and verifying the release's attestation" -> by S2.8's flow the subject comparison comes BEFORE VerifyAttestation (signature); (ii) "with no token gh exits 4"; (iii) runner passwordless sudo (S3.9); (iv) Playwright passes --no-sandbox (S3.4). Next: my earlier tool reading + tool-route.sh.
