// U8-F1 scratch (not committed to the spike; the builder's notes): ds_selftest's ws_split.txt when room-05's log is
// missing, run on r1.sh's own functions (cut as r1sh.test.mjs's fn() cuts them) with the VM calls stubbed, in a fresh
// temporary folder. Prints the file and bash's exit status.
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const R1 = readFileSync(new URL("../../../spikes/netcode-proof/ops/runs/r1.sh", import.meta.url), "utf8");
const fn = (name) => R1.match(new RegExp(`^${name}\\(\\) \\{(?:\\n[\\s\\S]*?\\n\\}| .*; \\})\\n`, "m"))[0];
const dir = mkdtempSync(join(tmpdir(), "u8f1-missing-"));
try {
  const script = [
    "set -euo pipefail",
    `R="${dir}/results" ROOMS="${dir}/no-such-rooms" ATT=''`,
    "timing_invalid() { :; }; stat_cpu() { :; }; lg_block() { :; }; lg_keep() { :; }; keep() { :; }",
    fn("out"), fn("logq"), fn("ds_selftest"),
    "ds_selftest",
    "echo exit=$?",
  ].join("\n");
  console.log(execFileSync("bash", ["-c", script], { encoding: "utf8" }).trim());
  console.log(`ws_split.txt: ${JSON.stringify(readFileSync(join(dir, "results", "ds-selftest", "ws_split.txt"), "utf8"))}`);
} finally {
  rmSync(dir, { recursive: true, force: true });
}
