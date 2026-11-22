// security-auditor probe for F26 (read-only): imports the real redact-check.mjs, and in-memory plants of it through a
// data: URL (no file written). Every address-shaped sample is built at run time, so this file stays clean.
// Run: node .checkpoints/netcode-proof-test/security-u6c/f26-probe.mjs
import { readFileSync } from "node:fs";
import { scanText, expandCanaries } from "../../../spikes/netcode-proof/ops/operator/redact-check.mjs";

const SRC_URL = new URL("../../../spikes/netcode-proof/ops/operator/redact-check.mjs", import.meta.url);
const src = readFileSync(SRC_URL, "utf8");

const pub = [8, 8, 4, 4];
const dotted = pub.join(".");
const dashed = pub.join("-");
const revDashed = [...pub].reverse().join("-");
const reg = "https://" + ["registry", "npmjs", "org"].join(".");
const regHost = ["registry", "npmjs", "org"].join(".");
const file = `crc-32-${[1, 2, 2].join(".")}.tgz`;
const ipName = `ip-${dashed}.example`;
const b22 = "Q".repeat(11) + "q".repeat(11);

const H = "address in a host name";
const cases = [
  ["A lock line", `"resolved": "${reg}/crc-32/-/${file}",`, []],
  ["B scoped", `"resolved": "${reg}/@sc/crc-32/-/${file}",`, []],
  ["C other host", `https://example.org/crc-32/-/${file}`, [H]],
  ["D http scheme", `http://${regHost}/crc-32/-/${file}`, [H]],
  ["E port 443", `https://${regHost}:443/crc-32/-/${file}`, [H]],
  ["F userinfo before host", `https://u@${regHost}/crc-32/-/${file}`, [H]],
  ["G userinfo-shaped registry then other host", `https://${regHost}@evil.example/crc-32/-/${file}`, [H]],
  ["H trailing-dot host", `https://${regHost}./crc-32/-/${file}`, [H]],
  ["I registry as a label of another host", `https://${regHost}.evil.example/crc-32/-/${file}`, [H]],
  ["J address-shaped subdomain", `https://${ipName.replace(".example", "")}.${regHost}/x/-/x-${file}`, [H, H]],
  ["K no /-/ segment", `${reg}/crc-32/${file}`, [H]],
  ["L root path", `${reg}/${file}`, [H]],
  ["M not .tgz", `${reg}/crc-32/-/${file.replace(".tgz", ".tar")}`, [H]],
  ["N a segment after .tgz", `${reg}/crc-32/-/${file}/${ipName}`, [H, H]],
  ["O query after .tgz", `${reg}/x/-/x.tgz?h=${ipName}`, [H]],
  ["P fragment after .tgz", `${reg}/x/-/x.tgz#${ipName}`, [H]],
  ["Q same line after the URL", `"${reg}/crc-32/-/${file}" then ${ipName}`, [H]],
  ["R next line", `"${reg}/crc-32/-/${file}"\nthen ${ipName}`, [H]],
  ["S residue: dashed address in the file name", `${reg}/x/-/ip-${dashed}.tgz`, []],
  ["T residue: dashed address in the name segment", `${reg}/${ipName}/-/x.tgz`, []],
  ["U residue: dashed address in the scope segment", `${reg}/@${ipName}/x/-/x.tgz`, []],
  ["V dotted address in the name segment", `${reg}/x-${dotted}/-/x.tgz`, ["IPv4 address"]],
  ["W canary (dashed form) in the file name", `${reg}/x/-/x-${dashed}.tgz`, ["canary"], { canaries: expandCanaries([dotted]) }],
  ["X residue: canary reversed-dashed in the file name", `${reg}/x/-/x-${revDashed}.tgz`, [], { canaries: expandCanaries([dotted]) }],
  ["Y join token in the file name", `${reg}/x/-/nptj_${b22}.tgz`, ["join token"]],
  ["Z long token in the file name", `${reg}/x/-/${"aB3x".repeat(12)}.tgz`, ["long token"]],
  ["AA git+https prefix", `git+${reg}/crc-32/-/${file}`, []],
  ["AB no left boundary (xhttps)", `x${reg}/crc-32/-/${file}`, []],
  ["AC nested in another URL's query", `https://evil.example/r?u=${reg}/x/-/ip-${dashed}.tgz`, []],
  ["AD upper-case host", `https://${regHost.toUpperCase()}/crc-32/-/${file}`, [H]],
  ["AE percent-encoded scope", `${reg}/@sc%2fcrc-32/-/${file}`, [H]],
  ["AF mapped address in the path", `${reg}/x/-/x-::ffff:${dotted}.tgz`, ["IPv4-mapped address"]],
  ["AG two URLs, the second another host", `${reg}/a/-/a.tgz https://example.org/crc-32/-/${file}`, [H]],
];

function run(scan, label) {
  let bad = 0;
  for (const [name, text, want, opts] of cases) {
    const got = scan(text, opts).map((f) => f.kind);
    const ok = JSON.stringify(got) === JSON.stringify(want);
    if (!ok) bad++;
    if (!ok || label === "real") console.log(`${label} ${ok ? "ok  " : "DIFF"} ${name}: got ${JSON.stringify(got)}`);
  }
  console.log(`${label}: ${cases.length - bad}/${cases.length} as expected`);
}

// The F26 test's six assertions (redact-check.test.mjs:52-61), replayed against each plant.
const pub4 = dotted;
function f26Test(scan) {
  const kinds = (t) => scan(t).map((f) => f.kind);
  const host = `ip-${pub4.replaceAll(".", "-")}.${regHost}`;
  const checks = [
    ["lock line", kinds(`"resolved": "${reg}/crc-32/-/${file}",`), []],
    ["scoped", kinds(`"resolved": "${reg}/@sc/crc-32/-/${file}",`), []],
    ["another host's path", kinds(`https://example.org/crc-32/-/${file}`), [H]],
    ["dotted in it", kinds(`${reg}/x/-/x-${pub4}.tgz`), ["IPv4 address"]],
    ["address-shaped host", kinds(`https://${host}/x/-/x-${file}`), [H, H]],
  ];
  return checks.filter(([, got, want]) => JSON.stringify(got) !== JSON.stringify(want)).map(([n]) => n);
}

async function planted(name, from, to) {
  if (!src.includes(from)) throw new Error(`plant ${name}: anchor not found`);
  const text = src.replace(from, to);
  const mod = await import("data:text/javascript;base64," + Buffer.from(text).toString("base64"));
  const failing = f26Test(mod.scanText);
  console.log(`plant ${name}: F26 test assertions failing: ${failing.length ? failing.join(", ") : "none (plant survives)"}`);
  run(mod.scanText, `  plant ${name}`);
}

run(scanText, "real");
console.log(`real F26 test assertions failing: ${f26Test(scanText).join(", ") || "none"}`);
await planted("any-host", "registry\\.npmjs\\.org\\/", "[A-Za-z0-9.-]+\\/");
await planted("no-/-/", "[A-Za-z0-9._~-]+\\/-\\/[A-Za-z0-9._~+-]+\\.tgz", "[A-Za-z0-9._~\\/-]+\\.tgz");
await planted("no-.tgz", "\\.tgz(?![A-Za-z0-9._~+\\/-])", "(?![A-Za-z0-9._~+\\/-])");
await planted("end-to-text-end", "m.index + m[0].length]);", "text.length]);");
await planted("whole-text-once-any-tarball", "from <= m.index && end <= to", "true");
