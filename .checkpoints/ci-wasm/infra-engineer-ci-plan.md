Task: CI-WASM, the facts and a draft of the exact CI text (infra-engineer, part "ci-plan"), started 2026-11-19.

## Plan (blocks, in order)
1. Today's CI and the old draft (toolchain.yml, gate.yml, #60 report, cs2-plan s18 + s7 5a, tool-route.sh,
   security-auditor-tool.md, build.mjs + build-checks.mjs + wasm-bindgen-tool.mjs, playwright config + 5 specs,
   cs2 report Evidence + Not verified). What the old draft gets wrong.
2. `gh release verify-asset` from official sources (command, proves / does not, gh version vs image, token,
   permissions, hosts, output and exit codes, `gh release verify`, immutability of 0.2.130, first-run list, lines to quote).
3. Browser on the runner: (a) Playwright 1.65.0 download, (b) preinstalled Chrome via SHAMBLE_CHROMIUM, (c) other;
   spec needs; recommendation with exact download list; FIREFOX-TESTS options.
4. The shape: inside the cut / own job / both; Chromium under unshare --net + no-new-privs; order; time; least privilege.
5. The exact text: proposed workflow files under .gate/ci-wasm/proposed/.github/workflows/, diff at
   .checkpoints/ci-wasm/ci-edit.diff, table (line -> why -> tried here).
6. Dry run here (build, boundary tests, type checks, browser tests once, member list), outputs and times.
7. First runner run: stop rule, planted changes.
8. CI side of follow-ups: RR-F5, SA-DIST, E2E-TYPES, TE-NT6 CI settings.
End: planned size, questions for Mustafa, Not verified, DONE.

## Notes (one line per finding)
- Memory: no `.claude/agent-memory-local/infra-engineer/` exists; nothing to load.
- Context read: .checkpoints/ci-wasm/brief-context.md (whole).

