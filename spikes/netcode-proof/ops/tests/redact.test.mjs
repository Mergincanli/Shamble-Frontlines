// Test 6 (u8-brief.md rev 1.1 section 3; advisor-u6c-brief.md section 5 item 6): redact-check over the whole spike
// folder, as check.mjs's `redact` step scans it (node_modules, target and .gate skipped by redact-check itself), so no
// file carries an address, a token or a key. Plant P4 (a real-looking address in R1.md) fails here. Since F26's fix
// (2a9498d: an npm tarball path on registry.npmjs.org is no host name) the scan has no finding at all.
import assert from "node:assert/strict";
import { test } from "node:test";
import { main } from "../operator/redact-check.mjs";
import { SPIKE } from "./helpers.mjs";

test("redact-check finds nothing in spikes/netcode-proof", () => {
  const lines = [];
  const cwd = process.cwd();
  process.chdir(SPIKE);
  try {
    main(["."], () => "", (l) => lines.push(l));
  } finally {
    process.chdir(cwd);
  }
  assert.deepEqual(lines.filter((l) => !l.startsWith("redact-check:")), []);
  assert.ok(lines.some((l) => /^redact-check: \d+ item\(s\) checked, 0 finding\(s\)/.test(l)), lines.join("\n"));
});
