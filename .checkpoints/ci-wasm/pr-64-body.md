**Draft. It replaces #63 and #62** (both stay open and untouched until Mustafa says to close them). Mustafa's words of 2026-11-23, 16:30:46 UTC, pasted into a new Claude Code chat he calls Builder V3 and recorded in `docs/DECISIONS.md` on this branch: that chat takes the CI-WASM pull request over, because the CI file is written where the repository's guard and the Accept edits mode work (his tap WRITE-WHERE, 14:54:47 UTC).

The branch `claude/loving-hopper-6p0mg2` stands in for `cs/ci-wasm`. It starts at the head of `claude/project-thread-qx9woh` (21b6ea0, #63's head; it contains the handover commit 2441b3e) and never pushes to that branch or to `claude/wonderful-hamilton-jdmlnf` (#62's). Risk tier 2 once the workflow file is in (a CI file): the advisor agent and security-auditor review it; it merges only on Mustafa's go in that chat, for the head the advisor agent approved.

## What it is
The small pull request his answer CI-WASM (a) of 2026-11-18 asks for, "opened right after change-set 2 is merged and merged before change-set 3's build starts". After it, the `toolchain` check cannot go green unless the Wasm builds with the checked wasm-bindgen tool, the boundary tests pass on that Wasm, the Worker, the page and the browser specs type-check, and the 12 browser tests pass, all inside the network cut. Before the tool's first run on a runner, GitHub's own check of the tool's file runs (`gh release verify-asset`); the report will quote its output.

## In it so far
- **Code** (3cddb55): a unit test of the build's refusal of cargo's settings from the environment; five browser checks made robust for a slower machine without lowering any bound, with readings printed before they are judged; a 16-line declaration file so that the browser specs can be type-checked with no new dependency; one `.gitignore` line (`dist-*/`). No manifest, no lockfile, no dependency changes.
- **The docs pass that records #61's merge** (c934fc0; his words of 2026-11-19, 16:08:04 UTC: it "stays with the CI pull request"): his go, his answer VISION-NOTE, his two notes and a pointer to his browser check in `docs/DECISIONS.md`; his four browser-report messages whole, with their reading, in `docs/reports/2026-11-19-cs2-browser-check.md`; the dated note he approved in `docs/VISION_V2.md`.
- **His words of 2026-11-23**, by script, in `docs/DECISIONS.md`: ANSWERS-7 (CHROME-SOURCE (a), CI-TEXT (a), FIREFOX-TESTS (c); for change-set 3: RNG-SOURCE (c), SIZE-3 (b), PRESS-PACE (a), DEPART-3 (a)); LONG-RUNS; WRITE-THREAD, replaced by WRITE-WHERE; BUILD-SEAT; ACCT-3; and his block for Builder V3 (8e7f503), which answers CI-MODEL with (a).
- **The workflow text, first write** (cb476b1; his yes CI-WRITE-1 of 2026-11-23, written with the chat's mode reading `default`, the app's Accept edits): the 93 lines he said yes to (sha256 of the diff 240c7db726dee600b54d10b0252b33e527482ecf88ba41af849660d55e8d2673; the file after the write: sha256 d1dbcb533460936e9f4f3ca2e97ed28c6d586638e52c00fa034e875f7a4e975c). **Its first runs stopped under the stop rule:** the download, its size and sha256, and GitHub's own check passed ("Verification succeeded! ... is present in release 0.2.130", with the token at contents: read); the step that installs the tool refused at its last check, because `/opt` on GitHub's runner is writable by anyone (mode 777). The check did what it is for. **`toolchain` is red on cb476b1** (the later commits are notes and start no run) until the corrected text is in.

## Still to come
0. The corrected install place (his OPT-PATH (a) of 2026-11-23, 18:40:57 UTC): the tool goes where every parent folder is root-owned and closed to others, and the check is widened to every parent up to `/`. Its exact text goes to him first; nothing under `.github/` is written before his yes.
1. Then the runs on a runner under the stop rule, and two planted changes, each taken back in the next commit (the planted CI lines and their take-back are each asked of him first; the planted render count is covered by his CI-TEXT (a)).
2. Reviews on the final diff (security-auditor, infra-engineer, test-engineer), the verifier, the advisor agent on every pushed head, STATUS, ROADMAP, the map and the archive, the report (`docs/reports/2026-11-19-ci-wasm.md`, with the workflow text kept apart from the docs, as he asked), and the checkpoint folded and deleted before this is marked ready.

## Where the detail is, until the report
The resume card: `.checkpoints/ci-wasm.md`. The plan (revision 4; the advisor agent's verdicts: APPROVE with conditions, all worked in): `.checkpoints/ci-wasm/plan.md`. The prepared workflow text: `.checkpoints/ci-wasm/ci-edit.diff`. security-auditor's reading of it: `.checkpoints/ci-wasm/security-auditor-ci-text.md`. The advisor agent's read of the handover: `.checkpoints/ci-wasm/advisor-takeover.md`.

## Not in it
Firefox in CI (his FIREFOX-TESTS (c): a small pull request of its own right after this one), change-set 3, any new dependency, any edit to `gate.yml`, the kit or the gate, anything a player sees.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01WLuqnvpAh9XTitx8ehKJcQ
