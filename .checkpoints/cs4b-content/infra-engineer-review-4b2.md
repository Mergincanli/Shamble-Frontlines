Task: infra-engineer review of change-set 4b-2 "The world", round 1, code head c442a80 (brief review-brief-4b2.md).

Plan (blocks, in order):
B1 read the CI workflow(s) in the tree: the toolchain job's steps, timeouts, network cut, browser-test limit.
B2 read CI's own logs (gh api, GET) for run 46033756121 (fbbd84d) and 46035714024 (885d382): phase times, the dev server's cargo run under the network cut.
B3 read the diff's build/deploy side: build.mjs content.json placement, vite.config.mjs /content.json route, content.ts fetch path.
B4 preview/hosted-build questions: content.json beside index.html (relative path, MIME, caching), 25 MiB per-file limit, players' file size.
B5 findings and answer.

Notes:
