// Tests for redact-check (plan §10.6). Every sensitive sample is built at run time, so this file itself stays clean.
// Run: node --test spikes/netcode-proof/ops/operator/redact-check.test.mjs

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { TOKEN_PREFIXES, checkBuffers, expandCanaries, main, makeAllow, scanText } from "./redact-check.mjs";

const pub4 = [8, 8, 4, 4].join("."); // a public address, never in the allow-list
const vm4 = [49, 12, 0, 7].join("."); // stands for a run's VM
const pub6 = ["2a01", "4f8", "c0c", "1234", "", "1"].join(":");
const kinds = (text, opts) => scanText(text, opts).map((f) => f.kind);

test("allowed addresses pass", () => {
  for (const text of ["127.0.0.1", "203.0.113.7:443", "[2001:db8::5]:443", "::1", "0.0.0.0", "198.51.100.2"]) {
    assert.deepEqual(kinds(`at ${text} now`), [], text);
  }
  assert.deepEqual(kinds(`RS is ${vm4}`, { vms: [vm4] }), [], "a run's VM is allowed");
  const priv = [10, 0, 0, 5].join(".");
  assert.deepEqual(kinds(`net ${priv}`), ["IPv4 address"], "a private address is not on the built-in list");
  assert.deepEqual(kinds(`net ${priv}`, { allow: makeAllow([`${[10, 0, 0, 0].join(".")}/16`]) }), [], "an allowed CIDR");
  assert.deepEqual(kinds(`metadata ${[169, 254, 169, 254].join(".")}, resolver ${[1, 1, 1, 1].join(".")}`), []);
});

test("other addresses fail, in every form", () => {
  assert.deepEqual(kinds(`the operator at ${pub4}.`), ["IPv4 address"]);
  assert.deepEqual(kinds(`peer ${pub6}`), ["IPv6 address"]);
  assert.deepEqual(kinds(`peer [${pub6}%eth0]:443`), ["IPv6 address"]);
  assert.deepEqual(kinds(`mapped ::ffff:${pub4}`), ["IPv4-mapped address"]);
  assert.deepEqual(kinds(`mapped ::ffff:203.0.113.9`), [], "a mapped documentation address");
  const reversed = pub4.split(".").reverse().join(".");
  assert.deepEqual(kinds(`rdns static.${reversed}.clients.your-server.de`), ["address in a host name"]);
  assert.deepEqual(kinds(`ptr ${reversed}.in-addr.arpa`), ["address in a host name"]);
  assert.deepEqual(kinds(`host ip-${pub4.replaceAll(".", "-")}.eu.example`), ["address in a host name"]);
  const vmRev = vm4.split(".").reverse().join(".");
  assert.deepEqual(kinds(`rdns static.${vmRev}.clients.your-server.de`, { vms: [vm4] }), [], "the VM's own rDNS");
  const decimal = String(pub4.split(".").reduce((n, o) => n * 256 + Number(o), 0));
  assert.deepEqual(kinds(`curl http://${decimal}/x`), ["decimal address in a URL"]);
  const padded = pub4
    .split(".")
    .map((o) => o.padStart(3, "0"))
    .join(".");
  assert.deepEqual(kinds(`zero-padded ${padded}`), ["IPv4 address"]);
  assert.deepEqual(kinds(`bare ${["ab", "cd"].join("::")} here`), ["IPv6 address"], "a valid IPv6 literal");
});

test("versions, dates, times and Rust paths are not addresses", () => {
  // Two hex words joined by a double colon form a valid IPv6 literal and are flagged by design (the test above);
  // Rust paths are followed by a name, so they pass.
  const text = [
    "Rust 1.99.1, quinn-proto 0.11.18, Chromium 153.0.7299.0, five parts 1.2.3.4.5",
    "2026-11-06 at 10:22:30, 2026-11-06T09:53:12.287Z, std::time::Instant, Value::from(x), de::Error, a::b::c, fe::fi",
    "docs/adr/0001-multiplayer-netcode.md, crates.io, PM-0001-4/B-S, aa:bb:cc:dd:ee:ff",
  ].join("\n");
  assert.deepEqual(kinds(text), []);
});

test("tokens and keys fail, the runner's public token passes", () => {
  const b = "Q".repeat(11) + "q".repeat(11);
  assert.deepEqual(kinds(`token ${TOKEN_PREFIXES.join}${b}`), ["join token"]);
  assert.deepEqual(kinds(`cap ${TOKEN_PREFIXES.capability}${b}`), ["capability"]);
  assert.deepEqual(kinds(`bind ${TOKEN_PREFIXES.bind}${b}`), ["bind secret"]);
  assert.deepEqual(kinds(`runner ${TOKEN_PREFIXES.runnerJoin}${b}9`), [], "public by design (§9, §10.3)");
  assert.deepEqual(kinds(`${"gh"}p_${"aB3".repeat(12)}`), ["GitHub token"]);
  assert.deepEqual(kinds(`${"sk"}-ant-${"x1Y".repeat(10)}`), ["API key"]);
  assert.deepEqual(kinds(`${"AK"}IA${"ABCDEFGHIJKLMNOP"}`), ["AWS key"]);
  assert.deepEqual(kinds(`hetzner ${"aB3x".repeat(16)}`), ["long token"]);
  assert.deepEqual(kinds(`upper-case hash ${"aB3d".repeat(16)}`), [], "mixed-case hex is still a hash");
  assert.deepEqual(kinds(`sha256 ${"ab12".repeat(16)}, commit ${"0f".repeat(20)}`), [], "hashes are not tokens");
  assert.deepEqual(kinds(`"integrity": "sha512-${"Ab9+/".repeat(18)}=="`), [], "an SRI hash is not a token");
  assert.deepEqual(kinds(`${"-----BEGIN "}OPENSSH PRIVATE KEY-----`), ["private key"]);
  assert.deepEqual(kinds(`${"-----BEGIN "}EC PRIVATE KEY-----`), ["private key"]);
  assert.deepEqual(kinds(`${"-----BEGIN "}PUBLIC KEY-----`), [], "a public key is fine");
  assert.deepEqual(kinds(`${"PuTTY-User"}-Key-File-3: ssh-ed25519`), ["private key"]);
});

test("canaries fail in every form an address takes", () => {
  const forms = expandCanaries([pub4, "# a comment", "", "guest-secret-canary-1"]);
  assert.ok(forms.includes(pub4));
  assert.ok(forms.includes("134743044"), "decimal");
  assert.ok(forms.includes("0x08080404"), "hex");
  const lettered = expandCanaries([[172, 217, 0, 1].join(".")]);
  assert.ok(lettered.includes("0xacd90001") && lettered.includes("0xACD90001"), "hex in both cases");
  assert.ok(forms.includes(pub4.replaceAll(".", "-")), "dashed");
  assert.ok(forms.includes(pub4.split(".").reverse().join(".")), "reversed");
  assert.ok(forms.includes(`::ffff:${pub4}`) && forms.includes(`::ffff:${["808", "404"].join(":")}`), "mapped");
  assert.ok(forms.includes("guest-secret-canary-1"));
  assert.deepEqual(kinds("n = 134743044;", { canaries: forms }), ["canary"]);
  assert.deepEqual(kinds("left a guest-secret-canary-1 here", { canaries: forms }), ["canary"]);
});

test("a binary file cannot be checked, so it fails", () => {
  const lines = [];
  const n = checkBuffers([{ name: "shot.png", buf: Buffer.from([0x89, 0x50, 0, 1, 2]) }], {}, (l) => lines.push(l));
  assert.equal(n, 1);
  assert.match(lines[0], /^shot\.png:1:1: binary file/);
});

test("findings name file, line and column, and mask the value", () => {
  const lines = [];
  const text = `line one\n  at ${pub4} two`;
  checkBuffers([{ name: "notes.md", buf: Buffer.from(text) }], {}, (l) => lines.push(l));
  assert.equal(lines.length, 1);
  assert.equal(lines[0], "notes.md:2:6: IPv4 address: 8.8…(7 chars)");
  assert.ok(!lines[0].includes(pub4), "the address itself is not printed");
});

function tempDir() {
  return mkdtempSync(join(tmpdir(), "redact-check-"));
}

test("the command line: files and folders, exit codes, bad usage", () => {
  const dir = tempDir();
  try {
    writeFileSync(join(dir, "clean.md"), "RS at 203.0.113.4\n");
    assert.equal(main([dir], undefined, () => {}), 0);
    writeFileSync(join(dir, "leak.md"), `operator ${pub4}\n`);
    const out = [];
    assert.equal(main([dir], undefined, (l) => out.push(l)), 1);
    assert.ok(out.some((l) => l.includes("leak.md:1:10: IPv4 address")), out.join("\n"));
    assert.equal(main(["--vm", pub4, dir], undefined, () => {}), 0, "allowed as this run's VM");
    assert.throws(() => main([], undefined, () => {}), /name at least one/);
    assert.throws(() => main(["--vm", "not-an-ip", dir], undefined, () => {}), /not an address/);
    assert.throws(() => main(["--what"], undefined, () => {}), /unknown option/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("--staged reads the index, and --pre-push reads the pushed commits and their messages", () => {
  const dir = tempDir();
  const cwd = process.cwd();
  const env = {
    ...process.env,
    GIT_AUTHOR_NAME: "t",
    GIT_AUTHOR_EMAIL: "t@example.invalid",
    GIT_COMMITTER_NAME: "t",
    GIT_COMMITTER_EMAIL: "t@example.invalid",
  };
  const git = (...args) => execFileSync("git", args, { cwd: dir, env }).toString("utf8").trim();
  try {
    git("init", "-q");
    writeFileSync(join(dir, "a.md"), "clean\n");
    git("add", "a.md");
    process.chdir(dir);
    assert.equal(main(["--staged"], undefined, () => {}), 0);
    writeFileSync(join(dir, "a.md"), `staged ${pub4}\n`);
    git("add", "a.md");
    writeFileSync(join(dir, "a.md"), "clean again on disk\n");
    assert.equal(main(["--staged"], undefined, () => {}), 1, "the index holds the leak, the disk does not");
    git("add", "a.md");
    git("commit", "-q", "-m", `a message naming ${pub4}`);
    const sha = git("rev-parse", "HEAD");
    const zero = "0".repeat(40);
    const out = [];
    const stdin = `refs/heads/main ${sha} refs/heads/main ${zero}\n`;
    assert.equal(main(["--pre-push"], () => stdin, (l) => out.push(l)), 1);
    assert.ok(out.some((l) => l.includes("(message):1:18: IPv4 address")), out.join("\n"));
    assert.equal(main(["--pre-push"], () => `refs/heads/x ${zero} refs/heads/x ${sha}\n`, () => {}), 0, "a delete pushes no content");
  } finally {
    process.chdir(cwd);
    rmSync(dir, { recursive: true, force: true });
  }
});
