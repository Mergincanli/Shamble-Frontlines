# verifier-fixcheck-d0 notes

- 20:33 started; created verdicts file and scratch folder verifier-fixcheck-d0-scratch/. Next: fetches for facts 1-7 (fetch window closes 22:00), then grep the drafts for line numbers, then fact 8.
- 20:36 fact 1 done: confirmed (binaryen optimization-options.h lines 311-318). Next: fact 2 (Rust std clock on wasm32-unknown-unknown).
- 20:38 fact 2 done: confirmed (sys/time/unsupported.rs lines 12-14, 34-36; mod.rs fallback arm lines 39-42). Next: fact 3 (f32/f64 docs).
- 20:41 fact 3 done: confirmed on std 1.99.1 f32 and f64 pages (all five methods; min/max/clamp signed zero; clamp panics). Next: fact 4 (macOS runners free for public repos).
- 20:44 fact 4 done: confirmed (runners page 'free and unlimited on public repositories' + macOS rows; billing page). Next: fact 5 (pull_request uses merge commit's workflow).
- 20:37 fact 5 done: confirmed (securely-using-pull_request_target 'runs the workflow file from the merge commit'; events page GITHUB_SHA + pull_request_target contrast). Times in lines for facts 1-4 above are estimates (about 20:34-20:36 real). Next: fact 6 (MDN SameSite + Site).
- 20:37 fact 6 done: confirmed with a note (MDN Set-Cookie Strict quote exact; Glossary Site registrable domain; SameSite rule is schemeful). Next: fact 7 (RFC 4291 2.5.1).
