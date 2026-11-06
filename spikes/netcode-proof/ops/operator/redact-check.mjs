#!/usr/bin/env node
// redact-check: refuses text that would leak an address, a secret or a canary into the repository (plan §10.6;
// security F14, F18, N10; R-21). Node 22, no dependencies and no shell tools, so the same file runs on Windows.
//
// It fails on:
//   - an IPv4 or IPv6 address outside the run's VM list and the short built-in allow-list, in dotted, IPv6,
//     IPv4-mapped, rDNS-embedded (dotted or dashed, forward or reversed, in a host name) and URL-decimal forms;
//   - the tokens this test mints (prefixes below) and common secret formats, except the runner's public join token;
//   - every canary value, and an IPv4 canary in its decimal, hex, dashed, reversed and mapped forms too;
//   - PEM and OpenSSH private-key headers, and PuTTY key files;
//   - a binary file, which cannot be checked (results are text only).
//
// Use:
//   node redact-check.mjs [options] <file or folder>...   check files as they are on disk
//   node redact-check.mjs [options] --staged [<path>...]    check what `git commit` would record (the index)
//   node redact-check.mjs [options] --pre-push              as a git pre-push hook: the pushed commits' files and
//                                                           messages, from the ref lines git gives on stdin
// Options: --vm <address> (repeatable: the run's VMs), --vms <file>, --allow <address or CIDR> (repeatable),
//          --canaries <file> (one value per line; # comments), --quiet.
// Exit: 0 clean, 1 findings, 2 bad usage or a git error. Findings print as file:line:col: kind: a masked excerpt.

import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { isIPv4, isIPv6 } from "node:net";
import { join, relative } from "node:path";
import { pathToFileURL } from "node:url";

// The token formats this test's code mints (U6 builds them to these prefixes): a join token per participant, the
// runner's single-use join token (public by design: it is a workflow input, §9 and §10.3), a session-control
// capability, and a join's bind secret. Each is the prefix plus at least 22 base64url characters (128 bits).
export const TOKEN_PREFIXES = { join: "nptj_", runnerJoin: "nptjr_", capability: "nptc_", bind: "nptb_" };

const B64URL = "[A-Za-z0-9_-]";
const SECRET_PATTERNS = [
  { kind: "join token", re: new RegExp(`(?<![A-Za-z0-9_])${TOKEN_PREFIXES.join}${B64URL}{22,}`, "g") },
  { kind: "capability", re: new RegExp(`(?<![A-Za-z0-9_])${TOKEN_PREFIXES.capability}${B64URL}{22,}`, "g") },
  { kind: "bind secret", re: new RegExp(`(?<![A-Za-z0-9_])${TOKEN_PREFIXES.bind}${B64URL}{22,}`, "g") },
  { kind: "GitHub token", re: /(?<![A-Za-z0-9_])gh[pousr]_[A-Za-z0-9]{36,}/g },
  { kind: "GitHub token", re: /(?<![A-Za-z0-9_])github_pat_[A-Za-z0-9_]{22,}/g },
  { kind: "API key", re: /(?<![A-Za-z0-9_])sk-[A-Za-z0-9_-]{20,}/g },
  { kind: "AWS key", re: /(?<![A-Za-z0-9_])(?:AKIA|ASIA)[0-9A-Z]{16}(?![A-Za-z0-9_])/g },
  { kind: "Slack token", re: /(?<![A-Za-z0-9_])xox[abprs]-[A-Za-z0-9-]{10,}/g },
  { kind: "private key", re: /-----BEGIN (?:[A-Z0-9]+ )*PRIVATE KEY-----/g },
  { kind: "private key", re: /PuTTY-User-Key-File-\d/g },
];
// A long mixed-case token (a Hetzner API token is 64 such characters; Cloudflare's are 40 with - and _). Pure hex
// (hashes, commits) and all-one-case words are not tokens.
const LONG_TOKEN = /(?<![A-Za-z0-9_-])[A-Za-z0-9_-]{40,}(?![A-Za-z0-9_-])/g;

