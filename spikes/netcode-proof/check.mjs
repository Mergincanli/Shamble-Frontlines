#!/usr/bin/env node
// The spike's test command (plan §12.4): every unit runs it, and its summary goes into the checkpoint.
//   node spikes/netcode-proof/check.mjs [--only <step,...>]
// Steps: fmt, clippy (host, all targets) and clippy-wasm32 (the leaf crates), each on the measured build and on the
// bites build with warnings as errors; test (cargo test on the measured build, then on the bites build); deps (the
// dependency rules of plan §4.2 that `cargo metadata` shows, with every feature and target, and its unsafe_code rule,
// from the crates' files: DEPS_CHECKED lists them); refs (nothing outside spikes/ references it, §4.2 and §4.9); marks
// (marks.json compiles in the judge, covers exactly the plan's marks, and every row it hashed is unchanged); self (this
// file's own tests); redact (redact-check's tests, then a scan of the spike's folder). It runs every step and exits 1
// if any failed.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = join(HERE, "..", "..");

/** The leaf crates (plan §4.2): built for wasm32, free of every transport crate and async runtime. */
export const LEAF = ["npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-client", "npt-log"];
/** Transport crates and async runtimes no leaf crate may reach (plan §4.2's list, with their own parts). */
export const TRANSPORT = [
  "tokio",
  "mio",
  "quinn",
  "quinn-proto",
  "quinn-udp",
  "wtransport",
  "web-transport-quinn",
  "tungstenite",
  "tokio-tungstenite",
  "async-std",
  "smol",
];

/** Each workspace crate's closure over normal and build dependencies (dev dependencies never ship). */
export function closures(meta) {
  const byId = new Map(meta.packages.map((p) => [p.id, p]));
  const nodes = new Map(meta.resolve.nodes.map((n) => [n.id, n]));
  const shipped = (d) => d.dep_kinds.some((k) => k.kind === null || k.kind === "build");
  const memo = new Map();
  const closure = (id) => {
    if (memo.has(id)) return memo.get(id);
    const out = new Set();
    memo.set(id, out);
    for (const d of nodes.get(id)?.deps ?? []) {
      if (!shipped(d)) continue;
      out.add(byId.get(d.pkg).name);
      for (const n of closure(d.pkg)) out.add(n);
    }
    return out;
  };
  const result = new Map();
  for (const id of meta.workspace_members) result.set(byId.get(id).name, closure(id));
  return result;
}

/** A normal or build dependency ships; a dev dependency does not. */
const isShipped = (d) => d.dep_kinds.some((k) => k.kind === null || k.kind === "build");

