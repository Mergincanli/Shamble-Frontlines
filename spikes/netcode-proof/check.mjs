#!/usr/bin/env node
// The spike's test command (plan §12.4): every unit runs it, and its summary goes into the checkpoint.
//   node spikes/netcode-proof/check.mjs [--only <step,...>]
// Steps: fmt, clippy (host, all targets) and clippy-wasm32 (the leaf crates and npt-tcpinfo's non-Linux stub), each on
// the measured build and on the bites build with warnings as errors; test (cargo test on the measured build, then on
// the bites build); deps (the dependency rules of plan §4.2 that `cargo metadata` shows, with every feature and
// target, and its unsafe_code rule, from the crates' files, the workspace manifest and the rustflags and cargo configs
// this command's cargo reads: DEPS_CHECKED lists them); refs (nothing outside spikes/ references it, §4.2 and §4.9);
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
 * invariant, a test and an #[allow] with its reason"), named in the workspace Cargo.toml. npt-client-wasm relaxes it in
 * its manifest (wasm-bindgen's generated code), so its files are not scanned for it; npt-tcpinfo only through the
 * attributes TCPINFO_UNSAFE_FNS allows, and has `unsafe` only inside those functions.
 */
export const UNSAFE_EXCEPTIONS = { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo" };
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
 * The `unsafe` keyword (a raw identifier `r#unsafe` is a name, not the keyword) and `global_asm!`, the one construct
 * the unsafe_code lint denies that edition 2024 lets go without the keyword (`#[no_mangle]`, `#[export_name]` and
 * `#[link_section]` need `#[unsafe(...)]` there; rustc 1.99.1 compiles a macro-built `#[allow($l)]` over
 * `global_asm!("")` with no `unsafe` in sight). Found in code, they are unsafe code whatever relaxed the lint: an
 * attribute this scan cannot read (one built by a macro), rustflags or `--cap-lints`.
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
 * What is caught:
 * - outside the two exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included
 *   (and a comment or literal inside the attribute's list), and any `unsafe` or `global_asm!` in code, which is unsafe
 *   code however the lint was relaxed;
 * - a manifest other than npt-client-wasm's that sets unsafe_code or does not inherit the workspace lints;
 * - in npt-tcpinfo, any relaxing but the two placed allows, `unsafe` outside their two functions, the two clippy lints
 *   named outside lib.rs's crate-level deny or that deny missing (tcpinfoProblems), and either allow missing;
 * - in every crate, a #[path] or include! that names anything but a .rs file inside the crate's folder (pathProblems),
 *   the way another crate could compile npt-tcpinfo's file under its own name.
 * npt-client-wasm's .rs files are not scanned for unsafe code: its manifest relaxes the lint for the whole crate.
 * What lies outside these files (the workspace manifest's lint, the targets' paths and editions, symbolic links,
 * rustflags and cargo configs) is checked by workspaceLintProblems, layoutProblems, crateFiles and
 * cargoConfigProblems.
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
 * The workspace manifest's unsafe_code lint, which the whole rule rests on (every crate but npt-client-wasm inherits
 * it): `[workspace.lints.rust] unsafe_code = "deny"` or "forbid" (or `{ level = ... }`). Every line naming unsafe_code
 * (comments aside) must be that, in that table, and one must exist; any other form, place or level is a problem, so a
 * rewritten or relaxed line fails closed (TOML forbids a second definition of the key).
 */
export function workspaceLintProblems(text) {
  const problems = [];
  const ok = /^["']?unsafe_code["']?\s*=\s*(?:(["'])(?:deny|forbid)\1|\{\s*level\s*=\s*(["'])(?:deny|forbid)\2\s*(?:,\s*priority\s*=\s*-?\d+\s*)?\})$/;
  let table = null;
  let found = 0;
  for (const [i, raw] of tomlScan(text).clean.split("\n").entries()) {
    const l = raw.trim();
    const header = l.match(/^\[\[?([^\]]+)\]\]?$/);
    if (header) table = header[1].replace(/\s+/g, "");
    else if (/\bunsafe_code\b/.test(l)) {
      if (table === "workspace.lints.rust" && ok.test(l)) found++;
      else problems.push(`Cargo.toml:${i + 1} names unsafe_code other than as "deny" or "forbid" in [workspace.lints.rust]`);
    }
  }
  if (found === 0) problems.push('Cargo.toml has no [workspace.lints.rust] unsafe_code = "deny" (or "forbid") for its crates to inherit');
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
 * Every way the cargo this command runs could relax a lint without a word in the crates (u3c review S1 and F-D), as a
 * sentence: a lint-lowering flag (LINT_LOWERING) in RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS or
 * CARGO_TARGET_<triple>_RUSTFLAGS (`env`), or in a `rustflags` key of a cargo config file (`configs`: { path, text },
 * the ones cargoConfigFiles finds); a rustflags value this check cannot read; and an alias that replaces clippy or fmt
 * (CARGO_ALIAS_CLIPPY or _FMT, or such a key in a config file).
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
  }
  return problems;
}

/**
 * The cargo config files cargo reads when it runs in `start`: .cargo/config.toml and .cargo/config in `start` and in
 * every folder above it, then in CARGO_HOME (default ~/.cargo); each that exists, as { path, text }.
 */
function cargoConfigFiles(start) {
  const dirs = [];
  for (let d = resolve(start); ; d = dirname(d)) {
    dirs.push(join(d, ".cargo"));
    if (dirname(d) === d) break;
  }
  dirs.push(process.env.CARGO_HOME ?? join(homedir(), ".cargo"));
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
  "npt-net uses it; npt-net has no direct libc; nothing depends on npt-lab; the workspace denies unsafe_code; it is " +
  "relaxed only by npt-client-wasm's manifest and the #[allow] on npt-tcpinfo's srtt_us and unacked_and_unsent, and " +
  "unsafe code appears only there and inside those two functions, under lib.rs's deny of the two clippy unsafe lints; " +
  "no rustflags or cargo config lowers a lint or replaces clippy or fmt; every member lives in crates/ and builds " +
  "only .rs files in its own folder, in edition 2024, none a proc-macro, and every path dependency is a member; no " +
  "symbolic link under crates/; every #[path] and include! names a .rs file in its own crate";

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
    const meta = JSON.parse(r.stdout);
    const { files, links } = crateFiles(join(HERE, "crates"));
    const configs = cargoConfigFiles(HERE);
    const problems = [
      ...dependencyProblems(meta),
      ...workspaceLintProblems(readFileSync(join(HERE, "Cargo.toml"), "utf8")),
      ...layoutProblems(meta),
      ...links.map((l) => `${l} is a symbolic link: the scan does not follow it, and rustc would`),
      ...cargoConfigProblems(process.env, configs),
      ...unsafeProblems(files, (f) => readFileSync(join(HERE, f), "utf8")),
    ];
    const read = `${files.length} crate files and ${configs.length} cargo config file(s) read`;
    return { ok: problems.length === 0, out: problems.join("\n") || `${DEPS_CHECKED} (${read})` };
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
