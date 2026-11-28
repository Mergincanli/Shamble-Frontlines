# UM-4: drafts for steps 1-2 (prep, 2026-11-27 14:5x UTC, on his 14:52 message; NOT applied, NOT tested)
Scratch only: nothing here is in the source. At the build, each piece is checked against the code first (failedRun's
signature, nodeCounts at check.mjs:1984, check.mjs's node:fs import for existsSync, check.test.mjs's runner helpers at
:1980-1981 and the STEPS list at :2847), then applied by Edit in um4-plan.md rev 0.3's order: the test first, red, then
the step, green (UM4-1, UM4-2).

## check.test.mjs:2847: the STEPS list gains "ops" after "web" (red first)

## check.mjs: the step, beside webStep (check.mjs:2663-2680), and its STEPS line after `web` (check.mjs:2713)
```js
/**
 * The ops step (UM-4): `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, both in HERE. It
 * fails, never skips, when either fails, when node's `# tests` count is 0, or when the wrappers' last line is not
 * `wrappers.test.sh: N ok, 0 failed` with N above 0 (UM4-1; the web step's U7R-TE-4 rule, check.mjs:2677-2678). A
 * missing ops/browser/node_modules fails with one line (UM-4 (d)). Through `runner`; `present` checks the folder.
 */
export function opsStep(runner = run, present = () => existsSync(join(HERE, "ops", "browser", "node_modules"))) {
  if (!present()) return { ok: false, out: "ops: ops/browser/node_modules is missing (UM-4 (d)): the ops tests need it" };
  const n = runner(process.execPath, ["--test", "ops/**/*.test.mjs"], HERE);
  if (!n.ok) return failedRun(n, `the ops step's node tests (${nodeCounts(n.out)})`);
  if (!(Number(n.out.match(/^# tests (\d+)/m)?.[1] ?? 0) > 0)) return failedRun(n, "the ops step's node tests (none ran)");
  const w = runner("bash", ["ops/tests/wrappers.test.sh"], HERE);
  if (!w.ok) return failedRun(w, "the ops step's wrappers test", n.out);
  const last = w.out.trim().split("\n").at(-1) ?? "";
  const m = last.match(/^wrappers\.test\.sh: (\d+) ok, 0 failed$/);
  if (!m || !(Number(m[1]) > 0)) return failedRun(w, `the ops step's wrappers test (its last line: ${last})`, n.out);
  return { ok: true, out: `${n.out}${w.out}\nops: ${nodeCounts(n.out)}; ${last}` };
}
```
STEPS, after `web: (runner = run) => webStep(runner),`:
```js
  ops: (runner = run) => opsStep(runner),
```
check.mjs's header step list gains `ops` after `web` (UM4-2).

## check.test.mjs: the step's own tests (each red first, then green; a recording runner and a stand-in `present`)
1. Both commands pass: ok; the runner saw exactly two calls, in order: [process.execPath, ["--test",
   "ops/**/*.test.mjs"], HERE] and ["bash", ["ops/tests/wrappers.test.sh"], HERE] (program, arguments and folder pinned,
   as :1980-1981 pin npm's); the summary line names the node counts and the wrappers' last line.
2. The node tests fail: not ok, the wrappers never run.
3. The node tests print "# tests 0": not ok ("none ran").
4. The wrappers fail: not ok.
5. The wrappers' last line is "wrappers.test.sh: 0 ok, 0 failed", or anything but the form: not ok.
6. `present` returns false: not ok with the one line, and the runner is never called.
Counts at the build: the self step's count rises by the new tests (recorded); step 5's bars stay as um4-plan.md gives them.