/** Every broken dependency rule, as a sentence; empty when the rules hold. */
export function dependencyProblems(meta) {
  const problems = [];
  const byId = new Map(meta.packages.map((p) => [p.id, p]));
  const all = closures(meta);
  const reach = (crate) => all.get(crate) ?? new Set();
  for (const leaf of LEAF) {
    if (!all.has(leaf)) problems.push(`${leaf} is not a workspace member`);
    for (const t of TRANSPORT) if (reach(leaf).has(t)) problems.push(`leaf crate ${leaf} depends on ${t}`);
  }
  if (reach("npt-client").has("npt-net")) problems.push("npt-client depends on npt-net");
  for (const c of ["npt-sim", "npt-room", "npt-proto"]) {
    if (reach("npt-super").has(c)) problems.push(`npt-super depends on ${c}`);
  }
  for (const c of ["npt-sim", "npt-room", "npt-proto", "npt-client"]) {
    if (reach("npt-host").has(c)) problems.push(`npt-host depends on ${c}`);
  }
  // npt-judge: only npt-log and serde (with serde_json), so its verdicts share no code with what they judge.
  const judgeId = meta.workspace_members.find((id) => byId.get(id).name === "npt-judge");
  const judgeNode = meta.resolve.nodes.find((n) => n.id === judgeId);
  for (const d of judgeNode?.deps ?? []) {
    const name = byId.get(d.pkg).name;
    const shipped = d.dep_kinds.some((k) => k.kind === null || k.kind === "build");
    if (shipped && !["npt-log", "serde", "serde_json"].includes(name)) problems.push(`npt-judge depends on ${name}`);
  }
  for (const name of reach("npt-judge")) {
    if (name.startsWith("npt-") && name !== "npt-log") problems.push(`npt-judge reaches ${name}`);
  }
  // npt-tcpinfo (U3c, R20 (a); u3c-plan.md 1.8 (d)): libc is its only shipped dependency, and only npt-net depends on it,
  // in any kind, so its two functions are reached through npt-net alone.
  const tcpinfoId = meta.workspace_members.find((id) => byId.get(id).name === "npt-tcpinfo");
  if (!tcpinfoId) problems.push("npt-tcpinfo is not a workspace member");
  else {
    const node = meta.resolve.nodes.find((n) => n.id === tcpinfoId);
    const shippedDeps = (node?.deps ?? []).filter(isShipped).map((d) => byId.get(d.pkg).name);
    for (const name of shippedDeps) if (name !== "libc") problems.push(`npt-tcpinfo depends on ${name}`);
    if (!shippedDeps.includes("libc")) problems.push("npt-tcpinfo does not depend on libc");
  }
  for (const n of meta.resolve.nodes) {
    const name = byId.get(n.id).name;
    if (name === "npt-net") continue;
    for (const d of n.deps) if (byId.get(d.pkg).name === "npt-tcpinfo") problems.push(`${name} depends on npt-tcpinfo`);
  }
  // npt-net stays free of libc (plan §4.2, the advisor's M14): no direct normal or build dependency on it, so its kernel
  // calls go through npt-tcpinfo alone. Reaching libc through tokio or npt-tcpinfo is fine; a dev dependency does not
  // ship (C10).
  const netId = meta.workspace_members.find((id) => byId.get(id).name === "npt-net");
  if (!netId) problems.push("npt-net is not a workspace member");
  const netNode = meta.resolve.nodes.find((n) => n.id === netId);
  for (const d of netNode?.deps ?? []) {
    if (byId.get(d.pkg).name === "libc" && isShipped(d)) problems.push("npt-net depends on libc directly");
  }
  // Nothing depends on npt-lab, in any kind.
  for (const n of meta.resolve.nodes) {
    for (const d of n.deps) {
      if (byId.get(d.pkg).name === "npt-lab") problems.push(`${byId.get(n.id).name} depends on npt-lab`);
    }
  }
  return problems;
}

/**
 * The crates that may relax `unsafe_code`, by folder: ADR 0002:264-265's exceptions ("an exception needs a written
 * invariant, a test and an #[allow] with its reason"), named in the workspace Cargo.toml. npt-client-wasm relaxes it in
 * its manifest (wasm-bindgen's generated code), so its files are not scanned; npt-tcpinfo only through the attributes
 * TCPINFO_UNSAFE_FNS allows.
 */
