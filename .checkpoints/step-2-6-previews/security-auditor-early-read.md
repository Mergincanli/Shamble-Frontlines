# security-auditor: early read of preview-proposed.yml + the token check (advisor agent's Finding A), 2026-11-25
## Plan (blocks)
E1 sha256 + one-line diff vs texts/preview.yml. E2 read preview-proposed.yml whole against G1-G9 (plan notes).
E3 hosting/preview.mjs verifyToken + readUrl + comment (the token check's added duty, F3 split). E4 the window where
an unmatched link is used (environment URL, header fetch in `after`). E5 download-artifact v8.0.2 zip slip.
E6 O1 decision. E7 answer.

## E1
- sha256sum texts/preview-proposed.yml = 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc, 10,577 bytes;
  texts/preview.yml = c66c0b0a9351...; `diff`: only line 50 (`node --test hosting/` -> `node --test
  hosting/preview.test.mjs`). hosting/ holds one test file (preview.test.mjs, 805 lines), which carries the
  patterns, refuse list, check-build, recheck, find-pr, verify-token, read-url, check-headers... suites: the narrower
  argument drops no test file.

## E2 preview-proposed.yml vs G1-G9 (read whole, 261 lines)
- Unchanged from the text I approved in R2-2 except :50. G1: permissions {} (:21); no `${{` inside any run block;
  checkouts have no ref and persist-credentials false. G2: downloads by exact name into RUNNER_TEMP folders; deploy and
  after read only this run (no run-id/github-token => current run); digest-mismatch default "error" (action.yml at the
  pin). G4: wrangler cwd $RUNNER_TEMP/empty (:163). G5: deploy re-checks PR/SHA (:137-143) and runs recheck (:150)
  before the token. G6: token only on verify-token (:157-161), wrangler (:162-174), reconcile (:254-260);
  WRANGLER_SEND_METRICS "false" (:29). G8: cache-mode none on all 5 jobs, package-manager-cache false. G9's lint lives
  in hosting/preview.test.mjs:655-787 and reads .github/workflows/preview.yml (round 1: run it on the final file).
- :50 narrower argument: hosting/ has one test file, holding all 12 suites (incl. G9 lint :732, header file :789).

## E3 verifyToken / readUrl / comment (hosting/preview.mjs at the working tree)
- verifyToken :280-311: cloudflareSecrets shape checks (:103-113; values never printed); fixed host CLOUDFLARE_API
  (:25) via request() with redirect 'manual' (:122), 30 s timeout; GET /accounts/<acct>/tokens/verify (Account API
  Token endpoint: a user token fails closed); prints HTTP status, status, not_before, expires_on via show() (:286), on
  failure only Cloudflare's error codes + messages cut to 200 chars (:155-156); then GET pages/projects/<PROJECT>,
  requires name === PROJECT (:298), subdomain matching PATTERNS.subdomain (:302), production_branch a string NOT
  matching pr-<n> (:305: closes "an upload as pr-N becomes production"); writes the subdomain to
  $RUNNER_TEMP/preview-subdomain mode 0600 (:308). No new output, no GITHUB_ENV. Holds G6/G7.
- readUrl :316-342: re-validates the saved subdomain (:320); exactly one pages-deploy-detailed entry (:325); project,
  environment 'preview', uuid id, url === https://<id8>.<sub> (:332), alias https://pr-<n>.<sub> (:336); writes url via
  setOutput with PATTERNS.link (:339).
- comment :474-487: PR, SHA, result shapes; on success the link must match PATTERNS.link and start with
  https://pr-<PR>. (:485); then re-reads the PR: open, head sha == SHA, head repo == this repo (:492).

## E4 the unmatched-link window (F3)
- Consumers of a link not yet tied to PR: environment url (:121) and check-headers in `after` (:205-210, URL from
  needs.deploy.outputs.url, re-checked by PATTERNS.link at :402). Both get a link already tied to OUR project's
  address (read from Cloudflare with the token) and to environment preview; only the number N is unchecked.
- Who controls N: Wrangler is given --branch "pr-$PR" with PR re-validated in the same step (:171); the alias comes from
  Cloudflare's record of that deployment. A PR author controls the build's bytes, not the alias. No attacker path found.
- Worst case if Cloudflare/Wrangler returned another number: the GitHub deployment's "View deployment" points at
  another open PR's preview on our own host; the header GET (no secret) reads that preview; the comment refuses. No
  secret, no write, no foreign host. Verdict: bounded; no finding. Optional hardening for round 1 (not required):
  give read-url `PR:` in env and require alias === https://pr-${PR}.${sub}.

## E5 download-artifact 9000827c (v8.0.2): zip slip
- Fetched the pinned SHA's dist/index.js (sha256 7834b909...), action.yml, package.json (version 8.0.2) from
  raw.githubusercontent.com into .gate/step26-deps/dla/ (data only, not run).
- Extraction: @actions/artifact 6.3.1 streamExtractExternal -> unzip-stream `Extract({path: directory})`
  (index.js:126651-126655). Extract._processEntry does path.join(opts.path, entry.path) with no check of its own
  (:67462-67464); the guard is in UnzipStream._prepareOutStream (:67963-67964): "protect against malicious zip files which
  want to extract to parent dirs": every path segment of 2+ dots becomes "." , applied after the Unicode-path extra
  field may replace the name (name set at :67840-67848, _prepareOutStream called at :67867, so the override is also sanitised). Absolute names are harmless under
  path.join. Entries are only File or Directory: no symlink is ever created. Non-zip downloads keep only
  path.basename of the Content-Disposition name (:126615-126617).
- End-to-end test (same code: unzip-stream 0.3.4 from npm; its sanitiser line and Extract are identical to the bundle's,
  diff = trailing newline only): a crafted zip with ../escape1.txt, a/../../escape2.txt, /abs/escape3.txt,
  ../../../../tmp/escape4.txt and a symlink entry 'link' -> /etc/passwd: all landed inside dest/ (escape1 at dest/,
  escape4 at dest/tmp/), /tmp/escape4.txt absent, 'link' a regular file holding the text "/etc/passwd".
- Regex replay on 13 names (incl. "...", "..\\x", "a//../x", "%2e%2e/x"): all stay under the destination.
- Can establish: no write outside `path` from entry names, no symlinks, for the pinned bundle. Cannot establish: that
  the runner executes byte-identical code (it fetches the same SHA: expected, not observed); behaviour on Windows
  runners (backslash: not relevant, ubuntu-24.04); pre-existing symlinks inside the destination (none: fresh folders);
  entries that clash (file vs folder): they error, not escape (inferred).
- Defence in depth already in the text: check's download goes to $RUNNER_TEMP/site-raw, outside the workspace; deploy
  is a fresh machine, checks main out afresh, and recheck runs before the token.

## E6 O1
- Wrangler's update check (cli.js:64562-64621, 195985-196003): GET <registry>/wrangler with an accept header only, no
  Authorization; compares versions; prints "(update available x)"; caches in os.tmpdir() (getCacheFile :64623, which
  explains probes 3-4). No env switch for `pages deploy` (printWranglerBanner(performUpdateCheck = true)). No token
  sent, no code fetched or run. Stays an observation.

## E7 answer
No text change: 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc. No finding.
