// reap.test.mjs: the e2e drivers' failure path (U7R-NC-4): every child they started is SIGKILLed and the current load's
// tokens file removed, so an unhandled rejection leaves no npt-host on 127.0.0.1:31000 and no token under .gate/.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { CHILDREN, reap, track } from "../e2e/room.mjs";
import { scratch } from "./helpers.mjs";

test("reap: each tracked child SIGKILLed, the set emptied, the load's tokens file removed (U7R-NC-4)", async () => {
  const dir = scratch("reap");
  writeFileSync(join(dir, "tokens"), "0 test-only\n");
  const kids = [0, 1].map(() => track(spawn(process.execPath, ["-e", "setTimeout(() => {}, 60000)"], { stdio: "ignore" })));
  const exits = kids.map((k) => new Promise((ok) => k.on("exit", (code, signal) => ok(signal))));
  assert.equal(CHILDREN.size, 2);
  reap(dir);
  assert.deepEqual(await Promise.all(exits), ["SIGKILL", "SIGKILL"]);
  assert.equal(CHILDREN.size, 0);
  assert.equal(existsSync(join(dir, "tokens")), false);
  reap(null);
});
