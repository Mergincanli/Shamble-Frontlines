// U8's merge, step C1 (advisor-u7-brief.md:118-119): one T5 (ii) leg's page records written through U8's save(), so
// npt-judge can read them with PM-0001-36's mark. A form test only: a container loopback leg (U7's T-builder tree,
// 7e2d6f3 plus its unsaved R-half; run 3 of 19:53 UTC, leg 6-mds) stamped R1, eu-path and ML*, kept under .gate/.
// Run from the main folder after u8-merge-plan.md's M4 and M4c: node .checkpoints/netcode-proof-test/u8-c1.mjs <H0>
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";

const M = "/home/user/Shamble-Frontlines";
const BROWSER = join(M, "spikes", "netcode-proof", "ops", "browser");
const T = join(M, ".claude", "worktrees", "agent-a9873cdba07a3219e");
const LEG = join(T, ".gate", "u7", "t5ii", "2026-11-25T19-53-08-358Z", "6-mds", "result.json");
const WASM = join(T, ".gate", "netcode-proof-test", "t5-dist", "npt_client_wasm_bg.wasm");
const OUT = join(M, ".gate", "u8-merge", "c1-logs");

const commit = process.argv[2] ?? "";
if (!/^[0-9a-f]{40}$/.test(commit)) throw new Error("usage: node u8-c1.mjs <H0, the full 40-hex commit>");
if (existsSync(OUT) && readdirSync(OUT).length > 0) throw new Error(`${OUT} is not empty: C1 judges one log only`);

const { save } = await import(join(BROWSER, "lib.mjs"));
const { RECORDS, ROOMS, logDate } = await import(join(BROWSER, "config.mjs"));

const page = JSON.parse(readFileSync(LEG, "utf8")).pages[0];
if (!Array.isArray(page.records) || page.records.length === 0) throw new Error("pages[0] holds no records");
// Only the fields legHeader and legLog read (config.mjs:133-171); nothing else of the leg's result is passed on.
const state = {
  records: page.records,
  nptClockResNs: page.nptClockResNs,
  nptDropped: page.nptDropped,
  version: page.version,
  status: page.status,
  timed_out: false,
};
const run = {
  run: "R1",
  date: logDate(new Date()),
  commit,
  wasm_sha256: createHash("sha256").update(readFileSync(WASM)).digest("hex"),
};
const leg = { suite: "mds", browser: "chrome", transport: "wt", nn: ROOMS.mds.nn, attempt: 0, slot: 0 };

mkdirSync(RECORDS, { recursive: true });
const base = save(run, leg, state, {
  browser: "chrome",
  slot: 0,
  path: "loopback",
  check: "C1 form test: U7's T5 (ii) run 3, leg 6-mds, through U8's save()",
  page_lines: page.records.length,
});
const log = `${base}.jsonl`;
mkdirSync(OUT, { recursive: true });
const copy = join(OUT, basename(log));
copyFileSync(log, copy);
const sha = createHash("sha256").update(readFileSync(log)).digest("hex");
console.log(`c1: saved ${log.slice(M.length + 1)}`);
console.log(`c1: copied to ${copy.slice(M.length + 1)}`);
console.log(`c1: log sha256 ${sha}; ${page.records.length} page lines; wasm sha256 ${run.wasm_sha256}`);
