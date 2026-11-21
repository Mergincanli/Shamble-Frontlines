// rust-reviewer, U4d2 round 2: read-only probes of FX-1 (buildInputProblems on env and config forms) and FX-2 (the
// loader's target_features read on both built modules and on hand-built edge cases). Prints only; writes nothing.
// Run through u4d2-run.sh: node <this file> <copy root>
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.argv[2];
const spike = join(root, "spikes", "netcode-proof");
const loader = await import(pathToFileURL(join(spike, "crates", "hash-wasm", "loader.mjs")).href);
const check = await import(pathToFileURL(join(spike, "check.mjs")).href);
const { moduleProblems, HASH_WASM_EXPORTS } = loader;
const { buildInputProblems, cargoConfigProblems, rustflagSources } = check;

const show = (label, v) => console.log(`${label}: ${JSON.stringify(v)}`);

// ---- An independent target_features reader (strict: varuint32 with the 5th byte < 16), for cross-checking.
function strict(bytes) {
  let at = 0;
  const u32 = () => {
    let n = 0;
    for (let i = 0; i < 5; i++) {
      const b = bytes[at++];
      if (b === undefined) throw new Error("eof");
      if (i === 4 && (b & 0x70)) throw new Error("varuint32 overflow");
      n += (b & 0x7f) * 2 ** (7 * i);
      if (!(b & 0x80)) return n;
    }
    throw new Error("too long");
  };
  const count = u32();
  const out = [];
  for (let k = 0; k < count; k++) {
    const p = String.fromCharCode(bytes[at++]);
    const len = u32();
    out.push(p + Buffer.from(bytes.subarray(at, at + len)).toString("utf8"));
    at += len;
  }
  if (at !== bytes.length) throw new Error("trailing");
  return out;
}

// ---- FX-2 (a): both built modules.
for (const f of [
  join(root, ".gate", "netcode-proof-test", "wasm-step", "bin", "npt-hash-wasm.wasm"),
  join(root, ".gate", "netcode-proof-test", "wasm-step", "bin", "npt-hash-wasm-bites.wasm"),
]) {
  const bytes = readFileSync(f);
  const m = new WebAssembly.Module(bytes);
  const secs = WebAssembly.Module.customSections(m, "target_features");
  console.log(`built ${f}: ${bytes.length} B; target_features sections ${secs.length}; strict read ${JSON.stringify(secs.map((s) => strict(new Uint8Array(s))))}`);
  show(`  moduleProblems`, moduleProblems(m));
  show(`  custom sections`, WebAssembly.Module.customSections(m, "producers").length);
}

// ---- FX-2 (b): hand-built modules (the listed exports and memory, then the given target_features bodies).
const leb = (n) => {
  const out = [];
  do {
    let b = n & 0x7f;
    n = Math.floor(n / 128);
    if (n) b |= 0x80;
    out.push(b);
  } while (n);
  return out;
};
const name = (s) => [...leb(Buffer.byteLength(s)), ...Buffer.from(s)];
const sec = (id, body) => [id, ...leb(body.length), ...body];
const entry = (p, n) => [p.charCodeAt(0), ...name(n)];
function mod(bodies) {
  const listed = [...HASH_WASM_EXPORTS, "memory"];
  const fns = HASH_WASM_EXPORTS;
  const body = [0, 0x41, 0, 0x0b];
  return new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...sec(1, [1, 0x60, 0, 1, 0x7f]),
    ...sec(3, [...leb(fns.length), ...fns.map(() => 0)]),
    ...sec(5, [1, 0x00, 1]),
    ...sec(7, [...leb(listed.length), ...listed.flatMap((x, i) => (x === "memory" ? [...name(x), 0x02, 0] : [...name(x), 0x00, ...leb(i)]))]),
    ...sec(10, [...leb(fns.length), ...fns.flatMap(() => [...leb(body.length), ...body])]),
    ...bodies.flatMap((b) => sec(0, [...name("target_features"), ...b])),
  ]);
}
const cases = {
  "clean +sign-ext -simd128": [[2, ...entry("+", "sign-ext"), ...entry("-", "simd128")]],
  "count 0 (an empty list)": [[0]],
  "empty body": [[]],
  "overlong count 1 in 5 bytes (spec-valid), +sign-ext": [[0x81, 0x80, 0x80, 0x80, 0x00, ...entry("+", "sign-ext")]],
  "overlong count 1 in 5 bytes, +simd128": [[0x81, 0x80, 0x80, 0x80, 0x00, ...entry("+", "simd128")]],
  "count with 5th byte 0x10 (2^32+1; spec-malformed)": [[0x81, 0x80, 0x80, 0x80, 0x10, ...entry("+", "sign-ext")]],
  "count in 6 bytes": [[0x81, 0x80, 0x80, 0x80, 0x80, 0x00, ...entry("+", "sign-ext")]],
  "name length 2^32-1": [[1, 0x2b, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x61]],
  "name length overlong 7 in 5 bytes, +simd128": [[1, 0x2b, 0x87, 0x80, 0x80, 0x80, 0x00, ...Buffer.from("simd128")]],
  "name length in 6 bytes": [[1, 0x2b, 0x87, 0x80, 0x80, 0x80, 0x80, 0x00, ...Buffer.from("simd128")]],
  "BOM then simd128 (+)": [[1, 0x2b, 10, 0xef, 0xbb, 0xbf, ...Buffer.from("simd128")]],
  "-simd128 then +simd128": [[2, ...entry("-", "simd128"), ...entry("+", "simd128")]],
  "=relaxed-simd": [[1, ...entry("=", "relaxed-simd")]],
  "prefix 0x00": [[1, 0x00, ...name("simd128")]],
  "a trailing byte": [[1, ...entry("+", "sign-ext"), 0]],
  "count 2, one entry": [[2, ...entry("+", "sign-ext")]],
  "two sections, both clean": [[1, ...entry("+", "sign-ext")], [1, ...entry("+", "sign-ext")]],
  "no section": [],
  "upper-case SIMD128 (+)": [[1, ...entry("+", "SIMD128")]],
};
for (const [label, bodies] of Object.entries(cases)) {
  let p;
  try {
    p = moduleProblems(new WebAssembly.Module(mod(bodies)));
  } catch (e) {
    p = [`(V8 refused the module: ${e.message})`];
  }
  show(`FX-2 ${label}`, p);
}

