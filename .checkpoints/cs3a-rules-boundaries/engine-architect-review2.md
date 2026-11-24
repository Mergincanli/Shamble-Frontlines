# engine-architect, review round 2 of cs3a (fix pass 79666ad9 over round-1 head 5c92311; main 1ec6f6c)

Plan: only `git diff 5c92311 79666ad9 -- . ':!.checkpoints'`. Order: (1) the diff stat; (2) F1: build-checks.mjs
simFeatureList(treeText, closureTree) + build.mjs's three lists + the doc's ADR 0002 part 3 cite; re-run my round-1
scratch ws-devfeat against the NEW function (copy, .gate/engine-architect-cs3a/); weigh it against PM-0004-74 and
ADR 0002 part 5 :353-357 (say what the planned dev-only ts-rs feature does to the compare); (3) F2 panic.rs
`start, private` + sim-wasm.test.mjs export list (read the built glue, no build in the shared tree); F3
panic-message.ts trapMessage/PANIC_TEXT_ROOM + host.ts/worker.ts callers; F4 backticks; F5/F6 (deferred: check the
pointers only); (4) MUL-ADD: f32_ops.rs check/check_as + cfg!(target_arch), vectors pins, rules_scan.rs HELD and the
widened scan vs ADR 0004 part 7 and step 2.3's lint; (5) security point: vite.config.mjs allow-list, the workspace link,
pkg-rules beside pkg; (A) vs (B) against ADR 0002 part 8; (6) new edges/coupling the fix introduced; verdict.
Rules: shared tree read-only (tests only), writes under .gate/engine-architect-cs3a/ only, no git state changes.

## Findings (as found)
- 16:05Z diff stat: 23 files (+667 -204); tree = 79666ad9 for crates/ and packages/ (git diff --quiet). Scratch copy of build-checks.mjs @79666ad9: .gate/engine-architect-cs3a/r2/ (sha256 2612f0d5... both).
- 16:15Z F1 FIXED ANOTHER WAY (modelled, not scoped). build-checks.mjs:283-302 simFeatureList(treeText, closureTree): closure = sim's packages from the normal-edge tree; found = every `sim feature` line + every line of a closure package, whole tree. build.mjs:100-106 three lists, tests = (normal,dev, --workspace) over closure (normal, --workspace). Cites ADR 0002 part 3 (:274); part 4 stays on the overflow doc (:263): right (Cargo.toml:25 "ADR 0002 part 4"). CI's release line now named "not modelled" (:280-281): the scope sentence I asked for.
- 16:20Z measured: round-1 ws-devfeat now ["sim default","sim probe"] (caught; sim's own dev-dep helper stays out). Real repo today, three lists = ["sim feature \"default\""], 0 differences (cargo tree --locked --offline, read-only; tree unchanged).
- 16:25Z PART 5 MEASURED (ws-part5: content = sim's dependency with `ts = []`; protocol's [dev-dependencies] content features ["ts"]; tools/extra turned on by room's dev-dep = PM-0004-74's control): cargo test --workspace passes, "content::TS in protocol's test build: true"; the compare (copy @79666ad9, lists as build.mjs asks): players/tools [content default, content v0.0.0, sim default], tests + `content feature "ts"` -> 1 difference -> checkCrates throws, every Wasm build (dev and CI) stops. Control passes (tools' extra unseen). r2/part5-check.out.
- 16:30Z every build stops, not only dev: build.mjs:225 buildWasm() first in main() for dev, --production and --wasm-only; :141 checkCrates() before any compile. Today protocol's derives are cfg_attr(test) (messages.rs:12...), no feature: nothing trips it yet; the nested case (protocol nesting content's or sim's types) does.
- 16:35Z NEW-A LOW measured (ws-version: sim -> x 1.0.0; tools -> x 2.0.0 features ["f"]): the whole-tree-by-name list (build-checks.mjs:297-300) counts x v2.0.0 and its "f" as sim's -> 2 false differences, every build stops (r2/version-check.out). Loud, forward (sim has no deps today). Fix when it matters: key closure and lines by name+version (a feature line's package node sits below it in the tree).
- 16:45Z F2 FIXED: panic.rs:75 `#[wasm_bindgen(start, private)]`; built pkg (sha256 383f5303...3963 = checkpoint) glue exports Sim, panic_buffer, tick_hz (sim_wasm.js:6,100,109) + initSync/default (:287): 5, no start; sim-wasm.test.mjs:100-104 asserts exactly that; still run at init (sim_wasm.js:190 wasm.__wbindgen_start()). Raw `start` stays in the module and in InitOutput (sim_wasm.d.ts:70) as round 1's probe showed: the Worker reads only `memory` from it; not a finding.
- 16:47Z F3 FIXED: panic-message.ts:29-35 trapMessage, the one sentence; host.ts:140 and worker.ts:57-59 call it (worker: an empty memory before load). Tests panic-message.test.ts:52-64.
- 16:48Z note (no finding): PANIC_TEXT_ROOM = 1016 (panic-message.ts:9) is a hand copy of panic.rs's 1024 - 8, now load-bearing (reader refuses any other room, :22); guarded by the boundary test reading the real Wasm (sim-wasm.test.mjs:94-96). Fold into the generated layout table when 3b touches it.
