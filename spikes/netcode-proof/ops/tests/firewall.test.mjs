// Test 2 (u8-brief.md rev 1.1 section 3): the two Hetzner firewall lists equal section 1 Block 2's table.
import assert from "node:assert/strict";
import { test } from "node:test";
import { leaks, text } from "./helpers.mjs";

// Block 2's table, rule for rule.
const TABLE = {
  "firewall/rs.txt": [
    ["in", "udp", "30001-30032", "<HOME_IPV4>/32 <LG_IPV4>/32"],
    ["in", "tcp", "31000-31032", "<HOME_IPV4>/32 <LG_IPV4>/32"],
    ["in", "tcp", "9100", "<LG_IPV4>/32"],
    ["in", "tcp", "22", "<HOME_IPV4>/32"],
  ],
  "firewall/lg.txt": [
    ["in", "tcp", "22", "<RS_IPV4>/32"],
    ["in", "icmp", "-", "<HOME_IPV4>/32"],
  ],
};
const PLACEHOLDER = /^<(HOME_IPV4|LG_IPV4|RS_IPV4)>\/32$/;

export function rules(body) {
  return body
    .split("\n")
    .filter((l) => l.trim() && !l.trimStart().startsWith("#"))
    .map((l) => {
      const [dir, proto, port, ...sources] = l.trim().split(/\s+/);
      return [dir, proto, port, sources.join(" ")];
    });
}

for (const [rel, want] of Object.entries(TABLE)) {
  const body = text(rel);
  const got = rules(body);

  test(`${rel}: each rule equals Block 2's table`, () => {
    assert.deepEqual(got, want);
  });

  test(`${rel}: no TCP 80, no any-source, placeholders only`, () => {
    for (const [, proto, port, sources] of got) {
      const [lo, hi = lo] = port.split("-").map(Number);
      assert.ok(!(proto === "tcp" && lo <= 80 && 80 <= hi), `${rel}: no rule admits TCP 80`);
      for (const s of sources.split(" ")) {
        assert.match(s, PLACEHOLDER, `${rel}: source ${s} is a placeholder /32, never any`);
      }
    }
    assert.doesNotMatch(body.replace(/^#.*$/gm, ""), /0\.0\.0\.0\/0|::\/0|\bany\b/i);
    assert.deepEqual(leaks(body), []);
  });
}

test("fw-rs: no ICMP rule and no IPv6 source (the test runs on IPv4, plan.md:956)", () => {
  const got = rules(text("firewall/rs.txt"));
  assert.ok(got.every(([, proto]) => proto === "udp" || proto === "tcp"));
});