// ---- FX-1: buildInputProblems (and the config reader) on env and config forms.
const envs = {
  "r1 first env": { CARGO_ENCODED_RUSTFLAGS: "-Ctarget-cpu=bleeding-edge", RUSTFLAGS: "-Ctarget-feature=-simd128,-relaxed-simd" },
  "encoded: only separators": { CARGO_ENCODED_RUSTFLAGS: "\x1f\x1f" },
  "encoded: one space": { CARGO_ENCODED_RUSTFLAGS: " " },
  "RUSTFLAGS: spaces and tab": { RUSTFLAGS: "  \t " },
  "RUSTFLAGS: U+FEFF only": { RUSTFLAGS: "﻿" },
  "RUSTFLAGS: U+0085 only": { RUSTFLAGS: "\u0085" },
  "CARGO_PROFILE_RELEASE_RUSTFLAGS": { CARGO_PROFILE_RELEASE_RUSTFLAGS: "-Cx" },
  "CARGO_HOST_RUSTFLAGS": { CARGO_HOST_RUSTFLAGS: "-Cx" },
  "CARGO_ENCODED_RUSTDOCFLAGS (not rustc's)": { CARGO_ENCODED_RUSTDOCFLAGS: "-Cx" },
  "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER": { CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: "/x/ld" },
  "CARGO_BUILD_TARGET unset but CARGO_BUILD_TARGET_DIR": { CARGO_BUILD_TARGET_DIR: "/t" },
};
for (const [label, env] of Object.entries(envs)) show(`FX-1 env ${label}`, buildInputProblems(env, []));
const configs = {
  "target cfg table": "[target.'cfg(target_arch = \"wasm32\")']\nrustflags = [\"-Ctarget-feature=+simd128\"]\n",
  "dotted target cfg": "target.'cfg(all())'.rustflags = \"-Cx\"\n",
  "dotted with spaces": "build . rustflags = [\"-Cx\"]\n",
  "inline table": "build = { rustflags = [\"--cfg\", \"feature=\\\"bites\\\"\"] }\n",
  "quoted key": "[build]\n\"rustflags\" = [\"-Cx\"]\n",
  "escaped key": "[build]\n\"rust\\u0066lags\" = [\"-Cx\"]\n",
  "literal multi-line value": "[build]\nrustflags = '''-Ctarget-feature=+simd128'''\n",
  "multi-line array": "[build]\nrustflags = [\n  \"-C\", # c\n  \"target-cpu=native\",\n]\n",
  "host table": "[host]\nrustflags = [\"-Cx\"]\n",
  "profile table": "[profile.release]\nrustflags = [\"-Cx\"]\n",
  "empty string value": "[build]\nrustflags = \"\"\n",
  "empty array": "[build]\nrustflags = []\n",
  "array of one empty string": "[build]\nrustflags = [\"\"]\n",
  "header with spaces": "[ build ]\nrustflags = \"-Cx\"\n",
  "target key quoted": "[build]\n'target' = \"wasm32-unknown-unknown\"\n",
  "target key dotted with spaces": "build . target = \"x\"\n",
  "target array empty": "[build]\ntarget = []\n",
  "target-dir only (control)": "[build]\ntarget-dir = \"t\"\n",
  "a target table's linker (control)": "[target.wasm32-unknown-unknown]\nlinker = \"/x/ld\"\n",
  "CRLF": "[build]\r\nrustflags = \"-Cx\"\r\n",
  "rustflags after a BOM": "﻿build.rustflags = [\"-Cx\"]\n",
};
for (const [label, text] of Object.entries(configs)) {
  const cs = [{ path: "c.toml", text }];
  show(`FX-1 config ${label}`, { input: buildInputProblems({}, cs), sources: rustflagSources({}, cs), config: cargoConfigProblems({}, cs) });
}
