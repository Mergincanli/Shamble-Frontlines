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
