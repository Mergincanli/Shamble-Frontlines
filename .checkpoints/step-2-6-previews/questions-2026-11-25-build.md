# Questions for Mustafa from step 2.6's build (lane 2, Builder V2), 2026-11-25 evening

Saved under his rule of 2026-11-25, 01:35 Amsterdam, item 6: "From now on, when you wait on me: save the full question as
a file in your notes, push it, and name the file and the commit in your message."

## What waits on him
1. **NEW-TEXT: may one line of the approved preview.yml change?**
   - What happened: building the tests showed that the line `run: node --test hosting/` (the job `tests`, line 50)
     fails on Node 22. Node loads the folder as a module ("Error: Cannot find module 'hosting'", exit 1) instead of
     looking for tests in it. So with the approved text, the job `tests` would be red on every run, and none of the
     tests would run.
   - The new line: `run: node --test hosting/preview.test.mjs`. That command runs all 117 tests (exit 0); a missing
     file fails loudly (exit 1). Nothing else in the file changes.
   - The new text: `.checkpoints/step-2-6-previews/texts/preview-proposed.yml`, 10,577 bytes, sha256
     31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc (the approved one: 10,561 bytes, sha256
     c66c0b0a...). The full text follows below, after the decisions.
   - Options:
     - (a) Yes, the new line. preview-close.yml and wall-test.yml stay as approved, byte for byte.
     - (b) Keep the approved text: the job `tests` stays red, and the tests never run in CI.
     - Not offered: a field in hosting/package.json that makes the folder argument work. It was measured to work, but
       it hides the fix in a file his word gave "for the upload tool only".
   - Recommendation: (a). Nothing goes into `.github/workflows/` before his answer.
2. **MISSING-BUILD: how a missing build is reported.**
   - What the plan said (section 2.7): once main's toolchain workflow hands the build over, the pull request's comment
     says when a build is missing.
   - What the approved texts allow: when the job `check` fails, the job that writes the comment does not run. So a
     missing build shows as a red run under main in the Actions tab, with an error line, and GitHub's own
     failed-run notice. The comment says nothing that time.
   - Options:
     - (a) Keep the texts. The red run is the signal; the runbook says where to look.
     - (b) Change preview.yml so that the comment can say it: a larger text change, read again by the security-auditor,
       then back to him with its sha256.
   - Recommendation: (a). A missing build should be rare once the hand-over step is in.

## Known, not a decision
- **R-COMMENT-PERM.** The job that writes the comment holds `pull-requests: write`. GitHub's docs, as the session's
  tools can read them, do not settle whether that alone allows writing a comment on a pull request: the permission page
  lists the comment calls under both "Issues" and "Pull requests". The first real run shows it. If GitHub refuses, the
  fix is a small pull request of risk tier 2.