function looksLikeToken(s) {
  if (/^[0-9a-fA-F]+$/.test(s)) return false;
  return /[a-z]/.test(s) && /[A-Z]/.test(s) && /[0-9]/.test(s);
}

// ---- Addresses -------------------------------------------------------------------------------------------------

function v4ToInt(a) {
  return a.split(".").reduce((n, o) => n * 256 + Number(o), 0);
}

function intToV4(n) {
  return [24, 16, 8, 0].map((s) => Math.floor(n / 2 ** s) % 256).join(".");
}

function parseCidr(text) {
  const [addr, bits] = text.split("/");
  if (isIPv4(addr)) {
    const b = bits === undefined ? 32 : Number(bits);
    if (!Number.isInteger(b) || b < 0 || b > 32) throw new Error(`bad CIDR ${text}`);
    return { v: 4, base: v4ToInt(addr), bits: b };
  }
  if (isIPv6(addr)) {
    const b = bits === undefined ? 128 : Number(bits);
    if (!Number.isInteger(b) || b < 0 || b > 128) throw new Error(`bad CIDR ${text}`);
    return { v: 6, base: v6ToBigInt(addr), bits: b };
  }
  throw new Error(`not an address or CIDR: ${text}`);
}

function v6ToBigInt(addr) {
  let a = addr.split("%")[0];
  // An embedded IPv4 tail becomes two groups.
  const tail = a.match(/(\d+\.\d+\.\d+\.\d+)$/);
  if (tail) {
    const n = v4ToInt(tail[1]);
    a = a.slice(0, -tail[1].length) + `${Math.floor(n / 65536).toString(16)}:${(n % 65536).toString(16)}`;
  }
  const [head, rest] = a.includes("::") ? a.split("::") : [a, null];
  const h = head ? head.split(":") : [];
  const r = rest === null ? [] : rest ? rest.split(":") : [];
  const groups = rest === null ? h : [...h, ...Array(8 - h.length - r.length).fill("0"), ...r];
  return groups.reduce((n, g) => n * 65536n + BigInt(parseInt(g || "0", 16)), 0n);
}

function inCidr(c, v, value) {
  if (c.v !== v) return false;
  const width = v === 4 ? 32 : 128;
  if (v === 4) {
    const span = 2 ** (width - c.bits);
    return Math.floor(value / span) === Math.floor(c.base / span);
  }
  const span = 2n ** BigInt(width - c.bits);
  return value / span === c.base / span;
}

// The short allow-list: loopback, unspecified, broadcast, the documentation ranges (RFC 5737, RFC 3849), the cloud
// metadata service the firewall rules name (169.254.169.254, fe80::a9fe:a9fe) and Cloudflare's public resolvers
// (1.1.1.1, 1.0.0.1), which the probes and DNS checks name. None of these is ever a person's address.
const BUILT_IN_ALLOW = [
  "127.0.0.0/8",
  "0.0.0.0/32",
  "255.255.255.255/32",
  "192.0.2.0/24",
  "198.51.100.0/24",
  "203.0.113.0/24",
  "169.254.169.254/32",
  "1.1.1.1/32",
  "1.0.0.1/32",
  "::/128",
  "::1/128",
  "2001:db8::/32",
  "fe80::a9fe:a9fe/128",
].map(parseCidr);

export function makeAllow(extra = []) {
  const list = [...BUILT_IN_ALLOW, ...extra.map(parseCidr)];
  return {
    v4: (a) => list.some((c) => inCidr(c, 4, v4ToInt(a))),
    v6: (a) => list.some((c) => inCidr(c, 6, v6ToBigInt(a))),
  };
}