export const UNSAFE_EXCEPTIONS = { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo" };
/** npt-tcpinfo's only relaxing attributes (u3c-plan.md 1.8 (a)): one on each of these functions, in this file. */
export const TCPINFO_UNSAFE_FILE = "crates/tcpinfo/src/linux.rs";
export const TCPINFO_UNSAFE_FNS = ["srtt_us", "unacked_and_unsent"];

/** An attribute that relaxes unsafe_code: allow, expect or warn, alone or in a list, possibly inside cfg_attr. */
const RELAX_UNSAFE = /\b(?:allow|expect|warn)\s*\(\s*(?:[\w:]+\s*,\s*)*unsafe_code\b/g;
/** The one form npt-tcpinfo may use: `allow(unsafe_code, reason = "...")` and nothing else in the attribute. */
const PLACED_ALLOW = /^\s*allow\s*\(\s*unsafe_code\s*,\s*reason\s*=\s*"(?:[^"\\]|\\[\s\S])*"\s*,?\s*\)\s*$/;

/**
 * Rust source with its comments blanked (`code`) and with its comments and literals blanked (`mask`), both as long as
 * the source, newlines kept, so offsets and line numbers carry over. Line comments, doc comments and nested block
 * comments are comments; strings, raw strings, byte and C strings and char literals are literals, kept whole in `code`,
 * so a `//` inside one is not a comment. A lifetime's quote is not a literal.
 */
export function lexRust(src) {
  const blank = (s) => s.replace(/[^\n]/g, " ");
  const charLit = /'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]{1,6}\}|[^\n]))'/y;
  const rawOpen = /[bc]?r(#*)"/y;
  let code = "";
  let mask = "";
  let i = 0;
  const keep = (j, isLiteral) => {
    const piece = src.slice(i, j);
    code += isLiteral ? piece : blank(piece);
    mask += blank(piece);
    i = j;
  };
  while (i < src.length) {
    const c = src[i];
    const prevIdent = i > 0 && /[\w]/.test(src[i - 1]);
    rawOpen.lastIndex = i;
    charLit.lastIndex = i;
    let m;
    if (c === "/" && src[i + 1] === "/") {
      const end = src.indexOf("\n", i);
      keep(end < 0 ? src.length : end, false);
    } else if (c === "/" && src[i + 1] === "*") {
      let depth = 0;
      let j = i;
      do {
        if (src.startsWith("/*", j)) (depth++, (j += 2));
        else if (src.startsWith("*/", j)) (depth--, (j += 2));
        else j++;
      } while (depth > 0 && j < src.length);
      keep(j, false);
    } else if (!prevIdent && (m = rawOpen.exec(src))) {
      const close = `"${m[1]}`;
      const end = src.indexOf(close, rawOpen.lastIndex);
      keep(end < 0 ? src.length : end + close.length, true);
    } else if (c === '"') {
      let j = i + 1;
      while (j < src.length && src[j] !== '"') j += src[j] === "\\" ? 2 : 1;
      keep(Math.min(j + 1, src.length), true);
    } else if (c === "'" && charLit.exec(src)) {
      keep(charLit.lastIndex, true);
    } else {
      code += c;
      mask += c;
      i++;
    }
  }
  return { code, mask };
}

/** Every attribute in a mask (lexRust's): its span, whether it is inner (`#!`), and its body's span (inside `[...]`). */
function rustAttributes(mask) {
  const attrs = [];
  const open = /#\s*(!)?\s*\[/g;
  for (let m = open.exec(mask); m; m = open.exec(mask)) {
    let depth = 1;
    let j = open.lastIndex;
    while (j < mask.length && depth > 0) {
      if (mask[j] === "[") depth++;
      else if (mask[j] === "]") depth--;
      j++;
    }
    attrs.push({ start: m.index, end: j, inner: Boolean(m[1]), bodyStart: open.lastIndex, bodyEnd: j - 1 });
  }
  return attrs;
}

/**
 * The item an outer attribute ending at `at` sits on, other outer attributes skipped: `item` is its first line (for the
 * message), `fn` its name when it is a `pub fn`, else null.
 */
function itemAfter(mask, at, attrs) {
  let j = at;
  for (;;) {
    while (j < mask.length && /\s/.test(mask[j])) j++;
    const next = attrs.find((a) => a.start === j && !a.inner);
    if (!next) break;
    j = next.end;
  }
  const rest = mask.slice(j);
  const item = rest.split("\n", 1)[0].trim() || "(nothing)";
  return { item, fn: rest.match(/^pub\s+fn\s+([A-Za-z_]\w*)\s*[<(]/)?.[1] ?? null };
}

const lineAt = (text, offset) => text.slice(0, offset).split("\n").length;

/**
 * Every `#[path = "..."]` (cfg_attr included) and `include!("...")` in a crate's .rs file that does not name a .rs file
 * inside that crate's folder, as a sentence. Either kind would otherwise compile code this scan does not read under its
 * rules: another crate's file (npt-tcpinfo's linux.rs under npt-room's name), or a file that is not .rs. The path is
 * resolved from the file's own folder (rustc resolves a #[path] inside an inline module from a deeper folder, so this
 * reading flags at least what leaves the crate); an absolute or computed path is a problem too.
 */
function pathProblems(f, crate, code, mask, attrs) {
  const problems = [];
  const inCrateRs = (target) => {
    if (!target.endsWith(".rs") || /^([/\\]|[A-Za-z]:)/.test(target)) return false;
    const parts = f.slice(0, f.lastIndexOf("/")).split("/");
    const floor = crate.split("/").filter(Boolean).length;
    for (const p of target.split(/[/\\]/)) {
      if (p === "..") {
        if (parts.length <= floor) return false;
        parts.pop();
      } else if (p !== "." && p !== "") parts.push(p);
    }
    return true;
  };
  const where = `a .rs file inside ${crate}`;
  for (const a of attrs) {
    for (const m of code.slice(a.bodyStart, a.bodyEnd).matchAll(/\bpath\s*=\s*"((?:[^"\\]|\\.)*)"/g)) {
      if (!inCrateRs(m[1])) problems.push(`${f}:${lineAt(code, a.start)} has #[path = "${m[1]}"], not ${where}`);
    }
  }
  // The mask has the literal blanked, so the argument is read from `code`, after the opening bracket.
  for (const m of mask.matchAll(/\binclude\s*!\s*[([{]/g)) {
    const arg = code.slice(m.index + m[0].length).match(/^\s*"((?:[^"\\]|\\.)*)"\s*[)\]}]/);
    if (!arg) problems.push(`${f}:${lineAt(code, m.index)} has include! with a computed path, not ${where}`);
    else if (!inCrateRs(arg[1])) problems.push(`${f}:${lineAt(code, m.index)} has include!("${arg[1]}"), not ${where}`);
  }
  return problems;
}

/**
 * npt-tcpinfo's placement rule (u3c-plan.md 1.8 (a)): every relaxing of unsafe_code in the crate is an outer
 * `#[allow(unsafe_code, reason = "...")]` in TCPINFO_UNSAFE_FILE sitting directly on `pub fn srtt_us` or
 * `pub fn unacked_and_unsent` (other outer attributes may sit between), one on each. Anything else is a problem with its
 * line: an inner attribute, cfg_attr, expect or warn, an allow on a module, a helper or any other item, a mention in a
 * string or a macro's text, a second allow on a function or none. `sites` counts the placed allows per function.
 */
function tcpinfoProblems(f, code, mask, attrs, sites) {
  const problems = [];
  let inAttributes = 0;
  for (const a of attrs) {
    const body = code.slice(a.bodyStart, a.bodyEnd);
    const n = body.match(RELAX_UNSAFE)?.length ?? 0;
    if (n === 0) continue;
    inAttributes += n;
    const { item, fn } = a.inner ? { item: "its whole module or crate (#!)", fn: null } : itemAfter(mask, a.end, attrs);
    const placed = f === TCPINFO_UNSAFE_FILE && PLACED_ALLOW.test(body) && TCPINFO_UNSAFE_FNS.includes(fn);
    if (placed) sites.set(fn, (sites.get(fn) ?? 0) + 1);
    else {
      const on = a.inner ? item : `\`${item}\``;
      problems.push(
        `${f}:${lineAt(code, a.start)} relaxes unsafe_code on ${on}; npt-tcpinfo may only put #[allow(unsafe_code, ` +
          `reason = "...")] directly on ${TCPINFO_UNSAFE_FNS.join(" and ")} in ${TCPINFO_UNSAFE_FILE}`,
      );
    }
  }
  const all = code.match(RELAX_UNSAFE)?.length ?? 0;
  if (all > inAttributes) problems.push(`${f} names an unsafe_code relaxing outside an attribute (a string or a macro's text)`);
  return problems;
}

/** Whether a crate manifest (comments stripped, one entry per line) has `[lints]` with `workspace = true`. */
function inheritsWorkspaceLints(lines) {
  let table = null;
  for (const raw of lines) {
    const l = raw.trim();
    const header = l.match(/^\[([^\]]+)\]$/);
    if (header) table = header[1].trim();
    else if (table === "lints" && /^workspace\s*=\s*true$/.test(l)) return true;
  }
  return false;
}

/**
 * Every place under crates/ that relaxes unsafe_code outside the two exceptions, as a sentence; empty when none does
 * (u3c-plan.md 1.8 (a) and (d)). `files` are paths relative to the spike's folder (crates/<crate>/...): its .rs files
 * and crate manifests. What is caught:
 * - outside the two exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included;
 * - a manifest other than npt-client-wasm's that sets unsafe_code or does not inherit the workspace lints;
 * - in npt-tcpinfo, any relaxing but the two placed allows (tcpinfoProblems), and either allow missing;
 * - in every crate, a #[path] or include! that names anything but a .rs file inside the crate's folder (pathProblems),
 *   the way another crate could compile npt-tcpinfo's file under its own name.
 * npt-client-wasm's .rs files are not scanned: its manifest relaxes the lint for the whole crate.
 */
export function unsafeProblems(files, read) {
  const problems = [];
  const sites = new Map();
  for (const f of files) {
    const crate = f.match(/^crates\/[^/]+\//)?.[0];
    if (!crate) continue;
    const text = read(f);
    if (f.endsWith(".rs")) {
      const { code, mask } = lexRust(text);
      const attrs = rustAttributes(mask);
      if (!(crate in UNSAFE_EXCEPTIONS) && text.match(RELAX_UNSAFE)) {
        problems.push(`${f} relaxes unsafe_code (only ${Object.values(UNSAFE_EXCEPTIONS).join(" and ")} may)`);
      }
      if (crate === "crates/tcpinfo/") problems.push(...tcpinfoProblems(f, code, mask, attrs, sites));
      problems.push(...pathProblems(f, crate, code, mask, attrs));
    } else if (f === `${crate}Cargo.toml` && crate !== "crates/client-wasm/") {
      const lines = text.split("\n").map((l) => l.replace(/#.*$/, ""));
      if (lines.some((l) => /\bunsafe_code\b/.test(l))) problems.push(`${f} sets unsafe_code`);
      if (!inheritsWorkspaceLints(lines)) problems.push(`${f} does not inherit the workspace lints ([lints] workspace = true)`);
    }
  }
  for (const fn of TCPINFO_UNSAFE_FNS) {
    const n = sites.get(fn) ?? 0;
    if (n !== 1) problems.push(`npt-tcpinfo's pub fn ${fn} has ${n} #[allow(unsafe_code, reason = ...)] attributes, not 1`);
  }
  return problems;
}

/** The spike's crate files the unsafe_code scan reads: every .rs file and crate manifest under crates/. */
function crateFiles() {
  return readdirSync(join(HERE, "crates"), { recursive: true })
    .map((f) => `crates/${String(f).replaceAll("\\", "/")}`)
    .filter((f) => !f.split("/").some((part) => part === "target" || part.startsWith(".")))
    .filter((f) => f.endsWith(".rs") || f.endsWith("/Cargo.toml"));
}

/**
 * Files outside spikes/ that reference it (plan §4.2: nothing outside spikes/ may, until the foundation's G5 and G8
 * exist). Prose may name the folder: docs/, .checkpoints/ and Markdown files are not read.
 */
export function referenceProblems(files, read) {
  const problems = [];
  for (const f of files) {
    if (f.startsWith("spikes/") || f.startsWith("docs/") || f.startsWith(".checkpoints/") || f.endsWith(".md")) continue;
    let text;
    try {
      text = read(f);
    } catch {
      continue;
    }
    if (text.includes("spikes/")) problems.push(`${f} references spikes/`);
  }
  return problems;
}

/** The marks this change-set judges (plan §6.2): every PM-0001 row of ADR 0001, PM-0004-67, PM-0006-11 to -27. */
export function expectedMarks(adr0001Text) {
  const ids = [...adr0001Text.matchAll(/^\| (PM-0001-\d+) \|/gm)].map((m) => m[1]);
  ids.push("PM-0004-67");
  for (let n = 11; n <= 27; n++) ids.push(`PM-0006-${n}`);
  return ids;
}

/**
 * marks.json against its drafts: every expected mark once and nothing else, and each mark's row, found by its id in
 * its ADR, unchanged since marks.json took its SHA-256 (of the row's line, without the line end).
 */
export function marksProblems(marks, readFile) {
  const problems = [];
  const files = new Map();
  const lines = (f) => {
    if (!files.has(f)) files.set(f, readFile(f).split("\n"));
    return files.get(f);
  };
  const expected = expectedMarks(readFile("docs/adr/0001-multiplayer-netcode.md"));
  const got = marks.marks.map((m) => m.id);
  for (const id of expected) {
    const n = got.filter((g) => g === id).length;
    if (n !== 1) problems.push(`${id} is in marks.json ${n} times, not once`);
  }
  for (const id of new Set(got)) if (!expected.includes(id)) problems.push(`${id} is in marks.json but not in plan §6.2's marks`);
  for (const m of marks.marks) {
    const at = lines(m.draft.file)
      .map((l, i) => [l.replace(/\r$/, ""), i + 1])
      .filter(([l]) => l.startsWith(`| ${m.id} |`));
    if (at.length !== 1) {
      problems.push(`${m.id}: ${at.length} rows in ${m.draft.file}, not 1`);
      continue;
    }
    const [row, line] = at[0];
    const hash = createHash("sha256").update(row, "utf8").digest("hex");
    if (hash !== m.draft.row_sha256) {
      problems.push(`${m.id}: its row (${m.draft.file}:${line}) changed since marks.json took its hash; re-read it`);
    }
  }
  return problems;
}

function run(cmd, args, cwd) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  // `stdout` alone is kept for the steps that parse it: cargo writes progress lines ("Blocking waiting for file
  // lock", "Downloaded …") to stderr, and they must never reach a JSON parser.
  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status };
}

/** cargo test's counts, summed over every test binary. */
function cargoCounts(out) {
  let passed = 0;
  let failed = 0;
  for (const m of out.matchAll(/test result: \w+\. (\d+) passed; (\d+) failed/g)) {
    passed += Number(m[1]);
    failed += Number(m[2]);
  }
  return `${passed} tests passed, ${failed} failed`;
}

/** node --test's counts. */
function nodeCounts(out) {
  const get = (k) => out.match(new RegExp(`^# ${k} (\\d+)`, "m"))?.[1] ?? "?";
  return `${get("pass")} of ${get("tests")} tests passed`;
}

function summarized(r, summary) {
  return { ...r, out: `${r.out}\n${summary(r.out)}` };
}

const WASM_ARGS = ["--target", "wasm32-unknown-unknown", ...LEAF.flatMap((c) => ["-p", c])];

/** clippy with warnings as errors on the measured build (no features), then on the bites build (every feature). */
function lintBothBuilds(scope) {
  const measured = run("cargo", ["clippy", "--locked", ...scope, "--", "-D", "warnings"], HERE);
  if (!measured.ok) return measured;
  const bites = run("cargo", ["clippy", "--locked", ...scope, "--all-features", "--", "-D", "warnings"], HERE);
  return { ok: bites.ok, out: `${measured.out}${bites.out}\nmeasured build and bites build both linted`, status: bites.status };
}

/** Both builds' tests: the measured build (no features), then the bites build (every feature, the planted defects
 * compiled in: plan §4.7). Each count is reported. */
function testBothBuilds() {
  const measured = run("cargo", ["test", "--locked", "--workspace"], HERE);
  if (!measured.ok) return measured;
  const bites = run("cargo", ["test", "--locked", "--workspace", "--all-features"], HERE);
  const summary = `measured build: ${cargoCounts(measured.out)}; bites build: ${cargoCounts(bites.out)}`;
  return { ok: bites.ok, out: `${measured.out}${bites.out}\n${summary}`, status: bites.status };
}

/**
 * What the deps step checks, as its summary line says it (plan §4.2's dependency and unsafe rules that are read from
 * `cargo metadata` and the crates' files; §4.2's wasm32 builds and clock reads are the clippy steps', its "no
 * reference into spikes/" is the refs step's, and its narrow public surfaces are not checked by this command).
 */
const DEPS_CHECKED =
  "held: no leaf crate reaches a transport or runtime; npt-client has no npt-net; npt-super and npt-host keep off the " +
  "sim, room, proto (and client) crates; npt-judge ships npt-log and serde only; npt-tcpinfo ships libc only and only " +
  "npt-net uses it; npt-net has no direct libc; nothing depends on npt-lab; unsafe_code is relaxed only by " +
  "npt-client-wasm's manifest and the #[allow] on npt-tcpinfo's srtt_us and unacked_and_unsent; every #[path] and " +
  "include! names a .rs file in its own crate";

const STEPS = {
  fmt: () => run("cargo", ["fmt", "--all", "--check"], HERE),
  // Both builds, as the tests run: the measured build (no features) and the bites build (every feature). A feature can
  // compile code out as well as in (npt-lab's probe has `cfg(not(feature = "bites"))` code), so one pass is not enough.
  clippy: () => lintBothBuilds(["--workspace", "--all-targets"]),
  "clippy-wasm32": () => lintBothBuilds(WASM_ARGS),
  test: testBothBuilds,
  deps: () => {
    const r = run("cargo", ["metadata", "--locked", "--format-version", "1", "--all-features"], HERE);
    if (!r.ok) return r;
    const files = crateFiles();
    const problems = [
      ...dependencyProblems(JSON.parse(r.stdout)),
      ...unsafeProblems(files, (f) => readFileSync(join(HERE, f), "utf8")),
    ];
    return { ok: problems.length === 0, out: problems.join("\n") || `${DEPS_CHECKED} (${files.length} crate files read)` };
  },
  refs: () => {
    const r = run("git", ["ls-files", "-z"], ROOT);
    if (!r.ok) return r;
    const files = r.out.split("\0").filter(Boolean);
    const problems = referenceProblems(files, (f) => readFileSync(join(ROOT, f), "utf8"));
    return { ok: problems.length === 0, out: problems.join("\n") || `${files.length} tracked files: none outside spikes/ references it` };
  },
  marks: () => {
    const c = run("cargo", ["run", "-q", "--locked", "-p", "npt-judge", "--", "--marks", "marks/marks.json", "--check-marks"], HERE);
    if (!c.ok) return c;
    const marks = JSON.parse(readFileSync(join(HERE, "marks", "marks.json"), "utf8"));
    // The rows are compared with this branch's own docs/adr/ (PR #41's readings are on main since d8c485a): any later
    // edit of a mark's row makes this step fail until marks.json is re-read against it.
    const problems = marksProblems(marks, (f) => readFileSync(join(ROOT, f), "utf8"));
    const summary = `${lastLines(c.out, 1)}; rows in this branch's docs/adr/: ${problems.length ? "" : "all unchanged"}`;
    return { ok: problems.length === 0, out: problems.length ? `${problems.join("\n")}\n${summary}` : summary };
  },
  self: () => summarized(run(process.execPath, ["--test", join(HERE, "check.test.mjs")], HERE), nodeCounts),
  redact: () => {
    const t = run(process.execPath, ["--test", join(HERE, "ops", "operator", "redact-check.test.mjs")], HERE);
    if (!t.ok) return t;
    const s = run(process.execPath, [join(HERE, "ops", "operator", "redact-check.mjs"), HERE], HERE);
    return { ...s, out: `${s.out}\n${nodeCounts(t.out)}; ${lastLines(s.out, 1)}` };
  },
};

function lastLines(text, n) {
  return text.trimEnd().split("\n").slice(-n).join("\n");
}

export function main(argv) {
  let only = null;
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--only" && i + 1 < argv.length) only = argv[++i].split(",");
    else throw new Error(`unknown argument ${argv[i]}`);
  }
  const names = only ?? Object.keys(STEPS);
  for (const n of names) if (!STEPS[n]) throw new Error(`no step ${n}; the steps: ${Object.keys(STEPS).join(", ")}`);
  let failed = 0;
  for (const n of names) {
    const started = Date.now();
    const r = STEPS[n]();
    const secs = ((Date.now() - started) / 1000).toFixed(1);
    console.log(`${r.ok ? "PASS" : "FAIL"}  ${n.padEnd(13)} ${secs} s  ${lastLines(r.out, 1)}`);
    if (!r.ok) {
      failed++;
      console.log(lastLines(r.out, 40).replace(/^/gm, "      "));
    }
  }
  console.log(failed ? `check: ${failed} step(s) failed` : `check: all ${names.length} steps pass`);
  return failed ? 1 : 0;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = main(process.argv.slice(2));
  } catch (e) {
    console.error(`check: ${e.message}`);
    process.exitCode = 2;
  }
}
