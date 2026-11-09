# U4a build: the wire (npt-proto), notes

Builder: general-purpose agent, opus, worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9297644e7c0354f3,
base dffef0e (branch claude/netcode-proof-test-plan-hcky5g). Plan: u4-build-plan.md rev 2 (sha256 0d69332c…) §5 U4a,
§4.1, §4.4-§4.6, §4.11, §4.14; advisor-u4.md (U4 verdict, U4s ruling, F1/F2 ruling); the catalogue; netcode.md;
u4s-carries.md entries 1-5 and 34. Files: crates/proto, check.mjs, check.test.mjs only. Commit nothing.

## Plan
1. npt-proto modules: bits (pub(crate) bit and byte readers and writers), quant (`Quant`: per-field codes from each
   `Field`'s min, max and bits, signed two's complement / unsigned / offset; -57's integer coarsening, -17/-58 tick16,
   -21 no-trailing and no-channel as its bites; `BITES`), wrap (`Wrap`: 8, 16, 32-bit compares by counter name,
   16-bit rebuild; bites-only plain-< defect for one named counter, no ids: their sites are U4e's), limits (`Limits`:
   every catalogue size, the redundancy cap, group cap, the per-type table), msg (the 15 types, `Message`,
   `Direction`, framing per channel, `encode`, `decode`), input (whole + delta repeats), snapshot (header with group
   index and count, `EntityDelta`, the layout tables from `EntityKind::layout()`, the record mapping), control
   (`SessionControl`, `RoomControl`), hash (`client_hash`, FNV-1a-64, never 0), error (`DecodeError`, `EncodeError`).
2. Tests a1-a9 + carry 4 (pitch -512) + catalogue items 1, 2, 3 (size half), 6 in tests/catalogue.rs with broken-copy
   bites.
3. check.mjs: sim/proto clippy.toml comparison (a9); npt_link-only-Channel grep for proto/src and client/src (carry 3);
   the four nits (carry 34): pin STEPS.doc and -D warnings (and fmt, clippy, test), doc the wasm32 build (tcpinfo's
   stub), lintBothBuilds summary on a bites-only failure, a measured-build failure's summary line names the first error.
   Self-tests with plants in check.test.mjs; the deps fixture gains a plant per new check.
4. Bites: mark bites in the bites build (unit-shown); scratch plants per test, in place, one at a time, sha256-restored.
5. Final: check.mjs twice, clippy host and wasm32 both builds, fmt.

## Progress
