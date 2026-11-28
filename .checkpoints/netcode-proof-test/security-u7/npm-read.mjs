// security-auditor scratch (read-only, writes nothing): one npm package version's registry facts, then its tarball
// read in memory (sha512/sha1 against dist, tar entry names, package.json scripts, binding.gyp).
// Use: NODE_USE_ENV_PROXY=1 node npm-read.mjs <name> <version> <before-ISO-date>
import { createHash } from "node:crypto";
import { gunzipSync } from "node:zlib";

const [name, version, before] = process.argv.slice(2);
const reg = "https://registry.npmjs.org/";
const res = await fetch(reg + encodeURIComponent(name), { headers: { accept: "application/json" } });
if (!res.ok) throw new Error(`packument HTTP ${res.status}`);
const doc = await res.json();
const v = doc.versions?.[version];
if (!v) throw new Error(`no ${name}@${version}`);
const time = doc.time ?? {};
console.log(`packument read at ${new Date().toISOString()}; created ${time.created}; modified ${time.modified}`);
console.log(`dist-tags: ${JSON.stringify(doc["dist-tags"])}`);
console.log(`${version} published ${time[version]}; before ${before}: ${new Date(time[version]) < new Date(before)}`);
const [maj, min] = version.split(".");
const line = Object.keys(doc.versions).filter((x) => x.startsWith(`${maj}.${min}.`));
console.log(`${maj}.${min}.x: ${line.map((x) => `${x} (${time[x]})`).join(", ")}`);
const stable = Object.keys(doc.versions).filter((x) => /^\d+\.\d+\.\d+$/.test(x));
const newer = stable.filter((x) => new Date(time[x]) > new Date(time[version]));
console.log(`stable versions published after ${version}: ${newer.map((x) => `${x} (${time[x]})`).join(", ") || "none"}`);
if (doc.versions[version].deprecated) console.log(`DEPRECATED: ${doc.versions[version].deprecated}`);
const pick = ["name", "version", "license", "bin", "engines", "scripts", "dependencies", "optionalDependencies",
  "peerDependencies", "bundleDependencies", "hasInstallScript", "gypfile", "os", "cpu", "deprecated"];
for (const k of pick) if (v[k] !== undefined) console.log(`manifest ${k}: ${JSON.stringify(v[k])}`);
console.log(`publisher: ${JSON.stringify(v._npmUser)}`);
const d = v.dist ?? {};
console.log(`dist.tarball ${d.tarball}`);
console.log(`dist.integrity ${d.integrity}`);
console.log(`dist.shasum ${d.shasum}; fileCount ${d.fileCount}; unpackedSize ${d.unpackedSize}`);
console.log(`dist.signatures ${JSON.stringify((d.signatures ?? []).map((s) => s.keyid))}`);
console.log(`dist.attestations ${d.attestations ? JSON.stringify({ url: d.attestations.url, type: d.attestations.provenance?.predicateType }) : "none"}`);

const tr = await fetch(d.tarball);
if (!tr.ok) throw new Error(`tarball HTTP ${tr.status}`);
const tgz = Buffer.from(await tr.arrayBuffer());
const sha512 = "sha512-" + createHash("sha512").update(tgz).digest("base64");
const sha1 = createHash("sha1").update(tgz).digest("hex");
console.log(`tarball ${tgz.length} B; sha512 ${sha512 === d.integrity ? "EQUAL" : "DIFFERENT " + sha512}; sha1 ${sha1 === d.shasum ? "EQUAL" : "DIFFERENT " + sha1}`);
const tar = gunzipSync(tgz);
const names = [];
let pkgJson = null;
for (let off = 0; off + 512 <= tar.length; ) {
  const h = tar.subarray(off, off + 512);
  if (h.every((b) => b === 0)) break;
  const nm = h.subarray(0, 100).toString("utf8").replace(/\0.*$/s, "");
  const prefix = h.subarray(345, 500).toString("utf8").replace(/\0.*$/s, "");
  const size = parseInt(h.subarray(124, 136).toString("utf8").replace(/\0.*$/s, "").trim() || "0", 8);
  const type = String.fromCharCode(h[156] || 48);
  const full = prefix ? `${prefix}/${nm}` : nm;
  if (type === "0" || type === "\0") names.push(full);
  if (full === "package/package.json") pkgJson = JSON.parse(tar.subarray(off + 512, off + 512 + size).toString("utf8"));
  off += 512 + Math.ceil(size / 512) * 512;
}
console.log(`tar files ${names.length}; binding.gyp ${names.some((n) => /(^|\/)binding\.gyp$/.test(n))}; .npmrc ${names.some((n) => /(^|\/)\.npmrc$/.test(n))}`);
console.log(`tar package.json version ${pkgJson?.version}; scripts ${JSON.stringify(pkgJson?.scripts ?? {})}`);
const life = ["preinstall", "install", "postinstall", "prepare", "preprepare", "postprepare"];
console.log(`install-time lifecycle scripts in tar package.json: ${JSON.stringify(life.filter((k) => pkgJson?.scripts?.[k]))}`);
console.log(`tar package.json dependencies ${JSON.stringify(pkgJson?.dependencies ?? {})}; optional ${JSON.stringify(pkgJson?.optionalDependencies ?? {})}`);
console.log(`top-level dirs: ${[...new Set(names.map((n) => n.split("/").slice(0, 2).join("/")))].join(" ")}`);