## The full new text (texts/preview-proposed.yml)
```yaml
# Step 2.6 (lane 2): a preview link for every pull request from this repository.
# The plan: step 2.6's plan, revision 2 (in its report); what to do when something fails: docs/runbooks/previews.md.
# The token lives only in the environment `preview`, whose one deployment rule is the branch `main`; no reviewer.
# Its workflow_run and schedule events run main's copy of this file; on a pull request only the job `tests` runs,
# with no secret. Values from the event or from another job reach a shell only through `env:`, and are checked there.
# The build is data: it is checked and uploaded, never run. No cache in any job.
name: preview

on:
  workflow_run:
    workflows: [toolchain, preview-close]
    types: [completed]
  schedule:
    - cron: "41 4 * * *"
  pull_request:
    paths:
      - "hosting/**"
      - ".github/workflows/preview.yml"
      - ".github/workflows/preview-close.yml"

permissions: {}

defaults:
  run:
    shell: bash

env:
  PROJECT: shamble-previews
  WRANGLER_SEND_METRICS: "false"

jobs:
  tests:
    # No secret, no environment: the scripts' tests, this file's own lint and the header file's test, each with a
    # planted defect that must turn it red.
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: The scripts' tests, the workflow lint and the header file test
        run: node --test hosting/preview.test.mjs

  check:
    # No secret. Only a successful pull-request run of main's toolchain workflow, from this repository.
    if: >-
      github.event_name == 'workflow_run' &&
      github.event.workflow_run.name == 'toolchain' &&
      github.event.workflow_run.path == '.github/workflows/toolchain.yml' &&
      github.event.workflow_run.event == 'pull_request' &&
      github.event.workflow_run.conclusion == 'success' &&
      github.event.workflow_run.head_repository.full_name == github.repository
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
      actions: read
      pull-requests: read
    outputs:
      pr: ${{ steps.pr.outputs.pr }}
      sha: ${{ steps.pr.outputs.sha }}
      go: ${{ steps.site.outputs.go }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Find the open pull request whose newest commit is this run's commit
        id: pr
        env:
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
          HEAD_SHA: ${{ github.event.workflow_run.head_sha }}
        run: node hosting/preview.mjs find-pr
      - name: Fetch this run's build into a fresh folder outside the workspace
        id: dl
        if: steps.pr.outputs.pr != ''
        continue-on-error: true
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site
          run-id: ${{ github.event.workflow_run.id }}
          github-token: ${{ github.token }}
          path: ${{ runner.temp }}/site-raw
      - name: Check the build (the refuse list), then add main's header file
        id: site
        if: steps.pr.outputs.pr != ''
        env:
          DOWNLOAD: ${{ steps.dl.outcome }}
        run: node hosting/preview.mjs check-build
      - name: Hand the checked build to the deploy job (this run only, kept one day)
        if: steps.site.outputs.go == 'true'
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site-checked
          if-no-files-found: error
          retention-days: 1

  deploy:
    # The only job with the deploy token, on two steps only. It runs nothing from the build.
    needs: check
    if: needs.check.outputs.go == 'true'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    environment:
      name: preview
      url: ${{ steps.url.outputs.url }}
    concurrency:
      group: preview-pr-${{ needs.check.outputs.pr }}
      cancel-in-progress: true
    permissions:
      contents: read
    outputs:
      url: ${{ steps.url.outputs.url }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Check the pull request's number and commit again
        env:
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
        run: |
          [[ "$PR" =~ ^[0-9]{1,6}$ ]] || { echo "not a pull request number"; exit 1; }
          [[ "$SHA" =~ ^[0-9a-f]{40}$ ]] || { echo "not a commit id"; exit 1; }
      - name: Fetch the checked build from this run only
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site
      - name: Check it again, and compare its header file with main's byte for byte
        run: node hosting/preview.mjs recheck
      - name: Install Wrangler from main's lockfile into a fresh folder, install scripts off
        run: |
          mkdir -p "$RUNNER_TEMP/wrangler" "$RUNNER_TEMP/empty"
          cp hosting/package.json hosting/package-lock.json "$RUNNER_TEMP/wrangler/"
          cd "$RUNNER_TEMP/wrangler"
          npm ci --ignore-scripts --no-audit --no-fund
      - name: Ask Cloudflare whether the token is valid (prints its status and dates, never the token)
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
        run: node hosting/preview.mjs verify-token
      - name: Upload the build as pr-<number>, run from an empty folder
        working-directory: ${{ runner.temp }}/empty
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
          WRANGLER_OUTPUT_FILE_PATH: ${{ runner.temp }}/wrangler-output.ndjson
        run: |
          [[ "$PR" =~ ^[0-9]{1,6}$ ]] && [[ "$SHA" =~ ^[0-9a-f]{40}$ ]] || exit 1
          "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy "$RUNNER_TEMP/site" \
            --project-name "$PROJECT" --branch "pr-$PR" --commit-hash "$SHA" \
            --commit-message "pull request $PR" --commit-dirty=true
      - name: Read the link from Wrangler's record (checked against the link's shape)
        id: url
        run: node hosting/preview.mjs read-url

  after:
    # No secret. The served headers on the link (with the check's bite), then one comment on the pull request,
    # written from checked values only.
    needs: [check, deploy]
    if: always() && needs.check.outputs.go == 'true'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
      pull-requests: write
      deployments: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Fetch the checked build's file list from this run
        if: needs.deploy.result == 'success'
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site
      - name: Check the served headers, then the bite (the same check on responses with the cache rule removed must fail)
        id: headers
        if: needs.deploy.result == 'success'
        env:
          URL: ${{ needs.deploy.outputs.url }}
        run: node hosting/preview.mjs check-headers
      - name: Write or update the link comment
        if: always()
        env:
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
          URL: ${{ needs.deploy.outputs.url }}
          DEPLOY: ${{ needs.deploy.result }}
          HEADERS: ${{ steps.headers.outcome }}
        run: node hosting/preview.mjs comment

  reconcile:
    # Deletes the previews of closed pull requests, and the older uploads of open ones. It reads nothing from the run
    # that started it: it looks every pull request up itself, deletes at most 20 per run, and logs each before it.
    needs: deploy
    if: >-
      always() && (
        github.event_name == 'schedule' ||
        needs.deploy.result == 'success' ||
        (github.event_name == 'workflow_run' &&
         github.event.workflow_run.name == 'preview-close' &&
         github.event.workflow_run.path == '.github/workflows/preview-close.yml'))
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    environment:
      name: preview
      deployment: false
    concurrency:
      group: preview-reconcile
      cancel-in-progress: false
    permissions:
      contents: read
      pull-requests: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Reconcile the project's previews with the open pull requests
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
        run: node hosting/preview.mjs reconcile
```
