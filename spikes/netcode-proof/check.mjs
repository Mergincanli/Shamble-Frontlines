#!/usr/bin/env node
// The spike's test command (plan §12.4): every unit runs it, and its summary goes into the checkpoint.
//   node spikes/netcode-proof/check.mjs [--only <step,...>]
// Steps: fmt, clippy (host, all targets) and clippy-wasm32 (the leaf crates and npt-tcpinfo's non-Linux stub), each on
// the measured build and on the bites build with warnings as errors; test (cargo test on the measured build, then on
// the bites build, its counts and any test that reported a skip); deps (the dependency rules of plan §4.2 that
// `cargo metadata` shows, with every feature and target, and its unsafe_code rule, from the crates' files, the
// manifests and the environment and cargo configs this command's cargo reads: DEPS_CHECKED says what it checks, and
// the comment above it what it does not); refs (nothing outside spikes/ references it, §4.2 and §4.9);
// marks (marks.json compiles in the judge, covers exactly the plan's marks, and every row it hashed is unchanged); self
// (this file's own tests); redact (redact-check's tests, then a scan of the spike's folder). It runs every step and
// exits 1 if any failed.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
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
 * invariant, a test and an #[allow] with its reason"), named in the workspace Cargo.toml. The workspace forbids the
 * lint, so these two carry their own [lints] tables in place of `workspace = true` (EXCEPTION_LEVELS):
 * npt-client-wasm allows it for the whole crate (wasm-bindgen's generated code), so its files are not scanned for it;
 * npt-tcpinfo denies it, which its two #[allow]s can relax (forbid could not), only through the attributes
 * TCPINFO_UNSAFE_FNS allows, and has `unsafe` only inside those functions.
 */
export const UNSAFE_EXCEPTIONS = { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo" };
/** The unsafe_code level each exception's own [lints.rust] gives, its other lints being the workspace's (review C2R-1). */
const EXCEPTION_LEVELS = { "crates/client-wasm/": "allow", "crates/tcpinfo/": "deny" };
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
 * attribute this scan cannot read (one built by a macro), rustflags or `--cap-lints`. In every crate but the two
 * exceptions the compiler refuses them first (the workspace's forbid: E0453 on any allow, a macro-built one included);
 * this scan is the second layer there, and inside npt-tcpinfo the only one.
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
 * can read here as a comment or a literal. Outside the two exception crates the workspace's forbid makes rustc refuse
 * unsafe code this misses; inside npt-tcpinfo (deny, not forbid) nothing but review does.
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
 * under the workspace's forbid is still refused by rustc if it holds unsafe code or an allow of the lint; into
 * npt-tcpinfo, only review stops it.
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
 * Every place under crates/ that breaks the unsafe_code rule, as a sentence; empty when none does (u3c-plan.md 1.8 (a)
 * and (d)). `files` are paths relative to the spike's folder (crates/<crate>/...): its .rs files and crate manifests.
 * What is caught, as lexRust reads the code (its known limit is there):
 * - outside the two exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included
 *   (and a comment or literal inside the attribute's list), and any `unsafe` or `global_asm!` in code;
 * - a manifest other than the two exceptions' that names unsafe_code (UNSAFE_CODE_NAME: `unsafe-code` too) or does not
 *   inherit the workspace lints (the exceptions' own tables are exceptionLintProblems');
 * - in npt-tcpinfo, any relaxing but the two placed allows, `unsafe` outside their two functions, the two clippy lints
 *   named outside lib.rs's crate-level deny or that deny missing (tcpinfoProblems), and either allow missing;
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
  const state = { rootDeny: false };
  const others = Object.values(UNSAFE_EXCEPTIONS).join(" and ");
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
      problems.push(...pathProblems(f, crate, code, mask, attrs));
    } else if (f === `${crate}Cargo.toml` && !(crate in UNSAFE_EXCEPTIONS)) {
      const lines = text.split("\n").map((l) => l.replace(/#.*$/, ""));
      if (lines.some((l) => UNSAFE_CODE_NAME.test(l))) problems.push(`${f} sets unsafe_code`);
      if (!inheritsWorkspaceLints(lines)) problems.push(`${f} does not inherit the workspace lints ([lints] workspace = true)`);
    }
  }
  for (const fn of TCPINFO_UNSAFE_FNS) {
    const n = sites.get(fn) ?? 0;
    if (n !== 1) problems.push(`npt-tcpinfo's pub fn ${fn} has ${n} #[allow(unsafe_code, reason = ...)] attributes, not 1`);
  }
  if (!state.rootDeny) {
    problems.push(`${TCPINFO_ROOT} does not deny clippy::${TCPINFO_CLIPPY_LINTS.join(" and clippy::")} at the crate root`);
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
 * "forbid"` (or `{ level = "forbid", priority = n }`), the compiler backstop (review C2R-1): every member but the two
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
 * The two exceptions' own lint tables (review C2R-1; replacing u3c-plan.md 1.8 (a)'s "keep [lints] workspace = true"
 * for npt-tcpinfo, the advisor's ruling of 2026-11-08): each crate's `[lints.<tool>]` tables hold exactly the
 * workspace's `[workspace.lints.<tool>]` keys and values, every tool and key, but unsafe_code, which is
 * EXCEPTION_LEVELS' level: "deny" for npt-tcpinfo (its two #[allow]s need a level they can relax), "allow" for
 * npt-client-wasm. Key for key as cargo reads the manifest: any lint given in a form lintTables does not read (a lint's
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
    }
  }
  return problems;
}

/**
 * A TOML text read for this file's checks: `clean` with its comments blanked, `mask` with its strings blanked too (both
 * as long as the text, newlines kept), and `strings`, each one's span and value (basic strings' escapes decoded,
 * multi-line and literal strings included). Enough for cargo's config and manifest files, not a full TOML parser: what
 * it cannot read is reported by its callers, never passed.
 */
function tomlScan(text) {
  const blank = (s) => s.replace(/[^\n]/g, " ");
  const escapes = { b: "\b", t: "\t", n: "\n", f: "\f", r: "\r", e: "\x1b", '"': '"', "\\": "\\" };
  const decode = (raw) =>
    raw.replace(/\\(?:u([0-9a-fA-F]{4})|U([0-9a-fA-F]{8})|x([0-9a-fA-F]{2})|([btnfre"\\])|[ \t]*\r?\n\s*)/g, (_, u, U, x, ch) => {
      if (u || U || x) return String.fromCodePoint(parseInt(u || U || x, 16));
      return ch ? escapes[ch] : "";
    });
  let clean = "";
  let mask = "";
  const strings = [];
  let i = 0;
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
 * Every way the cargo this command runs could relax a lint, or stand in for a step, without a word in the crates (u3c
 * review S1, F-D, C2R-4, C2R-7, C2R-8), as a sentence:
 * - a lint-lowering flag (LINT_LOWERING) in RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS or
 *   CARGO_TARGET_<triple>_RUSTFLAGS (`env`), or in a `rustflags` key of a cargo config file (`configs`: { path, text },
 *   the ones cargoConfigFiles finds); a rustflags value this check cannot read;
 * - an alias that replaces clippy or fmt (CARGO_ALIAS_CLIPPY or _FMT, or such a key in a config file);
 * - another compiler or a workspace wrapper (COMPILER_ENV, COMPILER_KEYS);
 * - a config `include` key, which loads a config file cargoConfigFiles does not read (cargo 1.99.1 honours it);
 * - a target runner (CARGO_TARGET_<triple>_RUNNER, or a `runner` key in a config file): it runs the test binaries in
 *   cargo's place, and the test step judges cargo's exit status (a runner `true` passes with no test run);
 * - RUSTC_BOOTSTRAP, whatever its value (review C2b F5): it unlocks unstable features on the pinned stable toolchain,
 *   and with it cargo 1.99.1 honoured a manifest's `cargo-features = ["profile-rustflags"]` and a profile's
 *   `rustflags = ["--cap-lints", "warn"]`, which turned the workspace's forbid into a warning (those manifest keys are
 *   manifestKeyProblems').
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
    if (/^CARGO_TARGET_\w+_RUNNER$/.test(k)) problems.push(`${k} in the environment runs the test binaries in cargo's place`);
    if (k === "RUSTC_BOOTSTRAP") problems.push(`${k} in the environment unlocks unstable cargo and rustc features (a manifest's profile rustflags)`);
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
    if (tomlValues(scan, "include").length || /^[ \t]*\[\[?[ \t]*include[ \t]*[\].]/m.test(scan.mask)) {
      problems.push(`${path} has a key \`include\`: it loads a config file this check does not read`);
    }
    if (tomlValues(scan, "runner").length) problems.push(`${path} has a key \`runner\`: it runs the test binaries in cargo's place`);
  }
  return problems;
}

/** The manifest keys manifestKeyProblems refuses, with why. */
const MANIFEST_KEYS = {
  "cargo-features": "it unlocks unstable manifest keys (a profile's rustflags among them)",
  rustflags: "a manifest's rustflags reach rustc (`--cap-lints warn` there turns the workspace's forbid into a warning)",
};

/**
 * The manifest keys that reach rustc's flags (u3c review C2b F5), refused in the workspace manifest and in every crate
 * manifest under crates/ (`manifests`: { path, text }), whatever their value and in any table or form (bare, dotted,
 * quoted, inline): `cargo-features`, which cargo 1.99.1 accepts only on a nightly toolchain or under RUSTC_BOOTSTRAP
 * (cargoConfigProblems refuses that), and any `rustflags` key (a profile's, under `cargo-features =
 * ["profile-rustflags"]`): with both, `rustflags = ["--cap-lints", "warn"]` compiled a planted allow and unsafe block
 * under the workspace's forbid with no warning while this step passed. Neither key is in any manifest of the spike.
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
 * The cargo config files cargo reads when it runs in `start`: .cargo/config.toml and .cargo/config in `start` and in
 * every folder above it, then in CARGO_HOME (`env`'s, default ~/.cargo); each that exists, as { path, text }.
 */
function cargoConfigFiles(start, env) {
  const dirs = [];
  for (let d = resolve(start); ; d = dirname(d)) {
    dirs.push(join(d, ".cargo"));
    if (dirname(d) === d) break;
  }
  dirs.push(env.CARGO_HOME ?? join(homedir(), ".cargo"));
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

function run(cmd, args, cwd) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  // `stdout` alone is kept for the steps that parse it: cargo writes progress lines ("Blocking waiting for file
  // lock", "Downloaded …") to stderr, and they must never reach a JSON parser.
  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status };
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

/** clippy with warnings as errors on the measured build (no features), then on the bites build (every feature). */
function lintBothBuilds(scope) {
  const measured = run("cargo", ["clippy", "--locked", ...scope, "--", "-D", "warnings"], HERE);
  if (!measured.ok) return measured;
  const bites = run("cargo", ["clippy", "--locked", ...scope, "--all-features", "--", "-D", "warnings"], HERE);
  return { ok: bites.ok, out: `${measured.out}${bites.out}\nmeasured build and bites build both linted`, status: bites.status };
}

/** Both builds' tests: the measured build (no features), then the bites build (every feature, the planted defects
 * compiled in: plan §4.7). Each count is reported, skips included (cargoCounts). */
function testBothBuilds() {
  const measured = run("cargo", ["test", "--locked", "--workspace"], HERE);
  if (!measured.ok) return measured;
  const bites = run("cargo", ["test", "--locked", "--workspace", "--all-features"], HERE);
  const summary = `measured build: ${cargoCounts(measured.out)}; bites build: ${cargoCounts(bites.out)}`;
  return { ok: bites.ok, out: `${measured.out}${bites.out}\n${summary}`, status: bites.status };
}

/**
 * What the deps step checks, as its summary line says it (plan §4.2's dependency and unsafe rules that are read from
 * `cargo metadata`, the manifests, the crates' files, the environment and the cargo configs; §4.2's wasm32 builds and
 * clock reads are the clippy steps', its "no reference into spikes/" is the refs step's, and its narrow public
 * surfaces are not checked by this command).
 *
 * The static unsafe scan guards against an accidental relaxation; review, and U5's audit, are the wall against a
 * deliberate one. Between them stands the compiler: under the workspace's forbid, rustc refuses unsafe code and any
 * allow of the lint (E0453, a macro-built allow included) in every crate but the two exceptions, whatever the scan
 * misreads. Known limits, which the step does not report:
 * - lexRust is not rustc's lexer (a shebang line; a `\u{...}` escape with underscores past six characters: review
 *   C2R-1), and pathProblems reads a #[path] or include! literal with its escapes undecoded and include! by its name
 *   only (C2R-2). The compiler covers both in the crates under the forbid; inside npt-tcpinfo only review does.
 * - Inputs that can lower lints and are not read here: a cargo config's [env] table (CLIPPY_ARGS, C2R-4 G, or any
 *   other variable it sets for rustc), what a member's build script prints (`cargo::rustc-env`, C2R-4 H), a
 *   RUSTC_WRAPPER program (allowed, for sccache) and a rust-toolchain.toml `path`. Lint capping by any of them
 *   (`--cap-lints`) lowers the forbid too.
 * - Which toolchain runs: a nightly one (rust-toolchain.toml's channel, RUSTUP_TOOLCHAIN, `cargo +nightly` or a rustup
 *   override) is not detected, and its unstable features are not read. The manifest keys that reach rustc's flags
 *   (`cargo-features`, `rustflags`) are refused whatever the channel, and so is RUSTC_BOOTSTRAP (review C2b F5).
 * - Unsafe code a dependency's macro expands into a crate is not linted by rustc (another crate's code); the
 *   dependencies are U5's audit.
 * - Too wide, not too narrow: the runner refusal covers every target triple, so a future wasm32 test runner
 *   (wasm-bindgen-test's `[target.wasm32-unknown-unknown] runner`) would be refused too, though the test step runs
 *   host tests only (no wasm test is planned; review C2b's note).
 */
const DEPS_CHECKED =
  "held: no leaf crate reaches a transport or runtime; npt-client has no npt-net; npt-super and npt-host keep off the " +
  "sim, room, proto (and client) crates; npt-judge ships npt-log and serde only; npt-tcpinfo ships libc only and only " +
  "npt-net uses it; npt-net has no direct libc; nothing depends on npt-lab; the workspace forbids unsafe_code and " +
  "every member inherits its lints but npt-client-wasm (allow) and npt-tcpinfo (deny), whose own lint tables are the " +
  "workspace's otherwise, key for key, with no lint given in a form the reader does not read (a lint's own table, a " +
  "quoted, dotted, hyphenated or uppercase name); no manifest has a cargo-features or rustflags key; as the scan " +
  "reads crates/ (lexRust), no other crate relaxes unsafe_code or has unsafe code, " +
  "and npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
  "them, and keeps lib.rs's deny of the two clippy unsafe lints; the environment and the cargo configs read pass no " +
  "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no config include and " +
  "no test runner, and the environment has no RUSTC_BOOTSTRAP; every member lives in crates/ and builds only .rs " +
  "files in its own folder, in edition 2024, none " +
  "a proc-macro, and every path dependency is a member; no symbolic link under crates/; every #[path] and include! " +
  "literal names a .rs file in its own crate";

/** `cargo metadata` for the spike in `here`, with every feature, parsed: { ok, meta }, or the failed run. */
function cargoMetadata(here) {
  const r = run("cargo", ["metadata", "--locked", "--format-version", "1", "--all-features"], here);
  return r.ok ? { ok: true, meta: JSON.parse(r.stdout) } : r;
}

/**
 * The deps step on the spike in `here`, with `env` as the environment and `metadata(here)` for cargo metadata: every
 * check it makes is called here, and STEPS.deps is this function (review C2R-5: check.test.mjs runs it on a fixture
 * with one plant per check, so a check unhooked from it fails a test).
 */
export function depsStep(here = HERE, env = process.env, metadata = cargoMetadata) {
  const r = metadata(here);
  if (!r.ok) return r;
  const { meta } = r;
  const { files, links } = crateFiles(join(here, "crates"));
  const configs = cargoConfigFiles(here, env);
  const read = (f) => readFileSync(join(here, f), "utf8");
  const workspace = read("Cargo.toml");
  const manifests = [{ path: "Cargo.toml", text: workspace }];
  for (const f of files) if (f.endsWith("/Cargo.toml")) manifests.push({ path: f, text: read(f) });
  const problems = [
    ...dependencyProblems(meta),
    ...workspaceLintProblems(workspace),
    ...exceptionLintProblems(workspace, read),
    ...manifestKeyProblems(manifests),
    ...layoutProblems(meta),
    ...links.map((l) => `${l} is a symbolic link: the scan does not follow it, and rustc would`),
    ...cargoConfigProblems(env, configs),
    ...unsafeProblems(files, read),
  ];
  const counts = `${files.length} crate files and ${configs.length} cargo config file(s) read`;
  return { ok: problems.length === 0, out: problems.join("\n") || `${DEPS_CHECKED} (${counts})` };
}

export const STEPS = {
  fmt: () => run("cargo", ["fmt", "--all", "--check"], HERE),
  // Both builds, as the tests run: the measured build (no features) and the bites build (every feature). A feature can
  // compile code out as well as in (npt-lab's probe has `cfg(not(feature = "bites"))` code), so one pass is not enough.
  clippy: () => lintBothBuilds(["--workspace", "--all-targets"]),
  "clippy-wasm32": () => lintBothBuilds(WASM_ARGS),
  test: testBothBuilds,
  deps: depsStep,
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
