#!/usr/bin/env node
// The spike's test command (plan §12.4): every unit runs it, and its summary goes into the checkpoint.
//   node spikes/netcode-proof/check.mjs [--only <step,...>]
// Steps: fmt, clippy (host, all targets) and clippy-wasm32 (the leaf crates and npt-tcpinfo's non-Linux stub), each on
// the measured build and on the bites build with warnings as errors; doc (rustdoc on every member, no dependency, both
// builds, then the wasm32 crates for wasm32, warnings as errors: a broken intra-doc link fails it); test (cargo test on
// the measured build, then on
// the bites build, its counts and any test that reported a skip); deps (the dependency rules of plan §4.2 that
// `cargo metadata` shows, with every feature and target, and its unsafe_code rule, from the crates' files, the
// manifests and the environment and cargo configs this command's cargo reads: DEPS_CHECKED says what it checks, and
// the comment above it what it does not; with U4s's three greps: the tick constants of u4-build-plan.md §4.3,
// npt-room's float-free sources, §4.11, and each bite id in one crate's sources, §4.14; U4a's two: npt-proto's
// clippy.toml against npt-sim's, §2, and npt-link's Channel alone in npt-proto and npt-client; and U4d2's: npt-sim's
// resolved features in each build that runs it, the build flags with rustc's read-back of them, and npt-hash-wasm's
// rules, the spike's third unsafe exception, §5 U4d2); wasm (the determinism check, plan.md §4.5 and u4-build-plan.md
// §5 U4d2: release builds of npt-lab for x86-64 and npt-hash-wasm for wasm32, measured and bites; the recordings
// manifest regenerated and checked; npt-lab's two native legs and the Node loader's Wasm leg over its recordings,
// compared tick by tick, with the field diff at a first mismatch; PM-0004-21's cross-target vectors; and its two
// bites, hash-map and std-sin, in the bites builds); refs (nothing outside spikes/ references it, §4.2 and §4.9);
// marks (marks.json compiles in the judge, covers exactly the plan's marks, and every row it hashed is unchanged); self
// (this file's own tests); redact (redact-check's tests, then a scan of the spike's folder). It runs every step and
// exits 1 if any failed.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { HASH_WASM_EXPORTS } from "./crates/hash-wasm/loader.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = join(HERE, "..", "..");

/**
 * The leaf crates (plan §4.2): built for wasm32, free of every transport crate and async runtime; npt-hash-wasm, the
 * check-only cdylib over npt-sim, among them (u4-build-plan.md §5 U4d2), so the wasm32 clippy and doc steps take it.
 */
export const LEAF = ["npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-client", "npt-log", "npt-hash-wasm"];
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

/**
 * npt-sim's only shipped dependencies (U4s; plan §4.5): libm, the one source of transcendentals (ADR 0004's float
 * allow-list), and ChaCha8Rng's two crates for `SimRng`, all pinned in the workspace. A new one is a reviewed change
 * to this list.
 */