const DOTTED = /(?<![0-9A-Za-z_.])(?:\d{1,3}\.){3}\d{1,3}(?![0-9A-Za-z_]|\.\d)/g;
const V6_CANDIDATE = /(?<![A-Za-z0-9_.:])[0-9A-Fa-f]*:[0-9A-Fa-f:.]*[0-9A-Fa-f:](?:%[0-9A-Za-z]+)?(?![A-Za-z0-9_:])/g;
const HOSTNAME = /(?<![A-Za-z0-9_.-])(?:[A-Za-z0-9-]+\.)+[A-Za-z][A-Za-z0-9-]*(?![A-Za-z0-9_-])/g;
const URL_DECIMAL = /(?<=:\/\/(?:[^/@\s]*@)?)\d{8,10}(?=[/:?#\s]|$)/g;

function quadsIn(labels) {
  // Every run of four numeric labels, as [forward, reversed]: rDNS names reverse the address.
  const out = [];
  for (let i = 0; i + 4 <= labels.length; i++) {
    const q = labels.slice(i, i + 4);
    if (q.every((l) => /^\d{1,3}$/.test(l) && Number(l) <= 255)) {
      out.push([q.map(Number).join("."), [...q].reverse().map(Number).join(".")]);
    }
  }
  return out;
}

// Subresource Integrity hashes (`sha512-…` in package locks) are hashes, not tokens: blanked before the long-token
// scan, keeping every index. Only a whole hash of its algorithm's exact base64 length counts (sha1 28 characters,
// sha256 44, sha384 64, sha512 88), so a `sha1-` written before a token cannot hide it.
const INTEGRITY =
  /(?<![A-Za-z0-9_-])sha(?:1-[A-Za-z0-9+/]{27}=|256-[A-Za-z0-9+/]{43}=|384-[A-Za-z0-9+/]{64}|512-[A-Za-z0-9+/]{86}==)(?![A-Za-z0-9+/=_-])/g;

function withoutIntegrity(text) {
  return text.replace(INTEGRITY, (m) => " ".repeat(m.length));
}

// ---- Scanning --------------------------------------------------------------------------------------------------

function mask(s) {
  return s.length <= 4 ? "…" : `${s.slice(0, 3)}…(${s.length} chars)`;
}

function position(text, index) {
  let line = 1;
  let last = -1;
  for (let i = text.indexOf("\n"); i !== -1 && i < index; i = text.indexOf("\n", i + 1)) {
    line++;
    last = i;
  }
  return { line, col: index - last };
}

/** The canary values, each IPv4 canary expanded to every form it could take. */
export function expandCanaries(values) {
  const out = new Set();
  for (const raw of values) {
    const v = raw.trim();
    if (!v || v.startsWith("#")) continue;
    out.add(v);
    if (isIPv4(v)) {
      const n = v4ToInt(v);
      const o = v.split(".");
      out.add(String(n));
      out.add(`0x${n.toString(16).padStart(8, "0")}`);
      out.add(`0x${n.toString(16).padStart(8, "0").toUpperCase()}`);
      out.add(o.join("-"));
      out.add([...o].reverse().join("."));
      out.add(`::ffff:${v}`);
      out.add(`::ffff:${Math.floor(n / 65536).toString(16)}:${(n % 65536).toString(16)}`);
    }
  }
  return [...out].filter((c) => c.length >= 4);
}

/**
 * Finds every leak in one text. `opts`: { allow (from makeAllow), vms (addresses allowed for this run), canaries
 * (from expandCanaries) }. Returns [{ index, kind, excerpt }], excerpts masked.
 */
export function scanText(text, opts = {}) {
  const allow = opts.allow ?? makeAllow();
  const vms = new Set(opts.vms ?? []);
  const canaries = opts.canaries ?? [];
  const raw = [];
  const add = (index, kind, s) => raw.push({ index, end: index + s.length, kind, excerpt: mask(s) });
  const v4Ok = (a) => vms.has(a) || allow.v4(a);
  const v6Ok = (a) => vms.has(a) || allow.v6(a);

  for (const m of text.matchAll(DOTTED)) {
    if (isIPv4(m[0]) && !v4Ok(m[0])) add(m.index, "IPv4 address", m[0]);
    else if (!isIPv4(m[0]) && m[0].split(".").every((o) => Number(o) <= 255)) {
      // Leading zeros (an octal reading): still an address to a resolver.
      const plain = m[0].split(".").map(Number).join(".");
      if (!v4Ok(plain)) add(m.index, "IPv4 address", m[0]);
    }
  }
  for (const m of text.matchAll(V6_CANDIDATE)) {
    const a = m[0].split("%")[0];
    if (!isIPv6(a)) continue;
    const tail = a.match(/(\d+\.\d+\.\d+\.\d+)$/);
    if (tail && /^::ffff:/i.test(a)) {
      if (!v4Ok(tail[1])) add(m.index, "IPv4-mapped address", m[0]);
    } else if (!v6Ok(a)) {
      add(m.index, "IPv6 address", m[0]);
    }
  }
  for (const m of text.matchAll(HOSTNAME)) {
    // A name embedding one of this run's VMs in either order is that VM's own name (rDNS reverses the address).
    // Otherwise both readings must be allowed: accepting either would let a real address through whenever its
    // reverse falls in an allowed range.
    const leak = ([f, r]) => !(vms.has(f) || vms.has(r) || (allow.v4(f) && allow.v4(r)));
    if (quadsIn(m[0].split(/[.-]/)).some(leak)) {
      add(m.index, "address in a host name", m[0]);
    }
  }
  for (const m of text.matchAll(URL_DECIMAL)) {
    const n = Number(m[0]);
    if (n >= 2 ** 24 && n < 2 ** 32 && !v4Ok(intToV4(n))) add(m.index, "decimal address in a URL", m[0]);
  }
  for (const { kind, re } of SECRET_PATTERNS) {
    for (const m of text.matchAll(re)) add(m.index, kind, m[0]);
  }
  for (const m of withoutIntegrity(text).matchAll(LONG_TOKEN)) {
    if (looksLikeToken(m[0]) && !m[0].startsWith(TOKEN_PREFIXES.runnerJoin)) add(m.index, "long token", m[0]);
  }
  for (const c of canaries) {
    for (let i = text.indexOf(c); i !== -1; i = text.indexOf(c, i + 1)) add(i, "canary", c);
  }
  // One finding per leak: a finding inside another's span (the dotted tail of a mapped address, a token that is
  // also long) is dropped; on equal spans the specific kind wins over "long token".
  raw.sort((a, b) => a.index - b.index || b.end - a.end || (a.kind === "long token") - (b.kind === "long token"));
  const kept = [];
  for (const f of raw) {
    if (!kept.some((k) => k.index <= f.index && f.end <= k.end)) kept.push(f);
  }
  return kept.map(({ index, kind, excerpt }) => ({ index, kind, excerpt }));
}

function isBinary(buf) {
  return buf.subarray(0, 8192).includes(0);
}

const SKIP_DIRS = new Set([".git", "node_modules", "target", ".gate"]);

function walk(path, out) {
  const st = statSync(path);
  if (st.isDirectory()) {
    for (const name of readdirSync(path).sort()) {
      if (!SKIP_DIRS.has(name)) walk(join(path, name), out);
    }
  } else if (st.isFile()) {
    out.push(path);
  }
}

function git(args, input) {
  return execFileSync("git", args, { input, maxBuffer: 1 << 30 });
}

/** Checks named buffers; returns the count of findings, printing each unless quiet. */
export function checkBuffers(items, opts, print = console.log) {
  let count = 0;
  for (const { name, buf } of items) {
    if (isBinary(buf)) {
      print(`${name}:1:1: binary file: cannot be checked (results are text only)`);
      count++;
      continue;
    }
    const text = buf.toString("utf8");
    for (const f of scanText(text, opts)) {
      const { line, col } = position(text, f.index);
      print(`${name}:${line}:${col}: ${f.kind}: ${f.excerpt}`);
      count++;
    }
  }
  return count;
}

function stagedItems(paths) {
  const names = git(["diff", "--cached", "--name-only", "--diff-filter=ACMR", "-z", "--", ...paths])
    .toString("utf8")
    .split("\0")
    .filter(Boolean);
  return names.map((name) => ({ name, buf: git(["show", `:${name}`]) }));
}

function prePushItems(stdin) {
  const zero = /^0+$/;
  const items = [];
  for (const line of stdin.split("\n").filter(Boolean)) {
    const [, localSha, , remoteSha] = line.trim().split(/\s+/);
    if (!localSha || zero.test(localSha)) continue;
    const range = zero.test(remoteSha ?? "0") ? [localSha, "--not", "--remotes"] : [`${remoteSha}..${localSha}`];
    const commits = git(["rev-list", ...range]).toString("utf8").split("\n").filter(Boolean);
    for (const c of commits) {
      items.push({ name: `${c.slice(0, 12)}:(message)`, buf: git(["log", "-1", "--format=%B", c]) });
      // Each commit against its first parent, so a merge's own changes (a conflict resolved by hand) are read too:
      // `git diff-tree` lists nothing for a merge commit. A root commit lists every file.
      const parents = git(["rev-list", "--parents", "-n", "1", c]).toString("utf8").trim().split(/\s+/).slice(1);
      const list = parents.length
        ? ["diff", "--name-only", "-z", "--diff-filter=ACMR", "--no-renames", parents[0], c]
        : ["diff-tree", "--no-commit-id", "--name-only", "-r", "-z", "--diff-filter=ACMR", "--root", c];
      const files = git(list).toString("utf8").split("\0").filter(Boolean);
      for (const f of files) items.push({ name: `${c.slice(0, 12)}:${f}`, buf: git(["show", `${c}:${f}`]) });
    }
  }
  return items;
}

function readLines(file) {
  return readFileSync(file, "utf8").split(/\r?\n/);
}

export function main(argv, stdinText = () => readFileSync(0, "utf8"), print = console.log) {
  const vms = [];
  const allowExtra = [];
  let canaryValues = [];
  let mode = "files";
  let quiet = false;
  const paths = [];
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const value = () => {
      if (i + 1 >= argv.length) throw new Error(`${a} needs a value`);
      return argv[++i];
    };
    if (a === "--vm") vms.push(value());
    else if (a === "--vms") vms.push(...readLines(value()).map((l) => l.trim()).filter((l) => l && !l.startsWith("#")));
    else if (a === "--allow") allowExtra.push(value());
    else if (a === "--canaries") canaryValues = canaryValues.concat(readLines(value()));
    else if (a === "--staged") mode = "staged";
    else if (a === "--pre-push") mode = "pre-push";
    else if (a === "--quiet") quiet = true;
    else if (a.startsWith("--")) throw new Error(`unknown option ${a}`);
    else paths.push(a);
  }
  for (const v of vms) {
    if (!isIPv4(v) && !isIPv6(v)) throw new Error(`--vm ${v} is not an address`);
  }
  const opts = { allow: makeAllow(allowExtra), vms, canaries: expandCanaries(canaryValues) };
  let items;
  if (mode === "staged") items = stagedItems(paths);
  else if (mode === "pre-push") items = prePushItems(stdinText());
  else {
    if (paths.length === 0) throw new Error("name at least one file or folder, or use --staged or --pre-push");
    const files = [];
    for (const p of paths) walk(p, files);
    items = files.map((f) => ({ name: relative(process.cwd(), f) || f, buf: readFileSync(f) }));
  }
  const count = checkBuffers(items, opts, quiet ? () => {} : print);
  if (!quiet) print(`redact-check: ${items.length} item(s) checked, ${count} finding(s)`);
  return count === 0 ? 0 : 1;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = main(process.argv.slice(2));
  } catch (e) {
    console.error(`redact-check: ${e.message}`);
    process.exitCode = 2;
  }
}