export const SIM_DEPENDENCIES = ["libm", "rand_chacha", "rand_core"];

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
  // npt-hash-wasm (U4d2; u4-build-plan.md §5 U4d2): npt-sim is its only shipped dependency, and nothing depends on it,
  // in any kind: a check-only cdylib that Node loads, whose exports are the spike's third unsafe exception. A missing
  // npt-hash-wasm is said once, by the LEAF loop above (it is a leaf crate), and so is a missing npt-sim (below).
  const hashWasmId = meta.workspace_members.find((id) => byId.get(id).name === "npt-hash-wasm");
  if (hashWasmId) {
    const node = meta.resolve.nodes.find((n) => n.id === hashWasmId);
    const shippedDeps = (node?.deps ?? []).filter(isShipped).map((d) => byId.get(d.pkg).name);
    for (const name of shippedDeps) if (name !== "npt-sim") problems.push(`npt-hash-wasm depends on ${name}`);
    if (!shippedDeps.includes("npt-sim") && all.has("npt-sim")) problems.push("npt-hash-wasm does not depend on npt-sim");
  }
  for (const n of meta.resolve.nodes) {
    for (const d of n.deps) {
      if (byId.get(d.pkg).name === "npt-hash-wasm") problems.push(`${byId.get(n.id).name} depends on npt-hash-wasm`);
    }
  }
  // npt-sim (U4s; the arch review's F4): it ships with SIM_DEPENDENCIES alone, so the sim stays free of every crate
  // above it and of any dependency no review has read; a dev dependency does not ship (C10). A missing npt-sim fails
  // here too, not only in the LEAF loop above (it is a leaf crate), which says the same sentence: it is said once.
  const simId = meta.workspace_members.find((id) => byId.get(id).name === "npt-sim");
  if (!simId) {
    if (!problems.includes("npt-sim is not a workspace member")) problems.push("npt-sim is not a workspace member");
  } else {
    const simNode = meta.resolve.nodes.find((n) => n.id === simId);
    for (const d of (simNode?.deps ?? []).filter(isShipped)) {
      const name = byId.get(d.pkg).name;
      if (!SIM_DEPENDENCIES.includes(name)) problems.push(`npt-sim depends on ${name}`);
    }
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
  // npt-proto and npt-client name npt-link `npt_link`, never renamed, so linkUseProblems sees every use of it (U4a;
  // u4s-carries.md carry 3).
  for (const p of meta.packages) {
    if (!["npt-proto", "npt-client"].includes(p.name)) continue;
    for (const d of p.dependencies ?? []) {
      if (d.name === "npt-link" && d.rename) problems.push(`${p.name} renames its npt-link dependency to ${d.rename}`);
    }
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
 * invariant, a test and an #[allow] with its reason"), named in the workspace Cargo.toml. The workspace forbids the
 * lint, so these three carry their own [lints] tables in place of `workspace = true` (EXCEPTION_LEVELS):
 * npt-client-wasm allows it for the whole crate (wasm-bindgen's generated code), so its files are not scanned for it;
 * npt-tcpinfo denies it, which its two #[allow]s can relax (forbid could not), only through the attributes
 * TCPINFO_UNSAFE_FNS allows, and has `unsafe` only inside those functions; npt-hash-wasm, the third (U4d2; the
 * advisor's condition 7), denies it too, and relaxes it only by one #[allow] on each of its listed exports
 * (HASH_WASM_EXPORTS, the Node loader's list), whose `#[unsafe(no_mangle)]` is its only `unsafe`.
 */
export const UNSAFE_EXCEPTIONS = {
  "crates/client-wasm/": "npt-client-wasm",
  "crates/tcpinfo/": "npt-tcpinfo",
  "crates/hash-wasm/": "npt-hash-wasm",
};
/** The unsafe_code level each exception's own [lints.rust] gives, its other lints being the workspace's (review C2R-1). */
const EXCEPTION_LEVELS = { "crates/client-wasm/": "allow", "crates/tcpinfo/": "deny", "crates/hash-wasm/": "deny" };
/**
 * Why a deny-level exception (npt-tcpinfo, npt-hash-wasm) may have no build script and may name `include` only as a
 * macro call (the advisor's U4d2 round-1 ruling, FX-6: SEC-U4D2-4, C2R-2's known limit): code a build script writes,
 * brought in by an `include!` the scan cannot see by its name (`use std::include as inc;`), would sit inside a listed
 * function's allow, where rustc compiles unsafe code the scan never reads.
 */
const DENY_EXCEPTION_WHY =
  "code a build script writes, brought in by a renamed include!, would sit inside a listed function's allow, where " +
  "rustc compiles unsafe code the scan never reads (SEC-U4D2-4)";
/** The exceptions' names as a sentence lists them: "a, b and c". */
const exceptionNames = () => {
  const names = Object.values(UNSAFE_EXCEPTIONS);
  return `${names.slice(0, -1).join(", ")} and ${names.at(-1)}`;
};
/** npt-hash-wasm's one file with unsafe code, its seven exports (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.4 (d)). */
export const HASH_WASM_UNSAFE_FILE = "crates/hash-wasm/src/lib.rs";
/**
 * The types an export's parameters and result may have (the advisor's U4D2-Q7 ruling): rustc lowers both to Wasm's
 * i32, so every export has integer signatures only (the third exception's written invariant).
 * Known limit (SEC-U4D2-3; the advisor's U4d2 round-1 ruling, FX-11): the scan reads the type's token, not the type.
 * A module-local `type u32 = f64;` or `use core::primitive::f64 as u32;` passes it, and the built module's export
 * types are not read (Wasm then types the export `(f64) -> (f64)`: r1-sec-rustc.log); review is the wall until the
 * loader reads the type section (Q7 (b), C-U7-2).
 */
const EXPORT_INTEGERS = ["u32", "i32"];
/** The one attribute whose `unsafe` npt-hash-wasm may have: `#[unsafe(no_mangle)]`, sitting on a listed export. */
const NO_MANGLE = /^\s*unsafe\s*\(\s*no_mangle\s*\)\s*$/;
/** npt-tcpinfo's only relaxing attributes (u3c-plan.md 1.8 (a)): one on each of these functions, in this file. */
export const TCPINFO_UNSAFE_FILE = "crates/tcpinfo/src/linux.rs";
export const TCPINFO_UNSAFE_FNS = ["srtt_us", "unacked_and_unsent"];
/**
 * npt-tcpinfo's crate root, whose crate-level `#![deny(...)]` holds every unsafe block to a SAFETY comment and one
 * unsafe operation (u3c-plan.md 1.8 (a)); these two clippy lints, and their group `restriction`, are named nowhere else
 * in the crate (rustc 1.99.1's clippy lets `allow(clippy::restriction)` relax both).
 */
const TCPINFO_ROOT = "crates/tcpinfo/src/lib.rs";
const TCPINFO_CLIPPY_LINTS = ["undocumented_unsafe_blocks", "multiple_unsafe_ops_per_block"];
const TCPINFO_CLIPPY_NAMES = /\b(?:undocumented_unsafe_blocks|multiple_unsafe_ops_per_block|restriction)\b/g;

/**
 * An attribute that relaxes unsafe_code: allow, expect or warn naming it in its list, alone or among other lints, raw
 * identifiers included (`r#allow`, `r#unsafe_code`, `r#dead_code` before it), possibly inside cfg_attr. The list is
 * read up to its first `)`, so the pattern also runs on lexRust's mask, where a comment or a literal inside the list is
 * blanked: rustc 1.99.1 accepts a block comment holding `)` between dead_code and unsafe_code in one list.
 */
const RELAX_UNSAFE = /\b(?:allow|expect|warn)\s*\([^)]*?\bunsafe_code\b/g;
/**
 * The lint's name as a manifest line may give it (u3c review C2b F1): cargo passes a lint key to rustc as written
 * (`--allow=unsafe-code`), and rustc reads `-` as `_`; case is matched loosely too, as the line scans are a second
 * layer behind lintTables, which reads lowercase `_` names only. In Rust source a lint is a path, so `unsafe-code`
 * cannot name it there (rustc 1.99.1 refuses `#![allow(unsafe-code)]`: "expected identifier, found keyword `unsafe`"),
 * and `UNSAFE_CODE` is an unknown lint (a warning, no effect), so the source scans keep their exact name.
 */
const UNSAFE_CODE_NAME = /\bunsafe[-_]code\b/i;
/**
 * The `unsafe` keyword (a raw identifier `r#unsafe` is a name, not the keyword) and `global_asm!`, the one construct
 * the unsafe_code lint denies that edition 2024 lets go without the keyword (`#[no_mangle]`, `#[export_name]` and
 * `#[link_section]` need `#[unsafe(...)]` there; rustc 1.99.1 compiles a macro-built `#[allow($l)]` over
 * `global_asm!("")` with no `unsafe` in sight). Found in code, they are unsafe code whatever relaxed the lint: an
 * attribute this scan cannot read (one built by a macro), rustflags or `--cap-lints`. In every crate but the three
 * exceptions the compiler refuses them first (the workspace's forbid: E0453 on any allow, a macro-built one included);
 * this scan is the second layer there, and inside npt-tcpinfo and npt-hash-wasm the only one.
 */
const UNSAFE_CODE = /(?<!r#)\bunsafe\b|\bglobal_asm\b/g;
/** The one form npt-tcpinfo may use: `allow(unsafe_code, reason = "...")` and nothing else in the attribute. */
const PLACED_ALLOW = /^\s*allow\s*\(\s*unsafe_code\s*,\s*reason\s*=\s*"((?:[^"\\]|\\[\s\S])*)"\s*,?\s*\)\s*$/;
/**
 * Whether an attribute body is PLACED_ALLOW with a reason that says something (ADR 0002:264-265: "an #[allow] with its
 * reason"): a letter or a digit once escapes and line continuations are taken out, so `reason = ""` or `"\n"` fails.
 */
const isPlacedAllow = (body) =>
  /[A-Za-z0-9]/.test((body.match(PLACED_ALLOW)?.[1] ?? "").replace(/\\(?:x[0-9a-fA-F]{2}|u\{[^}]*\}|[\s\S])/g, ""));

/**
 * Rust source with its comments blanked (`code`) and with its comments and literals blanked (`mask`), both as long as
 * the source, newlines kept, so offsets and line numbers carry over. Line comments, doc comments and nested block
 * comments are comments; strings, raw strings, byte and C strings and char literals are literals, kept whole in `code`,
 * so a `//` inside one is not a comment. A lifetime's quote is not a literal. A char literal holds one code point,
 * which may be two UTF-16 units (`'😀'`): read unit by unit, its closing quote would open a literal of its own and
 * blank the code after it.
 * Known limit (review C2R-1): this is not rustc's lexer. A first line `#!...` (a shebang, which rustc drops from any
 * source file) or a `\u{...}` escape with underscores past six characters puts it out of step, so text rustc compiles
 * can read here as a comment or a literal. Outside the three exception crates the workspace's forbid makes rustc refuse
 * unsafe code this misses; inside npt-tcpinfo and npt-hash-wasm (deny, not forbid) nothing but review does.
 */
export function lexRust(src) {
  const blank = (s) => s.replace(/[^\n]/g, " ");
  const charLit = /'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]{1,6}\}|[^\n]))'/uy;
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
 * reading flags at least what leaves the crate); an absolute or computed path is a problem too, and so is a #[path]
 * whose value is not a plain string literal (rustc 1.99.1 accepts `#[path = $p]` inside a macro_rules, with the path
 * given at the call).
 * Known limit (review C2R-2): the literal is read as written, its escapes not decoded (`.\x2e/` is `../` to rustc),
 * and include! is found by its name only (`use std::include as inc;` hides it). Code such a path brings into a crate
 * under the workspace's forbid is still refused by rustc if it holds unsafe code or an allow of the lint. Into
 * npt-tcpinfo or npt-hash-wasm (deny, not forbid), whose listed functions' allows would cover it, unsafeProblems
 * refuses the two routes found (SEC-U4D2-4): a build script, whose output an include! brings in, and `include` named
 * as a path; a literal with escapes there is review's.
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
    // The key is found in the mask (a `path =` inside a string is no key), its value read from the code.
    const body = code.slice(a.bodyStart, a.bodyEnd);
    for (const m of mask.slice(a.bodyStart, a.bodyEnd).matchAll(/\bpath\s*=/g)) {
      const value = body.slice(m.index + m[0].length).match(/^\s*"((?:[^"\\]|\\.)*)"/);
      const at = `${f}:${lineAt(code, a.start)}`;
      if (!value) problems.push(`${at} has a #[path] whose value is not a plain string literal, not ${where}`);
      else if (!inCrateRs(value[1])) problems.push(`${at} has #[path = "${value[1]}"], not ${where}`);
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

/** The spans of the bodies (`{` to its `}`) of the `pub fn`s named `names` in a mask. */
function fnBodies(mask, names) {
  const spans = [];
  for (const m of mask.matchAll(/\bpub\s+fn\s+([A-Za-z_]\w*)\b/g)) {
    if (!names.includes(m[1])) continue;
    const open = mask.indexOf("{", m.index + m[0].length);
    if (open < 0) continue;
    let depth = 0;
    let j = open;
    do {
      if (mask[j] === "{") depth++;
      else if (mask[j] === "}") depth--;
      j++;
    } while (depth > 0 && j < mask.length);
    spans.push([open, j]);
  }
  return spans;
}

/**
 * npt-tcpinfo's rules (u3c-plan.md 1.8 (a)), each broken one reported with its line:
 * - every relaxing of unsafe_code in the crate is an outer `#[allow(unsafe_code, reason = "...")]` in
 *   TCPINFO_UNSAFE_FILE sitting directly on `pub fn srtt_us` or `pub fn unacked_and_unsent` (other outer attributes may
 *   sit between), one on each, its reason not empty. Anything else is a problem: an inner attribute, cfg_attr, expect
 *   or warn, an allow on a module, a helper or any other item, a mention in a string or a macro's text, `unsafe_code`
 *   named in code outside an attribute (a macro can build the attribute from it), a second allow on a function or none;
 * - every `unsafe` (and `global_asm!`) lies inside the body of one of those two functions in TCPINFO_UNSAFE_FILE;
 * - the two clippy lints of TCPINFO_CLIPPY_LINTS, and their group, are named only in TCPINFO_ROOT's crate-level
 *   `#![deny(...)]` (or forbid), which must deny both: `state.rootDeny` records that it does.
 * `sites` counts the placed allows per function.
 */
function tcpinfoProblems(f, code, mask, attrs, sites, state) {
  const problems = [];
  const inAttribute = (i) => attrs.some((a) => i >= a.start && i < a.end);
  let inAttributes = 0;
  for (const a of attrs) {
    if (!mask.slice(a.bodyStart, a.bodyEnd).match(RELAX_UNSAFE)) continue;
    const body = code.slice(a.bodyStart, a.bodyEnd);
    inAttributes += body.match(RELAX_UNSAFE)?.length ?? 0;
    const { item, fn } = a.inner ? { item: "its whole module or crate (#!)", fn: null } : itemAfter(mask, a.end, attrs);
    const placed = f === TCPINFO_UNSAFE_FILE && !a.inner && isPlacedAllow(body) && TCPINFO_UNSAFE_FNS.includes(fn);
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
  for (const m of mask.matchAll(/\bunsafe_code\b/g)) {
    if (!inAttribute(m.index)) {
      problems.push(`${f}:${lineAt(mask, m.index)} names unsafe_code outside an attribute (a macro can build the attribute from it)`);
    }
  }
  const bodies = f === TCPINFO_UNSAFE_FILE ? fnBodies(mask, TCPINFO_UNSAFE_FNS) : [];
  for (const m of mask.matchAll(UNSAFE_CODE)) {
    if (bodies.some(([s, e]) => m.index > s && m.index < e)) continue;
    problems.push(
      `${f}:${lineAt(mask, m.index)} has \`${m[0]}\` outside the bodies of ${TCPINFO_UNSAFE_FNS.join(" and ")} in ` +
        TCPINFO_UNSAFE_FILE,
    );
  }
  // The crate-level deny: an inner attribute outside every brace (so it is the file's own, the crate root's when the
  // file is lib.rs), its body deny(...) or forbid(...) naming both lints, a raw identifier allowed.
  const rootDeny =
    f === TCPINFO_ROOT
      ? attrs.find((a) => {
          if (!a.inner) return false;
          const before = mask.slice(0, a.start);
          if ((before.match(/\{/g)?.length ?? 0) !== (before.match(/\}/g)?.length ?? 0)) return false;
          const body = mask.slice(a.bodyStart, a.bodyEnd);
          return (
            /^\s*(?:deny|forbid)\s*\(/.test(body) &&
            TCPINFO_CLIPPY_LINTS.every((l) => new RegExp(`\\bclippy\\s*::\\s*(?:r#)?${l}\\b`).test(body))
          );
        })
      : undefined;
  if (rootDeny) state.rootDeny = true;
  for (const m of mask.matchAll(TCPINFO_CLIPPY_NAMES)) {
    if (rootDeny && m.index >= rootDeny.start && m.index < rootDeny.end) continue;
    problems.push(
      `${f}:${lineAt(mask, m.index)} names clippy's ${m[0]} outside ${TCPINFO_ROOT}'s crate-level deny (relaxing it ` +
        "would let an unsafe block go without its SAFETY comment or hold more than one unsafe operation)",
    );
  }
  return problems;
}

/**
 * The item an outer attribute ending at `at` sits on, other outer attributes skipped, read for npt-hash-wasm's exports:
 * `item` its first line as written (from `code`), `export` its name when it is `pub extern "C" fn <name>(` with the
 * name on HASH_WASM_EXPORTS, else null. itemAfter reads `pub fn` only, and in the mask the `"C"` is blanked, so the ABI
 * is read from `code`.
 */
function exportAfter(code, mask, at, attrs) {
  let j = at;
  for (;;) {
    while (j < mask.length && /\s/.test(mask[j])) j++;
    const next = attrs.find((a) => a.start === j && !a.inner);
    if (!next) break;
    j = next.end;
  }
  const item = code.slice(j).split("\n", 1)[0].trim() || "(nothing)";
  const name = code.slice(j).match(/^pub\s+extern\s+"C"\s+fn\s+([A-Za-z_]\w*)\s*\(/)?.[1] ?? null;
  return { item, export: name && HASH_WASM_EXPORTS.includes(name) ? name : null };
}

/**
 * npt-hash-wasm's rules (the spike's third unsafe exception: u4-build-plan.md §5 U4d2; u4d2-brief.md 2.4 (d)), each
 * broken one reported with its line, in tcpinfoProblems' pattern:
 * - every relaxing of unsafe_code in the crate is an outer `#[allow(unsafe_code, reason = "...")]` in
 *   HASH_WASM_UNSAFE_FILE sitting directly on a listed export, `pub extern "C" fn <one of HASH_WASM_EXPORTS>` (other
 *   outer attributes may sit between), its reason not empty; anything else is a problem: an inner attribute, cfg_attr,
 *   expect or warn, an allow on a helper or any other item, a mention in a string or a macro's text, `unsafe_code`
 *   named in code outside an attribute; `sites` counts the placed allows per export (one each, checked by the caller);
 * - every `unsafe` in the crate is the `unsafe` of a `#[unsafe(no_mangle)]` on a listed export in that file: any other
 *   (a block, an unsafe fn, impl or trait, an `unsafe extern` block, `#[unsafe(export_name = ...)]`,
 *   `#[unsafe(link_section = ...)]`, a no_mangle on an export not on the list) and `global_asm!` is a problem;
 * - every listed export's parameters, and its result if it has one, are u32 or i32 (EXPORT_INTEGERS; the advisor's
 *   U4D2-Q7 ruling): Wasm gets i32 for both, so the written invariant's "integer signatures only" holds.
 */
function hashWasmProblems(f, code, mask, attrs, sites) {
  const problems = [];
  const lists = `its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`;
  const inAttribute = (i) => attrs.some((a) => i >= a.start && i < a.end);
  let inAttributes = 0;
  for (const a of attrs) {
    if (!mask.slice(a.bodyStart, a.bodyEnd).match(RELAX_UNSAFE)) continue;
    const body = code.slice(a.bodyStart, a.bodyEnd);
    inAttributes += body.match(RELAX_UNSAFE)?.length ?? 0;
    const after = a.inner ? { item: "its whole module or crate (#!)", export: null } : exportAfter(code, mask, a.end, attrs);
    const placed = f === HASH_WASM_UNSAFE_FILE && !a.inner && isPlacedAllow(body) && after.export;
    if (placed) sites.set(after.export, (sites.get(after.export) ?? 0) + 1);
    else {
      const on = a.inner ? after.item : `\`${after.item}\``;
      problems.push(
        `${f}:${lineAt(code, a.start)} relaxes unsafe_code on ${on}; npt-hash-wasm may only put #[allow(unsafe_code, ` +
          `reason = "...")] directly on ${lists}`,
      );
    }
  }
  const all = code.match(RELAX_UNSAFE)?.length ?? 0;
  if (all > inAttributes) problems.push(`${f} names an unsafe_code relaxing outside an attribute (a string or a macro's text)`);
  for (const m of mask.matchAll(/\bunsafe_code\b/g)) {
    if (!inAttribute(m.index)) {
      problems.push(`${f}:${lineAt(mask, m.index)} names unsafe_code outside an attribute (a macro can build the attribute from it)`);
    }
  }
  // The one `unsafe` each listed export may have: its #[unsafe(no_mangle)]'s.
  const allowed = new Set();
  for (const a of attrs) {
    const body = mask.slice(a.bodyStart, a.bodyEnd);
    if (a.inner || f !== HASH_WASM_UNSAFE_FILE || !NO_MANGLE.test(body)) continue;
    if (exportAfter(code, mask, a.end, attrs).export) allowed.add(a.bodyStart + body.indexOf("unsafe"));
  }
  for (const m of mask.matchAll(UNSAFE_CODE)) {
    if (allowed.has(m.index)) continue;
    const a = attrs.find((x) => m.index >= x.start && m.index < x.end);
    const where = a
      ? `in #[${code.slice(a.bodyStart, a.bodyEnd).trim()}] on \`${a.inner ? "its whole module or crate (#!)" : exportAfter(code, mask, a.end, attrs).item}\``
      : "outside an attribute";
    problems.push(
      `${f}:${lineAt(mask, m.index)} has \`${m[0]}\` ${where}; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] ` +
        `of ${lists}`,
    );
  }
  // Each listed export's signature: u32 or i32 in and out.
  if (f === HASH_WASM_UNSAFE_FILE) {
    for (const m of code.matchAll(/\bpub\s+extern\s+"C"\s+fn\s+([A-Za-z_]\w*)\s*\(/g)) {
      if (!HASH_WASM_EXPORTS.includes(m[1]) || mask.slice(m.index, m.index + 3) !== "pub") continue;
      const open = m.index + m[0].length - 1;
      let depth = 0;
      let close = open;
      do {
        if (mask[close] === "(") depth++;
        else if (mask[close] === ")") depth--;
        close++;
      } while (depth > 0 && close < mask.length);
      const params = mask.slice(open + 1, close - 1).split(",").map((p) => p.trim()).filter(Boolean);
      const types = params.map((p) => p.slice(p.indexOf(":") + 1).trim());
      const result = mask.slice(close).match(/^\s*(?:->\s*([^{]*?))?\s*(?:where\b[^{]*)?\{/)?.[1]?.trim();
      const bad = [...types.map((t, k) => (EXPORT_INTEGERS.includes(t) ? null : `parameter ${params[k]}`)), ...(result === undefined || EXPORT_INTEGERS.includes(result) ? [] : [`result ${result}`])].filter(Boolean);
      if (bad.length || !params.every((p) => p.includes(":"))) {
        problems.push(
          `${f}:${lineAt(code, m.index)} exports ${m[1]} with ${bad.join(" and ") || "a parameter it cannot read"}: npt-hash-wasm's ` +
            "exports take and give u32 or i32 only (the advisor's U4D2-Q7 ruling)",
        );
      }
    }
  }
  return problems;
}

/**
 * Whether a crate manifest has `[lints]` with `workspace = true`, read from tomlScan's mask, as lintTables reads, so a
 * header or a key inside a string or a comment is none (review C2c N2: the two lines moved into a multi-line string,
 * basic or literal, had read as inherited while cargo gave rustc no lint flag at all). Any other spelling (quoted,
 * dotted, inline) reads as not inherited, which fails closed.
 */
function inheritsWorkspaceLints(text) {
  let table = null;
  for (const raw of tomlScan(text).mask.split("\n")) {
    const l = raw.trim();
    const header = l.match(/^\[([^\]]+)\]$/);
    if (header) table = header[1].trim();
    else if (table === "lints" && /^workspace\s*=\s*true$/.test(l)) return true;
  }
  return false;
}

/**
 * Every place under crates/ that breaks the unsafe_code rule, as a sentence; empty when none does (u3c-plan.md 1.8 (a)
 * and (d)). `files` are paths relative to the spike's folder (crates/<crate>/...): its .rs files and crate manifests.
 * What is caught, as lexRust reads the code (its known limit is there):
 * - outside the three exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included
 *   (and a comment or literal inside the attribute's list), and any `unsafe` or `global_asm!` in code;
 * - a manifest other than the three exceptions' that names unsafe_code outside its comments (UNSAFE_CODE_NAME:
 *   `unsafe-code` too) or does not inherit the workspace lints, as read outside its strings and comments
 *   (inheritsWorkspaceLints; the exceptions' own tables are exceptionLintProblems');
 * - in npt-tcpinfo, any relaxing but the two placed allows, `unsafe` outside their two functions, the two clippy lints
 *   named outside lib.rs's crate-level deny or that deny missing (tcpinfoProblems), and either allow missing;
 * - in npt-hash-wasm, any relaxing but one placed allow on each listed export, any `unsafe` but a listed export's
 *   `#[unsafe(no_mangle)]`, a listed export whose parameters or result are not u32 or i32 (hashWasmProblems), and a
 *   listed export with no allow or two;
 * - in npt-tcpinfo and npt-hash-wasm (EXCEPTION_LEVELS' deny), a build.rs at the crate's root, a `build` key in its
 *   manifest, and `include` named other than as an include!, include_str! or include_bytes! call (the advisor's U4d2
 *   round-1 ruling, FX-6: SEC-U4D2-4); layoutProblems refuses a custom-build target there too;
 * - in every crate, a #[path] or include! whose literal names anything but a .rs file inside the crate's folder
 *   (pathProblems, with its known limit), the way another crate could compile npt-tcpinfo's file under its own name.
 * npt-client-wasm's .rs files are not scanned for unsafe code: its manifest allows the lint for the whole crate.
 * What lies outside these files (the manifests' lint tables, the targets' paths and editions, symbolic links, the
 * environment and cargo configs) is checked by workspaceLintProblems, exceptionLintProblems, manifestKeyProblems,
 * layoutProblems, crateFiles and cargoConfigProblems, all called by depsStep.
 */
export function unsafeProblems(files, read) {
  const problems = [];
  const sites = new Map();
  const hashSites = new Map();
  const state = { rootDeny: false };
  const others = exceptionNames();
  for (const f of files) {
    const crate = f.match(/^crates\/[^/]+\//)?.[0];
    if (!crate) continue;
    const text = read(f);
    if (f.endsWith(".rs")) {
      const { code, mask } = lexRust(text);
      const attrs = rustAttributes(mask);
      if (!(crate in UNSAFE_EXCEPTIONS)) {
        if (text.match(RELAX_UNSAFE) || mask.match(RELAX_UNSAFE)) problems.push(`${f} relaxes unsafe_code (only ${others} may)`);
        const uses = [...mask.matchAll(UNSAFE_CODE)];
        if (uses.length) {
          problems.push(`${f}:${lineAt(mask, uses[0].index)} has unsafe code (${uses.length} \`unsafe\` or \`global_asm\`; only ${others} may)`);
        }
      }
      if (crate === "crates/tcpinfo/") problems.push(...tcpinfoProblems(f, code, mask, attrs, sites, state));
      if (crate === "crates/hash-wasm/") problems.push(...hashWasmProblems(f, code, mask, attrs, hashSites));
      if (EXCEPTION_LEVELS[crate] === "deny") {
        const name = UNSAFE_EXCEPTIONS[crate];
        if (f === `${crate}build.rs`) problems.push(`${f} is a build script in ${name}, which may have none: ${DENY_EXCEPTION_WHY}`);
        // Carry 85 (c): `include` only directly before `!` and an opening bracket, so neither a rename nor a macro
        // given the bare name (`m!(include!)`) can build a call the include! rule does not see.
        for (const m of mask.matchAll(/\binclude\b(?!![([{])/g)) {
          problems.push(
            `${f}:${lineAt(mask, m.index)} names \`include\` other than directly before \`!\` and an opening bracket, as an ` +
              `include!, include_str! or include_bytes! call is written (a renamed include!, as \`use std::include as inc;\`, or ` +
              `one a macro builds, as \`m!(include!)\`, hides the call from the include! rule), which ${name} may not: ` +
              DENY_EXCEPTION_WHY,
          );
        }
      }
      // Carry 85 (c), as advisor-u4r-r1 widened it (FP-6 item 8: SEC-U4R-5): no attribute in any crate holds a
      // metavariable (`$`) anywhere in its text, from `#[` or `#![` to its matching `]` in the mask, so neither a #[path]
      // nor an allow built from a macro's fragment (`#[$p]`, `#[cfg_attr(all(), $a)]`) can bring an unread file under a
      // listed export's allow.
      for (const a of attrs) {
        if (mask.slice(a.bodyStart, a.bodyEnd).includes("$")) {
          problems.push(`${f}:${lineAt(mask, a.start)} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`);
        }
      }
      problems.push(...pathProblems(f, crate, code, mask, attrs));
    } else if (f === `${crate}Cargo.toml` && EXCEPTION_LEVELS[crate] === "deny") {
      if (tomlValues(tomlScan(text), "build").length) {
        problems.push(`${f} has a key \`build\`: a build script in ${UNSAFE_EXCEPTIONS[crate]}, which may have none: ${DENY_EXCEPTION_WHY}`);
      }
    } else if (f === `${crate}Cargo.toml` && !(crate in UNSAFE_EXCEPTIONS)) {
      // The name is looked for outside comments, strings included (a quoted key is the lint to cargo; a string that
      // names it fails closed), in tomlScan's `clean`: a `#` inside a string is no comment there (review C2c N2's
      // reader list: a regex had cut such a line short).
      if (UNSAFE_CODE_NAME.test(tomlScan(text).clean)) problems.push(`${f} sets unsafe_code`);
      if (!inheritsWorkspaceLints(text)) problems.push(`${f} does not inherit the workspace lints ([lints] workspace = true)`);
    }
  }
  for (const fn of TCPINFO_UNSAFE_FNS) {
    const n = sites.get(fn) ?? 0;
    if (n !== 1) problems.push(`npt-tcpinfo's pub fn ${fn} has ${n} #[allow(unsafe_code, reason = ...)] attributes, not 1`);
  }
  if (!state.rootDeny) {
    problems.push(`${TCPINFO_ROOT} does not deny clippy::${TCPINFO_CLIPPY_LINTS.join(" and clippy::")} at the crate root`);
  }
  for (const fn of HASH_WASM_EXPORTS) {
    const n = hashSites.get(fn) ?? 0;
    if (n !== 1) problems.push(`npt-hash-wasm's export ${fn} has ${n} #[allow(unsafe_code, reason = ...)] attributes, not 1`);
  }
  return problems;
}

/**
 * The spike's crate files the unsafe_code scan reads, under `cratesDir` (the spike's crates/): every .rs file and crate
 * manifest, as crates/<crate>/..., in every folder (a `target` or dot folder too: `mod target;` or a #[path] into one
 * compiles it); and every symbolic link there, which the deps step refuses: rustc follows one, so a link can compile
 * another crate's file under this crate's name, and whether a walk follows it depends on Node's version.
 */
export function crateFiles(cratesDir) {
  const files = [];
  const links = [];
  for (const e of readdirSync(cratesDir, { recursive: true, withFileTypes: true })) {
    const f = `crates/${relative(cratesDir, join(e.parentPath, e.name)).split(sep).join("/")}`;
    if (e.isSymbolicLink()) links.push(f);
    else if (e.isFile() && (f.endsWith(".rs") || f.endsWith("/Cargo.toml"))) files.push(f);
  }
  return { files: files.sort(), links: links.sort() };
}

/**
 * The tick constant's one home (plan §4.6, RQ6: "`TICK_HZ` one constant"): npt-sim's sources. Every tick-counted value
 * is a const fn of it (u4-build-plan.md §4.3). TICK_DERIVED are the constants npt-sim derives from it, each written
 * `<name in lower case>_at(TICK_HZ)`, its own function at the rate, which s1 pins at 30 and 60 Hz and ticks.rs's const
 * asserts tie to the constant at any rate.
 */
export const TICK_HZ_HOME = "crates/sim/src/";
export const TICK_DERIVED = ["LOOP_CATCHUP", "FULL_STATE_ACK_TICKS", "ACK_RING_TICKS", "SETTINGS_STEP_TICKS"];
/**
 * Every other name that holds a tick count, by the derived constants' own naming (plan §4.6: "no literal 7 anywhere";
 * the netcode review's F8): a name ending in `_TICKS` or holding `CATCHUP`. NOT_TICK_COUNTS are the names that match
 * and are not one, each with its reason.
 */
export const TICK_NAMED = /_TICKS$|CATCHUP/;
export const NOT_TICK_COUNTS = {
  // plan §4.6 (netcode NC2-F16): commands per client per tick, the redundancy cap of 8, not derived from TICK_HZ;
  // u4-build-plan.md §4.3 sets it from npt_proto::Limits::REDUNDANCY_CAP with a const assert.
  CMD_CATCHUP: "the redundancy cap",
};
/**
 * The crates the name rule does not read, each with its reason: npt-net's L2 harness drives transport tests with its
 * own 30 Hz traffic model (fixes.rs's TICK; HANG_TICKS in its tests), never links npt-sim, and keeps 30 Hz in the 60 Hz
 * run, which changes npt-sim alone (plan §4.6). TICK_HZ and the TICK_DERIVED names are read there too.
 */
export const TICK_NAMED_SKIP = ["crates/net/"];
/**
 * A `const` or `static` item (associated ones too): its name and initializer. The type may hold an array's `[T; N]`;
 * the initializer runs to the first `;`, so a block initializer is cut short and never matches a rule's form.
 */
const CONST_ITEM = /\b(?:const|static)\s+(?:mut\s+)?([A-Za-z_]\w*)\b\s*(?::(?:[^=;[\]]|\[[^\]]*\])*)?=([^;]*);/g;

/**
 * Every tick-counted name set in a form u4-build-plan.md §4.3 refuses, under crates/, as a sentence, read in lexRust's
 * mask (comments and strings blanked; whitespace in an initializer ignored):
 * - TICK_HZ is set from a literal once, in TICK_HZ_HOME; any other TICK_HZ item is a path to it.
 * - A TICK_DERIVED name, in TICK_HZ_HOME, is exactly its own function at the rate, `loop_catchup_at(TICK_HZ)` for
 *   LOOP_CATCHUP (the rust review's R4: `7 * TICK_HZ / 30` is 7 at 30 Hz and 14 at 60, and is refused); anywhere else
 *   it is a path to the same name (`npt_sim::LOOP_CATCHUP`), never derived again (the arch review's F6).
 * - Any other const or static named as a tick count (TICK_NAMED, bar NOT_TICK_COUNTS) names TICK_HZ, calls
 *   `ticks_for_ms` (the TICK_HZ form), or is a path to TICK_HZ or another tick-count name; never a literal (`7`,
 *   `30 / 4`, `SEVEN`, `ticks_for_ms_at(60, 500)`). In TICK_HZ_HOME such a name must join TICK_DERIVED, so s1 pins it.
 * Why not plan §4.6's "no literal 7 anywhere" word for word: a grep cannot tell a tick count from any other 7 (a test's
 * pinned `assert_eq!(LOOP_CATCHUP, 7)`, a sequence number, `0..7`), so a ban on the digit would refuse correct code;
 * the rules above cover every item named as a tick count, and LOOP_CATCHUP has one home.
 * Known limit: a grep, not rustc's name resolution. A tick count under another name (a struct field, a `let`, a
 * literal passed to a function such as `Schedule::new(30, 7)`) is review's and the tests' (s1; U4b's caller test).
 */
export function tickConstantProblems(files, read) {
  const problems = [];
  let home = null;
  const pathTo = (init, names) => new RegExp(`^(?:[A-Za-z_]\\w*::)*(?:${names.join("|")})$`).test(init);
  for (const f of files) {
    if (!f.startsWith("crates/") || !f.endsWith(".rs")) continue;
    const inHome = f.startsWith(TICK_HZ_HOME);
    const nameRule = !TICK_NAMED_SKIP.some((d) => f.startsWith(d));
    const { mask } = lexRust(read(f));
    for (const m of mask.matchAll(CONST_ITEM)) {
      const [, name, raw] = m;
      const derived = TICK_DERIVED.includes(name);
      const named = name !== "TICK_HZ" && !derived && TICK_NAMED.test(name) && !Object.hasOwn(NOT_TICK_COUNTS, name);
      if (name !== "TICK_HZ" && !derived && !(named && nameRule)) continue;
      const init = raw.replace(/\s+/g, "");
      const shown = raw.trim().replace(/\s+/g, " ");
      const at = `${f}:${lineAt(mask, m.index)}`;
      if (name === "TICK_HZ") {
        if (pathTo(init, ["TICK_HZ"])) continue;
        if (inHome && home === null) {
          home = at;
          continue;
        }
        const where = inHome ? `a second time (the first: ${home})` : `outside ${TICK_HZ_HOME}`;
        problems.push(`${at} sets TICK_HZ from \`${shown}\` ${where}: npt_sim::TICK_HZ is the one tick constant (plan §4.6)`);
      } else if (derived && inHome) {
        const own = `${name.toLowerCase()}_at(TICK_HZ)`;
        if (init !== own) {
          problems.push(
            `${at} sets ${name} from \`${shown}\`: in npt-sim it is its own function at the tick rate, \`${own}\`, ` +
              "which s1 pins at 30 and 60 Hz (u4-build-plan.md §4.3)",
          );
        }
      } else if (derived) {
        if (!pathTo(init, [name])) {
          problems.push(
            `${at} sets ${name} from \`${shown}\` outside ${TICK_HZ_HOME}: there it is a path to npt_sim::${name}, ` +
              "never derived again (one home: plan §4.6)",
          );
        }
      } else if (inHome) {
        problems.push(
          `${at} sets ${name}, a tick count by its name, in npt-sim outside TICK_DERIVED: it joins TICK_DERIVED as ` +
            `\`${name.toLowerCase()}_at(TICK_HZ)\`, its function pinned in s1 (u4-build-plan.md §4.3)`,
        );
      } else if (
        !(pathTo(init, ["TICK_HZ", ...TICK_DERIVED, "\\w*_TICKS", "\\w*CATCHUP\\w*"]) || /\bTICK_HZ\b/.test(init) || /\bticks_for_ms\(/.test(init))
      ) {
        problems.push(
          `${at} sets ${name} from \`${shown}\`: a tick-counted value is a const fn of npt_sim::TICK_HZ, never a literal ` +
            "(u4-build-plan.md §4.3)",
        );
      }
    }
  }
  return problems;
}

/** npt-room's sources, which hold no float (u4-build-plan.md §4.11, A3). */
export const FLOAT_FREE = "crates/room/src/";
/**
 * A float in Rust code: the types `f32` and `f64` (as names, paths or literal suffixes) and the float literals (`1.5`,
 * `1.`, `1e3`), but not a range (`0..5`), a tuple field (`t.0`), a method on an integer (`1.max(2)`), a hex literal or
 * a name that holds the digits (`buf32`).
 */
const FLOAT_TOKEN =
  /\bf(?:32|64)\b|(?<![\w.])\d[\d_]*(?:\.\d[\d_]*(?:[eE][+-]?\d[\d_]*)?(?:_*f(?:32|64))?|\.(?![\w.])|[eE][+-]?\d[\d_]*|_*f(?:32|64)\b)/g;

/**
 * Every file of npt-room's sources that holds a float, as a sentence: the room's interest, priority and window work on
 * grid integers, and every float stays in npt-sim (u4-build-plan.md §4.11, A3: "a check.mjs grep refuses `f32` and
 * `f64` in crates/room/src"). Read in lexRust's mask, so comments and strings may name them.
 * Known limit: a grep, not type inference. A float the room receives and passes on without naming its type (a value
 * from an npt-sim function, held in a `let` or a field of an npt-sim type) is not seen; the seam's types are integers
 * so that the room has none to hold (crates/sim/src/seam.rs), and review is the wall.
 */
export function roomFloatProblems(files, read) {
  const problems = [];
  for (const f of files) {
    if (!f.startsWith(FLOAT_FREE) || !f.endsWith(".rs")) continue;
    const { mask } = lexRust(read(f));
    const found = [...mask.matchAll(FLOAT_TOKEN)];
    if (found.length) {
      problems.push(
        `${f}:${lineAt(mask, found[0].index)} has \`${found[0][0]}\` (${found.length} float token(s)): npt-room is ` +
          "float-free (u4-build-plan.md §4.11)",
      );
    }
  }
  return problems;
}

/**
 * A bite id, as a string literal holds it whole (marks.json's ids): `PM-<ADR>-<row>/<name>`, the name with upper case
 * and dots too (`PM-0001-7/B-S`, `PM-0001-35/quinn-proto-0.11.17`; the rust fix-review's G1a): check.test.mjs pins
 * that every marks.json bite id matches.
 */
export const BITE_ID = /^PM-\d{4}-\d+\/[A-Za-z0-9.-]+$/;
/** A string literal that is exactly a bite id, in lexRust's code (its opening quote not an escaped one). */
const BITE_ID_LITERAL = new RegExp(`(?<!\\\\)"(${BITE_ID.source.slice(1, -1)})"`, "g");

/**
 * The spans of a mask's (lexRust's) inline modules under `#[cfg(test)]`: `#[cfg(test)] mod <name> { ... }`, other outer
 * attributes between allowed, from the attribute to the module's closing brace.
 */
function testModules(mask) {
  const attrs = rustAttributes(mask);
  const spans = [];
  for (const a of attrs) {
    if (a.inner || !/^\s*cfg\s*\(\s*test\s*\)\s*$/.test(mask.slice(a.bodyStart, a.bodyEnd))) continue;
    let j = a.end;
    for (;;) {
      while (j < mask.length && /\s/.test(mask[j])) j++;
      const next = attrs.find((b) => b.start === j && !b.inner);
      if (!next) break;
      j = next.end;
    }
    const head = mask.slice(j).match(/^(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+[A-Za-z_]\w*\s*\{/);
    if (!head) continue;
    let depth = 0;
    let k = j + head[0].length - 1;
    do {
      if (mask[k] === "{") depth++;
      else if (mask[k] === "}") depth--;
      k++;
    } while (depth > 0 && k < mask.length);
    spans.push([a.start, k]);
  }
  return spans;
}

/**
 * Every bite id planted in more than one crate, as a sentence naming each crate's first file:line (u4-build-plan.md
 * §4.14: each bite is planted where its code lives, so its id is in that one crate's sources; the rust fix-review's G1:
 * -17's id put back in npt-sim's grid beside npt-client's passed every check). Read in lexRust's code, so a comment
 * names nothing: each string literal that is exactly a bite id (BITE_ID), in crates/<crate>/src/. Tests name the ids
 * they run, and are not read: crates/<crate>/tests/, and an inline `#[cfg(test)] mod` in src/ (npt-lab's probe tests
 * run npt-l1's PM-0001-3 ids).
 * Known limit: a grep, not rustc's. An id built at run time (concat!, format!) is not seen; any other item under
 * #[cfg(test)] in src/ (a function, a constant, a test file a `#[cfg(test)] #[path]` module brings in) is read, which
 * fails closed.
 */
export function biteHomeProblems(files, read) {
  const homes = new Map();
  for (const f of files) {
    const crate = f.match(/^crates\/([^/]+)\/src\//)?.[1];
    if (!crate || !f.endsWith(".rs")) continue;
    const { code, mask } = lexRust(read(f));
    const tests = testModules(mask);
    for (const m of code.matchAll(BITE_ID_LITERAL)) {
      if (mask.slice(m.index, m.index + m[0].length).trim()) continue;
      if (tests.some(([s, e]) => m.index > s && m.index < e)) continue;
      if (!homes.has(m[1])) homes.set(m[1], new Map());
      const sites = homes.get(m[1]);
      if (!sites.has(crate)) sites.set(crate, `${f}:${lineAt(code, m.index)}`);
    }
  }
  const problems = [];
  for (const [id, sites] of homes) {
    if (sites.size < 2) continue;
    problems.push(
      `${id} is planted in ${sites.size} crates, ${[...sites.values()].join(" and ")}: a bite's id lives in the one ` +
        "crate whose code it plants (u4-build-plan.md §4.14)",
    );
  }
  return problems;
}

/**
 * The clippy.toml files that must be one list below their headers (u4-build-plan.md §2, "Drift"; plan §4.5): npt-proto
 * shares npt-sim's determinism bans, since its quantisation and decoding must give the same bits on native and Wasm,
 * and its file says the check script compares them.
 */
export const CLIPPY_TWINS = [["crates/sim/clippy.toml", "crates/proto/clippy.toml"]];

/** A clippy.toml's lines below its header (the leading `#` comment lines and blank lines), with each line's number. */
function belowHeader(text) {
  const lines = text.split("\n");
  if (lines.at(-1) === "") lines.pop();
  const start = lines.findIndex((l) => l.trim() !== "" && !l.trimStart().startsWith("#"));
  return start < 0 ? [] : lines.slice(start).map((line, i) => ({ line, at: start + i + 1 }));
}

/**
 * Every CLIPPY_TWINS pair that differs below its headers, as a sentence naming the first differing line of each
 * (a missing file is a problem too). Below the header every byte counts, comments included: the copy is the list.
 */
export function clippyTwinProblems(read) {
  const problems = [];
  for (const [home, copy] of CLIPPY_TWINS) {
    const texts = [];
    for (const f of [home, copy]) {
      try {
        texts.push(read(f));
      } catch {
        problems.push(`${f} is missing: it is one of the clippy.toml files that must match (${home}, ${copy})`);
      }
    }
    if (texts.length < 2) continue;
    const [a, b] = texts.map(belowHeader);
    for (let i = 0; i < Math.max(a.length, b.length); i++) {
      if (a[i]?.line === b[i]?.line) continue;
      const where = (f, lines) => (lines[i] ? `${f}:${lines[i].at}` : `${f}:${lines.at(-1)?.at ?? 0} (its end)`);
      problems.push(
        `${where(copy, b)} differs from ${where(home, a)} below their headers: ` +
          `${copy} is ${home}'s list, line for line (u4-build-plan.md §2, plan §4.5)`,
      );
      break;
    }
  }
  return problems;
}

/**
 * The sources that use npt-link for `Channel` only (catalogue item 2; u4-build-plan.md §4.1: npt-link is a dependency
 * of npt-proto and npt-client for `Channel`; the arch review's F5, u4s-carries.md carry 3).
 */
export const CHANNEL_ONLY = ["crates/proto/src/", "crates/client/src/"];
/** What may follow the name `npt_link` there: `::Channel` (a path into it, `::Channel::Datagram`, or `as` a name), or a
 * use list holding `Channel` alone. */
const CHANNEL_PATH = /^\s*::\s*(?:Channel\b|\{\s*Channel(?:\s+as\s+[A-Za-z_]\w*)?\s*,?\s*\})/;

/**
 * Every place in CHANNEL_ONLY's sources that names anything from `npt_link` but `Channel`, as a sentence naming its
 * file and line, read in lexRust's mask (comments and literals blanked): `use npt_link::Link;`, a use list with more,
 * a glob, the bare crate (`use npt_link;`, `extern crate npt_link as l;`), any other path.
 * Known limit: a grep, not rustc's. A renamed dependency would hide the name, so dependencyProblems refuses a rename
 * of npt-link in npt-proto's and npt-client's manifests; a macro that builds the path is review's.
 */
export function linkUseProblems(files, read) {
  const problems = [];
  for (const f of files) {
    if (!CHANNEL_ONLY.some((d) => f.startsWith(d)) || !f.endsWith(".rs")) continue;
    const { mask } = lexRust(read(f));
    for (const m of mask.matchAll(/\bnpt_link\b/g)) {
      const rest = mask.slice(m.index + m[0].length);
      if (CHANNEL_PATH.test(rest)) continue;
      const shown = `npt_link${rest.split(/[;\n]/, 1)[0]}`.replace(/\s+/g, " ").trim();
      problems.push(
        `${f}:${lineAt(mask, m.index)} names \`${shown}\` from npt_link: npt-proto and npt-client use npt-link for ` +
          "Channel only (catalogue item 2; u4-build-plan.md §4.1)",
      );
    }
  }
  return problems;
}

/**
 * The dotted key that starts at `from` in a tomlScan (spaces and tabs before it skipped), read no further than `end`:
 * its parts, each bare (letters, digits, `_` and `-`, from the mask) or quoted (a string, its value decoded), and the
 * offset just past it; null when no key starts there. `stringAt` maps each string's start to it.
 */
function tomlKeyAt(scan, stringAt, from, end) {
  const { mask } = scan;
  const parts = [];
  let j = from;
  const skip = () => {
    while (j < end && (mask[j] === " " || mask[j] === "\t") && !stringAt.has(j)) j++;
  };
  for (;;) {
    skip();
    const s = stringAt.get(j);
    if (s) {
      parts.push({ name: s.value, quoted: true });
      j = s.end;
    } else {
      const bare = mask.slice(j, end).match(/^[A-Za-z0-9_-]+/);
      if (!bare) return null;
      parts.push({ name: bare[0], quoted: false });
      j += bare[0].length;
    }
    skip();
    if (mask[j] !== ".") return { parts, end: j };
    j++;
  }
}

/** How many brackets and braces a stretch of tomlScan's mask leaves open (strings and comments are blanked there). */
const openBrackets = (masked) => (masked.match(/[[{]/g)?.length ?? 0) - (masked.match(/[\]}]/g)?.length ?? 0);

/**
 * A manifest's lint tables, read from tomlScan's mask, so a header or a key inside a string or a comment is none (review
 * C2R-3: a copy of the right lines inside a multi-line string no longer counts). Headers and keys are read as cargo's
 * TOML reads them: dotted, spaces around the dots, quoted parts decoded; a value's lines that run on (an array or an
 * inline table) are not read as keys or headers. `tables`: for each `[<prefix>lints.<tool>]` table, a Map of its keys
 * to their values as written, strings decoded and re-quoted and the spaces outside them dropped (`'deny'` reads
 * `"deny"`); `lines`: each `<tool>.<key>`'s line. `problems`: every form in which lints reach cargo that this reader
 * does not read, as sentences its callers report, so it never passes (u3c review C2b F1):
 * - a table header under `<prefix>lints` that is not exactly `[<prefix>lints]` or `[<prefix>lints.<tool>]` with bare
 *   lowercase names: a lint's own sub-table (`[lints.clippy.iter-over-hash-type]` with `level` and `priority`, which
 *   cargo 1.99.1 honours), a quoted part (`[workspace."lints".rust]` is the same table to cargo), an array of tables;
 * - in a tool table, a key that is quoted or dotted (a quoted key can spell unsafe_code in escapes), or a lint name with
 *   anything but lowercase letters, digits and `_`: cargo reads `unsafe-code` as unsafe_code (it passes
 *   `--allow=unsafe-code` and rustc reads `-` as `_`), so a hyphen twin beside the real key relaxed it unseen;
 * - a value there that runs past its line;
 * - a key in a `[<prefix>lints]` table but a crate's `workspace = true`;
 * - lints given under any other header, as a dotted key (`lints.rust.unsafe_code`, `"lints".clippy...`) or an inline
 *   table (`lints = { ... }`, or `workspace = { ... }` at the workspace manifest's root).
 */
function lintTables(text, prefix, file) {
  const scan = tomlScan(text);
  const stringAt = new Map(scan.strings.map((s) => [s.start, s]));
  const base = [...prefix.split(".").filter(Boolean), "lints"];
  const startsWith = (names, head) => head.every((p, k) => names[k] === p);
  const tables = new Map();
  const lines = new Map();
  const problems = [];
  // The current table: its header's names, its name as shown, and which lint table it is: "" for [<prefix>lints], the
  // tool for [<prefix>lints.<tool>], null for any other table, false for a lint header refused (its lines are skipped).
  let names = [];
  let table = "";
  let tool = null;
  let open = 0;
  let at = 0;
  for (const [i, line] of scan.mask.split("\n").entries()) {
    const start = at;
    const end = start + line.length;
    at = end + 1;
    if (open > 0) {
      open += openBrackets(line);
      continue;
    }
    const l = line.trim();
    if (!l) continue;
    const where = `${file}:${i + 1}`;
    if (l.startsWith("[")) {
      // A header (an array element line never starts here: `open` skips a value's lines). One cargo cannot parse fails
      // `cargo metadata`, so an unreadable one is left to it.
      const first = start + line.indexOf("[");
      const double = scan.mask[first + 1] === "[";
      const key = tomlKeyAt(scan, stringAt, first + (double ? 2 : 1), end);
      const close = key && scan.mask.slice(key.end, end).match(double ? /^\]\][ \t\r]*$/ : /^\][ \t\r]*$/);
      if (!close) continue;
      names = key.parts.map((p) => p.name);
      table = text.slice(first, key.end + (double ? 2 : 1));
      tool = null;
      if (!startsWith(names, base)) continue;
      const plain = !double && key.parts.every((p) => !p.quoted && /^[a-z0-9_]+$/.test(p.name));
      if (plain && names.length === base.length) tool = "";
      else if (plain && names.length === base.length + 1) tool = names[base.length];
      else {
        tool = false;
        problems.push(
          `${where} has the lint table ${table}, which this check does not read (only [${base.join(".")}] and ` +
            `[${base.join(".")}.<tool>], with bare names: not a lint's own table, a quoted part or an array of tables)`,
        );
      }
      continue;
    }
    const key = tomlKeyAt(scan, stringAt, start, end);
    const eq = key && scan.mask[key.end] === "=" ? key.end : -1;
    if (eq >= 0) open = Math.max(0, openBrackets(scan.mask.slice(eq + 1, end)));
    if (tool === false) continue;
    if (tool === null) {
      const path = [...names, ...(eq >= 0 ? key.parts.map((p) => p.name) : [])];
      if (eq >= 0 && (startsWith(path, base) || startsWith(base, path))) {
        problems.push(`${where} gives lints as a dotted key or an inline table, which this check does not read`);
      }
      continue;
    }
    const shown = names.join(".");
    if (eq < 0 || key.parts.length !== 1 || key.parts[0].quoted) {
      problems.push(`${where} has a key in [${shown}] that is quoted or dotted, which this check does not read`);
      continue;
    }
    const lint = key.parts[0].name;
    if (open > 0 || scan.strings.some((s) => s.start < end && s.end > end)) {
      problems.push(`${where} has a value in [${shown}] that runs past its line, which this check does not read`);
      continue;
    }
    let value = "";
    for (let j = eq + 1; j < end; ) {
      const s = stringAt.get(j);
      if (s) {
        value += JSON.stringify(s.value);
        j = s.end;
      } else {
        if (!/\s/.test(scan.mask[j])) value += scan.mask[j];
        j++;
      }
    }
    if (tool === "") {
      if (!(prefix === "" && lint === "workspace" && value === "true")) problems.push(`${where} sets ${lint} in [${shown}], which this check does not read`);
      continue;
    }
    if (!/^[a-z0-9_]+$/.test(lint)) {
      problems.push(
        `${where} names the lint ${lint} in [${shown}] in a form this check does not read (cargo reads \`-\` as \`_\`: ` +
          `only lowercase names with \`_\` are read)`,
      );
      continue;
    }
    if (!tables.has(tool)) tables.set(tool, new Map());
    tables.get(tool).set(lint, value);
    lines.set(`${tool}.${lint}`, i + 1);
  }
  return { tables, lines, problems };
}

/**
 * The workspace manifest's unsafe_code lint, which the whole rule rests on: `[workspace.lints.rust] unsafe_code =
 * "forbid"` (or `{ level = "forbid", priority = n }`), the compiler backstop (review C2R-1): every member but the three
 * exceptions inherits it, and under it rustc refuses unsafe code and any allow of the lint (E0453), a macro-built one
 * included. Every line naming unsafe_code outside strings and comments, in any case and with `-` for `_` (cargo reads
 * `unsafe-code` as the same lint), must be that entry; a form lintTables does not read, or another level, place or
 * form, is a problem, so a rewritten or relaxed line fails closed (TOML forbids a second definition of the key or the
 * table).
 */
export function workspaceLintProblems(text) {
  const { tables, lines, problems } = lintTables(text, "workspace.", "Cargo.toml");
  const level = tables.get("rust")?.get("unsafe_code") ?? "";
  const ok = /^(?:"forbid"|\{level="forbid"(?:,priority=-?\d+)?\})$/.test(level);
  for (const [i, line] of tomlScan(text).mask.split("\n").entries()) {
    if (UNSAFE_CODE_NAME.test(line) && !(ok && lines.get("rust.unsafe_code") === i + 1)) {
      problems.push(`Cargo.toml:${i + 1} names unsafe_code other than as "forbid" in [workspace.lints.rust]`);
    }
  }
  if (!ok) problems.push('Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit');
  return problems;
}

/**
 * The three exceptions' own lint tables (review C2R-1; replacing u3c-plan.md 1.8 (a)'s "keep [lints] workspace = true"
 * for npt-tcpinfo, the advisor's ruling of 2026-11-08): each crate's `[lints.<tool>]` tables hold exactly the
 * workspace's `[workspace.lints.<tool>]` keys and values, every tool and key, but unsafe_code, which is
 * EXCEPTION_LEVELS' level: "deny" for npt-tcpinfo (its two #[allow]s need a level they can relax) and for
 * npt-hash-wasm (its seven, U4d2), "allow" for npt-client-wasm. Key for key as cargo reads the manifest: any lint given in a form lintTables does not read (a lint's
 * own sub-table, a quoted, dotted, hyphenated or uppercase name, a dotted key or an inline table elsewhere) is reported
 * with them (review C2b F1: a `[lints.clippy.iter-over-hash-type]` table with level "allow" had relaxed the clippy
 * deny unseen). So leaving the workspace's lints costs them nothing else. `read(path)` reads a file under the spike.
 */
export function exceptionLintProblems(workspaceText, read) {
  const ws = lintTables(workspaceText, "workspace.", "Cargo.toml");
  const problems = [];
  for (const [folder, level] of Object.entries(EXCEPTION_LEVELS)) {
    const f = `${folder}Cargo.toml`;
    let text;
    try {
      text = read(f);
    } catch {
      problems.push(`${f} cannot be read`);
      continue;
    }
    const own = lintTables(text, "", f);
    problems.push(...own.problems);
    for (const tool of new Set([...ws.tables.keys(), ...own.tables.keys()])) {
      const want = ws.tables.get(tool) ?? new Map();
      const got = own.tables.get(tool) ?? new Map();
      for (const key of new Set([...want.keys(), ...got.keys()])) {
        if (tool === "rust" && key === "unsafe_code") continue;
        const lint = `[lints.${tool}] ${key}`;
        if (!got.has(key)) problems.push(`${f} has no ${lint}: the workspace's is ${want.get(key)}`);
        else if (!want.has(key)) problems.push(`${f} sets ${lint} = ${got.get(key)}, which the workspace does not`);
        else if (got.get(key) !== want.get(key)) problems.push(`${f} sets ${lint} = ${got.get(key)}, not the workspace's ${want.get(key)}`);
      }
    }
    const ownLevel = own.tables.get("rust")?.get("unsafe_code");
    if (ownLevel !== `"${level}"`) problems.push(`${f} sets [lints.rust] unsafe_code = ${ownLevel ?? "nothing"}, not "${level}"`);
  }
  return problems;
}

/**
 * Where the crates' code comes from, read from `cargo metadata` (u3c review F-E and S5 (c)): every workspace member's
 * manifest is crates/<folder>/Cargo.toml (the unsafe_code scan reads crates/ only), and every target it builds (lib,
 * bin, example, test, bench, build script) is a .rs file inside its own folder, in edition 2024. A manifest's `path`
 * or `build` key could otherwise compile another crate's file under its own name (npt-tcpinfo's linux.rs, with its
 * allows, as npt-room's lib) or a file the scan never reads; and an older edition lets `#[no_mangle]` go without the
 * `unsafe` keyword the scan looks for (rustc 1.99.1, edition 2021: a macro-built allow over it compiles). No member is
 * a proc-macro crate: one can emit an unsafe block whose keyword it builds from strings, and rustc 1.99.1 compiles that
 * expansion even under `--forbid unsafe_code` (code from another crate's macro is not linted), with no `unsafe` in the
 * calling crate's source. And every package with no registry or git source (a path dependency) is a member, so none of
 * the workspace's own code sits outside the scan.
 */
export function layoutProblems(meta) {
  const problems = [];
  const root = resolve(meta.workspace_root);
  const rel = (p) => relative(root, resolve(p)).split(sep).join("/");
  const byId = new Map(meta.packages.map((p) => [p.id, p]));
  for (const p of meta.packages) {
    if (p.source == null && !meta.workspace_members.includes(p.id)) {
      problems.push(`${p.name} (${rel(p.manifest_path)}) is a path dependency outside the workspace, which the scan does not read`);
    }
  }
  for (const id of meta.workspace_members) {
    const p = byId.get(id);
    const manifest = rel(p.manifest_path);
    if (!/^crates\/[^/]+\/Cargo\.toml$/.test(manifest)) {
      problems.push(`${p.name}'s manifest is ${manifest}, not crates/<folder>/Cargo.toml (the unsafe_code scan reads crates/ only)`);
    }
    const dir = dirname(resolve(p.manifest_path));
    for (const t of p.targets ?? []) {
      const what = `${p.name}'s ${t.kind.join("/")} target ${t.name}`;
      const src = resolve(t.src_path);
      if (!src.startsWith(dir + sep)) problems.push(`${what} compiles ${rel(src)}, outside its folder ${rel(dir)}/`);
      else if (!src.endsWith(".rs")) problems.push(`${what} compiles ${rel(src)}, not a .rs file the scan reads`);
      if (t.edition !== "2024") problems.push(`${what} is edition ${t.edition}, not 2024`);
      if (t.kind.includes("proc-macro")) problems.push(`${what} is a proc-macro crate, whose expansions the lint and the scan cannot see`);
      const denied = Object.keys(EXCEPTION_LEVELS).some((c) => EXCEPTION_LEVELS[c] === "deny" && UNSAFE_EXCEPTIONS[c] === p.name);
      if (denied && t.kind.includes("custom-build")) problems.push(`${what} is a build script in ${p.name}, which may have none: ${DENY_EXCEPTION_WHY}`);
    }
  }
  return problems;
}

/**
 * A TOML text read for this file's checks: `clean` with its comments blanked, `mask` with its strings blanked too (both
 * as long as the text, newlines kept), and `strings`, each one's span and value (basic strings' escapes decoded,
 * multi-line and literal strings included). Enough for cargo's config and manifest files, not a full TOML parser: what
 * it cannot read is reported by its callers, never passed. Every TOML file check.mjs reads goes through here, so this is
 * where one leading byte order mark (U+FEFF), which cargo 1.99.1 accepts and skips, is blanked like a comment (review
 * C2c N4: tomlKeyAt skips spaces and tabs only, so a key on line 1 after it had read as nothing, and a rustdoc lint
 * table given there got in unseen); offsets and lines stay the text's.
 */
function tomlScan(text) {
  const blank = (s) => s.replace(/[^\n]/g, " ");
  const escapes = { b: "\b", t: "\t", n: "\n", f: "\f", r: "\r", e: "\x1b", '"': '"', "\\": "\\" };
  const decode = (raw) =>
    raw.replace(/\\(?:u([0-9a-fA-F]{4})|U([0-9a-fA-F]{8})|x([0-9a-fA-F]{2})|([btnfre"\\])|[ \t]*\r?\n\s*)/g, (_, u, U, x, ch) => {
      if (u || U || x) return String.fromCodePoint(parseInt(u || U || x, 16));
      return ch ? escapes[ch] : "";
    });
  // One leading byte order mark (U+FEFF), blanked (see above).
  let clean = text.charCodeAt(0) === 0xfeff ? " " : "";
  let mask = clean;
  const strings = [];
  let i = clean.length;
  while (i < text.length) {
    const c = text[i];
    if (c === "#") {
      const nl = text.indexOf("\n", i);
      const piece = blank(text.slice(i, nl < 0 ? text.length : nl));
      clean += piece;
      mask += piece;
      i += piece.length;
    } else if (c === '"' || c === "'") {
      const q = text.startsWith(c.repeat(3), i) ? c.repeat(3) : c;
      let j = i + q.length;
      while (j < text.length && !text.startsWith(q, j)) j += c === '"' && text[j] === "\\" ? 2 : 1;
      while (q.length === 3 && text[j + 3] === c) j++; // `""""` ends a multi-line string with a quote inside it
      const end = Math.min(j + q.length, text.length);
      const raw = text.slice(i + q.length, j).replace(q.length === 3 ? /^\r?\n/ : /^$/, "");
      strings.push({ start: i, end, value: c === '"' ? decode(raw) : raw });
      clean += text.slice(i, end);
      mask += blank(text.slice(i, end));
      i = end;
    } else {
      clean += c;
      mask += c;
      i++;
    }
  }
  return { clean, mask, strings };
}

/**
 * The values of every key named `key` in a tomlScan (any table; bare, dotted or quoted): for each, its strings (the one
 * string, or every element of an array of strings), or null when it is anything else, which this check cannot read.
 */
function tomlValues(scan, key) {
  const { mask, strings } = scan;
  const starts = [];
  for (const m of mask.matchAll(new RegExp(`(?:^|[\\s.{,])${key}[ \\t]*=`, "gm"))) starts.push(m.index + m[0].length);
  for (const s of strings) {
    const eq = mask.slice(s.end).match(/^[ \t]*=/);
    if (s.value === key && eq && /(?:^|[\s.{,])$/.test(mask.slice(0, s.start))) starts.push(s.end + eq[0].length);
  }
  return starts.map((at) => {
    let v = at;
    while ((mask[v] === " " || mask[v] === "\t") && !strings.some((s) => s.start === v)) v++;
    const one = strings.find((s) => s.start === v);
    if (one) return [one.value];
    if (mask[v] !== "[") return null;
    let depth = 0;
    let j = v;
    do {
      if (mask[j] === "[") depth++;
      else if (mask[j] === "]") depth--;
      j++;
    } while (depth > 0 && j < mask.length);
    if (!/^[\s,]*$/.test(mask.slice(v + 1, j - 1))) return null;
    return strings.filter((s) => s.start > v && s.end < j).map((s) => s.value);
  });
}

/**
 * The first name of every table header in a tomlScan (`[a.b]` or `[[a.b]]`), read as cargo reads it: quoted parts
 * decoded and spaces around the dots allowed (tomlKeyAt), so `[["include"]]` is `include`, which cargo 1.99.1 loads like
 * `[[include]]` (review C2c N2's reader list: a regex on the mask had read the quoted part as nothing). A line inside a
 * multi-line array that only looks like a header counts too, which fails closed.
 */
function tomlHeaderHeads(scan) {
  const stringAt = new Map(scan.strings.map((s) => [s.start, s]));
  const heads = [];
  let at = 0;
  for (const line of scan.mask.split("\n")) {
    const start = at;
    at += line.length + 1;
    const first = start + line.search(/\S|$/);
    if (scan.mask[first] !== "[") continue;
    const key = tomlKeyAt(scan, stringAt, first + (scan.mask[first + 1] === "[" ? 2 : 1), start + line.length);
    if (key) heads.push(key.parts[0].name);
  }
  return heads;
}

/**
 * A rustc flag that lowers or caps lint levels, or one whose flags this check cannot read: -A, -W, --allow, --warn,
 * --force-warn and --cap-lints (rustc 1.99.1 compiles an unsafe block under `--deny unsafe_code` with
 * `--cap-lints warn` even beside `-F unsafe_code`, and with `--force-warn unsafe_code` even beside `-D warnings`), -Z,
 * and an @argfile.
 */
const LINT_LOWERING = /^(?:-A|-W|--allow|--warn|--force-warn|--cap-lints|-Z|@)/;
/**
 * check.mjs's cargo subcommands that live outside cargo (cargo-clippy, cargo-fmt): a cargo alias of the same name
 * replaces them, and cargo 1.99.1 only warns, so `[alias] clippy = "check"` would lint nothing and pass.
 */
const SHADOWABLE = ["clippy", "fmt"];
/**
 * The environment's ways to replace the compiler cargo runs, or to put a program around every workspace crate's rustc
 * call (review C2R-8): either can run another compiler or drop a lint flag. RUSTC_WRAPPER (CARGO_BUILD_RUSTC_WRAPPER,
 * build.rustc-wrapper) stays allowed: sccache is one, and it wraps every crate's call, the dependencies' too; what a
 * wrapper does is outside this check's reach.
 */
const COMPILER_ENV = ["RUSTC", "CARGO_BUILD_RUSTC", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"];
/** The same, as keys of a cargo config file ([build] rustc and rustc-workspace-wrapper, in any table or form). */
const COMPILER_KEYS = ["rustc", "rustc-workspace-wrapper"];
/**
 * The environment variables found that unlock unstable features on the pinned stable toolchain, refused whatever their
 * value, with what each does: RUSTC_BOOTSTRAP (review C2b F5) for cargo and rustc, and cargo's own test knob, which sets
 * the release channel cargo reads (review C2c N3: with it set to nightly, stable cargo 1.99.1 accepted
 * `cargo-features = ["profile-rustflags"]`, which it refuses without).
 */
const UNSTABLE_ENV = {
  RUSTC_BOOTSTRAP: "unlocks unstable cargo and rustc features (a manifest's profile rustflags)",
  __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS: "sets the release channel cargo reads (nightly unlocks its unstable features: a manifest's cargo-features)",
};

/**
 * Every way the cargo this command runs could relax a lint, or stand in for a step, without a word in the crates (u3c
 * review S1, F-D, C2R-4, C2R-7, C2R-8), as a sentence:
 * - a lint-lowering flag (LINT_LOWERING) in RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS or
 *   CARGO_TARGET_<triple>_RUSTFLAGS (`env`), or in a `rustflags` key of a cargo config file (`configs`: { path, text },
 *   the ones cargoConfigFiles finds); a rustflags value this check cannot read;
 * - an alias that replaces clippy or fmt (CARGO_ALIAS_CLIPPY or _FMT, or such a key in a config file);
 * - another compiler or a workspace wrapper (COMPILER_ENV, COMPILER_KEYS);
 * - a linker set for any target (carry 85 (a)), as the compiler is: an environment variable whose name ends in _LINKER
 *   (CARGO_TARGET_<triple>_LINKER, CARGO_HOST_LINKER) or a `linker` key in any table of a config file: a linker's own
 *   arguments can rewrite the Wasm module's target_features section, the linker's record that the loader reads;
 * - a profile set outside the manifests (carry 85 (b)): an environment variable whose name starts with CARGO_PROFILE_,
 *   or a `profile` key, dotted key or table in a config file (any table or form, which fails closed): a config's
 *   `[profile.release.package.npt-sim] overflow-checks = false` drops npt-sim's `-C overflow-checks=on`, which plan §4.5
 *   asks for in every profile;
 * - a config `include` key or `[[include]]` table, its name quoted or not, which loads a config file cargoConfigFiles
 *   does not read (cargo 1.99.1 honours it);
 * - a target runner (CARGO_TARGET_<triple>_RUNNER, or a `runner` key in a config file): it runs the test binaries in
 *   cargo's place, and the test step judges cargo's exit status (a runner `true` passes with no test run);
 * - RUSTC_BOOTSTRAP, whatever its value (review C2b F5): it unlocks unstable features on the pinned stable toolchain,
 *   and with it cargo 1.99.1 honoured a manifest's `cargo-features = ["profile-rustflags"]` and a profile's
 *   `rustflags = ["--cap-lints", "warn"]`, which turned the workspace's forbid into a warning (those manifest keys are
 *   manifestKeyProblems'); and __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS, cargo's own test knob, whatever its value
 *   (review C2c N3): it sets the channel cargo reads, and set to nightly it let cargo 1.99.1 accept the same
 *   `cargo-features` (UNSTABLE_ENV).
 * Not read: a config's [env] table (it can set CLIPPY_ARGS for clippy-driver, review C2R-4 G, or any other variable for
 * rustc) and what a member's build script prints (`cargo::rustc-env`, C2R-4 H). Both need a committed change that
 * review sees.
 */
export function cargoConfigProblems(env, configs) {
  const problems = [];
  const lowering = (where, flags) => {
    for (const fl of flags) {
      if (LINT_LOWERING.test(fl)) problems.push(`${where} passes ${JSON.stringify(fl)} to rustc: a flag that can relax unsafe_code`);
    }
  };
  for (const [k, v] of Object.entries(env)) {
    if (typeof v !== "string") continue;
    if (/RUSTFLAGS$/.test(k)) lowering(`${k} in the environment`, v.split(k === "CARGO_ENCODED_RUSTFLAGS" ? "\x1f" : /\s+/));
    const alias = k.match(/^CARGO_ALIAS_(\w+)$/)?.[1].toLowerCase();
    if (SHADOWABLE.includes(alias)) problems.push(`${k} in the environment replaces \`cargo ${alias}\` with an alias`);
    if (COMPILER_ENV.includes(k)) problems.push(`${k} in the environment replaces rustc or wraps the workspace's rustc calls (only RUSTC_WRAPPER may wrap them)`);
    if (/_LINKER$/.test(k)) problems.push(`${k} in the environment sets a linker (${JSON.stringify(v)}): its arguments can rewrite the target_features section the Wasm loader reads`);
    if (/^CARGO_PROFILE_/.test(k)) problems.push(`${k} in the environment sets a profile key (${JSON.stringify(v)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
    if (/^CARGO_TARGET_\w+_RUNNER$/.test(k)) problems.push(`${k} in the environment runs the test binaries in cargo's place`);
    if (Object.hasOwn(UNSTABLE_ENV, k)) problems.push(`${k} in the environment ${UNSTABLE_ENV[k]}`);
  }
  for (const { path, text } of configs) {
    const scan = tomlScan(text);
    for (const v of tomlValues(scan, "rustflags")) {
      if (v === null) problems.push(`${path} has a rustflags value this check cannot read`);
      else lowering(`${path}'s rustflags`, v.flatMap((s) => [s, ...s.split(/\s+/)]));
    }
    for (const name of SHADOWABLE) {
      if (tomlValues(scan, name).length) problems.push(`${path} has a key \`${name}\`: an alias that replaces \`cargo ${name}\``);
    }
    for (const name of COMPILER_KEYS) {
      if (tomlValues(scan, name).length) problems.push(`${path} has a key \`${name}\`: it replaces rustc or wraps the workspace's rustc calls`);
    }
    for (const v of tomlValues(scan, "linker")) {
      problems.push(`${path} has a key \`linker\` (${v === null ? "a value this check cannot read" : JSON.stringify(v.join(" "))}): it sets a linker, whose arguments can rewrite the target_features section the Wasm loader reads`);
    }
    // A key, dotted key or table header named `profile`, bare or quoted (as tomlValues reads keys, and `.` too).
    const profile =
      tomlHeaderHeads(scan).includes("profile") ||
      /(?:^|[\s.{,])profile[ \t]*[.=]/m.test(scan.mask) ||
      scan.strings.some((s) => s.value === "profile" && /^[ \t]*[.=]/.test(scan.mask.slice(s.end)) && /(?:^|[\s.{,[])$/.test(scan.mask.slice(0, s.start)));
    if (profile) {
      // Its line and text, as the refusals above name their value (advisor-u4r-r1, FP-6 item 9: SEC-U4R-6): the first
      // place a header, a bare or dotted key, or a quoted key names `profile`.
      const header = scan.mask.match(/^[ \t]*\[\[?[ \t]*profile[ \t]*[.\]]/m);
      const key = scan.mask.match(/(?:^|[\s.{,])profile[ \t]*[.=]/m);
      const quoted = scan.strings.find((s) => s.value === "profile" && /^[ \t]*[.=]/.test(scan.mask.slice(s.end)) && /(?:^|[\s.{,[])$/.test(scan.mask.slice(0, s.start)));
      const found = [header?.index, key ? key.index + key[0].indexOf("profile") : undefined, quoted?.start].filter((i) => i !== undefined);
      const line = lineAt(scan.mask, found.length ? Math.min(...found) : 0);
      const said = text.split("\n")[line - 1].trim();
      problems.push(`${path}:${line} has a profile key or table (${JSON.stringify(said)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
    }
    if (tomlValues(scan, "include").length || tomlHeaderHeads(scan).includes("include")) {
      problems.push(`${path} has a key \`include\`: it loads a config file this check does not read`);
    }
    if (tomlValues(scan, "runner").length) problems.push(`${path} has a key \`runner\`: it runs the test binaries in cargo's place`);
  }
  return problems;
}

/**
 * Every rustflags source cargo may read for a build (https://doc.rust-lang.org/cargo/reference/config.html,
 * build.rustflags: CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, the target tables, build.rustflags), each as { where, flags }
 * with its flags as rustc would get them: the environment's *RUSTFLAGS variables (CARGO_ENCODED_RUSTFLAGS split at
 * \x1f, the others at whitespace; CARGO_TARGET_<triple>_RUSTFLAGS and CARGO_BUILD_RUSTFLAGS among them) and every
 * `rustflags` key of the cargo configs read (a string split at whitespace, an array as it is), in any table. A value
 * this check cannot read is cargoConfigProblems' to report.
 */
export function rustflagSources(env, configs) {
  const sources = [];
  for (const [k, v] of Object.entries(env)) {
    if (typeof v !== "string" || !/RUSTFLAGS$/.test(k)) continue;
    sources.push({ where: `${k} in the environment`, flags: v.split(k === "CARGO_ENCODED_RUSTFLAGS" ? "\x1f" : /\s+/).filter(Boolean) });
  }
  for (const { path, text } of configs) {
    for (const v of tomlValues(tomlScan(text), "rustflags")) {
      if (v === null) continue;
      sources.push({ where: `${path}'s rustflags`, flags: v.length === 1 ? v[0].split(/\s+/).filter(Boolean) : v });
    }
  }
  return sources;
}

/** The target features plan.md §4.5's build flags leave off (baseline x86-64, simd128 and relaxed-simd off; no FMA). */
const REFUSED_FEATURES = ["fma", "simd128", "relaxed-simd"];
/** The CPU name plan.md §4.5 refuses: `target-cpu=native` builds for the machine that compiles. */
const REFUSED_CPUS = ["native"];

/**
 * The refused build flags in the rustflags sources (u4d2-brief.md 2.4 (c) (i); the advisor's condition 7 (a); ADR
 * 0002:256-258; plan.md §4.5): `+fma`, `+simd128` or `+relaxed-simd` in a target-feature list, and
 * `target-cpu=native`, in every form rustc reads a codegen option (`-C opt`, `-Copt`, `--codegen opt`,
 * `--codegen=opt`; the option split across two tokens or not; `_` for `-` in its name), each item of a comma list read;
 * a feature named without a sign is read as enabled, which fails closed. As sentences, naming the source and the flag.
 */
export function buildFlagProblems(sources) {
  const problems = [];
  for (const { where, flags } of sources) {
    const options = [];
    for (let i = 0; i < flags.length; i++) {
      const t = flags[i];
      if (t === "-C" || t === "--codegen") {
        if (i + 1 < flags.length) options.push(flags[++i]);
      } else if (t.startsWith("--codegen=")) options.push(t.slice("--codegen=".length));
      else if (t.startsWith("-C")) options.push(t.slice(2));
    }
    for (const o of options) {
      const eq = o.indexOf("=");
      const name = (eq < 0 ? o : o.slice(0, eq)).replace(/_/g, "-");
      const value = eq < 0 ? "" : o.slice(eq + 1);
      if (name === "target-feature") {
        for (const item of value.split(",").map((s) => s.trim())) {
          const feature = item.replace(/^[+-]/, "");
          if (REFUSED_FEATURES.includes(feature) && !item.startsWith("-")) {
            problems.push(`${where} passes ${JSON.stringify(`-C ${o}`)}: it enables ${feature}, which plan.md §4.5's build flags refuse`);
          }
        }
      } else if (name === "target-cpu" && REFUSED_CPUS.includes(value)) {
        problems.push(`${where} passes ${JSON.stringify(`-C ${o}`)}: plan.md §4.5's build flags refuse target-cpu=native`);
      }
    }
  }
  return problems;
}

/**
 * Every rustflags source that holds a flag, whatever the flag, and every build target set outside the builds' own
 * `--target` (the advisor's U4d2 round-1 ruling, FX-1: RR-M2 with TE-M1, SEC-U4D2-5, SEC-U4D2-6), as sentences naming
 * the source and its value:
 * - each source rustflagSources reads (every environment variable whose name ends in RUSTFLAGS, and every `rustflags`
 *   key of the cargo configs read) with a flag in it. Cargo gives rustc one source and drops the others
 *   (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags; carry 85 (e)): CARGO_ENCODED_RUSTFLAGS if
 *   set, even empty; else RUSTFLAGS if set, even empty; else the matching target tables' flags, if any; else
 *   build.rustflags. So a read-back given all of them could
 *   take a flag's negation from a source cargo drops (`-Ctarget-feature=-simd128,-relaxed-simd` in RUSTFLAGS beside
 *   CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge passed while the module cargo built held both: RR-M2), and a
 *   flag no name rule knows (`--cfg feature="bites"`, `-C overflow-checks=off`, `-C llvm-args=...`) reached rustc unseen
 *   by both (SEC-U4D2-5). The spike's builds use none, so with every source empty the read-back is given exactly what
 *   cargo gives rustc: no flag;
 * - CARGO_BUILD_TARGET in the environment, whatever its value, and a `target` key in a cargo config (build.target; read
 *   in any table or form, as this file reads config keys, which fails closed): either builds npt-lab's native legs for
 *   another target than the x86-64 one the read-back and the features comparison read (SEC-U4D2-6).
 */
export function buildInputProblems(env, configs) {
  const problems = [];
  for (const { where, flags } of rustflagSources(env, configs)) {
    if (!flags.length) continue;
    problems.push(
      `${where} holds ${JSON.stringify(flags.join(" "))}: no rustflags source may hold a flag (cargo uses one source ` +
        "and drops the rest, so the read-back could not be given what rustc gets: the advisor's U4d2 round-1 ruling, FX-1)",
    );
  }
  if (typeof env.CARGO_BUILD_TARGET === "string") {
    problems.push(
      `CARGO_BUILD_TARGET in the environment sets the build target (${JSON.stringify(env.CARGO_BUILD_TARGET)}): the native ` +
        "builds take cargo's default target, the x86-64 one the read-back and the features comparison read",
    );
  }
  for (const { path, text } of configs) {
    if (tomlValues(tomlScan(text), "target").length) {
      problems.push(
        `${path} has a key \`target\` (build.target): it sets the build target, and the native builds take cargo's ` +
          "default target, the x86-64 one the read-back and the features comparison read",
      );
    }
  }
  return problems;
}

/**
 * rustc's read-back of the target features (the advisor's condition 7 (a): it fails the step, never only reports; the
 * advisor's U4D2-Q4 ruling): `rustc --print cfg --target <triple>` run from the spike's folder (rust-toolchain.toml
 * picks the toolchain cargo uses), given every rustflag of every source read. The deps step refuses every source that
 * holds a flag (buildInputProblems: the advisor's U4d2 round-1 ruling, FX-1), so whenever the step can pass the
 * read-back is given no flag, which is exactly what cargo gives rustc; given flags, it still reads them (a CPU name
 * meant for one target empties or cuts the other's list: rustc 1.99.1, the advisor's U4d2 brief check, A15), on a step
 * that already fails. Per target: the features it must not list, and the one a read-back that read anything lists
 * (non-vacuity: a parse that reads nothing never passes).
 */
export const READ_BACK = [
  { target: "x86_64-unknown-linux-gnu", refused: ["fma"], present: "sse2" },
  { target: "wasm32-unknown-unknown", refused: ["simd128", "relaxed-simd"], present: "sign-ext" },
];

/** The read-back through `runner` (from `here`): { problems, line }, the line naming rustc's version and each list. */
export function readBackCheck(sources, runner, here) {
  const flags = sources.flatMap((s) => s.flags);
  const given = flags.length ? `given ${JSON.stringify(flags.join(" "))}` : "given no rustflags";
  const problems = [];
  const lists = [];
  const version = runner("rustc", ["--version"], here);
  const rustc = version.ok ? (version.stdout || version.out).trim().split("\n")[0] : "rustc (its version did not print)";
  if (!version.ok) problems.push(`rustc --version failed in ${here}: the read-back cannot run (${lastLines(version.out, 1)})`);
  for (const { target, refused, present } of READ_BACK) {
    const r = runner("rustc", ["--print", "cfg", "--target", target, ...flags], here);
    if (!r.ok) {
      problems.push(`rustc --print cfg --target ${target} (${given}) failed: ${lastLines(r.out, 1)}`);
      continue;
    }
    const features = [...(r.stdout || r.out).matchAll(/^target_feature="([^"]+)"\r?$/gm)].map((m) => m[1]);
    lists.push(`${target} ${features.join(", ") || "none"}`);
    for (const f of refused) {
      if (features.includes(f)) problems.push(`rustc's read-back for ${target} (${given}) lists target_feature="${f}", which plan.md §4.5's build flags refuse`);
    }
    if (!features.includes(present)) {
      problems.push(
        `rustc's read-back for ${target} (${given}) lists no target_feature="${present}": it read nothing a build for it has ` +
          "(a CPU name meant for the other target, or a parse that reads nothing), so it proves nothing",
      );
    }
  }
  return { problems, line: `${rustc}'s read-back, ${given}: ${lists.join("; ")}` };
}

/**
 * The builds whose npt-sim must resolve one feature set, like for like (ADR 0004:1184, PM-0004-74; u4-build-plan.md
 * §5 U4d2; the advisor's U4D2-Q6 ruling): the server's, npt-lab's (the native legs of the determinism check) and
 * npt-hash-wasm's for wasm32 (its Wasm leg); measured with measured, bites (`--features bites`) with bites.
 */
export const SIM_BUILDS = [
  { pkg: "npt-server", target: "x86_64-unknown-linux-gnu" },
  { pkg: "npt-lab", target: "x86_64-unknown-linux-gnu" },
  { pkg: "npt-hash-wasm", target: "wasm32-unknown-unknown" },
];

/** The `cargo tree` arguments for one build: its normal edges, every package once per path, each with its features. */
export const treeArgs = ({ pkg, target }, bites) => [
  "tree",
  "--locked",
  "-e",
  "normal",
  "--target",
  target,
  "-p",
  pkg,
  ...(bites ? ["--features", "bites"] : []),
  "--no-dedupe",
  "--prefix",
  "depth",
  "--format",
  "{p} [{f}]",
];

/**
 * npt-sim's subtree in one `cargo tree` output (treeArgs's form: `<depth><name> v<version>[ (<path>)] [<features>]`):
 * a Map from `<name> v<version>` to its features, sorted and joined, npt-sim's own included; null when npt-sim is not
 * in the tree. The first npt-sim met is walked, down to the next line at its depth or above (--no-dedupe repeats every
 * subtree in full, so none is cut short by a `(*)`); one build resolves one feature set per package, so every npt-sim
 * in a tree is the same.
 */
export function simSubtree(text) {
  const rows = text
    .split("\n")
    .map((l) => l.trimEnd().match(/^(\d+)(\S+) v(\S+)(?: \(.*\))? \[([^\]]*)\]$/))
    .filter(Boolean);
  const at = rows.findIndex((m) => m[2] === "npt-sim");
  if (at < 0) return null;
  const depth = Number(rows[at][1]);
  const out = new Map();
  for (let i = at; i < rows.length && (i === at || Number(rows[i][1]) > depth); i++) {
    const [, , name, version, features] = rows[i];
    out.set(`${name} v${version}`, features.split(",").map((s) => s.trim()).filter(Boolean).sort().join(", "));
  }
  return out;
}

/**
 * npt-sim's resolved features compared across SIM_BUILDS (u4d2-brief.md 2.4 (b)), through `runner` (`cargo tree` from
 * `here`): { problems, line }. Per feature set (measured, then bites), each pair of builds' npt-sim subtrees,
 * package by package (npt-sim's own features and each package below it, transitive ones included): a package missing
 * from one, or with other features, fails, naming it, the two builds and the features. Not vacuous: each
 * subtree must hold npt-sim and each normal dependency npt-sim's own manifest names (`meta`'s npt-sim package, the
 * cargo metadata the deps step takes). A measured tree is never compared with a bites tree.
 */
export function simFeatureCheck(meta, runner, here) {
  const problems = [];
  const sim = meta.packages.find((p) => p.name === "npt-sim" && meta.workspace_members.includes(p.id));
  const direct = (sim?.dependencies ?? []).filter((d) => d.kind === null || d.kind === undefined).map((d) => d.name);
  const shown = [];
  for (const bites of [false, true]) {
    const set = bites ? "bites" : "measured";
    const name = (b) => `${b.pkg} (${b.target}, ${set})`;
    const trees = [];
    for (const b of SIM_BUILDS) {
      const r = runner("cargo", treeArgs(b, bites), here);
      if (!r.ok) {
        problems.push(`cargo tree for the ${name(b)} build failed: ${lastLines(r.out, 1)}`);
        continue;
      }
      const tree = simSubtree(r.stdout || r.out);
      if (!tree) {
        problems.push(`the ${name(b)} build's tree holds no npt-sim subtree, so its features prove nothing`);
        continue;
      }
      const names = [...tree.keys()].map((k) => k.split(" ")[0]);
      for (const d of direct) {
        if (!names.includes(d)) problems.push(`the ${name(b)} build's npt-sim subtree lacks ${d}, which crates/sim/Cargo.toml names`);
      }
      trees.push({ b, tree });
    }
    // Every pair of builds in the set, so a difference names each build it sets apart.
    for (let i = 0; i < trees.length; i++) {
      for (let j = i + 1; j < trees.length; j++) {
        const [p, q] = [trees[i], trees[j]];
        for (const pkg of new Set([...p.tree.keys(), ...q.tree.keys()])) {
          const [x, y] = [p.tree.get(pkg), q.tree.get(pkg)];
          if (x === y) continue;
          const show = (v) => (v === undefined ? "is not in its npt-sim subtree" : `[${v}]`);
          problems.push(`npt-sim's ${pkg} resolves ${show(x)} in the ${name(p.b)} build and ${show(y)} in the ${name(q.b)} build`);
        }
      }
    }
    const first = trees[0];
    if (first) shown.push(`${set}: ${[...first.tree].map(([k, v]) => `${k.split(" ")[0]} [${v}]`).join(", ")}`);
  }
  return { problems, line: `npt-sim's features, one set in ${SIM_BUILDS.map((b) => b.pkg).join(", ")}: ${shown.join("; ")}` };
}

/** The manifest keys manifestKeyProblems refuses, with why. */
const MANIFEST_KEYS = {
  "cargo-features": "it unlocks unstable manifest keys (a profile's rustflags among them)",
  rustflags: "a manifest's rustflags reach rustc (`--cap-lints warn` there turns the workspace's forbid into a warning)",
};

/**
 * The manifest keys that reach rustc's flags (u3c review C2b F5), refused in the workspace manifest and in every crate
 * manifest under crates/ (`manifests`: { path, text }), whatever their value and in any table or form (bare, dotted,
 * quoted, inline): `cargo-features`, which cargo 1.99.1 accepts only on a nightly toolchain, under RUSTC_BOOTSTRAP or
 * under cargo's channel override (cargoConfigProblems refuses both), and any `rustflags` key (a profile's, under
 * `cargo-features = ["profile-rustflags"]`): with both, `rustflags = ["--cap-lints", "warn"]` compiled a planted allow
 * and unsafe block under the workspace's forbid with no warning while this step passed. Neither key is in any manifest
 * of the spike.
 */
export function manifestKeyProblems(manifests) {
  const problems = [];
  for (const { path, text } of manifests) {
    const scan = tomlScan(text);
    for (const [key, why] of Object.entries(MANIFEST_KEYS)) {
      if (tomlValues(scan, key).length) problems.push(`${path} has a key \`${key}\`: ${why}`);
    }
  }
  return problems;
}

/**
 * The folders cargo reads its config files from when it runs in `start`: .cargo in `start` and in every folder above
 * it, then CARGO_HOME (`env`'s) as cargo reads it (carry 85 (d)): an empty one as unset (~/.cargo), a relative one
 * against `start`, the folder cargo runs in (cargo's home crate: CARGO_HOME non-empty, joined to the working folder).
 */
export function cargoConfigDirs(start, env) {
  const dirs = [];
  for (let d = resolve(start); ; d = dirname(d)) {
    dirs.push(join(d, ".cargo"));
    if (dirname(d) === d) break;
  }
  dirs.push(env.CARGO_HOME ? resolve(start, env.CARGO_HOME) : join(homedir(), ".cargo"));
  return dirs;
}

/**
 * The cargo config files cargo reads when it runs in `start`: .cargo/config.toml and .cargo/config in each of
 * cargoConfigDirs; each that exists, as { path, text }.
 */
function cargoConfigFiles(start, env) {
  const dirs = cargoConfigDirs(start, env);
  const found = [];
  for (const dir of new Set(dirs)) {
    for (const name of ["config.toml", "config"]) {
      try {
        found.push({ path: join(dir, name), text: readFileSync(join(dir, name), "utf8") });
      } catch {
        // absent (or a folder): cargo reads nothing there either
      }
    }
  }
  return found;
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

/**
 * One command, waited for: `timeout` (ms; 0, none) stops it with SIGKILL once passed (a SIGTERM the call handles could
 * hold it past the deadline: advisor-u4r-r1, FP-6 item 11, INF-U4R-10 (1)), and the result says so
 * (`timedOut`: spawnSync's ETIMEDOUT), so a hung call is a named failure, not a lock held forever (INF-4). A call with a
 * timeout runs in its own process group (spawnSync's `detached`: the child calls setsid), and once it is stopped that
 * whole group is killed (SIGKILL to the group), so every process it started goes with it (carry 84 (b): spawnSync's
 * timeout signals the direct child alone, so `npt-lab hash`'s two leg processes kept running). Limits: a process that
 * leaves the group by its own setsid is not reached; and since the group is its own session, a signal that stops
 * check.mjs while such a call runs (Ctrl-C) does not reach the call: end it by hand (`pgrep -a npt-lab`; `pgrep -af
 * loader.mjs`). check.test.mjs runs it on real processes (carry 84 (c)).
 */
export function run(cmd, args, cwd, env = process.env, timeout = 0) {
  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout, detached: true, killSignal: "SIGKILL" } : {}) });
  const timedOut = r.error?.code === "ETIMEDOUT";
  if (timedOut && r.pid) {
    try {
      process.kill(-r.pid, "SIGKILL");
    } catch {
      // ESRCH: no process of the group is left.
    }
  }
  // `stdout` alone is kept for the steps that parse it: cargo writes progress lines ("Blocking waiting for file
  // lock", "Downloaded …") to stderr, and they must never reach a JSON parser.
  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status, timedOut };
}

/**
 * cargo test's counts, summed over every test binary, and the tests that reported a skip (review C2R-9): libtest has no
 * skip, so a test that cannot run on this host passes and writes one line `SKIPPED <test>: <why>` to stderr itself,
 * past libtest's capture (crates/tcpinfo's MPTCP test on a kernel without MPTCP). Each is counted and named here, so
 * the step's summary line shows it; it stays counted in `passed` too, as libtest counts it.
 */
export function cargoCounts(out) {
  let passed = 0;
  let failed = 0;
  for (const m of out.matchAll(/test result: \w+\. (\d+) passed; (\d+) failed/g)) {
    passed += Number(m[1]);
    failed += Number(m[2]);
  }
  const skipped = [...out.matchAll(/^SKIPPED (\S+): /gm)].map((m) => m[1]);
  const which = skipped.length ? ` (${skipped.join(", ")})` : "";
  return `${passed} tests passed, ${failed} failed, ${skipped.length} reported a skip${which}`;
}

/** node --test's counts. */
function nodeCounts(out) {
  const get = (k) => out.match(new RegExp(`^# ${k} (\\d+)`, "m"))?.[1] ?? "?";
  return `${get("pass")} of ${get("tests")} tests passed`;
}

function summarized(r, summary) {
  return { ...r, out: `${r.out}\n${summary(r.out)}` };
}

/**
 * The crates linted for wasm32: the leaf crates, and npt-tcpinfo, whose non-Linux stub is how "the workspace still
 * builds anywhere" (u3c-plan.md 1.8 (c); review F-N): wasm32 is a target that is not Linux, and the crate has no
 * dependency there. Its lib only: its tests and the probe are Linux's.
 */
export const WASM_CRATES = [...LEAF, "npt-tcpinfo"];
const WASM_ARGS = ["--target", "wasm32-unknown-unknown", ...WASM_CRATES.flatMap((c) => ["-p", c])];

/**
 * A failed cargo run with its summary line (the step's last line, which main prints) naming the first `error` line of
 * its output, not cargo's last line ("warning: build failed, waiting for other jobs to finish..."; the rust fix2
 * review's nit, u4s-carries.md entry 34): `what` says which run failed. The detail stays in the lines above it.
 */
export function failedRun(r, what, before = "", note = "") {
  const first = r.out.split("\n").find((l) => /^error(?:\[\w+\])?:/.test(l.trim()));
  const said = first ? first.trim() : "its output has no error line (the lines above)";
  return { ...r, ok: false, out: `${before}${r.out}${note}\n${what} failed: ${said}` };
}

/**
 * The failing tests' names, from libtest's `failures:` lists in a test run's stdout (the list after the panics, one
 * name a line, four spaces in), in order, each once (carry 65; the advisor's U4j round-2 ruling, B2.3: `run` joins
 * stdout before stderr and main prints a failed step's last 40 lines, so without this line the names never show).
 */
export function failingTests(stdout) {
  const names = [];
  const lines = stdout.split("\n").map((l) => l.replace(/\r$/, ""));
  for (let i = 0; i < lines.length; i++) {
    if (lines[i] !== "failures:") continue;
    for (let j = i + 1; j < lines.length && /^ {4}\S/.test(lines[j]); j++) {
      const name = lines[j].trim();
      if (!names.includes(name)) names.push(name);
    }
  }
  return names;
}

/** The line a failed test run gets just above its summary: its failing tests by name, or why none is named. */
function failingTestsLine(r, build) {
  const names = failingTests(r.stdout ?? r.out);
  return names.length
    ? `\n${build}'s failing tests (libtest's failures: list): ${names.join(", ")}`
    : `\n${build}: no failing test in libtest's failures: list (a build or harness error: the lines above)`;
}

/**
 * clippy with warnings as errors on the measured build (no features), then on the bites build (every feature), through
 * `runner` (`run`; check.test.mjs passes a recorder). A failure's summary names its build and its first error.
 */
export function lintBothBuilds(scope, runner = run) {
  const measured = runner("cargo", ["clippy", "--locked", ...scope, "--", "-D", "warnings"], HERE);
  if (!measured.ok) return failedRun(measured, "the measured build's clippy");
  const bites = runner("cargo", ["clippy", "--locked", ...scope, "--all-features", "--", "-D", "warnings"], HERE);
  if (!bites.ok) return failedRun(bites, "the bites build's clippy (the measured build's passed)", measured.out);
  return { ok: true, out: `${measured.out}${bites.out}\nmeasured build and bites build both linted`, status: bites.status };
}

/**
 * rustdoc on every member's own items, no dependency's, with warnings as errors (the rust fix-review's G2: a broken
 * intra-doc link had passed every step), on the measured build, then on the bites build: a feature compiles items, and
 * their docs, in and out; then the same two for wasm32 on WASM_CRATES, so the code a target compiles in (npt-tcpinfo's
 * non-Linux stub) is documented too (the rust fix2 review's nit, u4s-carries.md entry 34). The flags go in
 * RUSTDOCFLAGS; CARGO_ENCODED_RUSTDOCFLAGS, which cargo would read in its place, is dropped from this run's
 * environment (`base`, process.env when check.mjs runs it). Through `runner`, as lintBothBuilds.
 */
export function docBothBuilds(runner = run, base = process.env) {
  const { CARGO_ENCODED_RUSTDOCFLAGS: _, ...env } = base;
  env.RUSTDOCFLAGS = "-D warnings";
  const host = ["doc", "--locked", "--no-deps", "--workspace"];
  const wasm = ["doc", "--locked", "--no-deps", ...WASM_ARGS];
  let out = "";
  for (const [args, what] of [
    [host, "the measured build's rustdoc"],
    [[...host, "--all-features"], "the bites build's rustdoc"],
    [wasm, "the measured build's wasm32 rustdoc"],
    [[...wasm, "--all-features"], "the bites build's wasm32 rustdoc"],
  ]) {
    const r = runner("cargo", args, HERE, env);
    if (!r.ok) return failedRun(r, what, out);
    out += r.out;
  }
  return { ok: true, out: `${out}\nmeasured build and bites build both documented, host and wasm32, rustdoc warnings as errors` };
}

/** Both builds' tests: the measured build (no features), then the bites build (every feature, the planted defects
 * compiled in: plan §4.7). Each count is reported, skips included (cargoCounts); a failed run names its failing tests
 * just above its summary (failingTests: carry 65). Through `runner`, as lintBothBuilds. */
export function testBothBuilds(runner = run) {
  const measured = runner("cargo", ["test", "--locked", "--workspace"], HERE);
  if (!measured.ok) {
    return failedRun(measured, `the measured build's tests (${cargoCounts(measured.out)})`, "", failingTestsLine(measured, "the measured build"));
  }
  const bites = runner("cargo", ["test", "--locked", "--workspace", "--all-features"], HERE);
  const summary = `measured build: ${cargoCounts(measured.out)}; bites build: ${cargoCounts(bites.out)}`;
  if (!bites.ok) return failedRun(bites, `the bites build's tests (${summary})`, measured.out, failingTestsLine(bites, "the bites build"));
  return { ok: true, out: `${measured.out}${bites.out}\n${summary}`, status: bites.status };
}

/**
 * What the deps step checks, as its summary line says it (plan §4.2's dependency and unsafe rules that are read from
 * `cargo metadata`, the manifests, the crates' files, the environment and the cargo configs; §4.2's wasm32 builds and
 * clock reads are the clippy steps', its "no reference into spikes/" is the refs step's, and its narrow public
 * surfaces are not checked by this command).
 *
 * The static unsafe scan guards against an accidental relaxation; review, and U5's audit, are the wall against a
 * deliberate one. Between them stands the compiler: under the workspace's forbid, rustc refuses unsafe code and any
 * allow of the lint (E0453, a macro-built allow included) in every crate but the three exceptions, whatever the scan
 * misreads. Known limits, which the step does not report:
 * - lexRust is not rustc's lexer (a shebang line; a `\u{...}` escape with underscores past six characters: review
 *   C2R-1), and pathProblems reads a #[path] or include! literal with its escapes undecoded and include! by its name
 *   only (C2R-2). The compiler covers both in the crates under the forbid; inside npt-tcpinfo and npt-hash-wasm only
 *   review does.
 * - The build-flag scan and rustc's read-back read the rustflags sources above, not a build script's
 *   `cargo::rustc-flags` or a RUSTC_WRAPPER's own flags. No source may hold a flag (buildInputProblems: the advisor's
 *   U4d2 round-1 ruling, FX-1), so on a step that passes the read-back is given no flag, which is what cargo gives
 *   rustc from those sources. The Wasm leg's loader reads the module's target_features section (FX-2): a feature
 *   enabled at compile time, by a flag from another route or a source-level `#[target_feature]`, shows there and is
 *   refused; but the section is the linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can
 *   rewrite it (built: sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check at all
 *   (carry 85 (a)).
 * - Inputs that can lower lints and are not read here: a cargo config's [env] table (CLIPPY_ARGS, C2R-4 G, or any
 *   other variable it sets for rustc), what a member's build script prints (`cargo::rustc-env`, C2R-4 H), a
 *   RUSTC_WRAPPER program (allowed, for sccache) and a rust-toolchain.toml `path`. Lint capping by any of them
 *   (`--cap-lints`) lowers the forbid too.
 * - Which toolchain runs: a nightly one (rust-toolchain.toml's channel, RUSTUP_TOOLCHAIN, `cargo +nightly` or a rustup
 *   override) is not detected, and its unstable features are not read. The manifest keys that reach rustc's flags
 *   (`cargo-features`, `rustflags`) are refused whatever the channel, and so are the two variables found that unlock
 *   unstable features on the pinned stable toolchain, RUSTC_BOOTSTRAP (review C2b F5) and cargo's channel override
 *   __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS (C2c N3). Any other way to unlock them on a stable toolchain (another
 *   variable, a patched or rebuilt cargo or rustc) is out of the scan's reach.
 * - Unsafe code a dependency's macro expands into a crate is not linted by rustc (another crate's code); the
 *   dependencies are U5's audit.
 * - Too wide, not too narrow: the runner refusal covers every target triple, so a wasm32 test runner
 *   (wasm-bindgen-test's `[target.wasm32-unknown-unknown] runner`) would be refused too, though the test step runs host
 *   tests only (review C2b's note). The wasm step's Wasm legs need none: they are a Node script (crates/hash-wasm's
 *   loader, run through process.execPath), not a cargo test, so the refusal stands (u4-build-plan.md §5 U4d2).
 */
const DEPS_CHECKED =
  "held: no leaf crate reaches a transport or runtime; npt-client has no npt-net; npt-super and npt-host keep off the " +
  "sim, room, proto (and client) crates; npt-judge ships npt-log and serde only; npt-tcpinfo ships libc only and only " +
  "npt-net uses it; npt-net has no direct libc; npt-sim ships libm, rand_chacha and rand_core only; npt-hash-wasm " +
  "ships npt-sim only; nothing depends on npt-lab or npt-hash-wasm; the workspace forbids unsafe_code and " +
  "every member inherits its lints but npt-client-wasm (allow), npt-tcpinfo (deny) and npt-hash-wasm (deny), whose own " +
  "lint tables are the workspace's otherwise, key for key, with no lint given in a form the reader does not read (a " +
  "lint's own table, a quoted, dotted, hyphenated or uppercase name); no manifest has a cargo-features or rustflags " +
  "key; as the scan reads crates/ (lexRust), no other crate relaxes unsafe_code or has unsafe code, " +
  "npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
  "them, and keeps lib.rs's deny of the two clippy unsafe lints, and npt-hash-wasm relaxes it only by one #[allow] on " +
  "each of its seven listed exports, whose #[unsafe(no_mangle)] is its only unsafe code, each taking and giving u32 " +
  "or i32 only; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but directly before `!` " +
  "and an opening bracket; no attribute holds a metavariable; " +
  "the environment and the cargo configs read pass no " +
  "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no linker, no profile, " +
  "no config include and " +
  "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; no rustflags source holds a " +
  "flag (so none enables fma, simd128 or relaxed-simd or sets target-cpu=native) and neither the environment nor a " +
  "cargo config sets the build target, and rustc's read-back, given what cargo gives rustc, lists sse2 and " +
  "no fma for x86-64 and sign-ext and no simd128 or relaxed-simd for wasm32; npt-sim's resolved features, its own " +
  "and every package's below it, are one set in the npt-server, npt-lab and npt-hash-wasm (wasm32) builds, measured " +
  "with measured and bites with bites, each holding npt-sim's own dependencies; every member lives in " +
  "crates/ and builds only .rs files in its own folder, in edition 2024, none " +
  "a proc-macro, and every path dependency is a member; no symbolic link under crates/; every #[path] and include! " +
  "literal names a .rs file in its own crate; TICK_HZ is set from a literal once, in npt-sim; each derived tick " +
  "constant is its own function at TICK_HZ in npt-sim and a path to it elsewhere, and no other *_TICKS or *CATCHUP* " +
  "constant is set from a literal; npt-room's sources hold no float; no bite id is in two crates' sources; " +
  "npt-proto's clippy.toml is npt-sim's below its header; npt-proto's and npt-client's sources name only Channel " +
  "from npt_link, which neither manifest renames";

/** `cargo metadata` for the spike in `here`, with every feature, parsed: { ok, meta }, or the failed run. */
function cargoMetadata(here) {
  const r = run("cargo", ["metadata", "--locked", "--format-version", "1", "--all-features"], here);
  return r.ok ? { ok: true, meta: JSON.parse(r.stdout) } : r;
}

/**
 * The deps step on the spike in `here`, with `env` as the environment, `metadata(here)` for cargo metadata and
 * `runner` for the `cargo tree` and `rustc` calls of the features comparison and the read-back: every check it makes is
 * called here, and STEPS.deps is this function (review C2R-5: check.test.mjs runs it on a fixture with one plant per
 * check, so a check unhooked from it fails a test). Its summary line ends with the read-back's lists and npt-sim's
 * features.
 */
export function depsStep(here = HERE, env = process.env, metadata = cargoMetadata, runner = run) {
  const r = metadata(here);
  if (!r.ok) return r;
  const { meta } = r;
  const { files, links } = crateFiles(join(here, "crates"));
  const configs = cargoConfigFiles(here, env);
  const read = (f) => readFileSync(join(here, f), "utf8");
  const workspace = read("Cargo.toml");
  const manifests = [{ path: "Cargo.toml", text: workspace }];
  for (const f of files) if (f.endsWith("/Cargo.toml")) manifests.push({ path: f, text: read(f) });
  const sources = rustflagSources(env, configs);
  const readBack = readBackCheck(sources, runner, here);
  const features = simFeatureCheck(meta, runner, here);
  const problems = [
    ...dependencyProblems(meta),
    ...workspaceLintProblems(workspace),
    ...exceptionLintProblems(workspace, read),
    ...manifestKeyProblems(manifests),
    ...layoutProblems(meta),
    ...links.map((l) => `${l} is a symbolic link: the scan does not follow it, and rustc would`),
    ...cargoConfigProblems(env, configs),
    ...unsafeProblems(files, read),
    ...tickConstantProblems(files, read),
    ...roomFloatProblems(files, read),
    ...biteHomeProblems(files, read),
    ...clippyTwinProblems(read),
    ...linkUseProblems(files, read),
    ...buildFlagProblems(sources),
    ...buildInputProblems(env, configs),
    ...readBack.problems,
    ...features.problems,
  ];
  const counts = `${files.length} crate files and ${configs.length} cargo config file(s) read; ${readBack.line}; ${features.line}`;
  return { ok: problems.length === 0, out: problems.join("\n") || `${DEPS_CHECKED} (${counts})` };
}

/** The recordings manifest (u4-build-plan.md §4.15; carry 52), the std-sin scan data and h2's committed vector. */
export const RECORDINGS_MANIFEST = "crates/lab/recordings.json";
export const STD_SIN_SCAN = "crates/lab/std-sin-scan.txt";
export const H2_VECTOR = "crates/hash-wasm/co-initial-dump.txt";
/** The Node loader of the Wasm legs. */
export const LOADER = join(HERE, "crates", "hash-wasm", "loader.mjs");
/** Where the wasm step's copies of its builds, its recordings and its legs' outputs go: the checkout's .gate/. */
export const WASM_STEP_DIR = join(ROOT, ".gate", "netcode-proof-test", "wasm-step");
/**
 * The two bites the wasm step arms (PM-0004-21's, planted by U4d in npt-sim: plan.md §4.5:551, "one std `sin` and one
 * hash-map iteration, each caught at the first tick it changes"), named here as ops/runs/c-u2-pm-0001-3.mjs names
 * PM-0001-3's; each one's index (from 1, as npt-hash-wasm's `begin` takes it) is read from `npt-lab hash bites`.
 */
export const WASM_BITES = { hashMap: "PM-0004-21/hash-map", stdSin: "PM-0004-21/std-sin" };
/** The hash-map bite's arming: the manifest's first recording at tick 1,234 (D-2's form, sim/tests/replay.rs:291). */
export const HASH_MAP_ARM = 1_234;
/** The state word its order is folded into (world.rs's write_state: two words under one name). */
export const HASH_MAP_FIELD = "bite[0].hash_map_order";
/**
 * The step's release builds (u4d2-brief.md 2.4 (a) item 1: plan.md §4.5's flags, so none is added; wasm-opt is U7's,
 * C-U7-1), each artifact read from cargo's JSON messages and copied out before the next build replaces it.
 */
export const WASM_BUILDS = [
  { key: "lab", what: "npt-lab (x86-64, measured)", copy: "npt-lab", target: "npt-lab", kind: "bin", args: ["build", "--locked", "--release", "-p", "npt-lab", "--message-format=json"] },
  { key: "labBites", what: "npt-lab (x86-64, bites)", copy: "npt-lab-bites", target: "npt-lab", kind: "bin", args: ["build", "--locked", "--release", "-p", "npt-lab", "--features", "bites", "--message-format=json"] },
  {
    key: "wasm",
    what: "npt-hash-wasm (wasm32, measured)",
    copy: "npt-hash-wasm.wasm",
    target: "npt_hash_wasm",
    kind: "cdylib",
    args: ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "npt-hash-wasm", "--message-format=json"],
  },
  {
    key: "wasmBites",
    what: "npt-hash-wasm (wasm32, bites)",
    copy: "npt-hash-wasm-bites.wasm",
    target: "npt_hash_wasm",
    kind: "cdylib",
    args: ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "npt-hash-wasm", "--features", "bites", "--message-format=json"],
  },
];

/** The artifact a build's JSON messages name: a bin's executable, a cdylib's .wasm file; null if none. */
export function builtArtifact(stdout, { target, kind }) {
  let found = null;
  for (const line of stdout.split("\n")) {
    if (!line.startsWith("{")) continue;
    let m;
    try {
      m = JSON.parse(line);
    } catch {
      continue;
    }
    if (m.reason !== "compiler-artifact" || m.target?.name !== target || !m.target?.kind?.includes(kind)) continue;
    found = kind === "bin" ? (m.executable ?? null) : ((m.filenames ?? []).find((f) => f.endsWith(".wasm")) ?? null);
  }
  return found;
}

/**
 * One `<a> against <b>: ...` line of npt-lab's `hash` or `hash compare` (HashCompare's result, as compare_text writes
 * it), read: { line, compared, mismatching, equal, first: { recording, tick } when both sides' first mismatching entries
 * are the same tick, `at`: that entry as written, field: the first differing field when named, counts: each
 * recording's ticks in its first list }; null when no such line was written (the run failed).
 */
export function readComparison(text, a, b) {
  const head = `${a} against ${b}: `;
  const line = text.split("\n").find((l) => l.startsWith(head));
  if (!line) return null;
  const body = line.slice(head.length);
  const tail = body.match(/ \(([^()]*)\)$/);
  const counts = new Map(
    (tail?.[1] ?? "")
      .split(", ")
      .filter(Boolean)
      .map((s) => [s.split(" ")[0], Number(s.split(" ")[1])]),
  );
  const rest = tail ? body.slice(0, tail.index) : body;
  const compared = Number(rest.match(/^(\d+) ticks compared/)?.[1] ?? Number.NaN);
  const mismatching = compared === 0 ? 0 : Number(rest.match(/, (\d+) mismatching/)?.[1] ?? Number.NaN);
  const firstAt = rest.match(/; first mismatch at entry \d+: (.*)$/)?.[1] ?? null;
  const sides = firstAt?.match(/^(\S+) tick (\d+) hash \S+ against (\S+) tick (\d+) hash \S+;/);
  const first = sides && sides[1] === sides[3] && sides[2] === sides[4] ? { recording: sides[1], tick: Number(sides[2]) } : null;
  return {
    line,
    compared,
    mismatching,
    equal: /, 0 mismatching, equal$/.test(rest),
    first,
    at: firstAt,
    field: firstAt?.match(/; field (\S+) \(word \d+\)/)?.[1] ?? null,
    counts,
  };
}

/** The std-sin scan data (STD_SIN_SCAN; u4d2-brief.md 2.3): its catches (`catch <recording> <tick> <field>`), whether it
 * says "not catchable at this site on x86-64 and wasm32", and its other lines (the scan's record). */
export function readScan(text) {
  const catches = [];
  let notCatchable = false;
  const record = [];
  for (const raw of text.split("\n")) {
    const l = raw.trim();
    if (!l || l.startsWith("#")) continue;
    const c = l.match(/^catch (\S+) (\d+) (\S+)$/);
    if (c) catches.push({ recording: c[1], tick: Number(c[2]), field: c[3] });
    else if (l === "not catchable at this site on x86-64 and wasm32") notCatchable = true;
    else record.push(l);
  }
  return { catches, notCatchable, record };
}

/**
 * The wasm step's one deadline, shared by every call but its cargo builds from the first of them (carry 84 (a)): the
 * step's whole 5-minute budget.
 */
const WASM_CALL_LIMIT_MS = 300_000;
/** The real file operations of the wasm step (check.test.mjs passes its own), paths under the spike for `read`. */
const WASM_IO = {
  reset: (dir) => {
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
  },
  mkdir: (dir) => mkdirSync(dir, { recursive: true }),
  copy: (from, to) => copyFileSync(from, to),
  exists: (path) => existsSync(path),
  read: (path) => readFileSync(join(HERE, path), "utf8"),
  libc: () => {
    const v = process.report?.getReport?.()?.header?.glibcVersionRuntime;
    return v ? `glibc ${v} (Node's report)` : "no glibc found by Node's report";
  },
};

/**
 * The determinism check (u4-build-plan.md §5 U4d2; plan.md §4.5:546-551; ADR 0002:500-505), through `runner` for every
 * command (cargo, npt-lab, Node), `io` for its files and `now` for its clock (by default monotonic, performance.now,
 * which times each call and the shared deadline: advisor-u4r-r1, FP-6 item 11, INF-U4R-10 (3); npt-lab reads none):
 * 1. the release builds (WASM_BUILDS), each artifact's path read from cargo's JSON and copied out;
 * 2. `npt-lab hash` (measured): the manifest's recordings regenerated into the step's folder and checked by length and
 *    SHA-256, then its two native legs in lock step, leg 1 against leg 2;
 * 3. the Node loader, once per recording, its own process each (process.execPath): Wasm leg 1;
 * 4. `npt-lab hash compare`: native leg 1 against Wasm leg 1, and at a first mismatch the loader again with that tick's
 *    dump and the compare again, which names the field;
 * 5. h2's vectors on wasm32: each header-only recording through the loader to its dump, then `npt-lab hash vectors`;
 * 6. the bites half, in the bites builds: `npt-lab hash bites` for the indexes; PM-0004-21/hash-map on the manifest's
 *    first recording at tick 1,234, four legs (native 1 and 2, Wasm 1 and 2, each its own process), three pairs by
 *    D-2's rule (sim/tests/replay.rs:300-308): digests that differ must first differ at that recording, that tick,
 *    HASH_MAP_FIELD; equal digests are "not exercised" and no tick may differ; no pair exercised fails; equal Wasm
 *    digests are "not catchable on wasm32". PM-0004-21/std-sin by the scan data (STD_SIN_SCAN, the advisor's U4D2-Q5
 *    ruling): a catch (recording R, tick T', field F) arms it at T' on R, native leg 1 and Wasm leg 1, whose first
 *    mismatch must be (R, T', F); "not catchable at this site on x86-64 and wasm32" is printed with the scan's record,
 *    and no std-sin leg runs.
 *    The re-show (carry 83 (c)): the bitten native leg against a clean bites-build native leg (the hash-map half's
 *    native leg 1, clean on the manifest's first recording before HASH_MAP_ARM: that pair's equal prefix) must first
 *    differ at the scan's (recording, tick, field); a catch elsewhere fails the step, as no clean leg covers it.
 * It passes only with every manifest recording's ticks compared (its count each, their sum per pair) and 0 mismatching
 * for leg 1 against leg 2 and leg 1 against Wasm, `npt-lab hash` and its clean `compare` exiting 0 (carry 83 (b)), h2's
 * vectors equal and each bite's verdict as above. On a failure it still runs every comparison it has the legs for; its
 * last lines are one line per pair, h2 and each bite, then each problem, then the summary, which main prints: the
 * ticks per pair, the mismatches, each bite's outcome and each timed call's seconds.
 *
 * Its options (the advisor's U4r build ruling, point 1 (4)): `manifest`, the recordings manifest every `npt-lab hash`
 * call and the Wasm legs read (RECORDINGS_MANIFEST by default); `bitesHalf`, whether step 6 and its two bites builds
 * run (by default they do). ops/runs/det-u4.mjs runs the clean half on crates/lab/det-recordings.json with the bites
 * half off; check.mjs runs the defaults.
 */
export function wasmStep(runner = run, io = WASM_IO, now = () => performance.now(), dir = WASM_STEP_DIR, { manifest = RECORDINGS_MANIFEST, bitesHalf = true } = {}) {
  const problems = [];
  const lines = [];
  const timed = [];
  // Every call but the four cargo builds shares one deadline, WASM_CALL_LIMIT_MS from the first of them (carry 84 (a);
  // before it, each call had its own: the advisor's U4d2 round-1 ruling, FX-7: INF-4): each is given what is left, and
  // once one is stopped, or the time is spent, the rest are not started, each named as skipped. A hung leg would hold
  // the lane lock for every other lane's cargo. The builds have none: under the exclusive lock no other cargo holds
  // theirs, and a cold build's time is INF-1's to measure, not to cut.
  const share = `${WASM_CALL_LIMIT_MS / 1000} s shared by every call but its cargo builds, from the first of them`;
  let deadline = null;
  let stopped = null;
  const call = (label, cmd, args) => {
    const t0 = now();
    if (cmd !== "cargo") {
      deadline ??= t0 + WASM_CALL_LIMIT_MS;
      if (stopped || t0 >= deadline) {
        problems.push(`${label}: not started (skipped): ${stopped ? `${stopped} was stopped at the step's deadline` : "the step's deadline had passed"} (${share})`);
        return { ok: false, out: "", stdout: "", status: null, timedOut: false, skipped: true };
      }
    }
    // Whole ms, as spawnSync takes them (the monotonic clock is fractional); at least 1 while time is left, never 0
    // (no limit).
    const limit = cmd === "cargo" ? 0 : Math.ceil(deadline - t0);
    const r = runner(cmd, args, HERE, undefined, limit);
    timed.push(`${label} ${((now() - t0) / 1000).toFixed(1)}`);
    if (r.timedOut) {
      stopped ??= label;
      problems.push(`${label}: still running at the step's deadline (${share}; ${(limit / 1000).toFixed(1)} s were left for it), so it was stopped with every process it started`);
    }
    return r;
  };
  const failure = (r) => (r.skipped ? "not started (skipped: the step's deadline)" : lastLines(r.out, 1).trim() || `exit ${r.status}`);
  // Carry 83 (b): a clean-half call must also exit 0; its lines alone are not its verdict.
  const exited = (what, r) => {
    if (r && !r.skipped && !r.timedOut && r.status !== 0) problems.push(`${what}: exit ${r.status ?? "by a signal"}, not 0 (the clean half requires exit 0 as well as its lines)`);
  };
  const bin = join(dir, "bin");
  const rec = join(dir, "recordings");
  const meas = join(dir, "measured");
  const hm = join(dir, "hash-map");
  const sin = join(dir, "std-sin");
  io.reset(dir);
  for (const d of [bin, rec, meas, hm, sin]) io.mkdir(d);
  // 1. The builds (the two bites builds only with the bites half).
  const art = {};
  for (const b of WASM_BUILDS.filter((x) => bitesHalf || !["labBites", "wasmBites"].includes(x.key))) {
    const r = call(`build ${b.copy}`, "cargo", b.args);
    const path = r.ok ? builtArtifact(r.stdout ?? "", b) : null;
    if (!r.ok) problems.push(`the release build of ${b.what} failed: ${failure(r)}`);
    else if (!path) problems.push(`the release build of ${b.what} names no ${b.kind} artifact in cargo's JSON messages`);
    else {
      art[b.key] = join(bin, b.copy);
      io.copy(path, art[b.key]);
    }
  }
  // The manifest's recordings, as the loader needs them (npt-lab checks the manifest's own rules).
  let entries = [];
  try {
    const m = JSON.parse(io.read(manifest));
    entries = m.recordings.map((e) => ({ name: e.name, first: e.first_tick, ticks: e.ticks }));
  } catch (e) {
    problems.push(`${manifest} cannot be read: ${e.message}`);
  }
  const total = entries.reduce((s, e) => s + e.ticks, 0);
  const manifestArgs = ["--manifest", manifest, "--dir", rec];
  const loader = (wasm, e, out, extra = []) => [LOADER, "--wasm", wasm, "--rec", join(rec, `${e.name}.rec`), "--name", e.name, "--first-tick", String(e.first), "--out", out, ...extra];
  const stepped = (r) => Number(r.out.match(/: (\d+) ticks stepped/)?.[1] ?? Number.NaN);
  // A recording regenerated in this run (the folder is new each run, and npt-lab writes each file once it is checked).
  const has = (e) => io.exists(join(rec, `${e.name}.rec`));
  // One pair of the clean half: every recording at its count, the sum compared, 0 mismatching.
  const judgePair = (what, c, r) => {
    if (!c) {
      problems.push(`${what}: no comparison was written${r ? `: ${failure(r)}` : " (no leg to compare)"}`);
      return `${what}: not compared`;
    }
    if (c.compared !== total) problems.push(`${what}: ${c.compared} ticks compared, not the manifest's ${total}`);
    for (const e of entries) {
      if (c.counts.get(e.name) !== e.ticks) problems.push(`${what}: ${e.name} has ${c.counts.get(e.name) ?? 0} ticks compared, the manifest's ${e.ticks}`);
    }
    if (!c.equal) problems.push(`${what}: ${c.mismatching} mismatching; first mismatch at ${c.at ?? "no entry named"}`);
    return c.line;
  };
  // 2-4. The clean half.
  let vectors = [];
  if (art.lab && entries.length) {
    const legs = call("npt-lab hash", art.lab, ["hash", ...manifestArgs, "--out", meas]);
    exited("npt-lab hash", legs);
    const regen = legs.out.match(/^npt-lab hash: (.*(?:the manifest has|not this build's Recording::GENERATOR).*)$/m)?.[1];
    if (regen) problems.push(`the recordings manifest: ${regen}`);
    if (/order digest/.test(legs.out)) problems.push("a clean-half native leg wrote an order digest: a bites build's output given to the measured half");
    vectors = (legs.out.match(/^wrote the header-only recordings (.*) \(seed 1/m)?.[1] ?? "").split(", ").filter(Boolean);
    lines.push(judgePair("native leg 1 against native leg 2", readComparison(legs.out, "native leg 1", "native leg 2"), legs));
  } else if (!art.lab) problems.push("the clean half did not run: no measured npt-lab build");
  if (art.lab && art.wasm && entries.some(has)) {
    const theirs = [];
    for (const e of entries.filter(has)) {
      const out = join(meas, `wasm-${e.name}.lines`);
      const r = call(`Wasm ${e.name}`, process.execPath, loader(art.wasm, e, out));
      if (!r.ok) problems.push(`Wasm leg 1: ${failure(r)}`);
      else if (stepped(r) !== e.ticks) problems.push(`Wasm leg 1: ${e.name} stepped ${stepped(r)} ticks, the manifest's ${e.ticks}`);
      if (/\bbite\b/.test(r.out)) problems.push(`Wasm leg 1: ${e.name}'s leg carries a bite (a bites build's output given to the measured half)`);
      if (io.exists(out)) theirs.push(out);
    }
    const compareArgs = ["hash", "compare", "--ours", join(meas, "native-1.lines"), "--theirs", theirs.join(","), ...manifestArgs];
    const names = ["--ours-name", "native leg 1", "--theirs-name", "Wasm leg 1"];
    const cr = theirs.length ? call("compare", art.lab, [...compareArgs, ...names]) : null;
    exited("npt-lab hash compare (native leg 1 against Wasm leg 1)", cr);
    let c = cr ? readComparison(cr.out, "native leg 1", "Wasm leg 1") : null;
    const e = c?.first && !c.field ? entries.find((x) => x.name === c.first.recording) : null;
    if (e) {
      // The field diff: the loader again, with that tick's dump, then the compare naming the field.
      const dump = join(meas, `wasm-${e.name}-${c.first.tick}.dump`);
      const r = call(`Wasm dump ${e.name}`, process.execPath, loader(art.wasm, e, join(meas, `wasm-${e.name}-dump.lines`), ["--dump-at", String(c.first.tick), "--dump-out", dump]));
      if (r.ok) c = readComparison(call("compare field", art.lab, [...compareArgs, "--theirs-dump", dump, ...names]).out, "native leg 1", "Wasm leg 1") ?? c;
    }
    lines.push(judgePair("native leg 1 against Wasm leg 1", c, cr));
  } else problems.push(`the Wasm leg did not run: ${art.wasm ? "no recording was regenerated" : "no measured npt-hash-wasm build"}`);
  // 5. h2's vectors on wasm32.
  if (art.lab && art.wasm && vectors.length) {
    for (const v of vectors) {
      const r = call(`h2 ${v}`, process.execPath, loader(art.wasm, { name: v, first: 0 }, join(meas, `${v}.lines`), ["--dump-at", "end", "--dump-out", join(meas, `${v}.dump`)]));
      if (!r.ok) problems.push(`h2: ${failure(r)}`);
    }
    const r = call("h2 vectors", art.lab, ["hash", "vectors", "--dir", rec, "--dumps", meas, "--committed", H2_VECTOR]);
    const said = r.out.split("\n").filter((l) => l.startsWith("h2: "));
    lines.push(...said);
    const differs = said.filter((l) => l !== "h2: the vectors differ (the lines above)").map((l) => l.slice(4));
    if (!r.ok) problems.push(`h2: the vectors differ or did not run: ${differs.join("; ") || failure(r)}`);
  } else problems.push("h2: the vectors did not run (no build, or npt-lab wrote no header-only recording)");
  // 6. The bites half.
  const bites = { hashMap: "not run", stdSin: "not run" };
  const index = {};
  if (!bitesHalf) {
    // Off by the option: no bites build was made and no bites-build call starts (ops/runs/det-u4.mjs).
    bites.hashMap = "not run: the bites half is off (wasmStep's bitesHalf option)";
    bites.stdSin = bites.hashMap;
  } else if (art.labBites && art.wasmBites && entries.some(has)) {
    const listed = call("npt-lab bites", art.labBites, ["hash", "bites"]);
    for (const m of listed.out.matchAll(/^(\d+) (\S+)$/gm)) index[m[2]] = Number(m[1]);
    for (const id of Object.values(WASM_BITES)) if (!index[id]) problems.push(`npt-lab hash bites lists no ${id} (World::BITES)`);
  } else problems.push(`the bites half did not run: ${art.labBites && art.wasmBites ? "no recording was regenerated" : "no bites builds"}`);
  const rec0 = entries[0];
  // Whether the hash-map half's native legs ran: its leg 1 is the std-sin re-show's clean bites-build leg.
  let cleanLeg = false;
  if (index[WASM_BITES.hashMap] && rec0 && !has(rec0)) problems.push(`hash-map: ${rec0.name} was not regenerated, so it did not run`);
  if (index[WASM_BITES.hashMap] && rec0 && has(rec0)) {
    const arm = String(HASH_MAP_ARM);
    const bite = ["--bite", WASM_BITES.hashMap, "--arm", arm, "--on", rec0.name];
    const legs = call("hash-map native legs", art.labBites, ["hash", ...manifestArgs, "--out", hm, "--only", rec0.name, "--bite", WASM_BITES.hashMap, "--arm", arm, "--dump-at", arm]);
    cleanLeg = !legs.skipped && !legs.timedOut;
    const digests = { "native leg 1": legs.out.match(/^leg 1 order digest (\S+)/m)?.[1], "native leg 2": legs.out.match(/^leg 2 order digest (\S+)/m)?.[1] };
    for (const k of [1, 2]) {
      const r = call(`hash-map Wasm ${k}`, process.execPath, loader(art.wasmBites, rec0, join(hm, `wasm-${k}.lines`), ["--bite", String(index[WASM_BITES.hashMap]), "--arm", arm, "--dump-at", arm, "--dump-out", join(hm, `wasm-${k}.dump`)]));
      if (!r.ok) problems.push(`hash-map Wasm leg ${k}: ${failure(r)}`);
    }
    const pair = (label, a, b, ours, theirs) => {
      const r = call(label, art.labBites, ["hash", "compare", "--ours", join(hm, `${ours}.lines`), "--theirs", join(hm, `${theirs}.lines`), ...manifestArgs, "--ours-dump", join(hm, `${ours}.dump`), "--theirs-dump", join(hm, `${theirs}.dump`), "--digest-field", HASH_MAP_FIELD, ...bite, "--ours-name", a, "--theirs-name", b]);
      for (const n of [a, b]) digests[n] ??= r.out.match(new RegExp(`^${n} digest (\\S+) \\(`, "m"))?.[1];
      return readComparison(r.out, a, b);
    };
    const n12 = readComparison(legs.out, "native leg 1", "native leg 2");
    const nw = pair("hash-map compare native-Wasm", "native leg 1", "Wasm leg 1", "native-1", "wasm-1");
    const ww = pair("hash-map compare Wasm-Wasm", "Wasm leg 1", "Wasm leg 2", "wasm-1", "wasm-2");
    // D-2's rule, per pair (sim/tests/replay.rs:300-308).
    const verdicts = [];
    let exercised = 0;
    for (const [a, b, c] of [["native leg 1", "native leg 2", n12], ["native leg 1", "Wasm leg 1", nw], ["Wasm leg 1", "Wasm leg 2", ww]]) {
      const [da, db] = [digests[a], digests[b]];
      const what = `hash-map, ${a} against ${b}`;
      if (!c || !da || !db || da === "none" || db === "none") {
        problems.push(`${what}: ${c ? `a leg logged no order digest (${da ?? "none"}, ${db ?? "none"})` : "no comparison was written (the lines above)"}`);
        verdicts.push(`${a} - ${b} not judged`);
        continue;
      }
      if (c.compared !== rec0.ticks) problems.push(`${what}: ${c.compared} ticks compared, not ${rec0.name}'s ${rec0.ticks}`);
      if (da === db) {
        if (c.mismatching !== 0 || !c.equal) problems.push(`${what}: digests equal (${da}: not exercised), yet ${c.mismatching} ticks differ, first at ${c.at}`);
        verdicts.push(`${a} - ${b} not exercised (digests equal, ${da})`);
      } else {
        exercised++;
        const caught = c.first?.recording === rec0.name && c.first?.tick === HASH_MAP_ARM && c.field === HASH_MAP_FIELD;
        if (!caught) {
          problems.push(`${what}: digests ${da} and ${db} differ, but the first mismatch is ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}, not ${rec0.name} tick ${HASH_MAP_ARM}, field ${HASH_MAP_FIELD}`);
        }
        verdicts.push(`${a} - ${b} ${caught ? `caught at ${rec0.name} tick ${HASH_MAP_ARM}, ${HASH_MAP_FIELD}` : "NOT caught as D-2's rule asks"} (digests ${da}, ${db})`);
      }
    }
    if (exercised === 0) problems.push("hash-map: no pair was exercised (every pair's digests equal)");
    const wasmEqual = digests["Wasm leg 1"] && digests["Wasm leg 1"] === digests["Wasm leg 2"];
    bites.hashMap = `${verdicts.join("; ")}${wasmEqual ? "; not catchable on wasm32 (its two Wasm digests are equal)" : ""}`;
    lines.push(`PM-0004-21/hash-map (${rec0.name} armed at tick ${HASH_MAP_ARM}): ${bites.hashMap}`);
  }
  if (index[WASM_BITES.stdSin]) {
    let scan = null;
    try {
      scan = readScan(io.read(STD_SIN_SCAN));
    } catch (e) {
      problems.push(`${STD_SIN_SCAN} cannot be read: ${e.message}`);
    }
    const order = entries.map((e) => e.name);
    const catches = (scan?.catches ?? []).filter((c) => order.includes(c.recording)).sort((x, y) => order.indexOf(x.recording) - order.indexOf(y.recording));
    const record = scan?.record.join("; ") ?? "";
    const e = catches.length ? entries.find((x) => x.name === catches[0].recording) : null;
    if (e && !has(e)) problems.push(`std-sin: ${e.name} was not regenerated, so its leg did not run`);
    else if (e) {
      const { recording, tick, field } = catches[0];
      const t = String(tick);
      call("std-sin native legs", art.labBites, ["hash", ...manifestArgs, "--out", sin, "--only", recording, "--bite", WASM_BITES.stdSin, "--arm", t, "--dump-at", t]);
      const r = call("std-sin Wasm", process.execPath, loader(art.wasmBites, e, join(sin, "wasm-1.lines"), ["--bite", String(index[WASM_BITES.stdSin]), "--arm", t, "--dump-at", t, "--dump-out", join(sin, "wasm-1.dump")]));
      if (!r.ok) problems.push(`std-sin Wasm leg 1: ${failure(r)}`);
      const c = readComparison(
        call("std-sin compare", art.labBites, ["hash", "compare", "--ours", join(sin, "native-1.lines"), "--theirs", join(sin, "wasm-1.lines"), ...manifestArgs, "--ours-dump", join(sin, "native-1.dump"), "--theirs-dump", join(sin, "wasm-1.dump"), "--bite", WASM_BITES.stdSin, "--arm", t, "--on", recording, "--ours-name", "native leg 1", "--theirs-name", "Wasm leg 1"]).out,
        "native leg 1",
        "Wasm leg 1",
      );
      const caught = c?.first?.recording === recording && c.first.tick === tick && c.field === field;
      // The re-show (carry 83 (c)): the bite, not another native-Wasm difference, causes the mismatch judged. The bitten
      // native leg against a clean bites-build native leg, the hash-map half's leg 1 (clean on the manifest's first
      // recording before HASH_MAP_ARM: its pair's equal prefix), replayed clean in `compare`'s own process to name the
      // field against the bitten leg's dump.
      let reshown = false;
      if (recording !== rec0.name || tick >= HASH_MAP_ARM) {
        problems.push(
          `std-sin: the scan's catch ${recording} tick ${tick} is not on the manifest's first recording (${rec0.name}) before the hash-map bite's arming tick ${HASH_MAP_ARM}, so no clean bites-build native leg covers it and the bite's cause is not re-shown`,
        );
      } else if (!cleanLeg) {
        problems.push("std-sin: the re-show did not run: the hash-map half's native legs, its clean leg, did not run");
      } else {
        const s = call("std-sin re-show", art.labBites, ["hash", "compare", "--ours", join(hm, "native-1.lines"), "--theirs", join(sin, "native-1.lines"), ...manifestArgs, "--theirs-dump", join(sin, "native-1.dump"), "--ours-name", "clean native leg 1", "--theirs-name", "std-sin native leg 1"]);
        const rs = readComparison(s.out, "clean native leg 1", "std-sin native leg 1");
        reshown = rs?.first?.recording === recording && rs.first.tick === tick && rs.field === field;
        if (!reshown) {
          problems.push(
            `std-sin: the re-show (the bitten native leg against a clean bites-build native leg) ${rs ? `first differs at ${rs.first ? `${rs.first.recording} tick ${rs.first.tick}, field ${rs.field ?? "none"}` : (rs.at ?? "no entry")}` : `wrote no comparison: ${failure(s)}`}, not at the scan's ${recording} tick ${tick}, ${field}`,
          );
        }
      }
      bites.stdSin = caught
        ? `caught at ${recording} tick ${tick}, ${field}, as the scan found${reshown ? "; re-shown against a clean bites-build native leg" : ""}`
        : `NOT caught at the scan's ${recording} tick ${tick}, ${field}: ${c ? `first mismatch ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}` : "no comparison was written"}`;
      if (!caught) {
        problems.push(
          `std-sin: ${bites.stdSin}; this step ran on ${io.libc()}, the scan on ${scan.record.find((l) => /libc|ldd/i.test(l)) ?? "a libc it does not name"}; ` +
            "glibc picks its sinf by the CPU's features when it loads (an IFUNC), so on another CPU or libc the scan is re-run " +
            "there (u4d2-brief.md 2.4 (a) item 6; INF-3)",
        );
      }
      if (catches.length > 1) bites.stdSin += ` (the scan's other catches: ${catches.slice(1).map((x) => `${x.recording} tick ${x.tick}`).join(", ")})`;
    } else if (scan?.notCatchable) {
      bites.stdSin = `not catchable at this site on x86-64 and wasm32 (the scan: ${record}); no std-sin leg runs (the scratch plant stood in once: u4d2-brief.md 2.7)`;
    } else if (scan) {
      problems.push(`${STD_SIN_SCAN} records neither a catch on a manifest recording nor "not catchable at this site on x86-64 and wasm32"`);
    }
    lines.push(`PM-0004-21/std-sin: ${bites.stdSin}`);
  }
  const pairs = lines.filter((l) => / against /.test(l) && !l.startsWith("PM-"));
  const summary =
    `${problems.length ? `${problems.length} problem(s); ` : ""}${pairs.join("; ") || "no pair compared"}; h2 ${lines.some((l) => l.startsWith("h2: CO's wasm32 dump is the committed vector")) ? "equal" : "not equal"}; ` +
    `hash-map: ${bites.hashMap}; std-sin: ${bites.stdSin}; seconds: ${timed.join(", ")}`;
  return { ok: problems.length === 0, out: [...lines, ...problems, summary].join("\n") };
}

// Each cargo step takes the runner it calls cargo through (`run` when main calls it), so check.test.mjs pins what each
// step runs: its commands, its flags, -D warnings included (the rust fix2 review's nit, u4s-carries.md entry 34).
export const STEPS = {
  fmt: (runner = run) => runner("cargo", ["fmt", "--all", "--check"], HERE),
  // Both builds, as the tests run: the measured build (no features) and the bites build (every feature). A feature can
  // compile code out as well as in (npt-lab's probe has `cfg(not(feature = "bites"))` code), so one pass is not enough.
  clippy: (runner = run) => lintBothBuilds(["--workspace", "--all-targets"], runner),
  "clippy-wasm32": (runner = run) => lintBothBuilds(WASM_ARGS, runner),
  doc: docBothBuilds,
  test: testBothBuilds,
  deps: depsStep,
  // After deps, so the flags its builds use are checked first (u4d2-brief.md 2.4 (a)).
  wasm: (runner = run) => wasmStep(runner),
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

/**
 * Where a failed step's whole output is kept (the advisor's U4d2 round-1 ruling, FX-10: TE-L4): `<step>.log` in the
 * checkout's .gate/ (gitignored), one per step, removed when the step starts and written when it fails, so a panic
 * above the 40 lines main prints (B2.3's "name and panic") survives the run.
 */
export const FAILED_STEP_DIR = join(ROOT, ".gate", "netcode-proof-test", "failed-steps");

/**
 * The command: each step (all, or `--only`'s) from `steps`, its PASS or FAIL line printed through `log`; a failed
 * step's last 40 lines, then the path of the file in `keep` that holds its whole output.
 */
export function main(argv, steps = STEPS, log = console.log, keep = FAILED_STEP_DIR) {
  let only = null;
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--only" && i + 1 < argv.length) only = argv[++i].split(",");
    else throw new Error(`unknown argument ${argv[i]}`);
  }
  const names = only ?? Object.keys(steps);
  for (const n of names) if (!steps[n]) throw new Error(`no step ${n}; the steps: ${Object.keys(steps).join(", ")}`);
  let failed = 0;
  for (const n of names) {
    const file = join(keep, `${n}.log`);
    rmSync(file, { force: true });
    const started = Date.now();
    const r = steps[n]();
    const secs = ((Date.now() - started) / 1000).toFixed(1);
    log(`${r.ok ? "PASS" : "FAIL"}  ${n.padEnd(13)} ${secs} s  ${lastLines(r.out, 1)}`);
    if (!r.ok) {
      failed++;
      log(lastLines(r.out, 40).replace(/^/gm, "      "));
      try {
        mkdirSync(keep, { recursive: true });
        writeFileSync(file, r.out.endsWith("\n") ? r.out : `${r.out}\n`);
        log(`      the step's whole output (${r.out.split("\n").length} lines): ${file}`);
      } catch (e) {
        log(`      the step's whole output could not be kept in ${file}: ${e.message}`);
      }
    }
  }
  log(failed ? `check: ${failed} step(s) failed` : `check: all ${names.length} steps pass`);
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
